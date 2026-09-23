//! easel: a live Lua painting session over the claude-paint engine.
//!
//!   easel open <name> [--width 1000] [--undo 8]   start (or reattach to) a session
//!   easel do '<lua>' | -f chunk.lua | -            run a chunk on the live canvas
//!   easel look [--crop x0,y0,x1,y1] [--mode value|squint|mirror] [--dried] [--relief]
//!              [--grid [step]] [--probe x,y;...] [--show on|off|clear] [--scale 3.2]
//!   easel try '<lua>'                              run a chunk, keep its show()s, roll it back
//!   easel undo [n] | log | status | save [path] | frames on|off | check | close
//!   easel run paintings/lua/<name>.lua [--width 3200] [--out path] [--crop ...]
//!
//! See crates/easel/README.md.

mod api;
mod crop;
mod form;
mod world;
mod depth;
mod edit;
mod draw_outline;
mod draw_firs;
mod draw_trees;
mod look;
mod session;

use session::{Session, parse_program, root};
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, Instant};

const USAGE: &str = "easel: a live painting session (see crates/easel/README.md)

  easel open <name> [--width 1000] [--undo 8] [--checkpoints 6]   start or reattach; replays paintings/lua/<name>.lua if it exists
  easel do '<lua>'  |  easel do -f chunk.lua  |  easel do - (stdin)     [--look] also looks afterwards
  easel look [--crop x0,y0,x1,y1] [--mode value,squint,mirror] [--dried] [--relief] [--size 1000]
             [--grid [step]] [--probe x,y;x,y] [--show on|off|clear] [--scale 3.2 [--wait 90]]
  easel try '<lua>' | -f file | -  run a chunk to see its show()/probe()/print, then roll it back (not logged) [--look]
  easel undo [n]      take back the last n chunks (default 1); their code is kept (easel undone)
  easel show N        print chunk N's code
  easel edit N '<lua>' | -f chunk.lua | -    replace chunk N and replay from it (from the nearest checkpoint)
        [--insert] put it before chunk N instead   [--drop] remove chunk N   [--undone K] use undone chunk K's code   [--look]
  easel undone [K]    list the chunks undone or replaced (or print K's code)
  easel redo [K]      run undone chunk K (default: the latest) again as a new chunk
  easel log           the session so far (= paintings/lua/<name>.lua)
  easel status        chunks, clock, wet or dry
  easel save [path]   the canvas as a PNG (default out/easel/<name>/<name>.png)
  easel frames on|off save a frame after every chunk (a time-lapse)
  easel check         replay the log from scratch and compare with the live canvas
  easel close         end the session (the log stays)
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
        "hash-probe" => {
            println!("{}", session::hash_probe());
            Ok(())
        }
        "help" | "-h" | "--help" => {
            println!("{USAGE}");
            Ok(())
        }
        "do" | "try" | "look" | "undo" | "log" | "status" | "save" | "frames" | "check" | "close" => client(&cmd, &rest, name),
        "edit" | "show" | "undone" | "redo" => edit::client(&cmd, &rest, name),
        o => Err(format!("unknown command {o:?}\n\n{USAGE}")),
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

fn flag(args: &[String], f: &str) -> Option<String> {
    args.iter().position(|a| a == f).and_then(|i| args.get(i + 1).cloned())
}

fn valid_name(n: &str) -> Result<(), String> {
    if n.is_empty() || !n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        return Err(format!("session name {n:?}: letters, digits, _ and - only"));
    }
    Ok(())
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
    std::fs::read_to_string(root().join("out/easel/current"))
        .map(|s| s.trim().to_string())
        .map_err(|_| "no session: easel open <name> first (or pass -s <name>)".to_string())
}

