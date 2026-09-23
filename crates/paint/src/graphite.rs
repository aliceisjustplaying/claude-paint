//! Drawing on the ground: graphite pencils and black chalk, the underdrawing.
//!
//! Friedrich drew his compositions on the primed canvas with graphite pencils
//! of different hardness and black chalk, sometimes in two passes (a faint
//! outline, then a bolder one), the straights against a ruler. In the early
//! works the drawing stays visible through the very thin paint; lines "still
//! partly shimmer through" [CATS pp.128–132; NG p.49; see
//! notes/research/friedrich_materials.md §3].
//!
//! The model, from first principles and kept simple:
//! - **A point dragged over the tooth.** The point rides on the local tops of
//!   the ground (the weave showing through it); pressure lets it bite down
//!   into the hollows. A light line catches only the tops and breaks up into
//!   the grain of the canvas; a heavy one fills in.
//! - **Hardness** is the ratio of clay to graphite in the lead. A hard pencil
//!   (2H) lays less, of a lighter gray, and stays sharp; a soft one (4B) lays
//!   more, darker and glossier, and blunts fast (lines widen as it wears).
//!   Black chalk (carbon black in clay) is a deep matte black, crumbly and
//!   broad.
//! - **Optics.** The deposit covers a fraction `a` of each pixel with flakes of
//!   reflectance `r`; the ground shows between them. It is written into the
//!   dry picture, so every later layer of paint composites over it by
//!   Kubelka–Munk (`Pigment::over`): a thin veil lets it show, body color
//!   hides it. Graphite doesn't take on wet paint.
//! - **Loose until painted over.** A kneaded eraser lifts it, better from the
//!   tops than the hollows and never quite all of it; fixative binds it so the
//!   eraser no longer lifts it; once paint has gone over it, it is sealed in
//!   the picture.
//!
//! The bookkeeping (`Drawing`) exists only on a canvas that has been drawn
//! on, so paintings without a drawing are unchanged.

use crate::canvas::Canvas;
use crate::color::Rgb;
use crate::mask::Mask;
use crate::rng::{Rng, hash2};
use crate::surface::vnoise;

/// What a point is made of.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Medium {
    Graphite,
    Chalk,
}

/// A drawing point: a graphite pencil of some grade, or black chalk.
#[derive(Clone, Debug)]
pub struct Lead {
    pub medium: Medium,
    /// Grade as softness: 2H = -2, H = -1, F = -0.5, HB = 0, B = 1, 4B = 4.
    pub soft: f32,
    /// Reflectance of the laid flakes (linear, gray).
    pub flake: f32,
    /// Coverage laid in one pass at full contact and pressure (fraction of
    /// what is still bare).
    pub rate: f32,
    /// Most of a pixel's area the deposit can cover.
    pub cap: f32,
    /// Width of the sharpened point, mm.
    pub point_mm: f32,
    /// Length of line (mm) over which the point blunts (width grows toward 3×).
    pub blunt_mm: f32,
    /// How deep into the tooth the point reaches at full pressure, µm.
    pub bite_um: f32,
    /// Irregularity of the deposit (0 smooth, 1 crumbly).
    pub crumble: f32,
}

/// Softness of a pencil grade: "2H" → -2, "HB" → 0, "F" → -0.5, "4B" → 4.
pub fn softness(grade: &str) -> Option<f32> {
    let g = grade.trim().to_ascii_uppercase();
    match g.as_str() {
        "HB" => return Some(0.0),
        "F" => return Some(-0.5),
        _ => {}
    }
    // "2H", "B": ASCII digits then H or B (anything else, accented letters
    // and emoji included, is not a grade)
    let (num, sign) = if let Some(n) = g.strip_suffix('H') {
        (n, -1.0)
    } else if let Some(n) = g.strip_suffix('B') {
        (n, 1.0)
    } else {
        return None;
    };
    if !num.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let n: f32 = if num.is_empty() { 1.0 } else { num.parse().ok()? };
    if !(1.0..=9.0).contains(&n) {
        return None;
    }
    Some(sign * n)
}

impl Lead {
    /// A graphite pencil of softness `soft` (see `softness`), -9 (9H) to 9 (9B).
    pub fn graphite(soft: f32) -> Lead {
        let s = soft.clamp(-9.0, 9.0);
        // share of graphite (vs clay) in the lead, 0..1
        let t = (s + 9.0) / 18.0;
        Lead {
            medium: Medium::Graphite,
            soft: s,
            // clay-rich hard leads lay a pale silvery gray; soft ones a dark
            // gray (graphite is never black: its sheen keeps it gray)
            flake: 0.30 - 0.25 * t.powf(0.8),
            rate: (0.8 * 1.1f32.powf(s)).min(1.0),
            cap: (0.64 + 0.035 * s).clamp(0.35, 0.92),
            point_mm: 0.32 + 0.04 * s.max(0.0),
            blunt_mm: 4000.0 * 0.72f32.powf(s),
            bite_um: 90.0 * 1.1f32.powf(s),
            crumble: 0.2 + 0.03 * s.max(0.0),
        }
    }

    /// A graphite pencil by grade name ("2H", "HB", "2B", ...).
    pub fn pencil(grade: &str) -> Option<Lead> {
        softness(grade).map(Lead::graphite)
    }

    /// Natural black chalk: carbon black in clay. Deep, matte, broad and
    /// crumbly; it wears fast.
    pub fn chalk() -> Lead {
        Lead { medium: Medium::Chalk, soft: 6.0, flake: 0.022, rate: 1.0, cap: 0.93, point_mm: 0.9, blunt_mm: 350.0, bite_um: 150.0, crumble: 0.5 }
    }

    /// Width of the line (mm) after `worn_mm` of drawing since sharpening.
    pub fn width_mm(&self, worn_mm: f32) -> f32 {
        self.point_mm * (1.0 + 2.0 * (1.0 - (-worn_mm.max(0.0) / self.blunt_mm).exp()))
    }

    /// How readily a kneaded eraser takes it (per pass, at the tops).
    fn lift(&self) -> f32 {
        match self.medium {
            Medium::Graphite => 0.9,
            Medium::Chalk => 0.72,
        }
    }
}

/// One pixel of the drawing's bookkeeping.
#[derive(Clone, Copy, Default)]
struct Cell {
    /// Fraction of the pixel covered.
    a: f32,
    /// Mean reflectance of the flakes.
    r: f32,
    /// How readily an eraser lifts it (coverage-weighted).
    lift: f32,
    /// Fixed coverage: the eraser can't lift below this.
    floor: f32,
    /// The canvas's film (coats) when this was drawn: once paint has gone
    /// over it the film is thicker, and the drawing is sealed.
    film: f32,
}

