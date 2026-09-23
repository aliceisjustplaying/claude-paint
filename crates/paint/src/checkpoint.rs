//! Saving and restoring the complete state of a canvas, so a painting can
//! resume after a stage instead of repainting it (see `paintings::run`).
//!
//! The state is everything later painting depends on: the frame (and crop
//! window), the dry picture (color, surface relief, film), the support
//! (linen, physical size), the wet layer (volume, pigment mix, scattering,
//! stiffness and drying rate, dirty box), the stroke counter and the
//! per-pixel ids of the last stroke to lay or touch paint there (`wait`
//! reads them to tell which films were worked), the clock and the drying
//! state (see `drying`). Not stored, because nothing later reads them: the
//! film floors of past strokes and the brushes' contact surface (derived
//! from the relief and rebuilt on demand). Restoring is exact: a resumed run
//! paints bit-for-bit what an uninterrupted one does.
//!
//! Version 2 (`PAINTCK2`) added the drying rate, stroke ids, clock and
//! drying state; version 3 (`PAINTCK3`) adds the canvas's ground thickness
//! (for craquelure fitted to the ground); version 4 (`PAINTCK4`) adds each
//! wet pixel's paint coverage (pointed-tip marks). Older files are refused
//! (re-run to checkpoint again).
//!
//! Format: little-endian binary, `MAGIC`, then a free-form UTF-8 header
//! (length-prefixed; the caller's key=value lines), then the canvas. If you
//! add state to `Canvas` or `Wet`, add it here and bump `MAGIC`.

use crate::canvas::{Canvas, Frame};
use crate::surface::Linen;
use crate::wet::LAT;
use std::io::{self, Read, Write};

const MAGIC: &[u8; 8] = b"PAINTCK4";

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

/// Read just the header of a checkpoint (to validate it before loading).
pub fn read_header(r: &mut impl Read) -> io::Result<String> {
    let mut m = [0u8; 8];
    r.read_exact(&mut m)?;
    if &m != MAGIC {
        return Err(bad("not a canvas checkpoint (or an older format)"));
    }
    let n = get_u64(r)? as usize;
    if n > 1 << 20 {
        return Err(bad("checkpoint header too long"));
    }
    let mut h = vec![0u8; n];
    r.read_exact(&mut h)?;
    String::from_utf8(h).map_err(|_| bad("checkpoint header is not UTF-8"))
}

impl Canvas {
    /// Write the complete canvas state (dries nothing: wet paint stays wet)
    /// after `header`.
    pub fn write_state(&self, w: &mut impl Write, header: &str) -> io::Result<()> {
        w.write_all(MAGIC)?;
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
        put_all(w, wt.hide.iter().flat_map(|h| *h))?;
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
            put_all(w, ck.px.iter().flat_map(|p| [p.cure, p.lev, p.seen, p.sub, p.srate]))?;
        }
        put_f32(w, self.ground_um)?;
        put_all(w, wt.cover.iter().copied())?;
        Ok(())
    }

    /// Read a canvas written by `write_state`; returns it and the header.
    pub fn read_state(r: &mut impl Read) -> io::Result<(Canvas, String)> {
        let header = read_header(r)?;
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
        let hide = get_all(r, n * 3)?;
        wet.hide = hide.as_chunks::<3>().0.to_vec();
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
            _ => get_all(r, n * 5)?.as_chunks::<5>().0.iter().map(|q| crate::drying::Px { cure: q[0], lev: q[1], seen: q[2], sub: q[3], srate: q[4] }).collect(),
        };
        wet.clock = crate::drying::Clock { now, px, mark, tacky };
        let ground_um = get_f32(r)?;
        c.ground_um = if ground_um.is_finite() { ground_um.max(0.0) } else { 0.0 };
        wet.cover = get_all(r, n)?;
        c.wet = wet;
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

    #[test]
    fn round_trip() {
        let mut c = load(original()).unwrap();
        c.wet.touch(0, 0, 2, 1);
        let mut b = Vec::new();
        c.write_state(&mut b, "x=1\n").unwrap();
        let (d, h) = Canvas::read_state(&mut Cursor::new(b)).unwrap();
        assert_eq!((h.as_str(), d.keep, d.wet.dirty), ("x=1\n", (0, 0, 2, 2), Some((0, 0, 2, 1))));
    }

    /// Corrupt geometry is an error when loading, not a panic later (cases
    /// from the review: keep past the buffer, an overflowing frame).
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
