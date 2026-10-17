use super::*;
use crate::canvas::Crop;
use crate::color::hex;
use crate::wet::Paint;
use std::io::Cursor;

const DAY: f32 = 24.0 * 60.0;
const PAINTED: [f32; 4] = [100.0, 100.0, 900.0, 900.0];

#[test]
fn budgeted_network_keeps_the_unbounded_arm_geometry() {
    let k = crate::crack::Cracks { island_mm: Some(FINEST_MM), ground_um: Some(300.0), corners: false, grain: 0.0, ..crate::crack::Cracks::aged(11) };
    let original = crate::crack::network(&k, [80.0; 2], [0.7; 2]);
    let budget = crate::crack::Budget::limited([80.0; 2], NETWORK_MEMORY_BUDGET);
    let (bounded, _) = crate::crack::network_with_budget(&k, [80.0; 2], [0.0; 2], [80.0; 2], [0.7; 2], 1.0 / 6.0, budget, false);
    assert_eq!(original.size_mm, bounded.size_mm);
    assert_eq!(original.arms.len(), bounded.arms.len());
    for (a, b) in original.arms.iter().zip(&bounded.arms) {
        assert_eq!((a.crack, a.end, a.opening.to_bits(), a.generation), (b.crack, b.end, b.opening.to_bits(), b.generation));
        assert_eq!(a.pts.iter().map(|p| p.map(f32::to_bits)).collect::<Vec<_>>(), b.pts.iter().map(|p| p.map(f32::to_bits)).collect::<Vec<_>>());
    }
}

#[test]
fn oversized_physical_canvases_refuse_traction_before_allocating_the_grid() {
    let mut c = prepared(80, true, false).with_size_mm(5000.0);
    let error = crate::crack::with_traction_memory_reserved_for_test(1016 << 20, || {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| c.wait(60.0 * DAY))).unwrap_err()
    });
    let message = error.downcast_ref::<String>().unwrap();
    assert!(message.contains("5000 × 5000 mm"), "{message}");
    assert!(message.contains("1024 MiB memory budget"), "{message}");
    assert!(c.traction.net.0.is_none());
}

#[test]
fn memory_preflight_uses_the_whole_physical_canvas_even_for_a_small_crop() {
    let crop = Crop { units: [450.0, 450.0, 550.0, 550.0], margin: 0.0 };
    let c = canvas(80, true, Some(crop)).with_size_mm(5000.0);
    let error = crate::crack::with_traction_memory_reserved_for_test(1016 << 20, || c.check_traction_memory().unwrap_err());
    assert!(error.contains("5000 × 5000 mm"), "{error}");
    assert!(c.with_traction(false).check_traction_memory().is_ok());
}

#[test]
fn catching_a_memory_panic_does_not_hide_it_from_the_chunk_guard() {
    let mut c = prepared(80, true, false).with_size_mm(5000.0);
    let error = crate::crack::with_traction_memory_reserved_for_test(1016 << 20, || crate::crack::guard_traction_memory(|| {
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| c.wait(60.0 * DAY))).is_err());
    }).unwrap_err());
    assert!(error.contains("1024 MiB memory budget"), "{error}");
    assert!(crate::crack::guard_traction_memory(|| ()).is_ok());
}

#[test]
fn a_refused_speculative_region_build_does_not_fail_the_chunk() {
    let refuse = || -> () { crate::crack::Budget::limited([10.0; 2], 1).refused() };
    let speculative = crate::crack::guard_traction_memory(|| assert!(crate::crack::speculatively(refuse).is_err()));
    assert!(speculative.is_ok());
    // a refusal caught any other way still fails the chunk
    let caught = crate::crack::guard_traction_memory(|| assert!(std::panic::catch_unwind(refuse).is_err()));
    assert!(caught.is_err());
}

fn canvas(px: usize, traction: bool, crop: Option<Crop>) -> Canvas {
    Canvas::new_window(px, 1.0, hex("#d8cdb4"), crop).with_size_mm(80.0).with_engine(crate::ENGINE).with_traction(traction)
}

fn undercoat() -> Paint {
    Paint::body(hex("#201c18")).with_stiff(0.3).with_drying(0.5)
}
fn topcoat() -> Paint {
    Paint::body(hex("#ece6d6")).with_stiff(0.8).with_drying(2.0)
}

