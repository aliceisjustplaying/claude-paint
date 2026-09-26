//! The physical surface: woven linen, ground layers and paint films, as one
//! height field in micrometers, and how a wet layer levels before it sets.
//!
//! Sources (see notes/research/oil_paint_physics.md):
//! - leveling of a thin film, Orchard: a ridge of wavelength λ on a film of
//!   thickness h decays with time constant τ = 3η(λ/2π)⁴ / (σh³)
//!   (https://www.stevenabbott.co.uk/practical-coatings/levelling.php);
//! - a yield stress τ_y freezes ridges smaller than a_c = τ_y λ³ / (8π³σh)
//!   (Hester, JCT 1997); deep films (h > λ/6) use a_c ≈ τ_y λ² / (4π²σ);
//! - linseed/walnut oil surface tension σ ≈ 0.035 N/m;
//! - model oil paint: yield stress 10–20 Pa (fluid) up to ~3000 Pa (stiff)
//!   (Ranquet et al., Nat. Commun. 2023).
//!
//! Very thin fluid paint therefore flows into the valleys of the ground and
//! thins on the peaks ("pooling", Friedrich: CATS proceedings III p.127),
//! while stiff paint keeps its bristle striations.

use crate::canvas::Canvas;
use crate::rng::hash2;
use rayon::prelude::*;

/// Thickness of one coat of wet paint (wet volume 1.0), µm. Friedrich's
/// layers are "gossamer-thin" (CATS p.127); brushed mock-up coats measure
/// ~60–125 µm; one "coat" here is a lean, thin one.
pub const COAT_UM: f32 = 25.0;
/// Surface tension of drying oil, N/m.
const SIGMA: f32 = 0.035;
/// Time the paint levels before it has set enough to stop, s.
pub(crate) const SET_TIME: f32 = 900.0;
/// Thinnest layer `settle` treats as paint, µm (a ten-thousandth of a µm:
/// numerically nothing, far below any film). Heights are ~10² µm, where an
/// f32 resolves ~10⁻⁵ µm, so a deposit below this is lost in rounding and
/// its leveled share would be float residue.
const ADD_EPS_UM: f32 = 1e-4;

/// Plain-weave linen.
#[derive(Clone, Copy, Debug)]
pub struct Linen {
    /// Threads per cm of the vertical (warp) and horizontal (weft) threads.
    pub warp_per_cm: f32,
    pub weft_per_cm: f32,
    /// Height of the thread crowns above the interstices, µm.
    pub crown_um: f32,
    /// Irregularity of hand-spun thread (0 = even, 1 = strong slubs).
    pub slubs: f32,
    pub seed: u64,
}

impl Linen {
    /// Fine handwoven linen as Friedrich bought it ready primed in Dresden
    /// (10–16 threads/cm, proxy from Eckersberg's Dresden canvases, CATS
    /// p.45–47); the warp is more even than the weft (TCAP).
    pub fn fine(seed: u64) -> Self {
        Linen { warp_per_cm: 14.0, weft_per_cm: 12.0, crown_um: 160.0, slubs: 0.7, seed }
    }
}

/// Paint rheology from stiffness (0 = medium-rich glaze, 1 = stiff tube paint):
/// (low-shear viscosity Pa·s, yield stress Pa).
fn rheology(stiff: f32) -> (f32, f32) {
    let s = stiff.clamp(0.0, 1.0);
    (10f32.powf(0.3 + 3.0 * s), 5.0 * 60f32.powf(s))
}

/// Decay factor and frozen amplitude (µm) for a band of wavelength λ (m) in
/// a wet film of thickness h (m).
fn level_band(lambda: f32, h: f32, eta: f32, tau_y: f32, set_time: f32) -> (f32, f32) {
    if h < 5e-8 {
        return (1.0, f32::INFINITY);
    }
    let pi = std::f32::consts::PI;
    let (tau, a_c) = if h < lambda / 6.0 {
        (
            3.0 * eta * (lambda / (2.0 * pi)).powi(4) / (SIGMA * h * h * h),
            tau_y * lambda.powi(3) / (8.0 * pi.powi(3) * SIGMA * h),
        )
    } else {
        (2.0 * eta * lambda / (2.0 * pi * SIGMA), tau_y * lambda * lambda / (4.0 * pi * pi * SIGMA))
    };
    ((-set_time / tau).exp(), a_c * 1e6)
}

