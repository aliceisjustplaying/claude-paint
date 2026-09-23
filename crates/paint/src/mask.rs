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

    /// Value at a point in units, bilinear between pixel centers (clamped at
    /// the canvas edge).
    pub fn sample(&self, x: f32, y: f32) -> f32 {
        let f = self.f;
        let (px, py) = (x * f.scale - 0.5, y * f.scale - 0.5);
        let (x0, y0) = (px.floor(), py.floor());
        let (tx, ty) = (px - x0, py - y0);
        let cx = |i: f32| (i as isize).clamp(0, f.w as isize - 1) as usize;
        let cy = |j: f32| (j as isize).clamp(0, f.h as isize - 1) as usize;
        let at = |i: f32, j: f32| self.data[cy(j) * f.w + cx(i)];
        let a = at(x0, y0) + (at(x0 + 1.0, y0) - at(x0, y0)) * tx;
        let b = at(x0, y0 + 1.0) + (at(x0 + 1.0, y0 + 1.0) - at(x0, y0 + 1.0)) * tx;
        a + (b - a) * ty
    }

    /// Signed distance to the region's edge (the 0.5 level), in units:
    /// positive inside, negative outside. The values are a field, not
    /// coverage; turn them back into a mask with `map` or `band`.
    pub fn distance(&self) -> Mask {
        let (d, _) = self.signed_distance_px();
        let inv = 1.0 / self.f.scale;
        Mask { f: self.f, data: d.into_iter().map(|v| v * inv).collect() }
    }

    /// Signed distance in pixels (+ inside), and for every pixel the index of
    /// the nearest pixel inside the region (itself when inside).
    pub(crate) fn signed_distance_px(&self) -> (Vec<f32>, Vec<u32>) {
        signed_distance_region(self.f, &self.data.par_iter().map(|&v| v >= 0.5).collect::<Vec<bool>>())
    }

    /// Grow the region by `d` units (shrink it if negative), with an
    /// antialiased edge.
    pub fn offset(&self, d: f32) -> Mask {
        let (sd, _) = self.signed_distance_px();
        let k = d * self.f.scale;
        Mask { f: self.f, data: sd.into_par_iter().map(|v| (v + k + 0.5).clamp(0.0, 1.0)).collect() }
    }

    /// Grow the region by `d` units.
    pub fn dilate(&self, d: f32) -> Mask {
        self.offset(d.abs())
    }

    /// Shrink the region by `d` units.
    pub fn erode(&self, d: f32) -> Mask {
        self.offset(-d.abs())
    }

    /// The inside of the region within `width` units of its edge (a rim to
    /// cut in or catch light along), fading out over `soft` units inward.
    pub fn rim(&self, width: f32, soft: f32) -> Mask {
        let (sd, _) = self.signed_distance_px();
        let (w0, s) = (width * self.f.scale, soft.max(1e-3) * self.f.scale);
        let data = sd
            .into_par_iter()
            .map(|v| (v + 0.5).clamp(0.0, 1.0) * (1.0 - crate::smoothstep(w0 - s * 0.5, w0 + s * 0.5, v)))
            .collect();
        Mask { f: self.f, data }
    }

    /// 1 where the value lies between `lo` and `hi`, ramping over `soft` on
    /// either side (a band between two levels of any field).
    pub fn band(self, lo: f32, hi: f32, soft: f32) -> Mask {
        let s = soft.max(1e-6);
        self.map(move |v| crate::smoothstep(lo - s, lo + s, v) * (1.0 - crate::smoothstep(hi - s, hi + s, v)))
    }

    /// Re-edge the region with a softness that varies over the canvas:
    /// `width(x, y)` units of ramp across the edge (0 = crisp), evaluated at
    /// the nearest point inside, so a silhouette can be hard in one place and
    /// lost in the next. The ramp is centered on the old edge.
    pub fn soften(&self, width: impl Fn(f32, f32) -> f32 + Sync) -> Mask {
        let inside: Vec<bool> = self.data.par_iter().map(|&v| v >= 0.5).collect();
        soften_region(self.f, inside, width)
    }
}

/// Signed distance in pixels (+ inside) to the edge of a region given pixel
/// by pixel, and each pixel's nearest pixel inside (u32::MAX if none).
/// Working memory beyond the region: four w·h buffers of 4 bytes (the two
/// results, the column pass and its nearest rows), shared by both passes.
pub(crate) fn signed_distance_region(f: Frame, inside: &[bool]) -> (Vec<f32>, Vec<u32>) {
    let (w, h) = (f.w, f.h);
    let n = w * h;
    let mut col = vec![0.0f32; n];
    let mut sd = vec![0.0f32; n];
    // squared distance from the inside to the nearest outside pixel
    edt_into(&|i| !inside[i], w, h, &mut col, None, &mut sd, None, &|_, d2, _| d2);
    // then from the outside to the nearest inside pixel, combined in place
    let mut col_arg = vec![0u32; n];
    let mut near = vec![0u32; n];
    edt_into(&|i| inside[i], w, h, &mut col, Some(&mut col_arg), &mut sd, Some(&mut near), &|i, d_in, d_out| {
        if inside[i] { d_out.sqrt() - 0.5 } else { 0.5 - d_in.sqrt() }
    });
    (sd, near)
}

