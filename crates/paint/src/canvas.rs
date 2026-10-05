//! The painting surface: color, paint relief and the woven support.

use crate::color::{self, Rgb};
use crate::mask::Mask;
use crate::pigment::Pigment;
use crate::rng::hash2;
use crate::surface::{COAT_UM, Linen, vnoise};

/// How glossy an oil ground is (a lead white priming, semi-matte).
pub(crate) const OIL_GROUND_GLOSS: f32 = 0.5;
/// The share of light a paint surface reflects at its first surface (oil,
/// n ≈ 1.5: about 4%). A glossy surface sends it away from the viewer; a
/// matte one scatters it back, a veil of white over the colors (engine 4).
pub(crate) const SURFACE_REFLECTANCE: f32 = 0.04;
/// The oil a fresh absorbent (chalk and glue) ground can draw out of the
/// paint laid on it, coats per unit of `absorbent` (about 15 µm of oil).
pub(crate) const ABSORB_COATS: f32 = 0.6;

/// A matte surface's veil: color `p` seen with the first-surface reflection
/// a surface of gloss `g` scatters back to the viewer.
#[inline]
pub(crate) fn haze(p: Rgb, g: f32) -> Rgb {
    let k = SURFACE_REFLECTANCE * (1.0 - g.clamp(0.0, 1.0));
    [p[0] * (1.0 - k) + k, p[1] * (1.0 - k) + k, p[2] * (1.0 - k) + k]
}

