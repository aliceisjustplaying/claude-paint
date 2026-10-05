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
use crate::pigment::Pigment;
use crate::surface::{COAT_UM, SET_TIME};
use crate::{smoothstep, surface::vnoise};
use rayon::prelude::*;

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
    /// Seconds the open film levels for when it sets (it levels for
    /// `SET_TIME` after it is worked, at the fluidity it had then).
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
        if self.engine >= 3 && self.wet.has_solvent(self.f.w) {
            self.wait_on_grid(dt);
            return;
        }
        self.age(dt);
        self.wet.clock.now += dt as f64;
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
        let start = self.wet.clock.now;
        let end = start + dt as f64;
        let mut t = start;
        let mut aged = start;
        while t < end {
            let tick = ((t * TICKS).floor() + 1.0) / TICKS;
            let next = tick.min(end);
            self.evaporate((next - t) as f32);
            // the oil dries in the steps it always has: to each whole
            // minute and to the wait's end
            let minute = (aged.floor() + 1.0).min(end);
            if next == minute {
                self.age_from((next - aged) as f32, true);
                aged = next;
            }
            self.spread((next - t) as f32);
            t = next;
            self.wet.clock.now = t;
            if !self.wet.has_solvent(self.f.w) {
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
        let low = if flow { f32::MIN_POSITIVE } else { 1e-5 };
        let w = self.f.w;
        // age the open films
        if let Some((x0, y0, x1, y1)) = self.wet.dirty {
            let (x1, y1) = (x1.min(w), y1.min(self.f.h));
            let pace = Pace::of(self.engine);
            let wet = &mut self.wet;
            let (vol, hide) = (&wet.vol, &wet.hide);
            wet.clock.px[y0 * w..y1 * w].par_chunks_mut(w).enumerate().for_each(|(j, row)| {
                for x in x0..x1 {
                    let i = (y0 + j) * w + x;
                    if vol[i] >= low {
                        let p = &mut row[x];
                        p.cure = pace.age(p.cure, dt, rate(p.th, hide[i][1], hide[i][2]));
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
        let mut v = vec![0.0f32; ew * eh];
        let mut m = vec![0.0f32; ew * eh];
        for y in 0..eh {
            for x in 0..ew {
                let a = vol[(ey0 + y) * w + ex0 + x];
                if a >= 1e-5 {
                    v[y * ew + x] = a;
                    m[y * ew + x] = 1.0;
                }
            }
        }
        let blur = |f: &[f32]| crate::surface::box_blur(&crate::surface::box_blur(f, ew, eh, r), ew, eh, r);
        let (bv, bm) = (blur(&v), blur(&m));
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
        if self.engine >= 3 && self.wet.has_solvent(self.f.w) {
            self.wait_out_solvent();
        }
        if !self.wet.clock.px.is_empty() {
            self.absorb();
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
            let pace = Pace::of(self.engine);
            let (vol, hide, px) = (&self.wet.vol, &self.wet.hide, &self.wet.clock.px);
            left = (y0..y1)
                .into_par_iter()
                .map(|y| {
                    let mut m = 0.0f32;
                    for x in x0..x1 {
                        let i = y * w + x;
                        if vol[i] >= 1e-5 {
                            let (c, t) = px.get(i).map_or((0.0, th.get((y - y0) * bw + x - x0).copied().unwrap_or(0.0)), |p| (p.cure, p.th));
                            m = m.max(pace.to_dry(c, rate(t, hide[i][1], hide[i][2])));
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
        let any = (y0..y1).any(|y| (x0..x1).any(|x| self.wet.vol[y * w + x] >= 1e-5 && fresh(&self.wet, y * w + x)));
        let th = if any { self.film_thickness((x0, y0, x1, y1)) } else { Vec::new() };
        let bw = x1 - x0;
        let wet = &mut self.wet;
        let (vol, touched, stroke) = (&wet.vol, &wet.touched, &wet.stroke);
        wet.clock.px[y0 * w..y1 * w].par_chunks_mut(w).enumerate().for_each(|(j, row)| {
            for x in x0..x1 {
                let i = (y0 + j) * w + x;
                let v = vol[i];
                let p = &mut row[x];
                if v < 1e-5 {
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
        let Some((x0, y0, x1, y1)) = self.wet.dirty else { return };
        let (w, h) = (self.f.w, self.f.h);
        let (x1, y1) = (x1.min(w), y1.min(h));
        let pad = ((2.0 / self.px_mm()).ceil() as usize).max(2);
        let ex = (x0.saturating_sub(pad), y0.saturating_sub(pad), (x1 + pad).min(w), (y1 + pad).min(h));
        let (ew, eh) = (ex.2 - ex.0, ex.3 - ex.1);
        let mut add = vec![0.0f32; ew * eh];
        let mut stiff = vec![0.5f32; ew * eh];
        let mut oil = vec![1.0f32; ew * eh];
        let mut sets = vec![SET_TIME; ew * eh];
        // cure per minute of each film that bakes (for its tack afterwards)
        let mut rates = vec![0.0f32; if all { 0 } else { ew * eh }];
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
                    if let Some(p) = cp.get(i) {
                        sets[k] = p.lev;
                    }
                    if !all && let Some(p) = cp.get(i) {
                        rates[k] = pace.set_rate(rate(p.th, self.wet.hide[i][1], self.wet.hide[i][2]));
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
        let t = self.settle_for(ex, &add, &stiff, &sets, self.engine >= 4);
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
        let cover: Vec<f32> = {
            let wet = &self.wet;
            // (two passes: a gap two pixels wide closes too)
            let mut cv: Vec<f32> = (ex.1..ex.3).flat_map(|y| (ex.0..ex.2).map(move |x| if wet.vol[y * w + x] >= 1e-5 { wet.cover[y * w + x] } else { 0.0 })).collect();
            for _ in 0..2 {
                let prev = cv.clone();
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
        let wet = &mut self.wet;
        let (lat, hide) = (&wet.lat, &wet.hide);
        let add = &add;
        self.px[ex.1 * w..ex.3 * w]
            .par_chunks_mut(w)
            .zip(wet.vol[ex.1 * w..ex.3 * w].par_chunks_mut(w))
            .zip(self.film[ex.1 * w..ex.3 * w].par_chunks_mut(w))
            .zip(wet.cover[ex.1 * w..ex.3 * w].par_chunks_mut(w))
            .enumerate()
            .for_each(|(j, (((px, vv), ff), cv))| {
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
                    let c = mixbox::latent_to_linear_float_rgb(&lat[i]);
                    px[x] = crate::wet::over_share(Pigment::masstone(c, hide[i][0]), px[x], ti, crate::wet::bead_cover(cover[k], ti, px_um));
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
    fn stages_follow_the_clock_and_the_pigment() {
        let mut c = canvas();
        band(&mut c, lead_white(), 300.0, 1);
        band(&mut c, Paint::body(hex("#202020")).with_drying(drier::engine3::BONE_BLACK), 700.0, 2);
        assert_eq!(c.drying_at(500.0, 300.0), Stage::Open);
        c.wait(12.0 * 60.0);
        assert_eq!((c.drying_at(500.0, 300.0), c.drying_at(500.0, 700.0)), (Stage::Open, Stage::Open), "both are open after 12 h");
        c.wait(24.0 * 60.0);
        assert_eq!(c.drying_at(500.0, 300.0), Stage::Tacky, "lead white sets within 36 h");
        assert!(matches!(c.drying_at(500.0, 700.0), Stage::Open | Stage::Setting), "bone black is still wet");
        c.wait(36.0 * 60.0);
        assert_eq!(c.drying_at(500.0, 300.0), Stage::Dry, "lead white is touch-dry in three days");
        assert_ne!(c.drying_at(500.0, 700.0), Stage::Dry, "bone black is not");
        assert!((c.clock() - 72.0 * 60.0).abs() < 1e-3);
        c.dry();
        assert_eq!(c.drying_at(500.0, 700.0), Stage::Dry);
        assert_eq!(c.wet_total(), 0.0);
        assert!(c.clock() > 2.0 * 24.0 * 60.0, "bone black takes days: {}", c.clock());
    }

    #[test]
    fn thick_and_fat_films_dry_slower() {
        assert!(rate(1.0, 1.0, 1.0) > rate(4.0, 1.0, 1.0));
        assert!(rate(1.0, 1.0, 1.0) > rate(1.0, 0.1, 1.0));
        assert!(rate(1.0, 1.0, 2.0) > rate(1.0, 1.0, 1.0));
        // one lean coat of average paint: touch-dry in a day
        assert!((1.0 / rate(1.0, 1.0, 1.0) - TOUCH_DRY_MIN).abs() < 1e-2);
    }

    /// Waiting out the drying in steps bakes exactly what `dry()` bakes, and
    /// a zero wait changes nothing.
    #[test]
    fn waiting_it_out_equals_dry() {
        let paint = |c: &mut Canvas| {
            band(c, lead_white(), 400.0, 1);
            band(c, Paint::scumble(hex("#405070")), 450.0, 2);
        };
        let mut a = canvas();
        paint(&mut a);
        a.dry();
        let mut b = canvas();
        paint(&mut b);
        b.wait(0.0);
        b.wait(10.0);
        b.wait(10.0 * 24.0 * 60.0);
        b.dry();
        assert!(a.px == b.px && a.height == b.height && a.film == b.film);
    }

    /// Fresh paint over setting paint is open at once: a blending stroke
    /// right after it does what it does after a zero wait (the brush
    /// doesn't feel the old film's cure until the clock is next read).
    #[test]
    fn fresh_paint_over_setting_paint_is_open_before_the_next_wait() {
        let mut a = canvas();
        band(&mut a, Paint::body(hex("#2040a0")), 500.0, 1);
        a.wait(48.0 * 60.0);
        assert_eq!(a.drying_at(500.0, 500.0), Stage::Setting);
        let mut h = Held::new(Tool::filbert(40.0), 2);
        h.load(Paint::body(hex("#c02020")), 1.0);
        a.drag(&mut h, &Gesture::new(vec![(400.0, 500.0), (600.0, 500.0)]).pressure(0.9, 0.9), None);
        assert_eq!(a.drying_at(500.0, 500.0), Stage::Open);
        let mut b = a.clone();
        b.wait(0.0);
        for c in [&mut a, &mut b] {
            let mut h = Held::new(Tool::filbert(30.0), 7);
            c.drag(&mut h, &Gesture::new(vec![(350.0, 480.0), (650.0, 520.0)]).pressure(0.8, 0.8), None);
            c.dry();
        }
        assert!(a.px == b.px && a.height == b.height && a.film == b.film);
    }

    /// A non-finite wait is a caller's bug, not "wait until dry": it panics
    /// (the easel turns a panic into a failed chunk that changes nothing)
    /// instead of drying the canvas. `dry()` is the way to dry everything.
    #[test]
    #[should_panic(expected = "Canvas::wait: minutes must be finite")]
    fn an_infinite_wait_does_not_dry() {
        let mut c = canvas();
        band(&mut c, lead_white(), 400.0, 1);
        c.wait(f32::INFINITY);
    }

    /// A clean brush lifts wet paint, less as it sets, and nothing from set
    /// or dry paint; a tacky surface pulls paint off a loaded brush sooner
    /// than a dry one (more of it lands early in the stroke).
    #[test]
    fn brushes_feel_the_stage() {
        let mut lifted = Vec::new();
        let mut laid = Vec::new();
        for wait in [0.0, 360.0, 37.0 * 60.0, 5.0 * 24.0 * 60.0] {
            let mut c = canvas();
            band(&mut c, Paint::body(hex("#203050")).with_drying(drier::UMBER), 500.0, 1);
            c.wait(wait);
            let g = Gesture::new(vec![(200.0, 500.0), (800.0, 500.0)]).pressure(0.7, 0.7);
            let mut clean = Held::new(Tool::filbert(20.0), 8);
            let mut d = canvas_copy(&c);
            d.drag(&mut clean, &g, None);
            lifted.push(clean.bristles.iter().map(|b| b.vol as f64).sum::<f64>());
            // paint laid in the first quarter of the stroke
            let f = c.f;
            let early = |c: &Canvas| (0..c.wet.vol.len()).filter(|&i| f.ux(i % f.w) < 350.0).map(|i| c.wet.vol[i] as f64).sum::<f64>();
            let before = early(&c);
            let mut h = Held::new(Tool::filbert(20.0), 9);
            h.load(Paint::scumble(hex("#f0e8d0")), 0.6);
            c.drag(&mut h, &g, None);
            laid.push(early(&c) - before);
        }
        assert!(lifted[0] > 0.0 && lifted[1] < lifted[0], "setting paint lifts less: {lifted:?}");
        // (at 37 h the umber has set: a tacky film, not open paint that fresh
        // paint thins)
        assert!(lifted[2] < 0.05 * lifted[0] && lifted[3] == 0.0, "nothing lifts from set paint: {lifted:?}");
        assert!(laid[2] > laid[3] * 1.2, "tack pulls paint off the brush: {laid:?}");
    }

    /// Engine 3 keeps paint open longer and dries it 2.5 times slower
    /// than engines 1 and 2: one band of lead white, lifted with a clean
    /// brush 12 h after it was laid, and left to dry.
    #[test]
    fn engine_3_keeps_paint_open_longer() {
        let at_12h = |engine: u32| {
            let mut c = canvas().with_engine(engine);
            band(&mut c, lead_white(), 500.0, 1);
            c.wait(12.0 * 60.0);
            let stage = c.drying_at(500.0, 500.0);
            let mut d = canvas_copy(&c);
            let mut clean = Held::new(Tool::filbert(20.0), 8);
            d.drag(&mut clean, &Gesture::new(vec![(200.0, 500.0), (800.0, 500.0)]).pressure(0.7, 0.7), None);
            c.dry();
            (stage, clean.bristles.iter().map(|b| b.vol).sum::<f32>(), c.clock())
        };
        let (old, new) = (at_12h(2), at_12h(3));
        assert_eq!((old.0, new.0), (Stage::Tacky, Stage::Open), "at 12 h: engine 2 {old:?}, engine 3 {new:?}");
        assert!(old.1 < 0.01 * new.1, "lifted at 12 h: engine 2 {}, engine 3 {}", old.1, new.1);
        assert!((new.2 / old.2 - 2.5).abs() < 0.01, "touch-dry at {} min (engine 2), {} min (engine 3)", old.2, new.2);
    }

    /// Hours until an even film `coats` thick of paint of drying rate
    /// `drying` and stiffness `stiff`, painted with engine `engine`, no
    /// longer lifts cleanly (setting), gels (tacky) and is touch-dry, read
    /// every 15 minutes of `wait`.
    fn film(coats: f32, drying: f32, stiff: f32, engine: u32) -> [f32; 3] {
        let mut c = Canvas::new(40, 1.0, [0.5; 3]).with_size_mm(40.0).with_engine(engine);
        let lat = Paint::body([0.9; 3]).latent();
        for i in 0..c.wet.vol.len() {
            (c.wet.vol[i], c.wet.lat[i], c.wet.hide[i], c.wet.stroke[i]) = (coats, lat, [0.85, stiff, drying, 0.0, 1.0], 1);
        }
        c.wet.current = 1;
        c.wet.dirty = Some((0, 0, c.f.w, c.f.h));
        let mut at = [f32::NAN; 3];
        for k in 1..=40 * 24 * 4 {
            c.wait(15.0);
            let s = c.drying_at(500.0, 500.0) as usize;
            for (j, t) in at.iter_mut().enumerate() {
                if s > j && t.is_nan() {
                    *t = k as f32 / 4.0;
                }
            }
            if s == Stage::Dry as usize {
                break;
            }
        }
        at
    }

    /// The tube called `name`, if this build has it: a painter's build for
    /// one box holds only that box's tubes, and one with no box feature only
    /// the default box's (palette.rs `catalog`). The build with every box
    /// has every tube asked for, so there the measures cover them all.
    fn tube(name: &str) -> Option<crate::palette::Tube> {
        let t = crate::palette::catalog().into_iter().find(|t| t.name == name);
        assert!(t.is_some() || !cfg!(feature = "all-boxes"), "no tube {name:?} in the build with every box");
        t
    }

    /// Titanium white, in no box: stiffness 0.7, drying 0.9 ("average to
    /// slow", Natural Pigments) in engine 3; engine 2 would have had no
    /// other rate.
    const TITANIUM: (f32, f32) = (0.7, 0.9);

    /// One typical stroke (`STROKE`) of each tube paint whose touch-dry time
    /// has a source range dries within it in engine 3 (see
    /// `TOUCH_DRY_MIN_3`, `OPEN_SHARE`, `drier::engine3`), all on the one
    /// basis of ordinary brushed paint: lead white workable for at least 12 h,
    /// tacky around a day (18–30 h) and touch-dry in 1–2 days; fast pigments
    /// touch-dry within 2 days, medium ones in 2–5, titanium white in 3–5,
    /// alizarin in 7–14. (Set DRYING_TABLE to print every row, one coat and
    /// one stroke, in both engines.)
    #[test]
    fn strokes_dry_within_the_sources_ranges() {
        const D: f32 = 24.0;
        // tube, touch-dry range (hours), source
        let rows: &[(&str, f32, f32, &str)] = &[
            ("lead white", 1.0 * D, 2.0 * D, "fast (W&N, NP, Golden); painters 1-2 days"),
            ("raw umber", 1.0 * D, 2.0 * D, "fast (W&N, NP)"),
            ("burnt sienna", 1.0 * D, 2.0 * D, "fast (Field/Salter 1869: roasting improves drying)"),
            ("cobalt blue", 1.0 * D, 2.0 * D, "fast (W&N)"),
            ("Prussian blue", 1.0 * D, 2.0 * D, "fast (W&N)"),
            ("yellow ochre", 2.0 * D, 5.0 * D, "medium (W&N, NP)"),
            ("red earth", 2.0 * D, 5.0 * D, "medium (W&N ochres)"),
            ("Mars red", 2.0 * D, 5.0 * D, "medium (W&N)"),
            ("raw sienna", 2.0 * D, 5.0 * D, "medium (slower than burnt: Field/Salter 1869)"),
            ("cadmium red", 2.0 * D, 5.0 * D, "medium (W&N)"),
            ("deep cadmium", 2.0 * D, 5.0 * D, "medium (W&N)"),
            ("ultramarine blue", 2.0 * D, 5.0 * D, "medium (W&N)"),
            ("cobalt violet", 2.0 * D, 5.0 * D, "medium (W&N)"),
            ("bone black", 2.0 * D, 5.0 * D, "medium (NP, W&N ivory black)"),
            ("titanium white", 3.0 * D, 5.0 * D, "artists' guides 3-5 days (not a tube)"),
            ("rose madder", 7.0 * D, 14.0 * D, "alizarin 7-14 days"),
            ("permanent alizarin", 7.0 * D, 14.0 * D, "slow (W&N quinacridones); alizarin 7-14 days"),
        ];
        let table = std::env::var_os("DRYING_TABLE").is_some();
        for &(name, lo, hi, src) in rows {
            let (stiff, d2, d3) = if name == "titanium white" {
                (TITANIUM.0, TITANIUM.1, TITANIUM.1)
            } else {
                // a tube this build doesn't have (`tube`)
                let Some(t) = tube(name) else { continue };
                (t.stiff, t.drying, t.drying_3)
            };
            let at = film(STROKE, d3, stiff, 3);
            if table {
                let r = |c: f32, d: f32, e: u32| film(c, d, stiff, e).map(|h| (h * 4.0).round() / 4.0);
                println!("TABLE | {name} | {d2} -> {d3} | 1 coat: e2 {:?} e3 {:?} | {STROKE} coats: e2 {:?} e3 {:?} | {lo}-{hi} h | {src}", r(1.0, d2, 2), r(1.0, d3, 3), r(STROKE, d2, 2), r(STROKE, d3, 3));
            }
            assert!((lo..=hi).contains(&at[2]), "{name}: setting, tacky, dry at {at:?} h; {src}: {lo}-{hi} h");
        }
        let lead = film(STROKE, drier::LEAD_WHITE, 0.8, 3);
        assert!(lead[0] >= 12.0 && (18.0..=30.0).contains(&lead[1]), "a lead white stroke: workable until {} h, tacky at {} h", lead[0], lead[1]);
    }

    /// At equal thickness, lead white and raw umber are touch-dry before
    /// titanium white and bone black, and alizarin (permanent alizarin, rose
    /// madder) is the slowest of them (every source orders them so). Of the
    /// tubes this build has (`tube`): lead white is in every box, bone black
    /// in every box but the giverny box (which has no black), titanium white
    /// in none (its numbers are given here).
    #[test]
    fn fast_pigments_dry_before_slow_ones() {
        fn dry(name: &'static str) -> Option<(&'static str, f32)> {
            let (stiff, d) = if name == "titanium white" {
                TITANIUM
            } else {
                let t = tube(name)?;
                (t.stiff, t.drying_3)
            };
            Some((name, film(STROKE, d, stiff, 3)[2]))
        }
        let of = |names: &[&'static str]| names.iter().filter_map(|&n| dry(n)).collect::<Vec<_>>();
        let (fast, medium, slow) = (of(&["lead white", "raw umber"]), of(&["titanium white", "bone black"]), of(&["permanent alizarin", "rose madder"]));
        assert!(fast.iter().any(|f| f.0 == "lead white") && medium.iter().any(|m| m.0 == "titanium white"));
        let first = |v: &[(&str, f32)]| v.iter().map(|x| x.1).fold(f32::MAX, f32::min);
        let last = |v: &[(&str, f32)]| v.iter().map(|x| x.1).fold(f32::MIN, f32::max);
        assert!(last(&fast) < first(&medium), "{fast:?} before {medium:?}");
        if !slow.is_empty() {
            assert!(first(&slow) > last(&medium), "{slow:?} the slowest, after {medium:?}");
        }
    }

    /// Roasting improves sienna's drying (Field/Salter 1869, §§50 and 155):
    /// in engine 3 burnt sienna is touch-dry before raw sienna.
    #[test]
    fn burnt_sienna_dries_before_raw_sienna() {
        let (Some(raw), Some(burnt)) = (tube("raw sienna"), tube("burnt sienna")) else { return };
        let (r, b) = (film(STROKE, raw.drying_3, raw.stiff, 3)[2], film(STROKE, burnt.drying_3, burnt.stiff, 3)[2]);
        assert!(b < r, "burnt sienna touch-dry {b} min, raw sienna {r} min");
    }

    /// Golden's titanium white was touch-dry by day 2 at 3 mil and at 4–6
    /// days at 10 mil: a 10-coat film takes 2–3 times as long as a 3-coat
    /// one (`THICK`).
    #[test]
    fn thick_films_dry_as_much_slower_as_goldens() {
        let (stiff, d) = TITANIUM;
        let r = film(10.0, d, stiff, 3)[2] / film(3.0, d, stiff, 3)[2];
        assert!((2.0..=3.0).contains(&r), "10 coats take {r} times as long as 3");
    }

    fn canvas_copy(c: &Canvas) -> Canvas {
        let mut buf = Vec::new();
        c.write_state(&mut buf, "").unwrap();
        Canvas::read_state(&mut std::io::Cursor::new(buf)).unwrap().0
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
        assert!(a.px == b.px && a.height == b.height && a.film == b.film && a.clock() == b.clock());
    }

    /// Share of a field's pixels at each stage after `wait` minutes, for the
    /// same physical film on a canvas `px` wide: lead white in brush
    /// furrows 0.6 mm apart (1 ± 0.8 coats), each pixel holding the mean of
    /// the film over its area (as a brush's deposit does).
    fn stage_shares(px: usize, wait: f32) -> [f32; 4] {
        let mut c = Canvas::new(px, 1.0, hex("#c8b89a")).with_size_mm(440.0);
        let f = c.f;
        let mm = c.mm_per_unit;
        let lat = lead_white().latent();
        let (x0, y0, x1, y1) = (f.index(300.0, 300.0) % f.w, f.index(300.0, 300.0) / f.w, f.index(700.0, 700.0) % f.w, f.index(700.0, 700.0) / f.w);
        let k = std::f32::consts::TAU / 0.6;
        for y in y0..y1 {
            for x in x0..x1 {
                // the furrows' mean over the pixel's span (mm)
                let (a, b) = (f.ux(x) * mm - 0.5 * c.px_mm(), f.ux(x) * mm + 0.5 * c.px_mm());
                let t = 1.0 + 0.8 * ((k * a).cos() - (k * b).cos()) / (k * (b - a));
                let i = y * f.w + x;
                c.wet.vol[i] = t;
                c.wet.lat[i] = lat;
                c.wet.hide[i] = [0.85, 0.8, drier::LEAD_WHITE, 0.0, 1.0];
                c.wet.stroke[i] = 1;
            }
        }
        c.wet.current = 1;
        c.wet.dirty = Some((x0, y0, x1, y1));
        c.wait(wait);
        let mut n = [0.0f32; 4];
        let mut total = 0.0;
        for j in 0..40 {
            for i in 0..60 {
                let s = c.drying_at(330.0 + i as f32 * 5.7, 330.0 + j as f32 * 8.3);
                n[s as usize] += 1.0;
                total += 1.0;
            }
        }
        n.map(|v| v / total)
    }

    /// A film dries by its thickness over a few millimeters, so a coarse
    /// render (each pixel averaging ridges and furrows) and a fine one (the
    /// furrows apart) reach the same stages at the same times, within 10% of
    /// the field per stage. (A per-pixel rate would set the fine render's
    /// thin furrows early and its ridges late.)
    #[test]
    fn stages_dont_depend_on_resolution() {
        for wait in [30.0, 90.0, 150.0] {
            let (lo, hi) = (stage_shares(400, wait), stage_shares(1200, wait));
            let off = lo.iter().zip(&hi).map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
            assert!(off < 0.1, "at {wait} min, stage shares (open, setting, tacky, dry) {lo:?} at 400px vs {hi:?} at 1200px");
        }
    }

    #[test]
    fn drying_is_deterministic_across_thread_counts() {
        let run = || {
            let mut c = canvas();
            band(&mut c, lead_white(), 400.0, 1);
            c.wait(200.0);
            band(&mut c, Paint::scumble(hex("#405070")), 420.0, 2);
            c.wait(60.0);
            c.dry();
            (c.px, c.height)
        };
        let a = rayon::ThreadPoolBuilder::new().num_threads(1).build().unwrap().install(run);
        let b = rayon::ThreadPoolBuilder::new().num_threads(4).build().unwrap().install(run);
        assert!(a == b);
    }

    /// A film of alternating thin fast-drying and thick slow-drying stripes:
    /// its thin stripes set long before its thick ones.
    fn striped_film() -> Canvas {
        let mut c = Canvas::new(20, 1.0, [0.5; 3]).with_size_mm(20.0);
        let p = Paint::body([0.2; 3]);
        for y in 0..20 {
            for x in 0..20 {
                let i = y * 20 + x;
                let thin = x % 2 == 0;
                c.wet.vol[i] = if thin { 0.2 } else { 4.0 };
                c.wet.lat[i] = p.latent();
                c.wet.hide[i] = [p.scatter, 1.0, if thin { 2.0 } else { 0.4 }, 0.0, 1.0];
                c.wet.stroke[i] = 1;
            }
        }
        c.wet.current = 1;
        c.wet.dirty = Some((0, 0, 20, 20));
        c
    }

    /// Checking back often doesn't change the physics: `wait(21000)` and 70
    /// waits of 300 minutes dry a heterogeneous film to the same stages (a
    /// thick stripe's drying thickness stays fixed while its thin neighbors
    /// set, see `Px::th`).
    #[test]
    fn splitting_a_wait_changes_nothing() {
        let (mut a, mut b) = (striped_film(), striped_film());
        a.wait(21000.0);
        for _ in 0..70 {
            b.wait(300.0);
        }
        let i = 10 * 20 + 11;
        let (sa, sb) = (a.wet.clock.px[i].sub.min(1.0), b.wet.clock.px[i].sub.min(1.0));
        assert!((sa - sb).abs() < 1e-3, "slow stripe's cure: one wait {sa}, split {sb}");
        for y in 0..20 {
            for x in 0..20 {
                let (ux, uy) = (x as f32 * 50.0 + 25.0, y as f32 * 50.0 + 25.0);
                assert_eq!(a.drying_at(ux, uy), b.drying_at(ux, uy), "at pixel ({x}, {y})");
            }
        }
        assert_eq!(a.drying_at(575.0, 525.0), Stage::Dry);
    }

    /// The same for two ordinary brushstrokes of fast and slow paint on
    /// linen, checked back every 30 minutes for 50 hours: the same stage
    /// everywhere, the same picture within rounding (films bake in
    /// different groups, so leveling differs a little) and the same time to
    /// dry out.
    #[test]
    fn splitting_a_wait_changes_nothing_under_the_brush() {
        let mut a = Canvas::new(180, 1.0, hex("#c8b89a")).with_linen(Linen::fine(3));
        for (y, col, d, seed) in [(400.0, "#e8e4d8", 2.0, 1), (420.0, "#405070", 0.4, 2)] {
            let mut h = Held::new(Tool::filbert(40.0), seed);
            h.load(Paint::body(hex(col)).with_drying(d), 1.0);
            a.drag(&mut h, &Gesture::new(vec![(150.0, y), (850.0, y)]).pressure(0.9, 0.9), None);
        }
        let mut b = canvas_copy(&a);
        a.wait(3000.0);
        for _ in 0..100 {
            b.wait(30.0);
        }
        let f = a.f;
        let off = (0..f.w * f.h).filter(|&i| a.drying_at(f.ux(i % f.w), f.uy(i / f.w)) != b.drying_at(f.ux(i % f.w), f.uy(i / f.w))).count();
        assert_eq!(off, 0, "pixels at different stages");
        a.dry();
        b.dry();
        assert!((a.clock() - b.clock()).abs() < 1e-3 * a.clock(), "time to dry: {} vs {}", a.clock(), b.clock());
        let d = a.px.iter().zip(&b.px).flat_map(|(p, q)| (0..3).map(move |k| (p[k] - q[k]).abs())).fold(0.0f32, f32::max);
        assert!(d < 1e-3, "picture differs by {d}");
    }
}
