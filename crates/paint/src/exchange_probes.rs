//! Probes for the brush/canvas exchange experiment (AGENT_BRIEF_V2 §4c;
//! notes/engine3/HANDOVER-3.md, Lane A). All `#[ignore]`: they print tables
//! and, with `EXCH_DIR`, save the canvases as PNG for the sheets
//! (`notes/thinner/wash-experiment/exchange_sheet.py`).
//!
//! Settings (`EXCH_SETS`, `;`-separated): `today` (the committed per-stroke
//! ceiling), `b` and `B` (the reconstructed look-experiment controls,
//! `bristle::probe::Control`), `k,j` (the exchange experiment,
//! `thinner::set_exchange_kj`), `fk,j` (the same, keeping the pickup
//! rule of unthinned paint, `thinner::set_exchange_floor`).
//!
//! Paint is measured as everything above the primed ground: the open film
//! (`wet.vol`) plus what drying moved into the dry film (`film` minus its
//! value after priming), in µm. Solvent is the open film's (µm).
//!
//! `cargo test --release -p paint --lib exchange_probes -- --ignored --nocapture --test-threads 1`

use crate::bristle::probe::{self, Cont, Control};
use crate::bristle::{Gesture, Held, Tool};
use crate::canvas::Canvas;
use crate::color::hex;
use crate::palette::Palette;
use crate::surface::{COAT_UM, Linen};
use crate::wet::Paint;

#[derive(Clone, Copy, Debug)]
enum Set {
    Today,
    HeldCap,
    AbsCap,
    Exch(f32, f32),
    /// the exchange keeping the pickup rule of unthinned paint
    /// (`thinner::set_exchange_floor`)
    ExchF(f32, f32),
}

impl Set {
    fn parse(t: &str) -> Set {
        match t.trim() {
            "today" => Set::Today,
            "b" => Set::HeldCap,
            "B" => Set::AbsCap,
            t => {
                let (f, t) = t.strip_prefix('f').map_or((false, t), |r| (true, r));
                let (k, j) = t.split_once(',').expect("a setting is today, b, B, k,j or fk,j");
                let (k, j) = (k.parse().unwrap(), j.parse().unwrap());
                if f { Set::ExchF(k, j) } else { Set::Exch(k, j) }
            }
        }
    }
    fn name(&self) -> String {
        match self {
            Set::Today => "today".into(),
            Set::HeldCap => "b".into(),
            Set::AbsCap => "B".into(),
            Set::Exch(k, j) => format!("k{k}j{j}"),
            Set::ExchF(k, j) => format!("fk{k}j{j}"),
        }
    }
    fn apply(&self) {
        crate::thinner::set_exchange(matches!(self, Set::Exch(..) | Set::ExchF(..)));
        crate::thinner::set_exchange_floor(matches!(self, Set::ExchF(..)));
        let (k, j) = match self {
            Set::Exch(k, j) | Set::ExchF(k, j) => (*k, *j),
            _ => (0.0, 0.0),
        };
        crate::thinner::set_exchange_kj(k, j);
        probe::set_control(match self {
            Set::HeldCap => Control::HeldCap,
            Set::AbsCap => Control::AbsCap,
            _ => Control::Today,
        });
    }
}

fn reset() {
    Set::Today.apply();
    probe::set_fix_fill(None, None);
    probe::take_tally();
}

fn sets(default: &str) -> Vec<Set> {
    std::env::var("EXCH_SETS").unwrap_or_else(|_| default.into()).split(';').map(Set::parse).collect()
}

fn grounds() -> Vec<bool> {
    match std::env::var("EXCH_GROUNDS").as_deref() {
        Ok("linen") => vec![true],
        Ok("smooth") => vec![false],
        _ => vec![true, false],
    }
}

fn umber() -> Paint {
    let pal = Palette::named_box("inness").unwrap();
    pal.pile(vec![(pal.tubes.iter().position(|t| t.name == "raw umber").unwrap(), 1.0)]).laid(0.0).with_thinner(0.5)
}