fn client(cmd: &str, args: &[String], name: Option<String>) -> Result<(), String> {
    let name = current(name)?;
    let mut args = args.to_vec();
    let mut payload = Vec::new();
    if cmd == "do" || cmd == "try" {
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
            None => return Err("do: give a chunk: easel do '<lua>' | -f file.lua | - (stdin)".into()),
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
    let name = args.first().filter(|a| !a.starts_with('-')).ok_or("open <name> [--width 1000] [--undo 8]")?.clone();
    valid_name(&name)?;
    let width: usize = flag(args, "--width").map(|w| w.parse().map_err(|_| "--width N")).transpose()?.unwrap_or(1000);
    let undo: usize = flag(args, "--undo").map(|w| w.parse().map_err(|_| "--undo N")).transpose()?.unwrap_or(8);
    let keep: usize = flag(args, "--checkpoints").map(|w| w.parse().map_err(|_| "--checkpoints N")).transpose()?.unwrap_or(edit::CHECKPOINTS);
    let dir = session_dir(&name);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    std::fs::write(root().join("out/easel/current"), &name).map_err(|e| e.to_string())?;
    if let Ok((true, st)) = request(&name, "status", &[], &[]) {
        print!("reattached to {name:?}: {st}");
        return Ok(());
    }
    let _ = std::fs::remove_file(sock_path(&name));
    let log = std::fs::File::create(dir.join("server.log")).map_err(|e| e.to_string())?;
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    use std::os::unix::process::CommandExt;
    std::process::Command::new(exe)
        .args(["serve", &name, "--width", &width.to_string(), "--undo", &undo.to_string(), "--checkpoints", &keep.to_string()])
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
            print!("easel {name:?} open: {st}");
            if !st.contains("style=") {
                println!("next: easel do 'canvas{{style=\"friedrich\", aspect=1.4, seed=1}}'");
            }
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
    /// disk differs, someone edited it by hand and it must not be clobbered.
    written: Option<String>,
    /// Full-resolution crop windows following the log (`look --scale`).
    crops: crop::Crops,
}

fn serve(args: &[String]) -> Result<(), String> {
    let name = args.first().ok_or("serve <name>")?.clone();
    let width: usize = flag(args, "--width").and_then(|w| w.parse().ok()).unwrap_or(1000);
    let undo: usize = flag(args, "--undo").and_then(|w| w.parse().ok()).unwrap_or(8);
    let mut s = Session::new(width, undo).map_err(|e| format!("easel: fatal: {e}"))?;
    s.keep = flag(args, "--checkpoints").and_then(|w| w.parse().ok()).unwrap_or(edit::CHECKPOINTS);
    // resume from the log
    let lp = log_path(&name);
    let mut written = None;
    if let Ok(text) = std::fs::read_to_string(&lp) {
        written = Some(text.clone());
        let chunks = parse_program(&text);
        let t0 = Instant::now();
        for (i, c) in chunks.iter().enumerate() {
            if let Err(e) = s.run(c) {
                println!("warning: chunk {} of {} failed on replay and was dropped:\n{e}", i + 1, lp.display());
                break;
            }
        }
        println!("resumed {} chunks from {} in {:.1}s", s.log.len(), lp.display(), t0.elapsed().as_secs_f64());
    }
    let sock = sock_path(&name);
    let _ = std::fs::remove_file(&sock);
    let l = UnixListener::bind(&sock).map_err(|e| format!("easel: fatal: bind {}: {e}", sock.display()))?;
    let _ = std::io::stdout().flush();
    look::begin(&s.lua, "resume");
    let mut srv = Server { name, s, frames: false, written, crops: crop::Crops::default() };
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
        if cmd == "close" {
            let _ = std::fs::remove_file(&sock);
            break;
        }
    }
    Ok(())
}

impl Server {
    /// Write the session log. If the file was edited by hand since the
    /// easel last wrote it, the hand-edited version is kept beside it
    /// (`<name>.edited-N.lua`) and a note says so: the live session can't
    /// take in edits to chunks it has already painted (close, then reopen to
    /// replay the edited file).
    fn save_log(&mut self) -> Result<String, String> {
        let p = log_path(&self.name);
        std::fs::create_dir_all(p.parent().unwrap()).map_err(|e| e.to_string())?;
        let mut note = String::new();
        if let Ok(disk) = std::fs::read_to_string(&p)
            && self.written.as_deref() != Some(disk.as_str())
        {
            let mut k = 1;
            let kept = loop {
                let q = p.with_file_name(format!("{}.edited-{k}.lua", self.name));
                if !q.exists() {
                    break q;
                }
                k += 1;
            };
            std::fs::write(&kept, &disk).map_err(|e| e.to_string())?;
            note = format!(
                "note: {} was edited outside the session; your edited version is kept as {} (the session's own log is in {}). To paint from the edited version: easel close, copy it back, easel open.\n",
                p.display(),
                kept.display(),
                p.display()
            );
        }
        let text = self.s.program(&self.name);
        std::fs::write(&p, &text).map_err(|e| e.to_string())?;
        self.written = Some(text);
        Ok(note)
    }

