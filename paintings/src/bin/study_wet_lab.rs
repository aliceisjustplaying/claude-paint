//! The wet-paint failures the round 6 lab painters reported (notes/wet.md
//! §5), each rebuilt as one panel with the handling they used, and measured:
//!
//! 0. **blend on a wet seam**: a thin sky and a dark ground, both open, then
//!    the style's clean blender across the seam and a badger along it.
//! 1. **glaze hand over open thin sky** (`hand="glaze"`, `tool="filbert 6"`,
//!    medium 0.85).
//! 2. **the same over tacky sky** (the "crackle mosaic").
//! 3. **the same over dry sky** (the "net of dark rims").
//! 4. reference: the brush-free `glaze()` verb over dry sky, same color.
//! 5. **a thin stroke into open water**, a round 3 with a varying pressure,
//!    then a short level blend (the "tramlines").
//! 6. **a clean blender over a lean veil on open thin sky** (lab 3: "salmon
//!    dashes after `blend` at coverage 1.2 over a lean filbert veil").
//!
//! Printed: the share of each panel's worked area that shows the ground,
//! its texture (mean |L − L blurred over 2 units|, OKLab ×1000: rims,
//! mosaic and streaks raise it) and, for panel 5, the tramline index (how
//! much lighter the stroke's two edges are than its middle, as a share of
//! the stroke's lift; 0 = one soft line).
//!
//! `cargo paint study_wet_lab` (1000 px). `--width 3200` for the detail.

use paint::color::to_oklab;
use paint::{Canvas, Gesture, Held, Orient, Pigment, Stage, Style, Tool, hex};
use paintings::study::{Img, rect};

const W: f32 = 1000.0 / 7.0;
const Y0: f32 = 40.0;
const Y1: f32 = 360.0;

fn panel(k: usize) -> (f32, f32) {
    (k as f32 * W + 12.0, (k + 1) as f32 * W - 12.0)
}

const SKY: &str = "#b9c3cb";

/// A thin sky (the style's broad handling, medium 0.3) over panel `k`.
fn sky(c: &mut Canvas, st: &Style, k: usize, y1: f32) {
    let (x0, x1) = panel(k);
    let col = hex(SKY);
    let m = rect(c, x0, Y0, x1, y1);
    c.work(&m, &st.broad().color(move |_, _| col).medium(0.3).clip(true), 10 + k as u64);
}

/// Glaze-hand strokes (as `work(m, {hand="glaze", tool="filbert 6", medium=0.85})`).
fn glaze_hand(c: &mut Canvas, st: &Style, k: usize) {
    let (x0, x1) = panel(k);
    let m = rect(c, x0 + 15.0, 120.0, x1 - 15.0, 300.0);
    let col = hex("#8d8a86");
    let mut h = st.glaze(0.85).color(move |_, _| col);
    h.tool = Tool::filbert(6.0);
    c.work(&m, &h.length(20.0, 60.0), 20 + k as u64);
}

