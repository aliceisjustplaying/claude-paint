//! The hand's ledger: what the brushes did, and an estimate of the hand
//! time it takes.
//!
//! Every mark the canvas makes is counted here as it is planned (strokes,
//! their path length, stipple touches, trips to the palette to reload or mix
//! a new pile, wipes on the rag), and each is priced in seconds of hand time
//! (`pace`). Painting nothing here: the ledger only counts. What a caller
//! does with the time is up to it.
//!
//! Counting happens where the engine plans marks, on the whole canvas and in
//! a fixed order, before the tiles are split among threads or a crop drops
//! the ones off its window: the ledger is the same at any thread count. In
//! a crop it is the same but for what looks at the pixels the crop holds:
//! the look-and-fill dabs (which gaps it sees), and for aimed marks
//! (`color_over`, a stipple's look under a dip) the color a dip asks for,
//! so whether it is a reload or a new pile, and with a sitting's palette
//! (`Piles`) the piles later dips find. With hand time on the ledger is the
//! clock, so a crop can age its paint by a little more or less than the
//! whole canvas does.

use crate::bristle::{Kind, Tool};
use crate::canvas::Canvas;

/// Seconds of hand time per kind of move. Sources and estimates:
/// [S]: from a source; [E]: an estimate.
pub mod pace {
    /// Fitts's law for bringing the brush down where the next mark starts,
    /// `T = FITTS_A + FITTS_B · log2(1 + D / W)`. MacKenzie's stylus
    /// reanalysis gives b ≈ 0.095–0.122 s/bit [S]; the intercept here also
    /// holds the lift and set-down of a brush (0.1 s) [E].
    pub const FITTS_A: f64 = 0.1;
    pub const FITTS_B: f64 = 0.122;
    /// The steering law for drawing the stroke itself,
    /// `T = STEER_B · L / W`, with b ≈ 0.11 s for a stylus in a linear
    /// tunnel (Accot & Zhai 1997) [S].
    pub const STEER_B: f64 = 0.11;
    /// The tunnel a stroke steers through, in mark widths (looser than a
    /// strict tunnel task) [E].
    pub const TUNNEL: f64 = 4.0;
    /// The narrowest tunnel (mm), so a hairline mark is not priced as if
    /// steered through its own width [E].
    pub const TUNNEL_MIN_MM: f64 = 3.0;
    /// The fastest a brush travels (mm/s): a broad sweep of the arm [E].
    pub const V_MAX: f64 = 400.0;
    /// The hop to the next stroke is about a stroke long, and at least this
    /// many mark widths (hatching steps along a stroke's width at a time)
    /// [E].
    pub const HOP_MIN_W: f64 = 2.0;
    /// A dab or stipple touch: aimed at a spot about two tip widths across,
    /// three widths from the last (Fitts), with a short dwell in contact [E].
    pub const DWELL: f64 = 0.08;
    /// Fastest tapping: about 6 taps a second for an index finger [S], so
    /// no touch is quicker than this.
    pub const TAP_MIN: f64 = 1.0 / 6.0;
    /// A trip to the palette to reload from a pile already mixed: two
    /// reaches of ~300 mm to a ~10 mm pile (Fitts: about 5 bits, 0.7 s
    /// each) and a turn of the brush in the pile [E].
    pub const RELOAD: f64 = 2.5;
    /// Mixing a new pile (or adjusting one) with the knife or brush [E].
    pub const REMIX: f64 = 20.0;
    /// Wiping the brush on the rag (a blender between passes) [E].
    pub const WIPE: f64 = 2.0;
    /// A fill dab (`fill`: a dab into a gap of a passage) reuses the paint
    /// of the passage: one reload serves about this many dabs [E].
    pub const DABS_PER_RELOAD: f64 = 8.0;
    /// Pencil and chalk: drawing speed (mm/s) and the set-down per line [E].
    pub const PENCIL_MM_S: f64 = 40.0;
    pub const PENCIL_LINE: f64 = 0.3;
    /// A glaze or varnish brushed on: a 25 mm brush at 150 mm/s, each part
    /// gone over twice [E].
    pub const GLAZE_MM2_S: f64 = 25.0 * 150.0 / 2.0;
}

/// What the brushes did, and the hand time it took (seconds).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "replay-effects", derive(serde::Serialize, serde::Deserialize))]
pub struct Tally {
    /// Strokes dragged (planned passes, fill dabs, cut-in strokes, brush
    /// strokes by hand).
    pub strokes: u64,
    /// Touches of a tip (stipple, `touch`).
    pub touches: u64,
    /// Path length of the strokes, mm on the painting.
    pub length_mm: f64,
    /// Trips to the palette for more of a pile already mixed.
    pub reloads: f64,
    /// New piles mixed (or piles adjusted).
    pub remixes: u64,
    /// Wipes on the rag.
    pub wipes: u64,
    /// Pencil or chalk lines drawn.
    pub lines: u64,
    /// Hand time, seconds.
    pub secs: f64,
    /// The part of `secs` already on the painting's clock (hand time on).
    pub clocked: f64,
}

