//! Rückenfigur: a figure seen from behind, as in Friedrich's paintings.
//!
//! Built from a simple proportional skeleton (feet at `feet`, total height
//! `height`, y grows downward). Arms are solved with two-bone IK so hands can
//! rest on a stick, a rock or another figure's shoulder.

use crate::brush::{Brush, Medium};
use crate::canvas::Canvas;
use crate::color::{Mix, Rgb};
use crate::mask::Mask;
use crate::shape::Shape;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hat {
    None,
    /// Altdeutsch beret: soft, flat, wider than the head.
    Beret,
    TopHat,
    Cap,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Coat {
    /// Greatcoat to mid-calf.
    Long,
    /// Jacket to the hips, legs fully visible.
    Short,
    /// Cape over the shoulders, to the knee.
    Cloak,
    /// Monk's habit, to the ground.
    Habit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct Figure {
    pub feet: (f32, f32),
    pub height: f32,
    /// Lean of the whole body in radians (+ = toward the right).
    pub lean: f32,
    /// Distance between the feet as a fraction of height.
    pub stance: f32,
    /// Shoulder breadth multiplier (1 = average man).
    pub build: f32,
    pub hat: Hat,
    pub coat: Coat,
    /// Absolute hand targets (units). None = arm hangs down.
    pub left_hand: Option<(f32, f32)>,
    pub right_hand: Option<(f32, f32)>,
    /// Walking stick held in this hand, planted on the ground.
    pub stick: Option<Side>,
}

impl Figure {
    pub fn new(feet: (f32, f32), height: f32) -> Self {
        Figure {
            feet,
            height,
            lean: 0.0,
            stance: 0.11,
            build: 1.0,
            hat: Hat::None,
            coat: Coat::Long,
            left_hand: None,
            right_hand: None,
            stick: None,
        }
    }

    /// Skeleton point: `u` across (fraction of height, + = viewer's right),
    /// `v` up from the feet (fraction of height). Applies the lean.
    pub fn at(&self, u: f32, v: f32) -> (f32, f32) {
        let (x, y) = (u * self.height, -v * self.height);
        let (s, c) = self.lean.sin_cos();
        // lean rotates about the feet; + lean tips the top to the right
        (self.feet.0 + x * c - y * s, self.feet.1 + x * s + y * c)
    }

    pub fn shoulder(&self, side: Side) -> (f32, f32) {
        let b = 0.115 * self.build;
        match side {
            Side::Left => self.at(-b, 0.815),
            Side::Right => self.at(b, 0.815),
        }
    }

    pub fn head_top(&self) -> (f32, f32) {
        self.at(0.0, 1.0)
    }

    fn hand(&self, side: Side) -> (f32, f32) {
        let target = match side {
            Side::Left => self.left_hand,
            Side::Right => self.right_hand,
        };
        target.unwrap_or_else(|| {
            let s = if side == Side::Left { -1.0 } else { 1.0 };
            if self.stick == Some(side) {
                self.at(s * 0.2, 0.47)
            } else {
                self.at(s * 0.13 * self.build, 0.45)
            }
        })
    }

    /// Two-bone IK: returns (shoulder, elbow, hand).
    fn arm(&self, side: Side) -> [(f32, f32); 3] {
        let s = self.shoulder(side);
        let t = self.hand(side);
        let (l1, l2) = (0.19 * self.height, 0.18 * self.height);
        let (dx, dy) = (t.0 - s.0, t.1 - s.1);
        let d = (dx * dx + dy * dy).sqrt().clamp(1e-3, (l1 + l2) * 0.999);
        let th = dy.atan2(dx);
        let alpha = ((l1 * l1 + d * d - l2 * l2) / (2.0 * l1 * d)).clamp(-1.0, 1.0).acos();
        // bend the elbow outward, away from the body
        let out = if side == Side::Left { 1.0 } else { -1.0 };
        let a = th + out * alpha;
        let e = (s.0 + l1 * a.cos(), s.1 + l1 * a.sin());
        let h = (s.0 + dx / (dx * dx + dy * dy).sqrt().max(1e-3) * d, s.1 + dy / (dx * dx + dy * dy).sqrt().max(1e-3) * d);
        [s, e, h]
    }

    pub fn shape(&self) -> Shape {
        let hgt = self.height;
        let b = self.build;
        let mut sh = Shape::new();

        // legs (hip → knee → foot), drawn first; the coat covers the top
        let st = self.stance * 0.5;
        for s in [-1.0f32, 1.0] {
            let pts = [self.at(s * 0.06, 0.50), self.at(s * (0.045 + st * 0.5), 0.26), self.at(s * st, 0.02)];
            sh = sh.ribbon(&pts, &[0.085 * hgt, 0.06 * hgt, 0.048 * hgt]);
            // boot
            let f = self.at(s * st + s * 0.01, 0.012);
            sh = sh.add(Shape::new().ellipse(f.0, f.1, 0.042 * hgt, 0.018 * hgt));
        }

        // torso + coat as a closed smooth outline
        let (hem, hem_w, waist_w) = match self.coat {
            Coat::Long => (0.2, 0.14, 0.095),
            Coat::Short => (0.45, 0.11, 0.095),
            Coat::Cloak => (0.3, 0.19, 0.16),
            Coat::Habit => (0.0, 0.13, 0.1),
        };
        let sw = 0.118 * b;
        let torso = [
            self.at(-0.04, 0.875),
            self.at(0.04, 0.875),
            self.at(sw * 0.8, 0.845),
            self.at(sw, 0.80),
            self.at(waist_w * b + 0.005, 0.62),
            self.at(waist_w * b, 0.52),
            self.at((waist_w * b + hem_w) * 0.5 + 0.01, (0.52 + hem) * 0.5),
            self.at(hem_w * b, hem),
            self.at(0.0, hem - 0.008),
            self.at(-hem_w * b, hem),
            self.at(-(waist_w * b + hem_w) * 0.5 - 0.01, (0.52 + hem) * 0.5),
            self.at(-waist_w * b, 0.52),
            self.at(-waist_w * b - 0.005, 0.62),
            self.at(-sw, 0.80),
            self.at(-sw * 0.8, 0.845),
        ];
        sh = sh.add(Shape::new().smooth_poly(&torso));

        // arms (cloaks hide them unless raised)
        for side in [Side::Left, Side::Right] {
            let raised = match side {
                Side::Left => self.left_hand.is_some(),
                Side::Right => self.right_hand.is_some(),
            } || self.stick == Some(side);
            if self.coat == Coat::Cloak && !raised {
                continue;
            }
            let [s, e, h] = self.arm(side);
            sh = sh.ribbon(&[s, e, h], &[0.075 * hgt, 0.058 * hgt, 0.042 * hgt]);
            sh = sh.add(Shape::new().ellipse(h.0, h.1, 0.024 * hgt, 0.026 * hgt));
            if self.stick == Some(side) {
                let g = (h.0 + 0.03 * hgt * if side == Side::Left { -1.0 } else { 1.0 }, self.feet.1 + 0.005 * hgt);
                sh = sh.ribbon(&[(h.0, h.1 - 0.05 * hgt), g], &[0.016 * hgt, 0.013 * hgt]);
            }
        }

        // neck, head
        let n = self.at(0.0, 0.87);
        sh = sh.ribbon(&[n, self.at(0.0, 0.92)], &[0.05 * hgt, 0.05 * hgt]);
        let hc = self.at(0.0, 0.935);
        sh = sh.add(Shape::new().ellipse(hc.0, hc.1, 0.052 * hgt, 0.062 * hgt));
        // collar
        sh = sh.add(Shape::new().smooth_poly(&[
            self.at(-0.07 * b, 0.86),
            self.at(0.0, 0.895),
            self.at(0.07 * b, 0.86),
            self.at(0.05 * b, 0.83),
            self.at(-0.05 * b, 0.83),
        ]));

        // hat
        match self.hat {
            Hat::None => {}
            Hat::Beret => {
                let c = self.at(0.012, 0.99);
                sh = sh.add(Shape::new().ellipse(c.0, c.1, 0.085 * hgt, 0.03 * hgt));
            }
            Hat::TopHat => {
                sh = sh.add(Shape::new().poly(&[
                    self.at(-0.05, 0.98),
                    self.at(-0.047, 1.1),
                    self.at(0.047, 1.1),
                    self.at(0.05, 0.98),
                ]));
                let c = self.at(0.0, 0.98);
                sh = sh.add(Shape::new().ellipse(c.0, c.1, 0.085 * hgt, 0.012 * hgt));
            }
            Hat::Cap => {
                let c = self.at(0.0, 0.985);
                sh = sh.add(Shape::new().ellipse(c.0, c.1, 0.06 * hgt, 0.03 * hgt));
            }
        }
        sh
    }

    /// Paint the figure: a dark silhouette with a faint rim of light on the
    /// `light_dir` side (−1 = light from the left, +1 = from the right) and a
    /// few coat folds. Returns the figure's mask.
    pub fn paint(&self, c: &mut Canvas, coat: Rgb, rim: Option<(Rgb, f32)>, seed: u64) -> Mask {
        let m = Mask::from_shape(c.f, self.shape());
        c.fill(&m, coat, 1.0, Mix::Linear);
        let hgt = self.height;
        // folds: a few long vertical strokes, slightly darker/lighter
        let mut rng = crate::rng::Rng::new(seed);
        for i in 0..7 {
            let u = rng.range(-0.1, 0.1);
            let top = rng.range(0.5, 0.75);
            let pts = [self.at(u, top), self.at(u * 1.2, (top + 0.2) * 0.5), self.at(u * 1.35, 0.22)];
            let k = if rng.chance(0.5) { 0.7 } else { 1.35 };
            let col = [coat[0] * k, coat[1] * k, coat[2] * k];
            let b = Brush::default().width(0.02 * hgt).soft(0.8).streak(0.3, 4.0).impasto(0.4);
            c.stroke_clipped(&b, Medium::Body(col, 0.5), &pts, seed + i, Some(&m));
        }
        if let Some((rc, dir)) = rim {
            // rim light: the mask minus itself shifted away from the light
            let shift = 0.012 * hgt * c.f.scale;
            let (w, h) = (c.f.w, c.f.h);
            let dx = (-dir * shift).round() as isize;
            let mut edge = Mask::empty(c.f);
            for y in 0..h {
                for x in 0..w {
                    let sx = x as isize + dx;
                    let other = if sx >= 0 && (sx as usize) < w { m.data[y * w + sx as usize] } else { 0.0 };
                    edge.data[y * w + x] = (m.data[y * w + x] - other).max(0.0);
                }
            }
            let edge = edge.blur(0.004 * hgt);
            let edge = edge.mul(&m);
            c.fill(&edge, rc, 0.55, Mix::Linear);
        }
        m
    }
}
