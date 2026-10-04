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
//!   42-54, 1942). Turpentine cuts oil paint's viscosity by orders of
//!   magnitude. The size of the ceiling is an ESTIMATE, set so that raw
//!   sienna thinned half lays an imprimatura (notes/thinner/RESULTS.md);
//!   it fades out as thinner goes to 0, where today's paint path runs
//!   unchanged.
//! - **It evaporates** (`evaporation_tau_min`): at a fixed paint thickness
//!   the same share leaves in each equal time step, solvent = s0 ×
//!   exp(-t / τ). Turpentine on paper evaporates "in a few minutes" (W. J.
//!   Pearce [Jennings], Paint & Colour Mixing, 1902, "To Test the Purity of
//!   Turpentine", https://www.gutenberg.org/cache/epub/56738/pg56738-images.html);
//!   in a paint film the solvent left in it has to diffuse out through the
//!   film, so a thicker film holds it longer (C. M. Hansen, The Three
//!   Dimensional Solubility Parameter and Solvent Diffusion Coefficient,
//!   Danish Technical Press, 1967, on solvent retention in coatings). τ's
//!   numbers are ESTIMATES: no source gives one for oil paint and
//!   turpentine.
//! - **It makes the wet paint flow** (`spread_mm2_min`): while it is there
//!   the film levels, liquid running from higher to lower ground, carrying
//!   paint and solvent in the proportions they have where it starts. A
//!   surface-tension-driven film levels a ripple of wavelength λ at a rate
//!   of about σh³(2π/λ)⁴ / 3η (S. E. Orchard, "On surface levelling in
//!   viscous liquids and gels", Applied Scientific Research A 11, 451-464,
//!   1962; notes/research/oil_paint_physics.md). The engine uses a
//!   diffusion of the wet surface with one mobility, the ESTIMATE
//!   `SPREAD_MM2_MIN` at thinner 0.5, that leaves a wetting film
//!   (`WET_FILM_UM`) where it runs off: σ = 0.03 N/m, h = 10 µm, η = 0.1 Pa·s
//!   (paint thinned half; the note gives 1 Pa·s for a medium-rich glaze)
//!   and λ = 2 mm give 0.06 mm²/min. Unthinned paint doesn't flow here, as
//!   before (it levels when it sets, `drying`).
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
/// ESTIMATE (Orchard 1962, see the module notes).
pub const SPREAD_MM2_MIN: f32 = 0.06;
/// The flow doesn't drain a pixel below this much liquid (µm): a liquid
/// that wets the paint under it leaves a film on the weave's tops, it
/// doesn't run off them bare (Orchard's leveling rate goes as the film's
/// thickness cubed, so the last of a film barely moves). ESTIMATE.
pub const WET_FILM_UM: f32 = 2.0;

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

/// Below this (µm) a pixel's solvent is gone.
pub(crate) const SOLVENT_FLOOR: f32 = 1e-8;

use crate::canvas::Canvas;
use crate::surface::COAT_UM;
use crate::wet::{Latent, Prop};
use rayon::prelude::*;

/// Most substeps of the flow in one step of the clock; past it the flow
/// is slowed to stay stable (a film that thin and that fine-grained flows a
/// little less far per minute than its mobility says).
const MAX_SUBSTEPS: usize = 64;
/// Most of a pixel's liquid that can leave it in one substep.
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
    /// share, slowed as its oil approaches the gel point, `drying::fluid`).
    /// What moves carries the paint (pigment, scattering, stiffness, drying
    /// rate, cure) and the solvent of the pixel it leaves, in their
    /// proportions there; paint without solvent doesn't flow out. Each
    /// substep computes every pixel's outflow from the state before it,
    /// then every pixel gathers its inflow: the result doesn't depend on
    /// the order pixels are visited in (or the threads).
    pub(crate) fn spread(&mut self, dt: f32) {
        let Some((bx0, by0, bx1, by1)) = self.wet.dirty else { return };
        let (w, h) = (self.f.w, self.f.h);
        let dx = self.px_mm();
        let timed = self.wet.clock.px.len() == w * h;
        // the mobility (mm²/min) of each pixel in the dirty box
        // (the canvas holds solvent in µm; the flow works in coats)
        let mob = |wet: &crate::wet::Wet, i: usize| -> f32 {
            let (v, s) = (wet.vol[i], wet.solv[i] / COAT_UM);
            if s <= 0.0 || v + s <= 0.0 {
                return 0.0;
            }
            let fl = if timed { crate::drying::fluid(wet.clock.px[i].cure) } else { 1.0 };
            spread_mm2_min(s / (v + s)) * fl
        };
        let mut m_max = 0.0f32;
        for y in by0..by1.min(h) {
            for x in bx0..bx1.min(w) {
                m_max = m_max.max(mob(&self.wet, y * w + x));
            }
        }
        if m_max <= 0.0 {
            return;
        }
        let r_total = m_max * dt / (dx * dx);
        let n = ((r_total / 0.2).ceil() as usize).clamp(1, MAX_SUBSTEPS);
        // the flow's rate per substep, per mm²/min of mobility
        let k = (dt / n as f32 / (dx * dx)).min(0.2 / m_max.max(1e-12));
        let (mut x0, mut y0, mut x1, mut y1) = (bx0, by0, bx1.min(w), by1.min(h));
        for _ in 0..n {
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
                    if m <= 0.0 || l <= 0.0 {
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
                    // (down to the wetting film, no further)
                    let avail = (l - WET_FILM_UM / COAT_UM).max(0.0);
                    if sum > MAX_OUT * avail {
                        let f = if sum > 0.0 { MAX_OUT * avail / sum } else { 0.0 };
                        for v in &mut q {
                            *v *= f;
                        }
                    }
                    q
                })
                .collect();
            if out.iter().all(|q| q.iter().all(|&v| v <= 0.0)) {
                break;
            }
            // pass 2: each pixel keeps what didn't leave and gathers what came
            // in, each part with the paint and solvent of where it came from
            let cure_of = |i: usize| if timed { wet.clock.px[i].cure } else { 0.0 };
            let at = |x: usize, y: usize| (y - y0) * rw + x - x0;
            type Cell = (f32, f32, Latent, Prop, f32, bool);
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
                    let mut changed = gone > 0.0;
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
                        let lj = wet.vol[j] + wet.solv[j] / COAT_UM;
                        if lj <= 0.0 {
                            continue;
                        }
                        let (qp, qs) = (q * wet.vol[j] / lj, q * (wet.solv[j] / COAT_UM) / lj);
                        if qp > 0.0 {
                            cure = if pv + qp > 0.0 { cure + (cure_of(j) - cure) * qp / (pv + qp) } else { cure };
                            crate::wet::mix_into(&mut pv, &mut lat, &mut hide, qp, &wet.lat[j], wet.hide[j]);
                        }
                        ps += qs;
                        changed = true;
                    }
                    (pv, ps, lat, hide, cure, changed)
                })
                .collect();
            // write back
            let px_ok = timed;
            for (k2, (pv, ps, lat, hide, cure, changed)) in next.into_iter().enumerate() {
                if !changed {
                    continue;
                }
                let (x, y) = (x0 + k2 % rw, y0 + k2 / rw);
                let i = y * w + x;
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
                    p.cure = if pv < 1e-5 { 0.0 } else { cure };
                    // the flow isn't a brush working the film: the film keeps
                    // its own drying (its thickness is judged afresh only if
                    // it was bare)
                    if !was_bare {
                        p.seen = pv;
                    }
                }
            }
        }
        self.wet.touch(x0, y0, x1, y1);
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
