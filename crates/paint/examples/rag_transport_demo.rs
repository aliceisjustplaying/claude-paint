use paint::{
    COAT_UM, Canvas, Gesture, Held, Linen, Mask, Paint, Rgb, Tool,
    color::{hex, linear_to_srgb},
    rag::Rag,
    working_layer::WorkingSurface,
};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

enum Study {
    Baseline(Canvas),
    Candidate(WorkingSurface),
}

impl Study {
    fn rag(&self, seed: u64) -> Rag {
        match self {
            Self::Baseline(_) => Rag::new(126.0, seed),
            Self::Candidate(w) => w.rag(126.0, seed),
        }
    }
    fn dip(&mut self, r: &mut Rag, amount: f32) {
        match self {
            Self::Baseline(c) => r.dip(amount, c.now_min(), c.tally_mut()),
            Self::Candidate(w) => w.dip(r, amount),
        }
    }
    fn wipe(&mut self, r: &mut Rag, points: &[(f32, f32)], pressure: f32, seed: u64) -> f64 {
        match self {
            Self::Baseline(c) => c.rag_wipe(r, points, &[pressure], seed),
            Self::Candidate(w) => w.wipe(r, points, &[pressure], seed),
        }
    }
    fn age(&self, r: &mut Rag) {
        match self {
            Self::Baseline(c) => r.evaporate(c.now_min()),
            Self::Candidate(w) => w.age_rag(r),
        }
    }
    fn images(&self) -> (Vec<Rgb>, Vec<Rgb>) {
        match self {
            Self::Baseline(c) => (c.seen(), c.seen_lit(115.0, 55.0, 1.0)),
            Self::Candidate(w) => (w.seen(), w.seen_lit(115.0, 55.0, 1.0)),
        }
    }
    fn unit(&self) -> f64 {
        let mm = match self {
            Self::Baseline(c) => c.mm_per_unit(),
            Self::Candidate(w) => w.mm_per_unit(),
        };
        (mm as f64).powi(2) * COAT_UM as f64 / 1000.0
    }
    fn canvas_inventory(&self) -> (f64, f64) {
        match self {
            Self::Baseline(c) => (c.wet_total() * self.unit(), c.solvent_total() * self.unit()),
            Self::Candidate(w) => {
                let m = w.inventory();
                (m.paint_mm3, m.solvent_mm3)
            }
        }
    }
}

fn tone(width: usize) -> Canvas {
    let mut c = Canvas::new_window(width, 1.6, hex("#d3c7af"), None)
        .with_engine(5)
        .with_size_mm(100.0)
        .with_linen(Linen::fine(23));
    let mask = Mask::from_fn(c.frame(), |x, y| {
        if (95.0..905.0).contains(&x) && (90.0..415.0).contains(&y) {
            1.0
        } else {
            0.0
        }
    });
    let mut b = Held::new(Tool::hog_flat(145.0), 19).with_engine(5);
    b.load(
        Paint::body(hex("#765b46"))
            .with_stiff(0.55)
            .with_thinner(0.25),
        0.65,
    );
    for (i, y) in [145.0, 195.0, 245.0, 295.0, 345.0, 395.0]
        .into_iter()
        .enumerate()
    {
        if i % 2 == 0 {
            c.drag(
                &mut b,
                &Gesture::line((120.0, y), (880.0, y))
                    .pressure(0.55, 0.55)
                    .shake(0.2),
                Some(&mask),
            );
        } else {
            c.drag(
                &mut b,
                &Gesture::line((880.0, y), (120.0, y))
                    .pressure(0.55, 0.55)
                    .shake(0.2),
                Some(&mask),
            );
        }
    }
    c
}

fn row(
    file: &mut fs::File,
    mode: &str,
    stage: &str,
    study: &Study,
    rags: &mut [&mut Rag],
    net: f64,
    initial: (f64, f64),
) -> io::Result<()> {
    let (paint, solvent) = study.canvas_inventory();
    let (mut cp, mut cs, mut input, mut evap, mut mobile, mut gross, mut redeposited) =
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    for r in rags {
        study.age(r);
        if let Some(t) = &r.transport {
            let m = t.inventory();
            cp += m.paint_mm3;
            cs += m.solvent_mm3;
            input += t.solvent_input_mm3;
            evap += t.evaporated_solvent_mm3;
            mobile += t.mobile_mm3();
            gross += t.gross_pickup_mm3;
            redeposited += t.redeposited_mm3;
        } else {
            let capacity = (r.width as f64 * 0.1).powi(2) * 0.4;
            cp += r.soaked as f64 * 12.0 * capacity;
            cs += r.solvent_mm3;
            mobile += r.mobile.mm3 as f64;
        }
    }
    let ep = paint + cp - initial.0;
    let es = solvent + cs + evap - input - initial.1;
    assert!(
        ep.abs() < 1e-4 * initial.0.max(1.0),
        "{mode} {stage}: paint {ep}"
    );
    assert!(
        es.abs() < 1e-4 * initial.1.max(1.0),
        "{mode} {stage}: solvent {es}"
    );
    writeln!(
        file,
        "{mode},{stage},{paint:.9},{solvent:.9},{cp:.9},{mobile:.9},{cs:.9},{input:.9},{evap:.9},{gross:.9},{redeposited:.9},{net:.9},{ep:.9},{es:.9}"
    )
}

