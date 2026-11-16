//! Tracing a study's drawing onto another sheet (gap 4 of
//! notes/cassatt_support.md; notes/plan-intaglio.md §7, phase G4). The
//! numbers and their sources: notes/research/tracing.md.
//!
//! Cassatt moved figures from her studies to pastels and plates by tracing.
//! There are three steps, and each is a physical exchange:
//!
//! - **Which lines the tracer follows** (`guide_paths`). The study's drawn
//!   lines are its drawing guide (`Canvas::drawing_guide`). The tracer runs
//!   a point down the middle of every line it can see. A line is thinned to
//!   its middle (Zhang–Suen) and chained into paths. Lines fainter than the
//!   `level` the tracer sees are passed over [E].
//! - **How the hand follows a line** (`Hand`). This is steering through a
//!   tunnel as wide as the line, which is measured (Accot & Zhai 1997, a
//!   stylus on a tablet). The hand moves at a speed that grows with the
//!   tunnel's width, v = W/b (b = 78 ms), and so takes L·b/W for a line of
//!   length L. It strays to either side by σ = W/4.133: the width that holds
//!   96 % of the hits is the tunnel's (ISO 9241-9's effective width). The
//!   strays are corrective submovements, 2–3 a second (intermittent
//!   tracking), so they wander over v/2.5 Hz of the line. A tracing made by
//!   way of tracing paper is traced twice (study → tracing paper, tracing
//!   paper → sheet), so two hands' strays add (σ√2). Any wobble finer than
//!   σ in the study's own line is lost: the tracer follows the line's
//!   middle, not its edge.
//! - **What the point lays** (`Contact`, `Canvas::trace_transfer`). A ball
//!   stylus of radius R is pressed with force F through the sheet that
//!   carries the coat onto the sheet below it, and the stack gives. Each
//!   paper compresses by Chen et al.'s (2020) measured curve
//!   σ = (E/β)(e^{βε} − 1), β = 13.54 (sheet.rs `chen`), started at the
//!   paper model's own through-thickness modulus E (`Paper::z_mpa`, its
//!   mount folded in). It stops once its pores have closed, at ε = its
//!   porosity. The springs act side by side, each under the ball's local
//!   sink δ(r) = δ₀ − r²/2R. So F = 2πR ∫₀^{δ₀} p dδ, which has a closed
//!   form for each layer, and the contact's radius is a = √(2Rδ₀). More
//!   force widens the line. The width stops growing where the stack is
//!   fully dense, √(2R·Σtφ), or where the ball has sunk to its equator, R.
//!
//! **The coat.** Red chalk, charcoal or graphite is rubbed on the back of a
//! tracing (or of the study itself) and the excess wiped off, as
//! practitioners do. What stays fills the back's tooth: µ_back of it, the
//! paper's mean pore depth (`Paper::pore_um`). Where the ball's sink reaches
//! into the sheet below, the coat touches the share of that sheet's surface
//! within δ of its fiber tops, 1 − e^{−δ/µ} (the same bearing curve the
//! pastel stick and the knife see). Of the coat that touches, half comes
//! away (`SPLIT` [E]). Whatever leaves the back lies in the sheet's tooth
//! as loose grains, the coat's own (its color, its lift, its carbon). The
//! bookkeeping is exact: the back loses what the sheet gains.
//!
//! **A blind stylus** (no coat) lays nothing. Its line goes into the
//! sheet's drawing guide only: a guide mask to paint into. The dent it
//! leaves isn't modeled.
//!
//! **The seam for soft ground (P5).** P5's `trace` onto a plate reuses
//! `guide_paths`, `Hand` and `contact`. What a ball's contact does to a
//! soft ground under the sheet is P5's: here it only lifts a coat.

use crate::canvas::Canvas;
use crate::color::Rgb;
use crate::mask::Mask;
use crate::paper::Paper;
use crate::rng::Rng;

