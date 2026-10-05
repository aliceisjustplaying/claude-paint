use crate::{Canvas, Frame, color::Rgb};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::io::{self, Read, Write};

pub const RENDERER: &str = "legacy-look-1";

const MAGIC: &[u8; 8] = b"CPVIEW02";
const VERSION: u32 = 2;
const RENDERER_VERSION: u32 = 1;
const RENDERER_BYTES: &[u8; 16] = b"legacy-look-1\0\0\0";
const MAX_PIXELS: usize = 16_000_000;
const MAX_DIMENSION: usize = 9600;
const BLOCK_SIZE: usize = 65536;
const METADATA_LEN: usize = 148;
const HEADER_LEN: usize = METADATA_LEN + 32;

pub struct RenderSnapshot {
    frame: Frame,
    keep: (usize, usize, usize, usize),
    engine: u32,
    um_px: f32,
    revision: [u8; 32],
    runtime: [u8; 32],
    integrity: [u8; 32],
    content_id: [u8; 32],
    color: Vec<Rgb>,
    surface: Vec<f32>,
    wet: Vec<f32>,
}

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn word(bytes: &[u8]) -> u32 {
    u32::from_le_bytes(bytes.try_into().unwrap())
}

fn validate_metadata(
    frame: Frame,
    keep: (usize, usize, usize, usize),
    engine: u32,
    um_px: f32,
) -> io::Result<usize> {
    let n = frame
        .w
        .checked_mul(frame.h)
        .filter(|&n| n > 0 && n <= MAX_PIXELS)
        .ok_or_else(|| invalid("snapshot exceeds the held pixel budget"))?;
    if frame.full_w == 0
        || frame.full_h == 0
        || frame.full_w > MAX_DIMENSION
        || frame.full_h > MAX_DIMENSION
        || frame
            .x0
            .checked_add(frame.w)
            .is_none_or(|v| v > frame.full_w)
        || frame
            .y0
            .checked_add(frame.h)
            .is_none_or(|v| v > frame.full_h)
    {
        return Err(invalid("invalid snapshot frame bounds"));
    }
    if keep.0 >= keep.2 || keep.1 >= keep.3 || keep.2 > frame.w || keep.3 > frame.h {
        return Err(invalid("invalid snapshot keep bounds"));
    }
    if !frame.scale.is_finite()
        || frame.scale <= 0.0
        || !um_px.is_finite()
        || um_px <= 0.0
        || !(1..=5).contains(&engine)
    {
        return Err(invalid("unsupported engine or invalid snapshot scale"));
    }
    Ok(n)
}

fn reserved<T>(n: usize) -> io::Result<Vec<T>> {
    let mut values = Vec::new();
    values.try_reserve_exact(n).map_err(io::Error::other)?;
    Ok(values)
}

impl RenderSnapshot {
    pub fn capture(canvas: &Canvas, revision: [u8; 32], runtime: [u8; 32]) -> io::Result<Self> {
        let frame = canvas.window();
        let um_px = canvas.px_mm() * 1000.0;
        let n = validate_metadata(frame, canvas.keep, canvas.engine(), um_px)?;
        if canvas.px.len() != n
            || canvas.height.len() != n
            || canvas.wet.vol.len() != n
            || canvas.wet.lat.len() != n
            || canvas.wet.hide.len() != n
            || canvas.wet.cover.len() != n
            || (canvas.engine() >= 4 && canvas.gloss.len() != n)
        {
            return Err(invalid("invalid canvas optical buffer lengths"));
        }
        if canvas.wet.vol.iter().any(|v| !v.is_finite() || *v < 0.0) {
            return Err(invalid("invalid wet volume"));
        }
        let mut snapshot = Self {
            frame,
            keep: canvas.keep,
            engine: canvas.engine(),
            um_px,
            revision,
            runtime,
            integrity: [0; 32],
            content_id: [0; 32],
            color: canvas.seen(),
            surface: canvas.wet_surface(),
            wet: canvas.wet.vol.clone(),
        };
        if snapshot
            .color
            .iter()
            .flatten()
            .chain(&snapshot.surface)
            .any(|v| !v.is_finite())
        {
            return Err(invalid("non-finite snapshot optical value"));
        }
        let metadata = snapshot.metadata();
        let mut integrity = Sha256::new();
        integrity.update(metadata);
        snapshot.payload(|block| {
            integrity.update(block);
            Ok(())
        })?;
        snapshot.integrity = integrity.finalize().into();
        let mut content_id = Sha256::new();
        content_id.update(metadata);
        content_id.update(snapshot.integrity);
        snapshot.payload(|block| {
            content_id.update(block);
            Ok(())
        })?;
        snapshot.content_id = content_id.finalize().into();
        Ok(snapshot)
    }

