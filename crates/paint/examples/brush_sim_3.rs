use paint::color::{linear_to_srgb, luminance};
use paint::palette::Palette;
use paint::{Canvas, Gesture, Held, Linen, Paint, Tool, hex};
use std::{fs, path::Path, time::Instant};

const VARIANTS: [(u32, &str); 5] = [
    (5, "baseline"),
    (10, "body2"),
    (12, "lean3"),
    (13, "body3"),
    (14, "wild3"),
];
const SCENES: [&str; 7] = [
    "crossing",
    "spent",
    "dark",
    "wet",
    "thin-dark",
    "impasto-light",
    "lit-window",
];

#[derive(Default)]
struct Ledger {
    input: (f64, f64),
    residual: (f64, f64),
    requested_load: f64,
    preparation_input: (f64, f64),
}

impl Ledger {
    fn load(&mut self, h: &mut Held, paint: Paint, amount: f32) {
        assert!(amount.is_finite() && amount >= 0.0);
        let before = h.carried();
        let mut left = amount;
        while left > 0.0 {
            let dip = left.min(1.0);
            h.load(paint, dip);
            left -= dip;
        }
        let after = h.carried();
        self.input.0 += after.0 - before.0;
        self.input.1 += after.1 - before.1;
        self.requested_load += amount as f64;
    }

    fn retain(&mut self, h: &Held) {
        let (p, s) = h.carried();
        self.residual.0 += p;
        self.residual.1 += s;
    }
}

struct Run {
    canvas: Canvas,
    ledger: Ledger,
    initial_paint: f64,
    initial_solvent: f64,
    net_paint: f64,
    net_solvent: f64,
    secs: f64,
}

fn pull(
    c: &mut Canvas,
    engine: u32,
    seed: u64,
    paint: Paint,
    load: f32,
    width: f32,
    gestures: &[Gesture],
    ledger: &mut Ledger,
) {
    let mut h = Held::new(Tool::hog_flat(width), seed).with_engine(engine);
    ledger.load(&mut h, paint, load);
    for g in gestures {
        c.drag(&mut h, g, None);
    }
    ledger.retain(&h);
}

fn tube(name: &str, medium: f32) -> Paint {
    let pal = Palette::tube_box();
    let i = pal.tubes.iter().position(|t| t.name == name).unwrap();
    pal.pile(vec![(i, 1.0)]).paint(medium)
}

