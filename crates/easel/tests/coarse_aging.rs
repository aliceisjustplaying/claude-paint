#![cfg(feature = "replay")]

use std::path::Path;
use std::process::Command;

fn replay(root: &Path, setting: Option<&str>, workers: usize, tag: &str) -> paint::Canvas {
    let checkpoint = root.join(format!("{tag}.ckpt"));
    let mut command = Command::new(env!("CARGO_BIN_EXE_easel"));
    command.args(["run", root.join("painting.lua").to_str().unwrap(), "--width", "160", "--out", root.join(format!("{tag}.png")).to_str().unwrap(), "--checkpoint-out", checkpoint.to_str().unwrap()]);
    command.env("EASEL_ROOT", root).env("RAYON_NUM_THREADS", workers.to_string()).env("PAINT_SMALL_TILES", "1").env_remove("PAINT_COARSE_AGING").env_remove("EASEL_BOX");
    if let Some(value) = setting {
        command.env("PAINT_COARSE_AGING", value);
    }
    let output = command.output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    paint::Canvas::read_state(&mut std::io::BufReader::new(std::fs::File::open(checkpoint).unwrap())).unwrap().0
}

#[test]
fn coarser_aging_keeps_hand_time_waits_and_chunk_end_flushes() {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("easel-coarse-aging");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("painting.lua"), r#"
--@ engine 5
--@ chunk 1
canvas{size=440, aspect=1.0, linen=15, seed=7, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=120, apply="knife"}}}
p = pile{{"lead white", 6}, {"cobalt blue", 1}}
--@ chunk 2
work(rect(100, 100, 700, 500), {hand="body", pile=p, coverage=3, fill=false})
--@ chunk 3
print(wait(17.25))
stipple(rect(150, 250, 550, 300), {width=4, pile=p, coverage=1.5})
"#).unwrap();
    let baseline = replay(&root, None, 1, "baseline");
    let disabled = replay(&root, Some("0"), 1, "disabled");
    let one = replay(&root, Some("1"), 1, "coarse-one");
    let four = replay(&root, Some("1"), 4, "coarse-four");
    let state = |c: &paint::Canvas| {
        let mut bytes = Vec::new();
        c.write_state(&mut bytes, "").unwrap();
        bytes
    };
    assert_eq!(state(&baseline), state(&disabled));
    assert_ne!(state(&one), state(&baseline));
    assert_eq!(state(&one), state(&four));
    assert_eq!(one.tally(), baseline.tally());
    assert_eq!(four.hand_owed_secs(), 0.0);
    assert!((one.clock() - baseline.clock()).abs() < 1e-3);
    assert!((one.tally().clocked - one.tally().secs).abs() < 1e-8);
}
