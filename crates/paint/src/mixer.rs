//! Independent pigment mixing from Sochorová and Jamriška (2021), equations
//! 1–14: four primary concentrations and an additive linear RGB residual.
//! The synthetic primaries, their K/S data and both tables are original. Only
//! the MIT spectral.js D65/CIE conversion from `spectral.rs` is reused.
//! Regenerate with `uv run --script scripts/build_pigment_tables.py`.

use crate::color::{Rgb, linear_to_srgb};

pub const LATENT_SIZE: usize = 7;
const SIZE: usize = 33;
const INVERSE: &[u8; SIZE * SIZE * SIZE * 6] = include_bytes!("mixer/inverse.bin");
const FORWARD: &[u8; SIZE * SIZE * SIZE * 12] = include_bytes!("mixer/forward.bin");

pub fn encode(rgb: &Rgb) -> [f32; LATENT_SIZE] {
    let c = lookup(rgb.map(linear_to_srgb), false);
    let c3 = (1.0 - c.iter().sum::<f32>()).max(0.0);
    let mut lat = [c[0], c[1], c[2], c3, 0.0, 0.0, 0.0];
    let base = pigment_rgb(&lat);
    for k in 0..3 {
        lat[4 + k] = rgb[k] - base[k];
    }
    lat
}

pub fn decode(lat: &[f32; LATENT_SIZE]) -> Rgb {
    let base = pigment_rgb(lat);
    std::array::from_fn(|k| (base[k] + lat[4 + k]).clamp(0.0, 1.0))
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
    let c = [lat[0].max(0.0), lat[1].max(0.0), lat[2].max(0.0), lat[3].max(0.0)];
    let ratio = |a: f32, b: f32| {
        let a = a.sqrt();
        let b = b.sqrt();
        if a + b > 0.0 { a / (a + b) } else { 0.0 }
    };
    lookup([ratio(c[0], c[1] + c[2] + c[3]), ratio(c[1], c[2] + c[3]), ratio(c[2], c[3])], true)
}

fn lookup(at: [f32; 3], forward: bool) -> Rgb {
    let p = at.map(|v| v.clamp(0.0, 1.0) * (SIZE - 1) as f32);
    let lo = p.map(|v| (v as usize).min(SIZE - 2));
    let t: [f32; 3] = std::array::from_fn(|k| p[k] - lo[k] as f32);
    let mut out = [0.0; 3];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let index = ((lo[0] + i) * SIZE + lo[1] + j) * SIZE + lo[2] + k;
                let weight = (if i == 0 { 1.0 - t[0] } else { t[0] })
                    * (if j == 0 { 1.0 - t[1] } else { t[1] })
                    * (if k == 0 { 1.0 - t[2] } else { t[2] });
                for channel in 0..3 {
                    let value = if forward {
                        let offset = index * 12 + channel * 4;
                        f32::from_le_bytes(FORWARD[offset..offset + 4].try_into().unwrap())
                    } else {
                        let offset = index * 6 + channel * 2;
                        u16::from_le_bytes(INVERSE[offset..offset + 2].try_into().unwrap()) as f32 / 65535.0
                    };
                    out[channel] += weight * value;
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{hex, to_oklab};

    #[test]
    fn round_trips_colors_and_keeps_linear_latents() {
        for r in 0..9 {
            for g in 0..9 {
                for b in 0..9 {
                    let rgb = [r as f32 / 8.0, g as f32 / 8.0, b as f32 / 8.0];
                    let lat = encode(&rgb);
                    assert!(lat[..4].iter().all(|&c| c >= 0.0));
                    assert!((lat[..4].iter().sum::<f32>() - 1.0).abs() < 1e-5);
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
        let expected = to_oklab(hex("#4a7634"));
        let actual = to_oklab(green);
        let delta = (0..3).map(|k| (actual[k] - expected[k]).powi(2)).sum::<f32>().sqrt();
        assert!(delta < 0.04, "Prussian/chrome yellow ΔE {delta}: {green:?}");
    }

    #[test]
    fn palette_tints_stay_light_and_grays_stay_near_neutral() {
        let palette = crate::Palette::tube_box();
        let index = |name| palette.tubes.iter().position(|tube| tube.name == name).unwrap();
        for (name, fraction, minimum_lightness) in [
            ("raw umber", 0.3, 0.72),
            ("green earth", 0.3, 0.78),
            ("smalt", 0.1, 0.84),
        ] {
            let color = palette.pile(vec![(index("lead white"), 1.0 - fraction), (index(name), fraction)]).color;
            let lightness = to_oklab(color)[0];
            assert!(lightness > minimum_lightness, "lead white/{name} tint is too dark: {lightness}");
        }
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
}
