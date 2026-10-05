//! easel: a live Lua painting session over the claude-paint engine.
//!
//!   easel open [<name>]              start (or reattach to) a session
//!   easel do '<lua>' | -f chunk.lua | -            run a chunk on the live canvas
//!   easel look [--crop x0,y0,x1,y1] [--mode value|squint|mirror] [--grid [step]] [--size N]
//!   easel log | status | globals | save [path] | frames on|off | close
//!   easel check                                    (replay build) replay the log, compare
//!   easel note '<text>' | -                        append to notes/journal.md
//!   easel run paintings/lua/<name>.lua [--out path] [--look] [--state-digest digests.txt]
//!
//! Two builds (see `USAGE`). The replay build (feature `replay`, on by
//! default: developers, tests and the outside runner) has named sessions
//! (`-s`, `EASEL_SESSION`), `EASEL_ROOT`, `run`, `check` and `hash-probe`. The
//! painter build (`--no-default-features`) has none of them: its studio is
//! the directory above the executable's (`<studio>/bin/easel`) and holds one
//! painting, `PAINTING`. See notes/easel_guide.md.

mod api;
mod board;
mod check;
mod depth;
mod draw_edges;
mod draw_outline;
#[cfg(feature = "finish")]
mod finish;
mod form;
#[cfg(feature = "replay")]
mod frames;
#[cfg(feature = "replay")]
mod legacy;
mod look;
mod save;
mod palette_look;
mod session;
#[cfg(feature = "replay")]
mod state_dump;
mod time;
mod world;
#[cfg(all(test, feature = "replay"))]
mod thinner_measure;
#[cfg(all(test, feature = "replay"))]
mod thinner_tests;

use paint::Canvas;
use paint::color::linear_to_srgb;
use session::{Session, parse_program, root};
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, Instant};

/// The one width a painting is painted, replayed and delivered at (px).
const LIVE_WIDTH: usize = 2400;

/// The width a painting paints at: an existing one its log's (a sketch if
/// its head says so, `session::SKETCH_MARK`), a new one a sketch's if the
/// session's name starts with "sketch".
fn width_for(name: &str, log: Option<&str>) -> usize {
    let sketch = match log {
        Some(text) => session::logged_sketch(text),
        None => name.starts_with("sketch"),
    };
    if sketch { session::SKETCH_WIDTH } else { LIVE_WIDTH }
}

/// The painter build's one session.
#[cfg(not(feature = "replay"))]
const PAINTING: &str = "painting";

#[cfg(not(feature = "replay"))]
const USAGE: &str = "easel: a live painting session (see notes/easel_guide.md)

  easel open          start or reattach; replays paintings/lua/painting.lua if it exists
  easel do '<lua>'  |  easel do -f chunk.lua  |  easel do - (stdin)     [--look] also looks afterwards
  easel look [--crop x0,y0,x1,y1] [--mode value,squint,mirror,relief,gallery] [--grid [step]] [--size 1000]
             [--survey]   the whole canvas at full detail, in tiles
             [--compare <earlier look png>]   that look beside this one
             [--hold <knife or pile> --at x,y]   (speculative) the loaded knife held up to the canvas there
  easel look --palette   the palette board: every heap knifed out thick and smeared thin across a black stripe
  easel log           the painting so far (= paintings/lua/painting.lua)
  easel status        chunks, width, canvas
  easel globals       the painting's globals, one a line: chunk that last set it, name, what it holds
  easel save [path] [--light az,el | --gallery]   the canvas as a PNG (default out/easel/painting/painting.png), lit on its relief if asked
  easel frames on|off save a look after every chunk
  easel close         end the session (the log stays)
  easel note '<text>' | easel note - (stdin)    append a dated entry to notes/journal.md
  easel tubes [--markdown]   the tubes in the box (--markdown: as a table)";

