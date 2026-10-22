//! easel: a live Lua painting session over the claude-paint engine.
//!
//!   easel open [<name>]              start (or reattach to) a session
//!   easel do '<lua>' | -f chunk.lua | -            run a chunk on the live canvas
//!   easel look [--crop x0,y0,x1,y1] [--mode value,squint,edges,mirror,relief,gallery] [--light az,el] [--grid [step]] [--size N] [--mask name[,name]]
//!   easel log | status | globals | save [path] [--light az,el | --gallery] | frames on|off | close
//!   easel check                                    (replay build) replay the log, compare
//!   easel note '<text>' | -                        append to notes/journal.md
//!   easel run paintings/lua/<name>.lua [--out path] [--light az,el | --gallery] [--look] [--state-digest digests.txt]
//!                                    (replay build) replay at the width it was painted at
//!
//! One binary, two runtime modes (notes/easel.md#studio-selection). The
//! developer mode has named sessions, `EASEL_ROOT`, replay and finishing.
//! Painter mode has none of them: its studio is the directory above the
//! executable's (`<studio>/bin/easel`) and holds one painting, `PAINTING`.

mod api;
mod ground_cache;
mod board;
mod check;
mod depth;
mod display;
mod draw_edges;
mod draw_outline;
mod finish;
mod form;
mod frames;
mod legacy;
mod look;
mod output;
mod live_frames;
mod save;
mod replay_checkpoint;
#[cfg(feature = "state-store")]
mod painting_store;
#[cfg(feature = "render-snapshot")]
mod optical;
mod palette_look;
mod session;
mod state_dump;
mod studio;
mod time;
mod world;

use paint::Canvas;
use session::{Session, parse_program, root};
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, Instant};

/// The width a painting is painted, replayed and delivered at (px) unless it
/// names another (`session::WIDTH_MARK`, configured when it began: `configured_width`).
const LIVE_WIDTH: usize = 2400;

/// The width a painting paints at: an existing one its log's (a sketch if
/// its head says so, `session::SKETCH_MARK`; else the width its head names,
/// `session::WIDTH_MARK`, or `LIVE_WIDTH`), a new one a sketch's if the
/// session's name starts with "sketch", else the configured one (`configured_width`).
fn width_for(name: &str, log: Option<&str>) -> Result<usize, String> {
    match log {
        Some(text) if session::logged_sketch(text) => Ok(session::SKETCH_WIDTH),
        Some(text) => session::logged_width(text),
        None if name.starts_with("sketch") => Ok(session::SKETCH_WIDTH),
        None => configured_width(),
    }
}

/// A new painting's width, as its box is configured (`session::configured_box`): a file
/// `width` next to this executable (a painter's client passes the easel no EASEL_* variable),
/// else `EASEL_WIDTH`, else `LIVE_WIDTH`. One of `session::WIDTHS`; both set, they must agree.
fn configured_width() -> Result<usize, String> {
    let file = std::env::current_exe().ok().and_then(|e| e.canonicalize().ok()).and_then(|e| e.parent().map(|d| d.join("width")));
    let from_file = match file {
        Some(f) if f.exists() => Some((std::fs::read_to_string(&f).map_err(|e| format!("{}: {e}", f.display()))?, f.display().to_string())),
        _ => None,
    };
    new_width(from_file.as_ref().map(|(t, p)| (t.as_str(), p.as_str())), std::env::var("EASEL_WIDTH").ok().as_deref())
}

/// `configured_width` from what the file (its text and path) and `EASEL_WIDTH` say.
fn new_width(file: Option<(&str, &str)>, env: Option<&str>) -> Result<usize, String> {
    let parse = |v: &str, from: &str| {
        v.trim().parse::<usize>().ok().filter(|w| session::WIDTHS.contains(w))
            .ok_or_else(|| format!("{from} says width {:?}: want one of {:?}", v.trim(), session::WIDTHS))
    };
    let f = file.map(|(text, path)| parse(text, path)).transpose()?;
    let e = env.filter(|v| !v.trim().is_empty()).map(|v| parse(v, "EASEL_WIDTH")).transpose()?;
    match (f, e) {
        (Some(a), Some(b)) if a != b => Err(format!("the width file says {a} but EASEL_WIDTH says {b}: set one, or both the same")),
        (Some(w), _) | (None, Some(w)) => Ok(w),
        (None, None) => Ok(LIVE_WIDTH),
    }
}

/// Painter mode's one session.
const PAINTING: &str = "painting";

const PAINTER_USAGE: &str = "easel: a live painting session (see notes/easel_guide.md)

  easel open          start or reattach; replays paintings/lua/painting.lua if it exists
  easel do '<lua>'  |  easel do -f chunk.lua  |  easel do - (stdin)     [--look] also looks afterwards
  easel look [--crop x0,y0,x1,y1] [--mode value,squint,edges,mirror,relief,gallery] [--light az,el] [--grid [step]] [--size 1000] [--mask name[,name]]
             [--survey]   the whole canvas at full detail, in tiles
             [--compare <earlier look png>]   that look beside this one
             [--ref <picture>]   the motif (a photo or study in the studio) beside the canvas, same view
             [--hold <knife or pile> --at x,y]   (speculative) the loaded knife held up to the canvas there
             [--hold <stick> --at x,y [--pose force,alt,azimuth[,roll] | --side direction]]   with paper and pastel: where a pastel stick held there would touch
  easel look --palette   the palette board: every heap knifed out thick and smeared thin across a black stripe
  easel probe --at x,y | --crop x0,y0,x1,y1   the paint there: color and value (sRGB 0-255), in gallery light too, and its drying
  easel log           the painting so far (= paintings/lua/painting.lua)
  easel status        chunks, width, canvas
  easel globals       the painting's globals, one a line: chunk that last set it, name, what it holds
  easel save [path] [--light az,el | --gallery]   the canvas as a PNG inside the studio (default out/easel/painting/painting.png), lit on its relief if asked
  easel frames on|off save a look after every chunk
  easel close         end the session (the log stays)
  easel note '<text>' | easel note - (stdin)    append a dated entry to notes/journal.md
  easel tubes [--markdown]   the tubes in the box (--markdown: as a table)";

