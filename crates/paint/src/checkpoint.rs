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
//! Engine-1 writes retain base layout 8 inside the current envelope.
//! New writes use version 19 (`PAINTC19`): after the header, three u64 words
//! identify the pigment mixer, the base layout below and whether tube
//! parts follow (0 or 1). This identity is independent of the engine number.
//! Older files containing wet paint are refused because their latent vectors
//! cannot be decoded by this mixer. Empty older files can still be read.
//! Versions 7–18 store seven latent floats per pixel, not the current width.
//! Retained wet layers use version 20. A retained spectral dry backing uses
//! version 21: a wet-layer presence flag, those layers if present, then
//! 38 reflectances, three residuals and three cached RGB values per pixel.
//! A canvas painted with engine 9's material revision 1 or later
//! (`Canvas::with_material`, `crate::skin`) is version 22 (`PAINTC22`): the
//! header, the version it would otherwise be (19, 20 or 21, one u64), that
//! version's bytes after its header, then the skin section, the file's last:
//! a `SKIN` mark and the section's own version (1, `SKIN_V`), the material
//! revision, the clock minute of the last tick (f64), and each buffer
//! pixel's baked films top down (a count, then ten f32 per film:
//! thickness, front depth, unexpanded skin, membrane force, bulk moved,
//! birth minute, front rate, stiffness, binder share, measured slope). A
//! reader that
//! doesn't know version 22 refuses the file instead of loading the
//! picture as the old material.
//!
//! The base format is version 8 (`MAGIC` is `PAINTCK8`) for engines 1 and 2 and
//! version 9 (`PAINTCK9`) for engine 3, or version 10 for a raw canvas
//! (below). Version 9 is version 8's bytes after the magic, then the
//! thinner's section: the solvent in the open film, one f32 (µm) per buffer
//! pixel, row major, as the file's last 4 × pixels bytes (`crate::thinner`;
//! that it comes last is part of version 9; version 10 adds its section
//! after it). An engine-3 canvas saved as version 8 (by the easel before the
//! thinner, af49348) is refused, naming that version: it has no solvent
//! section, and this easel doesn't convert old saves. Version 11
//! (`PAINTC11`) is engine 4's and later: version 9 with two more properties
//! (solvent and oil) to each pixel of wet paint and, after the engine version and before
//! the solvent (still last), the surface's gloss and the ground's remaining
//! absorbency, one f32 each per pixel. An older engine retains its base
//! layout inside the current envelope. Version 12 combines version 11 with the raw canvas's
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
//!
//! A canvas with the oils switch on (`Canvas::with_oils`, engine 4's
//! materials and later) says so in its engine word (`OILS`) and has, after
//! the solvent, the oils section: an `OILS` mark and the section's own
//! version (1, `OILS_V`), then the share of each buffer pixel's oil that is
//! waxed (`crate::wet::Prop`'s last), one f32 each, row major. Its base
//! version is the one it would otherwise be (11 to 17); a drain section
//! after it carries the wax too, and must agree. The section is there
//! exactly when the switch is on: a canvas without it is written as before,
//! and a reader that doesn't know the `OILS` bit refuses the engine word
//! instead of loading the canvas without its wax.
//! A canvas on a sheet of paper (`crate::paper`) is version 13
//! (`PAINTC13`): version 11's bytes after the magic, then the paper's
//! section, which is the file's last: a `PAPR` mark and the section's own
//! version (1, `PAPER_V`), the sheet's make (grammage, fiber length, width
//! and thickness, coarseness, porosity, flocs and their size, pressing,
//! calendering, absorbency and z modulus, one f32 each), its seed, its felt
//! and laid wires (each a 0/1 flag, then its numbers), and the pores' mean
//! depth per buffer pixel (`Canvas::micro`). A sheet with extras writes
//! its section as version 2 (`PAPER_V2`): version 1's fields up to and
//! including the laid wires, then a u64 flags word of the extras it has,
//! then each extra's block in the order of its bit, then the pores as
//! before. Bit 0 (`PAPER_EXTRA_GRIT`) is a gritted ground (`paper::Grit`:
//! its size and cover, two f32, and its binder, a u64: 0 acrylic, 1 glue;
//! any other refused). Bit 1 (`PAPER_EXTRA_DRUM`) is a strainer's drum
//! (`paper::Drum`): bar_mm and bay_mpa_per_um, then bend_mm and patch_mm
//! (four f32), then the crossbars (a u64). A sheet may have both. A reader
//! refuses any bit it doesn't know, and an extras word of 0 in version 2
//! (such a sheet is version 1). A sheet with no extras writes version 1, so
//! a reader that doesn't know version 2 refuses a gritted or drummed sheet
//! instead of loading it as plain paper on a firm bed. The section is there exactly
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
const MAGIC19: &[u8; 8] = b"PAINTC19";
const MAGIC20: &[u8; 8] = b"PAINTC20";
const MAGIC21: &[u8; 8] = b"PAINTC21";
const MAGIC22: &[u8; 8] = b"PAINTC22";
const HISTORICAL_LAT: usize = 7;
/// Version 11 followed by the paper's section (`crate::paper`).
const MAGIC13: &[u8; 8] = b"PAINTC13";
/// Marks the paper's section ("PAPR"), and its version.
const PAPER_MARK: u64 = 0x52504150;
const PAPER_V: u64 = 1;
/// The paper's section of a sheet with extras: version 1's fields up to
/// the laid wires, a u64 of `PAPER_EXTRA_*` bits, each set bit's block in
/// bit order, then the pores.
const PAPER_V2: u64 = 2;
/// Extras bit 0: a gritted ground (`paper::Grit`: size f32, cover f32,
/// binder u64).
const PAPER_EXTRA_GRIT: u64 = 1;
/// Extras bit 1: a strainer's drum (`paper::Drum`: bar_mm, bay_mpa_per_um,
/// bend_mm and patch_mm, f32 each, then cross, a u64).
const PAPER_EXTRA_DRUM: u64 = 2;
/// The most crossbars a drum may have (`paper::MAX_CROSS`).
const MAX_CROSS: u64 = crate::paper::MAX_CROSS as u64;
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
/// Marks the skin section ("SKIN"), and its version.
const SKIN_MARK: u64 = 0x4e494b53;
const SKIN_V: u64 = 1;
/// Marks the oils section ("OILS"), and its version.
const OILS_MARK: u64 = 0x534c494f;
const OILS_V: u64 = 1;
const QUARTER_TILES: u64 = 1 << 33;
const ADAPTIVE_SWEEP: u64 = 1 << 34;
const GLAZE_FILM: u64 = 1 << 36;
const KNIFE_SKIM: u64 = 1 << 35;
// (1 << 35 and 1 << 36 are taken by the knife skim and the glaze film)
const BLEND_FEATHER: u64 = 1 << 37;
const BRUSH_SKIM: u64 = 1 << 38;
const RAG_LIFT: u64 = 1 << 39;
const LIQUID_FILM_LEVELS: u64 = 1 << 40;
const SEALED_DRAWING_GUIDE: u64 = 1 << 41;
const DRAWING_OPTICS: u64 = 1 << 42;
/// The oils switch (`Canvas::with_oils`): an oils section follows the solvent.
const OILS: u64 = 1 << 43;
/// Pastel takes water (`Canvas::with_wet_pastel`): the engine word has
/// this bit, and its section follows the word (`WET_PASTEL_MARK`).
/// (Bit 43 is the tube oils switch's, `OILS`.)
const WET_PASTEL: u64 = 1 << 44;
/// Marks the wet pastel section ("WETP"), and its version: written right
/// after the engine word exactly when it has `WET_PASTEL`: whether a
/// drawing follows (0 or 1), then for each of its cells (one triple per
/// buffer pixel) the water in it (µm), its particles' mean index and the
/// share of its gum it holds, the last two as their stored u16s (each an
/// f32), then whether the sheet has taken water (0 or 1) and, if it has,
/// the water under each cell (µm); then the paper's sizing (0 sized, its
/// Cobb60 as an f32; 1 unsized, 0.0). A reader that doesn't know the bit
/// refuses the file instead of losing the water.
const WET_PASTEL_MARK: u64 = 0x50544557;
const WET_PASTEL_V: u64 = 2;
/// A damp sheet (`crate::damp`, the damp-paper switch): the engine word has
/// this bit, and its section follows the wet pastel section's place
/// (`DAMP_MARK`). (Bit 44 is the wet pastel switch's, `WET_PASTEL`.)
const DAMP_PAPER: u64 = 1 << 45;
/// Marks the damp sheet's section ("DAMP"), and its version: written right
/// after the engine word (and the wet pastel section, if any) exactly when
/// it has `DAMP_PAPER`: the sheet's water, g/m² (f64 bits), and its open
/// faces (0, 1 or 2). A reader that doesn't know the bit refuses the file
/// instead of drying the sheet out; a section of another version, or a
/// damp canvas that isn't paper, is refused.
const DAMP_MARK: u64 = 0x504d4144;
const DAMP_V: u64 = 1;

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

/// `n` floats, read through a small buffer: the only allocation is the result
/// (a 2400 px canvas's latents are close to a gigabyte, and a canvas is read
/// back beside the live one when a chunk is rolled back).
fn get_all(r: &mut impl Read, n: usize) -> io::Result<Vec<f32>> {
    let mut v = Vec::with_capacity(n);
    get_into(r, n, |x| v.push(x))?;
    Ok(v)
}