#[cfg(feature = "replay")]
const USAGE: &str = "easel: a live painting session (see notes/easel_guide.md)

  easel open <name>    start or reattach; replays paintings/lua/<name>.lua if it exists
  easel do '<lua>'  |  easel do -f chunk.lua  |  easel do - (stdin)     [--look] also looks afterwards
  easel look [--crop x0,y0,x1,y1] [--mode value,squint,mirror,relief,gallery] [--grid [step]] [--size 1000]
             [--survey]   the whole canvas at full detail, in tiles
             [--compare <earlier look png>]   that look beside this one
             [--hold <knife or pile> --at x,y]   (speculative) the loaded knife held up to the canvas there
  easel look --palette   the palette board: every heap knifed out thick and smeared thin across a black stripe
  easel log           the session so far (= paintings/lua/<name>.lua)
  easel status        chunks, width, canvas
  easel globals       the painting's globals, one a line: chunk that last set it, name, what it holds
  easel save [path] [--light az,el | --gallery]   the canvas as a PNG (default out/easel/<name>/<name>.png), lit on its relief if asked
  easel frames on|off save a look after every chunk
  easel check         replay the log from scratch and compare with the live canvas
  easel close         end the session (the log stays)
  easel note '<text>' | easel note - (stdin)    append a dated entry to notes/journal.md
  easel run <file.lua> [--out path.png] [--look] [--state-digest digests.txt]    replay at 2400px and write the PNG
      [--frames-every <s> --frames-dir <dir> [--frame-width 1000]]   and a frame per <s> of hand time
      [--width <px>]   replay narrower, a preview for development (not the painting)
  easel finish <save> <out.png> [--log painting.lua] [--coats C] [--no-varnish] [--no-cracks] [--relief]
                      finish a closed session's save (out/easel/<name>/live.ckpt) without a replay
  easel tubes [--markdown]   the tubes in the box a new painting takes (--markdown: as a table)

  A new painting takes its box from a file `box` next to this executable, else EASEL_BOX,
  else the default tube box; its log names the box (a line \"--@ box <name>\"), and every
  replay of the log uses that box.

  -s <name> (or EASEL_SESSION) picks the session; default: the last opened.";

fn main() -> ExitCode {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let name = match take_session(&mut args) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    let Some(cmd) = args.first().cloned() else {
        println!("{USAGE}");
        return ExitCode::SUCCESS;
    };
    let rest = args[1..].to_vec();
    let r = match cmd.as_str() {
        "open" => open(&rest),
        "serve" => serve(&rest),
        #[cfg(feature = "replay")]
        "run" => run(&rest),
        #[cfg(all(feature = "replay", feature = "finish"))]
        "finish" => finish_cmd(&rest),
        "note" => note(&rest, name),
        "tubes" => tubes(&rest),
        #[cfg(feature = "replay")]
        "hash-probe" => {
            println!("{}", session::hash_probe());
            Ok(())
        }
        "help" | "-h" | "--help" => {
            println!("{USAGE}");
            Ok(())
        }
        "do" | "look" | "log" | "status" | "globals" | "save" | "frames" | "close" => client(&cmd, &rest, name),
        // replaying the log to compare it with the live canvas is the runner's, after the
        // painter's session (scripts/check_painting); the painter build has no `check`
        #[cfg(feature = "replay")]
        "check" => client(&cmd, &rest, name),
        o => Err(format!("easel: no command {o:?}\n\n{USAGE}")),
    };
    match r {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

/// The session a command is for: `-s <name>` (taken out of `args`), else
/// `EASEL_SESSION`, else (None) the last one opened.
#[cfg(feature = "replay")]
fn take_session(args: &mut Vec<String>) -> Result<Option<String>, String> {
    let mut name: Option<String> = std::env::var("EASEL_SESSION").ok();
    if let Some(i) = args.iter().position(|a| a == "-s" || a == "--session")
        && i + 1 < args.len()
    {
        name = Some(args.remove(i + 1));
        args.remove(i);
    }
    Ok(name)
}

/// The painter build has one session: `-s` is refused, `EASEL_SESSION`
/// ignored.
#[cfg(not(feature = "replay"))]
fn take_session(args: &mut Vec<String>) -> Result<Option<String>, String> {
    if let Some(a) = args.iter().find(|a| *a == "-s" || *a == "--session") {
        return Err(format!("easel: no option {a:?}: this studio has one painting, there are no sessions to pick\n\n{USAGE}"));
    }
    Ok(Some(PAINTING.to_string()))
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
    let file = format!("{:016x}.sock", fnv1a(dir.as_os_str().as_encoded_bytes()));
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

#[cfg(feature = "replay")]
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
    #[cfg(feature = "replay")]
    let not_running = || format!("no easel session {name:?} running: easel open {name}");
    #[cfg(not(feature = "replay"))]
    let not_running = || "no painting open: easel open".to_string();
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
    }
    let (ok, body) = request(&name, cmd, &args, &payload)?;
    if ok {
        print!("{body}");
        Ok(())
    } else {
        Err(body.trim_end().to_string())
    }
}

/// The session `open` is for: the name given (replay build).
#[cfg(feature = "replay")]
fn open_name(args: &[String]) -> Result<String, String> {
    let name = args.first().filter(|a| !a.starts_with('-')).ok_or("open <name>")?.clone();
    valid_name(&name)?;
    if args.len() != 1 {
        return Err("open: live sessions are fixed at 2400px; no width option".into());
    }
    Ok(name)
}

/// The painter build opens its one painting: `open` takes nothing.
#[cfg(not(feature = "replay"))]
fn open_name(args: &[String]) -> Result<String, String> {
    if let Some(a) = args.first() {
        return Err(format!("open: this studio has one painting; easel open takes no name or option (got {a:?})"));
    }
    Ok(PAINTING.to_string())
}

/// Remember the session opened last (replay build: commands without `-s`
/// go to it).
fn set_current(name: &str) -> Result<(), String> {
    #[cfg(feature = "replay")]
    std::fs::write(root().join("out/easel/current"), name).map_err(|e| e.to_string())?;
    let _ = name;
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
    if let Ok((ok, st)) = request(&name, "status", &[], &[]) {
        if !ok {
            return Err(st);
        }
        set_current(&name)?;
        print!("reattached to {name:?}: {st}");
        return Ok(());
    }
    // Replaying a real painting can take tens of minutes. Follow the server's
    // startup log so `open` shows that work instead of looking dead, and only
    // time out when the replay itself has made no progress for 30 minutes.
    // The server is started only by the `open` that takes the session's lock; any
    // other waits for it (or, if it has gone, takes the lock in turn).
    let mut started = false;
    let mut shown = 0usize;
    let mut last_progress = Instant::now();
    loop {
        if let Some(lock) = try_lock(&name)? {
            if started {
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
        // started again by another open since we last read it
        if log.len() < shown {
            shown = 0;
        }
        if let Some(end) = log[shown..].rfind('\n').map(|i| shown + i + 1) {
            for line in log[shown..end].lines() {
                if line.starts_with("resuming ") || line.starts_with("resumed ") || line.starts_with("warning") {
                    println!("{line}");
                }
            }
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            shown = end;
            last_progress = Instant::now();
        }
        if let Ok((true, st)) = request(&name, "status", &[], &[]) {
            set_current(&name)?;
            print!("easel {name:?} open: {st}");
            return Ok(());
        }
        if log.contains("easel: fatal") {
            return Err(log);
        }
        if last_progress.elapsed() > Duration::from_secs(1800) {
            return Err(format!("session made no replay progress for 30 minutes; see {}", dir.join("server.log").display()));
        }
    }
}

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
    s: Session,
    frames: bool,
    /// The log text as the easel last wrote (or read) it: if the file on
    /// disk differs, it was changed outside the session.
    written: Option<String>,
    /// How many of the log's chunks the state was replayed from (at reopen or rebuild),
    /// not painted live.
    replayed: usize,
}

fn serve(args: &[String]) -> Result<(), String> {
    let name = args.first().ok_or("serve <name>")?.clone();
    valid_name(&name)?;
    #[cfg(not(feature = "replay"))]
    if name != PAINTING {
        return Err(format!("easel: fatal: this studio has one painting, {PAINTING:?}"));
    }
    if args.len() != 1 {
        return Err("easel: fatal: live sessions are fixed at 2400px".into());
    }
    let _lock = serve_lock(&name).map_err(|e| format!("easel: fatal: {e}"))?;
    let mut srv = Server::resume(name.clone()).map_err(|e| format!("easel: fatal: {e}"))?;
    let sock = sock_path(&name);
    // the lock is ours: a socket there is a dead server's
    let _ = std::fs::remove_file(&sock);
    let l = UnixListener::bind(&sock).map_err(|e| format!("easel: fatal: bind {}: {e}", sock.display()))?;
    let _ = std::io::stdout().flush();
    // the check running on its own thread, if any (check.rs; replay build only)
    #[cfg(feature = "replay")]
    let mut checking: Option<check::Job> = None;

    for conn in l.incoming() {
        let Ok(mut conn) = conn else { continue };
        // a request is a few KB sent at once: a client that stalls mid-request must not hold
        // the one thread that serves every request
        let _ = conn.set_read_timeout(Some(Duration::from_secs(60)));
        let req = match read_request(&mut conn) {
            Ok(r) => r,
            Err(e) => {
                let _ = conn.write_all(format!("err\nthe easel couldn't read the request: {e}\n").as_bytes());
                eprintln!("request dropped: {e}");
                continue;
            }
        };
        let _ = conn.set_read_timeout(None);
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
        #[cfg(feature = "replay")]
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
        let r = srv.handle(&cmd, &args, payload);
        let reply = match &r {
            Ok(b) => format!("ok\n{b}"),
            Err(e) => format!("err\n{e}\n"),
        };
        let _ = conn.write_all(reply.as_bytes());
        eprintln!("{cmd} {:.2}s {}", t0.elapsed().as_secs_f64(), if r.is_ok() { "ok" } else { "err" });
        if cmd == "close" && r.is_ok() {
            #[cfg(feature = "replay")]
            if let Some(j) = checking.take() {
                j.stop();
            }
            let _ = std::fs::remove_file(&sock);
            break;
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
            srv.replayed = srv.s.log.len();
            eprintln!("rebuilt {:.2}s", t0.elapsed().as_secs_f64());
        }
    }
    Ok(())
}

/// Only the socket and immutable integrity witness cross threads. Lua (and its
/// Rc-backed engine state) stays on the serving thread throughout the replay.
fn rebuild_serving_status(listener: &UnixListener, srv: &mut Server) -> Result<(), String> {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    let completed = AtomicUsize::new(0);
    let done = AtomicBool::new(false);
    struct Stop<'a>(&'a AtomicBool);
    impl Drop for Stop<'_> {
        fn drop(&mut self) {
            self.0.store(true, Ordering::Release);
        }
    }
    // An absolute request deadline also bounds clients that trickle bytes.
    struct Reader<'a> {
        conn: &'a mut UnixStream,
        until: Instant,
        done: &'a AtomicBool,
    }
    impl Read for Reader<'_> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let remaining = self.until.saturating_duration_since(Instant::now());
            if remaining.is_zero() || self.done.load(Ordering::Acquire) {
                return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "rebuild request deadline reached"));
            }
            self.conn.set_read_timeout(Some(remaining))?;
            self.conn.read(buf)
        }
    }
    let total = srv.s.log.len();

    let name = &srv.name;
    let written = srv.written.as_deref().ok_or("session integrity: uninitialized log")?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    let result = std::thread::scope(|scope| {
        let worker = scope.spawn(|| -> Result<(), String> {
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
                // keep the scoped helper alive after replay finishes.
                conn.set_nonblocking(false).map_err(|e| e.to_string())?;
                let _ = conn.set_read_timeout(Some(Duration::from_millis(100)));
                let _ = conn.set_write_timeout(Some(Duration::from_millis(100)));
                let mut reader = Reader { conn: &mut conn, until: Instant::now() + Duration::from_millis(100), done: &done };
                let reply = match read_request(&mut reader) {
                    Err(e) => format!("err\n{e}\n"),
                    Ok(req) => match validate_log(name, written) {
                        Err(e) => format!("err\n{e}\n"),
                        Ok(()) => {
                            let head = req.split(|b| *b == b'\n').next().unwrap_or_default();
                            if head.split(|b| *b == b'\t').next() == Some(b"status".as_slice()) {
                                format!("ok\nrebuilding from the log ({} of {total} chunks)\n", completed.load(Ordering::Acquire))
                            } else {
                                "err\nthe easel is rebuilding from the log; retry after it finishes\n".into()
                            }
                        }
                    },
                };
                let _ = conn.write_all(reply.as_bytes());
            }
            Ok(())
        });
        let stop = Stop(&done);
        let replay = srv.s.rebuild_with_progress(|k, _| completed.store(k, Ordering::Release));
        drop(stop);
        worker.thread().unpark();
        let served = worker.join().map_err(|_| "rebuild status listener panicked".to_string())?;
        replay.and(served)
    });
    let blocking = listener.set_nonblocking(false).map_err(|e| e.to_string());
    result.and(blocking)
}

