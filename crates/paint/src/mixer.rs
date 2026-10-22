//! Independent pigment mixing from Sochorová and Jamriška (2021), equations
//! 1–14: measured pigment concentrations and an additive linear RGB residual.
//! The primaries' K/S data are derived from measured reflectance of real oil
//! paints (CC0 Revigo data, `scripts/pigment_data/revigo`); both tables are
//! original. Only the MIT spectral.js D65/CIE conversion from `spectral.rs` is
//! reused.
//! Regenerate with `uv run --script scripts/build_pigment_tables.py`.

use crate::color::{Rgb, linear_to_srgb};

pub const LATENT_SIZE: usize = 21;
const PRIMARIES: usize = 5;
const PIGMENTS: usize = LATENT_SIZE - 3;
const INVERSE_SIZE: usize = 33;
const INVERSE: &[u8; INVERSE_SIZE * INVERSE_SIZE * INVERSE_SIZE * 2 * (PRIMARIES - 1)] = include_bytes!("mixer/inverse.bin");
const COEFFICIENT_BYTES: usize = PIGMENTS * 2 * crate::spectral::N * 4;
const MEASURED: &[u8; COEFFICIENT_BYTES + PIGMENTS * 4] = include_bytes!("mixer/measured.bin");
const REFERENCE: &[u8; crate::spectral::N * 4] = include_bytes!("mixer/reference.bin");
const NAMES: [&str; PIGMENTS] = [
    "CobaltBlue", "ChromeYellowOtero", "Eosin", "BoneBlack", "ZincWhite",
    "BariumSulphate", "CalciumCarbonate", "ChromeYellow", "Cochineal", "EmeraldGreen",
    "Eosin1Wax", "Eosin2PbWax", "LeadWhite", "Minium", "PrussianBlue", "UltramarineBlue",
    "Vermillion", "ViridianGreen",
];

pub fn measured(name: &str) -> Option<[f32; LATENT_SIZE]> {
    let i = NAMES.iter().position(|&n| n == name)?;
    let mut lat = [0.0; LATENT_SIZE];
    lat[i] = 1.0;
    Some(lat)
}

fn tube_pigment(name: &str) -> Option<&'static str> {
    Some(match name {
        "lead white" => "LeadWhite",
        "zinc white" => "ZincWhite",
        "bone black" => "BoneBlack",
        "cobalt blue" => "CobaltBlue",
        "ultramarine blue" => "UltramarineBlue",
        "Prussian blue" => "PrussianBlue",
        "vermilion" | "orange vermilion" | "Chinese vermilion" => "Vermillion",
        "chrome yellow" => "ChromeYellow",
        "lemon chrome" => "ChromeYellowOtero",
        "viridian" => "ViridianGreen",
        "emerald green" => "EmeraldGreen",
        "red lead" => "Minium",
        "carmine lake" => "Cochineal",
        _ => return None,
    })
}

pub fn tube(name: &str) -> Option<[f32; LATENT_SIZE]> {
    measured(tube_pigment(name)?)
}

pub fn tube_strength(name: &str) -> Option<f32> {
    let name = tube_pigment(name)?;
    let p = NAMES.iter().position(|&n| n == name)?;
    let offset = COEFFICIENT_BYTES + p * 4;
    Some(f32::from_le_bytes(MEASURED[offset..offset + 4].try_into().unwrap()))
}

pub fn identity() -> u64 {
    use sha2::{Digest, Sha256};
    static IDENTITY: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    *IDENTITY.get_or_init(|| {
        let mut hash = Sha256::new();
        hash.update(b"Stillwet mixer/latent21/km-linear/measured-ks/finite100um/reference07A/constant-weights/additive-linear-rgb/v5");
        hash.update(MEASURED);
        hash.update(REFERENCE);
        hash.update(INVERSE);
        u64::from_le_bytes(hash.finalize()[..8].try_into().unwrap())
    })
}

