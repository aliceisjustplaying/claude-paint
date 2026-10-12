use std::{fs, io::Write, path::Path};
use watercolor_pilot::{Watercolor, PALETTE, PIGMENTS};

const SCALE: usize = 4;
const W: usize = 180 * SCALE;
const H: usize = 112 * SCALE;

fn shape(x: f64, y: f64, cx: f64, cy: f64, rx: f64, ry: f64) -> bool {
    let u = (x - cx) / rx;
    let v = (y - cy) / ry;
    let theta = v.atan2(u);
    let radius = 1.0 + 0.028 * (theta * 7.0 + cx).sin() + 0.018 * (theta * 13.0 + cy).cos();
    u * u + v * v < radius * radius
}

fn wash(
    s: &mut Watercolor,
    center: (f64, f64),
    radius: (f64, f64),
    water: f64,
    pigment: [f64; PIGMENTS],
    grade: bool,
) {
    for y in 0..H {
        for x in 0..W {
            let mut hits = 0;
            for sy in 0..4 {
                for sx in 0..4 {
                    let px = (x as f64 + (sx as f64 + 0.5) / 4.0) / SCALE as f64;
                    let py = (y as f64 + (sy as f64 + 0.5) / 4.0) / SCALE as f64;
                    let tooth = s.paper.height[y * W + x] - 0.5;
                    hits += usize::from(shape(
                        px,
                        py,
                        center.0,
                        center.1,
                        radius.0 + tooth * 2.2,
                        radius.1 + tooth * 2.2,
                    ));
                }
            }
            if hits > 0 {
                let coverage = hits as f64 / 16.0;
                let factor = if grade {
                    (1.0 - (y as f64 / SCALE as f64 - 15.0) / 97.0).clamp(0.08, 1.0)
                } else {
                    1.0
                };
                s.add_covered(
                    y * W + x,
                    water * coverage,
                    pigment.map(|v| v * factor * coverage),
                    coverage,
                );
            }
        }
    }
}

fn ppm(path: &Path, s: &Watercolor, raking: bool) {
    let mut f = fs::File::create(path).unwrap();
    write!(f, "P6\n{} {}\n255\n", W, H).unwrap();
    f.write_all(&s.rgb(raking)).unwrap();
}

fn save(out: &Path, name: &str, s: &Watercolor) {
    ppm(&out.join(format!("{name}-gallery.ppm")), s, false);
    ppm(&out.join(format!("{name}-raking.ppm")), s, true);
}

