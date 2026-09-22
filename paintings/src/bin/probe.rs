//! Debug probe: one dark rigger stroke on a flat canvas; report pixels that
//! came out lighter than the background (should be none).
use paint::{Canvas, Gesture, Held, Orient, Paint, Tool, hex};

fn main() {
    let relief = std::env::args().any(|a| a == "relief");
    let weave = std::env::args().any(|a| a == "weave");
    let mut c = Canvas::new(3200, 4.0, hex("#9aa3a6"));
    if weave {
        c = c.with_weave(0.9, 0.35, 1);
    }
    let bg = c.px[0];
    for (i, w) in [0.2f32, 0.3, 0.5, 0.8].iter().enumerate() {
        let y = 50.0 + i as f32 * 50.0;
        let tool = Tool { width: w * 1.6, length: w * 4.0, ..Tool::rigger(*w) };
        let mut held = Held::new(tool, 7 + i as u64);
        held.load(Paint { color: hex("#2a2420"), hiding: 0.95, body: 0.9 }, 1.0);
        let g = Gesture::new(vec![(100.0, y), (300.0, y + 10.0), (500.0, y - 5.0)]).pressure(1.0, 0.3).ramps(0.0, 0.4).orient(Orient::Across);
        c.drag(&mut held, &g, None);
    }
    c.dry();
    if relief {
        c.relief(0.5, 0.0);
    }
    let lum = |p: [f32; 3]| 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2];
    let lb = lum(bg);
    let mut n = 0;
    let mut worst = 0.0f32;
    let mut at = (0, 0);
    for (i, p) in c.px.iter().enumerate() {
        let d = lum(*p) - lb;
        if d > 0.005 {
            n += 1;
            if d > worst {
                worst = d;
                at = (i % c.f.w, i / c.f.w);
            }
        }
    }
    println!("bg lum {lb:.4}; lighter pixels: {n}; worst +{worst:.4} at {at:?}");
    c.save("out/probe.png").unwrap();
}
