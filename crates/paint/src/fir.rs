//! Firs and spruces grown into a silhouette the painter draws, and fir woods
//! grown behind a drawn skyline. Geometry only, as in `growth`: a leader,
//! whorls of boughs (polylines with widths), the needle masses they carry,
//! the short hatched strokes a pointed brush lays on them (Friedrich's firs
//! are "short, hatched strokes" [NG pp.49–50]) and how each faces the light.
//!
//! How a spruce is built, and what the envelope decides:
//!
//! - the leader runs from the foot to the envelope's apex, wandering a
//!   little, sometimes with an elbow where a lost top was replaced by a side
//!   shoot that turned up;
//! - every year's shoot ends in a whorl of 3–6 boughs around the stem (a
//!   new phase each year) plus a few short interwhorl ones; the eye sees
//!   them in projection, so a bough toward you is short and one to the side
//!   long, and the tiers come out uneven by themselves;
//! - each bough reaches toward the envelope at its height (the drawn
//!   silhouette is where the tips end, bough by bough, so the tree grows
//!   *into* it), rises near the top, sags lower down and lifts at the tip;
//! - some are missing (in runs: a gap in the crown), some broken, the lowest
//!   dead (gray, twiggy, without needles) and stubs stay on a bare trunk;
//! - the needles hang from each bough as a pad, thin above the bough and
//!   deep below it, where the pendant shoots hang (the comb of a Norway
//!   spruce); an old tree's pads break into separate tufts.
//!
//! Deterministic by seed.

use crate::noise::Fbm;
use crate::{Frame, Mask, Rng, Shape, lerp, smoothstep};
use std::f32::consts::{PI, TAU};

/// How a fir grows (the painter's envelope sets its size and outline).
#[derive(Clone, Debug)]
pub struct FirHabit {
    /// About how many whorls from the apex to the crown's base.
    pub tiers: f32,
    /// Boughs per whorl (min, max).
    pub per_whorl: (u32, u32),
    /// Short interwhorl boughs per year (mean).
    pub inter: f32,
    /// How far a bough reaches toward the envelope (fraction, min..max).
    pub fill: (f32, f32),
    /// Chance a bough is missing (in runs down the crown).
    pub gap: f32,
    /// Chance a live bough ends in a break.
    pub broken: f32,
    /// The share of the crown, from its base, where boughs are dead.
    pub dead_below: f32,
    /// Leader wander (fraction of the height).
    pub crook: f32,
    /// Chance of an elbow in the leader (a lost top).
    pub kink: f32,
    /// Bough angle at the top and at the crown's base (radians, + up).
    pub rise: (f32, f32),
    /// Sag along a bough.
    pub droop: f32,
    /// Lift at the tip.
    pub upturn: f32,
    /// Needle pad depth, as a fraction of the whorl spacing.
    pub pad: f32,
    /// 0: a continuous pad along each bough; 1: separate tufts.
    pub clumpy: f32,
    /// Inner share of a bough without needles.
    pub bare_inner: f32,
    /// Wind (+ blows to the right): windward boughs short, all swept leeward.
    pub wind: f32,
    /// Trunk width at the foot as a fraction of the height.
    pub trunk: f32,
    /// Chance the top is a dead spike.
    pub dead_top: f32,
}

impl FirHabit {
    /// A tall narrow spruce: dense, fairly regular, lower boughs hanging.
    pub fn spire() -> Self {
        FirHabit {
            tiers: 28.0,
            per_whorl: (4, 6),
            inter: 1.2,
            fill: (0.78, 1.02),
            gap: 0.06,
            broken: 0.04,
            dead_below: 0.1,
            crook: 0.006,
            kink: 0.12,
            rise: (0.55, -0.35),
            droop: 0.35,
            upturn: 0.35,
            pad: 1.3,
            clumpy: 0.3,
            bare_inner: 0.1,
            wind: 0.0,
            trunk: 0.028,
            dead_top: 0.0,
        }
    }
    /// An old, ragged fir: gaps, broken and dead boughs, tufted needles,
    /// a crooked leader.
    pub fn old() -> Self {
        FirHabit {
            tiers: 20.0,
            per_whorl: (4, 6),
            inter: 1.0,
            fill: (0.6, 1.05),
            gap: 0.18,
            broken: 0.22,
            dead_below: 0.32,
            crook: 0.02,
            kink: 0.6,
            rise: (0.35, -0.7),
            droop: 0.6,
            upturn: 0.22,
            pad: 1.5,
            clumpy: 0.7,
            bare_inner: 0.28,
            wind: 0.0,
            trunk: 0.04,
            dead_top: 0.35,
        }
    }
    /// A young fir: dense to the ground, boughs rising, few years.
    pub fn young() -> Self {
        FirHabit {
            tiers: 13.0,
            per_whorl: (4, 6),
            inter: 1.8,
            fill: (0.84, 1.02),
            gap: 0.02,
            broken: 0.0,
            dead_below: 0.0,
            crook: 0.004,
            kink: 0.0,
            rise: (0.8, -0.05),
            droop: 0.2,
            upturn: 0.3,
            pad: 1.35,
            clumpy: 0.12,
            bare_inner: 0.04,
            wind: 0.0,
            trunk: 0.03,
            dead_top: 0.0,
        }
    }
    /// A storm-bent fir: boughs flagged to leeward, the windward side bare.
    pub fn storm() -> Self {
        FirHabit {
            tiers: 22.0,
            per_whorl: (3, 5),
            inter: 0.8,
            fill: (0.5, 1.02),
            gap: 0.16,
            broken: 0.16,
            dead_below: 0.22,
            crook: 0.02,
            kink: 0.3,
            rise: (0.3, -0.45),
            droop: 0.45,
            upturn: 0.15,
            pad: 1.05,
            clumpy: 0.55,
            bare_inner: 0.22,
            wind: 0.8,
            trunk: 0.035,
            dead_top: 0.15,
        }
    }
    pub fn named(name: &str) -> Option<Self> {
        Some(match name {
            "spire" | "spruce" => Self::spire(),
            "old" | "ragged" => Self::old(),
            "young" => Self::young(),
            "storm" => Self::storm(),
            _ => return None,
        })
    }
}

