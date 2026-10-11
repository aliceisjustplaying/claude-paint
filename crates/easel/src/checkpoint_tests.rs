use super::*;

struct Temp(PathBuf);

impl Temp {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("easel-checkpoint-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn file(&self, name: &str) -> String {
        self.0.join(name).to_str().unwrap().to_string()
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn program(engine: u32) -> String {
    format!(r#"--@ engine {engine}
--@ chunk 1
canvas{{size=440, aspect=1.25, linen=15, seed=7, ground={{{{pile={{{{"lead white", 3}}, {{"yellow ochre", 1}}}}, um=120, apply="knife"}}}}}}
continuation = {{answer=42}}
--@ chunk 2
p = pile{{{{"lead white", 4}}, {{"cobalt blue", 1}}, medium=0.3}}
work(rect(0, 0, 650, 300), {{hand="broad", pile=p, coverage=2}})
wait(30)
"#)
}

#[test]
fn checkpoint_display_versions_keep_color_and_legacy_lighting_exact() {
    let temp = Temp::new("look-versions");
    let text = program(paint::ENGINE);
    let mut s = Session::replay_with(64, session::box_for(Some(&text)).unwrap()).unwrap();
    for chunk in parse_program(&text) {
        s.run(&chunk).unwrap();
    }
    let checkpoint = temp.file("saved.ckpt");
    save::write(&s, &text, Path::new(&checkpoint)).unwrap();
    let before = std::fs::read(&checkpoint).unwrap();
    let color1 = temp.file("color1.png");
    let color2 = temp.file("color2.png");
    for (version, out) in [("1", &color1), ("2", &color2)] {
        render_checkpoint(&[checkpoint.clone(), out.clone(), "--mode".into(), "color".into(), "--look-version".into(), version.into()]).unwrap();
    }
    assert_eq!(std::fs::read(color1).unwrap(), std::fs::read(color2).unwrap());
    let c = s.canvas().unwrap();
    for mode in ["gallery", "relief", "museum-flat", "museum", "museum-surface"] {
        let out = temp.file("legacy.png");
        render_checkpoint(&[checkpoint.clone(), out.clone(), "--mode".into(), mode.into(), "--look-version".into(), "1".into()]).unwrap();
        let view = look::View::parse(&["--mode".into(), mode.into()]).unwrap();
        let pixels = match (view.museum, view.light) {
            (Some(contrast), _) => c.seen_museum(contrast),
            (_, Some((az, el))) => c.seen_lit(az, el, 1.0),
            _ => unreachable!(),
        };
        let expected = temp.file("expected.png");
        deliver_seen(&c, Path::new(&expected), &pixels).unwrap();
        assert_eq!(std::fs::read(out).unwrap(), std::fs::read(expected).unwrap(), "legacy {mode}");
    }
    assert_eq!(std::fs::read(checkpoint).unwrap(), before);
    assert!(look::View::parse(&["--look-version".into(), "3".into()]).is_err());
}

#[test]
fn display_export_versions_pin_default_run_and_process_frames_to_legacy() {
    let temp = Temp::new("export-versions");
    let source = temp.file("painting.lua");
    std::fs::write(&source, program(paint::ENGINE)).unwrap();
    let mut outputs = Vec::new();
    let mut final_frames = Vec::new();
    let mut states = Vec::new();
    for (label, version) in [("default", None), ("legacy", Some("1")), ("gloss", Some("2"))] {
        let out = temp.file(&format!("{label}.png"));
        let state = temp.file(&format!("{label}.ckpt"));
        let frames_dir = temp.file(&format!("frames-{label}"));
        let mut args = vec![source.clone(), "--width".into(), "64".into(), "--out".into(), out.clone(), "--gallery".into(), "--checkpoint-out".into(), state.clone(), "--frames-every".into(), "60".into(), "--frames-dir".into(), frames_dir.clone(), "--frame-width".into(), "64".into()];
        if let Some(version) = version {
            args.extend(["--look-version".into(), version.into()]);
        }
        run(&args).unwrap();
        let index = std::fs::read_to_string(Path::new(&frames_dir).join("frames.tsv")).unwrap();
        assert!(index.lines().next().unwrap().ends_with("\tlook_version"));
        let row: Vec<_> = index.lines().last().unwrap().split('\t').collect();
        assert_eq!(row[5], version.unwrap_or("1"));
        final_frames.push(std::fs::read(Path::new(&frames_dir).join(row[0])).unwrap());
        outputs.push(std::fs::read(out).unwrap());
        states.push(std::fs::read(state).unwrap());
    }
    assert_eq!(outputs[0], outputs[1]);
    assert_ne!(outputs[1], outputs[2]);
    assert_eq!(final_frames[0], final_frames[1]);
    assert_ne!(final_frames[1], final_frames[2]);
    assert_eq!(states[0], states[1]);
    assert_eq!(states[1], states[2]);
}

#[test]
fn display_export_versions_are_honored_by_live_save() {
    let temp = Temp::new("save-versions");
    let name = format!("gloss-save-test-{}", std::process::id());
    let like = Session::replay(64).unwrap();
    let mut server = Server::resume_as(name.clone(), Some(&like)).unwrap();
    for chunk in parse_program(&program(paint::ENGINE)) {
        server.serve_cmd("do", &[], &chunk).unwrap();
    }
    let before = state_digest_line(&server.s, 2, 0.0);
    for (label, version) in [("default", None), ("legacy", Some("1")), ("gloss", Some("2"))] {
        let out = temp.file(&format!("{label}.png"));
        let mut args = vec![out.clone(), "--gallery".into()];
        if let Some(version) = version {
            args.extend(["--look-version".into(), version.into()]);
        }
        server.serve_cmd("save", &args, "").unwrap();
        let expected = temp.file("expected.png");
        let version = look::version_of(version.unwrap_or("1")).unwrap();
        let pixels = server.s.canvas().unwrap().seen_lit_version(115.0, 55.0, 1.0, version);
        deliver_seen(&server.s.canvas().unwrap(), Path::new(&expected), &pixels).unwrap();
        assert_eq!(std::fs::read(out).unwrap(), std::fs::read(expected).unwrap());
    }
    assert_eq!(before, state_digest_line(&server.s, 2, 0.0));
    std::fs::remove_file(log_path(&name)).unwrap();
    std::fs::remove_dir_all(session_dir(&name)).unwrap();
}

#[cfg(feature = "finish")]
#[test]
fn display_export_versions_pin_finishing_and_select_optional_gallery_light() {
    let temp = Temp::new("finish-versions");
    let source = temp.file("painting.lua");
    let state = temp.file("painting.ckpt");
    std::fs::write(&source, program(paint::ENGINE)).unwrap();
    run(&[source, "--width".into(), "64".into(), "--out".into(), temp.file("color.png"), "--checkpoint-out".into(), state.clone()]).unwrap();
    let source_bytes = std::fs::read(&state).unwrap();
    for lit in [false, true] {
        let mut outputs = Vec::new();
        for (label, version) in [("default", None), ("legacy", Some("1")), ("gloss", Some("2"))] {
            let out = temp.file(&format!("finish-{lit}-{label}.png"));
            let mut args = vec![state.clone(), out.clone(), "--no-varnish".into(), "--no-cracks".into(), "--relief".into()];
            if lit {
                args.push("--gallery".into());
            }
            if let Some(version) = version {
                args.extend(["--look-version".into(), version.into()]);
            }
            finish_cmd(&args).unwrap();
            outputs.push(std::fs::read(out).unwrap());
        }
        assert_eq!(outputs[0], outputs[1]);
        if lit {
            assert_ne!(outputs[1], outputs[2]);
        } else {
            assert_eq!(outputs[1], outputs[2]);
        }
    }
    assert_eq!(std::fs::read(state).unwrap(), source_bytes);
}

#[test]
fn checkpoint_render_matches_native_color_and_look_companions_for_every_engine() {
    let temp = Temp::new("parity");
    for engine in 1..=paint::ENGINE {
        let text = program(engine);
        let tubes = session::box_for(Some(&text)).unwrap();
        let mut s = Session::replay_with(96, tubes).unwrap();
        for chunk in parse_program(&text) {
            s.run(&chunk).unwrap();
        }
        let checkpoint = temp.file(&format!("engine-{engine}.ckpt"));
        let color = temp.file("original.png");
        let views = vec![(look::View::default(), PathBuf::from(temp.file("original-look.png")))];
        let before = state_digest_line(&s, 2, 0.0);
        let c = s.canvas().unwrap();
        deliver(&c, Path::new(&color)).unwrap();
        look::look(&c, &views[0].0, &views[0].1).unwrap();
        look::companions(&c, &views);
        drop(c);
        assert!(save::write(&s, &text, Path::new(&checkpoint)).unwrap());
        let restored = save::read(Path::new(&checkpoint)).unwrap();
        assert_eq!(restored.log_fnv, save::fnv1a(text.as_bytes()));
        assert_eq!(restored.chunks, 2);
        assert_eq!(restored.session.globals(), "");
        assert!(restored.session.lua.globals().get::<mlua::Value>("continuation").unwrap().is_nil());
        for (mode, light) in [("color", None), ("gallery", Some(GALLERY_LIGHT)), ("relief", look::DISPLAY_PRESETS[1].view.light)] {
            let out = temp.file("rendered.png");
            render_checkpoint(&[checkpoint.clone(), out.clone(), "--mode".into(), mode.into()]).unwrap();
            let expected = temp.file("expected.png");
            deliver_lit(&s.canvas().unwrap(), Path::new(&expected), light).unwrap();
            assert_eq!(std::fs::read(&out).unwrap(), std::fs::read(&expected).unwrap(), "native {mode}, engine {engine}");
            render_checkpoint(&[checkpoint.clone(), out.clone(), "--mode".into(), mode.into(), "--look".into()]).unwrap();
            let expected_look = match mode {
                "color" => views[0].1.clone(),
                "gallery" => temp.0.join(".gallery/original-look.png"),
                _ => temp.0.join(".relief/original-look.png"),
            };
            assert_eq!(std::fs::read(&out).unwrap(), std::fs::read(expected_look).unwrap(), "look {mode}, engine {engine}");
        }
        let out = temp.file("default.png");
        render_checkpoint(&[checkpoint.clone(), out.clone()]).unwrap();
        deliver_lit(&s.canvas().unwrap(), Path::new(&color), Some(GALLERY_LIGHT)).unwrap();
        assert_eq!(std::fs::read(out).unwrap(), std::fs::read(color).unwrap());
        assert_eq!(before, state_digest_line(&s, 2, 0.0));
    }
}

#[test]
fn final_checkpoint_export_needs_no_dump_and_existing_zstd_save_renders_without_log() {
    let temp = Temp::new("export");
    let source = temp.file("painting.lua");
    let out = temp.file("direct.png");
    let checkpoint = temp.file("saved/final.ckpt");
    let text = program(paint::ENGINE);
    std::fs::write(&source, &text).unwrap();
    run(&[source.clone(), "--width".into(), "96".into(), "--out".into(), out.clone(), "--checkpoint-out".into(), checkpoint.clone()]).unwrap();
    assert_eq!(std::fs::read_dir(temp.0.join("saved")).unwrap().count(), 1);
    let raw = std::fs::read(&checkpoint).unwrap();
    assert_eq!(&raw[..8], b"PAINTC11");
    assert!(raw.windows(b"easel save 1\n".len()).any(|v| v == b"easel save 1\n"));
    let compressed = temp.file("live.ckpt.zst");
    std::fs::write(&compressed, zstd::stream::encode_all(raw.as_slice(), 1).unwrap()).unwrap();
    let restored = temp.file("restored.png");
    render_checkpoint(&[compressed.clone(), restored.clone(), "--mode".into(), "color".into(), "--log".into(), source.clone()]).unwrap();
    assert_eq!(std::fs::read(&out).unwrap(), std::fs::read(&restored).unwrap());
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(checkpoint).unwrap();
    render_checkpoint(&[compressed, restored.clone(), "--mode".into(), "color".into()]).unwrap();
    assert_eq!(std::fs::read(out).unwrap(), std::fs::read(restored).unwrap());
}

#[test]
fn failed_run_publishes_no_final_checkpoint_or_replacement() {
    let temp = Temp::new("failure");
    let source = temp.file("failed.lua");
    let checkpoint = temp.file("final.ckpt");
    let args = [source.clone(), "--width".into(), "32".into(), "--out".into(), temp.file("out.png"), "--checkpoint-out".into(), checkpoint.clone()];
    std::fs::write(&source, format!("{}\n--@ chunk 3\nerror('failed chunk')", program(5))).unwrap();
    assert!(run(&args).unwrap_err().contains("chunk 3 failed"));
    assert!(!Path::new(&checkpoint).exists());
    std::fs::write(&checkpoint, b"previous accepted checkpoint").unwrap();
    assert!(run(&args).is_err());
    assert_eq!(std::fs::read(&checkpoint).unwrap(), b"previous accepted checkpoint");
    std::fs::write(&source, "--@ chunk 1\nx=42\n").unwrap();
    assert!(run(&args).unwrap_err().contains("never made a canvas"));
    assert_eq!(std::fs::read(&checkpoint).unwrap(), b"previous accepted checkpoint");
    std::fs::write(&source, program(5)).unwrap();
    let blocked = temp.file("directory.png");
    std::fs::create_dir(&blocked).unwrap();
    let mut blocked_args = args.clone();
    blocked_args[4] = blocked;
    assert!(run(&blocked_args).is_err());
    assert_eq!(std::fs::read(&checkpoint).unwrap(), b"previous accepted checkpoint");
    let mut alias_args = args;
    alias_args[6] = source.clone();
    assert!(run(&alias_args).unwrap_err().contains("must differ"));
    assert_eq!(std::fs::read_to_string(source).unwrap(), program(5));
}

#[test]
fn checkpoint_render_refuses_mismatched_log_bad_compression_and_continuation_options() {
    let temp = Temp::new("refusal");
    let text = program(5);
    let mut s = Session::replay_with(32, session::box_for(Some(&text)).unwrap()).unwrap();
    for chunk in parse_program(&text) {
        s.run(&chunk).unwrap();
    }
    let checkpoint = temp.file("live.ckpt");
    save::write(&s, &text, Path::new(&checkpoint)).unwrap();
    let out = temp.file("out.png");
    let log = temp.file("wrong.lua");
    std::fs::write(&log, format!("{text}\n")).unwrap();
    assert!(render_checkpoint(&[checkpoint.clone(), out.clone(), "--log".into(), log]).unwrap_err().contains("not a save"));
    assert!(render_checkpoint(&[checkpoint.clone(), out.clone(), "--resume".into()]).unwrap_err().contains("unknown argument"));
    assert!(render_checkpoint(&[checkpoint.clone(), out.clone(), "--mode".into()]).is_err());
    let alias = temp.file("alias.png");
    std::fs::hard_link(&checkpoint, &alias).unwrap();
    assert!(render_checkpoint(&[checkpoint.clone(), alias]).unwrap_err().contains("must differ"));
    assert!(!Path::new(&out).exists());
    let raw = std::fs::read(checkpoint).unwrap();
    let mut bytes = zstd::stream::encode_all(raw.as_slice(), 1).unwrap();
    bytes.truncate(bytes.len() - 4);
    let compressed = temp.file("truncated.ckpt.zst");
    std::fs::write(&compressed, bytes).unwrap();
    assert!(render_checkpoint(&[compressed, out.clone()]).is_err());
    assert!(!Path::new(&out).exists());
    let trailing = temp.file("trailing.ckpt");
    std::fs::write(&trailing, [raw.as_slice(), b"extra"].concat()).unwrap();
    assert!(save::read(Path::new(&trailing)).err().unwrap().contains("trailing data"));
}
