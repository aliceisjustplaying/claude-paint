//! Delivery contract: the PNG a painter saves is the canvas as it is seen
//! now (wet paint included), and a replay of the log delivers the same file.
// The replay build: EASEL_ROOT, named sessions and `easel run`.
#![cfg(feature = "replay")]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn root() -> PathBuf {
    Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("dl-{}", std::process::id()))
}

fn easel(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_easel")).args(args).env("EASEL_ROOT", root()).env("EASEL_SESSION", "wet").output().unwrap()
}

fn ok(args: &[&str]) -> String {
    let o = easel(args);
    assert!(o.status.success(), "easel {args:?}: {}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}

struct Closing;
impl Drop for Closing {
    fn drop(&mut self) {
        let _ = easel(&["close"]);
    }
}

const CANVAS: &str = r#"canvas{size=300, aspect=4, seed=5, linen=15, ground={{pile={{"lead white", 5}, {"yellow ochre", 1}}, um=80, apply="knife"}}}"#;
// freshly laid, still open: drying it would change how it looks
const WET: &str = r#"work(rect(0, 0, 1000, 250), {hand="broad", pile=pile{{"lead white", 2}, {"bone black", 1}, {"cobalt blue", 1}, medium=0.4}, coverage=4})"#;

fn rgb(p: &Path) -> image::RgbImage {
    image::open(p).unwrap_or_else(|e| panic!("{}: {e}", p.display())).to_rgb8()
}

#[test]
fn save_delivers_the_wet_canvas_as_seen_and_replay_matches() {
    let _ = std::fs::remove_dir_all(root());
    let _closing = Closing;
    ok(&["open", "wet"]);
    ok(&["do", CANVAS]);
    ok(&["do", WET]);
    assert!(ok(&["do", "print(drying(500, 125))"]).starts_with("open"), "the fixture must still be wet");
    let saved = root().join("saved.png");
    ok(&["save", saved.to_str().unwrap()]);
    // the look of a native 1:1 window is what the painter sees
    let look = ok(&["look", "--crop", "100,40,500,240"]);
    let look = rgb(Path::new(look.split_whitespace().next().unwrap()));
    let s = rgb(&saved);
    assert_eq!(s.width(), 2400, "delivered at the live width");
    let (x0, y0) = (240, 96); // 100 and 40 units at 2.4 px/unit
    let mut worst = 0i32;
    for (x, y, p) in look.enumerate_pixels() {
        let q = s.get_pixel(x0 + x, y0 + y);
        for c in 0..3 {
            worst = worst.max((p[c] as i32 - q[c] as i32).abs());
        }
    }
    // save dithers by at most one level; a dried preview differs far more
    assert!(worst <= 1, "the saved PNG is not the canvas as seen (max difference {worst} levels)");
    ok(&["close"]);
    // `easel run` of the log delivers the same bytes
    let replayed = root().join("replayed.png");
    ok(&["run", root().join("paintings/lua/wet.lua").to_str().unwrap(), "--out", replayed.to_str().unwrap()]);
    assert!(std::fs::read(&saved).unwrap() == std::fs::read(&replayed).unwrap(), "replay delivers a different PNG");
}

#[test]
fn replays_render_only_at_the_live_width() {
    let _ = std::fs::create_dir_all(root());
    let log = root().join("one.lua");
    std::fs::write(&log, format!("--@ chunk 1\n{CANVAS}\n")).unwrap();
    for extra in [&["--width", "1000"][..], &["--width", "2400"][..], &["--crop", "0,0,100,100"][..]] {
        let mut args = vec!["run", log.to_str().unwrap()];
        args.extend_from_slice(extra);
        let o = easel(&args);
        assert!(!o.status.success(), "run {extra:?} accepted: no alternate renders");
        assert!(String::from_utf8_lossy(&o.stderr).contains("unknown argument"), "{extra:?}: {}", String::from_utf8_lossy(&o.stderr));
    }
}
