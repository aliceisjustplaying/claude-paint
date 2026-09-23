//! Study sheet for stroke handling: the same twilight sky gradient worked
//! five ways with the Friedrich style's broad brush, left to right:
//!
//! 1. ruler: the old look (straight, even, evenly spread, random order)
//! 2. the style's `broad()` preset as it now stands (gentle arcs, drift)
//! 3. arcs: strong wrist arcs and S-curves (`curve`)
//! 4. criss-cross: two families at ±0.5 rad (`cross`)
//! 5. drift: the direction wandering across the passage (`drift`)
//!
//! Top row: the lay-in only (two passes, as in friedrich_moonrise_valley),
//! dried. Bottom row: the same lay-in fused with
//! the badger (the blender worked top to bottom, `sweep`), dried.
//!
//!   cargo paint study_strokes                       → out/study_strokes.png
//!   cargo paint study_strokes -- --width 3200       (then crop with scripts/peek)

use paint::{Handling, Mask, Mix, Rgb, Style, gradient, hex};
use std::f32::consts::FRAC_PI_2;

/// A way of working the passage: a change to the style's broad handling.
type Way = fn(Handling) -> Handling;

fn main() {
    let o = paintings::run::Run::new("study_strokes");
    let st = Style::friedrich();
    let mut c = st.prepare(o.width, 1.0, o.seed);
    let f = c.frame();
    let (w, h) = (c.width(), c.height());
    let n = 5;
    let pw = w / n as f32;
    let ph = h * 0.5;
    let stops: [(f32, Rgb); 5] = [(0.0, hex("#34405a")), (0.3, hex("#6a7288")), (0.6, hex("#b3a79c")), (0.85, hex("#dcc39a")), (1.0, hex("#efdcae"))];
    // each panel carries the whole gradient top to bottom
    let sky = move |_x: f32, y: f32| gradient(&stops, ((y % ph) / ph).clamp(0.0, 1.0), Mix::Pigment);
    let ways: [(&str, Way); 5] = [
        ("ruler", |h| h.ruler().angle_jitter(0.03)),
        ("broad", |h| h),
        ("arcs", |h| h.curve(0.12, 0.3)),
        ("criss-cross", |h| h.cross(0.5)),
        ("drift", |h| h.drift(0.5, 150.0)),
    ];
    for row in 0..2 {
        for (i, (name, way)) in ways.iter().enumerate() {
            let (x0, y0) = (i as f32 * pw + 4.0, row as f32 * ph + 4.0);
            let (x1, y1) = (x0 + pw - 8.0, y0 + ph - 8.0);
            let panel = Mask::from_fn(f, move |x, y| if x >= x0 && x < x1 && y >= y0 && y < y1 { 1.0 } else { 0.0 });
            let s = o.seed * 1000 + (row * 10 + i) as u64 * 7;
            // the sky recipe of friedrich_moonrise_valley: a full lay-in,
            // then a leaner second pass in shorter strokes
            c.work(&panel, &way(st.broad().color(sky).medium(0.3).load(0.64).pressure(0.7, 0.9).coverage(4.0).clip(true)), s);
            c.work(&panel, &way(st.broad().color(sky).coverage(2.0).length(60.0, 150.0).medium(0.55).clip(true)), s + 5);
            if row == 1 {
                for k in 0..st.blend_passes {
                    let Some(b) = st.blend() else { break };
                    // the old way: blender strokes anywhere, in random order
                    let b = if *name == "ruler" { b.ruler() } else { b.sweep(FRAC_PI_2) };
                    c.work(&panel, &b.clip(true), s + 1 + k as u64);
                }
            }
            eprintln!("  panel {row}/{name}");
        }
    }
    c.dry();
    c.relief(st.relief.0, st.relief.1);
    o.save(&mut c);
}
