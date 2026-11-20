//! Grounds, acid and aquatint: the physics a plate's etching tools use
//! (notes/plan-intaglio.md §4, phase P4; the easel's `--@ etch`). Sources:
//! notes/research/intaglio_physics.md (**IP §n**) and
//! notes/research/etch_physics.md (**EP §n**), with their tags: [M]
//! measured, [D] derived, [P] practitioners, [E] estimate.
//!
//! - **The bath** (EP §2). Every mordant here bites at the rate its oxidant
//!   reaches the metal: r = V_M·k·C/ν, with V_M the metal's molar volume, C
//!   the oxidant's concentration, ν the oxidant used per atom of metal and k
//!   the mass-transfer coefficient (D/δ). k comes from the standard
//!   correlations of natural and forced convection, with D from the
//!   Nernst–Hartley salt diffusivity and the bath's measured viscosity
//!   (Stokes–Einstein), and the solution the bite leaves at the face (denser:
//!   it holds the dissolved metal) driving the convection. Ferric chloride's
//!   temperature law is its measured activation energy, 24.9 kJ/mol [M]; its
//!   optimum near 2–2.25 M [M] comes out of the viscosity table, not a rule.
//! - **Undercut** (IP §3.4): the acid bites sideways under a ground at 1/K
//!   of its rate down, and a narrow line bites slower as it deepens, its acid
//!   diffusing down the slot.
//! - **Aquatint** (IP §4): a dust box sorts rosin by Stokes settling; the
//!   grains that land cover the plate as random discs (a Boolean model); the
//!   bite eats under them until each dot is lost.
//! - **The look** of a ground or a varnish: a thin absorbing film over the
//!   metal (bitumen), with the metal's Fresnel reflection beneath it.

use crate::plate::{Metal, fresnel, nk};
use crate::spectral::{self, Spectrum};
use std::f64::consts::PI;

// ---------------------------------------------------------------- constants

/// The gas constant (J/(mol·K)) and standard gravity (m/s²), exact.
pub const R_GAS: f64 = 8.314_462_618;
pub const G: f64 = 9.806_65;
const T20: f64 = 293.15;
const T25: f64 = 298.15;
/// Water's viscosity at 25 °C (mPa·s) [M, CRC Handbook].
const MU_W25: f64 = 0.8900;

/// Water's viscosity (mPa·s) at `t_k`: Vogel's equation, μ = A·10^(B/(T − C)),
/// A = 2.414 × 10⁻⁵ Pa·s, B = 247.8 K, C = 140 K [M, the standard fit to
/// water's measured viscosity, 0–100 °C; 1.002 mPa·s at 20 °C].
pub fn water_viscosity(t_k: f64) -> f64 {
    2.414e-2 * 10f64.powf(247.8 / (t_k - 140.0))
}

/// A metal's molar mass (kg/mol) and molar volume (m³/mol): copper 63.546 g/mol
/// at 8.96 g/cm³, zinc 65.38 g/mol at 7.14 g/cm³ [M, handbook].
pub fn molar(metal: Metal) -> (f64, f64) {
    match metal {
        Metal::Copper => (63.546e-3, 63.546e-3 / 8960.0),
        Metal::Zinc => (65.38e-3, 65.38e-3 / 7140.0),
    }
}

/// Limiting ionic diffusion coefficients at 25 °C (m²/s) [M, CRC Handbook,
/// "Ionic conductivity and diffusion at infinite dilution"].
const D_FE3: f64 = 0.604e-9;
const D_CL: f64 = 2.032e-9;
const D_H: f64 = 9.311e-9;
const D_NO3: f64 = 1.902e-9;
const D_CLO3: f64 = 1.720e-9;

/// The diffusivity of a salt of cation charge `zp` and anion charge `zm`
/// from its ions' (Nernst–Hartley): D = (z₊ + |z₋|)·D₊D₋/(|z₋|D₊ + z₊D₋).
/// The neutral salt is what diffuses where no other electrolyte carries the
/// current [D, standard electrolyte transport].
pub fn nernst_hartley(dp: f64, zp: f64, dm: f64, zm: f64) -> f64 {
    (zp + zm) * dp * dm / (zm * dp + zp * dm)
}