pub fn encode(rgb: &Rgb) -> [f32; LATENT_SIZE] {
    let c = inverse(rgb.map(linear_to_srgb));
    let last = (1.0 - c.iter().sum::<f32>()).max(0.0);
    let mut lat = [0.0; LATENT_SIZE];
    lat[..PRIMARIES].copy_from_slice(&[c[0], c[1], c[2], c[3], last]);
    let base = pigment_rgb(&lat);
    for k in 0..3 {
        lat[PIGMENTS + k] = rgb[k] - base[k];
    }
    lat
}

pub fn decode(lat: &[f32; LATENT_SIZE]) -> Rgb {
    let base = pigment_rgb(lat);
    std::array::from_fn(|k| (base[k] + lat[PIGMENTS + k]).clamp(0.0, 1.0))
}

pub fn lerp(a: &Rgb, b: &Rgb, t: f32) -> Rgb {
    let t = t.clamp(0.0, 1.0);
    if t == 0.0 { return *a; }
    if t == 1.0 { return *b; }
    let a = encode(a);
    let b = encode(b);
    decode(&std::array::from_fn(|k| a[k] + (b[k] - a[k]) * t))
}

fn pigment_rgb(lat: &[f32; LATENT_SIZE]) -> Rgb {
    static COEFFICIENTS: std::sync::OnceLock<[[crate::spectral::Spectrum; 2]; PIGMENTS]> = std::sync::OnceLock::new();
    let coefficients = COEFFICIENTS.get_or_init(|| std::array::from_fn(|p| std::array::from_fn(|ks| std::array::from_fn(|w| {
        let offset = ((p * 2 + ks) * crate::spectral::N + w) * 4;
        f32::from_le_bytes(MEASURED[offset..offset + 4].try_into().unwrap())
    }))));
    let mut absorption = [0.0; crate::spectral::N];
    let mut scattering = [0.0; crate::spectral::N];
    let total = lat[..PIGMENTS].iter().map(|c| c.max(0.0)).sum::<f32>().max(1e-12);
    for p in 0..PIGMENTS {
        let c = lat[p].max(0.0) / total;
        if c == 0.0 { continue; }
        for w in 0..crate::spectral::N {
            absorption[w] += c * coefficients[p][0][w];
            scattering[w] += c * coefficients[p][1][w];
        }
    }
    let reflectance = std::array::from_fn(|w| {
        let rg = f32::from_le_bytes(REFERENCE[w * 4..w * 4 + 4].try_into().unwrap());
        let r = film(absorption[w], scattering[w], rg);
        (1.0 - 0.04) * (1.0 - 0.49) * r / (1.0 - 0.49 * r)
    });
    crate::spectral::to_rgb(&reflectance).map(|v| v.clamp(0.0, 1.0))
}

fn film(k: f32, s: f32, rg: f32) -> f32 {
    let tau2 = k * (k + 2.0 * s);
    let tau = tau2.sqrt();
    if tau < 1e-3 {
        let u = 1.0 - tau2 / 3.0;
        return (rg + (s * (1.0 - rg) - rg * k) * u) / (1.0 + (k + s * (1.0 - rg)) * u);
    }
    let r = s / (k + s + tau);
    let e = (-2.0 * tau).exp();
    let a = 1.0 - rg * r;
    let b = r - rg;
    (r * a - b * e) / (a - r * b * e)
}