/// Chen et al.'s (2020) exponent for paper's z-compression (sheet.rs `chen`).
pub const CHEN_EXP: f32 = 13.54;
/// The share of a loose coat that leaves the back where it touches the sheet
/// below [E]. A bed of particles between two sheets of paper, held about
/// equally by both, parts at its weakest plane. With a uniform bed, that
/// plane lies at an even chance anywhere through it: half on average. A
/// weighing (a coated sheet before and after a ruled set of lines) would
/// set it (notes/research/tracing.md §4).
pub const SPLIT: f32 = 0.5;
/// The faintest line the tracer follows, as the guide reads it (1 is a firm
/// line, about 0.5 a light 2H line) [E]. An eraser's ghost reads below it.
pub const LEVEL: f32 = 0.25;
/// The steering law's slope for a stylus, s: T = a + b·L/W (Accot & Zhai
/// 1997, a stylus on a Wacom tablet, the straight tunnel: b = 78 ms).
pub const STEER_S: f32 = 0.078;
/// The tunnel's width over the hand's stray: the width that holds 96 % of
/// the hits is 4.133 σ (ISO 9241-9's effective width; Soukoreff &
/// MacKenzie 2004).
pub const EFFECTIVE_WIDTH: f32 = 4.133;
/// Corrective submovements in visually guided tracking, per second: 2–3
/// (Miall, Weir & Stein 1993, and the studies after them). Their middle.
pub const CORRECTIONS_HZ: f32 = 2.5;
/// A tracing stylus's ball, mm across (the common sizes are 1, 1.5, 2, 2.5
/// and 3 mm; the finest, 1 mm, is the one for a drawing's lines).
pub const STYLUS_MM: f32 = 1.0;
/// The set-down of the point at each line, s (the pencil's: tally.rs
/// `pace::PENCIL_LINE` [E]).
pub const SET_DOWN_S: f32 = 0.3;

/// One paper in the stack under the stylus.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Layer {
    /// Its caliper, µm.
    pub t_um: f32,
    /// Its through-thickness modulus at no load, MPa.
    pub e_mpa: f32,
    /// Its pore share: it compresses no further once they have closed.
    pub porosity: f32,
}

impl Layer {
    /// The sheet `p`: its caliper, its z modulus (with its mount) and its porosity.
    pub fn of(p: &Paper) -> Layer {
        Layer { t_um: p.caliper_um().max(10.0), e_mpa: p.z_mpa.max(0.1), porosity: p.porosity.clamp(0.02, 0.95) }
    }

    /// The pressure at which its pores have closed, MPa.
    fn p_dense(&self) -> f32 {
        self.e_mpa * ((CHEN_EXP * self.porosity).exp() - 1.0) / CHEN_EXP
    }

    /// Its strain under pressure `p` MPa.
    pub fn strain(&self, p: f32) -> f32 {
        ((1.0 + CHEN_EXP * p.max(0.0) / self.e_mpa).ln() / CHEN_EXP).min(self.porosity)
    }

    /// ∫₀^p p dδ over its compression, MPa·µm (closed form; nothing more once it is dense).
    fn work(&self, p: f32) -> f32 {
        let q = p.max(0.0).min(self.p_dense()) as f64;
        let (e, b) = (self.e_mpa as f64, CHEN_EXP as f64);
        (self.t_um as f64 * (q / b - e / (b * b) * (1.0 + b * q / e).ln())) as f32
    }
}

/// A ball pressed through a stack of papers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Contact {
    /// The ball's radius, mm.
    pub tip_mm: f32,
    /// How far it sinks at its middle, µm.
    pub sink_um: f32,
    /// The contact's radius, mm: half the line's width.
    pub a_mm: f32,
    /// Whether it is at the limit: the stack fully dense, or the ball sunk to its equator.
    pub limited: bool,
}

impl Contact {
    /// The ball's sink at `y_mm` from the line's middle, µm (≤ 0 off the contact).
    pub fn sink_at(&self, y_mm: f32) -> f32 {
        self.sink_um - y_mm * y_mm / (2.0 * self.tip_mm) * 1000.0
    }

    /// The line's width, mm.
    pub fn width_mm(&self) -> f32 {
        2.0 * self.a_mm
    }
}

/// The deepest a ball of radius `tip_mm` sinks into `stack`, µm: every
/// layer dense, or the ball to its equator.
fn sink_limit_um(tip_mm: f32, stack: &[Layer]) -> f32 {
    let dense: f32 = stack.iter().map(|l| l.t_um * l.porosity).sum();
    dense.min(500.0 * tip_mm)
}

/// The widest line a ball of radius `tip_mm` presses into `stack`, mm.
pub fn width_limit_mm(tip_mm: f32, stack: &[Layer]) -> f32 {
    2.0 * (2.0 * tip_mm * sink_limit_um(tip_mm, stack) * 1e-3).sqrt()
}

