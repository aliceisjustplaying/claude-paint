//! Wet paint sitting on top of the dry picture.
//!
//! Every pixel holds a volume of wet paint (thickness, in "layer units": 1.0
//! is one normal coat), its pigment mixture as a Mixbox latent vector of the
//! masstone (mixing is linear in latent space, weighted by volume) and its
//! Kubelka–Munk scattering per coat (mixed linearly by volume, as K and S mix
//! in the two-constant KM model; absorption follows from masstone and S).
//! Brushes exchange paint with this layer. It ages as the painting's clock
//! runs (`Canvas::wait`, see `drying`); `Canvas::dry` waits until it is
//! touch-dry, baking it into the dry picture with Kubelka–Munk.
//!
//! **What a paint's color means.** `Paint::color` is its *masstone*: the
//! color the paint has laid thick, which is also how it looks laid over
//! paint of that same color, at any thickness. Hiding decides how much of a
//! different underlayer shows through a thin coat. So a mark mixed to the
//! color of the field it sits in disappears into it, whether it is body
//! color or a thin scumble; a thin coat over a different color lands between
//! the two, as real paint does. To ask "what will this look like *here*",
//! use `Paint::over` and `Canvas::under`. Choose tube proportions explicitly
//! with `Palette::pile` and thin the resulting mixture with medium.
//!

use crate::canvas::Canvas;
use crate::surface::COAT_UM;

/// Bare for cure-on-lay/lift (strictly less than, not the contact cutoff).
const CURE_BARE_COATS: f32 = 1e-5;
/// A fine hair starts new coverage below this separate contact cutoff.
const COVER_BARE_COATS: f32 = 1e-6;
/// Solvent flow counts equality as a film.
const FLOW_BARE_COATS: f32 = 1e-5;
/// A wet film blocks drawing only strictly above this cutoff.
const DRAWING_BARE_COATS: f32 = 1e-5;
/// Drying observes >= as wet, and < as bare (not logical complements for NaN).
const DRYING_BARE_COATS: f32 = 1e-5;
/// The visible-film cutoff is strict <.
const VISIBLE_BARE_COATS: f32 = 1e-5;
/// Public cure reporting returns zero strictly below this cutoff.
const REPORTED_CURE_BARE_COATS: f32 = 1e-5;
/// A brush picks up film strictly above this contact cutoff.
const PICKUP_MIN_COATS: f32 = 1e-6;
/// A knife cuts film strictly above its own contact cutoff.
const KNIFE_MIN_COATS: f32 = 1e-6;
/// Rag lift skips equality at its own contact cutoff.
const RAG_MIN_COATS: f32 = 1e-6;
/// The share of tube paint volume that is oil (about 30–45%).
const OIL_SHARE: f32 = 0.4;
use crate::color::{Rgb, luminance};
use crate::pigment::{Pigment, hiding_of, scatter_for};

pub const LAT: usize = mixbox::LATENT_SIZE;
pub type Latent = [f32; LAT];
/// Paint properties mixed by volume alongside the pigment: [KM scattering per
/// coat, stiffness, drying rate, solvent, oil].
/// Stiffness 0 = fluid, medium-rich glaze; 1 = stiff tube paint. Drying rate
/// relative to average paint (see `drying::drier`). Solvent and oil are
/// engine 4's (see `Paint`; engine 3's thinner keeps its solvent beside the
/// paint, `crate::thinner`).
pub type Prop = [f32; 5];

#[inline]
fn lerp_prop(p: &mut Prop, q: Prop, a: f32) {
    for k in 0..p.len() {
        p[k] += (q[k] - p[k]) * a;
    }
}

/// A paint as squeezed from the tube and thinned with medium.
///
/// It carries its Kubelka–Munk scattering `scatter` directly, so a mixture
/// handed to the brush keeps the S it was mixed with, however opaque it is
/// (hiding saturates near 1 and can't carry a large S). Name paints by
/// hiding with `Paint::new`/`with_hiding`; `hiding()` reports it back.
#[derive(Clone, Copy, Debug)]
pub struct Paint {
    /// Masstone, linear RGB: the paint's color laid thick (and over itself).
    pub color: Rgb,
    /// Kubelka–Munk scattering per coat (flat across channels; absorption
    /// follows from it and the masstone). See `pigment::scatter_for`.
    pub scatter: f32,
    /// Stiffness: 0 = fluid, rich in medium (a glaze), 1 = stiff tube paint.
    /// Sets how the paint levels as it dries (see `surface::settle`). How
    /// much paint goes on the brush is the separate `amount` of `Held::load`.
    pub stiff: f32,
    /// How fast it dries, relative to average paint (1): lead white and
    /// umber about 2, bone black and lakes 0.3–0.4 (`drying::drier`). With
    /// film thickness and fat (low `stiff`) it sets how long the paint stays
    /// open (see `Canvas::wait`).
    pub drying: f32,
    /// Share of solvent (turpentine, spirits) by volume in what the brush
    /// takes up, 0 (none) to 0.95 (`crate::thinner`; engine 3). The other
    /// fields stay the paint's own: solvent is never mixed into them.
    pub thinner: f32,
    /// The share of the paint's volume that is volatile solvent (turpentine,
    /// 0..0.9): it makes the paint flow on the brush and evaporates as it is
    /// laid, leaving a film of the paint's own body, that much thinner
    /// (engine 4; `pile{turps=}`).
    pub solvent: f32,
    /// How rich in oil the paint is, relative to tube paint (1): more with
    /// medium, less blotted or drawn into an absorbent ground. A film's gloss
    /// follows it (engine 4).
    pub oil: f32,
}

