use paint::{Canvas, Gesture, Held, Paint, Tool, Touch, color::{hex, linear_to_srgb}, surface::Linen};
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let output = std::env::args().nth(1).ok_or("output directory required")?;
    let output = Path::new(&output);
    std::fs::create_dir_all(output)?;
    println!("kind,load,pressure,initial,deposit,remaining,mass_error,max_um");
    for (kind, tool) in [("flat", Tool::hog_flat(30.0)), ("filbert", Tool::filbert(30.0)), ("round", Tool::round_sable(30.0))] {
        for load in [1.0, 0.5, 0.08] {
            for p in [0.3, 0.85] {
                let mut c = Canvas::new(960, 3.0, hex("#bcb3a2")).with_size_mm(240.0).with_engine(5).with_linen(Linen { warp_per_cm: 18.0, weft_per_cm: 16.0, ..Linen::fine(1906) });
                c.prime(hex("#bcb3a2"), 0.92, 120.0, 0.9, 0.3, 1906);
                let mut b = Held::new(tool.clone(), 73);
                b.load(Paint::body(hex("#b56b32")), load);
                let initial = b.carried().0;
                c.drag(&mut b, &Gesture::line((70.0, 110.0), (900.0, 110.0)).pressure(p, p * 0.6).ramps(0.07, 0.14).shake(0.0), None);
                c.touch(&mut b, &Touch::at(100.0, 240.0).pressure(p), None);
                let total = c.wet_total();
                let rem = b.carried().0;
                let max = (0..320).flat_map(|y| (0..960).map(move |x| (x, y))).map(|(x, y)| c.wet_um(x as f32 / 0.96, y as f32 / 0.96)).fold(0.0, f32::max);
                println!("{kind},{load},{p},{initial},{total},{rem},{},{max}", (total + rem - initial) / initial);
                for (name, pixels) in [("flat", c.seen()), ("gallery", c.seen_lit(115.0, 55.0, 1.0))] {
                    let buf: Vec<u8> = pixels.iter().flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8)).collect();
                    image::save_buffer(output.join(format!("{kind}-{load}-{p}-{name}.png")), &buf, 960, 320, image::ColorType::Rgb8)?;
                }
            }
        }
    }
    let mut c = Canvas::new(960, 3.0, hex("#bcb3a2")).with_size_mm(240.0).with_engine(5).with_linen(Linen::fine(1906));
    c.prime(hex("#bcb3a2"), 0.92, 120.0, 0.9, 0.3, 1906);
    let mut b = Held::new(Tool::round_sable(30.0), 73);
    b.load(Paint::body(hex("#456579")).with_stiff(0.15).with_thinner(0.65), 1.0);
    c.drag(&mut b, &Gesture::line((70.0, 110.0), (900.0, 110.0)).pressure(0.7, 0.7).shake(0.0), None);
    for (name, pixels) in [("flat", c.seen()), ("gallery", c.seen_lit(115.0, 55.0, 1.0))] {
        let buf: Vec<u8> = pixels.iter().flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8)).collect();
        image::save_buffer(output.join(format!("soft-thin-{name}.png")), &buf, 960, 320, image::ColorType::Rgb8)?;
    }
    Ok(())
}
