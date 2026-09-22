//! After Caspar David Friedrich, "The Monk by the Sea" (1808–10).
//! Painted from memory of the style: a vast sky over a thin dark sea, a pale
//! dune and one tiny figure. Built the way he built pictures: light ground,
//! brown underpainting, body color, then thin glazes.

use paint::{Brush, Canvas, Fbm, Mask, Medium, Mix, Pigment, Rng, Shape, gradient, hex, smoothstep};

fn main() {
    let t0 = std::time::Instant::now();
    let o = paint::cli::opts("friedrich_monk");
    let mut rng = Rng::new(o.seed);
    let seed = o.seed as u32;

    // wide format, roughly the proportions of the original
    let mut c = Canvas::new(o.width, 1.56, hex("#e9e2d2"));
    let (w, h) = (c.width(), c.height());
    let f = c.f;

    // key horizontal lines
    let horizon = h * 0.745; // top of the sea
    let dune_n = Fbm::new(seed + 3, 4, 260.0);
    let dune_fine = Fbm::new(seed + 4, 5, 25.0);
    let dune_top = |x: f32| {
        // dune rises gently toward the left third then falls away right
        let n = &dune_n;
        h * 0.835 - 22.0 * smoothstep(1000.0, 250.0, x) * smoothstep(0.0, 250.0, x) + 6.0 * n.get(x, 0.0)
            + 1.2 * dune_fine.get(x, 0.0)
            + 18.0 * smoothstep(620.0, 1000.0, x)
    };

    // ---- 1. brown underpainting: value structure only
    let umber = Pigment::transparent(hex("#8a6a48"));
    c.glaze(&umber, None, |_, y| {
        let v = y / h;
        0.4 + 1.4 * smoothstep(0.35, 0.75, v)
    });

    // ---- 2. the sky: body color, pigment-mixed gradient
    let sky_stops = [
        (0.00, hex("#58708a")),
        (0.18, hex("#7c90a0")),
        (0.40, hex("#b8c0b8")),
        (0.52, hex("#c6c7b8")),
        (0.63, hex("#8e928c")),
        (0.71, hex("#555a58")),
        (0.745, hex("#3b403f")),
    ];
    let clouds = Fbm::new(seed + 10, 6, 380.0);
    let fine = Fbm::new(seed + 11, 5, 90.0);
    let sky = Mask::from_fn(f, |_, y| 1.0 - smoothstep(horizon - 1.0, horizon + 1.0, y));
    c.paint(Some(&sky), 1.0, Mix::Pigment, |x, y| {
        // horizontally stretched cloud masses shift the gradient up and down
        let warp = clouds.get(x * 0.35, y * 1.4) * 0.07 + fine.get(x * 0.5, y * 2.0) * 0.015;
        let t = (y / h + warp * smoothstep(0.05, 0.6, y / h)).clamp(0.0, 0.745);
        gradient(&sky_stops, t, Mix::Pigment)
    });

    // cloud masses: thin gray glaze where the stretched noise is high,
    // strongest in the upper sky, fading toward the luminous band
    let mass = Fbm::new(seed + 13, 6, 300.0);
    c.glaze(&Pigment::transparent(hex("#6d747a")), Some(&sky), |x, y| {
        let v = y / h;
        let n = mass.get(x * 0.45, y * 1.6);
        smoothstep(-0.25, 0.6, n) * (0.55 * (1.0 - smoothstep(0.1, 0.45, v)) + 0.15)
    });
    // a pale scumble lifting the light band
    c.veil(Some(&sky), hex("#d6d6c6"), |x, y| {
        let v = y / h;
        let band = (-((v - 0.47) / 0.08).powi(2)).exp();
        band * 0.35 * (0.7 + 0.6 * mass.get(x * 0.3 + 500.0, y))
    });

    // dark mist bank sitting on the sea
    let bank = Fbm::new(seed + 12, 5, 300.0);
    c.glaze(&Pigment::semi(hex("#3a403e")), Some(&sky), |x, y| {
        let d = (horizon - y) / h; // 0 at horizon
        let top = 0.13 + 0.05 * bank.get(x * 0.4, 0.0);
        0.6 * smoothstep(top, 0.0, d) * (0.8 + 0.2 * bank.get(x * 0.6, y))
    });

    // ---- 3. long soft horizontal strokes, barely visible (glazing brush)
    for i in 0..220 {
        let y = rng.range(-10.0, horizon - 8.0);
        let x = rng.range(-150.0, w);
        let len = rng.range(180.0, 520.0);
        let v = y / h;
        let light = gradient(&sky_stops, (v + rng.range(-0.04, 0.04)).clamp(0.0, 0.74), Mix::Pigment);
        let brush = Brush::default()
            .width(rng.range(14.0, 40.0))
            .soft(0.8)
            .streak(0.2, rng.range(20.0, 50.0))
            .load(1.0, 0.25);
        let sag = rng.range(-6.0, 6.0);
        let pts = [(x, y), (x + len * 0.5, y + sag), (x + len, y + rng.range(-3.0, 3.0))];
        c.stroke(&brush, Medium::Body(light, 0.12), &pts, o.seed * 1000 + i);
    }

    // ---- 4. the sea: near-black green band with a few whitecaps
    let sea = Mask::from_fn(f, |x, y| smoothstep(horizon - 0.6, horizon + 0.6, y) * (1.0 - smoothstep(dune_top(x) - 0.8, dune_top(x) + 0.8, y)));
    let swell = Fbm::new(seed + 20, 5, 120.0);
    c.paint(Some(&sea), 1.0, Mix::Pigment, |x, y| {
        let t = (y - horizon) / (h * 0.09);
        let base = gradient(&[(0.0, hex("#101719")), (0.5, hex("#18221f")), (1.0, hex("#263029"))], t, Mix::Pigment);
        let k = 1.0 + 0.12 * swell.get(x * 0.25, y * 3.0);
        [base[0] * k, base[1] * k, base[2] * k]
    });
    for i in 0..45 {
        let x = rng.range(0.0, w);
        // mostly breaking near the shore
        let t = 0.35 + 0.6 * rng.f().sqrt();
        let y = horizon + (dune_top(x) - horizon) * t;
        let len = rng.range(4.0, 16.0) * t;
        let brush = Brush::default().width(rng.range(0.7, 1.6)).soft(0.7).streak(0.5, 6.0).load(0.7, 0.9).taper(0.3, 0.5);
        let pts = [(x, y), (x + len * 0.5, y - 0.6), (x + len, y + 0.3)];
        c.stroke(&brush, Medium::Light(hex("#9aa29a"), 0.4), &pts, 5000 + i);
    }

    // a faint line of surf where the sea meets the sand
    let surf = Fbm::new(seed + 21, 4, 40.0);
    c.veil(Some(&sea), hex("#8d948c"), |x, y| {
        let d = dune_top(x) - y;
        0.5 * smoothstep(5.0, 0.5, d) * smoothstep(-0.2, 0.5, surf.get(x, 0.0))
    });

    // ---- 5. the dune: pale sand, a few grass tufts and darker hollows
    let dune = Mask::from_fn(f, |x, y| smoothstep(dune_top(x) - 0.7, dune_top(x) + 0.7, y));
    let sand = Fbm::new(seed + 30, 6, 140.0);
    c.paint(Some(&dune), 1.0, Mix::Pigment, |x, y| {
        let t = ((y - dune_top(x)) / (h - dune_top(x))).clamp(0.0, 1.0);
        let base = gradient(&[(0.0, hex("#b9b4a2")), (0.3, hex("#9c957f")), (1.0, hex("#625c4e"))], t, Mix::Pigment);
        let k = 1.0 + 0.10 * sand.get(x * 0.5, y);
        [base[0] * k, base[1] * k, base[2] * k]
    });
    let hollow = Pigment::transparent(hex("#8a8170"));
    let hol = Fbm::new(seed + 31, 4, 260.0);
    c.glaze(&hollow, Some(&dune), |x, y| smoothstep(-0.1, 0.6, hol.get(x * 0.4, y * 1.2)) * 1.6);
    // wind-combed sand: long faint strokes following the dune's slope
    for i in 0..90 {
        let x = rng.range(-60.0, w);
        let len = rng.range(40.0, 160.0);
        let off = rng.range(3.0, h - dune_top(x));
        let pts: Vec<(f32, f32)> = (0..5).map(|k| {
            let xx = x + len * k as f32 / 4.0;
            (xx, dune_top(xx) + off * (1.0 + 0.1 * (k as f32 - 2.0) / 2.0))
        }).collect();
        let dark = rng.chance(0.55);
        let col = if dark { hex("#6f6857") } else { hex("#c9c3b0") };
        let brush = Brush::default().width(rng.range(2.0, 7.0)).soft(0.8).streak(0.6, 12.0).load(0.8, 0.8);
        c.stroke(&brush, Medium::Body(col, 0.22), &pts, 6000 + i);
    }
    // grass in clumps, mostly along the crest
    for clump in 0..55 {
        let cx = rng.range(0.0, w);
        let cy = dune_top(cx) + rng.range(1.0, 40.0).powf(1.3).min(h - dune_top(cx) - 2.0);
        for j in 0..rng.range(3.0, 8.0) as u64 {
            let x = cx + rng.range(-5.0, 5.0);
            let y = cy + rng.range(-1.0, 2.0);
            let hgt = rng.range(4.0, 11.0);
            let lean = rng.range(-2.0, 4.0);
            let brush = Brush::default().width(rng.range(0.45, 0.8)).soft(0.5).streak(0.0, 1.0).taper(0.0, 0.9);
            c.stroke(&brush, Medium::Glaze(Pigment::semi(hex("#57513f")), 0.8), &[(x, y), (x + lean * 0.3, y - hgt * 0.5), (x + lean, y - hgt)], 7000 + clump * 16 + j);
        }
    }

    // ---- 6. the monk: a tiny dark figure seen from behind
    let mx = 318.0;
    let my = dune_top(mx) + 3.0;
    let s = 44.0; // figure height in units
    // head, then a robe: sloped shoulders, a slight bulge where the arm
    // is raised to the chin, narrowing at the waist, flaring at the hem
    let body = Shape::new().smooth_poly(&[
        (mx - 0.035 * s, my - 0.88 * s), // neck L
        (mx + 0.035 * s, my - 0.88 * s), // neck R
        (mx + 0.10 * s, my - 0.83 * s),  // shoulder R
        (mx + 0.125 * s, my - 0.70 * s), // raised elbow R
        (mx + 0.10 * s, my - 0.52 * s),  // waist R
        (mx + 0.115 * s, my - 0.25 * s),
        (mx + 0.14 * s, my - 0.02 * s),  // hem R
        (mx + 0.05 * s, my + 0.005 * s),
        (mx - 0.06 * s, my),
        (mx - 0.125 * s, my - 0.01 * s), // hem L
        (mx - 0.10 * s, my - 0.30 * s),
        (mx - 0.095 * s, my - 0.55 * s), // waist L
        (mx - 0.105 * s, my - 0.78 * s), // shoulder L
    ]);
    let head = Shape::new().ellipse(mx + 0.004 * s, my - 0.935 * s, 0.047 * s, 0.062 * s);
    let monk = Mask::from_shape(f, body).union(&Mask::from_shape(f, head));
    // contact shadow where the robe meets the sand
    let shadow = Mask::from_shape(f, Shape::new().ellipse(mx + 0.02 * s, my + 0.01 * s, 0.22 * s, 0.025 * s)).blur(0.8);
    c.glaze(&Pigment::transparent(hex("#5a5446")), Some(&shadow), |_, _| 1.2);
    c.fill(&monk, hex("#0d0c0b"), 1.0, Mix::Linear);

    // ---- 7. gulls, tiny pale flicks against the dark bank
    for i in 0..9 {
        let x = rng.range(150.0, 700.0);
        let y = rng.range(horizon - 95.0, horizon - 25.0);
        let sz = rng.range(1.6, 3.0);
        let tilt = rng.range(-0.4, 0.4);
        let brush = Brush::default().width(0.6).soft(0.7).streak(0.0, 1.0).taper(0.5, 0.5);
        // shallow gull wing: a soft arc dipping at the body
        let pts = [(x - sz, y - sz * (0.15 + tilt * 0.3)), (x - sz * 0.45, y - sz * 0.25), (x, y), (x + sz * 0.45, y - sz * 0.25), (x + sz, y - sz * (0.15 - tilt * 0.3))];
        c.stroke(&brush, Medium::Light(hex("#c4c6bc"), 0.5), &pts, 9000 + i);
    }

    // ---- 8. finish: unify with a thin warm varnish, surface texture
    c.glaze(&Pigment::transparent(hex("#e8d9b0")), None, |_, _| 0.35);
    c.mottle(60.0, 0.025, seed + 99);
    c.canvas_texture(1.1, 0.035, o.seed);

    c.save(&o.out).unwrap();
    paint::cli::done(&o, t0);
}
