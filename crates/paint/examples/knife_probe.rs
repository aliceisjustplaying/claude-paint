#[cfg(not(feature = "knife-1"))]
fn main() {
    eprintln!("knife_probe requires --features knife-1");
}

#[cfg(feature = "knife-1")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use paint::color::linear_to_srgb;
    use paint::state_dump::Values;
    use paint::{Canvas, Gesture, Held, Knife, Linen, Paint, Rgb, Tool, hex};
    use std::{fs, io::Write, path::Path};

    let output = std::env::args()
        .nth(1)
        .ok_or("knife_probe <output-directory>")?;
    let output = Path::new(&output);
    fs::create_dir_all(output)?;
    let mut csv = fs::File::create(output.join("volume.csv"))?;
    writeln!(
        csv,
        "probe,model,paint_before_mm3,paint_after_mm3,paint_error_mm3,solvent_before_mm3,solvent_after_mm3,solvent_error_mm3,canvas_wet_mm3,blade_paint_mm3,peak_wet_um,blade_r,blade_g,blade_b"
    )?;
    let field = |c: &Canvas, name: &str| -> Vec<f32> {
        let f = c
            .state_fields()
            .into_iter()
            .find(|f| f.name == name)
            .unwrap();
        match f.values {
            Values::F32(v) => v,
            _ => unreachable!(),
        }
    };
    let save = |path: &Path, c: &Canvas, pixels: Vec<Rgb>| -> Result<(), image::ImageError> {
        let bytes: Vec<u8> = pixels
            .iter()
            .flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8))
            .collect();
        let f = c.window();
        image::save_buffer(path, &bytes, f.w as u32, f.h as u32, image::ColorType::Rgb8)
    };
    for probe in ["slab", "edge", "scrape", "veil", "wet-mix", "ridges"] {
        let mut initial = Canvas::new(640, 2.0, hex("#d6cebb"))
            .with_size_mm(300.0)
            .with_engine(5)
            .with_linen(Linen::fine(23));
        initial.prime(hex("#e2d7bf"), 0.9, 65.0, 0.6, 0.35, 17);
        if probe == "veil" {
            initial.prime(hex("#294867"), 0.9, 100.0, 0.6, 0.08, 29);
        }
        if ["scrape", "wet-mix"].contains(&probe) {
            let mut h = Held::new(Tool::hog_flat(140.0), 31);
            h.load(
                Paint::body(hex("#294867")),
                if probe == "wet-mix" { 0.5 } else { 0.9 },
            );
            let end = if probe == "wet-mix" { 480.0 } else { 850.0 };
            initial.drag(
                &mut h,
                &Gesture::line((160.0, 250.0), (end, 250.0)).pressure(0.6, 0.6),
                None,
            );
            if probe != "wet-mix" {
                initial.dry();
            }
        }
        if probe == "scrape" {
            let mut h = Held::new(Tool::hog_flat(105.0), 38);
            h.load(Paint::body(hex("#c27742")), 0.8);
            initial.drag(
                &mut h,
                &Gesture::line((220.0, 250.0), (800.0, 250.0)).pressure(0.7, 0.7),
                None,
            );
        }
        let dry_film = field(&initial, "film")
            .iter()
            .map(|&v| v as f64)
            .sum::<f64>()
            / (0.64 * 0.64);
        let dry_height = field(&initial, "height");
        let dry_color = field(&initial, "color");
        #[cfg(not(feature = "knife-2"))]
        let models = &["baseline", "knife-1"][..];
        #[cfg(feature = "knife-2")]
        let models = &["baseline", "knife-1", "knife-2"][..];
        for &model in models {
            let mut c = initial.clone();
            let width = if probe == "edge" { 75.0 } else { 110.0 };
            let mut k = Knife::new(width);
            if model == "knife-1" {
                k = k.experimental(0.65);
            }
            #[cfg(feature = "knife-2")]
            if model == "knife-2" {
                k = k.experimental_2(0.65);
            }
            let (paint, amount, pressure, angle, tilt, lift, lay) = match probe {
                "slab" => (
                    Paint::body(hex("#b55332")),
                    0.9,
                    (0.35, 0.5),
                    None,
                    15.0,
                    0.08,
                    true,
                ),
                "edge" => (
                    Paint::body(hex("#b55332")),
                    0.08,
                    (0.5, 0.65),
                    Some(0.0),
                    80.0,
                    0.05,
                    true,
                ),
                "scrape" => (
                    Paint::body(hex("#b55332")),
                    0.0,
                    (1.0, 1.0),
                    None,
                    75.0,
                    0.0,
                    false,
                ),
                "veil" => (
                    Paint::new(hex("#c8a647"), 0.25, 0.1).with_thinner(0.35),
                    0.10,
                    (0.75, 0.8),
                    None,
                    10.0,
                    0.0,
                    true,
                ),
                "wet-mix" => (
                    Paint::body(hex("#d8b757")),
                    0.3,
                    (0.65, 0.8),
                    None,
                    25.0,
                    0.04,
                    true,
                ),
                _ => (
                    Paint::body(hex("#a89a7e")),
                    1.0,
                    (0.15, 0.3),
                    None,
                    10.0,
                    0.35,
                    true,
                ),
            };
            k.load(paint, amount);
            let factor = 0.025 * 0.3 * 0.3;
            let active_before = (c.wet_total() + k.paint_volume()) * factor;
            let before_paint = dry_film * factor + active_before;
            let before_solvent = (c.solvent_total() + k.solvent_volume()) * factor;
            let path = if probe == "ridges" {
                vec![(240.0, 250.0), (430.0, 250.0)]
            } else if probe == "slab" {
                vec![(190.0, 250.0), (400.0, 248.0), (540.0, 250.0)]
            } else {
                vec![(190.0, 250.0), (450.0, 248.0), (790.0, 250.0)]
            };
            if model != "baseline" {
                c.knife_1(&mut k, &path, pressure, angle, lay, lift, tilt);
            } else {
                c.knife(&mut k, &path, pressure, angle, lay, lift);
            }
            assert_eq!(
                dry_height,
                field(&c, "height"),
                "the knife protects the set support"
            );
            assert_eq!(
                dry_color,
                field(&c, "color"),
                "the knife protects set color"
            );
            let after_paint = (dry_film + c.wet_total() + k.paint_volume()) * factor;
            let after_solvent = (c.solvent_total() + k.solvent_volume()) * factor;
            if model != "baseline" {
                assert!((before_paint - after_paint).abs() < active_before.max(1.0) * 3e-6);
                assert!((before_solvent - after_solvent).abs() < before_solvent.max(1.0) * 3e-6);
            }
            let peak = field(&c, "wet.vol").into_iter().fold(0.0f32, f32::max) * 25.0;
            let color = k.paint().map_or([0.0; 3], |p| p.color);
            writeln!(
                csv,
                "{probe},{model},{before_paint:.9},{after_paint:.9},{:.9},{before_solvent:.9},{after_solvent:.9},{:.9},{:.9},{:.9},{peak:.6},{:.6},{:.6},{:.6}",
                after_paint - before_paint,
                after_solvent - before_solvent,
                c.wet_total() * factor,
                k.paint_volume() * factor,
                color[0],
                color[1],
                color[2]
            )?;
            for (view, pixels) in [
                ("color", c.seen()),
                ("gallery", c.seen_lit(115.0, 55.0, 1.0)),
                ("relief", c.seen_lit(135.0, 25.0, 1.0)),
            ] {
                save(
                    &output.join(format!("{probe}-{model}-{view}.png")),
                    &c,
                    pixels,
                )?;
            }
        }
    }
    Ok(())
}
