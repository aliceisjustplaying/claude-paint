//! The Friedrich ground must read as a painted ground seen through thin
//! paint, not as a mechanical wood-grain of parallel horizontal streaks
//! (loop-2 critic: "a horizontal wood-grain streaking covers the sky and
//! snow, which looks like the tool rather than a painting choice"; see
//! notes/loop2_grain.md). These tests measure the direction of the ground's
//! relief: how much of its slope runs down the columns (horizontal ridges)
//! against along the rows.

use paint::{Crop, Ground, Style};

/// Directional statistics of a height field (µm), inside `pad`.
#[derive(Debug)]
#[allow(dead_code)] // (read through Debug in the diagnostics)
struct Grain {
    /// Mean squared slope down the columns over that along the rows, at the
    /// pixel scale (bristle striations). Horizontal streaks make it large;
    /// an unbiased surface is ~1.
    fine: f32,
    /// The same for the height blurred over ~1 mm: the ridges strokes leave
    /// at their edges, whose parallel rows made the wood-grain.
    coarse: f32,
    /// How high the surface stands above its ~1 mm mean at the 99.5th
    /// percentile, µm: stroke-end lumps, where thin paint runs off and the
    /// ground breaks through.
    peak: f32,
    /// RMS slope, µm per mm.
    rms: f32,
}

fn grain(h: &[f32], w: usize, ht: usize, pad: usize, px_mm: f32) -> Grain {
    let at = |x: usize, y: usize| h[y * w + x] as f64;
    let (mut ex, mut ey, mut n) = (0.0f64, 0.0f64, 0.0f64);
    for y in pad..ht - pad {
        for x in pad..w - pad {
            let dx = at(x + 1, y) - at(x - 1, y);
            let dy = at(x, y + 1) - at(x, y - 1);
            ex += dx * dx;
            ey += dy * dy;
            n += 1.0;
        }
    }
    // box blur of radius ~0.5 mm (a ~1 mm window), then the same slopes
    let r = ((0.5 / px_mm).round() as usize).max(1);
    let mut b = vec![0.0f64; w * ht];
    for y in r..ht - r {
        for x in r..w - r {
            let mut s = 0.0;
            for j in y - r..=y + r {
                for i in x - r..=x + r {
                    s += at(i, j);
                }
            }
            b[y * w + x] = s;
        }
    }
    let (mut bx, mut by) = (0.0f64, 0.0f64);
    let q = pad.max(r + 2);
    for y in q..ht - q {
        for x in q..w - q {
            let dx = b[y * w + x + 1] - b[y * w + x - 1];
            let dy = b[(y + 1) * w + x] - b[(y - 1) * w + x];
            bx += dx * dx;
            by += dy * dy;
        }
    }
    // lumps: how far the surface stands above its ~1 mm mean (99.5th pct)
    let a = ((2 * r + 1) * (2 * r + 1)) as f64;
    let mut up: Vec<f64> = Vec::new();
    for y in q..ht - q {
        for x in q..w - q {
            up.push(at(x, y) - b[y * w + x] / a);
        }
    }
    up.sort_by(|p, q| p.total_cmp(q));
    Grain {
        peak: up[(up.len() as f64 * 0.995) as usize] as f32,
        fine: (ey / ex.max(1e-12)) as f32,
        coarse: (by / bx.max(1e-12)) as f32,
        rms: (((ex + ey) / (2.0 * n)).sqrt() * 0.5 / px_mm as f64) as f32,
    }
}

/// A canvas prepared in `st`'s manner, `width` px wide, in a 400 × 300
/// unit window of the upper middle (where the sky goes), and the grain of
/// its surface.
fn surface_grain(st: &Style, width: usize, seed: u64) -> Grain {
    let margin = 30.0;
    let crop = Crop { units: [300.0, 100.0, 700.0, 400.0], margin };
    let c = st.prepare_window(width, 1.3, seed, Some(crop));
    let f = c.window();
    let scale = width as f32 / 1000.0;
    // skip the margin (strokes start and end there) plus the stencil
    let pad = (margin * scale) as usize + 2;
    grain(c.surface_um(), f.w, f.h, pad, st.width_mm / width as f32)
}

fn only(st: &Style, g: Vec<Ground>) -> Style {
    Style { ground: g, ..st.clone() }
}

/// The brushed top ground is not a field of parallel horizontal ridges: a
/// primer's hand still favors the across direction a little, but its
/// relief runs every way. At 1200 px the knife-spread layers under it
/// measure 1.05 fine and 1.04 coarse; the brushed ground 1.29 and 1.61
/// (before loop 2: 2.63 and 5.6, the wood-grain). At 3200 px
/// (`print_ground_grain`, seeds 1, 23, 5) it is 1.73–1.76 and 2.04–2.50
/// (before: 3.6–4.1 and 7.4–8.5; linen + knives 1.5 and 2.5).
#[test]
fn friedrich_ground_has_no_horizontal_grain() {
    let st = Style::friedrich();
    let knives = only(&st, st.ground[..2].to_vec());
    // (1200 px keeps it quick in the debug profile; the full-resolution
    // numbers are in `print_ground_grain`)
    let seed = 1u64;
    let under = surface_grain(&knives, 1200, seed);
    let g = surface_grain(&st, 1200, seed);
    eprintln!("seed {seed}: knives {under:?}\n        friedrich {g:?}");
    assert!(g.fine < 2.2, "seed {seed}: horizontal striations dominate: {g:?}");
    assert!(g.coarse < 3.0, "seed {seed}: horizontal ridges (wood-grain): {g:?} vs knives {under:?}");
    // but the brush marks are still there to show through thin paint
    assert!(g.rms > 1.2 * under.rms, "seed {seed}: the brushed ground lost its striations: {g:?} vs knives {under:?}");
}