/// The primed canvas of the old `exchange_scenes` (1200 px wide, or
/// `EXCH_PX`), and its dry film then.
fn base(linen: bool) -> (Canvas, Vec<f32>) {
    let px = std::env::var("EXCH_PX").map_or(1200, |v| v.parse().unwrap());
    let mut c = Canvas::new(px, 0.7, hex("#d8cdb8")).with_engine(3);
    if linen {
        c = c.with_linen(Linen::fine(3));
    }
    c.prime(hex("#e4dcc8"), 0.9, 60.0, 0.6, if linen { 0.2 } else { 0.0 }, 7);
    let f0 = c.film.clone();
    (c, f0)
}

fn brush(seed: u64) -> Held {
    Held::new(Tool::hog_flat(40.0), seed)
}

fn g(pts: Vec<(f32, f32)>) -> Gesture {
    Gesture::new(pts).pressure(0.85, 0.85)
}

/// A mechanically exact gesture (no ramps, no shake): the same motion can
/// be cut into calls.
fn exact(pts: Vec<(f32, f32)>) -> Gesture {
    let mut g = g(pts);
    (g.attack, g.release, g.shake) = (0.0, 0.0, 0.0);
    g
}

/// Paint (µm) above the primed ground at buffer pixel `i`.
fn dep(c: &Canvas, f0: &[f32], i: usize) -> f32 {
    (c.wet.vol[i] + c.film[i] - f0[i]) * COAT_UM
}

#[derive(Clone, Copy, Default)]
struct Stat {
    /// paint above the ground, mean and relative spread
    dep: f64,
    spread: f64,
    /// of it, in the open film
    wet: f64,
    solvent: f64,
}

type Rect = (f32, f32, f32, f32);

fn stats(c: &Canvas, f0: &[f32], r: Rect) -> Stat {
    let f = c.f;
    let (mut n, mut s, mut s2, mut sw, mut ss) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for i in 0..f.w * f.h {
        let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
        if x >= r.0 && x < r.2 && y >= r.1 && y < r.3 {
            let p = dep(c, f0, i) as f64;
            n += 1.0;
            s += p;
            s2 += p * p;
            sw += (c.wet.vol[i] * COAT_UM) as f64;
            ss += c.wet.solv.get(i).copied().unwrap_or(0.0) as f64;
        }
    }
    let m = s / n;
    Stat { dep: m, spread: (s2 / n - m * m).max(0.0).sqrt() / m.max(1e-9), wet: sw / n, solvent: ss / n }
}

/// Paint above the ground and open solvent on the whole canvas, in the
/// units of `Held::carried` (coats × square units).
fn totals(c: &Canvas, f0: &[f32]) -> (f64, f64) {
    let a = 1.0 / (c.f.scale as f64 * c.f.scale as f64);
    let p: f64 = (0..c.wet.vol.len()).map(|i| (c.wet.vol[i] + c.film[i] - f0[i]) as f64).sum();
    let s: f64 = c.wet.solv.iter().map(|&v| (v / COAT_UM) as f64).sum();
    (p * a, s * a)
}

/// What went into the brushes and what was wiped off them (paint, solvent).
#[derive(Default)]
struct Ledger {
    loaded: (f64, f64),
    wiped: (f64, f64),
    brushes: Vec<Held>,
}

