//! Characterization tests: pin down current behavior (conservation, mixing,
//! determinism, and a golden fingerprint of a small scene) so refactors can
//! prove they change nothing. Regenerate the golden file deliberately with
//! `UPDATE_GOLDEN=1 cargo test -p paint` when a change is meant to alter
//! rendering. The golden is recorded in the default (debug) test profile;
//! release builds round floats differently and don't match it.

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
    c.work(&land, &Handling::new(Tool::hog_flat(8.0)).color(|_, _| hex("#4a4034")).paint(0.9, 0.9).load(0.8 * 0.9).coverage(3.0).clip(true), 13);
    for (i, tool) in [Tool::round_sable(2.0), Tool::fan(10.0), Tool::rigger(0.6)].into_iter().enumerate() {
        let mut held = Held::new(tool, 20 + i as u64);
        held.load(Paint::scumble(hex("#e0d8c0")), 0.8 * 0.6);
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

#[test]
#[should_panic(expected = "does not match canvas")]
fn mismatched_mask_is_rejected() {
    let mut c = Canvas::new(100, 1.0, hex("#808080"));
    let other = Canvas::new(120, 1.0, hex("#808080"));
    let m = Mask::from_fn(other.frame(), |_, _| 1.0);
    let mut h = Held::new(Tool::round_sable(3.0), 1);
    h.load(Paint::body(hex("#202020")), 1.0);
    c.drag(&mut h, &Gesture::line((100.0, 500.0), (900.0, 500.0)), Some(&m));
}

/// Every pixel a drag changes lies inside its computed footprint, even with
/// an extreme hand shake and a path whose spline overshoots its points.
#[test]
fn footprint_bounds_every_touched_pixel() {
    use crate::bristle::footprint;
    for (tool, shake, pts) in [
        (Tool::hog_flat(30.0), 3.0, vec![(300.0, 200.0), (500.0, 200.0), (500.0, 400.0), (300.0, 400.0)]),
        (Tool::fan(20.0), 1.0, vec![(200.0, 500.0), (800.0, 520.0)]),
        (Tool::rigger(0.6), 2.0, vec![(500.0, 300.0), (520.0, 330.0), (480.0, 360.0)]),
        (Tool::badger(40.0), 1.0, vec![(100.0, 700.0), (900.0, 690.0)]),
    ] {
        for scale in [0.1f32, 0.4] {
            let w = (1000.0 * scale) as usize;
            let mut c = Canvas::new(w, 1.0, hex("#c8b89a")).with_linen(crate::surface::Linen::fine(3));
            // wet paint everywhere to plough
            let mut under = Held::new(Tool::filbert(60.0), 2);
            for k in 0..12 {
                under.reload(Paint::body(hex("#304060")), 1.0);
                let y = 40.0 + k as f32 * 80.0;
                c.drag(&mut under, &Gesture::new(vec![(0.0, y), (1000.0, y)]).pressure(1.0, 1.0), None);
            }
            let before = c.wet.vol.clone();
            let mut h = Held::new(tool.clone(), 9);
            h.load(Paint::body(hex("#d0c060")), 1.0);
            c.drag(&mut h, &Gesture::new(pts.clone()).pressure(1.0, 1.0).shake(shake), None);
            let r = footprint(&tool, &pts, shake, c.f.scale, c.f.w, c.f.h).unwrap();
            for (i, (a, b)) in before.iter().zip(&c.wet.vol).enumerate() {
                if a != b {
                    let (x, y) = (i % c.f.w, i / c.f.w);
                    assert!(x >= r.0 && x < r.2 && y >= r.1 && y < r.3, "{:?} scale {scale}: ({x},{y}) outside {r:?}", tool.kind);
                }
            }
        }
    }
}

/// Tools that aren't physically possible are rejected before they can paint:
/// their strokes could leave the footprints the parallel scheduler relies on
/// (a negative length made the bend diverge; review finding).
#[test]
fn invalid_tools_are_rejected() {
    let t = Tool::round_sable(10.0);
    let bad: Vec<(&str, Tool)> = vec![
        ("length<0", Tool { length: -1.0, ..t.clone() }),
        ("length nan", Tool { length: f32::NAN, ..t.clone() }),
        ("width 0", Tool { width: 0.0, ..t.clone() }),
        ("width inf", Tool { width: f32::INFINITY, ..t.clone() }),
        ("stiffness>1", Tool { stiffness: 1.5, ..t.clone() }),
        ("stiffness<0", Tool { stiffness: -0.1, ..t.clone() }),
        ("bristles 0", Tool { bristles: 0, ..t.clone() }),
        ("hair 0", Tool { hair: 0.0, ..t.clone() }),
        ("run 0", Tool { run: 0.0, ..t.clone() }),
        ("lay<0", Tool { lay: -1.0, ..t.clone() }),
        ("pickup>1", Tool { pickup: 2.0, ..t.clone() }),
        ("push<0", Tool { push: -0.1, ..t.clone() }),
        ("splay<0", Tool { splay: -3.0, ..t.clone() }),
        ("ragged<0", Tool { ragged: -1.0, ..t.clone() }),
    ];
    for (name, tool) in &bad {
        assert!(tool.validate().is_err(), "{name} accepted");
    }
    for tool in [Tool::round_sable(1.0), Tool::hog_flat(8.0), Tool::filbert(4.0), Tool::fan(9.0), Tool::rigger(0.6), Tool::badger(30.0), Tool::stippler(2.0)] {
        tool.validate().unwrap();
    }
    // every painting entry point refuses them (before touching the canvas)
    let neg = Tool { length: -1.0, ..t };
    let entry = |f: &dyn Fn(&mut Canvas)| {
        let mut c = Canvas::new(100, 1.0, hex("#c8b89a"));
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut c)));
        assert!(r.is_err_and(|e| e.downcast_ref::<String>().is_some_and(|s| s.contains("length >= 0"))));
    };
    entry(&|c| {
        let mut h = Held::new(neg.clone(), 1);
        h.load(Paint::body(hex("#304060")), 1.0);
        c.drag(&mut h, &Gesture::line((400.0, 500.0), (500.0, 500.0)), None);
    });
    entry(&|c| c.touch(&mut Held::new(neg.clone(), 1), &crate::bristle::Touch::at(500.0, 500.0), None));
    entry(&|c| c.work(&Mask::from_fn(c.frame(), |_, _| 1.0), &Handling::new(neg.clone()), 1));
    entry(&|c| c.work(&Mask::from_fn(c.frame(), |_, _| 1.0), &Handling::new(Tool::filbert(10.0)).cut_in(neg.clone()), 1));
    entry(&|c| c.stipple(&Mask::from_fn(c.frame(), |_, _| 1.0), &crate::stipple::Stipple::new(neg.clone()), 1));
}

