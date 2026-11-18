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

/// The factor on an earth's (or viridian's) engine-9 absorption K per
/// volume that its measured dry powder shows, with the dry-earths switch
/// (`Palette::dry_earths`). Their oil tables fix K/S in oil, not K per
/// volume: each one's K comes from a Kremer gum-arabic chart swatch read as
/// an assumed 100 µm oil film, and its S from an estimated hiding
/// (scripts/pigment_data/researched-tubes.json), so a dry stick of them
/// came out as pale as a tint (pure burnt sienna sRGB 198 132 111, L* 61,
/// where the powder is 115 62 50, L* 33). Dry, each takes its K times this
/// factor over the Mie scattering of its particles in air at its index
/// (`rule_scatter`), and the factor is fitted so the pure stick matches the
/// same pigment's powder (Cosentino 2014 FORS powders, corrected for the
/// integrating sphere's floor and gain as for the lakes:
/// notes/research/dry_pigment_optics.md §7, dry_earth_fit.py). The factors
/// agree with the iron oxides' absorption (§7.3). None for any other tube.
pub fn powder_k(name: &str) -> Option<f32> {
    match name {
        "burnt sienna" => Some(31.71),
        "red earth" => Some(7.718),
        "raw sienna" => Some(10.08),
        "yellow ochre" => Some(2.087),
        "raw umber" => Some(16.49),
        "viridian" => Some(3.910),
        _ => None,
    }
}

/// A black's grains in air, with the dry-earths switch (`Palette::dry_earths`):
/// the S/K their bed has, from Mie of their complex index (dry_pigment_optics.md
/// §8.1, dry_colours_fit.py). Carbon absorbs within a tenth of a micrometre, so a
/// carbon-bearing grain is opaque and all it scatters is what its facets reflect:
/// its bed's K/S is the grain's, whatever its size, not the oil table's S times
/// `carbon_ratio` (which left bone black's dry stick at R∞ 10⁻⁴, its calibrated oil
/// S being near nothing, and vine black's at 0.20 from a masstone). Vine black:
/// charred wood (Jäger, Mutschke & Henning 1998, the mean of their cellulose chars
/// at 400 and 600 °C), 3 µm grains: R∞ 0.021. Bone black: hydroxyapatite with 10
/// wt % carbon spread through it (an effective medium, Bruggeman), 7 µm grains
/// (3–15 µm): R∞ 0.013. Dry, the black keeps its K (so its tints keep their
/// strength) and takes S = K × this. None for any other tube.
pub fn carbon_bed(name: &str) -> Option<f32> {
    match name {
        "vine black" => Some(0.04412),
        "bone black" => Some(0.02758),
        _ => None,
    }
}

/// The dry rule's scattering per coat of a particle of index `n` (1 µm),
/// spectrally: lead white's oil S `white` times its Mie air/oil ratio (its
/// dry S), times the Mie σ′(n)/σ′(2.0) (`mie_air`). It is what the rule
/// gives any pigment under n 1.9; with the dry-earths switch it is also an
/// earth's (`powder_k`).
pub fn rule_scatter(n: f32, white: &crate::spectral::Spectrum) -> crate::spectral::Spectrum {
    let (sigma_white, r_white) = mie_air(2.0);
    let (sigma, _) = mie_air(n);
    std::array::from_fn(|w| r_white * white[w] * sigma / sigma_white)
}

/// What a pigment's grains of index `n` (1 µm) scatter in fresh oil, as a
/// multiple of lead white's oil S: the dry rule's Mie S (`rule_scatter`) over
/// Mie's air/oil ratio at `n`. The oil-earths slots' S (`crate::mixer::OIL_EARTHS`).
pub fn oil_scatter(n: f32) -> f32 {
    let (sigma_white, r_white) = mie_air(2.0);
    let (sigma, ratio) = mie_air(n);
    r_white * sigma / sigma_white / ratio
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
