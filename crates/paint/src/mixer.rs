//! Independent pigment mixing from Sochorová and Jamriška (2021), equations
//! 1–14: measured pigment concentrations and an additive linear RGB residual.
//! The primaries' K/S data are derived from measured reflectance of real oil
//! paints (CC0 Revigo data, `scripts/pigment_data/revigo`); both tables are
//! original. Only the MIT spectral.js D65/CIE conversion from `spectral.rs` is
//! reused.
//! Regenerate with `uv run --script scripts/build_pigment_tables.py`.

use crate::color::{Rgb, linear_to_srgb};

mod proxy;
mod researched;
#[cfg(test)]
mod calibration;

pub const LEGACY_LATENT_SIZE: usize = 21;
pub const LATENT_SIZE: usize = LEGACY_LATENT_SIZE + researched::RESEARCHED_TUBES.len();
const PRIMARIES: usize = 5;
const PIGMENTS: usize = 18;
const INVERSE_SIZE: usize = 33;
const INVERSE: &[u8; INVERSE_SIZE * INVERSE_SIZE * INVERSE_SIZE * 2 * (PRIMARIES - 1)] = include_bytes!("mixer/inverse.bin");
const COEFFICIENT_BYTES: usize = PIGMENTS * 2 * crate::spectral::N * 4;
const MEASURED: &[u8; COEFFICIENT_BYTES + PIGMENTS * 4] = include_bytes!("mixer/measured.bin");
const CALIBRATED: &[u8; COEFFICIENT_BYTES + PIGMENTS * 4] = include_bytes!("mixer/calibrated.bin");
const DENSITIES: &[u8; PIGMENTS * 4] = include_bytes!("mixer/calibrated-densities.bin");
const RESEARCHED: &[u8; researched::RESEARCHED_TUBES.len() * 2 * crate::spectral::N * 4] = include_bytes!("mixer/researched.bin");
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