/// One bough as seen: from the stem to its tip.
#[derive(Clone, Debug)]
pub struct Bough {
    pub pts: Vec<(f32, f32)>,
    /// Full width at each point.
    pub w: Vec<f32>,
    /// Needle mass hanging from each point: (above, below) the bough line.
    pub pad: Vec<(f32, f32)>,
    /// Depth of the tip (units, + toward the viewer).
    pub z: f32,
    /// -1 left of the stem, 1 right (by where the tip lands).
    pub side: f32,
    /// Height in the crown: 0 at the apex, 1 at the crown's base.
    pub t: f32,
    pub dead: bool,
    pub broken: bool,
    /// A short interwhorl bough or a stub on the bare trunk.
    pub minor: bool,
}

/// What a hatch stroke lies on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Hanging from a bough: the pendant shoots, down and outward.
    Under,
    /// Along a bough's upper face.
    Top,
    /// Twigs on a dead bough.
    Twig,
}

impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Kind::Under => "under",
            Kind::Top => "top",
            Kind::Twig => "twig",
        }
    }
}

/// A short stroke of a pointed brush.
#[derive(Clone, Debug)]
pub struct Hatch {
    pub pts: [(f32, f32); 3],
    /// Suggested mark width (units).
    pub w: f32,
    /// How much it faces the light, 0..1.
    pub lit: f32,
    pub z: f32,
    pub bough: usize,
    pub kind: Kind,
}

/// A grown fir.
#[derive(Clone, Debug)]
pub struct Fir {
    pub leader: Vec<(f32, f32)>,
    pub leader_w: Vec<f32>,
    /// Index into `leader` from which it is dead (a spike); `len` if alive.
    pub leader_dead_from: usize,
    /// Back to front.
    pub boughs: Vec<Bough>,
    /// Back to front.
    pub strokes: Vec<Hatch>,
    pub apex: (f32, f32),
    pub foot: (f32, f32),
    /// The crown's base (lowest y of the envelope).
    pub crown_base: f32,
    /// Mean whorl spacing (units).
    pub tier: f32,
    /// Suggested hatch width (units).
    pub hatch: f32,
    pub envelope: Vec<(f32, f32)>,
    pub sun: (f32, f32, f32),
    pub seed: u64,
}

fn norm2(v: (f32, f32)) -> (f32, f32) {
    let d = (v.0 * v.0 + v.1 * v.1).sqrt().max(1e-6);
    (v.0 / d, v.1 / d)
}

/// The horizontal spans of a closed polygon at height `y`, sorted.
fn spans(poly: &[(f32, f32)], y: f32) -> Vec<(f32, f32)> {
    let n = poly.len();
    let mut xs = Vec::new();
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        if (a.1 <= y && b.1 > y) || (b.1 <= y && a.1 > y) {
            xs.push(a.0 + (y - a.1) / (b.1 - a.1) * (b.0 - a.0));
        }
    }
    xs.sort_by(f32::total_cmp);
    xs.as_chunks::<2>().0.iter().map(|c| (c[0], c[1])).collect()
}

/// The span at `y` holding `x` (or the nearest one).
fn span_at(poly: &[(f32, f32)], y: f32, x: f32) -> Option<(f32, f32)> {
    let s = spans(poly, y);
    s.iter()
        .copied()
        .min_by(|a, b| {
            let d = |s: &(f32, f32)| if x < s.0 { s.0 - x } else if x > s.1 { x - s.1 } else { 0.0 };
            d(a).total_cmp(&d(b))
        })
}

/// x of a polyline (monotone enough in y) at height y.
fn x_at_y(line: &[(f32, f32)], y: f32) -> f32 {
    for w in line.windows(2) {
        let (a, b) = (w[0], w[1]);
        let (lo, hi) = (a.1.min(b.1), a.1.max(b.1));
        if y >= lo && y <= hi && hi > lo {
            return a.0 + (y - a.1) / (b.1 - a.1) * (b.0 - a.0);
        }
    }
    // outside: nearest end
    let (f, l) = (line[0], line[line.len() - 1]);
    if (y - f.1).abs() < (y - l.1).abs() { f.0 } else { l.0 }
}

fn width_at_y(line: &[(f32, f32)], w: &[f32], y: f32) -> f32 {
    let mut best = (f32::MAX, w[0]);
    for (p, &wi) in line.iter().zip(w) {
        let d = (p.1 - y).abs();
        if d < best.0 {
            best = (d, wi);
        }
    }
    best.1
}

