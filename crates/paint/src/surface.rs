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
//! thins on the peaks ("pooling"), while stiff paint keeps its bristle
//! striations.

use crate::canvas::Canvas;
use crate::rng::hash2;
use rayon::prelude::*;

/// Thickness of one coat of wet paint (wet volume 1.0), µm. Brushed
/// mock-up coats measure ~60–125 µm; one "coat" here is a lean, thin one.
pub const COAT_UM: f32 = 25.0;
/// Surface tension of drying oil, N/m.
pub(crate) const SIGMA: f32 = 0.035;
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
    /// Fine plain-weave linen, 14 × 12 threads/cm (inside the 10–38
    /// threads/cm of 19th-century canvases); the warp is more even than the
    /// weft (TCAP, notes/research/oil_paint_physics.md §4).
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

/// Paint thickness (µm) over which a film bridges the fine relief under it
/// (engine 4, `settle_for`): a 40 µm film keeps about a third of the weave on
/// its surface, a 120 µm one a twentieth.
const BRIDGE_UM: f32 = 40.0;

/// `rheology` for a canvas of engine version `engine`. From engine 4 the
/// yield stress spans what oil paint has: 5 Pa for medium-rich paint up to
/// some 3000 Pa for stiff tube paint (notes/research/oil_paint_physics.md;
/// engines 1 to 3 stopped at 300 Pa, so a stroke's furrows leveled flat).
pub(crate) fn rheology_at(engine: u32, stiff: f32) -> (f32, f32) {
    if engine < 4 {
        return rheology(stiff);
    }
    let s = stiff.clamp(0.0, 1.0);
    (10f32.powf(0.3 + 3.0 * s), 5.0 * 600f32.powf(s))
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
#[derive(Default)]
struct BlurScratch {
    tmp: Vec<f32>,
    columns: Vec<Vec<f32>>,
}

thread_local! {
    static BLUR_SCRATCH: std::cell::RefCell<BlurScratch> = std::cell::RefCell::new(BlurScratch::default());
    static SETTLE_SCRATCH: std::cell::RefCell<[Vec<f32>; 8]> = std::cell::RefCell::new(Default::default());
}

pub(crate) fn box_blur(src: &[f32], w: usize, h: usize, r: usize) -> Vec<f32> {
    let mut out = Vec::new();
    box_blur_into(src, w, h, r, &mut out);
    out
}

pub(crate) fn box_blur_into(src: &[f32], w: usize, h: usize, r: usize, out: &mut Vec<f32>) {
    out.resize(w * h, 0.0);
    if r == 0 {
        out.copy_from_slice(src);
        return;
    }
    let n = (2 * r + 1) as f32;
    let BlurScratch { mut tmp, mut columns } = BLUR_SCRATCH.with(|s| std::mem::take(&mut *s.borrow_mut()));
    tmp.resize(w * h, 0.0);
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
    // vertical: running sums per column, processed in column blocks
    columns.resize_with(w.div_ceil(64), Vec::new);
    columns.par_iter_mut().enumerate().for_each(|(bi, res)| {
        let x0 = bi * 64;
        let nc = (w - x0).min(64);
        res.resize(nc * h, 0.0);
        let mut sums = [0.0f32; 64];
        for k in 0..=2 * r {
            let start = k.saturating_sub(r).min(h - 1) * w + x0;
            let row = &tmp[start..start + nc];
            for ci in 0..nc {
                sums[ci] += row[ci];
            }
        }
        for (y, row) in res.chunks_exact_mut(nc).enumerate() {
            let top = y.saturating_sub(r) * w + x0;
            let bottom = (y + r + 1).min(h - 1) * w + x0;
            let top = &tmp[top..top + nc];
            let bottom = &tmp[bottom..bottom + nc];
            for ci in 0..nc {
                row[ci] = sums[ci] / n;
                sums[ci] += bottom[ci] - top[ci];
            }
        }
    });
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for (bi, block) in columns.iter().enumerate() {
            let x0 = bi * 64;
            let nc = block.len() / h;
            row[x0..x0 + nc].copy_from_slice(&block[y * nc..(y + 1) * nc]);
        }
    });
    BLUR_SCRATCH.with(|s| *s.borrow_mut() = BlurScratch { tmp, columns });
}

impl Canvas {
    /// Millimeters per pixel.
    pub fn px_mm(&self) -> f32 {
        self.mm_per_unit / self.f.scale
    }

    /// Lay the bare support: a sheet of paper (`paper::lay`), or woven linen
    /// height in µm, averaged over each pixel's footprint so threads finer
    /// than a pixel don't alias.
    pub(crate) fn build_support(&mut self) {
        if let Some(p) = self.paper {
            let f = self.f;
            let s = crate::paper::lay(&p, f.x0, f.y0, f.w, f.h, self.px_mm(), self.relief_prefilter);
            self.height = s.height;
            self.micro = s.micro;
            self.surf_gen += 1;
            return;
        }
        let Some(l) = self.linen else { return };
        let (w, h) = (self.f.w, self.f.h);
        let (ox, oy) = (self.f.x0, self.f.y0);
        let px = self.px_mm();
        let pitch = (10.0 / l.warp_per_cm).min(10.0 / l.weft_per_cm);
        if self.relief_prefilter {
            // pixels as wide as the coarser thread see only the weave's
            // mean, read four times a thread (its thicknesses vary from
            // thread to thread)
            let coarse = (10.0 / l.warp_per_cm).max(10.0 / l.weft_per_cm);
            self.height = if px >= coarse {
                footprint_average(ox, oy, w, h, px, pitch / 4.0, |xm, ym| linen_mean_um(xm, ym, &l))
            } else {
                footprint_average(ox, oy, w, h, px, pitch / SAMPLES_PER_THREAD, |xm, ym| linen_um(xm, ym, &l))
            };
            self.surf_gen += 1;
            return;
        }
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
        self.settle_for(rect, add, stiff, &sets, false, None)
    }