/// Ferric chloride solutions at 20 °C: (mol/L, g/cm³, mPa·s) [M, CRC Handbook
/// "Concentrative properties of aqueous solutions" (Wolf 1966; Söhnel &
/// Novotný 1985), 0.5–40 % FeCl₃].
const FECL3: [(f64, f64, f64); 21] = [
    (0.031, 1.0025, 1.024),
    (0.062, 1.0068, 1.047),
    (0.125, 1.0153, 1.093),
    (0.189, 1.0238, 1.139),
    (0.255, 1.0323, 1.187),
    (0.321, 1.0408, 1.238),
    (0.388, 1.0493, 1.292),
    (0.457, 1.0580, 1.350),
    (0.526, 1.0668, 1.412),
    (0.597, 1.0760, 1.480),
    (0.669, 1.0853, 1.553),
    (0.817, 1.1040, 1.707),
    (0.969, 1.1228, 1.879),
    (1.126, 1.1420, 2.080),
    (1.289, 1.1615, 2.311),
    (1.457, 1.1816, 2.570),
    (1.810, 1.2234, 3.178),
    (2.189, 1.2679, 4.038),
    (2.595, 1.3153, 5.274),
    (3.030, 1.3654, 7.130),
    (3.496, 1.4176, 9.674),
];
/// The same for nitric acid, 0.5–40 % HNO₃ [M, CRC as above].
const HNO3: [(f64, f64, f64); 23] = [
    (0.079, 1.0009, 1.004),
    (0.159, 1.0037, 1.005),
    (0.320, 1.0091, 1.007),
    (0.483, 1.0146, 1.010),
    (0.648, 1.0202, 1.014),
    (0.814, 1.0257, 1.018),
    (0.982, 1.0314, 1.022),
    (1.152, 1.0370, 1.027),
    (1.324, 1.0427, 1.032),
    (1.498, 1.0485, 1.038),
    (1.673, 1.0543, 1.044),
    (2.030, 1.0660, 1.058),
    (2.395, 1.0780, 1.075),
    (2.768, 1.0901, 1.094),
    (3.149, 1.1025, 1.116),
    (3.539, 1.1150, 1.141),
    (3.937, 1.1277, 1.169),
    (4.344, 1.1406, 1.199),
    (4.760, 1.1536, 1.233),
    (5.185, 1.1668, 1.271),
    (5.618, 1.1801, 1.311),
    (6.060, 1.1934, 1.354),
    (7.913, 1.2466, 1.561),
];
/// Hydrochloric acid at 1.1 M (4 %), Dutch mordant's: 1.0179 g/cm³, 1.059
/// mPa·s at 20 °C [M, CRC as above].
const HCL_DUTCH: (f64, f64) = (1.0179, 1.059);

/// (density kg/m³, viscosity mPa·s) at 20 °C from a table, linear in molarity.
fn table(t: &[(f64, f64, f64)], c: f64) -> (f64, f64) {
    let c = c.clamp(t[0].0, t[t.len() - 1].0);
    let i = t.iter().position(|p| p.0 >= c).unwrap_or(t.len() - 1).max(1);
    let (a, b) = (t[i - 1], t[i]);
    let u = ((c - a.0) / (b.0 - a.0)).clamp(0.0, 1.0);
    ((a.1 + (b.1 - a.1) * u) * 1000.0, a.2 + (b.2 - a.2) * u)
}

/// Ferric chloride's molarity from its Baumé degrees: the heavy-liquid scale
/// SG = 145/(145 − °Bé) [M, the scale's definition], then the CRC table's
/// density read back to its molarity.
pub fn ferric_molar_of_baume(baume: f64) -> f64 {
    let sg = 145.0 / (145.0 - baume);
    let t = &FECL3;
    let rho = sg.clamp(t[0].1, t[t.len() - 1].1);
    let i = t.iter().position(|p| p.1 >= rho).unwrap_or(t.len() - 1).max(1);
    let (a, b) = (t[i - 1], t[i]);
    a.0 + (b.0 - a.0) * ((rho - a.1) / (b.1 - a.1)).clamp(0.0, 1.0)
}

/// Concentrated nitric acid: 70 % w/w, 15.8 mol/L [M, the reagent's
/// specification]. `parts` of water to one of it, by volume, make
/// 15.8/(1 + parts) mol/L [D, neglecting the small contraction on mixing].
pub const NITRIC_CONC_M: f64 = 15.8;

/// Dutch mordant (1911 Britannica's recipe [M, a recipe]: hydrochloric acid
/// 100 g, potassium chlorate 20 g, water 880 g): chlorate 20/122.55 mol in
/// about a litre.
pub const DUTCH_CHLORATE_M: f64 = 20.0 / 122.55 / 0.99;
/// Ferric chloride's activation energy on copper, 24.9 kJ/mol: "inward
/// diffusion … the rate-controlling step" [M, Hydrometallurgy 2010; IP §3.1].
pub const EA_FERRIC: f64 = 24.9e3;
/// Dutch mordant's rate at 20 °C, 0.5 µm/min [E: no measured rate exists
/// (IP §3.2); it "acts slowly", and "a very fine-grained aquatint could
/// over-etch after only 10 minutes" in it (IP §4.2 [P]): a 10 µm grain lost
/// to an isotropic undercut of 5 µm in 10 min]. Its chlorate reacts at the
/// surface slower than a stirred bath brings it; a still one is diffusion's.
pub const DUTCH_UM_MIN: f64 = 0.5;
/// Its activation energy, 50 kJ/mol [E: the textbook range of a surface
/// reaction's control, 40–60 kJ/mol, against diffusion's < 25; unmeasured].
pub const EA_DUTCH: f64 = 50e3;

/// Surface tension of water, 0.072 N/m at 20 °C [M], for a bubble's size.
const SIGMA_W: f64 = 0.072;
/// Nitric's gas bubbles: their contact angle on the bitten metal, 40° [E: no
/// measurement on etched zinc; a typical angle of a gas bubble on a wetted
/// metal], and the share of the face they cover when nobody sweeps them, 0.5
/// [E: bubbles grown to their departure size sit about a diameter apart].
const BUBBLE_ANGLE_DEG: f64 = 40.0;
const BUBBLE_COVER: f64 = 0.5;
/// A feather swept over the plate every 30 s when `feather = true` [E: the
/// manuals say "brushed away", without a period].
pub const FEATHER_S: f64 = 30.0;
/// A rocked tray: tilted ±5° each 4 s [E: a hand's rocking, unmeasured].
pub const ROCK_TILT_DEG: f64 = 5.0;
pub const ROCK_PERIOD_S: f64 = 4.0;
/// The tray: a bath 20 mm deep, 50 mm beyond the plate on each side [E: a
/// printmaker's tray, unmeasured].
pub const TRAY_DEPTH_M: f64 = 0.020;
pub const TRAY_MARGIN_M: f64 = 0.050;

