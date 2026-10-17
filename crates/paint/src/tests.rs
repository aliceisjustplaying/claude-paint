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

/// The drain switch: an absorbent ground drains a film to its drained
/// floor, as far as its pores allow: the pigment packs from the ground up,
/// the paint above keeping all its oil, so the surface's oil is the paint's
/// until the packed layer reaches it. What a brush takes off comes from the
/// top.
#[test]
fn an_absorbent_ground_drains_a_film_from_the_ground_up() {
    use crate::bristle::{ground_drain, surface_oil, take_from};
    // (a tube paint 40% oil by volume)
    let fresh = [0.8, 0.6, 1.0, 0.0, 1.0, 0.7, 0.6, 0.0, 0.4, 0.0];
    // two coats of tube paint: pigment 1.2 coats, oil 0.8, of which the
    // ground can take 0.8 × (1 − 0.6) = 0.32, 0.16 a coat. Taking 0.1
    // packs 0.625 coats of it into 0.525 coats at the floor's oil
    let (mut hd, mut cap) = (fresh, 0.1);
    let t = ground_drain(&mut cap, 2.0, &mut hd);
    assert!((t - 1.9).abs() < 1e-5 && cap == 0.0, "{t} {cap}");
    assert!((hd[7] - 0.525 / 1.9).abs() < 1e-5 && (hd[4] - (0.525 * 0.6 + 1.375) / 1.9).abs() < 1e-5, "{hd:?}");
    assert!((surface_oil(&hd) - 1.0).abs() < 1e-5, "the surface keeps its oil: {hd:?}");
    assert_eq!(hd[1], fresh[1], "the paint above the packed layer keeps its stiffness");
    // more pores than the film can give: down to the floor, all packed
    let mut cap = 1.0;
    let t = ground_drain(&mut cap, t, &mut hd);
    assert!((t - 1.68).abs() < 1e-5 && (cap - 0.78).abs() < 1e-5, "{t} {cap}");
    assert!((hd[4] - 0.6).abs() < 1e-5 && (hd[7] - 1.0).abs() < 1e-5 && (surface_oil(&hd) - 0.6).abs() < 1e-5, "{hd:?}");
    assert_eq!(ground_drain(&mut cap, t, &mut hd), t, "nothing below the floor");
    // nearly packed through, the surface still has the paint's own oil
    let nearly = [0.8, 0.6, 1.0, 0.0, 0.9995 * 0.6 + 0.0005, 0.7, 0.6, 0.9995, 0.4, 0.0];
    assert!((surface_oil(&nearly) - 1.0).abs() < 2e-3, "{}", surface_oil(&nearly));
    // packed through, it dries as lean as stiff paste (with the switch only)
    assert!(crate::drying::lean_of(true, &hd) == 1.0 && crate::drying::lean_of(false, &hd) == hd[1], "{hd:?}");
    // a coarse pigment's floor is below its packed oil: it drains further
    let (mut coarse, mut cap) = ([0.8, 0.6, 1.0, 0.0, 1.0, 0.8, 0.15, 0.0, 0.4, 0.0], 1.0);
    ground_drain(&mut cap, 2.0, &mut coarse);
    assert!((coarse[4] - 0.15).abs() < 1e-5, "{coarse:?}");
    // a brush takes the top first: the paint above the packed layer, as rich
    // as the surface; the film's packed share grows
    let (mut hd, mut cap) = (fresh, 0.16);
    ground_drain(&mut cap, 2.0, &mut hd);
    let (vol, f) = (2.0 - 0.16, hd[7]);
    // an oilier tube paint (60% oil by volume, as an oily earth's) gives
    // up more oil a coat as it packs to the same share of its own
    let (mut oily, mut cap) = ([0.8, 0.6, 1.0, 0.0, 1.0, 0.7, 0.6, 0.0, 0.6, 0.0], 1.0);
    let thinner = 2.0 - ground_drain(&mut cap, 2.0, &mut oily);
    assert!(thinner > 0.32 + 0.1 && (oily[4] - 0.6).abs() < 1e-5, "{thinner} {oily:?}");
    let got = take_from(&mut hd, vol, 0.4);
    assert!((got[4] - 1.0).abs() < 1e-5 && got[7] == 0.0, "{got:?}");
    assert!((hd[7] - f * vol / (vol - 0.4)).abs() < 1e-5 && (surface_oil(&hd) - 1.0).abs() < 1e-4, "{hd:?}");
}

#[test]
fn ground_drain_conserves_integrated_scattering() {
    use crate::bristle::ground_drain;
    for t in [1e-7, 0.1, 1.0, 3.0] {
        for scatter in [0.01, 1.0, 8.0] {
            for capacity in [0.0, 0.05, 10.0] {
                let mut hd = [scatter, 0.5, 1.0, 0.0, 1.0, 0.7, 0.4, 0.0, 0.5, 0.0];
                let mut cap = capacity;
                let mass = t * scatter;
                let left = ground_drain(&mut cap, t, &mut hd);
                assert!((left * hd[0] - mass).abs() <= mass * 1e-6, "{t} coats, capacity {capacity}: {mass} -> {}", left * hd[0]);
                assert!((t - left - (capacity - cap)).abs() < 1e-6);
                let packed = hd;
                let again = ground_drain(&mut cap, left, &mut hd);
                assert_eq!((again, hd), (left, packed));
            }
        }
    }
}