    /// The surface with the wet paint on it, as the painter's raking light
    /// sees it (`seen_lit`): the dry height plus each wet film's paint and solvent thickness.
    /// The liquid film, paint and the solvent in it alike, bridges the fine
    /// relief under it (engine 4, `BRIDGE_UM`): a liquid's surface levels as
    /// surface tension pulls it flat, the thinner the liquid the sooner, so a
    /// thinned film is no less level than its paint alone would be.
    /// With the drain switch only the paint still liquid bridges it: a
    /// packed layer follows the relief (`settle_for`).
    pub(crate) fn wet_surface(&self) -> Vec<f32> {
        let (w, h) = (self.f.w, self.f.h);
        let wet: Vec<f32> = self.wet.vol.par_iter().enumerate().map(|(i, v)| v * COAT_UM + self.wet.solv.get(i).copied().unwrap_or(0.0)).collect();
        // (the drain switch: the paint still liquid, above a packed layer)
        let liquid = |i: usize| {
            let paint = self.wet.vol[i] * COAT_UM;
            if self.drain { paint - paint * self.wet.hide[i][7].clamp(0.0, 1.0) } else { paint }
        };
        if self.engine < 4 {
            return self.height.par_iter().zip(&wet).map(|(a, b)| a + b).collect();
        }
        // (no wet paint anywhere: every pixel is its dry height, without the blurs)
        if wet.par_iter().all(|&a| a <= 0.0) {
            return self.height.clone();
        }
        let Bands { r1, .. } = Bands::at(self.px_mm());
        if self.engine >= 9 {
            return self.height.par_iter().zip(&wet).map(|(a, b)| a + b).collect();
        }
        let fine = box_blur(&box_blur(&self.height, w, h, r1), w, h, r1);
        (0..w * h)
            .into_par_iter()
            .map(|i| {
                let a = wet[i];
                if a <= 0.0 {
                    return self.height[i];
                }
                let bridge = if self.liquid_film_levels { liquid(i) + self.wet.solv.get(i).copied().unwrap_or(0.0) } else { liquid(i) };
                let keep = (-bridge / BRIDGE_UM).exp();
                fine[i] + (self.height[i] - fine[i]) * keep + a
            })
            .collect()
    }

    /// `settle`, with each pixel's paint leveling for its own time `sets`
    /// (s): how long it stayed fluid (see `drying`). With `fluid`, only that
    /// share of each pixel's film flows (the drain switch: the rest has
    /// packed on an absorbent ground, `bristle::ground_drain`). The packed
    /// layer is a filter cake bonded to the ground: it follows the relief
    /// and keeps its thickness, and the liquid above it levels over its top,
    /// its depth setting how fast (as depth cubed) and how much of the relief
    /// under it it bridges. Only the liquid is rescaled and conserved, so no
    /// pixel ends below its packed layer, and the film returned is the two's.
    /// Where nothing has packed (`fluid` 1) it is as without `fluid`.
    pub(crate) fn settle_for(&mut self, rect: (usize, usize, usize, usize), add: &[f32], stiff: &[f32], sets: &[f32], bridge: bool, fluid: Option<&[f32]>) -> Vec<f32> {
        self.settle_with_transport(rect, add, stiff, sets, bridge, fluid, None)
    }

