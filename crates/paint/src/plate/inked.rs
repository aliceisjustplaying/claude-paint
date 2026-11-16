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

/// The card (IP §5.2): pressed with 10 N [E: a hand pushing a card's edge]
/// on a bead 50 mm long and 2 mm across [E], so 100 kPa in the bead; drawn at
/// 20 mm/s [E] with a bead 5 mm long in the way it moves [E], so a point
/// lies under the bead 0.25 s, twice over (across, then diagonally [P]).
pub const CARD_PA: f64 = 10.0 / (50e-3 * 2e-3);
pub const CARD_DWELL_S: f64 = 2.0 * 5.0 / 20.0;
/// The card's edge drags a film behind it as a sliding pad does (Reynolds'
/// inclined slider, IP §5.2 [D]): h = √(6·K·η·U·ℓ/p), K = 0.0267 at the
/// best inclination [M, Reynolds' slider solution], U 20 mm/s and the edge
/// 1 mm across in the way it moves [E: a card's bevel], p `CARD_PA`.
pub const SLIDER_K: f64 = 0.0267;
pub const CARD_MM_S: f64 = 20.0;
pub const CARD_EDGE_MM: f64 = 1.0;
/// The dabber (a roll of felt worked with a twist, IP §5.2): 20 N on a face
/// 30 mm across [E], ten dabs on a point of 0.2 s each [E].
pub const DABBER_PA: f64 = 20.0 / (std::f64::consts::PI * 0.015 * 0.015);
pub const DABBER_DWELL_S: f64 = 10.0 * 0.2;
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
/// Times a hand wipe's pass touches each point [E: the palm sweeps the plate
/// in overlapping strokes].
pub const PALM_TOUCHES: f64 = 4.0;
/// Retroussage (IP §5.4): a soft muslin passed lightly over the plate. Its
/// loose fibre ends, cotton 15 µm thick [M: cotton fibres are 12–20 µm
/// across], hang into any opening wider than a fibre and touch the ink in
/// it; it draws ink up into its fibres by capillarity (Lucas–Washburn, h =
/// √(γ·r·t/(2η))): linseed oil's surface tension 0.033 N/m [M], the pores
/// between its fibres 10 µm [E], 0.05 s on a point a pass [E]. What it draws
/// up it lays beside the line within 0.5 mm [E: the cloth's contact along
/// its pass], but for what soaks into it (`rag`'s soaking share).
pub const FIBRE_UM: f64 = 15.0;
pub const OIL_TENSION: f64 = 0.033;
pub const MUSLIN_PORE_UM: f64 = 10.0;
pub const RETRO_TOUCH_S: f64 = 0.05;
pub const RETRO_BAND_MM: f64 = 0.5;

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
pub mod pace {
    pub const CARD_MM2_S: f64 = 72_800.0 / 262.5;
    pub const DABBER_MM2_S: f64 = 72_800.0 / 262.5 / 2.0;
    pub const TARLATAN_MM2_S: f64 = 3.0 * 72_800.0 / 525.0;
    pub const PALM_MM2_S: f64 = 2.0 * 72_800.0 / 262.5;
    /// Retroussage: a light pass, as fast as the palm's [E].
    pub const RETRO_MM2_S: f64 = PALM_MM2_S;
    /// Warming a plate on the hot plate: copper's ρct over a contact
    /// conductance of 300 W/m²K is 15 s, four of them to come within 2 % of
    /// the hot plate [D; the conductance E], with the reach to it and back.
    pub const WARM_S: f64 = 60.0 + 4.0 * 15.0;
    /// Cleaning ink off a plate with solvent and rag [E].
    pub const CLEAN_S: f64 = 300.0;
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

/// The film (µm) a card's edge leaves on the flat (`SLIDER_K`).
pub fn card_film_um(eta: f64) -> f64 {
    (6.0 * SLIDER_K * eta * CARD_MM_S * 1e-3 * CARD_EDGE_MM * 1e-3 / CARD_PA).sqrt() * 1e6
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

/// How ink goes on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inker {
    Card,
    Dabber,
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

/// How a wipe's pad moves: in circles (every way across the plate in turn)
/// or back and forth along a direction (degrees: 0 to the right, 90 down).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pattern {
    Circles,
    Along(f32),
}

impl Pattern {
    /// The ways the pad moves over a pass (unit vectors).
    fn ways(self) -> Vec<(f64, f64)> {
        match self {
            Pattern::Circles => (0..8).map(|i| {
                let a = i as f64 * std::f64::consts::TAU / 8.0;
                (a.cos(), a.sin())
            }).collect(),
            Pattern::Along(deg) => {
                let a = (deg as f64).to_radians();
                vec![(a.cos(), a.sin()), (-a.cos(), -a.sin())]
            }
        }
    }
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

