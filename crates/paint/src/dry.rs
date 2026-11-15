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

/// The refractive index of a dye lake's substrate, alumina hydrate
/// (gibbsite, 1.57; notes/research/dry_pigment_optics.md §6). With the
/// dry-lakes switch (`Palette::dry_lakes`) a lake scatters dry as its
/// substrate does, by the same rule as any pigment under 1.9.
pub const LAKE_SUBSTRATE_INDEX: f32 = 1.57;

/// A dye lake's absorption dry, in a pastel stick or a sheet's pulp, over
/// its absorption in the engine-9 oil tables, with the dry-lakes switch
/// (`Palette::dry_lakes`). The oil tables fix a lake's K/S in oil but not
/// its absorption per volume of pigment (the researched tubes' K is set by
/// an assumed film, the proxies' by a catalog masstone), and a lake's dry
/// color depends on exactly that: its dye against its substrate's
/// scattering in air. Each loading is fitted so the pure lake's dry color
/// matches measured powder (Cosentino 2014 FORS powders, corrected for the
/// integrating sphere's floor and gain, set by a carbon black powder's total
/// reflectance and rutile's: dry_pigment_optics.md §6, dry_lake_fit.py).
/// The loadings agree with dye chemistry: madder's K/S needs 2–9 wt% dye on
/// its alumina, as lakes carry. Magenta (fuchsine on alumina) has no
/// measured powder, so its loading comes from its dye: fuchsine's measured
/// molar absorptivity (8.06×10⁴ L mol⁻¹ cm⁻¹ at 543 nm, PhotochemCAD) is so
/// high that any lake of 3–20 wt% dye is held at the 1 µm grain's package
/// limit, K/S 5.7 at 543 nm (4.9–5.8; dry_pigment_optics.md §6.3), and the
/// loading gives the pure lake that K/S. None for a tube that isn't a dye
/// lake.
pub fn dye_loading(name: &str) -> Option<f32> {
    match name {
        "rose madder" => Some(35.7),
        "carmine lake" => Some(3.01),
        "yellow lake" => Some(12.2),
        "magenta" => Some(MAGENTA_LOADING),
        _ => None,
    }
}

/// Magenta's dye loading: K/S 5.7 at 543 nm over the engine-9 K/S there
/// with no loading (dry_lake_fit.py).
const MAGENTA_LOADING: f32 = 6.26;

/// The measured refractive index of a pigment that `refractive_index` has
/// wrong, given with the dry-lakes switch (`Palette::dry_lakes`; the table
/// stays for old paintings): permanent alizarin is quinacridone violet
/// PV19, 2.02–2.04, not a lake; green earth (glauconite, celadonite) 1.62;
/// verdigris 1.53/1.56; lead-tin yellow (type I) 2.29/2.31; transparent
/// oxide yellow and Mars yellow are both synthetic goethite, α 2.26, β 2.39,
/// γ 2.40 (the table has Mars yellow at 2.2); Rinmann's green is cobalt in
/// zinc oxide, zincite nω 2.013, nε 2.029 (the table falls back to 1.9)
/// (dry_pigment_optics.md §6.4). None where the table's index stands.
pub fn measured_index(name: &str) -> Option<f32> {
    match name {
        "permanent alizarin" => Some(2.03),
        "green earth" => Some(1.62),
        "copper green" => Some(1.545),
        "lead-tin yellow" => Some(2.30),
        "transparent oxide yellow" | "Mars yellow" => Some(2.35),
        "Rinmann's green" => Some(2.02),
        _ => None,
    }
}

/// The indices of the media `scatter_in` is tabled for: air to water.
const MEDIA: [f32; 10] = [1.0, 1.01, 1.02, 1.05, 1.1, 1.15, 1.2, 1.25, 1.3, 1.333];

