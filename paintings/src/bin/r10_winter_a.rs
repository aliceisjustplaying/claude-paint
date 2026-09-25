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
use paint::{Fbm, Gesture, Habit, Held, Limb, Mask, Mix, Paint, Pigment, Rgb, Rng, Shape, Skeleton, Stipple, Style, Tool, Touch, gradient, hex, shift, smoothstep};
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
        &[(0.0, hex("#76849b")), (0.28, hex("#96a0ac")), (0.52, hex("#b7b8b3")), (0.72, hex("#d0c3b5")), (0.88, hex("#e0cdad")), (1.0, hex("#ebd5a8"))],
        t,
        Mix::Light,
    );
    let g = (-((x - GLOW_X) / 280.0).powi(2)).exp() * smoothstep(0.45, 1.0, t);
    let warm = mix(base, hex("#f1dca8"), 0.45 * g, Mix::Light);
    let away = (1.0 - (-((x - GLOW_X) / 420.0).powi(2)).exp()) * smoothstep(0.55, 1.0, t);
    mix(warm, hex("#c3bdb8"), 0.35 * away, Mix::Light)
}

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
    let far = mix(hex("#d2cac8"), hex("#dfd2c2"), g, Mix::Light);
    let base = mix(far, hex("#8e95a8"), d.powf(0.75), Mix::Light);
    // drifts: a height field stretched along the ground; slopes facing the
    // glow (west, left) catch light, hollows go blue
    let e = 1.5;
    let hx = |x: f32, y: f32| drift.get(x, y * 3.2);
    let slope = (hx(x + e, y) - hx(x - e, y)) / (2.0 * e);
    let up = (hx(x, y - e) - hx(x, y + e)) / (2.0 * e);
    let lit = (-slope * 70.0 + up * 30.0).clamp(-1.0, 1.0) * (0.3 + 0.7 * d);
    let hollow = drift.get(x, y * 3.2);
    // the bank: a lit lip, then its face turned from the sky in shadow
    let below = y - bank(x);
    let face = if below > 0.0 { (1.0 - below / 16.0).max(0.0).powf(1.5) } else { 0.0 };
    let lip = if below <= 0.0 { (1.0 + below / 5.0).max(0.0) } else { 0.0 };
    shift(base, 0.045 * lit - 0.04 * hollow - 0.07 * face + 0.03 * lip, 0.004 * face, -0.014 * lit.min(0.0) - 0.012 * hollow - 0.025 * face)
}

/// Capstone of the dolmen: top and bottom edges.
fn cap_top(x: f32) -> f32 {
    401.0 + 11.0 * ((x - 492.0) / 68.0).powi(2) + 1.3 * (x / 9.0).sin()
}
fn cap_bot(x: f32) -> f32 {
    430.0 + 3.0 * (x / 23.0).sin() + 4.0 * ((x - 470.0) / 80.0)
}

