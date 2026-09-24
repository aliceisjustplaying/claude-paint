//! Measuring for the studies: a finished canvas read in OKLab, sampled in
//! canvas units (1000 across), and the few helpers that lay in their
//! underlayers.

use paint::color::to_oklab;
use paint::{Canvas, Mask, Rgb, Style};

/// Canvas units to mm at the canvas's 440 mm width.
pub const MM: f32 = 440.0 / 1000.0;

/// A mask covering the open box (x0, y0)–(x1, y1).
pub fn rect(c: &Canvas, x0: f32, y0: f32, x1: f32, y1: f32) -> Mask {
    Mask::from_fn(c.frame(), move |x, y| if x > x0 && x < x1 && y > y0 && y < y1 { 1.0 } else { 0.0 })
}

/// Cover the mask with `col` the way a painter lays in a passage (the
/// style's body handling, gaps filled).
pub fn field(c: &mut Canvas, st: &Style, col: Rgb, m: &Mask, medium: f32, seed: u64) {
    c.work(m, &st.body().color(move |_, _| col).medium(medium).angle(|_, _| 0.1).clip(true), seed);
}

/// Color difference in OKLab.
pub fn de(a: Rgb, b: Rgb) -> f32 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

/// A canvas's pixels in OKLab.
pub struct Img {
    pub lab: Vec<Rgb>,
    pub w: usize,
    pub h: usize,
    /// Pixels per unit.
    pub s: f32,
}