/// The session's committed record: the log as the session last wrote it.
fn witness_path(name: &str) -> PathBuf {
    session_dir(name).join("committed.lua")
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
fn hold_look(s: &Session, name: &str, at: (f32, f32), v: &look::View) -> Result<(usize, usize, Vec<u8>), String> {
    use image::ImageEncoder;
    if v.size.is_some() || v.grid.is_some() || v.mirror || v.palette {
        return Err("look --hold takes --at, --crop, --mode value, squint, relief or gallery and --light, nothing else".into());
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
    let (_, _, png) = look::render(&c, &seen_as(crop))?;
    let mut passage = image::load_from_memory(&png).map_err(|e| e.to_string())?.to_rgb8();
    // the knife: its paint laid thick on a steel blade, on a board of its own with the
    // painting's pixels to the unit, millimetres to the unit and engine
    let f = c.frame();
    let (len, wide) = (2.0 * blade, blade + 12.0);
    let board_h = wide + 20.0;
    let mut board = Canvas::new(f.full_w.max(16), 1000.0 / board_h, paint::hex("#9aa0a6")).with_size_mm(1000.0 * c.mm_per_unit()).with_engine(c.engine());
    let mut knife = paint::Knife::new(blade);
    knife.load(paint, full);
    let cy = board_h / 2.0;
    // (the pull starts before the part shown, so the paint comes to the blade's very end)
    board.knife(&mut knife, &[(10.0, cy), (50.0 + len, cy)], (0.15, 0.1), None, true, 0.25);
    let (_, _, bpng) = look::render(&board, &seen_as([30.0, cy - wide / 2.0, 30.0 + len, cy + wide / 2.0]))?;
    let blade_img = image::load_from_memory(&bpng).map_err(|e| e.to_string())?.to_rgb8();
    // held over the passage: the blade's end at the point, its length to the right
    let x0 = ((crop[0] * f.scale).round().max(0.0) as usize).clamp(f.x0, f.x0 + f.w) as i64;
    let y0 = ((crop[1] * f.scale).round().max(0.0) as usize).clamp(f.y0, f.y0 + f.h) as i64;
    let x = (at.0 * f.scale).round() as i64 - x0;
    let y = (at.1 * f.scale).round() as i64 - y0 - blade_img.height() as i64 / 2;
    image::imageops::overlay(&mut passage, &blade_img, x, y);
    let (w, h) = (passage.width(), passage.height());
    let mut out = Vec::new();
    image::codecs::png::PngEncoder::new(&mut out).write_image(passage.as_raw(), w, h, image::ExtendedColorType::Rgb8).map_err(|e| e.to_string())?;
    Ok((w as usize, h as usize, out))
}

/// Write a look as `dir/look-NNNN.png`, NNNN one above the highest there, never over a file
/// that exists (a pruned look or a stray look-prefixed file doesn't make it reuse a name).
fn new_look(dir: &Path, png: &[u8]) -> Result<PathBuf, String> {
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let number = |name: &str| name.strip_prefix("look-")?.strip_suffix(".png")?.parse::<u64>().ok();
    let mut n = std::fs::read_dir(dir).map_err(|e| e.to_string())?.filter_map(|e| number(&e.ok()?.file_name().to_string_lossy())).max().unwrap_or(0);
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
        validate_log(&self.name, self.written.as_deref().ok_or("session integrity: uninitialized log")?)
    }

    fn resume(name: String) -> Result<Self, String> {
        let lp = log_path(&name);
        let text = if lp.exists() || witness_path(&name).exists() { Some(std::fs::read_to_string(&lp).map_err(|e| format!("session integrity: {e}"))?) } else { None };
        if let Some(t) = &text {
            validate_log(&name, t)?;
        }
        // an existing painting goes on with the box its log names; a new one takes the
        // configured box (session::box_for)
        let tubes = session::box_for(text.as_deref())?;
        let width = width_for(&name, text.as_deref());
        let mut srv = Self { name, s: Session::with_box(width, tubes).map_err(|e| e.to_string())?, frames: false, written: None, replayed: 0 };
        if let Some(text) = text {
            srv.written = Some(text.clone());
            let chunks = parse_program(&text);
            // a failed chunk refuses the whole reopen: no snapshots, no time limit (session.rs)
            srv.s.set_replaying(true);
            for (i, chunk) in chunks.iter().enumerate() {
                let t0 = Instant::now();
                eprintln!("resuming chunk {}/{}", i + 1, chunks.len());
                srv.s.run(chunk).map_err(|e| format!("session integrity: replay failed at chunk {}: {e}; refusing partial session", i + 1))?;
                eprintln!("resumed chunk {}/{} {:.2}s", i + 1, chunks.len(), t0.elapsed().as_secs_f64());
            }
            srv.s.set_replaying(false);
            srv.replayed = chunks.len();
            if srv.s.program(&srv.name) != text {
                return Err("session integrity: noncanonical or incomplete log; refusing replay".into());
            }
            println!("resumed {} chunks from {}", srv.s.log.len(), lp.display());
        } else {
            srv.save_log()?;
        }
        Ok(srv)
    }

    /// Append only the new suffix. Commit the redundant witness afterward.
    /// A crash between writes fails closed on reopen, never partial replay.
    fn save_log(&mut self) -> Result<String, String> {
        let p = log_path(&self.name);
        let text = self.s.program(&self.name);
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
        Ok(String::new())
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
        std::fs::write(&txt, format!("chunks {}\nreplayed {}\n", self.s.log.len(), self.replayed)).map_err(|e| format!("{}: {e}", txt.display()))?;
        Ok(format!("the live canvas is in {}\n", png.display()))
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
        let (mut hold, mut at) = (None::<String>, None::<String>);
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
                "--survey" => survey = true,
                "--compare" => {
                    compare = Some(PathBuf::from(args.get(i + 1).ok_or("--compare needs an earlier look's png")?));
                    i += 1;
                }
                a => rest.push(a.to_string()),
            }
            i += 1;
        }
        if survey && compare.is_some() {
            return Err("look: --survey and --compare are two looks; ask for one".into());
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
            let v = look::View::parse(&rest)?;
            let t0 = Instant::now();
            let (w, h, png) = hold_look(&self.s, &pile, (p[0], p[1]), &v)?;
            let path = new_look(&session_dir(&self.name), &png)?;
            return Ok(format!("{} ({w}x{h}, {:.2}s): {pile} held up to the canvas at {},{}\n", path.display(), t0.elapsed().as_secs_f64(), p[0], p[1]));
        }
        if survey {
            return self.survey(&rest);
        }
        if let Some(prev) = compare {
            return self.compare(&rest, &prev);
        }
        let args = &rest[..];
        let v = look::View::parse(args)?;
        let t0 = Instant::now();
        if v.palette {
            let (w, h, png) = palette_look(&self.s)?;
            let path = new_look(&session_dir(&self.name), &png)?;
            return Ok(format!("{} ({w}x{h}, {:.2}s)\n", path.display(), t0.elapsed().as_secs_f64()));
        }
        let c = self.s.canvas().ok_or("no canvas yet: the first chunk is canvas{...}")?;
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
        Ok(format!("{} ({w}x{h}, {:.2}s)\n", path.display(), t0.elapsed().as_secs_f64()))
    }

    /// The whole canvas at full detail (1:1), in tiles of at most 500 units a side,
    /// in reading order; the view's modes (gallery, value...) apply to each.
    fn survey(&mut self, args: &[String]) -> Result<String, String> {
        if args.iter().any(|a| a == "--crop" || a == "--size") {
            return Err("look --survey covers the whole canvas at full detail: no --crop or --size".into());
        }
        let c = self.s.canvas().ok_or("no canvas yet: the first chunk is canvas{...}")?;
        // (the canvas as it is seen, lit or not, once for all the tiles)
        let seen = match look::View::parse(args)?.light {
            Some((az, el)) => c.seen_lit(az, el, 1.0),
            None => c.seen(),
        };
        let (wu, hu) = (c.width(), c.height());
        // (a tile is at most 500 units: 1200 px of a live canvas, the most a crop takes)
        let cols = (wu / 500.0).ceil().max(1.0) as usize;
        let rows = (hu / 500.0).ceil().max(1.0) as usize;
        let (tw, th) = (wu / cols as f32, hu / rows as f32);
        let mut out = format!("survey: {rows} rows x {cols} columns of {tw:.0} x {th:.0} units, at full detail\n");
        for r in 0..rows {
            for k in 0..cols {
                let crop = format!("{},{},{},{}", k as f32 * tw, r as f32 * th, (k + 1) as f32 * tw, (r + 1) as f32 * th);
                let mut a = args.to_vec();
                a.extend(["--crop".to_string(), crop.clone()]);
                let v = look::View::parse(&a)?;
                let (w, h, png) = look::render_seen(&c, &v, Some(&seen))?;
                let p = new_look(&session_dir(&self.name), &png)?;
                out += &format!("{} ({w}x{h}): row {} column {} ({crop})\n", p.display(), r + 1, k + 1);
            }
        }
        Ok(out)
    }

    /// An earlier look (left) beside the same view of the canvas now (right), at
    /// the same height, for judging what a change did.
    fn compare(&mut self, args: &[String], prev: &Path) -> Result<String, String> {
        let mut a = args.to_vec();
        if !a.iter().any(|x| x == "--size" || x == "--crop") {
            a.extend(["--size".to_string(), "800".to_string()]);
        }
        let v = look::View::parse(&a)?;
        let c = self.s.canvas().ok_or("no canvas yet: the first chunk is canvas{...}")?;
        let (_, _, png) = look::render(&c, &v)?;
        let now = image::load_from_memory(&png).map_err(|e| e.to_string())?.to_rgb8();
        // an earlier look of this studio: a picture under its folder, nothing outside it
        let studio = root().canonicalize().map_err(|e| e.to_string())?;
        let prev = if prev.is_absolute() { prev.to_path_buf() } else { studio.join(prev) };
        let real = prev.canonicalize().map_err(|e| format!("--compare {}: {e}", prev.display()))?;
        if !real.starts_with(&studio) {
            return Err(format!("--compare {}: an earlier look is a picture in this studio ({})", prev.display(), studio.display()));
        }
        let before = image::open(&real).map_err(|e| format!("--compare {}: {e}", prev.display()))?.to_rgb8();
        let h = now.height();
        let bw = ((before.width() as f64 * h as f64 / before.height() as f64).round() as u32).max(1);
        if u64::from(bw) > 4 * u64::from(now.width()) || 4 * u64::from(bw) < u64::from(now.width()) {
            return Err("--compare: the earlier look crop has a different shape".into());
        }
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
        Ok(format!("{} ({}x{}): left {}, right now\n", p.display(), both.width(), h, prev.display()))
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
    #[cfg(feature = "replay")]
    fn check_input(&self) -> Result<check::Input, String> {
        self.ready()?;
        Ok(check::Input {
            width: self.s.st.borrow().width,
            tubes: self.s.tube_box(),
            chunks: self.s.log.iter().map(|c| c.src.clone()).collect(),
            live: self.s.canvas().map(|a| (bits(&a.seen()), bits_f(a.surface_um()))),
        })
    }

    fn handle(&mut self, cmd: &str, args: &[String], payload: &str) -> Result<String, String> {
        self.ready()?;
        match cmd {
            "note" => append_note(self.s.st.borrow().clock, payload),
            "status" => Ok(format!("{}\n", self.s.status())),
            "globals" => Ok(self.s.globals()),
            "do" => match self.s.run(payload) {
                Ok(ran) => {
                    let note = self.save_log()?;
                    let n = self.s.log.len();
                    let mut out = note;
                    out.push_str(&ran.out);
                    out.push_str(&format!("{}\n", time::time_of_day(self.s.st.borrow().clock)));
                    out.push_str(&format!("ok · chunk {n} ({:.2} s to compute)\n", ran.secs));
                    // the chunk is in the log: a look that fails now is reported with it, not
                    // as a failed `do` (which would be sent again)
                    if self.frames {
                        let p = session_dir(&self.name).join("frames").join(format!("{n:04}.png"));
                        if let Err(e) = self.look(&[], Some(p)) {
                            out.push_str(&format!("(no frame saved: {e})\n"));
                        }
                    }
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
            "log" => Ok(self.s.program(&self.name)),
            "save" => {
                // save [path] [--light az,el | --gallery]: lit on the paint's relief, or color only
                let (path, light) = save_args(args)?;
                let p = path.unwrap_or_else(|| session_dir(&self.name).join(format!("{}.png", self.name)));
                let c = self.s.canvas().ok_or("no canvas yet")?;
                deliver_lit(&c, &p, light)?;
                Ok(format!("{}\n", p.display()))
            }
            "frames" => {
                self.frames = matches!(args.first().map(|s| s.as_str()), Some("on") | None);
                Ok(format!("frames {} ({})\n", if self.frames { "on" } else { "off" }, session_dir(&self.name).join("frames").display()))
            }
            "close" => {
                let note = self.save_log()?;
                let live = self.save_live().unwrap_or_else(|e| format!("the live canvas couldn't be saved: {e}\n"));
                Ok(format!("{note}{live}closed; the session is in {}\n", log_path(&self.name).display()))
            }
            o => Err(format!("unknown command {o:?}")),
        }
    }
}

#[cfg(feature = "replay")]
fn bits(v: &[paint::Rgb]) -> Vec<u32> {
    v.iter().flat_map(|p| p.map(f32::to_bits)).collect()
}
#[cfg(feature = "replay")]
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
    if p.len() != 2 || !p[0].is_finite() || !(3.0..=89.0).contains(&p[1]) {
        return Err(format!("light {s}: want azimuth,elevation in degrees (elevation 3 to 89)"));
    }
    Ok((p[0], p[1]))
}

/// `save`'s arguments: an optional path, then `--light az,el` or `--gallery`.
fn save_args(args: &[String]) -> Result<(Option<PathBuf>, Option<(f32, f32)>), String> {
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
    let buf: Vec<u8> = px.iter().flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8)).collect();
    if let Some(d) = out.parent() {
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    image::save_buffer(out, &buf, f.w as u32, f.h as u32, image::ColorType::Rgb8).map_err(|e| format!("{}: {e}", out.display()))
}

