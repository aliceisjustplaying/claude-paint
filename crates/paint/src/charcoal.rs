//! Charcoal: vine (willow) and compressed sticks, as pastel sticks
//! (`crate::pastel`) with their own color and lift. Source:
//! notes/research/charcoal.md and the physics in charcoal_optics.py.
//!
//! In the tooth, charcoal behaves as a black pastel does: loose particles
//! that the finger moves (`Canvas::rub`), a puff and a dry brush lift
//! (`sheet.rs`) and fixative binds (`Canvas::fix_pastel`), and that paint
//! over it seals. So it is a `Stick` and lays through the pastel stroke, and
//! what differs is what the stick is:
//!
//! - **Vine charcoal** is a carbonized twig with no binder: splinters of
//!   charred cell wall, a few micrometres thick, lying loose on the tooth.
//!   Its color is what carbon's facets reflect (`crate::dry::VINE_CHARCOAL`),
//!   and a kneaded eraser out-pulls the paper on a splinter that size by
//!   ten times and more, so it takes nearly all it touches (`VINE_LIFT`).
//! - **Compressed charcoal** is charcoal powder pressed with a gum binder: a
//!   fine compact, darker (`crate::dry::COMPRESSED_CHARCOAL`), a little
//!   harder, that lays fine particles. Below about 0.3 µm a particle holds
//!   to a fibre as a smooth sphere does, as hard as the eraser pulls it, so
//!   the eraser takes less (`COMPRESSED_LIFT`). Its gum is glassy at room
//!   humidity, so it bridges nothing to the sheet: the deposit is all loose,
//!   and the finger moves it as it moves vine.
//!
//! Not modeled: the stick picking up the loose charcoal it passes over (a
//! pastel over loose charcoal muddies in the tooth, where the two mix, but
//! the stick stays clean); a bloom of fine loose dust, greyer than the
//! splinters (charcoal_optics.py §1); graphite's sheen (no medium here has
//! one; charcoal is matte).

use crate::color::Rgb;
use crate::pastel::Stick;

/// Vine charcoal's dry color, R∞ (linear): about 0.021, neutral.
pub fn vine_color() -> Rgb {
    crate::dry::VINE_CHARCOAL.color(0.0)
}

/// Compressed charcoal's: about 0.010.
pub fn compressed_color() -> Rgb {
    crate::dry::COMPRESSED_CHARCOAL.color(0.0)
}

/// How readily the kneaded eraser takes vine's splinters (R 1.5 µm): the
/// chance it out-pulls their hold on the paper (JKR on the eraser against
/// 10–150 nN on the fibre, a decade wide; charcoal.md §4).
pub const VINE_LIFT: f32 = 0.98;
/// And compressed charcoal's fine particles, held to the fibre as smooth
/// spheres (van der Waals, A·R/6D²) and pulled at 0.79 times that: the
/// physical floor, for a deposit all of fines (whole crumbs would lift as
/// vine; their share is unmeasured, charcoal.md §4).
pub const COMPRESSED_LIFT: f32 = 0.42;

impl Stick {
    /// A stick of vine (willow) charcoal, Ø `d_mm` (thin 3–5, medium 5–8,
    /// thick 8–12), its end cut flat: soft and friable (σp 4 MPa, K 0.05),
    /// its deposit all loose.
    pub fn vine(d_mm: f32) -> Stick {
        let mut s = Stick::round(vine_color(), 0.75, d_mm);
        s.hardness = 4.0;
        s.wear = 0.05;
        s.length_mm = 150.0;
        s.lift = VINE_LIFT;
        s
    }

    /// A square stick of compressed charcoal, `side_mm` across (about 7):
    /// a little harder than vine but a friable compact (σp 5 MPa, K 0.07),
    /// darker, laying fine particles.
    pub fn compressed(side_mm: f32) -> Stick {
        let mut s = Stick::square(compressed_color(), 0.6, side_mm);
        s.hardness = 5.0;
        s.wear = 0.07;
        s.length_mm = 70.0;
        s.lift = COMPRESSED_LIFT;
        s
    }
}

// (its tests are the pastel engine's: `pastel::tests::charcoal`)
