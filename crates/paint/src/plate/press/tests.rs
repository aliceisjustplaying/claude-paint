//! The press against its formulas (notes/research/intaglio_physics.md §6).
//! Every expected number is computed here from the formula and the inputs.
//! Tiny plates and sheets only.

use super::*;
use crate::plate::inked::{Ink, Inker, Pattern};
use crate::rag::Rag;

fn black() -> Ink {
    Ink { name: "black".into(), tubes: vec![("bone black".into(), 1.0)], oil: 0.1, stiff: 1.0, pvc: 0.45, k: [8.0, 8.5, 9.0], s: [0.5, 0.5, 0.5], drying: 0.4 }
}

/// A plate 20 × 20 mm at 0.1 mm a cell.
fn plate() -> Plate {
    Plate::new(Metal::Copper, Temper::HalfHard, 20.0, 20.0, 1.3, 45.0, 200, 200)
}

/// A sheet of drawing paper 40 mm square at the plate's pitch (400 px).
fn sheet() -> Canvas {
    Canvas::new_window(400, 1.0, [0.9; 3], None).with_engine(9).with_size_mm(40.0).with_paper(crate::paper::Paper::drawing(4))
}

fn feel(c: &Canvas, damp: bool) -> SheetFeel {
    let p = c.paper().unwrap();
    SheetFeel { thick_um: p.thick_um as f64, stiffness: if damp { crate::damp::stiffness(0.6) } else { 1.0 }, damp, absorbent: p.absorbent as f64 }
}

fn paint() -> Paint {
    let pal = crate::palette::Palette::named_box("impressionist").or_else(|| crate::palette::Palette::named_box(crate::palette::Palette::fallback_box())).unwrap();
    let i = pal.tubes.iter().position(|t| t.name == "bone black").unwrap_or(0);
    pal.pile(vec![(i, 1.0)]).paint(0.1)
}

/// Openings across cells (x0..x1, y0..y1), `w` µm wide, `d` deep, covering `share`.
fn cut(p: &mut Plate, x0: usize, x1: usize, y0: usize, y1: usize, w: f64, d: f64, share: f64) {
    for y in y0..y1 {
        for x in x0..x1 {
            p.cell_mut(x, y).unwrap().add_open(w, share, share * d);
        }
    }
}

/// The felts: a Winkler layer under the roller. At the load the screws set
/// over the bed, the felts over the plate and beside it carry it all, and the
/// plate (squeezed by its thickness more) takes the higher pressure.
#[test]
fn the_felts_carry_the_load_as_a_winkler_layer() {
    let spec = PressSpec::default();
    let sq = squeeze(&spec, 20.0, 1.3);
    let t = FELTS_MM.iter().sum::<f64>() * 1e-3;
    let k = FELT_PA / t;
    let r = ROLLER_MM * 1e-3;
    let q = |d: f64| if d <= 0.0 { 0.0 } else { 4.0 * 2f64.sqrt() / 3.0 * k * r.sqrt() * d.powf(1.5) };
    let d = sq.squeeze_mm * 1e-3;
    let load = 0.7 * 130e3 * 0.45;
    assert!(((q(d) * 0.02 + q(d - 1.3e-3) * 0.43) / load - 1.0).abs() < 1e-6);
    assert!((sq.on_plate_pa - k * d).abs() < 1e-6 * sq.on_plate_pa);
    assert!(sq.on_plate_pa > sq.beside_pa);
    // IP §6.1: a few MPa under the felts
    assert!((0.5e6..15e6).contains(&sq.on_plate_pa), "{} MPa", sq.on_plate_pa * 1e-6);
    // and the contact's length 2√(2Rd)
    assert!((sq.contact_mm - 2.0 * (2.0 * r * d).sqrt() * 1e3).abs() < 1e-9);
    // more screws, more pressure; a thicker plate takes more of it
    assert!(squeeze(&PressSpec { pressure: 0.9, ..spec.clone() }, 20.0, 1.3).on_plate_pa > sq.on_plate_pa);
    assert!(squeeze(&spec, 20.0, 1.6).on_plate_pa > sq.on_plate_pa);
}

/// IP §6.4 (Aoki & Murakami): transfer rises with the fill at a fixed
/// press-in and with the press-in at a fixed fill, towards the split share,
/// never above it.
#[test]
fn transfer_rises_with_fill_and_press_in_to_the_split_share() {
    let s = sheet();
    let f = feel(&s, true);
    let (w, d, mu) = (150.0, 60.0, 6.0);
    let mut last = 0.0;
    for fill in [0.5, 0.7, 0.85, 0.95, 1.0] {
        let share = contact(&f, w, d, d * (1.0 - fill), 2e6, mu);
        assert!(share >= last && share <= 1.0, "fill {fill}: {share}");
        last = share;
    }
    let mut last = 0.0;
    for p in [0.5e6, 1e6, 2e6, 4e6, 8e6] {
        let share = contact(&f, w, d, d * 0.2, p, mu);
        assert!(share >= last && share <= 1.0, "{p} Pa: {share}");
        last = share;
    }
    // on a plate: the share of the line's ink the sheet takes is at most s
    for pressure in [0.3, 0.6, 0.9] {
        let mut p = plate();
        cut(&mut p, 20, 180, 100, 101, 100.0, 50.0, 1.0);
        p.ink(&black(), None, Inker::Card, 0.0).unwrap();
        let before = p.ink_um3();
        let mut sh = sheet();
        let fe = feel(&sh, true);
        let r = p.press(&mut sh, (10.0, 10.0), &PressSpec { pressure, ..PressSpec::default() }, &fe, &[paint()], 1.0).unwrap();
        let left = p.ink_um3();
        let lines = (before.0 - left.0) / before.0;
        assert!(lines <= r.split + 1e-6 && lines > 0.0, "{pressure}: lines gave {lines}");
        assert!(((before.1 - left.1) / before.1 - r.split).abs() < 1e-4, "the film splits by s");
    }
}

