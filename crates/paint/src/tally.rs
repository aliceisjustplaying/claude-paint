//! The hand's ledger: what the brushes actually did, and how long a
//! painter's hand takes to do it.
//!
//! Every mark the canvas makes is counted here as it is planned (strokes,
//! their path length, stipple touches, trips to the palette to reload or mix
//! a new pile, wipes on the rag), and each is priced in seconds of hand time
//! (`pace`). Painting nothing here: the ledger only counts. What a caller
//! does with the time is up to it (the easel's hand clock turns it into
//! `Canvas::wait`, so paint ages while the hand works; notes/time.md).
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
//! whole canvas does (notes/time.md, known issues).

use crate::bristle::{Kind, Tool};
use crate::canvas::Canvas;

/// Seconds of hand time per kind of move. Sources and estimates:
/// notes/time.md. [S]: from a source; [E]: my estimate.
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
    /// How loose a painter's path is: the tunnel a stroke steers through is
    /// this many mark widths wide (a brushstroke isn't a tunnel task) [E].
    pub const TUNNEL: f64 = 4.0;
    /// The narrowest tunnel (mm): a rigger line is drawn fluently, not
    /// steered through a hair's width [E].
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
    /// A fill dab (looking over a passage and dabbing into the gaps) reuses
    /// the paint of the passage: one reload serves about this many dabs [E].
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
/// A sitting keeps one palette for its passes and held brushes (the easel's
/// `Hand`, passed to `Canvas::work_with`); `Canvas::work` starts a clean one.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Piles {
    /// OKLab colors, oldest first.
    pub piles: Vec<[f32; 3]>,
}

impl Piles {
    /// Room on the palette [E].
    pub const PILES: usize = 16;
    /// Colors closer than this (OKLab distance) come from the same pile: a
    /// painter doesn't remix for the jitter of a hand-mixed pile [E].
    pub const SAME: f32 = 0.035;

