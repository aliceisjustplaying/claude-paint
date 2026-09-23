//! Trees as this painter paints them. The engine grows the wood
//! (`paint::Habit::grow` returns a skeleton of limbs); how it goes on the
//! canvas is the painter's habit, written here:
//!
//! - each limb is followed from where it springs to its tip in one
//!   continuous movement; where it thins past what one brush can do, the
//!   next smaller brush picks the stroke up wet, a little before the last
//!   one lifted, so there is no joint;
//! - trunk first, then main limbs, then their branches, then twigs: every
//!   branch is pulled out of paint that is already there;
//! - round sables for the wood, a rigger for twigs; pressure follows the
//!   limb's width so it tapers continuously; a live tip is lifted off to a
//!   point, a broken end stops blunt and is splintered with a few flicks;
//! - when the dark has dried, light: a few lean dry-brush streaks along the
//!   side of each big limb that faces the light.
//!
//! Evergreens (the spruce as Friedrich places it) are at the end.

use paint::{Canvas, Gesture, Hand, Held, Limb, Mark, Mask, Orient, Paint, Rng, Skeleton, Tool};

/// The paints for a tree, mixed by the painter from their palette (e.g.
/// `palette.mix(hex("#1f1a16")).paint(0.1)`).
#[derive(Clone, Copy, Debug)]
pub struct Bark {
    /// The wood's dark body color.
    pub dark: Paint,
    /// Dead wood, if it should differ (weathered grey).
    pub dead: Option<Paint>,
    /// Lean light for the lit side of the big limbs (dry-brushed).
    pub light: Option<Paint>,
    /// Pale fresh wood showing at a break.
    pub wood: Option<Paint>,
}

impl Bark {
    pub fn dark(dark: Paint) -> Self {
        Bark { dark, dead: None, light: None, wood: None }
    }
}

/// How the tree is painted.
#[derive(Clone, Copy, Debug)]
pub struct TreeHand {
    /// Where the light comes from (canvas direction, e.g. (-0.75, -0.66) =
    /// upper left).
    pub light_from: (f32, f32),
    /// Thinnest stroke the painter will make (units); finer twigs are
    /// painted at this width, lighter.
    pub finest: f32,
    /// Limbs wider than this get lit streaks.
    pub lit_above: f32,
}

impl Default for TreeHand {
    fn default() -> Self {
        TreeHand { light_from: (-0.75, -0.66), finest: 0.35, lit_above: 2.0 }
    }
}

/// Paint a grown tree. Returns its silhouette mask.
pub fn tree(c: &mut Canvas, sk: &Skeleton, bark: &Bark, hand: &TreeHand, seed: u64) -> Mask {
    let mut rng = Rng::new(seed);
    for l in sk.limbs.iter().filter(|l| !l.is_empty()) {
        let p = if l.dead { bark.dead.unwrap_or(bark.dark) } else { bark.dark };
        limb(c, l, p, hand, &mut rng);
    }
    // splinters at the breaks, then (once the dark is dry) the light
    for l in sk.limbs.iter().filter(|l| l.broken && !l.is_empty()) {
        splinter(c, l, bark, &mut rng);
    }
    let mask = sk.mask(c.frame());
    if let Some(lp) = bark.light {
        c.dry();
        for l in sk.limbs.iter().filter(|l| !l.is_empty() && l.w[0] > hand.lit_above && !l.root) {
            lit_side(c, l, lp, hand, &mask, &mut rng);
        }
    }
    mask
}

/// Footprint width of a round brush at pressure `p`, relative to full
/// pressure (see `bristle::drag_on`: half = w/2·(0.45+0.55p)·(1+splay(p−½))).
fn spread(p: f32, splay: f32) -> f32 {
    (0.45 + 0.55 * p) * (1.0 + splay * (p - 0.5))
}

