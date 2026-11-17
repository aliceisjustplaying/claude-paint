//! Steam on pastel, and crushed pastel sifted onto the sheet (the easel's
//! `--@ steam-sift`; notes/cassatt_support.md gap 6). Source:
//! notes/research/steam_sift.md (its sections cited as §n; [M] measured,
//! [D] derived here from measured values or standard physics, [P] a
//! practitioner's or museum's account, [E] an estimate, each with the
//! reason it is one).
//!
//! - **Steam** (`Canvas::steam`, §1–§3). A kettle's jet of steam mixes with
//!   the room's air as a turbulent round jet does (Chen & Rodi's decay of
//!   its centreline velocity and mass fraction); at the sheet it is a warm
//!   fog (`plume`). The hand moves the kettle along points (a pass): a point
//!   of the sheet is steamed for as long as the jet's steam share is on it,
//!   by the jet's own Gaussian profile, out to where the mixture's dew point
//!   falls to the cool sheet's and nothing condenses (`Plume::reach_m`). It condenses on the cooler sheet at the mass transfer
//!   of its stagnation flow (Sibulkin's laminar stagnation-point law, by the
//!   Chilton–Colburn analogy), and the latent heat warms the face, which
//!   conducts it into the sheet and the board behind (`condense`): the face
//!   soon nears the fog's dew point, and the water condensed then grows as
//!   the heat the board takes, about √t. The water is the wet pastel
//!   switch's (`crate::wet_pastel`): the pastel's pores take it first,
//!   what is left stands and sinks into the sheet at its sizing's rate, it
//!   dissolves the stick's gum, and as it dries the gum sets and binds the
//!   pastel where enough of it was kept (`wet_pastel::binds`). Steam carries
//!   no pigment: nothing moves sideways unless the wet pastel is then
//!   worked (a wet brush, `wash_pastel`).
//! - **Sifting** (`Canvas::sift`, §4–§6). Crushed pastel rubbed through a
//!   sieve held over points falls as crumbs, with no stroke, under the
//!   sieve's mesh. Its sizes follow crushing
//!   (Gates–Gaudin–Schuhmann, `GGS_M`) up to the sieve's aperture. On a dry
//!   sheet the crumbs lie on the tooth, loose, held by the size-independent
//!   adhesion of a dry grain (sheet.rs `ADHESION_N`): a tap sheds every
//!   crumb whose weight beats it. On a damp one the crumbs draw water into
//!   their pores (they pull harder than the sheet's); once they are full,
//!   the water left standing pulls them together into clusters as it dries
//!   (`cluster_um`), and a liquid bridge holds each to the sheet as
//!   charcoal's water bridge does, at the crumb's own radius (`wet_fall_um`).
//!   Dried, the gum they dissolved binds them where it was kept.
//!
//! The crumbs lying on the tooth (not pressed into it) are recorded beside
//! the drawing (`Drawing::perched`), lazily: empty in every painting that
//! never sifted, so tap, blow and drying run as before there.

use crate::canvas::Canvas;
use crate::climate::STUDIO;
use crate::graphite::Cell;
use crate::paper::Paper;
use crate::pastel::{Grains, PARTICLE_UM, Stick};
use crate::wet_pastel::POROSITY;

/// The heat a kettle puts into its water, W: an electric kettle draws
/// 1.8–3 kW at 80–90 % efficiency, a kettle on a gas ring about 2 kW at
/// 40–60 % [E: 1 kW, the lower middle of 0.5–2.5 kW; a studio kettle of the
/// 1880s sat on a stove or a spirit lamp, whose output isn't recorded].
pub const KETTLE_W: f64 = 1000.0;
/// The kettle's spout, mm across [E: a domestic kettle's 8–12 mm].
pub const SPOUT_MM: f64 = 10.0;
/// Water's latent heat at 100 °C, J/kg, and steam's density there, kg/m³
/// (steam tables) [M].
const H_FG_100: f64 = 2.257e6;
const RHO_STEAM_100: f64 = 0.598;
/// A turbulent round jet's centreline decay beyond its potential core:
/// U/U₀ = 6.3 (d/x) √(ρ₀/ρₐ), C/C₀ = 5.4 (d/x) √(ρ₀/ρₐ) (Chen & Rodi 1980,
/// as quoted in the variable-density jet literature) [M].
const JET_KU: f64 = 6.3;
const JET_KC: f64 = 5.4;
/// Its velocity half-width over the distance, 0.094–0.102 (Hussein, Capp &
/// George 1994; Pope 2000, Table 5.1) [M]: 0.094.
const JET_SPREAD: f64 = 0.094;
/// Air at 20 °C: density kg/m³, heat capacity J/kgK, conductivity W/mK,
/// kinematic viscosity m²/s, Prandtl number [M, handbook].
const AIR_RHO: f64 = 1.204;
const AIR_CP: f64 = 1006.0;
const AIR_K: f64 = 0.0257;
const AIR_NU: f64 = 1.51e-5;
const AIR_PR: f64 = 0.71;
/// Water vapour's and liquid water's heat capacities, J/kgK [M].
const CP_VAPOUR: f64 = 1860.0;
const CP_WATER: f64 = 4186.0;
/// Molar masses, kg/mol, and the room's pressure, Pa.
const M_W: f64 = 0.018_015;
const M_A: f64 = 0.028_965;
const P_ATM: f64 = 101_325.0;
/// Sibulkin's laminar heat transfer at an axisymmetric stagnation point:
/// Nu = 0.763 (β d²/ν)^½ Pr^0.4 (Sibulkin 1952), so h = 0.763 k Pr^0.4
/// √(β/ν) [M]. Measured jets give 1.2–1.8 times it; the laminar value is
/// the lower bound, and it matters little: the condensation is limited by
/// the heat the board takes once the face is warm (§2).
const SIBULKIN: f64 = 0.763;
/// Paper through its thickness: conductivity, W/mK (printing papers 0.08,
/// fibreboard 0.12 through the thickness; copy paper 0.09) and heat
/// capacity, J/kgK (paper solids 1.34 kJ/kgK) [M, §2].
const PAPER_K: f64 = 0.08;
const PAPER_CP: f64 = 1340.0;
/// The board a pastel sheet is pinned to (wet_pastel.md §4: "a sheet pinned
/// to a board"): paperboard, 0.072 W/mK at 480 kg/m³ (IES VE materials
/// table) [M], with the paper solids' heat capacity; 20 mm deep, past what
/// the heat reaches in ten minutes (√(αt) ≈ 6 mm).
const BOARD_K: f64 = 0.072;
const BOARD_RHO: f64 = 480.0;
const BOARD_MM: f64 = 20.0;
/// The time step of the face's heat balance, s.
const STEAM_DT: f64 = 0.01;
/// A round jet's scalar (its steam share's) half-width over the distance:
/// the mean scalar profile is Gaussian, C/C_c = exp(−ln 2 (r/b)²), and
/// spreads a little faster than the velocity: 0.102–0.11 (Mi, Nobes &
/// Nathan 2001, as quoted by the high-pressure jet DNS, arXiv 2009.12926),
/// 0.138 for a light helium jet (Panchapakesan & Lumley 1993b) [M]: 0.11.
const JET_SCALAR_SPREAD: f64 = 0.11;

