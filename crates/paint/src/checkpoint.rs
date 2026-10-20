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
//! floor, film when drawn; with a pastel stick's pastel, flag 3, the
//! reflectance in color, the pastel in the tooth, loose and bound, and the
//! non-pastel coverage and its fixed floor; flag 2 lacks those last two) and
//! the whole-canvas guide with its fixed floor;
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
//!
//! A canvas with the drain switch on (`Canvas::with_drain`, engine 4's
//! materials and later) is version 14 (`PAINTC14`): version 11's bytes after
//! the magic, then, after the solvent, the drain section: a `DRAN` mark and
//! the section's own version (1, `DRAIN_V`), a word saying whether a soak
//! section follows (1: a raw canvas, whose soak section then ends the file
//! as in version 12; 0: the drain section is the file's last), and the rest
//! of each buffer pixel's wet paint (`crate::wet::Prop`'s last five: its
//! packed oil, drained floor, the share of the film packed on an absorbent
//! ground, its tube paint's oil by volume and its wax), five f32 each, row
//! major. The section is there exactly when the switch is on, and so says
//! it is: a canvas without it is written as before, and a reader that
//! doesn't know version 14 refuses a draining canvas instead of loading it
//! without its packing. (Version 13 is left to the paper's section.)
//! A canvas on a sheet of paper (`crate::paper`) is version 13
//! (`PAINTC13`): version 11's bytes after the magic, then the paper's
//! section, which is the file's last: a `PAPR` mark and the section's own
//! version (1, `PAPER_V`), the sheet's make (grammage, fiber length, width
//! and thickness, coarseness, porosity, flocs and their size, pressing,
//! calendering, absorbency and z modulus, one f32 each), its seed, its felt
//! and laid wires (each a 0/1 flag, then its numbers), and the pores' mean
//! depth per buffer pixel (`Canvas::micro`). The section is there exactly
//! when the canvas has paper, whatever its engine: a canvas without paper
//! is written as before, and a reader that doesn't know version 13 refuses
//! a sheet instead of loading it as linen. A sheet of paper is engine 4's
//! or later (its pores take oil as an absorbent ground does) and never a
//! raw canvas.
//!
//! A canvas with a sheet of paper laid on the picture (`crate::sheet`, a
//! mask) is version 15 (`PAINTC15`): the bytes of the version it would
//! otherwise be (11, 12 or 13: its soak or paper section, if any, told by
//! its mark), then the sheet's section, which is the file's last: a `SHET`
//! mark and the section's own version (1, `SHEET_V`), its caliper, pore
//! depth and tone (five f32), where it lies (one f32 per buffer pixel, 0 or
//! 1), and the pastel caught on it (coverage, then color, per pixel). It is
//! there exactly when a sheet lies on the picture: every other canvas is
//! written as before, and a reader that doesn't know version 15 refuses the
//! file instead of loading the picture without its mask. (The easel lifts
//! a sheet within the chunk that laid it, so its chunk checkpoints never
//! hold one.)

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
/// Version 11 with the drain section (`Canvas::with_drain`), and a raw
/// canvas's soak section after it if there is one.
const MAGIC14: &[u8; 8] = b"PAINTC14";
const MAGIC16: &[u8; 8] = b"PAINTC16";
const MAGIC17: &[u8; 8] = b"PAINTC17";
const MAGIC18: &[u8; 8] = b"PAINTC18";
/// Version 11 followed by the paper's section (`crate::paper`).
const MAGIC13: &[u8; 8] = b"PAINTC13";
/// Marks the paper's section ("PAPR"), and its version.
const PAPER_MARK: u64 = 0x52504150;
const PAPER_V: u64 = 1;
/// Version 11, 12 or 13 followed by the sheet's section (`crate::sheet`).
const MAGIC15: &[u8; 8] = b"PAINTC15";
/// Marks the sheet's section ("SHET"), and its version.
const SHEET_MARK: u64 = 0x54454853;
const SHEET_V: u64 = 1;
/// Marks the soak section ("SOAK"), and its version.
const SOAK_MARK: u64 = 0x4b414f53;
const SOAK_V: u64 = 1;
/// Marks the drain section ("DRAN"), and its version.
const DRAIN_MARK: u64 = 0x4e415244;
const DRAIN_V: u64 = 1;
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
    read_soak_body(r, n)
}

