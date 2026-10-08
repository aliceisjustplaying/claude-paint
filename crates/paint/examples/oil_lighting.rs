//! Small display-only study. No session, replay, drying or reference assets.
use paint::{Canvas, Gesture, Held, Knife, Paint, Tool, color::{Rgb, hex, linear_to_srgb}};
use std::{error::Error, path::Path, time::Instant};

fn png(path: &Path, pixels: &[Rgb], w: usize, h: usize) -> Result<(), Box<dyn Error>> {
    // Identical output conversion for all views: no dither, tone mapping or crop.
    let bytes: Vec<u8> = pixels.iter().flat_map(|p| p.map(|v|
        (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8)).collect();
    image::save_buffer(path, &bytes, w as u32, h as u32, image::ColorType::Rgb8)?;
    Ok(())
}

fn scene(width: usize) -> Canvas {
    let mut c = Canvas::new(width, 1.5, hex("#746b60")).with_size_mm(24.0);
    // Smooth neutral support to isolate paint lighting, engine-5 geometry.
    let mut brush = Held::new(Tool::hog_flat(110.0), 73);
    brush.load(Paint::body(hex("#b56b32")), 1.0);
    c.drag(&mut brush, &Gesture::line((140.0, 145.0), (800.0, 170.0)).pressure(0.75, 0.55), None);
    // Same reservoir, now partly spent. No separate geometry experiment.
    c.drag(&mut brush, &Gesture::line((160.0, 325.0), (800.0, 295.0)).pressure(0.6, 0.35), None);
    let mut knife = Knife::new(130.0);
    knife.load(Paint::body(hex("#436b91")), 1.0);
    c.knife(&mut knife, &[(160.0, 490.0), (760.0, 490.0)], (0.45, 0.7), None, true, 0.12);
    c
}

fn main() -> Result<(), Box<dyn Error>> {
    let output = std::env::args().nth(1).ok_or("oil_lighting <output-directory>")?;
    let output = Path::new(&output);
    std::fs::create_dir_all(output)?;
    for width in [160, 320] {
        let t = Instant::now();
        let c = scene(width);
        let scene_seconds = t.elapsed().as_secs_f64();
        let f = c.window();
        let mut before = Vec::new();
        c.write_state(&mut before, "oil-lighting study")?;
        std::fs::write(output.join(format!("material-{width}.ckpt")), &before)?;
        println!("scene width={width} height={} mm_width=24 px_mm={} engine={} seed=73 clock={} build_scene_seconds={scene_seconds:.6}", f.h, c.px_mm(), c.engine(), c.clock());
        for (name, view) in [
            ("current", Canvas::seen_lit as fn(&Canvas, f32, f32, f32) -> Vec<Rgb>),
            ("candidate", Canvas::seen_lit_body_candidate),
        ] {
            let t = Instant::now();
            let pixels = view(&c, 135.0, 25.0, 0.8);
            let seconds = t.elapsed().as_secs_f64();
            assert!(pixels.iter().flatten().all(|v| v.is_finite() && *v >= 0.0));
            let max = pixels.iter().flatten().copied().fold(0.0f32, f32::max);
            png(&output.join(format!("{name}-{width}.png")), &pixels, f.w, f.h)?;
            let mut after = Vec::new();
            c.write_state(&mut after, "oil-lighting study")?;
            assert_eq!(before, after, "{name} changed material");
            println!("view={name} width={width} render_seconds={seconds:.6} max_linear={max:.6} checkpoint_bytes={} material_equal=true", after.len());
        }
        png(&output.join(format!("flat-{width}.png")), &c.seen(), f.w, f.h)?;
        let mut after = Vec::new();
        c.write_state(&mut after, "oil-lighting study")?;
        assert_eq!(before, after, "flat changed material");
        println!("view=flat width={width} material_equal=true");
    }
    Ok(())
}