impl Ledger {
    fn load(&mut self, h: &mut Held, paint: Paint, amount: f32) {
        let a = h.carried();
        h.load(paint, amount);
        let b = h.carried();
        self.loaded = (self.loaded.0 + b.0 - a.0, self.loaded.1 + b.1 - a.1);
    }
    fn reload(&mut self, h: &mut Held, paint: Paint, amount: f32) {
        let a = h.carried();
        h.wipe(0.85);
        let b = h.carried();
        self.wiped = (self.wiped.0 + a.0 - b.0, self.wiped.1 + a.1 - b.1);
        self.load(h, paint, amount);
    }
    fn fresh(&mut self, seed: u64, paint: Paint, amount: f32) -> Held {
        let mut h = brush(seed);
        self.load(&mut h, paint, amount);
        h
    }
    /// Brushes still in play (their contents count as carried).
    fn keep(&mut self, h: Held) {
        self.brushes.push(h);
    }
    /// The line: brush inventory (share of everything loaded, paint and
    /// solvent) and the balance error (canvas + brushes + wiped − loaded,
    /// relative to loaded), paint and solvent, before any wait.
    fn line(&self, c: &Canvas, f0: &[f32], extra: &[&Held]) -> String {
        let (cp, cs) = totals(c, f0);
        let (bp, bs) = self.brushes.iter().chain(extra.iter().copied()).fold((0.0, 0.0), |a, h| {
            let (p, s) = h.carried();
            (a.0 + p, a.1 + s)
        });
        let ep = (cp + bp + self.wiped.0 - self.loaded.0) / self.loaded.0;
        let es = (cs + bs + self.wiped.1 - self.loaded.1) / self.loaded.1.max(1e-12);
        format!("brush p {:3.0}% s {:3.0}% | bal p {:+.1e} s {:+.1e}", 100.0 * bp / self.loaded.0, 100.0 * bs / self.loaded.1.max(1e-12), ep, es)
    }
}

struct Out {
    dir: Option<std::path::PathBuf>,
}

impl Out {
    fn new() -> Self {
        let dir = std::env::var("EXCH_DIR").ok().map(std::path::PathBuf::from);
        if let Some(d) = &dir {
            std::fs::create_dir_all(d).unwrap();
        }
        Out { dir }
    }
    fn save(&self, c: &Canvas, name: &str) {
        if let Some(d) = &self.dir {
            c.clone().save(d.join(format!("{name}.png"))).unwrap();
        }
    }
}

const WAIT_MIN: f32 = 5.0;
const MID: Rect = (350.0, 280.0, 650.0, 420.0);
const BAND: Rect = (350.0, 340.0, 650.0, 362.0);

fn stat_text(s: Stat) -> String {
    format!("paint {:6.2} µm (spread {:.2}, open {:6.2}) solvent {:5.2}", s.dep, s.spread, s.wet, s.solvent)
}

/// One scene's report: paint and solvent now and after `WAIT_MIN` minutes
/// (a matched wait from the end of the scene), brush inventory, balance.
fn report(tag: &str, scene: &str, c: &Canvas, f0: &[f32], r: Rect, led: &Ledger, extra: &[&Held], out: &Out) {
    let a = stats(c, f0, r);
    let mut d = c.clone();
    d.wait(WAIT_MIN);
    let b = stats(&d, f0, r);
    println!("{tag:>14} {scene:<13} {} | {} | +{WAIT_MIN} min paint {:6.2} (spread {:.2}) solvent {:5.2}", stat_text(a), led.line(c, f0, extra), b.dep, b.spread, b.solvent);
    out.save(c, &format!("{tag}-{scene}"));
}