/// A particle's transport scattering with its pores full of a medium of
/// index `n_med` (1 air .. 1.333 water), as a share of its scattering in
/// air: Mie, size-averaged (lognormal σ_ln 0.5, λ 550 nm) at a pastel's
/// particle size, whiting's Sauter mean d₃₂ = 1.7 µm (`wet_pastel::
/// PARTICLE_D_UM`), its contrast m = n/n_med and its size parameter
/// π d n_med/λ both moving with the medium (notes/research/
/// dry_pigment_mie.py `media`; wet_pastel.md §2). Interpolated in both; at
/// most 1 (the table's Mie ripples aside). Only the wet pastel switch
/// reads it.
pub fn scatter_in(n: f32, n_med: f32) -> f32 {
    // (n, the share at each of MEDIA)
    const T: [(f32, [f32; 10]); 13] = [
        (1.50, [1.0, 0.9798, 0.9549, 0.8859, 0.8111, 0.7263, 0.5850, 0.4734, 0.3136, 0.2140]),
        (1.56, [1.0, 0.9743, 0.9614, 0.9193, 0.8575, 0.7671, 0.6789, 0.5621, 0.4442, 0.3593]),
        (1.59, [1.0, 0.9873, 0.9717, 0.9530, 0.8839, 0.7858, 0.6996, 0.6107, 0.4989, 0.4139]),
        (1.65, [1.0, 0.9753, 0.9583, 0.9667, 0.8444, 0.7349, 0.6747, 0.6265, 0.5723, 0.4955]),
        (1.70, [1.0, 0.9780, 0.9613, 0.9070, 0.8773, 0.7833, 0.7242, 0.6741, 0.5848, 0.5453]),
        (1.74, [1.0, 0.9704, 0.9461, 0.9155, 0.8497, 0.7933, 0.7152, 0.6408, 0.6024, 0.5797]),
        (1.84, [1.0, 0.9898, 0.9961, 0.9635, 0.8997, 0.8756, 0.7677, 0.7258, 0.6931, 0.6564]),
        (2.00, [1.0, 0.9811, 0.9687, 0.9214, 0.8819, 0.8348, 0.7653, 0.7196, 0.6677, 0.6592]),
        (2.15, [1.0, 0.9717, 0.9712, 0.9355, 0.8813, 0.8280, 0.7945, 0.7648, 0.6926, 0.6639]),
        (2.30, [1.0, 0.9755, 0.9728, 0.9914, 0.9215, 0.8682, 0.8335, 0.7580, 0.7315, 0.7055]),
        (2.40, [1.0, 0.9876, 0.9822, 0.9561, 0.9161, 0.8604, 0.8138, 0.7729, 0.7390, 0.7199]),
        (2.45, [1.0, 0.9954, 0.9839, 0.9676, 0.8801, 0.8438, 0.8263, 0.8198, 0.7664, 0.7662]),
        (3.00, [1.0, 0.9956, 0.9924, 0.9946, 0.9753, 0.9418, 0.9327, 0.8732, 0.8301, 0.8166]),
    ];
    let m = n_med.clamp(MEDIA[0], MEDIA[9]);
    let j = MEDIA.windows(2).position(|w| m <= w[1]).unwrap_or(8);
    let v = (m - MEDIA[j]) / (MEDIA[j + 1] - MEDIA[j]);
    let at = |row: &[f32; 10]| row[j] + (row[j + 1] - row[j]) * v;
    let n = n.clamp(T[0].0, T[12].0);
    let i = T.windows(2).position(|w| n <= w[1].0).unwrap_or(11);
    let u = (n - T[i].0) / (T[i + 1].0 - T[i].0);
    (at(&T[i].1) + (at(&T[i + 1].1) - at(&T[i].1)) * u).min(1.0)
}

/// Whiting (Champagne or Meudon chalk, ground calcite), the filler a pastel
/// tint is let down with: its refractive index, 1.59 (ground calcite; its
/// ω 1.658 and ε 1.486 average 1.60) [M]; its dry color, linear RGB of
/// L* 94, a* 0.6, b* 4.1 (Omya Blanc de Meudon, DIN 6174) [M]; and its
/// transport scattering per volume dry over lead white's, 0.376: Mie,
/// σ'_air 0.331 µm⁻¹ at d₃₂ 1.7 µm (n 1.59) against 0.879 at lead white's
/// 1 µm (n 2.0), the size `mie_air` is tabled at [D].
pub const WHITING_INDEX: f32 = 1.59;
const WHITING: Rgb = [0.892, 0.847, 0.790];
const WHITING_S_REL: f32 = 0.376;

/// A part's dry scattering per volume (as `dry_color`).
fn s_dry(s_oil: f32, n: f32, s_dry_white: f32, sigma_white: f32) -> f32 {
    if n < 0.0 {
        -n * s_oil
    } else {
        let (sigma, ratio) = mie_air(n);
        if n >= 1.9 { ratio * s_oil } else { s_dry_white * sigma / sigma_white }
    }
}

