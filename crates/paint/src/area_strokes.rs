//! Strokes over an area: hatching that follows a direction field, zigzags,
//! jitter and open gaps to the paper, for a pastel stick's `p:hatch` with
//! the easel's `--@ pastel-area-strokes` switch.
//!
//! This only places marks; each mark is then drawn as one stroke of the
//! stick, as `graphite::hatch_marks_graded`'s marks are, so the stroke model
//! (contact, wear, crumbs) is the stick's own.
//!
//! - **A fixed angle** places marks exactly as `hatch_marks_graded` does.
//! - **A direction field** (an angle in radians at every point) places
//!   evenly spaced streamlines through the mask (after Jobard and Lefer,
//!   "Creating evenly-spaced streamlines of arbitrary density", 1997), each
//!   cut into strokes as the fixed-angle hatching cuts its rows.
//! - **Open**, **jitter** and **zigzag** then act on each mark in turn,
//!   each with its own seeded draws per mark, so turning one on doesn't
//!   move what the others do.

use crate::graphite::{self, Mark};
use crate::mask::Mask;
use crate::path::arclen;
use crate::rng::Rng;
use std::collections::VecDeque;

/// Which way the marks run.
pub enum Direction<'a> {
    /// One angle (radians) everywhere.
    Fixed(f32),
    /// An angle (radians) at each point (units); read as a direction, so
    /// θ and θ + π are the same.
    Field(&'a (dyn Fn(f32, f32) -> f32 + Sync)),
}

/// Each mark turned into one back-and-forth stroke along it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Zigzag {
    /// How far the stroke swings to either side of the mark's line (units).
    pub amplitude: f32,
    /// The length along the line of one swing there and back (units).
    pub period: f32,
    /// 0: sharp corners (a triangle wave); 1: rounded right through (a sine).
    pub round: f32,
}

/// Each mark's length and angle varied.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Jitter {
    /// The length scaled by up to 1 ± this (a share, 0..1).
    pub length: f32,
    /// The mark turned by up to ± this about its middle (radians).
    pub angle: f32,
}

/// How much of the area is left open to the paper.
pub enum Open<'a> {
    /// The same share everywhere (0..1).
    Share(f32),
    /// The share at each mark's middle: the mask's value there.
    Mask(&'a Mask),
}

/// Hatching over an area.
pub struct AreaHatch<'a> {
    pub direction: Direction<'a>,
    pub spacing: f32,
    pub length: f32,
    pub pressure: f32,
    pub graded: bool,
    pub zigzag: Option<Zigzag>,
    pub jitter: Option<Jitter>,
    pub open: Option<Open<'a>>,
}

/// Per-mark draws: one generator per (option, mark), so options draw
/// independently of each other and of the placement.
fn mark_rng(seed: u64, salt: u64, k: usize) -> Rng {
    Rng::new(seed ^ salt.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (k as u64).wrapping_mul(0xD1B5_4A32_D192_ED03))
}
const OPEN_SALT: u64 = 0x0FE7;
const JITTER_SALT: u64 = 0x1177;
const ZIGZAG_SALT: u64 = 0x2162;

/// The marks for hatching `m` as `h` says.
pub fn area_marks(m: &Mask, h: &AreaHatch, seed: u64) -> Vec<Mark> {
    let level = if h.graded { 0.04 } else { 0.5 };
    let base = match h.direction {
        Direction::Fixed(a) => graphite::hatch_marks_graded(m, a, h.spacing, h.length, h.pressure, h.graded, seed),
        Direction::Field(f) => streamline_marks(m, f, h.spacing, h.length, h.pressure, h.graded, seed),
    };
    let mut out = Vec::with_capacity(base.len());
    for (k, mark) in base.into_iter().enumerate() {
        if mark.pts.len() < 2 {
            continue;
        }
        if let Some(open) = &h.open {
            let (x, y) = mark.pts[mark.pts.len() / 2];
            let share = match open {
                Open::Share(s) => *s,
                Open::Mask(o) => sample_in(o, x, y),
            };
            if mark_rng(seed, OPEN_SALT, k).f() < share.clamp(0.0, 1.0) {
                continue;
            }
        }
        let mark = match h.jitter {
            Some(j) => jittered(m, level, mark, j, &mut mark_rng(seed, JITTER_SALT, k)),
            None => mark,
        };
        let mark = match h.zigzag {
            Some(z) => zigzagged(m, level, &mark, z, &mut mark_rng(seed, ZIGZAG_SALT, k)),
            None => mark,
        };
        out.push(mark);
    }
    out
}

