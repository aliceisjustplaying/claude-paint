use paint::{
    COAT_UM, Canvas, Gesture, Held, Linen, Mask, Paint, Rgb, Tool,
    color::{hex, linear_to_srgb, luminance},
    rag::Rag,
    working_layer::WorkingSurface,
};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

#[derive(Clone)]
enum Surface {
    Homogeneous(Canvas),
    Layered(WorkingSurface),
}

impl Surface {
    fn rag(&self) -> Rag {
        match self {
            Self::Homogeneous(c) => Rag::new(126.0, 7).with_transport(c),
            Self::Layered(w) => w.rag(126.0, 7),
        }
    }
    fn lay(&mut self, b: &mut Held, g: &Gesture, m: &Mask) {
        match self {
            Self::Homogeneous(c) => c.drag(b, g, Some(m)),
            Self::Layered(w) => w.lay_drag(b, g, Some(m)).unwrap(),
        }
    }
    fn dip(&mut self, r: &mut Rag, d: f32) {
        match self {
            Self::Homogeneous(c) => r.dip(d, c.now_min(), c.tally_mut()),
            Self::Layered(w) => w.dip(r, d),
        }
    }
    fn wipe(&mut self, r: &mut Rag, p: &[(f32, f32)], pressure: f32) -> f64 {
        match self {
            Self::Homogeneous(c) => c.rag_wipe(r, p, &[pressure], 41),
            Self::Layered(w) => w.wipe(r, p, &[pressure], 41),
        }
    }
    fn refold(&mut self, r: &mut Rag) {
        match self {
            Self::Homogeneous(c) => r.refold_at(c.now_min(), c.tally_mut()),
            Self::Layered(w) => w.refold(r),
        }
    }
    fn inventory(&self) -> (f64, f64, f64, f64) {
        match self {
            Self::Homogeneous(c) => {
                let unit = (c.mm_per_unit() as f64).powi(2) * COAT_UM as f64 / 1000.0;
                (
                    c.wet_total() * unit,
                    c.solvent_total() * unit,
                    c.wet_total() * unit,
                    0.0,
                )
            }
            Self::Layered(w) => {
                let m = w.inventory();
                (
                    m.paint_mm3,
                    m.solvent_mm3,
                    w.working_paint_mm3(),
                    w.opened_paint_mm3(),
                )
            }
        }
    }
    fn pixels(&self) -> (Vec<Rgb>, Vec<Rgb>) {
        match self {
            Self::Homogeneous(c) => (c.seen(), c.seen_lit(115.0, 55.0, 1.0)),
            Self::Layered(w) => (w.seen(), w.seen_lit(115.0, 55.0, 1.0)),
        }
    }
    fn protected(&self) -> (Vec<Rgb>, Vec<f32>) {
        match self {
            Self::Homogeneous(c) => (c.pixels().to_vec(), c.surface_um().to_vec()),
            Self::Layered(w) => (
                w.protected_pixels().to_vec(),
                w.protected_surface_um().to_vec(),
            ),
        }
    }
}

fn fixture() -> Canvas {
    let mut c = Canvas::new_window(96, 1.0, hex("#c8bba5"), None)
        .with_engine(5)
        .with_size_mm(100.0)
        .with_linen(Linen::fine(23));
    let m = Mask::from_fn(c.frame(), |x, y| {
        if (100.0..900.0).contains(&x) && (250.0..450.0).contains(&y) {
            1.0
        } else {
            0.0
        }
    });
    let mut b = Held::new(Tool::hog_flat(130.0), 19).with_engine(5);
    b.load(Paint::body(hex("#252c36")).with_stiff(0.7), 0.7);
    for y in [310.0, 350.0, 390.0] {
        c.drag(
            &mut b,
            &Gesture::line((130.0, y), (870.0, y))
                .pressure(0.8, 0.8)
                .shake(0.0),
            Some(&m),
        );
    }
    c
}

fn save(out: &Path, label: &str, s: &Surface) -> io::Result<()> {
    let (a, b) = s.pixels();
    for (view, pixels) in [("color", a), ("gallery", b)] {
        let bytes: Vec<u8> = pixels
            .into_iter()
            .flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8))
            .collect();
        image::save_buffer(
            out.join(format!("{label}-{view}.png")),
            &bytes,
            96,
            96,
            image::ColorType::Rgb8,
        )
        .map_err(io::Error::other)?;
    }
    Ok(())
}

fn row(
    file: &mut fs::File,
    label: &str,
    s: &Surface,
    r: &Rag,
    net: f64,
    expected: (f64, f64),
) -> io::Result<()> {
    let (paint, solvent, working, opened) = s.inventory();
    let c = r.transport.as_ref().unwrap();
    let m = c.inventory();
    let ep = paint + m.paint_mm3 + c.waste.paint_mm3 - expected.0;
    let es = solvent + m.solvent_mm3 + c.waste.solvent_mm3 + c.evaporated_solvent_mm3
        - c.solvent_input_mm3
        - expected.1;
    assert!(ep.abs() < 1e-4 * expected.0.max(1.0), "{label} paint {ep}");
    assert!(
        es.abs() < 1e-4 * expected.1.max(1.0),
        "{label} solvent {es}"
    );
    let pixels = s.pixels().0;
    let mut mean = 0.0;
    let mut n = 0;
    for y in 29..40 {
        for x in 50..81 {
            mean += luminance(pixels[y * 96 + x]) as f64;
            n += 1;
        }
    }
    writeln!(
        file,
        "{label},{paint:.9},{solvent:.9},{working:.9},{opened:.9},{:.9},{:.9},{:.9},{net:.9},{ep:.9},{es:.9},{:.9}",
        m.paint_mm3,
        c.mobile_mm3(),
        c.evaporated_solvent_mm3,
        mean / n as f64
    )
}