const DEVELOPER_USAGE: &str = "easel: a live painting session (see notes/easel_guide.md)

  easel open <name>    start or reattach; replays paintings/lua/<name>.lua if it exists
                      (a new session named sketch... is a sketch, 600px)
  easel do '<lua>'  |  easel do -f chunk.lua  |  easel do - (stdin)     [--look] also looks afterwards
  easel look [--crop x0,y0,x1,y1] [--mode value,squint,edges,mirror,relief,gallery] [--light az,el] [--grid [step]] [--size 1000] [--mask name[,name]]
             [--survey]   the whole canvas at full detail, in tiles
             [--compare <earlier look png>]   that look beside this one
             [--ref <picture>]   the motif (a photo or study in the studio) beside the canvas, same view
             [--hold <knife or pile> --at x,y]   (speculative) the loaded knife held up to the canvas there
             [--hold <stick> --at x,y [--pose force,alt,azimuth[,roll] | --side direction]]   with paper and pastel: where a pastel stick held there would touch
  easel look --palette   the palette board: every heap knifed out thick and smeared thin across a black stripe
  easel probe --at x,y | --crop x0,y0,x1,y1   the paint there: color and value (sRGB 0-255), in gallery light too, and its drying
  easel log           the session so far (= paintings/lua/<name>.lua)
  easel status        chunks, width, canvas
  easel globals       the painting's globals, one a line: chunk that last set it, name, what it holds
  easel save [path] [--light az,el | --gallery]   the canvas as a PNG inside the studio (default out/easel/<name>/<name>.png), lit on its relief if asked
  easel frames on|off save a look after every chunk
  easel check         replay the log from scratch and compare with the live canvas
  easel close         end the session (the log stays)
  easel note '<text>' | easel note - (stdin)    append a dated entry to notes/journal.md
  easel run <file.lua> [--out path.png] [--light az,el | --gallery] [--look] [--state-digest digests.txt]
                      replay at its width and write the PNG
      [--checkpoint-out path.ckpt]   export the final finishable state after success
      [--frames-every <s> --frames-dir <dir> [--frame-width 1000]]   and a frame per <s> of hand time
      [--width <px>]   replay narrower, a preview for development (not the painting)
  easel finish <save> <out.png> [--log painting.lua] [--coats C] [--no-varnish] [--no-cracks] [--relief]
                      finish a closed session's save (out/easel/<name>/live.ckpt) without a replay
  easel render-checkpoint <save.ckpt[.zst]> <out.png> [--mode color|gallery|relief] [--look] [--log painting.lua]
                      render saved state without replay or aging (default gallery; --look uses look sizing)
  easel tubes [--markdown]   the tubes in the box a new painting takes (--markdown: as a table)

  A new painting takes its box from a file `box` next to this executable, else EASEL_BOX,
  else the default tube box; its log names the box (a line \"--@ box <name>\"), and every
  replay of the log uses that box.

  A new painting is 2400 px wide, or the width in a file `width` next to this executable,
  else EASEL_WIDTH (2400 or 4800); a log
  painted at another width names it (a line \"--@ width <px>\"), and every replay uses it.

  -s <name> (or EASEL_SESSION) picks the session; default: the last opened.";

fn main() -> ExitCode {
    if let Err(e) = studio::load() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let name = match take_session(&mut args) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    let Some(cmd) = args.first().cloned() else {
        print_usage();
        return ExitCode::SUCCESS;
    };
    let rest = args[1..].to_vec();
    let r = match cmd.as_str() {
        "open" => open(&rest),
        "serve" => serve(&rest),
        "run" => run(&rest),
        "render-checkpoint" => render_checkpoint(&rest),
        #[cfg(feature = "state-store")]
        "render-state" => render_state(&rest),
        #[cfg(feature = "render-snapshot")]
        "snapshot-state" => optical::export(&rest),
        #[cfg(feature = "render-snapshot")]
        "render-snapshot" => optical::render(&rest),
        #[cfg(feature = "render-snapshot")]
        "snapshot-companions" => optical::companions(&rest),
        "finish" => finish_cmd(&rest),
        "note" => note(&rest, name),
        "tubes" => tubes(&rest),
        "hash-probe" => {
            println!("{}", session::hash_probe());
            Ok(())
        }
        "help" | "-h" | "--help" => {
            print_usage();
            Ok(())
        }
        "do" | "look" | "probe" | "log" | "status" | "globals" | "save" | "frames" | "close" | "check" => client(&cmd, &rest, name),
        o => Err(format!("easel: no command {o:?}\n\n{}", usage())),
    };
    match r {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn print_usage() {
    println!("{}", usage());
    if studio::painter() {
        return;
    }
    #[cfg(feature = "state-store")]
    println!("\n  run … --state-store DIR [--state-budget-mib N]   publish each successful chunk\n  render-state DIR out.png [--mode color|gallery|relief] [--look]   open the committed revision\n  EASEL_STATE_SESSION=<name> opts that live painting into per-chunk state publication");
    #[cfg(feature = "render-snapshot")]
    println!("\n  snapshot-state DIR out.view   export compact optics of the committed revision\n  render-snapshot out.view out.png [--mode color|gallery|relief] [--light az,el,strength] [--crop x0,y0,x1,y1]\n  snapshot-companions out.view CACHE [--gallery-light az,el,strength] [--relief-light az,el,strength] [--crop x0,y0,x1,y1] [--cache-budget-mib N]");
}

fn usage() -> &'static str {
    if studio::painter() { PAINTER_USAGE } else { DEVELOPER_USAGE }
}

/// The session a command is for: `-s <name>` (taken out of `args`), else
/// `EASEL_SESSION`, else (None) the last one opened.
fn take_session(args: &mut Vec<String>) -> Result<Option<String>, String> {
    if studio::painter() {
        if let Some(a) = args.iter().find(|a| *a == "-s" || *a == "--session" || a.starts_with("--session=")) {
            return Err(format!("easel: no option {a:?}: this studio has one painting, there are no sessions to pick\n\n{}", usage()));
        }
        return Ok(Some(PAINTING.to_string()));
    }
    let mut name: Option<String> = std::env::var("EASEL_SESSION").ok();
    if let Some(i) = args.iter().position(|a| a == "-s" || a == "--session")
        && i + 1 < args.len()
    {
        name = Some(args.remove(i + 1));
        args.remove(i);
    }
    Ok(name)
}

fn session_dir(name: &str) -> PathBuf {
    root().join("out/easel").join(name)
}
fn sock_path(name: &str) -> PathBuf {
    short_sock(&session_dir(name))
}
/// The session's socket, in its directory when the path fits a sockaddr_un
/// (104 bytes on macOS, 108 on Linux), else under a private directory in the
/// temp dir (or /tmp, if the temp dir's own path is too long), named by a hash
/// of the session directory: a studio checked out deep in a tree fails to bind
/// with "path must be shorter than SUN_LEN" otherwise. If no private directory
/// is to be had, the long path stands, and bind says why.
fn short_sock(dir: &Path) -> PathBuf {
    let p = dir.join("sock");
    if p.as_os_str().len() < 100 {
        return p;
    }
    // fnv1a, not DefaultHasher: client and server must agree across Rust releases
    let file = format!("{:016x}.sock", save::fnv1a(dir.as_os_str().as_encoded_bytes()));
    // the user is whoever owns the studio (the nearest existing ancestor of the session dir)
    let owner = dir.ancestors().find_map(|a| std::fs::metadata(a).ok()).map(|m| std::os::unix::fs::MetadataExt::uid(&m));
    let Some(uid) = owner else { return p };
    for base in [std::env::temp_dir(), PathBuf::from("/tmp")] {
        let private = base.join(format!("easel-{uid}"));
        let s = private.join(&file);
        if s.as_os_str().len() < 100 && private_dir(&private, uid) {
            return s;
        }
    }
    p
}
/// Whether dir is a directory only uid can enter, making it (0700) if it is
/// missing. In a shared temp dir another user could make it first, or make
/// it a symlink: then it isn't private, and isn't used.
fn private_dir(dir: &Path, uid: u32) -> bool {
    use std::os::unix::fs::{DirBuilderExt, MetadataExt};
    let _ = std::fs::DirBuilder::new().mode(0o700).create(dir);
    match std::fs::symlink_metadata(dir) {
        Ok(m) => m.is_dir() && m.uid() == uid && m.mode() & 0o077 == 0,
        Err(_) => false,
    }
}
fn log_path(name: &str) -> PathBuf {
    root().join("paintings/lua").join(format!("{name}.lua"))
}
fn journal_path() -> PathBuf {
    root().join("notes/journal.md")
}

fn flag(args: &[String], f: &str) -> Option<String> {
    args.iter().position(|a| a == f).and_then(|i| args.get(i + 1).cloned())
}

fn valid_name(n: &str) -> Result<(), String> {
    if n.is_empty() || !n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        return Err(format!("session name {n:?}: letters, digits, _ and - only"));
    }
    Ok(())
}

// ---------------------------------------------------------------- tubes

/// `easel tubes [--markdown]`: the box a new painting here takes, as a list
/// of names or as the guide's table.
fn tubes(args: &[String]) -> Result<(), String> {
    let md = match args {
        [] => false,
        [a] if a == "--markdown" => true,
        _ => return Err("tubes [--markdown]".into()),
    };
    let b = session::box_for(None)?;
    if md {
        print!("{}", b.table());
    } else {
        for t in &b.tubes {
            println!("{}", t.name);
        }
    }
    Ok(())
}

// ---------------------------------------------------------------- journal

/// The entry `easel note` appends: a dated line, the text's further lines
/// indented under it.
fn journal_entry(clock: f64, text: &str) -> String {
    let mut lines = text.trim_end().lines();
    let first = lines.next().unwrap_or("").trim_end();
    let mut s = format!("- {}: {first}\n", time::time_of_day(clock));
    for l in lines {
        if l.trim().is_empty() {
            s.push('\n');
        } else {
            s.push_str("  ");
            s.push_str(l.trim_end());
            s.push('\n');
        }
    }
    s
}

/// Append an entry to the journal. The file only ever grows: the entry is
/// written at its end, after what is there.
fn note(args: &[String], name: Option<String>) -> Result<(), String> {
    let text = match args.first().map(|s| s.as_str()) {
        Some("-") => {
            let mut b = String::new();
            std::io::stdin().read_to_string(&mut b).map_err(|e| e.to_string())?;
            b
        }
        Some(_) => args.join(" "),
        None => return Err("note: give text".into()),
    };
    let (ok, body) = request(&current(name)?, "note", &[], text.as_bytes())?;
    if !ok {
        return Err(body);
    }
    print!("{body}");
    Ok(())
}

fn append_note(clock: f64, text: &str) -> Result<String, String> {
    if text.trim().is_empty() {
        return Err("note: empty".into());
    }
    let p = journal_path();
    std::fs::create_dir_all(p.parent().unwrap()).map_err(|e| e.to_string())?;
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&p).map_err(|e| format!("{}: {e}", p.display()))?;
    // an entry starts on a line of its own
    let len = f.metadata().map(|m| m.len()).unwrap_or(0);
    let mut entry = String::new();
    if len > 0 {
        let mut last = [0u8; 1];
        let mut r = std::fs::File::open(&p).map_err(|e| e.to_string())?;
        use std::io::Seek;
        r.seek(std::io::SeekFrom::End(-1)).map_err(|e| e.to_string())?;
        r.read_exact(&mut last).map_err(|e| e.to_string())?;
        if last[0] != b'\n' {
            entry.push('\n');
        }
    }
    entry.push_str(&journal_entry(clock, text));
    f.write_all(entry.as_bytes()).map_err(|e| e.to_string())?;
    Ok(format!("noted in {}\n", p.display()))
}

// ---------------------------------------------------------------- requests

/// The largest request the server reads (a chunk is a few KB; a note or a -f file more).
const MAX_REQUEST: usize = 64 << 20;

/// Send a request: a line with the byte length of what follows, then the head line and the
/// payload. The server reads exactly that many bytes, so it never waits for the client's end
/// of file: on macOS a client's shutdown of its sending side sometimes never reaches the
/// server (1 request in a few hundred), and a server reading to the end of file waited until
/// the client was killed (round 19: three paint calls hung for 12-31 minutes).
fn send_request(s: &mut impl Write, head: &[u8], payload: &[u8]) -> std::io::Result<()> {
    let mut buf = format!("{}\n", head.len() + payload.len()).into_bytes();
    buf.extend_from_slice(head);
    buf.extend_from_slice(payload);
    s.write_all(&buf)?;
    s.flush()
}

/// Read one request as `send_request` sent it: the head line and the payload, without
/// waiting for the end of file.
fn read_request(s: &mut impl Read) -> Result<Vec<u8>, String> {
    let mut len = Vec::new();
    let mut b = [0u8; 1];
    loop {
        let n = s.read(&mut b).map_err(|e| format!("the client didn't send its request within 60 s ({e})"))?;
        if n == 0 {
            return Err("the client closed before sending a request".into());
        }
        if b[0] == b'\n' {
            break;
        }
        if !b[0].is_ascii_digit() || len.len() >= 20 {
            return Err("the request has no length line: the easel client and server are different versions; close and reopen the easel".into());
        }
        len.push(b[0]);
    }
    let n: usize = std::str::from_utf8(&len).ok().and_then(|t| t.parse().ok()).ok_or("the request's length line is empty")?;
    if n > MAX_REQUEST {
        return Err(format!("a request of {n} bytes is more than the easel reads ({MAX_REQUEST})"));
    }
    let mut req = vec![0u8; n];
    s.read_exact(&mut req).map_err(|e| format!("the client didn't send its whole request within 60 s ({e})"))?;
    Ok(req)
}

// ---------------------------------------------------------------- client

fn request(name: &str, cmd: &str, args: &[String], payload: &[u8]) -> Result<(bool, String), String> {
    let not_running = || if studio::painter() { "no painting open: easel open".to_string() } else { format!("no easel session {name:?} running: easel open {name}") };
    let mut s = UnixStream::connect(sock_path(name)).map_err(|_| not_running())?;
    let mut head = cmd.to_string();
    for a in args {
        head.push('\t');
        head.push_str(&a.replace(['\t', '\n'], " "));
    }
    head.push('\n');
    send_request(&mut s, head.as_bytes(), payload).map_err(|e| e.to_string())?;
    let mut resp = String::new();
    s.read_to_string(&mut resp).map_err(|e| e.to_string())?;
    if resp.is_empty() {
        return Err(format!("the easel closed without answering {cmd:?}"));
    }
    let (status, body) = resp.split_once('\n').unwrap_or((&resp, ""));
    Ok((status == "ok", body.to_string()))
}

fn current(name: Option<String>) -> Result<String, String> {
    if let Some(n) = name {
        return Ok(n);
    }
    std::fs::read_to_string(root().join("out/easel/current")).map(|s| s.trim().to_string()).map_err(|_| "no session: easel open <name> first (or pass -s <name>)".to_string())
}

fn client(cmd: &str, args: &[String], name: Option<String>) -> Result<(), String> {
    let name = current(name)?;
    let mut args = args.to_vec();
    let mut payload = Vec::new();
    if cmd == "do" {
        let look = if let Some(i) = args.iter().position(|a| a == "--look") {
            args.remove(i);
            true
        } else {
            false
        };
        let target: Vec<String> = args.iter().filter(|a| *a == "--scratch" || *a == "--new").cloned().collect();
        args.retain(|a| a != "--scratch" && a != "--new");
        payload = match args.first().map(|s| s.as_str()) {
            Some("-f") => std::fs::read(args.get(1).ok_or("do -f <file>")?).map_err(|e| e.to_string())?,
            Some("-") => {
                let mut b = Vec::new();
                std::io::stdin().read_to_end(&mut b).map_err(|e| e.to_string())?;
                b
            }
            Some(src) => src.as_bytes().to_vec(),
            None => {
                return Err("do: give a chunk: easel do '<lua>' | -f file.lua | - (stdin)".into());
            }
        };
        args = if look { vec!["--look".into()] } else { vec![] };
        args.extend(target);
    }
    let (ok, body) = request(&name, cmd, &args, &payload)?;
    if ok {
        print!("{body}");
        Ok(())
    } else {
        Err(body.trim_end().to_string())
    }
}

/// The session `open` is for: the studio's painting or a developer's name.
fn open_name(args: &[String]) -> Result<String, String> {
    if studio::painter() {
        if let Some(a) = args.first() {
            return Err(format!("open: this studio has one painting; easel open takes no name or option (got {a:?})"));
        }
        return Ok(PAINTING.to_string());
    }
    let name = args.first().filter(|a| !a.starts_with('-')).ok_or("open <name>")?.clone();
    valid_name(&name)?;
    if args.len() != 1 {
        return Err("open: no width option (a log replays at its own width; a file `width` beside the easel, or EASEL_WIDTH, sets a new painting's)".into());
    }
    Ok(name)
}

/// Remember the session opened last (replay build: commands without `-s`
/// go to it).
fn set_current(name: &str) -> Result<(), String> {
    if !studio::painter() {
        std::fs::write(root().join("out/easel/current"), name).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// The session's lock: a file its one server holds locked (flock) for its whole life, and
/// an `open` while it starts that server. A session has at most one server, and only its
/// holder may remove the socket or start the server log.
fn lock_path(name: &str) -> PathBuf {
    session_dir(name).join("lock")
}

/// The session's lock, if no one holds it (None: a server, or an `open` starting one, does).
fn try_lock(name: &str) -> Result<Option<std::fs::File>, String> {
    let p = lock_path(name);
    let f = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(false).open(&p).map_err(|e| format!("{}: {e}", p.display()))?;
    match f.try_lock() {
        Ok(()) => Ok(Some(f)),
        Err(std::fs::TryLockError::WouldBlock) => Ok(None),
        Err(std::fs::TryLockError::Error(e)) => Err(format!("{}: {e}", p.display())),
    }
}

/// Set on the server `open` starts: its stdin is the session's lock, held for it.
const LOCK_ON_STDIN: &str = "EASEL_LOCK_ON_STDIN";

fn open(args: &[String]) -> Result<(), String> {
    let name = open_name(args)?;
    let dir = session_dir(&name);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    if let Ok((ok, st)) = request(&name, "status", &[], &[])
        && !replaying(&st)
    {
        if !ok {
            return Err(st);
        }
        set_current(&name)?;
        print!("reattached to {name:?}: {st}");
        return Ok(());
    }
    // Replaying a real painting can take tens of minutes. Follow the server's
    // startup log so `open` shows that work instead of looking dead, and only
    // time out when the replay itself has made no progress for longer than a chunk may run.
    // The server is started only by the `open` that takes the session's lock; any
    // other waits for it (or, if it has gone, takes the lock in turn).
    let mut started = false;
    let mut shown = 0usize;
    let mut last_progress = Instant::now();
    // a chunk may take its whole limit to replay (session::chunk_limit_for, by the width its log
    // names): no progress for that long and ten minutes more, and never less than 30 minutes, is a stall
    let width = std::fs::read_to_string(log_path(&name)).ok().and_then(|t| session::logged_width(&t).ok()).unwrap_or(LIVE_WIDTH);
    let stall = (session::chunk_limit_for(width) + Duration::from_secs(600)).max(Duration::from_secs(1800));
    loop {
        if let Some(lock) = try_lock(&name)? {
            if started {
                if let Ok(log) = std::fs::read_to_string(dir.join("server.log"))
                    && log.contains("easel: fatal")
                {
                    return Err(log);
                }
                return Err(format!("the session's server stopped without saying why; see {}", dir.join("server.log").display()));
            }
            // no server holds the session: a socket left there is a dead one's
            let _ = std::fs::remove_file(sock_path(&name));
            let log = std::fs::File::create(dir.join("server.log")).map_err(|e| e.to_string())?;
            let exe = std::env::current_exe().map_err(|e| e.to_string())?;
            use std::os::unix::process::CommandExt;
            std::process::Command::new(exe)
                .args(["serve", &name])
                .env(LOCK_ON_STDIN, "1")
                // the server holds the lock from here on (the same open file)
                .stdin(lock)
                .stdout(log.try_clone().map_err(|e| e.to_string())?)
                .stderr(log)
                .process_group(0)
                .spawn()
                .map_err(|e| format!("could not start the session: {e}"))?;
            started = true;
            shown = 0;
        }
        std::thread::sleep(Duration::from_millis(100));
        let log = std::fs::read_to_string(dir.join("server.log")).unwrap_or_default();
        if show_startup_progress(&log, &mut shown)? {
            last_progress = Instant::now();
        }
        if let Ok((true, st)) = request(&name, "status", &[], &[])
            && !replaying(&st)
        {
            let log = std::fs::read_to_string(dir.join("server.log")).unwrap_or_default();
            show_startup_progress(&log, &mut shown)?;
            set_current(&name)?;
            print!("easel {name:?} open: {st}");
            return Ok(());
        }
        if log.contains("easel: fatal") {
            #[cfg(feature = "state-store")]
            if !started && std::env::var("EASEL_STATE_SESSION").ok().as_deref() == Some(name.as_str()) {
                continue;
            }
            return Err(log);
        }
        if last_progress.elapsed() > stall {
            return Err(format!("session made no replay progress for {} minutes; see {}", stall.as_secs() / 60, dir.join("server.log").display()));
        }
    }
}

fn show_startup_progress(log: &str, shown: &mut usize) -> Result<bool, String> {
    if log.len() < *shown {
        *shown = 0;
    }
    let Some(end) = log[*shown..].rfind('\n').map(|i| *shown + i + 1) else { return Ok(false) };
    for line in log[*shown..end].lines() {
        if line.starts_with("warning: replay checkpoint") {
            eprintln!("{line}");
        } else if line.starts_with("resuming ") || line.starts_with("resumed ") || line.starts_with("reconstructed ") || line.starts_with("warning") {
            println!("{line}");
        }
    }
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    *shown = end;
    Ok(true)
}

/// Whether a `status` answer is a replay's progress (`replay_status`): the server is reopening
/// or rebuilding the painting from its log.
fn replaying(status: &str) -> bool {
    status.starts_with("rebuilding from the log (") || status.starts_with(STARTING)
}

/// `status` before the server has read the log it replays.
const STARTING: &str = "the easel is starting; retry in a moment";

/// The session's lock for `serve`, held until it exits: the one `open` passed on its
/// stdin, else taken here. A session that has a server already gets no second one.
fn serve_lock(name: &str) -> Result<std::fs::File, String> {
    let on_stdin = std::env::var_os(LOCK_ON_STDIN).is_some();
    // The signal is this server's alone: a child it starts must not take its stdin for the
    // lock. (fd 0 itself is inherited by any child all the same, not close-on-exec: a child
    // would hold the lock after the server died, and every open would wait for it. The
    // server starts no children.)
    // SAFETY: read and removed before the server starts any thread
    unsafe { std::env::remove_var(LOCK_ON_STDIN) };
    let lock = if on_stdin {
        use std::os::fd::AsFd;
        use std::os::unix::fs::MetadataExt;
        // a duplicate of stdin: the same open file, so the same lock
        let f = std::fs::File::from(std::io::stdin().as_fd().try_clone_to_owned().map_err(|e| e.to_string())?);
        let (held, want) = (f.metadata().map_err(|e| e.to_string())?, std::fs::metadata(lock_path(name)).map_err(|e| e.to_string())?);
        if (held.dev(), held.ino()) != (want.dev(), want.ino()) {
            return Err(format!("{LOCK_ON_STDIN} is set but stdin isn't {}", lock_path(name).display()));
        }
        // already ours (the open file `open` locked): this doesn't wait
        f.try_lock().map_err(|e| format!("{}: {e}", lock_path(name).display()))?;
        f
    } else {
        std::fs::create_dir_all(session_dir(name)).map_err(|e| e.to_string())?;
        match try_lock(name)? {
            Some(f) => f,
            None => {
                let pid = std::fs::read_to_string(lock_path(name)).unwrap_or_default();
                return Err(format!("another easel serves session {name:?} (pid {}); nothing ran", pid.trim()));
            }
        }
    };
    // who holds it, for anyone looking
    let mut f = &lock;
    let _ = f.set_len(0).and_then(|_| writeln!(f, "{}", std::process::id()));
    Ok(lock)
}

// ---------------------------------------------------------------- server

struct Server {
    name: String,
    // Captured at session startup, before a later rebuild can replace the executable.
    build: u64,
    s: Session,
    frames: bool,
    replay: replay_checkpoint::Live,
    hand_frames: Option<frames::Recorder>,
    /// The log text as the easel last wrote (or read) it: if the file on
    /// disk differs, it was changed outside the session.
    written: Option<String>,
    /// How many of the log's chunks the state was replayed from (at reopen or rebuild),
    /// not painted live.
    replayed: usize,
    /// The scratch canvas beside the painting (`--scratch`): a session of its own (its own
    /// log, canvas, palette, brushes and Lua state), set up as a copy of the painting's
    /// canvas, with its own clock: time spent or waited there doesn't pass for the painting.
    /// None until first used.
    scratch: Option<Box<Server>>,
    display: Option<std::rc::Rc<display::Pool>>,
    #[cfg(feature = "state-store")]
    committed: Option<painting_store::PaintingStore>,
    #[cfg(feature = "state-store")]
    state_failed: bool,
}

/// The scratch canvas's session beside the session `name`.
fn scratch_name(name: &str) -> String {
    if name == "painting" { "scratch".to_string() } else { format!("{name}-scratch") }
}


fn serve(args: &[String]) -> Result<(), String> {
    studio::serving();
    let name = args.first().ok_or("serve <name>")?.clone();
    valid_name(&name)?;
    if studio::painter() && name != PAINTING {
        return Err(format!("easel: fatal: this studio has one painting, {PAINTING:?}"));
    }
    if args.len() != 1 {
        return Err("easel: fatal: no width option (a log replays at its own width; a file `width` beside the easel, or EASEL_WIDTH, sets a new painting's)".into());
    }
    let _lock = serve_lock(&name).map_err(|e| format!("easel: fatal: {e}"))?;
    let sock = sock_path(&name);
    // the lock is ours: a socket there is a dead server's
    let _ = std::fs::remove_file(&sock);
    let l = UnixListener::bind(&sock).map_err(|e| format!("easel: fatal: bind {}: {e}", sock.display()))?;
    // the log replays with `status` answering how far it has got; other requests wait for it
    let (done, total) = (std::sync::atomic::AtomicUsize::new(0), std::sync::atomic::AtomicUsize::new(usize::MAX));
    let resumed = serving_status(&l, || replay_status(&done, &total), None, || {
        Server::resume(name.clone(), &|k, n| {
            total.store(n, std::sync::atomic::Ordering::Release);
            done.store(k, std::sync::atomic::Ordering::Release);
        })
    });
    let (mut srv, waiting) = match resumed {
        Ok((Ok(srv), waiting)) => (srv, waiting),
        Ok((Err(e), waiting)) => {
            let _ = std::fs::remove_file(&sock);
            for (mut conn, _) in waiting {
                let _ = conn.write_all(format!("err
{e}
").as_bytes());
            }
            return Err(format!("easel: fatal: {e}"));
        }
        Err(e) => {
            let _ = std::fs::remove_file(&sock);
            return Err(format!("easel: fatal: {e}"));
        }
    };
    let _ = std::io::stdout().flush();
    // the check running on its own thread, if any (check.rs; replay build only)
    let mut checking: Option<check::Job> = None;
    // requests that came while the log replayed or a chunk ran, read and waiting their turn
    let mut pending = std::collections::VecDeque::from(waiting);

    loop {
        let (mut conn, req) = match pending.pop_front() {
            Some(p) => p,
            None => {
                let Ok((mut conn, _)) = l.accept() else { continue };
                // a request is a few KB sent at once: a client that stalls mid-request must not hold
                // the one thread that serves every request
                let _ = conn.set_read_timeout(Some(Duration::from_secs(60)));
                match read_request(&mut conn) {
                    Ok(r) => {
                        let _ = conn.set_read_timeout(None);
                        (conn, r)
                    }
                    Err(e) => {
                        let _ = conn.write_all(format!("err\nthe easel couldn't read the request: {e}\n").as_bytes());
                        eprintln!("request dropped: {e}");
                        continue;
                    }
                }
            }
        };
        let text = String::from_utf8_lossy(&req).to_string();
        let (head, payload) = text.split_once('\n').unwrap_or((&text, ""));
        let mut parts = head.split('\t').map(|s| s.to_string());
        let cmd = parts.next().unwrap_or_default();
        let args: Vec<String> = parts.collect();
        // a client that gave up (timed out) waiting behind a long request: its request
        // doesn't run (a `do` it no longer waits for would go into the log unseen)
        if check::client_gone(&conn) {
            eprintln!("{cmd} skipped: the client went away before it ran");
            continue;
        }
        if cmd == "check" {
            if let Some(j) = checking.take() {
                j.stop();
            }
            match srv.check_input().and_then(|input| check::start(conn.try_clone().map_err(|e| e.to_string())?, input)) {
                Ok(j) => checking = Some(j),
                Err(e) => {
                    let _ = conn.write_all(format!("err\n{e}\n").as_bytes());
                    eprintln!("check 0.00s err");
                }
            }
            continue;
        }
        let t0 = Instant::now();
        let r = if cmd == "do" {
            // while the chunk runs, `status` answers how far it has got; other requests wait
            session::PROGRESS.clear();
            let (name, written) = (srv.name.clone(), srv.written.clone().unwrap_or_default());
            // once the chunk is kept, the log on disk has grown past `written`: `report` says so
            let status = || {
                if session::PROGRESS.kept() == 0 {
                    validate_log(&name, &written)?;
                }
                Ok(session::PROGRESS.report(t0.elapsed()))
            };
            match serving_status(&l, status, None, || srv.handle(&cmd, &args, payload)) {
                Ok((r, waiting)) => {
                    pending.extend(waiting);
                    r
                }
                Err(e) => Err(e),
            }
        } else {
            srv.handle(&cmd, &args, payload)
        };
        let reply = match &r {
            Ok(b) => format!("ok\n{b}"),
            Err(e) => format!("err\n{e}\n"),
        };
        let _ = conn.write_all(reply.as_bytes());
        eprintln!("{cmd} {:.2}s {}", t0.elapsed().as_secs_f64(), if r.is_ok() { "ok" } else { "err" });
        #[cfg(feature = "state-store")]
        if srv.state_failed {
            let _ = std::fs::remove_file(&sock);
            return Err("easel: fatal: state publication or journal export failed; reopen and inspect the accepted chunk count before retrying".into());
        }
        if cmd == "close" && r.is_ok() {
            if let Some(j) = checking.take() {
                j.stop();
            }
            for (mut conn, _) in pending.drain(..) {
                let _ = conn.write_all(b"err\nthe easel closed before this request ran\n");
            }
            let _ = std::fs::remove_file(&sock);
            break;
        }
        // the same for the scratch canvas, at once (its log is short)
        if let Some(sc) = srv.scratch.as_mut()
            && sc.s.stale
        {
            if let Err(e) = sc.rebuild(&|_, _| {}) {
                eprintln!("warning: the scratch canvas couldn't be rebuilt ({e}); it is put away");
                srv.scratch = None;
            }
        }
        // a failed chunk left the state inexact (session.rs `stale`): rebuild it from the log
        // after the reply; status stays available and other requests are refused until done
        if srv.s.stale {
            drop(conn);
            let t0 = Instant::now();
            eprintln!("rebuilding from the log ({} chunks)", srv.s.log.len());
            if let Err(e) = rebuild_serving_status(&l, &mut srv) {
                let _ = std::fs::remove_file(&sock);
                return Err(format!("easel: fatal: {e}"));
            }
            eprintln!("rebuilt {:.2}s", t0.elapsed().as_secs_f64());
        }
    }
    Ok(())
}

/// What a request other than `status` gets while the log replays after a failed chunk.
const REBUILDING: &str = "the easel is rebuilding from the log; retry after it finishes";

/// `status` while the log replays: how many of its chunks have (`total` unset: not begun).
fn replay_status(done: &std::sync::atomic::AtomicUsize, total: &std::sync::atomic::AtomicUsize) -> Result<String, String> {
    use std::sync::atomic::Ordering;
    match total.load(Ordering::Acquire) {
        usize::MAX => Err(STARTING.into()),
        n => Ok(format!("rebuilding from the log ({} of {n} chunks)", done.load(Ordering::Acquire))),
    }
}

/// Rebuild the session from its log (session.rs `stale`), answering `status` with its progress.
fn rebuild_serving_status(listener: &UnixListener, srv: &mut Server) -> Result<(), String> {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let (done, total) = (AtomicUsize::new(0), AtomicUsize::new(srv.s.log.len()));
    let (name, written) = (srv.name.clone(), srv.written.clone().ok_or("session integrity: uninitialized log")?);
    let status = || validate_log(&name, &written).and_then(|()| replay_status(&done, &total));
    serving_status(listener, status, Some(REBUILDING), || srv.rebuild(&|k, _| done.store(k, Ordering::Release)))?.0
}

/// Requests read while `work` ran (`serving_status`), each with its connection, in the order they came.
type Waiting = Vec<(UnixStream, Vec<u8>)>;

/// Run `work` on this thread while another thread answers `status` on the listener with
/// `status()`. Any other request gets `refusal`, or with none is read and handed back, in the
/// order it came, to run after `work`. Only the socket and what `status` reads cross threads:
/// Lua (and its Rc-backed engine state) stays on this one.
fn serving_status<R>(listener: &UnixListener, status: impl Fn() -> Result<String, String> + Sync, refusal: Option<&str>, work: impl FnOnce() -> R) -> Result<(R, Waiting), String> {
    use std::sync::atomic::{AtomicBool, Ordering};
    let done = AtomicBool::new(false);
    struct Stop<'a>(&'a AtomicBool);
    impl Drop for Stop<'_> {
        fn drop(&mut self) {
            self.0.store(true, Ordering::Release);
        }
    }
    // An absolute request deadline also bounds clients that trickle bytes; a request to be
    // refused is given up when `work` ends, one to be handed back is read to its end.
    struct Reader<'a> {
        conn: &'a mut UnixStream,
        until: Instant,
        done: Option<&'a AtomicBool>,
    }
    impl Read for Reader<'_> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let remaining = self.until.saturating_duration_since(Instant::now());
            if remaining.is_zero() || self.done.is_some_and(|d| d.load(Ordering::Acquire)) {
                return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "request deadline reached"));
            }
            self.conn.set_read_timeout(Some(remaining))?;
            self.conn.read(buf)
        }
    }
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    let result: Result<_, String> = std::thread::scope(|scope| {
        let worker = scope.spawn(|| -> Result<Waiting, String> {
            let mut kept = Vec::new();
            while !done.load(Ordering::Acquire) {
                let mut conn = match listener.accept() {
                    Ok((conn, _)) => conn,
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::park_timeout(Duration::from_millis(5));
                        continue;
                    }
                    Err(e) => return Err(e.to_string()),
                };
                // A stalled reader/writer must neither block status indefinitely nor
                // keep the scoped helper alive after `work` finishes.
                conn.set_nonblocking(false).map_err(|e| e.to_string())?;
                let _ = conn.set_write_timeout(Some(Duration::from_millis(100)));
                let wait = if refusal.is_some() { Duration::from_millis(100) } else { Duration::from_secs(10) };
                let mut reader = Reader { conn: &mut conn, until: Instant::now() + wait, done: refusal.is_some().then_some(&done) };
                let req = match read_request(&mut reader) {
                    Ok(req) => req,
                    Err(e) => {
                        let _ = conn.write_all(format!("err\n{e}\n").as_bytes());
                        continue;
                    }
                };
                let head = req.split(|b| *b == b'\n').next().unwrap_or_default();
                let reply = if head.split(|b| *b == b'\t').next() == Some(b"status".as_slice()) {
                    match status() {
                        Ok(s) => format!("ok\n{s}\n"),
                        Err(e) => format!("err\n{e}\n"),
                    }
                } else if let Some(r) = refusal {
                    format!("err\n{r}\n")
                } else {
                    let _ = conn.set_read_timeout(None);
                    let _ = conn.set_write_timeout(None);
                    kept.push((conn, req));
                    continue;
                };
                let _ = conn.write_all(reply.as_bytes());
            }
            Ok(kept)
        });
        let stop = Stop(&done);
        let r = work();
        drop(stop);
        worker.thread().unpark();
        let kept = worker.join().map_err(|_| "the status listener panicked".to_string())??;
        Ok((r, kept))
    });
    let blocking = listener.set_nonblocking(false).map_err(|e| e.to_string());
    let r = result?;
    blocking?;
    Ok(r)
}

