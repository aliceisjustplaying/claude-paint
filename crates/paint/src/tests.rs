//! Characterization tests: pin down current behavior (conservation, drying,
//! determinism, and a golden image of a fixed multi-pass fixture). Regenerate the golden file
//! deliberately with `UPDATE_GOLDEN=1 cargo test -p paint` when a change is
//! meant to alter rendering. The golden is recorded with the workspace's `[profile.test]`
//! (optimized, debug assertions on: see Cargo.toml). Cross-build image drift
//! follows notes/workflow.md; same-build state fingerprints remain exact.

use crate::bristle::{Gesture, Held, Orient, Tool};
use crate::canvas::Canvas;
use crate::color::hex;
#[cfg(tube_box)]
use crate::handling::Handling;
#[cfg(tube_box)]
use crate::mask::Mask;
#[cfg(tube_box)]
use crate::pigment::Pigment;
#[cfg(tube_box)]
use crate::style::Style;
use crate::wet::Paint;

fn brush_volume(h: &Held) -> f64 {
    h.bristles.iter().map(|b| b.vol as f64).sum()
}

/// A drag away from the edges neither creates nor destroys paint: what
/// leaves the brush is on the canvas, and what the canvas lost is on the brush.
#[test]
fn drag_conserves_paint() {
    for tool in [Tool::round_sable(3.0), Tool::hog_flat(12.0), Tool::filbert(9.0), Tool::fan(14.0), Tool::rigger(0.8), Tool::badger(20.0)] {
        let mut c = Canvas::new(400, 1.0, hex("#c8b89a")).with_linen(crate::surface::Linen::fine(3));
        // some wet paint to pick up
        let mut under = Held::new(Tool::filbert(20.0), 2);
        under.load(Paint::body(hex("#304060")), 1.0);
        c.drag(&mut under, &Gesture::new(vec![(300.0, 480.0), (700.0, 520.0)]).pressure(0.9, 0.9), None);
        let mut h = Held::new(tool.clone(), 5);
        h.load(Paint::body(hex("#d0c060")), 0.8);
        let before = c.wet_total() + brush_volume(&h);
        c.drag(&mut h, &Gesture::new(vec![(250.0, 450.0), (500.0, 540.0), (750.0, 470.0)]).pressure(0.8, 0.5).orient(Orient::Across), None);
        let after = c.wet_total() + brush_volume(&h);
        assert!((after - before).abs() <= before * 2e-3, "{:?}: {before} -> {after}", tool.kind);
    }
}

/// A stroke's mark reaches the end of its path (engine 8, `crate::REACH_END`)
/// however far its hairs trail behind the hand, so a limb drawn up to the
/// point where it forks meets the forks drawn from there. Before engine 8 a
/// rigger's mark stopped short of the end by its trail (4 px here).
#[test]
fn a_stroke_reaches_its_end() {
    let rigger = Tool { point: 1.0, stiffness: 0.45, ..Tool::rigger(1.0) };
    let pointed = Tool { point: 1.0, stiffness: 0.5, ..Tool::round_sable(1.6) };
    for tool in [rigger, pointed, Tool::round_sable(3.0), Tool::filbert(6.0)] {
        let mut c = Canvas::new(2400, 4.0, hex("#c8b89a")).with_linen(crate::surface::Linen::fine(3)).with_engine(crate::REACH_END);
        let mut h = Held::new(tool.clone(), 5);
        h.load(Paint::body(hex("#302820")), 0.6);
        let (x, y0, y1) = (500.0f32, 200.0f32, 80.0f32);
        c.drag(&mut h, &Gesture::new(vec![(x, y0), (x, (y0 + y1) * 0.5), (x, y1)]).pressure(0.8, 0.7).ramps(0.02, 0.0), None);
        let (w, s) = (c.f.w, c.f.scale);
        let band = ((x - 4.0) * s) as usize..((x + 4.0) * s) as usize;
        let at = |y: usize| band.clone().map(|x| c.wet.vol[y * w + x]).fold(0.0f32, f32::max);
        let body = at(((y0 + y1) * 0.5 * s) as usize);
        let top = (0..c.f.h).find(|&y| at(y) > 0.05 * body).unwrap();
        let short = top as f32 - y1 * s;
        assert!(short <= 1.5, "{:?}: the mark ends {short} px short of its path's end", tool.kind);
    }
}

#[test]
fn dry_is_idempotent_and_clears_wet() {
    let mut c = Canvas::new(300, 1.0, hex("#c8b89a")).with_linen(crate::surface::Linen::fine(3));
    let mut h = Held::new(Tool::filbert(20.0), 2);
    h.load(Paint::body(hex("#304060")), 1.0);
    c.drag(&mut h, &Gesture::new(vec![(200.0, 500.0), (800.0, 500.0)]), None);
    assert!(c.wet_total() > 0.0);
    c.dry();
    assert_eq!(c.wet_total(), 0.0);
    let snap = (c.px.clone(), c.height.clone(), c.film.clone());
    c.dry();
    assert!(snap.0 == c.px && snap.1 == c.height && snap.2 == c.film);
}

