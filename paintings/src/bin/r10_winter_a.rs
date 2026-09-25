//! r10_winter_a — "Dolmen in the Snow at Dusk".
//!
//! A winter evening just after sunset on the flat Pomeranian coast land. A
//! Hünengrab (a passage grave: three uprights under one great capstone) sits
//! on the crown of a low snow-covered rise, on the skyline. A stag-headed oak
//! stands beside it, a smaller broken oak behind on the left, against the
//! afterglow. Far off: a thin band of woods and a church spire on the flat
//! horizon; above, a young crescent moon and the evening star in a sky that
//! goes from smalt gray to a pale lemon glow. A lone man in a dark coat
//! stands in the foreground snow, his back to us; crows in the oak; dry
//! grass stalks through the snow, painted last.
//!
//!   cargo paint r10_winter_a                 1000px
//!   cargo paint r10_winter_a -- --full       3200px
//!
//! Notes: notes/r10_winter_a.md

use paint::color::mix;
use paint::graphite::{Lead, hand_line};
use paint::{Fbm, Gesture, Held, Limb, Mask, Mix, Paint, Pigment, Rgb, Rng, Shape, Skeleton, Stipple, Style, Tool, Touch, gradient, hex, shift, smoothstep};
use paintings::run::{Finish, Run};

const ASPECT: f32 = 1.42;
/// The far horizon (the flat land's edge).
const HOR: f32 = 470.0;
/// Where the sun went down (x).
const GLOW_X: f32 = 250.0;
const MOON: (f32, f32) = (178.0, 150.0);

fn far_line(x: f32) -> f32 {
    HOR + 1.1 * (x / 37.0).sin() + 0.7 * (x / 13.0 + 1.0).sin()
}

/// The top of the snow rise in front (the knoll with the dolmen).
fn knoll(x: f32) -> f32 {
    494.0 - 40.0 * (-((x - 470.0) / 185.0).powi(2)).exp() + 2.5 * (x / 61.0).sin() + 1.2 * (x / 23.0 + 2.0).sin()
}

/// Height of the low far woods above the horizon line.
fn woods(x: f32) -> f32 {
    let band = smoothstep(10.0, 40.0, x) * (1.0 - smoothstep(215.0, 250.0, x)) + smoothstep(690.0, 730.0, x) * (1.0 - smoothstep(905.0, 960.0, x));
    let bumps = 0.6 + 0.4 * ((x / 7.0).sin() * (x / 11.3 + 0.7).sin()).abs();
    band * (6.0 + 5.0 * bumps + 3.0 * (x / 53.0).sin())
}

fn sky(x: f32, y: f32) -> Rgb {
    let t = (y / HOR).clamp(0.0, 1.0);
    let base = gradient(
        &[(0.0, hex("#6d7ca3")), (0.28, hex("#8f9bb0")), (0.52, hex("#b7b8b3")), (0.72, hex("#d0c3b5")), (0.88, hex("#e0cdad")), (1.0, hex("#ebd5a8"))],
        t,
        Mix::Light,
    );
    let g = (-((x - GLOW_X) / 280.0).powi(2)).exp() * smoothstep(0.45, 1.0, t);
    let warm = mix(base, hex("#f1dca8"), 0.45 * g, Mix::Light);
    let away = (1.0 - (-((x - GLOW_X) / 420.0).powi(2)).exp()) * smoothstep(0.55, 1.0, t);
    mix(warm, hex("#c3bdb8"), 0.35 * away, Mix::Light)
}

// ckpt: from field
/// A low bank running across the middle ground (a sunken lane's edge): the
/// crest line.
fn bank(x: f32) -> f32 {
    556.0 + 9.0 * (x / 150.0 + 1.0).sin() + 4.0 * (x / 47.0).sin()
}

/// Snow on the field: a cool, dusk-lit white, warm and light on the far
/// slope toward the glow, blue-gray and darker near; modeled by drifts lit
/// from the west and by the bank's shadowed face.
fn snow(x: f32, y: f32, drift: &Fbm) -> Rgb {
    let top = knoll(x);
    let d = ((y - top) / (704.0 - top)).clamp(0.0, 1.0);
    let g = (-((x - GLOW_X) / 300.0).powi(2)).exp();
    let far = mix(hex("#cec8cc"), hex("#dcd0c6"), g, Mix::Light);
    let base = mix(far, hex("#8e95a8"), d.powf(0.75), Mix::Light);
    // drifts: a height field stretched along the ground; slopes facing the
    // glow (west, left) catch light, hollows go blue
    let e = 1.5;
    let hx = |x: f32, y: f32| drift.get(x, y * 4.0);
    let slope = (hx(x + e, y) - hx(x - e, y)) / (2.0 * e);
    let up = (hx(x, y - e) - hx(x, y + e)) / (2.0 * e);
    let lit = (-slope * 110.0 + up * 45.0).clamp(-1.0, 1.0) * (0.3 + 0.7 * d);
    let hollow = drift.get(x, y * 4.0);
    // the bank: a lit lip, then its face turned from the sky in shadow
    let below = y - bank(x);
    let face = if below > 0.0 { (1.0 - below / 16.0).max(0.0).powf(1.5) } else { 0.0 };
    let lip = if below <= 0.0 { (1.0 + below / 5.0).max(0.0) } else { 0.0 };
    shift(base, 0.05 * lit - 0.04 * hollow - 0.095 * face + 0.035 * lip, 0.005 * face, -0.018 * lit.min(0.0) - 0.016 * hollow - 0.03 * face)
}

// ckpt: end

/// Capstone of the dolmen: top and bottom edges.
fn cap_top(x: f32) -> f32 {
    let u = ((x - 490.0) / 84.0).clamp(-1.0, 1.0);
    412.0 + 3.0 * u - 25.0 * (1.0 - u.powi(4)).max(0.0).sqrt() + 1.4 * (x / 9.0).sin() + 2.5 * (x / 31.0 + 1.0).sin() - 3.5 * (-((x - 452.0) / 22.0).powi(2)).exp()
}
fn cap_bot(x: f32) -> f32 {
    let u = ((x - 490.0) / 84.0).clamp(-1.0, 1.0);
    412.0 + 3.0 * u + 17.0 * (1.0 - u.powi(4)).max(0.0).sqrt() + 2.0 * (x / 23.0).sin()
}

/// Paint one limb of a grown tree as the hand would: one movement from
/// where it springs to its tip, handing on to a finer brush where it thins.
// ckpt: from oaks
fn hash01(x: f32, y: f32, k: f32) -> f32 {
    let v = ((x * 12.9898 + y * 78.233 + k * 37.719).sin() * 43758.547).fract();
    v.abs()
}

/// The limb's nodes as the hand draws them: each node pushed a little off
/// the smooth line (oak wood is elbowed and knotted, not a noodle). The push
/// is a function of the node's position, so a twig springing from a node
/// moves with it.
fn gnarled(l: &Limb) -> Vec<(f32, f32)> {
    let n = l.pts.len();
    (0..n)
        .map(|i| {
            let p = l.pts[i];
            if (l.order == 0 && i == 0) || l.root {
                return p;
            }
            let a = if i > 0 { l.pts[i - 1] } else { p };
            let b = if i + 1 < n { l.pts[i + 1] } else { p };
            let seg = 0.5 * (((p.0 - a.0).powi(2) + (p.1 - a.1).powi(2)).sqrt() + ((p.0 - b.0).powi(2) + (p.1 - b.1).powi(2)).sqrt());
            let amp = (0.09 * seg).min(3.0);
            let (qx, qy) = ((p.0 * 4.0).round(), (p.1 * 4.0).round());
            (p.0 + amp * (2.0 * hash01(qx, qy, 1.0) - 1.0), p.1 + amp * (2.0 * hash01(qx, qy, 2.0) - 1.0))
        })
        .collect()
}

/// The painter's taper: oak wood thins faster than the pipe model's
/// widths, from the trunk down to the twigs (w' = W (w / W)^k).
fn taper(mut sk: Skeleton, k: f32) -> Skeleton {
    let top = sk.limbs[0].w[0];
    for l in sk.limbs.iter_mut() {
        for w in l.w.iter_mut() {
            *w = (top * (*w / top).powf(k)).max(0.28);
        }
    }
    sk
}