/// A ball of radius `tip_mm` pressed with `force_n` N into `stack`. The
/// layers act side by side under its sink δ(r) = δ₀ − r²/2R, so
/// F = 2πR Σ ∫₀^p p dδᵢ at the middle's pressure p. That is solved for p by
/// bisection, and then δ₀ = Σ tᵢεᵢ(p).
pub fn contact(tip_mm: f32, force_n: f32, stack: &[Layer]) -> Contact {
    let r = tip_mm.max(1e-3);
    let limit = sink_limit_um(r, stack);
    let sink = |p: f32| stack.iter().map(|l| l.t_um * l.strain(p)).sum::<f32>();
    // N: 2πR (mm) × MPa·µm = 1e-3 N
    let force = |p: f32| 2.0 * std::f32::consts::PI * r * stack.iter().map(|l| l.work(p)).sum::<f32>() * 1e-3;
    let p_hi = stack.iter().map(|l| l.p_dense()).fold(0.0f32, f32::max);
    let (mut lo, mut hi) = (0.0f32, p_hi);
    if force(p_hi) <= force_n || stack.is_empty() {
        lo = p_hi;
    } else {
        for _ in 0..60 {
            let m = 0.5 * (lo + hi);
            if force(m) < force_n { lo = m } else { hi = m }
        }
    }
    let d = sink(0.5 * (lo + hi.max(lo))).min(limit);
    let limited = d >= limit * 0.9999;
    Contact { tip_mm: r, sink_um: d.max(0.0), a_mm: (2.0 * r * d.max(0.0) * 1e-3).sqrt().min(r), limited }
}

/// A natural tracing paper, 90 g/m², 76 µm (ISO 534; HP's and Alps' sheets).
/// It is 1.18 g/cm³ against the fiber wall's 1.5, so its pores are 21 % of
/// it (hence transparent). Collapsed fibers 4.3 µm thick give that caliper
/// at the drawing paper's fiber width and coarseness. Its z modulus is the
/// drawing paper's [E].
pub fn tracing_paper(seed: u64) -> Paper {
    let mut p = Paper { grammage: 90.0, porosity: 0.21, calender: 0.8, felt: None, laid: None, cobb60: Some(25.0), ..Paper::drawing(seed) };
    // caliper = c̄·t_f/(1 − ε) = 76 µm
    p.thick_um = 76.0 * (1.0 - p.porosity) / p.coverage();
    p
}

/// The tracer's hand on a line `w_px` px wide (`Hand::follow`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hand {
    /// The line's width, px: the tunnel the hand steers through.
    pub w_px: f32,
    /// How many times the line is traced by hand on its way (1 at a window
    /// or straight from the study, 2 by way of tracing paper).
    pub generations: u32,
}

impl Hand {
    /// The hand's stray to either side, px (its standard deviation).
    pub fn sigma_px(&self) -> f32 {
        self.w_px / EFFECTIVE_WIDTH * (self.generations.max(1) as f32).sqrt()
    }

    /// The length of line over which a stray wanders, px: the hand's speed
    /// W/b over the rate of its corrections.
    pub fn wander_px(&self) -> f32 {
        self.w_px / STEER_S / CORRECTIONS_HZ
    }

    /// The seconds a line of `len_px` takes to trace (every generation).
    pub fn secs(&self, len_px: f32) -> f32 {
        self.generations.max(1) as f32 * (SET_DOWN_S + len_px * STEER_S / self.w_px.max(1e-3))
    }

    /// The path the point takes along `path` (px): smoothed through the
    /// thinned line's pixel steps, then strayed to either side by the hand.
    /// The stray is a sum of five sinusoids about the wander length, with
    /// phases from `seed`. Its standard deviation is exactly `sigma_px`.
    pub fn follow(&self, path: &[(f32, f32)], seed: u64) -> Vec<(f32, f32)> {
        if path.len() < 2 {
            return path.to_vec();
        }
        let p = crate::graphite::resample(path, true, 0.5);
        let s = crate::path::arclen(&p);
        let (sigma, lam) = (self.sigma_px(), self.wander_px().max(1.0));
        let mut rng = Rng::new(seed ^ 0x7ACE);
        const K: [f32; 5] = [0.62, 0.81, 1.0, 1.27, 1.6];
        let waves: Vec<(f32, f32)> = K.iter().map(|k| (std::f32::consts::TAU / (lam * k), rng.range(0.0, std::f32::consts::TAU))).collect();
        let amp = sigma * (2.0 / K.len() as f32).sqrt();
        let n = p.len();
        (0..n)
            .map(|i| {
                let (a, b) = (p[i.saturating_sub(1)], p[(i + 1).min(n - 1)]);
                let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                let l = (dx * dx + dy * dy).sqrt().max(1e-6);
                let d = amp * waves.iter().map(|(w, ph)| (w * s[i] + ph).sin()).sum::<f32>();
                (p[i].0 - dy / l * d, p[i].1 + dx / l * d)
            })
            .collect()
    }
}

