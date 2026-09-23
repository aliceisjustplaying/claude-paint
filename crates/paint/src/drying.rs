//! Time and drying: the painting has a clock, and wet paint ages.
//!
//! Oil paint dries by oxidation: the oil takes up oxygen, cross-links and
//! turns from a liquid into a gel into a solid film. A painter feels four
//! stages (sources and numbers in notes/drying.md):
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
//! film dilutes its cure by volume. When the film reaches the gel point it
//! levels for as long as it was fluid, then it bakes into the dry picture:
//! from then on it is part of the surface and its tack lives on in `sub`
//! until it is touch-dry. So a pixel can hold new wet paint over a set layer.
//!
//! `Canvas::wait(minutes)` advances the clock. `Canvas::dry()` waits until all
//! paint is touch-dry. A painting that never waits renders as it did before
//! the clock existed (the drying state isn't even allocated).

use crate::canvas::Canvas;
use crate::pigment::Pigment;
use crate::surface::{COAT_UM, SET_TIME};
use crate::{smoothstep, surface::vnoise};
use rayon::prelude::*;

/// Minutes to touch-dry for one lean 25 µm coat of average paint
/// (`drying` 1). Thin-film touch-dry times run 1–2 days (umber, lead white)
/// to 2–5 days (blacks) and 7–14 (alizarin); one coat here is thinner than
/// those test films. Estimate from those ranges, see notes/drying.md.
pub const TOUCH_DRY_MIN: f32 = 24.0 * 60.0;
/// Cure at the gel point: the film stops flowing and becomes tacky.
/// Estimate: a lead-white-rich coat (`drying` 2) gels after ~1.8 h, an
/// average one after ~3.6 h.
pub const GEL: f32 = 0.15;
/// How much a film's thickness slows its drying: time ∝ (h / 1 coat)^THICK.
/// Surface skinning is reaction-limited in thin films and increasingly
/// oxygen-limited in thick ones [E].
const THICK: f32 = 0.7;
/// How much a fat, medium-rich paint (stiff 0) dries slower than stiff tube
/// paint (stiff 1) [E].
const FAT: f32 = 0.6;
/// How much faster a tacky surface pulls paint off a brush [E].
const GRAB: f32 = 2.0;

/// Relative drying rates of the period pigments ground in oil (1 = average;
/// higher dries faster). From Mayer's comparative list (fast: lead white,
/// umbers, chrome yellow, Prussian blue; medium: earths, cobalt; slow to very
/// slow: vermilion, ivory/lamp/vine black, madder, alizarin) and touch-dry
/// ranges for thin films; see notes/drying.md. Cobalt glass (smalt) was
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
}

/// Cure gained per minute by an open film `vol` coats thick, of stiffness
/// `stiff` and pigment drying rate `drying`.
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
    /// Volume of the open film at the last `wait` (fresh paint since then
    /// dilutes the cure).
    pub seen: f32,
    /// Cure of the top set film, baked into the dry picture (≥ 1: dry).
    pub sub: f32,
    /// Its cure per minute.
    pub srate: f32,
}