/// The soak section after its mark.
fn read_soak_body(r: &mut impl Read, n: usize) -> io::Result<crate::soak::Soak> {
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

fn write_paper(w: &mut impl Write, p: &crate::paper::Paper, micro: &[f32]) -> io::Result<()> {
    put_u64(w, PAPER_MARK)?;
    put_u64(w, PAPER_V)?;
    for v in [p.grammage, p.fiber_mm, p.fiber_um, p.thick_um, p.coarseness, p.porosity, p.floc, p.floc_mm, p.press, p.calender, p.absorbent, p.z_mpa] {
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
    put_all(w, micro.iter().copied())
}

fn read_paper(r: &mut impl Read, n: usize) -> io::Result<(crate::paper::Paper, Vec<f32>)> {
    if get_u64(r)? != PAPER_MARK {
        return Err(bad("checkpoint paper mark is wrong"));
    }
    read_paper_body(r, n)
}

/// The paper's section after its mark.
fn read_paper_body(r: &mut impl Read, n: usize) -> io::Result<(crate::paper::Paper, Vec<f32>)> {
    use crate::paper::{Felt, Laid, Paper};
    match get_u64(r)? {
        PAPER_V => {}
        v => return Err(bad(&format!("checkpoint paper section is version {v}; this easel reads version {PAPER_V} only"))),
    }
    let mut v = [0.0f32; 12];
    for x in v.iter_mut() {
        *x = get_f32(r)?;
    }
    let seed = get_u64(r)?;
    let felt = match get_u64(r)? {
        0 => None,
        1 => Some(Felt { cell_mm: get_f32(r)?, depth_um: get_f32(r)? }),
        _ => return Err(bad("checkpoint paper felt flag is invalid")),
    };
    let laid = match get_u64(r)? {
        0 => None,
        1 => Some(Laid { per_cm: get_f32(r)?, chain_mm: get_f32(r)?, deficit: get_f32(r)? }),
        _ => return Err(bad("checkpoint paper laid flag is invalid")),
    };
    let micro = get_all(r, n)?;
    // (a corrupt file is an error here, not a panic or NaN pixels later)
    let felt_ok = felt.is_none_or(|f| f.cell_mm.is_finite() && f.depth_um.is_finite());
    let laid_ok = laid.is_none_or(|l| [l.per_cm, l.chain_mm, l.deficit].iter().all(|x| x.is_finite()));
    if !(v.iter().all(|x| x.is_finite()) && felt_ok && laid_ok && micro.iter().all(|x| x.is_finite() && *x >= 0.0)) {
        return Err(bad("checkpoint paper is invalid"));
    }
    let p = Paper {
        grammage: v[0], fiber_mm: v[1], fiber_um: v[2], thick_um: v[3], coarseness: v[4], porosity: v[5],
        floc: v[6], floc_mm: v[7], press: v[8], calender: v[9], absorbent: v[10], z_mpa: v[11], felt, laid, seed,
    };
    Ok((p, micro))
}

fn write_sheet(w: &mut impl Write, s: &crate::sheet::Sheet) -> io::Result<()> {
    put_u64(w, SHEET_MARK)?;
    put_u64(w, SHEET_V)?;
    for v in [s.caliper_um, s.micro_um, s.tone[0], s.tone[1], s.tone[2]] {
        put_f32(w, v)?;
    }
    put_all(w, s.over.iter().map(|&o| if o { 1.0 } else { 0.0 }))?;
    put_all(w, s.a.iter().copied())?;
    put_all(w, s.r.iter().flat_map(|c| *c))
}

/// The sheet's section after its mark.
fn read_sheet_body(r: &mut impl Read, n: usize) -> io::Result<crate::sheet::Sheet> {
    match get_u64(r)? {
        SHEET_V => {}
        v => return Err(bad(&format!("checkpoint sheet section is version {v}; this easel reads version {SHEET_V} only"))),
    }
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
    Ok(crate::sheet::Sheet { over, caliper_um: v[0], micro_um: v[1], tone: [v[2], v[3], v[4]], a, r })
}

/// Read just the header of a checkpoint (to validate it before loading).
pub fn read_header(r: &mut impl Read) -> io::Result<String> {
    read_magic_header(r).map(|(_, h)| h)
}

/// The format version (7–12 or 14) and the header. An engine-3 canvas in a
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
        m if m == MAGIC14 => 14,
        m if m == MAGIC16 => 16,
        m if m == MAGIC17 => 17,
        m if m == MAGIC18 => 18,
        m if m == MAGIC13 => 13,
        m if m == MAGIC15 => 15,
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
        let pigments = self.wet.lat.iter().any(|p| !p.parts.is_empty());
        // (a raw canvas is engine 3's: version 10 holds version 9's bytes)
        if self.soak.is_some() && self.engine < 3 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "a raw canvas is engine 3's: this one is set to an older engine"));
        }
        // (the drain switch: version 14 holds version 11's bytes)
        if self.drain && self.engine < 4 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "the drain switch drains engine 4's oil: this canvas is set to an older engine"));
        }
        // (a sheet of paper: version 13 holds version 11's bytes)
        if self.paper.is_some() && (self.engine < 4 || self.soak.is_some() || self.micro.len() != self.f.w * self.f.h) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "a sheet of paper is engine 4's or later, on no raw cloth, with its pores laid (`with_paper`)"));
        }
        // (a sheet laid on the picture: version 15 holds version 11's bytes and on)
        if self.sheet.is_some() && self.engine < 4 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "a sheet laid on the picture is engine 4's or later"));
        }
        let version = match (self.engine, self.soak.is_some()) {
            _ if self.drain && self.sheet.is_some() => 17,
            _ if self.sheet.is_some() => 15,
            _ if self.drain && self.paper.is_some() => 16,
            _ if self.drain => 14,
            _ if self.paper.is_some() => 13,
            (3, true) => 10,
            (3, false) => 9,
            (4.., true) => 12,
            (4.., false) => 11,
            _ => 8,
        };
        let magic = match version {
            14 => MAGIC14,
            16 => MAGIC16,
            17 => MAGIC17,
            15 => MAGIC15,
            13 => MAGIC13,
            12 => MAGIC12,
            11 => MAGIC11,
            10 => MAGIC10,
            9 => MAGIC9,
            _ => MAGIC,
        };
        w.write_all(if pigments { MAGIC18 } else { magic })?;
        put_u64(w, header.len() as u64)?;
        w.write_all(header.as_bytes())?;
        if pigments { put_u64(w, version)?; }
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
        put_all(w, wt.lat.iter().flat_map(|l| l.color))?;
        if version >= 11 {
            // (engine 4's five properties: the pigment's packing, `Prop`'s
            // last five, is the drain section's, version 14; without it, it
            // reads back as paint naming no tubes)
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
                // 1: graphite and chalk; 3: with pastel, its colored cells,
                // what lies in the tooth and non-pastel coverage (graphite.rs)
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
        // the drain section (version 14): whether a soak section follows,
        // and the pigment's packing per pixel
        if matches!(version, 14 | 16 | 17) {
            put_u64(w, DRAIN_MARK)?;
            put_u64(w, DRAIN_V)?;
            put_u64(w, u64::from(self.soak.is_some()))?;
            put_all(w, wt.hide.iter().flat_map(|h| [h[5], h[6], h[7], h[8], h[9]]))?;
        }
        // a raw canvas's soak section (version 10), last
        if let Some(s) = &self.soak {
            write_soak(w, s)?;
        }
        // a sheet of paper's section (version 13), last
        if let Some(p) = &self.paper {
            write_paper(w, p, &self.micro)?;
        }
        // a sheet laid on the picture (version 15), last
        if let Some(s) = &self.sheet {
            write_sheet(w, s)?;
        }
        if pigments {
            for pixel in &self.wet.lat {
                put_u64(w, pixel.parts.len() as u64)?;
                for &(i, amount) in &pixel.parts {
                    put_u64(w, i as u64)?;
                    put_f32(w, amount)?;
                }
            }
        }
        Ok(())
    }

    /// Read a canvas written by `write_state`; returns it and the header.
    pub fn read_state(r: &mut impl Read) -> io::Result<(Canvas, String)> {
        let (version, header) = read_magic_header(r)?;
        let pigments = version == 18;
        let version = if pigments {
            let version = get_u64(r)?;
            if !(8..=17).contains(&version) { return Err(bad("checkpoint base format is invalid")); }
            version as u32
        } else { version };
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
        wet.lat = lat.as_chunks::<LAT>().0.iter().copied().map(crate::wet::Latent::from_color).collect();
        // (the pigment's packing is the drain section's, version 14; else
        // paint naming no tubes')
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
            v @ (1 | 2 | 3) => {
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
                return Err(bad("an engine-4 canvas in an older format: this easel saves engine 4 and later as PAINTC11, PAINTC12 or PAINTC14"));
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
        // the drain section (version 14): the switch, the pigment's packing
        // per pixel, and whether a soak section follows
        let raw = match version {
            10 | 12 => true,
            14 | 16 | 17 => {
                if get_u64(r)? != DRAIN_MARK {
                    return Err(bad("checkpoint drain mark is wrong"));
                }
                match get_u64(r)? {
                    DRAIN_V => {}
                    v => return Err(bad(&format!("checkpoint drain section is version {v}; this easel reads version {DRAIN_V} only"))),
                }
                let raw = match get_u64(r)? {
                    0 => false,
                    1 => true,
                    _ => return Err(bad("checkpoint drain section's soak flag is invalid")),
                };
                let pack = get_all(r, n * 5)?;
                // (the ground's drain would carry a non-finite one on)
                if !pack.iter().all(|v| v.is_finite()) {
                    return Err(bad("checkpoint paint packing is invalid"));
                }
                for (h, k) in c.wet.hide.iter_mut().zip(pack.as_chunks::<5>().0) {
                    h[5..].copy_from_slice(k);
                }
                c.drain = true;
                if !raw && version == 14 && !pigments && r.read(&mut [0u8; 1])? != 0 {
                    return Err(bad("checkpoint has trailing data"));
                }
                raw
            }
            _ => false,
        };
        // raw formats go on with the soak section, which ends the file
        if raw {
            if !c.f.is_whole() {
                return Err(bad("checkpoint soak is on a crop render"));
            }
            c.soak = Some(Box::new(read_soak(r, n)?));
            if version != 17 && !pigments && r.read(&mut [0u8; 1])? != 0 {
                return Err(bad("checkpoint has trailing data"));
            }
        }
        // a sheet of paper's section, which ends the file
        if matches!(version, 13 | 16) {
            let (p, micro) = read_paper(r, n)?;
            c.paper = Some(p);
            c.micro = micro;
            if !pigments && r.read(&mut [0u8; 1])? != 0 {
                return Err(bad("checkpoint has trailing data"));
            }
        }
        // a sheet on the picture: the soak or paper section if any, each by
        // its mark, then the sheet's, which ends the file
        if matches!(version, 15 | 17) {
            let mut mark = get_u64(r)?;
            if mark == SOAK_MARK {
                if !c.f.is_whole() {
                    return Err(bad("checkpoint soak is on a crop render"));
                }
                c.soak = Some(Box::new(read_soak_body(r, n)?));
                mark = get_u64(r)?;
            } else if mark == PAPER_MARK {
                let (p, micro) = read_paper_body(r, n)?;
                c.paper = Some(p);
                c.micro = micro;
                mark = get_u64(r)?;
            }
            if mark != SHEET_MARK {
                return Err(bad("checkpoint sheet mark is wrong"));
            }
            c.sheet = Some(read_sheet_body(r, n)?);
            if !pigments && r.read(&mut [0u8; 1])? != 0 {
                return Err(bad("checkpoint has trailing data"));
            }
        }
        if version == 7 && !pigments && r.read(&mut [0u8; 1])? != 0 {
            return Err(bad("version-7 checkpoint has trailing data"));
        }
        if pigments {
            for pixel in &mut c.wet.lat {
                let count = usize::try_from(get_u64(r)?).map_err(|_| bad("checkpoint pigment count is invalid"))?;
                for _ in 0..count {
                    let i = usize::try_from(get_u64(r)?).map_err(|_| bad("checkpoint pigment is invalid"))?;
                    let amount = get_f32(r)?;
                    if !amount.is_finite() || amount < 0.0 || pixel.parts.iter().any(|p| p.0 == i) {
                        return Err(bad("checkpoint pigment amount is invalid"));
                    }
                    pixel.parts.push((i, amount));
                }
            }
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

    fn set(b: &mut [u8], pos: usize, v: u64) {
        b[pos..pos + 8].copy_from_slice(&v.to_le_bytes());
    }

    fn load(b: Vec<u8>) -> io::Result<Canvas> {
        Canvas::read_state(&mut Cursor::new(b)).map(|(c, _)| c)
    }

    /// A canvas with the drain switch is written as version 14: version 11's
    /// bytes, then the drain section, so it keeps the switch and all ten
    /// properties of its wet paint, and is rewritten as it was read; the
    /// same canvas without the switch is version 11 as before. The section
    /// is refused with a wrong mark or version, a soak flag other than 0 or
    /// 1, a non-finite packing or data after it, and the switch is refused
    /// on a canvas before engine 4.
    #[test]
    fn the_drain_switch_has_its_own_format() {
        let canvas = |drain: bool| {
            let mut c = Canvas::new_window(2, 1.0, [0.1; 3], None).with_engine(5).with_drain(drain);
            c.absorb.iter_mut().for_each(|a| *a = 0.3);
            c.wet.vol[0] = 1.5;
            c.wet.hide[0] = [0.7, 0.6, 1.2, 0.4, 0.5, 0.45, 0.3, 0.25, 0.6, 0.2];
            let mut b = Vec::new();
            c.write_state(&mut b, "").unwrap();
            b
        };
        let (off, on) = (canvas(false), canvas(true));
        assert_eq!((&off[..8], &on[..8]), (&b"PAINTC11"[..], &b"PAINTC14"[..]));
        assert!(on[8..off.len()] == off[8..], "version 11's bytes, then the drain section");
        let n = 4;
        assert_eq!(on.len(), off.len() + 24 + n * 5 * 4);
        let d = load(on.clone()).unwrap();
        assert!(d.drain && d.absorb_any && d.wet.hide[0] == [0.7, 0.6, 1.2, 0.4, 0.5, 0.45, 0.3, 0.25, 0.6, 0.2]);
        let mut again = Vec::new();
        d.write_state(&mut again, "").unwrap();
        assert!(again == on, "rewritten as it was read");
        assert!(!load(off.clone()).unwrap().drain);
        let at = off.len();
        for (pos, v) in [(at, 0u64), (at + 8, 2), (at + 16, 2)] {
            let mut b = on.clone();
            set(&mut b, pos, v);
            assert!(load(b).is_err(), "byte {pos} = {v}");
        }
        let mut b = on.clone();
        b[at + 24..at + 28].copy_from_slice(&f32::NAN.to_le_bytes());
        assert!(load(b).is_err(), "a NaN packing");
        let mut b = on.clone();
        b.push(0);
        assert!(load(b).is_err(), "trailing data");
        let c = Canvas::new_window(2, 1.0, [0.1; 3], None).with_engine(3).with_drain(true);
        assert!(c.write_state(&mut Vec::new(), "").is_err());
    }

    #[test]
    fn drained_paper_keeps_both_sections() {
        let mut c = Canvas::new_window(32, 1.0, [0.5; 3], None)
            .with_paper(crate::paper::Paper::drawing(2))
            .with_drain(true);
        c.wet.vol[0] = 1.5;
        c.wet.hide[0] = [0.7, 0.6, 1.2, 0.4, 0.5, 0.45, 0.3, 0.25, 0.6, 0.2];
        let mut bytes = Vec::new();
        c.write_state(&mut bytes, "").unwrap();
        assert_eq!(&bytes[..8], b"PAINTC16");
        let d = load(bytes.clone()).unwrap();
        assert!(d.drain && d.paper == c.paper && d.micro == c.micro && d.wet.hide == c.wet.hide);
        let mut again = Vec::new();
        d.write_state(&mut again, "").unwrap();
        assert_eq!(again, bytes);
        bytes.push(0);
        assert!(load(bytes).is_err());
    }

    #[test]
    fn drained_masking_sheets_keep_each_support() {
        for drain in [false, true] {
            for support in 0..3 {
                let mut c = Canvas::new_window(32, 1.0, [0.5; 3], None).with_drain(drain);
                match support {
                    1 => c = c.with_paper(crate::paper::Paper::drawing(2)),
                    2 => c.raw_canvas(crate::soak::Fabric::cotton_duck(), 2),
                    _ => {}
                }
                c.wet.hide[0] = [0.7, 0.6, 1.2, 0.4, 0.5, 0.45, 0.3, 0.25, 0.6, 0.2];
                let m = crate::mask::Mask::from_fn(c.frame(), |x, _| if x < 500.0 { 1.0 } else { 0.0 });
                c.lay_sheet(&m, 100.0, 5.0, [0.8, 0.78, 0.7]).unwrap();
                c.sheet_catch(&[(0, 2.0)], [0.7, 0.2, 0.1]);
                let mut bytes = Vec::new();
                c.write_state(&mut bytes, "").unwrap();
                assert_eq!(&bytes[..8], if drain { MAGIC17 } else { MAGIC15 });
                let r = load(bytes.clone()).unwrap();
                assert_eq!(r.drain, drain);
                assert_eq!(r.is_raw(), c.is_raw());
                assert_eq!(r.paper, c.paper);
                assert_eq!(r.micro, c.micro);
                assert_eq!(r.sheet, c.sheet);
                assert_eq!(r.pixels(), c.pixels());
                if drain {
                    assert_eq!(r.wet.hide, c.wet.hide);
                }
                let mut again = Vec::new();
                r.write_state(&mut again, "").unwrap();
                assert_eq!(again, bytes);
                assert!(load(bytes[..bytes.len() - 1].to_vec()).is_err());
                bytes.push(0);
                assert!(load(bytes).is_err());
            }
        }
    }

    fn rejected(b: Vec<u8>, what: &str) {
        match load(b) {
            Ok(_) => panic!("{what}: accepted"),
            Err(e) => assert_eq!(e.kind(), io::ErrorKind::InvalidData, "{what}: {e}"),
        }
    }

    /// A sheet of paper is written as version 13 and comes back bit for
    /// bit (the sheet's make, its pores, its surface, matte face and
    /// absorbency), and goes on taking a ground as it would have; without
    /// the sheet (linen put in its place) the canvas is version 11 again.
    /// The section is refused when it is cut short, followed by more, or
    /// carries a wrong mark or a NaN, and a sheet is no engine-3 canvas.
    #[test]
    fn a_sheet_of_paper_survives_a_checkpoint() {
        use crate::paper::{Laid, Paper};
        let p = Paper { laid: Some(Laid { per_cm: 9.0, chain_mm: 24.0, deficit: 0.2 }), ..Paper::drawing(2) };
        let mut c = Canvas::new_window(240, 1.5, [0.5; 3], None).with_size_mm(120.0).with_paper(p);
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        assert_eq!(&b[..8], b"PAINTC13");
        let mut d = load(b.clone()).unwrap();
        assert!(d.paper == c.paper && d.micro == c.micro && d.height == c.height && d.absorb == c.absorb && d.gloss == c.gloss && d.absorb_any);
        let mut again = Vec::new();
        d.write_state(&mut again, "").unwrap();
        assert!(again == b, "rewritten as it was read");
        for k in [&mut c, &mut d] {
            k.prime([0.8, 0.78, 0.7], 0.6, 40.0, 0.3, 0.4, 5);
            k.ground_finish(0.5);
        }
        assert!(d.pixels() == c.pixels() && d.height == c.height && d.absorb == c.absorb);
        let mut linen = Vec::new();
        c.clone().with_linen(Linen::fine(1)).write_state(&mut linen, "").unwrap();
        assert_eq!(&linen[..8], b"PAINTC11");
        let mut cut = b.clone();
        cut.pop();
        assert_eq!(load(cut).err().unwrap().kind(), io::ErrorKind::UnexpectedEof);
        let mut trailing = b.clone();
        trailing.push(0);
        rejected(trailing, "trailing byte");
        let n = c.f.w * c.f.h;
        // (the section: mark, version, 12 numbers, seed, felt, laid, then the pores)
        let section = b.len() - (8 + 8 + 12 * 4 + 8 + (8 + 2 * 4) + (8 + 3 * 4) + 4 * n);
        let mut mark = b.clone();
        set(&mut mark, section, 0);
        rejected(mark, "paper mark");
        let mut nan = b.clone();
        let last = nan.len() - 4;
        nan[last..].copy_from_slice(&f32::NAN.to_le_bytes());
        rejected(nan, "a NaN pore");
        let mut no_section = b[..section].to_vec();
        no_section[..8].copy_from_slice(b"PAINTC11");
        assert!(load(no_section.clone()).is_ok());
        no_section[..8].copy_from_slice(b"PAINTC13");
        assert_eq!(load(no_section).err().unwrap().kind(), io::ErrorKind::UnexpectedEof);
        let e = c.clone().with_engine(3).write_state(&mut Vec::new(), "").unwrap_err();
        assert_eq!(e.kind(), io::ErrorKind::InvalidInput);
    }

    /// A sheet of paper with stick pastel in its tooth survives a
    /// checkpoint bit for bit (the paper, its micro-roughness, the loose and
    /// bound pastel), and goes on taking pastel as it would have; a drawing
    /// with stick pastel is flag 3, and a corrupt one is refused.
    #[test]
    fn paper_and_stick_pastel_survive_a_checkpoint() {
        use crate::pastel::{Pose, Stick, StrokePoint};
        let mut c = Canvas::new_window(240, 1.5, [0.5; 3], None).with_size_mm(120.0).with_paper(crate::paper::Paper::drawing(2));
        let mut st = Stick::round([0.7, 0.2, 0.1], 0.8, 12.0);
        let pose = Pose { force: 2.0, alt: 1.0, az: 0.8, roll: 0.0 };
        let line = |y: f32| [StrokePoint { x: 100.0, y, pose, speed: 60.0 }, StrokePoint { x: 900.0, y, pose, speed: 60.0 }];
        c.stick_stroke(&mut st, &line(200.0));
        c.fix_pastel(None, 0.15);
        c.stick_stroke(&mut st, &line(260.0));
        let d = c.drawing.as_ref().unwrap();
        assert!(d.layout() == 3 && d.cells.iter().any(|c| c.loose > 0.0) && d.cells.iter().any(|c| c.bound > 0.0));
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        let (mut r, _) = Canvas::read_state(&mut Cursor::new(b.clone())).unwrap();
        assert!(r.pixels() == c.pixels() && r.height == c.height && r.micro == c.micro && r.paper == c.paper);
        let mut again = Vec::new();
        r.write_state(&mut again, "").unwrap();
        assert!(again == b, "rewritten as it was read");
        let mut st2 = st.clone();
        c.stick_stroke(&mut st, &line(300.0));
        r.stick_stroke(&mut st2, &line(300.0));
        assert!(r.pixels() == c.pixels() && st == st2);
        // a negative volume in the tooth is refused
        c.drawing.as_mut().unwrap().cells[7].loose = -1.0;
        let mut bad = Vec::new();
        c.write_state(&mut bad, "").unwrap();
        assert_eq!(Canvas::read_state(&mut Cursor::new(bad)).err().unwrap().kind(), io::ErrorKind::InvalidData);
    }

    /// A sheet of paper (version 13), and one with a sheet laid on it
    /// (version 15), carry the recorded quarter tiles and adaptive sweep in
    /// their engine word, as every other version does.
    #[test]
    fn a_sheet_of_paper_keeps_the_recorded_settings_in_its_engine_word() {
        let c = Canvas::new_window(60, 1.5, [0.5; 3], None).with_engine(5).with_size_mm(120.0).with_paper(crate::paper::Paper::drawing(2)).with_quarter_tiles(true).with_adaptive_sweep(true);
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        assert_eq!(&b[..8], MAGIC13);
        let r = load(b.clone()).unwrap();
        assert!(r.quarter_tiles() && r.adaptive_sweep() && r.paper == c.paper);
        let mut again = Vec::new();
        r.write_state(&mut again, "").unwrap();
        assert_eq!(again, b);
        // and with a sheet laid on the picture (version 15)
        let mut c = c;
        let m = crate::mask::Mask::from_fn(c.frame(), |x, _| if x < 500.0 { 1.0 } else { 0.0 });
        c.lay_sheet(&m, 100.0, 5.0, [0.8, 0.78, 0.7]).unwrap();
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        assert_eq!(&b[..8], MAGIC15);
        let r = load(b.clone()).unwrap();
        assert!(r.quarter_tiles() && r.adaptive_sweep() && r.sheet == c.sheet);
        let mut again = Vec::new();
        r.write_state(&mut again, "").unwrap();
        assert_eq!(again, b);
    }

    /// A sheet of paper laid on the picture survives a checkpoint, with the
    /// pastel caught on it: version 15, read back as written, on paper and
    /// on linen alike; none laid (or lifted), none written, and the file is
    /// what it was. A mask that isn't 0 or 1, or a wrong mark, is refused.
    #[test]
    fn a_laid_sheet_survives_a_checkpoint() {
        use crate::pastel::{Pose, Stick, StrokePoint};
        let mut c = Canvas::new_window(240, 1.5, [0.5; 3], None).with_size_mm(120.0).with_paper(crate::paper::Paper::drawing(2));
        let bytes = |c: &Canvas| {
            let mut b = Vec::new();
            c.write_state(&mut b, "").unwrap();
            b
        };
        let bare = bytes(&c);
        assert_eq!(&bare[..8], b"PAINTC13");
        assert!(load(bare.clone()).unwrap().sheet.is_none());
        let m = crate::mask::Mask::from_fn(c.frame(), |x, _| if x < 500.0 { 1.0 } else { 0.0 });
        c.lay_sheet(&m, 100.0, 5.0, [0.8, 0.78, 0.7]).unwrap();
        let mut st = Stick::round([0.7, 0.2, 0.1], 0.8, 12.0);
        let pose = Pose { force: 2.0, alt: 1.0, az: 0.8, roll: 0.0 };
        c.stick_stroke(&mut st, &[StrokePoint { x: 100.0, y: 300.0, pose, speed: 60.0 }, StrokePoint { x: 900.0, y: 300.0, pose, speed: 60.0 }]);
        assert!(c.sheet.as_ref().unwrap().a.iter().any(|&a| a > 0.0), "the stroke caught on the sheet");
        let b = bytes(&c);
        assert_eq!(&b[..8], b"PAINTC15");
        let r = load(b.clone()).unwrap();
        assert!(r.sheet.is_some() && r.sheet == c.sheet && r.paper == c.paper && r.micro == c.micro && r.pixels() == c.pixels());
        assert!(bytes(&r) == b, "rewritten as it was read");
        // on linen: version 11's bytes, then the sheet
        let mut linen = Canvas::new_window(240, 1.5, [0.5; 3], None).with_size_mm(120.0);
        linen.lay_sheet(&m, 100.0, 5.0, [0.8, 0.78, 0.7]).unwrap();
        let lb = bytes(&linen);
        assert_eq!(&lb[..8], b"PAINTC15");
        assert!(load(lb).unwrap().sheet == linen.sheet);
        let n = c.f.w * c.f.h;
        // (the section: mark, version, five numbers, the mask, its coverage and colors)
        let section = b.len() - (8 + 8 + 5 * 4 + 4 * n + 4 * n + 12 * n);
        let mut mark = b.clone();
        set(&mut mark, section, 0);
        rejected(mark, "sheet mark");
        let mut half = b.clone();
        let at = section + 8 + 8 + 5 * 4;
        half[at..at + 4].copy_from_slice(&0.5f32.to_le_bytes());
        rejected(half, "a mask of 0.5");
        assert!(c.lift_sheet());
        assert_eq!(&bytes(&c)[..8], b"PAINTC13");
    }
}
