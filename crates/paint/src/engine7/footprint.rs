//! Brush footprints for engine 7 (lane STIL-67).
//!
//! Contract (frozen, CONTRACTS-e7.md):
//! - `conservative` is the legacy safety bound, `bristle::footprint`.
//! - `exact(held, g, id, ..)` is every pixel `drag_on` reads or writes for
//!   that brush, gesture and stroke id (capsules, plough destinations and the
//!   exchange's ±(rb + 1) reads), never clipped by a mask. It is a subset of
//!   `conservative`. Pointed tools may fall back to a bound.
//! - `planned(tool, pts, shake, orient, ..)` bounds `exact` for every brush of that
//!   tool and every stroke id, before the brush that paints the stroke is
//!   known: `work` schedules tiles by it, so it must be a true bound, never
//!   an estimate.
//!
//! `bristle::footprint` pads a gesture's path on every side by the most any
//! hair could ever reach: the widest splay, the longest bend, roots at their
//! limit in every direction. For a 24 mm filbert at 2400 px that is ~100 px
//! a side around a ~64 px mark, and `work` sizes its tiles at twice the
//! largest reach. Both `exact` and `planned` instead run `drag_on`'s
//! geometry dry, without any exchange: the resampled path, the hand's
//! position and direction step by step, the half-width at the pressure, the
//! bend relaxing toward its targets behind the motion, the capsules and the
//! plough. `exact` follows every hair of the brush it is given. `planned`
//! doesn't know the brush (a tile's brush is seeded by the tile, which the
//! footprint decides), the stroke id (it seeds the hand's wander and
//! breathing) or the pressure, so it bounds the hair layout by `Held::new`'s
//! for the tool's kind and the rest by their ranges.

use super::Rect;
use crate::bristle::{FINE_RB, Gesture, Held, Kind, LEN_MAX, Orient, ROOT_MAX, Tool, footprint, wander};

pub struct Footprint;

impl Footprint {
    /// The pixels a drag of `held` through `g` as stroke `id` reads or writes,
    /// on a `w` × `h` px canvas at `scale` px per unit.
    pub fn exact(held: &Held, g: &Gesture, id: u32, scale: f32, w: usize, h: usize) -> Option<Rect> {
        let r = if held.tool.point > 0.0 { bound(&held.tool, &g.pts, g.shake, g.orient, Some(g), scale, w, h) } else { hairs(held, g, id, scale, w, h) };
        within(r?, Self::conservative(&held.tool, g, scale, w, h)?)
    }

    /// The legacy safety bound (`bristle::footprint`).
    pub fn conservative(tool: &Tool, g: &Gesture, scale: f32, w: usize, h: usize) -> Option<Rect> {
        footprint(tool, &g.pts, g.shake, scale, w, h)
    }

    /// A bound on `exact` over every brush of `tool` and every stroke id, for
    /// a stroke along `pts` (units) with hand shake `shake`, held as `orient`,
    /// at any pressure: what `work` schedules by.
    pub fn planned(tool: &Tool, pts: &[(f32, f32)], shake: f32, orient: Orient, scale: f32, w: usize, h: usize) -> Option<Rect> {
        within(bound(tool, pts, shake, orient, None, scale, w, h)?, footprint(tool, pts, shake, scale, w, h)?)
    }

    /// How far (units, per axis) to size a covering pass's tiles for, from
    /// its strokes' anchors (units) and planned footprints (pixels at
    /// §scale§): the reach that all but the longest tenth of its strokes
    /// stay within. A tile twice that wide runs beside its checkerboard
    /// neighbors' neighbors; the few strokes reaching farther only make
    /// their tile's rectangle overlap more, and §run_ordered§ keeps
    /// overlapping tiles in order, so the result is the same however many
    /// workers paint it. (The longest strokes, often carried in from a
    /// region's edge, reach 1.6–2× as far as the rest: sized by them, a
    /// pass's tiles are too big to run more than a few at once.)
    pub fn tile_reach(strokes: impl IntoIterator<Item = ((f32, f32), Rect)>, scale: f32) -> (f32, f32) {
        let (mut rx, mut ry): (Vec<f32>, Vec<f32>) = strokes
            .into_iter()
            .map(|((cx, cy), r)| {
                let (px, py) = (cx * scale, cy * scale);
                ((px - r.0 as f32).max(r.2 as f32 - px) / scale, (py - r.1 as f32).max(r.3 as f32 - py) / scale)
            })
            .unzip();
        let at = |v: &mut Vec<f32>| {
            v.sort_by(f32::total_cmp);
            v.get(((v.len() as f32 * 0.9) as usize).min(v.len().saturating_sub(1))).copied().unwrap_or(0.0)
        };
        (at(&mut rx), at(&mut ry))
    }
}