/// Concentrations of the first four primaries at an sRGB point, trilinear
/// over the inverse grid.
fn inverse(at: [f32; 3]) -> [f32; PRIMARIES - 1] {
    let p = at.map(|v| v.clamp(0.0, 1.0) * (INVERSE_SIZE - 1) as f32);
    let lo = p.map(|v| (v as usize).min(INVERSE_SIZE - 2));
    let t: [f32; 3] = std::array::from_fn(|k| p[k] - lo[k] as f32);
    let mut out = [0.0; PRIMARIES - 1];
    for corner in 0..8 {
        let mut weight = 1.0;
        let mut index = 0;
        for k in 0..3 {
            let bit = (corner >> k) & 1;
            weight *= if bit == 0 { 1.0 - t[k] } else { t[k] };
            index = index * INVERSE_SIZE + lo[k] + bit;
        }
        for (channel, value) in out.iter_mut().enumerate() {
            let offset = (index * (PRIMARIES - 1) + channel) * 2;
            *value += weight * u16::from_le_bytes(INVERSE[offset..offset + 2].try_into().unwrap()) as f32 / 65535.0;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{hex, to_oklab};

    #[test]
    fn finite_film_keeps_backing_and_absorber_limits() {
        for rg in [0.0, 0.2, 0.9, 1.0] {
            assert_eq!(film(0.0, 0.0, rg), rg);
            assert!((film(0.3, 0.0, rg) - rg * (-0.6f32).exp()).abs() < 2e-6);
            assert!((film(0.0, 0.5, rg) - (rg + 0.5 * (1.0 - rg)) / (1.0 + 0.5 * (1.0 - rg))).abs() < 2e-6);
            assert!((film(20.0, 40.0, rg) - crate::spectral::km(0.5)).abs() < 2e-6);
        }
    }

    #[test]
    fn measured_tubes_keep_their_pigment_through_piles_and_brush_loads() {
        let palette = crate::Palette::new("measured study", crate::palette::catalog());
        let index = |name| palette.tubes.iter().position(|t| t.name == name).unwrap();
        let white = index("zinc white");
        assert_eq!(palette.tubes.iter().filter(|t| tube(t.name).is_some()).count(), 15);
        for name in ["Prussian blue", "carmine lake"] {
            let pigment = index(name);
            let a = tube("zinc white").unwrap();
            let b = tube(name).unwrap();
            assert_eq!(palette.tubes[pigment].color, decode(&b));
            let (wa, wb) = (0.9 * palette.tubes[white].strength, 0.1 * palette.tubes[pigment].strength);
            assert_eq!(palette.tubes[pigment].strength, tube_strength(name).unwrap());
            let expected = std::array::from_fn(|k| (wa * a[k] + wb * b[k]) / (wa + wb));
            let pile = palette.pile(vec![(white, 0.9), (pigment, 0.1)]);
            assert_eq!(pile.pigment_latent, expected);
            assert_eq!(pile.color, decode(&expected));
            assert_ne!(encode(&pile.color), expected);
            for medium in [-0.3, 0.0, 0.5] {
                let paint = pile.laid(medium).with_thinner(0.2);
                assert_eq!(paint.latent().color, expected);
                let mut knife = crate::Knife::new(20.0);
                knife.load(paint, 0.5);
                assert!((0..LATENT_SIZE).all(|k| (knife.paint().unwrap().latent()[k] - expected[k]).abs() < 1e-6));
            }
        }
    }

    #[test]
    fn rgb_only_tubes_and_mixed_recipes_keep_linear_latents() {
        let palette = crate::Palette::new("mixed study", crate::palette::catalog());
        let index = |name| palette.tubes.iter().position(|t| t.name == name).unwrap();
        let (rgb, measured) = (index("yellow ochre"), index("vermilion"));
        let a = encode(&palette.tubes[rgb].color);
        let b = tube("vermilion").unwrap();
        assert_eq!(palette.tubes[rgb].strength, 0.8);
        let pile = palette.pile(vec![(rgb, 0.5), (measured, 0.5)]);
        let weight = palette.tubes[measured].strength;
        let expected: [f32; LATENT_SIZE] = std::array::from_fn(|k| (0.8 * a[k] + weight * b[k]) / (0.8 + weight));
        assert!((0..LATENT_SIZE).all(|k| (pile.pigment_latent[k] - expected[k]).abs() < 1e-6));
        assert_eq!(pile.paint(0.0).latent().color, pile.pigment_latent);
        let nested = decode(&std::array::from_fn(|k| 0.25 * a[k] + 0.75 * b[k]));
        let midpoint: [f32; LATENT_SIZE] = std::array::from_fn(|k| 0.5 * (a[k] + b[k]));
        let direct = decode(&std::array::from_fn(|k| 0.5 * (midpoint[k] + b[k])));
        assert!((0..3).all(|k| (nested[k] - direct[k]).abs() < 1e-6));
    }

    #[test]
    fn measured_pigment_survives_wet_exchange_and_checkpoints() {
        let palette = crate::Palette::new("checkpoint study", crate::palette::catalog());
        let index = palette.tubes.iter().position(|t| t.name == "Prussian blue").unwrap();
        let mut knife = crate::Knife::new(200.0);
        knife.load(palette.pile(vec![(index, 1.0)]).paint(0.0), 0.5);
        let mut canvas = crate::Canvas::new_window(16, 1.0, [0.3; 3], None).with_engine(crate::PIGMENT_MIXER);
        canvas.knife(&mut knife, &[(300.0, 500.0), (700.0, 500.0)], (0.5, 0.5), Some(0.0), true, 0.0);
        let pigment = NAMES.iter().position(|&n| n == "PrussianBlue").unwrap();
        assert!(canvas.wet.lat.iter().any(|lat| lat[pigment] > 0.0));
        let mut bytes = Vec::new();
        canvas.write_state(&mut bytes, "measured pigment").unwrap();
        let (restored, _) = crate::Canvas::read_state(&mut std::io::Cursor::new(&bytes)).unwrap();
        let mut rebuilt = Vec::new();
        restored.write_state(&mut rebuilt, "measured pigment").unwrap();
        assert_eq!(rebuilt, bytes);
    }

    #[test]
    fn round_trips_colors_and_keeps_linear_latents() {
        for r in 0..9 {
            for g in 0..9 {
                for b in 0..9 {
                    let rgb = [r as f32 / 8.0, g as f32 / 8.0, b as f32 / 8.0];
                    let lat = encode(&rgb);
                    assert!(lat[..5].iter().all(|&c| c >= 0.0));
                    assert!((lat[..5].iter().sum::<f32>() - 1.0).abs() < 1e-5);
                    let back = decode(&lat);
                    assert!((0..3).all(|k| (rgb[k] - back[k]).abs() < 2e-6));
                }
            }
        }
        let a = encode(&hex("#172440"));
        let b = encode(&hex("#e8b21c"));
        let ab: [f32; LATENT_SIZE] = std::array::from_fn(|k| (a[k] + b[k]) * 0.5);
        let nested = decode(&std::array::from_fn(|k| ab[k] * 0.5 + b[k] * 0.5));
        let direct = decode(&std::array::from_fn(|k| a[k] * 0.25 + b[k] * 0.75));
        assert!((0..3).all(|k| (nested[k] - direct[k]).abs() < 2e-6));
    }

    #[test]
    fn blue_and_yellow_make_green_including_the_palette_case() {
        let green = lerp(&hex("#002185"), &hex("#fcd200"), 0.5);
        assert!(green[1] > green[0] && green[1] > green[2]);
        let green = lerp(&hex("#172440"), &hex("#e8b21c"), 0.8 / (0.8 + 3.0 * 0.2));
        let [_, a, b] = to_oklab(green);
        let hue = b.atan2(a).to_degrees();
        let chroma = a.hypot(b);
        assert!((110.0..170.0).contains(&hue) && chroma > 0.05, "Prussian/chrome yellow is not a chromatic green: hue {hue}, chroma {chroma}");
    }

    #[test]
    fn palette_tints_stay_light_and_grays_stay_near_neutral() {
        let palette = crate::Palette::tube_box();
        let index = |name| palette.tubes.iter().position(|tube| tube.name == name).unwrap();
        // Raw umber, green earth and smalt have no measured data; their
        // tint-lightness bounds predate the measured mixer.
        for fraction in [0.2, 0.5, 0.8] {
            let color = palette.pile(vec![(index("lead white"), 1.0 - fraction), (index("bone black"), fraction)]).color;
            let [_, a, b] = to_oklab(color);
            let chroma = (a * a + b * b).sqrt();
            assert!(chroma < 0.025, "black/white {fraction} tint has a color cast: {chroma}");
        }
    }

    #[test]
    fn white_tints_keep_their_blue_chroma() {
        let palette = crate::Palette::new("tint study", crate::palette::catalog());
        let index = |name| palette.tubes.iter().position(|tube| tube.name == name).unwrap();
        for (name, fraction, minimum_chroma) in [
            ("Prussian blue", 0.1, 0.02),
            ("ultramarine blue", 0.5, 0.08),
        ] {
            let color = palette.pile(vec![(index("lead white"), 1.0 - fraction), (index(name), fraction)]).color;
            let [_, a, b] = to_oklab(color);
            let chroma = (a * a + b * b).sqrt();
            assert!(b < 0.0 && chroma > minimum_chroma, "lead white/{name} tint lost its blue chroma: {chroma}");
        }
    }

    #[test]
    fn white_and_black_keep_middle_tints_neutral() {
        let palette = crate::Palette::tube_box();
        let index = |name| palette.tubes.iter().position(|tube| tube.name == name).unwrap();
        for fraction in [0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] {
            let color = palette.pile(vec![(index("lead white"), 1.0 - fraction), (index("bone black"), fraction)]).color;
            let [_, a, b] = to_oklab(color);
            let chroma = (a * a + b * b).sqrt();
            assert!(chroma < 0.01, "black/white {fraction} middle tint has a color cast: {chroma}");
            assert!(b >= 0.0, "black/white {fraction} tint is blue: {b}");
        }
    }

    #[test]
    fn palette_violet_and_ultramarine_tints_keep_their_hue() {
        let palette = crate::Palette::new("hue study", crate::palette::catalog());
        let index = |name| palette.tubes.iter().position(|tube| tube.name == name).unwrap();
        let white = index("lead white");
        let blue = index("ultramarine blue");
        let violet = palette.pile(vec![(white, 0.6), (blue, 0.3), (index("vermilion"), 0.1)]).color;
        assert!(to_oklab(violet)[1] > 0.0, "violet lost its red component: {violet:?}");
        let [_, a, b] = to_oklab(palette.pile(vec![(blue, 1.0)]).color);
        let masstone_hue = b.atan2(a);
        for fraction in [0.1, 0.2, 0.3, 0.5, 0.7, 0.9] {
            let color = palette.pile(vec![(white, 1.0 - fraction), (blue, fraction)]).color;
            let [_, a, b] = to_oklab(color);
            let delta = b.atan2(a) - masstone_hue;
            let degrees = delta.sin().atan2(delta.cos()).to_degrees().abs();
            assert!(degrees < 10.0, "ultramarine {fraction} tint shifted hue by {degrees} degrees");
        }
    }

    // A 1% trace may tint white as much as the strongest real tinter in the
    // measured data does, and no more. Revigo sample 13G, 1.1% Prussian blue
    // in zinc white, sits 0.302 from the pure zinc white (07A) in OKLab, both
    // measured over the white field and shown specular-excluded
    // (scripts/pigment_data/revigo, Geldof et al. 2018). The bound is that
    // value plus a margin. With the current tables the strongest 1% traces are
    // ultramarine blue 0.107, Prussian blue 0.102 and viridian 0.093.
    #[test]
    fn catalog_traces_do_not_overpower_white() {
        let palette = crate::Palette::new("trace study", crate::palette::catalog());
        let white = palette.tubes.iter().find(|tube| tube.name == "lead white").unwrap().color;
        let start = to_oklab(white);
        for tube in &palette.tubes {
            let tint = to_oklab(lerp(&white, &tube.color, 0.01));
            let delta = (0..3).map(|k| (tint[k] - start[k]).powi(2)).sum::<f32>().sqrt();
            assert!(delta <= 0.33, "1% {} overpowered white: {delta}", tube.name);
        }
    }
}
