//! Drawn outlines: a few rough points become a line a hand drew.
//!
//! A painter places a handful of points (marking which are corners, or
//! letting the hand decide from the angle) and gets back:
//!
//! - the line: a centripetal Catmull-Rom curve through the points, broken at
//!   the corners, then moved in and out along its normal the way a hand
//!   moves: a slow wobble, and depending on the character, straight facets
//!   with small notches (a broken rock edge) or rounded lobes (a leafy edge);
//! - how the hand drew it: strokes that lift and start again, overlap or
//!   leave small gaps, overshoot at corners, restate a stretch slightly off
//!   the line (a searching sketch), each with its own pressure along it;
//! - a mask whose edge is that same line (crisp, or soft in places);
//! - helpers: an offset or inset outline, a band along the line, the region
//!   below or above an open line, and a silhouette grown from a skeleton
//!   (a spine and limbs with widths: a sheep from five points).
//!
//! It's geometry only: nothing is painted until a brush is dragged along
//! the strokes (`Outline::paint`). Everything is deterministic by seed and
//! independent of the canvas resolution (the fields that grow bodies and
//! offsets are sampled on their own grid in canvas units).

use crate::bristle::{Gesture, Held, Orient};
use crate::canvas::{Canvas, Frame};
use crate::mask::Mask;
use crate::noise::Fbm;
use crate::path::{arclen, dist, length};
use crate::rng::Rng;
use crate::shape::Shape;
use rayon::prelude::*;
use std::collections::HashMap;
use std::f32::consts::{PI, TAU};

pub type P = (f32, f32);

/// How a hand draws a line. Lengths are fractions of the hand's scale
/// (`Outline::scale`, which grows a little slower than the drawing's size:
/// a small thing is drawn with the fingers, a big one with the arm).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Character {
    /// Slow wobble of the line: amplitude and wavelength.
    pub wobble: f32,
    pub wobble_period: f32,
    /// Straight facets with small kinks between them (a broken edge): mean
    /// facet length (0 = none) and how far the kinks sit off the curve.
    pub facet: f32,
    pub facet_amp: f32,
    /// Chance that a kink is a notch: a small step in, as if a chip broke off.
    pub notch: f32,
    /// Rounded lobes bulging out (a leafy edge): mean lobe width (0 = none)
    /// and lobe height as a fraction of the width.
    pub lobe: f32,
    pub lobe_height: f32,
    /// How long the hand draws before it lifts (min, max).
    pub stroke_len: (f32, f32),
    /// Chance a lift leaves a gap (otherwise the next stroke overlaps the
    /// last a little), and the size of gaps and overlaps.
    pub gap: f32,
    pub gap_len: f32,
    /// Chance of running past a corner, and how far.
    pub overshoot: f32,
    pub overshoot_len: f32,
    /// Restatements: extra strokes per stroke laid over part of the line,
    /// and how far any stroke drifts off the line.
    pub restate: f32,
    pub restate_off: f32,
    /// Pressure: mean, variation along the line, and extra weight where the
    /// line faces down (the shadowed underside a draftsman presses into).
    pub pressure: f32,
    pub pressure_var: f32,
    pub underside: f32,
    /// Attack and release of each stroke (fractions of its length).
    pub ramps: (f32, f32),
    /// The mask's edge: softness (fraction of the scale; 0 = crisp) and how
    /// much it varies along the edge (0..1: 1 = lost in places).
    pub edge: f32,
    pub edge_var: f32,
    /// Multiplier on every displacement amplitude (wobble, facets and
    /// notches, lobe height, restatement drift, overshoot), set by
    /// `amount`. Applied when the line is drawn, so setting a lobe or a
    /// facet after `amount` is still scaled by it (0: a clean curve).
    pub irregularity: f32,
}

impl Character {
    /// A sure contour: little wobble, long strokes, slight overshoots.
    pub fn firm() -> Self {
        Character {
            wobble: 0.009,
            wobble_period: 0.3,
            facet: 0.0,
            facet_amp: 0.0,
            notch: 0.0,
            lobe: 0.0,
            lobe_height: 0.0,
            stroke_len: (0.35, 0.8),
            gap: 0.15,
            gap_len: 0.012,
            overshoot: 0.45,
            overshoot_len: 0.02,
            restate: 0.0,
            restate_off: 0.002,
            pressure: 0.75,
            pressure_var: 0.2,
            underside: 0.15,
            ramps: (0.08, 0.2),
            edge: 0.0,
            edge_var: 0.0,
            irregularity: 1.0,
        }
    }

    /// A sketch line feeling for the form: short strokes, restated slightly
    /// off each other, running past corners.
    pub fn searching() -> Self {
        Character {
            wobble: 0.01,
            wobble_period: 0.18,
            stroke_len: (0.1, 0.28),
            gap: 0.3,
            gap_len: 0.012,
            overshoot: 0.85,
            overshoot_len: 0.045,
            restate: 0.7,
            restate_off: 0.008,
            pressure: 0.55,
            pressure_var: 0.35,
            underside: 0.2,
            ramps: (0.15, 0.3),
            edge: 0.004,
            edge_var: 0.5,
            ..Self::firm()
        }
    }

    /// A broken edge (rock, stone, bark): straight facets, kinks and chips,
    /// short strokes with gaps.
    pub fn broken() -> Self {
        Character {
            wobble: 0.006,
            wobble_period: 0.25,
            facet: 0.045,
            facet_amp: 0.009,
            notch: 0.18,
            stroke_len: (0.06, 0.22),
            gap: 0.45,
            gap_len: 0.01,
            overshoot: 0.3,
            overshoot_len: 0.015,
            restate: 0.1,
            restate_off: 0.003,
            pressure: 0.7,
            pressure_var: 0.35,
            underside: 0.25,
            ramps: (0.05, 0.15),
            edge: 0.0,
            edge_var: 0.0,
            ..Self::firm()
        }
    }

    /// A soft edge (foliage, wool, a far tree line): rounded lobes, light
    /// short strokes with gaps, a mask edge that softens and is lost in places.
    pub fn soft() -> Self {
        Character {
            wobble: 0.01,
            wobble_period: 0.3,
            lobe: 0.06,
            lobe_height: 0.35,
            stroke_len: (0.04, 0.14),
            gap: 0.5,
            gap_len: 0.01,
            overshoot: 0.0,
            overshoot_len: 0.0,
            restate: 0.2,
            restate_off: 0.004,
            pressure: 0.45,
            pressure_var: 0.4,
            underside: 0.1,
            ramps: (0.2, 0.3),
            edge: 0.012,
            edge_var: 0.8,
            ..Self::firm()
        }
    }

    pub fn named(name: &str) -> Option<Self> {
        Some(match name {
            "firm" => Self::firm(),
            "searching" | "sketch" => Self::searching(),
            "broken" | "rock" => Self::broken(),
            "soft" | "foliage" => Self::soft(),
            _ => return None,
        })
    }

    /// The same hand, more (k > 1) or less (k < 1) irregular: multiplies
    /// `irregularity`, so it scales lobes and facets set before or after
    /// it alike.
    pub fn amount(mut self, k: f32) -> Self {
        self.irregularity *= k.max(0.0);
        self
    }

    /// The amplitudes as drawn: `irregularity` multiplied in (and reset to 1).
    pub fn applied(mut self) -> Self {
        let k = self.irregularity.max(0.0);
        self.wobble *= k;
        self.facet_amp *= k;
        self.lobe_height *= k;
        self.restate_off *= k;
        self.overshoot_len *= k;
        self.notch = (self.notch * k).min(1.0);
        self.irregularity = 1.0;
        self
    }
}

/// One drawn line: dense points about `step` apart (a closed line's last
/// point joins its first), the pressure at each and the corners.
#[derive(Clone, Debug)]
pub struct Line {
    pub pts: Vec<P>,
    pub pressure: Vec<f32>,
    pub closed: bool,
    /// Indices into `pts`.
    pub corners: Vec<usize>,
}

/// One movement of the hand: points and the pressure at each.
#[derive(Clone, Debug)]
pub struct Stroke {
    pub pts: Vec<P>,
    pub pressure: Vec<f32>,
}

