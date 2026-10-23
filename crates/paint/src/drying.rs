//! Time and drying: the painting has a clock, and wet paint ages.
//!
//! Oil paint dries by oxidation: the oil takes up oxygen, cross-links and
//! turns from a liquid into a gel into a solid film. The film passes four
//! stages:
//!
//! - **open**: fully workable. Blends wet into wet, brushes lift and push it,
//!   it levels. It stiffens slowly as it ages.
//! - **setting**: close to the gel point. Stiff and sticky: a brush lifts
//!   little and pushes less, fresh marks no longer level.
//! - **tacky**: past the gel point the film is a sticky solid. It doesn't flow,
//!   mix or come up on the brush, but it grabs: a brush dragged over it
//!   leaves its paint quickly and in broken patches (stick and slip).
//! - **touch-dry**: a skin. New paint sits on top without mixing, as it does
//!   on dry paint. (Through-drying under the skin takes weeks to months and
//!   isn't modeled.)
//!
//! Each pixel's open film carries its own oxidation progress `cure`
//! (0 = fresh, 1 = touch-dry). Its rate follows from the paint laid there:
//! pigment (`Paint::drying`: lead white and umber are driers, bone black and
//! lakes slow), thickness (thick films dry slower) and oil content (fat,
//! medium-rich paint dries slower than lean). Fresh paint worked into an older
//! film dilutes its cure by volume as it is laid (engine 1: at the next
//! `wait`). How much of its time to touch-dry a film spends open before its
//! gel point, and how long it takes to touch-dry, depend on the engine
//! version (`Pace`: 15% of a day for one coat of average paint in engines 1
//! and 2, 60% of two and a half days in engine 3). When the film reaches the gel point it levels for as long as it
//! was fluid, then it bakes into the dry picture: from then on it is part of
//! the surface and its tack lives on in `sub` until it is touch-dry. So a pixel can hold new wet paint over a set layer.
//!
//! `Canvas::wait(minutes)` advances the clock. `Canvas::dry()` waits until all
//! paint is touch-dry. The per-pixel drying state is allocated on the first
//! `wait`; a painting that never waits doesn't allocate it.

use crate::canvas::Canvas;
use crate::surface::{COAT_UM, SET_TIME};
use crate::{smoothstep, surface::vnoise};
use rayon::prelude::*;

thread_local! {
    static FILM_SCRATCH: std::cell::RefCell<[Vec<f32>; 5]> = std::cell::RefCell::new(Default::default());
    static BAKE_SCRATCH: std::cell::RefCell<[Vec<f32>; 8]> = std::cell::RefCell::new(Default::default());
}

/// Minutes to touch-dry for one lean 25 µm coat of average paint
/// (`drying` 1) in engines 1 and 2 (engine 3: `TOUCH_DRY_MIN_3`). Estimate
/// from thin-film touch-dry times: 1–2 days (umber, lead white) to 2–5 days
/// (blacks) and 7–14 (alizarin).
pub const TOUCH_DRY_MIN: f32 = 24.0 * 60.0;
/// The same in engine 3: two and a half days, so that a typical brushstroke
/// (`STROKE`) of lead white tube paint (`drier::LEAD_WHITE`, stiffness 0.8)
/// is touch-dry in 45 h, in the 1–2 days painters and makers give for lead
/// white in linseed oil without driers (Winsor & Newton and Natural
/// Pigments: fast, about 2 days; Golden: fast, 1–2 days). Each tube's
/// source range is checked at `STROKE` too, and a tube takes an engine-3
/// rate where its engine-2 rate misses it (`drier::engine3`). Titanium
/// white (in no box) at 0.9, "average to slow" (Natural Pigments), takes
/// 4.4 days (artists' guides: 3–5). Golden measured a titanium white without
/// driers touch-dry by day 2 at 3 mil and at 4–6 days at 10 mil: their ratio
/// checks `THICK`, but the days themselves are faster than these: such a
/// paint here would be 16 days at 10 mil. That conflict is left standing.
pub const TOUCH_DRY_MIN_3: f32 = 60.0 * 60.0;
/// The thickness (coats) of a typical brushstroke, at which the painters'
/// and makers' drying times are taken: a broad brush (filbert 40, flat 36)
/// loaded 0.9 lays 1.2–1.9 coats (median film thickness of one stroke).
pub const STROKE: f32 = 1.5;
/// Cure (oxidation) at the gel point: the film stops flowing and becomes
/// tacky. When a film reaches it depends on the engine (`Pace`).
pub const GEL: f32 = 0.15;
/// The share of its time to touch-dry a film spends before its gel point
/// (engine 3; see `Pace`). It is open (lifts cleanly) for the first half of
/// that. Drying oils take up oxygen only after an induction period, then
/// fast (Tumosa and Mecklenburg 2005, "The influence of lead ions on the
/// drying of oils"); a film begins to skin over as its oxygen uptake nears
/// its peak, and is touch-dry at the peak (Golden, "Weighing In on the
/// Drying of Oils"). Painters work into lead white 12–24 h after laying
/// it, and it is tacky and skinning after about a day: with
/// `TOUCH_DRY_MIN_3`, 0.6 keeps a stroke (`STROKE`) open for 13 h and gels
/// it at 27 h.
pub const OPEN_SHARE: f32 = 0.6;
/// How much a film's thickness slows its drying: time ∝ (h / 1 coat)^THICK.
/// Golden's titanium white was touch-dry by day 2 at 3 mil and at 4–6 days at
/// 10 mil (above): an exponent of 0.58–0.91. Surface skinning is
/// reaction-limited in thin films and increasingly oxygen-limited in thick
/// ones.
const THICK: f32 = 0.7;
/// How much a fat, medium-rich paint (stiff 0) dries slower than stiff tube
/// paint (stiff 1) (estimate).
const FAT: f32 = 0.6;
/// How much faster a tacky surface pulls paint off a brush (estimate).
const GRAB: f32 = 2.0;
/// Spread (sd, mm) of the patch over which a film's thickness sets its
/// drying rate. A film skins over as a whole: the bristle ridges and
/// furrows of a brushstroke (a fraction of a mm) don't dry on their own
/// clocks, a stroke and its neighbors do. It also keeps the rate the same at
/// any resolution: a coarse pixel averages thin and thick paint, a fine one
/// sees them apart; both are judged over the same few millimeters
/// (estimate).
pub const FILM_MM: f32 = 1.25;

/// Relative drying rates of pigments ground in oil (1 = average;
/// higher dries faster). From Mayer's comparative list (fast: lead white,
/// umbers, chrome yellow, Prussian blue; medium: earths, cobalt; slow to very
/// slow: vermilion, ivory/lamp/vine black, madder, alizarin) and touch-dry
/// ranges for thin films. Cobalt glass (smalt) was
/// itself used as a drier.
pub mod drier {
    pub const LEAD_WHITE: f32 = 2.0;
    pub const UMBER: f32 = 2.4;
    pub const CHROME_YELLOW: f32 = 1.8;
    pub const PRUSSIAN_BLUE: f32 = 1.8;
    pub const SMALT: f32 = 1.6;
    pub const COBALT_BLUE: f32 = 1.4;
    pub const SIENNA: f32 = 1.2;
    pub const RED_EARTH: f32 = 1.0;
    pub const OCHRE: f32 = 0.8;
    pub const ULTRAMARINE: f32 = 0.8;
    pub const VERMILION: f32 = 0.4;
    pub const BONE_BLACK: f32 = 0.4;
    pub const LAMP_BLACK: f32 = 0.35;
    pub const MADDER_LAKE: f32 = 0.3;
    pub const ZINC_WHITE: f32 = 0.35;

    // Round 20's tubes (notes/r20/TUBES.md). Estimates from the tube
    // proposals' sources, not measurements.