/// Paint one limb of a grown tree as the hand would: one movement from
/// where it springs to its tip, handing on to a finer brush where it thins.
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
        while i1 < n - 1 && l.w[i1] > w0 * 0.5 && i1 - i0 < 10 && l.dead_at(i1) == is_dead {
            i1 += 1;
        }
        let last = i1 == n - 1;
        let w1 = l.w[i1].max(0.3);
        let tool = if w0 < 1.3 {
            Tool::rigger((w0 * 1.4).max(0.5))
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
        h.load(if is_dead { dead } else { live }, 0.9);
        let release = if last && !l.broken { 0.45 } else { 0.08 };
        c.drag(&mut h, &Gesture::new(pts).pressure(pa, pb).ramps(0.04, release).shake(0.35), None);
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
        let a = if side < 0.0 { std::f32::consts::PI - 0.25 } else { 0.25 };
        let l = w0 * rng.range(0.8, 1.3);
        let pts = vec![(base.0 + side * w0 * 0.2, base.1 - w0 * 0.3), (base.0 + side * w0 * 0.2 + l * a.cos(), base.1 - w0 * 0.3 + l * a.sin())];
        limbs.push(Limb { z: vec![0.0; 2], age: vec![1; 2], w: vec![w0 * 0.55, w0 * 0.15], order: 1, parent: Some(0), at: 0, dead: false, dead_from: 2, broken: false, root: true, pts });
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
    let t = Tool::rigger((w * 1.3).clamp(0.5, 2.0));
    let pa = t.pressure_for(w).max(0.25);
    h.reload(p, 0.5);
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
                next += rng.range(4.0, 11.0) * (0.6 + 0.15 * w);
                let d = l.dir(i);
                let base_a = d.1.atan2(d.0);
                let side = if rng.f() < 0.5 { -1.0 } else { 1.0 };
                let dir = base_a + side * rng.range(0.55, 1.15);
                let len = rng.range(6.0, 16.0) * (0.6 + 0.25 * w.min(4.0)) * (sk.height / 260.0).sqrt();
                let tp = if l.dead_at(i - 1) { dead } else { live };
                claw(c, &mut th, tp, b, dir, len, (w * 0.5).clamp(0.4, 1.6), 1, rng);
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
                    (lp[q].0 + nx * l.w[q] * 0.36, lp[q].1 + ny * l.w[q] * 0.36)
                })
                .collect();
            let mut h = Held::new(Tool::rigger((w * 0.22).clamp(0.5, 2.2)), seed ^ (i as u64 * 7919 + j as u64));
            h.load(rim_p, 0.45);
            c.drag(&mut h, &Gesture::new(pts).pressure(0.45, 0.3).ramps(0.25, 0.4).shake(0.6), None);
            j = k + 1;
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
            if w < 0.9 || d.1.abs() > 0.8 || rng.f() < 0.3 {
                i += 1;
                continue;
            }
            // a run of segments while it stays level
            let mut j = i + 1;
            while j < n - 1 && l.dir(j).1.abs() < 0.8 && j - i < 5 {
                j += 1;
            }
            let pts: Vec<(f32, f32)> = (i..=j).map(|q| (lp[q].0, lp[q].1 - l.w[q] * 0.42)).collect();
            let sw = (w * 0.45).clamp(0.5, 3.5);
            let t = Tool::rigger(sw);
            h = Held::new(t, seed ^ (i as u64 * 977 + l.pts.len() as u64));
            h.load(snow_p, 0.7);
            c.drag(&mut h, &Gesture::new(pts).pressure(0.55, 0.25).ramps(0.2, 0.4).shake(0.5), None);
            i = j + 1;
        }
    }
    let _ = h;
}

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
            let x = 414.0 + i as f32 * 10.5;
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
        // thin streaks of cloud low in the west, violet-gray, their lower
        // edges touched by the glow
        let streak = Fbm::new(21, 4, 160.0);
        let cloud_cov = move |x: f32, y: f32| {
            let band = (-((y - 352.0) / 22.0).powi(2)).exp() + 0.7 * (-((y - 300.0) / 12.0).powi(2)).exp() + 0.5 * (-((y - 398.0) / 9.0).powi(2)).exp();
            let n = streak.get(x * 0.25, y * 3.0);
            (band * smoothstep(-0.05, 0.35, n) * 2.2 * (1.0 - smoothstep(620.0, 900.0, x))).max(0.0)
        };
        let cloud = Stipple::new(Tool::stippler(2.4)).mixed(&sky_pal, 0.55).color_over(|_, _, u| shift(u, -0.07, 0.012, -0.01)).coverage(cloud_cov).pressure(0.45, 0.8).drag(1.5, Some(0.0)).dips(18, 0.35, 0.6);
        c.stipple(&sky_m, &cloud, 15);
        c.dry();
    }

    // ---- moon and evening star
    if o.stage("moon", &mut c, &mut rng) {
        let r = 10.0;
        // the lit limb faces the sun, below the horizon toward the glow
        let sun_dir = ((GLOW_X - MOON.0), (HOR + 40.0 - MOON.1));
        let a0 = sun_dir.1.atan2(sun_dir.0);
        let moon_p = pal.paint(hex("#f6efd6"), 0.05).with_hiding(0.97).with_stiff(0.8);
        let mut b = Held::new(Tool::round_sable(2.4), 77);
        for pass in 0..3 {
            let inset = pass as f32 * 0.9;
            let pts: Vec<(f32, f32)> = (0..=16)
                .map(|i| {
                    let t = i as f32 / 16.0;
                    let a = a0 + (t - 0.5) * 2.6;
                    let rr = r - inset * (1.0 - (2.0 * t - 1.0).powi(2)).sqrt() - 0.6;
                    (MOON.0 + rr * a.cos(), MOON.1 + rr * a.sin())
                })
                .collect();
            b.reload(moon_p, 0.8);
            c.drag(&mut b, &Gesture::new(pts).pressure(0.35, 0.35).swell(vec![0.2, 1.0, 0.2]).ramps(0.3, 0.3).shake(0.2), None);
        }
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
            if w > 4.0 && rng.f() < 0.55 {
                let ht = w + rng.range(2.0, 8.0);
                let base = far_line(x) - w * 0.5;
                b.reload(tree_p, 0.6);
                c.drag(&mut b, &Gesture::new(vec![(x, base), (x + 0.2, base - ht)]).pressure(0.7, 0.0).ramps(0.05, 0.7).shake(0.3), None);
            }
            x += rng.range(2.5, 7.0);
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
    let drift = Fbm::new(5, 5, 170.0);
    if o.stage("field", &mut c, &mut rng) {
        let snow_pal = pal.only(&["lead white", "cobalt blue", "pale smalt", "yellow ochre", "red earth", "raw umber", "bone black"]);
        // the lay-in: long strokes following the lie of the ground, curving
        // over the knoll
        let ground_angle = move |x: f32, y: f32| {
            let slope = (knoll(x + 4.0) - knoll(x - 4.0)) / 8.0;
            let fade = 1.0 - smoothstep(0.0, 90.0, y - knoll(x));
            (slope * fade).atan() + 0.05 * (x / 170.0 + y / 90.0).sin()
        };
        let lay = st.broad().palette(&snow_pal).color(move |x, y| snow(x, y, &drift)).angle(ground_angle).length(60.0, 160.0).coverage(4.5).medium(0.2);
        c.work(&field_m, &lay, 31);
        if let Some(b) = st.blend() {
            c.work(&field_m, &b.angle(ground_angle).coverage(2.0), 32);
        }
        c.wait(20.0);
        // body snow in the foreground: shorter strokes, stiffer paint, the
        // drifts modeled
        let near_m = field_m.clone().mul_fn(|_, y| smoothstep(520.0, 600.0, y));
        let body = st.body().palette(&snow_pal).color(move |x, y| snow(x, y, &drift)).angle(ground_angle).length(25.0, 70.0).coverage(2.5).medium(0.12);
        c.work(&near_m, &body, 33);
        c.dry();
    }

    // ---- the dolmen
    let stone_f = Fbm::new(9, 4, 14.0);
    let cap_m = Mask::from_fn(f, move |x, y| {
        let inx = smoothstep(412.0, 418.0, x) * (1.0 - smoothstep(556.0, 563.0, x));
        inx * smoothstep(cap_top(x) - 0.7, cap_top(x) + 0.7, y) * (1.0 - smoothstep(cap_bot(x) - 0.7, cap_bot(x) + 0.7, y))
    })
    .roughen(3, 12.0, 1.4, 0.5);
    let uprights = Mask::from_shape(
        f,
        Shape::new()
            .poly(&[(424.0, 428.0), (449.0, 430.0), (452.0, 462.0), (419.0, 464.0)])
            .add(Shape::new().poly(&[(517.0, 432.0), (546.0, 430.0), (553.0, 468.0), (514.0, 466.0)])),
    )
    .roughen(4, 10.0, 1.3, 0.5)
    .mul(&Mask::from_fn(f, move |x, y| 1.0 - smoothstep(kn(x) - 1.5, kn(x) + 1.0, y)));
    let back_up = Mask::from_shape(f, Shape::new().poly(&[(476.0, 432.0), (497.0, 433.0), (499.0, 462.0), (474.0, 462.0)]))
        .roughen(5, 10.0, 1.0, 0.5)
        .mul(&Mask::from_fn(f, move |x, y| 1.0 - smoothstep(kn(x) - 1.5, kn(x) + 1.0, y)));
    let boulders = Mask::from_shape(f, Shape::new().ellipse(396.0, 466.0, 13.0, 6.0).add(Shape::new().ellipse(578.0, 474.0, 10.0, 5.0)).add(Shape::new().ellipse(458.0, 471.0, 8.0, 4.0)))
        .roughen(6, 8.0, 1.2, 0.5)
        .mul(&Mask::from_fn(f, move |_, y| 1.0 - smoothstep(0.0, 1.0, 0.0 * y)));
    if o.stage("dolmen", &mut c, &mut rng) {
        // the hollow under the capstone: deep, cool shadow
        let under_m = Mask::from_shape(f, Shape::new().poly(&[(447.0, 430.0), (518.0, 433.0), (516.0, 462.0), (450.0, 461.0)]))
            .roughen(8, 12.0, 1.5, 0.6)
            .mul(&Mask::from_fn(f, move |x, y| 1.0 - smoothstep(kn(x) - 1.0, kn(x) + 1.0, y)));
        c.work(&under_m, &st.detail().color(|_, y| mix(hex("#2e2d33"), hex("#4a4a55"), smoothstep(440.0, 462.0, y), Mix::Light)).angle(|_, _| 0.0).length(4.0, 12.0).coverage(4.0), 40);
        // the far upright, in the shadow under the stone
        c.work(&back_up, &st.detail().color(move |x, y| shift(hex("#45434a"), 0.03 * stone_f.get(x, y), 0.0, 0.0)).angle(|_, _| 1.5).length(4.0, 10.0).coverage(4.0), 41);
        // stones: granite, dark in the dusk; the west faces catch a little
        // warm light from the glow, the tops cool light from the sky
        let stone = move |x: f32, y: f32, top: f32, left: f32, right: f32| {
            let n = stone_f.get(x, y);
            let west = 1.0 - smoothstep(left, left + 7.0, x);
            let upper = 1.0 - smoothstep(top, top + 8.0, y);
            let east = smoothstep(right - 8.0, right, x);
            let base = mix(hex("#4b4847"), hex("#5a5550"), 0.5 + 0.5 * n, Mix::Pigment);
            let b = mix(base, hex("#857563"), 0.55 * west, Mix::Light);
            let b = mix(b, hex("#6e727c"), 0.45 * upper, Mix::Light);
            shift(b, -0.03 * east, 0.0, -0.005 * east)
        };
        let up_hd = st.detail().color(move |x, y| if x < 490.0 { stone(x, y, 434.0, 424.0, 450.0) } else { stone(x, y, 436.0, 516.0, 549.0) }).angle(|_, _| 1.45).angle_jitter(0.25).length(5.0, 14.0).coverage(4.0);
        c.work(&uprights, &up_hd, 42);
        let cap_hd = st.detail().color(move |x, y| stone(x, y, cap_top(x) + 3.0, 415.0, 560.0)).angle(|x, _| 0.08 * ((x - 490.0) / 70.0)).angle_jitter(0.2).length(6.0, 18.0).coverage(4.0);
        c.work(&cap_m, &cap_hd, 43);
        c.work(&boulders, &st.detail().color(move |x, y| stone(x, y, 460.0, 390.0, 590.0)).angle(|_, _| 0.1).length(3.0, 8.0).coverage(4.0), 44);
        c.wait(30.0);
        // lichen and grain: small touches, dark pits and pale flecks
        let mut b = Held::new(Tool::round_sable(1.0), 45);
        let all = cap_m.clone().union(&uprights).union(&boulders);
        for k in 0..260 {
            let x = rng.range(410.0, 590.0);
            let y = rng.range(398.0, 476.0);
            if all.sample(x, y) < 0.8 {
                continue;
            }
            let col = if k % 3 == 0 { hex("#8a8674") } else { hex("#35322f") };
            b.reload(pal.paint(col, 0.1), 0.5);
            c.touch(&mut b, &Touch::at(x, y).pressure(rng.range(0.25, 0.55)).drag(rng.range(-0.8, 0.8), rng.range(-0.3, 0.3)), None);
        }
        // the crack under the capstone's edge and the joints: dark lines
        let mut r = Held::new(Tool::rigger(0.8), 46);
        let bot: Vec<(f32, f32)> = (0..=12).map(|i| {
            let x = 418.0 + i as f32 * 11.5;
            (x, cap_bot(x) - 0.8)
        }).collect();
        r.load(pal.paint(hex("#232226"), 0.1), 0.8);
        c.drag(&mut r, &Gesture::new(bot).pressure(0.6, 0.4).ramps(0.1, 0.3).shake(0.6), None);
        c.dry();
        // snow lying on the capstone and in the ledges: a soft irregular cap
        let cap_snow = cap_m.clone().mul_fn(move |x, y| {
            let depth = 4.5 + 2.5 * (x / 13.0).sin() + 1.5 * stone_f.get(x * 2.0, 0.0);
            1.0 - smoothstep(cap_top(x) + depth - 1.0, cap_top(x) + depth + 1.0, y)
        });
        let snow_col = move |x: f32, y: f32| {
            let t = smoothstep(cap_top(x), cap_top(x) + 6.0, y);
            mix(hex("#dcd8da"), hex("#b9bac6"), t, Mix::Light)
        };
        let snow_hd = st.detail().color(snow_col).medium(0.05).angle(|x, _| 0.1 * ((x - 490.0) / 70.0)).length(5.0, 15.0).coverage(4.0);
        c.work(&cap_snow, &snow_hd, 47);
        let up_snow = uprights.clone().mul_fn(move |x, y| {
            let t = if x < 490.0 { 431.0 } else { 433.0 };
            1.0 - smoothstep(t + 1.5, t + 3.5, y)
        });
        c.work(&up_snow, &st.detail().color(|_, _| hex("#c9c8d0")).length(3.0, 8.0).coverage(3.5), 48);
        let b_snow = boulders.clone().mul_fn(|_, y| 1.0 - smoothstep(465.0, 470.0, y));
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
        let live = pal.paint(hex("#2a2522"), 0.2);
        let dead = pal.paint(hex("#3b3632"), 0.2);
        let snow_p = pal.paint(hex("#d6d4d8"), 0.05).with_stiff(0.8);
        // the small one behind first (farther), a touch grayer in the air
        let live_f = pal.paint(hex("#38333a"), 0.2);
        let dead_f = pal.paint(hex("#47434a"), 0.2);
        let rim = pal.paint(hex("#6f6259"), 0.2);
        let glow = (GLOW_X, HOR + 30.0);
        paint_tree(&mut c, &small, live_f, dead_f, rim, snow_p, glow, &mut rng, 500);
        paint_tree(&mut c, &big, live, dead, rim, snow_p, glow, &mut rng, 900);
        c.dry();
        // snow drifted against the foot of each trunk
        for (x, w) in [(612.0f32, 30.0f32), (336.0, 16.0)] {
            let foot = Mask::from_fn(f, move |px, py| {
                let top = knoll(px) - 3.0 + 3.0 * ((px - x) / w).powi(2);
                smoothstep(top - 0.6, top + 0.6, py) * (1.0 - smoothstep(w * 0.6, w, (px - x).abs())) * (1.0 - smoothstep(knoll(px) + 4.0, knoll(px) + 6.0, py))
            });
            c.work(&foot, &st.detail().color(move |px, py| snow(px, py, &drift)).angle(|_, _| 0.0).length(4.0, 10.0).coverage(3.5), 60 + x as u64);
        }
        c.dry();
    }

    // ---- the man in the snow, back turned, and his shadowless footprints
    if o.stage("figure", &mut c, &mut rng) {
        let (fx, fy) = (402.0f32, 598.0f32);
        let s = 46.0; // height
        let coat = Mask::from_shape(f, Shape::new().smooth_poly(&[
            (fx - 0.10 * s, fy - 0.80 * s),
            (fx + 0.10 * s, fy - 0.80 * s),
            (fx + 0.15 * s, fy - 0.55 * s),
            (fx + 0.19 * s, fy - 0.12 * s),
            (fx + 0.03 * s, fy - 0.10 * s),
            (fx - 0.17 * s, fy - 0.12 * s),
            (fx - 0.15 * s, fy - 0.55 * s),
        ]));
        c.work(&coat, &st.detail().color(|_, y| mix(hex("#24232a"), hex("#2f2c2c"), smoothstep(560.0, 595.0, y), Mix::Light)).angle(|_, _| 1.57).length(3.0, 9.0).coverage(4.5), 70);
        let dark = pal.paint(hex("#1f1d20"), 0.1);
        let mut b = Held::new(Tool::round_sable(2.2), 71);
        // legs and boots under the coat
        for dx in [-0.06f32, 0.07] {
            b.reload(dark, 0.8);
            c.drag(&mut b, &Gesture::new(vec![(fx + dx * s, fy - 0.14 * s), (fx + dx * s * 1.2, fy - 0.01 * s)]).pressure(0.55, 0.6).ramps(0.05, 0.1).shake(0.2), None);
        }
        // head and hat (a low-crowned hat), a collar
        let mut hb = Held::new(Tool::round_sable(3.0), 72);
        hb.load(pal.paint(hex("#2b2522"), 0.1), 0.8);
        c.touch(&mut hb, &Touch::at(fx, fy - 0.86 * s).pressure(0.75), None);
        hb.reload(dark, 0.8);
        c.drag(&mut hb, &Gesture::new(vec![(fx - 0.09 * s, fy - 0.9 * s), (fx + 0.09 * s, fy - 0.9 * s)]).pressure(0.6, 0.6).ramps(0.05, 0.1).shake(0.2), None);
        c.drag(&mut hb, &Gesture::new(vec![(fx - 0.04 * s, fy - 0.93 * s), (fx + 0.04 * s, fy - 0.95 * s)]).pressure(0.7, 0.7).ramps(0.05, 0.1).shake(0.2), None);
        // his stick, planted to the right
        let mut r = Held::new(Tool::rigger(0.7), 73);
        r.load(pal.paint(hex("#2e2823"), 0.1), 0.8);
        c.drag(&mut r, &Gesture::new(vec![(fx + 0.17 * s, fy - 0.5 * s), (fx + 0.27 * s, fy + 0.01 * s)]).pressure(0.7, 0.6).ramps(0.05, 0.1).shake(0.3), None);
        // a cool rim of sky light on his shoulders
        let mut rim = Held::new(Tool::rigger(0.6), 74);
        rim.load(pal.paint(hex("#7c7f8c"), 0.1), 0.6);
        c.drag(&mut rim, &Gesture::new(vec![(fx - 0.12 * s, fy - 0.74 * s), (fx - 0.02 * s, fy - 0.80 * s), (fx + 0.1 * s, fy - 0.78 * s)]).pressure(0.4, 0.3).ramps(0.2, 0.3).shake(0.3), None);
        // footprints leading back to him from the lower left: blue hollows
        let fp = pal.paint(hex("#9097a8"), 0.15);
        let mut t = Held::new(Tool::round_sable(2.6), 75);
        for k in 0..16 {
            let u = k as f32 / 15.0;
            let px = fx - 12.0 - u * 150.0 + 6.0 * (u * 9.0).sin() + if k % 2 == 0 { -2.5 } else { 2.5 };
            let py = fy + 4.0 + u * 95.0;
            let sz = 0.3 + 0.35 * u;
            t.reload(fp, 0.6);
            c.touch(&mut t, &Touch::at(px, py).pressure(sz).drag(1.5 + 2.0 * u, 0.0), None);
        }
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
        let mut w = Held::new(Tool::rigger(1.0), 85);
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
            pal.paint(hex("#4d4236"), 0.15),
            pal.paint(hex("#5e5040"), 0.15),
            pal.paint(hex("#86705a"), 0.15),
            pal.paint(hex("#2e2822"), 0.15),
        ];
        let mut rg = Held::new(Tool::rigger(0.7), 90);
        let mut sb = Held::new(Tool::round_sable(1.1), 91);
        let mut n = 0usize;
        let mut spots: Vec<(f32, f32)> = Vec::new();
        for _ in 0..260 {
            let x = rng.range(-10.0, 1010.0);
            let y = bank(x) + rng.range(-3.0, 2.0);
            if dens.get(x, 0.0) > -0.05 {
                spots.push((x, y));
            }
        }
        for _ in 0..420 {
            let x = rng.range(-10.0, 1010.0);
            let y = rng.range(600.0, h + 5.0);
            if dens.get(x, y) > 0.1 {
                spots.push((x, y));
            }
        }
        for _ in 0..40 {
            let x = rng.range(380.0, 640.0);
            let y = knoll(x) + rng.range(2.0, 12.0);
            spots.push((x, y));
        }
        for &(x, y) in &spots {
            if (x - 402.0).abs() < 22.0 && y > 540.0 && y < 612.0 {
                continue;
            }
            let near = smoothstep(470.0, h, y);
            let blades = 2 + (rng.f() * 5.0) as usize;
            for _ in 0..blades {
                let ht = (3.0 + 10.0 * rng.f() * rng.f() + 4.0 * rng.f()) * (0.3 + 1.2 * near);
                let lean = rng.range(-0.5, 0.5) + 0.12;
                let bx = x + rng.range(-2.5, 2.5) * (0.4 + near);
                let bend = rng.range(-0.15, 0.15);
                let pts = vec![(bx, y + 0.5), (bx + lean * ht * 0.3, y - ht * 0.5), (bx + (lean + bend) * ht, y - ht)];
                let fine = ht < 6.0 || n % 3 != 0;
                let hb = if fine { &mut rg } else { &mut sb };
                let pk = match n % 7 {
                    0 | 3 => 0,
                    1 | 5 => 1,
                    2 => 2,
                    _ => 3,
                };
                hb.reload(paints[pk], 0.55);
                c.drag(hb, &Gesture::new(pts).pressure(0.65 + 0.2 * near, 0.0).ramps(0.04, 0.8).shake(0.5), None);
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
        c.glaze(&Pigment::transparent(hex("#8f7f6c")), None, move |x, y| {
            let dx = (x - 480.0) / 560.0;
            let dy = (y - hgt * 0.55) / (hgt * 0.62);
            0.55 * smoothstep(0.55, 1.25, (dx * dx + dy * dy).sqrt())
        });
    }

    o.finish(&mut c, &mut rng, &Finish::aged(st.relief));
}
