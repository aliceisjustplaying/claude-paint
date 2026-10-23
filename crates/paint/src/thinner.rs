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

type FlowCell = (f32, f32, crate::wet::Latent, crate::wet::Prop, f32, bool, f32, f32, f32, Option<crate::oil::exchange::MaterialSink>);

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
            let before = *s;
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
            if *s < before && let Some(material) = &mut self.wet.lat[i].material {
                let volume = self.wet.vol[i] as f64;
                material.scale(volume);
                let removed = material.remove_solvent((before - *s) as f64);
                material.scale(if volume > 0.0 { 1.0 / volume } else { 0.0 });
                assert_eq!(self.wet.material_sinks.len(), self.wet.vol.len(), "chemical evaporation requires a sink ledger");
                self.wet.material_sinks[i].push(crate::oil::exchange::MaterialSink { kind: crate::oil::exchange::SinkKind::Evaporated, material: removed });
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
        // the drain switch: paint packed on an absorbent ground
        // (`bristle::ground_drain`) stays where it is; only the liquid above
        // it flows, the paint in it as rich as the film's surface
        // (`bristle::surface_oil`)
        let drain = self.drain;
        let packed = |wet: &crate::wet::Wet, i: usize| if drain { wet.hide[i][7].clamp(0.0, 1.0) } else { 0.0 };
        // and paint the flow brings to a pixel is laid there as a brush lays
        // it (`Surf::add`): fluid, and drained by the ground under it while
        // its pores take oil, from that pixel's absorbency
        let soaks = drain && self.engine >= 4 && self.absorb_any;
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
        // (with the drain switch, of the liquid above any packed paint, which
        // doesn't flow; the sparse set below is built from it)
        let mob = |wet: &crate::wet::Wet, i: usize| -> f32 { mob_of(wet.vol[i] * (1.0 - packed(wet, i)), wet.solv[i] / COAT_UM, if timed { wet.clock.px[i].cure } else { 0.0 }) };
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
            let absorb = &self.absorb;
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
                .collect_into_vec(&mut out);
            if out.iter().all(|q| q.iter().all(|&v| v <= 0.0)) {
                // level: the rest of the time moves nothing either
                break;
            }
            let material_flow: Vec<_> = if wet.material_sinks.is_empty() { Vec::new() } else {
                (0..len).map(|k2| {
                    let i = index(k2);
                    let material = wet.lat[i].material.as_ref()?;
                    let v = wet.vol[i];
                    let flowing = v * (1.0 - packed(wet, i));
                    let liquid = flowing + wet.solv[i] / COAT_UM;
                    let gone = if liquid > 0.0 { out[k2].iter().sum::<f32>() * flowing / liquid } else { 0.0 };
                    let (mut below, mut top) = material.split_top_with_geometry(if v > 0.0 { gone as f64 / v as f64 } else { 0.0 }, v as f64 * COAT_UM as f64);
                    below.scale(if v > gone { v as f64 / (v - gone) as f64 } else { 0.0 });
                    top.scale(if gone > 0.0 { v as f64 / gone as f64 } else { 0.0 });
                    Some((below, top))
                }).collect()
            };
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
                    let (mut pv, mut ps, mut lat, mut hide, mut cure) = (v * keep, s * keep, wet.lat[i].clone(), wet.hide[i], cure_of(i));
                    if let Some(Some((below, _))) = material_flow.get(k2) { lat.material = Some(below.clone()); }
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
                        let Some(k) = at(fx, fy) else { continue };
                        let q = out[k][d];
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
                            let mut incoming;
                            let source = if let Some(Some((_, top))) = material_flow.get(k) {
                                incoming = wet.lat[j].clone();
                                incoming.material = Some(top.clone());
                                incoming.layers.clear();
                                &incoming
                            } else { &wet.lat[j] };
                            crate::wet::mix_into(&mut pv, &mut lat, &mut hide, qp, source, hj);
                        }
                        ps += qs;
                        changed = true;
                    }
                    // (the drain switch: the ground here drains the film the
                    // paint came into, as `Surf::add` does; what is left of
                    // its pores goes back with the rest)
                    let mut cap = if soaks { absorb[i] } else { 0.0 };
                    let mut sink = None;
                    if soaks && qin > 0.0 {
                        let before = pv;
                        let packed = hide[7];
                        pv = crate::bristle::ground_drain(&mut cap, before, &mut hide);
                        if pv < before { sink = lat.drain_material(before, pv, packed, hide[7]); }
                    }
                    let cure = if pv <= 0.0 { 0.0 } else { cure };
                    // (the drying thickness a bare pixel takes, by amount)
                    let own = th_of(i);
                    let th_in = if qin <= 0.0 { own } else if own > 0.0 && v * keep > 0.0 { (own * v * keep + thin) / (v * keep + qin) } else { thin / qin };
                    let flowing = if drain { pv * (1.0 - hide[7].clamp(0.0, 1.0)) } else { pv };
                    (pv, ps, lat, hide, cure, changed, mob_of(flowing, ps, cure), th_in, cap, sink)
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
            for (k2, (pv, ps, lat, hide, cure, changed, _, th_in, cap, sink)) in next.iter().cloned().enumerate() {
                if !changed {
                    continue;
                }
                let i = index(k2);
                if !self.wet.material_changed.is_empty() { self.wet.material_changed[i] = 1; }
                if let Some(sink) = sink {
                    assert_eq!(self.wet.material_sinks.len(), self.wet.vol.len(), "chemical flow requires a sink ledger");
                    self.wet.material_sinks[i].push(sink);
                }
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
                self.wet.lat[i].layers.clear();
                self.wet.hide[i] = hide;
                if soaks {
                    self.absorb[i] = cap;
                }
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
                            (p.seen, p.lev) = (pv, if self.engine >= 9 { 0.0 } else { crate::surface::SET_TIME * crate::drying::fluid(cure) });
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
        if self.material_exchange_needed() { return self.material_store().unwrap().sample(i).expect("material pixel").network; }
        if self.wet.vol[i] < 1e-5 {
            return 0.0;
        }
        self.wet.clock.px.get(i).map_or(0.0, |p| p.cure)
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::color::hex;
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

    /// With the drain switch the flow's sparse set (`spread`'s `active` and
    /// its mobilities, built from `mob`) reads each film's packed share: two
    /// lone spots of paint packed through, under solvent, give up only their
    /// solvent, which still flows (they are in the set); half packed, they
    /// keep their packed paint and what leaves is the liquid above it, leaner
    /// in paint than an unpacked film's (and, thinner, it flows faster);
    /// without the switch the packed share is not read. (Two spots 60 px apart: the set is sparse, 2 of a box
    /// some 60 × 3 px.)
    #[test]
    fn the_sparse_flow_set_moves_only_the_liquid_above_packed_paint() {
        let run = |drain: bool, pk: f32| {
            let mut c = canvas(5).with_drain(drain);
            c.wet.ensure_solvent();
            let w = c.f.w;
            let y = c.f.h / 2;
            let lat = sienna(0.0).latent();
            let spots = [y * w + 30, y * w + 90];
            for &i in &spots {
                c.wet.vol[i] = 2.0;
                c.wet.solv[i] = super::COAT_UM;
                c.wet.lat[i] = lat.clone();
                c.wet.hide[i] = [0.8, 0.6, 1.0, 0.0, 1.0, 0.7, 0.6, pk, 0.5, 0.0];
                c.wet.stroke[i] = 1;
            }
            let mut solvent: Vec<usize> = spots.to_vec();
            c.spread(10.0, &mut solvent);
            let moved: f64 = spots.iter().flat_map(|&i| [i - 1, i + 1, i - w, i + w]).map(|j| c.wet.vol[j] as f64).sum();
            let solv: f64 = spots.iter().flat_map(|&i| [i - 1, i + 1, i - w, i + w]).map(|j| c.wet.solv[j] as f64).sum();
            (c.wet.vol.clone(), spots.map(|i| c.wet.vol[i]), moved, solv)
        };
        let (_, kept, moved, solv) = run(true, 1.0);
        assert!(kept == [2.0, 2.0] && moved == 0.0 && solv > 0.0, "packed through: only the solvent leaves ({kept:?} {moved} {solv})");
        let (fluid, half) = (run(true, 0.0), run(true, 0.5));
        assert!(fluid.2 > 0.0 && half.2 > 0.0 && half.1.iter().all(|&v| v >= 1.0 - 1e-6), "half packed keeps its packed paint: {:?}", half.1);
        assert!(half.2 / half.3 < fluid.2 / fluid.3, "the liquid above packed paint is leaner: {} < {}", half.2 / half.3, fluid.2 / fluid.3);
        crate::tests::image_tolerance::assert_fields_close(run(false, 1.0).0, fluid.0);
    }

    /// With the drain switch, thinned paint that flows off a film whose
    /// ground is saturated onto bare absorbent ground is laid there as a
    /// brush lays it (`Surf::add`): the ground drains it (`ground_drain`),
    /// so its packed share rises and the ground's absorbency there falls by
    /// the oil it took, and the paint is thinner by that oil. Without the
    /// switch the flow doesn't read the absorbency: it moves the paint as on
    /// an oil ground and leaves the absorbency as it was; with the switch on
    /// an oil ground it flows as without it.
    #[test]
    fn paint_the_flow_brings_onto_absorbent_ground_drains_there() {
        let run = |drain: bool, absorbent: f32| {
            let mut c = canvas(5).with_drain(drain);
            c.ground_finish(absorbent);
            c.wet.ensure_solvent();
            let (w, y) = (c.f.w, c.f.h / 2);
            let lat = sienna(0.0).latent();
            let src = y * w + 60;
            // (thick paint under as much solvent: the ground under it took
            // all it could)
            c.wet.vol[src] = 3.0;
            c.wet.solv[src] = 3.0 * super::COAT_UM;
            c.wet.lat[src] = lat;
            c.wet.hide[src] = [0.8, 0.0, 1.0, 0.0, 0.9, 0.7, 0.6, 0.0, 0.5, 0.0];
            c.wet.stroke[src] = 1;
            c.absorb[src] = 0.0;
            let mut solvent = vec![src];
            c.spread(10.0, &mut solvent);
            let near: Vec<usize> = [src - 1, src + 1, src - w, src + w].into();
            (c.wet.vol.clone(), c.wet.solv.clone(), c.wet.hide.clone(), c.absorb.clone(), near)
        };
        let full = crate::canvas::ABSORB_COATS;
        let (vol, _, hide, absorb, near) = run(true, 1.0);
        for &j in &near {
            assert!(vol[j] > 0.0 && hide[j][7] > 0.5 && absorb[j] < full - 1e-4, "drained where it came: {} coats packed {} absorbency {}", vol[j], hide[j][7], absorb[j]);
            assert!(hide[j][4] < 0.9, "leaner: {:?}", hide[j]);
        }
        let (off, oil) = (run(false, 1.0), run(false, 0.0));
        crate::tests::image_tolerance::assert_fields_close(off.0.iter().copied(), oil.0.iter().copied());
        crate::tests::image_tolerance::assert_fields_close(off.1.iter().copied(), oil.1.iter().copied());
        crate::tests::image_tolerance::assert_fields_close(off.2.iter().flatten().copied(), oil.2.iter().flatten().copied());
        assert!(off.3.iter().filter(|&&a| a != full).count() == 1, "nor changed (but under the source, set saturated)");
        for &j in &near {
            assert!(off.2[j][7] == 0.0 && vol[j] < off.0[j], "unpacked, and thicker by the oil: {} > {}", off.0[j], vol[j]);
        }
        let on_oil = run(true, 0.0);
        crate::tests::image_tolerance::assert_fields_close(on_oil.0, oil.0);
        crate::tests::image_tolerance::assert_fields_close(on_oil.1, oil.1);
        crate::tests::image_tolerance::assert_fields_close(on_oil.2.iter().flatten().copied(), oil.2.iter().flatten().copied());
    }

    #[test]
    fn drained_flow_conserves_integrated_scattering() {
        let mut c = canvas(5).with_drain(true);
        c.ground_finish(1.0);
        c.wet.ensure_solvent();
        let src = (c.f.h / 2) * c.f.w + c.f.w / 2;
        c.wet.vol[src] = 3.0;
        c.wet.solv[src] = 3.0 * super::COAT_UM;
        c.wet.lat[src] = sienna(0.0).latent();
        c.wet.hide[src] = [0.8, 0.0, 1.0, 0.0, 0.9, 0.7, 0.6, 0.0, 0.5, 0.0];
        c.wet.stroke[src] = 1;
        c.absorb[src] = 0.0;
        let mut solvent = vec![src];
        c.spread(10.0, &mut solvent);
        let mass: f64 = c.wet.vol.iter().zip(&c.wet.hide).map(|(&v, h)| (v * h[0]) as f64).sum();
        let volume: f32 = c.wet.vol.iter().sum();
        assert!(volume < 3.0 - 1e-4);
        assert!((mass - 2.4).abs() < 1e-6, "2.4 -> {mass}");
    }
}
