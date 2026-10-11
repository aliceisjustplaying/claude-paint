#![cfg(feature = "replay")]

use std::{path::PathBuf, process::Command};

#[test]
fn optional_color_export_reads_the_same_final_canvas_and_keeps_default_delivery() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("also-color-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let log = dir.join("probe.lua");
    std::fs::write(&log, "--@ engine 5\n--@ chunk 1\ncanvas{size=160,aspect=2,linen=16,seed=73,ground={{pile={{\"lead white\",3}},um=100,apply=\"knife\"}}}\nb=brush(\"flat\",100); b:load(pile{{\"bone black\",1}},0.8); b:stroke({{100,250},{900,250}})\n").unwrap();
    let gallery = dir.join("gallery.png");
    let color = dir.join("color.png");
    let default = dir.join("default.png");
    let default_gallery = dir.join("default-gallery.png");
    let run = |extra: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_easel"))
            .args(["run", log.to_str().unwrap(), "--width", "64"])
            .args(extra)
            .env("EASEL_ROOT", &dir)
            .env_remove("EASEL_BOX")
            .env_remove("EASEL_BRUSH_SIM")
            .env("RAYON_NUM_THREADS", "2")
            .output()
            .unwrap()
    };
    for args in [
        vec![
            "--out",
            gallery.to_str().unwrap(),
            "--gallery",
            "--also-color",
            color.to_str().unwrap(),
        ],
        vec!["--out", default.to_str().unwrap()],
        vec!["--out", default_gallery.to_str().unwrap(), "--gallery"],
    ] {
        let output = run(&args);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    assert_eq!(
        std::fs::read(&color).unwrap(),
        std::fs::read(&default).unwrap()
    );
    assert_eq!(
        std::fs::read(&gallery).unwrap(),
        std::fs::read(&default_gallery).unwrap()
    );
    assert_eq!(image::image_dimensions(&color).unwrap(), (64, 32));
    assert_ne!(
        std::fs::read(&color).unwrap(),
        std::fs::read(&gallery).unwrap()
    );
    for args in [
        vec!["--also-color"],
        vec!["--also-color", color.to_str().unwrap()],
        vec![
            "--out",
            color.to_str().unwrap(),
            "--gallery",
            "--also-color",
            color.to_str().unwrap(),
        ],
    ] {
        assert!(
            !run(&args).status.success(),
            "invalid arguments accepted: {args:?}"
        );
    }
    std::fs::remove_dir_all(dir).unwrap();
}
