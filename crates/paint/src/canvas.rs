//! The painting surface: color, paint relief and the woven support.

use crate::color::{self, Rgb};
use crate::mask::Mask;
use crate::pigment::Pigment;
use crate::rng::hash2;
use crate::surface::{COAT_UM, Linen, vnoise};

/// Fraction of a glaze layer that stays as film (the rest of the "thickness"
/// is how deep the color reads; a glaze is mostly medium, and thin).
const GLAZE_FILM: f32 = 0.3;
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
    /// Pixel index for a point in units (clamped).
    #[inline]
    pub fn index(&self, x: f32, y: f32) -> usize {
        let px = ((x * self.scale) as isize).clamp(0, self.w as isize - 1) as usize;
        let py = ((y * self.scale) as isize).clamp(0, self.h as isize - 1) as usize;
        py * self.w + px
    }
}

pub struct Canvas {
    pub(crate) f: Frame,
    /// Linear RGB reflectance, row major.
    pub(crate) px: Vec<Rgb>,
    /// Physical surface height, µm: woven linen, ground layers, paint films.
    pub(crate) height: Vec<f32>,
    /// Accumulated paint film in coats (bookkeeping).
    pub(crate) film: Vec<f32>,
    pub(crate) linen: Option<Linen>,
    /// Physical size: millimeters per unit (the canvas is 1000 units wide).
    pub(crate) mm_per_unit: f32,
    /// Wet paint on top of the dry picture.
    pub(crate) wet: crate::wet::Wet,
    /// Bumped whenever the height changes; `base` caches the surface relief
    /// bristles feel.
    pub(crate) surf_gen: u64,
    pub(crate) base: Option<(u64, Vec<f32>)>,
}

impl Canvas {
    /// `aspect` = width / height.
    pub fn new(width_px: usize, aspect: f32, ground: Rgb) -> Self {
        let h = (width_px as f32 / aspect).round() as usize;
        let f = Frame { w: width_px, h, scale: width_px as f32 / Frame::WIDTH_UNITS };
        let n = width_px * h;
        Canvas {
            f,
            px: vec![ground; n],
            height: vec![0.0; n],
            film: vec![0.0; n],
            linen: None,
            mm_per_unit: 0.7,
            wet: crate::wet::Wet::new(n),
            surf_gen: 0,
            base: None,
        }
    }

    /// Physical width of the painting in mm (default 700).
    pub fn with_size_mm(mut self, width_mm: f32) -> Self {
        self.mm_per_unit = width_mm / Frame::WIDTH_UNITS;
        self.build_support();
        self
    }

    /// Use a woven linen support.
    pub fn with_linen(mut self, l: Linen) -> Self {
        self.linen = Some(l);
        self.build_support();
        self
    }

    /// A ground layer over the whole canvas: `um` µm of paint of `color` and
    /// `hiding`, spread with a knife or broad brush, leveled and set.
    /// `stiff` 0..1 is its body (fluid chalk-glue ≈ 0.2, oil lead white ≈ 0.6);
    /// `texture` 0..1 roughens it before it levels (a roller or scraped knife).
    pub fn prime(&mut self, color: Rgb, hiding: f32, um: f32, stiff: f32, texture: f32, seed: u64) {
        self.dry();
        let (w, h) = (self.f.w, self.f.h);
        let px = self.px_mm();
        let add: Vec<f32> = (0..w * h)
            .into_par_iter()
            .map(|i| {
                let (x, y) = ((i % w) as f32 * px, (i / w) as f32 * px);
                let n = 0.65 * vnoise(x / 0.3, y / 0.3, seed) + 0.35 * vnoise(x / 0.9, y / 0.9, seed + 1) - 0.5;
                um * (1.0 + texture * 1.4 * n).max(0.0)
            })
            .collect();
        let sv = vec![stiff; w * h];
        let t = self.settle((0, 0, w, h), &add, &sv);
        let pig = Pigment::masstone_hiding(color, hiding);
        self.px.par_iter_mut().zip(&t).for_each(|(p, &ti)| *p = pig.over(*p, ti / COAT_UM));
        self.film.par_iter_mut().zip(&t).for_each(|(f, &ti)| *f += ti / COAT_UM);
    }

    /// Pixel dimensions and scale (for building masks).
    pub fn frame(&self) -> Frame {
        self.f
    }

    /// Linear RGB pixels (read-only).
    pub fn pixels(&self) -> &[Rgb] {
        &self.px
    }

    /// Surface height in µm (read-only). Edit it through canvas operations
    /// so derived data (the brushes' contact surface) stays in sync.
    pub fn surface_um(&self) -> &[f32] {
        &self.height
    }