fn main() -> io::Result<()> {
    let directory = std::env::args()
        .nth(1)
        .expect("rag_working_layer_probe <output-dir>");
    let out = Path::new(&directory);
    fs::create_dir_all(out)?;
    let mut csv = fs::File::create(out.join("layer-inventory.csv"))?;
    writeln!(
        csv,
        "stage,canvas_paint_mm3,canvas_solvent_mm3,working_paint_mm3,opened_paint_mm3,cloth_paint_mm3,cloth_mobile_mm3,cloth_evaporation_mm3,net_lift_mm3,paint_error_mm3,solvent_error_mm3,pale_mean_Y"
    )?;
    let mut budget = fs::File::create(out.join("initial-budgets.csv"))?;
    writeln!(
        budget,
        "case,mode,dark_wet_mm3,pale_brush_input_mm3,pale_brush_left_mm3,canvas_after_lay_mm3,total_closed_wet_and_brush_mm3"
    )?;
    for protected in [false, true] {
        let case = if protected { "protected" } else { "open-dark" };
        let mut base = fixture();
        if protected {
            base.dry();
        }
        let mask = Mask::from_fn(base.frame(), |x, y| {
            if (100.0..900.0).contains(&x) && (250.0..450.0).contains(&y) {
                1.0
            } else {
                0.0
            }
        });
        let unit = (base.mm_per_unit() as f64).powi(2) * COAT_UM as f64 / 1000.0;
        let dark = base.wet_total() * unit;
        let mut protected_control = None;
        for layered in [false, true] {
            let mode = if layered { "layered" } else { "homogeneous" };
            let prefix = format!("{case}-{mode}");
            let mut s = if layered {
                Surface::Layered(WorkingSurface::new(base.clone()).unwrap())
            } else {
                Surface::Homogeneous(base.clone())
            };
            let mut b = Held::new(Tool::hog_flat(130.0), 71).with_engine(5);
            b.load(Paint::body(hex("#dacfa9")).with_stiff(0.7), 0.7);
            let input = b.carried().0 * unit;
            let passes: &[f32] = if protected {
                &[350.0]
            } else {
                &[310.0, 350.0, 390.0]
            };
            for &y in passes {
                s.lay(
                    &mut b,
                    &Gesture::line((130.0, y), (870.0, y))
                        .pressure(0.2, 0.2)
                        .shake(0.0),
                    &mask,
                );
            }
            if protected {
                let views = s.pixels();
                if let Some(control) = &protected_control {
                    assert_eq!(&views, control);
                } else {
                    protected_control = Some(views);
                }
            }
            let left = b.carried().0 * unit;
            let expected = s.inventory();
            assert!((dark + input - expected.0 - left).abs() < 1e-3);
            writeln!(
                budget,
                "{case},{mode},{dark:.9},{input:.9},{left:.9},{:.9},{:.9}",
                expected.0,
                expected.0 + left
            )?;
            let r = s.rag();
            row(
                &mut csv,
                &format!("{prefix}-initial"),
                &s,
                &r,
                0.0,
                (expected.0, expected.1),
            )?;
            save(out, &format!("{prefix}-initial"), &s)?;
            for (stage, pressure, dip, path) in [
                ("light", 0.15, 0.0, vec![(150.0, 350.0), (850.0, 350.0)]),
                ("firm", 0.6, 0.0, vec![(150.0, 350.0), (850.0, 350.0)]),
                (
                    "scrub",
                    0.95,
                    0.0,
                    vec![
                        (150.0, 350.0),
                        (850.0, 350.0),
                        (150.0, 350.0),
                        (850.0, 350.0),
                    ],
                ),
                ("wet", 0.6, 0.95, vec![(150.0, 350.0), (850.0, 350.0)]),
                ("blot", 0.8, 0.0, vec![(750.0, 350.0)]),
            ] {
                let mut c = s.clone();
                let mut r = c.rag();
                if dip > 0.0 {
                    c.dip(&mut r, dip);
                }
                let fixed = c.protected();
                let net = c.wipe(&mut r, &path, pressure);
                assert_eq!(fixed, c.protected());
                let label = format!("{prefix}-{stage}");
                row(&mut csv, &label, &c, &r, net, (expected.0, expected.1))?;
                save(out, &label, &c)?;
                if stage == "firm" {
                    for folds in [0, 1, 12] {
                        let mut lane = c.clone();
                        let mut face = r;
                        for _ in 0..folds {
                            lane.refold(&mut face);
                        }
                        let carry = lane.wipe(&mut face, &[(650.0, 725.0), (850.0, 725.0)], 0.5);
                        let label = format!("{prefix}-carry-folds-{folds}");
                        row(
                            &mut csv,
                            &label,
                            &lane,
                            &face,
                            carry,
                            (expected.0, expected.1),
                        )?;
                        save(out, &label, &lane)?;
                    }
                }
            }
        }
    }
    Ok(())
}
