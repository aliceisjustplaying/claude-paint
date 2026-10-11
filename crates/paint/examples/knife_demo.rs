#[cfg(not(feature = "knife-2"))]
fn main() {
    eprintln!("knife_demo requires --features knife-2");
}

#[cfg(feature = "knife-2")]
mod demo {
    use paint::color::linear_to_srgb;
    use paint::{Canvas, Knife, Linen, Paint, Rgb, hex};
    use std::{fs, path::Path};

    struct Pass {
        width: f32,
        paint: Paint,
        amount: f32,
        points: &'static [(f32, f32)],
        pressure: (f32, f32),
        tilt: f32,
        angle: Option<f32>,
        lift: f32,
        lay: bool,
    }

    fn apply(c: &mut Canvas, model: u8, p: Pass) {
        let mut k = Knife::new(p.width);
        if model == 2 {
            k = k.experimental_2(0.65);
        }
        #[cfg(feature = "knife-3")]
        if model == 3 {
            k = k.experimental_3(0.65);
        }
        k.load(p.paint, p.amount);
        if model != 0 {
            c.knife_1(&mut k, p.points, p.pressure, p.angle, p.lay, p.lift, p.tilt);
        } else {
            c.knife(&mut k, p.points, p.pressure, p.angle, p.lay, p.lift);
        }
    }

    fn study(mut c: Canvas, model: u8) -> Canvas {
        apply(
            &mut c,
            model,
            Pass {
                width: 200.0,
                paint: Paint::body(hex("#35556a")),
                amount: 1.0,
                points: &[(120.0, 215.0), (520.0, 185.0), (875.0, 225.0)],
                pressure: (0.45, 0.6),
                tilt: 15.0,
                angle: None,
                lift: 0.14,
                lay: true,
            },
        );
        c.dry();
        apply(
            &mut c,
            model,
            Pass {
                width: 170.0,
                paint: Paint::body(hex("#b3673e")),
                amount: 0.72,
                points: &[(285.0, 315.0), (500.0, 292.0), (740.0, 320.0)],
                pressure: (0.35, 0.45),
                tilt: 20.0,
                angle: None,
                lift: 0.16,
                lay: true,
            },
        );
        apply(
            &mut c,
            model,
            Pass {
                width: 60.0,
                paint: Paint::body(hex("#b3673e")),
                amount: 0.0,
                points: &[(380.0, 292.0), (660.0, 300.0)],
                pressure: (1.0, 1.0),
                tilt: 75.0,
                angle: None,
                lift: 0.0,
                lay: false,
            },
        );
        c.dry();
        apply(
            &mut c,
            model,
            Pass {
                width: 180.0,
                paint: Paint::new(hex("#baa452"), 0.25, 0.1).with_thinner(0.35),
                amount: 0.16,
                points: &[(155.0, 180.0), (515.0, 160.0), (800.0, 185.0)],
                pressure: (0.75, 0.82),
                tilt: 10.0,
                angle: None,
                lift: 0.0,
                lay: true,
            },
        );
        apply(
            &mut c,
            model,
            Pass {
                width: 130.0,
                paint: Paint::body(hex("#a28f70")),
                amount: 0.9,
                points: &[(220.0, 520.0), (440.0, 500.0), (670.0, 528.0)],
                pressure: (0.18, 0.32),
                tilt: 10.0,
                angle: None,
                lift: 0.4,
                lay: true,
            },
        );
        apply(
            &mut c,
            model,
            Pass {
                width: 65.0,
                paint: Paint::body(hex("#39534f")),
                amount: 0.18,
                points: &[(155.0, 420.0), (800.0, 420.0)],
                pressure: (0.4, 0.6),
                tilt: 80.0,
                angle: Some(0.0),
                lift: 0.02,
                lay: true,
            },
        );
        c
    }

    fn save(path: &Path, c: &Canvas, pixels: Vec<Rgb>) -> Result<(), image::ImageError> {
        assert!(
            pixels.iter().flatten().all(|v| v.is_finite()),
            "finite demo pixels: {}",
            path.display()
        );
        let bytes: Vec<u8> = pixels
            .iter()
            .flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8))
            .collect();
        let f = c.window();
        image::save_buffer(path, &bytes, f.w as u32, f.h as u32, image::ColorType::Rgb8)
    }

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let output = std::env::args()
            .nth(1)
            .ok_or("knife_demo <output-directory>")?;
        let output = Path::new(&output);
        fs::create_dir_all(output)?;
        let mut initial = Canvas::new(1200, 1.5, hex("#d2c8af"))
            .with_size_mm(300.0)
            .with_engine(5)
            .with_linen(Linen::fine(41));
        initial.prime(hex("#dfd3ba"), 0.9, 90.0, 0.6, 0.25, 44);
        #[cfg(not(feature = "knife-3"))]
        let models = &[("baseline", 0), ("knife-2", 2)][..];
        #[cfg(feature = "knife-3")]
        let models = &[("baseline", 0), ("knife-2", 2), ("knife-3", 3)][..];
        for &(name, model) in models {
            let c = study(initial.clone(), model);
            save(&output.join(format!("{name}-color.png")), &c, c.seen())?;
            save(
                &output.join(format!("{name}-gallery.png")),
                &c,
                c.seen_lit(115.0, 55.0, 1.0),
            )?;
        }
        Ok(())
    }
}

#[cfg(feature = "knife-2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    demo::run()
}