    /// Dip into `c` (linear RGB): true if a new pile had to be mixed.
    pub fn dip(&mut self, c: crate::Rgb) -> bool {
        let l = crate::color::to_oklab(c);
        let d = |p: &[f32; 3]| ((p[0] - l[0]).powi(2) + (p[1] - l[1]).powi(2) + (p[2] - l[2]).powi(2)).sqrt();
        if let Some(i) = self.piles.iter().rposition(|p| d(p) < Self::SAME) {
            // the pile in use moves to the front of the painter's mind
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

/// Path length of `pts` (units).
pub fn path_len(pts: &[(f32, f32)]) -> f32 {
    pts.windows(2).map(|w| ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt()).sum()
}

impl Tally {
    /// A stroke of `tool` along `pts` (units).
    pub fn stroke(&mut self, tool: &Tool, pts: &[(f32, f32)], mm_per_unit: f32) {
        let len = path_len(pts) as f64 * mm_per_unit as f64;
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

impl Canvas {
    /// Hand time: with `Some(slice)` (minutes), passes are painted in slices
    /// of about that much hand time with the paint ageing between them
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

    /// Hand time counted but not yet on the clock (s); 0 with hand time off.
    pub fn hand_owed_secs(&self) -> f64 {
        if self.hand_slice.is_none() { 0.0 } else { (self.tally.secs - self.tally.clocked).max(0.0) }
    }

    /// Put the owed hand time on the clock: the paint ages by it. Returns
    /// the minutes. With hand time off, nothing.
    pub fn clock_hand_min(&mut self) -> f64 {
        let owed = self.hand_owed_secs();
        if owed > 0.0 {
            self.hand_pass(owed);
        }
        owed / 60.0
    }

    /// `secs` of hand time pass (a slice of a pass): wait, and count them as
    /// clocked.
    pub(crate) fn hand_pass(&mut self, secs: f64) {
        self.wait((secs / 60.0) as f32);
        self.tally.clocked += secs;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::Crop;
    use crate::color::hex;
    use crate::handling::Handling;
    use crate::mask::Mask;
    use crate::stipple::Stipple;

    fn in_pool<T: Send>(threads: usize, f: impl FnOnce() -> T + Send) -> T {
        rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap().install(f)
    }

    /// A sky, a stipple over it and a few strokes, on a 440 mm canvas, with
    /// hand time on (a short slice) or off.
    fn scene(crop: Option<Crop>, hand: Option<f32>) -> Canvas {
        let mut c = Canvas::new_window(240, 1.4, hex("#c8b89a"), crop).with_size_mm(440.0);
        c.set_hand_time(hand);
        let f = c.frame();
        let sky = Mask::from_fn(f, |_, y| if y < 420.0 { 1.0 } else { 0.0 });
        let hd = Handling::new(Tool::filbert(18.0)).color(|_, y| if y < 200.0 { hex("#6f84a8") } else { hex("#d8ccb0") }).coverage(3.0).fill(false);
        c.work(&sky, &hd, 3);
        let band = Mask::from_fn(f, |_, y| if (300.0..500.0).contains(&y) { 1.0 } else { 0.0 });
        let mut sp = Stipple::new(Tool::stippler(6.0));
        sp.color = Box::new(|_, _| hex("#cfccc2"));
        c.stipple(&band, &sp, 5);
        c.clock_hand_min();
        c
    }

    /// The picture, its relief and the wet film on it (paint that aged but
    /// hasn't set yet differs only there).
    fn px_bits(c: &Canvas) -> Vec<u32> {
        c.pixels().iter().flat_map(|p| p.map(f32::to_bits)).chain(c.height.iter().map(|v| v.to_bits())).chain(c.wet.vol.iter().chain(c.wet.top.iter().map(|t| &t.v)).map(|v| v.to_bits())).collect()
    }

    #[test]
    fn the_ledger_is_the_same_at_any_thread_count_and_in_a_crop() {
        let a = in_pool(1, || scene(None, None).tally());
        let b = in_pool(4, || scene(None, None).tally());
        assert_eq!(a, b);
        assert!(a.strokes > 100 && a.touches > 100 && a.remixes >= 2 && a.secs > 60.0, "{a}");
        // a crop plans the same marks (no fill dabs here: they look at the
        // pixels a crop holds)
        let w = in_pool(4, || scene(Some(Crop { units: [300.0, 250.0, 460.0, 400.0], margin: 20.0 }), None).tally());
        assert_eq!(a, w);
    }

    #[test]
    fn hand_time_ages_the_paint_as_it_goes_and_is_deterministic() {
        // off: the clock doesn't move, and nothing is owed
        let off = scene(None, None);
        let c0 = Canvas::new(240, 1.4, hex("#c8b89a")).clock();
        assert_eq!(off.clock(), c0);
        assert_eq!(off.hand_owed_secs(), 0.0);
        // on: the whole ledger is on the clock, and in slices of 2 minutes
        // the pass aged as it went; 1 and 4 threads paint the same
        let a = in_pool(1, || scene(None, Some(2.0)));
        let b = in_pool(4, || scene(None, Some(2.0)));
        assert_eq!(px_bits(&a), px_bits(&b));
        let t = a.tally();
        assert!(((a.clock() - c0) - t.minutes()).abs() < 1e-3, "clock {} vs hand {}", a.clock() - c0, t.minutes());
        assert_eq!(t.clocked, t.secs);
        assert_ne!(px_bits(&a), px_bits(&off), "paint that aged while the hand worked");
    }

    /// A checkpoint of a hand-timed painting resumes with its ledger, the
    /// time still owed and hand time on: a resumed run paints and clocks
    /// exactly what an uninterrupted one does (review r6, finding 1).
    #[test]
    fn a_checkpoint_keeps_hand_time_and_the_ledger() {
        let first = |c: &mut Canvas| {
            // (down to the bottom: the sweep's last slice has strokes to owe)
            let ground = Mask::from_fn(c.frame(), |_, y| if y > 250.0 { 1.0 } else { 0.0 });
            c.work(&ground, &Handling::new(Tool::filbert(18.0)).color(|_, _| hex("#6f84a8")).coverage(3.0).fill(false), 3);
        };
        let then = |c: &mut Canvas| {
            let band = Mask::from_fn(c.frame(), |_, y| if (300.0..500.0).contains(&y) { 1.0 } else { 0.0 });
            c.work(&band, &Handling::new(Tool::filbert(14.0)).color(|_, _| hex("#d8ccb0")).coverage(3.0).fill(false), 4);
            c.clock_hand_min();
        };
        let mut a = Canvas::new(240, 1.4, hex("#c8b89a")).with_size_mm(440.0);
        a.set_hand_time(Some(1.0));
        first(&mut a);
        assert!(a.hand_owed_secs() > 0.0 && a.tally().strokes > 0, "time still owed at the checkpoint: {} {:?}", a.tally(), a.hand_owed_secs());
        let mut buf = Vec::new();
        a.write_state(&mut buf, "").unwrap();
        let (mut b, _) = Canvas::read_state(&mut std::io::Cursor::new(buf)).unwrap();
        assert_eq!(b.hand_time(), a.hand_time());
        assert_eq!(b.tally(), a.tally());
        assert_eq!(b.hand_owed_secs(), a.hand_owed_secs());
        then(&mut a);
        then(&mut b);
        assert_eq!(b.tally(), a.tally());
        assert_eq!(b.clock().to_bits(), a.clock().to_bits());
        assert_eq!(px_bits(&b), px_bits(&a));
    }

    /// With hand time on, only the default order becomes a sweep down: an
    /// order the painter asked for is kept (review r6, finding 3). Seen in
    /// the paint's age: a sweep paints the top in the first slices (older
    /// than the bottom); scatter's slices each reach over the whole area. A
    /// preset's order counts as asked for (`ruler()`, thermos B2).
    #[test]
    fn hand_time_keeps_an_order_asked_for() {
        use crate::handling::Order;
        let age = |set: &dyn Fn(Handling) -> Handling| {
            let mut c = Canvas::new(240, 1.4, hex("#c8b89a")).with_size_mm(440.0);
            c.set_hand_time(Some(1.0));
            let all = Mask::from_fn(c.frame(), |_, _| 1.0);
            let hd = set(Handling::new(Tool::filbert(18.0)).color(|_, _| hex("#6f84a8")).coverage(3.0).fill(false));
            c.work(&all, &hd, 3);
            let (w, h) = (c.f.w, c.f.h);
            let mean = |y0: usize, y1: usize| {
                let v: Vec<f32> = (y0 * w..y1 * w).map(|i| c.wet.clock.px[i].cure).collect();
                v.iter().sum::<f32>() / v.len() as f32
            };
            let (top, bottom) = (mean(h / 10, h * 3 / 10), mean(h * 6 / 10, h * 8 / 10));
            (top, bottom, c.hand_owed_secs())
        };
        let (t, b, _) = age(&|h| h);
        assert!(t > 1.8 * b, "the default sweeps down: top {t} bottom {b}");
        for o in [Order::Scatter, Order::Passages] {
            let (t, b, _) = age(&|h| h.order(o));
            assert!(t < 1.5 * b && b < 1.5 * t, "{o:?} asked for is kept: top {t} bottom {b}");
        }
        let (t, b, _) = age(&|h| h.ruler().coverage(3.0));
        assert!(t < 1.5 * b && b < 1.5 * t, "the ruler's scatter is kept: top {t} bottom {b}");
    }

    /// A wait between slices must see the strokes of the slices after it as
    /// fresh work (their ids above its watermark), or their paint takes the
    /// cure and thickness of the film it went over: setting streaks along
    /// the slices' seams (found in notes/time's example in sittings).
    #[test]
    fn the_slices_after_a_wait_are_fresh_work() {
        let mut c = Canvas::new(240, 1.4, hex("#c8b89a")).with_size_mm(440.0);
        c.set_hand_time(Some(1.0));
        let all = Mask::from_fn(c.frame(), |_, _| 1.0);
        c.work(&all, &Handling::new(Tool::filbert(18.0)).color(|_, _| hex("#6f84a8")).coverage(3.0).fill(false), 3);
        assert!(c.clock() > 0.0 && c.hand_owed_secs() > 0.0, "sliced, with the last slice still owed");
        let mark = c.wet.clock.mark;
        let fresh = c.wet.stroke.iter().filter(|&&id| id > mark).count();
        let older = c.wet.stroke.iter().filter(|&&id| id > 0 && id <= mark).count();
        assert!(fresh > 0 && older > 0, "fresh {fresh}, older {older}");
    }

    #[test]
    fn piles_reload_near_colors_and_mix_new_ones() {
        let mut p = Piles::default();
        assert!(p.dip(hex("#6f84a8")));
        assert!(!p.dip(hex("#7084a9")), "the jitter of one pile");
        assert!(p.dip(hex("#d8ccb0")));
        assert!(!p.dip(hex("#6f84a8")), "back to the first pile");
        for i in 0..Piles::PILES {
            assert!(p.dip([0.05 + 0.3 * (i % 4) as f32, 0.05 + 0.3 * (i / 4) as f32, 0.9]));
        }
        assert!(p.dip(hex("#d8ccb0")), "scraped off to make room");
    }

    #[test]
    fn fine_strokes_are_slower_per_mm() {
        let broad = stroke_secs(100.0, 12.0) / 100.0;
        let fine = stroke_secs(100.0, 0.5) / 100.0;
        assert!(fine > 3.0 * broad, "fine {fine} broad {broad}");
        // a 10 cm broad sweep takes under a second, a 10 cm hairline a few
        assert!(stroke_secs(100.0, 12.0) < 1.0);
        assert!((2.0..8.0).contains(&stroke_secs(100.0, 0.5)));
    }

    #[test]
    fn touches_are_no_faster_than_tapping() {
        let t = touch_secs();
        assert!((pace::TAP_MIN..0.6).contains(&t), "{t}");
    }

    /// A checkpoint stores the ledger in PAINTCK7's order: the five counts,
    /// then the bits of the length, reloads, seconds and clocked seconds.
    #[test]
    fn the_ledger_keeps_its_checkpoint_layout() {
        let t = Tally { strokes: 1, touches: 2, length_mm: 6.5, reloads: 7.5, remixes: 3, wipes: 4, lines: 5, secs: 8.5, clocked: 9.5 };
        assert_eq!(t.to_words(), [1, 2, 3, 4, 5, 6.5f64.to_bits(), 7.5f64.to_bits(), 8.5f64.to_bits(), 9.5f64.to_bits()]);
        assert_eq!(Tally::from_words(t.to_words()), Some(t));
        let mut w = t.to_words();
        w[7] = f64::NAN.to_bits();
        assert_eq!(Tally::from_words(w), None);
    }

    #[test]
    fn since_is_the_difference() {
        let tool = Tool::round_sable(3.0);
        let mut a = Tally::default();
        a.stroke(&tool, &[(0.0, 0.0), (30.0, 40.0)], 0.44);
        let b0 = a;
        a.touch();
        a.remix();
        let d = a.since(&b0);
        assert_eq!((d.strokes, d.touches, d.remixes), (0, 1, 1));
        assert!((d.secs - (touch_secs() + pace::REMIX + pace::RELOAD)).abs() < 1e-9);
        assert!((b0.length_mm - 22.0).abs() < 1e-4);
    }
}