impl Fir {
    /// Grow a fir into the closed `envelope` (canvas units). `foot` is where
    /// the trunk meets the ground (default: under the envelope's base);
    /// `sun` points toward the sun (x right, y down, z toward the viewer).
    pub fn grow(envelope: &[(f32, f32)], foot: Option<(f32, f32)>, habit: &FirHabit, sun: (f32, f32, f32), seed: u64) -> Fir {
        let mut rng = Rng::new(seed ^ 0xF1F1_2024);
        let env: Vec<(f32, f32)> = envelope.to_vec();
        let apex = *env.iter().min_by(|a, b| a.1.total_cmp(&b.1)).unwrap();
        let y_bot = env.iter().map(|p| p.1).fold(f32::MIN, f32::max);
        let hc = (y_bot - apex.1).max(1.0);
        let foot = foot.unwrap_or_else(|| {
            let y = y_bot - 0.04 * hc;
            let x = span_at(&env, y, apex.0).map(|s| 0.5 * (s.0 + s.1)).unwrap_or(apex.0);
            (x, y_bot + 0.015 * hc)
        });
        let h = (foot.1 - apex.1).max(1.0);

        // the leader: foot (i = 0) to apex
        let m = 48;
        let wander = Fbm::new((seed as u32) ^ 0x1ead, 3, 0.35);
        let kink = if rng.chance(habit.kink) { Some((rng.range(0.7, 0.9), rng.range(-1.0, 1.0) * rng.range(0.012, 0.03) * h)) } else { None };
        let mut leader = Vec::with_capacity(m + 1);
        let mut leader_w = Vec::with_capacity(m + 1);
        for i in 0..=m {
            let t = i as f32 / m as f32;
            let mut x = lerp(foot.0, apex.0, t) + wander.get(t * 2.0, 0.5) * habit.crook * h * 4.0 * t * (1.0 - t);
            x += habit.wind * 0.03 * h * (PI * t).sin();
            if let Some((tk, d)) = kink
                && t > tk
            {
                x += d * (1.0 - (t - tk) / (1.0 - tk)) * smoothstep(tk, tk + 0.015, t);
            }
            leader.push((x, lerp(foot.1, apex.1, t)));
            leader_w.push((habit.trunk * h * (1.0 - t).powf(0.85)).max(0.35));
        }
        let leader_dead_from = if rng.chance(habit.dead_top) { (m as f32 * rng.range(0.9, 0.96)) as usize } else { m + 1 };

        let tiers = (habit.tiers * rng.range(0.9, 1.1)).max(3.0);
        let sp = hc / tiers;
        let gapz = Fbm::new((seed as u32) ^ 0x9a95, 2, 0.12);
        let tuft = Fbm::new((seed as u32) ^ 0x7af7, 2, 0.22);
        let mut boughs: Vec<Bough> = Vec::new();

        // one bough springing at height y0, azimuth phi
        let bough = |rng: &mut Rng, y0: f32, phi: f32, scale: f32, minor: bool, stub: bool| {
            let t = ((y0 - apex.1) / hc).clamp(0.0, 1.2);
            let ax = x_at_y(&leader, y0);
            let (c, sz) = (phi.cos(), phi.sin());
            let reach = |y: f32| -> f32 {
                match span_at(&env, y, ax) {
                    Some((xl, xr)) => lerp((ax - xl).max(0.0), (xr - ax).max(0.0), 0.5 * (1.0 + c)),
                    None => 0.0,
                }
            };
            let dead = stub || (habit.dead_below > 0.0 && rng.f() < smoothstep(1.0 - habit.dead_below, 1.0, t) * 1.15);
            let mut fill = rng.range(habit.fill.0, habit.fill.1) * scale;
            if habit.wind != 0.0 && c * habit.wind < 0.0 {
                fill *= 1.0 - 0.65 * habit.wind.abs() * c.abs();
            }
            if dead {
                fill *= rng.range(0.35, 0.8);
            }
            let a0 = lerp(habit.rise.0, habit.rise.1, t.min(1.0)) + rng.normal() * 0.1 - if dead { 0.2 } else { 0.0 };
            let sag = habit.droop * (0.2 + 0.8 * t.min(1.0)) * (1.0 + 0.25 * rng.normal()).max(0.3);
            let up = if dead { 0.0 } else { habit.upturn * (0.6 + 0.4 * t.min(1.0)) };
            let lift = |s: f32| a0.tan() * s - sag * s * s + up * 2.2 * (s - 0.55).max(0.0).powi(2);
            let mut l = if stub { hc * 0.05 * rng.range(0.4, 1.4) } else { reach(y0) * fill };
            // the tip lands inside the envelope at its own height
            if !stub {
                for _ in 0..2 {
                    let ty = y0 - l * lift(1.0) - 0.1 * l * sz;
                    let r = reach(ty);
                    if l * c.abs() > r * 1.02 && c.abs() > 0.2 {
                        l = r * 1.02 / c.abs();
                    }
                }
            }
            if l < 0.6 {
                return None;
            }
            let broken = !stub && rng.chance(if dead { 0.5 } else { habit.broken });
            let end = if broken { rng.range(0.35, 0.8) } else { 1.0 };
            let n = 10;
            let tw = width_at_y(&leader, &leader_w, y0);
            let w0 = (tw * 0.5 * (l / (0.25 * h)).sqrt().min(1.0)).max(0.25) * if minor { 0.7 } else { 1.0 };
            let pad_d = habit.pad * sp * rng.range(0.8, 1.25) * (l / (0.07 * h)).sqrt().clamp(0.55, 1.0);
            let tseed = rng.range(0.0, 100.0);
            let mut pts = Vec::with_capacity(n + 1);
            let mut w = Vec::with_capacity(n + 1);
            let mut pad = Vec::with_capacity(n + 1);
            for k in 0..=n {
                let s = end * k as f32 / n as f32;
                let r = l * s;
                let x = ax + r * c + habit.wind * 0.3 * l * s * s;
                let y = y0 - l * lift(s) - 0.1 * r * sz;
                pts.push((x, y));
                w.push((w0 * (1.0 - 0.8 * s)).max(0.15));
                if dead {
                    pad.push((0.0, 0.0));
                } else {
                    // inner shoots keep some needles by the stem unless the tree is old
                    let inner = if minor { 0.0 } else { 0.5 * (1.0 - habit.bare_inner * 2.5).max(0.0) };
                    let mut e = lerp(inner, 1.0, smoothstep(habit.bare_inner, habit.bare_inner + 0.2, s)) * (1.0 - 0.3 * smoothstep(0.8, 1.0, s));
                    let tf = smoothstep(-0.15, 0.35, tuft.get(tseed + s * 3.0, tseed));
                    e *= lerp(1.0, tf, habit.clumpy);
                    if broken {
                        e *= 1.0 - smoothstep(end - 0.15, end, s) * 0.6;
                    }
                    pad.push((0.28 * pad_d * e, pad_d * e));
                }
            }
            let tip = *pts.last().unwrap();
            Some(Bough { side: if tip.0 < ax { -1.0 } else { 1.0 }, pts, w, pad, z: l * sz, t, dead, broken, minor })
        };

        let mut y = apex.1 + sp * rng.range(0.25, 0.5);
        let mut year = 0;
        while y < y_bot {
            let t = (y - apex.1) / hc;
            let pmiss = habit.gap * (0.4 + 1.8 * smoothstep(0.0, 0.45, gapz.get(t * 3.0, 1.0)));
            let n = rng.range(habit.per_whorl.0 as f32, habit.per_whorl.1 as f32 + 1.0) as u32;
            let phi0 = rng.range(0.0, TAU);
            for k in 0..n {
                if rng.chance(pmiss) {
                    continue;
                }
                let phi = phi0 + k as f32 * TAU / n as f32 + rng.normal() * 0.3;
                let yy = y + rng.normal() * sp * 0.05;
                if let Some(b) = bough(&mut rng, yy, phi, 1.0, false, false) {
                    boughs.push(b);
                }
            }
            let ni = (habit.inter + rng.f()) as u32;
            for _ in 0..ni {
                if rng.chance(pmiss) {
                    continue;
                }
                let yy = y + rng.range(0.2, 0.9) * sp;
                let (phi, sc) = (rng.range(0.0, TAU), rng.range(0.3, 0.6));
                if yy < y_bot && let Some(b) = bough(&mut rng, yy, phi, sc, true, false) {
                    boughs.push(b);
                }
            }
            year += 1;
            y += sp * rng.range(0.72, 1.28) * (1.0 + 0.2 * (t - 0.5));
        }
        let _ = year;
        // stubs on the bare trunk under the crown
        let mut yb = y_bot + sp * rng.range(0.3, 1.0);
        while yb < foot.1 - sp * 0.5 {
            let phi = rng.range(0.0, TAU);
            if rng.chance(0.55)
                && let Some(b) = bough(&mut rng, yb, phi, 1.0, true, true)
            {
                boughs.push(b);
            }
            yb += sp * rng.range(0.5, 1.4);
        }
        boughs.sort_by(|a, b| a.z.total_cmp(&b.z));

        let hatch = (sp * 0.17).clamp(0.45, 3.0);
        let mut fir = Fir {
            leader,
            leader_w,
            leader_dead_from,
            boughs,
            strokes: vec![],
            apex,
            foot,
            crown_base: y_bot,
            tier: sp,
            hatch,
            envelope: env,
            sun,
            seed,
        };
        fir.strokes = fir.lay_strokes(&mut rng);
        fir
    }