fn scene(name: &str, engine: u32, factor: f32) -> Run {
    let ground = match name {
        "dark" | "thin-dark" => "#eee8da",
        "impasto-light" | "lit-window" => "#343941",
        _ => "#bcb3a2",
    };
    let mut c = Canvas::new(640, 2.0, hex(ground))
        .with_size_mm(160.0)
        .with_engine(5)
        .with_linen(Linen::fine(1906));
    c.prime(hex(ground), 0.92, 100.0, 0.9, 0.3, 1906);
    let (initial_paint, initial_solvent) = (c.material_total(), c.solvent_total());
    let mut ledger = Ledger::default();
    let blue = Paint::body(hex("#456e8a")).with_stiff(0.92);
    let orange = Paint::body(hex("#be5e32")).with_stiff(0.92);
    let black = tube("bone black", 0.05);
    let white = tube("lead white", 0.0).with_stiff(0.98);
    let start = Instant::now();
    if name == "lit-window" {
        let wall = Paint::body(hex("#202936"))
            .with_stiff(0.88)
            .with_thinner(0.12);
        for (i, y) in [100.0, 180.0, 260.0, 340.0, 420.0].into_iter().enumerate() {
            pull(
                &mut c,
                5,
                200 + i as u64,
                wall,
                0.9,
                190.0,
                &[Gesture::line((60.0, y), (960.0, y + 8.0)).pressure(0.85, 0.85)],
                &mut ledger,
            );
        }
        ledger.preparation_input = ledger.input;
    }
    c = c.with_engine(engine);
    match name {
        "crossing" => {
            pull(
                &mut c,
                engine,
                31,
                blue,
                0.9 * factor,
                110.0,
                &[Gesture::line((90.0, 240.0), (930.0, 240.0)).pressure(0.75, 0.75)],
                &mut ledger,
            );
            pull(
                &mut c,
                engine,
                73,
                orange,
                0.75 * factor,
                85.0,
                &[
                    Gesture::line((420.0, 70.0), (620.0, 440.0)).pressure(0.4, 0.85),
                    Gesture::line((700.0, 390.0), (940.0, 410.0)).pressure(0.6, 0.5),
                ],
                &mut ledger,
            );
        }
        "spent" => pull(
            &mut c,
            engine,
            73,
            orange,
            0.07 * factor,
            85.0,
            &[Gesture::line((110.0, 250.0), (930.0, 250.0)).pressure(0.7, 0.7)],
            &mut ledger,
        ),
        "dark" => pull(
            &mut c,
            engine,
            73,
            black,
            0.9 * factor,
            110.0,
            &[Gesture::line((110.0, 250.0), (930.0, 250.0)).pressure(0.85, 0.85)],
            &mut ledger,
        ),
        "wet" => {
            for y in [175.0, 275.0] {
                pull(
                    &mut c,
                    engine,
                    31,
                    blue,
                    0.9 * factor,
                    130.0,
                    &[Gesture::line((360.0, y), (610.0, y)).pressure(0.85, 0.85)],
                    &mut ledger,
                );
            }
            pull(
                &mut c,
                engine,
                73,
                orange,
                0.18 * factor,
                85.0,
                &[
                    Gesture::line((120.0, 250.0), (930.0, 250.0)).pressure(0.65, 0.85),
                    Gesture::line((700.0, 400.0), (940.0, 400.0)).pressure(0.65, 0.65),
                ],
                &mut ledger,
            );
        }
        "thin-dark" => pull(
            &mut c,
            engine,
            73,
            black.with_thinner(0.45),
            0.12 * factor,
            110.0,
            &[Gesture::line((110.0, 250.0), (930.0, 250.0)).pressure(0.75, 0.75)],
            &mut ledger,
        ),
        "impasto-light" => {
            for (i, y) in [205.0, 245.0, 285.0].into_iter().enumerate() {
                pull(
                    &mut c,
                    engine,
                    90 + i as u64,
                    white,
                    0.95 * factor,
                    130.0,
                    &[Gesture::line((330.0, y), (620.0, y + 12.0)).pressure(0.85, 0.7)],
                    &mut ledger,
                );
            }
        }
        "lit-window" => {
            let light = Paint::body(hex("#f1d69c")).with_stiff(0.94);
            for (i, x) in [455.0, 495.0, 535.0].into_iter().enumerate() {
                pull(
                    &mut c,
                    engine,
                    310 + i as u64,
                    light,
                    0.85 * factor,
                    90.0,
                    &[Gesture::line((x, 135.0), (x + 7.0, 325.0)).pressure(0.8, 0.75)],
                    &mut ledger,
                );
            }
            let mut blend = Held::new(Tool::badger(75.0), 411).with_engine(engine);
            c.drag(
                &mut blend,
                &Gesture::line((425.0, 120.0), (425.0, 345.0)).pressure(0.45, 0.3),
                None,
            );
            c.drag(
                &mut blend,
                &Gesture::line((570.0, 335.0), (570.0, 125.0)).pressure(0.4, 0.25),
                None,
            );
            ledger.retain(&blend);
        }
        _ => unreachable!(),
    }
    let net_paint = c.material_total() - initial_paint;
    let net_solvent = c.solvent_total() - initial_solvent;
    Run {
        canvas: c,
        ledger,
        initial_paint,
        initial_solvent,
        net_paint,
        net_solvent,
        secs: start.elapsed().as_secs_f64(),
    }
}

