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
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let exists = path.try_exists().map_err(|e| e.to_string())?;
    let key = chunk.to_string();
    if exists && recover(dir)?.lines().skip(1).any(|row| row.split('\t').next() == Some(key.as_str())) {
        return Ok(());
    }
    let pending = dir.join(format!("{chunk:06}.index-pending"));
    if !pending.try_exists().map_err(|e| e.to_string())? {
        let painted = c.has_drawing() || c.wet_total() > 0.0 || ground_signature(c) != ground;
        atomic_write(&pending, format!("{chunk}\t{}\t{}\n", c.tally().secs, u8::from(painted)).as_bytes())?;
    }
    if exists {
        recover(dir)?;
        return Ok(());
    }
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
    recover(dir)?;
    eprintln!("timelapse: chunk {chunk}, {:.3}s, {} bytes, {}x{}", start.elapsed().as_secs_f64(), bytes.len(), width, height);
    Ok(())
}

fn recover(dir: &Path) -> Result<String, String> {
    let index = dir.join("index.tsv");
    let mut text = match std::fs::read_to_string(&index) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => "chunk\thand_s\tpainted\n".into(),
        Err(e) => return Err(e.to_string()),
    };
    let mut pending = Vec::new();
    for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        let Some(chunk) = path.file_name().and_then(|name| name.to_str())
            .and_then(|name| name.strip_suffix(".index-pending"))
            .and_then(|name| name.parse::<usize>().ok()) else { continue };
        if dir.join(format!("{chunk:06}.jpg")).try_exists().map_err(|e| e.to_string())? {
            pending.push((chunk, path));
        }
    }
    pending.sort_by_key(|(chunk, _)| *chunk);
    let previous = text.clone();
    for (chunk, path) in &pending {
        let row = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let key = chunk.to_string();
        let rows: Vec<_> = text.lines().skip(1).filter(|line| line.split('\t').next() == Some(key.as_str())).collect();
        if rows.len() == 1 && rows[0] == row.trim_end() { continue; }
        text = text.lines().filter(|line| line.split('\t').next() != Some(key.as_str()))
            .map(|line| format!("{line}\n")).collect();
        text.push_str(&row);
    }
    if text != previous {
        let mut rows: Vec<_> = text.lines().collect();
        rows.sort_by_key(|row| row.split('\t').next().and_then(|key| key.parse::<usize>().ok()).unwrap_or(0));
        text = rows.into_iter().map(|row| format!("{row}\n")).collect();
        atomic_write(&index, text.as_bytes())?;
    }
    for (_, path) in pending {
        std::fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    Ok(text)
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
    fn index_write_failure_keeps_metadata_for_the_next_frame_and_upserts_once() {
        let dir = std::env::temp_dir().join(format!("live-frame-index-{}", std::process::id()));
        let mut canvas = Canvas::new(16, 0.2, [0.5; 3]);
        let ground = ground_signature(&canvas);
        write(&canvas, ground, &dir, 1).unwrap();
        let index = dir.join("index.tsv");
        let previous = std::fs::read(&index).unwrap();
        let blocked = index.with_extension(format!("{}.pending", std::process::id()));
        std::fs::create_dir(&blocked).unwrap();
        canvas.tally_mut().secs = 3.0;
        assert!(write(&canvas, ground, &dir, 2).is_err());
        assert_eq!(std::fs::read(&index).unwrap(), previous);
        let path = dir.join("000002.jpg");
        let bytes = std::fs::read(&path).unwrap();
        std::fs::remove_dir(blocked).unwrap();
        canvas.tally_mut().secs = 9.0;
        write(&canvas, ground ^ 1, &dir, 3).unwrap();
        let text = std::fs::read_to_string(&index).unwrap();
        assert_eq!(text, "chunk\thand_s\tpainted\n1\t0\t0\n2\t3\t0\n3\t9\t1\n");
        let pending = dir.join("000002.index-pending");
        atomic_write(&pending, b"2\t3\t0\n").unwrap();
        std::fs::write(&index, format!("{text}2\t99\t1\n")).unwrap();
        write(&canvas, ground ^ 1, &dir, 3).unwrap();
        let repaired = std::fs::read(&index).unwrap();
        assert_eq!(repaired, text.as_bytes());
        atomic_write(&pending, b"2\t3\t0\n").unwrap();
        write(&canvas, ground ^ 1, &dir, 2).unwrap();
        assert_eq!(std::fs::read(&index).unwrap(), repaired);
        assert_eq!(std::fs::read(path).unwrap(), bytes);
        assert!(!pending.exists());
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