/// Extreme but valid tools (very long, limp or stiff hairs, wide splay,
/// ragged, a single bristle, fine or coarse hair) stay in their footprint:
/// every pixel a drag or touch visits (stroke id) or changes.
#[test]
fn footprint_bounds_extreme_valid_tools() {
    use crate::bristle::{Touch, footprint};
    let base = Tool::round_sable(10.0);
    let tools = [
        Tool { length: 200.0, stiffness: 0.0, ..base.clone() },
        Tool { length: 200.0, stiffness: 1.0, ..base.clone() },
        Tool { length: 0.0, ..base.clone() },
        Tool { splay: 6.0, ragged: 4.0, ..base.clone() },
        Tool { bristles: 1, hair: 8.0, ..base.clone() },
        Tool { hair: 0.01, push: 1.0, pickup: 1.0, ..Tool::hog_flat(25.0) },
        Tool { length: 150.0, splay: 3.0, ..Tool::rigger(2.0) },
    ];
    for tool in tools {
        tool.validate().unwrap();
        let w = 300;
        let mut c = Canvas::new(w, 1.0, hex("#c8b89a"));
        let mut under = Held::new(Tool::filbert(60.0), 2);
        for k in 0..12 {
            under.reload(Paint::body(hex("#304060")), 1.0);
            let y = 40.0 + k as f32 * 80.0;
            c.drag(&mut under, &Gesture::new(vec![(0.0, y), (1000.0, y)]).pressure(1.0, 1.0), None);
        }
        let check = |c: &Canvas, before: &[f32], r: (usize, usize, usize, usize), what: &str| {
            // (the stroke's id is one below the counter next_stroke_ids leaves)
            let id = c.wet.current - 1;
            for i in 0..c.wet.vol.len() {
                if c.wet.touched[i] == id || c.wet.stroke[i] == id || before[i] != c.wet.vol[i] {
                    let (x, y) = (i % c.f.w, i / c.f.w);
                    assert!(x >= r.0 && x < r.2 && y >= r.1 && y < r.3, "{what} {tool:?}: ({x},{y}) outside {r:?}");
                }
            }
        };
        let pts = vec![(400.0, 500.0), (600.0, 450.0), (650.0, 650.0)];
        let before = c.wet.vol.clone();
        let mut h = Held::new(tool.clone(), 9);
        h.load(Paint::body(hex("#d0c060")), 1.0);
        c.drag(&mut h, &Gesture::new(pts.clone()).pressure(1.0, 1.0).shake(2.0), None);
        let r = footprint(&tool, &pts, 2.0, c.f.scale, c.f.w, c.f.h).unwrap();
        check(&c, &before, r, "drag");
        let before = c.wet.vol.clone();
        let t = Touch::at(300.0, 300.0).pressure(1.0).drag(30.0, -20.0).twist(2.0);
        c.touch(&mut h, &t, None);
        let r = crate::bristle::touch_footprint(&tool, &t, c.f.scale, c.f.w, c.f.h).unwrap();
        check(&c, &before, r, "touch");
    }
}

