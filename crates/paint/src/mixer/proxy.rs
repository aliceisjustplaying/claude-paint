use crate::color::{Rgb, luminance, to_oklab};

pub(super) fn tube(name: &str) -> Option<[f32; super::LATENT_SIZE]> {
    let names = ["yellow ochre", "raw umber"];
    let index = names.iter().position(|&n| n == name)?;
    static FITS: std::sync::OnceLock<[[f32; super::LATENT_SIZE]; 2]> = std::sync::OnceLock::new();
    Some(FITS.get_or_init(|| {
        let catalog = crate::palette::catalog();
        std::array::from_fn(|i| {
            let tube = catalog.iter().find(|t| t.name == names[i]).unwrap();
            fit(tube.color, tube.hiding)
        })
    })[index])
}

fn fit(rgb: Rgb, hiding: f32) -> [f32; super::LATENT_SIZE] {
    let old = super::encode(&rgb);
    let white = super::tube("lead white").unwrap();
    let strength = super::tube_strength("lead white").unwrap();
    let fractions = [0.01, 0.05, 0.1, 0.25, 0.5];
    let targets = fractions.map(|f| {
        let latent = std::array::from_fn(|k| white[k] * strength * (1.0 - f) + old[k] * f);
        to_oklab(super::decode(&super::normalized9(&latent)))
    });
    let residual = |mut latent: [f32; super::LATENT_SIZE]| {
        for k in 0..3 { latent[super::PIGMENTS + k] = 0.0; }
        let base = super::decode9(&latent);
        for k in 0..3 { latent[super::PIGMENTS + k] = rgb[k] - base[k]; }
        latent
    };
    let target = to_oklab(rgb);
    let error = |shape: [f32; super::LATENT_SIZE], amount: f32| {
        let latent = shape.map(|v| v * amount);
        let tints = fractions.iter().zip(targets).map(|(&f, target)| {
            let mix = std::array::from_fn(|k| white[k] * (1.0 - f) + latent[k] * f);
            let color = to_oklab(super::decode9(&mix));
            (0..3).map(|k| (color[k] - target[k]).powi(2)).sum::<f32>()
        }).sum::<f32>();
        let scatter = super::scattering9(&latent);
        let opaque = to_oklab(super::over9(&latent, scatter, [0.01; 3], 4.0));
        let black = luminance(super::over9(&latent, scatter, [0.0; 3], 1.0));
        let white = luminance(super::over9(&latent, scatter, [1.0; 3], 1.0));
        tints + (0..3).map(|k| (opaque[k] - target[k]).powi(2)).sum::<f32>()
            + (black / white.max(1e-6) - hiding).powi(2)
    };
    let mut best = residual(old);
    let mut amount = 1.0 / strength;
    let mut distance = error(best, amount);
    for _ in 0..40 {
        let start = best;
        let start_amount = amount;
        for i in 0..super::PIGMENTS {
            for t in [0.00390625, 0.015625, 0.03125, 0.0625, 0.125, 0.25, 0.5, 1.0] {
                let mut shape = start.map(|v| v * (1.0 - t));
                shape[i] += t;
                let shape = residual(shape);
                for scale in [0.75, 0.95, 1.0, 1.05, 1.3333333] {
                    let candidate_amount = start_amount * scale;
                    let d = error(shape, candidate_amount);
                    if d < distance {
                        best = shape;
                        amount = candidate_amount;
                        distance = d;
                    }
                }
            }
        }
    }
    best.map(|v| v * amount)
}

