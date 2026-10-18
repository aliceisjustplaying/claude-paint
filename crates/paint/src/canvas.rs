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
#[cfg_attr(feature = "replay-effects", derive(serde::Serialize, serde::Deserialize))]
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
    /// Physical size: millimeters per unit (the canvas is 1000 units wide).
    pub(crate) mm_per_unit: f32,
    /// Wet paint on top of the dry picture.
    pub(crate) wet: crate::wet::Wet,
    /// Bumped whenever the height changes; `base` caches the surface relief
    /// bristles feel.
    pub(crate) surf_gen: u64,
    pub(crate) base: Option<(u64, Vec<f32>)>,
    /// Engine 7: `base`, cached block by block (`crate::engine7::contact_cache`).
    pub(crate) e7_contact: crate::engine7::contact_cache::ContactCache,
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
    pub(crate) quarter_tiles: bool,
    pub(crate) adaptive_sweep: bool,
    pub(crate) liquid_film_levels: bool,
    /// Engine 7: the `glaze` and `scumble` hands lay a thin film, and their
    /// coverage is the share of the passage it reaches (`Handling::film`).
    pub(crate) glaze_film: bool,
    /// The `work` pass under way with a film ceiling (`Handling::film`):
    /// the most wet film (coats) it lays on any pixel, and its first stroke
    /// id. Transient.
    pub(crate) film_pass: Option<(f32, u32)>,
    /// Engine 7: a rag keeps what it lifts, and a mask wipe lifts only
    /// inside its mask (`rag`).
    pub(crate) rag_lift: bool,
    /// Engine 7: a knife's paint laid on wet paint stays on top of it
    /// (`Canvas::knife`).
    pub(crate) knife_skim: bool,
    /// Engine 7: a blender lays the wet paint it carries only where wet paint
    /// was when its pass began, and a little way past it (`Canvas::work`).
    pub(crate) blend_feather: bool,
    /// Engine 7: a brush's paint laid on wet paint stays on top of it
    /// (`bristle::BRUSH_SKIM_COATS`).
    pub(crate) brush_skim: bool,
    /// While a blender pass with `blend_feather` runs: how much of the paint
    /// a bristle carries each pixel takes (0..1, see `Canvas::receptive`).
    pub(crate) receptive: Option<Vec<f32>>,
    /// The bare cloth of a raw canvas (None: a primed canvas): `soak`.
    pub(crate) soak: Option<Box<crate::soak::Soak>>,
    /// The drain switch (`with_drain`): an absorbent ground drains paint as
    /// its pigment packs (`bristle::ground_drain`).
    pub(crate) drain: bool,
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
            mm_per_unit: 0.7,
            wet: crate::wet::Wet::new(n),
            surf_gen: 0,
            base: None,
            e7_contact: Default::default(),
            drawing: None,
            tally: crate::tally::Tally::default(),
            hand_slice: None,
            engine: crate::ENGINE,
            quarter_tiles: false,
            adaptive_sweep: false,
            liquid_film_levels: false,
            glaze_film: false,
            film_pass: None,
            rag_lift: false,
            knife_skim: false,
            blend_feather: false,
            brush_skim: false,
            receptive: None,
            soak: None,
            drain: false,
        }
    }

    /// Paint with engine version `v` (see `crate::ENGINE`): a replay
    /// paints as the version its painting was painted with.
    pub fn with_engine(mut self, v: u32) -> Self {
        self.engine = v;
        self.quarter_tiles &= v == 5;
        self.adaptive_sweep &= v == 5;
        self.liquid_film_levels &= v >= 4;
        self.glaze_film &= crate::engine7::active(v);
        self.rag_lift &= crate::engine7::active(v);
        self.knife_skim &= matches!(v, crate::engine7::ENGINE | crate::REACH_END);
        self.blend_feather &= matches!(v, crate::engine7::ENGINE | crate::REACH_END);
        self.brush_skim &= matches!(v, crate::engine7::ENGINE | crate::REACH_END);
        self
    }
    pub fn engine(&self) -> u32 {
        self.engine
    }

    /// Turn on the drain switch (an easel log's `--@ drain`, engine 5 and
    /// later): an absorbent ground drains the film over it as its pigment
    /// packs, to the oil left when the pigment packs and below it only for
    /// pigments coarser than the ground's pores, the packed layer growing up
    /// from the ground while the surface keeps its oil, flow and gloss
    /// (`bristle::ground_drain`); paint comes off from the top down; a film's
    /// gloss reads its surface's oil against its pigment's packing
    /// (`bake`). Without it an absorbent ground draws oil from the paint
    /// just laid, as engine 4 has it (`bristle::ground_draw`), and a painting
    /// is as it always was. A checkpoint keeps it (version 14).
    pub fn with_drain(mut self, on: bool) -> Self {
        self.drain = on;
        self
    }
    /// The drain switch (`with_drain`).
    pub fn drain(&self) -> bool {
        self.drain
    }

    pub fn with_quarter_tiles(mut self, enabled: bool) -> Self {
        self.quarter_tiles = enabled && self.engine == 5;
        self
    }

    pub fn quarter_tiles(&self) -> bool {
        self.quarter_tiles
    }

    pub(crate) fn work_tile_mode(&self) -> Option<String> {
        if self.engine != 5 {
            return None;
        }
        std::env::var("PAINT_SMALL_TILES").ok().or_else(|| self.quarter_tiles.then(|| "quarter".to_string()))
    }

    pub fn with_adaptive_sweep(mut self, enabled: bool) -> Self {
        self.adaptive_sweep = enabled && self.engine == 5;
        self
    }

    pub fn adaptive_sweep(&self) -> bool {
        self.adaptive_sweep
    }

    pub fn with_glaze_film(mut self, enabled: bool) -> Self {
        self.glaze_film = enabled && crate::engine7::active(self.engine);
        self
    }

    pub fn glaze_film(&self) -> bool {
        self.glaze_film
    }

    pub fn with_liquid_film_levels(mut self, enabled: bool) -> Self {
        self.liquid_film_levels = enabled && self.engine >= 4;
        self
    }

    pub fn liquid_film_levels(&self) -> bool {
        self.liquid_film_levels
    }

    pub fn with_rag_lift(mut self, enabled: bool) -> Self {
        self.rag_lift = enabled && crate::engine7::active(self.engine);
        self
    }

    pub fn rag_lift(&self) -> bool {
        self.rag_lift
    }

    pub fn with_knife_skim(mut self, enabled: bool) -> Self {
        self.knife_skim = enabled && matches!(self.engine, crate::engine7::ENGINE | crate::REACH_END);
        self
    }

    pub fn knife_skim(&self) -> bool {
        self.knife_skim
    }

    pub fn with_blend_feather(mut self, enabled: bool) -> Self {
        self.blend_feather = enabled && matches!(self.engine, crate::engine7::ENGINE | crate::REACH_END);
        self
    }

    pub fn blend_feather(&self) -> bool {
        self.blend_feather
    }

    pub fn with_brush_skim(mut self, enabled: bool) -> Self {
        self.brush_skim = enabled && matches!(self.engine, crate::engine7::ENGINE | crate::REACH_END);
        self
    }

    pub fn brush_skim(&self) -> bool {
        self.brush_skim
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
        crate::prepared_optics::PreparedLighting::new(
            &self.wet_surface(), self.f.w, self.f.h, self.px_mm() * 1000.0,
        ).lit(&self.seen(), &self.wet.vol, azimuth, elevation, strength)
    }

    pub fn prepare_optics(&self) -> crate::PreparedOptics {
        crate::PreparedOptics::capture(self)
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
mod review_lighting_tests {
    use super::*;

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

#[cfg(test)]
mod tests {
    // From engine 4 a liquid film bridges the fine relief under it (surface.rs,
    // `BRIDGE_UM`): a thick film of thinned liquid has a level top over the
    // weave, where the same thickness of dry paint keeps the weave's texture.
    // Outside the film the two surfaces are the same.
    #[test]
    #[cfg(tube_box)]
    fn a_liquid_film_levels_the_weave_under_it_from_engine_4() {
        let mut liquid = crate::Style::oil().prepare(160, 1.0, 3).with_engine(4).with_liquid_film_levels(true);
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
}
