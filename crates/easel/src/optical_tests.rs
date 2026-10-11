use super::*;
use std::path::PathBuf;

struct Temp(PathBuf);

impl Temp {
    fn new(name: &str) -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/optical-tests").join(format!("{}-{}-{name}", std::process::id(), NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)));
        std::fs::create_dir_all(&dir).unwrap();
        Self(dir)
    }
    fn snapshot(&self, revision: [u8; 32]) -> (PathBuf, RenderSnapshot) {
        let canvas = fixture();
        let snapshot = RenderSnapshot::capture(&canvas, revision, runtime().unwrap()).unwrap();
        let path = self.0.join(format!("{}.view", hex(&revision)));
        atomic(&path, 100000, |w| snapshot.write(w)).unwrap();
        (path, snapshot)
    }
}

impl Drop for Temp { fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); } }

fn fixture() -> paint::Canvas {
    let mut c = paint::Canvas::new(32, 1.25, [0.12, 0.4, 0.7]);
    c.prime([0.2, 0.3, 0.6], 0.9, 120.0, 0.8, 0.4, 7);
    c
}

fn arguments(input: &Path, output: &Path, tail: &[&str]) -> Vec<String> {
    let mut args = vec![input.to_string_lossy().into_owned(), output.to_string_lossy().into_owned()];
    args.extend(tail.iter().map(|s| (*s).into()));
    args
}

#[test]
fn standalone_pngs_match_native_delivery_in_all_legacy_engines() {
    let temp = Temp::new("pngs");
    for engine in 1..=paint::ENGINE {
        let c = fixture().with_engine(engine);
        let snapshot = RenderSnapshot::capture(&c, [engine as u8; 32], runtime().unwrap()).unwrap();
        let view = temp.0.join("snapshot.view");
        atomic(&view, 100000, |w| snapshot.write(w)).unwrap();
        for (mode, illumination) in [("color", None), ("gallery", Some((115.0, 55.0))), ("relief", Some((135.0, 25.0)))] {
            let native = temp.0.join("native.png");
            let compact = temp.0.join("compact.png");
            crate::deliver_lit(&c, &native, illumination).unwrap();
            render(&arguments(&view, &compact, &["--mode", mode])).unwrap();
            assert_eq!(std::fs::read(&native).unwrap(), std::fs::read(&compact).unwrap(), "{engine} {mode}");
        }
    }
}

#[test]
fn crops_keep_the_complete_lighting_context_and_refuse_missing_pixels() {
    let temp = Temp::new("crop");
    let mut c = paint::Canvas::new_window(48, 1.25, [0.12, 0.4, 0.7], Some(paint::Crop { units: [250.0, 300.0, 800.0, 650.0], margin: 70.0 }));
    c.prime([0.2, 0.3, 0.6], 0.9, 120.0, 0.8, 0.4, 7);
    let snapshot = RenderSnapshot::capture(&c, [2; 32], runtime().unwrap()).unwrap();
    let view = temp.0.join("window.view");
    atomic(&view, 100000, |w| snapshot.write(w)).unwrap();
    let target = temp.0.join("crop.png");
    let f = snapshot.frame();
    let keep = snapshot.keep();
    assert!(f.x0 > 0 && f.y0 > 0 && keep.0 > 0 && keep.1 > 0);
    let r = [f.x0 + keep.0 + 1, f.y0 + keep.1 + 1, f.x0 + keep.2 - 1, f.y0 + keep.3 - 1];
    let crop = r.map(|v| v.to_string()).join(",");
    render(&arguments(&view, &target, &["--crop", &crop])).unwrap();
    let lit = c.seen_lit(115.0, 55.0, 1.0);
    let mut raw = Vec::new();
    for y in r[1]..r[3] { for x in r[0]..r[2] { raw.extend(lit[(y - f.y0) * f.w + x - f.x0].map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8)); } }
    let expected = temp.0.join("expected.png");
    image::save_buffer(&expected, &raw, (r[2] - r[0]) as u32, (r[3] - r[1]) as u32, image::ColorType::Rgb8).unwrap();
    assert_eq!(std::fs::read(&target).unwrap(), std::fs::read(expected).unwrap());
    render(&arguments(&view, &target, &[])).unwrap();
    assert_eq!(image::image_dimensions(&target).unwrap(), ((keep.2 - keep.0) as u32, (keep.3 - keep.1) as u32));
    let held = [f.x0, f.y0, f.x0 + f.w, f.y0 + f.h].map(|v| v.to_string()).join(",");
    for bad in ["0,0,1,1", &held, "0,0,48,38", "20,20,10,10"] {
        let before = std::fs::read(&target).unwrap();
        assert!(render(&arguments(&view, &target, &["--crop", bad])).is_err());
        assert_eq!(std::fs::read(&target).unwrap(), before);
    }
}

