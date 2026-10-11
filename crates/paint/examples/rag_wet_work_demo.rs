use paint::{
    Canvas, Gesture, Held, Linen, Mask, Paint, Rgb, Tool,
    color::{hex, linear_to_srgb},
    rag::{Rag, transport::Material},
    working_layer::WorkingSurface,
};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

fn patch(c: &mut Canvas, bounds: [f32; 4], color: &str, seed: u64) {
    let mask = Mask::from_fn(c.frame(), |x, y| {
        if x >= bounds[0] && x <= bounds[2] && y >= bounds[1] && y <= bounds[3] {
            1.0
        } else {
            0.0
        }
    });
    let mut brush = Held::new(Tool::hog_flat(145.0), seed).with_engine(5);
    brush.load(
        Paint::body(hex(color)).with_stiff(0.55).with_thinner(0.25),
        0.7,
    );
    for (j, y) in [170.0, 215.0, 260.0, 305.0].into_iter().enumerate() {
        let ends = if j % 2 == 0 {
            ((bounds[0] + 25.0, y), (bounds[2] - 25.0, y))
        } else {
            ((bounds[2] - 25.0, y), (bounds[0] + 25.0, y))
        };
        c.drag(
            &mut brush,
            &Gesture::line(ends.0, ends.1).pressure(0.6, 0.6).shake(0.25),
            Some(&mask),
        );
    }
}

fn fixture(width: usize, two_colors: bool, dried: bool) -> Canvas {
    let mut c = Canvas::new_window(width, 1.6, hex("#c8bba5"), None)
        .with_engine(5)
        .with_size_mm(100.0)
        .with_linen(Linen::fine(23));
    if two_colors {
        patch(&mut c, [120.0, 110.0, 470.0, 355.0], "#b97948", 19);
        patch(&mut c, [485.0, 110.0, 830.0, 355.0], "#496978", 29);
    } else {
        patch(&mut c, [120.0, 110.0, 720.0, 355.0], "#765b46", 19);
    }
    if dried {
        c.dry();
    }
    c
}

fn save(out: &Path, label: &str, w: &WorkingSurface) -> io::Result<()> {
    for (view, pixels) in [
        ("color", w.seen()),
        ("gallery", w.seen_lit(115.0, 55.0, 1.0)),
    ] {
        let bytes: Vec<u8> = pixels
            .into_iter()
            .flat_map(|p: Rgb| {
                p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8)
            })
            .collect();
        let f = w.frame();
        image::save_buffer(
            out.join(format!("{label}-{view}.png")),
            &bytes,
            f.w as u32,
            f.h as u32,
            image::ColorType::Rgb8,
        )
        .map_err(io::Error::other)?;
    }
    Ok(())
}

fn row(
    csv: &mut fs::File,
    label: &str,
    w: &WorkingSurface,
    b: &Held,
    r: Option<&Rag>,
    expected: Material,
    net: f64,
) -> io::Result<()> {
    let canvas = w.inventory();
    let brush = w.brush_inventory(b);
    let mut total = canvas;
    total.add(brush);
    let mut cloth = Material::default();
    let (mut evap, mut input) = (0.0, 0.0);
    if let Some(r) = r {
        let t = r.transport.as_ref().unwrap();
        cloth = t.inventory();
        total.add(cloth);
        total.add(t.waste);
        evap = t.evaporated_solvent_mm3;
        input = t.solvent_input_mm3;
        total.solvent_mm3 += evap - input;
    }
    let paint = total.paint_mm3 - expected.paint_mm3;
    let solvent = total.solvent_mm3 - expected.solvent_mm3;
    let cure = total.cure_moment - expected.cure_moment;
    let latent = total
        .latent_moment
        .into_iter()
        .zip(expected.latent_moment)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0, f64::max);
    let props = total
        .property_moment
        .into_iter()
        .zip(expected.property_moment)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0, f64::max);
    let near = |a: f64, b: f64| {
        assert!(
            (a - b).abs() < 5e-5 * b.abs().max(1.0),
            "{label}: {a} != {b}"
        )
    };
    near(total.paint_mm3, expected.paint_mm3);
    near(total.solvent_mm3, expected.solvent_mm3);
    near(total.cure_moment, expected.cure_moment);
    for (a, b) in total.latent_moment.into_iter().zip(expected.latent_moment) {
        near(a, b);
    }
    for (a, b) in total
        .property_moment
        .into_iter()
        .zip(expected.property_moment)
    {
        near(a, b);
    }
    writeln!(
        csv,
        "{label},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{evap:.9},{input:.9},{net:.9},{paint:.9},{solvent:.9},{latent:.9},{props:.9},{cure:.9}",
        canvas.paint_mm3,
        canvas.solvent_mm3,
        w.opened_paint_mm3(),
        brush.paint_mm3,
        brush.solvent_mm3,
        cloth.paint_mm3,
        cloth.solvent_mm3
    )
}

