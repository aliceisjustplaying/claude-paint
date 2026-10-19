use std::path::{Path, PathBuf};
use std::process::Command;

const LOG: &str = "--@ chunk 1
canvas{size=300, aspect=1.25, linen={16,14}, seed=3, ground={{pile={{\"lead white\",6},{\"raw umber\",1}}, um=50, apply=\"roller\"}}}
p=pile{{\"lead white\",6},{\"cobalt blue\",1}}; b=brush(\"filbert\",8); b:load(p,0.9)
b:stroke({{100,100},{800,130}}, {pressure=0.8})
--@ chunk 2
b:stroke({{100,300},{800,330}}, {pressure=0.8})
wait(5)
--@ chunk 3
b:load(p,0.9)
b:stroke({{100,500},{800,560}}, {pressure=0.6})
--@ chunk 4
b:stroke({{200,200},{300,700}}, {pressure=0.7})
--@ chunk 5
b:load(p,0.9)
b:stroke({{600,200},{700,700}}, {pressure=0.9})
";

struct Temp(PathBuf);

impl Temp {
    fn new() -> Self {
        let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("replay-frames-resume-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("painting.lua"), LOG).unwrap();
        Self(dir)
    }

    fn run(&self, frames: &str, checkpoint: Option<usize>) {
        let mut args = vec!["run".to_string(), self.0.join("painting.lua").display().to_string(), "--width".into(), "300".into(),
            "--out".into(), self.0.join(format!("{frames}.png")).display().to_string(),
            "--frames-every".into(), "2".into(), "--frames-dir".into(), self.0.join(frames).display().to_string(), "--frame-width".into(), "120".into()];
        if let Some(until) = checkpoint {
            args.extend(["--replay-checkpoint".into(), self.0.join("replay.ckpt").display().to_string(), "--until-chunk".into(), until.to_string()]);
        }
        let out = Command::new(env!("CARGO_BIN_EXE_easel")).args(&args).current_dir("/").env("RAYON_NUM_THREADS", "2").env_remove("EASEL_WIDTH").env_remove("EASEL_BOX").output().unwrap();
        assert!(out.status.success(), "{args:?}: {}", String::from_utf8_lossy(&out.stderr));
    }

    fn frames(&self, dir: &str) -> Vec<(String, Vec<u8>)> {
        let mut names: Vec<_> = std::fs::read_dir(self.0.join(dir)).unwrap().map(|e| e.unwrap().file_name().into_string().unwrap()).filter(|n| n.ends_with(".png") || n == "frames.tsv").collect();
        names.sort();
        names.into_iter().map(|n| { let bytes = std::fs::read(self.0.join(dir).join(&n)).unwrap(); (n, bytes) }).collect()
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn frames_resume_from_a_replay_checkpoint_as_if_never_interrupted() {
    let t = Temp::new();
    t.run("whole", None);
    t.run("resumed", Some(2));
    let tsv = t.0.join("resumed/frames.tsv");
    let mut stray = std::fs::read_to_string(&tsv).unwrap();
    stray.push_str("99999.png\t0.0\t0\t2\ttick\n");
    std::fs::write(&tsv, stray).unwrap();
    std::fs::write(t.0.join("resumed/99999.png"), b"left by an interrupted chunk").unwrap();
    t.run("resumed", Some(4));
    t.run("resumed", Some(5));
    let whole = t.frames("whole");
    assert!(whole.len() > 6, "too few frames to test: {}", whole.len());
    assert_eq!(whole, t.frames("resumed"));
    assert_eq!(std::fs::read(t.0.join("whole.png")).unwrap(), std::fs::read(t.0.join("resumed.png")).unwrap());
}