// ---------------------------------------------------------------- the bath

/// A mordant (the 1890s Paris box's, plan-intaglio I5, with ferric
/// chloride, which a printmaker of then would have known from photogravure).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acid {
    /// Ferric chloride: Cu (or Zn) + 2 Fe³⁺ → Cu²⁺ + 2 Fe²⁺, no gas.
    Ferric,
    /// Nitric acid: 3 Cu + 8 HNO₃ → 3 Cu(NO₃)₂ + 2 NO + 4 H₂O; in dilute acid
    /// 4 Zn + 10 HNO₃ → 4 Zn(NO₃)₂ + N₂O + 5 H₂O [M, textbook stoichiometry].
    Nitric,
    /// Dutch mordant: chlorate in hydrochloric acid, ClO₃⁻ + 6 H⁺ + 3 Cu →
    /// Cl⁻ + 3 Cu²⁺ + 3 H₂O, no gas (it "causes no ebullition", 1911
    /// Britannica).
    Dutch,
}

impl Acid {
    pub fn word(self) -> u64 {
        match self {
            Acid::Ferric => 0,
            Acid::Nitric => 1,
            Acid::Dutch => 2,
        }
    }

    pub fn of(w: u64) -> Option<Acid> {
        [Acid::Ferric, Acid::Nitric, Acid::Dutch].into_iter().find(|a| a.word() == w)
    }

    pub fn name(self) -> &'static str {
        match self {
            Acid::Ferric => "ferric",
            Acid::Nitric => "nitric",
            Acid::Dutch => "dutch",
        }
    }

    /// Mol of oxidant used per mol of metal, and mol of gas given off.
    pub fn stoichiometry(self, metal: Metal) -> (f64, f64) {
        match (self, metal) {
            (Acid::Ferric, _) => (2.0, 0.0),
            (Acid::Nitric, Metal::Copper) => (8.0 / 3.0, 2.0 / 3.0),
            (Acid::Nitric, Metal::Zinc) => (2.5, 0.25),
            (Acid::Dutch, _) => (1.0 / 3.0, 0.0),
        }
    }

    /// The etch factor K, depth over the sideways bite under a ground on one
    /// side (IP §3.4): ferric 1.2 [D: a still, diffusion-limited bath bites
    /// almost isotropically, 1–1.5; the corner under the ground is shadowed
    /// from the mouth]; nitric 1.0 [E: IP's 1–2, its low end: it "underbites"];
    /// Dutch 1.0 [D: a surface reaction bites every bare face at one rate].
    pub fn etch_factor(self) -> f64 {
        match self {
            Acid::Ferric => 1.2,
            Acid::Nitric | Acid::Dutch => 1.0,
        }
    }

    /// The oxidant's diffusivity at 25 °C in water (m²/s): ferric chloride's
    /// and nitric acid's as salts (Nernst–Hartley); chlorate as a minor ion in
    /// excess hydrochloric acid, its own.
    fn d25(self) -> f64 {
        match self {
            Acid::Ferric => nernst_hartley(D_FE3, 3.0, D_CL, 1.0),
            Acid::Nitric => nernst_hartley(D_H, 1.0, D_NO3, 1.0),
            Acid::Dutch => D_CLO3,
        }
    }

    /// The bath's density (kg/m³) and viscosity (mPa·s) at 20 °C, made up at
    /// `molar` of its oxidant.
    fn liquid(self, molar: f64) -> (f64, f64) {
        match self {
            Acid::Ferric => table(&FECL3, molar),
            Acid::Nitric => table(&HNO3, molar),
            Acid::Dutch => (HCL_DUTCH.0 * 1000.0, HCL_DUTCH.1),
        }
    }
}

/// How the plate lies in the bath.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Face {
    /// Face down, held off the tray's floor: the dense spent acid falls away
    /// from it (Kiekeben's way with ferric [P]).
    Down,
    /// Upright in a tank: the spent acid runs down its face.
    Upright,
    /// Face up in a tray: the spent acid, denser, lies on it. Still, the
    /// oxidant reaches it only by diffusion; rocked, the bath sweeps it.
    Up,
}

impl Face {
    pub fn word(self) -> u64 {
        match self {
            Face::Down => 0,
            Face::Upright => 1,
            Face::Up => 2,
        }
    }

    pub fn of(w: u64) -> Option<Face> {
        [Face::Down, Face::Upright, Face::Up].into_iter().find(|f| f.word() == w)
    }

    pub fn name(self) -> &'static str {
        match self {
            Face::Down => "down",
            Face::Upright => "upright",
            Face::Up => "up",
        }
    }
}

/// A bite's conditions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bath {
    pub acid: Acid,
    /// The oxidant now in the bath (mol/L) and as it was made up (its
    /// viscosity's).
    pub molar: f64,
    pub molar0: f64,
    pub temp_c: f64,
    pub face: Face,
    /// Rocked: tilt (degrees) and period (s).
    pub rock: Option<(f64, f64)>,
    /// Feathered: seconds between sweeps.
    pub feather: Option<f64>,
}