/// Crushed pastel's sizes: the mass finer than d is (d/D)^m up to the
/// sieve's aperture D (Gates–Gaudin–Schuhmann). m falls with a material's
/// hardness, 0.9 for quartz to 0.3 for gypsum (Abouzeid 1994, via Clark
/// 2005) [M]; a gum-bound chalk compact (σt ≈ 0.2 MPa) is softer than either
/// [E: 0.3, the softest value reported; no crushed pastel was measured].
pub const GGS_M: f64 = 0.3;
/// The smallest crumb that lies on the tooth, µm: finer grains go into its
/// pores with the deposit (`pastel::CRUMB_MIN_UM`).
pub const CRUMB_MIN_UM: f64 = crate::pastel::CRUMB_MIN_UM as f64;
/// The primary particles a crumb is made of, µm (whiting's d₃₂,
/// `wet_pastel::PARTICLE_D_UM`): the finest a crushing gives.
const PRIMARY_UM: f64 = crate::wet_pastel::PARTICLE_D_UM as f64;
/// A dry crumb's density, kg/m³ (pigment and chalk about 2500, the crumb
/// about half pores: sheet.rs's `CRUMB_KG_M3`), and wet, its pores full.
const CRUMB_KG_M3: f64 = 1250.0;
pub(crate) const WET_CRUMB_KG_M3: f64 = CRUMB_KG_M3 + 0.5 * 998.2;
/// The pores' water, its gum dissolved: surface tension N/m (the wet pastel
/// switch's, gum arabic's 47–57 mN/m [M]).
pub(crate) const GAMMA: f64 = 0.050;
/// Dropwise condensation's self-similar coverage, the share of a surface
/// its drops cover once they coalesce: about 0.5 (simulations 0.5 on every
/// substrate, Int. J. Heat Mass Transfer 2011; Beysens et al.'s θ = 90°
/// breath figures 0.55) [M].
const DROP_COVER: f64 = 0.5;
/// Water's contact angle on a sized sheet, 105° (IPPTA 2007, Cobb 25–48
/// papers; wet_pastel.md §4) [M]; an unsized one is wetted (0°), where a
/// film rather than drops stands: drops at 30° stand in for it [E].
const SIZED_ANGLE_DEG: f64 = 105.0;
const WETTED_ANGLE_DEG: f64 = 30.0;
/// The crumb sizes the sift draws, log-spaced bins over [5 µm, aperture].
const SIZE_BINS: usize = 12;

/// The record of crumbs lying on the tooth, per cell (`Drawing::perched`):
/// their volume as crushed (µm over the pixel, a stick's packing), the same
/// as it lies in the cell's `loose` and `bound` (`Grains::lies`), the
/// largest crumb there (µm), the size of the clusters among them (0: none),
/// and the share held by gum or fixative.
pub(crate) const PERCH: usize = 5;
const P_V: usize = 0;
const P_L: usize = 1;
const P_TOP: usize = 2;
const P_UNIT: usize = 3;
const P_HELD: usize = 4;

/// The error function (Abramowitz & Stegun 7.1.26, within 1.5e-7).
fn erf(x: f64) -> f64 {
    let t = 1.0 / (1.0 + 0.327_591_1 * x.abs());
    let y = 1.0 - t * (0.254_829_592 + t * (-0.284_496_736 + t * (1.421_413_741 + t * (-1.453_152_027 + t * 1.061_405_429)))) * (-x * x).exp();
    if x < 0.0 { -y } else { y }
}

/// Water's saturated vapour pressure, Pa, at `t` °C (Buck 1981, as
/// `wet_pastel`).
fn p_sat(t: f64) -> f64 {
    611.21 * ((18.678 - t / 234.5) * (t / (257.14 + t))).exp()
}

/// Saturated vapour density, kg/m³, at `t` °C.
fn rho_sat(t: f64) -> f64 {
    p_sat(t) * M_W / (8.314_462 * (t + 273.15))
}

/// Water's latent heat of condensation at `t` °C, J/kg [M, a linear fit].
fn latent(t: f64) -> f64 {
    2.501e6 - 2370.0 * t
}

/// The vapour's mass fraction in air saturated at `t` °C.
fn y_sat(t: f64) -> f64 {
    let x = (p_sat(t) / P_ATM).min(1.0);
    x * M_W / (x * M_W + (1.0 - x) * M_A)
}

/// The kettle's jet where it meets the sheet `distance_mm` from the spout.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plume {
    /// The steam's mass share of the jet there (Chen & Rodi).
    pub steam: f64,
    /// Its temperature, °C, and dew point, °C.
    pub temp_c: f64,
    pub dew_c: f64,
    /// Its vapour density, kg/m³ (fog droplets carry the rest of its water).
    pub rho_v: f64,
    /// Centreline velocity, m/s, and half-width, m.
    pub u: f64,
    pub half_m: f64,
    /// Heat and mass transfer at the stagnation point, W/m²K and m/s.
    pub h_c: f64,
    pub h_m: f64,
    /// The steam share's half-width, m (`JET_SCALAR_SPREAD`), and how far
    /// from the axis any vapour condenses on the sheet at the room's
    /// temperature, m: where the mixture's dew point falls to it.
    pub scalar_half_m: f64,
    pub reach_m: f64,
}

impl Plume {
    /// The steam share off the axis by `r_m`, over the axis's: the jet's
    /// Gaussian scalar profile, 0 past `reach_m`.
    pub fn share_at(&self, r_m: f64) -> f64 {
        if r_m >= self.reach_m { 0.0 } else { (-std::f64::consts::LN_2 * (r_m / self.scalar_half_m).powi(2)).exp() }
    }
}

/// The jet at `distance_mm` (§1): a boiling kettle gives `KETTLE_W` / h_fg
/// of steam through its spout; its centreline velocity and steam share
/// decay as a round jet's; mixed with the studio's air its enthalpy sets
/// its temperature, saturated (a fog) where it holds more water than its
/// air can; Sibulkin's stagnation flow at a velocity gradient of its
/// centreline velocity over its half-width [E: the order of an impinging
/// jet's strain] gives the heat transfer, and Chilton–Colburn the mass.
pub fn plume(distance_mm: f64) -> Plume {
    let x = distance_mm.max(10.0 * SPOUT_MM) * 1e-3;
    let d0 = SPOUT_MM * 1e-3;
    let mdot = KETTLE_W / H_FG_100;
    let u0 = mdot / (RHO_STEAM_100 * std::f64::consts::FRAC_PI_4 * d0 * d0);
    let root = (RHO_STEAM_100 / AIR_RHO).sqrt();
    let s = (JET_KC * d0 / x * root).min(1.0);
    let u = JET_KU * u0 * d0 / x * root;
    let half = JET_SPREAD * x;
    // the room's air: its vapour share, and the mixture's enthalpy per kg
    // (dry air and liquid water at 0 °C as zero)
    let ta = STUDIO.air_c;
    let ya = STUDIO.rh * y_sat(ta);
    let hv = |t: f64| 2.501e6 + CP_VAPOUR * t;
    let h = s * hv(100.0) + (1.0 - s) * ((1.0 - ya) * AIR_CP * ta + ya * hv(ta));
    let yw = s + (1.0 - s) * ya;
    let enthalpy = |t: f64| {
        let yv = yw.min(y_sat(t));
        (1.0 - yw) * AIR_CP * t + yv * hv(t) + (yw - yv) * CP_WATER * t
    };
    let (mut lo, mut hi) = (ta, 100.0);
    for _ in 0..80 {
        let m = 0.5 * (lo + hi);
        if enthalpy(m) < h { lo = m } else { hi = m }
    }
    let t = 0.5 * (lo + hi);
    let yv = yw.min(y_sat(t));
    // the vapour's partial pressure, from its mole fraction
    let xv = (yv / M_W) / (yv / M_W + (1.0 - yw) / M_A);
    let pv = xv * P_ATM;
    let rho_v = pv * M_W / (8.314_462 * (t + 273.15));
    let (mut lo, mut hi) = (-20.0, 100.0);
    for _ in 0..80 {
        let m = 0.5 * (lo + hi);
        if p_sat(m) < pv { lo = m } else { hi = m }
    }
    let dew = 0.5 * (lo + hi);
    // where the room's air and the jet's steam hold no more vapour than air
    // saturated at the room's temperature, the sheet takes none: s ya + ... ≤ y_sat
    let scalar_half = JET_SCALAR_SPREAD * x;
    let s_dew = ((y_sat(ta) - ya) / (1.0 - ya)).max(0.0);
    let reach = if s > s_dew { scalar_half * ((s / s_dew).ln() / std::f64::consts::LN_2).sqrt() } else { 0.0 };
    let beta = u / half;
    let h_c = SIBULKIN * AIR_K * AIR_PR.powf(0.4) * (beta / AIR_NU).sqrt();
    // Chilton–Colburn: h/(ρ c_p) Le^(−2/3), vapour in air at 20 °C (Massman)
    let d_v = 2.178e-5 * ((ta + 273.15) / 273.15f64).powf(1.81);
    let le = AIR_K / (AIR_RHO * AIR_CP) / d_v;
    let h_m = h_c / (AIR_RHO * AIR_CP) * le.powf(-2.0 / 3.0);
    Plume { steam: s, temp_c: t, dew_c: dew, rho_v, u, half_m: half, h_c, h_m, scalar_half_m: scalar_half, reach_m: reach }
}

