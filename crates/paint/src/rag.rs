//! A rag: a soft cotton cloth bunched into a pad, wiped or pressed over open
//! paint to lift it, the way a painter wipes out a passage or blots a sky.
//!
//! One broad, soft contact. The pad conforms to the canvas: it rests on the
//! tops of the weave and reaches into the hollows as it is pressed harder,
//! and its cloth touches in creases and folds, so a wipe lifts in streaks
//! along its path and a blot in a crumpled patch. What it lifts soaks into
//! the cloth: the face in use loads up and lifts less, until the rag is
//! refolded to a cleaner face (a rag that has soaked up a lot has no clean
//! face left). Before engine 3 it can't lift the last of the film: the
//! pigment caught in the ground's tooth stays as a stain, more of it in the
//! hollows. On engine 3 the last of the film comes away more and more
//! slowly instead (`SLOW_COATS`): a dry rag leaves a pale tint that more
//! wiping thins, spirits take it nearly to the ground.
//!
//! It lifts only the wet layer, by how fluid the paint still is
//! (`reach_fluid`, from `drying::fluid`): fresh paint comes away, paint
//! near its gel point barely does, and paint past the gel point is no
//! longer wet paint at all (`drying` has baked it into the dry picture), so
//! the rag can't reach it.
//!
//! Before engine 3 the rag's paint is not laid back down: a loaded face
//! lifts less, it doesn't smear what it carries onto the canvas. On engine
//! 3 a wiping face smears a little of what it has just lifted back over
//! its light-pressed rim, the wipe's frayed sides and trailing end
//! (`SMEAR`), and that comes off its load. Paint is taken off the
//! wet layer as `bristle::Surf::take` takes it (the same three steps), and
//! nothing new is kept on the canvas: the rag's own state is the `Rag` the
//! caller holds.
//!
//! In a crop render the rag sees only the window's paint, so its load (and
//! what later strokes lift) can differ a little from the whole canvas's,
//! as a brush's pick-up does.

use crate::canvas::Canvas;
use crate::mask::Mask;
use crate::rng::{Rng, hash2};
use crate::smoothstep;
use crate::surface::COAT_UM;
use crate::tally::{Tally, pace as hand};

/// Hand time of a rag. [E]: estimates.
pub mod pace {
    /// How fast a pad pressed into wet paint is dragged (mm/s): slower than
    /// a loaded brush's sweep (`tally::pace::V_MAX`), since the hand bears
    /// down and the cloth drags [E].
    pub const RAG_MM_S: f64 = 150.0;
    /// A blot: the pad pressed straight down and held a moment, then
    /// lifted off [E].
    pub const BLOT_PRESS: f64 = 0.6;
    /// Refolding: opening the bunch, turning a cleaner part of the cloth
    /// outward and bunching it again [E].
    pub const REFOLD: f64 = 3.0;
    /// A fresh rag: taking a clean one from the pile, shaking it out and
    /// bunching it into a pad [E].
    pub const FRESH: f64 = 5.0;
    /// Dipping a corner of the pad into the cup of spirits: a reach to the
    /// palette and back, as a reload (`tally::pace::RELOAD`) [E].
    pub const DIP: f64 = 2.5;
}

/// The pad's width, mm, when none is given: a cloth bunched over two or
/// three fingers [E].
pub const PAD_MM: f32 = 40.0;
/// Lift per pass, a rate: one pass of a clean face at pressure 0.5, through
/// the pad's middle where a crease presses fully, takes `1 - exp(-LIFT)`
/// (98%) of the fresh paint in its reach; the folds between creases (down
/// to 0.4 of that contact) and the pad's soft edge take less. Set so one
/// pass lifts about half of a thin fresh sky film, as in the table in
/// notes/rag/README.md [E].
const LIFT: f32 = 4.0;
/// What a blot lifts, as a share of a full pass (no drag) [E].
const BLOT: f32 = 0.8;
/// The stain before engine 3: coats of film the cloth can't take, pigment
/// caught in the tooth, where the cloth reaches all the film; twice as much
/// where it reaches none of it (the hollows) [E]. Engine 3: `SLOW_COATS`.
const STAIN_COATS: f32 = 0.04;
/// How far the cloth bridges between peaks of the surface (mm): about the
/// spacing of the threads of a 15-thread linen [E].
const BRIDGE_MM: f32 = 0.7;
/// How deep below the local peaks the cloth reaches at pressure 0.5 (µm);
/// it goes from a quarter of this barely touching to 1.75 times pressed
/// hard [E].
const SAG_UM: f32 = 60.0;
/// Share of the paint out of the cloth's reach its fibers wick up anyway [E].
const WICK: f32 = 0.25;
/// How much one face of the pad holds before it lifts nothing more: a film
/// this thick (µm) over the face (`width` square). Cotton cloth some 0.3 mm
/// thick, with paint caught in its surface as well [E].
const CAP_UM: f32 = 400.0;
/// A face dipped in spirits lifts wet paint more readily: its lift rate
/// is `1 + DAMP_LIFT × damp` times a dry face's, and that is all a dip
/// does. "To strengthen the lights, dip the rag into OMS and then wipe them
/// out" (R. Palesca, wipe-out underpainting). Set so a rag dipped at 0.5
/// and gone over a thin tone three times by hand takes 95% of the film from
/// the hollows as well as the tops (the test
/// `a_dry_rag_leaves_a_pale_tint_and_spirits_lift_nearly_to_the_ground`) [E].
const DAMP_LIFT: f32 = 8.0;
/// Spirits evaporate from a damp face as the painting goes on: half of what
/// is left goes every `DAMP_HALF_MIN` minutes of painting time (the clock,
/// hand time included), and below `DRY_DAMP` the face is dry, some 17
/// minutes after a dip at 0.5. "Pour a few drops on a sheet of white
/// writing paper; if it is pure the mark will evaporate in a few minutes"
/// (Arthur Seymour Jennings, Paint & Colour Mixing, 1902, "To Test the
/// Purity of Turpentine", pp. 74-75,
/// https://www.gutenberg.org/cache/epub/56738/pg56738-images.html); a
/// bunched cloth holds more than a few drops and shields part of it, so
/// it takes somewhat longer [E].
pub const DAMP_HALF_MIN: f64 = 3.0;
/// A face this little damp is dry [E].
const DRY_DAMP: f32 = 0.01;
/// How many faces a rag can be refolded to before none is clean: a cloth
/// about 30 cm square [E].
const FACES: f32 = 12.0;
/// The pad's creases: spacing across the path (mm), and how far along the
/// path a crease runs before the cloth shifts (mm) [E].
const CREASE_MM: f32 = 3.0;
const SHIFT_MM: f32 = 25.0;
/// Engine 3: the folds between the creases miss the canvas. The cloth's
/// contact goes from 0 (a fold that doesn't touch) to 1 (a crease pressed
/// fully) over this range of the crease noise (`cloth3`) at pressure 0.5,
/// so about a tenth of the pad misses; pressing harder flattens the folds
/// onto the canvas, moving the range down by `FOLD_PRESS` per unit of
/// pressure (at 0.9 about 3% misses) [E].
const FOLD_MISS: (f32, f32) = (0.25, 0.5);
const FOLD_PRESS: f32 = 0.2;
/// Engine 3: the cloth's contact shares out what the pad's rate takes
/// after that rate saturates, so a damp cloth still leaves the creases'
/// streaks. Spirits work gradually ("rub the paint off gradually", the
/// wipe-out method as the Seattle Artist League describes it,
/// https://www.seattleartistleague.com/2016/08/15/the-wipe-out-method/): a
/// damp face lifts `1 + DAMP_LIFT3 × damp` times as fast as a dry one,
/// reaches `1 + DAMP_REACH × damp` times as deep into the hollows, and its
/// fibers wick up `WICK + (1 - WICK) × DAMP_WICK × damp` of the paint below
/// that. Set by eye (notes/rag/thin-experiment, round 2, "mild"): one
/// damp wipe of a thin wash takes about 80% of it, three about 93% [E].
const DAMP_LIFT3: f32 = 1.0;
const DAMP_REACH: f32 = 0.3;
const DAMP_WICK: f32 = 0.3;
/// Engine 3: no stain the cloth can't take. The last of a film comes away
/// more and more slowly: a wipe takes `v / (v + h)` of what its rate would,
/// `h` = `SLOW_COATS` where the cloth reaches all the film, twice that
/// where it reaches none of it, and spirits shrink `h` by `1 + SLOW_DAMP ×
/// damp`. So a dry rag leaves a pale tint that more wiping thins, and a
/// damp one takes it nearly to the ground. Set by eye (notes/rag/
/// thin-experiment, round 1, version 4); no source gives a thickness [E].
const SLOW_COATS: f32 = 0.04;
const SLOW_DAMP: f32 = 1.0;
/// Engine 3: the pad presses less toward its rim, from `RIM_PRESS` of its
/// half-width out, so there it reaches only the tops of the weave and its
/// fibers wick less; its outline fades only over the outer `1 -
/// RIM_EDGE` (notes/rag/thin-experiment, round 3) [E].
const RIM_PRESS: f32 = 0.3;
const RIM_EDGE: f32 = 0.85;
/// Engine 3: the hand's wander along a wipe: the pad's width by up to
/// ±`WANDER_W` and its line by up to ±`WANDER_OFF` pad widths, at two
/// scales (2.5 or 3 and 0.7 pad widths along the path) [E].
const WANDER_W: f32 = 0.25;
const WANDER_OFF: f32 = 0.15;
/// Engine 3: the pad's sides fray: each side's edge comes in by up to
/// `FRAY` of the pad's half-width, varying over `FRAY_MM` along the path
/// (and a third of that); and the cloth comes down and lifts off unevenly
/// across the pad: each end of a wipe stops between 0.4 short of and 0.8
/// past the pad's round end (pad half-widths), varying over `END_MM`
/// across it [E].
const FRAY: f32 = 0.35;
const FRAY_MM: f32 = 6.0;
const END_MM: f32 = 8.0;
/// Engine 3: a loaded face smears back some of what it has just lifted.
/// The paint lifted in a wipe stays at the cloth's surface for a while: a
/// share `SOAK` of it soaks into the cloth at each step of the pad (a
/// quarter of its width), and of what is left the face lays the share
/// `SMEAR` back at the next step, over its light-pressed rim (the frayed
/// sides and the trailing end of the wipe). What it lays back comes off
/// the rag's load [E].
const SOAK: f32 = 0.35;
const SMEAR: f32 = 0.2;

/// Paint at the cloth's surface during one wipe (engine 3, `SMEAR`):
/// coats × pixels, its mean color, hiding and cure, and its solvent (µm ×
/// pixels).
#[derive(Clone, Copy, Default)]
struct Pool {
    vol: f32,
    lat: crate::wet::Latent,
    hide: crate::wet::Prop,
    cure: f32,
    solv: f32,
}

/// A rag in the hand: its pad width (units), how loaded the face in use is
/// (0 clean .. 1 full), how much the whole cloth has soaked up (0 .. 1, all
/// `FACES` faces full), how damp with spirits the face in use is (0 dry ..
/// 1 dipped well, as of `wet_at`; a refold turns out a dry face, and the
/// spirits evaporate, `DAMP_HALF_MIN`), the fold in use, and its own
/// randomness (the cloth's creases).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rag {
    pub width: f32,
    pub load: f32,
    pub soaked: f32,
    pub damp: f32,
    /// When `damp` was last brought up to date: minutes of painting time
    /// (`Canvas::now_min`).
    pub wet_at: f64,
    pub fold: u32,
    pub seed: u64,
    /// The solvent the cloth has taken off the canvas with the paint, mm³,
    /// cumulative (`crate::thinner`; engine 3). Bookkeeping only: it isn't
    /// the cloth's own dampness (`damp`) and lifts nothing.
    pub solvent_mm3: f64,
}

