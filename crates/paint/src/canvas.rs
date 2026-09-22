//! The painting surface.

use crate::color::{self, Mix, Rgb};
use crate::mask::Mask;
use crate::pigment::Pigment;
use crate::rng::hash2;
use rayon::prelude::*;

/// Pixel dimensions plus the units → pixels scale.
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub w: usize,
    pub h: usize,
    /// Pixels per unit.
    pub scale: f32,
}

impl Frame {
    pub const WIDTH_UNITS: f32 = 1000.0;

    pub fn width(&self) -> f32 {
        Self::WIDTH_UNITS
    }
    pub fn height(&self) -> f32 {
        self.h as f32 / self.scale
    }
}

pub struct Canvas {
    pub f: Frame,
    /// Linear RGB reflectance, row major.
    pub px: Vec<Rgb>,
}

impl Canvas {
    /// `aspect` = width / height.
    pub fn new(width_px: usize, aspect: f32, ground: Rgb) -> Self {
        let h = (width_px as f32 / aspect).round() as usize;
        let f = Frame { w: width_px, h, scale: width_px as f32 / Frame::WIDTH_UNITS };
        Canvas { f, px: vec![ground; width_px * h] }
    }

    /// Canvas width in units (always 1000).
    pub fn width(&self) -> f32 {
        self.f.width()
    }
    /// Canvas height in units.
    pub fn height(&self) -> f32 {
        self.f.height()
    }

    /// Per-pixel transform; `g(x, y, current) -> new`, x/y in units.
    pub fn apply(&mut self, g: impl Fn(f32, f32, Rgb) -> Rgb + Sync) {
        let inv = 1.0 / self.f.scale;
        self.px.par_chunks_mut(self.f.w).enumerate().for_each(|(y, row)| {
            let yu = (y as f32 + 0.5) * inv;
            for (x, p) in row.iter_mut().enumerate() {
                *p = g((x as f32 + 0.5) * inv, yu, *p);
            }
        });
    }

    /// Like `apply`, but only where `m` > 0; `g` receives the coverage.
    pub fn apply_masked(&mut self, m: &Mask, g: impl Fn(f32, f32, Rgb, f32) -> Rgb + Sync) {
        let inv = 1.0 / self.f.scale;
        let w = self.f.w;
        self.px.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            let yu = (y as f32 + 0.5) * inv;
            let mrow = &m.data[y * w..(y + 1) * w];
            for (x, p) in row.iter_mut().enumerate() {
                let c = mrow[x];
                if c > 0.0 {
                    *p = g((x as f32 + 0.5) * inv, yu, *p, c);
                }
            }
        });
    }

    /// Opaque paint: blend a (position-dependent) color into the canvas with
    /// coverage `mask * opacity`.
    pub fn paint(
        &mut self,
        mask: Option<&Mask>,
        opacity: f32,
        mode: Mix,
        color: impl Fn(f32, f32) -> Rgb + Sync,
    ) {
        match mask {
            Some(m) => self.apply_masked(m, |x, y, p, c| color::mix(p, color(x, y), c * opacity, mode)),
            None => self.apply(|x, y, p| color::mix(p, color(x, y), opacity, mode)),
        }
    }

    /// Flat color fill through a mask.
    pub fn fill(&mut self, mask: &Mask, c: Rgb, opacity: f32, mode: Mix) {
        self.paint(Some(mask), opacity, mode, |_, _| c);
    }

    /// Kubelka–Munk glaze: a layer of `pigment` whose thickness is
    /// `thickness(x, y)` (times mask coverage, if given).
    pub fn glaze(
        &mut self,
        pigment: &Pigment,
        mask: Option<&Mask>,
        thickness: impl Fn(f32, f32) -> f32 + Sync,
    ) {
        match mask {
            Some(m) => self.apply_masked(m, |x, y, p, c| pigment.over(p, thickness(x, y) * c)),
            None => self.apply(|x, y, p| pigment.over(p, thickness(x, y))),
        }
    }

    /// Atmospheric veil (fog, haze, light): optical blend toward `c` with
    /// strength 1 - exp(-density).
    pub fn veil(
        &mut self,
        mask: Option<&Mask>,
        c: Rgb,
        density: impl Fn(f32, f32) -> f32 + Sync,
    ) {
        let g = |x: f32, y: f32, p: Rgb, cov: f32| {
            let t = 1.0 - (-density(x, y).max(0.0) * cov).exp();
            color::mix(p, c, t, Mix::Linear)
        };
        match mask {
            Some(m) => self.apply_masked(m, g),
            None => self.apply(|x, y, p| g(x, y, p, 1.0)),
        }
    }

    /// Woven canvas texture. `thread` = thread spacing in units, `strength`
    /// ~0.02–0.08. Paint sits in the weave, so this modulates brightness.
    pub fn canvas_texture(&mut self, thread: f32, strength: f32, seed: u64) {
        // A weave finer than ~3px can't be represented; it would alias into a
        // grid. Fade it out at low resolution.
        let strength = strength * crate::smoothstep(1.5, 3.5, thread * self.f.scale);
        if strength <= 0.0 {
            return;
        }
        self.apply(|x, y, p| {
            let u = x / thread;
            let v = y / thread;
            let (iu, iv) = (u.floor() as i64, v.floor() as i64);
            let (fu, fv) = (u - iu as f32, v - iv as f32);
            // thread thickness irregularity per thread
            let tw = 0.75 + 0.5 * hash2(iu, 0, seed);
            let th = 0.75 + 0.5 * hash2(0, iv, seed + 1);
            let over = (iu + iv).rem_euclid(2) == 0;
            let warp = (std::f32::consts::PI * fu).sin().powf(0.7) * tw;
            let weft = (std::f32::consts::PI * fv).sin().powf(0.7) * th;
            let hgt = if over { 0.65 * warp + 0.35 * weft } else { 0.35 * warp + 0.65 * weft };
            let k = 1.0 + strength * (hgt - 0.5) * 2.0;
            [p[0] * k, p[1] * k, p[2] * k]
        });
    }

    /// Fine, low-contrast mottling (uneven paint film / aged surface).
    pub fn mottle(&mut self, period: f32, strength: f32, seed: u32) {
        let n = crate::noise::Fbm::new(seed, 5, period);
        self.apply(|x, y, p| {
            let k = 1.0 + strength * n.get(x, y);
            [p[0] * k, p[1] * k, p[2] * k]
        });
    }

    /// Save as an 8-bit sRGB PNG with triangular dither (prevents banding in
    /// the long, subtle gradients Friedrich loves).
    pub fn save(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        let (w, h) = (self.f.w, self.f.h);
        let mut buf = vec![0u8; w * h * 3];
        buf.par_chunks_mut(w * 3).enumerate().for_each(|(y, row)| {
            for x in 0..w {
                let p = self.px[y * w + x];
                for c in 0..3 {
                    let d = hash2(x as i64, y as i64, c as u64 * 7 + 1)
                        - hash2(x as i64, y as i64, c as u64 * 7 + 2);
                    let v = color::linear_to_srgb(p[c]) * 255.0 + d;
                    row[x * 3 + c] = v.round().clamp(0.0, 255.0) as u8;
                }
            }
        });
        if let Some(dir) = path.as_ref().parent() {
            std::fs::create_dir_all(dir)?;
        }
        image::save_buffer(path.as_ref(), &buf, w as u32, h as u32, image::ColorType::Rgb8)
            .map_err(std::io::Error::other)
    }
}
