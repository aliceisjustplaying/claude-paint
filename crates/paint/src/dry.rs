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
/// (`Palette::dry_earths`), for the earths LBNL measured no film of
/// (`lbnl_film` has the rest). Their oil tables fix K/S in oil, not K per
/// volume: each one's K comes from a Kremer gum-arabic chart swatch read as
/// an assumed 100 µm oil film, and its S from an estimated hiding
/// (scripts/pigment_data/researched-tubes.json), so a dry stick of them
/// came out as pale as a tint. Dry, each takes its K times this
/// factor over what its grains scatter in air (`earth_scatter`), and the
/// factor is fitted so the pure stick matches the same pigment's powder
/// (Cosentino 2014 FORS powders, corrected for the integrating sphere's floor
/// and gain, the floor the measured 0.022: notes/research/dry_pigment_optics.md
/// §7, §9, dry_earth_fit.py). Red earth and yellow ochre are natural ochres,
/// and LBNL measured only synthetic iron oxides; viridian (hydrated PG 18) it
/// didn't measure (§10). None for any other tube.
pub fn powder_k(name: &str) -> Option<f32> {
    match name {
        "red earth" => Some(3.600),
        "yellow ochre" => Some(3.137),
        "viridian" => Some(4.298),
        _ => None,
    }
}

/// The grains of an earth without a film of its own (`powder_k`), with the
/// dry-earths and oil-earths switches: what they scatter per pigment volume in
/// a binder of fresh oil's index, and Mie's air/oil ratio of S at their
/// measured size. Red earth takes burnt sienna's measured S, yellow ochre raw
/// sienna's (µm⁻¹, the 550–700 nm means of LBNL's films, `lbnl_film`; both are
/// iron oxide in clay, hematite and goethite), and the ratio at the measured
/// size of Kremer's earths by laser diffraction (burnt umber 40710, 6.4 µm,
/// 0.2–42 µm; yellow ochre 116421, 10.5 µm, 0.2–76 µm: Reichert et al. 2025,
/// Anal. Bioanal. Chem. 417:4351; dry_pigment_optics.md §9, earth_scatter_lbnl.py).
fn earth_grains(name: &str) -> Option<(f32, f32)> {
    match name {
        "red earth" => Some((0.1845, 1.551)),
        "yellow ochre" => Some((0.3817, 2.139)),
        _ => None,
    }
}

