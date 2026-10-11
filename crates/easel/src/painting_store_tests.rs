use super::*;
use std::collections::BTreeMap;
use std::os::unix::fs::PermissionsExt;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

const RUNTIME: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const CANVAS: &str = r#"canvas{size=100, aspect=1, linen=15, seed=7, ground={{pile={{"test white", 1}}, um=40, apply="knife"}}}"#;
const PHASES: [&str; 8] = [
    "source.synced",
    "checkpoint.synced",
    "objects.synced",
    "manifest.synced",
    "manifest.installed",
    "pointer.synced",
    "pointer.installed",
    "pointer.directory.synced",
];

struct Temp(PathBuf);

impl Temp {
    fn new() -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        loop {
            let serial = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("easel-store-tests-{}-{serial}", std::process::id()));
            match std::fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
                Err(e) => panic!("create test directory: {e}"),
            }
        }
    }

    fn store(&self) -> PathBuf {
        self.0.join("store")
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn session() -> Session {
    let mut tubes = paint::Palette::new(
        "store-test",
        vec![paint::palette::Tube {
            name: "test white",
            pigment: "test white",
            color: [0.8, 0.8, 0.8],
            hiding: 0.8,
            stiff: 0.7,
            strength: 1.0,
            drying: 1.0,
            drying_3: 1.0,
        }],
    );
    tubes.engine = 5;
    Session::with_box(16, tubes).unwrap()
}

fn material() -> Session {
    let mut s = session();
    s.run(CANVAS).unwrap();
    s
}

fn source(s: &Session) -> String {
    s.program("store-test")
}

fn open(path: &Path) -> PaintingStore {
    PaintingStore::open(path.to_path_buf(), DEFAULT_DISK_BYTES, RUNTIME.into()).unwrap()
}

fn names(path: &Path) -> BTreeSet<String> {
    std::fs::read_dir(path)
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .collect()
}

fn files(path: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut result = BTreeMap::new();
    for dir in [
        path.to_path_buf(),
        path.join("objects"),
        path.join("revisions"),
    ] {
        for entry in std::fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                result.insert(
                    entry.path().strip_prefix(path).unwrap().to_path_buf(),
                    std::fs::read(entry.path()).unwrap(),
                );
            }
        }
    }
    result
}

fn assert_retained(store: &PaintingStore) {
    let mut revisions = BTreeSet::new();
    let mut objects = BTreeSet::new();
    if let Some((id, m)) = &store.current {
        revisions.insert(format!("{id}.json"));
        let mut retained = vec![m.clone()];
        if let Some(parent) = &m.parent {
            revisions.insert(format!("{parent}.json"));
            retained.push(manifest(&store.dir, parent).unwrap());
        }
        for m in retained {
            verify_revision(&store.dir, &m).unwrap();
            objects.insert(format!("{}.zst", m.source.id));
            if let Some(checkpoint) = m.checkpoint {
                objects.insert(format!("{}.zst", checkpoint.id));
            }
        }
    }
    assert_eq!(names(&store.dir.join("revisions")), revisions);
    assert_eq!(names(&store.dir.join("objects")), objects);
    assert!(!names(&store.dir).iter().any(|n| n.starts_with(".pending-")));
}

fn assert_revision(dir: &Path, s: &Session, expected_parent: Option<&str>) {
    let read = ReadRevision::open(dir.to_path_buf()).unwrap();
    let text = source(s);
    assert_eq!(read.source, text);
    assert_eq!(read.manifest.parent.as_deref(), expected_parent);
    assert_eq!(read.manifest.chunks, s.log.len());
    assert_eq!(read.manifest.runtime_sha256, RUNTIME);
    assert_eq!(read.manifest.engine, 5);
    assert_eq!(read.manifest.painting_width, 16);
    assert!(read.manifest.renderable && read.manifest.finishable && !read.manifest.resumable);
    let mut raw = Vec::new();
    assert!(save::write_to(s, &text, &mut raw).unwrap());
    let checkpoint = read.manifest.checkpoint.as_ref().unwrap();
    assert_eq!(checkpoint.id, digest("easel-checkpoint/1", &raw));
    assert_eq!(checkpoint.bytes, raw.len() as u64);
    let decoded =
        zstd::stream::decode_all(File::open(read.checkpoint().unwrap()).unwrap()).unwrap();
    assert_eq!(decoded, raw);
}