#[test]
fn cache_reuses_verified_images_and_changes_with_revision_lighting_and_crop() {
    let temp = Temp::new("cache");
    let (view, _) = temp.snapshot([3; 32]);
    let cache = temp.0.join("cache");
    let args = arguments(&view, &cache, &[]);
    companions(&args).unwrap();
    let a = current(&cache).unwrap().unwrap();
    let ga = generation(&cache, &a).unwrap();
    let metadata = std::fs::metadata(cache.join(format!("{}.png", ga.images[1].id))).unwrap();
    companions(&args).unwrap();
    assert_eq!(current(&cache).unwrap().unwrap(), a);
    assert_eq!(std::fs::metadata(cache.join(format!("{}.png", ga.images[1].id))).unwrap().modified().unwrap(), metadata.modified().unwrap());
    companions(&arguments(&view, &cache, &["--gallery-light", "20,35,1"])).unwrap();
    let b = current(&cache).unwrap().unwrap();
    assert_ne!(a, b);
    let gb = generation(&cache, &b).unwrap();
    assert_eq!(ga.images[0].id, gb.images[0].id);
    assert_ne!(ga.images[1].id, gb.images[1].id);
    let (next, _) = temp.snapshot([4; 32]);
    companions(&arguments(&next, &cache, &[])).unwrap();
    let c = current(&cache).unwrap().unwrap();
    assert_ne!(b, c);
    assert_ne!(generation(&cache, &c).unwrap().config.revision, ga.config.revision);
    companions(&arguments(&next, &cache, &["--crop", "2,3,29,21"])).unwrap();
    let d = current(&cache).unwrap().unwrap();
    assert_ne!(c, d);
    assert_eq!(std::fs::read_dir(&cache).unwrap().filter(|e| e.as_ref().unwrap().path().extension().is_some_and(|s| s == "json")).count(), 2);
    let before = std::fs::read(cache.join("CURRENT")).unwrap();
    assert!(companions(&arguments(&next, &cache, &["--cache-budget-mib", "1", "--gallery-light", "30,40,1"])).is_err());
    assert_eq!(std::fs::read(cache.join("CURRENT")).unwrap(), before);
}

#[test]
fn corrupted_cache_and_renderer_mismatch_never_return_stale_companions() {
    let temp = Temp::new("invalid-cache");
    let (view, _) = temp.snapshot([5; 32]);
    let cache = temp.0.join("cache");
    companions(&arguments(&view, &cache, &[])).unwrap();
    let root = current(&cache).unwrap().unwrap();
    let value = generation(&cache, &root).unwrap();
    std::fs::write(cache.join(format!("{}.png", value.images[0].id)), b"corrupt").unwrap();
    assert!(companions(&arguments(&view, &cache, &[])).unwrap_err().contains("integrity"));
    let snapshot = RenderSnapshot::capture(&fixture(), [5; 32], [0; 32]).unwrap();
    atomic(&view, 100000, |w| snapshot.write(w)).unwrap();
    let output = temp.0.join("no.png");
    assert!(render(&arguments(&view, &output, &[])).unwrap_err().contains("renderer binary differs"));
    assert!(!output.exists());
}