/// A fixture through most of the engine: in the upper half a broad pass
/// from a hand-knifed pile and the style's blend; after drying, in the lower half a
/// hog-flat pass, three held-brush drags, a glaze; then relief.
#[cfg(tube_box)]
fn fixture() -> Canvas {
    // (the golden was recorded with engine 2: the scene stays that engine's)
    let mut st = Style::oil();
    st.palette.engine = 2;
    let mut c = st.prepare(240, 1.5, 7);
    let (w, h) = (c.width(), c.height());
    let upper = Mask::from_fn(c.f, |_, y| if y < h * 0.5 { 1.0 } else { 0.0 });
    let pal = st.palette.only(&["lead white", "cobalt blue"]);
    c.work(&upper, &st.broad().piled(&pal, pal.pile(vec![(0, 0.75), (1, 0.25)]), 0.45).angle(|_, _| 0.0), 11);
    if let Some(b) = st.blend() {
        c.work(&upper, &b, 12);
    }
    c.dry();
    let lower = Mask::from_fn(c.f, |_, y| if y >= h * 0.5 { 1.0 } else { 0.0 });
    c.work(&lower, &Handling::new(Tool::hog_flat(8.0)).color(|_, _| hex("#4a4034")).paint(0.9, 0.9).load(0.8 * 0.9).coverage(3.0).clip(true), 13);
    for (i, tool) in [Tool::round_sable(2.0), Tool::fan(10.0), Tool::rigger(0.6)].into_iter().enumerate() {
        let mut held = Held::new(tool, 20 + i as u64);
        held.load(Paint::scumble(hex("#e0d8c0")), 0.8 * 0.6);
        let y = h * (0.55 + 0.1 * i as f32);
        c.drag(&mut held, &Gesture::new(vec![(w * 0.1, y), (w * 0.5, y - 20.0), (w * 0.9, y)]).pressure(0.7, 0.3), Some(&lower));
    }
    c.glaze(&Pigment::transparent(hex("#6a4c34")), Some(&lower), |_, _| 1.0);
    c.relief(0.3, 0.02);
    c
}

#[cfg(tube_box)]
fn fingerprint(c: &Canvas) -> String {
    // FNV-1a over the exact bits of every state buffer
    let mut hsh = 0xcbf2_9ce4_8422_2325u64;
    let mut eat = |v: f32| {
        for b in v.to_bits().to_le_bytes() {
            hsh ^= b as u64;
            hsh = hsh.wrapping_mul(0x100_0000_01b3);
        }
    };
    for p in &c.px {
        p.iter().for_each(|&v| eat(v));
    }
    c.height.iter().for_each(|&v| eat(v));
    c.film.iter().for_each(|&v| eat(v));
    format!("{hsh:016x}")
}

#[cfg(tube_box)]
fn in_pool<T: Send>(threads: usize, f: impl FnOnce() -> T + Send) -> T {
    rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap().install(f)
}

#[test]
#[cfg(tube_box)]
fn fixture_is_deterministic_across_thread_counts() {
    let a = in_pool(1, || fingerprint(&fixture()));
    let b = in_pool(4, || fingerprint(&fixture()));
    assert_eq!(a, b);
}

#[test]
#[cfg(tube_box)]
fn fixture_matches_golden() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden_scene.png");
    let platform_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden_scene.platform");
    let platform = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);
    if std::env::var("UPDATE_GOLDEN").is_ok() {
        fixture().save(path).unwrap();
        std::fs::write(platform_path, format!("{platform}\n")).unwrap();
        return;
    }
    let rendered = std::env::temp_dir().join(format!("paint-golden-{}.png", std::process::id()));
    fixture().save(&rendered).unwrap();
    let got_bytes = std::fs::read(&rendered).unwrap();
    let got = image::open(&rendered).unwrap().to_rgba8();
    std::fs::remove_file(rendered).unwrap();
    let reference_platform = std::fs::read_to_string(platform_path).unwrap_or_else(|_| "unknown".into());
    let known_platforms = ["macos-aarch64", "macos-x86_64", "linux-aarch64", "linux-x86_64", "windows-aarch64", "windows-x86_64"];
    if reference_platform.trim() == platform || !known_platforms.contains(&reference_platform.trim()) || !known_platforms.contains(&platform.as_str()) {
        assert!(got_bytes == std::fs::read(path).unwrap(), "golden PNG bytes changed on the same or unknown platform");
        return;
    }
    let want = image::open(path).expect("no golden: run with UPDATE_GOLDEN=1").to_rgba8();
    assert_eq!(got.dimensions(), want.dimensions());
    let mut changed = 0usize;
    let mut maximum = 0u8;
    for (a, b) in got.pixels().zip(want.pixels()) {
        assert_eq!(a.0[3], b.0[3], "golden image alpha changed");
        let delta = a.0[..3].iter().zip(&b.0[..3]).map(|(x, y)| x.abs_diff(*y)).max().unwrap();
        changed += usize::from(delta != 0);
        maximum = maximum.max(delta);
    }
    let pixels = got.width() as usize * got.height() as usize;
    assert!(maximum <= 2 && changed * 1_000 < pixels,
            "cross-platform golden image drift: {changed}/{pixels} pixels, max {maximum}/255; allowed <0.1% pixels and <=2/255 per channel");
}