/// The particle index of a pastel stick, for how water in its pores darkens
/// it (`crate::wet_pastel`): its pigments' and its filler's, weighted by
/// scattering × volume (each part scatters in water by its own contrast).
/// A pile's whites stand for the whiting a pastel tint is let down with: a
/// stick whose pile holds a white is its other pigments with the share of
/// whiting that gives the stick's dry color (`dry_color`) its luminance by
/// Kubelka–Munk, whiting at its own scattering and color (`WHITING`).
/// Parts as `dry_color`'s with each one's refractive index and whether it
/// is a white. A pile of whites alone is whiting; one without is its
/// pigments.
pub fn particle_index(parts: &[(Rgb, f32, f32, f32, f32, bool)], s_oil_white: f32) -> f32 {
    let (sigma_white, r_white) = mie_air(2.0);
    let s_dry_white = r_white * s_oil_white;
    let s_w = WHITING_S_REL * s_dry_white;
    let weighted = |list: &mut dyn Iterator<Item = (f32, f32)>| {
        let (mut sn, mut ss) = (0.0f32, 0.0f32);
        for (s, n) in list {
            sn += s * n;
            ss += s;
        }
        if ss > 0.0 { sn / ss } else { WHITING_INDEX }
    };
    let pigments: Vec<_> = parts.iter().filter(|p| !p.5 && p.4 > 0.0).collect();
    let total: f32 = parts.iter().map(|p| p.4.max(0.0)).sum();
    if total <= 0.0 {
        return WHITING_INDEX;
    }
    if pigments.len() == parts.iter().filter(|p| p.4 > 0.0).count() {
        return weighted(&mut parts.iter().map(|p| (p.4.max(0.0) * s_dry(p.1, p.2, s_dry_white, sigma_white), p.3)));
    }
    if pigments.is_empty() {
        return WHITING_INDEX;
    }
    let lum = |r: Rgb| 0.2126 * r[0] + 0.7152 * r[1] + 0.0722 * r[2];
    let target = lum(dry_color(&parts.iter().map(|p| (p.0, p.1, p.2, p.4)).collect::<Vec<_>>(), s_oil_white));
    let vp: f32 = pigments.iter().map(|p| p.4).sum();
    // KM of the pigments and x of whiting per volume of pigment
    let mix = |x: f32| -> f32 {
        let mut r = [0.0f32; 3];
        for (q, o) in r.iter_mut().enumerate() {
            let (mut k, mut s) = (0.0f32, 0.0f32);
            for p in &pigments {
                let m = p.0[q].clamp(1e-4, 0.9999);
                k += p.4 / vp * (1.0 - m) * (1.0 - m) / (2.0 * m) * p.1;
                s += p.4 / vp * s_dry(p.1, p.2, s_dry_white, sigma_white);
            }
            let wq = WHITING[q];
            k += x * (1.0 - wq) * (1.0 - wq) / (2.0 * wq) * s_w;
            s += x * s_w;
            let ks = k / s.max(1e-9);
            *o = 1.0 + ks - (ks * ks + 2.0 * ks).sqrt();
        }
        lum(r)
    };
    // (luminance rises with the whiting: bisect its log)
    let (mut lo, mut hi) = ((1e-4f32).ln(), (1e4f32).ln());
    let x = if mix(hi.exp()) < target {
        hi.exp()
    } else if mix(lo.exp()) > target {
        lo.exp()
    } else {
        for _ in 0..60 {
            let m = 0.5 * (lo + hi);
            if mix(m.exp()) < target { lo = m } else { hi = m }
        }
        (0.5 * (lo + hi)).exp()
    };
    weighted(&mut pigments.iter().map(|p| (p.4 / vp * s_dry(p.1, p.2, s_dry_white, sigma_white), p.3)).chain(std::iter::once((x * s_w, WHITING_INDEX))))
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

/// The dry rule's scattering per coat for a particle of index `n` below 1.9
/// at 1 µm (`spectral_reflectance`): lead white's oil S `white` times its
/// air/oil ratio, times the Mie σ′(n)/σ′(2.0).
pub fn rule_scatter(n: f32, white: &crate::spectral::Spectrum) -> crate::spectral::Spectrum {
    let (sigma_white, r_white) = mie_air(2.0);
    let (sigma, _) = mie_air(n);
    std::array::from_fn(|w| r_white * white[w] * sigma / sigma_white)
}

/// Dry color retaining measured absorption and wavelength-dependent
/// scattering. The air/oil rule is the same as `dry_color`.
pub fn spectral_color(parts: &[(crate::spectral::SpectralPigment, f32, f32)], white: &crate::spectral::Spectrum) -> Rgb {
    crate::spectral::to_rgb(&spectral_reflectance(parts, white))
}

/// The dry reflectance spectrum behind `spectral_color`.
pub fn spectral_reflectance(parts: &[(crate::spectral::SpectralPigment, f32, f32)], white: &crate::spectral::Spectrum) -> crate::spectral::Spectrum {
    let total = parts.iter().map(|p| p.2).sum::<f32>().max(1e-9);
    let (sigma_white, r_white) = mie_air(2.0);
    std::array::from_fn(|w| {
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
    })
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
