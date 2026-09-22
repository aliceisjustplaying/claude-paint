//! Gnarled trees in the manner of Friedrich's oaks: short heavy trunks,
//! limbs that zigzag with sudden elbows, broken stubs, horizontal reach, and
//! a dense tangle of fine twigs at the ends. Grown recursively.

use crate::brush::{Brush, Medium};
use crate::canvas::Canvas;
use crate::color::Rgb;
use crate::mask::Mask;
use crate::rng::Rng;
use crate::shape::Shape;
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
    /// Branching depth (5–8).
    pub depth: u32,
    /// Probability a limb is broken off (dead oak ≈ 0.25).
    pub broken: f32,
    /// Twigs per branch tip.
    pub twigs: u32,
    /// Exposed roots at the base.
    pub roots: u32,
    pub seed: u64,
}

impl Oak {
    pub fn new(base: (f32, f32), height: f32, seed: u64) -> Self {
        Oak {
            base,
            height,
            trunk: height * 0.07,
            lean: 0.0,
            gnarl: 0.6,
            depth: 6,
            broken: 0.12,
            twigs: 5,
            roots: 0,
            seed,
        }
    }

    pub fn grow(&self) -> Vec<Limb> {
        let mut rng = Rng::new(self.seed);
        let mut out = Vec::new();
        // trunk: up (−π/2) plus lean
        let trunk_len = self.height * rng.range(0.32, 0.42);
        self.limb(&mut out, &mut rng, self.base, -FRAC_PI_2 + self.lean, trunk_len, self.trunk, 0, 0.0);
        for i in 0..self.roots {
            let side = if i % 2 == 0 { 1.0 } else { -1.0 };
            let a = FRAC_PI_2 - side * rng.range(0.9, 1.4); // down and out
            let start = (self.base.0 + side * self.trunk * 0.3, self.base.1 - self.trunk * 0.3);
            let rl = self.height * rng.range(0.08, 0.16);
            self.limb(&mut out, &mut rng, start, a, rl, self.trunk * 0.45, self.depth.saturating_sub(2), 0.0);
        }
        out
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
    ) {
        if depth > self.depth || width < 0.12 || len < 0.8 {
            return;
        }
        let broken = depth >= 1 && depth <= 3 && rng.chance(self.broken);
        let len = if broken { len * rng.range(0.25, 0.5) } else { len };
        let nseg = (4.0 + len / 12.0).min(9.0) as usize;
        let seg = len / nseg as f32;
        let mut pts = vec![start];
        let mut ws = vec![width];
        let mut a = angle;
        let (mut x, mut y) = start;
        // oaks reach sideways: deeper limbs are pulled toward horizontal-ish
        let target = if depth == 0 {
            -FRAC_PI_2 + self.lean
        } else {
            let out_a = if side >= 0.0 { -0.35 } else { -PI + 0.35 };
            out_a
        };
        let pull = if depth == 0 { 0.25 } else { 0.12 + 0.05 * depth as f32 };
        let mut kids = Vec::new();
        for i in 1..=nseg {
            // gnarl: jitter plus occasional sharp elbows
            a += rng.normal() * 0.18 * self.gnarl;
            if rng.chance(0.18 * self.gnarl) {
                a += rng.range(0.35, 0.8) * if rng.chance(0.5) { 1.0 } else { -1.0 };
            }
            a += angle_diff(target, a) * pull;
            x += a.cos() * seg;
            y += a.sin() * seg;
            let t = i as f32 / nseg as f32;
            let w = width * (1.0 - 0.45 * t);
            pts.push((x, y));
            ws.push(w);
            // side branches
            let p_branch = if depth == 0 { 0.35 } else { 0.45 };
            if i < nseg && i >= 1 && rng.chance(p_branch) && !broken {
                let s = if rng.chance(0.5) { 1.0 } else { -1.0 };
                kids.push(((x, y), a + s * rng.range(0.5, 1.1), len * rng.range(0.4, 0.7), w * rng.range(0.45, 0.7), s));
            }
        }
        let tip = (x, y);
        let tip_w = *ws.last().unwrap();
        out.push(Limb { pts, w: ws, depth });

        if broken {
            return; // a blunt stub: nothing grows past the break
        }
        for (p, a2, l2, w2, s) in kids {
            let side = if depth == 0 { s } else { side_of(a2) };
            self.limb(out, rng, p, a2, l2, w2, depth + 1, side);
        }
        if depth < self.depth {
            // fork at the tip
            let spread = rng.range(0.35, 0.8);
            let l2 = len * rng.range(0.6, 0.85);
            for s in [-1.0f32, 1.0] {
                let a2 = a + s * spread * rng.range(0.6, 1.2);
                let w2 = tip_w * rng.range(0.6, 0.8);
                let side = if depth == 0 { s } else { side_of(a2) };
                self.limb(out, rng, tip, a2, l2, w2, depth + 1, side);
            }
        } else {
            // fine twigs: short, jagged, thin
            for _ in 0..self.twigs {
                let mut a2 = a + rng.normal() * 0.9;
                let mut p = tip;
                let mut pts = vec![p];
                let mut ws = vec![tip_w.min(0.35)];
                let l = len * rng.range(0.3, 0.8);
                for _ in 0..3 {
                    a2 += rng.normal() * 0.4;
                    p = (p.0 + a2.cos() * l / 3.0, p.1 + a2.sin() * l / 3.0);
                    pts.push(p);
                    ws.push(0.15);
                }
                out.push(Limb { pts, w: ws, depth: depth + 1 });
            }
        }
    }

    /// Paint the tree as a dark silhouette: limbs thicker than ~1 unit are
    /// filled shapes, thinner ones are brush strokes. `light` rims the limbs
    /// on the side toward the light.
    pub fn paint(&self, c: &mut Canvas, dark: Rgb, light: Option<Rgb>, seed: u64) -> Mask {
        let limbs = self.grow();
        let mut shape = Shape::new();
        let mut thin = Vec::new();
        for l in &limbs {
            if l.w[0] > 1.0 {
                shape = shape.ribbon(&l.pts, &l.w);
            } else {
                thin.push(l);
            }
        }
        let mask = Mask::from_shape(c.f, shape);
        // bark: dark body painted with short strokes along the grain
        c.fill(&mask, dark, 0.95, crate::color::Mix::Linear);
        for (i, l) in thin.iter().enumerate() {
            let w = (l.w.iter().sum::<f32>() / l.w.len() as f32).max(0.18);
            let b = Brush::default().width(w).soft(0.35).streak(0.0, 1.0).taper(0.0, 0.7).load(1.0, 0.1).impasto(0.3);
            c.stroke(&b, Medium::Body(dark, 0.95), &l.pts, seed + i as u64);
        }
        if let Some(lc) = light {
            // thin highlight along the upper-left edge of big limbs
            for (i, l) in limbs.iter().enumerate().filter(|(_, l)| l.w[0] > 2.0) {
                let pts: Vec<(f32, f32)> = l
                    .pts
                    .iter()
                    .zip(&l.w)
                    .map(|(&(x, y), &w)| (x - w * 0.28, y - w * 0.18))
                    .collect();
                let w = l.w[0] * 0.18;
                let b = Brush::default().width(w).soft(0.8).streak(0.6, 5.0).load(0.7, 0.7).impasto(0.4);
                c.stroke_clipped(&b, Medium::Light(lc, 0.35), &pts, seed + 10_000 + i as u64, Some(&mask));
            }
        }
        mask
    }
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
