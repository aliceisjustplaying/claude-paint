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
mod session;
mod time;
mod world;

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

/// The painter build's one session.
#[cfg(not(feature = "replay"))]
const PAINTING: &str = "painting";

#[cfg(not(feature = "replay"))]
const USAGE: &str = "easel: a live painting session (see notes/easel_guide.md)

  easel open          start or reattach; replays paintings/lua/painting.lua if it exists
  easel do '<lua>'  |  easel do -f chunk.lua  |  easel do - (stdin)     [--look] also looks afterwards
  easel look [--crop x0,y0,x1,y1] [--mode value,squint,mirror] [--grid [step]] [--size 1000]
  easel log           the painting so far (= paintings/lua/painting.lua)
  easel status        chunks, width, canvas
  easel globals       the painting's globals, one a line: chunk that last set it, name, what it holds
  easel save [path]   the canvas as a PNG (default out/easel/painting/painting.png)
  easel frames on|off save a look after every chunk
  easel close         end the session (the log stays)
  easel note '<text>' | easel note - (stdin)    append a dated entry to notes/journal.md
  easel tubes [--markdown]   the tubes in the box (--markdown: as a table)";

#[cfg(feature = "replay")]
const USAGE: &str = "easel: a live painting session (see notes/easel_guide.md)

  easel open <name>    start or reattach; replays paintings/lua/<name>.lua if it exists
  easel do '<lua>'  |  easel do -f chunk.lua  |  easel do - (stdin)     [--look] also looks afterwards
  easel look [--crop x0,y0,x1,y1] [--mode value,squint,mirror] [--grid [step]] [--size 1000]
  easel log           the session so far (= paintings/lua/<name>.lua)
  easel status        chunks, width, canvas
  easel globals       the painting's globals, one a line: chunk that last set it, name, what it holds
  easel save [path]   the canvas as a PNG (default out/easel/<name>/<name>.png)
  easel frames on|off save a look after every chunk
  easel check         replay the log from scratch and compare with the live canvas
  easel close         end the session (the log stays)
  easel note '<text>' | easel note - (stdin)    append a dated entry to notes/journal.md
  easel run <file.lua> [--out path.png] [--look] [--state-digest digests.txt]    replay at 2400px and write the PNG
      [--frames-every <s> --frames-dir <dir> [--frame-width 1000]]   and a frame per <s> of hand time
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
    session_dir(name).join("sock")
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
        let mut srv = Self { name, s: Session::with_box(LIVE_WIDTH, tubes).map_err(|e| e.to_string())?, frames: false, written: None, replayed: 0 };
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

    /// The live canvas as a PNG, `save`'s (`live.png`, next to the committed log), and in
    /// `live.txt` how many chunks it holds and how many of them were replayed: the runner's
    /// check (scripts/check_painting) compares a replay of the log with it.
    fn save_live(&self) -> Result<String, String> {
        let dir = session_dir(&self.name);
        let (png, txt) = (dir.join("live.png"), dir.join("live.txt"));
        let _ = std::fs::remove_file(&txt);
        let Some(c) = self.s.canvas() else {
            let _ = std::fs::remove_file(&png);
            return Ok(String::new());
        };
        deliver(&c, &png)?;
        std::fs::write(&txt, format!("chunks {}\nreplayed {}\n", self.s.log.len(), self.replayed)).map_err(|e| format!("{}: {e}", txt.display()))?;
        Ok(format!("the live canvas is in {}\n", png.display()))
    }

    fn look(&mut self, args: &[String], path: Option<PathBuf>) -> Result<String, String> {
        let v = look::View::parse(args)?;
        let t0 = Instant::now();
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
                    "{e}\n(the chunk failed and changed nothing. It had changed tables from earlier chunks, and though what they hold is back, how they are laid out (which decides the order `pairs` walks them in) can't be put back, so the easel now rebuilds the painting from its log, as a reopen does; status reports progress and other commands must retry after that)"
                )),
                Err(e) => Err(format!("{e}\n(the chunk failed and changed nothing)")),
            },
            "look" => self.look(args, None),
            "log" => Ok(self.s.program(&self.name)),
            "save" => {
                let p = args.first().map(PathBuf::from).unwrap_or_else(|| session_dir(&self.name).join(format!("{}.png", self.name)));
                let c = self.s.canvas().ok_or("no canvas yet")?;
                deliver(&c, &p)?;
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

/// The delivered PNG: the canvas as it is seen now (wet paint as laid, no
/// drying), 8-bit sRGB. `save` and `run` both write it.
fn deliver(c: &Canvas, out: &Path) -> Result<(), String> {
    let f = c.window();
    let buf: Vec<u8> = c.seen().iter().flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8)).collect();
    if let Some(d) = out.parent() {
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    image::save_buffer(out, &buf, f.w as u32, f.h as u32, image::ColorType::Rgb8).map_err(|e| format!("{}: {e}", out.display()))
}

