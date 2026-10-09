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
//! soak section after the solvent. Versions 13 and 14 are engine 6's:
//! versions 11 and 12 with five more properties to each pixel of wet paint
//! (its packed oil, its drained floor, the share of the film packed on an
//! absorbent ground, its tube paint's oil by volume and its wax,
//! `crate::wet::Prop`), and after the solvent (before
//! a raw canvas's soak section) the paper, if the support is paper, with
//! its micro-roughness per pixel; an engine-7 canvas then gives the sheet of
//! paper lying on it, if any (where it lies and the pastel on it). After the header the
//! writer stores, in order: the frame and crop window, the scale and mm per
//! unit, the linen (if any), the surface generation, the stroke counter and
//! dirty box, then per pixel the color, relief, film, wet volume, pigment
//! mix, hiding and the ids of the last stroke to lay and to touch paint;
//! the clock with its tacky box and each pixel's drying state; the ground
//! thickness (for craquelure fitted to the ground); each wet pixel's paint
//! coverage (pointed-tip marks); the drawing (`graphite::Drawing`), if any:
//! every cell of the deposit (coverage, flake reflectance, lift, fixed
//! floor, film when drawn; with pastel, flag 2, the reflectance in color and
//! the tooth's fill) and the whole-canvas guide with its fixed floor;
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
/// Engine 3, with the thinner's section (`crate::thinner`).
const MAGIC9: &[u8; 8] = b"PAINTCK9";
/// A raw canvas: version 9, then its soak section (`crate::soak`).
const MAGIC10: &[u8; 8] = b"PAINTC10";
/// Engine 4 material properties, gloss and absorbency.
const MAGIC11: &[u8; 8] = b"PAINTC11";
/// Engine 4 materials followed by the raw canvas's soak section.
const MAGIC12: &[u8; 8] = b"PAINTC12";
/// Engine 6: version 11 with each wet pixel's packing (`crate::wet::Prop`).
const MAGIC13: &[u8; 8] = b"PAINTC13";
/// Engine 6 on a raw canvas: version 12 with each wet pixel's packing.
const MAGIC14: &[u8; 8] = b"PAINTC14";
/// Marks the soak section ("SOAK"), and its version.
const SOAK_MARK: u64 = 0x4b414f53;
const SOAK_V: u64 = 1;

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

