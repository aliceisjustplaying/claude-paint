//! Ink on a plate: inking, the warm plate, tarlatan, the hand wipe and
//! retroussage (notes/plan-intaglio.md §5, phase P3b; the easel's
//! `--@ press`). Physics: notes/research/intaglio_physics.md (**IP §n**),
//! with its tags: [M] measured, [D] derived, [P] practitioners, [E]
//! estimate.
//!
//! **The ink** (`Ink`) is an oil film: pigment from the box's tubes ground
//! in burnt plate oil (plan-intaglio I3). Its optics are its tubes' (their
//! masstone and scattering, thinned by the plate oil added), its drying is
//! the oil film's (`drying::rate` and engine 3's `Pace`, by the film's
//! thickness and its pigments' rates), and its viscosity is the plate oil's
//! raised by its pigment (Krieger–Dougherty) and lowered by warmth.
//!
//! **What a cell keeps** (`InkCell`): the ink in its openings, per width bin
//! as the plate keeps its openings (µm over the cell), the film on its flat
//! (µm over the cell), the ink's cure, and which of the plate's inks it is
//! (volume shares, at most `INKS`). Stored in tiles beside the cells.
//!
//! **Where.** Every tool here is moved by the hand along points the painter
//! gives and acts only where it touches (`plate::hand`): the card's edge
//! where it passes, a dabbed face's disc, a slid pad (tarlatan, palm, muslin)
//! for as far as it slides over a point. None takes a region.
//!
//! **Inking** pushes ink into every opening under the card or dabber and
//! leaves a film on the flat (`Plate::ink`). **Wiping** takes it off where a
//! cloth reaches it (`Plate::wipe`): a tarlatan is a `Rag` with its own
//! stiffness (`Tarlatan`), whose face loads, refolds and smears back as a
//! rag's does on a canvas; what it reaches is new: the yarn's dip into a
//! fine line (sheet.rs `tip_reach_um`, the dry brush's formula) and the
//! starched cloth's sag over a wide one, p·x²(w − x)²/(24B), which goes as
//! w⁴ at the middle (IP §5.3); and the burr's lee, where the cloth is held
//! off for (B·h/p)^¼ (IP §1.5). The **hand wipe** is a soft elastic contact
//! (skin) that reaches the flat and only the top of each opening and
//! polishes the film by splitting it; **retroussage** draws ink up out of
//! the lines into a soft cloth (Lucas–Washburn) and lays it beside them.
//!
//! **Ink is conserved.** Every verb reports what it put on the plate or took
//! off it, and where it went (the cloth's, the palm's), so the plate's ink
//! before is its ink after plus what left it, to rounding.

use super::*;
use crate::drying::{GEL, Pace};
use crate::rag::Rag;
use crate::surface::COAT_UM;

/// Inks a plate keeps (plan-intaglio §2.2: at most 8 in a session).
pub const INKS: usize = 8;

// ---------------------------------------------------------------- the ink's physics

/// The viscosity of burnt plate oil (Pa·s) at 20 °C, light (#00) and heavy
/// (#3), between which `Ink::stiff` takes it log-linearly. #3 "medium" is
/// taken as the 40-poise lithographic varnish of the Printing Ink Manual
/// (1961) [M, a litho varnish's working grade]; #00 as 0.1 Pa·s [E: the
/// makers say only "like cooking oil", 0.05–0.1 Pa·s for vegetable oils].
pub const PLATE_OIL_PA_S: (f64, f64) = (0.1, 4.0);
/// Krieger–Dougherty: a suspension's viscosity rises as (1 − φ/φ_m)^(−[η]φ_m)
/// with its particles' volume share φ, [η] = 2.5 for spheres (Einstein) and
/// φ_m = 0.64, random close packing [M, standard suspension rheology].
pub const KD_INTRINSIC: f64 = 2.5;
pub const KD_PACKING: f64 = 0.64;
/// Linseed oil's viscosity halves for every 15–20 °C of warming (IP §5.1,
/// standard oil data): the middle.
pub const HALVING_C: f64 = 17.5;
/// The ink's yield stress (Pa): IP §5.1's 10²–10³ for stiff letterpress and
/// offset inks, their geometric middle [E: none measured for etching ink].
pub const YIELD_PA: f64 = 300.0;
/// The share of an opening's volume the ink fills when nothing else limits
/// it: air trapped at the bottom of the groove under the card's bead [E:
/// plan-intaglio §5.1's 0.9 until Unknown 3 is weighed].
pub const PHI_AIR: f64 = 0.9;

/// The card (IP §5.2), dragged along the painter's points: its edge 50 mm
/// long [E] pressed with 10 N [E: a hand pushing a card's edge] on a bead
/// 2 mm across [E], so 100 kPa in the bead; the bead 5 mm long in the way it
/// moves [E], so a point lies under it 5 mm over the card's speed each time
/// the edge passes (0.25 s at 20 mm/s; printers go across, then diagonally
/// [P]). The pressure is the hand's force over the bead.
pub const CARD_MM: f64 = 50.0;
pub const CARD_N: f64 = 10.0;
pub const CARD_BEAD_MM: f64 = 2.0;
pub const CARD_RUN_MM: f64 = 5.0;
pub const CARD_PA: f64 = CARD_N / (CARD_MM * 1e-3 * CARD_BEAD_MM * 1e-3);
/// The card's edge drags a film behind it as a sliding pad does (Reynolds'
/// inclined slider, IP §5.2 [D]): h = √(6·K·η·U·ℓ/p), K = 0.0267 at the
/// best inclination [M, Reynolds' slider solution], U the card's speed (20
/// mm/s when none is given [E]) and the edge 1 mm across in the way it moves
/// [E: a card's bevel], p the bead's pressure.
pub const SLIDER_K: f64 = 0.0267;
pub const CARD_MM_S: f64 = 20.0;
pub const CARD_EDGE_MM: f64 = 1.0;
/// The dabber (a roll of felt worked with a twist, IP §5.2): 20 N on a face
/// 30 mm across [E], a dab 0.2 s on the plate [E]. The face, soft, is pressed
/// flat on the plate: it touches its disc, at the hand's force over it.
pub const DABBER_MM: f64 = 30.0;
pub const DABBER_N: f64 = 20.0;
pub const DAB_S: f64 = 0.2;
pub const DABBER_PA: f64 = DABBER_N / (std::f64::consts::PI * 0.25 * DABBER_MM * 1e-3 * DABBER_MM * 1e-3);
/// The poupée (à la poupée inking): a small dabber of rolled felt or
/// tarlatan, its face 20 mm across [E: rolled from a strip of felt two inches
/// wide (Belinda Del Pesco's dauber), sized to the colour it carries], pressed
/// at the dabber's pressure when no force is given.
pub const POUPEE_MM: f64 = 20.0;
/// The film a dabber leaves: half (a film's split, IP §6.4) of a charge
/// 20 µm thick on its face [E].
pub const DABBER_FILM_UM: f64 = 10.0;

/// The pressure under a tarlatan pad at `pressure` 1 (Pa): IP §5.3's "a
/// hand's 20–50 N over 20–50 cm²", 4–25 kPa, its top at 20 kPa (so 10 kPa,
/// the note's working figure, at 0.5) [D from the hand's force].
pub const PAD_PA: f64 = 20e3;
/// A tarlatan yarn's radius (µm) [E: IP §5.3's cheesecloth yarn; Unknown 8]
/// (`etch::YARN_UM`).
pub const YARN_UM: f64 = crate::etch::YARN_UM;
/// Starched tarlatan's bending rigidity (N·m) at `stiff` 0 and 1: IP
/// §5.3's 10⁻⁵–10⁻⁴ [E: the fabric range is measured (Kawabata KES-F),
/// the starched value extrapolated; Unknown 8], log-linearly between.
pub const BEND_NM: (f64, f64) = (1e-5, 1e-4);
/// The palm (IP §5.4): skin's modulus 0.3 MPa [E: IP's 0.1–1 MPa, its
/// geometric middle], Poisson 0.5 (nearly incompressible, [M]); pressed at
/// 10 kPa at `pressure` 1 [E: 20–50 N over a palm's 50 cm²].
pub const SKIN_PA: f64 = 0.3e6;
pub const SKIN_NU: f64 = 0.5;
pub const PALM_PA: f64 = 10e3;
/// A film between two surfaces that part splits about evenly (IP §6.4's
/// split share, 0.45–0.5 for a non-absorbent sheet): skin takes half the
/// film it touches [D]. Whiting drinks the oil (IP §5.4) and takes nearly
/// all of it: 0.9 [E: unmeasured].
pub const PALM_SPLIT: f64 = 0.5;
pub const WHITING_SPLIT: f64 = 0.9;
/// Times a hand wipe's pass over the plate touched each point in Cassatt's
/// pace (`pace::PALM_MM2_S`) [E: the palm sweeps in overlapping strokes]:
/// it makes the pace of one touch (`pace::PALM_TOUCH_MM2_S`). Each touch is
/// a pass of the palm's edge over the point along the painter's points.
pub const PALM_TOUCHES: f64 = 4.0;
/// The palm's breadth across the way it is drawn: 80 mm [M: hand breadth at
/// the metacarpals, 50th percentile 85 mm for British men and 75 mm for
/// women (Pheasant, Bodyspace)].
pub const PALM_MM: f64 = 80.0;
/// Retroussage (IP §5.4): a soft muslin passed lightly over the plate. Its
/// loose fibre ends, cotton 15 µm thick [M: cotton fibres are 12–20 µm
/// across], hang into any opening wider than a fibre and touch the ink in
/// it; it draws ink up into its fibres by capillarity (Lucas–Washburn, h =
/// √(γ·r·t/(2η))): linseed oil's surface tension 0.033 N/m [M], the pores
/// between its fibres 10 µm [E], 0.05 s on a point each time the cloth
/// passes over it [E]. The cloth is a pad as wide as a rag's
/// (`rag::PAD_MM`) when none is given. What it draws
/// up it lays beside the line within 0.5 mm [E: the cloth's contact along
/// its pass], but for what soaks into it (`rag`'s soaking share).
pub const FIBRE_UM: f64 = 15.0;
pub const OIL_TENSION: f64 = 0.033;
pub const MUSLIN_PORE_UM: f64 = 10.0;
pub const RETRO_TOUCH_S: f64 = 0.05;
pub const RETRO_BAND_MM: f64 = 0.5;
/// The share of the plate a tarlatan's loose fibre ends touch where the cloth
/// stands clear of it (under the contact laws): the length of fibre standing
/// out of its yarns, Uster's hairiness H ≈ 5 cm per cm of yarn [M: carded
/// ring-spun cotton 4–7], over its 3000 m of yarn per m² (15 threads/cm each
/// way), is 15 km of fibre per m²: a cover of 0.22 at `FIBRE_UM` if it all
/// lay flat on the plate; a hanging end touches with its tip, taken as a
/// tenth of that [E: unmeasured].
pub const HAIRINESS: f64 = 5.0;
pub const TARLATAN_YARN_M_M2: f64 = 3000.0;
pub const TIP_SHARE: f64 = 0.1;
pub const FIBRE_END_SHARE: f64 = HAIRINESS * TARLATAN_YARN_M_M2 * FIBRE_UM * 1e-6 * TIP_SHARE;

/// A plate cooling in air off the hot plate: still air on both faces and
/// the bench under it, 10 W/m²K over each unit of its area [E: natural
/// convection from a warm horizontal plate is 2–5 W/m²K a face, polished
/// copper radiates almost nothing]. Its heat per area is ρ·c·t: copper
/// 8960 kg/m³ × 385 J/kgK, zinc 7140 × 388 [M, handbook tables].
pub const COOL_W_M2K: f64 = 10.0;
pub const ROOM_C: f64 = 20.0;

/// Hand time (IP §5.4: Cassatt's letter, 15–20 min to ink and wipe a plate
/// for her three-plate color proofs, eight or ten a day [P, a historical
/// measurement]). Her plates were about 35 × 21 cm (728 cm²); the usual
/// sequence is a card's inking, three tarlatan passes and two of the palm,
/// 17.5 min, shared as a quarter to the card, a half to the tarlatan and a
/// quarter to the palm [E: the split]: 728 cm² in 262 s for the card, and
/// 525 s over three passes and 262 s over two for tarlatan and palm.
///
/// A tool moved along points takes the area it passes over (each pass
/// counted) at the pace of one pass: the card's inking went over the plate
/// twice (across, then diagonally [P]), so a pass of the card is twice its
/// pace; the dabber's was ten dabs on a point, so a dab covers its face ten
/// times as fast; a tarlatan pass is one pass; a pass of the palm touched
/// each point `PALM_TOUCHES` times.
pub mod pace {
    pub const CARD_MM2_S: f64 = 72_800.0 / 262.5;
    pub const DABBER_MM2_S: f64 = 72_800.0 / 262.5 / 2.0;
    pub const TARLATAN_MM2_S: f64 = 3.0 * 72_800.0 / 525.0;
    pub const PALM_MM2_S: f64 = 2.0 * 72_800.0 / 262.5;
    /// Retroussage: a light pass, as fast as the palm's [E].
    pub const RETRO_MM2_S: f64 = PALM_MM2_S;
    /// A pass of the card's edge (or of a dabber's face rubbed along), a dab,
    /// and a touch of the palm.
    pub const CARD_PASS_MM2_S: f64 = 2.0 * CARD_MM2_S;
    pub const DAB_MM2_S: f64 = 10.0 * DABBER_MM2_S;
    pub const PALM_TOUCH_MM2_S: f64 = super::PALM_TOUCHES * PALM_MM2_S;
    /// Warming a plate on the hot plate: copper's ρct over a contact
    /// conductance of 300 W/m²K is 15 s, four of them to come within 2 % of
    /// the hot plate [D; the conductance E], with the reach to it and back.
    pub const WARM_S: f64 = 60.0 + 4.0 * 15.0;
    /// A tarlatan refolded: as a rag (`rag::pace::REFOLD`).
    pub const REFOLD: f64 = crate::rag::pace::REFOLD;
}

/// Burnt plate oil's viscosity (Pa·s, 20 °C) at grade `stiff` (0 light .. 1 heavy).
pub fn plate_oil_pa_s(stiff: f64) -> f64 {
    let (a, b) = PLATE_OIL_PA_S;
    a * (b / a).powf(stiff.clamp(0.0, 1.0))
}

/// Krieger–Dougherty's factor at a pigment volume share `phi`.
pub fn krieger(phi: f64) -> f64 {
    let r = (1.0 - phi / KD_PACKING).max(1e-3);
    r.powf(-KD_INTRINSIC * KD_PACKING)
}

/// How much less viscous linseed oil is at `temp_c` than at 20 °C.
pub fn warmth(temp_c: f64) -> f64 {
    0.5f64.powf((temp_c - ROOM_C) / HALVING_C)
}

/// How deep (µm) ink of viscosity `eta` (Pa·s) and yield stress `tau` (Pa)
/// is pushed into an opening `w` µm wide at `p` Pa for `t` s [D]:
/// - viscously, plane Poiseuille flow into a slot, dL/dt = w²p/(12ηL), so
///   L = w·√(p·t/(6η));
/// - against its yield stress, a plug held by both walls, p·w = 2τL, so
///   L = p·w/(2τ).
pub fn pushed_um(w: f64, p: f64, t: f64, eta: f64, tau: f64) -> f64 {
    let visc = w * (p * t / (6.0 * eta.max(1e-6))).sqrt();
    let plug = if tau > 0.0 { p * w / (2.0 * tau) } else { f64::INFINITY };
    visc.min(plug)
}

/// The share of an opening `w` µm wide and `d` µm deep the ink fills.
pub fn fill(w: f64, d: f64, p: f64, t: f64, eta: f64, tau: f64) -> f64 {
    if d <= 0.0 {
        return 0.0;
    }
    (pushed_um(w, p, t, eta, tau) / d).min(PHI_AIR)
}

/// The film (µm) a card's edge drawn at `u_mm_s` and pressing its bead at
/// `p` Pa leaves on the flat (`SLIDER_K`).
pub fn card_film_um(eta: f64, u_mm_s: f64, p: f64) -> f64 {
    (6.0 * SLIDER_K * eta * u_mm_s * 1e-3 * CARD_EDGE_MM * 1e-3 / p.max(1.0)).sqrt() * 1e6
}

/// A cloth's sag (µm) at `x` µm in from the rim of a gap `w` µm wide, pressed
/// at `p` Pa, its bending rigidity `b` N·m: a beam clamped at both rims under
/// a uniform load, p·x²(w − x)²/(24B), p·w⁴/(384B) at the middle (IP §5.3).
pub fn sag_um(w: f64, x: f64, p: f64, b: f64) -> f64 {
    let (w, x) = (w * 1e-6, x.clamp(0.0, w) * 1e-6);
    p * x * x * (w - x) * (w - x) / (24.0 * b) * 1e6
}

/// How far (µm) a yarn of radius `r` µm dips into an opening `w` µm wide,
/// lying along it: the chord r − √(r² − w²/4) (sheet.rs `tip_reach_um`, the
/// dry brush's tips over paper pores); in a gap wider than the yarn it goes
/// in to its axis and no further, unless the cloth sags (`sag_um`).
pub fn yarn_dip_um(r: f64, w: f64) -> f64 {
    let d = crate::sheet::tip_reach_um(r as f32, w as f32);
    if d.is_finite() { d as f64 } else { r }
}

/// How far (µm) the cloth is held off the plate past a burr `h` µm high,
/// pressed at `p` Pa, bending rigidity `b` N·m: (B·h/p)^¼ (IP §1.5).
pub fn lee_um(h: f64, p: f64, b: f64) -> f64 {
    (b * h * 1e-6 / p).powf(0.25) * 1e6
}

// ---------------------------------------------------------------- the cloth's face (contact laws)