fn get_into(r: &mut impl Read, n: usize, mut put: impl FnMut(f32)) -> io::Result<()> {
    let mut buf = [0u8; 1 << 16];
    let mut left = n * 4;
    while left > 0 {
        let take = left.min(buf.len());
        r.read_exact(&mut buf[..take])?;
        for b in buf[..take].as_chunks::<4>().0 {
            put(f32::from_le_bytes(*b));
        }
        left -= take;
    }
    Ok(())
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
    // (a sheet with no extras is written as before: version 1)
    let extras = if p.grit.is_some() { PAPER_EXTRA_GRIT } else { 0 } | if p.drum.is_some() { PAPER_EXTRA_DRUM } else { 0 };
    put_u64(w, if extras != 0 { PAPER_V2 } else { PAPER_V })?;
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
    if extras != 0 {
        put_u64(w, extras)?;
    }
    if let Some(g) = p.grit {
        put_f32(w, g.size_um)?;
        put_f32(w, g.cover)?;
        put_u64(w, match g.binder {
            crate::paper::Binder::Acrylic => 0,
            crate::paper::Binder::Glue => 1,
        })?;
    }
    if let Some(d) = p.drum {
        for v in [d.bar_mm, d.bay_mpa_per_um, d.bend_mm, d.patch_mm] {
            put_f32(w, v)?;
        }
        put_u64(w, d.cross as u64)?;
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
    use crate::paper::{Binder, Drum, Felt, Grit, Laid, Paper};
    let v2 = match get_u64(r)? {
        PAPER_V => false,
        PAPER_V2 => true,
        v => return Err(bad(&format!("checkpoint paper section is version {v}; this easel reads versions {PAPER_V} and {PAPER_V2} only"))),
    };
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
    let extras = if v2 { get_u64(r)? } else { 0 };
    if v2 && extras == 0 {
        return Err(bad("checkpoint paper section is version 2 with no extras (such a sheet is version 1)"));
    }
    if extras & !(PAPER_EXTRA_GRIT | PAPER_EXTRA_DRUM) != 0 {
        return Err(bad(&format!("checkpoint paper extras {extras:#x} has bits this easel doesn't know")));
    }
    let grit = if extras & PAPER_EXTRA_GRIT != 0 {
        let (size_um, cover) = (get_f32(r)?, get_f32(r)?);
        let binder = match get_u64(r)? {
            0 => Binder::Acrylic,
            1 => Binder::Glue,
            b => return Err(bad(&format!("checkpoint paper grit binder {b} is unknown"))),
        };
        if !(size_um.is_finite() && size_um > 0.0 && cover.is_finite() && (0.0..=1.0).contains(&cover)) {
            return Err(bad("checkpoint paper grit is invalid"));
        }
        Some(Grit { size_um, cover, binder })
    } else {
        None
    };
    let drum = if extras & PAPER_EXTRA_DRUM != 0 {
        let v = [get_f32(r)?, get_f32(r)?, get_f32(r)?, get_f32(r)?];
        let cross = get_u64(r)?;
        // (bars, a positive give, bending length and patch; a few crossbars)
        if !(v.iter().all(|x| x.is_finite()) && v[0] >= 0.0 && v[1..].iter().all(|&x| x > 0.0) && cross <= MAX_CROSS) {
            return Err(bad("checkpoint paper drum is invalid"));
        }
        Some(Drum { bar_mm: v[0], bay_mpa_per_um: v[1], bend_mm: v[2], patch_mm: v[3], cross: cross as u32 })
    } else {
        None
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
        floc: v[6], floc_mm: v[7], press: v[8], calender: v[9], absorbent: v[10], z_mpa: v[11], felt, laid, grit, drum, seed,
        // (written in the wet pastel section, where only it is read)
        cobb60: Paper::drawing(0).cobb60,
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
        m if m == MAGIC19 => 19,
        m if m == MAGIC20 => 20,
        m if m == MAGIC21 => 21,
        m if m == MAGIC22 => 22,
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
        let layers = self.wet.lat.iter().any(|p| !p.layers.is_empty());
        let pigments = layers || self.wet.lat.iter().any(|p| !p.parts.is_empty());
        let backing = self.spectral_films() && !self.backing.is_empty();
        // (a raw canvas is engine 3's: version 10 holds version 9's bytes)
        if self.soak.is_some() && self.engine < 3 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "a raw canvas is engine 3's: this one is set to an older engine"));
        }
        // (the drain switch: version 14 holds version 11's bytes)
        if self.drain && self.engine < 4 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "the drain switch drains engine 4's oil: this canvas is set to an older engine"));
        }
        // (the oils switch: its wax is in engine 4's materials)
        if self.oils && self.engine < 4 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "the oils switch's wax dulls engine 4's gloss: this canvas is set to an older engine"));
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
        let material = self.material >= 1;
        w.write_all(if material { MAGIC22 } else if backing { MAGIC21 } else if layers { MAGIC20 } else { MAGIC19 })?;
        put_u64(w, header.len() as u64)?;
        w.write_all(header.as_bytes())?;
        if material {
            put_u64(w, if backing { 21 } else if layers { 20 } else { 19 })?;
        }
        put_u64(w, if self.spectral_films() { crate::mixer::film_identity() } else if self.engine >= crate::PIGMENT_MIXER { crate::mixer::identity9() } else { crate::mixer::identity() })?;
        put_u64(w, version)?;
        put_u64(w, u64::from(pigments))?;
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
        let width = if self.engine >= crate::PIGMENT_MIXER { LAT } else { crate::mixer::LEGACY_LATENT_SIZE };
        put_all(w, wt.lat.iter().flat_map(|l| l.color[..width].iter().copied()))?;
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
        put_u64(w, self.engine as u64 | if self.drawing_optics { DRAWING_OPTICS } else { 0 } | if self.sealed_drawing_guide { SEALED_DRAWING_GUIDE } else { 0 } | if self.liquid_film_levels { LIQUID_FILM_LEVELS } else { 0 } | if self.quarter_tiles { QUARTER_TILES } else { 0 } | if self.adaptive_sweep { ADAPTIVE_SWEEP } else { 0 } | if self.glaze_film { GLAZE_FILM } else { 0 } | if self.rag_lift { RAG_LIFT } else { 0 } | if self.knife_skim { KNIFE_SKIM } else { 0 } | if self.blend_feather { BLEND_FEATHER } else { 0 } | if self.brush_skim { BRUSH_SKIM } else { 0 } | if self.oils { OILS } else { 0 } | if self.wet_pastel { WET_PASTEL } else { 0 } | if self.damp.is_some() { DAMP_PAPER } else { 0 })?;
        if self.wet_pastel {
            put_u64(w, WET_PASTEL_MARK)?;
            put_u64(w, WET_PASTEL_V)?;
            put_u64(w, u64::from(self.drawing.is_some()))?;
            if let Some(d) = &self.drawing {
                put_all(w, d.cells.iter().flat_map(|c| [c.water, c.index as f32, c.gum as f32]))?;
                put_u64(w, u64::from(!d.sheet.is_empty()))?;
                put_all(w, d.sheet.iter().copied())?;
            }
            let cobb = self.paper.as_ref().map_or(crate::paper::Paper::drawing(0).cobb60, |p| p.cobb60);
            put_u64(w, u64::from(cobb.is_none()))?;
            put_f32(w, cobb.unwrap_or(0.0))?;
        }
        if let Some(d) = &self.damp {
            if self.paper.is_none() || !(d.water.is_finite() && d.water >= 0.0) || d.faces > 2 {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("the damp sheet's state is invalid: {d:?}")));
            }
            put_u64(w, DAMP_MARK)?;
            put_u64(w, DAMP_V)?;
            put_u64(w, d.water.to_bits())?;
            put_u64(w, d.faces as u64)?;
        }
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
        // the oils section (the switch, `OILS`): the wax per pixel
        if self.oils {
            put_u64(w, OILS_MARK)?;
            put_u64(w, OILS_V)?;
            put_all(w, wt.hide.iter().map(|h| h[9]))?;
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
        if backing {
            put_u64(w, u64::from(layers))?;
        }
        if layers {
            for pixel in &self.wet.lat {
                put_u64(w, pixel.layers.len() as u64)?;
                for layer in &pixel.layers {
                    put_f32(w, layer.share)?;
                    put_all(w, layer.lat.color.into_iter())?;
                    put_all(w, layer.hide.into_iter())?;
                    put_u64(w, layer.lat.parts.len() as u64)?;
                    for &(i, amount) in &layer.lat.parts {
                        put_u64(w, i as u64)?;
                        put_f32(w, amount)?;
                    }
                }
            }
        }
        if backing {
            put_all(w, self.backing.iter().flat_map(|b| b.reflectance.into_iter().chain(b.residual).chain(b.color)))?;
        }
        if material {
            put_u64(w, SKIN_MARK)?;
            put_u64(w, SKIN_V)?;
            put_u64(w, u64::from(self.material))?;
            w.write_all(&self.skin.ticked.to_le_bytes())?;
            for i in 0..self.f.w * self.f.h {
                put_u64(w, self.skin.column(i).count() as u64)?;
                put_all(w, self.skin.column(i).flat_map(|p| [p.h, p.s, p.x, p.n, p.w, p.born, p.v0, p.stiff, p.binder, p.slope]))?;
            }
        }
        Ok(())
    }

    /// Read a canvas written by `write_state`; returns it and the header.
    pub fn read_state(r: &mut impl Read) -> io::Result<(Canvas, String)> {
        let (version, header) = read_magic_header(r)?;
        let material = version == 22;
        let version = if material {
            match get_u64(r)? {
                v @ 19..=21 => v as u32,
                _ => return Err(bad("checkpoint material base format is invalid")),
            }
        } else {
            version
        };
        let mut layers = version == 20;
        let spectral_backing = version == 21;
        let own_mixer = version >= 19;
        let identity = if own_mixer { get_u64(r)? } else { 0 };
        if own_mixer && identity != crate::mixer::identity() && identity != crate::mixer::identity9() && identity != crate::mixer::film_identity() {
            return Err(bad("checkpoint uses an obsolete pigment representation; recreate the checkpoint with this build"));
        }
        let mut pigments = version == 18;
        let version = if pigments || own_mixer {
            let version = get_u64(r)?;
            if !(8..=17).contains(&version) { return Err(bad("checkpoint base format is invalid")); }
            if own_mixer {
                pigments = match get_u64(r)? {
                    0 => false,
                    1 => true,
                    _ => return Err(bad("checkpoint pigment flag is invalid")),
                };
            }
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
        // every per-pixel field is read below (the floor is not stored), so none is
        // allocated twice: a 2400 px canvas's latents alone are close to a gigabyte
        let mut wet = crate::wet::Wet::new(0);
        wet.floor = vec![0.0; n];
        wet.vol = get_all(r, n)?;
        let width = if !own_mixer { HISTORICAL_LAT } else if identity == crate::mixer::identity() { crate::mixer::LEGACY_LATENT_SIZE } else { LAT };
        if own_mixer {
            // the stored latents go straight into the pixels' (no copy of them all as floats)
            let mut lat = Vec::with_capacity(n);
            let (mut color, mut k) = ([0.0; LAT], 0);
            get_into(r, n * width, |x| {
                color[k] = x;
                k += 1;
                if k == width {
                    lat.push(crate::wet::Latent::from_color(color));
                    (color, k) = ([0.0; LAT], 0);
                }
            })?;
            wet.lat = lat;
        } else {
            get_into(r, n * width, |_| {})?;
            wet.lat = vec![crate::wet::Latent::default(); n];
        }
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
            v @ (1 | 2 | 3 | 4 | 5) => {
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
        let mut damp = false;
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
            c.sealed_drawing_guide = word & SEALED_DRAWING_GUIDE != 0;
            c.drawing_optics = word & DRAWING_OPTICS != 0;
            c.oils = word & OILS != 0;
            c.wet_pastel = word & WET_PASTEL != 0;
            damp = word & DAMP_PAPER != 0;
            match u32::try_from(word & !(OILS | WET_PASTEL | DAMP_PAPER | DRAWING_OPTICS | SEALED_DRAWING_GUIDE | LIQUID_FILM_LEVELS | QUARTER_TILES | ADAPTIVE_SWEEP | GLAZE_FILM | RAG_LIFT | KNIFE_SKIM | BLEND_FEATHER | BRUSH_SKIM)) {
                Ok(v) if crate::supported_engine(v) && (!(c.liquid_film_levels || c.oils) || v >= 4) && (!(c.quarter_tiles || c.adaptive_sweep) || v == 5) && (!(c.glaze_film || c.rag_lift || c.knife_skim || c.blend_feather || c.brush_skim) || matches!(v, crate::engine7::ENGINE | crate::REACH_END | crate::PIGMENT_MIXER)) => v,
                _ => return Err(bad("checkpoint engine version is invalid")),
            }
        };
        if c.drawing.as_ref().is_some_and(|d| d.optics() != c.drawing_optics) {
            return Err(bad("checkpoint drawing optics do not match its layout"));
        }
        // (the paper's sizing, from the wet pastel section, once the paper is read)
        let mut wet_cobb = None;
        if c.wet_pastel {
            if !c.drawing_optics || get_u64(r)? != WET_PASTEL_MARK || get_u64(r)? != WET_PASTEL_V {
                return Err(bad("checkpoint wet pastel section is invalid"));
            }
            match (get_u64(r)?, c.drawing.as_deref_mut()) {
                (0, None) => {}
                (1, Some(d)) => {
                    let wet = get_all(r, 3 * d.cells.len())?;
                    let u16_ok = |v: f32| v >= 0.0 && v <= u16::MAX as f32 && v.fract() == 0.0;
                    if !wet.chunks_exact(3).all(|q| q[0].is_finite() && q[0] >= 0.0 && u16_ok(q[1]) && u16_ok(q[2])) {
                        return Err(bad("checkpoint pastel water is invalid"));
                    }
                    for (cell, q) in d.cells.iter_mut().zip(wet.chunks_exact(3)) {
                        (cell.water, cell.index, cell.gum) = (q[0], q[1] as u16, q[2] as u16);
                    }
                    d.sheet = match get_u64(r)? {
                        0 => Vec::new(),
                        1 => get_all(r, d.cells.len())?,
                        _ => return Err(bad("checkpoint pastel sheet water flag is invalid")),
                    };
                    if !d.sheet.iter().all(|v| v.is_finite() && *v >= 0.0) {
                        return Err(bad("checkpoint pastel sheet water is invalid"));
                    }
                    d.damp = d.cells.iter().any(|c| c.water > 0.0) || d.sheet.iter().any(|&v| v > 0.0);
                }
                _ => return Err(bad("checkpoint wet pastel section disagrees with its drawing")),
            }
            let (kind, v) = (get_u64(r)?, get_f32(r)?);
            wet_cobb = Some(match kind {
                0 if v.is_finite() && v >= 0.0 => Some(v),
                1 => None,
                _ => return Err(bad("checkpoint paper sizing is invalid")),
            });
        }
        if damp {
            if get_u64(r)? != DAMP_MARK {
                return Err(bad("checkpoint damp sheet section is invalid"));
            }
            match get_u64(r)? {
                DAMP_V => {}
                v => return Err(bad(&format!("checkpoint damp sheet section is version {v}; this easel reads version {DAMP_V} only"))),
            }
            let water = f64::from_bits(get_u64(r)?);
            let faces = get_u64(r)?;
            if !(water.is_finite() && water >= 0.0) || faces > 2 {
                return Err(bad("checkpoint damp sheet's water is invalid"));
            }
            c.damp = Some(crate::damp::Damp { water, faces: faces as u8 });
        }
        if own_mixer && identity != if c.spectral_films() { crate::mixer::film_identity() } else if c.engine >= crate::PIGMENT_MIXER { crate::mixer::identity9() } else { crate::mixer::identity() } {
            return Err(bad("checkpoint uses an obsolete pigment representation; recreate the checkpoint with this build"));
        }
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
        // the oils section (the switch, `OILS`): the wax per pixel
        let mut wax = None;
        if c.oils {
            if get_u64(r)? != OILS_MARK {
                return Err(bad("checkpoint oils mark is wrong"));
            }
            match get_u64(r)? {
                OILS_V => {}
                v => return Err(bad(&format!("checkpoint oils section is version {v}; this easel reads version {OILS_V} only"))),
            }
            let w = get_all(r, n)?;
            if !w.iter().all(|v| v.is_finite()) {
                return Err(bad("checkpoint paint wax is invalid"));
            }
            for (h, &k) in c.wet.hide.iter_mut().zip(&w) {
                h[9] = k;
            }
            wax = Some(w);
            // (on base version 11 with nothing after it, the section ends the file)
            if version == 11 && !pigments && !layers && !spectral_backing && !material && r.read(&mut [0u8; 1])? != 0 {
                return Err(bad("checkpoint has trailing data"));
            }
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
                // (the oils section's wax and the drain section's are one)
                if wax.as_ref().is_some_and(|w| w.iter().zip(pack.as_chunks::<5>().0).any(|(a, k)| a.to_bits() != k[4].to_bits())) {
                    return Err(bad("checkpoint oils and drain sections disagree on the paint's wax"));
                }
                for (h, k) in c.wet.hide.iter_mut().zip(pack.as_chunks::<5>().0) {
                    h[5..].copy_from_slice(k);
                }
                c.drain = true;
                if !raw && version == 14 && !pigments && !spectral_backing && !material && r.read(&mut [0u8; 1])? != 0 {
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
            if version != 17 && !pigments && !spectral_backing && !material && r.read(&mut [0u8; 1])? != 0 {
                return Err(bad("checkpoint has trailing data"));
            }
        }
        // a sheet of paper's section, which ends the file
        if matches!(version, 13 | 16) {
            let (mut p, micro) = read_paper(r, n)?;
            if let Some(cobb) = wet_cobb {
                p.cobb60 = cobb;
            }
            c.paper = Some(p);
            c.micro = micro;
            if !pigments && !spectral_backing && !material && r.read(&mut [0u8; 1])? != 0 {
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
                let (mut p, micro) = read_paper_body(r, n)?;
                if let Some(cobb) = wet_cobb {
                    p.cobb60 = cobb;
                }
                c.paper = Some(p);
                c.micro = micro;
                mark = get_u64(r)?;
            }
            if mark != SHEET_MARK {
                return Err(bad("checkpoint sheet mark is wrong"));
            }
            c.sheet = Some(read_sheet_body(r, n)?);
            if !pigments && !spectral_backing && !material && r.read(&mut [0u8; 1])? != 0 {
                return Err(bad("checkpoint has trailing data"));
            }
        }
        if version == 7 && !pigments && !material && r.read(&mut [0u8; 1])? != 0 {
            return Err(bad("version-7 checkpoint has trailing data"));
        }
        if pigments {
            for pixel in &mut c.wet.lat {
                let count = usize::try_from(get_u64(r)?).map_err(|_| bad("checkpoint pigment count is invalid"))?;
                for _ in 0..count {
                    let i = usize::try_from(get_u64(r)?).map_err(|_| bad("checkpoint pigment is invalid"))?;
                    let amount = get_f32(r)?;
                    if !amount.is_finite() || amount < 0.0 || pixel.parts.iter().any(|p| p.0 == i) {
                        return Err(bad("checkpoint pigment list is invalid"));
                    }
                    pixel.parts.push((i, amount));
                }
            }
            if !layers && !spectral_backing && !material && r.read(&mut [0u8; 1])? != 0 {
                return Err(bad("checkpoint has trailing data"));
            }
        }
        if spectral_backing {
            layers = match get_u64(r)? {
                0 => false,
                1 => true,
                _ => return Err(bad("checkpoint spectral backing layer flag is invalid")),
            };
        }
        if layers {
            if c.engine < crate::PIGMENT_MIXER || !pigments { return Err(bad("checkpoint layers need engine 9 pigment storage")); }
            for pixel in &mut c.wet.lat {
                let count = get_u64(r)?;
                if count > 1 << 20 { return Err(bad("checkpoint layer count is invalid")); }
                for _ in 0..count {
                    let share = get_f32(r)?;
                    let color: [f32; LAT] = get_all(r, LAT)?.try_into().unwrap();
                    let hide: crate::wet::Prop = get_all(r, 10)?.try_into().unwrap();
                    if !share.is_finite() || share <= 0.0 || !color.iter().chain(&hide).all(|x| x.is_finite()) { return Err(bad("checkpoint layer is invalid")); }
                    let mut lat = crate::wet::Latent::from_color(color);
                    let parts = get_u64(r)?;
                    if parts > 1 << 20 { return Err(bad("checkpoint layer pigment count is invalid")); }
                    for _ in 0..parts {
                        let i = usize::try_from(get_u64(r)?).map_err(|_| bad("checkpoint layer pigment is invalid"))?;
                        let amount = get_f32(r)?;
                        if !amount.is_finite() || amount < 0.0 || lat.parts.iter().any(|p| p.0 == i) { return Err(bad("checkpoint layer pigment list is invalid")); }
                        lat.parts.push((i, amount));
                    }
                    pixel.layers.push(crate::wet::Layer { share, lat, hide });
                }
                let sum: f32 = pixel.layers.iter().map(|l| l.share).sum();
                if count > 0 && (sum - 1.0).abs() > 1e-4 { return Err(bad("checkpoint layer shares are invalid")); }
            }
            if !spectral_backing && !material && r.read(&mut [0u8; 1])? != 0 { return Err(bad("checkpoint has trailing data")); }
        }
        if spectral_backing {
            if !c.spectral_films() {
                return Err(bad("checkpoint spectral backing requires spectral film optics"));
            }
            c.backing.reserve_exact(n);
            let mut bytes = [0; (crate::spectral::N + 6) * 4];
            for _ in 0..n {
                r.read_exact(&mut bytes)?;
                let mut b = crate::mixer::Backing::default();
                for (v, bytes) in b.reflectance.iter_mut().chain(&mut b.residual).chain(&mut b.color).zip(bytes.chunks_exact(4)) {
                    *v = f32::from_le_bytes(bytes.try_into().unwrap());
                    if !v.is_finite() {
                        return Err(bad("checkpoint spectral backing is invalid"));
                    }
                }
                c.backing.push(b);
            }
            if !material && r.read(&mut [0u8; 1])? != 0 { return Err(bad("checkpoint has trailing data")); }
        }
        if material {
            if get_u64(r)? != SKIN_MARK || get_u64(r)? != SKIN_V {
                return Err(bad("checkpoint skin section is invalid"));
            }
            let rev = u32::try_from(get_u64(r)?).map_err(|_| bad("checkpoint material revision is invalid"))?;
            if rev < 1 || rev > crate::skin::MATERIAL || c.engine < crate::PIGMENT_MIXER {
                return Err(bad("checkpoint material revision is not one this build paints with"));
            }
            c.material = rev;
            let mut b = [0u8; 8];
            r.read_exact(&mut b)?;
            let ticked = f64::from_le_bytes(b);
            if !ticked.is_finite() {
                return Err(bad("checkpoint skin clock is invalid"));
            }
            let mut columns = Vec::with_capacity(n);
            for _ in 0..n {
                let count = usize::try_from(get_u64(r)?).map_err(|_| bad("checkpoint skin count is invalid"))?;
                if count > 1 << 16 {
                    return Err(bad("checkpoint skin count is invalid"));
                }
                let v = get_all(r, count * 10)?;
                if !v.iter().all(|x| x.is_finite()) {
                    return Err(bad("checkpoint skin film is invalid"));
                }
                columns.push(v.as_chunks::<10>().0.iter().map(|&[h, s, x, n, w, born, v0, stiff, binder, slope]| crate::skin::Packet { h, s, x, n, w, born, v0, stiff, binder, slope, below: u32::MAX }).collect());
            }
            c.skin = crate::skin::Film::from_columns(columns, ticked);
            if r.read(&mut [0u8; 1])? != 0 { return Err(bad("checkpoint has trailing data")); }
        }
        if !own_mixer {
            if c.wet.vol.iter().any(|&v| v > 0.0) {
                return Err(bad("checkpoint contains wet paint in an obsolete pigment representation; reopen it with the original engine build and export the painting, or replay its log with this build"));
            }
            c.wet.lat.fill(crate::wet::Latent::default());
        }
        if c.damp.is_some() && c.paper.is_none() {
            return Err(bad("checkpoint damp sheet section on a canvas that isn't paper"));
        }
        Ok((c, header))
    }

    /// Write the canvas state to `w` (`write_state`) and keep aside what it
    /// leaves out: `unpack` of the two gives this canvas back bit for bit,
    /// as a copy would, in far less memory. A copy of a painted canvas is
    /// the pixels' latents, pigment lists and skim layers over again; the
    /// written state is dense, mostly zero lanes, and compresses well.
    pub fn pack(&self, w: &mut impl Write) -> io::Result<Aside> {
        self.write_state(w, "")?;
        let wet = &self.wet;
        Ok(Aside {
            absorb_any: self.absorb_any,
            ground_um: self.ground_um,
            hand_slice: self.hand_slice,
            film_pass: self.film_pass,
            receptive: self.receptive.clone(),
            floor: wet.floor.clone(),
            solv: wet.solv.clone(),
            laid: wet.laid.clone(),
            laid_id: wet.laid_id.clone(),
            hide: (!self.drain).then(|| wet.hide.clone()),
            drawing: self.drawing.clone(),
            soak: self.soak.clone(),
            paper: self.paper,
            micro: self.micro.clone(),
            sheet: self.sheet.clone(),
        })
    }

    /// The canvas `pack` wrote to `r` and kept `aside`, bit for bit. The
    /// brushes' contact level is not kept: it is rebuilt from the relief
    /// to the same bits (`engine7::contact_cache`).
    pub fn unpack(r: &mut impl Read, aside: Aside) -> io::Result<Canvas> {
        let (mut c, _) = Canvas::read_state(r)?;
        c.absorb_any = aside.absorb_any;
        c.ground_um = aside.ground_um;
        c.hand_slice = aside.hand_slice;
        c.film_pass = aside.film_pass;
        c.receptive = aside.receptive;
        c.wet.floor = aside.floor;
        c.wet.solv = aside.solv;
        c.wet.laid = aside.laid;
        c.wet.laid_id = aside.laid_id;
        if let Some(hide) = aside.hide {
            c.wet.hide = hide;
        }
        c.drawing = aside.drawing;
        c.soak = aside.soak;
        c.paper = aside.paper;
        c.micro = aside.micro;
        c.sheet = aside.sheet;
        Ok(c)
    }
}

/// What `write_state` leaves out of a canvas, or reads back sanitized,
/// kept beside the written state by `Canvas::pack`: the film floors and
/// ceilings only the stroke that set them reads, the solvent buffers as
/// allocated, the five packing lanes of the paint's properties that only
/// the drain section writes, the values `read_state` clamps, a pass under
/// way, and the drawing, cloth, paper and sheet, which are cheap to copy.
/// (Cloned to put one packed canvas back more than once: the easel's
/// resting surfaces, whose packed state also serves a failed chunk's
/// rollback.)
#[derive(Clone)]
pub struct Aside {
    absorb_any: bool,
    ground_um: f32,
    hand_slice: Option<f32>,
    film_pass: Option<(f32, u32)>,
    receptive: Option<Vec<f32>>,
    floor: Vec<f32>,
    solv: Vec<f32>,
    laid: Vec<f32>,
    laid_id: Vec<u32>,
    hide: Option<Vec<crate::wet::Prop>>,
    drawing: Option<Box<crate::graphite::Drawing>>,
    soak: Option<Box<crate::soak::Soak>>,
    paper: Option<crate::paper::Paper>,
    micro: Vec<f32>,
    sheet: Option<crate::sheet::Sheet>,
}

/// A buffer written bit for bit after its length (u64; `u64::MAX`: none), for
/// what `Aside::spill` moves to disk.
pub(crate) fn spill_vec<T: Copy + SpillWord>(w: &mut impl Write, v: Option<&[T]>) -> io::Result<()> {
    let Some(v) = v else { return put_u64(w, u64::MAX) };
    put_u64(w, v.len() as u64)?;
    let mut buf = Vec::with_capacity(1 << 16);
    for x in v {
        buf.extend_from_slice(&x.word().to_le_bytes());
        if buf.len() >= 1 << 16 {
            w.write_all(&buf)?;
            buf.clear();
        }
    }
    w.write_all(&buf)
}

/// `spill_vec` read back.
pub(crate) fn unspill_vec<T: SpillWord>(r: &mut impl Read) -> io::Result<Option<Vec<T>>> {
    let n = get_u64(r)?;
    if n == u64::MAX {
        return Ok(None);
    }
    let mut v = Vec::with_capacity(n as usize);
    let mut b = [0u8; 4];
    for _ in 0..n {
        r.read_exact(&mut b)?;
        v.push(T::of(u32::from_le_bytes(b)));
    }
    Ok(Some(v))
}

/// A 32-bit value `spill_vec` writes as its bits.
pub(crate) trait SpillWord: Copy {
    fn word(self) -> u32;
    fn of(w: u32) -> Self;
}

impl SpillWord for f32 {
    fn word(self) -> u32 {
        self.to_bits()
    }
    fn of(w: u32) -> Self {
        f32::from_bits(w)
    }
}

impl SpillWord for u32 {
    fn word(self) -> u32 {
        self
    }
    fn of(w: u32) -> Self {
        w
    }
}

impl Aside {
    /// Move its large buffers (the film floors, the solvent buffers, the
    /// laid lanes, the receptive map, the paper's pores and the drawing) out
    /// to `w`, bit for bit, keeping the rest: what a canvas put away for a
    /// while keeps in memory beside its packed state is then a few scalars
    /// (the easel's resting surfaces). `unspill` puts them back.
    pub fn spill(&mut self, w: &mut impl Write) -> io::Result<()> {
        spill_vec(w, self.receptive.as_deref())?;
        for v in [&self.floor, &self.solv, &self.laid, &self.micro] {
            spill_vec(w, Some(v.as_slice()))?;
        }
        spill_vec(w, Some(self.laid_id.as_slice()))?;
        // (the paint's packing lanes, kept only without the drain switch: ten words a pixel)
        let hide: Option<Vec<f32>> = self.hide.as_ref().map(|h| h.iter().flatten().copied().collect());
        spill_vec(w, hide.as_deref())?;
        drop(hide);
        put_u64(w, self.drawing.is_some() as u64)?;
        if let Some(d) = &self.drawing {
            d.spill(w)?;
        }
        self.receptive = None;
        self.floor = Vec::new();
        self.solv = Vec::new();
        self.laid = Vec::new();
        self.micro = Vec::new();
        self.laid_id = Vec::new();
        self.hide = self.hide.as_ref().map(|_| Vec::new());
        self.drawing = None;
        Ok(())
    }

    /// The buffers `spill` moved to `r`, back in place.
    pub fn unspill(&mut self, r: &mut impl Read) -> io::Result<()> {
        let short = || bad("a spilled canvas buffer is missing");
        self.receptive = unspill_vec(r)?;
        self.floor = unspill_vec(r)?.ok_or_else(short)?;
        self.solv = unspill_vec(r)?.ok_or_else(short)?;
        self.laid = unspill_vec(r)?.ok_or_else(short)?;
        self.micro = unspill_vec(r)?.ok_or_else(short)?;
        self.laid_id = unspill_vec(r)?.ok_or_else(short)?;
        let hide: Option<Vec<f32>> = unspill_vec(r)?;
        if hide.is_some() != self.hide.is_some() || hide.as_ref().is_some_and(|h| h.len() % 10 != 0) {
            return Err(bad("a spilled canvas's packing lanes don't fit it"));
        }
        self.hide = hide.map(|h| h.as_chunks::<10>().0.to_vec());
        self.drawing = match get_u64(r)? {
            0 => None,
            1 => Some(Box::new(crate::graphite::Drawing::unspill(r)?)),
            _ => return Err(bad("a spilled drawing's flag is invalid")),
        };
        Ok(())
    }

    /// The bytes it keeps in memory beside the packed state (its buffers;
    /// the few scalars and a laid sheet's mask aside): what a canvas put
    /// away costs in memory besides its compressed file (the easel's
    /// resting surfaces).
    pub fn heap_bytes(&self) -> usize {
        let f = |v: &Vec<f32>| v.capacity() * 4;
        self.receptive.as_ref().map_or(0, f) + f(&self.floor) + f(&self.solv) + f(&self.laid) + self.laid_id.capacity() * 4
            + self.hide.as_ref().map_or(0, |h| h.capacity() * std::mem::size_of::<crate::wet::Prop>())
            + self.drawing.as_ref().map_or(0, |d| d.heap_bytes())
            + f(&self.micro)
    }
}

// ---------------------------------------------------------------- surfaces

/// A checkpoint of several named surfaces (the easel's `--@ surfaces`: a
/// painting with a study sheet beside it, and later a plate), version 1:
/// `SURFACES_MAGIC`, the section's version (`SURFACES_V`, one u64), the
/// caller's header (length-prefixed UTF-8, as a canvas's), the name of the
/// surface that is current, the number of surfaces, then each surface: its
/// name, its kind (one u64, the caller's), its fields (length-prefixed
/// UTF-8, the caller's `key=value` lines) and its canvas, a whole canvas
/// checkpoint (`write_state`, empty header) with its own magic and
/// version, framed: chunks of at most `FRAME` bytes, each after its length
/// (u64), ended by a chunk of length 0. The frame is what lets a canvas
/// whose own format must end its file (the skin section of version 22)
/// stand before the next surface: its reader sees the frame's end as the
/// file's.
///
/// Only a session that has named a surface writes one: a session with the
/// painting alone writes its canvas checkpoint exactly as before. A reader
/// that doesn't know this magic refuses the file (as one that doesn't know
/// version 22 refuses a material canvas) instead of loading the painting
/// without its other sheets, and a section of another version is refused.
const SURFACES_MAGIC: &[u8; 8] = b"PAINTSRF";
pub const SURFACES_V: u64 = 1;
/// The longest chunk of a framed canvas (bytes).
const FRAME: usize = 1 << 20;
/// The most surfaces a checkpoint holds, and the longest name (bytes).
pub const SURFACES_MAX: usize = 64;
pub const SURFACE_NAME_MAX: usize = 64;

/// One surface of a multi-surface checkpoint, but its canvas.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceEntry {
    pub name: String,
    pub kind: u64,
    pub fields: String,
}