    pub(crate) fn settle_with_transport(&mut self, rect: (usize, usize, usize, usize), add: &[f32], stiff: &[f32], sets: &[f32], bridge: bool, fluid: Option<&[f32]>, transport: Option<&mut Vec<(usize, usize, f32)>>) -> Vec<f32> {
        // far below any film is nothing at all (see ADD_EPS_UM): zero it so
        // float residue can't pose as paint in the ratios below
        let clean: Vec<f32>;
        let add = if add.iter().any(|&a| a > 0.0 && a < ADD_EPS_UM) {
            clean = add.iter().map(|&a| if a < ADD_EPS_UM { 0.0 } else { a }).collect();
            &clean[..]
        } else {
            add
        };
        // (the drain switch: each pixel's packed layer, which stays as it
        // is, and the liquid above it, which levels; a liquid far below any
        // film is none, see ADD_EPS_UM: that pixel is packed through)
        let split: Option<(Vec<f32>, Vec<f32>)> = fluid.map(|f| {
            let liquid: Vec<f32> = add.iter().zip(f).map(|(&a, &f)| if a * f < ADD_EPS_UM { 0.0 } else { a * f }).collect();
            let packed = add.iter().zip(&liquid).map(|(&a, &l)| a - l).collect();
            (packed, liquid)
        });
        let (liq, under): (&[f32], Option<&[f32]>) = match &split {
            Some((p, l)) => (&l[..], Some(&p[..])),
            None => (add, None),
        };
        let (x0, y0, x1, y1) = rect;
        let (rw, rh) = (x1 - x0, y1 - y0);
        let w = self.f.w;
        let Bands { r1, r2, lam1, lam2 } = Bands::at(self.px_mm());
        // (engine 4's stiffer paint is the painting's films; a ground is
        // laid and leveled as it always was)
        let engine = if bridge { self.engine } else { self.engine.min(3) };
        let mut scratch = SETTLE_SCRATCH.with(|s| std::mem::take(&mut *s.borrow_mut()));
        let [old, s, mid, l1, l2, laid, kept, fine] = &mut scratch;
        old.resize(rw * rh, 0.0);
        s.resize(rw * rh, 0.0);
        for y in 0..rh {
            for x in 0..rw {
                let i = y * rw + x;
                old[i] = self.height[(y0 + y) * w + x0 + x];
                s[i] = old[i] + add[i];
            }
        }

        box_blur_into(s, rw, rh, r1, mid);
        box_blur_into(mid, rw, rh, r1, l1);
        box_blur_into(l1, rw, rh, r2, mid);
        box_blur_into(mid, rw, rh, r2, l2);
        let mut out = vec![0.0f32; rw * rh];
        out.par_chunks_mut(rw).enumerate().for_each(|(y, row)| {
            for x in 0..rw {
                let i = y * rw + x;
                let a = liq[i];
                if a <= 0.0 {
                    row[x] = 0.0;
                    continue;
                }
                let hm = a * 1e-6;
                let (eta, ty) = rheology_at(engine, stiff[i]);
                let (k1, c1) = level_band(lam1, hm, eta, ty, sets[i]);
                let (k2, c2) = level_band(lam2, hm, eta, ty, sets[i]);
                let d1 = shrink(s[i] - l1[i], k1, c1);
                let d2 = shrink(l1[i] - l2[i], k2, c2);
                let lev = l2[i] + d1 + d2;
                // the wet film can move sideways but not dig into what's
                // under it (with the drain switch, the liquid levels over
                // the packed layer's top)
                let base = match under {
                    Some(p) if p[i] > 0.0 => old[i] + p[i],
                    _ => old[i],
                };
                row[x] = (lev - base).max(0.0);
            }
        });
        // conserve paint: the leveled shape says where the wet film gathers,
        // but only the paint that was put down can move, and only nearby.
        // Rescale by the ratio of paint laid to paint kept, both averaged
        // over the leveling distance, so each neighborhood keeps its volume.
        // (With the drain switch only the liquid is laid, kept and moved.)
        box_blur_into(liq, rw, rh, r2, mid);
        box_blur_into(mid, rw, rh, r2, laid);
        box_blur_into(&out, rw, rh, r2, mid);
        box_blur_into(mid, rw, rh, r2, kept);
        out.par_iter_mut().enumerate().for_each(|(i, o)| {
            // (a ratio of blur residues is not a ratio: it can overflow and
            // turn 0 × ∞ into NaN, which then paints full masstone)
            let r = laid[i] / kept[i];
            if liq[i] <= 0.0 {
                *o = 0.0;
            } else if kept[i] > laid[i] * 1e-3 && r.is_finite() && (*o * r).is_finite() {
                *o *= r;
            } else {
                // everything drained away: nowhere to pool, keep it in place
                *o = liq[i];
            }
        });
        let mut local_transport = Vec::new();
        let transport = if self.engine >= 9 && bridge { Some(transport.unwrap_or(&mut local_transport)) } else { transport };
        if self.engine >= 9 && bridge {
            let ceiling = s.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            out.par_iter_mut().enumerate().for_each(|(i, o)| {
                let base = old[i] + under.map_or(0.0, |p| p[i]);
                *o = o.clamp(0.0, (ceiling - base).max(0.0));
            });
        } else {
            conserve_total(liq, &mut out, |_| f32::NEG_INFINITY, |_| f32::INFINITY);
        }
        if let Some(transport) = transport {
            let target = out;
            out = liq.to_vec();
            for vertical in [false, true] {
                let (lines, len) = if vertical { (rw, rh) } else { (rh, rw) };
                for line in 0..lines {
                    let index = |p| if vertical { p * rw + line } else { line * rw + p };
                    let donors: Vec<_> = (0..len).filter(|&p| out[index(p)] > target[index(p)]).collect();
                    let receivers: Vec<_> = (0..len).filter(|&p| out[index(p)] < target[index(p)]).collect();
                    let (mut d, mut r) = (0, 0);
                    while d < donors.len() && r < receivers.len() {
                        let (from, to) = (donors[d], receivers[r]);
                        if from.abs_diff(to) > 2 * r2 || (from.min(to)..from.max(to)).any(|p| liq[index(p)] <= 0.0) {
                            if from < to { d += 1; } else { r += 1; }
                            continue;
                        }
                        let (i, j) = (index(from), index(to));
                        let give = out[i] - target[i];
                        let need = target[j] - out[j];
                        let moved = give.min(need);
                        if moved > 0.0 {
                            out[i] -= moved;
                            out[j] += moved;
                            transport.push((i, j, moved));
                        }
                        if give <= need { d += 1; }
                        if need <= give { r += 1; }
                    }
                }
            }
        }
        // (the drain switch: the liquid, which bridges the relief, and the
        // film, the packed layer under it plus the liquid)
        let liquid = under.map(|p| {
            let l = out.clone();
            out.par_iter_mut().zip(p).for_each(|(o, &p)| {
                if p > 0.0 {
                    *o += p;
                }
            });
            l
        });
        self.raise(rect, &old, add, &out);
        // engine 4: a thick film bridges the fine relief under it (the weave,
        // the ground's brush marks): its top is shaped by the brush, not by
        // what it covers, so the fine relief fades from its surface over
        // some tens of µm of paint. Only the surface: the film's thickness
        // (and so its color) is as leveled above.
        if bridge && self.engine < 9 {
            box_blur_into(old, rw, rh, r1, mid);
            box_blur_into(mid, rw, rh, r1, fine);
            for y in 0..rh {
                for x in 0..rw {
                    let i = y * rw + x;
                    if add[i] > 0.0 {
                        // (by the film as it leveled here: thin on a peak it
                        // drained from; with the drain switch its liquid)
                        let depth = liquid.as_ref().map_or(out[i], |l| l[i]);
                        let keep = (-depth.max(0.0) / BRIDGE_UM).exp();
                        self.height[(y0 + y) * w + x0 + x] -= (old[i] - fine[i]) * (1.0 - keep);
                    }
                }
            }
        }
        SETTLE_SCRATCH.with(|s| *s.borrow_mut() = scratch);
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
    /// every step (tens of µm of film). Here the
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
        let (eta, ty) = rheology_at(self.engine, stiff);
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

/// The footprint each pixel's relief is averaged over under
/// `--@ relief-prefilter` (`footprint_average`): a low-pass filter, so the
/// pixel grid carries only the relief it can resolve. A grid of pixels
/// carries nothing finer than two pixels a cycle (Nyquist, 0.5 cycles a
/// pixel): anything finer folds back below it as a false, regular pattern
/// (16 threads/cm at 0.38 mm a pixel are 0.61 cycles a pixel and fold to a
/// checker at 0.39). A plain box over the pixel passes 64% of a ridge at
/// Nyquist and 50% at 0.6, and a Gaussian can't part a thread just past
/// Nyquist from the real relief just below it, so the footprint is the
/// ideal low-pass (a sinc, Shannon) cut at `FOOTPRINT_CUT` and windowed to
/// `FOOTPRINT_REACH` pixels by a Kaiser window of shape `FOOTPRINT_BETA`
/// (Kaiser and Schafer, IEEE Trans. ASSP 28, 1980). It passes 99% of the
/// relief up to 0.25 cycles a pixel and 0.3% at Nyquist and past it, and
/// keeps the mean exactly (the weights sum to one). Its weights dip below
/// zero beside the center (to −16% of the center's), so a sharp step in
/// the relief rings by a few percent of the step; the support's relief has
/// none. Neither physics nor measurement sets these three numbers (they say
/// how the render is sampled, not what the canvas is): they are the
/// textbook choice for a grid, estimates checked by
/// `prefiltered_weave_has_no_aliased_peak`.
pub(crate) const FOOTPRINT_CUT: f32 = 0.35;
/// The footprint's reach each side of the pixel's center, pixels.
pub(crate) const FOOTPRINT_REACH: f32 = 6.0;
/// The Kaiser window's shape parameter (6: some 50 to 60 dB down past the band).
pub(crate) const FOOTPRINT_BETA: f32 = 6.0;

/// How finely the weave is read for its footprint average, samples a
/// thread. Where a weft thread passes from under one warp to over the next
/// its height steps (`linen_um`), so the relief has harmonics far past the
/// thread's own frequency, and a regular grid of samples folds them back
/// as a faint pattern beating with the weave; one point at random in each
/// cell of the grid (stratified) turns that into noise. At 8 a thread the
/// footprint averages come within 1.5 µm rms of the weave read 48 times a
/// thread (on a weave whose averaged relief varies by 6.4 µm rms, at 16 × 15
/// threads/cm and 0.38 mm a pixel), 0.8 µm at 12, 0.6 at 16.
pub(crate) const SAMPLES_PER_THREAD: f32 = 8.0;

/// Most samples a pixel takes each way for its footprint average. Coarser
/// than this, relief finer than a sample comes out as faint noise (the
/// stratified points) rather than averaging to nothing exactly.
const MAX_SAMPLES_PER_PX: usize = 8;

/// The modified Bessel function I₀ (its power series, to f32 precision for
/// the window's arguments up to 10).
fn bessel_i0(x: f32) -> f32 {
    let (mut sum, mut term, q) = (1.0f64, 1.0f64, (x as f64 * 0.5).powi(2));
    for k in 1..40 {
        term *= q / (k * k) as f64;
        sum += term;
        if term < 1e-12 * sum {
            break;
        }
    }
    sum as f32
}

/// The footprint's weight `o` pixels from a pixel's center (unnormalized).
pub(crate) fn footprint_weight(o: f32) -> f32 {
    use std::f32::consts::PI;
    let t = o / FOOTPRINT_REACH;
    if t.abs() >= 1.0 {
        return 0.0;
    }
    let a = 2.0 * FOOTPRINT_CUT * o;
    let sinc = if a.abs() < 1e-6 { 1.0 } else { (PI * a).sin() / (PI * a) };
    sinc * bessel_i0(FOOTPRINT_BETA * (1.0 - t * t).sqrt()) / bessel_i0(FOOTPRINT_BETA)
}

/// The relief `f` (µm at a point in mm) averaged over each pixel's
/// footprint, for the window `(x0, y0, w, h)` of a canvas whose pixels are
/// `px` mm, from samples at most `spacing` mm apart (but no more than
/// `MAX_SAMPLES_PER_PX` a pixel each way), so relief finer than the pixels
/// can carry averages out and only what they resolve stays. The samples
/// fall one at random in each cell of a grid fixed to the canvas, not the
/// window, so a crop gets the whole canvas's values.
pub(crate) fn footprint_average(x0: usize, y0: usize, w: usize, h: usize, px: f32, spacing: f32, f: impl Fn(f32, f32) -> f32 + Sync) -> Vec<f32> {
    let ss = ((px / spacing.max(1e-6)).ceil() as usize).clamp(1, MAX_SAMPLES_PER_PX);
    footprint_average_over(x0, y0, w, h, px, ss, (FOOTPRINT_REACH.ceil() as usize, &footprint_weight), f)
}

/// The weights of the taps of a pixel's footprint sampled `ss` times a
/// pixel, `r` pixels each side, by `kernel` (pixels from the center),
/// normalized: tap j sits (j − r·ss + ½ − ss/2)/ss pixels from the center.
pub(crate) fn footprint_taps(ss: usize, r: usize, kernel: &dyn Fn(f32) -> f32) -> Vec<f32> {
    let taps = (2 * r + 1) * ss;
    let mut wts: Vec<f32> = (0..taps).map(|j| kernel((j as f32 - (r * ss) as f32 + 0.5 - 0.5 * ss as f32) / ss as f32)).collect();
    let sum: f32 = wts.iter().sum();
    wts.iter_mut().for_each(|v| *v /= sum);
    wts
}

/// `footprint_average` from `ss` samples a pixel each way, over a footprint
/// of `kernel` (weight by pixels from the center) reaching `r` pixels each
/// side.
#[allow(clippy::too_many_arguments)]
pub(crate) fn footprint_average_over(x0: usize, y0: usize, w: usize, h: usize, px: f32, ss: usize, (r, kernel): (usize, &(dyn Fn(f32) -> f32 + Sync)), f: impl Fn(f32, f32) -> f32 + Sync) -> Vec<f32> {
    let mut out = vec![0.0f32; w * h];
    if w == 0 || h == 0 {
        return out;
    }
    let ss = ss.max(1);
    let d = px / ss as f32;
    let wts = footprint_taps(ss, r, kernel);
    let taps = wts.len();
    // bands of rows one after another, each read and averaged across in
    // parallel, row by row of samples (the footprint's reach each side is
    // read again by the next band: 5% more samples at 256 rows a band)
    let band = 256usize;
    let (gx0, nfx) = ((x0 * ss) as i64 - (r * ss) as i64, (w - 1) * ss + taps);
    let mut across: Vec<f32> = Vec::new();
    for (bi, chunk) in out.chunks_mut(band * w).enumerate() {
        let rows = chunk.len() / w;
        let gy0 = ((y0 + bi * band) * ss) as i64 - (r * ss) as i64;
        let nfy = (rows - 1) * ss + taps;
        across.clear();
        across.resize(nfy * w, 0.0);
        across.par_chunks_mut(w).enumerate().for_each_init(
            || vec![0.0f32; nfx],
            |fine, (fy, arow)| {
                let gy = gy0 + fy as i64;
                for (fx, v) in fine.iter_mut().enumerate() {
                    let gx = gx0 + fx as i64;
                    // one point at random in each cell of the sample grid
                    // (stratified): what the grid can't resolve comes out
                    // as noise, not a pattern beating with the weave
                    let (jx, jy) = (hash2(gx, gy, 0x51D3_F00D), hash2(gx, gy, 0x7E57_CE11));
                    *v = f((gx as f32 + jx) * d, (gy as f32 + jy) * d);
                }
                for (x, a) in arow.iter_mut().enumerate() {
                    *a = wts.iter().zip(&fine[x * ss..x * ss + taps]).map(|(k, v)| k * v).sum();
                }
            },
        );
        chunk.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            for (j, k) in wts.iter().enumerate() {
                let src = &across[(y * ss + j) * w..(y * ss + j + 1) * w];
                row.iter_mut().zip(src).for_each(|(o, v)| *o += k * v);
            }
        });
    }
    out
}

