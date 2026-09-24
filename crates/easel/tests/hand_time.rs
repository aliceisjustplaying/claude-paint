//! Hand time (notes/time.md): off by default, so existing logs replay
//! exactly as before; on, the same program paints the same picture at any
//! thread count.

use std::path::{Path, PathBuf};
use std::process::Command;

fn dir() -> PathBuf {
    let d = Path::new(env!("CARGO_TARGET_TMPDIR")).join("easel-hand-time");
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// FNV-1a: a hash that stays the same across Rust versions.
fn fnv(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &b| (h ^ b as u64).wrapping_mul(0x100_0000_01b3))
}

/// Replay `src` at `width`; returns stdout and the PNG's bytes.
fn replay(src: &Path, width: u32, threads: Option<usize>, tag: &str) -> (String, Vec<u8>) {
    let png = dir().join(format!("{tag}.png"));
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_easel"));
    cmd.args(["run", src.to_str().unwrap(), "--width", &width.to_string(), "--out", png.to_str().unwrap()]);
    if let Some(n) = threads {
        cmd.env("RAYON_NUM_THREADS", n.to_string());
    }
    let out = cmd.output().unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    (String::from_utf8(out.stdout).unwrap(), std::fs::read(&png).unwrap())
}

/// The owner's logs replay byte for byte as they did before hand time
/// existed: the PNG hashes were recorded with the easel of commit 2c5a658,
/// built in each profile, then re-recorded when the Friedrich relief default
/// went from 0.2 to 0.06 (Round 6, the owner's pick): only the finishing
/// relief changed.
#[test]
fn existing_logs_replay_unchanged() {
    // (the benchmark near is checked in release only: a debug replay takes
    // minutes)
    let logs: &[(&str, u64)] = if cfg!(debug_assertions) {
        &[("paintings/lua/example.lua", 0x99a5_7233_a465_f4a9)]
    } else {
        &[("paintings/lua/example.lua", 0x99a5_7233_a465_f4a9), ("notes/loops/l5_near.lua", 0xd503_b048_7f30_dedc)]
    };
    for &(log, want) in logs {
        let (_, png) = replay(&root().join(log), 160, None, "unchanged");
        assert_eq!(fnv(&png), want, "{log} at 160px changed");
    }
}

const PROGRAM: &str = r##"
--@ chunk 1
canvas{style="friedrich", aspect=1.4, seed=7, hand=true}
--@ chunk 2
HZ = 470
sky = function(x, y) return gradient({{0,"#5d7396"},{0.6,"#9fabb8"},{1,"#e9d6a6"}}, y/HZ) end
skym = above(function(x) return HZ + 15 end)
work(skym, {hand="broad", color=sky, angle=0, coverage=4.5, medium=0.25})
print(string.format("sky %.3f", clock()))
--@ chunk 3
rest(3)
stipple(rect(0, 380, 1000, 120), {width=3, color="#cfccc2", coverage=1.5})
b = brush("round", 3); b:load("#2b2620", 0.9)
for i = 1, 40 do b:stroke({{100 + 20 * i, 520}, {110 + 20 * i, 460}}) end
local t = timesheet()
print(string.format("clock %.4f sitting %d %.4f open %.4f setting %.4f tacky %.4f", t.clock, t.sittings, t.sitting, t.open, t.setting, t.tacky))
"##;

/// With hand time on, the sky is painted in slices with the paint ageing
/// between them, a rest lets it set, and the stipple and strokes go on
/// setting paint: all of it the same at 1 and 4 threads.
#[test]
fn hand_time_is_the_same_at_any_thread_count() {
    let src = dir().join("threads.lua");
    std::fs::write(&src, PROGRAM).unwrap();
    let (o1, p1) = replay(&src, 200, Some(1), "threads-1");
    let (o4, p4) = replay(&src, 200, Some(4), "threads-4");
    let lines = |o: &str| o.lines().filter(|l| l.starts_with("sky") || l.starts_with("clock")).map(String::from).collect::<Vec<_>>();
    assert_eq!(lines(&o1), lines(&o4));
    assert_eq!(lines(&o1).len(), 2, "{o1}");
    assert!(p1 == p4, "the pictures differ between 1 and 4 threads");
    // the sky took its time, and the rest began a second sitting
    let sky: f64 = lines(&o1)[0].trim_start_matches("sky ").parse().unwrap();
    assert!(sky > 15.0, "a sky takes more than one slice: {sky} min");
    assert!(lines(&o1)[1].contains("sitting 2"), "{o1}");
}