/// A limb of a skeleton: points along it and the full width at each.
#[derive(Clone, Debug)]
pub struct Bone {
    pub pts: Vec<P>,
    pub widths: Vec<f32>,
}

#[derive(Clone, Debug)]
pub struct Outline {
    pub lines: Vec<Line>,
    pub strokes: Vec<Stroke>,
    pub ch: Character,
    /// The hand's scale in canvas units (what `Character`'s fractions are of).
    pub scale: f32,
    pub seed: u64,
}

/// The scale a hand works at for a drawing `size` units across: grows
/// sublinearly (equal to the size at 100 units).
pub fn hand_scale(size: f32) -> f32 {
    size.max(1.0).powf(0.7) * 100f32.powf(0.3)
}


fn bbox(pts: impl Iterator<Item = P>) -> (f32, f32, f32, f32) {
    pts.fold((f32::MAX, f32::MAX, f32::MIN, f32::MIN), |b, p| (b.0.min(p.0), b.1.min(p.1), b.2.max(p.0), b.3.max(p.1)))
}

/// Twice the signed area (> 0: clockwise on screen, y down).
fn area2(pts: &[P]) -> f32 {
    let n = pts.len();
    (0..n).map(|i| pts[i].0 * pts[(i + 1) % n].1 - pts[(i + 1) % n].0 * pts[i].1).sum()
}

/// A point on the centripetal Catmull-Rom segment from p1 to p2 (Barry and
/// Goldman's pyramid, alpha = 1/2: no cusps or loops within a segment).
fn cr(p0: P, p1: P, p2: P, p3: P, u: f32) -> P {
    let k = |a: P, b: P| dist(a, b).sqrt().max(1e-4);
    let t1 = k(p0, p1);
    let t2 = t1 + k(p1, p2);
    let t3 = t2 + k(p2, p3);
    let t = t1 + u * (t2 - t1);
    let l = |a: P, b: P, ta: f32, tb: f32| {
        let (wa, wb) = ((tb - t) / (tb - ta), (t - ta) / (tb - ta));
        (a.0 * wa + b.0 * wb, a.1 * wa + b.1 * wb)
    };
    let a1 = l(p0, p1, 0.0, t1);
    let a2 = l(p1, p2, t1, t2);
    let a3 = l(p2, p3, t2, t3);
    let b1 = l(a1, a2, 0.0, t2);
    let b2 = l(a2, a3, t1, t3);
    l(b1, b2, t1, t2)
}

/// A smooth curve through `q`, finely sampled. Open: the ends continue
/// straight (phantom points mirrored); `periodic`: the curve closes.
fn spline(q: &[P], periodic: bool, step: f32) -> Vec<P> {
    let n = q.len();
    if n < 2 {
        return q.to_vec();
    }
    let at = |i: isize| -> P {
        if periodic {
            q[i.rem_euclid(n as isize) as usize]
        } else if i < 0 {
            (2.0 * q[0].0 - q[1].0, 2.0 * q[0].1 - q[1].1)
        } else if i as usize >= n {
            (2.0 * q[n - 1].0 - q[n - 2].0, 2.0 * q[n - 1].1 - q[n - 2].1)
        } else {
            q[i as usize]
        }
    };
    let segs = if periodic { n } else { n - 1 };
    let mut out = Vec::new();
    for i in 0..segs as isize {
        let (p0, p1, p2, p3) = (at(i - 1), at(i), at(i + 1), at(i + 2));
        let m = ((dist(p1, p2) / (step * 0.5)).ceil() as usize).clamp(1, 4000);
        for j in 0..m {
            out.push(cr(p0, p1, p2, p3, j as f32 / m as f32));
        }
    }
    if !periodic {
        out.push(q[n - 1]);
    }
    out
}

/// Points `step` apart along a polyline (a closed one wraps; its last point
/// then stops one step short of the first).
fn resample(pts: &[P], closed: bool, step: f32) -> Vec<P> {
    let mut p = pts.to_vec();
    if closed && p.len() > 1 {
        p.push(p[0]);
    }
    if p.len() < 2 {
        return pts.to_vec();
    }
    let cum = arclen(&p);
    let total = *cum.last().unwrap();
    if total < 1e-6 {
        return vec![p[0]];
    }
    let n = ((total / step).round() as usize).max(if closed { 3 } else { 1 });
    let count = if closed { n } else { n + 1 };
    let mut out = Vec::with_capacity(count);
    let mut k = 0;
    for i in 0..count {
        let s = total * i as f32 / n as f32;
        while k + 2 < cum.len() && cum[k + 1] < s {
            k += 1;
        }
        let seg = (cum[k + 1] - cum[k]).max(1e-9);
        let t = ((s - cum[k]) / seg).clamp(0.0, 1.0);
        out.push((p[k].0 + (p[k + 1].0 - p[k].0) * t, p[k].1 + (p[k + 1].1 - p[k].1) * t));
    }
    out
}

/// Turning angle (radians) at each point of a polyline over `k` neighbors.
fn turning(pts: &[P], closed: bool, k: usize) -> Vec<f32> {
    let n = pts.len();
    (0..n)
        .map(|i| {
            let (a, b) = if closed {
                (pts[(i + n - k % n) % n], pts[(i + k) % n])
            } else {
                if i < k || i + k >= n {
                    return 0.0;
                }
                (pts[i - k], pts[i + k])
            };
            let p = pts[i];
            let (u, v) = ((p.0 - a.0, p.1 - a.1), (b.0 - p.0, b.1 - p.1));
            let (lu, lv) = ((u.0 * u.0 + u.1 * u.1).sqrt(), (v.0 * v.0 + v.1 * v.1).sqrt());
            if lu < 1e-6 || lv < 1e-6 {
                return 0.0;
            }
            ((u.0 * v.0 + u.1 * v.1) / (lu * lv)).clamp(-1.0, 1.0).acos()
        })
        .collect()
}

/// Which of a painter's points are corners, judged by the angle the line
/// turns there (degrees). A line's two ends are always corners.
pub fn auto_corners(pts: &[P], closed: bool, degrees: f32) -> Vec<bool> {
    let n = pts.len();
    let t = turning(pts, closed, 1);
    (0..n).map(|i| (!closed && (i == 0 || i + 1 == n)) || t[i] > degrees.to_radians()).collect()
}

/// Corners of a dense line: local maxima of the turning over `k` points
/// above `degrees`.
fn dense_corners(pts: &[P], closed: bool, k: usize, degrees: f32) -> Vec<usize> {
    let n = pts.len();
    let t = turning(pts, closed, k);
    let th = degrees.to_radians();
    let mut out = Vec::new();
    for i in 0..n {
        if t[i] <= th {
            continue;
        }
        let peak = (1..=k).all(|j| {
            let (a, b) = if closed { ((i + n - j % n) % n, (i + j) % n) } else { (i.saturating_sub(j), (i + j).min(n - 1)) };
            t[i] >= t[a] && t[i] > t[b]
        });
        if peak {
            out.push(i);
        }
    }
    out
}

/// 1-D fractal noise along a line, about -1..1. Closed lines sample it
/// around a circle of the same length, so it has no seam.
struct Along {
    n: Fbm,
    total: f32,
    closed: bool,
}

impl Along {
    fn new(seed: u32, octaves: usize, period: f32, total: f32, closed: bool) -> Self {
        Along { n: Fbm::new(seed, octaves, period.max(1e-3)), total, closed }
    }
    fn get(&self, s: f32) -> f32 {
        if self.closed {
            let r = self.total / TAU;
            let a = s / self.total.max(1e-6) * TAU;
            1.4 * self.n.get(r * a.cos() + 7.3, r * a.sin() - 3.1)
        } else {
            1.4 * self.n.get(s, 11.7)
        }
    }
}

/// Knots with lognormal spacing (median `mean`, spread `sigma`).
fn knots_ln(rng: &mut Rng, total: f32, mean: f32, sigma: f32) -> Vec<f32> {
    let mut s = vec![0.0];
    let mut x = 0.0;
    loop {
        x += mean * (sigma * rng.normal()).exp().clamp(0.3, 3.0);
        if x >= total - mean * 0.4 {
            break;
        }
        s.push(x);
    }
    s.push(total);
    s
}

