//! The painting and the scratch canvas take turns in memory (session.rs `rest`): the one
//! the painter isn't using is packed away on disk, and comes back bit for bit.
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

const CANVAS: &str = "canvas{size=300, aspect=5, linen={16,14}, seed=3, ground={{pile={{\"lead white\",6},{\"raw umber\",1}}, um=50, apply=\"roller\"}}}";
const PREPARE: &str = "p=pile{{\"lead white\",6},{\"cobalt blue\",1}}; b=brush(\"filbert\",8); b:load(p,0.9)";
const MARK: &str = "b:stroke({{100,100},{800,130}}, {pressure=0.8})";

struct Studio {
    root: PathBuf,
    child: Option<Child>,
}

impl Studio {
    fn new(name: &str) -> Self {
        let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("scratch-memory-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("out/easel/painting")).unwrap();
        let root = root.canonicalize().unwrap();
        let mut studio = Self { root, child: None };
        let log = std::fs::File::create(studio.root.join("out/easel/painting/server.log")).unwrap();
        studio.child = Some(studio.command().args(["serve", "painting"]).stdin(Stdio::null()).stdout(log.try_clone().unwrap()).stderr(log).spawn().unwrap());
        let started = Instant::now();
        loop {
            let output = studio.cmd(&["status"]);
            if output.status.success() && String::from_utf8_lossy(&output.stdout).starts_with("0 chunks") {
                break;
            }
            assert!(studio.child.as_mut().unwrap().try_wait().unwrap().is_none(), "server exited: {}", studio.server_log());
            assert!(started.elapsed() < Duration::from_secs(120), "server did not start: {}", studio.server_log());
            std::thread::sleep(Duration::from_millis(20));
        }
        studio
    }

    fn command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_easel"));
        command.current_dir("/").env("EASEL_ROOT", &self.root).env("EASEL_SESSION", "painting").env("RAYON_NUM_THREADS", "2")
            .env_remove("EASEL_WIDTH").env_remove("EASEL_BOX").env_remove("EASEL_STATE_SESSION").env_remove("EASEL_STATE_DISK_MIB").env_remove("EASEL_LOCK_ON_STDIN");
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

    fn server_log(&self) -> String {
        std::fs::read_to_string(self.root.join("out/easel/painting/server.log")).unwrap_or_default()
    }

    /// The server's resident set, KiB.
    fn rss(&self) -> usize {
        let pid = self.child.as_ref().unwrap().id().to_string();
        let output = Command::new(if cfg!(target_os = "macos") { "/bin/ps" } else { "ps" }).args(["-o", "rss=", "-p", &pid]).output().unwrap();
        assert!(output.status.success());
        String::from_utf8_lossy(&output.stdout).trim().parse().unwrap()
    }

    fn close(&mut self) {
        self.ok(&["close"]);
        assert!(self.child.take().unwrap().wait().unwrap().success(), "{}", self.server_log());
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

/// A sitting with a scratch canvas holds one canvas in memory, not two: while the scratch
/// canvas is painted on the painting rests on disk, and the other way round. The server's
/// resident set with the scratch canvas up stays within half a canvas of the painting alone.
#[test]
fn the_painting_and_the_scratch_canvas_take_turns_in_memory() {
    let mut studio = Studio::new("turns");
    let base = studio.rss();
    studio.ok(&["do", CANVAS]);
    studio.ok(&["do", PREPARE]);
    studio.ok(&["do", MARK]);
    let painting = studio.rss() - base;
    let within = |rss: usize, what: &str| assert!(rss - base < painting * 3 / 2, "{what}: {} KiB resident beside the painting's {painting} KiB (base {base} KiB)", rss - base);
    studio.ok(&["do", "--scratch", PREPARE]);
    studio.ok(&["do", "--scratch", MARK]);
    within(studio.rss(), "the scratch canvas up");
    assert!(studio.ok(&["look", "--scratch"]).contains("scratch"));
    assert!(studio.ok(&["look"]).contains("painting"));
    assert!(studio.ok(&["status"]).starts_with("3 chunks"));
    studio.ok(&["do", MARK]);
    within(studio.rss(), "back on the painting");
    studio.ok(&["do", "--scratch", "--new", PREPARE]);
    studio.ok(&["do", "--scratch", MARK]);
    within(studio.rss(), "a fresh scratch canvas up");
    // the fresh scratch canvas's own chunks: its canvas, then the two painted on it
    assert!(studio.ok(&["status", "--scratch"]).starts_with("3 chunks"));
    studio.ok(&["do", MARK]);
    assert!(studio.ok(&["status"]).starts_with("5 chunks"));
    studio.close();
    // the painting, rested and woken between its chunks, is what its log replays to, bit for bit
    let log = studio.root.join("paintings/lua/painting.lua");
    let replayed = studio.root.join("replayed.ckpt");
    let output = Command::new(env!("CARGO_BIN_EXE_easel")).args(["run", log.to_str().unwrap(), "--checkpoint-out", replayed.to_str().unwrap()])
        .current_dir("/").env("RAYON_NUM_THREADS", "2").env_remove("EASEL_WIDTH").env_remove("EASEL_BOX").output().unwrap();
    assert!(output.status.success(), "replay: {}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    assert!(std::fs::read(studio.root.join("out/easel/painting/live.ckpt")).unwrap() == std::fs::read(&replayed).unwrap(), "the live painting differs from its log's replay");
}