/// What steam condensed on a sheet over time (`condense`): at each step of
/// `STEAM_DT` seconds, the water (µm, g/m²) and the face's temperature (°C).
#[derive(Clone, Debug)]
pub struct Condensed {
    pub water_um: Vec<f64>,
    pub face_c: Vec<f64>,
}

impl Condensed {
    /// The water condensed after `secs` of steam (linear between steps).
    pub fn at(&self, secs: f64) -> f64 {
        let s = (secs / STEAM_DT).max(0.0);
        let i = s.floor() as usize;
        if i + 1 >= self.water_um.len() {
            return *self.water_um.last().unwrap_or(&0.0);
        }
        let f = s - i as f64;
        self.water_um[i] + f * (self.water_um[i + 1] - self.water_um[i])
    }
}

/// `seconds` of the jet `distance_mm` away on a sheet of paper `p` pinned to
/// a board (§2): the face takes the fog's sensible heat and the latent heat
/// of the vapour that condenses on it, h_c (T_p − T_s) + L h_m (ρ_v − ρ_sat(T_s)),
/// and conducts it into the sheet and the board (backward Euler on a grid
/// fine at the face). Both start at the room's temperature.
pub fn condense(p: &Paper, distance_mm: f64, seconds: f64) -> Condensed {
    let pl = plume(distance_mm);
    let cal = (p.caliper_um() as f64 * 1e-6).max(20e-6);
    let rho_sheet = p.grammage as f64 * 1e-3 / cal;
    // control volumes: 8 across the sheet, then the board's growing by 15 %
    let mut dz = vec![cal / 8.0; 8];
    let mut kk = vec![PAPER_K; 8];
    let mut rc = vec![rho_sheet * PAPER_CP; 8];
    let (mut depth, mut step) = (0.0, cal / 8.0);
    while depth < BOARD_MM * 1e-3 {
        dz.push(step);
        kk.push(BOARD_K);
        rc.push(BOARD_RHO * PAPER_CP);
        depth += step;
        step *= 1.15;
    }
    let n = dz.len();
    let g: Vec<f64> = (0..n - 1).map(|j| 1.0 / (dz[j] / (2.0 * kk[j]) + dz[j + 1] / (2.0 * kk[j + 1]))).collect();
    let mut t = vec![STUDIO.air_c; n];
    let flux = |ts: f64| pl.h_c * (pl.temp_c - ts) + latent(ts) * (pl.h_m * (pl.rho_v - rho_sat(ts))).max(0.0);
    let cond = |ts: f64| (pl.h_m * (pl.rho_v - rho_sat(ts))).max(0.0);
    let steps = (seconds.max(0.0) / STEAM_DT).ceil() as usize;
    let mut out = Condensed { water_um: Vec::with_capacity(steps + 1), face_c: Vec::with_capacity(steps + 1) };
    out.water_um.push(0.0);
    out.face_c.push(t[0]);
    let mut w = 0.0;
    let (mut a, mut b, mut c, mut d) = (vec![0.0; n], vec![0.0; n], vec![0.0; n], vec![0.0; n]);
    for _ in 0..steps {
        let t0 = t[0];
        // the face's flux, linearised about its temperature now
        let q0 = flux(t0);
        let dq = (flux(t0 + 1e-3) - q0) / 1e-3;
        for j in 0..n {
            let m = rc[j] * dz[j] / STEAM_DT;
            a[j] = if j > 0 { -g[j - 1] } else { 0.0 };
            c[j] = if j + 1 < n { -g[j] } else { 0.0 };
            b[j] = m - a[j] - c[j];
            d[j] = m * t[j];
        }
        b[0] -= dq;
        d[0] += q0 - dq * t0;
        // Thomas
        for j in 1..n {
            let k = a[j] / b[j - 1];
            b[j] -= k * c[j - 1];
            d[j] -= k * d[j - 1];
        }
        t[n - 1] = d[n - 1] / b[n - 1];
        for j in (0..n - 1).rev() {
            t[j] = (d[j] - c[j] * t[j + 1]) / b[j];
        }
        // the water condensed over the step, at the face's mean temperature
        // (kg/m² is a mm of water: 1000 µm)
        w += 0.5 * (cond(t0) + cond(t[0])) * STEAM_DT * 1e3;
        out.water_um.push(w);
        out.face_c.push(t[0]);
    }
    out
}

/// The share of crushed pastel's mass (sieved at `top_um`) finer than `d`.
pub fn ggs_finer(d: f64, top_um: f64) -> f64 {
    if top_um <= 0.0 { 1.0 } else { (d.max(0.0) / top_um).min(1.0).powf(GGS_M) }
}

/// The share of the crumbs lying on the tooth (`CRUMB_MIN_UM` to `top_um`)
/// whose size passes `d`, by mass.
pub fn ggs_share_above(d: f64, top_um: f64) -> f64 {
    if d >= top_um || top_um <= CRUMB_MIN_UM {
        return 0.0;
    }
    let (lo, hi) = (CRUMB_MIN_UM.powf(GGS_M), top_um.powf(GGS_M));
    ((hi - d.max(CRUMB_MIN_UM).powf(GGS_M)) / (hi - lo)).clamp(0.0, 1.0)
}

/// The crumbs' mass-mean size, µm, between `CRUMB_MIN_UM` and `top_um`:
/// m/(m + 1) (D^(m+1) − d₀^(m+1)) / (D^m − d₀^m).
pub fn mean_crumb_um(top_um: f64) -> f64 {
    mean_size_um(CRUMB_MIN_UM, top_um)
}

/// The largest dry crumb a tap at `g` gravities leaves, µm: its weight
/// m·a passes the size-independent adhesion of a dry grain (sheet.rs's
/// `tap_pastel`, the same law).
pub fn dry_fall_um(g: f64) -> f64 {
    crate::sheet::dry_fall_um(g as f32) as f64
}