/// Piecewise-linear interpolation of knot values.
fn knot_value(ks: &[f32], vs: &[f32], s: f32) -> f32 {
    let i = ks.partition_point(|&k| k <= s).clamp(1, ks.len() - 1);
    let (a, b) = (ks[i - 1], ks[i]);
    let t = ((s - a) / (b - a).max(1e-6)).clamp(0.0, 1.0);
    vs[i - 1] + (vs[i] - vs[i - 1]) * t
}

/// Unit tangents from neighbors `k` apart.
fn tangents(pts: &[P], closed: bool, k: usize) -> Vec<P> {
    let n = pts.len();
    (0..n)
        .map(|i| {
            let (a, b) = if closed { (pts[(i + n - k % n) % n], pts[(i + k) % n]) } else { (pts[i.saturating_sub(k)], pts[(i + k).min(n - 1)]) };
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let l = (dx * dx + dy * dy).sqrt().max(1e-9);
            (dx / l, dy / l)
        })
        .collect()
}

/// Outward normals: a closed line runs with its inside on the left, so the
/// outside is on the right; an open line's outside is on its left (up, for
/// a line drawn left to right).
fn normals(pts: &[P], closed: bool, k: usize) -> Vec<P> {
    tangents(pts, closed, k).into_iter().map(|(tx, ty)| if closed { (-ty, tx) } else { (ty, -tx) }).collect()
}

/// A few passes of neighbor averaging (keeps the ends of open lines and
/// the given corners in place).
fn relax(pts: &mut [P], closed: bool, passes: usize, keep: &[usize]) {
    let n = pts.len();
    if n < 3 {
        return;
    }
    for _ in 0..passes {
        let old = pts.to_vec();
        for i in 0..n {
            if (!closed && (i == 0 || i + 1 == n)) || keep.contains(&i) {
                continue;
            }
            let (a, b) = (old[(i + n - 1) % n], old[(i + 1) % n]);
            pts[i] = (0.25 * a.0 + 0.5 * old[i].0 + 0.25 * b.0, 0.25 * a.1 + 0.5 * old[i].1 + 0.25 * b.1);
        }
    }
}

/// A path the hand is asked to draw: dense points, closed or not, corners.
struct Plan {
    pts: Vec<P>,
    closed: bool,
    corners: Vec<usize>,
    /// Half the local thickness of the shape at each point (empty: ample).
    /// Lobes and facets shrink where it is thin (a leg, a neck).
    room: Vec<f32>,
}