#[test]
fn drained_deposition_keeps_its_pigment_when_wet_and_baked() {
    for engine in [5, 7] {
        let mut c = Canvas::new(2, 1.0, [0.8; 3]).with_engine(engine).with_drain(true);
        c.ground_finish(1.0);
        let paint = Paint { scatter: 1.0, oil: 1.0, packed: 0.7, floor: 0.4, oil_volume: 0.5, ..Paint::body([0.2, 0.3, 0.4]) };
        let (lat, hd) = (paint.latent(), paint.prop());
        let mut mass = 0.0;
        for v in [1.0, 0.2, 0.4] {
            let sf = c.surf();
            for i in 0..c.wet.vol.len() {
                unsafe {
                    if engine == 7 {
                        crate::engine7::add::Film::of(&sf).add(i, v, &crate::engine7::add::Paint7::new(sf.lean, sf.drain, &lat, hd, 0.0));
                    } else {
                        sf.add(i, v, &lat, hd, 0.0);
                    }
                }
            }
            mass += v * paint.scatter;
            assert!((c.wet.vol[0] * c.wet.hide[0][0] - mass).abs() < 1e-6, "engine {engine}: {mass} -> {}", c.wet.vol[0] * c.wet.hide[0][0]);
        }
        assert!(c.wet.vol[0] < 1.6);
        c.wet.cover.fill(1.0);
        c.wet.dirty = Some((0, 0, c.f.w, c.f.h));
        let color = mixbox::latent_to_linear_float_rgb(&lat);
        let expected = crate::pigment::Pigment::masstone(color, mass).over([0.8; 3], 1.0);
        let wet = c.look_px(0);
        c.dry();
        assert_eq!(c.wet.vol[0], 0.0);
        for channel in 0..3 {
            assert!((wet[channel] - expected[channel]).abs() < 1e-6, "engine {engine}: wet {wet:?} != {expected:?}");
            assert!((c.px[0][channel] - expected[channel]).abs() < 1e-6, "engine {engine}: baked {:?} != {expected:?}", c.px[0]);
        }
    }
}

/// The drain switch on a chalk ground: a thin film of smalt, coarser than
/// the ground's pores, drains below its packed oil and dries matte; one of
/// Prussian blue, finer, keeps its packed layer saturated and dries
/// glossier; a film too thick for the ground to drain keeps its surface's
/// oil and dries about as glossy as on an oil ground. The paint keeps its
/// stiffness (engine 4's draw, without the switch, stiffens it).
#[test]
#[cfg(tube_box)]
fn an_absorbent_ground_drains_paint_as_its_pigment_packs() {
    use crate::palette::Palette;
    let pal = Palette::tube_box();
    let run = |tube: &str, absorbent: f32, passes: usize| {
        let i = pal.tubes.iter().position(|t| t.name == tube).unwrap();
        let mut c = Canvas::new(300, 1.0, [0.8; 3]).with_drain(true);
        c.ground_finish(absorbent);
        for _ in 0..passes {
            let mut h = Held::new(Tool::filbert(60.0), 3);
            h.load(pal.pile(vec![(i, 1.0)]).laid(0.0), if passes > 1 { 1.0 } else { 0.4 });
            c.drag(&mut h, &Gesture::new(vec![(100.0, 500.0), (900.0, 500.0)]).pressure(0.9, 0.9), None);
        }
        let i = c.f.index(500.0, 500.0);
        let (vol, hd) = (c.wet.vol[i], c.wet.hide[i]);
        c.dry();
        eprintln!("{tube} absorbent {absorbent} passes {passes}: {vol} coats {hd:?} gloss {}", c.gloss[i]);
        (vol, hd, c.gloss[i])
    };
    let (smalt, smalt_oil) = (run("smalt", 1.0, 1), run("smalt", 0.0, 1));
    let prussian = run("Prussian blue", 1.0, 1);
    assert!(smalt.1[7] > 0.95 && prussian.1[7] > 0.95, "thin films pack through: {smalt:?} {prussian:?}");
    assert!(smalt.1[4] < 0.2 && smalt.2 < 0.1, "smalt drains below its packed oil, matte: {smalt:?}");
    assert!(prussian.2 > smalt.2 + 0.1, "Prussian blue keeps its packed layer: {prussian:?} {smalt:?}");
    assert!((smalt.1[1] - smalt_oil.1[1]).abs() < 1e-5, "stiffness: {smalt:?} {smalt_oil:?}");
    let (thick, thick_oil) = (run("smalt", 1.0, 8), run("smalt", 0.0, 8));
    assert!(thick.1[7] < 0.7 && thick.2 > thick_oil.2 - 0.05, "a thick film keeps its surface: {thick:?} {thick_oil:?}");
}