/// Tarlatan's weight (g/m²): 35–55 sold [M: TNTEX's 15- and 17-thread stiff
/// tarlatans 35 and 41 g/m², Whaleys' cotton tarlatan 55 g/m²], the middle.
pub const TARLATAN_GSM: f64 = 45.0;
/// The Peirce bending length (cm) of tarlatan softened for wiping, at
/// `stiff` 0 and 1 [E: printmakers soften the cloth before use (IP §5.3);
/// ordinary cotton cloth measures 2–3 cm, which softened starched cloth is
/// taken to come to; the stiff bolt's would be some 5 cm].
pub const PEIRCE_CM: (f64, f64) = (2.0, 3.0);
/// The tarlatan's bending rigidity (N·m) under the contact laws: Peirce's
/// cantilever, B = W·c³ (W its weight per area, c its bending length) [M,
/// Peirce 1930, the standard (ASTM D1388) bending test], c taken linearly
/// between `PEIRCE_CM` by `stiff`: 3.5 × 10⁻⁶ to 1.2 × 10⁻⁵ N·m, inside
/// ordinary cotton's 2 × 10⁻⁶ – 1.2 × 10⁻⁵ by Kawabata's KES-FB2.
pub fn softened_bend_nm(stiff: f64) -> f64 {
    let c = (PEIRCE_CM.0 + (PEIRCE_CM.1 - PEIRCE_CM.0) * stiff.clamp(0.0, 1.0)) * 1e-2;
    TARLATAN_GSM * 1e-3 * 9.81 * c * c * c
}
/// The cloth's face, its thickness (µm) under the KES reference load and the
/// share of it the KES-FB3 squeeze takes off: a woven cloth stands with its
/// crimp far thicker than its fibres would pack (45 g/m² of cotton is 30 µm
/// solid): two crossing yarns of 12–26 tex (15 threads/cm at 35 g/m² to 22
/// threads/inch at 55 g/m²) are 0.13–0.19 mm across each, nested by the
/// crimp, 0.25 mm [D from the yarns, E for the nesting]; woven cottons lose
/// 20–40 % of their thickness between 50 Pa and 5 kPa [M, KES-FB3 EMC], 30 %.
pub const FACE_T0_UM: f64 = 250.0;
pub const FACE_EMC: f64 = 0.3;
/// KES-FB3's reference and top loads (Pa): 0.5 and 50 gf/cm² [M].
pub const KES_PA: (f64, f64) = (50.0, 5000.0);
/// Cotton's density, 1.52 g/cm³ [M, cellulose fibre], so 45 g/m² is 29.6 µm
/// of solid; and the share of a yarn's section its fibres fill, 0.6 (the
/// yarn packing `FACE_T0_UM` was sized by) [M: ring-spun cotton yarns
/// 0.5–0.65].
pub const COTTON_DENSITY: f64 = 1.52;
pub const FIBRE_PACKING: f64 = 0.6;

/// A cloth's face pressed at `p` Pa (under the contact laws): its yarns'
/// crimp and fibres are a compressible skin between the cloth and the
/// plate, which follows van Wyk's law for a fibre mass, p = K/(T − T_c)³
/// (T_c the incompressible core), as de Jong et al. and Hu & Newton found
/// for woven fabrics' low-load compression [M, J. Text. Inst. 88 (1997)].
/// Its two constants come from `FACE_T0_UM` and `FACE_EMC` at `KES_PA`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Face {
    pub p: f64,
    /// The compressible thickness left at p (µm): T − T_c.
    pub s: f64,
    /// How far it is squeezed at p from its free thickness (µm): a lift of
    /// the cloth by more than this opens a true gap.
    pub delta: f64,
}

impl Face {
    pub fn at(p: f64) -> Face {
        let ratio = (KES_PA.1 / KES_PA.0).cbrt();
        let sm = FACE_EMC * FACE_T0_UM / (ratio - 1.0);
        let s = sm * (KES_PA.1 / p.max(1.0)).cbrt();
        Face { p, s, delta: (sm * ratio - s).max(0.0) }
    }

    /// Its stiffness at p (Pa/m), dp/dT = 3p/(T − T_c): a Winkler skin's.
    pub fn k_pa_m(&self) -> f64 {
        3.0 * self.p / (self.s * 1e-6)
    }

    /// The pressure (Pa) it bears with the cloth lifted `u` µm off its place
    /// at p, over a base raised `hb` µm (a burr's top): van Wyk's law; none
    /// past its free thickness (a true gap).
    fn bears(&self, u: f64, hb: f64) -> f64 {
        if u - hb >= self.delta {
            return 0.0;
        }
        self.p * (self.s / (self.s + u - hb).max(1e-9)).powi(3)
    }

    /// The share of its fibres at p that lie in a film on the plate with the
    /// cloth lifted `u` µm off its place (below zero: pressed `−u` harder, as
    /// over a burr's top), while the face still bears: s/(s + u) = r^⅓, r the
    /// share of p it bears. The compressible layer keeps its fibres as it
    /// swells or is squeezed (van Wyk's homogeneous mass), so their density,
    /// and the number in any slab of it at the plate, goes as 1/(T − T_c).
    /// Squeezed, no denser than fibres pack (`FIBRE_PACKING` over the face's
    /// own density at p) [D]. Past its squeeze δ it stands clear: none.
    pub fn fibres(&self, u: f64) -> f64 {
        if u >= self.delta {
            return 0.0;
        }
        (self.s / (self.s + u).max(1e-9)).min(self.packed())
    }

    /// How many times denser its fibres can be squeezed than at p: the
    /// packing of fibres in a yarn over the cloth's own fibre share at p,
    /// its cotton's solid thickness (`TARLATAN_GSM` at `COTTON_DENSITY`)
    /// over the thickness it is pressed to, T_c + s [D].
    pub fn packed(&self) -> f64 {
        let ratio = (KES_PA.1 / KES_PA.0).cbrt();
        let sm = FACE_EMC * FACE_T0_UM / (ratio - 1.0);
        let core = FACE_T0_UM - sm * ratio;
        let solid = TARLATAN_GSM / COTTON_DENSITY;
        FIBRE_PACKING / (solid / (core + self.s))
    }

    /// The share of p it bears and the true gap (µm) under the cloth lifted
    /// `u` µm off the flat.
    pub fn state(&self, u: f64) -> (f64, f64) {
        let u = u.max(0.0);
        if u >= self.delta { (0.0, u - self.delta) } else { ((self.s / (self.s + u)).powi(3), 0.0) }
    }
}

/// How the tarlatan lies past a burr under the contact laws: its lift (µm)
/// off its place on the flat at `i·dx` µm from the burr's middle. Past some
/// 3/4 of a bending length out it dips a little below its place (a plate on
/// a Winkler skin rings, by at most e^(−π) ≈ 4 % of the lift): the wipe
/// reads that as full contact, not the slightly firmer press it is [E: a
/// few per cent of the pressure over a narrow ring].
#[derive(Clone, Debug, PartialEq)]
pub struct Lift {
    pub dx: f64,
    pub u: Vec<f32>,
}

impl Lift {
    /// The lift at `x` µm from the burr's middle (none past its end).
    pub fn at(&self, x: f64) -> f64 {
        let f = x.max(0.0) / self.dx;
        let i = f.floor() as usize;
        if i + 1 >= self.u.len() {
            return 0.0;
        }
        let t = f - i as f64;
        self.u[i] as f64 * (1.0 - t) + self.u[i + 1] as f64 * t
    }

    /// How far (µm) from the burr's middle it reaches.
    pub fn reach(&self) -> f64 {
        self.dx * self.u.len() as f64
    }
}

/// The tarlatan over a burr `h` µm high whose top bears `c` µm either side
/// of its middle, pressed at `p` Pa, bending rigidity `b` N·m, under the
/// contact laws [D]: a plate (the cloth) on its own compressible face
/// (`Face`) under the pad's even pressure, the burr a base raised h under
/// the face over |x| ≤ c. Plane strain, symmetric about the burr:
/// B·u'''' = q(u) − p, q the face's van Wyk pressure (none where the face
/// stands free of the plate). The burr squeezes the face over its top (the
/// face's core can't be squeezed, so a burr taller than the face's give
/// lifts the cloth by the rest) and the cloth, lifted u₀ there, settles back
/// over the Winkler length (4B/k)^¼ of its face's stiffness k. Solved by
/// Newton's method on a grid of a sixtieth of that length (the burr's top,
/// narrower, bears over its share of the nodes it covers), raising the burr
/// in steps; the far end simply supported, ten bending lengths out.
pub fn cloth_lift(h: f64, c: f64, p: f64, b: f64) -> Lift {
    let face = Face::at(p);
    let k = face.k_pa_m();
    let lam = (4.0 * b / k).powf(0.25) * 1e6;
    let dx = lam / 60.0;
    let n = (((10.0 * lam + c) / dx).ceil() as usize + 1).max(8);
    // the share of node i's span (node 0's half span) that lies on the burr's top
    let on = |i: usize| if i == 0 { (c / (0.5 * dx)).min(1.0) } else { ((c - (i as f64 - 0.5) * dx) / dx).clamp(0.0, 1.0) };
    let c4 = b * 1e18 / dx.powi(4);
    let beta = 2f64.sqrt() / lam;
    let x = |i: usize| i as f64 * dx;
    let mut u = vec![0.0f64; n];
    if h <= 0.0 {
        return Lift { dx, u: Vec::new() };
    }
    // the stencil of u'''' at row i over the unknowns 0..n (mirror at 0, u = u'' = 0 at n)
    let stencil = |i: usize| -> [(usize, f64); 5] {
        let mut out = [(0usize, 0.0f64); 5];
        for (slot, (o, w)) in [(-2i64, 1.0), (-1, -4.0), (0, 6.0), (1, -4.0), (2, 1.0)].into_iter().enumerate() {
            let j = i as i64 + o;
            out[slot] = if j < 0 { ((-j) as usize, w) } else if j as usize == n { (0, 0.0) } else if j as usize == n + 1 { (n - 1, -w) } else { (j as usize, w) };
        }
        out
    };
    let steps = ((h / face.s) * 4.0).ceil().max(4.0) as usize;
    let mut band = vec![[0.0f64; 5]; n];
    let mut rhs = vec![0.0f64; n];
    let mut hb_prev = 0.0;
    for st in 1..=steps {
        let hb = h * st as f64 / steps as f64;
        // carry the cloth up with the burr, by the shape of a lifted plate on a Winkler skin
        for (i, v) in u.iter_mut().enumerate() {
            let s = beta * (x(i) - c).max(0.0);
            *v += (hb - hb_prev) * (-s).exp() * (s.cos() + s.sin());
        }
        hb_prev = hb;
        for _ in 0..80 {
            let mut worst = 0.0f64;
            for i in 0..n {
                // the face on the burr's top (its share of the node) and on the flat
                let (f, (qb, qf)) = (on(i), (face.bears(u[i], hb), face.bears(u[i], 0.0)));
                let q = f * qb + (1.0 - f) * qf;
                let dq = f * if qb > 0.0 { 3.0 * qb / (face.s + u[i] - hb).max(1e-9) } else { 0.0 } + (1.0 - f) * if qf > 0.0 { 3.0 * qf / (face.s + u[i]).max(1e-9) } else { 0.0 };
                let half = if i == 0 { 0.5 } else { 1.0 };
                let mut row = [0.0f64; 5];
                let mut d4 = 0.0;
                for (col, w) in stencil(i) {
                    if w == 0.0 {
                        continue;
                    }
                    d4 += w * u[col];
                    row[(col as i64 - i as i64 + 2) as usize] += w * c4 * half;
                }
                row[2] += dq * half;
                band[i] = row;
                rhs[i] = -(c4 * d4 - (q - face.p)) * half;
            }
            // pentadiagonal elimination (no pivoting: the system is a symmetric beam's, made
            // so by halving the middle row, and its foundation adds to the diagonal)
            for i in 0..n {
                let piv = band[i][2];
                for r in 1..=2 {
                    if i + r >= n {
                        break;
                    }
                    let f = band[i + r][2 - r] / piv;
                    if f == 0.0 {
                        continue;
                    }
                    for k in 0..=2 {
                        if 2 - r + k <= 4 && k + 2 <= 4 {
                            band[i + r][2 - r + k] -= f * band[i][2 + k];
                        }
                    }
                    rhs[i + r] -= f * rhs[i];
                }
            }
            let mut du = vec![0.0f64; n];
            for i in (0..n).rev() {
                let mut v = rhs[i];
                for k in 1..=2 {
                    if i + k < n {
                        v -= band[i][2 + k] * du[i + k];
                    }
                }
                du[i] = v / band[i][2];
            }
            // keep the face's squeezed thickness positive: a step at most 80 % of the way to its core
            let mut a = 1.0f64;
            for i in 0..n {
                let base = if on(i) > 0.0 { hb } else { 0.0 };
                let t = face.s + u[i] - base;
                if du[i] < 0.0 && t > 0.0 {
                    a = a.min(0.8 * t / -du[i]);
                }
            }
            for i in 0..n {
                u[i] += a * du[i];
                worst = worst.max((a * du[i]).abs());
            }
            if worst < 1e-5 {
                break;
            }
        }
    }
    // where it has settled: the face bearing within 0.5 % of p
    // (or within a hundredth of its lift, for a burr too low to matter)
    let still = (face.s * (0.995f64.powf(-1.0 / 3.0) - 1.0)).min(0.01 * u[0].abs());
    let last = u.iter().rposition(|&v| v.abs() > still).map_or(0, |i| i + 2).min(n);
    Lift { dx, u: u[..last].iter().map(|&v| v as f32).collect() }
}

/// The surface of a burr's fin (µm over the plate) `s` µm from its crest
/// (below zero toward its groove), under the contact laws [D]: the engine's
/// fin is a ridge of triangular section `h` high (`Cell::burr_height`, the
/// deposit laid by area, `Plate::deposit`), its inner face reaching `ei`
/// across from its crest to the groove's rim and its outer face `eo` out to
/// its toe; none past them. In ploughing the metal rises against the tool's
/// face (the indenter touches its own pile-up: the contact in a scratch
/// reaches above the plate), so the fin's inner face is the groove's wall
/// carried on up, and its crest stands h·tan of that wall's angle from
/// upright out from the rim (`Plate::fences`); the rest of its section,
/// whose area is its side's share of the groove's, lies outside the crest.
pub fn fin_um(h: f64, ei: f64, eo: f64, s: f64) -> f64 {
    let e = if s < 0.0 { ei } else { eo };
    if h <= 0.0 || e <= 0.0 || s.abs() >= e { 0.0 } else { h * (1.0 - s.abs() / e) }
}

/// How far (µm) a tarlatan yarn resting on a burr's crest stands off the
/// plate `d` µm from it, the burr `h` µm high (under the contact laws) [D,
/// E]: a yarn is a bundle of fibres packed to some 0.6 of its section, all
/// but as full as fibres pack, so below its own radius it keeps its round
/// shape; the face's give is its crimp and hairs, which flex over the
/// weave's pitch. That is how the engine already reaches into a line, by a
/// yarn's chord (`yarn_dip_um`), and the same yarn resting on the fin's
/// crest falls away beside it as its circle, h − (R − √(R² − d²)), R the
/// yarn's radius (`YARN_UM` [E]); none past √(2Rh) or so. The fin is a ridge
/// (`fin_um`), not a table: over its faces, which fall away under the yarn
/// far faster than its circle does (slopes of h/ei and h/eo, about 1.4 and
/// 0.6 for a 2 N fin, against d/R), the yarn stands off them by this less
/// the face's height, so the ink on the fin's own faces is shaded too, and
/// only the crest is wiped. Over the groove between two crests, its circle
/// stands it off the line's rims (a 2 N line's crests 0.115 mm apart under a
/// yarn 0.15 mm across): it reaches none of the line's ink. In that shade only the ink
/// standing above the yarn, and its loose fibre ends, are reached: the ink on
/// and against the burr, a drypoint line's velvet. The yarn is taken lying
/// along the fin, its round section across it, which shades least; a yarn
/// crossing the fin is the cloth itself bent over it, its lift `cloth_lift`
/// [D].
pub fn yarn_shade_um(h: f64, d: f64) -> f64 {
    if h <= 0.0 {
        return 0.0;
    }
    let x = d.abs();
    if x >= YARN_UM {
        return (h - YARN_UM).max(0.0);
    }
    (h - (YARN_UM - (YARN_UM * YARN_UM - x * x).sqrt())).max(0.0)
}

/// How far (µm) the ridge of the palm riding a burr's crest stands off the
/// burr's face `s` µm from its crest (`fin_um`'s faces, `ei` and `eo`), under
/// the contact laws [D, E]: the ridge carries its share of the palm's load,
/// P = p·λ a length (`p` Pa, λ the ridges' pitch), on the crest, and skin
/// pressed by a two-dimensional wedge whose faces slope h/ei and h/eo touches
/// it over a half-width a = 2P/(E*·(h/ei + h/eo)) [D: a two-dimensional punch
/// of profile f on an elastic half-space carries P = (E*/2)∫x·f′(x)/√(a² −
/// x²)dx over its contact (Johnson, *Contact Mechanics*, ch. 5), a·E*·tan β
/// for a wedge whose faces slope tan β; the mean of the two faces' here]:
/// some 6 µm either side of a 2 N fin's crest at 5 kPa, of faces 20 and 45 µm
/// across. Below the contact's edge the face falls away under the skin: it
/// stands off by the face's drop from there, h(|s| − a)/e, the least the
/// skin, less pressed outside its contact, stands off [D, a bound]. That the
/// riding ridge carries its own share is the estimate [E: a ridge propped by
/// the burr carries more, by its own compression; doubling P doubles a].
pub fn palm_on_fin_um(h: f64, ei: f64, eo: f64, s: f64, p: f64) -> f64 {
    let e = if s < 0.0 { ei } else { eo };
    if h <= 0.0 || e <= 0.0 || s.abs() >= e {
        return 0.0;
    }
    let es = SKIN_PA / (1.0 - SKIN_NU * SKIN_NU);
    // (its faces sloping h/ei and h/eo: the wedge's mean slope)
    let a = 2.0 * p * RIDGE_PITCH_UM * 1e-6 / (es * (h / ei.max(1e-6) + h / eo.max(1e-6))) * 1e6;
    (h * (s.abs() - a) / e).max(0.0)
}

