//! Thinner: solvent (turpentine, spirits) knifed into a pile (engine 3).
//!
//! A thinned pile is paint plus a share `t` of solvent by volume
//! (`Paint::with_thinner`). On the brush the solvent rides with the paint
//! (`Bristle::solvent`); laid, it sits in the open film beside the paint
//! (`Wet::solv`, in µm) and leaves it over painting
//! time. Paint is never mixed with solvent: pigment, oil, stiffness and the
//! drying rate stay the paint's own, the film's optics and its oil cure
//! read only the paint, and the solvent is a separate quantity beside it.
//!
//! Three things the solvent does, all engine 3, all estimates:
//!
//! - **A thinned stroke lays a thin film** (`stroke_limit_um`). One stroke
//!   adds at most this much wet film (paint + solvent) to any pixel, a
//!   ceiling shared by all the hairs and all the parts of the stroke; what
//!   the brush can't lay stays on it. A thin, fluid liquid leaves a thinner
//!   film behind a moving surface than a thick one: the film drawn up by a
//!   moving plate grows with viscosity as (ηU)^(2/3) (Landau and Levich,
//!   "Dragging of a liquid by a moving plate", Acta Physicochimica URSS 17,
//!   42-54, 1942; a law for a plate drawn out of a liquid, used here only
//!   qualitatively, not as a calibrated brush law). The model assumes that
//!   turpentine, far more fluid than oil paint, makes the thinned paint
//!   more fluid the more of it there is; no measurement of a paint's
//!   viscosity against its thinner is used. The size of the ceiling is an
//!   ESTIMATE, set so that raw
//!   sienna thinned half lays an imprimatura (notes/thinner/RESULTS.md);
//!   it fades out as thinner goes to 0, where today's paint path runs
//!   unchanged.
//! - **It evaporates** (`evaporation_tau_min`): at a fixed paint thickness
//!   the same share leaves in each equal time step, solvent = s0 ×
//!   exp(-t / τ). A few drops of pure turpentine on white writing paper
//!   evaporate "in a few minutes" (Arthur Seymour Jennings, Paint & Colour
//!   Mixing, 1902, "To Test the Purity of Turpentine", pp. 74-75,
//!   https://www.gutenberg.org/cache/epub/56738/pg56738-images.html): a
//!   purity test for the turpentine, not a measurement of its release from
//!   an oil-paint film, so it only sets the scale of `TAU_MIN`. In a paint
//!   film the solvent left in it has to diffuse out through the
//!   film, so a thicker film holds it longer (C. M. Hansen, The Three
//!   Dimensional Solubility Parameter and Solvent Diffusion Coefficient,
//!   Danish Technical Press, 1967, §5.2: evaporation from the surface and
//!   diffusion out of the film are separate stages, and a thicker film
//!   releases its solvent more slowly). τ's numbers, and its linear growth
//!   with thickness, are ESTIMATES: no source gives them for oil paint and
//!   turpentine.
//! - **It makes the wet paint flow** (`spread_mm2_min`): while it is there
//!   the film levels, liquid running from higher to lower ground, carrying
//!   paint and solvent in the proportions they have where it starts. A
//!   surface-tension-driven film levels a ripple of wavelength λ at a rate
//!   of about σh³(2π/λ)⁴ / 3η (S. E. Orchard, "On surface levelling in
//!   viscous liquids and gels", Applied Scientific Research A 11, 451-464;
//!   the publisher dates it July 1963, https://doi.org/10.1007/BF03184629,
//!   while some secondary sources give 1962; notes/research/oil_paint_physics.md).
//!   The engine uses a diffusion of the wet surface with one mobility, the
//!   ESTIMATE `SPREAD_MM2_MIN` at thinner 0.5, slowed in a thin film as
//!   Orchard's h³ says (`thin_film`, `THIN_FILM_UM`). Matching Orchard's decay at one
//!   wavelength, D = σh³(2π/λ)² / 3η, with these inputs, all ESTIMATES
//!   (none measured for this paint): surface tension σ = 0.03 N/m, film
//!   h = 10 µm, viscosity η = 0.1 Pa·s (paint thinned half; the note gives
//!   1 Pa·s for a medium-rich glaze) and wavelength λ = 2 mm, gives about
//!   0.06 mm²/min. That is a wavelength-specific estimate, not a measured
//!   mobility. Unthinned paint doesn't flow here, as before (it levels when
//!   it sets, `drying`).
//!
//! Not modeled, on purpose: solvent evaporating from the brush or the
//! palette pile (the pile keeps its share), solvent soaking into the
//! ground, solvent dissolving set or dry paint underneath, and solvent-wet
//! paint coming up more readily on a brush or rag than the same paint
//! without it (extra pickup). The brush and the rag take solvent with the
//! paint they lift, in the film's own proportions.

/// One stroke of paint thinned half (`t` = 0.5) adds at most this much wet
/// film (µm) to a pixel. ESTIMATE, set before measuring; the card keeps 84%
/// (load 0.3) and 81% (load 0.6) of its contrast with it, and from 6 to 36
/// µm it kept 62-89% (notes/thinner/RESULTS.md, the sweep).
pub const STROKE_FILM_UM: f32 = 6.0;

/// The most wet film (paint + solvent, µm) one stroke may add to a pixel,
/// for paint holding the share `t` of solvent: `STROKE_FILM_UM` at 0.5,
/// growing without bound as `t` → 0 (`f32::INFINITY` at 0: no ceiling, the
/// paste law alone), shrinking as `t` → 1. Continuous: a hundredth of
/// thinner barely changes a stroke.
pub fn stroke_limit_um(t: f32) -> f32 {
    if t.is_nan() || t <= 0.0 {
        return f32::INFINITY;
    }
    let t = t.min(0.95);
    STROKE_FILM_UM * (1.0 - t) / t
}

/// Evaporation time (minutes) of the solvent in a film too thin to hold it
/// back. ESTIMATE ("a few minutes", Jennings 1902).
pub const TAU_MIN: f64 = 2.0;
/// The film thickness (µm of paint) that doubles it. ESTIMATE, with no
/// source for its size. First set at 20 µm; at that, thinned paint flowing
/// into the weave's hollows during the wait deepens the deepest film enough
/// (77 → 81 µm on the card) that the time the card test waits (ten times τ
/// after the pass) falls short of ten times τ at the end. 100 µm keeps the
/// dependence (a thicker film still holds its solvent longer) with less
/// sensitivity; notes/thinner/RESULTS.md.
pub const TAU_DOUBLING_UM: f64 = 100.0;

/// The solvent's evaporation time (minutes) at a pixel holding `paint_um`
/// of paint (solvent-free): over `dt` minutes, `exp(-dt / τ)` of it stays.
/// Positive, finite, never shorter for a thicker film.
pub fn evaporation_tau_min(paint_um: f32) -> f64 {
    let h = if paint_um.is_finite() { paint_um.max(0.0) as f64 } else { 0.0 };
    TAU_MIN * (1.0 + h / TAU_DOUBLING_UM)
}

