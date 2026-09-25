//! Hand time (notes/time.md), through `easel run`: a recorded log replays
//! to its recorded picture; with hand time on, the same program paints the
//! same picture at any thread count; old logs that overran their sittings
//! still replay, and new ones are held to them.

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

/// A deliberate pixel tripwire for the engine and the Lua API (the README's example); re-record the hash on intended changes.
#[test]
fn example_log_replays_as_recorded() {
    let (_, png) = replay(&root().join("paintings/lua/example.lua"), 160, None, "example");
    assert_eq!(fnv(&png), 0x016d_3a13_a4ec_1fd6, "paintings/lua/example.lua at 160px changed");
}

/// Opt-in, slow: the benchmark near replays to its recorded hash (re-record on intended changes). Run with
/// `cargo test --release -p easel --test hand_time -- --ignored` (a debug replay takes minutes).
#[test]
#[ignore]
fn l5_near_replays_as_recorded() {
    let (_, png) = replay(&root().join("notes/loops/l5_near.lua"), 160, None, "l5_near");
    assert_eq!(fnv(&png), 0xbb35_6706_3532_5dac, "notes/loops/l5_near.lua at 160px changed");
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

/// A log from before sittings were enforced replays as it did then, overrun
/// and all: tests/logs/overran.lua (hand time on, a 12-minute sitting that
/// runs 20 min, a second one that runs 3.8 h) is not refused, and prints
/// what it printed with the easel of commit 82fd8fb, the last before
/// sittings were enforced (release, 160px). Its pixels are the tripwire's
/// job, not this test's.
#[test]
fn an_old_overrunning_log_replays_unchanged() {
    let (out, _) = replay(&root().join("crates/easel/tests/logs/overran.lua"), 160, None, "overran");
    assert_eq!(
        out,
        "sitting 1: 20 min at the easel, 12 min planned; finish the passage while it is open, then rest(hours)
sky: sitting 1 0.337 of 0.200 h
sitting 2: 3.8 h at the easel, 30 min planned; finish the passage while it is open, then rest(hours)
clock 247.8083 sitting 2 226.4621 of 0.500 h
clock 368.2586 sittings 3
"
    );
}

/// The same log marked as a new one (the header line the easel writes for
/// every new session) is held to its sittings on replay: its third chunk
/// strokes after the first sitting ran out, and is refused.
#[test]
fn a_strict_log_is_held_to_its_sittings_on_replay() {
    let old = std::fs::read_to_string(root().join("crates/easel/tests/logs/overran.lua")).unwrap();
    let src = dir().join("overran_strict.lua");
    std::fs::write(&src, old.replacen("\n\n--@ chunk 1", &format!("\n{}\n\n--@ chunk 1", STRICT), 1)).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_easel")).args(["run", src.to_str().unwrap(), "--width", "160", "--out", dir().join("strict.png").to_str().unwrap()]).output().unwrap();
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success() && err.contains("chunk 3 failed") && err.contains("the sitting is over after 12 min: rest(hours) first"), "{err}");
}

const STRICT: &str = "-- sittings enforced: a sitting ends at its length; the easel refuses marks until rest(hours) (notes/time.md)";

/// A strict log that rests when its sittings end paints the same picture
/// at any thread count, and at every replay: with hand time on, the sky is
/// painted in slices with the paint ageing between them, a rest lets it set
/// and begins a second sitting, and the stipple and strokes go on setting
/// paint.
#[test]
fn a_strict_log_is_deterministic() {
    let src = dir().join("strict.lua");
    let prog = PROGRAM.replace("hand=true}", "hand=true}; sitting{hours=1.5}").replace("rest(3)", "rest(3); sitting{hours=6}");
    std::fs::write(&src, format!("{STRICT}\n{prog}")).unwrap();
    let (o1, p1) = replay(&src, 200, Some(1), "strict-1");
    let (o4, p4) = replay(&src, 200, Some(4), "strict-4");
    let (o4b, p4b) = replay(&src, 200, Some(4), "strict-4b");
    assert!(o1 == o4 && o4 == o4b, "{o1}\n{o4}");
    assert!(p1 == p4 && p4 == p4b, "the pictures differ between replays");
    // the sky took its time, and the rest began a second sitting
    let sky: f64 = o1.lines().find_map(|l| l.strip_prefix("sky ")).and_then(|t| t.parse().ok()).unwrap_or_else(|| panic!("no sky clock: {o1}"));
    assert!(sky > 15.0, "a sky takes more than one slice: {sky} min");
    assert!(o1.contains("sitting 2"), "{o1}");
}