/// The plate's polish under the contact laws (IP §5.4, "Plate tone"): a
/// copper plate sold "mirror polished" yields little or no tone and one of
/// mill finish a significant tone [P, Tuckamore Press], because the ink in
/// its finest scratches is out of a wipe's reach: a cotton fibre (12–20 µm)
/// or the palm's skin, pressed at kPa, bridges a scratch a µm or two across
/// (skin bulges p(1 − ν²)w/E ≈ 0.03 µm into one 1 µm wide at 10 kPa, `palm_dip_um`).
/// Its roughness, Ra 0.1 µm [E: between a mirror polish's 0.05 µm and a
/// cold-rolled mill finish's 0.2–1 µm (general finishing data; no measurement
/// of a printmaker's plate was found)]. A cloth's fibres rest on its summits
/// and leave at least the void below its mean plane, σ/√(2π) for Gaussian
/// heights, σ = Ra·√(π/2): Ra/2 [D]. The palm's skin is softer than that:
/// pressed at 10 kPa it conforms to undulations of amplitude A whose
/// wavelength λ passes π·E*·A/p (Johnson's full-contact pressure p* =
/// π·E*·A/λ), some 10 µm for this roughness, so it leaves only the void of
/// the shorter wavelengths; their share of the roughness is unmeasured, and
/// Ra/2 is kept for both as its order [E].
pub const POLISH_RA_UM: f64 = 0.1;
pub const POLISH_HOLD_UM: f64 = 0.5 * POLISH_RA_UM;

/// How far (µm) skin pressed at `p` Pa bulges into an opening `w` µm wide,
/// `x` µm in from its rim: a pressurised slit in an elastic half-space,
/// 2p(1 − ν²)/E·√(x(w − x)), p·w(1 − ν²)/E at the middle [D, Sneddon's
/// crack opening].
pub fn palm_dip_um(w: f64, x: f64, p: f64) -> f64 {
    let x = x.clamp(0.0, w);
    2.0 * p * (1.0 - SKIN_NU * SKIN_NU) / SKIN_PA * (x * (w - x)).sqrt()
}

/// How far (µm) skin pressed at `p` Pa is held off past a step `h` µm high:
/// the span over which its compliance takes up the step, h·E/(p(1 − ν²))
/// [D, the same slit].
pub fn palm_lee_um(h: f64, p: f64) -> f64 {
    h * SKIN_PA / (p * (1.0 - SKIN_NU * SKIN_NU))
}

/// The palm's gap past a burr (`Plate::contact`, `--@ intaglio-contact`; IP
/// §5.4): skin pressed at `p` Pa onto the plate is propped up by a burr `h`
/// µm high whose top bears on it over `c` µm either side of its middle.
/// Plane strain, an elastic half-space (E* = E/(1 − ν²)) on a rigid flat: the
/// skin lifts off over `a` either side, a gap in the contact opened by the
/// burr's line load P and closed by the pressure. It closes smoothly (no
/// singular pressure at its edges: K = P/√(πa) − p√(πa) = 0, so P = πpa), and
/// the skin then stands (2pa/E*)·(acosh(a/x) − √(1 − x²/a²)) over the plate at
/// x from the burr's middle, h at x = c [D: the crack-opening displacements of
/// a pressurised crack and of a pair of point forces at its middle (Tada,
/// Paris & Irwin, The Stress Analysis of Cracks Handbook, 5.1 and 5.4), half
/// of each for one elastic face on a rigid one]. The span `a` solves that at
/// x = c.
pub fn palm_span_um(h: f64, c: f64, p: f64) -> f64 {
    let es = SKIN_PA / (1.0 - SKIN_NU * SKIN_NU);
    let c = c.max(1e-3);
    let at = |a: f64| 2.0 * p * a / es * ((a / c).acosh() - (1.0 - (c / a).powi(2)).max(0.0).sqrt());
    if h <= 0.0 || p <= 0.0 {
        return c;
    }
    let (mut lo, mut hi) = (c, 2.0 * c);
    while at(hi) < h && hi < 1e7 {
        lo = hi;
        hi *= 2.0;
    }
    for _ in 0..60 {
        let m = 0.5 * (lo + hi);
        if at(m) < h { lo = m } else { hi = m }
    }
    0.5 * (lo + hi)
}

/// The skin's height (µm) over the plate at `x` µm from a burr's middle
/// (`palm_span_um`: its height `h`, its top's half-width `c`, the pressure
/// `p`, the span `a`): `h` on the burr, nothing past the span.
pub fn palm_gap_um(h: f64, c: f64, p: f64, a: f64, x: f64) -> f64 {
    if x <= c {
        return h;
    }
    if x >= a {
        return 0.0;
    }
    let es = SKIN_PA / (1.0 - SKIN_NU * SKIN_NU);
    (2.0 * p * a / es * ((a / x).acosh() - (1.0 - (x / a).powi(2)).sqrt())).clamp(0.0, h)
}

/// The palm's papillary ridges (under the contact laws): 59 ± 19 µm high and
/// 485 ± 71 µm apart [M: laser profilometry of silicone prints of fingertips,
/// Stücker et al., J. Forensic Sci. 46 (2001) 857–861; the palm is friction
/// ridge skin of the same kind, taken alike: E]. A ridge's crest is then
/// λ²/(4π²Δ) = 0.20 mm round, against the 0.3 mm measured on fingertips
/// (Hamilton et al., PNAS 117 (2020)): the same order, a check.
pub const RIDGE_UM: f64 = 59.0;
pub const RIDGE_PITCH_UM: f64 = 485.0;

/// The palm's ridges pressed on the flat plate at `p` Pa (under the contact
/// laws) [D]: a wavy elastic surface, z = Δ(1 − cos 2πx/λ) with Δ half the
/// ridges' height, on a rigid flat, Westergaard's contact (Johnson, Contact
/// Mechanics §13.2). It is flattened all over at p* = π·E*·Δ/λ (76 kPa,
/// far above a hand's); below that each ridge touches over |x| < a,
/// sin²(πa/λ) = p/p*, under p(x) = (2p/sin²(πa/λ))·cos(πx/λ)·√(sin²(πa/λ) −
/// sin²(πx/λ)), and the furrow stands open between: its gap is the wave less
/// what that pressure presses the surface in, by the periodic line load's
/// displacement −(2/πE*)·ln|2 sin(πx/λ)| summed over the contact.
#[derive(Clone, Debug, PartialEq)]
pub struct Ridges {
    pub lambda: f64,
    /// Each ridge's contact half-width (µm).
    pub a: f64,
    /// The gap from a ridge's middle (0) to the furrow's (λ/2), at `dx` µm.
    pub gap: Vec<f32>,
    pub dx: f64,
}

impl Ridges {
    pub fn at(p: f64) -> Ridges {
        use std::f64::consts::PI;
        let es = SKIN_PA / (1.0 - SKIN_NU * SKIN_NU);
        let (lam, del) = (RIDGE_PITCH_UM, 0.5 * RIDGE_UM);
        let n = 240;
        let dx = 0.5 * lam / n as f64;
        let pstar = PI * es * del / lam;
        if p >= pstar {
            return Ridges { lambda: lam, a: 0.5 * lam, gap: vec![0.0; n + 1], dx };
        }
        let s2 = (p / pstar).max(0.0);
        let a = lam / PI * s2.sqrt().asin();
        let m = 2000;
        let w = 2.0 * a / m as f64;
        let ts: Vec<(f64, f64)> = (0..m).map(|i| {
            let t = -a + (i as f64 + 0.5) * w;
            let q = 2.0 * p / s2 * (PI * t / lam).cos() * (s2 - (PI * t / lam).sin().powi(2)).max(0.0).sqrt();
            (t, q * w)
        }).collect();
        let pressed = |x: f64| 2.0 / (PI * es) * ts.iter().map(|&(t, f)| -f * (2.0 * (PI * (x - t) / lam).sin()).abs().max(1e-12).ln()).sum::<f64>();
        let z = |x: f64| del * (1.0 - (2.0 * PI * x / lam).cos());
        let (z0, d0) = (z(a), pressed(a));
        let gap = (0..=n).map(|i| {
            let x = i as f64 * dx;
            if x <= a { 0.0 } else { (z(x) - z0 + pressed(x) - d0).max(0.0) as f32 }
        }).collect();
        Ridges { lambda: lam, a, gap, dx }
    }

    /// The furrow's gap at its middle (µm): a burr no taller fits under it.
    pub fn clearance(&self) -> f64 {
        *self.gap.last().unwrap_or(&0.0) as f64
    }

    /// How far (µm) from a ridge's middle the free skin stands `h` µm off
    /// the plate (half the pitch if the furrow is lower): a burr nearer the
    /// ridge's middle than this props it up.
    pub fn reach(&self, h: f64) -> f64 {
        if h <= 0.0 {
            return self.a;
        }
        for i in 1..self.gap.len() {
            let (g0, g1) = (self.gap[i - 1] as f64, self.gap[i] as f64);
            if g1 >= h {
                let t = if g1 > g0 { (h - g0) / (g1 - g0) } else { 1.0 };
                return ((i as f64 - 1.0 + t.clamp(0.0, 1.0)) * self.dx).max(self.a);
            }
        }
        0.5 * self.lambda
    }

    /// The share of its touches a point `d` µm from a burr `h` µm high loses
    /// to a ridge riding the burr [D, E]: a ridge whose middle lies within
    /// `reach(h)` of the burr rests on it and stands off over its own contact,
    /// so the point is held off while the ridge touching it is one so
    /// propped: over the ridge's place (uniform over a pitch as the palm
    /// slides), the overlap of |ξ| < reach(h) with |ξ − d| < a, over λ. The
    /// ridges are taken parallel to the line, where they shield the most [E:
    /// the palm's ridges run every way over it; a ridge crossing the line
    /// rests on it over a shorter stretch].
    pub fn shield(&self, h: f64, d: f64) -> f64 {
        let ah = self.reach(h);
        let (lo, hi) = ((d - self.a).max(-ah), (d + self.a).min(ah));
        ((hi - lo).max(0.0) / self.lambda).min(1.0)
    }
}

/// Points a cell is sampled at across (and down) for a burr's gap and lee
/// (`Plate::contact`): its sub-cell detail, so that a line's lee is as smooth
/// across cells as the line.
const SUB: usize = 4;

/// What a burr does to a contact at one point (`Plate::gaps_of`): for the
/// cloth, its lift `v` (µm) and, over a burr's top, the face's lift off that
/// raised base `top` (µm, below zero: squeezed harder); for the palm, the
/// share of its touches a ridge riding a burr holds off `v`, the gap then
/// `aux` (µm), and `top` 0 over a burr's top. `top` is infinite elsewhere.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Prop {
    v: f32,
    aux: f32,
    top: f32,
    fin: f32,
}

const NO_PROP: Prop = Prop { v: 0.0, aux: 0.0, top: f32::INFINITY, fin: 0.0 };

/// Props over cells near a burr (`Plate::gaps_of`): a slot of `SUB`² points
/// for each cell that has one.
struct Gaps {
    idx: Vec<u32>,
    slots: Vec<[Prop; SUB * SUB]>,
}

impl Gaps {
    fn get(&self, i: usize) -> Option<&[Prop; SUB * SUB]> {
        self.idx.get(i).and_then(|&k| (k != u32::MAX).then(|| &self.slots[k as usize]))
    }
}

/// How a contact meets the ink at a point near a burr (`Contact::touch`), for
/// the film on the flat or the ink in the openings: it touches it for `r` of
/// a full touch (the cloth: the share of its fibres in the film; the palm: the
/// share of its touches in contact), and for `off` more of a touch it stands
/// `g` µm off, reaching only the ink above that; `hair` is the share of a
/// touch its loose fibre ends hang where the face stands clear (the cloth in a
/// true gap).
#[derive(Clone, Copy, Debug, PartialEq)]
struct Touch {
    r: f64,
    off: f64,
    g: f64,
    hair: f64,
}

const FULL_TOUCH: Touch = Touch { r: 1.0, off: 0.0, g: 0.0, hair: 0.0 };

/// A piece of a burr standing in one cell (`Plate::contact`): its middle (µm
/// on the plate), its line's way, its half-length (the cell's share of the
/// line: consecutive cells of a straight line tile it), its height and its
/// top's half-width (µm), and the way it is crossed (a roulette's ring: from
/// every way, `ring`).
struct Fence {
    x: f64,
    y: f64,
    t: (f64, f64),
    half: f64,
    h: f64,
    c: f64,
    ring: bool,
    /// Its faces' reach across from its crest (µm): `ei` toward its groove,
    /// `eo` away from it (`Plate::across`); and which way across (along n =
    /// (−t.1, t.0)) is away: ±1.
    ei: f64,
    eo: f64,
    out: f64,
}

/// A line across (`Plate::across`): its centreline's offset from a cell's
/// middle, its groove's width and its fin's base on each side (µm).
struct Across {
    o: f64,
    w: f64,
    base: [f64; 2],
    /// The groove's section (µm²).
    area: f64,
}

impl Fence {
    /// The distance (µm) from (x, y) to the piece.
    #[cfg(test)]
    fn dist(&self, x: f64, y: f64) -> f64 {
        self.signed(x, y).abs()
    }

    /// The distance (µm) from (x, y) to the piece, below zero on its
    /// groove's side of its crest.
    fn signed(&self, x: f64, y: f64) -> f64 {
        let (dx, dy) = (x - self.x, y - self.y);
        if self.ring {
            return dx.hypot(dy);
        }
        let along = (dx * self.t.0 + dy * self.t.1).abs();
        let across = (dy * self.t.0 - dx * self.t.1) * self.out;
        if along <= self.half { across } else { (along - self.half).hypot(across).copysign(across) }
    }

    /// The fin's surface (µm over the plate) `s` µm from its crest (`signed`).
    fn fin(&self, s: f64) -> f64 {
        fin_um(self.h, self.ei, self.eo, s)
    }
}

/// How deep (µm) a soft cloth's fibres draw ink of viscosity `eta` up in
/// `t` s: Lucas–Washburn, √(γ·r·t/(2η)).
pub fn washburn_um(eta: f64, t: f64) -> f64 {
    (OIL_TENSION * MUSLIN_PORE_UM * 1e-6 * t / (2.0 * eta.max(1e-6))).sqrt() * 1e6
}

/// A plate's thermal time constant (s) cooling in air: ρ·c·t/h.
pub fn cool_s(metal: Metal, thickness_mm: f64) -> f64 {
    let rho_c = match metal {
        Metal::Copper => 8960.0 * 385.0,
        Metal::Zinc => 7140.0 * 388.0,
    };
    rho_c * thickness_mm * 1e-3 / COOL_W_M2K
}

// ---------------------------------------------------------------- the ink

/// An etching ink: tubes from the box ground in burnt plate oil.
#[derive(Clone, Debug, PartialEq)]
pub struct Ink {
    pub name: String,
    /// Its tubes and their parts by volume.
    pub tubes: Vec<(String, f32)>,
    /// Plate oil added beyond the tubes' paste: its share of the ink's volume.
    pub oil: f32,
    /// The plate oil's grade, 0 light (#00) .. 1 heavy (#3).
    pub stiff: f32,
    /// The pigment's share of the ink's volume.
    pub pvc: f32,
    /// Kubelka–Munk absorption and scattering per coat (25 µm), linear RGB.
    pub k: [f32; 3],
    pub s: [f32; 3],
    /// Its pigments' drying rate (`Palette::drying_of`), by volume.
    pub drying: f32,
}

impl Ink {
    /// An ink of `tubes` (names and parts) from `pal`, with `oil` of plate oil
    /// of grade `stiff` added. Err: a tube the box hasn't.
    pub fn of_tubes(pal: &crate::palette::Palette, tubes: &[(String, f32)], oil: f32, stiff: f32, name: String) -> Result<Ink, String> {
        let total: f32 = tubes.iter().map(|t| t.1.max(0.0)).sum();
        if tubes.is_empty() || total <= 0.0 {
            return Err("an ink needs a tube and its parts".into());
        }
        let (mut k, mut s, mut pvc, mut drying) = ([0.0f32; 3], [0.0f32; 3], 0.0f32, 0.0f32);
        for (n, parts) in tubes {
            let i = pal.tubes.iter().position(|t| t.name == n.as_str()).ok_or_else(|| format!("the box {:?} has no tube {n:?}", pal.name))?;
            let t = &pal.tubes[i];
            let share = parts.max(0.0) / total;
            let pig = crate::pigment::Pigment::masstone(t.color, crate::pigment::scatter_for(crate::color::luminance(t.color), t.hiding));
            for q in 0..3 {
                k[q] += share * pig.k[q];
                s[q] += share * pig.s[q];
            }
            let (_, _, oil_share) = crate::palette::tube_packing(t, pal.name);
            pvc += share * (1.0 - oil_share);
            drying += share * pal.drying_of(t);
        }
        // the plate oil added thins the pigment: its optics per coat and its share alike
        let thin = 1.0 - oil.clamp(0.0, 0.9);
        Ok(Ink { name, tubes: tubes.to_vec(), oil, stiff, pvc: pvc * thin, k: k.map(|v| v * thin), s: s.map(|v| v * thin), drying })
    }

    /// The ink as the oil paint it prints: its tubes from `pal` mixed by
    /// their shares (its parts over their sum, as `of_tubes` takes them),
    /// thinned with its plate oil as a brush's pile is with medium, drying
    /// at its tubes' rate (`Mixture::laid`). Err: a tube the box hasn't.
    pub fn paint(&self, pal: &crate::palette::Palette) -> Result<crate::wet::Paint, String> {
        let total: f32 = self.tubes.iter().map(|t| t.1.max(0.0)).sum();
        if total <= 0.0 {
            return Err(format!("the ink {:?} has no parts", self.name));
        }
        let parts = self.tubes.iter().map(|(n, p)| {
            pal.tubes.iter().position(|t| t.name == n.as_str()).map(|i| (i, p.max(0.0) / total)).ok_or_else(|| format!("the ink {:?} is made of tubes this box hasn't", self.name))
        }).collect::<Result<Vec<_>, _>>()?;
        Ok(pal.pile(parts).laid(self.oil))
    }

    /// Its viscosity (Pa·s) at `temp_c`: the plate oil's, raised by its
    /// pigment (`krieger`), lowered by warmth (`warmth`).
    pub fn eta(&self, temp_c: f64) -> f64 {
        plate_oil_pa_s(self.stiff as f64) * krieger(self.pvc as f64) * warmth(temp_c)
    }

    /// How lean the film dries, as `drying::rate` takes stiffness: an ink
    /// with no oil added is stiff paste, one with half its volume added oil
    /// is fat [E: `drying::rate`'s FAT spans that].
    fn lean(&self) -> f32 {
        (1.0 - 2.0 * self.oil).clamp(0.0, 1.0)
    }

