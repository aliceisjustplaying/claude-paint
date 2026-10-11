use paint::{Canvas, Gesture, Held, Linen, Paint, Rgb, Tool, color::{hex, linear_to_srgb}, state_dump::Values};
use std::{fs, path::Path, time::Instant};

fn scene(width: usize, name: &str) -> Canvas {
    let mut c = Canvas::new_window(width, 2.0, hex("#cdc2af"), None)
        .with_size_mm(100.0)
        .with_linen(Linen { crown_um: 12.0, ..Linen::fine(23) });
    let mut brush = Held::new(Tool::hog_flat(140.0), 19).with_engine(5);
    let stiffness = if name == "stiff" { 0.95 } else { 0.65 };
    brush.load(Paint::body(hex("#bb652d")).with_stiff(stiffness), 0.7);
    if name == "spent" { brush.wipe(0.96); }
    c.drag(&mut brush, &Gesture::line((140.0, 240.0), (860.0, 260.0)).pressure(0.8, 0.8).shake(0.0), None);
    if name == "crossing" {
        let mut blue = Held::new(Tool::hog_flat(110.0), 71).with_engine(5);
        blue.load(Paint::body(hex("#325a8f")).with_stiff(stiffness), 0.5);
        c.drag(&mut blue, &Gesture::line((360.0, 90.0), (650.0, 390.0)).pressure(0.8, 0.8).shake(0.0), None);
    }
    c
}

fn save(path: &Path, c: &Canvas, pixels: &[Rgb]) {
    let f = c.window();
    let rgb: Vec<u8> = pixels.iter().flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8)).collect();
    image::save_buffer(path, &rgb, f.w as u32, f.h as u32, image::ColorType::Rgb8).unwrap();
}

fn field(c: &Canvas, name: &str) -> Vec<f32> {
    c.state_fields().into_iter().find_map(|f| match (f.name == name, f.values) {
        (true, Values::F32(v)) => Some(v), _ => None,
    }).unwrap()
}

fn main() {
    let directory = std::env::args().nth(1).expect("body_relaxation <output-directory>");
    let out = Path::new(&directory);
    fs::create_dir_all(out).unwrap();
    println!("scene,width,mode,stage,steps,wet_total,peak_wet_um,film_plus_wet,seconds");
    for width in [160, 320] {
        for name in ["loaded", "stiff", "spent", "crossing"] {
            let baseline = scene(width, name);
            for mode in ["baseline", "poc"] {
                let mut c = baseline.clone();
                let start = Instant::now();
                let before = c.wet_total();
                let steps = if mode == "poc" { c.relax_fresh_body(2.0).unwrap() } else { 0 };
                assert!((c.wet_total() / before - 1.0).abs() < 2e-5);
                let elapsed = start.elapsed().as_secs_f64();
                for stage in ["fresh", "dry"] {
                    if stage == "dry" { c.dry(); }
                    let v = field(&c, "wet.vol");
                    let mass = v.iter().zip(field(&c, "film")).map(|(&a, b)| (a + b) as f64).sum::<f64>() / (c.window().scale as f64).powi(2);
                    assert!((mass / before - 1.0).abs() < 2e-4);
                    let peak = v.iter().copied().fold(0.0, f32::max) * paint::COAT_UM;
                    println!("{name},{width},{mode},{stage},{steps},{:.9},{peak:.6},{mass:.9},{elapsed:.6}", c.wet_total());
                    for (view, pixels) in [("color", c.seen()), ("gallery", c.seen_lit(115.0, 55.0, 1.0)), ("raking", c.seen_lit(135.0, 25.0, 1.0))] {
                        save(&out.join(format!("{name}-{width}-{mode}-{stage}-{view}.png")), &c, &pixels);
                    }
                    let height: Vec<f32> = c.surface_um().iter().zip(&v).map(|(&h, &v)| h + v * paint::COAT_UM).collect();
                    let bytes: Vec<u8> = height.iter().flat_map(|v| v.to_le_bytes()).collect();
                    fs::write(out.join(format!("{name}-{width}-{mode}-{stage}-height.f32")), bytes).unwrap();
                }
            }
        }
    }
}
