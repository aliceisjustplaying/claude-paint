//! A measurement for notes/look (not a check): raw against burnt sienna by
//! the standard hiding measure, the contrast ratio (reflectance of a film
//! over black ÷ over white, ASTM D2805 / ISO 6504-3; lower = more
//! see-through), beside the card's difference measure (thinner check 13
//! (b), crates/paint/tests/thinner_pigments.rs, whose card this copies).
//!
//!   LOOK_OUT=<dir> cargo test --release -p paint --test look_sienna -- --ignored --nocapture

use paint::color::luminance;
use paint::{Canvas, Gesture, Held, Linen, Paint, Palette, Pigment, Rgb, Tool, hex};

fn sargent(name: &str) -> Paint {
    let pal = Palette::named_box("sargent").expect("the sargent box");
    let i = pal.tubes.iter().position(|t| t.name == name).unwrap();
    pal.pile(vec![(i, 1.0)]).laid(0.0)
}

fn mean_rgb(c: &Canvas, r: (f32, f32, f32, f32)) -> Rgb {
    let f = c.frame();
    let pts: Vec<(f32, f32)> = (0..f.w * f.h).map(|i| (f.ux(i % f.w), f.uy(i / f.w))).filter(|&(x, y)| x >= r.0 && x < r.2 && y >= r.1 && y < r.3).collect();
    let mut a = [0.0f64; 3];
    for &(x, y) in &pts {
        let p = c.under(x, y, 0.0);
        for k in 0..3 {
            a[k] += p[k] as f64;
        }
    }
    a.map(|v| (v / pts.len() as f64) as f32)
}

const BLACK: (f32, f32, f32, f32) = (200.0, 330.0, 800.0, 470.0);
const WHITE: (f32, f32, f32, f32) = (200.0, 530.0, 800.0, 670.0);

/// thinner_pigments.rs's card: a bone-black and a lead-white band, dried.
fn card() -> Canvas {
    let mut c = Canvas::new(400, 1.0, hex("#d8cdb8")).with_engine(3).with_linen(Linen::fine(3));
    c.prime(hex("#b9a98c"), 0.9, 60.0, 0.6, 0.2, 7);
    for (name, y0) in [("bone black", 300.0f32), ("lead white", 500.0)] {
        for k in 0..10 {
            let mut h = Held::new(Tool::hog_flat(40.0), 1 + k);
            h.load(sargent(name), 1.0);
            let y = y0 + 10.0 + 20.0 * k as f32;
            c.drag(&mut h, &Gesture::new(vec![(100.0, y), (900.0, y)]).pressure(0.9, 0.9), None);
        }
    }
    c.dry();
    c
}

/// The card's twenty filbert strokes (load 0.5) of `paint`; returns the
/// band colors after and the mean paint (µm) over the two bands.
fn painted(mut c: Canvas, paint: Paint) -> (Rgb, Rgb, f32) {
    for k in 0..20 {
        let mut h = Held::new(Tool::filbert(30.0), 100 + k);
        h.load(paint, 0.5);
        let y = 310.0 + 20.0 * k as f32;
        c.drag(&mut h, &Gesture::new(vec![(150.0, y), (850.0, y)]).pressure(0.8, 0.8), None);
    }
    let f = c.frame();
    let um: Vec<f32> = (0..f.w * f.h).map(|i| (f.ux(i % f.w), f.uy(i / f.w))).filter(|&(x, y)| x >= 200.0 && x < 800.0 && ((y >= 330.0 && y < 470.0) || (y >= 530.0 && y < 670.0))).map(|(x, y)| c.wet_um(x, y)).collect();
    (mean_rgb(&c, BLACK), mean_rgb(&c, WHITE), um.iter().sum::<f32>() / um.len() as f32)
}

fn ratio(b: Rgb, w: Rgb) -> String {
    format!("Y {:.3}  R {:.3} G {:.3} B {:.3}", luminance(b) / luminance(w), b[0] / w[0], b[1] / w[1], b[2] / w[2])
}