/// The mean height of plain weave over a period of its over and under (two
/// threads each way), in crowns, for warp threads 1 thick and weft threads
/// `rho` thick: `linen_um`'s threads averaged over a crossing where the
/// warp is on top and one where the weft is, by the midpoint rule on a
/// grid whose cells end where the threads' heights step. Tabulated once
/// for `rho` from 0.25 to 4 (a weft four times thinner or thicker than the
/// warp: `linen_um`'s threads run 0.6 to 2 times).
fn weave_mean(rho: f32) -> f32 {
    use std::f32::consts::PI;
    const N: usize = 96;
    const STEPS: usize = 75;
    static TABLE: std::sync::OnceLock<Vec<f32>> = std::sync::OnceLock::new();
    let rho_at = |k: usize| 0.25 + 3.75 * k as f32 / STEPS as f32;
    let table = TABLE.get_or_init(|| {
        let s: Vec<f32> = (0..N).map(|i| (PI * (i as f32 + 0.5) / N as f32).sin()).collect();
        (0..=STEPS)
            .map(|k| {
                let rho = rho_at(k);
                let mut acc = 0.0f64;
                for &sv in &s {
                    for &su in &s {
                        let (warp, weft) = (su.powf(0.7), sv.powf(0.7) * rho);
                        let over = (warp * sv.sqrt()).max(weft * 0.55);
                        let under = (warp * 0.55).max(weft * su.sqrt());
                        acc += 0.5 * (over + under) as f64;
                    }
                }
                (acc / (N * N) as f64) as f32
            })
            .collect()
    });
    let t = ((rho - 0.25) / 3.75 * STEPS as f32).clamp(0.0, STEPS as f32);
    let k = (t as usize).min(STEPS - 1);
    table[k] + (table[k + 1] - table[k]) * (t - k as f32)
}

