//! Bristle brush strokes.
//!
//! A stroke is a smooth path (Catmull-Rom through the given points). The brush
//! is stamped along it; each stamp knows its across-stroke coordinate, so each
//! bristle leaves a continuous streak. Paint load runs down along the stroke,
//! and weaker bristles give out first (dry-brush breakup). The brush can pick
//! up wet paint from the canvas and drag it along (smudge), and it leaves a
//! ridged relief. Ideas borrowed from libmypaint's dab model
//! (https://github.com/mypaint/libmypaint), with a bristle profile added.

use crate::canvas::Canvas;
use crate::color::{self, Mix, Rgb};
use crate::mask::Mask;
use crate::path::densify;
use crate::pigment::Pigment;
use crate::rng::Rng;
use crate::smoothstep;
use rayon::prelude::*;

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
    /// Wet pickup: fraction of the brush color replaced by the canvas color
    /// per 10 units traveled. 0 = clean brush, 0.5 = heavy smudge.
    pub pickup: f32,
    /// Paint relief: ridge height laid down (0 = flat glaze, 1 = impasto).
    pub impasto: f32,
    /// Random wobble of the path, in units.
    pub wobble: f32,
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
            pickup: 0.0,
            impasto: 0.15,
            wobble: 0.0,
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
    pub fn pickup(mut self, p: f32) -> Self {
        self.pickup = p;
        self
    }
    pub fn impasto(mut self, i: f32) -> Self {
        self.impasto = i;
        self
    }
    pub fn wobble(mut self, w: f32) -> Self {
        self.wobble = w;
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
    /// No new paint: only drags what's there (a dry blending brush).
    Blend(f32),
}

impl Canvas {
    /// Paint one stroke through `pts` (units).
    pub fn stroke(&mut self, brush: &Brush, medium: Medium, pts: &[(f32, f32)], seed: u64) {
        self.stroke_clipped(brush, medium, pts, seed, None);
    }

