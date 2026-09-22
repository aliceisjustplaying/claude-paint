//! Gnarled oaks in the manner of Friedrich (e.g. "Oak Tree in the Snow",
//! "The Abbey in the Oakwood"): a tall leader that keeps going up, crooked
//! side limbs made of straight lengths joined by sharp elbows, broken stubs,
//! an open crown, and short claw-like twigs all along the outer limbs.
//! Grown recursively, then painted with simulated brushes: each limb is one
//! drag of a round sable sized to it, twigs with a rigger, light with a
//! dry-brush pass on the lit side.

use crate::bristle::{Gesture, Held, Orient, Tool};
use crate::canvas::Canvas;
use crate::color::Rgb;
use crate::mask::Mask;
use crate::rng::Rng;
use crate::shape::Shape;
use crate::wet::Paint;
use std::f32::consts::{FRAC_PI_2, PI};

#[derive(Clone, Debug)]
pub struct Limb {
    pub pts: Vec<(f32, f32)>,
    /// Full width at each point (units).
    pub w: Vec<f32>,
    pub depth: u32,
}

#[derive(Clone, Debug)]
pub struct Oak {
    /// Base of the trunk (units).
    pub base: (f32, f32),
    /// Rough overall height (units).
    pub height: f32,
    /// Trunk width at the base (units).
    pub trunk: f32,
    /// Lean of the trunk in radians (+ = right).
    pub lean: f32,
    /// 0 = straight, 1 = very gnarled.
    pub gnarl: f32,
    /// Branching depth (4–6).
    pub depth: u32,
    /// Probability a limb is broken off (dead oak ≈ 0.3).
    pub broken: f32,
    /// Twig clusters per unit of outer limb length.
    pub twigs: f32,
    /// Exposed roots at the base.
    pub roots: u32,
    pub seed: u64,
}

impl Oak {
    pub fn new(base: (f32, f32), height: f32, seed: u64) -> Self {
        Oak { base, height, trunk: height * 0.06, lean: 0.0, gnarl: 0.7, depth: 5, broken: 0.18, twigs: 0.14, roots: 2, seed }
    }

    pub fn grow(&self) -> Vec<Limb> {
        let mut rng = Rng::new(self.seed);
        let mut out = Vec::new();
        self.leader(&mut out, &mut rng);
        for i in 0..self.roots {
            let side = if i % 2 == 0 { 1.0 } else { -1.0 };
            let a = FRAC_PI_2 - side * rng.range(1.1, 1.45); // out and a little down
            let start = (self.base.0 + side * self.trunk * 0.25, self.base.1 - self.trunk * 0.15);
            let rl = self.height * rng.range(0.05, 0.1);
            let w = self.trunk * 0.55;
            self.limb(&mut out, &mut rng, start, a, rl, w, self.depth, 0.0, false);
        }
        out
    }