pub fn tube9(name: &str) -> Option<[f32; LATENT_SIZE]> {
    if let Some(lat) = tube(name) { return Some(lat); }
    let i = researched::RESEARCHED_TUBES.iter().position(|&n| n == name)?;
    let mut lat = [0.0; LATENT_SIZE];
    lat[LEGACY_LATENT_SIZE + i] = 1.0;
    Some(lat)
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

/// Checkpoint identity for spectral film optics and their physical wet S.
pub fn film_identity() -> u64 {
    use sha2::{Digest, Sha256};
    let mut hash = Sha256::new();
    hash.update(identity9().to_le_bytes());
    hash.update(b"film25um/spectral-backing/transmitted-residual/v2");
    u64::from_le_bytes(hash.finalize()[..8].try_into().unwrap())
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

pub fn identity9() -> u64 {
    use sha2::{Digest, Sha256};
    let mut hash = Sha256::new();
    hash.update(identity().to_le_bytes());
    hash.update(b"extensive-pigment-and-residual/volume-strength/unit-measured-tubes/v2");
    hash.update(b"opacity-constrained-rgb-tube-proxies/joint-earth-tints/catalog-inputs/v4");
    hash.update(b"physical-volume-ks/dark-calibration/retained-backing/v2");
    hash.update(CALIBRATED);
    hash.update(DENSITIES);
    hash.update(b"independent-researched-ks/legacy18-residual3-extension/tuple-f32/v1");
    hash.update((LATENT_SIZE as u64).to_le_bytes());
    for name in researched::RESEARCHED_TUBES {
        hash.update((name.len() as u64).to_le_bytes());
        hash.update(name.as_bytes());
    }
    hash.update(RESEARCHED);
    u64::from_le_bytes(hash.finalize()[..8].try_into().unwrap())
}

pub fn encode9(rgb: &Rgb) -> [f32; LATENT_SIZE] {
    let mut lat = encode(rgb);
    lat[PIGMENTS..].fill(0.0);
    let base = decode9(&lat);
    for k in 0..3 {
        lat[PIGMENTS + k] = rgb[k] - base[k];
    }
    lat
}

pub fn encode_tube9(rgb: &Rgb, hiding: f32) -> [f32; LATENT_SIZE] { proxy::encode(*rgb, hiding) }

pub fn tube_strength9(name: &str) -> Option<f32> {
    if researched::RESEARCHED_TUBES.contains(&name) && tube_pigment(name).is_none() { return Some(1.0); }
    let name = tube_pigment(name)?;
    let p = NAMES.iter().position(|&n| n == name)?;
    let offset = COEFFICIENT_BYTES + p * 4;
    Some(f32::from_le_bytes(CALIBRATED[offset..offset + 4].try_into().unwrap()))
}

/// Estimated wet-paint density in g/cm³, not a Revigo density measurement.
/// Inputs and their sources are in `scripts/pigment_densities.json`.
pub fn measured_density9(name: &str) -> Option<f32> {
    let p = NAMES.iter().position(|&n| n == name)?;
    Some(f32::from_le_bytes(DENSITIES[p * 4..p * 4 + 4].try_into().unwrap()))
}

fn normalized9(lat: &[f32; LATENT_SIZE]) -> [f32; LATENT_SIZE] {
    let total = lat[..PIGMENTS].iter().chain(&lat[LEGACY_LATENT_SIZE..]).map(|c| c.max(0.0)).sum::<f32>();
    if total <= 0.0 { return [0.0; LATENT_SIZE]; }
    lat.map(|c| c / total)
}

pub fn decode9(lat: &[f32; LATENT_SIZE]) -> Rgb {
    let lat = normalized9(lat);
    let p = pigment9(&lat);
    let reflectance = std::array::from_fn(|w| {
        let rg = f32::from_le_bytes(REFERENCE[w * 4..w * 4 + 4].try_into().unwrap());
        outside(film(p.k[w], p.s[w], rg))
    });
    let base = crate::spectral::to_rgb(&reflectance).map(|v| v.clamp(0.0, 1.0));
    std::array::from_fn(|k| (base[k] + lat[PIGMENTS + k]).clamp(0.0, 1.0))
}

pub fn pigment9(lat: &[f32; LATENT_SIZE]) -> crate::spectral::SpectralPigment {
    static COEFFICIENTS: std::sync::OnceLock<[[crate::spectral::Spectrum; 2]; PIGMENTS]> = std::sync::OnceLock::new();
    let coefficients = COEFFICIENTS.get_or_init(|| std::array::from_fn(|p| std::array::from_fn(|ks| std::array::from_fn(|w| {
        let offset = ((p * 2 + ks) * crate::spectral::N + w) * 4;
        f32::from_le_bytes(CALIBRATED[offset..offset + 4].try_into().unwrap())
    }))));
    let mut absorption = [0.0; crate::spectral::N];
    let mut scattering = [0.0; crate::spectral::N];
    for p in 0..PIGMENTS {
        let c = lat[p].max(0.0);
        if c == 0.0 { continue; }
        for w in 0..crate::spectral::N {
            absorption[w] += c * coefficients[p][0][w];
            scattering[w] += c * coefficients[p][1][w];
        }
    }
    static EXTRA: std::sync::OnceLock<[[crate::spectral::Spectrum; 2]; researched::RESEARCHED_TUBES.len()]> = std::sync::OnceLock::new();
    let extra = EXTRA.get_or_init(|| std::array::from_fn(|p| std::array::from_fn(|ks| std::array::from_fn(|w| {
        let offset = ((p * 2 + ks) * crate::spectral::N + w) * 4;
        f32::from_le_bytes(RESEARCHED[offset..offset + 4].try_into().unwrap())
    }))));
    for (p, coefficients) in extra.iter().enumerate() {
        let c = lat[LEGACY_LATENT_SIZE + p].max(0.0);
        if c == 0.0 { continue; }
        for w in 0..crate::spectral::N {
            absorption[w] += c * coefficients[0][w];
            scattering[w] += c * coefficients[1][w];
        }
    }
    crate::spectral::SpectralPigment { k: absorption, s: scattering }
}

pub fn scattering9(lat: &[f32; LATENT_SIZE]) -> f32 {
    pigment9(lat).s.iter().sum::<f32>() / crate::spectral::N as f32 * crate::COAT_UM / 100.0
}

pub fn over9(lat: &[f32; LATENT_SIZE], scatter: f32, under: Rgb, coats: f32) -> Rgb {
    Backing::default().under(under).over(&normalized9(lat), scatter, coats, 1.0).color
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
    let pigment = pigment(lat);
    let reflectance = std::array::from_fn(|w| {
        let rg = f32::from_le_bytes(REFERENCE[w * 4..w * 4 + 4].try_into().unwrap());
        let r = film(pigment.k[w], pigment.s[w], rg);
        outside(r)
    });
    crate::spectral::to_rgb(&reflectance).map(|v| v.clamp(0.0, 1.0))
}

/// The latent's two-constant coefficients, per 100 µm of tube paint.
pub fn pigment(lat: &[f32; LATENT_SIZE]) -> crate::spectral::SpectralPigment {
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
    crate::spectral::SpectralPigment { k: absorption, s: scattering }
}

fn outside(r: f32) -> f32 {
    (1.0 - 0.04) * (1.0 - 0.49) * r / (1.0 - 0.49 * r)
}

fn inside(r: f32) -> f32 {
    r / ((1.0 - 0.04) * (1.0 - 0.49) + 0.49 * r)
}

#[derive(Clone, Copy)]
pub(crate) struct Backing {
    pub reflectance: crate::spectral::Spectrum,
    pub residual: Rgb,
    pub color: Rgb,
}

impl Default for Backing {
    fn default() -> Self {
        Self { reflectance: [0.0; crate::spectral::N], residual: [0.0; 3], color: [-1.0; 3] }
    }
}

impl Backing {
    pub fn under(self, color: Rgb) -> Self {
        if self.color == color {
            return self;
        }
        Self { reflectance: crate::spectral::from_rgb(color), residual: [0.0; 3], color }
    }

    pub fn over(mut self, lat: &[f32; LATENT_SIZE], scatter: f32, coats: f32, cover: f32) -> Self {
        if coats <= 0.0 || scatter <= 0.0 || cover.is_nan() || cover <= 0.0 {
            return self;
        }
        let cover = cover.min(1.0);
        let p = pigment9(lat);
        let mean_s = p.s.iter().sum::<f32>() / crate::spectral::N as f32;
        let x = (coats / cover).min(1e4) * scatter / mean_s.max(1e-12);
        let mut transmission = [0.0; crate::spectral::N];
        let reflectance = std::array::from_fn(|w| {
            let sub = self.reflectance[w] as f64;
            let sub = sub / ((1.0 - 0.04) * (1.0 - 0.49) + 0.49 * sub);
            let (k, s, x) = (p.k[w] as f64, p.s[w] as f64, x as f64);
            let (r, t) = if s < 1e-6 {
                (0.0, (-k * x).exp())
            } else {
                let a = 1.0 + k / s;
                let b = (a * a - 1.0).max(0.0).sqrt();
                if b < 1e-3 {
                    let sx = s * x;
                    (sx / (1.0 + a * sx), 1.0 / (1.0 + a * sx))
                } else {
                    let bsx = (b * s * x).min(40.0);
                    let (sh, ch) = (bsx.sinh(), bsx.cosh());
                    let c = a * sh + b * ch;
                    (sh / c, b / c)
                }
            };
            transmission[w] = (t * t) as f32;
            let r = r + t * t * sub / (1.0 - r * sub).max(1e-6);
            ((1.0 - 0.04) * (1.0 - 0.49) * r / (1.0 - 0.49 * r)) as f32
        });
        let transmitted = crate::spectral::to_rgb(&transmission);
        let residual: Rgb = std::array::from_fn(|k| {
            self.residual[k] + (1.0 - transmitted[k]).clamp(0.0, 1.0) * (lat[PIGMENTS + k] - self.residual[k])
        });
        let rgb = crate::spectral::to_rgb(&reflectance);
        self.color = std::array::from_fn(|k| self.color[k] + ((rgb[k] + residual[k]).clamp(0.0, 1.0) - self.color[k]) * cover);
        for w in 0..crate::spectral::N {
            self.reflectance[w] = if cover == 1.0 { reflectance[w] } else { self.reflectance[w] + cover * (reflectance[w] - self.reflectance[w]) };
        }
        for k in 0..3 {
            self.residual[k] = if cover == 1.0 { residual[k] } else { self.residual[k] + cover * (residual[k] - self.residual[k]) };
        }
        self
    }

    pub fn over_wet(self, lat: &crate::wet::Latent, scatter: f32, coats: f32, cover: f32) -> Self {
        if lat.layers.is_empty() {
            return self.over(&normalized9(lat), scatter, coats, cover);
        }
        if coats <= 0.0 || scatter <= 0.0 || cover.is_nan() || cover <= 0.0 {
            return self;
        }
        let cover = cover.min(1.0);
        let mean: f32 = lat.layers.iter().map(|l| l.share * l.hide[0]).sum();
        let scale = if mean > 0.0 { scatter / mean } else { 1.0 };
        let mut over = lat.layers.iter().fold(self, |backing, layer| {
            backing.over(&normalized9(&layer.lat), layer.hide[0] * scale, (coats * layer.share / cover).min(1e4), 1.0)
        });
        if cover < 1.0 {
            for w in 0..crate::spectral::N {
                over.reflectance[w] = self.reflectance[w] + cover * (over.reflectance[w] - self.reflectance[w]);
            }
            for k in 0..3 {
                over.residual[k] = self.residual[k] + cover * (over.residual[k] - self.residual[k]);
            }
            over.color = std::array::from_fn(|k| self.color[k] + (over.color[k] - self.color[k]) * cover);
        }
        over
    }
}

/// Mean scattering per engine coat (25 µm), carried with the wet paint.
pub fn scattering(lat: &[f32; LATENT_SIZE]) -> f32 {
    pigment(lat).s.iter().sum::<f32>() / crate::spectral::N as f32 * crate::COAT_UM / 100.0
}

/// A latent film over the existing specular-excluded reflectance. The
/// carried scattering scales both K and S as medium or drainage changes
/// pigment per volume. RGB reconstruction residuals fade with transmission.
pub fn over(lat: &[f32; LATENT_SIZE], scatter: f32, under: Rgb, coats: f32) -> Rgb {
    if coats <= 0.0 || scatter <= 0.0 {
        return under;
    }
    let p = pigment(lat);
    let mean_s = p.s.iter().sum::<f32>() / crate::spectral::N as f32;
    let x = coats * scatter / mean_s.max(1e-12);
    let sub = crate::spectral::from_rgb(under).map(inside);
    let mut transmission = [0.0; crate::spectral::N];
    let reflectance = std::array::from_fn(|w| {
        let (r, t) = crate::pigment::layer1(p.k[w], p.s[w], x);
        transmission[w] = t * t;
        outside(r + t * t * sub[w] / (1.0 - r * sub[w]).max(1e-6))
    });
    let rgb = crate::spectral::to_rgb(&reflectance);
    let transmitted = crate::spectral::to_rgb(&transmission);
    std::array::from_fn(|k| (rgb[k] + lat[PIGMENTS + k] * (1.0 - transmitted[k]).clamp(0.0, 1.0)).clamp(0.0, 1.0))
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
    fn splitting_measured_films_preserves_the_backing_spectrum() {
        let mut palette = crate::Palette::new("repeated film study", crate::palette::catalog());
        palette.engine = crate::PIGMENT_MIXER;
        palette.glaze_film = true;
        let index = |name| palette.tubes.iter().position(|t| t.name == name).unwrap();
        let yellow = index("chrome yellow");
        let white = index("lead white");
        for (parts, under, total) in [
            (vec![(yellow, 1.0)], [0.85; 3], 0.2),
            (vec![(yellow, 0.05), (white, 0.95)], [0.5; 3], 1.0),
        ] {
            let paint = palette.pile(parts).paint(0.0);
            let expected = paint.over(under, total);
            for n in [1, 4, 100] {
                let mut canvas = crate::Canvas::new_window(4, 1.0, under, None)
                    .with_engine(crate::PIGMENT_MIXER).with_glaze_film(true);
                for coat in 0..n {
                    canvas.wet.vol.fill(total / n as f32);
                    canvas.wet.lat.fill(paint.latent());
                    canvas.wet.hide.fill(paint.prop());
                    canvas.wet.dirty = Some((0, 0, canvas.f.w, canvas.f.h));
                    let wet = canvas.look_px(0);
                    canvas.dry();
                    assert_eq!(wet, canvas.px[0]);
                    if coat == n / 2 {
                        let mut bytes = Vec::new();
                        canvas.write_state(&mut bytes, "repeated film").unwrap();
                        let (restored, _) = crate::Canvas::read_state(&mut &bytes[..]).unwrap();
                        let mut rebuilt = Vec::new();
                        restored.write_state(&mut rebuilt, "repeated film").unwrap();
                        assert_eq!(bytes, rebuilt);
                        canvas = restored;
                    }
                }
                let actual = canvas.px[0];
                let a = to_oklab(actual);
                let b = to_oklab(expected);
                let delta = (0..3).map(|k| (a[k] - b[k]).powi(2)).sum::<f32>().sqrt();
                eprintln!("{n} coats: {actual:?}, one {expected:?}, delta {delta:.9}");
                assert!(delta < 2e-6, "{n} coats changed the film: {actual:?} != {expected:?}, delta {delta}");
                if n == 4 {
                    let mut latent = paint.latent();
                    latent.layers = (0..4).map(|_| crate::wet::Layer { share: 0.25, lat: paint.latent(), hide: paint.prop() }).collect();
                    canvas.wet.vol.fill(total);
                    canvas.wet.lat.fill(latent);
                    canvas.wet.hide.fill(paint.prop());
                    canvas.wet.dirty = Some((0, 0, canvas.f.w, canvas.f.h));
                    let mut bytes = Vec::new();
                    canvas.write_state(&mut bytes, "retained wet and dry films").unwrap();
                    assert_eq!(&bytes[..8], b"PAINTC21");
                    let (mut restored, _) = crate::Canvas::read_state(&mut &bytes[..]).unwrap();
                    let expected = paint.over(under, total * 2.0);
                    let wet = restored.look_px(0);
                    restored.dry();
                    assert_eq!(wet, restored.px[0]);
                    let a = to_oklab(restored.px[0]);
                    let b = to_oklab(expected);
                    let delta = (0..3).map(|k| (a[k] - b[k]).powi(2)).sum::<f32>().sqrt();
                    assert!(delta < 2e-6, "retained wet layers changed the backing: {:?} != {expected:?}, delta {delta}", restored.px[0]);
                }
            }
        }
    }

    #[test]
    fn retained_backing_keeps_diluted_rgb_residuals() {
        let mut palette = crate::Palette::new("diluted residuals", crate::palette::catalog());
        palette.engine = crate::PIGMENT_MIXER;
        palette.glaze_film = true;
        for name in ["green earth", "magenta"] {
            let i = palette.tubes.iter().position(|t| t.name == name).unwrap();
            let paint = palette.pile(vec![(i, 1.0)]).paint(0.8);
            let under = [0.8; 3];
            let expected = paint.over(under, 0.5);
            let actual = Backing::default().under(under).over_wet(&paint.latent(), paint.scatter, 0.5, 1.0).color;
            assert!((0..3).all(|k| (actual[k] - expected[k]).abs() < 2e-6), "{name}: {actual:?} != {expected:?}");
        }
    }

    #[test]
    fn measured_tube_volumes_keep_their_calibrated_coefficients() {
        let mut palette = crate::Palette::new("measured tube volumes", crate::palette::catalog());
        palette.engine = crate::PIGMENT_MIXER;
        for spectral in [false, true] {
            palette.glaze_film = spectral;
            for (i, tube) in palette.tubes.iter().enumerate() {
                let Some(latent) = super::tube(tube.name) else { continue };
                let measured = pigment9(&latent);
                for medium in [0.0, 0.8] {
                    let paint = palette.pile(vec![(i, 1.0)]).paint(medium);
                    let actual = pigment9(&paint.latent());
                    for w in 0..crate::spectral::N {
                        assert!((actual.k[w] - measured.k[w] * (1.0 - medium)).abs() < 1e-6, "{}, medium={medium}, wavelength={w}", tube.name);
                        assert!((actual.s[w] - measured.s[w] * (1.0 - medium)).abs() < 1e-6, "{}, medium={medium}, wavelength={w}", tube.name);
                    }
                }
            }
        }
    }

    #[test]
    fn researched_tubes_have_independent_exact_spectra() {
        for (i, name) in researched::RESEARCHED_TUBES.iter().enumerate() {
            assert!(tube(name).is_none());
            let lat = tube9(name).unwrap();
            assert_eq!(lat[LEGACY_LATENT_SIZE + i], 1.0);
            assert_eq!(lat.iter().filter(|&&c| c != 0.0).count(), 1);
            assert_eq!(tube_strength9(name), Some(1.0));
            let pigment = pigment9(&lat);
            for (ks, actual) in [pigment.k, pigment.s].iter().enumerate() {
                for (w, value) in actual.iter().enumerate() {
                    let offset = ((i * 2 + ks) * crate::spectral::N + w) * 4;
                    let expected = f32::from_le_bytes(RESEARCHED[offset..offset + 4].try_into().unwrap());
                    assert_eq!(value.to_bits(), expected.to_bits(), "{name}, {ks}, {w}");
                    assert!(value.is_finite() && *value >= 0.0);
                }
            }
            let legacy = super::pigment(&lat);
            assert!(legacy.k.iter().chain(&legacy.s).all(|&c| c == 0.0));
        }
        for name in NAMES { assert_eq!(measured(name).unwrap()[LEGACY_LATENT_SIZE..], [0.0; LATENT_SIZE - LEGACY_LATENT_SIZE]); }
        for name in ["lead white", "Prussian blue", "vermilion"] { assert_eq!(tube9(name), tube(name)); }
        let mut residual = tube9("yellow ochre").unwrap();
        residual[PIGMENTS..LEGACY_LATENT_SIZE].copy_from_slice(&[0.02, -0.01, 0.03]);
        let diluted = residual.map(|c| c * 0.25);
        let expected = decode9(&residual);
        let actual = decode9(&diluted);
        assert!((0..3).all(|k| (actual[k] - expected[k]).abs() < 2e-6));
    }

    #[test]
    fn researched_spectra_mix_by_volume_and_dilute_through_knives_and_wet_paint() {
        let mut palette = crate::Palette::new("extension transport", crate::palette::catalog());
        palette.engine = crate::PIGMENT_MIXER;
        palette.glaze_film = true;
        let at = |name| palette.tubes.iter().position(|t| t.name == name).unwrap();
        let a = palette.pile(vec![(at("yellow ochre"), 1.0)]).paint(0.6);
        let b = palette.pile(vec![(at("vermilion"), 1.0)]).paint(0.0);
        assert!(a.parts.is_empty() && b.parts.is_empty());
        let expected: [f32; LATENT_SIZE] = std::array::from_fn(|k| (a.latent()[k] + 3.0 * b.latent()[k]) / 4.0);
        let pa = pigment9(&a.latent());
        let pb = pigment9(&b.latent());
        let mixed = pigment9(&expected);
        for w in 0..crate::spectral::N {
            assert!((mixed.k[w] - (pa.k[w] + 3.0 * pb.k[w]) / 4.0).abs() < 2e-6);
            assert!((mixed.s[w] - (pa.s[w] + 3.0 * pb.s[w]) / 4.0).abs() < 2e-6);
        }
        let pure = palette.pile(vec![(at("yellow ochre"), 1.0)]).paint(0.0);
        let diluted = pigment9(&a.latent());
        let undiluted = pigment9(&pure.latent());
        for w in 0..crate::spectral::N {
            assert!((diluted.k[w] - 0.4 * undiluted.k[w]).abs() < 2e-6);
            assert!((diluted.s[w] - 0.4 * undiluted.s[w]).abs() < 2e-6);
        }
        for under in [[0.02; 3], [0.8; 3]] {
            let actual = a.over(under, 1.0);
            let expected = pure.over(under, 0.4);
            assert!((0..3).all(|k| (actual[k] - expected[k]).abs() < 2e-6));
        }
        let mut knife = crate::Knife::new(200.0);
        knife.load(a.clone(), 0.1);
        knife.load(b.clone(), 0.3);
        let carried = knife.paint().unwrap();
        assert!((0..LATENT_SIZE).all(|k| (carried.latent()[k] - expected[k]).abs() < 2e-6));
        let mut vol = 0.0;
        let mut lat = crate::wet::Latent::default();
        let mut prop = [0.0; 10];
        crate::wet::mix_into(&mut vol, &mut lat, &mut prop, 1.0, &a.latent(), a.prop());
        crate::wet::mix_into(&mut vol, &mut lat, &mut prop, 3.0, &b.latent(), b.prop());
        assert_eq!(vol, 4.0);
        assert!((0..LATENT_SIZE).all(|k| (lat[k] - expected[k]).abs() < 2e-6));
        let mut canvas = crate::Canvas::new_window(16, 1.0, [0.3; 3], None).with_engine(crate::PIGMENT_MIXER).with_glaze_film(true);
        canvas.knife(&mut knife, &[(300.0, 500.0), (700.0, 500.0)], (0.5, 0.5), Some(0.0), true, 0.0);
        assert!(canvas.wet_total() > 0.0);
        for (i, latent) in canvas.wet.lat.iter().enumerate().filter(|(i, _)| canvas.wet.vol[*i] > 0.0) {
            assert!((0..LATENT_SIZE).all(|k| (latent[k] - expected[k]).abs() < 2e-6), "wet pixel {i}");
        }
        let mut bytes = Vec::new();
        canvas.write_state(&mut bytes, "extension transport").unwrap();
        let (restored, _) = crate::Canvas::read_state(&mut &bytes[..]).unwrap();
        assert_eq!(restored.wet.lat, canvas.wet.lat);
        assert_eq!(restored.seen(), canvas.seen());
        let mut again = Vec::new();
        restored.write_state(&mut again, "extension transport").unwrap();
        assert_eq!(again, bytes);
    }

    #[test]
    fn extension_lanes_leave_legacy_optics_and_palette_construction_unchanged() {
        let rgb = hex("#365b7e");
        let legacy = encode(&rgb);
        let mut extended = legacy;
        extended[LEGACY_LATENT_SIZE..].fill(2.0);
        assert_eq!(decode(&legacy), decode(&extended));
        assert_eq!(pigment(&legacy).k, pigment(&extended).k);
        assert_eq!(pigment(&legacy).s, pigment(&extended).s);
        assert_eq!(over(&legacy, 0.7, [0.8; 3], 0.3), over(&extended, 0.7, [0.8; 3], 0.3));
        let mut palette = crate::Palette::new("legacy extension", crate::palette::catalog());
        let i = palette.tubes.iter().position(|t| t.name == "yellow ochre").unwrap();
        for engine in [8, 9, 8] {
            palette.engine = engine;
            let lat = palette.pile(vec![(i, 1.0)]).pigment_latent;
            if engine == 8 {
                let expected = encode(&palette.tubes[i].color);
                assert!((0..LATENT_SIZE).all(|k| (lat[k] - expected[k]).abs() < 1e-6));
                assert!(lat[LEGACY_LATENT_SIZE..].iter().all(|&c| c == 0.0));
            }
            else { assert_eq!(lat, tube9("yellow ochre").unwrap()); }
        }
    }

    #[test]
    fn films_keep_measured_coefficients_thickness_and_medium() {
        let mut palette = crate::Palette::new("film study", crate::palette::catalog());
        palette.engine = crate::PIGMENT_MIXER;
        palette.glaze_film = true;
        for name in ["Prussian blue", "ultramarine blue", "lead white", "yellow ochre"] {
            let i = palette.tubes.iter().position(|t| t.name == name).unwrap();
            let pile = palette.pile(vec![(i, 1.0)]);
            let lat = &pile.pigment_latent;
            let p = pigment9(lat);
            for medium in [0.0, 0.8] {
                let paint = pile.paint(medium);
                for under in [[0.0; 3], [0.8; 3]] {
                    let sub = crate::spectral::from_rgb(under).map(inside);
                    let expected = crate::spectral::to_rgb(&std::array::from_fn(|w| outside(film(p.k[w] * (1.0 - medium), p.s[w] * (1.0 - medium), sub[w]))));
                    let actual = paint.over(under, 100.0 / crate::COAT_UM);
                    if tube(name).is_some() {
                        assert!((0..3).all(|k| (actual[k] - expected[k]).abs() < 2e-6), "{name}: {actual:?} != {expected:?}");
                    }
                    assert_eq!(paint.over(under, 0.0), under);
                    let diluted = paint.over(under, 1.0);
                    let thinner = pile.paint(0.0).over(under, 1.0 - medium);
                    assert!((0..3).all(|k| (diluted[k] - thinner[k]).abs() < 2e-6));
                }
            }
        }
    }

    #[test]
    fn palette_previews_follow_canvas_film_optics() {
        for (engine, glaze_film, spectral) in [(8, false, false), (8, true, false), (9, false, false), (9, true, true)] {
            let mut palette = crate::Palette::new("preview study", crate::palette::catalog());
            palette.engine = engine;
            palette.glaze_film = glaze_film;
            let i = palette.tubes.iter().position(|t| t.name == "Prussian blue").unwrap();
            let pile = palette.pile(vec![(i, 1.0)]);
            for medium in [0.0, 0.8] {
                let paint = pile.laid(medium);
                let expected_latent = if engine >= crate::PIGMENT_MIXER { pile.pigment_latent.map(|c| c * (1.0 - medium)) } else { pile.pigment_latent };
                assert_eq!(paint.pigment_latent, Some(expected_latent));
                assert_eq!(paint.latent().color, expected_latent);
                let mut optics_difference = 0.0f32;
                for under in [[0.01; 3], [0.85; 3]] {
                    for coats in [0.16, 0.48, 8.0] {
                        let mut canvas = crate::Canvas::new(4, 1.0, under).with_engine(engine).with_glaze_film(glaze_film);
                        assert_eq!(canvas.spectral_films(), spectral);
                        canvas.wet.vol.fill(coats);
                        canvas.wet.lat.fill(paint.latent());
                        canvas.wet.hide.fill(paint.prop());
                        canvas.wet.dirty = Some((0, 0, canvas.f.w, canvas.f.h));
                        let rgb = paint.pigment().over(under, coats);
                        let measured = if engine >= crate::PIGMENT_MIXER { over9(&expected_latent, paint.scatter, under, coats) } else { over(&pile.pigment_latent, paint.scatter, under, coats) };
                        for k in 0..3 {
                            optics_difference = optics_difference.max((measured[k] - rgb[k]).abs());
                        }
                        let expected = if spectral { measured } else { rgb };
                        let preview = paint.over(under, coats);
                        assert!((0..3).all(|k| (preview[k] - expected[k]).abs() < 2e-6), "engine {engine}, glaze_film {glaze_film}: {preview:?} != {expected:?}");
                        let wet = canvas.look_px(0);
                        assert!((0..3).all(|k| (preview[k] - wet[k]).abs() < 2e-6));
                        canvas.dry();
                        assert!((0..3).all(|k| (preview[k] - canvas.px[0][k]).abs() < 2e-6));
                    }
                }
                assert!(optics_difference > 2e-6, "engine {engine}, glaze_film {glaze_film}, medium {medium}: the optics paths must differ beyond the comparison tolerance");
            }
        }
    }

    #[test]
    fn measured_films_report_errors_over_both_revigo_fields() {
        let spectra: Vec<(&str, Vec<f32>)> = include_str!("../../../scripts/pigment_data/revigo/revigo-aggregated-palette-spectral-data.txt")
            .lines().skip(1).map(|line| {
                let mut fields = line.split('\t');
                (fields.next().unwrap(), fields.map(|v| v.parse::<f32>().unwrap() / 100.0).collect())
            }).collect();
        let spectrum = |id: &str| &spectra.iter().find(|p| p.0 == id).unwrap().1;
        for (name, id) in [("Eosin1Wax", "01A"), ("Eosin", "02A"), ("Eosin2PbWax", "03A"), ("Cochineal", "04A"), ("Minium", "05A"), ("Vermillion", "06A"), ("ZincWhite", "07A"), ("LeadWhite", "08A"), ("ViridianGreen", "09A"), ("EmeraldGreen", "10A"), ("UltramarineBlue", "11A"), ("CobaltBlue", "12A"), ("PrussianBlue", "13A"), ("ChromeYellow", "14A"), ("ChromeYellowOtero", "15A"), ("BariumSulphate", "16A"), ("CalciumCarbonate", "17A")] {
            let p = pigment(&measured(name).unwrap());
            let mut errors = [0.0; 2];
            for (field, (bare, painted)) in [("_b", "_ob"), ("_w", "_ow")].into_iter().enumerate() {
                let bare = spectrum(&format!("{id}{bare}"));
                let painted = spectrum(&format!("{id}{painted}"));
                let mut sum = 0.0;
                for w in 0..31 {
                    let sub = inside(bare[w] - 0.04);
                    let expected = outside(film(p.k[w + 2], p.s[w + 2], sub)) + 0.04;
                    let (r, t) = crate::pigment::layer1(p.k[w + 2], p.s[w + 2], 1.0);
                    let actual = outside(r + t * t * sub / (1.0 - r * sub)) + 0.04;
                    assert!((actual - expected).abs() < 2e-6, "{name} {field} {w}: {actual} != {expected}");
                    sum += (actual - painted[w]).powi(2);
                }
                errors[field] = (sum / 31.0).sqrt();
            }
            eprintln!("100um {name}: black RMSE {:.6}, white RMSE {:.6}", errors[0], errors[1]);
        }
    }

    #[test]
    fn wet_and_baked_films_keep_the_same_measured_pigment_and_partial_coverage() {
        let mut palette = crate::Palette::new("film study", crate::palette::catalog());
        palette.engine = crate::PIGMENT_MIXER;
        palette.glaze_film = true;
        for name in ["Prussian blue", "lead white", "yellow ochre"] {
            let i = palette.tubes.iter().position(|t| t.name == name).unwrap();
            let paint = palette.pile(vec![(i, 1.0)]).paint(0.8);
            for cover in [0.2, 1.0] {
                let mut canvas = crate::Canvas::new(4, 1.0, [0.8; 3]).with_engine(crate::PIGMENT_MIXER).with_glaze_film(true);
                canvas.wet.vol.fill(0.15);
                canvas.wet.lat.fill(paint.latent());
                canvas.wet.hide.fill(paint.prop());
                canvas.wet.cover.fill(cover);
                canvas.wet.dirty = Some((0, 0, canvas.f.w, canvas.f.h));
                let expected = paint.over([0.8; 3], 0.15 / cover);
                let expected = expected.map(|v| 0.8 + cover * (v - 0.8));
                let wet = canvas.look_px(0);
                assert!((0..3).all(|k| (wet[k] - expected[k]).abs() < 2e-6), "{name}: {wet:?} != {expected:?}");
                let mut state = Vec::new();
                canvas.write_state(&mut state, "").unwrap();
                let (restored, _) = crate::Canvas::read_state(&mut &state[..]).unwrap();
                assert_eq!(restored.look_px(0), wet);
                state[16..24].copy_from_slice(&identity().to_le_bytes());
                assert!(crate::Canvas::read_state(&mut &state[..]).is_err());
                canvas.dry();
                assert!((0..3).all(|k| (canvas.px[0][k] - expected[k]).abs() < 2e-6), "{name}: {:?} != {expected:?}", canvas.px[0]);
            }
        }
    }

    #[test]
    fn measured_dry_colors_do_not_reconstruct_absorption_from_rgb() {
        let mut palette = crate::Palette::new("pastel study", crate::palette::catalog());
        palette.engine = crate::PIGMENT_MIXER;
        for name in ["Prussian blue", "ultramarine blue", "lead white", "bone black"] {
            let i = palette.tubes.iter().position(|t| t.name == name).unwrap();
            let parts = [(i, 1.0)];
            let dry = palette.dry_color(&parts);
            palette.tubes[i].color = [0.5; 3];
            assert_eq!(palette.dry_color(&parts), dry);
            assert!(dry.iter().all(|v| v.is_finite() && *v >= 0.0));
        }
    }

    #[test]
    fn dry_colors_are_continuous_as_measured_parts_go_to_zero() {
        let mut palette = crate::Palette::new("dry trace study", crate::palette::catalog());
        let white = palette.tubes.iter().position(|t| t.name == "lead white").unwrap();
        for engine in [crate::PIGMENT_MIXER, crate::PIGMENT_MIXER + 1] {
            palette.engine = engine;
            for (i, t) in palette.tubes.iter().enumerate() {
                if tube(t.name).is_some() {
                    continue;
                }
                let pure = crate::color::to_oklab(palette.dry_color(&[(i, 1.0)]));
                for f in [0.0, 1e-8, 1e-6] {
                    let trace = crate::color::to_oklab(palette.dry_color(&[(i, 1.0 - f), (white, f)]));
                    let delta = (0..3).map(|k| (pure[k] - trace[k]).powi(2)).sum::<f32>().sqrt();
                    assert!(delta < 1e-4, "engine {engine}, {} with {f} white: delta {delta}", t.name);
                }
            }
        }
    }

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
