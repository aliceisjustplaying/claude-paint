//! The painting surface: color, paint relief and the woven support.

use crate::color::{self, Mix, Rgb};
use crate::mask::Mask;
use crate::pigment::Pigment;
use crate::rng::hash2;
use crate::surface::{COAT_UM, Linen, vnoise};

/// Fraction of a glaze layer that stays as film (the rest of the "thickness"
/// is how deep the color reads; a glaze is mostly medium, and thin).
const GLAZE_FILM: f32 = 0.3;
use crate::smoothstep;
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
    pub f: Frame,
    /// Linear RGB reflectance, row major.
    pub px: Vec<Rgb>,
    /// Physical surface height, µm: woven linen, ground layers, paint films.
    pub height: Vec<f32>,
    /// Accumulated paint film in coats (bookkeeping).
    pub film: Vec<f32>,
    pub linen: Option<Linen>,
    /// Physical size: millimeters per unit (the canvas is 1000 units wide).
    pub mm_per_unit: f32,
    /// Wet paint on top of the dry picture.
    pub wet: crate::wet::Wet,
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

    /// Legacy: linen with a thread spacing of `thread` units (warp), weft a
    /// little coarser. `_amp` is ignored (the crown height is physical now).
    pub fn with_weave(self, thread: f32, _amp: f32, seed: u64) -> Self {
        let per_cm = 10.0 / (thread * self.mm_per_unit);
        self.with_linen(Linen { warp_per_cm: per_cm, weft_per_cm: per_cm * 0.87, ..Linen::fine(seed) })
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
        let pig = Pigment::with_hiding(color, hiding);
        self.px.par_iter_mut().zip(&t).for_each(|(p, &ti)| *p = pig.over(*p, ti / COAT_UM));
        self.film.par_iter_mut().zip(&t).for_each(|(f, &ti)| *f += ti / COAT_UM);
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

    fn add_film(&mut self, mask: Option<&Mask>, amount: f32) {
        self.surf_gen += 1;
        match mask {
            Some(m) => self.film.par_iter_mut().zip(&m.data).for_each(|(f, c)| *f += c * amount),
            None => self.film.par_iter_mut().for_each(|f| *f += amount),
        }
    }

    /// Opaque paint: blend a (position-dependent) color into the canvas with
    /// coverage `mask * opacity`. This is a flat, brushless fill; prefer
    /// `fill_strokes` for anything that should look painted.
    pub fn paint(
        &mut self,
        mask: Option<&Mask>,
        opacity: f32,
        mode: Mix,
        color: impl Fn(f32, f32) -> Rgb + Sync,
    ) {
        self.dry();
        match mask {
            Some(m) => self.apply_masked(m, |x, y, p, c| color::mix(p, color(x, y), c * opacity, mode)),
            None => self.apply(|x, y, p| color::mix(p, color(x, y), opacity, mode)),
        }
        self.add_film(mask, 0.5 * opacity);
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

    /// Atmospheric veil (fog, haze, light): optical blend toward `c` with
    /// strength 1 - exp(-density).
    pub fn veil(
        &mut self,
        mask: Option<&Mask>,
        c: Rgb,
        density: impl Fn(f32, f32) -> f32 + Sync,
    ) {
        self.dry();
        let g = |x: f32, y: f32, p: Rgb, cov: f32| {
            let t = 1.0 - (-density(x, y).max(0.0) * cov).exp();
            color::mix(p, c, t, Mix::Linear)
        };
        match mask {
            Some(m) => self.apply_masked(m, g),
            None => self.apply(|x, y, p| g(x, y, p, 1.0)),
        }
    }

    /// Fine, low-contrast mottling (uneven paint film / aged surface).
    pub fn mottle(&mut self, period: f32, strength: f32, seed: u32) {
        let n = crate::noise::Fbm::new(seed, 5, period);
        self.apply(|x, y, p| {
            let k = 1.0 + strength * n.get(x, y);
            [p[0] * k, p[1] * k, p[2] * k]
        });
    }

    /// Age cracks (craquelure): an irregular network of fine cracks, cell size
    /// in units. Darkens slightly and cuts grooves into the relief.
    pub fn craquelure(&mut self, cell: f32, strength: f32, seed: u64) {
        let warp = crate::noise::Fbm::new(seed as u32 + 77, 4, cell * 3.0);
        // where the paint film cracked: patchy, not uniform
        let cluster = crate::noise::Fbm::new(seed as u32 + 78, 3, cell * 12.0);
        let width = 0.012 * cell; // hairline, in units
        let inv = 1.0 / self.f.scale;
        let aa = 0.8 * inv; // ~1px antialias
        let w = self.f.w;
        let mut cracks = vec![0.0f32; w * self.f.h];
        cracks.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            let yu = (y as f32 + 0.5) * inv;
            for (x, out) in row.iter_mut().enumerate() {
                let xu = (x as f32 + 0.5) * inv;
                let wx = xu + warp.get(xu, yu) * cell * 0.3;
                let wy = yu + warp.get(xu + 91.0, yu - 37.0) * cell * 0.3;
                // a crack thinner than a pixel covers only part of it
                let line = |e: f32, wd: f32| {
                    let wp = wd.max(aa);
                    (1.0 - smoothstep(wp * 0.5 - aa * 0.5, wp * 0.5 + aa * 0.5, e)) * (wd / wp)
                };
                // main network, stretched a bit horizontally like canvas cracks
                let (e1, a1, b1) = voronoi_edge(wx / (cell * 1.25), wy / cell, seed);
                // each crack segment (cell pair) gets its own strength; many are faint
                let seg1 = hash2(a1.min(b1), a1.max(b1), seed + 5).powf(1.8);
                let c1 = line(e1 * cell, width) * seg1;
                // finer secondary cracks inside cells, mostly faint
                let (e2, a2, b2) = voronoi_edge(wx / (cell * 0.45), wy / (cell * 0.4), seed + 9);
                let seg2 = hash2(a2.min(b2), a2.max(b2), seed + 6).powf(3.0) * 0.6;
                let c2 = line(e2 * cell * 0.4, width * 0.7) * seg2;
                let k = smoothstep(-0.3, 0.4, cluster.get(xu, yu));
                *out = c1.max(c2) * (0.25 + 0.75 * k);
            }
        });
        let s = strength;
        self.px.par_iter_mut().zip(&cracks).for_each(|(p, c)| {
            let k = 1.0 - s * c;
            *p = [p[0] * k, p[1] * k, p[2] * k];
        });
        self.surf_gen += 1;
        self.height.par_iter_mut().zip(&cracks).for_each(|(h, c)| *h -= c * 8.0);
    }

    /// Light the surface relief (paint ridges + weave) from the upper left.
    /// `strength` ≈ 0.3–1.0; `gloss` adds a faint varnish sheen on ridges.
    pub fn relief(&mut self, strength: f32, gloss: f32) {
        self.dry();
        if std::env::var("PAINT_DEBUG").is_ok() {
            let mut v: Vec<f32> = self.film.clone();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let q = |p: f32| v[((v.len() - 1) as f32 * p) as usize];
            eprintln!("film quantiles 10/50/90/99: {:.2} {:.2} {:.2} {:.2}", q(0.1), q(0.5), q(0.9), q(0.99));
        }
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


/// Distance to the nearest Voronoi cell border (F2 - F1)/2, in cell units,
/// plus ids of the two nearest cells (identifies the crack segment).
fn voronoi_edge(x: f32, y: f32, seed: u64) -> (f32, i64, i64) {
    let (ix, iy) = (x.floor() as i64, y.floor() as i64);
    let (mut d1, mut d2) = (f32::MAX, f32::MAX);
    let (mut id1, mut id2) = (0i64, 0i64);
    for j in -1..=1 {
        for i in -1..=1 {
            let (cx, cy) = (ix + i, iy + j);
            let px = cx as f32 + hash2(cx, cy, seed);
            let py = cy as f32 + hash2(cx, cy, seed + 101);
            let d = ((px - x).powi(2) + (py - y).powi(2)).sqrt();
            let id = cx.wrapping_mul(73_856_093) ^ cy.wrapping_mul(19_349_663);
            if d < d1 {
                d2 = d1;
                id2 = id1;
                d1 = d;
                id1 = id;
            } else if d < d2 {
                d2 = d;
                id2 = id;
            }
        }
    }
    ((d2 - d1) * 0.5, id1, id2)
}

impl Canvas {
}