#[cfg(feature = "replay")]
const RUN_USAGE: &str = "run <file.lua> [--out path.png] [--light az,el | --gallery] [--look] [--state-digest digests.txt] [--frames-every <s> --frames-dir <dir> [--frame-width 1000]] [--width <px>] (replays at the width it was painted at, 2400px, a sketch 600px, unless --width: a smaller preview for development, not the painting)";

#[cfg(feature = "replay")]
fn run(args: &[String]) -> Result<(), String> {
    let file = args.first().filter(|a| !a.starts_with('-')).ok_or(RUN_USAGE)?;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" | "--dump-surface" | "--dump-state" | "--frames-every" | "--frames-dir" | "--frame-width" | "--state-digest" | "--width" | "--light" if i + 1 < args.len() => i += 2,
            "--look" | "--gallery" => i += 1,
            o => return Err(format!("run: unknown argument {o:?} ({RUN_USAGE})")),
        }
    }
    let text = std::fs::read_to_string(file).map_err(|e| format!("{file}: {e}"))?;
    let stem = Path::new(file).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or("easel".into());
    // a development preview may replay narrower (kernel radii are in mm, so it is not the
    // painting at a smaller size: see notes/workflow.md); the painting is the width it was
    // painted at (LIVE_WIDTH, a sketch's SKETCH_WIDTH)
    let width = match flag(args, "--width") {
        None => width_for(&stem, Some(&text)),
        Some(w) => w.parse::<usize>().ok().filter(|w| (16..=LIVE_WIDTH * 4).contains(w)).ok_or_else(|| format!("--width {w}: want px, 16 to {}", LIVE_WIDTH * 4))?,
    };
    let out = flag(args, "--out").map(PathBuf::from).unwrap_or_else(|| root().join("out/lua").join(format!("{stem}.png")));
    let chunks = parse_program(&text);
    if chunks.is_empty() {
        return Err(format!("{file}: no chunks (each starts with a line \"{}\")", session::MARK));
    }
    // the box the log was painted from, whatever this easel's own box is
    let tubes = session::box_for(Some(&text)).map_err(|e| format!("{file}: {e}"))?;
    // hand-time frames (frames.rs): only read the canvas, so the replay is
    // the same with or without them
    // --light or --gallery: the picture (and its frames) lit on the paint's relief
    let light = if args.iter().any(|a| a == "--gallery") { Some(GALLERY_LIGHT) } else { flag(args, "--light").map(|l| light_of(&l)).transpose()? };
    let frames = match (flag(args, "--frames-every"), flag(args, "--frames-dir")) {
        (None, None) => false,
        (Some(e), Some(d)) => {
            let every: f64 = e.parse().map_err(|_| format!("--frames-every {e}: want seconds of hand time"))?;
            let fw = flag(args, "--frame-width").map(|w| w.parse::<u32>().map_err(|_| format!("--frame-width {w}: want px"))).transpose()?.unwrap_or(1000);
            frames::start(every, PathBuf::from(d), fw, light)?;
            true
        }
        _ => return Err(format!("run: --frames-every and --frames-dir go together ({RUN_USAGE})")),
    };
    let mut s = Session::replay_with(width, tubes).map_err(|e| e.to_string())?;
    // --state-digest: one line of state digests after every chunk (see `state_digest_line`)
    let mut digests = flag(args, "--state-digest").map(|p| std::fs::File::create(&p).map_err(|e| format!("{p}: {e}"))).transpose()?;
    let t0 = Instant::now();
    for (i, c) in chunks.iter().enumerate() {
        let r = s.run(c).map_err(|e| format!("chunk {} failed:\n{e}", i + 1))?;
        print!("{}", r.out);
        eprintln!("  chunk {:>3}  {:>7.2}s", i + 1, r.secs);
        if let Some(f) = digests.as_mut() {
            f.write_all(state_digest_line(&s, i + 1, r.secs).as_bytes()).map_err(|e| format!("--state-digest: {e}"))?;
        }
        // --dump-state <dir>: every state value after the chunk (state_dump.rs; reads only)
        if let Some(d) = flag(args, "--dump-state") {
            state_dump::write_chunk(&s, i + 1, Path::new(&d))?;
        }
        if frames && let Some(c) = s.canvas() {
            frames::chunk_end(&c, i + 1);
        }
    }
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
    Ok(())
}

