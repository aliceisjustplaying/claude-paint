//! Editing a chunk in place: `easel edit N`, `show N`, `undone`, `redo`.
//!
//! The log stays the one source of truth: an edit replaces chunk N in it
//! and replays every chunk after it, starting from the nearest checkpoint
//! at or before N (a full snapshot of the session: canvas, Lua heap,
//! brushes, clock, kept in memory; see `Session::splice`). Code taken out
//! of the log, by `undo` or by an edit, is kept in
//! `out/easel/<name>/undone.lua`, so undo never throws work away.

use crate::{Server, request, session_dir};
use std::io::Read;
use std::path::PathBuf;

/// Checkpoints kept beyond the undo snapshots by default. Each costs what
/// an undo snapshot does (about 50 MB at 1000px).
pub const CHECKPOINTS: usize = 6;

/// Marks an entry in the undone file.
const UNDONE: &str = "--@ undone";

fn undone_path(name: &str) -> PathBuf {
    session_dir(name).join("undone.lua")
}

/// The undone file's entries: (header, code).
fn undone_entries(name: &str) -> Vec<(String, String)> {
    let text = std::fs::read_to_string(undone_path(name)).unwrap_or_default();
    let mut out: Vec<(String, String)> = Vec::new();
    for l in text.lines() {
        if let Some(h) = l.strip_prefix(UNDONE) {
            out.push((h.trim().to_string(), String::new()));
        } else if let Some((_, c)) = out.last_mut() {
            c.push_str(l);
            c.push('\n');
        }
    }
    for (_, c) in out.iter_mut() {
        *c = c.trim_end().to_string();
    }
    out
}

/// Keep chunks taken out of the log (`first` is the first one's number).
pub fn keep_undone(name: &str, first: usize, why: &str, srcs: &[String]) -> Result<(), String> {
    if srcs.is_empty() {
        return Ok(());
    }
    let p = undone_path(name);
    std::fs::create_dir_all(p.parent().unwrap()).map_err(|e| e.to_string())?;
    let mut n = undone_entries(name).len();
    let mut text = std::fs::read_to_string(&p).unwrap_or_default();
    for (i, s) in srcs.iter().enumerate() {
        n += 1;
        text.push_str(&format!("{UNDONE} {n} · was chunk {} · {why}\n{s}\n\n", first + i));
    }
    std::fs::write(&p, text).map_err(|e| e.to_string())
}

fn undone_code(name: &str, k: Option<usize>) -> Result<(usize, String), String> {
    let e = undone_entries(name);
    if e.is_empty() {
        return Err("nothing undone yet".into());
    }
    let k = k.unwrap_or(e.len());
    e.get(k.wrapping_sub(1)).map(|(_, c)| (k, c.clone())).ok_or_else(|| format!("undone {k}: there are 1 to {}", e.len()))
}

// ---------------------------------------------------------------- client

pub fn client(cmd: &str, args: &[String], name: Option<String>) -> Result<(), String> {
    let name = crate::current(name)?;
    let mut args = args.to_vec();
    let mut payload = Vec::new();
    if cmd == "edit" {
        let n = args.first().filter(|a| a.parse::<usize>().is_ok()).cloned().ok_or("edit N '<lua>' | -f chunk.lua | - | --drop | --undone K  [--insert] [--look]")?;
        args.remove(0);
        let mut flags = vec![n];
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--look" | "--insert" | "--drop" => flags.push(args[i].clone()),
                "--undone" => {
                    flags.push("--undone".into());
                    flags.push(args.get(i + 1).cloned().ok_or("--undone K")?);
                    i += 1;
                }
                "-f" => {
                    let f = args.get(i + 1).ok_or("edit N -f <file>")?;
                    payload = std::fs::read(f).map_err(|e| format!("{f}: {e}"))?;
                    i += 1;
                }
                "-" => {
                    std::io::stdin().read_to_end(&mut payload).map_err(|e| e.to_string())?;
                }
                src => payload = src.as_bytes().to_vec(),
            }
            i += 1;
        }
        args = flags;
    }
    let (ok, body) = request(&name, cmd, &args, &payload)?;
    if ok {
        print!("{body}");
        Ok(())
    } else {
        Err(body.trim_end().to_string())
    }
}

