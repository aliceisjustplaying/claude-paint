//! Wet control: identical strokes across drying stage, load and pressure,
//! measured (notes/wet.md §8). Built unchanged on each engine compared.
//!
//! Rows (gestures), top to bottom:
//! A. a loaded stiff light accent (a filbert touch and a short stroke) into
//!    a dark;
//! B. a hog flat loaded with the light dragged along a light/dark contour
//!    (softening it);
//! C. a flat's light stroke ending in the middle of a dark (its end);
//! D. a thin glaze-hand veil (medium 0.85, filbert 6) over a thin sky.
//!
//! Columns: stage (open, setting, tacky, dry) × load (full 0.9, lean 0.3)
//! × pressure (light 0.45, firm 0.9), in that nesting.
//!
//! Printed per cell: retained light (A, C: how clean the light is, 1 = its
//! own paint over dry dark), exposed ground (% of the track within ΔE 0.06
//! of the ground), edge width 10–90% in mm (B: across the contour; C: along
//! the stroke past its end).
//!
//! `cargo paint study_wet_control` (1000 px); `--width 3200`.

use paint::color::to_oklab;
use paint::{Canvas, Gesture, Held, Mask, Orient, Paint, Rgb, Stage, Style, Tool, Touch, hex};

const CW: f32 = 1000.0 / 16.0;
const RH: f32 = 70.0;
const STAGES: [&str; 4] = ["open", "setting", "tacky", "dry"];

fn cell(c: usize, r: usize) -> (f32, f32) {
    (c as f32 * CW, r as f32 * RH)
}
fn load_of(c: usize) -> f32 {
    if (c / 2) % 2 == 0 { 0.9 } else { 0.3 }
}
fn press_of(c: usize) -> f32 {
    if c % 2 == 0 { 0.45 } else { 0.9 }
}

fn rect(c: &Canvas, x0: f32, y0: f32, x1: f32, y1: f32) -> Mask {
    Mask::from_fn(c.frame(), move |x, y| if x > x0 && x < x1 && y > y0 && y < y1 { 1.0 } else { 0.0 })
}

const DARK: &str = "#26301f";
const LIGHT: &str = "#c9c79a";
const SKY: &str = "#b9c3cb";
const VEIL: &str = "#8d8a86";

fn field(c: &mut Canvas, st: &Style, col: &str, m: &Mask, medium: f32, seed: u64) {
    let k = hex(col);
    c.work(m, &st.body().color(move |_, _| k).medium(medium).angle(|_, _| 0.1).clip(true), seed);
}

/// The underlayer of row `r` in column `c`.
fn under(cv: &mut Canvas, st: &Style, c: usize, r: usize) {
    let (x0, y0) = cell(c, r);
    let seed = (c * 4 + r) as u64;
    match r {
        0 | 2 => {
            let m = rect(cv, x0 + 4.0, y0 + 6.0, x0 + CW - 4.0, y0 + RH - 6.0);
            field(cv, st, DARK, &m, 0.2, seed);
        }
        1 => {
            let m = rect(cv, x0 + 4.0, y0 + 6.0, x0 + CW - 4.0, y0 + 35.0);
            field(cv, st, LIGHT, &m, 0.2, seed);
            let m = rect(cv, x0 + 4.0, y0 + 35.0, x0 + CW - 4.0, y0 + RH - 6.0);
            field(cv, st, DARK, &m, 0.2, seed + 100);
        }
        _ => {
            let m = rect(cv, x0 + 4.0, y0 + 6.0, x0 + CW - 4.0, y0 + RH - 6.0);
            let k = hex(SKY);
            cv.work(&m, &st.broad().color(move |_, _| k).medium(0.3).clip(true), seed);
        }
    }
}

fn light() -> Paint {
    Paint::body(hex(LIGHT))
}

