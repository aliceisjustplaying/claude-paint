//! `easel run --dump-state <dir>` (replay build): after every chunk, every
//! state value of the session as named fields, one file a chunk
//! (`<dir>/chunk-<NNN>.state`), and after the last chunk the session's save
//! (`<dir>/final.ckpt`, as `easel close` writes it, `save.rs`). It only
//! reads the session: a run with it paints and digests exactly as a run
//! without it (`notes/thinner/baseline/README.md` has the proof).
//!
//! File format `CPSTATE1` (`scripts/state_compare` reads it):
//!
//!   8 bytes   "CPSTATE1"
//!   u64 LE    length of the header
//!   header    JSON: {"format", "chunk", "digest", "fields": [{"name",
//!             "dtype", "shape", "offset", "nbytes"}, ...]}; offsets count
//!             from the end of the header
//!   data      the fields' values, little-endian, row major
//!
//! dtype is f32, f64, u32, u64 or utf8 (text, shape [bytes]). `digest` is the
//! line `easel run --state-digest` writes for this chunk (with secs=0.000).
//! The fields:
//!
//! - the canvas's, `paint::Canvas::state_fields` (paint's state_dump.rs);
//! - `studio.*`: seed, chunk, calls, clock, clock0 and the piles on the
//!   palette (OKLab, `[n, 3]`);
//! - `brushes.debug`, `rags.debug`, `knives.debug` and `studio.debug`: the exact text the
//!   state digest hashes (the held brushes' `Debug`, one a line, every
//!   bristle's load, pigment mix, bend and cure; the rags' and knives'; the studio's
//!   seed, clocks, counters, setup, piles and RNG). Rust's `Debug` prints
//!   floats in their shortest round-trip form, so the text holds every value
//!   exactly; `scripts/state_compare` parses it into named values.
//! - `studio.board`: the palette board's exact `Debug` text.

use crate::session::Session;
use std::path::Path;

fn json_str(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

/// Write the state after chunk `n` to `<dir>/chunk-<NNN>.state`.
pub fn write_chunk(s: &Session, n: usize, dir: &Path) -> Result<(), String> {
    let digest = crate::state_digest_line(s, n, 0.0);
    let fields = s.st.borrow_mut().dump_fields();
    let mut head = format!("{{\"format\": \"CPSTATE1\", \"chunk\": {n}, \"digest\": {}, \"fields\": [", json_str(digest.trim_end()));
    let mut off = 0usize;
    for (i, (name, dtype, shape, bytes)) in fields.iter().enumerate() {
        let shape: Vec<String> = shape.iter().map(|d| d.to_string()).collect();
        head.push_str(&format!(
            "{}{{\"name\": {}, \"dtype\": \"{dtype}\", \"shape\": [{}], \"offset\": {off}, \"nbytes\": {}}}",
            if i == 0 { "" } else { ", " },
            json_str(name),
            shape.join(", "),
            bytes.len()
        ));
        off += bytes.len();
    }
    head.push_str("]}\n");
    let mut out = Vec::with_capacity(16 + head.len() + off);
    out.extend_from_slice(b"CPSTATE1");
    out.extend_from_slice(&(head.len() as u64).to_le_bytes());
    out.extend_from_slice(head.as_bytes());
    for (_, _, _, bytes) in &fields {
        out.extend_from_slice(bytes);
    }
    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let path = dir.join(format!("chunk-{n:03}.state"));
    std::fs::write(&path, out).map_err(|e| format!("{}: {e}", path.display()))
}

/// After the last chunk: the session's save, as a closing live session writes it.
pub fn write_save(s: &Session, program: &str, dir: &Path) -> Result<(), String> {
    crate::save::write(s, program, &dir.join("final.ckpt")).map(|_| ())
}
