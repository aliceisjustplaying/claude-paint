#![cfg(all(feature = "replay", feature = "state-store"))]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const CANVAS: &str = r#"canvas{size=440, aspect=1.25, linen=15, seed=7, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=120, apply="knife"}}}"#;

struct Temp(PathBuf);

impl Temp {
    fn new(name: &str) -> Self {
        let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("committed-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn file(&self, name: &str) -> String { self.0.join(name).to_str().unwrap().to_string() }
    fn command(&self, args: &[&str], name: Option<&str>, budget: Option<&str>) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_easel"));
        command.args(args).env("EASEL_ROOT", &self.0).env("RAYON_NUM_THREADS", "2").env_remove("EASEL_STATE_SESSION").env_remove("EASEL_STATE_DISK_MIB").env_remove("EASEL_SESSION");
        if let Some(name) = name { command.env("EASEL_STATE_SESSION", name); }
        if let Some(budget) = budget { command.env("EASEL_STATE_DISK_MIB", budget); }
        command.output().unwrap()
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        for name in ["sketch-durable", "sketch-budget", "sketch-default"] {
            let path = self.0.join(format!("out/easel/{name}/lock"));
            if let (Ok(file), Ok(pid)) = (std::fs::OpenOptions::new().read(true).write(true).open(&path), std::fs::read_to_string(&path)) {
                if matches!(file.try_lock(), Err(std::fs::TryLockError::WouldBlock)) && pid.trim().parse::<u32>().is_ok() { let _ = Command::new("kill").args(["-KILL", pid.trim()]).status(); }
            }
        }
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn success(output: Output) -> String {
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).unwrap()
}

fn current(dir: &Path) -> (String, serde_json::Value) {
    let id = std::fs::read_to_string(dir.join("CURRENT")).unwrap().trim().to_string();
    let manifest = serde_json::from_slice(&std::fs::read(dir.join("revisions").join(format!("{id}.json"))).unwrap()).unwrap();
    (id, manifest)
}

#[test]
fn publication_does_not_change_replay_pixels_or_chunk_digests_in_any_legacy_engine() {
    let temp = Temp::new("parity");
    for engine in 1..=paint::ENGINE {
        let source = temp.file("painting.lua");
        let program = format!(r#"--@ engine {engine}
--@ chunk 1
{CANVAS}
items={{}}
for i=1,8 do items[{{index=i}}]=i end
--@ chunk 2
for k,v in pairs(items) do
work(rect(v*90, 0, 55, 600), {{hand="broad", pile=pile{{{{"lead white", 3}}, {{"cobalt blue", 1}}}}, coverage=2}})
end
--@ chunk 3
wait(30)
"#);
        std::fs::write(&source, program).unwrap();
        let plain = temp.file(&format!("plain-{engine}.png"));
        let stored = temp.file(&format!("stored-{engine}.png"));
        let a = temp.file("plain.dig");
        let b = temp.file("stored.dig");
        let store = temp.file(&format!("state-{engine}"));
        success(temp.command(&["run", &source, "--width", "32", "--out", &plain, "--state-digest", &a], None, None));
        success(temp.command(&["run", &source, "--width", "32", "--out", &stored, "--state-digest", &b, "--state-store", &store], None, None));
        assert_eq!(std::fs::read(&plain).unwrap(), std::fs::read(&stored).unwrap(), "engine {engine}");
        let digests = |path: &str| std::fs::read_to_string(path).unwrap().lines().map(|line| line.split_whitespace().filter(|p| !p.starts_with("secs=")).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>();
        assert_eq!(digests(&a), digests(&b), "engine {engine}");
        let rendered = temp.file("rendered.png");
        success(temp.command(&["render-state", &store, &rendered, "--mode", "color"], None, None));
        assert_eq!(std::fs::read(&plain).unwrap(), std::fs::read(rendered).unwrap());
        assert_eq!(current(Path::new(&store)).1["chunks"], 3);
        assert_eq!(std::fs::read_dir(Path::new(&store).join("revisions")).unwrap().count(), 2);
    }
}

#[test]
fn failed_chunk_leaves_the_last_successful_revision_renderable_without_a_log() {
    let temp = Temp::new("failed-chunk");
    let source = temp.file("failed.lua");
    std::fs::write(&source, format!("--@ engine 5\n--@ chunk 1\n{CANVAS}\n--@ chunk 2\nwait(1)\n--@ chunk 3\nerror('failed chunk')\n")).unwrap();
    let store = temp.file("state");
    let out = temp.file("out.png");
    let result = temp.command(&["run", &source, "--width", "32", "--out", &out, "--state-store", &store], None, None);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("chunk 3 failed"));
    assert!(!Path::new(&out).exists());
    assert_eq!(current(Path::new(&store)).1["chunks"], 2);
    std::fs::remove_file(&source).unwrap();
    for mode in ["color", "gallery", "relief"] {
        success(temp.command(&["render-state", &store, &out, "--mode", mode], None, None));
    }
}

#[test]
fn live_chunks_survive_abrupt_server_exit_and_repair_only_known_source_exports() {
    let temp = Temp::new("live");
    let name = "sketch-durable";
    success(temp.command(&["open", name], Some(name), None));
    success(temp.command(&["-s", name, "do", CANVAS], Some(name), None));
    success(temp.command(&["-s", name, "do", "wait(1)"], Some(name), None));
    let dir = temp.0.join(format!("out/easel/{name}"));
    let state = dir.join("state");
    let (id, manifest) = current(&state);
    assert_eq!(manifest["chunks"], 2);
    assert!(!dir.join("live.ckpt").exists());
    let pid = std::fs::read_to_string(dir.join("lock")).unwrap();
    assert!(Command::new("kill").args(["-KILL", pid.trim()]).status().unwrap().success());
    let log = temp.0.join(format!("paintings/lua/{name}.lua"));
    let source = std::fs::read(&log).unwrap();
    std::fs::write(&log, &source[..source.len()/2]).unwrap();
    std::fs::remove_file(dir.join("committed.lua")).unwrap();
    let rendered = temp.file("committed.png");
    success(temp.command(&["render-state", state.to_str().unwrap(), &rendered, "--mode", "color"], None, None));
    success(temp.command(&["open", name], Some(name), None));
    assert_eq!(std::fs::read(&log).unwrap(), source);
    assert_eq!(std::fs::read(dir.join("committed.lua")).unwrap(), source);
    assert_eq!(current(&state).0, id);
    success(temp.command(&["-s", name, "do", "wait(2)"], Some(name), None));
    success(temp.command(&["-s", name, "close"], Some(name), None));
    let accepted = current(&state).0;
    std::fs::write(&log, b"error('untrusted replacement')").unwrap();
    let result = temp.command(&["open", name], Some(name), None);
    assert!(!result.status.success());
    assert_eq!(current(&state).0, accepted);
}

#[test]
fn budget_failure_discards_unaccepted_vm_state_and_default_sessions_still_use_legacy_saves() {
    let temp = Temp::new("budget");
    let name = "sketch-budget";
    success(temp.command(&["open", name], Some(name), Some("1")));
    let state = temp.0.join(format!("out/easel/{name}/state"));
    let accepted = current(&state).0;
    let result = temp.command(&["-s", name, "do", CANVAS], Some(name), Some("1"));
    assert!(!result.status.success());
    assert_eq!(current(&state).0, accepted);
    assert_eq!(current(&state).1["chunks"], 0);
    success(temp.command(&["open", name], Some(name), Some("1")));
    success(temp.command(&["-s", name, "close"], Some(name), Some("1")));
    let name = "sketch-default";
    success(temp.command(&["open", name], None, None));
    success(temp.command(&["-s", name, "do", CANVAS], None, None));
    success(temp.command(&["-s", name, "close"], None, None));
    let dir = temp.0.join(format!("out/easel/{name}"));
    assert!(!dir.join("state").exists());
    assert!(dir.join("live.ckpt").exists());
}

#[test]
fn store_namespace_cannot_be_used_as_a_source_or_render_output() {
    let temp = Temp::new("aliases");
    let source = temp.file("painting.lua");
    std::fs::write(&source, format!("--@ engine 5\n--@ chunk 1\n{CANVAS}\n")).unwrap();
    let state = temp.file("state");
    let out = temp.file("state/inside.png");
    assert!(!temp.command(&["run", &source, "--width", "32", "--out", &out, "--state-store", &state], None, None).status.success());
    let out = temp.file("outside.png");
    success(temp.command(&["run", &source, "--width", "32", "--out", &out, "--state-store", &state], None, None));
    let pointer = temp.file("state/CURRENT");
    let previous = std::fs::read(&pointer).unwrap();
    assert!(!temp.command(&["render-state", &state, &pointer], None, None).status.success());
    assert_eq!(std::fs::read(pointer).unwrap(), previous);
}