/// The loose drawing on a canvas: what was laid where, so it can be lifted,
/// and the guide: where the lines were drawn, over the whole canvas.
#[derive(Clone)]
pub struct Drawing {
    /// The deposit, per pixel of the window (the canvas's optical buffers).
    cells: Vec<Cell>,
    /// The drawn lines as geometry, per pixel of the whole canvas (not just
    /// the window of a crop render): 1 on a line drawn with ordinary
    /// pressure, fading as the pressure goes to zero. Independent of the
    /// tooth, the grain and wet paint, so it is the same in a crop render
    /// and a whole one.
    guide: Vec<f32>,
    /// What fixative bound of the guide: the eraser can't lift below this.
    guide_floor: Option<Vec<f32>>,
}

/// How much of the point touches at pressure `p`: none at 0, rising
/// smoothly to all of it at 0.12 and above (a line laid with no pressure
/// lays nothing; a lifting stroke fades out instead of stopping short).
pub fn touch(p: f32) -> f32 {
    let t = (p / 0.12).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// What the eraser takes of the guide per pass at full strength.
const GUIDE_LIFT: f32 = 0.85;

/// A line to draw: a dense path (units) with the pressure at every point.
#[derive(Clone, Debug, Default)]
pub struct Mark {
    pub pts: Vec<(f32, f32)>,
    pub pressure: Vec<f32>,
}

impl Mark {
    /// Length in units.
    pub fn length(&self) -> f32 {
        self.pts.windows(2).map(|w| ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt()).sum()
    }
}

/// Catmull–Rom through `pts` (or straight between them when `!smooth`),
/// resampled every `step` units.
pub fn resample(pts: &[(f32, f32)], smooth: bool, step: f32) -> Vec<(f32, f32)> {
    if pts.len() < 2 {
        return pts.to_vec();
    }
    let step = step.max(0.01);
    let n = pts.len();
    let at = |i: isize| pts[i.clamp(0, n as isize - 1) as usize];
    let mut out = vec![pts[0]];
    for i in 0..n - 1 {
        let (p0, p1, p2, p3) = (at(i as isize - 1), at(i as isize), at(i as isize + 1), at(i as isize + 2));
        let len = ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt();
        let k = ((len / step).ceil() as usize).max(1);
        for j in 1..=k {
            let t = j as f32 / k as f32;
            let p = if smooth {
                let (t2, t3) = (t * t, t * t * t);
                let f = |a: f32, b: f32, c: f32, d: f32| 0.5 * (2.0 * b + (c - a) * t + (2.0 * a - 5.0 * b + 4.0 * c - d) * t2 + (3.0 * b - a - 3.0 * c + d) * t3);
                (f(p0.0, p1.0, p2.0, p3.0), f(p0.1, p1.1, p2.1, p3.1))
            } else {
                (p1.0 + (p2.0 - p1.0) * t, p1.1 + (p2.1 - p1.1) * t)
            };
            out.push(p);
        }
    }
    out
}

/// Cumulative arc length along a path.
fn arclen(p: &[(f32, f32)]) -> Vec<f32> {
    let mut s = vec![0.0; p.len()];
    for i in 1..p.len() {
        s[i] = s[i - 1] + ((p[i].0 - p[i - 1].0).powi(2) + (p[i].1 - p[i - 1].1).powi(2)).sqrt();
    }
    s
}

/// Unit normal of a path at point `i`.
fn normal(p: &[(f32, f32)], i: usize) -> (f32, f32) {
    let (a, b) = (p[i.saturating_sub(1)], p[(i + 1).min(p.len() - 1)]);
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l = (dx * dx + dy * dy).sqrt().max(1e-6);
    (-dy / l, dx / l)
}

/// Pressure along a path of `n` points from a profile given at evenly spaced
/// stations (one value: even; two: start to end; more: through them).
pub fn pressure_along(profile: &[f32], s: &[f32]) -> Vec<f32> {
    let total = s.last().copied().unwrap_or(0.0).max(1e-6);
    s.iter()
        .map(|&si| match profile.len() {
            0 => 0.5,
            1 => profile[0],
            m => {
                let u = (si / total).clamp(0.0, 1.0) * (m - 1) as f32;
                let k = (u.floor() as usize).min(m - 2);
                profile[k] + (profile[k + 1] - profile[k]) * (u - k as f32)
            }
        })
        .map(|p| p.clamp(0.0, 1.0))
        .collect()
}

/// A hand-drawn line through `pts`: smoothed (unless `ruler` or `!smooth`),
/// with the hand's slight tremor (`tremor` units; none against a ruler), and
/// the pressure profile along it.
pub fn hand_line(pts: &[(f32, f32)], profile: &[f32], smooth: bool, ruler: bool, tremor: f32, seed: u64) -> Mark {
    let step = 0.25;
    let mut p = resample(pts, smooth && !ruler, step);
    if !ruler && tremor > 0.0 {
        let s = arclen(&p);
        let q = p.clone();
        for i in 0..p.len() {
            let n = normal(&q, i);
            // a slow sway of the arm and a finer tremor of the fingers
            let w = tremor * (1.4 * (vnoise(s[i] / 30.0, 0.5, seed) - 0.5) + 0.6 * (vnoise(s[i] / 4.0, 3.5, seed + 1) - 0.5));
            p[i] = (q[i].0 + n.0 * w, q[i].1 + n.1 * w);
        }
    }
    let s = arclen(&p);
    let pressure = pressure_along(profile, &s);
    Mark { pts: p, pressure }
}

/// A searching line: `passes` light strokes along the same path, each
/// wandering off it a little (`wander` units), starting and ending a little
/// early or late, and now and then lifting off; the first pass lightest.
pub fn sketch_marks(pts: &[(f32, f32)], pressure: f32, passes: usize, wander: f32, smooth: bool, tremor: f32, seed: u64) -> Vec<Mark> {
    let base = resample(pts, smooth, 0.25);
    let s = arclen(&base);
    let total = s.last().copied().unwrap_or(0.0);
    if total <= 0.0 {
        return Vec::new();
    }
    let mut rng = Rng::new(seed);
    let mut out = Vec::new();
    for k in 0..passes.max(1) {
        let ps = seed.wrapping_add(k as u64 * 7919);
        let amp = wander * rng.range(0.6, 1.2);
        // each pass is its own guess: a little to one side, bowed its own way
        let side = wander * rng.range(-0.6, 0.6);
        let press = pressure * if k == 0 { 0.75 } else { rng.range(0.85, 1.15) };
        // start and end: short of the ends or past them
        let a0 = rng.range(-0.04, 0.06) * total;
        let a1 = total + rng.range(-0.06, 0.05) * total;
        // lift-offs: the hand breaks a long line into a few strokes
        let pieces = 1 + (total / 120.0 * rng.range(0.3, 1.2)) as usize;
        let mut cuts: Vec<f32> = (1..pieces).map(|j| a0 + (a1 - a0) * (j as f32 + rng.range(-0.25, 0.25)) / pieces as f32).collect();
        cuts.insert(0, a0);
        cuts.push(a1);
        for w in cuts.windows(2) {
            let (lo, hi) = (w[0] + rng.range(0.0, 0.015) * total, w[1]);
            if hi - lo < 0.5 {
                continue;
            }
            // points along the base path (extended straight past its ends)
            let n = ((hi - lo) / 0.25).ceil() as usize + 1;
            let mut p = Vec::with_capacity(n);
            for j in 0..n {
                let a = lo + (hi - lo) * j as f32 / (n - 1) as f32;
                let (pt, nm) = point_at(&base, &s, a);
                let d = side + amp * (2.0 * vnoise(a / 45.0, k as f32 * 3.1, ps) - 1.0) + tremor * (vnoise(a / 4.0, 1.7, ps + 1) - 0.5);
                p.push((pt.0 + nm.0 * d, pt.1 + nm.1 * d));
            }
            let sp = arclen(&p);
            // each stroke swells in and lifts off
            let pr = pressure_along(&[press * 0.5, press, press * rng.range(0.8, 1.1), press * 0.4], &sp);
            out.push(Mark { pts: p, pressure: pr });
        }
    }
    out
}

/// The point at arc length `a` of a path (extended straight past its ends)
/// and the normal there.
fn point_at(p: &[(f32, f32)], s: &[f32], a: f32) -> ((f32, f32), (f32, f32)) {
    let n = p.len();
    let k = match s.iter().position(|&v| v >= a) {
        Some(0) => 1,
        Some(k) => k,
        None => n - 1,
    };
    let (p0, p1) = (p[k - 1], p[k]);
    let seg = (s[k] - s[k - 1]).max(1e-6);
    let t = (a - s[k - 1]) / seg;
    let pt = (p0.0 + (p1.0 - p0.0) * t, p0.1 + (p1.1 - p0.1) * t);
    (pt, normal(p, k))
}

/// Hatching: short parallel strokes across `m` (where it is > 0.5) at
/// `angle`, `spacing` units apart, each at most `length` units, bowed a
/// little by the wrist, heavy where it starts and lifting off at the end.
pub fn hatch_marks(m: &Mask, angle: f32, spacing: f32, length: f32, pressure: f32, seed: u64) -> Vec<Mark> {
    let f = m.f;
    let (w, h) = (f.width(), f.height());
    let (ca, sa) = (angle.cos(), angle.sin());
    // bounding box of the mask (units)
    let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for (i, &v) in m.data.iter().enumerate() {
        if v > 0.5 {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x);
            y1 = y1.max(y);
        }
    }
    if x0 > x1 {
        return Vec::new();
    }
    // along (u) and across (v) the strokes
    let corners = [(x0, y0), (x1, y0), (x0, y1), (x1, y1)];
    let (mut v0, mut v1, mut u0, mut u1) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
    for &(x, y) in &corners {
        let (u, v) = (x * ca + y * sa, -x * sa + y * ca);
        u0 = u0.min(u);
        u1 = u1.max(u);
        v0 = v0.min(v);
        v1 = v1.max(v);
    }
    let spacing = spacing.max(0.2);
    let length = length.max(spacing);
    let mut rng = Rng::new(seed);
    let step = (0.5 / f.scale).max(0.2);
    let inside = |u: f32, v: f32| {
        let (x, y) = (u * ca - v * sa, u * sa + v * ca);
        x >= 0.0 && y >= 0.0 && x < w && y < h && m.sample(x, y) > 0.5
    };
    let mut out = Vec::new();
    let mut v = v0 + rng.range(0.0, spacing);
    while v <= v1 {
        let vv = v + rng.range(-0.18, 0.18) * spacing;
        // runs inside the mask along this line
        let mut u = u0;
        let mut run: Option<f32> = None;
        let mut runs = Vec::new();
        while u <= u1 + step {
            let ins = u <= u1 && inside(u, vv);
            match (run, ins) {
                (None, true) => run = Some(u),
                (Some(a), false) => {
                    runs.push((a, u - step));
                    run = None;
                }
                _ => {}
            }
            u += step;
        }
        for (a, b) in runs {
            // split long runs into strokes that overlap or leave small gaps
            let n = ((b - a) / length).ceil().max(1.0) as usize;
            for j in 0..n {
                let (sa_, sb) = (a + (b - a) * j as f32 / n as f32, a + (b - a) * (j + 1) as f32 / n as f32);
                let (sa_, sb) = (sa_ + rng.range(-0.08, 0.1) * length, sb + rng.range(-0.1, 0.06) * length);
                let (sa_, sb) = (sa_.max(a - 0.3 * spacing), sb.min(b + 0.3 * spacing));
                if sb - sa_ < 0.4 * spacing {
                    continue;
                }
                let bow = rng.range(-0.06, 0.06) * (sb - sa_);
                let k = ((sb - sa_) / 0.25).ceil() as usize + 1;
                let pts: Vec<(f32, f32)> = (0..k)
                    .map(|i| {
                        let t = i as f32 / (k - 1) as f32;
                        let uu = sa_ + (sb - sa_) * t;
                        let off = vv + bow * 4.0 * t * (1.0 - t);
                        (uu * ca - off * sa, uu * sa + off * ca)
                    })
                    .collect();
                let p = pressure * rng.range(0.85, 1.12);
                let s = arclen(&pts);
                out.push(Mark { pressure: pressure_along(&[p * 0.8, p, p * 0.8, p * 0.25], &s), pts });
            }
        }
        v += spacing;
    }
    out
}