/// Pressure that gives `ratio` of the full-pressure width.
fn pressure_for(ratio: f32, splay: f32) -> f32 {
    let target = ratio.clamp(0.0, 1.0) * spread(1.0, splay);
    let (mut lo, mut hi) = (0.0f32, 1.0f32);
    for _ in 0..20 {
        let m = 0.5 * (lo + hi);
        if spread(m, splay) < target { lo = m } else { hi = m }
    }
    0.5 * (lo + hi)
}

/// A brush for a mark `w` units wide at full pressure: round sable for the
/// wood, rigger for twigs.
fn brush_for(w: f32) -> Tool {
    if w > 1.6 {
        let t = Tool::round_sable(1.0);
        Tool { ragged: 0.3, ..Tool::round_sable(w / spread(1.0, t.splay)) }
    } else {
        let t = Tool::rigger(1.0);
        let tw = w / spread(1.0, t.splay);
        Tool { length: tw * 3.0, ..Tool::rigger(tw) }
    }
}

/// One limb, base to tip, in as many brushes as its taper needs.
fn limb(c: &mut Canvas, l: &Limb, paint: Paint, hand: &TreeHand, rng: &mut Rng) {
    let n = l.pts.len();
    let w: Vec<f32> = l.w.iter().map(|w| w.max(hand.finest)).collect();
    let arc = cumulative(&l.pts);
    // sections: each brush covers widths down to about 40% of its own, and
    // no longer than its load lasts (a starved brush skips over the weave)
    let mut cuts = vec![0usize];
    for i in 1..n {
        let a = *cuts.last().unwrap();
        let run = brush_for(w[a]).run * 0.7;
        if (w[i] < w[a] * 0.4 || arc[i] - arc[a] > run) && i + 1 < n {
            cuts.push(i);
        }
    }
    cuts.push(n - 1);
    for k in 0..cuts.len() - 1 {
        let (a, b) = (cuts[k], cuts[k + 1]);
        let first = k == 0;
        let last = k + 2 == cuts.len();
        // start early and end late (a limb's width or more) so the strokes
        // overlap wet and the change of brush does not show
        let ov = w[a] * 1.5;
        let a0 = if first { a } else { (0..a).rev().find(|&i| arc[a] - arc[i] >= ov).unwrap_or(0) };
        let b0 = if last { b } else { (b + 1..n).find(|&i| arc[i] - arc[b] >= ov).unwrap_or(n - 1) };
        let pts: Vec<(f32, f32)> = l.pts[a0..=b0].to_vec();
        if pts.len() < 2 {
            continue;
        }
        let tool = brush_for(w[a]);
        let splay = tool.splay;
        let total = (arc[b0] - arc[a0]).max(1e-3);
        // the next brush sets down at full pressure inside the wider wet
        // stroke (a soft attack there would lift paint, not lay it)
        let attack = 0.0;
        // pressure from the width at each end (linear between is close
        // enough within one section); a live tip is lifted off to a point
        let p0 = 1.0;
        let p1 = pressure_for(w[b0] / w[a], splay).max(0.05);
        let release = if last {
            if l.broken { (w[b] * 0.3 / total).clamp(0.0, 0.2) } else { (w[a].min(6.0) * 2.5 / total).clamp(0.15, 0.6) }
        } else {
            ((arc[b0] - arc[b]) / total).clamp(0.02, 0.5)
        };
        let mut held = Held::new(tool, rng.next_u64());
        // the finest twigs are only indicated: a lean, dry touch, so the
        // outer crown reads as a haze of twigs rather than a solid mass
        let thin = (l.w[a] / (2.0 * hand.finest)).clamp(0.2, 1.0);
        held.load(paint.with_hiding(paint.hiding() * (0.4 + 0.6 * thin)), 0.9 * thin.sqrt());
        let g = Gesture::new(pts).pressure(p0, p1).ramps(attack, release).orient(Orient::Across).shake(0.6);
        c.drag(&mut held, &g, None);
    }
}