fn paint_limb(c: &mut paint::Canvas, l: &Limb, live: Paint, dead: Paint, seed: u64) {
    if l.is_empty() {
        return;
    }
    let lp = gnarled(l);
    let n = l.pts.len();
    let mut i0 = 0;
    let mut k = 0;
    while i0 < n - 1 {
        let w0 = l.w[i0].max(0.35);
        let is_dead = l.dead_at(i0);
        let mut i1 = i0 + 1;
        let mut run = ((lp[1.min(n - 1)].0 - lp[0].0).powi(2) + (lp[1.min(n - 1)].1 - lp[0].1).powi(2)).sqrt() * 0.0;
        // a load lasts about 35 units of limb (less for a fat brush)
        let reach = (35.0 - w0).max(14.0);
        while i1 < n - 1 && l.w[i1] > w0 * 0.5 && l.dead_at(i1) == is_dead {
            run += ((lp[i1].0 - lp[i1 - 1].0).powi(2) + (lp[i1].1 - lp[i1 - 1].1).powi(2)).sqrt();
            if run > reach {
                break;
            }
            i1 += 1;
        }
        let last = i1 == n - 1;
        let w1 = l.w[i1].max(0.3);
        let tool = if w0 < 1.3 {
            Tool { point: 1.0, ..Tool::rigger((w0 * 1.4).max(0.5)) }
        } else if w0 < 6.0 {
            Tool { point: 0.7, ..Tool::round_sable(w0 * 1.25) }
        } else {
            Tool::round_sable((w0 * 1.25).min(30.0))
        };
        let pa = tool.pressure_for(w0);
        let pb = if last && !l.broken { 0.05 } else { tool.pressure_for(w1) };
        // set down a little back inside the wet end of the previous section
        let mut pts: Vec<(f32, f32)> = Vec::with_capacity(i1 - i0 + 2);
        if i0 > 0 {
            let (a, b) = (lp[i0 - 1], lp[i0]);
            pts.push((b.0 + (a.0 - b.0) * 0.35, b.1 + (a.1 - b.1) * 0.35));
        }
        pts.extend_from_slice(&lp[i0..=i1]);
        let mut h = Held::new(tool, seed * 131 + k);
        h.load(if is_dead { dead } else { live }, 1.0);
        // a section set down inside the previous one goes down at full
        // pressure; only the limb's own start and live tip have ramps
        let attack = if i0 == 0 { 0.04 } else { 0.0 };
        let release = if last && !l.broken { 0.45 } else { 0.0 };
        c.drag(&mut h, &Gesture::new(pts).pressure(pa, pb).ramps(attack, release).shake(0.35), None);
        // a broken end: splinters, one longer than the rest
        if last && l.broken && w1 > 1.0 {
            let d = l.dir(n - 1);
            let e = lp[n - 1];
            let mut sp = Held::new(Tool { point: 1.0, ..Tool::rigger((w1 * 0.5).clamp(0.5, 3.0)) }, seed * 17 + 3);
            for (k2, off) in [-0.3f32, 0.05, 0.35].iter().enumerate() {
                let (nx, ny) = (-d.1, d.0);
                let st_ = (e.0 + nx * off * w1, e.1 + ny * off * w1);
                let ln = w1 * if k2 == 1 { 1.6 } else { 0.7 } * (0.8 + 0.4 * hash01(e.0, e.1, k2 as f32));
                let tip = (st_.0 + d.0 * ln + nx * off * w1 * 0.3, st_.1 + d.1 * ln + ny * off * w1 * 0.3);
                sp.reload(if is_dead { dead } else { live }, 0.6);
                c.drag(&mut sp, &Gesture::new(vec![(st_.0 - d.0 * w1 * 0.3, st_.1 - d.1 * w1 * 0.3), tip]).pressure(0.7, 0.0).ramps(0.0, 0.8).shake(0.3), None);
            }
        }
        i0 = i1;
        k += 1;
    }
}

/// My oak: a short heavy bole that splits low into a few crooked limbs,
/// each turning at elbows from bud to bud, with side limbs off them down
/// to the fourth order; some limbs broken off short, some dead. Returned
/// as a `Skeleton` so the same hand paints it.
fn my_oak(base: (f32, f32), height: f32, lean: f32, broken_top: bool, seed: u64) -> Skeleton {
    use std::f32::consts::FRAC_PI_2;
    let mut rng = Rng::new(seed);
    let mut limbs: Vec<Limb> = Vec::new();
    let w0 = height * 0.08;
    #[allow(clippy::too_many_arguments)]
    fn grow(limbs: &mut Vec<Limb>, parent: Option<(usize, usize)>, start: (f32, f32), ang: f32, len: f32, w: f32, order: u32, height: f32, broken_top: bool, rng: &mut Rng) {
        let seg = (8.0 + 5.0 * rng.f()) * (if order == 0 { 1.4 } else { 1.0 });
        let n = ((len / seg).round() as usize).max(2);
        let mut pts = vec![start];
        let mut ws = vec![w];
        let mut a = ang;
        let mut q = start;
        let broken = order >= 1 && order <= 2 && rng.f() < 0.22;
        let n = if broken { (n as f32 * rng.range(0.4, 0.75)).ceil() as usize } else { n };
        let tip_w = if order == 0 { w * 0.55 } else { (w * 0.3).max(0.35) };
        for k in 1..=n {
            // elbows: a sharp turn now and then, small wanders otherwise
            let turn = if rng.f() < 0.3 { rng.range(-0.55, 0.55) } else { rng.range(-0.18, 0.18) };
            a += turn * if order == 0 { 0.35 } else { 1.0 };
            // limbs reach out and then turn up to the light; big ones sag a little
            let up = -FRAC_PI_2;
            let pull = if order == 0 { 0.25 } else if order == 1 { 0.06 } else { 0.1 };
            a += pull * (up - a).sin().clamp(-1.0, 1.0) * if order == 1 && k < n / 2 { -0.4 } else { 1.0 };
            let l = len / n as f32 * rng.range(0.75, 1.25);
            q = (q.0 + l * a.cos(), q.1 + l * a.sin());
            pts.push(q);
            let t = k as f32 / n as f32;
            ws.push(w + (tip_w - w) * t.powf(0.8));
        }
        let dead = order >= 2 && rng.f() < 0.3 || broken && rng.f() < 0.6;
        let np = pts.len();
        let idx = limbs.len();
        limbs.push(Limb {
            z: vec![0.0; np],
            age: vec![1; np],
            w: ws.clone(),
            order,
            parent: parent.map(|p| p.0),
            at: parent.map_or(0, |p| p.1),
            dead,
            dead_from: if dead { 0 } else if order == 0 && broken_top { np - 2 } else { np },
            broken: broken || (order == 0 && broken_top),
            root: false,
            pts: pts.clone(),
        });
        if order >= 4 || len < 14.0 {
            return;
        }
        // side limbs
        let kids = match order {
            0 => 4 + (rng.f() * 2.0) as usize,
            1 => 3 + (rng.f() * 3.0) as usize,
            2 => 2 + (rng.f() * 3.0) as usize,
            _ => 1 + (rng.f() * 2.0) as usize,
        };
        let mut side = if rng.f() < 0.5 { -1.0 } else { 1.0 };
        for j in 0..kids {
            let t = if order == 0 { rng.range(0.55, 1.0) } else { rng.range(0.2, 0.95) };
            let i = ((t * (np - 1) as f32) as usize).clamp(1, np - 1);
            let (p0, p1) = (pts[i - 1], pts[i]);
            let pa = (p1.1 - p0.1).atan2(p1.0 - p0.0);
            let (ca, cl, cw);
            if order == 0 {
                // the bole splits: main limbs fanning out and up
                ca = -FRAC_PI_2 + side * rng.range(0.25, 1.15) * (0.6 + 0.4 * (j as f32 / kids as f32));
                cl = height * rng.range(0.45, 0.68) * if j == 0 { 1.1 } else { 1.0 };
                cw = ws[i] * rng.range(0.5, 0.72);
            } else {
                ca = pa + side * rng.range(0.45, 1.05);
                cl = len * (1.0 - t * 0.5) * rng.range(0.4, 0.7);
                cw = ws[i] * rng.range(0.45, 0.7);
            }
            side = -side;
            grow(limbs, Some((idx, i)), pts[i], ca, cl, cw, order + 1, height, broken_top, rng);
        }
    }
    let trunk_len = height * rng.range(0.28, 0.36);
    grow(&mut limbs, None, base, -FRAC_PI_2 + lean, trunk_len, w0, 0, height, broken_top, &mut rng);
    // buttress roots into the snow
    for side in [-1.0f32, 1.0] {
        let a = if side < 0.0 { std::f32::consts::PI - 0.1 } else { 0.1 };
        let l = w0 * rng.range(0.7, 1.1);
        let pts = vec![(base.0 + side * w0 * 0.15, base.1 - w0 * 0.12), (base.0 + side * w0 * 0.15 + l * a.cos(), base.1 - w0 * 0.12 + l * a.sin())];
        limbs.push(Limb { z: vec![0.0; 2], age: vec![1; 2], w: vec![w0 * 0.4, w0 * 0.12], order: 1, parent: Some(0), at: 0, dead: false, dead_from: 2, broken: false, root: true, pts });
    }
    Skeleton { limbs, base, height, pipe: 2.0, leaf: paint::Leafing::none() }
}

