
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

struct Temp(PathBuf);

impl Temp {
    fn new(name: &str) -> Self {
        let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("checkpoint-paths-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn file(&self, name: &str) -> String {
        self.0.join(name).to_str().unwrap().to_string()
    }

    fn source(&self, name: &str, failing: bool) -> String {
        let source = self.file(name);
        let program = "--@ engine 5\n--@ chunk 1\ncanvas{size=440, aspect=1.25, linen=15, seed=7, ground={{pile={{\"lead white\", 3}, {\"yellow ochre\", 1}}, um=120, apply=\"knife\"}}}\n";
        std::fs::write(&source, format!("{program}{}", if failing { "--@ chunk 2\nerror('failed chunk')\n" } else { "" })).unwrap();
        source
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn finish(checkpoint: &str, out: &str, source: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_easel"))
        .args(["finish", checkpoint, out, "--log", source, "--no-varnish", "--no-cracks"])
        .env("RAYON_NUM_THREADS", "2")
        .output().unwrap()
}

#[test]
fn finishing_refuses_source_and_checkpoint_output_aliases() {
    use std::os::unix::fs::symlink;
    let temp = Temp::new("finish-aliases");
    let source = temp.source("painting.lua", false);
    let checkpoint = temp.file("live.ckpt");
    let out = temp.file("out.png");
    let result = run(&source, &out, &checkpoint, &[]);
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    std::fs::remove_file(&out).unwrap();
    for victim in [&source, &checkpoint] {
        for hard_link in [false, true] {
            let original = std::fs::read(victim).unwrap();
            if hard_link { std::fs::hard_link(victim, &out).unwrap(); }
            else { symlink(victim, &out).unwrap(); }
            let result = finish(&checkpoint, &out, &source);
            assert_eq!(std::fs::read(victim).unwrap(), original, "finish changed {victim}");
            assert!(!result.status.success());
            assert!(String::from_utf8_lossy(&result.stderr).contains("must differ"));
            std::fs::remove_file(&out).unwrap();
        }
    }
    let result = finish(&checkpoint, &out, &source);
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert!(std::fs::read(&out).unwrap().starts_with(b"\x89PNG"));
}

#[test]
fn finishing_wrapper_preserves_sources_for_part_and_final_output_aliases() {
    use std::os::unix::fs::symlink;
    let temp = Temp::new("finish-wrapper");
    let scripts = temp.0.join("scripts");
    std::fs::create_dir_all(&scripts).unwrap();
    std::fs::copy(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scripts/finish_painting"), scripts.join("finish_painting")).unwrap();
    std::fs::write(scripts.join("replay_easel"), format!("#!/usr/bin/env bash\nprintf '%s\\n' '{}'\n", env!("CARGO_BIN_EXE_easel"))).unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(scripts.join("replay_easel"), std::fs::Permissions::from_mode(0o755)).unwrap();
    let logs = temp.0.join("studio/paintings/lua");
    std::fs::create_dir_all(&logs).unwrap();
    let source = temp.source("studio/paintings/lua/painting.lua", false);
    let original = std::fs::read(&source).unwrap();
    let checkpoint = temp.file("studio/out/easel/painting/live.ckpt");
    let out = temp.file("out.png");
    let result = run(&source, &out, &checkpoint, &[]);
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    std::fs::remove_file(&out).unwrap();
    let saved = std::fs::read(&checkpoint).unwrap();
    let invoke = || Command::new("bash").arg(scripts.join("finish_painting"))
        .args([&source, &out, "--no-varnish", "--no-cracks"])
        .env("RAYON_NUM_THREADS", "2").env_remove("EASEL_WIDTH").output().unwrap();
    for victim in [&source, &checkpoint] {
        for destination in [&out, &temp.file("out.lua")] {
            for hard_link in [false, true] {
                if hard_link { std::fs::hard_link(victim, destination).unwrap(); }
                else { symlink(victim, destination).unwrap(); }
                let result = invoke();
                assert_eq!(std::fs::read(&source).unwrap(), original);
                assert_eq!(std::fs::read(&checkpoint).unwrap(), saved);
                assert!(!result.status.success());
                std::fs::remove_file(destination).unwrap();
                assert!(!Path::new(&out).exists());
                assert!(!temp.0.join("out.lua").exists());
            }
        }
    }
    std::fs::hard_link(&source, temp.0.join("out.lua.part")).unwrap();
    let result = invoke();
    assert_eq!(std::fs::read(&source).unwrap(), original);
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert_eq!(std::fs::read(temp.0.join("out.lua.part")).unwrap(), original);
    assert_eq!(std::fs::read(temp.0.join("out.lua")).unwrap(), original);
    assert!(std::fs::read(&out).unwrap().starts_with(b"\x89PNG"));
}

fn run(source: &str, out: &str, checkpoint: &str, extra: &[String]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_easel")).env_remove("EASEL_WIDTH")
        .args(["run", source, "--width", "32", "--out", out, "--checkpoint-out", checkpoint])
        .args(extra)
        .env("RAYON_NUM_THREADS", "2")
        .output()
        .unwrap()
}

#[test]
fn every_output_alias_is_refused_before_opening_outputs_or_running_chunks() {
    let temp = Temp::new("all-outputs");
    let source = temp.source("failed.lua", true);
    let out = temp.file("out.png");
    let accepted = temp.file("accepted.ckpt");
    let original = b"previous accepted checkpoint";
    for flag in ["--state-digest", "--dump-surface"] {
        std::fs::write(&accepted, original).unwrap();
        let result = run(&source, &out, &accepted, &[flag.into(), accepted.clone()]);
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("must differ"));
        assert_eq!(std::fs::read(&accepted).unwrap(), original);
        assert!(!Path::new(&out).exists());
    }
    let result = run(&source, &accepted, &accepted, &[]);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("must differ"));
    assert_eq!(std::fs::read(&accepted).unwrap(), original);
    std::fs::remove_file(&accepted).unwrap();
    let look = temp.file("out.look.png");
    std::fs::write(&look, original).unwrap();
    let result = run(&source, &out, &look, &["--look".into()]);
    assert!(!result.status.success());
    assert_eq!(std::fs::read(&look).unwrap(), original);
    assert!(!Path::new(&out).exists());
    for (flag, filename) in [("--dump-state", "chunk-001.state"), ("--frames-dir", "frames.tsv")] {
        let dir = temp.0.join(flag.trim_start_matches('-'));
        std::fs::create_dir(&dir).unwrap();
        let checkpoint = dir.join(filename);
        std::fs::write(&checkpoint, original).unwrap();
        let result = run(&source, &out, checkpoint.to_str().unwrap(), &[flag.into(), dir.to_str().unwrap().into()]);
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("must be outside"));
        assert_eq!(std::fs::read(&checkpoint).unwrap(), original);
        std::fs::hard_link(&checkpoint, &accepted).unwrap();
        let result = run(&source, &out, &accepted, &[flag.into(), dir.to_str().unwrap().into()]);
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("must not alias"));
        std::fs::remove_file(&accepted).unwrap();
    }
}