fn cumulative(pts: &[(f32, f32)]) -> Vec<f32> {
    let mut arc = vec![0.0f32; pts.len()];
    for i in 1..pts.len() {
        arc[i] = arc[i - 1] + ((pts[i].0 - pts[i - 1].0).powi(2) + (pts[i].1 - pts[i - 1].1).powi(2)).sqrt();
    }
    arc
}

/// A break: the end stops blunt and splits into a few splinters, one longer
/// than the rest, with a touch of pale wood on the break if given.
fn splinter(c: &mut Canvas, l: &Limb, bark: &Bark, rng: &mut Rng) {
    let n = l.pts.len();
    let end = l.pts[n - 1];
    let d = l.dir(n - 1);
    let nrm = (-d.1, d.0);
    let w = l.w[n - 1];
    if w < 0.8 {
        return;
    }
    let paint = if l.dead { bark.dead.unwrap_or(bark.dark) } else { bark.dark };
    let k = 3 + (rng.f() * 2.0) as usize;
    let long = (rng.f() * k as f32) as usize;
    for i in 0..k {
        let off = (i as f32 / (k - 1) as f32 - 0.5) * w * 0.75 + rng.normal() * w * 0.08;
        let len = w * if i == long { rng.range(1.0, 2.2) } else { rng.range(0.3, 0.9) };
        let bend = rng.normal() * 0.25;
        let s = (end.0 - d.0 * w * 0.6 + nrm.0 * off, end.1 - d.1 * w * 0.6 + nrm.1 * off);
        let dd = (d.0 + nrm.0 * bend, d.1 + nrm.1 * bend);
        let e = (s.0 + dd.0 * len, s.1 + dd.1 * len);
        let m = ((s.0 + e.0) * 0.5 + nrm.0 * rng.normal() * w * 0.1, (s.1 + e.1) * 0.5 + nrm.1 * rng.normal() * w * 0.1);
        let tw = (w * rng.range(0.25, 0.4)).max(0.35);
        let mut held = Held::new(brush_for(tw), rng.next_u64());
        held.load(paint, 0.8);
        c.drag(&mut held, &Gesture::new(vec![s, m, e]).pressure(0.9, 0.1).ramps(0.0, 0.7).orient(Orient::Across), None);
    }
    if let Some(wood) = bark.wood {
        // the torn face catches the light: one short touch across the end
        let mut held = Held::new(Tool::round_sable((w * 0.3).max(0.5)), rng.next_u64());
        held.load(wood, 0.5);
        let a = (end.0 - d.0 * w * 0.2 - nrm.0 * w * 0.3, end.1 - d.1 * w * 0.2 - nrm.1 * w * 0.3);
        let b = (end.0 - d.0 * w * 0.1 + nrm.0 * w * 0.25, end.1 - d.1 * w * 0.1 + nrm.1 * w * 0.25);
        c.drag(&mut held, &Gesture::new(vec![a, b]).pressure(0.6, 0.3).ramps(0.2, 0.5), None);
    }
}

