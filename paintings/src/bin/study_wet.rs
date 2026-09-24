//! Wet on wet: the same four gestures laid over paint that is open (fresh),
//! setting (near its gel point), tacky (set, sticky) and touch-dry, with the
//! same brush, load, colors and seed in every column. It measures what each
//! gesture did (edge widths, how clean a light stays, how much the brush
//! picked up and carried) and prints a table; `notes/wet.md` reads it.
//!
//! Rows (top to bottom), columns open · setting · tacky · dry:
//! 1. **lights into a dark mass**: touches and short strokes of a stiff,
//!    loaded light green into a dark green mass (foliage);
//! 2. **sky down across a hill**: a light sky brought down in level strokes
//!    across the top of a blue-gray hill, then the join worked along with a
//!    clean badger (a lost edge);
//! 3. **contact shadow**: a dark stroke dragged along the base of a light
//!    form where it meets the ground color;
//! 4. **thick light over thin dark**: a loaded stiff light stroke laid
//!    lightly over a thin medium-rich dark (upper stroke), and the same color
//!    thinned and pressed (lower stroke).
//!
//! `cargo paint study_wet` (1000 px, square). `--width 2000` for detail.
//! After the table it runs the sketchbook's three pitfalls (translucent
//! rock, fog band, plowed river) in painter's terms. `WET_LOOK=1` also
//! prints the table for the wet look before drying; `PITFALL_PNG=<dir>`
//! saves the pitfall canvases.

use paint::color::to_oklab;
use paint::drying::drier;
use paint::{Canvas, Gesture, Held, Mask, Orient, Paint, Stage, Style, Tool, Touch, hex};
use paintings::study::{Img, MM, de, field, median, rect, width};

const CELL: f32 = 250.0;
const COLS: [&str; 4] = ["open", "setting", "tacky", "dry"];

fn cell(col: usize, row: usize) -> (f32, f32) {
    (col as f32 * CELL, row as f32 * CELL)
}

// ---- the four rows: underlayers and gestures (cell origin (ox, oy)) ----

fn dark_green() -> Paint {
    Paint::body(hex("#27331f")).with_drying(drier::OCHRE)
}
fn light_green() -> Paint {
    Paint::body(hex("#b7bd72")).with_drying(drier::CHROME_YELLOW)
}
fn hill() -> Paint {
    Paint::body(hex("#3b4659")).with_drying(drier::SMALT)
}
fn sky() -> Paint {
    Paint::body(hex("#c3ccd0")).with_drying(drier::LEAD_WHITE)
}
fn rock() -> Paint {
    Paint::body(hex("#c9c1ae")).with_drying(drier::LEAD_WHITE)
}
fn earth() -> Paint {
    Paint::body(hex("#76704f")).with_drying(drier::OCHRE)
}
fn shadow() -> Paint {
    Paint::body(hex("#2f2a26")).with_drying(drier::UMBER)
}
fn thin_dark() -> Paint {
    Paint::new(hex("#25262e"), 0.6, 0.25).with_drying(drier::BONE_BLACK)
}
fn thick_light() -> Paint {
    Paint::body(hex("#e3dbc4")).with_drying(drier::LEAD_WHITE)
}