/// The lines of a guide (a whole-canvas mask) the tracer follows: where it
/// reads at least `level`, thinned to their middles and chained into paths,
/// in whole-canvas pixels (centers at +0.5). A lone dot is a one-point path.
/// Also returns the lines' mean width, px: their area over their length.
pub fn guide_paths(m: &Mask, level: f32) -> (Vec<Vec<(f32, f32)>>, f32) {
    let (w, h) = (m.f.w, m.f.h);
    let on: Vec<bool> = m.data.iter().map(|&v| v >= level).collect();
    let area = on.iter().filter(|&&b| b).count();
    let mut sk = on.clone();
    thin(&mut sk, w, h);
    let paths = chains(&sk, w, h);
    let len: f32 = paths.iter().map(|p| crate::path::arclen(p).last().copied().unwrap_or(0.0).max(1.0)).sum();
    let paths: Vec<Vec<(f32, f32)>> = paths.into_iter().map(|p| p.into_iter().map(|(x, y)| (x + 0.5, y + 0.5)).collect()).collect();
    let width = if len > 0.0 { (area as f32 / len).max(1.0) } else { 1.0 };
    (paths, width)
}

/// Zhang & Suen's (1984) thinning, in place: a line of any width becomes
/// its one-pixel middle, its ends and branches kept.
fn thin(b: &mut [bool], w: usize, h: usize) {
    if w < 3 || h < 3 {
        return;
    }
    let mut del = Vec::new();
    loop {
        let mut changed = false;
        for step in 0..2 {
            del.clear();
            for y in 1..h - 1 {
                for x in 1..w - 1 {
                    let i = y * w + x;
                    if !b[i] {
                        continue;
                    }
                    // P2..P9 clockwise from the top
                    let n = [b[i - w], b[i - w + 1], b[i + 1], b[i + w + 1], b[i + w], b[i + w - 1], b[i - 1], b[i - w - 1]];
                    let count = n.iter().filter(|&&v| v).count();
                    if !(2..=6).contains(&count) {
                        continue;
                    }
                    let trans = (0..8).filter(|&k| !n[k] && n[(k + 1) % 8]).count();
                    if trans != 1 {
                        continue;
                    }
                    let (p2, p4, p6, p8) = (n[0], n[2], n[4], n[6]);
                    let ok = if step == 0 { !(p2 && p4 && p6) && !(p4 && p6 && p8) } else { !(p2 && p4 && p8) && !(p2 && p6 && p8) };
                    if ok {
                        del.push(i);
                    }
                }
            }
            for &i in &del {
                b[i] = false;
            }
            changed |= !del.is_empty();
        }
        if !changed {
            break;
        }
    }
}

/// A thinned line's pixels chained into paths: from each end, then round
/// whatever is left (loops). A path that runs into one already walked (a
/// junction) takes that pixel as its last, so branches meet.
fn chains(b: &[bool], w: usize, h: usize) -> Vec<Vec<(f32, f32)>> {
    const N4: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    const ND: [(isize, isize); 4] = [(1, 1), (-1, 1), (-1, -1), (1, -1)];
    let at = |x: isize, y: isize| -> Option<usize> { (x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h && b[y as usize * w + x as usize]).then_some(y as usize * w + x as usize) };
    let nbrs = |i: usize| -> Vec<usize> {
        let (x, y) = ((i % w) as isize, (i / w) as isize);
        N4.iter().chain(ND.iter()).filter_map(|&(dx, dy)| at(x + dx, y + dy)).collect()
    };
    let mut seen = vec![false; b.len()];
    let mut out = Vec::new();
    let ends: Vec<usize> = (0..b.len()).filter(|&i| b[i] && nbrs(i).len() == 1).collect();
    let rest: Vec<usize> = (0..b.len()).filter(|&i| b[i]).collect();
    for start in ends.into_iter().chain(rest) {
        if seen[start] {
            continue;
        }
        seen[start] = true;
        let mut path = vec![start];
        loop {
            let cur = *path.last().unwrap();
            let ns = nbrs(cur);
            if let Some(&n) = ns.iter().find(|&&n| !seen[n]) {
                seen[n] = true;
                path.push(n);
                continue;
            }
            // into a junction walked already: meet it
            let recent = &path[path.len().saturating_sub(3)..];
            if let Some(&n) = ns.iter().find(|n| !recent.contains(n)) {
                path.push(n);
            }
            break;
        }
        out.push(path.iter().map(|&i| ((i % w) as f32, (i / w) as f32)).collect());
    }
    out
}