    fn look(&mut self, args: &[String], path: Option<PathBuf>) -> Result<String, String> {
        let v = look::View::parse(args)?;
        let mut out = String::new();
        if let Some(cmd) = &v.show {
            out.push_str(&look::show_cmd(&self.s.lua, cmd));
        }
        let relief = self.s.st.borrow().style.as_ref().map(|s| s.relief).unwrap_or((0.5, 0.1));
        let t0 = Instant::now();
        let c = self.s.canvas().ok_or("no canvas yet: easel do 'canvas{style=\"friedrich\", aspect=1.4, seed=1}'")?;
        // probes read the live canvas (and the world in the globals)
        for (i, &(x, y)) in v.probes.iter().enumerate() {
            let t = look::probe_at(&self.s.lua, &c, x, y, None).and_then(|t| look::probe_text(&t)).map_err(|e| e.to_string())?;
            out.push_str(&format!("probe {} {t}\n", i + 1));
        }
        let (marks, from) = look::marks(&self.s.lua);
        if !marks.is_empty() {
            out.push_str(&format!("overlay: {} marks from {from} (look --show off hides them, --show clear drops them)\n", marks.len()));
        }
        let dir = session_dir(&self.name);
        let path = path.unwrap_or_else(|| {
            let n = std::fs::read_dir(&dir).map(|d| d.filter_map(|e| e.ok()).filter(|e| e.file_name().to_string_lossy().starts_with("look-")).count()).unwrap_or(0);
            dir.join(format!("look-{:04}.jpg", n + 1))
        });
        let width = v.scale.map(|s| (s * 1000.0).round() as usize).filter(|&w| w != self.s.st.borrow().width);
        let (w, h) = match (width, v.crop) {
            (Some(width), Some(crop)) => {
                let hgt = c.frame().height();
                drop(c);
                let log: Vec<String> = self.s.log.iter().map(|c| c.src.clone()).collect();
                let got = self.crops.get(width, crop, hgt, &log, v.wait)?;
                out.push_str(&got.note);
                look::look(&got.canvas, got.relief, &v, &marks, &path)?
            }
            _ => look::look(&c, relief, &v, &marks, &path)?,
        };
        out.push_str(&format!("{} ({w}x{h}, {:.2}s)\n", path.display(), t0.elapsed().as_secs_f64()));
        Ok(out)
    }

    /// Run a chunk as `do` does, in a live session.
    fn run_chunk(&mut self, payload: &str, from: &str) -> Result<session::Ran, String> {
        look::begin(&self.s.lua, from);
        self.s.run(payload)
    }

    fn sync_crops(&mut self) {
        let log: Vec<String> = self.s.log.iter().map(|c| c.src.clone()).collect();
        self.crops.sync(&log);
    }