/// Shrink a band coefficient by leveling, down to the yield floor.
#[inline]
fn shrink(d: f32, decay: f32, a_c: f32) -> f32 {
    let m = d.abs();
    d.signum() * (m * decay).max(m.min(a_c))
}

/// The two bands a wet layer levels in at a pixel size: the bristle and
/// thread scale (box radius `r1` px, wavelength `lam1` m) and the stroke
/// scale (`r2`, `lam2`).
struct Bands {
    r1: usize,
    r2: usize,
    lam1: f32,
    lam2: f32,
}

impl Bands {
    fn at(px_mm: f32) -> Bands {
        let r1 = ((0.15 / px_mm).round() as usize).max(1);
        let r2 = ((0.9 / px_mm).round() as usize).max(r1 + 1);
        let lam1 = ((2 * r1 + 1) as f32 * px_mm * 1.5).max(0.25) * 1e-3;
        let lam2 = ((2 * r2 + 1) as f32 * px_mm * 1.5).max(1.5) * 1e-3;
        Bands { r1, r2, lam1, lam2 }
    }
}

/// Make the film's total exactly the paint laid: the local conservation
/// before this is only approximate (blur edges), so the film is scaled by
/// one small factor, except that no pixel is pushed past its bound (below
/// `lo` when the factor shrinks the film, above `hi` when it grows it):
/// those stop at the bound and the rest make up the difference. When no
/// bound is in the way (always, for `settle`) it is the one factor.
fn conserve_total(add: &[f32], out: &mut [f32], lo: impl Fn(usize) -> f32 + Sync, hi: impl Fn(usize) -> f32 + Sync) {
    let (sa, so): (f64, f64) = (add.iter().map(|&v| v as f64).sum(), out.iter().map(|&v| v as f64).sum());
    debug_assert!(so.is_finite(), "settle: non-finite film");
    if !(so > 0.0 && so.is_finite()) {
        return;
    }
    let mut k = (sa / so) as f32;
    let down = k < 1.0;
    // where a pixel stops, and whether the factor would carry it past that
    let stop = |i: usize, v: f32| if down { v.min(lo(i)) } else { v.max(hi(i)) };
    let past = |i: usize, v: f32, k: f32| if down { v * k < lo(i) } else { v * k > hi(i) };
    let mut pinned = vec![false; out.len()];
    // pinning some makes the factor on the rest stronger, which may carry
    // more past theirs: repeat until a round pins nothing new (every round
    // that goes on pins at least one more pixel, so this ends within n + 1)
    loop {
        let mut more = false;
        for (i, p) in pinned.iter_mut().enumerate() {
            if !*p && past(i, out[i], k) {
                *p = true;
                more = true;
            }
        }
        if !more {
            break;
        }
        let (mut fixed, mut free) = (0.0f64, 0.0f64);
        for (i, &v) in out.iter().enumerate() {
            if pinned[i] {
                fixed += stop(i, v) as f64;
            } else {
                free += v as f64;
            }
        }
        if free <= 0.0 {
            break;
        }
        k = ((sa - fixed) / free) as f32;
    }
    out.par_iter_mut().zip(&pinned).enumerate().for_each(|(i, (o, &p))| *o = if p { stop(i, *o) } else { *o * k });
}