    /// How much a point on a bough faces the light.
    fn light(&self, b: &Bough, s: f32, top: bool, q: (f32, f32)) -> f32 {
        let (sx, sy, sz) = self.sun;
        let ax = x_at_y(&self.leader, q.1);
        let r = span_at(&self.envelope, q.1, ax).map(|(l, r)| (r - l) * 0.5).unwrap_or(1.0).max(1.0);
        let u = ((q.0 - ax) / r).clamp(-1.2, 1.2);
        let d = (sx * sx + sy * sy + sz * sz).sqrt().max(1e-6);
        let (sx, sy, sz) = (sx / d, sy / d, sz / d);
        let side = u * sx * 1.4;
        let front = (b.z / (r + 1.0)).clamp(-1.0, 1.0) * sz * 0.5;
        let upper = if top { -sy * 0.35 } else { -0.05 };
        let high = (0.5 - b.t) * 0.3;
        let out = (s - 0.5) * 0.35;
        (0.42 + 0.45 * side + front + upper + high + out).clamp(0.0, 1.0)
    }

    fn lay_strokes(&self, rng: &mut Rng) -> Vec<Hatch> {
        let mut out = Vec::new();
        let hw = self.hatch;
        for (bi, b) in self.boughs.iter().enumerate() {
            let n = b.pts.len();
            let seg: Vec<f32> = b.pts.windows(2).map(|p| ((p[1].0 - p[0].0).powi(2) + (p[1].1 - p[0].1).powi(2)).sqrt()).collect();
            let len: f32 = seg.iter().sum();
            if len < 0.5 {
                continue;
            }
            if b.dead {
                // a few twigs off a dead bough
                let k = (len / (self.tier * 0.6)).ceil() as usize;
                for _ in 0..k {
                    let s = rng.range(0.25, 0.95);
                    let i = ((s * (n - 1) as f32) as usize).min(n - 2);
                    let p = b.pts[i];
                    let dir = norm2((b.pts[i + 1].0 - p.0, b.pts[i + 1].1 - p.1));
                    let ang = rng.range(0.4, 0.9) * if rng.chance(0.6) { 1.0 } else { -1.0 };
                    let (c, sn) = (ang.cos(), ang.sin());
                    let d2 = (dir.0 * c - dir.1 * sn, dir.0 * sn + dir.1 * c);
                    let l = self.tier * rng.range(0.25, 0.6);
                    let e = (p.0 + d2.0 * l, p.1 + d2.1 * l + l * 0.2);
                    out.push(Hatch { pts: [p, ((p.0 + e.0) * 0.5, (p.1 + e.1) * 0.5), e], w: hw * 0.35, lit: 0.3, z: b.z, bough: bi, kind: Kind::Twig });
                }
                continue;
            }
            // walk the bough at hatch spacing
            let step = hw * rng.range(0.9, 1.25);
            let mut at = rng.range(0.0, step);
            let mut i = 0;
            let mut acc = 0.0;
            while at < len {
                while i < seg.len() - 1 && acc + seg[i] < at {
                    acc += seg[i];
                    i += 1;
                }
                let f = ((at - acc) / seg[i].max(1e-6)).clamp(0.0, 1.0);
                let (a, c) = (b.pts[i], b.pts[i + 1]);
                let p = (lerp(a.0, c.0, f), lerp(a.1, c.1, f));
                let pa = lerp(b.pad[i].0, b.pad[i + 1].0, f);
                let pb = lerp(b.pad[i].1, b.pad[i + 1].1, f);
                let s = at / len;
                at += step * rng.range(0.7, 1.3);
                if pb < hw * 0.6 || rng.chance(0.08) {
                    continue;
                }
                let dir = norm2((c.0 - a.0, c.1 - a.1));
                // up normal (y down: the one with negative y)
                let nu = if dir.0 >= 0.0 { (dir.1, -dir.0) } else { (-dir.1, dir.0) };
                let start = (p.0 + nu.0 * pa * rng.range(0.2, 1.0), p.1 + nu.1 * pa * rng.range(0.2, 1.0));
                // pendant shoots hang down and outward
                let k = rng.range(0.4, 0.78);
                let hang = norm2((lerp(dir.0, 0.0, k) + rng.normal() * 0.08, lerp(dir.1, 1.0, k)));
                let l = (pa + pb) * rng.range(0.7, 1.15);
                let bend = rng.normal() * 0.12 * l;
                let perp = (-hang.1, hang.0);
                let mid = (start.0 + hang.0 * l * 0.5 + perp.0 * bend, start.1 + hang.1 * l * 0.5 + perp.1 * bend);
                let end = (start.0 + hang.0 * l, start.1 + hang.1 * l);
                let lit = (self.light(b, s, false, start) + rng.normal() * 0.07).clamp(0.0, 1.0);
                out.push(Hatch { pts: [start, mid, end], w: hw, lit, z: b.z, bough: bi, kind: Kind::Under });
                // now and then a stroke along the upper face
                if pa > hw * 0.3 && rng.chance(0.3) {
                    let l2 = self.tier * rng.range(0.25, 0.5);
                    let d2 = norm2((dir.0, dir.1 - rng.range(0.0, 0.25)));
                    let s0 = (p.0 + nu.0 * pa, p.1 + nu.1 * pa);
                    let e2 = (s0.0 + d2.0 * l2, s0.1 + d2.1 * l2);
                    let lit = (self.light(b, s, true, s0) + rng.normal() * 0.07).clamp(0.0, 1.0);
                    out.push(Hatch { pts: [s0, ((s0.0 + e2.0) * 0.5, (s0.1 + e2.1) * 0.5 - l2 * 0.04), e2], w: hw * 0.8, lit, z: b.z, bough: bi, kind: Kind::Top });
                }
            }
        }
        out
    }