/// `r` cut to `c` (engine 5's `drag_on` clamps every access to `c`).
fn within(r: Rect, c: Rect) -> Option<Rect> {
    let r = (r.0.max(c.0), r.1.max(c.1), r.2.min(c.2), r.3.min(c.3));
    (r.2 > r.0 && r.3 > r.1).then_some(r)
}

/// `Rng::normal` is a sum of four uniforms: it never leaves ±2·1.732.
const NORMAL_MAX: f32 = 2.0 * 1.732;

/// Bounds on a bristle's root offset in the brush frame (half-widths) for a
/// tool kind, as `Held::new` lays them out with their jitter: |rx|, |ry| and
/// the distance from the axis.
fn roots(kind: Kind) -> (f32, f32, f32) {
    let j = 0.02 * NORMAL_MAX;
    let (rx, ry, r) = match kind {
        // a disc (√t < 1), and a filbert's squashed to 0.3 across
        Kind::Round | Kind::Rigger | Kind::Blender => (1.0, 1.0, 1.0),
        Kind::Filbert => (1.0, 0.3, 1.0),
        Kind::Flat => (1.0, 0.18, 1.0f32.hypot(0.18)),
        Kind::Fan => {
            let (rx, ry) = (0.95f32.sin(), 0.35 * (1.0 - 0.95f32.cos()) + 0.04);
            (rx, ry, rx.hypot(ry))
        }
    };
    ((rx + j).min(ROOT_MAX), (ry + j).min(ROOT_MAX), (r + j * std::f32::consts::SQRT_2).min(ROOT_MAX * std::f32::consts::SQRT_2))
}

/// The gesture's pressure at `u` before the hand's breathing
/// (`Gesture::pressure_at`).
fn pressure_at(g: &Gesture, u: f32) -> f32 {
    let k = &g.swell;
    let swell = match k.len() {
        0 => 1.0,
        1 => k[0],
        n => {
            let t = u.clamp(0.0, 1.0) * (n - 1) as f32;
            let i = (t as usize).min(n - 2);
            let s = crate::smoothstep(0.0, 1.0, t - i as f32);
            k[i] + (k[i + 1] - k[i]) * s
        }
    };
    let base = (g.pressure.0 + (g.pressure.1 - g.pressure.0) * u) * swell;
    let a = if g.attack > 0.0 { 0.08 + 0.92 * crate::smoothstep(0.0, g.attack, u) } else { 1.0 };
    let r = if g.release > 0.0 { 0.05 + 0.95 * crate::smoothstep(0.0, g.release, 1.0 - u) } else { 1.0 };
    base * a * r
}

/// `drag_on`'s walk along the resampled path (pixels): for each step, the
/// distance along it, the hand (before its wander) and the direction.
fn walk(pts: &[(f32, f32)], s: f32, step: f32, mut f: impl FnMut(f32, f32, (f32, f32), (f32, f32))) {
    let pts: Vec<(f32, f32)> = pts.iter().map(|&(x, y)| (x * s, y * s)).collect();
    let path = if pts.len() >= 2 { crate::path::densify(&pts) } else { vec![pts[0], pts[0]] };
    let mut arc = vec![0.0f32; path.len()];
    for i in 1..path.len() {
        let (dx, dy) = (path[i].0 - path[i - 1].0, path[i].1 - path[i - 1].1);
        arc[i] = arc[i - 1] + (dx * dx + dy * dy).sqrt();
    }
    let total = arc[arc.len() - 1];
    let nsteps = ((total / step).ceil() as usize).max(1);
    let mut seg = 0usize;
    let mut last_dir = (1.0f32, 0.0f32);
    for k in 0..=nsteps {
        let d = (k as f32 * step).min(total);
        while seg + 1 < path.len() - 1 && arc[seg + 1] < d {
            seg += 1;
        }
        let seg_len = (arc[seg + 1] - arc[seg]).max(1e-6);
        let f0 = ((d - arc[seg]) / seg_len).clamp(0.0, 1.0);
        let (ax, ay) = path[seg];
        let (bx, by) = path[seg + 1];
        let dir = if seg_len > 1e-3 { ((bx - ax) / seg_len, (by - ay) / seg_len) } else { last_dir };
        last_dir = dir;
        f(d, total, (ax + (bx - ax) * f0, ay + (by - ay) * f0), dir);
    }
}