fn main() {
    let o = paintings::run::Run::new("study_wet_lab");
    let st = Style::friedrich();
    let mut c = st.prepare(o.width, 1000.0 / 400.0, o.seed);
    let g = to_oklab(c.pixels()[0]);
    // skies to dry (glaze on dry, the glaze verb)
    sky(&mut c, &st, 3, Y1);
    sky(&mut c, &st, 4, Y1);
    c.dry();
    // the sky that will be tacky
    sky(&mut c, &st, 2, Y1);
    let (px, py) = (0.5 * (panel(2).0 + panel(2).1), 200.0);
    let t0 = c.clock();
    while c.drying_at(px, py) != Stage::Tacky && c.clock() - t0 < 3.0 * 24.0 * 60.0 {
        c.wait(30.0);
    }
    println!("panel 2's sky is {:?} after {:.0} min", c.drying_at(px, py), c.clock() - t0);
    glaze_hand(&mut c, &st, 2);
    glaze_hand(&mut c, &st, 3);
    let (x0, x1) = panel(4);
    let m = rect(&c, x0 + 15.0, 120.0, x1 - 15.0, 300.0);
    c.glaze(&Pigment::semi(hex("#8d8a86")), Some(&m), |_, _| 0.35);
    // open: the seam, the glaze hand over open sky, the water
    sky(&mut c, &st, 0, 200.0);
    let (x0, x1) = panel(0);
    let ground = hex("#5b5a3e");
    c.work(&rect(&c, x0, 200.0, x1, Y1), &st.body().color(move |_, _| ground).medium(0.3).clip(true), 30);
    if std::env::var("LAB_NOBLEND").is_err() {
        let b = st.blend().expect("friedrich has a blender").coverage(1.2);
        c.work(&rect(&c, x0, 170.0, x1, 230.0), &b, 31);
        if std::env::var("LAB_NOBADGER").is_err() {
            let mut bad = Held::new(Tool::badger(20.0), 32);
            c.drag(&mut bad, &Gesture::new(vec![(x0 + 5.0, 200.0), (x1 - 5.0, 201.0)]).pressure(0.35, 0.3), None);
        }
    }
    sky(&mut c, &st, 1, Y1);
    glaze_hand(&mut c, &st, 1);
    let (x0, x1) = panel(5);
    let water = hex("#5d6670");
    c.work(&rect(&c, x0, Y0, x1, Y1), &st.body().color(move |_, _| water).medium(0.4).angle(|_, _| 0.0).clip(true), 40);
    let mut r = Held::new(Tool::round_sable(3.0), 41);
    r.load(paint::Paint::body(hex("#c9ccd0")).with_stiff(0.6), 0.7);
    c.drag(&mut r, &Gesture::new(vec![(x0 + 10.0, 200.0), (x1 - 10.0, 200.0)]).pressure(0.45, 0.45).swell(vec![0.25, 1.1, 0.7, 1.2, 0.1]).orient(Orient::Across), None);
    let lb = st.blend().expect("blender").coverage(1.0).angle(|_, _| 0.0).length(20.0, 40.0);
    c.work(&rect(&c, x0 + 10.0, 192.0, x1 - 10.0, 208.0), &lb, 42);
    sky(&mut c, &st, 6, Y1);
    let (x0, x1) = panel(6);
    let veil = hex("#9a968e");
    let mut vh = st.body().color(move |_, _| veil).medium(0.5).load(0.3).length(20.0, 50.0);
    vh.tool = Tool::filbert(6.0);
    c.work(&rect(&c, x0 + 15.0, 120.0, x1 - 15.0, 300.0), &vh, 50);
    c.work(&rect(&c, x0 + 15.0, 120.0, x1 - 15.0, 300.0), &st.blend().expect("blender").coverage(1.2), 51);
    c.dry();
    // measure
    let img = Img::new(&c, c.pixels());
    let area = |k: usize| {
        let (x0, x1) = panel(k);
        match k {
            0 => (x0 + 5.0, 172.0, x1 - 5.0, 228.0),
            5 => (x0 + 15.0, 190.0, x1 - 15.0, 210.0),
            _ => (x0 + 20.0, 125.0, x1 - 20.0, 295.0),
        }
    };
    let names = ["blend on a wet seam", "glaze hand, open sky", "glaze hand, tacky sky", "glaze hand, dry sky", "glaze() verb, dry sky", "thin stroke into open water", "blend over a lean veil, open"];
    for k in 0..7 {
        println!("{k} {:<28} ground {:>5.1}%  texture {:>5.1}", names[k], 100.0 * img.ground(g, area(k)), img.texture(area(k)));
    }
    // tramlines: the stroke's cross profile, averaged along it
    let (x0, x1) = panel(5);
    let prof: Vec<f32> = (0..=24).map(|j| {
        let y = 194.0 + 0.5 * j as f32;
        let xs: Vec<f32> = (0..60).map(|i| x0 + 25.0 + (x1 - x0 - 50.0) * i as f32 / 59.0).collect();
        xs.iter().map(|&x| img.px(x, y)).sum::<f32>() / xs.len() as f32
    }).collect();
    let base = 0.5 * (prof[0] + prof[prof.len() - 1]);
    let peak = prof.iter().cloned().fold(f32::MIN, f32::max);
    let mid = prof.len() / 2;
    let (l, r) = (prof[..mid].iter().cloned().fold(f32::MIN, f32::max), prof[mid..].iter().cloned().fold(f32::MIN, f32::max));
    let center = prof[mid - 2..=mid + 2].iter().cloned().fold(f32::MIN, f32::max);
    println!("5 tramline index {:.2} (profile L: {})", ((l.min(r) - center) / (peak - base)).max(0.0), prof.iter().map(|v| format!("{v:.3}")).collect::<Vec<_>>().join(" "));
    o.save(&mut c);
}