/// The session's committed record: the log as the session last wrote it.
fn witness_path(name: &str) -> PathBuf {
    session_dir(name).join("committed.lua")
}

#[cfg(feature = "state-store")]
fn export_journal(path: &Path, source: &str) -> Result<(), String> {
    let dir = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let mut file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    file.write_all(source.as_bytes()).and_then(|_| file.sync_all()).map_err(|e| e.to_string())?;
    std::fs::File::open(dir).and_then(|f| f.sync_all()).map_err(|e| e.to_string())
}

/// Put a session's log aside as `<name>-N.lua` (the first N free) and drop its committed
/// record, so a fresh session of that name starts empty. The path it went to, if any.
fn put_aside(name: &str) -> Result<Option<PathBuf>, String> {
    let lp = log_path(name);
    let _ = std::fs::remove_file(witness_path(name));
    if !lp.exists() {
        return Ok(None);
    }
    let mut n = 1;
    let to = loop {
        let p = log_path(&format!("{name}-{n}"));
        if !p.exists() {
            break p;
        }
        n += 1;
    };
    std::fs::rename(&lp, &to).map_err(|e| format!("{}: {e}", lp.display()))?;
    Ok(Some(to))
}

/// The log and the committed record on disk both hold `expected`.
fn validate_log(name: &str, expected: &str) -> Result<(), String> {
    for p in [log_path(name), witness_path(name)] {
        let actual = std::fs::read(&p).map_err(|e| format!("session integrity: {} can't be read ({e}). The easel goes on only from the log it wrote; nothing ran.", p.display()))?;
        if actual != expected.as_bytes() {
            return Err(format!("session integrity: {} differs from the log the session wrote. The easel goes on only from the log it wrote; nothing ran.", p.display()));
        }
    }
    Ok(())
}

/// The palette look (look.rs `palette`): the piles the globals hold, over this canvas's
/// ground. Only reads: no hand time, nothing in the log, the canvas and state untouched.
fn palette_look(s: &Session) -> Result<(usize, usize, Vec<u8>), String> {
    let ground = s.ground_color().ok_or("no canvas yet: the first chunk is canvas{...}")?;
    look::palette(&s.piles(), ground)
}

/// Half the side (units) of the passage a held knife is seen against, when no crop is given.
const HOLD_HALF: f32 = 120.0;

/// SPECULATIVE (notes/open-questions.md). The loaded knife held up to the canvas
/// (`look --hold <knife or pile> --at x,y`): a passage of the canvas at full detail
/// (`--crop`, or 240 units square around the point) with a knife held over it, the
/// blade's end at the point, so its paint meets the picture there in one light and one
/// surround. The knife is a knife global with what is on it (a mix scraped off the
/// canvas too), or a pile's name: a fresh load from its heap as it is on the board now.
/// The engine lays that paint thick on a steel blade at the painting's scale and engine,
/// and the blade is seen as the passage is: lit, in grays or squinted with it.
/// It shows the paint on the knife and nothing of how it would look laid: not thinned by
/// a brush, not mixed into what is wet there, not over what is under it, not dried.
/// Only reads: no hand time, nothing in the log, the canvas and state untouched.
fn hold_look(s: &Session, name: &str, at: (f32, f32), v: &look::View) -> Result<(usize, usize, Vec<u8>, [f32; 4]), String> {
    use image::ImageEncoder;
    if v.size.is_some() || v.grid.is_some() || v.mirror || v.edges || v.palette {
        return Err("look --hold takes --at, --crop, --mask, --mode value, squint, relief or gallery and --light, nothing else".into());
    }
    let (paint, blade, full) = s.held(name)?;
    let c = s.canvas().ok_or("no canvas yet: the first chunk is canvas{...}")?;
    let (wu, hu) = (c.width(), c.height());
    if !(at.0.is_finite() && at.1.is_finite() && (0.0..=wu).contains(&at.0) && (0.0..=hu).contains(&at.1)) {
        return Err(format!("look --hold: --at {},{} is not on the canvas ({wu} x {hu} units)", at.0, at.1));
    }
    let crop = v.crop.unwrap_or([(at.0 - HOLD_HALF).max(0.0), (at.1 - HOLD_HALF).max(0.0), (at.0 + HOLD_HALF).min(wu), (at.1 + HOLD_HALF).min(hu)]);
    if !(crop[0] <= at.0 && at.0 <= crop[2] && crop[1] <= at.1 && at.1 <= crop[3]) {
        return Err(format!("look --hold: --at {},{} lies outside the --crop", at.0, at.1));
    }
    let seen_as = |crop: [f32; 4]| look::View { crop: Some(crop), value: v.value, squint: v.squint, light: v.light, ..look::View::default() };
    let mut passage_view = seen_as(crop);
    passage_view.overlays = v.overlays.clone();
    let (_, _, png) = look::render(&c, &passage_view)?;
    let mut passage = image::load_from_memory(&png).map_err(|e| e.to_string())?.to_rgb8();
    // the knife: its paint laid thick on a steel blade, on a board of its own with the
    // painting's pixels to the unit, millimetres to the unit and engine
    let f = c.frame();
    let (len, wide) = (2.0 * blade, blade + 12.0);
    let board_h = wide + 20.0;
    let mut board = Canvas::new(f.full_w.max(16), 1000.0 / board_h, paint::hex("#9aa0a6")).with_size_mm(1000.0 * c.mm_per_unit()).with_engine(c.engine()).with_drain(c.drain());
    let mut knife = paint::Knife::new(blade);
    knife.load(paint, full);
    let cy = board_h / 2.0;
    // (the pull starts before the part shown, so the paint comes to the blade's very end)
    board.knife(&mut knife, &[(10.0, cy), (50.0 + len, cy)], (0.15, 0.1), None, true, 0.25);
    let (_, _, bpng) = look::render(&board, &seen_as([30.0, cy - wide / 2.0, 30.0 + len, cy + wide / 2.0]))?;
    let blade_img = image::load_from_memory(&bpng).map_err(|e| e.to_string())?.to_rgb8();
    // held over the passage: the blade's end at the point, its length to the right. The
    // passage may be shown smaller than the canvas's own pixels (a crop of a painting wider
    // than 2400 px, look.rs LOOK_SCALE; the blade is shown so too): the point is placed in
    // the passage as shown
    let px = |u: f32, lo: usize, hi: usize| ((u * f.scale).round().max(0.0) as usize).clamp(lo, hi) as i64;
    let (x0, x1) = (px(crop[0], f.x0, f.x0 + f.w), px(crop[2], f.x0, f.x0 + f.w));
    let (y0, y1) = (px(crop[1], f.y0, f.y0 + f.h), px(crop[3], f.y0, f.y0 + f.h));
    let (kx, ky) = (passage.width() as f32 / (x1 - x0) as f32, passage.height() as f32 / (y1 - y0) as f32);
    let shown = |n: i64| if f.scale > 2.4 { ((n as f32 * 2.4 / f.scale).round() as u32).max(1) } else { n as u32 };
    let ratio = passage.width().max(passage.height()) as f32 / shown((x1 - x0).max(y1 - y0)) as f32;
    let blade_img = if ratio < 1.0 {
        image::imageops::resize(&blade_img, (blade_img.width() as f32 * ratio).round().max(1.0) as u32, (blade_img.height() as f32 * ratio).round().max(1.0) as u32, image::imageops::FilterType::Lanczos3)
    } else {
        blade_img
    };
    let x = (((at.0 * f.scale).round() as i64 - x0) as f32 * kx).round() as i64;
    let y = (((at.1 * f.scale).round() as i64 - y0) as f32 * ky).round() as i64 - blade_img.height() as i64 / 2;
    image::imageops::overlay(&mut passage, &blade_img, x, y);
    let (w, h) = (passage.width(), passage.height());
    let mut out = Vec::new();
    image::codecs::png::PngEncoder::new(&mut out).write_image(passage.as_raw(), w, h, image::ExtendedColorType::Rgb8).map_err(|e| e.to_string())?;
    Ok((w as usize, h as usize, out, crop))
}