impl Rag {
    /// A clean rag bunched to a pad `width` units across.
    pub fn new(width: f32, seed: u64) -> Self {
        Rag { width: width.max(0.1), load: 0.0, soaked: 0.0, damp: 0.0, wet_at: 0.0, fold: 0, seed, solvent_mm3: 0.0 }
    }

    /// Turn a cleaner, dry face outward. No face is cleaner than the paint
    /// soaked through the whole cloth so far (`soaked`) leaves it. Counts
    /// the hand time in `t`.
    pub fn refold(&mut self, t: &mut Tally) {
        self.fold = self.fold.wrapping_add(1);
        self.load = self.soaked.clamp(0.0, 1.0);
        self.damp = 0.0;
        t.secs += pace::REFOLD;
    }

    /// Dip the face in use into spirits, the reach starting at `now_min`
    /// (`Canvas::now_min`): `amount` 0..1 (a light dip about 0.5). It is
    /// that damp when the hand is back (`pace::DIP` later), and stays damp
    /// until it is refolded or the spirits evaporate (`evaporate`). Counts
    /// the hand time in `t`.
    pub fn dip(&mut self, amount: f32, now_min: f64, t: &mut Tally) {
        self.evaporate(now_min + pace::DIP / 60.0);
        self.damp = self.damp.max(amount.clamp(0.0, 1.0));
        t.secs += pace::DIP;
    }

    /// The spirits in the face in use evaporated up to `now_min` minutes of
    /// painting time (`Canvas::now_min`): `DAMP_HALF_MIN`.
    pub fn evaporate(&mut self, now_min: f64) {
        let dt = now_min - self.wet_at;
        if dt > 0.0 && self.damp > 0.0 {
            self.damp = (self.damp as f64 * (-dt * std::f64::consts::LN_2 / DAMP_HALF_MIN).exp()) as f32;
            if self.damp < DRY_DAMP {
                self.damp = 0.0;
            }
        }
        self.wet_at = self.wet_at.max(now_min);
    }

    /// How damp the face in use is at `now_min` (`evaporate`), without
    /// changing the rag.
    pub fn damp_at(&self, now_min: f64) -> f32 {
        let mut r = *self;
        r.evaporate(now_min);
        r.damp
    }

    /// How readily the face in use still takes paint (1 clean .. 0 full):
    /// a half-loaded cloth still drinks; a nearly full one barely does.
    fn thirst(&self) -> f32 {
        1.0 - self.load.clamp(0.0, 1.0).powi(2)
    }

    fn cloth_seed(&self, s: u64) -> u64 {
        self.seed ^ (self.fold as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ s.wrapping_mul(0xD6E8_FEB8_6659_FD93)
    }
}

impl Tally {
    /// A rag dragged `len_mm` with a pad `w_mm` wide: brought down where it
    /// starts (Fitts, as a stroke) and dragged at `pace::RAG_MM_S`.
    pub fn rag_stroke(&mut self, len_mm: f64, w_mm: f64) {
        let hop = len_mm.max(hand::HOP_MIN_W * w_mm);
        let aim = hand::FITTS_A + hand::FITTS_B * (1.0 + hop / (2.0 * w_mm.max(0.05))).log2();
        self.strokes += 1;
        self.length_mm += len_mm;
        self.secs += aim + len_mm / pace::RAG_MM_S;
    }

    /// A rag pressed down once (a blot).
    pub fn rag_blot(&mut self) {
        self.touches += 1;
        self.secs += crate::tally::touch_secs() + pace::BLOT_PRESS;
    }
}

/// How readily the cloth takes open paint of cure `c`: 1 fresh, falling as
/// the paint stiffens, 0 at the gel point (`drying::fluid`, the same
/// viscosity a brush feels). A cloth pressed and rubbed into the film soaks
/// up stiff paint that hairs slide over, so it takes the square root of the
/// brush's share: at 60% of the way to the gel point a brush lifts a third
/// as much as from fresh paint, the rag about 60% [E].
#[inline]
fn reach_fluid(c: f32) -> f32 {
    crate::drying::fluid(c).max(0.0).sqrt()
}

/// The largest value within `r` pixels (a square) of every pixel of a `w`
/// × `h` grid, separably.
fn local_max(v: &[f32], w: usize, h: usize, r: usize) -> Vec<f32> {
    let mut a = vec![0.0f32; v.len()];
    for y in 0..h {
        for x in 0..w {
            let (l, rr) = (x.saturating_sub(r), (x + r + 1).min(w));
            a[y * w + x] = v[y * w + l..y * w + rr].iter().copied().fold(f32::MIN, f32::max);
        }
    }
    let mut b = vec![0.0f32; v.len()];
    for y in 0..h {
        let (t, bb) = (y.saturating_sub(r), (y + r + 1).min(h));
        for x in 0..w {
            b[y * w + x] = (t..bb).map(|j| a[j * w + x]).fold(f32::MIN, f32::max);
        }
    }
    b
}

/// Value noise 0..1 (smooth).
#[inline]
fn vn(x: f32, y: f32, seed: u64) -> f32 {
    let (ix, iy) = (x.floor() as i64, y.floor() as i64);
    let (fx, fy) = (x - ix as f32, y - iy as f32);
    let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let (a, b) = (hash2(ix, iy, seed), hash2(ix + 1, iy, seed));
    let (c, d) = (hash2(ix, iy + 1, seed), hash2(ix + 1, iy + 1, seed));
    let top = a + (b - a) * sx;
    let bot = c + (d - c) * sx;
    top + (bot - top) * sy
}

/// The cloth's contact at pad-local (`u`, `v`) mm: creases that press and
/// folds between them that barely touch, at two scales (0.4..1). Before
/// engine 3; engine 3's folds miss (`cloth3`).
#[inline]
fn cloth(u: f32, v: f32, seed: u64) -> f32 {
    let a = vn(u / CREASE_MM, v / SHIFT_MM, seed);
    let b = vn(u / (3.0 * CREASE_MM), v / (2.0 * SHIFT_MM), seed ^ 0x51ED);
    let t = 0.55 * a + 0.45 * b;
    0.4 + 0.6 * smoothstep(0.2, 0.7, t)
}

/// Engine 3's cloth at pressure `p`: as `cloth`, but the folds between
/// creases miss (`FOLD_MISS`, `FOLD_PRESS`): 0..1.
#[inline]
fn cloth3(u: f32, v: f32, seed: u64, p: f32) -> f32 {
    let a = vn(u / CREASE_MM, v / SHIFT_MM, seed);
    let b = vn(u / (3.0 * CREASE_MM), v / (2.0 * SHIFT_MM), seed ^ 0x51ED);
    let t = 0.55 * a + 0.45 * b;
    let sh = FOLD_PRESS * (p.clamp(0.0, 1.0) - 0.5);
    smoothstep(FOLD_MISS.0 - sh, FOLD_MISS.1 - sh, t)
}

/// A planned pass of the rag over a region (`Canvas::rag_region`).
#[derive(Clone, Debug)]
pub struct RagPass {
    /// Pressure 0..1.
    pub pressure: f32,
    /// Direction of the strokes (radians).
    pub angle: f32,
    /// Times over the region.
    pub passes: u32,
    /// Refold whenever the face in use is loaded past this (None: never).
    pub refold: Option<f32>,
    pub seed: u64,
}

impl Canvas {
    /// The wet film under pixel `i` lifted by `take` coats, as
    /// `bristle::Surf::take` does it: a film left bare holds no cure. The
    /// solvent in the film comes away with the paint, in their proportions
    /// there; returns how much (coats; the canvas holds it in µm).
    #[inline]
    fn rag_take(&mut self, i: usize, take: f32) -> f32 {
        let v0 = self.wet.vol[i];
        let mut ts = 0.0;
        if let Some(s) = self.wet.solv.get_mut(i)
            && *s > 0.0
            && v0 > 0.0
        {
            let tu = (*s * take / v0).min(*s);
            *s -= tu;
            ts = tu / COAT_UM;
        }
        let v = &mut self.wet.vol[i];
        *v -= take;
        if self.engine >= 2 && *v < 1e-5 && self.wet.clock.px.len() == self.wet.vol.len() {
            self.wet.clock.px[i].cure = 0.0;
        }
        ts
    }

    /// One contact of the rag over the pixels in `bbox` (units): `expo(x,
    /// y)` is how much of the pad passes over that point (1 = one pass
    /// through its middle), the cloth's contact there and how lightly the
    /// pad's rim presses there (0 in its middle, 1 at its edge or trailing
    /// end; engine 3, `SMEAR`). With `pool` (engine 3), the face first lays
    /// back some of the paint at its surface (`SMEAR`), then takes this
    /// step's lift into it. Lifts the open
    /// paint and loads the rag; returns the volume lifted (mm³).
    ///
    /// The cloth bridges between the local peaks of the surface (the ground,
    /// set paint and the wet film on it, within `BRIDGE_MM`) and sags into
    /// the hollows by `SAG_UM` more as it is pressed: the wet paint above
    /// that level is in reach; below it, in the hollows of the weave and
    /// between ridges, the fibers wick only a share (`WICK`).
    fn rag_contact(&mut self, rag: &mut Rag, bbox: (f32, f32, f32, f32), pressure: f32, mut pool: Option<&mut Pool>, expo: impl Fn(f32, f32) -> (f32, f32, f32, f32)) -> f64 {
        let f = self.f;
        let s = f.scale;
        let r = ((bbox.0 * s).floor().max(0.0) as usize, (bbox.1 * s).floor().max(0.0) as usize, ((bbox.2 * s).ceil().max(0.0) as usize + 1).min(f.full_w), ((bbox.3 * s).ceil().max(0.0) as usize + 1).min(f.full_h));
        if r.2 <= r.0 || r.3 <= r.1 {
            return 0.0;
        }
        let Some((x0, y0, x1, y1)) = f.clip(r) else { return 0.0 };
        let p = pressure.clamp(0.0, 1.0);
        let px_mm = self.px_mm();
        // the surface (µm) and its local peaks, over the box and a bridge's
        // reach around it
        let rb = ((BRIDGE_MM / px_mm).round() as usize).max(1);
        let (bx0, by0, bx1, by1) = (x0.saturating_sub(rb), y0.saturating_sub(rb), (x1 + rb).min(f.w), (y1 + rb).min(f.h));
        let bw = bx1 - bx0;
        let surf: Vec<f32> = (by0..by1).flat_map(|y| (bx0..bx1).map(move |x| (y, x))).map(|(y, x)| {
            let i = y * f.w + x;
            self.height[i] + self.wet.vol[i].max(0.0) * COAT_UM
        }).collect();
        let peaks = local_max(&surf, bw, by1 - by0, rb);
        rag.evaporate(self.now_min());
        let d = rag.damp.clamp(0.0, 1.0);
        let e3 = self.engine >= 3;
        // (engine 3: spirits reach deeper, `DAMP_REACH`)
        let (reach, wick) = if e3 { (SAG_UM * (0.25 + 1.5 * p) * (1.0 + DAMP_REACH * d), WICK + (1.0 - WICK) * (DAMP_WICK * d).min(1.0)) } else { (SAG_UM * (0.25 + 1.5 * p), WICK) };
        let k = LIFT * (0.7 + 0.6 * p) * rag.thirst() * (1.0 + if e3 { DAMP_LIFT3 } else { DAMP_LIFT } * d);
        let timed = self.wet.clock.px.len() == self.wet.vol.len();
        let mut lifted = 0.0f64;
        let mut lifted_s = 0.0f64;
        let mut got = Pool::default();
        let pooled = pool.is_some();
        for y in y0..y1 {
            let mut row = 0.0f32;
            let mut row_s = 0.0f32;
            for x in x0..x1 {
                let i = y * f.w + x;
                let v = self.wet.vol[i];
                if v <= 1e-6 {
                    continue;
                }
                let (pad, c, _, press) = expo(f.ux(x), f.uy(y));
                let e = pad * c;
                if e <= 0.0 {
                    continue;
                }
                let fl = if timed { reach_fluid(self.wet.clock.px[i].cure) } else { 1.0 };
                if fl <= 0.0 {
                    continue;
                }
                let j = (y - by0) * bw + (x - bx0);
                // the film above the cloth's level, coats
                // (engine 3: the rim presses lightly, `RIM_PRESS`)
                let (reach, wick) = if e3 && press < 1.0 {
                    (SAG_UM * (0.25 + 1.5 * p * press) * (1.0 + DAMP_REACH * d), wick * press)
                } else {
                    (reach, wick)
                };
                let level = peaks[j] - reach;
                let near = ((surf[j] - level) / COAT_UM).clamp(0.0, v);
                let avail = near + wick * (v - near);
                // engine 3: the cloth's contact shares out what the pad's
                // rate takes, so its creases show however damp it is
                let frac = if e3 { (1.0 - (-k * fl * pad).exp()) * c } else { 1.0 - (-k * fl * e).exp() };
                // the stain: more of it where the cloth didn't reach
                let hollow = 2.0 - near / v;
                let take = if e3 {
                    // (the last of the film comes away slowly, `SLOW_COATS`)
                    let h = SLOW_COATS * hollow / (1.0 + SLOW_DAMP * d);
                    (avail * frac * v / (v + h)).min(v)
                } else {
                    (avail * frac).min(v - STAIN_COATS * hollow)
                };
                if take > 0.0 {
                    if pooled {
                        let l = &self.wet.lat[i];
                        for q in 0..l.len() {
                            got.lat[q] += l[q] * take;
                        }
                        let h = &self.wet.hide[i];
                        for q in 0..3 {
                            got.hide[q] += h[q] * take;
                        }
                        if timed {
                            got.cure += self.wet.clock.px[i].cure * take;
                        }
                        got.vol += take;
                    }
                    let ts = self.rag_take(i, take);
                    got.solv += ts * COAT_UM;
                    row_s += ts;
                    row += take;
                }
            }
            lifted += row as f64;
            lifted_s += row_s as f64;
        }
        if let Some(pl) = pool.as_deref_mut() {
            let (laid, laid_s) = self.rag_smear(pl, &got, (x0, y0, x1, y1), &expo);
            lifted -= laid;
            lifted_s -= laid_s;
        }
        let mm3 = lifted * (px_mm as f64).powi(2) * (COAT_UM as f64 / 1000.0);
        rag.solvent_mm3 += lifted_s * (px_mm as f64).powi(2) * (COAT_UM as f64 / 1000.0);
        let w_mm = (rag.width * self.mm_per_unit) as f64;
        let cap = w_mm * w_mm * (CAP_UM as f64 / 1000.0);
        if pooled {
            // (a step can lay back more than it lifts)
            rag.load = (rag.load + (mm3 / cap) as f32).clamp(0.0, 1.0);
            rag.soaked = (rag.soaked + (mm3 / (cap * FACES as f64)) as f32).clamp(0.0, 1.0);
        } else {
            rag.load = (rag.load + (mm3 / cap) as f32).min(1.0);
            rag.soaked = (rag.soaked + (mm3 / (cap * FACES as f64)) as f32).min(1.0);
        }
        mm3
    }