/// Separable box blur of a rect-sized buffer (w × h), radius r, clamped.
pub(crate) fn box_blur(src: &[f32], w: usize, h: usize, r: usize) -> Vec<f32> {
    if r == 0 {
        return src.to_vec();
    }
    let n = (2 * r + 1) as f32;
    let mut tmp = vec![0.0f32; w * h];
    tmp.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let s = &src[y * w..(y + 1) * w];
        let mut acc = 0.0f32;
        for k in 0..=2 * r {
            acc += s[k.saturating_sub(r).min(w - 1)];
        }
        for (x, o) in row.iter_mut().enumerate() {
            *o = acc / n;
            acc += s[(x + r + 1).min(w - 1)] - s[x.saturating_sub(r)];
        }
    });
    let mut out = vec![0.0f32; w * h];
    // vertical: running sums per column, processed in column blocks
    let cols: Vec<usize> = (0..w).collect();
    let colvals: Vec<Vec<f32>> = cols
        .par_chunks(64)
        .map(|cs| {
            let mut res = vec![0.0f32; cs.len() * h];
            for (ci, &x) in cs.iter().enumerate() {
                let mut acc = 0.0f32;
                for k in 0..=2 * r {
                    acc += tmp[k.saturating_sub(r).min(h - 1) * w + x];
                }
                for y in 0..h {
                    res[ci * h + y] = acc / n;
                    acc += tmp[(y + r + 1).min(h - 1) * w + x] - tmp[y.saturating_sub(r) * w + x];
                }
            }
            res
        })
        .collect();
    for (bi, block) in colvals.iter().enumerate() {
        let x0 = bi * 64;
        let nc = block.len() / h;
        for ci in 0..nc {
            for y in 0..h {
                out[y * w + x0 + ci] = block[ci * h + y];
            }
        }
    }
    out
}

impl Canvas {
    /// Millimeters per pixel.
    pub fn px_mm(&self) -> f32 {
        self.mm_per_unit / self.f.scale
    }