/// Fraction of a glaze layer that stays as film (the rest of the "thickness"
/// is how deep the color reads; a glaze is mostly medium, and thin).
const GLAZE_FILM: f32 = 0.3;
/// Thinnest glaze film that forms, µm: a numerical floor, not a physical
/// one. A float tail (a long soft falloff, a blurred mask's residue) ends
/// here, fading smoothly from `MIN_FILM_UM` to half of it (no cut, so no
/// edge). Real thin veils, a few tenths of a µm and up, are laid as asked.
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
    /// A function of x (e.g. a curve y = g(x))
    /// tabulated at every pixel column's center of the whole canvas, so a
    /// mask that calls it for every pixel evaluates it once per column.
    /// Exact: at any other x it calls `g`.
    ///
    /// It returns a reference, which is `Copy`: the same profile can go into
    /// a mask closure and any number of `move` color closures, and is called
    /// as `profile(x)`. The table (4 bytes per pixel column) and `g` live for
    /// the rest of the program, so make profiles once, not per stroke.
    ///
    /// ```ignore
    /// let n = Fbm::new(3, 4, 200.0);                     // Copy too
    /// let profile = f.per_column(move |x| 420.0 + 30.0 * n.get(x, 0.0));
    /// let below = Mask::from_fn(f, move |x, y| if y > profile(x) { 1.0 } else { 0.0 });
    /// let color = move |x: f32, y: f32| if y - profile(x) < 20.0 { a } else { b };
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
    /// How glossy the dry surface is, 0 (matte: lean paint, an absorbent
    /// ground) to 1 (oily paint, varnish). Engine 4 shows a matte surface
    /// with the light its first surface scatters (`haze`).
    pub(crate) gloss: Vec<f32>,
    /// The oil an absorbent ground can still draw out of paint laid on it,
    /// coats (0: an oil ground, or one sealed by paint). Engine 4.
    pub(crate) absorb: Vec<f32>,
    /// Whether any of the ground is absorbent (so the brushes look).
    pub(crate) absorb_any: bool,
    /// Total thickness of the ground layers primed so far, µm (what
    /// `Cracks::aged` fits its craquelure to).
    pub(crate) ground_um: f32,
    pub(crate) linen: Option<Linen>,
    /// A paper support (engine 6), instead of linen.
    pub(crate) paper: Option<crate::paper::Paper>,
    /// The micro-roughness under a pixel, µm: the mean depth of the pores
    /// below the surface's top envelope (`paper`), what a pastel stick and
    /// a finger meet. Empty unless the support is paper; elsewhere it
    /// follows the surface's gloss (`Canvas::micro_um`).
    pub(crate) micro: Vec<f32>,
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
    /// The engine version it is painted with (`crate::ENGINE`).
    pub(crate) engine: u32,
    /// The bare cloth of a raw canvas (None: a primed canvas): `soak`.
    pub(crate) soak: Option<Box<crate::soak::Soak>>,
    /// A sheet of paper laid over part of the picture (engine 7, `sheet.rs`).
    pub(crate) sheet: Option<crate::sheet::Sheet>,
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
            gloss: vec![OIL_GROUND_GLOSS; n],
            absorb: vec![0.0; n],
            absorb_any: false,
            ground_um: 0.0,
            linen: None,
            paper: None,
            micro: Vec::new(),
            mm_per_unit: 0.7,
            wet: crate::wet::Wet::new(n),
            surf_gen: 0,
            base: None,
            drawing: None,
            tally: crate::tally::Tally::default(),
            hand_slice: None,
            engine: crate::ENGINE,
            soak: None,
            sheet: None,
        }
    }

    /// Paint with engine version `v` (see `crate::ENGINE`): a replay
    /// paints as the version its painting was painted with.
    pub fn with_engine(mut self, v: u32) -> Self {
        self.engine = v;
        self
    }
    pub fn engine(&self) -> u32 {
        self.engine
    }

    /// Physical width of the painting in mm (default 700).
    pub fn with_size_mm(mut self, width_mm: f32) -> Self {
        self.mm_per_unit = width_mm / Frame::WIDTH_UNITS;
        self.build_support();
        self
    }

    /// Use a sheet of paper as the support (engine 6): its surface from its
    /// fibres, flocs, mould, felt and pressing (`paper::lay`), its pores
    /// taking oil as an absorbent ground does.
    pub fn with_paper(mut self, p: crate::paper::Paper) -> Self {
        self.linen = None;
        self.paper = Some(p);
        self.build_support();
        // the pores take oil: the sheet's pore volume, in coats
        let cap = p.absorbent.clamp(0.0, 1.0) * p.porosity * p.caliper_um() / crate::surface::COAT_UM;
        self.absorb.iter_mut().for_each(|v| *v = cap);
        self.absorb_any = cap > 0.0;
        // bare paper is matte
        self.gloss.iter_mut().for_each(|v| *v = 0.05);
        self
    }

    /// The micro-roughness at pixel `i` of the window, µm (see `micro`): the
    /// paper's where paper is bare or only stained; over a paint film, the
    /// film's own, which follows its gloss (a glossy film is smooth, about
    /// 0.1 µm; a lean, matte one has its pigment standing proud, about
    /// 2 µm: notes/research/dry_pigment_optics.md §4, estimates).
    pub(crate) fn micro_um(&self, i: usize) -> f32 {
        let paint = crate::lerp(2.0, 0.1, self.gloss[i].clamp(0.0, 1.0));
        match self.micro.get(i) {
            // a film thinner than the pores leaves them open
            Some(&m) => {
                let film_um = self.film[i] * crate::surface::COAT_UM;
                let t = (film_um / m.max(1e-3)).min(1.0);
                crate::lerp(m, paint, t)
            }
            None => paint,
        }
    }

    /// How stiffly the surface at pixel `i` gives under a point load, MPa
    /// per µm (a Winkler foundation: the sheet's z modulus over its
    /// caliper). A canvas on its stretcher, or a board, is all but rigid.
    pub(crate) fn give_mpa_per_um(&self, _i: usize) -> f32 {
        match self.paper {
            Some(p) => p.z_mpa / p.caliper_um().max(10.0),
            None => 10.0,
        }
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
    /// veil of a few tenths of a µm is laid as asked. It dries at once.
    /// The caller must ensure the mask-covered substrate is touch-dry;
    /// this operation neither advances time nor dries paint elsewhere.
    pub fn glaze(
        &mut self,
        pigment: &Pigment,
        mask: Option<&Mask>,
        thickness: impl Fn(f32, f32) -> f32 + Sync,
    ) {
        if let Some(m) = mask {
            self.check_mask(m);
        }
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

    /// The top ground layer just laid is absorbent (`absorbent` 0..1, a
    /// chalk and glue ground: it draws oil out of the paint laid on it and
    /// dries matte) or not (an oil ground: semi-matte, absorbs nothing).
    pub fn ground_finish(&mut self, absorbent: f32) {
        let a = absorbent.clamp(0.0, 1.0);
        self.absorb.iter_mut().for_each(|v| *v = a * ABSORB_COATS);
        let g = crate::lerp(OIL_GROUND_GLOSS, 0.05, a);
        self.gloss.iter_mut().for_each(|v| *v = g);
        self.absorb_any = a > 0.0;
    }

    /// A varnish was laid over the whole picture: its surface is glossy, and
    /// a ground still absorbent anywhere is sealed.
    pub fn varnished(&mut self) {
        self.gloss.iter_mut().for_each(|v| *v = 1.0);
        self.absorb.iter_mut().for_each(|v| *v = 0.0);
        self.absorb_any = false;
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

    /// The picture as it is seen under a raking light, without changing
    /// anything (the painter's look; `relief` lights the finished picture):
    /// the dry relief plus the wet paint's own thickness, lit from `azimuth`
    /// (degrees: 0 from the right, 90 from the top, 135 from the upper left)
    /// at `elevation` degrees above the canvas. Ridges cast shadows along the
    /// light, and wet paint has an oily sheen. The higher the light, the
    /// more of the room's light fills the shadows (a raking lamp in a dim
    /// room, or a gallery's light from above).
    pub fn seen_lit(&self, azimuth: f32, elevation: f32, strength: f32) -> Vec<Rgb> {
        use rayon::prelude::*;
        // the room's light that fills the shadows: little where one low lamp
        // rakes across the picture in a dim room, much under a gallery's high
        // light, where walls and ceiling light the picture too
        let ambient = crate::lerp(0.3, 0.55, crate::smoothstep(10.0, 55.0, elevation));
        let base = self.seen();
        let (w, h) = (self.f.w, self.f.h);
        let um_px = self.px_mm() * 1000.0;
        // the surface: dry height plus the wet film where paint is wet
        let surf = self.wet_surface();
        // elevation as the easel takes it, 0 to 90 degrees: overhead (90) has no
        // horizontal part and casts no shadow; grazing (0) is a lamp in the
        // canvas's plane, which lights no flat paint, only slopes turned to it
        let elevation = elevation.clamp(0.0, 90.0);
        let (overhead, grazing) = (elevation >= 90.0, elevation <= 1e-3);
        let (az, el) = (azimuth.to_radians(), elevation.to_radians());
        // toward the light, in pixel axes (y runs down: light from the top is -y)
        let (lx, ly, lz) = if overhead {
            (0.0, 0.0, 1.0)
        } else if grazing {
            (az.cos(), -az.sin(), 0.0)
        } else {
            (el.cos() * az.cos(), -el.cos() * az.sin(), el.sin())
        };
        let k = 0.5 / um_px;
        // µm the light ray climbs per pixel toward the light (overhead and grazing, none)
        let rise = if overhead || grazing { 0.0 } else { el.tan() * um_px };
        let (hi, lo) = surf.par_iter().fold(|| (f32::MIN, f32::MAX), |(a, b), &v| (a.max(v), b.min(v))).reduce(|| (f32::MIN, f32::MAX), |(a, b), (c, d)| (a.max(c), b.min(d)));
        let m = (lx * lx + ly * ly).sqrt();
        let march = m > 1e-6 && (rise > 0.0 || grazing);
        let steps = if !march {
            0
        } else if grazing {
            w.saturating_add(h)
        } else {
            (((hi - lo) / rise).ceil() as usize).max(1).min(w.saturating_add(h))
        };
        let (sx, sy) = if march { (lx / m, ly / m) } else { (0.0, 0.0) };
        let at = |x: isize, y: isize| surf[(y.clamp(0, h as isize - 1) as usize) * w + x.clamp(0, w as isize - 1) as usize];
        // shadows are cast by the relief a pixel can resolve: bumps finer than
        // a pixel (a stroke's furrows) shade by their slope, above, and don't
        // shadow the paint around them
        let shade_surf = crate::surface::box_blur(&surf, w, h, 1);
        // Bound the actual sampled surface, including blur rounding.
        let shade_hi = shade_surf.par_iter().copied().reduce(|| f32::MIN, f32::max);
        let sat = |x: isize, y: isize| shade_surf[(y.clamp(0, h as isize - 1) as usize) * w + x.clamp(0, w as isize - 1) as usize];
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
                // paint edges round over (as `relief`): soft-limit the slope,
                // so the furrows of a stroke model it and don't turn it to bark
                let g = (dx * dx + dy * dy).sqrt();
                let soft = 1.0 / (1.0 + g / 1.2);
                let (dx, dy) = (dx * soft, dy * soft);
                let m = (dx * dx + dy * dy + 1.0).sqrt();
                let n = [-dx / m, -dy / m, 1.0 / m];
                let ndl = (n[0] * lx + n[1] * ly + n[2] * lz).max(0.0);
                // cast shadow: does the surface toward the light rise above the ray?
                let h0 = shade_surf[i];
                let mut lit = 1.0f32;
                for s in 1..=steps {
                    let ray_height = h0 + rise * s as f32;
                    // (at or above the highest paint nothing further shades it: a
                    // grazing ray, which doesn't climb, stops there at once)
                    if ray_height >= shade_hi || lit == 0.0 {
                        break;
                    }
                    let (px, py) = (x as f32 + sx * s as f32, y as f32 + sy * s as f32);
                    if px < 0.0 || py < 0.0 || px >= w as f32 || py >= h as f32 { break; }
                    let over = sat(px.round() as isize, py.round() as isize) - ray_height;
                    if over > 0.0 {
                        // (grazing, anything higher hides the lamp)
                        lit = if grazing { 0.0 } else { lit.min(1.0 - (over / (0.5 * rise)).min(1.0)) };
                    }
                }
                // (a slope facing a low lamp is lit more than the flat canvas,
                // 1; capped, so the lowest lights don't burn ridges out to white;
                // grazing, only slopes toward the lamp are lit, by n·l itself: lz is 0)
                let direct = if grazing { ndl * lit } else { ndl * lit / lz };
                let diffuse = (ambient + (1.0 - ambient) * direct).min(1.6);
                let shade = (1.0 + strength * (diffuse - 1.0)).max(0.0);
                // sheen: wet oil shines, dry paint barely
                let wet = (self.wet.vol[i] * 4.0).min(1.0);
                let ndh = (n[0] * hv[0] + n[1] * hv[1] + n[2] * hv[2]).max(0.0);
                let spec = (0.02 + 0.10 * wet) * ndh.powf(40.0) * lit;
                let p = base[i];
                [(p[0] * shade + spec).max(0.0), (p[1] * shade + spec).max(0.0), (p[2] * shade + spec).max(0.0)]
            })
            .collect()
    }

    /// Save as an 8-bit sRGB PNG with triangular dither (prevents banding in
    /// long, low-contrast gradients). A crop render saves just
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
                let i = (y + ky0) * bw + x + kx0;
                let p = if self.engine >= 4 { haze(self.px[i], self.gloss[i]) } else { self.px[i] };
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
    // Solvent contributes geometry under raking light, while remaining
    // optically clear in the diffuse view. The same relief must shade alike
    // whether its height comes from the dry support or the liquid film.
    // (Engine 3, where a film adds its thickness as it lies: from engine 4 a
    // liquid film levels the fine relief under it, the test below.)
    #[test]
    #[cfg(tube_box)]
    fn raking_light_includes_solvent_thickness() {
        let mut liquid = crate::Style::oil().prepare(48, 1.0, 3).with_engine(3);
        let mut raised = crate::Style::oil().prepare(48, 1.0, 3).with_engine(3);
        let diffuse = liquid.seen();
        liquid.wet.solv = vec![0.0; liquid.height.len()];
        for i in 0..liquid.height.len() {
            let x = i % liquid.f.w;
            let thickness = if (16..32).contains(&x) { 600.0 } else { 0.0 };
            liquid.wet.solv[i] = thickness;
            raised.height[i] += thickness;
        }
        assert_eq!(liquid.seen(), diffuse, "clear solvent leaves diffuse color unchanged");
        for azimuth in [0.0, 135.0, 270.0] {
            assert!(liquid.seen_lit(azimuth, 10.0, 1.0) == raised.seen_lit(azimuth, 10.0, 1.0), "equal film geometry shades alike at {azimuth} degrees");
        }
    }

    // From engine 4 a liquid film bridges the fine relief under it (surface.rs,
    // `BRIDGE_UM`): a thick film of thinned liquid has a level top over the
    // weave, where the same thickness of dry paint keeps the weave's texture.
    // Outside the film the two surfaces are the same.
    #[test]
    #[cfg(tube_box)]
    fn a_liquid_film_levels_the_weave_under_it_from_engine_4() {
        let mut liquid = crate::Style::oil().prepare(160, 1.0, 3).with_engine(4);
        let mut raised = crate::Style::oil().prepare(160, 1.0, 3).with_engine(4);
        let w = liquid.f.w;
        let band = |i: usize| (40..120).contains(&(i % w));
        liquid.wet.solv = vec![0.0; liquid.height.len()];
        for i in 0..liquid.height.len() {
            if band(i) {
                liquid.wet.solv[i] = 600.0;
                raised.height[i] += 600.0;
            }
        }
        let (l, r) = (liquid.wet_surface(), raised.wet_surface());
        // the band's relief away from its edges: the mean step between neighbours along a row
        let relief = |s: &[f32]| {
            let steps: Vec<f32> = (0..s.len()).filter(|&i| (50..109).contains(&(i % w))).map(|i| (s[i + 1] - s[i]).abs()).collect();
            steps.iter().sum::<f32>() / steps.len() as f32
        };
        assert!(relief(&r) > 0.0, "the ground has a weave to level");
        assert!(relief(&l) < 0.5 * relief(&r), "a liquid film levels the weave: {} against dry paint's {}", relief(&l), relief(&r));
        for i in (0..l.len()).filter(|&i| !band(i)) {
            assert_eq!(l[i], r[i], "outside the film, pixel {i}");
        }
    }

    #[cfg(tube_box)]
    use crate::color::hex;
    #[cfg(tube_box)]
    use crate::mask::Mask;
    #[cfg(tube_box)]
    use crate::pigment::Pigment;
    #[cfg(tube_box)]
    use crate::style::Style;

    // Engine contract: glazing a dry patch must not advance time or cure
    // other paint. The Lua guard alone cannot prevent an engine dry().
    #[test]
    #[cfg(tube_box)]
    fn masked_glaze_preserves_wet_paint_and_clock_elsewhere() {
        let mut c = Style::oil().prepare(100, 1.0, 3);
        let pal = crate::Palette::tube_box();
        let paint = pal.pile(vec![(pal.tubes.iter().position(|t| t.name == "bone black").unwrap(), 1.0)]).laid(0.0);
        let mut held = crate::Held::new(crate::Tool::hog_flat(20.0), 1);
        held.load(paint, 1.0);
        c.drag(&mut held, &crate::Gesture::line((100.0, 200.0), (400.0, 200.0)).pressure(0.8, 0.8), None);
        let stages = c.stages();
        assert!(stages.iter().any(|s| *s != crate::Stage::Dry));
        let clock = c.clock();
        let m = Mask::from_fn(c.frame(), |x, y| if x > 700.0 && y > 700.0 { 1.0 } else { 0.0 });
        c.glaze(&Pigment::transparent(hex("#302010")), Some(&m), |_, _| 0.5);
        assert_eq!(c.clock(), clock, "glaze must not advance the drying clock");
        assert_eq!(c.stages(), stages, "paint outside the glaze stays wet");
    }

    /// Largest channel change per distance band (40 units) from `at`.
    #[cfg(tube_box)]
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

    /// A glaze with a long Gaussian falloff
    /// is tiny but positive far out (down to f32 denormals). The pixels stay
    /// finite there, and the change falls off with the thickness to below
    /// half an 8-bit step in the tail.
    #[test]
    #[cfg(tube_box)]
    fn glaze_long_falloff_has_no_edge() {
        let st = Style::oil();
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

    /// A blurred mask leaves float residue
    /// out to the canvas edges; a glaze through it must not lay a rectangle.
    #[test]
    #[cfg(tube_box)]
    fn glaze_through_blurred_mask_leaves_no_rectangle() {
        let st = Style::oil();
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

    /// A thin veil of 0.045-0.06 coats, a film of 0.34-0.45 µm, is laid as
    /// asked: it darkens by more than a fifth as much as one four times as
    /// thick.
    #[test]
    #[cfg(tube_box)]
    fn a_thin_veil_is_laid() {
        let st = Style::oil();
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

#[cfg(test)]
mod review_lighting_tests {
    use super::*;

    #[test]
    fn review_tall_relief_casts_shadows_beyond_64_pixels() {
        let mut c = Canvas::new(300, 1.0, [0.8; 3]).with_size_mm(300.0);
        let plain = c.seen_lit(0.0, 3.0, 1.0);
        for y in 0..300 { for x in 200..205 { c.height[y * 300 + x] += 8000.0; } }
        let lit = c.seen_lit(0.0, 3.0, 1.0);
        let i = 150 * 300 + 100;
        assert!(lit[i][0] < plain[i][0] * 0.8, "a ridge 100 pixels toward the light must cast a shadow");
    }

    /// The light's two ends, exactly: overhead (90°) casts no shadow and
    /// lights flat paint fully; grazing (0°), a lamp in the canvas's plane,
    /// leaves flat paint to the room's light, lights the ridge's face turned
    /// to it, and hides whatever lies behind the ridge. No NaN at either end.
    #[test]
    fn review_overhead_and_grazing_lights_are_exact() {
        let mut c = Canvas::new(300, 1.0, [0.8; 3]).with_size_mm(300.0);
        for y in 0..300 { for x in 200..205 { c.height[y * 300 + x] += 8000.0; } }
        let (over, graze, low) = (c.seen_lit(0.0, 90.0, 1.0), c.seen_lit(0.0, 0.0, 1.0), c.seen_lit(0.0, 30.0, 1.0));
        assert!(over.iter().chain(&graze).all(|p| p.iter().all(|v| v.is_finite())));
        // the light comes from the right (azimuth 0): x 100 lies behind the ridge, x 250 in front of it
        let (behind, front, face) = (150 * 300 + 100, 150 * 300 + 250, 150 * 300 + 205);
        assert!((over[behind][0] - over[front][0]).abs() < 1e-4, "overhead: no shadow ({} vs {})", over[behind][0], over[front][0]);
        assert!(graze[front][0] < low[front][0] * 0.6, "grazing: flat paint gets only the room's light");
        assert!(graze[face][0] > graze[front][0] * 1.5, "grazing: the ridge's face toward the lamp is lit");
        assert!(graze[behind][0] <= graze[front][0], "grazing: nothing behind the ridge is lit by the lamp");
    }
}