/// The brush's orientation for direction `dir` (`drag_on`'s `theta`).
fn theta(orient: Orient, dir: (f32, f32)) -> f32 {
    match orient {
        Orient::Across => dir.1.atan2(dir.0) + std::f32::consts::FRAC_PI_2,
        Orient::Along => dir.1.atan2(dir.0),
        Orient::Fixed(a) => a,
    }
}

/// The hair radius a drag draws with (pixels).
fn hair_rb(tool: &Tool, s: f32) -> f32 {
    (tool.hair_radius() * s).max(if tool.point > 0.0 { FINE_RB } else { 0.55 })
}

/// How fast a hair's bend relaxes toward its target per step.
fn bend_rate(tool: &Tool, step: f32, s: f32) -> f32 {
    let bend_len = tool.length * (0.25 + 0.75 * (1.0 - tool.stiffness));
    (1.0 - (-(step / s) / (bend_len.max(0.0) + 1e-3)).exp()).clamp(0.0, 1.0)
}

/// The pixel rectangle a box of capsule ends (x0, y0, x1, y1, pixels)
/// reaches: `exchange` reads the capsule ± (radius `rk` + 1), rounded out,
/// and its plough moves paint from a pixel in contact (center within
/// `rk` + √½ of the capsule) 0.875 of `off` away, shared between the four
/// pixels around the target; `off` is twice the hair, or up to 3 px for a
/// clump. Plus a pixel for rounding.
fn reach(tool: &Tool, b: (f32, f32, f32, f32), rk: f32, off: f32, w: usize, h: usize) -> Option<Rect> {
    let r = if tool.push > 0.0 { (rk + 1.0).max(rk + std::f32::consts::FRAC_1_SQRT_2 + 0.875 * off + 0.5) } else { rk + 1.0 };
    let pad = r + 1.0;
    let (x0, y0, x1, y1) = (b.0 - pad, b.1 - pad, b.2 + pad, b.3 + pad);
    if b.0 > b.2 || b.1 > b.3 || !x0.is_finite() || !y0.is_finite() || !x1.is_finite() || !y1.is_finite() {
        return None;
    }
    let c = |v: f32, n: usize| (v.max(0.0) as usize).min(n);
    let r = (c(x0.floor(), w), c(y0.floor(), h), c(x1.ceil() + 1.0, w), c(y1.ceil() + 1.0, h));
    (r.2 > r.0 && r.3 > r.1).then_some(r)
}