/// Row 1: the dark mass.
fn under_mass(c: &mut Canvas, st: &Style, (ox, oy): (f32, f32)) {
    let (cx, cy) = (ox + 125.0, oy + 125.0);
    let m = Mask::from_fn(c.frame(), move |x, y| if ((x - cx) / 95.0).powi(2) + ((y - cy) / 85.0).powi(2) < 1.0 { 1.0 } else { 0.0 });
    field(c, st, dark_green().color, &m, 0.2, 11);
}
/// Where the touches of row 1 land (relative to the cell).
const DABS: [(f32, f32); 6] = [(90.0, 80.0), (120.0, 72.0), (150.0, 86.0), (105.0, 108.0), (140.0, 115.0), (170.0, 106.0)];
fn dabs(c: &mut Canvas, (ox, oy): (f32, f32)) {
    let mut h = Held::new(Tool::filbert(10.0), 21);
    for (k, &(x, y)) in DABS.iter().enumerate() {
        if k % 3 == 0 {
            h.reload(light_green(), 0.9);
        }
        c.touch(&mut h, &Touch::at(ox + x, oy + y).pressure(0.8).drag(2.0, 1.0).angle(0.5), None);
    }
    // three short strokes, one load
    h.reload(light_green(), 0.9);
    for k in 0..3 {
        let (x, y) = (ox + 80.0 + 35.0 * k as f32, oy + 150.0 + 6.0 * k as f32);
        c.drag(&mut h, &Gesture::new(vec![(x, y), (x + 14.0, y - 5.0), (x + 24.0, y - 4.0)]).pressure(0.6, 0.3).orient(Orient::Across), None);
    }
}

/// Row 2: the hill's top edge (y, cell-relative) at cell-relative x.
fn ridge(x: f32) -> f32 {
    125.0 + 10.0 * (x / 37.0).sin() + 4.0 * (x / 11.0).sin()
}
fn under_hill(c: &mut Canvas, st: &Style, (ox, oy): (f32, f32)) {
    let m = Mask::from_fn(c.frame(), move |x, y| if x > ox + 15.0 && x < ox + 235.0 && y > oy + ridge(x - ox) && y < oy + 235.0 { 1.0 } else { 0.0 });
    field(c, st, hill().color, &m, 0.2, 12);
}
/// The sky: laid in above, then brought down in strokes that follow the
/// hill's top, the last one reaching about 8 units (3.5 mm) into it. No
/// mask: the brush makes the edge.
fn sky_down(c: &mut Canvas, st: &Style, (ox, oy): (f32, f32)) {
    let top = Mask::from_fn(c.frame(), move |x, y| if x > ox + 15.0 && x < ox + 235.0 && y > oy + 15.0 && y < oy + ridge(x - ox) - 10.0 { 1.0 } else { 0.0 });
    field(c, st, sky().color, &top, 0.2, 16);
    let mut h = Held::new(Tool::hog_flat(12.0), 22);
    for (k, dy) in [-14.0f32, -10.0, -6.0, -2.0, 2.0].into_iter().enumerate() {
        if k % 2 == 0 {
            h.reload(sky(), 0.8);
        }
        let pts: Vec<(f32, f32)> = (0..=20).map(|j| {
            let x = 20.0 + 10.5 * j as f32;
            (ox + x, oy + ridge(x) + dy)
        }).collect();
        c.drag(&mut h, &Gesture::new(pts).pressure(0.8, 0.75).orient(Orient::Across), None);
    }
}
/// A clean badger worked along the join, three light passes.
fn badger(c: &mut Canvas, (ox, oy): (f32, f32)) {
    let mut soft = Held::new(Tool::badger(22.0), 23);
    for dy in [-6.0, 0.0, 6.0] {
        let pts: Vec<(f32, f32)> = (0..=10).map(|k| {
            let x = 20.0 + 21.0 * k as f32;
            (ox + x, oy + ridge(x) + dy)
        }).collect();
        c.drag(&mut soft, &Gesture::new(pts).pressure(0.35, 0.3), None);
    }
}