    fn pigment(&self) -> crate::pigment::Pigment {
        crate::pigment::Pigment { k: self.k, s: self.s }
    }
}

/// What one cell holds of ink.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InkCell {
    /// Ink in the openings, per width bin (µm over the cell).
    pub line: [f32; BINS],
    /// The film on the flat (µm over the cell).
    pub film: f32,
    /// The ink's cure (`drying`: 0 fresh, `GEL` at its gel point, 1 touch-dry).
    pub cure: f32,
    /// Which inks, as volume shares of 65535.
    pub mix: [u16; INKS],
}

impl InkCell {
    pub fn total(&self) -> f32 {
        self.line.iter().sum::<f32>() + self.film
    }

    pub fn lines(&self) -> f32 {
        self.line.iter().sum()
    }

    /// Its mix as shares (0..1).
    pub fn shares(&self) -> [f32; INKS] {
        self.mix.map(|m| m as f32 / 65535.0)
    }

    /// Mix `add` µm of ink of shares `of` (and cure `cure`) into the cell's
    /// mixture, which held `had` µm.
    fn blend(&mut self, had: f32, add: f32, of: &[f32; INKS], cure: f32) {
        let t = had + add;
        if t <= 0.0 || add <= 0.0 {
            return;
        }
        let old = self.shares();
        let mut v = [0.0f32; INKS];
        for i in 0..INKS {
            v[i] = (old[i] * had + of[i] * add) / t;
        }
        self.mix = quantize(&v);
        self.cure = (self.cure * had + cure * add) / t;
    }
}

/// Shares (summing to 1) as u16 parts of 65535, summing to it exactly.
fn quantize(v: &[f32; INKS]) -> [u16; INKS] {
    let sum: f32 = v.iter().sum();
    if sum <= 0.0 {
        return [0; INKS];
    }
    let mut out = v.map(|x| ((x / sum) * 65535.0).round().clamp(0.0, 65535.0) as u16);
    let got: i64 = out.iter().map(|&x| x as i64).sum();
    let big = (0..INKS).max_by(|&a, &b| out[a].cmp(&out[b])).unwrap();
    out[big] = (out[big] as i64 + 65535 - got).clamp(0, 65535) as u16;
    out
}

/// The plate's ink: its inks, the cells' ink in tiles beside the cells, its
/// warmth, and the clock its ink is aged to.
#[derive(Clone, Debug, PartialEq)]
pub struct Inking {
    pub inks: Vec<Ink>,
    tiles: Vec<Option<Box<[InkCell]>>>,
    /// The temperature it was last warmed to (°C) and when it came off the
    /// hot plate (minutes on its clock); it cools in air from then.
    pub warm_c: f32,
    pub warm_at: f64,
    /// The clock (minutes) its ink's cure is brought up to.
    pub aged_to: f64,
}

/// How ink goes on: a card's edge dragged along the points, or a dabber's or
/// a poupée's face dabbed at each point or (`InkTool::moved`) rubbed along
/// them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inker {
    Card,
    Dabber,
    Poupee,
}

/// An inking tool: what it is, its size (the card's edge's length, the
/// face's diameter, mm) and whether a face is rubbed along the points rather
/// than dabbed at each.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InkTool {
    pub by: Inker,
    pub width_mm: f64,
    pub moved: bool,
}

impl InkTool {
    /// The tool `by` at its usual size, dabbed.
    pub fn of(by: Inker) -> InkTool {
        InkTool { by, width_mm: match by { Inker::Card => CARD_MM, Inker::Dabber => DABBER_MM, Inker::Poupee => POUPEE_MM }, moved: false }
    }

    /// The force (N) it is pressed with when none is given: the card's 10 N, the
    /// dabber's 20 N, a poupée's the dabber's pressure over its face.
    pub fn force(&self) -> f64 {
        match self.by {
            Inker::Card => CARD_N,
            _ => DABBER_PA * self.face_m2(),
        }
    }

    fn face_m2(&self) -> f64 {
        std::f64::consts::PI * 0.25 * (self.width_mm * 1e-3).powi(2)
    }

    /// Under the contact laws: the diameter (mm) a dabbed or rubbed face
    /// touches pressed with `f` N. Its size is its contact at the force it is
    /// pressed with when none is given (`force`); a rounded elastic body's
    /// contact grows as the cube root of the force (Hertz) [D].
    pub fn contact_mm(&self, f: f64) -> f64 {
        self.width_mm * (f.max(0.0) / self.force().max(1e-12)).cbrt()
    }

    /// The pressure (Pa) under it pressed with `f` N: over the card's bead, or
    /// over the face pressed flat.
    pub fn pressure(&self, f: f64) -> f64 {
        match self.by {
            Inker::Card => f / (self.width_mm * 1e-3 * CARD_BEAD_MM * 1e-3),
            _ => f / self.face_m2(),
        }
    }
}

/// What an inking did.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InkReport {
    /// Ink put on the plate (µm³), the film left on the flat (µm), the
    /// openings' mean fill (share of their volume), the plate's temperature
    /// and the ink's viscosity there, the area inked (mm²) and hand time (s).
    pub added_um3: f64,
    pub film_um: f64,
    pub fill: f64,
    pub temp_c: f64,
    pub eta: f64,
    pub area_mm2: f64,
    pub secs: f64,
}

/// What a wipe did: the plate's ink before and after and where the
/// difference went (µm³).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WipeReport {
    pub before_um3: f64,
    pub after_um3: f64,
    /// Taken up by the cloth or palm, and laid back by it (on the plate).
    pub taken_um3: f64,
    pub laid_um3: f64,
    /// What the cloth or palm kept: taken − laid = before − after.
    pub kept_um3: f64,
    /// The film left on the flat, mean over the area (µm), and the ink left
    /// in the openings (µm over the area).
    pub film_um: f64,
    pub lines_um: f64,
    pub area_mm2: f64,
    pub secs: f64,
    /// The plate's temperature (°C).
    pub temp_c: f64,
}

/// The tracks a cloth's face draws over `cells` moving `v`: each point of
/// the face slides along `v`, so what it carries stays on its line. The
/// cells as (track: their row across `v`, a cell wide; their place along
/// `v`, in cells; their index in `cells`), track by track, each in the
/// order the face passes them.
fn tracks(cells: &[(usize, usize, f32)], v: (f64, f64)) -> Vec<(i64, f64, usize)> {
    let mut t: Vec<(i64, f64, usize)> = cells.iter().enumerate().map(|(i, &(x, y, _))| {
        let (px, py) = (x as f64 + 0.5, y as f64 + 0.5);
        ((py * v.0 - px * v.1).floor() as i64, px * v.0 + py * v.1, i)
    }).collect();
    t.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.total_cmp(&b.1)));
    t
}

/// A tarlatan's own state beside its rag's face (`Rag::cloth`).
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "replay-effects", derive(serde::Serialize, serde::Deserialize))]
pub struct Tarlatan {
    /// Starch, 0 soft .. 1 stiff, and its bending rigidity (N·m, `BEND_NM`).
    pub stiff: f32,
    pub bend_nm: f64,
    /// Ink wet at the face's surface (µm³), which it can lay back, and its
    /// shares of the plate's inks.
    pub pool_um3: f64,
    pub pool_mix: [f32; INKS],
    /// All the ink the cloth holds (µm³).
    pub held_um3: f64,
}

impl Rag {
    /// A tarlatan bunched to a pad `width_mm` across, starched `stiff` (0
    /// soft .. 1 stiff): a rag whose cloth is a tarlatan's.
    pub fn tarlatan(width_mm: f32, seed: u64, stiff: f32) -> Rag {
        let mut r = Rag::new(width_mm, seed);
        r.cloth = Some(Box::new(Tarlatan::new(stiff)));
        r
    }
}

impl Tarlatan {
    pub fn new(stiff: f32) -> Tarlatan {
        let (a, b) = BEND_NM;
        Tarlatan { stiff, bend_nm: a * (b / a).powf(stiff.clamp(0.0, 1.0) as f64), ..Tarlatan::default() }
    }
}

/// A wipe's cloth or palm: how it reaches, holds off and takes.
struct Contact {
    /// Reach (µm) into an opening `w` wide at `x` from its rim.
    reach: Box<dyn Fn(f64, f64) -> f64>,
    /// How far (µm) it is held off past a burr `h` high.
    lee: Box<dyn Fn(f64) -> f64>,
    /// Under the contact laws: how it meets the film on the flat and the ink
    /// in the openings at a point (`Touch`) from what its `Gaps` keep there,
    /// and what the cloth's loose fibre ends draw up in a touch where it
    /// stands clear (µm of the ink in a cell, none for skin).
    touch: Box<dyn Fn(&Prop) -> (Touch, Touch)>,
    wick: Option<Box<dyn Fn(&InkCell) -> f64>>,
}

impl Plate {
    // ------------------------------------------------------------ reading

    /// The plate's ink, if it has been inked.
    pub fn inking(&self) -> Option<&Inking> {
        self.ink.as_deref()
    }

    /// Whether any ink is on the plate.
    pub fn inked(&self) -> bool {
        self.ink.as_ref().is_some_and(|k| k.tiles.iter().flatten().flat_map(|t| t.iter()).any(|c| c.total() > 0.0))
    }

    pub fn ink_cell(&self, x: usize, y: usize) -> InkCell {
        let Some(k) = self.ink.as_deref() else { return InkCell::default() };
        if x >= self.w || y >= self.h {
            return InkCell::default();
        }
        let tw = self.w.div_ceil(TILE);
        match &k.tiles[(y / TILE) * tw + x / TILE] {
            Some(t) => t[(y % TILE) * TILE + x % TILE],
            None => InkCell::default(),
        }
    }

    pub(super) fn ink_cell_mut(&mut self, x: usize, y: usize) -> &mut InkCell {
        let tw = self.w.div_ceil(TILE);
        let n = self.tiles.len();
        let k = self.ink.get_or_insert_with(|| Box::new(Inking { inks: Vec::new(), tiles: vec![None; n], warm_c: ROOM_C as f32, warm_at: 0.0, aged_to: 0.0 }));
        let t = k.tiles[(y / TILE) * tw + x / TILE].get_or_insert_with(|| vec![InkCell::default(); TILE * TILE].into_boxed_slice());
        &mut t[(y % TILE) * TILE + x % TILE]
    }

    /// The plate's ink (µm³): in its openings and on its flat.
    pub fn ink_um3(&self) -> (f64, f64) {
        let a = self.pitch * self.pitch;
        let Some(k) = self.ink.as_deref() else { return (0.0, 0.0) };
        k.tiles.iter().flatten().flat_map(|t| t.iter()).fold((0.0, 0.0), |(l, f), c| (l + c.lines() as f64 * a, f + c.film as f64 * a))
    }

    /// The plate's temperature (°C) at `now` minutes on its clock: what it
    /// was warmed to, cooled in air since (`cool_s`).
    pub fn temp_c(&self, now: f64) -> f64 {
        let Some(k) = self.ink.as_deref() else { return ROOM_C };
        let dt = ((now - k.warm_at) * 60.0).max(0.0);
        ROOM_C + (k.warm_c as f64 - ROOM_C) * (-dt / cool_s(self.metal, self.thickness_mm as f64)).exp()
    }

    // ------------------------------------------------------------ the clock

    /// Put the plate on the hot plate at `temp_c` until `now` (minutes): it
    /// comes off at that temperature and cools from then.
    pub fn warm(&mut self, temp_c: f64, now: f64) {
        self.age_ink(now);
        let _ = self.ink_cell_mut(0, 0);
        let k = self.ink.as_deref_mut().unwrap();
        k.warm_c = temp_c as f32;
        k.warm_at = now;
    }

    /// Bring the ink's cure up to `now` (minutes on the plate's clock): each
    /// cell's ink dries as an oil film of its thickness (`drying::rate`),
    /// at engine 3's pace (which every later engine keeps).
    pub fn age_ink(&mut self, now: f64) {
        let Some(k) = self.ink.as_deref_mut() else { return };
        let dt = (now - k.aged_to) as f32;
        if dt <= 0.0 {
            return;
        }
        k.aged_to = now;
        let pace = Pace::of(3);
        let inks = k.inks.clone();
        for t in k.tiles.iter_mut().flatten() {
            for c in t.iter_mut() {
                let v = c.total();
                if v <= 0.0 || c.cure >= 1.0 {
                    continue;
                }
                let sh = c.shares();
                let (mut lean, mut drying) = (0.0f32, 0.0f32);
                for (i, ink) in inks.iter().enumerate() {
                    lean += sh[i] * ink.lean();
                    drying += sh[i] * ink.drying;
                }
                let r = crate::drying::rate(v / COAT_UM, lean, drying);
                c.cure = pace.age(c.cure, dt, r).min(1.0);
            }
        }
    }

    /// Register `ink` among the plate's inks: its index. Err: the plate
    /// holds `INKS` already.
    fn ink_index(&mut self, ink: &Ink) -> Result<usize, String> {
        let _ = self.ink_cell_mut(0, 0);
        let k = self.ink.as_deref_mut().unwrap();
        if let Some(i) = k.inks.iter().position(|i| i == ink) {
            return Ok(i);
        }
        if k.inks.len() >= INKS {
            return Err(format!("a plate holds at most {INKS} inks"));
        }
        k.inks.push(ink.clone());
        Ok(k.inks.len() - 1)
    }

    /// The cells the stretches touch, each once (the union), as (x, y).
    fn union(&self, stretches: &[super::hand::Stretch]) -> Vec<(usize, usize)> {
        let mut seen = vec![false; self.w * self.h];
        let mut out = Vec::new();
        for s in stretches {
            for &(x, y, m) in &s.cells {
                let i = y * self.w + x;
                if m > 0.0 && !seen[i] {
                    seen[i] = true;
                    out.push((x, y));
                }
            }
        }
        out
    }

    // ------------------------------------------------------------ inking

    /// Ink the plate with `tool` moved along `pts` (mm; each point's force
    /// the hand's, N, and its speed, mm/s) at `now` (IP §5.2). The ink goes
    /// only where the tool touches: a cell the card's edge passes over, or a
    /// dabbed face's disc covers, over half; a cell a face rubbed along slides
    /// over.
    /// - Each opening there fills to its share (`fill`: the ink pushed in
    ///   viscously or against its yield stress, at most `PHI_AIR`), by the
    ///   tool's pressure (`InkTool::pressure`) over the time the cell spends
    ///   under it: the card's bead `CARD_RUN_MM` over its speed each time its
    ///   edge passes, a dab `DAB_S`, a face rubbed along (`Plate::slide`) the
    ///   length it slides over the cell over its speed. The pushes add (the depth pushed goes as
    ///   √(p·t)).
    /// - The flat keeps the film the card's edge drags at its speed and
    ///   pressure (`card_film_um`) or a face leaves (`DABBER_FILM_UM`).
    pub fn ink(&mut self, ink: &Ink, tool: InkTool, pts: &[Point], now: f64) -> Result<InkReport, String> {
        self.age_ink(now);
        let idx = self.ink_index(ink)?;
        let temp = self.temp_c(now);
        let eta = ink.eta(temp);
        let w = tool.width_mm;
        // (the contact laws: a face is a smooth compliant body, its contact as wide as
        // `InkTool::contact_mm` at the hand's force and pressed by Hertz's law)
        let soft = self.contact && tool.by != Inker::Card;
        let stretches = match tool.by {
            Inker::Card => self.sweep(pts, &|_| w, 0.25 * w),
            _ if tool.moved && soft => self.slide_pressed(pts, &|pt| tool.contact_mm(pt.force as f64), 0.25 * w),
            _ if tool.moved => self.slide(pts, &|_| w, 0.25 * w),
            _ if soft => self.dabs(pts, &|pt| tool.contact_mm(pt.force as f64)),
            _ => self.dabs(pts, &|_| w),
        };
        let mut of = [0.0f32; INKS];
        of[idx] = 1.0;
        let before = self.ink_um3();
        // per cell touched: Σ p·t, the most pressure, the film
        let mut acc: std::collections::BTreeMap<(usize, usize), (f64, f64, f64)> = std::collections::BTreeMap::new();
        let (mut swept, mut film_max) = (0.0f64, 0.0f64);
        for st in &stretches {
            let f = st.pose.force as f64;
            let speed = (st.pose.speed as f64).max(1.0);
            let p = if soft { f / (std::f64::consts::PI * 0.25 * (st.width_mm * 1e-3).powi(2)).max(1e-12) } else { tool.pressure(f) };
            let film = if tool.by == Inker::Card { card_film_um(eta, speed, p) } else { DABBER_FILM_UM };
            film_max = film_max.max(film);
            swept += st.area_mm2(self.pitch);
            let pitch = self.pitch * 1e-3;
            for &(x, y, m) in &st.cells {
                if soft {
                    // the face pressed by Hertz's law: at a dab, the pressure at the cell; rubbed
                    // along, the most it meets, under the stretch's line nearest it. It pushes ink
                    // in by Σ p·t, and lays the film it carries over the share of the cell its
                    // texture touches through the ink, which grows with the load until it is
                    // pressed into the charge [E: taken at the dabber's working pressure,
                    // `DABBER_PA`, unmeasured; Greenwood–Williamson below it]
                    let (cx, cy) = ((x as f64 + 0.5) * pitch - st.pose.x as f64, (y as f64 + 0.5) * pitch - st.pose.y as f64);
                    let a = 0.5 * st.width_mm;
                    let (pl, dwell) = if tool.moved {
                        let u = (cy * st.v.0 - cx * st.v.1).abs();
                        (p * Plate::hertz(u, a), m as f64 * st.width_mm / speed)
                    } else if m > 0.0 {
                        (p * Plate::hertz(cx.hypot(cy), a), DAB_S)
                    } else {
                        continue;
                    };
                    if pl <= 0.0 {
                        continue;
                    }
                    // rubbed along, Σ p·t is the mean pressure over the pressure-weighted share
                    let pt = if tool.moved { p * dwell } else { pl * dwell };
                    let e = acc.entry((y, x)).or_insert((0.0, 0.0, 0.0));
                    e.0 += pt;
                    e.1 = e.1.max(pl);
                    e.2 = e.2.max(DABBER_FILM_UM * (pl / DABBER_PA).min(1.0));
                    continue;
                }
                // the time under it: the card's bead each pass, a dab, a face rubbed along for the
                // length it slides over the cell (its share of a pass, `slide`, times its width)
                let dwell = match tool.by {
                    Inker::Card if m >= 0.5 => CARD_RUN_MM / speed,
                    Inker::Card => continue,
                    _ if tool.moved => m as f64 * w / speed,
                    _ if m >= 0.5 => DAB_S,
                    _ => continue,
                };
                let e = acc.entry((y, x)).or_insert((0.0, 0.0, 0.0));
                e.0 += p * dwell;
                e.1 = e.1.max(p);
                e.2 = e.2.max(film);
            }
        }
        let (mut fsum, mut fvol) = (0.0f64, 0.0f64);
        for (&(y, x), &(pt, p, film)) in &acc {
            let c = self.cell(x, y);
            let mut ic = self.ink_cell(x, y);
            let had = ic.total();
            for k in 0..BINS {
                if c.open[k] <= 0.0 || c.vol[k] <= 0.0 {
                    continue;
                }
                let (w, d) = ((c.wsum[k] / c.open[k]) as f64, (c.vol[k] / c.open[k]) as f64);
                let phi = fill(w, d, p, pt / p.max(1e-9), eta, YIELD_PA);
                fsum += phi * c.vol[k] as f64;
                fvol += c.vol[k] as f64;
                ic.line[k] = ic.line[k].max((phi * c.vol[k] as f64) as f32);
            }
            // the flat: the card scrapes it to its film; a dabber adds to a thinner one
            let flat = (1.0 - c.open_share().min(1.0)) as f64;
            ic.film = ic.film.max((film * flat) as f32);
            let add = ic.total() - had;
            ic.blend(had, add, &of, 0.0);
            *self.ink_cell_mut(x, y) = ic;
        }
        let after = self.ink_um3();
        let area = acc.len() as f64 * (self.pitch * 1e-3).powi(2);
        let secs = swept / match tool.by {
            Inker::Card => pace::CARD_PASS_MM2_S,
            _ if tool.moved => pace::CARD_PASS_MM2_S,
            _ => pace::DAB_MM2_S,
        };
        Ok(InkReport { added_um3: after.0 + after.1 - before.0 - before.1, film_um: film_max, fill: if fvol > 0.0 { fsum / fvol } else { 0.0 }, temp_c: temp, eta, area_mm2: area, secs })
    }