impl Paint {
    /// Its properties as they mix in a brush and on the canvas (`Prop`).
    pub fn prop(&self) -> Prop {
        [self.scatter(), self.stiff, self.drying, self.solvent, self.oil]
    }
    /// A paint of masstone `color` whose one coat hides `hiding` (contrast
    /// ratio: over black ÷ over white; 0.05 = glaze, 0.5 = scumble,
    /// 0.92 = body).
    pub fn new(color: Rgb, hiding: f32, stiff: f32) -> Self {
        Paint { color, scatter: scatter_for(luminance(color), hiding), stiff, drying: 1.0, thinner: 0.0, solvent: 0.0, oil: 1.0 }
    }
    /// A paint of masstone `color` that scatters `scatter` per coat.
    pub fn km(color: Rgb, scatter: f32, stiff: f32) -> Self {
        Paint { color, scatter, stiff, drying: 1.0, thinner: 0.0, solvent: 0.0, oil: 1.0 }
    }
    pub fn body(color: Rgb) -> Self {
        Paint::new(color, 0.92, 1.0)
    }
    pub fn scumble(color: Rgb) -> Self {
        Paint::new(color, 0.5, 0.6)
    }
    /// This paint with its scattering set so one coat hides `hiding`
    /// (the masstone, stiffness and drying rate stay).
    pub fn with_hiding(self, hiding: f32) -> Self {
        Paint { scatter: scatter_for(luminance(self.color), hiding), ..self }
    }
    /// This paint with stiffness `stiff`.
    pub fn with_stiff(self, stiff: f32) -> Self {
        Paint { stiff, ..self }
    }
    /// This paint drying at `rate` relative to average paint (see
    /// `drying::drier`: lead white 2, bone black 0.4).
    pub fn with_drying(self, rate: f32) -> Self {
        Paint { drying: rate, ..self }
    }
    /// This paint thinned: `t` (0..0.95) of what the brush takes up is
    /// solvent, the rest this paint, unchanged (`crate::thinner`; engine 3).
    pub fn with_thinner(self, t: f32) -> Self {
        Paint { thinner: if t.is_nan() { 0.0 } else { t.clamp(0.0, 0.95) }, ..self }
    }
    /// The share of solvent by volume (`with_thinner`).
    pub fn thinner(&self) -> f32 {
        self.thinner
    }
    /// Hiding power of one coat (contrast ratio), derived from the
    /// scattering; for reporting (it rounds to 1 for strong scatterers).
    pub fn hiding(&self) -> f32 {
        hiding_of(luminance(self.color), self.scatter)
    }
    pub fn latent(&self) -> Latent {
        mixbox::linear_float_rgb_to_latent(&self.color)
    }
    /// Kubelka–Munk scattering per coat (the `scatter` field).
    pub fn scatter(&self) -> f32 {
        self.scatter
    }
    /// The paint as a Kubelka–Munk layer (per coat).
    pub fn pigment(&self) -> Pigment {
        Pigment::masstone(self.color, self.scatter)
    }
    /// What `coats` of this paint look like laid over `under`.
    pub fn over(&self, under: Rgb, coats: f32) -> Rgb {
        self.pigment().over(under, coats)
    }
}

#[derive(Clone)]
pub(crate) struct Wet {
    pub(crate) vol: Vec<f32>,
    pub(crate) lat: Vec<Latent>,
    /// [scattering, stiffness, drying rate, solvent, oil] of the wet paint.
    pub(crate) hide: Vec<Prop>,
    /// Which stroke last laid paint here (a stroke barely re-picks its own paint).
    pub(crate) stroke: Vec<u32>,
    /// Stroke that last touched a pixel, and the film floor that stroke may
    /// not lift below (one pass lifts only part of the film).
    pub(crate) touched: Vec<u32>,
    pub(crate) floor: Vec<f32>,
    /// Share of the pixel the wet paint covers (1 = all of it). Only the
    /// hairs of a pointed tool, finer than a pixel, lay paint over part of
    /// one; `dry` composites that paint at its real thickness over that
    /// share, so a hairline looks alike at any resolution.
    pub(crate) cover: Vec<f32>,
    /// Id of the stroke being painted.
    pub(crate) current: u32,
    /// Dirty bounding box in pixels (x0, y0, x1, y1), if any paint is wet.
    pub(crate) dirty: Option<(usize, usize, usize, usize)>,
    /// The painting's clock and how far each film has dried (`drying`).
    pub(crate) clock: crate::drying::Clock,
    /// Solvent in the open film, µm (not coats: it is saved as µm, and a
    /// save must restore it bit for bit), engine 3: empty before engine 3
    /// (`crate::thinner`). Saved (PAINTCK9).
    pub(crate) solv: Vec<f32>,
    /// The wet film (paint + solvent, coats) the stroke `laid_id` has added
    /// to each pixel, against its ceiling (`thinner::stroke_limit_um`).
    /// Transient: an entry counts only for the stroke `laid_id`, and when
    /// the stroke ids wrap (`Canvas::next_stroke_ids`) every entry is
    /// forgotten, so a stale entry is never read; not saved. Empty before
    /// engine 3 (allocated by the film's tile view).
    pub(crate) laid: Vec<f32>,
    pub(crate) laid_id: Vec<u32>,
}

pub(crate) fn drying_cutoff() -> f32 {
    DRYING_BARE_COATS
}

impl Wet {
    #[inline]
    pub(crate) fn below_rag_contact_cutoff(v: f32) -> bool {
        v <= RAG_MIN_COATS
    }
    #[inline]
    pub(crate) fn volume(&self, i: usize) -> f32 {
        self.vol[i]
    }
    #[inline]
    pub(crate) fn latent(&self, i: usize) -> Latent {
        self.lat[i]
    }
    #[inline]
    pub(crate) fn properties(&self, i: usize) -> Prop {
        self.hide[i]
    }
    #[inline]
    pub(crate) fn timed(&self) -> bool {
        self.clock.px.len() == self.vol.len()
    }
    #[inline]
    pub(crate) fn drying(&self, i: usize) -> Option<crate::drying::Px> {
        self.clock.px.get(i).copied()
    }
    #[inline]
    pub(crate) fn solvent_um(&self, i: usize) -> f32 {
        self.solv.get(i).copied().unwrap_or(0.0)
    }
    /// Drawing's strict > comparison intentionally differs from drying's >=.
    #[inline]
    pub(crate) fn blocks_drawing(&self, i: usize) -> bool {
        self.vol[i] > DRAWING_BARE_COATS
    }

    pub(crate) fn next_stroke_ids(&mut self, n: u32) -> u32 {
        let old = self.current;
        let first = old.wrapping_add(1).max(1);
        self.current = first.wrapping_add(n);
        if first <= old || self.current < first {
            self.forget_laid();
        }
        first
    }

    /// Evaporation changes solvent alone; no material mixing or cure reset.
    pub(crate) fn evaporate_solvent(&mut self, i: usize, dt: f32, engine: u32) -> bool {
        use crate::thinner::{SOLVENT_FLOOR, THIN_FILM_UM, evaporation_tau_min};
        let s = &mut self.solv[i];
        if *s > 0.0 {
            let h = self.vol[i] * COAT_UM;
            *s = (*s as f64 * (-(dt as f64) / evaporation_tau_min(h)).exp()) as f32;
            // Engine 5's trace floor is separate from bare-paint thresholds.
            let floor = if engine >= 5 { SOLVENT_FLOOR.max(1e-5 * (h + THIN_FILM_UM)) } else { SOLVENT_FLOOR };
            if *s < floor {
                *s = 0.0;
            }
        }
        *s > 0.0
    }

    pub fn new(n: usize) -> Self {
        Wet { vol: vec![0.0; n], lat: vec![[0.0; LAT]; n], hide: vec![[0.0, 0.5, 1.0, 0.0, 1.0]; n], stroke: vec![0; n], touched: vec![0; n], floor: vec![0.0; n], cover: vec![1.0; n], current: 0, dirty: None, clock: Default::default(), solv: Vec::new(), laid: Vec::new(), laid_id: Vec::new() }
    }

