use paint::panel::{Ground as PanelGround, Panel, Topography};
use paint::{
    Apply, Canvas, Gesture, Ground, Held, Linen, Mask, Paint, Palette, Rng, Style, Tool, hex,
};
use std::path::Path;

pub fn prepare(name: &str, width: usize, topography: Topography) -> Canvas {
    let style = Style {
        width_mm: 120.0,
        linen: Linen::fine(23),
        raw: hex("#a8966f"),
        ground: vec![Ground {
            color: Panel::chalk(23).color(),
            hiding: 0.98,
            um: 180.0,
            stiff: 0.25,
            apply: Apply::Knife { texture: 0.05 },
            absorbent: 1.0,
        }],
        ..Style::oil()
    };
    match name {
        "linen" => style.prepare(width, 1.0, 23),
        "chalk" => style.prepare_panel(
            width,
            1.0,
            Panel {
                topography,
                ..Panel::chalk(23)
            },
        ),
        "smooth" => style.prepare_panel(width, 1.0, Panel::chalk(23)),
        "oil" => style.prepare_panel(
            width,
            1.0,
            Panel {
                topography,
                ground: PanelGround::OilPrimed,
                seed: 23,
            },
        ),
        _ => unreachable!(),
    }
}

fn band(c: &mut Canvas, paint: Paint, rect: [f32; 4], load: f32, lay: f32, seed: u64) {
    let [x0, y0, x1, y1] = rect;
    let clip = Mask::from_fn(c.frame(), |x, y| {
        let edge = (x - x0).min(x1 - x).min(y - y0).min(y1 - y);
        paint::smoothstep(0.0, 8.0, edge)
    });
    let tool = Tool {
        lay,
        run: 2200.0,
        pickup: 0.02,
        push: 0.01,
        ragged: 0.6,
        ..Tool::round_sable(95.0)
    };
    let mut rng = Rng::new(seed);
    let n = ((y1 - y0) / 35.0).ceil() as usize;
    for i in 0..n {
        let y = y0 + 22.0 + i as f32 * 35.0;
        let wobble = rng.range(-8.0, 8.0);
        let mut h = Held::new(tool.clone(), seed + i as u64).with_engine(c.engine());
        h.load(paint, load * rng.range(0.85, 1.15));
        c.drag(
            &mut h,
            &Gesture::new(vec![
                (x0 - 25.0, y),
                ((x0 + x1) * 0.5, y + wobble),
                (x1 + 30.0, y - 0.4 * wobble),
            ])
            .pressure(0.68, 0.58)
            .ramps(0.02, 0.02)
            .shake(0.25),
            Some(&clip),
        );
    }
}

pub fn views(c: &Canvas, dir: &Path, name: &str, receipt: &mut String) {
    let mut data = Vec::new();
    c.write_state(&mut data, "panel-2 layers").unwrap();
    let (d, _) = Canvas::read_state(&mut data.as_slice()).unwrap();
    let mut restored = Vec::new();
    d.write_state(&mut restored, "panel-2 layers").unwrap();
    assert_eq!(data, restored);
    let f = c.window();
    for (view, a, b) in [
        ("color", c.seen(), d.seen()),
        (
            "gallery",
            c.seen_lit(115.0, 55.0, 1.0),
            d.seen_lit(115.0, 55.0, 1.0),
        ),
        (
            "relief",
            c.seen_lit(135.0, 25.0, 1.0),
            d.seen_lit(135.0, 25.0, 1.0),
        ),
    ] {
        assert_eq!(a.len(), b.len());
        assert!(
            a.iter()
                .flatten()
                .zip(b.iter().flatten())
                .all(|(a, b)| a.to_bits() == b.to_bits())
        );
        let rgb: Vec<u8> = a
            .iter()
            .flat_map(|p| {
                p.map(|v| {
                    (paint::color::linear_to_srgb(v) * 255.0)
                        .round()
                        .clamp(0.0, 255.0) as u8
                })
            })
            .collect();
        image::save_buffer(
            dir.join(format!("{name}-{view}.png")),
            &rgb,
            f.w as u32,
            f.h as u32,
            image::ColorType::Rgb8,
        )
        .unwrap();
    }
    receipt.push_str(&format!("{name},{},0,0\n", data.len()));
}