/// The scenes of AGENT_BRIEF_V2 §4c for each setting and ground.
/// `EXCH_SETS="today;b;B;2,0;..." EXCH_GROUNDS=linen|smooth EXCH_DIR=...`
#[test]
#[ignore]
fn exchange_scenes() {
    let umber = umber();
    let out = Out::new();
    for linen in grounds() {
        for set in sets("today;b;B;0,0") {
            set.apply();
            let tag = format!("{}-{}", if linen { "linen" } else { "smooth" }, set.name());
            // one stroke, one fresh load
            {
                let (mut c, f0) = base(linen);
                let mut led = Ledger::default();
                let mut h = led.fresh(5, umber, 0.9);
                c.drag(&mut h, &g(vec![(250.0, 350.0), (750.0, 353.0)]), None);
                report(&tag, "stroke", &c, &f0, BAND, &led, &[&h], &out);
            }
            // a broad pass (8 overlapping strokes, each freshly loaded),
            // three wet, three dried between
            for (name, n, dry) in [("broad", 1, false), ("3wet", 3, false), ("3dry", 3, true)] {
                let (mut c, f0) = base(linen);
                let mut led = Ledger::default();
                for p in 0..n {
                    if p > 0 && dry {
                        c.dry();
                    }
                    for k in 0..8 {
                        let y = 250.0 + 25.0 * k as f32;
                        let mut h = led.fresh(100 * p as u64 + 1 + k, umber, 0.9);
                        c.drag(&mut h, &g(vec![(250.0, y), (750.0, y + 3.0)]), None);
                        led.keep(h);
                    }
                }
                // (after drying the open solvent is gone; the balance counts
                // only what was never baked or evaporated in the open film)
                report(&tag, name, &c, &f0, MID, &led, &[], &out);
            }
            // brush depletion: one load, six strokes side by side
            {
                let (mut c, f0) = base(linen);
                let mut led = Ledger::default();
                let mut h = led.fresh(9, umber, 0.9);
                let p0 = h.carried().0;
                let mut row = Vec::new();
                for k in 0..6 {
                    let y = 240.0 + 45.0 * k as f32;
                    c.drag(&mut h, &g(vec![(250.0, y), (750.0, y + 3.0)]), None);
                    let s = stats(&c, &f0, (350.0, y - 10.0, 650.0, y + 12.0));
                    row.push(format!("{:.2}µm/{:.0}%", s.dep, 100.0 * h.carried().0 / p0));
                }
                println!("{tag:>14} {:<13} per stroke paint/brush left: {} | {}", "deplete", row.join(" "), led.line(&c, &f0, &[&h]));
                out.save(&c, &format!("{tag}-deplete"));
            }
            // the actions over one band, out 250 → 750 and back, exact
            // gestures: uninterrupted (one call); partitioned at the turn
            // without a lift (test-only continuation); a real lift without
            // reload (two calls); a fresh reload onto the wet film; a fresh
            // reload after drying
            let out_leg = || exact(vec![(250.0, 350.0), (750.0, 350.0)]);
            let back_leg = || exact(vec![(750.0, 350.0), (250.0, 350.0)]);
            for action in ["scrub", "cont", "lift", "reload", "afterdry"] {
                let (mut c, f0) = base(linen);
                let mut led = Ledger::default();
                let mut h = led.fresh(13, umber, 0.9);
                match action {
                    "scrub" => c.drag(&mut h, &exact(vec![(250.0, 350.0), (750.0, 350.0), (250.0, 350.0)]), None),
                    "cont" => {
                        let (id, next) = probe::drag(&mut c, &mut h, &out_leg(), None, Some(Cont { d0: 0.0, phase: 0.0, open_end: true, keep: false }));
                        probe::drag(&mut c, &mut h, &back_leg(), Some(id), next);
                    }
                    "lift" => {
                        c.drag(&mut h, &out_leg(), None);
                        c.drag(&mut h, &back_leg(), None);
                    }
                    "reload" => {
                        c.drag(&mut h, &out_leg(), None);
                        led.reload(&mut h, umber, 0.9);
                        c.drag(&mut h, &back_leg(), None);
                    }
                    _ => {
                        c.drag(&mut h, &out_leg(), None);
                        c.dry();
                        led.reload(&mut h, umber, 0.9);
                        c.drag(&mut h, &back_leg(), None);
                    }
                }
                report(&tag, action, &c, &f0, BAND, &led, &[&h], &out);
            }
        }
    }
    reset();
}