fn main() -> io::Result<()> {
    let dir = std::env::args()
        .nth(1)
        .expect("rag_wet_work_demo <output-dir> [width]");
    let width = std::env::args()
        .nth(2)
        .map(|x| x.parse().unwrap())
        .unwrap_or(1200);
    let out = Path::new(&dir);
    fs::create_dir_all(out)?;
    let mut csv = fs::File::create(out.join("wet-work-inventory.csv"))?;
    writeln!(
        csv,
        "stage,canvas_paint_mm3,canvas_solvent_mm3,opened_paint_mm3,brush_paint_mm3,brush_solvent_mm3,cloth_paint_mm3,cloth_solvent_mm3,cloth_evaporation_mm3,dip_input_mm3,net_lift_mm3,paint_error_mm3,solvent_error_mm3,latent_max_error_mm3,property_max_error_mm3,cure_error_mm3"
    )?;
    for (case, two_colors, dried) in [
        ("wet-passage", false, false),
        ("two-wet-colors", true, false),
        ("dried-passage", false, true),
    ] {
        let base = fixture(width, two_colors, dried);
        let protected = (base.pixels().to_vec(), base.surface_um().to_vec());
        let mut control = None;
        for working in [false, true] {
            let mode = if working { "rag-3" } else { "rag-2" };
            let mut w = WorkingSurface::new(base.clone()).unwrap();
            let initial = w.inventory();
            let mut b = Held::new(Tool::hog_flat(126.0), 71).with_engine(5);
            let label = format!("{case}-{mode}-initial");
            save(out, &label, &w)?;
            row(&mut csv, &label, &w, &b, None, initial, 0.0)?;
            let views = (w.seen(), w.seen_lit(115.0, 55.0, 1.0));
            if let Some(ref reference) = control {
                assert_eq!(&views, reference);
            } else {
                control = Some(views);
            }
            let g = Gesture::line((200.0, 235.0), (900.0, 235.0))
                .pressure(0.85, 0.85)
                .shake(0.25);
            if working {
                w.work_drag(&mut b, &g, None).unwrap();
            } else {
                w.lay_drag(&mut b, &g, None).unwrap();
            }
            let label = format!("{case}-{mode}-drag");
            save(out, &label, &w)?;
            row(&mut csv, &label, &w, &b, None, initial, 0.0)?;
            if dried {
                assert_eq!(w.brush_inventory(&b), Material::default());
            } else if working {
                assert!(w.brush_inventory(&b).paint_mm3 > 0.01);
            } else {
                assert_eq!(w.brush_inventory(&b), Material::default());
            }
            let g = Gesture::line((620.0, 485.0), (880.0, 485.0))
                .pressure(0.65, 0.65)
                .shake(0.25);
            if working {
                w.work_drag(&mut b, &g, None).unwrap();
            } else {
                w.lay_drag(&mut b, &g, None).unwrap();
            }
            let label = format!("{case}-{mode}-carry");
            save(out, &label, &w)?;
            row(&mut csv, &label, &w, &b, None, initial, 0.0)?;
            assert_eq!(
                protected,
                (
                    w.protected_pixels().to_vec(),
                    w.protected_surface_um().to_vec()
                )
            );
        }
        if !two_colors && !dried {
            for (name, dip) in [("dry-cloth", 0.0), ("wet-cloth", 0.95)] {
                let mut w = WorkingSurface::new(base.clone()).unwrap();
                let initial = w.inventory();
                let mut r = w.rag(126.0, 7);
                if dip > 0.0 {
                    w.dip(&mut r, dip);
                }
                let net = w.wipe(&mut r, &[(180.0, 235.0), (880.0, 235.0)], &[0.6], 41);
                let b = Held::new(Tool::hog_flat(126.0), 71).with_engine(5);
                let label = format!("wet-passage-{name}");
                save(out, &label, &w)?;
                row(&mut csv, &label, &w, &b, Some(&r), initial, net)?;
                assert_eq!(
                    protected,
                    (
                        w.protected_pixels().to_vec(),
                        w.protected_surface_um().to_vec()
                    )
                );
            }
        }
    }
    Ok(())
}
