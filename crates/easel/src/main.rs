//! easel: a live Lua painting session over the claude-paint engine.
//!
//!   easel open <name>                start (or reattach to) a session
//!   easel do '<lua>' | -f chunk.lua | -            run a chunk on the live canvas
//!   easel look [--crop x0,y0,x1,y1] [--mode value|squint|mirror|wet] [--grid [step]] [--size N]
//!   easel log | status | save [path] | frames on|off | check | close
//!   easel note '<text>' | -                        append to notes/journal.md
//!   easel run paintings/lua/<name>.lua [--width 1000] [--out path] [--crop ...]
//!
//! See notes/easel_guide.md.

mod api;
mod depth;
mod draw_edges;
mod draw_outline;
mod form;
mod look;
mod session;
mod time;
mod world;

use session::{Session, parse_program, root};
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, Instant};

const USAGE: &str = "easel: a live painting session (see notes/easel_guide.md)

  easel open <name>    start or reattach; replays paintings/lua/<name>.lua if it exists
  easel do '<lua>'  |  easel do -f chunk.lua  |  easel do - (stdin)     [--look] also looks afterwards
  easel look [--crop x0,y0,x1,y1] [--mode value,squint,mirror,wet] [--grid [step]] [--size 1000]
  easel log           the session so far (= paintings/lua/<name>.lua)
  easel status        chunks, width, canvas
  easel save [path]   the canvas as a PNG (default out/easel/<name>/<name>.png)
  easel frames on|off save a look after every chunk
  easel check         replay the log from scratch and compare with the live canvas
  easel close         end the session (the log stays)
  easel note '<text>' | easel note - (stdin)    append a dated entry to notes/journal.md
  easel run <file.lua> [--width 1000] [--out path.png] [--crop x0,y0,x1,y1] [--margin 40] [--look]

  -s <name> (or EASEL_SESSION) picks the session; default: the last opened.";

