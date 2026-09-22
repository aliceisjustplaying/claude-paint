//! Bristle brush strokes.
//!
//! A stroke is a smooth path (Catmull-Rom through the given points). The brush
//! is stamped along it; each stamp knows its across-stroke coordinate, so each
//! bristle leaves a continuous streak. Paint load runs down along the stroke,
//! and weaker bristles give out first (dry-brush breakup). Ideas borrowed from
//! libmypaint's dab model (https://github.com/mypaint/libmypaint), with a
//! bristle profile added.

use crate::canvas::Canvas;
use crate::color::{self, Mix, Rgb};
use crate::pigment::Pigment;
use crate::rng::Rng;
use crate::smoothstep;

#[derive(Clone, Debug)]
pub struct Brush {
    /// Width in units.
    pub width: f32,
    /// Number of bristle streaks across the width.
    pub bristles: f32,
    /// 0 = perfectly even stroke, 1 = strongly streaked.
    pub streak: f32,
    /// Edge falloff as a fraction of the radius (0 = hard edge, 1 = airbrush).
    pub softness: f32,
    /// Paint load at the start (0..1). 1 = fully covered, lower = dry brush.
    pub load: f32,
    /// How much load is lost by the end of the stroke (0..1).
    pub decay: f32,
    /// Taper lengths at start/end as fractions of stroke length.
    pub taper: (f32, f32),
}

impl Default for Brush {
    fn default() -> Self {
        Brush {
            width: 10.0,
            bristles: 24.0,
            streak: 0.3,
            softness: 0.3,
            load: 1.0,
            decay: 0.3,
            taper: (0.1, 0.15),
        }
    }
}

impl Brush {
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    pub fn soft(mut self, s: f32) -> Self {
        self.softness = s;
        self
    }
    pub fn streak(mut self, s: f32, bristles: f32) -> Self {
        self.streak = s;
        self.bristles = bristles;
        self
    }
    pub fn load(mut self, load: f32, decay: f32) -> Self {
        self.load = load;
        self.decay = decay;
        self
    }
    pub fn taper(mut self, a: f32, b: f32) -> Self {
        self.taper = (a, b);
        self
    }
}

/// What the brush deposits.
#[derive(Clone, Copy, Debug)]
pub enum Medium {
    /// Transparent/semi-transparent layer with this max thickness.
    Glaze(Pigment, f32),
    /// Body color mixed physically into what's there, with this opacity.
    Body(Rgb, f32),
    /// Optical veil (light scumble), with this opacity.
    Light(Rgb, f32),
}

impl Canvas {
    /// Paint one stroke through `pts` (units).
    pub fn stroke(&mut self, brush: &Brush, medium: Medium, pts: &[(f32, f32)], seed: u64) {
        if pts.len() < 2 {
            return;
        }
        let s = self.f.scale;
        let path = densify(&pts.iter().map(|&(x, y)| (x * s, y * s)).collect::<Vec<_>>());
        if path.len() < 2 {
            return;
        }
        // cumulative arclength
        let mut arc = vec![0.0f32; path.len()];
        for i in 1..path.len() {
            let (dx, dy) = (path[i].0 - path[i - 1].0, path[i].1 - path[i - 1].1);
            arc[i] = arc[i - 1] + (dx * dx + dy * dy).sqrt();
        }
        let total = arc[arc.len() - 1].max(1e-3);
        let r0 = (brush.width * s * 0.5).max(0.6);

        // bristle strength profile across the brush
        let mut rng = Rng::new(seed);
        let nb = brush.bristles.max(1.0).ceil() as usize + 2;
        let strengths: Vec<f32> = (0..nb).map(|_| rng.f()).collect();
        let bristle = |a: f32| -> f32 {
            // a in [-1, 1] → smooth interpolation between bristles
            let t = (a * 0.5 + 0.5) * (nb - 2) as f32;
            let i = (t.floor() as usize).min(nb - 2);
            let f = t - i as f32;
            let f = f * f * (3.0 - 2.0 * f);
            strengths[i] + (strengths[i + 1] - strengths[i]) * f
        };

        // bounding box
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for &(x, y) in &path {
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x);
            y1 = y1.max(y);
        }
        let pad = r0 + 2.0;
        let bx0 = ((x0 - pad).floor().max(0.0)) as usize;
        let by0 = ((y0 - pad).floor().max(0.0)) as usize;
        let bx1 = ((x1 + pad).ceil() as usize).min(self.f.w);
        let by1 = ((y1 + pad).ceil() as usize).min(self.f.h);
        if bx1 <= bx0 || by1 <= by0 {
            return;
        }
        let (bw, bh) = (bx1 - bx0, by1 - by0);
        let mut cov = vec![0.0f32; bw * bh];