/// The gesture of row `r` in column `c`.
fn gesture(cv: &mut Canvas, st: &Style, c: usize, r: usize) {
    let (x0, y0) = cell(c, r);
    let (ld, p) = (load_of(c), press_of(c));
    let seed = 1000 + (c * 4 + r) as u64;
    match r {
        0 => {
            let mut h = Held::new(Tool::filbert(12.0), seed);
            h.reload(light(), ld);
            cv.touch(&mut h, &Touch::at(x0 + 20.0, y0 + 24.0).pressure(p).drag(1.5, 0.5).angle(0.4), None);
            let mut h = Held::new(Tool::filbert(6.0), seed + 1);
            h.reload(light(), ld);
            cv.drag(&mut h, &Gesture::new(vec![(x0 + 13.0, y0 + 48.0), (x0 + 30.0, y0 + 46.0), (x0 + 47.0, y0 + 47.0)]).pressure(p, p * 0.8).orient(Orient::Across), None);
        }
        1 => {
            let mut h = Held::new(Tool::hog_flat(8.0), seed);
            h.reload(light(), ld);
            cv.drag(&mut h, &Gesture::new(vec![(x0 + 8.0, y0 + 35.0), (x0 + 31.0, y0 + 35.5), (x0 + CW - 8.0, y0 + 35.0)]).pressure(p, p).orient(Orient::Across), None);
        }
        2 => {
            let mut h = Held::new(Tool::hog_flat(10.0), seed);
            h.reload(light(), ld);
            cv.drag(&mut h, &Gesture::new(vec![(x0 + 8.0, y0 + 35.0), (x0 + 26.0, y0 + 35.0), (x0 + 42.0, y0 + 35.0)]).pressure(p, p).ramps(0.1, 0.35).orient(Orient::Across), None);
        }
        _ => {
            let k = hex(VEIL);
            let mut hd = st.glaze(0.85).color(move |_, _| k).load(ld).pressure(p, p).length(12.0, 30.0);
            hd.tool = Tool::filbert(6.0);
            let m = rect(cv, x0 + 12.0, y0 + 18.0, x0 + 50.0, y0 + 52.0);
            cv.work(&m, &hd, seed);
        }
    }
}

struct Img {
    lab: Vec<Rgb>,
    w: usize,
    s: f32,
}
impl Img {
    fn at(&self, x: f32, y: f32, r: f32) -> Rgb {
        let (cx, cy, rp) = (x * self.s, y * self.s, (r * self.s).max(0.6));
        let (mut a, mut n) = ([0.0f32; 3], 0.0);
        for j in (cy - rp).floor() as i64..=(cy + rp).ceil() as i64 {
            for i in (cx - rp).floor() as i64..=(cx + rp).ceil() as i64 {
                let (dx, dy) = (i as f32 + 0.5 - cx, j as f32 + 0.5 - cy);
                if dx * dx + dy * dy <= rp * rp {
                    let p = self.lab[j as usize * self.w + i as usize];
                    for k in 0..3 {
                        a[k] += p[k];
                    }
                    n += 1.0;
                }
            }
        }
        [a[0] / n, a[1] / n, a[2] / n]
    }
    fn l(&self, x: f32, y: f32, r: f32) -> f32 {
        self.at(x, y, r)[0]
    }
    /// Lightness along a line from a to b (units), one sample per pixel,
    /// averaged across ±band units.
    fn line(&self, a: (f32, f32), b: (f32, f32), band: f32) -> Vec<f32> {
        let len = ((b.0 - a.0).hypot(b.1 - a.1) * self.s) as usize;
        let (dx, dy) = ((b.0 - a.0) / len as f32, (b.1 - a.1) / len as f32);
        let (nx, ny) = (-dy, dx);
        let nb = ((band * self.s) as i32).max(0);
        (0..len)
            .map(|k| {
                let (x, y) = (a.0 + dx * k as f32, a.1 + dy * k as f32);
                let mut acc = 0.0;
                for j in -nb..=nb {
                    let t = j as f32 / self.s;
                    let (px, py) = ((x + nx * t * len as f32 / len as f32) * self.s, (y + ny * t) * self.s);
                    acc += self.lab[py as usize * self.w + px as usize][0];
                }
                acc / (2 * nb + 1) as f32
            })
            .collect()
    }
    fn ground(&self, g: Rgb, b: (f32, f32, f32, f32)) -> f32 {
        let (mut n, mut k) = (0.0f32, 0.0f32);
        for y in (b.1 * self.s) as usize..(b.3 * self.s) as usize {
            for x in (b.0 * self.s) as usize..(b.2 * self.s) as usize {
                let p = self.lab[y * self.w + x];
                n += 1.0;
                if (p[0] - g[0]).hypot(p[1] - g[1]).hypot(p[2] - g[2]) < 0.06 {
                    k += 1.0;
                }
            }
        }
        k / n.max(1.0)
    }
}

