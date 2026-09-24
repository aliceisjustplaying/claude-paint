//! The painting surface: color, paint relief and the woven support.

use crate::color::{self, Rgb};
use crate::mask::Mask;
use crate::pigment::Pigment;
use crate::rng::hash2;
use crate::surface::{COAT_UM, Linen, vnoise};

/// Fraction of a glaze layer that stays as film (the rest of the "thickness"
/// is how deep the color reads; a glaze is mostly medium, and thin).
const GLAZE_FILM: f32 = 0.3;
/// Thinnest glaze film that forms, µm: a numerical floor, not a physical
/// one. A float tail (a long soft falloff, a blurred mask's residue) ends
/// here, fading smoothly from `MIN_FILM_UM` to half of it (no cut, so no
/// edge). Real thin veils, a few tenths of a µm and up, are laid as asked
/// (Round 7: the 1 µm floor of 1742baa erased the winter painter's 0.34 µm
/// veil; the edge bug it was also meant to stop was the NaN in settle,
/// guarded there).
pub const MIN_FILM_UM: f32 = 0.05;
use rayon::prelude::*;

/// Film that forms from a request of `um` µm: all of it above `MIN_FILM_UM`,
/// fading smoothly (C¹) to nothing at half of it.
#[inline]
pub(crate) fn formed_film(um: f32) -> f32 {
    if um >= MIN_FILM_UM {
        um
    } else {
        um * crate::smoothstep(0.5 * MIN_FILM_UM, MIN_FILM_UM, um)
    }
}

/// The pixels a buffer holds and the units → pixels scale.
///
/// A buffer covers either the whole canvas or a window of it (a crop
/// render): `w × h` pixels whose top-left pixel is (`x0`, `y0`) of the whole
/// `full_w × full_h` canvas. Units always refer to the whole canvas
/// (`width()` is 1000, `height()` the whole height), so a painting program
/// is the same whatever window it is rendered in. Convert between units and
/// buffer pixels only through the methods below.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    /// Pixels held (the window).
    pub w: usize,
    pub h: usize,
    /// Pixels per unit.
    pub scale: f32,
    /// Window origin in whole-canvas pixels (0, 0 unless cropped).
    pub x0: usize,
    pub y0: usize,
    /// The whole canvas in pixels.
    pub full_w: usize,
    pub full_h: usize,
}

impl Frame {
    pub const WIDTH_UNITS: f32 = 1000.0;

    /// A frame covering a whole canvas of `w × h` pixels.
    pub fn new(w: usize, h: usize, scale: f32) -> Self {
        Frame { w, h, scale, x0: 0, y0: 0, full_w: w, full_h: h }
    }