    /// A rag dampened with solvent, a pad `pad_mm` across, rubbed along `pts`
    /// (mm; each point's speed its own, mm/s) `passes` times at `now` (EP §8,
    /// "Cleaning"). The pad slides along the points as a tarlatan does
    /// (`Plate::slide`; pressed soft under the contact laws), and each cell
    /// gets its share `m` of a pass each time:
    /// - **the film on the flat**: the rubbing mixes the solvent the face
    ///   lays (`etch::SOLVENT_UM`) into the film f it meets, and the cloth
    ///   drinks the mixture but for the film a wiped face keeps
    ///   (`etch::RESIDUE_UM`): it keeps RESIDUE/(f + SOLVENT) of its ink a
    ///   whole pass (f the film the pass came to), that to the power m for a
    ///   share of one [D];
    /// - **the ink in an opening**: the solvent fills the opening above the
    ///   ink and joins it to the cloth, whose pores draw the ink up by
    ///   capillarity as retroussage's fibres do (Lucas–Washburn,
    ///   `washburn_um`, at the ink's own viscosity at the plate's warmth,
    ///   for the time the pad lies over the point, m·pad/speed): the ink
    ///   sinks by that depth [D]. A stiff ink comes out of deep lines more
    ///   slowly than a loose one, and a warm plate gives it up sooner.
    ///
    /// A cell whose ink comes below an oil's monolayer
    /// (`etch::MONOLAYER_UM`) holds none. What the rag takes it keeps: a
    /// solvent rag is refolded as it fills and lays nothing back. Nothing
    /// is taken where the rag doesn't go.
    pub fn clean_ink(&mut self, pts: &[Point], pad_mm: f64, passes: u32, now: f64) -> WipeReport {
        self.age_ink(now);
        let temp = self.temp_c(now);
        let before = self.ink_um3();
        let pad = pad_mm.max(0.1);
        let stretches = if self.contact { self.slide_pressed(pts, &|_| pad, 0.25 * pad) } else { self.slide(pts, &|_| pad, 0.25 * pad) };
        let inks: Vec<Ink> = self.ink.as_ref().map(|k| k.inks.clone()).unwrap_or_default();
        let unit = self.pitch * self.pitch;
        let (hs, hr, mono) = (crate::etch::SOLVENT_UM, crate::etch::RESIDUE_UM, crate::etch::MONOLAYER_UM);
        let mut swept = 0.0f64;
        for _ in 0..passes {
            // (the film each cell had as the pass came to it, and the time the pad has lain over
            // it so far in the pass: a pass keeps RESIDUE/(f + SOLVENT) of that film, and draws
            // the lines' ink up by Washburn over its whole time there, shared out over the
            // stretches that rub it, so a pass is one pass however its path is cut up)
            let mut met: std::collections::HashMap<(usize, usize), (f64, f64)> = std::collections::HashMap::new();
            for st in &stretches {
                let secs = pad / (st.pose.speed as f64).max(1e-3);
                for &(x, y, m) in &st.cells {
                    swept += m as f64 * unit;
                    let ic = self.ink_cell(x, y);
                    if ic.total() <= 0.0 || m <= 0.0 {
                        continue;
                    }
                    let c = self.cell(x, y);
                    let sh = ic.shares();
                    let eta: f64 = inks.iter().enumerate().map(|(i, k)| sh[i] as f64 * k.eta(temp)).sum::<f64>().max(1e-6);
                    let mut out = ic;
                    let f = ic.film as f64;
                    let seen = met.entry((x, y)).or_insert((f, 0.0));
                    let (f0, t0) = *seen;
                    seen.1 += m as f64 * secs;
                    let draw = washburn_um(eta, t0 + m as f64 * secs) - washburn_um(eta, t0);
                    out.film = (f * (hr / (f0 + hs)).min(1.0).powf(m as f64)) as f32;
                    for k in 0..BINS {
                        let open = c.open[k] as f64;
                        if out.line[k] <= 0.0 || open <= 0.0 {
                            continue;
                        }
                        let depth = out.line[k] as f64 / open;
                        out.line[k] = (open * (depth - draw).max(0.0)) as f32;
                    }
                    if out.film < mono as f32 {
                        out.film = 0.0;
                    }
                    for k in 0..BINS {
                        if (out.line[k] as f64) < mono * (c.open[k] as f64) {
                            out.line[k] = 0.0;
                        }
                    }
                    *self.ink_cell_mut(x, y) = out;
                }
            }
        }
        let after = self.ink_um3();
        let taken = (before.0 + before.1) - (after.0 + after.1);
        let area_mm2 = self.union(&stretches).len() as f64 * unit * 1e-6;
        self.wipe_report(before, after, taken, 0.0, area_mm2, swept * 1e-6 / pace::TARLATAN_MM2_S, temp)
    }

    // ------------------------------------------------------------ wiping

    /// Each wide opening's distance from its rim (µm), per cell and bin:
    /// bins whose openings are at least two cells wide, by a chamfer
    /// distance over the cells at least half open in that bin.
    fn rims(&self) -> Vec<Option<Vec<f32>>> {
        let p = self.pitch;
        let mut out = vec![None; BINS];
        for (k, slot) in out.iter_mut().enumerate() {
            // (the smallest width in the bin)
            let lo = if k == 0 { 0.0 } else { 16.0 * 2f64.powi(k as i32) };
            if lo < 2.0 * p {
                continue;
            }
            let inside: Vec<bool> = (0..self.w * self.h).map(|i| self.cell(i % self.w, i / self.w).open[k] >= 0.5).collect();
            if !inside.iter().any(|&b| b) {
                continue;
            }
            let big = f32::MAX / 4.0;
            let mut d: Vec<f32> = inside.iter().map(|&b| if b { big } else { 0.0 }).collect();
            let (w, h) = (self.w, self.h);
            let at = |x: i64, y: i64, d: &Vec<f32>| if x < 0 || y < 0 || x >= w as i64 || y >= h as i64 { 0.0 } else { d[y as usize * w + x as usize] };
            for y in 0..h as i64 {
                for x in 0..w as i64 {
                    let i = y as usize * w + x as usize;
                    if d[i] == 0.0 {
                        continue;
                    }
                    let v = (at(x - 1, y, &d) + 1.0).min(at(x, y - 1, &d) + 1.0).min(at(x - 1, y - 1, &d) + 1.4142).min(at(x + 1, y - 1, &d) + 1.4142);
                    d[i] = d[i].min(v);
                }
            }
            for y in (0..h as i64).rev() {
                for x in (0..w as i64).rev() {
                    let i = y as usize * w + x as usize;
                    if d[i] == 0.0 {
                        continue;
                    }
                    let v = (at(x + 1, y, &d) + 1.0).min(at(x, y + 1, &d) + 1.0).min(at(x + 1, y + 1, &d) + 1.4142).min(at(x - 1, y + 1, &d) + 1.4142);
                    d[i] = d[i].min(v);
                }
            }
            *slot = Some(d.iter().map(|&c| ((c as f64 - 0.5).max(0.0) * p) as f32).collect());
        }
        out
    }

    /// The share of each cell (flat, openings) the cloth moving `v` can't
    /// reach near a burr (IP §1.5): pressed over the burr as over a fence,
    /// it is held off the plate for `lee(h)` on either side of it (a plate
    /// bent over a support lifts on both sides), to the extent it crosses the
    /// line (|sin φ|, φ between its way and the line's): wiping along a line
    /// its drape slides along the burr and wipes beside it. The burr's own
    /// cell, and the line's openings beside it, are under the drape. A ring
    /// (a roulette's pit) is crossed square from every way.
    fn shields(&self, v: (f64, f64), lee: &dyn Fn(f64) -> f64) -> Option<(Vec<f32>, Vec<f32>)> {
        let p = self.pitch;
        let mut flat = vec![0.0f32; self.w * self.h];
        let mut open = vec![0.0f32; self.w * self.h];
        let mut any = false;
        let tw = self.w.div_ceil(TILE);
        for ty in 0..self.h.div_ceil(TILE) {
            for tx in 0..tw {
                if self.tiles[ty * tw + tx].is_none() {
                    continue;
                }
                for y in ty * TILE..((ty + 1) * TILE).min(self.h) {
                    for x in tx * TILE..((tx + 1) * TILE).min(self.w) {
                        let c = self.cell(x, y);
                        let hb = c.burr_height() as f64;
                        if hb < 0.5 || c.side == 0 {
                            continue;
                        }
                        any = true;
                        // the line's way (a ring: square to the cloth), and how squarely the cloth crosses it
                        let t = if c.dir == 255 { (-v.1, v.0) } else {
                            let a = (c.dir as f64).to_radians();
                            (a.cos(), a.sin())
                        };
                        let sinp = (t.0 * v.1 - t.1 * v.0).abs();
                        if sinp <= 1e-9 {
                            continue;
                        }
                        let n = (-t.1, t.0);
                        let l = lee(hb);
                        let span = (l / p).ceil() as i64 + 1;
                        for dy in -span..=span {
                            for dx in -span..=span {
                                let (xx, yy) = (x as i64 + dx, y as i64 + dy);
                                if xx < 0 || yy < 0 || xx >= self.w as i64 || yy >= self.h as i64 {
                                    continue;
                                }
                                let (ox, oy) = (dx as f64 * p, dy as f64 * p);
                                let across = (ox * n.0 + oy * n.1).abs();
                                let along = (ox * t.0 + oy * t.1).abs();
                                // this fence's piece is one cell long; the drape is `l` wide each side of it
                                if along > 0.5 * p + 1e-9 {
                                    continue;
                                }
                                let inside = ((l - (across - 0.5 * p)) / p).clamp(0.0, 1.0);
                                let s = (sinp * inside) as f32;
                                let i = yy as usize * self.w + xx as usize;
                                flat[i] = flat[i].max(s);
                                open[i] = open[i].max(s);
                            }
                        }
                    }
                }
            }
        }
        any.then_some((flat, open))
    }

    /// The burr's pieces (`Fence`), as `shields` reads them: under the contact
    /// laws, a cell's piece of a line's fin for each side its burr stands on.
    ///
    /// The fin stands where it was thrown, beside its groove: its base runs
    /// from the groove's rim out (`Plate::deposit`), its inner face the
    /// groove's wall carried on up (`fin_um`), so its crest lies w/2 + ei from
    /// the line's centreline, w the groove's width and ei = h·w²/(4S) (the
    /// wall's slope from upright in a V of the groove's width and section S),
    /// and its outer face reaches the rest of its base out, all read from the
    /// line's cells about it (`across`). It sat on the centreline itself, with
    /// its top as wide as the cell's share of the base: a yarn resting on it
    /// rested over the groove, and the fin's own cells were wiped as the open
    /// plate.
    fn fences(&self) -> Vec<Fence> {
        let p = self.pitch;
        let mut out = Vec::new();
        let tw = self.w.div_ceil(TILE);
        for ty in 0..self.h.div_ceil(TILE) {
            for tx in 0..tw {
                if self.tiles[ty * tw + tx].is_none() {
                    continue;
                }
                for y in ty * TILE..((ty + 1) * TILE).min(self.h) {
                    for x in tx * TILE..((tx + 1) * TILE).min(self.w) {
                        let c = self.cell(x, y);
                        let hb = c.burr_height() as f64;
                        if hb < 0.5 || c.side == 0 {
                            continue;
                        }
                        let ring = c.dir == 255;
                        let t = if ring { (1.0, 0.0) } else {
                            let a = (c.dir as f64).to_radians();
                            (a.cos(), a.sin())
                        };
                        // a straight line crosses a cell every p / max(|cos|, |sin|) along it
                        let half = if ring { 0.0 } else { 0.5 * p / t.0.abs().max(t.1.abs()) };
                        let (mx, my) = ((x as f64 + 0.5) * p, (y as f64 + 0.5) * p);
                        let line = if ring { None } else { self.across(x, y, c.dir, t) };
                        match line {
                            Some(l) => {
                                // each side's fin on its own place across the line (bit 0: −n, bit 1: +n)
                                for (bit, sgn, base) in [(1u8, -1.0f64, l.base[0]), (2, 1.0, l.base[1])] {
                                    if c.side & bit == 0 || base <= 0.0 {
                                        continue;
                                    }
                                    // its inner face the groove's wall carried on up, at the slope a
                                    // V of the groove's width and section has, w²/(4S) from upright
                                    let ei = (hb * l.w * l.w / (4.0 * l.area.max(1e-6))).clamp(0.5, 0.9 * base);
                                    let o = l.o + sgn * (0.5 * l.w + ei);
                                    out.push(Fence { x: mx - o * t.1, y: my + o * t.0, t, half, h: hb, c: (0.5 * base).max(1.0), ring, ei, eo: (base - ei).max(0.5), out: sgn });
                                }
                            }
                            None => {
                                // (a ring, or a burr with no groove of its line near: at its
                                // cell's middle, its top over its share of the cell)
                                let area = c.burr_area as f64 * p * p;
                                let base = if ring { (area / std::f64::consts::PI).sqrt() } else { area / (2.0 * half) };
                                let c = (0.5 * base).clamp(1.0, 0.5 * p);
                                out.push(Fence { x: mx, y: my, t, half, h: hb, c, ring, ei: c, eo: c, out: 1.0 });
                            }
                        }
                    }
                }
            }
        }
        out
    }

