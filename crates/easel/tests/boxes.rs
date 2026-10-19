//! Boxes: a new painting takes its box from a file `box` next to the easel
//! (a studio's `bin/box`), else EASEL_BOX, else the default tube box; its log
//! names the box, and every replay uses the log's box whatever easel runs it.
// The replay build: EASEL_ROOT, named sessions and `easel run`.
#![cfg(feature = "replay")]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[path = "../../paint/tests/support/image_tolerance.rs"]
mod image_tolerance;

fn base(tag: &str) -> PathBuf {
    let d = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("boxes-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

/// Run `exe` with EASEL_ROOT `root` and EASEL_BOX `env_box` (unset if None).
fn run(exe: &Path, root: &Path, env_box: Option<&str>, args: &[&str]) -> Output {
    let mut c = Command::new(exe);
    c.args(args).env("EASEL_ROOT", root).env("EASEL_SESSION", "p").env_remove("EASEL_BOX").env_remove("EASEL_WIDTH");
    if let Some(b) = env_box {
        c.env("EASEL_BOX", b);
    }
    c.output().unwrap()
}

fn ok(exe: &Path, root: &Path, env_box: Option<&str>, args: &[&str]) -> String {
    let o = run(exe, root, env_box, args);
    assert!(o.status.success(), "easel {args:?}: {}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}

/// The plain replay build: no box file next to it (the tests unset EASEL_BOX).
fn plain() -> PathBuf {
    let e = PathBuf::from(env!("CARGO_BIN_EXE_easel"));
    assert!(!e.with_file_name("box").exists(), "a box file next to the test's easel");
    assert!(!e.with_file_name("width").exists(), "a width file next to the test's easel");
    e
}

/// A log from round 19 (no box line, the fourteen tubes) replays to the
/// picture within image tolerance that
/// round 19's easel made of it (recorded with r19-base,
/// 5069814). At the live width it takes about 27 s: `scripts/test --all` runs
/// it; `scripts/test` runs the same log at 320 px against af49348's release
/// replay (scripts/tests/old_logs.sh, case r19).
#[test]
#[ignore = "slow"]
fn a_round_19_log_replays_as_before() {
    let dir = base("r19");
    let fixture = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/r19_default_box.lua");
    let png = dir.join("r19.png");
    let out = ok(&plain(), &dir, None, &["run", fixture, "--out", png.to_str().unwrap()]);
    assert!(out.starts_with("lead white, smalt, pale smalt, yellow ochre, red earth, vermilion, raw umber, bone black, cobalt blue, chrome yellow, Prussian blue, green earth, Rinmann's green, copper green\n"), "{out}");
    let reference = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/r19_default_box.png");
    let got = image::open(&png).unwrap().to_rgba8();
    let want = image::open(reference).unwrap().to_rgba8();
    image_tolerance::assert_close(&got, &want);
}
