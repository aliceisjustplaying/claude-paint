//! Measurements for notes/look (not checks): the rim of a single thinned
//! stroke, and the thinned broad pass with linen and on a plain ground.
//! Both are `#[ignore]`d and only print numbers (and write PNGs to
//! $LOOK_OUT when it is set).
//!
//!   LOOK_OUT=<dir> cargo test --release -p paint --test look_rim -- --ignored --nocapture
//!
//! `LOOK_TOOL` (diagnostic only) changes the filbert of the single stroke:
//! `push0` (no plough), `pickup0` (no pickup) or `both0` (neither).

use paint::color::linear_to_srgb;
use paint::{Canvas, Gesture, Held, Linen, Mask, Palette, Style, Tool, hex};

fn save(c: &Canvas, path: &str) {
    let f = c.frame();
    let px: Vec<u8> = c.seen().iter().flat_map(|p| p.map(|v| (linear_to_srgb(v).clamp(0.0, 1.0) * 255.0).round() as u8)).collect();
    image::RgbImage::from_raw(f.w as u32, f.h as u32, px).unwrap().save(path).unwrap();
}

/// Paint µm across the stroke at x 480-520 (mean), one value a pixel row,
/// y 270-400; then the edge rows (the largest of the first three and of
/// the last three rows holding paint) and the inside (the mean of the
/// rows between them).
fn profile(c: &Canvas) -> (Vec<f32>, f32, f32, f32) {
    let f = c.frame();
    let prof: Vec<f32> = (0..f.h).map(|y| f.uy(y)).filter(|&y| (270.0..400.0).contains(&y)).map(|y| (480..=520).map(|x| c.wet_um(x as f32, y)).sum::<f32>() / 41.0).collect();
    let rows: Vec<usize> = (0..prof.len()).filter(|&k| prof[k] > 0.05).collect();
    if rows.len() < 8 {
        return (prof, 0.0, 0.0, 0.0);
    }
    let (a, b) = (rows[0], *rows.last().unwrap());
    let e1 = prof[a..a + 3].iter().copied().fold(0.0, f32::max);
    let e2 = prof[b - 2..=b].iter().copied().fold(0.0, f32::max);
    let inside = &prof[a + 3..b - 2];
    (prof.clone(), e1, e2, inside.iter().sum::<f32>() / inside.len() as f32)
}

/// One filbert stroke on a plain ground (the scene of
/// notes/thinner/single_stroke_edge.jpg: raw umber 2/3 + bone black 1/3,
/// load 0.6, pressure 0.8, 256 px, engine 3).
#[test]
#[ignore = "a measurement for notes/look"]
fn single_stroke_rim() {
    let pal = Palette::named_box("inness").unwrap();
    let idx = |n: &str| pal.tubes.iter().position(|t| t.name == n).unwrap();
    let mix = pal.pile(vec![(idx("raw umber"), 2.0 / 3.0), (idx("bone black"), 1.0 / 3.0)]);
    let out = std::env::var("LOOK_OUT").ok();
    let diag = std::env::var("LOOK_TOOL").unwrap_or_default();
    let tag = std::env::var("LOOK_TAG").unwrap_or_else(|_| "today".into());
    for (name, t, wait) in [("thinned_0min", 0.5f32, 0.0f32), ("thinned_30min", 0.5, 30.0), ("unthinned", 0.0, 0.0)] {
        let mut c = Canvas::new(256, 1.5, hex("#e8dcc0")).with_engine(3).with_size_mm(300.0);
        c.prime(hex("#e3d6b8"), 0.85, 40.0, 0.5, 0.0, 2);
        let mut tool = Tool::filbert(60.0);
        match diag.as_str() {
            "push0" => tool.push = 0.0,
            "pickup0" => tool.pickup = 0.0,
            "both0" => {
                tool.push = 0.0;
                tool.pickup = 0.0;
            }
            _ => {}
        }
        let mut h = Held::new(tool, 5);
        let p = mix.laid(0.0);
        h.load(if t > 0.0 { p.with_thinner(t) } else { p }, 0.6);
        c.drag(&mut h, &Gesture::new(vec![(150.0, 330.0), (850.0, 340.0)]).pressure(0.8, 0.8), None);
        if wait > 0.0 {
            c.wait(wait);
        }
        let (prof, e1, e2, inside) = profile(&c);
        let s: Vec<String> = prof.iter().map(|v| format!("{v:.1}")).collect();
        println!("{tag} {diag} {name}: edges {e1:.2} / {e2:.2} µm, inside {inside:.2} µm; across: {}", s.join(" "));
        if let Some(o) = &out {
            save(&c, &format!("{o}/stroke_{tag}_{name}.png"));
        }
    }
}

/// The thinned broad pass of the rag study (raw umber 2 + bone black 1,
/// thinner 0.5, the style's broad hand, coverage 2.5, horizontal) with
/// 15-thread linen and on a plain ground, 256 px, engine 3.
#[test]
#[ignore = "a measurement for notes/look"]
fn broad_pass() {
    let pal = Palette::named_box("inness").unwrap();
    let idx = |n: &str| pal.tubes.iter().position(|t| t.name == n).unwrap();
    let sty = Style::oil_with(pal.clone());
    let out = std::env::var("LOOK_OUT").ok();
    let tag = std::env::var("LOOK_TAG").unwrap_or_else(|_| "today".into());
    for (name, linen) in [("linen", true), ("plain", false)] {
        let mut c = Canvas::new(256, 1.5, hex("#e8dcc0")).with_engine(3).with_size_mm(300.0);
        if linen {
            c = c.with_linen(Linen { warp_per_cm: 15.0, weft_per_cm: 15.0, ..Linen::fine(2) });
        }
        c.prime(hex("#e3d6b8"), 0.85, 40.0, 0.5, 0.0, 2);
        let f = c.frame();
        let m = Mask::from_fn(f, |x, y| if (100.0..900.0).contains(&x) && (120.0..430.0).contains(&y) { 1.0 } else { 0.0 });
        let mix = pal.pile(vec![(idx("raw umber"), 2.0), (idx("bone black"), 1.0)]);
        let hd = sty.broad().piled(&pal, mix, sty.body_medium).thinner(0.5).coverage(2.5).angle(|_, _| 0.0);
        c.work(&m, &hd, 1);
        // mean paint and its spread over the middle of the pass
        let pts: Vec<f32> = (0..f.w * f.h).map(|i| (f.ux(i % f.w), f.uy(i / f.w))).filter(|&(x, y)| (200.0..800.0).contains(&x) && (180.0..370.0).contains(&y)).map(|(x, y)| c.wet_um(x, y)).collect();
        let mean = pts.iter().sum::<f32>() / pts.len() as f32;
        let sd = (pts.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / pts.len() as f32).sqrt();
        println!("{tag} broad pass, {name}: paint {mean:.2} µm, spread (sd) {sd:.2} µm, sd/mean {:.2}", sd / mean.max(1e-6));
        if let Some(o) = &out {
            save(&c, &format!("{o}/broad_{tag}_{name}.png"));
        }
    }
}