/// The measured films (the LBNL Pigment Database: Levinson, Berdahl & Akbari,
/// Sol. Energy Mater. Sol. Cells 89 (2005) 319–349, 351–389): K and S per
/// pigment volume, µm⁻¹, at `spectral::wavelength` (380–750 nm), of Liquitex
/// acrylic films (binder n 1.49, fresh linseed oil 1.48) of burnt sienna B13
/// (calcined natural iron oxide, PBr 7), raw sienna B14 and raw umber B15
/// (natural, PBr 7), Mars red R04 (synthetic red iron oxide, PR 101) and Mars
/// yellow Y01 (synthetic, PY 42): each film's K and S, inverted from its
/// reflectance over a void, a white and a black ground with its measured
/// thickness, over its pigment volume concentration (13.4, 20.1, 10.9, 13, 16
/// %), read back from the database's vector charts (notes/research/earth_lbnl_ks.py,
/// which checks each against its own film over the two grounds, rms ≤ 0.006).
/// Raw sienna's chart starts at 445 nm, and its values below are held there.
/// Then Mie's air/oil ratio of S at the grains' size: the earths' measured (as
/// `earth_grains`, Kremer's burnt umber 40710; Reichert et al. 2025), the Mars
/// colours' the dry rule's at 1 µm (`mie_air`; no size measured).
#[allow(clippy::type_complexity)]
const LBNL_FILMS: [(&str, [f32; crate::spectral::N], [f32; crate::spectral::N], f32); 5] = [
    ("burnt sienna", [2.3725, 2.3355, 2.285, 2.2296, 2.1587, 2.0725, 1.9832, 1.8831, 1.7984, 1.7282, 1.6667, 1.6063, 1.537, 1.477, 1.4105, 1.3378, 1.2479, 1.1371, 0.99808, 0.83515, 0.62478, 0.47677, 0.37436, 0.30745, 0.2604, 0.22725, 0.20205, 0.18144, 0.16387, 0.14789, 0.13344, 0.12031, 0.10791, 0.098068, 0.089005, 0.082685, 0.077586, 0.074968],
        [0.063798, 0.063623, 0.063361, 0.063251, 0.061671, 0.065085, 0.064828, 0.064322, 0.06426, 0.063879, 0.063753, 0.064479, 0.068712, 0.070606, 0.076069, 0.083739, 0.097381, 0.11764, 0.15147, 0.19579, 0.21573, 0.22677, 0.22835, 0.22326, 0.21497, 0.20569, 0.19624, 0.18616, 0.17692, 0.16733, 0.15762, 0.14892, 0.13946, 0.13149, 0.12326, 0.11588, 0.1082, 0.10373], 1.669),
    ("raw sienna", [1.7579, 1.7579, 1.7579, 1.7579, 1.7579, 1.7579, 1.7579, 1.583, 1.4956, 1.5698, 1.5589, 1.8778, 1.4041, 1.3415, 1.1365, 0.88697, 0.70538, 0.56313, 0.44212, 0.3434, 0.26695, 0.21605, 0.18517, 0.16773, 0.15786, 0.1512, 0.14511, 0.13902, 0.13186, 0.12336, 0.11462, 0.1047, 0.094836, 0.085253, 0.076619, 0.069708, 0.064456, 0.061252],
        [0.2625, 0.2625, 0.2625, 0.2625, 0.2625, 0.2625, 0.2625, 0.24763, 0.23859, 0.25221, 0.26252, 0.34624, 0.30259, 0.35072, 0.36824, 0.36566, 0.38083, 0.39489, 0.40607, 0.41695, 0.42322, 0.42322, 0.41653, 0.40605, 0.39528, 0.38458, 0.37344, 0.36389, 0.35475, 0.34671, 0.34031, 0.33328, 0.32746, 0.32164, 0.3154, 0.30948, 0.30228, 0.29729], 2.349),
    ("raw umber", [3.2085, 2.9753, 2.7423, 2.5505, 2.363, 2.152, 1.9379, 1.7664, 1.6526, 1.5645, 1.4824, 1.3854, 1.2772, 1.1688, 1.0618, 0.96084, 0.86807, 0.78939, 0.72386, 0.67061, 0.61503, 0.56979, 0.53302, 0.50013, 0.4702, 0.44248, 0.41682, 0.3911, 0.36951, 0.34946, 0.33113, 0.3138, 0.29792, 0.2826, 0.26673, 0.25426, 0.24069, 0.22964],
        [0.044438, 0.044262, 0.042649, 0.041104, 0.040542, 0.041965, 0.0425, 0.042044, 0.040627, 0.036766, 0.036265, 0.037423, 0.043816, 0.050245, 0.06136, 0.072353, 0.084746, 0.096163, 0.10488, 0.11211, 0.11026, 0.1078, 0.10599, 0.10455, 0.1029, 0.1012, 0.099335, 0.097086, 0.094645, 0.091683, 0.089553, 0.087004, 0.084071, 0.081349, 0.078301, 0.076141, 0.073287, 0.070957], 1.936),
    ("Mars red", [3.2495, 3.2368, 3.3185, 3.3571, 3.282, 3.354, 3.4407, 3.4103, 3.3343, 3.311, 3.2948, 3.3365, 3.1957, 3.224, 3.2688, 3.2368, 3.0776, 2.8848, 2.6215, 1.9507, 1.2174, 0.76445, 0.51863, 0.40441, 0.34702, 0.31557, 0.29064, 0.26574, 0.24001, 0.21277, 0.18406, 0.15727, 0.13236, 0.11098, 0.094046, 0.08314, 0.077029, 0.076343],
        [0.14149, 0.14586, 0.15044, 0.15446, 0.15114, 0.15507, 0.15908, 0.16003, 0.15753, 0.16454, 0.1673, 0.1774, 0.18466, 0.1986, 0.22296, 0.2515, 0.29989, 0.3945, 0.59542, 0.8332, 1.0147, 1.1257, 1.1529, 1.1327, 1.0896, 1.0467, 1.0065, 0.971, 0.93644, 0.90615, 0.87818, 0.85151, 0.82766, 0.80253, 0.77928, 0.7541, 0.73299, 0.70901], 1.454),
    ("Mars yellow", [2.4035, 2.3225, 2.2597, 2.2084, 2.08, 1.9118, 1.7149, 1.522, 1.4393, 1.3762, 1.2913, 1.1536, 0.95294, 0.75175, 0.55416, 0.37652, 0.23585, 0.14541, 0.093062, 0.067886, 0.057021, 0.053558, 0.054038, 0.055814, 0.05779, 0.058988, 0.059395, 0.05887, 0.057167, 0.054715, 0.05099, 0.046759, 0.042354, 0.037694, 0.032993, 0.029427, 0.026654, 0.024764],
        [0.16746, 0.17883, 0.19921, 0.22464, 0.25535, 0.31526, 0.40122, 0.43817, 0.43774, 0.43815, 0.43859, 0.45416, 0.48213, 0.51531, 0.53694, 0.55135, 0.55525, 0.54824, 0.53213, 0.51269, 0.49365, 0.47515, 0.45751, 0.44166, 0.42654, 0.41338, 0.40101, 0.3896, 0.37831, 0.36791, 0.35866, 0.34881, 0.34137, 0.33266, 0.32557, 0.3157, 0.30867, 0.30002], 1.666),
];