/// Uniform physical film, as a brush deposits it; no drying state is seeded.
fn film(c: &mut Canvas, paint: Paint, um: f32, bounds: [f32; 4], stroke: u32) {
    let f = c.f;
    let p = |u: f32| (u * f.scale).round().max(0.0) as usize;
    let Some((x0, y0, x1, y1)) = f.clip((p(bounds[0]), p(bounds[1]), p(bounds[2]), p(bounds[3]))) else {
        return;
    };
    let lat = paint.latent();
    for y in y0..y1 {
        for x in x0..x1 {
            let i = y * f.w + x;
            c.wet.vol[i] = um / COAT_UM;
            c.wet.lat[i] = lat;
            c.wet.hide[i] = paint.prop();
            c.wet.stroke[i] = stroke;
        }
    }
    c.wet.current = stroke;
    c.wet.touch(x0, y0, x1, y1);
}

fn prepared(px: usize, traction: bool, hardened: bool) -> Canvas {
    let mut c = canvas(px, traction, None);
    film(&mut c, undercoat(), 200.0, PAINTED, 1);
    c.dry();
    if hardened {
        c.wait(3.0 * 365.0 * DAY);
    }
    film(&mut c, topcoat(), 25.0, PAINTED, 2);
    c
}

fn region(c: &Canvas) -> impl Iterator<Item = usize> + '_ {
    (0..c.px.len()).filter(|&i| {
        let (x, y) = (c.f.ux(i % c.f.w), c.f.uy(i / c.f.w));
        (150.0..850.0).contains(&x) && (150.0..850.0).contains(&y)
    })
}

fn luminance(p: Rgb) -> f32 {
    0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2]
}
fn mean_luminance(c: &Canvas) -> f32 {
    let (sum, n) = region(c).fold((0.0f64, 0), |(sum, n), i| (sum + luminance(c.px[i]) as f64, n + 1));
    (sum / n as f64) as f32
}
fn dark_share(c: &Canvas) -> f32 {
    let (dark, n) = region(c).fold((0, 0), |(dark, n), i| (dark + usize::from(luminance(c.px[i]) < 0.4), n + 1));
    dark as f32 / n as f32
}

fn surface_near(a: &Canvas, b: &Canvas) {
    assert_eq!(a.f, b.f);
    for i in 0..a.px.len() {
        let p = Surface::capture(a, i).values();
        let q = Surface::capture(b, i).values();
        for k in 0..p.len() {
            let tolerance = if k == 3 { 0.002 } else { 0.00002 };
            assert!((p[k] - q[k]).abs() <= tolerance, "pixel {i}, surface field {k}: {} vs {}", p[k], q[k]);
        }
    }
}

fn anchored_near(a: &Canvas, b: &Canvas) {
    assert_eq!(a.traction.soft.len(), b.traction.soft.len());
    for (i, (s, t)) in a.traction.soft.iter().zip(&b.traction.soft).enumerate() {
        assert!((s.amount - t.amount).abs() < 0.0002, "soft amount at {i}: {} vs {}", s.amount, t.amount);
        assert!((s.rate - t.rate).abs() < 1e-10, "through-rate at {i}");
        assert!((s.at - t.at).abs() < 0.02, "soft date at {i}: {} vs {}", s.at, t.at);
    }
    assert_eq!(a.traction.cracks.len(), b.traction.cracks.len());
    for (s, t) in a.traction.cracks.iter().zip(&b.traction.cracks) {
        assert_eq!(s.i, t.i);
        assert!(
            (s.set_at - t.set_at).abs() < 0.02 && (s.dry_at - t.dry_at).abs() < 0.02,
            "crack dates at {}",
            s.i
        );
        assert!(
            (s.shown - t.shown).abs() < 0.00002 && (s.hw_mm - t.hw_mm).abs() < 0.00002,
            "crack opening at {}",
            s.i
        );
        assert_eq!(s.d_mm, t.d_mm);
    }
}

fn saved(c: &Canvas) -> Vec<u8> {
    let mut bytes = Vec::new();
    c.write_state(&mut bytes, "").unwrap();
    bytes
}

#[test]
fn soft_undercoat_cracks_but_hardened_and_traction_off_controls_do_not() {
    let mut soft = prepared(180, true, false);
    let mut hard = prepared(180, true, true);
    let mut five = prepared(180, false, false);
    soft.wait(60.0 * DAY);
    hard.wait(60.0 * DAY);
    five.wait(60.0 * DAY);
    assert!(dark_share(&soft) > 0.01, "soft-undercoat dark share {}", dark_share(&soft));
    assert!(mean_luminance(&soft) + 0.02 < mean_luminance(&hard));
    assert!(dark_share(&hard) < 0.001 && hard.traction.cracks.is_empty());
    assert!(dark_share(&five) < 0.001 && five.traction.cracks.is_empty() && five.traction.soft.is_empty());
    surface_near(&hard, &five);
}

