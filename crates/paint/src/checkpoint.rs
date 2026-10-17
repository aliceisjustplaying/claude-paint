//! Saving and restoring the complete state of a canvas, so a painting can
//! resume after a stage instead of repainting it (see `paintings::run`).
//!
//! The state is everything later painting depends on: the frame (and crop
//! window), the dry picture (color, surface relief, film), the support
//! (linen, physical size), the wet layer (volume, pigment mix, scattering,
//! stiffness and drying rate, dirty box), the stroke counter and the
//! per-pixel ids of the last stroke to lay or touch paint there (`wait`
//! reads them to tell which films were worked), the clock and the drying
//! state (see `drying`), and the pencil drawing if there is one. Not stored, because nothing later reads them: the
//! film floors of past strokes and the brushes' contact surface (derived
//! from the relief and rebuilt on demand). Restoring is exact: a resumed run
//! paints bit-for-bit what an uninterrupted one does.
//!
//! Format: little-endian binary, `MAGIC`, then a free-form UTF-8 header
//! (length-prefixed; the caller's key=value lines), then the canvas. If you
//! add state to `Canvas` or `Wet`, add it here and bump `MAGIC`.
//!
//! The reader also accepts version 7 (`PAINTCK7`), the engine-1 layout before
//! edition numbers: it ends after the hand-time ledger, without an engine word.
//! New engine-1 writes remain version 8; no historical bytes are rewritten on read.
//! The format is version 8 (`MAGIC` is `PAINTCK8`) for engines 1 and 2 and
//! version 9 (`PAINTCK9`) for engine 3, or version 10 for a raw canvas
//! (below); files of any other version are refused (re-run to checkpoint
//! again). Version 9 is version 8's bytes after the magic, then the
//! thinner's section: the solvent in the open film, one f32 (µm) per buffer
//! pixel, row major, as the file's last 4 × pixels bytes (`crate::thinner`;
//! that it comes last is part of version 9; version 10 adds its section
//! after it). An engine-3 canvas saved as version 8 (by the easel before the
//! thinner, af49348) is refused, naming that version: it has no solvent
//! section, and this easel doesn't convert old saves. Version 11
//! (`PAINTC11`) is engine 4's and later: version 9 with two more properties
//! (solvent and oil) to each pixel of wet paint and, after the engine version and before
//! the solvent (still last), the surface's gloss and the ground's remaining
//! absorbency, one f32 each per pixel. An older engine's canvas is written
//! as it always was. Version 12 combines version 11 with the raw canvas's
//! soak section after the solvent. After the header the
//! writer stores, in order: the frame and crop window, the scale and mm per
//! unit, the linen (if any), the surface generation, the stroke counter and
//! dirty box, then per pixel the color, relief, film, wet volume, pigment
//! mix, hiding and the ids of the last stroke to lay and to touch paint;
//! the clock with its tacky box and each pixel's drying state; the ground
//! thickness (for craquelure fitted to the ground); each wet pixel's paint
//! coverage (pointed-tip marks); the drawing (`graphite::Drawing`), if any:
//! every cell of the deposit (coverage, flake reflectance, lift, fixed
//! floor, film when drawn) and the whole-canvas guide with its fixed floor;
//! and hand time (`tally`): the slice setting and the complete ledger, with
//! the part already on the clock, so a resumed hand-timed painting keeps
//! aging its passes and owes the time it owed; and the engine version it is
//! painted with (`crate::ENGINE`).
//!
//! A raw canvas (`crate::soak`, engine 3) is version 10 (`PAINTC10`):
//! version 9's bytes after the magic, then its soak section, which is the
//! file's last: a `SOAK` mark and the section's own version (1, `SOAK_V`),
//! the fabric (name, colour, pore volume, warp bias), the clock and seed the
//! cloth was set up with, and the weave per buffer pixel, row major. The
//! dry cloth's absorption isn't stored: it follows from the fabric's colour.
//! Every other canvas writes version 8 or 9 as before, so a reader that
//! doesn't know version 10 refuses a raw canvas instead of loading it
//! without its cloth. A section of any other version is refused too.

