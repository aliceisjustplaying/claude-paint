//! Thinner: turpentine or mineral spirits knifed into a pile.
//!
//! A solvent thins oil paint without adding oil (that is `medium`). The
//! painter's records: Hopper began "with almost pure turpentine" as the
//! medium (notes/research/hopper_materials.md, [MORSE]); Sargent's lay-in
//! was "thinned with a little turpentine" (sargent_materials.md, [CH
//! pp.181-182]); Inness rubbed in transparent paint "thinned with a
//! vehicle—turpentine and Siccatif" (inness_materials.md, [MAN p.32]); a
//! turpentine-bearing medium "does also speed up drying a good bit"
//! (tonn_materials.md, [MED21; MED25]).
//!
//! What the solvent does, mapped onto what paint already carries (no new
//! canvas state: a thinned brushload is a `Paint` like any other):
//!
//! - **A lean, transparent film.** A share `t` of the wet paint is solvent,
//!   which evaporates within minutes and leaves pigment and oil behind. So a
//!   coat of thinned paint, as the brush lays it, holds `1 - t` of the
//!   pigment of a coat of tube paint: its Kubelka–Munk absorption and
//!   scattering per coat fall by `1 - t`, as medium's do (two-constant KM:
//!   K and S per coat scale with pigment concentration). The same load
//!   covers `1 / (1 - t)` times the area to the same depth of color.
//! - **Fluid.** The solvent lowers the paint's viscosity and yield stress,
//!   so its bristle ridges and stroke ridges level (oil_paint_physics.md §1:
//!   striations survive only above a yield stress of ~100 Pa; "with solvent
//!   evaporation, moderate yield stress can actually reduce residual
//!   unevenness" [5][S]; bob_ross.md: "thinner lowers yield stress"). The
//!   stiffness falls by `(1 - t)²`, as medium's does (estimate).
//! - **The oil cures as the pile's own oil.** The solvent flashes off in
//!   minutes; the oil left behind oxidizes as the unthinned pile's oil does
//!   (wash films stay workable while laid and set in an hour or hours, not
//!   minutes). So the solvent's fluidity doesn't count as fat: the drying
//!   rate is raised by exactly what `drying::rate`'s fat term takes off for
//!   the lower stiffness, and no more. A thinned film dries faster only
//!   because it is thinner (it spreads further, below): the engine's own
//!   thickness law (`drying::rate`: time ∝ film^0.7, no faster below about
//!   0.37 coats) sets how much, so the speed-up is a factor on the engine's
//!   drying constants, at most 2× against one coat. The flash-off itself
//!   (a matte, stiffer film within minutes) isn't modeled: no canvas state.
//!
//! - **Spread further.** Fluid paint is spread into a thinner film: a brush
//!   loaded with it lays less per unit of travel and one load runs
//!   `1 / (1 - t)` as far (`bristle::run_of`, an estimate). The solvent
//!   share rides on the brush (`Paint::thinner`, mixed by volume in each
//!   bristle), not on the canvas: paint a brush lifts off the canvas holds
//!   none. Touches (`stipple`) lay a set film per load and don't spread.
//!
//! What it doesn't do (v1): the wet film on the canvas keeps the volume the
//! brush laid (solvent included) rather than shrinking as the solvent
//! leaves; the drying rate stands in for the thinner film (above).

use crate::drying::rate;
use crate::wet::Paint;

/// The most solvent a pile can hold: "almost pure turpentine" (Hopper)
/// still has pigment in it.
pub const MAX: f32 = 0.9;

/// `paint` thinned with the share `t` (0..`MAX`) of solvent. `t` 0 returns
/// it unchanged, bit for bit.
pub fn thin(paint: Paint, t: f32) -> Paint {
    if !(t > 0.0) {
        return paint;
    }
    let k = 1.0 - t.min(MAX);
    let stiff = paint.stiff * k * k;
    // the oil cures as the pile's own: undo the fat term's slowdown for the
    // solvent's fluidity (the thickness term cancels at any film)
    let drying = paint.drying * rate(1.0, paint.stiff, 1.0) / rate(1.0, stiff, 1.0);
    Paint { scatter: paint.scatter * k, stiff, drying, thinner: t.min(MAX), ..paint }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// No thinner is no change, to the bit.
    #[test]
    fn zero_is_identity() {
        let p = Paint::new([0.6, 0.5, 0.3], 0.82, 0.8).with_drying(2.0);
        for t in [0.0, -0.1, f32::NAN] {
            let q = thin(p, t);
            assert_eq!(
                [q.color[0], q.color[1], q.color[2], q.scatter, q.stiff, q.drying].map(f32::to_bits),
                [p.color[0], p.color[1], p.color[2], p.scatter, p.stiff, p.drying].map(f32::to_bits)
            );
        }
    }

    /// Thinned paint scatters less per coat, is more fluid and dries faster;
    /// its masstone stays. More thinner, more of each.
    #[test]
    fn thinner_thins() {
        let p = Paint::new([0.6, 0.5, 0.3], 0.82, 0.8).with_drying(2.0);
        let (a, b) = (thin(p, 0.3), thin(p, 0.6));
        assert_eq!(a.color, p.color);
        assert!(p.scatter > a.scatter && a.scatter > b.scatter);
        assert!(p.stiff > a.stiff && a.stiff > b.stiff);
        // a film of thinned paint cures as a film of the pile's own paint
        // as thick: the solvent's fluidity isn't fat
        let t = |q: Paint, h: f32| 1.0 / rate(h, q.stiff, q.drying);
        for h in [0.3, 1.0, 3.0] {
            assert!((t(a, h) / t(p, h) - 1.0).abs() < 1e-5 && (t(b, h) / t(p, h) - 1.0).abs() < 1e-5, "{h}: {} {} {}", t(p, h), t(a, h), t(b, h));
        }
        // optically, a coat of it is 1 - t coats of the pile's paint
        let under = [0.2, 0.25, 0.3];
        let (x, y) = (a.over(under, 1.0), p.over(under, 0.7));
        assert!((0..3).all(|i| (x[i] - y[i]).abs() < 1e-5), "{x:?} vs {y:?}");
        // and capped
        assert_eq!(thin(p, 0.95).scatter, thin(p, MAX).scatter);
    }
}