    /// Engine 3 (`SMEAR`): the face lays back the share `SMEAR` of the
    /// paint at its surface (`pl`) over the pixels in `px` where its rim
    /// presses lightly (`expo`'s third value), then this step's lift (`got`,
    /// sums by volume) joins what is left after a share `SOAK` has soaked
    /// in. Returns the paint and solvent laid back (coats × pixels).
    fn rag_smear(&mut self, pl: &mut Pool, got: &Pool, px: (usize, usize, usize, usize), expo: &impl Fn(f32, f32) -> (f32, f32, f32, f32)) -> (f64, f64) {
        let f = self.f;
        let (x0, y0, x1, y1) = px;
        let mut out = 0.0f32;
        if pl.vol > 1e-9 {
            let rate = SMEAR * pl.vol / (((x1 - x0) * (y1 - y0)) as f32).max(1.0);
            let per_s = pl.solv / pl.vol;
            let mut bounds: Option<(usize, usize, usize, usize)> = None;
            for y in y0..y1 {
                for x in x0..x1 {
                    let (pad, _, rim, _) = expo(f.ux(x), f.uy(y));
                    let a = (rate * rim).min(pl.vol - out);
                    if pad <= 0.0 || rim <= 0.0 || a <= 0.0 {
                        continue;
                    }
                    self.rag_lay(y * f.w + x, a, pl, a * per_s);
                    out += a;
                    bounds = Some(match bounds {
                        None => (x, y, x + 1, y + 1),
                        Some((a0, b0, c0, d0)) => (a0.min(x), b0.min(y), c0.max(x + 1), d0.max(y + 1)),
                    });
                }
            }
            if let Some((a0, b0, c0, d0)) = bounds {
                self.wet.touch(a0, b0, c0, d0);
            }
        }
        let laid_s = if pl.vol > 1e-9 { out * pl.solv / pl.vol } else { 0.0 };
        pl.solv -= laid_s;
        pl.vol -= out;
        // what is left soaks in a little more; this step's lift joins it
        let v0 = pl.vol.max(0.0) * (1.0 - SOAK);
        let t = v0 + got.vol;
        if t > 0.0 {
            for q in 0..pl.lat.len() {
                pl.lat[q] = (pl.lat[q] * v0 + got.lat[q]) / t;
            }
            for q in 0..3 {
                pl.hide[q] = (pl.hide[q] * v0 + got.hide[q]) / t;
            }
            pl.cure = (pl.cure * v0 + got.cure) / t;
        }
        pl.vol = t;
        pl.solv = pl.solv.max(0.0) * (1.0 - SOAK) + got.solv;
        (out as f64, (laid_s / COAT_UM) as f64)
    }

    /// Engine 3 (`SMEAR`): lay `v` coats of the rag's paint `pl` with
    /// `solv_um` µm of solvent at pixel `i`, mixing by volume as a brush's
    /// paint does (`bristle::Surf::add`).
    fn rag_lay(&mut self, i: usize, v: f32, pl: &Pool, solv_um: f32) {
        let t = self.wet.vol[i] + v;
        let a = v / t;
        let l = &mut self.wet.lat[i];
        for k in 0..l.len() {
            l[k] += (pl.lat[k] - l[k]) * a;
        }
        let hd = &mut self.wet.hide[i];
        for k in 0..3 {
            hd[k] += (pl.hide[k] - hd[k]) * a;
        }
        if self.wet.clock.px.len() == self.wet.vol.len() {
            let p = &mut self.wet.clock.px[i];
            p.cure = if t < 1e-5 { 0.0 } else { p.cure + (pl.cure - p.cure) * a };
        }
        self.wet.cover[i] = 1.0;
        self.wet.vol[i] = t;
        if solv_um > 0.0
            && let Some(sv) = self.wet.solv.get_mut(i)
        {
            *sv += solv_um;
        }
    }

    /// Wipe the rag along `pts` (units) at `pressure` (one value, or values
    /// spread along the path). Counts the hand time; returns the volume
    /// lifted (mm³). One point is a blot (`rag_blot`).
    pub fn rag_wipe(&mut self, rag: &mut Rag, pts: &[(f32, f32)], pressure: &[f32], seed: u64) -> f64 {
        if pts.is_empty() {
            return 0.0;
        }
        let pr = |t: f32| -> f32 {
            match pressure.len() {
                0 => 0.5,
                1 => pressure[0],
                n => {
                    let x = t.clamp(0.0, 1.0) * (n - 1) as f32;
                    let i = (x as usize).min(n - 2);
                    pressure[i] + (pressure[i + 1] - pressure[i]) * (x - i as f32)
                }
            }
        };
        let len: f32 = pts.windows(2).map(|w| ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt()).sum();
        if pts.len() == 1 || len < 1e-3 {
            return self.rag_blot(rag, pts[0].0, pts[0].1, pr(0.0), seed);
        }
        let mmu = self.mm_per_unit;
        self.tally.rag_stroke((len * mmu) as f64, (rag.width * mmu) as f64);
        let cs = rag.cloth_seed(seed);
        let e3 = self.engine >= 3;
        let w = rag.width;
        let r0 = 0.5 * w;
        let step = 0.5 * r0;
        let mut pool = Pool::default();
        let mut total = 0.0;
        let mut s_at = 0.0f32;
        for seg in pts.windows(2) {
            let (a, b) = (seg[0], seg[1]);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let l = (dx * dx + dy * dy).sqrt();
            if l < 1e-6 {
                continue;
            }
            let (tx, ty) = (dx / l, dy / l);
            let n = (l / step).ceil().max(1.0) as usize;
            for k in 0..n {
                let (l0, l1) = (l * k as f32 / n as f32, l * (k + 1) as f32 / n as f32);
                let sm = s_at + 0.5 * (l0 + l1);
                // the pad's width and line wander as the cloth shifts in the hand
                let wob = 2.0 * vn(sm / (2.5 * w), 0.5, cs ^ 0xA1) - 1.0;
                let side = 2.0 * vn(sm / (3.0 * w), 1.5, cs ^ 0xB2) - 1.0;
                let (r, off) = if e3 {
                    // (engine 3: more, and at a second, quicker scale)
                    let wob2 = 2.0 * vn(sm / (0.7 * w), 2.5, cs ^ 0xA7) - 1.0;
                    let side2 = 2.0 * vn(sm / (0.7 * w), 3.5, cs ^ 0xB8) - 1.0;
                    (r0 * (1.0 + WANDER_W * (0.65 * wob + 0.35 * wob2)), WANDER_OFF * w * (0.65 * side + 0.35 * side2))
                } else {
                    (r0 * (1.0 + 0.12 * wob), 0.06 * w * side)
                };
                let p = pr(sm / len);
                let (ax, ay) = (a.0 + tx * l0, a.1 + ty * l0);
                let sl = l1 - l0;
                // Bound the shifted pad, so sideways hand wander cannot be
                // clipped into a straight edge by the contact rectangle.
                let (bx, by) = if e3 { (ax - ty * off, ay + tx * off) } else { (ax, ay) };
                let bbox = (bx.min(bx + tx * sl) - r - 1.0, by.min(by + ty * sl) - r - 1.0, bx.max(bx + tx * sl) + r + 1.0, by.max(by + ty * sl) + r + 1.0);
                let s0 = s_at + l0;
                let pl = if e3 { Some(&mut pool) } else { None };
                total += self.rag_contact(rag, bbox, p, pl, |x, y| {
                    let (qx, qy) = (x - ax, y - ay);
                    let sp = qx * tx + qy * ty;
                    let d = qx * -ty + qy * tx - off;
                    let ad = d.abs();
                    if ad >= r {
                        return (0.0, 0.0, 0.0, 0.0);
                    }
                    let c = (r * r - d * d).sqrt();
                    let over = ((sp + c).min(sl) - (sp - c).max(0.0)).max(0.0);
                    if over <= 0.0 {
                        return (0.0, 0.0, 0.0, 0.0);
                    }
                    if !e3 {
                        let edge = 1.0 - smoothstep(0.55 * r, r, ad);
                        return (over / (2.0 * r) * edge, cloth(d * mmu, (s0 + sp) * mmu, cs), 0.0, 1.0);
                    }
                    // engine 3: frayed sides, and ends where the cloth comes
                    // down and lifts off unevenly across the pad
                    let sg = s0 + sp;
                    let fray = 0.6 * vn(sg * mmu / FRAY_MM, if d > 0.0 { 3.1 } else { 7.3 }, cs ^ 0xD4) + 0.4 * vn(sg * mmu * 3.0 / FRAY_MM, if d > 0.0 { 5.2 } else { 9.4 }, cs ^ 0xD5);
                    let re = r * (1.0 - FRAY * fray);
                    let start = ((sg + r * (1.2 * vn(d * mmu / END_MM, 13.7, cs ^ 0xE6) - 0.4)) / (0.25 * r)).clamp(0.0, 1.0);
                    let end = ((len + r * (1.2 * vn(d * mmu / END_MM, 11.3, cs ^ 0xE5) - 0.4) - sg) / (0.25 * r)).clamp(0.0, 1.0);
                    // the light-pressed rim: the frayed sides and the trailing end
                    let rim = smoothstep(0.4 * re, re, ad).max(smoothstep(len - r, len + 0.5 * r, sg));
                    // the cloth's outline, and its pressure falling off toward
                    // the rim, where it reaches only the tops of the weave
                    let edge = 1.0 - smoothstep(RIM_EDGE * re, re, ad);
                    let press = 1.0 - smoothstep(RIM_PRESS * re, re, ad);
                    (over / (2.0 * r) * edge * start * end, cloth3(d * mmu, sg * mmu, cs, p), rim, press)
                });
            }
            s_at += l;
        }
        total
    }

