//! A painter's hand working at one spot: a local frame (origin, scale, and
//! "up") so a small motif can be written as a list of brush gestures in its
//! own coordinates. It knows nothing about what is painted; figures and other
//! motifs are written stroke by stroke in the paintings themselves.

use crate::bristle::{Gesture, Held, Orient, Tool};
use crate::canvas::Canvas;
use crate::mask::Mask;
use crate::rng::Rng;
use crate::wet::Paint;

pub struct Hand {
    /// Origin in canvas units (e.g. between a figure's feet).
    pub at: (f32, f32),
    /// Canvas units per local unit (e.g. a figure's height).
    pub size: f32,
    /// Tremor: random offset of each control point, in local units.
    pub tremor: f32,
    pub rng: Rng,
}

/// One gesture in local coordinates: `u` across (+ right), `v` up.
pub struct Mark<'a> {
    pub pts: &'a [(f32, f32)],
    /// Pressure at start and end.
    pub pressure: (f32, f32),
    /// Attack and release fractions.
    pub ramps: (f32, f32),
}

impl Hand {
    pub fn new(at: (f32, f32), size: f32, seed: u64) -> Self {
        Hand { at, size, tremor: 0.004, rng: Rng::new(seed) }
    }

    /// Local (u, v) to canvas units.
    pub fn p(&self, u: f32, v: f32) -> (f32, f32) {
        (self.at.0 + u * self.size, self.at.1 - v * self.size)
    }

    /// Pick up a brush of `width` local units, loaded with `paint`.
    pub fn take(&mut self, tool: fn(f32) -> Tool, width: f32, paint: Paint, load: f32) -> Held {
        let mut h = Held::new(tool(width * self.size), self.rng.next_u64());
        h.load(paint, load);
        h
    }

    /// Drag the held brush through local points.
    pub fn mark(&mut self, c: &mut Canvas, held: &mut Held, m: Mark, clip: Option<&Mask>) {
        let t = self.tremor;
        let pts: Vec<(f32, f32)> = m
            .pts
            .iter()
            .map(|&(u, v)| {
                let (du, dv) = (self.rng.normal() * t, self.rng.normal() * t);
                self.p(u + du, v + dv)
            })
            .collect();
        let g = Gesture::new(pts).pressure(m.pressure.0, m.pressure.1).ramps(m.ramps.0, m.ramps.1).orient(Orient::Across);
        c.drag(held, &g, clip);
    }

    /// Shorthand: a stroke with given pressures and default ramps.
    pub fn line(&mut self, c: &mut Canvas, held: &mut Held, pts: &[(f32, f32)], p0: f32, p1: f32) {
        self.mark(c, held, Mark { pts, pressure: (p0, p1), ramps: (0.1, 0.25) }, None);
    }

    /// A dab: a very short press at (u, v) in direction `dir` (radians, 0 = right, up positive).
    pub fn dab(&mut self, c: &mut Canvas, held: &mut Held, u: f32, v: f32, len: f32, dir: f32, p: f32) {
        let (du, dv) = (dir.cos() * len * 0.5, dir.sin() * len * 0.5);
        self.mark(c, held, Mark { pts: &[(u - du, v - dv), (u + du, v + dv)], pressure: (p, p), ramps: (0.0, 0.3) }, None);
    }
}