/// With the paper-and-pastel switch: a pastel stick held over the canvas
/// (`look --hold <stick> --at x,y [--pose force,alt,azimuth[,roll] | --side
/// direction]`): the passage at full detail with, seen from above, the
/// stick's low part as a light shadow, the band where its crumbs would settle
/// tinted with its color, and where it rests on the tooth at that force in
/// its color: where a stroke from there would lay. `--side` lays it flat
/// across a stroke going that way (degrees). Only reads: no hand time,
/// nothing in the log, the canvas and its wear untouched.
fn stick_hold_look(s: &Session, name: &str, at: (f32, f32), pose: [f32; 4], v: &look::View) -> Result<(usize, usize, Vec<u8>, String), String> {
    use image::ImageEncoder;
    if v.size.is_some() || v.grid.is_some() || v.mirror || v.palette {
        return Err("look --hold takes --at, --pose or --side, --crop, --mask, --mode value, squint, relief or gallery and --light, nothing else".into());
    }
    let (stick, turned) = s.held_stick(name)?;
    let c = s.canvas().ok_or("no canvas yet: the first chunk is canvas{...}")?;
    let (wu, hu) = (c.width(), c.height());
    if !(at.0.is_finite() && at.1.is_finite() && (0.0..=wu).contains(&at.0) && (0.0..=hu).contains(&at.1)) {
        return Err(format!("look --hold: --at {},{} is not on the canvas ({wu} x {hu} units)", at.0, at.1));
    }
    let half = 60.0f32;
    let crop = v.crop.unwrap_or([(at.0 - half).max(0.0), (at.1 - half).max(0.0), (at.0 + half).min(wu), (at.1 + half).min(hu)]);
    // (as `hold_look`: the point in the passage shown)
    if !(crop[0] <= at.0 && at.0 <= crop[2] && crop[1] <= at.1 && at.1 <= crop[3]) {
        return Err(format!("look --hold: --at {},{} lies outside the --crop", at.0, at.1));
    }
    let seen_as = look::View { crop: Some(crop), value: v.value, squint: v.squint, light: v.light, overlays: v.overlays.clone(), ..look::View::default() };
    let (_, _, png) = look::render(&c, &seen_as)?;
    let mut img = image::load_from_memory(&png).map_err(|e| e.to_string())?.to_rgb8();
    let p = paint::pastel::Pose { force: pose[0], alt: pose[1].to_radians(), az: pose[2].to_radians(), roll: (turned + pose[3]).to_radians() };
    let cells = c.stick_contact(&stick, at.0, at.1, p);
    let f = c.window();
    // (the passage as rendered: a crop past the canvas's edge is clipped, as in `hold_look`)
    let px = |u: f32, lo: usize, hi: usize| ((u * f.scale).round().max(0.0) as usize).clamp(lo, hi);
    let (cx0, cx1) = (px(crop[0], f.x0, f.x0 + f.w), px(crop[2], f.x0, f.x0 + f.w));
    let (cy0, cy1) = (px(crop[1], f.y0, f.y0 + f.h), px(crop[3], f.y0, f.y0 + f.h));
    let (kx, ky) = (img.width() as f32 / (cx1 - cx0) as f32, img.height() as f32 / (cy1 - cy0) as f32);
    let mut overlay = vec![0u8; img.width() as usize * img.height() as usize];
    let col = stick.color.map(|v| (paint::color::linear_to_srgb(v) * 255.0).round() as f32);
    let (mut touch, mut bed) = (0usize, 0usize);
    for &(i, k) in &cells {
        let (x, y) = (i % f.w + f.x0, i / f.w + f.y0);
        if x < cx0 || y < cy0 || x >= cx1 || y >= cy1 {
            continue;
        }
        match k {
            2 => touch += 1,
            1 => bed += 1,
            _ => {}
        }
        let (x0, x1) = (((x - cx0) as f32 * kx).floor() as usize, (((x + 1 - cx0) as f32 * kx).ceil() as usize).min(img.width() as usize));
        let (y0, y1) = (((y - cy0) as f32 * ky).floor() as usize, (((y + 1 - cy0) as f32 * ky).ceil() as usize).min(img.height() as usize));
        for y in y0..y1 {
            for x in x0..x1 {
                let q = &mut overlay[y * img.width() as usize + x];
                *q = (*q).max(k + 1);
            }
        }
    }
    for (q, k) in img.pixels_mut().zip(overlay) {
        let (share, to): (f32, [f32; 3]) = match k {
            3 => (1.0, col),
            2 => (0.45, col),
            1 => (0.22, [20.0, 20.0, 20.0]),
            _ => continue,
        };
        for ch in 0..3 {
            q[ch] = (q[ch] as f32 * (1.0 - share) + to[ch] * share).round() as u8;
        }
    }
    let mm2 = c.px_mm() * c.px_mm();
    let note = format!("touching {:.1} mm², crumbs settle over {:.1} mm² more", touch as f32 * mm2, bed as f32 * mm2);
    let (w, h) = (img.width(), img.height());
    let mut out = Vec::new();
    image::codecs::png::PngEncoder::new(&mut out).write_image(img.as_raw(), w, h, image::ExtendedColorType::Rgb8).map_err(|e| e.to_string())?;
    Ok((w as usize, h as usize, out, note))
}