impl Drawing {
    fn new(n: usize, whole: usize) -> Self {
        Drawing { cells: vec![Cell { film: -1.0, ..Cell::default() }; n], guide: vec![0.0; whole], guide_floor: None }
    }

    /// Serialized as f32s: the window's cells (a, r, lift, floor, film), the
    /// guide (whole canvas), then 0 or 1 and the guide's floor.
    pub(crate) fn to_f32s(&self) -> impl Iterator<Item = f32> + '_ {
        let cells = self.cells.iter().flat_map(|c| [c.a, c.r, c.lift, c.floor, c.film]);
        let floor = std::iter::once(if self.guide_floor.is_some() { 1.0 } else { 0.0 }).chain(self.guide_floor.iter().flatten().copied());
        cells.chain(self.guide.iter().copied()).chain(floor)
    }

    /// The inverse of `to_f32s`, reading with `get(count)`; `n` window
    /// pixels, `whole` canvas pixels. None if the data is inconsistent.
    pub(crate) fn from_f32s<E>(n: usize, whole: usize, mut get: impl FnMut(usize) -> Result<Vec<f32>, E>) -> Result<Option<Self>, E> {
        let raw = get(n * 5)?;
        let cells: Vec<Cell> = raw.as_chunks::<5>().0.iter().map(|q| Cell { a: q[0], r: q[1], lift: q[2], floor: q[3], film: q[4] }).collect();
        let guide = get(whole)?;
        let has_floor = get(1)?[0];
        let guide_floor = if has_floor == 1.0 { Some(get(whole)?) } else { None };
        let ok = |v: f32| v.is_finite() && (0.0..=1.0).contains(&v);
        let valid = (has_floor == 0.0 || has_floor == 1.0)
            && cells.iter().all(|c| ok(c.a) && ok(c.r) && ok(c.lift) && ok(c.floor) && c.film.is_finite())
            && guide.iter().chain(guide_floor.iter().flatten()).all(|&v| ok(v));
        Ok(valid.then_some(Drawing { cells, guide, guide_floor }))
    }
}