    /// Lay the bare support: woven linen height in µm, averaged over each
    /// pixel's footprint so threads finer than a pixel don't alias.
    pub(crate) fn build_support(&mut self) {
        let Some(l) = self.linen else { return };
        let (w, h) = (self.f.w, self.f.h);
        let (ox, oy) = (self.f.x0, self.f.y0);
        let px = self.px_mm();
        let pitch = (10.0 / l.warp_per_cm).min(10.0 / l.weft_per_cm);
        let ss = ((3.0 * px / pitch).ceil() as usize).clamp(2, 6);
        self.height.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            for (x, v) in row.iter_mut().enumerate() {
                let mut acc = 0.0;
                for j in 0..ss {
                    for i in 0..ss {
                        let xm = ((x + ox) as f32 + (i as f32 + 0.5) / ss as f32) * px;
                        let ym = ((y + oy) as f32 + (j as f32 + 0.5) / ss as f32) * px;
                        acc += linen_um(xm, ym, &l);
                    }
                }
                *v = acc / (ss * ss) as f32;
            }
        });
        let _ = h;
        self.surf_gen += 1;
    }

    /// A wet layer of thickness `add` (µm, per pixel of `rect`) and stiffness
    /// `stiff` goes onto the surface, levels and sets. Updates the height
    /// field and returns the redistributed thickness (µm) per pixel of `rect`:
    /// thin fluid paint gathers in the valleys and thins on the peaks.
    pub(crate) fn settle(&mut self, rect: (usize, usize, usize, usize), add: &[f32], stiff: &[f32]) -> Vec<f32> {
        let sets = vec![SET_TIME; add.len()];
        self.settle_for(rect, add, stiff, &sets)
    }

    /// `settle`, with each pixel's paint leveling for its own time `sets`
    /// (s): how long it stayed fluid (see `drying`).
    pub(crate) fn settle_for(&mut self, rect: (usize, usize, usize, usize), add: &[f32], stiff: &[f32], sets: &[f32]) -> Vec<f32> {
        // far below any film is nothing at all (see ADD_EPS_UM): zero it so
        // float residue can't pose as paint in the ratios below
        let clean: Vec<f32>;
        let add = if add.iter().any(|&a| a > 0.0 && a < ADD_EPS_UM) {
            clean = add.iter().map(|&a| if a < ADD_EPS_UM { 0.0 } else { a }).collect();
            &clean[..]
        } else {
            add
        };
        let (x0, y0, x1, y1) = rect;
        let (rw, rh) = (x1 - x0, y1 - y0);
        let w = self.f.w;
        let Bands { r1, r2, lam1, lam2 } = Bands::at(self.px_mm());
        let mut old = vec![0.0f32; rw * rh];
        let mut s = vec![0.0f32; rw * rh];
        for y in 0..rh {
            for x in 0..rw {
                let i = y * rw + x;
                old[i] = self.height[(y0 + y) * w + x0 + x];
                s[i] = old[i] + add[i];
            }
        }
        let l1 = box_blur(&box_blur(&s, rw, rh, r1), rw, rh, r1);
        let l2 = box_blur(&box_blur(&l1, rw, rh, r2), rw, rh, r2);
        let mut out = vec![0.0f32; rw * rh];
        out.par_chunks_mut(rw).enumerate().for_each(|(y, row)| {
            for x in 0..rw {
                let i = y * rw + x;
                let a = add[i];
                if a <= 0.0 {
                    row[x] = 0.0;
                    continue;
                }
                let hm = a * 1e-6;
                let (eta, ty) = rheology(stiff[i]);
                let (k1, c1) = level_band(lam1, hm, eta, ty, sets[i]);
                let (k2, c2) = level_band(lam2, hm, eta, ty, sets[i]);
                let d1 = shrink(s[i] - l1[i], k1, c1);
                let d2 = shrink(l1[i] - l2[i], k2, c2);
                let lev = l2[i] + d1 + d2;
                // the wet film can move sideways but not dig into what's under it
                row[x] = (lev - old[i]).max(0.0);
            }
        });
        // conserve paint: the leveled shape says where the wet film gathers,
        // but only the paint that was put down can move, and only nearby.
        // Rescale by the ratio of paint laid to paint kept, both averaged
        // over the leveling distance, so each neighborhood keeps its volume.
        let laid = box_blur(&box_blur(add, rw, rh, r2), rw, rh, r2);
        let kept = box_blur(&box_blur(&out, rw, rh, r2), rw, rh, r2);
        out.par_iter_mut().enumerate().for_each(|(i, o)| {
            // (a ratio of blur residues is not a ratio: it can overflow and
            // turn 0 × ∞ into NaN, which then paints full masstone)
            let r = laid[i] / kept[i];
            if add[i] <= 0.0 {
                *o = 0.0;
            } else if kept[i] > laid[i] * 1e-3 && r.is_finite() && (*o * r).is_finite() {
                *o *= r;
            } else {
                // everything drained away: nowhere to pool, keep it in place
                *o = add[i];
            }
        });
        conserve_total(add, &mut out, |_| f32::NEG_INFINITY, |_| f32::INFINITY);
        self.raise(rect, &old, add, &out);
        out
    }

    /// Raise the height of `rect` to the surface under the paint (`old`)
    /// plus the film settled there (`out`), where paint was laid (`add`);
    /// all three are `rect`-sized.
    fn raise(&mut self, rect: (usize, usize, usize, usize), old: &[f32], add: &[f32], out: &[f32]) {
        let (x0, y0, x1, y1) = rect;
        let (rw, w) = (x1 - x0, self.f.w);
        for y in 0..y1 - y0 {
            for x in 0..rw {
                let i = y * rw + x;
                if add[i] > 0.0 {
                    self.height[(y0 + y) * w + x0 + x] = old[i] + out[i];
                }
            }
        }
        self.surf_gen += 1;
    }

    /// A thin fluid film (a glaze or varnish, `add` µm per pixel of the
    /// whole buffer) over a **dry** surface levels its own thickness, not the
    /// relief under it. Returns the film per pixel (µm) and raises the height.
    ///
    /// `settle` levels the whole surface as if it were fluid, which is right
    /// when the wet layer is thick next to the relief. A 2 µm varnish over
    /// 100–300 µm dry impasto can't level the impasto: leveling there puts
    /// the level below the ridge tops (no film) and far above the foot of
    /// every step (tens of µm of film, dark brown lines at 3200px). Here the
    /// film follows the relief and only flows along it (lubrication theory,
    /// ∂h/∂t = −∇·(h³σ/3η ∇∇²z)): on a convex spot of band amplitude A (above
    /// the yield floor a_c) it thins as dh/dt = −h³σAk⁴/3η, which integrates
    /// in closed form to h = h₀ / √(1 + 2 (A/h₀)(T/τ₀)) (τ₀ = Orchard's τ at
    /// h₀): drainage slows as the film thins, so peaks keep a film. They keep
    /// at least `PEAK_FILM_UM` (a wetting film: the solvent is gone and the
    /// resin has set before it drains further). What drains moves downhill
    /// about one bristle band and gathers in the concave spots there, at
    /// most `POOL_MAX` times the film laid (a little deeper in the hollows,
    /// not a line of pooled color at a step's foot). Volume is conserved
    /// locally and then exactly, without pushing a pixel past either bound
    /// (see `conserve_total`).
    pub(crate) fn settle_film(&mut self, add: &[f32], stiff: f32) -> Vec<f32> {
        let (w, h) = (self.f.w, self.f.h);
        let n = w * h;
        debug_assert_eq!(add.len(), n);
        let add: Vec<f32> = add.iter().map(|&a| if a.is_finite() && a >= ADD_EPS_UM { a } else { 0.0 }).collect();
        let Bands { r1, r2, lam1, lam2 } = Bands::at(self.px_mm());
        let old = self.height.clone();
        // the relief's bands (convex > 0, concave < 0), µm
        let l1 = box_blur(&box_blur(&old, w, h, r1), w, h, r1);
        let l2 = box_blur(&box_blur(&l1, w, h, r2), w, h, r2);
        let (eta, ty) = rheology(stiff);
        // per pixel: the drainage rate 2 Σ (A/h₀)(T/τ₀) on convex bands and
        // the gathering weight on concave ones
        let (rate, gather): (Vec<f32>, Vec<f32>) = (0..n)
            .into_par_iter()
            .map(|i| {
                let a = add[i];
                if a <= 0.0 {
                    return (0.0, 0.0);
                }
                let hm = a * 1e-6;
                let mut rate = 0.0;
                let mut gather = 0.0;
                for (lam, d) in [(lam1, old[i] - l1[i]), (lam2, l1[i] - l2[i])] {
                    let (decay, a_c) = level_band(lam, hm, eta, ty, SET_TIME);
                    if !a_c.is_finite() {
                        continue;
                    }
                    // T/τ₀ from the decay factor e^(−T/τ₀)
                    let t_tau = -decay.max(1e-30).ln();
                    let over = (d.abs() - a_c).max(0.0) / a;
                    if d > 0.0 {
                        rate += 2.0 * over * t_tau;
                    } else {
                        gather += over * t_tau;
                    }
                }
                (rate, gather.min(1.0))
            })
            .unzip();
        // film left where it drains, and what it gives up
        let kept: Vec<f32> = (0..n)
            .map(|i| {
                let a = add[i];
                if a <= 0.0 {
                    return 0.0;
                }
                (a / (1.0 + rate[i]).sqrt()).max(a.min(PEAK_FILM_UM))
            })
            .collect();
        let drain: Vec<f32> = (0..n).map(|i| add[i] - kept[i]).collect();
        // it moves about one bristle band downhill, into the concave spots
        let wgt: Vec<f32> = (0..n).map(|i| add[i] * gather[i]).collect();
        let rd = 2 * r1;
        let bd = box_blur(&box_blur(&drain, w, h, rd), w, h, rd);
        let bw = box_blur(&box_blur(&wgt, w, h, rd), w, h, rd);
        let gain: Vec<f32> = (0..n)
            .map(|i| {
                if wgt[i] <= 0.0 || bw[i] <= 0.0 {
                    return 0.0;
                }
                let g = wgt[i] * bd[i] / bw[i];
                if g.is_finite() { g.min((POOL_MAX - 1.0) * add[i]) } else { 0.0 }
            })
            .collect();
        // only what found a place to gather left the peaks: scale the
        // drainage by the gain it made nearby (the rest stays in place)
        let bg = box_blur(&box_blur(&gain, w, h, rd), w, h, rd);
        let mut out: Vec<f32> = (0..n)
            .map(|i| {
                let a = add[i];
                if a <= 0.0 {
                    return 0.0;
                }
                let used = if bd[i] > 0.0 { (bg[i] / bd[i]).clamp(0.0, 1.0) } else { 0.0 };
                let v = a - drain[i] * used + gain[i];
                if v.is_finite() { v.max(0.0) } else { a }
            })
            .collect();
        conserve_total(&add, &mut out, |i| add[i].min(PEAK_FILM_UM), |i| POOL_MAX * add[i]);
        self.raise((0, 0, w, h), &old, &add, &out);
        out
    }
}

