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
//!   seen as it is: a bed of particles has no first-surface veil over it
//!   (`Cell::bare_carbon`). The stick is mostly pores, so a worn volume lays
//!   a fifth of itself in splinters, and covers less (`VINE_COVER_UM`). A
//!   kneaded eraser out-pulls the paper's hold on a splinter that size, and
//!   the water condensed at its contact, most of the time (`VINE_LIFT`).
//! - **Compressed charcoal** is charcoal powder and lamp black pressed with
//!   a gum binder: darker (`crate::dry::COMPRESSED_CHARCOAL`), a little
//!   harder, a compact that lays fine grains which cover more per volume
//!   (`COMPRESSED_COVER_UM`). Below about 0.3 µm a particle holds to a fibre
//!   as a smooth sphere does, as hard as the eraser pulls it, so the eraser
//!   takes less (`COMPRESSED_LIFT`). Its gum is glassy at room humidity, so
//!   it bridges nothing to the sheet: the deposit is all loose, and the
//!   finger moves it as it moves vine.
//!
//! Not modeled: the stick picking up the loose charcoal it passes over (a
//! pastel over loose charcoal muddies in the tooth, where the two mix, but
//! the stick stays clean); a bloom of fine loose dust, greyer than the
//! splinters (charcoal_optics.py §1); graphite's sheen (no medium here has
//! one; charcoal is matte).

use crate::color::Rgb;
use crate::pastel::Stick;

/// The studio's relative humidity (0..1) the constants below are taken at:
/// 50 % (estimates-audit-2). It sets the water condensed at a particle's
/// contacts with the paper (`VINE_LIFT`) and keeps compressed charcoal's gum
/// glassy (`gum_tg_c`). HOOK(#322): #322 adds the one studio-humidity
/// setting; read it from there when it lands. Don't add a second one.
pub const STUDIO_RH: f32 = 0.50;

/// Gum arabic's glass transition (°C) at water activity `aw` (a gum at
/// equilibrium with the room: aw = RH): 62 °C at aw 0.33 and 42.6 °C at
/// 0.54, measured for the pure gum (Righetto & Netto 2005, Int. J. Food
/// Prop. 8:337), straight between and past them (92 K per unit aw).
pub fn gum_tg_c(aw: f32) -> f32 {
    42.6 - (62.0 - 42.6) / (0.54 - 0.33) * (aw - 0.54)
}

/// Vine charcoal's dry color, R∞ (linear): about 0.021, neutral.
pub fn vine_color() -> Rgb {
    crate::dry::VINE_CHARCOAL.color(0.0)
}

/// Compressed charcoal's: about 0.016.
pub fn compressed_color() -> Rgb {
    crate::dry::COMPRESSED_CHARCOAL.color(0.0)
}

/// How readily the kneaded eraser takes vine's splinters (R 1.5 µm): the
/// chance it out-pulls their hold on the paper (JKR on the eraser against
/// 10–150 nN of van der Waals on the fibre, a decade wide, plus the 90 nN
/// of the water bridge a hydrophilic char and fibre condense at a 0.1 µm
/// asperity at `STUDIO_RH`; 0.72–0.92 over asperities of 0.2–0.05 µm;
/// charcoal.md §4).
pub const VINE_LIFT: f32 = 0.85;
/// And compressed charcoal's deposit, a share f of fines (sub-0.3 µm, held
/// to the fibre as smooth spheres) and the rest crumbs held as vine's
/// splinters, both with the water bridge `VINE_LIFT` has at `STUDIO_RH`:
/// fines taken 0.10, crumbs 0.85, so 0.10 f + 0.85 (1 − f) = 0.48 with
/// f = 0.5, a half-soot stick (bounds 0.10–0.85 over f; charcoal_optics.py
/// §3). The audit's 0.70 (estimates-audit-2 C4) is the same mix dry, which
/// contradicts vine's own water term. f is unmeasured: an SEM of a mark
/// would set it.
pub const COMPRESSED_LIFT: f32 = 0.48;

/// The area a volume worn from a vine stick covers, as a layer's
/// thickness, µm: willow charcoal is about 80 % pores (apparent density
/// 0.27–0.35 against its walls' 1.44 g/cm³), and its splinters, a cell wall
/// (3 µm) thick, cover their own area per their thickness: 3/(1 − ε), 12–16.
pub const VINE_COVER_UM: f32 = 14.0;
/// And compressed's: the compact (60 % solids) gives up 3 µm grains, which
/// cover 1.5/d of their volume: d/(1.5 × 0.6).
pub const COMPRESSED_COVER_UM: f32 = 3.3;

/// The resin a light spray of workable fixative lays, µm: Krylon Workable
/// Fixatif is 5.25 % resin (its SDS) and an 11 oz can covers 15–20 ft² a
/// coat (retail listings), 8–11 µm of resin at 1.07 g/cm³ (poly(n-butyl
/// methacrylate)). About half of it lands and a light spray is a third of a
/// coat (both estimates): 1.4–1.8 µm.
pub const LIGHT_SPRAY_UM: f32 = 1.5;
/// The easel's light spray, as `Canvas::fix_pastel`'s `wet` (pastel's
/// share wetted, an estimate): what `LIGHT_SPRAY_UM` is the resin of.
pub const LIGHT_SPRAY_WET: f32 = 0.15;
/// The porosity of a loose bed of particles in the tooth (the audit's, as
/// loose pastel's).
const BED_EPS: f32 = 0.55;