/// 10–90% width (units) of a transition from `hi` (start) to `lo` (end),
/// from the 50% crossing nearest the middle outward.
fn width(p: &[f32], hi: f32, lo: f32, s: f32) -> f32 {
    let n: Vec<f32> = p.iter().map(|&v| (v - lo) / (hi - lo)).collect();
    let mid = n.len() as f32 / 2.0;
    let Some(c) = (1..n.len()).filter(|&i| (n[i - 1] - 0.5) * (n[i] - 0.5) <= 0.0).min_by(|&a, &b| (a as f32 - mid).abs().total_cmp(&(b as f32 - mid).abs())) else { return f32::NAN };
    let a = (0..c).rev().find(|&i| n[i] >= 0.9).unwrap_or(0);
    let b = (c..n.len()).find(|&i| n[i] <= 0.1).unwrap_or(n.len() - 1);
    (b - a) as f32 / s
}

fn median(mut v: Vec<f32>) -> f32 {
    v.retain(|x| x.is_finite());
    if v.is_empty() {
        return f32::NAN;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    v[v.len() / 2]
}

fn main() {
    let o = paintings::run::Run::new("study_wet_control");
    let st = Style::friedrich();
    let mut cv = st.prepare(o.width, 1000.0 / (4.0 * RH), o.seed);
    let g = to_oklab(cv.pixels()[0]);
    let mm = 440.0 / 1000.0;
    let cols = |stage: usize| (0..4).map(move |k| stage * 4 + k);
    // dry
    for c in cols(3) {
        for r in 0..4 {
            under(&mut cv, &st, c, r);
        }
    }
    cv.dry();
    // tacky
    for c in cols(2) {
        for r in 0..4 {
            under(&mut cv, &st, c, r);
        }
    }
    let probe = |c: usize, r: usize| {
        let (x0, y0) = cell(c, r);
        (x0 + CW - 10.0, y0 + if r == 1 { 55.0 } else { 12.0 })
    };
    let t0 = cv.clock();
    while (0..4).any(|r| {
        let p = probe(8, r);
        cv.drying_at(p.0, p.1) != Stage::Tacky
    }) && cv.clock() - t0 < 5.0 * 24.0 * 60.0
    {
        cv.wait(15.0);
    }
    // setting: each row's cells as soon as its film is setting
    for c in cols(1) {
        for r in 0..4 {
            under(&mut cv, &st, c, r);
        }
    }
    let mut done = [false; 4];
    let t1 = cv.clock();
    while done.iter().any(|d| !d) && cv.clock() - t1 < 3.0 * 24.0 * 60.0 {
        cv.wait(5.0);
        for r in 0..4 {
            let p = probe(4, r);
            if !done[r] && cv.drying_at(p.0, p.1) != Stage::Open {
                for c in cols(1) {
                    gesture(&mut cv, &st, c, r);
                }
                done[r] = true;
            }
        }
    }
    let stage_at = |cv: &Canvas, c: usize| (0..4).map(|r| format!("{:?}", { let p = probe(c, r); cv.drying_at(p.0, p.1) })).collect::<Vec<_>>().join("/");
    println!("stages before their gestures: tacky {}, dry {}", stage_at(&cv, 8), stage_at(&cv, 12));
    for c in cols(2).chain(cols(3)) {
        for r in 0..4 {
            gesture(&mut cv, &st, c, r);
        }
    }
    for c in cols(0) {
        for r in 0..4 {
            under(&mut cv, &st, c, r);
        }
    }
    for c in cols(0) {
        for r in 0..4 {
            gesture(&mut cv, &st, c, r);
        }
    }
    cv.dry();
    let img = Img { lab: cv.pixels().iter().map(|&p| to_oklab(p)).collect(), w: cv.window().w, s: cv.window().w as f32 / 1000.0 };
    let lp = to_oklab(light().color)[0];
    // per cell
    let clean = |c: usize, r: usize| {
        let (x0, y0) = cell(c, r);
        let d = img.l(x0 + CW - 10.0, y0 + 14.0, 2.5);
        let v = match r {
            0 => [img.l(x0 + 20.5, y0 + 24.2, 1.2), img.l(x0 + 30.0, y0 + 46.5, 1.2)].iter().sum::<f32>() / 2.0,
            _ => img.l(x0 + 22.0, y0 + 35.0, 1.5),
        };
        (v - d) / (lp - d)
    };
    let ground = |c: usize, r: usize| {
        let (x0, y0) = cell(c, r);
        100.0 * match r {
            0 => (img.ground(g, (x0 + 15.0, y0 + 20.0, x0 + 26.0, y0 + 29.0)) + img.ground(g, (x0 + 14.0, y0 + 43.0, x0 + 46.0, y0 + 51.0))) / 2.0,
            1 => img.ground(g, (x0 + 10.0, y0 + 30.0, x0 + CW - 10.0, y0 + 40.0)),
            2 => img.ground(g, (x0 + 10.0, y0 + 31.0, x0 + 44.0, y0 + 39.0)),
            _ => img.ground(g, (x0 + 14.0, y0 + 20.0, x0 + 48.0, y0 + 50.0)),
        }
    };
    let edge = |c: usize, r: usize| {
        let (x0, y0) = cell(c, r);
        match r {
            1 => {
                let hi = img.l(x0 + 31.0, y0 + 14.0, 3.0);
                let lo = img.l(x0 + 31.0, y0 + 58.0, 3.0);
                median((0..9).map(|k| {
                    let x = x0 + 16.0 + 4.0 * k as f32;
                    width(&img.line((x, y0 + 20.0), (x, y0 + 52.0), 0.8), hi, lo, img.s)
                }).collect()) * mm
            }
            2 => {
                let hi = img.l(x0 + 22.0, y0 + 35.0, 1.5);
                let lo = img.l(x0 + CW - 8.0, y0 + 35.0, 1.5);
                width(&img.line((x0 + 20.0, y0 + 35.0), (x0 + CW - 6.0, y0 + 35.0), 1.5), hi, lo, img.s) * mm
            }
            _ => f32::NAN,
        }
    };
    let head = (0..16).map(|c| format!("{}{}{}", &STAGES[c / 4][..1], if load_of(c) > 0.5 { "F" } else { "L" }, if press_of(c) < 0.6 { "l" } else { "p" })).collect::<Vec<_>>().join(" ");
    println!("cells: stage (o/s/t/d) · load (F full, L lean) · pressure (l light, p firm)");
    println!("{:<34} {head}", "");
    let row = |name: &str, f: &dyn Fn(usize) -> f32| println!("{name:<34} {}", (0..16).map(|c| { let v = f(c); if v.is_nan() { "  - ".to_string() } else { format!("{v:4.2}") } }).collect::<Vec<_>>().join(" "));
    let rowp = |name: &str, f: &dyn Fn(usize) -> f32| println!("{name:<34} {}", (0..16).map(|c| format!("{:4.0}", f(c))).collect::<Vec<_>>().join(" "));
    row("A accent: retained light", &|c| clean(c, 0));
    rowp("A accent: ground %", &|c| ground(c, 0));
    row("B contour: edge 10-90% mm", &|c| edge(c, 1));
    rowp("B contour: ground %", &|c| ground(c, 1));
    row("C flat end: end 10-90% mm", &|c| edge(c, 2));
    row("C flat end: retained light", &|c| clean(c, 2));
    rowp("C flat end: ground %", &|c| ground(c, 2));
    rowp("D glaze veil: ground %", &|c| ground(c, 3));
    o.save(&mut cv);
}