/// Whether a checkpoint starting with these 8 bytes holds several surfaces
/// (`write_surfaces`) rather than one canvas.
pub fn is_surfaces(magic: &[u8]) -> bool {
    magic.len() >= 8 && &magic[..8] == SURFACES_MAGIC
}

fn put_str(w: &mut impl Write, s: &str) -> io::Result<()> {
    put_u64(w, s.len() as u64)?;
    w.write_all(s.as_bytes())
}

fn get_str(r: &mut impl Read, limit: usize, what: &str) -> io::Result<String> {
    let n = get_u64(r)?;
    if n > limit as u64 {
        return Err(bad(&format!("surfaces checkpoint: {what} too long")));
    }
    let mut b = vec![0u8; n as usize];
    r.read_exact(&mut b)?;
    String::from_utf8(b).map_err(|_| bad(&format!("surfaces checkpoint: {what} is not UTF-8")))
}

/// Writes what it is given as length-prefixed chunks (`write_surfaces`).
struct Framed<'a, W: Write> {
    w: &'a mut W,
    buf: Vec<u8>,
}

impl<W: Write> Framed<'_, W> {
    fn flush_chunk(&mut self) -> io::Result<()> {
        if !self.buf.is_empty() {
            put_u64(self.w, self.buf.len() as u64)?;
            self.w.write_all(&self.buf)?;
            self.buf.clear();
        }
        Ok(())
    }
}