    /// The line through cell (x, y), of direction `dir` (way `t`), across it
    /// (along n = (−t.1, t.0)), from its cells about it: its centreline's
    /// offset from the cell's middle (µm), its groove's width (µm) and section
    /// (µm²), and its fin's base on each side (µm; −n, +n). The line's own
    /// groove is found nearest the cell across it (a burr thrown by a
    /// leaning needle stands up to a few cells off its groove); the
    /// centreline is the centroid of the groove's volume in the cells of that
    /// line within one cell along and three and a half across of that (wide
    /// enough for the whole of a 4 N groove from its edge). The groove is laid
    /// across its width at a quarter of a cell (`Plate::deposit`), so its
    /// centroid finds the line to within that. Its width is its openings'
    /// (Σ share·width over Σ share), its length in the window Σ share·p²/w
    /// (three cells either way along), its section its openings' volume over
    /// that length, and each side's base the area of the burr joined to its
    /// groove over that length, within six cells across (a 4 N fin's toe
    /// stands 0.15 mm out). A piece of burr placed at its cell's middle
    /// instead followed a staircase of ±p/2 along a slanting line, and so did
    /// the film beside it.
    fn across(&self, x: usize, y: usize, dir: u8, t: (f64, f64)) -> Option<Across> {
        // (the width and the fins are summed over three cells either way along: a window one
        // cell either way holds one cell of a diagonal line's middle and two beside it, which
        // read its fins half as wide again as they are)
        const ALONG: f64 = 3.0;
        let p = self.pitch;
        let n = (-t.1, t.0);
        let same = |xx: i64, yy: i64| -> Option<Cell> {
            if xx < 0 || yy < 0 || xx >= self.w as i64 || yy >= self.h as i64 {
                return None;
            }
            let o = self.cell(xx as usize, yy as usize);
            let turn = (o.dir as i64 - dir as i64).rem_euclid(180);
            (o.dir != 255 && turn.min(180 - turn) <= 3).then_some(o)
        };
        // the line's groove nearest across: a step of a cell along n at a time, either way
        let mut at = None;
        'find: for j in 0..=8i64 {
            for sgn in [1i64, -1] {
                let (ox, oy) = ((j * sgn) as f64 * n.0, (j * sgn) as f64 * n.1);
                let (gx, gy) = (x as i64 + ox.round() as i64, y as i64 + oy.round() as i64);
                if same(gx, gy).is_some_and(|o| o.groove_vol() > 0.0) {
                    at = Some((gx, gy));
                    break 'find;
                }
            }
        }
        let (gx, gy) = at?;
        let (mut sv, mut so, mut open, mut wsum, mut gvol) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
        for dy in -7i64..=7 {
            for dx in -7i64..=7 {
                let (ox, oy) = (dx as f64, dy as f64);
                let along = (ox * t.0 + oy * t.1).abs();
                if along > ALONG || (ox * n.0 + oy * n.1).abs() > 3.5 {
                    continue;
                }
                if let Some(o) = same(gx + dx, gy + dy)
                    && o.groove_vol() > 0.0
                {
                    if along <= 1.0 {
                        let v = o.groove_vol() as f64;
                        sv += v;
                        so += v * (((gx + dx - x as i64) as f64 * n.0 + (gy + dy - y as i64) as f64 * n.1) * p);
                    }
                    open += o.open_share() as f64;
                    wsum += o.wsum.iter().sum::<f32>() as f64;
                    gvol += o.groove_vol() as f64;
                }
            }
        }
        if sv <= 0.0 || open <= 0.0 || wsum <= 0.0 {
            return None;
        }
        let o = so / sv;
        let w = wsum / open;
        // the window's length along the line (µm): its openings' area over their width
        let len = open * p * p / w;
        let area = gvol * p * p / len;
        // the fin on each side: the line's burr cells joined to its groove (8-connected,
        // within `ALONG` cells along and six across), so a crossing line's burr, or a parallel
        // line's beyond a gap, isn't counted as this one's
        let inside = |dx: i64, dy: i64| {
            let (ox, oy) = (dx as f64, dy as f64);
            (ox * t.0 + oy * t.1).abs() <= ALONG && (ox * n.0 + oy * n.1).abs() <= 6.0
        };
        let mut seen = [[false; 15]; 15];
        let mut todo: Vec<(i64, i64)> = Vec::new();
        for dy in -7i64..=7 {
            for dx in -7i64..=7 {
                if inside(dx, dy) && same(gx + dx, gy + dy).is_some_and(|o| o.groove_vol() > 0.0) {
                    seen[(dy + 7) as usize][(dx + 7) as usize] = true;
                    todo.push((dx, dy));
                }
            }
        }
        let mut base = [0.0f64; 2];
        while let Some((dx, dy)) = todo.pop() {
            let Some(c) = same(gx + dx, gy + dy) else { continue };
            for (k, bit) in [1u8, 2].into_iter().enumerate() {
                if c.side & bit != 0 && c.burr_area > 0.0 {
                    // (a cell holding both sides' burr gives each half its area)
                    let share = if c.side == 3 { 0.5 } else { 1.0 };
                    base[k] += share * c.burr_area as f64 * p * p / len;
                }
            }
            for (ex, ey) in [(-1i64, -1i64), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
                let (nx, ny) = (dx + ex, dy + ey);
                if nx.abs() > 7 || ny.abs() > 7 || !inside(nx, ny) || seen[(ny + 7) as usize][(nx + 7) as usize] {
                    continue;
                }
                if same(gx + nx, gy + ny).is_some_and(|o| o.burr_area > 0.0) {
                    seen[(ny + 7) as usize][(nx + 7) as usize] = true;
                    todo.push((nx, ny));
                }
            }
        }
        Some(Across { o, w, base, area })
    }

    /// Each cell within `reach(piece)` of a piece of burr, with what the
    /// pieces do to a contact at each of its `SUB`² points: `prop(piece,
    /// distance)`, joined over the pieces by `join` (the cloth's largest lift,
    /// the palm's largest shield). Within `fine` µm of a piece every point is
    /// measured; past it the cell's middle stands for its points (the prop
    /// changes little across a cell there).
    fn gaps_of(&self, fences: &[Fence], reach: &dyn Fn(usize) -> f64, prop: &dyn Fn(usize, f64) -> Option<Prop>, join: &dyn Fn(&mut Prop, Prop), fine: f64) -> Gaps {
        let p = self.pitch;
        let mut out = Gaps { idx: vec![u32::MAX; self.w * self.h], slots: Vec::new() };
        for (k, f) in fences.iter().enumerate() {
            let r = reach(k) + f.half;
            let (x0, x1) = (((f.x - r) / p).floor().max(0.0) as usize, (((f.x + r) / p).ceil() as usize).min(self.w));
            let (y0, y1) = (((f.y - r) / p).floor().max(0.0) as usize, (((f.y + r) / p).ceil() as usize).min(self.h));
            for y in y0..y1 {
                for x in x0..x1 {
                    let i = y * self.w + x;
                    let mid = f.signed((x as f64 + 0.5) * p, (y as f64 + 0.5) * p);
                    let mut put = |j: Option<usize>, g: Option<Prop>| {
                        let Some(g) = g else { return };
                        if out.idx[i] == u32::MAX {
                            out.idx[i] = out.slots.len() as u32;
                            out.slots.push([NO_PROP; SUB * SUB]);
                        }
                        let e = &mut out.slots[out.idx[i] as usize];
                        match j {
                            Some(j) => join(&mut e[j], g),
                            None => e.iter_mut().for_each(|v| join(v, g)),
                        }
                    };
                    if mid.abs() > fine + p {
                        put(None, prop(k, mid));
                        continue;
                    }
                    for j in 0..SUB * SUB {
                        let (sx, sy) = ((x as f64 + ((j % SUB) as f64 + 0.5) / SUB as f64) * p, (y as f64 + ((j / SUB) as f64 + 0.5) / SUB as f64) * p);
                        put(Some(j), prop(k, f.signed(sx, sy)));
                    }
                }
            }
        }
        out
    }

    /// What burrs do to the palm over each cell near one (`contact`), pressed
    /// at `p` Pa, at each of its `SUB`² points: the share of its touches a
    /// ridge riding a burr holds off there and how far (`Ridges::shield`;
    /// where a burr stands taller than the furrows, the whole skin's gap,
    /// `palm_gap_um`), and whether it lies on a burr's top, which the ridge
    /// riding it touches. Overlapping burrs: the larger shield, then the
    /// larger gap.
    fn palm_gaps(&self, p: f64) -> Gaps {
        let fences = self.fences();
        let ridges = Ridges::at(p);
        let tall: Vec<Option<(f64, f64)>> = fences.iter().map(|f| {
            let over = f.h - ridges.clearance();
            (over > 0.0).then(|| (over, palm_span_um(over, f.c, p)))
        }).collect();
        let reach = |k: usize| ridges.reach(fences[k].h) + ridges.a + tall[k].map_or(0.0, |t| t.1);
        let prop = |k: usize, d: f64| {
            let f = &fences[k];
            // over the fin the ridge riding its crest, standing off its faces below its contact
            let top = if -f.ei <= d && d <= f.eo { palm_on_fin_um(f.h, f.ei, f.eo, d, p) as f32 } else { f32::INFINITY };
            let d = d.abs();
            if let Some((over, span)) = tall[k]
                && d < span
            {
                return Some(Prop { v: 1.0, aux: palm_gap_um(over, f.c, p, span, d) as f32, top, fin: 0.0 });
            }
            let s = ridges.shield(f.h, d);
            (s > 0.0 || top.is_finite()).then_some(Prop { v: s as f32, aux: f.h as f32, top, fin: 0.0 })
        };
        let join = |e: &mut Prop, g: Prop| {
            if g.v > e.v || (g.v == e.v && g.aux > e.aux) {
                (e.v, e.aux) = (g.v, g.aux);
            }
            e.top = e.top.min(g.top);
        };
        self.gaps_of(&fences, &reach, &prop, &join, f64::INFINITY)
    }

    /// The tarlatan's lift over each cell near a burr under the contact laws
    /// (`cloth_lift`): the cloth's lift off its place on the flat at each of
    /// its `SUB`² points, the most any piece of burr lifts it, and over a
    /// burr's top the face's lift off that raised base (the least over the
    /// pieces), pressed at `p` Pa with bending rigidity `b` (`Face` reads
    /// them). It is a static contact, the same whichever way the pad moves.
    /// Pieces of like height and top share a solution (to 0.25 µm and 0.5
    /// µm). Points are measured within 0.3 mm of the burr, where it bends
    /// most; past that a cell's middle stands for them.
    fn cloth_gaps(&self, p: f64, b: f64) -> Gaps {
        let fences = self.fences();
        let mut lifts: std::collections::HashMap<(i64, i64), Lift> = std::collections::HashMap::new();
        let keys: Vec<(i64, i64)> = fences.iter().map(|f| ((f.h * 4.0).round() as i64, (f.c * 2.0).round() as i64)).collect();
        let mut todo: Vec<(i64, i64)> = keys.clone();
        todo.sort_unstable();
        todo.dedup();
        use rayon::prelude::*;
        lifts.extend(todo.par_iter().map(|&(h, c)| ((h, c), cloth_lift(h as f64 / 4.0, c as f64 / 2.0, p, b))).collect::<Vec<_>>());
        let prop = |k: usize, s: f64| {
            let f = &fences[k];
            let h = keys[k].0 as f64 / 4.0;
            let d = s.abs();
            let u = lifts[&keys[k]].at(d);
            let on = -f.ei <= s && s <= f.eo && h > 0.0;
            let top = if on { (u - h) as f32 } else { f32::INFINITY };
            let shade = yarn_shade_um(h, d);
            (u > 0.0 || top.is_finite() || shade > 0.0).then_some(Prop { v: u as f32, aux: shade as f32, top, fin: f.fin(s) as f32 })
        };
        let join = |e: &mut Prop, g: Prop| {
            e.v = e.v.max(g.v);
            e.aux = e.aux.max(g.aux);
            e.top = e.top.min(g.top);
            e.fin = e.fin.max(g.fin);
        };
        self.gaps_of(&fences, &|k| lifts[&keys[k]].reach(), &prop, &join, 300.0)
    }

    /// The film (µm over cell `c`) out of any wipe's reach: the polish's
    /// hold over its flat, under the contact laws (`POLISH_HOLD_UM`).
    fn polish_hold(&self, c: &Cell) -> f64 {
        if self.contact { POLISH_HOLD_UM * (1.0 - c.open_share().min(1.0)) as f64 } else { 0.0 }
    }

    /// `reach_sweep` under the contact laws, point by point over each cell
    /// near a burr (`Gaps`, read by `ct.touch` for the film on the flat and
    /// for the ink in the openings, `Touch`):
    /// - for a share r of a full touch it meets all it reaches, at the rate of
    ///   a touch r times as long (`law` over the share of a pass times r).
    ///   For the cloth r is the share of its fibres that dip into the film
    ///   beside the burr, its face's fibre density there (`Face::fibres`):
    ///   the film is a liquid as deep as a fibre is thick, which a fibre takes
    ///   from by dipping into it, not by pressing on the metal through it.
    ///   For the palm, the share of its touches that come down on the plate;
    /// - for `off` more of a touch it stands g off the plate (a true gap; a
    ///   ridge riding a burr), and meets only the ink standing above g on the
    ///   flat, and g less far into the openings;
    /// - and where the cloth's face stands clear of the plate (a true gap) its
    ///   loose fibre ends hang in the gap and draw on the film below it: they
    ///   lift the film under their tips, up to what capillarity lifts in a
    ///   touch (`ct.wick`: retroussage's law), over the share of it the tips
    ///   touch (`FIBRE_END_SHARE`), by the share of a pass. While the face is
    ///   on the plate they are fibres of its face, counted in r. (Its fibre
    ///   ends hang into the lines wherever it goes, gap or not; what it takes
    ///   from them is its reach.)
    #[allow(clippy::type_complexity)]
    fn reach_sweep_gap(&self, cells: &[(usize, usize, f32)], ct: &Contact, rims: &[Option<Vec<f32>>], law: &dyn Fn(usize, usize, &InkCell) -> (f64, f64), gaps: &Gaps) -> Vec<(usize, usize, f64, [f64; BINS])> {
        let mut out = Vec::new();
        for &(x, y, m) in cells {
            let ic = self.ink_cell(x, y);
            if ic.total() <= 0.0 {
                continue;
            }
            // the share of what it reaches a touch takes over a share `m` of a pass: A·(1 − e^(−κm))
            let (kappa, amp) = law(x, y, &ic);
            if amp <= 0.0 {
                continue;
            }
            let fr = |m: f64| amp * (1.0 - (-kappa * m).exp());
            let i = y * self.w + x;
            let c = self.cell(x, y);
            let hold = self.polish_hold(&c);
            let rim = |k: usize| rims[k].as_ref().and_then(|r| (c.open[k] >= 0.5).then_some(r[i]));
            let avail = (ic.film as f64 - hold).max(0.0);
            let reach: [f64; BINS] = std::array::from_fn(|k| Self::within_reach(&c, k, ic.line[k], rim(k), &*ct.reach));
            let m = m as f64;
            let (flat, bins) = match gaps.get(i) {
                None => {
                    let f = fr(m);
                    (avail * f, reach.map(|b| b * f))
                }
                Some(g) => {
                    let n = (SUB * SUB) as f64;
                    // the fibre ends' draw over a share `m` of a pass where the face stands clear:
                    // their tips touch `FIBRE_END_SHARE` of the film and lift up to the capillary
                    // depth of a touch from under them
                    let depth = ct.wick.as_ref().map_or(0.0, |w| w(&ic));
                    let (mut flat, mut bins) = (0.0f64, [0.0f64; BINS]);
                    let touches: [(Touch, Touch); SUB * SUB] = std::array::from_fn(|j| (ct.touch)(&g[j]));
                    let films = Self::sheltered(ic.film as f64, hold, &touches.map(|(tf, _)| tf));
                    for (j, &(tf, to)) in touches.iter().enumerate() {
                        // the film: above the gap it is met for r + off of a touch, below it for r
                        let f = films[j];
                        let avail = (f - hold).max(0.0);
                        let above = if tf.g > 0.0 { (f - tf.g.max(hold)).max(0.0).min(avail) } else { avail };
                        flat += above * fr(m * (tf.r + tf.off)) + (avail - above) * fr(m * tf.r);
                        flat += FIBRE_END_SHARE * m * tf.hair * (avail - above).min(depth);
                        // the openings: the ink within g less reach for r + off, the rest of its reach for r
                        let (whole, part) = (fr(m * (to.r + to.off)), fr(m * to.r));
                        for (k, b) in bins.iter_mut().enumerate() {
                            if reach[k] <= 0.0 {
                                continue;
                            }
                            let short = if to.g > 0.0 { Self::within_reach(&c, k, ic.line[k], rim(k), &|w, x| (ct.reach)(w, x) - to.g) } else { reach[k] };
                            *b += short * whole + (reach[k] - short) * part;
                        }
                    }
                    (flat / n, bins.map(|b| b / n))
                }
            };
            if flat + bins.iter().sum::<f64>() > 0.0 {
                out.push((x, y, flat, bins));
            }
        }
        out
    }

    /// How a cell's film (`film` µm over the cell, the polish's `hold` under
    /// it) lies over its `SUB`² points near a burr, given what a contact does
    /// at each (`touches`): the ink a wipe can't reach stays where it is, so a
    /// cell's film lies first where it is sheltered, under a gap no touch
    /// comes down through (a yarn's shade, the fin's faces under the yarn or
    /// the ridge riding its crest: `Touch::g` with `r` none), up to a common
    /// level; and what is more than that shelter holds lies evenly over the
    /// whole cell, above each gap [D: the film is one number a cell, its
    /// points' share of it unrecorded; ink is taken off and laid on, not
    /// moved about within a cell]. Spread evenly instead, each stroke met the
    /// whole film at the cell's least sheltered point (a fin's crest), and
    /// over a wipe's strokes the cell came down to that point's gap: a 42 µm
    /// cell over a 2 N fin, mostly the fin's faces, kept 0.2 µm. With no
    /// sheltered point the film lies evenly, as before.
    fn sheltered(film: f64, hold: f64, touches: &[Touch; SUB * SUB]) -> [f64; SUB * SUB] {
        let s: [f64; SUB * SUB] = touches.map(|t| if t.g > 0.0 && t.r <= 0.0 { t.g.max(hold) } else { hold });
        let n = (SUB * SUB) as f64;
        let mean = s.iter().sum::<f64>() / n;
        if film >= mean {
            return s.map(|v| v + film - mean);
        }
        // the level L with the mean of min(s, L) the film
        let (mut lo, mut hi) = (0.0f64, s.iter().cloned().fold(0.0, f64::max));
        for _ in 0..50 {
            let l = 0.5 * (lo + hi);
            if s.iter().map(|&v| v.min(l)).sum::<f64>() / n < film { lo = l } else { hi = l }
        }
        let l = 0.5 * (lo + hi);
        s.map(|v| v.min(l))
    }

    /// Each cell's share of a full touch under the contact laws, on its film
    /// and in its openings: the means over its points of the share it meets
    /// each for there (`Touch::r`; none where it stands off, as a yarn held
    /// off the line by the fin), (1, 1) for a cell no burr reaches.
    fn touch_of(&self, gaps: &Gaps, touch: &dyn Fn(&Prop) -> (Touch, Touch), x: usize, y: usize) -> (f64, f64) {
        gaps.get(y * self.w + x).map_or((1.0, 1.0), |g| {
            let n = (SUB * SUB) as f64;
            g.iter().map(|pr| touch(pr)).fold((0.0, 0.0), |a, (f, o)| (a.0 + f.r / n, a.1 + o.r / n))
        })
    }

    /// The ink in bin `k` of cell `c` (`line` µm over the cell) above the
    /// level a contact reaches (`reach` at x from the rim): what it can take.
    /// A wide opening's cell knows its distance from the rim (`rim`); a
    /// narrow one is averaged across its width.
    fn within_reach(c: &Cell, k: usize, line: f32, rim: Option<f32>, reach: &dyn Fn(f64, f64) -> f64) -> f64 {
        if c.open[k] <= 0.0 || line <= 0.0 {
            return 0.0;
        }
        let s = c.open[k] as f64;
        let (w, d) = ((c.wsum[k] / c.open[k]) as f64, (c.vol[k] / c.open[k]) as f64);
        let f = (line as f64 / s).min(d);
        let above = |x: f64| (f - (d - reach(w, x)).max(0.0)).max(0.0);
        let h = match rim {
            Some(x) => above((x as f64).min(0.5 * w)),
            None => {
                let n = 8;
                (0..n).map(|i| above((i as f64 + 0.5) * w / n as f64)).sum::<f64>() / n as f64
            }
        };
        s * h
    }

    /// One sweep of a contact over the cells (`cells`) moving `v`: each
    /// cell's share `frac` (its contact, the lift's rate) of what the contact
    /// reaches. Returns the takes, cell by cell: (x, y, from the flat, from
    /// each bin).
    #[allow(clippy::type_complexity)]
    fn reach_sweep(&self, cells: &[(usize, usize, f32)], v: (f64, f64), ct: &Contact, rims: &[Option<Vec<f32>>], frac: &dyn Fn(usize, usize, f32, &InkCell) -> f64) -> Vec<(usize, usize, f64, [f64; BINS])> {
        // (the contact laws don't come here: their cloth and palm are held off by gaps, `reach_sweep_gap`)
        let shield = self.shields(v, &*ct.lee);
        let mut out = Vec::new();
        for &(x, y, m) in cells {
            let ic = self.ink_cell(x, y);
            if ic.total() <= 0.0 {
                continue;
            }
            let fr = frac(x, y, m, &ic);
            if fr <= 0.0 {
                continue;
            }
            let i = y * self.w + x;
            let (sf, so) = shield.as_ref().map_or((0.0, 0.0), |(f, o)| (f[i] as f64, o[i] as f64));
            let c = self.cell(x, y);
            let flat = (ic.film as f64 - self.polish_hold(&c)).max(0.0) * fr * (1.0 - sf);
            let mut bins = [0.0f64; BINS];
            for k in 0..BINS {
                let rim = rims[k].as_ref().and_then(|r| (c.open[k] >= 0.5).then_some(r[i]));
                bins[k] = Self::within_reach(&c, k, ic.line[k], rim, &*ct.reach) * fr * (1.0 - so);
            }
            out.push((x, y, flat, bins));
        }
        out
    }

    /// Take `scale` of each cell's asks off the plate: the ink taken (µm
    /// over cells) and its shares, summed; and each cell's (µm and shares,
    /// aligned with `asks`).
    fn take(&mut self, asks: &[(usize, usize, f64, [f64; BINS])], scale: f64) -> (f64, [f64; INKS], Vec<(f64, [f32; INKS])>) {
        let (mut got, mut mix) = (0.0f64, [0.0f64; INKS]);
        let mut each = Vec::with_capacity(asks.len());
        for &(x, y, flat, bins) in asks {
            let ic = self.ink_cell_mut(x, y);
            let sh = ic.shares();
            let mut t = (flat * scale).min(ic.film as f64);
            ic.film = (ic.film as f64 - t).max(0.0) as f32;
            for k in 0..BINS {
                let b = (bins[k] * scale).min(ic.line[k] as f64);
                ic.line[k] = (ic.line[k] as f64 - b).max(0.0) as f32;
                t += b;
            }
            if ic.total() <= 0.0 {
                *ic = InkCell::default();
            }
            got += t;
            for i in 0..INKS {
                mix[i] += t * sh[i] as f64;
            }
            each.push((t, sh));
        }
        (got, mix, each)
    }

    /// Lay `um` (µm over cells) of ink back on `cells`, by their weights
    /// (all alike if none weighs anything), the `j`th of shares `mix(j)`: on
    /// each cell's flat by its share of the cell, into its openings by theirs,
    /// or by `split(j)`, the share that goes into the openings, if it gives one.
    fn lay(&mut self, cells: &[(usize, usize, f64)], um: f64, mix: impl Fn(usize) -> [f32; INKS], split: impl Fn(usize) -> Option<f64>, cure: f32) {
        let total: f64 = cells.iter().map(|c| c.2).sum();
        if cells.is_empty() || um <= 0.0 {
            return;
        }
        let even = total <= 0.0;
        for (j, &(x, y, wt)) in cells.iter().enumerate() {
            let a = if even { um / cells.len() as f64 } else { um * wt / total };
            if a <= 0.0 {
                continue;
            }
            let c = self.cell(x, y);
            let open = c.open_share().min(1.0) as f64;
            let open = if open > 0.0 { split(j).unwrap_or(open).clamp(0.0, 1.0) } else { 0.0 };
            let ic = self.ink_cell_mut(x, y);
            let had = ic.total();
            ic.film += (a * (1.0 - open)) as f32;
            if open > 0.0 {
                for k in 0..BINS {
                    ic.line[k] += (a * c.open[k] as f64 / c.open_share() as f64 * open) as f32;
                }
            }
            ic.blend(had, a as f32, &mix(j), cure);
        }
    }

    /// Wipe the plate with tarlatan `rag` (its face, load and creases a
    /// rag's; its stiffness `rag.cloth`) moved along `pts` (mm; each point's
    /// force the hand's pressure, 0..1) `passes` times, at `now` (IP §5.3).
    ///
    /// The pad, its width across, slides along the points (`Plate::slide`,
    /// in stretches a quarter of its width), and rubs each cell for its share
    /// of a pass there (one under its middle on a straight stroke). The cloth
    /// takes by the rag's lift law (its face's thirst, its creases' contact,
    /// the ink's fluidity), compounded over the share of a pass, what it
    /// reaches, moving the way the pad goes there:
    /// - the film on the flat, but in a burr's lee (`shields`, `lee_um`;
    ///   under the contact laws, `contact`, by the fibres of its face in the
    ///   film where the burr lifts it, and only the ink above a yarn resting
    ///   on the fin in its shade, at the stretch's pressure, `cloth_gaps`,
    ///   whichever way it moves, and never below the polish's hold,
    ///   `POLISH_HOLD_UM`);
    /// - the ink in each opening above the level it reaches: the yarn's dip
    ///   (`yarn_dip_um`) and the cloth's sag across the opening (`sag_um`,
    ///   at the cell's distance from the rim of a wide one).
    ///
    /// The face holds no more than it has room for, and smears back the
    /// share `rag::SMEAR` of the ink at its surface every quarter of its
    /// width it slides, as the rag does, of which `rag::SOAK` soaks in: what
    /// it lays back is the plate tone. Nothing is wiped where the pad
    /// doesn't go.
    pub fn wipe(&mut self, rag: &mut Rag, pts: &[Point], passes: u32, now: f64) -> WipeReport {
        self.age_ink(now);
        let temp = self.temp_c(now);
        let before = self.ink_um3();
        let cloth = rag.cloth.get_or_insert_with(|| Box::new(Tarlatan::new(0.7))).as_ref().clone();
        // (under the contact laws the cloth is softened for wiping, `softened_bend_nm`)
        let b = if self.contact { softened_bend_nm(cloth.stiff as f64) } else { cloth.bend_nm };
        let pad_mm = rag.width as f64;
        let stretches = if self.contact { self.slide_pressed(pts, &|_| pad_mm, 0.25 * pad_mm) } else { self.slide(pts, &|_| pad_mm, 0.25 * pad_mm) };
        let inks: Vec<Ink> = self.ink.as_ref().map(|k| k.inks.clone()).unwrap_or_default();
        let rims = self.rims();
        let pad_um = pad_mm * 1000.0;
        let cap = pad_um * pad_um * crate::rag::CAP_UM as f64;
        let (mut taken, mut laid, mut swept) = (0.0f64, 0.0f64, 0.0f64);
        let unit = self.pitch * self.pitch;
        // what a track of the face (`tracks`) keeps of its pool over a µm of
        // its slide: the share it keeps a quarter of the pad's width,
        // `rag::SMEAR` laid back and `rag::SOAK` soaked in
        let fade_um = -((1.0 - crate::rag::SMEAR as f64) * (1.0 - crate::rag::SOAK as f64)).ln() / (pad_um / 4.0);
        let mut at = vec![u32::MAX; self.w * self.h];
        // each track's pool where the last stretch left it (its place along the way), while the
        // pad keeps moving the same way: the face's points carry it on
        let mut carry: (Option<(f64, f64)>, std::collections::HashMap<i64, (f64, [f64; INKS])>) = (None, std::collections::HashMap::new());
        // (the contact laws: the cloth's drape over a burr at each pressure the pad is pressed
        // with, the same whichever way it moves)
        let mut cloth_gaps: std::collections::HashMap<u64, Gaps> = std::collections::HashMap::new();
        for pass in 0..passes {
            for (j, st) in stretches.iter().enumerate() {
                let v = st.v;
                let pressure = st.pose.force.clamp(0.0, 1.0);
                let p = PAD_PA * pressure.max(0.01) as f64;
                let face = Face::at(p);
                let inks = inks.clone();
                let ct = Contact {
                    reach: Box::new(move |w, x| yarn_dip_um(YARN_UM, w.min(2.0 * YARN_UM)) + if w > 2.0 * YARN_UM { sag_um(w, x, p, b) } else { 0.0 }),
                    lee: Box::new(move |h| lee_um(h, p, b)),
                    // beside a burr its face meets the film by the fibres that dip into it
                    // (`Face::fibres`), harder over the burr's top; past its squeeze a true
                    // gap, where its loose fibre ends hang
                    touch: Box::new(move |pr: &Prop| {
                        let (u, shade) = (pr.v as f64, pr.aux as f64);
                        let gap = if u >= face.delta || shade > 0.0 {
                            // a true gap: past the face's squeeze, or in a yarn's shade by the fin
                            // (`yarn_shade_um`), where it meets only the ink above the gap
                            Some(Touch { r: 0.0, off: face.fibres(u.min(face.delta - 1e-9)), g: (u - face.delta).max(shade), hair: 1.0 })
                        } else {
                            None
                        };
                        let open = gap.unwrap_or(Touch { r: face.fibres(u), ..FULL_TOUCH });
                        // (in the openings: a yarn resting on a crest stands `shade` over the plate
                        // there by its own circle, its dip and all, so it reaches none of an
                        // opening's ink; its chord dips into an opening only where it lies on the
                        // opening's rims, `yarn_dip_um`, as it does in a true gap's)
                        let lines = match gap {
                            Some(t) if shade > 0.0 => Touch { g: t.g.max(shade + YARN_UM), ..t },
                            _ => open,
                        };
                        // over a fin: the yarn on its crest, squeezing the face there, stands off its
                        // faces by its circle less their height; it meets their ink only above that
                        let flat = if pr.top.is_finite() {
                            let g = shade - pr.fin as f64;
                            if g > 0.0 { Touch { r: 0.0, off: face.fibres(pr.top as f64), g, hair: 1.0 } } else { Touch { r: face.fibres(pr.top as f64), ..FULL_TOUCH } }
                        } else {
                            open
                        };
                        (flat, lines)
                    }),
                    // the fibre ends in a gap: retroussage's capillary rise in a touch, over the
                    // share of the plate their tips touch
                    wick: Some(Box::new(move |ic: &InkCell| {
                        let sh = ic.shares();
                        let eta: f64 = inks.iter().enumerate().map(|(i, k)| sh[i] as f64 * k.eta(temp)).sum::<f64>().max(1e-6);
                        washburn_um(eta, RETRO_TOUCH_S) * crate::drying::fluid(ic.cure) as f64
                    })),
                };
                {
                    let cells = &st.cells;
                    // the area the pad passes over (a pass's share a cell) and the quarter-pad steps it slides
                    swept += cells.iter().map(|c| c.2 as f64).sum::<f64>() * unit;
                    let quarters = (4.0 * st.len_mm / pad_mm).max(1e-3);
                    let rate = crate::rag::LIFT as f64 * (0.7 + 0.6 * pressure as f64) * rag.thirst() as f64;
                    let cs = rag.cloth_seed(((pass as u64) << 32) | j as u64);
                    let pitch = self.pitch;
                    let frac = |x: usize, y: usize, m: f32, ic: &InkCell| {
                        // the pad's creases, in its own frame (mm), moving `v`
                        let (px, py) = ((x as f64 + 0.5) * pitch * 1e-3, (y as f64 + 0.5) * pitch * 1e-3);
                        let (u, w) = (px * v.1 - py * v.0, px * v.0 + py * v.1);
                        let c = crate::rag::cloth3(u as f32, w as f32, cs, pressure) as f64;
                        let fl = crate::drying::fluid(ic.cure) as f64;
                        (1.0 - (-rate * fl * m as f64).exp()) * c
                    };
                    let law = |x: usize, y: usize, ic: &InkCell| {
                        let (px, py) = ((x as f64 + 0.5) * pitch * 1e-3, (y as f64 + 0.5) * pitch * 1e-3);
                        let (u, w) = (px * v.1 - py * v.0, px * v.0 + py * v.1);
                        (rate * crate::drying::fluid(ic.cure) as f64, crate::rag::cloth3(u as f32, w as f32, cs, pressure) as f64)
                    };
                    let gaps = if self.contact { Some(&*cloth_gaps.entry(p.to_bits()).or_insert_with(|| self.cloth_gaps(p, b))) } else { None };
                    let asks = match gaps {
                        Some(g) => self.reach_sweep_gap(cells, &ct, &rims, &law, g),
                        None => self.reach_sweep(cells, v, &ct, &rims, &frac),
                    };
                    let want: f64 = asks.iter().map(|a| a.2 + a.3.iter().sum::<f64>()).sum::<f64>() * unit;
                    let room = (1.0 - rag.load as f64).max(0.0) * cap;
                    let scale = if want > room && want > 0.0 { room / want } else { 1.0 };
                    let (got, mix, each) = self.take(&asks, scale);
                    let got_um3 = got * unit;
                    // the face's surface: laid back as it slides, soaking in
                    let cl = rag.cloth.as_mut().unwrap();
                    // (the ink the face comes to this stretch with)
                    let (pool_in, pool_in_mix) = (cl.pool_um3, cl.pool_mix);
                    let n = quarters.ceil().max(1.0) as usize;
                    let (sm, so) = (1.0 - (1.0 - crate::rag::SMEAR as f64).powf(quarters / n as f64), 1.0 - (1.0 - crate::rag::SOAK as f64).powf(quarters / n as f64));
                    let got_mix: [f32; INKS] = if got > 0.0 { std::array::from_fn(|i| (mix[i] / got) as f32) } else { [0.0; INKS] };
                    let mut lay_mix = [0.0f64; INKS];
                    let mut lay_um3 = 0.0f64;
                    for _ in 0..n {
                        let a = sm * cl.pool_um3;
                        for i in 0..INKS {
                            lay_mix[i] += a * cl.pool_mix[i] as f64;
                        }
                        lay_um3 += a;
                        let left = (cl.pool_um3 - a) * (1.0 - so);
                        let add = got_um3 / n as f64;
                        let t = left + add;
                        if t > 0.0 {
                            for i in 0..INKS {
                                cl.pool_mix[i] = ((cl.pool_mix[i] as f64 * left + got_mix[i] as f64 * add) / t) as f32;
                            }
                        }
                        cl.pool_um3 = t;
                    }
                    let lay_shares: [f32; INKS] = if lay_um3 > 0.0 { std::array::from_fn(|i| (lay_mix[i] / lay_um3) as f32) } else { [0.0; INKS] };
                    // laid where the cloth touched, out of the lee (under the contact
                    // laws: by its share of full contact there, and by what the face
                    // carries there, below)
                    // (under the laws, by its touch on the flat and in the openings, each over
                    // its share of the cell, and split between them so: `splits`)
                    let mut splits: Vec<Option<f64>> = vec![None; cells.len()];
                    let mut wts: Vec<(usize, usize, f64)> = match gaps {
                        Some(g) => cells.iter().enumerate().map(|(j, &(x, y, m))| {
                            let (rf, ro) = self.touch_of(g, &*ct.touch, x, y);
                            let open = self.cell(x, y).open_share().min(1.0) as f64;
                            let (wf, wo) = ((1.0 - open) * rf, open * ro);
                            if wf + wo > 0.0 {
                                splits[j] = Some(wo / (wf + wo));
                            }
                            (x, y, m as f64 * (wf + wo))
                        }).collect(),
                        None => {
                            let shield = self.shields(v, &*ct.lee);
                            cells.iter().map(|&(x, y, m)| {
                                let sf = shield.as_ref().map_or(0.0, |(f, _)| f[y * self.w + x] as f64);
                                (x, y, m as f64 * (1.0 - sf))
                            }).collect()
                        }
                    };
                    // what it lays at a cell is what its face carries there: a
                    // point of the face slides along its track, so the pool it
                    // lays from is the ink it took along the track, fading by
                    // the smear law behind it, from where the last stretch left
                    // it if the pad goes on the same way, else from its share of
                    // the pool the face came to the stretch with (the face
                    // `pad_um` wide, a track a cell's). (The face's pool, kept
                    // whole above, counts the volumes it lays.)
                    if carry.0 != Some(v) {
                        carry = (Some(v), std::collections::HashMap::new());
                    }
                    for (k, a) in asks.iter().enumerate() {
                        at[a.1 * self.w + a.0] = k as u32;
                    }
                    let track_share = self.pitch / pad_um;
                    let mut carried = vec![lay_shares; cells.len()];
                    // (the contact laws: the ink the face's point comes to each cell with)
                    let mut arrive = vec![0.0f64; cells.len()];
                    let mut pool = [0.0f64; INKS];
                    let mut last: Option<(i64, f64)> = None;
                    for &(row, s, ci) in &tracks(cells, v) {
                        match last {
                            Some((r, s0)) if r == row => {
                                let keep = (-fade_um * (s - s0) * self.pitch).exp();
                                pool.iter_mut().for_each(|q| *q *= keep);
                            }
                            _ => {
                                if let Some((r, s0)) = last {
                                    carry.1.insert(r, (s0, pool));
                                }
                                pool = match carry.1.get(&row) {
                                    Some(&(s0, q)) if s >= s0 => {
                                        let keep = (-fade_um * (s - s0) * self.pitch).exp();
                                        q.map(|v| v * keep)
                                    }
                                    _ => std::array::from_fn(|i| pool_in * track_share * pool_in_mix[i] as f64),
                                };
                            }
                        }
                        last = Some((row, s));
                        arrive[ci] = pool.iter().sum();
                        let (x, y, _) = cells[ci];
                        if let Some(&(t, sh)) = each.get(at[y * self.w + x] as usize) {
                            for i in 0..INKS {
                                pool[i] += t * unit * sh[i] as f64;
                            }
                        }
                        let sum: f64 = pool.iter().sum();
                        if sum > 0.0 {
                            carried[ci] = std::array::from_fn(|i| (pool[i] / sum) as f32);
                        }
                    }
                    if let Some((r, s0)) = last {
                        carry.1.insert(r, (s0, pool));
                    }
                    for a in &asks {
                        at[a.1 * self.w + a.0] = u32::MAX;
                    }
                    // under the contact laws the face lays where it carries ink: each point of
                    // it smears back its share of what it has picked up along its track, so
                    // ink taken off a line is laid on past it, the way the pad goes
                    if gaps.is_some() {
                        for (w, a) in wts.iter_mut().zip(&arrive) {
                            w.2 *= a;
                        }
                    }
                    self.lay(&wts, lay_um3 / unit, |k| carried[k], |k| splits[k], 0.0);
                    let cl = rag.cloth.as_mut().unwrap();
                    cl.held_um3 += got_um3 - lay_um3;
                    taken += got_um3;
                    laid += lay_um3;
                    let net_mm3 = (got_um3 - lay_um3) * 1e-9;
                    rag.load = (rag.load as f64 + net_mm3 / (cap * 1e-9)).clamp(0.0, 1.0) as f32;
                    rag.soaked = (rag.soaked as f64 + net_mm3 / (cap * 1e-9 * crate::rag::FACES as f64)).clamp(0.0, 1.0) as f32;
                }
            }
        }
        let after = self.ink_um3();
        let area_mm2 = self.union(&stretches).len() as f64 * unit * 1e-6;
        self.wipe_report(before, after, taken, laid, area_mm2, swept * 1e-6 / pace::TARLATAN_MM2_S, temp)
    }

    #[allow(clippy::too_many_arguments)]
    fn wipe_report(&self, before: (f64, f64), after: (f64, f64), taken: f64, laid: f64, area_mm2: f64, secs: f64, temp: f64) -> WipeReport {
        let a_um2 = (area_mm2 * 1e6).max(1e-12);
        WipeReport {
            before_um3: before.0 + before.1,
            after_um3: after.0 + after.1,
            taken_um3: taken,
            laid_um3: laid,
            kept_um3: taken - laid,
            film_um: after.1 / a_um2,
            lines_um: after.0 / a_um2,
            area_mm2,
            secs,
            temp_c: temp,
        }
    }

    /// The hand wipe (IP §5.4): the palm, soft, drawn along `pts` (mm; each
    /// point's force the hand's pressure, 0..1) `passes` times. The palm,
    /// `PALM_MM` across, slides along the points (`Plate::slide`) and rubs
    /// each cell for its share of a pass there; each pass, skin bulges into
    /// each opening by `palm_dip_um` (the top few µm of a fine line, the
    /// whole of an open bite), is held off past a burr by `palm_lee_um`
    /// (under the contact laws, `contact`: it bears by its ridges, and a burr
    /// lower than their furrows props only a ridge riding it, `palm_gaps`,
    /// `Ridges`; there it reaches only the ink above the burr), and parts from
    /// the film it touches taking its split share (`PALM_SPLIT`, or
    /// `WHITING_SPLIT` with whiting), compounded over the share of a pass.
    /// The printer wipes the palm clean between strokes: what it takes leaves
    /// the plate. Nothing is wiped where the palm doesn't go.
    pub fn hand_wipe(&mut self, pts: &[Point], passes: u32, whiting: bool, now: f64) -> WipeReport {
        self.age_ink(now);
        let temp = self.temp_c(now);
        let before = self.ink_um3();
        let stretches = self.slide(pts, &|_| PALM_MM, 0.25 * PALM_MM);
        let rims = self.rims();
        let q = if whiting { WHITING_SPLIT } else { PALM_SPLIT };
        let unit = self.pitch * self.pitch;
        let (mut taken, mut swept) = (0.0f64, 0.0f64);
        // (the contact laws: the gap a burr props the skin up by at each pressure the palm is
        // pressed with, the same whichever way it moves)
        let mut gaps: std::collections::HashMap<u64, Gaps> = std::collections::HashMap::new();
        for _ in 0..passes {
            for st in &stretches {
                let p = PALM_PA * st.pose.force.clamp(0.01, 1.0) as f64;
                let ct = Contact {
                    reach: Box::new(move |w, x| palm_dip_um(w, x, p)),
                    lee: Box::new(move |h| palm_lee_um(h, p)),
                    // (`palm_gaps`: the share of its touches a ridge riding a burr holds off, and
                    // the gap then; it touches whole wherever it comes down, and a burr's top
                    // under the ridge riding it)
                    touch: Box::new(|pr: &Prop| {
                        let open = Touch { r: 1.0 - pr.v as f64, off: pr.v as f64, g: pr.aux as f64, hair: 0.0 };
                        // (over a fin, the ridge on its crest: whole on its contact, off the faces below)
                        let top = if pr.top <= 0.0 { FULL_TOUCH } else { Touch { r: 0.0, off: 1.0, g: pr.top as f64, hair: 0.0 } };
                        (if pr.top.is_finite() { top } else { open }, open)
                    }),
                    wick: None,
                };
                swept += st.cells.iter().map(|c| c.2 as f64).sum::<f64>() * unit;
                let frac = |_: usize, _: usize, m: f32, ic: &InkCell| (1.0 - (1.0 - q).powf(m as f64)) * crate::drying::fluid(ic.cure) as f64;
                let asks = if self.contact {
                    let g = gaps.entry(p.to_bits()).or_insert_with(|| self.palm_gaps(p));
                    self.reach_sweep_gap(&st.cells, &ct, &rims, &|_, _, ic: &InkCell| (-(1.0 - q).ln(), crate::drying::fluid(ic.cure) as f64), g)
                } else {
                    self.reach_sweep(&st.cells, st.v, &ct, &rims, &frac)
                };
                taken += self.take(&asks, 1.0).0 * unit;
            }
        }
        let after = self.ink_um3();
        let area_mm2 = self.union(&stretches).len() as f64 * unit * 1e-6;
        self.wipe_report(before, after, taken, 0.0, area_mm2, swept * 1e-6 / pace::PALM_TOUCH_MM2_S, temp)
    }

    /// Retroussage (IP §5.4): a soft muslin `width_mm` across passed lightly
    /// along `pts` (mm) over the wiped plate `passes` times, rubbing each cell
    /// for its share of a pass (`Plate::slide`). Its fibre ends hang into every opening wider than a
    /// fibre (`FIBRE_UM`) and draw up as much as capillarity lifts in their
    /// touch (`washburn_um`: more on a warm plate, the ink thinner), by the
    /// share of a pass, and lay
    /// it on the plate within `RETRO_BAND_MM` around, but for the share that
    /// soaks into the cloth (`rag::SOAK`), which leaves the plate. Nothing is
    /// drawn up where the cloth doesn't go.
    pub fn retrousse(&mut self, pts: &[Point], width_mm: f64, passes: u32, now: f64) -> WipeReport {
        self.age_ink(now);
        let temp = self.temp_c(now);
        let before = self.ink_um3();
        let stretches = self.slide(pts, &|_| width_mm, 0.25 * width_mm);
        let unit = self.pitch * self.pitch;
        let band = ((RETRO_BAND_MM * 1000.0 / self.pitch).round() as i64).max(1);
        let inks: Vec<Ink> = self.ink.as_ref().map(|k| k.inks.clone()).unwrap_or_default();
        let (mut taken, mut laid, mut swept) = (0.0f64, 0.0f64, 0.0f64);
        for _ in 0..passes {
            for st in &stretches {
                {
                    let cells = &st.cells;
                    swept += cells.iter().map(|c| c.2 as f64).sum::<f64>() * unit;
                    let mut moves: Vec<(usize, usize, f64, [f32; INKS], f32)> = Vec::new();
                    for &(x, y, m) in cells {
                        let ic = self.ink_cell(x, y);
                        if ic.lines() <= 0.0 {
                            continue;
                        }
                        let c = self.cell(x, y);
                        let sh = ic.shares();
                        let eta: f64 = inks.iter().enumerate().map(|(i, k)| sh[i] as f64 * k.eta(temp)).sum::<f64>().max(1e-6);
                        let up = washburn_um(eta, RETRO_TOUCH_S) * crate::drying::fluid(ic.cure) as f64;
                        let mut t = 0.0f64;
                        let mut cell = ic;
                        for k in 0..BINS {
                            // a fibre hangs in and touches the ink, and draws up its capillary rise
                            if c.open[k] <= 0.0 || ic.line[k] <= 0.0 || (c.wsum[k] / c.open[k]) as f64 <= FIBRE_UM {
                                continue;
                            }
                            let a = (c.open[k] as f64 * up).min(ic.line[k] as f64) * m as f64;
                            cell.line[k] = (cell.line[k] as f64 - a).max(0.0) as f32;
                            t += a;
                        }
                        if t > 0.0 {
                            *self.ink_cell_mut(x, y) = cell;
                            moves.push((x, y, t, sh, ic.cure));
                        }
                    }
                    for (x, y, t, sh, cure) in moves {
                        taken += t * unit;
                        let lay = t * (1.0 - crate::rag::SOAK as f64);
                        laid += lay * unit;
                        let mut wts = Vec::new();
                        for dy in -band..=band {
                            for dx in -band..=band {
                                let (xx, yy) = (x as i64 + dx, y as i64 + dy);
                                if xx < 0 || yy < 0 || xx >= self.w as i64 || yy >= self.h as i64 || dx * dx + dy * dy > band * band {
                                    continue;
                                }
                                wts.push((xx as usize, yy as usize, 1.0));
                            }
                        }
                        self.lay(&wts, lay, |_| sh, |_| None, cure);
                    }
                }
            }
        }
        let after = self.ink_um3();
        let area_mm2 = self.union(&stretches).len() as f64 * unit * 1e-6;
        self.wipe_report(before, after, taken, laid, area_mm2, swept * 1e-6 / pace::RETRO_MM2_S, temp)
    }

    // ------------------------------------------------------------ looking

    /// An inked cell's colour: the metal under the ink of its openings and
    /// its film, each a Kubelka–Munk layer of its thickness over what it
    /// lies on (`flat`, the openings' `wall` return, the burr's `burr`).
    pub(super) fn inked_rgb(&self, x: usize, y: usize, c: &Cell, flat: Rgb, walls: &[Rgb], burr: Rgb) -> Option<Rgb> {
        let k = self.ink.as_deref()?;
        let ic = self.ink_cell(x, y);
        if ic.total() <= 0.0 {
            return None;
        }
        let sh = ic.shares();
        let (mut kk, mut ss) = ([0.0f32; 3], [0.0f32; 3]);
        for (i, ink) in k.inks.iter().enumerate() {
            let pg = ink.pigment();
            for q in 0..3 {
                kk[q] += sh[i] * pg.k[q];
                ss[q] += sh[i] * pg.s[q];
            }
        }
        let pg = crate::pigment::Pigment { k: kk, s: ss };
        let open = c.open_share().min(1.0);
        let bshare = c.burr_area.min(1.0 - open);
        let flat_share = (1.0 - open).max(1e-6);
        let film = ic.film / flat_share / COAT_UM;
        let mut px = pg.over(flat, film).map(|v| v * (1.0 - open - bshare));
        for q in 0..3 {
            px[q] += bshare * pg.over(burr, film)[q];
        }
        for kb in 0..BINS {
            if c.open[kb] <= 0.0 || c.vol[kb] <= 0.0 {
                continue;
            }
            let (w, d) = (c.wsum[kb] / c.open[kb], c.vol[kb] / c.open[kb]);
            let wall = ((4.0 * d / w.max(1e-6)) as f64).atan().to_degrees().round() as usize;
            let under = walls[wall.min(90)];
            let f = ic.line[kb] / c.open[kb] / COAT_UM;
            let col = pg.over(under, f + film);
            for q in 0..3 {
                px[q] += c.open[kb] * col[q];
            }
        }
        Some(px)
    }

    /// For a crop's plate drawn again finer (`rebuilt`): each finer cell's
    /// ink from its coarse cell's, as the share of each bin's openings it
    /// fills and the film's thickness over the flat.
    pub(super) fn ink_onto(&self, fine: &mut Plate, factor: usize, window: (usize, usize, usize, usize)) {
        let Some(k) = self.ink.as_deref() else { return };
        fine.ink = Some(Box::new(Inking { inks: k.inks.clone(), tiles: vec![None; fine.tiles.len()], warm_c: k.warm_c, warm_at: k.warm_at, aged_to: k.aged_to }));
        let (x0, y0, x1, y1) = window;
        for y in y0..y1.min(fine.h) {
            for x in x0..x1.min(fine.w) {
                let (cx, cy) = (x / factor, y / factor);
                let ic = self.ink_cell(cx, cy);
                if ic.total() <= 0.0 {
                    continue;
                }
                let c = self.cell(cx, cy);
                let fc = fine.cell(x, y);
                let mut out = InkCell { cure: ic.cure, mix: ic.mix, ..InkCell::default() };
                for b in 0..BINS {
                    if c.vol[b] > 0.0 {
                        out.line[b] = ic.line[b] / c.vol[b] * fc.vol[b];
                    }
                }
                let flat = (1.0 - c.open_share().min(1.0)).max(1e-6);
                out.film = ic.film / flat * (1.0 - fc.open_share().min(1.0));
                *fine.ink_cell_mut(x, y) = out;
            }
        }
    }

    /// Whether the ink at a cell has set past its gel point (it can't be
    /// wiped or printed: it has dried in).
    pub fn ink_set(&self, x: usize, y: usize) -> bool {
        self.ink_cell(x, y).cure >= GEL
    }
}