use crate::canvas::{Canvas, Frame};
use crate::surface::Linen;
use crate::wet::LAT;
use std::io::{self, Read, Write};

const MAGIC: &[u8; 8] = b"PAINTCK8";
const MAGIC7: &[u8; 8] = b"PAINTCK7";
/// Engine 3, with the thinner's section (`crate::thinner`).
const MAGIC9: &[u8; 8] = b"PAINTCK9";
/// A raw canvas: version 9, then its soak section (`crate::soak`).
const MAGIC10: &[u8; 8] = b"PAINTC10";
/// Engine 4 material properties, gloss and absorbency.
const MAGIC11: &[u8; 8] = b"PAINTC11";
/// Engine 4 materials followed by the raw canvas's soak section.
const MAGIC12: &[u8; 8] = b"PAINTC12";
/// Marks the soak section ("SOAK"), and its version.
const SOAK_MARK: u64 = 0x4b414f53;
const SOAK_V: u64 = 1;
const QUARTER_TILES: u64 = 1 << 33;
const ADAPTIVE_SWEEP: u64 = 1 << 34;
const GLAZE_FILM: u64 = 1 << 36;
const KNIFE_SKIM: u64 = 1 << 35;
// (1 << 35 and 1 << 36 are taken by the knife skim and the glaze film)
const BLEND_FEATHER: u64 = 1 << 37;
const BRUSH_SKIM: u64 = 1 << 38;
const RAG_LIFT: u64 = 1 << 39;
const LIQUID_FILM_LEVELS: u64 = 1 << 40;

/// Why an engine-3 PAINTCK8 file can't be read.
const OLD_ENGINE3: &str = "an engine-3 canvas saved before the thinner (format PAINTCK8, by the easel at af49348): this easel saves engine 3 as PAINTCK9 and doesn't convert old saves; open it with that version (git af49348) or replay its log";

fn put_u64(w: &mut impl Write, v: u64) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
fn put_f32(w: &mut impl Write, v: f32) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
fn get_u64(r: &mut impl Read) -> io::Result<u64> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b)?;
    Ok(u64::from_le_bytes(b))
}
fn get_f32(r: &mut impl Read) -> io::Result<f32> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(f32::from_le_bytes(b))
}

/// Write a slice of f32 in chunks (fast, no per-value syscalls).
fn put_all(w: &mut impl Write, v: impl Iterator<Item = f32>) -> io::Result<()> {
    let mut buf = Vec::with_capacity(1 << 16);
    for x in v {
        buf.extend_from_slice(&x.to_le_bytes());
        if buf.len() >= 1 << 16 {
            w.write_all(&buf)?;
            buf.clear();
        }
    }
    w.write_all(&buf)
}

fn get_all(r: &mut impl Read, n: usize) -> io::Result<Vec<f32>> {
    let mut bytes = vec![0u8; n * 4];
    r.read_exact(&mut bytes)?;
    Ok(bytes.as_chunks::<4>().0.iter().map(|b| f32::from_le_bytes(*b)).collect())
}

fn bad(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg.to_string())
}

fn write_soak(w: &mut impl Write, s: &crate::soak::Soak) -> io::Result<()> {
    let name = s.fabric.name.as_bytes();
    // (a fabric the reader would refuse is an error here, not a save that
    // can't be opened)
    if !s.fabric.is_valid() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("the raw canvas's fabric is invalid: {:?}", s.fabric)));
    }
    put_u64(w, SOAK_MARK)?;
    put_u64(w, SOAK_V)?;
    put_u64(w, name.len() as u64)?;
    w.write_all(name)?;
    let f = &s.fabric;
    for v in [f.color[0], f.color[1], f.color[2], f.cap_um, f.warp_bias] {
        put_f32(w, v)?;
    }
    put_u64(w, s.t0.to_bits())?;
    put_u64(w, s.seed)?;
    put_all(w, s.weave.iter().copied())
}

