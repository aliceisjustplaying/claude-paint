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
    c.set_hand_time(Some(15.0));
    let m = Mask::from_fn(c.frame(), |x, y|
        if (100.0..900.0).contains(&x) && (120.0..550.0).contains(&y) { 1.0 } else { 0.0 });
    let hd = Handling::new(Tool::filbert(22.0)).color(|_, _| hex("#3f2b22"))
        .paint(0.85, 0.75).coverage(1.0).load(0.3).angle(|_, _| 0.0).fill(true);
    c.work(&m, &hd, 11);
    c.clock_hand_min();
    save(&c, &out.join("untouched.png"));
    for damp in [false, true] {
        let mut canvas = c.clone();
        let mut rag = Rag::new(100.0, 7);
        if damp { rag.dip(0.5, canvas.now_min(), canvas.tally_mut()); }
        for pass in 1..=2 {
            canvas.rag_wipe(&mut rag, &[(300.0, 340.0), (700.0, 340.0)], &[0.8], 19);
            let name = format!("{}-{pass}.png", if damp { "damp" } else { "dry" });
            save(&canvas, &out.join(name));
        }
    }
    println!("Rendered identical starting paint at 2400px canvas width; pressure 0.8, width 100, dip 0 / 0.5, same seeds and paths; no refolding between wipes.");
}