// ---------------------------------------------------------------- persistence

/// The ink's part of a plate's section (`PLAT` version 3): the inks, the
/// warmth, the clock it is aged to, then the tiles in use with their cells.
pub(super) fn write_ink(k: &Inking, w: &mut impl Write) -> io::Result<()> {
    put64(w, k.inks.len() as u64)?;
    for ink in &k.inks {
        put_str(w, &ink.name)?;
        put64(w, ink.tubes.len() as u64)?;
        for (n, parts) in &ink.tubes {
            put_str(w, n)?;
            put32(w, parts.to_bits())?;
        }
        for v in [ink.oil, ink.stiff, ink.pvc, ink.k[0], ink.k[1], ink.k[2], ink.s[0], ink.s[1], ink.s[2], ink.drying] {
            put32(w, v.to_bits())?;
        }
    }
    put32(w, k.warm_c.to_bits())?;
    put64(w, k.warm_at.to_bits())?;
    put64(w, k.aged_to.to_bits())?;
    let used: Vec<usize> = (0..k.tiles.len()).filter(|&i| k.tiles[i].as_ref().is_some_and(|t| t.iter().any(|c| c.total() > 0.0))).collect();
    put64(w, used.len() as u64)?;
    let mut buf = Vec::with_capacity(TILE * TILE * INK_BYTES);
    for i in used {
        put64(w, i as u64)?;
        buf.clear();
        for c in k.tiles[i].as_ref().unwrap().iter() {
            for v in c.line.iter().chain([&c.film, &c.cure]) {
                buf.extend_from_slice(&v.to_bits().to_le_bytes());
            }
            for m in c.mix {
                buf.extend_from_slice(&m.to_le_bytes());
            }
        }
        w.write_all(&buf)?;
    }
    Ok(())
}