    /// Press the rag down at (`x`, `y`) units and lift it off: a crumpled
    /// patch of contact about the pad's width. Counts the hand time; returns
    /// the volume lifted (mm³).
    pub fn rag_blot(&mut self, rag: &mut Rag, x: f32, y: f32, pressure: f32, seed: u64) -> f64 {
        self.tally.rag_blot();
        let cs = rag.cloth_seed(seed);
        let mmu = self.mm_per_unit;
        let r0 = 0.5 * rag.width;
        let turn = hash2(1, 2, cs) * std::f32::consts::TAU;
        let (ct, st) = (turn.cos(), turn.sin());
        let bbox = (x - 1.2 * r0 - 1.0, y - 1.2 * r0 - 1.0, x + 1.2 * r0 + 1.0, y + 1.2 * r0 + 1.0);
        let e3 = self.engine >= 3;
        self.rag_contact(rag, bbox, pressure, None, |px, py| {
            let (qx, qy) = (px - x, py - y);
            let d = (qx * qx + qy * qy).sqrt();
            // an irregular outline: the bunch is lumpier than a disc
            let ang = qy.atan2(qx);
            let lump = 2.0 * vn(3.0 * (ang.cos() + 1.0), 3.0 * (ang.sin() + 1.0), cs ^ 0xC3) - 1.0;
            let r = r0 * (1.0 + 0.18 * lump);
            if d >= r {
                return (0.0, 0.0, 0.0, 0.0);
            }
            let (u, v) = (qx * ct + qy * st, -qx * st + qy * ct);
            // crumpled: creases both ways
            let c = if e3 { cloth3(u * mmu, v * mmu * (SHIFT_MM / CREASE_MM), cs, pressure) } else { cloth(u * mmu, v * mmu * (SHIFT_MM / CREASE_MM), cs) };
            (BLOT * (1.0 - smoothstep(0.5 * r, r, d)), c, 0.0, 1.0)
        })
    }

