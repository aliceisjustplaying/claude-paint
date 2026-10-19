//! How pigments look dry, out of oil: in a pastel stick, or as a toned
//! sheet of paper's pulp (`Palette::dry_color`). Sources:
//! notes/research/dry_pigment_optics.md (and dry_pigment_mie.py, which
//! computes its Mie table).
//!
//! A pigment's absorption is the same in air as in oil; its scattering is
//! not. In oil (n about 1.5) a pigment of index near 1.5 hardly scatters,
//! so it is deep and glassy; in air the contrast is large, so the same
//! particles scatter strongly and look paler and brighter, the more the
//! nearer their index is to oil's. A pigment of high index (vermilion,
//! the cadmiums) scatters well in either and barely changes.

use crate::color::Rgb;

/// A pigment's refractive index (mean of its principal indices), by tube
/// name (notes/research/dry_pigment_optics.md §1); 1.9 if not listed.
pub fn refractive_index(name: &str) -> f32 {
    match name {
        "lead white" => 2.0,
        "zinc white" => 2.02,
        "cobalt blue" => 1.74,
        "ultramarine blue" | "ultramarine ash" => 1.5,
        "cerulean blue" => 1.84,
        "Prussian blue" | "Antwerp blue" => 1.56,
        "emerald green" => 1.75,
        "viridian" => 1.8,
        "chrome yellow" | "lemon chrome" => 2.4,
        "orange chrome" => 2.55,
        "barium yellow" => 1.96,
        "strontium yellow" => 1.97,
        "zinc yellow" => 1.87,
        "pale cadmium" | "cadmium yellow" | "deep cadmium" => 2.5,
        "cadmium red" => 2.6,
        "Naples yellow" => 2.15,
        "Indian yellow" => 1.65,
        "yellow lake" => 1.55,
        "yellow ochre" | "brown ochre" => 2.1,
        "Mars yellow" => 2.2,
        "red earth" | "Mars red" | "Indian red" => 2.7,
        "raw sienna" => 2.0,
        "burnt sienna" | "Mars brown" | "Mars orange" => 2.3,
        "raw umber" => 2.1,
        "vermilion" | "orange vermilion" | "Chinese vermilion" => 3.0,
        "red lead" => 2.42,
        "rose madder" | "carmine lake" | "magenta" | "permanent alizarin" => 1.6,
        "cobalt violet" => 1.68,
        "bone black" | "bone brown" => 1.65,
        "vine black" => 1.9,
        "smalt" | "pale smalt" => 1.5,
        _ => 1.9,
    }
}

/// Transport scattering per unit volume of a 1 µm pigment in air, relative
/// (Mie, size-averaged: the research note's table, d = 1 µm), and the ratio
/// of its scattering dry to in fresh oil.
fn mie_air(n: f32) -> (f32, f32) {
    // (n, σ'_air µm⁻¹, S_air/S_oil) at d = 1 µm
    const T: [(f32, f32, f32); 12] = [
        (1.50, 0.68, 700.0),
        (1.56, 0.71, 43.0),
        (1.65, 0.73, 9.0),
        (1.70, 0.80, 5.8),
        (1.74, 0.75, 3.9),
        (1.84, 0.82, 2.5),
        (2.00, 0.88, 1.9),
        (2.15, 0.91, 1.7),
        (2.30, 0.89, 1.6),
        (2.40, 1.04, 1.8),
        (2.45, 0.95, 1.5),
        (3.00, 1.03, 1.4),
    ];
    if n <= T[0].0 {
        return (T[0].1, T[0].2);
    }
    for w in T.windows(2) {
        if n <= w[1].0 {
            let u = (n - w[0].0) / (w[1].0 - w[0].0);
            // the ratio spans decades: interpolate its log
            return (w[0].1 + (w[1].1 - w[0].1) * u, (w[0].2.ln() + (w[1].2.ln() - w[0].2.ln()) * u).exp());
        }
    }
    (T[11].1, T[11].2)
}

/// The air/oil scattering ratio of a carbon black, whose
/// absorption comes from carbon on (or in) its particles: bone black's
/// apatite matrix ×9 at 1 µm (dry_pigment_optics.md, table 2.2), a carbon
/// (vine) black's ×2.4 (§2.4: "a dry carbon black is a little grayer, never
/// pale"). None for any other tube.
pub fn carbon_ratio(name: &str) -> Option<f32> {
    match name {
        "bone black" | "bone brown" | "ivory black" => Some(9.0),
        "vine black" | "lamp black" => Some(2.4),
        _ => None,
    }
}

/// How pigments look dry, in a pastel stick or a sheet's pulp:
/// two-constant Kubelka–Munk from each tube's masstone in oil (R∞ → K/S),
/// its scattering in oil (`s_oil`, per coat), and its refractive index.
/// Absorption K stays; the scattering rises with the index contrast against
/// air: for n ≥ 1.9 by the Mie ratio over its scattering in oil; below that,
/// where oil all but matches the pigment and its oil value says little, from
/// the dry Mie scattering itself, scaled to lead white's (n = 2: its oil S ×
/// 1.9). Each part is (masstone in oil, scattering in oil, refractive index,
/// share by volume). A negative index is a carbon black's air/oil ratio
/// (`carbon_ratio`), taken as it is.
pub fn dry_color(parts: &[(Rgb, f32, f32, f32)], s_oil_white: f32) -> Rgb {
    let total: f32 = parts.iter().map(|p| p.3).sum::<f32>().max(1e-9);
    let (sigma_white, r_white) = mie_air(2.0);
    let s_dry_white = r_white * s_oil_white;
    let mut out = [0.0f32; 3];
    for (q, o) in out.iter_mut().enumerate() {
        let (mut k, mut s) = (0.0f32, 0.0f32);
        for &(m, s_oil, n, c) in parts {
            let r = m[q].clamp(1e-4, 0.9999);
            let ks = (1.0 - r) * (1.0 - r) / (2.0 * r);
            let s_dry = if n < 0.0 {
                -n * s_oil
            } else {
                let (sigma, ratio) = mie_air(n);
                if n >= 1.9 { ratio * s_oil } else { s_dry_white * sigma / sigma_white }
            };
            k += c / total * ks * s_oil;
            s += c / total * s_dry;
        }
        let ks = k / s.max(1e-9);
        *o = 1.0 + ks - (ks * ks + 2.0 * ks).sqrt();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ultramarine is deep in oil and pale and bright dry; vermilion barely
    /// changes.
    #[test]
    fn dry_pigments_are_paler_by_their_index() {
        let ultra = [0.03f32, 0.04, 0.35];
        let verm = [0.62f32, 0.05, 0.03];
        let du = dry_color(&[(ultra, 0.3, 1.5, 1.0)], 1.0);
        let dv = dry_color(&[(verm, 1.0, 3.0, 1.0)], 1.0);
        assert!(du[2] > ultra[2] + 0.2, "ultramarine dry {du:?}");
        assert!((dv[0] - verm[0]).abs() < 0.15, "vermilion dry {dv:?}");
    }
}