    /// Canvas width in units (always 1000, also in a window).
    pub fn width(&self) -> f32 {
        Self::WIDTH_UNITS
    }
    /// Canvas height in units (of the whole canvas, also in a window).
    pub fn height(&self) -> f32 {
        self.full_h as f32 / self.scale
    }
    /// Buffer index of the pixel under a point in units (clamped to the buffer).
    #[inline]
    pub fn index(&self, x: f32, y: f32) -> usize {
        let px = (((x * self.scale) as isize) - self.x0 as isize).clamp(0, self.w as isize - 1) as usize;
        let py = (((y * self.scale) as isize) - self.y0 as isize).clamp(0, self.h as isize - 1) as usize;
        py * self.w + px
    }
    /// True if the point (units) falls on a pixel this buffer holds.
    #[inline]
    pub fn holds(&self, x: f32, y: f32) -> bool {
        let (px, py) = ((x * self.scale).floor(), (y * self.scale).floor());
        px >= self.x0 as f32 && py >= self.y0 as f32 && px < (self.x0 + self.w) as f32 && py < (self.y0 + self.h) as f32
    }
    /// Units of the center of buffer column `x` / row `y`.
    #[inline]
    pub fn ux(&self, x: usize) -> f32 {
        ((x + self.x0) as f32 + 0.5) * (1.0 / self.scale)
    }
    #[inline]
    pub fn uy(&self, y: usize) -> f32 {
        ((y + self.y0) as f32 + 0.5) * (1.0 / self.scale)
    }
    /// The same canvas, whole (no window).
    pub fn whole(&self) -> Frame {
        Frame::new(self.full_w, self.full_h, self.scale)
    }
    /// True unless this is a window of a larger canvas.
    pub fn is_whole(&self) -> bool {
        self.w == self.full_w && self.h == self.full_h
    }
    /// A window of this canvas: whole-canvas pixels `r` = (x0, y0, x1, y1),
    /// end-exclusive, clamped to the canvas.
    pub fn window(&self, r: (usize, usize, usize, usize)) -> Frame {
        let (fw, fh) = (self.full_w, self.full_h);
        let (x0, y0) = (r.0.min(fw - 1), r.1.min(fh - 1));
        let (x1, y1) = (r.2.clamp(x0 + 1, fw), r.3.clamp(y0 + 1, fh));
        Frame { w: x1 - x0, h: y1 - y0, scale: self.scale, x0, y0, full_w: fw, full_h: fh }
    }
    /// The window as whole-canvas pixels (x0, y0, x1, y1), end-exclusive.
    pub fn rect(&self) -> (usize, usize, usize, usize) {
        (self.x0, self.y0, self.x0 + self.w, self.y0 + self.h)
    }
    /// Whole-canvas pixel rect `r` intersected with the window, in buffer
    /// pixels; None if they don't meet.
    pub fn clip(&self, r: (usize, usize, usize, usize)) -> Option<(usize, usize, usize, usize)> {
        let (a, b, c, d) = self.rect();
        let (x0, y0, x1, y1) = (r.0.max(a), r.1.max(b), r.2.min(c), r.3.min(d));
        if x1 <= x0 || y1 <= y0 { None } else { Some((x0 - a, y0 - b, x1 - a, y1 - b)) }
    }
    /// A function of x (a horizon, a ridge line, the top of a ledge)
    /// tabulated at every pixel column's center of the whole canvas, so a
    /// mask that calls it for every pixel evaluates it once per column.
    /// Exact: at any other x it calls `g`.
    ///
    /// It returns a reference, which is `Copy`: the same profile can go into
    /// a mask closure and any number of `move` color closures, and is called
    /// as `ridge(x)`. The table (4 bytes per pixel column) and `g` live for
    /// the rest of the program, so make profiles once, not per stroke.
    ///
    /// ```ignore
    /// let n = Fbm::new(3, 4, 200.0);                     // Copy too
    /// let ridge = f.per_column(move |x| 420.0 + 30.0 * n.get(x, 0.0));
    /// let land = Mask::from_fn(f, move |x, y| if y > ridge(x) { 1.0 } else { 0.0 });
    /// let color = move |x: f32, y: f32| if y - ridge(x) < 20.0 { lit } else { shade };
    /// ```
    pub fn per_column<'a, G: Fn(f32) -> f32 + Sync + 'a>(&self, g: G) -> &'a (impl Fn(f32) -> f32 + Sync + 'a) {
        let (n, scale) = (self.full_w, self.scale);
        let inv = 1.0 / scale;
        let table: Vec<f32> = (0..n).into_par_iter().map(|i| g((i as f32 + 0.5) * inv)).collect();
        Box::leak(Box::new(move |x: f32| {
            let i = (x * scale - 0.5).round();
            if i >= 0.0 && (i as usize) < n && (i + 0.5) * inv == x { table[i as usize] } else { g(x) }
        }))
    }

    /// Index into a whole-canvas buffer (a mask) of buffer pixel `i`.
    #[inline]
    pub fn whole_index(&self, i: usize) -> usize {
        if self.is_whole() { i } else { (i / self.w + self.y0) * self.full_w + i % self.w + self.x0 }
    }
}