#[test]
fn renderer_binary_and_edition_invalidate_otherwise_identical_cached_views() {
    let temp = Temp::new("renderer-key");
    let (view, _) = temp.snapshot([7; 32]);
    let cache = temp.0.join("cache");
    let args = arguments(&view, &cache, &[]);
    companions(&args).unwrap();
    for field in ["runtime", "renderer", "encoder"] {
        let root = current(&cache).unwrap().unwrap();
        let mut old = generation(&cache, &root).unwrap();
        old.parent = None;
        match field {
            "runtime" => old.config.runtime = hex(&[0; 32]),
            "renderer" => old.config.renderer = "legacy-look-0".into(),
            _ => old.config.encoder = "image-png-0.24".into(),
        }
        let bytes = serde_json::to_vec(&old).unwrap();
        let old_id = hex(&Sha256::digest(&bytes));
        atomic(&cache.join(format!("{old_id}.json")), 16384, |w| w.write_all(&bytes)).unwrap();
        atomic(&cache.join("CURRENT"), 65, |w| writeln!(w, "{old_id}")).unwrap();
        generation(&cache, &old_id).unwrap();
        companions(&args).unwrap();
        let new_id = current(&cache).unwrap().unwrap();
        assert_ne!(new_id, old_id, "{field}");
        let new = generation(&cache, &new_id).unwrap();
        assert_eq!(new.config.runtime, hex(&runtime().unwrap()));
        assert_eq!(new.config.renderer, RENDERER);
        assert_eq!(new.config.encoder, "image-png-0.25");
    }
}

#[test]
fn cache_writer_ownership_and_lost_pointer_protect_retained_images() {
    let temp = Temp::new("cache-ownership");
    let (view, _) = temp.snapshot([8; 32]);
    let cache = temp.0.join("cache");
    let args = arguments(&view, &cache, &[]);
    companions(&args).unwrap();
    let pointer = std::fs::read(cache.join("CURRENT")).unwrap();
    let lock = File::open(cache.join("writer.lock")).unwrap();
    lock.lock().unwrap();
    assert!(companions(&args).unwrap_err().contains("another companion writer"));
    assert_eq!(std::fs::read(cache.join("CURRENT")).unwrap(), pointer);
    drop(lock);
    let files: Vec<_> = std::fs::read_dir(&cache).unwrap().map(|e| e.unwrap().path()).filter(|p| p.extension().is_some_and(|s| s == "png")).collect();
    std::fs::remove_file(cache.join("CURRENT")).unwrap();
    assert!(companions(&args).unwrap_err().contains("lost its root"));
    assert!(files.iter().all(|p| p.exists()));
}

#[test]
fn atomic_output_and_alias_guards_leave_inputs_and_old_outputs_intact() {
    let temp = Temp::new("atomic");
    let (view, _) = temp.snapshot([6; 32]);
    let before = std::fs::read(&view).unwrap();
    let alias = temp.0.join("alias.png");
    std::fs::hard_link(&view, &alias).unwrap();
    assert!(render(&arguments(&view, &alias, &[])).is_err());
    assert_eq!(std::fs::read(&view).unwrap(), before);
    let output = temp.0.join("old.png");
    std::fs::write(&output, b"old complete output").unwrap();
    assert!(atomic(&output, 100, |w| { w.write_all(b"partial")?; Err(invalid("failure")) }).is_err());
    assert_eq!(std::fs::read(output).unwrap(), b"old complete output");
    assert!(!std::fs::read_dir(&temp.0).unwrap().any(|e| e.unwrap().file_name().to_string_lossy().ends_with(".part")));
    assert!(companions(&arguments(&view, &temp.0, &[])).is_err());
}
