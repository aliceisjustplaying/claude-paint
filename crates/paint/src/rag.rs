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
//! With the rag lift switch (`Canvas::with_rag_lift`, engine 7) the face
//! keeps all it lifts: nothing is smeared back, so a loaded face lifts less
//! and lays nothing down. A wipe over a mask lifts only inside the mask, by
//! the mask's value, wherever the pad's rim reaches.
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
/// A face loaded this far is full: it lifts almost nothing more (its
/// thirst is under a tenth and its room a twentieth of a clean face's).
/// With the rag lift switch the easel says so when a wipe starts on one.
pub const FULL: f32 = 0.95;

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

/// The paint still wet at the surface of the face in use (engine 3): what
/// the face can smear back (`SMEAR`), until it soaks into the cloth
/// (`SOAK`, with time, on the canvas or off it). It is part of `Rag::load`
/// already. mm³ of paint, its mean color, hiding and cure, mm³ of solvent,
/// and when it was last brought up to date: minutes of painting time
/// (`Canvas::now_min`) and hand seconds (`Tally::secs`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mobile {
    pub mm3: f32,
    pub lat: crate::wet::Latent,
    pub hide: crate::wet::Prop,
    pub cure: f32,
    pub solv_mm3: f32,
    pub at_min: f64,
    pub at_secs: f64,
}

/// One contact of the pad (`rag_contact`): how long it lasts (s) and how
/// far the pad slides over the canvas meanwhile (mm). A wipe's step slides
/// its own length at `pace::RAG_MM_S`; a blot presses for
/// `pace::BLOT_PRESS` and doesn't slide. Engine 3: the paint at the cloth's
/// surface soaks in with the time (`SOAK`), the face smears it back with
/// the slide (`SMEAR`).
#[derive(Clone, Copy, Debug)]
struct Touch {
    secs: f32,
    slip_mm: f32,
}

/// A straight piece of a wipe's path (engine 3, `rag_wipe3`): where it
/// starts (units), its direction, length, how far along the whole path it
/// starts, and the pad's sideways direction along it (`side`): the left
/// of the motion, kept from the piece before through a turn sharper than a
/// right angle, so a scrub back and forth doesn't turn the pad around.
#[derive(Clone, Copy, Debug)]
struct Seg {
    a: (f32, f32),
    t: (f32, f32),
    l: f32,
    s: f32,
    side: (f32, f32),
}

/// A rag in the hand: its pad width (units), how loaded the face in use is
/// (0 clean .. 1 full), how much the whole cloth has soaked up (0 .. 1, all
/// `FACES` faces full), how damp with spirits the face in use is (0 dry ..
/// 1 dipped well, as of `wet_at`; refolding preserves this dampness, and
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
    /// Engine 3: the paint wet at the face's surface, carried from one
    /// wipe or blot to the next until it soaks in; a refold turns out a
    /// face without any.
    pub mobile: Mobile,
}

impl Rag {
    /// A clean rag bunched to a pad `width` units across.
    pub fn new(width: f32, seed: u64) -> Self {
        Rag { width: width.max(0.1), load: 0.0, soaked: 0.0, damp: 0.0, wet_at: 0.0, fold: 0, seed, solvent_mm3: 0.0, mobile: Mobile::default() }
    }

    /// Turn a cleaner face outward, retaining the cloth's dampness.
    /// No face is cleaner than the paint
    /// soaked through the whole cloth so far (`soaked`) leaves it. Counts
    /// the hand time in `t`.
    pub fn refold(&mut self, t: &mut Tally) {
        self.fold = self.fold.wrapping_add(1);
        self.load = self.soaked.clamp(0.0, 1.0);
        // (what was wet at the old face's surface is folded inside, in
        // `soaked`)
        self.mobile = Mobile::default();
        t.secs += pace::REFOLD;
    }

