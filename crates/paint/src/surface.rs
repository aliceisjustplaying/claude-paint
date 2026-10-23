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
fn rheology_at(engine: u32, stiff: f32) -> (f32, f32) {
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
            let s = crate::paper::lay(&p, f.x0, f.y0, f.w, f.h, self.px_mm());
            self.height = s.height;
            self.micro = s.micro;
            self.surf_gen += 1;
            self.sync_material_support();
            return;
        }
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
        self.sync_material_support();
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
        if let Some(store) = self.material_store() {
            return (0..self.f.w * self.f.h).map(|i| store.surface_height(i).expect("material pixel")).collect();
        }
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
        for engine in [4, 5, 7, 8] {
            let mut c = crate::Style::oil().prepare(80, 1.0, 3).with_engine(engine);
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
