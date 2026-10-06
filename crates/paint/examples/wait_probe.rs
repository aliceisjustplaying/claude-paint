//! Read-only checkpoint benchmark. Never opens an easel session or studio.
use paint::{Canvas, color::linear_to_srgb};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    time::Instant,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        return Err("wait_probe <checkpoint> <minutes> <output-prefix>".into());
    }
    let (mut c, _) = Canvas::read_state(&mut BufReader::new(File::open(&args[0])?))?;
    let minutes: f32 = args[1].parse()?;
    println!(
        "BEFORE clock={} engine={} solvent_total={}",
        c.clock(),
        c.engine(),
        c.solvent_total()
    );
    let t = Instant::now();
    c.wait(minutes);
    println!("WAIT_SECONDS {:.6}", t.elapsed().as_secs_f64());
    println!(
        "AFTER clock={} solvent_total={}",
        c.clock(),
        c.solvent_total()
    );
    let frame = c.window();
    let bytes: Vec<u8> = c
        .seen()
        .iter()
        .flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8))
        .collect();
    image::save_buffer(
        format!("{}.png", args[2]),
        &bytes,
        frame.w as u32,
        frame.h as u32,
        image::ColorType::Rgb8,
    )?;
    c.write_state(
        &mut BufWriter::new(File::create(format!("{}.ckpt", args[2]))?),
        "",
    )?;
    Ok(())
}