    /// Allocate the solvent and the strokes' ceilings (a thinned paint is
    /// about to be laid).
    pub(crate) fn ensure_solvent(&mut self) {
        let n = self.vol.len();
        if self.solv.len() != n {
            self.solv = vec![0.0; n];
        }
        if self.laid.len() != n {
            self.laid = vec![0.0; n];
            self.laid_id = vec![0; n];
        }
    }

    /// Forget every stroke's ceiling (the stroke ids wrapped). 0 is no
    /// stroke's id.
    pub(crate) fn forget_laid(&mut self) {
        self.laid.fill(0.0);
        self.laid_id.fill(0);
    }

    /// Whether any open film holds solvent (within the dirty box; `w` is
    /// the buffer's width).
    pub(crate) fn has_solvent(&self, w: usize) -> bool {
        let Some((x0, y0, x1, y1)) = self.dirty else { return false };
        if self.solv.is_empty() {
            return false;
        }
        let x1 = x1.min(w);
        let h = self.solv.len() / w.max(1);
        (y0..y1.min(h)).any(|y| self.solv[y * w + x0..y * w + x1].iter().any(|&s| s > 0.0))
    }

    pub fn touch(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        self.dirty = Some(match self.dirty {
            None => (x0, y0, x1, y1),
            Some((a, b, c, d)) => (a.min(x0), b.min(y0), c.max(x1), d.max(y1)),
        });
    }
}

/// Exclusive wet-film contact view, bounded to a pixel footprint. Copies
/// exist only inside `paint_tiles`, where the scheduler serializes overlaps.
pub(crate) struct Tile<'a> {
    borrow: std::marker::PhantomData<&'a mut Wet>,
    not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
    first: usize,
    end: usize,
    columns: (usize, usize),
    height: *const f32,
    frame: crate::canvas::Frame,
    /// The buffers' pixels: a window (origin `ox`, `oy`, size `w` × `h`) of
    /// the whole `fw` × `fh` canvas. Brush geometry works in whole-canvas
    /// pixels; only buffer indices are translated.
    vol: *mut f32,
    lat: *mut Latent,
    hide: *mut Prop,
    stroke: *mut u32,
    touched: *mut u32,
    floor: *mut f32,
    cover: *mut f32,
    base: *const f32,
    /// Drying state (null until the canvas has waited): how open or tacky
    /// the paint under a bristle is (see `drying::feel`).
    dry: *mut crate::drying::Px,
    /// Paint laid or moved updates the film's cure at once (see `add`;
    /// engine 2). Engine 1 left it to the next `wait`.
    cure_now: bool,
    /// Engine 4: bristles in stiff paint gather into clumps that lay it in
    /// ridges and furrows and plough it aside a clump's width (`exchange`).
    clump: bool,
    /// Engine 4: paint laid loses its solvent at once (it evaporates in
    /// minutes), and an absorbent ground draws oil out of it (`add`).
    lean: bool,
    /// The ground's remaining absorbency per pixel (coats of oil; null when
    /// none of it is absorbent or before engine 4).
    absorb: *mut f32,
    /// Solvent in the open film, and each pixel's wet film laid by the
    /// stroke `laid_id` against its ceiling (`Wet::solv`, `Wet::laid`):
    /// null before engine 3.
    solv: *mut f32,
    laid: *mut f32,
    laid_id: *mut u32,
}
// SAFETY: a Tile is exclusive and not Sync or Clone. Its borrowed buffers
// remain alive until it is dropped; sending it transfers that exclusive view.
unsafe impl Send for Tile<'_> {}
// Only this private owner may share the raw buffers. It never exposes its
// whole-window tile; each worker receives a non-Sync, footprint-bounded tile.
struct SharedTiles<'a>(Tile<'a>);
// SAFETY: paint_tiles uses run_ordered, and every read/write is checked against
// the same rectangles run_ordered uses to serialize overlapping footprints.
unsafe impl Sync for SharedTiles<'_> {}
impl SharedTiles<'_> {
    fn at(&self, rect: crate::sched::Rect) -> Tile<'_> {
        self.0.copy_for(rect)
    }
}

