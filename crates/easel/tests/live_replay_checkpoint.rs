use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

const CHUNKS: [&str; 12] = [
    r#"canvas{size=300, aspect=5, linen={16,14}, seed=3, ground={{pile={{"lead white",6},{"raw umber",1}}, um=50, apply="roller"}}}"#,
    r#"p=pile{{"lead white",6},{"cobalt blue",1}}; q=pile{{"raw umber",1}}; b=brush("filbert",1); alias=b
local count, mix = 0, p
function mark(x)
  assert(alias == b)
  count = count + 1
  counter = count
  alias:load(mix,0.8)
  alias:stroke({{x,100},{x+2,102}}, {pressure=0.6})
end"#,
    "mark(100)",
    "wait(3)",
    "for i=1,24 do mark(200+i%4) end",
    "assert(counter == 25); alias:load(q,0.4)",
    "mark(300)",
    "wait(7)",
    "mark(400)",
    "mark(500)",
    "assert(counter == 28); mark(600)",
    "wait(2); mark(700); assert(counter == 30)",
];

struct Studio {
    root: PathBuf,
    binary: PathBuf,
    child: Option<Child>,
    identity: &'static str,
    state_store: bool,
}

impl Studio {
    fn new(tag: &str, painter: bool) -> Self {
        let root = Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join(format!("live-replay-checkpoint-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let root = root.canonicalize().unwrap();
        let binary = if painter {
            std::fs::create_dir(root.join("bin")).unwrap();
            let binary = root.join("bin/easel");
            std::fs::hard_link(env!("CARGO_BIN_EXE_easel"), &binary).unwrap();
            std::fs::write(root.join("bin/studio"), "painter none\n").unwrap();
            binary
        } else {
            PathBuf::from(env!("CARGO_BIN_EXE_easel"))
        };
        Self { root, binary, child: None, identity: "original", state_store: false }
    }

    fn command(&self) -> Command {
        let mut command = Command::new(&self.binary);
        command.current_dir("/")
            .env("EASEL_ROOT", &self.root)
            .env("EASEL_SESSION", "painting")
            .env("RAYON_NUM_THREADS", "2")
            .env("PAINT_LIVE_CHECKPOINT_TEST_IDENTITY", self.identity)
            .env_remove("EASEL_WIDTH")
            .env_remove("EASEL_BOX")
            .env_remove("EASEL_STATE_SESSION")
            .env_remove("EASEL_STATE_DISK_MIB")
            .env_remove("EASEL_LOCK_ON_STDIN");
        if self.state_store { command.env("EASEL_STATE_SESSION", "painting"); }
        command
    }

    fn cmd(&self, args: &[&str]) -> Output {
        self.command().args(args).output().unwrap()
    }

    fn ok(&self, args: &[&str]) -> String {
        let output = self.cmd(args);
        assert!(output.status.success(), "{args:?}: {}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        String::from_utf8(output.stdout).unwrap()
    }

    fn dir(&self) -> PathBuf { self.root.join("out/easel/painting") }
    fn log(&self) -> PathBuf { self.root.join("paintings/lua/painting.lua") }
    fn bytes(&self, name: &str) -> Vec<u8> { std::fs::read(self.dir().join(name)).unwrap() }

    fn start(&mut self) {
        assert!(self.child.is_none());
        std::fs::create_dir_all(self.dir()).unwrap();
        let log = std::fs::File::create(self.dir().join("server.log")).unwrap();
        self.child = Some(self.command().args(["serve", "painting"])
            .stdin(Stdio::null()).stdout(log.try_clone().unwrap()).stderr(log)
            .spawn().unwrap());
        let chunks = std::fs::read_to_string(self.log()).map(|source| source.lines().filter(|line| line.starts_with("--@ chunk ")).count()).unwrap_or(0);
        self.wait_for_chunks(chunks);
    }

    fn wait_for_chunks(&mut self, chunks: usize) {
        let started = Instant::now();
        loop {
            let output = self.cmd(&["status"]);
            if output.status.success() && String::from_utf8_lossy(&output.stdout).starts_with(&format!("{chunks} chunks")) {
                break;
            }
            let exited = self.child.as_mut().unwrap().try_wait().unwrap();
            assert!(exited.is_none(), "server exited: {}", String::from_utf8_lossy(&self.bytes("server.log")));
            assert!(started.elapsed() < Duration::from_secs(120), "server did not resume: {}", String::from_utf8_lossy(&self.bytes("server.log")));
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    fn kill(&mut self) {
        let mut child = self.child.take().unwrap();
        child.kill().unwrap();
        child.wait().unwrap();
    }

    fn close(&mut self) {
        self.ok(&["close"]);
        assert!(self.child.take().unwrap().wait().unwrap().success());
    }

    fn paint(&self) {
        for chunk in CHUNKS { self.ok(&["do", chunk]); }
    }

    fn batch(&self) {
        let output = Command::new(env!("CARGO_BIN_EXE_easel"))
            .args(["run", self.log().to_str().unwrap(), "--frames-every", "60",
                "--frames-dir", self.root.join("whole").to_str().unwrap(), "--frame-width", "1000",
                "--gallery", "--out", self.root.join("whole.png").to_str().unwrap(),
                "--checkpoint-out", self.root.join("whole.ckpt").to_str().unwrap()])
            .current_dir("/").env("RAYON_NUM_THREADS", "2")
            .env("PAINT_LIVE_CHECKPOINT_TEST_IDENTITY", self.identity)
            .env_remove("EASEL_WIDTH").env_remove("EASEL_BOX").output().unwrap();
        assert!(output.status.success(), "batch: {}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        assert!(self.bytes("live.ckpt") == std::fs::read(self.root.join("whole.ckpt")).unwrap(), "live canvas differs from canonical batch replay");
        let frames = |dir: &Path| files(dir).into_iter().filter(|(name, _)| name != "frames.states").collect::<Vec<_>>();
        assert_files(&frames(&self.dir().join("hand-frames")), &frames(&self.root.join("whole")));
        let states = |dir: &Path| std::fs::read_to_string(dir.join("frames.states")).unwrap().lines().map(|line| {
            let mut state: serde_json::Value = serde_json::from_str(line).unwrap();
            assert!(state["identity"]["source"].is_array());
            state["identity"]["source"] = serde_json::Value::Null;
            state
        }).collect::<Vec<_>>();
        assert_eq!(states(&self.dir().join("hand-frames")), states(&self.root.join("whole")));
    }
}

impl Drop for Studio {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn files(dir: &Path) -> Vec<(String, Vec<u8>)> {
    let mut files: Vec<_> = std::fs::read_dir(dir).unwrap().map(|entry| {
        let path = entry.unwrap().path();
        (path.file_name().unwrap().to_str().unwrap().to_string(), std::fs::read(&path).unwrap())
    }).collect();
    files.sort_by(|a, b| a.0.cmp(&b.0));
    files
}

fn assert_files(actual: &[(String, Vec<u8>)], expected: &[(String, Vec<u8>)]) {
    assert_eq!(actual.iter().map(|(name, _)| name).collect::<Vec<_>>(), expected.iter().map(|(name, _)| name).collect::<Vec<_>>());
    for ((name, actual), (_, expected)) in actual.iter().zip(expected) {
        assert!(actual == expected, "{name} differs");
    }
}

fn assert_tail(log: &str) {
    assert!(log.contains("reconstructed 10 chunks"), "{log}");
    let repaint: Vec<_> = log.lines().filter(|line| line.starts_with("resuming chunk ")).collect();
    assert_eq!(repaint, ["resuming chunk 11/12", "resuming chunk 12/12"], "{log}");
}

fn resume_matches_uninterrupted(tag: &str, painter: bool) {
    let mut resumed = Studio::new(tag, painter);
    resumed.start();
    let mut checkpoint = Vec::new();
    for (i, chunk) in CHUNKS.iter().enumerate() {
        resumed.ok(&["do", chunk]);
        if i == 4 {
            let frames = files(&resumed.dir().join("hand-frames"));
            let jpegs = files(&resumed.dir().join("frames"));
            let source = std::fs::read(resumed.log()).unwrap();
            let failed = resumed.cmd(&["do", "for i=1,24 do mark(900) end; error('failed painted chunk')"]);
            assert!(!failed.status.success());
            assert!(String::from_utf8_lossy(&failed.stderr).contains("failed painted chunk"));
            assert_files(&files(&resumed.dir().join("hand-frames")), &frames);
            assert_files(&files(&resumed.dir().join("frames")), &jpegs);
            assert_eq!(std::fs::read(resumed.log()).unwrap(), source);
            assert_eq!(resumed.bytes("committed.lua"), source);
        }
        if i == 7 {
            let frames = files(&resumed.dir().join("hand-frames"));
            let source = std::fs::read(resumed.log()).unwrap();
            resumed.ok(&["do", "sp=pile{{'raw umber',1}}; sb=brush('filbert',1); for i=1,24 do sb:load(sp,0.8); sb:stroke({{100,100},{102,102}}) end", "--scratch"]);
            assert_files(&files(&resumed.dir().join("hand-frames")), &frames);
            assert_eq!(std::fs::read(resumed.log()).unwrap(), source);
            assert!(!resumed.root.join("out/easel/painting-scratch/hand-frames").exists());
        }
        if i == 8 { assert!(!resumed.dir().join("replay.ckpt").exists()); }
        if i == 9 { checkpoint = resumed.bytes("replay.ckpt"); }
    }
    assert!(!checkpoint.is_empty());
    assert!(resumed.bytes("replay.ckpt") == checkpoint, "checkpoint changed before another ten commits");
    let jpegs = files(&resumed.dir().join("frames"));
    let source = std::fs::read(resumed.log()).unwrap();
    let before = String::from_utf8(resumed.bytes("hand-frames/frames.tsv")).unwrap();
    assert!(before.lines().any(|line| line.ends_with("\ttick")), "no 60-second tick: {before}");
    assert!(!before.lines().any(|line| line.ends_with("\tfinal")), "{before}");
    resumed.kill();
    resumed.start();
    assert_tail(&String::from_utf8(resumed.bytes("server.log")).unwrap());
    assert_files(&files(&resumed.dir().join("frames")), &jpegs);
    assert_eq!(std::fs::read(resumed.log()).unwrap(), source);
    resumed.close();
    let provenance = String::from_utf8(resumed.bytes("live.txt")).unwrap();
    assert!(provenance.lines().any(|line| line == "replayed 12"), "{provenance}");
    let completed = files(&resumed.dir().join("hand-frames"));
    let final_index = String::from_utf8(resumed.bytes("hand-frames/frames.tsv")).unwrap();
    assert_eq!(final_index.lines().filter(|line| line.ends_with("\tfinal")).count(), 1);
    let frame = image::open(resumed.dir().join("hand-frames/00001.png")).unwrap();
    assert_eq!((frame.width(), frame.height()), (1000, 200));
    resumed.start();
    assert!(!String::from_utf8(resumed.bytes("hand-frames/frames.tsv")).unwrap().lines().any(|line| line.ends_with("\tfinal")));
    resumed.close();
    let provenance = String::from_utf8(resumed.bytes("live.txt")).unwrap();
    assert!(provenance.lines().any(|line| line == "replayed 12"), "{provenance}");
    assert_files(&files(&resumed.dir().join("hand-frames")), &completed);

    let mut whole = Studio::new(&format!("{tag}-uninterrupted"), painter);
    whole.start();
    whole.paint();
    whole.close();
    assert_eq!(std::fs::read(whole.log()).unwrap(), source);
    assert!(resumed.bytes("live.ckpt") == whole.bytes("live.ckpt"), "resumed canvas differs from uninterrupted live session");
    assert_files(&completed, &files(&whole.dir().join("hand-frames")));
    resumed.batch();
}

#[test]
fn live_checkpoint_reconstructs_prefix_and_repaints_only_tail() {
    resume_matches_uninterrupted("developer", false);
}

#[test]
fn painter_profile_uses_the_same_binary_checkpoint_and_hand_frames() {
    resume_matches_uninterrupted("painter", true);
}

#[test]
fn invalid_checkpoint_falls_back_to_full_replay_and_matching_frames() {
    for mode in ["corrupt", "identity", "missing-frame"] {
        let mut studio = Studio::new(mode, false);
        studio.start();
        studio.paint();
        assert!(!studio.bytes("replay.ckpt").is_empty());
        studio.kill();
        if mode == "corrupt" {
            std::fs::write(studio.dir().join("replay.ckpt"), [0u8; 48]).unwrap();
        } else if mode == "missing-frame" {
            std::fs::remove_file(studio.dir().join("hand-frames/00001.png")).unwrap();
        } else {
            studio.identity = "changed";
        }
        studio.start();
        let log = String::from_utf8(studio.bytes("server.log")).unwrap();
        assert!(log.contains("replay checkpoint"), "missing fallback diagnostic: {log}");
        let reason = match mode { "corrupt" => "unsupported format", "missing-frame" => "00001.png", _ => "differs" };
        assert!(log.contains(reason), "missing {mode} fallback reason: {log}");
        if mode != "missing-frame" { assert!(!log.contains("reconstructed 10 chunks"), "{log}"); }
        let repaint: Vec<_> = log.lines().filter(|line| line.starts_with("resuming chunk ")).map(str::to_string).collect();
        assert_eq!(repaint, (1..=12).map(|n| format!("resuming chunk {n}/12")).collect::<Vec<_>>(), "{log}");
        studio.close();
        studio.batch();
    }
}

#[test]
fn an_unreplaceable_checkpoint_does_not_abort_full_replay_or_close() {
    let mut studio = Studio::new("checkpoint-directory", false);
    std::fs::create_dir_all(studio.dir().join("replay.ckpt")).unwrap();
    std::fs::write(studio.dir().join("replay.ckpt/kept"), b"not a checkpoint").unwrap();
    studio.start();
    for n in 1..=12 { studio.ok(&["do", &format!("counter = {n}")]); }
    let source = std::fs::read(studio.log()).unwrap();
    studio.kill();
    studio.start();
    let log = String::from_utf8(studio.bytes("server.log")).unwrap();
    let repaint: Vec<_> = log.lines().filter(|line| line.starts_with("resuming chunk ")).map(str::to_string).collect();
    assert_eq!(repaint, (1..=12).map(|n| format!("resuming chunk {n}/12")).collect::<Vec<_>>(), "{log}");
    assert!(log.contains("replay checkpoint"), "{log}");
    studio.close();
    assert_eq!(std::fs::read(studio.log()).unwrap(), source);
    assert_eq!(studio.bytes("committed.lua"), source);
    assert_eq!(studio.bytes("replay.ckpt/kept"), b"not a checkpoint");
}

fn stale_rebuild_after_executable_replacement(state_store: bool) {
    use std::os::unix::fs::MetadataExt;

    let mut studio = Studio::new(if state_store { "rebuild-state-store" } else { "rebuild-developer" }, true);
    std::fs::write(studio.root.join("bin/studio"), "developer\n").unwrap();
    studio.state_store = state_store;
    studio.start();
    studio.ok(&["do", "t = {}; for i=1,30 do t['k' .. i] = i end; t[2] = 2"]);
    let source = std::fs::read(studio.log()).unwrap();
    let revision = state_store.then(|| studio.bytes("state/CURRENT"));
    let old_inode = std::fs::metadata(&studio.binary).unwrap().ino();
    let replacement = studio.binary.with_extension("replacement");
    std::fs::copy(env!("CARGO_BIN_EXE_easel"), &replacement).unwrap();
    std::fs::rename(replacement, &studio.binary).unwrap();
    assert_ne!(std::fs::metadata(&studio.binary).unwrap().ino(), old_inode);
    let failed = studio.cmd(&["do", "for i=1,1000 do t['x' .. i] = i end; for i=1,1000 do t['x' .. i] = nil end; error('force stale rebuild')"]);
    assert!(!failed.status.success());
    let error = String::from_utf8_lossy(&failed.stderr);
    assert!(error.contains("force stale rebuild") && error.contains("rebuild"), "{error}");
    studio.wait_for_chunks(1);
    let log = String::from_utf8(studio.bytes("server.log")).unwrap();
    assert!(log.contains("rebuilding from the log (1 chunks)") && log.lines().any(|line| line.starts_with("rebuilt ")), "{log}");
    assert_eq!(std::fs::read(studio.log()).unwrap(), source);
    assert_eq!(studio.bytes("committed.lua"), source);
    if let Some(revision) = revision { assert_eq!(studio.bytes("state/CURRENT"), revision); }
    studio.ok(&["do", "local n=0; for _ in pairs(t) do n=n+1 end; assert(n == 31); assert(t[2] == 2 and t.k30 == 30 and t.x1 == nil)"]);
    assert!(studio.ok(&["status"]).starts_with("2 chunks"));
    studio.close();
}

#[test]
fn stale_rebuild_survives_executable_replacement() {
    stale_rebuild_after_executable_replacement(false);
}

#[cfg(feature = "state-store")]
#[test]
fn stale_rebuild_reuses_the_already_locked_state_store_after_executable_replacement() {
    stale_rebuild_after_executable_replacement(true);
}