/// One crooked twig: two or three straight pieces with hard elbows (oak
/// twigs zig-zag from bud to bud), pressed where it leaves the limb and
/// lifted off to a point; then, sometimes, twigs off it.
fn claw(c: &mut paint::Canvas, h: &mut Held, p: Paint, at: (f32, f32), dir: f32, len: f32, w: f32, depth: u32, rng: &mut Rng) {
    let segs = 2 + (rng.f() * 2.0) as usize;
    let mut pts = vec![at];
    let mut a = dir;
    let mut q = at;
    for k in 0..segs {
        let l = len / segs as f32 * rng.range(0.7, 1.3);
        // elbows alternate, and oak twigs turn upward as they go
        a += if k % 2 == 0 { 1.0 } else { -1.0 } * rng.range(0.2, 0.55) - 0.12 * a.cos().signum() * (a + std::f32::consts::FRAC_PI_2).sin().abs().min(0.0);
        let up = -std::f32::consts::FRAC_PI_2;
        a += 0.12 * (up - a).sin().signum() * (up - a).sin().abs().min(0.4);
        q = (q.0 + l * a.cos(), q.1 + l * a.sin());
        pts.push(q);
    }
    let t = Tool { point: 1.0, ..Tool::rigger((w * 1.2).clamp(0.45, 1.6)) };
    let pa = t.pressure_for(w * 0.8).clamp(0.12, 0.5);
    h.reload(p, 0.4);
    c.drag(h, &Gesture::new(pts.clone()).pressure(pa, 0.0).ramps(0.03, 0.7).shake(0.5), None);
    if depth > 0 && len > 7.0 {
        let n = 1 + (rng.f() * 2.5) as usize;
        for _ in 0..n {
            let i = 1 + (rng.f() * (pts.len() - 1) as f32) as usize;
            let i = i.min(pts.len() - 1);
            let side = if rng.f() < 0.5 { -1.0 } else { 1.0 };
            claw(c, h, p, pts[i - 1], dir + side * rng.range(0.5, 1.0), len * rng.range(0.35, 0.6), w * 0.6, depth - 1, rng);
        }
    }
}

/// A grown tree painted limb by limb (trunk first), then snow lying along
/// the tops of its level limbs.
fn paint_tree(c: &mut paint::Canvas, sk: &Skeleton, live: Paint, dead: Paint, rim_p: Paint, snow_p: Paint, glow: (f32, f32), rng: &mut Rng, seed: u64) {
    for (i, l) in sk.limbs.iter().enumerate() {
        paint_limb(c, l, live, dead, seed + i as u64);
    }
    // the twig habit: along every limb of middling girth, short crooked
    // claws on either side, denser toward the crown's edge
    let mut th = Held::new(Tool::rigger(1.0), seed ^ 0xc1a);
    for l in sk.limbs.iter().filter(|l| !l.root && !l.is_empty() && l.order >= 1) {
        let lp = gnarled(l);
        let mut s_acc = 0.0;
        let mut next = rng.range(4.0, 12.0);
        for i in 1..lp.len() {
            let (a, b) = (lp[i - 1], lp[i]);
            let seg = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
            s_acc += seg;
            let w = l.w[i];
            if w > 9.0 || w < 0.3 {
                continue;
            }
            while s_acc > next {
                next += rng.range(5.0, 14.0) * (0.6 + 0.15 * w);
                let d = l.dir(i);
                let base_a = d.1.atan2(d.0);
                let side = if rng.f() < 0.5 { -1.0 } else { 1.0 };
                let dir = base_a + side * rng.range(0.55, 1.15);
                let len = rng.range(6.0, 16.0) * (0.6 + 0.25 * w.min(4.0)) * (sk.height / 260.0).sqrt();
                let tp = if l.dead_at(i - 1) { dead } else { live };
                claw(c, &mut th, tp, b, dir, len * rng.range(0.5, 1.5), (w * 0.4).clamp(0.3, 1.2), 2, rng);
            }
        }
    }
    c.wait(15.0);
    // the side of the bigger wood that faces the afterglow takes a dull warm
    // light: a lean broken line down that edge
    for (i, l) in sk.limbs.iter().enumerate().filter(|(_, l)| !l.root && !l.is_empty()) {
        let lp = gnarled(l);
        let n = lp.len();
        let mut j = 0;
        while j < n - 1 {
            let w = l.w[j];
            if w < 2.2 || rng.f() < 0.25 {
                j += 1;
                continue;
            }
            let k = (j + 3).min(n - 1);
            let pts: Vec<(f32, f32)> = (j..=k)
                .map(|q| {
                    let d = l.dir(q);
                    // the normal on the side toward the glow
                    let (mut nx, mut ny) = (-d.1, d.0);
                    let (gx, gy) = (glow.0 - lp[q].0, glow.1 - lp[q].1);
                    if nx * gx + ny * gy < 0.0 {
                        nx = -nx;
                        ny = -ny;
                    }
                    (lp[q].0 + nx * l.w[q] * 0.4, lp[q].1 + ny * l.w[q] * 0.4)
                })
                .collect();
            let mut h = Held::new(Tool::rigger((w * 0.12).clamp(0.45, 1.4)), seed ^ (i as u64 * 7919 + j as u64));
            h.load(rim_p, 0.35);
            c.drag(&mut h, &Gesture::new(pts).pressure(0.45, 0.3).ramps(0.25, 0.4).shake(0.6), None);
            j = k + 1;
        }
    }
    // bark: oak's furrows, short broken strokes along the big wood, some
    // darker than the bark, some catching a little light
    let mut bk = Held::new(Tool::rigger(0.9), seed ^ 0xba4c);
    let furrow = [live, Paint { color: shift(live.color, 0.05, 0.0, 0.005), ..live }, Paint { color: shift(live.color, -0.04, 0.0, 0.0), ..live }];
    for (i, l) in sk.limbs.iter().enumerate().filter(|(_, l)| !l.root && !l.is_empty() && l.order == 0) {
        let lp = gnarled(l);
        let n = lp.len();
        for q in 0..n - 1 {
            let w = l.w[q];
            if w < 7.0 {
                break;
            }
            let strokes = (w * 0.5) as usize + 1;
            for _ in 0..strokes {
                let d = l.dir(q);
                let (nx, ny) = (-d.1, d.0);
                let off = rng.range(-0.42, 0.42) * w;
                let t0 = rng.f();
                let seg = ((lp[q + 1].0 - lp[q].0).powi(2) + (lp[q + 1].1 - lp[q].1).powi(2)).sqrt();
                let len = rng.range(0.4, 1.0) * seg;
                let p0 = (lp[q].0 + d.0 * seg * t0 * 0.5 + nx * off, lp[q].1 + d.1 * seg * t0 * 0.5 + ny * off);
                let p1 = (p0.0 + d.0 * len + nx * rng.range(-0.6, 0.6), p0.1 + d.1 * len + ny * rng.range(-0.6, 0.6));
                // light on the side toward the glow
                let (gx, gy) = (glow.0 - p0.0, glow.1 - p0.1);
                let toward = (nx * gx + ny * gy).signum() * off.signum() > 0.0 && off.abs() > 0.2 * w;
                let col = if toward && rng.f() < 0.35 { rim_p } else { furrow[(rng.f() * 3.0) as usize % 3] };
                bk.reload(col, 0.4);
                c.drag(&mut bk, &Gesture::new(vec![p0, ((p0.0 + p1.0) * 0.5 + nx * rng.range(-0.5, 0.5), (p0.1 + p1.1) * 0.5), p1]).pressure(0.45, 0.2).ramps(0.2, 0.4).shake(0.7), None);
            }
            let _ = i;
        }
    }
    // snow: a thin broken line on the upper side of limbs that lie level
    // enough to hold it
    let mut h = Held::new(Tool::rigger(1.2), seed ^ 0x5eed);
    for l in sk.limbs.iter().filter(|l| !l.root && !l.is_empty()) {
        let lp = gnarled(l);
        let n = l.pts.len();
        let mut i = 0;
        while i < n - 1 {
            let w = l.w[i];
            let d = l.dir(i);
            if w < 1.2 || d.1.abs() > 0.55 || rng.f() < 0.35 {
                i += 1;
                continue;
            }
            // a run of segments while it stays level
            let mut j = i + 1;
            while j < n - 1 && l.dir(j).1.abs() < 0.55 && j - i < 3 {
                j += 1;
            }
            let pts: Vec<(f32, f32)> = (i..=j)
                .map(|q| {
                    // the upper edge: along the normal that points up
                    let d = l.dir(q);
                    let (mut nx, mut ny) = (-d.1, d.0);
                    if ny > 0.0 {
                        nx = -nx;
                        ny = -ny;
                    }
                    (lp[q].0 + nx * l.w[q] * 0.5, lp[q].1 + ny * l.w[q] * 0.5)
                })
                .collect();
            let sw = (w * 0.3).clamp(0.5, 2.5);
            let t = Tool::rigger(sw);
            h = Held::new(t, seed ^ (i as u64 * 977 + l.pts.len() as u64));
            h.load(snow_p, 0.7);
            c.drag(&mut h, &Gesture::new(pts).pressure(0.55, 0.25).ramps(0.2, 0.4).shake(0.5), None);
            i = j + 1;
        }
    }
    let _ = h;
}