/// Lay a mark's line into the guide (a whole-canvas buffer of frame `wf`):
/// the path with its width `wid` (units; at least two pixels, so it samples
/// as an unbroken line), as firm as the point touches.
fn guide_line(g: &mut [f32], wf: crate::canvas::Frame, pts: &[(f32, f32)], wid: &[f32], pressure: &[f32]) {
    let pxu = 1.0 / wf.scale;
    for i in 0..pts.len() - 1 {
        let (a, b) = (pts[i], pts[i + 1]);
        let (ta, tb) = (touch(pressure[i]), touch(pressure[i + 1]));
        if ta <= 0.0 && tb <= 0.0 {
            continue;
        }
        let hw = 0.5 * wid[i].max(wid[i + 1]).max(2.0 * pxu) + pxu;
        let to_px = |u: f32, n: usize| ((u * wf.scale).floor().max(0.0) as usize).min(n);
        let (px0, px1) = (to_px(a.0.min(b.0) - hw, wf.w), (to_px(a.0.max(b.0) + hw, wf.w) + 1).min(wf.w));
        let (py0, py1) = (to_px(a.1.min(b.1) - hw, wf.h), (to_px(a.1.max(b.1) + hw, wf.h) + 1).min(wf.h));
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let l2 = (dx * dx + dy * dy).max(1e-12);
        for y in py0..py1 {
            let yu = wf.uy(y);
            for x in px0..px1 {
                let xu = wf.ux(x);
                let t = (((xu - a.0) * dx + (yu - a.1) * dy) / l2).clamp(0.0, 1.0);
                let d = ((xu - a.0 - t * dx).powi(2) + (yu - a.1 - t * dy).powi(2)).sqrt();
                let w = (wid[i] + (wid[i + 1] - wid[i]) * t).max(2.0 * pxu);
                let v = (0.5 - (d - 0.5 * w) / pxu).clamp(0.0, 1.0) * (ta + (tb - ta) * t);
                let k = y * wf.w + x;
                if v > g[k] {
                    g[k] = v;
                }
            }
        }
    }
}

impl Canvas {
    /// Whether anything has been drawn on this canvas.
    pub fn has_drawing(&self) -> bool {
        self.drawing.is_some()
    }

    fn drawing_mut(&mut self) -> &mut Drawing {
        let (n, whole) = (self.px.len(), self.f.full_w * self.f.full_h);
        self.drawing.get_or_insert_with(|| Box::new(Drawing::new(n, whole)))
    }

    /// Radius (px) over which the point rides on the tops of the ground.
    fn tooth_px(&self) -> isize {
        (0.35 / self.px_mm()).round().max(1.0) as isize
    }