/// The format version (8, 9 or 10) and the header. An engine-3 canvas in a
/// PAINTCK8 file whose header says so (`engine=3`, as an easel save's
/// does) is refused here, before anything else is read.
fn read_magic_header(r: &mut impl Read) -> io::Result<(u32, String)> {
    let mut m = [0u8; 8];
    r.read_exact(&mut m)?;
    let version = match &m {
        m if m == MAGIC => 8,
        m if m == MAGIC9 => 9,
        m if m == MAGIC10 => 10,
        m if m == MAGIC11 => 11,
        m if m == MAGIC12 => 12,
        m if m == MAGIC13 => 13,
        m if m == MAGIC14 => 14,
        _ => return Err(bad("not a canvas checkpoint (or an older format)")),
    };
    let n = get_u64(r)? as usize;
    if n > 1 << 20 {
        return Err(bad("checkpoint header too long"));
    }
    let mut h = vec![0u8; n];
    r.read_exact(&mut h)?;
    let head = String::from_utf8(h).map_err(|_| bad("checkpoint header is not UTF-8"))?;
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
            (6.., true) => 14,
            (6.., false) => 13,
            (4.., true) => 12,
            (4.., false) => 11,
            _ => 8,
        };
        w.write_all(match version {
            14 => MAGIC14,
            13 => MAGIC13,
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
        if version >= 13 {
            put_all(w, wt.hide.iter().flat_map(|h| *h))?;
        } else if version >= 11 {
            // (five properties: engines 4 and 5 have no ground's draw)
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
                // 2: a drawing with pastel, its cells in color; 3: engine 6's pastel,
        // with what lies in the tooth (graphite.rs)
                put_u64(w, d.layout())?;
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
        put_u64(w, self.engine as u64)?;
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
        // engine 6's section (versions 13 and 14): the paper (if the support
        // is paper) and its micro-roughness per pixel
        if self.engine >= 6 {
            match self.paper {
                None => put_u64(w, 0)?,
                Some(p) => {
                    put_u64(w, 1)?;
                    for v in [p.grammage, p.fibre_mm, p.fibre_um, p.thick_um, p.coarseness, p.porosity, p.floc, p.floc_mm, p.press, p.calender, p.absorbent, p.z_mpa] {
                        put_f32(w, v)?;
                    }
                    put_u64(w, p.seed)?;
                    match p.felt {
                        None => put_u64(w, 0)?,
                        Some(f) => {
                            put_u64(w, 1)?;
                            put_f32(w, f.cell_mm)?;
                            put_f32(w, f.depth_um)?;
                        }
                    }
                    match p.laid {
                        None => put_u64(w, 0)?,
                        Some(l) => {
                            put_u64(w, 1)?;
                            put_f32(w, l.per_cm)?;
                            put_f32(w, l.chain_mm)?;
                            put_f32(w, l.deficit)?;
                        }
                    }
                    put_all(w, self.micro.iter().copied())?;
                }
            }
        }
        // engine 7's section: a sheet of paper lying on the picture (sheet.rs),
        // where it lies and the pastel caught on it
        if self.engine >= 7 {
            match &self.sheet {
                None => put_u64(w, 0)?,
                Some(s) => {
                    put_u64(w, 1)?;
                    for v in [s.caliper_um, s.micro_um, s.tone[0], s.tone[1], s.tone[2]] {
                        put_f32(w, v)?;
                    }
                    put_all(w, s.over.iter().map(|&o| if o { 1.0 } else { 0.0 }))?;
                    put_all(w, s.a.iter().copied())?;
                    put_all(w, s.r.iter().flat_map(|c| *c))?;
                }
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
        let (p, f, q) = (crate::wet::PACKED_OIL, crate::wet::DRAINED_FLOOR, crate::wet::OIL_VOLUME);
        wet.hide = if version >= 13 {
            let h = get_all(r, n * 10)?;
            // (engine 6's five: the ground's drain would carry a non-finite one on)
            if !h.as_chunks::<10>().0.iter().all(|p| p[5..].iter().all(|v| v.is_finite())) {
                return Err(bad("checkpoint paint packing is invalid"));
            }
            h.as_chunks::<10>().0.to_vec()
        } else if version >= 11 {
            // (engines 4 and 5: no ground's draw)
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
            v @ (1..=3) => {
                let whole = full_w.checked_mul(full_h).filter(|&m| m <= 1 << 31).ok_or_else(|| bad("checkpoint frame is invalid"))?;
                let d = crate::graphite::Drawing::from_f32s(n, whole, v, |k| get_all(r, k))?;
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
        c.engine = match get_u64(r)? {
            v if (1..=crate::ENGINE as u64).contains(&v) => v as u32,
            _ => return Err(bad("checkpoint engine version is invalid")),
        };
        if version >= 11 {
            if c.engine < 4 {
                return Err(bad("checkpoint material format holds a canvas of an engine before 4"));
            }
            if (version >= 13) != (c.engine >= 6) {
                return Err(bad("checkpoint format and engine disagree: engine 6 and later save as PAINTC13 or PAINTC14, engines 4 and 5 as PAINTC11 or PAINTC12"));
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
        // engine 6's section: the paper
        if c.engine >= 6 {
            match get_u64(r)? {
                0 => {}
                1 => {
                    let mut v = [0.0f32; 12];
                    for x in v.iter_mut() {
                        *x = get_f32(r)?;
                    }
                    if !v.iter().all(|x| x.is_finite()) {
                        return Err(bad("checkpoint paper is invalid"));
                    }
                    let seed = get_u64(r)?;
                    let felt = match get_u64(r)? {
                        0 => None,
                        1 => Some(crate::paper::Felt { cell_mm: get_f32(r)?, depth_um: get_f32(r)? }),
                        _ => return Err(bad("checkpoint paper felt flag is invalid")),
                    };
                    if felt.is_some_and(|f| !(f.cell_mm.is_finite() && f.depth_um.is_finite())) {
                        return Err(bad("checkpoint paper felt is invalid"));
                    }
                    let laid = match get_u64(r)? {
                        0 => None,
                        1 => Some(crate::paper::Laid { per_cm: get_f32(r)?, chain_mm: get_f32(r)?, deficit: get_f32(r)? }),
                        _ => return Err(bad("checkpoint paper laid flag is invalid")),
                    };
                    if laid.is_some_and(|l| !(l.per_cm.is_finite() && l.chain_mm.is_finite() && l.deficit.is_finite())) {
                        return Err(bad("checkpoint paper laid is invalid"));
                    }
                    c.paper = Some(crate::paper::Paper {
                        grammage: v[0], fibre_mm: v[1], fibre_um: v[2], thick_um: v[3], coarseness: v[4], porosity: v[5],
                        floc: v[6], floc_mm: v[7], press: v[8], calender: v[9], absorbent: v[10], z_mpa: v[11], felt, laid, seed,
                    });
                    c.micro = get_all(r, n)?;
                    if !c.micro.iter().all(|x| x.is_finite() && *x >= 0.0) {
                        return Err(bad("checkpoint micro-roughness is invalid"));
                    }
                }
                _ => return Err(bad("checkpoint paper flag is invalid")),
            }
        }
        // engine 7's section: the sheet
        if c.engine >= 7 {
            match get_u64(r)? {
                0 => {}
                1 => {
                    let mut v = [0.0f32; 5];
                    for x in v.iter_mut() {
                        *x = get_f32(r)?;
                    }
                    let over = get_all(r, n)?;
                    if !over.iter().all(|&o| o == 0.0 || o == 1.0) {
                        return Err(bad("checkpoint sheet mask is invalid"));
                    }
                    let over: Vec<bool> = over.into_iter().map(|o| o > 0.5).collect();
                    let a = get_all(r, n)?;
                    let rgb = get_all(r, n * 3)?;
                    if !v.iter().chain(&a).chain(&rgb).all(|x| x.is_finite()) {
                        return Err(bad("checkpoint sheet is invalid"));
                    }
                    let r = rgb.as_chunks::<3>().0.to_vec();
                    c.sheet = Some(crate::sheet::Sheet { over, caliper_um: v[0], micro_um: v[1], tone: [v[2], v[3], v[4]], a, r });
                }
                _ => return Err(bad("checkpoint sheet flag is invalid")),
            }
        }
        // raw formats go on with the soak section, which ends the file
        if version == 10 || version == 12 || version == 14 {
            if !c.f.is_whole() {
                return Err(bad("checkpoint soak is on a crop render"));
            }
            c.soak = Some(Box::new(read_soak(r, n)?));
            if r.read(&mut [0u8; 1])? != 0 {
                return Err(bad("checkpoint has trailing data"));
            }
        }
        Ok((c, header))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // offsets in a checkpoint with an empty header and no linen
    const W: usize = 16;
    const X0: usize = 32;
    const KEEP: usize = 64;
    const SCALE: usize = 96;
    const MM: usize = 100;
    const DIRTY: usize = 128;

    fn set(b: &mut [u8], pos: usize, v: u64) {
        b[pos..pos + 8].copy_from_slice(&v.to_le_bytes());
    }

    fn load(b: Vec<u8>) -> io::Result<Canvas> {
        Canvas::read_state(&mut Cursor::new(b)).map(|(c, _)| c)
    }

    fn rejected(b: Vec<u8>, what: &str) {
        match load(b) {
            Ok(_) => panic!("{what}: accepted"),
            Err(e) => assert_eq!(e.kind(), io::ErrorKind::InvalidData, "{what}: {e}"),
        }
    }

    fn original() -> Vec<u8> {
        let c = Canvas::new_window(2, 1.0, [0.1; 3], None);
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        b
    }

    /// An engine-6 canvas is written as version 13 and keeps all ten
    /// properties of its wet paint (its packed and drained oil, the share
    /// packed on its ground, its tube oil by volume and its wax); an engine-4 canvas is written as version
    /// 11 and keeps its gloss, its ground's absorbency and the oil in its
    /// wet paint, read with a paint naming no tubes' packing and nothing
    /// packed; an older engine's canvas is written as before (version 8 or
    /// 9, three properties, no gloss) and read with an oil ground's gloss and
    /// a tube paint's oil.
    #[test]
    fn engine_4_and_6_have_their_own_formats_and_older_engines_keep_theirs() {
        let canvas = |engine: u32| {
            let mut c = Canvas::new_window(2, 1.0, [0.1; 3], None).with_engine(engine);
            c.gloss.iter_mut().for_each(|g| *g = 0.9);
            c.absorb.iter_mut().for_each(|a| *a = 0.3);
            c.wet.vol[0] = 1.5;
            c.wet.hide[0] = [0.7, 0.6, 1.2, 0.4, 0.5, 0.45, 0.3, 0.25, 0.6, 0.2];
            let mut b = Vec::new();
            c.write_state(&mut b, "").unwrap();
            b
        };
        let b = canvas(crate::ENGINE);
        assert_eq!(&b[..8], b"PAINTC13");
        let d = load(b).unwrap();
        assert_eq!((d.gloss[0], d.absorb[0], d.absorb_any, d.wet.hide[0]), (0.9, 0.3, true, [0.7, 0.6, 1.2, 0.4, 0.5, 0.45, 0.3, 0.25, 0.6, 0.2]));
        let (p, f, q) = (crate::wet::PACKED_OIL, crate::wet::DRAINED_FLOOR, crate::wet::OIL_VOLUME);
        for engine in [4, 5] {
            let b = canvas(engine);
            assert_eq!(&b[..8], b"PAINTC11");
            let d = load(b).unwrap();
            assert_eq!((d.gloss[0], d.absorb[0], d.absorb_any, d.wet.hide[0]), (0.9, 0.3, true, [0.7, 0.6, 1.2, 0.4, 0.5, p, f, 0.0, q, 0.0]), "engine {engine}");
        }
        for (engine, magic) in [(2, b"PAINTCK8"), (3, b"PAINTCK9")] {
            let b = canvas(engine);
            assert_eq!(&b[..8], magic);
            let d = load(b).unwrap();
            assert_eq!((d.gloss[0], d.absorb[0], d.absorb_any, d.wet.hide[0]), (crate::canvas::OIL_GROUND_GLOSS, 0.0, false, [0.7, 0.6, 1.2, 0.0, 1.0, p, f, 0.0, q, 0.0]), "engine {engine}");
        }
    }

    #[test]
    fn round_trip() {
        let mut c = load(original()).unwrap();
        c.wet.touch(0, 0, 2, 1);
        let mut b = Vec::new();
        c.write_state(&mut b, "x=1\n").unwrap();
        let (d, h) = Canvas::read_state(&mut Cursor::new(b)).unwrap();
        assert_eq!((h.as_str(), d.keep, d.wet.dirty), ("x=1\n", (0, 0, 2, 2), Some((0, 0, 2, 1))));
    }

    /// Engine 7: a sheet of paper laid on the picture survives a checkpoint,
    /// with the pastel caught on it; none laid, none read back.
    #[test]
    fn a_laid_sheet_survives_a_checkpoint() {
        use crate::pastel::{Pose, Stick, StrokePoint};
        let mut c = Canvas::new_window(240, 1.5, [0.5; 3], None).with_size_mm(120.0).with_engine(7).with_paper(crate::paper::Paper::drawing(2));
        let back = |c: &Canvas| {
            let mut b = Vec::new();
            c.write_state(&mut b, "").unwrap();
            Canvas::read_state(&mut Cursor::new(b)).unwrap().0
        };
        assert!(back(&c).sheet.is_none());
        let m = crate::mask::Mask::from_fn(c.frame(), |x, _| if x < 500.0 { 1.0 } else { 0.0 });
        c.lay_sheet(&m, 100.0, 5.0, [0.8, 0.78, 0.7]).unwrap();
        let mut st = Stick::round([0.7, 0.2, 0.1], 0.8, 12.0);
        let pose = Pose { force: 2.0, alt: 1.0, az: 0.8, roll: 0.0 };
        c.stick_stroke(&mut st, &[StrokePoint { x: 100.0, y: 300.0, pose, speed: 60.0 }, StrokePoint { x: 900.0, y: 300.0, pose, speed: 60.0 }]);
        let r = back(&c);
        assert!(r.sheet.is_some() && r.sheet == c.sheet && r.pixels() == c.pixels());
        assert!(c.sheet.as_ref().unwrap().a.iter().any(|&a| a > 0.0), "the stroke caught on the sheet");
    }

    /// Engine 6: a sheet of paper with stick pastel in its tooth survives a
    /// checkpoint bit for bit (the paper, its micro-roughness, the loose and
    /// bound pastel), and goes on taking pastel as it would have.
    #[test]
    fn paper_and_stick_pastel_survive_a_checkpoint() {
        use crate::pastel::{Pose, Stick, StrokePoint};
        let mut c = Canvas::new_window(240, 1.5, [0.5; 3], None).with_size_mm(120.0).with_engine(6).with_paper(crate::paper::Paper::drawing(2));
        let mut st = Stick::round([0.7, 0.2, 0.1], 0.8, 12.0);
        let pose = Pose { force: 2.0, alt: 1.0, az: 0.8, roll: 0.0 };
        let line = |y: f32| [StrokePoint { x: 100.0, y, pose, speed: 60.0 }, StrokePoint { x: 900.0, y, pose, speed: 60.0 }];
        c.stick_stroke(&mut st, &line(200.0));
        c.fix_pastel(None, 0.15);
        c.stick_stroke(&mut st, &line(260.0));
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        let (mut r, _) = Canvas::read_state(&mut Cursor::new(b)).unwrap();
        assert!(r.pixels() == c.pixels() && r.height == c.height && r.micro == c.micro && r.paper == c.paper);
        let mut st2 = st.clone();
        c.stick_stroke(&mut st, &line(300.0));
        r.stick_stroke(&mut st2, &line(300.0));
        assert!(r.pixels() == c.pixels() && st == st2);
    }

    /// A pastel drawing (in color, with its tooth) survives a checkpoint bit
    /// for bit, and goes on taking pastel as it would have.
    #[test]
    fn pastel_survives_a_checkpoint() {
        use crate::graphite::hand_line;
        use crate::Lead;
        let mut c = Canvas::new_window(300, 1.5, [0.8; 3], None).with_size_mm(440.0);
        let red = Lead::pastel([0.7, 0.1, 0.05], 0.8);
        let blue = Lead::pastel([0.05, 0.1, 0.6], 0.6);
        c.draw(&red.side(10.0), &hand_line(&[(100.0, 100.0), (900.0, 100.0)], &[0.6], false, true, 0.0, 5), 0.0, 9);
        c.draw(&blue, &hand_line(&[(100.0, 100.0), (900.0, 110.0)], &[0.8], false, true, 0.0, 6), 0.0, 10);
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        let (mut r, _) = Canvas::read_state(&mut Cursor::new(b)).unwrap();
        assert!(r.drawing_view() == c.drawing_view() && r.pixels() == c.pixels());
        for k in [&mut c, &mut r] {
            k.draw(&blue.side(10.0), &hand_line(&[(100.0, 104.0), (900.0, 104.0)], &[0.5], false, true, 0.0, 7), 0.0, 11);
        }
        assert!(r.pixels() == c.pixels());
    }

    /// A resumed canvas keeps its drawing: the deposit (so the
    /// eraser and fixative still act on it and `drawing_mask` has it) and
    /// the guide, bit for bit; erasing, redrawing and painting into the
    /// drawing after resuming does what it does without the checkpoint.
    #[test]
    fn drawing_survives_a_checkpoint() {
        use crate::bristle::Tool;
        use crate::graphite::hand_line;
        use crate::handling::Handling;
        use crate::{Lead, Mask};
        let mut c = Canvas::new_window(300, 1.5, [0.8; 3], None).with_size_mm(440.0);
        let lead = Lead::pencil("2B").unwrap();
        c.draw(&lead, &hand_line(&[(100.0, 100.0), (900.0, 100.0)], &[0.8], false, true, 0.0, 5), 0.0, 9);
        c.draw(&lead, &hand_line(&[(100.0, 300.0), (900.0, 300.0)], &[0.8], false, true, 0.0, 6), 0.0, 10);
        let f = c.frame();
        c.fix_drawing(Some(&Mask::from_fn(f, |_, y| if y > 200.0 { 1.0 } else { 0.0 })));
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        let (mut r, _) = Canvas::read_state(&mut Cursor::new(b)).unwrap();
        assert!(r.has_drawing() && r.drawing_mask().data == c.drawing_mask().data && r.drawing_guide().data == c.drawing_guide().data);
        assert!(r.drawing_view() == c.drawing_view());
        let hd = Handling::new(Tool::round_sable(4.0)).length(10.0, 20.0).coverage(2.0).color(|_, _| [0.08, 0.12, 0.18]).hug(false).clip(false).fill(false);
        for k in [&mut c, &mut r] {
            let snap = k.pixels().to_vec();
            k.erase(&Mask::full(f), 1.0);
            assert!(k.pixels() != &snap[..], "the loose line lifts");
            k.draw(&lead, &hand_line(&[(100.0, 200.0), (900.0, 200.0)], &[0.6], false, true, 0.0, 7), 0.0, 11);
            let g = k.drawing_guide().dilate(3.0);
            k.work(&g, &hd, 77);
        }
        assert!(r.pixels() == c.pixels(), "resumed painting differs");
        assert!(r.drawing_mask().data == c.drawing_mask().data && r.drawing_guide().data == c.drawing_guide().data);
        // a corrupt drawing is refused
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        let at = b.len() - 4 * (300 * 200 + 1) - 4;
        b[at..at + 4].copy_from_slice(&f32::NAN.to_le_bytes());
        rejected(b, "nan in the guide");
    }

    /// Corrupt geometry is an error when loading, not a panic later (keep
    /// past the buffer, an overflowing frame).
    #[test]
    fn malformed_geometry_is_invalid_data() {
        let o = original();
        let mut b = o.clone();
        set(&mut b, KEEP + 16, 3);
        rejected(b, "keep x1 past the buffer");
        let mut b = o.clone();
        set(&mut b, KEEP, 2);
        rejected(b, "empty keep");
        let mut b = o.clone();
        set(&mut b, W, u64::MAX);
        set(&mut b, X0, 1);
        rejected(b, "overflowing frame");
        let mut b = o.clone();
        set(&mut b, W, 1 << 32);
        set(&mut b, W + 8, 1 << 32);
        rejected(b, "overflowing area");
        for (v, what) in [(f32::NAN, "nan"), (0.0, "zero"), (-1.0, "negative"), (f32::INFINITY, "infinite")] {
            for (pos, field) in [(SCALE, "scale"), (MM, "mm per unit")] {
                let mut b = o.clone();
                b[pos..pos + 4].copy_from_slice(&v.to_le_bytes());
                rejected(b, &format!("{what} {field}"));
            }
        }
        for rect in [[0u64, 0, 3, 3], [1, 0, 0, 2], [0, 0, 2, u64::MAX]] {
            let mut b = o.clone();
            set(&mut b, DIRTY, 1);
            let r: Vec<u8> = rect.iter().flat_map(|v| v.to_le_bytes()).collect();
            b.splice(DIRTY + 8..DIRTY + 8, r);
            rejected(b, &format!("dirty {rect:?}"));
        }
        // a valid dirty box still loads
        let mut b = o.clone();
        set(&mut b, DIRTY, 1);
        let r: Vec<u8> = [0u64, 0, 2, 2].iter().flat_map(|v| v.to_le_bytes()).collect();
        b.splice(DIRTY + 8..DIRTY + 8, r);
        assert_eq!(load(b).unwrap().wet.dirty, Some((0, 0, 2, 2)));
    }
}
