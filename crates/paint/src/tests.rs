//! Characterization tests: pin down current behavior (conservation, mixing,
//! determinism, and a golden fingerprint of a small scene) so refactors can
//! prove they change nothing. Regenerate the golden file deliberately with
//! `UPDATE_GOLDEN=1 cargo test -p paint` when a change is meant to alter
//! rendering.

use crate::bristle::{Gesture, Held, Orient, Tool};
use crate::canvas::Canvas;
use crate::color::hex;
use crate::handling::Handling;
use crate::mask::Mask;
use crate::pigment::Pigment;
use crate::style::Style;
use crate::wet::{Paint, mix_into};

fn brush_volume(h: &Held) -> f64 {
    h.bristles.iter().map(|b| b.vol as f64).sum()
}

#[test]
fn mix_into_is_volume_weighted() {
    let a = mixbox::linear_float_rgb_to_latent(&[0.8, 0.1, 0.1]);
    let b = mixbox::linear_float_rgb_to_latent(&[0.1, 0.1, 0.8]);
    let (mut v, mut l, mut p) = (1.0f32, a, [0.2f32, 0.4]);
    mix_into(&mut v, &mut l, &mut p, 3.0, &b, [0.6, 0.8]);
    assert!((v - 4.0).abs() < 1e-6);
    for k in 0..l.len() {
        assert!((l[k] - (0.25 * a[k] + 0.75 * b[k])).abs() < 1e-5);
    }
    assert!((p[0] - 0.5).abs() < 1e-6 && (p[1] - 0.7).abs() < 1e-6);
    // zero or negative volume is a no-op
    let before = (v, l, p);
    mix_into(&mut v, &mut l, &mut p, 0.0, &a, [0.0, 0.0]);
    assert_eq!(before, (v, l, p));
}

#[test]
fn load_wipe_reload() {
    let mut h = Held::new(Tool::filbert(10.0), 1);
    assert_eq!(brush_volume(&h), 0.0);
    h.load(Paint::body(hex("#445566")), 1.0);
    let full = brush_volume(&h);
    assert!(full > 0.0);
    h.wipe(0.5);
    assert!((brush_volume(&h) - full * 0.5).abs() < full * 1e-5);
    h.reload(Paint::body(hex("#445566")), 1.0);
    assert!(brush_volume(&h) > full * 0.5);
}

/// A drag away from the edges neither creates nor destroys paint: what
/// leaves the brush is on the canvas, and what the canvas lost is on the brush.
#[test]
fn drag_conserves_paint() {
    for tool in [Tool::round_sable(3.0), Tool::hog_flat(12.0), Tool::filbert(9.0), Tool::fan(14.0), Tool::rigger(0.8), Tool::badger(20.0)] {
        let mut c = Canvas::new(400, 1.0, hex("#c8b89a")).with_weave(1.2, 0.3, 3);
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

#[test]
fn dry_is_idempotent_and_clears_wet() {
    let mut c = Canvas::new(300, 1.0, hex("#c8b89a")).with_weave(1.2, 0.3, 3);
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

/// A small scene through most of the engine.
fn scene() -> Canvas {
    let st = Style::friedrich();
    let mut c = st.prepare(240, 1.5, 7);
    let (w, h) = (c.width(), c.height());
    let sky = Mask::from_fn(c.f, |_, y| if y < h * 0.5 { 1.0 } else { 0.0 });
    c.work(&sky, &st.broad().color(|_, y| if y < 200.0 { hex("#5a6d8c") } else { hex("#c9b48e") }).angle(|_, _| 0.0), 11);
    if let Some(b) = st.blend() {
        c.work(&sky, &b, 12);
    }
    c.dry();
    let land = Mask::from_fn(c.f, |_, y| if y >= h * 0.5 { 1.0 } else { 0.0 });
    c.work(&land, &Handling::new(Tool::hog_flat(8.0)).color(|_, _| hex("#4a4034")).paint(0.9, 0.9).coverage(3.0).clip(true), 13);
    for (i, tool) in [Tool::round_sable(2.0), Tool::fan(10.0), Tool::rigger(0.6)].into_iter().enumerate() {
        let mut held = Held::new(tool, 20 + i as u64);
        held.load(Paint::scumble(hex("#e0d8c0")), 0.8);
        let y = h * (0.55 + 0.1 * i as f32);
        c.drag(&mut held, &Gesture::new(vec![(w * 0.1, y), (w * 0.5, y - 20.0), (w * 0.9, y)]).pressure(0.7, 0.3), Some(&land));
    }
    c.glaze(&Pigment::transparent(hex("#6a4c34")), Some(&land), |_, _| 1.0);
    c.relief(0.3, 0.02);
    c
}

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

fn in_pool<T: Send>(threads: usize, f: impl FnOnce() -> T + Send) -> T {
    rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap().install(f)
}

#[test]
fn scene_is_deterministic_across_thread_counts() {
    let a = in_pool(1, || fingerprint(&scene()));
    let b = in_pool(4, || fingerprint(&scene()));
    assert_eq!(a, b);
}

#[test]
fn scene_matches_golden() {
    let got = fingerprint(&scene());
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden_scene.txt");
    if std::env::var("UPDATE_GOLDEN").is_ok() {
        std::fs::create_dir_all(concat!(env!("CARGO_MANIFEST_DIR"), "/tests")).unwrap();
        std::fs::write(path, &got).unwrap();
        return;
    }
    let want = std::fs::read_to_string(path).expect("no golden: run with UPDATE_GOLDEN=1");
    assert_eq!(got, want.trim(), "rendering changed; if intended, rerun with UPDATE_GOLDEN=1");
}

#[test]
fn km_zero_absorption_is_finite() {
    let p = Pigment { k: [0.0; 3], s: [1.0; 3] };
    let (r, t) = p.layer(1.0);
    for i in 0..3 {
        assert!((r[i] - 0.5).abs() < 1e-5 && (t[i] - 0.5).abs() < 1e-5, "{r:?} {t:?}");
    }
    // continuous with the general formula just above the threshold
    let q = Pigment { k: [1e-5; 3], s: [1.0; 3] };
    let (r2, t2) = q.layer(1.0);
    assert!((r2[0] - r[0]).abs() < 1e-3 && (t2[0] - t[0]).abs() < 1e-3);
}
