#![cfg(feature = "replay")]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const CANVAS: &str = "canvas{size=300, aspect=5, linen={16,14}, seed=3, ground={{pile={{\"lead white\",6},{\"raw umber\",1}}, um=50, apply=\"roller\"}}}";
const PREPARE: &str = "p=pile{{\"lead white\",6},{\"cobalt blue\",1}}; b=brush(\"filbert\",8); b:load(p,0.9)";
const MARK: &str = "b:stroke({{100,100},{800,130}}, {pressure=0.8})";

struct Studio(PathBuf);

impl Studio {
    fn new(name: &str) -> Self {
        let studio = Self(Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("live-frames-{name}-{}", std::process::id())));
        std::fs::create_dir_all(&studio.0).unwrap();
        studio.ok(&["open", "painting"]);
        studio
    }

    fn cmd(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_easel")).args(args).env("EASEL_ROOT", &self.0)
            .env("EASEL_SESSION", "painting").env("RAYON_NUM_THREADS", "2").env_remove("EASEL_WIDTH").env_remove("EASEL_BOX").output().unwrap()
    }

    fn ok(&self, args: &[&str]) -> String {
        let result = self.cmd(args);
        assert!(result.status.success(), "{args:?}: {}", String::from_utf8_lossy(&result.stderr));
        String::from_utf8(result.stdout).unwrap()
    }

    fn frames(&self) -> PathBuf { self.0.join("out/easel/painting/frames") }
}

impl Drop for Studio {
    fn drop(&mut self) {
        let _ = self.cmd(&["close"]);
    }
}

#[test]
fn committed_frames_are_numbered_atomic_and_not_replayed() {
    let studio = Studio::new("commits");
    studio.ok(&["do", CANVAS]);
    let first = studio.frames().join("000001.jpg");
    let image = image::open(&first).unwrap();
    assert_eq!((image.width(), image.height()), (1920, 384));
    studio.ok(&["do", PREPARE]);
    studio.ok(&["do", MARK]);
    let index = std::fs::read_to_string(studio.frames().join("index.tsv")).unwrap();
    let painted: Vec<_> = index.lines().skip(1).map(|row| row.split('\t').last().unwrap()).collect();
    assert_eq!(painted, ["0", "0", "1"]);
    assert!(!studio.cmd(&["do", "error('failed chunk')"]).status.success());
    assert!(!studio.frames().join("000004.jpg").exists());
    studio.ok(&["frames", "off"]);
    studio.ok(&["do", MARK]);
    assert!(!studio.frames().join("000004.jpg").exists());
    studio.ok(&["frames", "on"]);
    studio.ok(&["do", MARK]);
    assert!(studio.frames().join("000005.jpg").exists());
    studio.ok(&["do", "--scratch", "x=1"]);
    assert!(!studio.0.join("out/easel/scratch/frames").exists());
    studio.ok(&["close"]);
    let before: Vec<_> = std::fs::read_dir(studio.frames()).unwrap().map(|entry| {
        let path = entry.unwrap().path();
        assert!(!path.extension().is_some_and(|extension| extension == "pending"));
        let modified = std::fs::metadata(&path).unwrap().modified().unwrap();
        let bytes = std::fs::read(&path).unwrap();
        (path, modified, bytes)
    }).collect();
    studio.ok(&["open", "painting"]);
    studio.ok(&["close"]);
    assert_eq!(std::fs::read_dir(studio.frames()).unwrap().count(), before.len());
    for (path, modified, bytes) in before {
        assert_eq!(std::fs::metadata(&path).unwrap().modified().unwrap(), modified);
        assert_eq!(std::fs::read(path).unwrap(), bytes);
    }
    std::fs::remove_file(first).unwrap();
    studio.ok(&["open", "painting"]);
    assert!(!studio.frames().join("000001.jpg").exists());
    std::fs::rename(studio.frames(), studio.frames().with_extension("kept")).unwrap();
    std::fs::write(studio.frames(), b"not a directory").unwrap();
    let reply = studio.ok(&["do", MARK]);
    assert!(reply.contains("ok · chunk 6"), "{reply}");
    assert!(!reply.contains("frame"), "{reply}");
    assert!(std::fs::read_to_string(studio.0.join("out/easel/painting/committed.lua")).unwrap().contains("--@ chunk 6"));
    let log = std::fs::read_to_string(studio.0.join("out/easel/painting/server.log")).unwrap();
    assert!(log.contains("chunk 6 committed, frame not saved"), "{log}");
}

#[test]
fn recording_leaves_painting_bytes_and_replay_unchanged() {
    let on = Studio::new("on");
    let off = Studio::new("off");
    off.ok(&["frames", "off"]);
    for studio in [&on, &off] {
        for chunk in [CANVAS, PREPARE, MARK, "wait(60)", MARK] {
            studio.ok(&["do", chunk]);
        }
        studio.ok(&["save", "plain.png"]);
        studio.ok(&["save", "gallery.png", "--gallery"]);
        studio.ok(&["check"]);
        studio.ok(&["close"]);
    }
    for file in ["plain.png", "gallery.png", "out/easel/painting/committed.lua", "out/easel/painting/live.png", "out/easel/painting/live.ckpt"] {
        assert_eq!(std::fs::read(on.0.join(file)).unwrap(), std::fs::read(off.0.join(file)).unwrap(), "{file}");
    }
    assert!(!off.frames().exists());
}