        // stamp dabs
        let step = (r0 * 0.12).clamp(0.5, 6.0);
        let mut d = 0.0;
        let mut seg = 0;
        while d <= total {
            while seg + 1 < path.len() - 1 && arc[seg + 1] < d {
                seg += 1;
            }
            let seg_len = (arc[seg + 1] - arc[seg]).max(1e-6);
            let f = ((d - arc[seg]) / seg_len).clamp(0.0, 1.0);
            let (ax, ay) = path[seg];
            let (bx, by) = path[seg + 1];
            let (px, py) = (ax + (bx - ax) * f, ay + (by - ay) * f);
            let (tx, ty) = ((bx - ax) / seg_len, (by - ay) / seg_len);
            let (nx, ny) = (-ty, tx);

            let u = d / total;
            let taper = {
                let a = if brush.taper.0 > 0.0 { smoothstep(0.0, brush.taper.0, u) } else { 1.0 };
                let b = if brush.taper.1 > 0.0 { smoothstep(0.0, brush.taper.1, 1.0 - u) } else { 1.0 };
                (a * b).max(0.15)
            };
            let r = r0 * taper;
            let load = (brush.load * (1.0 - brush.decay * u)).clamp(0.0, 1.0);
            let dry_th = 1.0 - load;

            let ix0 = ((px - r - 1.0).floor() as isize).max(bx0 as isize);
            let iy0 = ((py - r - 1.0).floor() as isize).max(by0 as isize);
            let ix1 = ((px + r + 1.0).ceil() as isize).min(bx1 as isize - 1);
            let iy1 = ((py + r + 1.0).ceil() as isize).min(by1 as isize - 1);
            for iy in iy0..=iy1 {
                for ix in ix0..=ix1 {
                    let dx = ix as f32 + 0.5 - px;
                    let dy = iy as f32 + 0.5 - py;
                    let dist = (dx * dx + dy * dy).sqrt() / r;
                    if dist > 1.0 {
                        continue;
                    }
                    let across = ((dx * nx + dy * ny) / r).clamp(-1.0, 1.0);
                    let edge = 1.0 - smoothstep(1.0 - brush.softness.max(0.02), 1.0, dist);
                    let b = bristle(across);
                    let dry = smoothstep(dry_th - 0.12, dry_th + 0.12, b);
                    let streak = 1.0 - brush.streak * (1.0 - b);
                    let c = edge * dry * streak;
                    let k = (iy as usize - by0) * bw + (ix as usize - bx0);
                    if c > cov[k] {
                        cov[k] = c;
                    }
                }
            }
            d += step;
        }

        // deposit
        let w = self.f.w;
        for yy in 0..bh {
            for xx in 0..bw {
                let c = cov[yy * bw + xx];
                if c <= 0.0 {
                    continue;
                }
                let p = &mut self.px[(yy + by0) * w + xx + bx0];
                *p = match medium {
                    Medium::Glaze(pig, th) => pig.over(*p, th * c),
                    Medium::Body(col, op) => color::mix(*p, col, c * op, Mix::Pigment),
                    Medium::Light(col, op) => color::mix(*p, col, c * op, Mix::Linear),
                };
            }
        }
    }
}

/// Catmull-Rom resample so consecutive points are ~2px apart.
fn densify(p: &[(f32, f32)]) -> Vec<(f32, f32)> {
    let n = p.len();
    let mut out = Vec::new();
    for i in 0..n - 1 {
        let p0 = p[i.saturating_sub(1)];
        let p1 = p[i];
        let p2 = p[i + 1];
        let p3 = p[(i + 2).min(n - 1)];
        let len = ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt();
        let k = ((len / 2.0).ceil() as usize).max(1);
        for j in 0..k {
            let t = j as f32 / k as f32;
            let (t2, t3) = (t * t, t * t * t);
            let cr = |a: f32, b: f32, c: f32, d: f32| {
                0.5 * (2.0 * b + (-a + c) * t + (2.0 * a - 5.0 * b + 4.0 * c - d) * t2
                    + (-a + 3.0 * b - 3.0 * c + d) * t3)
            };
            out.push((cr(p0.0, p1.0, p2.0, p3.0), cr(p0.1, p1.1, p2.1, p3.1)));
        }
    }
    out.push(p[n - 1]);
    out
}