#[test]
fn first_dry_retains_substrate_softness_and_matches_waiting() {
    let mut dry = canvas(120, true, None);
    film(&mut dry, undercoat(), 200.0, PAINTED, 1);
    let mut wait = dry.clone();
    assert!(dry.wet.clock.px.is_empty());
    dry.dry();
    assert!(dry.wet.clock.tacky.is_none());
    assert!(dry.wet.clock.px.iter().all(|p| p.sub >= 1.0));
    assert!(dry.traction.soft.iter().any(|s| s.at(dry.clock()) > SOFT0));
    wait.wait(dry.clock() as f32);
    surface_near(&dry, &wait);
    anchored_near(&dry, &wait);
    for c in [&mut dry, &mut wait] {
        film(c, topcoat(), 25.0, PAINTED, 2);
        c.wait(60.0 * DAY);
    }
    let mut control = prepared(120, false, false);
    control.wait(60.0 * DAY);
    assert!(mean_luminance(&dry) + 0.01 < mean_luminance(&control), "first dry left no cracking");
    surface_near(&dry, &wait);
    anchored_near(&dry, &wait);
}

#[test]
fn cracking_survives_long_and_fractionally_split_waits() {
    let start = prepared(120, true, false);
    for total in [60.0 * DAY, 10.0 * 365.0 * DAY] {
        let mut long = start.clone();
        let mut split = start.clone();
        long.wait(total);
        // Exact binary fractions; the last part completes the same interval.
        for dt in [0.125, 719.875, 3.0 * DAY, total - 720.0 - 3.0 * DAY] {
            split.wait(dt);
        }
        assert_eq!(long.clock(), split.clock());
        let mut control = prepared(120, false, false);
        control.wait(total);
        assert!(
            mean_luminance(&long) + 0.01 < mean_luminance(&control),
            "long wait suppressed cracking at {total} minutes"
        );
        surface_near(&long, &split);
        anchored_near(&long, &split);
    }
}

#[test]
fn checkpoint_continues_opening_and_threads_do_not_change_it() {
    let run = || {
        let mut c = prepared(120, true, false);
        c.wait(5.0 * DAY);
        assert!(!c.traction.cracks.is_empty());
        let early = mean_luminance(&c);
        let bytes = saved(&c);
        let (mut resumed, _) = Canvas::read_state(&mut Cursor::new(&bytes)).unwrap();
        assert_eq!(saved(&resumed), bytes);
        c.wait(55.0 * DAY);
        resumed.wait(55.0 * DAY);
        assert!(mean_luminance(&c) < early - 0.01, "cracks did not widen after checkpoint");
        assert_eq!(saved(&c), saved(&resumed));
        assert_eq!(c.px, resumed.px);
        saved(&c)
    };
    let one = rayon::ThreadPoolBuilder::new().num_threads(1).build().unwrap().install(run);
    let four = rayon::ThreadPoolBuilder::new().num_threads(4).build().unwrap().install(run);
    assert_eq!(one, four);
}

#[test]
fn different_films_and_a_recoat_preserve_wait_partitioning() {
    let mut long = canvas(120, true, None);
    film(&mut long, undercoat(), 200.0, PAINTED, 1);
    long.dry();
    film(&mut long, topcoat(), 0.75 * COAT_UM, [100.0, 100.0, 480.0, 900.0], 2);
    film(&mut long, topcoat().with_drying(0.9), 1.75 * COAT_UM, [520.0, 100.0, 900.0, 900.0], 3);
    let mut split = long.clone();
    long.wait(5.0 * DAY);
    for dt in [0.125, 719.875, 2.0 * DAY, 2.5 * DAY] {
        split.wait(dt);
    }
    assert!(!long.traction.cracks.is_empty());
    surface_near(&long, &split);
    for c in [&mut long, &mut split] {
        film(c, Paint::body(hex("#ca5842")).with_stiff(0.7).with_drying(1.5), 0.15 * COAT_UM, PAINTED, 4);
    }
    long.wait(120.0 * DAY);
    for dt in [0.125, 719.875, DAY, 118.5 * DAY] {
        split.wait(dt);
    }
    assert_eq!(long.clock(), split.clock());
    surface_near(&long, &split);
    anchored_near(&long, &split);
}