/// `Mask::soften` for a region given pixel by pixel.
pub(crate) fn soften_region(f: Frame, inside: Vec<bool>, width: impl Fn(f32, f32) -> f32 + Sync) -> Mask {
    let (mut sd, near) = signed_distance_region(f, &inside);
    drop(inside);
    let inv = 1.0 / f.scale;
    // the mask replaces the distances in place
    sd.par_iter_mut().zip(near.par_iter()).for_each(|(v, &j)| {
        *v = if j == u32::MAX {
            0.0
        } else {
            let j = j as usize;
            let (x, y) = (((j % f.w) as f32 + 0.5) * inv, ((j / f.w) as f32 + 0.5) * inv);
            let s = (width(x, y) * f.scale).max(1.0);
            crate::smoothstep(-0.5 * s, 0.5 * s, *v)
        };
    });
    Mask { f, data: sd }
}

/// Exact squared Euclidean distance transform (Felzenszwalb & Huttenlocher,
/// "Distance Transforms of Sampled Functions", 2012), in pixels², to the
/// nearest pixel where `seed(i)`. For every pixel `i`, `d[i]` becomes
/// `combine(i, d², d[i])` (d² is f32::MAX if there is no seed), and if
/// `idx` is given it gets the nearest seed's index (u32::MAX if none).
///
/// Buffers are the caller's so passes can share them: `col` (w·h) holds the
/// column pass, `col_arg` (w·h) its nearest rows, needed only with `idx`.
/// Column distances are whole pixels², exact in f32 up to 4096² px frames.
#[allow(clippy::too_many_arguments)]
pub(crate) fn edt_into(
    seed: &(dyn Fn(usize) -> bool + Sync),
    w: usize,
    h: usize,
    col: &mut [f32],
    mut col_arg: Option<&mut [u32]>,
    d: &mut [f32],
    idx: Option<&mut [u32]>,
    combine: &(dyn Fn(usize, f32, f32) -> f32 + Sync),
) {
    const BIG: f64 = 1e20;
    // columns first (on the transposed grid so rows are contiguous)
    let column = |x: usize, c: &mut [f32], a: Option<&mut [u32]>| {
        let f: Vec<f64> = (0..h).map(|y| if seed(y * w + x) { 0.0 } else { BIG }).collect();
        let mut dd = vec![0.0f64; h];
        let mut arg = vec![0u32; h];
        edt1(&f, &mut dd, &mut arg);
        for y in 0..h {
            c[y] = dd[y] as f32;
        }
        if let Some(a) = a {
            a.copy_from_slice(&arg);
        }
    };
    match col_arg.as_deref_mut() {
        Some(ca) => col.par_chunks_mut(h).zip(ca.par_chunks_mut(h)).enumerate().for_each(|(x, (c, a))| column(x, c, Some(a))),
        None => col.par_chunks_mut(h).enumerate().for_each(|(x, c)| column(x, c, None)),
    }
    let (col, ca): (&[f32], Option<&[u32]>) = (col, col_arg.as_deref());
    // then rows over the column results
    let row = |y: usize, dr: &mut [f32], ir: Option<&mut [u32]>| {
        let f: Vec<f64> = (0..w).map(|x| col[x * h + y] as f64).collect();
        let mut dd = vec![0.0f64; w];
        let mut arg = vec![0u32; w];
        edt1(&f, &mut dd, &mut arg);
        let empty = |x: usize| dd[x] >= BIG * 0.5;
        for x in 0..w {
            let i = y * w + x;
            dr[x] = combine(i, if empty(x) { f32::MAX } else { dd[x] as f32 }, dr[x]);
        }
        if let (Some(ir), Some(ca)) = (ir, ca) {
            for x in 0..w {
                let qx = arg[x] as usize;
                ir[x] = if empty(x) { u32::MAX } else { (ca[qx * h + y] as usize * w + qx) as u32 };
            }
        }
    };
    match idx {
        Some(ix) => d.par_chunks_mut(w).zip(ix.par_chunks_mut(w)).enumerate().for_each(|(y, (dr, ir))| row(y, dr, Some(ir))),
        None => d.par_chunks_mut(w).enumerate().for_each(|(y, dr)| row(y, dr, None)),
    }
}

