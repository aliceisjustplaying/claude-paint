//! After Caspar David Friedrich, "The Monk by the Sea" (1808–10).
//! Painted from memory of the style: a vast sky over a thin dark sea, a pale
//! dune and one tiny figure. Built the way he built pictures: light ground,
//! brown underpainting, body color laid in with strokes, softened with a dry
//! blending brush, then thin glazes. Aged with varnish and craquelure.

use paint::{
    Brush, Canvas, Fbm, Mask, Medium, Mix, Pigment, Rgb, Rng, Shape, StrokeFill, gradient, hex,
    smoothstep,
};

fn main() {
    let o = paintings::run::Run::new("friedrich_monk");
    let mut rng = Rng::new(o.seed);
    let seed = o.seed as u32;

    // wide format, roughly the proportions of the original, on fine linen
    let mut c = Canvas::new(o.width, 1.56, hex("#e9e2d2")).with_weave(1.15, 0.6, o.seed);
    let (w, h) = (c.width(), c.height());
    let f = c.frame();

    // ---- geometry
    let horizon = h * 0.745; // top of the sea
    let dune_n = Fbm::new(seed + 3, 4, 260.0);
    let dune_fine = Fbm::new(seed + 4, 5, 25.0);
    let dune_top = |x: f32| {
        h * 0.835 - 22.0 * smoothstep(1000.0, 250.0, x) * smoothstep(0.0, 250.0, x)
            + 6.0 * dune_n.get(x, 0.0)
            + 1.2 * dune_fine.get(x, 0.0)
            + 18.0 * smoothstep(620.0, 1000.0, x)
    };
    let dune_slope = |x: f32| ((dune_top(x + 4.0) - dune_top(x - 4.0)) / 8.0).atan();

    let sky = Mask::from_fn(f, |_, y| 1.0 - smoothstep(horizon - 0.8, horizon + 0.8, y));
    let sea = Mask::from_fn(f, |x, y| {
        smoothstep(horizon - 0.6, horizon + 0.6, y) * (1.0 - smoothstep(dune_top(x) - 0.8, dune_top(x) + 0.8, y))
    });
    let dune = Mask::from_fn(f, |x, y| smoothstep(dune_top(x) - 0.8, dune_top(x) + 0.8, y));

    // ---- 1. brown underpainting: value structure only
    let umber = Pigment::transparent(hex("#8a6a48"));
    c.glaze(&umber, None, |_, y| 0.4 + 1.4 * smoothstep(0.35, 0.75, y / h));

    // ---- 2. sky
    let sky_stops: [(f32, Rgb); 7] = [
        (0.00, hex("#55708c")),
        (0.18, hex("#7a8fa0")),
        (0.40, hex("#b8c0b6")),
        (0.52, hex("#c8c8b6")),
        (0.63, hex("#8e928a")),
        (0.71, hex("#565b57")),
        (0.745, hex("#3c4140")),
    ];
    let clouds = Fbm::new(seed + 10, 6, 380.0);
    let fine = Fbm::new(seed + 11, 5, 90.0);
    let sky_color = |x: f32, y: f32| {
        let warp = clouds.get(x * 0.35, y * 1.4) * 0.07 + fine.get(x * 0.5, y * 2.0) * 0.015;
        let t = (y / h + warp * smoothstep(0.05, 0.6, y / h)).clamp(0.0, 0.745);
        gradient(&sky_stops, t, Mix::Pigment)
    };
    // cloud banks tilt the strokes a little, like a brush following the forms
    let sky_angle = |x: f32, y: f32| {
        let e = 6.0;
        let dy = clouds.get(x * 0.35, (y + e) * 1.4) - clouds.get(x * 0.35, (y - e) * 1.4);
        0.02 * (fine.get(x, y) + 0.2) - dy * 0.6
    };

    // 2a. thin lay-in so no bare ground shows between strokes
    c.paint(Some(&sky), 0.85, Mix::Pigment, |x, y| sky_color(x, y));
    // 2b. broad strokes, wet into wet
    let broad = Brush::default().width(26.0).soft(0.55).streak(0.25, 40.0).load(1.0, 0.35).pickup(0.3).impasto(0.35);
    c.fill_strokes(
        &sky,
        &StrokeFill::new(broad)
            .length(90.0, 240.0)
            .spacing(0.5)
            .angle(sky_angle)
            .color(sky_color)
            .jitter(0.035, 0.01)
            .opacity(0.8),
        o.seed * 100 + 1,
    );
    // 2c. finer strokes to tighten the transitions
    let mid = Brush::default().width(11.0).soft(0.5).streak(0.3, 22.0).load(0.95, 0.5).pickup(0.25).impasto(0.3);
    c.fill_strokes(
        &sky,
        &StrokeFill::new(mid)
            .length(50.0, 140.0)
            .spacing(0.9)
            .angle(sky_angle)
            .color(sky_color)
            .jitter(0.025, 0.007)
            .opacity(0.6),
        o.seed * 100 + 2,
    );
    // 2d. dry blending brush: Friedrich's skies are smooth, but not airbrushed
    let blender = Brush::default().width(30.0).soft(0.8).streak(0.4, 60.0).pickup(0.35).impasto(0.12);
    c.fill_strokes(
        &sky,
        &StrokeFill::new(blender).length(120.0, 300.0).spacing(0.7).angle(sky_angle).blend(true).opacity(0.3),
        o.seed * 100 + 3,
    );

    // 2e. glazes: cloud masses, pale light band, mist bank on the sea
    let mass = Fbm::new(seed + 13, 6, 300.0);
    c.glaze(&Pigment::transparent(hex("#6d747a")), Some(&sky), |x, y| {
        let v = y / h;
        smoothstep(-0.25, 0.6, mass.get(x * 0.45, y * 1.6)) * (0.55 * (1.0 - smoothstep(0.1, 0.45, v)) + 0.15)
    });
    c.veil(Some(&sky), hex("#d6d6c6"), |x, y| {
        let band = (-((y / h - 0.47) / 0.08).powi(2)).exp();
        band * 0.3 * (0.7 + 0.6 * mass.get(x * 0.3 + 500.0, y))
    });
    let bank = Fbm::new(seed + 12, 5, 300.0);
    c.glaze(&Pigment::semi(hex("#3a403e")), Some(&sky), |x, y| {
        let d = (horizon - y) / h;
        let top = 0.13 + 0.05 * bank.get(x * 0.4, 0.0);
        0.6 * smoothstep(top, 0.0, d) * (0.8 + 0.2 * bank.get(x * 0.6, y))
    });

    // ---- 3. the sea: dark horizontal strokes
    let swell = Fbm::new(seed + 20, 5, 120.0);
    let sea_color = |x: f32, y: f32| {
        let t = (y - horizon) / (h * 0.09);
        let base = gradient(&[(0.0, hex("#101719")), (0.5, hex("#18221f")), (1.0, hex("#263029"))], t, Mix::Pigment);
        let k = 1.0 + 0.12 * swell.get(x * 0.25, y * 3.0);
        [base[0] * k, base[1] * k, base[2] * k]
    };
    c.paint(Some(&sea), 0.9, Mix::Pigment, sea_color);
    let sea_brush = Brush::default().width(5.0).soft(0.4).streak(0.35, 14.0).load(1.0, 0.5).pickup(0.2).impasto(0.3);
    c.fill_strokes(
        &sea,
        &StrokeFill::new(sea_brush).length(30.0, 110.0).spacing(0.6).angle(|_, _| 0.0).angle_jitter(0.01).color(sea_color).jitter(0.012, 0.004).opacity(0.8).threshold(0.2),
        o.seed * 100 + 4,
    );
    for i in 0..45 {
        let x = rng.range(0.0, w);
        let t = 0.35 + 0.6 * rng.f().sqrt();
        let y = horizon + (dune_top(x) - horizon) * t;
        let len = rng.range(4.0, 16.0) * t;
        let brush = Brush::default().width(rng.range(0.7, 1.6)).soft(0.7).streak(0.5, 6.0).load(0.7, 0.9).taper(0.3, 0.5).impasto(0.4);
        c.stroke(&brush, Medium::Light(hex("#9aa29a"), 0.4), &[(x, y), (x + len * 0.5, y - 0.6), (x + len, y + 0.3)], 5000 + i);
    }
    let surf = Fbm::new(seed + 21, 4, 40.0);
    c.veil(Some(&sea), hex("#8d948c"), |x, y| {
        0.5 * smoothstep(5.0, 0.5, dune_top(x) - y) * smoothstep(-0.2, 0.5, surf.get(x, 0.0))
    });

    // ---- 4. the dune: strokes following its slope
    let sand = Fbm::new(seed + 30, 6, 140.0);
    let sand_color = |x: f32, y: f32| {
        let t = ((y - dune_top(x)) / (h - dune_top(x))).clamp(0.0, 1.0);
        let base = gradient(&[(0.0, hex("#b9b4a2")), (0.3, hex("#9c957f")), (1.0, hex("#625c4e"))], t, Mix::Pigment);
        let k = 1.0 + 0.10 * sand.get(x * 0.5, y);
        [base[0] * k, base[1] * k, base[2] * k]
    };
    c.paint(Some(&dune), 0.9, Mix::Pigment, sand_color);
    let sand_brush = Brush::default().width(9.0).soft(0.45).streak(0.45, 18.0).load(0.9, 0.6).pickup(0.2).impasto(0.45);
    c.fill_strokes(
        &dune,
        &StrokeFill::new(sand_brush)
            .length(25.0, 80.0)
            .spacing(0.55)
            .angle(|x, _| dune_slope(x) * 0.8)
            .angle_jitter(0.12)
            .color(sand_color)
            .jitter(0.03, 0.008)
            .opacity(0.8)
            .threshold(0.2),
        o.seed * 100 + 5,
    );
    let hollow = Pigment::transparent(hex("#8a8170"));
    let hol = Fbm::new(seed + 31, 4, 260.0);
    c.glaze(&hollow, Some(&dune), |x, y| smoothstep(-0.1, 0.6, hol.get(x * 0.4, y * 1.2)) * 1.6);
    for clump in 0..55 {
        let cx = rng.range(0.0, w);
        let cy = dune_top(cx) + rng.range(1.0, 40.0).powf(1.3).min(h - dune_top(cx) - 2.0);
        for j in 0..rng.range(3.0, 8.0) as u64 {
            let x = cx + rng.range(-5.0, 5.0);
            let y = cy + rng.range(-1.0, 2.0);
            let hgt = rng.range(4.0, 11.0);
            let lean = rng.range(-2.0, 4.0);
            let brush = Brush::default().width(rng.range(0.45, 0.8)).soft(0.5).streak(0.0, 1.0).taper(0.0, 0.9).impasto(0.3);
            c.stroke(&brush, Medium::Glaze(Pigment::semi(hex("#57513f")), 0.8), &[(x, y), (x + lean * 0.3, y - hgt * 0.5), (x + lean, y - hgt)], 7000 + clump * 16 + j);
        }
    }

    // ---- 5. the monk
    let mx = 318.0;
    let my = dune_top(mx) + 3.0;
    let s = 44.0;
    let body = Shape::new().smooth_poly(&[
        (mx - 0.035 * s, my - 0.88 * s),
        (mx + 0.035 * s, my - 0.88 * s),
        (mx + 0.10 * s, my - 0.83 * s),
        (mx + 0.125 * s, my - 0.70 * s),
        (mx + 0.10 * s, my - 0.52 * s),
        (mx + 0.115 * s, my - 0.25 * s),
        (mx + 0.14 * s, my - 0.02 * s),
        (mx + 0.05 * s, my + 0.005 * s),
        (mx - 0.06 * s, my),
        (mx - 0.125 * s, my - 0.01 * s),
        (mx - 0.10 * s, my - 0.30 * s),
        (mx - 0.095 * s, my - 0.55 * s),
        (mx - 0.105 * s, my - 0.78 * s),
    ]);
    let head = Shape::new().ellipse(mx + 0.004 * s, my - 0.935 * s, 0.047 * s, 0.062 * s);
    let monk = Mask::from_shape(f, body).union(&Mask::from_shape(f, head));
    let shadow = Mask::from_shape(f, Shape::new().ellipse(mx + 0.02 * s, my + 0.01 * s, 0.22 * s, 0.025 * s)).blur(0.8);
    c.glaze(&Pigment::transparent(hex("#5a5446")), Some(&shadow), |_, _| 1.2);
    // painted, not stamped: small vertical strokes clipped to the figure
    c.fill(&monk, hex("#12110f"), 0.9, Mix::Linear);
    let robe = Brush::default().width(1.6).soft(0.4).streak(0.4, 6.0).impasto(0.5);
    c.fill_strokes(
        &monk,
        &StrokeFill::new(robe).length(6.0, 16.0).spacing(0.5).angle(|_, _| std::f32::consts::FRAC_PI_2).color(|_, _| hex("#0d0c0b")).jitter(0.015, 0.0).threshold(0.1),
        o.seed * 100 + 6,
    );

    // ---- 6. gulls
    for i in 0..9 {
        let x = rng.range(150.0, 700.0);
        let y = rng.range(horizon - 95.0, horizon - 25.0);
        let sz = rng.range(1.6, 3.0);
        let tilt = rng.range(-0.4, 0.4);
        let brush = Brush::default().width(0.6).soft(0.7).streak(0.0, 1.0).taper(0.5, 0.5).impasto(0.3);
        let pts = [(x - sz, y - sz * (0.15 + tilt * 0.3)), (x - sz * 0.45, y - sz * 0.25), (x, y), (x + sz * 0.45, y - sz * 0.25), (x + sz, y - sz * (0.15 - tilt * 0.3))];
        c.stroke(&brush, Medium::Light(hex("#c4c6bc"), 0.5), &pts, 9000 + i);
    }

    // ---- 7. finish: aged varnish, cracks, then light the surface
    let varnish = Fbm::new(seed + 98, 3, 400.0);
    c.glaze(&Pigment::varnish(hex("#e6d3a4")), None, |x, y| 0.4 + 0.12 * varnish.get(x, y));
    c.mottle(60.0, 0.02, seed + 99);
    c.craquelure(12.0, 0.12, o.seed);
    c.relief(0.7, 0.03);

    o.save(&mut c);
}