/// `exact` for a blunt brush: every hair's contacts, as `drag_on` moves
/// them (nothing in a blunt brush's geometry depends on its paint).
fn hairs(held: &Held, g: &Gesture, id: u32, scale: f32, w: usize, h: usize) -> Option<Rect> {
    if g.pts.is_empty() {
        return None;
    }
    let (tool, s) = (&held.tool, scale);
    let rb = hair_rb(tool, s);
    let step = rb.max(1.25);
    let rate = bend_rate(tool, step, s);
    let tip = held.bristles.iter().enumerate().min_by(|a, b| a.1.thresh.total_cmp(&b.1.thresh)).map_or(0, |(i, _)| i);
    let mut state: Vec<((f32, f32), bool)> = vec![((0.0, 0.0), false); held.bristles.len()];
    let lw = (tool.width * 2.5 + 6.0) * s;
    let hs = id as u64 * 7919;
    let mut b = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    walk(&g.pts, s, step, |d, total, (hx, hy), dir| {
        let off = g.shake * (0.05 * tool.width + 0.12) * s * (wander(d / lw, hs + 1) + 0.5 * wander(d / (lw * 0.37), hs + 2));
        let (hx, hy) = (hx - dir.1 * off, hy + dir.0 * off);
        let u = if total > 0.0 { d / total } else { 0.5 };
        let p = (pressure_at(g, u) * (1.0 + g.shake * 0.14 * wander(d / (lw * 0.8), hs + 3))).clamp(0.0, 1.0);
        let (st, ct) = theta(g.orient, dir).sin_cos();
        let half = tool.width * 0.5 * s * (0.45 + 0.55 * p) * (1.0 + tool.splay * (p - 0.5));
        for (bi, (br, (bend, down))) in held.bristles.iter().zip(state.iter_mut()).enumerate() {
            let reach = (p - br.thresh) / (1.0 - br.thresh).max(1e-3);
            if reach <= 0.0 && !(bi == tip && p > 0.0) {
                *down = false;
                continue;
            }
            let wv = wander(d / s / tool.width.max(2.0) * 1.3, br.seed) * tool.ragged;
            let (ox, oy) = ((br.rx + wv * 0.12) * half, (br.ry + wv * 0.05) * half);
            let root = (hx + ox * ct - oy * st, hy + ox * st + oy * ct);
            let trail = tool.length * s * br.len * p;
            let spread = tool.splay * p * 0.3;
            let target = (-dir.0 * trail + (ox * ct - oy * st) * spread, -dir.1 * trail + (ox * st + oy * ct) * spread);
            bend.0 += (target.0 - bend.0) * rate;
            bend.1 += (target.1 - bend.1) * rate;
            // (the capsule from its last contact, already in the box)
            let cur = (root.0 + bend.0 * 0.6, root.1 + bend.1 * 0.6);
            b = (b.0.min(cur.0), b.1.min(cur.1), b.2.max(cur.0), b.3.max(cur.1));
            *down = true;
        }
    });
    reach(tool, b, rb, (2.0 * (tool.hair_radius() * s).min(rb)).max(3.0), w, h)
}

/// The largest |half-width| (pixels, before a pointed tuft gathers, which
/// only narrows it) over pressures `lo..=hi`: `drag_on`'s `half` is a
/// quadratic in the pressure.
fn half_max(tool: &Tool, s: f32, lo: f32, hi: f32) -> f32 {
    let f = |p: f32| (tool.width * 0.5 * s * (0.45 + 0.55 * p) * (1.0 + tool.splay * (p - 0.5))).abs();
    let mut m = f(lo).max(f(hi));
    if tool.splay > 0.0 {
        let c = 1.0 - 0.5 * tool.splay;
        let v = -(0.45 * tool.splay + 0.55 * c) / (1.1 * tool.splay);
        if v > lo && v < hi {
            m = m.max(f(v));
        }
    }
    m
}

