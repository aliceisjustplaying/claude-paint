
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[path = "../../paint/tests/support/image_tolerance.rs"]
mod image_tolerance;

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
fn interrupted_index_publication_recovers_at_the_next_commit_after_checkpoint_resume() {
    let studio = Studio::new("index-recovery");
    std::fs::create_dir_all(studio.frames()).unwrap();
    let index = studio.frames().join("index.tsv");
    std::fs::create_dir(&index).unwrap();
    studio.ok(&["do", CANVAS]);
    assert!(studio.frames().join("000001.jpg").is_file());
    let log = std::fs::read_to_string(studio.0.join("out/easel/painting/server.log")).unwrap();
    assert!(log.contains("chunk 1 committed, frame not saved"), "{log}");
    for chunk in [PREPARE, MARK, "wait(1)", "wait(1)", "wait(1)", "wait(1)", "wait(1)", "wait(1)", "wait(1)"] {
        studio.ok(&["do", chunk]);
    }
    assert!(studio.0.join("out/easel/painting/replay.ckpt").is_file());
    studio.ok(&["close"]);
    let frames: Vec<_> = (1..=10).map(|chunk| {
        let path = studio.frames().join(format!("{chunk:06}.jpg"));
        let bytes = std::fs::read(&path).unwrap();
        let modified = std::fs::metadata(&path).unwrap().modified().unwrap();
        (path, bytes, modified)
    }).collect();
    std::fs::remove_dir(&index).unwrap();
    studio.ok(&["open", "painting"]);
    let log = std::fs::read_to_string(studio.0.join("out/easel/painting/server.log")).unwrap();
    assert!(log.contains("reconstructed 10 chunks from"), "{log}");
    studio.ok(&["do", "wait(1)"]);
    let text = std::fs::read_to_string(&index).unwrap();
    let rows: Vec<_> = text.lines().skip(1).collect();
    assert_eq!(rows.len(), 11, "{text}");
    for (chunk, row) in (1..=11).zip(&rows) {
        assert_eq!(row.split('\t').next().unwrap(), chunk.to_string());
    }
    assert_eq!(rows[0], "1\t0\t0");
    let latest: Vec<_> = rows.last().unwrap().split('\t').collect();
    assert!(latest[1].parse::<f64>().unwrap() > 0.0);
    assert_eq!(latest[2], "1");
    for (path, bytes, modified) in frames {
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
        assert_eq!(std::fs::metadata(path).unwrap().modified().unwrap(), modified);
    }
    studio.ok(&["close"]);
    studio.ok(&["open", "painting"]);
    studio.ok(&["do", "wait(1)"]);
    let text = std::fs::read_to_string(&index).unwrap();
    assert_eq!(text.lines().filter(|line| *line == "1\t0\t0").count(), 1);
    assert_eq!(text.lines().count(), 13);
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
    for file in ["plain.png", "gallery.png", "out/easel/painting/live.png"] {
        image_tolerance::assert_close(&image::open(on.0.join(file)).unwrap().to_rgba8(), &image::open(off.0.join(file)).unwrap().to_rgba8());
    }
    let committed = "out/easel/painting/committed.lua";
    assert_eq!(std::fs::read(on.0.join(committed)).unwrap(), std::fs::read(off.0.join(committed)).unwrap());
    let checkpoint = "out/easel/painting/live.ckpt";
    let (on_canvas, _) = paint::Canvas::read_state(&mut std::io::BufReader::new(std::fs::File::open(on.0.join(checkpoint)).unwrap())).unwrap();
    let (off_canvas, _) = paint::Canvas::read_state(&mut std::io::BufReader::new(std::fs::File::open(off.0.join(checkpoint)).unwrap())).unwrap();
    assert_eq!((on_canvas.window().w, on_canvas.window().h), (off_canvas.window().w, off_canvas.window().h));
    image_tolerance::assert_fields_close(on_canvas.seen().into_iter().flatten(), off_canvas.seen().into_iter().flatten());
    image_tolerance::assert_fields_close(on_canvas.surface_um().iter().copied(), off_canvas.surface_um().iter().copied());
    assert!(!off.frames().exists());
}