    /// Draw one line with `lead` along the mark (units), the point having
    /// drawn `worn_mm` since it was sharpened. Returns the mm drawn (the
    /// point wears by that much). Wet paint doesn't take graphite: the line
    /// skips it. The deposit fades to nothing as the pressure goes to zero
    /// (`touch`). The line also goes into the drawing's guide (see
    /// `drawing_guide`), over the whole canvas even in a crop render.
    pub fn draw(&mut self, lead: &Lead, mark: &Mark, worn_mm: f32, seed: u64) -> f32 {
        let pts = &mark.pts;
        if pts.len() < 2 {
            return 0.0;
        }
        let f = self.f;
        let mmu = self.mm_per_unit;
        let pxu = 1.0 / f.scale;
        let s = arclen(pts);
        // width per point (units): the point wears along the line and
        // flattens a little under pressure
        let wid: Vec<f32> = (0..pts.len()).map(|i| lead.width_mm(worn_mm + s[i] * mmu) * (0.8 + 0.4 * mark.pressure[i]) / mmu).collect();
        // footprint: (pixel index, coverage, pressure)
        let mut hits: Vec<(usize, f32, f32)> = Vec::new();
        for i in 0..pts.len() - 1 {
            let (a, b) = (pts[i], pts[i + 1]);
            let hw = 0.5 * wid[i].max(wid[i + 1]).max(pxu) + pxu;
            let (bx0, by0) = (a.0.min(b.0) - hw, a.1.min(b.1) - hw);
            let (bx1, by1) = (a.0.max(b.0) + hw, a.1.max(b.1) + hw);
            let to_px = |u: f32, o: usize, n: usize| ((u * f.scale).floor() as isize - o as isize).clamp(0, n as isize) as usize;
            let (px0, px1) = (to_px(bx0, f.x0, f.w), (to_px(bx1, f.x0, f.w) + 1).min(f.w));
            let (py0, py1) = (to_px(by0, f.y0, f.h), (to_px(by1, f.y0, f.h) + 1).min(f.h));
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let l2 = (dx * dx + dy * dy).max(1e-12);
            for y in py0..py1 {
                let yu = f.uy(y);
                for x in px0..px1 {
                    let xu = f.ux(x);
                    let t = (((xu - a.0) * dx + (yu - a.1) * dy) / l2).clamp(0.0, 1.0);
                    let d = ((xu - a.0 - t * dx).powi(2) + (yu - a.1 - t * dy).powi(2)).sqrt();
                    let w = wid[i] + (wid[i + 1] - wid[i]) * t;
                    let cov = (0.5 - (d - (0.5 * w).max(0.5 * pxu)) / pxu).clamp(0.0, 1.0) * (w / pxu).min(1.0);
                    if cov > 0.0 {
                        let p = mark.pressure[i] + (mark.pressure[i + 1] - mark.pressure[i]) * t;
                        hits.push((y * f.w + x, cov, p));
                    }
                }
            }
        }
        guide_line(&mut self.drawing_mut().guide, f.whole(), pts, &wid, &mark.pressure);
        // one deposit per pixel: the nearest pass of the line over it
        hits.sort_unstable_by(|p, q| p.0.cmp(&q.0).then(q.1.total_cmp(&p.1)));
        hits.dedup_by_key(|h| h.0);
        let rt = self.tooth_px();
        let (w, h) = (f.w as isize, f.h as isize);
        let height = &self.height;
        let top = |i: usize| {
            let (x, y) = ((i % f.w) as isize, (i / f.w) as isize);
            let mut m = f32::MIN;
            for j in (y - rt).max(0)..=(y + rt).min(h - 1) {
                for k in (x - rt).max(0)..=(x + rt).min(w - 1) {
                    m = m.max(height[(j * w + k) as usize]);
                }
            }
            m
        };
        // how far each pixel lies below the tops the point rides on (µm)
        let depths: Vec<f32> = hits.iter().map(|&(i, _, _)| top(i) - height[i]).collect();
        let lift = lead.lift();
        // a pixel coarser than the weave's threads holds tops of its own
        // (its averaged height overstates how far the point is from them)
        // and averages many crumbs of the deposit
        let sub = (0.15 / self.px_mm()).min(1.0);
        let (depth_k, crumble) = (sub.powf(0.6), lead.crumble * sub.sqrt());
        self.drawing_mut();
        let film = std::mem::take(&mut self.film);
        let wetv: Vec<bool> = hits.iter().map(|&(i, _, _)| self.wet.vol[i] > 1e-5).collect();
        let mut px = std::mem::take(&mut self.px);
        {
            let d = self.drawing_mut();
            for (k, &(i, cov, p)) in hits.iter().enumerate() {
                if wetv[k] {
                    continue;
                }
                let c = &mut d.cells[i];
                if c.film < 0.0 || film[i] > c.film + 1e-4 {
                    // bare, or painted over since: a new drawing on top
                    *c = Cell { film: film[i], ..Cell::default() };
                }
                let depth = depths[k] * depth_k;
                let bite = lead.bite_um * p.powf(1.2) + 3.0;
                let contact = (-depth.max(0.0) / bite).exp();
                let (gx, gy) = ((i % f.w + f.x0) as i64, (i / f.w + f.y0) as i64);
                let grain = 1.0 - crumble * hash2(gx, gy, seed);
                let dep = (lead.rate * contact * cov * grain * touch(p)).clamp(0.0, 1.0);
                let cap = lead.cap * (0.55 + 0.45 * p);
                let da = (cap - c.a).max(0.0) * dep;
                if da <= 0.0 {
                    continue;
                }
                let a1 = c.a + da;
                let under = uncover(px[i], c.a, c.r);
                c.r = (c.a * c.r + da * lead.flake) / a1;
                c.lift = (c.a * c.lift + da * lift) / a1;
                c.a = a1;
                px[i] = cover(under, c.a, c.r);
            }
        }
        self.px = px;
        self.film = film;
        s.last().copied().unwrap_or(0.0) * mmu
    }

    /// A kneaded eraser pressed and rolled over `m` (coverage 0..1): lifts
    /// loose drawing (not fixed, not painted over) by up to `strength` 0..1
    /// of what is there, better from the tops of the tooth than from the
    /// hollows, and never quite all of it. It lifts the guide too (see
    /// `drawing_guide`), wherever it goes.
    pub fn erase(&mut self, m: &Mask, strength: f32) {
        self.check_mask(m);
        let Some(d) = self.drawing.as_mut() else { return };
        let st = strength.clamp(0.0, 1.0);
        for (k, g) in d.guide.iter_mut().enumerate() {
            let mv = m.data[k].min(1.0);
            if mv > 0.0 && *g > 0.0 {
                let floor = d.guide_floor.as_ref().map_or(0.0, |fl| fl[k]);
                *g = (*g * (1.0 - (st * mv * GUIDE_LIFT).min(0.97))).max(floor.min(*g));
            }
        }
        let f = self.f;
        let rt = (0.35 / (self.mm_per_unit / f.scale)).round().max(1.0) as isize;
        let (w, h) = (f.w as isize, f.h as isize);
        let height = &self.height;
        for i in 0..self.px.len() {
            let mv = m.data[f.whole_index(i)];
            let c = &mut d.cells[i];
            if mv <= 0.0 || c.a <= 0.0 || self.film[i] > c.film + 1e-4 || self.wet.vol[i] > 1e-5 {
                continue;
            }
            let (x, y) = ((i % f.w) as isize, (i / f.w) as isize);
            let mut top = f32::MIN;
            for j in (y - rt).max(0)..=(y + rt).min(h - 1) {
                for k in (x - rt).max(0)..=(x + rt).min(w - 1) {
                    top = top.max(height[(j * w + k) as usize]);
                }
            }
            // the eraser touches the tops; the hollows keep more
            let reach = 1.0 - ((top - height[i]) / 10.0).clamp(0.0, 1.0);
            let take = (strength.clamp(0.0, 1.0) * mv.min(1.0) * c.lift * (0.55 + 0.4 * reach)).min(0.97);
            let a1 = (c.a * (1.0 - take)).max(c.floor.min(c.a));
            let under = uncover(self.px[i], c.a, c.r);
            c.a = a1;
            self.px[i] = cover(under, c.a, c.r);
        }
    }

    /// Fixative over `m` (or all): binds the loose drawing so an eraser no
    /// longer lifts it.
    pub fn fix_drawing(&mut self, m: Option<&Mask>) {
        if let Some(m) = m {
            self.check_mask(m);
        }
        let f = self.f;
        let Some(d) = self.drawing.as_mut() else { return };
        for (i, c) in d.cells.iter_mut().enumerate() {
            if m.is_none_or(|m| m.data[f.whole_index(i)] > 0.5) {
                c.floor = c.a;
            }
        }
        let fl = d.guide_floor.get_or_insert_with(|| vec![0.0; d.guide.len()]);
        for (k, (v, g)) in fl.iter_mut().zip(&d.guide).enumerate() {
            if m.is_none_or(|m| m.data[k] > 0.5) {
                *v = *g;
            }
        }
    }

