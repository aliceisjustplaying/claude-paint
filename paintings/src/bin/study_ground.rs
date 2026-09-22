//! Calibration sheet for the physical surface: a Friedrich canvas as bought
//! (linen + ground), then thin fluid paint (should pool in the ground's
//! hollows), a lean sky scumble and stiff body paint (should keep its marks).
//! Top band: bare ground. Then thin smalt-blue paint, a stiff white stroke
//! band, and a dark glaze.

use paint::{Gesture, Held, Mask, Orient, Paint, Pigment, Style, Tool, hex};

fn main() {
    let t0 = std::time::Instant::now();
    let o = paint::cli::opts("study_ground");
    let st = Style::friedrich();
    let mut c = st.prepare(o.width, 2.0, o.seed);
    eprintln!("  ground     {:>6.2}s", t0.elapsed().as_secs_f32());
    let h = c.height();
    // band 2: thin fluid blue, laid with a soft filbert in long strokes
    let fr = c.f;
    let band = |a: f32, b: f32| Mask::from_fn(fr, move |_, y| if y >= a * h && y < b * h { 1.0 } else { 0.0 });
    let b2 = band(0.25, 0.5);
    let mut fil = Held::new(Tool::filbert(30.0), 3);
    for i in 0..12 {
        fil.reload(Paint { color: hex("#5d6f94"), hiding: 0.35, body: 0.15 }, 0.8);
        let y = h * 0.26 + i as f32 * h * 0.02;
        c.drag(&mut fil, &Gesture::new(vec![(0.0, y), (1000.0, y)]).pressure(0.8, 0.8).orient(Orient::Across), Some(&b2));
    }
    c.dry();
    // band 3: stiff lead white, hog brush, keeps its marks
    let b3 = band(0.5, 0.75);
    let mut hog = Held::new(Tool::hog_flat(24.0), 4);
    for i in 0..10 {
        hog.reload(Paint { color: hex("#e6e1d3"), hiding: 0.9, body: 1.0 }, 1.0);
        let y = h * 0.51 + i as f32 * h * 0.024;
        c.drag(&mut hog, &Gesture::new(vec![(0.0, y), (500.0, y + 2.0), (1000.0, y)]).pressure(0.85, 0.85).orient(Orient::Across), Some(&b3));
    }
    c.dry();
    // band 4: a dark umber glaze
    let b4 = band(0.75, 1.0);
    c.glaze(&Pigment::transparent(hex("#3b2a1c")), Some(&b4), |_, _| 2.0);
    c.relief(0.35, 0.02);
    c.save(&o.out).unwrap();
    paint::cli::done(&o, t0);
}
