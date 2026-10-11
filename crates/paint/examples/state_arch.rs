use paint::{Canvas, color::Rgb, render_snapshot::RenderSnapshot};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::Path,
};

fn png(path: &Path, pixels: &[Rgb], w: usize, h: usize) -> Result<(), Box<dyn std::error::Error>> {
    let bytes: Vec<u8> = pixels
        .iter()
        .flatten()
        .map(|&v| {
            (paint::color::linear_to_srgb(v) * 255.0)
                .round()
                .clamp(0.0, 255.0) as u8
        })
        .collect();
    image::save_buffer(path, &bytes, w as u32, h as u32, image::ColorType::Rgb8)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 || !["capture", "render"].contains(&args[1].as_str()) {
        return Err(
            "usage: state_arch capture CHECKPOINT SNAPSHOT | render SNAPSHOT OUTPUT_DIR".into(),
        );
    }
    let start = std::time::Instant::now();
    let mut input = BufReader::with_capacity(1 << 20, File::open(&args[2])?);
    if args[1] == "capture" {
        let (canvas, header) = Canvas::read_state(&mut input)?;
        let snapshot = RenderSnapshot::capture(&canvas)?;
        let mut output = BufWriter::with_capacity(1 << 20, File::create(&args[3])?);
        snapshot.write(&mut output)?;
        output.flush()?;
        output.get_ref().sync_all()?;
        let restored = RenderSnapshot::read(&mut BufReader::with_capacity(
            1 << 20,
            File::open(&args[3])?,
        ))?;
        let color = canvas.seen();
        assert_eq!(color.len(), restored.color().len());
        assert!(
            color
                .iter()
                .flatten()
                .zip(restored.color().iter().flatten())
                .all(|(a, b)| a.to_bits() == b.to_bits())
        );
        drop(color);
        for (name, az, el) in [("gallery", 115.0, 55.0), ("relief", 135.0, 25.0)] {
            let expected = canvas.seen_lit(az, el, 1.0);
            let actual = restored.lit(az, el, 1.0);
            assert_eq!(expected.len(), actual.len());
            let changed = expected
                .iter()
                .flatten()
                .zip(actual.iter().flatten())
                .filter(|(a, b)| a.to_bits() != b.to_bits())
                .count();
            println!("{name}: differing f32 channels={changed}");
            if changed != 0 {
                return Err("optical parity failed".into());
            }
        }
        println!("source_header:\n{header}");
        let (width, height) = snapshot.dimensions();
        println!(
            "snapshot={width}x{height} bytes={}",
            std::fs::metadata(&args[3])?.len()
        );
    } else {
        let snapshot = RenderSnapshot::read(&mut input)?;
        let out = Path::new(&args[3]);
        std::fs::create_dir_all(out)?;
        let (width, height) = snapshot.dimensions();
        png(&out.join("color.png"), snapshot.color(), width, height)?;
        for (name, az, el) in [
            ("gallery", 115.0, 55.0),
            ("relief", 135.0, 25.0),
            ("alternate", 20.0, 35.0),
        ] {
            png(
                &out.join(format!("{name}.png")),
                &snapshot.lit(az, el, 1.0),
                width,
                height,
            )?;
        }
        println!("rendered {width}x{height} without canvas, checkpoint or Lua");
    }
    println!("seconds={:.3}", start.elapsed().as_secs_f64());
    Ok(())
}
