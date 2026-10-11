use paint::{Canvas, Gesture, Held, LookVersion, Paint, Tool, color::linear_to_srgb};
use std::path::Path;

fn save(c: &Canvas, path: &Path, pixels: &[paint::Rgb]) -> Result<(), Box<dyn std::error::Error>> {
    let f = c.window();
    let bytes: Vec<u8> = pixels.iter().flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8)).collect();
    image::save_buffer(path, &bytes, f.w as u32, f.h as u32, image::ColorType::Rgb8)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::env::args().nth(1).ok_or("gloss_light_probe <output-directory>")?;
    let states = std::env::args().nth(2).map(std::path::PathBuf::from);
    let out = Path::new(&out);
    std::fs::create_dir_all(out)?;
    for (name, polished, raised, wet) in [
        ("flat-matte", false, false, false),
        ("flat-glossy", true, false, false),
        ("raised-matte", false, true, false),
        ("raised-glossy", true, true, false),
        ("fresh-dark", false, false, true),
    ] {
        let state = if let Some(states) = &states {
            std::fs::read(states.join(format!("{name}.ckpt")))?
        } else {
            let dark = [0.02, 0.015, 0.01];
            let mut c = Canvas::new_window(320, 1.0, dark, None).with_size_mm(50.0);
            c.ground_finish(1.0);
            if raised {
                c.prime(dark, 0.92, 250.0, 1.0, 1.0, 91);
            }
            if polished {
                c.varnished();
            }
            if wet {
                let mut brush = Held::new(Tool::hog_flat(90.0), 23);
                brush.load(Paint::body(dark), 1.0);
                c.drag(&mut brush, &Gesture::line((100.0, 400.0), (900.0, 600.0)).pressure(0.8, 0.8), None);
            }
            let mut state = Vec::new();
            c.write_state(&mut state, name)?;
            state
        };
        std::fs::write(out.join(format!("{name}.ckpt")), &state)?;
        let (restored, _) = Canvas::read_state(&mut state.as_slice())?;
        save(&restored, &out.join(format!("{name}-color.png")), &restored.seen())?;
        for (version, label) in [(LookVersion::Legacy, "before"), (LookVersion::Gloss, "after")] {
            save(&restored, &out.join(format!("{name}-gallery-{label}.png")), &restored.seen_lit_version(115.0, 55.0, 1.0, version))?;
            save(&restored, &out.join(format!("{name}-relief-{label}.png")), &restored.seen_lit_version(135.0, 25.0, 1.0, version))?;
            save(&restored, &out.join(format!("{name}-museum-{label}.png")), &restored.seen_museum_version(0.18, version))?;
        }
        let mut after = Vec::new();
        restored.write_state(&mut after, name)?;
        assert_eq!(state, after, "rendering mutated {name}");
        println!("{name}: saved-state views, Color exact, material bytes exact");
    }
    Ok(())
}