    /// Dip the face in use into spirits, the reach starting at `now_min`
    /// (`Canvas::now_min`): `amount` 0..1 (a light dip about 0.5). It is
    /// that damp when the hand is back (`pace::DIP` later), and stays damp
    /// until the spirits evaporate (`evaporate`). Counts
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

/// Engine 3: how much of the pad passes over the point (`x`, `y`) in one
/// step made of the straight `pieces` (`rag_wipe3`, drift included), with
/// the cloth's contact, how lightly its rim presses (`SMEAR`) and its
/// pressure toward the rim (`RIM_PRESS`), each averaged over the pieces by
/// how much of the pad they pass over it. A pad of radius `r`, the whole
/// path `len` long, pressure `p`.
#[allow(clippy::too_many_arguments)]
fn pad3(pieces: &[Seg], x: f32, y: f32, r: f32, len: f32, p: f32, mmu: f32, cs: u64) -> (f32, f32, f32, f32) {
    let (mut pad, mut cl, mut rim, mut press) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
    for q in pieces {
        let (qx, qy) = (x - q.a.0, y - q.a.1);
        let sp = qx * q.t.0 + qy * q.t.1;
        let d = qx * q.side.0 + qy * q.side.1;
        let ad = d.abs();
        if ad >= r {
            continue;
        }
        let c = (r * r - d * d).sqrt();
        let over = ((sp + c).min(q.l) - (sp - c).max(0.0)).max(0.0);
        if over <= 0.0 {
            continue;
        }
        // frayed sides, and ends where the cloth comes down and lifts off
        // unevenly across the pad
        let sg = q.s + sp;
        let fray = 0.6 * vn(sg * mmu / FRAY_MM, if d > 0.0 { 3.1 } else { 7.3 }, cs ^ 0xD4) + 0.4 * vn(sg * mmu * 3.0 / FRAY_MM, if d > 0.0 { 5.2 } else { 9.4 }, cs ^ 0xD5);
        let re = r * (1.0 - FRAY * fray);
        let start = ((sg + r * (1.2 * vn(d * mmu / END_MM, 13.7, cs ^ 0xE6) - 0.4)) / (0.25 * r)).clamp(0.0, 1.0);
        let end = ((len + r * (1.2 * vn(d * mmu / END_MM, 11.3, cs ^ 0xE5) - 0.4) - sg) / (0.25 * r)).clamp(0.0, 1.0);
        // the light-pressed rim: the frayed sides and the trailing end
        let rm = smoothstep(0.4 * re, re, ad).max(smoothstep(len - r, len + 0.5 * r, sg));
        // the cloth's outline, and its pressure falling off toward the
        // rim, where it reaches only the tops of the weave
        let edge = 1.0 - smoothstep(RIM_EDGE * re, re, ad);
        let pr = 1.0 - smoothstep(RIM_PRESS * re, re, ad);
        let e = over / (2.0 * r) * edge * start * end;
        if e <= 0.0 {
            continue;
        }
        pad += e;
        cl += e * cloth3(d * mmu, sg * mmu, cs, p);
        rim += e * rm;
        press += e * pr;
    }
    if pad <= 0.0 {
        return (0.0, 0.0, 0.0, 0.0);
    }
    (pad, cl / pad, rim / pad, press / pad)
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
    fn rag_contact(&mut self, rag: &mut Rag, bbox: (f32, f32, f32, f32), pressure: f32, touch: Touch, mut pool: Option<&mut Pool>, expo: impl Fn(f32, f32) -> (f32, f32, f32, f32)) -> f64 {
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
        // what each pixel gives, row by row, before any of it is taken (a
        // pixel's share depends only on its own film and on the surface as
        // the contact began)
        let mut asks: Vec<(usize, f32)> = Vec::new();
        let mut rows: Vec<usize> = Vec::with_capacity(y1 - y0);
        for y in y0..y1 {
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
                    asks.push((i, take));
                }
            }
            rows.push(asks.len());
        }
        // engine 3: the face takes no more than it has room for (`CAP_UM`);
        // what it can't hold stays on the canvas, every pixel's share cut
        // alike
        let unit = (px_mm as f64).powi(2) * (COAT_UM as f64 / 1000.0);
        let w_mm = (rag.width * self.mm_per_unit) as f64;
        let cap = w_mm * w_mm * (CAP_UM as f64 / 1000.0);
        let scale = if e3 {
            let want = asks.iter().map(|a| a.1 as f64).sum::<f64>() * unit;
            let room = (1.0 - rag.load as f64).max(0.0) * cap;
            if want > room { (room / want) as f32 } else { 1.0 }
        } else {
            1.0
        };
        let mut k0 = 0;
        for &k1 in &rows {
            let mut row = 0.0f32;
            let mut row_s = 0.0f32;
            for &(i, take) in &asks[k0..k1] {
                let take = take * scale;
                if take > 0.0 {
                    if pooled {
                        let l = &self.wet.lat[i];
                        for q in 0..l.len() {
                            got.lat[q] += l[q] * take;
                        }
                        let h = &self.wet.hide[i];
                        for q in 0..h.len() {
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
            k0 = k1;
        }
        if let Some(pl) = pool.as_deref_mut() {
            let (laid, laid_s) = self.rag_smear(pl, &got, (x0, y0, x1, y1), touch, 0.25 * w_mm as f32, &expo);
            lifted -= laid;
            lifted_s -= laid_s;
        }
        let mm3 = lifted * unit;
        rag.solvent_mm3 += lifted_s * unit;
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
    ///
    /// `SMEAR` and `SOAK` are the shares for a step a quarter of the pad's
    /// width long (`quarter_mm`) at `pace::RAG_MM_S`: a contact smears by
    /// how far it slides and soaks by how long it lasts (`touch`), so a
    /// shorter step does less of each.
    fn rag_smear(&mut self, pl: &mut Pool, got: &Pool, px: (usize, usize, usize, usize), touch: Touch, quarter_mm: f32, expo: &impl Fn(f32, f32) -> (f32, f32, f32, f32)) -> (f64, f64) {
        let f = self.f;
        let (x0, y0, x1, y1) = px;
        let quarter = quarter_mm.max(1e-6);
        let smear = 1.0 - (1.0 - SMEAR).powf(touch.slip_mm / quarter);
        let left = (1.0 - SOAK).powf(touch.secs * pace::RAG_MM_S as f32 / quarter);
        let mut out = 0.0f32;
        if pl.vol > 1e-9 {
            let rate = smear * pl.vol / (((x1 - x0) * (y1 - y0)) as f32).max(1.0);
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
        let v0 = pl.vol.max(0.0) * left;
        let t = v0 + got.vol;
        if t > 0.0 {
            for q in 0..pl.lat.len() {
                pl.lat[q] = (pl.lat[q] * v0 + got.lat[q]) / t;
            }
            for q in 0..pl.hide.len() {
                pl.hide[q] = (pl.hide[q] * v0 + got.hide[q]) / t;
            }
            pl.cure = (pl.cure * v0 + got.cure) / t;
        }
        pl.vol = t;
        pl.solv = pl.solv.max(0.0) * left + got.solv;
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
        for k in 0..hd.len() {
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
        self.rag_wipe_in(rag, pts, pressure, seed, None)
    }

    /// `rag_wipe`, lifting only inside `clip` (by its value) when it is given.
    fn rag_wipe_in(&mut self, rag: &mut Rag, pts: &[(f32, f32)], pressure: &[f32], seed: u64, clip: Option<&Mask>) -> f64 {
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
        if self.engine >= 3 {
            return self.rag_wipe3(rag, pts, &pr, cs, clip);
        }
        // before engine 3: steps of each segment in turn
        let w = rag.width;
        let r0 = 0.5 * w;
        let step = 0.5 * r0;
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
                let (r, off) = (r0 * (1.0 + 0.12 * wob), 0.06 * w * side);
                let p = pr(sm / len);
                let (ax, ay) = (a.0 + tx * l0, a.1 + ty * l0);
                let sl = l1 - l0;
                // (the box isn't shifted with the pad: as these engines did)
                let bbox = (ax.min(ax + tx * sl) - r - 1.0, ay.min(ay + ty * sl) - r - 1.0, ax.max(ax + tx * sl) + r + 1.0, ay.max(ay + ty * sl) + r + 1.0);
                let s0 = s_at + l0;
                let touch = Touch { secs: sl * mmu / pace::RAG_MM_S as f32, slip_mm: sl * mmu };
                total += self.rag_contact(rag, bbox, p, touch, None, |x, y| {
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
                    let edge = 1.0 - smoothstep(0.55 * r, r, ad);
                    (over / (2.0 * r) * edge, cloth(d * mmu, (s0 + sp) * mmu, cs), 0.0, 1.0)
                });
            }
            s_at += l;
        }
        total
    }

    /// Engine 3: the face's mobile paint as a `Pool` in this canvas's units,
    /// soaked by the time since it was last brought up to date, less
    /// `busy_s` (the hand time of the contact about to begin, which the
    /// tally already counts).
    fn rag_pool(&self, rag: &Rag, busy_s: f64) -> Pool {
        let m = rag.mobile;
        if m.mm3 <= 0.0 {
            return Pool::default();
        }
        let mut dt = (self.now_min() - m.at_min) * 60.0;
        if self.hand_slice_secs().is_none() {
            // (hand time isn't on the clock: count it here)
            dt += self.tally.secs - m.at_secs;
        }
        let dt = (dt - busy_s).max(0.0) as f32;
        let quarter = 0.25 * rag.width * self.mm_per_unit;
        let left = (1.0 - SOAK).powf(dt * pace::RAG_MM_S as f32 / quarter.max(1e-6));
        let unit = self.px_mm().powi(2) * (COAT_UM / 1000.0);
        let sunit = self.px_mm().powi(2) / 1000.0;
        Pool { vol: m.mm3 * left / unit, lat: m.lat, hide: m.hide, cure: m.cure, solv: m.solv_mm3 * left / sunit }
    }

    /// Engine 3: keep `pl` as the face's mobile paint, as of now.
    fn rag_keep(&self, rag: &mut Rag, pl: &Pool) {
        let unit = self.px_mm().powi(2) * (COAT_UM / 1000.0);
        let sunit = self.px_mm().powi(2) / 1000.0;
        let v = pl.vol.max(0.0);
        rag.mobile = if v > 0.0 {
            Mobile { mm3: v * unit, lat: pl.lat, hide: pl.hide, cure: pl.cure, solv_mm3: pl.solv.max(0.0) * sunit, at_min: self.now_min(), at_secs: self.tally.secs }
        } else {
            Mobile::default()
        };
    }

    /// Engine 3's wipe along `pts`: the pad moves along the whole path in
    /// steps of equal length (about a quarter of its width), counted over
    /// the path's length and not per segment, so extra points along a
    /// straight line change nothing, and a step may run through a corner or
    /// a reversal (its pieces, `Seg`). Each step is one hand pose: its
    /// pressure (`pr`, by the share of the path's length), the pad's width
    /// and its drift off the line (`WANDER_W`, `WANDER_OFF`), the same for
    /// what the pad touches and for the box searched. A contact lasts the
    /// step's slide at `pace::RAG_MM_S` (`Touch`). Returns the volume lifted
    /// (mm³).
    fn rag_wipe3(&mut self, rag: &mut Rag, pts: &[(f32, f32)], pr: &dyn Fn(f32) -> f32, cs: u64, clip: Option<&Mask>) -> f64 {
        let mmu = self.mm_per_unit;
        let w = rag.width;
        let r0 = 0.5 * w;
        let step = 0.5 * r0;
        let mut segs: Vec<Seg> = Vec::with_capacity(pts.len());
        let mut s = 0.0f32;
        for p in pts.windows(2) {
            let (dx, dy) = (p[1].0 - p[0].0, p[1].1 - p[0].1);
            let l = (dx * dx + dy * dy).sqrt();
            if l < 1e-6 {
                continue;
            }
            let t = (dx / l, dy / l);
            let mut side = (-t.1, t.0);
            if let Some(q) = segs.last()
                && side.0 * q.side.0 + side.1 * q.side.1 < 0.0
            {
                side = (-side.0, -side.1);
            }
            segs.push(Seg { a: p[0], t, l, s, side });
            s += l;
        }
        let len = s;
        if segs.is_empty() {
            return 0.0;
        }
        let n = (len / step).ceil().max(1.0) as usize;
        let sl = len / n as f32;
        let touch = Touch { secs: sl * mmu / pace::RAG_MM_S as f32, slip_mm: sl * mmu };
        // the face's mobile paint from before, soaked in the meantime (with
        // the rag lift switch the face keeps it all: no pool)
        let keep = self.rag_lift;
        let mut pool = if keep { Pool::default() } else { self.rag_pool(rag, (len * mmu) as f64 / pace::RAG_MM_S) };
        let mut total = 0.0;
        let mut first = 0usize;
        let mut pieces: Vec<Seg> = Vec::with_capacity(4);
        for k in 0..n {
            let s0 = len * k as f32 / n as f32;
            let s1 = if k + 1 == n { len } else { len * (k + 1) as f32 / n as f32 };
            let sm = 0.5 * (s0 + s1);
            // the pad's width and line wander as the cloth shifts in the hand
            let wob = 2.0 * vn(sm / (2.5 * w), 0.5, cs ^ 0xA1) - 1.0;
            let side = 2.0 * vn(sm / (3.0 * w), 1.5, cs ^ 0xB2) - 1.0;
            let wob2 = 2.0 * vn(sm / (0.7 * w), 2.5, cs ^ 0xA7) - 1.0;
            let side2 = 2.0 * vn(sm / (0.7 * w), 3.5, cs ^ 0xB8) - 1.0;
            let r = r0 * (1.0 + WANDER_W * (0.65 * wob + 0.35 * wob2));
            let off = WANDER_OFF * w * (0.65 * side + 0.35 * side2);
            let p = pr(sm / len);
            while first + 1 < segs.len() && segs[first].s + segs[first].l <= s0 {
                first += 1;
            }
            pieces.clear();
            for g in &segs[first..] {
                if g.s >= s1 {
                    break;
                }
                let (u0, u1) = ((s0 - g.s).max(0.0), (s1 - g.s).min(g.l));
                if u1 > u0 {
                    pieces.push(Seg { a: (g.a.0 + g.t.0 * u0, g.a.1 + g.t.1 * u0), l: u1 - u0, s: g.s + u0, ..*g });
                }
            }
            if pieces.is_empty() {
                continue;
            }
            // one drift off the line for the whole step, across the pad's
            // side averaged over its pieces
            let (mut nx, mut ny) = (0.0f32, 0.0f32);
            for q in &pieces {
                nx += q.side.0 * q.l;
                ny += q.side.1 * q.l;
            }
            let nl = (nx * nx + ny * ny).sqrt();
            let (nx, ny) = if nl > 1e-6 { (nx / nl, ny / nl) } else { pieces[0].side };
            let mut bbox = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
            for q in &mut pieces {
                q.a = (q.a.0 + off * nx, q.a.1 + off * ny);
                let (ex, ey) = (q.a.0 + q.t.0 * q.l, q.a.1 + q.t.1 * q.l);
                bbox = (bbox.0.min(q.a.0.min(ex) - r - 1.0), bbox.1.min(q.a.1.min(ey) - r - 1.0), bbox.2.max(q.a.0.max(ex) + r + 1.0), bbox.3.max(q.a.1.max(ey) + r + 1.0));
            }
            let pc = &pieces;
            total += self.rag_contact(rag, bbox, p, touch, if keep { None } else { Some(&mut pool) }, |x, y| {
                let e = pad3(pc, x, y, r, len, p, mmu, cs);
                match clip {
                    Some(m) => (e.0 * m.sample(x, y).clamp(0.0, 1.0), e.1, e.2, e.3),
                    None => e,
                }
            });
        }
        // what is still at the cloth's surface goes with the face as the
        // pad lifts off (it is in `load` already)
        if !keep {
            self.rag_keep(rag, &pool);
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
        let touch = Touch { secs: pace::BLOT_PRESS as f32, slip_mm: 0.0 };
        // engine 3: what it lifts goes with how long it is pressed; the
        // face's mobile paint doesn't smear (no slide) but soaks in while it
        // is pressed, and what it lifts joins it
        let dwell = if e3 { touch.secs / pace::BLOT_PRESS as f32 } else { 1.0 };
        let mut pool = if e3 && !self.rag_lift { Some(self.rag_pool(rag, pace::BLOT_PRESS)) } else { None };
        let got = self.rag_contact(rag, bbox, pressure, touch, pool.as_mut(), |px, py| {
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
            (BLOT * dwell * (1.0 - smoothstep(0.5 * r, r, d)), c, 0.0, 1.0)
        });
        if let Some(pl) = &pool {
            self.rag_keep(rag, pl);
        }
        got
    }

    /// Wipe a region: strokes side by side across the mask in `o.angle`'s
    /// direction, back and forth, each anchored inside the mask and stopping
    /// short of its edge by about a third of the pad (the pad's soft edge
    /// reaches it, a little past in places). `o.passes` times over, each
    /// pass a little turned. Refolds when `o.refold` says so. Long passes
    /// age the paint as they go (hand time slices). Returns the strokes,
    /// and how many of them began on a full face (`FULL`).
    pub fn rag_region(&mut self, rag: &mut Rag, m: &Mask, o: &RagPass) -> (usize, usize) {
        self.check_mask(m);
        let mut rng = Rng::new(o.seed ^ 0x7A6E_0000);
        let (mut n, mut full) = (0, 0);
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
                full += usize::from(rag.load >= FULL);
                self.rag_wipe_in(rag, pts, &prof, s, self.rag_lift.then_some(m));
                n += 1;
                if let Some(slice) = self.hand_slice_secs() {
                    let owed = self.hand_owed_secs();
                    if owed >= slice {
                        self.hand_pass(owed);
                    }
                }
            }
        }
        (n, full)
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