fn read_soak(r: &mut impl Read, n: usize) -> io::Result<crate::soak::Soak> {
    if get_u64(r)? != SOAK_MARK {
        return Err(bad("checkpoint soak mark is wrong"));
    }
    match get_u64(r)? {
        SOAK_V => {}
        v => return Err(bad(&format!("checkpoint soak section is version {v}; this easel reads version {SOAK_V} only"))),
    }
    let len = get_u64(r)?;
    if len > crate::soak::NAME_MAX as u64 {
        return Err(bad("checkpoint fabric name is invalid"));
    }
    let mut name = vec![0u8; len as usize];
    r.read_exact(&mut name)?;
    let name = String::from_utf8(name).map_err(|_| bad("checkpoint fabric name is not UTF-8"))?;
    let mut v = [0.0f32; 5];
    for x in v.iter_mut() {
        *x = get_f32(r)?;
    }
    let t0 = f64::from_bits(get_u64(r)?);
    let seed = get_u64(r)?;
    let weave = get_all(r, n)?;
    // (a corrupt file is an error here, not a panic or NaN pixels later)
    // a cloth of the caller's own keeps its name, spelt as it was; its
    // numbers come from the file, as a named one's do
    let fabric = crate::soak::Fabric { name: name.into(), color: [v[0], v[1], v[2]], cap_um: v[3], warp_bias: v[4] };
    let amp = crate::soak::WEAVE_AMP;
    if !(fabric.is_valid() && t0.is_finite() && weave.iter().all(|&q| (1.0 - amp..=1.0 + amp).contains(&q))) {
        return Err(bad("checkpoint soak is invalid"));
    }
    Ok(crate::soak::Soak::new(fabric, t0, seed, weave))
}

/// Read just the header of a checkpoint (to validate it before loading).
pub fn read_header(r: &mut impl Read) -> io::Result<String> {
    read_magic_header(r).map(|(_, h)| h)
}

/// The format version (7–12) and the header. An engine-3 canvas in a
/// PAINTCK8 file whose header says so (`engine=3`, as an easel save's
/// does) is refused here, before anything else is read.
fn read_magic_header(r: &mut impl Read) -> io::Result<(u32, String)> {
    let mut m = [0u8; 8];
    r.read_exact(&mut m)?;
    let version = match &m {
        m if m == MAGIC7 => 7,
        m if m == MAGIC => 8,
        m if m == MAGIC9 => 9,
        m if m == MAGIC10 => 10,
        m if m == MAGIC11 => 11,
        m if m == MAGIC12 => 12,
        _ => return Err(bad("not a canvas checkpoint (or an older format)")),
    };
    let n = get_u64(r)? as usize;
    if n > 1 << 20 {
        return Err(bad("checkpoint header too long"));
    }
    let mut h = vec![0u8; n];
    r.read_exact(&mut h)?;
    let head = String::from_utf8(h).map_err(|_| bad("checkpoint header is not UTF-8"))?;
    if version == 7 && head.lines().filter_map(|l| l.strip_prefix("engine=")).any(|v| v.trim() != "1") {
        return Err(bad("PAINTCK7 is an engine-1 checkpoint"));
    }
    if version == 8 && head.lines().filter_map(|l| l.strip_prefix("engine=")).any(|v| v.trim().parse::<u32>().is_ok_and(|e| e >= 3)) {
        return Err(bad(OLD_ENGINE3));
    }
    Ok((version, head))
}