/// The largest wet crumb or cluster a tap at `g` gravities leaves, µm (§5):
/// a liquid bridge to a wetted sheet holds a sphere of radius R with 4πγR
/// (cos θ ≈ 1 on both wetted faces; Israelachvili; Butt & Kappl 2009), the
/// water bridge charcoal's adhesion counts at an asperity
/// (`charcoal::VINE_LIFT`) grown to the crumb's own radius once a film
/// stands thicker than the tooth's roughness; against the wet crumb's
/// weight: d² = 12γ/(ρ g a).
pub fn wet_fall_um(g: f64) -> f64 {
    (12.0 * GAMMA / (WET_CRUMB_KG_M3 * 9.81 * g.max(1e-9))).sqrt() * 1e6
}

/// How the water at a cell holds the crumbs lying there (§5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Wet {
    /// None: a dry grain's adhesion (sheet.rs `ADHESION_N`).
    Dry,
    /// Only in the pastel's pores: bridges at the touching grains, beside
    /// the dry hold (`pore_bridge_n`, `pore_fall_um`).
    Pores,
    /// Standing, or in the sheet beneath: a bridge at the crumb's own
    /// radius (`wet_fall_um`).
    Film,
}

/// The pastel's pores' radius, µm: twice their hydraulic radius,
/// 2εd/6(1 − ε) (wet_pastel's Washburn radius, #322 §3): 0.69 µm.
pub fn pore_radius_um() -> f64 {
    2.0 * POROSITY as f64 * crate::wet_pastel::PARTICLE_D_UM as f64 / (6.0 * (1.0 - POROSITY as f64))
}

/// How far a bridge fed by the pores' water reaches across a gap, µm (§5):
/// water drawn out of full pores leaves them at their entry suction,
/// 2γ/r_p, so a bridge they feed stops growing when its meniscus pulls as
/// hard, γ/r_k = 2γ/r_p: it spans gaps to 2r_k = r_p (cos θ ≈ 1).
pub fn pore_bridge_reach_um() -> f64 {
    pore_radius_um()
}

/// How far a crumb's grains stand proud of it, µm: a grain's radius
/// (whiting's d₃₂, `wet_pastel::PARTICLE_D_UM`; its d₅₀ of 2.4–2.7 µm would
/// give 1.2–1.35). A bridge that spans less (`pore_bridge_reach_um`) holds
/// at the one grain it touches; one that spans more fills the contact
/// to the crumb's own curvature (§5).
pub const GRAIN_RELIEF_UM: f64 = crate::wet_pastel::PARTICLE_D_UM as f64 / 2.0;

/// A bridge at the one grain a crumb touches the sheet with, N (§5):
/// 4πγa, a the grain's radius (`GRAIN_RELIEF_UM`; cos θ ≈ 1), the
/// single-asperity bridge charcoal's adhesion counts (`charcoal::VINE_LIFT`)
/// and the rough limit measured by van Zwol, Palasantzas & De Hosson (2007):
/// 0.53 µN, 59 times less than the 100 µm crumb's 4πγR [E: one grain
/// touching; a crumb resting on two or three makes it 1.1–1.6 µN].
pub fn pore_bridge_n() -> f64 {
    4.0 * std::f64::consts::PI * GAMMA * GRAIN_RELIEF_UM * 1e-6
}

/// The largest crumb a tap at `g` gravities leaves where the water is
/// only in the pores, µm (§5): the dry hold and the grain's bridge
/// (`pore_bridge_n`) against the wet crumb's weight, d = (6F/πρa)^⅓:
/// 87 µm at 100 g, against a dry crumb's 43 and a film's 585.
pub fn pore_fall_um(g: f64) -> f64 {
    let f = crate::sheet::ADHESION_N as f64 + pore_bridge_n();
    (6.0 * f / (std::f64::consts::PI * WET_CRUMB_KG_M3 * 9.81 * g.max(1e-9))).cbrt() * 1e6
}

/// The water standing that fills a crumb's contact to its own curvature,
/// µm over the cell (§5): Halsey & Levine's bridge saturates at 2πγR
/// (sphere to sphere) once it holds V₂ = l_R² R, l_R the roughness; over
/// the section π R² of the smallest crumb (`CRUMB_MIN_UM`), with
/// l_R = `GRAIN_RELIEF_UM`: l_R²/(π R), 0.09 µm.
pub fn film_saturates_um() -> f64 {
    GRAIN_RELIEF_UM * GRAIN_RELIEF_UM / (std::f64::consts::PI * CRUMB_MIN_UM / 2.0)
}

/// How the water at cell `c` holds its crumbs (§5), `p` its perched crumbs'
/// record and `under_um` the water in the sheet beneath it (its
/// own, `Drawing::sheet`, and a damp sheet's free water, `crate::damp`).
/// The pores (the pastel's, the perched crumbs counted as crushed, as
/// they drank on the sift) take the water first; water standing past
/// them (`film_saturates_um`), or held in the paper's larger pores
/// beneath (r ≈ 6 µm, so a weaker suction: the bridges reach 6 µm), feeds
/// a bridge that fills the contact (`Wet::Film`); pore water alone, at the
/// pores' suction, reaches only 0.69 µm, under the grains' relief
/// (`Wet::Pores`).
pub(crate) fn wetness(c: &Cell, p: Option<&[f32]>, under_um: f32) -> Wet {
    if !(c.water > 0.0) {
        return Wet::Dry;
    }
    let crushed = p.map_or(0.0, |p| crumb_pore_extra(p, c));
    let pores = POROSITY * ((c.loose + c.bound).max(0.0) + crushed);
    if (c.water - pores) as f64 > film_saturates_um() || under_um > 0.0 { Wet::Film } else { Wet::Pores }
}

/// A spherical cap's volume over π R³/3 at contact angle θ:
/// (2 − 3 cos θ + cos³ θ)/sin³ θ.
fn cap_factor(theta: f64) -> f64 {
    let (c, s) = (theta.cos(), theta.sin());
    (2.0 - 3.0 * c + c * c * c) / (s * s * s)
}

/// The radius of the drops `film_um` of water standing on a sheet breaks
/// into as it dries, µm (§6): drops coalescing on a partly wetting surface
/// cover a constant share of it (`DROP_COVER`), so the mean height is that
/// share times a cap's: h̄ = ε² R f(θ)/3, R = 3h̄/(ε² f(θ)).
pub fn drop_radius_um(film_um: f64, sized: bool) -> f64 {
    let theta = if sized { SIZED_ANGLE_DEG } else { WETTED_ANGLE_DEG }.to_radians();
    3.0 * film_um.max(0.0) / (DROP_COVER * cap_factor(theta))
}

/// The mass-mean size, µm, of crushed pastel between `lo` and `hi` µm:
/// m/(m + 1) (hi^(m+1) − lo^(m+1)) / (hi^m − lo^m).
pub fn mean_size_um(lo: f64, hi: f64) -> f64 {
    let m = GGS_M;
    if hi <= lo * 1.0001 {
        return lo;
    }
    m / (m + 1.0) * (hi.powf(m + 1.0) - lo.powf(m + 1.0)) / (hi.powf(m) - lo.powf(m))
}

/// How wet crumbs gather as a standing film dries (`cluster`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cluster {
    /// The film's drops' radius, µm (`drop_radius_um`).
    pub drop_um: f64,
    /// The largest crumb a drop takes in, µm: its own diameter, 2R.
    pub take_um: f64,
    /// The share of the sift (by mass, fines included) small enough.
    pub share: f64,
    /// Those crumbs' mass-mean size, µm.
    pub crumb_um: f64,
    /// The islands they dry into: their diameter, µm.
    pub size_um: f64,
}