/// Model a big limb as a lit cylinder: a few thin lean streaks along it on
/// the side facing the light, stepping from half-tone near the axis to the
/// lightest near the edge; light pressure so the canvas tooth and the bark
/// ridges break them up, each covering a random stretch of the limb.
fn lit_side(c: &mut Canvas, l: &Limb, lp: Paint, hand: &TreeHand, mask: &Mask, rng: &mut Rng) {
    let ld = hand.light_from;
    let n = l.pts.len();
    let normals: Vec<(f32, f32)> = (0..n)
        .map(|i| {
            let d = l.dir(i);
            let nn = (-d.1, d.0);
            if nn.0 * ld.0 + nn.1 * ld.1 >= 0.0 { nn } else { (-nn.0, -nn.1) }
        })
        .collect();
    let facing = normals.iter().map(|nn| (nn.0 * ld.0 + nn.1 * ld.1).max(0.0)).sum::<f32>() / n as f32;
    let streaks = 2 + (l.w[0] / 3.0).min(3.0) as usize;
    for k in 0..streaks {
        let u = 0.12 + 0.28 * (k as f32 + rng.range(0.0, 0.8)) / streaks as f32;
        let amount = (0.25 + 0.75 * (k + 1) as f32 / streaks as f32) * (0.3 + 0.7 * facing);
        // only where the limb is still wide enough to show a lit side
        let pts: Vec<(f32, f32)> = (0..n)
            .take_while(|&i| l.w[i] > hand.lit_above * 0.6)
            .map(|i| (l.pts[i].0 + normals[i].0 * l.w[i] * u, l.pts[i].1 + normals[i].1 * l.w[i] * u))
            .collect();
        if pts.len() < 2 {
            continue;
        }
        let a = rng.range(0.0, 0.35);
        let b = (a + rng.range(0.4, 0.95)).min(1.0);
        let sub = sub_path(&pts, a, b);
        let tool = Tool { ragged: 0.6, ..Tool::round_sable((l.w[0] * rng.range(0.1, 0.16)).max(0.5)) };
        let mut held = Held::new(tool, rng.next_u64());
        held.load(lp, rng.range(0.15, 0.3) * amount);
        let g = Gesture::new(sub).pressure(rng.range(0.3, 0.5), rng.range(0.15, 0.3)).ramps(0.2, 0.4).orient(Orient::Across);
        c.drag(&mut held, &g, Some(mask));
    }
}

/// The part of a polyline between arc-length fractions `a` and `b`.
fn sub_path(pts: &[(f32, f32)], a: f32, b: f32) -> Vec<(f32, f32)> {
    let arc = cumulative(pts);
    let total = *arc.last().unwrap();
    let at = |u: f32| -> (usize, (f32, f32)) {
        let t = u * total;
        for i in 0..pts.len() - 1 {
            if arc[i + 1] >= t {
                let f = ((t - arc[i]) / (arc[i + 1] - arc[i]).max(1e-6)).clamp(0.0, 1.0);
                return (i, (pts[i].0 + (pts[i + 1].0 - pts[i].0) * f, pts[i].1 + (pts[i + 1].1 - pts[i].1) * f));
            }
        }
        (pts.len() - 2, pts[pts.len() - 1])
    };
    let (i0, p0) = at(a);
    let (i1, p1) = at(b);
    let mut out = vec![p0];
    out.extend_from_slice(&pts[i0 + 1..=i1]);
    out.push(p1);
    out
}

/// A grown spruce painted as Friedrich places them: the stem first, tapering
/// to the leader's point; then each limb from the stem outward, and its
/// needles as short strokes hanging from it, so each tier is a dark ragged
/// shelf drooping from the stem.
pub fn grown_spruce(c: &mut Canvas, sk: &Skeleton, needles: Paint, seed: u64) -> Mask {
    let mut rng = Rng::new(seed);
    let hand = TreeHand::default();
    for l in sk.limbs.iter().filter(|l| !l.is_empty()) {
        limb(c, l, needles, &hand, &mut rng);
    }
    // needles hang from every limb but the stem: short drooping strokes
    // along it, longer toward the stem where the shoots are older
    let h = sk.height;
    let nw = (h * 0.006).max(0.5);
    let mut b = Held::new(Tool::round_sable(nw), rng.next_u64());
    for l in sk.limbs.iter().filter(|l| l.order >= 1 && !l.is_empty()) {
        let arc = cumulative(&l.pts);
        let total = *arc.last().unwrap();
        let step = nw * 1.4;
        let count = (total / step) as usize;
        if count == 0 {
            continue;
        }
        b.reload(needles, 0.7);
        for k in 0..count {
            let t = (k as f32 + rng.f()) * step;
            let i = arc.iter().position(|&a| a >= t).unwrap_or(l.pts.len() - 1).max(1);
            let f = ((t - arc[i - 1]) / (arc[i] - arc[i - 1]).max(1e-6)).clamp(0.0, 1.0);
            let p = (l.pts[i - 1].0 + (l.pts[i].0 - l.pts[i - 1].0) * f, l.pts[i - 1].1 + (l.pts[i].1 - l.pts[i - 1].1) * f);
            let d = l.dir(i);
            let u = t / total;
            // shorter on short (young, high) limbs, so the top is a spire
            let len = (h * 0.022).min(total * 0.45) * (1.15 - 0.7 * u) * rng.range(0.6, 1.2) / (1.0 + 0.4 * (l.order - 1) as f32);
            // hang down and a little outward
            let out = d.0.signum();
            let e = (p.0 + out * len * 0.35 + rng.normal() * len * 0.1, p.1 + len);
            let m = (p.0 + out * len * 0.2, p.1 + len * 0.45);
            if k % 12 == 11 {
                b.reload(needles, 0.7);
            }
            c.drag(&mut b, &Gesture::new(vec![p, m, e]).pressure(0.8, 0.2).ramps(0.0, 0.6).orient(Orient::Across), None);
        }
    }
    sk.mask(c.frame())
}

