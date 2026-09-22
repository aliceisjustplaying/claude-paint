//! Calibration sheet for craquelure (`paint::crack`). Each panel is a small
//! Friedrich-primed canvas with painted bands (a smalt-blue glaze, stiff
//! lead-white strokes, a dark umber glaze), aged varnish, then cracked and
//! lit by raking light.
//!
//! Top row, 50 mm panels (macro): the same 3 mm network over a thin brittle
//! ground (follows the weave: jagged, rectangular islands), a medium ground
//! and a thick double ground (free, curved).
//! Bottom row, 160 mm panels: island sizes 2, 4 and 6 mm, with corner cracks.

use paint::{Canvas, Cracks, Gesture, Held, Mask, Orient, Paint, Pigment, Style, Tool, hex};

fn panel(width_px: usize, width_mm: f32, aspect: f32, k: &Cracks, seed: u64) -> Canvas {
    let st = Style { width_mm, ..Style::friedrich() };
    let mut c = st.prepare(width_px, aspect, seed);
    let h = c.height();
    let fr = c.frame();
    let band = |a: f32, b: f32| Mask::from_fn(fr, move |_, y| if y >= a * h && y < b * h { 1.0 } else { 0.0 });
    // smalt sky glaze over the upper half, deeper at the top
    c.glaze(&Pigment::semi(hex("#6f82a6")), Some(&band(0.0, 0.5)), move |_, y| 2.2 - 1.6 * y / (0.5 * h));
    // stiff lead white, hog brush
    let b3 = band(0.5, 0.72);
    let mut hog = Held::new(Tool::hog_flat(60.0), seed + 3);
    for i in 0..6 {
        hog.reload(Paint { color: hex("#e3dccb"), hiding: 0.85, stiff: 0.9 }, 0.8);
        let y = h * (0.52 + 0.035 * i as f32);
        c.drag(&mut hog, &Gesture::new(vec![(0.0, y), (500.0, y + 4.0), (1000.0, y)]).pressure(0.8, 0.8).orient(Orient::Across), Some(&b3));
    }
    c.dry();
    // dark umber glaze below
    c.glaze(&Pigment::transparent(hex("#3b2a1c")), Some(&band(0.72, 1.0)), |_, _| 2.5);
    // aged varnish, then the cracks, then raking light
    c.glaze(&Pigment::varnish(hex("#e6d3a4")), None, |_, _| 0.4);
    let t = std::time::Instant::now();
    c.crack(k);
    eprintln!("    cracked {}x{} px ({width_mm} mm) in {:.2}s", fr.w, fr.h, t.elapsed().as_secs_f32());
    c.relief(1.0, 0.03);
    c
}

fn main() {
    let o = paintings::run::Run::new("study_cracks");
    let pw = o.width / 3;
    let aspect = 4.0 / 3.0;
    let ph = (pw as f32 / aspect).round() as usize;
    let s = o.seed;
    let base = Cracks { corners: false, island_mm: 3.0, ..Cracks::aged(s) };
    let panels = [
        panel(pw, 50.0, aspect, &Cracks { ground_um: 0.0, ..base }, s),
        panel(pw, 50.0, aspect, &Cracks { ground_um: 50.0, ..base }, s),
        panel(pw, 50.0, aspect, &Cracks { ground_um: 200.0, ..base }, s),
        panel(pw, 160.0, aspect, &Cracks { island_mm: 2.0, ground_um: 150.0, corners: true, ..base }, s),
        panel(pw, 160.0, aspect, &Cracks { island_mm: 4.0, ground_um: 150.0, corners: true, ..base }, s),
        panel(pw, 160.0, aspect, &Cracks { island_mm: 6.0, ground_um: 150.0, corners: true, ..base }, s),
    ];
    let gap = (pw / 60).max(2);
    let (w, h) = (3 * pw + 2 * gap, 2 * ph + gap);
    let mut sheet = Canvas::new(w, w as f32 / h as f32, [0.02, 0.02, 0.02]);
    let scale = sheet.frame().scale;
    sheet.apply(|x, y, bg| {
        let (xp, yp) = ((x * scale) as usize, (y * scale) as usize);
        let (col, cx) = (xp / (pw + gap), xp % (pw + gap));
        let (row, cy) = (yp / (ph + gap), yp % (ph + gap));
        if col > 2 || row > 1 || cx >= pw || cy >= ph {
            return bg;
        }
        panels[row * 3 + col].pixels()[cy * pw + cx]
    });
    o.save(&mut sheet);
}
