
use std::path::{Path, PathBuf};
use std::process::Command;
use image::ImageEncoder;

struct Studio(PathBuf);

impl Studio {
    fn run(&self, args: &[&str]) -> String {
        let output = Command::new(env!("CARGO_BIN_EXE_easel"))
            .args(args).env("EASEL_ROOT", &self.0).env_remove("EASEL_WIDTH")
            .env_remove("EASEL_SESSION").env("RAYON_NUM_THREADS", "2")
            .output().unwrap();
        assert!(output.status.success(), "{args:?}: {}", String::from_utf8_lossy(&output.stderr));
        String::from_utf8(output.stdout).unwrap()
    }

    fn look(&self, name: &str) -> image::RgbImage {
        let output = self.run(&["-s", "reference-decode", "look", "--ref", name, "--size", "100"]);
        image::open(output.split_whitespace().next().unwrap()).unwrap().to_rgb8()
    }
}

impl Drop for Studio {
    fn drop(&mut self) {
        self.run(&["-s", "reference-decode", "close"]);
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn reference_views_decode_webp_and_apply_jpeg_orientation() {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("reference-decode-{}", std::process::id()));
    std::fs::create_dir_all(root.join("notes")).unwrap();
    let studio = Studio(root);
    let pixels = image::RgbImage::from_fn(40, 60, |x, y| image::Rgb([if x < 20 { 220 } else { 20 }, if y < 30 { 200 } else { 30 }, 70]));
    pixels.save(studio.0.join("study.webp")).unwrap();
    pixels.save(studio.0.join("study.png")).unwrap();
    let mut jpeg = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg, 95);
    encoder.set_exif_metadata(vec![b'I', b'I', 42, 0, 8, 0, 0, 0, 1, 0, 0x12, 1, 3, 0, 1, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    encoder.encode_image(&pixels).unwrap();
    std::fs::write(studio.0.join("oriented.jpg"), &jpeg).unwrap();
    image::imageops::rotate90(&image::load_from_memory(&jpeg).unwrap().to_rgb8())
        .save(studio.0.join("normalized.png")).unwrap();
    studio.run(&["open", "reference-decode"]);
    studio.run(&["-s", "reference-decode", "do", "canvas{size=300, aspect=1.5, linen=15, seed=3, ground={{pile={{\"lead white\", 3}}, um=80, apply=\"knife\"}}}"]);
    assert_eq!(studio.look("study.webp"), studio.look("study.png"));
    assert_eq!(studio.look("oriented.jpg"), studio.look("normalized.png"));
}