/// A mask's value at a point, the point held inside its frame.
fn sample_in(m: &Mask, x: f32, y: f32) -> f32 {
    let f = m.f;
    m.sample(x.clamp(0.0, f.width() - 1e-3), y.clamp(0.0, f.height() - 1e-3))
}

fn inside(m: &Mask, level: f32, x: f32, y: f32) -> bool {
    let f = m.f;
    x >= 0.0 && y >= 0.0 && x < f.width() && y < f.height() && m.sample(x, y) > level
}

/// A mark turned about its middle and scaled in length (growing only as far
/// as the area: a mark that would leave it keeps its length).
fn jittered(m: &Mask, level: f32, mark: Mark, j: Jitter, rng: &mut Rng) -> Mark {
    let turn = j.angle * rng.range(-1.0, 1.0);
    let mut k = 1.0 + j.length.clamp(0.0, 1.0) * rng.range(-1.0, 1.0);
    let c = mark.pts[mark.pts.len() / 2];
    let (ct, st) = (turn.cos(), turn.sin());
    let moved = |k: f32, p: (f32, f32)| {
        let (dx, dy) = ((p.0 - c.0) * k, (p.1 - c.1) * k);
        (c.0 + dx * ct - dy * st, c.1 + dx * st + dy * ct)
    };
    if k > 1.0 {
        let (a, b) = (moved(k, mark.pts[0]), moved(k, *mark.pts.last().unwrap()));
        if !inside(m, level, a.0, a.1) || !inside(m, level, b.0, b.1) {
            k = 1.0;
        }
    }
    Mark { pts: mark.pts.iter().map(|&p| moved(k, p)).collect(), pressure: mark.pressure }
}

/// The wave a zigzag swings by, -1..1 at `t` swings along (1 at t = 0).
fn wave(t: f32, round: f32) -> f32 {
    let u = t - t.floor();
    let tri = 4.0 * (u - 0.5).abs() - 1.0;
    let sine = (std::f32::consts::TAU * u).cos();
    tri + (sine - tri) * round.clamp(0.0, 1.0)
}

/// A mark made one continuous stroke swinging across its line: `amplitude`
/// to either side, one swing there and back every `period` along it, from
/// a seeded phase. Where a swing would leave the area it is cut short at
/// its edge, so the band stays inside the mask.
fn zigzagged(m: &Mask, level: f32, mark: &Mark, z: Zigzag, rng: &mut Rng) -> Mark {
    let (amp, period) = (z.amplitude.max(0.0), z.period.max(0.5));
    let s0 = arclen(&mark.pts);
    let total = s0.last().copied().unwrap_or(0.0);
    if total <= 0.0 || amp <= 0.0 {
        return mark.clone();
    }
    // the line resampled finely enough that the swung path's points stay
    // about 0.25 units apart
    let slope = 4.0 * amp / period;
    let step = 0.25 / (1.0 + slope);
    let line = graphite::resample(&mark.pts, false, step);
    let s = arclen(&line);
    let phase = rng.f();
    let n = line.len();
    let mut pts = Vec::with_capacity(n);
    let mut pressure = Vec::with_capacity(n);
    let mut j = 0;
    for i in 0..n {
        let (a, b) = (line[i.saturating_sub(1)], line[(i + 1).min(n - 1)]);
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let l = (dx * dx + dy * dy).sqrt().max(1e-6);
        let nm = (-dy / l, dx / l);
        let d = amp * wave(s[i] / period + phase, z.round);
        let mut q = (line[i].0 + nm.0 * d, line[i].1 + nm.1 * d);
        if !inside(m, level, q.0, q.1) {
            // back toward the line until inside (or on it)
            let mut f = 1.0;
            for _ in 0..6 {
                f *= 0.5;
                q = (line[i].0 + nm.0 * d * f, line[i].1 + nm.1 * d * f);
                if inside(m, level, q.0, q.1) {
                    break;
                }
            }
            if !inside(m, level, q.0, q.1) {
                q = line[i];
            }
        }
        pts.push(q);
        // the mark's pressure at this far along it
        while j + 1 < s0.len() && s0[j + 1] < s[i] {
            j += 1;
        }
        let p = if j + 1 < s0.len() {
            let t = ((s[i] - s0[j]) / (s0[j + 1] - s0[j]).max(1e-6)).clamp(0.0, 1.0);
            mark.pressure[j] + (mark.pressure[j + 1] - mark.pressure[j]) * t
        } else {
            *mark.pressure.last().unwrap()
        };
        pressure.push(p);
    }
    Mark { pts, pressure }
}