/// Mobility (mm²/min) of a wet film half solvent: how fast it levels.
/// ESTIMATE (Orchard 1963, see the module notes).
pub const SPREAD_MM2_MIN: f32 = 0.06;
/// The liquid (paint + solvent, µm) at which a film keeps half its
/// mobility (`thin_film`). ESTIMATE, chosen, not measured: equal to the
/// hard floor it replaces (`WET_FILM_UM` until engine-3 Lane B, 2026-10-04),
/// so a film of 5 µm or more keeps at least 94% of the mobility it had.
pub const THIN_FILM_UM: f32 = 2.0;

/// The share of its mobility a film of `h_um` liquid (paint + solvent, µm)
/// keeps: h³ / (h³ + `THIN_FILM_UM`³). Orchard's leveling rate goes as the
/// film's thickness cubed (σh³/3η, module notes), so below `THIN_FILM_UM`
/// the film slows as h³ and the last of it barely moves, but no thickness
/// stops it outright (the hard floor this replaces stopped all outflow at or
/// below 2 µm of liquid, so one stroke at thinner 0.75 or more never
/// flowed: REVIEW_RESPONSE §3). Bounded: never above 1, so a thick film
/// keeps the mobility `SPREAD_MM2_MIN` was estimated for (at 10 µm, 0.992)
/// and the flow never runs faster than it did. The saturating form and its
/// half point are ESTIMATES; only the h³ of a thin film comes from Orchard.
#[inline]
pub fn thin_film(h_um: f32) -> f32 {
    let h3 = if h_um.is_finite() { h_um.max(0.0).powi(3) } else { 0.0 };
    h3 / (h3 + THIN_FILM_UM.powi(3))
}

/// Mobility of a film whose liquid holds the share `phi` of solvent: 0
/// without solvent, `SPREAD_MM2_MIN` at one half, more the thinner it is
/// (viscosity falls steeply with solvent). ESTIMATE.
pub fn spread_mm2_min(phi: f32) -> f32 {
    if phi.is_nan() || phi <= 0.0 {
        return 0.0;
    }
    let phi = phi.min(0.95);
    SPREAD_MM2_MIN * phi / (1.0 - phi)
}