    /// The graphite deposit as a mask: 1 on a firm line, fading with the
    /// deposit (0.5 of a pixel covered or more reads as 1). Includes drawing
    /// that paint has since covered, until something is drawn over it.
    ///
    /// It is the physical deposit, broken by the tooth and the grain of the
    /// lead, and it is known only where the canvas holds pixels: in a crop
    /// render it is zero outside the window. To paint into the drawing (a
    /// mask to plan strokes with) use `drawing_guide`.
    pub fn drawing_mask(&self) -> Mask {
        let whole = self.f.whole();
        let mut m = Mask::empty(whole);
        if let Some(d) = &self.drawing {
            let f = self.f;
            for (i, c) in d.cells.iter().enumerate() {
                m.data[f.whole_index(i)] = (c.a / 0.5).min(1.0);
            }
        }
        m
    }

    /// The drawing as geometry: where the lines were drawn, over the whole
    /// canvas (a crop render has all of it too). 1 on a line drawn with
    /// ordinary pressure (at least two pixels wide), fading as the pressure goes
    /// to zero; lifted by the eraser (never below what fixative bound).
    /// Unlike `drawing_mask` it follows the line continuously, not the
    /// grain of the deposit, and paint over the drawing doesn't change it.
    /// This is the mask to paint into the drawing with:
    /// `work(&c.drawing_guide().dilate(1.0), ...)`.
    pub fn drawing_guide(&self) -> Mask {
        let whole = self.f.whole();
        match &self.drawing {
            Some(d) => Mask { f: whole, data: d.guide.clone() },
            None => Mask::empty(whole),
        }
    }

    /// The drawing alone, as it would look on a white ground (the pixels of
    /// the window): what an infrared reflectogram shows of an underdrawing.
    pub fn drawing_view(&self) -> Vec<Rgb> {
        const PAPER: f32 = 0.82;
        match &self.drawing {
            None => vec![[PAPER; 3]; self.px.len()],
            Some(d) => d.cells.iter().map(|c| cover([PAPER; 3], c.a, c.r)).collect(),
        }
    }
}

/// The ground under a deposit of coverage `a` and flake reflectance `r`.
fn uncover(p: Rgb, a: f32, r: f32) -> Rgb {
    if a <= 0.0 {
        return p;
    }
    let k = 1.0 / (1.0 - a).max(1e-3);
    [((p[0] - a * r) * k).max(0.0), ((p[1] - a * r) * k).max(0.0), ((p[2] - a * r) * k).max(0.0)]
}

