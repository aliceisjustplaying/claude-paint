//! Drawing on the ground: graphite pencils and black chalk.
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
//! A pastel stick (`crate::pastel`) lays its colored particles into the
//! same drawing: a cell's reflectance is RGB (gray for graphite and chalk,
//! the same arithmetic per channel) and it keeps the pastel in the tooth.
//!
//! The bookkeeping (`Drawing`) exists only on a canvas that has been drawn
//! on, so paintings without a drawing are unchanged; a drawing with no
//! stick pastel in it keeps its old layout (`to_f32s`) bit for bit.

use crate::canvas::Canvas;
use crate::color::Rgb;
use crate::mask::Mask;
use crate::path::arclen;
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
    let (num, sign) = match g.strip_suffix('H') {
        Some(n) => (n, -1.0),
        None => (g.strip_suffix('B')?, 1.0),
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
pub(crate) struct Cell {
    /// Fraction of the pixel covered.
    pub(crate) a: f32,
    /// Mean reflectance of the flakes (linear RGB; gray for graphite and
    /// chalk).
    pub(crate) r: Rgb,
    /// How readily an eraser lifts it (coverage-weighted).
    pub(crate) lift: f32,
    /// Fixed coverage: the eraser can't lift below this.
    pub(crate) floor: f32,
    /// The canvas's film (coats) when this was drawn: once paint has gone
    /// over it the film is thicker, and the drawing is sealed.
    pub(crate) film: f32,
    /// A pastel stick's pastel in the tooth, µm (volume per area as it
    /// lies, packed): loose, and bound by fixative (`crate::pastel`).
    pub(crate) loose: f32,
    pub(crate) bound: f32,
}

/// The loose drawing on a canvas: what was laid where, so it can be lifted,
/// and the guide: where the lines were drawn, over the whole canvas.
#[derive(Clone)]
pub struct Drawing {
    /// The deposit, per pixel of the window (the canvas's optical buffers).
    pub(crate) cells: Vec<Cell>,
    /// The drawn lines as geometry, per pixel of the whole canvas (not just
    /// the window of a crop render): the coverage each line would lay on a
    /// perfectly smooth ground (the lead's rate and cap at the pressure),
    /// with no tooth, no grain and no wet paint to skip, so it is continuous
    /// along the line and the same in a crop render and a whole one.
    guide: Vec<f32>,
    /// What fixative bound of the guide: the eraser can't lift below this.
    guide_floor: Option<Vec<f32>>,
    /// Whether a pastel stick's pastel has landed in the tooth (or the
    /// finger has moved some): the cells are then serialized with their
    /// color and the pastel in the tooth (`to_f32s`).
    pub(crate) sticks: bool,
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

/// A line through `pts`: smoothed (unless `ruler` or `!smooth`), displaced
/// sideways by a small tremor (`tremor` units; none with `ruler`), with the
/// pressure profile along it.
pub fn hand_line(pts: &[(f32, f32)], profile: &[f32], smooth: bool, ruler: bool, tremor: f32, seed: u64) -> Mark {
    let step = 0.25;
    let mut p = resample(pts, smooth && !ruler, step);
    if !ruler && tremor > 0.0 {
        let s = arclen(&p);
        let q = p.clone();
        for i in 0..p.len() {
            let n = normal(&q, i);
            // a slow sway (30-unit scale) and a finer tremor (4-unit scale)
            let w = tremor * (1.4 * (vnoise(s[i] / 30.0, 0.5, seed) - 0.5) + 0.6 * (vnoise(s[i] / 4.0, 3.5, seed + 1) - 0.5));
            p[i] = (q[i].0 + n.0 * w, q[i].1 + n.1 * w);
        }
    }
    let s = arclen(&p);
    let pressure = pressure_along(profile, &s);
    Mark { pts: p, pressure }
}

/// Repeated passes over one path: `passes` light strokes along it, each
/// displaced sideways by amounts on the order of `wander` units, starting
/// and ending a few percent of the length short of or past the ends, and
/// broken into more pieces the longer the path (about one per 120-400
/// units); the first pass at 0.75 of `pressure`.
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
        // each pass has its own sideways offset and bow
        let side = wander * rng.range(-0.6, 0.6);
        let press = pressure * if k == 0 { 0.75 } else { rng.range(0.85, 1.15) };
        // start and end: short of the ends or past them
        let a0 = rng.range(-0.04, 0.06) * total;
        let a1 = total + rng.range(-0.06, 0.05) * total;
        // lift-offs: a long line is broken into a few strokes
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
/// `angle`, `spacing` units apart, each at most `length` units, bowed by up
/// to 6% of its length, heavy where it starts and lifting off at the end.
pub fn hatch_marks(m: &Mask, angle: f32, spacing: f32, length: f32, pressure: f32, seed: u64) -> Vec<Mark> {
    hatch_marks_graded(m, angle, spacing, length, pressure, false, seed)
}

/// `hatch_marks`, and with `graded` the mask is a weight, not a shape:
/// strokes run wherever it is above a trace (0.04), each pressed at every
/// point in proportion to the mask's value there, so a hatched passage
/// fades into the next instead of stopping at a line.
pub fn hatch_marks_graded(m: &Mask, angle: f32, spacing: f32, length: f32, pressure: f32, graded: bool, seed: u64) -> Vec<Mark> {
    let level = if graded { 0.04 } else { 0.5 };
    let f = m.f;
    let (w, h) = (f.width(), f.height());
    let (ca, sa) = (angle.cos(), angle.sin());
    // bounding box of the mask (units)
    let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for (i, &v) in m.data.iter().enumerate() {
        if v > level {
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
        x >= 0.0 && y >= 0.0 && x < w && y < h && m.sample(x, y) > level
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
                let mut pr = pressure_along(&[p * 0.8, p, p * 0.8, p * 0.25], &s);
                if graded {
                    // pressed as hard as the weight there asks
                    for (q, pt) in pr.iter_mut().zip(&pts) {
                        let (x, y) = (pt.0.clamp(0.0, w - 1e-3), pt.1.clamp(0.0, h - 1e-3));
                        *q *= m.sample(x, y).clamp(0.0, 1.0);
                    }
                    if pr.iter().cloned().fold(0.0, f32::max) < 0.03 {
                        continue;
                    }
                }
                out.push(Mark { pressure: pr, pts });
            }
        }
        v += spacing;
    }
    out
}

impl Drawing {
    fn new(n: usize, whole: usize) -> Self {
        Drawing { cells: vec![Cell { film: -1.0, ..Cell::default() }; n], guide: vec![0.0; whole], guide_floor: None, sticks: false }
    }

    /// The serialized layout (the checkpoint's drawing flag): 1 graphite
    /// and chalk only, 2 with a pastel stick's pastel (`sticks`).
    pub(crate) fn layout(&self) -> u64 {
        if self.sticks { 2 } else { 1 }
    }

    /// Serialized as f32s: the window's cells (a, r, lift, floor, film; with
    /// stick pastel, layout 2: a, r, g, b, lift, floor, film, loose, bound),
    /// the guide (whole canvas), then 0 or 1 and the guide's floor.
    pub(crate) fn to_f32s(&self) -> impl Iterator<Item = f32> + '_ {
        let sticks = self.sticks;
        let cells = self.cells.iter().flat_map(move |c| {
            let q = if sticks { [c.a, c.r[0], c.r[1], c.r[2], c.lift, c.floor, c.film, c.loose, c.bound] } else { [c.a, c.r[0], c.lift, c.floor, c.film, 0.0, 0.0, 0.0, 0.0] };
            q.into_iter().take(if sticks { 9 } else { 5 })
        });
        let floor = std::iter::once(if self.guide_floor.is_some() { 1.0 } else { 0.0 }).chain(self.guide_floor.iter().flatten().copied());
        cells.chain(self.guide.iter().copied()).chain(floor)
    }

    /// The inverse of `to_f32s`, reading with `get(count)`; `n` window
    /// pixels, `whole` canvas pixels, `layout` the serialized layout (1 or
    /// 2, `layout`). None if the data is inconsistent.
    pub(crate) fn from_f32s<E>(n: usize, whole: usize, layout: u64, mut get: impl FnMut(usize) -> Result<Vec<f32>, E>) -> Result<Option<Self>, E> {
        let sticks = layout >= 2;
        let cells: Vec<Cell> = if sticks {
            get(n * 9)?.as_chunks::<9>().0.iter().map(|q| Cell { a: q[0], r: [q[1], q[2], q[3]], lift: q[4], floor: q[5], film: q[6], loose: q[7], bound: q[8] }).collect()
        } else {
            get(n * 5)?.as_chunks::<5>().0.iter().map(|q| Cell { a: q[0], r: [q[1]; 3], lift: q[2], floor: q[3], film: q[4], ..Cell::default() }).collect()
        };
        let guide = get(whole)?;
        let has_floor = get(1)?[0];
        let guide_floor = if has_floor == 1.0 { Some(get(whole)?) } else { None };
        let ok = |v: f32| v.is_finite() && (0.0..=1.0).contains(&v);
        let valid = (has_floor == 0.0 || has_floor == 1.0)
            && cells.iter().all(|c| ok(c.a) && c.r.iter().all(|&v| ok(v)) && ok(c.lift) && ok(c.floor) && c.film.is_finite() && c.loose.is_finite() && c.loose >= 0.0 && c.bound.is_finite() && c.bound >= 0.0)
            && guide.iter().chain(guide_floor.iter().flatten()).all(|&v| ok(v));
        Ok(valid.then_some(Drawing { cells, guide, guide_floor, sticks }))
    }
}

