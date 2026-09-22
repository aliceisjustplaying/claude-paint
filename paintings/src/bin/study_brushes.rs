//! Brush physics study sheet: each row tests one behavior.

use paint::{Gesture, Held, Orient, Paint, Tool, hex};

fn main() {
    let o = paintings::run::Run::new("study_brushes");
    // lead-white priming on linen
    let mut c = paint::Canvas::new(o.width, 1.5, hex("#ece6d8")).with_linen(paint::Linen::fine(3));

    let ultramarine = hex("#1f2a78");
    let sienna = hex("#8a3b12");
    let sap = hex("#4f6b1c");
    let ivory_black = hex("#1a1816");
    let cad_yellow = hex("#f2c200");
    let cobalt = hex("#1c4fa0");
    let crimson = hex("#9a1030");
    let white = hex("#f4f1e8");
    let ochre = hex("#c08a2a");

    let wave = |x0: f32, y: f32, len: f32, amp: f32| -> Vec<(f32, f32)> {
        (0..6).map(|i| {
            let t = i as f32 / 5.0;
            (x0 + t * len, y + (t * std::f32::consts::TAU).sin() * amp)
        }).collect()
    };

    // ---- row 1: each tool, fully loaded, one S-stroke
    let tools = [
        (Tool::round_sable(8.0), ultramarine),
        (Tool::hog_flat(20.0), sienna),
        (Tool::filbert(16.0), sap),
        (Tool::fan(26.0), ivory_black),
        (Tool::rigger(1.6), ivory_black),
    ];
    for (i, (t, col)) in tools.iter().enumerate() {
        let mut b = Held::new(t.clone(), 10 + i as u64);
        b.load(Paint::body(*col), 1.0);
        let x = 30.0 + i as f32 * 190.0;
        c.drag(&mut b, &Gesture::new(wave(x, 60.0, 160.0, 14.0)), None);
    }

    // ---- row 2: pressure falls off → dry brush over the weave
    let mut b = Held::new(Tool::hog_flat(22.0), 20);
    b.load(Paint::body(sienna), 0.8);
    c.drag(&mut b, &Gesture::line((30.0, 150.0), (470.0, 150.0)).pressure(1.0, 0.12).ramps(0.03, 0.02), None);
    let mut b = Held::new(Tool::round_sable(10.0), 21);
    b.load(Paint::body(ultramarine), 0.5);
    c.drag(&mut b, &Gesture::new(wave(520.0, 150.0, 450.0, 8.0)).pressure(0.9, 0.3), None);

    // ---- row 3: wet into wet — yellow dragged through wet blue picks it up
    let mut b = Held::new(Tool::hog_flat(24.0), 30);
    for k in 0..4 {
        b.reload(Paint::body(cobalt), 1.0);
        let y = 215.0 + k as f32 * 16.0;
        c.drag(&mut b, &Gesture::line((30.0, y), (330.0, y)).pressure(0.9, 0.8), None);
    }
    let mut b = Held::new(Tool::filbert(14.0), 31);
    for k in 0..5 {
        b.reload(Paint::body(cad_yellow), 1.0);
        let x = 60.0 + k as f32 * 55.0;
        c.drag(&mut b, &Gesture::line((x, 200.0), (x + 20.0, 290.0)).pressure(0.95, 0.9), None);
    }
    // white into wet dark
    let mut b = Held::new(Tool::hog_flat(24.0), 32);
    for k in 0..4 {
        b.reload(Paint::body(ivory_black), 1.0);
        let y = 215.0 + k as f32 * 16.0;
        c.drag(&mut b, &Gesture::line((370.0, y), (620.0, y)), None);
    }
    let mut b = Held::new(Tool::round_sable(9.0), 33);
    b.load(Paint::body(white), 1.0);
    c.drag(&mut b, &Gesture::new(vec![(390.0, 280.0), (450.0, 215.0), (520.0, 280.0), (600.0, 210.0)]).pressure(0.9, 0.7), None);

    // blender: two wet colors side by side, badger zigzag across the join
    let mut b = Held::new(Tool::hog_flat(24.0), 34);
    for k in 0..5 {
        b.reload(Paint::body(ochre), 1.0);
        let y = 205.0 + k as f32 * 16.0;
        c.drag(&mut b, &Gesture::line((660.0, y), (810.0, y)), None);
        b.reload(Paint::body(crimson), 1.0);
        c.drag(&mut b, &Gesture::line((810.0, y), (970.0, y)), None);
    }
    let mut bl = Held::new(Tool::badger(34.0), 35);
    let zig: Vec<(f32, f32)> = (0..10).map(|i| (790.0 + if i % 2 == 0 { -18.0 } else { 18.0 }, 200.0 + i as f32 * 10.0)).collect();
    c.drag(&mut bl, &Gesture::new(zig).pressure(0.5, 0.5), None);
    let zig2: Vec<(f32, f32)> = (0..10).map(|i| (840.0 + if i % 2 == 0 { -14.0 } else { 14.0 }, 200.0 + i as f32 * 10.0)).collect();
    c.drag(&mut bl, &Gesture::new(zig2).pressure(0.35, 0.35), None);

    // ---- row 4: let it dry, then glaze crimson over part of rows 3
    c.dry();
    let mut g = Held::new(Tool::filbert(30.0), 40);
    for k in 0..3 {
        g.reload(Paint::glaze(crimson), 1.0 * 0.3);
        let y = 215.0 + k as f32 * 22.0;
        c.drag(&mut g, &Gesture::line((180.0, y), (520.0, y)).pressure(0.6, 0.6), None);
    }

    // ---- row 5: Cézanne-style hatching with a flat at a fixed angle, and fan dabs
    let mut b = Held::new(Tool::hog_flat(12.0), 50);
    let greens = [hex("#5a7a3a"), hex("#7d9a4a"), hex("#3f5f3a"), hex("#9aa860"), hex("#6f8fa0")];
    let mut r = paint::Rng::new(5);
    for k in 0..60 {
        b.reload(Paint::body(greens[k % greens.len()]), 0.7);
        let x = 40.0 + r.range(0.0, 380.0);
        let y = 340.0 + r.range(0.0, 90.0);
        let a = -0.9f32;
        c.drag(&mut b, &Gesture::line((x, y), (x + a.cos() * 18.0, y + a.sin() * 18.0)).orient(Orient::Fixed(a + std::f32::consts::FRAC_PI_2)).pressure(0.85, 0.7), None);
    }
    let mut fan = Held::new(Tool::fan(30.0), 51);
    for k in 0..14 {
        fan.reload(Paint::scumble(if k % 2 == 0 { hex("#3d5a2a") } else { hex("#8aa050") }), 0.5 * 0.6);
        let x = 480.0 + k as f32 * 34.0;
        let y = 400.0 + r.range(-20.0, 20.0);
        c.drag(&mut fan, &Gesture::line((x, y + 10.0), (x + 6.0, y - 30.0)).orient(Orient::Across).pressure(0.6, 0.3), None);
    }

    // ---- row 6: one dirty brush dragged across wet colors without cleaning
    let bands = [cobalt, cad_yellow, crimson, white, sap];
    let mut b = Held::new(Tool::hog_flat(26.0), 60);
    for (k, col) in bands.iter().enumerate() {
        b.reload(Paint::body(*col), 1.0);
        let x = 40.0 + k as f32 * 70.0;
        c.drag(&mut b, &Gesture::line((x, 480.0), (x, 610.0)).orient(Orient::Across), None);
    }
    let mut b = Held::new(Tool::filbert(18.0), 61);
    b.load(Paint::body(white), 0.6);
    for k in 0..3 {
        let y = 505.0 + k as f32 * 40.0;
        c.drag(&mut b, &Gesture::line((25.0, y), (400.0, y + 8.0)).pressure(0.9, 0.9), None);
    }
    // rigger twigs
    let mut rg = Held::new(Tool::rigger(1.4), 62);
    for k in 0..8 {
        rg.reload(Paint::body(ivory_black), 1.0);
        let x = 500.0 + k as f32 * 55.0;
        let pts: Vec<(f32, f32)> = (0..6).map(|i| (x + (i as f32 * 1.7).sin() * 8.0 + i as f32 * 6.0, 620.0 - i as f32 * 22.0)).collect();
        c.drag(&mut rg, &Gesture::new(pts).pressure(0.9, 0.2).ramps(0.02, 0.6), None);
    }

    c.relief(0.7, 0.04);
    o.save(&mut c);
}