    /// Shape of the needle pads (core) and the hanging strokes.
    fn needle_shape(&self, mut shape: Shape) -> Shape {
        for b in self.boughs.iter().filter(|b| !b.dead) {
            let mut up = Vec::new();
            let mut down = Vec::new();
            let n = b.pts.len();
            for k in 0..n {
                let (a, c) = (b.pts[k.saturating_sub(1)], b.pts[(k + 1).min(n - 1)]);
                let dir = norm2((c.0 - a.0, c.1 - a.1));
                let nu = if dir.0 >= 0.0 { (dir.1, -dir.0) } else { (-dir.1, dir.0) };
                let (pa, pb) = b.pad[k];
                let p = b.pts[k];
                up.push((p.0 + nu.0 * pa, p.1 + nu.1 * pa));
                down.push((p.0 + dir.0 * pb * 0.15, p.1 + pb * 0.78));
            }
            if b.pad.iter().all(|p| p.1 < 0.05) {
                continue;
            }
            down.reverse();
            up.extend(down);
            shape = shape.add(Shape::new().poly(&up));
        }
        for s in self.strokes.iter().filter(|s| s.kind != Kind::Twig) {
            shape = shape.ribbon(&s.pts, &[s.w * 0.9, s.w * 0.75, s.w * 0.2]);
        }
        shape
    }