/// Row 3: the base of the light form (y, cell-relative) at cell-relative x.
fn base(x: f32) -> f32 {
    let u = (x - 125.0) / 80.0;
    150.0 + 22.0 * (1.0 - u * u).max(0.0).sqrt()
}
fn under_form(c: &mut Canvas, st: &Style, (ox, oy): (f32, f32)) {
    let g = rect(c, ox + 15.0, oy + 150.0, ox + 235.0, oy + 235.0);
    field(c, st, earth().color, &g, 0.2, 13);
    let r = Mask::from_fn(c.frame(), move |x, y| {
        let u = (x - ox - 125.0) / 80.0;
        if u.abs() < 1.0 && y > oy + 150.0 - 90.0 * (1.0 - u * u).sqrt() && y < oy + base(x - ox) { 1.0 } else { 0.0 }
    });
    field(c, st, rock().color, &r, 0.2, 14);
}
fn contact(c: &mut Canvas, (ox, oy): (f32, f32)) {
    let mut h = Held::new(Tool::filbert(6.0), 24);
    for k in 0..2 {
        h.reload(shadow(), 0.8);
        let pts: Vec<(f32, f32)> = (0..=12).map(|j| {
            let x = 58.0 + 11.0 * j as f32;
            (ox + x, oy + base(x) - 1.0 + 2.0 * k as f32)
        }).collect();
        c.drag(&mut h, &Gesture::new(pts).pressure(0.7, 0.55).orient(Orient::Across), None);
    }
}

/// Row 4: a thin, medium-rich dark.
fn under_thin(c: &mut Canvas, st: &Style, (ox, oy): (f32, f32)) {
    let m = rect(c, ox + 15.0, oy + 30.0, ox + 235.0, oy + 220.0);
    field(c, st, thin_dark().color, &m, 0.6, 15);
}
/// Stroke rows of row 4 (cell-relative y): the loaded stiff stroke, the thinned one.
const LIGHT_Y: [f32; 2] = [90.0, 160.0];
fn over_thin(c: &mut Canvas, (ox, oy): (f32, f32)) {
    let mut h = Held::new(Tool::hog_flat(10.0), 25);
    h.reload(thick_light(), 1.0);
    c.drag(&mut h, &Gesture::new(vec![(ox + 35.0, oy + LIGHT_Y[0]), (ox + 125.0, oy + LIGHT_Y[0] - 2.0), (ox + 215.0, oy + LIGHT_Y[0])]).pressure(0.45, 0.4).orient(Orient::Across), None);
    let thinned = thick_light().with_stiff(0.3).with_hiding(0.6);
    let mut h = Held::new(Tool::hog_flat(10.0), 26);
    h.reload(thinned, 0.45);
    c.drag(&mut h, &Gesture::new(vec![(ox + 35.0, oy + LIGHT_Y[1]), (ox + 125.0, oy + LIGHT_Y[1] - 2.0), (ox + 215.0, oy + LIGHT_Y[1])]).pressure(0.85, 0.8).orient(Orient::Across), None);
}

fn under(c: &mut Canvas, st: &Style, col: usize) {
    under_mass(c, st, cell(col, 0));
    under_hill(c, st, cell(col, 1));
    under_form(c, st, cell(col, 2));
    under_thin(c, st, cell(col, 3));
}
/// The gestures of row `row` in column `col`; row 2 (the sky) records its
/// edge width before the badger in `pre`.
fn gesture(c: &mut Canvas, st: &Style, col: usize, row: usize, pre: &mut [f32; 4]) {
    let o = cell(col, row);
    match row {
        0 => dabs(c, o),
        1 => {
            sky_down(c, st, o);
            pre[col] = edge(&Img::new(c, &c.seen()), col);
            badger(c, o);
        }
        2 => contact(c, o),
        _ => over_thin(c, o),
    }
}
/// A point in each row's underlayer, away from the gestures (cell-relative).
const PROBE: [(f32, f32); 4] = [(125.0, 200.0), (125.0, 200.0), (125.0, 200.0), (125.0, 125.0)];

// ---- measuring ----

/// Median 10–90% width (units) of the sky–hill join in column `col`.
fn edge(im: &Img, col: usize) -> f32 {
    let (ox, oy) = cell(col, 1);
    let hi = im.l(ox + 125.0, oy + 60.0, 8.0);
    let lo = im.l(ox + 125.0, oy + 200.0, 8.0);
    median((0..20).filter_map(|k| {
        let x = 30.0 + 9.5 * k as f32;
        let yc = ridge(x);
        width(&im.column(ox + x, oy + yc - 25.0, oy + yc + 25.0, 1.5), hi, lo, im.s, false)
    }).collect())
}