/// The piles of mixed paint on the palette: a dip into a color close to a
/// pile already there is a reload, a new color is a new pile to mix. The
/// palette holds `PILES` at most; the oldest is scraped off for a new one.
/// A sitting keeps one palette for its passes and held brushes (passed to `Canvas::work_with`); `Canvas::work` starts a clean one.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "replay-effects", derive(serde::Serialize, serde::Deserialize))]
pub struct Piles {
    /// OKLab colors, oldest first.
    pub piles: Vec<[f32; 3]>,
}

impl Piles {
    /// Room on the palette [E].
    pub const PILES: usize = 16;
    /// Colors closer than this (OKLab distance) come from the same pile, so
    /// color jitter within a pile doesn't count as a new mix [E].
    pub const SAME: f32 = 0.035;

    /// Dip into `c` (linear RGB): true if a new pile had to be mixed.
    pub fn dip(&mut self, c: crate::Rgb) -> bool {
        let l = crate::color::to_oklab(c);
        let d = |p: &[f32; 3]| ((p[0] - l[0]).powi(2) + (p[1] - l[1]).powi(2) + (p[2] - l[2]).powi(2)).sqrt();
        if let Some(i) = self.piles.iter().rposition(|p| d(p) < Self::SAME) {
            // the pile in use becomes the most recent
            let p = self.piles.remove(i);
            self.piles.push(p);
            return false;
        }
        if self.piles.len() >= Self::PILES {
            self.piles.remove(0);
        }
        self.piles.push(l);
        true
    }

    /// Dip into `c` and count the trip in `t` (a remix or a reload).
    pub fn trip(&mut self, t: &mut Tally, c: crate::Rgb) {
        if self.dip(c) { t.remix() } else { t.reload(1.0) }
    }

    /// Knife a pile of color `c` on the palette (no brush loaded): the
    /// mixing time in `t`, and the pile on the palette for later dips.
    pub fn knife(&mut self, t: &mut Tally, c: crate::Rgb) {
        self.dip(c);
        t.knife();
    }
}

/// The width (mm) a mark of `tool` makes, for pacing it.
fn mark_mm(tool: &Tool, mm_per_unit: f32) -> f64 {
    let w = tool.width as f64 * mm_per_unit as f64;
    // a pointed brush mostly works with its point, a stippler with its face
    match tool.kind {
        Kind::Rigger => 0.5 * w,
        _ => w,
    }
    .max(0.05)
}

/// Hand time (s) of one stroke of path length `len_mm` with a mark `w_mm`
/// wide.
pub fn stroke_secs(len_mm: f64, w_mm: f64) -> f64 {
    let hop = len_mm.max(pace::HOP_MIN_W * w_mm);
    let aim = pace::FITTS_A + pace::FITTS_B * (1.0 + hop / (2.0 * w_mm)).log2();
    let tunnel = (pace::TUNNEL * w_mm).max(pace::TUNNEL_MIN_MM);
    let draw = (pace::STEER_B * len_mm / tunnel).max(len_mm / pace::V_MAX);
    aim + draw
}

/// Hand time (s) of one touch of a tip: the same for a tip of any size (a
/// spot about two tips across, three tips from the last: the index of
/// difficulty doesn't depend on the size).
pub fn touch_secs() -> f64 {
    (pace::FITTS_A + pace::FITTS_B * (1.0 + 3.0 / 2.0f64).log2() + pace::DWELL).max(pace::TAP_MIN)
}

impl Tally {
    /// A stroke of `tool` along `pts` (units).
    pub fn stroke(&mut self, tool: &Tool, pts: &[(f32, f32)], mm_per_unit: f32) {
        let len = crate::path::length(pts) as f64 * mm_per_unit as f64;
        self.strokes += 1;
        self.length_mm += len;
        self.secs += stroke_secs(len, mark_mm(tool, mm_per_unit));
    }

    /// A touch of a tip.
    pub fn touch(&mut self) {
        self.touches += 1;
        self.secs += touch_secs();
    }

    /// `n` trips to the palette for more paint (fractions for fill dabs).
    pub fn reload(&mut self, n: f64) {
        self.reloads += n;
        self.secs += n * pace::RELOAD;
    }

    /// A new pile mixed (and the brush loaded from it).
    pub fn remix(&mut self) {
        self.remixes += 1;
        self.secs += pace::REMIX + pace::RELOAD;
    }

    /// A pile knifed on the palette, no brush loaded.
    pub fn knife(&mut self) {
        self.remixes += 1;
        self.secs += pace::REMIX;
    }

