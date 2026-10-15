use image::{ImageEncoder, GenericImageView};
use paint::color::linear_to_srgb;
use rayon::prelude::*;
use std::path::Path;
use std::time::Instant;

fn timed<T>(name: &str, f: impl FnOnce() -> T) -> T {
    let t = Instant::now();
    let result = f();
    eprintln!("BENCH,{name},{:.6}", t.elapsed().as_secs_f64());
    result
}

fn thumbnail_parallel(img: &image::RgbImage, width: u32, height: u32) -> image::RgbImage {
    let (mut a, mut b) = (img.height(), height);
    while b != 0 { (a, b) = (b, a % b); }
    let out_rows = height / a * 16;
    let in_rows = img.height() / a * 16;
    let mut out = vec![0; width as usize * height as usize * 3];
    out.par_chunks_mut(width as usize * out_rows as usize * 3).enumerate().for_each(|(i, dest)| {
        let y = i as u32 * in_rows;
        let source = img.view(0, y, img.width(), in_rows.min(img.height() - y));
        let rows = dest.len() as u32 / (width * 3);
        let small = image::imageops::thumbnail(&*source, width, rows);
        dest.copy_from_slice(small.as_raw());
    });
    image::RgbImage::from_raw(width, height, out).unwrap()
}

pub fn run(args: &[String]) -> Result<(), String> {
    let path = args.first().ok_or("render-speed checkpoint output-dir [threads]")?;
    let out = Path::new(args.get(1).ok_or("missing output-dir")?);
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let restored = timed("read_checkpoint", || crate::save::read(Path::new(path)))?;
    let held = restored.session.canvas().ok_or("no canvas")?;
    let canvas: &paint::Canvas = &held;
    let f = canvas.window();
    for repeat in 0..3 {
        let cloned = timed(&format!("clone,{repeat}"), || canvas.clone());
        std::hint::black_box(&cloned);
    }
    let threads = args.get(2).map(String::as_str).unwrap_or("1,2,4");
    let first_threads = threads.split(',').next().unwrap();
    let lut: Vec<u8> = (0..=65535).map(|i| (linear_to_srgb(i as f32 / 65535.0) * 255.0).round().clamp(0.0,255.0) as u8).collect();
    for n in threads.split(',') {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(n.parse().map_err(|_| "bad threads")?).build().map_err(|e| e.to_string())?;
        pool.install(|| -> Result<(), String> {
            for mode in ["color", "gallery", "relief"] {
                for repeat in 0..3 {
                    let label = format!("{n},{mode},{repeat}");
                    let px = timed(&format!("{label},shade"), || match mode {
                        "color" => canvas.seen(),
                        "gallery" => canvas.seen_lit(115.0, 55.0, 1.0),
                        _ => canvas.seen_lit(135.0, 25.0, 1.0),
                    });
                    let convert = |p: &[f32; 3]| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8);
                    let serial: Vec<u8> = timed(&format!("{label},convert_serial"), || px.iter().flat_map(convert).collect());
                    let parallel: Vec<[u8; 3]> = timed(&format!("{label},convert_parallel"), || px.par_iter().map(convert).collect());
                    if serial.as_slice() != parallel.as_flattened() { return Err("conversion differs".into()); }
                    let lookup: Vec<[u8; 3]> = timed(&format!("{label},convert_lut"), || px.par_iter().map(|p| p.map(|v| lut[(v.clamp(0.0, 1.0) * 65535.0).round() as usize])).collect());
                    let max_diff = serial.iter().zip(lookup.as_flattened()).map(|(&a,&b)| a.abs_diff(b)).max().unwrap_or(0);
                    let different = serial.iter().zip(lookup.as_flattened()).filter(|(a,b)| a != b).count();
                    eprintln!("LUT_DIFF,{label},{max_diff},{different},{}", serial.len());
                    if repeat == 0 && n == first_threads {
                        image::save_buffer(out.join(format!("{n}-{mode}-lut.png")), lookup.as_flattened(), f.w as u32, f.h as u32, image::ColorType::Rgb8).map_err(|e| e.to_string())?;
                    }
                    let img = image::RgbImage::from_raw(f.w as u32, f.h as u32, serial).ok_or("bad image")?;
                    for width in [720, 1920, f.w as u32] {
                        let height = ((f.h as f64 * width as f64 / f.w as f64).round() as u32).max(1);
                        let small = timed(&format!("{label},resize_{width}"), || image::imageops::thumbnail(&img, width, height));
                        let striped = timed(&format!("{label},resize_parallel_{width}"), || thumbnail_parallel(&img, width, height));
                        let different = small.as_raw().iter().zip(striped.as_raw()).filter(|(a, b)| a != b).count();
                        eprintln!("RESIZE_DIFF,{label},{width},{different}");
                        for (name, compression, filter) in [
                            ("default", image::codecs::png::CompressionType::Fast, image::codecs::png::FilterType::Adaptive),
                            ("fast_sub", image::codecs::png::CompressionType::Fast, image::codecs::png::FilterType::Sub),
                            ("none", image::codecs::png::CompressionType::Uncompressed, image::codecs::png::FilterType::NoFilter),
                        ] {
                            let mut bytes = Vec::new();
                            timed(&format!("{label},png_{width}_{name}"), || image::codecs::png::PngEncoder::new_with_quality(&mut bytes, compression, filter).write_image(&small, width, height, image::ExtendedColorType::Rgb8)).map_err(|e| e.to_string())?;
                            eprintln!("BYTES,{label},{width},{name},{}", bytes.len());
                            if repeat == 0 && n == first_threads {
                                timed(&format!("{label},write_{width}_{name}"), || std::fs::write(out.join(format!("{n}-{mode}-{width}-{name}.png")), bytes)).map_err(|e| e.to_string())?;
                            }
                        }
                        if repeat == 0 && n == first_threads {
                            striped.save(out.join(format!("{n}-{mode}-{width}-striped.png"))).map_err(|e| e.to_string())?;
                        }
                    }
                }
                timed(&format!("{n},{mode},look"), || crate::look::look(canvas, &crate::look::View { light: match mode { "gallery" => Some((115.0,55.0)), "relief" => Some((135.0,25.0)), _ => None }, ..Default::default() }, &out.join(format!("{n}-{mode}-look.png"))))?;
            }
            Ok(())
        })?;
    }
    Ok(())
}