pub(super) fn encode(rgb: Rgb, hiding: f32) -> [f32; super::LATENT_SIZE] {
    static CACHE: std::sync::OnceLock<
        std::sync::Mutex<std::collections::HashMap<[u32; 4], [f32; super::LATENT_SIZE]>>,
    > = std::sync::OnceLock::new();
    let cache = CACHE.get_or_init(Default::default);
    let key = [
        rgb[0].to_bits(),
        rgb[1].to_bits(),
        rgb[2].to_bits(),
        hiding.to_bits(),
    ];
    if let Some(lat) = cache.lock().unwrap().get(&key) {
        return *lat;
    }
    let scatter = crate::pigment::scatter_for(luminance(rgb), hiding);
    let mut vertices = [[0.0; super::LATENT_SIZE]; super::PIGMENTS];
    let means: [f32; super::PIGMENTS] = std::array::from_fn(|i| {
        vertices[i][i] = 1.0;
        super::scattering9(&vertices[i])
    });
    let scatter = scatter.clamp(
        means.iter().copied().fold(f32::INFINITY, f32::min),
        means.iter().copied().fold(0.0, f32::max),
    );
    let target = to_oklab(rgb);
    let error = |lat: &[f32; super::LATENT_SIZE]| {
        let color = to_oklab(super::decode9(lat));
        (0..3).map(|k| (color[k] - target[k]).powi(2)).sum::<f32>()
    };
    let mut candidates = Vec::new();
    for i in 0..super::PIGMENTS {
        for j in i + 1..super::PIGMENTS {
            if scatter < means[i].min(means[j])
                || scatter > means[i].max(means[j])
                || means[i] == means[j]
            {
                continue;
            }
            let t = (scatter - means[i]) / (means[j] - means[i]);
            let mut lat = [0.0; super::LATENT_SIZE];
            lat[i] = 1.0 - t;
            lat[j] = t;
            candidates.push(lat);
        }
    }
    let mut best = *candidates
        .iter()
        .min_by(|a, b| error(a).total_cmp(&error(b)))
        .unwrap();
    let mut distance = error(&best);
    for _ in 0..4 {
        let start = best;
        for candidate in &candidates {
            for t in [0.125, 0.25, 0.5, 1.0] {
                let lat = std::array::from_fn(|k| start[k] * (1.0 - t) + candidate[k] * t);
                let d = error(&lat);
                if d < distance {
                    best = lat;
                    distance = d;
                }
            }
        }
    }
    let base = super::decode9(&best);
    for k in 0..3 {
        best[super::PIGMENTS + k] = rgb[k] - base[k];
    }
    let scatter = super::scattering9(&best);
    let (mut lo, mut hi) = (-9.0f32, 9.0f32);
    for _ in 0..40 {
        let mid = 0.5 * (lo + hi);
        let scatter = scatter * mid.exp();
        let black = luminance(super::over9(&best, scatter, [0.0; 3], 1.0));
        let white = luminance(super::over9(&best, scatter, [1.0; 3], 1.0));
        if black / white.max(1e-6) < hiding.clamp(1e-4, 0.9995) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let amount = (0.5 * (lo + hi)).exp();
    best.iter_mut().for_each(|c| *c *= amount);
    cache.lock().unwrap().insert(key, best);
    best
}

#[cfg(test)]
mod tests {
    use crate::color::{luminance, to_oklab};

    #[test]
    fn joint_earth_fits_keep_the_frozen_white_tint_ladders() {
        let mut palette = crate::Palette::new("earth tint study", crate::palette::catalog());
        palette.engine = crate::PIGMENT_MIXER;
        palette.glaze_film = true;
        let mut legacy = palette.clone();
        legacy.engine = 5;
        let at = |name| palette.tubes.iter().position(|t| t.name == name).unwrap();
        let white = at("lead white");
        let distance = |a, b| {
            let (a, b) = (to_oklab(a), to_oklab(b));
            (0..3).map(|k| (a[k] - b[k]).powi(2)).sum::<f32>().sqrt()
        };
        let legacy_white = legacy.pile(vec![(white, 1.0)]).color;
        for (name, ladder, mid, lightness) in [
            ("yellow ochre", [0.047, 0.103, 0.137, 0.193, 0.245], 0.1947, 0.806),
            ("raw umber", [0.055, 0.145, 0.204, 0.301, 0.394], 0.1824, 0.688),
        ] {
            let tube = at(name);
            for (fraction, expected_distance) in [0.01, 0.05, 0.1, 0.25, 0.5].into_iter().zip(ladder) {
                let parts = vec![(white, 1.0 - fraction), (tube, fraction)];
                let expected = legacy.pile(parts.clone()).color;
                assert!((distance(expected, legacy_white) - expected_distance).abs() < 0.0015, "{name}, {fraction}: frozen STIL-271 ladder");
                let mixture = palette.pile(parts);
                let delta = distance(mixture.color, expected);
                assert!(delta < 0.015, "{name}, {fraction}: palette tint delta {delta}");
                let delta = distance(mixture.paint(0.0).over([0.85; 3], 4.8), expected);
                assert!(delta < 0.015, "{name}, {fraction}: 120 um film tint delta {delta}");
            }
            let color = palette.pile(vec![(white, 1.0 - mid), (tube, mid)]).color;
            assert!((to_oklab(color)[0] - lightness).abs() < 0.015, "{name}: mid-tint lightness");
            let latent = super::tube(name).unwrap();
            let scatter = crate::mixer::scattering9(&latent);
            let black = luminance(crate::mixer::over9(&latent, scatter, [0.0; 3], 1.0));
            let white = luminance(crate::mixer::over9(&latent, scatter, [1.0; 3], 1.0));
            assert!((black / white - 0.8).abs() < 0.01, "{name}: estimated 25 um hiding");
        }
    }

    #[test]
    fn cadmium_covers_dark_paint_and_green_earth_keeps_backing_contrast() {
        let mut palette = crate::Palette::new("opacity study", crate::palette::catalog());
        palette.engine = crate::PIGMENT_MIXER;
        palette.glaze_film = true;
        let at = |name| palette.tubes.iter().position(|t| t.name == name).unwrap();
        let red = palette.pile(vec![(at("cadmium red"), 1.0)]).paint(0.0);
        let green = palette.pile(vec![(at("green earth"), 1.0)]).paint(0.0);
        let ratio = |paint: &crate::Paint, coats| {
            luminance(paint.over([0.01; 3], coats)) / luminance(paint.over([0.85; 3], coats))
        };
        assert!(
            ratio(&red, 4.0) > 0.8,
            "100 um cadmium contrast ratio {}",
            ratio(&red, 4.0)
        );
        assert!(
            ratio(&green, 1.0) < 0.5,
            "25 um green earth contrast ratio {}",
            ratio(&green, 1.0)
        );
        let reference = to_oklab(palette.tubes[at("cadmium red")].color);
        let covered = to_oklab(red.over([0.01; 3], 4.0));
        assert!(
            (0..3)
                .map(|k| (covered[k] - reference[k]).powi(2))
                .sum::<f32>()
                .sqrt()
                < 0.05
        );
        let tube = palette.tubes[at("cadmium red")].clone();
        let ratios: Vec<_> = [0.01, 0.99]
            .into_iter()
            .map(|hiding| {
                let mut tube = tube.clone();
                tube.hiding = hiding;
                let mut palette = crate::Palette::new("opacity control", vec![tube]);
                palette.engine = crate::PIGMENT_MIXER;
                palette.glaze_film = true;
                ratio(&palette.pile(vec![(0, 1.0)]).paint(0.0), 1.0)
            })
            .collect();
        assert!(ratios[1] > ratios[0] + 0.5, "hiding controls {ratios:?}");
    }
}