impl Px {
    pub const FRESH: Px = Px { cure: 0.0, lev: SET_TIME, seen: 0.0, sub: 1.0, srate: 0.0 };
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
    /// history, so a painting can work wet into wet (`wait(0.0)`, or no wait),
    /// come back to tacky paint (`wait(180.0)`) or to a dry layer
    /// (`wait(24.0 * 60.0)`).
    pub fn wait(&mut self, minutes: f32) {
        let dt = minutes.max(0.0);
        if !dt.is_finite() {
            return self.dry();
        }
        let n = self.f.w * self.f.h;
        if self.wet.clock.px.len() != n {
            self.wet.clock.px = vec![Px::FRESH; n];
        }
        self.absorb();
        let w = self.f.w;
        // age the open films
        if let Some((x0, y0, x1, y1)) = self.wet.dirty {
            let (x1, y1) = (x1.min(w), y1.min(self.f.h));
            let wet = &mut self.wet;
            let (vol, hide) = (&wet.vol, &wet.hide);
            wet.clock.px[y0 * w..y1 * w].par_chunks_mut(w).enumerate().for_each(|(j, row)| {
                for x in x0..x1 {
                    let i = (y0 + j) * w + x;
                    let v = vol[i];
                    if v >= 1e-5 {
                        row[x].cure += dt * rate(v, hide[i][1], hide[i][2]);
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
        self.wet.clock.now += dt as f64;
    }

    /// Where the paint at a point (units) is in drying.
    pub fn drying_at(&self, x: f32, y: f32) -> Stage {
        let i = self.f.index(x, y);
        let v = self.wet.vol[i];
        let p = self.wet.clock.px.get(i).copied().unwrap_or(Px::FRESH);
        if v >= 1e-3 {
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
    pub fn dry(&mut self) {
        if !self.wet.clock.px.is_empty() {
            self.absorb();
        }
        // how long until the slowest film is touch-dry
        let mut left = 0.0f32;
        if let Some((x0, y0, x1, y1)) = self.wet.dirty {
            let (w, h) = (self.f.w, self.f.h);
            let (x1, y1) = (x1.min(w), y1.min(h));
            let (vol, hide, px) = (&self.wet.vol, &self.wet.hide, &self.wet.clock.px);
            left = (y0..y1)
                .into_par_iter()
                .map(|y| {
                    let mut m = 0.0f32;
                    for i in y * w + x0..y * w + x1 {
                        if vol[i] >= 1e-5 {
                            let c = px.get(i).map_or(0.0, |p| p.cure);
                            m = m.max((1.0 - c).max(0.0) / rate(vol[i], hide[i][1], hide[i][2]));
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

    /// Fold what was painted since the last `wait` into the drying state:
    /// fresh paint dilutes the cure of the film it went into, and a film that
    /// was worked levels again, at the fluidity it has now.
    fn absorb(&mut self) {
        let Some((x0, y0, x1, y1)) = self.wet.dirty else {
            self.wet.clock.mark = self.wet.current;
            return;
        };
        let w = self.f.w;
        let (x1, y1) = (x1.min(w), y1.min(self.f.h));
        let wet = &mut self.wet;
        let mark = wet.clock.mark;
        let (vol, touched, stroke) = (&wet.vol, &wet.touched, &wet.stroke);
        wet.clock.px[y0 * w..y1 * w].par_chunks_mut(w).enumerate().for_each(|(j, row)| {
            for x in x0..x1 {
                let i = (y0 + j) * w + x;
                let v = vol[i];
                let p = &mut row[x];
                if v < 1e-5 {
                    (p.cure, p.lev, p.seen) = (0.0, SET_TIME, 0.0);
                    continue;
                }
                if touched[i] > mark || stroke[i] > mark {
                    p.cure *= p.seen.min(v) / v;
                    p.lev = SET_TIME * fluid(p.cure);
                }
                p.seen = v;
            }
        });
        wet.clock.mark = wet.current;
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
        let mut sets = vec![SET_TIME; ew * eh];
        // cure per minute of each film that bakes (for its tack afterwards)
        let mut rates = vec![0.0f32; if all { 0 } else { ew * eh }];
        let mut any = false;
        let cp = &self.wet.clock.px;
        for y in 0..eh {
            for x in 0..ew {
                let i = (ex.1 + y) * w + ex.0 + x;
                let v = self.wet.vol[i];
                if v >= 1e-5 && (all || cp.get(i).is_some_and(|p| p.cure >= GEL)) {
                    let k = y * ew + x;
                    add[k] = v * COAT_UM;
                    stiff[k] = self.wet.hide[i][1];
                    if let Some(p) = cp.get(i) {
                        sets[k] = p.lev;
                    }
                    if !all {
                        rates[k] = rate(v, self.wet.hide[i][1], self.wet.hide[i][2]);
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
            }
            return;
        }
        let t = self.settle_for(ex, &add, &stiff, &sets);
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
                    px[x] = crate::wet::over_share(Pigment::masstone(c, hide[i][0]), px[x], ti, cover[k]);
                    ff[x] += ti;
                    vv[x] = 0.0;
                    cv[x] = 1.0;
                }
            });
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
                    (p.cure, p.lev, p.seen) = (0.0, SET_TIME, 0.0);
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
        band(&mut c, Paint::body(hex("#202020")).with_drying(drier::BONE_BLACK), 700.0, 2);
        assert_eq!(c.drying_at(500.0, 300.0), Stage::Open);
        c.wait(30.0);
        assert_eq!((c.drying_at(500.0, 300.0), c.drying_at(500.0, 700.0)), (Stage::Open, Stage::Open));
        c.wait(150.0);
        assert_eq!(c.drying_at(500.0, 300.0), Stage::Tacky, "lead white sets within 3 h");
        assert!(matches!(c.drying_at(500.0, 700.0), Stage::Open | Stage::Setting), "bone black is still wet at 3 h");
        c.wait(21.0 * 60.0);
        assert_eq!(c.drying_at(500.0, 300.0), Stage::Dry, "lead white is touch-dry the next day");
        assert_ne!(c.drying_at(500.0, 700.0), Stage::Dry, "bone black is not");
        assert!((c.clock() - 24.0 * 60.0).abs() < 1e-3);
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
        b.wait(3.0 * 24.0 * 60.0);
        b.dry();
        assert!(a.px == b.px && a.height == b.height && a.film == b.film);
    }

    /// A clean brush lifts wet paint, less as it sets, and nothing from set
    /// or dry paint; a tacky surface pulls paint off a loaded brush sooner
    /// than a dry one (more of it lands early in the stroke).
    #[test]
    fn brushes_feel_the_stage() {
        let mut lifted = Vec::new();
        let mut laid = Vec::new();
        for wait in [0.0, 60.0, 180.0, 36.0 * 60.0] {
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
        // (at 3 h only the thickest ridges of the umber are still open)
        assert!(lifted[2] < 0.05 * lifted[0] && lifted[3] == 0.0, "nothing lifts from set paint: {lifted:?}");
        assert!(laid[2] > laid[3] * 1.2, "tack pulls paint off the brush: {laid:?}");
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
}