/// The metrics of every cell: printed as a table.
fn measure(img: &Img, pre: &[f32; 4]) -> Vec<String> {
    let mut out = Vec::new();
    let refl = |p: Paint| to_oklab(p.color)[0];
    out.push(format!("{:<44} {:>8} {:>8} {:>8} {:>8}", "(units: mm at the canvas's 440 mm width)", COLS[0], COLS[1], COLS[2], COLS[3]));
    let row = |name: &str, v: [String; 4]| format!("{name:<44} {:>8} {:>8} {:>8} {:>8}", v[0], v[1], v[2], v[3]);
    let f2 = |v: f32| if v.is_nan() { "–".to_string() } else { format!("{v:.2}") };
    let per = |g: &dyn Fn(usize) -> f32| -> [String; 4] { std::array::from_fn(|k| f2(g(k))) };
    // 1. lights into dark
    let lp = refl(light_green());
    let clean = |col: usize, k: usize| {
        let (ox, oy) = cell(col, 0);
        let d = img.l(ox + PROBE[0].0, oy + PROBE[0].1 - 30.0, 6.0);
        let (x, y) = DABS[k];
        (img.l(ox + x + 1.0, oy + y + 0.5, 1.5) - d) / (lp - d)
    };
    out.push(row("1 light touches: clean, first of a load", per(&|col| (clean(col, 0) + clean(col, 3)) / 2.0)));
    out.push(row("  clean, third of a load", per(&|col| (clean(col, 2) + clean(col, 5)) / 2.0)));
    let dab_area = |col: usize, soft: bool| {
        let (ox, oy) = cell(col, 0);
        let d = img.l(ox + PROBE[0].0, oy + PROBE[0].1 - 30.0, 6.0);
        // area (mm² per touch) that reads light (above halfway), or the share
        // of the mark that is a soft fringe (10–50%) around it
        let (mut core, mut fringe) = (0.0f32, 0.0f32);
        for &(x, y) in &DABS {
            for j in -16..=16 {
                for i in -16..=16 {
                    let n = (img.l(ox + x + i as f32 * 0.5, oy + y + j as f32 * 0.5, 0.5) - d) / (lp - d);
                    if n > 0.5 {
                        core += 1.0;
                    } else if n > 0.1 {
                        fringe += 1.0;
                    }
                }
            }
        }
        if soft { fringe / (core + fringe).max(1.0) } else { core * 0.25 * 0.44 * 0.44 / DABS.len() as f32 }
    };
    out.push(row("  area reading light, mm² per touch", per(&|col| dab_area(col, false))));
    out.push(row("  soft fringe share of the touch", per(&|col| dab_area(col, true))));
    let stroke_clean = |col: usize| {
        let (ox, oy) = cell(col, 0);
        let d = img.l(ox + PROBE[0].0, oy + PROBE[0].1 - 30.0, 6.0);
        let v: Vec<f32> = (0..3).map(|k| (img.l(ox + 80.0 + 35.0 * k as f32 + 8.0, oy + 150.0 + 6.0 * k as f32 - 2.5, 1.5) - d) / (lp - d)).collect();
        v.iter().sum::<f32>() / 3.0
    };
    out.push(row("  short strokes, clean", per(&stroke_clean)));
    // 2. sky across the hill
    out.push(row("2 sky over hill: edge 10–90% before badger", per(&|col| pre[col] * MM)));
    out.push(row("  edge 10–90% after badger (dried)", per(&|col| edge(img, col) * MM)));
    let dirty = |col: usize| {
        // how far the sky just above the join moved toward the hill (0 clean)
        let (ox, oy) = cell(col, 1);
        let s0 = img.at(ox + 125.0, oy + 60.0, 8.0);
        let h0 = img.at(ox + 125.0, oy + 200.0, 8.0);
        let v: Vec<f32> = (0..20).map(|k| {
            let x = 30.0 + 9.5 * k as f32;
            let p = img.at(ox + x, oy + ridge(x) - 14.0, 2.0);
            (s0[0] - p[0]) / (s0[0] - h0[0])
        }).collect();
        v.iter().sum::<f32>() / v.len() as f32
    };
    out.push(row("  sky 6 mm above the join, pulled to hill (0..1)", per(&dirty)));
    let bite = |col: usize| {
        // how far the hill 6 mm below the join moved toward the sky
        let (ox, oy) = cell(col, 1);
        let s0 = img.at(ox + 125.0, oy + 60.0, 8.0);
        let h0 = img.at(ox + 125.0, oy + 200.0, 8.0);
        let v: Vec<f32> = (0..20).map(|k| {
            let x = 30.0 + 9.5 * k as f32;
            let p = img.at(ox + x, oy + ridge(x) + 14.0, 2.0);
            (p[0] - h0[0]) / (s0[0] - h0[0])
        }).collect();
        v.iter().sum::<f32>() / v.len() as f32
    };
    out.push(row("  hill 6 mm below the join, pulled to sky", per(&bite)));
    // 3. contact shadow
    let lsh = refl(shadow());
    let depth = |col: usize| {
        let (ox, oy) = cell(col, 2);
        let r = img.l(ox + 125.0, oy + 120.0, 8.0);
        let v: Vec<f32> = (0..10).map(|k| {
            let x = 70.0 + 11.0 * k as f32;
            let p = img.column(ox + x, oy + base(x) - 8.0, oy + base(x) + 8.0, 1.0);
            let m = p.iter().cloned().fold(f32::MAX, f32::min);
            (r - m) / (r - lsh)
        }).collect();
        v.iter().sum::<f32>() / v.len() as f32
    };
    out.push(row("3 contact shadow: depth reached (1 = paint)", per(&depth)));
    let sh_edges = |col: usize, upper: bool| {
        let (ox, oy) = cell(col, 2);
        let r = img.l(ox + 125.0, oy + 120.0, 8.0);
        let g = img.l(ox + 125.0, oy + 222.0, 8.0);
        median((0..10).filter_map(|k| {
            let x = 70.0 + 11.0 * k as f32;
            let p = img.column(ox + x, oy + base(x) - 16.0, oy + base(x) + 16.0, 1.0);
            let (mi, &m) = p.iter().enumerate().min_by(|a, b| a.1.total_cmp(b.1))?;
            if upper { width(&p[..=mi], r, m, img.s, false) } else { width(&p[mi..], m, g, img.s, false) }
        }).collect()) * MM
    };
    out.push(row("  edge to the form above, 10–90%", per(&|col| sh_edges(col, true))));
    out.push(row("  edge to the ground below, 10–90%", per(&|col| sh_edges(col, false))));
    // 4. thick light over thin dark
    let lt = refl(thick_light());
    for (k, name) in ["4 loaded stiff light, clean (start/end)", "  thinned light pressed, clean (start/end)"].iter().enumerate() {
        let v = per(&|col| {
            let (ox, oy) = cell(col, 3);
            let d = img.l(ox + PROBE[3].0, oy + PROBE[3].1, 6.0);
            let n = |x: f32| (img.l(ox + x, oy + LIGHT_Y[k] - 2.0 * (1.0 - ((x - 125.0) / 90.0).powi(2)), 2.0) - d) / (lt - d);
            (n(55.0) + n(75.0)) / 2.0
        });
        let e = per(&|col| {
            let (ox, oy) = cell(col, 3);
            let d = img.l(ox + PROBE[3].0, oy + PROBE[3].1, 6.0);
            let n = |x: f32| (img.l(ox + x, oy + LIGHT_Y[k] - 2.0 * (1.0 - ((x - 125.0) / 90.0).powi(2)), 2.0) - d) / (lt - d);
            (n(175.0) + n(195.0)) / 2.0
        });
        out.push(row(name, std::array::from_fn(|i| format!("{}/{}", v[i], e[i]))));
    }
    // hue: how far the stiff light's color is from its own paint over a dry dark (ΔE OKLab)
    let hue = per(&|col| {
        let (ox, oy) = cell(col, 3);
        let (_, oy3) = cell(3, 3);
        de(img.at(ox + 65.0, oy + LIGHT_Y[0] - 1.5, 2.0), img.at(3.0 * CELL + 65.0, oy3 + LIGHT_Y[0] - 1.5, 2.0))
    });
    out.push(row("  stiff light ΔE vs the dry column", hue));
    out
}