    fn handle(&mut self, cmd: &str, args: &[String], payload: &str) -> Result<String, String> {
        match cmd {
            "status" => Ok(format!("{}\n", self.s.status())),
            "try" => {
                // run it to see what it shows and prints, then take it back
                let ran = look::try_chunk(&mut self.s, payload).map_err(|e| format!("{e}\n(nothing changed)"))?;
                let mut out = ran.out;
                let (marks, _) = look::marks(&self.s.lua);
                out.push_str(&format!("tried ({:.2}s): rolled back, not logged; {} overlay marks for the next look\n", ran.secs, marks.len()));
                if args.iter().any(|a| a == "--look") {
                    out.push_str(&self.look(&[], None)?);
                }
                Ok(out)
            }
            "do" => {
                let from = format!("chunk {}", self.s.log.len() + 1);
                let r = self.run_chunk(payload, &from);
                match r {
                    Ok(ran) => {
                        let note = self.save_log()?;
                        self.sync_crops();
                        let n = self.s.log.len();
                        let mut out = note;
                        out.push_str(&ran.out);
                        let fields = if ran.field_secs > 0.005 { format!(" (Lua fields {:.2}s)", ran.field_secs) } else { String::new() };
                        out.push_str(&format!("ok · chunk {n} · {:.2}s{fields} · {}\n", ran.secs, self.s.status().split(" · ").skip(3).collect::<Vec<_>>().join(" · ")));
                        if self.frames {
                            let p = session_dir(&self.name).join("frames").join(format!("{n:04}.jpg"));
                            self.look(&[], Some(p))?;
                        }
                        if args.iter().any(|a| a == "--look") {
                            out.push_str(&self.look(&[], None)?);
                        }
                        Ok(out)
                    }
                    Err(e) => Err(format!("{e}\n(rolled back: the canvas is as it was before this chunk)")),
                }
            }
            "look" => self.look(args, None),
            "undo" => {
                let n: usize = args.first().map(|a| a.parse().map_err(|_| "undo [n]")).transpose()?.unwrap_or(1);
                let len = self.s.log.len();
                let gone = self.s.undo(n)?;
                edit::keep_undone(&self.name, len + 1 - gone.len(), "undone", &gone)?;
                self.sync_crops();
                let note = self.save_log()?;
                Ok(format!("{note}undid {n} · {}\n", self.s.status()))
            }
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
                    Err("replay DIFFERS from the live canvas: a chunk depended on state from a failed or undone chunk (e.g. a table it changed); the log is the truth: close and reopen to continue from it".into())
                }
            }
            "close" => {
                let note = self.save_log()?;
                Ok(format!("{note}closed; the session is in {}\n", log_path(&self.name).display()))
            }
            "edit" | "show" | "undone" | "redo" => self.handle_edit(cmd, args, payload),
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
    if args.iter().any(|a| a == "--look") {
        let relief = s.st.borrow().style.as_ref().map(|s| s.relief).unwrap_or((0.5, 0.1));
        let jpg = out.with_extension("jpg");
        let (w, h) = look::look(&c, relief, &look::View::default(), &[], &jpg)?;
        println!("{} ({w}x{h})", jpg.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    //! The live server's command path (`Server::handle`), where the overlay
    //! store is live and taken-out code goes to the undone file.
    use super::*;

    /// Removes a test session's log and directory, before and after.
    struct Scratch(String);
    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(log_path(&self.0));
            let _ = std::fs::remove_dir_all(session_dir(&self.0));
        }
    }

    fn server(name: &str) -> (Server, Scratch) {
        let g = Scratch(format!("test-{name}"));
        let _ = std::fs::remove_file(log_path(&g.0));
        let _ = std::fs::remove_dir_all(session_dir(&g.0));
        let mut s = Session::new(160, 8).unwrap();
        s.keep = edit::CHECKPOINTS;
        look::begin(&s.lua, "resume");
        let mut srv = Server { name: g.0.clone(), s, frames: false, written: None, crops: crop::Crops::default() };
        srv.handle("do", &[], "canvas{aspect=1.4, seed=1}").unwrap();
        (srv, g)
    }

    fn overlay(srv: &Server) -> (Vec<String>, String) {
        let (m, from) = look::marks(&srv.s.lua);
        (m.iter().map(|m| m.label.clone().unwrap_or_default()).collect(), from)
    }

    fn a(s: &str) -> Vec<String> {
        s.split_whitespace().map(|s| s.to_string()).collect()
    }

