//! Replays must not depend on the process: tables keyed by objects walk in
//! the same order in every run (review 3, finding 2).

use std::process::Command;

const PROGRAM: &str = r##"
--@ chunk 1
canvas{aspect=1, seed=7}
items = {}
for i = 1, 32 do items[{index = i}] = i end
fs = {}
for i = 1, 8 do fs[function() return i end] = i end
--@ chunk 2
local first = next(items)
local o = {}
for k, v in pairs(items) do o[#o + 1] = v end
for k, v in pairs(fs) do o[#o + 1] = v end
print("first", first.index, table.concat(o, ","))
glaze(everywhere(), {color = rgb(first.index * 7, 0, 0), coats = 1})
"##;

#[test]
fn object_keyed_tables_replay_the_same_in_every_process() {
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("easel-determinism");
    std::fs::create_dir_all(&dir).unwrap();
    let src = dir.join("object-pairs.lua");
    std::fs::write(&src, PROGRAM).unwrap();
    let mut runs = Vec::new();
    for i in 0..3 {
        let png = dir.join(format!("object-pairs-{i}.png"));
        let out = Command::new(env!("CARGO_BIN_EXE_easel"))
            .args(["run", src.to_str().unwrap(), "--width", "80", "--out", png.to_str().unwrap()])
            .output()
            .unwrap();
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        let stdout = String::from_utf8(out.stdout).unwrap();
        runs.push((stdout, std::fs::read(&png).unwrap()));
    }
    let want: String = (1..=32).map(|i| i.to_string()).chain((1..=8).map(|i| i.to_string())).collect::<Vec<_>>().join(",");
    assert!(runs[0].0.contains(&format!("first\t1\t{want}")), "creation order: {}", runs[0].0);
    for r in &runs[1..] {
        assert_eq!(r.0, runs[0].0);
        assert!(r.1 == runs[0].1, "the PNGs differ");
    }
}
