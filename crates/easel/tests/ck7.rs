//! Synthetic fixtures generated at runtime from a sixteen-by-sixteen procedural
//! canvas and literal RGB colors. The current writer's payload is repackaged
//! as an obsolete v7 layout to test rejection, not historical rendering.

use std::path::Path;
use std::process::Command;

fn synthetic_v7() -> Vec<u8> {
    let mut canvas = paint::Canvas::new_window(16, 1.0, [0.3; 3], None).with_engine(1);
    let mut knife = paint::Knife::new(200.0);
    knife.load(paint::Paint::body([0.2, 0.4, 0.7]), 0.5);
    canvas.knife(&mut knife, &[(300.0, 500.0), (700.0, 500.0)], (0.5, 0.5), Some(0.0), true, 0.0);
    assert!(canvas.wet_total() > 0.0);
    let mut current = Vec::new();
    canvas.write_state(&mut current, "engine=1\n").unwrap();
    assert_eq!(&current[..8], b"PAINTC19");
    let header_len = u64::from_le_bytes(current[8..16].try_into().unwrap()) as usize;
    let envelope = 16 + header_len;
    assert_eq!(u64::from_le_bytes(current[envelope..envelope + 8].try_into().unwrap()), paint::mixer::identity());
    assert_eq!(u64::from_le_bytes(current[envelope + 8..envelope + 16].try_into().unwrap()), 8);
    assert_eq!(u64::from_le_bytes(current[envelope + 16..envelope + 24].try_into().unwrap()), 0);
    assert_eq!(u64::from_le_bytes(current[current.len() - 8..].try_into().unwrap()), 1);
    let mut obsolete = current[..envelope].to_vec();
    obsolete[..8].copy_from_slice(b"PAINTCK7");
    obsolete.extend_from_slice(&current[envelope + 24..current.len() - 8]);
    obsolete
}

#[test]
fn synthetic_v7_cli_refuses_raw_and_compressed_checkpoints() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("synthetic-ck7");
    std::fs::create_dir_all(&dir).unwrap();
    let compressed = dir.join("live.ckpt.zst");
    let raw = dir.join("live.ckpt");
    let png = dir.join("color.png");
    let checkpoint = synthetic_v7();
    std::fs::write(&compressed, zstd::stream::encode_all(checkpoint.as_slice(), 1).unwrap()).unwrap();
    std::fs::write(&raw, checkpoint).unwrap();
    for save in [&compressed, &raw] {
        let result = Command::new(env!("CARGO_BIN_EXE_easel")).env_remove("EASEL_WIDTH")
            .args(["render-checkpoint", save.to_str().unwrap(), png.to_str().unwrap(), "--mode", "color"])
            .env("RAYON_NUM_THREADS", "2")
            .output().unwrap();
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("obsolete pigment representation"), "{}", String::from_utf8_lossy(&result.stderr));
    }
}

#[test]
fn occupied_synthetic_v7_layout_is_explicitly_refused() {
    let raw = synthetic_v7();
    assert_eq!(&raw[..8], b"PAINTCK7");
    let error = paint::Canvas::read_state(&mut raw.as_slice()).err().unwrap();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert!(error.to_string().contains("wet paint in an obsolete pigment representation"));
}

#[test]
fn synthetic_v7_rejects_truncation_trailing_engine_words_and_wrong_editions() {
    let raw = synthetic_v7();
    assert_eq!(paint::Canvas::read_state(&mut &raw[..raw.len() - 1]).err().unwrap().kind(), std::io::ErrorKind::UnexpectedEof);
    let mut trailing = raw.clone();
    trailing.extend_from_slice(&1u64.to_le_bytes());
    assert!(paint::Canvas::read_state(&mut trailing.as_slice()).err().unwrap().to_string().contains("trailing data"));
    let mut wrong_engine = raw.clone();
    let position = wrong_engine.windows(9).position(|s| s == b"engine=1\n").unwrap();
    wrong_engine[position + 7] = b'2';
    assert!(paint::Canvas::read_state(&mut wrong_engine.as_slice()).err().unwrap().to_string().contains("PAINTCK7 is an engine-1 checkpoint"));
    let mut unknown = raw;
    unknown[7] = b'6';
    assert!(paint::Canvas::read_state(&mut unknown.as_slice()).err().unwrap().to_string().contains("not a canvas checkpoint"));
}