#[test]
fn clipped_plough_stays_inside_mask() {
    let mut c = Canvas::new(400, 1.0, hex("#c8b89a"));
    let m = Mask::from_fn(c.frame(), |x, _| if x < 500.0 { 1.0 } else { 0.0 });
    let mut h = Held::new(Tool { push: 0.3, ..Tool::hog_flat(30.0) }, 4);
    for k in 0..4 {
        h.reload(Paint::body(hex("#304060")), 1.0);
        let x = 470.0 + k as f32 * 3.0;
        c.drag(&mut h, &Gesture::new(vec![(x, 100.0), (x, 900.0)]).pressure(1.0, 1.0), Some(&m));
    }
    let outside: usize = (0..c.wet.vol.len()).filter(|&i| m.data[i] == 0.0 && c.wet.vol[i] > 0.0).count();
    assert_eq!(outside, 0);
}

/// Leveling moves wet paint, it neither destroys it nor makes it from the
/// dry relief underneath (cases from the adversarial review).
#[test]
fn settle_conserves_paint() {
    let setup = |relief: &dyn Fn(usize, usize) -> f32| {
        let mut c = Canvas::new(101, 1.0, hex("#808080")).with_size_mm(10.1);
        for y in 0..c.f.h {
            for x in 0..c.f.w {
                c.height[y * c.f.w + x] = relief(x, y);
            }
        }
        c
    };
    let (w, h) = (101usize, 101usize);
    let check = |name: &str, c: &mut Canvas, add: Vec<f32>, stiff: f32| {
        let before: f32 = add.iter().sum();
        let t = c.settle((0, 0, w, h), &add, &vec![stiff; w * h]);
        let after: f32 = t.iter().sum();
        assert!((after - before).abs() <= before * 0.03, "{name}: {before} -> {after}");
        assert!(t.iter().zip(&add).all(|(&ti, &a)| ti >= 0.0 && (a > 0.0 || ti == 0.0)), "{name}: paint on dry pixels");
    };
    // one fluid deposit on a flat surface
    let mut c = setup(&|_, _| 0.0);
    let mut add = vec![0.0; w * h];
    add[50 * w + 50] = 25.0;
    check("isolated", &mut c, add, 0.05);
    // a deposit in a one-pixel hole among dry 160 µm relief
    let mut c = setup(&|x, y| if x == 50 && y == 50 { 0.0 } else { 160.0 });
    let mut add = vec![0.0; w * h];
    add[50 * w + 50] = 25.0;
    check("depression", &mut c, add, 0.05);
    // a thin uniform layer over alternating columns: pools, conserved
    let mut c = setup(&|x, _| if x % 2 == 0 { 160.0 } else { 0.0 });
    check("columns", &mut c, vec![5.0; w * h], 0.05);
    // stiff paint barely moves
    let mut c = setup(&|x, _| if x % 2 == 0 { 160.0 } else { 0.0 });
    check("columns stiff", &mut c, vec![5.0; w * h], 1.0);
}



/// A brushed ground lays about the thickness it asks for, and none at 0.
#[test]
fn brushed_ground_honors_thickness() {
    use crate::style::{Apply, Ground};
    let mean_um = |um: f32| {
        let mut st = Style::friedrich();
        st.ground = vec![Ground { color: hex("#a9785a"), hiding: 0.8, um, stiff: 0.35, apply: Apply::Brush }];
        let c = st.prepare(300, 1.5, 3);
        c.film.iter().sum::<f32>() / c.film.len() as f32 * crate::surface::COAT_UM
    };
    assert_eq!(mean_um(0.0), 0.0);
    for want in [30.0f32, 90.0] {
        let got = mean_um(want);
        assert!((got - want).abs() < want * 0.2, "asked {want} µm, got {got}");
    }
}

