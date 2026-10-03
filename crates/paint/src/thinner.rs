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
//! What the solvent does, in this version (no new canvas state):
//!
//! - **It flashes off as the paint is laid.** A brush loaded from a thinned
//!   pile holds the wash, solvent and all (its bristles carry the share `t`,
//!   `Paint::thinner`), but a stroke lays only the paint that is left once
//!   the solvent has gone: of every volume the brush gives up, `1 - t`
//!   reaches the canvas as the pile's own paint, and the rest evaporates
//!   (`bristle::exchange`). Less pigment per area, a thinner, leaner film.
//!   There is no wet stage with solvent in it: a real wash stays fluid for
//!   some minutes and goes matte as the solvent leaves, which this version
//!   doesn't model.
//! - **Spread further.** Fluid paint is brushed out further: a bristle
//!   carrying the share `t` of solvent runs `1 / (1 - t)` as far per load
//!   (`bristle::run_of`, an estimate).
//! - **Dries sooner because it is thinner.** The film on the canvas is the
//!   pile's own paint (its stiffness and drying rate), so the oil cures as
//!   the pile's does, and a thinner film dries sooner through the engine's
//!   thickness law (`drying::rate`: time ∝ film^0.7, no faster below about
//!   0.37 coats).
//!
//! Paint a brush lifts off the canvas holds no solvent: the canvas holds
//! none.

use crate::wet::Paint;

/// The most solvent a pile can hold: "almost pure turpentine" (Hopper)
/// still has pigment in it.
pub const MAX: f32 = 0.9;

/// `paint` with the share `t` (0..`MAX`) of solvent: the same paint, which
/// a brush lays `1 - t` of (`bristle::exchange`). `t` 0 returns it
/// unchanged, bit for bit.
pub fn thin(paint: Paint, t: f32) -> Paint {
    if !(t > 0.0) {
        return paint;
    }
    Paint { thinner: t.min(MAX), ..paint }
}

/// How much of a volume of `paint` is left once its solvent has gone.
#[inline]
pub fn left(paint: &Paint) -> f32 {
    1.0 - paint.thinner
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
                [q.color[0], q.color[1], q.color[2], q.scatter, q.stiff, q.drying, q.thinner].map(f32::to_bits),
                [p.color[0], p.color[1], p.color[2], p.scatter, p.stiff, p.drying, p.thinner].map(f32::to_bits)
            );
        }
    }

    /// Thinned paint is the pile's own paint carrying its solvent share
    /// (capped); a volume of it leaves `1 - t` of paint.
    #[test]
    fn thinned_paint_is_the_piles_paint_and_its_solvent() {
        let p = Paint::new([0.6, 0.5, 0.3], 0.82, 0.8).with_drying(2.0);
        let a = thin(p, 0.3);
        assert_eq!((a.color, a.scatter, a.stiff, a.drying), (p.color, p.scatter, p.stiff, p.drying));
        assert_eq!((a.thinner, left(&a)), (0.3, 0.7));
        assert_eq!(thin(p, 0.95).thinner, MAX);
        assert_eq!(left(&p), 1.0);
    }
}
