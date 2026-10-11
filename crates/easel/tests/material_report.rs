#![cfg(feature = "replay")]

use paint::{Canvas, color::linear_to_srgb};
use std::{
    path::PathBuf,
    process::{Command, Output},
};

const HEADER: &str = "chunk_index,native_width_px,native_height_px,recorded_engine,material_total_volume_proxy_coats_times_square_units,solvent_total_volume_proxy_coats_times_square_units";
const LOG: &str = "--@ engine 5\n--@ chunk 1\ncanvas{size=160,aspect=2,linen=16,seed=73,ground={{pile={{\"lead white\",3}},um=100,apply=\"knife\"}}}\nprint(\"canvas ready\")\n--@ chunk 2\nb=brush(\"flat\",100); b:load(pile{{\"bone black\",1},thinner=0.5},0.8); b:stroke({{100,250},{900,250}})\nprint(\"paint laid\")\n";

struct Fixture(PathBuf);

impl Fixture {
    fn new(name: &str, log: &str) -> Self {
        let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
            .join(format!("material-report-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("probe.lua"), log).unwrap();
        Self(dir)
    }

    fn run(&self, extra: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_easel"))
            .arg("run")
            .arg(self.0.join("probe.lua"))
            .args(["--width", "64"])
            .args(extra)
            .env("EASEL_ROOT", &self.0)
            .env_remove("EASEL_SESSION")
            .env_remove("EASEL_BOX")
            .env_remove("EASEL_BRUSH_SIM")
            .env("RAYON_NUM_THREADS", "2")
            .output()
            .unwrap()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn success(output: &Output) {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn rgb_bytes(pixels: Vec<[f32; 3]>) -> Vec<u8> {
    pixels
        .iter()
        .flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8))
        .collect()
}

#[test]
fn report_and_paired_pngs_match_the_saved_final_canvas_without_changing_defaults() {
    let fixture = Fixture::new("paired", LOG);
    let gallery = fixture.0.join("gallery.png");
    let color = fixture.0.join("color.png");
    let report = fixture.0.join("materials.csv");
    let state = fixture.0.join("final.ckpt");
    let output = fixture.run(&[
        "--out",
        gallery.to_str().unwrap(),
        "--gallery",
        "--also-color",
        color.to_str().unwrap(),
        "--material-report",
        report.to_str().unwrap(),
        "--save-final-canvas",
        state.to_str().unwrap(),
    ]);
    success(&output);
    let (canvas, header) = Canvas::read_state(&mut std::fs::File::open(&state).unwrap()).unwrap();
    assert_eq!(header, LOG);
    let csv = std::fs::read_to_string(&report).unwrap();
    let mut lines = csv.lines();
    assert_eq!(lines.next(), Some(HEADER));
    let rows: Vec<Vec<&str>> = lines.map(|line| line.split(',').collect()).collect();
    assert_eq!(rows.len(), 2);
    for (i, row) in rows.iter().enumerate() {
        assert_eq!(row.len(), 6);
        assert_eq!(row[0].parse::<usize>().unwrap(), i + 1);
        assert_eq!(row[1].parse::<usize>().unwrap(), canvas.window().w);
        assert_eq!(row[2].parse::<usize>().unwrap(), canvas.window().h);
        assert_eq!(row[3].parse::<u32>().unwrap(), canvas.engine());
        assert!(row[4].parse::<f64>().unwrap().is_finite());
        assert!(row[5].parse::<f64>().unwrap().is_finite());
    }
    assert_eq!(canvas.window().w, 64);
    assert_eq!(canvas.window().h, 32);
    assert_eq!(canvas.engine(), 5);
    assert_eq!(rows[1][4].parse::<f64>().unwrap(), canvas.material_total());
    assert_eq!(rows[1][5].parse::<f64>().unwrap(), canvas.solvent_total());
    assert!(canvas.material_total() > rows[0][4].parse::<f64>().unwrap());
    assert!(canvas.solvent_total() > 0.0);
    assert_eq!(
        image::open(&color).unwrap().to_rgb8().into_raw(),
        rgb_bytes(canvas.seen())
    );
    assert_eq!(
        image::open(&gallery).unwrap().to_rgb8().into_raw(),
        rgb_bytes(canvas.seen_lit(115.0, 55.0, 1.0))
    );
    let default = fixture.run(&[]);
    success(&default);
    assert_eq!(output.stdout, default.stdout);
    assert_eq!(default.stdout, b"canvas ready\npaint laid\n");
    let default_png = fixture.0.join("out/lua/probe.png");
    assert_eq!(
        std::fs::read(&color).unwrap(),
        std::fs::read(default_png).unwrap()
    );
    let default_gallery = fixture.0.join("default-gallery.png");
    let default = fixture.run(&["--out", default_gallery.to_str().unwrap(), "--gallery"]);
    success(&default);
    assert_eq!(output.stdout, default.stdout);
    assert_eq!(
        std::fs::read(&gallery).unwrap(),
        std::fs::read(default_gallery).unwrap()
    );
}

#[test]
fn export_options_are_independent_and_report_the_logs_engine() {
    let fixture = Fixture::new("independent", &LOG.replace("--@ engine 5", "--@ engine 4"));
    let report = fixture.0.join("materials.csv");
    success(&fixture.run(&["--material-report", report.to_str().unwrap()]));
    let csv = std::fs::read_to_string(report).unwrap();
    assert!(
        csv.lines()
            .skip(1)
            .all(|line| line.split(',').nth(3) == Some("4"))
    );
    let state = fixture.0.join("nested/final.ckpt");
    success(&fixture.run(&["--save-final-canvas", state.to_str().unwrap()]));
    let (canvas, _) = Canvas::read_state(&mut std::fs::File::open(state).unwrap()).unwrap();
    assert_eq!(canvas.engine(), 4);
}

#[test]
fn malformed_export_arguments_and_conflicting_paths_are_rejected_before_replay() {
    let fixture = Fixture::new("arguments", LOG);
    let shared = fixture.0.join("shared");
    let path = shared.to_str().unwrap();
    for option in ["--material-report", "--save-final-canvas"] {
        for args in [
            vec![option],
            vec![option, ""],
            vec![option, "--gallery"],
            vec![option, path, option, path],
        ] {
            let output = fixture.run(&args);
            assert!(!output.status.success(), "accepted {args:?}");
            assert!(output.stdout.is_empty());
            assert!(!shared.exists());
        }
        for other in [
            "--out",
            "--also-color",
            "--state-digest",
            "--dump-surface",
            "--dump-state",
            "--frames-dir",
        ] {
            let args = ["--gallery", option, path, other, path];
            let output = fixture.run(&args);
            assert!(!output.status.success(), "accepted {args:?}");
            assert!(String::from_utf8_lossy(&output.stderr).contains("must differ"));
            assert!(output.stdout.is_empty());
            assert!(!shared.exists());
        }
        let look = shared.with_extension("look.png");
        let output = fixture.run(&["--out", path, "--look", option, look.to_str().unwrap()]);
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("must differ"));
        assert!(!look.exists());
    }
    let output = fixture.run(&["--material-report", path, "--save-final-canvas", path]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("must differ"));
    assert!(!shared.exists());
    let alias = fixture.0.join("nested/../shared");
    let output = fixture.run(&["--out", path, "--material-report", alias.to_str().unwrap()]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("must differ"));
    assert!(!shared.exists());
    let default = fixture.0.join("out/lua/probe.png");
    let output = fixture.run(&["--save-final-canvas", default.to_str().unwrap()]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("must differ"));
    assert!(!default.exists());
}

#[test]
fn failed_chunks_are_not_reported_and_never_save_a_final_canvas() {
    let fixture = Fixture::new(
        "failed",
        &format!("{LOG}--@ chunk 3\nerror('intentional failure')\n"),
    );
    let report = fixture.0.join("materials.csv");
    let state = fixture.0.join("final.ckpt");
    let output = fixture.run(&[
        "--material-report",
        report.to_str().unwrap(),
        "--save-final-canvas",
        state.to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("chunk 3 failed"));
    assert_eq!(std::fs::read_to_string(report).unwrap().lines().count(), 3);
    assert!(!state.exists());
}
