//! The save file a live session writes when it closes (`live.ckpt`, next to
//! `live.png`): the canvas (`Canvas::write_state`) with, in its header, the
//! studio fields the finishing verbs read. `easel finish` (replay build)
//! restores it into a fresh session and runs the finishing chunk there, so a
//! finished picture no longer needs a replay of the whole log.
//!
//! Saved: the canvas (paint's checkpoint, unchanged) and the finishing
//! fields from the studio inventory (studio_state.rs): seed, clocks, piles,
//! palette board, setup, style relief and chunk/call counters. Lua, held tools
//! and world references are not finishing inputs and are not saved. Version 1
//! saves load with an untouched board; version 2 stores its exact float bits.
//! The header also names the box and engine and holds the log's FNV-1a hash
//! and chunk count, so a save is used only with its original log.
//!
//! The header is plain `key=value` lines and holds no time or path: a replay
//! of the log closed in another folder writes the same bytes, which
//! `scripts/check_painting` compares.

use crate::session::Session;
use std::io::Write;

/// The first line of a save's header.
const MAGIC: &str = "easel save 2";

/// FNV-1a, 64 bit.
pub fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// The header: the studio fields, then what ties the save to its log.
fn header(s: &Session, program: &str) -> String {
    format!("{MAGIC}\nlog_fnv={:016x}\nchunks={}\n{}", fnv1a(program.as_bytes()), s.log.len(), s.st.borrow_mut().save_header())
}

/// Write the session's save to `path` (through a temporary file, renamed into place). No
/// canvas yet: nothing is written and an old save there is removed.
pub fn write(s: &Session, program: &str, path: &std::path::Path) -> Result<bool, String> {
    let head = header(s, program);
    let st = s.st.borrow();
    let Some(c) = st.canvas.as_ref() else {
        let _ = std::fs::remove_file(path);
        return Ok(false);
    };
    let part = path.with_extension("ckpt.part");
    let f = std::fs::File::create(&part).map_err(|e| format!("{}: {e}", part.display()))?;
    let mut w = std::io::BufWriter::with_capacity(1 << 20, f);
    c.write_state(&mut w, &head)
        .and_then(|_| w.flush())
        .and_then(|_| w.get_ref().sync_all())
        .map_err(|e| format!("{}: {e}", part.display()))?;
    drop(w);
    std::fs::rename(&part, path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(true)
}

/// A save read back: the session holding its state, and what ties it to its log.
#[cfg(feature = "replay")]
pub struct Restored {
    pub session: Session,
    pub chunks: usize,
    pub log_fnv: u64,
}

#[cfg(feature = "replay")]
pub(crate) fn field<'a>(h: &'a std::collections::BTreeMap<&str, &str>, k: &str) -> Result<&'a str, String> {
    h.get(k).copied().ok_or_else(|| format!("the save has no {k}= in its header"))
}

#[cfg(feature = "replay")]
pub(crate) fn parse<T: std::str::FromStr>(h: &std::collections::BTreeMap<&str, &str>, k: &str) -> Result<T, String> {
    let v = field(h, k)?;
    v.parse().map_err(|_| format!("the save's {k}={v:?} doesn't parse"))
}

#[cfg(feature = "replay")]
pub(crate) fn hex64(h: &std::collections::BTreeMap<&str, &str>, k: &str) -> Result<u64, String> {
    let v = field(h, k)?;
    u64::from_str_radix(v, 16).map_err(|_| format!("the save's {k}={v:?} isn't hex"))
}

/// Read a save into a fresh replay session: the canvas, and the studio fields as the live
/// session left them. Its next chunk runs as chunk `chunks + 1`, as it would at the end of
/// the log.
#[cfg(feature = "replay")]
pub fn read(path: &std::path::Path) -> Result<Restored, String> {
    use crate::session::{BOX_MARK, ENGINE_MARK};
    let f = std::fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut r = std::io::BufReader::with_capacity(1 << 20, f);
    let (canvas, head) = paint::Canvas::read_state(&mut r).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut lines = head.lines();
    let version = match lines.next() { Some("easel save 1") => 1, Some(MAGIC) => 2, _ => 0 };
    if version == 0 {
        return Err(format!("{}: not an easel save (its header doesn't start {MAGIC:?})", path.display()));
    }
    let h: std::collections::BTreeMap<&str, &str> = lines.filter_map(|l| l.split_once('=')).collect();
    // the box and engine as a log's head names them, so the box is found as a replay finds it
    let (bx, engine): (String, u32) = (field(&h, "box")?.to_string(), parse(&h, "engine")?);
    let mut logged = String::new();
    if paint::palette::default_box() != Some(bx.as_str()) {
        logged.push_str(&format!("{BOX_MARK} {bx}\n"));
    }
    if engine != 1 {
        logged.push_str(&format!("{ENGINE_MARK} {engine}\n"));
    }
    let tubes = crate::session::box_for(Some(&logged))?;
    let width: usize = parse(&h, "width")?;
    let mut s = Session::replay_with(width, tubes.clone()).map_err(|e| e.to_string())?;
    {
        let mut st = s.st.borrow_mut();
        st.read_header(&h, version)?;
        st.canvas = Some(canvas);
    }
    let chunks: usize = parse(&h, "chunks")?;
    s.chunks_before = chunks;
    Ok(Restored { session: s, chunks, log_fnv: hex64(&h, "log_fnv")? })
}