/// What a coat is made of: the grains it leaves in the tooth.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coat {
    pub color: Rgb,
    /// How readily the eraser takes it (the cell's `lift`).
    pub lift: f32,
    /// The share of it that is bare carbon (charcoal).
    pub carbon: f32,
}

impl Coat {
    /// Rubbed from a pastel or charcoal stick (red chalk is a hard pastel of red earth).
    pub fn of_stick(s: &crate::pastel::Stick) -> Coat {
        Coat { color: s.color, lift: s.lift, carbon: if s.carbon { 1.0 } else { 0.0 } }
    }

    /// Rubbed from a graphite pencil or black chalk: its flakes' gray. Loose
    /// powder in the tooth, as rubbed graphite is; its sheen isn't modeled.
    pub fn of_lead(l: &crate::graphite::Lead) -> Coat {
        Coat { color: [l.flake; 3], lift: l.lift(), carbon: 0.0 }
    }

    fn grains(&self) -> crate::pastel::Grains {
        crate::pastel::Grains { lift: self.lift, carbon: self.carbon, lies: 1.0, index: 0.0 }
    }
}

/// What a tracing laid.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Laid {
    /// The coat that left the back, mm³ (as it lies in the tooth): all of it now on the sheet.
    pub moved_mm3: f64,
    /// The coat the back held over the sheet before, mm³.
    pub coat_mm3: f64,
    /// Pixels the point passed over wet paint, where it laid nothing.
    pub wet: usize,
}

/// Where the back of a tracing lies on the sheet below, in that sheet's
/// whole-canvas pixels (x0, y0, x1, y1), end-exclusive.
pub type Back = (f32, f32, f32, f32);