/// The press-in: a strip held by its shear and bending, p·x(w − x)/(2GT) +
/// p·x²(w − x)²/(24D); a damp sheet is softer (`damp::stiffness`) and goes in
/// further.
#[test]
fn the_sheet_presses_in_by_its_shear_and_bending() {
    let s = sheet();
    let (dry, damp) = (feel(&s, false), feel(&s, true));
    let t = dry.thick_um * 1e-6;
    let (w, x, p) = (300.0f64, 150.0f64, 2e6f64);
    let g = 30e6;
    let dd = 4e9 * t.powi(3) / 12.0;
    let want = (p * (x * 1e-6) * ((w - x) * 1e-6) / (2.0 * g * t) + p * (x * 1e-6).powi(2) * ((w - x) * 1e-6).powi(2) / (24.0 * dd)) * 1e6;
    assert!((dry.press_in_um(w, x, p) - want).abs() < 1e-6 * want);
    assert!(damp.press_in_um(w, x, p) > 4.0 * dry.press_in_um(w, x, p));
    assert!(dry.split() < damp.split() || dry.absorbent >= 1.0);
}

/// A damp sheet takes more than a dry one (it presses in further and splits
/// the ink more its way); ink is conserved, plate → sheet; the print is the
/// plate mirrored.
#[test]
fn a_damp_sheet_takes_more_and_the_print_is_mirrored() {
    let make = || {
        let mut p = plate();
        // a line on the plate's left half only
        cut(&mut p, 10, 90, 100, 101, 100.0, 50.0, 1.0);
        p.ink(&black(), None, Inker::Card, 0.0).unwrap();
        p.wipe(&mut Rag::tarlatan(40.0, 2, 0.7), None, 0.5, 5, Pattern::Circles, 1.0);
        p
    };
    let mut took = Vec::new();
    for damp in [false, true] {
        let mut p = make();
        let mut sh = sheet();
        let fe = feel(&sh, damp);
        let before = p.ink_um3();
        let wet0 = sh.wet_coats();
        let r = p.press(&mut sh, (10.0, 10.0), &PressSpec::default(), &fe, &[paint()], 2.0).unwrap();
        let after = p.ink_um3();
        let px = sh.px_mm() as f64 * 1000.0;
        let gained = (sh.wet_coats() - wet0) * COAT_UM as f64 * px * px;
        let lost = before.0 + before.1 - after.0 - after.1;
        assert!((lost - gained).abs() <= 1e-4 * lost, "lost {lost} µm³, the sheet gained {gained}");
        assert!((r.taken_um3 - lost).abs() <= 1e-6 * lost);
        // mirrored: the plate's left (x 10..90) lands at the print's right of the plate's place
        let row = 100 + 100;
        let at = |x: usize| sh.wet.vol[row * 400 + x];
        let (plate_x0, plate_w) = (100usize, 200usize);
        let right: f32 = (plate_x0 + plate_w - 90..plate_x0 + plate_w - 10).map(at).sum();
        let left: f32 = (plate_x0 + 10..plate_x0 + 90).map(at).sum();
        assert!(right > 10.0 * left.max(1e-9), "right {right}, left {left}");
        took.push(lost);
    }
    assert!(took[1] > took[0], "damp {} against dry {}", took[1], took[0]);
}

/// A damp sheet keeps the plate's shape: the footprint lies below the margin
/// by the plate mark (`MARK_SET` of the plate's thickness), the lines stand
/// up by their press-in. A dry sheet springs back.
#[test]
fn a_damp_sheet_keeps_the_plate_mark_and_the_lines() {
    for damp in [true, false] {
        let mut p = plate();
        cut(&mut p, 10, 190, 100, 101, 300.0, 60.0, 1.0);
        let mut sh = sheet();
        let h0 = sh.height.clone();
        let fe = feel(&sh, damp);
        let r = p.press(&mut sh, (10.0, 10.0), &PressSpec::default(), &fe, &[], 1.0).unwrap();
        let dz = |x: usize, y: usize| sh.height[y * 400 + x] - h0[y * 400 + x];
        if damp {
            let want = MARK_SET * 1300.0;
            assert!((r.mark_um - want).abs() < 1e-3);
            assert!((dz(200, 150) as f64 + want).abs() < 1e-2, "the footprint {}", dz(200, 150));
            assert_eq!(dz(50, 50), 0.0, "the margin");
            // the line stands up from the footprint by its press-in, at most its depth
            let line = dz(200, 200) - dz(200, 150);
            assert!(line > 0.0 && line <= 60.0 + 1e-3, "{line}");
            // the bevel: the mark ramps over thickness / tan(bevel)
            assert!(dz(100, 150) > dz(110, 150) + 1.0);
        } else {
            assert!(r.mark_um == 0.0 && dz(200, 150) == 0.0 && dz(200, 200) == 0.0);
        }
    }
}