#[test]
fn crop_uses_the_whole_canvas_crack_coordinates() {
    let crop = Crop {
        units: [250.0, 230.0, 750.0, 770.0],
        margin: 150.0,
    };
    let mut whole = canvas(180, true, None);
    let mut window = canvas(180, true, Some(crop));
    for c in [&mut whole, &mut window] {
        film(c, undercoat(), 200.0, [0.0, 0.0, 1000.0, 1000.0], 1);
        c.dry();
        film(c, topcoat(), 25.0, [0.0, 0.0, 1000.0, 1000.0], 2);
        c.wait(60.0 * DAY);
    }
    assert!(window.f.x0 > 0 && window.f.y0 > 0 && !window.traction.cracks.is_empty());
    let (x0, y0, x1, y1) = window.keep;
    for y in y0..y1 {
        for x in x0..x1 {
            let i = y * window.f.w + x;
            let j = window.f.whole_index(i);
            assert_eq!(Surface::capture(&window, i), Surface::capture(&whole, j), "whole-canvas pixel {j}");
        }
    }
}

#[test]
fn network_cache_tracks_physical_size_and_support_seed() {
    let mut c = canvas(120, true, None);
    let first = c.traction_net([0, 0], 0);
    let mut wider = c.clone().with_size_mm(120.0);
    let resized = wider.traction_net([0, 0], 0);
    assert!(!std::sync::Arc::ptr_eq(&first, &resized));
    assert_eq!(resized.size_mm, [120.0; 2]);
    let mut linen = c.with_linen(crate::surface::Linen::fine(51));
    let reseeded = linen.traction_net([0, 0], 0);
    assert!(!std::sync::Arc::ptr_eq(&first, &reseeded));
    assert_ne!(first.seed, reseeded.seed);
    let (mut resumed, _) = Canvas::read_state(&mut Cursor::new(saved(&wider))).unwrap();
    let restored = resumed.traction_net([0, 0], 0);
    assert_eq!(resized.size_mm, restored.size_mm);
    assert_eq!(resized.seed, restored.seed);
    for point in [[10.0, 15.0], [59.0, 83.0], [111.0, 105.0]] {
        assert_eq!(resized.nearest(point, 4), restored.nearest(point, 4));
    }
}

#[test]
fn canvas_snapshots_do_not_pin_an_evicted_crack_region() {
    let mut c = canvas(80, true, None);
    let cached = std::sync::Arc::downgrade(&c.traction_net([0, 0], 0));
    let snapshot = c.clone();
    assert!(snapshot.traction.net.0.is_none());
    c.traction.net.0 = None;
    assert!(cached.upgrade().is_none());
}

#[test]
fn large_canvas_regions_match_a_restored_crop() {
    let crop = Crop { units: [400.0, 400.0, 600.0, 600.0], margin: 0.0 };
    let mut whole = canvas(96, true, None).with_size_mm(640.0);
    let mut window = canvas(96, true, Some(crop)).with_size_mm(640.0);
    assert!(whole.clone().with_size_mm(5000.0).check_traction_memory().is_ok());
    for c in [&mut whole, &mut window] {
        film(c, undercoat(), 200.0, PAINTED, 1);
        c.dry();
        film(c, topcoat(), 25.0, PAINTED, 2);
    }
    let (mut window, _) = Canvas::read_state(&mut Cursor::new(saved(&window))).unwrap();
    whole.wait(60.0 * DAY);
    window.wait(60.0 * DAY);
    assert!(!window.traction.cracks.is_empty());
    assert_eq!(window.traction.net.0.as_ref().unwrap().tile, [1, 1]);
    for i in 0..window.px.len() {
        assert_eq!(Surface::capture(&window, i), Surface::capture(&whole, window.f.whole_index(i)), "crop pixel {i}");
    }
}

