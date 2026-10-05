//! Thinner check 13 (notes/thinner/ACCEPTANCE.md): the pigments stay as
//! they were, and burnt sienna is the more transparent of the two siennas
//! (Field/Salter 1869, §§50 and 155), measured as the lower contrast ratio
//! of equal films on the same named substrates. Uses only the API af49348
//! already had, so it runs on unchanged code too.
//!
//!   cargo test -p paint --release --test thinner_pigments -- --show-output
//!
//! `tests/thinner/tubes_af49348.txt` is `tube_table()` as unchanged af49348
//! prints it. `tubes_current.txt` separately freezes the expanded catalog and
//! current box engine labels.

use paint::color::luminance;
use paint::pigment::{hiding_of, scatter_for};
use paint::{COAT_UM, Canvas, Gesture, Held, Linen, Paint, Palette, Rgb, Tool, hex};

/// Every tube the engine knows (`Debug`: name, pigment, masstone, hiding,
/// stiffness, strength, drying rates, floats in their shortest exact
/// form), then each box this build has, its tubes in order and its engine.
fn tube_table() -> String {
    let mut s = String::from("catalog\n");
    for t in paint::palette::catalog() {
        s += &format!("{t:?}\n");
    }
    for name in Palette::box_names() {
        let b = Palette::named_box(name).unwrap_or_else(|| panic!("box {name}"));
        s += &format!("box {name} (engine {})\n", b.engine);
        for t in &b.tubes {
            s += &format!("  {t:?}\n");
        }
    }
    s
}

/// Prints `tube_table()` (to make the af49348 file; not a check).
#[test]
#[ignore = "prints the tube table; run on af49348 to make tests/thinner/tubes_af49348.txt"]
fn print_tube_table() {
    let out = std::env::var("TUBE_TABLE_OUT").expect("TUBE_TABLE_OUT: the file to write");
    std::fs::write(&out, tube_table()).unwrap_or_else(|e| panic!("{out}: {e}"));
}

/// Check 13 (a): historical pigment fields and legacy box tube ordering remain
/// exact. New catalog tubes, new boxes and current engine labels are allowed,
/// but the expanded current table has its own exact reviewed snapshot.
#[test]
fn c13_every_pigment_value_is_af49348s() {
    fn sections(table: &str) -> std::collections::BTreeMap<&str, Vec<&str>> {
        let mut result = std::collections::BTreeMap::new();
        let mut section = "catalog";
        for line in table.lines() {
            if line == "catalog" {
                result.insert(section, Vec::new());
            } else if let Some(header) = line.strip_prefix("box ") {
                section = header.split_once(" (engine ").expect("box engine label").0;
                assert!(result.insert(section, Vec::new()).is_none(), "duplicate box {section}");
            } else {
                result.get_mut(section).expect("table section").push(line.trim_start());
            }
        }
        result
    }
    let got = tube_table();
    let old = sections(include_str!("thinner/tubes_af49348.txt"));
    let now = sections(&got);
    for (section, tubes) in old {
        let current = now.get(section).unwrap_or_else(|| panic!("missing historical section {section}"));
        if section == "catalog" {
            for tube in tubes {
                let name = tube.split_once(", pigment:").expect("tube name").0;
                let actual = current.iter().find(|line| line.split_once(", pigment:").expect("tube name").0 == name);
                assert_eq!(actual.copied(), Some(tube), "historical catalog fields changed: {name}");
            }
        } else {
            assert_eq!(current, &tubes, "historical box tube values/order changed: {section}");
        }
    }
    assert_eq!(got, include_str!("thinner/tubes_current.txt"), "current expanded tube table changed");
}

/// A tube of the Sargent box (it has both siennas), straight from the tube.
fn sargent(name: &str) -> Paint {
    let pal = Palette::named_box("sargent").expect("the sargent box");
    let i = pal.tubes.iter().position(|t| t.name == name).unwrap_or_else(|| panic!("no {name:?} in the sargent box"));
    pal.pile(vec![(i, 1.0)]).laid(0.0)
}

/// Check 13 (b)'s film thicknesses, µm of nonvolatile paint (the tube paint
/// carries no solvent, so the whole film is nonvolatile), fixed before the
/// check was restated: the thicknesses of the existing sienna diagnostic
/// (`look_sienna.rs`, notes/look/logs/sienna.txt). 25 µm is one coat
/// (`COAT_UM`).
const FILMS_UM: [f32; 4] = [3.0, 10.0, 25.0, 30.0];

/// How much lower burnt sienna's luminance contrast ratio must be than raw
/// sienna's, at every film and substrate: an equality or an order inside
/// numerical noise fails. Chosen from the arithmetic, not from the measured
/// gap. The ratio (0..1) comes from a few dozen f32 operations, exp and
/// sinh/cosh among them (`Pigment::over`); f32 resolves 6e-8 at 1, so even
/// a pessimistic accumulation of rounding, and libm's last-place
/// differences between platforms, stays near 1e-5. 1e-4 is ten times that,
/// and forty times smaller than one 8-bit display step (1/255).
const MARGIN: f32 = 1e-4;