/// The sketchbook's three pitfalls, in painter's terms (the style's
/// handlings, as at the easel), each over a passage still open (half an
/// hour old) and over the same passage dried first. Prints how far the later passage got toward
/// its look over the dried one (1 = all the way), and for the river the
/// share of its track that shows the ground.
fn pitfalls(width: usize) {
    let st = Style::friedrich();
    let run = |dry_first: bool| -> [f32; 4] {
        let mut c = st.prepare(width / 2, 2.0, 5);
        let col = |h: &str| {
            let c = hex(h);
            move |_: f32, _: f32| c
        };
        // translucent rock: a thick body floor, then a rock over it
        c.work(&rect(&c, 20.0, 40.0, 320.0, 460.0), &st.body().color(col("#4a4636")).coverage(4.5).clip(true), 1);
        // fog band: a near-black wood, then snow brought up over its foot
        c.work(&rect(&c, 360.0, 40.0, 640.0, 300.0), &st.body().color(col("#1b221f")).coverage(3.0).clip(true), 2);
        // plowed river: valley paint, then a river laid with a flat into it
        c.work(&rect(&c, 680.0, 40.0, 980.0, 460.0), &st.body().color(col("#62703c")).coverage(3.0).clip(true), 3);
        if dry_first {
            c.dry();
        } else {
            c.wait(30.0);
            println!("  after wait(30): floor {:?}, wood {:?}, valley {:?}", c.drying_at(170.0, 250.0), c.drying_at(500.0, 270.0), c.drying_at(800.0, 215.0));
        }
        let rock = Mask::from_fn(c.frame(), |x, y| if ((x - 170.0) / 100.0).powi(2) + ((y - 250.0) / 80.0).powi(2) < 1.0 { 1.0 } else { 0.0 });
        c.work(&rock, &st.body().color(col("#bdb4a2")).coverage(2.5).clip(true), 4);
        c.work(&rect(&c, 360.0, 250.0, 640.0, 460.0), &st.body().color(col("#e4e8ea")).coverage(3.0).clip(true), 5);
        let mut h = Held::new(Tool::hog_flat(8.0), 6);
        for k in 0..6 {
            h.reload(Paint::body(hex("#9fb0b6")), 0.6);
            let y = 200.0 + 6.0 * k as f32;
            c.drag(&mut h, &Gesture::new(vec![(700.0, y), (830.0, y + 20.0), (960.0, y + 10.0)]).pressure(0.9, 0.85).orient(Orient::Across), None);
        }
        c.dry();
        if let Ok(d) = std::env::var("PITFALL_PNG") {
            c.save(format!("{d}/pitfalls_{}.png", if dry_first { "dried" } else { "open" })).unwrap();
        }
        let img = Img::new(&c, c.pixels());
        let ground = to_oklab(st.prepare(8, 1.0, 5).pixels()[0]);
        let floor = img.l(170.0, 420.0, 10.0);
        let wood = img.l(500.0, 150.0, 10.0);
        let track = (0..40).filter(|&k| {
            let x = 720.0 + 5.8 * k as f32;
            let y = 215.0 + if x < 830.0 { 20.0 * (x - 700.0) / 130.0 } else { 20.0 - 10.0 * (x - 830.0) / 130.0 };
            (-10..=10).any(|dy| de(img.at(x, y + dy as f32, 0.0), ground) < 0.06)
        }).count() as f32 / 40.0;
        [img.l(170.0, 250.0, 30.0) - floor, img.l(500.0, 270.0, 8.0) - wood, track, 0.0]
    };
    println!("pitfalls (the later passage 30 min after, or after dry()):");
    let (open, dried) = (run(false), run(true));
    println!("  translucent rock: the rock gets {:.2} of the way to its look over a dried floor", open[0] / dried[0]);
    println!("  fog band: snow over the wood's foot gets {:.2} of the way", open[1] / dried[1]);
    println!("  plowed river: {:.0}% of the river's track shows the ground (dried first: {:.0}%)", open[2] * 100.0, dried[2] * 100.0);
}