/// Write a look as `dir/look-NNNN.png`, NNNN one above the highest there, never over a file
/// that exists (a pruned look or a stray look-prefixed file doesn't make it reuse a name).
fn new_look(dir: &Path, png: &[u8]) -> Result<PathBuf, String> {
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let number = |name: &str| name.strip_prefix("look-")?.strip_suffix(".png")?.parse::<u64>().ok();
    let mut n = std::fs::read_dir(dir).map_err(|e| e.to_string())?.filter_map(|e| number(&e.ok()?.file_name().to_string_lossy())).max().unwrap_or(0);
    for preset in look::DISPLAY_PRESETS {
        if let Ok(entries) = std::fs::read_dir(dir.join(preset.folder)) {
            n = n.max(entries.filter_map(|e| number(&e.ok()?.file_name().to_string_lossy())).max().unwrap_or(0));
        }
    }
    loop {
        n = n.checked_add(1).ok_or_else(|| format!("{}: no look number above look-{n}.png is left", dir.display()))?;
        let p = dir.join(format!("look-{n:04}.png"));
        match std::fs::OpenOptions::new().write(true).create_new(true).open(&p) {
            Ok(mut f) => {
                if let Err(e) = f.write_all(png).and_then(|_| f.sync_all()) {
                    let _ = std::fs::remove_file(&p);
                    return Err(format!("{}: {e}", p.display()));
                }
                return Ok(p);
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(format!("{}: {e}", p.display())),
        }
    }
}

impl Server {
    // Independent local witness, not a signature or an access-control boundary.
    // Editing the log alone is detected; coordinated edits of both files or
    // restoring the entire directory are outside this local integrity model.
    fn witness(&self) -> PathBuf {
        witness_path(&self.name)
    }

    fn validate(&self) -> Result<(), String> {
        #[cfg(feature = "state-store")]
        if let Some(store) = &self.committed {
            if self.state_failed { return Err("session integrity: state publication requires a restart".into()); }
            store.validate_source(self.written.as_deref().ok_or("session integrity: uninitialized log")?).map_err(|e| format!("state integrity: {e}"))?;
        }
        validate_log(&self.name, self.written.as_deref().ok_or("session integrity: uninitialized log")?)
    }

    /// Reopen the session `name` from its log, reporting the chunks replayed (`progress`:
    /// done, of how many; zero first).
    fn resume(name: String, progress: &dyn Fn(usize, usize)) -> Result<Self, String> {
        let display = if display::enabled() { Some(std::rc::Rc::new(display::Pool::configured()?)) } else { None };
        Self::resume_as(name, None, display, progress)
    }

    /// `resume`; a new session's box and width are `like`'s when given (the scratch canvas
    /// takes the painting's), else the configured box and the name's width.
    fn resume_as(name: String, like: Option<&Session>, display: Option<std::rc::Rc<display::Pool>>, progress: &dyn Fn(usize, usize)) -> Result<Self, String> {
        let lp = log_path(&name);
        #[cfg(feature = "state-store")]
        let committed = if like.is_none() && std::env::var("EASEL_STATE_SESSION").ok().as_deref() == Some(name.as_str()) {
            let budget = painting_store::PaintingStore::budget(std::env::var("EASEL_STATE_DISK_MIB").ok().as_deref()).map_err(|e| e.to_string())?;
            Some(painting_store::PaintingStore::open(session_dir(&name).join("state"), budget, painting_store::PaintingStore::runtime_identity().map_err(|e| e.to_string())?).map_err(|e| format!("state store: {e}"))?)
        } else { None };
        #[cfg(feature = "state-store")]
        if let Some(source) = committed.as_ref().map(|s| s.source()).transpose().map_err(|e| e.to_string())?.flatten() {
            for path in [&lp, &witness_path(&name)] {
                match std::fs::read(path) {
                    Ok(bytes) if !source.as_bytes().starts_with(&bytes) => return Err(format!("state integrity: {} is not an export of the accepted source", path.display())),
                    Ok(_) => {}
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                    Err(e) => return Err(e.to_string()),
                }
                export_journal(path, &source)?;
            }
        }
        let text = if lp.exists() || witness_path(&name).exists() { Some(std::fs::read_to_string(&lp).map_err(|e| format!("session integrity: {e}"))?) } else { None };
        if let Some(t) = &text {
            validate_log(&name, t)?;
        }
        // an existing painting goes on with the box its log names; a new one takes the
        // configured box (session::box_for)
        let (tubes, width) = match (like, &text) {
            (Some(l), None) => ((*l.st.borrow().tubes).clone(), l.st.borrow().width),
            _ => (session::box_for(text.as_deref())?, width_for(&name, text.as_deref())?),
        };
        // the state store's budgets are a 2400 px painting's (a 4800 px state is about four times as large)
        #[cfg(feature = "state-store")]
        if committed.is_some() && width > LIVE_WIDTH {
            return Err(format!("EASEL_STATE_SESSION keeps paintings of {LIVE_WIDTH} px; this one is {width} px"));
        }
        #[cfg(feature = "state-store")]
        let width = committed.as_ref().and_then(|s| s.current.as_ref()).map(|(_, m)| m.painting_width).unwrap_or(width);
        let executable = std::env::current_exe().and_then(std::fs::read).map_err(|e| format!("read build identity: {e}"))?;
        let frames = like.is_none() && name != "scratch" && !name.ends_with("-scratch") && !name.starts_with("sketch");
        let mut srv = Self { name, build: save::fnv1a(&executable), s: Session::with_box(width, tubes).map_err(|e| e.to_string())?, frames, replay: replay_checkpoint::Live::new(), hand_frames: None, written: None, replayed: 0, scratch: None, display,
            #[cfg(feature = "state-store")] committed,
            #[cfg(feature = "state-store")] state_failed: false,
        };
        // the paper-and-pastel switch: a log's own; a new painting's on, a scratch canvas's
        // its painting's
        let paper = match &text {
            Some(text) => session::logged_paper(text),
            None => like.is_none_or(|s| s.st.borrow().paper),
        };
        srv.s.set_paper(paper).map_err(|e| e.to_string())?;
        if let Some(text) = text {
            srv.written = Some(text.clone());
            srv.replay_log(&text, progress)?;
            #[cfg(feature = "state-store")]
            if let Some(store) = &mut srv.committed { store.publish(&srv.s, &text).map_err(|e| format!("state integrity: {e}"))?; }
        } else {
            srv.start_hand_frames(None)?;
            srv.save_log()?;
        }
        Ok(srv)
    }

    fn rebuild(&mut self, progress: &dyn Fn(usize, usize)) -> Result<(), String> {
        self.validate()?;
        let text = self.written.clone().ok_or("session integrity: uninitialized log")?;
        let (width, paper) = { let st = self.s.st.borrow(); (st.width, st.paper) };
        let mut fresh = Session::with_box(width, self.s.tube_box()).and_then(|s| s.with_paper(paper)).map_err(|e| e.to_string())?;
        fresh.chunk_limit = self.s.chunk_limit;
        self.s = fresh;
        self.replay = replay_checkpoint::Live::new();
        self.replay_log(&text, progress)
    }

    fn replay_log(&mut self, text: &str, progress: &dyn Fn(usize, usize)) -> Result<(), String> {
        let chunks = parse_program(text);
        let width = self.s.st.borrow().width;
        progress(0, chunks.len());
        self.s.set_replaying(true);
        let checkpoint = session_dir(&self.name).join("replay.ckpt");
        let resumed = self.replay.resume(&checkpoint, text, width, &mut self.s, &chunks, &self.name, progress)
            .and_then(|done| { self.start_hand_frames(Some(done))?; Ok(done) });
        let done = match resumed {
            Ok(done) => done,
            Err(e) => {
                eprintln!("warning: replay checkpoint: {e}; replaying the full log");
                let limit = self.s.chunk_limit;
                self.s = Session::with_box(width, self.s.tube_box()).and_then(|s| s.with_paper(session::logged_paper(text))).map_err(|e| e.to_string())?;
                self.s.chunk_limit = limit;
                self.s.set_replaying(true);
                self.replay = replay_checkpoint::Live::new();
                self.start_hand_frames(None)?;
                progress(0, chunks.len());
                0
            }
        };
        for (i, chunk) in chunks.iter().enumerate().skip(done) {
            let t0 = Instant::now();
            eprintln!("resuming chunk {}/{}", i + 1, chunks.len());
            self.run_chunk(chunk).map_err(|e| format!("session integrity: replay failed at chunk {}: {e}; refusing partial session", i + 1))?;
            if let Err(e) = self.record_replay() { eprintln!("warning: chunk committed, replay checkpoint/frames not saved: {e}"); }
            eprintln!("resumed chunk {}/{} {:.2}s", i + 1, chunks.len(), t0.elapsed().as_secs_f64());
            progress(i + 1, chunks.len());
        }
        self.s.set_replaying(false);
        self.replayed = chunks.len();
        if self.s.program(&self.name) != text {
            return Err("session integrity: noncanonical or incomplete log; refusing replay".into());
        }
        println!("resumed {} chunks from {}", self.s.log.len(), log_path(&self.name).display());
        Ok(())
    }

    /// Append only the new suffix. Commit the redundant witness afterward.
    /// A crash between writes fails closed on reopen, never partial replay.
    fn save_log(&mut self) -> Result<String, String> {
        let p = log_path(&self.name);
        let text = self.s.program(&self.name);
        let changed = self.written.as_deref() != Some(text.as_str());
        #[cfg(feature = "state-store")]
        if self.committed.is_some() {
            if self.written.is_some() { self.validate()?; }
            if let Err(e) = self.committed.as_mut().unwrap().publish(&self.s, &text) {
                self.state_failed = true;
                return Err(format!("state publication failed: {e}; reopen and inspect the accepted chunk count before retrying"));
            }
            self.written = Some(text.clone());
            for path in [p, self.witness()] {
                if let Err(e) = export_journal(&path, &text) {
                    self.state_failed = true;
                    eprintln!("state store: chunk is committed but journal export needs recovery: {e}");
                }
            }
            if changed {
                self.record_frame();
                if let Err(e) = self.record_replay() { eprintln!("warning: chunk committed, replay checkpoint/frames not saved: {e}"); }
            }
            return Ok(String::new());
        }
        let previous = self.written.as_deref().unwrap_or("");
        if self.written.is_some() {
            self.validate()?;
        }
        let suffix = text.strip_prefix(previous).ok_or("session integrity: log would rewrite history")?;
        std::fs::create_dir_all(p.parent().unwrap()).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(session_dir(&self.name)).map_err(|e| e.to_string())?;
        let mut opts = std::fs::OpenOptions::new();
        opts.append(true);
        if self.written.is_none() {
            opts.create_new(true);
        }
        let mut f = opts.open(&p).map_err(|e| format!("session integrity: {e}"))?;
        f.write_all(suffix.as_bytes()).and_then(|_| f.sync_all()).map_err(|e| format!("session integrity: {e}"))?;
        let pending = self.witness().with_extension("pending");
        let mut f = std::fs::File::create(&pending).map_err(|e| e.to_string())?;
        f.write_all(text.as_bytes()).and_then(|_| f.sync_all()).map_err(|e| e.to_string())?;
        std::fs::rename(pending, self.witness()).map_err(|e| e.to_string())?;
        self.written = Some(text);
        if changed {
            self.record_frame();
            if let Err(e) = self.record_replay() { eprintln!("warning: chunk committed, replay checkpoint/frames not saved: {e}"); }
        }
        Ok(String::new())
    }

    fn start_hand_frames(&mut self, done: Option<usize>) -> Result<(), String> {
        let resume = done.map(|n| self.replay.identity(&self.s.program(&self.name), self.s.st.borrow().width).map(|identity| (n, identity))).transpose()?;
        frames::start(60.0, session_dir(&self.name).join("hand-frames"), 1000, Some(GALLERY_LIGHT), resume)?;
        self.hand_frames = frames::stop();
        Ok(())
    }

    fn run_chunk(&mut self, src: &str) -> Result<session::Ran, String> {
        let result = frames::with(&mut self.hand_frames, || self.replay.with(|| self.s.run(src)));
        if result.is_err() {
            self.start_hand_frames((!self.s.log.is_empty()).then_some(self.s.log.len()))?;
        }
        result
    }

    fn record_replay(&mut self) -> Result<(), String> {
        let n = self.s.log.len();
        if n == 0 { return Ok(()); }
        let width = self.s.st.borrow().width;
        let source = self.s.program(&self.name);
        let identity = self.replay.identity(&source, width)?;
        frames::with(&mut self.hand_frames, || frames::chunk_end(self.s.canvas().as_deref(), n, &identity))?;
        self.replay.completed(&session_dir(&self.name).join("replay.ckpt"), &self.s, &source, width)
    }

    fn record_frame(&self) {
        if !self.frames { return; }
        let n = self.s.log.len();
        session::PROGRESS.keep(n);
        let st = self.s.st.borrow();
        let Some(c) = st.canvas.as_ref() else { return };
        if let Err(e) = live_frames::write(c, st.recording_ground, &session_dir(&self.name).join("frames"), n) {
            eprintln!("timelapse: chunk {n} committed, frame not saved: {e}");
        }
    }

    /// The live canvas as a PNG, `save`'s (`live.png`, next to the committed log), its save
    /// file (`live.ckpt`: the canvas and the studio fields the finishing verbs read, save.rs),
    /// and in `live.txt` how many chunks they hold and how many of them were replayed: the
    /// runner's check (scripts/check_painting) compares a replay of the log with them, and
    /// scripts/finish_painting finishes from the save instead of replaying the log.
    fn save_live(&self) -> Result<String, String> {
        let dir = session_dir(&self.name);
        let (png, txt, ckpt) = (dir.join("live.png"), dir.join("live.txt"), dir.join("live.ckpt"));
        let _ = std::fs::remove_file(&txt);
        let Some(c) = self.s.canvas() else {
            let _ = std::fs::remove_file(&png);
            let _ = std::fs::remove_file(&ckpt);
            return Ok(String::new());
        };
        deliver(&c, &png)?;
        drop(c);
        // a save that can't be written leaves none (finish_painting then replays the log)
        if let Err(e) = save::write(&self.s, &self.s.program(&self.name), &ckpt) {
            let _ = std::fs::remove_file(&ckpt);
            eprintln!("the save file couldn't be written: {e}");
        }
        std::fs::write(&txt, format!("chunks {}\nreplayed {}\nbuild {:016x}\nplatform {}-{}\n", self.s.log.len(), self.replayed, self.build, std::env::consts::OS, std::env::consts::ARCH)).map_err(|e| format!("{}: {e}", txt.display()))?;
        Ok(format!("the live canvas is in {}\n", png.display()))
    }

    fn view(&self, args: &[String]) -> Result<look::View, String> {
        let mut v = look::View::parse(args)?;
        v.overlays = v.mask.iter().map(|name| self.s.mask(name)).collect::<Result<_, _>>()?;
        Ok(v)
    }

    fn look(&mut self, args: &[String], path: Option<PathBuf>) -> Result<String, String> {
        // --survey: the whole canvas at full detail, in tiles; --compare <png>: an
        // earlier look beside this one (taken out before the view's own arguments)
        let (mut survey, mut compare, mut rest) = (false, None::<PathBuf>, Vec::new());
        if args.iter().any(|a| a == "--palette") {
            // (it takes no other option: the view's own check)
            look::View::parse(args)?;
            // the palette board beside the easel (palette_look.rs)
            let t0 = Instant::now();
            let png = {
                let st = self.s.st.borrow();
                palette_look::render(&st.board, &st.tubes)?
            };
            let p = new_look(&session_dir(&self.name), &png.2)?;
            return Ok(format!("{} ({}x{}, {:.2}s)\n", p.display(), png.0, png.1, t0.elapsed().as_secs_f64()));
        }
        let (mut hold, mut at, mut reference) = (None::<String>, None::<String>, None::<PathBuf>);
        let (mut pose, mut side) = (None::<String>, None::<String>);
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--hold" => {
                    hold = Some(args.get(i + 1).ok_or("--hold needs a knife or a pile: the name of a global that holds one")?.clone());
                    i += 1;
                }
                "--at" => {
                    at = Some(args.get(i + 1).ok_or("--at needs a point on the canvas: x,y in units")?.clone());
                    i += 1;
                }
                "--pose" => {
                    pose = Some(args.get(i + 1).ok_or("--pose needs force,alt,azimuth[,roll] (N, degrees)")?.clone());
                    i += 1;
                }
                "--side" => {
                    side = Some(args.get(i + 1).ok_or("--side needs the stroke's direction in degrees")?.clone());
                    i += 1;
                }
                "--survey" => survey = true,
                "--compare" => {
                    // (not a following option taken for the path)
                    compare = Some(PathBuf::from(args.get(i + 1).filter(|a| !a.starts_with("--")).ok_or("--compare needs the path of an image in this studio")?));
                    i += 1;
                }
                "--ref" => {
                    reference = Some(PathBuf::from(args.get(i + 1).ok_or("--ref needs a picture of the motif (a file in the studio)")?));
                    i += 1;
                }
                a => rest.push(a.to_string()),
            }
            i += 1;
        }
        if survey && compare.is_some() {
            return Err("look: --survey and --compare are two looks; ask for one".into());
        }
        // (a pastel stick comes with the paper-and-pastel switch: without it there is none to hold)
        let sticks = self.s.st.borrow().paper;
        if (pose.is_some() || side.is_some()) && !sticks {
            return Err(format!("look: --pose and --side hold a pastel stick, which needs the paper-and-pastel switch; this painting doesn't have it (its log's head has no {:?} line)", session::PAPER_MARK));
        }
        if (pose.is_some() || side.is_some()) && hold.is_none() {
            return Err("look: --pose and --side hold a pastel stick: they go with --hold <stick> --at x,y".into());
        }
        if let Some(r) = reference {
            if survey || compare.is_some() || hold.is_some() || at.is_some() {
                return Err("look: --ref is a look of its own: no --survey, --compare or --hold".into());
            }
            return self.reference(&rest, &r);
        }
        // --hold <pile> --at x,y: the loaded knife held up to the canvas (speculative: `hold_look`)
        if hold.is_some() || at.is_some() {
            let (Some(pile), Some(at)) = (hold, at) else { return Err("look: --hold <knife or pile> and --at x,y go together".into()) };
            if survey || compare.is_some() {
                return Err("look: --hold is a look of its own: no --survey or --compare".into());
            }
            let p: Vec<f32> = at.split(',').map(|t| t.trim().parse::<f32>()).collect::<Result<_, _>>().map_err(|_| format!("--at {at}: want x,y in units"))?;
            if p.len() != 2 {
                return Err(format!("--at {at}: want x,y in units"));
            }
            let v = self.view(&rest)?;
            let t0 = Instant::now();
            if sticks && self.s.held_stick(&pile).is_ok() {
                let nums = |t: &str| -> Result<Vec<f32>, String> { t.split(',').map(|x| x.trim().parse::<f32>()).collect::<Result<_, _>>().map_err(|_| format!("{t}: want numbers separated by commas")) };
                let ps = match (&pose, &side) {
                    (Some(_), Some(_)) => return Err("look --hold: --pose or --side, not both".into()),
                    (Some(t), None) => {
                        let n = nums(t)?;
                        if !(3..=4).contains(&n.len()) {
                            return Err("--pose force,alt,azimuth[,roll]: newtons and degrees".into());
                        }
                        [n[0], n[1], n[2], n.get(3).copied().unwrap_or(0.0)]
                    }
                    // laid flat across a stroke going that way (as p:side holds it)
                    (None, Some(t)) => {
                        let n = nums(t)?;
                        if n.len() != 1 {
                            return Err("--side: the stroke's direction, degrees".into());
                        }
                        [2.0, 0.0, n[0] + 90.0, 0.0]
                    }
                    (None, None) => [1.5, 60.0, 45.0, 0.0],
                };
                if !(0.0..=20.0).contains(&ps[0]) || !(0.0..=90.0).contains(&ps[1]) {
                    return Err("--pose: force 0 to 20 N, alt 0 to 90 degrees".into());
                }
                if !(ps[2].is_finite() && ps[3].is_finite()) {
                    return Err("--pose, --side: the azimuth, direction and roll are numbers (degrees)".into());
                }
                let (w, h, png, note) = stick_hold_look(&self.s, &pile, (p[0], p[1]), ps, &v)?;
                let path = new_look(&session_dir(&self.name), &png)?;
                return Ok(format!("{} ({w}x{h}, {:.2}s): {pile} held at {},{}: {note}\n", path.display(), t0.elapsed().as_secs_f64(), p[0], p[1]));
            }
            if pose.is_some() || side.is_some() {
                return Err(format!("look --hold {pile}: --pose and --side hold a pastel stick, not a knife or a pile"));
            }
            let (w, h, png, crop) = hold_look(&self.s, &pile, (p[0], p[1]), &v)?;
            let path = new_look(&session_dir(&self.name), &png)?;
            let c = self.s.canvas().unwrap();
            let scale = look::crop_scale(c.window(), &look::View { crop: Some(crop), ..v }, w, h)?;
            return Ok(format!("{} ({w}x{h}, {:.2}s): {pile} held up to the canvas at {},{}{scale}\n", path.display(), t0.elapsed().as_secs_f64(), p[0], p[1]));
        }
        if survey {
            return self.survey(&rest);
        }
        if let Some(prev) = compare {
            return self.compare(&rest, &prev);
        }
        let args = &rest[..];
        let v = self.view(args)?;
        let t0 = Instant::now();
        if v.palette {
            let (w, h, png) = palette_look(&self.s)?;
            let path = new_look(&session_dir(&self.name), &png)?;
            return Ok(format!("{} ({w}x{h}, {:.2}s)\n", path.display(), t0.elapsed().as_secs_f64()));
        }
        let c = self.s.canvas().ok_or("no canvas yet: the first chunk is canvas{...}")?;
        if let Some(pool) = &self.display && v.display_eligible() {
            let mut reply = None;
            pool.submit(|| {
                let optics = c.prepare_optics();
                let (w, h, png) = look::render_pixels(optics.frame(), &v, optics.color())?;
                let path = match path {
                    Some(p) => {
                        if let Some(dir) = p.parent() {
                            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
                        }
                        std::fs::write(&p, &png).map_err(|e| e.to_string())?;
                        p
                    }
                    None => display::new_look(&session_dir(&self.name), &png)?,
                };
                let scale = look::crop_scale(optics.frame(), &v, w, h)?;
                let publication = display::Publication::new(vec![(v, path.clone())])?;
                reply = Some(format!("{} ({w}x{h}, {:.2}s){scale}\n", path.display(), t0.elapsed().as_secs_f64()));
                Ok(Box::new(move |cancel| publication.render(optics, cancel)))
            })?;
            return Ok(reply.unwrap());
        }
        let (w, h, path) = match path {
            Some(p) => {
                let (w, h) = look::look(&c, &v, &p)?;
                (w, h, p)
            }
            None => {
                let (w, h, png) = look::render(&c, &v)?;
                (w, h, new_look(&session_dir(&self.name), &png)?)
            }
        };
        let scale = look::crop_scale(c.window(), &v, w, h)?;
        let reply = format!("{} ({w}x{h}, {:.2}s){scale}\n", path.display(), t0.elapsed().as_secs_f64());
        look::companions(&c, &[(v, path)]);
        Ok(reply)
    }

    /// The whole canvas at full detail (2.4 px a unit, as a crop), in tiles of at most 500 units a side,
    /// in reading order; the view's modes (gallery, value...) apply to each.
    fn survey(&mut self, args: &[String]) -> Result<String, String> {
        if args.iter().any(|a| a == "--crop" || a == "--size") {
            return Err("look --survey covers the whole canvas at full detail: no --crop or --size".into());
        }
        let c = self.s.canvas().ok_or("no canvas yet: the first chunk is canvas{...}")?;
        // (the canvas as it is seen, lit or not, once for all the tiles)
        let view = self.view(args)?;
        let (wu, hu) = (c.width(), c.height());
        // (a tile is at most 500 units: 1200 px as a crop is shown, the most a crop takes)
        let cols = (wu / 500.0).ceil().max(1.0) as usize;
        let rows = (hu / 500.0).ceil().max(1.0) as usize;
        let (tw, th) = (wu / cols as f32, hu / rows as f32);
        let render = |seen: &[paint::Rgb], stable: bool| -> Result<_, String> {
            let mut out = format!("survey: {rows} rows x {cols} columns of {tw:.0} x {th:.0} units, at full detail\n");
            let mut views = Vec::new();
            for r in 0..rows {
                for k in 0..cols {
                    let crop = format!("{},{},{},{}", k as f32 * tw, r as f32 * th, (k + 1) as f32 * tw, (r + 1) as f32 * th);
                    let mut v = view.clone();
                    v.crop = Some([k as f32 * tw, r as f32 * th, (k + 1) as f32 * tw, (r + 1) as f32 * th]);
                    let (w, h, png) = look::render_pixels(c.window(), &v, seen)?;
                    let p = if stable { display::new_look(&session_dir(&self.name), &png)? } else { new_look(&session_dir(&self.name), &png)? };
                    out += &format!("{} ({w}x{h}): row {} column {} ({crop})\n", p.display(), r + 1, k + 1);
                    views.push((v, p));
                }
            }
            Ok((out, views))
        };
        if let Some(pool) = &self.display && view.display_eligible() {
            let mut reply = None;
            pool.submit(|| {
                let optics = c.prepare_optics();
                let (out, views) = render(optics.color(), true)?;
                let publication = display::Publication::new(views)?;
                reply = Some(out);
                Ok(Box::new(move |cancel| publication.render(optics, cancel)))
            })?;
            return Ok(reply.unwrap());
        }
        let seen = match view.light {
            Some((az, el)) => c.seen_lit(az, el, 1.0),
            None => c.seen(),
        };
        let (out, views) = render(&seen, false)?;
        drop(seen);
        look::companions(&c, &views);
        Ok(out)
    }

    /// The paint at a point (`--at x,y`) or averaged over a window (`--crop x0,y0,x1,y1`), in
    /// units: its color and value in 8-bit sRGB as a plain look and a value look show them,
    /// the same in the gallery's light, and its drying stage (over a window, each stage's share).
    fn probe(&self, args: &[String]) -> Result<String, String> {
        let numbers = |s: &str, n: usize| {
            let p: Vec<f32> = s.split(',').map(|t| t.trim().parse::<f32>()).collect::<Result<_, _>>().ok()?;
            (p.len() == n && p.iter().all(|v| v.is_finite())).then_some(p)
        };
        let (place, [x0, y0, x1, y1]) = match args {
            [a, s] if a == "--at" => {
                let p = numbers(s, 2).ok_or(format!("probe: --at {s}: want x,y in canvas units"))?;
                (format!("at {},{}", p[0], p[1]), [p[0], p[1], p[0], p[1]])
            }
            [a, s] if a == "--crop" => {
                let p = numbers(s, 4).ok_or(format!("probe: --crop {s}: want x0,y0,x1,y1 in canvas units (opposite corners)"))?;
                (format!("over {},{},{},{}", p[0], p[1], p[2], p[3]), [p[0].min(p[2]), p[1].min(p[3]), p[0].max(p[2]), p[1].max(p[3])])
            }
            _ => return Err("probe: want --at x,y or --crop x0,y0,x1,y1 (canvas units)".into()),
        };
        let c = self.s.canvas().ok_or("no canvas yet: the first chunk is canvas{...}")?;
        let f = c.window();
        // (to half a pixel: a canvas's height in units is its pixels', which needn't be round)
        let (w, h) = (c.width(), c.height());
        if x0 < 0.0 || y0 < 0.0 || x1 > w + 0.5 / f.scale || y1 > h + 0.5 / f.scale {
            let round = |v: f32| (v * 100.0).round() / 100.0;
            return Err(format!("probe: {place} is outside the canvas (0,0 to {},{})", round(w), round(h)));
        }
        // the pixels under the point, or every pixel the window touches
        let span = |a: f32, b: f32, lo: usize, n: usize| {
            let i = ((a * f.scale).floor() as usize).saturating_sub(lo).min(n - 1);
            (i, ((b * f.scale).ceil() as usize).saturating_sub(lo).clamp(i + 1, n))
        };
        let ((px0, px1), (py0, py1)) = (span(x0, x1, f.x0, f.w), span(y0, y1, f.y0, f.h));
        let (plain, lit, stages) = (c.seen(), c.seen_lit(GALLERY_LIGHT.0, GALLERY_LIGHT.1, 1.0), c.stages());
        let (mut sums, mut counts) = ([[0.0f64; 3]; 2], [0usize; 4]);
        for y in py0..py1 {
            for x in px0..px1 {
                let i = y * f.w + x;
                for (sum, seen) in sums.iter_mut().zip([&plain, &lit]) {
                    for q in 0..3 {
                        sum[q] += seen[i][q] as f64;
                    }
                }
                counts[stages[i] as usize] += 1;
            }
        }
        let n = (px1 - px0) * (py1 - py0);
        let level = |v: f64| (paint::color::linear_to_srgb(v as f32) * 255.0).round().clamp(0.0, 255.0) as u8;
        let color = |sum: [f64; 3]| {
            let rgb = sum.map(|v| v / n as f64);
            let value = 0.2126 * rgb[0] + 0.7152 * rgb[1] + 0.0722 * rgb[2];
            format!("{},{},{} · value {}", level(rgb[0]), level(rgb[1]), level(rgb[2]), level(value))
        };
        let names = ["open", "setting", "tacky", "dry"];
        let drying = if n == 1 {
            names[counts.iter().position(|&k| k == 1).unwrap()].to_string()
        } else {
            names.iter().zip(counts).filter(|(_, k)| *k > 0).map(|(s, k)| format!("{s} {}%", (100.0 * k as f64 / n as f64).round())).collect::<Vec<_>>().join(", ")
        };
        let what = if n == 1 { place } else { format!("{place}, averaged over {} x {} pixels", px1 - px0, py1 - py0) };
        Ok(format!("{what}\ncolor {}\nin gallery light {}\ndrying: {drying}\n", color(sums[0]), color(sums[1])))
    }

    /// An earlier look (left) beside the same view of the canvas now (right), at
    /// the same height, for judging what a change did.
    fn compare(&mut self, args: &[String], prev: &Path) -> Result<String, String> {
        let mut a = args.to_vec();
        if !a.iter().any(|x| x == "--size" || x == "--crop") {
            a.extend(["--size".to_string(), "800".to_string()]);
        }
        let v = self.view(&a)?;
        let c = self.s.canvas().ok_or("no canvas yet: the first chunk is canvas{...}")?;
        let (_, _, png) = look::render(&c, &v)?;
        let now = image::load_from_memory(&png).map_err(|e| e.to_string())?.to_rgb8();
        // an image of this studio (usually an earlier look): a picture under its folder, nothing outside it
        let studio = root().canonicalize().map_err(|e| e.to_string())?;
        let prev = if prev.is_absolute() { prev.to_path_buf() } else { studio.join(prev) };
        let real = prev.canonicalize().map_err(|e| format!("--compare {}: {e}", prev.display()))?;
        if !real.starts_with(&studio) {
            return Err(format!("--compare {}: compare takes an image in this studio ({})", prev.display(), studio.display()));
        }
        let before = image::open(&real).map_err(|e| format!("--compare {}: {e}", prev.display()))?.to_rgb8();
        let h = now.height();
        let bw = ((before.width() as f64 * h as f64 / before.height() as f64).round() as u32).max(1);
        let total_width = bw.checked_add(12).and_then(|w| w.checked_add(now.width()));
        if total_width.is_none_or(|w| w > 16384 || u64::from(w) * u64::from(h) > 32_000_000) {
            return Err("--compare: the combined picture is too large".into());
        }
        let before = image::imageops::resize(&before, bw, h, image::imageops::FilterType::Lanczos3);
        let gap = 12;
        let mut both = image::RgbImage::from_pixel(bw + gap + now.width(), h, image::Rgb([24, 24, 28]));
        image::imageops::replace(&mut both, &before, 0, 0);
        image::imageops::replace(&mut both, &now, (bw + gap) as i64, 0);
        let mut bytes = Vec::new();
        both.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).map_err(|e| e.to_string())?;
        let p = new_look(&session_dir(&self.name), &bytes)?;
        let scale = look::crop_scale(c.window(), &v, now.width() as usize, h as usize)?;
        Ok(format!("{} ({}x{}): left {}, right now{scale}\n", p.display(), both.width(), h, prev.display()))
    }

    /// The motif beside the canvas, as a painter pins a photograph or a study
    /// beside the easel: the picture (a file in the studio) fitted to the
    /// canvas's shape (centered, cropped to its aspect), on the left; the
    /// same view of the canvas on the right. Both go through the view's
    /// crop, size, modes and grid alike, so a passage is set beside the same
    /// part of the motif. It is only looked at: nothing of it reaches the
    /// painting, its log or its randomness.
    fn reference(&mut self, args: &[String], pic: &Path) -> Result<String, String> {
        let mut a = args.to_vec();
        if !a.iter().any(|x| x == "--size" || x == "--crop") {
            a.extend(["--size".to_string(), "800".to_string()]);
        }
        let v = self.view(&a)?;
        if v.palette {
            return Err("look: --ref sets the motif beside the canvas, not the palette".into());
        }
        let studio = root().canonicalize().map_err(|e| e.to_string())?;
        let pic = if pic.is_absolute() { pic.to_path_buf() } else { studio.join(pic) };
        let real = pic.canonicalize().map_err(|e| format!("--ref {}: {e}", pic.display()))?;
        if !real.starts_with(&studio) {
            return Err(format!("--ref {}: the motif is a picture in this studio ({})", pic.display(), studio.display()));
        }
        use image::ImageDecoder;
        let im = (|| -> image::ImageResult<image::RgbImage> {
            let mut decoder = image::ImageReader::open(&real)?.into_decoder()?;
            let orientation = decoder.orientation()?;
            let mut im = image::DynamicImage::from_decoder(decoder)?;
            im.apply_orientation(orientation);
            Ok(im.into_rgb8())
        })().map_err(|e| format!("--ref {}: {e}", pic.display()))?;
        let c = self.s.canvas().ok_or("no canvas yet: the first chunk is canvas{...}")?;
        let (f, whole) = (c.window(), c.frame());
        // the picture fitted to the canvas's shape: scaled to cover it, centered
        let (iw, ih) = (im.width() as f32, im.height() as f32);
        let k = (iw / whole.w as f32).min(ih / whole.h as f32);
        let (ox, oy) = (0.5 * (iw - k * whole.w as f32), 0.5 * (ih - k * whole.h as f32));
        let lin: Vec<f32> = (0..=255).map(|b| paint::color::srgb_to_linear(b as f32 / 255.0)).collect();
        let at = |x: f32, y: f32| -> [f32; 3] {
            // (bilinear between the picture's pixels)
            let (x, y) = ((x - 0.5).clamp(0.0, iw - 1.0), (y - 0.5).clamp(0.0, ih - 1.0));
            let (x0, y0) = (x.floor() as u32, y.floor() as u32);
            let (x1, y1) = ((x0 + 1).min(im.width() - 1), (y0 + 1).min(im.height() - 1));
            let (fx, fy) = (x - x0 as f32, y - y0 as f32);
            let p = |xx: u32, yy: u32| im.get_pixel(xx, yy).0.map(|b| lin[b as usize]);
            let (a, b, cc, d) = (p(x0, y0), p(x1, y0), p(x0, y1), p(x1, y1));
            [0, 1, 2].map(|q| (a[q] * (1.0 - fx) + b[q] * fx) * (1.0 - fy) + (cc[q] * (1.0 - fx) + d[q] * fx) * fy)
        };
        let motif: Vec<[f32; 3]> = (0..f.w * f.h)
            .map(|i| {
                let (x, y) = ((i % f.w + f.x0) as f32 + 0.5, (i / f.w + f.y0) as f32 + 0.5);
                at(ox + x * k, oy + y * k)
            })
            .collect();
        // (one scale for both: the size budget may shrink a busy photograph's panel)
        let ((_, _, left), (w, h, right)) = look::render_pair(&c, &v, &motif)?;
        let scale = look::crop_scale(f, &v, w, h)?;
        drop(c);
        let (l, r) = (image::load_from_memory(&left).map_err(|e| e.to_string())?.to_rgb8(), image::load_from_memory(&right).map_err(|e| e.to_string())?.to_rgb8());
        let gap = 12;
        let h = l.height().max(r.height());
        let mut both = image::RgbImage::from_pixel(l.width() + gap + r.width(), h, image::Rgb([24, 24, 28]));
        image::imageops::replace(&mut both, &l, 0, 0);
        image::imageops::replace(&mut both, &r, (l.width() + gap) as i64, 0);
        let mut bytes = Vec::new();
        both.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).map_err(|e| e.to_string())?;
        let p = new_look(&session_dir(&self.name), &bytes)?;
        Ok(format!("{} ({}x{}): left the motif ({}), right the canvas{scale}\n", p.display(), both.width(), h, pic.display()))
    }

    /// The log on disk is the one the session wrote, and holds everything it ran.
    fn ready(&self) -> Result<(), String> {
        self.validate()?;
        if self.written.as_deref() != Some(self.s.program(&self.name).as_str()) {
            return Err("session integrity: uncommitted state; restart required".into());
        }
        Ok(())
    }

    /// What `check` replays and compares with (check.rs runs it on its own thread).
    fn check_input(&self) -> Result<check::Input, String> {
        self.ready()?;
        Ok(check::Input {
            width: self.s.st.borrow().width,
            paper: self.s.st.borrow().paper,
            tubes: self.s.tube_box(),
            chunks: self.s.log.iter().map(|c| c.src.clone()).collect(),
            live: self.s.canvas().map(|a| (bits(&a.seen()), bits_f(a.surface_um()))),
        })
    }

    /// A command for the painting, or with `--scratch` for the scratch canvas beside it.
    /// `do --scratch --new` puts the scratch canvas aside and starts a fresh one.
    fn handle(&mut self, cmd: &str, args: &[String], payload: &str) -> Result<String, String> {
        let scratch = args.iter().any(|a| a == "--scratch");
        let new = args.iter().any(|a| a == "--new");
        let args: Vec<String> = args.iter().filter(|a| *a != "--scratch" && *a != "--new").cloned().collect();
        if new && !(scratch && cmd == "do") {
            return Err("--new starts a fresh scratch canvas: do --scratch --new".into());
        }
        if !scratch {
            return self.serve_cmd(cmd, &args, payload);
        }
        if !matches!(cmd, "do" | "look" | "probe" | "status" | "log" | "globals" | "save") {
            return Err(format!("{cmd} is the painting's: it takes no --scratch"));
        }
        self.ready()?;
        let mut out = String::new();
        // reopened when first asked for, so it never holds up the painting's reopen
        if self.scratch.is_none() && !new {
            out += &self.resume_scratch();
        }
        if new || (self.scratch.is_none() && cmd == "do") {
            let fresh = self.new_scratch().map_err(|e| format!("{out}{e}"))?;
            out += &fresh;
        }
        let sc = self.scratch.as_mut().ok_or("there is no scratch canvas yet: the first chunk painted on it sets it up beside the painting")?;
        if let Err(e) = sc.ready() {
            self.scratch = None;
            return Err(format!("{out}{e}\n(the scratch canvas is put away; the next chunk painted on it starts a fresh one)"));
        }
        let reply = sc.serve_cmd(cmd, &args, payload).map_err(|e| format!("{out}{e}"))?;
        out += &reply;
        Ok(out)
    }

    /// Run a chunk of the easel's own (not the painter's) and log it.
    fn run_own(&mut self, src: &str) -> Result<(), String> {
        self.ready()?;
        self.run_chunk(src)?;
        self.save_log()?;
        Ok(())
    }

    /// Reopen the scratch canvas the studio has (its log), if any, at its first use after the
    /// painting's reopen. A scratch canvas that can't be reopened is put aside, as `--new`
    /// does, and doesn't stop the painting.
    /// What happened, if the painter should know (a scratch canvas put aside).
    fn resume_scratch(&mut self) -> String {
        let name = scratch_name(&self.name);
        if !(log_path(&name).exists() || witness_path(&name).exists()) {
            return String::new();
        }
        match Server::resume_as(name.clone(), Some(&self.s), self.display.clone(), &|_, _| {}) {
            Ok(sc) => {
                self.scratch = Some(Box::new(sc));
                String::new()
            }
            Err(e) => {
                eprintln!("warning: the scratch canvas couldn't be reopened ({e}); it is put aside");
                match put_aside(&name) {
                    Ok(_) => "(the scratch canvas couldn't be reopened, so it is put aside)\n".into(),
                    Err(e) => format!("(the scratch canvas couldn't be reopened, nor put aside: {e})\n"),
                }
            }
        }
    }

    /// A fresh scratch canvas: the old one's log is put aside, and the new one is set up as
    /// the painting's canvas was (its `canvas{...}`, the easel's first chunk there).
    fn new_scratch(&mut self) -> Result<String, String> {
        let src = self.s.st.borrow().canvas_src.clone().ok_or(
            "the scratch canvas is set up as a copy of the painting's canvas, and the painting has none yet: its first chunk is canvas{...}",
        )?;
        let name = scratch_name(&self.name);
        self.scratch = None;
        let aside = put_aside(&name)?;
        let mut sc = Server::resume_as(name, Some(&self.s), self.display.clone(), &|_, _| {})?;
        sc.run_own(&src)?;
        self.scratch = Some(Box::new(sc));
        Ok(match aside {
            Some(p) => format!("a fresh scratch canvas (the last one's log is {})\n", p.strip_prefix(root()).unwrap_or(&p).display()),
            None => "a scratch canvas beside the painting, set up as the painting's canvas was\n".into(),
        })
    }

    fn serve_cmd(&mut self, cmd: &str, args: &[String], payload: &str) -> Result<String, String> {
        self.ready()?;
        match cmd {
            "note" => append_note(self.s.st.borrow().clock, payload),
            "status" => Ok(format!("{}\n", self.s.status())),
            "globals" => Ok(self.s.globals()),
            "do" => match self.run_chunk(payload) {
                Ok(ran) => {
                    let note = self.save_log()?;
                    let n = self.s.log.len();
                    session::PROGRESS.keep(n);
                    let mut out = note;
                    out.push_str(&ran.out);
                    out.push_str(&format!("{}\n", time::time_of_day(self.s.st.borrow().clock)));
                    out.push_str(&format!("ok · chunk {n} ({:.2} s to compute)\n", ran.secs));
                    if args.iter().any(|a| a == "--look") {
                        match self.look(&[], None) {
                            Ok(l) => out.push_str(&l),
                            Err(e) => out.push_str(&format!("(no look: {e})\n")),
                        }
                    }
                    Ok(out)
                }
                Err(e) if self.s.unrestored => Err(format!(
                    "{e}\n(the chunk failed and is not in the log; status reports the rebuild's progress and other commands must retry after it)"
                )),
                Err(e) if self.s.stale => Err(format!(
                    "{e}\n(the chunk failed and changed nothing. It had changed tables from earlier chunks, and though what they hold is back, how they are laid out (which decides the order `pairs` walks them in, from engine 3 only the length `#` finds in a table with holes) can't be put back, so the easel now rebuilds the painting from its log, as a reopen does; status reports progress and other commands must retry after that)"
                )),
                Err(e) => Err(format!("{e}\n(the chunk failed and changed nothing)")),
            },
            "look" => self.look(args, None),
            "probe" => self.probe(args),
            "log" => Ok(self.s.program(&self.name)),
            "save" => {
                // save [path] [--light az,el | --gallery]: lit on the paint's relief, or color only
                let (path, light) = save_args(args)?;
                let p = match path {
                    Some(p) if !p.is_absolute() => root().join(p),
                    Some(p) => p,
                    None => session_dir(&self.name).join(format!("{}.png", self.name)),
                };
                #[cfg(feature = "state-store")]
                if let Some(store) = &self.committed {
                    store.protect_output(&p).map_err(|e| e.to_string())?;
                    if same_file(&log_path(&self.name), &p)? || same_file(&self.witness(), &p)? {
                        return Err("save output must differ from the painting's journal exports".into());
                    }
                }
                let p = in_studio_png(&p)?;
                // A path inside the studio can still share an inode with a file outside it.
                // Keep the store's more specific alias checks above this general refusal.
                use std::os::unix::fs::MetadataExt;
                if std::fs::metadata(&p).is_ok_and(|m| m.is_file() && m.nlink() > 1) {
                    return Err("save: that picture has hard links: save to a new file in this studio".into());
                }
                let c = self.s.canvas().ok_or("no canvas yet")?;
                #[cfg(feature = "state-store")]
                if self.committed.is_some() {
                    painting_store::atomic_output(&p, |part| deliver_lit(&c, part, light))?;
                } else {
                    deliver_lit(&c, &p, light)?;
                }
                #[cfg(not(feature = "state-store"))]
                deliver_lit(&c, &p, light)?;
                Ok(format!("{}\n", p.display()))
            }
            "frames" => {
                self.frames = match args.first().map(|s| s.as_str()) {
                    Some("on") | None => true,
                    Some("off") => false,
                    _ => return Err("frames on|off".into()),
                };
                Ok(format!("frames {} ({})\n", if self.frames { "on" } else { "off" }, session_dir(&self.name).join("frames").display()))
            }
            "close" => {
                let note = self.save_log()?;
                frames::with(&mut self.hand_frames, || {
                    if let Some(c) = self.s.canvas() { frames::finish(&c); }
                });
                if let Some(e) = self.hand_frames.as_ref().and_then(|r| r.err.as_ref()) {
                    eprintln!("warning: final hand-time frame not saved: {e}");
                }
                let live = self.save_live().unwrap_or_else(|e| format!("the live canvas couldn't be saved: {e}\n"));
                if let Some(pool) = &self.display { pool.flush()?; }
                Ok(format!("{note}{live}closed; the session is in {}\n", log_path(&self.name).display()))
            }
            o => Err(format!("unknown command {o:?}")),
        }
    }
}