#[cfg(all(feature = "replay", feature = "finish"))]
const FINISH_USAGE: &str = "finish <save file> <out.png> [--log painting.lua] [--coats C] [--no-varnish] [--no-cracks] [--relief]
       finish --print-chunk [--coats C] [--no-varnish] [--no-cracks] [--relief]";

/// `easel finish <save> <out.png> [options]`: restore a live session's save (live.ckpt,
/// save.rs) into a fresh session, run the finishing chunk scripts/finish_painting builds
/// (finish.rs `chunk`) and write the PNG, without replaying the log. With `--log`, the save
/// must be of that log (its hash and chunk count). `--print-chunk` prints the chunk only.
#[cfg(all(feature = "replay", feature = "finish"))]
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
    if !(varnish || cracks || relief) {
        return Err("nothing to do".into());
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
    let ran = s.run(&chunk).map_err(|e| format!("the finishing chunk failed:\n{e}"))?;
    print!("{}", ran.out);
    let c = s.canvas().ok_or("the save holds no canvas")?;
    deliver(&c, Path::new(out))?;
    eprintln!("wrote {out} (restored {} chunks in {restored:.1}s, finished in {:.1}s, total {:.1}s)", r.chunks, ran.secs, t0.elapsed().as_secs_f64());
    Ok(())
}

