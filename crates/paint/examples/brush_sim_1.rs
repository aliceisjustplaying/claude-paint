use paint::{Canvas, Gesture, Held, Linen, Paint, Tool, hex};
use paint::color::linear_to_srgb;
use std::{fs, path::Path, time::Instant};

fn pull(c: &mut Canvas, engine: u32, seed: u64, color: &str, load: f32, g: Gesture, width: f32, budget: &mut (f64, f64)) {
    let mut h = Held::new(Tool::hog_flat(width), seed).with_engine(engine);
    h.load(Paint::body(hex(color)).with_stiff(0.92), load);
    budget.0 += h.carried().0;
    c.drag(&mut h, &g, None);
    budget.1 += h.carried().0;
}

fn scene(name: &str, engine: u32) -> (Canvas, f64, f64, f64, f64) {
    let mut c = Canvas::new(640, 2.0, hex("#bcb3a2")).with_size_mm(160.0).with_engine(engine).with_linen(Linen::fine(1906));
    c.prime(hex("#bcb3a2"), 0.92, 100.0, 0.9, 0.3, 1906);
    if name == "scumble" {
        let mut b = (0.0, 0.0);
        for y in [145.0, 220.0, 295.0, 370.0] {
            pull(&mut c, engine, 31, "#465d67", 0.65, Gesture::line((60.0, y), (940.0, y)).pressure(0.8, 0.8), 130.0, &mut b);
        }
        c.wait(7.0 * 24.0 * 60.0);
    }
    let initial = c.material_total();
    let mut budget = (0.0, 0.0);
    let t = Instant::now();
    match name {
        "loaded" | "spent" => pull(&mut c, engine, 73, "#be5e32", if name == "loaded" { 0.9 } else { 0.07 }, Gesture::line((110.0, 250.0), (930.0, 250.0)).pressure(0.7, 0.7), 85.0, &mut budget),
        "curve" => pull(&mut c, engine, 73, "#be5e32", 0.75, Gesture::new(vec![(130.0, 350.0), (350.0, 160.0), (610.0, 310.0), (920.0, 180.0)]).pressure(0.2, 0.95), 85.0, &mut budget),
        "crossing" => {
            pull(&mut c, engine, 31, "#456e8a", 0.9, Gesture::line((90.0, 240.0), (930.0, 240.0)).pressure(0.75, 0.75), 110.0, &mut budget);
            let mut h = Held::new(Tool::hog_flat(85.0), 73).with_engine(engine);
            h.load(Paint::body(hex("#be5e32")).with_stiff(0.92), 0.75);
            budget.0 += h.carried().0;
            c.drag(&mut h, &Gesture::line((420.0, 70.0), (620.0, 440.0)).pressure(0.4, 0.85), None);
            c.drag(&mut h, &Gesture::line((700.0, 390.0), (940.0, 410.0)).pressure(0.6, 0.5), None);
            budget.1 += h.carried().0;
        }
        "scumble" => {
            for (i, y) in [180.0, 250.0, 320.0].iter().enumerate() {
                pull(&mut c, engine, 73 + i as u64, "#d7b87c", 0.055, Gesture::new(vec![(120.0, *y + 15.0), (440.0, *y - 17.0), (900.0, *y + 3.0)]).pressure(0.28, 0.4), 85.0, &mut budget);
            }
        }
        "passage" => {
            for (i, (a, b, color, width, load)) in [
                ((75.0, 105.0), (900.0, 135.0), "#748995", 150.0, 0.8),
                ((80.0, 185.0), (920.0, 205.0), "#8799a0", 145.0, 0.75),
                ((85.0, 265.0), (930.0, 265.0), "#607265", 155.0, 0.8),
                ((60.0, 355.0), (950.0, 350.0), "#a19061", 190.0, 0.8),
                ((80.0, 425.0), (940.0, 440.0), "#8b7851", 125.0, 0.7),
                ((140.0, 280.0), (440.0, 140.0), "#445c55", 65.0, 0.7),
                ((300.0, 210.0), (660.0, 275.0), "#526457", 90.0, 0.6),
                ((470.0, 355.0), (650.0, 280.0), "#c7b385", 60.0, 0.55),
                ((660.0, 365.0), (810.0, 380.0), "#bd7b4d", 70.0, 0.6),
                ((190.0, 380.0), (430.0, 415.0), "#dac591", 45.0, 0.15),
                ((520.0, 260.0), (570.0, 390.0), "#3c4e48", 22.0, 0.45),
                ((780.0, 275.0), (880.0, 260.0), "#c8c5a2", 35.0, 0.18),
            ].iter().enumerate() {
                pull(&mut c, engine, 73 + i as u64 * 7, color, *load, Gesture::line(*a, *b).pressure(0.6, 0.75), *width, &mut budget);
            }
        }
        _ => unreachable!(),
    }
    let secs = t.elapsed().as_secs_f64();
    let deposited = c.material_total() - initial;
    (c, deposited, budget.0, budget.1, secs)
}

fn save(c: &Canvas, dir: &Path, stem: &str) -> Result<(), Box<dyn std::error::Error>> {
    for (name, pixels) in [("color", c.seen()), ("gallery", c.seen_lit(115.0, 55.0, 1.0))] {
        let buf: Vec<u8> = pixels.iter().flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8)).collect();
        image::save_buffer(dir.join(format!("{stem}-{name}.png")), &buf, 640, 320, image::ColorType::Rgb8)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::env::args().nth(1).expect("output directory");
    let dir = Path::new(&dir);
    fs::create_dir_all(dir)?;
    let rounds = std::env::args().nth(2).map(|s| s.parse::<usize>().expect("positive timing rounds")).unwrap_or(8);
    assert!(rounds > 0);
    let variants = if cfg!(feature = "brush-sim-1") { vec![(5, "baseline"), (6, "lean"), (7, "body"), (8, "wild")] } else { vec![(5, "baseline")] };
    let mut rows = String::from("scene,variant,round,paint_used,input_paint,remaining_paint,conservation_error,time_per_chunk_s,height_mean_um,height_p95_um,height_p99_um,height_max_um\n");
    for round in 0..rounds {
        for offset in 0..variants.len() {
            let (engine, variant) = variants[(offset + round) % variants.len()];
            for name in ["loaded", "spent", "curve", "crossing", "scumble", "passage"] {
                let (c, used, input, left, secs) = scene(name, engine);
                if round == 0 { save(&c, dir, &format!("{name}-{variant}"))?; }
                let f = c.frame();
                let mut heights: Vec<f32> = c.surface_um().iter().enumerate().map(|(i, h)| *h + c.wet_um(f.ux(i % f.w), f.uy(i / f.w))).collect();
                heights.sort_by(f32::total_cmp);
                rows.push_str(&format!("{name},{variant},{round},{used},{input},{left},{},{secs},{},{},{},{}\n", used + left - input, heights.iter().map(|x| *x as f64).sum::<f64>() / heights.len() as f64, heights[heights.len() * 95 / 100], heights[heights.len() * 99 / 100], heights.last().unwrap()));
            }
        }
    }
    fs::write(dir.join("metrics.csv"), rows)?;
    Ok(())
}