    // review 4 (session), finding 1: code with lines that look like the undone
    // file's headers (in a long string, in a comment) comes back intact
    #[test]
    fn undone_code_round_trips_marker_like_lines() {
        let (mut srv, _g) = server("undone-markers");
        let chunk = "message = [[first line\n--@ undone this is painting text, not an archive delimiter\n--@ undone 9 · was chunk 1 · undone · 3 bytes\nlast line]]\n--[[\n--@ undone 2 · was chunk 7 · undone\n]]\nprint(#message)";
        srv.handle("do", &[], chunk).unwrap();
        srv.handle("do", &[], "x = 1").unwrap();
        srv.handle("undo", &a("2"), "").unwrap();
        let list = srv.handle("undone", &[], "").unwrap();
        assert_eq!(list.lines().filter(|l| l.contains(" · was chunk ")).count(), 2, "two entries:\n{list}");
        assert_eq!(srv.handle("undone", &a("1"), "").unwrap(), format!("{chunk}\n"));
        assert_eq!(srv.handle("undone", &a("2"), "").unwrap(), "x = 1\n");
        // redo runs it again, whole
        let long = chunk.split_once("[[").unwrap().1.split_once("]]").unwrap().0;
        let out = srv.handle("redo", &a("1"), "").unwrap();
        assert!(out.contains(&format!("{}\n", long.len())), "{out}");
        assert_eq!(srv.s.log[1].src, chunk);
        // and edit --undone puts it in place of a chunk
        srv.handle("do", &[], "y = 2").unwrap();
        srv.handle("edit", &a("3 --undone 1"), "").unwrap();
        assert_eq!(srv.s.log[2].src, chunk);
        // the replaced chunk is kept as entry 3, after both
        assert_eq!(srv.handle("undone", &a("3"), "").unwrap(), "y = 2\n");
        assert!(srv.handle("check", &[], "").is_ok());
    }

    // finding 2: an edit replays its chunks as `do` ran them (each one's
    // first show() replaces the overlay), not piling their marks up
    #[test]
    fn an_edit_replays_the_overlay_chunk_by_chunk() {
        let (mut srv, _g) = server("edit-overlay");
        srv.handle("do", &[], r#"show(100, 100, "old")"#).unwrap();
        srv.handle("do", &[], r#"show(200, 100, "latest")"#).unwrap();
        assert_eq!(overlay(&srv), (vec!["latest".to_string()], "chunk 3".to_string()));
        srv.handle("edit", &a("2"), r#"show(100, 200, "replacement")"#).unwrap();
        assert_eq!(overlay(&srv), (vec!["latest".to_string()], "chunk 3".to_string()));
        // the last chunk shows nothing: the overlay is the one before it, and says so
        srv.handle("edit", &a("3"), "x = 1").unwrap();
        assert_eq!(overlay(&srv), (vec!["replacement".to_string()], "chunk 2".to_string()));
        let look = srv.handle("look", &a("--size 100"), "").unwrap();
        assert!(look.contains("overlay: 1 marks from chunk 2"), "{look}");
    }

    // a failed edit changes nothing, the overlay included
    #[test]
    fn a_failed_edit_leaves_the_overlay_as_it_was() {
        let (mut srv, _g) = server("failed-edit-overlay");
        srv.handle("do", &[], r#"show(100, 100, "original")"#).unwrap();
        let e = srv.handle("edit", &a("2"), r#"show(900, 900, "failed replacement"); error("stop")"#).unwrap_err();
        assert!(e.contains("nothing changed"), "{e}");
        assert_eq!(overlay(&srv), (vec!["original".to_string()], "chunk 2".to_string()));
        // a replayed chunk after the edited one fails
        srv.handle("do", &[], r#"show(300, 300, "third"); assert(not flag, "flagged")"#).unwrap();
        let e = srv.handle("edit", &a("2"), r#"show(1, 1, "new"); flag = true"#).unwrap_err();
        assert!(e.contains("flagged") && e.contains("nothing changed"), "{e}");
        assert_eq!(overlay(&srv), (vec!["third".to_string()], "chunk 3".to_string()));
    }

    // the overlay keeps the label of the run that made it
    #[test]
    fn the_overlay_keeps_its_origin() {
        let (mut srv, _g) = server("overlay-origin");
        srv.handle("do", &[], r#"show(100, 100, "two")"#).unwrap();
        srv.handle("do", &[], "x = 1").unwrap();
        assert_eq!(overlay(&srv).1, "chunk 2");
        srv.handle("try", &[], r#"show(100, 100, "tried")"#).unwrap();
        srv.handle("do", &[], "y = 1").unwrap();
        assert_eq!(overlay(&srv), (vec!["tried".to_string()], "try".to_string()));
        let look = srv.handle("look", &a("--size 100"), "").unwrap();
        assert!(look.contains("overlay: 1 marks from try"), "{look}");
    }
}
