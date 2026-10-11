use paint::panel::{Ground as PanelGround, Panel, Topography};
use paint::{
    Apply, Canvas, Gesture, Ground, Held, Knife, Lead, Linen, Paint, Pigment, Style, Tool, Touch,
    hex,
};
use std::path::Path;

fn png(c: &Canvas, pixels: &[[f32; 3]], path: &Path) {
    let f = c.window();
    let rgb: Vec<u8> = pixels
        .iter()
        .flat_map(|p| {
            p.map(|v| {
                (paint::color::linear_to_srgb(v) * 255.0)
                    .round()
                    .clamp(0.0, 255.0) as u8
            })
        })
        .collect();
    image::save_buffer(path, &rgb, f.w as u32, f.h as u32, image::ColorType::Rgb8).unwrap();
}

fn state(c: &Canvas) -> Vec<u8> {
    let mut data = Vec::new();
    c.write_state(&mut data, "panel-1 matched probes").unwrap();
    data
}

fn strokes(c: &mut Canvas) {
    for (row, load, lay, color, hiding, stiff) in [
        (0, 0.95, 1.2, "#ac5b33", 0.85, 0.75),
        (1, 0.07, 0.5, "#725d3b", 0.7, 0.9),
        (2, 0.2, 0.25, "#69807b", 0.35, 0.55),
        (5, 0.65, 0.32, "#171b20", 0.16, 0.15),
    ] {
        let y = 90.0 + row as f32 * 140.0;
        let tool = Tool {
            lay,
            run: 700.0,
            ragged: 0.5,
            ..Tool::hog_flat(85.0)
        };
        let mut h = Held::new(tool, 100 + row).with_engine(c.engine());
        h.load(Paint::new(hex(color), hiding, stiff), load);
        c.drag(
            &mut h,
            &Gesture::new(vec![
                (140.0, y + 3.0),
                (380.0, y - 5.0),
                (650.0, y + 6.0),
                (870.0, y - 2.0),
            ])
            .pressure(0.72, 0.65)
            .shake(0.18),
            None,
        );
    }
    let mark = paint::graphite::hand_line(
        &[
            (140.0, 504.0),
            (350.0, 536.0),
            (580.0, 501.0),
            (865.0, 523.0),
        ],
        &[0.65],
        false,
        false,
        0.15,
        33,
    );
    c.draw(&Lead::pencil("2B").unwrap(), &mark, 0.0, 34);
    let veil = paint::Mask::from_fn(c.frame(), |x, y| {
        if x > 500.0 && x < 910.0 && y > 480.0 && y < 556.0 {
            1.0
        } else {
            0.0
        }
    });
    c.glaze(
        &Pigment::masstone_hiding(hex("#875b34"), 0.07),
        Some(&veil),
        |_, _| 0.18,
    );
    let mut knife = Knife::new(90.0);
    knife.load(Paint::body(hex("#bbaa7d")), 0.8);
    c.knife(
        &mut knife,
        &[(150.0, 650.0), (430.0, 644.0), (860.0, 656.0)],
        (0.7, 0.65),
        Some(0.35),
        true,
        0.25,
    );
}

fn parity(c: &Canvas) -> usize {
    let data = state(c);
    let (d, _) = Canvas::read_state(&mut data.as_slice()).unwrap();
    assert_eq!(data, state(&d));
    for (a, b) in [
        (c.seen(), d.seen()),
        (c.seen_lit(115.0, 55.0, 1.0), d.seen_lit(115.0, 55.0, 1.0)),
        (c.seen_lit(135.0, 25.0, 1.0), d.seen_lit(135.0, 25.0, 1.0)),
    ] {
        assert_eq!(a.len(), b.len());
        assert!(
            a.iter()
                .flatten()
                .zip(b.iter().flatten())
                .all(|(a, b)| a.to_bits() == b.to_bits())
        );
    }
    data.len()
}

