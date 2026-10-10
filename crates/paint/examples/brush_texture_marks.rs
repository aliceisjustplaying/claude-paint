use paint::{Canvas, Gesture, Handling, Held, Mask, Paint, Stipple, Tool, color::{hex, linear_to_srgb}, surface::Linen};
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let output = std::env::args().nth(1).ok_or("output directory required")?;
    let output = Path::new(&output);
    std::fs::create_dir_all(output)?;
    let mut c = Canvas::new(960, 1.5, hex("#bcb3a2")).with_size_mm(292.0).with_engine(5).with_linen(Linen { warp_per_cm: 18.0, weft_per_cm: 16.0, ..Linen::fine(1906) });
    c.prime(hex("#bcb3a2"), 0.92, 120.0, 0.9, 0.3, 1906);
    for (k, tool) in [Tool::hog_flat(30.0), Tool::filbert(30.0), Tool::round_sable(30.0)].into_iter().enumerate() {
        for (n, load) in [0.7, 0.08].into_iter().enumerate() {
            let mut b = Held::new(tool.clone(), 73);
            b.load(Paint::body(hex("#b56b32")), load);
            let x = 90.0 + k as f32 * 300.0 + n as f32 * 130.0;
            c.drag(&mut b, &Gesture::line((x, 80.0), (x + 100.0, 85.0)).pressure(0.65, 0.45).ramps(0.1, 0.15).shake(0.0), None);
        }
    }
    let m = Mask::from_fn(c.frame(), |x, y| if (60.0..460.0).contains(&x) && (180.0..390.0).contains(&y) { 1.0 } else { 0.0 });
    let hd = Handling::new(Tool::hog_flat(30.0)).length(50.0, 110.0).coverage(1.7).angle(|_, _| -0.15).color(|_, _| hex("#677862")).paint(0.92, 1.0).dips(4, 0.7, 0.7);
    c.work(&m, &hd, 1906);
    let m = Mask::from_fn(c.frame(), |x, y| if (540.0..940.0).contains(&x) && (180.0..390.0).contains(&y) { 1.0 } else { 0.0 });
    let sp = Stipple::new(Tool::filbert(20.0)).coverage(|_, _| 0.65).color(|_, _| hex("#677862")).paint(0.92, 1.0).pressure(0.4, 0.65).dips(24, 0.3, 0.7).drag(4.0, Some(0.0));
    c.stipple(&m, &sp, 1906);
    let mut under = Held::new(Tool::filbert(85.0), 31);
    under.load(Paint::body(hex("#456579")), 1.0);
    c.drag(&mut under, &Gesture::line((100.0, 530.0), (500.0, 530.0)).pressure(0.85, 0.85), None);
    let mut b = Held::new(Tool::hog_flat(30.0), 73);
    b.load(Paint::body(hex("#b56b32")), 0.3);
    c.drag(&mut b, &Gesture::line((200.0, 515.0), (900.0, 550.0)).pressure(0.65, 0.3).ramps(0.07, 0.14).shake(0.0), None);
    for (name, pixels) in [("flat", c.seen()), ("gallery", c.seen_lit(115.0, 55.0, 1.0))] {
        let buf: Vec<u8> = pixels.iter().flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8)).collect();
        image::save_buffer(output.join(format!("marks-{name}.png")), &buf, 960, 640, image::ColorType::Rgb8)?;
    }
    Ok(())
}