/// A crop render: only the window `units` = (x0, y0, x1, y1) of the canvas
/// is painted, at full resolution, plus `margin` units around it that are
/// painted but not saved (paint leveling, the brushes' feel of the surface
/// and strokes crossing the edge need some context).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Crop {
    pub units: [f32; 4],
    pub margin: f32,
}

static CROP: std::sync::Mutex<Option<Crop>> = std::sync::Mutex::new(None);

/// Make every canvas created from now on (`Canvas::new`) a crop render of
/// `crop` (None: whole canvases). `paintings::run::Run` sets this from
/// `--crop`, so painting programs need no changes.
pub fn set_crop(crop: Option<Crop>) {
    *CROP.lock().unwrap() = crop;
}

#[derive(Clone)]
pub struct Canvas {
    /// The pixels held: the whole canvas, or the window of a crop render.
    /// Masks are always whole (`frame()`).
    pub(crate) f: Frame,
    /// The part of the buffer `save` writes (buffer pixels, end-exclusive):
    /// all of it, or the crop without its margin.
    pub(crate) keep: (usize, usize, usize, usize),
    /// Linear RGB reflectance, row major.
    pub(crate) px: Vec<Rgb>,
    /// Physical surface height, µm: woven linen, ground layers, paint films.
    pub(crate) height: Vec<f32>,
    /// Accumulated paint film in coats (bookkeeping).
    pub(crate) film: Vec<f32>,
    /// Total thickness of the ground layers primed so far, µm (what
    /// `Cracks::aged` fits its craquelure to).
    pub(crate) ground_um: f32,
    pub(crate) linen: Option<Linen>,
    /// Physical size: millimeters per unit (the canvas is 1000 units wide).
    pub(crate) mm_per_unit: f32,
    /// Wet paint on top of the dry picture.
    pub(crate) wet: crate::wet::Wet,
    /// Bumped whenever the height changes; `base` caches the surface relief
    /// bristles feel.
    pub(crate) surf_gen: u64,
    pub(crate) base: Option<(u64, Vec<f32>)>,
    /// Loose graphite and chalk on the picture (None until something is
    /// drawn): `graphite`.
    pub(crate) drawing: Option<Box<crate::graphite::Drawing>>,
    /// What the brushes did and the hand time it took (`tally`): counted
    /// only, it never changes what is painted.
    pub(crate) tally: crate::tally::Tally,
    /// Hand time on (`set_hand_time`): the slice of hand time (minutes) a
    /// long pass is painted in, the paint ageing between slices.
    pub(crate) hand_slice: Option<f32>,
}

impl Canvas {
    /// `aspect` = width / height. A crop render if `set_crop` asked for one.
    pub fn new(width_px: usize, aspect: f32, ground: Rgb) -> Self {
        let crop = *CROP.lock().unwrap();
        Self::new_window(width_px, aspect, ground, crop)
    }