    /// The brush wiped on the rag.
    pub fn wipe(&mut self) {
        self.wipes += 1;
        self.secs += pace::WIPE;
    }

    /// Pencil or chalk: `lines` lines, `mm` drawn in all.
    pub fn draw(&mut self, lines: u64, mm: f64) {
        self.lines += lines;
        self.secs += lines as f64 * pace::PENCIL_LINE + mm.max(0.0) / pace::PENCIL_MM_S;
    }

    /// A glaze brushed over `area_mm2`.
    pub fn glaze(&mut self, area_mm2: f64) {
        self.secs += area_mm2.max(0.0) / pace::GLAZE_MM2_S;
    }

    /// What was done since `before` (an earlier copy of this ledger).
    pub fn since(&self, before: &Tally) -> Tally {
        Tally {
            strokes: self.strokes - before.strokes,
            touches: self.touches - before.touches,
            length_mm: self.length_mm - before.length_mm,
            reloads: self.reloads - before.reloads,
            remixes: self.remixes - before.remixes,
            wipes: self.wipes - before.wipes,
            lines: self.lines - before.lines,
            secs: self.secs - before.secs,
            clocked: self.clocked - before.clocked,
        }
    }

    /// Hand time in minutes.
    pub fn minutes(&self) -> f64 {
        self.secs / 60.0
    }

    /// The ledger as a checkpoint stores it (PAINTCK7): the counts, then the
    /// bits of the lengths and times.
    pub(crate) fn to_words(self) -> [u64; 9] {
        let t = self;
        [t.strokes, t.touches, t.remixes, t.wipes, t.lines, t.length_mm.to_bits(), t.reloads.to_bits(), t.secs.to_bits(), t.clocked.to_bits()]
    }

    /// A ledger read back from `to_words`; None if a length or a time isn't
    /// finite.
    pub(crate) fn from_words(w: [u64; 9]) -> Option<Tally> {
        let [strokes, touches, remixes, wipes, lines, ..] = w;
        let [length_mm, reloads, secs, clocked] = [w[5], w[6], w[7], w[8]].map(f64::from_bits);
        [length_mm, reloads, secs, clocked].iter().all(|v| v.is_finite()).then_some(Tally { strokes, touches, length_mm, reloads, remixes, wipes, lines, secs, clocked })
    }
}

impl std::fmt::Display for Tally {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} strokes ({:.1} m of path), {} touches, {:.0} reloads, {} piles mixed, {} wipes, {} pencil lines: {:.1} min",
            self.strokes,
            self.length_mm / 1000.0,
            self.touches,
            self.reloads,
            self.remixes,
            self.wipes,
            self.lines,
            self.minutes()
        )
    }
}

/// A callback run after every slice of hand time put on the clock (a
/// slice of a long pass, or the rest of a verb's time): the canvas as it is
/// then, with `tally().clocked` the hand time so far. One per thread; it
/// only reads. For replays that record the painting as it is made.
#[cfg(feature = "hand-hook")]
pub mod hand_hook {
    use crate::Canvas;
    use std::cell::RefCell;

    type Hook = Box<dyn FnMut(&Canvas)>;

    thread_local! {
        static HOOK: RefCell<Option<Hook>> = const { RefCell::new(None) };
    }

    /// Set (or with None, clear) this thread's hook; returns the old one.
    pub fn set(f: Option<Hook>) -> Option<Hook> {
        HOOK.with(|h| std::mem::replace(&mut *h.borrow_mut(), f))
    }

    pub(crate) fn call(c: &Canvas) {
        // taken out while it runs, so a hook that paints can't re-enter it
        let Some(mut f) = HOOK.with(|h| h.borrow_mut().take()) else { return };
        f(c);
        HOOK.with(|h| {
            let mut h = h.borrow_mut();
            if h.is_none() {
                *h = Some(f);
            }
        });
    }
}

impl Canvas {
    /// Hand time: with `Some(slice)` (minutes), passes are painted in slices
    /// of about that much hand time with the paint aging between them
    /// (`wait`), and `clock_hand_min` puts the rest of the hand time counted on
    /// the clock. `None` (the default): marks take no time on the clock.
    /// Either way the ledger counts from here as already clocked.
    pub fn set_hand_time(&mut self, slice_min: Option<f32>) {
        self.hand_slice = slice_min.filter(|s| s.is_finite() && *s > 0.0);
        self.tally.clocked = self.tally.secs;
    }

    /// The slice (minutes) with hand time on.
    pub fn hand_time(&self) -> Option<f32> {
        self.hand_slice
    }

    pub(crate) fn hand_slice_secs(&self) -> Option<f64> {
        self.hand_slice.map(|m| m as f64 * 60.0)
    }

