//! Cover a region with real brush strokes instead of a flat fill.
//!
//! Strokes are placed on a jittered grid over the mask, shuffled, then traced
//! along a direction field. Each stroke takes its color from `color(x, y)` at
//! its center plus a small random shift in lightness and hue, like paint
//! mixed a little differently on every dip of the brush.

use crate::brush::{Brush, Medium};
use crate::canvas::Canvas;
use crate::color::{Rgb, from_oklab, to_oklab};
use crate::mask::Mask;
use crate::rng::Rng;

type Field<'a, T> = Box<dyn Fn(f32, f32) -> T + Sync + 'a>;

pub struct StrokeFill<'a> {
    pub brush: Brush,
    /// Stroke length range in units.
    pub length: (f32, f32),
    /// Center spacing as a fraction of brush width (smaller = more overlap).
    pub spacing: f32,
    /// Stroke direction in radians at a point (0 = left→right).
    pub angle: Field<'a, f32>,
    /// Paint color at a point.
    pub color: Field<'a, Rgb>,
    /// Per-stroke random lightness shift (OKLab L), e.g. 0.02.
    pub jitter: f32,
    /// Per-stroke random hue shift (OKLab a/b), e.g. 0.01.
    pub hue_jitter: f32,
    /// Random deviation of the angle, radians.
    pub angle_jitter: f32,
    /// Body opacity.
    pub opacity: f32,
    /// Use `Medium::Light` (optical) instead of pigment mixing.
    pub light: bool,
    /// Dry blending brush: deposits nothing, drags wet paint (softens).
    pub blend: bool,
    /// Minimum mask value at the stroke center for a stroke to be placed.
    pub threshold: f32,
}

impl<'a> StrokeFill<'a> {
    pub fn new(brush: Brush) -> Self {
        StrokeFill {
            brush,
            length: (20.0, 60.0),
            spacing: 0.55,
            angle: Box::new(|_, _| 0.0),
            color: Box::new(|_, _| [0.5; 3]),
            jitter: 0.02,
            hue_jitter: 0.006,
            angle_jitter: 0.08,
            opacity: 0.85,
            light: false,
            blend: false,
            threshold: 0.3,
        }
    }
    pub fn length(mut self, a: f32, b: f32) -> Self {
        self.length = (a, b);
        self
    }
    pub fn spacing(mut self, s: f32) -> Self {
        self.spacing = s;
        self
    }
    pub fn angle(mut self, f: impl Fn(f32, f32) -> f32 + Sync + 'a) -> Self {
        self.angle = Box::new(f);
        self
    }
    pub fn color(mut self, f: impl Fn(f32, f32) -> Rgb + Sync + 'a) -> Self {
        self.color = Box::new(f);
        self
    }
    pub fn jitter(mut self, l: f32, hue: f32) -> Self {
        self.jitter = l;
        self.hue_jitter = hue;
        self
    }
    pub fn angle_jitter(mut self, a: f32) -> Self {
        self.angle_jitter = a;
        self
    }
    pub fn opacity(mut self, o: f32) -> Self {
        self.opacity = o;
        self
    }
    pub fn light(mut self, on: bool) -> Self {
        self.light = on;
        self
    }
    pub fn blend(mut self, on: bool) -> Self {
        self.blend = on;
        self
    }
    pub fn threshold(mut self, t: f32) -> Self {
        self.threshold = t;
        self
    }
}

impl Canvas {
    /// Paint the region `mask` with strokes. Coverage is clipped by the mask,
    /// so give it a roughened/soft mask for painterly edges.
    pub fn fill_strokes(&mut self, mask: &Mask, sf: &StrokeFill, seed: u64) {
        self.check_mask(mask);
        let mut rng = Rng::new(seed);
        let f = self.f;
        let gap = (sf.brush.width * sf.spacing).max(0.5);
        let (cols, rows) = ((f.width() / gap).ceil() as usize + 1, (f.height() / gap).ceil() as usize + 1);

        // candidate centers inside the mask
        let mut centers = Vec::new();
        for j in 0..rows {
            for i in 0..cols {
                let x = (i as f32 + rng.f()) * gap;
                let y = (j as f32 + rng.f()) * gap;
                if x > f.width() || y > f.height() {
                    continue;
                }
                if mask.data[f.index(x, y)] >= sf.threshold {
                    centers.push((x, y));
                }
            }
        }
        // shuffle so strokes don't stack in scanline order
        for i in (1..centers.len()).rev() {
            let j = (rng.next_u64() % (i as u64 + 1)) as usize;
            centers.swap(i, j);
        }

        for (n, &(cx, cy)) in centers.iter().enumerate() {
            let len = rng.range(sf.length.0, sf.length.1);
            let bend = rng.normal() * sf.angle_jitter;
            // trace a streamline both ways from the center
            let steps = 3;
            let h = len / (2 * steps) as f32;
            let mut fwd = vec![(cx, cy)];
            let mut back = vec![];
            let (mut x, mut y) = (cx, cy);
            for k in 0..steps {
                let a = (sf.angle)(x, y) + bend * (1.0 + k as f32 * 0.3);
                x += a.cos() * h;
                y += a.sin() * h;
                fwd.push((x, y));
            }
            let (mut x, mut y) = (cx, cy);
            for k in 0..steps {
                let a = (sf.angle)(x, y) + bend * (1.0 + k as f32 * 0.3);
                x -= a.cos() * h;
                y -= a.sin() * h;
                back.push((x, y));
            }
            back.reverse();
            back.extend(fwd);
            // stroke direction is random so starts/ends don't all line up
            if rng.chance(0.5) {
                back.reverse();
            }

            let base = (sf.color)(cx, cy);
            let lab = to_oklab(base);
            let c = from_oklab([
                lab[0] + rng.normal() * sf.jitter,
                lab[1] + rng.normal() * sf.hue_jitter,
                lab[2] + rng.normal() * sf.hue_jitter,
            ]);
            let mut b = sf.brush.clone();
            b.width *= rng.range(0.8, 1.15);
            let op = sf.opacity * rng.range(0.85, 1.0);
            let medium = if sf.blend {
                Medium::Blend(op)
            } else if sf.light {
                Medium::Light(c, op)
            } else {
                Medium::Body(c, op)
            };
            self.stroke_clipped(&b, medium, &back, seed.wrapping_mul(7919).wrapping_add(n as u64), Some(mask));
        }
    }
}