/// The drain switch: only the fluid share of a film levels (`settle_for`'s
/// `fluid`): over the weave a film packed through follows the relief under
/// it, as even as it was laid; a fluid one gathers in the hollows.
#[test]
fn a_packed_film_follows_the_relief() {
    let film = |fluid: Option<f32>| {
        // (80 mm over 300 px: the threads resolved; a fluid glaze's body)
        let mut c = Canvas::new(300, 1.0, hex("#c8b89a")).with_size_mm(80.0).with_linen(crate::surface::Linen::fine(3));
        let (w, h) = (c.f.w, c.f.h);
        let add = vec![30.0f32; w * h];
        let stiff = vec![0.0f32; w * h];
        let sets = vec![crate::surface::SET_TIME; w * h];
        let fl = fluid.map(|f| vec![f; w * h]);
        c.settle_for((0, 0, w, h), &add, &stiff, &sets, true, fl.as_deref())
    };
    let spread = |t: &[f32]| t.iter().fold((f32::MAX, 0f32), |(a, b), &x| (a.min(x), b.max(x)));
    let (packed, fluid) = (spread(&film(Some(0.0))), spread(&film(None)));
    assert!(packed.1 - packed.0 < 0.01, "packed through: an even film {packed:?}");
    assert!(fluid.1 - fluid.0 > 1.0, "fluid: it gathers in the hollows {fluid:?}");
    assert_eq!(spread(&film(Some(1.0))), fluid);
}

/// The drain switch: a stroke baked with packed edges beside a still-fluid
/// center (`settle_for`'s `fluid`, as `bake` passes it, `1 − Prop[7]`): the
/// packed edges keep the thickness they packed at, however the fluid
/// center levels and is conserved beside them; a half-packed band keeps at
/// least its packed layer and its liquid levels over it; the stroke's total
/// is the paint laid. With nothing packed it bakes as without the switch.
#[test]
fn baking_leaves_packed_paint_where_it_packed() {
    // (80 mm over 300 px over the weave, as `a_packed_film_follows_the_relief`)
    let mut c = Canvas::new(300, 1.0, hex("#c8b89a")).with_size_mm(80.0).with_linen(crate::surface::Linen::fine(3));
    let (w, h) = (c.f.w, c.f.h);
    // across the stroke (x 100..200): packed through, half packed, fluid
    let band = |x: usize| match x {
        100..110 | 190..200 => Some((12.0f32, 0.0f32)),
        110..120 | 180..190 => Some((20.0, 0.5)),
        120..180 => Some((40.0, 1.0)),
        _ => None,
    };
    let (mut add, mut fluid) = (vec![0.0f32; w * h], vec![1.0f32; w * h]);
    for y in 100..200 {
        for x in 0..w {
            if let Some((a, f)) = band(x) {
                add[y * w + x] = a;
                fluid[y * w + x] = f;
            }
        }
    }
    let stiff = vec![0.0f32; w * h];
    let sets = vec![crate::surface::SET_TIME; w * h];
    let height = c.height.clone();
    let t = c.settle_for((0, 0, w, h), &add, &stiff, &sets, true, Some(&fluid));
    let (mut lo, mut hi) = (f32::MAX, 0f32);
    for y in 100..200 {
        for x in 100..200 {
            let i = y * w + x;
            match band(x) {
                Some((a, f)) if f == 0.0 => {
                    assert!(t[i] == a, "a packed edge keeps its thickness: {} at ({x}, {y})", t[i]);
                    // (and its surface follows the relief under it)
                    assert!((c.height[i] - height[i] - a).abs() < 1e-3, "({x}, {y})");
                }
                Some((a, f)) if f < 1.0 => assert!(t[i] >= a * (1.0 - f), "a half-packed band keeps its packed layer: {} at ({x}, {y})", t[i]),
                _ => (lo, hi) = (lo.min(t[i]), hi.max(t[i])),
            }
        }
    }
    assert!(hi - lo > 1.0, "the fluid center levels: {lo}..{hi}");
    let (laid, kept): (f64, f64) = (add.iter().map(|&v| v as f64).sum(), t.iter().map(|&v| v as f64).sum());
    assert!(((kept - laid) / laid).abs() < 1e-5, "paint is conserved: {kept} of {laid}");
    // with nothing packed: bit for bit as without the switch
    let flat = |fl: Option<&[f32]>| {
        let mut c = Canvas::new(300, 1.0, hex("#c8b89a")).with_size_mm(80.0).with_linen(crate::surface::Linen::fine(3));
        let t = c.settle_for((0, 0, w, h), &add, &stiff, &sets, true, fl);
        (t, c.height)
    };
    assert!(flat(Some(&vec![1.0; w * h])) == flat(None));
}