/// What the bath does to a face at a moment.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Transport {
    /// The rate it bites a flat bare face (µm/min), the diffusion layer's
    /// thickness (µm), the share of the face under bubbles, and the
    /// diffusion coefficient (m²/s) and Rayleigh number it was found with.
    pub rate_um_min: f64,
    pub delta_um: f64,
    pub bubbles: f64,
    pub d: f64,
    pub ra: f64,
}

/// The mass-transfer coefficient (m/s) of natural convection over a plate
/// whose spent acid, `drho_rel` denser, falls away (`Face::Down`: L = area
/// over perimeter; the "lower face of a cold plate", Sh = 0.54 Ra^¼ up to
/// Ra 10⁷, 0.15 Ra^⅓ above [M, Lloyd & Moran 1974; Incropera]) or runs down
/// it (`Face::Upright`: L its height; Churchill & Chu 1975, the whole range).
/// Returns (k, Ra).
pub fn natural(face: Face, drho_rel: f64, l: f64, nu: f64, d: f64) -> (f64, f64) {
    let ra = G * drho_rel.max(0.0) * l.powi(3) / (nu * d);
    let sc = nu / d;
    let sh = match face {
        Face::Down => {
            if ra < 1e7 {
                0.54 * ra.powf(0.25)
            } else {
                0.15 * ra.cbrt()
            }
        }
        Face::Upright => {
            let f = (1.0 + (0.492 / sc).powf(9.0 / 16.0)).powf(8.0 / 27.0);
            (0.825 + 0.387 * ra.powf(1.0 / 6.0) / f).powi(2)
        }
        Face::Up => 0.0,
    };
    (sh * d / l, ra)
}

/// The mass-transfer coefficient (m/s) of a rocked tray's flow over a plate
/// `l` m long: laminar flow over a flat plate, Sh = 0.664 Re^½ Sc^⅓ [M,
/// Pohlhausen; Incropera]. The speed is the bath's as the tray tilts: rocked
/// slowly (against its sloshing period 2L/√(gh)), its surface stays level and
/// a wedge of it, L²θ/8 of section, crosses the middle each quarter period,
/// so U = L²θ/(2hT) at most there, (2/π)U on average [D].
pub fn rocked(tilt_deg: f64, period_s: f64, tray_m: f64, depth_m: f64, l: f64, nu: f64, d: f64) -> f64 {
    let u = 2.0 / PI * tray_m * tray_m * tilt_deg.to_radians() / (2.0 * depth_m * period_s);
    let re = u * l / nu;
    0.664 * re.sqrt() * (nu / d).cbrt() * d / l
}

/// The bath's bite on a flat bare face of `metal`, `t_s` seconds after it
/// went in (a still face-up plate's diffusion layer grows with it). `l_nat`
/// is the plate's length for natural convection (its area over perimeter
/// face down, its height upright), `l_flow` its length along the rocking.
///
/// - **D** at 20 °C from the oxidant's at 25 °C in water by Stokes–Einstein,
///   D ∝ T/μ, with the bath's measured viscosity [D].
/// - **Δρ**: where the oxidant runs out the bath holds the dissolved metal,
///   M_metal·C/ν more per volume [D, neglecting the solution's small change of
///   volume]; that drives the natural convection.
/// - **The temperature.** Ferric: the measured activation energy, Arrhenius
///   from 20 °C [M]. Nitric: dilute, so its D and viscosity follow water's
///   [D]. Dutch: a surface reaction [E], `DUTCH_UM_MIN` and `EA_DUTCH`, but
///   never faster than its chlorate arrives.
/// - **Nitric's bubbles** cover a share of the face, under which it doesn't
///   bite. A bubble grows until it leaves at Fritz's diameter, 0.0208·θ·√(σ/gΔρ)
///   [M, Fritz 1935]; a face holds (2/3)·d·share of gas as bubbles of d; the
///   gas comes at its stoichiometric share of the metal bitten (as an ideal
///   gas at the bath's temperature). Swept every T_f by a feather, the cover
///   grows back toward `BUBBLE_COVER` with time constant τ = (2/3)·d·θ_max/q,
///   and its mean over a sweep is θ_max·(1 − (τ/T_f)(1 − e^(−T_f/τ))) [D].
pub fn transport(bath: &Bath, metal: Metal, l_nat: f64, l_flow: f64, t_s: f64) -> Transport {
    let (m_metal, v_metal) = molar(metal);
    let (nu_ox, gas) = bath.acid.stoichiometry(metal);
    let tk = bath.temp_c + 273.15;
    let (rho, mu20) = bath.acid.liquid(bath.molar0);
    // the viscosity and the diffusion coefficient at the bath's temperature (ferric: at 20 °C,
    // its measured activation energy then taking it to the bath's)
    let (mu, t_d) = match bath.acid {
        Acid::Ferric => (mu20, T20),
        Acid::Nitric | Acid::Dutch => (mu20 * water_viscosity(tk) / water_viscosity(T20), tk),
    };
    let d = bath.acid.d25() * (MU_W25 / mu) * (t_d / T25);
    let nu = mu * 1e-3 / rho;
    let c = bath.molar.max(0.0) * 1000.0;
    let drho_rel = m_metal * c / nu_ox / rho;
    let (mut k, mut ra) = (0.0, 0.0);
    match (bath.face, bath.rock) {
        (Face::Up, None) => {
            // a stable layer: diffusion alone, the flux C·√(D/(πt)) of a semi-infinite one
            k = (d / (PI * t_s.max(1.0))).sqrt();
        }
        (face, rock) => {
            if face != Face::Up {
                (k, ra) = natural(face, drho_rel, l_nat, nu, d);
            }
            if let Some((tilt, period)) = rock {
                let tray = l_flow + 2.0 * TRAY_MARGIN_M;
                let kf = rocked(tilt, period, tray, TRAY_DEPTH_M, l_flow, nu, d);
                // (natural and forced convection combine as Churchill's cubes)
                k = (k.powi(3) + kf.powi(3)).cbrt();
            }
        }
    }
    let mut rate = v_metal * k * c / nu_ox;
    match bath.acid {
        Acid::Ferric => rate *= (-EA_FERRIC / R_GAS * (1.0 / tk - 1.0 / T20)).exp(),
        Acid::Dutch => {
            let kinetic = DUTCH_UM_MIN * 1e-6 / 60.0 * (-EA_DUTCH / R_GAS * (1.0 / tk - 1.0 / T20)).exp();
            rate = rate.min(kinetic);
        }
        Acid::Nitric => {}
    }
    let mut bubbles = 0.0;
    if gas > 0.0 && rate > 0.0 {
        let d_bubble = 0.0208 * BUBBLE_ANGLE_DEG * (SIGMA_W / (G * rho)).sqrt();
        let q0 = gas * rate / v_metal * R_GAS * tk / 101_325.0;
        // (the gas comes only where it bites: under the bubbles it doesn't; a few rounds settle it)
        for _ in 0..32 {
            let q = q0 * (1.0 - bubbles);
            let tau = 2.0 / 3.0 * d_bubble * BUBBLE_COVER / q.max(1e-30);
            bubbles = match bath.feather {
                Some(tf) if tf > 0.0 => BUBBLE_COVER * (1.0 - tau / tf * (1.0 - (-tf / tau).exp())),
                _ => BUBBLE_COVER,
            };
        }
        rate *= 1.0 - bubbles;
    }
    let delta = if k > 0.0 { d / k } else { f64::INFINITY };
    Transport { rate_um_min: rate * 1e6 * 60.0, delta_um: delta * 1e6, bubbles, d, ra }
}