impl Wet {
    /// Borrow all film buffers for a contact pass. The scheduler runs copies
    /// only on nonoverlapping footprints; the lifetime keeps buffers alive
    /// and prevents reallocation until the whole pass has joined.
    pub(crate) fn tile<'a>(&'a mut self, f: crate::canvas::Frame, engine: u32, base: &'a [f32], height: &'a [f32], absorb: &'a mut [f32], absorb_any: bool) -> Tile<'a> {
        let n = f.w.checked_mul(f.h).expect("film window area overflow");
        if engine >= 3 {
            self.ensure_solvent();
        }
        assert!(
            [self.vol.len(), self.lat.len(), self.hide.len(), self.stroke.len(), self.touched.len(), self.floor.len(), self.cover.len(), base.len(), height.len(), absorb.len()]
                .iter()
                .all(|&l| l == n),
            "canvas buffers out of sync with frame"
        );
        Tile {
            borrow: std::marker::PhantomData,
            not_sync: std::marker::PhantomData,
            first: 0,
            end: n,
            columns: (0, f.w),
            height: height.as_ptr(),
            frame: f,
            vol: self.vol.as_mut_ptr(),
            lat: self.lat.as_mut_ptr(),
            hide: self.hide.as_mut_ptr(),
            stroke: self.stroke.as_mut_ptr(),
            touched: self.touched.as_mut_ptr(),
            floor: self.floor.as_mut_ptr(),
            cover: self.cover.as_mut_ptr(),
            base: base.as_ptr(),
            dry: if self.clock.px.len() == n { self.clock.px.as_mut_ptr() } else { std::ptr::null_mut() },
            cure_now: engine >= 2,
            clump: engine >= 4,
            lean: engine >= 4,
            absorb: if engine >= 4 && absorb_any { absorb.as_mut_ptr() } else { std::ptr::null_mut() },
            solv: if self.solv.len() == n { self.solv.as_mut_ptr() } else { std::ptr::null_mut() },
            laid: if self.laid.len() == n { self.laid.as_mut_ptr() } else { std::ptr::null_mut() },
            laid_id: if self.laid_id.len() == n { self.laid_id.as_mut_ptr() } else { std::ptr::null_mut() },
        }
    }
}
impl Tile<'_> {
    #[inline]
    pub(crate) fn frame(&self) -> crate::canvas::Frame { self.frame }
    #[inline]
    pub(crate) fn clumps(&self) -> bool { self.clump }

    fn copy_for(&self, rect: crate::sched::Rect) -> Tile<'_> {
        assert!(rect.0 <= rect.2 && rect.1 <= rect.3, "invalid film footprint");
        let (x0, x1) = (rect.0.clamp(self.frame.x0, self.frame.x0 + self.frame.w) - self.frame.x0, rect.2.clamp(self.frame.x0, self.frame.x0 + self.frame.w) - self.frame.x0);
        let (y0, y1) = (rect.1.clamp(self.frame.y0, self.frame.y0 + self.frame.h) - self.frame.y0, rect.3.clamp(self.frame.y0, self.frame.y0 + self.frame.h) - self.frame.y0);
        let first = y0 * self.frame.w + x0;
        let end = if x0 < x1 && y0 < y1 { (y1 - 1) * self.frame.w + x1 } else { first };
        Tile {
            borrow: std::marker::PhantomData,
            not_sync: std::marker::PhantomData,
            first,
            end,
            columns: (x0, x1),
            height: self.height,
            frame: self.frame,
            vol: self.vol,
            lat: self.lat,
            hide: self.hide,
            stroke: self.stroke,
            touched: self.touched,
            floor: self.floor,
            cover: self.cover,
            base: self.base,
            dry: self.dry,
            cure_now: self.cure_now,
            clump: self.clump,
            lean: self.lean,
            absorb: self.absorb,
            solv: self.solv,
            laid: self.laid,
            laid_id: self.laid_id,
        }
    }

    /// Parallel edits stay inside the footprints whose overlap the scheduler
    /// orders. No raw view or shared mutable film escapes the joined pass.
    pub(crate) fn paint_tiles<R: Send>(self, order: &[usize], rects: &[Option<crate::sched::Rect>], paint: impl Fn(&Tile<'_>, usize) -> R + Sync) -> Vec<R> {
        let shared = SharedTiles(self);
        crate::sched::run_ordered(order, rects, |t| paint(&shared.at(rects[t].expect("tile without footprint")), t))
    }

    #[inline]
    fn check(&self, i: usize) {
        // The cached span bounds the rows and the first/last columns;
        // the column check rejects the holes between those rows. A whole-
        // width view needs no division in this per-pixel hot path.
        assert!(i >= self.first && i < self.end, "film pixel outside tile footprint");
        if self.columns != (0, self.frame.w) {
            let x = i % self.frame.w;
            assert!(x >= self.columns.0 && x < self.columns.1, "film pixel outside tile footprint");
        }
    }
    #[inline]
    pub(crate) fn brush_can_pick_up(v: f32) -> bool {
        v > PICKUP_MIN_COATS
    }
    #[inline]
    pub(crate) fn knife_can_cut(v: f32) -> bool {
        v > KNIFE_MIN_COATS
    }
    #[inline]
    pub(crate) fn volume(&self, i: usize) -> f32 {
        self.check(i);
        unsafe { *self.vol.add(i) }
    }
    #[inline]
    pub(crate) fn latent(&self, i: usize) -> Latent {
        self.check(i);
        unsafe { *self.lat.add(i) }
    }
    #[inline]
    pub(crate) fn properties(&self, i: usize) -> Prop {
        self.check(i);
        unsafe { *self.hide.add(i) }
    }
    #[inline]
    pub(crate) fn stroke(&self, i: usize) -> u32 {
        self.check(i);
        unsafe { *self.stroke.add(i) }
    }
    #[inline]
    pub(crate) fn mark_stroke(&self, i: usize, id: u32) {
        self.check(i);
        unsafe {
            *self.stroke.add(i) = id;
        }
    }
    #[inline]
    pub(crate) fn contact(&self, i: usize) -> f32 {
        self.check(i);
        unsafe { *self.base.add(i) }
    }
    #[inline]
    pub(crate) fn height(&self, i: usize) -> f32 {
        self.check(i);
        unsafe { *self.height.add(i) }
    }
    #[inline]
    pub(crate) fn timed(&self) -> bool {
        !self.dry.is_null()
    }
    #[inline]
    pub(crate) fn drying(&self, i: usize) -> Option<crate::drying::Px> {
        self.check(i);
        if self.timed() { unsafe { Some(*self.dry.add(i)) } } else { None }
    }
    #[inline]
    pub(crate) fn cure(&self, i: usize) -> f32 {
        self.drying(i).map_or(0.0, |p| p.cure)
    }
    #[inline]
    pub(crate) fn fluid(&self, i: usize) -> f32 {
        self.drying(i).map_or(1.0, |p| crate::drying::fluid(p.cure))
    }
    #[inline]
    pub(crate) fn has_ceiling(&self) -> bool {
        !self.laid.is_null() && !self.laid_id.is_null()
    }
    #[inline]
    pub(crate) fn solvent_coats(&self, i: usize) -> f32 {
        self.check(i);
        if self.solv.is_null() { 0.0 } else { unsafe { *self.solv.add(i) / COAT_UM } }
    }

    /// The first contact sets the stroke's pickup floor, not its lay ceiling.
    #[inline]
    pub(crate) fn pickup_floor(&self, i: usize, id: u32, v: f32, pickup: f32, fl: f32) -> f32 {
        self.check(i);
        unsafe {
            if *self.touched.add(i) != id {
                *self.touched.add(i) = id;
                *self.floor.add(i) = v * (1.0 - pickup * fl);
            }
            *self.floor.add(i)
        }
    }

    /// Brush pickup uses solvent in µm before converting to its reservoir's
    /// coats × pixel area. Keep that order distinct from ploughing and rag lift.
    #[inline]
    pub(crate) fn lift_brush(&self, i: usize, take: f32, v: f32, px_area: f32) -> Option<f32> {
        self.check(i);
        let mut solvent = None;
        unsafe {
            if !self.solv.is_null() {
                let sv = &mut *self.solv.add(i);
                if *sv > 0.0 {
                    let ts = (*sv * take / v).min(*sv);
                    *sv -= ts;
                    solvent = Some(ts / COAT_UM * px_area);
                }
            }
        }
        self.take(i, take);
        solvent
    }

    #[inline]
    fn hair_cover(&self, i: usize, fine: bool, share: f32) {
        self.check(i);
        unsafe {
            let cv = &mut *self.cover.add(i);
            *cv = if fine { ((if *self.vol.add(i) < COVER_BARE_COATS { 0.0 } else { *cv }) + share).min(1.0) } else { 1.0 };
        }
    }
    #[inline]
    pub(crate) fn lay_hair(&self, i: usize, v: f32, fine: bool, share: f32, id: u32, lat: &Latent, hide: Prop, cure: f32) {
        self.hair_cover(i, fine, share);
        self.add(i, v, lat, hide, cure);
        self.mark_stroke(i, id);
    }
    #[inline]
    pub(crate) fn lay_liquid_hair(&self, i: usize, mut d: f32, phi: f32, cap: f32, capped: bool, fine: bool, share: f32, id: u32, lat: &Latent, hide: Prop, cure: f32) -> f32 {
        self.check(i);
        unsafe {
            if capped {
                assert!(self.has_ceiling(), "film ceiling unavailable");
                let used = if *self.laid_id.add(i) == id { *self.laid.add(i) } else { 0.0 };
                d = d.min((cap - used).max(0.0));
                *self.laid_id.add(i) = id;
                *self.laid.add(i) = used + d;
            }
            if d > 0.0 {
                self.hair_cover(i, fine, share);
                self.add(i, d * (1.0 - phi), lat, hide, cure);
                if !self.solv.is_null() {
                    *self.solv.add(i) += d * phi * COAT_UM;
                }
                *self.stroke.add(i) = id;
            }
        }
        d
    }

    /// The knife cuts paint first, then its independently calculated solvent.
    #[inline]
    pub(crate) fn lift_knife(&self, i: usize, v: f32, solvent: f32) {
        self.take(i, v);
        unsafe {
            if !self.solv.is_null() {
                *self.solv.add(i) -= solvent * COAT_UM;
            }
        }
    }
    /// Knife lays add paint and solvent before setting full coverage. Unlike a
    /// hair or droplet there is no per-stroke ceiling and no positivity guard.
    #[inline]
    pub(crate) fn lay_knife(&self, i: usize, v: f32, solvent: f32, lat: &Latent, hide: Prop, cure: f32) {
        self.add(i, v, lat, hide, cure);
        unsafe {
            if !self.solv.is_null() {
                *self.solv.add(i) += solvent * COAT_UM;
            }
            *self.cover.add(i) = 1.0;
        }
    }

    /// One accepted plough destination. Cure is reread for each destination,
    /// including after a previous lift left the source bare.
    #[inline]
    pub(crate) fn move_plough(&self, i: usize, j: usize, m: f32, v: f32, fine: bool, capped: bool, cap: f32, id: u32, l: &Latent, hd: Prop) {
        self.check(i);
        self.check(j);
        unsafe {
            let sol = if self.solv.is_null() { 0.0 } else { *self.solv.add(i) / COAT_UM };
            let vi = *self.vol.add(i);
            let ms = if sol > 0.0 && vi > 0.0 { (sol * m / vi).min(sol) } else { 0.0 };
            let (m, ms) = if capped && j != i && m > 0.0 {
                assert!(self.has_ceiling(), "film ceiling unavailable");
                let used = if *self.laid_id.add(j) == id { *self.laid.add(j) } else { 0.0 };
                let room = (cap - used).max(0.0);
                let f = if m + ms > room { room / (m + ms) } else { 1.0 };
                *self.laid_id.add(j) = id;
                *self.laid.add(j) = used + (m + ms) * f;
                (m * f, ms * f)
            } else {
                (m, ms)
            };
            if j != i && m > 0.0 {
                if ms > 0.0 {
                    let si = &mut *self.solv.add(i);
                    let mu = (ms * COAT_UM).min(*si);
                    *si -= mu;
                    *self.solv.add(j) += mu;
                }
                let cure = if self.dry.is_null() { 0.0 } else { (*self.dry.add(i)).cure };
                // the paint moved covers its share of
                // the pixel it came from
                // (its film there thins but still covers it)
                let cj = &mut *self.cover.add(j);
                *cj = if fine { ((if *self.vol.add(j) < COVER_BARE_COATS { 0.0 } else { *cj }) + (m / v.max(1e-9)).min(1.0) * *self.cover.add(i)).min(1.0) } else { 1.0 };
                self.take(i, m);
                self.add(j, m, l, hd, cure);
            }
        }
    }

    /// Lay `v` coats of paint of cure `cure` (0: fresh) at pixel `i`: it
    /// mixes by volume into the color and into the film's cure, so the next
    /// bristle feels the film as it now is, not as it was at the last wait.
    #[inline]
    fn add(&self, i: usize, v: f32, lat: &Latent, hide: Prop, cure: f32) {
        unsafe {
            self.check(i);
            if v <= 0.0 {
                return;
            }
            // engine 4: the solvent in the paint evaporates as it is laid
            // (in minutes: before anything else can work it), leaving a film
            // of the paint's own body that much thinner
            let (v, hide) = if self.lean && hide[3] > 0.0 {
                let mut h = hide;
                h[3] = 0.0;
                (v * (1.0 - hide[3].clamp(0.0, 0.95)), h)
            } else {
                (v, hide)
            };
            let vol = &mut *self.vol.add(i);
            let l = &mut *self.lat.add(i);
            let hd = &mut *self.hide.add(i);
            let (t, a) = mix_material(*vol, l, hd, v, lat, hide);
            if self.cure_now && !self.dry.is_null() {
                // (a film too thin for `wait` to count is bare: no cure)
                let p = &mut *self.dry.add(i);
                p.cure = if t < CURE_BARE_COATS { 0.0 } else { p.cure + (cure - p.cure) * a };
            }
            *vol = t;
            // engine 4: an absorbent ground under the film draws oil out of
            // the paint just laid, until its pores are full: a thin wash goes
            // lean (stiff, matte, quick to set), thick paint barely notices
            if !self.absorb.is_null() {
                let cap = &mut *self.absorb.add(i);
                if *cap > 0.0 {
                    let oil_in = v * OIL_SHARE * hd[4].max(0.0);
                    let take = (*cap).min(0.8 * oil_in);
                    if take > 0.0 {
                        *cap -= take;
                        let film_oil = (t * OIL_SHARE * hd[4].max(0.0)).max(1e-9);
                        let lean = (take / film_oil).min(0.9);
                        hd[4] *= 1.0 - lean;
                        hd[1] = (hd[1] * (1.0 + lean) * (1.0 + lean)).min(1.0);
                        *vol = (t - take).max(0.0);
                    }
                }
            }
        }
    }

    /// Lift `v` coats off pixel `i`. The paint left keeps its cure; a pixel
    /// left bare (as `wait` judges it) holds no film to have one.
    #[inline]
    fn take(&self, i: usize, v: f32) {
        unsafe {
            self.check(i);
            let vol = &mut *self.vol.add(i);
            *vol -= v;
            if self.cure_now && *vol < CURE_BARE_COATS && !self.dry.is_null() {
                (*self.dry.add(i)).cure = 0.0;
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn lay_droplet(&self, i: usize, v: f32, phi: f32, cap: f32, capped: bool, id: u32, lat: &Latent, hide: Prop, cure: f32) -> f32 {
        self.check(i);
        let mut d = v.max(0.0);
        // SAFETY: check(i) enforces this tile's footprint, and the scheduler
        // never runs overlapping tile footprints concurrently.
        unsafe {
            if capped {
                assert!(self.has_ceiling(), "film ceiling unavailable");
                let used = if *self.laid_id.add(i) == id { *self.laid.add(i) } else { 0.0 };
                d = d.min((cap - used).max(0.0));
                *self.laid_id.add(i) = id;
                *self.laid.add(i) = used + d;
            }
            if d > 0.0 {
                *self.cover.add(i) = 1.0;
                self.add(i, d * (1.0 - phi), lat, hide, cure);
                if phi > 0.0 && !self.solv.is_null() {
                    *self.solv.add(i) += d * phi * COAT_UM;
                }
                *self.stroke.add(i) = id;
            }
        }
        d
    }
}

impl Wet {
    pub(crate) fn ensure_drying(&mut self) {
        if self.clock.px.len() != self.vol.len() {
            self.clock.px = vec![crate::drying::Px::FRESH; self.vol.len()];
        }
    }
    #[inline]
    pub(crate) fn counts_for_drying(v: f32) -> bool {
        v >= DRYING_BARE_COATS
    }
    #[inline]
    pub(crate) fn bare_for_drying(v: f32) -> bool {
        v < DRYING_BARE_COATS
    }
    pub(crate) fn reported_cure(&self, i: usize) -> f32 {
        if self.vol[i] < REPORTED_CURE_BARE_COATS {
            return 0.0;
        }
        self.drying(i).map_or(0.0, |p| p.cure)
    }
    /// Whether a newly worked film needs its neighborhood thickness judged.
    pub(crate) fn needs_drying_thickness(&self, rect: crate::sched::Rect, w: usize, cure_now: bool) -> bool {
        use rayon::prelude::*;
        let (x0, y0, x1, y1) = rect;
        let mark = self.clock.mark;
        // a worked film's thickness is judged afresh over the paint that's
        // wet around it now; an untouched one keeps its own (see `Px::th`)
        let now = cure_now;
        let worked = |wet: &Wet, i: usize| wet.touched[i] > mark || wet.stroke[i] > mark || (now && wet.clock.px[i].seen != wet.vol[i]);
        let fresh = |wet: &Wet, i: usize| worked(wet, i) || wet.clock.px[i].th <= 0.0;
        let any = (y0..y1).into_par_iter().any(|y| (x0..x1).any(|x| self.vol[y * w + x] >= DRYING_BARE_COATS && fresh(self, y * w + x)));
        any
    }
    /// Record the film as worked before aging it. Engine 1 dilutes cure
    /// here; engine 2 already mixed cure when paint was laid. Flow preserves
    /// the drying state of positive films below the usual bare cutoff.
    pub(crate) fn observe_worked_films(&mut self, rect: crate::sched::Rect, w: usize, now: bool, flow: bool, th: &[f32]) {
        use rayon::prelude::*;
        let (x0, y0, x1, y1) = rect;
        let mark = self.clock.mark;
        let bw = x1 - x0;
        let wet = self;
        let (vol, touched, stroke) = (&wet.vol, &wet.touched, &wet.stroke);
        wet.clock.px[y0 * w..y1 * w].par_chunks_mut(w).enumerate().for_each(|(j, row)| {
            for x in x0..x1 {
                let i = (y0 + j) * w + x;
                let v = vol[i];
                let p = &mut row[x];
                if v < DRYING_BARE_COATS {
                    if !(flow && v > 0.0) {
                        (p.cure, p.lev, p.seen, p.th) = (0.0, crate::surface::SET_TIME, 0.0, 0.0);
                    }
                    continue;
                }
                let worked = touched[i] > mark || stroke[i] > mark || (now && p.seen != v);
                if worked {
                    if !now {
                        p.cure *= p.seen.min(v) / v;
                    }
                    p.lev = crate::surface::SET_TIME * crate::drying::fluid(p.cure);
                }
                if worked || p.th <= 0.0 {
                    p.th = th[j * bw + x - x0];
                }
                p.seen = v;
            }
        });
        wet.clock.mark = wet.current;
    }
    /// A film baked into the dry picture holds no solvent any more (by then
    /// it has long gone; `dry` takes the rest with it).
    pub(crate) fn clear_baked_solvent(&mut self, ex: crate::sched::Rect, w: usize) {
        if self.solv.is_empty() {
            return;
        }
        for y in ex.1..ex.3 {
            for x in ex.0..ex.2 {
                let i = y * w + x;
                if self.vol[i] == 0.0 {
                    self.solv[i] = 0.0;
                }
            }
        }
    }

    /// Rag lift resets bare cure only for engines with cure-on-lift. The
    /// solvent in the film comes away with the paint, in their proportions
    /// there; returns how much (coats; the canvas holds it in µm).
    #[inline]
    pub(crate) fn lift_rag(&mut self, i: usize, take: f32, cure_on_lift: bool) -> f32 {
        let v0 = self.vol[i];
        let mut ts = 0.0;
        if let Some(s) = self.solv.get_mut(i)
            && *s > 0.0
            && v0 > 0.0
        {
            let tu = (*s * take / v0).min(*s);
            *s -= tu;
            ts = tu / COAT_UM;
        }
        let v = &mut self.vol[i];
        *v -= take;
        if cure_on_lift && *v < CURE_BARE_COATS && self.clock.px.len() == self.vol.len() {
            self.clock.px[i].cure = 0.0;
        }
        ts
    }

    /// Rag smear: no brush lean-on-lay or ground absorption, and cure mixes
    /// whenever drying state exists (not the brush's engine-2 gate). Solvent
    /// arrives already in µm; only a positive amount is added.
    pub(crate) fn smear_rag(&mut self, i: usize, v: f32, lat: &Latent, hide: Prop, cure: f32, solv_um: f32) {
        let (t, a) = mix_material(self.vol[i], &mut self.lat[i], &mut self.hide[i], v, lat, hide);
        if self.clock.px.len() == self.vol.len() {
            let p = &mut self.clock.px[i];
            p.cure = if t < CURE_BARE_COATS { 0.0 } else { p.cure + (cure - p.cure) * a };
        }
        self.cover[i] = 1.0;
        self.vol[i] = t;
        if solv_um > 0.0
            && let Some(sv) = self.solv.get_mut(i)
        {
            *sv += solv_um;
        }
    }

    /// Write one solvent-flow result. Flow is not a brush working the film:
    /// keep existing drying thickness and inherit it only on a bare receiver.
    /// Equality at FLOW_BARE_COATS is wet; this is not the hair-cover cutoff.
    pub(crate) fn move_flow(&mut self, i: usize, pv: f32, ps: f32, lat: Latent, hide: Prop, cure: f32, th_in: f32) -> bool {
        let was_bare = self.vol[i] < FLOW_BARE_COATS;
        self.vol[i] = pv;
        let new_solvent = ps > 0.0 && self.solv[i] <= 0.0;
        self.solv[i] = ps * COAT_UM;
        self.lat[i] = lat;
        self.hide[i] = hide;
        if was_bare && pv >= FLOW_BARE_COATS {
            self.cover[i] = 1.0;
        }
        if self.clock.px.len() == self.vol.len() {
            let p = &mut self.clock.px[i];
            p.cure = cure;
            // the flow isn't a brush working the film: the film keeps
            // its own drying. A bare pixel it reaches takes the
            // drying thickness of the films it came from (when they
            // have one; else it is judged at the next drying step,
            // as a bare pixel's always was), so its drying doesn't
            // depend on when that step falls (`wait_on_grid`)
            if !was_bare {
                p.seen = pv;
            } else if th_in > 0.0 {
                p.th = th_in;
                if pv >= FLOW_BARE_COATS {
                    (p.seen, p.lev) = (pv, crate::surface::SET_TIME * crate::drying::fluid(cure));
                }
            }
        }
        new_solvent
    }
}

/// Only material arithmetic is shared. Callers retain the order in which
/// cure, volume, cover and their different solvent policies are recorded.
#[inline]
fn mix_material(rv: f32, rl: &mut Latent, rh: &mut Prop, v: f32, lat: &Latent, hide: Prop) -> (f32, f32) {
    let t = rv + v;
    let a = v / t;
    for k in 0..LAT {
        rl[k] += (lat[k] - rl[k]) * a;
    }
    lerp_prop(rh, hide, a);
    (t, a)
}

/// Mix `v` of (`lat`, `hide`) into a reservoir (`rv`, `rl`, `rh`).
#[inline]
pub fn mix_into(rv: &mut f32, rl: &mut Latent, rh: &mut Prop, v: f32, lat: &Latent, hide: Prop) {
    if v <= 0.0 {
        return;
    }
    let (t, _) = mix_material(*rv, rl, rh, v, lat, hide);
    *rv = t;
}

/// `coats` of `pig` over `under`, laid over the share `cover` of a pixel:
/// the paint sits there `coats / cover` thick and the rest of the pixel
/// shows `under` (area-weighted in linear light, as the eye averages it).
/// However small the share, it weighs what it covers: a hair's edge over a
/// thousandth of a pixel darkens it by a thousandth. (The paint's thickness
/// there is capped far beyond hiding, so a vanishing share stays finite.)
pub(crate) fn over_share(pig: Pigment, under: Rgb, coats: f32, cover: f32) -> Rgb {
    if cover >= 1.0 {
        return pig.over(under, coats);
    }
    if cover.is_nan() || cover <= 0.0 {
        return under;
    }
    let c = cover;
    let o = pig.over(under, (coats / c).min(1e4));
    [under[0] + (o[0] - under[0]) * c, under[1] + (o[1] - under[1]) * c, under[2] + (o[2] - under[2]) * c]
}

/// Paste stands at most this tall for its width (a bead of oil paint: stroke
/// ridges of 0.1-0.3 mm on strokes 1-2 mm wide are 0.1-0.3; 0.5 is stiff
/// impasto).
const BEAD_ASPECT: f32 = 0.5;

/// The share of a pixel (`px_um` wide) that `coats` of wet paint in it
/// cover: at least `cover` (what the hairs touched), and at least enough
/// that the paint on it stands no taller than `BEAD_ASPECT` times the
/// pixel's width. A sliver of a pixel can't hold a bead taller than that:
/// the paint has slumped across it (c ≥ coats·COAT_UM / (BEAD_ASPECT·px_um)).
/// Linear in the paint, so a hairline's fringe keeps its share wherever the
/// line falls between pixel centers; at 1000px (pixels of 0.2-0.9 mm) it
/// takes a film over 100 µm on the sliver to act. At 3200 (0.1 mm pixels)
/// it spreads the tens of µm that leveling pours into a pixel a hair only
/// grazed over the pixel, instead of leaving them on a sliver with the rest
/// of the pixel bare.
pub(crate) fn bead_cover(cover: f32, coats: f32, px_um: f32) -> f32 {
    if cover >= 1.0 || coats.is_nan() || coats <= 0.0 || px_um.is_nan() || px_um <= 0.0 {
        return cover;
    }
    let need = coats * crate::surface::COAT_UM / (BEAD_ASPECT * px_um);
    cover.max(need).min(1.0)
}

impl Canvas {
    /// The visible color at pixel `i`: the dry picture with any wet paint
    /// on it (at its laid thickness, before it levels).
    pub(crate) fn look_px(&self, i: usize) -> Rgb {
        let v = self.wet.vol[i];
        if v < VISIBLE_BARE_COATS {
            return self.px[i];
        }
        let c = mixbox::latent_to_linear_float_rgb(&self.wet.lat[i]);
        over_share(Pigment::masstone(c, self.wet.hide[i][0]), self.px[i], v, bead_cover(self.wet.cover[i], v, self.px_mm() * 1000.0))
    }

    /// The visible colors, pixel by pixel over the window: the dry
    /// picture with any wet paint on it (at its laid thickness).
    pub fn seen(&self) -> Vec<Rgb> {
        use rayon::prelude::*;
        if self.engine >= 4 {
            // engine 4: a matte surface veils its colors (wet paint is oily, glossy)
            return (0..self.px.len())
                .into_par_iter()
                .map(|i| {
                    // (the thinnest wet film lets the surface under it show: no edge where it ends)
                    let v = self.wet.vol[i];
                    let cover = bead_cover(self.wet.cover[i], v, self.px_mm() * 1000.0);
                    let gloss_share = if cover > 0.0 { cover * crate::smoothstep(0.0, 0.1, v / cover) } else { 0.0 };
                    let g = crate::lerp(self.gloss[i], 1.0, gloss_share);
                    crate::canvas::haze(self.look_px(i), g)
                })
                .collect();
        }
        (0..self.px.len()).into_par_iter().map(|i| self.look_px(i)).collect()
    }

    /// What is on the canvas around (`x`, `y`) within radius `r` (units),
    /// dry picture plus wet paint, averaged in linear light: the underlayer a
    /// new mark there will sit on.
    pub fn under(&self, x: f32, y: f32, r: f32) -> Rgb {
        let f = self.f;
        let n = if r * f.scale < 1.5 { 0 } else { 2 };
        let (mut acc, mut k) = ([0.0f32; 3], 0.0f32);
        for j in -n..=n {
            for i in -n..=n {
                let (dx, dy) = (i as f32 / 2.0 * r, j as f32 / 2.0 * r);
                if n > 0 && dx * dx + dy * dy > r * r * 1.01 {
                    continue;
                }
                let (px, py) = (x + dx, y + dy);
                // (a crop render holds only its window: look where it can)
                if px < 0.0 || py < 0.0 || px >= f.width() || py >= f.height() || !f.holds(px, py) {
                    continue;
                }
                let c = self.look_px(f.index(px, py));
                for q in 0..3 {
                    acc[q] += c[q];
                }
                k += 1.0;
            }
        }
        if k == 0.0 {
            return self.look_px(f.index(x, y));
        }
        [acc[0] / k, acc[1] / k, acc[2] / k]
    }

    /// Wet paint film at a point (units), µm: 0 where the paint is dry.
    pub fn wet_um(&self, x: f32, y: f32) -> f32 {
        self.wet.vol[self.f.index(x, y)] * crate::surface::COAT_UM
    }

    /// Total wet paint on the canvas (for tests / debugging).
    pub fn wet_total(&self) -> f64 {
        self.wet.vol.iter().map(|&v| v as f64).sum::<f64>() / (self.f.scale as f64 * self.f.scale as f64)
    }

    /// Solvent in the open film at a point (units), µm: 0 where there is
    /// none (`crate::thinner`). `wet_um` is the paint alone.
    pub fn solvent_um(&self, x: f32, y: f32) -> f32 {
        self.wet.solv.get(self.f.index(x, y)).copied().unwrap_or(0.0)
    }

    /// Total solvent on the canvas, in `wet_total`'s units (coats × square
    /// units).
    pub fn solvent_total(&self) -> f64 {
        self.wet.solv.iter().map(|&v| v as f64).sum::<f64>() / crate::surface::COAT_UM as f64 / (self.f.scale as f64 * self.f.scale as f64)
    }
}

#[cfg(test)]
mod tests {

    /// The safe view must reject every pixel outside its scheduled footprint,
    /// including holes inside the cached row span and cropped footprints.
    #[test]
    fn a_tile_rejects_pixels_outside_its_scheduled_footprint() {
        use super::Wet;
        use crate::canvas::Frame;
        use std::panic::{AssertUnwindSafe, catch_unwind};
        for (ox, oy, rect) in [(0, 0, (1, 1, 3, 3)), (5, 7, (7, 8, 11, 10)), (0, 0, (0, 1, 4, 3))] {
            let f = Frame { w: 4, h: 4, scale: 1.0, x0: ox, y0: oy, full_w: 16, full_h: 16 };
            let mut wet = Wet::new(16);
            let (base, height, mut absorb) = ([0.0; 16], [0.0; 16], [0.0; 16]);
            wet.tile(f, 2, &base, &height, &mut absorb, false).paint_tiles(&[0], &[Some(rect)], |tile, _| {
                for i in 0..=16 {
                    let (x, y) = (i % 4 + ox, i / 4 + oy);
                    let inside = i < 16 && x >= rect.0 && y >= rect.1 && x < rect.2 && y < rect.3;
                    let read = catch_unwind(AssertUnwindSafe(|| tile.volume(i)));
                    assert_eq!(read.is_ok(), inside, "pixel {i} in footprint {rect:?}");
                    if inside { assert_eq!(read.unwrap(), 0.0); }
                }
            });
        }
    }


    #[test]
    fn a_partial_wet_mark_keeps_the_uncovered_surface_matte() {
        let mut c = crate::Canvas::new(32, 1.0, [0.1; 3]).with_engine(4);
        c.gloss.fill(0.0);
        let matte = c.seen()[0][0];
        c.wet.lat[0] = mixbox::linear_float_rgb_to_latent(&[0.1; 3]);
        c.wet.vol[0] = 0.1;
        c.wet.cover[0] = 1.0;
        let full = c.seen()[0][0];
        c.wet.cover[0] = 0.01;
        let partial = c.seen()[0][0];
        assert!((partial - matte).abs() < (full - matte).abs() * 0.05,
            "1% wet coverage cannot remove the matte veil from the whole pixel: {matte}, {partial}, {full}");
    }

    /// Setting a paint's hiding changes its scattering and nothing else: a
    /// slow-drying paint stays slow whichever builder comes first.
    #[test]
    fn with_hiding_keeps_the_other_fields() {
        let a = super::Paint::body([0.3; 3]).with_stiff(0.4).with_drying(0.3).with_hiding(0.5);
        let b = super::Paint::body([0.3; 3]).with_stiff(0.4).with_hiding(0.5).with_drying(0.3);
        assert_eq!((a.drying, a.stiff, a.color), (0.3, 0.4, [0.3; 3]));
        assert_eq!((a.drying, a.stiff, a.color, a.scatter), (b.drying, b.stiff, b.color, b.scatter));
        assert!((a.hiding() - 0.5).abs() < 0.01);
    }

    /// A hairline's fringe keeps its share (the floor is linear in the paint
    /// and far below a hair's film at 1000px); a sliver holding a bead many
    /// times taller than the pixel is wide spreads over the pixel.
    #[test]
    fn bead_cover_floors_only_impossible_beads() {
        // 0.02 coats on 5% of a 0.3 mm pixel: a 10 µm film, left alone
        assert_eq!(super::bead_cover(0.05, 0.02, 300.0), 0.05);
        // linear: twice the paint on twice the share is the same film
        assert_eq!(super::bead_cover(0.10, 0.04, 300.0), 0.10);
        // 4 coats (100 µm) on 10% of a 0.1 mm pixel would stand 1 mm tall
        assert_eq!(super::bead_cover(0.10, 4.0, 100.0), 1.0);
        // 1 coat on 10% of a 0.1 mm pixel: at least half of it
        assert!((super::bead_cover(0.10, 1.0, 100.0) - 0.5).abs() < 1e-6);
        assert_eq!(super::bead_cover(1.0, 9.0, 100.0), 1.0);
        assert_eq!(super::bead_cover(0.3, 0.0, 100.0), 0.3);
    }

    /// Paint over a sliver of a pixel darkens it by that sliver's share, not
    /// by a 1% floor; no share shows nothing, and a vanishing share of thick
    /// paint stays finite.
    #[test]
    fn sub_percent_cover_keeps_its_area() {
        let pig = super::Paint::body([0.01; 3]).pigment();
        let under = [1.0; 3];
        for cover in [0.001f32, 0.005, 0.02, 0.5] {
            let thick = pig.over(under, 0.1 / cover);
            let want: [f32; 3] = std::array::from_fn(|i| under[i] + (thick[i] - under[i]) * cover);
            let got = super::over_share(pig, under, 0.1, cover);
            assert!((0..3).all(|i| (got[i] - want[i]).abs() < 1e-6), "cover {cover}: {got:?} vs {want:?}");
        }
        assert_eq!(super::over_share(pig, under, 0.1, 0.0), under);
        let tiny = super::over_share(pig, under, 5.0, 1e-30);
        assert!(tiny.iter().all(|v| v.is_finite() && (v - 1.0).abs() < 1e-6), "{tiny:?}");
    }
}