    /// Wipe a region: strokes side by side across the mask in `o.angle`'s
    /// direction, back and forth, each anchored inside the mask and stopping
    /// short of its edge by about a third of the pad (the pad's soft edge
    /// reaches it, a little past in places). `o.passes` times over, each
    /// pass a little turned. Refolds when `o.refold` says so. Long passes
    /// age the paint as they go (hand time slices). Returns the strokes.
    pub fn rag_region(&mut self, rag: &mut Rag, m: &Mask, o: &RagPass) -> usize {
        self.check_mask(m);
        let mut rng = Rng::new(o.seed ^ 0x7A6E_0000);
        let mut n = 0;
        for pass in 0..o.passes.max(1) {
            let ang = o.angle + if pass == 0 { 0.0 } else { rng.range(-0.18, 0.18) };
            let shift = if pass % 2 == 1 { 0.5 } else { 0.0 };
            let plan = plan_region(m, rag.width, ang, shift, &mut rng);
            for (k, pts) in plan.iter().enumerate() {
                if let Some(lim) = o.refold
                    && rag.load > lim
                {
                    rag.refold(&mut self.tally);
                }
                let p = (o.pressure * (1.0 + 0.1 * rng.normal())).clamp(0.0, 1.0);
                let prof = [p * rng.range(0.8, 0.95), p, p * rng.range(0.85, 1.0)];
                let s = o.seed.wrapping_add(((pass as u64) << 32) | k as u64);
                self.rag_wipe(rag, pts, &prof, s);
                n += 1;
                if let Some(slice) = self.hand_slice_secs() {
                    let owed = self.hand_owed_secs();
                    if owed >= slice {
                        self.hand_pass(owed);
                    }
                }
            }
        }
        n
    }
}

/// Strokes over mask `m` (whole canvas) for a pad `w` units wide, in
/// direction `ang`: rows about 0.6 pad apart (offset by `shift` rows),
/// each the runs of the row inside the mask, pulled in at both ends,
/// bowed a little, alternating direction.
pub fn plan_region(m: &Mask, w: f32, ang: f32, shift: f32, rng: &mut Rng) -> Vec<Vec<(f32, f32)>> {
    let f = m.f;
    let (c, s) = (ang.cos(), ang.sin());
    // the mask's extent along (u) and across (v) the strokes
    let (mut umin, mut umax, mut vmin, mut vmax) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
    let stride = ((0.1 * w * f.scale) as usize).max(1);
    for py in (0..f.h).step_by(stride) {
        for px in (0..f.w).step_by(stride) {
            if m.data[py * f.w + px] >= 0.5 {
                let (x, y) = (f.ux(px), f.uy(py));
                let (u, v) = (x * c + y * s, -x * s + y * c);
                umin = umin.min(u);
                umax = umax.max(u);
                vmin = vmin.min(v);
                vmax = vmax.max(v);
            }
        }
    }
    let mut out = Vec::new();
    if umin > umax {
        return out;
    }
    let gap = 0.6 * w;
    let at = |u: f32, v: f32| (u * c - v * s, u * s + v * c);
    // a region narrower than the pad takes one row down its middle
    let narrow = vmax - vmin < 0.6 * w;
    let mut v = if narrow { 0.5 * (vmin + vmax) } else { vmin + 0.3 * w + shift * gap };
    let mut row = 0usize;
    let du = 0.125 * w;
    while (narrow && row == 0) || (!narrow && v <= vmax - 0.2 * w) {
        // the runs of this row inside the mask
        let mut runs = Vec::new();
        let mut start: Option<f32> = None;
        let mut u = umin - du;
        while u <= umax + du {
            let (x, y) = at(u, v);
            let inside = m.sample(x, y) >= 0.5;
            match (inside, start) {
                (true, None) => start = Some(u),
                (false, Some(a)) => {
                    runs.push((a, u - du));
                    start = None;
                }
                _ => {}
            }
            u += du;
        }
        if let Some(a) = start {
            runs.push((a, umax));
        }
        let back = row % 2 == 1;
        if back {
            runs.reverse();
        }
        for (a, b) in runs {
            let (ia, ib) = (w * rng.range(0.2, 0.4), w * rng.range(0.2, 0.4));
            let (mut ua, mut ub) = (a + ia, b - ib);
            if ub - ua < 0.2 * w {
                let mid = 0.5 * (a + b);
                ua = mid - 0.1 * w;
                ub = mid + 0.1 * w;
            }
            let tilt = rng.range(-0.06, 0.06);
            let bow = w * rng.range(-0.08, 0.08);
            let vv = v + w * 0.08 * rng.normal();
            let k = ((ub - ua) / w).ceil().clamp(2.0, 12.0) as usize;
            let mut pts: Vec<(f32, f32)> = (0..=k)
                .map(|j| {
                    let t = j as f32 / k as f32;
                    let uu = ua + (ub - ua) * t;
                    let dv = (t - 0.5) * (ub - ua) * tilt + bow * 4.0 * t * (1.0 - t);
                    at(uu, vv + dv)
                })
                .collect();
            if back {
                pts.reverse();
            }
            out.push(pts);
        }
        v += gap * (1.0 + 0.15 * rng.normal()).clamp(0.7, 1.3);
        row += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bristle::{Gesture, Held, Orient, Tool};
    use crate::color::hex;
    use crate::drying::GEL;
    use crate::handling::Handling;
    use crate::surface::Linen;
    use crate::canvas::Crop;

    /// The live width (px): the weave is resolved (a thread about 3.6 px).
    const LIVE: usize = 2400;
    /// The window the measures paint (a crop render, to keep them quick).
    const WINDOW: Crop = Crop { units: [260.0, 190.0, 740.0, 490.0], margin: 30.0 };

    fn blank(width: usize) -> Canvas {
        let mut c = Canvas::new_window(width, 1.5, hex("#b08060"), Some(WINDOW)).with_size_mm(440.0).with_linen(Linen { warp_per_cm: 15.0, weft_per_cm: 15.0, ..Linen::fine(3) });
        c.prime(hex("#e4dcc8"), 0.9, GROUND_UM, 0.6, 0.3, 5);
        c
    }

    /// Is buffer pixel `i` inside mask `m` (a whole-canvas mask)?
    fn inside(c: &Canvas, m: &Mask, i: usize) -> bool {
        m.data[c.f.whole_index(i)] > 0.5
    }

    /// One thin brushed coat of ground: the weave shows through it.
    const GROUND_UM: f32 = 25.0;

    /// A 440 mm canvas on 15-thread linen with a thin brushed ground, the
    /// weave showing through, and a sky film laid over the middle with a
    /// broad brush.
    fn sky(width: usize, coverage: f32, load: f32) -> Canvas {
        let mut c = blank(width);
        c.set_hand_time(Some(15.0));
        let f = c.frame();
        let m = Mask::from_fn(f, |x, y| if (100.0..900.0).contains(&x) && (120.0..550.0).contains(&y) { 1.0 } else { 0.0 });
        let hd = Handling::new(Tool::filbert(22.0)).color(|_, _| hex("#8fa6c0")).paint(0.85, 0.75).coverage(coverage).load(load).angle(|_, _| 0.0).fill(true);
        c.work(&m, &hd, 11);
        c.clock_hand_min();
        c
    }

    /// The patch the rag wipes, and the part of it the measures read (its
    /// middle, which every stroke's full width reaches).
    fn patch(c: &Canvas) -> (Mask, Mask) {
        let f = c.frame();
        let wipe = Mask::from_fn(f, |x, y| if (300.0..700.0).contains(&x) && (230.0..450.0).contains(&y) { 1.0 } else { 0.0 });
        let read = Mask::from_fn(f, |x, y| if (340.0..660.0).contains(&x) && (270.0..410.0).contains(&y) { 1.0 } else { 0.0 });
        (wipe, read)
    }

    /// Paint over the ground at each pixel (coats): the wet film plus what
    /// has set into the dry picture since `ground` (the film before painting).
    fn paint_at(c: &Canvas, ground: &[f32]) -> Vec<f32> {
        (0..c.wet.vol.len()).map(|i| c.wet.vol[i] + c.film[i] - ground[i]).collect()
    }

    /// Share of the paint removed between `a` and `b` on the tops of the
    /// weave (the upper third of contact levels in `read`) and in the hollows
    /// (the lower third), and over all of `read`.
    fn shares(c: &mut Canvas, a: &[f32], b: &[f32], read: &Mask) -> (f64, f64, f64) {
        let _ = c.surf();
        let base = &c.base.as_ref().unwrap().1;
        let idx: Vec<usize> = (0..a.len()).filter(|&i| inside(c, read, i) && a[i] > 0.02).collect();
        let mut lv: Vec<f32> = idx.iter().map(|&i| base[i]).collect();
        lv.sort_by(f32::total_cmp);
        let (lo, hi) = (lv[lv.len() / 3], lv[2 * lv.len() / 3]);
        let share = |keep: &dyn Fn(f32) -> bool| {
            let (mut x, mut y) = (0.0f64, 0.0f64);
            for &i in &idx {
                if keep(base[i]) {
                    x += a[i] as f64;
                    y += (a[i] - b[i]) as f64;
                }
            }
            y / x
        };
        (share(&|v| v >= hi), share(&|v| v <= lo), share(&|_| true))
    }

    fn mean_cure(c: &Canvas, read: &Mask) -> f32 {
        let (mut s, mut n) = (0.0, 0.0);
        for i in 0..c.wet.vol.len() {
            if inside(c, read, i) && c.wet.vol[i] > 1e-3 {
                s += c.wet.clock.px[i].cure;
                n += 1.0;
            }
        }
        if n > 0.0 { s / n } else { f32::NAN }
    }

    /// The canvas aged until its film's mean cure in `read` reaches `cure`
    /// (a share of the gel point), or until nothing in it is wet any more.
    fn aged(mut c: Canvas, read: &Mask, cure: f32) -> (Canvas, f64) {
        let t0 = c.clock();
        c.wait(0.0);
        while c.clock() - t0 < 30.0 * 24.0 * 60.0 {
            let m = mean_cure(&c, read);
            if m.is_nan() || m >= cure * GEL {
                break;
            }
            c.wait(2.0);
        }
        let t = c.clock() - t0;
        (c, t)
    }

    /// Is any paint in the wet layer (open)?
    fn any_open(c: &Canvas) -> bool {
        c.wet.vol.iter().any(|&v| v > 0.0)
    }

    /// The canvas aged until no paint on it is open, stopping at most two
    /// minutes after the last pixel sets, as `aged(.., 2.0)` over the whole
    /// canvas does: hour-long waits while a copy aged one hour more still
    /// has open paint, then two-minute waits. (`aged` waits two minutes at a
    /// time from the start: over a thin film's day or two that took more
    /// than a minute.)
    fn aged_until_set(mut c: Canvas) -> Canvas {
        let t0 = c.clock();
        c.wait(0.0);
        while c.clock() - t0 < 30.0 * 24.0 * 60.0 {
            let mut ahead = c.clone();
            ahead.wait(60.0);
            if !any_open(&ahead) {
                break;
            }
            c = ahead;
        }
        while any_open(&c) && c.clock() - t0 < 30.0 * 24.0 * 60.0 {
            c.wait(2.0);
        }
        c
    }

    /// The rag over the patch `n` times, refolding to a cleaner face
    /// before each pass and within a pass once a face is half loaded.
    fn wipe_n(c: &mut Canvas, wipe: &Mask, n: u32, pressure: f32) -> Rag {
        let mut r = Rag::new(PAD_MM / c.mm_per_unit(), 77);
        for k in 0..n {
            if k > 0 {
                r.refold(&mut c.tally);
            }
            c.rag_region(&mut r, wipe, &RagPass { pressure, angle: 0.0, passes: 1, refold: Some(0.5), seed: 100 + k as u64 });
        }
        r
    }

    /// Today's way: a flat 16 wiped clean before every short stroke, laid
    /// side by side across the patch (the round 22.1 painter's loop).
    fn brush_wipe(c: &mut Canvas, wipe: &Mask) {
        let mut b = Held::new(Tool { stiffness: 0.6, ..Tool::hog_flat(16.0) }, 9);
        let mut rng = Rng::new(4);
        for _ in 0..900 {
            let (x, y) = (rng.range(300.0, 700.0), rng.range(230.0, 450.0));
            if wipe.sample(x, y) > 0.5 {
                b.wipe(1.0);
                c.tally.wipe();
                let g = Gesture::new(vec![(x - 30.0, y), (x + 30.0, y)]).pressure(0.7, 0.6).orient(Orient::Across);
                c.drag(&mut b, &g, None);
            }
        }
        c.clock_hand_min();
    }

    /// The table in the report: share removed on the tops and in the
    /// hollows for 1, 2 and 3 passes, fresh, near the gel point and past
    /// it, and the brush-wipe loop for comparison.
    /// `cargo test -p paint --lib rag::tests::table -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn table() {
        for (film, cov, load) in [("thick", 2.5f32, 0.8f32), ("thin", 1.0, 0.3)] {
        let c0 = sky(LIVE, cov, load);
        println!("--- {film} film (coverage {cov}, load {load})");
        let (wipe, read) = patch(&c0);
        let ground = blank(LIVE);
        let gf = ground.film.clone();
        for (label, cure) in [("fresh", 0.0f32), ("setting (cure 0.6 gel)", 0.6), ("near gel (cure 0.9 gel)", 0.9), ("past gel", 2.0)] {
            let (aged0, mins) = aged(c0.clone(), &read, cure);
            let before = paint_at(&aged0, &gf);
            print!("{label} (aged {mins:.0} min, mean cure {:.3} gel):", mean_cure(&aged0, &read) / GEL);
            for n in 1..=3 {
                let mut c = aged0.clone();
                let t0 = c.tally.secs;
                let r = wipe_n(&mut c, &wipe, n, 0.5);
                let after = paint_at(&c, &gf);
                let (t, h, a) = shares(&mut c, &before, &after, &read);
                print!("  {n}: tops {:.0}% hollows {:.0}% all {:.0}% ({:.0} s, load {:.2})", 100.0 * t, 100.0 * h, 100.0 * a, c.tally.secs - t0, r.load);
            }
            println!();
        }
        let mut c = c0.clone();
        let before = paint_at(&c, &gf);
        let (t0, n0) = (c.tally.secs, c.tally.strokes);
        brush_wipe(&mut c, &wipe);
        let after = paint_at(&c, &gf);
        let (t, h, a) = shares(&mut c, &before, &after, &read);
        println!("brush-wipe loop, fresh: tops {:.0}% hollows {:.0}% all {:.0}% ({} strokes, {:.0} s)", 100.0 * t, 100.0 * h, 100.0 * a, c.tally.strokes - n0, c.tally.secs - t0);
        // finding 5 of the review: the blender is wiped every 3 strokes
        #[cfg(tube_box)]
        {
            let st = crate::style::Style::oil();
            let hd = st.blend().unwrap();
            let mut c = c0.clone();
            for k in 0..3 {
                c.work(&wipe, &hd, 40 + k);
                let after = paint_at(&c, &gf);
                let (t, h, a) = shares(&mut c, &before, &after, &read);
                println!("blend(), {} pass(es), fresh: tops {:.0}% hollows {:.0}% all {:.0}%", k + 1, 100.0 * t, 100.0 * h, 100.0 * a);
            }
        }
        for p in [0.2f32, 0.8] {
            let mut c = c0.clone();
            wipe_n(&mut c, &wipe, 1, p);
            let after = paint_at(&c, &gf);
            let (t, h, a) = shares(&mut c, &before, &after, &read);
            println!("fresh, 1 pass, pressure {p}: tops {:.0}% hollows {:.0}% all {:.0}%", 100.0 * t, 100.0 * h, 100.0 * a);
        }
        let ins: Vec<usize> = (0..before.len()).filter(|&i| inside(&c0, &read, i)).collect();
        let mean: f64 = ins.iter().map(|&i| before[i] as f64).sum::<f64>() / ins.len() as f64;
        println!("film in the read area: {mean:.2} coats");
        }
    }

    /// Everything the canvas shows and holds: pixels, surface, film and the
    /// wet layer.
    fn state_bits(c: &Canvas) -> Vec<u32> {
        c.px.iter().flat_map(|p| p.map(f32::to_bits)).chain(c.height.iter().chain(&c.film).chain(&c.wet.vol).map(|v| v.to_bits())).collect()
    }

    /// The spirits in a dipped face evaporate as the painting goes on: half
    /// of them every `DAMP_HALF_MIN` minutes, none left within the half hour
    /// (nor a week later), and a dip wets it again. A rag dipped and then
    /// left half an hour lifts exactly what a dry one does; dipped just
    /// before the wipe, it lifts more.
    #[test]
    fn the_spirits_in_a_dipped_rag_evaporate() {
        let mut t = Tally::default();
        let mut r = Rag::new(10.0, 1);
        // dipped at 100 minutes, back with the hand 2.5 s later
        r.dip(0.5, 100.0 - pace::DIP / 60.0, &mut t);
        assert_eq!(r.damp_at(100.0), 0.5);
        assert!((r.damp_at(100.0 + DAMP_HALF_MIN) - 0.25).abs() < 1e-6, "{}", r.damp_at(100.0 + DAMP_HALF_MIN));
        assert!(r.damp_at(110.0) > 0.04 && r.damp_at(110.0) < 0.06, "{}", r.damp_at(110.0));
        assert_eq!(r.damp_at(130.0), 0.0);
        assert_eq!(r.damp_at(100.0 + 7.0 * 24.0 * 60.0), 0.0, "a week later");
        // reading it changes nothing; using it at a later time brings it up to date
        assert_eq!(r.damp, 0.5);
        r.evaporate(103.0);
        assert!((r.damp - 0.25).abs() < 1e-6 && r.wet_at == 103.0);
        r.evaporate(200.0);
        assert_eq!(r.damp, 0.0);
        r.dip(0.5, 200.0, &mut t);
        assert_eq!(r.damp_at(200.0 + pace::DIP / 60.0), 0.5, "dipped again");

        let c0 = sky(LIVE, 1.0, 0.3);
        let (wipe, read) = patch(&c0);
        let gf = blank(LIVE).film;
        let pass = RagPass { pressure: 0.6, angle: 0.0, passes: 1, refold: None, seed: 9 };
        let fresh = || Rag::new(PAD_MM / c0.mm_per_unit(), 4);
        let wiped = |dip_then_wait: Option<bool>| {
            let mut c = c0.clone();
            let mut r = fresh();
            if dip_then_wait == Some(true) {
                r.dip(0.5, c.now_min(), &mut c.tally);
            }
            c.wait(30.0);
            if dip_then_wait == Some(false) {
                r.dip(0.5, c.now_min(), &mut c.tally);
            }
            // the same hand time either way
            if dip_then_wait.is_none() {
                c.tally.secs += pace::DIP;
            }
            let before = paint_at(&c, &gf);
            c.rag_region(&mut r, &wipe, &pass);
            let after = paint_at(&c, &gf);
            let off = shares(&mut c, &before, &after, &read).2;
            (c, r, off)
        };
        let (dry, dry_r, dry_off) = wiped(None);
        let (left, left_r, _) = wiped(Some(true));
        let (_, _, damp_off) = wiped(Some(false));
        assert!(dry_off > 0.05, "the paint is still open after half an hour: {dry_off}");
        assert!(state_bits(&left) == state_bits(&dry) && (left_r.load, left_r.soaked, left_r.damp) == (dry_r.load, dry_r.soaked, 0.0), "a rag left to dry lifts as a dry one");
        assert!(damp_off > dry_off + 0.05, "a fresh dip lifts more: {damp_off} vs {dry_off}");
    }

    /// Two wipes must transfer paint into the cloth without creating or losing
    /// it, including paint the loaded rim lays back. A second damp wipe must
    /// continue clearing the same passage. Existing lift tests do not account
    /// for the paint held by the cloth after smear-back.
    #[test]
    fn repeated_wipes_conserve_paint_and_a_second_damp_wipe_clears_more() {
        let c0 = sky(480, 1.0, 0.3);
        let ground = blank(480);
        let (_, read) = patch(&c0);
        let tone = tint(&c0, &ground, &read);
        let volume = |c: &Canvas| c.wet.vol.iter().map(|&v| v as f64).sum::<f64>()
            * (c.px_mm() as f64).powi(2) * COAT_UM as f64 / 1000.0;
        let before = volume(&c0);
        for damp in [false, true] {
            let mut c = c0.clone();
            let mut r = Rag::new(100.0, 7);
            let cap = (r.width * c.mm_per_unit()) as f64;
            let cap = cap * cap * CAP_UM as f64 / 1000.0;
            if damp {
                r.dip(0.5, c.now_min(), &mut c.tally);
            }
            let mut previous = before;
            let mut previous_tone = tone;
            for pass in 1..=2 {
                c.rag_wipe(&mut r, &[(300.0, 340.0), (700.0, 340.0)], &[0.8], 19);
                let after = volume(&c);
                let left = tint(&c, &ground, &read);
                assert!(r.load > 0.0 && r.load < 0.9, "cloth has spare capacity");
                assert!((after + r.load as f64 * cap - before).abs() < before * 1e-5,
                    "damp={damp} pass={pass}: canvas {after}, cloth {}, original {before}", r.load as f64 * cap);
                assert!(after < previous, "each wipe removes more paint");
                if damp {
                    assert!(left < previous_tone, "a second damp wipe clears more color");
                }
                println!("damp={damp} pass={pass}: paint left {:.1}%, tone left {:.1}%", 100.0 * after / before, 100.0 * left / tone);
                if let Ok(out) = std::env::var("RAG_REVIEW_DIR") {
                    c.clone().save(std::path::Path::new(&out).join(format!("{}-{pass}.png", if damp { "damp" } else { "dry" }))).unwrap();
                }
                previous = after;
                previous_tone = left;
            }
        }
        if let Ok(out) = std::env::var("RAG_REVIEW_DIR") {
            c0.clone().save(std::path::Path::new(&out).join("before.png")).unwrap();
        }
    }

    /// Paint past its gel point has left the wet layer: no wipe, blot or
    /// pressure lifts any of it, and the rag comes away clean. The hand
    /// time is still spent.
    #[test]
    fn nothing_is_lifted_past_the_gel_point() {
        let c0 = sky(LIVE, 1.0, 0.3);
        let (wipe, read) = patch(&c0);
        // aged until no paint on the canvas is open (the thickest dabs set last),
        // within two minutes of the last one setting
        let mut c = aged_until_set(c0);
        assert!(c.wet.vol.iter().all(|&v| v <= 0.0), "paint still open");
        let ground = blank(LIVE);
        let laid: f32 = (0..c.film.len()).filter(|&i| inside(&c, &read, i)).map(|i| c.film[i] - ground.film[i]).sum();
        assert!(laid > 0.0);
        let before = state_bits(&c);
        let t0 = c.tally.secs;
        let mut r = Rag::new(PAD_MM / c.mm_per_unit(), 5);
        // the patch lies within the crop window: wipe only over it
        c.rag_region(&mut r, &wipe, &RagPass { pressure: 1.0, angle: 0.3, passes: 3, refold: Some(0.2), seed: 1 });
        c.rag_wipe(&mut r, &[(320.0, 300.0), (680.0, 360.0)], &[1.0], 2);
        c.rag_blot(&mut r, 500.0, 340.0, 1.0, 3);
        // nor a rag damp with spirits
        r.dip(1.0, c.now_min(), &mut c.tally);
        c.rag_region(&mut r, &wipe, &RagPass { pressure: 1.0, angle: 0.0, passes: 1, refold: None, seed: 4 });
        assert!(state_bits(&c) == before, "the rag changed set paint");
        assert_eq!((r.load, r.soaked), (0.0, 0.0));
        assert!(c.tally.secs > t0);
    }

    /// On fresh paint: a pass lifts more from the tops of the weave than
    /// from the hollows; harder pressure and a second pass lift more; a
    /// loaded face lifts less than a clean one, and a refold to a cleaner
    /// face lifts more again; a stain stays.
    #[test]
    fn the_rag_lifts_open_paint_from_the_tops_first() {
        let c0 = sky(LIVE, 1.0, 0.3);
        let (wipe, read) = patch(&c0);
        let gf = blank(LIVE).film;
        let before = paint_at(&c0, &gf);
        let removed = |c: &mut Canvas| shares(c, &before, &paint_at(c, &gf), &read);
        let pass = |pressure| RagPass { pressure, angle: 0.0, passes: 1, refold: None, seed: 100 };
        let fresh = || Rag::new(PAD_MM / c0.mm_per_unit(), 77);

        let mut c = c0.clone();
        let mut r = fresh();
        c.rag_region(&mut r, &wipe, &pass(0.5));
        let one = removed(&mut c);
        assert!(one.0 > one.1 + 0.1, "tops {:.2} hollows {:.2}", one.0, one.1);
        assert!(r.load > 0.1);
        // the same face again: it lifts more, but less than a clean one does
        let mut again = c.clone();
        let mut loaded = r;
        again.rag_region(&mut loaded, &wipe, &RagPass { seed: 101, ..pass(0.5) });
        let mut clean = c.clone();
        let mut refolded = r;
        refolded.refold(&mut clean.tally);
        assert!(refolded.load < r.load);
        clean.rag_region(&mut refolded, &wipe, &RagPass { seed: 101, ..pass(0.5) });
        let (two_loaded, two_clean) = (removed(&mut again).2, removed(&mut clean).2);
        assert!(one.2 < two_loaded && two_loaded < two_clean, "one {:.2} loaded {two_loaded:.2} refolded {two_clean:.2}", one.2);
        // pressed harder
        let mut hard = c0.clone();
        hard.rag_region(&mut fresh(), &wipe, &pass(0.9));
        let h = removed(&mut hard);
        assert!(h.2 > one.2 + 0.1 && h.1 > one.1, "pressure 0.9 {h:?} vs 0.5 {one:?}");
        // even pressed hard and gone over five times, a dry rag thins the
        // film without baring the ground anywhere (engine 3: `SLOW_COATS`)
        let mut many = c0.clone();
        let mut r = fresh();
        many.rag_region(&mut r, &wipe, &RagPass { passes: 5, refold: Some(0.3), ..pass(0.9) });
        let left = paint_at(&many, &gf);
        let ins: Vec<usize> = (0..left.len()).filter(|&i| inside(&many, &read, i) && before[i] > 0.1).collect();
        assert!(ins.iter().all(|&i| left[i] > 0.0 && left[i] < before[i]), "five passes thin every pixel and bare none");
    }

    /// A toning layer as the wipe-out handouts lay it: thin earth-brown
    /// paint brushed over the whole patch (`work`, one layer, a light load).
    fn toned() -> Canvas {
        let mut c = blank(LIVE);
        c.set_hand_time(Some(15.0));
        let f = c.frame();
        let m = Mask::from_fn(f, |x, y| if (100.0..900.0).contains(&x) && (120.0..550.0).contains(&y) { 1.0 } else { 0.0 });
        let hd = Handling::new(Tool::filbert(22.0)).color(|_, _| hex("#5b4330")).paint(0.8, 0.4).coverage(1.0).load(0.3).angle(|_, _| 0.0).fill(true);
        c.work(&m, &hd, 12);
        c.clock_hand_min();
        c
    }

    /// Mean OKLab distance of what the eye sees in `read` from the bare
    /// ground there.
    fn tint(c: &Canvas, ground: &Canvas, read: &Mask) -> f32 {
        let (a, g) = (c.seen(), ground.seen());
        let ins: Vec<usize> = (0..a.len()).filter(|&i| inside(c, read, i)).collect();
        let d = |i: usize| {
            let (p, q) = (crate::color::to_oklab(a[i]), crate::color::to_oklab(g[i]));
            ((p[0] - q[0]).powi(2) + (p[1] - q[1]).powi(2) + (p[2] - q[2]).powi(2)).sqrt()
        };
        ins.iter().map(|&i| d(i)).sum::<f32>() / ins.len() as f32
    }

    /// The rag over the patch by hand, `passes` times, stroke by stroke:
    /// the painter turns a cleaner face out whenever the one in use is half
    /// loaded, and with `dip` dips it in spirits as it is turned out (and at
    /// the start).
    fn by_hand(c: &mut Canvas, wipe: &Mask, passes: u32, pressure: f32, dip: Option<f32>) {
        let mut r = Rag::new(PAD_MM / c.mm_per_unit(), 3);
        let mut rng = Rng::new(31);
        if let Some(d) = dip {
            r.dip(d, c.now_min(), &mut c.tally);
        }
        for pass in 0..passes {
            let plan = plan_region(wipe, r.width, 0.0, (pass % 2) as f32 * 0.5, &mut rng);
            for (k, pts) in plan.iter().enumerate() {
                if r.load > 0.5 {
                    r.refold(&mut c.tally);
                    if let Some(d) = dip {
                        r.dip(d, c.now_min(), &mut c.tally);
                    }
                }
                c.rag_wipe(&mut r, pts, &[pressure], 500 + ((pass as u64) << 16) + k as u64);
            }
        }
    }

    /// What painters describe of a wipe-out (R. Palesca, "How to create a
    /// wipe-out underpainting in oil"; S. Downing-White, handout 3, the
    /// rub-out method): a clean dry rag over the toning layer "should wipe
    /// away the paint and leave a pale tint on the canvas"; a rag dipped in
    /// spirits wipes the lights out nearly to the ground. The thresholds are
    /// a reading of those words. Dry, three times over by hand: more than
    /// half of the tone's color goes, but a tint stays (a fifth of it at
    /// least, well above a just-noticeable difference). With spirits, three
    /// times over: nearly all the film goes, from the tops of the weave and
    /// from the hollows (90% of each), more than a dry rag takes from the
    /// hollows, and a faint stain stays. (What the eye sees doesn't fall as
    /// far: the stain the cloth can't take is the same, dry or damp; see
    /// notes/rag/README.md.)
    #[test]
    fn a_dry_rag_leaves_a_pale_tint_and_spirits_lift_nearly_to_the_ground() {
        let c0 = toned();
        let (wipe, read) = patch(&c0);
        let ground = blank(LIVE);
        let tone = tint(&c0, &ground, &read);
        let before = paint_at(&c0, &ground.film);
        let mut dry = c0.clone();
        by_hand(&mut dry, &wipe, 3, 0.8, None);
        let left_dry = tint(&dry, &ground, &read);
        let after = paint_at(&dry, &ground.film);
        let dry_film = shares(&mut dry, &before, &after, &read);
        let mut wet = c0.clone();
        by_hand(&mut wet, &wipe, 3, 0.8, Some(0.5));
        let left_wet = tint(&wet, &ground, &read);
        let after = paint_at(&wet, &ground.film);
        let wet_film = shares(&mut wet, &before, &after, &read);
        println!("tone {tone:.3}; dry rag: tint {left_dry:.3} ({:.0}% of the tone), film off tops {:.0}% hollows {:.0}%; with spirits: tint {left_wet:.3} ({:.0}%), film off tops {:.0}% hollows {:.0}%",
            100.0 * left_dry / tone, 100.0 * dry_film.0, 100.0 * dry_film.1, 100.0 * left_wet / tone, 100.0 * wet_film.0, 100.0 * wet_film.1);
        assert!(left_dry < 0.5 * tone, "a dry rag lifts the tone: {left_dry} of {tone}");
        assert!(left_dry > 0.2 * tone && left_dry > 0.02, "a dry rag leaves a pale tint: {left_dry} of {tone}");
        // (engine 3: spirits work gradually, `DAMP_LIFT3`)
        assert!(wet_film.0 > 0.9 && wet_film.1 > 0.9, "spirits lift nearly all the film: {wet_film:?}");
        assert!(dry_film.1 < wet_film.1, "a dry rag leaves more in the hollows: {dry_film:?} vs {wet_film:?}");
        assert!(wet_film.1 < 1.0 && left_wet > 0.0, "a faint stain stays");
        assert!(left_wet < left_dry, "spirits leave less color than a dry rag: {left_wet} vs {left_dry}");
    }

    /// Thin films (rag work, step 1; HANDOVER 6.1 (1)).
    mod thin {
        use super::*;
        use crate::Paint;
        use crate::palette::Palette;

        /// The probe's canvas: 480 px, 440 mm, engine 3, a smooth or a
        /// linen canvas with a thin ground.
        pub fn ground(linen: bool) -> Canvas {
            let mut c = Canvas::new(480, 1.5, hex("#b08060")).with_engine(3).with_size_mm(440.0);
            if linen {
                c = c.with_linen(Linen { warp_per_cm: 15.0, weft_per_cm: 15.0, ..Linen::fine(3) });
            }
            c.prime(hex("#e4dcc8"), 0.9, 25.0, 0.6, if linen { 0.3 } else { 0.0 }, 5);
            c
        }

        pub fn sienna() -> Paint {
            let pal = Palette::named_box("inness").unwrap();
            let name = std::env::var("RAG_EXP_TUBE").unwrap_or_else(|_| "raw sienna".into());
            let i = pal.tubes.iter().position(|t| t.name == name).unwrap();
            pal.pile(vec![(i, 1.0)]).laid(0.0)
        }

        /// The band painted (units), and the band read: the middle of the
        /// wipe, which the pad's full width reaches.
        pub const PAINTED: (f32, f32, f32, f32) = (150.0, 240.0, 850.0, 440.0);
        pub const READ: (f32, f32, f32, f32) = (330.0, 320.0, 670.0, 360.0);
        pub const LINE: [(f32, f32); 2] = [(250.0, 340.0), (750.0, 340.0)];

        /// Buffer pixels whose centers lie in `r`.
        pub fn px_in(c: &Canvas, r: (f32, f32, f32, f32)) -> Vec<usize> {
            let f = c.f;
            (0..f.w * f.h).filter(|&i| {
                let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
                x >= r.0 && x < r.2 && y >= r.1 && y < r.3
            }).collect()
        }

        /// Paint the band with heavy overlapping strokes of `paint`.
        pub fn brushed_in(c: &mut Canvas, paint: Paint, load: f32, r: (f32, f32, f32, f32)) {
            let rows = ((r.3 - r.1 - 20.0) / 22.5).round() as u64 + 1;
            for k in 0..rows {
                let y = r.1 + 10.0 + 22.5 * k as f32;
                let mut h = Held::new(Tool::hog_flat(40.0), 40 + k);
                h.load(paint, load);
                c.drag(&mut h, &Gesture::new(vec![(r.0, y), (r.2, y + 3.0)]).pressure(0.85, 0.85), None);
            }
        }

        /// A direct fixture: the band painted, then every pixel in it set to
        /// exactly `um` µm of fresh, unthinned paint (the brush only supplies
        /// the color), so neither a ridge nor a lateral pile hides the film's
        /// thickness.
        pub fn film(linen: bool, um: f32) -> Canvas {
            let mut c = ground(linen);
            set_film(&mut c, um, PAINTED);
            c
        }

        /// `film`'s fixture on canvas `c`, over the rectangle `r`.
        pub fn set_film(c: &mut Canvas, um: f32, r: (f32, f32, f32, f32)) {
            brushed_in(c, sienna(), 0.9, r);
            let lat = sienna().latent();
            let mid = ((r.0 + r.2) / 2.0, (r.1 + r.3) / 2.0);
            let hide = c.wet.hide[px_in(c, (mid.0, mid.1, mid.0 + 2.0, mid.1 + 2.0))[0]];
            let n = c.wet.vol.len();
            if c.wet.clock.px.len() != n {
                c.wait(0.0);
            }
            for i in px_in(c, r) {
                c.wet.vol[i] = um / COAT_UM;
                c.wet.lat[i] = lat;
                c.wet.hide[i] = hide;
                c.wet.cover[i] = 1.0;
                c.wet.clock.px[i].cure = 0.0;
                if let Some(s) = c.wet.solv.get_mut(i) {
                    *s = 0.0;
                }
            }
        }

        /// µm of open paint per pixel of `idx`.
        pub fn film_um(c: &Canvas, idx: &[usize]) -> Vec<f32> {
            idx.iter().map(|&i| c.wet.vol[i].max(0.0) * COAT_UM).collect()
        }

        pub fn total(v: &[f32]) -> f64 {
            v.iter().map(|&x| x as f64).sum()
        }

        /// The paint on the canvas and in the rag (mm³).
        pub fn on_canvas(c: &Canvas) -> f64 {
            c.wet.vol.iter().map(|&v| v.max(0.0) as f64).sum::<f64>() * (c.px_mm() as f64).powi(2) * COAT_UM as f64 / 1000.0
        }

        /// `wipes` wipes along `LINE` with one face, dry or dipped at 0.5.
        pub fn wiped(c: &mut Canvas, damp: bool, wipes: u32) -> (Rag, f64) {
            let mut r = Rag::new(100.0, 7);
            if damp {
                r.dip(0.5, c.now_min(), &mut c.tally);
            }
            let mut lifted = 0.0;
            for k in 0..wipes {
                lifted += c.rag_wipe(&mut r, &LINE, &[0.8], 19 + k as u64);
            }
            (r, lifted)
        }

        /// The table behind step 1: share of a direct film removed by one and
        /// two wipes, dry and damp, on the smooth and the linen ground, with
        /// the thinnest film left and the material balance.
        /// `cargo test --release -p paint --lib rag::tests::thin::table -- --ignored --nocapture`
        #[test]
        #[ignore]
        fn table() {
            for linen in [false, true] {
                println!("--- {} ground", if linen { "linen" } else { "smooth" });
                for um in [0.25f32, 0.5, 1.0, 3.0, 10.0, 69.0] {
                    let c0 = film(linen, um);
                    let idx = px_in(&c0, READ);
                    let a = film_um(&c0, &idx);
                    print!("{um:>5} µm:");
                    for (damp, wipes) in [(false, 1), (false, 2), (true, 1), (true, 2)] {
                        let mut c = c0.clone();
                        let before = on_canvas(&c);
                        let (_, lifted) = wiped(&mut c, damp, wipes);
                        let b = film_um(&c, &idx);
                        let off = 1.0 - total(&b) / total(&a);
                        let min = b.iter().copied().fold(f32::MAX, f32::min);
                        let bal = (before - on_canvas(&c) - lifted).abs() / before;
                        print!("  {}{}: {:5.1}% off, min left {:.2} µm (bal {:.0e})", if damp { "damp" } else { "dry" }, wipes, 100.0 * off, min, bal);
                    }
                    println!();
                }
            }
        }
    }

    /// Path sampling and transfer receipts (AGENT_BRIEF_V2 §2-3). The
    /// probes are diagnostics (`--ignored`); they print, they don't judge.
    /// `cargo test --release -p paint --lib rag::tests::path -- --ignored --nocapture --test-threads 1`
    mod path {
        use super::*;
        use super::thin::{PAINTED, READ, film_um, on_canvas, px_in, set_film, total};

        /// `thin::ground` at engine `e`, `w` px wide.
        pub fn ground_e(linen: bool, e: u32, w: usize) -> Canvas {
            let mut c = Canvas::new(w, 1.5, hex("#b08060")).with_engine(e).with_size_mm(440.0);
            if linen {
                c = c.with_linen(Linen { warp_per_cm: 15.0, weft_per_cm: 15.0, ..Linen::fine(3) });
            }
            c.prime(hex("#e4dcc8"), 0.9, 25.0, 0.6, if linen { 0.3 } else { 0.0 }, 5);
            c
        }

        pub fn fixture(linen: bool, e: u32, w: usize, um: f32) -> Canvas {
            let mut c = ground_e(linen, e, w);
            set_film(&mut c, um, PAINTED);
            c
        }

        /// FNV-1a-64 over the canvas fields a rag can change and the rag.
        pub fn fnv(c: &Canvas, r: &Rag) -> u64 {
            let mut h = 0xcbf2_9ce4_8422_2325u64;
            let mut eat = |b: &[u8]| {
                for &x in b {
                    h ^= x as u64;
                    h = h.wrapping_mul(0x0100_0000_01b3);
                }
            };
            for i in 0..c.wet.vol.len() {
                eat(&c.wet.vol[i].to_bits().to_le_bytes());
                eat(&c.film[i].to_bits().to_le_bytes());
                eat(&c.height[i].to_bits().to_le_bytes());
                eat(&c.wet.cover[i].to_bits().to_le_bytes());
                for q in c.wet.lat[i] {
                    eat(&q.to_bits().to_le_bytes());
                }
                for q in c.wet.hide[i] {
                    eat(&q.to_bits().to_le_bytes());
                }
            }
            for s in &c.wet.solv {
                eat(&s.to_bits().to_le_bytes());
            }
            for p in &c.wet.clock.px {
                eat(&p.cure.to_bits().to_le_bytes());
            }
            eat(format!("{r:?}").as_bytes());
            h
        }

        /// `pts` with every segment cut into `n` equal collinear pieces.
        pub fn dense(pts: &[(f32, f32)], n: usize) -> Vec<(f32, f32)> {
            let mut out = vec![pts[0]];
            for w in pts.windows(2) {
                for k in 1..=n {
                    let t = k as f32 / n as f32;
                    out.push((w[0].0 + (w[1].0 - w[0].0) * t, w[0].1 + (w[1].1 - w[0].1) * t));
                }
            }
            out
        }

        /// `pts` with `n` collinear points inserted at random places along
        /// each segment (uneven pieces, some far shorter than a step).
        pub fn uneven(pts: &[(f32, f32)], n: usize, seed: u64) -> Vec<(f32, f32)> {
            let mut rng = Rng::new(seed);
            let mut out = vec![pts[0]];
            for w in pts.windows(2) {
                let mut ts: Vec<f32> = (0..n).map(|_| rng.range(0.0, 1.0)).collect();
                ts.sort_by(f32::total_cmp);
                for t in ts.into_iter().chain([1.0]) {
                    out.push((w[0].0 + (w[1].0 - w[0].0) * t, w[0].1 + (w[1].1 - w[0].1) * t));
                }
            }
            out
        }

        pub const REVERSAL: [(f32, f32); 3] = [(250.0, 340.0), (750.0, 340.0), (300.0, 340.0)];
        pub const CORNER: [(f32, f32); 3] = [(250.0, 300.0), (620.0, 300.0), (620.0, 430.0)];

        /// The actions the digest replays: (name, damp, path or None for a
        /// blot at the band's middle).
        pub fn actions() -> Vec<(&'static str, bool, Option<Vec<(f32, f32)>>)> {
            vec![
                ("line2", false, Some(thin::LINE.to_vec())),
                ("line51", false, Some(dense(&thin::LINE, 50))),
                ("damp2", true, Some(thin::LINE.to_vec())),
                ("reversal", false, Some(REVERSAL.to_vec())),
                ("corner", false, Some(CORNER.to_vec())),
                ("blot", false, None),
            ]
        }

        /// A digest of each action on a 3 µm direct film at engines 1, 2 and
        /// 3, and of a region pass with refolds: engine 1 and 2 must print
        /// the same before and after a change to engine 3's rag.
        #[test]
        #[ignore]
        fn digest() {
            let w: usize = std::env::var("RAG_W").ok().and_then(|v| v.parse().ok()).unwrap_or(480);
            for e in 1..=3u32 {
                let c0 = fixture(true, e, w, 3.0);
                for (name, damp, pts) in actions() {
                    let mut c = c0.clone();
                    let mut r = Rag::new(100.0, 7);
                    if damp {
                        r.dip(0.5, c.now_min(), &mut c.tally);
                    }
                    let lifted = match &pts {
                        Some(p) => c.rag_wipe(&mut r, p, &[0.8], 19),
                        None => c.rag_blot(&mut r, 500.0, 340.0, 0.8, 19),
                    };
                    println!("engine {e} {name:>9}: {:016x} lifted {lifted:.6e} mm3 load {:.6}", fnv(&c, &r), r.load);
                }
                let mut c = c0.clone();
                let mut r = Rag::new(100.0, 7);
                let m = Mask::from_fn(c.frame(), |x, y| if (200.0..800.0).contains(&x) && (250.0..430.0).contains(&y) { 1.0 } else { 0.0 });
                c.rag_region(&mut r, &m, &RagPass { pressure: 0.7, angle: 0.1, passes: 2, refold: Some(0.3), seed: 3 });
                println!("engine {e}    region: {:016x} load {:.6} fold {}", fnv(&c, &r), r.load, r.fold);
            }
        }

        /// Largest and mean |difference| (µm) of open paint between two
        /// canvases, and the larger total.
        pub fn delta(a: &Canvas, b: &Canvas) -> (f32, f64) {
            let (mut mx, mut s) = (0.0f32, 0.0f64);
            for i in 0..a.wet.vol.len() {
                let d = ((a.wet.vol[i] - b.wet.vol[i]) * COAT_UM).abs();
                mx = mx.max(d);
                s += d as f64;
            }
            (mx, s / a.wet.vol.len() as f64)
        }

        /// The same continuous motion given with 2 points, 51 evenly spaced
        /// collinear points and 2 + 9 unevenly spaced ones, and a reversal
        /// and a corner each given plain and densely subdivided: film left
        /// in the read band, the largest per-pixel difference from the
        /// plain path, the rag's load, and the material balance (canvas
        /// change against what `rag_wipe` reports lifted).
        #[test]
        #[ignore]
        fn partition() {
            let w: usize = std::env::var("RAG_W").ok().and_then(|v| v.parse().ok()).unwrap_or(480);
            let paths: Vec<(&str, Vec<Vec<(f32, f32)>>)> = vec![
                ("line", vec![thin::LINE.to_vec(), dense(&thin::LINE, 50), uneven(&thin::LINE, 9, 5)]),
                ("reversal", vec![REVERSAL.to_vec(), dense(&REVERSAL, 25), uneven(&REVERSAL, 9, 6)]),
                ("corner", vec![CORNER.to_vec(), dense(&CORNER, 25), uneven(&CORNER, 9, 7)]),
            ];
            for linen in [false, true] {
                for um in [1.0f32, 3.0, 69.0] {
                    let c0 = fixture(linen, 3, w, um);
                    let idx = px_in(&c0, READ);
                    let a = total(&film_um(&c0, &idx));
                    for damp in [false, true] {
                        for (name, variants) in &paths {
                            let mut first: Option<Canvas> = None;
                            for (vi, pts) in variants.iter().enumerate() {
                                let mut c = c0.clone();
                                let mut r = Rag::new(100.0, 7);
                                if damp {
                                    r.dip(0.5, c.now_min(), &mut c.tally);
                                }
                                let before = on_canvas(&c);
                                let lifted = c.rag_wipe(&mut r, pts, &[0.8], 19);
                                let left = total(&film_um(&c, &idx)) / a;
                                let bal = (before - on_canvas(&c) - lifted).abs() / before;
                                let (mx, mean) = first.as_ref().map_or((0.0, 0.0), |f| delta(f, &c));
                                println!("{} {um:>4} µm {} {name:>8} {:>3} pts: left {:6.2}% max|Δ| {mx:8.4} µm mean|Δ| {mean:.2e} µm load {:.5} bal {bal:.0e}",
                                    if linen { "linen " } else { "smooth" }, if damp { "damp" } else { "dry " }, pts.len(), 100.0 * left, r.load);
                                if vi == 0 {
                                    first = Some(c);
                                }
                            }
                        }
                    }
                }
            }
        }

        /// A tube of the inness box as fresh paint.
        pub fn tube(name: &str) -> crate::Paint {
            let pal = crate::palette::Palette::named_box("inness").unwrap();
            let i = pal.tubes.iter().position(|t| t.name == name).unwrap();
            pal.pile(vec![(i, 1.0)]).laid(0.0)
        }

        /// `thin::set_film` with paint `paint`: every pixel in `r` exactly
        /// `um` µm of it, fresh and unthinned.
        pub fn set_film_of(c: &mut Canvas, um: f32, r: (f32, f32, f32, f32), paint: crate::Paint) {
            thin::brushed_in(c, paint, 0.9, r);
            let lat = paint.latent();
            let mid = ((r.0 + r.2) / 2.0, (r.1 + r.3) / 2.0);
            let hide = c.wet.hide[px_in(c, (mid.0, mid.1, mid.0 + 2.0, mid.1 + 2.0))[0]];
            if c.wet.clock.px.len() != c.wet.vol.len() {
                c.wait(0.0);
            }
            for i in px_in(c, r) {
                c.wet.vol[i] = um / COAT_UM;
                c.wet.lat[i] = lat;
                c.wet.hide[i] = hide;
                c.wet.cover[i] = 1.0;
                c.wet.clock.px[i].cure = 0.0;
                if let Some(s) = c.wet.solv.get_mut(i) {
                    *s = 0.0;
                }
            }
        }

        /// The face's capacity (mm³), as `rag_contact` computes it.
        pub fn cap(c: &Canvas, r: &Rag) -> f64 {
            let w = (r.width * c.mm_per_unit()) as f64;
            w * w * CAP_UM as f64 / 1000.0
        }

        /// Bounded pickup: one face, never refolded, wiped back and forth
        /// over a thick film until it is full: the face's load against what
        /// the canvas has lost to it (the ledger, in faces), dry and dipped
        /// at 1.0. A load stuck at 1 while the ledger passes it is overfill
        /// the face can't hold.
        #[test]
        #[ignore]
        fn capacity() {
            let w: usize = std::env::var("RAG_W").ok().and_then(|v| v.parse().ok()).unwrap_or(480);
            for um in [69.0f32, 300.0] {
                for damp in [0.0f32, 1.0] {
                    let c0 = fixture(false, 3, w, um);
                    let mut c = c0.clone();
                    let mut r = Rag::new(100.0, 7);
                    let cap = cap(&c, &r);
                    let start = on_canvas(&c);
                    print!("{um:>5} µm damp {damp}:");
                    for k in 0..12u64 {
                        if damp > 0.0 {
                            r.dip(damp, c.now_min(), &mut c.tally);
                        }
                        let pts = if k % 2 == 0 { thin::LINE.to_vec() } else { vec![thin::LINE[1], thin::LINE[0]] };
                        c.rag_wipe(&mut r, &pts, &[0.9], 40 + k);
                        if k % 3 == 2 {
                            print!("  w{}: load {:.6} ledger {:.6}", k + 1, r.load, (start - on_canvas(&c)) / cap);
                        }
                    }
                    println!("  soaked×FACES {:.6}", r.soaked * FACES);
                }
            }
        }

        /// Share of paint `a` (latent `la`) in a pixel of mixed `a` and `b`
        /// paint, from its latent vector: mixing is linear by volume in
        /// `rag_lay` and `bristle`, so the latent works as a tracer here.
        /// It is not a pigment ledger.
        pub fn share_of(l: &crate::wet::Latent, la: &crate::wet::Latent, lb: &crate::wet::Latent) -> f32 {
            let (mut num, mut den) = (0.0f32, 0.0f32);
            for q in 0..l.len() {
                num += (l[q] - lb[q]) * (la[q] - lb[q]);
                den += (la[q] - lb[q]).powi(2);
            }
            (num / den.max(1e-12)).clamp(0.0, 1.0)
        }

        /// Dirty two-color carryover and refolding on direct 3 µm films:
        /// raw umber left of x = 500, lead white right of it. (a) one wipe
        /// from the umber into the white; (b) the same face wiping the umber,
        /// lifted, then wiping the white; (c) as (b) with a refold between;
        /// (d) a clean face over the white alone. Umber laid on the white
        /// (mm³, by the latent tracer), the white's film change and the
        /// balance.
        #[test]
        #[ignore]
        fn carry() {
            let w: usize = std::env::var("RAG_W").ok().and_then(|v| v.parse().ok()).unwrap_or(480);
            for linen in [false, true] {
                let mut c0 = ground_e(linen, 3, w);
                let (ua, wb) = (tube("raw umber"), tube("lead white"));
                set_film_of(&mut c0, 3.0, (150.0, 240.0, 500.0, 440.0), ua);
                set_film_of(&mut c0, 3.0, (500.0, 240.0, 850.0, 440.0), wb);
                let (la, lb) = (ua.latent(), wb.latent());
                let white = px_in(&c0, (520.0, 240.0, 850.0, 440.0));
                let mm3 = (c0.px_mm() as f64).powi(2) * COAT_UM as f64 / 1000.0;
                let umber_on_white = |c: &Canvas| white.iter().map(|&i| (c.wet.vol[i].max(0.0) * share_of(&c.wet.lat[i], &la, &lb)) as f64).sum::<f64>() * mm3;
                let white_film = |c: &Canvas| white.iter().map(|&i| c.wet.vol[i].max(0.0) as f64).sum::<f64>() * mm3;
                let (u0, f0) = (umber_on_white(&c0), white_film(&c0));
                for case in ["a: umber->white, one wipe", "b: umber, lift, white", "c: umber, refold, white", "d: clean face, white only"] {
                    let mut c = c0.clone();
                    let mut r = Rag::new(100.0, 7);
                    let before = on_canvas(&c);
                    let mut lifted = 0.0;
                    match &case[..1] {
                        "a" => lifted += c.rag_wipe(&mut r, &[(250.0, 340.0), (750.0, 340.0)], &[0.8], 3),
                        "d" => lifted += c.rag_wipe(&mut r, &[(560.0, 340.0), (750.0, 340.0)], &[0.8], 4),
                        k => {
                            lifted += c.rag_wipe(&mut r, &[(250.0, 340.0), (480.0, 340.0)], &[0.8], 3);
                            if k == "c" {
                                r.refold(&mut c.tally);
                            }
                            lifted += c.rag_wipe(&mut r, &[(560.0, 340.0), (750.0, 340.0)], &[0.8], 4);
                        }
                    }
                    let bal = (before - on_canvas(&c) - lifted).abs() / before;
                    println!("{} {case:<28}: umber on white {:+.4} mm3, white region film {:+.3} mm3, load {:.4}, bal {bal:.0e}",
                        if linen { "linen " } else { "smooth" }, umber_on_white(&c) - u0, white_film(&c) - f0, r.load);
                }
            }
        }

        /// A blot on a 3 µm film: what it lifts, and the balance.
        #[test]
        #[ignore]
        fn blot() {
            let w: usize = std::env::var("RAG_W").ok().and_then(|v| v.parse().ok()).unwrap_or(480);
            for linen in [false, true] {
                for um in [0.25f32, 1.0, 3.0, 69.0] {
                    let c0 = fixture(linen, 3, w, um);
                    let mut c = c0.clone();
                    let mut r = Rag::new(100.0, 7);
                    let before = on_canvas(&c);
                    let t0 = c.tally.secs;
                    let lifted = c.rag_blot(&mut r, 500.0, 340.0, 0.8, 9);
                    let bal = (before - on_canvas(&c) - lifted).abs() / before;
                    println!("{} {um:>5} µm blot: lifted {lifted:.4} mm3 ({:.2}% of the film), load {:.5}, hand {:.2} s, bal {bal:.0e}",
                        if linen { "linen " } else { "smooth" }, 100.0 * lifted / before, r.load, c.tally.secs - t0);
                }
            }
        }

        /// A dry underlayer under a fresh 1 µm film: a hard damp wipe and a
        /// blot change only the wet layer; the set paint (`film`) and the
        /// surface (`height`) stay bit for bit.
        #[test]
        #[ignore]
        fn underlayer() {
            let w: usize = std::env::var("RAG_W").ok().and_then(|v| v.parse().ok()).unwrap_or(480);
            for linen in [false, true] {
                let mut c = ground_e(linen, 3, w);
                set_film_of(&mut c, 20.0, PAINTED, tube("raw umber"));
                c.dry();
                set_film_of(&mut c, 1.0, PAINTED, tube("raw sienna"));
                let (film0, height0) = (c.film.clone(), c.height.clone());
                let idx = px_in(&c, READ);
                let a = total(&film_um(&c, &idx));
                let mut r = Rag::new(100.0, 7);
                r.dip(1.0, c.now_min(), &mut c.tally);
                for k in 0..3 {
                    c.rag_wipe(&mut r, &thin::LINE, &[1.0], 60 + k);
                }
                c.rag_blot(&mut r, 500.0, 340.0, 1.0, 9);
                let same = c.film.iter().zip(&film0).all(|(a, b)| a.to_bits() == b.to_bits()) && c.height.iter().zip(&height0).all(|(a, b)| a.to_bits() == b.to_bits());
                let neg = c.wet.vol.iter().filter(|&&v| v < 0.0).count();
                println!("{} dry 20 µm under fresh 1 µm: film+height unchanged {same}, wet film left {:.2}%, negative wet pixels {neg}",
                    if linen { "linen " } else { "smooth" }, 100.0 * total(&film_um(&c, &idx)) / a);
            }
        }
    }
}