fn calibrate(name: &str, engine: u32, target: f64, steps: usize) -> Result<(f32, usize), String> {
    let (mut lo, mut hi) = (0.0, 1.0);
    let mut probes = 1;
    if scene(name, engine, lo).net_paint > target {
        return Err(format!(
            "{name}/{engine}: common preparation exceeds target"
        ));
    }
    loop {
        probes += 1;
        if scene(name, engine, hi).net_paint >= target {
            break;
        }
        hi *= 2.0;
        if hi > 8.0 {
            return Err(format!(
                "{name}/{engine}: cannot bracket target within factor 8"
            ));
        }
    }
    for _ in 0..steps {
        let mid = (lo + hi) * 0.5;
        probes += 1;
        if scene(name, engine, mid).net_paint < target {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Ok(((lo + hi) * 0.5, probes))
}

fn stats(mut xs: Vec<f32>) -> [f64; 5] {
    assert!(!xs.is_empty() && xs.iter().all(|v| v.is_finite()));
    xs.sort_by(f32::total_cmp);
    [
        xs.iter().map(|v| *v as f64).sum::<f64>() / xs.len() as f64,
        xs[xs.len() / 20] as f64,
        xs[xs.len() / 2] as f64,
        xs[xs.len() * 95 / 100] as f64,
        *xs.last().unwrap() as f64,
    ]
}

fn rois(name: &str) -> Vec<(&'static str, (usize, usize, usize, usize))> {
    match name {
        "lit-window" => vec![
            ("light_core", (298, 105, 334, 190)),
            ("soft_edge", (258, 75, 286, 220)),
            ("wet_wall", (100, 100, 220, 220)),
        ],
        "impasto-light" => vec![
            ("light_core", (230, 128, 370, 188)),
            ("light_edge", (210, 115, 395, 200)),
        ],
        _ => vec![
            ("core", (160, 150, 480, 170)),
            ("edge", (160, 115, 480, 150)),
            ("carry", (460, 249, 580, 262)),
        ],
    }
}

fn export(
    run: &Run,
    name: &str,
    variant: &str,
    dir: &Path,
    values: &mut String,
    transport: &mut String,
    profiles: &mut String,
) -> Result<(), Box<dyn std::error::Error>> {
    let c = &run.canvas;
    let f = c.frame();
    let wet: Vec<f32> = (0..f.w * f.h)
        .map(|i| c.wet_um(f.ux(i % f.w), f.uy(i / f.w)))
        .collect();
    let solvent: Vec<f32> = (0..f.w * f.h)
        .map(|i| c.solvent_um(f.ux(i % f.w), f.uy(i / f.w)))
        .collect();
    let height: Vec<f32> = c
        .surface_um()
        .iter()
        .zip(&wet)
        .zip(&solvent)
        .map(|((h, w), s)| h + w + s)
        .collect();
    let color = c.seen();
    let gallery = c.seen_lit(115.0, 55.0, 1.0);
    if name == "lit-window" {
        let (y0, y1) = (105, 190);
        for x in 220..390 {
            let mut sums = [0.0f64; 5];
            for y in y0..y1 {
                let i = y * f.w + x;
                sums[0] += luminance(color[i]) as f64;
                sums[1] += luminance(gallery[i]) as f64;
                sums[2] += wet[i] as f64;
                sums[3] += solvent[i] as f64;
                sums[4] += height[i] as f64;
            }
            let means = sums.map(|sum| sum / (y1 - y0) as f64);
            profiles.push_str(&format!(
                "{name},{variant},{x},{},{y0},{y1},{},{},{},{},{}\n",
                f.ux(x),
                means[0],
                means[1],
                means[2],
                means[3],
                means[4]
            ));
        }
    }
    for (view, pixels) in [("color", &color), ("gallery", &gallery)] {
        let buf: Vec<u8> = pixels
            .iter()
            .flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8))
            .collect();
        image::save_buffer(
            dir.join(format!("{name}-{variant}-{view}.png")),
            &buf,
            640,
            320,
            image::ColorType::Rgb8,
        )?;
        for (roi, (x0, y0, x1, y1)) in rois(name) {
            let mut ys = Vec::new();
            let mut ws = Vec::new();
            let mut hs = Vec::new();
            let mut ss = Vec::new();
            let mut gradients = Vec::new();
            let mut light_delta = Vec::new();
            for y in y0..y1 {
                for x in x0..x1 {
                    let i = y * f.w + x;
                    let lum = luminance(pixels[i]);
                    ys.push(lum);
                    ws.push(wet[i]);
                    hs.push(height[i]);
                    ss.push(solvent[i]);
                    light_delta.push(luminance(gallery[i]) - luminance(color[i]));
                    if x + 1 < x1 && y + 1 < y1 {
                        gradients.push(
                            ((luminance(pixels[i + 1]) - lum).powi(2)
                                + (luminance(pixels[i + f.w]) - lum).powi(2))
                            .sqrt(),
                        );
                    }
                }
            }
            let dark_share = ys.iter().filter(|y| **y < 0.02).count() as f64 / ys.len() as f64;
            let y = stats(ys);
            let w = stats(ws);
            let h = stats(hs);
            let s = stats(ss);
            let g = stats(gradients);
            let d = stats(light_delta);
            values.push_str(&format!("{name},{variant},{view},{roi},{x0},{y0},{x1},{y1},{},{},{},{},{dark_share},{},{},{},{},{},{},{},{},{},{}\n",
                y[0], y[1], y[2], y[3], w[0], w[3], h[0], h[3], h[4], s[0], g[0], g[3], d[0], d[3]));
        }
    }
    for (point, x, y) in [("drag_tail", 800.0, 250.0), ("next_mark", 800.0, 400.0)] {
        let p = c.under(x, y, 8.0);
        transport.push_str(&format!(
            "{name},{variant},{point},{x},{y},{},{},{},{}\n",
            p[0],
            p[1],
            p[2],
            p[2] - p[0]
        ));
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let output = args.first().ok_or("brush_sim_3 <new-output-dir> [--matched] [--scene NAME] [--variant NAME] [--rounds N] [--calibration-steps N]")?;
    let option = |key: &str| {
        args.iter()
            .position(|a| a == key)
            .and_then(|i| args.get(i + 1))
    };
    let matched = args.iter().any(|a| a == "--matched");
    let rounds: usize = option("--rounds")
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(1);
    let steps: usize = option("--calibration-steps")
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(10);
    assert!((1..=5).contains(&rounds) && (1..=16).contains(&steps));
    let names: Vec<_> = SCENES
        .into_iter()
        .filter(|name| option("--scene").is_none_or(|s| s == name))
        .collect();
    let variants: Vec<_> = VARIANTS
        .into_iter()
        .filter(|(_, name)| option("--variant").is_none_or(|s| s == name))
        .collect();
    if names.is_empty() || variants.is_empty() {
        return Err("unknown scene or variant".into());
    }
    let dir = Path::new(output);
    if dir.exists() && fs::read_dir(dir)?.next().is_some() {
        return Err("output directory must be empty; existing artifacts are never replaced".into());
    }
    fs::create_dir_all(dir)?;
    fs::write(
        dir.join("scope.txt"),
        "Native 640x320, physical 160x80 mm. Gallery 115/55/1 and Color read the SAME final wet Canvas. No image filters. Matching scope: total final canvas material_total minus its primed initial material_total, including the common baseline-prepared wet wall. This conserved paint quantity has units coats*square canvas units (a volume proxy, not density-weighted physical mass). No per-pixel or per-stroke matching. Calibration scales only variant loads; factors above one use multiple finite dips <=1 before each mark. Each retained brush contributes its paint AND solvent residual, including the clean blender after pickup. No wait/drying is called. Height includes dry surface, wet paint and solvent; wet thickness is paint alone. Linear Y and native one-pixel forward gradient statistics include background pixels inside explicitly recorded ROIs. The lit-window edge profile is analysis only: for each native pixel column x in [220,390), arithmetic means over rows y in [105,190) record Color/Gallery linear Y, wet paint, solvent and total height; exported PNGs are untouched. Matching tolerance and monotonicity are empirical; achieved error is reported, never hidden.\n",
    )?;
    let mut metrics = String::from(
        "scene,variant,round,mode,load_factor,calibration_probes,target_net_paint,match_error_pct,requested_load,initial_paint,final_paint,net_paint,input_paint,residual_paint,paint_ledger_error,initial_solvent,final_solvent,net_solvent,input_solvent,residual_solvent,solvent_ledger_error,preparation_input_paint,preparation_input_solvent,time_s,wet_mean_um,wet_p95_um,wet_max_um,height_mean_um,height_p95_um,height_max_um\n",
    );
    let mut values = String::from(
        "scene,variant,view,roi,x0,y0,x1,y1,y_mean,y_p05,y_median,y_p95,share_below_002,wet_mean_um,wet_p95_um,height_mean_um,height_p95_um,height_max_um,solvent_mean_um,gradient_mean_y_per_px,gradient_p95_y_per_px,gallery_minus_color_y_mean,gallery_minus_color_y_p95\n",
    );
    let mut transport =
        String::from("scene,variant,point,x,y,linear_r,linear_g,linear_b,b_minus_r\n");
    let mut profiles = String::from(
        "scene,variant,x_px,x_units,strip_y0_px,strip_y1_px,color_y_mean,gallery_y_mean,wet_paint_mean_um,solvent_mean_um,total_height_mean_um\n",
    );
    for name in names {
        let target = scene(name, 5, 1.0).net_paint;
        for (engine, variant) in &variants {
            let (factor, probes) = if matched && *engine != 5 {
                calibrate(name, *engine, target, steps)?
            } else {
                (1.0, 0)
            };
            for round in 0..rounds {
                let run = scene(name, *engine, factor);
                if round == 0 {
                    export(
                        &run,
                        name,
                        variant,
                        dir,
                        &mut values,
                        &mut transport,
                        &mut profiles,
                    )?;
                }
                let c = &run.canvas;
                let f = c.frame();
                let wet: Vec<f32> = (0..f.w * f.h)
                    .map(|i| c.wet_um(f.ux(i % f.w), f.uy(i / f.w)))
                    .collect();
                let heights = c
                    .surface_um()
                    .iter()
                    .enumerate()
                    .map(|(i, h)| h + wet[i] + c.solvent_um(f.ux(i % f.w), f.uy(i / f.w)))
                    .collect();
                let w = stats(wet);
                let h = stats(heights);
                let l = &run.ledger;
                metrics.push_str(&format!("{name},{variant},{round},{},{factor},{probes},{target},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                    if matched { "matched-total-canvas-net-paint" } else { "equal-requested-input" }, 100.0 * (run.net_paint / target - 1.0),
                    l.requested_load, run.initial_paint, c.material_total(), run.net_paint, l.input.0, l.residual.0, run.net_paint + l.residual.0 - l.input.0,
                    run.initial_solvent, c.solvent_total(), run.net_solvent, l.input.1, l.residual.1, run.net_solvent + l.residual.1 - l.input.1,
                    l.preparation_input.0, l.preparation_input.1, run.secs, w[0], w[3], w[4], h[0], h[3], h[4]));
                eprintln!(
                    "{name}/{variant} round {round}: factor {factor:.5}, net {}, match {:+.3}%, {:.2}s",
                    run.net_paint,
                    100.0 * (run.net_paint / target - 1.0),
                    run.secs
                );
            }
        }
        fs::write(dir.join("metrics.csv"), &metrics)?;
        fs::write(dir.join("luminance-thickness.csv"), &values)?;
        fs::write(dir.join("transport.csv"), &transport)?;
        fs::write(dir.join("edge-profile.csv"), &profiles)?;
    }
    Ok(())
}