// ckpt: end

fn main() {
    let o = Run::new("r10_winter_a");
    let st = Style::friedrich();
    let pal = &st.palette;
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = c.height();

    let far_l = f.per_column(far_line);
    let kn = f.per_column(knoll);
    let wd = f.per_column(woods);

    // ---- the underdrawing: horizon against the ruler, the rise, the
    // dolmen and the oak trunks, in graphite on the ground
    if o.stage("drawing", &mut c, &mut rng) {
        let hb = Lead::pencil("HB").unwrap();
        let h2 = Lead::pencil("2H").unwrap();
        let mut worn = 0.0;
        worn += c.draw(&h2, &hand_line(&[(0.0, HOR), (1000.0, HOR)], &[0.4, 0.45], false, true, 0.0, 1), worn, 1);
        let rise: Vec<(f32, f32)> = (0..=40).map(|i| {
            let x = i as f32 * 25.0;
            (x, knoll(x))
        }).collect();
        worn += c.draw(&hb, &hand_line(&rise, &[0.5, 0.6, 0.45], true, false, 0.6, 2), worn, 2);
        let capl: Vec<(f32, f32)> = (0..=14).map(|i| {
            let x = 402.0 + i as f32 * 12.5;
            (x, cap_top(x))
        }).collect();
        worn += c.draw(&hb, &hand_line(&capl, &[0.6, 0.7], true, false, 0.4, 3), worn, 3);
        for (a, b) in [((612.0f32, 470.0f32), (606.0f32, 300.0f32)), ((336.0, 468.0), (330.0, 360.0))] {
            worn += c.draw(&hb, &hand_line(&[a, b], &[0.5, 0.3], true, false, 0.5, 4), worn, 4);
        }
        let _ = worn;
    }

    // ---- the sky: laid in thin, fused, stippled wet, then a finer
    // stipple once dry lifting the glow
    let sky_m = Mask::from_fn(f, move |x, y| 1.0 - smoothstep(-1.0, 3.0, y - far_l(x).min(kn(x)) + 2.0));
    if o.stage("sky", &mut c, &mut rng) {
        let sky_pal = pal.only(&["lead white", "cobalt blue", "pale smalt", "yellow ochre", "red earth", "raw umber"]);
        let lay = st.broad().palette(&sky_pal).color(move |x, y| shift(sky(x, y), -0.03, 0.0, 0.0)).angle(|x, _| 0.04 * (x / 300.0).sin()).coverage(4.5).medium(0.3).dips(1, 0.6, 0.6);
        c.work(&sky_m, &lay, 11);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.angle(|_, _| 0.0), 12);
        }
        let s1 = Stipple::new(Tool::stippler(3.0)).mixed(&sky_pal, 0.45).color(sky).coverage(|_, _| 2.0).pressure(0.5, 0.85).dips(20, 0.4, 0.5);
        c.stipple(&sky_m, &s1, 13);
        c.dry();
    }

    if o.stage("glow", &mut c, &mut rng) {
        let sky_pal = pal.only(&["lead white", "cobalt blue", "pale smalt", "yellow ochre", "red earth", "raw umber"]);
        let glow = move |x: f32, y: f32| {
            let g = (-((x - GLOW_X) / 240.0).powi(2)).exp();
            mix(sky(x, y), hex("#f4e2b6"), 0.05 + 0.25 * g * smoothstep(250.0, HOR, y), Mix::Light)
        };
        let s2 = Stipple::new(Tool::stippler(1.6)).mixed(&sky_pal, 0.5).color(glow).coverage(|_, y| 0.6 + 1.6 * smoothstep(80.0, HOR, y)).pressure(0.5, 0.85).dips(24, 0.35, 0.6);
        c.stipple(&sky_m, &s2, 14);
        c.dry();
    }

    // ---- moon and evening star
    if o.stage("moon", &mut c, &mut rng) {
        let r = 11.0;
        // the lit limb faces the sun, below the horizon toward the glow
        let sun_dir = ((GLOW_X - MOON.0), (HOR + 40.0 - MOON.1));
        let sl = (sun_dir.0 * sun_dir.0 + sun_dir.1 * sun_dir.1).sqrt();
        let (sx, sy) = (sun_dir.0 / sl, sun_dir.1 / sl);
        // a young crescent: the disk beyond the terminator (an ellipse
        // across it), thick in the middle and sharp at the horns
        let crescent = Mask::from_fn(f, move |x, y| {
            let (px, py) = (x - MOON.0, y - MOON.1);
            let a = px * sx + py * sy;
            let b = -px * sy + py * sx;
            let disk = 1.0 - smoothstep(r - 0.5, r + 0.5, (px * px + py * py).sqrt());
            let term = 0.62 * (r * r - b * b).max(0.0).sqrt();
            disk * smoothstep(term - 0.4, term + 0.4, a)
        });
        let dark = Mask::from_fn(f, move |x, y| {
            let d = ((x - MOON.0).powi(2) + (y - MOON.1).powi(2)).sqrt();
            1.0 - smoothstep(r - 0.6, r + 0.6, d)
        })
        .subtract(&crescent);
        // earthshine: the rest of the disk a breath lighter than the sky
        c.work(&dark, &st.detail().color_over(|_, _, u| shift(u, 0.013, 0.0, 0.003)).length(2.0, 5.0).coverage(3.0).medium(0.5), 76);
        c.work(&crescent, &st.detail().color(|_, _| hex("#f5eed6")).angle(move |x, y| (y - MOON.1).atan2(x - MOON.0) + std::f32::consts::FRAC_PI_2).length(2.0, 6.0).coverage(4.5).medium(0.05), 77);
        let mut s = Held::new(Tool::round_sable(1.6), 78);
        s.load(pal.paint(hex("#fbf4df"), 0.05).with_hiding(0.97), 0.8);
        c.touch(&mut s, &Touch::at(318.0, 232.0).pressure(0.45), None);
        c.dry();
    }

    // ---- the far land: woods and a spire on the horizon, the far plain
    let far_m = Mask::from_fn(f, move |x, y| {
        let top = far_l(x) - wd(x);
        smoothstep(top - 0.8, top + 0.8, y) * (1.0 - smoothstep(kn(x) - 1.0, kn(x) + 1.0, y))
    });
    if o.stage("far", &mut c, &mut rng) {
        let far_col = move |x: f32, y: f32| {
            let fl = far_line(x);
            if y < fl + 0.5 {
                // woods: blue-gray in the haze, lighter near the glow
                let g = (-((x - GLOW_X) / 200.0).powi(2)).exp();
                mix(hex("#6e7382"), hex("#9b9493"), 0.5 * g, Mix::Light)
            } else {
                mix(hex("#c9c0c0"), hex("#e0d2bc"), (-((x - GLOW_X) / 260.0).powi(2)).exp(), Mix::Light)
            }
        };
        let hd = st.hatch().color(far_col).angle(|_, _| 0.0).length(4.0, 10.0).coverage(3.5).clip(true);
        c.work(&far_m, &hd, 21);
        // single trees stand up out of the woods band: tiny pointed spruces
        let tree_p = pal.paint(hex("#686c7a"), 0.15);
        let mut b = Held::new(Tool::round_sable(1.1), 22);
        let mut x = 15.0;
        while x < 960.0 {
            let w = woods(x);
            if w > 4.0 && rng.f() < 0.3 {
                let ht = w * rng.range(0.6, 1.0) + rng.range(1.0, 7.0) * rng.f();
                let base = far_line(x) - w * 0.5;
                b.reload(tree_p, 0.6);
                c.drag(&mut b, &Gesture::new(vec![(x, base), (x + 0.2, base - ht)]).pressure(0.7, 0.0).ramps(0.05, 0.7).shake(0.3), None);
            }
            x += rng.range(2.0, 12.0) * rng.f() + 1.5;
        }
        // the church: nave, tower and a tall spire, far and blue in the air
        let ch = pal.paint(hex("#737684"), 0.15);
        let cx = 842.0;
        let fl = far_line(cx);
        let body = Mask::from_shape(f, Shape::new().poly(&[(cx - 16.0, fl), (cx - 16.0, fl - 7.0), (cx - 2.0, fl - 9.5), (cx + 3.0, fl - 9.5), (cx + 3.0, fl)]).add(Shape::new().rect(cx + 3.0, fl - 17.0, 5.0, 17.0)));
        c.work(&body, &st.detail().color(|_, _| hex("#737684")).angle(|_, _| 1.57).length(2.0, 6.0).coverage(4.0), 23);
        let mut r = Held::new(Tool::rigger(0.9), 24);
        r.load(ch, 0.8);
        c.drag(&mut r, &Gesture::new(vec![(cx + 5.5, fl - 16.0), (cx + 5.6, fl - 34.0)]).pressure(0.9, 0.0).ramps(0.02, 0.8).shake(0.1), None);
        c.dry();
    }

    // ---- the snow field in front, rising to the knoll
    let field_m = Mask::from_fn(f, move |x, y| smoothstep(kn(x) - 1.0, kn(x) + 1.0, y));
    let drift = Fbm::new(5, 3, 300.0);
    if o.stage("field", &mut c, &mut rng) {
        let snow_pal = pal.only(&["lead white", "cobalt blue", "pale smalt", "yellow ochre", "red earth", "raw umber", "bone black"]);
        // the lay-in: long strokes following the lie of the ground, curving
        // over the knoll
        let ground_angle = move |x: f32, y: f32| {
            let slope = (knoll(x + 4.0) - knoll(x - 4.0)) / 8.0;
            let fade = 1.0 - smoothstep(0.0, 90.0, y - knoll(x));
            (slope * fade).atan() + 0.05 * (x / 170.0 + y / 90.0).sin()
        };
        let lay = st.broad().palette(&snow_pal).color(move |x, y| snow(x, y, &drift)).angle(ground_angle).length(60.0, 160.0).coverage(4.5).medium(0.2).mix_jitter(0.012);
        c.work(&field_m, &lay, 31);
        if let Some(b) = st.blend() {
            c.work(&field_m, &b.angle(ground_angle).coverage(2.0), 32);
        }
        c.wait(20.0);
        // body snow in the foreground: shorter strokes, stiffer paint, the
        // drifts modeled
        let near_m = field_m.clone().mul_fn(|_, y| smoothstep(520.0, 600.0, y));
        let body = st.body().palette(&snow_pal).color(move |x, y| snow(x, y, &drift)).angle(ground_angle).length(25.0, 70.0).coverage(2.5).medium(0.12).mix_jitter(0.015);
        c.work(&near_m, &body, 33);
        // fuse the body strokes into the lay-in, gently, along the ground
        if let Some(b) = st.blend() {
            c.work(&near_m, &b.angle(ground_angle).coverage(2.5).clip(false), 34);
        }
        c.dry();
    }

    // ---- wind-carved drift crests in the near snow: a soft blue shadow
    // under each crest, a lit lip along it
    let crest_list: Vec<(f32, f32, f32, f32, f32)> = {
        let mut r = Rng::new(o.seed + 99);
        (0..9)
            .map(|k| {
                let y0 = 574.0 + k as f32 * 14.0 + r.range(-5.0, 5.0);
                let len = r.range(160.0, 400.0) * (0.7 + 0.5 * k as f32 / 9.0);
                let xa = r.range(-80.0, 1000.0 - len * 0.4);
                (y0, len, xa, r.range(2.0, 6.0), r.range(0.0, 6.0))
            })
            .collect()
    };
    let crest_at = |cr: &(f32, f32, f32, f32, f32), x: f32| cr.0 + cr.3 * ((x - cr.2) / cr.1 * 5.0 + cr.4).sin() + 1.5 * ((x - cr.2) / 23.0).sin();
    let crests_c = crest_list.clone();
    let drift_sh = Mask::from_fn(f, move |x, y| {
        let mut m: f32 = 0.0;
        for cr in &crests_c {
            let u = (x - cr.2) / cr.1;
            if !(0.0..=1.0).contains(&u) {
                continue;
            }
            let ext = smoothstep(0.0, 0.25, u.min(1.0 - u));
            let d = y - crest_at(cr, x);
            let depth = 3.5 + 4.0 * smoothstep(560.0, 704.0, cr.0);
            let v = smoothstep(-0.8, 0.8, d) * (1.0 - smoothstep(depth * 0.2, depth, d)) * smoothstep(0.0, 0.25, u.min(1.0 - u));
            m = m.max(v * ext);
        }
        m
    });
    let crests_c = crest_list.clone();
    let drift_lip = Mask::from_fn(f, move |x, y| {
        let mut m: f32 = 0.0;
        for cr in &crests_c {
            let u = (x - cr.2) / cr.1;
            if !(0.0..=1.0).contains(&u) {
                continue;
            }
            let ext = smoothstep(0.05, 0.3, u.min(1.0 - u));
            let d = crest_at(cr, x) - y;
            let v = smoothstep(-0.5, 0.5, d) * (1.0 - smoothstep(1.5, 3.5, d));
            m = m.max(v * ext);
        }
        m
    });
    if o.stage("drifts", &mut c, &mut rng) {
        let snow_pal = pal.only(&["lead white", "cobalt blue", "pale smalt", "yellow ochre", "red earth", "raw umber"]);
        let g_ang = |_: f32, _: f32| 0.0;
        c.work(&drift_sh.clone().blur(1.0), &st.detail().palette(&snow_pal).color_over(|_, y, u| shift(u, -0.018 - 0.014 * smoothstep(560.0, 704.0, y), -0.001, -0.007)).angle(g_ang).length(12.0, 40.0).coverage(3.0).medium(0.35).mix_jitter(0.008), 35);
        c.wait(20.0);
        c.work(&drift_lip, &st.detail().palette(&snow_pal).color_over(|_, _, u| shift(u, 0.025, 0.002, 0.006)).angle(g_ang).length(8.0, 24.0).coverage(2.5).medium(0.1), 37);
        c.dry();
    }

    // ---- the dolmen
    let stone_f = Fbm::new(9, 4, 14.0);
    let cap_m = Mask::from_fn(f, move |x, y| {
        let inx = smoothstep(405.0, 407.0, x) * (1.0 - smoothstep(573.0, 575.0, x));
        inx * smoothstep(cap_top(x) - 0.7, cap_top(x) + 0.7, y) * (1.0 - smoothstep(cap_bot(x) - 0.7, cap_bot(x) + 0.7, y))
    })
    .roughen(3, 12.0, 1.4, 0.5);
    let uprights = Mask::from_shape(
        f,
        Shape::new()
            .poly(&[(416.0, 424.0), (446.0, 427.0), (450.0, 466.0), (410.0, 468.0)])
            .add(Shape::new().poly(&[(522.0, 429.0), (556.0, 427.0), (563.0, 472.0), (518.0, 470.0)])),
    )
    .roughen(4, 9.0, 2.2, 0.5)
    .mul(&Mask::from_fn(f, move |x, y| 1.0 - smoothstep(kn(x) - 1.5, kn(x) + 1.0, y)));
    let back_up = Mask::from_shape(f, Shape::new().poly(&[(477.0, 430.0), (494.0, 431.0), (497.0, 464.0), (475.0, 464.0)]))
        .roughen(5, 10.0, 1.0, 0.5)
        .mul(&Mask::from_fn(f, move |x, y| 1.0 - smoothstep(kn(x) - 1.5, kn(x) + 1.0, y)));
    let boulders = Mask::from_shape(f, Shape::new().ellipse(388.0, 467.0, 15.0, 7.0).add(Shape::new().ellipse(590.0, 476.0, 12.0, 6.0)).add(Shape::new().ellipse(455.0, 473.0, 9.0, 4.5)).add(Shape::new().ellipse(371.0, 475.0, 7.0, 3.5)))
        .roughen(6, 8.0, 1.2, 0.5)
        .mul(&Mask::from_fn(f, move |_, y| 1.0 - smoothstep(0.0, 1.0, 0.0 * y)));
    if o.stage("dolmen", &mut c, &mut rng) {
        // under the capstone the evening shows through between the stones:
        // only a thin cool shadow band hangs under the stone's belly, and
        // the chamber's floor of snow is in shade
        let under_m = Mask::from_fn(f, move |x, y| {
            let inx = smoothstep(446.0, 449.0, x) * (1.0 - smoothstep(519.0, 522.0, x));
            let belly = smoothstep(cap_bot(x) - 1.0, cap_bot(x), y) * (1.0 - smoothstep(cap_bot(x) + 2.5, cap_bot(x) + 4.5, y));
            let floor = smoothstep(kn(x) - 5.0, kn(x) - 3.0, y) * (1.0 - smoothstep(kn(x) + 0.5, kn(x) + 1.5, y));
            inx * belly.max(floor)
        })
        .roughen(8, 10.0, 0.8, 0.4);
        c.work(&under_m, &st.detail().color(|x, y| if y < knoll(x) - 6.0 { hex("#3e3d45") } else { hex("#8e919f") }).angle(|_, _| 0.0).length(4.0, 12.0).coverage(4.0), 40);
        // the far upright, in the shadow under the stone
        c.work(&back_up, &st.detail().color(move |x, y| shift(hex("#45434a"), 0.03 * stone_f.get(x, y), 0.0, 0.0)).angle(|_, _| 1.5).length(4.0, 10.0).coverage(4.0), 41);
        // stones: granite, dark in the dusk; the west faces catch a little
        // warm light from the glow, the tops cool light from the sky
        let fine = Fbm::new(19, 3, 4.0);
        // granite at dusk: the tops take the cool sky, the west ends a
        // little warm light from the glow, the undersides are dark, and
        // low down the snow throws a cool reflected light back up
        let stone = move |x: f32, y: f32, top: f32, bot: f32, left: f32, right: f32| {
            let t = ((y - top) / (bot - top).max(1.0)).clamp(0.0, 1.0);
            let base = gradient(&[(0.0, hex("#6a6c73")), (0.3, hex("#555250")), (0.75, hex("#3c3937")), (1.0, hex("#302e2f"))], t, Mix::Light);
            let west = 1.0 - smoothstep(left, left + 9.0, x);
            let east = smoothstep(right - 9.0, right, x);
            let b = mix(base, hex("#76685b"), 0.5 * west, Mix::Light);
            let b = shift(b, -0.035 * east, 0.0, -0.006 * east);
            let n = 0.05 * stone_f.get(x, y) + 0.03 * fine.get(x, y);
            shift(b, n, 0.004 * stone_f.get(y, x), 0.0)
        };
        let up_col = move |x: f32, y: f32| {
            let (top, l, r) = if x < 490.0 { (426.0, 412.0, 449.0) } else { (428.0, 519.0, 560.0) };
            let b = stone(x, y, top, knoll(x) + 2.0, l, r);
            // reflected light off the snow at the foot
            let refl = smoothstep(knoll(x) - 14.0, knoll(x), y);
            mix(b, hex("#7c808e"), 0.35 * refl, Mix::Light)
        };
        let up_hd = st.detail().color(up_col).angle(|_, _| 1.45).angle_jitter(0.25).length(5.0, 14.0).coverage(4.0);
        c.work(&uprights, &up_hd, 42);
        let cap_col = move |x: f32, y: f32| {
            let b = stone(x, y, cap_top(x), cap_bot(x), 410.0, 572.0);
            // the underside above the snow also takes reflected light
            let refl = smoothstep(cap_bot(x) - 5.0, cap_bot(x), y);
            mix(b, hex("#5d606c"), 0.3 * refl, Mix::Light)
        };
        // strokes follow the dome of the stone
        let cap_ang = move |x: f32, y: f32| {
            let t = ((y - cap_top(x)) / (cap_bot(x) - cap_top(x)).max(1.0)).clamp(0.0, 1.0);
            let st_ = (cap_top(x + 2.0) - cap_top(x - 2.0)) / 4.0;
            let sb = (cap_bot(x + 2.0) - cap_bot(x - 2.0)) / 4.0;
            (st_ * (1.0 - t) + sb * t).atan()
        };
        let cap_hd = st.detail().color(cap_col).angle(cap_ang).angle_jitter(0.2).length(6.0, 18.0).coverage(4.0);
        c.work(&cap_m, &cap_hd, 43);
        c.work(&boulders, &st.detail().color(move |x, y| stone(x, y, 458.0, 480.0, 376.0, 602.0)).angle(|_, _| 0.1).length(3.0, 8.0).coverage(4.0), 44);
        c.wait(30.0);
        // the planes turned up to the sky: lean, dry strokes of a lighter
        // cool gray dragged over the dark, catching on the grain; then
        // fissures, then lichen and grain
        let all = cap_m.clone().union(&uprights).union(&boulders);
        c.wait(20.0);
        // the upper planes of the capstone, lighter where they turn up to
        // the sky: broken patches, one plane beside the next
        let facets = Fbm::new(23, 3, 18.0);
        let upper = cap_m.clone().mul_fn(move |x, y| {
            let t = (y - cap_top(x)) / (cap_bot(x) - cap_top(x)).max(1.0);
            (1.0 - smoothstep(0.25, 0.55, t)) * smoothstep(-0.05, 0.15, facets.get(x, y * 1.6))
        });
        c.work(&upper, &st.detail().color_over(|_, _, u| shift(u, 0.06, -0.002, -0.01)).angle(cap_ang).angle_jitter(0.15).length(5.0, 14.0).coverage(3.0).medium(0.1), 52);
        let mut fis = Held::new(Tool::rigger(0.7), 51);
        for _ in 0..16 {
            let x = rng.range(412.0, 566.0);
            let y = rng.range(cap_top(x) + 6.0, knoll(x) - 3.0);
            if all.sample(x, y) < 0.95 {
                continue;
            }
            let a = rng.range(0.0, std::f32::consts::PI);
            let pts: Vec<(f32, f32)> = (0..4).map(|i| {
                let t = i as f32 * rng.range(2.0, 4.0);
                (x + t * (a + rng.range(-0.4, 0.4)).cos(), y + t * (a + rng.range(-0.4, 0.4)).sin())
            }).collect();
            fis.reload(pal.paint(hex("#221f20"), 0.1), 0.6);
            c.drag(&mut fis, &Gesture::new(pts).pressure(0.5, 0.1).ramps(0.1, 0.6).shake(0.6), Some(&all));
        }
        let mut b = Held::new(Tool::round_sable(2.0), 45);
        for k in 0..420 {
            let x = rng.range(370.0, 605.0);
            let y = rng.range(385.0, 482.0);
            if all.sample(x, y) < 0.9 {
                continue;
            }
            if k % 5 == 3 {
                continue;
            }
            // lichen a shade off the stone, pits darker
            let under = c.sample(x, y);
            let want = match k % 5 {
                0 => shift(under, 0.03, -0.004, 0.012),
                1 => shift(under, 0.02, 0.0, -0.004),
                _ => shift(under, -0.05, 0.0, 0.0),
            };
            b.reload(pal.paint(want, 0.1), 0.45);
            c.touch(&mut b, &Touch::at(x, y).pressure(rng.range(0.15, 0.5)).drag(rng.range(-1.0, 1.0), rng.range(-0.4, 0.4)).twist(0.5), Some(&all));
        }
        // the crack under the capstone's edge and the joints: dark lines
        let mut r = Held::new(Tool::rigger(0.8), 46);
        let bot: Vec<(f32, f32)> = (0..=12).map(|i| {
            let x = 410.0 + i as f32 * 13.0;
            (x, cap_bot(x) - 0.8)
        }).collect();
        r.load(pal.paint(hex("#232226"), 0.1), 0.8);
        c.drag(&mut r, &Gesture::new(bot).pressure(0.6, 0.4).ramps(0.1, 0.3).shake(0.6), None);
        c.dry();
        // snow lying on the capstone and in the ledges: a soft irregular cap
        let cap_snow = cap_m.clone().mul_fn(move |x, y| {
            let u = (x - 490.0) / 84.0;
            let depth = 2.5 + 10.0 * (1.0 - u * u).max(0.0).powf(1.5) + 1.5 * (x / 17.0).sin() + 3.0 * stone_f.get(x * 0.6, 7.0);
            1.0 - smoothstep(cap_top(x) + depth - 1.0, cap_top(x) + depth + 1.0, y)
        });
        let snow_col = move |x: f32, y: f32| {
            let t = smoothstep(cap_top(x), cap_top(x) + 6.0, y);
            mix(hex("#dcd8da"), hex("#b9bac6"), t, Mix::Light)
        };
        let snow_hd = st.detail().color(snow_col).medium(0.05).angle(|x, _| 0.1 * ((x - 490.0) / 70.0)).length(5.0, 15.0).coverage(4.0);
        c.work(&cap_snow, &snow_hd, 47);
        let up_snow = uprights.clone().mul_fn(move |x, y| {
            let t = if x < 490.0 { 426.0 } else { 428.0 };
            1.0 - smoothstep(t + 0.8, t + 2.4, y)
        });
        c.work(&up_snow, &st.detail().color(|_, _| hex("#c9c8d0")).length(3.0, 8.0).coverage(3.5), 48);
        let b_snow = boulders.clone().mul_fn(|x, y| {
            let cy = if x < 400.0 { if x < 380.0 { 475.0 } else { 467.0 } } else if x < 500.0 { 473.0 } else { 476.0 };
            1.0 - smoothstep(cy - 1.0 + 1.2 * (x / 5.0).sin(), cy + 1.0 + 1.2 * (x / 5.0).sin(), y)
        });
        c.work(&b_snow, &st.detail().color(|_, _| hex("#cdcad0")).length(3.0, 8.0).coverage(3.5), 49);
        c.dry();
    }

    // ---- the oaks
    let oak_seed: u64 = std::env::var("OAK_SEED").ok().and_then(|v| v.parse().ok()).unwrap_or(5);
    let big = my_oak((612.0, knoll(612.0) + 4.0), 360.0, -0.04, false, o.seed * 7 + oak_seed);
    let small = my_oak((336.0, knoll(336.0) + 3.0), 200.0, 0.1, true, o.seed * 7 + 101);
    if std::env::var("OAK_STATS").is_ok() {
        for (name, sk) in [("big", &big), ("small", &small)] {
            let mut by = [0usize; 6];
            let mut thin = 0;
            let mut dead = 0;
            for l in &sk.limbs {
                by[(l.order as usize).min(5)] += 1;
                if l.w.last().copied().unwrap_or(0.0) < 0.6 { thin += 1; }
                if l.dead { dead += 1; }
            }
            eprintln!("{name}: {} limbs by order {:?}, tips<0.6: {thin}, dead {dead}, trunk w {:.1}", sk.limbs.len(), by, sk.limbs[0].w[0]);
        }
    }
    if o.stage("oaks", &mut c, &mut rng) {
        let live = pal.paint(hex("#2a2522"), 0.3);
        let dead = pal.paint(hex("#35302d"), 0.3);
        let snow_p = pal.paint(hex("#d6d4d8"), 0.05).with_stiff(0.8);
        // the small one behind first (farther), a touch grayer in the air
        let live_f = pal.paint(hex("#38333a"), 0.3);
        let dead_f = pal.paint(hex("#47434a"), 0.3);
        let rim = pal.paint(hex("#6f6259"), 0.2);
        let glow = (GLOW_X, HOR + 30.0);
        paint_tree(&mut c, &small, live_f, dead_f, rim, snow_p, glow, &mut rng, 500);
        paint_tree(&mut c, &big, live, dead, rim, snow_p, glow, &mut rng, 900);
        c.dry();
        // snow drifted against the foot of each trunk
        for (x, w) in [(612.0f32, 30.0f32), (336.0, 16.0)] {
            let foot = Mask::from_fn(f, move |px, py| {
                let top = knoll(px) - 3.0 + 3.0 * ((px - x) / w).powi(2);
                smoothstep(top - 0.6, top + 0.6, py) * (1.0 - smoothstep(w * 0.9, w * 1.3, (px - x).abs())) * (1.0 - smoothstep(knoll(px) + 14.0, knoll(px) + 16.0, py))
            });
            c.work(&foot, &st.detail().color(move |px, py| snow(px, py, &drift)).angle(|_, _| 0.0).length(4.0, 10.0).coverage(3.5), 60 + x as u64);
        }
        c.dry();
    }

    // ---- the man in the snow, back turned, and his shadowless footprints
    if o.stage("figure", &mut c, &mut rng) {
        let (fx, fy) = (402.0f32, 598.0f32);
        let s = 46.0; // height
        let p = |x: f32, y: f32| (fx + x * s, fy + y * s);
        // footprints first: he came in from the lower left; each print a
        // small dent, darker and bluer than the snow, with its lit rim
        let snow_pal = pal.only(&["lead white", "cobalt blue", "pale smalt", "yellow ochre", "raw umber"]);
        let mut t = Held::new(Tool { point: 0.5, ..Tool::round_sable(3.0) }, 75);
        for k in 0..18 {
            let u = k as f32 / 17.0;
            let side = if k % 2 == 0 { -1.0 } else { 1.0 };
            let cx = fx - 6.0 - u * 170.0 + 7.0 * (u * 7.0).sin();
            let cy = fy + 2.0 + u * 100.0;
            let (dx, dy) = (-170.0f32, 100.0f32);
            let dl = (dx * dx + dy * dy).sqrt();
            let (nx, ny) = (-dy / dl, dx / dl);
            let px = cx + side * nx * (1.5 + 1.5 * u);
            let py = cy + side * ny * (1.5 + 1.5 * u);
            let sz = 1.0 + 1.6 * u;
            let want = shift(c.sample(px, py), -0.075, 0.0, -0.018);
            let pn = c.aim(&snow_pal, want, (px, py), 1.5, 0.15, 1.2);
            // heel and toe, the toe toward him
            for (j, (q, pr)) in [(0.0f32, 0.36f32), (1.3, 0.3)].iter().enumerate() {
                let hx = px - dx / dl * sz * q * 1.4;
                let hy = py - dy / dl * sz * q * 1.4;
                t.reload(pn, 0.7);
                let _ = j;
                c.touch(&mut t, &Touch::at(hx, hy).pressure(pr + 0.12 * sz).drag(-dx / dl * sz * 0.8, -dy / dl * sz * 0.8).twist(0.3), None);
            }
        }
        c.wait(10.0);
        let coat = Mask::from_shape(f, Shape::new().smooth_poly(&[
            p(-0.10, -0.835),
            p(0.10, -0.835),
            p(0.14, -0.79),
            p(0.135, -0.62),
            p(0.115, -0.47),
            p(0.16, -0.20),
            p(0.17, -0.165),
            p(0.0, -0.15),
            p(-0.17, -0.165),
            p(-0.16, -0.20),
            p(-0.115, -0.47),
            p(-0.135, -0.62),
            p(-0.14, -0.79),
        ]));
        c.work(&coat, &st.detail().color(|_, y| mix(hex("#23232b"), hex("#2c2a2c"), smoothstep(560.0, 595.0, y), Mix::Light)).angle(|_, _| 1.57).length(3.0, 9.0).coverage(4.5), 70);
        let dark = pal.paint(hex("#1d1b1f"), 0.1);
        let mut b = Held::new(Tool { point: 0.6, ..Tool::round_sable(2.0) }, 71);
        // trousers below the hem and boots
        for dx in [-0.055f32, 0.06] {
            b.reload(dark, 0.8);
            c.drag(&mut b, &Gesture::new(vec![p(dx, -0.17), p(dx * 1.05, -0.02)]).pressure(0.6, 0.6).ramps(0.05, 0.05).shake(0.2), None);
            b.reload(pal.paint(hex("#18161a"), 0.1), 0.8);
            c.drag(&mut b, &Gesture::new(vec![p(dx * 1.05, -0.025), p(dx * 1.05 + 0.02, -0.005)]).pressure(0.75, 0.6).ramps(0.05, 0.1).shake(0.1), None);
        }
        // the right arm, bent, a touch lighter so it separates from the back
        b.reload(pal.paint(hex("#2e2e37"), 0.1), 0.8);
        c.drag(&mut b, &Gesture::new(vec![p(0.125, -0.77), p(0.165, -0.64), p(0.19, -0.52)]).pressure(0.75, 0.6).ramps(0.05, 0.1).shake(0.2), None);
        // folds down the back catching the cool sky
        let mut r = Held::new(Tool { point: 1.0, ..Tool::rigger(0.8) }, 73);
        for (x0, x1) in [(-0.03f32, -0.05f32), (0.05, 0.08), (-0.08, -0.12)] {
            r.reload(pal.paint(hex("#3a3b47"), 0.1), 0.5);
            c.drag(&mut r, &Gesture::new(vec![p(x0, -0.6), p(x1, -0.2)]).pressure(0.35, 0.15).ramps(0.3, 0.4).shake(0.3), None);
        }
        // collar, head, and a top hat
        let mut hb = Held::new(Tool { point: 0.5, ..Tool::round_sable(2.6) }, 72);
        hb.load(dark, 0.8);
        c.drag(&mut hb, &Gesture::new(vec![p(-0.075, -0.835), p(0.075, -0.835)]).pressure(0.7, 0.7).ramps(0.05, 0.05).shake(0.1), None);
        hb.reload(pal.paint(hex("#2a2320"), 0.1), 0.8);
        c.touch(&mut hb, &Touch::at(fx, fy - 0.875 * s).pressure(0.6), None);
        let hat = Mask::from_shape(f, Shape::new().poly(&[p(-0.058, -0.915), p(0.058, -0.915), p(0.052, -1.02), p(-0.052, -1.02)]));
        c.work(&hat, &st.detail().color(|_, _| hex("#19171a")).angle(|_, _| 1.57).length(2.0, 5.0).coverage(4.5), 76);
        r.reload(dark, 0.8);
        c.drag(&mut r, &Gesture::new(vec![p(-0.1, -0.912), p(0.0, -0.918), p(0.1, -0.91)]).pressure(0.8, 0.8).ramps(0.05, 0.05).shake(0.1), None);
        // his stick, from the hand to the snow
        r.reload(pal.paint(hex("#2e2823"), 0.1), 0.8);
        c.drag(&mut r, &Gesture::new(vec![p(0.2, -0.54), p(0.285, 0.005)]).pressure(0.55, 0.45).ramps(0.05, 0.1).shake(0.3), None);
        // the glow side (his left, our left) takes a faint warm rim
        let mut rim = Held::new(Tool { point: 1.0, ..Tool::rigger(0.6) }, 74);
        rim.load(pal.paint(hex("#6d625c"), 0.1), 0.5);
        c.drag(&mut rim, &Gesture::new(vec![p(-0.1, -0.83), p(-0.14, -0.78), p(-0.135, -0.62), p(-0.12, -0.48)]).pressure(0.3, 0.15).ramps(0.2, 0.5).shake(0.3), None);
        rim.reload(pal.paint(hex("#6d625c"), 0.1), 0.4);
        c.drag(&mut rim, &Gesture::new(vec![p(-0.05, -1.018), p(-0.056, -0.93)]).pressure(0.25, 0.1).ramps(0.2, 0.5).shake(0.2), None);
        c.dry();
    }

    // ---- crows: some in the big oak, two flying toward the woods
    if o.stage("crows", &mut c, &mut rng) {
        let crow = pal.paint(hex("#1b1a1c"), 0.1);
        let mut b = Held::new(Tool::round_sable(1.8), 80);
        // perched: on level stretches of the upper limbs, spaced apart
        let mut perches: Vec<(f32, f32)> = Vec::new();
        for l in big.limbs.iter().filter(|l| l.order >= 1 && !l.root) {
            let lp = gnarled(l);
            for i in 1..lp.len().saturating_sub(1) {
                let p = lp[i];
                if p.1 < 330.0 && l.dir(i).1.abs() < 0.3 && l.w[i] > 1.2 && l.w[i] < 6.0 && perches.iter().all(|q| (q.0 - p.0).abs() + (q.1 - p.1).abs() > 45.0) {
                    perches.push((p.0, p.1 - l.w[i] * 0.45));
                }
            }
        }
        for (k, p) in perches.iter().take(3).enumerate() {
            let sz = 4.2;
            let face = if k % 2 == 0 { 1.0 } else { -1.0 };
            b.reload(crow, 0.9);
            // body: from the tail down behind to the shoulders, then the head
            c.drag(&mut b, &Gesture::new(vec![(p.0 - face * sz * 1.0, p.1 - sz * 0.2), (p.0 - face * sz * 0.1, p.1 - sz * 0.9), (p.0 + face * sz * 0.35, p.1 - sz * 1.5)]).pressure(0.95, 0.7).ramps(0.1, 0.2).shake(0.3), None);
            c.touch(&mut b, &Touch::at(p.0 + face * sz * 0.55, p.1 - sz * 1.75).pressure(0.6), None);
            // the beak
            let mut r = Held::new(Tool::rigger(0.6), 81 + k as u64);
            r.load(crow, 0.6);
            c.drag(&mut r, &Gesture::new(vec![(p.0 + face * sz * 0.7, p.1 - sz * 1.75), (p.0 + face * sz * 1.25, p.1 - sz * 1.6)]).pressure(0.6, 0.0).ramps(0.05, 0.7).shake(0.2), None);
        }
        let mut w = Held::new(Tool { point: 1.0, ..Tool::rigger(1.1) }, 85);
        for &(x, y, sz, beat) in &[(712.0f32, 196.0f32, 7.0f32, 0.5f32), (748.0, 224.0, 6.0, -0.3)] {
            for side in [-1.0f32, 1.0] {
                w.reload(crow, 0.7);
                let tip = (x + side * sz, y - sz * beat);
                let mid = (x + side * sz * 0.45, y - sz * 0.25 - sz * 0.25 * beat.abs());
                c.drag(&mut w, &Gesture::new(vec![(x, y), mid, tip]).pressure(0.85, 0.0).ramps(0.05, 0.7).shake(0.4), None);
            }
            b.reload(crow, 0.7);
            c.touch(&mut b, &Touch::at(x, y).pressure(0.45).drag(1.2, 0.2), None);
        }
        c.dry();
    }

    // ---- dry grass and weeds through the snow, flicked upward, last:
    // tufts along the bank's lip and in the near snow, a few tall stalks
    if o.stage("grass", &mut c, &mut rng) {
        let dens = Fbm::new(41, 3, 70.0);
        let paints = [
            pal.paint(hex("#4a4038"), 0.15),
            pal.paint(hex("#5d5249"), 0.15),
            pal.paint(hex("#7a6a58"), 0.15),
            pal.paint(hex("#2d2824"), 0.15),
        ];
        let mut rg = Held::new(Tool { point: 1.0, ..Tool::rigger(0.8) }, 90);
        let mut sb = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.3) }, 91);
        let mut n = 0usize;
        // tufts: (x, y, blades)
        let mut spots: Vec<(f32, f32, usize)> = Vec::new();
        for _ in 0..120 {
            let x = rng.range(-10.0, 1010.0);
            let y = bank(x) + rng.range(-2.0, 1.5);
            if dens.get(x, 0.0) > -0.1 {
                spots.push((x, y, 4 + (rng.f() * 12.0) as usize));
            }
        }
        for _ in 0..200 {
            let x = rng.range(-10.0, 1010.0);
            let y = rng.range(610.0, h + 5.0);
            if dens.get(x, y) > 0.05 {
                spots.push((x, y, 5 + (rng.f() * rng.f() * 18.0) as usize));
            }
        }
        for _ in 0..24 {
            let x = rng.range(360.0, 660.0);
            let y = knoll(x) + rng.range(3.0, 14.0);
            spots.push((x, y, 2 + (rng.f() * 4.0) as usize));
        }
        for &(x, y, blades) in &spots {
            if (x - 402.0).abs() < 22.0 && y > 540.0 && y < 612.0 {
                continue;
            }
            let near = smoothstep(470.0, h, y);
            let spread = 2.0 + blades as f32 * 0.5;
            for _ in 0..blades {
                let ht = (3.0 + 18.0 * rng.f() * rng.f() * rng.f() + 5.0 * rng.f()) * (0.3 + 1.2 * near);
                let lean = rng.range(-0.7, 0.7) * rng.f() + 0.12;
                let bx = x + rng.range(-spread, spread) * (0.4 + near);
                let by = y + rng.range(-1.0, 1.0) * (0.3 + near);
                let bend = rng.range(-0.15, 0.15);
                let pts = vec![(bx, by + 0.5), (bx + lean * ht * 0.3, by - ht * 0.5), (bx + (lean + bend) * ht, by - ht)];
                let fine = ht < 7.0 || n % 4 != 0;
                let hb = if fine { &mut rg } else { &mut sb };
                let pk = match n % 7 {
                    0 | 3 => 0,
                    1 | 5 => 1,
                    2 => 2,
                    _ => 3,
                };
                hb.reload(paints[pk], 0.55);
                c.drag(hb, &Gesture::new(pts).pressure(0.3 + 0.2 * near, 0.0).ramps(0.04, 0.85).shake(0.5), None);
                n += 1;
            }
        }
        // a few tall dead weed stalks in the foreground, with seed heads
        for &(x, y, ht) in &[(88.0f32, 690.0f32, 78.0f32), (104.0, 686.0, 55.0), (742.0, 678.0, 64.0), (905.0, 694.0, 88.0), (921.0, 690.0, 50.0)] {
            let lean = rng.range(-0.12, 0.2);
            let pts: Vec<(f32, f32)> = (0..=6).map(|i| {
                let t = i as f32 / 6.0;
                (x + lean * ht * t * t + 0.8 * (t * 7.0).sin(), y - ht * t)
            }).collect();
            rg.reload(paints[3], 0.7);
            c.drag(&mut rg, &Gesture::new(pts.clone()).pressure(0.8, 0.3).ramps(0.03, 0.3).shake(0.4), None);
            // side shoots and heads
            for k in 2..=6 {
                let p = pts[k];
                let side = if k % 2 == 0 { 1.0 } else { -1.0 };
                let len = ht * 0.12 * rng.range(0.6, 1.2);
                rg.reload(paints[0], 0.5);
                let tip = (p.0 + side * len, p.1 - len * 0.7);
                c.drag(&mut rg, &Gesture::new(vec![p, tip]).pressure(0.55, 0.05).ramps(0.05, 0.6).shake(0.4), None);
                sb.reload(paints[3], 0.6);
                c.touch(&mut sb, &Touch::at(tip.0, tip.1).pressure(0.5), None);
            }
            sb.reload(paints[3], 0.6);
            let top = pts[6];
            c.touch(&mut sb, &Touch::at(top.0, top.1).pressure(0.7), None);
        }
        c.dry();
    }

    // ---- the whole darkened a little toward the edges (his advice to
    // Carus for a moonlit picture), then varnish and age
    if o.stage("veil", &mut c, &mut rng) {
        let hgt = h;
        c.glaze(&Pigment::transparent(hex("#86807e")), None, move |x, y| {
            let dx = (x - 480.0) / 560.0;
            let dy = (y - hgt * 0.55) / (hgt * 0.62);
            0.55 * smoothstep(0.55, 1.25, (dx * dx + dy * dy).sqrt()) * (0.3 + 0.7 * smoothstep(0.0, hgt, y))
        });
    }

    // a picture about two hundred years old, but well kept: the network
    // finer in its grime than the default
    let aged = paint::Cracks::aged(0);
    let cracks = paint::Cracks { dirt: 0.12, grime: 0.3, depth_um: 8.0, cupping_um: 6.0, width_um: Some(12.0), ..aged };
    o.finish(&mut c, &mut rng, &Finish { cracks: Some(cracks), ..Finish::aged(st.relief) });
}