    /// Red lead "exerts a powerful action on drying oils" (Artists'
    /// Pigments vol. 1 p.114): a little above lead white.
    pub const RED_LEAD: f32 = 2.2;
    /// Naples yellow (lead antimonate): lead compounds promote drying; set
    /// between average and lead white. Unsourced estimate.
    pub const NAPLES_YELLOW: f32 = 1.6;
    /// Antwerp blue: Prussian blue on an inert white base, which dilutes its
    /// fast drying a little. Estimate.
    pub const ANTWERP_BLUE: f32 = 1.6;
    /// Copper pigments promote the drying of oil (Artists' Pigments vol. 2
    /// p.136): emerald green, at copper green's rate.
    pub const COPPER: f32 = 1.6;
    /// Mars (synthetic iron oxide) colors are "good driers for oil paints"
    /// (MFA CAMEO, "Mars colors"): a little above the natural red earth.
    pub const MARS: f32 = 1.1;
    /// Viridian: no drier action reported; average. Estimate.
    pub const VIRIDIAN: f32 = 1.0;
    /// Strontium and barium chromates hold no lead, so none of lead
    /// chromate's drier action, and are inert, nearly insoluble salts:
    /// average. Estimate (materials research,
    /// notes/research/impressionist_materials.md).
    pub const CHROMATE: f32 = 1.0;
    /// Zinc yellow (potassium zinc chromate) holds no lead either, and it
    /// brings zinc into the film, which slows an oil's drying (see
    /// `ZINC_WHITE`): a little below average. Estimate; no measured rate found.
    pub const ZINC_YELLOW: f32 = 0.8;
    /// Indian yellow: an early account has it drying in oil "nearly as soon
    /// or sooner than" other colors (Artists' Pigments vol. 1 p.24); set near
    /// average. Uncertain.
    pub const INDIAN_YELLOW: f32 = 0.8;
    /// Bone brown (bone roasted short of black): "bad driers in oil"
    /// (Field's Chromatography, revised by Salter, 1869, §243): below bone
    /// black. Estimate.
    pub const BONE_BROWN: f32 = 0.3;
    /// Cadmium pigments "do not retard drying … and may be regarded as slow
    /// but reliable driers" (Artists' Pigments vol. 1 p.72): slower than the
    /// earths, faster than vermilion.
    pub const CADMIUM: f32 = 0.6;
    /// Bitumen (asphaltum) slows the drying of linseed oil and never fully
    /// cures (MFA CAMEO, "Asphaltum"): the slowest tube, below madder lake.
    pub const BITUMEN: f32 = 0.15;

    /// Engine 3's drying rates for the tubes whose own touch-dry range
    /// called for another rate than their `Tube::drying` (which engines 1
    /// and 2 keep); a tube carries its own as `Tube::drying_3`. A typical
    /// stroke (`super::STROKE`) of each is touch-dry inside its range in
    /// engine 3 (`super::TOUCH_DRY_MIN_3`): Winsor & Newton's Artists' Oil
    /// Colour classes (fast, about 2 days: cobalt blues, Prussian blue, raw
    /// sienna, umbers, lead whites; medium, about 5: cadmiums, ultramarines,
    /// ochres, burnt sienna, Mars colors, ivory and lamp black; slow, over 5:
    /// alizarin, quinacridones), Natural Pigments' (fast about 2 days, medium
    /// 2–5, slow over 5; its bone black medium), Golden's (fast 1–2 days) and
    /// alizarin's 7–14 days. Tubes without one have no source range or are
    /// inside theirs already.
    pub mod engine3 {
        /// Bone black: medium, 2–5 days: 4.4 days (engine 2's 0.4: 9.8).
        pub const BONE_BLACK: f32 = 0.9;
        /// Cobalt blue: fast, about 2 days: 45 h (1.4: 71 h).
        pub const COBALT_BLUE: f32 = 2.2;
        /// Prussian blue: fast, about 2 days: 44 h (1.8: 59 h).
        pub const PRUSSIAN_BLUE: f32 = 2.4;
        /// Burnt sienna: dries better than raw sienna, as roasting improves
        /// sienna's drying (Field, Chromatography, rev. Salter 1869, §§50 and
        /// 155, https://www.gutenberg.org/files/20915/20915-h/20915-h.htm):
        /// fast, about 2 days: 45 h (1.2: 86 h). Raw sienna keeps 1.2 (86 h,
        /// medium). Modern makers (W&N) class raw sienna fast and burnt
        /// medium; the box follows the historical order.
        pub const BURNT_SIENNA: f32 = 2.3;
        /// The cadmiums: medium, 2–5 days: 4.4–4.6 days (0.6: 6.5–6.9).
        pub const CADMIUM: f32 = 0.9;
        /// The ultramarines: medium, 2–5 days: 4.5 days (0.8: 5.4).
        pub const ULTRAMARINE: f32 = 0.95;
        /// Rose madder and permanent alizarin (a quinacridone): alizarin,
        /// 7–14 days, and slow, over 5: 11–11.5 days (0.3: 14.7–15.4).
        pub const ALIZARIN: f32 = 0.4;
    }
}

/// How fast a film's cure advances before and after its gel point, as a
/// multiple of its `rate`: the one place drying differs by engine version
/// (`Canvas::engine`, `crate::ENGINE`).
/// - engines 1 and 2: cure grows evenly, so a film gels after `GEL` (15%)
///   of its time to touch-dry (`TOUCH_DRY_MIN`): one coat of lead white tube
///   paint after 2 h, touch-dry after 13 h.
/// - engine 3: a film takes 2.5 times as long to touch-dry
///   (`TOUCH_DRY_MIN_3`), spends `OPEN_SHARE` of it before its gel point and
///   the rest tacky: a stroke of lead white (`STROKE`) is open for 13 h,
///   gels after 27 h and is touch-dry after 45 h.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Pace {
    /// Before the gel point.
    open: f32,
    /// After it.
    set: f32,
}

impl Pace {
    const EVEN: Pace = Pace { open: 1.0, set: 1.0 };

    pub(crate) fn of(engine: u32) -> Pace {
        if engine >= 3 {
            let k = TOUCH_DRY_MIN / TOUCH_DRY_MIN_3;
            Pace { open: k * GEL / OPEN_SHARE, set: k * (1.0 - GEL) / (1.0 - OPEN_SHARE) }
        } else {
            Pace::EVEN
        }
    }

    /// The cure of an open film of cure `c` and rate `r` after `dt` minutes.
    fn age(self, c: f32, dt: f32, r: f32) -> f32 {
        if self == Pace::EVEN {
            return c + dt * r;
        }
        if c >= GEL {
            return c + dt * r * self.set;
        }
        let to_gel = (GEL - c) / (r * self.open);
        if dt <= to_gel { c + dt * r * self.open } else { GEL + (dt - to_gel) * r * self.set }
    }

    /// Minutes until an open film of cure `c` and rate `r` is touch-dry.
    fn to_dry(self, c: f32, r: f32) -> f32 {
        if self == Pace::EVEN || c >= GEL {
            return (1.0 - c).max(0.0) / (r * self.set);
        }
        (GEL - c) / (r * self.open) + (1.0 - GEL) / (r * self.set)
    }

    /// Cure per minute of a set film whose open film's rate was `r`.
    fn set_rate(self, r: f32) -> f32 {
        if self == Pace::EVEN { r } else { r * self.set }
    }
}

/// How lean a film whose properties are `hd` dries, as the stiffness `rate`
/// takes: its own, and with the drain switch (`drain`) its packed share as
/// lean as stiff paste (packed on an absorbent ground, it holds only the
/// oil its pigment packs with: `bristle::ground_drain`).
#[inline]
pub(crate) fn lean_of(drain: bool, hd: &crate::wet::Prop) -> f32 {
    if drain { hd[1] + (1.0 - hd[1].clamp(0.0, 1.0)) * hd[7].clamp(0.0, 1.0) } else { hd[1] }
}

/// Cure gained per minute by an open film `vol` coats thick, of stiffness
/// `stiff` and pigment drying rate `drying` (before `Pace`).
pub(crate) fn rate(vol: f32, stiff: f32, drying: f32) -> f32 {
    let thick = vol.max(0.0).powf(THICK).max(0.5);
    let fat = 1.0 + FAT * (1.0 - stiff.clamp(0.0, 1.0));
    drying.max(0.01) / (TOUCH_DRY_MIN * thick * fat)
}

/// How fluid an open film of cure `c` still is (1 fresh → 0 at the gel
/// point): viscosity rises slowly at first and diverges at the gel point.
#[inline]
pub(crate) fn fluid(c: f32) -> f32 {
    1.0 - smoothstep(0.0, GEL, c)
}

/// Tack of a set film of cure `s`: strongest at the gel point, gone when
/// touch-dry.
#[inline]
fn set_tack(s: f32) -> f32 {
    if s >= 1.0 { 0.0 } else { 1.0 - smoothstep(GEL, 1.0, s) }
}

/// What a brush feels at a pixel: (fluidity of the open paint, tack of the
/// surface), from the open film's volume and cure and the set film's cure.
#[inline]
pub(crate) fn feel(vol: f32, cure: f32, sub: f32) -> (f32, f32) {
    let open_tack = if vol > 1e-3 { smoothstep(0.5 * GEL, GEL, cure) } else { 0.0 };
    (fluid(cure), open_tack.max(set_tack(sub)))
}

/// Stick and slip: over a tacky surface a bristle catches and lets go, so
/// its paint comes off in patches. A deposit factor with mean ~1 at bristle
/// position (`x`, `y`, pixels), bristle size `rb` and tack `tack`.
#[inline]
pub(crate) fn stick(x: f32, y: f32, rb: f32, seed: u64, tack: f32) -> f32 {
    if tack <= 0.0 {
        return 1.0;
    }
    let l = 3.0 * rb.max(1.0);
    let n = vnoise(x / l, y / l, seed ^ 0x5717_c1c5);
    (1.0 + tack * (2.2 * smoothstep(0.3, 0.7, n) - 1.1)).max(0.0)
}

/// How much faster a surface of tack `tack` empties a brush.
#[inline]
pub(crate) fn grab(tack: f32) -> f32 {
    1.0 + GRAB * tack
}

