//! Float coverage masks the size of the canvas.

use crate::canvas::Frame;
use crate::noise::Fbm;
use crate::shape::Shape;
use rayon::prelude::*;

#[derive(Clone)]
pub struct Mask {
    pub f: Frame,
    pub data: Vec<f32>,
}

impl Mask {
    pub fn empty(f: Frame) -> Self {
        Mask { f, data: vec![0.0; f.w * f.h] }
    }

    pub fn full(f: Frame) -> Self {
        Mask { f, data: vec![1.0; f.w * f.h] }
    }

    /// Evaluate `g(x, y)` (units) at every pixel center.
    pub fn from_fn(f: Frame, g: impl Fn(f32, f32) -> f32 + Sync) -> Self {
        let mut data = vec![0.0; f.w * f.h];
        let inv = 1.0 / f.scale;
        data.par_chunks_mut(f.w).enumerate().for_each(|(y, row)| {
            let yu = (y as f32 + 0.5) * inv;
            for (x, v) in row.iter_mut().enumerate() {
                *v = g((x as f32 + 0.5) * inv, yu);
            }
        });
        Mask { f, data }
    }

    /// Antialiased fill of a shape.
    pub fn from_shape(f: Frame, shape: Shape) -> Self {
        let mut m = Mask::empty(f);
        let paths = shape.paths();
        if !paths.is_empty() {
            let mut tm = tiny_skia::Mask::new(f.w as u32, f.h as u32).unwrap();
            for path in &paths {
                tm.fill_path(
                    path,
                    tiny_skia::FillRule::Winding,
                    true,
                    tiny_skia::Transform::from_scale(f.scale, f.scale),
                );
            }
            for (d, s) in m.data.iter_mut().zip(tm.data()) {
                *d = *s as f32 / 255.0;
            }
        }
        m
    }

    /// Approximate Gaussian blur (3 box passes). `radius` in units.
    pub fn blur(mut self, radius: f32) -> Self {
        let r = (radius * self.f.scale / 1.7).round() as usize;
        if r == 0 {
            return self;
        }
        for _ in 0..3 {
            box_rows(&mut self.data, self.f.w, r);
            let mut t = transpose(&self.data, self.f.w, self.f.h);
            box_rows(&mut t, self.f.h, r);
            self.data = transpose(&t, self.f.h, self.f.w);
        }
        self
    }

    /// Break up the edge with noise: the 0.5 contour gets pushed around by
    /// `amount` (0..1) of fbm at the given period, then re-sharpened to `edge` softness.
    pub fn roughen(mut self, seed: u32, period: f32, amount: f32, edge: f32) -> Self {
        let n = Fbm::new(seed, 5, period);
        let inv = 1.0 / self.f.scale;
        let w = self.f.w;
        self.data.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            let yu = (y as f32 + 0.5) * inv;
            for (x, v) in row.iter_mut().enumerate() {
                let d = *v + n.get((x as f32 + 0.5) * inv, yu) * amount;
                *v = crate::smoothstep(0.5 - edge, 0.5 + edge, d);
            }
        });
        self
    }

    pub fn map(mut self, g: impl Fn(f32) -> f32 + Sync) -> Self {
        self.data.par_iter_mut().for_each(|v| *v = g(*v));
        self
    }

    /// Multiply by a function of position.
    pub fn mul_fn(mut self, g: impl Fn(f32, f32) -> f32 + Sync) -> Self {
        let inv = 1.0 / self.f.scale;
        let w = self.f.w;
        self.data.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            let yu = (y as f32 + 0.5) * inv;
            for (x, v) in row.iter_mut().enumerate() {
                *v *= g((x as f32 + 0.5) * inv, yu);
            }
        });
        self
    }

    pub fn mul(mut self, o: &Mask) -> Self {
        self.data.par_iter_mut().zip(&o.data).for_each(|(a, b)| *a *= b);
        self
    }

    pub fn union(mut self, o: &Mask) -> Self {
        self.data.par_iter_mut().zip(&o.data).for_each(|(a, b)| *a = a.max(*b));
        self
    }

    pub fn subtract(mut self, o: &Mask) -> Self {
        self.data.par_iter_mut().zip(&o.data).for_each(|(a, b)| *a *= 1.0 - b);
        self
    }

    pub fn invert(self) -> Self {
        self.map(|v| 1.0 - v)
    }

    #[inline]
    pub fn at(&self, i: usize) -> f32 {
        self.data[i]
    }
}

fn box_rows(data: &mut [f32], w: usize, r: usize) {
    data.par_chunks_mut(w).for_each(|row| {
        let src = row.to_vec();
        let mut acc = 0.0f32;
        let n = (2 * r + 1) as f32;
        // clamp-to-edge
        let get = |i: isize| src[i.clamp(0, w as isize - 1) as usize];
        for i in -(r as isize)..=(r as isize) {
            acc += get(i);
        }
        for x in 0..w {
            row[x] = acc / n;
            acc += get(x as isize + r as isize + 1) - get(x as isize - r as isize);
        }
    });
}

fn transpose(src: &[f32], w: usize, h: usize) -> Vec<f32> {
    let mut dst = vec![0.0; w * h];
    dst.par_chunks_mut(h).enumerate().for_each(|(x, col)| {
        for y in 0..h {
            col[y] = src[y * w + x];
        }
    });
    dst
}
