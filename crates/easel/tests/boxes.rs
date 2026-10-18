//! Boxes: a new painting takes its box from a file `box` next to the easel
//! (a studio's `bin/box`), else EASEL_BOX, else the default tube box; its log
//! names the box, and every replay uses the log's box whatever easel runs it.
// The replay build: EASEL_ROOT, named sessions and `easel run`.
#![cfg(feature = "replay")]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

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
/// picture (platform-aware image comparison) and exact surface that
/// round 19's easel made of it (recorded with r19-base,
/// 5069814). At the live width it takes about 27 s: `scripts/test --all` runs
/// it; `scripts/test` runs the same log at 320 px against af49348's release
/// replay (scripts/tests/old_logs.sh, case r19).
#[test]
#[ignore = "slow"]
fn a_round_19_log_replays_as_before() {
    let dir = base("r19");
    let fixture = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/r19_default_box.lua");
    let (png, surf) = (dir.join("r19.png"), dir.join("r19.surf"));
    let out = ok(&plain(), &dir, None, &["run", fixture, "--out", png.to_str().unwrap(), "--dump-surface", surf.to_str().unwrap()]);
    assert!(out.starts_with("lead white, smalt, pale smalt, yellow ochre, red earth, vermilion, raw umber, bone black, cobalt blue, chrome yellow, Prussian blue, green earth, Rinmann's green, copper green\n"), "{out}");
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for b in std::fs::read(&surf).unwrap() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100_0000_01b3);
    }
    let surface_reference = if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/r19_default_box.surface-fnv.linux-x86_64")
    } else {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/r19_default_box.surface-fnv")
    };
    let want = std::fs::read_to_string(surface_reference).unwrap();
    assert_eq!(format!("{h:016x}"), want.trim(), "the round 19 surface changed");
    let reference = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/r19_default_box.png");
    let reference_platform = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/r19_default_box.platform")).unwrap_or_else(|_| "unknown".into());
    let platform = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);
    let known_platforms = ["macos-aarch64", "macos-x86_64", "linux-aarch64", "linux-x86_64", "windows-aarch64", "windows-x86_64"];
    if reference_platform.trim() == platform || !known_platforms.contains(&reference_platform.trim()) || !known_platforms.contains(&platform.as_str()) {
        assert!(std::fs::read(&png).unwrap() == std::fs::read(reference).unwrap(), "round 19 PNG bytes changed on the same or unknown platform");
        return;
    }
    let got = image::open(&png).unwrap().to_rgba8();
    let want = image::open(reference).unwrap().to_rgba8();
    assert_eq!(got.dimensions(), want.dimensions());
    let mut changed = 0usize;
    let mut maximum = 0u8;
    for (a, b) in got.pixels().zip(want.pixels()) {
        assert_eq!(a.0[3], b.0[3], "round 19 image alpha changed");
        let delta = a.0[..3].iter().zip(&b.0[..3]).map(|(x, y)| x.abs_diff(*y)).max().unwrap();
        changed += usize::from(delta != 0);
        maximum = maximum.max(delta);
    }
    let pixels = got.width() as usize * got.height() as usize;
    assert!(maximum <= 2 && changed * 1_000 < pixels,
            "cross-platform round 19 image drift: {changed}/{pixels} pixels, max {maximum}/255; allowed <0.1% pixels and <=2/255 per channel");
}
