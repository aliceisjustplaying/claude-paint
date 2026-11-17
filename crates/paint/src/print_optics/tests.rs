//! A print's optics against their formulas and against measured papers
//! (notes/research/intaglio_physics.md §6.7). Tiny sheets only.

use super::*;

/// The frequency (cycles/mm) at which the paper's MTF falls to a half.
fn half_cy_mm(s: f64, kos: f64) -> f64 {
    let (mut lo, mut hi) = (0.0, 50.0);
    for _ in 0..60 {
        let m = 0.5 * (lo + hi);
        if mtf(std::f64::consts::TAU * m * 1e-3, s, kos) > 0.5 { lo = m } else { hi = m }
    }
    lo
}

/// The MTF is Kubelka–Munk's reflectance at the lateral wavenumber's
/// absorption over its reflectance at none: one at q = 0, falling, and for
/// a small q the diffusion limit K(q) ≈ K + q²/(2(K + S)).
#[test]
fn the_paper_mtf_is_kubelka_munk_at_the_lateral_absorption() {
    let (s, kos) = (0.016, 0.025);
    assert!((mtf(0.0, s, kos) - 1.0).abs() < 1e-12);
    let mut last = 1.0;
    for i in 1..40 {
        let m = mtf(i as f64 * 1e-3, s, kos);
        assert!(m < last && m > 0.0);
        last = m;
    }
    let q = 1e-4;
    let k = kos * s + q * q / (2.0 * (kos * s + s));
    let a = 1.0 + k / s;
    let expect = (a - (a * a - 1.0).sqrt()) / r_inf(1.0 + kos);
    assert!((mtf(q, s, kos) - expect).abs() < 1e-6, "{} {}", mtf(q, s, kos), expect);
}

/// Against measured papers: an uncoated sheet's MTF falls to a half near
/// 2.2 cycles/mm (Arney et al. 1996's noncoated paper, k_p = 0.455 mm) and
/// near 5 (Rogers 1998's survey of uncoated papers). The engine's rag sheets,
/// from s = 30 m²/kg at their own density and their tone, fall between
/// those, by Kubelka–Munk with nothing fitted.
#[test]
fn a_rag_sheet_spreads_light_as_measured_uncoated_papers() {
    for p in [crate::paper::Paper::drawing(4), crate::paper::PRESETS.iter().find(|p| p.name == "cream laid").unwrap().paper(1)] {
        let s = sheet_s_per_um(&p);
        let f = half_cy_mm(s, kos_of(0.85));
        assert!((1.5..=6.0).contains(&f), "{} g/m², caliper {} µm, S {s} /µm: half at {f} cy/mm", p.grammage, p.caliper_um());
    }
}

/// A uniform film over the paper gets nothing: the lateral light is only
/// the difference at its edges. At an edge the paper beside the ink is
/// darkened (its light goes under the ink and is absorbed) and the ink by
/// the edge is lightened (light from the paper comes up under it); far from
/// it nothing changes.
#[test]
fn the_lateral_light_darkens_the_paper_by_ink_and_lightens_the_ink_by_paper() {
    // (25 µm pixels: the ink's half 6.4 mm, past the spread's 3 mm reach)
    let (w, h) = (512, 4);
    let paper = [0.8f32; 3];
    let film = ([0.0f32; 3], [0.2f32; 3]);
    let body_of = |f: &(Rgb, Rgb)| -> Rgb {
        f.1.map(|t| crate::mixer::outside(t * t * crate::mixer::inside(0.8)))
    };
    // all film: unchanged
    let films = vec![film; w * h];
    let mut body = vec![body_of(&film); w * h];
    let before = body.clone();
    add_lateral(&mut body, &films, &vec![paper; w * h], w, h, 0.016, 25.0);
    for (a, b) in body.iter().zip(&before) {
        assert!((a[0] - b[0]).abs() < 1e-5, "{a:?} {b:?}");
    }
    // the left half inked
    let bare = ([0.0f32; 3], [1.0f32; 3]);
    let films: Vec<(Rgb, Rgb)> = (0..w * h).map(|i| if i % w < w / 2 { film } else { bare }).collect();
    let mut body: Vec<Rgb> = films.iter().map(|f| if f.1[0] < 1.0 { body_of(f) } else { paper }).collect();
    let before = body.clone();
    add_lateral(&mut body, &films, &vec![paper; w * h], w, h, 0.016, 25.0);
    let row = 2 * w;
    assert!(body[row + w / 2][0] < before[row + w / 2][0] - 0.01, "the paper by the ink darkens: {:?}", &body[row + w / 2 - 4..row + w / 2 + 4]);
    assert!(body[row + w / 2 - 1][0] > before[row + w / 2 - 1][0] + 0.001, "the ink by the paper lightens");
    assert!((body[row + 8][0] - before[row + 8][0]).abs() < 0.01 * before[row + 8][0], "far into the ink: as before");
    assert!((body[row + w - 8][0] - before[row + w - 8][0]).abs() < 0.01 * before[row + w - 8][0], "far out on the paper: as before");
}

