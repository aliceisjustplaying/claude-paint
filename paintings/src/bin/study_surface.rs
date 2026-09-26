//! Covering a region. Dark body paint over `Style::friedrich()`'s prepared
//! ground, four panels a row:
//!
//! Top, the body handling: coverage 1 (strokes side by side, ground between
//! them); coverage 2.5 with `fill(false)` (the gaps between strokes stay
//! open); coverage 2.5 with the default fill (bare cells between strokes
//! get extra dabs); coverage 2.5 at load 0.12 (below the fill threshold of
//! load 0.25, so the gaps are never filled).
//!
//! Bottom, the same with the broad handling, and last a light-pressure
//! pass (pressure 0.15–0.3, load 0.1) that touches only the weave's peaks.

use paint::{Handling, Mask, Style, hex};

fn main() {
    let o = paintings::run::Run::new("study_surface");
    let st = Style::friedrich();
    let mut c = st.prepare(o.width, 2.0, o.seed);
    let f = c.frame();
    let dark = |_: f32, _: f32| hex("#2c2925");
    let panel = |k: usize, row: usize| {
        let (x0, y0) = (20.0 + k as f32 * 245.0, 20.0 + row as f32 * 240.0);
        Mask::from_fn(f, move |x, y| if x >= x0 && x < x0 + 225.0 && y >= y0 && y < y0 + 220.0 { 1.0 } else { 0.0 })
    };
    for row in 0..2 {
        let hd = |cov: f32| (if row == 0 { st.body() } else { st.broad() }).color(dark).clip(true).threshold(0.5).coverage(cov);
        let passes: [Handling; 4] = [
            hd(1.0),
            hd(2.5).fill(false),
            hd(2.5),
            if row == 0 { Handling { load: 0.12, ..hd(2.5) } } else { Handling { load: 0.1, ..hd(2.5).pressure(0.15, 0.3) } },
        ];
        for (k, p) in passes.iter().enumerate() {
            c.work(&panel(k, row), p, 10 + (row * 4 + k) as u64);
        }
    }
    c.dry();
    c.relief(0.3, 0.02);
    o.save(&mut c);
}