    fn metadata(&self) -> [u8; METADATA_LEN] {
        let mut bytes = [0; METADATA_LEN];
        bytes[..8].copy_from_slice(MAGIC);
        bytes[8..12].copy_from_slice(&VERSION.to_le_bytes());
        bytes[12..16].copy_from_slice(&RENDERER_VERSION.to_le_bytes());
        bytes[16..32].copy_from_slice(RENDERER_BYTES);
        for (slot, value) in bytes[32..84].chunks_exact_mut(4).zip([
            self.frame.full_w as u32,
            self.frame.full_h as u32,
            self.frame.x0 as u32,
            self.frame.y0 as u32,
            self.frame.w as u32,
            self.frame.h as u32,
            self.frame.scale.to_bits(),
            self.keep.0 as u32,
            self.keep.1 as u32,
            self.keep.2 as u32,
            self.keep.3 as u32,
            self.engine,
            self.um_px.to_bits(),
        ]) {
            slot.copy_from_slice(&value.to_le_bytes());
        }
        bytes[84..116].copy_from_slice(&self.revision);
        bytes[116..148].copy_from_slice(&self.runtime);
        bytes
    }

    fn payload(&self, mut consume: impl FnMut(&[u8]) -> io::Result<()>) -> io::Result<()> {
        let mut block = [0; BLOCK_SIZE];
        let mut used = 0;
        for value in self
            .color
            .iter()
            .flatten()
            .chain(&self.surface)
            .chain(&self.wet)
        {
            block[used..used + 4].copy_from_slice(&value.to_le_bytes());
            used += 4;
            if used == BLOCK_SIZE {
                consume(&block)?;
                used = 0;
            }
        }
        if used != 0 {
            consume(&block[..used])?;
        }
        Ok(())
    }

    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(&self.metadata())?;
        writer.write_all(&self.integrity)?;
        self.payload(|block| writer.write_all(block))
    }

    pub fn read(reader: &mut impl Read) -> io::Result<Self> {
        let mut header = [0; HEADER_LEN];
        reader.read_exact(&mut header)?;
        if &header[..8] != MAGIC || word(&header[8..12]) != VERSION {
            return Err(invalid("unsupported render snapshot version"));
        }
        if word(&header[12..16]) != RENDERER_VERSION || &header[16..32] != RENDERER_BYTES {
            return Err(invalid("unsupported render snapshot renderer"));
        }
        let frame = Frame {
            full_w: word(&header[32..36]) as usize,
            full_h: word(&header[36..40]) as usize,
            x0: word(&header[40..44]) as usize,
            y0: word(&header[44..48]) as usize,
            w: word(&header[48..52]) as usize,
            h: word(&header[52..56]) as usize,
            scale: f32::from_bits(word(&header[56..60])),
        };
        let keep = (
            word(&header[60..64]) as usize,
            word(&header[64..68]) as usize,
            word(&header[68..72]) as usize,
            word(&header[72..76]) as usize,
        );
        let engine = word(&header[76..80]);
        let um_px = f32::from_bits(word(&header[80..84]));
        let n = validate_metadata(frame, keep, engine, um_px)?;
        let mut color = reserved(n)?;
        let mut surface = reserved(n)?;
        let mut wet = reserved(n)?;
        let mut integrity = Sha256::new();
        integrity.update(&header[..METADATA_LEN]);
        let mut content_id = Sha256::new();
        content_id.update(header);
        let mut block = [0; BLOCK_SIZE];
        let mut position = 0;
        let mut rgb = [0.0; 3];
        while position < n * 5 {
            let len = ((n * 5 - position) * 4).min(BLOCK_SIZE);
            reader.read_exact(&mut block[..len])?;
            integrity.update(&block[..len]);
            content_id.update(&block[..len]);
            for bytes in block[..len].chunks_exact(4) {
                let value = f32::from_bits(word(bytes));
                if !value.is_finite() {
                    return Err(invalid("non-finite snapshot optical value"));
                }
                if position < n * 3 {
                    rgb[position % 3] = value;
                    if position % 3 == 2 {
                        color.push(rgb);
                    }
                } else if position < n * 4 {
                    surface.push(value);
                } else {
                    if value < 0.0 {
                        return Err(invalid("negative snapshot wet volume"));
                    }
                    wet.push(value);
                }
                position += 1;
            }
        }
        let expected: [u8; 32] = header[METADATA_LEN..].try_into().unwrap();
        let actual: [u8; 32] = integrity.finalize().into();
        if actual != expected {
            return Err(invalid("render snapshot integrity mismatch"));
        }
        let mut extra = [0];
        loop {
            match reader.read(&mut extra) {
                Ok(0) => break,
                Ok(_) => return Err(invalid("trailing render snapshot data")),
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(error),
            }
        }
        Ok(Self {
            frame,
            keep,
            engine,
            um_px,
            revision: header[84..116].try_into().unwrap(),
            runtime: header[116..148].try_into().unwrap(),
            integrity: actual,
            content_id: content_id.finalize().into(),
            color,
            surface,
            wet,
        })
    }

    pub fn revision(&self) -> [u8; 32] {
        self.revision
    }

    pub fn runtime(&self) -> [u8; 32] {
        self.runtime
    }

    pub fn content_id(&self) -> [u8; 32] {
        self.content_id
    }

    pub fn frame(&self) -> Frame {
        self.frame
    }

    pub fn keep(&self) -> (usize, usize, usize, usize) {
        self.keep
    }

    pub fn color(&self) -> &[Rgb] {
        &self.color
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.frame.w, self.frame.h)
    }

    pub fn lit(&self, azimuth: f32, elevation: f32, strength: f32) -> Vec<Rgb> {
        let ambient = crate::lerp(0.3, 0.55, crate::smoothstep(10.0, 55.0, elevation));
        let base = &self.color;
        let (w, h) = (self.frame.w, self.frame.h);
        let um_px = self.um_px;
        let surf = &self.surface;
        // 0 to 90 degrees, both ends exact (as `Canvas::seen_lit`)
        let elevation = elevation.clamp(0.0, 90.0);
        let (overhead, grazing) = (elevation >= 90.0, elevation <= 1e-3);
        let (az, el) = (azimuth.to_radians(), elevation.to_radians());
        let (lx, ly, lz) = if overhead {
            (0.0, 0.0, 1.0)
        } else if grazing {
            (az.cos(), -az.sin(), 0.0)
        } else {
            (el.cos() * az.cos(), -el.cos() * az.sin(), el.sin())
        };
        let k = 0.5 / um_px;
        let rise = if overhead || grazing { 0.0 } else { el.tan() * um_px };
        let (hi, lo) = surf
            .par_iter()
            .fold(|| (f32::MIN, f32::MAX), |(a, b), &v| (a.max(v), b.min(v)))
            .reduce(
                || (f32::MIN, f32::MAX),
                |(a, b), (c, d)| (a.max(c), b.min(d)),
            );
        let m = (lx * lx + ly * ly).sqrt();
        let march = m > 1e-6 && (rise > 0.0 || grazing);
        let steps = if !march {
            0
        } else if grazing {
            w.saturating_add(h)
        } else {
            (((hi - lo) / rise).ceil() as usize)
                .max(1)
                .min(w.saturating_add(h))
        };
        let (sx, sy) = if march { (lx / m, ly / m) } else { (0.0, 0.0) };
        let at = |x: isize, y: isize| {
            surf[(y.clamp(0, h as isize - 1) as usize) * w + x.clamp(0, w as isize - 1) as usize]
        };
        let shade_surf = crate::surface::box_blur(surf, w, h, 1);
        let shade_hi = shade_surf.par_iter().copied().reduce(|| f32::MIN, f32::max);
        let sat = |x: isize, y: isize| {
            shade_surf
                [(y.clamp(0, h as isize - 1) as usize) * w + x.clamp(0, w as isize - 1) as usize]
        };
        let hv = {
            let v = [lx, ly, lz + 1.0];
            let hm = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            [v[0] / hm, v[1] / hm, v[2] / hm]
        };
        (0..w * h)
            .into_par_iter()
            .map(|i| {
                let (x, y) = ((i % w) as isize, (i / w) as isize);
                let dx = (at(x + 1, y) - at(x - 1, y)) * k;
                let dy = (at(x, y + 1) - at(x, y - 1)) * k;
                let g = (dx * dx + dy * dy).sqrt();
                let soft = 1.0 / (1.0 + g / 1.2);
                let (dx, dy) = (dx * soft, dy * soft);
                let m = (dx * dx + dy * dy + 1.0).sqrt();
                let n = [-dx / m, -dy / m, 1.0 / m];
                let ndl = (n[0] * lx + n[1] * ly + n[2] * lz).max(0.0);
                let h0 = shade_surf[i];
                let mut lit = 1.0f32;
                for s in 1..=steps {
                    let ray_height = h0 + rise * s as f32;
                    if ray_height >= shade_hi || lit == 0.0 {
                        break;
                    }
                    let (px, py) = (x as f32 + sx * s as f32, y as f32 + sy * s as f32);
                    if px < 0.0 || py < 0.0 || px >= w as f32 || py >= h as f32 {
                        break;
                    }
                    let over = sat(px.round() as isize, py.round() as isize) - ray_height;
                    if over > 0.0 {
                        lit = if grazing {
                            0.0
                        } else {
                            lit.min(1.0 - (over / (0.5 * rise)).min(1.0))
                        };
                    }
                }
                let direct = if grazing { ndl * lit } else { ndl * lit / lz };
                let diffuse = (ambient + (1.0 - ambient) * direct).min(1.6);
                let shade = (1.0 + strength * (diffuse - 1.0)).max(0.0);
                let wet = (self.wet[i] * 4.0).min(1.0);
                let ndh = (n[0] * hv[0] + n[1] * hv[1] + n[2] * hv[2]).max(0.0);
                let spec = (0.02 + 0.10 * wet) * ndh.powf(40.0) * lit;
                let p = base[i];
                [
                    (p[0] * shade + spec).max(0.0),
                    (p[1] * shade + spec).max(0.0),
                    (p[2] * shade + spec).max(0.0),
                ]
            })
            .collect()
    }
}

#[cfg(test)]
#[path = "render_snapshot_tests.rs"]
mod tests;