    /// The trunk continues upward as a leader, thinning, throwing side limbs.
    fn leader(&self, out: &mut Vec<Limb>, rng: &mut Rng) {
        // the trunk runs up to where it splits into two or three main limbs
        let total = self.height * rng.range(0.38, 0.55);
        let nseg = 5;
        let seg = total / nseg as f32;
        let (mut x, mut y) = self.base;
        let mut a = -FRAC_PI_2 + self.lean;
        let mut pts = vec![(x, y)];
        let mut ws = vec![self.trunk * 1.25]; // flare at the foot
        let mut kids = Vec::new();
        let mut side = if rng.chance(0.5) { 1.0 } else { -1.0 };
        for i in 1..=nseg {
            let t = i as f32 / nseg as f32;
            if i > 1 && rng.chance(0.35 * self.gnarl) {
                a += rng.range(0.15, 0.4) * if rng.chance(0.5) { 1.0 } else { -1.0 };
            }
            a += angle_diff(-FRAC_PI_2 + self.lean, a) * 0.35;
            x += a.cos() * seg;
            y += a.sin() * seg;
            let w = self.trunk * (1.0 - 0.3 * t);
            pts.push((x, y));
            ws.push(w);
            // limbs start a third of the way up, alternating sides
            if t > 0.45 && t < 0.95 && rng.chance(0.6) {
                let n = 1;
                for _ in 0..n {
                    side = -side;
                    let a2 = -FRAC_PI_2 + side * rng.range(0.7, 1.45);
                    let len = self.height * rng.range(0.22, 0.42) * (1.1 - 0.5 * t);
                    kids.push(((x, y), a2, len, w * rng.range(0.5, 0.8), side));
                }
            }
            // stubs of lost limbs
            if rng.chance(0.3 * self.broken / 0.18) {
                let s = if rng.chance(0.5) { 1.0 } else { -1.0 };
                let a2 = -FRAC_PI_2 + s * rng.range(0.9, 1.6);
                let l = w * rng.range(0.8, 2.2);
                out.push(Limb { pts: vec![(x, y), (x + a2.cos() * l, y + a2.sin() * l)], w: vec![w * 0.45, w * 0.4], depth: 1 });
            }
        }
        // the split: main limbs going up and out
        let n = if rng.chance(0.4) { 3 } else { 2 };
        let w_top = *ws.last().unwrap();
        for k in 0..n {
            let u = if n == 1 { 0.0 } else { k as f32 / (n - 1) as f32 * 2.0 - 1.0 };
            let a2 = a + u * rng.range(0.35, 0.7) + rng.normal() * 0.1;
            let s = if u == 0.0 { side_of(a2 + 0.001) } else { u.signum() };
            let len = self.height * rng.range(0.3, 0.45);
            kids.push(((x, y), a2, len, w_top * rng.range(0.6, 0.8), s));
        }
        out.push(Limb { pts, w: ws, depth: 0 });
        for (p, a2, l2, w2, s) in kids {
            self.limb(out, rng, p, a2, l2, w2, 1, s, true);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn limb(
        &self,
        out: &mut Vec<Limb>,
        rng: &mut Rng,
        start: (f32, f32),
        angle: f32,
        len: f32,
        width: f32,
        depth: u32,
        side: f32,
        can_break: bool,
    ) {
        if depth > self.depth + 1 || width < 0.1 || len < 0.8 {
            return;
        }
        let broken = can_break && depth <= 3 && rng.chance(self.broken);
        let len = if broken { len * rng.range(0.2, 0.45) } else { len };
        // a few straight lengths joined by elbows
        let nseg = (2.0 + len / 18.0).clamp(2.0, 5.0) as usize;
        let mut pts = vec![start];
        let mut ws = vec![width];
        let mut a = angle;
        let (mut x, mut y) = start;
        // outer limbs turn back upward at elbows; some droop and recover
        let up = -FRAC_PI_2 + side * rng.range(0.35, 0.8);
        let mut kids = Vec::new();
        for i in 1..=nseg {
            let t = i as f32 / nseg as f32;
            let seg = len / nseg as f32 * rng.range(0.7, 1.3);
            x += a.cos() * seg;
            y += a.sin() * seg;
            let w = width * (1.0 - 0.55 * t);
            pts.push((x, y));
            ws.push(w);
            // elbow: a sharp turn, mostly back toward "up", sometimes away
            let elbow = rng.range(0.3, 0.95) * self.gnarl;
            let toward = angle_diff(up, a).signum();
            a += if rng.chance(0.72) { toward * elbow } else { -toward * elbow * 0.8 };
            if i < nseg && !broken && rng.chance(0.55) {
                let s = if rng.chance(0.5) { 1.0 } else { -1.0 };
                let a2 = a + s * rng.range(0.6, 1.3);
                kids.push(((x, y), a2, len * rng.range(0.45, 0.7), w * rng.range(0.45, 0.7), side_of(a2)));
            }
        }
        let tip = (x, y);
        let tip_w = *ws.last().unwrap();
        out.push(Limb { pts: pts.clone(), w: ws.clone(), depth });
        if broken {
            return;
        }
        for (p, a2, l2, w2, s) in kids {
            self.limb(out, rng, p, a2, l2, w2, depth + 1, s, true);
        }
        if depth < self.depth {
            let l2 = len * rng.range(0.55, 0.8);
            let a2 = a + rng.normal() * 0.25;
            self.limb(out, rng, tip, a2, l2, tip_w * 0.8, depth + 1, side, true);
        }
        // claw-like twig clusters along the outer limbs
        if depth + 2 >= self.depth {
            let total: f32 = pts.windows(2).map(|p| dist(p[0], p[1])).sum();
            let n = (total * self.twigs * rng.range(0.6, 1.4)).round() as usize;
            for _ in 0..n {
                let (bi, t) = pick_along(&pts, rng.f());
                let p0 = lerp(pts[bi], pts[bi + 1], t);
                let w0 = (ws[bi] * 0.4).clamp(0.1, 0.28);
                let mut a2 = -FRAC_PI_2 + rng.normal() * 0.9;
                let mut p = p0;
                let mut tp = vec![p];
                let mut tw = vec![w0];
                let l = self.height * rng.range(0.015, 0.045);
                for k in 0..3 {
                    p = (p.0 + a2.cos() * l / 3.0, p.1 + a2.sin() * l / 3.0);
                    tp.push(p);
                    tw.push(w0 * (0.7 - 0.2 * k as f32).max(0.2));
                    a2 += rng.range(0.3, 0.9) * if rng.chance(0.5) { 1.0 } else { -1.0 };
                }
                out.push(Limb { pts: tp, w: tw, depth: depth + 2 });
            }
        }
    }

    /// Silhouette mask of the limbs (for glazing, mist, clipping).
    pub fn mask(&self, c: &Canvas, limbs: &[Limb]) -> Mask {
        let mut shape = Shape::new();
        for l in limbs {
            shape = shape.ribbon(&l.pts, &l.w);
        }
        Mask::from_shape(c.f, shape)
    }

    /// Paint the tree with simulated brushes: every limb a single drag of a
    /// round sable as wide as the limb (the bristles give bark streaks), twigs
    /// with a rigger, then a light dry-brush pass along the lit edge.
    pub fn paint(&self, c: &mut Canvas, dark: Rgb, light: Option<Rgb>, seed: u64) -> Mask {
        let limbs = self.grow();
        let mut rng = Rng::new(seed);
        // thick to thin, like a painter blocking the tree in
        let mut order: Vec<&Limb> = limbs.iter().collect();
        order.sort_by(|a, b| b.w[0].partial_cmp(&a.w[0]).unwrap());
        let paint = Paint { color: dark, hiding: 0.95, body: 0.9 };
        for l in &order {
            let w0 = l.w[0];
            let w1 = *l.w.last().unwrap();
            let tool = if w0 < 0.9 {
                Tool { width: w0 * 1.6, length: w0 * 3.0, splay: 0.15, stiffness: 0.3, ragged: 0.05, ..Tool::rigger(w0) }
            } else {
                Tool { width: w0, length: w0 * 0.5, ragged: 0.35, ..Tool::round_sable(w0) }
            };
            let mut held = Held::new(tool, rng.next_u64());
            held.load(paint, 1.0);
            // pressure sets the width: full at the base, lighter to the tip
            let p1 = (w1 / w0).clamp(0.15, 1.0);
            let g = Gesture::new(l.pts.clone()).pressure(1.0, p1).ramps(0.0, if l.depth > 2 { 0.4 } else { 0.1 }).orient(Orient::Across);
            c.drag(&mut held, &g, None);
            // a stub from a broken limb ends blunt: dab the end
        }
        if let Some(lc) = light {
            // let the dark dry, then drag a lean light scumble along the lit
            // edge with hardly any pressure: it catches only the ridges of the
            // bark strokes and the canvas tooth, broken like real dry-brush
            c.dry();
            let mask = self.mask(c, &limbs);
            // model each big limb as a lit cylinder: a few thin lean streaks
            // laid along the limb on the side facing the light (upper left),
            // stepping from half-tone near the axis to the lightest near the
            // edge; low pressure so the canvas tooth and the bark ridges break
            // them up, and each covers a random stretch of the limb
            let light_dir = (-0.75f32, -0.66f32);
            for l in limbs.iter().filter(|l| l.w[0] > 2.0) {
                let n = l.pts.len();
                let normals: Vec<(f32, f32)> = (0..n)
                    .map(|i| {
                        let a = l.pts[i.saturating_sub(1)];
                        let b = l.pts[(i + 1).min(n - 1)];
                        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                        let d = (dx * dx + dy * dy).sqrt().max(1e-4);
                        let nn = (-dy / d, dx / d);
                        if nn.0 * light_dir.0 + nn.1 * light_dir.1 >= 0.0 { nn } else { (-nn.0, -nn.1) }
                    })
                    .collect();
                // how squarely the limb faces the light: vertical limbs get a
                // clear lit side, limbs pointing at the light hardly any
                let facing = normals.iter().map(|nn| (nn.0 * light_dir.0 + nn.1 * light_dir.1).max(0.0)).sum::<f32>() / n as f32;
                let streaks = 2 + (l.w[0] / 3.0).min(3.0) as usize;
                for k in 0..streaks {
                    let u = 0.12 + 0.3 * (k as f32 + rng.range(0.0, 0.8)) / streaks as f32;
                    let tone = (0.25 + 0.75 * (k + 1) as f32 / streaks as f32) * (0.4 + 0.6 * facing);
                    let pts: Vec<(f32, f32)> = (0..n)
                        .map(|i| {
                            let w = l.w[i];
                            (l.pts[i].0 + normals[i].0 * w * u, l.pts[i].1 + normals[i].1 * w * u)
                        })
                        .collect();
                    // a random stretch of the limb
                    let a = rng.range(0.0, 0.4);
                    let b = (a + rng.range(0.35, 0.9)).min(1.0);
                    let sub = sub_path(&pts, a, b);
                    let col = crate::color::mix(dark, lc, tone, crate::color::Mix::Pigment);
                    let tool = Tool { ragged: 0.6, ..Tool::round_sable(l.w[0] * rng.range(0.1, 0.18)) };
                    let mut held = Held::new(tool, rng.next_u64());
                    held.load(Paint { color: col, hiding: 0.6, body: 0.4 }, rng.range(0.4, 0.8));
                    let g = Gesture::new(sub).pressure(rng.range(0.3, 0.5), rng.range(0.15, 0.3)).ramps(0.2, 0.4).orient(Orient::Across);
                    c.drag(&mut held, &g, Some(&mask));
                }
            }
            return mask;
        }
        self.mask(c, &limbs)
    }
}

fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn lerp(a: (f32, f32), b: (f32, f32), t: f32) -> (f32, f32) {
    (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t)
}

/// Segment index and fraction at arc-length fraction `u` along a polyline.
fn pick_along(pts: &[(f32, f32)], u: f32) -> (usize, f32) {
    let total: f32 = pts.windows(2).map(|p| dist(p[0], p[1])).sum();
    let mut target = u * total;
    for i in 0..pts.len() - 1 {
        let d = dist(pts[i], pts[i + 1]);
        if target <= d || i == pts.len() - 2 {
            return (i, (target / d.max(1e-6)).clamp(0.0, 1.0));
        }
        target -= d;
    }
    (0, 0.0)
}

fn angle_diff(target: f32, a: f32) -> f32 {
    let mut d = target - a;
    while d > PI {
        d -= 2.0 * PI;
    }
    while d < -PI {
        d += 2.0 * PI;
    }
    d
}

fn side_of(a: f32) -> f32 {
    if a.cos() >= 0.0 { 1.0 } else { -1.0 }
}

/// The part of a polyline between arc-length fractions `a` and `b`.
fn sub_path(pts: &[(f32, f32)], a: f32, b: f32) -> Vec<(f32, f32)> {
    let (i0, t0) = pick_along(pts, a);
    let (i1, t1) = pick_along(pts, b);
    let mut out = vec![lerp(pts[i0], pts[i0 + 1], t0)];
    for p in &pts[i0 + 1..=i1] {
        out.push(*p);
    }
    out.push(lerp(pts[i1], pts[i1 + 1], t1));
    out
}
