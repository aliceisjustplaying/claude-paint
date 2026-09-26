//! Craquelure old vs new. One Friedrich-primed canvas
//! (`Style::friedrich`: 440 mm, 240 µm ground) with a pale smalt sky glaze,
//! a band of stiff lead white and a dark umber glaze, varnished, then:
//!
//! - left: the old `Cracks::aged` numbers (3.5 mm islands on an assumed
//!   60 µm, partly weave-bound ground, 70 µm cracks, dirt 0.6, even aging),
//! - right: the new `Cracks::aged`, fitted to this canvas (240 µm ground →
//!   ~3.6 mm islands, curved; ~33 µm openings; uneven aging; milky varnish).
//!
//! Top row: the whole canvas at the run's width (1000px by default).
//! Bottom row: the same 3200px window of each (sky, lead white, dark glaze).
//!
//!   cargo paint study_aging

use paint::{Canvas, Crop, Cracks, Gesture, Held, Mask, Orient, Paint, Pigment, Style, Tool, hex, set_crop};

const ASPECT: f32 = 4.0 / 3.0;
/// The 3200px window, units (x0, y0, x1, y1).
const WIN: [f32; 4] = [300.0, 250.0, 612.5, 437.5];

fn painted(width_px: usize, seed: u64) -> Canvas {
    let st = Style::friedrich();
    let mut c = st.prepare(width_px, ASPECT, seed);
    let h = c.height();
    let fr = c.frame();
    let band = |a: f32, b: f32| Mask::from_fn(fr, move |_, y| if y >= a * h && y < b * h { 1.0 } else { 0.0 });
    // pale smalt sky, deeper at the top
    c.glaze(&Pigment::semi(hex("#8a9bb8")), Some(&band(0.0, 0.4)), move |_, y| 2.4 - 1.8 * y / (0.4 * h));
    // stiff lead white, hog brush: a snow bank
    let b3 = band(0.4, 0.5);
    let mut hog = Held::new(Tool::hog_flat(40.0), seed + 3);
    for i in 0..5 {
        hog.reload(Paint::new(hex("#e8e2d4"), 0.9, 0.9), 0.9);
        let y = h * (0.405 + 0.02 * i as f32);
        c.drag(&mut hog, &Gesture::new(vec![(0.0, y), (500.0, y + 3.0), (1000.0, y)]).pressure(0.85, 0.85).orient(Orient::Across), Some(&b3));
    }
    c.dry();
    // a warm mid-tone then a dark umber glaze below
    c.glaze(&Pigment::semi(hex("#7a6448")), Some(&band(0.5, 1.0)), |_, _| 1.5);
    c.glaze(&Pigment::transparent(hex("#3b2a1c")), Some(&band(0.5, 1.0)), move |x, y| 0.4 + 1.2 * ((y - 0.5 * h) / (0.5 * h)) * (0.6 + 0.4 * (x / 1000.0)));
    c.glaze(&Pigment::varnish(hex("#e6d3a4")), None, |_, _| 0.4);
    c
}

fn finish(mut c: Canvas, k: &Cracks) -> Canvas {
    let t = std::time::Instant::now();
    c.crack(k);
    eprintln!("    cracked {}x{} px in {:.2}s", c.window().w, c.window().h, t.elapsed().as_secs_f32());
    c.relief(Style::friedrich().relief.0, Style::friedrich().relief.1);
    c
}

fn main() {
    let o = paintings::run::Run::new("study_aging");
    let s = o.seed;
    let old = Cracks { island_mm: Some(3.5), ground_um: Some(60.0), width_um: Some(70.0), depth_um: 35.0, cupping_um: 30.0, dirt: 0.6, corners: true, vary: 0.0, veil: 0.0, hierarchy: 0.0, patchy: 0.0, grain: 0.0, grime: 0.0, seed: s };
    let new = Cracks::aged(s);
    let w = o.width;
    eprintln!("  {w}px canvases");
    let whole = [finish(painted(w, s), &old), finish(painted(w, s), &new)];
    // the 3200px window (a crop render: only it and a margin are painted)
    eprintln!("  3200px window");
    set_crop(Some(Crop { units: WIN, margin: 20.0 }));
    let crops = [finish(painted(3200, s), &old), finish(painted(3200, s), &new)];
    set_crop(None);
    let cw = ((WIN[2] - WIN[0]) * 3.2).round() as usize;
    let ch = ((WIN[3] - WIN[1]) * 3.2).round() as usize;
    let (x0, y0) = ((WIN[0] * 3.2).round() as usize, (WIN[1] * 3.2).round() as usize);
    let wh = (w as f32 / ASPECT).round() as usize;
    let gap = 8;
    let sw = 2 * w.max(cw) + gap;
    let sh = wh + gap + ch;
    let mut sheet = Canvas::new(sw, sw as f32 / sh as f32, [0.02, 0.02, 0.02]);
    let scale = sheet.frame().scale;
    sheet.apply(|x, y, bg| {
        let (xp, yp) = ((x * scale) as usize, (y * scale) as usize);
        let col = xp / (w.max(cw) + gap);
        let cx = xp % (w.max(cw) + gap);
        if col > 1 {
            return bg;
        }
        if yp < wh {
            let c = &whole[col];
            let f = c.window();
            if cx < f.w && yp < f.h { c.pixels()[yp * f.w + cx] } else { bg }
        } else if yp >= wh + gap && yp < wh + gap + ch && cx < cw {
            let c = &crops[col];
            let f = c.window();
            let (gx, gy) = (x0 + cx, y0 + yp - wh - gap);
            if gx >= f.x0 && gy >= f.y0 && gx - f.x0 < f.w && gy - f.y0 < f.h { c.pixels()[(gy - f.y0) * f.w + gx - f.x0] } else { bg }
        } else {
            bg
        }
    });
    o.save(&mut sheet);
}
