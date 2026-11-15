//! What holds a dry particle to a paper fibre, and whether a kneaded eraser
//! pressed on it out-pulls that hold: the law charcoal's lift is taken from
//! (charcoal.md §4, charcoal_optics.py §3), here as code, so that pastel's
//! lift comes from the same law (notes/research/pastel_lift.md, and
//! pastel_lift.py, which computes it separately).
//!
//! - **The eraser** is a soft putty (E ~0.1 MPa) that wraps a particle:
//!   JKR pull-off 1.5πWR, W = A/12πD₀², A the eraser–particle Hamaker
//!   constant (the geometric mean of the two).
//! - **The fibre** holds a rough particle above the asperity scale by
//!   10–150 nN whatever its size (measured for irregular grains, Nagaashi
//!   et al. 2021; pastel_brushing.md §1), a decade wide (lognormal); a
//!   finer one sits on one smooth asperity, DMT A·R/6D₀².
//! - **Water** condenses at the contact when the room's Kelvin diameter is
//!   wider than the contact's gap (at any studio humidity: above 0.2 % RH),
//!   and the bridge at an asperity of radius a holds 2πγa(cos θ_p + cos θ_f)
//!   more. The fibre is wetted (θ_f 0); the eraser's hydrocarbon putty is
//!   not, so it gets no bridge.
//!
//! The share the eraser takes is the chance its pull beats the hold.

/// Cellulose's Hamaker constant, J (Bergström et al. 1999).
const CELLULOSE_A: f64 = 5.8e-20;
/// A kneaded eraser's: polyisobutylene with oil and CaCO₃/TiO₂ fillers, as
/// a hydrocarbon polymer (6–7 ×10⁻²⁰ J).
const ERASER_A: f64 = 6.5e-20;
/// Char's (carbon fibres 5.2–6.2, activated carbon 5.0–7.8 ×10⁻²⁰ J:
/// Maurer, Mersmann & Peukert 2001).
pub const CARBON_A: f64 = 6.0e-20;
/// Calcite's, measured 10.19 ± 0.55 ×10⁻²⁰ J (Bergström 1997): whiting's.
pub const WHITING_A: f64 = 10.1e-20;
/// The contact's cut-off distance, m.
const D0: f64 = 0.165e-9;
/// Water's surface tension (J/m²) and molar volume (m³/mol).
const GAMMA: f64 = 0.072;
const V_M: f64 = 18.0e-6;
/// The fibre's asperity a particle rests on, m (0.05–0.2 µm; Nagaashi's
/// effective contact radii are 0.14–0.5 µm).
const ASPERITY: f64 = 0.1e-6;
/// A rough particle's hold on the fibre, N: the geometric middle of 10–150 nN.
const ROUGH_HOLD: f64 = 3.872_983_346_207_417e-8;
/// Below this radius (m) a particle sits on one smooth asperity.
const FINE_R: f64 = 0.3e-6;

/// A kind of particle: its Hamaker constant (J) and the cosine of water's
/// contact angle on it (1 wetted; 0 for one water stands on at 90°).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Particle {
    pub hamaker: f64,
    pub wets: f64,
}

/// Twice the Kelvin radius of a meniscus in `room`, m: water condenses in
/// any narrower gap.
pub fn kelvin_diameter(room: crate::climate::Climate) -> f64 {
    let rh = room.rh.clamp(1e-9, 1.0 - 1e-12);
    2.0 * GAMMA * V_M / (8.314 * (room.air_c + 273.15) * (1.0 / rh).ln())
}

/// The share of particles of radius `r` (m) one eraser press takes off a
/// fibre in `room` (0..1).
pub fn taken(r: f64, p: Particle, room: crate::climate::Climate) -> f64 {
    let w = (ERASER_A * p.hamaker).sqrt() / (12.0 * std::f64::consts::PI * D0 * D0);
    let pull = 1.5 * std::f64::consts::PI * w * r;
    let vdw = if r < FINE_R { (CELLULOSE_A * p.hamaker).sqrt() * r / (6.0 * D0 * D0) } else { ROUGH_HOLD };
    let water = if kelvin_diameter(room) > D0 { 2.0 * std::f64::consts::PI * GAMMA * r.min(ASPERITY) * (p.wets + 1.0).max(0.0) } else { 0.0 };
    // a decade wide: ±1σ spans ×10
    normal_cdf((pull / (vdw + water)).ln() / (10f64.ln() / 2.0))
}

/// `taken` over a lognormal size distribution by volume with Sauter mean
/// `d32_um` (µm) and spread `sigma` (σ_ln), averaged by projected area: the
/// share of a mark's coverage the eraser takes (coverage per volume goes as
/// 1/d).
pub fn taken_of_sizes(d32_um: f64, sigma: f64, p: Particle, room: crate::climate::Climate) -> f64 {
    // (by area the lognormal's median is d32·e^(−σ²/2))
    let median = d32_um * (-sigma * sigma / 2.0).exp();
    const N: usize = 240;
    let (mut acc, mut tot) = (0.0, 0.0);
    for i in 0..N {
        let z = -6.0 + 12.0 * (i as f64 + 0.5) / N as f64;
        let w = (-z * z / 2.0).exp();
        acc += w * taken(median * (sigma * z).exp() / 2.0 * 1e-6, p, room);
        tot += w;
    }
    acc / tot
}