// ---------------------------------------------------------------- a bitten opening

/// A slot's slowing as it deepens: an opening of width `w` µm under a
/// diffusion layer `delta` µm thick adds its depth to the layer by the share
/// δ/(δ + w): a narrow slot the layer can't enter adds all of it, a wide
/// dish none (the layer follows its floor) [D, an interpolation between the
/// two limits; IP §3.4].
pub fn slot_share(w: f64, delta: f64) -> f64 {
    if !delta.is_finite() { 1.0 } else { delta / (delta + w.max(0.0)) }
}

/// The depth (µm) a front bitten `z0` µm reaches after a dose of `dose` µm
/// (the flat face's bite in that time), through a mouth `t` µm deep (the
/// ground) and a diffusion layer `delta` µm, its slot share `g`: dz/dt =
/// r·δ/(δ + g·(t + z)), integrated exactly over the dose [D].
pub fn advance(z0: f64, dose: f64, delta: f64, g: f64, t: f64) -> f64 {
    if dose <= 0.0 {
        return z0;
    }
    if !delta.is_finite() || g <= 0.0 {
        return z0 + dose;
    }
    // (δ + g·t)·z + g·z²/2 = (δ + g·t)·z0 + g·z0²/2 + δ·dose
    let (a, b) = (0.5 * g, delta + g * t);
    let c = b * z0 + a * z0 * z0 + delta * dose;
    (-b + (b * b + 4.0 * a * c).sqrt()) / (2.0 * a)
}

/// The section of a line bitten `z` µm deep from a window `w0` µm wide in the
/// ground, with etch factor `k` (IP §3.4): a flat floor as wide as the
/// window and two quarter-ellipses of semi-axes z and z/K under its edges.
/// (area µm², width at the plate's face µm.)
pub fn line_section(w0: f64, z: f64, k: f64) -> (f64, f64) {
    (w0 * z + PI * z * z / (2.0 * k), w0 + 2.0 * z / k)
}

/// A fin of triangular section `area` µm² and `height` µm, bitten `z` µm from
/// both faces: an inward offset scales a triangle about its incentre by
/// 1 − z/r, r its inradius (area over half its perimeter) [D, geometry].
/// Returns the scale left (0: gone).
pub fn fin_left(area: f64, height: f64, z: f64) -> f64 {
    if area <= 0.0 || height <= 0.0 {
        return 0.0;
    }
    let base = 2.0 * area / height;
    let half_perimeter = 0.5 * (base + 2.0 * (height * height + 0.25 * base * base).sqrt());
    (1.0 - z / (area / half_perimeter)).max(0.0)
}

// ---------------------------------------------------------------- aquatint

