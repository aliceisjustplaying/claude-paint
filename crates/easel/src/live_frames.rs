use paint::Canvas;
use std::io::Write;
use std::path::Path;

pub fn ground_signature(c: &Canvas) -> u64 {
    c.pixels().iter().flatten().chain(c.surface_um()).fold(0xcbf29ce484222325, |h, v| {
        (h ^ u64::from(v.to_bits())).wrapping_mul(0x100000001b3)
    })
}

pub fn write(c: &Canvas, ground: u64, dir: &Path, chunk: usize) -> Result<(), String> {
    let path = dir.join(format!("{chunk:06}.jpg"));
    if path.try_exists().map_err(|e| e.to_string())? {
        return Ok(());
    }
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let start = std::time::Instant::now();
    let f = c.window();
    let pixels = c.seen_lit(crate::GALLERY_LIGHT.0, crate::GALLERY_LIGHT.1, 1.0);
    let image = image::RgbImage::from_raw(f.w as u32, f.h as u32, crate::output::rgb8(&pixels, f.w)).ok_or("frame: bad buffer")?;
    let scale = 1920.0 / f.w.max(f.h) as f64;
    let width = (f.w as f64 * scale).round().max(1.0) as u32;
    let height = (f.h as f64 * scale).round().max(1.0) as u32;
    let image = crate::output::thumbnail(&image, width, height);
    let mut bytes = Vec::new();
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut bytes, 90).encode_image(&image).map_err(|e| e.to_string())?;
    atomic_write(&path, &bytes)?;
    let painted = c.has_drawing() || c.wet_total() > 0.0 || ground_signature(c) != ground;
    let index = dir.join("index.tsv");
    let mut text = match std::fs::read_to_string(&index) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => "chunk\thand_s\tpainted\n".into(),
        Err(e) => return Err(e.to_string()),
    };
    text.push_str(&format!("{chunk}\t{}\t{}\n", c.tally().secs, u8::from(painted)));
    atomic_write(&index, text.as_bytes())?;
    eprintln!("timelapse: chunk {chunk}, {:.3}s, {} bytes, {}x{}", start.elapsed().as_secs_f64(), bytes.len(), width, height);
    Ok(())
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let pending = path.with_extension(format!("{}.pending", std::process::id()));
    let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(&pending).map_err(|e| e.to_string())?;
    let result = (|| {
        file.write_all(bytes)?;
        file.sync_all()?;
        std::fs::rename(&pending, path)?;
        std::fs::File::open(path.parent().unwrap())?.sync_all()
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(pending);
    }
    result.map_err(|e: std::io::Error| format!("{}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portrait_frames_fit_the_long_side_and_existing_frames_are_untouched() {
        let dir = std::env::temp_dir().join(format!("portrait-frame-{}", std::process::id()));
        let canvas = Canvas::new(16, 0.2, [0.5; 3]);
        let ground = ground_signature(&canvas);
        write(&canvas, ground, &dir, 7).unwrap();
        let path = dir.join("000007.jpg");
        let image = image::open(&path).unwrap();
        assert_eq!((image.width(), image.height()), (384, 1920));
        let index = std::fs::read(dir.join("index.tsv")).unwrap();
        std::fs::write(&path, b"existing frame").unwrap();
        write(&canvas, ground, &dir, 7).unwrap();
        assert_eq!(std::fs::read(path).unwrap(), b"existing frame");
        assert_eq!(std::fs::read(dir.join("index.tsv")).unwrap(), index);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn atomic_frame_publication_keeps_the_previous_file_on_failure() {
        let dir = std::env::temp_dir().join(format!("live-frame-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("000001.jpg");
        atomic_write(&path, b"previous").unwrap();
        let pending = path.with_extension(format!("{}.pending", std::process::id()));
        std::fs::create_dir(&pending).unwrap();
        assert!(atomic_write(&path, b"incomplete").is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"previous");
        std::fs::remove_dir(&pending).unwrap();
        atomic_write(&path, b"complete").unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"complete");
        assert!(!pending.exists());
        let blocked = dir.join("blocked.jpg");
        std::fs::create_dir(&blocked).unwrap();
        assert!(atomic_write(&blocked, b"failed rename").is_err());
        assert!(!blocked.with_extension(format!("{}.pending", std::process::id())).exists());
        std::fs::remove_dir_all(dir).unwrap();
    }
}