#[test]
fn nearest_region_query_matches_a_full_segment_search() {
    let mut c = canvas(80, true, None);
    let net = c.traction_net([0, 0], 0);
    let mut checked = 0;
    for i in 0..160 {
        let p = [80.0 * crate::rng::hash2(i, 0, 73), 80.0 * crate::rng::hash2(i, 1, 73)];
        let gmax = (i % 5) as u8;
        let reference = net.segs.iter().filter(|s| s.4 <= gmax).map(|&(a, b, wa, wb, _)| {
            let ab = [b[0] - a[0], b[1] - a[1]];
            let l2 = (ab[0] * ab[0] + ab[1] * ab[1]).max(1e-12);
            let t = (((p[0] - a[0]) * ab[0] + (p[1] - a[1]) * ab[1]) / l2).clamp(0.0, 1.0);
            let d = ((p[0] - a[0] - t * ab[0]).powi(2) + (p[1] - a[1] - t * ab[1]).powi(2)).sqrt();
            (d, wa + (wb - wa) * t)
        }).min_by(|a, b| a.0.total_cmp(&b.0));
        if let Some(expected) = reference.filter(|p| p.0 < net.cell) {
            let actual = net.nearest(p, gmax).unwrap();
            assert!((actual.0 - expected.0).abs() < 1e-5, "distance at {p:?}: {actual:?} vs {expected:?}");
            assert!((actual.1 - expected.1).abs() < 1e-5, "opening at {p:?}: {actual:?} vs {expected:?}");
            checked += 1;
        }
    }
    assert!(checked > 50);
}

#[test]
fn overlap_budget_is_independent_of_a_warm_region_cache() {
    let mut warm = canvas(32, true, None).with_size_mm(640.0);
    let patch = [100.0, 100.0, 200.0, 200.0];
    film(&mut warm, undercoat(), 200.0, patch, 1);
    warm.dry();
    film(&mut warm, topcoat(), 25.0, patch, 2);
    let cost = warm.traction_net([0, 0], 0).build_cost;
    let mut cold = warm.clone();
    for c in [&mut warm, &mut cold] {
        crate::crack::with_traction_memory_reserved_for_test(NETWORK_MEMORY_BUDGET - cost - (2 << 20), || c.wait(60.0 * DAY));
    }
    assert!(!warm.traction.cracks.is_empty());
    assert_eq!(saved(&warm), saved(&cold));
}

#[test]
fn cracks_restore_actual_textured_substrate_and_all_material_fields() {
    let mut c = canvas(120, true, None);
    film(&mut c, undercoat(), 200.0, PAINTED, 1);
    c.dry();
    for i in 0..c.height.len() {
        c.height[i] += 12.0 * ((i % c.f.w) as f32 * 0.4).sin();
    }
    let under: Vec<Surface> = (0..c.px.len()).map(|i| Surface::capture(&c, i)).collect();
    film(&mut c, topcoat(), 25.0, PAINTED, 2);
    // The lean top has different gloss, making its exposure observable too.
    for hd in &mut c.wet.hide {
        hd[4] = 0.25;
    }
    c.wait(3.0 * DAY);
    assert!(!c.traction.cracks.is_empty());
    for crack in &c.traction.cracks {
        assert_eq!(crack.under, under[crack.i], "substrate at {}", crack.i);
    }
    let indices: Vec<usize> = c.traction.cracks.iter().map(|p| p.i).collect();
    // Force full pixel coverage to check the interpolation's material endpoint.
    let pm = c.px_mm();
    for crack in &mut c.traction.cracks {
        crack.d_mm = 0.0;
        crack.hw_mm = 10.0 * pm;
    }
    c.wait(60.0 * DAY);
    for i in indices {
        assert_eq!(Surface::capture(&c, i), under[i], "fully exposed substrate at {i}");
    }
}

#[test]
fn glaze_and_varnish_are_preserved_by_later_waits() {
    let mut start = prepared(120, true, false);
    start.wait(5.0 * DAY);
    assert!(!start.traction.cracks.is_empty());
    for glaze in [false, true] {
        let mut c = start.clone();
        if glaze {
            c.glaze(&crate::pigment::Pigment::transparent(hex("#304060")), None, |_, _| 0.3);
        } else {
            c.varnished();
        }
        let surface: Vec<Surface> = (0..c.px.len()).map(|i| Surface::capture(&c, i)).collect();
        c.wait(60.0 * DAY);
        assert!(c.traction.cracks.is_empty());
        assert_eq!(surface, (0..c.px.len()).map(|i| Surface::capture(&c, i)).collect::<Vec<_>>());
    }
}