/// The clusters crumbs sifted at `v_um` (µm of stick volume over the area)
/// through a sieve of `top_um` gather into as `film_um` of water standing
/// among them dries (§6). The film breaks into drops (`drop_radius_um`).
/// A drop takes in the crumbs smaller than itself (to 2R) that fall within
/// R + d/2 of its centre, its menisci pulling them together as it shrinks
/// (the lateral capillary force between particles in a thinning film,
/// Kralchevsky & Nagayama 1994), and dries into one island a crumb thick;
/// larger crumbs drink the drops they touch and stay single. Those small
/// crumbs, of mass-mean size d, cover φ = v_s/(2d/3) of the sheet, so an
/// island's area is φ π (R + d/2)²: its diameter 2√φ (R + d/2), never less
/// than one crumb. It grows with the film, as R does.
pub fn cluster(film_um: f64, v_um: f64, top_um: f64, sized: bool) -> Cluster {
    let r = drop_radius_um(film_um, sized);
    let take = (2.0 * r).clamp(PRIMARY_UM, top_um.max(PRIMARY_UM));
    let share = ggs_finer(take, top_um);
    let d = mean_size_um(PRIMARY_UM, take);
    let phi = (v_um * share / (2.0 * d / 3.0)).clamp(0.0, 1.0);
    Cluster { drop_um: r, take_um: take, share, crumb_um: d, size_um: d.max(2.0 * phi.sqrt() * (r + 0.5 * d)) }
}

/// What a unit of crumbs of size `d_um` covers for its volume, against a
/// stick's deposit (`Grains::lies`): a sphere covers its section, 2d/3 of
/// its volume as a layer's thickness, against `PARTICLE_UM`; never more
/// than a stick's deposit.
fn lies_of(d_um: f64) -> f32 {
    (PARTICLE_UM as f64 / (2.0 * d_um / 3.0)).min(1.0) as f32
}

/// Draw a Poisson count of mean `lambda`.
fn poisson(rng: &mut crate::rng::Rng, lambda: f64) -> f64 {
    if lambda <= 0.0 {
        return 0.0;
    }
    if lambda > 30.0 {
        return (lambda + lambda.sqrt() * rng.normal() as f64).round().max(0.0);
    }
    let l = (-lambda).exp();
    let (mut k, mut p) = (0.0, 1.0);
    loop {
        p *= rng.f() as f64;
        if p <= l {
            return k;
        }
        k += 1.0;
    }
}

/// Keep a cell's record of perched crumbs within what the cell still holds:
/// a tool that took loose pastel (a finger, the eraser, a puff) took its
/// share of the crumbs; fixative or gum that bound it, its share.
pub(crate) fn sync(p: &mut [f32], c: &Cell) {
    if p[P_V] <= 0.0 {
        return;
    }
    let held = p[P_HELD].clamp(0.0, 1.0);
    let (lo, bo) = (p[P_L] * (1.0 - held), p[P_L] * held);
    let (lo2, bo2) = (lo.min(c.loose.max(0.0)), bo.min(c.bound.max(0.0)));
    if lo2 + bo2 < p[P_L] {
        let k = if p[P_L] > 0.0 { (lo2 + bo2) / p[P_L] } else { 0.0 };
        p[P_V] *= k;
        p[P_L] = lo2 + bo2;
        p[P_HELD] = if p[P_L] > 0.0 { bo2 / p[P_L] } else { 0.0 };
    }
    if p[P_V] <= 1e-9 {
        p.fill(0.0);
    }
}

/// The crumbs perched on cell `c` are bound (`held` of the loose pastel
/// there, by gum or fixative): their record's held share follows.
pub(crate) fn bind(p: &mut [f32], held: f32) {
    if p[P_V] > 0.0 {
        p[P_HELD] = p[P_HELD].max(held.clamp(0.0, 1.0));
    }
}

/// A tap at `g` gravities on cell `c` with its perched crumbs `p`: the
/// loose ones whose weight beats their hold fall (dry: a grain's adhesion;
/// wet, while the cell holds water: a liquid bridge at the crumb's radius).
/// With `under` (the steam-and-sift switch's bridges: the water in the
/// sheet beneath, µm) the bridge is weighed as the water lies
/// (`wetness`): at the crumb's radius only where a film stands or the
/// sheet beneath is wet, at the touching grain where the pores alone hold
/// it. Returns (the volume shed as crushed, µm; as it lay, µm; the loose
/// crumbs left, as they lie).
pub(crate) fn tap(p: &mut [f32], c: &Cell, g: f32, under: Option<f32>) -> (f32, f32, f32) {
    sync(p, c);
    if p[P_V] <= 0.0 {
        return (0.0, 0.0, 0.0);
    }
    let free = 1.0 - p[P_HELD].clamp(0.0, 1.0);
    let wet = match under {
        None if c.water > 0.0 => Wet::Film,
        None => Wet::Dry,
        Some(u) => wetness(c, Some(p), u),
    };
    let dc = match wet {
        Wet::Dry => dry_fall_um(g as f64),
        Wet::Pores => pore_fall_um(g as f64),
        Wet::Film => wet_fall_um(g as f64),
    };
    // (the crumbs' sizes as the sieve's, clusters among them: the islands
    // are smaller than a wet bridge lets fall, and bound once dry)
    let share = ggs_share_above(dc, p[P_TOP] as f64) as f32;
    let (dv, dl) = (p[P_V] * free * share, p[P_L] * free * share);
    if share > 0.0 {
        let bo = p[P_L] * (1.0 - free);
        p[P_V] -= dv;
        p[P_L] -= dl;
        p[P_TOP] = p[P_TOP].min(dc as f32);
        p[P_HELD] = if p[P_L] > 0.0 { (bo / p[P_L]).min(1.0) } else { 0.0 };
    }
    let left = p[P_L] * (1.0 - p[P_HELD]);
    if p[P_V] <= 1e-9 {
        p.fill(0.0);
    }
    (dv, dl, left)
}

/// The loose perched crumbs a puff may move at cell `c` (as they lie, µm):
/// dry ones only; a liquid bridge holds wet ones.
pub(crate) fn blowable(p: &mut [f32], c: &Cell) -> f32 {
    sync(p, c);
    if c.water > 0.0 { 0.0 } else { p[P_L] * (1.0 - p[P_HELD].clamp(0.0, 1.0)) }
}

/// The loose perched crumbs at cell `c`, wet or dry (as they lie, µm), the
/// largest of them, µm, and how the water holds them (`wetness`, `under_um`
/// the sheet's water beneath): what a puff weighs against their hold when a
/// liquid bridge may hold them (sheet.rs `blow_pastel_with`).
pub(crate) fn loose_perched(p: &mut [f32], c: &Cell, under_um: f32) -> (f32, f32, Wet) {
    sync(p, c);
    (p[P_L] * (1.0 - p[P_HELD].clamp(0.0, 1.0)), p[P_TOP], wetness(c, Some(p), under_um))
}

/// A puff left no perched crumb at cell `p` larger than `d_um` (it moved
/// every one whose threshold it passed): the record's largest falls to it,
/// as a tap's does.
pub(crate) fn blown_to(p: &mut [f32], d_um: f32) {
    if p[P_V] > 0.0 {
        p[P_TOP] = p[P_TOP].min(d_um);
    }
}

