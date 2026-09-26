//! Replays must not depend on the process or the thread count: tables
//! keyed by objects walk in the same order in every run, and a program with
//! hand time (a long pass painted in slices, the paint ageing between them)
//! paints the same picture at any thread count.

use std::path::{Path, PathBuf};
use std::process::Command;

fn dir() -> PathBuf {
    let d = Path::new(env!("CARGO_TARGET_TMPDIR")).join("easel-determinism");
    std::fs::create_dir_all(&d).unwrap();
    d
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

const CANVAS: &str = r#"canvas{size=440, aspect=1, linen=15, seed=7, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=120, apply="knife"}}}"#;

#[test]
fn object_keyed_tables_replay_the_same_in_every_process() {
    let program = format!(
        r#"
--@ chunk 1
{CANVAS}
items = {{}}
for i = 1, 32 do items[{{index = i}}] = i end
fs = {{}}
for i = 1, 8 do fs[function() return i end] = i end
--@ chunk 2
local first = next(items)
local o = {{}}
for k, v in pairs(items) do o[#o + 1] = v end
for k, v in pairs(fs) do o[#o + 1] = v end
print("first", first.index, table.concat(o, ","))
work(rect(0, 0, 10 * first.index, 1000), {{hand="broad", pile=pile{{{{"bone black", 1}}}}}})
"#
    );
    let src = dir().join("object-pairs.lua");
    std::fs::write(&src, program).unwrap();
    let runs: Vec<_> = (0..3).map(|i| replay(&src, 80, None, &format!("object-pairs-{i}"))).collect();
    let want: String = (1..=32).map(|i| i.to_string()).chain((1..=8).map(|i| i.to_string())).collect::<Vec<_>>().join(",");
    assert!(runs[0].0.contains(&format!("first\t1\t{want}")), "creation order: {}", runs[0].0);
    for r in &runs[1..] {
        assert_eq!(r.0, runs[0].0);
        assert!(r.1 == runs[0].1, "the PNGs differ");
    }
}

/// With hand time the paint ages while the hand works; the same program
/// paints the same picture at any thread count and on every replay.
#[test]
fn hand_time_is_deterministic_across_thread_counts() {
    let program = format!(
        r#"
--@ chunk 1
{CANVAS}
--@ chunk 2
p = pile{{{{"lead white", 6}}, {{"cobalt blue", 1}}, {{"yellow ochre", 0.3}}, medium=0.3}}
work(rect(0, 0, 1000, 470), {{hand="broad", pile=p, angle=0, coverage=4.5}})
print(wait(0))
--@ chunk 3
print(wait(3 * 60))
stipple(rect(200, 380, 600, 120), {{width=3, pile=pile{{{{"lead white", 1}}}}, coverage=1.5}})
b = brush("round", 3); b:load(pile{{{{"bone black", 1}}, {{"raw umber", 1}}}}, 0.9)
for i = 1, 40 do b:stroke({{{{100 + 20 * i, 520}}, {{110 + 20 * i, 460}}}}) end
print(drying(500, 200), drying(500, 490))
"#
    );
    let src = dir().join("hand-time.lua");
    std::fs::write(&src, program).unwrap();
    let (o1, p1) = replay(&src, 200, Some(1), "hand-1");
    let (o4, p4) = replay(&src, 200, Some(4), "hand-4");
    let (o4b, p4b) = replay(&src, 200, Some(4), "hand-4b");
    assert!(o1 == o4 && o4 == o4b, "{o1}\n{o4}");
    assert!(p1 == p4 && p4 == p4b, "the pictures differ between replays");
    // the broad pass took the hand a while (later than 09:15 when it ended)
    let first = o1.lines().next().unwrap();
    assert!(first.starts_with("day 1, ") && first > "day 1, 09:15", "{o1}");
}
