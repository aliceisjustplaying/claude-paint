//! Color semantics: judging paint by its look on the canvas.
//!
//! Top band, a sky gradient painted with a broad brush, then small dabs
//! mixed to match it, each half of the band in two thin paints (left
//! quarter semi 0.55 medium, then thin 0.8):
//!   x 0–500    before: the pile read the old way (its color = how one coat
//!              looks over white), so every semi-transparent dab dries
//!              darker than the sky it sits on ("digital rain")
//!   x 500–750  after: `Canvas::aim`, dabs matched to what is under them;
//!              they vanish into the sky
//!   x 750–1000 after, aimed a little lighter than the sky: a stipple that
//!              reads as intended (Friedrich's lightening of a sky)
//! Middle band, a glaze wedge: upper half light body color, lower half dark,
//! then a dark transparent glaze (umber and black in 0.9 medium, by
//! masstone) growing from nothing to 5 coats left → right: it deepens the
//! light, barely touches the dark.
//! Bottom band, three smalt → lead-white skies over the warm ground:
//!   left   `by_masstone()`: piles mixed to the target masstone, not aimed
//!   middle aimed at the look (the default for `mixed`), full palette
//!   right  aimed, with a family set out for the passage (`Palette::only`)

use paint::color::{from_oklab, to_oklab};
use paint::{Gesture, Held, Mask, Mix, Paint, Rng, Style, Tool, gradient, hex, smoothstep};
use std::time::Instant;

