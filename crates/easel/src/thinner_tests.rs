//! `pile{..., thinner=}`: what solvent does to a pass of paint, measured
//! (notes/thinner/README.md). The table test prints the numbers there:
//!
//!   cargo test -p easel --release thinner_table -- --ignored --nocapture

use crate::session::Session;
use paint::Palette;
use paint::color::luminance;

/// The canvas: an Inness-box toned ground (lead white and raw umber, a
/// light brown stain's value) on which a black band and a white band (a
/// card) are painted thick and left to dry for 60 days.
const CANVAS: &str = r#"canvas{size=400, aspect=1, linen=15, seed=5, ground={{pile={{"lead white", 3}, {"raw umber", 1}}, um=90, apply="knife"}}}"#;

/// The rows (one per pile) and the bands across each: ground, black, white.
const ROW_H: f32 = 480.0;
const BAND_H: f32 = 160.0;

fn card() -> String {
    let mut s = String::from(
        r#"local k, w = pile{{"bone black", 1}}, pile{{"lead white", 1}}
"#,
    );
    for row in [0.0, 520.0] {
        let (b, wy) = (row + BAND_H, row + 2.0 * BAND_H);
        s += &format!(
            r#"work(rect(0, {b}, 1000, {BAND_H}), {{hand="body", pile=k, load=1, coverage=5, clip=true, seed=1}})
work(rect(0, {wy}, 1000, {BAND_H}), {{hand="body", pile=w, load=1, coverage=5, clip=true, seed=2}})
"#
        );
    }
    s + "wait(60 * 24 * 60)\n"
}

/// Mean linear RGB over a rectangle, sampled every unit.
fn mean(s: &Session, x0: f32, y0: f32, x1: f32, y1: f32) -> [f32; 3] {
    let c = s.canvas().unwrap();
    let (mut a, mut n) = ([0.0f64; 3], 0.0f64);
    let mut y = y0;
    while y < y1 {
        let mut x = x0;
        while x < x1 {
            let p = c.under(x, y, 0.0);
            for i in 0..3 {
                a[i] += p[i] as f64;
            }
            n += 1.0;
            x += 1.0;
        }
        y += 1.0;
    }
    a.map(|v| (v / n) as f32)
}

/// How far `seen` has gone from `under` toward `mass`, 0 (untouched) .. 1
/// (masstone), along the line between them in linear RGB.
fn share(under: [f32; 3], seen: [f32; 3], mass: [f32; 3]) -> f32 {
    let d: [f32; 3] = std::array::from_fn(|i| mass[i] - under[i]);
    let e: [f32; 3] = std::array::from_fn(|i| seen[i] - under[i]);
    (d[0] * e[0] + d[1] * e[1] + d[2] * e[2]) / (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).max(1e-9)
}

/// One pile's pass at each (load, thinner): its share of masstone over the
/// ground, over black and over white.
struct Cell {
    load: f32,
    thinner: f32,
    ground: f32,
    black: f32,
    white: f32,
    /// The card's black and white bands before the pass and after it.
    card0: [[f32; 3]; 2],
    card1: [[f32; 3]; 2],
}

impl Cell {
    /// How much of the card's black/white contrast (in luminance) still
    /// shows through the pass: 1 untouched, 0 hidden.
    fn shows(&self) -> f32 {
        contrast(self.card1) / contrast(self.card0)
    }
}

fn contrast(card: [[f32; 3]; 2]) -> f32 {
    luminance(card[1]) - luminance(card[0])
}