fn main() -> ExitCode {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let mut name: Option<String> = std::env::var("EASEL_SESSION").ok();
    if let Some(i) = args.iter().position(|a| a == "-s" || a == "--session")
        && i + 1 < args.len()
    {
        name = Some(args.remove(i + 1));
        args.remove(i);
    }
    let Some(cmd) = args.first().cloned() else {
        println!("{USAGE}");
        return ExitCode::SUCCESS;
    };
    let rest = args[1..].to_vec();
    let r = match cmd.as_str() {
        "open" => open(&rest),
        "serve" => serve(&rest),
        "run" => run(&rest),
        "note" => note(&rest, name),
        "hash-probe" => {
            println!("{}", session::hash_probe());
            Ok(())
        }
        "help" | "-h" | "--help" => {
            println!("{USAGE}");
            Ok(())
        }
        "do" | "look" | "log" | "status" | "save" | "frames" | "check" | "close" => client(&cmd, &rest, name),
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

fn flag(args: &[String], f: &str) -> Option<String> {
    args.iter().position(|a| a == f).and_then(|i| args.get(i + 1).cloned())
}

fn valid_name(n: &str) -> Result<(), String> {
    if n.is_empty() || !n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        return Err(format!("session name {n:?}: letters, digits, _ and - only"));
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

// ---------------------------------------------------------------- client

fn request(name: &str, cmd: &str, args: &[String], payload: &[u8]) -> Result<(bool, String), String> {
    let mut s = UnixStream::connect(sock_path(name)).map_err(|_| format!("no easel session {name:?} running: easel open {name}"))?;
    let mut head = cmd.to_string();
    for a in args {
        head.push('\t');
        head.push_str(&a.replace(['\t', '\n'], " "));
    }
    head.push('\n');
    s.write_all(head.as_bytes()).and_then(|_| s.write_all(payload)).map_err(|e| e.to_string())?;
    s.shutdown(std::net::Shutdown::Write).map_err(|e| e.to_string())?;
    let mut resp = String::new();
    s.read_to_string(&mut resp).map_err(|e| e.to_string())?;
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

fn open(args: &[String]) -> Result<(), String> {
    let name = args.first().filter(|a| !a.starts_with('-')).ok_or("open <name>")?.clone();
    valid_name(&name)?;
    if args.len() != 1 {
        return Err("open: live sessions are fixed at 2400px; no width option".into());
    }
    let dir = session_dir(&name);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    if let Ok((ok, st)) = request(&name, "status", &[], &[]) {
        if !ok {
            return Err(st);
        }
        std::fs::write(root().join("out/easel/current"), &name).map_err(|e| e.to_string())?;
        print!("reattached to {name:?}: {st}");
        return Ok(());
    }
    let _ = std::fs::remove_file(sock_path(&name));
    let log = std::fs::File::create(dir.join("server.log")).map_err(|e| e.to_string())?;
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    use std::os::unix::process::CommandExt;
    std::process::Command::new(exe)
        .args(["serve", &name])
        .stdin(std::process::Stdio::null())
        .stdout(log.try_clone().map_err(|e| e.to_string())?)
        .stderr(log)
        .process_group(0)
        .spawn()
        .map_err(|e| format!("could not start the session: {e}"))?;
    let t0 = Instant::now();
    loop {
        std::thread::sleep(Duration::from_millis(100));
        if let Ok((true, st)) = request(&name, "status", &[], &[]) {
            let resumed = std::fs::read_to_string(dir.join("server.log")).unwrap_or_default();
            for l in resumed.lines().filter(|l| l.starts_with("resumed") || l.starts_with("warning")) {
                println!("{l}");
            }
            std::fs::write(root().join("out/easel/current"), &name).map_err(|e| e.to_string())?;
            print!("easel {name:?} open: {st}");
            return Ok(());
        }
        let log = std::fs::read_to_string(dir.join("server.log")).unwrap_or_default();
        if log.contains("easel: fatal") {
            return Err(log);
        }
        if t0.elapsed() > Duration::from_secs(1800) {
            return Err(format!("session did not come up; see {}", dir.join("server.log").display()));
        }
    }
}

// ---------------------------------------------------------------- server

struct Server {
    name: String,
    s: Session,
    frames: bool,
    /// The log text as the easel last wrote (or read) it: if the file on
    /// disk differs, it was changed outside the session.
    written: Option<String>,
}

fn serve(args: &[String]) -> Result<(), String> {
    let name = args.first().ok_or("serve <name>")?.clone();
    valid_name(&name)?;
    if args.len() != 1 {
        return Err("easel: fatal: live sessions are fixed at 2400px".into());
    }
    let mut srv = Server::resume(name.clone()).map_err(|e| format!("easel: fatal: {e}"))?;
    let sock = sock_path(&name);
    let l = UnixListener::bind(&sock).map_err(|e| format!("easel: fatal: bind {}: {e}", sock.display()))?;
    let _ = std::io::stdout().flush();

    for conn in l.incoming() {
        let Ok(mut conn) = conn else { continue };
        let mut req = Vec::new();
        if conn.read_to_end(&mut req).is_err() {
            continue;
        }
        let text = String::from_utf8_lossy(&req).to_string();
        let (head, payload) = text.split_once('\n').unwrap_or((&text, ""));
        let mut parts = head.split('\t').map(|s| s.to_string());
        let cmd = parts.next().unwrap_or_default();
        let args: Vec<String> = parts.collect();
        let t0 = Instant::now();
        let r = srv.handle(&cmd, &args, payload);
        let reply = match &r {
            Ok(b) => format!("ok\n{b}"),
            Err(e) => format!("err\n{e}\n"),
        };
        let _ = conn.write_all(reply.as_bytes());
        eprintln!("{cmd} {:.2}s {}", t0.elapsed().as_secs_f64(), if r.is_ok() { "ok" } else { "err" });
        if cmd == "close" && r.is_ok() {
            let _ = std::fs::remove_file(&sock);
            break;
        }
    }
    Ok(())
}

impl Server {
    // Independent local witness, not a signature or an access-control boundary.
    // Editing the log alone is detected; coordinated edits of both files or
    // restoring the entire directory are outside this local integrity model.
    fn witness(&self) -> PathBuf {
        session_dir(&self.name).join("committed.lua")
    }

    fn validate(&self) -> Result<(), String> {
        let expected = self.written.as_deref().ok_or("session integrity: uninitialized log")?;
        for p in [log_path(&self.name), self.witness()] {
            let actual = std::fs::read(&p).map_err(|e| format!("session integrity: {}: {e}", p.display()))?;
            if actual != expected.as_bytes() {
                return Err(format!("session integrity: {} differs from the committed log; refusing request", p.display()));
            }
        }
        Ok(())
    }

    fn resume(name: String) -> Result<Self, String> {
        let mut srv = Self { name, s: Session::new(2400).map_err(|e| e.to_string())?, frames: false, written: None };
        let lp = log_path(&srv.name);
        if lp.exists() || srv.witness().exists() {
            let text = std::fs::read_to_string(&lp).map_err(|e| format!("session integrity: {e}"))?;
            srv.written = Some(text.clone());
            srv.validate()?;
            for (i, chunk) in parse_program(&text).iter().enumerate() {
                srv.s.run(chunk).map_err(|e| format!("session integrity: replay failed at chunk {}: {e}; refusing partial session", i + 1))?;
            }
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

    fn look(&mut self, args: &[String], path: Option<PathBuf>) -> Result<String, String> {
        let v = look::View::parse(args)?;
        let t0 = Instant::now();
        let c = self.s.canvas().ok_or("no canvas yet: the first chunk is canvas{...}")?;
        let dir = session_dir(&self.name);
        let path = path.unwrap_or_else(|| {
            let n = std::fs::read_dir(&dir).map(|d| d.filter_map(|e| e.ok()).filter(|e| e.file_name().to_string_lossy().starts_with("look-")).count()).unwrap_or(0);
            dir.join(format!("look-{:04}.png", n + 1))
        });
        let (w, h) = look::look(&c, &v, &path)?;
        Ok(format!("{} ({w}x{h}, {:.2}s)\n", path.display(), t0.elapsed().as_secs_f64()))
    }

    fn handle(&mut self, cmd: &str, args: &[String], payload: &str) -> Result<String, String> {
        self.validate()?;
        if self.written.as_deref() != Some(self.s.program(&self.name).as_str()) {
            return Err("session integrity: uncommitted state; restart required".into());
        }
        match cmd {
            "note" => append_note(self.s.st.borrow().clock, payload),
            "status" => Ok(format!("{}\n", self.s.status())),
            "do" => match self.s.run(payload) {
                Ok(ran) => {
                    let note = self.save_log()?;
                    let n = self.s.log.len();
                    let mut out = note;
                    out.push_str(&ran.out);
                    out.push_str(&format!("ok · chunk {n} ({:.2} s to compute)\n", ran.secs));
                    if self.frames {
                        let p = session_dir(&self.name).join("frames").join(format!("{n:04}.png"));
                        self.look(&[], Some(p))?;
                    }
                    if args.iter().any(|a| a == "--look") {
                        out.push_str(&self.look(&[], None)?);
                    }
                    Ok(out)
                }
                Err(e) => Err(format!("{e}\n(the chunk failed and changed nothing)")),
            },
            "look" => self.look(args, None),
            "log" => Ok(self.s.program(&self.name)),
            "save" => {
                let p = args.first().map(PathBuf::from).unwrap_or_else(|| session_dir(&self.name).join(format!("{}.png", self.name)));
                let mut c = self.s.canvas().ok_or("no canvas yet")?.clone();
                c.save(&p).map_err(|e| e.to_string())?;
                Ok(format!("{}\n", p.display()))
            }
            "frames" => {
                self.frames = matches!(args.first().map(|s| s.as_str()), Some("on") | None);
                Ok(format!("frames {} ({})\n", if self.frames { "on" } else { "off" }, session_dir(&self.name).join("frames").display()))
            }
            "check" => {
                let t0 = Instant::now();
                let width = self.s.st.borrow().width;
                let mut fresh = Session::replay(width).map_err(|e| e.to_string())?;
                for (i, c) in self.s.log.iter().enumerate() {
                    fresh.run(&c.src).map_err(|e| format!("replay failed at chunk {}: {e}", i + 1))?;
                }
                let same = match (self.s.canvas(), fresh.canvas()) {
                    (Some(a), Some(b)) => bits(&a.seen()) == bits(&b.seen()) && bits_f(a.surface_um()) == bits_f(b.surface_um()),
                    (None, None) => true,
                    _ => false,
                };
                if same {
                    Ok(format!("replay matches the live canvas exactly ({} chunks, {:.1}s)\n", self.s.log.len(), t0.elapsed().as_secs_f64()))
                } else {
                    Err("replay DIFFERS from the live canvas: a chunk depended on state from a failed chunk (e.g. a table it changed); the log is the painting: close and reopen to continue from it"
                        .into())
                }
            }
            "close" => {
                let note = self.save_log()?;
                Ok(format!("{note}closed; the session is in {}\n", log_path(&self.name).display()))
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

fn run(args: &[String]) -> Result<(), String> {
    let file = args.first().filter(|a| !a.starts_with('-')).ok_or("run <file.lua> [--width N] [--out path.png] [--crop x0,y0,x1,y1] [--margin 40] [--look]")?;
    let width: usize = flag(args, "--width").map(|w| w.parse().map_err(|_| "--width N")).transpose()?.unwrap_or(1000);
    let text = std::fs::read_to_string(file).map_err(|e| format!("{file}: {e}"))?;
    let stem = Path::new(file).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or("easel".into());
    if let Some(c) = flag(args, "--crop") {
        let p: Vec<f32> = c.split(',').map(|t| t.trim().parse::<f32>()).collect::<Result<_, _>>().map_err(|_| "--crop x0,y0,x1,y1 (units)")?;
        if p.len() != 4 {
            return Err("--crop x0,y0,x1,y1 (units)".into());
        }
        let margin: f32 = flag(args, "--margin").and_then(|m| m.parse().ok()).unwrap_or(40.0);
        paint::set_crop(Some(paint::Crop { units: [p[0], p[1], p[2], p[3]], margin }));
    }
    let out = flag(args, "--out").map(PathBuf::from).unwrap_or_else(|| {
        let crop = if flag(args, "--crop").is_some() { "_crop" } else { "" };
        root().join("out/lua").join(format!("{stem}_{width}{crop}.png"))
    });
    let chunks = parse_program(&text);
    if chunks.is_empty() {
        return Err(format!("{file}: no chunks (each starts with a line \"{}\")", session::MARK));
    }
    let mut s = Session::replay(width).map_err(|e| e.to_string())?;
    let t0 = Instant::now();
    for (i, c) in chunks.iter().enumerate() {
        let r = s.run(c).map_err(|e| format!("chunk {} failed:\n{e}", i + 1))?;
        print!("{}", r.out);
        eprintln!("  chunk {:>3}  {:>7.2}s", i + 1, r.secs);
    }
    let paint_secs = t0.elapsed().as_secs_f64();
    let mut c = s.canvas().ok_or("the program never made a canvas")?.clone();
    c.save(&out).map_err(|e| e.to_string())?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn journal_entries_are_dated_lines() {
        assert_eq!(journal_entry(907.5, "first line\nsecond\n\nthird\n"), "- day 2, 00:07: first line\n  second\n\n  third\n");
    }
}

#[cfg(test)]
mod integrity_regressions {
    use super::*;

    // Real request boundary: external edits must be rejected before Lua or
    // any other request can mutate state. Existing replay tests only use RAM.
    #[test]
    fn edited_or_missing_logs_block_every_request_before_mutation() {
        let name = format!("integrity-{}", std::process::id());
        let mut srv = Server { name: name.clone(), s: Session::new(80).unwrap(), frames: false, written: None };
        srv.save_log().unwrap();
        srv.handle("do", &[], "x = 1").unwrap();
        let original = std::fs::read(log_path(&name)).unwrap();
        for replacement in [Some(b"-- edited\n".as_slice()), Some(b"".as_slice()), None] {
            if let Some(bytes) = replacement {
                std::fs::write(log_path(&name), bytes).unwrap();
            } else {
                std::fs::remove_file(log_path(&name)).unwrap();
            }
            for cmd in ["do", "status", "log", "frames", "note", "look", "save", "check", "close"] {
                let e = srv.handle(cmd, &[], "x = 2").expect_err(cmd);
                assert!(e.contains("integrity"), "{cmd}: {e}");
                assert_eq!(srv.s.lua.globals().get::<i64>("x").unwrap(), 1);
                assert_eq!(srv.s.log.len(), 1);
            }
            std::fs::write(log_path(&name), &original).unwrap();
        }
        assert!(srv.handle("status", &[], "").is_ok());
    }
}