    fn wood_shape(&self, mut shape: Shape, from: usize) -> Shape {
        let top = self.leader.len();
        shape = shape.ribbon(&self.leader[from.min(top - 2)..], &self.leader_w[from.min(top - 2)..]);
        for b in &self.boughs {
            shape = shape.ribbon(&b.pts, &b.w);
        }
        shape
    }

    /// Where the needles are.
    pub fn needles(&self, f: Frame) -> Mask {
        Mask::from_shape(f, self.needle_shape(Shape::new()))
    }

    /// The leader and every bough (the wood, needles or not).
    pub fn wood(&self, f: Frame) -> Mask {
        Mask::from_shape(f, self.wood_shape(Shape::new(), 0))
    }

    /// The bare trunk under the crown (what shows at a wood's foot).
    pub fn trunk(&self, f: Frame) -> Mask {
        let pts: Vec<(f32, f32)> = self.leader.iter().copied().filter(|p| p.1 >= self.crown_base - self.tier).collect();
        let w: Vec<f32> = self.leader.iter().zip(&self.leader_w).filter(|(p, _)| p.1 >= self.crown_base - self.tier).map(|(_, w)| *w).collect();
        let mut shape = Shape::new();
        if pts.len() >= 2 {
            shape = shape.ribbon(&pts, &w);
        }
        for b in self.boughs.iter().filter(|b| b.dead && b.t > 1.0) {
            shape = shape.ribbon(&b.pts, &b.w);
        }
        Mask::from_shape(f, shape)
    }

    /// Needles facing the light: the strokes whose `lit` is at least `from`.
    pub fn lit(&self, f: Frame, from: f32) -> Mask {
        let mut shape = Shape::new();
        for s in self.strokes.iter().filter(|s| s.kind != Kind::Twig && s.lit >= from) {
            shape = shape.ribbon(&s.pts, &[s.w * 1.3, s.w * 1.1, s.w * 0.4]);
        }
        Mask::from_shape(f, shape).mul(&self.needles(f))
    }

    /// The whole tree: needles, boughs and trunk.
    pub fn mask(&self, f: Frame) -> Mask {
        Mask::from_shape(f, self.wood_shape(self.needle_shape(Shape::new()), 0))
    }

    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        let mut b = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        let mut add = |p: (f32, f32), r: f32| b = (b.0.min(p.0 - r), b.1.min(p.1 - r), b.2.max(p.0 + r), b.3.max(p.1 + r));
        for p in &self.leader {
            add(*p, 1.0);
        }
        for bo in &self.boughs {
            for (p, pd) in bo.pts.iter().zip(&bo.pad) {
                add(*p, pd.1);
            }
        }
        b
    }
}

/// A made-up envelope for a fir of a habit: apex, crown base and half width
/// (for woods, where nobody draws every tree).
pub fn envelope_for(habit: &str, apex: (f32, f32), base: f32, halfw: f32, seed: u64) -> Vec<(f32, f32)> {
    let mut rng = Rng::new(seed ^ 0xE57E);
    let n = 16;
    let h = (base - apex.1).max(1.0);
    let lean = rng.normal() * 0.02 * h;
    let (pw, bulge) = match habit {
        "young" => (0.85, 0.05),
        "old" | "ragged" => (0.6, 0.12),
        "storm" => (0.7, 0.08),
        _ => (0.9, 0.0),
    };
    let rough = Fbm::new(seed as u32 ^ 0x5eed, 2, 0.3);
    let side = |sgn: f32, rng: &mut Rng| -> Vec<(f32, f32)> {
        let asym = 1.0 + rng.normal() * 0.1;
        (0..=n)
            .map(|i| {
                let t = i as f32 / n as f32;
                let r = halfw * asym * (t.powf(pw) + bulge * (PI * t).sin()) * (1.0 + 0.18 * rough.get(t * 3.0, sgn * 5.0)) * (1.0 - 0.15 * smoothstep(0.92, 1.0, t));
                (apex.0 + lean * (1.0 - t) + sgn * r.max(0.0), apex.1 + t * h)
            })
            .collect()
    };
    let right = side(1.0, &mut rng);
    let mut left = side(-1.0, &mut rng);
    left.reverse();
    let mut pts = right;
    pts.extend(left);
    pts.pop(); // the apex again
    pts
}

/// A fir wood seen from outside: rows receding behind a drawn skyline.
#[derive(Clone, Debug)]
pub struct Wood {
    /// Front row first.
    pub rows: Vec<Row>,
    pub skyline: Vec<(f32, f32)>,
    pub foot: Vec<(f32, f32)>,
    pub horizon: f32,
}

#[derive(Clone, Debug)]
pub struct Row {
    pub trees: Vec<Fir>,
    /// Apparent size relative to the front row.
    pub scale: f32,
    /// Share of air between this row and the eye (0 front .. ~0.7).
    pub haze: f32,
    /// Where this row's trees stand.
    pub foot: Vec<(f32, f32)>,
}

/// Options for a wood.
#[derive(Clone, Debug)]
pub struct WoodSpec {
    pub rows: usize,
    /// Trees in the front row.
    pub count: usize,
    pub horizon: Option<f32>,
    /// Each row back is this much farther (1 + r * recede).
    pub recede: f32,
    /// Air per row (haze = 1 - exp(-r * air)).
    pub air: f32,
    /// Crown width as a fraction of height (min, max).
    pub width: (f32, f32),
    /// Habit mix (weights): spire, old, young.
    pub mix: (f32, f32, f32),
    /// Bare trunk under the crown, fraction of height (min, max).
    pub bare: (f32, f32),
    pub sun: (f32, f32, f32),
}