/// Where a pixel's paint is in drying.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// Workable wet paint: blends, lifts, levels.
    Open,
    /// Wet paint near its gel point: stiff and sticky, barely blends.
    Setting,
    /// A set film: doesn't move, grabs the brush.
    Tacky,
    /// Touch-dry (or bare ground): new paint sits on top.
    Dry,
}

/// Per-pixel drying state (allocated on the first `wait`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Px {
    /// Oxidation of the open (wet) film: 0 fresh, `GEL` sets, 1 touch-dry.
    pub cure: f32,
    /// Seconds the open film levels after it is worked, at its fluidity
    /// then. Engine 9 consumes this once while the film is still wet;
    /// older engines defer it until baking.
    pub lev: f32,
    /// Volume of the open film at the last `wait`. A film whose volume
    /// changed since was worked, even where no bristle touched it (paint
    /// ploughed into it); in engine 1, fresh paint since then dilutes the
    /// cure at the next wait.
    pub seen: f32,
    /// Cure of the top set film, baked into the dry picture (≥ 1: dry).
    pub sub: f32,
    /// Its cure per minute.
    pub srate: f32,
    /// Thickness (coats) of the open film's neighborhood, judged when the
    /// film was last worked (see `Canvas::film_thickness`). It stays fixed
    /// while the film dries untouched, so its neighbors setting (and leaving
    /// the wet layer) doesn't change its rate: a wait split into many short
    /// ones dries it exactly as one long one does.
    pub th: f32,
}

impl Px {
    pub const FRESH: Px = Px { cure: 0.0, lev: SET_TIME, seen: 0.0, sub: 1.0, srate: 0.0, th: 0.0 };
}

/// The clock and the drying state of the wet layer.
#[derive(Clone, Debug, Default)]
pub(crate) struct Clock {
    /// Minutes since the canvas was made.
    pub now: f64,
    /// Per-pixel state, empty until the first `wait`.
    pub px: Vec<Px>,
    /// The newest stroke id seen at the last `wait`: pixels touched by a
    /// later stroke were worked since.
    pub mark: u32,
    /// Box (buffer pixels, end-exclusive) holding every set film that isn't
    /// touch-dry yet.
    pub tacky: Option<(usize, usize, usize, usize)>,
}

fn union(a: Option<(usize, usize, usize, usize)>, b: (usize, usize, usize, usize)) -> (usize, usize, usize, usize) {
    match a {
        None => b,
        Some(a) => (a.0.min(b.0), a.1.min(b.1), a.2.max(b.2), a.3.max(b.3)),
    }
}

impl Canvas {
    /// Minutes on the painting's clock (advanced by `wait` and `dry`).
    pub fn clock(&self) -> f64 {
        if let Some(material) = &self.material {
            return material.store.clock.seconds / 60.0;
        }
        self.wet.clock.now
    }

    /// Let `minutes` pass: the wet paint ages where it lies. Open paint
    /// stiffens; paint that reaches its gel point levels (for as long as it
    /// stayed fluid) and sets; set paint loses its tack and becomes
    /// touch-dry. What happens to each pixel follows from its own paint and
    /// history. `wait(minutes)` advances simulated time. Use `drying_at` to
    /// inspect a location; `dry()` waits until all paint is touch-dry.
    ///
    /// Panics if `minutes` is infinite: a non-finite wait is a caller's
    /// bug, not a way to dry the canvas (that is `dry()`).
    pub fn wait(&mut self, minutes: f32) {
        let dt = minutes.max(0.0);
        assert!(dt.is_finite(), "Canvas::wait: minutes must be finite (got {minutes}); use dry() to wait until touch-dry");
        if self.material.is_some() {
            self.try_wait(minutes).expect("material clock advance");
            return;
        }
        if self.engine >= 3 && self.wet.has_solvent(self.f.w) {
            self.wait_on_grid(dt);
            return;
        }
        self.age(dt);
        self.wet.clock.now += dt as f64;
    }

    pub fn try_wait(&mut self, minutes: f32) -> Result<(), crate::oil::store::MaterialError> {
        if self.material.is_none() { self.wait(minutes); return Ok(()); }
        let dt = minutes.max(0.0);
        if !dt.is_finite() { return Err(crate::oil::store::MaterialError::Invalid("nonfinite material wait")); }
        if dt == 0.0 { return Ok(()); }
        self.material_store().unwrap().check_clock_advance()?;
        self.advance_material_clock(self.clock() * 60.0 + dt as f64 * 60.0)
    }

    /// Wait `dt` minutes with solvent in the paint (engine 3,
    /// `crate::thinner`). The solvent's loss and its flow step on a fine
    /// grid of `FLOW_TICKS` ticks a minute counted from the canvas's start
    /// (not from this wait), each over every step, whole or part: the loss
    /// exact in closed form, the flow over the time the step lasts. So a
    /// wait's flow follows the time waited, a part of a tick included (it
    /// is there before the next stroke or look), and a wait split on the
    /// grid is exactly the wait in one; split off the grid, the flow's
    /// substeps fall differently, within rounding of the same.
    /// Engine 5 keeps this grid while mobility is high, then uses aligned
    /// quarter minutes at a diffusion number <= 0.0025. Fractional splits
    /// consequently have a 1e-3 relative allowance (and 0.001 µm absolute
    /// for paint/solvent); the exact split cases with solvent throughout
    /// remain exact. See notes/thinner/ACCEPTANCE.md, check 17.
    /// The oil's drying steps as before, in steps that end on whole minutes
    /// (or the wait's end), and brushwork's hand time (which waits too)
    /// keeps the same grid; each drying step precedes that tick's flow. Once
    /// the last solvent is gone the rest of the wait is the ordinary one.
    ///
    /// Paint the flow carries keeps the cure it had where it came from,
    /// which is that film's cure at the last drying step, as every open
    /// film's is until the next; so it dries over the whole step, wherever
    /// it is when the step ends. That holds for the thinnest film too: in
    /// this wait a film under `age`'s bare threshold (1e-5 coats) that the
    /// flow brought keeps its cure and dries with the rest (`age_from`),
    /// instead of starting fresh at each drying step, so how the drying
    /// steps fall (a wait split off the grid) doesn't change it.
    fn wait_on_grid(&mut self, dt: f32) {
        const TICKS: f64 = crate::thinner::FLOW_TICKS as f64;
        if self.engine >= 9 && dt > 0.0 {
            let n = self.f.w * self.f.h;
            if self.wet.clock.px.len() != n {
                self.wet.clock.px = vec![Px::FRESH; n];
            }
            self.absorb_with(true);
            self.level_open();
        }
        let start = self.wet.clock.now;
        let end = start + dt as f64;
        let mut t = start;
        let mut aged = start;
        let mut solvent: Vec<usize> = self.wet.solv.iter().enumerate().filter_map(|(i, &s)| (s > 0.0).then_some(i)).collect();
        while t < end {
            // Engine 5 may use the quarter-minute grid once its explicit
            // diffusion number is <= 0.0025 (80 times below the stability
            // limit). Always include minute boundaries for oil aging.
            let ticks = if self.engine >= 5 && self.peak_mobility(&solvent) * 0.25 / self.px_mm().powi(2) <= 0.0025 { 4.0 } else { TICKS };
            let tick = ((t * ticks).floor() + 1.0) / ticks;
            let next = tick.min(end);
            self.evaporate_indices((next - t) as f32, &mut solvent);
            // the oil dries in the steps it always has: to each whole
            // minute and to the wait's end
            let minute = (aged.floor() + 1.0).min(end);
            if next == minute {
                self.age_from((next - aged) as f32, true);
                aged = next;
            }
            self.spread((next - t) as f32, &mut solvent);
            t = next;
            self.wet.clock.now = t;
            if solvent.is_empty() {
                if end > aged {
                    self.age_from((end - aged) as f32, true);
                }
                break;
            }
        }
        self.wet.clock.now = end;
    }

    /// `wait` on the minute grid until no solvent is left: to the next
    /// whole minute, then a whole minute at a time. Ends: each minute every
    /// pixel keeps at most exp(-1 / τ) of its solvent, τ finite, and below
    /// `SOLVENT_FLOOR` it is gone.
    fn wait_out_solvent(&mut self) {
        while self.wet.has_solvent(self.f.w) {
            let now = self.wet.clock.now;
            let step = (now.floor() + 1.0 - now) as f32;
            self.wait_on_grid(if step > 0.0 { step } else { 1.0 });
        }
    }

    /// The open films age by `dt` minutes where they lie (the body of
    /// `wait`, without moving the clock).
    fn age(&mut self, dt: f32) {
        self.age_from(dt, false);
    }

