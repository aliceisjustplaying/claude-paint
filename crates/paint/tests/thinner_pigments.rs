//! Thinner check 13 (notes/thinner/ACCEPTANCE.md): the pigments stay as
//! they were, and burnt sienna is the more transparent of the two siennas
//! (Field/Salter 1869, §§50 and 155). Uses only the API af49348 already
//! had, so it runs on unchanged code too.
//!
//!   cargo test -p paint --release --test thinner_pigments
//!
//! `tests/thinner/tubes_af49348.txt` is `tube_table()` as unchanged af49348
//! prints it (`print_tube_table`; commands in ACCEPTANCE.md).

use paint::color::luminance;
use paint::pigment::scatter_for;
use paint::{Canvas, Gesture, Held, Linen, Paint, Palette, Tool, hex};

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

/// Check 13 (a): every pigment value, in the catalog and in every box,
/// equals af49348's exactly.
#[test]
fn c13_every_pigment_value_is_af49348s() {
    let want = include_str!("thinner/tubes_af49348.txt");
    let got = tube_table();
    if got != want {
        let at = got.lines().zip(want.lines()).position(|(a, b)| a != b);
        panic!("the tube table differs from af49348's (first differing line {at:?}: now {:?}, then {:?})", at.and_then(|i| got.lines().nth(i)), at.and_then(|i| want.lines().nth(i)));
    }
}

/// A tube of the Sargent box (it has both siennas), straight from the tube.
fn sargent(name: &str) -> Paint {
    let pal = Palette::named_box("sargent").expect("the sargent box");
    let i = pal.tubes.iter().position(|t| t.name == name).unwrap_or_else(|| panic!("no {name:?} in the sargent box"));
    pal.pile(vec![(i, 1.0)]).laid(0.0)
}

/// The card: a bone-black band and a lead-white band laid thick and dried,
/// then the same twenty filbert strokes of `paint` (unthinned, load 0.5)
/// across both. Returns the share of the card's black/white contrast still
/// showing and the open paint (µm) at every pixel.
fn card_kept(paint: Paint) -> (f32, Vec<f32>) {
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
    let mean_l = |c: &Canvas, r: (f32, f32, f32, f32)| -> f32 {
        let pts: Vec<(f32, f32)> = (0..f.w * f.h).map(|i| (f.ux(i % f.w), f.uy(i / f.w))).filter(|&(x, y)| x >= r.0 && x < r.2 && y >= r.1 && y < r.3).collect();
        let mut a = [0.0f64; 3];
        for &(x, y) in &pts {
            let p = c.under(x, y, 0.0);
            for k in 0..3 {
                a[k] += p[k] as f64;
            }
        }
        luminance(a.map(|v| (v / pts.len() as f64) as f32))
    };
    let (black, white) = ((200.0, 330.0, 800.0, 470.0), (200.0, 530.0, 800.0, 670.0));
    let before = mean_l(&c, white) - mean_l(&c, black);
    for k in 0..20 {
        let mut h = Held::new(Tool::filbert(30.0), 100 + k);
        h.load(paint, 0.5);
        let y = 310.0 + 20.0 * k as f32;
        c.drag(&mut h, &Gesture::new(vec![(150.0, y), (850.0, y)]).pressure(0.8, 0.8), None);
    }
    let after = mean_l(&c, white) - mean_l(&c, black);
    let film = (0..f.w * f.h).map(|i| c.wet_um(f.ux(i % f.w), f.uy(i / f.w))).collect();
    (after / before, film)
}

/// Check 13 (b): measured the card's way, an equal (solvent-free) film of
/// burnt sienna lets at least as much of the card's black/white contrast
/// show as raw sienna. Field/Salter 1869 §155 calls burnt sienna "more
/// transparent than the raw earth". Prints both tubes' hiding and
/// scattering per coat alongside.
#[test]
fn c13_burnt_sienna_shows_the_card_at_least_as_well_as_raw_sienna() {
    for name in ["raw sienna", "burnt sienna"] {
        let t = paint::palette::catalog().into_iter().find(|t| t.name == name).unwrap();
        println!("{name}: hiding {} (contrast ratio of one coat), scattering {} per coat, masstone luminance {}", t.hiding, scatter_for(luminance(t.color), t.hiding), luminance(t.color));
    }
    let (raw, film_raw) = card_kept(sargent("raw sienna"));
    let (burnt, film_burnt) = card_kept(sargent("burnt sienna"));
    let worst = film_raw.iter().zip(&film_burnt).map(|(a, b)| (a - b).abs()).fold(0.0, f32::max);
    let total: f32 = film_raw.iter().sum();
    println!("card contrast showing: raw sienna {:.2}%, burnt sienna {:.2}% (films differ by at most {worst} µm; raw's mean {} µm)", 100.0 * raw, 100.0 * burnt, total / film_raw.len() as f32);
    assert!(total > 0.0, "the strokes laid paint");
    assert!(worst <= 1e-3, "the two films are equal (solvent-free): they differ by up to {worst} µm at a pixel");
    assert!(burnt >= raw, "burnt sienna shows {:.2}% of the card, raw sienna {:.2}%: Field/Salter (§155) has burnt the more transparent", 100.0 * burnt, 100.0 * raw);
}