    /// A canvas that holds only the window `crop` (see `Crop`), or all of it.
    pub fn new_window(width_px: usize, aspect: f32, ground: Rgb, crop: Option<Crop>) -> Self {
        let h = (width_px as f32 / aspect).round() as usize;
        let whole = Frame::new(width_px, h, width_px as f32 / Frame::WIDTH_UNITS);
        let (f, keep) = match crop {
            None => (whole, (0, 0, width_px, h)),
            Some(c) => {
                let s = whole.scale;
                let px = |u: f32, m: f32| ((u + m) * s).round().max(0.0) as usize;
                let [a, b, cc, d] = c.units;
                let want = whole.window((px(a.min(cc), 0.0), px(b.min(d), 0.0), px(a.max(cc), 0.0), px(b.max(d), 0.0)));
                let m = c.margin.max(0.0);
                let f = whole.window((px(a.min(cc), -m), px(b.min(d), -m), px(a.max(cc), m), px(b.max(d), m)));
                let k = f.clip(want.rect()).expect("crop outside the canvas");
                (f, k)
            }
        };
        let n = f.w * f.h;
        Canvas {
            f,
            keep,
            px: vec![ground; n],
            height: vec![0.0; n],
            film: vec![0.0; n],
            ground_um: 0.0,
            linen: None,
            mm_per_unit: 0.7,
            wet: crate::wet::Wet::new(n),
            surf_gen: 0,
            base: None,
            drawing: None,
            tally: crate::tally::Tally::default(),
            hand_slice: None,
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
        let (ox, oy) = (self.f.x0, self.f.y0);
        let px = self.px_mm();
        let add: Vec<f32> = (0..w * h)
            .into_par_iter()
            .map(|i| {
                let (x, y) = ((i % w + ox) as f32 * px, (i / w + oy) as f32 * px);
                let n = 0.65 * vnoise(x / 0.3, y / 0.3, seed) + 0.35 * vnoise(x / 0.9, y / 0.9, seed + 1) - 0.5;
                um * (1.0 + texture * 1.4 * n).max(0.0)
            })
            .collect();
        let sv = vec![stiff; w * h];
        let t = self.settle((0, 0, w, h), &add, &sv);
        let pig = Pigment::masstone_hiding(color, hiding);
        self.px.par_iter_mut().zip(&t).for_each(|(p, &ti)| *p = pig.over(*p, ti / COAT_UM));
        self.film.par_iter_mut().zip(&t).for_each(|(f, &ti)| *f += ti / COAT_UM);
        self.ground_um += um;
    }

    /// Total thickness of the ground layers primed on this canvas, µm.
    pub fn ground_um(&self) -> f32 {
        self.ground_um
    }

    /// The whole canvas's frame, for building masks (masks always cover the
    /// whole canvas, also in a crop render, so strokes are planned the same).
    pub fn frame(&self) -> Frame {
        self.f.whole()
    }

    /// The pixels this canvas holds: the whole canvas or a crop window
    /// (`pixels()` and `surface_um()` are laid out in it).
    pub fn window(&self) -> Frame {
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

    /// Surface height in µm over the pixels `save` writes (a crop without its
    /// margin), row by row, with that width and height: for diagnostics
    /// that line the relief up with a saved PNG.
    pub fn kept_surface_um(&self) -> (usize, usize, Vec<f32>) {
        let (bw, (kx0, ky0, kx1, ky1)) = (self.f.w, self.keep);
        let v = (ky0..ky1).flat_map(|y| (kx0..kx1).map(move |x| y * bw + x)).map(|i| self.height[i]).collect();
        (kx1 - kx0, ky1 - ky0, v)
    }

    /// Panics unless `m` was made for this canvas's (whole) frame.
    #[track_caller]
    pub(crate) fn check_mask(&self, m: &Mask) {
        let f = self.f.whole();
        assert!(
            m.f == f && m.data.len() == f.w * f.h,
            "mask {}x{} does not match canvas {}x{} (build masks with canvas.frame())",
            m.f.w,
            m.f.h,
            f.w,
            f.h
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
        let f = self.f;
        self.px.par_chunks_mut(f.w).enumerate().for_each(|(y, row)| {
            let yu = f.uy(y);
            for (x, p) in row.iter_mut().enumerate() {
                *p = g(f.ux(x), yu, *p);
            }
        });
    }

    /// Like `apply`, but only where `m` > 0; `g` receives the coverage.
    pub fn apply_masked(&mut self, m: &Mask, g: impl Fn(f32, f32, Rgb, f32) -> Rgb + Sync) {
        self.check_mask(m);
        let f = self.f;
        let w = f.w;
        self.px.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            let yu = f.uy(y);
            let mi = f.whole_index(y * w);
            let mrow = &m.data[mi..mi + w];
            for (x, p) in row.iter_mut().enumerate() {
                let c = mrow[x];
                if c > 0.0 {
                    *p = g(f.ux(x), yu, *p, c);
                }
            }
        });
    }

    /// Kubelka–Munk glaze: a layer of `pigment` whose thickness is
    /// `thickness(x, y)` (times mask coverage, if given), in coats.
    ///
    /// The glaze is mostly medium: its film is `GLAZE_FILM` of a coat per
    /// coat of color depth. A request thinner than `MIN_FILM_UM` (0.05 µm)
    /// fades out smoothly, so a long soft falloff, or a blurred mask's
    /// float residue, ends softly, not at the last nonzero float; a thin
    /// veil of a few tenths of a µm is laid as asked. It dries at once (a glaze over dry
    /// paint; see `drying` for wet paint and time).
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
        let f = self.f;
        let (w, h) = (f.w, f.h);
        let th: Vec<f32> = (0..w * h)
            .into_par_iter()
            .map(|i| {
                let c = mask.map_or(1.0, |m| m.data[f.whole_index(i)]);
                if c <= 0.0 {
                    return 0.0;
                }
                let t = thickness(f.ux(i % w), f.uy(i / w)).max(0.0) * c;
                // (a NaN request is no glaze)
                if t.is_nan() || t <= 0.0 {
                    return 0.0;
                }
                let um = t * COAT_UM * GLAZE_FILM;
                let formed = formed_film(um);
                if formed >= um { t } else { t * formed / um }
            })
            .collect();
        let add: Vec<f32> = th.iter().map(|t| t * COAT_UM * GLAZE_FILM).collect();
        let t = self.settle_film(&add, 0.05);
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
    /// the long, subtle gradients Friedrich loves). A crop render saves just
    /// the crop (without its margin).
    pub fn save(&mut self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        self.dry();
        let (bw, (kx0, ky0, kx1, ky1)) = (self.f.w, self.keep);
        let (w, h) = (kx1 - kx0, ky1 - ky0);
        // dither by whole-canvas pixel, so a crop matches a whole render
        let (ox, oy) = (self.f.x0 + kx0, self.f.y0 + ky0);
        let mut buf = vec![0u8; w * h * 3];
        buf.par_chunks_mut(w * 3).enumerate().for_each(|(y, row)| {
            for x in 0..w {
                let p = self.px[(y + ky0) * bw + x + kx0];
                let (gx, gy) = ((x + ox) as i64, (y + oy) as i64);
                for c in 0..3 {
                    let d = hash2(gx, gy, c as u64 * 7 + 1) - hash2(gx, gy, c as u64 * 7 + 2);
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

#[cfg(test)]
mod tests {
    use crate::color::hex;
    use crate::mask::Mask;
    use crate::pigment::Pigment;
    use crate::style::Style;

    /// Largest channel change per distance band (40 units) from `at`.
    fn change_by_distance(c: &super::Canvas, before: &[crate::color::Rgb], at: (f32, f32)) -> Vec<f32> {
        let f = c.f;
        let mut bins = vec![0.0f32; 16];
        for i in 0..c.px.len() {
            let d = ((f.ux(i % f.w) - at.0).powi(2) + (f.uy(i / f.w) - at.1).powi(2)).sqrt();
            let e = (0..3).map(|k| (c.px[i][k] - before[i][k]).abs()).fold(0.0, f32::max);
            assert!(c.px[i].iter().all(|v| v.is_finite()) && c.height[i].is_finite(), "non-finite pixel at d {d}");
            let b = ((d / 40.0) as usize).min(15);
            bins[b] = bins[b].max(e);
        }
        bins
    }

    /// Amnesia friction 2 (winter #11): a glaze with a long Gaussian falloff
    /// is tiny but positive far out (down to f32 denormals). It used to
    /// settle to NaN there and paint the glaze's full masstone in a ring
    /// ending where `exp` underflows: a hard pale edge. Now the change falls
    /// off monotonically with the thickness and is invisible in the tail.
    #[test]
    fn glaze_long_falloff_has_no_edge() {
        let st = Style::friedrich();
        let mut c = st.prepare(300, 1.5, 3);
        let before = c.px.clone();
        let at = (500.0f32, 300.0f32);
        c.glaze(&Pigment::transparent(hex("#e8e0c0")), None, move |x, y| {
            let d = ((x - at.0).powi(2) + (y - at.1).powi(2)).sqrt();
            0.35 * (-(d / 38.0).powi(2)).exp()
        });
        let bins = change_by_distance(&c, &before, at);
        assert!(bins[0] > 2.0 / 255.0, "the glaze shows at its center: {bins:?}");
        for b in 3..bins.len() {
            assert!(bins[b] < 0.5 / 255.0, "visible glaze in the tail at band {b}: {bins:?}");
        }
    }

    /// Amnesia friction 2 (coast #0a): a blurred mask leaves float residue
    /// out to the canvas edges; a glaze through it must not lay a rectangle.
    #[test]
    fn glaze_through_blurred_mask_leaves_no_rectangle() {
        let st = Style::friedrich();
        let mut c = st.prepare(300, 1.5, 4);
        let before = c.px.clone();
        let m = Mask::from_fn(c.frame(), |x, y| if (x - 300.0).abs() < 20.0 && (y - 200.0).abs() < 60.0 { 1.0 } else { 0.0 }).blur(1.6);
        let residue = m.data.iter().filter(|&&v| v > 0.0 && v < 1e-3).count();
        c.glaze(&Pigment::transparent(hex("#3a2a1a")), Some(&m), |_, _| 1.2);
        let f = c.f;
        let mut far = 0.0f32;
        for i in 0..c.px.len() {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            assert!(c.px[i].iter().all(|v| v.is_finite()) && c.height[i].is_finite());
            if (x - 300.0).abs() > 60.0 || (y - 200.0).abs() > 100.0 {
                far = far.max((0..3).map(|k| (c.px[i][k] - before[i][k]).abs()).fold(0.0, f32::max));
            }
        }
        assert!(far < 0.5 / 255.0, "glaze outside the blurred mask: {far} ({residue} residue pixels)");
    }

    /// The film fades in smoothly below the minimum (no step, no overshoot).
    #[test]
    fn formed_film_is_smooth_and_monotone() {
        let mut last = 0.0f32;
        for k in 0..=4000 {
            let um = k as f32 * 0.0005;
            let f = super::formed_film(um);
            assert!(f >= last - 1e-7 && f <= um + 1e-7, "{um}: {f}");
            assert!(f - last < 0.0025, "step at {um}");
            last = f;
        }
        assert_eq!(super::formed_film(0.02), 0.0);
        assert_eq!(super::formed_film(0.2), 0.2);
        assert_eq!(super::formed_film(3.0), 3.0);
    }

    /// Round 7 (notes/round7/winter_ab.md): the winter painter's veil is
    /// 0.045-0.06 coats, a film of 0.34-0.45 µm. The 1 µm floor erased it
    /// and left a bare, lighter oval; a thin veil must lay what was asked,
    /// in proportion to a thicker one.
    #[test]
    fn a_thin_veil_is_laid() {
        let st = Style::friedrich();
        let dark = |coats: f32| {
            let mut c = st.prepare(200, 1.5, 5);
            let before = c.px.clone();
            c.glaze(&Pigment::transparent(hex("#4a3a30")), None, move |_, _| coats);
            let lum = |p: &crate::color::Rgb| 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2];
            before.iter().zip(&c.px).map(|(a, b)| (lum(a) - lum(b)) as f64).sum::<f64>() / c.px.len() as f64
        };
        let (thin, thicker) = (dark(0.045), dark(0.18));
        assert!(thin > 0.0 && thin > 0.2 * thicker, "a 0.34 µm veil darkens {thin}, 1.35 µm {thicker}");
    }
}