/// The share of the pores of `loose_um` of loose charcoal (as it lies) that
/// `resin_um` of resin fills, 0..1.
pub fn wet_share(resin_um: f32, loose_um: f32) -> f32 {
    if loose_um <= 0.0 { 0.0 } else { (resin_um / (BED_EPS * loose_um)).clamp(0.0, 1.0) }
}

/// How carbon's K/S scales with resin (n 1.49) in all its pores, per
/// channel (R, G, B): its facets reflect about half against resin, its
/// absorption barely moves (Mie of `crate::dry::VINE_CHARCOAL` and
/// `COMPRESSED_CHARCOAL` in air and resin, their mean: 1.505/1.541,
/// 1.281/1.315, 0.944/0.974; charcoal_optics.py §2). Pastel's rule, ×5 for
/// a particle of n ≈ 2, would turn carbon to soot.
pub const WET_KS: [f32; 3] = [1.523, 1.298, 0.959];

impl Stick {
    /// A stick of vine (willow) charcoal, Ø `d_mm` (thin 5, medium 8, thick
    /// 10: Winsor & Newton's), 140 mm long, its end cut flat: soft and
    /// friable (σp 3 MPa, measured for wood charcoal fired at 450 °C; K 0.05,
    /// an estimate), its deposit all loose, bare carbon.
    pub fn vine(d_mm: f32) -> Stick {
        let mut s = Stick::round(vine_color(), 0.75, d_mm);
        s.hardness = 3.0;
        s.wear = 0.05;
        s.length_mm = 140.0;
        s.lift = VINE_LIFT;
        s.cover_um = VINE_COVER_UM;
        s.carbon = true;
        s
    }

    /// A square stick of compressed charcoal, `side_mm` across (General's
    /// is ¼″, 6.35 mm) and 76 mm (3″) long: a little harder than vine but a
    /// friable compact (σp 5 MPa, K 0.07, estimates), darker, laying fine
    /// grains.
    pub fn compressed(side_mm: f32) -> Stick {
        let mut s = Stick::square(compressed_color(), 0.6, side_mm);
        s.hardness = 5.0;
        s.wear = 0.07;
        s.length_mm = 76.0;
        s.lift = COMPRESSED_LIFT;
        s.cover_um = COMPRESSED_COVER_UM;
        s.carbon = true;
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compressed charcoal's gum is glassy in the studio: its glass
    /// transition is well above a warm room's 25 °C at `STUDIO_RH` (46 °C at
    /// 50 %), and reaches it only in a room above about 73 % RH.
    #[test]
    fn the_gum_is_glassy_at_the_studio_humidity() {
        assert!(gum_tg_c(STUDIO_RH) > 25.0 + 15.0, "{}", gum_tg_c(STUDIO_RH));
        assert!((gum_tg_c(0.33) - 62.0).abs() < 1e-3 && (gum_tg_c(0.54) - 42.6).abs() < 1e-3);
        let aw25 = 0.54 + (42.6 - 25.0) / ((62.0 - 42.6) / 0.21);
        assert!((gum_tg_c(aw25) - 25.0).abs() < 1e-3 && (0.72..0.74).contains(&aw25), "{aw25}");
    }

    /// The light spray's resin (Krylon's SDS and coverage) and what it wets:
    /// the pores of 10, 20 and 30 µm of loose bed take 0.27, 0.14 and 0.09
    /// (estimates-audit-2 C11); a thin line is soaked.
    #[test]
    fn a_light_spray_wets_thin_charcoal_more_than_thick() {
        let resin = |ft2: f32| 312.0 * 0.0525 / (ft2 * 0.0929) / 1.07 / 6.0;
        assert!(resin(20.0) < LIGHT_SPRAY_UM && LIGHT_SPRAY_UM < resin(15.0), "{} {}", resin(20.0), resin(15.0));
        for (t, want) in [(10.0, 0.27), (20.0, 0.14), (30.0, 0.09)] {
            assert!((wet_share(LIGHT_SPRAY_UM, t) - want).abs() < 0.005, "{t}: {}", wet_share(LIGHT_SPRAY_UM, t));
        }
        assert_eq!(wet_share(LIGHT_SPRAY_UM, 1.0), 1.0);
        assert_eq!(wet_share(LIGHT_SPRAY_UM, 0.0), 0.0);
    }

    /// Vine's coverage per worn volume from its porosity: 3 µm walls of a
    /// stick 0.27–0.35 g/cm³ against 1.44 cover as 12–16 µm; compressed's
    /// 3 µm grains of a 60 %-solids compact as 3.3.
    #[test]
    fn what_a_stick_lays_covers_by_what_it_is_made_of() {
        let cover = |rho: f32| 3.0 / (rho / 1.44);
        assert!(cover(0.35) < VINE_COVER_UM && VINE_COVER_UM < cover(0.27), "{} {}", cover(0.35), cover(0.27));
        assert!((COMPRESSED_COVER_UM - 3.0 / (1.5 * 0.6)).abs() < 0.05);
        assert_eq!(Stick::round([0.5; 3], 0.5, 10.0).cover_um, crate::pastel::PARTICLE_UM);
    }
}