/// Path consistency (HANDOVER-3 Lane A (c)). The same exact line 250 → 750
/// at y 350 painted as: 2 points in one call (the reference); 51 collinear
/// points; the reference nudged 0.001 units down (the floor set by
/// rounding, a geometric change far below a pixel); cut at 500 and
/// continued without a lift, same stroke id; the same under a new id; the
/// same with the hairs set down afresh at the cut; the same with the step
/// grid restarted at the cut; and two ordinary calls (a real lift: new id,
/// hairs set down afresh, new grid, new wander). Prints the largest pixel
/// difference against the reference (paint µm) upstream of the cut (x <
/// 450), near it (450-550) and downstream (> 550), its 99th percentile
/// over the painted pixels, the mean |difference| in
/// the band, the paint on the canvas and the brush's paint, relative.
#[test]
#[ignore]
fn exchange_paths() {
    let umber = umber();
    let line = |c: &mut Canvas, h: &mut Held, how: &str| {
        let a = (250.0, 350.0);
        let m = (500.0, 350.0);
        let b = (750.0, 350.0);
        let first = Cont { d0: 0.0, phase: 0.0, open_end: true, keep: false };
        match how {
            "2 points" => c.drag(h, &exact(vec![a, b]), None),
            "51 points" => c.drag(h, &exact((0..=50).map(|k| (250.0 + 10.0 * k as f32, 350.0)).collect()), None),
            "nudged" => c.drag(h, &exact(vec![(250.0, 350.001), (750.0, 350.001)]), None),
            "cont same id" => {
                let (id, next) = probe::drag(c, h, &exact(vec![a, m]), None, Some(first));
                probe::drag(c, h, &exact(vec![m, b]), Some(id), next);
            }
            "cont new id" => {
                let (_, next) = probe::drag(c, h, &exact(vec![a, m]), None, Some(first));
                probe::drag(c, h, &exact(vec![m, b]), None, next);
            }
            "set down" => {
                let (id, next) = probe::drag(c, h, &exact(vec![a, m]), None, Some(first));
                probe::drag(c, h, &exact(vec![m, b]), Some(id), next.map(|n| Cont { keep: false, ..n }));
            }
            "new grid" => {
                // the first call ends with its own step at the cut, the
                // second starts with one there (hairs and wander go on)
                let (id, next) = probe::drag(c, h, &exact(vec![a, m]), None, Some(Cont { open_end: false, ..first }));
                let d0 = next.map_or(0.0, |n| n.d0);
                let d0 = if d0 > 0.0 { d0 } else { 250.0 * c.f.scale };
                probe::drag(c, h, &exact(vec![m, b]), Some(id), Some(Cont { d0, phase: 0.0, open_end: false, keep: true }));
            }
            "lift (2 calls)" => {
                c.drag(h, &exact(vec![a, m]), None);
                c.drag(h, &exact(vec![m, b]), None);
            }
            _ => unreachable!(),
        }
    };
    for linen in grounds() {
        for set in sets("today;0,0") {
            set.apply();
            let tag = format!("{}-{}", if linen { "linen" } else { "smooth" }, set.name());
            let run = |how: &str| {
                let (mut c, f0) = base(linen);
                let mut h = brush(13);
                h.load(umber, 0.9);
                line(&mut c, &mut h, how);
                let d: Vec<f32> = (0..c.wet.vol.len()).map(|i| dep(&c, &f0, i)).collect();
                (c, f0, d, h.carried().0)
            };
            let (r, rf0, rd, rb) = run("2 points");
            let rt = totals(&r, &rf0).0;
            for how in ["51 points", "nudged", "cont same id", "cont new id", "set down", "new grid", "lift (2 calls)"] {
                let (c, f0, d, hb) = run(how);
                let f = c.f;
                let (mut up, mut near, mut down, mut sum, mut n) = (0.0f32, 0.0f32, 0.0f32, 0.0f64, 0.0f64);
                let mut all = Vec::new();
                for i in 0..d.len() {
                    let x = f.ux(i % f.w);
                    let y = f.uy(i / f.w);
                    let e = (d[i] - rd[i]).abs();
                    if x < 450.0 {
                        up = up.max(e);
                    } else if x < 550.0 {
                        near = near.max(e);
                    } else {
                        down = down.max(e);
                    }
                    if x >= BAND.0 && x < BAND.2 && y >= BAND.1 && y < BAND.3 {
                        sum += e as f64;
                        n += 1.0;
                    }
                    if rd[i] > 0.1 || d[i] > 0.1 {
                        all.push(e);
                    }
                }
                all.sort_by(f32::total_cmp);
                let p99 = all[all.len() * 99 / 100];
                let t = totals(&c, &f0).0;
                println!(
                    "{tag:>14} {how:<15} largest |Δ| µm: up {up:6.3} near {near:6.3} down {down:6.3}, 99th pct {p99:.3} | band mean |Δ| {:.3} µm (ref mean {:.2}) | canvas paint {:+.2}% brush paint {:+.2}%",
                    sum / n,
                    stats(&r, &rf0, BAND).dep,
                    100.0 * (t - rt) / rt,
                    100.0 * (hb - rb) / rb
                );
            }
        }
    }
    reset();
}