fn main() {
    let o = paintings::run::Run::new("study_color");
    let st = Style::friedrich_early();
    let pal = &st.palette;
    let mut c = st.prepare(o.width, 1.0, o.seed);
    let f = c.frame();
    let mut rng = Rng::new(o.seed * 7 + 1);

    // ---- top band: sky, then tone-matched dabs
    let band = |y0: f32, y1: f32| Mask::from_fn(f, move |_, y| if y >= y0 && y < y1 { 1.0 } else { 0.0 });
    let sky = band(0.0, 300.0);
    let stops = [(0.0, hex("#5f7398")), (0.6, hex("#a9b3bd")), (1.0, hex("#ddd6c2"))];
    let sky_c = move |_: f32, y: f32| gradient(&stops, y / 300.0, Mix::Pigment);
    let t = Instant::now();
    c.work(&sky, &st.broad().color(sky_c).coverage(5.0).load(0.6), 1);
    c.dry();
    eprintln!("sky (aimed): {:.2}s", t.elapsed().as_secs_f32());
    let mut dab = |c: &mut paint::Canvas, p: Paint, x: f32, y: f32, seed: u64| {
        let mut b = Held::new(Tool { ragged: 0.2, ..Tool::round_sable(5.0) }, seed);
        b.load(p, 0.5);
        let a = rng.range(0.0, 6.28);
        c.drag(&mut b, &Gesture::new(vec![(x, y), (x + a.cos() * 1.5, y + a.sin() * 1.5)]).pressure(0.7, 0.5).ramps(0.1, 0.4), None);
    };
    let mut pts = Vec::new();
    let mut r2 = Rng::new(o.seed * 13 + 3);
    for _ in 0..900 {
        pts.push((r2.range(8.0, 992.0), r2.range(12.0, 288.0)));
    }
    let t = Instant::now();
    for (k, &(x, y)) in pts.iter().enumerate() {
        let quarter = (x / 250.0) as usize;
        let medium = if (x % 250.0) < 125.0 { 0.55 } else { 0.8 };
        let under = c.under(x, y, 2.5);
        let p = match quarter {
            0 | 1 => {
                // the old reading of a pile's color: its look, one coat over white
                let m = pal.mix(under).paint(medium);
                Paint::tint(m.color, m.hiding, m.stiff)
            }
            2 => c.aim(pal, under, (x, y), 2.5, medium, 0.6),
            _ => {
                let mut l = to_oklab(under);
                l[0] += 0.06;
                c.aim(pal, from_oklab(l), (x, y), 2.5, medium, 0.6)
            }
        };
        dab(&mut c, p, x, y, 100 + k as u64);
    }
    c.dry();
    eprintln!("900 dabs: {:.2}s", t.elapsed().as_secs_f32());

    // ---- middle band: glaze wedge over light and dark
    let light = band(320.0, 440.0);
    let dark = band(440.0, 560.0);
    c.work(&light, &st.body().color(|_, _| hex("#d9d1bd")).clip(true).threshold(0.5), 2);
    c.work(&dark, &st.body().color(|_, _| hex("#2c2925")).clip(true).threshold(0.5), 3);
    c.dry();
    // a dark transparent glaze (raw umber and black in 0.9 medium, mixed by
    // masstone), laid as an even film growing from 0 to 5 coats left → right
    let umber = pal.only(&["raw umber", "bone black"]);
    let glaze = umber.paint(hex("#2b1f14"), 0.9);
    eprintln!("glaze: {} hiding {:.3}", umber.recipe(&umber.mix(hex("#2b1f14"))), glaze.hiding);
    let wedge = Mask::from_fn(f, |x, y| if (320.0..560.0).contains(&y) && x > 40.0 { 1.0 } else { 0.0 });
    c.glaze(&glaze.pigment(), Some(&wedge), |x, _| 5.0 * smoothstep(40.0, 1000.0, x));
    for x in [100.0, 300.0, 500.0, 700.0, 950.0] {
        let (l, d) = (to_oklab(c.sample(x, 380.0))[0], to_oklab(c.sample(x, 500.0))[0]);
        eprintln!("  wedge x{x:.0}: over light L {l:.3}, over dark L {d:.3}");
    }

    // ---- bottom band: smalt → lead white, three ways
    let family = pal.only(&["lead white", "smalt", "yellow ochre"]);
    let sky2 = [(0.0, hex("#5b6f99")), (1.0, hex("#e4ddcc"))];
    let sky2_c = move |_: f32, y: f32| gradient(&sky2, (y - 580.0) / 420.0, Mix::Light);
    let col = |x0: f32| Mask::from_fn(f, move |x, y| if y >= 580.0 && x >= x0 && x < x0 + 333.0 { 1.0 } else { 0.0 });
    for (k, (x0, how)) in [(0.0, 0), (333.0, 1), (666.0, 2)].into_iter().enumerate() {
        let hd = st.broad().color(sky2_c).coverage(3.0).clip(true).threshold(0.5);
        let hd = match how {
            0 => hd.by_masstone(),
            1 => hd,
            _ => hd.palette(&family),
        };
        let t = Instant::now();
        c.work(&col(x0), &hd, 10 + k as u64);
        eprintln!("gradient {how}: {:.2}s", t.elapsed().as_secs_f32());
    }
    c.dry();
    // how far each sky's look is from what was asked, by height (mean color
    // of a strip vs the target there; and the spread within the strip)
    for (x0, name) in [(0.0, "masstone"), (333.0, "aimed"), (666.0, "family")] {
        let mut line = format!("{name:9}");
        for k in 0..7 {
            let y = 590.0 + 60.0 * k as f32;
            let mut acc = [0.0f32; 3];
            let mut n = 0.0;
            for j in 0..20 {
                for i in 0..60 {
                    let p = c.sample(x0 + 20.0 + i as f32 * 5.0, y + j as f32 * 2.0);
                    for q in 0..3 {
                        acc[q] += p[q];
                    }
                    n += 1.0;
                }
            }
            let m = to_oklab([acc[0] / n, acc[1] / n, acc[2] / n]);
            let w = to_oklab(sky2_c(0.0, y + 20.0));
            let d = ((m[0] - w[0]).powi(2) + (m[1] - w[1]).powi(2) + (m[2] - w[2]).powi(2)).sqrt();
            line += &format!(" y{:.0}: ΔE {d:.3}", y);
        }
        eprintln!("{line}");
    }
    c.relief(0.3, 0.02);
    o.save(&mut c);
}