const INK_BYTES: usize = 4 * (BINS + 2) + 2 * INKS;

pub(super) fn read_ink(p: &Plate, r: &mut impl Read) -> io::Result<Inking> {
    let bad = |m: &str| io::Error::new(io::ErrorKind::InvalidData, format!("plate: {m}"));
    let f = |r: &mut dyn Read| -> io::Result<f32> { Ok(f32::from_bits(get32(r)?)) };
    let n = get64(r)?;
    if n > INKS as u64 {
        return Err(bad("more inks than a plate holds"));
    }
    let mut inks = Vec::new();
    for _ in 0..n {
        let name = get_str(r)?;
        let nt = get64(r)?;
        if nt > 64 {
            return Err(bad("an ink of too many tubes"));
        }
        let mut tubes = Vec::new();
        for _ in 0..nt {
            let t = get_str(r)?;
            tubes.push((t, f(r)?));
        }
        let v: Vec<f32> = (0..10).map(|_| f(r)).collect::<io::Result<_>>()?;
        if v.iter().any(|x| !x.is_finite()) {
            return Err(bad("an ink is damaged"));
        }
        inks.push(Ink { name, tubes, oil: v[0], stiff: v[1], pvc: v[2], k: [v[3], v[4], v[5]], s: [v[6], v[7], v[8]], drying: v[9] });
    }
    let warm_c = f(r)?;
    let warm_at = f64::from_bits(get64(r)?);
    let aged_to = f64::from_bits(get64(r)?);
    if !warm_c.is_finite() || !warm_at.is_finite() || !aged_to.is_finite() {
        return Err(bad("its ink's clock is damaged"));
    }
    let mut k = Inking { inks, tiles: vec![None; p.tiles.len()], warm_c, warm_at, aged_to };
    let used = get64(r)?;
    if used > k.tiles.len() as u64 {
        return Err(bad("more ink tiles than it has"));
    }
    let mut buf = vec![0u8; TILE * TILE * INK_BYTES];
    let mut last = None;
    for _ in 0..used {
        let i = get64(r)? as usize;
        if i >= k.tiles.len() || last.is_some_and(|l| i <= l) {
            return Err(bad("an ink tile out of place"));
        }
        last = Some(i);
        r.read_exact(&mut buf)?;
        let mut cells = vec![InkCell::default(); TILE * TILE];
        let mut at = 0;
        for c in cells.iter_mut() {
            let mut word = || {
                let v = f32::from_bits(u32::from_le_bytes(buf[at..at + 4].try_into().unwrap()));
                at += 4;
                v
            };
            for b in 0..BINS {
                c.line[b] = word();
            }
            c.film = word();
            c.cure = word();
            for m in c.mix.iter_mut() {
                *m = u16::from_le_bytes(buf[at..at + 2].try_into().unwrap());
                at += 2;
            }
            if c.line.iter().chain([&c.film, &c.cure]).any(|v| !v.is_finite() || *v < 0.0) {
                return Err(bad("an ink cell is damaged"));
            }
        }
        k.tiles[i] = Some(cells.into_boxed_slice());
    }
    Ok(k)
}

fn put_str(w: &mut impl Write, s: &str) -> io::Result<()> {
    put64(w, s.len() as u64)?;
    w.write_all(s.as_bytes())
}

fn get_str(r: &mut impl Read) -> io::Result<String> {
    let n = get64(r)?;
    if n > 4096 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "plate: a name too long"));
    }
    let mut b = vec![0u8; n as usize];
    r.read_exact(&mut b)?;
    String::from_utf8(b).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "plate: a name that isn't text"))
}

#[cfg(test)]
mod tests;