/// A deposit of coverage `a` and flake reflectance `r` over `under`.
fn cover(under: Rgb, a: f32, r: f32) -> Rgb {
    [under[0] * (1.0 - a) + a * r, under[1] * (1.0 - a) + a * r, under[2] * (1.0 - a) + a * r]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Style;
    use crate::pigment::Pigment;
    use crate::color::{hex, luminance};

    fn canvas() -> Canvas {
        Style { width_mm: 440.0, ..Style::friedrich_early() }.prepare(400, 1.5, 3)
    }

    fn mean_lum(c: &Canvas, x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
        let f = c.window();
        let (mut s, mut n) = (0.0, 0.0);
        for (i, p) in c.pixels().iter().enumerate() {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            if x >= x0 && x < x1 && y >= y0 && y < y1 {
                s += luminance(*p);
                n += 1.0;
            }
        }
        s / n
    }

    fn band(c: &mut Canvas, lead: &Lead, y: f32, p: f32, lines: usize) {
        for k in 0..lines {
            let yy = y + k as f32 * 0.25;
            let m = hand_line(&[(100.0, yy), (300.0, yy)], &[p], false, true, 0.0, 5 + k as u64);
            c.draw(lead, &m, 0.0, 9 + k as u64);
        }
    }

    #[test]
    fn grades_parse() {
        assert_eq!(softness("2H"), Some(-2.0));
        assert_eq!(softness("hb"), Some(0.0));
        assert_eq!(softness("4B"), Some(4.0));
        assert_eq!(softness("B"), Some(1.0));
        assert_eq!(softness("F"), Some(-0.5));
        assert_eq!(softness("12B"), None);
        assert_eq!(softness("2X"), None);
        // not a grade, and never a panic (review 4 #9: "é" split mid-char)
        for g in ["é", "éB", "2é", "Bé", "🙂", "2🙂", "", " ", "+2B", "1.5B", "0B", "HH"] {
            assert_eq!(softness(g), None, "{g:?}");
            assert!(Lead::pencil(g).is_none());
        }
    }

    fn flat() -> Canvas {
        Canvas::new_window(400, 1.5, [0.8; 3], None).with_size_mm(440.0)
    }

    /// Darkening laid by one 2B line at pressure `p` on a flat ground.
    fn laid(p: &[f32]) -> (Canvas, f32) {
        let mut c = flat();
        let m = hand_line(&[(100.0, 100.0), (900.0, 100.0)], p, false, true, 0.0, 5);
        c.draw(&Lead::pencil("2B").unwrap(), &m, 0.0, 9);
        let dark: f32 = c.pixels().iter().map(|q| 0.8 - q[0]).sum();
        (c, dark)
    }

    /// Review 4 #7: no pressure, no line; the deposit fades continuously to
    /// nothing as the pressure goes to zero, and a lifting stroke fades out.
    #[test]
    fn zero_pressure_lays_nothing() {
        let (c, dark) = laid(&[0.0]);
        assert!(dark == 0.0 && c.drawing_mask().data.iter().all(|&v| v == 0.0), "p=0 laid {dark}");
        assert!(c.drawing_guide().data.iter().all(|&v| v == 0.0));
        let ds: Vec<f32> = [0.005, 0.02, 0.05, 0.1, 0.2, 0.4].iter().map(|&p| laid(&[p]).1).collect();
        assert!(ds.windows(2).all(|w| w[0] < w[1]), "monotone in pressure: {ds:?}");
        assert!(ds[0] < 0.01 * ds[5] && ds[1] < 0.1 * ds[5], "continuous toward zero: {ds:?}");
        // a stroke lifting off: nothing at its end
        let (c, _) = laid(&[0.8, 0.0]);
        let f = c.window();
        let col = |x: f32| (0..f.h).map(|y| 0.8 - c.pixels()[y * f.w + (x * f.scale) as usize][0]).sum::<f32>();
        assert!(col(899.6) < 1e-4 && col(880.0) < col(700.0) && col(700.0) < col(300.0), "lift-off {} {} {} {}", col(899.6), col(880.0), col(700.0), col(300.0));
    }

    /// The guide is the drawn line: continuous along it where the deposit
    /// breaks up into the grain (the beaded limbs of `pencil.lua`); lifted
    /// by the eraser, kept by fixative, unchanged by paint over it.
    #[test]
    fn guide_follows_the_line() {
        let mut c = Style { width_mm: 440.0, ..Style::friedrich_early() }.prepare(1000, 1.4, 11);
        let m = hand_line(&[(779.0, 420.0), (812.0, 398.0), (838.0, 366.0), (858.0, 330.0)], &[0.5, 0.2], true, false, 0.15 / 0.44, 5);
        c.draw(&Lead::pencil("2B").unwrap(), &m, 400.0, 9);
        let (g, dm) = (c.drawing_guide(), c.drawing_mask());
        let on: Vec<(f32, f32)> = m.pts.iter().map(|p| (g.sample(p.0, p.1), dm.sample(p.0, p.1))).collect();
        assert!(on.iter().all(|s| s.0 > 0.75), "guide breaks along the line: {:?}", on.iter().map(|s| s.0).fold(1.0, f32::min));
        assert!(on.iter().any(|s| s.1 < 0.55), "the deposit itself is beaded (the test shows the difference)");
        // off the line: nothing
        assert_eq!(g.sample(700.0, 300.0), 0.0);
        let glaze = Pigment::masstone_hiding(hex("#b8c4cc"), 0.98);
        c.glaze(&glaze, None, |_, _| 4.0);
        assert!(c.drawing_guide().data == g.data, "paint doesn't move the guide");
        let f = c.frame();
        let left = Mask::from_fn(f, |x, _| if x < 820.0 { 1.0 } else { 0.0 });
        c.fix_drawing(Some(&left));
        c.erase(&Mask::full(f), 1.0);
        let e = c.drawing_guide();
        let near = |x: f32| *m.pts.iter().min_by(|p, q| (p.0 - x).abs().total_cmp(&(q.0 - x).abs())).unwrap();
        let (kept, lifted) = (near(790.0), near(850.0));
        assert!(e.sample(kept.0, kept.1) == g.sample(kept.0, kept.1), "fixed part kept");
        let v = e.sample(lifted.0, lifted.1);
        assert!(v < 0.2 && v > 0.0, "loose part lifted, a ghost left: {v}");
    }

    fn crop_draw(c: &mut Canvas) {
        for y in (150..550).step_by(20) {
            let m = hand_line(&[(50.0, y as f32), (950.0, y as f32)], &[0.8], false, true, 0.0, 5);
            c.draw(&Lead::chalk(), &m, 200.0, 9);
        }
        let k = hand_line(&[(200.0, 600.0), (480.0, 320.0), (800.0, 640.0)], &[0.6, 0.1], true, false, 0.0, 6);
        c.draw(&Lead::pencil("2B").unwrap(), &k, 0.0, 10);
    }

    /// Review 4 #1: painting into the drawing in a crop render paints what a
    /// whole render does (the guide has the whole drawing; the deposit mask
    /// doesn't, so it changed every stroke's random stream).
    #[test]
    fn guide_is_the_same_in_a_crop() {
        use crate::bristle::Tool;
        use crate::canvas::Crop;
        use crate::handling::Handling;
        let mut a = Canvas::new_window(500, 1.4, [0.8; 3], None).with_size_mm(440.0);
        let mut b = Canvas::new_window(500, 1.4, [0.8; 3], Some(Crop { units: [400.0, 250.0, 600.0, 450.0], margin: 80.0 })).with_size_mm(440.0);
        crop_draw(&mut a);
        crop_draw(&mut b);
        let (ga, gb) = (a.drawing_guide(), b.drawing_guide());
        assert!(ga.data == gb.data, "the guide is the whole drawing in a crop too");
        let hd = Handling::new(Tool::round_sable(4.0)).length(10.0, 20.0).coverage(2.0).color(|_, _| [0.08, 0.12, 0.18]).hug(false).clip(false).fill(false);
        a.work(&ga.dilate(4.0), &hd, 77);
        b.work(&gb.dilate(4.0), &hd, 77);
        a.dry();
        b.dry();
        let f = b.window();
        let (mut n, mut max) = (0, 0.0f32);
        for (i, p) in b.pixels().iter().enumerate() {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            if !(400.0..600.0).contains(&x) || !(250.0..450.0).contains(&y) {
                continue;
            }
            let q = a.pixels()[f.whole_index(i)];
            n += 1;
            for k in 0..3 {
                max = max.max((p[k] - q[k]).abs());
            }
        }
        assert!(n > 5000 && max < 1e-3, "crop and whole differ by {max} over {n} pixels");
    }

    /// Softer grades and more pressure lay darker lines; chalk is darkest.
    #[test]
    fn softer_and_harder_pressed_is_darker() {
        let mut c = canvas();
        let bare = mean_lum(&c, 100.0, 50.0, 300.0, 60.0);
        band(&mut c, &Lead::pencil("2H").unwrap(), 50.0, 0.6, 40);
        band(&mut c, &Lead::pencil("4B").unwrap(), 80.0, 0.6, 40);
        band(&mut c, &Lead::pencil("4B").unwrap(), 110.0, 0.2, 40);
        band(&mut c, &Lead::chalk(), 140.0, 0.6, 40);
        let h2 = mean_lum(&c, 110.0, 50.0, 290.0, 60.0);
        let b4 = mean_lum(&c, 110.0, 80.0, 290.0, 90.0);
        let b4l = mean_lum(&c, 110.0, 110.0, 290.0, 120.0);
        let ch = mean_lum(&c, 110.0, 140.0, 290.0, 150.0);
        assert!(bare > h2 && h2 > b4 && b4 > ch, "bare {bare} 2H {h2} 4B {b4} chalk {ch}");
        assert!(b4l > b4, "light 4B {b4l} vs firm {b4}");
    }

    /// A light line catches the tops of the tooth first.
    #[test]
    fn light_line_catches_the_tops() {
        let mut c = Style { width_mm: 440.0, ..Style::friedrich_early() }.prepare(1600, 1.5, 3);
        let before = c.surface_um().to_vec();
        let lead = Lead::pencil("HB").unwrap();
        for k in 0..30 {
            let y = 100.0 + k as f32 * 0.1;
            c.draw(&lead, &hand_line(&[(100.0, y), (200.0, y)], &[0.15], false, true, 0.0, 1), 0.0, k);
        }
        let d = c.drawing.as_ref().unwrap();
        let f = c.window();
        // correlate deposit with height inside the band
        let mut pairs = Vec::new();
        for (i, cell) in d.cells.iter().enumerate() {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            if (110.0..190.0).contains(&x) && (100.3..102.6).contains(&y) {
                pairs.push((before[i], cell.a));
            }
        }
        let n = pairs.len() as f32;
        let (mh, ma) = (pairs.iter().map(|p| p.0).sum::<f32>() / n, pairs.iter().map(|p| p.1).sum::<f32>() / n);
        let cov: f32 = pairs.iter().map(|p| (p.0 - mh) * (p.1 - ma)).sum::<f32>() / n;
        assert!(cov > 0.0, "deposit should favor the tops (cov {cov}, mean a {ma})");
        assert!(ma > 0.02, "a light line still leaves something: {ma}");
    }

    /// Erasing lifts most but not all; fixed drawing can't be lifted; a
    /// drawing painted over is sealed.
    #[test]
    fn erase_fix_and_seal() {
        let mut c = canvas();
        let bare = mean_lum(&c, 110.0, 50.0, 290.0, 60.0);
        let lead = Lead::pencil("2B").unwrap();
        band(&mut c, &lead, 50.0, 0.7, 40);
        band(&mut c, &lead, 80.0, 0.7, 40);
        let drawn = mean_lum(&c, 110.0, 50.0, 290.0, 60.0);
        let f = c.frame();
        let top = Mask::from_fn(f, |_, y| if y < 70.0 { 1.0 } else { 0.0 });
        c.erase(&top, 1.0);
        let erased = mean_lum(&c, 110.0, 50.0, 290.0, 60.0);
        assert!(erased > drawn + 0.5 * (bare - drawn), "most is lifted: bare {bare} drawn {drawn} erased {erased}");
        assert!(erased < bare - 0.002, "a ghost remains: bare {bare} erased {erased}");
        // the lower band is fixed, then erased: unchanged
        c.fix_drawing(None);
        let fixed = mean_lum(&c, 110.0, 80.0, 290.0, 90.0);
        let low = Mask::from_fn(f, |_, y| if y > 70.0 { 1.0 } else { 0.0 });
        c.erase(&low, 1.0);
        assert_eq!(fixed, mean_lum(&c, 110.0, 80.0, 290.0, 90.0));
        // a glaze seals the drawing; erasing does nothing to the picture
        let mut c2 = canvas();
        band(&mut c2, &lead, 50.0, 0.7, 40);
        c2.glaze(&Pigment::transparent(hex("#c8b890")), None, |_, _| 0.3);
        let snap = c2.pixels().to_vec();
        c2.erase(&Mask::full(f), 1.0);
        assert!(snap == c2.pixels(), "sealed drawing lifted");
    }

    /// Thin paint lets the drawing show; body color hides it.
    #[test]
    fn thin_paint_shows_body_hides() {
        let mut c = canvas();
        let lead = Lead::pencil("2B").unwrap();
        band(&mut c, &lead, 50.0, 0.8, 40);
        let f = c.frame();
        let thin = Pigment::masstone_hiding(hex("#b8c4cc"), 0.2);
        let body = Pigment::masstone_hiding(hex("#b8c4cc"), 0.98);
        let left = Mask::from_fn(f, |x, _| if x < 200.0 { 1.0 } else { 0.0 });
        let right = Mask::from_fn(f, |x, _| if x >= 200.0 { 1.0 } else { 0.0 });
        let line0 = mean_lum(&c, 110.0, 50.0, 190.0, 60.0);
        let off0 = mean_lum(&c, 110.0, 70.0, 190.0, 80.0);
        println!("bare contrast {line0} vs {off0}");
        c.glaze(&thin, Some(&left), |_, _| 1.0);
        c.glaze(&body, Some(&right), |_, _| 4.0);
        let line_l = mean_lum(&c, 110.0, 50.0, 190.0, 60.0);
        let off_l = mean_lum(&c, 110.0, 70.0, 190.0, 80.0);
        let line_r = mean_lum(&c, 210.0, 50.0, 290.0, 60.0);
        let off_r = mean_lum(&c, 210.0, 70.0, 290.0, 80.0);
        assert!(off_l - line_l > 0.03, "thin paint: the line shows ({line_l} vs {off_l})");
        assert!((off_r - line_r).abs() < 0.3 * (off_l - line_l), "body color hides it ({line_r} vs {off_r})");
    }

    /// No drawing: the canvas is untouched and carries no bookkeeping.
    #[test]
    fn no_drawing_no_change() {
        let mut c = canvas();
        let snap = c.pixels().to_vec();
        c.erase(&Mask::full(c.frame()), 1.0);
        c.fix_drawing(None);
        assert!(!c.has_drawing() && snap == c.pixels());
    }

    /// The point blunts with use; soft leads faster.
    #[test]
    fn points_wear() {
        let (h, b) = (Lead::pencil("2H").unwrap(), Lead::pencil("4B").unwrap());
        assert!(h.width_mm(500.0) < h.point_mm * 1.2);
        assert!(b.width_mm(500.0) > b.point_mm * 1.5);
        assert!(b.width_mm(1e6) <= b.point_mm * 3.0 + 1e-4);
    }
}