/// Rosin's density, 1.07 g/cm³ [M, CAMEO; IP §4.1], and air's viscosity at
/// 20 °C, 1.81 × 10⁻⁵ Pa·s [M].
pub const ROSIN_KG_M3: f64 = 1070.0;
pub const AIR_MU: f64 = 1.81e-5;
/// The dust box's cloud: rosin ground to a log-normal size by mass, median
/// 30 µm, geometric spread 2 [E: no measured size distribution of an
/// aquatint box's rosin exists (IP §4.1, Unknown 7); a pounded resin], at
/// 82 g/m³ just after it is stirred [E: the loading that gives the workshops'
/// 50 % cover (IP §4.1 [P]) on the plan's example, a 0.6 m box, a 30 s wait
/// and a minute in it].
pub const DUST_MMD_UM: f64 = 30.0;
pub const DUST_SPREAD: f64 = 2.0;
pub const DUST_KG_M3: f64 = 0.082;
/// A melted grain spreads to 1.3 times its diameter [E: IP §4.1's 1.2–1.5;
/// no contact angle of molten rosin on copper was found].
pub const MELT_SPREAD: f64 = 1.3;
/// Grain sizes: log-spaced bins over 1–300 µm.
pub const DUST_BINS: usize = 32;
const DUST_MAX_UM: f64 = 300.0;

/// A size bin's bounds and its middle diameter (µm).
pub fn dust_bin(i: usize) -> (f64, f64, f64) {
    let e = |j: f64| DUST_MAX_UM.powf(j / DUST_BINS as f64);
    (e(i as f64), e(i as f64 + 1.0), e(i as f64 + 0.5))
}

/// A rosin grain's settling speed in still air (m/s): Stokes, v = ρgd²/(18µ)
/// [M; IP §4.1]. (Its Reynolds number stays under 1 to about 70 µm; the slip
/// correction is under 3 % above 5 µm.)
pub fn stokes(d_um: f64) -> f64 {
    ROSIN_KG_M3 * G * (d_um * 1e-6).powi(2) / (18.0 * AIR_MU)
}

/// The largest grain (µm) still in the air of a box `h` m tall `wait_s` s
/// after it was stirred: v·t = H.
pub fn largest_grain(h: f64, wait_s: f64) -> f64 {
    (18.0 * AIR_MU * h / (ROSIN_KG_M3 * G * wait_s.max(1e-9))).sqrt() * 1e6
}

fn erf(x: f64) -> f64 {
    // Abramowitz & Stegun 7.1.26 (error under 1.5 × 10⁻⁷)
    let s = x.signum();
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.327_591_1 * x);
    let y = 1.0 - (((((1.061_405_429 * t - 1.453_152_027) * t) + 1.421_413_741) * t - 0.284_496_736) * t + 0.254_829_592) * t * (-x * x).exp();
    s * y
}

/// The grains that land on a plate put into a box `h` m tall `wait_s` s after
/// the cloud was stirred and left for `minutes` (number per µm², per size
/// bin). A well-stirred cloud settles as a front, each size at its Stokes
/// speed, so a plate at the floor takes size d at n(d)·v while the cloud of
/// that size still reaches the floor: n(d)·[min(H, v(t + m)) − v·t]⁺ [D].
pub fn dust_box(h: f64, wait_s: f64, minutes: f64) -> [f64; DUST_BINS] {
    let phi = |d: f64| 0.5 * (1.0 + erf((d / DUST_MMD_UM).ln() / DUST_SPREAD.ln() / std::f64::consts::SQRT_2));
    std::array::from_fn(|i| {
        let (lo, hi, d) = dust_bin(i);
        let mass = ROSIN_KG_M3 * PI * (d * 1e-6).powi(3) / 6.0;
        let n = DUST_KG_M3 * (phi(hi) - phi(lo)) / mass;
        let v = stokes(d);
        let column = ((v * (wait_s + 60.0 * minutes)).min(h) - v * wait_s).max(0.0);
        n * column * 1e-12
    })
}

/// The share of the plate grains of these counts (per µm² per bin) cover,
/// each a disc of `spread` times its diameter, laid at random: 1 −
/// e^(−Σ nπd²/4) (a Boolean model).
pub fn cover(per_um2: &[f64], spread: f64) -> f64 {
    let a: f64 = per_um2.iter().enumerate().map(|(i, n)| n * PI * (spread * dust_bin(i).2).powi(2) / 4.0).sum();
    1.0 - (-a).exp()
}

/// The bared face between melted dots, bitten `z` µm with etch factor `k`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Network {
    /// How far the face's highest metal is below the plate (µm): 0 while
    /// dots remain on their pillars; once all are lost, the tips of the
    /// pillars left.
    pub level: f64,
    /// The pits: their share of the face at its top, their mean width there
    /// (µm), and their volume (µm over the face).
    pub share: f64,
    pub width: f64,
    pub vol: f64,
}

/// An open bite: an opening wider than anything the cloth spans (µm).
pub const OPEN_BITE_UM: f64 = 4096.0;

