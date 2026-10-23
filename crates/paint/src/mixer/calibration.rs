use super::*;
use crate::color::to_oklab;

#[test]
fn measured_dark_recipes_include_black_and_match_the_source() {
    let mut palette = crate::Palette::new("dark calibration", crate::palette::catalog());
    palette.engine = crate::PIGMENT_MIXER;
    palette.glaze_film = true;
    let at = |name| palette.tubes.iter().position(|t| t.name == name).unwrap();
    let spectra = include_str!(
        "../../../../scripts/pigment_data/revigo/revigo-aggregated-palette-spectral-data.txt"
    );
    for (tube, identity, sid, mass_share) in [
        ("ultramarine blue", "UltramarineBlue", "11E_ow", 0.901),
        ("emerald green", "EmeraldGreen", "10D_ow", 0.99),
        ("carmine lake", "Cochineal", "04E_ow", 0.895),
    ] {
        let volume = mass_share / measured_density9(identity).unwrap();
        let black = (1.0 - mass_share) / measured_density9("BoneBlack").unwrap();
        let pile = palette.pile(vec![
            (at(tube), volume / (volume + black)),
            (at("bone black"), black / (volume + black)),
        ]);
        let measured: Vec<f32> = spectra
            .lines()
            .find(|line| line.starts_with(&format!("{sid}\t")))
            .unwrap()
            .split('\t')
            .skip(1)
            .map(|s| s.parse::<f32>().unwrap() / 100.0 - 0.04)
            .collect();
        let reflectance = std::array::from_fn(|w| measured[w.saturating_sub(2).min(30)]);
        let target = to_oklab(crate::spectral::to_rgb(&reflectance).map(|v| v.clamp(0.0, 1.0)));
        let reference = std::array::from_fn(|w| {
            outside(f32::from_le_bytes(
                REFERENCE[w * 4..w * 4 + 4].try_into().unwrap(),
            ))
        });
        for actual in [
            pile.color,
            pile.paint(0.0)
                .over(crate::spectral::to_rgb(&reference), 4.0),
        ] {
            let actual = to_oklab(actual);
            let delta = (0..3)
                .map(|k| (actual[k] - target[k]).powi(2))
                .sum::<f32>()
                .sqrt();
            assert!(delta < 0.03, "{sid}: dark-mixture error {delta}");
        }
    }
}

#[test]
fn source_mass_and_palette_volumes_describe_the_same_13g_film() {
    let mut palette = crate::Palette::new("mass-volume calibration", crate::palette::catalog());
    palette.engine = crate::PIGMENT_MIXER;
    palette.glaze_film = true;
    let at = |name| palette.tubes.iter().position(|t| t.name == name).unwrap();
    let masses = [0.989, 0.011];
    let densities = [
        measured_density9("ZincWhite").unwrap(),
        measured_density9("PrussianBlue").unwrap(),
    ];
    let volumes = [masses[0] / densities[0], masses[1] / densities[1]];
    let total = volumes.iter().sum::<f32>();
    let blue_share = volumes[1] / total;
    assert!((blue_share - masses[1]).abs() > 0.001);
    let by_volume = palette
        .pile(vec![
            (at("zinc white"), volumes[0] / total),
            (at("Prussian blue"), blue_share),
        ])
        .paint(0.0);
    let pigments = [
        pigment9(&measured("ZincWhite").unwrap()),
        pigment9(&measured("PrussianBlue").unwrap()),
    ];
    let mass_k: crate::spectral::Spectrum = std::array::from_fn(|w| {
        (0..2)
            .map(|i| masses[i] * pigments[i].k[w] / densities[i])
            .sum::<f32>()
            / total
    });
    let mass_s: crate::spectral::Spectrum = std::array::from_fn(|w| {
        (0..2)
            .map(|i| masses[i] * pigments[i].s[w] / densities[i])
            .sum::<f32>()
            / total
    });
    let backing = crate::spectral::from_rgb([0.85; 3]).map(inside);
    let expected = crate::spectral::to_rgb(&std::array::from_fn(|w| {
        outside(film(mass_k[w], mass_s[w], backing[w]))
    }))
    .map(|v| v.clamp(0.0, 1.0));
    let actual = by_volume.over([0.85; 3], 4.0);
    assert!(
        (0..3).all(|k| (actual[k] - expected[k]).abs() < 2e-6),
        "13G mass {expected:?}, volume {actual:?}"
    );
}