fn bits(v: &[paint::Rgb]) -> Vec<u32> {
    v.iter().flat_map(|p| p.map(f32::to_bits)).collect()
}
fn bits_f(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}

// ---------------------------------------------------------------- replay

/// A gallery's light: from above and in front, high (55°) and a little from
/// the left, as a picture hangs on a wall: impasto models softly.
pub const GALLERY_LIGHT: (f32, f32) = (115.0, 55.0);

/// `light az,el` (degrees) as a pair.
pub(crate) fn light_of(s: &str) -> Result<(f32, f32), String> {
    let p: Vec<f32> = s.split(',').map(|t| t.trim().parse::<f32>()).collect::<Result<_, _>>().map_err(|_| format!("light {s}: want azimuth,elevation in degrees"))?;
    if p.len() != 2 || !p[0].is_finite() || !(0.0..=90.0).contains(&p[1]) {
        return Err(format!("light {s}: want azimuth,elevation in degrees (elevation 0 to 90)"));
    }
    Ok((p[0], p[1]))
}

/// `save`'s arguments: an optional path, then `--light az,el` or `--gallery`.
fn save_args(args: &[String]) -> Result<(Option<PathBuf>, Option<(f32, f32)>), String> {
    if args.iter().any(|a| a == "--gallery") && args.iter().any(|a| a == "--light") {
        return Err("save: --light and --gallery are two lights; give one".into());
    }
    let (mut path, mut light) = (None, None);
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--light" => {
                light = Some(light_of(args.get(i + 1).ok_or("--light needs azimuth,elevation")?)?);
                i += 1;
            }
            "--gallery" => light = Some(GALLERY_LIGHT),
            a if !a.starts_with('-') && path.is_none() => path = Some(PathBuf::from(a)),
            o => return Err(format!("save [path] [--light az,el | --gallery]: unknown {o:?}")),
        }
        i += 1;
    }
    Ok((path, light))
}

/// Where `save <path>` may write: a `.png` inside this studio (EASEL_ROOT,
/// or the painter's studio), as `look --compare` may only read one. A
/// relative path is the studio's. The deepest folder of it that exists must
/// be in the studio once its links are followed, the folders below it are
/// plain names (no `..`), and the file itself, if it is there, is no link:
/// so stable paths cannot lead outside the studio.
fn in_studio_png(path: &Path) -> Result<PathBuf, String> {
    use std::path::Component;
    let refuse = |why: &str| Err(format!("save {}: {why}", path.display()));
    if path.extension().and_then(|e| e.to_str()).is_none_or(|e| !e.eq_ignore_ascii_case("png")) {
        return refuse("a picture is saved as a .png");
    }
    let studio = root().canonicalize().map_err(|e| format!("save: the studio {}: {e}", root().display()))?;
    let p = if path.is_absolute() { path.to_path_buf() } else { studio.join(path) };
    // the deepest folder that exists, and the plain names below it
    let mut base = p.parent().ok_or("save: no folder")?.to_path_buf();
    let mut tail = Vec::new();
    while !base.exists() {
        let name = base.file_name().map(|n| n.to_os_string());
        match name {
            Some(n) => tail.push(n),
            None => return refuse("no folder of it exists"),
        }
        base = base.parent().ok_or("save: no folder")?.to_path_buf();
    }
    if p.components().any(|c| matches!(c, Component::ParentDir)) && !tail.is_empty() {
        return refuse("`..` in a folder that doesn't exist yet");
    }
    let real = base.canonicalize().map_err(|e| format!("save {}: {e}", path.display()))?;
    if !real.starts_with(&studio) {
        return Err(format!("save {}: a picture is saved in this studio ({})", path.display(), studio.display()));
    }
    let mut out = real;
    for n in tail.into_iter().rev() {
        out.push(n);
    }
    let out = out.join(p.file_name().ok_or("save: no file name")?);
    // Check the path we will actually write: a trailing slash or `/.` may
    // have kept symlink_metadata on the original spelling from seeing a link.
    if std::fs::symlink_metadata(&out).is_ok_and(|m| m.file_type().is_symlink()) {
        return refuse("that is a link: save to a picture in this studio");
    }
    Ok(out)
}