// ---------------------------------------------------------------- server

impl Server {
    pub(crate) fn handle_edit(&mut self, cmd: &str, args: &[String], payload: &str) -> Result<String, String> {
        let num = |i: usize, what: &str| -> Result<Option<usize>, String> { args.get(i).filter(|a| !a.starts_with("--")).map(|a| a.parse::<usize>().map_err(|_| format!("{what}: want a number, got {a:?}"))).transpose() };
        let look = args.iter().any(|a| a == "--look");
        match cmd {
            "show" => {
                let n = num(0, "show N")?.ok_or("show N")?;
                let c = self.s.log.get(n.wrapping_sub(1)).ok_or_else(|| format!("chunk {n}: the log has chunks 1 to {}", self.s.log.len()))?;
                Ok(format!("{} {n} · clock {}\n{}\n", crate::session::MARK, c.clock, c.src))
            }
            "undone" => match num(0, "undone K")? {
                Some(k) => Ok(format!("{}\n", undone_code(&self.name, Some(k))?.1)),
                None => {
                    let e = undone_entries(&self.name);
                    if e.is_empty() {
                        return Ok(format!("nothing undone yet ({})\n", undone_path(&self.name).display()));
                    }
                    let mut out = String::new();
                    for (h, c) in &e {
                        let first = c.lines().find(|l| !l.trim().is_empty()).unwrap_or("").trim();
                        let first: String = first.chars().take(70).collect();
                        out.push_str(&format!("{h} · {} lines · {first}\n", c.lines().count()));
                    }
                    out.push_str(&format!("(easel undone K prints one; easel redo K runs it again; kept in {})\n", undone_path(&self.name).display()));
                    Ok(out)
                }
            },
            "redo" => {
                let (_, src) = undone_code(&self.name, num(0, "redo K")?)?;
                let a: Vec<String> = if look { vec!["--look".into()] } else { vec![] };
                self.handle("do", &a, &src)
            }
            "edit" => {
                let n = num(0, "edit N")?.ok_or("edit N")?;
                let drop = args.iter().any(|a| a == "--drop");
                let insert = args.iter().any(|a| a == "--insert");
                let from_undone = args.iter().position(|a| a == "--undone").and_then(|i| args.get(i + 1)).map(|k| k.parse::<usize>().map_err(|_| "--undone K")).transpose()?;
                let src = match from_undone {
                    Some(k) => Some(undone_code(&self.name, Some(k))?.1),
                    None if drop => None,
                    None if payload.trim().is_empty() => return Err("edit N: give the new chunk ('<lua>', -f file, - for stdin, --undone K) or --drop".into()),
                    None => Some(payload.to_string()),
                };
                if drop && src.is_some() {
                    return Err("edit N --drop takes no code".into());
                }
                let len = self.s.log.len();
                let full: f64 = self.s.log.iter().map(|c| c.secs).sum();
                let remove = if insert { 0 } else { 1 };
                if !insert && n > len {
                    return Err(format!("chunk {n}: the log has chunks 1 to {len} (easel do adds one at the end)"));
                }
                let ins: Vec<String> = src.into_iter().collect();
                let r = self.s.splice(n, remove, &ins)?;
                // the live 3200 crops follow the new log
                self.sync_crops();
                let what = if drop { "dropped" } else if insert { "inserted before" } else { "replaced" };
                keep_undone(&self.name, n, &format!("{what} by edit"), &r.removed)?;
                let note = self.save_log()?;
                let mut out = note;
                out.push_str(&format!(
                    "{what} chunk {n} · replayed {} chunks from the checkpoint after chunk {} in {:.1}s (painting the whole log took {full:.1}s)\n",
                    r.replayed, r.from, r.secs
                ));
                if insert || drop {
                    out.push_str("note: the chunks after it are renumbered, so their random choices (rand, auto seeds) changed\n");
                }
                if !r.removed.is_empty() {
                    out.push_str(&format!("the old code is kept: easel undone {}\n", undone_entries(&self.name).len()));
                }
                out.push_str(&format!("{}\n", self.s.status()));
                if look {
                    out.push_str(&self.look(&[], None)?);
                }
                Ok(out)
            }
            o => Err(format!("unknown command {o:?}")),
        }
    }
}