#[test]
fn palette_mixes_what_it_can() {
    use crate::palette::Palette;
    let p = Palette::friedrich_1820();
    // a tube's own color is reachable exactly
    for t in &p.tubes {
        let m = p.mix(t.color);
        assert!(m.error < 0.01, "{}: {} ({})", t.name, m.error, p.recipe(&m));
    }
    // a mid gray from white and black
    let g = p.mix([0.2, 0.2, 0.2]);
    assert!(g.error < 0.03, "{} {}", g.error, p.recipe(&g));
    // a saturated green is out of this palette's gamut: comes out duller
    let green = [0.05, 0.6, 0.1];
    let m = p.mix(green);
    let (a, b) = (crate::color::to_oklab(green), crate::color::to_oklab(m.color));
    assert!((b[1].hypot(b[2])) < (a[1].hypot(a[2])) * 0.8, "chroma {} vs {}", b[1].hypot(b[2]), a[1].hypot(a[2]));
    // fractions sum to 1, deterministic
    assert!((m.parts.iter().map(|x| x.1).sum::<f32>() - 1.0).abs() < 1e-4);
    assert_eq!(p.recipe(&p.mix(green)), Palette::friedrich_1820().recipe(&m));
}


/// Cutting in keeps paint within about a brush width of the region, and
/// fills forms narrower than the body brush.
#[test]
fn cut_in_stays_near_the_region() {
    let st = Style::friedrich();
    let mut c = Canvas::new(500, 1.0, hex("#c8b89a")).with_linen(crate::surface::Linen::fine(2));
    let (cx, cy, r) = (500.0f32, 500.0f32, 120.0f32);
    let disc = Mask::from_fn(c.frame(), move |x, y| crate::smoothstep(0.6, -0.6, ((x - cx).powi(2) + (y - cy).powi(2)).sqrt() - r));
    // a spire narrower than the body brush
    let spire = Mask::from_fn(c.frame(), |x, y| if (x - 800.0).abs() < 4.0 && y > 200.0 && y < 700.0 { 1.0 } else { 0.0 });
    let edge = Tool::round_sable(2.2);
    c.work(&disc, &st.body().color(|_, _| hex("#303038")).cut_in(edge.clone()), 5);
    c.work(&spire, &st.body().color(|_, _| hex("#303038")).cut_in(edge), 6);
    let (w, s) = (c.f.w, c.f.scale);
    let (mut total, mut far, mut spire_in) = (0.0f64, 0.0f64, 0.0f64);
    for (i, &v) in c.wet.vol.iter().enumerate() {
        let (x, y) = ((i % w) as f32 / s + 0.5 / s, (i / w) as f32 / s + 0.5 / s);
        total += v as f64;
        let d_disc = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt() - r;
        let d_spire = if y > 200.0 && y < 700.0 { (x - 800.0).abs() - 4.0 } else { f32::MAX };
        if d_disc.min(d_spire) > 4.0 {
            far += v as f64;
        }
        if d_spire < 0.0 {
            spire_in += v as f64;
        }
    }
    assert!(far < total * 0.01, "paint far outside: {:.3}%", far / total * 100.0);
    assert!(spire_in > 0.0, "the thin spire got no paint");
}

/// Largest and mean color difference of `c` (a crop render) to the same
/// pixels of `whole` over the kept crop, and the largest height difference.
fn crop_diff(c: &Canvas, whole: &Canvas) -> (f32, f32, f32) {
    let (f, k) = (c.f, c.keep);
    let (mut mx, mut sum, mut hmx, mut n) = (0.0f32, 0.0f64, 0.0f32, 0usize);
    for y in k.1..k.3 {
        for x in k.0..k.2 {
            let (i, j) = (y * f.w + x, (y + f.y0) * whole.f.w + x + f.x0);
            for ch in 0..3 {
                let d = (c.px[i][ch] - whole.px[j][ch]).abs();
                mx = mx.max(d);
                sum += d as f64;
            }
            hmx = hmx.max((c.height[i] - whole.height[j]).abs());
            n += 3;
        }
    }
    (mx, (sum / n as f64) as f32, hmx)
}