/// The point at arc length `a` along `p` (arc lengths `s`), held to its ends.
fn at_length(p: &[(f32, f32)], s: &[f32], a: f32) -> (f32, f32) {
    let k = s.partition_point(|&v| v < a).clamp(1, p.len() - 1);
    let t = ((a - s[k - 1]) / (s[k] - s[k - 1]).max(1e-6)).clamp(0.0, 1.0);
    (p[k - 1].0 + (p[k].0 - p[k - 1].0) * t, p[k - 1].1 + (p[k].1 - p[k - 1].1) * t)
}

/// Points already traced, in cells `cell` units wide, for the spacing test.
struct Traced {
    x0: f32,
    y0: f32,
    cell: f32,
    nx: usize,
    ny: usize,
    /// (x, y, line, index along it)
    cells: Vec<Vec<(f32, f32, u32, i32)>>,
}

impl Traced {
    fn new(b: (f32, f32, f32, f32), cell: f32) -> Self {
        let nx = ((b.2 - b.0) / cell).ceil() as usize + 1;
        let ny = ((b.3 - b.1) / cell).ceil() as usize + 1;
        Traced { x0: b.0, y0: b.1, cell, nx, ny, cells: vec![Vec::new(); nx * ny] }
    }
    fn cell_of(&self, x: f32, y: f32) -> (isize, isize) {
        (((x - self.x0) / self.cell).floor() as isize, ((y - self.y0) / self.cell).floor() as isize)
    }
    fn add(&mut self, x: f32, y: f32, line: u32, idx: i32) {
        let (i, j) = self.cell_of(x, y);
        if i >= 0 && j >= 0 && (i as usize) < self.nx && (j as usize) < self.ny {
            self.cells[j as usize * self.nx + i as usize].push((x, y, line, idx));
        }
    }
    /// Whether a traced point lies within `d` (≤ `cell`) of (x, y), not
    /// counting the points of `own` line within `skip` indices of `idx`.
    fn near(&self, x: f32, y: f32, d: f32, own: Option<(u32, i32, i32)>) -> bool {
        let (ci, cj) = self.cell_of(x, y);
        for j in cj - 1..=cj + 1 {
            for i in ci - 1..=ci + 1 {
                if i < 0 || j < 0 || i as usize >= self.nx || j as usize >= self.ny {
                    continue;
                }
                for &(px, py, line, k) in &self.cells[j as usize * self.nx + i as usize] {
                    if let Some((l, idx, skip)) = own
                        && l == line
                        && (k - idx).abs() <= skip
                    {
                        continue;
                    }
                    if (px - x).powi(2) + (py - y).powi(2) < d * d {
                        return true;
                    }
                }
            }
        }
        false
    }
}

