//! After Caspar David Friedrich, "The Monk by the Sea" (1808–10), v2:
//! every passage painted with simulated bristle brushes in wet paint, using
//! the Friedrich style profile (`Style::friedrich`). No reference images.
//!
//! Order of work (see style.rs for sources): reddish ground under light brown
//! priming, brown underpaint for values, sky laid in with soft filberts and
//! fused wet with a badger, dried, glazed; sea and dune in body color; the
//! figure cut in last with a small sable; varnish, cracks.

use paint::{
    Canvas, Fbm, Gesture, Held, Mask, Mix, Orient, Paint, Pigment, Rgb, Rng, Shape, Style, Tool,
    gradient, hex, smoothstep,
};

fn main() {
    let t0 = std::time::Instant::now();
    let o = paint::cli::opts("friedrich_monk2");
    let mut rng = Rng::new(o.seed);
    let seed = o.seed as u32;
    let st = Style::friedrich();
    let lap = |label: &str| eprintln!("  {label:<10} {:>6.2}s", t0.elapsed().as_secs_f32());

    // ---- ground: bright reddish ground, then lighter brown priming
    let mut c = Canvas::new(o.width, 1.56, st.ground[0].0).with_weave(st.weave.0, st.weave.1, o.seed);
    for &(col, th) in &st.ground[1..] {
        c.glaze(&Pigment::semi(col), None, |_, _| th);
    }
    let (w, h) = (c.width(), c.height());
    let f = c.f;

    // ---- geometry
    let horizon = h * 0.745;
    let dune_n = Fbm::new(seed + 3, 4, 260.0);
    let dune_fine = Fbm::new(seed + 4, 5, 25.0);
    let dune_top = |x: f32| {
        h * 0.835 - 22.0 * smoothstep(1000.0, 250.0, x) * smoothstep(0.0, 250.0, x)
            + 6.0 * dune_n.get(x, 0.0)
            + 1.2 * dune_fine.get(x, 0.0)
            + 18.0 * smoothstep(620.0, 1000.0, x)
    };
    let dune_slope = |x: f32| ((dune_top(x + 4.0) - dune_top(x - 4.0)) / 8.0).atan();
    // the painter cuts the horizon with a straight edge but by hand: tiny waver
    let hz_n = Fbm::new(seed + 5, 3, 90.0);
    let hz = |x: f32| horizon + 0.35 * hz_n.get(x, 0.0);

    let sky = Mask::from_fn(f, |x, y| 1.0 - smoothstep(hz(x) - 0.6, hz(x) + 0.6, y));
    let sea = Mask::from_fn(f, |x, y| smoothstep(hz(x) - 0.5, hz(x) + 0.5, y) * (1.0 - smoothstep(dune_top(x) - 0.8, dune_top(x) + 0.8, y)));
    let dune = Mask::from_fn(f, |x, y| smoothstep(dune_top(x) - 0.8, dune_top(x) + 0.8, y));

    // ---- underpaint: thin brown glaze for the value structure, like sepia
    c.glaze(&Pigment::transparent(hex("#6e5238")), None, |_, y| 0.3 + 1.6 * smoothstep(0.4, 0.8, y / h));
    lap("ground");

    // ---- sky
    let sky_stops: [(f32, Rgb); 7] = [
        (0.00, hex("#51708e")),
        (0.18, hex("#7890a2")),
        (0.40, hex("#b9c1b8")),
        (0.52, hex("#cbcab8")),
        (0.63, hex("#8f938b")),
        (0.71, hex("#575c58")),
        (0.745, hex("#3b403f")),
    ];
    let clouds = Fbm::new(seed + 10, 6, 380.0);
    let fine = Fbm::new(seed + 11, 5, 90.0);
    let sky_color = |x: f32, y: f32| {
        let warp = clouds.get(x * 0.35, y * 1.4) * 0.07 + fine.get(x * 0.5, y * 2.0) * 0.015;
        let t = (y / h + warp * smoothstep(0.05, 0.6, y / h)).clamp(0.0, 0.745);
        gradient(&sky_stops, t, Mix::Pigment)
    };
    let sky_angle = |x: f32, y: f32| {
        let e = 6.0;
        let dy = clouds.get(x * 0.35, (y + e) * 1.4) - clouds.get(x * 0.35, (y - e) * 1.4);
        0.02 * (fine.get(x, y) + 0.2) - dy * 0.6
    };
    // lay-in: fuller paint so the ground is covered
    c.work(&sky, &st.broad().color(sky_color).angle(sky_angle).paint(0.8, 0.8).pressure(0.7, 0.9).coverage(4.0), o.seed * 100 + 1);
    lap("sky lay");
    // second, thinner pass wet into wet to tune the transitions
    c.work(&sky, &st.broad().color(sky_color).angle(sky_angle).coverage(2.0).length(60.0, 150.0).paint(0.45, 0.5), o.seed * 100 + 2);
    lap("sky 2");
    // fuse with the badger while wet
    if let Some(b) = st.blend() {
        let b = b.angle(sky_angle);
        for k in 0..st.blend_passes {
            c.work(&sky, &b, o.seed * 100 + 3 + k as u64);
        }
    }
    lap("blend");
    c.dry();

    // glazes over the dry sky: cloud masses, pale band, mist bank
    let mass = Fbm::new(seed + 13, 6, 300.0);
    c.glaze(&Pigment::transparent(hex("#6d747a")), Some(&sky), |x, y| {
        let v = y / h;
        smoothstep(-0.25, 0.6, mass.get(x * 0.45, y * 1.6)) * (0.55 * (1.0 - smoothstep(0.1, 0.45, v)) + 0.15)
    });
    c.veil(Some(&sky), hex("#d6d6c6"), |x, y| {
        let band = (-((y / h - 0.47) / 0.08).powi(2)).exp();
        band * 0.25 * (0.7 + 0.6 * mass.get(x * 0.3 + 500.0, y))
    });
    let bank = Fbm::new(seed + 12, 5, 300.0);
    c.glaze(&Pigment::semi(hex("#3a403e")), Some(&sky), |x, y| {
        let d = (horizon - y) / h;
        let top = 0.13 + 0.05 * bank.get(x * 0.4, 0.0);
        0.55 * smoothstep(top, 0.0, d) * (0.8 + 0.2 * bank.get(x * 0.6, y))
    });
    lap("glazes");

    // ---- sea: dark body color, horizontal pulls with a smaller filbert
    let swell = Fbm::new(seed + 20, 5, 120.0);
    let sea_color = |x: f32, y: f32| {
        let t = (y - horizon) / (h * 0.09);
        let base = gradient(&[(0.0, hex("#0f1618")), (0.5, hex("#17211e")), (1.0, hex("#253028"))], t, Mix::Pigment);
        let k = 1.0 + 0.12 * swell.get(x * 0.25, y * 3.0);
        [base[0] * k, base[1] * k, base[2] * k]
    };
    let sea_tool = Tool { width: 6.0, ..st.body.clone() };
    c.work(
        &sea,
        &st.body().color(sea_color).angle(|_, _| 0.0).angle_jitter(0.008).length(40.0, 130.0).coverage(4.5).paint(0.9, 0.8).threshold(0.2).clip(true),
        o.seed * 100 + 4,
    );
    let _ = sea_tool;
    // whitecaps: a few light touches with the sable, dragged lightly
    let mut sable = Held::new(Tool::round_sable(1.4), 7);
    for _ in 0..22 {
        let x = rng.range(0.0, w);
        let t = 0.35 + 0.6 * rng.f().sqrt();
        let y = hz(x) + (dune_top(x) - hz(x)) * t;
        let len = rng.range(4.0, 16.0) * t;
        sable.reload(Paint { color: hex("#8f978f"), hiding: 0.3, body: 0.4 }, 0.4);
        c.drag(&mut sable, &Gesture::new(vec![(x, y), (x + len * 0.5, y - 0.4), (x + len, y + 0.2)]).pressure(0.4, 0.2).orient(Orient::Along), Some(&sea));
    }
    // surf along the shore
    let surf = Fbm::new(seed + 21, 4, 40.0);
    c.veil(Some(&sea), hex("#8d948c"), |x, y| 0.5 * smoothstep(5.0, 0.5, dune_top(x) - y) * smoothstep(-0.2, 0.5, surf.get(x, 0.0)));
    lap("sea");

    // ---- dune: body color along the slope, drier, a bit rougher
    let sand = Fbm::new(seed + 30, 6, 140.0);
    let sand_color = |x: f32, y: f32| {
        let t = ((y - dune_top(x)) / (h - dune_top(x))).clamp(0.0, 1.0);
        let base = gradient(&[(0.0, hex("#bab5a2")), (0.3, hex("#9c957f")), (1.0, hex("#5f594b"))], t, Mix::Pigment);
        let k = 1.0 + 0.10 * sand.get(x * 0.5, y);
        [base[0] * k, base[1] * k, base[2] * k]
    };
    c.work(
        &dune,
        &st.body().color(sand_color).angle(|x, _| dune_slope(x) * 0.8).angle_jitter(0.12).length(25.0, 80.0).threshold(0.2).coverage(3.5).clip(true),
        o.seed * 100 + 5,
    );
    // a second, drier, lighter-pressure pass: dry-brush catches the tooth
    c.work(
        &dune,
        &st.body().color(sand_color).angle(|x, _| dune_slope(x) * 0.8).angle_jitter(0.15).length(20.0, 60.0).coverage(0.6).pressure(0.3, 0.5).dips(4, 0.5, 0.9).threshold(0.3).clip(true),
        o.seed * 100 + 6,
    );
    if let Some(b) = st.blend() {
        let mut b = b.angle(|x, _| dune_slope(x) * 0.8).length(40.0, 100.0).pressure(0.35, 0.45).coverage(1.5).clip(true);
        b.tool.width = 18.0;
        c.work(&dune, &b, o.seed * 100 + 9);
    }
    c.dry();
    let hol = Fbm::new(seed + 31, 4, 260.0);
    c.glaze(&Pigment::transparent(hex("#8a8170")), Some(&dune), |x, y| smoothstep(-0.1, 0.6, hol.get(x * 0.4, y * 1.2)) * 1.4);
    // grass tufts with a rigger, flicked upward
    let mut rigger = Held::new(st.line_tool(0.55), 11);
    for clump in 0..55 {
        let cx = rng.range(0.0, w);
        let cy = dune_top(cx) + rng.range(1.0, 40.0).powf(1.3).min(h - dune_top(cx) - 2.0);
        rigger.reload(Paint { color: hex("#4f4a3a"), hiding: 0.7, body: 0.7 }, 1.0);
        for _ in 0..rng.range(3.0, 8.0) as u64 {
            let x = cx + rng.range(-5.0, 5.0);
            let y = cy + rng.range(-1.0, 2.0);
            let hgt = rng.range(4.0, 11.0);
            let lean = rng.range(-2.0, 4.0);
            let g = Gesture::new(vec![(x, y), (x + lean * 0.3, y - hgt * 0.5), (x + lean, y - hgt)]).pressure(0.8, 0.1).ramps(0.02, 0.7).orient(Orient::Along);
            c.drag(&mut rigger, &g, None);
            let _ = clump;
        }
    }
    lap("dune");

    // ---- the monk, cut in last with a small sable, vertical strokes
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
    let robe = st.detail().color(|_, _| hex("#100f0d")).angle(|_, _| std::f32::consts::FRAC_PI_2).angle_jitter(0.05).length(8.0, 20.0).coverage(8.0).paint(0.97, 1.0).pressure(0.85, 1.0);
    c.work(&monk, &robe, o.seed * 100 + 7);
    c.work(&monk, &robe, o.seed * 100 + 8);
    lap("monk");

    // ---- gulls: two flicks of a small sable each
    for _ in 0..9 {
        let x = rng.range(150.0, 700.0);
        let y = rng.range(horizon - 95.0, horizon - 25.0);
        let sz = rng.range(1.6, 3.0);
        let tilt = rng.range(-0.4, 0.4);
        let mut b = Held::new(Tool::round_sable(0.7), rng.next_u64());
        b.load(Paint::scumble(hex("#c8cabf")), 0.8);
        c.drag(&mut b, &Gesture::new(vec![(x - sz, y - sz * (0.15 + tilt * 0.3)), (x - sz * 0.45, y - sz * 0.25), (x, y)]).pressure(0.3, 0.7).ramps(0.3, 0.1), None);
        c.drag(&mut b, &Gesture::new(vec![(x, y), (x + sz * 0.45, y - sz * 0.25), (x + sz, y - sz * (0.15 - tilt * 0.3))]).pressure(0.7, 0.3).ramps(0.1, 0.3), None);
    }

    // ---- finish: aged varnish, cracks, light the surface
    c.dry();
    let varnish = Fbm::new(seed + 98, 3, 400.0);
    c.glaze(&Pigment::transparent(hex("#e6d3a4")), None, |x, y| 0.4 + 0.12 * varnish.get(x, y));
    c.craquelure(12.0, 0.12, o.seed);
    c.relief(st.relief.0, st.relief.1);
    lap("finish");

    c.save(&o.out).unwrap();
    paint::cli::done(&o, t0);
}
