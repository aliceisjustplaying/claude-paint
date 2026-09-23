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
    parse_undone(&std::fs::read_to_string(undone_path(name)).unwrap_or_default())
}

/// One entry of the undone file: a header line that ends in the code's
/// length, then exactly that many bytes of code and a blank line. The code
/// is taken by length, never by looking for the next header, so a chunk
/// may hold lines that look like headers (in a long string or a comment).
fn undone_entry(n: usize, chunk: usize, why: &str, src: &str) -> String {
    format!("{UNDONE} {n} · was chunk {chunk} · {why} · {} bytes\n{src}\n\n", src.len())
}

/// The code length at the end of a header (`... · 123 bytes`), and the
/// header without it.
fn header_len(h: &str) -> Option<(&str, usize)> {
    let (head, n) = h.strip_suffix(" bytes")?.rsplit_once(" · ")?;
    Some((head, n.parse().ok()?))
}

/// Parse the undone file. Entries written before headers carried the
/// length (easel before review 4) run to the next header line; they are
/// read as they always were.
fn parse_undone(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut i = 0;
    // skip anything before the first header
    while i < text.len() && !text[i..].starts_with(UNDONE) {
        i = text[i..].find('\n').map_or(text.len(), |k| i + k + 1);
    }
    while i < text.len() {
        let eol = text[i..].find('\n').map_or(text.len(), |k| i + k);
        let h = text[i + UNDONE.len()..eol].trim();
        let body = (eol + 1).min(text.len());
        if let Some((head, n)) = header_len(h)
            && let Some(end) = body.checked_add(n)
            && text.is_char_boundary(end)
        {
            out.push((head.to_string(), text[body..end].to_string()));
            i = end;
            // the blank line after the code
            for _ in 0..2 {
                if text[i..].starts_with('\n') {
                    i += 1;
                }
            }
            continue;
        }
        // an old entry: its code runs to the next header line
        let mut end = body;
        while end < text.len() && !text[end..].starts_with(UNDONE) {
            end = text[end..].find('\n').map_or(text.len(), |k| end + k + 1);
        }
        out.push((h.to_string(), text[body..end].trim_end().to_string()));
        i = end;
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
    let mut text = std::fs::read_to_string(&p).unwrap_or_default();
    let mut n = parse_undone(&text).len();
    if !text.is_empty() && !text.ends_with('\n') {
        text.push('\n');
    }
    for (i, s) in srcs.iter().enumerate() {
        n += 1;
        text.push_str(&undone_entry(n, first + i, why, s));
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

#[cfg(test)]
mod tests {
    use super::*;

    // review 4 (session), finding 1: the code is taken by length, so lines
    // that look like headers stay inside it
    #[test]
    fn undone_entries_round_trip_whatever_the_code_holds() {
        let codes = [
            "message = [[first line\n--@ undone this is painting text, not an archive delimiter\nlast line]]\nprint(message)",
            "--[[\n--@ undone 3 · was chunk 9 · undone · 12 bytes\n]]\nx = 1",
            "s = [==[\n\n--@ undone\n\n]==]",
            "-- ünïcödé · and a trailing blank line\n\n",
            "y = 2",
        ];
        let mut text = String::new();
        for (i, c) in codes.iter().enumerate() {
            text.push_str(&undone_entry(i + 1, i + 4, "undone", c));
        }
        let e = parse_undone(&text);
        assert_eq!(e.len(), codes.len());
        for (i, (h, c)) in e.iter().enumerate() {
            assert_eq!(c, codes[i]);
            assert_eq!(h, &format!("{} · was chunk {} · undone", i + 1, i + 4));
        }
    }

    // files written before the lengths still read as they did, and new
    // entries follow them
    #[test]
    fn old_undone_files_still_read() {
        let mut text = "--@ undone 1 · was chunk 3 · undone\nx = 1\nprint(x)\n\n--@ undone 2 · was chunk 4 · replaced by edit\ny = 2\n\n".to_string();
        text.push_str(&undone_entry(3, 5, "undone", "--@ undone look-alike\nz = 3"));
        let e = parse_undone(&text);
        let got: Vec<(&str, &str)> = e.iter().map(|(h, c)| (h.as_str(), c.as_str())).collect();
        assert_eq!(
            got,
            [
                ("1 · was chunk 3 · undone", "x = 1\nprint(x)"),
                ("2 · was chunk 4 · replaced by edit", "y = 2"),
                ("3 · was chunk 5 · undone", "--@ undone look-alike\nz = 3"),
            ]
        );
        // a length that doesn't fit (a hand-edited file) falls back to the old reading
        let bad = "--@ undone 1 · was chunk 3 · undone · 999 bytes\nx = 1\n\n--@ undone 2 · was chunk 4 · undone · 5 bytes\ny = 2\n\n";
        let e = parse_undone(bad);
        assert_eq!(e.len(), 2);
        assert_eq!((e[0].1.as_str(), e[1].1.as_str()), ("x = 1", "y = 2"));
        assert!(parse_undone("").is_empty());
    }
}
