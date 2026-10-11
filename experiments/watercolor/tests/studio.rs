use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
use watercolor_pilot::{studio, Watercolor};
static ID: AtomicU64 = AtomicU64::new(0);
fn root() -> PathBuf {
    let p = std::env::temp_dir().join(format!(
        "watercolor-studio-{}-{}",
        std::process::id(),
        ID.fetch_add(1, Ordering::Relaxed)
    ));
    studio::init(&p).unwrap();
    p
}

#[test]
fn explicit_box_and_duplicate_chunks_fail_closed() {
    let p = root();
    fs::write(p.join("box"), "oil\n").unwrap();
    assert!(studio::run(&p, "paper{size=3}").is_err());
    fs::write(p.join("box"), "watercolor\n").unwrap();
    let code = "paper{size=3,aspect=1.5}";
    studio::run(&p, code).unwrap();
    let before = fs::read(p.join("checkpoint.json")).unwrap();
    assert!(studio::run(&p, code).is_err());
    assert_eq!(before, fs::read(p.join("checkpoint.json")).unwrap());
    fs::remove_dir_all(p).unwrap();
}

#[test]
fn resume_keeps_named_loads_clock_material_and_images_exact() {
    let a = root();
    let b = root();
    let start="paper{size=4,aspect=1.5}; b=brush{name='b',width=0.6}; b:load{water=2,pigments={ultramarine=0.01,earth=0.005}}; b:stroke{{0.4,1},{3.6,1.8}}; wait(0.123456789)";
    studio::run(&a, start).unwrap();
    studio::run(&b, start).unwrap();
    let bytes = fs::read(a.join("checkpoint.json")).unwrap();
    let restored = studio::Session::decode(&bytes).unwrap();
    assert_eq!(restored, studio::load(&a).unwrap());
    let finish="b=brush{name='b'}; assert(clock()>0.12); wait(1.876543211); look{path='final.png',width=160,mode='color'}; save('saved.json')";
    let x = studio::run(&a, finish).unwrap();
    let y = studio::run(&b, finish).unwrap();
    assert_eq!(x, y);
    assert_eq!(
        fs::read(a.join("final.png")).unwrap(),
        fs::read(b.join("final.png")).unwrap()
    );
    x.check_balance().unwrap();
    assert!(x.brushes["b"].water > 0.0);
    for p in [a, b] {
        fs::remove_dir_all(p).unwrap();
    }
}

#[test]
fn failed_chunk_rolls_back_state_outputs_and_named_saves() {
    let p = root();
    studio::run(
        &p,
        "paper{size=3}; b=brush{name='b'}; b:load{water=1,pigments={phthalo=0.01}}",
    )
    .unwrap();
    let before = fs::read(p.join("checkpoint.json")).unwrap();
    assert!(studio::run(&p,"b=brush{name='b'}; b:stroke{{0.3,0.5},{2,1}}; look{path='bad.png',width=64}; save('bad.json'); error('deliberate')").is_err());
    assert_eq!(before, fs::read(p.join("checkpoint.json")).unwrap());
    assert!(!p.join("bad.png").exists());
    assert!(!p.join("bad.json").exists());
    fs::remove_dir_all(p).unwrap();
}

#[test]
fn lift_blot_dry_contact_and_frame_hooks_conserve_whole_studio() {
    let p = root();
    let s=studio::run(&p,"paper{size=4,kind='hot-press'}; b=brush{name='b',width=0.6}; b:load{water=2,pigments={earth=0.04,ultramarine=0.01}}; b:wash{x=0.2,y=0.2,width=3.5,height=2}; frames{every=0.0625,width=64}; wait(0.25); frames(false); blot{x=0.2,y=0.2,width=1,height=1}; d=brush{name='d',width=0.4}; d:load{water=0.1}; d:lift{{1,1},{3,1}}; t=brush{name='t',width=0.3}; t:load{pigments={earth=0.01}}; t:stroke({{0.5,2},{3,2}},{dry=true}); assert(balance().conserved)").unwrap();
    s.check_balance().unwrap();
    assert!(s.blotted_water > 0.0);
    assert!(s.brushes["d"].pigment.iter().sum::<f64>() > 0.0);
    assert_eq!(
        fs::read_dir(&p)
            .unwrap()
            .filter(|e| e
                .as_ref()
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with("frame-"))
            .count(),
        4
    );
    fs::remove_dir_all(p).unwrap();
}

#[test]
fn fractional_coverage_is_optical_and_keeps_its_inventory() {
    let mut s = Watercolor::new(20, 20, 71);
    s.add_covered(210, 0.0, [0.01, 0.0, 0.0], 0.25);
    s.wait_seconds(0.0625);
    assert_eq!(s.active.coverage[210], 0.25);
    assert!((s.balance().pigment_error[0]).abs() < 1e-12);
    let partial = s.rgb_color();
    s.active.coverage[210] = 1.0;
    let full = s.rgb_color();
    assert!(partial[210 * 3] > full[210 * 3]);
}

#[test]
fn directional_contact_extends_along_the_stroke_from_a_tooth_peak() {
    let mut s = Watercolor::new(25, 25, 7);
    s.paper.height.fill(0.2);
    s.paper.height[12 * 25 + 12] = 0.95;
    for i in 0..625 {
        s.dry_contact_directional(i, [0.01, 0.0, 0.0], 0.3, [1.0, 0.0]);
    }
    let row = s.active.deposited[12 * 25..13 * 25]
        .iter()
        .filter(|p| p[0] > 0.0)
        .count();
    let column = (0..25)
        .filter(|y| s.active.deposited[y * 25 + 12][0] > 0.0)
        .count();
    assert!(row > column * 3);
    assert!(s.balance().pigment_error[0].abs() < 1e-12);
}

#[test]
fn named_checkpoint_can_resume_and_a_loop_is_bounded() {
    let p = root();
    studio::run(&p,"paper{size=3}; b=brush{name='b'}; b:load{water=1,pigments={earth=0.01}}; wait(0.123456789); save('before.json')").unwrap();
    let saved = studio::Session::decode(&fs::read(p.join("before.json")).unwrap()).unwrap();
    studio::run(&p, "wait(2)").unwrap();
    studio::restore(&p, "before.json").unwrap();
    assert_eq!(saved, studio::load(&p).unwrap());
    let bytes = fs::read(p.join("checkpoint.json")).unwrap();
    assert!(studio::run(&p, "while true do end").is_err());
    assert_eq!(bytes, fs::read(p.join("checkpoint.json")).unwrap());
    fs::remove_dir_all(p).unwrap();
}

#[test]
fn look_without_arguments_uses_color_and_keeps_material_unchanged() {
    let p = root();
    let state = studio::run(&p, "paper{size=3}; look()").unwrap();
    studio::render(
        state.canvas.as_ref().unwrap(),
        &p.join("expected.png"),
        "color",
        1200,
        None,
    )
    .unwrap();
    assert_eq!(
        fs::read(p.join("look.png")).unwrap(),
        fs::read(p.join("expected.png")).unwrap()
    );
    fs::remove_dir_all(p).unwrap();
}