    /// `age`; with `flow` (`wait_on_grid`), a film under the bare
    /// threshold that holds any paint keeps its cure and dries too.
    fn age_from(&mut self, dt: f32, flow: bool) {
        let n = self.f.w * self.f.h;
        if self.wet.clock.px.len() != n {
            self.wet.clock.px = vec![Px::FRESH; n];
        }
        self.absorb_with(flow);
        if self.engine >= 9 && dt > 0.0 {
            self.level_open();
        }
        let low = if flow { f32::MIN_POSITIVE } else { 1e-5 };
        let w = self.f.w;
        // age the open films
        if let Some((x0, y0, x1, y1)) = self.wet.dirty {
            let (x1, y1) = (x1.min(w), y1.min(self.f.h));
            let (pace, drain) = (Pace::of(self.engine), self.drain);
            let wet = &mut self.wet;
            let (vol, hide) = (&wet.vol, &wet.hide);
            wet.clock.px[y0 * w..y1 * w].par_chunks_mut(w).enumerate().for_each(|(j, row)| {
                for x in x0..x1 {
                    let i = (y0 + j) * w + x;
                    if vol[i] >= low {
                        let p = &mut row[x];
                        p.cure = pace.age(p.cure, dt, rate(p.th, lean_of(drain, &hide[i]), hide[i][2]));
                    }
                }
            });
        }
        // and the set ones
        if let Some((x0, y0, x1, y1)) = self.wet.clock.tacky {
            let mut left = false;
            self.wet.clock.px[y0 * w..y1 * w]
                .par_chunks_mut(w)
                .map(|row| {
                    let mut any = false;
                    for p in &mut row[x0..x1] {
                        if p.sub < 1.0 {
                            p.sub += dt * p.srate;
                            any |= p.sub < 1.0;
                        }
                    }
                    any
                })
                .collect::<Vec<bool>>()
                .into_iter()
                .for_each(|a| left |= a);
            if !left {
                self.wet.clock.tacky = None;
            }
        }
        // films past the gel point level and set
        self.bake(false);
    }

    /// The open film's thickness (coats) as it dries, for each pixel of the
    /// buffer box `b` (end-exclusive, row major): the mean thickness of the
    /// wet paint around it, over `FILM_MM` (a double box filter; bare pixels
    /// don't count, so a film's edge isn't judged thinner than its body).
    /// Pixels without wet paint get 0.
    fn film_thickness(&self, b: (usize, usize, usize, usize)) -> Vec<f32> {
        let (w, h) = (self.f.w, self.f.h);
        let (x0, y0, x1, y1) = (b.0, b.1, b.2.min(w), b.3.min(h));
        if x1 <= x0 || y1 <= y0 {
            return Vec::new();
        }
        // two box passes of radius r have a spread (sd) of √(2r(r+1)/3)
        // pixels: pick r for FILM_MM
        let q = FILM_MM / self.px_mm();
        let r = ((0.5 * ((1.0 + 6.0 * q * q).sqrt() - 1.0)).round() as usize).max(1);
        let pad = 2 * r;
        let (ex0, ey0, ex1, ey1) = (x0.saturating_sub(pad), y0.saturating_sub(pad), (x1 + pad).min(w), (y1 + pad).min(h));
        let (ew, eh) = (ex1 - ex0, ey1 - ey0);
        let vol = &self.wet.vol;
        let mut scratch = FILM_SCRATCH.with(|s| std::mem::take(&mut *s.borrow_mut()));
        let [v, m, mid, bv, bm] = &mut scratch;
        v.resize(ew * eh, 0.0);
        v.fill(0.0);
        m.resize(ew * eh, 0.0);
        m.fill(0.0);
        for y in 0..eh {
            for x in 0..ew {
                let a = vol[(ey0 + y) * w + ex0 + x];
                if a >= 1e-5 {
                    v[y * ew + x] = a;
                    m[y * ew + x] = 1.0;
                }
            }
        }
        crate::surface::box_blur_into(v, ew, eh, r, mid);
        crate::surface::box_blur_into(mid, ew, eh, r, bv);
        crate::surface::box_blur_into(m, ew, eh, r, mid);
        crate::surface::box_blur_into(mid, ew, eh, r, bm);
        let bw = x1 - x0;
        let mut out = vec![0.0f32; bw * (y1 - y0)];
        for y in y0..y1 {
            for x in x0..x1 {
                let k = (y - ey0) * ew + x - ex0;
                if m[k] > 0.0 {
                    out[(y - y0) * bw + x - x0] = bv[k] / bm[k].max(1e-6);
                }
            }
        }
        FILM_SCRATCH.with(|s| *s.borrow_mut() = scratch);
        out
    }

    /// Where the paint at a point (units) is in drying.
    pub fn drying_at(&self, x: f32, y: f32) -> Stage {
        self.stage_at_index(self.f.index(x, y))
    }

    /// Where every pixel the canvas holds is in drying (buffer order, as
    /// `pixels()`).
    pub fn stages(&self) -> Vec<Stage> {
        (0..self.f.w * self.f.h).map(|i| self.stage_at_index(i)).collect()
    }

    /// Shares of the canvas (the part `save` writes) that are open, setting,
    /// tacky and dry.
    pub fn stage_shares(&self) -> [f64; 4] {
        let (x0, y0, x1, y1) = self.keep;
        let mut n = [0u64; 4];
        for y in y0..y1 {
            for x in x0..x1 {
                n[self.stage_at_index(y * self.f.w + x) as usize] += 1;
            }
        }
        let t = n.iter().sum::<u64>().max(1) as f64;
        n.map(|k| k as f64 / t)
    }

    /// The stage at pixel `i` of the buffer: open or setting where wet paint
    /// lies (by its cure), else tacky or dry (by the set film's).
    fn stage_at_index(&self, i: usize) -> Stage {
        if let Some(material) = &self.material {
            return crate::oil::stage(&material.store.read_column(i).expect("material pixel"));
        }
        let p = self.wet.clock.px.get(i).copied().unwrap_or(Px::FRESH);
        if self.wet.vol[i] >= 1e-3 {
            if p.cure < 0.5 * GEL { Stage::Open } else { Stage::Setting }
        } else if p.sub < 1.0 {
            Stage::Tacky
        } else {
            Stage::Dry
        }
    }

    /// Let the wet paint dry: wait until every film on the canvas is
    /// touch-dry. Each film levels over the surface for as long as it was
    /// fluid (thin fluid paint pools in the hollows, stiff paint keeps its
    /// marks), then it is composited over the dry picture with Kubelka–Munk
    /// using the settled thickness, and the wet layer is cleared.
    ///
    /// With solvent in the paint (engine 3, `crate::thinner`), the clock
    /// first runs on whole minutes, as `wait` does, until the last of it
    /// has evaporated, so the film flows and loses its solvent as it would
    /// have waiting; then the rest of the way to touch-dry is the shortcut
    /// above.
    pub fn dry(&mut self) {
        if self.material.is_some() {
            while self.stages().iter().any(|stage| *stage != Stage::Dry)
                || (0..self.f.w * self.f.h).any(|i| self.material_store().unwrap().read_column(i).unwrap().state.free_solvent_um > crate::thinner::SOLVENT_FLOOR) {
                self.wait(10.0);
            }
            return;
        }
        if self.engine >= 3 && self.wet.has_solvent(self.f.w) {
            self.wait_out_solvent();
        }
        if self.engine >= 9 && self.wet.dirty.is_some() {
            let n = self.f.w * self.f.h;
            if self.wet.clock.px.len() != n {
                self.wet.clock.px = vec![Px::FRESH; n];
            }
        }
        if !self.wet.clock.px.is_empty() {
            self.absorb();
        }
        if self.engine >= 9 {
            self.level_open();
        }
        // how long until the slowest film is touch-dry
        let mut left = 0.0f32;
        if let Some((x0, y0, x1, y1)) = self.wet.dirty {
            let (w, h) = (self.f.w, self.f.h);
            let (x1, y1) = (x1.min(w), y1.min(h));
            // (never waited: judge each film's thickness now, as the first
            // `wait` would)
            let th = if self.wet.clock.px.is_empty() { self.film_thickness((x0, y0, x1, y1)) } else { Vec::new() };
            let bw = x1 - x0;
            let (pace, drain) = (Pace::of(self.engine), self.drain);
            let (vol, hide, px) = (&self.wet.vol, &self.wet.hide, &self.wet.clock.px);
            left = (y0..y1)
                .into_par_iter()
                .map(|y| {
                    let mut m = 0.0f32;
                    for x in x0..x1 {
                        let i = y * w + x;
                        if vol[i] >= 1e-5 {
                            let (c, t) = px.get(i).map_or((0.0, th.get((y - y0) * bw + x - x0).copied().unwrap_or(0.0)), |p| (p.cure, p.th));
                            m = m.max(pace.to_dry(c, rate(t, lean_of(drain, &hide[i]), hide[i][2])));
                        }
                    }
                    m
                })
                .reduce(|| 0.0, f32::max);
        }
        if let Some((x0, y0, x1, y1)) = self.wet.clock.tacky.take() {
            let w = self.f.w;
            for y in y0..y1 {
                for p in &mut self.wet.clock.px[y * w + x0..y * w + x1] {
                    if p.sub < 1.0 {
                        left = left.max((1.0 - p.sub) / p.srate.max(1e-9));
                        p.sub = 1.0;
                    }
                }
            }
        }
        self.bake(true);
        self.wet.clock.now += left as f64;
    }

