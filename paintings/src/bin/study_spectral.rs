//! Study: spectral Kubelka–Munk (a port of spectral.js, `paint::spectral`)
//! against the engine's color model (Mixbox for mixing, KM per RGB channel
//! for layering), on painter's questions. A measuring sheet, not a
//! painting: each swatch is a computed color, laid flat so the models can
//! be compared side by side. The numbers are printed to stderr and
//! discussed in notes/spectral.md.
//!
//! Layout (1000 units wide, top to bottom):
//! A  greens, four pairs, blue → yellow in 11 steps by volume, each pair
//!    four strips:  1 Mixbox (the engine's palette)  2 KM per RGB channel
//!    (two-constant)  3 spectral KM (two-constant)  4 spectral.js `mix`
//!    pairs: Prussian blue + yellow ochre, Prussian blue + chrome yellow,
//!    smalt + yellow ochre, smalt + chrome yellow
//! B  tints, smalt → lead white and Prussian blue → lead white, the same
//!    four strips each
//! C  glazes, each row RGB-KM (upper strip) over spectral (lower strip):
//!    substrate, then 0.5, 1, 2, 4 coats, then the stack in the last cells
//!    umber over sky blue · madder over yellow ochre · Prussian blue over
//!    chrome yellow · umber over madder over sky blue
//! D  aged varnish (`Finish::aged`'s #e6d3a4) at 0, 0.4, 1, 2, 4 coats
//!    over a dark, a dark green, a deep blue, a mid sky, lead white;
//!    RGB-KM strip over spectral strip
//! F  the same questions with *pigment-shaped* spectra (`shape_of`: each
//!    tube's known spectral shape, fitted to its masstone): each block is
//!    Mixbox · spectral KM on the spectral.js basis · spectral KM shaped;
//!    mixes as in A/B, then glazes (0.5, 1, 2, 4 coats) and varnish
//!
//!   cargo paint study_spectral

use paint::color::{luminance, to_oklab};
use paint::pigment::{Pigment, ks_of, scatter_for};
use paint::spectral::{self, SpectralPigment};
use paint::{Canvas, Palette, Rgb, hex};
use std::hint::black_box;
use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq)]
enum How {
    Mixbox,
    RgbKm,
    SpecKm,
    SpecJs,
}
const HOWS: [How; 4] = [How::Mixbox, How::RgbKm, How::SpecKm, How::SpecJs];

/// All the tubes of the Friedrich palettes in one list.
fn tubes() -> Palette {
    let mut t = Palette::friedrich_early_greens().tubes;
    for x in Palette::friedrich_1820_greens().tubes {
        if !t.iter().any(|y| y.name == x.name) {
            t.push(x);
        }
    }
    Palette::new("all", t)
}

fn idx(p: &Palette, name: &str) -> usize {
    p.tubes.iter().position(|t| t.name == name).unwrap_or_else(|| panic!("{name}"))
}

/// Scattering per coat of tube `i` (from its hiding; cached, the solve is
/// a bisection).
fn scat(p: &Palette, i: usize) -> f32 {
    thread_local! {
        static S: std::cell::RefCell<Vec<f32>> = const { std::cell::RefCell::new(Vec::new()) };
    }
    S.with(|s| {
        let mut s = s.borrow_mut();
        if s.len() != p.tubes.len() {
            *s = p.tubes.iter().map(|t| scatter_for(luminance(t.color), t.hiding)).collect();
        }
        s[i]
    })
}