impl Outline {
    /// Draw through a painter's points. `corners[i]` marks point i as a
    /// corner (empty: decided from the angle, over 60°). Open lines always
    /// have corners at their ends. `size`: the hand's scale in units
    /// (None: from the points' extent).
    pub fn draw(pts: &[P], corners: &[bool], closed: bool, ch: Character, seed: u64, size: Option<f32>) -> Outline {
        let mut q: Vec<P> = pts.to_vec();
        q.dedup_by(|a, b| dist(*a, *b) < 1e-4);
        let mut cor: Vec<bool> = if corners.len() == pts.len() && q.len() == pts.len() { corners.to_vec() } else { auto_corners(&q, closed, 60.0) };
        if closed && q.len() > 2 && area2(&q) > 0.0 {
            // run with the inside on the left
            q.reverse();
            cor.reverse();
        }
        let (x0, y0, x1, y1) = bbox(q.iter().copied());
        let scale = size.unwrap_or_else(|| hand_scale(((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt()));
        let step = (scale / 400.0).clamp(0.1, 1.0);
        let n = q.len();
        if n < 2 {
            return Outline { lines: Vec::new(), strokes: Vec::new(), ch, scale, seed };
        }
        let plan = if closed && n >= 3 && !cor.iter().any(|&c| c) {
            Plan { pts: resample(&spline(&q, true, step), true, step), closed: true, corners: Vec::new(), room: Vec::new() }
        } else {
            // spans from corner to corner, each its own smooth curve
            let mut idx: Vec<usize> = (0..n).filter(|&i| cor[i]).collect();
            if !closed {
                idx.retain(|&i| i != 0 && i != n - 1);
                idx.insert(0, 0);
                idx.push(n - 1);
            } else if n < 3 {
                idx = vec![0, 1];
            }
            let mut spans: Vec<Vec<P>> = Vec::new();
            let m = idx.len();
            let nspans = if closed { m } else { m - 1 };
            for s in 0..nspans {
                let (a, b) = (idx[s], idx[(s + 1) % m]);
                let mut span = vec![q[a]];
                let mut i = a;
                loop {
                    i = (i + 1) % n;
                    span.push(q[i]);
                    if i == b {
                        break;
                    }
                }
                spans.push(span);
            }
            // a hand's "straight" line from corner to corner bows a little
            let mut brng = Rng::new(seed ^ 0xB0B0_5EED);
            let bow = 0.025 * (ch.applied().wobble / 0.01).min(2.0);
            for span in spans.iter_mut().filter(|s| s.len() == 2) {
                let (a, b) = (span[0], span[1]);
                let k = brng.normal() * bow;
                let mid = (0.5 * (a.0 + b.0) - (b.1 - a.1) * k, 0.5 * (a.1 + b.1) + (b.0 - a.0) * k);
                span.insert(1, mid);
            }
            let mut pts = Vec::new();
            let mut corners = Vec::new();
            for span in &spans {
                let d = resample(&spline(span, false, step), false, step);
                if closed || !pts.is_empty() {
                    corners.push(pts.len());
                }
                pts.extend_from_slice(&d[..d.len() - 1]);
            }
            if !closed {
                pts.push(*spans.last().unwrap().last().unwrap());
                corners.retain(|&c| c > 0);
            }
            Plan { pts, closed, corners, room: Vec::new() }
        };
        Self::from_plans(vec![plan], ch, seed, scale, step)
    }

    /// A silhouette grown from a skeleton: the first limb is the spine, the
    /// others (legs, neck, arms) join it smoothly over `blend` times their
    /// width. Every limb is a smooth curve through its points, as wide as
    /// `widths` there.
    pub fn body(limbs: &[Bone], blend: f32, ch: Character, seed: u64, size: Option<f32>) -> Outline {
        // a limb that starts outside the spine reaches into it (a leg drawn
        // a little below the belly still joins the body)
        let limbs: Vec<Bone> = limbs
            .iter()
            .enumerate()
            .map(|(k, l)| {
                let mut l = l.clone();
                let spine = &limbs[0];
                if k == 0 || l.pts.is_empty() || spine.pts.is_empty() {
                    return l;
                }
                let r = l.widths.first().copied().unwrap_or(1.0) * 0.5;
                let s0 = l.pts[0];
                // nearest point of the spine's center line and its radius there
                let mut best = (f32::MAX, s0, 0.0);
                for i in 0..spine.pts.len() {
                    let a = spine.pts[i];
                    let b = spine.pts[(i + 1).min(spine.pts.len() - 1)];
                    let (wa, wb) = (spine.widths.get(i).or(spine.widths.last()).copied().unwrap_or(1.0), spine.widths.get(i + 1).or(spine.widths.last()).copied().unwrap_or(1.0));
                    let (bx, by) = (b.0 - a.0, b.1 - a.1);
                    let h = (((s0.0 - a.0) * bx + (s0.1 - a.1) * by) / (bx * bx + by * by).max(1e-12)).clamp(0.0, 1.0);
                    let c = (a.0 + bx * h, a.1 + by * h);
                    let d = dist(s0, c);
                    if d < best.0 {
                        best = (d, c, 0.5 * (wa + (wb - wa) * h));
                    }
                }
                let (d, c, rs) = best;
                if d > rs - r * 0.5 && d > 1e-4 {
                    let reach = d - rs + r;
                    let u = ((c.0 - s0.0) / d, (c.1 - s0.1) / d);
                    l.pts.insert(0, (s0.0 + u.0 * reach, s0.1 + u.1 * reach));
                    let w0 = l.widths.first().copied().unwrap_or(1.0);
                    l.widths.insert(0, w0);
                }
                l
            })
            .collect();
        // each limb as a chain of short tapered capsules
        let mut caps: Vec<Vec<(P, P, f32, f32)>> = Vec::new();
        let mut min_r = f32::MAX;
        let mut all = Vec::new();
        for l in &limbs {
            if l.pts.is_empty() {
                continue;
            }
            let w = |i: usize| l.widths.get(i).or(l.widths.last()).copied().unwrap_or(1.0).max(0.02) * 0.5;
            let n = l.pts.len();
            let mut c = Vec::new();
            if n == 1 {
                c.push((l.pts[0], l.pts[0], w(0), w(0)));
            }
            for i in 0..n.saturating_sub(1) {
                let (p0, p1, p2, p3) = (
                    if i == 0 { (2.0 * l.pts[0].0 - l.pts[1].0, 2.0 * l.pts[0].1 - l.pts[1].1) } else { l.pts[i - 1] },
                    l.pts[i],
                    l.pts[i + 1],
                    if i + 2 < n { l.pts[i + 2] } else { (2.0 * l.pts[n - 1].0 - l.pts[n - 2].0, 2.0 * l.pts[n - 1].1 - l.pts[n - 2].1) },
                );
                let k = ((dist(p1, p2) / (w(i).min(w(i + 1)) * 0.5)).ceil() as usize).clamp(2, 200);
                let mut prev = p1;
                for j in 1..=k {
                    let u = j as f32 / k as f32;
                    let p = cr(p0, p1, p2, p3, u);
                    let (ra, rb) = (w(i) + (w(i + 1) - w(i)) * (j - 1) as f32 / k as f32, w(i) + (w(i + 1) - w(i)) * u);
                    c.push((prev, p, ra, rb));
                    prev = p;
                }
            }
            for &(a, b, ra, rb) in &c {
                min_r = min_r.min(ra.min(rb));
                all.push((a.0 - ra, a.1 - ra));
                all.push((b.0 + rb, b.1 + rb));
                all.push((a.0 + ra, a.1 + ra));
                all.push((b.0 - rb, b.1 - rb));
            }
            caps.push(c);
        }
        if caps.is_empty() {
            return Outline { lines: Vec::new(), strokes: Vec::new(), ch, scale: 1.0, seed };
        }
        let (x0, y0, x1, y1) = bbox(all.into_iter());
        let scale = size.unwrap_or_else(|| hand_scale(((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt()));
        let step = (scale / 400.0).clamp(0.1, 1.0);
        let g = (scale / 250.0).min(min_r * 0.6).max(0.02);
        let mean_r: Vec<f32> = caps.iter().map(|c| c.iter().map(|q| q.2 + q.3).sum::<f32>() / (2.0 * c.len() as f32)).collect();
        let sd = |x: f32, y: f32| -> f32 {
            let mut acc = f32::MAX;
            for (li, c) in caps.iter().enumerate() {
                let mut d = f32::MAX;
                for &(a, b, ra, rb) in c {
                    let (bx, by) = (b.0 - a.0, b.1 - a.1);
                    let (px, py) = (x - a.0, y - a.1);
                    let h = ((px * bx + py * by) / (bx * bx + by * by).max(1e-12)).clamp(0.0, 1.0);
                    let e = ((px - bx * h).powi(2) + (py - by * h).powi(2)).sqrt() - (ra + (rb - ra) * h);
                    d = d.min(e);
                }
                acc = if li == 0 { d } else { smin(acc, d, blend * mean_r[li]) };
            }
            acc
        };
        let pad = 3.0 * g + blend * mean_r.iter().cloned().fold(0.0, f32::max);
        let loops = contour_of(x0 - pad, y0 - pad, x1 + pad, y1 + pad, g, |x, y| -sd(x, y), 0.0);
        let mut plans = loops_to_plans(loops, step, scale);
        // room: the radius of the limb nearest each point of the contour
        for pl in plans.iter_mut() {
            pl.room = pl
                .pts
                .par_iter()
                .map(|&(x, y)| {
                    let mut best = (f32::MAX, 0.0);
                    for c in &caps {
                        for &(a, b, ra, rb) in c {
                            let (bx, by) = (b.0 - a.0, b.1 - a.1);
                            let (px, py) = (x - a.0, y - a.1);
                            let h = ((px * bx + py * by) / (bx * bx + by * by).max(1e-12)).clamp(0.0, 1.0);
                            let r = ra + (rb - ra) * h;
                            let e = (((px - bx * h).powi(2) + (py - by * h).powi(2)).sqrt() - r).abs();
                            if e < best.0 {
                                best = (e, r);
                            }
                        }
                    }
                    best.1
                })
                .collect();
        }
        Self::from_plans(plans, ch, seed, scale, step)
    }

    /// A line `d` units outside this one (inside, if negative), drawn by the
    /// same hand with its irregularity scaled by `amount` (the parallel line
    /// keeps this one's shape; 0.4 adds a little of its own). Closed lines
    /// only (open ones are shifted along their normals).
    pub fn offset(&self, d: f32, amount: f32) -> Outline {
        let ch = self.ch.amount(amount);
        let seed = self.seed.wrapping_mul(0x9E3779B97F4A7C15).wrapping_add((d * 1000.0) as i64 as u64);
        let step = (self.scale / 400.0).clamp(0.1, 1.0);
        let mut plans = Vec::new();
        let closed: Vec<&Line> = self.lines.iter().filter(|l| l.closed).collect();
        if !closed.is_empty() {
            let segs: Vec<(P, P)> = closed.iter().flat_map(|l| (0..l.pts.len()).map(|i| (l.pts[i], l.pts[(i + 1) % l.pts.len()]))).collect();
            let (x0, y0, x1, y1) = bbox(closed.iter().flat_map(|l| l.pts.iter().copied()));
            let g = (self.scale / 300.0).clamp(0.03, 1.0).min(d.abs().max(0.1) * 0.5);
            let pad = d.max(0.0) + 3.0 * g;
            let loops = contour_of(x0 - pad, y0 - pad, x1 + pad, y1 + pad, g, |x, y| signed_distance(&segs, x, y), -d);
            plans.extend(loops_to_plans(loops, step, self.scale));
        }
        for l in self.lines.iter().filter(|l| !l.closed) {
            let nn = normals(&l.pts, false, 2);
            let pts: Vec<P> = l.pts.iter().zip(&nn).map(|(p, n)| (p.0 + n.0 * d, p.1 + n.1 * d)).collect();
            plans.push(Plan { pts: resample(&pts, false, step), closed: false, corners: Vec::new(), room: Vec::new() });
        }
        Self::from_plans(plans, ch, seed, self.scale, step)
    }

    /// The hand at work on dense plans: displace, weigh, plan strokes.
    fn from_plans(plans: Vec<Plan>, ch: Character, seed: u64, scale: f32, step: f32) -> Outline {
        let drawn = ch.applied();
        let mut rng = Rng::new(seed ^ 0xD1B54A32D192ED03);
        let mut lines = Vec::new();
        for (li, plan) in plans.into_iter().enumerate() {
            if plan.pts.len() < 2 {
                continue;
            }
            let s32 = (seed as u32).wrapping_add(li as u32 * 7919);
            lines.push(hand_line(plan, &drawn, s32, scale, step, &mut rng));
        }
        let mut strokes = Vec::new();
        for l in &lines {
            plan_strokes(l, &drawn, scale, step, &mut rng, &mut strokes);
        }
        Outline { lines, strokes, ch, scale, seed }
    }

    /// True if this outline has lines and none of them is closed (so it
    /// bounds no region: use `below` or `above`). An outline with no lines
    /// at all (an inset that consumed its shape) is not open: its `mask` is
    /// empty.
    pub fn is_open(&self) -> bool {
        !self.lines.is_empty() && !self.lines.iter().any(|l| l.closed)
    }

    /// The region inside the closed lines, its edge exactly the drawn line,
    /// softened as the character says (in places, if `edge_var` > 0).
    /// Empty if there are none (an inset deeper than the shape is thick).
    pub fn mask(&self, f: Frame) -> Mask {
        let mut s = Shape::new();
        for l in self.lines.iter().filter(|l| l.closed) {
            s = s.poly(&l.pts);
        }
        let m = Mask::from_shape(f, s);
        self.edge(m)
    }

    /// Soften a mask's edge as this outline's character says.
    fn edge(&self, m: Mask) -> Mask {
        let e = self.ch.edge * self.scale;
        if e <= 0.0 {
            return m;
        }
        let var = Fbm::new(self.seed as u32 ^ 0x5eed, 3, self.scale * 0.15);
        let k = self.ch.edge_var;
        m.soften(move |x, y| e * (1.0 + k * 1.3 * var.get(x, y)).max(0.1))
    }

    /// Everything below an open line (down to `bottom`; the line's ends
    /// drop straight down), edged like `mask`.
    pub fn below(&self, f: Frame, bottom: f32) -> Mask {
        let mut s = Shape::new();
        for l in &self.lines {
            if l.closed {
                s = s.poly(&l.pts);
            } else {
                s = s.below(&l.pts, bottom);
            }
        }
        self.edge(Mask::from_shape(f, s))
    }

    /// Everything above an open line (up to the top edge).
    pub fn above(&self, f: Frame) -> Mask {
        let mut s = Shape::new();
        for l in &self.lines {
            if l.closed {
                s = s.poly(&l.pts);
            } else {
                s = s.below(&l.pts, -1.0);
            }
        }
        self.edge(Mask::from_shape(f, s))
    }

    /// A band `width` units wide centered on the line (a rim, a crack, a
    /// shadow line along a contour). `taper`: 0 = even, 1 = as wide as the
    /// line's pressure (full width at the heaviest point).
    pub fn band(&self, f: Frame, width: f32, taper: f32) -> Mask {
        let mut s = Shape::new();
        let sp = (width * 0.4).max(0.2);
        for l in &self.lines {
            let pmax = l.pressure.iter().cloned().fold(1e-3, f32::max);
            let mut pts = l.pts.clone();
            let mut pr = l.pressure.clone();
            if l.closed {
                pts.push(pts[0]);
                pr.push(pr[0]);
            }
            // thin the points to about 0.4 widths apart
            let mut keep_p = Vec::new();
            let mut keep_w = Vec::new();
            let mut acc = f32::MAX;
            for i in 0..pts.len() {
                if i > 0 {
                    acc += dist(pts[i - 1], pts[i]);
                }
                if acc >= sp || i + 1 == pts.len() {
                    keep_p.push(pts[i]);
                    keep_w.push(width * (1.0 - taper + taper * pr[i] / pmax));
                    acc = 0.0;
                }
            }
            s = s.add(Shape::new().ribbon(&keep_p, &keep_w));
        }
        Mask::from_shape(f, s)
    }

    /// Total length of the lines.
    pub fn length(&self) -> f32 {
        self.lines.iter().map(line_len).sum()
    }

    /// Point, unit tangent and outward normal at fraction `t` of the first
    /// line's length.
    pub fn at(&self, t: f32) -> Option<(P, P, P)> {
        let l = self.lines.first()?;
        let n = l.pts.len();
        if n < 2 {
            return None;
        }
        let total = line_len(l);
        let want = t.clamp(0.0, 1.0) * total;
        let segs = if l.closed { n } else { n - 1 };
        let mut acc = 0.0;
        let tan = tangents(&l.pts, l.closed, 2);
        for i in 0..segs {
            let (a, b) = (l.pts[i], l.pts[(i + 1) % n]);
            let d = dist(a, b);
            if acc + d >= want || i + 1 == segs {
                let u = ((want - acc) / d.max(1e-9)).clamp(0.0, 1.0);
                let p = (a.0 + (b.0 - a.0) * u, a.1 + (b.1 - a.1) * u);
                let (t0, t1) = (tan[i], tan[(i + 1) % n]);
                let (tx, ty) = (t0.0 + (t1.0 - t0.0) * u, t0.1 + (t1.1 - t0.1) * u);
                let tl = (tx * tx + ty * ty).sqrt().max(1e-9);
                let tg = (tx / tl, ty / tl);
                let nm = if l.closed { (-tg.1, tg.0) } else { (tg.1, -tg.0) };
                return Some((p, tg, nm));
            }
            acc += d;
        }
        None
    }

    /// The strokes as gestures: pressure scaled by `pressure`, the stroke's
    /// own pressure as a swell along it.
    pub fn gestures(&self, pressure: f32, shake: f32) -> Vec<Gesture> {
        self.strokes
            .iter()
            .filter(|s| s.pts.len() >= 2)
            .map(|s| {
                let pm = s.pressure.iter().sum::<f32>() / s.pressure.len() as f32;
                let knots = 12.min(s.pressure.len());
                let swell: Vec<f32> = (0..knots)
                    .map(|k| {
                        let i = ((k as f32 / (knots - 1).max(1) as f32) * (s.pressure.len() - 1) as f32).round() as usize;
                        s.pressure[i] / pm.max(1e-3)
                    })
                    .collect();
                let p = (pm * pressure).clamp(0.0, 1.0);
                Gesture::new(s.pts.clone()).pressure(p, p).ramps(self.ch.ramps.0, self.ch.ramps.1).swell(swell).shake(shake).orient(Orient::Across)
            })
            .collect()
    }

    /// Drag a held brush along every stroke (a pointed brush makes the
    /// line swell and thin with the pressure).
    pub fn paint(&self, c: &mut Canvas, held: &mut Held, pressure: f32, clip: Option<&Mask>) {
        for g in self.gestures(pressure, 0.3) {
            c.drag(held, &g, clip);
        }
    }
}

fn line_len(l: &Line) -> f32 {
    let close = match (l.closed, l.pts.first(), l.pts.last()) {
        (true, Some(&a), Some(&b)) => dist(b, a),
        _ => 0.0,
    };
    length(&l.pts) + close
}

/// Polynomial smooth minimum over `k` units.
fn smin(a: f32, b: f32, k: f32) -> f32 {
    if k <= 1e-6 {
        return a.min(b);
    }
    let h = (k - (a - b).abs()).max(0.0) / k;
    a.min(b) - h * h * k * 0.25
}

/// Signed distance to closed polylines given as segments (+ inside, by the
/// even-odd rule).
fn signed_distance(segs: &[(P, P)], x: f32, y: f32) -> f32 {
    let mut d2 = f32::MAX;
    let mut inside = false;
    for &(a, b) in segs {
        let (bx, by) = (b.0 - a.0, b.1 - a.1);
        let (px, py) = (x - a.0, y - a.1);
        let h = ((px * bx + py * by) / (bx * bx + by * by).max(1e-12)).clamp(0.0, 1.0);
        d2 = d2.min((px - bx * h).powi(2) + (py - by * h).powi(2));
        if (a.1 > y) != (b.1 > y) && x < a.0 + (y - a.1) / (b.1 - a.1) * bx {
            inside = !inside;
        }
    }
    if inside { d2.sqrt() } else { -d2.sqrt() }
}

/// Closed contours where `field` crosses `level` (marching squares over a
/// grid of `g` units covering the box), each running with the region
/// `field > level` on its left. The box's border must lie outside.
fn contour_of(x0: f32, y0: f32, x1: f32, y1: f32, g: f32, field: impl Fn(f32, f32) -> f32 + Sync, level: f32) -> Vec<Vec<P>> {
    let nx = (((x1 - x0) / g).ceil() as usize + 1).clamp(2, 3000);
    let ny = (((y1 - y0) / g).ceil() as usize + 1).clamp(2, 3000);
    let gx = (x1 - x0) / (nx - 1) as f32;
    let gy = (y1 - y0) / (ny - 1) as f32;
    let v: Vec<f32> = (0..nx * ny).into_par_iter().map(|k| field(x0 + (k % nx) as f32 * gx, y0 + (k / nx) as f32 * gy) - level).collect();
    let ins = |i: usize, j: usize| v[j * nx + i] > 0.0;
    // edge ids: horizontal (i, j)-(i+1, j) = 2k, vertical (i, j)-(i, j+1) = 2k+1
    let hid = |i: usize, j: usize| 2 * (j * nx + i);
    let vid = |i: usize, j: usize| 2 * (j * nx + i) + 1;
    let point = |e: usize| -> P {
        let k = e / 2;
        let (i, j) = (k % nx, k / nx);
        let (i2, j2) = if e.is_multiple_of(2) { (i + 1, j) } else { (i, j + 1) };
        let (a, b) = (v[j * nx + i], v[j2 * nx + i2]);
        let t = (a / (a - b)).clamp(0.0, 1.0);
        (x0 + (i as f32 + (i2 as f32 - i as f32) * t) * gx, y0 + (j as f32 + (j2 as f32 - j as f32) * t) * gy)
    };
    let mut next: HashMap<usize, usize> = HashMap::new();
    for j in 0..ny - 1 {
        for i in 0..nx - 1 {
            let c = [ins(i, j), ins(i + 1, j), ins(i + 1, j + 1), ins(i, j + 1)];
            if c.iter().all(|&b| b) || c.iter().all(|&b| !b) {
                continue;
            }
            // perimeter clockwise on screen: top, right, bottom, left
            let edges = [hid(i, j), vid(i + 1, j), hid(i, j + 1), vid(i, j)];
            let mut cross: Vec<(usize, bool)> = Vec::new(); // (edge, entering)
            for k in 0..4 {
                let (a, b) = (c[k], c[(k + 1) % 4]);
                if a != b {
                    cross.push((edges[k], b));
                }
            }
            let m = cross.len();
            let center_in = (v[j * nx + i] + v[j * nx + i + 1] + v[(j + 1) * nx + i + 1] + v[(j + 1) * nx + i]) > 0.0;
            for k in 0..m {
                if !cross[k].1 {
                    continue;
                }
                // an entering crossing pairs with the exit after it, except
                // in a saddle whose center is inside (the exit before it)
                let e = if m == 4 && center_in { cross[(k + m - 1) % m].0 } else { cross[(k + 1) % m].0 };
                next.insert(cross[k].0, e);
            }
        }
    }
    let mut loops = Vec::new();
    let mut starts: Vec<usize> = next.keys().copied().collect();
    starts.sort_unstable();
    let mut seen = std::collections::HashSet::new();
    for s in starts {
        if seen.contains(&s) {
            continue;
        }
        let mut lp = Vec::new();
        let mut e = s;
        loop {
            if !seen.insert(e) {
                break;
            }
            lp.push(point(e));
            match next.get(&e) {
                Some(&n) => e = n,
                None => break,
            }
        }
        if lp.len() >= 4 {
            loops.push(lp);
        }
    }
    loops
}

/// Contours as plans: drop specks, smooth the grid out, resample, find corners.
fn loops_to_plans(loops: Vec<Vec<P>>, step: f32, scale: f32) -> Vec<Plan> {
    // specks go; so do small holes (a chink between an arm and a coat reads
    // as a buttonhole once it is outlined)
    let min_area = (scale * 0.01).powi(2);
    let min_hole = (scale * 0.08).powi(2);
    loops
        .into_iter()
        .filter(|l| {
            let a = area2(l) * 0.5;
            if a > 0.0 { a > min_hole } else { -a > min_area }
        })
        .map(|l| {
            let mut pts = resample(&l, true, step);
            relax(&mut pts, true, 3, &[]);
            let k = ((scale * 0.012 / step).round() as usize).max(2);
            let corners = dense_corners(&pts, true, k, 55.0);
            Plan { pts, closed: true, corners, room: Vec::new() }
        })
        .collect()
}

/// Move a planned path the way a hand moves and weigh it.
fn hand_line(plan: Plan, ch: &Character, seed: u32, scale: f32, step: f32, rng: &mut Rng) -> Line {
    let Plan { pts, closed, corners, room } = plan;
    // lobes and facets no bigger than the shape is thick there
    let fit = |i: usize, size: f32| -> f32 { room.get(i).map_or(1.0, |&r| (r / size.max(1e-6)).clamp(0.12, 1.0)) };
    let n = pts.len();
    let cum = arclen(&pts);
    let total = cum[n - 1] + if closed { dist(pts[n - 1], pts[0]) } else { 0.0 };
    let k = ((scale * 0.01 / step).round() as usize).clamp(1, n / 4 + 1);
    let nrm = normals(&pts, closed, k);
    let wob = Along::new(seed, 3, ch.wobble_period * scale, total, closed);
    let press = Along::new(seed.wrapping_add(101), 3, 0.2 * scale, total, closed);
    // facets: straight runs between kinks, some kinks chipped in
    let facet = (ch.facet > 0.0).then(|| {
        let ks = knots_ln(rng, total, ch.facet * scale, 0.5);
        let mut vs: Vec<f32> = ks.iter().map(|_| rng.normal() * ch.facet_amp * scale).collect();
        for v in vs.iter_mut() {
            if rng.f() < ch.notch {
                *v -= ch.facet_amp * scale * rng.range(1.2, 2.6);
            }
        }
        if closed {
            let l = vs.len();
            vs[l - 1] = vs[0];
        }
        (ks, vs, Along::new(seed.wrapping_add(505), 2, 0.3 * scale, total, closed))
    });
    // lobes: rounded bulges with pinched dips between
    // (two sizes: crowns and the smaller masses on them; sizes lognormal,
    // the big ones taller in groups)
    let lobes = (ch.lobe > 0.0).then(|| {
        let group = Along::new(seed.wrapping_add(303), 2, ch.lobe * scale * 4.0, total, closed);
        let mut layers = Vec::new();
        for (size, height) in [(1.0f32, 1.0f32), (0.32, 0.8)] {
            let ks = knots_ln(rng, total, ch.lobe * scale * size, 0.45);
            let hs: Vec<f32> = ks
                .windows(2)
                .map(|w| {
                    let g = if size == 1.0 { (1.0 + 0.6 * group.get(0.5 * (w[0] + w[1]))).max(0.2) } else { 1.0 };
                    (w[1] - w[0]) * ch.lobe_height * height * (0.35 * rng.normal()).exp() * g
                })
                .collect();
            let bs = lobe_troughs(&hs, closed);
            layers.push((ks, hs, bs));
        }
        layers
    });
    let mut out = Vec::with_capacity(n);
    let mut pressure = Vec::with_capacity(n);
    for i in 0..n {
        let s = cum[i];
        let mut d = ch.wobble * scale * wob.get(s);
        if let Some((ks, vs, env)) = &facet {
            // quiet runs and broken stretches, not an even tear
            d += knot_value(ks, vs, s) * (0.5 + 1.1 * env.get(s)).clamp(0.08, 1.8) * fit(i, ch.facet * scale);
        }
        for (ks, hs, bs) in lobes.iter().flatten() {
            d += lobe_offset(ks, hs, bs, s) * fit(i, ch.lobe * scale);
        }
        out.push((pts[i].0 + nrm[i].0 * d, pts[i].1 + nrm[i].1 * d));
        let p = ch.pressure * (1.0 + ch.pressure_var * press.get(s)) + ch.underside * nrm[i].1.max(0.0);
        pressure.push(p.clamp(0.05, 1.0));
    }
    Line { pts: out, pressure, closed, corners }
}

/// The dips between lobes of heights `hs` (one per interval between
/// knots): each knot's dip is shared by the lobes on either side (0.55 of
/// their mean height below the curve), so the line is continuous where
/// lobes of different heights meet; on a closed line the first and last
/// knot (the same point) share one.
fn lobe_troughs(hs: &[f32], closed: bool) -> Vec<f32> {
    let m = hs.len();
    if m == 0 {
        return vec![0.0];
    }
    let mut bs = Vec::with_capacity(m + 1);
    let ends = if closed { 0.5 * (hs[0] + hs[m - 1]) } else { hs[0] };
    bs.push(-0.55 * ends);
    for k in 1..m {
        bs.push(-0.55 * 0.5 * (hs[k - 1] + hs[k]));
    }
    bs.push(if closed { -0.55 * ends } else { -0.55 * hs[m - 1] });
    bs
}

/// Displacement at arc length `s` of the lobes on knots `ks`: the dips
/// `bs` joined straight, and on each interval a rounded bulge of height
/// `hs` that vanishes at both ends (so equal neighbors give the old
/// `h·(sin(πu)^0.6 − 0.55)` exactly and unequal ones meet without a step).
fn lobe_offset(ks: &[f32], hs: &[f32], bs: &[f32], s: f32) -> f32 {
    if hs.is_empty() {
        return 0.0;
    }
    let j = ks.partition_point(|&x| x <= s).clamp(1, ks.len() - 1);
    let u = ((s - ks[j - 1]) / (ks[j] - ks[j - 1]).max(1e-6)).clamp(0.0, 1.0);
    bs[j - 1] + (bs[j] - bs[j - 1]) * u + hs[j - 1] * (PI * u).sin().max(0.0).powf(0.6)
}

/// Plan how a hand draws a line: strokes between lifts, gaps and overlaps,
/// overshoots at corners, restatements.
fn plan_strokes(l: &Line, ch: &Character, scale: f32, step: f32, rng: &mut Rng, out: &mut Vec<Stroke>) {
    let n = l.pts.len();
    if n < 2 {
        return;
    }
    let steps = |len: f32| ((len / step).round() as isize).max(1);
    // lift points: every corner, and every stroke length between
    let start: usize = if l.closed { l.corners.first().copied().unwrap_or_else(|| (rng.f() * n as f32) as usize % n) } else { 0 };
    let span = if l.closed { n } else { n - 1 };
    let is_corner = |i: usize| l.corners.contains(&(i % n));
    let mut lifts: Vec<usize> = vec![0]; // offsets from `start`
    let mut next_lift = steps(scale * rng.range(ch.stroke_len.0, ch.stroke_len.1)) as usize;
    for o in 1..span {
        let i = (start + o) % n;
        if is_corner(i) || o >= next_lift {
            // near the end, don't leave a sliver
            if span - o > 3 {
                lifts.push(o);
            }
            next_lift = o + steps(scale * rng.range(ch.stroke_len.0, ch.stroke_len.1)) as usize;
        }
    }
    lifts.push(span);
    let tan = tangents(&l.pts, l.closed, 2);
    let nrm = normals(&l.pts, l.closed, 2);
    let idx = |o: isize| -> Option<usize> {
        if l.closed {
            Some((start as isize + o).rem_euclid(n as isize) as usize)
        } else if o >= 0 && (o as usize) < n {
            Some(o as usize)
        } else {
            None
        }
    };
    let gap = steps(ch.gap_len * scale);
    for w in lifts.windows(2) {
        let (a, b) = (w[0] as isize, w[1] as isize);
        // the join at `a`: a gap (start later) or an overlap (start earlier)
        let (mut a2, b2) = (a, b);
        if a > 0 || l.closed {
            if rng.f() < ch.gap {
                a2 += (gap as f32 * rng.range(0.4, 1.4)) as isize;
            } else {
                a2 -= (gap as f32 * rng.range(0.0, 1.0)) as isize;
            }
        }
        if b2 - a2 < 2 {
            continue;
        }
        let drift = (rng.normal() * ch.restate_off * scale, rng.normal() * ch.restate_off * scale);
        let pk = rng.range(0.85, 1.1);
        let mut s = Stroke { pts: Vec::new(), pressure: Vec::new() };
        let m = (b2 - a2) as f32;
        for o in a2..=b2 {
            let Some(i) = idx(o) else { continue };
            let u = (o - a2) as f32 / m;
            let off = drift.0 * (1.0 - u) + drift.1 * u;
            s.pts.push((l.pts[i].0 + nrm[i].0 * off, l.pts[i].1 + nrm[i].1 * off));
            s.pressure.push((l.pressure[i] * pk).min(1.0));
        }
        // overshoot past a corner (either end), curving off a little
        let over = |rng: &mut Rng, at: usize, dir: f32, s: &mut Stroke, front: bool| {
            let len = ch.overshoot_len * scale * rng.range(0.5, 1.4);
            let bend = rng.normal() * 0.25;
            let t = tan[at];
            let p0 = if front { s.pts[0] } else { *s.pts.last().unwrap() };
            let pr = if front { s.pressure[0] } else { *s.pressure.last().unwrap() };
            let k = steps(len).max(2);
            let mut ext = Vec::new();
            for j in 1..=k {
                let u = j as f32 / k as f32;
                let ang = bend * u;
                let (tx, ty) = (t.0 * ang.cos() - t.1 * ang.sin(), t.0 * ang.sin() + t.1 * ang.cos());
                ext.push(((p0.0 + dir * tx * len * u), (p0.1 + dir * ty * len * u), pr * (1.0 - 0.6 * u)));
            }
            if front {
                for e in ext {
                    s.pts.insert(0, (e.0, e.1));
                    s.pressure.insert(0, e.2);
                }
            } else {
                for e in ext {
                    s.pts.push((e.0, e.1));
                    s.pressure.push(e.2);
                }
            }
        };
        if let (Some(ia), Some(ib)) = (idx(a), idx(b))
            && s.pts.len() >= 2
        {
            if is_corner(ib) && (l.closed || (b as usize) < n - 1) && rng.f() < ch.overshoot {
                over(rng, ib, 1.0, &mut s, false);
            }
            if is_corner(ia) && (l.closed || a > 0) && rng.f() < ch.overshoot * 0.5 {
                over(rng, ia, -1.0, &mut s, true);
            }
        }
        if s.pts.len() >= 2 {
            out.push(s);
        }
        // restatements over part of this stretch
        let mut r = ch.restate;
        while r > 0.0 {
            if rng.f() < r.min(1.0) {
                let len = (b - a) as f32 * rng.range(0.4, 0.9);
                let o0 = a + (((b - a) as f32 - len).max(0.0) * rng.f()) as isize;
                let off = rng.normal() * ch.restate_off * scale * 1.5;
                let mut s = Stroke { pts: Vec::new(), pressure: Vec::new() };
                for o in o0..=o0 + len as isize {
                    let Some(i) = idx(o) else { continue };
                    s.pts.push((l.pts[i].0 + nrm[i].0 * off, l.pts[i].1 + nrm[i].1 * off));
                    s.pressure.push(l.pressure[i] * 0.7);
                }
                if s.pts.len() >= 3 {
                    out.push(s);
                }
            }
            r -= 1.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rock() -> Vec<P> {
        vec![(300.0, 600.0), (320.0, 450.0), (420.0, 380.0), (560.0, 400.0), (650.0, 480.0), (680.0, 600.0), (500.0, 620.0)]
    }

    #[test]
    fn deterministic_and_seeded() {
        let a = Outline::draw(&rock(), &[], true, Character::broken(), 7, None);
        let b = Outline::draw(&rock(), &[], true, Character::broken(), 7, None);
        let c = Outline::draw(&rock(), &[], true, Character::broken(), 8, None);
        assert_eq!(a.lines[0].pts, b.lines[0].pts);
        assert_eq!(a.strokes.len(), b.strokes.len());
        assert_ne!(a.lines[0].pts, c.lines[0].pts);
    }

    #[test]
    fn line_stays_near_the_points_and_passes_the_corners() {
        let pts = rock();
        let mut cor = vec![false; pts.len()];
        cor[2] = true;
        cor[5] = true;
        for ch in [Character::firm(), Character::searching(), Character::broken(), Character::soft()] {
            let o = Outline::draw(&pts, &cor, true, ch, 3, None);
            let l = &o.lines[0];
            assert!(l.closed && l.pts.len() > 200);
            assert_eq!(l.corners.len(), 2);
            // every painter's point has the line within a few units
            for p in &pts {
                let d = l.pts.iter().map(|q| dist(*p, *q)).fold(f32::MAX, f32::min);
                assert!(d < 0.05 * o.scale, "{ch:?}: {d} from {p:?}");
            }
            // the line never strays far from the smooth curve's extent
            let (x0, y0, x1, y1) = bbox(l.pts.iter().copied());
            assert!(x0 > 270.0 && x1 < 710.0 && y0 > 340.0 && y1 < 650.0);
            assert!(!o.strokes.is_empty());
            for s in &o.strokes {
                assert_eq!(s.pts.len(), s.pressure.len());
                assert!(s.pressure.iter().all(|p| (0.0..=1.0).contains(p)));
            }
        }
    }

    #[test]
    fn mask_edge_is_the_line() {
        let f = Frame::new(500, 375, 0.5);
        let o = Outline::draw(&rock(), &[], true, Character::firm(), 1, None);
        let m = o.mask(f);
        assert!(m.sample(480.0, 500.0) > 0.99);
        assert!(m.sample(100.0, 100.0) < 0.01);
        // just inside and outside the drawn line, along its normals
        let l = &o.lines[0];
        let nn = normals(&l.pts, true, 3);
        let mut ok = 0;
        for i in (0..l.pts.len()).step_by(37) {
            let (p, n) = (l.pts[i], nn[i]);
            if m.sample(p.0 - n.0 * 4.0, p.1 - n.1 * 4.0) > 0.8 && m.sample(p.0 + n.0 * 4.0, p.1 + n.1 * 4.0) < 0.2 {
                ok += 1;
            }
        }
        assert!(ok * 10 >= 9 * (l.pts.len() / 37), "{ok}");
    }

    #[test]
    fn body_from_a_skeleton_and_offsets() {
        // a sheep: spine rump -> head, four legs
        let spine = Bone { pts: vec![(100.0, 100.0), (110.0, 99.0), (120.0, 100.0), (126.0, 97.0), (130.0, 99.0)], widths: vec![9.0, 11.0, 10.0, 5.0, 4.0] };
        let legs: Vec<Bone> = [102.0, 106.0, 116.0, 119.0].iter().map(|&x| Bone { pts: vec![(x, 102.0), (x, 110.0)], widths: vec![1.6, 1.1] }).collect();
        let mut limbs = vec![spine];
        limbs.extend(legs);
        let o = Outline::body(&limbs, 0.6, Character::soft(), 5, None);
        assert_eq!(o.lines.len(), 1, "one silhouette");
        let (x0, y0, x1, y1) = bbox(o.lines[0].pts.iter().copied());
        assert!(x0 > 90.0 && x0 < 99.0 && x1 > 130.0 && x1 < 136.0, "{x0} {x1}");
        assert!(y1 > 109.0 && y1 < 113.0 && y0 > 90.0 && y0 < 97.0, "{y0} {y1}");
        let a = |o: &Outline| area2(&o.lines[0].pts).abs() * 0.5;
        let grown = o.offset(1.0, 0.0);
        let shrunk = o.offset(-1.0, 0.0);
        assert!(a(&grown) > a(&o) && a(&shrunk) < a(&o), "{} {} {}", a(&shrunk), a(&o), a(&grown));
    }

    /// Review 4 #8: lobes of different heights meet without a step, and a
    /// closed line's lobes meet across its seam.
    #[test]
    fn lobes_are_continuous_at_joins_and_the_seam() {
        let ks = [0.0, 10.0, 20.0, 30.0, 40.0];
        let hs = [2.0, 4.0, 1.0, 3.0];
        for closed in [false, true] {
            let bs = lobe_troughs(&hs, closed);
            for &k in &ks[1..4] {
                let (a, b) = (lobe_offset(&ks, &hs, &bs, k - 1e-4), lobe_offset(&ks, &hs, &bs, k + 1e-4));
                assert!((a - b).abs() < 0.02, "closed {closed}: step {} at knot {k}", (a - b).abs());
            }
            // the rounded bulge still rises well above the dips
            assert!(lobe_offset(&ks, &hs, &bs, 15.0) - lobe_offset(&ks, &hs, &bs, 10.0) > 3.0);
        }
        let bs = lobe_troughs(&hs, true);
        let (first, last) = (lobe_offset(&ks, &hs, &bs, 0.0), lobe_offset(&ks, &hs, &bs, 40.0 - 1e-4));
        assert!((first - last).abs() < 0.02, "seam step {}", (first - last).abs());
        // equal heights: the old shape
        let (hq, bq) = ([2.0; 4], lobe_troughs(&[2.0; 4], false));
        for u in [0.1f32, 0.37, 0.5, 0.8] {
            let want = 2.0 * ((PI * u).sin().powf(0.6) - 0.55);
            assert!((lobe_offset(&ks, &hq, &bq, 10.0 + 10.0 * u) - want).abs() < 1e-5);
        }
        // a whole soft closed line: no step between neighbors bigger than
        // the rounded lobes' own slope allows
        let pts: Vec<P> = (0..24).map(|i| {
            let a = i as f32 / 24.0 * TAU;
            (500.0 + 200.0 * a.cos(), 400.0 + 150.0 * a.sin())
        }).collect();
        let o = Outline::draw(&pts, &[], true, Character { wobble: 0.0, ..Character::soft() }, 4, None);
        let l = &o.lines[0].pts;
        let n = l.len();
        let steps: Vec<f32> = (0..n).map(|i| dist(l[i], l[(i + 1) % n])).collect();
        let mut sorted = steps.clone();
        sorted.sort_by(f32::total_cmp);
        let med = sorted[n / 2];
        assert!(steps.iter().all(|&d| d < 12.0 * med), "max step {} vs median {med}", sorted[n - 1]);
    }

    /// Review 4 #6: `amount` scales lobes and facets however they were set,
    /// before it or after it; 0 is a clean curve.
    #[test]
    fn amount_scales_lobes_set_after_it() {
        let line = [(100.0, 200.0), (300.0, 200.0), (500.0, 200.0)];
        let dev = |ch: Character| {
            let o = Outline::draw(&line, &[], false, ch, 1, None);
            o.lines[0].pts.iter().map(|p| (p.1 - 200.0).abs()).fold(0.0, f32::max)
        };
        // the binding's order: amount, then an explicit lobe (default height)
        let lobed = |k: f32| {
            let mut ch = Character::firm().amount(k);
            ch.lobe = 24.0 / 100.0;
            if ch.lobe_height == 0.0 {
                ch.lobe_height = 0.35;
            }
            ch
        };
        let mut soft0 = Character::soft().amount(0.0);
        soft0.lobe = 0.2;
        assert!(dev(soft0) < 1e-3, "soft, amount 0, explicit lobe: {}", dev(soft0));
        assert!(dev(lobed(0.0)) < 1e-3, "firm, amount 0, explicit lobe: {}", dev(lobed(0.0)));
        let (half, full) = (dev(lobed(0.5)), dev(lobed(1.0)));
        assert!(half > 0.3 && full > 1.5 * half, "amount scales an explicit lobe: {half} vs {full}");
        // the order doesn't matter
        let mut after = Character::firm();
        after.lobe = 0.24;
        after.lobe_height = 0.35;
        let before = after.amount(0.5);
        assert_eq!(dev(before), dev(lobed(0.5)));
    }

    /// Review 4 #5: an inset that consumes a closed shape is empty, not
    /// open: its mask is empty (so `o:mask() - o:inset(d):mask()` keeps the
    /// shape). A drawn open line is open.
    #[test]
    fn consumed_inset_is_an_empty_region() {
        let sq = [(100.0, 100.0), (110.0, 100.0), (110.0, 110.0), (100.0, 110.0)];
        let o = Outline::draw(&sq, &[true; 4], true, Character::firm().amount(0.0), 1, None);
        let f = Frame::new(400, 300, 0.4);
        let inset = o.offset(-7.0, 0.4);
        assert!(inset.lines.is_empty() && !inset.is_open(), "{} lines", inset.lines.len());
        assert!(inset.mask(f).data.iter().all(|&v| v == 0.0));
        let rim = o.mask(f).subtract(&inset.mask(f));
        assert!(rim.data == o.mask(f).data);
        assert!(!o.is_open() && !o.offset(-2.0, 0.4).is_open() && !o.offset(-2.0, 0.4).lines.is_empty());
        let open = Outline::draw(&[(0.0, 400.0), (1000.0, 390.0)], &[], false, Character::soft(), 2, None);
        assert!(open.is_open());
    }

    #[test]
    fn open_lines_mask_below() {
        let f = Frame::new(400, 300, 0.4);
        let o = Outline::draw(&[(0.0, 400.0), (300.0, 380.0), (700.0, 410.0), (1000.0, 390.0)], &[], false, Character::soft(), 2, Some(60.0));
        assert!(!o.lines[0].closed);
        let m = o.below(f, 750.0);
        assert!(m.sample(500.0, 600.0) > 0.99 && m.sample(500.0, 200.0) < 0.01);
        let b = o.band(f, 10.0, 0.0);
        let (p, _, _) = o.at(0.5).unwrap();
        assert!(b.sample(p.0, p.1) > 0.9);
    }
}
