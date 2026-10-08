//! Small reproducible candidate; no easel, reference assets or new lighting.
use paint::{Canvas, Crop, Gesture, Held, Linen, Paint, Rgb, Tool, color::{hex, linear_to_srgb}, state_dump::Values};
use std::{fs, path::Path, time::Instant};

fn draw(c: &mut Canvas, h: &mut Held, g: &Gesture, release: Option<f32>) {
    if let Some(scale) = release { c.drag_bristle_folds_release_study(h, g, None, scale); } else { c.drag(h, g, None); }
}

fn support(w: usize, crop: Option<Crop>) -> Canvas {
    Canvas::new_window(w, 2.0, hex("#d4cbc0"), crop).with_size_mm(100.0)
        .with_linen(Linen { crown_um: 12.0, ..Linen::fine(23) })
}

fn scene(w: usize, name: &str, release: Option<f32>, crop: Option<Crop>) -> (Canvas, Vec<Held>, f64) {
    let mut c = support(w, crop);
    let mut h = Held::new(Tool::hog_flat(140.0), 19).with_engine(5);
    h.load(Paint::body(hex("#bb652d")).with_stiff(0.95), 0.9);
    if name == "spent" { h.wipe(0.96); }
    let mut total = h.carried().0;
    let g = Gesture::line((140.0, 240.0), (860.0, 260.0)).pressure(0.8, 0.8).shake(0.0);
    if name == "curve" {
        draw(&mut c, &mut h, &Gesture::new(vec![(140.0, 250.0), (330.0, 160.0), (540.0, 330.0), (860.0, 230.0)])
            .pressure(0.25, 0.9).swell(vec![1.0, 0.6, 1.2, 0.8]).shake(0.0), release);
    } else { draw(&mut c, &mut h, &g, release); }
    let mut brushes = vec![h];
    if name == "crossing" {
        let mut b = Held::new(Tool::hog_flat(110.0), 71).with_engine(5);
        b.load(Paint::body(hex("#325a8f")).with_stiff(0.95), 0.5);
        total += b.carried().0;
        draw(&mut c, &mut b, &Gesture::line((360.0, 90.0), (650.0, 390.0)).pressure(0.8, 0.8).shake(0.0), release);
        // Same dirty brush: no load/reset between crossing and continuation.
        draw(&mut c, &mut b, &Gesture::line((680.0, 370.0), (900.0, 380.0)).pressure(0.65, 0.65).shake(0.0), release);
        brushes.push(b);
    }
    (c, brushes, total)
}