/// The masstone of a pile, four ways. Two-constant KM: K/S of the mix is
/// Σ w·S·(K/S) / Σ w·S with w = volume × tinting strength (the weights the
/// palette gives Mixbox), S of each tube from its hiding.
fn masstone(p: &Palette, parts: &[(usize, f32)], how: How) -> Rgb {
    match how {
        How::Mixbox => p.pile(parts.to_vec()).color,
        How::RgbKm => std::array::from_fn(|c| {
            let (mut num, mut den) = (0.0, 0.0);
            for &(i, f) in parts {
                let w = f * p.tubes[i].strength * scat(p, i);
                num += w * ks_of(p.tubes[i].color[c]);
                den += w;
            }
            spectral::km(num / den)
        }),
        How::SpecKm => {
            let ps: Vec<(SpectralPigment, f32)> = parts.iter().map(|&(i, f)| (SpectralPigment::masstone(p.tubes[i].color, scat(p, i)), f * p.tubes[i].strength)).collect();
            SpectralPigment::mix(&ps).masstone_color()
        }
        How::SpecJs => spectral::mix_js(&parts.iter().map(|&(i, f)| (p.tubes[i].color, f, p.tubes[i].strength)).collect::<Vec<_>>()),
    }
}

fn de(a: Rgb, b: Rgb) -> f32 {
    let (a, b) = (to_oklab(a), to_oklab(b));
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

/// OKLCh as text: lightness, chroma, hue in degrees.
fn lch(c: Rgb) -> String {
    let l = to_oklab(c);
    let h = l[2].atan2(l[1]).to_degrees();
    format!("L{:.2} C{:.3} h{:>4.0}", l[0], (l[1] * l[1] + l[2] * l[2]).sqrt(), if h < 0.0 { h + 360.0 } else { h })
}

fn hexs(c: Rgb) -> String {
    let q = |v: f32| (paint::color::linear_to_srgb(v) * 255.0).round() as u8;
    format!("#{:02x}{:02x}{:02x}", q(c[0]), q(c[1]), q(c[2]))
}

/// A tube's spectral shape (masstone reflectance, relative), written down
/// from what is known of the pigment's absorption; `spectral::fit_shape`
/// scales it to the tube's masstone. Approximations from pigment knowledge
/// (qualitative features, not measured curves):
/// - smalt: Co²⁺ in glass, three absorption bands near 540, 590 and 640 nm,
///   reflecting blue and the deep red (its violet cast);
/// - Prussian blue: a broad reflectance maximum near 450–490 nm, strong
///   broad absorption over 580–720 nm (the intervalence band near 700);
/// - chrome yellow: a sharp absorption edge near 500–520 nm;
/// - yellow ochre (goethite): a gradual edge 480–580 nm, a shoulder near
///   650 nm;
/// - vermilion: a very sharp edge near 590–600 nm;
/// - raw umber: low, rising slowly to the red;
/// - green earth (celadonite/glaucophane): a weak, broad maximum near
///   500–550 nm, Fe²⁺ absorption toward the red;
/// - lead white: flat, dropping a little in the violet.
fn shape_of(name: &str) -> Option<spectral::Spectrum> {
    let c = spectral::curve;
    Some(match name {
        "smalt" | "pale smalt" => c(&[(380.0, 0.4), (470.0, 0.4), (520.0, 0.12), (540.0, 0.07), (565.0, 0.1), (590.0, 0.05), (615.0, 0.08), (640.0, 0.05), (670.0, 0.12), (700.0, 0.25), (750.0, 0.4)]),
        "Prussian blue" => c(&[(380.0, 0.05), (420.0, 0.07), (460.0, 0.09), (490.0, 0.08), (530.0, 0.05), (580.0, 0.025), (640.0, 0.012), (700.0, 0.01), (750.0, 0.02)]),
        "chrome yellow" => c(&[(380.0, 0.06), (470.0, 0.06), (495.0, 0.12), (515.0, 0.35), (535.0, 0.62), (560.0, 0.8), (750.0, 0.85)]),
        "yellow ochre" => c(&[(380.0, 0.06), (450.0, 0.08), (500.0, 0.12), (540.0, 0.25), (580.0, 0.45), (620.0, 0.5), (655.0, 0.46), (700.0, 0.5), (750.0, 0.55)]),
        "vermilion" => c(&[(380.0, 0.05), (560.0, 0.05), (585.0, 0.15), (605.0, 0.5), (630.0, 0.65), (750.0, 0.7)]),
        "raw umber" => c(&[(380.0, 0.04), (500.0, 0.06), (600.0, 0.1), (700.0, 0.14), (750.0, 0.16)]),
        "green earth" => c(&[(380.0, 0.08), (450.0, 0.12), (510.0, 0.17), (545.0, 0.17), (590.0, 0.13), (640.0, 0.1), (700.0, 0.09), (750.0, 0.1)]),
        "lead white" => c(&[(380.0, 0.8), (420.0, 0.88), (460.0, 0.9), (750.0, 0.92)]),
        _ => return None,
    })
}

/// Tube `i` as a spectral paint: shaped (`shape_of`) or on the basis.
fn spec_tube(p: &Palette, i: usize, shaped: bool) -> SpectralPigment {
    let t = &p.tubes[i];
    match shape_of(t.name).filter(|_| shaped) {
        Some(sh) => SpectralPigment::from_masstone_spectrum(&spectral::fit_shape(t.color, &sh), scat(p, i)),
        None => SpectralPigment::masstone(t.color, scat(p, i)),
    }
}

fn spec_mix(p: &Palette, parts: &[(usize, f32)], shaped: bool) -> SpectralPigment {
    SpectralPigment::mix(&parts.iter().map(|&(i, f)| (spec_tube(p, i, shaped), f * p.tubes[i].strength)).collect::<Vec<_>>())
}

/// A glaze or film, both ways.
struct Film {
    rgb: Pigment,
    spec: SpectralPigment,
}

impl Film {
    /// A palette paint thinned with `medium` (as `Mixture::paint` does).
    fn paint(p: &Palette, name: &str, medium: f32) -> Film {
        let i = idx(p, name);
        let s = scat(p, i) * (1.0 - medium);
        Film { rgb: Pigment::masstone(p.tubes[i].color, s), spec: SpectralPigment::masstone(p.tubes[i].color, s) }
    }
    /// A film named by its look over white (`Pigment::with_hiding`).
    fn tint(tint: Rgb, hiding: f32) -> Film {
        Film { rgb: Pigment::with_hiding(tint, hiding), spec: SpectralPigment::with_hiding(tint, hiding) }
    }
    fn varnish(tint: Rgb) -> Film {
        Film { rgb: Pigment::varnish(tint), spec: SpectralPigment::varnish(tint) }
    }
}

fn varnish_rgb() -> Pigment {
    Pigment::varnish(hex("#e6d3a4"))
}

/// Films laid in order over `sub`, each `x` coats: (RGB-KM, spectral).
fn stack(sub: Rgb, films: &[(&Film, f32)]) -> (Rgb, Rgb) {
    let mut r = sub;
    let mut s = spectral::from_rgb(sub);
    for (f, x) in films {
        r = f.rgb.over(r, *x);
        s = f.spec.over_spectrum(&s, *x);
    }
    (r, spectral::to_rgb(&s))
}

fn main() {
    let o = paintings::run::Run::new("study_spectral");
    let p = tubes();
    let mut c = Canvas::new(o.width, 1000.0 / 1380.0, [0.5; 3]);
    let mut cells: Vec<(f32, f32, f32, f32, Rgb)> = Vec::new();
    let mut y = 10.0;
    let (x0, cw) = (10.0, 980.0 / 11.0);

    // ---- A and B: mixing ramps
    eprintln!("tube S per coat: {}", p.tubes.iter().enumerate().map(|(i, t)| format!("{} {:.2}", t.name, scat(&p, i))).collect::<Vec<_>>().join(", "));
    let pairs = [
        ("A", "Prussian blue", "yellow ochre"),
        ("A", "Prussian blue", "chrome yellow"),
        ("A", "smalt", "yellow ochre"),
        ("A", "smalt", "chrome yellow"),
        ("B", "smalt", "lead white"),
        ("B", "Prussian blue", "lead white"),
    ];
    for (sec, a, b) in pairs {
        let (ia, ib) = (idx(&p, a), idx(&p, b));
        eprintln!("\n[{sec}] {a} → {b} (fraction of {b}: 0, .1, … 1)");
        let mut rows: Vec<Vec<Rgb>> = Vec::new();
        for how in HOWS {
            let row: Vec<Rgb> = (0..11).map(|k| {
                let f = k as f32 / 10.0;
                masstone(&p, &[(ia, 1.0 - f), (ib, f)], how)
            }).collect();
            for (k, &col) in row.iter().enumerate() {
                cells.push((x0 + k as f32 * cw, y, cw - 2.0, 15.0, col));
            }
            y += 16.0;
            rows.push(row);
        }
        y += 8.0;
        for (h, row) in HOWS.iter().zip(&rows) {
            let picks = [2usize, 5, 8];
            let txt: Vec<String> = picks.iter().map(|&k| format!("{:.1}: {} {}", k as f32 / 10.0, hexs(row[k]), lch(row[k]))).collect();
            let d: f32 = (1..10).map(|k| de(row[k], rows[0][k])).fold(0.0, f32::max);
            eprintln!("  {:7} {}   max ΔE vs Mixbox {d:.3}", format!("{h:?}"), txt.join(" | "));
        }
        let d: f32 = (1..10).map(|k| de(rows[1][k], rows[2][k])).fold(0.0, f32::max);
        eprintln!("  max ΔE RGB-KM vs spectral KM {d:.3}");
    }

    // ---- C: glazes
    let sky = masstone(&p, &[(idx(&p, "smalt"), 0.45), (idx(&p, "lead white"), 0.55)], How::Mixbox);
    let ochre = p.tubes[idx(&p, "yellow ochre")].color;
    let chrome = p.tubes[idx(&p, "chrome yellow")].color;
    let umber = Film::paint(&p, "raw umber", 0.9);
    let prussian = Film::paint(&p, "Prussian blue", 0.8);
    // a madder lake (not in the palettes): a transparent rose, tinting a
    // white ground a madder pink at one coat
    let madder = Film::tint(hex("#d8768a"), 0.06);
    let madder_over = |c: Rgb| stack(c, &[(&madder, 1.0)]);
    let glazes: [(&str, Rgb, Vec<&Film>); 4] = [
        ("umber over sky blue", sky, vec![&umber]),
        ("madder over yellow ochre", ochre, vec![&madder]),
        ("Prussian blue over chrome yellow", chrome, vec![&prussian]),
        ("umber over madder over sky blue", madder_over(sky).0, vec![&umber]),
    ];
    eprintln!("\n[C] glazes (RGB-KM | spectral | ΔE)");
    y += 6.0;
    let coats = [0.0, 0.5, 1.0, 2.0, 4.0];
    for (name, sub, films) in &glazes {
        let f = films[0];
        // for the stacked row the spectral substrate is itself a spectral stack
        let spec_sub = if name.starts_with("umber over madder") { madder_over(sky).1 } else { *sub };
        let mut line = format!("  {name:34}");
        for (k, &x) in coats.iter().enumerate() {
            let r = if x == 0.0 { *sub } else { f.rgb.over(*sub, x) };
            let s = if x == 0.0 { spec_sub } else { f.spec.over(spec_sub, x) };
            cells.push((x0 + k as f32 * cw * 2.0, y, 2.0 * cw - 2.0, 18.0, r));
            cells.push((x0 + k as f32 * cw * 2.0, y + 19.0, 2.0 * cw - 2.0, 18.0, s));
            line += &format!(" {x}: {}|{} {:.3}", hexs(r), hexs(s), de(r, s));
        }
        eprintln!("{line}");
        y += 44.0;
    }
    // the stacks the glazes' order question asks: A then B vs B then A
    let (a1, b1) = stack(sky, &[(&madder, 1.0), (&umber, 1.0)]);
    let (a2, b2) = stack(sky, &[(&umber, 1.0), (&madder, 1.0)]);
    eprintln!("  sky + madder then umber: {}|{}; umber then madder: {}|{} (order ΔE rgb {:.3} spectral {:.3})", hexs(a1), hexs(b1), hexs(a2), hexs(b2), de(a1, a2), de(b1, b2));

    // ---- D: aged varnish
    let varnish = Film::varnish(hex("#e6d3a4"));
    let subs = [("dark", hex("#1e1b19")), ("dark green", hex("#2c3a26")), ("deep blue", hex("#26324f")), ("mid sky", hex("#8d9bb8")), ("lead white", hex("#efe9dc"))];
    eprintln!("\n[D] aged varnish #e6d3a4 (RGB-KM | spectral, ΔE between; shift = ΔE from the unvarnished)");
    y += 6.0;
    for (name, sub) in subs {
        let mut line = format!("  {name:11}");
        for (k, &x) in [0.0, 0.4, 1.0, 2.0, 4.0].iter().enumerate() {
            let (r, s) = stack(sub, &[(&varnish, x)]);
            cells.push((x0 + k as f32 * cw * 2.0, y, 2.0 * cw - 2.0, 14.0, r));
            cells.push((x0 + k as f32 * cw * 2.0, y + 15.0, 2.0 * cw - 2.0, 14.0, s));
            line += &format!(" {x}: {}|{} {:.3} (shift {:.3}|{:.3})", hexs(r), hexs(s), de(r, s), de(r, sub), de(s, sub));
        }
        eprintln!("{line}");
        y += 34.0;
    }
    // ---- F: pigment-shaped spectra
    eprintln!("\n[F] pigment-shaped spectra: Mixbox | spectral basis | spectral shaped");
    y += 10.0;
    let fpairs = [("Prussian blue", "yellow ochre"), ("Prussian blue", "chrome yellow"), ("smalt", "yellow ochre"), ("smalt", "chrome yellow"), ("smalt", "lead white"), ("Prussian blue", "lead white"), ("smalt", "green earth"), ("vermilion", "lead white")];
    for (a, b) in fpairs {
        let (ia, ib) = (idx(&p, a), idx(&p, b));
        let rows: Vec<Vec<Rgb>> = (0..3)
            .map(|how| {
                (0..11)
                    .map(|k| {
                        let f = k as f32 / 10.0;
                        let parts = [(ia, 1.0 - f), (ib, f)];
                        match how {
                            0 => masstone(&p, &parts, How::Mixbox),
                            1 => spec_mix(&p, &parts, false).masstone_color(),
                            _ => spec_mix(&p, &parts, true).masstone_color(),
                        }
                    })
                    .collect()
            })
            .collect();
        for row in &rows {
            for (k, &col) in row.iter().enumerate() {
                cells.push((x0 + k as f32 * cw, y, cw - 2.0, 11.0, col));
            }
            y += 12.0;
        }
        y += 6.0;
        let d: f32 = (1..10).map(|k| de(rows[1][k], rows[2][k])).fold(0.0, f32::max);
        let dm: f32 = (1..10).map(|k| de(rows[0][k], rows[2][k])).fold(0.0, f32::max);
        let picks: Vec<String> = [2usize, 5, 8].iter().map(|&k| format!("{:.1}: {} {} | {} {}", k as f32 / 10.0, hexs(rows[1][k]), lch(rows[1][k]), hexs(rows[2][k]), lch(rows[2][k]))).collect();
        eprintln!("  {a} → {b}: shaped vs basis max ΔE {d:.3}, shaped vs Mixbox {dm:.3}\n     {}", picks.join("\n     "));
    }
    // glazes and varnish with shaped spectra
    let sh = |n: &str| spectral::fit_shape(p.tubes[idx(&p, n)].color, &shape_of(n).unwrap());
    let ochre_s = sh("yellow ochre");
    let chrome_s = sh("chrome yellow");
    let sky_s = spec_mix(&p, &[(idx(&p, "smalt"), 0.45), (idx(&p, "lead white"), 0.55)], true);
    let sky_b = spec_mix(&p, &[(idx(&p, "smalt"), 0.45), (idx(&p, "lead white"), 0.55)], false);
    let glaze_of = |n: &str, medium: f32, shaped: bool| spec_tube(&p, idx(&p, n), shaped).scaled(1.0 - medium);
    // madder: two alizarin/purpurin bands near 510 and 540 nm, rose over white
    let madder_shape = spectral::curve(&[(380.0, 0.45), (420.0, 0.4), (460.0, 0.3), (495.0, 0.14), (515.0, 0.08), (530.0, 0.1), (545.0, 0.07), (570.0, 0.15), (600.0, 0.55), (640.0, 0.8), (750.0, 0.85)]);
    let madder_w = spectral::fit_shape(hex("#d8768a"), &madder_shape);
    let madder_s = SpectralPigment::from_appearance_spectra(&madder_w, &madder_w.map(|v| v * 0.06));
    // aged varnish: absorption rising steeply below ~500 nm
    let varn_shape = spectral::curve(&[(380.0, 0.3), (420.0, 0.45), (460.0, 0.62), (500.0, 0.78), (540.0, 0.86), (600.0, 0.9), (750.0, 0.92)]);
    let varn_w = spectral::fit_shape(hex("#e6d3a4"), &varn_shape);
    let varn_s = SpectralPigment::from_appearance_spectra(&varn_w, &varn_w.map(|v| v * 0.004));
    let dark_green = spec_mix(&p, &[(idx(&p, "Prussian blue"), 0.3), (idx(&p, "yellow ochre"), 0.5), (idx(&p, "raw umber"), 0.2)], true);
    let dark_green_b = spec_mix(&p, &[(idx(&p, "Prussian blue"), 0.3), (idx(&p, "yellow ochre"), 0.5), (idx(&p, "raw umber"), 0.2)], false);
    type Case<'a> = (&'a str, Rgb, spectral::Spectrum, Rgb, &'a Pigment, SpectralPigment, SpectralPigment);
    let rgb_madder = &madder.rgb;
    let rgb_varn = &varnish_rgb();
    let ums = Pigment::masstone(p.tubes[idx(&p, "raw umber")].color, scat(&p, idx(&p, "raw umber")) * 0.1);
    let prs = Pigment::masstone(p.tubes[idx(&p, "Prussian blue")].color, scat(&p, idx(&p, "Prussian blue")) * 0.2);
    let cases: Vec<Case> = vec![
        ("madder over yellow ochre", ochre, ochre_s, ochre, rgb_madder, madder.spec, madder_s),
        ("Prussian blue glaze over chrome yellow", chrome, chrome_s, chrome, &prs, glaze_of("Prussian blue", 0.8, false), glaze_of("Prussian blue", 0.8, true)),
        ("umber glaze over smalt sky", sky_b.masstone_color(), sky_s.masstone_spectrum(), sky_b.masstone_color(), &ums, glaze_of("raw umber", 0.9, false), glaze_of("raw umber", 0.9, true)),
        ("varnish over smalt sky", sky_b.masstone_color(), sky_s.masstone_spectrum(), sky_b.masstone_color(), rgb_varn, varnish.spec, varn_s),
        ("varnish over a Prussian-ochre dark green", dark_green_b.masstone_color(), dark_green.masstone_spectrum(), dark_green_b.masstone_color(), rgb_varn, varnish.spec, varn_s),
    ];
    for (name, sub_rgb, sub_shaped, sub_basis, rgbf, basisf, shapedf) in cases {
        let mut line = format!("  {name:42}");
        let bsub = spectral::from_rgb(sub_basis);
        for (k, &x) in [0.0f32, 0.5, 1.0, 2.0, 4.0].iter().enumerate() {
            let r = if x == 0.0 { sub_rgb } else { rgbf.over(sub_rgb, x) };
            let b = spectral::to_rgb(&basisf.over_spectrum(&bsub, x));
            let s2 = spectral::to_rgb(&shapedf.over_spectrum(&sub_shaped, x));
            for (j, col) in [r, b, s2].into_iter().enumerate() {
                cells.push((x0 + k as f32 * cw * 2.0, y + j as f32 * 12.0, 2.0 * cw - 2.0, 11.0, col));
            }
            line += &format!(" {x}: {}|{}|{} Δ{:.3}", hexs(r), hexs(b), hexs(s2), de(b, s2));
        }
        eprintln!("{line}");
        y += 42.0;
    }
    eprintln!("sheet height used: {y:.0} of {:.0}", c.height());

    // ---- E: cost per call
    let pw = [(idx(&p, "Prussian blue"), 0.3), (idx(&p, "yellow ochre"), 0.6), (idx(&p, "lead white"), 0.1)];
    let n = 200_000;
    let time = |label: &str, f: &dyn Fn(u32) -> Rgb| {
        let t = Instant::now();
        let mut acc = 0.0;
        for k in 0..n {
            acc += black_box(f(k))[1];
        }
        let ns = t.elapsed().as_nanos() as f64 / n as f64;
        eprintln!("  {label:44} {ns:8.0} ns/call  ({acc:.1})");
        ns
    };
    eprintln!("\n[E] cost");
    let jit = |k: u32| (k % 7) as f32 * 1e-3;
    let lat_mix = time("mix 3 tubes, Mixbox (palette eval)", &|k| masstone(&p, &[(pw[0].0, 0.3 + jit(k)), (pw[1].0, 0.6), (pw[2].0, 0.1 - jit(k))], How::Mixbox));
    time("mix 3 tubes, KM per RGB channel", &|k| masstone(&p, &[(pw[0].0, 0.3 + jit(k)), (pw[1].0, 0.6), (pw[2].0, 0.1 - jit(k))], How::RgbKm));
    let spec_mix = time("mix 3 tubes, spectral KM", &|k| masstone(&p, &[(pw[0].0, 0.3 + jit(k)), (pw[1].0, 0.6), (pw[2].0, 0.1 - jit(k))], How::SpecKm));
    time("mix 3 tubes, spectral.js mix", &|k| masstone(&p, &[(pw[0].0, 0.3 + jit(k)), (pw[1].0, 0.6), (pw[2].0, 0.1 - jit(k))], How::SpecJs));
    let pr = Pigment::masstone(ochre, 2.0);
    let ps = SpectralPigment::masstone(ochre, 2.0);
    let rgb_over = time("layer over RGB substrate, KM per channel", &|k| pr.over([0.2, 0.3, 0.4 + jit(k)], 0.7));
    let spec_over = time("layer over RGB substrate, spectral (RGB in/out)", &|k| ps.over([0.2, 0.3, 0.4 + jit(k)], 0.7));
    let sub = spectral::from_rgb([0.2, 0.3, 0.4]);
    time("layer over a spectrum, spectral (no conversion)", &|k| {
        let r = ps.over_spectrum(&sub, 0.7 + jit(k));
        [r[0], r[15], r[30]]
    });
    time("SpectralPigment::masstone (build from RGB)", &|k| SpectralPigment::masstone([0.2, 0.3, 0.4 + jit(k)], 1.0).k[0..3].try_into().unwrap());
    eprintln!("  ratio: mixing {:.1}×, layering {:.1}×", spec_mix / lat_mix, spec_over / rgb_over);

    // ---- the sheet
    let cells2 = cells.clone();
    c.apply(move |x, y, _| {
        for &(cx, cy, w, h, col) in &cells2 {
            if x >= cx && x < cx + w && y >= cy && y < cy + h {
                return col;
            }
        }
        [0.5; 3]
    });
    o.save(&mut c);
}