/// The piles: a lead-white mixture and a transparent earth.
const PILES: [(&str, &str); 2] = [("lead white 6, yellow ochre 1", r#"{"lead white", 6}, {"yellow ochre", 1}"#), ("raw sienna", r#"{"raw sienna", 1}"#)];

/// Lay a strip of `hand` for every load × thinner over every band, both
/// piles, and measure.
fn measure(width: usize, hand: &str, loads: &[f32], thinners: &[f32]) -> Vec<(String, Vec<Cell>)> {
    let pal = Palette::named_box("inness").unwrap();
    let mut s = Session::with_box(width, pal.clone()).unwrap();
    s.run(CANVAS).unwrap();
    s.run(&card()).unwrap();
    let n = loads.len() * thinners.len();
    let col = 1000.0 / n as f32;
    let (sw, inset) = (col * 0.78, 10.0);
    let mut out = Vec::new();
    for (r, (name, parts)) in PILES.iter().enumerate() {
        let row = r as f32 * 520.0;
        let idx = |name: &str| pal.tubes.iter().position(|t| t.name == name).unwrap();
        let mass = match r {
            0 => pal.pile(vec![(idx("lead white"), 6.0 / 7.0), (idx("yellow ochre"), 1.0 / 7.0)]).color,
            _ => pal.pile(vec![(idx("raw sienna"), 1.0)]).color,
        };
        // what each band of each strip is before the pass
        let rect = |c: usize, band: usize| {
            let x0 = c as f32 * col + (col - sw) / 2.0;
            let y0 = row + band as f32 * BAND_H;
            (x0 + inset, y0 + inset, x0 + sw - inset, y0 + BAND_H - inset)
        };
        let before: Vec<[[f32; 3]; 3]> = (0..n).map(|c| std::array::from_fn(|b| {
            let (a, bb, cc, d) = rect(c, b);
            mean(&s, a, bb, cc, d)
        })).collect();
        let mut chunk = String::new();
        let mut cells = Vec::new();
        for (li, &load) in loads.iter().enumerate() {
            for (ti, &t) in thinners.iter().enumerate() {
                let c = li * thinners.len() + ti;
                let x0 = c as f32 * col + (col - sw) / 2.0;
                let th = if t > 0.0 { format!(", thinner={t}") } else { String::new() };
                chunk += &format!(
                    "work(rect({x0}, {row}, {sw}, {ROW_H}), {{hand=\"{hand}\", pile=pile{{{parts}{th}}}, load={load}, clip=true, seed={}}})\n",
                    11 + c
                );
                cells.push((c, load, t));
            }
        }
        s.run(&chunk).unwrap();
        let cells = cells
            .into_iter()
            .map(|(c, load, thinner)| {
                let sh = |b: usize| {
                    let (x0, y0, x1, y1) = rect(c, b);
                    share(before[c][b], mean(&s, x0, y0, x1, y1), mass)
                };
                let at = |b: usize| {
                    let (x0, y0, x1, y1) = rect(c, b);
                    mean(&s, x0, y0, x1, y1)
                };
                Cell { load, thinner, ground: sh(0), black: sh(1), white: sh(2), card0: [before[c][1], before[c][2]], card1: [at(1), at(2)] }
            })
            .collect();
        out.push((name.to_string(), cells));
    }
    out
}

/// The minutes from laying a patch until its center is setting and until it
/// is tacky (past the gel point), checked every 5 minutes of `wait`.
fn gel_minutes(width: usize, parts: &str, thinner: f32, load: f32) -> (f32, f32) {
    let mut s = Session::with_box(width, Palette::named_box("inness").unwrap()).unwrap();
    s.run(CANVAS).unwrap();
    let th = if thinner > 0.0 { format!(", thinner={thinner}") } else { String::new() };
    s.run(&format!("work(rect(300, 300, 400, 400), {{hand=\"body\", pile=pile{{{parts}{th}}}, load={load}, clip=true, seed=3}})")).unwrap();
    let r = s
        .run(
            r#"local pts, set, tack, t = {}, nil, nil, 0
for i = 0, 4 do for j = 0, 4 do pts[#pts + 1] = {420 + 40 * i, 420 + 40 * j} end end
local function count(st) local n = 0 for _, p in ipairs(pts) do local d = drying(p[1], p[2]) if d == st or (st == "setting" and (d == "tacky" or d == "dry")) or (st == "tacky" and d == "dry") then n = n + 1 end end return n end
while t < 3 * 24 * 60 and not tack do
  wait(5); t = t + 5
  if not set and count("setting") >= 13 then set = t end
  if count("tacky") >= 13 then tack = t end
end
print(set or -1, tack or -1)"#,
        )
        .unwrap();
    let v: Vec<f32> = r.out.split_whitespace().map(|w| w.parse().unwrap()).collect();
    (v[0], v[1])
}

fn table(rows: &[(String, Vec<Cell>)]) -> String {
    let mut t = String::from("| pile | load | thinner | share of masstone over the ground | over black | over white | card contrast showing |\n|---|---|---|---|---|---|---|\n");
    for (name, cells) in rows {
        for c in cells {
            t += &format!(
                "| {name} | {} | {} | {:.0}% | {:.0}% | {:.0}% | {:.0}% |\n",
                c.load,
                c.thinner,
                100.0 * c.ground,
                100.0 * c.black,
                100.0 * c.white,
                100.0 * c.shows()
            );
        }
    }
    t
}

/// One body pass of a pile over a 400-unit square on a fresh canvas, and
/// right after it, over the square's middle: the mean wet film (µm) and
/// the pigment there (film × scattering per coat: Kubelka–Munk's S per
/// area, which pigment carries). The paint is still open (it hasn't
/// reached its gel point, so none of it has gone into the dry picture).
fn laid(width: usize, parts: &str, thinner: f32, load: f32) -> (f32, f32) {
    let mut s = Session::with_box(width, Palette::named_box("inness").unwrap()).unwrap();
    s.run(CANVAS).unwrap();
    let th = if thinner > 0.0 { format!(", thinner={thinner}") } else { String::new() };
    s.run(&format!("work(rect(300, 300, 400, 400), {{hand=\"body\", pile=pile{{{parts}{th}}}, load={load}, clip=true, seed=3}})")).unwrap();
    let c = s.canvas().unwrap();
    let (mut film, mut pigment, mut n) = (0.0f64, 0.0f64, 0.0f64);
    for j in 0..200 {
        for i in 0..200 {
            let (x, y) = (400.0 + i as f32, 400.0 + j as f32);
            assert_ne!(c.drying_at(x, y), paint::Stage::Tacky, "the pass is still wet");
            let (um, sc) = c.wet_film(x, y);
            film += um as f64;
            pigment += (um * sc) as f64;
            n += 1.0;
        }
    }
    ((film / n) as f32, (pigment / n) as f32)
}

/// The numbers for notes/thinner/README.md, at the live width (slow: run it
/// in release, see the module's comment).
#[test]
#[ignore = "prints the README's numbers at the live width; no assertions"]
fn thinner_table() {
    for hand in ["body", "scumble"] {
        let rows = measure(2400, hand, &[0.1, 0.3, 0.6], &[0.0, 0.3, 0.5, 0.7]);
        println!("hand {hand}\n{}", table(&rows));
    }
    for (name, parts) in [("lead white 6, yellow ochre 1", r#"{"lead white", 6}, {"yellow ochre", 1}"#), ("raw sienna", r#"{"raw sienna", 1}"#)] {
        for load in [0.3, 0.6] {
            let (f0, p0) = laid(2400, parts, 0.0, load);
            for t in [0.0, 0.3, 0.5, 0.7] {
                let (f, p) = laid(2400, parts, t, load);
                println!("laid | {name} | load {load} | thinner {t} | wet film {f:.1} µm ({:.2}) | pigment {:.2} of unthinned | (1 - t) {:.2}", f / f0, p / p0, 1.0 - t);
            }
        }
        for t in [0.0, 0.3, 0.5, 0.7] {
            let (set, tack) = gel_minutes(2400, parts, t, 0.6);
            println!("drying | {name} | thinner {t} | setting at {set} min | tacky at {tack} min");
        }
    }
}

/// Targets from the physics (notes/thinner/README.md), not from what the
/// code lays:
/// 1. Evaporation is in `thinned_paint_lays_less_pigment`; here, more
///    solvent lets more of the card show.
/// 2. Raw sienna thinned half is an imprimatura: `an_earth_thinned_half_is_an_imprimatura`.
/// 3. The lead-white mixture is a scumble, not a glaze: thinned half, at
///    load 0.6 it still hides at least half the card's contrast, and it
///    hides less than the unthinned pass. More solvent, less hidden.
#[test]
fn a_thinned_pass_reads_as_thin() {
    let rows = measure(600, "body", &[0.3, 0.6], &[0.0, 0.5, 0.7]);
    let get = |r: usize, load: f32, t: f32| rows[r].1.iter().find(|c| c.load == load && c.thinner == t).unwrap();
    eprintln!("{}", table(&rows));
    for load in [0.3, 0.6] {
        for r in 0..2 {
            let none = get(r, load, 0.0);
            let mut last = none.shows();
            for t in [0.5, 0.7] {
                let c = get(r, load, t);
                assert!(c.shows() > last, "{} load {load} thinner {t}: more solvent, more shows", rows[r].0);
                last = c.shows();
            }
        }
    }
    let white = get(0, 0.6, 0.5);
    assert!(white.shows() <= 0.5, "lead white thinned half at load 0.6 is still a scumble: {:.2} of the card shows", white.shows());
}

/// Raw sienna, a transparent earth, thinned half with solvent is an
/// imprimatura: at least half the card's black/white contrast shows
/// through it at loads 0.3 and 0.6. FAILS: measured (width 600) 0.49 at
/// load 0.3 and 0.20 at load 0.6. The pass lays 0.32 of the unthinned
/// pigment (under 1 - t), but one coat of tube raw sienna (hiding 0.4)
/// lets only ~20-25% of the card show; see notes/thinner/README.md.
#[test]
fn an_earth_thinned_half_is_an_imprimatura() {
    let rows = measure(600, "body", &[0.3, 0.6], &[0.0, 0.5]);
    let shows: Vec<(f32, f32)> = [0.3, 0.6].iter().map(|&load| (load, rows[1].1.iter().find(|c| c.load == load && c.thinner == 0.5).unwrap().shows())).collect();
    assert!(shows.iter().all(|&(_, v)| v >= 0.5), "raw sienna thinned half shows (load, share of the card's contrast) {shows:.2?}; an imprimatura shows at least 0.5");
}

/// The solvent evaporates, so the dry film is (1 - t) of the wet film: a
/// thinned pass lays at most (1 - t) of the pigment per area that the same
/// pass lays unthinned (5% slack for the hand's unevenness). Thinned paint
/// also spreads further (`bristle::run_of`), so it lays less wet paint too.
#[test]
fn thinned_paint_lays_less_pigment() {
    for parts in [r#"{"lead white", 6}, {"yellow ochre", 1}"#, r#"{"raw sienna", 1}"#] {
        for load in [0.3, 0.6] {
            let (f0, p0) = laid(300, parts, 0.0, load);
            for t in [0.5, 0.7] {
                let (f, p) = laid(300, parts, t, load);
                assert!(p <= (1.0 - t) * p0 * 1.05, "{parts} load {load} thinner {t}: pigment {:.2} of the unthinned pass's, at most {:.2}", p / p0, 1.0 - t);
                assert!(f < f0, "{parts} load {load} thinner {t}: wet film {f:.1} µm vs {f0:.1}");
            }
        }
    }
}

/// A thinned film reaches its gel point sooner than the same pile unthinned
/// (it is a thinner film), but only moderately: the oil cures as the
/// pile's own oil, so the speed-up is the engine's thickness law's, at most
/// 2.5× here (time ∝ film^0.7, floored at half a coat).
#[test]
fn a_thinned_film_sets_sooner() {
    let parts = r#"{"lead white", 6}, {"yellow ochre", 1}"#;
    let (s0, t0) = gel_minutes(300, parts, 0.0, 0.6);
    for t in [0.5, 0.7] {
        let (s1, t1) = gel_minutes(300, parts, t, 0.6);
        assert!(t0 > 0.0 && t1 > 0.0, "both set: {t0} {t1}");
        assert!(t1 < 0.8 * t0 && s1 <= s0, "thinner {t} sets sooner: setting {s1} vs {s0}, tacky {t1} vs {t0}");
        assert!(t1 >= t0 / 2.5, "thinner {t} sets only moderately sooner: tacky at {t1} min vs {t0}");
    }
}

/// A pile without thinner, or with thinner 0, paints exactly what it always
/// did; the same thinned chunk paints the same canvas twice; `print` and
/// `p.thinner` show it; out-of-range thinner is refused.
#[test]
fn thinner_absent_or_zero_is_unchanged_and_thinned_is_deterministic() {
    let bits = |chunk: &str| {
        let mut s = Session::with_box(300, Palette::named_box("inness").unwrap()).unwrap();
        s.run(CANVAS).unwrap();
        let r = s.run(chunk).unwrap();
        let c = s.canvas().unwrap();
        let mut v: Vec<u32> = c.seen().iter().flat_map(|p| p.map(f32::to_bits)).collect();
        v.extend(c.kept_surface_um().2.iter().map(|x| x.to_bits()));
        (v, r.out)
    };
    let pass = |opt: &str| {
        format!(
            r#"p = pile{{{{"lead white", 6}}, {{"raw sienna", 1}}, medium=0.2{opt}}}
work(rect(200, 200, 600, 600), {{hand="body", pile=p, load=0.4}})
b = brush("filbert", 8); b:load(p, 0.7); b:stroke({{{{100, 100}}, {{900, 850}}}})
stipple(ellipse(500, 500, 120, 90), {{pile=p}})
wait(90); print(p, p.thinner, drying(500, 500))"#
        )
    };
    let (absent, out_a) = bits(&pass(""));
    let (zero, out_z) = bits(&pass(", thinner=0"));
    assert!(absent == zero, "thinner=0 paints what no thinner does");
    assert_eq!(out_a, out_z);
    assert!(out_a.starts_with("pile(lead white 6, raw sienna 1; medium 0.2)\t0.0\t"), "{out_a}");
    let (t1, out_t) = bits(&pass(", thinner=0.5"));
    let (t2, _) = bits(&pass(", thinner=0.5"));
    assert!(t1 == t2, "a thinned chunk replays the same");
    assert!(t1 != absent, "thinner changes the paint");
    assert!(out_t.starts_with("pile(lead white 6, raw sienna 1; medium 0.2, thinner 0.5)\t0.5\t"), "{out_t}");
    let mut s = Session::with_box(100, Palette::named_box("inness").unwrap()).unwrap();
    s.run(CANVAS).unwrap();
    for bad in ["thinner=-0.1", "thinner=0.95", "thinner='a lot'"] {
        assert!(s.run(&format!(r#"pile{{{{"raw sienna", 1}}, {bad}}}"#)).is_err(), "{bad}");
    }
}