#[cfg(test)]
mod probe {
    use crate::style::Style;
    #[test]
    #[ignore]
    fn tooth_depths() {
        for (wmm, px) in [(440.0, 1000usize), (440.0, 3200), (1714.0, 1000), (1714.0, 3200)] {
            for st in [Style { width_mm: wmm, ..Style::friedrich() }, Style { width_mm: wmm, ..Style::friedrich_early() }] {
                let c = st.prepare(px, 1.5, 3);
                let f = c.window();
                let h = c.surface_um();
                let rt = c.tooth_px();
                let mut d = Vec::new();
                for y in (200..f.h - 200).step_by(7) {
                    for x in (200..f.w - 200).step_by(5) {
                        let mut m = f32::MIN;
                        for j in y - rt as usize..=y + rt as usize {
                            for k in x - rt as usize..=x + rt as usize {
                                m = m.max(h[j * f.w + k]);
                            }
                        }
                        d.push(m - h[y * f.w + x]);
                    }
                }
                d.sort_by(|a, b| a.total_cmp(b));
                let q = |p: f32| d[((d.len() - 1) as f32 * p) as usize];
                println!("{wmm}mm {px}px rt {rt} px_mm {:.3}: depth q10 {:.1} q50 {:.1} q90 {:.1}", c.px_mm(), q(0.1), q(0.5), q(0.9));
            }
        }
    }
}