/// A pigment's Hamaker constant (J) from its refractive index `n`, by
/// Tabor–Winterton: ¾kT((ε−1)/(ε+1))² + 3ħω_e/(16√2) (n²−1)²/(n²+1)^1.5.
/// The UV absorption frequency ω_e is the class's (Bergström 1997): 1.96
/// ×10¹⁶ rad/s for wide-gap insulators (silica, alumina, BeO, calcite, MgO,
/// quartz: 1.71–2.11) below n 1.85, 0.815 for the high-index oxides above
/// it (TiO₂ 0.735, ZnO 0.895). ε, unknown for most pigments, is n², its
/// least; the static term is under 3 % of A. Against measured: calcite
/// 9.6 (10.1), TiO₂ 15.7 (15), ZnO 8.1 (9.2), MgO 13.3 (12.1), silica 6.1
/// (6.5), hematite 23.3 (≈ 25).
pub fn hamaker_of_index(n: f64) -> f64 {
    let (hbar, kt) = (1.054_571_8e-34, 1.380_649e-23 * 293.15);
    let w_e = if n < 1.85 { 1.96e16 } else { 0.815e16 };
    let eps = n * n;
    0.75 * kt * ((eps - 1.0) / (eps + 1.0)).powi(2) + 3.0 * hbar * w_e / (16.0 * std::f64::consts::SQRT_2) * (n * n - 1.0).powi(2) / (n * n + 1.0).powf(1.5)
}

/// The standard normal's cumulative distribution (Abramowitz & Stegun
/// 7.1.26 erf, error under 1.5×10⁻⁷).
fn normal_cdf(x: f64) -> f64 {
    let z = x / std::f64::consts::SQRT_2;
    let t = 1.0 / (1.0 + 0.327_591_1 * z.abs());
    let y = 1.0 - (((((1.061_405_429 * t - 1.453_152_027) * t) + 1.421_413_741) * t - 0.284_496_736) * t + 0.254_829_592) * t * (-z * z).exp();
    0.5 * (1.0 + if z >= 0.0 { y } else { -y })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::climate::{Climate, STUDIO};

    /// Charcoal's lifts follow from this law: vine's splinters (R 1.5 µm,
    /// carbon) are taken 0.85 (`charcoal::VINE_LIFT`), compressed's
    /// sub-0.3 µm fines 0.10, which with half crumbs gives
    /// `charcoal::COMPRESSED_LIFT` (charcoal_optics.py §3).
    #[test]
    fn charcoal_follows_the_law() {
        let carbon = Particle { hamaker: CARBON_A, wets: 1.0 };
        let vine = taken(1.5e-6, carbon, STUDIO);
        let fines = taken(0.1e-6, carbon, STUDIO);
        assert!((vine - crate::charcoal::VINE_LIFT as f64).abs() < 0.005, "{vine}");
        assert!((fines - 0.10).abs() < 0.005, "{fines}");
        assert!((0.5 * fines + 0.5 * vine - crate::charcoal::COMPRESSED_LIFT as f64).abs() < 0.01);
    }

    /// The water bridge forms in any studio: the Kelvin diameter at 50 % RH
    /// is 1.53 nm, wider than the 0.165 nm contact (pastel_lift.py §1). In
    /// a bone-dry room there is none, and the eraser takes more.
    #[test]
    fn the_studio_condenses_water_at_the_contact() {
        assert!((kelvin_diameter(STUDIO) - 1.53e-9).abs() < 0.01e-9, "{}", kelvin_diameter(STUDIO));
        let dry = Climate { rh: 0.001, ..STUDIO };
        assert!(kelvin_diameter(dry) < D0);
        let whiting = Particle { hamaker: WHITING_A, wets: 1.0 };
        assert!(taken(1.0e-6, whiting, dry) > taken(1.0e-6, whiting, STUDIO) + 0.1);
    }

    /// Whiting (d₃₂ 1.7 µm, σ_ln 0.905) is taken 0.61 at the studio's
    /// humidity: 0.608 in pastel_lift.py §3.
    #[test]
    fn whiting_as_the_script_has_it() {
        let w = taken_of_sizes(1.7, 0.905, Particle { hamaker: WHITING_A, wets: 1.0 }, STUDIO);
        assert!((w - 0.608).abs() < 0.002, "{w}");
    }

    /// Tabor–Winterton by class against the measured constants (pastel_lift.py §2).
    #[test]
    fn hamaker_from_the_index() {
        for (n, want) in [(1.586, 9.6e-20), (2.464, 15.7e-20), (1.91, 8.1e-20), (1.448, 6.1e-20), (3.0, 23.3e-20)] {
            assert!((hamaker_of_index(n) - want).abs() < 0.06e-20, "{n}: {}", hamaker_of_index(n));
        }
    }

    #[test]
    fn the_normal_cdf() {
        assert!((normal_cdf(0.0) - 0.5).abs() < 1e-9);
        assert!((normal_cdf(1.0) - 0.841_344_746).abs() < 2e-7);
        assert!((normal_cdf(-1.96) - 0.024_997_895).abs() < 2e-7);
    }
}
