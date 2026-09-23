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
    let mut c = Canvas::new(o.width, 1000.0 / 1300.0, [0.5; 3]);
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