impl Default for WoodSpec {
    fn default() -> Self {
        WoodSpec { rows: 4, count: 12, horizon: None, recede: 0.5, air: 0.4, width: (0.2, 0.32), mix: (0.7, 0.18, 0.12), bare: (0.06, 0.18), sun: (-0.55, -0.75, 0.35) }
    }
}

fn along(line: &[(f32, f32)], x: f32) -> f32 {
    if line.len() == 1 {
        return line[0].1;
    }
    for w in line.windows(2) {
        let (a, b) = if w[0].0 <= w[1].0 { (w[0], w[1]) } else { (w[1], w[0]) };
        if x >= a.0 && x <= b.0 && b.0 > a.0 {
            return a.1 + (x - a.0) / (b.0 - a.0) * (b.1 - a.1);
        }
    }
    let (f, l) = (line[0], line[line.len() - 1]);
    if (x - f.0).abs() < (x - l.0).abs() { f.1 } else { l.1 }
}

impl Wood {
    /// Grow a wood: the front row's tops on `skyline` (x ascending), its
    /// feet on `foot`, and `spec.rows - 1` rows behind, smaller, standing
    /// higher toward the horizon and hazier.
    pub fn grow(skyline: &[(f32, f32)], foot: &[(f32, f32)], spec: &WoodSpec, seed: u64) -> Wood {
        let mut rng = Rng::new(seed ^ 0x3001);
        let x0 = skyline.iter().map(|p| p.0).fold(f32::MAX, f32::min);
        let x1 = skyline.iter().map(|p| p.0).fold(f32::MIN, f32::max);
        let mean_h = {
            let k = 16;
            (0..k).map(|i| {
                let x = lerp(x0, x1, (i as f32 + 0.5) / k as f32);
                along(foot, x) - along(skyline, x)
            }).sum::<f32>() / k as f32
        }
        .max(4.0);
        let foot_mean = foot.iter().map(|p| p.1).sum::<f32>() / foot.len() as f32;
        let horizon = spec.horizon.unwrap_or(foot_mean - 0.12 * mean_h);
        let mut rows = Vec::new();
        for r in 0..spec.rows.max(1) {
            let d = 1.0 + r as f32 * spec.recede;
            let s = 1.0 / d;
            let haze = 1.0 - (-(r as f32) * spec.air).exp();
            let n = ((spec.count as f32) * (1.0 + r as f32 * 0.55)).round().max(1.0) as usize;
            let pad = (x1 - x0) * 0.02;
            let xs = crate::noise::uneven(n, x0 + pad, x1 - pad, 0.6, 0.45, (seed as u32).wrapping_add(r as u32 * 7919));
            let row_foot: Vec<(f32, f32)> = (0..=24).map(|i| {
                let x = lerp(x0, x1, i as f32 / 24.0);
                (x, horizon + (along(foot, x) - horizon) * s)
            }).collect();
            let mut trees = Vec::new();
            for (k, &x) in xs.iter().enumerate() {
                let x = x + rng.normal() * (x1 - x0) / n as f32 * 0.12;
                let ft = along(&row_foot, x);
                let h_front = along(foot, x) - along(skyline, x);
                let mut h = h_front * s;
                // the front row reaches the drawn line (a few fall short, one pokes above)
                h *= if r == 0 {
                    let u = rng.f();
                    if u < 0.1 { rng.range(1.04, 1.12) } else if u < 0.35 { rng.range(0.78, 0.94) } else { rng.range(0.94, 1.02) }
                } else {
                    rng.range(0.78, 1.18)
                };
                if h < 3.0 {
                    continue;
                }
                let u = rng.f() * (spec.mix.0 + spec.mix.1 + spec.mix.2);
                let hab = if u < spec.mix.0 { "spire" } else if u < spec.mix.0 + spec.mix.1 { "old" } else { "young" };
                let mut habit = FirHabit::named(hab).unwrap();
                // small trees are seen with fewer tiers
                habit.tiers = (habit.tiers * (h / 220.0).clamp(0.3, 1.0)).max(6.0);
                let bare = if hab == "young" { rng.range(0.0, 0.04) } else { rng.range(spec.bare.0, spec.bare.1) };
                let apex = (x, ft - h);
                let base = ft - h * bare;
                let halfw = 0.5 * h * rng.range(spec.width.0, spec.width.1);
                let tseed = seed.wrapping_mul(31).wrapping_add((r * 1000 + k) as u64);
                let env = envelope_for(hab, apex, base, halfw, tseed);
                trees.push(Fir::grow(&env, Some((x + rng.normal() * halfw * 0.05, ft)), &habit, spec.sun, tseed));
            }
            rows.push(Row { trees, scale: s, haze, foot: row_foot });
        }
        Wood { rows, skyline: skyline.to_vec(), foot: foot.to_vec(), horizon }
    }

    /// Needles of row `r`.
    pub fn needles(&self, f: Frame, r: usize) -> Mask {
        let mut shape = Shape::new();
        for t in &self.rows[r].trees {
            shape = t.needle_shape(shape);
        }
        Mask::from_shape(f, shape)
    }

    /// Trunks and boughs of row `r` (the wood).
    pub fn wood(&self, f: Frame, r: usize) -> Mask {
        let mut shape = Shape::new();
        for t in &self.rows[r].trees {
            shape = t.wood_shape(shape, 0);
        }
        Mask::from_shape(f, shape)
    }