impl Img {
    /// `px` laid out in `c`'s window (its `pixels()` or `seen()`).
    pub fn new(c: &Canvas, px: &[Rgb]) -> Self {
        let f = c.window();
        Img { lab: px.iter().map(|&p| to_oklab(p)).collect(), w: f.w, h: f.h, s: f.w as f32 / 1000.0 }
    }
    /// Lightness of the pixel at (x, y).
    pub fn px(&self, x: f32, y: f32) -> f32 {
        self.lab[(y * self.s) as usize * self.w + (x * self.s) as usize][0]
    }
    /// Mean OKLab over a disc of radius `r` units (at least half a pixel),
    /// the part of it on the canvas.
    pub fn at(&self, x: f32, y: f32, r: f32) -> Rgb {
        let (cx, cy, rp) = (x * self.s, y * self.s, (r * self.s).max(0.5));
        let (mut a, mut n) = ([0.0f32; 3], 0.0);
        for j in (cy - rp).floor() as i64..=(cy + rp).ceil() as i64 {
            for i in (cx - rp).floor() as i64..=(cx + rp).ceil() as i64 {
                let (dx, dy) = (i as f32 + 0.5 - cx, j as f32 + 0.5 - cy);
                if dx * dx + dy * dy <= rp * rp && i >= 0 && j >= 0 && (i as usize) < self.w && (j as usize) < self.h {
                    let p = self.lab[j as usize * self.w + i as usize];
                    for k in 0..3 {
                        a[k] += p[k];
                    }
                    n += 1.0;
                }
            }
        }
        [a[0] / n, a[1] / n, a[2] / n]
    }
    /// Mean lightness over a disc (see `at`).
    pub fn l(&self, x: f32, y: f32, r: f32) -> f32 {
        self.at(x, y, r)[0]
    }
    /// Lightness along a vertical line from `y0` to `y1` at `x`, one sample
    /// per pixel row, averaged over ±`band` units in x.
    pub fn column(&self, x: f32, y0: f32, y1: f32, band: f32) -> Vec<f32> {
        let n = ((y1 - y0) * self.s) as usize;
        let (a, b) = (((x - band) * self.s) as usize, ((x + band) * self.s) as usize);
        (0..n)
            .map(|k| {
                let y = ((y0 * self.s) as usize + k) * self.w;
                (a..=b).map(|i| self.lab[y + i][0]).sum::<f32>() / (b - a + 1) as f32
            })
            .collect()
    }
    /// Lightness along a line from `a` to `b`, one sample per pixel,
    /// averaged across ±`band` units.
    pub fn line(&self, a: (f32, f32), b: (f32, f32), band: f32) -> Vec<f32> {
        let units = (b.0 - a.0).hypot(b.1 - a.1);
        let len = (units * self.s) as usize;
        // the step along the line (one pixel) and the unit normal across it:
        // the band is ±`band` canvas units at any resolution (the offset used
        // to scale by the step, which shrank the band as `s` grew)
        let (dx, dy) = ((b.0 - a.0) / len as f32, (b.1 - a.1) / len as f32);
        let (nx, ny) = (-(b.1 - a.1) / units, (b.0 - a.0) / units);
        let nb = ((band * self.s) as i32).max(0);
        (0..len)
            .map(|k| {
                let (x, y) = (a.0 + dx * k as f32, a.1 + dy * k as f32);
                (-nb..=nb).map(|j| j as f32 / self.s).map(|t| self.px(x + nx * t, y + ny * t)).sum::<f32>() / (2 * nb + 1) as f32
            })
            .collect()
    }
    /// The pixels of the box (x0, y0, x1, y1).
    fn pixels_in(&self, b: (f32, f32, f32, f32)) -> impl Iterator<Item = (usize, usize)> {
        let (a, bb, cc, d) = ((b.0 * self.s) as usize, (b.1 * self.s) as usize, (b.2 * self.s) as usize, (b.3 * self.s) as usize);
        (bb..d).flat_map(move |y| (a..cc).map(move |x| (x, y)))
    }
    /// Share of the box within ΔE 0.06 of the ground's color `g`.
    pub fn ground(&self, g: Rgb, b: (f32, f32, f32, f32)) -> f32 {
        let (mut n, mut k) = (0.0f32, 0.0f32);
        for (x, y) in self.pixels_in(b) {
            let p = self.lab[y * self.w + x];
            n += 1.0;
            if (p[0] - g[0]).hypot(p[1] - g[1]).hypot(p[2] - g[2]) < 0.06 {
                k += 1.0;
            }
        }
        k / n.max(1.0)
    }
    /// Mean |L − box-blurred L| over the box (blur radius 2 units), ×1000.
    pub fn texture(&self, b: (f32, f32, f32, f32)) -> f32 {
        let r = ((2.0 * self.s) as i64).max(1);
        let (mut n, mut acc) = (0.0f32, 0.0f32);
        for (x, y) in self.pixels_in(b) {
            let (mut m, mut q) = (0.0f32, 0.0f32);
            for j in -r..=r {
                for i in -r..=r {
                    m += self.lab[(y as i64 + j) as usize * self.w + (x as i64 + i) as usize][0];
                    q += 1.0;
                }
            }
            acc += (self.lab[y * self.w + x][0] - m / q).abs();
            n += 1.0;
        }
        1000.0 * acc / n
    }
}

/// 10–90% width (units) of the transition in a profile from level `hi` (at
/// its start) to level `lo` (at its end): from the 50% crossing nearest the
/// middle, out to where it reaches 90% before and 10% after. None without a
/// crossing; a side that never reaches its level is None too, or with
/// `to_ends` counts to the profile's end.
pub fn width(p: &[f32], hi: f32, lo: f32, s: f32, to_ends: bool) -> Option<f32> {
    let n: Vec<f32> = p.iter().map(|&v| (v - lo) / (hi - lo)).collect();
    let mid = n.len() as f32 / 2.0;
    let c = (1..n.len()).filter(|&i| (n[i - 1] - 0.5) * (n[i] - 0.5) <= 0.0).min_by(|&a, &b| (a as f32 - mid).abs().total_cmp(&(b as f32 - mid).abs()))?;
    let a = (0..c).rev().find(|&i| n[i] >= 0.9).or(to_ends.then_some(0))?;
    let b = (c..n.len()).find(|&i| n[i] <= 0.1).or(to_ends.then_some(n.len() - 1))?;
    Some((b - a) as f32 / s)
}

/// The median, NaN if empty.
pub fn median(mut v: Vec<f32>) -> f32 {
    if v.is_empty() {
        return f32::NAN;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    v[v.len() / 2]
}