#[test]
fn nonexistent_destinations_resolve_dotdot_and_symlinks_before_comparison() {
    use std::os::unix::fs::symlink;
    let temp = Temp::new("normalization");
    let source = temp.source("painting.lua", false);
    let out = temp.file("new.png");
    let checkpoint = temp.file("sub/../new.png");
    let result = run(&source, &out, &checkpoint, &[]);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("must differ"));
    assert!(!Path::new(&out).exists());
    assert!(!temp.0.join("sub").exists());
    let alias = temp.file("alias.png");
    symlink("new.png", &alias).unwrap();
    let result = run(&source, &out, &alias, &[]);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("must differ"));
    assert!(!Path::new(&out).exists());
    std::fs::create_dir(temp.0.join("real")).unwrap();
    std::fs::create_dir(temp.0.join("real/child")).unwrap();
    symlink("real/child", temp.0.join("linked")).unwrap();
    let real_out = temp.file("real/new.png");
    let linked = temp.file("linked/../new.png");
    let result = run(&source, &real_out, &linked, &[]);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("must differ"));
    assert!(!Path::new(&real_out).exists());
}

#[test]
fn exclusive_temp_save_preserves_part_sources_and_accepted_state_on_write_failure() {
    let temp = Temp::new("exclusive-temp");
    let source = temp.source("final.ckpt.part", false);
    let original_source = std::fs::read(&source).unwrap();
    let out = temp.file("out.png");
    let checkpoint = temp.file("final.ckpt");
    let result = run(&source, &out, &checkpoint, &[]);
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert_eq!(std::fs::read(&source).unwrap(), original_source);
    let original = std::fs::read(&checkpoint).unwrap();
    assert!(original.len() > 10240);
    let accepted = temp.file("accepted.ckpt");
    let part = temp.file("accepted.ckpt.part");
    std::fs::write(&accepted, &original).unwrap();
    std::fs::hard_link(&accepted, &part).unwrap();
    let before: std::collections::BTreeSet<_> = std::fs::read_dir(&temp.0).unwrap().map(|entry| entry.unwrap().file_name()).collect();
    let result = Command::new("bash")
        .args(["-c", "trap '' XFSZ; ulimit -f 10; exec \"$@\"", "checkpoint-file-limit", env!("CARGO_BIN_EXE_easel"), "run", &source, "--width", "32", "--out", &out, "--checkpoint-out", &accepted])
        .env("RAYON_NUM_THREADS", "2")
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("accepted.ckpt"), "{}", String::from_utf8_lossy(&result.stderr));
    assert_eq!(std::fs::read(&accepted).unwrap(), original);
    assert_eq!(std::fs::read(&part).unwrap(), original);
    assert_eq!(std::fs::read(&source).unwrap(), original_source);
    let after: std::collections::BTreeSet<_> = std::fs::read_dir(&temp.0).unwrap().map(|entry| entry.unwrap().file_name()).collect();
    assert_eq!(before, after, "a failed save left a temporary file");
}