/// The pores the perched crumbs `p` at cell `c` hold beyond what the cell
/// counts for them, as stick volume, µm: a crumb drinks its pores as
/// crushed (`Canvas::sift`), while the cell counts it as it lies
/// (`Grains::lies`). Read through a synced copy: the record isn't changed.
pub(crate) fn crumb_pore_extra(p: &[f32], c: &Cell) -> f32 {
    let mut q = [0.0f32; PERCH];
    q.copy_from_slice(&p[..PERCH]);
    sync(&mut q, c);
    (q[P_V] - q[P_L]).max(0.0)
}

/// `taken` of a cell's loose perched crumbs (as they lie, µm) went with a puff.
pub(crate) fn blown(p: &mut [f32], taken: f32) {
    let lo = p[P_L] * (1.0 - p[P_HELD]);
    if lo <= 0.0 || taken <= 0.0 {
        return;
    }
    let t = taken.min(lo);
    let k = t / p[P_L];
    p[P_V] *= 1.0 - k;
    let bo = p[P_L] - lo;
    p[P_L] -= t;
    p[P_HELD] = if p[P_L] > 0.0 { bo / p[P_L] } else { 0.0 };
    if p[P_V] <= 1e-9 {
        p.fill(0.0);
    }
}

/// What `Canvas::steam` did.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Steamed {
    /// The most water condensed, µm (g/m²): on the jet's track.
    pub water_um: f64,
    /// The face's temperature at the end, °C, and the jet's dew point.
    pub face_c: f64,
    pub dew_c: f64,
    /// The hand's time, s: the pass.
    pub secs: f64,
}

/// What `Canvas::sift` did. Volumes in mm³ as crushed (a stick's packing).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Sifted {
    /// All that fell on the picture.
    pub laid_mm3: f64,
    /// What fell beside the picture, the sieve held over its edge.
    pub off_mm3: f64,
    /// The fines that went into the tooth's pores.
    pub fines_mm3: f64,
    /// The crumbs lying on the tooth, loose (dry) or held by water (wet).
    pub loose_mm3: f64,
    pub stuck_mm3: f64,
    /// Caught by a mask sheet laid on the picture, or lost in wet paint.
    pub caught_mm3: f64,
    /// The clusters' mass-mean size where the crumbs fell wet and full, µm
    /// (0: none did), and the water film standing there once the crumbs
    /// drank, µm (both weighted by the clusters' mass).
    pub cluster_um: f64,
    pub film_um: f64,
    /// The single crumbs' mass-mean size, µm.
    pub crumb_um: f64,
}

impl Canvas {
    /// A kettle's steam `distance_mm` away, moved along `pts` (units) at an
    /// even pace in `seconds` (one point: held there). Each point of the
    /// sheet is steamed for the time the jet's steam share is on it, by the
    /// jet's Gaussian profile out to its reach (`steam_exposure`), and the
    /// water the jet condenses there in that time (`condense`) is laid as the wet pastel
    /// switch's water (`wet_pastel::add_water`): the pastel's pores take it
    /// first, what is left stands and sinks into the sheet, the gum
    /// dissolves and sets again as it dries. Not under a mask sheet, on wet
    /// paint, or on pastel painted over. Needs paper, drawing optics and the
    /// wet pastel switch.
    pub fn steam(&mut self, pts: &[(f32, f32)], seconds: f64, distance_mm: f64) -> Result<Steamed, String> {
        let p = *self.paper.as_ref().ok_or("steam: it is not a sheet of paper")?;
        if !(self.wet_pastel && self.drawing_optics) {
            return Err("steam: needs the wet pastel switch (water in pastel) and drawing optics".into());
        }
        let pl = plume(distance_mm);
        let taus = self.steam_exposure(pts, seconds, &pl);
        let most = taus.iter().cloned().fold(0.0f64, f64::max);
        let cd = condense(&p, distance_mm, most);
        self.drawing_mut();
        let film = std::mem::take(&mut self.film);
        let mut pxs = std::mem::take(&mut self.px);
        let mut d = self.drawing.take().expect("drawing_mut made it");
        let mut any = false;
        for i in 0..d.cells.len() {
            let tau = taus[i];
            if tau <= 0.0 || self.sheet_over(i) || self.wet.vol[i] > 1e-5 {
                continue;
            }
            let c = &mut d.cells[i];
            if c.film >= 0.0 && film[i] > c.film + 1e-4 && c.loose + c.bound > 0.0 {
                // (pastel under paint: the paint keeps the water off)
                continue;
            }
            if c.film < 0.0 || film[i] > c.film + 1e-4 {
                c.restart(film[i], pxs[i]);
            }
            let um = cd.at(tau) as f32;
            if um <= 0.0 {
                continue;
            }
            let before = (c.a, c.reflectance());
            crate::wet_pastel::add_water(c, um);
            pxs[i] = c.repaint(pxs[i], before);
            any = true;
        }
        if any {
            d.damp = true;
        }
        self.film = film;
        self.px = pxs;
        self.drawing = Some(d);
        Ok(Steamed { water_um: cd.at(most), face_c: *cd.face_c.last().unwrap_or(&STUDIO.air_c), dew_c: pl.dew_c, secs: seconds })
    }

    /// Each pixel's steaming time, s, for the jet `pl` moved along `pts`
    /// (units) at an even pace in `seconds` (one point, or none apart: held
    /// at the first): ∫ C(r(t))/C_c dt, the steam share on it over the
    /// axis's (`Plume::share_at`), in closed form along each straight leg.
    pub fn steam_exposure(&self, pts: &[(f32, f32)], seconds: f64, pl: &Plume) -> Vec<f64> {
        let reach = pl.reach_m * 1e3;
        let k = std::f64::consts::LN_2 / (pl.scalar_half_m * 1e3).powi(2);
        let sk = k.sqrt();
        let half_gauss = 0.5 * std::f64::consts::PI.sqrt() / sk;
        self.over_path(pts, reach, |legs, total, x, y| {
            if total <= 0.0 {
                let (ax, ay) = legs[0].0;
                let r = ((x - ax).powi(2) + (y - ay).powi(2)).sqrt();
                return seconds * pl.share_at(r * 1e-3);
            }
            let mut t = 0.0;
            for &((ax, ay), (ex, ey), l) in legs {
                // along the leg u, across it y; the part of it within reach
                let (u, v) = ((x - ax) * ex + (y - ay) * ey, -(x - ax) * ey + (y - ay) * ex);
                if v.abs() >= reach {
                    continue;
                }
                let h = (reach * reach - v * v).sqrt();
                let (a, c) = ((u - h).max(0.0), (u + h).min(l));
                if c <= a {
                    continue;
                }
                let dt_per_mm = seconds / total;
                t += dt_per_mm * (-k * v * v).exp() * half_gauss * (erf(sk * (u - a)) - erf(sk * (u - c)));
            }
            t
        })
    }

    /// The share of a sift held over `pts` (units) that falls on each pixel:
    /// the sieve's mesh a disc `sieve_mm` across, the crumbs falling straight
    /// down, moved along the points at an even pace (one point: held there).
    pub fn sift_share(&self, pts: &[(f32, f32)], sieve_mm: f64) -> Vec<f64> {
        let r = 0.5 * sieve_mm;
        let px_mm2 = (self.px_mm() as f64).powi(2);
        let disc = std::f64::consts::PI * r * r;
        self.over_path(pts, r, |legs, total, x, y| {
            if total <= 0.0 {
                let (ax, ay) = legs[0].0;
                return if (x - ax).powi(2) + (y - ay).powi(2) < r * r { px_mm2 / disc } else { 0.0 };
            }
            let mut along = 0.0;
            for &((ax, ay), (ex, ey), l) in legs {
                let (u, v) = ((x - ax) * ex + (y - ay) * ey, -(x - ax) * ey + (y - ay) * ex);
                if v.abs() >= r {
                    continue;
                }
                let h = (r * r - v * v).sqrt();
                along += ((u + h).min(l) - (u - h).max(0.0)).max(0.0);
            }
            along / total * px_mm2 / disc
        })
    }