/// Film a glaze or varnish keeps on the peaks of a dry relief however much
/// it drains, µm (capped by the film laid): a wetting film about as thick
/// as its particles.
pub(crate) const PEAK_FILM_UM: f32 = 1.0;
/// Deepest a thin film gathers in a hollow of a dry relief, times the film laid.
const POOL_MAX: f32 = 2.0;

/// Smooth value noise in 0..1 (bilinear with smoothstep), lattice spacing 1.
pub(crate) fn vnoise(x: f32, y: f32, seed: u64) -> f32 {
    let (ix, iy) = (x.floor() as i64, y.floor() as i64);
    let (fx, fy) = (x - ix as f32, y - iy as f32);
    let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let a = hash2(ix, iy, seed);
    let b = hash2(ix + 1, iy, seed);
    let c = hash2(ix, iy + 1, seed);
    let d = hash2(ix + 1, iy + 1, seed);
    let top = a + (b - a) * sx;
    let bot = c + (d - c) * sx;
    top + (bot - top) * sy
}

/// Bare plain-weave linen height at a point (mm), µm.
fn linen_um(xm: f32, ym: f32, l: &Linen) -> f32 {
    use std::f32::consts::PI;
    let seed = l.seed;
    let (x0, y0) = (xm * l.warp_per_cm / 10.0, ym * l.weft_per_cm / 10.0);
    // threads wander a little; the weft more than the warp
    let u = x0 + 0.25 * (vnoise(x0 * 0.45, y0 * 0.06, seed) - 0.5);
    let v = y0 + 0.5 * l.slubs * (vnoise(x0 * 0.06, y0 * 0.45, seed + 3) - 0.5) + 0.25 * (vnoise(x0 * 0.2, y0 * 0.9, seed + 9) - 0.5);
    let (iu, iv) = (u.floor() as i64, v.floor() as i64);
    let (fu, fv) = (u - iu as f32, v - iv as f32);
    // thickness: warp even, weft with slubs (thick spots a few mm long)
    let tw = 0.85 + 0.3 * vnoise(iu as f32 * 0.93, v * 0.3, seed + 11);
    let slub = vnoise(u * 0.25, iv as f32 * 0.93, seed + 13);
    let th = 0.8 + 0.3 * vnoise(u * 0.4, iv as f32 * 0.93, seed + 17) + l.slubs * 1.2 * (slub - 0.6).max(0.0);
    let over = (iu + iv).rem_euclid(2) == 0;
    let warp = (PI * fu).sin().max(0.0).powf(0.7) * tw;
    let weft = (PI * fv).sin().max(0.0).powf(0.7) * th;
    let (a, b) = if over { (warp * (PI * fv).sin().max(0.0).sqrt(), weft * 0.55) } else { (warp * 0.55, weft * (PI * fu).sin().max(0.0).sqrt()) };
    // fiber fuzz
    let grit = vnoise(x0 * 3.1, y0 * 3.1, seed + 19) - 0.5;
    l.crown_um * (a.max(b) + 0.06 * grit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::hex;

    fn total(v: &[f32]) -> f64 {
        v.iter().map(|&x| x as f64).sum()
    }

    /// The exact global correction stops each pixel at its bound (the peak
    /// film when it scales down, the pool cap when it scales up) and the
    /// others make up the difference, so the total is still exact.
    #[test]
    fn the_total_is_made_exact_within_the_bounds() {
        // scaling down: pixel 0 is at its floor
        let add = [1.0f32, 1.0, 1.0, 1.0];
        let mut out = [0.5f32, 2.0, 1.0, 1.0];
        conserve_total(&add, &mut out, |_| 0.5, |_| f32::INFINITY);
        assert_eq!(out[0], 0.5, "{out:?}");
        assert!((total(&out) - 4.0).abs() < 1e-6, "{out:?}");
        // scaling up: pixel 1 is at its cap
        let mut out = [0.5f32, 2.0, 0.5, 0.5];
        conserve_total(&add, &mut out, |_| 0.0, |_| 2.0);
        assert_eq!(out[1], 2.0, "{out:?}");
        assert!((total(&out) - 4.0).abs() < 1e-6, "{out:?}");
        // no bound in the way: one factor for all, as before
        let mut out = [0.5f32, 2.0, 1.0, 1.0];
        conserve_total(&add, &mut out, |_| 0.0, |_| f32::INFINITY);
        let k = (4.0f64 / 4.5) as f32;
        assert_eq!(out, [0.5 * k, 2.0 * k, k, k]);
    }

    /// A cascade that needs many pinning rounds (each round's stronger
    /// factor carries a few more pixels to the floor) still ends inside
    /// the bounds: it used to stop after 8 rounds and leave pixels below
    /// the floor (review of the maintenance round, finding 1).
    #[test]
    fn a_long_pinning_cascade_keeps_the_bounds() {
        let n = 1024usize;
        let add = vec![0.5f32; n];
        let mut out: Vec<f32> = (0..n).map(|i| 0.5 + 0.5 * i as f32 / (n - 1) as f32).collect();
        conserve_total(&add, &mut out, |_| 0.5, |_| 1.0);
        let min = out.iter().cloned().fold(f32::INFINITY, f32::min);
        assert!(min >= 0.5, "a pixel below the floor: {min}");
        assert!((total(&out) - 512.0).abs() < 1e-3, "total {}", total(&out));
    }

    /// A varnish over tall dry impasto at 3200px (0.094 mm/px) keeps its
    /// peak film and pool cap exactly, not to within the global correction.
    #[test]
    fn a_film_over_impasto_keeps_its_bounds_exactly() {
        let (w, h) = (240usize, 240usize);
        let mut c = Canvas::new(w, 1.0, hex("#808080")).with_size_mm(w as f32 * 0.094);
        for y in 0..h {
            for x in 0..w {
                let (xf, yf) = (x as f32, y as f32);
                let mut z = 20.0 * ((xf * 0.9).sin() * (yf * 0.8).sin()).abs();
                for k in 0..9 {
                    let (cx, cy) = (30.0 + 80.0 * (k % 3) as f32, 30.0 + 80.0 * (k / 3) as f32);
                    if ((xf - cx) / 22.0).powi(2) + ((yf - cy) / 12.0).powi(2) < 1.0 {
                        z += 100.0 + 25.0 * k as f32;
                    }
                }
                c.height[y * w + x] = z;
            }
        }
        let add = vec![2.25f32; w * h];
        let t = c.settle_film(&add, 0.05);
        assert!((total(&t) - total(&add)).abs() < total(&add) * 1e-6, "volume {} -> {}", total(&add), total(&t));
        let (lo, hi) = t.iter().fold((f32::MAX, 0.0f32), |(l, m), &v| (l.min(v), m.max(v)));
        assert!(lo >= PEAK_FILM_UM, "a peak thinned past the peak film: {lo} µm");
        assert!(hi <= POOL_MAX * 2.25, "a hollow pooled past the cap: {hi} µm");
    }
}