/// A tube whose pigment LBNL measured as a film (`LBNL_FILMS`), with the
/// dry-earths and oil-earths switches: its K and S per pigment volume in oil
/// (µm⁻¹), measured together, so its absorption's shape and its scattering's
/// are its own, not a thin swatch's K times one factor over lead white's
/// shape of S (dry_pigment_optics.md §10); and the air/oil ratio its S takes
/// dry. None for any other tube.
pub fn lbnl_film(name: &str) -> Option<(&'static [f32; crate::spectral::N], &'static [f32; crate::spectral::N], f32)> {
    LBNL_FILMS.iter().find(|f| f.0 == name).map(|f| (&f.1, &f.2, f.3))
}

/// Lead white's S per pigment volume in oil, µm⁻¹, the scale every earth's
/// measured S and K enter the engine on (its own S is calibrated, per coat):
/// the dry rule's ¾ σ′ in oil at 1 µm (`mie_air`). A measurement-anchored
/// route brackets it (white lead's tinting strength 1/18.5 of rutile's by
/// weight, over LBNL's measured rutile: 0.25–0.38; dry_pigment_optics.md §10).
pub fn lead_white_scatter() -> f32 {
    let (sigma_white, r_white) = mie_air(2.0);
    0.75 * sigma_white / r_white
}

/// What tube `name`'s grains scatter, with the dry-earths and oil-earths
/// switches, as multiples of lead white's oil S: (in oil, dry). An earth's is
/// its measured S per volume (`earth_grains`) over lead white's on the dry
/// rule's scale (¾ σ′ in oil at 1 µm, `mie_air`); dry, times Mie's air/oil
/// ratio at the grains' size. Viridian, with nothing measured, keeps the
/// rule's: Mie at 1 µm at its index. None for a tube without `powder_k`
/// (the measured films, `lbnl_film`, carry their own S).
pub fn earth_scatter(name: &str) -> Option<(f32, f32)> {
    powder_k(name)?;
    let (sigma_white, r_white) = mie_air(2.0);
    let white = lead_white_scatter();
    if let Some((s, ratio)) = earth_grains(name) {
        return Some((s / white, s / white * ratio));
    }
    let (sigma, ratio) = mie_air(refractive_index(name));
    let dry = r_white * sigma / sigma_white;
    Some((dry / ratio, dry))
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