fn save(out: &Path, label: &str, width: usize, study: &Study) -> io::Result<()> {
    let (color, gallery) = study.images();
    for (view, pixels) in [("color", color), ("gallery", gallery)] {
        let height = pixels.len() / width;
        let bytes: Vec<u8> = pixels
            .into_iter()
            .flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8))
            .collect();
        image::save_buffer(
            out.join(format!("{label}-{view}.png")),
            &bytes,
            width as u32,
            height as u32,
            image::ColorType::Rgb8,
        )
        .map_err(io::Error::other)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let directory = args
        .next()
        .expect("rag_transport_demo <output-dir> [width=1200]");
    let width = args.next().map_or(1200, |s| s.parse::<usize>().unwrap());
    let out = Path::new(&directory);
    fs::create_dir_all(out)?;
    let base = tone(width);
    let unit = (base.mm_per_unit() as f64).powi(2) * COAT_UM as f64 / 1000.0;
    let initial = (base.wet_total() * unit, base.solvent_total() * unit);
    let mut csv = fs::File::create(out.join("inventory.csv"))?;
    writeln!(
        csv,
        "mode,stage,canvas_paint_mm3,canvas_solvent_mm3,cloth_paint_mm3,cloth_mobile_mm3,cloth_solvent_mm3,dip_input_mm3,cloth_evaporation_mm3,gross_pickup_mm3,redeposited_mm3,net_lift_mm3,paint_error_mm3,solvent_error_mm3"
    )?;
    for candidate in [false, true] {
        let name = if candidate { "rag-2" } else { "baseline" };
        let mut study = if candidate {
            Study::Candidate(WorkingSurface::new(base.clone()).unwrap())
        } else {
            Study::Baseline(base.clone())
        };
        let mut cloth = study.rag(7);
        let mut wet = study.rag(9);
        save(out, &format!("{name}-initial"), width, &study)?;
        row(
            &mut csv,
            name,
            "initial",
            &study,
            &mut [&mut cloth, &mut wet],
            0.0,
            initial,
        )?;
        let lift = study.wipe(
            &mut cloth,
            &[(145.0, 215.0), (310.0, 180.0), (580.0, 230.0)],
            0.78,
            41,
        );
        row(
            &mut csv,
            name,
            "wipe-lights",
            &study,
            &mut [&mut cloth, &mut wet],
            lift,
            initial,
        )?;
        let repeat = study.wipe(
            &mut cloth,
            &[(145.0, 245.0), (330.0, 240.0), (580.0, 265.0)],
            0.78,
            45,
        );
        row(
            &mut csv,
            name,
            "repeat-lights",
            &study,
            &mut [&mut cloth, &mut wet],
            repeat,
            initial,
        )?;
        let carry = study.wipe(&mut cloth, &[(650.0, 505.0), (875.0, 555.0)], 0.45, 42);
        row(
            &mut csv,
            name,
            "dirty-carry",
            &study,
            &mut [&mut cloth, &mut wet],
            carry,
            initial,
        )?;
        study.dip(&mut wet, 0.95);
        let flood = study.wipe(&mut wet, &[(685.0, 310.0), (870.0, 345.0)], 0.72, 43);
        row(
            &mut csv,
            name,
            "wet-flood",
            &study,
            &mut [&mut cloth, &mut wet],
            flood,
            initial,
        )?;
        let blot = study.wipe(&mut cloth, &[(330.0, 355.0)], 0.82, 44);
        row(
            &mut csv,
            name,
            "dirty-blot",
            &study,
            &mut [&mut cloth, &mut wet],
            blot,
            initial,
        )?;
        save(out, name, width, &study)?;
    }
    Ok(())
}
