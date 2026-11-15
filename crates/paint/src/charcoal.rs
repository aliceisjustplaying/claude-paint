//! Charcoal: vine (willow) and compressed sticks, as pastel sticks
//! (`crate::pastel`) with their own dials. Source:
//! notes/research/charcoal.md; every number here is an estimate from it.
//!
//! In the tooth, charcoal behaves as a black pastel does: loose particles
//! that the finger moves (`Canvas::rub`), a puff and a dry brush lift
//! (`sheet.rs`) and fixative binds and darkens (`Canvas::fix_pastel`), and
//! that paint over it seals. So it is a `Stick` and lays through the
//! pastel stroke, and what differs is what the stick is:
//!
//! - **Vine charcoal** is a carbonized twig with no binder: soft, friable,
//!   a grey-black powder that lies loose on the tooth. Its deposit is all
//!   loose, and the kneaded eraser takes it more readily than any other dry
//!   medium here (`lift` 1, above graphite's 0.9).
//! - **Compressed charcoal** is charcoal (and lamp black) powder pressed with
//!   a gum or wax binder: darker, a little harder, and the binder holds part
//!   of what it lays (`grip`): that share is bound in the tooth as it lands,
//!   so the finger, a puff and the eraser leave it, and what is loose lifts
//!   less readily (`lift`).
//!
//! Not modeled: the stick picking up the loose charcoal it passes over (a
//! pastel over loose charcoal muddies in the tooth, where the two mix, but
//! the stick stays clean); the particles' size (charcoal's are larger
//! splinters than pastel's: one particle thickness, `PARTICLE_UM`, serves
//! both); graphite's sheen (no medium here has one; charcoal is matte).

use crate::color::Rgb;
use crate::pastel::Stick;

/// Vine charcoal's dry color, R∞ (linear): a powder of wood charcoal
/// reflects under 5 % in the visible (charcoal.md §2); grey-black.
pub const VINE_COLOR: Rgb = [0.045, 0.045, 0.047];
/// Compressed charcoal's: charcoal with lamp black, "ultra dark",
/// "blue-black" (charcoal.md §2).
pub const COMPRESSED_COLOR: Rgb = [0.022, 0.022, 0.026];
/// How readily the kneaded eraser takes them (graphite 0.9, black chalk
/// 0.72, pastel 0.55; charcoal.md §4).
pub const VINE_LIFT: f32 = 1.0;
pub const COMPRESSED_LIFT: f32 = 0.35;
/// The share of compressed charcoal's deposit its binder holds (charcoal.md §3).
pub const COMPRESSED_GRIP: f32 = 0.5;

impl Stick {
    /// A stick of vine (willow) charcoal, Ø `d_mm` (thin 3–5, medium 5–8,
    /// thick 8–12), its end cut flat: soft and friable (σp 4 MPa, K 0.05),
    /// its deposit all loose.
    pub fn vine(d_mm: f32) -> Stick {
        let mut s = Stick::round(VINE_COLOR, 0.75, d_mm);
        s.hardness = 4.0;
        s.wear = 0.05;
        s.length_mm = 150.0;
        s.lift = VINE_LIFT;
        s
    }

    /// A square stick of compressed charcoal, `side_mm` across (about 7):
    /// a little harder than vine but a friable compact (σp 5 MPa, K 0.07),
    /// darker, its binder holding `COMPRESSED_GRIP` of what it lays.
    pub fn compressed(side_mm: f32) -> Stick {
        let mut s = Stick::square(COMPRESSED_COLOR, 0.6, side_mm);
        s.hardness = 5.0;
        s.wear = 0.07;
        s.length_mm = 70.0;
        s.lift = COMPRESSED_LIFT;
        s.grip = COMPRESSED_GRIP;
        s
    }
}

// (its tests are the pastel engine's: `pastel::tests::charcoal`)