/// Check 13 (b)'s named substrates, (name, black, white) in linear RGB:
/// the hiding chart's black (0) and white of 80% reflectance (the contrast
/// ratio's usual chart, ASTM D2805 / ISO 6504-3), and the Sargent box's
/// own bone black and lead white laid thick (their masstones, which 13 (a)
/// freezes). Neither depends on how a brush lays paint.
fn substrates() -> [(&'static str, Rgb, Rgb); 2] {
    let pal = Palette::named_box("sargent").expect("the sargent box");
    let masstone = |name: &str| pal.tubes.iter().find(|t| t.name == name).unwrap_or_else(|| panic!("no {name:?} in the sargent box")).color;
    [("black/80% white chart", [0.0; 3], [0.8; 3]), ("bone black/lead white masstones", masstone("bone black"), masstone("lead white"))]
}

/// A film `um` µm thick of `paint` over `under`: the engine's own uniform-film
/// optics (Kubelka-Munk, `Paint::over`), no brush involved.
fn film(paint: &Paint, under: Rgb, um: f32) -> Rgb {
    paint.over(under, um / COAT_UM)
}

/// Check 13 (b): at each of `FILMS_UM`, on each of `substrates()`, an equal
/// film of burnt sienna has a lower RGB luminance contrast ratio (luminance
/// over the black ÷ over the white; lower = more of the substrate shows
/// through) than raw sienna, by at least `MARGIN`. Field/Salter 1869 §155
/// calls burnt sienna "more transparent than the raw earth".
///
/// Prints, as diagnostics that aren't checked: the substrates' RGB and
/// luminance; each film's RGB over each; each channel's contrast ratio
/// (the channels need not rank alike); the retained absolute substrate
/// difference (the card's old measure); and, for each tube, its catalog
/// hiding beside the one-coat contrast ratio the rendered RGB film has.
#[test]
fn c13_burnt_sienna_has_a_lower_contrast_ratio_than_raw_sienna_at_equal_film() {
    let siennas = [("raw sienna", sargent("raw sienna")), ("burnt sienna", sargent("burnt sienna"))];
    for (name, p) in &siennas {
        let t = paint::palette::catalog().into_iter().find(|t| t.name == *name).unwrap();
        let y = luminance(t.color);
        let ratio = |w: f32| luminance(film(p, [0.0; 3], COAT_UM)) / luminance(film(p, [w; 3], COAT_UM));
        println!(
            "{name}: catalog hiding {} = an input calibration scalar: the one-coat contrast ratio of a gray paint of the masstone's luminance {y:.4} over black/white 1.0 (`hiding_of`, here {:.4}), which fixes the scattering {:.4} per coat (`scatter_for`). The rendered RGB film (masstone {:.4?}, scattering {:.4}): one coat's luminance contrast ratio over black/white 1.0 {:.4}, over the black/80% white chart {:.4}",
            t.hiding,
            hiding_of(y, scatter_for(y, t.hiding)),
            scatter_for(y, t.hiding),
            p.color,
            p.scatter,
            ratio(1.0),
            ratio(0.8)
        );
    }
    let mut fails = Vec::new();
    for (sub, black, white) in substrates() {
        let (yb, yw) = (luminance(black), luminance(white));
        println!("substrate {sub}: black RGB {black:.4?} Y {yb:.4}; white RGB {white:.4?} Y {yw:.4}");
        for um in FILMS_UM {
            let mut cr = [[0.0f32; 4]; 2];
            for (k, (name, p)) in siennas.iter().enumerate() {
                let (b, w) = (film(p, black, um), film(p, white, um));
                cr[k] = [luminance(b) / luminance(w), b[0] / w[0], b[1] / w[1], b[2] / w[2]];
                let kept = (luminance(w) - luminance(b)) / (yw - yb);
                println!(
                    "  {um:>4} µm {name:<12}: over black RGB {b:.4?}, over white RGB {w:.4?}; contrast ratio Y {:.4} (R {:.4} G {:.4} B {:.4}); retained absolute substrate difference {:.1}% (diagnostic)",
                    cr[k][0],
                    cr[k][1],
                    cr[k][2],
                    cr[k][3],
                    100.0 * kept
                );
            }
            let not_lower: Vec<&str> = ["R", "G", "B"].into_iter().enumerate().filter(|&(c, _)| cr[1][c + 1] >= cr[0][c + 1]).map(|(_, n)| n).collect();
            println!("  {um:>4} µm: burnt - raw contrast ratio Y {:+.4}; channels where burnt's is not lower (diagnostic): {not_lower:?}", cr[1][0] - cr[0][0]);
            if !(cr[1][0] <= cr[0][0] - MARGIN) {
                fails.push(format!("{sub}, {um} µm: burnt {:.5}, raw {:.5}", cr[1][0], cr[0][0]));
            }
        }
    }
    assert!(fails.is_empty(), "burnt sienna's luminance contrast ratio is not lower than raw sienna's by {MARGIN} ({}): Field/Salter (§155) has burnt the more transparent", fails.join("; "));
}

/// The card: a bone-black band and a lead-white band laid thick and dried,
/// then the same twenty filbert strokes of `paint` (unthinned, load 0.5)
/// across both. Returns the bands' mean RGB before the strokes (black,
/// white), after (black, white), the open paint (µm) at every pixel and its
/// mean over the two bands.
fn card(paint: Paint) -> ([Rgb; 4], Vec<f32>, f32) {
    let mut c = Canvas::new(400, 1.0, hex("#d8cdb8")).with_engine(3).with_linen(Linen::fine(3));
    c.prime(hex("#b9a98c"), 0.9, 60.0, 0.6, 0.2, 7);
    for (name, y0) in [("bone black", 300.0f32), ("lead white", 500.0)] {
        for k in 0..10 {
            let mut h = Held::new(Tool::hog_flat(40.0), 1 + k);
            h.load(sargent(name), 1.0);
            let y = y0 + 10.0 + 20.0 * k as f32;
            c.drag(&mut h, &Gesture::new(vec![(100.0, y), (900.0, y)]).pressure(0.9, 0.9), None);
        }
    }
    c.dry();
    let f = c.frame();
    let mean = |c: &Canvas, r: (f32, f32, f32, f32)| -> Rgb {
        let pts: Vec<(f32, f32)> = (0..f.w * f.h).map(|i| (f.ux(i % f.w), f.uy(i / f.w))).filter(|&(x, y)| x >= r.0 && x < r.2 && y >= r.1 && y < r.3).collect();
        let mut a = [0.0f64; 3];
        for &(x, y) in &pts {
            let p = c.under(x, y, 0.0);
            for k in 0..3 {
                a[k] += p[k] as f64;
            }
        }
        a.map(|v| (v / pts.len() as f64) as f32)
    };
    let (black, white) = ((200.0, 330.0, 800.0, 470.0), (200.0, 530.0, 800.0, 670.0));
    let before = [mean(&c, black), mean(&c, white)];
    for k in 0..20 {
        let mut h = Held::new(Tool::filbert(30.0), 100 + k);
        h.load(paint, 0.5);
        let y = 310.0 + 20.0 * k as f32;
        c.drag(&mut h, &Gesture::new(vec![(150.0, y), (850.0, y)]).pressure(0.8, 0.8), None);
    }
    let film = (0..f.w * f.h).map(|i| c.wet_um(f.ux(i % f.w), f.uy(i / f.w))).collect();
    let on_bands: Vec<f32> = (0..f.w * f.h).map(|i| (f.ux(i % f.w), f.uy(i / f.w))).filter(|&(x, y)| [black, white].iter().any(|r| x >= r.0 && x < r.2 && y >= r.1 && y < r.3)).map(|(x, y)| c.wet_um(x, y)).collect();
    ([before[0], before[1], mean(&c, black), mean(&c, white)], film, on_bands.iter().sum::<f32>() / on_bands.len() as f32)
}

/// A diagnostic, not an ordering check (until the restatement this was
/// check 13 (b), which required burnt ≥ raw here): on the brush-painted
/// card, the share of the bands' absolute black/white luminance difference
/// that an equal film of each sienna still shows (the retained absolute
/// substrate difference, an underpainting-value measure), beside the same
/// bands' contrast ratio, per channel too. It depends on the brush's
/// deposition, so its numbers belong to the code that ran it. It checks only
/// its own fixture: the strokes laid paint and the two films are equal.
#[test]
fn c13_diagnostic_sienna_card_retained_absolute_substrate_difference() {
    let (raw, film_raw, um_raw) = card(sargent("raw sienna"));
    let (burnt, film_burnt, um_burnt) = card(sargent("burnt sienna"));
    let worst = film_raw.iter().zip(&film_burnt).map(|(a, b)| (a - b).abs()).fold(0.0, f32::max);
    let total: f32 = film_raw.iter().sum();
    println!("card bands before the strokes: black RGB {:.4?} Y {:.4}, white RGB {:.4?} Y {:.4}", raw[0], luminance(raw[0]), raw[1], luminance(raw[1]));
    for (name, m, um) in [("raw sienna", raw, um_raw), ("burnt sienna", burnt, um_burnt)] {
        let kept = (luminance(m[3]) - luminance(m[2])) / (luminance(m[1]) - luminance(m[0]));
        println!(
            "{name}: mean film on the bands {um:.2} µm; over black RGB {:.4?}, over white RGB {:.4?}; retained absolute substrate difference {:.2}%; contrast ratio Y {:.4} (R {:.4} G {:.4} B {:.4})",
            m[2],
            m[3],
            100.0 * kept,
            luminance(m[2]) / luminance(m[3]),
            m[2][0] / m[3][0],
            m[2][1] / m[3][1],
            m[2][2] / m[3][2]
        );
    }
    println!("(films differ by at most {worst} µm at a pixel; raw's mean over the canvas {} µm)", total / film_raw.len() as f32);
    assert!(total > 0.0, "the strokes laid paint");
    assert!(worst <= 1e-3, "the two films are equal (solvent-free): they differ by up to {worst} µm at a pixel");
    assert_eq!([raw[0], raw[1]], [burnt[0], burnt[1]], "both siennas go on the same card");
}