#[cfg(feature = "replay")]
const RUN_USAGE: &str = "run <file.lua> [--out path.png] [--look] [--state-digest digests.txt] [--frames-every <s> --frames-dir <dir> [--frame-width 1000]] (replays at the live width, 2400px)";

#[cfg(feature = "replay")]
fn run(args: &[String]) -> Result<(), String> {
    let file = args.first().filter(|a| !a.starts_with('-')).ok_or(RUN_USAGE)?;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" | "--dump-surface" | "--frames-every" | "--frames-dir" | "--frame-width" | "--state-digest" if i + 1 < args.len() => i += 2,
            "--look" => i += 1,
            o => return Err(format!("run: unknown argument {o:?} ({RUN_USAGE})")),
        }
    }
    let width = LIVE_WIDTH;
    let text = std::fs::read_to_string(file).map_err(|e| format!("{file}: {e}"))?;
    let stem = Path::new(file).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or("easel".into());
    let out = flag(args, "--out").map(PathBuf::from).unwrap_or_else(|| root().join("out/lua").join(format!("{stem}.png")));
    let chunks = parse_program(&text);
    if chunks.is_empty() {
        return Err(format!("{file}: no chunks (each starts with a line \"{}\")", session::MARK));
    }
    // the box the log was painted from, whatever this easel's own box is
    let tubes = session::box_for(Some(&text)).map_err(|e| format!("{file}: {e}"))?;
    // hand-time frames (frames.rs): only read the canvas, so the replay is
    // the same with or without them
    let frames = match (flag(args, "--frames-every"), flag(args, "--frames-dir")) {
        (None, None) => false,
        (Some(e), Some(d)) => {
            let every: f64 = e.parse().map_err(|_| format!("--frames-every {e}: want seconds of hand time"))?;
            let fw = flag(args, "--frame-width").map(|w| w.parse::<u32>().map_err(|_| format!("--frame-width {w}: want px"))).transpose()?.unwrap_or(1000);
            frames::start(every, PathBuf::from(d), fw)?;
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
        if frames && let Some(c) = s.canvas() {
            frames::chunk_end(&c, i + 1);
        }
    }
    let paint_secs = t0.elapsed().as_secs_f64();
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
    deliver(&c, &out)?;
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

/// FNV-1a, 64 bit.
#[cfg(feature = "replay")]
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

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
///   bend). `nbrushes` counts them (a brush Lua has dropped counts until it
///   is collected).
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
    let brushes_h = fnv1a(brushes.join("\n").as_bytes());
    let studio = format!("seed={} clock={:?} clock0={:?} chunk={} calls={} setup={:?} piles={:?} rng={:?}", st.seed, st.clock, st.clock0, st.chunk, st.calls, st.setup, st.hand.piles, st.rng);
    let studio_h = fnv1a(studio.as_bytes());
    format!("chunk {n} secs={secs:.3} canvas={canvas:016x} brushes={brushes_h:016x} nbrushes={} studio={studio_h:016x}\n", brushes.len())
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn journal_entries_are_dated_lines() {
        assert_eq!(journal_entry(907.5, "first line\nsecond\n\nthird\n"), "- day 2, 00:07: first line\n  second\n\n  third\n");
    }
}
