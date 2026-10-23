use crate::color::{Rgb, luminance, to_oklab};

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
    fn customized_earth_inputs_rebuild_their_optics_through_clones() {
        for name in ["yellow ochre", "raw umber"] {
            let tube = crate::palette::catalog().into_iter().find(|t| t.name == name).unwrap();
            let mut original = crate::Palette::new("custom earth", vec![tube]);
            original.engine = crate::PIGMENT_MIXER;
            original.glaze_film = true;
            let mut ratios = Vec::new();
            for hiding in [0.01, 0.99] {
                original.tubes[0].hiding = hiding;
                let cloned = original.clone();
                let mut fresh = crate::Palette::new("custom earth", original.tubes.clone());
                fresh.engine = crate::PIGMENT_MIXER;
                fresh.glaze_film = true;
                let paint = cloned.pile(vec![(0, 1.0)]).paint(0.0);
                let expected = fresh.pile(vec![(0, 1.0)]).paint(0.0);
                assert_eq!(paint.latent(), expected.latent());
                assert_eq!(paint.scatter, expected.scatter);
                ratios.push(luminance(paint.over([0.01; 3], 1.0)) / luminance(paint.over([0.85; 3], 1.0)));
            }
            assert!(ratios[1] > ratios[0] + 0.5, "{name}: supplied hiding controls {ratios:?}");
            original.tubes[0].color = [0.02, 0.1, 0.3];
            let color = original.clone().pile(vec![(0, 1.0)]).color;
            assert!((0..3).all(|k| (color[k] - original.tubes[0].color[k]).abs() < 1e-6), "{name}: supplied RGB {color:?}");
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
        let reference = to_oklab(red.color);
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
                tube.name = "unnamed opacity control";
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