fn saturation(dir: &Path) {
    let mut csv = String::from(
        "ground,touches,center_available_coats,center_absorbed_coats,total_available_coat_pixels,total_absorbed_coat_pixels,wet_coat_pixels\n",
    );
    for (name, ground) in [
        ("chalk", PanelGround::ChalkGlue),
        ("oil", PanelGround::OilPrimed),
    ] {
        let mut c = Canvas::new_window(160, 1.0, hex("#ffffff"), None)
            .with_size_mm(40.0)
            .with_panel(Panel {
                ground,
                ..Panel::chalk(7)
            });
        let center = c.frame().index(500.0, 500.0);
        for n in 0..=32 {
            if [0, 1, 2, 4, 8, 16, 32].contains(&n) {
                let available = c.panel_oil_available().unwrap();
                let absorbed = c.panel_oil_absorbed().unwrap();
                csv.push_str(&format!(
                    "{name},{n},{:.9},{:.9},{:.9},{:.9},{:.9}\n",
                    available[center],
                    absorbed[center],
                    available.iter().map(|&v| v as f64).sum::<f64>(),
                    absorbed.iter().map(|&v| v as f64).sum::<f64>(),
                    c.wet_total()
                ));
            }
            if n < 32 {
                let mut h = Held::new(Tool::hog_flat(180.0), 31).with_engine(5);
                h.load(
                    Paint {
                        oil: 2.0,
                        ..Paint::scumble(hex("#974729"))
                    },
                    1.0,
                );
                c.touch(&mut h, &Touch::at(500.0, 500.0).pressure(1.0), None);
            }
        }
    }
    std::fs::write(dir.join("uptake.csv"), csv).unwrap();
}

fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("panel_probe <output-directory>");
    let dir = Path::new(&arg);
    std::fs::create_dir_all(dir).unwrap();
    let oak = if std::env::args().nth(2).as_deref() == Some("gallery") {
        Topography::QuarterSawnOakGallery
    } else if std::env::args().nth(2).as_deref() == Some("rays") {
        Topography::QuarterSawnOakRays
    } else {
        Topography::QuarterSawnOak
    };
    let white = Panel::chalk(23).color();
    let style = Style {
        width_mm: 120.0,
        linen: Linen::fine(23),
        raw: hex("#a8966f"),
        ground: vec![Ground {
            color: white,
            hiding: 0.98,
            um: 180.0,
            stiff: 0.25,
            apply: Apply::Knife { texture: 0.05 },
            absorbent: 1.0,
        }],
        ..Style::oil()
    };
    let cases = [
        ("linen", style.prepare(960, 1.0 / 0.9, 23)),
        (
            "smooth-chalk",
            style.prepare_panel(960, 1.0 / 0.9, Panel::chalk(23)),
        ),
        (
            "oak-chalk",
            style.prepare_panel(
                960,
                1.0 / 0.9,
                Panel {
                    topography: oak,
                    ..Panel::chalk(23)
                },
            ),
        ),
        (
            "oil-primed",
            style.prepare_panel(
                960,
                1.0 / 0.9,
                Panel {
                    ground: PanelGround::OilPrimed,
                    topography: oak,
                    seed: 23,
                },
            ),
        ),
    ];
    let mut receipt = String::from(
        "case,wet_checkpoint_bytes,dry_checkpoint_bytes,roundtrip_differing_bytes,view_differing_f32_channels,absorbed_oil_coat_pixels\n",
    );
    for (name, mut c) in cases {
        strokes(&mut c);
        let wet_bytes = parity(&c);
        c.dry();
        let dry_bytes = parity(&c);
        for (view, pixels) in [
            ("color", c.seen()),
            ("gallery", c.seen_lit(115.0, 55.0, 1.0)),
            ("relief", c.seen_lit(135.0, 25.0, 1.0)),
        ] {
            png(&c, &pixels, &dir.join(format!("{name}-{view}.png")));
        }
        let oil: f64 = c
            .panel_oil_absorbed()
            .map_or(0.0, |v| v.iter().map(|&x| x as f64).sum());
        receipt.push_str(&format!("{name},{wet_bytes},{dry_bytes},0,0,{oil:.9}\n"));
    }
    std::fs::write(dir.join("roundtrip.csv"), receipt).unwrap();
    saturation(dir);
}