/// The pits between dots (`dots`: number per µm² and radius µm), bitten `z`
/// µm with etch factor `k` (IP §4.2; EP §4).
///
/// The acid bites down z between the dots and sideways under them: a point
/// y below the face, inside a dot's edge by u, is reached once (uK)² + y² ≤
/// z², so the pillar under a dot of radius R is R − √(z² − y²)/K wide at
/// depth y and tops out at √(z² − K²R²) once z > KR (the dot is lost). The
/// metal left at depth y covers c(y) = 1 − e^(−πΣn·max(0, R − √(z² − y²)/K)²)
/// (the shrunken dots as a Boolean model), and the pits hold ∫(1 − c) dy.
/// Their width is the mean chord between the dots at the face, 1/(2Σn·R)
/// (a Boolean model's free path, π(1 − c)/S_A).
pub fn network(dots: &[(f64, f64)], z: f64, k: f64) -> Network {
    let dots: Vec<(f64, f64)> = dots.iter().copied().filter(|d| d.0 > 0.0 && d.1 > 0.0).collect();
    if z <= 0.0 {
        return Network::default();
    }
    if dots.is_empty() {
        return Network { level: z, ..Network::default() };
    }
    let radius = |r: f64, y: f64| (r - (z * z - y * y).max(0.0).sqrt() / k).max(0.0);
    let covered = |y: f64| 1.0 - (-PI * dots.iter().map(|&(n, r)| n * radius(r, y).powi(2)).sum::<f64>()).exp();
    let top = dots.iter().map(|&(_, r)| if z > k * r { (z * z - k * k * r * r).sqrt() } else { 0.0 }).fold(f64::INFINITY, f64::min);
    // ∫ (1 − c) dy over [top, z] (Simpson, 64 steps)
    let n = 64;
    let h = (z - top) / n as f64;
    let mut s = (1.0 - covered(top)) + (1.0 - covered(z));
    for i in 1..n {
        s += if i % 2 == 1 { 4.0 } else { 2.0 } * (1.0 - covered(top + i as f64 * h));
    }
    let vol = (s * h / 3.0).max(0.0);
    let at = if top > 0.0 { 0.5 * (top + z) } else { 0.0 };
    let edge: f64 = dots.iter().map(|&(n, r)| n * radius(r, at)).sum();
    let width = if edge > 0.0 { (1.0 / (2.0 * edge)).min(OPEN_BITE_UM) } else { OPEN_BITE_UM };
    Network { level: top, share: 1.0 - covered(if top > 0.0 { top } else { 0.0 }), width, vol }
}

/// A tarlatan yarn's radius, 75 µm [E: IP §4.2, cheesecloth yarn].
pub const YARN_UM: f64 = 75.0;

/// The ink a wipe leaves in openings of share `share`, width `w` µm and mean
/// depth `d` µm (µm over the face; IP §4.2): what lies below the yarn's dip
/// r − √(r² − w²/4). An opening wider than a yarn's diameter lets the yarn
/// in by its radius at most (the cloth's sag is the wipe's, phase P3); an
/// open bite is wiped clean.
pub fn held_um(share: f64, w: f64, d: f64) -> f64 {
    let r = YARN_UM;
    let dip = if w >= OPEN_BITE_UM {
        d
    } else if w < 2.0 * r {
        r - (r * r - 0.25 * w * w).sqrt()
    } else {
        r
    };
    share * (d - dip).max(0.0)
}

// ---------------------------------------------------------------- the look of a film

/// Hard ground's bitumen: refractive index 1.6 [M, asphalt binders by
/// ellipsometry, about 1.6 in the visible], its absorption 0.1 /µm at 550 nm
/// [E: the workshop's "the colour of strong tea" (IP §2.1), a double pass
/// through 4 µm leaving 0.45], rising to the blue as an Urbach tail of 0.3
/// eV [E: asphaltenes' absorption is an Urbach edge (Mullins); its slope for
/// an etching ground is unmeasured].
pub const GROUND_N: f64 = 1.6;
pub const GROUND_ALPHA550: f64 = 0.1;
pub const GROUND_URBACH_EV: f64 = 0.3;
/// Hard ground laid 4 µm thick by default [E: IP §2.1's 2–8 µm].
pub const GROUND_UM: f64 = 4.0;
/// Stopping-out varnish (asphaltum in turpentine, brushed): 20 µm [E: a
/// brushed varnish, unmeasured], the ground's bitumen. It is the film a
/// loaded brush lays (`stop_out_spread`, `Plate`'s stopping-out).
pub const STOP_UM: f64 = 20.0;

// ---------------------------------------------------------------- the hand's tools on a ground

/// A hard ground's leather roller: 150 mm long [P: "the ideal roller is
/// about 150 mm long and 50 mm in diameter, covered in leather" (Etching: a
/// guide to traditional techniques, ch. 11); makers sell 100–200 mm (Artools
/// 116-10, 116-20)]. Its line of contact lays the ground where it rolls.
pub const ROLLER_MM: f64 = 150.0;
/// Rolling a ground: the roller pushed at 100 mm/s [E: a brisk roll; no
/// timed account], and the plate warmed on the hot plate and let cool, 8
/// minutes [E: EP's 10 minutes for the whole, less the rolling].
pub const ROLL_MM_S: f64 = 100.0;
pub const GROUND_WARM_S: f64 = 480.0;
/// Stopping-out varnish (asphaltum and rosin in turpentine) as a liquid:
/// its surface tension that of turpentine, 27 mN/m [M: handbook value for
/// turpentine and α-pinene, 26–27 mN/m near 20 °C]; its viscosity 1 Pa·s
/// [E: makers call it of "thicker consistency" than straw-hat varnish, so
/// over a brushing paint's 0.1–0.3 Pa·s; unmeasured].
pub const STOP_TENSION: f64 = 0.027;
pub const STOP_PA_S: f64 = 1.0;
/// It flows for 10 minutes before it sets [P: about 10 minutes to dry under
/// a fan (Glasgow Print Studio's handbook), an hour in still air
/// (MungoLux); a film stops flowing well before it is dry to the touch, so
/// the shorter].
pub const STOP_SET_S: f64 = 600.0;
/// Cox–Voinov's logarithm ln(L/λ), the ratio of the drop's size to the
/// microscopic cut-off at its contact line: 9.2 [M: Bonn et al., Rev. Mod.
/// Phys. 81, 739 (2009), ln 10⁴ fitted across fluids in complete wetting].
pub const COX_LOG: f64 = 9.2;
/// Brushing varnish, 20 mm/s [E: EP's stop-out pace, a brush 8 mm wide at
/// 20 mm/s].
pub const BRUSH_MM_S: f64 = 20.0;
/// A sable round's belly (Rosemary & Co Series 33: 15.7/3.4, 20/3.9 and
/// 24.2/5.2 mm hair length over width for sizes 4, 6, 8 [M]): a cone of its
/// width `d_mm` and 4.7 times that long, the liquid filling 0.6 of it [E: the
/// hairs' packing in a wet bundle] (the wet brush's own, `bristle`'s
/// `capacity_mm3` where wet pastel has it). A size 6, 3.9 mm, holds 44 mm³.
pub fn round_belly_mm3(d_mm: f64) -> f64 {
    0.6 * std::f64::consts::PI / 12.0 * d_mm * d_mm * (4.7 * d_mm)
}
/// The stop-out brush when none is given: a size 6 sable round, 3.9 mm.
pub const STOP_BRUSH_MM: f64 = 3.9;

