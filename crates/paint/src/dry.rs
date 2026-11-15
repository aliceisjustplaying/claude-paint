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

/// The indices of the media `scatter_in` is tabled for: air to water.
const MEDIA: [f32; 10] = [1.0, 1.01, 1.02, 1.05, 1.1, 1.15, 1.2, 1.25, 1.3, 1.333];

/// A particle's transport scattering with its pores full of a medium of
/// index `n_med` (1 air .. 1.333 water), as a share of its scattering in
/// air: Mie, size-averaged as `mie_air` (d = 1 µm, λ 550 nm), its contrast
/// m = n/n_med and its size parameter π d n_med/λ both moving with the
/// medium (notes/research/dry_pigment_mie.py `media`; wet_pastel.md §2).
/// Interpolated in both; at most 1 (the table's Mie ripples aside).
pub fn scatter_in(n: f32, n_med: f32) -> f32 {
    // (n, the share at each of MEDIA)
    const T: [(f32, [f32; 10]); 12] = [
        (1.50, [1.0, 0.9812, 0.9588, 0.8910, 0.7588, 0.6294, 0.4854, 0.3403, 0.2080, 0.1371]),
        (1.56, [1.0, 0.9728, 0.9549, 0.8924, 0.8251, 0.7285, 0.6022, 0.4608, 0.3269, 0.2445]),
        (1.65, [1.0, 0.9862, 0.9779, 0.9507, 0.9199, 0.7980, 0.6900, 0.6144, 0.5118, 0.4345]),
        (1.70, [1.0, 0.9976, 0.9683, 0.8901, 0.8105, 0.7859, 0.7171, 0.6447, 0.5438, 0.4713]),
        (1.74, [1.0, 0.9942, 0.9904, 0.9944, 0.9629, 0.8592, 0.7955, 0.7054, 0.6185, 0.5616]),
        (1.84, [1.0, 1.0108, 0.9542, 0.9171, 0.8745, 0.7975, 0.7857, 0.7183, 0.6860, 0.6437]),
        (2.00, [1.0, 0.9778, 0.9613, 0.9156, 0.8688, 0.8274, 0.7859, 0.7325, 0.6949, 0.6926]),
        (2.15, [1.0, 1.0025, 0.9830, 0.9659, 0.9167, 0.8353, 0.8087, 0.7639, 0.7155, 0.7081]),
        (2.30, [1.0, 0.9889, 0.9776, 0.9517, 0.9447, 0.9679, 0.8982, 0.8493, 0.7760, 0.7758]),
        (2.40, [1.0, 0.9844, 0.9620, 0.9278, 0.8714, 0.7956, 0.7584, 0.7329, 0.7020, 0.6963]),
        (2.45, [1.0, 0.9868, 0.9773, 0.9565, 0.9769, 0.9225, 0.8437, 0.8060, 0.7945, 0.7785]),
        (3.00, [1.0, 1.0055, 0.9979, 0.9696, 0.9510, 0.9312, 0.9215, 0.8984, 0.8783, 0.8361]),
    ];
    let m = n_med.clamp(MEDIA[0], MEDIA[9]);
    let j = MEDIA.windows(2).position(|w| m <= w[1]).unwrap_or(8);
    let v = (m - MEDIA[j]) / (MEDIA[j + 1] - MEDIA[j]);
    let at = |row: &[f32; 10]| row[j] + (row[j + 1] - row[j]) * v;
    let n = n.clamp(T[0].0, T[11].0);
    let i = T.windows(2).position(|w| n <= w[1].0).unwrap_or(10);
    let u = (n - T[i].0) / (T[i + 1].0 - T[i].0);
    (at(&T[i].1) + (at(&T[i + 1].1) - at(&T[i].1)) * u).min(1.0)
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

/// Dry color retaining measured absorption and wavelength-dependent
/// scattering. The air/oil rule is the same as `dry_color`.
pub fn spectral_color(parts: &[(crate::spectral::SpectralPigment, f32, f32)], white: &crate::spectral::Spectrum) -> Rgb {
    let total = parts.iter().map(|p| p.2).sum::<f32>().max(1e-9);
    let (sigma_white, r_white) = mie_air(2.0);
    let reflectance = std::array::from_fn(|w| {
        let (mut k, mut s) = (0.0, 0.0);
        for (pigment, n, c) in parts {
            let s_dry = if *n < 0.0 {
                -n * pigment.s[w]
            } else {
                let (sigma, ratio) = mie_air(*n);
                if *n >= 1.9 { ratio * pigment.s[w] } else { r_white * white[w] * sigma / sigma_white }
            };
            k += c / total * pigment.k[w];
            s += c / total * s_dry;
        }
        crate::spectral::km(k / s.max(1e-9))
    });
    crate::spectral::to_rgb(&reflectance)
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