fn stages(c: &Canvas, col: usize) -> String {
    (0..4).map(|row| {
        let (ox, oy) = cell(col, row);
        format!("{:?}", c.drying_at(ox + PROBE[row].0, oy + PROBE[row].1))
    }).collect::<Vec<_>>().join(" ")
}

fn main() {
    let o = paintings::run::Run::new("study_wet");
    let st = Style::friedrich();
    let mut c = st.prepare(o.width, 1.0, o.seed);
    let t0 = c.clock();
    // dry column: underlayers laid and left to dry
    under(&mut c, &st, 3);
    c.dry();
    // tacky column: laid, then left until every row's film has set
    under(&mut c, &st, 2);
    let set = |c: &Canvas, col: usize| (0..4).all(|row| {
        let (ox, oy) = cell(col, row);
        c.drying_at(ox + PROBE[row].0, oy + PROBE[row].1) == Stage::Tacky
    });
    let mut n = 0;
    while !set(&c, 2) && n < 400 {
        c.wait(15.0);
        n += 1;
    }
    println!("tacky column set after {:.0} min", c.clock() - t0 - 0.0);
    // setting column: laid, then each row's gesture as soon as its film is setting
    under(&mut c, &st, 1);
    let t1 = c.clock();
    let mut done = [false; 4];
    let mut pre = [f32::NAN; 4];
    let mut n = 0;
    while done.iter().any(|d| !d) && n < 800 {
        c.wait(5.0);
        n += 1;
        for row in 0..4 {
            let (ox, oy) = cell(1, row);
            let s = c.drying_at(ox + PROBE[row].0, oy + PROBE[row].1);
            if !done[row] && s != Stage::Open {
                assert_eq!(s, Stage::Setting, "row {row} of the setting column skipped setting");
                println!("setting row {} at {:.0} min", row + 1, c.clock() - t1);
                gesture(&mut c, &st, 1, row, &mut pre);
                done[row] = true;
            }
        }
    }
    // tacky and dry columns
    println!("stages, tacky column: {}; dry column: {}", stages(&c, 2), stages(&c, 3));
    for col in [2, 3] {
        for row in 0..4 {
            gesture(&mut c, &st, col, row, &mut pre);
        }
    }
    // open column: laid and worked at once
    under(&mut c, &st, 0);
    println!("stages, open column: {}", stages(&c, 0));
    for row in 0..4 {
        gesture(&mut c, &st, 0, row, &mut pre);
    }
    if std::env::var("WET_LOOK").is_ok() {
        println!("-- the wet look, before drying --");
        for l in measure(&Img::new(&c, &c.seen()), &pre) {
            println!("{l}");
        }
    }
    c.dry();
    let img = Img::new(&c, c.pixels());
    for l in measure(&img, &pre) {
        println!("{l}");
    }
    o.save(&mut c);
    pitfalls(o.width);
}