/// The width (mm) a round brush `d_mm` across lays pressed at `pressure`
/// (0..1): a wet tuft is a cone, so its mark widens with the pressure until
/// its whole belly is down (`bristle::P_FULL`), the bristle model's law.
pub fn brush_width_mm(d_mm: f64, pressure: f64) -> f64 {
    d_mm * (pressure / crate::bristle::P_FULL as f64).clamp(0.0, 1.0)
}

/// How far (mm) the edge of a strip of stopping-out varnish `w_mm` wide and
/// `h_um` thick creeps out on a clean plate before it sets: the strip is a
/// ridge of section A = h·w, its contact angle θ = 3A/(2a²) for its half-width
/// a (a parabolic section), and its edge advances by Cox–Voinov in complete
/// wetting, θ³ = 9·ln(L/λ)·ηU/γ, so a⁷ = a₀⁷ + 7γ(3A/2)³t/(9η·ln(L/λ)) over
/// `STOP_SET_S` [D: Tanner's law for a two-dimensional ridge]. A 20 µm strip
/// 2.3 mm wide creeps 26 µm: the edge is the brush's.
pub fn stop_out_spread(w_mm: f64, h_um: f64) -> f64 {
    let a0 = 0.5 * w_mm * 1e-3;
    let area = h_um * 1e-6 * w_mm * 1e-3;
    if a0 <= 0.0 || area <= 0.0 {
        return 0.0;
    }
    let a7 = a0.powi(7) + 7.0 * STOP_TENSION * (1.5 * area).powi(3) * STOP_SET_S / (9.0 * STOP_PA_S * COX_LOG);
    (a7.powf(1.0 / 7.0) - a0) * 1e3
}
/// Rosin: index 1.54 [M, CAMEO], pale amber: absorption 0.02 /µm at 450 nm
/// [E: a rosin of pale grade, unmeasured], with an Urbach tail of 0.3 eV [E].
pub const ROSIN_N: f64 = 1.54;
pub const ROSIN_ALPHA450: f64 = 0.02;

/// A film's absorption (1/µm) at `lambda_nm`: α₀ at λ₀ and an Urbach tail.
pub fn urbach(alpha0: f64, lambda0_nm: f64, ev: f64, lambda_nm: f64) -> f64 {
    let e = |l: f64| 1239.84 / l;
    alpha0 * ((e(lambda_nm) - e(lambda0_nm)) / ev).exp()
}

/// A metal seen at normal incidence through a film of index `n_film`,
/// absorption `alpha(λ)` (1/µm) and `t_um` thick: the film's face reflects
/// r₁ = ((n − 1)/(n + 1))², the light that enters crosses the film, meets
/// the metal (its Fresnel reflection against the film, N/n_film), and comes
/// back, the internal reflections summed:
/// R = r₁ + (1 − r₁)²·R_m·T²/(1 − r₁·R_m·T²), T = e^(−αt) [D, a thin film's
/// incoherent sum].
pub fn film_over_metal(metal: Metal, n_film: f64, alpha: impl Fn(f64) -> f64, t_um: f64) -> Spectrum {
    let r1 = ((n_film - 1.0) / (n_film + 1.0)).powi(2);
    std::array::from_fn(|i| {
        let l = spectral::wavelength(i) as f64;
        let (n, k) = nk(metal, l / 1000.0);
        let rm = fresnel(n / n_film, k / n_film, 0.0);
        let t2 = (-2.0 * alpha(l) * t_um).exp();
        (r1 + (1.0 - r1).powi(2) * rm * t2 / (1.0 - r1 * rm * t2)) as f32
    })
}

/// The ground's bitumen absorption (1/µm) at `lambda_nm`.
pub fn ground_alpha(lambda_nm: f64) -> f64 {
    urbach(GROUND_ALPHA550, 550.0, GROUND_URBACH_EV, lambda_nm)
}

/// Rosin's absorption (1/µm) at `lambda_nm`.
pub fn rosin_alpha(lambda_nm: f64) -> f64 {
    urbach(ROSIN_ALPHA450, 450.0, GROUND_URBACH_EV, lambda_nm)
}

#[cfg(test)]
mod tests;