    fn ink_cell_mut(&mut self, x: usize, y: usize) -> &mut InkCell {
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

    /// The cells under `mask` (one value a cell, row by row; None: all) at
    /// least half in it.
    fn masked(&self, mask: Option<&[f32]>) -> Vec<(usize, usize, f32)> {
        let mut out = Vec::new();
        for y in 0..self.h {
            for x in 0..self.w {
                let m = mask.map_or(1.0, |m| m[y * self.w + x].clamp(0.0, 1.0));
                if m > 0.0 {
                    out.push((x, y, m));
                }
            }
        }
        out
    }

    // ------------------------------------------------------------ inking

    /// Ink the plate under `mask` (cells at least half in it) by card or
    /// dabber at `now` (IP §5.2): each opening fills to its share (`fill`:
    /// the ink pushed in viscously or against its yield stress, at most
    /// `PHI_AIR`), and the flat keeps the film the card's edge drags
    /// (`card_film_um`) or the dabber leaves.
    pub fn ink(&mut self, ink: &Ink, mask: Option<&[f32]>, by: Inker, now: f64) -> Result<InkReport, String> {
        self.age_ink(now);
        let idx = self.ink_index(ink)?;
        let temp = self.temp_c(now);
        let eta = ink.eta(temp);
        let (p, t) = match by {
            Inker::Card => (CARD_PA, CARD_DWELL_S),
            Inker::Dabber => (DABBER_PA, DABBER_DWELL_S),
        };
        let film = match by {
            Inker::Card => card_film_um(eta),
            Inker::Dabber => DABBER_FILM_UM,
        };
        let mut of = [0.0f32; INKS];
        of[idx] = 1.0;
        let before = self.ink_um3();
        let cells = self.masked(mask);
        let (mut fsum, mut fvol, mut n) = (0.0f64, 0.0f64, 0usize);
        for &(x, y, m) in &cells {
            if m < 0.5 {
                continue;
            }
            n += 1;
            let c = self.cell(x, y);
            let mut ic = self.ink_cell(x, y);
            let had = ic.total();
            for k in 0..BINS {
                if c.open[k] <= 0.0 || c.vol[k] <= 0.0 {
                    continue;
                }
                let (w, d) = ((c.wsum[k] / c.open[k]) as f64, (c.vol[k] / c.open[k]) as f64);
                let phi = fill(w, d, p, t, eta, YIELD_PA);
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
        let area = n as f64 * (self.pitch * 1e-3).powi(2);
        let secs = area / match by {
            Inker::Card => pace::CARD_MM2_S,
            Inker::Dabber => pace::DABBER_MM2_S,
        };
        Ok(InkReport { added_um3: after.0 + after.1 - before.0 - before.1, film_um: film, fill: if fvol > 0.0 { fsum / fvol } else { 0.0 }, temp_c: temp, eta, area_mm2: area, secs })
    }

    /// Solvent and a rag: every bit of ink off the plate (µm³ taken).
    pub fn clean_ink(&mut self) -> f64 {
        let (l, f) = self.ink_um3();
        if let Some(k) = self.ink.as_deref_mut() {
            for t in k.tiles.iter_mut() {
                *t = None;
            }
        }
        l + f
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
            let flat = ic.film as f64 * fr * (1.0 - sf);
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
    /// over cells) and its shares, summed.
    fn take(&mut self, asks: &[(usize, usize, f64, [f64; BINS])], scale: f64) -> (f64, [f64; INKS]) {
        let (mut got, mut mix) = (0.0f64, [0.0f64; INKS]);
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
        }
        (got, mix)
    }

    /// Lay `um` (µm over cells) of ink of shares `mix` back on `cells`, by
    /// their weights (all alike if none weighs anything): on each cell's
    /// flat by its share of the cell, into its openings by theirs.
    fn lay(&mut self, cells: &[(usize, usize, f64)], um: f64, mix: &[f32; INKS], cure: f32) {
        let total: f64 = cells.iter().map(|c| c.2).sum();
        if cells.is_empty() || um <= 0.0 {
            return;
        }
        let even = total <= 0.0;
        for &(x, y, wt) in cells {
            let a = if even { um / cells.len() as f64 } else { um * wt / total };
            if a <= 0.0 {
                continue;
            }
            let c = self.cell(x, y);
            let open = c.open_share().min(1.0) as f64;
            let ic = self.ink_cell_mut(x, y);
            let had = ic.total();
            ic.film += (a * (1.0 - open)) as f32;
            if open > 0.0 {
                for k in 0..BINS {
                    ic.line[k] += (a * c.open[k] as f64 / c.open_share() as f64 * open) as f32;
                }
            }
            ic.blend(had, a as f32, mix, cure);
        }
    }

    /// Wipe the plate under `mask` with tarlatan `rag` (its face, load and
    /// creases a rag's; its stiffness `rag.cloth`) at `pressure` 0..1 for
    /// `passes`, the pad moving by `pattern`, at `now` (IP §5.3).
    ///
    /// Each pass is a sweep of the pad over the area each way the pattern
    /// moves. At each cell, the cloth takes by the rag's lift law (its face's
    /// thirst, its creases' contact, the ink's fluidity) what it reaches:
    /// - the film on the flat, but in a burr's lee (`shields`, `lee_um`);
    /// - the ink in each opening above the level it reaches: the yarn's dip
    ///   (`yarn_dip_um`) and the cloth's sag across the opening (`sag_um`,
    ///   at the cell's distance from the rim of a wide one).
    ///
    /// The face holds no more than it has room for, and smears back the
    /// share `rag::SMEAR` of the ink at its surface every quarter of its
    /// width it slides, as the rag does, of which `rag::SOAK` soaks in: what
    /// it lays back is the plate tone.
    #[allow(clippy::too_many_arguments)]
    pub fn wipe(&mut self, rag: &mut Rag, mask: Option<&[f32]>, pressure: f32, passes: u32, pattern: Pattern, now: f64) -> WipeReport {
        self.age_ink(now);
        let temp = self.temp_c(now);
        let before = self.ink_um3();
        let cloth = rag.cloth.get_or_insert_with(|| Box::new(Tarlatan::new(0.7))).as_ref().clone();
        let b = cloth.bend_nm;
        let p = PAD_PA * pressure.clamp(0.01, 1.0) as f64;
        let ct = Contact {
            reach: Box::new(move |w, x| yarn_dip_um(YARN_UM, w.min(2.0 * YARN_UM)) + if w > 2.0 * YARN_UM { sag_um(w, x, p, b) } else { 0.0 }),
            lee: Box::new(move |h| lee_um(h, p, b)),
        };
        let cells = self.masked(mask);
        let rims = self.rims();
        let ways = pattern.ways();
        let kk = ways.len() as f64;
        let area_um2 = cells.iter().map(|c| c.2 as f64).sum::<f64>() * self.pitch * self.pitch;
        let pad_um = rag.width as f64 * 1000.0;
        let cap = pad_um * pad_um * crate::rag::CAP_UM as f64;
        // quarter-pad steps a sweep slides (the area over the pad's width, K ways a pass)
        let quarters = (4.0 * area_um2 / (pad_um * pad_um * kk)).max(1e-3);
        let (mut taken, mut laid) = (0.0f64, 0.0f64);
        let pressure = pressure.clamp(0.0, 1.0);
        let unit = self.pitch * self.pitch;
        for pass in 0..passes {
            for (j, &v) in ways.iter().enumerate() {
                let rate = crate::rag::LIFT as f64 * (0.7 + 0.6 * pressure as f64) * rag.thirst() as f64 / kk;
                let cs = rag.cloth_seed(((pass as u64) << 8) | j as u64);
                let pitch = self.pitch;
                let frac = |x: usize, y: usize, m: f32, ic: &InkCell| {
                    // the pad's creases, in its own frame (mm), moving `v`
                    let (px, py) = ((x as f64 + 0.5) * pitch * 1e-3, (y as f64 + 0.5) * pitch * 1e-3);
                    let (u, w) = (px * v.1 - py * v.0, px * v.0 + py * v.1);
                    let c = crate::rag::cloth3(u as f32, w as f32, cs, pressure) as f64;
                    let fl = crate::drying::fluid(ic.cure) as f64;
                    (1.0 - (-rate * fl).exp()) * c * m as f64
                };
                let asks = self.reach_sweep(&cells, v, &ct, &rims, &frac);
                let want: f64 = asks.iter().map(|a| a.2 + a.3.iter().sum::<f64>()).sum::<f64>() * unit;
                let room = (1.0 - rag.load as f64).max(0.0) * cap;
                let scale = if want > room && want > 0.0 { room / want } else { 1.0 };
                let (got, mix) = self.take(&asks, scale);
                let got_um3 = got * unit;
                // the face's surface: laid back as it slides, soaking in
                let cl = rag.cloth.as_mut().unwrap();
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
                // laid where the cloth touched, out of the lee
                let shield = self.shields(v, &*ct.lee);
                let wts: Vec<(usize, usize, f64)> = cells.iter().map(|&(x, y, m)| {
                    let sf = shield.as_ref().map_or(0.0, |(f, _)| f[y * self.w + x] as f64);
                    (x, y, m as f64 * (1.0 - sf))
                }).collect();
                self.lay(&wts, lay_um3 / unit, &lay_shares, 0.0);
                let cl = rag.cloth.as_mut().unwrap();
                cl.held_um3 += got_um3 - lay_um3;
                taken += got_um3;
                laid += lay_um3;
                let net_mm3 = (got_um3 - lay_um3) * 1e-9;
                rag.load = (rag.load as f64 + net_mm3 / (cap * 1e-9)).clamp(0.0, 1.0) as f32;
                rag.soaked = (rag.soaked as f64 + net_mm3 / (cap * 1e-9 * crate::rag::FACES as f64)).clamp(0.0, 1.0) as f32;
            }
        }
        let after = self.ink_um3();
        let area_mm2 = area_um2 * 1e-6;
        self.wipe_report(before, after, taken, laid, area_mm2, passes as f64 * area_mm2 / pace::TARLATAN_MM2_S, temp)
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

    /// The hand wipe (IP §5.4): the palm, soft, pressed at `pressure` 0..1
    /// for `passes`. Skin bulges into each opening by `palm_dip_um` (the top
    /// few µm of a fine line, the whole of an open bite), is held off past a
    /// burr by `palm_lee_um`, and parts from the film it touches taking its
    /// split share (`PALM_SPLIT`, or `WHITING_SPLIT` with whiting), each of
    /// `PALM_TOUCHES` times a pass. The printer wipes the palm clean between
    /// strokes: what it takes leaves the plate.
    #[allow(clippy::too_many_arguments)]
    pub fn hand_wipe(&mut self, mask: Option<&[f32]>, pressure: f32, passes: u32, whiting: bool, pattern: Pattern, now: f64) -> WipeReport {
        self.age_ink(now);
        let temp = self.temp_c(now);
        let before = self.ink_um3();
        let p = PALM_PA * pressure.clamp(0.01, 1.0) as f64;
        let ct = Contact { reach: Box::new(move |w, x| palm_dip_um(w, x, p)), lee: Box::new(move |h| palm_lee_um(h, p)) };
        let cells = self.masked(mask);
        let rims = self.rims();
        let ways = pattern.ways();
        let kk = ways.len() as f64;
        let q = if whiting { WHITING_SPLIT } else { PALM_SPLIT };
        // (each way, the touches of a pass shared out: (1 − q)^(touches/K) left of what it reaches)
        let keep = (1.0 - q).powf(PALM_TOUCHES / kk);
        let unit = self.pitch * self.pitch;
        let mut taken = 0.0f64;
        for _ in 0..passes {
            for &v in &ways {
                let frac = |_: usize, _: usize, m: f32, ic: &InkCell| (1.0 - keep) * crate::drying::fluid(ic.cure) as f64 * m as f64;
                let asks = self.reach_sweep(&cells, v, &ct, &rims, &frac);
                taken += self.take(&asks, 1.0).0 * unit;
            }
        }
        let after = self.ink_um3();
        let area_mm2 = cells.iter().map(|c| c.2 as f64).sum::<f64>() * unit * 1e-6;
        self.wipe_report(before, after, taken, 0.0, area_mm2, passes as f64 * area_mm2 / pace::PALM_MM2_S, temp)
    }

    /// Retroussage (IP §5.4): a soft muslin passed lightly over the wiped
    /// plate `passes` times. Its fibre ends hang into every opening wider
    /// than a fibre (`FIBRE_UM`) and draw up as much as capillarity lifts in
    /// their touch (`washburn_um`: more on a warm plate, the ink thinner),
    /// and lay it on the plate within `RETRO_BAND_MM` around, but for the
    /// share that soaks into the cloth (`rag::SOAK`), which leaves the plate.
    pub fn retrousse(&mut self, mask: Option<&[f32]>, passes: u32, now: f64) -> WipeReport {
        self.age_ink(now);
        let temp = self.temp_c(now);
        let before = self.ink_um3();
        let cells = self.masked(mask);
        let unit = self.pitch * self.pitch;
        let band = ((RETRO_BAND_MM * 1000.0 / self.pitch).round() as i64).max(1);
        let inks: Vec<Ink> = self.ink.as_ref().map(|k| k.inks.clone()).unwrap_or_default();
        let (mut taken, mut laid) = (0.0f64, 0.0f64);
        for _ in 0..passes {
            let mut moves: Vec<(usize, usize, f64, [f32; INKS], f32)> = Vec::new();
            for &(x, y, m) in &cells {
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
                self.lay(&wts, lay, &sh, cure);
            }
        }
        let after = self.ink_um3();
        let area_mm2 = cells.iter().map(|c| c.2 as f64).sum::<f64>() * unit * 1e-6;
        self.wipe_report(before, after, taken, laid, area_mm2, passes as f64 * area_mm2 / pace::RETRO_MM2_S, temp)
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