/// Lay a mark's line into the guide (a whole-canvas buffer of frame `wf`):
/// the path with its width `wid` (units; at least two pixels, so it samples
/// as an unbroken line), laying what `lead` lays on a smooth ground.
fn guide_line(g: &mut [f32], wf: crate::canvas::Frame, lead: &Lead, pts: &[(f32, f32)], wid: &[f32], pressure: &[f32]) {
    let pxu = 1.0 / wf.scale;
    // (pixel, coverage, pressure); one pass per pixel, the nearest
    let mut hits: Vec<(usize, f32, f32)> = Vec::new();
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
                let cov = (0.5 - (d - 0.5 * w) / pxu).clamp(0.0, 1.0);
                if cov > 0.0 {
                    hits.push((y * wf.w + x, cov, pressure[i] + (pressure[i + 1] - pressure[i]) * t));
                }
            }
        }
    }
    hits.sort_unstable_by(|p, q| p.0.cmp(&q.0).then(q.1.total_cmp(&p.1)));
    hits.dedup_by_key(|h| h.0);
    for (k, cov, p) in hits {
        let dep = (lead.rate * cov * touch(p)).clamp(0.0, 1.0);
        let cap = lead.cap * (0.55 + 0.45 * p);
        g[k] += (cap - g[k]).max(0.0) * dep;
    }
}

