//! Study sheet for crop renders: the same small painting (sky laid and
//! fused, a ridge clipped to its edge, spruces, grass, aged varnish and
//! cracks) rendered whole, then only a window of it at the same resolution.
//! Left: that window cut from the whole render; middle: the crop render;
//! right: their difference ×8. Prints the difference and both timings.
//!
//!   cargo paint study_workflow                 (1200px canvas)
//!   cargo paint study_workflow -- --width 2400

use paint::{Canvas, Crop, Fbm, Gesture, Held, Mask, Mix, Orient, Paint, Rgb, Rng, Style, gradient, hex, smoothstep};
use paintings::run::{DEFAULT_MARGIN, Finish};
use std::time::Instant;

const WINDOW: [f32; 4] = [380.0, 300.0, 620.0, 460.0];

fn paint(width: usize, crop: Option<Crop>, seed: u64) -> Canvas {
    paint::set_crop(crop);
    let st = Style::friedrich();
    let mut c = st.prepare(width, 1.5, seed);
    paint::set_crop(None);
    let (w, h) = (c.width(), c.height());
    let f = c.frame();
    let n = Fbm::new(seed as u32 + 1, 5, 160.0);
    let ridge = f.per_column(|x: f32| h * 0.62 + 18.0 * n.get(x, 0.0) - 30.0 * (-((x - 520.0) / 150.0).powi(2)).exp());
    let sky = Mask::from_fn(f, |x, y| 1.0 - smoothstep(-0.6, 0.6, y - ridge(x)));
    let land = Mask::from_fn(f, |x, y| smoothstep(-0.6, 0.6, y - ridge(x)));
    let stops: [(f32, Rgb); 3] = [(0.0, hex("#3d4a64")), (0.45, hex("#a9a39a")), (0.62, hex("#e4cfa2"))];
    let sky_c = |_x: f32, y: f32| gradient(&stops, (y / h).clamp(0.0, 0.62), Mix::Pigment);
    c.work(&sky, &st.broad().color(sky_c).angle(|_, _| 0.0).coverage(3.0), seed * 10 + 1);
    if let Some(b) = st.blend() {
        c.work(&sky, &b.angle(|_, _| 0.0), seed * 10 + 2);
    }
    c.dry();
    c.work(&land, &st.body().color(|_, _| hex("#2f3036")).angle(|_, _| 0.0).length(20.0, 60.0).coverage(5.0).threshold(0.05).clip(true), seed * 10 + 3);
    c.dry();
    let mut rng = Rng::new(seed);
    for _ in 0..40 {
        let x = rng.range(0.0, w);
        paintings::trees::spruce(&mut c, (x, ridge(x) + 2.0), rng.range(12.0, 30.0), hex("#1d2024"), rng.next_u64());
    }
    let mut rig = Held::new(st.line_tool(0.5), 5);
    for _ in 0..120 {
        let (x, y) = (rng.range(0.0, w), 0.0);
        let y = y + ridge(x) + rng.range(20.0, h - ridge(x));
        rig.reload(Paint { color: hex("#1a1a17"), hiding: 0.85, stiff: 0.6 }, 0.6);
        let lean = rng.range(-3.0, 3.0);
        c.drag(&mut rig, &Gesture::new(vec![(x, y), (x + lean * 0.4, y - 4.0), (x + lean, y - 8.0)]).pressure(0.8, 0.1).ramps(0.02, 0.7).orient(Orient::Along), None);
    }
    c.dry();
    let fin = Finish::aged(st.relief);
    let var = Fbm::new(seed as u32 + 98, 3, 400.0);
    c.glaze(&paint::Pigment::varnish(fin.varnish), None, |x, y| fin.varnish_coats + fin.varnish_vary * var.get(x, y));
    let ground_um = st.ground.iter().map(|g| g.um).sum();
    c.crack(&paint::Cracks { ground_um, seed, ..paint::Cracks::aged(0) });
    c.relief(fin.relief.0, fin.relief.1);
    c
}

fn main() {
    let o = paintings::run::Run::new("study_workflow");
    let width = if std::env::args().any(|a| a == "--width") { o.width } else { 1200 };
    let t = Instant::now();
    let whole = paint(width, None, o.seed);
    let t_whole = t.elapsed().as_secs_f32();
    let t = Instant::now();
    let crop = paint(width, Some(Crop { units: WINDOW, margin: DEFAULT_MARGIN }), o.seed);
    let t_crop = t.elapsed().as_secs_f32();
    // the crop's kept pixels (without margin) in whole-canvas pixels
    let s = whole.frame().scale;
    let (x0, y0) = ((WINDOW[0] * s).round() as usize, (WINDOW[1] * s).round() as usize);
    let (cw, ch) = ((WINDOW[2] * s).round() as usize - x0, (WINDOW[3] * s).round() as usize - y0);
    let win = crop.window();
    let at_whole = |x: usize, y: usize| whole.pixels()[(y0 + y) * whole.window().w + x0 + x];
    let at_crop = |x: usize, y: usize| crop.pixels()[(y0 + y - win.y0) * win.w + x0 + x - win.x0];
    let (mut sum, mut mx) = (0.0f64, 0.0f32);
    for y in 0..ch {
        for x in 0..cw {
            let (a, b) = (at_whole(x, y), at_crop(x, y));
            for k in 0..3 {
                let d = (paint::color::linear_to_srgb(a[k]) - paint::color::linear_to_srgb(b[k])).abs() * 255.0;
                sum += d as f64;
                mx = mx.max(d);
            }
        }
    }
    eprintln!(
        "whole {t_whole:.1}s, crop {t_crop:.1}s ({:.0}% of the canvas + margin {DEFAULT_MARGIN} units); crop vs whole: mean {:.2}/255, max {mx:.0}/255",
        100.0 * (cw * ch) as f32 / (whole.window().w * whole.window().h) as f32,
        sum / (3 * cw * ch) as f64
    );
    // sheet: whole | crop | difference ×8
    let gap = 6;
    let sw = 3 * cw + 2 * gap;
    let mut sheet = Canvas::new_window(sw, sw as f32 / ch as f32, [0.02; 3], None);
    let ss = sheet.frame().scale;
    sheet.apply(|x, y, bg| {
        let (xp, yp) = ((x * ss) as usize, ((y * ss) as usize).min(ch - 1));
        let (col, cx) = (xp / (cw + gap), xp % (cw + gap));
        if col > 2 || cx >= cw {
            return bg;
        }
        match col {
            0 => at_whole(cx, yp),
            1 => at_crop(cx, yp),
            _ => {
                let (a, b) = (at_whole(cx, yp), at_crop(cx, yp));
                let d = (0..3).map(|k| (paint::color::linear_to_srgb(a[k]) - paint::color::linear_to_srgb(b[k])).abs()).fold(0.0, f32::max);
                let v = paint::color::srgb_to_linear((d * 8.0).min(1.0));
                [v; 3]
            }
        }
    });
    o.save(&mut sheet);
}
