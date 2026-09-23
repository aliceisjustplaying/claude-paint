//! Covering a passage (notes/surface.md). Dark body paint over the warm
//! Friedrich ground, four panels a row:
//!
//! Top, the body filbert: coverage 1 (strokes side by side, ground between
//! them, as asked); 2.5 without looking (`fill(false)`: the gaps a hand
//! leaves between its strokes, the old flecks); 2.5 as painted now (the
//! painter looks and dabs the gaps); 2.5 with a nearly dry brush (load
//! 0.12: dry brush, broken on purpose, never filled).
//!
//! Bottom, the same with the broad filbert, and last a light-pressure
//! scumble (pressure 0.15–0.3, load 0.1) that skims the weave's peaks.

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