// EXPERIMENT (AGENT_BRIEF_V2 §4c), per thread: false today (a thinned
// stroke adds at most `stroke_limit_um` of wet film to a pixel, per stroke
// id, and lifts at most its share of the film there once per stroke id);
// true exchange (no ceiling and no per-stroke allowance: the hair lays a
// share of what it carries and lifts a share of the wet film it touches,
// every segment, so what stays on the canvas follows the brush's actual
// supply).
thread_local! {
    static EXCHANGE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn set_exchange(on: bool) {
    EXCHANGE.with(|c| c.set(on));
}

pub fn exchange() -> bool {
    EXCHANGE.with(|c| c.get())
}

// EXPERIMENT, with the exchange on: a thinned hair lets go of its liquid
// at (1 - t)^k of the rate tube paint does (runnier paint leaves a thinner
// film behind a moving surface, Landau-Levich, qualitatively), and a
// brush dipped in thinned paint holds (1 - t)^j of the liquid (runny
// paint drains off the hairs). k and j are chosen estimates; 0 is off.
thread_local! {
    static EXCH_KJ: std::cell::Cell<(f32, f32)> = const { std::cell::Cell::new((0.0, 0.0)) };
}

pub fn set_exchange_kj(k: f32, j: f32) {
    EXCH_KJ.with(|c| c.set((k, j)));
}

pub fn exchange_kj() -> (f32, f32) {
    EXCH_KJ.with(|c| c.get())
}

/// Ticks a minute on the grid the solvent's loss and flow step on
/// (`Canvas::wait`): a chosen numerical resolution, not a physical
/// constant. A power of two, so quarter minutes fall on it exactly.
pub const FLOW_TICKS: u32 = 64;

/// Below this (µm) a pixel's solvent is gone. A chosen numerical cutoff
/// (it lets a wait end with the solvent gone), not a physical constant.
pub(crate) const SOLVENT_FLOOR: f32 = 1e-8;

use crate::canvas::Canvas;
use crate::surface::COAT_UM;
use crate::wet::{Latent, Prop};
use rayon::prelude::*;

/// Most substeps of the flow in one call of `spread`; past it the flow
/// is slowed to stay stable (a film that thin and that fine-grained flows a
/// little less far per minute than its mobility says). On the
/// 1/64-minute grid it binds only above
/// 819 dx² mm²/min (dx the pixel in mm): 27.5 at 2400 px across 440 mm,
/// 1.1 at 12000 px, against 1.14 at the most solvent. A chosen numerical cutoff, not
/// a physical constant, as are `MAX_OUT` and the explicit scheme's
/// stability factor 0.2 in `spread`.
const MAX_SUBSTEPS: usize = 64;
/// Most of a pixel's liquid that can leave it in one substep. A chosen
/// numerical cutoff.
const MAX_OUT: f32 = 0.5;

impl Canvas {
    /// `dt` minutes of the solvent's loss: each pixel keeps
    /// `exp(-dt / τ(h))` of it, h its paint.
    pub(crate) fn evaporate(&mut self, dt: f32) {
        if dt <= 0.0 || self.wet.solv.is_empty() {
            return;
        }
        let Some((x0, y0, x1, y1)) = self.wet.dirty else { return };
        let w = self.f.w;
        let (x1, y1) = (x1.min(w), y1.min(self.f.h));
        let wet = &mut self.wet;
        let vol = &wet.vol;
        wet.solv[y0 * w..y1 * w].par_chunks_mut(w).enumerate().for_each(|(j, row)| {
            for x in x0..x1 {
                let s = &mut row[x];
                if *s > 0.0 {
                    let h = vol[(y0 + j) * w + x] * COAT_UM;
                    let keep = (-(dt as f64) / evaporation_tau_min(h)).exp();
                    *s = (*s as f64 * keep) as f32;
                    if *s < SOLVENT_FLOOR {
                        *s = 0.0;
                    }
                }
            }
        });
    }

    /// Solvent-wet paint flows for `dt` minutes: the wet surface (relief +
    /// paint + solvent) levels by diffusion, each pixel's liquid running to
    /// lower neighbors at its own mobility (`spread_mm2_min` of its solvent
    /// share, slowed as its oil approaches the gel point, `drying::fluid`,
    /// and in a thin film, `thin_film`). What moves carries the paint
    /// (pigment, scattering, stiffness, drying rate, cure) and the solvent
    /// of the pixel it leaves, in their proportions there; paint without
    /// solvent doesn't flow out. Each substep computes every pixel's outflow
    /// from the state before it, then every pixel gathers its inflow: the
    /// result doesn't depend on the order pixels are visited in (or the
    /// threads). A pixel gives at most `MAX_OUT` of its liquid a substep, so
    /// none goes below zero; what one gives another gains.
    ///
    /// The substeps: the explicit scheme is stable while a substep moves at
    /// most 0.2 of a pixel's difference to a neighbor (mobility × minutes /
    /// pixel² ≤ 0.2). The largest mobility in the region sets that, and the
    /// flow changes it (a thin film that receives liquid thickens and speeds
    /// up; mixing changes the solvent share), so it is found afresh before
    /// every substep and the time left is split into as many equal substeps
    /// as it then needs. With an unchanging mobility that is the even split
    /// of `dt` the flow always used.
    ///
    /// The paint keeps the cure it had where it came from, mixed by amount,
    /// at any thickness (`Canvas::wait_on_grid` dries it over the whole
    /// step); a bare pixel it reaches takes the drying thickness of the
    /// films it came from too.
    pub(crate) fn spread(&mut self, dt: f32) {
        let Some((bx0, by0, bx1, by1)) = self.wet.dirty else { return };
        let (w, h) = (self.f.w, self.f.h);
        // engine 6: paint packed on an absorbent ground (`bristle::ground_drain`)
        // stays where it is; only the liquid above it flows, the paint in it
        // as rich as the film's surface (`bristle::surface_oil`)
        let drain = self.engine >= 6;
        let packed = |wet: &crate::wet::Wet, i: usize| if drain { wet.hide[i][7].clamp(0.0, 1.0) } else { 0.0 };
        let dx = self.px_mm();
        let timed = self.wet.clock.px.len() == w * h;
        // the mobility (mm²/min) of each pixel in the dirty box
        // (the canvas holds solvent in µm; the flow works in coats)
        // (of paint `v` and solvent `s`, coats, at cure `cure`)
        let mob_of = |v: f32, s: f32, cure: f32| -> f32 {
            if s <= 0.0 || v + s <= 0.0 {
                return 0.0;
            }
            let fl = if timed { crate::drying::fluid(cure) } else { 1.0 };
            spread_mm2_min(s / (v + s)) * fl * thin_film((v + s) * COAT_UM)
        };
        let mob = |wet: &crate::wet::Wet, i: usize| -> f32 { mob_of(wet.vol[i], wet.solv[i] / COAT_UM, if timed { wet.clock.px[i].cure } else { 0.0 }) };
        // the largest mobility, and the box of the pixels that have any
        // (only they can give liquid; each substep reaches one pixel further)
        let wet = &self.wet;
        type Acc = (f32, usize, usize, usize, usize);
        let none: Acc = (0.0, usize::MAX, usize::MAX, 0, 0);
        let join = |a: Acc, b: Acc| (a.0.max(b.0), a.1.min(b.1), a.2.min(b.2), a.3.max(b.3), a.4.max(b.4));
        let (m0, ax0, ay0, ax1, ay1) = (by0..by1.min(h))
            .into_par_iter()
            .map(|y| {
                (bx0..bx1.min(w)).fold(none, |a, x| {
                    let m = mob(wet, y * w + x);
                    if m > 0.0 { join(a, (m, x, y, x + 1, y + 1)) } else { a }
                })
            })
            .reduce(|| none, join);
        if m0 <= 0.0 || dt <= 0.0 {
            return;
        }
        let px2 = dx * dx;
        let (mut x0, mut y0, mut x1, mut y1) = (ax0, ay0, ax1, ay1);
        // the box of the pixels the flow changed
        let mut moved: Option<(usize, usize, usize, usize)> = None;
        let mut m_max = m0;
        let mut left = dt;
        let mut used = 0;
        while left > 0.0 && m_max > 0.0 && used < MAX_SUBSTEPS {
            // the substeps the time left needs at today's largest mobility,
            // each as long as the rest; if the cap would cut them short,
            // the longest stable substep (and the time past the cap is lost)
            let need = ((m_max * left / px2) / 0.2).ceil().max(1.0);
            let sub = if need > (MAX_SUBSTEPS - used) as f32 { 0.2 * px2 / m_max } else { left / need };
            // the flow's rate this substep, per mm²/min of mobility
            let k = sub / px2;
            // liquid can reach one pixel further each substep
            (x0, y0, x1, y1) = (x0.saturating_sub(1), y0.saturating_sub(1), (x1 + 1).min(w), (y1 + 1).min(h));
            let (rw, rh) = (x1 - x0, y1 - y0);
            let wet = &self.wet;
            let height = &self.height;
            // pass 1: each pixel's outflow (coats of liquid) to its four
            // neighbors (left, right, up, down)
            let out: Vec<[f32; 4]> = (0..rw * rh)
                .into_par_iter()
                .map(|k2| {
                    let (x, y) = (x0 + k2 % rw, y0 + k2 / rw);
                    let i = y * w + x;
                    let l = wet.vol[i] + wet.solv[i] / COAT_UM;
                    let m = mob(wet, i);
                    // (what can leave: the liquid above any packed paint)
                    let lm = l - wet.vol[i] * packed(wet, i);
                    if m <= 0.0 || l <= 0.0 || lm <= 0.0 {
                        return [0.0; 4];
                    }
                    let z = height[i] + l * COAT_UM;
                    // (within the region: every pixel that can receive is in it)
                    let nb = [(x > x0).then(|| i - 1), (x + 1 < x1).then(|| i + 1), (y > y0).then(|| i - w), (y + 1 < y1).then(|| i + w)];
                    let mut q = [0.0f32; 4];
                    let mut sum = 0.0f32;
                    for (d, j) in nb.iter().enumerate() {
                        if let Some(j) = *j {
                            let zj = height[j] + wet.vol[j] * COAT_UM + wet.solv[j];
                            if z > zj {
                                q[d] = m * k * (z - zj) / COAT_UM;
                                sum += q[d];
                            }
                        }
                    }
                    if sum > MAX_OUT * lm {
                        let f = MAX_OUT * lm / sum;
                        for v in &mut q {
                            *v *= f;
                        }
                    }
                    q
                })
                .collect();
            if out.iter().all(|q| q.iter().all(|&v| v <= 0.0)) {
                // level: the rest of the time moves nothing either
                break;
            }
            used += 1;
            left -= sub;
            // pass 2: each pixel keeps what didn't leave and gathers what came
            // in, each part with the paint and solvent of where it came from
            let cure_of = |i: usize| if timed { wet.clock.px[i].cure } else { 0.0 };
            let th_of = |i: usize| if timed { wet.clock.px[i].th } else { 0.0 };
            let at = |x: usize, y: usize| (y - y0) * rw + x - x0;
            // (and the mobility it then has, for the next substep's length,
            // and the drying thickness of the paint that came in)
            type Cell = (f32, f32, Latent, Prop, f32, bool, f32, f32);
            let next: Vec<Cell> = (0..rw * rh)
                .into_par_iter()
                .map(|k2| {
                    let (x, y) = (x0 + k2 % rw, y0 + k2 / rw);
                    let i = y * w + x;
                    let (v, s) = (wet.vol[i], wet.solv[i] / COAT_UM);
                    let l = v + s;
                    let gone: f32 = out[k2].iter().sum();
                    let keep = if l > 0.0 { ((l - gone) / l).max(0.0) } else { 0.0 };
                    let (mut pv, mut ps, mut lat, mut hide, mut cure) = (v * keep, s * keep, wet.lat[i], wet.hide[i], cure_of(i));
                    let pk = packed(wet, i);
                    if pk > 0.0 && gone > 0.0 {
                        // the liquid left from above the packed paint, which
                        // stays: the film's packed share grows, its oil falls
                        // by the surface paint's
                        let lm = l - v * pk;
                        let (pl, sl) = (gone * v * (1.0 - pk) / lm, gone * s / lm);
                        (pv, ps) = ((v - pl).max(0.0), (s - sl).max(0.0));
                        if pv > 1e-9 {
                            let top = crate::bristle::surface_oil(&hide);
                            hide[4] = ((hide[4] * v - top * pl) / pv).max(0.0);
                            hide[7] = (pk * v / pv).min(1.0);
                        }
                    }
                    let mut changed = gone > 0.0;
                    let (mut qin, mut thin) = (0.0f32, 0.0f32);
                    // from the left neighbor (its rightward flow), the right
                    // (leftward), above (downward), below (upward)
                    let from = [(x > 0 && x > x0).then(|| (x - 1, y, 1usize)), (x + 1 < x1).then(|| (x + 1, y, 0usize)), (y > 0 && y > y0).then(|| (x, y - 1, 3usize)), (y + 1 < y1).then(|| (x, y + 1, 2usize))];
                    for f in from.iter().flatten() {
                        let (fx, fy, d) = *f;
                        let q = out[at(fx, fy)][d];
                        if q <= 0.0 {
                            continue;
                        }
                        let j = fy * w + fx;
                        // (the liquid above any packed paint, which came)
                        let vj = wet.vol[j] * (1.0 - packed(wet, j));
                        let lj = vj + wet.solv[j] / COAT_UM;
                        if lj <= 0.0 {
                            continue;
                        }
                        let (qp, qs) = (q * vj / lj, q * (wet.solv[j] / COAT_UM) / lj);
                        if qp > 0.0 {
                            qin += qp;
                            thin += qp * th_of(j);
                            cure = if pv + qp > 0.0 { cure + (cure_of(j) - cure) * qp / (pv + qp) } else { cure };
                            let hj = wet.hide[j];
                            let hj = if drain { let mut h = crate::bristle::fluid_of(hj); h[4] = crate::bristle::surface_oil(&hj); h } else { hj };
                            crate::wet::mix_into(&mut pv, &mut lat, &mut hide, qp, &wet.lat[j], hj);
                        }
                        ps += qs;
                        changed = true;
                    }
                    let cure = if pv <= 0.0 { 0.0 } else { cure };
                    // (the drying thickness a bare pixel takes, by amount)
                    let own = th_of(i);
                    let th_in = if qin <= 0.0 { own } else if own > 0.0 && v * keep > 0.0 { (own * v * keep + thin) / (v * keep + qin) } else { thin / qin };
                    (pv, ps, lat, hide, cure, changed, mob_of(pv, ps, cure), th_in)
                })
                .collect();
            // the largest mobility now: every pixel that has any is in the
            // region (the next substep reaches one pixel further, where
            // nothing has any yet)
            m_max = next.par_iter().map(|c| c.6).reduce(|| 0.0, f32::max);
            // write back
            let px_ok = timed;
            for (k2, (pv, ps, lat, hide, cure, changed, _, th_in)) in next.into_iter().enumerate() {
                if !changed {
                    continue;
                }
                let (x, y) = (x0 + k2 % rw, y0 + k2 / rw);
                let i = y * w + x;
                moved = Some(match moved {
                    None => (x, y, x + 1, y + 1),
                    Some((a, b, c, d)) => (a.min(x), b.min(y), c.max(x + 1), d.max(y + 1)),
                });
                let was_bare = self.wet.vol[i] < 1e-5;
                self.wet.vol[i] = pv;
                self.wet.solv[i] = ps * COAT_UM;
                self.wet.lat[i] = lat;
                self.wet.hide[i] = hide;
                if was_bare && pv >= 1e-5 {
                    self.wet.cover[i] = 1.0;
                }
                if px_ok {
                    let p = &mut self.wet.clock.px[i];
                    p.cure = cure;
                    // the flow isn't a brush working the film: the film keeps
                    // its own drying. A bare pixel it reaches takes the
                    // drying thickness of the films it came from (when they
                    // have one; else it is judged at the next drying step,
                    // as a bare pixel's always was), so its drying doesn't
                    // depend on when that step falls (`wait_on_grid`)
                    if !was_bare {
                        p.seen = pv;
                    } else if th_in > 0.0 {
                        p.th = th_in;
                        if pv >= 1e-5 {
                            (p.seen, p.lev) = (pv, crate::surface::SET_TIME * crate::drying::fluid(cure));
                        }
                    }
                }
            }
        }
        if let Some((a, b, c, d)) = moved {
            self.wet.touch(a, b, c, d);
        }
    }

    /// Cure of the open film at a point (units): 0 fresh, `drying::GEL` at
    /// the gel point; 0 where there is no open film or it has never waited.
    pub fn cure_at(&self, x: f32, y: f32) -> f32 {
        let i = self.f.index(x, y);
        if self.wet.vol[i] < 1e-5 {
            return 0.0;
        }
        self.wet.clock.px.get(i).map_or(0.0, |p| p.cure)
    }
}

#[cfg(test)]
mod tests {
    use crate::bristle::{Gesture, Held, Tool, Touch};
    use crate::canvas::Canvas;
    use crate::color::hex;
    use crate::handling::Handling;
    use crate::mask::Mask;
    use crate::palette::Palette;
    use crate::stipple::Stipple;
    use crate::wet::Paint;

    fn canvas(engine: u32) -> Canvas {
        let mut c = Canvas::new(120, 1.0, hex("#d8cdb8")).with_engine(engine);
        c.prime(hex("#b9a98c"), 0.9, 40.0, 0.6, 0.0, 7);
        c
    }

    fn sienna(t: f32) -> Paint {
        let p = Paint::new(hex("#9a5a2a"), 0.85, 0.8);
        if t > 0.0 { p.with_thinner(t) } else { p }
    }

    fn thinned_brush(t: f32) -> Held {
        let mut h = Held::new(Tool::filbert(30.0), 3);
        h.load(sienna(t), 0.6);
        h
    }

    fn total(c: &Canvas, f: impl Fn(&Canvas, f32, f32) -> f32) -> f64 {
        let fr = c.frame();
        (0..fr.w * fr.h).map(|i| f(c, fr.ux(i % fr.w), fr.uy(i / fr.w)) as f64).sum()
    }

    /// Thinner is engine 3 only: before it the film has no solvent, so every
    /// way of putting a thinned brush or pass on such a canvas panics before
    /// drawing, rather than letting the solvent vanish.
    #[test]
    fn thinned_paint_is_refused_before_engine_3() {
        type Use = fn(&mut Canvas);
        let uses: [(&str, Use); 4] = [
            ("Canvas::drag", |c| c.drag(&mut thinned_brush(0.5), &Gesture::new(vec![(200.0, 500.0), (800.0, 500.0)]), None)),
            ("Canvas::touch", |c| c.touch(&mut thinned_brush(0.5), &Touch::at(500.0, 500.0), None)),
            ("Canvas::work", |c| {
                let (m, pal) = (Mask::full(c.frame()), Palette::named_box("inness").unwrap());
                c.work(&m, &Handling::new(Tool::filbert(30.0)).piled(&pal, pal.pile(vec![(0, 1.0)]), 0.0).thinner(0.5), 1)
            }),
            ("Canvas::stipple", |c| {
                let (m, pal) = (Mask::full(c.frame()), Palette::named_box("inness").unwrap());
                c.stipple(&m, &Stipple::new(Tool::stippler(20.0)).piled(&pal, pal.pile(vec![(0, 1.0)]), 0.0).thinner(0.5), 1)
            }),
        ];
        for (name, f) in uses {
            for engine in [1, 2] {
                let mut c = canvas(engine);
                let before = c.seen();
                let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut c)));
                let msg = r.expect_err(name);
                let msg = msg.downcast_ref::<String>().cloned().unwrap_or_default();
                assert!(msg.contains(name) && msg.contains("needs engine 3"), "{name}, engine {engine}: {msg}");
                assert!(c.seen() == before, "{name}, engine {engine}: drew before refusing");
            }
            // engine 3 takes it
            let mut c = canvas(3);
            f(&mut c);
            assert!(total(&c, Canvas::solvent_um) > 0.0, "{name}: engine 3 laid no solvent");
        }
    }

    /// `dry()` on a canvas with solvent in its paint is the same as waiting
    /// on the minute grid until the solvent has gone, then drying: the film
    /// flows and loses its solvent first, it isn't baked as it lies.
    #[test]
    fn dry_with_solvent_waits_it_out_first() {
        let mut a = canvas(3);
        for k in 0..6 {
            let y = 300.0 + 80.0 * k as f32;
            a.drag(&mut thinned_brush(0.6), &Gesture::new(vec![(150.0, y), (850.0, y + 30.0)]), None);
        }
        assert!(total(&a, Canvas::solvent_um) > 0.0);
        let mut b = a.clone();
        a.dry();
        let mut minutes = 0;
        while total(&b, Canvas::solvent_um) > 0.0 {
            b.wait(1.0);
            minutes += 1;
            assert!(minutes < 10_000, "the solvent never went");
        }
        b.dry();
        assert!(a.seen() == b.seen(), "dry() differs from waiting the solvent out, then drying");
        assert_eq!(a.clock(), b.clock());
    }

    /// A stroke's ceiling counts only that stroke's own film: when the
    /// stroke ids wrap and a new stroke gets an id an old one had, it still
    /// lays about as much as a stroke under a fresh id. (Not exactly as
    /// much: the reused id also marks the old stroke's film as the new
    /// one's own in `Wet::stroke`, which predates the thinner; it costs
    /// about 5% here. Before the ceilings were forgotten on a wrap, the
    /// second stroke found its ceiling used up and took paint away: −83
    /// against 318.)
    #[test]
    fn a_reused_stroke_id_starts_a_fresh_ceiling() {
        let lay = |wrap: bool| -> f64 {
            let mut c = canvas(3);
            let g = Gesture::new(vec![(200.0, 500.0), (800.0, 520.0)]);
            c.drag(&mut thinned_brush(0.5), &g, None);
            // `current` is one past the last id handed out: the stroke was 1
            let first = c.wet.current;
            assert_eq!(first, 2, "the first stroke's id is 1");
            if wrap {
                // the next stroke's id wraps round to the first one's
                c.wet.current = u32::MAX;
            }
            let before = total(&c, Canvas::wet_um);
            c.drag(&mut thinned_brush(0.5), &g, None);
            if wrap {
                assert_eq!(c.wet.current, first, "the second stroke's id wrapped round to 1, the first one's");
            }
            total(&c, Canvas::wet_um) - before
        };
        let (fresh, reused) = (lay(false), lay(true));
        assert!(fresh > 0.0);
        assert!(reused >= 0.9 * fresh, "a reused id laid {reused}, a fresh one {fresh}");
    }

    /// HANDOVER 6.2 (1): equal 0.02-minute waits from different places on
    /// the clock should move about the same paint. Prints the share of the
    /// paint each wait moves (half the summed |change| over the total).
    /// `cargo test --release -p paint --lib thinner::tests::short_waits -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn short_waits() {
        let mut c = Canvas::new(480, 1.0, hex("#d8cdb8")).with_engine(3);
        c.prime(hex("#b9a98c"), 0.9, 40.0, 0.6, 0.0, 7);
        for k in 0..6 {
            let y = 300.0 + 25.0 * k as f32;
            let mut h = Held::new(Tool::hog_flat(40.0), 10 + k);
            h.load(sienna(0.5), 0.9);
            c.drag(&mut h, &Gesture::new(vec![(250.0, y), (750.0, y + 3.0)]).pressure(0.85, 0.85), None);
        }
        let c0 = c;
        for phase in [0.10f32, 0.50, 0.97, 0.99] {
            let mut c = c0.clone();
            let to = (c.clock().floor() + 1.0) as f32 + phase - c.clock() as f32;
            c.wait(to);
            let before = c.wet.vol.clone();
            c.wait(0.02);
            let moved: f64 = c.wet.vol.iter().zip(&before).map(|(a, b)| (a - b).abs() as f64).sum::<f64>() / 2.0;
            let all: f64 = before.iter().map(|&v| v as f64).sum();
            println!("0.02 min from phase {phase:.2}: moved {:.4}% of the paint", 100.0 * moved / all);
        }
    }


    /// How long a wait with solvent takes (wall clock): a thinned patch at
    /// 2400 px, then `wait(30)` and `wait(240)`.
    /// `cargo test --release -p paint --lib thinner::tests::wait_cost -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn wait_cost() {
        let mut c = Canvas::new(2400, 1.5, hex("#d8cdb8")).with_engine(3);
        c.prime(hex("#b9a98c"), 0.9, 40.0, 0.6, 0.0, 7);
        for k in 0..12 {
            let y = 150.0 + 25.0 * k as f32;
            let mut h = Held::new(Tool::hog_flat(40.0), 10 + k);
            h.load(sienna(0.5), 0.9);
            c.drag(&mut h, &Gesture::new(vec![(150.0, y), (850.0, y + 3.0)]).pressure(0.85, 0.85), None);
        }
        for m in [30.0f32, 240.0] {
            let mut d = c.clone();
            let t = std::time::Instant::now();
            d.wait(m);
            println!("wait({m}): {:.2} s", t.elapsed().as_secs_f64());
        }
    }


    /// HANDOVER 6.2 (3): one stroke at thinner 0.5, 0.75 and 0.9 on a flat
    /// ground, then five minutes: the share of its paint the flow moves.
    /// `cargo test --release -p paint --lib thinner::tests::single_strokes -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn single_strokes() {
        for t in [0.5f32, 0.75, 0.9] {
            let mut c = Canvas::new(480, 1.0, hex("#d8cdb8")).with_engine(3);
            c.prime(hex("#b9a98c"), 0.9, 40.0, 0.6, 0.0, 7);
            let mut h = Held::new(Tool::hog_flat(40.0), 10);
            h.load(sienna(t), 0.9);
            c.drag(&mut h, &Gesture::new(vec![(250.0, 500.0), (750.0, 503.0)]).pressure(0.85, 0.85), None);
            let before = c.wet.vol.clone();
            let liquid: Vec<f32> = (0..before.len()).map(|i| before[i] * crate::surface::COAT_UM + c.wet.solv.get(i).copied().unwrap_or(0.0)).filter(|&l| l > 0.01).collect();
            let mut l = liquid.clone();
            l.sort_by(f32::total_cmp);
            c.wait(5.0);
            let moved: f64 = c.wet.vol.iter().zip(&before).map(|(a, b)| (a - b).abs() as f64).sum::<f64>() / 2.0;
            let all: f64 = before.iter().map(|&v| v as f64).sum();
            println!("thinner {t}: wet film median {:.2} µm, p90 {:.2}; 5 min moved {:.4}% of the paint", l[l.len() / 2], l[l.len() * 9 / 10], 100.0 * moved / all);
        }
    }


    /// Pictures for HANDOVER 6.2 (3): raw umber strokes at thinner 0.5, 0.75
    /// and 0.9 and a broad thinned wash, each after 5 minutes, saved to
    /// `THIN_EXP_DIR` with the prefix `THIN_EXP_TAG`.
    #[test]
    #[ignore]
    fn flow_pictures() {
        use crate::canvas::Crop;
        let pal = Palette::named_box("inness").unwrap();
        let umber = pal.pile(vec![(pal.tubes.iter().position(|t| t.name == "raw umber").unwrap(), 1.0)]).laid(0.0);
        let d = std::path::PathBuf::from(std::env::var("THIN_EXP_DIR").unwrap());
        let tag = std::env::var("THIN_EXP_TAG").unwrap();
        let mut c = Canvas::new_window(2400, 1.5, hex("#d8cdb8"), Some(Crop { units: [260.0, 190.0, 740.0, 490.0], margin: 30.0 })).with_engine(3).with_size_mm(440.0);
        c.prime(hex("#e4dcc8"), 0.9, 40.0, 0.6, 0.0, 7);
        for (k, t) in [0.5f32, 0.75, 0.9].into_iter().enumerate() {
            let y = 230.0 + 45.0 * k as f32;
            let mut h = Held::new(Tool::hog_flat(40.0), 10 + k as u64);
            h.load(umber.with_thinner(t), 0.9);
            c.drag(&mut h, &Gesture::new(vec![(290.0, y), (710.0, y + 3.0)]).pressure(0.85, 0.85), None);
        }
        for k in 0..5 {
            let y = 380.0 + 22.0 * k as f32;
            let mut h = Held::new(Tool::hog_flat(40.0), 50 + k);
            h.load(umber.with_thinner(0.5), 0.9);
            c.drag(&mut h, &Gesture::new(vec![(290.0, y), (710.0, y + 3.0)]).pressure(0.85, 0.85), None);
        }
        c.wait(5.0);
        c.save(d.join(format!("{tag}.png"))).unwrap();
    }


    /// Check 9 under the flow without `FLOW_MIN`: for every pixel-minute
    /// check 9 examines, how much of the miss against exp(-1/τ) comes from
    /// the flow (solvent in/out) and how much from evaporation alone.
    /// `cargo test --release -p paint --lib thinner::tests::c09_probe -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn c09_probe() {
        use super::evaporation_tau_min;
        let pal = Palette::named_box("inness").unwrap();
        let i = pal.tubes.iter().position(|t| t.name == "raw sienna").unwrap();
        let rs = pal.pile(vec![(i, 1.0)]).laid(0.0).with_thinner(0.5);
        let mut c = Canvas::new(300, 1.0, hex("#d8cdb8")).with_engine(3);
        c.prime(hex("#b9a98c"), 0.9, 60.0, 0.6, 0.0, 7);
        let patch = |c: &mut Canvas, x: (f32, f32), seed: u64| {
            for k in 0..((700.0f32 - 300.0) / 25.0).ceil() as usize {
                let yk = 300.0 + 400.0 * (k as f32 + 0.5) / 16.0;
                let mut h = Held::new(Tool::hog_flat(40.0), seed + k as u64);
                h.load(rs, 0.9);
                c.drag(&mut h, &Gesture::new(vec![(x.0, yk), (x.1, yk + 3.0)]).pressure(0.85, 0.85), None);
            }
        };
        patch(&mut c, (100.0, 450.0), 41);
        for k in 0..5 {
            patch(&mut c, (550.0, 900.0), 141 + 20 * k);
        }
        let f = c.frame();
        let (w, hh) = (f.w, f.h);
        let ids: Vec<usize> = (0..w * hh)
            .filter(|&i| {
                let (x, y) = (f.ux(i % w), f.uy(i / w));
                (x >= 150.0 && x < 400.0 || x >= 600.0 && x < 850.0) && y >= 350.0 && y < 650.0
            })
            .collect();
        let pu = |c: &Canvas| -> Vec<f32> { ids.iter().map(|&i| c.wet.vol[i] * super::COAT_UM).collect() };
        let su = |c: &Canvas| -> Vec<f32> { ids.iter().map(|&i| c.wet.solv[i]).collect() };
        let (mut examined, mut fails) = (0usize, 0usize);
        let mut worst = (0.0f64, 0usize, 0usize, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
        let mut flow_abs = Vec::new();
        let eps = [1e-3f64, 3e-3, 1e-2, 3e-2];
        let mut sel = vec![(0usize, 0usize, [0usize; 2]); eps.len()];
        for m in 1..=40 {
            let (ph, ps) = (pu(&c), su(&c));
            let ph_all: Vec<f32> = c.wet.vol.iter().map(|v| v * super::COAT_UM).collect();
            let ps_all = c.wet.solv.clone();
            // per pixel: product of evaporation-only keeps; net solvent the flow moved
            let mut keep = vec![1.0f64; ids.len()];
            let mut flow = vec![0.0f64; ids.len()];
            for _ in 0..super::FLOW_TICKS {
                let s0 = su(&c);
                let mut d = c.clone();
                d.evaporate(1.0 / super::FLOW_TICKS as f32);
                let se = su(&d);
                c.wait(1.0 / super::FLOW_TICKS as f32);
                let s1 = su(&c);
                for k in 0..ids.len() {
                    if s0[k] > 0.0 {
                        keep[k] *= se[k] as f64 / s0[k] as f64;
                    }
                    flow[k] += (s1[k] - se[k]) as f64;
                }
            }
            let (h1, s1) = (pu(&c), su(&c));
            for k in 0..ids.len() {
                let (a, b) = (ps[k] as f64, s1[k] as f64);
                let d = ((h1[k] - ph[k]) / ph[k].max(1e-6)).abs() as f64;
                if ph[k] < 1.0 || d > 1e-3 || a < 0.01 {
                    continue;
                }
                // the 4-neighborhood's solvent ÷ paint at the minute's start
                let r = |j: usize| ps[j] as f64 / (ph[j] as f64).max(1e-9);
                let (x, y) = (ids[k] % w, ids[k] / w);
                let mut spread = 0.0f64;
                for (dx, dy) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1)] {
                    let j = ((y as i64 + dy) as usize) * w + (x as i64 + dx) as usize;
                    let jr = ps_all[j] as f64 / (ph_all[j] as f64).max(1e-9);
                    spread = spread.max((jr - r(k)).abs() / r(k));
                }
                for (e, cnt) in eps.iter().zip(sel.iter_mut()) {
                    if spread <= *e {
                        cnt.0 += 1;
                        if (b / a - (-1.0f64 / evaporation_tau_min(ph[k])).exp()).abs() > 1e-4 + 2.0 * d {
                            cnt.1 += 1;
                        }
                        cnt.2[if f.ux(ids[k] % w) < 500.0 { 0 } else { 1 }] += 1;
                    }
                }
                examined += 1;
                let want = (-1.0f64 / evaporation_tau_min(ph[k])).exp();
                let got = b / a;
                flow_abs.push((flow[k] / a).abs());
                let miss = (got - want).abs() - (1e-4 + 2.0 * d);
                if miss > 0.0 {
                    fails += 1;
                }
                if miss > worst.0 || worst.1 == 0 {
                    worst = (miss, m, ids[k], got - want, keep[k] - want, flow[k] / a, d);
                }
            }
        }
        for (e, cnt) in eps.iter().zip(&sel) {
            println!("neighbors' ratio within {e:.0e}: {} pixel-minutes (thin {}, thick {}), {} over", cnt.0, cnt.2[0], cnt.2[1], cnt.1);
        }
        flow_abs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let q = |p: f64| flow_abs[((flow_abs.len() - 1) as f64 * p) as usize];
        println!("pixel-minutes examined {examined}, over check 9's tolerance {fails}");
        println!("flow's net solvent / solvent at minute start, |.|: median {:.2e}, p99 {:.2e}, max {:.2e}", q(0.5), q(0.99), q(1.0));
        println!("worst: margin {:.2e} at minute {}, pixel {}: got-law {:.3e} = evaporation-only {:.3e} + flow {:.3e} (paint change d {:.2e}, allowed {:.2e})",
            worst.0, worst.1, worst.2, worst.3, worst.4, worst.5, worst.6, 1e-4 + 2.0 * worst.6);
    }

    /// AGENT_BRIEF_V2 §4c: today's rule against the exchange experiment
    /// (`set_exchange`), raw umber thinned 0.5, on linen and on a smooth
    /// ground. Scenes: one stroke; a broad pass (8 overlapping strokes,
    /// each freshly loaded); three such passes wet; three with the paint
    /// dried between; one load scrubbed 4 times over a band (lifted between,
    /// no reload); one exact stroke against the same stroke in two calls.
    /// Prints paint (µm) mean and relative spread in the middle, solvent,
    /// and the brush's paint left, right after and after 5 minutes. With
    /// `EXCH_DIR`, saves each canvas as PNG.
    /// `EXCH_DIR=/tmp/x cargo test --release -p paint --lib thinner::tests::exchange_scenes -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn exchange_scenes() {
        use crate::Linen;
        let pal = Palette::named_box("inness").unwrap();
        let umber = pal.pile(vec![(pal.tubes.iter().position(|t| t.name == "raw umber").unwrap(), 1.0)]).laid(0.0).with_thinner(0.5);
        let out = std::env::var("EXCH_DIR").ok().map(std::path::PathBuf::from);
        let base = |linen: bool| {
            let mut c = Canvas::new(1200, 0.7, hex("#d8cdb8")).with_engine(3);
            if linen {
                c = c.with_linen(Linen::fine(3));
            }
            c.prime(hex("#e4dcc8"), 0.9, 60.0, 0.6, if linen { 0.2 } else { 0.0 }, 7);
            c
        };
        let g = |pts: Vec<(f32, f32)>| Gesture::new(pts).pressure(0.85, 0.85);
        let pass = |c: &mut Canvas, seed: u64| {
            for k in 0..8 {
                let y = 250.0 + 25.0 * k as f32;
                let mut h = Held::new(Tool::hog_flat(40.0), seed + k);
                h.load(umber, 0.9);
                c.drag(&mut h, &g(vec![(250.0, y), (750.0, y + 3.0)]), None);
            }
        };
        // (paint µm mean, relative spread, solvent µm mean) in a rectangle
        let stats = |c: &Canvas, r: (f32, f32, f32, f32)| {
            let f = c.f;
            let (mut n, mut sp, mut sp2, mut ss) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
            for i in 0..f.w * f.h {
                let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
                if x >= r.0 && x < r.2 && y >= r.1 && y < r.3 {
                    let p = (c.wet.vol[i] * super::COAT_UM) as f64;
                    n += 1.0;
                    sp += p;
                    sp2 += p * p;
                    ss += c.wet.solv[i] as f64;
                }
            }
            let m = sp / n;
            (m, (sp2 / n - m * m).max(0.0).sqrt() / m.max(1e-9), ss / n)
        };
        let mid = (350.0, 280.0, 650.0, 420.0);
        for linen in [true, false] {
            let sets: Vec<(bool, f32, f32)> = std::env::var("EXCH_SETS").ok().map_or(vec![(false, 0.0, 0.0), (true, 0.0, 0.0)], |v| {
                v.split(';').map(|t| if t == "today" { (false, 0.0, 0.0) } else { let (k, j) = t.split_once(',').unwrap(); (true, k.parse().unwrap(), j.parse().unwrap()) }).collect()
            });
            if !linen && std::env::var("EXCH_LINEN_ONLY").is_ok() {
                continue;
            }
            for (ex, kk, jj) in sets {
                super::set_exchange(ex);
                super::set_exchange_kj(kk, jj);
                let tag = format!("{}-{}", if linen { "linen" } else { "smooth" }, if ex { format!("k{kk}j{jj}") } else { "today".into() });
                let mut report = |name: &str, c: &mut Canvas, r: (f32, f32, f32, f32), brush: Option<f64>| {
                    let a = stats(c, r);
                    let mut d = c.clone();
                    d.wait(5.0);
                    let b = stats(&d, r);
                    println!(
                        "{tag:>16} {name:<14} paint {:6.2} µm (spread {:.2}) solvent {:5.2} | after 5 min paint {:6.2} (spread {:.2}) solvent {:5.2}{}",
                        a.0, a.1, a.2, b.0, b.1, b.2,
                        brush.map_or(String::new(), |p| format!(" | brush paint left {:.0}%", 100.0 * p))
                    );
                    if let Some(o) = &out {
                        std::fs::create_dir_all(o).unwrap();
                        c.save(o.join(format!("{tag}-{}.png", name.replace(' ', "_")))).unwrap();
                    }
                };
                // one stroke
                let mut c = base(linen);
                let mut h = Held::new(Tool::hog_flat(40.0), 5);
                h.load(umber, 0.9);
                let p0 = h.carried().0;
                c.drag(&mut h, &g(vec![(250.0, 350.0), (750.0, 353.0)]), None);
                report("1 stroke", &mut c, (350.0, 340.0, 650.0, 362.0), Some(h.carried().0 / p0));
                // a broad pass, three wet, three dried between
                for (name, n, dry) in [("broad pass", 1, false), ("3 passes wet", 3, false), ("3 passes dry", 3, true)] {
                    let mut c = base(linen);
                    for p in 0..n {
                        if p > 0 && dry {
                            c.dry();
                        }
                        pass(&mut c, 100 * p as u64 + 1);
                    }
                    report(name, &mut c, mid, None);
                }
                // one load scrubbed back and forth 4 times
                let mut c = base(linen);
                let mut h = Held::new(Tool::hog_flat(40.0), 9);
                h.load(umber, 0.9);
                let p0 = h.carried().0;
                for k in 0..4 {
                    let (a, b) = if k % 2 == 0 { (250.0, 750.0) } else { (750.0, 250.0) };
                    c.drag(&mut h, &g(vec![(a, 350.0), (b, 352.0)]), None);
                    if k == 0 || k == 3 {
                        report(&format!("scrub {}", k + 1), &mut c, (350.0, 340.0, 650.0, 362.0), Some(h.carried().0 / p0));
                    }
                }
                // one exact stroke, whole and in two calls
                let exact = |pts: Vec<(f32, f32)>| {
                    let mut g = g(pts);
                    (g.attack, g.release, g.shake) = (0.0, 0.0, 0.0);
                    g
                };
                let mut whole = base(linen);
                let mut h = Held::new(Tool::hog_flat(40.0), 13);
                h.load(umber, 0.9);
                whole.drag(&mut h, &exact(vec![(250.0, 350.0), (750.0, 350.0)]), None);
                let mut split = base(linen);
                let mut h2 = Held::new(Tool::hog_flat(40.0), 13);
                h2.load(umber, 0.9);
                split.drag(&mut h2, &exact(vec![(250.0, 350.0), (500.0, 350.0)]), None);
                split.drag(&mut h2, &exact(vec![(500.0, 350.0), (750.0, 350.0)]), None);
                let (a, b) = (stats(&whole, (350.0, 340.0, 650.0, 362.0)), stats(&split, (350.0, 340.0, 650.0, 362.0)));
                let maxd = whole.wet.vol.iter().zip(&split.wet.vol).map(|(x, y)| ((x - y) * super::COAT_UM).abs()).fold(0.0f32, f32::max);
                println!("{tag:>16} whole/2 calls  paint {:.2} / {:.2} µm, largest pixel difference {:.2} µm", a.0, b.0, maxd);
            }
        }
        super::set_exchange(false);
        super::set_exchange_kj(0.0, 0.0);
    }

    /// Side checks for `exchange_scenes`: linen against smooth at light
    /// pressure; where a stroke in two calls differs from it whole.
    #[test]
    #[ignore]
    fn exchange_side() {
        use crate::Linen;
        let pal = Palette::named_box("inness").unwrap();
        let umber = pal.pile(vec![(pal.tubes.iter().position(|t| t.name == "raw umber").unwrap(), 1.0)]).laid(0.0).with_thinner(0.5);
        let base = |linen: bool| {
            let mut c = Canvas::new(1200, 0.7, hex("#d8cdb8")).with_engine(3);
            if linen {
                c = c.with_linen(Linen::fine(3));
            }
            c.prime(hex("#e4dcc8"), 0.9, 60.0, 0.6, if linen { 0.2 } else { 0.0 }, 7);
            c
        };
        for p in [0.85f32, 0.5, 0.3] {
            for load in [0.9f32, 0.3] {
                let mut m = [0.0f64; 2];
                for (k, linen) in [true, false].into_iter().enumerate() {
                    let mut c = base(linen);
                    let mut h = Held::new(Tool::hog_flat(40.0), 5);
                    h.load(umber, load);
                    c.drag(&mut h, &Gesture::new(vec![(250.0, 350.0), (750.0, 353.0)]).pressure(p, p), None);
                    m[k] = c.wet.vol.iter().map(|v| *v as f64).sum::<f64>() * super::COAT_UM as f64;
                }
                println!("pressure {p} load {load}: paint laid linen {:.0} smooth {:.0} (µm·px)", m[0], m[1]);
            }
        }
        for ex in [false, true] {
            super::set_exchange(ex);
            let exact = |pts: Vec<(f32, f32)>| {
                let mut g = Gesture::new(pts).pressure(0.85, 0.85);
                (g.attack, g.release, g.shake) = (0.0, 0.0, 0.0);
                g
            };
            let mut whole = base(true);
            let mut h = Held::new(Tool::hog_flat(40.0), 13);
            h.load(umber, 0.9);
            whole.drag(&mut h, &exact(vec![(250.0, 350.0), (750.0, 350.0)]), None);
            let mut split = base(true);
            let mut h2 = Held::new(Tool::hog_flat(40.0), 13);
            h2.load(umber, 0.9);
            split.drag(&mut h2, &exact(vec![(250.0, 350.0), (500.0, 350.0)]), None);
            split.drag(&mut h2, &exact(vec![(500.0, 350.0), (750.0, 350.0)]), None);
            // the same line in one call with 51 points
            let mut dense = base(true);
            let mut h3 = Held::new(Tool::hog_flat(40.0), 13);
            h3.load(umber, 0.9);
            dense.drag(&mut h3, &exact((0..=50).map(|k| (250.0 + 10.0 * k as f32, 350.0)).collect()), None);
            let dd = whole.wet.vol.iter().zip(&dense.wet.vol).map(|(x, y)| ((x - y) * super::COAT_UM).abs()).fold(0.0f32, f32::max);
            println!("exchange {ex}: 2 points against 51 in one call: largest difference {dd:.3} µm");
            let f = whole.f;
            // the difference by x band (units), largest and mean
            let mut bands = vec![(0.0f32, 0.0f64, 0usize); 20];
            for i in 0..f.w * f.h {
                let d = ((whole.wet.vol[i] - split.wet.vol[i]) * super::COAT_UM).abs();
                let b = ((f.ux(i % f.w) / 50.0) as usize).min(19);
                bands[b].0 = bands[b].0.max(d);
                bands[b].1 += d as f64;
                bands[b].2 += 1;
            }
            let row: Vec<String> = bands.iter().enumerate().filter(|(_, b)| b.0 > 0.0).map(|(k, b)| format!("x{}:{:.1}", k * 50, b.0)).collect();
            println!("exchange {ex}: largest difference by 50-unit band: {}", row.join(" "));
        }
        super::set_exchange(false);
    }

    /// Fingerprint of the thinner's state after thinned strokes and waits
    /// (for checking that a refactor leaves results bit-identical).
    #[test]
    #[ignore]
    fn fingerprint() {
        fn h(acc: &mut u64, v: &[f32]) {
            for x in v {
                for b in x.to_bits().to_le_bytes() {
                    *acc = (*acc ^ b as u64).wrapping_mul(0x100000001b3);
                }
            }
        }
        let mut c = Canvas::new(480, 1.0, hex("#d8cdb8")).with_engine(3);
        c.prime(hex("#b9a98c"), 0.9, 40.0, 0.6, 0.0, 7);
        for (k, t) in [0.5f32, 0.75, 0.9, 0.95].into_iter().enumerate() {
            let y = 200.0 + 60.0 * k as f32;
            let mut hd = Held::new(Tool::hog_flat(40.0), 10 + k as u64);
            hd.load(sienna(t), 0.9);
            hd.load(sienna(t), 0.9);
            c.drag(&mut hd, &Gesture::new(vec![(250.0, y), (750.0, y + 3.0)]).pressure(0.85, 0.85), None);
        }
        for k in 0..6 {
            let y = 520.0 + 20.0 * k as f32;
            let mut hd = Held::new(Tool::hog_flat(40.0), 40 + k);
            hd.load(sienna(0.5), 0.9);
            c.drag(&mut hd, &Gesture::new(vec![(200.0, y), (800.0, y + 3.0)]).pressure(0.85, 0.85), None);
        }
        for (step, m) in [0.02f32, 0.37, 1.0, 3.0, 0.02, 10.0, 30.0].into_iter().enumerate() {
            c.wait(m);
            if step == 3 {
                let mut hd = Held::new(Tool::hog_flat(30.0), 99);
                hd.load(sienna(0.6), 0.9);
                c.drag(&mut hd, &Gesture::new(vec![(300.0, 150.0), (320.0, 700.0)]).pressure(0.8, 0.8), None);
            }
            let mut acc = 0xcbf29ce484222325u64;
            h(&mut acc, &c.wet.vol);
            h(&mut acc, &c.wet.solv);
            h(&mut acc, &c.film);
            println!("FP step {step} wait {m}: {acc:016x}");
        }
    }

}
