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

type FlowCell = (f32, f32, crate::wet::Latent, crate::wet::Prop, f32, bool, f32, f32);

#[derive(Default)]
struct FlowScratch {
    active: Vec<usize>,
    indices: Vec<usize>,
    out: Vec<[f32; 4]>,
    next: Vec<FlowCell>,
}

thread_local! {
    static FLOW_SCRATCH: std::cell::RefCell<FlowScratch> = std::cell::RefCell::new(FlowScratch::default());
}

/// Below this (µm) a pixel's solvent is gone. A chosen numerical cutoff
/// (it lets a wait end with the solvent gone), not a physical constant.
pub(crate) const SOLVENT_FLOOR: f32 = 1e-8;

use crate::canvas::Canvas;
use crate::surface::COAT_UM;
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
    /// Maximum current flow mobility in the solvent work set.
    pub(crate) fn peak_mobility(&self, solvent: &[usize]) -> f32 {
        solvent.par_iter().map(|&i| {
            let (h, s) = (self.wet.vol[i] * COAT_UM, self.wet.solv[i]);
            if s <= 0.0 || h + s <= 0.0 { return 0.0; }
            let fl = self.wet.clock.px.get(i).map_or(1.0, |p| crate::drying::fluid(p.cure));
            spread_mm2_min(s / (h + s)) * fl * thin_film(h + s)
        }).reduce(|| 0.0, f32::max)
    }

    /// The wait owns this work set: evaporation removes entries and flow
    /// adds receivers. It is rebuilt at every wait, so brushwork and saves
    /// need no cache invalidation or checkpoint-format change.
    pub(crate) fn evaporate_indices(&mut self, dt: f32, solvent: &mut Vec<usize>) {
        solvent.retain(|&i| {
            let s = &mut self.wet.solv[i];
            if *s > 0.0 {
                let h = self.wet.vol[i] * COAT_UM;
                *s = (*s as f64 * (-(dt as f64) / evaporation_tau_min(h)).exp()) as f32;
                // Engine 5: a trace below ten parts per million of the
                // local film is numerically gone. The 2 µm term keeps
                // the threshold finite on bare weave. A numerical estimate,
                // not a measured evaporation law; earlier engines keep 1e-8.
                let floor = if self.engine >= 5 { SOLVENT_FLOOR.max(1e-5 * (h + THIN_FILM_UM)) } else { SOLVENT_FLOOR };
                if *s < floor { *s = 0.0; }
            }
            *s > 0.0
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
    pub(crate) fn spread(&mut self, dt: f32, solvent: &mut Vec<usize>) {
        if solvent.is_empty() { return; }
        let (w, h) = (self.f.w, self.f.h);
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
        let (m0, ax0, ay0, ax1, ay1) = solvent.par_iter().map(|&i| {
            let m = mob(wet, i);
            let (x, y) = (i % w, i / w);
            if m > 0.0 { (m, x, y, x + 1, y + 1) } else { none }
        }).reduce(|| none, join);
        if m0 <= 0.0 || dt <= 0.0 {
            return;
        }
        let px2 = dx * dx;
        let (mut x0, mut y0, mut x1, mut y1) = (ax0, ay0, ax1, ay1);
        // the box of the pixels the flow changed
        let mut moved: Option<(usize, usize, usize, usize)> = None;
        let original_len = solvent.len();
        let FlowScratch { mut active, mut indices, mut out, mut next } = FLOW_SCRATCH.with(|s| std::mem::take(&mut *s.borrow_mut()));
        active.clear();
        active.extend(solvent.iter().copied().filter(|&i| mob(wet, i) > 0.0));
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
            let sparse = active.len() < rw * rh / 8;
            indices.clear();
            if sparse {
                for &i in &active {
                    let (x, y) = (i % w, i / w);
                    indices.push(i);
                    if x > 0 { indices.push(i - 1); }
                    if x + 1 < w { indices.push(i + 1); }
                    if y > 0 { indices.push(i - w); }
                    if y + 1 < h { indices.push(i + w); }
                }
                indices.sort_unstable();
                indices.dedup();
            }
            let len = if sparse { indices.len() } else { rw * rh };
            let index = |k: usize| if sparse { indices[k] } else { (y0 + k / rw) * w + x0 + k % rw };
            // pass 1: each pixel's outflow (coats of liquid) to its four
            // neighbors (left, right, up, down)
            (0..len)
                .into_par_iter()
                .map(|k2| {
                    let i = index(k2);
                    let (x, y) = (i % w, i / w);
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
                    if sum > MAX_OUT * l {
                        let f = MAX_OUT * l / sum;
                        for v in &mut q {
                            *v *= f;
                        }
                    }
                    q
                })
                .collect_into_vec(&mut out);
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
            let at = |x: usize, y: usize| if sparse { indices.binary_search(&(y * w + x)).ok() } else { Some((y - y0) * rw + x - x0) };
            // (and the mobility it then has, for the next substep's length,
            // and the drying thickness of the paint that came in)
            (0..len)
                .into_par_iter()
                .map(|k2| {
                    let i = index(k2);
                    let (x, y) = (i % w, i / w);
                    let (v, s) = (wet.vol[i], wet.solv[i] / COAT_UM);
                    let l = v + s;
                    let gone: f32 = out[k2].iter().sum();
                    let keep = if l > 0.0 { ((l - gone) / l).max(0.0) } else { 0.0 };
                    let (mut pv, mut ps, mut lat, mut hide, mut cure) = (v * keep, s * keep, wet.lat[i], wet.hide[i], cure_of(i));
                    let mut changed = gone > 0.0;
                    let (mut qin, mut thin) = (0.0f32, 0.0f32);
                    // from the left neighbor (its rightward flow), the right
                    // (leftward), above (downward), below (upward)
                    let from = [(x > 0 && x > x0).then(|| (x - 1, y, 1usize)), (x + 1 < x1).then(|| (x + 1, y, 0usize)), (y > 0 && y > y0).then(|| (x, y - 1, 3usize)), (y + 1 < y1).then(|| (x, y + 1, 2usize))];
                    for f in from.iter().flatten() {
                        let (fx, fy, d) = *f;
                        let Some(k) = at(fx, fy) else { continue };
                        let q = out[k][d];
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
                            qin += qp;
                            thin += qp * th_of(j);
                            cure = if pv + qp > 0.0 { cure + (cure_of(j) - cure) * qp / (pv + qp) } else { cure };
                            crate::wet::mix_into(&mut pv, &mut lat, &mut hide, qp, &wet.lat[j], wet.hide[j]);
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
                .collect_into_vec(&mut next);
            // the largest mobility now: every pixel that has any is in the
            // region (the next substep reaches one pixel further, where
            // nothing has any yet)
            m_max = next.par_iter().map(|c| c.6).reduce(|| 0.0, f32::max);
            active.clear();
            active.extend(next.iter().enumerate().filter_map(|(k, c)| (c.6 > 0.0).then(|| index(k))));
            // write back
            let px_ok = timed;
            for (k2, (pv, ps, lat, hide, cure, changed, _, th_in)) in next.iter().copied().enumerate() {
                if !changed {
                    continue;
                }
                let i = index(k2);
                let (x, y) = (i % w, i / w);
                moved = Some(match moved {
                    None => (x, y, x + 1, y + 1),
                    Some((a, b, c, d)) => (a.min(x), b.min(y), c.max(x + 1), d.max(y + 1)),
                });
                let was_bare = self.wet.vol[i] < 1e-5;
                self.wet.vol[i] = pv;
                if ps > 0.0 && self.wet.solv[i] <= 0.0 { solvent.push(i); }
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
        if solvent.len() != original_len {
            solvent.sort_unstable();
            solvent.dedup();
        }
        solvent.retain(|&i| self.wet.solv[i] > 0.0);
        if let Some((a, b, c, d)) = moved {
            self.wet.touch(a, b, c, d);
        }
        FLOW_SCRATCH.with(|s| *s.borrow_mut() = FlowScratch { active, indices, out, next });
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