/// A spruce written as brush gestures, for small and distant trees: a stem
/// tapering to a pointed leader, then tiers of drooping branch strokes from
/// the top down, each a little longer, ending in a true spire: the top tiers
/// are tiny touches with a small brush and the leader is lifted off to a
/// point. `base` is where the trunk meets the ground; `height` in units.
/// `paint` is mixed by the painter.
pub fn spruce(c: &mut Canvas, base: (f32, f32), height: f32, paint: Paint, seed: u64) {
    let mut h = Hand::new(base, height, seed);
    h.tremor = 0.0015;
    let p = paint;
    // the stem, lifted off toward the top
    let mut t = h.take(Tool::round_sable, 0.014, p, 0.6);
    h.mark(c, &mut t, Mark { pts: &[(0.0, 0.0), (0.002, 0.5), (0.0, 0.93)], pressure: (0.9, 0.2), ramps: (0.0, 0.4) }, None);
    // full-size tiers below, finer brushes as the spire narrows
    let mut b = h.take(Tool::round_sable, 0.026, p, 0.6);
    let mut fine = h.take(Tool::round_sable, 0.013, p, 0.5);
    let mut finest = h.take(Tool::rigger, 0.006, p, 0.5);
    let tiers = 22 + (height / 12.0).min(16.0) as usize;
    let slim = h.rng.range(0.17, 0.23);
    for i in 0..tiers {
        let v = 0.955 - (i as f32 + h.rng.range(0.0, 0.6)) / tiers as f32 * 0.9;
        // reach grows linearly from nothing at the tip: a straight-sided cone
        let reach = (0.004 + slim * (0.99 - v)) * h.rng.range(0.8, 1.1);
        if i % 3 == 0 {
            b.reload(p, 0.6);
            fine.reload(p, 0.5);
            finest.reload(p, 0.5);
        }
        let brush = if reach < 0.03 {
            &mut finest
        } else if reach < 0.06 {
            &mut fine
        } else {
            &mut b
        };
        for s in [-1.0f32, 1.0] {
            // out and down from the stem, the tip turning a little up
            let droop = reach * h.rng.range(0.3, 0.5);
            let pts = [(s * 0.002, v), (s * reach * 0.55, v - droop), (s * reach, v - droop * 0.8)];
            h.mark(c, brush, Mark { pts: &pts, pressure: (0.9, 0.3), ramps: (0.0, 0.5) }, None);
        }
    }
    // the leader: a fine flick lifted off to a point
    finest.reload(p, 0.5);
    h.mark(c, &mut finest, Mark { pts: &[(0.0, 0.9), (0.0005, 0.96), (0.001, 1.0)], pressure: (0.8, 0.0), ramps: (0.0, 0.8) }, None);
}