    pub(crate) fn covering_slice_secs(&self, slice: Option<f64>, paints: impl IntoIterator<Item = crate::wet::Paint>, secs: &[f64]) -> Option<f64> {
        if std::env::var("PAINT_COARSE_AGING").is_ok_and(|v| v == "1") {
            slice.map(|s| self.bounded_slice_secs(s, paints, secs))
        } else {
            slice
        }
    }

    /// An opt-in covering slice, capped at four legacy slices and one hour.
    /// The fastest possible open-film oxidation may advance by at most 0.02
    /// per batch (at most 0.2 fluidity, since smoothstep's slope is <= 1.5/GEL).
    /// Only passes that stay wholly open under that upper bound qualify;
    /// persistent solvent, tacky layers and crops keep their original cadence. This is
    /// a local drying-lag bound, not a bound on final image differences.
    fn bounded_slice_secs(&self, slice: f64, paints: impl IntoIterator<Item = crate::wet::Paint>, secs: &[f64]) -> f64 {
        use crate::drying::{GEL, OPEN_SHARE, TOUCH_DRY_MIN, TOUCH_DRY_MIN_3, rate};

        let total: f64 = secs.iter().sum();
        if self.engine != 5 || self.f.w != self.f.full_w || self.f.h != self.f.full_h || total <= slice || !total.is_finite() || self.wet.clock.tacky.is_some() || self.wet.has_solvent(self.f.w) {
            return slice;
        }
        let mut drying = 0.01f32;
        for paint in paints {
            if paint.thinner > 0.0 || !paint.drying.is_finite() {
                return slice;
            }
            drying = drying.max(paint.drying);
        }
        let mut cure = 0.0f32;
        if let Some((x0, y0, x1, y1)) = self.wet.dirty {
            for y in y0..y1.min(self.f.h) {
                for x in x0..x1.min(self.f.w) {
                    let i = y * self.f.w + x;
                    if self.wet.vol[i] <= 0.0 {
                        continue;
                    }
                    let c = self.wet.clock.px.get(i).map_or(0.0, |p| p.cure);
                    let d = self.wet.hide[i][2];
                    if !c.is_finite() || !d.is_finite() || c >= 0.5 * GEL {
                        return slice;
                    }
                    cure = cure.max(c);
                    drying = drying.max(d);
                }
            }
        }
        let fastest = rate(0.0, 1.0, drying) as f64 * (TOUCH_DRY_MIN / TOUCH_DRY_MIN_3 * GEL / OPEN_SHARE) as f64;
        if cure as f64 + fastest * total / 60.0 >= (0.5 * GEL) as f64 {
            return slice;
        }
        let bounded = (4.0 * slice).min(3600.0).min(60.0 * 0.02 / fastest);
        let largest_tile = secs.iter().copied().fold(0.0f64, f64::max);
        if bounded >= slice + largest_tile { bounded } else { slice }
    }

    /// Hand time counted but not yet on the clock (s); 0 with hand time off.
    pub fn hand_owed_secs(&self) -> f64 {
        if self.hand_slice.is_none() { 0.0 } else { (self.tally.secs - self.tally.clocked).max(0.0) }
    }

    /// Minutes of painting time: the clock and the hand time owed to it.
    pub fn now_min(&self) -> f64 {
        self.clock() + self.hand_owed_secs() / 60.0
    }

    /// Put the owed hand time on the clock: the paint ages by it. Returns
    /// the minutes. With hand time off, nothing.
    pub fn clock_hand_min(&mut self) -> f64 {
        let owed = self.hand_owed_secs();
        if owed > 0.0 {
            self.hand_pass(owed);
        }
        if self.hand_slice.is_some() && (matches!(self.work_tile_mode().as_deref(), Some("1" | "quarter")) || (self.engine == 5 && std::env::var("PAINT_COARSE_AGING").is_ok_and(|v| v == "1"))) {
            self.tally.clocked = self.tally.secs;
        }
        owed / 60.0
    }

    /// `secs` of hand time pass (a slice of a pass): wait, and count them as
    /// clocked.
    pub(crate) fn hand_pass(&mut self, secs: f64) {
        self.wait((secs / 60.0) as f32);
        self.tally.clocked += secs;
        #[cfg(feature = "hand-hook")]
        hand_hook::call(self);
    }

    /// The hand's ledger so far (since the canvas was made).
    pub fn tally(&self) -> Tally {
        self.tally
    }

    /// The ledger, to count marks made outside the canvas (a palette trip
    /// for a held brush, a glaze's brushing).
    pub fn tally_mut(&mut self) -> &mut Tally {
        &mut self.tally
    }

    /// Millimeters per canvas unit.
    pub fn mm_per_unit(&self) -> f32 {
        self.mm_per_unit
    }

}
