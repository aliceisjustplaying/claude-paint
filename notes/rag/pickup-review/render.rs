// Same geometry, brush handling and seeds as the release wipe comparison.
// Changes: 2400px canvas width and dark brown #3f2b22 paint.
// Writes visible wet paint, without baking the canvas or changing its state.
use paint::{Canvas, Crop, Handling, Linen, Mask, Tool, hex};
use paint::rag::Rag;
use std::path::Path;

fn save(c: &Canvas, path: &Path) {
    let f = c.window();
    let pixels = c.seen().iter().flat_map(|p| p.map(|x|
        (paint::color::linear_to_srgb(x).clamp(0.0, 1.0) * 255.0).round() as u8
    )).collect();
    image::RgbImage::from_raw(f.w as u32, f.h as u32, pixels).unwrap().save(path).unwrap();
}

fn main() {
    let out = std::env::args().nth(1).expect("output directory");
    let out = Path::new(&out);
    std::fs::create_dir_all(out).unwrap();
    let crop = Crop { units: [260.0, 190.0, 740.0, 490.0], margin: 30.0 };
    let mut c = Canvas::new_window(2400, 1.5, hex("#b08060"), Some(crop))
        .with_engine(3).with_size_mm(440.0)
        .with_linen(Linen { warp_per_cm: 15.0, weft_per_cm: 15.0, ..Linen::fine(3) });
    c.prime(hex("#e4dcc8"), 0.9, 25.0, 0.6, 0.3, 5);
    let ground = c.clone();
    c.set_hand_time(Some(15.0));
    let m = Mask::from_fn(c.frame(), |x, y|
        if (100.0..900.0).contains(&x) && (120.0..550.0).contains(&y) { 1.0 } else { 0.0 });
    let hd = Handling::new(Tool::filbert(22.0)).color(|_, _| hex("#3f2b22"))
        .paint(0.85, 0.75).coverage(1.0).load(0.3).angle(|_, _| 0.0).fill(true);
    c.work(&m, &hd, 11);
    c.clock_hand_min();
    save(&c, &out.join("untouched.png"));
    save(&ground, &out.join("ground.png"));
    let mut csv = String::from("case,pass,region,film_um,film_left_percent,color_left_percent\n");
    measure(&c, &c, &ground, "untouched", 0, &mut csv);
    for damp in [false, true] {
        let mut canvas = c.clone();
        let mut rag = Rag::new(100.0, 7);
        if damp { rag.dip(0.5, canvas.now_min(), canvas.tally_mut()); }
        for pass in 1..=2 {
            canvas.rag_wipe(&mut rag, &[(300.0, 340.0), (700.0, 340.0)], &[0.8], 19);
            let name = format!("{}-{pass}.png", if damp { "damp" } else { "dry" });
            save(&canvas, &out.join(name));
            measure(&canvas, &c, &ground, if damp { "damp" } else { "dry" }, pass, &mut csv);
        }
    }
    std::fs::write(out.join("measurements.csv"), csv).unwrap();
}

// Identical fixed regions for all settings and passes, independent of output.
// Core excludes the stroke starts, ends and lateral edges. Envelope includes
// those edges and some untouched pixels, so it is reported separately.
fn measure(c: &Canvas, before: &Canvas, ground: &Canvas, kind: &str, pass: usize, csv: &mut String) {
    let f = c.window();
    let (seen, initial, bare) = (c.seen(), before.seen(), ground.seen());
    let luma = |p: [f32; 3]| 0.2126*p[0] as f64 + 0.7152*p[1] as f64 + 0.0722*p[2] as f64;
    for (region, bounds) in [("core", [350.0,310.0,650.0,370.0]), ("envelope", [250.0,280.0,750.0,400.0])] {
        let (mut film, mut film0, mut tone, mut tone0, mut n) = (0.0,0.0,0.0,0.0,0);
        for y in 0..f.h { for x in 0..f.w {
            let (u,v) = (f.ux(x), f.uy(y));
            if u < bounds[0] || u >= bounds[2] || v < bounds[1] || v >= bounds[3] { continue; }
            let i = y*f.w+x;
            film += c.wet_um(u,v) as f64;
            film0 += before.wet_um(u,v) as f64;
            tone += luma(bare[i])-luma(seen[i]);
            tone0 += luma(bare[i])-luma(initial[i]);
            n += 1;
        }}
        csv.push_str(&format!("{kind},{pass},{region},{:.6},{:.6},{:.6}\n", film/n as f64, 100.0*film/film0, 100.0*tone/tone0));
    }
}