/// A canvas with linen, a knife ground, then (`strokes`) a pass of short
/// hog strokes and a pass of long ones, whole or cropped.
fn crop_scene(crop: Option<crate::canvas::Crop>, strokes: usize) -> Canvas {
    let st = Style::friedrich();
    let mut c = Canvas::new_window(400, 1.4, st.raw, crop).with_size_mm(st.width_mm).with_linen(crate::surface::Linen { seed: 1, ..st.linen });
    c.prime(st.ground[0].color, 0.8, 110.0, 0.25, 0.35, 5);
    let all = Mask::from_fn(c.frame(), |_, _| 1.0);
    if strokes >= 1 {
        c.work(&all, &Handling::new(Tool::hog_flat(12.0)).color(|x, _| if x < 450.0 { hex("#a9785a") } else { hex("#50607a") }).paint(0.8, 0.4).length(20.0, 40.0).coverage(2.0), 7);
        c.dry();
    }
    if strokes >= 2 {
        c.work(&all, &Handling::new(Tool::filbert(20.0)).color(|_, y| if y < 300.0 { hex("#6a4c34") } else { hex("#c9b48e") }).paint(0.5, 0.4).length(150.0, 300.0).coverage(1.5), 8);
        c.dry();
    }
    c.relief(0.5, 0.05);
    c
}

/// A crop render is the same picture as the whole render there: the
/// support and grounds exactly, brushwork closely (strokes are planned on
/// the whole canvas; outside the window a brush can only be estimated, see
/// notes/workflow.md), and the difference falls as the margin grows.
#[test]
fn crop_matches_whole() {
    use crate::canvas::Crop;
    let crop = |m: f32| Some(Crop { units: [300.0, 250.0, 460.0, 400.0], margin: m });
    let (w0, c0) = (crop_scene(None, 0), crop_scene(crop(12.0), 0));
    let d0 = crop_diff(&c0, &w0);
    assert!(d0.0 < 1e-5 && d0.2 < 0.01, "linen + ground differ: {d0:?}");
    let w2 = crop_scene(None, 2);
    let near = crop_diff(&crop_scene(crop(20.0), 2), &w2);
    let far = crop_diff(&crop_scene(crop(120.0), 2), &w2);
    eprintln!("crop vs whole (color max, mean, height max µm): margin 20 {near:?}, margin 120 {far:?}");
    assert!(near.1 < 0.005 && far.1 < 0.0015 && far.1 < near.1 && far.0 < near.0, "crop drifts from the whole render: {near:?} {far:?}");
}

/// Strokes are seeded past the canvas edges, so a lay-in covers the border
/// as well as the middle (the ground used to show along the edges).
#[test]
fn work_covers_the_edges() {
    let st = Style::friedrich();
    let mut c = Canvas::new(200, 1.0, hex("#ffffff"));
    let all = Mask::from_fn(c.f, |_, _| 1.0);
    c.work(&all, &st.broad().color(|_, _| hex("#304060")).coverage(4.0), 3);
    let (w, h) = (c.f.w, c.f.h);
    let b = (w / 40).max(2);
    let mut edge = (0, 0);
    for y in 0..h {
        for x in 0..w {
            if x < b || y < b || x >= w - b || y >= h - b {
                edge.1 += 1;
                if c.wet.vol[y * w + x] > 0.02 {
                    edge.0 += 1;
                }
            }
        }
    }
    let frac = edge.0 as f32 / edge.1 as f32;
    assert!(frac > 0.97, "border covered {frac}");
}

/// Stroke geometry is hand-like by default and ruler-straight on request.
#[test]
fn strokes_bow_unless_ruled() {
    use crate::bristle::Tool;
    let bows = |h: &Handling| {
        let drift = crate::noise::Fbm::new(1, 3, 300.0);
        let mut rng = crate::rng::Rng::new(9);
        let mut v: Vec<f32> = (0..200)
            .map(|_| {
                let p = crate::handling::hand_trace_for_test(h, &drift, 500.0, 500.0, 150.0, &mut rng);
                let (a, b, m) = (p[0], p[p.len() - 1], p[p.len() / 2]);
                let chord = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
                // distance of the middle from the chord, relative to its length
                ((b.0 - a.0) * (a.1 - m.1) - (a.0 - m.0) * (b.1 - a.1)).abs() / chord / chord
            })
            .collect();
        v.sort_by(f32::total_cmp);
        v[v.len() / 2]
    };
    let hand = bows(&Handling::new(Tool::filbert(10.0)).curve(0.08, 0.0).drift(0.0, 100.0));
    let ruler = bows(&Handling::new(Tool::filbert(10.0)).ruler());
    assert!(hand > 0.03 && ruler < 1e-3, "median bow: hand {hand}, ruler {ruler}");
}