fn row(csv: &mut String, name: &str, s: &Watercolor) {
    let b = s.balance();
    csv.push_str(&format!(
        "{name},water,{:.12},{:.12},{:.12},0,{:.12},{:.12},{:.3e}\n",
        s.ledger.water_added,
        b.surface_water,
        b.capillary_water,
        s.ledger.water_evaporated,
        s.ledger.water_lifted,
        b.water_error
    ));
    for (p, pigment) in PALETTE.iter().enumerate() {
        csv.push_str(&format!(
            "{name},{},{:.12},{:.12},{:.12},{:.12},0,{:.12},{:.3e}\n",
            pigment.name,
            s.ledger.pigment_added[p],
            b.suspended[p],
            b.deposited[p],
            b.bound[p],
            s.ledger.pigment_lifted[p],
            b.pigment_error[p]
        ));
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let dir = args.get(1).expect("output directory");
    let only = args.get(2).map(String::as_str).unwrap_or("all");
    let enabled = |name| only == "all" || only == name;
    let out = Path::new(&dir);
    fs::create_dir_all(out).unwrap();
    let mut csv = String::from("probe,reservoir,input,surface_or_suspended,capillary_or_deposited,bound,evaporated,lifted,error\n");
    if enabled("flat") {
        let mut s = Watercolor::new_at(W, H, 71, 0.05);
        wash(
            &mut s,
            (90.0, 56.0),
            (70.0, 39.0),
            0.15,
            [0.004, 0.0, 0.0],
            false,
        );
        save(out, "01-flat-wet", &s);
        s.wait_seconds(170.0);
        save(out, "01-flat", &s);
        row(&mut csv, "01-flat", &s);
    }

    if enabled("bloom") {
        let mut s = Watercolor::new_at(W, H, 71, 0.05);
        wash(
            &mut s,
            (90.0, 56.0),
            (73.0, 40.0),
            0.17,
            [0.0, 0.0, 0.0],
            false,
        );
        s.wait_seconds(0.5);
        wash(
            &mut s,
            (84.0, 55.0),
            (66.0, 34.0),
            0.015,
            [0.006, 0.0, 0.001],
            false,
        );
        s.wait_seconds(8.0);
        for y in 0..H {
            for x in 0..W {
                let dx = (x as f64 / SCALE as f64 - 102.0) * 0.2;
                let dy = (y as f64 / SCALE as f64 - 58.0) * 0.2;
                let water = 0.95 * (-(dx * dx + dy * dy) / (2.0 * 0.75_f64.powi(2))).exp();
                if water > 0.0001 {
                    s.add(y * W + x, water, [0.0; PIGMENTS]);
                }
            }
        }
        let mut movie = s.clone();
        let frames = out.join("slow-bloom");
        fs::create_dir_all(&frames).unwrap();
        save(&frames, "frame-000", &movie);
        let mut frame = 0;
        movie.wait_ns_observed(8_000_000_000, 1, |state| {
            frame += 1;
            save(&frames, &format!("frame-{frame:03}"), state);
        });
        let mut last = 0.0;
        for (i, time) in [0.0, 0.25, 0.5, 1.0, 2.0, 4.0, 8.0].iter().enumerate() {
            s.wait_seconds(time - last);
            save(out, &format!("bloom-{i}"), &s);
            row(&mut csv, &format!("bloom-{time:.2}s"), &s);
            last = *time;
        }
        assert_eq!(movie, s);
        s.wait_seconds(170.0);
        save(out, "02-bloom", &s);
        row(&mut csv, "02-bloom", &s);
    }

    if enabled("graded") {
        let mut s = Watercolor::new_at(W, H, 71, 0.05);
        wash(
            &mut s,
            (90.0, 56.0),
            (72.0, 43.0),
            0.14,
            [0.0, 0.014, 0.0],
            true,
        );
        s.wait_seconds(170.0);
        save(out, "03-graded", &s);
        row(&mut csv, "03-graded", &s);
    }

    if enabled("granulation") {
        let mut s = Watercolor::new_at(W, H, 71, 0.05);
        wash(
            &mut s,
            (48.0, 56.0),
            (35.0, 40.0),
            0.15,
            [0.007, 0.0, 0.0],
            false,
        );
        wash(
            &mut s,
            (133.0, 56.0),
            (35.0, 40.0),
            0.15,
            [0.0, 0.01, 0.0],
            false,
        );
        s.wait_seconds(170.0);
        save(out, "04-granulation-staining", &s);
        row(&mut csv, "04-granulation-staining", &s);
    }

    if enabled("dry-brush") {
        let mut s = Watercolor::new_at(W, H, 71, 0.05);
        for y in 25 * SCALE..86 * SCALE {
            for x in 16 * SCALE..165 * SCALE {
                let px = x as f64 / SCALE as f64;
                let py = y as f64 / SCALE as f64;
                let center = 55.0 + 8.0 * (px / 41.0).sin();
                let edge = ((py - center).abs() / 18.0).clamp(0.0, 1.0);
                let release = 0.65 + 0.35 * (px / 17.0 + py / 7.0).sin();
                if edge < 0.94 {
                    s.dry_contact_directional(
                        y * W + x,
                        [0.0, 0.0, 0.032 * release * (1.0 - edge * edge)],
                        0.24,
                        [1.0, 8.0 / 41.0 * (px / 41.0).cos()],
                    );
                }
            }
        }
        save(out, "05-dry-brush", &s);
        row(&mut csv, "05-dry-brush", &s);
    }

    if enabled("glaze") {
        let mut s = Watercolor::new_at(W, H, 71, 0.05);
        wash(
            &mut s,
            (65.0, 54.0),
            (47.0, 37.0),
            0.14,
            [0.0, 0.0, 0.009],
            false,
        );
        s.wait_seconds(170.0);
        s.finish_glaze().unwrap();
        save(out, "06-glaze-underlayer", &s);
        wash(
            &mut s,
            (113.0, 58.0),
            (46.0, 37.0),
            0.12,
            [0.010, 0.0, 0.0],
            false,
        );
        s.wait_seconds(170.0);
        save(out, "06-glaze", &s);
        row(&mut csv, "06-glaze", &s);
    }

    if enabled("lifting") {
        let mut s = Watercolor::new_at(W, H, 71, 0.05);
        wash(
            &mut s,
            (48.0, 56.0),
            (35.0, 40.0),
            0.14,
            [0.007, 0.0, 0.0],
            false,
        );
        wash(
            &mut s,
            (133.0, 56.0),
            (35.0, 40.0),
            0.14,
            [0.0, 0.009, 0.0],
            false,
        );
        s.wait_seconds(170.0);
        s.finish_glaze().unwrap();
        save(out, "07-lift-before", &s);
        for y in 43 * SCALE..68 * SCALE {
            for x in 17 * SCALE..164 * SCALE {
                let dy = ((y as f64 / SCALE as f64 - 55.0) / 12.0).abs();
                if dy < 1.0 {
                    s.add(y * W + x, 0.055 * (1.0 - dy * dy), [0.0; PIGMENTS]);
                }
            }
        }
        s.wait_seconds(1.0);
        for y in 43 * SCALE..68 * SCALE {
            for x in 17 * SCALE..164 * SCALE {
                let dy = ((y as f64 / SCALE as f64 - 55.0) / 12.0).abs();
                if dy < 1.0 {
                    s.lift(y * W + x, 0.95 * (1.0 - dy * dy));
                }
            }
        }
        s.wait_seconds(170.0);
        save(out, "07-lifting", &s);
        row(&mut csv, "07-lifting", &s);
    }
    fs::write(out.join("conservation.csv"), csv).unwrap();
    println!("Selected probes ({only}) rendered to {}", out.display());
}
