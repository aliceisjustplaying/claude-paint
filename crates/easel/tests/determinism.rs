//! Replays must not depend on the process or the thread count: tables
//! keyed by objects walk in the same order in every run, and a program with
//! hand time (a long pass painted in slices, the paint ageing between them)
//! paints the same picture at any thread count.
// The replay build: EASEL_ROOT, named sessions and `easel run`.
#![cfg(feature = "replay")]

use std::path::{Path, PathBuf};
use std::process::Command;

fn dir() -> PathBuf {
    let d = Path::new(env!("CARGO_TARGET_TMPDIR")).join("easel-determinism");
    std::fs::create_dir_all(&d).unwrap();
    d
}

/// Replay `src` (at the live width); returns stdout and the PNG's bytes.
fn replay(src: &Path, threads: Option<usize>, tag: &str) -> (String, Vec<u8>) {
    let png = dir().join(format!("{tag}.png"));
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_easel"));
    cmd.args(["run", src.to_str().unwrap(), "--out", png.to_str().unwrap()]);
    if let Some(n) = threads {
        cmd.env("RAYON_NUM_THREADS", n.to_string());
    }
    let out = cmd.output().unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    (String::from_utf8(out.stdout).unwrap(), std::fs::read(&png).unwrap())
}

const CANVAS: &str = r#"canvas{size=440, aspect=1.6, linen=15, seed=7, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=120, apply="knife"}}}"#;

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
    let runs: Vec<_> = (0..3).map(|i| replay(&src, None, &format!("object-pairs-{i}"))).collect();
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
    let (o1, p1) = replay(&src, Some(1), "hand-1");
    let (o4, p4) = replay(&src, Some(4), "hand-4");
    let (o4b, p4b) = replay(&src, Some(4), "hand-4b");
    assert!(o1 == o4 && o4 == o4b, "{o1}\n{o4}");
    assert!(p1 == p4 && p4 == p4b, "the pictures differ between replays");
    // the broad pass took the hand a while (later than 09:15 when it ended)
    let first = o1.lines().next().unwrap();
    assert!(first.starts_with("day 1, ") && first > "day 1, 09:15", "{o1}");
}

/// `run --state-digest` writes one line of state digests per chunk, and two
/// replays of a log (here at different thread counts) write the same digests:
/// canvas (wet layer and drying state included), held brushes and studio.
#[test]
fn state_digests_are_the_same_in_every_replay() {
    let program = r#"
--@ chunk 1
canvas{size=440, aspect=5, linen=15, seed=5, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=120, apply="knife"}}}
--@ chunk 2
p = pile{{"lead white", 4}, {"cobalt blue", 1}, medium=0.3}
work(rect(0, 0, 1000, 90), {hand="broad", pile=p, angle=0, coverage=3})
wait(30)
--@ chunk 3
b = brush("filbert", 5); b:load(pile{{"raw umber", 1}, {"bone black", 1}}, 0.8)
for i = 1, 12 do b:stroke({{60 + 70 * i, 150}, {90 + 70 * i, 40}}) end
"#;
    let src = dir().join("digest.lua");
    std::fs::write(&src, program).unwrap();
    let digests = |threads: &str, tag: &str| {
        let (png, txt) = (dir().join(format!("{tag}.png")), dir().join(format!("{tag}.txt")));
        let out = Command::new(env!("CARGO_BIN_EXE_easel"))
            .args(["run", src.to_str().unwrap(), "--out", png.to_str().unwrap(), "--state-digest", txt.to_str().unwrap()])
            .env("RAYON_NUM_THREADS", threads)
            .output()
            .unwrap();
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        // drop the chunk's timing: every other field is state
        let text = std::fs::read_to_string(&txt).unwrap();
        text.lines().map(|l| l.split(' ').filter(|f| !f.starts_with("secs=")).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>()
    };
    let a = digests("1", "digest-1");
    let b = digests("4", "digest-4");
    assert_eq!(a, b, "the state digests differ between two replays");
    assert_eq!(a.len(), 3, "{a:?}");
    for (n, l) in a.iter().enumerate() {
        let f: Vec<&str> = l.split(' ').collect();
        assert_eq!(f[..2], ["chunk", &(n + 1).to_string()], "{l}");
        assert!(f[2].starts_with("canvas=") && f[2] != "canvas=0000000000000000" && f[3].starts_with("brushes=") && f[5].starts_with("studio="), "{l}");
    }
    // the brush made in chunk 3 is held
    assert!(a[1].contains("nbrushes=0") && a[2].contains("nbrushes=1"), "{a:?}");
    // the wait and the brush changed the canvas
    assert!(a[0][8..30] != a[1][8..30] && a[1][8..30] != a[2][8..30], "{a:?}");
}