#[cfg(feature = "replay")]
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
#[cfg(feature = "replay")]
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
    #[test]
    fn a_long_session_dir_gets_a_short_socket() {
        use std::os::unix::fs::MetadataExt;
        let short = std::path::Path::new("/s/out/easel/p");
        assert_eq!(super::short_sock(short), short.join("sock"));
        // under a directory this test makes, so its nearest existing ancestor is this
        // user's (the temp dir itself may be root's, as /tmp is on Linux)
        let mine = std::env::temp_dir().join(format!("easel-long-test-{}", std::process::id()));
        std::fs::create_dir_all(&mine).unwrap();
        let long = mine.join("d".repeat(120)).join("out/easel/p");
        let s = super::short_sock(&long);
        assert!(s.as_os_str().len() < 100, "{s:?}");
        let private = std::fs::symlink_metadata(s.parent().unwrap()).unwrap();
        assert!(private.is_dir() && private.mode() & 0o077 == 0, "the socket's directory is private");
        assert_eq!(s, super::short_sock(&long), "the same directory, the same socket");
        // The private <tmp>/easel-<uid> it made stays: it is the one live servers use,
        // and removing it under a running easel would take its socket away.
        std::fs::remove_dir(&mine).unwrap();
    }
    #[test]
    fn a_shared_socket_dir_is_not_used() {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let d = std::env::temp_dir().join(format!("easel-shared-test-{}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        std::fs::set_permissions(&d, std::fs::Permissions::from_mode(0o777)).unwrap();
        let me = std::fs::metadata(&d).unwrap().uid();
        assert!(!super::private_dir(&d, me), "open to others");
        std::fs::set_permissions(&d, std::fs::Permissions::from_mode(0o700)).unwrap();
        assert!(super::private_dir(&d, me));
        assert!(!super::private_dir(&d, me.wrapping_add(1)), "another user's");
        let link = d.with_extension("link");
        let _ = std::fs::remove_file(&link);
        std::os::unix::fs::symlink(&d, &link).unwrap();
        assert!(!super::private_dir(&link, me), "a symlink");
        std::fs::remove_file(&link).unwrap();
        std::fs::remove_dir(&d).unwrap();
    }
    use super::*;

    /// A stray look numbered at the top of u64 leaves no number above it: the look is
    /// refused, not a panic (debug) or a wrap back to look-0000 (release).
    #[test]
    fn a_look_above_the_last_number_is_refused() {
        let dir = std::env::temp_dir().join(format!("easel-look-top-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(format!("look-{}.png", u64::MAX)), b"stray").unwrap();
        let r = std::panic::catch_unwind(|| new_look(&dir, b"png"));
        let names: Vec<_> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
        let _ = std::fs::remove_dir_all(&dir);
        let r = r.expect("new_look panicked");
        assert!(r.as_ref().is_err_and(|e| e.contains("look")), "{r:?}");
        assert_eq!(names.len(), 1, "a look was written: {names:?}");
    }

    /// The lock-on-stdin signal is for this server only: once read it leaves the
    /// environment, so no child the server ever starts takes it (or its stdin) as the lock.
    #[test]
    fn the_lock_on_stdin_signal_is_not_passed_on() {
        // SAFETY: no other test reads or writes this variable
        unsafe { std::env::set_var(LOCK_ON_STDIN, "1") };
        // stdin here is no lock file: refused, but the signal is read all the same
        let r = serve_lock("lock-signal-test");
        assert!(r.is_err(), "a test's stdin was taken as the lock");
        assert_eq!(std::env::var_os(LOCK_ON_STDIN), None, "{LOCK_ON_STDIN} is still set");
    }

    /// A request is served from its length line, even when the client's end of file never
    /// comes (macOS sometimes loses a half-close; the server then waited for it until the
    /// client was killed). The client here keeps its socket open, as a lost end of file looks.
    #[test]
    fn a_request_is_read_without_the_clients_end_of_file() {
        let (mut client, mut server) = UnixStream::pair().unwrap();
        server.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
        // more than a socket buffer holds: the client writes while the server reads
        let payload = "-- ünïcode\n".repeat(2000);
        let sent = payload.clone();
        let (done_tx, done_rx) = std::sync::mpsc::channel();
        let t = std::thread::spawn(move || {
            send_request(&mut client, b"do\n", sent.as_bytes()).unwrap();
            // keep the socket open until the server has read: no end of file
            let _ = done_rx.recv();
            drop(client);
        });
        let req = read_request(&mut server).expect("the server waited for an end of file");
        done_tx.send(()).unwrap();
        t.join().unwrap();
        assert_eq!(req, [b"do\n".as_slice(), payload.as_bytes()].concat());
    }

    #[test]
    fn a_request_from_an_older_client_is_refused_with_a_reason() {
        let (mut client, mut server) = UnixStream::pair().unwrap();
        client.write_all(b"status\n").unwrap();
        let e = read_request(&mut server).unwrap_err();
        assert!(e.contains("different versions"), "{e}");
    }

    const PALETTE_CANVAS: &str = r#"canvas{size=300, aspect=1.25, seed=3, linen=15, ground={{pile={{"lead white", 4}, {"red earth", 1}}, um=80, apply="knife"}}}"#;

    /// The held knife (speculative) only reads, as the palette look: the state digest, the
    /// log, the globals and the clock are as they were. It shows the passage with a knife
    /// over it at the point: a knife global with its own load, or a pile freshly loaded.
    #[cfg(feature = "replay")]
    #[test]
    fn a_held_knife_shows_its_paint_over_the_passage_and_changes_nothing() {
        let mut s = Session::new(1000).unwrap();
        s.run(PALETTE_CANVAS).unwrap();
        s.run(r#"skyP = pile{{"lead white", 6}, {"smalt", 1}, medium=0.2}; dk = pile{{"raw umber", 2}, {"bone black", 1}}
                 b = brush("filbert", 8); b:load(skyP, 0.8); b:stroke({{100, 300}, {700, 340}})
                 k = knife{width=40}; k:load(dk, 0.8); clean = knife{width=20}"#).unwrap();
        let before = (state_digest_line(&s, 2, 0.0), s.program("t"), s.globals(), s.st.borrow().clock);
        let plain = look::View::default();
        let at = (400.0, 320.0);
        let rgb = |png: &[u8]| image::load_from_memory(png).unwrap().to_rgb8();
        let (w, h, png) = hold_look(&s, "k", at, &plain).unwrap();
        // the passage is 240 units square: at this canvas's one px a unit, 240 px (give or take a rounded edge)
        assert!((239..=241).contains(&w) && (239..=241).contains(&h), "{w} x {h}");
        let side = w.min(h);
        let held = rgb(&png);
        // the same passage without the knife: the same left of the point, another picture right of it
        let bare = rgb(&look::render(&s.canvas().unwrap(), &look::View { crop: Some([at.0 - HOLD_HALF, at.1 - HOLD_HALF, at.0 + HOLD_HALF, at.1 + HOLD_HALF]), ..look::View::default() }).unwrap().2);
        let mid = (side / 2) as u32;
        assert!((0..mid - 1).all(|x| (0..side as u32).all(|y| held.get_pixel(x, y) == bare.get_pixel(x, y))), "the passage left of the point is untouched");
        let covered = (mid..side as u32).flat_map(|x| (0..side as u32).map(move |y| (x, y))).filter(|&(x, y)| held.get_pixel(x, y) != bare.get_pixel(x, y)).count();
        assert!(covered > 100, "the blade covers {covered} pixels");
        // the dark paint is on the blade: some of it much darker than the steel
        let darkest = (mid..side as u32).map(|x| held.get_pixel(x, mid).0.iter().map(|&v| v as u32).sum::<u32>()).min().unwrap();
        assert!(darkest < 200, "the darkest of the blade's middle row sums to {darkest}");
        // a pile by its name is a fresh load; in grays the blade is gray too
        hold_look(&s, "skyP", at, &plain).unwrap();
        // The render clamps an out-of-bounds crop; blade placement uses the
        // same origin, so it matches the explicitly clamped crop exactly.
        let edge = (100.0, 100.0);
        let outside = hold_look(&s, "k", edge, &look::View { crop: Some([-50.0, -50.0, 200.0, 200.0]), ..look::View::default() }).unwrap();
        let clamped = hold_look(&s, "k", edge, &look::View { crop: Some([0.0, 0.0, 200.0, 200.0]), ..look::View::default() }).unwrap();
        assert!(outside == clamped, "crop clipping cannot move the held blade");
        let gray = rgb(&hold_look(&s, "k", at, &look::View { value: true, ..look::View::default() }).unwrap().2);
        assert!((mid..side as u32).all(|x| { let p = gray.get_pixel(x, mid).0; p[0] == p[1] && p[1] == p[2] }));
        // (kept for the eye: target/easel-look-test/held-knife.png)
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/easel-look-test");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("held-knife.png"), &png).unwrap();
        let after = (state_digest_line(&s, 2, 0.0), s.program("t"), s.globals(), s.st.borrow().clock);
        assert_eq!(before, after);
        s.run("assert(k:fullness() > 0)").unwrap();
        s.run("k:wipe(); k:load(dk, 0.01)").unwrap();
        assert!((s.held("k").unwrap().2 - 0.01).abs() < 1e-5,
            "the preview must preserve a nearly empty knife's load");
        s.run(r#"skyP = pile{{"lead white", 1}, thinner=0.4}
            skyP:add{{"lead white", 1}}
            k:wipe(); k:load(skyP, 0.5)"#).unwrap();
        assert!((s.held("skyP").unwrap().0.thinner - 0.2).abs() < 1e-5,
            "a palette preview must use the current diluted thinner share");
        assert!((s.held("k").unwrap().0.thinner - 0.2).abs() < 1e-5,
            "a knife preview must retain the blade's carried solvent share");
        // what it refuses: a clean knife, a name that holds neither, a point off the canvas or the crop, a view of its own
        assert!(hold_look(&s, "clean", at, &plain).unwrap_err().contains("the knife is clean"));
        let e = hold_look(&s, "nope", at, &plain).unwrap_err();
        assert!(e.contains("no global of that name holds a knife or a pile") && e.contains("clean, dk, k, skyP"), "{e}");
        assert!(hold_look(&s, "k", (1400.0, 320.0), &plain).unwrap_err().contains("not on the canvas"));
        assert!(hold_look(&s, "k", (f32::NAN, 320.0), &plain).unwrap_err().contains("not on the canvas"));
        assert!(hold_look(&s, "k", at, &look::View { crop: Some([0.0, 0.0, 100.0, 100.0]), ..look::View::default() }).unwrap_err().contains("outside the --crop"));
        assert!(hold_look(&s, "k", at, &look::View { mirror: true, ..look::View::default() }).is_err());
    }

    /// A palette look only reads: the state digest (canvas, brushes, studio), the log and the
    /// globals are the same before and after it, and it puts no time on the clock.
    #[test]
    #[cfg(tube_box)]
    fn a_palette_look_changes_nothing() {
        let mut s = Session::new(320).unwrap();
        s.run(PALETTE_CANVAS).unwrap();
        s.run(r#"skyP = pile{{"lead white", 6}, {"smalt", 1}, medium=0.2}; dk = pile{{"raw umber", 2}, {"bone black", 1}}
                 b = brush("filbert", 8); b:load(skyP, 0.8); b:stroke({{100, 300}, {700, 340}})"#).unwrap();
        let before = (state_digest_line(&s, 2, 0.0), s.program("t"), s.globals(), s.st.borrow().clock);
        let (w, h, png) = palette_look(&s).unwrap();
        assert!(w > 0 && h > 0 && png.starts_with(b"\x89PNG"));
        assert_eq!(s.piles().iter().map(|p| p.0.as_str()).collect::<Vec<_>>(), ["dk", "skyP"]);
        let after = (state_digest_line(&s, 2, 0.0), s.program("t"), s.globals(), s.st.borrow().clock);
        assert_eq!(before, after);
        // and the next chunk runs as it would have without the look
        let mut t = Session::new(320).unwrap();
        for c in &s.log {
            t.run(&c.src).unwrap();
        }
        s.run("b:stroke({{100, 500}, {700, 520}})").unwrap();
        t.run("b:stroke({{100, 500}, {700, 520}})").unwrap();
        assert_eq!(state_digest_line(&s, 3, 0.0), state_digest_line(&t, 3, 0.0));
    }

    /// The palette's thick swatch is what `work` lays thick with that pile: within 2/255 of
    /// the wet paint's mean in the middle of a heavily covered patch, for a few piles (one
    /// test each, so they run side by side: together they took 11 s).
    #[test]
    #[cfg(tube_box)]
    fn the_thick_swatch_matches_paint_laid_thick() {
        thick_swatch_matches(r#"{"lead white", 6}, {"smalt", 1}, medium=0.2"#);
    }

    #[test]
    #[cfg(tube_box)]
    fn the_thick_swatch_matches_paint_laid_thick_dark() {
        thick_swatch_matches(r#"{"raw umber", 2}, {"bone black", 1}"#);
    }

    #[test]
    #[cfg(tube_box)]
    fn the_thick_swatch_matches_paint_laid_thick_earths() {
        thick_swatch_matches(r#"{"yellow ochre", 3}, {"red earth", 1}, {"lead white", 2}"#);
    }

    #[cfg(tube_box)]
    fn thick_swatch_matches(recipe: &str) {
        let srgb = |c: paint::Rgb| c.map(|v| linear_to_srgb(v) * 255.0);
        {
            let mut s = Session::new(320).unwrap();
            s.run(PALETTE_CANVAS).unwrap();
            s.run(&format!(r#"p = pile{{{recipe}}}; work(rect(200, 200, 800, 600), {{hand="body", pile=p, coverage=6}})"#)).unwrap();
            let (_, _, paint) = s.piles().into_iter().find(|p| p.0 == "p").unwrap();
            let ground = s.ground_color().unwrap();
            let want = srgb(look::swatches(&paint, ground)[0]);
            let c = s.canvas().unwrap();
            let f = c.window();
            let seen = c.seen();
            let mut acc = [0.0f64; 3];
            let mut n = 0.0;
            for y in (f.h * 3 / 8)..(f.h * 5 / 8) {
                for x in (f.w * 3 / 8)..(f.w * 5 / 8) {
                    for q in 0..3 {
                        acc[q] += seen[y * f.w + x][q] as f64;
                    }
                    n += 1.0;
                }
            }
            let got = srgb(acc.map(|v| (v / n) as f32));
            for q in 0..3 {
                assert!((got[q] - want[q]).abs() <= 2.0, "{recipe}: laid {got:?}, swatch {want:?}");
            }
        }
    }

    #[test]
    fn journal_entries_are_dated_lines() {
        assert_eq!(journal_entry(907.5, "first line\nsecond\n\nthird\n"), "- day 2, 00:07: first line\n  second\n\n  third\n");
    }

    #[test]
    fn save_takes_a_path_and_a_light() {
        let args = |a: &[&str]| save_args(&a.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        assert_eq!(args(&[]), Ok((None, None)));
        assert_eq!(args(&["a.png"]), Ok((Some(PathBuf::from("a.png")), None)));
        assert_eq!(args(&["a.png", "--gallery"]), Ok((Some(PathBuf::from("a.png")), Some(GALLERY_LIGHT))));
        assert_eq!(args(&["--light", "135,25", "a.png"]), Ok((Some(PathBuf::from("a.png")), Some((135.0, 25.0)))));
        for bad in [&["--light"][..], &["--light", "135"], &["--light", "135,91"], &["--light", "nan,25"], &["--light", "inf,25"], &["a.png", "b.png"], &["--lit"]] {
            assert!(args(bad).is_err(), "{bad:?}");
        }
    }

    /// A new session is a sketch by its name; a log by what its head says.
    #[test]
    fn a_log_replays_at_the_width_it_was_painted_at() {
        assert_eq!(width_for("sketch-1", None), session::SKETCH_WIDTH);
        assert_eq!(width_for("painting", None), LIVE_WIDTH);
        let head = |mark: &str| format!("-- easel session\n--@ engine 4\n{mark}\n--@ chunk 1\ncanvas{{}}\n");
        assert_eq!(width_for("renamed", Some(&head(session::SKETCH_MARK))), session::SKETCH_WIDTH);
        assert_eq!(width_for("sketchbook", Some(&head(""))), LIVE_WIDTH);
    }
}