    /// Panics unless `m` was made for this canvas's frame.
    #[track_caller]
    pub(crate) fn check_mask(&self, m: &Mask) {
        assert!(
            m.f.w == self.f.w && m.f.h == self.f.h && m.data.len() == self.f.w * self.f.h,
            "mask {}x{} does not match canvas {}x{}",
            m.f.w,
            m.f.h,
            self.f.w,
            self.f.h
        );
    }

    /// Canvas width in units (always 1000).
    pub fn width(&self) -> f32 {
        self.f.width()
    }
    /// Canvas height in units.
    pub fn height(&self) -> f32 {
        self.f.height()
    }

    /// Current color at a point in units.
    #[inline]
    pub fn sample(&self, x: f32, y: f32) -> Rgb {
        self.px[self.f.index(x, y)]
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
        self.check_mask(m);
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

    /// Kubelka–Munk glaze: a layer of `pigment` whose thickness is
    /// `thickness(x, y)` (times mask coverage, if given).
    pub fn glaze(
        &mut self,
        pigment: &Pigment,
        mask: Option<&Mask>,
        thickness: impl Fn(f32, f32) -> f32 + Sync,
    ) {
        if let Some(m) = mask {
            self.check_mask(m);
        }
        self.dry();
        // the glaze is mostly medium: a thin fluid film that levels and pools
        // in the hollows of the surface, so it is deeper there
        let (w, h) = (self.f.w, self.f.h);
        let inv = 1.0 / self.f.scale;
        let th: Vec<f32> = (0..w * h)
            .into_par_iter()
            .map(|i| {
                let c = mask.map_or(1.0, |m| m.data[i]);
                if c <= 0.0 { 0.0 } else { thickness(((i % w) as f32 + 0.5) * inv, ((i / w) as f32 + 0.5) * inv).max(0.0) * c }
            })
            .collect();
        let add: Vec<f32> = th.iter().map(|t| t * COAT_UM * GLAZE_FILM).collect();
        let sv = vec![0.05f32; w * h];
        let t = self.settle((0, 0, w, h), &add, &sv);
        self.px.par_iter_mut().enumerate().for_each(|(i, p)| {
            if add[i] > 0.0 {
                *p = pigment.over(*p, th[i] * t[i] / add[i]);
            }
        });
        self.film.par_iter_mut().zip(&t).for_each(|(f, &ti)| *f += ti / COAT_UM);
    }

    /// Light the surface relief (paint ridges + weave) from the upper left.
    /// `strength` ≈ 0.3–1.0; `gloss` adds a faint varnish sheen on ridges.
    pub fn relief(&mut self, strength: f32, gloss: f32) {
        self.dry();
        let (w, h) = (self.f.w, self.f.h);
        let surf = &self.height;
        // light from the upper left at ~35° elevation
        let l = {
            let v = [-0.58f32, -0.58, 0.57];
            let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            [v[0] / n, v[1] / n, v[2] / n]
        };
        // true slopes: µm of height per µm across (central difference)
        let k = 0.5 / (self.px_mm() * 1000.0);
        self.px.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            for x in 0..w {
                let at = |xx: usize, yy: usize| surf[yy.min(h - 1) * w + xx.min(w - 1)];
                let dx = (at(x + 1, y) - at(x.saturating_sub(1), y)) * k;
                let dy = (at(x, y + 1) - at(x, y.saturating_sub(1))) * k;
                // paint edges round over: soft-limit the slope so a hairline
                // ridge doesn't shade to black on one side and white on the other
                let g = (dx * dx + dy * dy).sqrt();
                let lim = 1.0 / (1.0 + g / 1.2);
                let (dx, dy) = (dx * lim, dy * lim);
                let n = {
                    let v = [-dx, -dy, 1.0];
                    let m = (v[0] * v[0] + v[1] * v[1] + 1.0).sqrt();
                    [v[0] / m, v[1] / m, v[2] / m]
                };
                let ndl = n[0] * l[0] + n[1] * l[1] + n[2] * l[2];
                let shade = 1.0 + strength * (ndl / l[2] - 1.0);
                // Blinn-Phong sheen, view straight on
                let hv = [l[0], l[1], l[2] + 1.0];
                let hm = (hv[0] * hv[0] + hv[1] * hv[1] + hv[2] * hv[2]).sqrt();
                let ndh = ((n[0] * hv[0] + n[1] * hv[1] + n[2] * hv[2]) / hm).max(0.0);
                let flat = (l[2] + 1.0) / hm;
                let spec = (gloss * (ndh.powf(60.0) - flat.powf(60.0)).max(0.0)).min(gloss * 0.2);
                let p = &mut row[x];
                for c in 0..3 {
                    p[c] = (p[c] * shade + spec).max(0.0);
                }
            }
        });
    }

    /// Save as an 8-bit sRGB PNG with triangular dither (prevents banding in
    /// the long, subtle gradients Friedrich loves).
    pub fn save(&mut self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        self.dry();
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


impl Canvas {
}