impl Canvas {
    /// The coated back of a tracing, lying over `back`, its coat `coat_um`
    /// µm thick (as it lies in the tooth), retraced along `paths` (this
    /// canvas's whole pixels, the hand's strays in them) by a ball in
    /// `contact`. Each path takes its share from what the back still holds
    /// under it (`SPLIT` of what touches: 1 − e^{−δ/µ}, δ the ball's sink
    /// there and µ this sheet's pores) and lays it as loose grains of
    /// `coat`. Each pixel gets one pass per path, its nearest. Every path
    /// goes into the drawing guide as a line the contact's width (at least
    /// two pixels), as firm as its middle touches. With no coat (a blind
    /// stylus) only the guide takes it. Nothing lands on wet paint.
    pub fn trace_transfer(&mut self, paths: &[Vec<(f32, f32)>], back: Back, contact: Contact, coat: Option<(Coat, f32)>) -> Laid {
        let f = self.f;
        let wf = f.whole();
        let px_mm = self.px_mm();
        let a_px = contact.a_mm / px_mm;
        let reach = a_px.max(1.0) + 1.0;
        let mut laid = Laid::default();
        let px_mm2 = (px_mm as f64) * (px_mm as f64);
        // the coat over each pixel of the window the back covers
        let (bx0, by0, bx1, by1) = back;
        let under = |gx: usize, gy: usize| (gx as f32 + 0.5) >= bx0 && (gx as f32 + 0.5) < bx1 && (gy as f32 + 0.5) >= by0 && (gy as f32 + 0.5) < by1;
        let mut left: Vec<f32> = match coat {
            Some((_, um)) => (0..f.w * f.h).map(|i| if under(i % f.w + f.x0, i / f.w + f.y0) { um } else { 0.0 }).collect(),
            None => Vec::new(),
        };
        laid.coat_mm3 = left.iter().map(|&v| v as f64).sum::<f64>() * px_mm2 * 1e-3;
        self.drawing_mut();
        let mut d = self.drawing.take().unwrap();
        let mut pxs = std::mem::take(&mut self.px);
        // a middle's touch on this sheet's mean pores, for the guide
        let mu_mean = self.paper.map_or(2.0, |p| p.pore_um());
        let firm = (1.0 - (-contact.sink_um / mu_mean.max(1e-3)).exp()).clamp(0.0, 1.0);
        let gw = a_px.max(1.0);
        for path in paths {
            // (whole pixel, distance px): the nearest pass of the path over each
            let mut hits: Vec<(usize, f32)> = Vec::new();
            let segs: Vec<((f32, f32), (f32, f32))> = if path.len() == 1 { vec![(path[0], path[0])] } else { path.windows(2).map(|s| (s[0], s[1])).collect() };
            for (a, b) in segs {
                let to = |u: f32, n: usize| (u.floor().max(0.0) as usize).min(n);
                let (x0, x1) = (to(a.0.min(b.0) - reach, wf.w), (to(a.0.max(b.0) + reach, wf.w) + 1).min(wf.w));
                let (y0, y1) = (to(a.1.min(b.1) - reach, wf.h), (to(a.1.max(b.1) + reach, wf.h) + 1).min(wf.h));
                let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                let l2 = dx * dx + dy * dy;
                for y in y0..y1 {
                    for x in x0..x1 {
                        let (xc, yc) = (x as f32 + 0.5, y as f32 + 0.5);
                        let t = if l2 > 0.0 { (((xc - a.0) * dx + (yc - a.1) * dy) / l2).clamp(0.0, 1.0) } else { 0.0 };
                        let dist = ((xc - a.0 - t * dx).powi(2) + (yc - a.1 - t * dy).powi(2)).sqrt();
                        if dist < reach {
                            hits.push((y * wf.w + x, dist));
                        }
                    }
                }
            }
            hits.sort_unstable_by(|p, q| p.0.cmp(&q.0).then(p.1.total_cmp(&q.1)));
            hits.dedup_by_key(|h| h.0);
            for &(k, dist) in &hits {
                // the guide: the line the contact's width, its edge antialiased
                let cov = (0.5 - (dist - gw)).clamp(0.0, 1.0);
                if cov > 0.0 {
                    d.lay_guide(k, 0.5 * firm * cov);
                }
                let Some((c, _)) = coat else { continue };
                let (gx, gy) = (k % wf.w, k / wf.w);
                if gx < f.x0 || gy < f.y0 || gx >= f.x0 + f.w || gy >= f.y0 + f.h {
                    continue;
                }
                let i = (gy - f.y0) * f.w + gx - f.x0;
                let o = contact.sink_at(dist * px_mm);
                if o <= 0.0 || left[i] <= 0.0 {
                    continue;
                }
                if self.wet.vol[i] > 1e-5 {
                    laid.wet += 1;
                    continue;
                }
                let share = SPLIT * (1.0 - (-o / self.micro_um(i).max(1e-3)).exp());
                let dv = share * left[i];
                if dv <= 0.0 {
                    continue;
                }
                left[i] -= dv;
                laid.moved_mm3 += dv as f64 * px_mm2 * 1e-3;
                let g = c.grains();
                if g.carbon > 0.0 && d.optics() {
                    d.carbon = true;
                }
                d.sticks = true;
                let cell = &mut d.cells[i];
                if cell.film < 0.0 || self.film[i] > cell.film + 1e-4 {
                    cell.restart(self.film[i], pxs[i]);
                }
                let q = 1.0 - (-dv / crate::pastel::PARTICLE_UM).exp();
                let before = (cell.a, cell.reflectance());
                crate::pastel::lay_into(cell, q, c.color, dv, g);
                pxs[i] = cell.repaint(pxs[i], before);
            }
        }
        self.px = pxs;
        self.drawing = Some(d);
        laid
    }

    /// The loose pastel in the window's tooth, mm³ (as it lies): what a
    /// tracing's bookkeeping is checked against.
    pub fn loose_mm3(&self) -> f64 {
        let px = self.px_mm() as f64;
        self.drawing.as_ref().map_or(0.0, |d| d.cells.iter().map(|c| c.loose.max(0.0) as f64).sum::<f64>()) * px * px * 1e-3
    }
}

#[cfg(test)]
mod tests;