fn replay_source(temp: &Temp) {
    let source = temp.source("painting.lua", false);
    let mut text = std::fs::read_to_string(&source).unwrap();
    text.push_str("--@ chunk 2\ncounter = 2\n--@ chunk 3\ncounter = 3\n");
    std::fs::write(source, text).unwrap();
    std::fs::write(temp.0.join("out.png"), b"previous picture").unwrap();
}

fn replay(temp: &Temp, extra: &[String]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_easel"));
    command.args(["run", &temp.file("painting.lua"), "--width", "32"]);
    if !extra.iter().any(|a| a == "--out") { command.args(["--out", &temp.file("out.png")]); }
    command.args(extra).env("RAYON_NUM_THREADS", "2").env_remove("EASEL_WIDTH").env_remove("EASEL_BOX").output().unwrap()
}

fn snapshot(dir: &Path) -> std::collections::BTreeMap<PathBuf, (Option<PathBuf>, Option<Vec<u8>>)> {
    let mut files = std::collections::BTreeMap::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        let kind = std::fs::symlink_metadata(&path).unwrap().file_type();
        files.insert(path.clone(), (kind.is_symlink().then(|| std::fs::read_link(&path).unwrap()), std::fs::read(&path).ok()));
        if kind.is_dir() { files.extend(snapshot(&path)); }
    }
    files
}

fn reject_unchanged(temp: &Temp, args: &[String]) {
    let before = snapshot(&temp.0);
    let result = replay(temp, args);
    assert!(!result.status.success(), "accepted {args:?}");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(stderr.contains("must differ") || stderr.contains("must be outside") || stderr.contains("must not alias") || stderr.contains("must be separate"), "{stderr}");
    assert!(!stderr.contains("reconstructed"), "{stderr}");
    assert_eq!(snapshot(&temp.0), before, "rejected {args:?} changed an input or artifact");
}

#[test]
fn replay_outputs_refuse_direct_symlink_and_hardlink_source_aliases() {
    use std::os::unix::fs::symlink;
    let temp = Temp::new("replay-source");
    replay_source(&temp);
    for option in ["--state-digest", "--out", "--dump-surface", "--checkpoint-out", "--replay-checkpoint"] {
        reject_unchanged(&temp, &[option.into(), temp.file("painting.lua")]);
        for hard in [false, true] {
            let alias = temp.file("alias.png");
            if hard { std::fs::hard_link(temp.file("painting.lua"), &alias).unwrap(); }
            else { symlink("painting.lua", &alias).unwrap(); }
            reject_unchanged(&temp, &[option.into(), alias.clone()]);
            std::fs::remove_file(alias).unwrap();
        }
    }
    for hard in [false, true] {
        let look = temp.file("out.look.png");
        if hard { std::fs::hard_link(temp.file("painting.lua"), &look).unwrap(); }
        else { symlink("painting.lua", &look).unwrap(); }
        reject_unchanged(&temp, &["--look".into()]);
        std::fs::remove_file(look).unwrap();
    }
}

#[test]
fn replay_frame_and_dump_files_cannot_alias_source() {
    use std::os::unix::fs::symlink;
    let temp = Temp::new("replay-generated-source");
    replay_source(&temp);
    for (option, names) in [("--frames-dir", vec!["frames.tsv", "frames.states", "00001.png"]), ("--dump-state", vec!["chunk-001.state", "final.ckpt"])] {
        let dir = temp.0.join("generated");
        std::fs::create_dir(&dir).unwrap();
        for name in names {
            for hard in [false, true] {
                let alias = dir.join(name);
                if hard { std::fs::hard_link(temp.file("painting.lua"), &alias).unwrap(); }
                else { symlink(temp.file("painting.lua"), &alias).unwrap(); }
                let mut args = vec![option.into(), dir.display().to_string()];
                if option == "--frames-dir" { args.extend(["--frames-every".into(), "1".into()]); }
                reject_unchanged(&temp, &args);
                std::fs::remove_file(alias).unwrap();
            }
        }
        std::fs::remove_dir(dir).unwrap();
    }
}