    /// `f(legs, length, x, y)` at each pixel's middle (mm) within `reach_mm`
    /// of the path `pts` (units), 0 elsewhere. The legs are (start, unit
    /// direction, length), in mm; with no length, one leg at the first point.
    fn over_path(&self, pts: &[(f32, f32)], reach_mm: f64, f: impl Fn(&[((f64, f64), (f64, f64), f64)], f64, f64, f64) -> f64) -> Vec<f64> {
        let fr = self.f;
        let mut out = vec![0.0; self.px.len()];
        if pts.is_empty() || !(reach_mm > 0.0) {
            return out;
        }
        let mpu = self.mm_per_unit() as f64;
        let mm: Vec<(f64, f64)> = pts.iter().map(|&(x, y)| (x as f64 * mpu, y as f64 * mpu)).collect();
        let mut legs = Vec::new();
        let mut total = 0.0;
        for w in mm.windows(2) {
            let (dx, dy) = (w[1].0 - w[0].0, w[1].1 - w[0].1);
            let l = (dx * dx + dy * dy).sqrt();
            if l > 0.0 {
                legs.push((w[0], (dx / l, dy / l), l));
                total += l;
            }
        }
        if legs.is_empty() {
            legs.push((mm[0], (1.0, 0.0), 0.0));
        }
        let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
        for &(x, y) in &mm {
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x);
            y1 = y1.max(y);
        }
        let (x0, y0, x1, y1) = (x0 - reach_mm, y0 - reach_mm, x1 + reach_mm, y1 + reach_mm);
        for (i, o) in out.iter_mut().enumerate() {
            let (x, y) = (fr.ux(i % fr.w) as f64 * mpu, fr.uy(i / fr.w) as f64 * mpu);
            if x < x0 || x > x1 || y < y0 || y > y1 {
                continue;
            }
            *o = f(&legs, total, x, y);
        }
        out
    }

    /// Sift `vol_mm3` of crushed `stick` (its color, particle index, carbon
    /// and lift; a stick's packing) through a sieve of aperture `grain_mm`
    /// and `sieve_mm` across, held over `pts` (`sift_share`, §4); what falls
    /// beside the picture is lost.
    /// The fines (below `CRUMB_MIN_UM`) settle into the tooth's pores as a
    /// deposit does. The crumbs fall one by one (Poisson, by size bins of
    /// the crushing's distribution, `GGS_M`), and lie on the tooth where
    /// they fall, covering their sections (`lies_of`). Where the sheet or
    /// the pastel is wet (standing water, water in the sheet beneath, or a
    /// damp sheet's own free water, `crate::damp`), they draw water into
    /// their pores first; once full, the water left standing gathers them
    /// into clusters (`cluster_um`), which fall in their place. `seed` makes
    /// the fall repeatable. Needs paper and drawing optics.
    #[allow(clippy::too_many_arguments)]
    pub fn sift(&mut self, pts: &[(f32, f32)], vol_mm3: f64, grain_mm: f64, sieve_mm: f64, stick: &Stick, seed: u64) -> Result<Sifted, String> {
        let p = *self.paper.as_ref().ok_or("sift: it is not a sheet of paper")?;
        if !self.drawing_optics {
            return Err("sift: needs drawing optics".into());
        }
        if !(vol_mm3 >= 0.0 && grain_mm > 0.0 && sieve_mm > 0.0) {
            return Err("sift: the amount is mm³ (0 or more), the sieve's aperture and width mm (more than 0)".into());
        }
        let top = grain_mm * 1e3;
        let s_mm = self.px_mm() as f64;
        let px_um2 = (s_mm * 1e3).powi(2);
        let sized = p.cobb60.is_some();
        let mut grains = Grains::of(stick);
        if self.wet_pastel {
            grains.index = stick.index;
        }
        // the sieve's crumbs: the fines, and log-spaced bins of the rest
        let fine_share = ggs_finer(CRUMB_MIN_UM, top);
        let edges: Vec<f64> = if top > CRUMB_MIN_UM {
            let r = (top / CRUMB_MIN_UM).powf(1.0 / SIZE_BINS as f64);
            (0..=SIZE_BINS).map(|k| CRUMB_MIN_UM * r.powi(k as i32)).collect()
        } else {
            Vec::new()
        };
        let crumb_d = mean_crumb_um(top);
        // where it falls: each pixel's share of it under the sieve
        let n = self.px.len();
        let mut weights = self.sift_share(pts, sieve_mm);
        // (the pixels' middles sample the mesh's disc: never more than all of it)
        let raw: f64 = weights.iter().sum();
        if raw > 1.0 {
            weights.iter_mut().for_each(|w| *w /= raw);
        }
        let total_w = raw.min(1.0);
        let mut out = Sifted { laid_mm3: vol_mm3 * total_w, off_mm3: vol_mm3 * (1.0 - total_w), crumb_um: crumb_d, ..Sifted::default() };
        if total_w <= 0.0 || vol_mm3 <= 0.0 {
            return Ok(out);
        }
        // µm of stick volume over a pixel, per unit share
        let per_w = vol_mm3 * 1e9 / px_um2;
        let global_free = self.damp_free() as f64;
        let sheet_area_mm2 = self.width() as f64 * self.height() as f64 * (self.mm_per_unit() as f64).powi(2);
        self.drawing_mut();
        let mut d = self.drawing.take().expect("drawing_mut made it");
        if d.perched.len() != d.cells.len() * PERCH {
            d.perched = vec![0.0; d.cells.len() * PERCH];
        }
        let mut rng = crate::rng::Rng::new(seed ^ 0x5157_4654_5349_4654);
        // single crumbs above `above` µm, by size bins: (volume, as it lies), µm
        let singles = |rng: &mut crate::rng::Rng, v: f64, above: f64| -> (f64, f64) {
            let (mut got, mut lying) = (0.0, 0.0);
            for k in 0..edges.len().saturating_sub(1) {
                let (a, b) = (edges[k].max(above), edges[k + 1]);
                if b <= a {
                    continue;
                }
                let share = ggs_finer(b, top) - ggs_finer(a, top);
                let dm = mean_size_um(a, b);
                let vu = std::f64::consts::FRAC_PI_6 * dm * dm * dm;
                let g = poisson(rng, v * share * px_um2 / vu) * vu / px_um2;
                got += g;
                lying += g * lies_of(dm) as f64;
            }
            (got, lying)
        };
        // pass 1: what falls where, wet or dry (µm of stick volume over the pixel)
        struct Fall {
            i: usize,
            fine: f64,
            v: f64,
            lying: f64,
            unit: f64,
            from_sheet: f64,
            from_damp: f64,
            wet: bool,
        }
        let mut falls = Vec::new();
        let (mut drawn, mut expected) = (0.0f64, 0.0f64);
        let (mut cl_m, mut cl_v, mut film_m) = (0.0f64, 0.0f64, 0.0f64);
        for i in 0..n {
            let w = weights[i];
            if w <= 0.0 {
                continue;
            }
            let v = per_w * w;
            if self.sheet_over(i) || self.wet.vol[i] > 1e-5 {
                out.caught_mm3 += v * px_um2 * 1e-9;
                continue;
            }
            let c = &d.cells[i];
            // (crumbs sifted before hold their pores as crushed: `crumb_pore_extra`)
            let crumbs = crumb_pore_extra(&d.perched[i * PERCH..(i + 1) * PERCH], c);
            let pores = POROSITY as f64 * ((c.loose + c.bound).max(0.0) + crumbs) as f64;
            let standing = (c.water as f64 - pores).max(0.0);
            let sheet_u = d.sheet.get(i).copied().unwrap_or(0.0) as f64;
            let avail_sheet = sheet_u + global_free;
            let wet = standing + avail_sheet > 0.0;
            let (mut from_sheet, mut from_damp, mut film_left, mut full) = (0.0, 0.0, 0.0, false);
            if wet {
                // the crumbs' pores draw first from what stands, then from the sheet
                let need = POROSITY as f64 * v;
                let take_s = standing.min(need);
                let more = (need - take_s).min(avail_sheet);
                from_sheet = more.min(sheet_u);
                from_damp = more - from_sheet;
                film_left = standing - take_s;
                full = take_s + more >= need * (1.0 - 1e-6);
            }
            let (fine, v_cr, lying, unit) = if wet && full && film_left > 0.0 {
                // the drops of the film left gather the crumbs smaller than
                // themselves, fines and all, into islands; larger ones stay single
                let cg = cluster(film_left, v, top, sized);
                let vs = v * cg.share;
                let vu = std::f64::consts::PI * cg.size_um * cg.size_um * cg.crumb_um / 6.0;
                let g = poisson(&mut rng, vs * px_um2 / vu) * vu / px_um2;
                let (gs, ls) = singles(&mut rng, v, cg.take_um.max(CRUMB_MIN_UM));
                cl_m += g * cg.size_um;
                cl_v += g;
                film_m += g * film_left;
                expected += vs + v * (1.0 - cg.share.max(fine_share));
                (v * (fine_share - cg.share).max(0.0), g + gs, g * lies_of(cg.crumb_um) as f64 + ls, cg.size_um)
            } else {
                let (gs, ls) = singles(&mut rng, v, CRUMB_MIN_UM);
                expected += v * (1.0 - fine_share);
                (v * fine_share, gs, ls, 0.0)
            };
            drawn += v_cr;
            falls.push(Fall { i, fine, v: v_cr, lying, unit, from_sheet, from_damp, wet });
        }
        // the crumbs' count is random; their total is what was sifted
        let scale = if drawn > 0.0 { expected / drawn } else { 0.0 };
        if drawn <= 0.0 && expected > 0.0 {
            // (too few to draw: they settle as fines would)
            let each = expected / falls.len().max(1) as f64;
            for fl in &mut falls {
                fl.fine += each;
            }
        }
        let film = std::mem::take(&mut self.film);
        let mut pxs = std::mem::take(&mut self.px);
        let mut taken_damp = 0.0f64;
        let mut any_wet = false;
        for fl in &mut falls {
            fl.v *= scale;
            fl.lying *= scale;
            let i = fl.i;
            let c = &mut d.cells[i];
            if c.film < 0.0 || film[i] > c.film + 1e-4 {
                c.restart(film[i], pxs[i]);
            }
            let (fv, cv) = (fl.fine as f32, fl.lying as f32);
            let dv = fv + cv;
            if dv > 0.0 {
                let before = (c.a, c.reflectance());
                let q = 1.0 - (-dv / PARTICLE_UM).exp();
                crate::pastel::lay_into(c, q, stick.color, dv, Grains { lies: 1.0, ..grains });
                pxs[i] = c.repaint(pxs[i], before);
            }
            if fl.wet {
                // the water the crumbs drew from the sheet beneath, and from a damp sheet's own
                let up = (fl.from_sheet + fl.from_damp) as f32;
                if let Some(s) = d.sheet.get_mut(i) {
                    *s = (*s - fl.from_sheet as f32).max(0.0);
                }
                taken_damp += fl.from_damp * px_um2;
                let before = (c.a, c.reflectance());
                c.water += up;
                if c.gum == 0 && c.loose + c.bound > 0.0 {
                    c.gum = crate::wet_pastel::to_gum(1.0 - c.carbon_share());
                }
                pxs[i] = c.repaint(pxs[i], before);
                any_wet = any_wet || c.water > 0.0;
            }
            if fl.v > 0.0 {
                let pr = &mut d.perched[i * PERCH..(i + 1) * PERCH];
                sync(pr, c);
                let bo = pr[P_L] * pr[P_HELD];
                pr[P_V] += fl.v as f32;
                pr[P_L] += cv;
                pr[P_HELD] = if pr[P_L] > 0.0 { bo / pr[P_L] } else { 0.0 };
                pr[P_TOP] = pr[P_TOP].max(top as f32);
                pr[P_UNIT] = pr[P_UNIT].max(fl.unit as f32);
            }
            let mm3 = |um: f64| um * px_um2 * 1e-9;
            out.fines_mm3 += mm3(fl.fine);
            if fl.wet { out.stuck_mm3 += mm3(fl.v) } else { out.loose_mm3 += mm3(fl.v) }
        }
        if any_wet {
            d.damp = true;
        }
        self.film = film;
        self.px = pxs;
        self.drawing = Some(d);
        if taken_damp > 0.0 {
            if let Some(dm) = self.damp.as_mut() {
                // (µm·µm² over the sheet's µm²)
                dm.water = (dm.water - taken_damp / (sheet_area_mm2 * 1e6)).max(0.0);
            }
        }
        out.cluster_um = if cl_v > 0.0 { cl_m / cl_v } else { 0.0 };
        out.film_um = if cl_v > 0.0 { film_m / cl_v } else { 0.0 };
        Ok(out)
    }

    /// The pastel on the picture's tooth, mm³ as crushed or worn from a
    /// stick: what the cells hold, the perched crumbs counted as they were
    /// crushed rather than as they lie (`Drawing::perched`).
    pub fn pastel_mm3(&self) -> f64 {
        let Some(d) = self.drawing.as_ref() else { return 0.0 };
        let px_um2 = (self.px_mm() as f64 * 1e3).powi(2);
        let mut v = 0.0f64;
        for (i, c) in d.cells.iter().enumerate() {
            v += (c.loose + c.bound).max(0.0) as f64;
            if let Some(p) = d.perched.get(i * PERCH..(i + 1) * PERCH) {
                v += (p[P_V] - p[P_L]) as f64;
            }
        }
        v * px_um2 * 1e-9
    }

    /// The crumbs perched on the tooth, mm³ as crushed: (loose, held by gum
    /// or fixative). Read as the cells now hold them.
    pub fn perched_mm3(&self) -> (f64, f64) {
        let Some(d) = self.drawing.as_ref() else { return (0.0, 0.0) };
        let px_um2 = (self.px_mm() as f64 * 1e3).powi(2);
        let (mut lo, mut bo) = (0.0f64, 0.0f64);
        for (i, c) in d.cells.iter().enumerate() {
            let Some(p) = d.perched.get(i * PERCH..(i + 1) * PERCH) else { break };
            let mut q = [0.0f32; PERCH];
            q.copy_from_slice(p);
            sync(&mut q, c);
            lo += (q[P_V] * (1.0 - q[P_HELD])) as f64;
            bo += (q[P_V] * q[P_HELD]) as f64;
        }
        (lo * px_um2 * 1e-9, bo * px_um2 * 1e-9)
    }
}


#[cfg(test)]
mod tests;