/// A bound on the contacts of every brush of `tool` along `pts` held as
/// `orient`, for any stroke id: at the gesture `g`'s pressure if known
/// (else any).
#[allow(clippy::too_many_arguments)]
fn bound(tool: &Tool, pts: &[(f32, f32)], shake: f32, orient: Orient, g: Option<&Gesture>, scale: f32, w: usize, h: usize) -> Option<Rect> {
    if pts.is_empty() {
        return None;
    }
    let s = scale;
    let pointed = tool.point > 0.0;
    let rb = hair_rb(tool, s);
    let step = rb.max(1.25);
    let rate = bend_rate(tool, step, s);
    // hair layout: roots (with the wander across the stroke, |wv| ≤ ragged)
    // and lengths
    let (rx, ry, rr) = roots(tool.kind);
    let rag = tool.ragged;
    let (ox, oy, or) = (rx + 0.12 * rag, ry + 0.05 * rag, rr + 0.13 * rag);
    let (lmin, lmax) = ((1.0 - 0.15 * rag * NORMAL_MAX).clamp(0.4, LEN_MAX), (1.0 + 0.15 * rag * NORMAL_MAX).clamp(0.4, LEN_MAX));
    let trail = tool.length * s;
    // the hand's sideways wander and pressure breathing, at their amplitude
    let drift = shake.abs() * (0.05 * tool.width + 0.12) * s * 1.5;
    let breathe = (shake * 0.14).abs();
    // contacts' box, and the bend's bounds per axis: every hair's bend is a
    // running blend toward its targets, moved only on the steps it touches,
    // so it stays between bounds that relax the same way toward theirs
    let mut b = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    let (mut bx, mut by) = ((0.0f32, 0.0f32), (0.0f32, 0.0f32));
    let relax = |b: &mut (f32, f32), lo: f32, hi: f32| {
        b.0 += rate * (lo - b.0).min(0.0);
        b.1 += rate * (hi - b.1).max(0.0);
    };
    walk(pts, s, step, |d, total, (hx, hy), dir| {
        let (plo, phi) = match g {
            Some(g) => {
                let p = pressure_at(g, if total > 0.0 { d / total } else { 0.5 });
                let (pa, pb) = (p * (1.0 - breathe), p * (1.0 + breathe));
                (pa.min(pb).clamp(0.0, 1.0), pa.max(pb).clamp(0.0, 1.0))
            }
            None => (0.0, 1.0),
        };
        // no hair touches without pressure (not even the tip)
        if phi <= 0.0 {
            return;
        }
        let half = half_max(tool, s, plo, phi);
        // roots around the hand, rotated into the canvas
        let (st, ct) = theta(orient, dir).sin_cos();
        let (rot_x, rot_y) = ((ox * ct.abs() + oy * st.abs()).min(or) * half, (ox * st.abs() + oy * ct.abs()).min(or) * half);
        // targets: tips trail behind the motion and splay outward
        let spread = tool.splay * phi * 0.3;
        let (ta, tb) = (-trail * lmin * plo, -trail * lmax * phi);
        relax(&mut bx, (dir.0 * ta).min(dir.0 * tb) - spread * rot_x, (dir.0 * ta).max(dir.0 * tb) + spread * rot_x);
        relax(&mut by, (dir.1 * ta).min(dir.1 * tb) - spread * rot_y, (dir.1 * ta).max(dir.1 * tb) + spread * rot_y);
        let (sx, sy) = (dir.1.abs() * drift, dir.0.abs() * drift);
        b = (b.0.min(hx - sx - rot_x + 0.6 * bx.0), b.1.min(hy - sy - rot_y + 0.6 * by.0), b.2.max(hx + sx + rot_x + 0.6 * bx.1), b.3.max(hy + sy + rot_y + 0.6 * by.1));
    });
    // a pointed tuft's hairs lay tracks shifted across by up to its widest
    // half-width, up to that wide, and one that has just come down reaches
    // back a step
    let widest = tool.width * 0.5 * s * (1.0 + tool.splay * 0.5);
    let (rk, extra, off) = if pointed { (widest.max(rb), widest + step, 2.0 * widest.max(rb)) } else { (rb, 0.0, (2.0 * rb).max(3.0)) };
    reach(tool, (b.0 - extra, b.1 - extra, b.2 + extra, b.3 + extra), rk, off, w, h)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::Canvas;
    use crate::color::hex;
    use crate::rng::Rng;
    use crate::wet::Paint;

    fn area(r: Option<Rect>) -> f64 {
        r.map_or(0.0, |r| ((r.2 - r.0) * (r.3 - r.1)) as f64)
    }

    fn inside(a: Option<Rect>, b: Option<Rect>) -> bool {
        match (a, b) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(a), Some(b)) => a.0 >= b.0 && a.1 >= b.1 && a.2 <= b.2 && a.3 <= b.3,
        }
    }

    /// A random valid tool: a preset, maybe pushed to an extreme.
    fn tool(rng: &mut Rng) -> Tool {
        let w = rng.range(0.5, 40.0);
        let t = match (rng.f() * 7.0) as usize {
            0 => Tool::round_sable(w),
            1 => Tool::hog_flat(w),
            2 => Tool::filbert(w),
            3 => Tool::fan(w),
            4 => Tool::rigger(w.min(4.0)),
            5 => Tool::badger(w),
            _ => Tool::stippler(w),
        };
        let t = if rng.chance(0.3) {
            Tool { splay: rng.range(0.0, 4.0), ragged: rng.range(0.0, 3.0), stiffness: rng.f(), length: t.length * rng.range(0.0, 4.0), ..t }
        } else {
            t
        };
        let t = if rng.chance(0.15) { Tool { point: rng.range(0.2, 1.0), ..t } } else { t };
        t.validate().unwrap();
        t
    }

    /// A random gesture over a 1000-unit canvas: 1 to 6 points, any
    /// pressure profile, orientation and hand.
    fn gesture(rng: &mut Rng) -> Gesture {
        let n = 1 + (rng.f() * 6.0) as usize;
        let (mut x, mut y) = (rng.range(100.0, 900.0), rng.range(100.0, 900.0));
        let step = rng.range(1.0, 120.0);
        let mut pts = Vec::with_capacity(n);
        for _ in 0..n {
            pts.push((x, y));
            x += rng.range(-step, step);
            y += rng.range(-step, step);
        }
        let orient = match (rng.f() * 3.0) as usize {
            0 => Orient::Across,
            1 => Orient::Along,
            _ => Orient::Fixed(rng.range(-3.2, 3.2)),
        };
        let swell = if rng.chance(0.4) { (0..2 + (rng.f() * 3.0) as usize).map(|_| rng.range(0.35, 1.6)).collect() } else { Vec::new() };
        Gesture::new(pts).pressure(rng.f(), rng.f()).orient(orient).ramps(rng.range(0.0, 0.3), rng.range(0.0, 0.3)).shake(rng.range(0.0, 3.0)).swell(swell)
    }

    /// On 1000 random gestures across tools: exact ⊆ planned ⊆
    /// conservative, with the bounds before they are held to the
    /// conservative one, and the exact footprint much the smallest.
    #[test]
    fn exact_lies_in_planned_lies_in_conservative() {
        let mut rng = Rng::new(7);
        let (mut sum_c, mut sum_p, mut sum_e) = (0.0, 0.0, 0.0);
        for i in 0..1000 {
            let (t, g) = (tool(&mut rng), gesture(&mut rng));
            let scale = rng.range(0.1, 3.0);
            let (w, h) = ((1000.0 * scale) as usize, (1000.0 * scale) as usize);
            let held = Held::new(t.clone(), rng.f().to_bits() as u64);
            let id = (rng.f() * 1e6) as u32;
            let c = footprint(&t, &g.pts, g.shake, scale, w, h);
            let p = bound(&t, &g.pts, g.shake, g.orient, None, scale, w, h);
            let e = if t.point > 0.0 { bound(&t, &g.pts, g.shake, g.orient, Some(&g), scale, w, h) } else { hairs(&held, &g, id, scale, w, h) };
            assert!(inside(p, c), "{i}: planned {p:?} outside conservative {c:?} for {t:?} {g:?} at {scale}");
            assert!(inside(e, p), "{i}: exact {e:?} outside planned {p:?} for {t:?} {g:?} at {scale}");
            assert_eq!(Footprint::planned(&t, &g.pts, g.shake, g.orient, scale, w, h), p, "{i}");
            assert_eq!(Footprint::exact(&held, &g, id, scale, w, h), e, "{i}");
            sum_c += area(c);
            sum_p += area(p);
            sum_e += area(e);
        }
        assert!(sum_e < sum_p && sum_p < sum_c * 0.8, "exact {sum_e}, planned {sum_p}, conservative {sum_c}");
    }

    /// Every pixel a drag reads or writes lies in its exact footprint (and
    /// so its planned one): contact (the stroke's `touched` mark, on a film
    /// thick enough that every pixel under a hair is felt), deposit
    /// (`stroke`), pickup and plough (any change of volume or cover).
    #[test]
    fn exact_bounds_every_read_and_write() {
        let mut rng = Rng::new(11);
        for i in 0..200 {
            let (t, g) = (tool(&mut rng), gesture(&mut rng));
            let scale = [0.1f32, 0.25, 0.5][i % 3];
            for v in [5, 7] {
                let mut c = Canvas::new((1000.0 * scale) as usize, 1.0, hex("#c8b89a")).with_engine(v);
                // surface ≥ 1.2 everywhere: every hair's contact weight is its cover
                c.wet.vol.iter_mut().for_each(|v| *v = 4.0);
                let (vol, cover) = (c.wet.vol.clone(), c.wet.cover.clone());
                let mut held = Held::new(t.clone(), i as u64).with_engine(v);
                held.load(Paint::body(hex("#d0c060")), [0.1, 0.5, 1.0][i % 3]);
                let id = c.wet.current.wrapping_add(1).max(1);
                let r = Footprint::exact(&held, &g, id, c.f.scale, c.f.w, c.f.h);
                let plan = Footprint::planned(&t, &g.pts, g.shake, g.orient, c.f.scale, c.f.w, c.f.h);
                assert!(inside(r, plan), "{i}: exact {r:?} outside planned {plan:?}");
                c.drag(&mut held, &g, None);
                assert_eq!(c.wet.current - 1, id);
                for k in 0..c.wet.vol.len() {
                    let wet = &c.wet;
                    if wet.touched[k] == id || wet.stroke[k] == id || wet.vol[k] != vol[k] || wet.cover[k] != cover[k] {
                        let (x, y) = (k % c.f.w, k / c.f.w);
                        let r = r.unwrap_or_else(|| panic!("{i}: ({x},{y}) painted with no exact footprint, {t:?} {g:?}"));
                        assert!(x >= r.0 && x < r.2 && y >= r.1 && y < r.3, "{i} engine {v}: ({x},{y}) outside {r:?} for {t:?} {g:?} at {scale}");
                    }
                }
            }
        }
    }

    #[test]
    fn tiles_are_sized_for_all_but_the_longest_tenth() {
        let strokes = (0..20).map(|k| ((100.0, 100.0), (100 - k, 90, 110 + 10 * k, 100 + k / 2)));
        assert_eq!(Footprint::tile_reach(strokes, 1.0), (190.0, 10.0));
        assert_eq!(Footprint::tile_reach([], 2.0), (0.0, 0.0));
    }

    /// An engine 7 covering pass paints the same on one worker as on four.
    #[test]
    fn work_paints_the_same_on_any_number_of_workers() {
        use crate::handling::Handling;
        use crate::mask::Mask;
        let paint = |n: usize| {
            rayon::ThreadPoolBuilder::new().num_threads(n).build().unwrap().install(|| {
                let mut c = Canvas::new(300, 1.0, hex("#c8b89a")).with_engine(7);
                let m = Mask::from_fn(c.frame(), |x, y| if (x - 500.0).abs() < 380.0 && (y - 500.0).abs() < 300.0 + 0.3 * x { 1.0 } else { 0.0 });
                c.work(&m, &Handling::new(Tool::filbert(24.0)).color(|_, _| hex("#803020")).length(30.0, 300.0), 5);
                c.work(&m, &Handling::new(Tool::hog_flat(12.0)).color(|x, _| if x < 450.0 { hex("#a9785a") } else { hex("#50607a") }).coverage(3.0), 6);
                c.wet.vol.iter().chain(c.wet.cover.iter()).map(|v| v.to_bits()).collect::<Vec<_>>()
            })
        };
        assert_eq!(paint(1), paint(4));
    }

    /// A 24 mm filbert's 30 mm stroke at 2400 px on a 900 mm canvas: the
    /// planned footprint is a few times smaller than the conservative one.
    #[test]
    fn a_filbert_stroke_is_much_smaller_than_its_padding() {
        let t = Tool::filbert(24.0);
        let scale = 2400.0 / 900.0;
        let pts = [(400.0, 400.0), (415.0, 401.0), (430.0, 400.0)];
        let (c, p) = (footprint(&t, &pts, 1.0, scale, 2400, 2400), Footprint::planned(&t, &pts, 1.0, Orient::Across, scale, 2400, 2400));
        assert!(area(c) > 2.0 * area(p), "conservative {c:?}, planned {p:?}");
    }
}