#[test]
#[ignore = "a measurement for notes/look"]
fn siennas_by_contrast_ratio() {
    let c0 = card();
    let (kb, kw) = (mean_rgb(&c0, BLACK), mean_rgb(&c0, WHITE));
    let diff0 = luminance(kw) - luminance(kb);
    println!("card bands: black Y {:.4}, white Y {:.4}", luminance(kb), luminance(kw));
    for name in ["raw sienna", "burnt sienna"] {
        let p = sargent(name);
        println!("{name}: masstone Y {:.4}, scattering {:.4} per coat, hiding of one coat {:.4}", luminance(p.color), p.scatter, p.hiding());
    }
    // (25 µm is one coat: the tubes' `hiding` is the contrast ratio of one coat on luminance alone)
    println!("\n(1) a uniform film, the engine's own optics (Kubelka-Munk), thinned or not alike (the solvent has no color):");
    for um in [3.0f32, 10.0, 25.0, 30.0] {
        let x = um / paint::COAT_UM;
        for name in ["raw sienna", "burnt sienna"] {
            let p = sargent(name);
            let pig = Pigment::masstone(p.color, p.scatter);
            // the standard chart: black (0) and a white of 80% reflectance
            let (b, w) = (pig.over([0.0; 3], x), pig.over([0.8; 3], x));
            // the card's own black and white bands
            let (cb, cw) = (pig.over(kb, x), pig.over(kw, x));
            let kept = (luminance(cw) - luminance(cb)) / diff0;
            println!("  {um:>4} µm {name:<12}: contrast ratio on a black/80% white chart {}; on the card's bands {}; difference kept on the card {:.1}%", ratio(b, w), ratio(cb, cw), 100.0 * kept);
        }
    }
    println!("\n(2) the card painted as check 13 (b) paints it, twenty strokes at load 0.5:");
    for t in [0.0f32, 0.5] {
        for name in ["raw sienna", "burnt sienna"] {
            let p = sargent(name);
            let (b, w, um) = painted(c0.clone(), if t > 0.0 { p.with_thinner(t) } else { p });
            println!("  thinner {t}: {name:<12} mean paint {um:.2} µm; contrast ratio {}; difference kept {:.1}%", ratio(b, w), 100.0 * (luminance(w) - luminance(b)) / diff0);
        }
    }
}

/// The two siennas, the same thin film (thinner 0.5, load 0.5, the same
/// strokes), over light, mid-grey and dark bands: one 256 px panel each,
/// written to $LOOK_OUT as sienna_raw.png and sienna_burnt.png.
#[test]
#[ignore = "a picture for notes/look"]
fn siennas_side_by_side() {
    let out = std::env::var("LOOK_OUT").expect("LOOK_OUT");
    let pal = Palette::named_box("sargent").unwrap();
    let idx = |n: &str| pal.tubes.iter().position(|t| t.name == n).unwrap();
    let grey = pal.pile(vec![(idx("lead white"), 0.6), (idx("bone black"), 0.4)]).laid(0.0);
    for (file, name) in [("sienna_raw", "raw sienna"), ("sienna_burnt", "burnt sienna")] {
        let mut c = Canvas::new(256, 1.0, hex("#d8cdb8")).with_engine(3).with_size_mm(300.0);
        c.prime(hex("#b9a98c"), 0.9, 60.0, 0.6, 0.0, 7);
        for (band, y0) in [(sargent("lead white"), 100.0f32), (grey, 380.0), (sargent("bone black"), 660.0)] {
            for k in 0..12 {
                let mut h = Held::new(Tool::hog_flat(40.0), 1 + k);
                h.load(band, 1.0);
                let y = y0 + 10.0 + 20.0 * k as f32;
                c.drag(&mut h, &Gesture::new(vec![(40.0, y), (960.0, y)]).pressure(0.9, 0.9), None);
            }
        }
        c.dry();
        for k in 0..14 {
            let mut h = Held::new(Tool::filbert(30.0), 100 + k);
            h.load(sargent(name).with_thinner(0.5), 0.5);
            let x = 230.0 + 40.0 * k as f32;
            c.drag(&mut h, &Gesture::new(vec![(x, 60.0), (x + 10.0, 960.0)]).pressure(0.8, 0.8), None);
        }
        c.wait(40.0);
        let f = c.frame();
        let px: Vec<u8> = c.seen().iter().flat_map(|p| p.map(|v| (paint::color::linear_to_srgb(v).clamp(0.0, 1.0) * 255.0).round() as u8)).collect();
        image::RgbImage::from_raw(f.w as u32, f.h as u32, px).unwrap().save(format!("{out}/{file}.png")).unwrap();
    }
}