#[test]
fn replay_checkpoint_and_progress_stay_outside_frame_outputs_before_first_batch() {
    use std::os::unix::fs::symlink;
    let temp = Temp::new("replay-frame-checkpoint");
    replay_source(&temp);
    let frames = temp.file("frames");
    for name in ["frames.states", "frames.tsv", "00001.png", "sub/../00002.png"] {
        reject_unchanged(&temp, &["--frames-dir".into(), frames.clone(), "--frames-every".into(), "1".into(), "--replay-checkpoint".into(), temp.file(&format!("frames/{name}")), "--until-chunk".into(), "2".into()]);
    }
    std::fs::create_dir(&frames).unwrap();
    symlink("../out.replay-progress.json", temp.file("frames/frames.tsv")).unwrap();
    reject_unchanged(&temp, &["--frames-dir".into(), frames, "--frames-every".into(), "1".into()]);
}

#[test]
fn replay_rejects_output_collisions_and_directory_overlap_without_creating_paths() {
    use std::os::unix::fs::symlink;
    let temp = Temp::new("replay-output-collisions");
    replay_source(&temp);
    let frames = temp.file("frames");
    for option in ["--out", "--state-digest", "--dump-surface", "--checkpoint-out"] {
        reject_unchanged(&temp, &["--frames-dir".into(), frames.clone(), "--frames-every".into(), "1".into(), option.into(), temp.file("frames/00002.png")]);
    }
    reject_unchanged(&temp, &["--frames-dir".into(), frames.clone(), "--frames-every".into(), "1".into(), "--dump-state".into(), temp.file("frames/dumps")]);
    reject_unchanged(&temp, &["--state-digest".into(), temp.file("new.png"), "--dump-surface".into(), temp.file("sub/../new.png")]);
    reject_unchanged(&temp, &["--state-digest".into(), temp.file("new"), "--dump-surface".into(), temp.file("new/surface")]);
    reject_unchanged(&temp, &["--state-digest".into(), temp.file("new/digest"), "--dump-surface".into(), temp.file("new")]);
    reject_unchanged(&temp, &["--look".into(), "--state-digest".into(), temp.file("out.look.png")]);
    for hard in [false, true] {
        let alias = temp.file("surface");
        if hard { std::fs::hard_link(temp.file("out.png"), &alias).unwrap(); }
        else { symlink("out.png", &alias).unwrap(); }
        reject_unchanged(&temp, &["--dump-surface".into(), alias.clone()]);
        std::fs::remove_file(alias).unwrap();
    }
    symlink("new.png", temp.file("alias.png")).unwrap();
    reject_unchanged(&temp, &["--state-digest".into(), temp.file("new.png"), "--dump-surface".into(), temp.file("alias.png")]);
    std::fs::create_dir(&frames).unwrap();
    symlink("00002.png", temp.file("frames/frames.states")).unwrap();
    reject_unchanged(&temp, &["--frames-dir".into(), frames, "--frames-every".into(), "1".into()]);
}

#[test]
fn replay_preflight_preserves_a_valid_paused_batch_and_resume() {
    use std::os::unix::fs::symlink;
    let temp = Temp::new("replay-paused-alias");
    replay_source(&temp);
    let args = vec!["--frames-dir".into(), temp.file("frames"), "--frames-every".into(), "1".into(), "--frame-width".into(), "32".into(), "--replay-checkpoint".into(), temp.file("replay.ckpt"), "--state-digest".into(), temp.file("digests.tsv"), "--checkpoint-out".into(), temp.file("final.ckpt")];
    let mut paused = args.clone();
    paused.extend(["--until-chunk".into(), "2".into()]);
    let result = replay(&temp, &paused);
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    for hard in [false, true] {
        let alias = temp.file("frames/00003.png");
        if hard { std::fs::hard_link(temp.file("digests.tsv"), &alias).unwrap(); }
        else { symlink("../digests.tsv", &alias).unwrap(); }
        reject_unchanged(&temp, &args);
        std::fs::remove_file(alias).unwrap();
    }
    let result = replay(&temp, &args);
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert!(std::fs::read(temp.file("out.png")).unwrap().starts_with(b"\x89PNG"));
    assert!(Path::new(&temp.file("final.ckpt")).exists());
    assert_eq!(std::fs::read_to_string(temp.file("digests.tsv")).unwrap().lines().count(), 3);
}