struct Subprocess(Child);

impl Subprocess {
    fn spawn(temp: &Temp, mode: &str, phase: &str, seeded: bool) -> Self {
        Self(
            Command::new(std::env::current_exe().unwrap())
                .args([
                    "--ignored",
                    "--exact",
                    "painting_store::tests::store_subprocess",
                    "--test-threads=1",
                    "--nocapture",
                ])
                .env("EASEL_STORE_TEST_DIR", &temp.0)
                .env("EASEL_STORE_TEST_MODE", mode)
                .env("EASEL_STORE_TEST_PHASE", phase)
                .env("EASEL_STORE_TEST_SEEDED", if seeded { "1" } else { "0" })
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .stderr(Stdio::inherit())
                .spawn()
                .unwrap(),
        )
    }

    fn signal(&mut self) {
        self.0.stdin.as_mut().unwrap().write_all(b"1").unwrap();
    }

    fn marker(&mut self, temp: &Temp, name: &str) {
        let deadline = Instant::now() + Duration::from_secs(60);
        while !temp.0.join(name).exists() {
            assert!(
                self.0.try_wait().unwrap().is_none(),
                "subprocess exited before {name}"
            );
            assert!(Instant::now() < deadline, "subprocess did not reach {name}");
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    fn finish(&mut self) -> ExitStatus {
        let deadline = Instant::now() + Duration::from_secs(60);
        loop {
            if let Some(status) = self.0.try_wait().unwrap() {
                return status;
            }
            assert!(Instant::now() < deadline, "subprocess did not exit");
            std::thread::sleep(Duration::from_millis(5));
        }
    }
}

impl Drop for Subprocess {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
#[ignore = "invoked only by painting-store subprocess tests"]
fn store_subprocess() {
    let dir =
        PathBuf::from(std::env::var_os("EASEL_STORE_TEST_DIR").expect("explicit test directory"));
    let mode = std::env::var("EASEL_STORE_TEST_MODE").unwrap();
    if mode == "writer-refusal" {
        let error = PaintingStore::open(dir.join("store"), DEFAULT_DISK_BYTES, RUNTIME.into())
            .err()
            .expect("second writer must fail");
        assert!(error.to_string().contains("another writer"), "{error}");
        std::fs::write(dir.join("refused"), b"refused").unwrap();
        return;
    }
    if mode == "gc" {
        std::fs::write(dir.join("opening"), b"opening").unwrap();
        let store = open(&dir.join("store"));
        assert_retained(&store);
        return;
    }
    let mut store = open(&dir.join("store"));
    let mut s = material();
    if mode == "crash" {
        if std::env::var("EASEL_STORE_TEST_SEEDED").unwrap() == "1" {
            s.run("wait(1)").unwrap();
        }
        let phase = std::env::var("EASEL_STORE_TEST_PHASE").unwrap();
        store
            .publish_observed(&s, &source(&s), &mut |step| {
                if step == phase {
                    std::process::exit(73);
                }
                Ok(())
            })
            .unwrap();
        panic!("crash phase {phase} was not reached");
    }
    assert_eq!(mode, "lease");
    std::fs::write(dir.join("ready"), b"ready").unwrap();
    let signal = || {
        let mut byte = [0];
        std::io::stdin().read_exact(&mut byte).unwrap();
        assert_eq!(byte, *b"1");
    };
    signal();
    s.run("wait(1)").unwrap();
    store
        .publish_observed(&s, &source(&s), &mut |step| {
            if step == "manifest.installed" {
                std::fs::write(dir.join("staged"), b"staged").unwrap();
                signal();
                std::fs::write(dir.join("locking"), b"locking").unwrap();
            }
            Ok(())
        })
        .unwrap();
    s.run("wait(2)").unwrap();
    store.publish(&s, &source(&s)).unwrap();
    assert_retained(&store);
}

#[test]
fn abrupt_exit_at_every_publish_phase_exposes_only_a_complete_old_or_new_revision() {
    for seeded in [false, true] {
        for phase in PHASES {
            let temp = Temp::new();
            let mut s = material();
            let old_id = if seeded {
                let mut store = open(&temp.store());
                store.publish(&s, &source(&s)).unwrap();
                Some(store.current.as_ref().unwrap().0.clone())
            } else {
                None
            };
            let old_source = seeded.then(|| source(&s));
            if seeded {
                s.run("wait(1)").unwrap();
            }
            let mut child = Subprocess::spawn(&temp, "crash", phase, seeded);
            assert_eq!(child.finish().code(), Some(73), "{phase}, seeded={seeded}");
            let committed = matches!(phase, "pointer.installed" | "pointer.directory.synced");
            let pointer = root(&temp.store()).unwrap();
            if committed {
                assert!(pointer.is_some());
                assert_ne!(pointer, old_id);
                assert_revision(&temp.store(), &s, old_id.as_deref());
            } else {
                assert_eq!(pointer, old_id, "{phase}, seeded={seeded}");
                if seeded {
                    assert_revision(&temp.store(), &material(), None);
                } else {
                    assert!(ReadRevision::open(temp.store()).is_err());
                }
            }
            let store = open(&temp.store());
            assert_eq!(
                store.source().unwrap(),
                if committed {
                    Some(source(&s))
                } else {
                    old_source
                }
            );
            assert_retained(&store);
            assert_eq!(store.current.as_ref().map(|(id, _)| id.clone()), pointer);
        }
    }
}

#[test]
fn only_one_writer_can_own_the_store_and_exit_releases_ownership() {
    let temp = Temp::new();
    let store = open(&temp.store());
    let error = PaintingStore::open(temp.store(), DEFAULT_DISK_BYTES, RUNTIME.into())
        .err()
        .expect("second writer must fail");
    assert!(error.to_string().contains("another writer"));
    let mut child = Subprocess::spawn(&temp, "writer-refusal", "", false);
    assert!(child.finish().success());
    assert!(temp.0.join("refused").exists());
    drop(store);
    let reopened = open(&temp.store());
    assert!(reopened.current.is_none());
}

#[test]
fn publication_refuses_a_tampered_expected_parent_before_and_after_staging() {
    let temp = Temp::new();
    let mut store = open(&temp.store());
    let mut s = material();
    store.publish(&s, &source(&s)).unwrap();
    let first = store.current.as_ref().unwrap().0.clone();
    s.run("wait(1)").unwrap();
    store.publish(&s, &source(&s)).unwrap();
    let second = store.current.as_ref().unwrap().0.clone();
    let accepted = source(&s);
    s.run("wait(2)").unwrap();
    std::fs::write(temp.store().join("CURRENT"), format!("{first}\n")).unwrap();
    let before = files(&temp.store());
    let mut observed = Vec::new();
    let error = store
        .publish_observed(&s, &source(&s), &mut |step| {
            observed.push(step.to_string());
            Ok(())
        })
        .unwrap_err();
    assert!(error.to_string().contains("changed unexpectedly"));
    assert!(observed.is_empty());
    assert_eq!(files(&temp.store()), before);
    assert!(store.validate_source(&accepted).is_err());
    std::fs::write(temp.store().join("CURRENT"), format!("{second}\n")).unwrap();
    let error = store
        .publish_observed(&s, &source(&s), &mut |step| {
            if step == "manifest.installed" {
                std::fs::write(temp.store().join("CURRENT"), format!("{first}\n"))?;
            }
            Ok(())
        })
        .unwrap_err();
    assert!(error.to_string().contains("changed before publication"));
    assert_eq!(
        root(&temp.store()).unwrap().as_deref(),
        Some(first.as_str())
    );
    assert_eq!(store.current.as_ref().unwrap().0, second);
    assert!(store.poisoned);
    assert!(
        store
            .publish(&s, &source(&s))
            .unwrap_err()
            .to_string()
            .contains("reopen")
    );
    drop(store);
    let store = open(&temp.store());
    assert_eq!(store.source().unwrap(), Some(source(&material())));
    assert_retained(&store);
}

fn overwrite_immutable(path: &Path, bytes: &[u8]) {
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).unwrap();
    std::fs::write(path, bytes).unwrap();
}

fn rewrite_manifest(dir: &Path, m: &Manifest) -> String {
    let bytes = serde_json::to_vec(m).unwrap();
    let id = digest("easel-revision/1", &bytes);
    std::fs::write(dir.join("revisions").join(format!("{id}.json")), bytes).unwrap();
    std::fs::write(dir.join("CURRENT"), format!("{id}\n")).unwrap();
    id
}

#[test]
fn corrupt_pointer_manifest_source_and_checkpoint_are_refused_without_repair() {
    for damaged in ["pointer", "manifest", "source", "checkpoint"] {
        let temp = Temp::new();
        let mut store = open(&temp.store());
        let s = material();
        store.publish(&s, &source(&s)).unwrap();
        let (id, m) = store.current.as_ref().unwrap().clone();
        drop(store);
        match damaged {
            "pointer" => std::fs::write(temp.store().join("CURRENT"), b"../escape\n").unwrap(),
            "manifest" => overwrite_immutable(
                &temp.store().join("revisions").join(format!("{id}.json")),
                b"{}",
            ),
            "source" | "checkpoint" => {
                let object = if damaged == "source" {
                    &m.source
                } else {
                    m.checkpoint.as_ref().unwrap()
                };
                let replacement =
                    zstd::stream::encode_all(&b"incorrect object bytes"[..], 1).unwrap();
                overwrite_immutable(&object_path(&temp.store(), &object.id), &replacement);
            }
            _ => unreachable!(),
        }
        let before = files(&temp.store());
        assert!(ReadRevision::open(temp.store()).is_err(), "{damaged}");
        assert!(
            PaintingStore::open(temp.store(), DEFAULT_DISK_BYTES, RUNTIME.into()).is_err(),
            "{damaged}"
        );
        assert_eq!(
            files(&temp.store()),
            before,
            "{damaged}: must not silently repair accepted state"
        );
    }
}

#[test]
fn valid_object_hashes_do_not_hide_source_or_checkpoint_metadata_mismatches() {
    for damaged in [
        "source-hash",
        "chunks",
        "checkpoint-source",
        "engine",
        "box",
        "width",
    ] {
        let temp = Temp::new();
        let mut store = open(&temp.store());
        let mut s = material();
        store.publish(&s, &source(&s)).unwrap();
        let old_checkpoint = store.current.as_ref().unwrap().1.checkpoint.clone();
        s.run("wait(1)").unwrap();
        store.publish(&s, &source(&s)).unwrap();
        let mut m = store.current.as_ref().unwrap().1.clone();
        drop(store);
        match damaged {
            "source-hash" => m.source_sha256 = "f".repeat(64),
            "chunks" => m.chunks += 1,
            "checkpoint-source" => m.checkpoint = old_checkpoint,
            "engine" => m.engine = 4,
            "box" => m.pigment_box = "other-test-box".into(),
            "width" => m.painting_width = 17,
            _ => unreachable!(),
        }
        let id = rewrite_manifest(&temp.store(), &m);
        assert_eq!(manifest(&temp.store(), &id).unwrap().source.id, m.source.id);
        verify_object(
            &temp.store(),
            &m.source,
            "easel-source/1",
            MAX_SOURCE_BYTES,
            false,
        )
        .unwrap();
        verify_object(
            &temp.store(),
            m.checkpoint.as_ref().unwrap(),
            "easel-checkpoint/1",
            MAX_STATE_BYTES,
            false,
        )
        .unwrap();
        assert!(ReadRevision::open(temp.store()).is_err(), "{damaged}");
        assert!(
            PaintingStore::open(temp.store(), DEFAULT_DISK_BYTES, RUNTIME.into()).is_err(),
            "{damaged}"
        );
    }
}

#[test]
fn runtime_identity_is_lowercase_sha256_and_mismatched_writers_are_refused() {
    let temp = Temp::new();
    assert!(valid_id(RUNTIME));
    for runtime in [
        "test-runtime".to_string(),
        "A".repeat(64),
        "0".repeat(63),
        "g".repeat(64),
    ] {
        assert!(PaintingStore::open(temp.store(), DEFAULT_DISK_BYTES, runtime).is_err());
    }
    let mut store = open(&temp.store());
    let s = material();
    store.publish(&s, &source(&s)).unwrap();
    drop(store);
    let error = PaintingStore::open(temp.store(), DEFAULT_DISK_BYTES, "f".repeat(64))
        .err()
        .expect("different runtime must fail");
    assert!(error.to_string().contains("runtime differs"));
    assert_eq!(ReadRevision::open(temp.store()).unwrap().source, source(&s));
}

#[test]
fn disk_admission_reserves_the_whole_transaction_before_writing_anything() {
    let temp = Temp::new();
    let mut store = open(&temp.store());
    let mut s = material();
    store.publish(&s, &source(&s)).unwrap();
    s.run("wait(1)").unwrap();
    let text = source(&s);
    let mut state = HashCount::new("easel-checkpoint/1", MAX_STATE_BYTES);
    assert!(save::write_to(&s, &text, &mut state).unwrap());
    let required = zstd::zstd_safe::compress_bound(text.len()) as u64
        + zstd::zstd_safe::compress_bound(state.bytes as usize) as u64
        + RESERVE_BYTES;
    let admission = store.disk_bytes().unwrap() + required;
    store.budget = admission - 1;
    let before = files(&temp.store());
    let mut steps = Vec::new();
    let error = store
        .publish_observed(&s, &text, &mut |step| {
            steps.push(step.to_string());
            Ok(())
        })
        .unwrap_err();
    assert!(error.to_string().contains("insufficient transaction space"));
    assert!(steps.is_empty());
    assert_eq!(files(&temp.store()), before);
    assert!(!store.poisoned);
    store.budget = admission;
    store.publish(&s, &text).unwrap();
    assert!(store.disk_bytes().unwrap() <= admission);
    assert_retained(&store);
    assert_eq!(store.source().unwrap(), Some(text));
    let used = store.disk_bytes().unwrap();
    drop(store);
    assert!(PaintingStore::open(temp.store(), used - 1, RUNTIME.into()).is_err());
    assert!(PaintingStore::open(temp.store(), used, RUNTIME.into()).is_ok());
}

#[test]
fn raw_admission_checks_source_state_and_compressed_file_bounds() {
    let temp = Temp::new();
    let mut store = open(&temp.store());
    let mut s = session();
    s.log.push(crate::session::Chunk {
        src: format!("--{}", "x".repeat(MAX_SOURCE_BYTES as usize)),
    });
    let text = source(&s);
    assert!(text.len() as u64 > MAX_SOURCE_BYTES);
    let before = files(&temp.store());
    assert!(
        store
            .publish(&s, &text)
            .unwrap_err()
            .to_string()
            .contains("source exceeds")
    );
    assert_eq!(files(&temp.store()), before);
    let mut state = HashCount::new("easel-checkpoint/1", MAX_STATE_BYTES);
    state.bytes = MAX_STATE_BYTES - 1;
    state.write_all(b"x").unwrap();
    let id = state.id();
    assert!(
        state
            .write_all(b"y")
            .unwrap_err()
            .to_string()
            .contains("raw byte budget")
    );
    assert_eq!(state.bytes, MAX_STATE_BYTES);
    assert_eq!(state.id(), id);
    let mut output = LimitedFile {
        file: File::create(temp.0.join("limited")).unwrap(),
        remaining: 3,
    };
    output.write_all(b"abc").unwrap();
    assert!(
        output
            .write_all(b"d")
            .unwrap_err()
            .to_string()
            .contains("disk budget")
    );
    assert_eq!(std::fs::read(temp.0.join("limited")).unwrap(), b"abc");
    let object = Object {
        id: "0".repeat(64),
        bytes: MAX_STATE_BYTES + 1,
    };
    assert!(
        verify_object(
            &temp.store(),
            &object,
            "easel-checkpoint/1",
            MAX_STATE_BYTES,
            false
        )
        .unwrap_err()
        .to_string()
        .contains("object budget")
    );
    let expanded = b"abcdef";
    let object = Object {
        id: digest("easel-source/1", expanded),
        bytes: expanded.len() as u64 - 1,
    };
    std::fs::write(
        object_path(&temp.store(), &object.id),
        zstd::stream::encode_all(&expanded[..], 1).unwrap(),
    )
    .unwrap();
    assert!(
        verify_object(
            &temp.store(),
            &object,
            "easel-source/1",
            MAX_SOURCE_BYTES,
            true
        )
        .unwrap_err()
        .to_string()
        .contains("raw byte budget")
    );
    assert_eq!(PaintingStore::budget(None).unwrap(), DEFAULT_DISK_BYTES);
    assert_eq!(PaintingStore::budget(Some("1")).unwrap(), 1 << 20);
    assert_eq!(PaintingStore::budget(Some("65536")).unwrap(), 65536 << 20);
    for value in ["0", "65537", "-1", "1.5", "18446744073709551616"] {
        assert!(PaintingStore::budget(Some(value)).is_err());
    }
}

#[test]
fn unchanged_source_and_material_are_a_noop_but_unjournaled_changes_are_refused() {
    let temp = Temp::new();
    let mut store = open(&temp.store());
    let s = material();
    let text = source(&s);
    store.publish(&s, &text).unwrap();
    let before = files(&temp.store());
    let id = store.current.as_ref().unwrap().0.clone();
    let mut phases = Vec::new();
    for _ in 0..4 {
        store
            .publish_observed(&s, &text, &mut |step| {
                phases.push(step.to_string());
                Ok(())
            })
            .unwrap();
    }
    assert!(phases.is_empty());
    assert_eq!(files(&temp.store()), before);
    assert_eq!(store.current.as_ref().unwrap().0, id);
    for dir in ["objects", "revisions"] {
        for entry in std::fs::read_dir(temp.store().join(dir)).unwrap() {
            assert_eq!(
                entry.unwrap().metadata().unwrap().permissions().mode() & 0o777,
                0o400
            );
        }
    }
    s.st.borrow_mut().seed += 1;
    assert!(
        store
            .publish(&s, &text)
            .unwrap_err()
            .to_string()
            .contains("material changed without a journal")
    );
    assert_eq!(files(&temp.store()), before);
    let other = material().program("renamed-test");
    assert!(
        store
            .publish(&material(), &other)
            .unwrap_err()
            .to_string()
            .contains("rewrite the accepted prefix")
    );
    assert_eq!(files(&temp.store()), before);
}

#[test]
fn a_source_only_revision_has_no_checkpoint_until_a_canvas_is_accepted() {
    let temp = Temp::new();
    let mut store = open(&temp.store());
    let mut s = session();
    s.run("answer = 42").unwrap();
    store.publish(&s, &source(&s)).unwrap();
    {
        let read = ReadRevision::open(temp.store()).unwrap();
        assert_eq!(read.source, source(&s));
        assert!(!read.manifest.renderable && !read.manifest.finishable && !read.manifest.resumable);
        assert!(read.manifest.checkpoint.is_none());
        assert!(read.checkpoint().is_err());
    }
    let parent = store.current.as_ref().unwrap().0.clone();
    s.run(CANVAS).unwrap();
    store.publish(&s, &source(&s)).unwrap();
    assert_revision(&temp.store(), &s, Some(&parent));
    assert_retained(&store);
    assert_eq!(names(&temp.store().join("objects")).len(), 3);
}

#[test]
fn many_chunks_retain_exactly_two_revisions_and_four_objects() {
    let temp = Temp::new();
    let mut store = open(&temp.store());
    let mut s = material();
    let mut previous = None;
    for chunk in 0..24 {
        if chunk != 0 {
            s.run(&format!("wait(1); accepted = {chunk}")).unwrap();
        }
        store.publish(&s, &source(&s)).unwrap();
        let (id, m) = store.current.as_ref().unwrap();
        assert_eq!(m.parent, previous);
        previous = Some(id.clone());
        assert_retained(&store);
        assert_eq!(
            names(&temp.store().join("revisions")).len(),
            if chunk == 0 { 1 } else { 2 }
        );
        assert_eq!(
            names(&temp.store().join("objects")).len(),
            if chunk == 0 { 2 } else { 4 }
        );
        assert!(store.disk_bytes().unwrap() < 1 << 20);
    }
    let expected = files(&temp.store());
    drop(store);
    let store = open(&temp.store());
    assert_eq!(files(&temp.store()), expected);
    assert_eq!(store.source().unwrap(), Some(source(&s)));
    assert_retained(&store);
}

#[test]
fn a_read_lease_pins_its_revision_until_publication_and_gc_can_proceed() {
    let temp = Temp::new();
    let mut store = open(&temp.store());
    let s = material();
    store.publish(&s, &source(&s)).unwrap();
    let (old_id, old) = store.current.as_ref().unwrap().clone();
    drop(store);
    let mut child = Subprocess::spawn(&temp, "lease", "", true);
    child.marker(&temp, "ready");
    let read = ReadRevision::open(temp.store()).unwrap();
    child.signal();
    child.marker(&temp, "staged");
    child.signal();
    child.marker(&temp, "locking");
    let gate = File::open(temp.store().join("readers.lock")).unwrap();
    assert!(
        gate.try_lock().is_err(),
        "the reader holds the exclusive publication/GC gate"
    );
    assert_eq!(
        root(&temp.store()).unwrap().as_deref(),
        Some(old_id.as_str())
    );
    assert_eq!(read.id, old_id);
    assert_eq!(read.source, source(&s));
    assert_eq!(
        verify_revision(&temp.store(), &read.manifest).unwrap(),
        source(&s)
    );
    assert!(read.checkpoint().unwrap().exists());
    assert!(
        temp.store()
            .join("revisions")
            .join(format!("{old_id}.json"))
            .exists()
    );
    drop(read);
    assert!(child.finish().success());
    assert!(
        !temp
            .store()
            .join("revisions")
            .join(format!("{old_id}.json"))
            .exists()
    );
    assert!(!object_path(&temp.store(), &old.source.id).exists());
    assert!(!object_path(&temp.store(), &old.checkpoint.unwrap().id).exists());
    let mut expected = material();
    expected.run("wait(1)").unwrap();
    expected.run("wait(2)").unwrap();
    let store = open(&temp.store());
    assert_eq!(store.source().unwrap(), Some(source(&expected)));
    assert_retained(&store);
}

#[test]
fn a_read_lease_also_blocks_reopen_garbage_collection() {
    let temp = Temp::new();
    let mut store = open(&temp.store());
    let s = material();
    store.publish(&s, &source(&s)).unwrap();
    drop(store);
    let read = ReadRevision::open(temp.store()).unwrap();
    let orphan = object_path(&temp.store(), &"f".repeat(64));
    let pending = temp.store().join("objects/.pending-abandoned-test");
    std::fs::write(&orphan, b"orphan").unwrap();
    std::fs::write(&pending, b"pending").unwrap();
    let mut child = Subprocess::spawn(&temp, "gc", "", false);
    child.marker(&temp, "opening");
    let gate = File::open(temp.store().join("readers.lock")).unwrap();
    assert!(gate.try_lock().is_err());
    assert!(orphan.exists() && pending.exists());
    assert_eq!(
        verify_revision(&temp.store(), &read.manifest).unwrap(),
        source(&s)
    );
    drop(read);
    assert!(child.finish().success());
    assert!(!orphan.exists() && !pending.exists());
    let store = open(&temp.store());
    assert_retained(&store);
}