    /// Paint one stroke, with coverage multiplied by `clip` (if given).
    pub fn stroke_clipped(
        &mut self,
        brush: &Brush,
        medium: Medium,
        pts: &[(f32, f32)],
        seed: u64,
        clip: Option<&Mask>,
    ) {
        if let Some(m) = clip {
            self.check_mask(m);
        }
        if pts.len() < 2 {
            return;
        }
        self.surf_gen += 1;
        let s = self.f.scale;
        let mut rng = Rng::new(seed);
        let mut path = densify(&pts.iter().map(|&(x, y)| (x * s, y * s)).collect::<Vec<_>>());
        if path.len() < 2 {
            return;
        }
        if brush.wobble > 0.0 {
            let n = crate::noise::Fbm::new(seed as u32, 3, 60.0);
            let amp = brush.wobble * s;
            for p in path.iter_mut() {
                p.0 += n.get(p.0 / s, p.1 / s) * amp;
                p.1 += n.get(p.0 / s + 311.0, p.1 / s) * amp;
            }
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
        let nb = brush.bristles.max(1.0).ceil() as usize + 2;
        let strengths: Vec<f32> = (0..nb).map(|_| rng.f()).collect();
        let bristle = |a: f32| -> f32 {
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
        let bx1 = ((x1 + pad).ceil().max(0.0) as usize).min(self.f.w);
        let by1 = ((y1 + pad).ceil().max(0.0) as usize).min(self.f.h);
        if bx1 <= bx0 || by1 <= by0 {
            return;
        }
        let (bw, bh) = (bx1 - bx0, by1 - by0);

        // what's on the brush
        let (mut brush_col, smudge_only) = match medium {
            Medium::Body(c, _) | Medium::Light(c, _) => (c, false),
            Medium::Glaze(..) => ([0.0; 3], false),
            Medium::Blend(_) => {
                let (px, py) = path[0];
                (self.sample(px / s, py / s), true)
            }
        };
        let pickup = if smudge_only { brush.pickup.max(0.15) } else { brush.pickup };

        // ---- plan the dabs (sequential: pickup reads the canvas as it goes)
        struct Dab {
            x: f32,
            y: f32,
            r: f32,
            nx: f32,
            ny: f32,
            dry_th: f32,
            col: Rgb,
        }
        let step = (r0 * 0.2).clamp(0.5, 8.0);
        let pick_k = if pickup > 0.0 { 1.0 - (1.0 - pickup.min(0.99)).powf(step / (10.0 * s)) } else { 0.0 };
        let mut dabs = Vec::with_capacity((total / step) as usize + 2);
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
            if pick_k > 0.0 {
                let under = self.sample(px / s, py / s);
                brush_col = color::mix(brush_col, under, pick_k, Mix::Linear);
            }
            let u = d / total;
            let taper = {
                let a = if brush.taper.0 > 0.0 { smoothstep(0.0, brush.taper.0, u) } else { 1.0 };
                let b = if brush.taper.1 > 0.0 { smoothstep(0.0, brush.taper.1, 1.0 - u) } else { 1.0 };
                (a * b).max(0.15)
            };
            let load = (brush.load * (1.0 - brush.decay * u)).clamp(0.0, 1.0);
            dabs.push(Dab { x: px, y: py, r: r0 * taper, nx: -ty, ny: tx, dry_th: 1.0 - load, col: brush_col });
            d += step;
        }

        // ---- stamp: each row of the stroke's bounding box in parallel
        #[derive(Clone, Copy)]
        struct Px {
            cov: f32,
            col: Rgb,
            ridge: f32,
        }
        let mut local = vec![Px { cov: 0.0, col: [0.0; 3], ridge: 0.0 }; bw * bh];
        let soft0 = 1.0 - brush.softness.max(0.02);
        local.par_chunks_mut(bw).enumerate().for_each(|(yy, row)| {
            let fy = (yy + by0) as f32 + 0.5;
            for dab in &dabs {
                let dy = fy - dab.y;
                if dy.abs() > dab.r {
                    continue;
                }
                let half = (dab.r * dab.r - dy * dy).sqrt();
                let ix0 = ((dab.x - half - 0.5).floor() as isize).max(bx0 as isize);
                let ix1 = ((dab.x + half + 0.5).ceil() as isize).min(bx1 as isize - 1);
                let inv_r = 1.0 / dab.r;
                for ix in ix0..=ix1 {
                    let dx = ix as f32 + 0.5 - dab.x;
                    let dist = (dx * dx + dy * dy).sqrt() * inv_r;
                    if dist > 1.0 {
                        continue;
                    }
                    let edge = 1.0 - smoothstep(soft0, 1.0, dist);
                    let across = ((dx * dab.nx + dy * dab.ny) * inv_r).clamp(-1.0, 1.0);
                    let b = bristle(across);
                    let dry = smoothstep(dab.dry_th - 0.12, dab.dry_th + 0.12, b);
                    let c = edge * dry * (1.0 - brush.streak * (1.0 - b));
                    let p = &mut row[ix as usize - bx0];
                    if c > p.cov {
                        p.cov = c;
                        p.col = dab.col;
                        // ridges: bristle grooves, plus a lip at the stroke edge
                        p.ridge = (0.35 + 0.65 * b) * edge + 0.25 * smoothstep(0.6, 0.95, dist) * edge;
                    }
                }
            }
        });

        // ---- deposit, row-parallel
        let w = self.f.w;
        let impasto = brush.impasto;
        let rows = self.px[by0 * w..by1 * w]
            .par_chunks_mut(w)
            .zip(self.height[by0 * w..by1 * w].par_chunks_mut(w))
            .zip(self.film[by0 * w..by1 * w].par_chunks_mut(w))
            .zip(local.par_chunks(bw));
        rows.enumerate().for_each(|(yy, (((prow, hrow), frow), lrow))| {
            let y = yy + by0;
            for xx in 0..bw {
                let lp = lrow[xx];
                let mut c = lp.cov;
                if c <= 0.0 {
                    continue;
                }
                let x = xx + bx0;
                if let Some(m) = clip {
                    c *= m.data[y * w + x];
                    if c <= 0.0 {
                        continue;
                    }
                }
                let p = &mut prow[x];
                let (op, film) = match medium {
                    Medium::Glaze(pig, th) => {
                        *p = pig.over(*p, th * c);
                        (0.4, th * 0.1)
                    }
                    Medium::Body(_, op) => {
                        // Mixbox only matters when the colors really differ
                        let diff = (p[0] - lp.col[0]).abs().max((p[1] - lp.col[1]).abs()).max((p[2] - lp.col[2]).abs());
                        let mode = if diff > 0.04 { Mix::Pigment } else { Mix::Linear };
                        *p = color::mix(*p, lp.col, c * op, mode);
                        (op, 0.5 * op)
                    }
                    Medium::Light(_, op) => {
                        *p = color::mix(*p, lp.col, c * op, Mix::Linear);
                        (op, 0.3 * op)
                    }
                    Medium::Blend(op) => {
                        *p = color::mix(*p, lp.col, c * op, Mix::Linear);
                        (op * 0.6, 0.0)
                    }
                };
                if impasto > 0.0 {
                    // µm of relief (legacy stamp brush)
                    hrow[x] += impasto * lp.ridge * 60.0 * (c * op).min(1.0);
                }
                frow[x] += film * c;
            }
        });
    }
}