impl<W: Write> Write for Framed<'_, W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let take = bytes.len().min(FRAME - self.buf.len());
        self.buf.extend_from_slice(&bytes[..take]);
        if self.buf.len() == FRAME {
            self.flush_chunk()?;
        }
        Ok(take)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Reads a framed canvas, and ends where its frame does.
struct Unframed<'a, R: Read> {
    r: &'a mut R,
    left: u64,
    done: bool,
}

impl<R: Read> Read for Unframed<'_, R> {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        while self.left == 0 {
            if self.done || out.is_empty() {
                return Ok(0);
            }
            self.left = get_u64(self.r)?;
            if self.left == 0 {
                self.done = true;
                return Ok(0);
            }
            if self.left > FRAME as u64 {
                return Err(bad("surfaces checkpoint: a canvas's chunk is too long"));
            }
        }
        let take = (self.left as usize).min(out.len());
        let n = self.r.read(&mut out[..take])?;
        if n == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "surfaces checkpoint: a canvas ends early"));
        }
        self.left -= n as u64;
        Ok(n)
    }
}

/// Write a checkpoint of several surfaces (`SURFACES_MAGIC`): `header`, the
/// current surface's name, and each surface of `entries` with the canvas
/// `canvas(i, w)` writes for it, which must be a whole canvas checkpoint
/// with an empty header (`Canvas::write_state(w, "")`, or the bytes of a
/// packed canvas, `Canvas::pack`, copied as they are).
pub fn write_surfaces(w: &mut impl Write, header: &str, current: &str, entries: &[SurfaceEntry], mut canvas: impl FnMut(usize, &mut dyn Write) -> io::Result<()>) -> io::Result<()> {
    let invalid = |m: &str| io::Error::new(io::ErrorKind::InvalidInput, format!("surfaces checkpoint: {m}"));
    if entries.is_empty() || entries.len() > SURFACES_MAX {
        return Err(invalid("1 to 64 surfaces"));
    }
    for (i, e) in entries.iter().enumerate() {
        if e.name.is_empty() || e.name.len() > SURFACE_NAME_MAX || entries[..i].iter().any(|o| o.name == e.name) {
            return Err(invalid("each surface has a name of its own, 1 to 64 bytes"));
        }
    }
    if !entries.iter().any(|e| e.name == current) {
        return Err(invalid("the current surface is one of them"));
    }
    w.write_all(SURFACES_MAGIC)?;
    put_u64(w, SURFACES_V)?;
    put_str(w, header)?;
    put_str(w, current)?;
    put_u64(w, entries.len() as u64)?;
    for (i, e) in entries.iter().enumerate() {
        put_str(w, &e.name)?;
        put_u64(w, e.kind)?;
        put_str(w, &e.fields)?;
        let mut f = Framed { w: &mut *w, buf: Vec::with_capacity(FRAME) };
        canvas(i, &mut f)?;
        f.flush_chunk()?;
        put_u64(w, 0)?;
    }
    Ok(())
}