/// Bare linen's height at a point (mm), µm, where the pixels are at least a
/// thread wide: the weave's mean over its over and under (`weave_mean`) for
/// the warp and weft threads there, as thick as `linen_um` makes them, the
/// weft's slubs with them. A pixel this wide resolves neither the threads
/// nor their over and under (twice their pitch, past the footprint's band),
/// only the weave's mean and how it varies from thread to thread and slub
/// to slub; this is that, without the threads. The thicknesses `linen_um`
/// gives each whole thread (by its index) vary smoothly here, matching each
/// thread at its middle, and the threads' wandering (a fraction of a thread)
/// is left out.
pub(crate) fn linen_mean_um(xm: f32, ym: f32, l: &Linen) -> f32 {
    let seed = l.seed;
    let (x0, y0) = (xm * l.warp_per_cm / 10.0, ym * l.weft_per_cm / 10.0);
    let tw = 0.85 + 0.3 * vnoise((x0 - 0.5) * 0.93, y0 * 0.3, seed + 11);
    let slub = vnoise(x0 * 0.25, (y0 - 0.5) * 0.93, seed + 13);
    let th = 0.8 + 0.3 * vnoise(x0 * 0.4, (y0 - 0.5) * 0.93, seed + 17) + l.slubs * 1.2 * (slub - 0.6).max(0.0);
    l.crown_um * tw * weave_mean(th / tw)
}