fn metrics(c: &Canvas, name: &str, stage: &str, rect: [f32; 4], output: &mut String) {
    let f = c.window();
    let color = c.seen();
    let gallery = c.seen_lit(115.0, 55.0, 1.0);
    let mut n = 0.0;
    let mut lum = 0.0;
    let mut sheen = 0.0;
    let mut gloss = 0.0;
    let mut absorbed = 0.0;
    for i in 0..f.w * f.h {
        let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
        if x < rect[0] || x > rect[2] || y < rect[1] || y > rect[3] {
            continue;
        }
        n += 1.0;
        lum += paint::color::luminance(color[i]) as f64;
        sheen += (paint::color::luminance(gallery[i]) - paint::color::luminance(color[i])) as f64;
        gloss += c.panel_gloss().map_or(0.0, |g| g[i]) as f64;
        absorbed += c.panel_oil_absorbed().map_or(0.0, |v| v[i]) as f64;
    }
    let gloss = if c.panel().is_some() {
        format!("{:.9}", gloss / n)
    } else {
        "NA".into()
    };
    let absorbed = if c.panel().is_some() {
        format!("{:.9}", absorbed / n)
    } else {
        "NA".into()
    };
    output.push_str(&format!(
        "{name},{stage},{:.9},{:.9},{gloss},{absorbed}\n",
        lum / n,
        sheen / n
    ));
}

fn layers(c: &mut Canvas, name: &str, dir: &Path, receipt: &mut String, numbers: &mut String) {
    let warm = Paint {
        oil: 1.5,
        ..Paint::new(hex("#a26a32"), 0.18, 0.18)
    };
    band(c, warm, [70.0, 70.0, 930.0, 350.0], 0.16, 0.16, 701);
    c.dry();
    metrics(c, name, "warm-first", [160.0, 120.0, 830.0, 310.0], numbers);
    views(c, dir, &format!("{name}-warm"), receipt);
    let dark = Paint {
        oil: 1.6,
        ..Paint::new(hex("#29211a"), 0.55, 0.12)
    };
    band(c, dark, [510.0, 70.0, 930.0, 350.0], 0.10, 0.18, 702);
    band(c, dark, [70.0, 430.0, 930.0, 610.0], 0.10, 0.12, 703);
    c.dry();
    metrics(
        c,
        name,
        "dark-over-warm",
        [570.0, 120.0, 840.0, 310.0],
        numbers,
    );
    metrics(c, name, "dark-first", [160.0, 470.0, 830.0, 575.0], numbers);
    let pal = Palette::tube_box().only(&["lead white"]);
    band(
        c,
        pal.pile(vec![(0, 1.0)]).laid(0.02),
        [70.0, 690.0, 930.0, 900.0],
        0.95,
        2.8,
        704,
    );
    c.dry();
    views(c, dir, &format!("{name}-layers"), receipt);
}