#[test]
fn invalid_traction_materials_indices_and_dates_are_rejected() {
    let surface = Surface {
        color: [0.2, 0.3, 0.4],
        height: 10.0,
        film: 1.0,
        gloss: 0.4,
        absorb: 0.0,
    };
    let crack = |i| CrackPx {
        i,
        under: surface,
        top: surface,
        d_mm: 0.1,
        hw_mm: 0.2,
        shown: 0.3,
        set_at: 2.0,
        dry_at: 4.0,
    };
    let state = Traction {
        soft: vec![
            Soft {
                amount: 50.0,
                rate: 1e-4,
                at: 1.0
            };
            2
        ],
        cracks: vec![crack(0), crack(1)],
        net: NetCache::default(),
    };
    let mut valid = Vec::new();
    state.write(&mut valid, 2, 10.0).unwrap();
    let reject = |bytes: Vec<u8>| {
        assert_eq!(Traction::read(&mut &bytes[..], 2, 10.0).err().unwrap().kind(), io::ErrorKind::InvalidData);
    };
    // mark, version, softness flag, crack count, first index, second index.
    for (offset, value) in [(0, 0), (8, 0), (8, 2), (16, 2), (72, 3), (80, 2), (172, 0)] {
        let mut bytes = valid.clone();
        bytes[offset..offset + 8].copy_from_slice(&u64::to_le_bytes(value));
        reject(bytes);
    }
    // Soft amount/rate/date and crack setting/dry dates.
    for (offset, value) in [
        (24, f64::NAN),
        (24, -1.0),
        (32, -1.0),
        (32, 0.0),
        (40, 11.0),
        (156, 11.0),
        (164, 1.0),
        (164, 2.0),
    ] {
        let mut bytes = valid.clone();
        bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
        reject(bytes);
    }
    // RGB, film, gloss, distance, width and shown coverage.
    for (offset, value) in [(88, f32::NAN), (88, 1.1), (104, -1.0), (108, 1.1), (144, -1.0), (148, -1.0), (152, 1.1)] {
        let mut bytes = valid.clone();
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        reject(bytes);
    }
    let no_substrate = Traction { soft: Vec::new(), ..state };
    assert!(no_substrate.write(&mut Vec::new(), 2, 10.0).is_err());
}

/// Reproducible synthetic evidence; the normal suite never writes studies.
#[test]
#[ignore = "writes the small traction study when TRACTION_STUDY_OUT is set"]
fn write_traction_study() {
    let Some(out) = std::env::var_os("TRACTION_STUDY_OUT") else { return };
    let out = std::path::PathBuf::from(out);
    std::fs::create_dir_all(&out).unwrap();
    let mut csv = String::from("case,traction,undercoat_um,topcoat_um,extra_undercoat_wait_minutes,topcoat_wait_minutes,dark_share,mean_luminance\n");
    let mut panels = image::RgbImage::new(960, 320);
    for (column, (label, traction, hardened)) in [("soft-undercoat", true, false), ("hardened-undercoat", true, true), ("traction-off", false, false)]
        .into_iter()
        .enumerate()
    {
        let mut c = prepared(320, traction, hardened);
        c.wait(60.0 * DAY);
        csv.push_str(&format!(
            "{label},{traction},200,25,{},{},{:.8},{:.8}\n",
            if hardened { 3.0 * 365.0 * DAY } else { 0.0 },
            60.0 * DAY,
            dark_share(&c),
            mean_luminance(&c)
        ));
        let path = out.join(format!("{label}.png"));
        c.save(&path).unwrap();
        let panel = image::open(path).unwrap().to_rgb8();
        image::imageops::replace(&mut panels, &panel, column as i64 * 320, 0);
    }
    panels.save(out.join("traction-controls.png")).unwrap();
    std::fs::write(out.join("measurements.csv"), csv).unwrap();
}

#[test]
fn a_gel_rounding_boundary_can_be_checkpointed() {
    let mut c = canvas(40, true, None);
    let paint = Paint::body(hex("#606060")).with_stiff(0.5).with_drying(1.0);
    film(&mut c, paint, 25.0, [0.0, 0.0, 1000.0, 1000.0], 1);
    c.wait(f32::from_bits(2808.0f32.to_bits() - 1));
    assert!(c.wet.vol.iter().all(|&v| v == 0.0), "film rounded to gel and baked");
    assert!(c.traction.soft.iter().any(|s| s.amount > 0.0));
    let bytes = saved(&c);
    let (resumed, _) = Canvas::read_state(&mut Cursor::new(&bytes)).unwrap();
    assert_eq!(saved(&resumed), bytes);
}
