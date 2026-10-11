#[cfg(feature = "brush-sim-5")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use paint::{Canvas, color::{luminance, linear_to_srgb}};
    use std::{fs, io::BufReader};
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !(2..=3).contains(&args.len()) { return Err("brush_sim_5_surface <final canvas> <output.csv> [geometry-only engine]".into()); }
    let (mut c, _) = Canvas::read_state(&mut BufReader::new(fs::File::open(&args[0])?))?;
    if let Some(engine) = args.get(2) { c = c.with_engine(engine.parse()?); }
    let f = c.window();
    let top = c.wet_surface_um();
    let color = c.seen();
    let gallery = c.seen_lit(115.0, 55.0, 1.0);
    let mut csv = String::from("x_px,y_px,dry_height_um,wet_paint_um,solvent_um,visible_surface_um,color_y,gallery_y\n");
    for y in 180..460 {
        for x in 160..680 {
            let i = y * f.w + x;
            csv.push_str(&format!("{x},{y},{},{},{},{},{},{}\n", c.surface_um()[i], c.wet_um(f.ux(x), f.uy(y)), c.solvent_um(f.ux(x), f.uy(y)), top[i], luminance(color[i]), luminance(gallery[i])));
        }
    }
    fs::write(&args[1], csv)?;
    if args.len() == 3 {
        let data = gallery.iter().flat_map(|pixel| pixel.map(|v| (linear_to_srgb(v.clamp(0.0, 1.0)) * 255.0).round() as u8)).collect();
        image::RgbImage::from_raw(f.w as u32, f.h as u32, data).unwrap().save(std::path::Path::new(&args[1]).with_extension("png"))?;
    }
    Ok(())
}

#[cfg(not(feature = "brush-sim-5"))]
fn main() { panic!("build with --features brush-sim-5"); }