/// The delivered PNG: the canvas as it is seen now (wet paint as laid, no
/// drying), 8-bit sRGB. `save` and `run` both write it.
fn deliver(c: &Canvas, out: &Path) -> Result<(), String> {
    deliver_lit(c, out, None)
}

/// `deliver`, lit from (azimuth, elevation) in degrees on the paint's relief
/// (`Canvas::seen_lit`), or not lit (None: color only).
fn deliver_lit(c: &Canvas, out: &Path, light: Option<(f32, f32)>) -> Result<(), String> {
    let f = c.window();
    let px = match light {
        Some((az, el)) => c.seen_lit(az, el, 1.0),
        None => c.seen(),
    };
    let buf = output::rgb8_exact(&px, f.w);
    if let Some(d) = out.parent() {
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    image::save_buffer(out, &buf, f.w as u32, f.h as u32, image::ColorType::Rgb8).map_err(|e| format!("{}: {e}", out.display()))
}

const RUN_USAGE: &str = "run <file.lua> [--out path.png] [--light az,el | --gallery] [--look] [--checkpoint-out path.ckpt] [--state-digest digests.txt] [--replay-checkpoint replay.ckpt [--until-chunk N] [--for-minutes M]] [--frames-every <s> --frames-dir <dir> [--frame-width 1000]] [--width <px>] (replays at the width it was painted at, 2400px or the width its head names, a sketch 600px, unless --width: a smaller preview for development, not the painting)";

fn normalized_path(path: &Path) -> std::io::Result<PathBuf> {
    let absolute = std::path::absolute(path)?;
    let mut pending: std::collections::VecDeque<_> = absolute.components().map(|c| c.as_os_str().to_os_string()).collect();
    let mut resolved = PathBuf::new();
    let mut links = 0;
    while let Some(part) = pending.pop_front() {
        if part == "/" {
            resolved = PathBuf::from("/");
        } else if part == ".." {
            resolved.pop();
        } else if part != "." {
            resolved.push(part);
            match std::fs::read_link(&resolved) {
                Ok(target) => {
                    links += 1;
                    if links > 40 {
                        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "too many symbolic links in output path"));
                    }
                    resolved.pop();
                    for component in target.components().rev() {
                        pending.push_front(component.as_os_str().to_os_string());
                    }
                }
                Err(e) if matches!(e.kind(), std::io::ErrorKind::NotFound | std::io::ErrorKind::InvalidInput) => {}
                Err(e) => return Err(e),
            }
        }
    }
    Ok(resolved)
}

fn same_file(a: &Path, b: &Path) -> Result<bool, String> {
    use std::os::unix::fs::MetadataExt;
    let resolve = |p: &Path| normalized_path(p).map_err(|e| format!("{}: {e}", p.display()));
    Ok(resolve(a)? == resolve(b)? || match (std::fs::metadata(a), std::fs::metadata(b)) {
        (Ok(a), Ok(b)) => (a.dev(), a.ino()) == (b.dev(), b.ino()),
        _ => false,
    })
}

fn preflight_run(args: &[String], file: &Path, out: &Path, chunks: usize) -> Result<(), String> {
    use std::os::unix::fs::MetadataExt;
    let checkpoint = flag(args, "--replay-checkpoint").map(PathBuf::from);
    let progress = checkpoint.as_deref().unwrap_or(out).with_extension("replay-progress.json");
    let mut outputs = vec![("source", file.to_path_buf()), ("--out", out.to_path_buf()), ("replay progress", progress)];
    if args.iter().any(|a| a == "--look") { outputs.push(("--look", out.with_extension("look.png"))); }
    for option in ["--checkpoint-out", "--replay-checkpoint", "--state-digest", "--dump-surface"] {
        if let Some(p) = flag(args, option) { outputs.push((option, PathBuf::from(p))); }
    }
    let resolve = |p: &Path| normalized_path(p).map_err(|e| format!("{}: {e}", p.display()));
    let mut dirs: Vec<(&str, PathBuf)> = Vec::new();
    for option in ["--frames-dir", "--dump-state"] {
        let Some(p) = flag(args, option) else { continue };
        let dir = PathBuf::from(p);
        let resolved = resolve(&dir)?;
        for (name, path) in &outputs {
            let path = resolve(path)?;
            if path.starts_with(&resolved) || resolved.starts_with(&path) {
                return Err(format!("{name} must be outside {option}"));
            }
        }
        for (other, path) in &dirs {
            if resolved.starts_with(path) || path.starts_with(&resolved) {
                return Err(format!("{option} must be separate from {other}"));
            }
        }
        dirs.push((option, resolved.clone()));
        let mut generated = if option == "--frames-dir" {
            vec![dir.join("frames.tsv"), dir.join("frames.states")]
        } else {
            (1..=chunks).map(|n| dir.join(format!("chunk-{n:03}.state"))).chain([dir.join("final.ckpt")]).collect()
        };
        match std::fs::read_dir(&dir) {
            Ok(entries) => {
                for entry in entries {
                    let path = entry.map_err(|e| format!("{}: {e}", dir.display()))?.path();
                    for (name, output) in &outputs {
                        if same_file(output, &path)? {
                            return Err(format!("{name} must not alias an output in {option}"));
                        }
                    }
                    if option == "--frames-dir" && path.extension().is_some_and(|e| e == "png") && path.file_stem().and_then(|n| n.to_str()).is_some_and(|n| n.parse::<usize>().is_ok()) {
                        generated.push(path);
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(format!("{}: {e}", dir.display())),
        }
        if option == "--frames-dir" {
            for path in &generated {
                let target = resolve(path)?;
                if target.parent() == Some(resolved.as_path()) && target.extension().is_some_and(|e| e == "png") && target.file_stem().and_then(|n| n.to_str()).is_some_and(|n| n.parse::<usize>().is_ok()) && target != resolved.join(path.file_name().unwrap()) {
                    return Err(format!("{option} must differ from source, progress and outputs"));
                }
            }
        }
        outputs.extend(generated.into_iter().map(|p| (option, p)));
    }
    let mut paths = std::collections::BTreeSet::<PathBuf>::new();
    let mut files = std::collections::BTreeSet::new();
    for (name, path) in outputs {
        let resolved = resolve(&path)?;
        if resolved.ancestors().any(|p| paths.contains(p)) || paths.range(resolved.clone()..).next().is_some_and(|p| p.starts_with(&resolved)) || std::fs::metadata(&path).is_ok_and(|m| !files.insert((m.dev(), m.ino()))) {
            return Err(format!("{name} must differ from source, progress and outputs"));
        }
        paths.insert(resolved);
    }
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    let file = args.first().filter(|a| !a.starts_with('-')).ok_or(RUN_USAGE)?;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            #[cfg(feature = "state-store")]
            "--state-store" | "--state-budget-mib" if i + 1 < args.len() => i += 2,
            "--out" | "--dump-surface" | "--dump-state" | "--checkpoint-out" | "--frames-every" | "--frames-dir" | "--frame-width" | "--state-digest" | "--width" | "--light" | "--replay-checkpoint" | "--until-chunk" | "--for-minutes" if i + 1 < args.len() => i += 2,
            "--look" | "--gallery" => i += 1,
            o => return Err(format!("run: unknown argument {o:?} ({RUN_USAGE})")),
        }
    }
    if flag(args, "--replay-checkpoint").is_some() && args.iter().any(|a| matches!(a.as_str(), "--state-store" | "--dump-state")) {
        return Err("--replay-checkpoint cannot resume a state store or per-chunk state dumps".into());
    }
    let text = std::fs::read_to_string(file).map_err(|e| format!("{file}: {e}"))?;
    let stem = Path::new(file).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or("easel".into());
    // a development preview may replay narrower (kernel radii are in mm, so it is not the
    // painting at a smaller size: see notes/workflow.md); the painting is the width it was
    // painted at (LIVE_WIDTH, the width its head names, a sketch's SKETCH_WIDTH)
    let width = match flag(args, "--width") {
        None => width_for(&stem, Some(&text))?,
        Some(w) => w.parse::<usize>().ok().filter(|w| (16..=LIVE_WIDTH * 4).contains(w)).ok_or_else(|| format!("--width {w}: want px, 16 to {}", LIVE_WIDTH * 4))?,
    };
    let out = flag(args, "--out").map(PathBuf::from).unwrap_or_else(|| root().join("out/lua").join(format!("{stem}.png")));
    let chunks = parse_program(&text);
    if chunks.is_empty() {
        return Err(format!("{file}: no chunks (each starts with a line \"{}\")", session::MARK));
    }
    preflight_run(args, Path::new(file), &out, chunks.len())?;
    // the state store's budgets are a 2400 px painting's (as for a live session: resume_as)
    #[cfg(feature = "state-store")]
    if flag(args, "--state-store").is_some() && width > LIVE_WIDTH {
        return Err(format!("--state-store keeps paintings of {LIVE_WIDTH} px; {file} is {width} px"));
    }
    #[cfg(feature = "state-store")]
    let mut store = match flag(args, "--state-store") {
        Some(dir) => {
            let path = PathBuf::from(dir);
            let resolved = normalized_path(&path).map_err(|e| e.to_string())?;
            let mut outputs = vec![PathBuf::from(file), out.clone()];
            for option in ["--state-digest", "--dump-surface", "--checkpoint-out"] {
                if let Some(p) = flag(args, option) { outputs.push(PathBuf::from(p)); }
            }
            if args.iter().any(|a| a == "--look") { outputs.push(out.with_extension("look.png")); }
            for p in outputs {
                if normalized_path(&p).map_err(|e| e.to_string())?.starts_with(&resolved) { return Err("source and outputs must be outside the painting store".into()); }
            }
            for option in ["--dump-state", "--frames-dir"] {
                if let Some(p) = flag(args, option) {
                    let p = normalized_path(Path::new(&p)).map_err(|e| e.to_string())?;
                    if p.starts_with(&resolved) || resolved.starts_with(&p) { return Err(format!("{option} must be separate from the painting store")); }
                }
            }
            let budget = painting_store::PaintingStore::budget(flag(args, "--state-budget-mib").as_deref()).map_err(|e| e.to_string())?;
            let s = painting_store::PaintingStore::open(path, budget, painting_store::PaintingStore::runtime_identity().map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
            if s.current.is_some() { return Err("run --state-store needs an empty store; render-state opens its committed revision without replay".into()); }
            Some(s)
        }
        None if flag(args, "--state-budget-mib").is_some() => return Err("--state-budget-mib needs --state-store".into()),
        None => None,
    };
    // the box the log was painted from, whatever this easel's own box is
    let tubes = session::box_for(Some(&text)).map_err(|e| format!("{file}: {e}"))?;
    // hand-time frames (frames.rs): only read the canvas, so the replay is
    // the same with or without them
    // --light or --gallery: the picture (and its frames) lit on the paint's relief
    if args.iter().any(|a| a == "--gallery") && args.iter().any(|a| a == "--light") {
        return Err(format!("run: --light and --gallery are two lights; give one ({RUN_USAGE})"));
    }
    let light = if args.iter().any(|a| a == "--gallery") { Some(GALLERY_LIGHT) } else { flag(args, "--light").map(|l| light_of(&l)).transpose()? };
    let frames = match (flag(args, "--frames-every"), flag(args, "--frames-dir")) {
        (None, None) => None,
        (Some(e), Some(d)) => {
            let every: f64 = e.parse().map_err(|_| format!("--frames-every {e}: want seconds of hand time"))?;
            let fw = flag(args, "--frame-width").map(|w| w.parse::<u32>().map_err(|_| format!("--frame-width {w}: want px"))).transpose()?.unwrap_or(1000);
            Some((every, PathBuf::from(d), fw))
        }
        _ => return Err(format!("run: --frames-every and --frames-dir go together ({RUN_USAGE})")),
    };
    let mut s = Session::replay_with(width, tubes).and_then(|s| s.with_paper(session::logged_paper(&text))).map_err(|e| e.to_string())?;
    let mut batch = replay_checkpoint::Batch::open(args, &text, width, &mut s, &chunks, &out)?;
    if let Some((every, dir, fw)) = frames.clone() {
        frames::start(every, dir, fw, light, (batch.done > 0).then(|| (batch.done, batch.identity().clone())))?;
    }
    batch.begin()?;
    let frames = frames.is_some();
    // --state-digest: one line of state digests after every chunk (see `state_digest_line`)
    let mut digests = flag(args, "--state-digest").map(|p| std::fs::File::create(&p).map_err(|e| format!("{p}: {e}"))).transpose()?;
    if let (Some(file), Some(rows)) = (&mut digests, &batch.digests) {
        for row in rows { file.write_all(row.as_bytes()).map_err(|e| e.to_string())?; }
    }
    let t0 = Instant::now();
    for (i, c) in chunks.iter().enumerate().skip(batch.done) {
        if batch.stopped() { break; }
        batch.begin_chunk(i + 1)?;
        let r = s.run(c).map_err(|e| format!("chunk {} failed:\n{e}", i + 1))?;
        #[cfg(feature = "state-store")]
        if let Some(store) = &mut store { store.publish(&s, &s.program(&stem)).map_err(|e| format!("state publication after chunk {} failed: {e}", i + 1))?; }
        print!("{}", r.out);
        eprintln!("  chunk {:>3}  {:>7.2}s", i + 1, r.secs);
        if paint::engine7::stats() {
            eprintln!("E7CHUNK	{}	{:.6}", i + 1, r.secs);
        }
        if let Some(f) = digests.as_mut() {
            let line = state_digest_line(&s, i + 1, r.secs);
            f.write_all(line.as_bytes()).map_err(|e| format!("--state-digest: {e}"))?;
            batch.digests.as_mut().unwrap().push(line);
        }
        // --dump-state <dir>: every state value after the chunk (state_dump.rs; reads only)
        if let Some(d) = flag(args, "--dump-state") {
            state_dump::write_chunk(&s, i + 1, Path::new(&d))?;
        }
        if frames {
            frames::chunk_end(s.canvas().as_deref(), i + 1, batch.identity())?;
        }
        if batch.completed(&s, i + 1)? { break; }
    }
    if batch.done < chunks.len() { return Ok(()); }
    let paint_secs = t0.elapsed().as_secs_f64();
    if let Some(d) = flag(args, "--dump-state") {
        state_dump::write_save(&s, &text, Path::new(&d))?;
    }
    let c = s.canvas().ok_or("the program never made a canvas")?.clone();
    if frames {
        frames::finish(&c);
        let rec = frames::stop().ok_or("frames: the recorder went away")?;
        if let Some(e) = rec.err {
            return Err(format!("frames: {e}"));
        }
        eprintln!(
            "frames: {} written ({} hand-time intervals of {}s crossed, {} chunks) over {:.1} min of hand time → {}",
            rec.written,
            rec.ticks,
            flag(args, "--frames-every").unwrap_or_default(),
            chunks.len(),
            rec.hand / 60.0,
            flag(args, "--frames-dir").unwrap_or_default()
        );
    }
    deliver_lit(&c, &out, light)?;
    eprintln!("wrote {} ({} chunks, painted in {paint_secs:.1}s, total {:.1}s)", out.display(), chunks.len(), t0.elapsed().as_secs_f64());
    if let Some(p) = flag(args, "--dump-surface") {
        // the dried surface height (µm) under the saved pixels: little-endian
        // f32, row by row
        let (w, h, v) = c.kept_surface_um();
        let bytes: Vec<u8> = v.iter().flat_map(|x| x.to_le_bytes()).collect();
        std::fs::write(&p, bytes).map_err(|e| format!("{p}: {e}"))?;
        eprintln!("surface {w}x{h} µm → {p}");
    }
    if args.iter().any(|a| a == "--look") {
        let jpg = out.with_extension("look.png");
        let (w, h) = look::look(&c, &look::View::default(), &jpg)?;
        println!("{} ({w}x{h})", jpg.display());
    }
    drop(c);
    if let Some(p) = flag(args, "--checkpoint-out") {
        let path = Path::new(&p);
        if let Some(d) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
            std::fs::create_dir_all(d).map_err(|e| format!("{}: {e}", d.display()))?;
        }
        if !save::write(&s, &text, path)? {
            return Err("the program never made a canvas".into());
        }
        eprintln!("checkpoint → {p} (finishable, not a resumable Lua session)");
    }
    Ok(())
}

const RENDER_CHECKPOINT_USAGE: &str = "render-checkpoint <save.ckpt[.zst]> <out.png> [--mode color|gallery|relief] [--look] [--log painting.lua]";

fn render_checkpoint(args: &[String]) -> Result<(), String> {
    let (mut files, mut mode, mut log, mut bounded) = (Vec::new(), "gallery".to_string(), None::<String>, false);
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--mode" | "--log" if i + 1 < args.len() => {
                if args[i] == "--mode" {
                    mode = args[i + 1].clone();
                } else {
                    log = Some(args[i + 1].clone());
                }
                i += 1;
            }
            "--look" => bounded = true,
            a if !a.starts_with('-') => files.push(a.to_string()),
            o => return Err(format!("render-checkpoint: unknown argument {o:?}\n{RENDER_CHECKPOINT_USAGE}")),
        }
        i += 1;
    }
    let [checkpoint, out] = files.as_slice() else { return Err(RENDER_CHECKPOINT_USAGE.into()) };
    if same_file(Path::new(checkpoint), Path::new(out))? {
        return Err("render-checkpoint output must differ from the save and source log".into());
    }
    if let Some(l) = &log && same_file(Path::new(l), Path::new(out))? {
        return Err("render-checkpoint output must differ from the save and source log".into());
    }
    let light = match mode.as_str() {
        "color" | "normal" => None,
        "gallery" => Some(GALLERY_LIGHT),
        "relief" => Some(look::DISPLAY_PRESETS[1].light),
        o => return Err(format!("--mode {o}: color, gallery, relief")),
    };
    let t0 = Instant::now();
    let r = save::read(Path::new(checkpoint))?;
    if let Some(l) = log {
        let text = std::fs::read_to_string(&l).map_err(|e| format!("{l}: {e}"))?;
        if save::fnv1a(text.as_bytes()) != r.log_fnv || parse_program(&text).len() != r.chunks {
            return Err(format!("{checkpoint} is not a save of {l} (chunk count or text differs)"));
        }
    }
    let c = r.session.canvas().ok_or("the save holds no canvas")?;
    if bounded {
        look::look(&c, &look::View { light, ..look::View::default() }, Path::new(out))?;
    } else {
        deliver_lit(&c, Path::new(out), light)?;
    }
    eprintln!("wrote {out} ({} saved chunks, {mode}, no replay, {:.2}s)", r.chunks, t0.elapsed().as_secs_f64());
    Ok(())
}