/// Read a checkpoint of several surfaces (`write_surfaces`), handing each
/// surface's entry and canvas to `each` in turn as it is read (so the
/// caller can put one away before the next is read). Returns the header
/// and the name of the current surface. Refuses another version of the
/// section, a damaged frame, a canvas that doesn't end where its frame
/// does, and data after the last surface.
pub fn read_surfaces(r: &mut impl Read, mut each: impl FnMut(SurfaceEntry, Canvas) -> io::Result<()>) -> io::Result<(String, String)> {
    let mut m = [0u8; 8];
    r.read_exact(&mut m)?;
    if !is_surfaces(&m) {
        return Err(bad("not a surfaces checkpoint"));
    }
    let v = get_u64(r)?;
    if v != SURFACES_V {
        return Err(bad(&format!("surfaces checkpoint of version {v}: this build reads version {SURFACES_V}")));
    }
    let header = get_str(r, 1 << 20, "header")?;
    let current = get_str(r, SURFACE_NAME_MAX, "current surface's name")?;
    let n = get_u64(r)?;
    if n == 0 || n > SURFACES_MAX as u64 {
        return Err(bad("surfaces checkpoint: 1 to 64 surfaces"));
    }
    let mut names: Vec<String> = Vec::new();
    for _ in 0..n {
        let name = get_str(r, SURFACE_NAME_MAX, "surface name")?;
        if name.is_empty() || names.contains(&name) {
            return Err(bad("surfaces checkpoint: a surface without a name of its own"));
        }
        let kind = get_u64(r)?;
        let fields = get_str(r, 1 << 20, "surface fields")?;
        let mut u = Unframed { r: &mut *r, left: 0, done: false };
        let (canvas, head) = Canvas::read_state(&mut u)?;
        if !head.is_empty() || u.read(&mut [0u8; 1])? != 0 || !u.done {
            return Err(bad("surfaces checkpoint: a surface's canvas doesn't end with its frame"));
        }
        names.push(name.clone());
        each(SurfaceEntry { name, kind, fields }, canvas)?;
    }
    if !names.contains(&current) {
        return Err(bad("surfaces checkpoint: the current surface isn't among them"));
    }
    if r.read(&mut [0u8; 1])? != 0 {
        return Err(bad("surfaces checkpoint has trailing data"));
    }
    Ok((header, current))
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

    fn base_format(b: &[u8]) -> u64 {
        assert_eq!(&b[..8], MAGIC19);
        let header = u64::from_le_bytes(b[8..16].try_into().unwrap()) as usize;
        assert_eq!(u64::from_le_bytes(b[16 + header..24 + header].try_into().unwrap()), crate::mixer::identity());
        u64::from_le_bytes(b[24 + header..32 + header].try_into().unwrap())
    }

    fn set_base_format(b: &mut [u8], version: u64) {
        base_format(b);
        let header = u64::from_le_bytes(b[8..16].try_into().unwrap()) as usize;
        set(b, 24 + header, version);
    }

    #[test]
    fn a_different_mixer_identity_is_refused_before_reading_latents() {
        let c = Canvas::new_window(2, 1.0, [0.1; 3], None);
        let mut bytes = Vec::new();
        c.write_state(&mut bytes, "").unwrap();
        assert!(load(bytes.clone()).is_ok());
        for identity in [1, crate::mixer::identity() ^ 1] {
            set(&mut bytes, 16, identity);
            let error = load(bytes.clone()).err().unwrap();
            assert!(error.to_string().contains("obsolete pigment representation"));
        }
    }

    #[test]
    fn legacy_checkpoints_and_dumps_keep_twenty_one_lanes() {
        for engine in (1..=8).filter(|&engine| crate::supported_engine(engine)) {
            let mut canvas = Canvas::new_window(2, 1.0, [0.1; 3], None).with_engine(engine);
            let color = crate::mixer::encode(&[0.2, 0.4, 0.6]);
            canvas.wet.vol[0] = 1.0;
            canvas.wet.lat[0] = crate::wet::Latent::from_color(color);
            let mut bytes = Vec::new();
            canvas.write_state(&mut bytes, "").unwrap();
            canvas.wet.lat[0].color[crate::mixer::LEGACY_LATENT_SIZE..].fill(1.0);
            let mut with_extra = Vec::new();
            canvas.write_state(&mut with_extra, "").unwrap();
            assert_eq!(with_extra, bytes);
            let restored = load(bytes.clone()).unwrap();
            assert_eq!(restored.wet.lat[0].color, color);
            let mut again = Vec::new();
            restored.write_state(&mut again, "").unwrap();
            assert_eq!(again, bytes);
            let fields = restored.state_fields();
            let lat = fields.iter().find(|f| f.name == "wet.lat").unwrap();
            assert_eq!(lat.shape, [2, 2, crate::mixer::LEGACY_LATENT_SIZE]);
            assert_eq!(lat.values.le_bytes().len(), 4 * 4 * crate::mixer::LEGACY_LATENT_SIZE);
        }
    }

    #[test]
    fn old_engine_nine_identity_is_rejected_without_a_payload() {
        use sha2::{Digest, Sha256};
        let mut hash = Sha256::new();
        hash.update(crate::mixer::identity().to_le_bytes());
        hash.update(b"extensive-pigment-and-residual/volume-strength/v1");
        hash.update(b"opacity-constrained-rgb-tube-proxies/v2");
        hash.update(b"physical-volume-ks/dark-calibration/v1");
        hash.update(include_bytes!("mixer/calibrated.bin"));
        hash.update(include_bytes!("mixer/calibrated-densities.bin"));
        let old: u64 = u64::from_le_bytes(hash.finalize()[..8].try_into().unwrap());
        let mut film = Sha256::new();
        film.update(old.to_le_bytes());
        film.update(b"film25um/spectral-dry/transmitted-residual/v1");
        let old_film = u64::from_le_bytes(film.finalize()[..8].try_into().unwrap());
        for identity in [old, old_film] {
            let mut bytes = MAGIC19.to_vec();
            put_u64(&mut bytes, 0).unwrap();
            put_u64(&mut bytes, identity).unwrap();
            let error = load(bytes).err().unwrap();
            assert_eq!(error.kind(), io::ErrorKind::InvalidData);
            assert!(error.to_string().contains("obsolete pigment representation"));
        }
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
        assert_eq!((base_format(&off), base_format(&on)), (11, 14));
        assert!(on[40..off.len()] == off[40..], "version 11's payload, then the drain section");
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

    /// A canvas with the oils switch says so in its engine word (`OILS`)
    /// and has, after the solvent, the oils section (the wax per pixel),
    /// then the drain section if it drains too; its base version is the one
    /// it would otherwise be (11, or 14 draining). It keeps the switch and
    /// its paint's wax and is rewritten as it was read. The section is
    /// refused with a wrong mark or version, a non-finite wax, a wax the
    /// drain section's disagrees with, or data after it; and the switch on
    /// a canvas before engine 4.
    #[test]
    fn the_oils_switch_has_its_own_section() {
        let canvas = |oils: bool, drain: bool| {
            let mut c = Canvas::new_window(2, 1.0, [0.1; 3], None).with_engine(5).with_oils(oils).with_drain(drain);
            c.absorb.iter_mut().for_each(|a| *a = 0.3);
            c.wet.vol[0] = 1.5;
            c.wet.hide[0] = [0.7, 0.6, 1.2, 0.4, 0.5, 0.45, 0.3, 0.25, 0.6, 0.2];
            let mut b = Vec::new();
            c.write_state(&mut b, "").unwrap();
            b
        };
        let (off, on, drain, both) = (canvas(false, false), canvas(true, false), canvas(false, true), canvas(true, true));
        assert_eq!([&off, &on, &drain, &both].map(|b| base_format(b)), [11, 11, 14, 14]);
        let n = 4;
        let section = 16 + n * 4;
        // (the same bytes but the engine word's `OILS` bit, then the section)
        for (plain, oiled) in [(&off, &on), (&drain, &both)] {
            let at = off.len();
            assert_eq!(oiled.len(), plain.len() + section);
            let differ: Vec<usize> = (0..at).filter(|&i| plain[i] != oiled[i]).collect();
            assert!(differ.len() == 1 && plain[differ[0]] ^ oiled[differ[0]] == 1 << 3, "only the OILS bit differs: {differ:?}");
            assert!(oiled[at + section..] == plain[at..], "and the drain section after it");
        }
        let at = off.len();
        let (p, f, q) = (crate::wet::PACKED_OIL, crate::wet::DRAINED_FLOOR, crate::wet::OIL_VOLUME);
        let d = load(on.clone()).unwrap();
        assert!(d.oils && !d.drain && d.wet.hide[0] == [0.7, 0.6, 1.2, 0.4, 0.5, p, f, 0.0, q, 0.2], "{:?}", d.wet.hide[0]);
        let e = load(both.clone()).unwrap();
        assert!(e.oils && e.drain && e.wet.hide[0] == [0.7, 0.6, 1.2, 0.4, 0.5, 0.45, 0.3, 0.25, 0.6, 0.2]);
        for (d, b) in [(d, &on), (e, &both)] {
            let mut again = Vec::new();
            d.write_state(&mut again, "").unwrap();
            assert!(&again == b, "rewritten as it was read");
        }
        assert!(!load(off.clone()).unwrap().oils && !load(drain.clone()).unwrap().oils);
        for (b, pos, v) in [(&on, at, 0u64), (&on, at + 8, 2), (&both, at, 0)] {
            let mut b = b.clone();
            set(&mut b, pos, v);
            assert!(load(b).is_err(), "byte {pos} = {v}");
        }
        let mut b = on.clone();
        b[at + 16..at + 20].copy_from_slice(&f32::NAN.to_le_bytes());
        assert!(load(b).is_err(), "a NaN wax");
        let mut b = both.clone();
        b[at + 16..at + 20].copy_from_slice(&0.5f32.to_le_bytes());
        assert!(load(b).is_err(), "the drain section's wax disagrees");
        let mut b = on.clone();
        b.push(0);
        assert!(load(b).is_err(), "trailing data");
        let c = Canvas::new_window(2, 1.0, [0.1; 3], None).with_engine(3).with_oils(true);
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
        assert_eq!(base_format(&bytes), 16);
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
                let mut c = Canvas::new_window(32, 1.0, [0.5; 3], None).with_size_mm(8.0).with_drain(drain);
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
                assert_eq!(base_format(&bytes), if drain { 17 } else { 15 });
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

    /// A gritted sheet's paper section is version 2 with the grit's extras
    /// bit and comes back with its grit, its tooth and the pastel in it, bit
    /// for bit, and the next stroke lays the same (the stick wears against
    /// the grit as before); the same sheet without grit writes version 1. An
    /// unknown binder, a NaN grit, a section version this reader doesn't
    /// know, an extras word of 0, an unknown extras bit and a drum bit with
    /// no drum's block are refused. (Named so the gate's pastel checkpoint
    /// filter runs it.)
    #[test]
    fn paper_and_stick_pastel_survive_a_checkpoint_on_a_gritted_sheet() {
        use crate::paper::{Binder, Grit, Paper};
        use crate::pastel::{Pose, Stick, StrokePoint};
        let grit = Grit { size_um: 46.2, cover: 0.55, binder: Binder::Glue };
        let mut c = Canvas::new_window(64, 1.0, [0.5; 3], None).with_size_mm(40.0).with_paper(Paper { grit: Some(grit), ..Paper::drawing(3) });
        let mut st = Stick::round([0.7, 0.2, 0.1], 0.8, 12.0);
        let pose = Pose { force: 1.5, alt: 1.0, az: 0.8, roll: 0.0 };
        let line = |y: f32| [StrokePoint { x: 100.0, y, pose, speed: 60.0 }, StrokePoint { x: 900.0, y, pose, speed: 60.0 }];
        c.stick_stroke(&mut st, &line(400.0));
        c.stick_stroke(&mut st, &line(420.0));
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        assert_eq!(base_format(&b), 13);
        let mut d = load(b.clone()).unwrap();
        assert!(d.paper == c.paper && d.micro == c.micro && d.absorb == c.absorb && d.pixels() == c.pixels());
        let mut again = Vec::new();
        d.write_state(&mut again, "").unwrap();
        assert!(again == b, "rewritten as it was read");
        let mut st2 = st.clone();
        let (a1, a2) = (c.stick_stroke(&mut st, &line(410.0)), d.stick_stroke(&mut st2, &line(410.0)));
        assert!(d.pixels() == c.pixels() && st == st2 && a1.volume_mm3 == a2.volume_mm3);
        let n = c.f.w * c.f.h;
        // (the section, the file's last: mark, version, 12 numbers, seed,
        // felt, laid, the extras word, the grit, then the pores)
        let grit_at = b.len() - (2 * 4 + 8 + 4 * n);
        let extras_at = grit_at - 8;
        let section = extras_at - (8 + 8 + 12 * 4 + 8 + (8 + 2 * 4) + 8);
        assert_eq!(&b[section..section + 8], &PAPER_MARK.to_le_bytes());
        assert_eq!(&b[section + 8..section + 16], &PAPER_V2.to_le_bytes());
        assert_eq!(&b[extras_at..extras_at + 8], &PAPER_EXTRA_GRIT.to_le_bytes());
        let mut plain = Vec::new();
        Canvas::new_window(64, 1.0, [0.5; 3], None).with_size_mm(40.0).with_paper(Paper::drawing(3)).write_state(&mut plain, "").unwrap();
        let plain_at = plain.len() - (8 + 8 + 12 * 4 + 8 + (8 + 2 * 4) + 8 + 4 * n);
        assert_eq!(&plain[plain_at + 8..plain_at + 16], &PAPER_V.to_le_bytes(), "no extras: version 1");
        let mut binder = b.clone();
        set(&mut binder, grit_at + 8, 7);
        rejected(binder, "unknown binder");
        let mut nan = b.clone();
        nan[grit_at..grit_at + 4].copy_from_slice(&f32::NAN.to_le_bytes());
        rejected(nan, "NaN grit");
        let mut version = b.clone();
        set(&mut version, section + 8, 3);
        rejected(version, "paper section version 3");
        let mut none = b.clone();
        set(&mut none, extras_at, 0);
        rejected(none, "version 2 with extras 0");
        let mut unknown = b.clone();
        set(&mut unknown, extras_at, PAPER_EXTRA_GRIT | 4);
        rejected(unknown, "an unknown extras bit");
        let mut drum = b.clone();
        set(&mut drum, extras_at, PAPER_EXTRA_GRIT | PAPER_EXTRA_DRUM);
        rejected(drum, "a drum bit with no drum's block (the pores read as one)");
    }

    /// A sheet on a strainer writes its paper section as version 2, with an
    /// extras word of just the drum's bit and the drum's block before the
    /// pores, and comes back bit for bit, giving as it gave; a sheet on
    /// board or canvas (no extras) writes version 1, as every sheet did
    /// before. Refused: an extras bit no build knows, an extras word of 0 in
    /// version 2, a grit bit with no grit's block (the drum's bytes read as
    /// one), a NaN in the drum, too many crossbars.
    #[test]
    fn a_sheet_on_a_strainer_keeps_its_drum() {
        use crate::paper::{Mount, Paper};
        let section_version = |b: &[u8]| {
            let at = b.windows(8).rposition(|w| w == PAPER_MARK.to_le_bytes()).unwrap();
            u64::from_le_bytes(b[at + 8..at + 16].try_into().unwrap())
        };
        let strainer = Mount::Strainer { bar_mm: 30.0, cross: 1 };
        for (m, v) in [(Mount::Board, PAPER_V), (Mount::Canvas, PAPER_V), (Mount::named("strainer").unwrap(), PAPER_V2), (strainer, PAPER_V2)] {
            let c = Canvas::new_window(240, 1.5, [0.5; 3], None).with_size_mm(400.0).with_paper(Paper::drawing(2).mounted(m));
            let mut b = Vec::new();
            c.write_state(&mut b, "").unwrap();
            assert_eq!(base_format(&b), 13);
            assert_eq!(section_version(&b), v, "{m:?}");
            let d = load(b.clone()).unwrap();
            assert!(d.paper == c.paper && d.micro == c.micro, "{m:?}");
            let n = c.f.w * c.f.h;
            assert!((0..n).all(|i| d.give_mpa_per_um(i).to_bits() == c.give_mpa_per_um(i).to_bits()), "{m:?}");
            let mut again = Vec::new();
            d.write_state(&mut again, "").unwrap();
            assert!(again == b, "{m:?}: rewritten as it was read");
            if v != PAPER_V2 {
                continue;
            }
            // the extras word, then the drum's block (four f32, a u64), then the pores
            let drum_at = b.len() - 4 * n - 24;
            let extras_at = drum_at - 8;
            assert_eq!(u64::from_le_bytes(b[extras_at..drum_at].try_into().unwrap()), PAPER_EXTRA_DRUM);
            let says = |extras: u64| {
                let mut x = b.clone();
                x[extras_at..drum_at].copy_from_slice(&extras.to_le_bytes());
                match load(x) {
                    Ok(_) => panic!("extras {extras:#x}: accepted"),
                    Err(e) => {
                        assert_eq!(e.kind(), io::ErrorKind::InvalidData);
                        e.to_string()
                    }
                }
            };
            assert!(says(PAPER_EXTRA_DRUM | 4).contains("bits this easel doesn't know"));
            assert!(says(8).contains("bits this easel doesn't know"));
            assert!(says(0).contains("version 2 with no extras"));
            // (a grit block read from the drum's bytes: its binder word is
            // the bending length's and patch's bits, no binder)
            assert!(says(PAPER_EXTRA_GRIT | PAPER_EXTRA_DRUM).contains("binder"));
            assert!(says(PAPER_EXTRA_GRIT).contains("binder"));
            for (off, what) in [(0, "a NaN bar"), (4, "a NaN bay"), (8, "a NaN bending length"), (12, "a NaN patch")] {
                let mut nan = b.clone();
                nan[drum_at + off..drum_at + off + 4].copy_from_slice(&f32::NAN.to_le_bytes());
                rejected(nan, what);
            }
            let mut many = b.clone();
            many[drum_at + 16..drum_at + 24].copy_from_slice(&(MAX_CROSS + 1).to_le_bytes());
            rejected(many, "too many crossbars");
        }
    }

    /// A gritted sheet on a strainer has both extras: its paper section is
    /// version 2 with the grit's and the drum's bits, the grit's block then
    /// the drum's, then the pores. It comes back bit for bit, giving as it
    /// gave, and the next stroke lays the same on both.
    #[test]
    fn a_gritted_sheet_on_a_strainer_keeps_its_grit_and_drum() {
        use crate::paper::{Binder, Grit, Mount, Paper};
        use crate::pastel::{Pose, Stick, StrokePoint};
        let grit = Grit { size_um: 35.0, cover: 0.6, binder: Binder::Acrylic };
        let paper = Paper { grit: Some(grit), ..Paper::drawing(5) }.mounted(Mount::Strainer { bar_mm: 30.0, cross: 1 });
        assert!(paper.grit.is_some() && paper.drum.is_some());
        let mut c = Canvas::new_window(96, 1.0, [0.5; 3], None).with_size_mm(60.0).with_paper(paper);
        let mut st = Stick::round([0.2, 0.3, 0.7], 0.8, 12.0);
        let pose = Pose { force: 1.5, alt: 1.0, az: 0.8, roll: 0.0 };
        let line = |y: f32| [StrokePoint { x: 100.0, y, pose, speed: 60.0 }, StrokePoint { x: 900.0, y, pose, speed: 60.0 }];
        c.stick_stroke(&mut st, &line(400.0));
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        assert_eq!(base_format(&b), 13);
        let n = c.f.w * c.f.h;
        // (mark, version, 12 numbers, seed, felt, laid (none), extras, grit
        // (two f32, a u64), drum (four f32, a u64), the pores)
        let drum_at = b.len() - 4 * n - 24;
        let grit_at = drum_at - 16;
        let extras_at = grit_at - 8;
        assert_eq!(u64::from_le_bytes(b[extras_at..grit_at].try_into().unwrap()), PAPER_EXTRA_GRIT | PAPER_EXTRA_DRUM);
        assert_eq!(f32::from_le_bytes(b[grit_at..grit_at + 4].try_into().unwrap()), 35.0);
        assert_eq!(f32::from_le_bytes(b[drum_at..drum_at + 4].try_into().unwrap()), 30.0);
        let section = b.windows(8).rposition(|w| w == PAPER_MARK.to_le_bytes()).unwrap();
        assert_eq!(&b[section + 8..section + 16], &PAPER_V2.to_le_bytes());
        let mut d = load(b.clone()).unwrap();
        assert!(d.paper == c.paper && d.micro == c.micro && d.absorb == c.absorb && d.pixels() == c.pixels());
        assert!((0..n).all(|i| d.give_mpa_per_um(i).to_bits() == c.give_mpa_per_um(i).to_bits()));
        let mut again = Vec::new();
        d.write_state(&mut again, "").unwrap();
        assert!(again == b, "rewritten as it was read");
        let mut st2 = st.clone();
        let (a1, a2) = (c.stick_stroke(&mut st, &line(430.0)), d.stick_stroke(&mut st2, &line(430.0)));
        assert!(d.pixels() == c.pixels() && st == st2 && a1.volume_mm3 == a2.volume_mm3);
        let mut unknown = b.clone();
        set(&mut unknown, extras_at, PAPER_EXTRA_GRIT | PAPER_EXTRA_DRUM | 4);
        rejected(unknown, "an unknown extras bit beside grit and drum");
        let mut none = b.clone();
        set(&mut none, extras_at, 0);
        rejected(none, "version 2 with extras 0");
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
        assert_eq!(base_format(&b), 13);
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
        assert_eq!(base_format(&linen), 11);
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
        set_base_format(&mut no_section, 11);
        assert!(load(no_section.clone()).is_ok());
        set_base_format(&mut no_section, 13);
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

    /// Pastel dusted off a fixed layer, on a drawing without separate
    /// optics (a log from before `--@ drawing-optics`), can keep a rounding
    /// residue of loose pastel just below zero; the checkpoint reads back as
    /// it was written and goes on as the painting would have.
    #[test]
    fn dusted_pastel_survives_a_checkpoint() {
        use crate::pastel::{Pose, Stick, StrokePoint};
        let mut c = crate::tests::drawing_paper(160);
        let mut st = Stick::round([0.85, 0.55, 0.6], 0.8, 12.0);
        let pose = Pose { force: 2.5, alt: 1.0, az: 0.8, roll: 0.0 };
        let line = |y: f32| [StrokePoint { x: 100.0, y, pose, speed: 60.0 }, StrokePoint { x: 900.0, y, pose, speed: 60.0 }];
        c.stick_stroke(&mut st, &line(300.0));
        c.fix_pastel(None, 0.15);
        c.stick_stroke(&mut st, &line(320.0));
        c.dust_pastel(20.0, 10.0, &[(100.0, 310.0), (900.0, 310.0)], &mut 0.0, 5.0);
        assert!(c.drawing.as_ref().unwrap().cells.iter().any(|c| c.loose < 0.0), "the brush left a residue below zero");
        let bytes = |c: &Canvas| {
            let mut b = Vec::new();
            c.write_state(&mut b, "").unwrap();
            b
        };
        let b = bytes(&c);
        let (mut r, _) = Canvas::read_state(&mut Cursor::new(b.clone())).unwrap();
        assert!(bytes(&r) == b, "rewritten as it was read");
        let mut st2 = st.clone();
        c.stick_stroke(&mut st, &line(340.0));
        r.stick_stroke(&mut st2, &line(340.0));
        assert!(bytes(&r) == bytes(&c) && st == st2);
    }

    /// A sheet of paper (version 13), and one with a sheet laid on it
    /// (version 15), carry the recorded quarter tiles and adaptive sweep in
    /// their engine word, as every other version does.
    #[test]
    fn a_sheet_of_paper_keeps_the_recorded_settings_in_its_engine_word() {
        let c = Canvas::new_window(60, 1.5, [0.5; 3], None).with_engine(5).with_size_mm(120.0).with_paper(crate::paper::Paper::drawing(2)).with_quarter_tiles(true).with_adaptive_sweep(true);
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        assert_eq!(base_format(&b), 13);
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
        assert_eq!(base_format(&b), 15);
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
        assert_eq!(base_format(&bare), 13);
        assert!(load(bare.clone()).unwrap().sheet.is_none());
        let m = crate::mask::Mask::from_fn(c.frame(), |x, _| if x < 500.0 { 1.0 } else { 0.0 });
        c.lay_sheet(&m, 100.0, 5.0, [0.8, 0.78, 0.7]).unwrap();
        let mut st = Stick::round([0.7, 0.2, 0.1], 0.8, 12.0);
        let pose = Pose { force: 2.0, alt: 1.0, az: 0.8, roll: 0.0 };
        c.stick_stroke(&mut st, &[StrokePoint { x: 100.0, y: 300.0, pose, speed: 60.0 }, StrokePoint { x: 900.0, y: 300.0, pose, speed: 60.0 }]);
        assert!(c.sheet.as_ref().unwrap().a.iter().any(|&a| a > 0.0), "the stroke caught on the sheet");
        let b = bytes(&c);
        assert_eq!(base_format(&b), 15);
        let r = load(b.clone()).unwrap();
        assert!(r.sheet.is_some() && r.sheet == c.sheet && r.paper == c.paper && r.micro == c.micro && r.pixels() == c.pixels());
        assert!(bytes(&r) == b, "rewritten as it was read");
        // on linen: version 11's bytes, then the sheet
        let mut linen = Canvas::new_window(240, 1.5, [0.5; 3], None).with_size_mm(120.0);
        linen.lay_sheet(&m, 100.0, 5.0, [0.8, 0.78, 0.7]).unwrap();
        let lb = bytes(&linen);
        assert_eq!(base_format(&lb), 15);
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
        assert_eq!(base_format(&bytes(&c)), 13);
    }

    fn state(c: &Canvas) -> Vec<u8> {
        let mut b = Vec::new();
        c.write_state(&mut b, "").unwrap();
        b
    }

    /// The canvases of a multi-surface checkpoint (the easel's `--@ surfaces`).
    fn three_surfaces() -> Vec<(SurfaceEntry, Canvas)> {
        // the painting: an engine-9 material canvas, whose skin section ends its own
        // format; a sheet of paper with pastel on it; a bare sheet
        let painting = Canvas::new(24, 1.5, [0.8; 3]).with_engine(9).with_size_mm(6.0).with_material(1);
        let mut study = Canvas::new_window(64, 1.25, [0.9; 3], None).with_size_mm(80.0).with_paper(crate::paper::Paper::drawing(4));
        let mut st = crate::pastel::Stick::round([0.2, 0.3, 0.6], 0.8, 12.0);
        let pose = crate::pastel::Pose { force: 2.0, alt: 1.0, az: 0.8, roll: 0.0 };
        study.stick_stroke(&mut st, &[crate::pastel::StrokePoint { x: 100.0, y: 300.0, pose, speed: 60.0 }, crate::pastel::StrokePoint { x: 800.0, y: 400.0, pose, speed: 60.0 }]);
        let japan = Canvas::new_window(40, 0.8, [0.95; 3], None).with_size_mm(50.0).with_paper(crate::paper::Paper::drawing(9));
        let e = |name: &str, kind: u64, fields: &str| SurfaceEntry { name: name.into(), kind, fields: fields.into() };
        vec![(e("painting", 0, ""), painting), (e("study", 1, "since=1\n"), study), (e("japan", 1, "since=2\n"), japan)]
    }

    fn surfaces_bytes(all: &[(SurfaceEntry, Canvas)], current: &str) -> Vec<u8> {
        let entries: Vec<SurfaceEntry> = all.iter().map(|(e, _)| e.clone()).collect();
        let mut b = Vec::new();
        write_surfaces(&mut b, "easel save 1\nx=1\n", current, &entries, |i, w| all[i].1.write_state(&mut &mut *w, "")).unwrap();
        b
    }

    /// Three surfaces go into one checkpoint and come back as they were, each
    /// canvas rewritten as it was read, with the header, the entries and the
    /// current surface; a canvas that needs its file's end (version 22's skin
    /// section) reads within its frame.
    #[test]
    fn several_surfaces_survive_a_checkpoint() {
        let all = three_surfaces();
        let b = surfaces_bytes(&all, "study");
        assert!(is_surfaces(&b) && !is_surfaces(&state(&all[0].1)));
        let mut back = Vec::new();
        let (header, current) = read_surfaces(&mut Cursor::new(b.clone()), |e, c| {
            back.push((e, c));
            Ok(())
        }).unwrap();
        assert_eq!((header.as_str(), current.as_str()), ("easel save 1\nx=1\n", "study"));
        assert_eq!(back.len(), 3);
        assert_eq!(&state(&all[0].1)[..8], MAGIC22);
        for ((e0, c0), (e1, c1)) in all.iter().zip(&back) {
            assert_eq!(e0, e1);
            assert!(state(c0) == state(c1), "{}: rewritten as it was read", e0.name);
        }
        assert!(surfaces_bytes(&back, "study") == b, "the checkpoint rewritten as it was read");
    }

    /// A packed canvas's buffers spilled to disk come back bit for bit: the
    /// same buffers, and the same canvas unpacked with them (the easel's
    /// resting surfaces keep them on disk, not in memory).
    #[test]
    fn spilled_aside_comes_back_bit_for_bit() {
        let all = three_surfaces();
        for (e, c) in &all {
            let mut packed = Vec::new();
            let aside = c.pack(&mut packed).unwrap();
            let mut spilled = aside.clone();
            let mut disk = Vec::new();
            spilled.spill(&mut disk).unwrap();
            assert!(spilled.heap_bytes() == 0, "{}: nothing large kept in memory", e.name);
            let mut back = spilled.clone();
            back.unspill(&mut Cursor::new(&disk)).unwrap();
            let mut again = Vec::new();
            back.clone().spill(&mut again).unwrap();
            assert!(again == disk, "{}: the same buffers", e.name);
            assert_eq!((back.receptive.clone(), back.floor.clone(), back.solv.clone(), back.laid.clone(), back.laid_id.clone(), back.micro.clone()), (aside.receptive.clone(), aside.floor.clone(), aside.solv.clone(), aside.laid.clone(), aside.laid_id.clone(), aside.micro.clone()));
            assert_eq!(back.drawing.is_some(), aside.drawing.is_some());
            let a = Canvas::unpack(&mut Cursor::new(&packed), aside).unwrap();
            let b = Canvas::unpack(&mut Cursor::new(&packed), back).unwrap();
            assert!(state(&a) == state(&b), "{}", e.name);
            assert!(a.drawing_guide().data == b.drawing_guide().data);
        }
        assert!(all[1].1.has_drawing(), "the study has a drawing to spill");
    }

    /// A resting canvas's drawing spilled to disk keeps charcoal's carbon
    /// shares and wet pastel's water, index and gum and the sheet's water,
    /// bit for bit, so a surface put down wet with charcoal on it wakes as
    /// it was.
    #[test]
    fn a_spilled_drawing_keeps_its_carbon_and_water() {
        use crate::pastel::{Pose, Stick, StrokePoint};
        let mut c = crate::tests::drawing_paper(160).with_drawing_optics(true).with_wet_pastel(true);
        let pose = Pose { force: 2.0, alt: 1.0, az: 0.8, roll: 0.0 };
        let line = |y: f32| [StrokePoint { x: 100.0, y, pose, speed: 60.0 }, StrokePoint { x: 900.0, y, pose, speed: 60.0 }];
        c.stick_stroke(&mut Stick::round([0.1, 0.1, 0.5], 0.8, 12.0), &line(400.0));
        c.stick_stroke(&mut Stick::vine(8.0), &line(420.0));
        let mut wet = Stick::round([0.5, 0.1, 0.1], 0.8, 12.0);
        wet.water = wet.tip_capacity_mm3();
        c.stick_stroke(&mut wet, &line(410.0));
        c.wait(0.1);
        let d = c.drawing.as_deref().unwrap();
        assert!(d.carbon && d.damp && d.sheet.iter().any(|&u| u > 0.0));
        assert!(d.cells.iter().any(|q| q.carbon > 0.0 && q.water > 0.0 && q.gum != 0 && q.index != 0));
        let mut packed = Vec::new();
        let aside = c.pack(&mut packed).unwrap();
        let mut spilled = aside.clone();
        let mut disk = Vec::new();
        spilled.spill(&mut disk).unwrap();
        assert!(spilled.heap_bytes() == 0);
        spilled.unspill(&mut Cursor::new(&disk)).unwrap();
        let (a, b) = (aside.drawing.as_deref().unwrap(), spilled.drawing.as_deref().unwrap());
        assert!(a.carbon == b.carbon && a.damp == b.damp && a.sheet == b.sheet);
        assert!(a.cells.iter().zip(&b.cells).all(|(x, y)| (x.carbon, x.carbon_bound, x.water, x.index, x.gum, x.loose, x.bound) == (y.carbon, y.carbon_bound, y.water, y.index, y.gum, y.loose, y.bound)));
        let (mut x, mut y) = (Canvas::unpack(&mut Cursor::new(&packed), aside).unwrap(), Canvas::unpack(&mut Cursor::new(&packed), spilled).unwrap());
        assert!(state(&x) == state(&y) && x.pixels() == y.pixels());
        x.dry();
        y.dry();
        assert!(state(&x) == state(&y) && x.pixels() == y.pixels(), "and they dry the same");
    }

    /// Another version of the section, a damaged frame, a canvas cut short, a
    /// current surface that isn't there, two surfaces of one name and trailing
    /// data are refused.
    #[test]
    fn a_damaged_surfaces_checkpoint_is_refused() {
        let all = three_surfaces();
        let b = surfaces_bytes(&all, "painting");
        let refused = |b: Vec<u8>, what: &str| {
            let r = read_surfaces(&mut Cursor::new(b), |_, _| Ok(()));
            assert!(r.is_err(), "{what} was read");
            r.unwrap_err().to_string()
        };
        let mut v2 = b.clone();
        set(&mut v2, 8, 2);
        assert!(refused(v2, "version 2").contains("version 2"));
        refused(b[..b.len() - 9].to_vec(), "a truncated checkpoint");
        let mut longer = b.clone();
        longer.push(0);
        assert!(refused(longer, "trailing data").contains("trailing"));
        // the first canvas's first chunk said to be longer than it is
        let head = 8 + 8 + (8 + "easel save 1\nx=1\n".len()) + (8 + "painting".len()) + 8 + (8 + "painting".len()) + 8;
        let first = u64::from_le_bytes(b[head..head + 8].try_into().unwrap());
        let mut frame = b.clone();
        set(&mut frame, head, first + 1);
        refused(frame, "a damaged frame");
        let entries: Vec<SurfaceEntry> = all.iter().map(|(e, _)| e.clone()).collect();
        let mut w = Vec::new();
        assert!(write_surfaces(&mut w, "", "nowhere", &entries, |i, w| all[i].1.write_state(&mut &mut *w, "")).is_err());
        let mut twice = entries.clone();
        twice[2].name = "study".into();
        assert!(write_surfaces(&mut Vec::new(), "", "study", &twice, |i, w| all[i].1.write_state(&mut &mut *w, "")).is_err());
        // written by hand: the third surface takes the second's name
        let mut dup = b.clone();
        let at = dup.windows(5).rposition(|x| x == b"japan").unwrap();
        dup[at..at + 5].copy_from_slice(b"study");
        refused(dup, "two surfaces of one name");
    }
}