/// 1-D squared distance transform of `f` (lower envelope of parabolas);
/// `arg` gets the position of the minimizing sample.
fn edt1(f: &[f64], d: &mut [f64], arg: &mut [u32]) {
    let n = f.len();
    let mut v = vec![0usize; n];
    let mut z = vec![0.0f64; n + 1];
    let mut k = 0usize;
    z[0] = f64::NEG_INFINITY;
    z[1] = f64::INFINITY;
    let inter = |q: usize, p: usize| ((f[q] + (q * q) as f64) - (f[p] + (p * p) as f64)) / (2.0 * (q as f64 - p as f64));
    for q in 1..n {
        let mut s = inter(q, v[k]);
        // z[0] = -inf, so this stops at k = 0
        while s <= z[k] {
            k -= 1;
            s = inter(q, v[k]);
        }
        k += 1;
        v[k] = q;
        z[k] = s;
        z[k + 1] = f64::INFINITY;
    }
    let mut k = 0;
    for q in 0..n {
        while z[k + 1] < q as f64 {
            k += 1;
        }
        let p = v[k];
        d[q] = (q as f64 - p as f64).powi(2) + f[p];
        arg[q] = p as u32;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn frame() -> Frame {
        Frame::new(400, 300, 0.4)
    }

    fn disk(r: f32) -> Mask {
        Mask::from_fn(frame(), move |x, y| if ((x - 500.0).powi(2) + (y - 375.0).powi(2)).sqrt() < r { 1.0 } else { 0.0 })
    }

    #[test]
    fn distance_is_euclidean_and_signed() {
        let d = disk(200.0).distance();
        assert!((d.sample(500.0, 375.0) - 200.0).abs() < 4.0, "center {}", d.sample(500.0, 375.0));
        assert!((d.sample(500.0 + 250.0, 375.0) + 50.0).abs() < 4.0, "outside {}", d.sample(750.0, 375.0));
        // diagonal: exact, not chessboard or city block
        let q = 500.0 + 150.0 * std::f32::consts::FRAC_1_SQRT_2;
        assert!((d.sample(q, 375.0 + 150.0 * std::f32::consts::FRAC_1_SQRT_2) - 50.0).abs() < 4.0);
    }

    /// The shared-buffer transform against brute force on a ragged region.
    #[test]
    fn signed_distance_matches_brute_force() {
        let f = Frame::new(37, 23, 1.0);
        let m = Mask::from_fn(f, |x, y| if ((x * 0.37).sin() + (y * 0.53).cos() + (x * y * 0.01).sin()) > 0.6 { 1.0 } else { 0.0 });
        let (sd, near) = m.signed_distance_px();
        let inside: Vec<bool> = m.data.iter().map(|&v| v >= 0.5).collect();
        assert!(inside.iter().any(|&b| b) && inside.iter().any(|&b| !b));
        for i in 0..f.w * f.h {
            let (x, y) = ((i % f.w) as f32, (i / f.w) as f32);
            let nearest = |want: bool| {
                (0..f.w * f.h).filter(|&j| inside[j] == want).map(|j| ((j % f.w) as f32 - x).powi(2) + ((j / f.w) as f32 - y).powi(2)).fold(f32::MAX, f32::min)
            };
            let want = if inside[i] { nearest(false).sqrt() - 0.5 } else { 0.5 - nearest(true).sqrt() };
            assert!((sd[i] - want).abs() < 1e-4, "{i}: {} vs {want}", sd[i]);
            let j = near[i] as usize;
            assert!(inside[j]);
            let dj = ((j % f.w) as f32 - x).powi(2) + ((j / f.w) as f32 - y).powi(2);
            assert!((dj - nearest(true)).abs() < 1e-3);
        }
    }

    #[test]
    fn offset_grows_and_shrinks() {
        let area = |m: &Mask| m.data.iter().sum::<f32>() / (m.f.scale * m.f.scale);
        let m = disk(150.0);
        let a0 = area(&m);
        let grown = area(&m.dilate(30.0));
        let shrunk = area(&m.erode(30.0));
        let pi = std::f32::consts::PI;
        assert!((a0 - pi * 150.0f32.powi(2)).abs() / a0 < 0.03);
        assert!((grown - pi * 180.0f32.powi(2)).abs() / grown < 0.03, "{grown}");
        assert!((shrunk - pi * 120.0f32.powi(2)).abs() / shrunk < 0.03, "{shrunk}");
    }

    #[test]
    fn rim_band_and_soften() {
        let m = disk(200.0);
        let r = m.rim(20.0, 2.0);
        assert!(r.sample(500.0 + 190.0, 375.0) > 0.9);
        assert!(r.sample(500.0 + 150.0, 375.0) < 0.05);
        assert!(r.sample(500.0 + 230.0, 375.0) < 0.05);
        let b = Mask::from_fn(frame(), |x, _| x / 1000.0).band(0.3, 0.6, 0.01);
        assert!(b.sample(450.0, 10.0) > 0.99 && b.sample(200.0, 10.0) < 0.01 && b.sample(700.0, 10.0) < 0.01);
        // soft on the right half only
        let s = m.soften(|x, _| if x > 500.0 { 60.0 } else { 0.0 });
        let right = s.sample(500.0 + 200.0 + 15.0, 375.0);
        let left = s.sample(500.0 - 200.0 - 15.0, 375.0);
        assert!(right > 0.1 && right < 0.5, "{right}");
        assert!(left < 0.01, "{left}");
    }
}