/// Bare plain-weave linen height at a point (mm), µm.
pub(crate) fn linen_um(xm: f32, ym: f32, l: &Linen) -> f32 {
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
mod box_blur_tests {
    use super::*;

    fn reference(src: &[f32], w: usize, h: usize, r: usize) -> Vec<f32> {
        if r == 0 {
            return src.to_vec();
        }
        let n = (2 * r + 1) as f32;
        let mut tmp = vec![0.0; w * h];
        for (y, row) in tmp.chunks_exact_mut(w).enumerate() {
            let s = &src[y * w..(y + 1) * w];
            let mut acc = 0.0f32;
            for k in 0..=2 * r {
                acc += s[k.saturating_sub(r).min(w - 1)];
            }
            for (x, o) in row.iter_mut().enumerate() {
                *o = acc / n;
                acc += s[(x + r + 1).min(w - 1)] - s[x.saturating_sub(r)];
            }
        }
        let mut out = vec![0.0; w * h];
        for x in 0..w {
            let mut acc = 0.0f32;
            for k in 0..=2 * r {
                acc += tmp[k.saturating_sub(r).min(h - 1) * w + x];
            }
            for y in 0..h {
                out[y * w + x] = acc / n;
                acc += tmp[(y + r + 1).min(h - 1) * w + x] - tmp[y.saturating_sub(r) * w + x];
            }
        }
        out
    }

    #[test]
    fn row_major_columns_preserve_reference_bits() {
        let values = [0.0, -0.0, f32::from_bits(1), -f32::from_bits(1), f32::MIN_POSITIVE,
            -f32::MIN_POSITIVE, 1.0, -1.0, f32::from_bits(0x3f800001), 1e20, -1e20];
        for threads in [1, 4] {
            rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap().install(|| {
                let mut actual = vec![f32::NAN; 10000];
                let mut check = |src: &[f32], w, h, r| {
                    let expected = reference(src, w, h, r);
                    box_blur_into(src, w, h, r, &mut actual);
                    assert_eq!(actual.len(), expected.len());
                    for (i, (&a, &b)) in actual.iter().zip(&expected).enumerate() {
                        assert_eq!(a.to_bits(), b.to_bits(), "{w}x{h} r={r}, pixel {i}, threads={threads}");
                    }
                };
                for w in [129, 1, 2, 63, 64, 65, 127] {
                    for h in [1, 2, 5, 65] {
                        for pattern in 0..3 {
                            let src: Vec<f32> = (0..w * h).map(|i| match pattern {
                                0 => -0.0,
                                1 => values[i % values.len()],
                                _ => f32::from_bits(((i as u32).wrapping_mul(1664525).wrapping_add(1013904223) & 0x807fffff) | 0x3f000000),
                            }).collect();
                            for r in [0, 1, 3, w.max(h) + 1] {
                                check(&src, w, h, r);
                            }
                        }
                    }
                }
                for paper in [false, true] {
                    let c = Canvas::new(65, 1.3, crate::hex("#a8966f")).with_size_mm(13.0);
                    let c = if paper { c.with_paper(crate::paper::Paper::drawing(7)) } else { c.with_linen(Linen::fine(7)) };
                    for r in [0, 1, 3, 67] {
                        check(&c.height, c.f.w, c.f.h, r);
                    }
                }
            });
        }
    }
}

#[cfg(test)]
mod liquid_film_tests {
    use super::*;

    #[test]
    fn absent_switch_keeps_the_old_surface_and_lighting_through_checkpoints() {
        let prepared = crate::Style::oil().prepare(80, 1.0, 3);
        for engine in [4, 5, 7, 8] {
            let mut c = prepared.clone().with_engine(engine);
            c.wet.solv = vec![0.0; c.height.len()];
            for i in 0..c.height.len() {
                c.wet.vol[i] = (i % 7) as f32 * 0.3;
                c.wet.solv[i] = (i % 11) as f32 * 30.0;
            }
            assert!(!c.liquid_film_levels());
            let (w, h) = (c.f.w, c.f.h);
            let Bands { r1, .. } = Bands::at(c.px_mm());
            let fine = box_blur(&box_blur(&c.height, w, h, r1), w, h, r1);
            let expected: Vec<f32> = (0..w * h).map(|i| {
                let a = c.wet.vol[i] * COAT_UM + c.wet.solv[i];
                if a <= 0.0 {
                    return c.height[i];
                }
                let keep = (-c.wet.vol[i] * COAT_UM / BRIDGE_UM).exp();
                fine[i] + (c.height[i] - fine[i]) * keep + a
            }).collect();
            crate::tests::image_tolerance::assert_fields_close(c.wet_surface(), expected.iter().copied());
            let old_lit = c.seen_lit(315.0, 55.0, 1.0);
            for levels in [false, true] {
                let c = c.clone().with_liquid_film_levels(levels);
                let mut saved = Vec::new();
                c.write_state(&mut saved, "").unwrap();
                let restored = Canvas::read_state(&mut &saved[..]).unwrap().0;
                assert_eq!(restored.liquid_film_levels(), levels);
                assert_eq!(restored.wet_surface(), c.wet_surface());
                assert_eq!(restored.seen_lit(315.0, 55.0, 1.0), c.seen_lit(315.0, 55.0, 1.0));
                if levels {
                    assert_ne!(restored.seen_lit(315.0, 55.0, 1.0), old_lit);
                } else {
                    crate::tests::image_tolerance::assert_fields_close(restored.wet_surface(), expected.iter().copied());
                    crate::tests::image_tolerance::assert_fields_close(restored.seen_lit(315.0, 55.0, 1.0).into_iter().flatten(), old_lit.iter().flatten().copied());
                }
            }
        }
    }
}

#[cfg(test)]
mod relief_prefilter_tests {
    use super::*;

    /// 16 × 15 threads/cm, as on the Marly canvas.
    fn marly() -> Linen {
        Linen { warp_per_cm: 16.0, weft_per_cm: 15.0, crown_um: 160.0, slubs: 0.7, seed: 1880 }
    }

    /// A square canvas `n` px wide whose pixels are `px` mm, on `l`.
    fn canvas(n: usize, px: f32, l: Linen, prefilter: bool) -> Canvas {
        Canvas::new(n, 1.0, [0.8; 3]).with_relief_prefilter(prefilter).with_size_mm(n as f32 * px).with_linen(l)
    }

    /// The power of a line of samples at `f` cycles a sample (its mean taken out).
    fn power(v: &[f64], f: f64) -> f64 {
        let m = v.iter().sum::<f64>() / v.len() as f64;
        let (mut re, mut im) = (0.0, 0.0);
        for (i, x) in v.iter().enumerate() {
            let a = std::f64::consts::TAU * f * i as f64;
            re += (x - m) * a.cos();
            im -= (x - m) * a.sin();
        }
        (re * re + im * im) / v.len() as f64
    }

    /// Along lines of the weave across its warp (`y` fixed, so the footprint
    /// acts in x alone), the power of `coarse` (n pixels of `px` mm) at the
    /// frequency `f` cycles a pixel and its neighbors, and the power the
    /// weave itself has there (read 16 times a pixel): as it is, and passed
    /// by `gain` (the footprint's gain at a frequency).
    fn powers(l: &Linen, n: usize, px: f32, f: f64, coarse: &dyn Fn(f32) -> Vec<f64>, gain: &dyn Fn(f64) -> f64) -> (f64, f64, f64) {
        let k = 16usize;
        let (mut pc, mut pt, mut pg) = (0.0, 0.0, 0.0);
        for line in 0..32 {
            let yl = 3.1 + line as f32 * 1.7;
            let c = coarse(yl);
            let fine: Vec<f64> = (0..n * k).map(|i| linen_um((i as f32 + 0.5) * px / k as f32, yl, l) as f64).collect();
            for dk in [-1.0f64, 0.0, 1.0] {
                let ff = f + dk / n as f64;
                let t = power(&fine, ff / k as f64) / k as f64;
                pc += power(&c, ff);
                pt += t;
                pg += gain(ff).powi(2) * t;
            }
        }
        (pc, pt, pg)
    }

    /// The gain at `f` cycles a pixel of the footprint's taps sampled `ss`
    /// times a pixel.
    fn footprint_gain(ss: usize, f: f64) -> f64 {
        let r = FOOTPRINT_REACH.ceil() as usize;
        let w = footprint_taps(ss, r, &footprint_weight);
        w.iter().enumerate().map(|(j, &v)| v as f64 * (std::f64::consts::TAU * f * (j as f64 - (r * ss) as f64 + 0.5 - 0.5 * ss as f64) / ss as f64).cos()).sum::<f64>().abs()
    }

    /// A weave whose threads come a little finer than two pixels folds back
    /// below Nyquist when read at points: 16 threads/cm at 0.383 mm a pixel
    /// (0.61 cycles a pixel) show at 1 − 0.61 = 0.39, and 14 threads/cm
    /// (0.54, just past Nyquist) at 0.46, a pixel checker. Read at two
    /// points a pixel, as the canvas was laid, the pixels hold tens of times
    /// the power the weave itself has at those frequencies, a peak a render
    /// that resolves the threads doesn't have. Averaged over each pixel's
    /// footprint, what folded is a few percent of the weave's own power
    /// there, and the pixels hold what the footprint passes of it.
    #[test]
    fn prefiltered_weave_has_no_aliased_peak() {
        let (n, px) = (384usize, 0.383f32);
        for per_cm in [16.0f32, 14.0, 20.0] {
            let l = Linen { warp_per_cm: per_cm, ..marly() };
            let f = (px * per_cm / 10.0) as f64;
            let fold = 1.0 - f;
            let pitch = 10.0 / per_cm;
            let ss = ((px / (pitch / SAMPLES_PER_THREAD)).ceil() as usize).clamp(1, 64);
            let points = |yl: f32| (0..n).map(|x| 0.5 * (linen_um((x as f32 + 0.25) * px, yl, &l) + linen_um((x as f32 + 0.75) * px, yl, &l)) as f64).collect::<Vec<f64>>();
            let footprint = |yl: f32| footprint_average(0, 0, n, 1, px, pitch / SAMPLES_PER_THREAD, |x, _| linen_um(x, yl, &l)).iter().map(|&v| v as f64).collect::<Vec<f64>>();
            let (po, pt, pb) = powers(&l, n, px, fold, &points, &|f| (std::f64::consts::PI * f / 2.0).cos().abs());
            let (pn, _, pg) = powers(&l, n, px, fold, &footprint, &|f| footprint_gain(ss, f));
            let (old, new) = ((po - pb) / pt, (pn - pg) / pt);
            eprintln!("{per_cm} threads/cm, at {fold:.3} cycles a pixel: folded power, over the weave's own there: read at points {old:.1}, footprint {new:.3} (it passes {:.3} of the weave's)", pg / pt);
            assert!(old > 10.0, "{per_cm}/cm: reading at points should fold the threads ({old})");
            assert!(new.abs() < 0.05, "{per_cm}/cm: folded power at {fold} is {new} of the weave's own there");
        }
    }

    /// The footprint keeps the relief the pixels can resolve, at its real
    /// scale: along the lines, the power from 0.03 to 0.25 cycles a pixel
    /// (the slubs, thick and thin threads, 1.5 to 13 mm) is the weave's own
    /// as the footprint passes it (at least 0.94 of its amplitude), within
    /// 12% (6% of the amplitude: the noise of reading the weave
    /// `SAMPLES_PER_THREAD` times a thread).
    #[test]
    fn prefiltered_weave_keeps_what_the_pixels_resolve() {
        let (n, px) = (384usize, 0.383f32);
        let l = marly();
        let pitch = 10.0 / l.warp_per_cm;
        let ss = ((px / (pitch / SAMPLES_PER_THREAD)).ceil() as usize).clamp(1, 64);
        let footprint = |yl: f32| footprint_average(0, 0, n, 1, px, pitch / SAMPLES_PER_THREAD, |x, _| linen_um(x, yl, &l)).iter().map(|&v| v as f64).collect::<Vec<f64>>();
        for f in [0.03f64, 0.06, 0.1, 0.15, 0.2, 0.25] {
            let g = footprint_gain(ss, f);
            let (pc, pt, pg) = powers(&l, n, px, f, &footprint, &|f| footprint_gain(ss, f));
            eprintln!("{f} cycles a pixel: gain {g:.3}; the pixels hold {:.3} of the weave's power, {:.3} of it as passed", pc / pt, pc / pg);
            assert!(g > 0.94 && (pc / pg - 1.0).abs() < 0.12, "at {f} cycles a pixel: gain {g}, the pixels hold {} of the power passed", pc / pg);
        }
    }

    /// Averaging over the footprint keeps the relief's mean: away from the
    /// window's edge (where the footprint reaches past it), the footprint
    /// pixels' mean is the box average of the weave read sixteen times a
    /// pixel over the same patch.
    #[test]
    fn prefiltered_weave_keeps_the_mean() {
        let (n, px) = (256usize, 0.383f32);
        let l = marly();
        let new = canvas(n, px, l, true);
        let (k, e) = (16usize, 8usize);
        let d = px / k as f32;
        let fine = (e * k..(n - e) * k).into_par_iter().map(|j| (e * k..(n - e) * k).map(|i| linen_um((i as f32 + 0.5) * d, (j as f32 + 0.5) * d, &l) as f64).sum::<f64>()).sum::<f64>() / (((n - 2 * e) * k) as f64).powi(2);
        let m = (e..n - e).flat_map(|y| (e..n - e).map(move |x| (y, x))).map(|(y, x)| new.height[y * n + x] as f64).sum::<f64>() / ((n - 2 * e) as f64).powi(2);
        eprintln!("mean relief: footprint {m:.3} µm, box average of the fine weave {fine:.3} µm");
        assert!((m / fine - 1.0).abs() < 2e-3, "footprint mean {m} vs box average {fine}");
    }

    /// Where a pixel is wider than a thread (1 mm a pixel on 16 × 15
    /// threads/cm), the canvas holds the weave's mean (`linen_mean_um`)
    /// averaged over each footprint: it has the mean of the weave itself
    /// (within 1%) and follows the weave averaged over the footprint from
    /// samples 32 times a pixel (within a third of how much that varies:
    /// it keeps the thick and thin threads and the slubs, not the per-thread
    /// wander), and it varies as much (within 30%).
    #[test]
    fn coarse_pixels_hold_the_weaves_mean() {
        let (n, px) = (96usize, 1.0f32);
        let l = marly();
        let c = canvas(n, px, l, true);
        let r = FOOTPRINT_REACH.ceil() as usize;
        let full = footprint_average_over(0, 0, n, n, px, 32, (r, &footprint_weight), |x, y| linen_um(x, y, &l));
        let e = 8usize;
        let inner = |v: &[f32]| -> Vec<f64> { (e..n - e).flat_map(|y| (e..n - e).map(move |x| (y, x))).map(|(y, x)| v[y * n + x] as f64).collect() };
        let (a, b) = (inner(&c.height), inner(&full));
        let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
        let (ma, mb) = (mean(&a), mean(&b));
        let sd = |v: &[f64], m: f64| (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / v.len() as f64).sqrt();
        let (sa, sb) = (sd(&a, ma), sd(&b, mb));
        let diff = (a.iter().zip(&b).map(|(x, y)| ((x - ma) - (y - mb)).powi(2)).sum::<f64>() / a.len() as f64).sqrt();
        eprintln!("1 mm pixels: mean {ma:.2} µm (weave {mb:.2}), varies {sa:.2} µm rms (weave {sb:.2}), off by {diff:.2} µm rms");
        assert!((ma / mb - 1.0).abs() < 0.01, "mean {ma} vs {mb}");
        assert!(diff < sb / 3.0 && (sa / sb - 1.0).abs() < 0.3, "varies {sa} vs {sb}, off by {diff}");
    }

    /// A crop gets the whole canvas's footprint averages, bit for bit, as it
    /// got its point-sampled weave.
    #[test]
    fn a_crop_has_the_whole_canvas_relief() {
        let whole = Canvas::new(200, 1.25, [0.8; 3]).with_relief_prefilter(true).with_size_mm(76.6).with_linen(marly());
        let crop = Canvas::new_window(200, 1.25, [0.8; 3], Some(crate::canvas::Crop { units: [300.0, 200.0, 650.0, 520.0], margin: 0.0 })).with_relief_prefilter(true).with_size_mm(76.6).with_linen(marly());
        let f = crop.f;
        assert!(f.x0 > 0 && f.y0 > 0 && f.w < 200);
        for y in 0..f.h {
            for x in 0..f.w {
                assert_eq!(crop.height[y * f.w + x].to_bits(), whole.height[(y + f.y0) * whole.f.w + x + f.x0].to_bits(), "pixel ({x}, {y})");
            }
        }
    }

    /// Without the switch the support and a knifed ground are laid as they
    /// were: the weave read at ss×ss points a pixel, the ground's texture at
    /// each pixel's corner. With it, the felt of a paper, the weave and a
    /// knifed ground's texture all change, and a paper's fibers and pores
    /// stay as they are.
    #[test]
    fn absent_switch_lays_the_old_support() {
        let l = marly();
        let c = canvas(64, 0.383, l, false);
        let (w, px) = (64usize, c.px_mm());
        let pitch = (10.0 / l.warp_per_cm).min(10.0 / l.weft_per_cm);
        let ss = ((3.0 * px / pitch).ceil() as usize).clamp(2, 6);
        for (i, v) in c.height.iter().enumerate() {
            let (x, y) = (i % w, i / w);
            let mut acc = 0.0;
            for j in 0..ss {
                for k in 0..ss {
                    acc += linen_um((x as f32 + (k as f32 + 0.5) / ss as f32) * px, (y as f32 + (j as f32 + 0.5) / ss as f32) * px, &l);
                }
            }
            assert_eq!(v.to_bits(), (acc / (ss * ss) as f32).to_bits());
        }
        let n = canvas(64, 0.383, l, true);
        assert_ne!(c.height, n.height);
        let (mut a, mut b) = (c.clone(), n.clone());
        a.prime(crate::hex("#e8e0d0"), 1.0, 60.0, 0.6, 0.25, 9);
        b.prime(crate::hex("#e8e0d0"), 1.0, 60.0, 0.6, 0.25, 9);
        // the ground's own share: what the knife laid over the relief
        let (ta, tb): (Vec<f32>, Vec<f32>) = (a.film.clone(), b.film.clone());
        assert_ne!(ta, tb);
        let paper = |on: bool| Canvas::new(64, 1.0, [0.8; 3]).with_relief_prefilter(on).with_size_mm(24.5).with_paper(crate::paper::Paper::drawing(4));
        let (p0, p1) = (paper(false), paper(true));
        assert_eq!(p0.micro, p1.micro);
        let felt = crate::paper::Paper::drawing(4).felt.is_some();
        assert_eq!(p0.height != p1.height, felt);
    }
}