impl Canvas {
    /// Whether anything has been drawn on this canvas.
    pub fn has_drawing(&self) -> bool {
        self.drawing.is_some()
    }

    pub(crate) fn drawing_mut(&mut self) -> &mut Drawing {
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
        guide_line(&mut self.drawing_mut().guide, f.whole(), lead, pts, &wid, &mark.pressure);
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
                c.r = c.r.map(|r| (c.a * r + da * lead.flake) / a1);
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
    /// hollows, and never quite all of it. Loose stick pastel leaves the
    /// tooth with what it lifts, so an erased passage takes pastel again. It
    /// doesn't reach under a sheet laid on the picture. It lifts the guide
    /// too (see `drawing_guide`), wherever it goes.
    pub fn erase(&mut self, m: &Mask, strength: f32) {
        self.check_mask(m);
        let f = self.f;
        let Some(d) = self.drawing.as_mut() else { return };
        let sheet = self.sheet.as_ref();
        let st = strength.clamp(0.0, 1.0);
        for (k, g) in d.guide.iter_mut().enumerate() {
            let mv = m.data[k].min(1.0);
            if mv > 0.0 && *g > 0.0 && !sheet.is_some_and(|s| s.covers_whole_pixel(f, k)) {
                let floor = d.guide_floor.as_ref().map_or(0.0, |fl| fl[k]);
                *g = (*g * (1.0 - (st * mv * GUIDE_LIFT).min(0.97))).max(floor.min(*g));
            }
        }
        let rt = (0.35 / (self.mm_per_unit / f.scale)).round().max(1.0) as isize;
        let (w, h) = (f.w as isize, f.h as isize);
        let height = &self.height;
        for i in 0..self.px.len() {
            let mv = m.data[f.whole_index(i)];
            let c = &mut d.cells[i];
            if mv <= 0.0 || c.a <= 0.0 || self.film[i] > c.film + 1e-4 || self.wet.vol[i] > 1e-5 || self.sheet.as_ref().is_some_and(|s| s.over[i]) {
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
            if c.loose > 0.0 && take > 0.0 {
                let removable = c.a - c.floor;
                c.loose *= if removable > 0.0 { (a1 - c.floor).max(0.0) / removable } else { 0.0 };
            }
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
        // (a sheet laid on the picture keeps the spray off what it covers)
        let sheet = self.sheet.as_ref();
        for (i, c) in d.cells.iter_mut().enumerate() {
            if m.is_none_or(|m| m.data[f.whole_index(i)] > 0.5) && !sheet.is_some_and(|s| s.over[i]) {
                c.floor = c.a;
            }
        }
        let fl = d.guide_floor.get_or_insert_with(|| vec![0.0; d.guide.len()]);
        for (k, (v, g)) in fl.iter_mut().zip(&d.guide).enumerate() {
            if m.is_none_or(|m| m.data[k] > 0.5) && !sheet.is_some_and(|s| s.covers_whole_pixel(f, k)) {
                *v = *g;
            }
        }
    }

    /// The graphite deposit as a mask: 1 on a firm line, fading with the
    /// deposit (0.5 of a pixel covered or more counts as 1). Includes drawing
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
    /// canvas (a crop render has all of it too), at least two pixels wide.
    /// It reads like `drawing_mask` would on a perfectly smooth ground: 1 on
    /// a firm line (0.5 of a pixel covered or more), about 0.5 on a light
    /// 2H line, fading to nothing as the pressure goes to zero. The eraser
    /// lifts it (never below what fixative bound); paint over the drawing
    /// doesn't change it. Unlike `drawing_mask` it follows each line
    /// continuously instead of breaking up in the tooth and the grain.
    /// This is the mask to paint into the drawing with, e.g. its firm lines:
    /// `work(&c.drawing_guide().band(0.7, 1.0, 0.1), ...)`.
    pub fn drawing_guide(&self) -> Mask {
        let whole = self.f.whole();
        match &self.drawing {
            Some(d) => Mask { f: whole, data: d.guide.iter().map(|a| (a / 0.5).min(1.0)).collect() },
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
pub(crate) fn uncover(p: Rgb, a: f32, r: Rgb) -> Rgb {
    if a <= 0.0 {
        return p;
    }
    let k = 1.0 / (1.0 - a).max(1e-3);
    [((p[0] - a * r[0]) * k).max(0.0), ((p[1] - a * r[1]) * k).max(0.0), ((p[2] - a * r[2]) * k).max(0.0)]
}

/// A deposit of coverage `a` and flake reflectance `r` over `under`.
pub(crate) fn cover(under: Rgb, a: f32, r: Rgb) -> Rgb {
    [under[0] * (1.0 - a) + a * r[0], under[1] * (1.0 - a) + a * r[1], under[2] * (1.0 - a) + a * r[2]]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat() -> Canvas {
        Canvas::new_window(400, 1.5, [0.8; 3], None).with_size_mm(440.0)
    }

    /// Mean color over a band of the window (units).
    fn band_mean(c: &Canvas, y0: f32, y1: f32, x0: f32, x1: f32) -> Rgb {
        let f = c.window();
        let (mut s, mut n) = ([0.0f32; 3], 0.0);
        for (i, q) in c.pixels().iter().enumerate() {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            if x >= x0 && x < x1 && y >= y0 && y < y1 {
                for k in 0..3 {
                    s[k] += q[k];
                }
                n += 1.0;
            }
        }
        s.map(|v| v / n)
    }

    /// Darkening laid by one 2B line at pressure `p` on a flat ground.
    fn laid(p: &[f32]) -> (Canvas, f32) {
        let mut c = flat();
        let m = hand_line(&[(100.0, 100.0), (900.0, 100.0)], p, false, true, 0.0, 5);
        c.draw(&Lead::pencil("2B").unwrap(), &m, 0.0, 9);
        let dark: f32 = c.pixels().iter().map(|q| 0.8 - q[0]).sum();
        (c, dark)
    }

    /// Graded hatching presses each stroke by the weight under it: over a
    /// ramp it lays more where the weight is high, nothing where it is zero,
    /// and the passage fades with no line where an ungraded hatch stops.
    #[test]
    fn graded_hatching_follows_the_weight() {
        let f = flat().frame();
        let ramp = Mask::from_fn(f, |x, _| ((x - 200.0) / 600.0).clamp(0.0, 1.0));
        let mut c = flat();
        let lead = Lead::pencil("4B").unwrap();
        for m in hatch_marks_graded(&ramp, 1.2, 4.0, 30.0, 0.8, true, 3) {
            c.draw(&lead, &m, 0.0, 7);
        }
        let dark = |x0: f32, x1: f32| 0.8 - band_mean(&c, 60.0, 600.0, x0, x1)[0];
        let (d0, d1, d2, d3) = (dark(100.0, 190.0), dark(300.0, 400.0), dark(500.0, 600.0), dark(700.0, 800.0));
        assert!(d0 < 1e-4, "no weight, nothing laid: {d0}");
        assert!(d1 < d2 && d2 < d3, "graded: {d1} {d2} {d3}");
        // the old call is unchanged: hatch_marks is the ungraded one
        let (a, b) = (hatch_marks(&ramp, 1.2, 4.0, 30.0, 0.8, 3), hatch_marks_graded(&ramp, 1.2, 4.0, 30.0, 0.8, false, 3));
        assert!(a.len() == b.len() && a.iter().zip(&b).all(|(a, b)| a.pts == b.pts && a.pressure == b.pressure));
    }

    /// A drawing in graphite alone keeps its old serialized layout (five
    /// numbers a cell, flag 1), so older states and digests are unchanged.
    #[test]
    fn graphite_drawing_keeps_its_layout() {
        let (c, _) = laid(&[0.6]);
        let d = c.drawing.as_ref().unwrap();
        assert!(!d.sticks && d.layout() == 1);
        let n = c.pixels().len();
        assert_eq!(d.to_f32s().count(), n * 5 + c.frame().w * c.frame().h + 1);
    }
}