pub fn winter(c: &mut Canvas, load_scale: f32, broken: bool) {
    let pal = Palette::tube_box().only(&[
        "lead white",
        "yellow ochre",
        "raw umber",
        "bone black",
        "cobalt blue",
    ]);
    let tone = pal
        .pile(if broken {
            vec![(0, 0.05), (1, 0.15), (2, 0.80)]
        } else {
            vec![(0, 0.4), (1, 0.3), (2, 0.3)]
        })
        .laid(if broken { 0.28 } else { 0.45 })
        .with_hiding(if broken { 0.55 } else { 0.3 });
    band(
        c,
        tone,
        [40.0, 40.0, 960.0, 960.0],
        (if broken { 0.18 } else { 0.24 }) * load_scale,
        if broken { 0.20 } else { 0.36 },
        810,
    );
    c.dry();
    let drawing = pal.pile(vec![(2, 0.65), (3, 0.35)]).laid(0.35);
    let mut drawing_brush = Held::new(
        Tool {
            point: 1.0,
            lay: 0.22,
            run: 1400.0,
            ..Tool::rigger(3.0)
        },
        809,
    )
    .with_engine(c.engine());
    drawing_brush.load(drawing, 0.35 * load_scale);
    c.drag(
        &mut drawing_brush,
        &Gesture::new(vec![
            (80.0, 630.0),
            (360.0, 596.0),
            (650.0, 628.0),
            (930.0, 592.0),
        ])
        .pressure(0.35, 0.2)
        .shake(0.2),
        None,
    );
    c.dry();
    let sky = pal
        .pile(vec![(0, 0.93), (4, 0.04), (2, 0.03)])
        .laid(0.4)
        .with_hiding(0.48);
    if broken {
        let clip = Mask::from_fn(c.frame(), |x, y| {
            paint::smoothstep(
                0.0,
                10.0,
                (x - 60.0).min(940.0 - x).min(y - 60.0).min(610.0 - y),
            )
        });
        let mut rng = Rng::new(811);
        for i in 0..95 {
            let x = rng.range(55.0, 850.0);
            let y = rng.range(65.0, 580.0);
            let len = rng.range(65.0, 190.0);
            let tool = Tool {
                lay: 0.6,
                run: 320.0,
                pickup: 0.02,
                push: 0.01,
                ragged: 0.8,
                ..Tool::hog_flat(rng.range(45.0, 85.0))
            };
            let mut h = Held::new(tool, 811 + i).with_engine(c.engine());
            h.load(sky, rng.range(0.035, 0.10) * load_scale);
            c.drag(
                &mut h,
                &Gesture::new(vec![
                    (x, y),
                    (x + 0.45 * len, y + rng.range(-14.0, 14.0)),
                    (x + len, y + rng.range(-10.0, 10.0)),
                ])
                .pressure(rng.range(0.25, 0.5), rng.range(0.08, 0.28))
                .ramps(0.08, 0.25)
                .shake(0.7),
                Some(&clip),
            );
        }
    } else {
        band(
            c,
            sky,
            [60.0, 60.0, 940.0, 610.0],
            0.30 * load_scale,
            0.55,
            811,
        );
    }
    c.dry();
    let white = pal.pile(vec![(0, 1.0)]).laid(0.02);
    let mut rng = Rng::new(812);
    for row in 0..4 {
        let y = 650.0 + row as f32 * 70.0;
        let tool = Tool {
            lay: 3.0,
            run: 1800.0,
            ragged: 0.7,
            ..Tool::hog_flat(180.0)
        };
        let mut h = Held::new(tool, 812 + row).with_engine(c.engine());
        h.load(white, 0.9 * load_scale);
        c.drag(
            &mut h,
            &Gesture::new(vec![
                (70.0, y + 15.0),
                (400.0, y - 24.0),
                (700.0, y + rng.range(-14.0, 14.0)),
                (935.0, y - 10.0),
            ])
            .pressure(0.75, 0.65)
            .shake(0.4),
            None,
        );
    }
    c.dry();
    let branch = pal.pile(vec![(2, 0.4), (3, 0.6)]).laid(0.32);
    for (k, (x, y)) in [(250.0, 840.0), (520.0, 790.0), (790.0, 855.0)]
        .into_iter()
        .enumerate()
    {
        let top = y - 460.0;
        let tool = Tool {
            point: 1.0,
            lay: 0.45,
            run: 1500.0,
            ..Tool::rigger(7.0)
        };
        let mut h = Held::new(tool.clone(), 820 + k as u64).with_engine(c.engine());
        h.load(branch, 0.6 * load_scale);
        c.drag(
            &mut h,
            &Gesture::new(vec![(x, y), (x - 8.0, y - 190.0), (x + 7.0, top)])
                .pressure(0.55, 0.1)
                .shake(0.25),
            None,
        );
        for j in 0..7 {
            let at = y - 90.0 - j as f32 * 43.0;
            let side = if j % 2 == 0 { -1.0 } else { 1.0 };
            let reach = rng.range(45.0, 100.0);
            let mut h = Held::new(tool.clone(), 850 + k as u64 * 10 + j).with_engine(c.engine());
            h.load(branch, 0.35 * load_scale);
            c.drag(
                &mut h,
                &Gesture::new(vec![
                    (x, at),
                    (x + side * reach * 0.5, at - 38.0),
                    (x + side * reach, at - 92.0),
                ])
                .pressure(0.4, 0.03)
                .shake(0.3),
                None,
            );
        }
    }
    c.dry();
}

pub fn run_layers() {
    let args: Vec<_> = std::env::args().collect();
    let dir = Path::new(args.get(1).expect("panel_layers <directory> [width]"));
    let width = args.get(2).map_or(960, |s| s.parse().unwrap());
    std::fs::create_dir_all(dir).unwrap();
    let mut receipt = String::from(
        "case,checkpoint_bytes,roundtrip_differing_bytes,view_differing_f32_channels\n",
    );
    let mut numbers = String::from(
        "support,stage,color_luminance,gallery_minus_color_luminance,mean_gloss,mean_absorbed_oil_coats\n",
    );
    for name in ["linen", "chalk", "oil"] {
        let mut c = prepare(name, width, Topography::QuarterSawnOakRays);
        views(&c, dir, &format!("{name}-bare"), &mut receipt);
        layers(&mut c, name, dir, &mut receipt, &mut numbers);
    }
    for name in ["linen", "smooth", "chalk", "oil"] {
        let mut c = prepare(name, width, Topography::QuarterSawnOakRays);
        winter(&mut c, 1.0, false);
        views(&c, dir, &format!("{name}-winter"), &mut receipt);
    }
    std::fs::write(dir.join("roundtrip.csv"), receipt).unwrap();
    std::fs::write(dir.join("layer-metrics.csv"), numbers).unwrap();
}
