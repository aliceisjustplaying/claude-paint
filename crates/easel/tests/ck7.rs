
use std::path::Path;
use std::process::Command;

#[path = "../../paint/tests/support/image_tolerance.rs"]
mod image_tolerance;

const SAVE: &[u8] = include_bytes!("fixtures/ck7/live.ckpt.zst");
const COLOR: &[u8] = include_bytes!("fixtures/ck7/color.png");
const SOURCE: &str = include_str!("fixtures/ck7/painting.lua");

#[test]
fn historical_v7_native_color_is_png_byte_identical_without_replay() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("native-ck7-color");
    std::fs::create_dir_all(&dir).unwrap();
    let compressed = dir.join("live.ckpt.zst");
    let raw = dir.join("live.ckpt");
    let png = dir.join("color.png");
    let source = dir.join("painting.lua");
    std::fs::write(&compressed, SAVE).unwrap();
    std::fs::write(&raw, zstd::stream::decode_all(SAVE).unwrap()).unwrap();
    std::fs::write(&source, SOURCE).unwrap();
    for save in [&compressed, &raw] {
        let result = Command::new(env!("CARGO_BIN_EXE_easel")).env_remove("EASEL_WIDTH")
            .args(["render-checkpoint", save.to_str().unwrap(), png.to_str().unwrap(), "--mode", "color", "--log", source.to_str().unwrap()])
            .env("RAYON_NUM_THREADS", "2")
            .output().unwrap();
        assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
        image_tolerance::assert_close(&image::open(&png).unwrap().to_rgba8(), &image::load_from_memory(COLOR).unwrap().to_rgba8());
    }
    std::fs::remove_file(&source).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_easel")).env_remove("EASEL_WIDTH")
        .args(["render-checkpoint", compressed.to_str().unwrap(), png.to_str().unwrap(), "--mode", "color"])
        .env("RAYON_NUM_THREADS", "2")
        .output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    image_tolerance::assert_close(&image::open(&png).unwrap().to_rgba8(), &image::load_from_memory(COLOR).unwrap().to_rgba8());
}

#[test]
fn v7_material_import_is_lossless_and_upgrades_only_magic_and_engine_word() {
    let raw = zstd::stream::decode_all(SAVE).unwrap();
    assert_eq!(&raw[..8], b"PAINTCK7");
    let (canvas, header) = paint::Canvas::read_state(&mut raw.as_slice()).unwrap();
    assert_eq!(canvas.engine(), 1);
    assert!(canvas.wet_total() > 0.0);
    let mut actual = Vec::new();
    canvas.write_state(&mut actual, &header).unwrap();
    let mut expected = raw;
    expected[..8].copy_from_slice(b"PAINTCK8");
    expected.extend_from_slice(&1u64.to_le_bytes());
    assert_eq!(actual, expected);
}

#[test]
fn v7_rejects_truncation_trailing_engine_words_and_wrong_editions() {
    let raw = zstd::stream::decode_all(SAVE).unwrap();
    assert!(paint::Canvas::read_state(&mut &raw[..raw.len() - 1]).is_err());
    let mut trailing = raw.clone();
    trailing.extend_from_slice(&1u64.to_le_bytes());
    assert!(paint::Canvas::read_state(&mut trailing.as_slice()).is_err());
    let mut wrong_engine = raw.clone();
    let position = wrong_engine.windows(9).position(|s| s == b"engine=1\n").unwrap();
    wrong_engine[position + 7] = b'2';
    assert!(paint::Canvas::read_state(&mut wrong_engine.as_slice()).is_err());
    let mut unknown = raw;
    unknown[7] = b'6';
    assert!(paint::Canvas::read_state(&mut unknown.as_slice()).is_err());
}