/// Hatching along a direction field: evenly spaced streamlines through the
/// mask, `spacing` apart, each cut into strokes of at most `length` that
/// overlap or leave small gaps, bowed a little and lifting off at the end,
/// as `hatch_marks_graded` cuts its rows (and pressed by the mask with
/// `graded`).
pub fn streamline_marks(m: &Mask, field: &(dyn Fn(f32, f32) -> f32 + Sync), spacing: f32, length: f32, pressure: f32, graded: bool, seed: u64) -> Vec<Mark> {
    let level = if graded { 0.04 } else { 0.5 };
    let f = m.f;
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
    let spacing = spacing.max(0.2);
    let length = length.max(spacing);
    let step = (0.5 / f.scale).max(0.2).min(0.25 * spacing);
    let (dsep, dtest) = (spacing, 0.5 * spacing);
    // a line's own points this near it along the line are its neighbors, not a crossing
    let skip = (2.0 * dsep / step).ceil() as i32 + 2;
    let max_steps = ((2.0 * ((x1 - x0) + (y1 - y0)) + 4.0 * spacing) / step) as usize;
    let mut rng = Rng::new(seed);
    let dir = |x: f32, y: f32| {
        let a = field(x, y);
        let (c, s) = (a.cos(), a.sin());
        if c.is_finite() && s.is_finite() { (c, s) } else { (1.0, 0.0) }
    };
    // a direction turned to agree with `prev` (a direction has no sign)
    let along = |d: (f32, f32), prev: (f32, f32)| if d.0 * prev.0 + d.1 * prev.1 < 0.0 { (-d.0, -d.1) } else { d };
    let mut traced = Traced::new((x0 - spacing, y0 - spacing, x1 + spacing, y1 + spacing), dsep);
    let mut lines: Vec<Vec<(f32, f32)>> = Vec::new();
    let mut queue: VecDeque<(f32, f32)> = VecDeque::new();
    // fallback seeds over the whole box, for the parts no line reaches
    let (ox, oy) = (rng.range(0.0, spacing), rng.range(0.0, spacing));
    let (gx, gy) = (((x1 - x0) / spacing).ceil() as usize + 1, ((y1 - y0) / spacing).ceil() as usize + 1);
    let mut grid = (0..gx * gy).map(move |k| (x0 + ox + (k % gx) as f32 * spacing, y0 + oy + (k / gx) as f32 * spacing));
    loop {
        let Some(seed_pt) = queue.pop_front().or_else(|| grid.next()) else { break };
        let (sx, sy) = seed_pt;
        if !inside(m, level, sx, sy) || traced.near(sx, sy, dsep, None) {
            continue;
        }
        let id = lines.len() as u32;
        traced.add(sx, sy, id, 0);
        let d0 = dir(sx, sy);
        let mut halves: [Vec<(f32, f32)>; 2] = [Vec::new(), Vec::new()];
        for (h, sign) in [(0usize, 1.0f32), (1, -1.0)] {
            let (mut p, mut prev) = ((sx, sy), (d0.0 * sign, d0.1 * sign));
            for n in 1..=max_steps {
                // midpoint (second-order) step
                let d1 = along(dir(p.0, p.1), prev);
                let mid = (p.0 + d1.0 * step * 0.5, p.1 + d1.1 * step * 0.5);
                let d2 = along(dir(mid.0, mid.1), d1);
                let q = (p.0 + d2.0 * step, p.1 + d2.1 * step);
                let idx = if h == 0 { n as i32 } else { -(n as i32) };
                // a sharp turn is a singular point of the field: stop there
                if !inside(m, level, q.0, q.1) || d2.0 * prev.0 + d2.1 * prev.1 < 0.5 || traced.near(q.0, q.1, dtest, Some((id, idx, skip))) {
                    break;
                }
                traced.add(q.0, q.1, id, idx);
                halves[h].push(q);
                prev = d2;
                p = q;
            }
        }
        let [fwd, back] = halves;
        let line: Vec<(f32, f32)> = back.into_iter().rev().chain(std::iter::once((sx, sy))).chain(fwd).collect();
        // new seeds a spacing to either side, every spacing along it
        let every = ((dsep / step).round() as usize).max(1);
        for i in (0..line.len()).step_by(every) {
            let (a, b) = (line[i.saturating_sub(1)], line[(i + 1).min(line.len() - 1)]);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let l = (dx * dx + dy * dy).sqrt();
            if l <= 0.0 {
                continue;
            }
            let nm = (-dy / l, dx / l);
            queue.push_back((line[i].0 + nm.0 * dsep, line[i].1 + nm.1 * dsep));
            queue.push_back((line[i].0 - nm.0 * dsep, line[i].1 - nm.1 * dsep));
        }
        lines.push(line);
    }
    // each line cut into strokes, as hatch_marks_graded cuts its runs
    let mut out = Vec::new();
    for line in &lines {
        if line.len() < 2 {
            continue;
        }
        let s = arclen(line);
        let total = *s.last().unwrap();
        let n = (total / length).ceil().max(1.0) as usize;
        for j in 0..n {
            let (a, b) = (total * j as f32 / n as f32, total * (j + 1) as f32 / n as f32);
            let (a, b) = (a + rng.range(-0.08, 0.1) * length, b + rng.range(-0.1, 0.06) * length);
            let (a, b) = (a.max(0.0), b.min(total));
            if b - a < 0.4 * spacing {
                continue;
            }
            let bow = rng.range(-0.06, 0.06) * (b - a);
            let k = ((b - a) / 0.25).ceil() as usize + 1;
            let centre: Vec<(f32, f32)> = (0..k).map(|i| at_length(line, &s, a + (b - a) * i as f32 / (k - 1) as f32)).collect();
            let pts: Vec<(f32, f32)> = (0..k)
                .map(|i| {
                    let t = i as f32 / (k - 1) as f32;
                    let (p, q) = (centre[i.saturating_sub(1)], centre[(i + 1).min(k - 1)]);
                    let (dx, dy) = (q.0 - p.0, q.1 - p.1);
                    let l = (dx * dx + dy * dy).sqrt().max(1e-6);
                    let off = bow * 4.0 * t * (1.0 - t);
                    (centre[i].0 - dy / l * off, centre[i].1 + dx / l * off)
                })
                .collect();
            let p = pressure * rng.range(0.85, 1.12);
            let sp = arclen(&pts);
            let mut pr = graphite::pressure_along(&[p * 0.8, p, p * 0.8, p * 0.25], &sp);
            if graded {
                for (q, pt) in pr.iter_mut().zip(&pts) {
                    *q *= sample_in(m, pt.0, pt.1).clamp(0.0, 1.0);
                }
                if pr.iter().cloned().fold(0.0, f32::max) < 0.03 {
                    continue;
                }
            }
            out.push(Mark { pressure: pr, pts });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::Frame;
    use crate::shape::Shape;

    fn square() -> Mask {
        let f = Frame::new(200, 200, 0.2);
        Mask::from_shape(f, Shape::new().rect(200.0, 200.0, 600.0, 600.0))
    }

    fn direction(m: &Mark) -> f32 {
        let (a, b) = (m.pts[0], *m.pts.last().unwrap());
        let d = (b.1 - a.1).atan2(b.0 - a.0);
        // as a direction, in (-π/2, π/2]
        if d > std::f32::consts::FRAC_PI_2 { d - std::f32::consts::PI } else if d <= -std::f32::consts::FRAC_PI_2 { d + std::f32::consts::PI } else { d }
    }

    fn plain<'a>(direction: Direction<'a>) -> AreaHatch<'a> {
        AreaHatch { direction, spacing: 10.0, length: 60.0, pressure: 0.5, graded: false, zigzag: None, jitter: None, open: None }
    }

    #[test]
    fn a_fixed_angle_places_the_graded_hatchings_marks() {
        let m = square();
        let a = area_marks(&m, &plain(Direction::Fixed(-0.9)), 7);
        let b = graphite::hatch_marks_graded(&m, -0.9, 10.0, 60.0, 0.5, false, 7);
        assert_eq!(a.len(), b.len());
        assert!(a.iter().zip(&b).all(|(a, b)| a.pts == b.pts && a.pressure == b.pressure));
    }

    #[test]
    fn a_field_turns_the_marks_and_spaces_them() {
        let m = square();
        // left half at 0, right half at π/2
        let field = |x: f32, _y: f32| if x < 500.0 { 0.0 } else { std::f32::consts::FRAC_PI_2 };
        let marks = area_marks(&m, &plain(Direction::Field(&field)), 3);
        assert!(marks.len() > 20, "{}", marks.len());
        let (mut left, mut right) = (0, 0);
        for mk in &marks {
            let (x, _) = mk.pts[mk.pts.len() / 2];
            let d = direction(mk).abs();
            if x < 480.0 {
                assert!(d < 0.2, "left mark at {d}");
                left += 1;
            } else if x > 520.0 {
                assert!(d > 1.37, "right mark at {d}");
                right += 1;
            }
            assert!(mk.pts.iter().all(|p| p.0 > 195.0 && p.0 < 805.0 && p.1 > 195.0 && p.1 < 805.0));
        }
        assert!(left > 5 && right > 5, "{left} {right}");
        // a constant field: parallel lines about a spacing apart
        let flat = |_: f32, _: f32| 0.0;
        let marks = area_marks(&m, &plain(Direction::Field(&flat)), 3);
        let mut ys: Vec<f32> = marks.iter().map(|mk| mk.pts[0].1).collect();
        ys.sort_by(f32::total_cmp);
        ys.dedup_by(|a, b| (*a - *b).abs() < 3.0);
        let rows = ys.len() as f32;
        assert!((rows - 60.0).abs() <= 4.0, "{rows} rows over 600 units at 10 apart");
    }

    #[test]
    fn a_zigzag_mark_is_one_path_over_a_band() {
        let m = square();
        let mut h = plain(Direction::Fixed(0.0));
        h.spacing = 50.0;
        h.zigzag = Some(Zigzag { amplitude: 20.0, period: 15.0, round: 0.3 });
        let plainm = area_marks(&m, &plain(Direction::Fixed(0.0)), 5);
        let zig = area_marks(&m, &h, 5);
        let mk = &zig[zig.len() / 2];
        let ys: Vec<f32> = mk.pts.iter().map(|p| p.1).collect();
        let band = ys.iter().cloned().fold(f32::MIN, f32::max) - ys.iter().cloned().fold(f32::MAX, f32::min);
        assert!(band > 35.0, "band {band}");
        // back and forth: the direction across the line changes sign many times
        let turns = ys.windows(3).filter(|w| (w[1] - w[0]) * (w[2] - w[1]) < 0.0).count();
        assert!(turns >= 4, "{turns}");
        // consecutive points stay close: one continuous stroke
        assert!(mk.pts.windows(2).all(|w| ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt() < 1.0));
        // the swings stay inside the area across the marks
        assert!(zig.iter().all(|mk| mk.pts.iter().all(|p| p.1 > 199.0 && p.1 < 801.0)));
        assert!(plainm.len() > zig.len());
    }

    #[test]
    fn jitter_and_open_are_seeded_and_independent() {
        let m = square();
        let mut h = plain(Direction::Fixed(0.3));
        h.jitter = Some(Jitter { length: 0.3, angle: 0.3 });
        let a = area_marks(&m, &h, 11);
        let b = area_marks(&m, &h, 11);
        let c = area_marks(&m, &h, 12);
        assert!(a.iter().zip(&b).all(|(a, b)| a.pts == b.pts));
        assert!(a.len() != c.len() || a.iter().zip(&c).any(|(a, c)| a.pts != c.pts));
        let base = area_marks(&m, &plain(Direction::Fixed(0.3)), 11);
        let turned = a.iter().zip(&base).filter(|(a, b)| (direction(a) - direction(b)).abs() > 0.02).count();
        assert!(turned > base.len() / 2, "{turned} of {}", base.len());
        // open drops about its share of the marks, and the same ones whatever the jitter
        h.open = Some(Open::Share(0.5));
        let opened = area_marks(&m, &h, 11);
        let share = opened.len() as f32 / a.len() as f32;
        assert!((0.35..0.65).contains(&share), "{share}");
        h.jitter = None;
        assert_eq!(area_marks(&m, &h, 11).len(), opened.len());
    }
}