impl Canvas {
    /// Write the complete canvas state (dries nothing: wet paint stays wet)
    /// after `header`.
    pub fn write_state(&self, w: &mut impl Write, header: &str) -> io::Result<()> {
        // (a raw canvas is engine 3's: version 10 holds version 9's bytes)
        if self.soak.is_some() && self.engine < 3 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "a raw canvas is engine 3's: this one is set to an older engine"));
        }
        let version = match (self.engine, self.soak.is_some()) {
            (3, true) => 10,
            (3, false) => 9,
            (4.., true) => 12,
            (4.., false) => 11,
            _ => 8,
        };
        w.write_all(match version {
            12 => MAGIC12,
            11 => MAGIC11,
            10 => MAGIC10,
            9 => MAGIC9,
            _ => MAGIC,
        })?;
        put_u64(w, header.len() as u64)?;
        w.write_all(header.as_bytes())?;
        let f = self.f;
        for v in [f.w, f.h, f.x0, f.y0, f.full_w, f.full_h, self.keep.0, self.keep.1, self.keep.2, self.keep.3] {
            put_u64(w, v as u64)?;
        }
        put_f32(w, f.scale)?;
        put_f32(w, self.mm_per_unit)?;
        match self.linen {
            None => put_u64(w, 0)?,
            Some(l) => {
                put_u64(w, 1)?;
                for v in [l.warp_per_cm, l.weft_per_cm, l.crown_um, l.slubs] {
                    put_f32(w, v)?;
                }
                put_u64(w, l.seed)?;
            }
        }
        put_u64(w, self.surf_gen)?;
        let wt = &self.wet;
        put_u64(w, wt.current as u64)?;
        match wt.dirty {
            None => put_u64(w, 0)?,
            Some((a, b, c, d)) => {
                put_u64(w, 1)?;
                for v in [a, b, c, d] {
                    put_u64(w, v as u64)?;
                }
            }
        }
        put_all(w, self.px.iter().flat_map(|p| *p))?;
        put_all(w, self.height.iter().copied())?;
        put_all(w, self.film.iter().copied())?;
        put_all(w, wt.vol.iter().copied())?;
        put_all(w, wt.lat.iter().flat_map(|l| *l))?;
        if version >= 11 {
            // (engine 4's five properties: the pigment's packing, `Prop`'s
            // last five, stays in memory, and reads back as paint naming no
            // tubes)
            put_all(w, wt.hide.iter().flat_map(|h| [h[0], h[1], h[2], h[3], h[4]]))?;
        } else {
            // (three properties, as before engine 4: its paint is a tube's in oil)
            put_all(w, wt.hide.iter().flat_map(|h| [h[0], h[1], h[2]]))?;
        }
        put_all(w, wt.stroke.iter().map(|&v| f32::from_bits(v)))?;
        put_all(w, wt.touched.iter().map(|&v| f32::from_bits(v)))?;
        let ck = &wt.clock;
        put_u64(w, ck.now.to_bits())?;
        put_u64(w, ck.mark as u64)?;
        match ck.tacky {
            None => put_u64(w, 0)?,
            Some((a, b, c, d)) => {
                put_u64(w, 1)?;
                for v in [a, b, c, d] {
                    put_u64(w, v as u64)?;
                }
            }
        }
        put_u64(w, u64::from(!ck.px.is_empty()))?;
        if !ck.px.is_empty() {
            put_all(w, ck.px.iter().flat_map(|p| [p.cure, p.lev, p.seen, p.sub, p.srate, p.th]))?;
        }
        put_f32(w, self.ground_um)?;
        put_all(w, wt.cover.iter().copied())?;
        match &self.drawing {
            None => put_u64(w, 0)?,
            Some(d) => {
                put_u64(w, 1)?;
                put_all(w, d.to_f32s())?;
            }
        }
        // hand time (version 7)
        match self.hand_slice {
            None => put_u64(w, 0)?,
            Some(m) => {
                put_u64(w, 1)?;
                put_f32(w, m)?;
            }
        }
        for v in self.tally.to_words() {
            put_u64(w, v)?;
        }
        put_u64(w, self.engine as u64 | if self.liquid_film_levels { LIQUID_FILM_LEVELS } else { 0 } | if self.quarter_tiles { QUARTER_TILES } else { 0 } | if self.adaptive_sweep { ADAPTIVE_SWEEP } else { 0 } | if self.glaze_film { GLAZE_FILM } else { 0 } | if self.rag_lift { RAG_LIFT } else { 0 } | if self.knife_skim { KNIFE_SKIM } else { 0 } | if self.blend_feather { BLEND_FEATHER } else { 0 } | if self.brush_skim { BRUSH_SKIM } else { 0 })?;
        // engine 4's section (versions 11 and 12): gloss and absorbency per pixel
        if version >= 11 {
            put_all(w, self.gloss.iter().copied())?;
            put_all(w, self.absorb.iter().copied())?;
        }
        // the thinner's section (version 9), last: the solvent per pixel
        if self.engine >= 3 {
            let n = self.f.w * self.f.h;
            if self.wet.solv.len() == n {
                put_all(w, self.wet.solv.iter().copied())?;
            } else {
                put_all(w, std::iter::repeat_n(0.0f32, n))?;
            }
        }
        // a raw canvas's soak section (version 10), last
        if let Some(s) = &self.soak {
            write_soak(w, s)?;
        }
        Ok(())
    }

    /// Read a canvas written by `write_state`; returns it and the header.
    pub fn read_state(r: &mut impl Read) -> io::Result<(Canvas, String)> {
        let (version, header) = read_magic_header(r)?;
        let mut u = [0usize; 10];
        for v in u.iter_mut() {
            *v = usize::try_from(get_u64(r)?).map_err(|_| bad("checkpoint frame is invalid"))?;
        }
        let [w, h, x0, y0, full_w, full_h, k0, k1, k2, k3] = u;
        let scale = get_f32(r)?;
        let mm_per_unit = get_f32(r)?;
        let linen = match get_u64(r)? {
            0 => None,
            _ => {
                let (a, b, c, d) = (get_f32(r)?, get_f32(r)?, get_f32(r)?, get_f32(r)?);
                Some(Linen { warp_per_cm: a, weft_per_cm: b, crown_um: c, slubs: d, seed: get_u64(r)? })
            }
        };
        let surf_gen = get_u64(r)?;
        let current = get_u64(r)? as u32;
        let dirty = match get_u64(r)? {
            0 => None,
            _ => {
                let mut d = [0usize; 4];
                for v in d.iter_mut() {
                    *v = usize::try_from(get_u64(r)?).map_err(|_| bad("checkpoint dirty box is invalid"))?;
                }
                Some((d[0], d[1], d[2], d[3]))
            }
        };
        // geometry is checked before anything is allocated or indexed: a
        // corrupt file is an error here, not a panic later
        let fits = |o: usize, n: usize, full: usize| o.checked_add(n).is_some_and(|e| e <= full);
        let n = w.checked_mul(h).filter(|&n| n > 0 && n <= 1 << 30);
        let Some(n) = n.filter(|_| fits(x0, w, full_w) && fits(y0, h, full_h)) else {
            return Err(bad("checkpoint frame is invalid"));
        };
        if !(k0 < k2 && k2 <= w && k1 < k3 && k3 <= h) {
            return Err(bad("checkpoint crop bounds are invalid"));
        }
        if let Some((a, b, c, d)) = dirty
            && !(a <= c && c <= w && b <= d && d <= h)
        {
            return Err(bad("checkpoint dirty box is invalid"));
        }
        if !(scale.is_finite() && scale > 0.0 && mm_per_unit.is_finite() && mm_per_unit > 0.0) {
            return Err(bad("checkpoint scale is invalid"));
        }
        if let Some(l) = linen
            && ![l.warp_per_cm, l.weft_per_cm, l.crown_um, l.slubs].iter().all(|v| v.is_finite())
        {
            return Err(bad("checkpoint linen is invalid"));
        }
        let f = Frame { w, h, scale, x0, y0, full_w, full_h };
        // a 1-pixel canvas, then every field replaced
        let mut c = Canvas::new_window(1, 1.0, [0.0; 3], None);
        c.f = f;
        c.keep = (k0, k1, k2, k3);
        c.mm_per_unit = mm_per_unit;
        c.linen = linen;
        c.surf_gen = surf_gen;
        c.base = None;
        let px = get_all(r, n * 3)?;
        c.px = px.as_chunks::<3>().0.to_vec();
        c.height = get_all(r, n)?;
        c.film = get_all(r, n)?;
        let mut wet = crate::wet::Wet::new(n);
        wet.vol = get_all(r, n)?;
        let lat = get_all(r, n * LAT)?;
        wet.lat = lat.as_chunks::<LAT>().0.to_vec();
        // (the pigment's packing is not written: paint naming no tubes')
        let (p, f, q) = (crate::wet::PACKED_OIL, crate::wet::DRAINED_FLOOR, crate::wet::OIL_VOLUME);
        wet.hide = if version >= 11 {
            get_all(r, n * 5)?.as_chunks::<5>().0.iter().map(|h| [h[0], h[1], h[2], h[3], h[4], p, f, 0.0, q, 0.0]).collect()
        } else {
            // (before engine 4: a tube paint's oil)
            get_all(r, n * 3)?.as_chunks::<3>().0.iter().map(|h| [h[0], h[1], h[2], 0.0, 1.0, p, f, 0.0, q, 0.0]).collect()
        };
        wet.stroke = get_all(r, n)?.into_iter().map(f32::to_bits).collect();
        wet.touched = get_all(r, n)?.into_iter().map(f32::to_bits).collect();
        wet.current = current;
        wet.dirty = dirty;
        let now = f64::from_bits(get_u64(r)?);
        let mark = get_u64(r)? as u32;
        let tacky = match get_u64(r)? {
            0 => None,
            _ => {
                let mut d = [0usize; 4];
                for v in d.iter_mut() {
                    *v = usize::try_from(get_u64(r)?).map_err(|_| bad("checkpoint tacky box is invalid"))?;
                }
                if !(d[0] <= d[2] && d[2] <= w && d[1] <= d[3] && d[3] <= h) {
                    return Err(bad("checkpoint tacky box is invalid"));
                }
                Some((d[0], d[1], d[2], d[3]))
            }
        };
        if !now.is_finite() {
            return Err(bad("checkpoint clock is invalid"));
        }
        let px = match get_u64(r)? {
            0 => Vec::new(),
            _ => get_all(r, n * 6)?.as_chunks::<6>().0.iter().map(|q| crate::drying::Px { cure: q[0], lev: q[1], seen: q[2], sub: q[3], srate: q[4], th: q[5] }).collect(),
        };
        wet.clock = crate::drying::Clock { now, px, mark, tacky };
        let ground_um = get_f32(r)?;
        c.ground_um = if ground_um.is_finite() { ground_um.max(0.0) } else { 0.0 };
        wet.cover = get_all(r, n)?;
        c.wet = wet;
        c.drawing = match get_u64(r)? {
            0 => None,
            1 => {
                let whole = full_w.checked_mul(full_h).filter(|&m| m <= 1 << 31).ok_or_else(|| bad("checkpoint frame is invalid"))?;
                let d = crate::graphite::Drawing::from_f32s(n, whole, |k| get_all(r, k))?;
                Some(Box::new(d.ok_or_else(|| bad("checkpoint drawing is invalid"))?))
            }
            _ => return Err(bad("checkpoint drawing flag is invalid")),
        };
        c.hand_slice = match get_u64(r)? {
            0 => None,
            1 => Some(get_f32(r)?).filter(|m| m.is_finite() && *m > 0.0),
            _ => return Err(bad("checkpoint hand time flag is invalid")),
        };
        let mut words = [0u64; 9];
        for v in words.iter_mut() {
            *v = get_u64(r)?;
        }
        c.tally = crate::tally::Tally::from_words(words).ok_or_else(|| bad("checkpoint hand-time ledger is invalid"))?;
        c.engine = if version == 7 {
            1
        } else {
            let word = get_u64(r)?;
            c.quarter_tiles = word & QUARTER_TILES != 0;
            c.adaptive_sweep = word & ADAPTIVE_SWEEP != 0;
            c.glaze_film = word & GLAZE_FILM != 0;
            c.rag_lift = word & RAG_LIFT != 0;
            c.knife_skim = word & KNIFE_SKIM != 0;
            c.blend_feather = word & BLEND_FEATHER != 0;
            c.brush_skim = word & BRUSH_SKIM != 0;
            c.liquid_film_levels = word & LIQUID_FILM_LEVELS != 0;
            match u32::try_from(word & !(LIQUID_FILM_LEVELS | QUARTER_TILES | ADAPTIVE_SWEEP | GLAZE_FILM | RAG_LIFT | KNIFE_SKIM | BLEND_FEATHER | BRUSH_SKIM)) {
                Ok(v) if crate::supported_engine(v) && (!c.liquid_film_levels || v >= 4) && (!(c.quarter_tiles || c.adaptive_sweep) || v == 5) && (!(c.glaze_film || c.rag_lift || c.knife_skim || c.blend_feather || c.brush_skim) || matches!(v, crate::engine7::ENGINE | crate::REACH_END)) => v,
                _ => return Err(bad("checkpoint engine version is invalid")),
            }
        };
        if version >= 11 {
            if c.engine < 4 {
                return Err(bad("checkpoint material format holds a canvas of an engine before 4"));
            }
            c.gloss = get_all(r, n)?;
            c.absorb = get_all(r, n)?;
            if !c.gloss.iter().chain(&c.absorb).all(|v| v.is_finite() && *v >= 0.0) {
                return Err(bad("checkpoint gloss or absorbency is invalid"));
            }
        } else {
            if c.engine >= 4 {
                return Err(bad("an engine-4 canvas in an older format: this easel saves engine 4 and later as PAINTC11 or PAINTC12"));
            }
            // (an older engine's canvas: an oil ground's gloss, nothing absorbent)
            c.gloss = vec![crate::canvas::OIL_GROUND_GLOSS; n];
            c.absorb = vec![0.0; n];
        }
        c.absorb_any = c.absorb.iter().any(|&a| a > 0.0);
        match (version >= 9, c.engine >= 3) {
            (false, true) => return Err(bad(OLD_ENGINE3)),
            (true, false) => return Err(bad(&format!("checkpoint version {version} holds an engine-1 or engine-2 canvas"))),
            (true, true) => {
                let s = get_all(r, n)?;
                if !s.iter().all(|v| v.is_finite() && *v >= 0.0) {
                    return Err(bad("checkpoint solvent is invalid"));
                }
                // (an engine-3 canvas's buffers, as `surf` allocates them)
                c.wet.ensure_solvent();
                c.wet.solv = s;
            }
            (false, false) => {}
        }
        // raw formats go on with the soak section, which ends the file
        if version == 10 || version == 12 {
            if !c.f.is_whole() {
                return Err(bad("checkpoint soak is on a crop render"));
            }
            c.soak = Some(Box::new(read_soak(r, n)?));
            if r.read(&mut [0u8; 1])? != 0 {
                return Err(bad("checkpoint has trailing data"));
            }
        }
        if version == 7 && r.read(&mut [0u8; 1])? != 0 {
            return Err(bad("version-7 checkpoint has trailing data"));
        }
        Ok((c, header))
    }
}
