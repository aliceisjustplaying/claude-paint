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

/// A bed of carbon particles in air, the way a charcoal stick lays it:
/// Kubelka–Munk absorption K = 2σa and scattering S = 0.75σs(1−g) per unit
/// particle volume (µm⁻¹), per channel (R, G, B at 610, 550, 460 nm), from
/// Mie, size-averaged as `mie_air`'s table, of carbon's measured complex
/// index. Computed in notes/research/charcoal_optics.py (its "for dry.rs"
/// lines); charcoal.md §2. Carbon absorbs within a tenth of a micrometre, so
/// what it scatters is what its facets reflect (about 9 % against air, 5 %
/// against resin): the bed is dark whatever its particles' size. That
/// reflection is the bed's first surface: no veil goes over it
/// (`Cell::bare_carbon`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CarbonBed {
    pub k_air: Rgb,
    pub s_air: Rgb,
    /// With a resin of n 1.49 (a fixative's, poly(n-butyl methacrylate)) in
    /// its pores and around it.
    pub k_resin: Rgb,
    pub s_resin: Rgb,
}

impl CarbonBed {
    /// R∞ (linear RGB) with a share `wet` (0..1) of its pores filled by
    /// resin: the two media mixed by share.
    pub fn color(&self, wet: f32) -> Rgb {
        let w = wet.clamp(0.0, 1.0);
        std::array::from_fn(|q| {
            let k = (1.0 - w) * self.k_air[q] + w * self.k_resin[q];
            let s = (1.0 - w) * self.s_air[q] + w * self.s_resin[q];
            let ks = k / s;
            1.0 + ks - (ks * ks + 2.0 * ks).sqrt()
        })
    }
}

/// Vine (willow) charcoal: splinters of charred cell wall, median 3 µm, of
/// cellulose fired at 500 °C (n, k from Jäger, Mutschke & Henning 1998,
/// halfway between their 400 and 600 °C chars; the firing from FAO 1987).
pub const VINE_CHARCOAL: CarbonBed = CarbonBed {
    k_air: [0.55891, 0.54904, 0.53309],
    s_air: [0.024504, 0.024225, 0.0234],
    k_resin: [0.52805, 0.51231, 0.48283],
    s_resin: [0.01538, 0.017648, 0.022447],
};

/// Compressed charcoal: charcoal and lamp black pressed with a gum binder
/// (5 % of the solids, n 1.52), 60 % solids, laid as crumbs (median 3 µm).
/// Where the carbon is soot of tens of nanometres the crumb is a fine
/// compact whose pores are finer than light: an effective medium
/// (Bruggeman) of carbon, gum and air, its index (1.22 + 0.34i) nearer
/// air's, R∞ 0.010. Where it is ground charcoal of 1–10 µm it is a bed of
/// opaque splinters, as vine's (0.021). Half of each (the soot's share is
/// unmeasured; estimates-audit-2 C5, bounds 0.010–0.021), mixed by
/// Kubelka–Munk: R∞ 0.016.
pub const COMPRESSED_CHARCOAL: CarbonBed = CarbonBed {
    k_air: [0.57422, 0.56463, 0.54939],
    s_air: [0.018359, 0.018013, 0.01718],
    k_resin: [0.53992, 0.52662, 0.50192],
    s_resin: [0.011199, 0.012773, 0.016114],
};

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

    /// Charcoal's R∞ as charcoal_optics.py prints it: under the measured
    /// ceiling for charcoal powder (below 5 % in the visible, arXiv
    /// 2110.05832), and seen as it is, with no veil; compressed darker than
    /// vine, between its fine compact's 0.010 and the splinters' 0.021; a
    /// fixative's resin darkens it, and only a little; the K/S scaling the
    /// tooth's fixative uses for carbon (`crate::charcoal::WET_KS`) is
    /// these beds'.
    #[test]
    fn charcoal_is_dark_by_its_facets() {
        let (v, k) = (VINE_CHARCOAL.color(0.0), COMPRESSED_CHARCOAL.color(0.0));
        for (got, want) in v.iter().zip([0.0210, 0.0211, 0.0210]).chain(k.iter().zip([0.0155, 0.0155, 0.0152])) {
            assert!((got - want).abs() < 2e-4, "{v:?} {k:?}");
        }
        assert!(v[1] < 0.05, "{v:?}");
        assert!(k.iter().zip(&v).all(|(k, v)| k < v && *k > 0.010), "{k:?} {v:?}");
        let fixed = VINE_CHARCOAL.color(1.0);
        assert!(fixed[1] < v[1] && fixed[1] > 0.7 * v[1], "{fixed:?}");
        for bed in [VINE_CHARCOAL, COMPRESSED_CHARCOAL] {
            for q in 0..3 {
                let g = (bed.k_resin[q] / bed.s_resin[q]) / (bed.k_air[q] / bed.s_air[q]);
                assert!((g - crate::charcoal::WET_KS[q]).abs() < 0.02, "{q}: {g}");
            }
        }
    }
}