    /// Needles of row `r` facing the light.
    pub fn lit(&self, f: Frame, r: usize, from: f32) -> Mask {
        let mut shape = Shape::new();
        for t in &self.rows[r].trees {
            for s in t.strokes.iter().filter(|s| s.kind != Kind::Twig && s.lit >= from) {
                shape = shape.ribbon(&s.pts, &[s.w * 1.3, s.w * 1.1, s.w * 0.4]);
            }
        }
        Mask::from_shape(f, shape).mul(&self.needles(f, r))
    }

    /// Everything of row `r` (needles and wood).
    pub fn all(&self, f: Frame, r: usize) -> Mask {
        let mut shape = Shape::new();
        for t in &self.rows[r].trees {
            shape = t.wood_shape(t.needle_shape(shape), 0);
        }
        Mask::from_shape(f, shape)
    }

    /// The floor of the wood: from the front row's feet up to its lowest
    /// needles, where trunks stand in the dark (and, higher, the interior
    /// between the back rows' crowns).
    pub fn floor(&self, f: Frame) -> Mask {
        let front = &self.rows[0];
        let x0 = self.skyline.iter().map(|p| p.0).fold(f32::MAX, f32::min);
        let x1 = self.skyline.iter().map(|p| p.0).fold(f32::MIN, f32::max);
        let back_foot = &self.rows[self.rows.len() - 1].foot;
        let mut top = Vec::new();
        let mut bot = Vec::new();
        let k = 48;
        for i in 0..=k {
            let x = lerp(x0, x1, i as f32 / k as f32);
            // the lowest crown base near x, or the back row's foot
            let mut cb = along(back_foot, x) - 0.35 * (along(&self.foot, x) - along(&self.skyline, x)) * self.rows[self.rows.len() - 1].scale;
            for t in &front.trees {
                let (bx0, _, bx1, _) = t.bounds();
                if x >= bx0 && x <= bx1 {
                    cb = cb.max(t.crown_base - t.tier);
                }
            }
            top.push((x, cb.min(along(&self.foot, x) - 1.0)));
            bot.push((x, along(&self.foot, x) + 1.0));
        }
        bot.reverse();
        top.extend(bot);
        Mask::from_shape(f, Shape::new().poly(&top))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spire_env() -> Vec<(f32, f32)> {
        vec![(500.0, 100.0), (530.0, 250.0), (570.0, 420.0), (500.0, 440.0), (430.0, 420.0), (470.0, 250.0)]
    }

    #[test]
    fn boughs_end_inside_the_envelope() {
        let env = spire_env();
        let f = Fir::grow(&env, None, &FirHabit::spire(), (-0.5, -0.8, 0.3), 3);
        assert!(f.boughs.len() > 40, "{}", f.boughs.len());
        let mut out = 0;
        for b in f.boughs.iter().filter(|b| !b.minor || !b.dead) {
            let tip = *b.pts.last().unwrap();
            let inside = spans(&env, tip.1).iter().any(|s| tip.0 >= s.0 - 4.0 && tip.0 <= s.1 + 4.0);
            if !inside && tip.1 < 440.0 {
                out += 1;
            }
        }
        assert!(out * 20 < f.boughs.len(), "{out} of {} tips outside", f.boughs.len());
        assert!(f.strokes.len() > 300, "{}", f.strokes.len());
        // it reaches the apex and fills the envelope's width
        let (x0, y0, x1, _) = f.bounds();
        assert!(y0 < 112.0 && x0 < 445.0 && x1 > 555.0, "{:?}", f.bounds());
    }

    #[test]
    fn deterministic_and_seeds_differ() {
        let env = spire_env();
        let a = Fir::grow(&env, None, &FirHabit::old(), (-0.5, -0.8, 0.3), 5);
        let b = Fir::grow(&env, None, &FirHabit::old(), (-0.5, -0.8, 0.3), 5);
        let c = Fir::grow(&env, None, &FirHabit::old(), (-0.5, -0.8, 0.3), 6);
        assert_eq!(a.strokes.len(), b.strokes.len());
        assert_eq!(a.boughs[3].pts, b.boughs[3].pts);
        assert_ne!(a.strokes.len(), c.strokes.len());
        // an old fir has dead and broken boughs; a young one doesn't
        assert!(a.boughs.iter().any(|b| b.dead) && a.boughs.iter().any(|b| b.broken));
        let y = Fir::grow(&env, None, &FirHabit::young(), (-0.5, -0.8, 0.3), 5);
        assert!(!y.boughs.iter().any(|b| b.dead && b.t <= 1.0));
    }

    #[test]
    fn a_wood_recedes() {
        let sky = vec![(0.0, 120.0), (300.0, 100.0), (600.0, 180.0)];
        let foot = vec![(0.0, 380.0), (600.0, 380.0)];
        let w = Wood::grow(&sky, &foot, &WoodSpec::default(), 9);
        assert_eq!(w.rows.len(), 4);
        let mean_h = |r: &Row| r.trees.iter().map(|t| t.foot.1 - t.apex.1).sum::<f32>() / r.trees.len() as f32;
        for k in 1..4 {
            assert!(mean_h(&w.rows[k]) < mean_h(&w.rows[k - 1]), "row {k}");
            assert!(w.rows[k].haze > w.rows[k - 1].haze);
            assert!(w.rows[k].trees.len() >= w.rows[k - 1].trees.len());
        }
        // front trees stand on the foot line and reach near the skyline
        let t = &w.rows[0].trees[0];
        assert!((t.foot.1 - 380.0).abs() < 1.0);
    }
}