/// Diagnostic: the grain of each ground stack at full resolution.
/// `cargo test --release -p paint --test ground_grain -- --ignored --nocapture`
#[test]
#[ignore]
fn print_ground_grain() {
    let base = Style::friedrich();
    let cases: Vec<(&str, Style)> = vec![
        ("linen", only(&base, vec![])),
        ("linen+knives", only(&base, base.ground[..2].to_vec())),
        ("friedrich", base.clone()),
        ("friedrich_early (roller)", Style::friedrich_early()),
    ];
    for (name, st) in &cases {
        for seed in [1u64, 23, 5] {
            eprintln!("{name:26} seed {seed:2}: {:?}", surface_grain(st, 3200, seed));
        }
    }
}

/// Diagnostic: lit crops of the bare grounds (GRAIN_OUT=dir).
#[test]
#[ignore]
fn save_ground_crops() {
    let dir = std::env::var("GRAIN_OUT").unwrap_or_else(|_| "grain_out".into());
    let base = Style::friedrich();
    for (name, st) in [("knives", only(&base, base.ground[..2].to_vec())), ("friedrich", base.clone()), ("early", Style::friedrich_early())] {
        let crop = Crop { units: [400.0, 100.0, 560.0, 260.0], margin: 30.0 };
        let mut c = st.prepare_window(3200, 1.3, 23, Some(crop));
        c.relief(0.35, 0.02);
        c.save(format!("{dir}/{name}.png")).unwrap();
    }
}

/// The top ground as it was brushed before loop 2 (long parallel strokes
/// across the canvas), for comparison.
fn old_brushed(width: usize, seed: u64, crop: Crop) -> paint::Canvas {
    use paint::{Handling, Mask, Tool};
    let st = Style::friedrich();
    let g = st.ground[2];
    let mut c = Style { ground: st.ground[..2].to_vec(), ..st.clone() }.prepare_window(width, 1.3, seed, Some(crop));
    let all = Mask::from_fn(c.frame(), |_, _| 1.0);
    let hog = Tool { lay: 1.2, ragged: 0.2, ..Tool::hog_flat(40.0) };
    let h = Handling::new(hog)
        .color(move |_, _| g.color)
        .paint(g.hiding, g.stiff)
        .angle(|_, _| 0.0)
        .angle_jitter(0.04)
        .curve(0.04, 0.3)
        .drift(0.25, 450.0)
        .cross(0.1)
        .tail(0.1)
        .broken(0.1)
        .swell(0.12)
        .length(250.0, 600.0)
        .coverage(3.5)
        .pressure(0.8, 0.95)
        .dips(1, (g.um / 318.0).min(1.0), 0.3)
        .jitter(0.004, 0.002)
        .shake(0.15);
    c.work(&all, &h, seed * 31 + 2);
    c.dry();
    c
}

/// Pixels of bare (saturated) ground under a thin broad sky, blended as
/// the style blends it, on a prepared canvas.
fn sky_bare(mut c: paint::Canvas, margin: f32) -> usize {
    use paint::{Mask, hex};
    let base = Style::friedrich();
    let sky = Mask::from_fn(c.frame(), |_, _| 1.0);
    c.work(&sky, &base.broad().color(|_, _| hex("#d9dcd6")).angle(|_, _| 0.0).coverage(4.5).medium(0.3), 11);
    if let Some(b) = base.blend() {
        c.work(&sky, &b, 12);
    }
    c.dry();
    let f = c.window();
    let pad = (margin * 3.2) as usize;
    let mut n = 0;
    for y in pad..f.h - pad {
        for x in pad..f.w - pad {
            let p = c.pixels()[y * f.w + x];
            if p[0] - p[2] > 0.12 {
                n += 1;
            }
        }
    }
    n
}

/// A thin blended sky (as the easel's `hand="broad"` lays it) leaves
/// about as few pixels of bare ground over the new brushed ground as over
/// the old one: three seeds at 3200 px, old 98 in all, new 164 (before the
/// fix, 883, as one-pixel dotted contours along the ridges the priming
/// brush had ploughed up). What is left are thin spots in the sky's own
/// strokes, not ridge crests (see `tests::diag_sky_bare_pixels`). Ignored:
/// ~1 min in release, far longer in debug; run with
/// `cargo test --release -p paint --test ground_grain -- --ignored sky_bares --nocapture`.
#[test]
#[ignore]
fn sky_bares_ground() {
    let crop = Crop { units: [400.0, 100.0, 560.0, 260.0], margin: 40.0 };
    let st = Style::friedrich();
    let (mut old, mut new) = (0, 0);
    for seed in [23u64, 1, 5] {
        let o = sky_bare(old_brushed(3200, seed, crop), 40.0);
        let n = sky_bare(st.prepare_window(3200, 1.3, seed, Some(crop)), 40.0);
        eprintln!("seed {seed:2}: bare px old ground {o:4}, new ground {n:4}");
        old += o;
        new += n;
    }
    eprintln!("total: old {old}, new {new}");
    assert!(new <= 2 * old + 20, "the thin sky leaves bare ground along the new ground's ridges: {new} px vs {old} on the old ground");
}
