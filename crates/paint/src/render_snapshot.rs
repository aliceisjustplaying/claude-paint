use crate::{Canvas, color::Rgb};
use rayon::prelude::*;
use std::io::{self, Read, Write};

const MAGIC: &[u8; 8] = b"CPVIEW01";
const MAX_PIXELS: usize = 16_000_000;

pub struct RenderSnapshot {
    width: usize,
    height: usize,
    engine: u32,
    um_px: f32,
    color: Vec<Rgb>,
    surface: Vec<f32>,
    wet: Vec<f32>,
}

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn word(r: &mut impl Read) -> io::Result<u32> {
    let mut bytes = [0; 4];
    r.read_exact(&mut bytes)?;
    Ok(u32::from_le_bytes(bytes))
}

impl RenderSnapshot {
    pub fn capture(canvas: &Canvas) -> io::Result<Self> {
        let frame = canvas.window();
        if !frame.is_whole() || canvas.keep != (0, 0, frame.w, frame.h) {
            return Err(invalid("prototype supports whole-canvas snapshots only"));
        }
        if frame
            .w
            .checked_mul(frame.h)
            .is_none_or(|n| n == 0 || n > MAX_PIXELS)
        {
            return Err(invalid("snapshot exceeds the prototype pixel budget"));
        }
        Ok(Self {
            width: frame.w,
            height: frame.h,
            engine: canvas.engine(),
            um_px: canvas.px_mm() * 1000.0,
            color: canvas.seen(),
            surface: canvas.wet_surface(),
            wet: canvas.wet.vol.clone(),
        })
    }

    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(MAGIC)?;
        for value in [
            self.width as u32,
            self.height as u32,
            self.engine,
            self.um_px.to_bits(),
        ] {
            writer.write_all(&value.to_le_bytes())?;
        }
        let mut block = Vec::with_capacity(65536);
        for value in self
            .color
            .iter()
            .flatten()
            .chain(&self.surface)
            .chain(&self.wet)
        {
            block.extend_from_slice(&value.to_le_bytes());
            if block.len() == 65536 {
                writer.write_all(&block)?;
                block.clear();
            }
        }
        writer.write_all(&block)
    }

    pub fn read(reader: &mut impl Read) -> io::Result<Self> {
        let mut magic = [0; 8];
        reader.read_exact(&mut magic)?;
        if &magic != MAGIC {
            return Err(invalid("unsupported render snapshot"));
        }
        let width = word(reader)? as usize;
        let height = word(reader)? as usize;
        let engine = word(reader)?;
        let um_px = f32::from_bits(word(reader)?);
        let n = width
            .checked_mul(height)
            .filter(|&n| n > 0 && n <= MAX_PIXELS)
            .ok_or_else(|| invalid("snapshot exceeds the prototype pixel budget"))?;
        if !um_px.is_finite() || um_px <= 0.0 || !(1..=crate::ENGINE).contains(&engine) {
            return Err(invalid("unsupported engine or invalid pixel scale"));
        }
        let mut scalar = || -> io::Result<f32> {
            let value = f32::from_bits(word(reader)?);
            if !value.is_finite() {
                return Err(invalid("non-finite snapshot value"));
            }
            Ok(value)
        };
        let mut color = Vec::with_capacity(n);
        for _ in 0..n {
            color.push([scalar()?, scalar()?, scalar()?]);
        }
        let mut surface = Vec::with_capacity(n);
        for _ in 0..n {
            surface.push(scalar()?);
        }
        let mut wet = Vec::with_capacity(n);
        for _ in 0..n {
            let value = scalar()?;
            if value < 0.0 {
                return Err(invalid("negative wet volume"));
            }
            wet.push(value);
        }
        let mut extra = [0];
        if reader.read(&mut extra)? != 0 {
            return Err(invalid("trailing snapshot data"));
        }
        Ok(Self {
            width,
            height,
            engine,
            um_px,
            color,
            surface,
            wet,
        })
    }

    pub fn color(&self) -> &[Rgb] {
        &self.color
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn lit(&self, azimuth: f32, elevation: f32, strength: f32) -> Vec<Rgb> {
        let ambient = crate::lerp(0.3, 0.55, crate::smoothstep(10.0, 55.0, elevation));
        let base = &self.color;
        let (w, h) = (self.width, self.height);
        let um_px = self.um_px;
        let surf = &self.surface;
        let (az, el) = (
            azimuth.to_radians(),
            elevation.clamp(3.0, 89.0).to_radians(),
        );
        let (lx, ly, lz) = (el.cos() * az.cos(), -el.cos() * az.sin(), el.sin());
        let k = 0.5 / um_px;
        let rise = el.tan() * um_px;
        let (hi, lo) = surf
            .par_iter()
            .fold(|| (f32::MIN, f32::MAX), |(a, b), &v| (a.max(v), b.min(v)))
            .reduce(
                || (f32::MIN, f32::MAX),
                |(a, b), (c, d)| (a.max(c), b.min(d)),
            );
        let steps = (((hi - lo) / rise).ceil() as usize)
            .max(1)
            .min(w.saturating_add(h));
        let (sx, sy) = {
            let m = (lx * lx + ly * ly).sqrt().max(1e-6);
            (lx / m, ly / m)
        };
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
                    if ray_height > shade_hi || lit == 0.0 {
                        break;
                    }
                    let (px, py) = (x as f32 + sx * s as f32, y as f32 + sy * s as f32);
                    if px < 0.0 || py < 0.0 || px >= w as f32 || py >= h as f32 {
                        break;
                    }
                    let over = sat(px.round() as isize, py.round() as isize) - ray_height;
                    if over > 0.0 {
                        lit = lit.min(1.0 - (over / (0.5 * rise)).min(1.0));
                    }
                }
                let diffuse = (ambient + (1.0 - ambient) * ndl * lit / lz).min(1.6);
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
mod tests {
    use super::*;

    #[test]
    fn optical_roundtrip_is_bit_exact() {
        for engine in 1..=crate::ENGINE {
            let mut canvas = Canvas::new(32, 1.25, [0.3, 0.5, 0.2]).with_engine(engine);
            canvas.gloss.resize(canvas.px.len(), 0.0);
            if engine >= 3 {
                canvas.wet.solv.resize(canvas.px.len(), 0.0);
            }
            for i in 0..canvas.px.len() {
                canvas.height[i] = (i % 13) as f32 * 4.3;
                canvas.wet.vol[i] = (i % 7) as f32 * 0.07;
                canvas.gloss[i] = (i % 5) as f32 * 0.2;
                if engine >= 3 {
                    canvas.wet.solv[i] = (i % 11) as f32 * 0.03;
                }
            }
            let snapshot = RenderSnapshot::capture(&canvas).unwrap();
            let mut bytes = Vec::new();
            snapshot.write(&mut bytes).unwrap();
            assert_eq!(bytes.len(), 24 + canvas.px.len() * 20);
            let restored = RenderSnapshot::read(&mut bytes.as_slice()).unwrap();
            for (az, el) in [(115.0, 55.0), (135.0, 25.0), (15.0, 8.0)] {
                let expected = canvas.seen_lit(az, el, 1.0);
                let actual = restored.lit(az, el, 1.0);
                assert_eq!(expected.len(), actual.len());
                assert!(
                    expected
                        .iter()
                        .flatten()
                        .zip(actual.iter().flatten())
                        .all(|(a, b)| a.to_bits() == b.to_bits())
                );
            }
            assert_eq!(restored.color(), canvas.seen());
            assert!(RenderSnapshot::read(&mut &bytes[..bytes.len() - 1]).is_err());
            bytes.extend_from_slice(&[0]);
            assert!(RenderSnapshot::read(&mut bytes.as_slice()).is_err());
            canvas.keep.2 -= 1;
            assert!(RenderSnapshot::capture(&canvas).is_err());
        }
    }

    #[test]
    fn refuses_oversized_and_unknown_snapshots() {
        let mut bytes = MAGIC.to_vec();
        for value in [u32::MAX, u32::MAX, 5, 1.0f32.to_bits()] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        assert!(RenderSnapshot::read(&mut bytes.as_slice()).is_err());
        bytes[0] = b'X';
        assert!(RenderSnapshot::read(&mut bytes.as_slice()).is_err());
    }
}
