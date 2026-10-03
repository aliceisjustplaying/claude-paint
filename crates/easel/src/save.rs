//! The save file a live session writes when it closes (`live.ckpt`, next to
//! `live.png`): the canvas (`Canvas::write_state`) with, in its header, the
//! studio fields the finishing verbs read. `easel finish` (replay build)
//! restores it into a fresh session and runs the finishing chunk there, so a
//! finished picture no longer needs a replay of the whole log.
//!
//! Saved: the canvas (paint's checkpoint, unchanged), and the fields of the
//! session's snapshot (`session::Snap`) other than the Lua heap, the brushes
//! and the world view: the seed, the clock and its start, the hand (the piles
//! on the palette), the `canvas{}` setup and the style's relief (`relief()`
//! reads it; the rest of the style is the box's oil style, which no finishing
//! verb reads); and the chunk and call counters. The header also names the
//! box and engine and holds the log's FNV-1a hash and chunk count, so a save
//! is used only with the log it was written from.
//!
//! The header is plain `key=value` lines and holds no time or path: a replay
//! of the log closed in another folder writes the same bytes, which
//! `scripts/check_painting` compares.

use crate::session::Session;
use std::io::Write;

/// The first line of a save's header.
const MAGIC: &str = "easel save 1";

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
    let st = s.st.borrow();
    let mut h = format!("{MAGIC}\n");
    let mut kv = |k: &str, v: String| {
        h.push_str(k);
        h.push('=');
        h.push_str(&v);
        h.push('\n');
    };
    kv("log_fnv", format!("{:016x}", fnv1a(program.as_bytes())));
    kv("chunks", s.log.len().to_string());
    kv("box", st.tubes.name.to_string());
    kv("engine", st.tubes.engine.to_string());
    kv("width", st.width.to_string());
    kv("seed", st.seed.to_string());
    kv("chunk", st.chunk.to_string());
    kv("calls", st.calls.to_string());
    kv("clock", format!("{:016x}", st.clock.to_bits()));
    kv("clock0", format!("{:016x}", st.clock0.to_bits()));
    if let Some(setup) = &st.setup {
        kv("setup", setup.replace('\n', " "));
    }
    if let Some(sty) = &st.style {
        kv("style", sty.name.to_string());
        kv("relief", format!("{:08x},{:08x}", sty.relief.0.to_bits(), sty.relief.1.to_bits()));
    }
    let piles: Vec<String> = st.hand.piles.piles.iter().map(|p| format!("{:08x}{:08x}{:08x}", p[0].to_bits(), p[1].to_bits(), p[2].to_bits())).collect();
    kv("piles", piles.join(","));
    h
}

/// Write the session's save to `path` (through a temporary file, renamed into place). No
/// canvas yet: nothing is written and an old save there is removed.
pub fn write(s: &Session, program: &str, path: &std::path::Path) -> Result<bool, String> {
    let st = s.st.borrow();
    let Some(c) = st.canvas.as_ref() else {
        let _ = std::fs::remove_file(path);
        return Ok(false);
    };
    let part = path.with_extension("ckpt.part");
    let f = std::fs::File::create(&part).map_err(|e| format!("{}: {e}", part.display()))?;
    let mut w = std::io::BufWriter::with_capacity(1 << 20, f);
    let head = header(s, program);
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
fn field<'a>(h: &'a std::collections::BTreeMap<&str, &str>, k: &str) -> Result<&'a str, String> {
    h.get(k).copied().ok_or_else(|| format!("the save has no {k}= in its header"))
}

#[cfg(feature = "replay")]
fn parse<T: std::str::FromStr>(h: &std::collections::BTreeMap<&str, &str>, k: &str) -> Result<T, String> {
    let v = field(h, k)?;
    v.parse().map_err(|_| format!("the save's {k}={v:?} doesn't parse"))
}

#[cfg(feature = "replay")]
fn hex64(h: &std::collections::BTreeMap<&str, &str>, k: &str) -> Result<u64, String> {
    let v = field(h, k)?;
    u64::from_str_radix(v, 16).map_err(|_| format!("the save's {k}={v:?} isn't hex"))
}

/// Read a save into a fresh replay session: the canvas, and the studio fields as the live
/// session left them. Its next chunk runs as chunk `chunks + 1`, as it would at the end of
/// the log.
#[cfg(feature = "replay")]
pub fn read(path: &std::path::Path) -> Result<Restored, String> {
    use crate::session::{BOX_MARK, ENGINE_MARK};
    use std::rc::Rc;
    let f = std::fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut r = std::io::BufReader::with_capacity(1 << 20, f);
    let (canvas, head) = paint::Canvas::read_state(&mut r).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut lines = head.lines();
    if lines.next() != Some(MAGIC) {
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
        st.seed = parse(&h, "seed")?;
        st.chunk = parse(&h, "chunk")?;
        st.calls = parse(&h, "calls")?;
        st.clock = f64::from_bits(hex64(&h, "clock")?);
        st.clock0 = f64::from_bits(hex64(&h, "clock0")?);
        st.setup = h.get("setup").map(|v| v.to_string());
        if h.contains_key("style") {
            let rel = field(&h, "relief")?;
            let bits = |v: &str| u32::from_str_radix(v, 16).map(f32::from_bits).map_err(|_| format!("the save's relief={rel:?} isn't two hex floats"));
            let (a, b) = rel.split_once(',').ok_or_else(|| format!("the save's relief={rel:?} isn't two hex floats"))?;
            st.style = Some(Rc::new(paint::Style { relief: (bits(a)?, bits(b)?), ..paint::Style::oil_with(tubes) }));
        }
        let piles = field(&h, "piles")?;
        st.hand.piles.piles = piles
            .split(',')
            .filter(|p| !p.is_empty())
            .map(|p| {
                let q = |i: usize| p.get(i * 8..i * 8 + 8).and_then(|v| u32::from_str_radix(v, 16).ok()).map(f32::from_bits);
                match (p.len(), q(0), q(1), q(2)) {
                    (24, Some(a), Some(b), Some(c)) => Ok([a, b, c]),
                    _ => Err(format!("the save's pile {p:?} isn't three hex floats")),
                }
            })
            .collect::<Result<_, _>>()?;
        st.canvas = Some(canvas);
    }
    let chunks: usize = parse(&h, "chunks")?;
    s.chunks_before = chunks;
    Ok(Restored { session: s, chunks, log_fnv: hex64(&h, "log_fnv")? })
}
