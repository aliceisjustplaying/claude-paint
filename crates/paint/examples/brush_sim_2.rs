use paint::{Canvas, Gesture, Held, Linen, Paint, Tool, hex};
use paint::color::{linear_to_srgb, luminance};
use paint::palette::Palette;
use std::{fs, path::Path, time::Instant};

fn pull(c: &mut Canvas, engine: u32, seed: u64, paint: Paint, load: f32, g: Gesture, width: f32, budget: &mut (f64, f64)) {
    let mut h = Held::new(Tool::hog_flat(width), seed).with_engine(engine);
    h.load(paint, load);
    budget.0 += h.carried().0;
    c.drag(&mut h, &g, None);
    budget.1 += h.carried().0;
}

fn scene(name: &str, engine: u32, scale: f32) -> (Canvas, f64, f64, f64, f64) {
    let ground = if name == "dark" { "#eee8da" } else { "#bcb3a2" };
    let mut c = Canvas::new(640, 2.0, hex(ground)).with_size_mm(160.0).with_engine(engine).with_linen(Linen::fine(1906));
    c.prime(hex(ground), 0.92, 100.0, 0.9, 0.3, 1906);
    let initial = c.material_total();
    let mut budget = (0.0, 0.0);
    let blue = Paint::body(hex("#456e8a")).with_stiff(0.92);
    let orange = Paint::body(hex("#be5e32")).with_stiff(0.92);
    let t = Instant::now();
    match name {
        "crossing" => {
            pull(&mut c, engine, 31, blue, 0.9 * scale, Gesture::line((90.0, 240.0), (930.0, 240.0)).pressure(0.75, 0.75), 110.0, &mut budget);
            let mut h = Held::new(Tool::hog_flat(85.0), 73).with_engine(engine);
            h.load(orange, 0.75 * scale);
            budget.0 += h.carried().0;
            c.drag(&mut h, &Gesture::line((420.0, 70.0), (620.0, 440.0)).pressure(0.4, 0.85), None);
            c.drag(&mut h, &Gesture::line((700.0, 390.0), (940.0, 410.0)).pressure(0.6, 0.5), None);
            budget.1 += h.carried().0;
        }
        "dark" => {
            let pal = Palette::tube_box();
            let black = pal.tubes.iter().position(|t| t.name == "bone black").unwrap();
            let paint = pal.pile(vec![(black, 1.0)]).paint(0.05);
            pull(&mut c, engine, 73, paint, 0.9 * scale, Gesture::line((110.0, 250.0), (930.0, 250.0)).pressure(0.85, 0.85), 110.0, &mut budget);
        }
        "wet" => {
            for y in [175.0, 275.0] {
                pull(&mut c, engine, 31, blue, 0.9 * scale, Gesture::line((360.0, y), (610.0, y)).pressure(0.85, 0.85), 130.0, &mut budget);
            }
            let mut h = Held::new(Tool::hog_flat(85.0), 73).with_engine(engine);
            h.load(orange, 0.18 * scale);
            budget.0 += h.carried().0;
            c.drag(&mut h, &Gesture::line((120.0, 250.0), (930.0, 250.0)).pressure(0.65, 0.85), None);
            c.drag(&mut h, &Gesture::line((700.0, 400.0), (940.0, 400.0)).pressure(0.65, 0.65), None);
            budget.1 += h.carried().0;
        }
        "spent" => pull(&mut c, engine, 73, orange, 0.07 * scale, Gesture::line((110.0, 250.0), (930.0, 250.0)).pressure(0.7, 0.7), 85.0, &mut budget),
        _ => unreachable!(),
    }
    let secs = t.elapsed().as_secs_f64();
    let deposited = c.material_total() - initial;
    (c, deposited, budget.0, budget.1, secs)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::env::args().nth(1).expect("output directory");
    let dir = Path::new(&output);
    fs::create_dir_all(dir)?;
    let variants = [(5, "baseline"), (7, "iteration1"), (9, "lean2"), (10, "iteration2"), (11, "wild2")];
    let names = ["crossing", "dark", "wet", "spent"];
    let matched = std::env::args().nth(2).is_some_and(|arg| arg == "--matched");
    let mut scales = [[1.0; 4]; 5];
    if matched {
        for (i, name) in names.iter().enumerate() {
            let target = scene(name, 5, 1.0).1;
            for (j, (engine, _)) in variants.iter().enumerate().skip(1) {
                let (mut lo, mut hi) = (0.3, if *name == "spent" { 2.5 } else { 1.11 });
                assert!(scene(name, *engine, lo).1 < target && scene(name, *engine, hi).1 > target);
                for _ in 0..13 {
                    let scale = (lo + hi) * 0.5;
                    if scene(name, *engine, scale).1 < target { lo = scale; } else { hi = scale; }
                }
                scales[j][i] = (lo + hi) * 0.5;
            }
        }
    }
    let mut rows = String::from("scene,variant,round,load_scale,paint_used,input_paint,remaining_paint,conservation_error,time_s,height_mean_um,height_p95_um,height_p99_um,height_max_um\n");
    let mut values = String::from("scene,variant,view,roi,x0,y0,x1,y1,y_min,y_p05,y_median,y_p95,share_below_002\n");
    let mut transport = String::from("scene,variant,point,x,y,linear_r,linear_g,linear_b,b_minus_r\n");
    for round in 0..3 {
        for offset in 0..variants.len() {
            let index = (offset + round) % variants.len();
            let (engine, variant) = variants[index];
            for (i, name) in names.iter().enumerate() {
                let scale = scales[index][i];
                let (c, used, input, left, secs) = scene(name, engine, scale);
                if round == 0 {
                    for (point, x, y) in [("drag_tail", 800.0, 250.0), ("next_mark", 800.0, 400.0)] {
                        let p = c.under(x, y, 8.0);
                        transport.push_str(&format!("{name},{variant},{point},{x},{y},{},{},{},{}\n", p[0], p[1], p[2], p[2] - p[0]));
                    }
                    for (view, pixels) in [("color", c.seen()), ("gallery", c.seen_lit(115.0, 55.0, 1.0))] {
                        let buf: Vec<u8> = pixels.iter().flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8)).collect();
                        image::save_buffer(dir.join(format!("{name}-{variant}-{view}.png")), &buf, 640, 320, image::ColorType::Rgb8)?;
                        for (roi, r) in [("core", (160, 150, 480, 170)), ("carry", (460, 249, 580, 262))] {
                            let mut ys = Vec::new();
                            for y in r.1..r.3 { for x in r.0..r.2 { ys.push(luminance(pixels[y * 640 + x])); } }
                            ys.sort_by(f32::total_cmp);
                            values.push_str(&format!("{name},{variant},{view},{roi},{},{},{},{},{},{},{},{},{}\n", r.0, r.1, r.2, r.3, ys[0], ys[ys.len() / 20], ys[ys.len() / 2], ys[ys.len() * 95 / 100], ys.iter().filter(|y| **y < 0.02).count() as f64 / ys.len() as f64));
                        }
                    }
                }
                let f = c.frame();
                let mut heights: Vec<f32> = c.surface_um().iter().enumerate().map(|(i, h)| *h + c.wet_um(f.ux(i % f.w), f.uy(i / f.w))).collect();
                heights.sort_by(f32::total_cmp);
                rows.push_str(&format!("{name},{variant},{round},{scale},{used},{input},{left},{},{secs},{},{},{},{}\n", used + left - input, heights.iter().map(|x| *x as f64).sum::<f64>() / heights.len() as f64, heights[heights.len() * 95 / 100], heights[heights.len() * 99 / 100], heights.last().unwrap()));
            }
        }
    }
    fs::write(dir.join("metrics.csv"), rows)?;
    fs::write(dir.join("luminance.csv"), values)?;
    fs::write(dir.join("transport.csv"), transport)?;
    Ok(())
}