/// The ground/pressure/load response with each setting explicitly on
/// (HANDOVER-3 Lane A (b), (d)). One stroke (the old `exchange_side`
/// gesture) at pressures .85/.5/.3 and loads .9/.3 on linen and smooth:
/// paint laid (µm·px, the open film: nothing has dried), the gross paint
/// the hairs laid and lifted (µm·px), and the contact-weighted means of
/// what couples the load to the exchange: `hunger` (pickup), the reach
/// into the weave (max(wick, reach), which the load's wetness sets) and
/// the share of a hair's track in contact (`touch`). Then the load ratio
/// .3/.9 with the hunger and/or the contact held at a fixed load (fill
/// 0.5), to separate them from the supply.
#[test]
#[ignore]
fn exchange_response() {
    let umber = umber();
    let one = |linen: bool, p: f32, load: f32| {
        let (mut c, _) = base(linen);
        let mut h = brush(5);
        h.load(umber, load);
        probe::take_tally();
        c.drag(&mut h, &Gesture::new(vec![(250.0, 350.0), (750.0, 353.0)]).pressure(p, p), None);
        let t = probe::take_tally();
        let k = COAT_UM as f64 * (c.f.scale as f64).powi(2);
        let laid = c.wet.vol.iter().map(|v| *v as f64).sum::<f64>() * COAT_UM as f64;
        (laid, t.laid_paint * k, t.lifted_paint * k, t.hunger_w / t.w, t.wet_w / t.w, t.touch_cov / t.cov)
    };
    for set in sets("today;0,0") {
        set.apply();
        for p in [0.85f32, 0.5, 0.3] {
            for load in [0.9f32, 0.3] {
                let mut line = format!("{:>6} pressure {p:.2} load {load:.1}:", set.name());
                for linen in [true, false] {
                    let (laid, gl, gp, hu, re, to) = one(linen, p, load);
                    line += &format!(" {} net {laid:6.0} (laid {gl:6.0} lifted {gp:6.0}; hunger {hu:.2} reach {re:.2} touch {to:.2}) |", if linen { "linen" } else { "smooth" });
                }
                println!("{line}");
            }
        }
        for p in [0.85f32, 0.5] {
            for (name, fc, fh) in [("coupled", None, None), ("hunger fixed", None, Some(0.5)), ("contact fixed", Some(0.5), None), ("both fixed", Some(0.5), Some(0.5))] {
                probe::set_fix_fill(fc, fh);
                let mut line = format!("{:>6} pressure {p:.2} {name:<13} load .3/.9:", set.name());
                for linen in [true, false] {
                    let a = one(linen, p, 0.3);
                    let b = one(linen, p, 0.9);
                    line += &format!(" {} {:.3} (net {:.0}/{:.0}, lifted {:.0}/{:.0})", if linen { "linen" } else { "smooth" }, a.0 / b.0, a.0, b.0, a.2, b.2);
                }
                println!("{line}");
            }
            probe::set_fix_fill(None, None);
        }
    }
    reset();
}