fn bytes(c: &Canvas) -> Vec<u8> {
    c.state_fields().into_iter().flat_map(|f| f.values.le_bytes()).collect()
}
fn save(path: &Path, c: &Canvas, p: &[Rgb]) {
    let f = c.window();
    let rgb: Vec<u8> = p.iter().flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8)).collect();
    image::save_buffer(path, &rgb, f.w as u32, f.h as u32, image::ColorType::Rgb8).unwrap();
}
fn wet(c: &Canvas) -> Vec<f32> {
    c.state_fields().into_iter().find_map(|f| match (f.name, f.values) {
        ("wet.vol", Values::F32(v)) => Some(v), _ => None,
    }).unwrap()
}
fn film_and_wet(c: &Canvas) -> f64 {
    let film: f64 = c.state_fields().into_iter().filter(|f| f.name == "film" || f.name == "wet.vol")
        .map(|f| match f.values { Values::F32(v) => v.iter().map(|&v| v as f64).sum::<f64>(), _ => unreachable!() }).sum();
    film / (c.window().scale as f64).powi(2)
}
fn main() {
    let out = std::env::args().nth(1).expect("bristle_folds <output-directory>");
    let out = Path::new(&out);
    fs::create_dir_all(out).unwrap();
    let mut drying = String::from("scene,width,mode,minutes,wet_before,wet_after,film_plus_wet_before,film_plus_wet_after,relative_error\n");
    println!("scene,width,mode,initial,canvas,brushes,relative_mass_error,coverage,max_wet_um,seconds");
    let sweep = std::env::args().nth(2).map(|s| { assert_eq!(s, "--release-sweep"); true }).unwrap_or(false);
    let modes = if sweep { vec![(None, "baseline"), (Some(1.0), "candidate"), (Some(0.9), "release-0.9"), (Some(0.8), "release-0.8")] }
        else { vec![(None, "baseline"), (Some(1.0), "candidate")] };
    for w in [160, 320] {
        for name in ["loaded", "spent", "curve", "crossing"] {
            for &(release, mode) in &modes {
                let start = Instant::now();
                let (c, brushes, initial) = scene(w, name, release, None);
                let paint = c.wet_total();
                let carried: f64 = brushes.iter().map(|b| b.carried().0).sum();
                let error = (paint + carried) / initial - 1.0;
                assert!(error.abs() < 2e-5);
                let v = wet(&c);
                let cover = v.iter().filter(|&&v| v > 0.01).count() as f64 / v.len() as f64;
                let max = v.iter().copied().fold(0.0, f32::max) * paint::COAT_UM;
                let before = bytes(&c);
                let lit = c.seen_lit(315.0, 30.0, 0.9); // SAME existing renderer for both
                let flat = c.seen();
                assert_eq!(before, bytes(&c), "view must not change state");
                save(&out.join(format!("{name}-{w}-{mode}-lit.png")), &c, &lit);
                save(&out.join(format!("{name}-{w}-{mode}-flat.png")), &c, &flat);
                // Linear grayscale diagnostic: black=0, white=250 um wet film.
                let height: Vec<Rgb> = v.iter().map(|&v| [((v * paint::COAT_UM) / 250.0).clamp(0.0, 1.0); 3]).collect();
                save(&out.join(format!("{name}-{w}-{mode}-wet-height.png")), &c, &height);
                let (repeat, repeat_brushes, _) = scene(w, name, release, None);
                assert_eq!(before, bytes(&repeat), "repeat all state exactly");
                assert_eq!(format!("{brushes:?}"), format!("{repeat_brushes:?}"));
                // Checkpoint contract: unchanged format and restored physical
                // state; continuation selects the same method explicitly.
                let mut checkpoint = Vec::new();
                c.write_state(&mut checkpoint, "bristle-folds explicit-call example").unwrap();
                let (mut restored, _) = Canvas::read_state(&mut checkpoint.as_slice()).unwrap();
                let mut continued = c.clone();
                let (mut a, mut b) = (brushes[0].clone(), brushes[0].clone());
                let next = Gesture::line((180.0, 420.0), (400.0, 430.0)).shake(0.0);
                draw(&mut continued, &mut a, &next, release);
                draw(&mut restored, &mut b, &next, release);
                let (mut a_state, mut b_state) = (Vec::new(), Vec::new());
                continued.write_state(&mut a_state, "").unwrap();
                restored.write_state(&mut b_state, "").unwrap();
                assert_eq!(a_state, b_state, "checkpoint continuation saved state");
                assert_eq!(continued.seen(), restored.seen());
                assert_eq!(format!("{a:?}"), format!("{b:?}"));
                let mut aged = c.clone();
                let film_before = film_and_wet(&aged);
                aged.wait(48.0 * 60.0);
                let film_after = film_and_wet(&aged);
                let aging_error = film_after / film_before - 1.0;
                assert!(aging_error.abs() < 1e-4, "dry film + wet budget: {aging_error}");
                assert!(aged.surface_um().iter().all(|h| h.is_finite()));
                assert!(aged.wet_total() <= paint * (1.0 + 1e-5));
                drying.push_str(&format!("{name},{w},{mode},2880,{paint:.9},{:.9},{film_before:.9},{film_after:.9},{aging_error:.9}\n", aged.wet_total()));
                println!("{name},{w},{mode},{initial:.9},{paint:.9},{carried:.9},{error:.9},{cover:.6},{max:.6},{:.3}", start.elapsed().as_secs_f64());
            }
        }
    }
    fs::write(out.join("drying.csv"), drying).unwrap();
    // Crop keeps 150 units of context: equality is measured below, not
    // promised for a window that omits previous wet paint or contacted cloth.
    let crop = Crop { units: [250.0, 160.0, 700.0, 330.0], margin: 150.0 };
    for release in [1.0, 0.9, 0.8] {
        let (whole, _, _) = scene(320, "loaded", Some(release), None);
        let (cropped, _, _) = scene(320, "loaded", Some(release), Some(crop));
        let a = wet(&whole);
        let b = wet(&cropped);
        let f = cropped.window();
        let mut max_delta = 0.0f32;
        for y in 70..95 { for x in 95..210 {
            max_delta = max_delta.max((a[y * 320 + x] - b[(y - f.y0) * f.w + x - f.x0]).abs());
        }}
        assert!(max_delta < 0.002, "crop interior delta {max_delta}");
        eprintln!("repeat_state=exact repeat_brushes=exact view_state=unchanged checkpoint_continuation=exact release={release} crop_max_delta_coats={max_delta}");
    }
}
