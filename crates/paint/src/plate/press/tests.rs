//! The press against its formulas (notes/research/intaglio_physics.md §6).
//! Every expected number is computed here from the formula and the inputs.
//! Tiny plates and sheets only.

use super::*;
use crate::plate::inked::{Ink, InkTool, Inker};
use crate::rag::Rag;

fn black() -> Ink {
    Ink { name: "black".into(), tubes: vec![("bone black".into(), 1.0)], oil: 0.1, stiff: 1.0, pvc: 0.45, k: [8.0, 8.5, 9.0], s: [0.5, 0.5, 0.5], drying: 0.4 }
}

fn pt(x: f32, y: f32, force: f32) -> crate::plate::Point {
    crate::plate::Point { x, y, force, alt: 90.0, az: 0.0, speed: 20.0 }
}

/// The card (50 mm, 10 N) dragged across a 20 mm plate: its edge spans it.
fn card() -> Vec<crate::plate::Point> {
    vec![pt(0.0, 10.0, 10.0), pt(20.0, 10.0, 10.0)]
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
    SheetFeel { thick_um: p.thick_um as f64, stiffness: if damp { crate::damp::stiffness(0.6) } else { 1.0 }, damp, absorbent: p.absorbent as f64, strain: (0.0, 0.0) }
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
        p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
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
        p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
        p.wipe(&mut Rag::tarlatan(40.0, 2, 0.7), &[pt(-25.0, 10.0, 0.5), pt(45.0, 10.0, 0.5)], 5, 1.0);
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

/// A plate whose cells are not quite the sheet's pixels (here 20 mm in 199
/// cells over 0.1 mm pixels) prints an even film evenly: each cell's ink is
/// laid over the pixels its footprint covers, by area, so no row or column of
/// pixels is left without ink where the two pitches drift apart, and the ink
/// laid is the ink the plate gave up.
#[test]
fn a_cell_lays_its_ink_over_the_pixels_it_covers() {
    let mut p = Plate::new(Metal::Copper, Temper::HalfHard, 20.0, 20.0, 1.3, 45.0, 199, 199);
    p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
    let mut sh = sheet();
    let fe = feel(&sh, true);
    let v0 = sh.wet_coats();
    let r = p.press(&mut sh, (10.0, 10.0), &PressSpec::default(), &fe, &[paint()], 1.0).unwrap();
    // every row across the middle of the print holds within a few percent of the mean
    let row = |y: usize| (120..280).map(|x| sh.wet.vol[y * 400 + x] as f64).sum::<f64>() / 160.0;
    let rows: Vec<f64> = (110..290).map(row).collect();
    let mean = rows.iter().sum::<f64>() / rows.len() as f64;
    assert!(mean > 0.0);
    for (j, v) in rows.iter().enumerate() {
        assert!((v / mean - 1.0).abs() < 0.05, "row {}: {v} against {mean}", 110 + j);
    }
    // conserved: the coats laid are the ink taken, at the coat's thickness
    let laid = (sh.wet_coats() - v0) * COAT_UM as f64 * 100.0 * 100.0;
    assert!((laid / r.taken_um3 - 1.0).abs() < 1e-3, "laid {laid} µm³, taken {}", r.taken_um3);
}

/// The sheet's size at the pull (`SheetFeel::strain`, under the contact
/// laws): a damp sheet is larger than as made by its fibres' swelling
/// (`damp::strain`), so a plate printed on it covers fewer of its fibres,
/// and the print, dried back to the sheet as made, is smaller than the plate
/// by 1/(1 + ε). Two plates pulled at one strain meet in register; a sheet
/// that dried between them is out by Δε·L from where it is held (`at`).
#[test]
fn a_damp_sheet_prints_the_plate_smaller_and_drying_moves_the_register() {
    // the ink's middle across the sheet (pixels), in two halves
    let centres = |strain: (f64, f64), contact: bool| {
        let mut p = plate();
        p.contact = contact;
        cut(&mut p, 20, 21, 0, 200, 100.0, 50.0, 1.0);
        cut(&mut p, 180, 181, 0, 200, 100.0, 50.0, 1.0);
        p.ink(&black(), InkTool::of(Inker::Card), &card(), 0.0).unwrap();
        // the palm drawn hard straight across the plate's middle, three times
        p.hand_wipe(&[pt(-45.0, 10.0, 1.0), pt(65.0, 10.0, 1.0)], 3, true, 0.5);
        let mut sh = sheet();
        let fe = SheetFeel { strain, ..feel(&sh, true) };
        p.press(&mut sh, (10.0, 10.0), &PressSpec::default(), &fe, &[paint()], 1.0).unwrap();
        let f = sh.window();
        let mut out = [(0.0f64, 0.0f64); 2];
        for y in 0..f.h {
            for x in 0..f.w {
                let v = sh.wet.vol[y * f.w + x] as f64;
                // (the lines' ink, not the polish's film)
                if v > 0.01 {
                    let k = usize::from(x >= 200);
                    out[k].0 += v * (x as f64 + 0.5);
                    out[k].1 += v;
                }
            }
        }
        out.map(|(s, w)| s / w)
    };
    let e = 0.02;
    let flat = centres((0.0, 0.0), true);
    let wet = centres((e, e), true);
    // the plate's lines 16 mm apart (160 cells) print 160/(1 + ε) pixels apart, mirrored
    assert!(((flat[0] - flat[1]).abs() - 160.0).abs() < 0.05, "{flat:?}");
    assert!(((wet[0] - wet[1]).abs() - 160.0 / (1.0 + e)).abs() < 0.05, "{wet:?}");
    // each lands where its plate mm, measured on the sheet now from its held edge, falls as made
    for k in 0..2 {
        assert!((wet[k] - flat[k] / (1.0 + e)).abs() < 0.05, "{k}: {} against {}", wet[k], flat[k] / (1.0 + e));
    }
    // a sheet that dried by 0.5 % between two pulls: out by 0.005·L at L from the held edge
    let drier = centres((e - 0.005, e - 0.005), true);
    let shift = drier[1] - wet[1];
    assert!((shift - flat[1] * (1.0 / (1.0 + e - 0.005) - 1.0 / (1.0 + e))).abs() < 0.05 && shift > 1.0, "{shift} px");
    // without the laws the sheet's size isn't read
    assert_eq!(centres((e, e), false), flat);
}

/// A level 2 N drypoint line on a 6 mm plate at 25 µm cells under the contact
/// laws, carded, wiped round twice with a tarlatan refolded between, palmed
/// once along it, and pulled on a damp sheet at the plate's pitch. Returns the
/// optical density across it (log₁₀ of the open plate's luminance over each
/// row's, rows averaged along the line's middle 3 mm), its row 0 at the line,
/// and the pitch (µm).
fn printed_2n_line() -> (Vec<f64>, f64) {
    let mut p = Plate::new(Metal::Copper, Temper::HalfHard, 6.0, 6.0, 1.3, 45.0, 240, 240);
    p.contact = true;
    let line = vec![
        crate::plate::Point { x: 0.5, y: 3.0, force: 2.0, alt: 90.0, az: 0.0, speed: 25.0 },
        crate::plate::Point { x: 5.5, y: 3.0, force: 2.0, alt: 90.0, az: 0.0, speed: 25.0 },
    ];
    p.apply(crate::plate::Op::Drypoint { tip_um: 25.0, cone_deg: 30.0, pts: line });
    p.ink(&black(), InkTool::of(Inker::Card), &[pt(-25.0, 1.0, 10.0), pt(30.0, 1.0, 10.0), pt(-25.0, 5.0, 10.0), pt(30.0, 5.0, 10.0)], 0.0).unwrap();
    let ring: Vec<crate::plate::Point> = (0..=72).map(|i| {
        let a = i as f32 / 24.0 * std::f32::consts::TAU;
        pt(3.0 + 6.0 * a.cos(), 3.0 + 6.0 * a.sin(), 0.5)
    }).collect();
    let mut t = Rag::tarlatan(40.0, 3, 0.7);
    p.wipe(&mut t, &ring, 1, 1.0);
    t.refold(&mut crate::tally::Tally::default());
    p.wipe(&mut t, &ring, 1, 1.0);
    p.hand_wipe(&[pt(-40.0, 3.0, 0.5), pt(46.0, 3.0, 0.5)], 1, false, 1.5);
    let mut sh = Canvas::new_window(480, 1.0, [0.9; 3], None).with_engine(9).with_size_mm(12.0).with_paper(crate::paper::Paper::drawing(4));
    let fe = feel(&sh, true);
    p.press(&mut sh, (3.0, 3.0), &PressSpec::default(), &fe, &[paint()], 2.0).unwrap();
    let w = 480;
    let seen = sh.seen();
    let lum = |x: usize, y: usize| {
        let c = seen[y * w + x];
        0.2126 * c[0] as f64 + 0.7152 * c[1] as f64 + 0.0722 * c[2] as f64
    };
    let row = |y: usize| (180..300).map(|x| lum(x, y)).sum::<f64>() / 120.0;
    let rows: Vec<f64> = (120..360).map(row).collect();
    // (the line near the middle, away from the plate's inked edges)
    let (i0, _) = rows.iter().enumerate().skip(100).take(40).fold((0, f64::MAX), |b, (i, &v)| if v < b.1 { (i, v) } else { b });
    // the open plate 1.0–1.5 mm off, both sides, as the scans' paper between lines
    let open: Vec<f64> = (40..60).flat_map(|k| [rows[i0 - k], rows[i0 + k]]).collect();
    let refl = open.iter().sum::<f64>() / open.len() as f64;
    let od: Vec<f64> = (0..80).map(|k| (refl / rows[i0 + k].min(rows[i0 - k]).max(1e-6)).log10()).collect();
    (od, 25.0)
}

/// A printed drypoint line against real ones (notes/research/intaglio_physics.md
/// §5.7: The Met's open-access scans, linearised): Rembrandt's pure drypoint
/// *Clump of Trees with a Vista* (1652) has half-peak widths of 0.17–0.34 mm and
/// OD above 0.3 over 0.17–0.57 mm; Whistler's *Weary* (1863) 0.21 mm and OD above
/// 0.1 over 0.32 mm. Peaked, no plateau: hatching 0.1–0.3 mm apart stays open.
/// The engine's 2 N line, wiped as a printer wipes it, is peaked too: its
/// half-peak width no more than 0.35 mm and its OD no more than 0.15 at 0.3 mm
/// off (it printed a plateau of OD 0.7–0.8 out there before the lee took the
/// film by its fibres in it). It is a line: dark at its core, with velvet by
/// the burr: the ink on the fin's faces and against them, which the yarn
/// resting on its crest and the palm's ridge riding it can't reach and the
/// damp sheet, pressed round the fin, takes. From below, by the scans: OD
/// above 0.3 over at least Rembrandt's narrowest, 0.17 mm (it was 0.125 mm
/// while the fin's own ink was wiped as the open plate). Its half-peak and its
/// OD above 0.1 stay below the scans' (0.075 and 0.275 mm): its core prints
/// twice its velvet's OD, and §5.7 says why that isn't bounded here.
#[test]
fn a_printed_drypoint_line_is_peaked_like_real_ones() {
    let (od, um) = printed_2n_line();
    let peak = od[0];
    assert!(peak > 1.0, "the line prints dark: OD {peak}");
    // (rows at `um` µm; a width counts both sides and the middle row)
    let width = |t: f64| (2.0 * od.iter().position(|&v| v < t).unwrap() as f64 - 1.0) * um * 1e-3;
    let half = width(0.5 * peak);
    assert!(half <= 0.35, "half-peak width {half} mm: {od:?}");
    let k = (300.0 / um).round() as usize;
    assert!(od[k] <= 0.15, "OD {} at 0.3 mm off: {od:?}", od[k]);
    assert!(od[(50.0 / um) as usize] > 0.1, "the velvet by the burr: {od:?}");
    let dark = width(0.3);
    assert!(dark >= 0.17, "OD above 0.3 over {dark} mm, Rembrandt's narrowest 0.17: {od:?}");
}