    /// Fold what was painted since the last `wait` into the drying state: a
    /// film that was worked levels again, at the fluidity it has now, and
    /// its thickness is judged afresh. (Fresh paint diluted its cure as it
    /// was laid, in `Surf::add`. Engine 1 diluted it here, so until the
    /// next wait a brush felt the film as it was before: see `ENGINE`.)
    fn absorb(&mut self) {
        self.absorb_with(false);
    }

    /// `absorb`; with `flow` (`age_from`), a film under the bare threshold
    /// that holds any paint keeps its drying state.
    fn absorb_with(&mut self, flow: bool) {
        let Some((x0, y0, x1, y1)) = self.wet.dirty else {
            self.wet.clock.mark = self.wet.current;
            return;
        };
        let w = self.f.w;
        let (x1, y1) = (x1.min(w), y1.min(self.f.h));
        let mark = self.wet.clock.mark;
        // a worked film's thickness is judged afresh over the paint that's
        // wet around it now; an untouched one keeps its own (see `Px::th`)
        let now = self.engine >= 2;
        let worked = |wet: &crate::wet::Wet, i: usize| wet.touched[i] > mark || wet.stroke[i] > mark || (now && wet.clock.px[i].seen != wet.vol[i]);
        let fresh = |wet: &crate::wet::Wet, i: usize| worked(wet, i) || wet.clock.px[i].th <= 0.0;
        let any = (y0..y1).into_par_iter().any(|y| (x0..x1).any(|x| self.wet.vol[y * w + x] >= 1e-5 && fresh(&self.wet, y * w + x)));
        let th = if any { self.film_thickness((x0, y0, x1, y1)) } else { Vec::new() };
        let bw = x1 - x0;
        let wet = &mut self.wet;
        let (vol, touched, stroke) = (&wet.vol, &wet.touched, &wet.stroke);
        wet.clock.px[y0 * w..y1 * w].par_chunks_mut(w).enumerate().for_each(|(j, row)| {
            for x in x0..x1 {
                let i = (y0 + j) * w + x;
                let v = vol[i];
                let p = &mut row[x];
                if !(v >= 1e-5) {
                    if !(flow && v > 0.0) {
                        (p.cure, p.lev, p.seen, p.th) = (0.0, SET_TIME, 0.0, 0.0);
                    }
                    continue;
                }
                let worked = touched[i] > mark || stroke[i] > mark || (now && p.seen != v);
                if worked {
                    if !now {
                        p.cure *= p.seen.min(v) / v;
                    }
                    p.lev = SET_TIME * fluid(p.cure);
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
    fn clear_bare_solvent(&mut self, ex: (usize, usize, usize, usize)) {
        if self.wet.solv.is_empty() {
            return;
        }
        let w = self.f.w;
        for y in ex.1..ex.3 {
            for x in ex.0..ex.2 {
                let i = y * w + x;
                if self.wet.vol[i] == 0.0 {
                    self.wet.solv[i] = 0.0;
                }
            }
        }
    }

    /// Level and bake open films into the dry picture: every film (`all`,
    /// clearing the wet layer's residue too) or those past the gel point.
    fn bake(&mut self, all: bool) {
        BAKE_SCRATCH.with(|s| self.bake_with_scratch(all, &mut s.borrow_mut()));
    }

    fn level_open(&mut self) {
        let Some((x0, y0, x1, y1)) = self.wet.dirty else { return };
        let (w, h) = (self.f.w, self.f.h);
        let pad = ((2.0 / self.px_mm()).ceil() as usize).max(2);
        let ex = (x0.saturating_sub(pad), y0.saturating_sub(pad), (x1 + pad).min(w), (y1 + pad).min(h));
        let (ew, eh) = (ex.2 - ex.0, ex.3 - ex.1);
        if !(ex.1..ex.3).any(|y| (ex.0..ex.2).any(|x| {
            let i = y * w + x;
            self.wet.vol[i] >= 1e-5 && self.wet.clock.px[i].lev > 0.0
        })) {
            return;
        }
        let mut add = Vec::with_capacity(ew * eh);
        let mut stiff = Vec::with_capacity(ew * eh);
        let mut sets = Vec::with_capacity(ew * eh);
        let mut fluid = Vec::with_capacity(ew * eh);
        let mut old = Vec::with_capacity(ew * eh);
        for y in ex.1..ex.3 {
            for x in ex.0..ex.2 {
                let i = y * w + x;
                add.push(if self.wet.vol[i] >= 1e-5 { self.wet.vol[i] * COAT_UM } else { 0.0 });
                stiff.push(self.wet.hide[i][1]);
                sets.push(self.wet.clock.px[i].lev);
                fluid.push(1.0 - self.wet.hide[i][7].clamp(0.0, 1.0));
                old.push(self.height[i]);
            }
        }
        let mut transport = Vec::new();
        let t = self.settle_with_transport(ex, &add, &stiff, &sets, true, self.drain.then_some(&fluid[..]), Some(&mut transport));
        self.transport_settled(ex, &add, &t, &transport, &mut vec![0.0; ew * eh]);
        for y in ex.1..ex.3 {
            for x in ex.0..ex.2 {
                let i = y * w + x;
                let k = (y - ex.1) * ew + x - ex.0;
                self.height[i] = old[k];
                if add[k] > 0.0 {
                    self.wet.vol[i] = t[k] / COAT_UM;
                    let p = &mut self.wet.clock.px[i];
                    p.seen = self.wet.vol[i];
                    p.lev = 0.0;
                }
            }
        }
    }

    fn transport_settled(&mut self, ex: (usize, usize, usize, usize), add: &[f32], t: &[f32], transport: &[(usize, usize, f32)], oil: &mut [f32]) {
        let (w, ew, eh) = (self.f.w, ex.2 - ex.0, ex.3 - ex.1);
        let mut volumes = add.to_vec();
        let packed: Vec<_> = add.iter().enumerate().map(|(k, &a)| a * self.wet.hide[(ex.1 + k / ew) * w + ex.0 + k % ew][7]).collect();
        for &(from, to, amount) in transport {
            let i = (ex.1 + from / ew) * w + ex.0 + from % ew;
            let j = (ex.1 + to / ew) * w + ex.0 + to % ew;
            let source = self.wet.lat[i].clone();
            let prop = if self.drain { crate::bristle::take_from(&mut self.wet.hide[i], volumes[from], amount) } else { crate::bristle::fluid_of(self.wet.hide[i]) };
            let latent = &mut self.wet.lat[j];
            let hide = &mut self.wet.hide[j];
            let a = volumes[to] * hide[0] / crate::mixer::scattering(latent).max(1e-9);
            let b = amount * prop[0] / crate::mixer::scattering(&source).max(1e-9);
            let volume = volumes[to] + amount;
            if self.wet.clock.px.len() == self.f.w * self.f.h {
                let cure = self.wet.clock.px[i].cure;
                let receiver = &mut self.wet.clock.px[j];
                receiver.cure += (cure - receiver.cure) * (amount / volume);
            }
            let weight = if a + b > 0.0 { b / (a + b) } else { amount / volume };
            for k in 0..crate::wet::LAT {
                latent[k] += (source[k] - latent[k]) * weight;
            }
            latent.blend_parts(&source, amount / volume);
            for k in 0..hide.len() {
                hide[k] += (prop[k] - hide[k]) * (amount / volume);
            }
            hide[0] = crate::mixer::scattering(latent) * (a + b) / volume;
            volumes[from] -= amount;
            volumes[to] = volume;
        }
        for y in 0..eh {
            for x in 0..ew {
                let k = y * ew + x;
                if t[k] <= 0.0 { continue; }
                let i = (ex.1 + y) * w + ex.0 + x;
                let hd = &mut self.wet.hide[i];
                hd[7] = packed[k] / t[k];
                oil[k] = if self.drain { crate::bristle::surface_oil(hd) * crate::wet::PACKED_OIL / hd[5].max(0.05) } else { hd[4] };
            }
        }
    }

    fn bake_with_scratch(&mut self, all: bool, scratch: &mut [Vec<f32>; 8]) {
        let Some((x0, y0, x1, y1)) = self.wet.dirty else { return };
        let (w, h) = (self.f.w, self.f.h);
        let (x1, y1) = (x1.min(w), y1.min(h));
        let pad = ((2.0 / self.px_mm()).ceil() as usize).max(2);
        let ex = (x0.saturating_sub(pad), y0.saturating_sub(pad), (x1 + pad).min(w), (y1 + pad).min(h));
        let (ew, eh) = (ex.2 - ex.0, ex.3 - ex.1);
        // Most minute steps set no film. Avoid allocating the five full
        // buffers in that case; this is the same selection as below.
        if !all && !(ex.1..ex.3).into_par_iter().any(|y| {
            (ex.0..ex.2).any(|x| {
                let i = y * w + x;
                self.wet.vol[i] >= 1e-5 && self.wet.clock.px.get(i).is_some_and(|p| p.cure >= GEL)
            })
        }) {
            return;
        }
        let [add, stiff, oil, sets, rates, cv, prev, fluid] = scratch;
        for (v, fill) in [(&mut *add, 0.0), (&mut *stiff, 0.5), (&mut *oil, 1.0), (&mut *sets, SET_TIME)] {
            v.resize(ew * eh, fill);
            v.fill(fill);
        }
        // (the drain switch: the share of each film still fluid above its
        // packed layer; without it the buffer stays empty)
        let drain = self.drain;
        if drain {
            fluid.resize(ew * eh, 1.0);
            fluid.fill(1.0);
        }
        // cure per minute of each film that bakes (for its tack afterwards)
        rates.resize(if all { 0 } else { ew * eh }, 0.0);
        rates.fill(0.0);
        let mut any = false;
        let pace = Pace::of(self.engine);
        let cp = &self.wet.clock.px;
        for y in 0..eh {
            for x in 0..ew {
                let i = (ex.1 + y) * w + ex.0 + x;
                let v = self.wet.vol[i];
                if v >= 1e-5 && (all || cp.get(i).is_some_and(|p| p.cure >= GEL)) {
                    let k = y * ew + x;
                    add[k] = v * COAT_UM;
                    stiff[k] = self.wet.hide[i][1];
                    oil[k] = self.wet.hide[i][4];
                    if drain {
                        // its surface is as rich as the paint above the
                        // packed layer; its gloss reads that against the
                        // pigment's packing (a paint at its packed oil is as
                        // glossy as paint naming no tubes at `PACKED_OIL`)
                        let hd = &self.wet.hide[i];
                        oil[k] = crate::bristle::surface_oil(hd) * crate::wet::PACKED_OIL / hd[5].max(0.05);
                        fluid[k] = 1.0 - hd[7].clamp(0.0, 1.0);
                    }
                    if let Some(p) = cp.get(i) {
                        sets[k] = p.lev;
                    }
                    if !all && let Some(p) = cp.get(i) {
                        rates[k] = pace.set_rate(rate(p.th, lean_of(drain, &self.wet.hide[i]), self.wet.hide[i][2]));
                    }
                    any = true;
                }
            }
        }
        if all {
            self.wet.dirty = None;
        }
        if !any {
            if all {
                for y in ex.1..ex.3 {
                    for v in &mut self.wet.vol[y * w + ex.0..y * w + ex.2] {
                        if *v < 1e-5 {
                            *v = 0.0;
                        }
                    }
                }
                self.clear_bare_solvent(ex);
            }
            return;
        }
        let t = if self.engine >= 9 {
            for y in 0..eh {
                for x in 0..ew {
                    self.height[(ex.1 + y) * w + ex.0 + x] += add[y * ew + x];
                }
            }
            self.surf_gen += 1;
            add.clone()
        } else {
            self.settle_for(ex, &add, &stiff, &sets, self.engine >= 4, drain.then_some(&fluid[..]))
        };
        // engine 4: the film's surface is as glossy as it is rich in oil (a
        // thin one shows the surface under it through), and dry paint seals
        // an absorbent ground's pores
        if self.engine >= 4 {
            for y in 0..ex.3 - ex.1 {
                for x in 0..ew {
                    let k = y * ew + x;
                    if add[k] > 0.0 {
                        let i = (ex.1 + y) * w + ex.0 + x;
                        // (the film as it settled here, not as it was laid)
                        let coats = t[k].max(0.0) / COAT_UM;
                        let g = smoothstep(0.15, 1.3, oil[k]);
                        self.gloss[i] += (g - self.gloss[i]) * smoothstep(0.05, 0.6, coats);
                        self.absorb[i] *= (-coats / 0.4).exp();
                    }
                }
            }
        }
        // wet paint closes pinholes: a pixel's share of paint is at least
        // what the two neighbors on opposite sides of it both hold (bare
        // neighbors hold none), so the gaps between the hairs of a wide
        // pointed-tool mark fill as it levels, while a hairline (bare on
        // either side) keeps its share (see `wet::over_share`)
        let cover = {
            let wet = &self.wet;
            // (two passes: a gap two pixels wide closes too)
            cv.clear();
            cv.extend((ex.1..ex.3).flat_map(|y| (ex.0..ex.2).map(move |x| if wet.vol[y * w + x] >= 1e-5 { wet.cover[y * w + x] } else { 0.0 })));
            for _ in 0..2 {
                prev.clone_from(cv);
                let held = |x: usize, y: usize| prev[(y - ex.1) * ew + x - ex.0];
                for y in ex.1 + 1..ex.3.saturating_sub(1) {
                    for x in ex.0 + 1..ex.2.saturating_sub(1) {
                        let c = held(x, y);
                        if c >= 1.0 || c <= 0.0 {
                            continue;
                        }
                        // bridged between two opposite neighbors, any direction
                        let across = [((x - 1, y), (x + 1, y)), ((x, y - 1), (x, y + 1)), ((x - 1, y - 1), (x + 1, y + 1)), ((x + 1, y - 1), (x - 1, y + 1))];
                        cv[(y - ex.1) * ew + x - ex.0] = across.iter().fold(c, |m, &(a, b)| m.max(held(a.0, a.1).min(held(b.0, b.1))));
                    }
                }
            }
            cv
        };
        let px_um = self.px_mm() * 1000.0;
        let spectral = self.spectral_films();
        if spectral && self.backing.is_empty() {
            self.backing.resize(self.px.len(), Default::default());
        }
        let backing_rows = if spectral {
            rayon::iter::Either::Left(self.backing[ex.1 * w..ex.3 * w].par_chunks_mut(w).map(Some))
        } else {
            rayon::iter::Either::Right((ex.1..ex.3).into_par_iter().map(|_| None))
        };
        let wet = &mut self.wet;
        let (lat, hide) = (&wet.lat, &wet.hide);
        let add = &add;
        self.px[ex.1 * w..ex.3 * w]
            .par_chunks_mut(w)
            .zip(wet.vol[ex.1 * w..ex.3 * w].par_chunks_mut(w))
            .zip(self.film[ex.1 * w..ex.3 * w].par_chunks_mut(w))
            .zip(wet.cover[ex.1 * w..ex.3 * w].par_chunks_mut(w))
            .zip(backing_rows)
            .enumerate()
            .for_each(|(j, ((((px, vv), ff), cv), mut backing))| {
                let y = ex.1 + j;
                for x in ex.0..ex.2 {
                    let k = j * ew + x - ex.0;
                    if vv[x] < 1e-5 {
                        if all {
                            vv[x] = 0.0;
                            cv[x] = 1.0;
                        }
                        continue;
                    }
                    if add[k] <= 0.0 {
                        continue;
                    }
                    let ti = t[k] / COAT_UM;
                    let i = y * w + x;
                    let cover = crate::wet::bead_cover(cover[k], ti, px_um);
                    px[x] = if let Some(backing) = backing.as_deref_mut() {
                        backing[x] = backing[x].under(px[x]).over_wet(&lat[i], hide[i][0], ti, cover);
                        backing[x].color
                    } else {
                        crate::wet::over_latent(&lat[i], hide[i][0], px[x], ti, cover, false)
                    };
                    ff[x] += ti;
                    vv[x] = 0.0;
                    cv[x] = 1.0;
                }
            });
        self.clear_bare_solvent(ex);
        let wet = &mut self.wet;
        if wet.clock.px.is_empty() {
            return;
        }
        // the baked films' drying state: a film set at the gel point keeps
        // its tack until it is touch-dry; `all` leaves everything dry
        let spans: Vec<Option<(usize, usize)>> = wet.clock.px[ex.1 * w..ex.3 * w]
            .par_chunks_mut(w)
            .enumerate()
            .map(|(j, row)| {
                let mut span: Option<(usize, usize)> = None;
                for x in ex.0..ex.2 {
                    let k = j * ew + x - ex.0;
                    if add[k] <= 0.0 {
                        continue;
                    }
                    let p = &mut row[x];
                    // (the top film decides the surface's tack)
                    if all || p.cure >= 1.0 {
                        p.sub = 1.0;
                    } else {
                        p.sub = p.cure;
                        p.srate = rates[k];
                        span = Some(span.map_or((x, x + 1), |(a, _)| (a, x + 1)));
                    }
                    (p.cure, p.lev, p.seen, p.th) = (0.0, SET_TIME, 0.0, 0.0);
                }
                span
            })
            .collect();
        for (j, s) in spans.into_iter().enumerate() {
            if let Some((a, b)) = s {
                let y = ex.1 + j;
                wet.clock.tacky = Some(union(wet.clock.tacky, (a, y, b, y + 1)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bristle::{Gesture, Held, Tool};
    use crate::color::hex;
    use crate::surface::Linen;
    use crate::wet::Paint;

    fn canvas() -> Canvas {
        Canvas::new(300, 1.0, hex("#c8b89a")).with_linen(Linen::fine(3))
    }

    fn band(c: &mut Canvas, p: Paint, y: f32, seed: u64) {
        let mut h = Held::new(Tool::filbert(40.0), seed);
        h.load(p, 1.0);
        c.drag(&mut h, &Gesture::new(vec![(150.0, y), (850.0, y)]).pressure(0.9, 0.9), None);
    }

    fn lead_white() -> Paint {
        Paint::body(hex("#e8e4d8")).with_drying(drier::LEAD_WHITE)
    }

    #[test]
    fn solvent_wet_settling_does_not_depend_on_wait_partition() {
        let mut c = Canvas::new(32, 1.0, [1.0; 3]).with_engine(9).with_size_mm(4.0);
        let paint = Paint::new([0.4; 3], 0.9, 0.1).with_drying(0.01);
        c.wet.solv = vec![0.0; 32 * 32];
        for y in 8..24 {
            for x in 8..24 {
                let i = y * 32 + x;
                c.wet.vol[i] = if x < 16 { 4.0 } else { 0.5 };
                c.wet.lat[i] = paint.latent();
                c.wet.hide[i] = paint.prop();
                c.wet.solv[i] = 20.0;
                c.wet.cover[i] = 1.0;
            }
        }
        c.wet.dirty = Some((8, 8, 24, 24));
        let mut single = c.clone();
        single.wait(1.0);
        for parts in [4, crate::thinner::FLOW_TICKS] {
            let mut split = c.clone();
            for _ in 0..parts {
                split.wait(1.0 / parts as f32);
            }
            for (i, (&a, &b)) in single.wet.vol.iter().zip(&split.wet.vol).enumerate() {
                assert!((a - b).abs() * COAT_UM <= 1e-3, "{parts} waits, pixel {i}: {a} vs {b}");
            }
            assert!(split.wet.clock.px.iter().zip(&split.wet.vol).all(|(p, &v)| v < 1e-5 || p.lev == 0.0));
            let mut baked = single.clone();
            baked.dry();
            split.dry();
            for (i, (&a, &b)) in baked.height.iter().zip(&split.height).enumerate() {
                assert!((a - b).abs() <= 1e-3, "dry, {parts} waits, pixel {i}: {a} vs {b}");
            }
        }
    }

    #[test]
    fn settling_carries_cure_before_the_receiving_film_ages() {
        let mut c = Canvas::new(16, 1.0, [1.0; 3]).with_engine(9).with_size_mm(2.0);
        let paint = Paint::new([0.4; 3], 0.9, 0.1);
        for i in 0..256 {
            c.wet.vol[i] = if i % 16 < 8 { 4.0 } else { 0.5 };
            c.wet.lat[i] = paint.latent();
            c.wet.hide[i] = paint.prop();
            c.wet.cover[i] = 1.0;
        }
        c.wet.dirty = Some((0, 0, 16, 16));
        c.wait(0.0);
        for (i, p) in c.wet.clock.px.iter_mut().enumerate() {
            p.cure = if i % 16 < 8 { 0.0 } else { GEL - 1e-6 };
            p.lev = SET_TIME * fluid(p.cure);
        }
        let inventory = |c: &Canvas| c.wet.vol.iter().zip(&c.wet.clock.px).map(|(&v, p)| v as f64 * p.cure as f64).sum::<f64>();
        let before = inventory(&c);
        c.level_open();
        let after = inventory(&c);
        assert!((before - after).abs() < 1e-5, "cure inventory: {before} -> {after}");
        let receiver = 8 * 16 + 8;
        let v = c.wet.vol[receiver];
        assert!(v > 0.5, "fresh paint must enter the old film: {v}");
        assert!((c.wet.clock.px[receiver].cure - (GEL - 1e-6) * 0.5 / v).abs() < 1e-6);
        c.wait(0.1);
        assert_eq!(c.wet.vol[receiver], v);
        assert_eq!(c.film[receiver], 0.0);

        let mut c = Canvas::new(3, 1.0, [1.0; 3]).with_engine(9);
        c.wet.clock.px = vec![Px::FRESH; 9];
        for (i, cure) in [(0, 0.12), (1, 0.0), (2, 0.03)] {
            c.wet.hide[i] = paint.prop();
            c.wet.lat[i] = paint.latent();
            c.wet.clock.px[i].cure = cure;
        }
        c.transport_settled((0, 0, 3, 1), &[20.0, 10.0, 10.0], &[15.0, 10.0, 15.0], &[(0, 1, 5.0), (1, 2, 5.0)], &mut [0.0; 3]);
        for (i, cure) in [(0, 0.12), (1, 0.04), (2, 1.0 / 30.0)] {
            assert!((c.wet.clock.px[i].cure - cure).abs() < 1e-6, "cure at {i}: {}", c.wet.clock.px[i].cure);
        }
    }

    #[test]
    fn settling_transfers_surface_oil_and_keeps_the_packed_layer() {
        let mut c = Canvas::new(3, 1.0, [1.0; 3]).with_engine(9).with_drain(true);
        let paint = Paint::body(hex("#405070"));
        let add = [20.0, 10.0, 10.0];
        for (i, oil, packed) in [(0, 0.8, 0.5), (1, 0.7, 0.0), (2, 0.9, 0.5)] {
            c.wet.lat[i] = paint.latent();
            c.wet.hide[i] = paint.prop();
            c.wet.hide[i][4] = oil;
            c.wet.hide[i][5] = crate::wet::PACKED_OIL;
            c.wet.hide[i][6] = 0.6;
            c.wet.hide[i][7] = packed;
        }
        let mut oil = [0.0; 3];
        c.transport_settled((0, 0, 3, 1), &add, &[15.0, 15.0, 10.0], &[(0, 1, 5.0)], &mut oil);
        for (i, bulk, top, packed) in [(0, 11.0 / 15.0, 1.0, 10.0), (1, 0.8, 0.8, 0.0), (2, 0.9, 1.2, 5.0)] {
            let hd = &c.wet.hide[i];
            assert!((hd[4] - bulk).abs() < 1e-6, "bulk oil at {i}: {}", hd[4]);
            assert!((oil[i] - top).abs() < 1e-6, "surface oil at {i}: {}", oil[i]);
            assert!((hd[7] * [15.0, 15.0, 10.0][i] - packed).abs() < 1e-6, "packed paint at {i}");
        }

        let t = [14.0, 15.0, 11.0];
        c.transport_settled((0, 0, 3, 1), &[15.0, 15.0, 10.0], &t, &[(0, 2, 3.0), (2, 0, 2.0)], &mut oil);
        for (i, bulk, top, packed) in [(0, 10.25 / 14.0, 1.0625, 10.0), (1, 0.8, 0.8, 0.0), (2, 9.75 / 11.0, 1.125, 5.0)] {
            let hd = &c.wet.hide[i];
            assert!((hd[4] - bulk).abs() < 1e-6, "bulk oil at {i}: {}", hd[4]);
            assert!((oil[i] - top).abs() < 1e-6, "surface oil at {i}: {}", oil[i]);
            assert!((hd[7] * t[i] - packed).abs() < 1e-6, "packed paint at {i}");
        }
        let total: f32 = t.iter().enumerate().map(|(i, v)| v * c.wet.hide[i][4]).sum();
        assert!((total - 32.0).abs() < 1e-5, "total oil: {total}");
    }

    #[test]
    fn untouched_relief_does_not_depend_on_wait_partition() {
        for (size, mm) in [(128, 700.0), (400, 80.0)] {
            let mut c = Canvas::new(size, 1.0, [0.45; 3]).with_engine(9).with_size_mm(mm).with_drain(true).with_glaze_film(true);
            for (x, um) in [(220.0, 25.0f32), (500.0, 76.2), (780.0, 254.0)] {
                let mut knife = crate::Knife::new(260.0);
                knife.load(Paint::new([0.45; 3], 0.9, 0.8), 1.0);
                let pressure = 1.0 - (um / 300.0).powf(2.0 / 3.0);
                c.knife(&mut knife, &[(x, 100.0), (x, 900.0)], (pressure, pressure), Some(0.0), true, 0.0);
            }
            c.wait(0.0);
            let mut dry = c.clone();
            dry.dry();
            let total = dry.clock() as f32 * 1.1;
            for share in [0.25, 0.6, 1.0] {
                let mut single = c.clone();
                single.wait(total * share);
                for n in [10, 100] {
                    let mut split = c.clone();
                    for step in 0..n {
                        split.wait(total * share / n as f32);
                        if step == n / 2 {
                            let mut bytes = Vec::new();
                            split.write_state(&mut bytes, "").unwrap();
                            split = Canvas::read_state(&mut &bytes[..]).unwrap().0;
                        }
                    }
                    for (i, (a, b)) in single.wet_surface().iter().zip(split.wet_surface()).enumerate() {
                        assert!((a - b).abs() <= 1e-3, "{size}px, share {share}, {n} waits, pixel {i}: {a} vs {b}");
                    }
                    if share == 1.0 {
                        assert_eq!(single.height, split.height);
                        assert_eq!(single.film, split.film);
                        assert_eq!(single.height, dry.height);
                    }
                }
            }
        }
    }

    #[test]
    fn covers_stay_above_dry_paint_without_inventing_peaks() {
        let mut base = Canvas::new(384, 1.0, [0.45; 3]).with_engine(9).with_size_mm(48.0);
        for pass in 0..3 {
            let mut held = Held::new(Tool::hog_flat(200.0), 17 + pass).with_engine(9);
            held.load(Paint::new([0.45; 3], 0.9, 1.0), 1.0);
            base.drag(&mut held, &Gesture::line((200.0, 490.0 + pass as f32 * 5.0), (800.0, 490.0 + pass as f32 * 5.0)).ramps(0.0, 0.0).shake(0.0), None);
        }
        for _ in 0..2000 {
            base.wait(60.0);
            if base.wet.vol.iter().all(|v| *v < 1e-5) {
                break;
            }
        }
        base.dry();
        assert!(base.height.iter().any(|h| *h > 500.0));
        for gap in [5.0f32, 120.0] {
            let mut c = base.clone();
            let mut knife = crate::Knife::new(260.0);
            knife.load(Paint::new([0.45; 3], 0.9, 0.1), 1.0);
            let p = 1.0 - (gap / 300.0).powf(2.0 / 3.0);
            c.knife(&mut knife, &[(300.0, 500.0), (700.0, 500.0)], (p, p), Some(std::f32::consts::FRAC_PI_2), true, 0.0);
            let fresh = c.wet_surface();
            let ceiling = fresh.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            c.dry();
            for (i, ((wet, baked), old)) in fresh.iter().zip(&c.height).zip(&base.height).enumerate() {
                assert!(wet >= old && baked >= old, "gap {gap}, pixel {i}: old {old}, wet {wet}, baked {baked}");
                assert!(*baked <= ceiling + 1e-3, "gap {gap}, pixel {i}: {baked} exceeds fresh maximum {ceiling}");
            }
        }
        let mut c = Canvas::new(384, 1.0, [0.45; 3]).with_engine(9).with_size_mm(48.0);
        for (i, h) in c.height.iter_mut().enumerate() {
            *h = 100.0 + 100.0 * (std::f32::consts::TAU * (i % 384) as f32 * 0.125 / 0.5).sin();
        }
        let old = c.height.clone();
        let film = c.settle_for((0, 0, 384, 384), &vec![2.0; old.len()], &vec![1.0; old.len()], &vec![SET_TIME; old.len()], true, None);
        for (new, old) in c.height.iter().zip(&old) {
            assert!(new >= old, "a 2 µm cover cannot remove a dry sine ridge: {old} -> {new}");
        }
        let mean = film.iter().map(|v| *v as f64).sum::<f64>() / film.len() as f64;
        assert!((mean - 2.0).abs() < 1e-5, "cover volume: {mean}");
    }

    #[test]
    fn settling_conserves_each_colorant_in_overlapping_knife_marks() {
        let mut p = crate::Palette::new("drying", crate::palette::catalog());
        p.engine = 9;
        p.glaze_film = true;
        p.carried_paint = true;
        let tube = |name| p.tubes.iter().position(|t| t.name == name).unwrap();
        let mut c = Canvas::new(400, 1.0, [1.0; 3]).with_engine(9).with_drain(true).with_glaze_film(true);
        let mut k = crate::Knife::new(200.0);
        k.load(p.pile(vec![(tube("Prussian blue"), 1.0)]).paint(0.9), 1.0);
        c.knife(&mut k, &[(420.0, 250.0), (420.0, 750.0)], (0.45, 0.45), Some(0.0), true, 0.1);
        k.wipe();
        k.load(p.pile(vec![(tube("lead white"), 1.0)]).paint(0.9), 1.0);
        c.knife(&mut k, &[(580.0, 350.0), (580.0, 650.0)], (0.9, 0.9), Some(0.0), true, 0.9);
        let inventory = |c: &Canvas, coats: &[f32]| {
            let mut values = (0..crate::wet::LAT).map(|pig| coats.iter().enumerate().map(|(i, &v)| {
                v as f64 * c.wet.lat[i][pig] as f64 * c.wet.hide[i][0] as f64
                    / crate::mixer::scattering(&c.wet.lat[i]).max(1e-9) as f64
            }).sum::<f64>()).collect::<Vec<_>>();
            values.extend((0..p.tubes.len()).map(|tube| coats.iter().enumerate().map(|(i, &v)| {
                v as f64 * c.wet.lat[i].parts.iter().filter(|p| p.0 == tube).map(|p| p.1 as f64).sum::<f64>()
            }).sum::<f64>()));
            values
        };
        let before = inventory(&c, &c.wet.vol);
        let volume = c.wet.vol.iter().map(|&v| v as f64).sum::<f64>();
        let old = c.film.clone();
        c.dry();
        let film: Vec<_> = c.film.iter().zip(old).map(|(a, b)| a - b).collect();
        let after = inventory(&c, &film);
        for (pig, (&a, &b)) in before.iter().zip(&after).enumerate() {
            assert!((a - b).abs() <= a.abs() * 1e-5 + 1e-6, "colorant {pig}: {a:.8} -> {b:.8}");
        }
        let baked = film.iter().map(|&v| v as f64).sum::<f64>();
        assert!((volume - baked).abs() <= volume * 1e-6, "volume: {volume} -> {baked}");

        let mut c = Canvas::new(3, 1.0, [1.0; 3]).with_engine(9).with_drain(true);
        let mut add = vec![0.0; 9];
        for (i, name, depth) in [(0, "Prussian blue", 20.0), (1, "lead white", 10.0), (3, "vermilion", 20.0)] {
            let paint = p.pile(vec![(tube(name), 1.0)]).paint(0.9);
            c.wet.vol[i] = depth / COAT_UM;
            c.wet.lat[i] = paint.latent();
            c.wet.hide[i] = paint.prop();
            c.wet.hide[i][7] = 0.2;
            add[i] = depth;
        }
        let before = inventory(&c, &c.wet.vol);
        let mut t = add.clone();
        t[0] = 15.0;
        t[1] = 20.0;
        t[3] = 15.0;
        c.transport_settled((0, 0, 3, 3), &add, &t, &[(0, 1, 10.0), (3, 0, 5.0)], &mut [0.0; 9]);
        let after = inventory(&c, &t.iter().map(|t| t / COAT_UM).collect::<Vec<_>>());
        for (pig, (&a, &b)) in before.iter().zip(&after).enumerate() {
            assert!((a - b).abs() <= a.abs() * 1e-5 + 1e-6, "2D colorant {pig}: {a:.8} -> {b:.8}");
        }
        for i in [0, 1, 3] {
            assert!((t[i] * c.wet.hide[i][7] - add[i] * 0.2).abs() < 1e-6, "packed paint at {i}");
        }
    }

    #[test]
    fn a_scraped_nan_film_can_wait_without_a_thickness_buffer() {
        let mut c = canvas().with_engine(crate::ENGINE);
        band(&mut c, lead_white(), 400.0, 1);
        c.wait(0.01);
        for p in &mut c.wet.clock.px {
            p.cure = f32::NAN;
        }
        let mut knife = crate::bristle::Knife::new(8.0);
        c.knife(&mut knife, &[(644.0, 390.0), (644.0, 410.0)], (1.0, 1.0), Some(0.0), false, 1.0);
        assert!(c.wet.vol.iter().any(|v| v.is_nan()));
        for v in &mut c.wet.vol {
            if *v > 0.0 {
                *v = f32::NAN;
            }
        }
        c.wait(1.0);
    }

    /// A checkpoint taken while paint is drying resumes exactly.
    #[test]
    fn checkpoint_mid_drying_resumes_exactly() {
        let mut a = canvas();
        band(&mut a, lead_white(), 400.0, 1);
        a.wait(45.0);
        band(&mut a, Paint::scumble(hex("#405070")), 430.0, 2);
        let mut buf = Vec::new();
        a.write_state(&mut buf, "").unwrap();
        let (mut b, _) = Canvas::read_state(&mut std::io::Cursor::new(buf)).unwrap();
        for c in [&mut a, &mut b] {
            c.wait(120.0);
            band(c, Paint::body(hex("#a04020")), 460.0, 3);
            c.wait(30.0);
            c.dry();
        }
        crate::tests::image_tolerance::assert_fields_close(a.px.iter().flatten().copied(), b.px.iter().flatten().copied());
        crate::tests::image_tolerance::assert_fields_close(a.height.iter().copied(), b.height.iter().copied());
        crate::tests::image_tolerance::assert_fields_close(a.film.iter().copied(), b.film.iter().copied());
        assert_eq!(a.clock(), b.clock());
    }
}