#[cfg(feature = "state-store")]
fn render_state(args: &[String]) -> Result<(), String> {
    let Some(dir) = args.first().filter(|s| !s.starts_with('-')) else { return Err("render-state <store directory> <out.png> [--mode color|gallery|relief] [--look]".into()) };
    let Some(out) = args.get(1).filter(|s| !s.starts_with('-')) else { return Err("render-state needs an output PNG".into()) };
    let resolved = normalized_path(Path::new(dir)).map_err(|e| e.to_string())?;
    if normalized_path(Path::new(out)).map_err(|e| e.to_string())?.starts_with(resolved) { return Err("render-state output must be outside the store".into()); }
    let revision = painting_store::ReadRevision::open(PathBuf::from(dir)).map_err(|e| format!("state integrity: {e}"))?;
    revision.protect_output(Path::new(out)).map_err(|e| e.to_string())?;
    for pair in args[2..].windows(2) {
        if pair[0] == "--log" && same_file(Path::new(&pair[1]), Path::new(out))? {
            return Err("render-state output must differ from the source log".into());
        }
    }
    let checkpoint = revision.checkpoint().map_err(|e| e.to_string())?;
    let mut render = vec![checkpoint.to_string_lossy().into_owned()];
    render.extend_from_slice(&args[1..]);
    painting_store::atomic_output(Path::new(out), |part| {
        render[1] = part.to_string_lossy().into_owned();
        render_checkpoint(&render)
    })?;
    eprintln!("committed revision {} ({} source bytes)", revision.id, revision.source.len());
    Ok(())
}

const FINISH_USAGE: &str = "finish <save file> <out.png> [--log painting.lua] [--coats C] [--no-varnish] [--no-cracks] [--relief]
       finish --print-chunk [--coats C] [--no-varnish] [--no-cracks] [--relief]";

/// `easel finish <save> <out.png> [options]`: restore a live session's save (live.ckpt,
/// save.rs) into a fresh session, run the finishing chunk scripts/finish_painting builds
/// (finish.rs `chunk`) and write the PNG, without replaying the log. With `--log`, the save
/// must be of that log (its hash and chunk count). `--print-chunk` prints the chunk only.
fn finish_cmd(args: &[String]) -> Result<(), String> {
    let (mut coats, mut varnish, mut cracks, mut relief) = ("0.4".to_string(), true, true, false);
    let (mut print_chunk, mut log, mut files) = (false, None::<String>, Vec::new());
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--coats" if i + 1 < args.len() => {
                coats = args[i + 1].clone();
                i += 1;
            }
            "--log" if i + 1 < args.len() => {
                log = Some(args[i + 1].clone());
                i += 1;
            }
            "--no-varnish" => varnish = false,
            "--no-cracks" => cracks = false,
            "--relief" => relief = true,
            "--print-chunk" => print_chunk = true,
            a if !a.starts_with('-') => files.push(a.to_string()),
            o => return Err(format!("finish: unknown argument {o:?}\n{FINISH_USAGE}")),
        }
        i += 1;
    }
    // the script's own test: digits with at most one point
    let number = !coats.is_empty() && coats.chars().all(|c| c.is_ascii_digit() || c == '.') && coats.matches('.').count() <= 1 && coats != ".";
    if !number {
        return Err(format!("--coats takes a number, not '{coats}'"));
    }
    let chunk = finish::chunk(&coats, varnish, cracks, relief);
    if print_chunk {
        if !files.is_empty() {
            return Err(FINISH_USAGE.into());
        }
        println!("{chunk}");
        return Ok(());
    }
    let [save, out] = files.as_slice() else { return Err(FINISH_USAGE.into()) };
    if same_file(Path::new(save), Path::new(out))? {
        return Err("finish output must differ from the save and source log".into());
    }
    if let Some(l) = &log && same_file(Path::new(l), Path::new(out))? {
        return Err("finish output must differ from the save and source log".into());
    }
    let t0 = Instant::now();
    let r = save::read(Path::new(save))?;
    if let Some(l) = &log {
        let text = std::fs::read_to_string(l).map_err(|e| format!("{l}: {e}"))?;
        let n = parse_program(&text).len();
        if fnv1a(text.as_bytes()) != r.log_fnv || n != r.chunks {
            return Err(format!("{save} is not a save of {l} ({} chunks in the save, {n} in the log, or the text differs)", r.chunks));
        }
    }
    let restored = t0.elapsed().as_secs_f64();
    let mut s = r.session;
    let mut finish_secs = 0.0;
    if !chunk.is_empty() {
        let ran = s.run(&chunk).map_err(|e| format!("the finishing chunk failed:\n{e}"))?;
        print!("{}", ran.out);
        finish_secs = ran.secs;
    }
    let c = s.canvas().ok_or("the save holds no canvas")?;
    deliver(&c, Path::new(out))?;
    eprintln!("wrote {out} (restored {} chunks in {restored:.1}s, finished in {:.1}s, total {:.1}s)", r.chunks, finish_secs, t0.elapsed().as_secs_f64());
    Ok(())
}

use save::fnv1a;

/// `run --state-digest`'s line for the state after chunk `n` (which took
/// `secs`): the receipt that a change to the engine left a replay's physical
/// state bit for bit as it was, not just its PNG. Compare two files with the
/// `secs=` field dropped: every other field is a digest of state.
///
///   chunk <n> secs=<s> canvas=<16 hex> brushes=<16 hex> nbrushes=<k> studio=<16 hex>
///
/// - `canvas`: FNV-1a-64 of the canvas checkpoint bytes (`Canvas::write_state`
///   with an empty header, see paint's checkpoint.rs): dry picture, relief,
///   film, the wet layer (volume, pigment mix, hiding, stroke ids, coverage),
///   the clock with every pixel's drying state, the drawing, hand time and
///   the engine version. 0 before `canvas{}`.
/// - `brushes`: FNV-1a-64 of the live held brushes' `Debug` text, one a line,
///   in the order they were made: tool and every bristle (load, pigment mix,
///   bend), then the rags in the hand (`paint::rag::Rag`), each on its own
///   line. `nbrushes` counts the brushes (a brush Lua has dropped counts
///   until it is collected).
/// - `studio`: FNV-1a-64 of the studio's seed, clocks, chunk and call
///   counters, `canvas{}` arguments, the piles on the palette and the
///   chunk's RNG, as `Debug` text.
///
/// Floats are hashed by their bits (checkpoint) or their shortest round-trip
/// `Debug` text, which tells every value apart but NaN payloads. Not covered:
/// the Lua state (globals, masks and fields a later chunk may use), the style
/// object and world view in the studio, and derived caches (brush contact
/// surface, film floors of past strokes) that checkpoints leave out too.
/// The format is fixed (recorded goldens depend on it): a field added to
/// `Held`, `Hand` or the checkpoint changes the digests.
fn state_digest_line(s: &Session, n: usize, secs: f64) -> String {
    let mut st = s.st.borrow_mut();
    let canvas = match st.canvas.as_ref() {
        Some(c) => {
            let mut buf = Vec::new();
            c.write_state(&mut buf, "").expect("checkpoint to memory");
            fnv1a(&buf)
        }
        None => 0,
    };
    let brushes: Vec<String> = st.live_brushes().iter().map(|b| format!("{:?}", b.borrow())).collect();
    // the rags in the hand after them (none: the same digest as before rags)
    let rags: Vec<String> = st.live_rags().iter().map(|r| format!("{:?}", r.borrow())).collect();
    let brushes_h = fnv1a(brushes.iter().chain(&rags).cloned().collect::<Vec<_>>().join("\n").as_bytes());
    let studio = format!("seed={} clock={:?} clock0={:?} chunk={} calls={} setup={:?} piles={:?} rng={:?}", st.seed, st.clock, st.clock0, st.chunk, st.calls, st.setup, st.hand.piles, st.rng);
    let studio_h = fnv1a(studio.as_bytes());
    // (knives came with engine 4: a painting without one keeps the line it had)
    let knives: Vec<String> = st.live_knives().iter().map(|k| format!("{:?}", k.borrow())).collect();
    let knives_s = if knives.is_empty() { String::new() } else { format!(" knives={:016x} nknives={}", fnv1a(knives.join("\n").as_bytes()), knives.len()) };
    format!("chunk {n} secs={secs:.3} canvas={canvas:016x} brushes={brushes_h:016x} nbrushes={} studio={studio_h:016x}{knives_s}\n", brushes.len())
}

#[cfg(test)]
mod tests {
    /// A motif for `look --ref` is often a photograph: the easel reads JPEG
    /// as well as PNG.
    #[test]
    fn a_jpeg_motif_can_be_read() {
        let im = image::RgbImage::from_fn(16, 12, |x, y| image::Rgb([(x * 16) as u8, (y * 20) as u8, 128]));
        let mut jpg = Vec::new();
        im.write_to(&mut std::io::Cursor::new(&mut jpg), image::ImageFormat::Jpeg).unwrap();
        let back = image::load_from_memory(&jpg).unwrap().to_rgb8();
        assert_eq!(back.dimensions(), (16, 12));
    }
    use super::*;

    /// A pastel stick held up (with the paper-and-pastel switch) only reads, as the held
    /// knife: the state digest, the log and the clock are as they were, and the stick's
    /// wear in the Lua heap too. It tints the passage where the stick would touch and says
    /// how much; laid on its side it touches more, and its face lies near the paper over
    /// much more.
    #[test]
    fn a_held_stick_shows_where_it_would_touch_and_changes_nothing() {
        let mut s = Session::new(1000).unwrap().with_paper(true).unwrap();
        s.run(r#"canvas{size=200, aspect=1.25, seed=3, paper={grammage=200}}"#).unwrap();
        s.run(r#"P = {glow = pastel(pile{{"vermilion", 1}}, {kind="soft"})}"#).unwrap();
        let before = (state_digest_line(&s, 2, 0.0), s.program("t"), s.st.borrow().clock);
        let plain = look::View::default();
        let at = (500.0, 400.0);
        let (stick, turned) = s.held_stick("P.glow").unwrap();
        assert_eq!(turned, 0.0);
        let (w, h, png, note) = stick_hold_look(&s, "P.glow", at, [1.5, 60.0, 45.0, 0.0], &plain).unwrap();
        let bare = look::render(&s.canvas().unwrap(), &look::View { crop: Some([at.0 - 60.0, at.1 - 60.0, at.0 + 60.0, at.1 + 60.0]), ..look::View::default() }).unwrap();
        assert_eq!((w, h), (bare.0, bare.1));
        let (held, bare) = (image::load_from_memory(&png).unwrap().to_rgb8(), image::load_from_memory(&bare.2).unwrap().to_rgb8());
        // (the vermilion where it touches: redder than the paper)
        let red = held.pixels().zip(bare.pixels()).filter(|(a, b)| a[0] as i32 - a[2] as i32 > b[0] as i32 - b[2] as i32 + 40).count();
        assert!(red > 0, "the stick's contact shows in its color");
        let touch = |note: &str| note.strip_prefix("touching ").and_then(|t| t.split(' ').next()).and_then(|t| t.parse::<f32>().ok()).unwrap();
        assert!(touch(&note) > 0.0 && note.contains("crumbs settle over"), "{note}");
        let (.., flat) = stick_hold_look(&s, "P.glow", at, [2.0, 0.0, 135.0, 0.0], &plain).unwrap();
        // (laid flat, its face lies near the paper over much more: it rests on the highest fibers only)
        let bed = |note: &str| note.split("settle over ").nth(1).and_then(|t| t.split(' ').next()).and_then(|t| t.parse::<f32>().ok()).unwrap();
        assert!(touch(&flat) > touch(&note) && bed(&flat) > 3.0 * bed(&note), "on its side it covers more: {flat} vs {note}");
        assert_eq!(before, (state_digest_line(&s, 2, 0.0), s.program("t"), s.st.borrow().clock));
        assert_eq!(s.held_stick("P.glow").unwrap().0, stick, "no wear");
        assert!(s.held_stick("nope").unwrap_err().contains("holds no pastel stick"));
        assert!(stick_hold_look(&s, "P.glow", (1400.0, 400.0), [1.5, 60.0, 45.0, 0.0], &plain).unwrap_err().contains("not on the canvas"));
        assert!(stick_hold_look(&s, "P.glow", at, [1.5, 60.0, 45.0, 0.0], &look::View { mirror: true, ..look::View::default() }).is_err());
    }

    #[test]
    fn a_held_stick_keeps_masks_and_scaled_contact() {
        let mut s = Session::new(4800).unwrap().with_paper(true).unwrap();
        let c = Canvas::new_window(4800, 1.25, [0.8; 3], Some(paint::Crop { units: [400.0, 300.0, 600.0, 500.0], margin: 0.0 }))
            .with_size_mm(200.0).with_paper(paint::paper::Paper::drawing(3));
        let mask = std::sync::Arc::new(paint::Mask::from_fn(c.frame(), |_, _| 1.0));
        s.st.borrow_mut().canvas = Some(c);
        s.run(r#"p = pastel(pile{{"vermilion", 1}}, {kind="soft"})"#).unwrap();
        let crop = [440.0, 340.0, 560.0, 460.0];
        let v = look::View { crop: Some(crop), overlays: vec![mask], ..look::View::default() };
        let c = s.canvas().unwrap();
        let (stick, _) = s.held_stick("p").unwrap();
        let pose = paint::pastel::Pose { force: 1.5, alt: 60f32.to_radians(), az: 45f32.to_radians(), roll: 0.0 };
        let cells = c.stick_contact(&stick, 500.0, 400.0, pose);
        let (touch, bed) = (cells.iter().filter(|(_, k)| *k == 2).count(), cells.iter().filter(|(_, k)| *k == 1).count());
        let f = c.window();
        let (x0, y0) = ((crop[0] * f.scale).round() as usize, (crop[1] * f.scale).round() as usize);
        let expected: std::collections::BTreeSet<_> = cells.iter().filter(|(_, k)| *k == 2)
            .map(|(i, _)| (((i % f.w + f.x0 - x0) / 2) as u32, ((i / f.w + f.y0 - y0) / 2) as u32)).collect();
        let mm2 = c.px_mm().powi(2);
        let bare = look::render(&c, &v).unwrap();
        let plain = look::render(&c, &look::View { crop: Some(crop), ..look::View::default() }).unwrap();
        drop(c);
        let (w, h, png, note) = stick_hold_look(&s, "p", (500.0, 400.0), [1.5, 60.0, 45.0, 0.0], &v).unwrap();
        assert_eq!((w, h), (288, 288));
        assert_eq!(note, format!("touching {:.1} mm², crumbs settle over {:.1} mm² more", touch as f32 * mm2, bed as f32 * mm2));
        let held = image::load_from_memory(&png).unwrap().to_rgb8();
        let bare = image::load_from_memory(&bare.2).unwrap().to_rgb8();
        let plain = image::load_from_memory(&plain.2).unwrap().to_rgb8();
        assert_eq!(held.get_pixel(0, 0), bare.get_pixel(0, 0));
        assert_ne!(held.get_pixel(0, 0), plain.get_pixel(0, 0));
        let color = stick.color.map(|v| (paint::color::linear_to_srgb(v) * 255.0).round() as u8);
        let contact: std::collections::BTreeSet<_> = held.enumerate_pixels().filter(|(_, _, p)| p.0 == color).map(|(x, y, _)| (x, y)).collect();
        assert!(!contact.is_empty(), "the contact stays visible after downsampling");
        assert_eq!(contact, expected, "the resized contact covers its source footprint");
    }

    #[test]
    fn reference_looks_do_not_bypass_stick_pose_gating() {
        for paper in [false, true] {
            let mut server = Server {
                name: "test".into(), build: 0, s: Session::new(160).unwrap().with_paper(paper).unwrap(),
                frames: false, written: None, replayed: 0, scratch: None, display: None,
                replay: replay_checkpoint::Live::new(), hand_frames: None,
                #[cfg(feature = "state-store")] committed: None,
                #[cfg(feature = "state-store")] state_failed: false,
            };
            for flag in ["--pose", "--side"] {
                let args = ["--ref", "missing.jpg", flag, "garbage"].map(String::from);
                let e = server.look(&args, None).unwrap_err();
                assert!(e.contains(if paper { "go with --hold" } else { "paper-and-pastel switch" }), "{e}");
            }
        }
    }
}
