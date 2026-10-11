#[cfg(feature = "brush-sim-5")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use paint::{Canvas, color::luminance};
    use std::{fs, io::BufReader};
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 { return Err("brush_sim_5_surface <final canvas> <output.csv>".into()); }
    let (c, _) = Canvas::read_state(&mut BufReader::new(fs::File::open(&args[0])?))?;
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
    Ok(())
}

#[cfg(not(feature = "brush-sim-5"))]
fn main() { panic!("build with --features brush-sim-5"); }