/// The press marks the sheet a print, and a save keeps it.
#[test]
fn a_print_keeps_its_optics_through_a_save() {
    let mut c = crate::canvas::Canvas::new_window(32, 1.0, [0.9; 3], None).with_engine(9).with_size_mm(4.0).with_paper(crate::paper::Paper::drawing(4));
    assert!(!c.print_optics());
    c.print_optics = true;
    let mut buf = Vec::new();
    c.write_state(&mut buf, "print").unwrap();
    let (back, _) = crate::canvas::Canvas::read_state(&mut &buf[..]).unwrap();
    assert!(back.print_optics());
    let mut buf = Vec::new();
    let aside = c.pack(&mut buf).unwrap();
    assert!(crate::canvas::Canvas::unpack(&mut &buf[..], aside).unwrap().print_optics());
}

fn bone_black() -> crate::wet::Paint {
    let pal = crate::palette::Palette::named_box("impressionist").unwrap();
    let i = pal.tubes.iter().position(|t| t.name == "bone black").unwrap();
    pal.pile(vec![(i, 1.0)]).paint(0.1)
}

fn y(p: Rgb) -> f64 {
    0.2126 * p[0] as f64 + 0.7152 * p[1] as f64 + 0.0722 * p[2] as f64
}

/// A sheet 4 mm square (25 µm pixels) with a band of `um` µm of bone black
/// laid across its middle, `rows` pixels wide, printed or not.
fn band(um: f32, rows: usize, printed: bool) -> crate::canvas::Canvas {
    let mut c = crate::canvas::Canvas::new_window(160, 1.0, [0.85; 3], None).with_engine(9).with_size_mm(4.0).with_paper(crate::paper::Paper::drawing(4));
    c.print_optics = printed;
    let paint = bone_black();
    for y in 80 - rows / 2..80 - rows / 2 + rows {
        for x in 0..160 {
            c.lay_ink(y * 160 + x, um / crate::surface::COAT_UM, &paint, 0.0);
        }
    }
    c.wet.touch(0, 0, 160, 160);
    c
}

/// A print's ink is matte: its surface is the film split from the plate's,
/// held by the ink's yield stress. So however thick, a black reads at most
/// the paper over the first-surface veil a matte surface scatters back
/// (`canvas::SURFACE_REFLECTANCE`, the paper's own): log₁₀((k + (1 − k)·P)/k),
/// about 1.3 on a white sheet, where dry uncoated prints' solid blacks read
/// 1.20–1.25 and Rembrandt's heaviest drypoint strokes 1.15–1.27. Unprinted,
/// the same film shows glossy, as wet oil paint.
#[test]
fn a_printed_black_reads_no_darker_than_a_matte_surface() {
    let k = crate::canvas::SURFACE_REFLECTANCE as f64;
    for printed in [false, true] {
        let c = band(30.0, 80, printed);
        let s = c.seen();
        let (paper, ink) = (y(s[5 * 160 + 80]), y(s[80 * 160 + 80]));
        let od = (paper / ink).log10();
        let bare = (paper - k) / (1.0 - k);
        let cap = ((k + (1.0 - k) * bare) / k).log10();
        if printed {
            assert!(od <= cap + 1e-3 && od > cap - 0.05, "OD {od}, the matte cap {cap}");
        } else {
            assert!(od > cap + 0.5, "glossy, wet: OD {od}");
        }
    }
}

/// What the sheet shows while its ink is wet it keeps when the ink dries
/// into it: the lateral light is written into the dry picture as it bakes.
#[test]
fn a_print_looks_the_same_wet_and_dry() {
    let mut c = band(8.0, 6, true);
    let wet = c.seen();
    c.dry();
    let dry = c.seen();
    for x in [10usize, 80] {
        for row in 60..100 {
            let (a, b) = (y(wet[row * 160 + x]), y(dry[row * 160 + x]));
            assert!((a / b).log10().abs() < 0.01, "row {row}: {a} wet, {b} dry");
        }
    }
    // (and the lateral light is there: the paper by the band darker than far off)
    let far = y(dry[10 * 160 + 80]);
    assert!(y(dry[84 * 160 + 80]) < far * 0.97, "{} against {far}", y(dry[84 * 160 + 80]));
}

