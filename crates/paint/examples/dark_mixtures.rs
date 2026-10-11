use paint::{Palette, Pigment, color::{linear_to_srgb, luminance}};

fn main() {
    let palette = Palette::named_box("hopper").unwrap();
    let recipes: &[(&str, &[(&str, f32)])] = &[
        ("bone_black", &[("bone black", 1.0)]),
        ("ultramarine", &[("ultramarine blue", 1.0)]),
        ("viridian", &[("viridian", 1.0)]),
        ("street3", &[("bone black", 2.5), ("ultramarine blue", 1.0), ("burnt sienna", 0.6), ("viridian", 0.2), ("lead white", 0.3)]),
        ("streetD", &[("bone black", 3.0), ("ultramarine blue", 1.0), ("burnt sienna", 0.7), ("viridian", 0.2), ("lead white", 0.15)]),
        ("fdark", &[("bone black", 3.0), ("ultramarine blue", 0.8), ("burnt sienna", 0.4), ("lead white", 0.1)]),
        ("skyT2", &[("ultramarine blue", 2.0), ("viridian", 1.1), ("bone black", 3.2), ("lead white", 0.45)]),
    ];
    println!("recipe,white,coats,masstone_linear_y,masstone_encoded_y,over_white_linear_y,over_white_encoded_y,over_white_haze_encoded_y,white_ground_contribution_linear_y");
    for &(name, recipe) in recipes {
        for keep_white in [true, false] {
            let parts: Vec<_> = recipe.iter().filter(|(n, _)| keep_white || *n != "lead white").map(|&(name, amount)| {
                (palette.tubes.iter().position(|t| t.name == name).unwrap(), amount)
            }).collect();
            let sum: f32 = parts.iter().map(|p| p.1).sum();
            let paint = palette.pile(parts.into_iter().map(|(i, f)| (i, f / sum)).collect()).paint(0.05);
            let pigment = Pigment::masstone(paint.color, paint.scatter);
            let t = ((paint.oil - 0.15) / (1.3 - 0.15)).clamp(0.0, 1.0);
            let gloss = t * t * (3.0 - 2.0 * t);
            let haze = 0.04 * (1.0 - gloss);
            for coats in [0.1, 0.25, 0.5, 1.0, 2.0, 5.0, 10.0, 40.0] {
                let white = pigment.over([1.0; 3], coats);
                let black = pigment.over([0.0; 3], coats);
                let seen = white.map(|v| v * (1.0 - haze) + haze);
                println!("{name},{keep_white},{coats},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9}", luminance(paint.color), luminance(paint.color.map(linear_to_srgb)), luminance(white), luminance(white.map(linear_to_srgb)), luminance(seen.map(linear_to_srgb)), luminance(white) - luminance(black));
            }
        }
    }
}
