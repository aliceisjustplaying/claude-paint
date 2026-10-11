use paint::color::luminance;
use paint::Canvas;
use std::{fs, io::BufReader};

fn percentile(values: &[f32], fraction: f32) -> f32 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f32::total_cmp);
    sorted[((sorted.len() - 1) as f32 * fraction).round() as usize]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 { return Err("brush_sim_4_state <final canvas> <output prefix>".into()); }
    let (canvas, _) = Canvas::read_state(&mut BufReader::new(fs::File::open(&args[0])?))?;
    let frame = canvas.window();
    assert_eq!((frame.w, frame.h), (1200, 800));
    let color = canvas.seen();
    let gallery = canvas.seen_lit(115.0, 55.0, 1.0);
    let (x0, y0, x1, y1) = (160, 180, 680, 460);
    let mut wet = Vec::new();
    let mut heights = Vec::new();
    let mut colors = Vec::new();
    let mut lights = Vec::new();
    let mut transverse = Vec::new();
    let mut profile = String::from("y_px,color_y_mean,gallery_y_mean,wet_paint_mean_um,total_height_mean_um\n");
    for y in y0..y1 {
        let mut sums = [0.0f64; 4];
        for x in x0..x1 {
            let i = y * frame.w + x;
            let thickness = canvas.wet_um(frame.ux(x), frame.uy(y));
            let height = canvas.surface_um()[i] + thickness + canvas.solvent_um(frame.ux(x), frame.uy(y));
            let cy = luminance(color[i]);
            let gy = luminance(gallery[i]);
            wet.push(thickness);
            heights.push(height);
            colors.push(cy);
            lights.push(gy);
            for (sum, value) in sums.iter_mut().zip([cy, gy, thickness, height]) { *sum += value as f64; }
            if y + 1 < y1 {
                let next = canvas.surface_um()[i + frame.w] + canvas.wet_um(frame.ux(x), frame.uy(y + 1)) + canvas.solvent_um(frame.ux(x), frame.uy(y + 1));
                transverse.push((next - height).abs());
            }
        }
        let means = sums.map(|sum| sum / (x1 - x0) as f64);
        profile.push_str(&format!("{y},{},{},{},{}\n", means[0], means[1], means[2], means[3]));
    }
    let mean = |v: &[f32]| v.iter().map(|x| *x as f64).sum::<f64>() / v.len() as f64;
    fs::write(format!("{}-profile.csv", args[1]), profile)?;
    fs::write(format!("{}-metrics.csv", args[1]), format!("engine,x0,y0,x1,y1,material_total,solvent_total,wet_mean_um,wet_p95_um,height_mean_um,height_p95_um,transverse_height_gradient_mean_um_per_px,transverse_height_gradient_p95_um_per_px,color_y_median,gallery_y_median,gallery_y_p95\n{},{x0},{y0},{x1},{y1},{},{},{},{},{},{},{},{},{},{},{}\n", canvas.engine(), canvas.material_total(), canvas.solvent_total(), mean(&wet), percentile(&wet, 0.95), mean(&heights), percentile(&heights, 0.95), mean(&transverse), percentile(&transverse, 0.95), percentile(&colors, 0.5), percentile(&lights, 0.5), percentile(&lights, 0.95)))?;
    Ok(())
}
