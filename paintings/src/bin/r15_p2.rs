//! r15_p2: "Oak in the snow at dusk".
//!
//! A winter evening after sunset. A single old oak, gnarled and part dead,
//! stands on a snow-covered rise at the left; the rise falls away to the
//! right into a wide snow plain. Far off, a low band of woods on the
//! horizon and a church spire against the afterglow. A trodden path leads
//! from the foreground to a lone walker heading toward the church. Crows
//! sit in the oak.

use paint::{
    Apply, Gesture, Held, Paint, Touch, Fbm, Ground, Handling, Lead, Mask, Mix, Pigment, Rgb, Rng, Shape, Stipple, Style, Tool, gradient, graphite,
    hex, shift, smoothstep,
};
use paint::Cracks;
use paintings::run::{Finish, Run};
use std::f32::consts::FRAC_PI_2;

const ASPECT: f32 = 1.4;
/// The far snow line (foot of the distant woods), units.
const HOR: f32 = 524.0;
/// Where the afterglow is brightest (the sun set just behind the woods).
const GLOW_X: f32 = 720.0;

fn lerpc(a: Rgb, b: Rgb, t: f32) -> Rgb {
    gradient(&[(0.0, a), (1.0, b)], t.clamp(0.0, 1.0), Mix::Linear)
}

/// The crest of the rise on the left (units): high at the left edge, falling
/// right until it sinks below the far snow line near the tree.
fn knoll_y(x: f32, n: &Fbm) -> f32 {
    468.0 + 100.0 * smoothstep(-60.0, 660.0, x) + 3.0 * n.get(x / 60.0, 0.5)
}

/// Top of the distant woods.
fn woods_y(x: f32, n: &Fbm) -> f32 {
    // copses with rounded crowns, gaps where the land shows between them
    let clumps = 6.0 * n.get(x / 30.0, 3.1) + 2.5 * n.get(x / 8.0, 7.7) + 1.2 * n.get(x / 3.0, 1.3);
    let rise = 8.0 * (-((x - 770.0) / 110.0).powi(2)).exp() + 5.0 * (-((x - 940.0) / 60.0).powi(2)).exp();
    HOR - 12.0 - rise - clumps.max(-9.0)
}

/// The far snow line: the foot of the woods, gently uneven.
fn far_y(x: f32, n: &Fbm) -> f32 {
    HOR + 1.6 * n.get(x / 45.0, 9.1) + 0.7 * n.get(x / 11.0, 2.2)
}

/// The trodden path: centre line and half-width by y.
fn path_x(y: f32) -> f32 {
    // control points from the lower edge up to where it is lost near the woods
    const P: [(f32, f32); 8] = [
        (714.0, 585.0),
        (690.0, 618.0),
        (660.0, 628.0),
        (630.0, 602.0),
        (600.0, 628.0),
        (574.0, 676.0),
        (556.0, 712.0),
        (536.0, 748.0),
    ];
    if y >= P[0].0 {
        return P[0].1;
    }
    for i in 0..P.len() - 1 {
        let (y0, x0) = P[i];
        let (y1, x1) = P[i + 1];
        if y <= y0 && y >= y1 {
            // Catmull-Rom through the neighbours
            let t = (y0 - y) / (y0 - y1);
            let xm = if i > 0 { P[i - 1].1 } else { 2.0 * x0 - x1 };
            let xp = if i + 2 < P.len() { P[i + 2].1 } else { 2.0 * x1 - x0 };
            let t2 = t * t;
            let t3 = t2 * t;
            return 0.5 * (2.0 * x0 + (-xm + x1) * t + (2.0 * xm - 5.0 * x0 + 4.0 * x1 - xp) * t2 + (-xm + 3.0 * x0 - 3.0 * x1 + xp) * t3);
        }
    }
    P[P.len() - 1].1
}
fn path_half(y: f32) -> f32 {
    // lost in the distance before it reaches the woods
    ((y - HOR) * 0.075).max(0.0) * smoothstep(538.0, 556.0, y)
}

fn sky_col(x: f32, y: f32) -> Rgb {
    let t = (y / HOR).clamp(0.0, 1.0);
    let base = gradient(
        &[
            (0.0, hex("#7c8aa3")),
            (0.28, hex("#9ea9b9")),
            (0.52, hex("#c3c4c4")),
            (0.72, hex("#dcd1bf")),
            (0.88, hex("#e8cba7")),
            (1.0, hex("#e7bb91")),
        ],
        t,
        Mix::Linear,
    );
    // the glow over the place the sun went down; cooler to the left
    let g = (-((x - GLOW_X) / 320.0).powi(2)).exp() * smoothstep(0.45, 1.0, t);
    let warm = lerpc(base, hex("#f1d7b0"), 0.45 * g);
    let cool = (1.0 - g) * smoothstep(0.4, 1.0, t) * smoothstep(500.0, 0.0, x);
    lerpc(warm, shift(warm, -0.01, -0.004, -0.02), cool)
}

/// Stratus bands low in the sky: (cx, cy, half length, half thickness).
const BANDS: [(f32, f32, f32, f32); 6] = [
    (560.0, 364.0, 260.0, 8.0),
    (800.0, 394.0, 300.0, 11.0),
    (430.0, 420.0, 220.0, 6.5),
    (860.0, 446.0, 190.0, 8.0),
    (640.0, 467.0, 280.0, 6.0),
    (330.0, 486.0, 150.0, 5.0),
];

/// Cloud cover (0..1) and position across the band (-1 top .. 1 bottom).
fn cloud_at(x: f32, y: f32, n: &Fbm) -> (f32, f32) {
    let mut best = (0.0f32, 0.0f32);
    for (k, &(cx, cy, hl, th)) in BANDS.iter().enumerate() {
        let sag = 5.0 * ((x - cx) / hl * 1.3 + k as f32).sin();
        let dx = (x - cx) / hl;
        let dy = (y - cy - sag) / (th * (0.7 + 0.6 * n.get01(x / 70.0, k as f32 * 3.3)));
        let v = smoothstep(1.0, 0.55, dx.abs()) * smoothstep(1.0, 0.25, dy.abs())
            * (0.55 + 0.45 * n.get01(x / 35.0, y / 8.0 + k as f32));
        if v > best.0 {
            best = (v, dy.clamp(-1.0, 1.0));
        }
    }
    best
}

fn cloud_col(x: f32, y: f32, across: f32) -> Rgb {
    // darker lavender grey on top, lit rose-gold underneath, warmer near the glow
    let g = (-((x - GLOW_X) / 300.0).powi(2)).exp();
    let top = lerpc(hex("#a9a0aa"), hex("#b7a3a0"), g);
    let under = lerpc(hex("#dcb9a4"), hex("#eebf96"), g);
    let hi = smoothstep(460.0, 380.0, y); // higher bands are cooler, less lit
    let c = lerpc(top, under, smoothstep(-0.3, 0.9, across));
    lerpc(c, shift(c, -0.02, 0.0, -0.02), hi * 0.6)
}

// ckpt: from oak
/// One limb, branch or twig of the oak: a polyline with a width (units) at
/// each point.
#[derive(Clone)]
#[allow(dead_code)]
struct Br {
    pts: Vec<(f32, f32)>,
    ws: Vec<f32>,
    depth: u32,
    /// Broken off: ends in a blunt stub.
    dead: bool,
}

/// The thinnest twig painted, units.
const WMIN: f32 = 0.32;

/// Grows a branch from `p` in direction `ang` (radians, canvas), `w` wide
/// at its root. Oak habit: the axis kinks at knots instead of curving,
/// thick limbs spread sideways, laterals leave at wide angles, twigs are
/// short and crooked; some limbs are broken off.
fn grow(out: &mut Vec<Br>, r: &mut Rng, p: (f32, f32), ang: f32, w: f32, len: f32, depth: u32, wmin: f32) {
    let dead = depth >= 2 && depth <= 3 && w > 2.5 && r.chance(0.14);
    let len = if dead { len * r.range(0.25, 0.55) } else { len };
    let step = (w * 0.9).clamp(1.6, 12.0);
    let n = ((len / step).ceil() as usize).max(2);
    let mut pts = vec![p];
    let mut ws = vec![w];
    let mut a = ang;
    let taper = if w > 6.0 { 0.16 } else { 0.22 };
    // the bend per step grows with the step, so thin twigs don't curl
    let kink = (0.075 * step.sqrt()).clamp(0.09, 0.24);
    // oak elbows: a sharp turn answered a step or two later
    let mut pending = 0.0f32;
    for i in 1..=n {
        a += pending;
        pending = 0.0;
        if w > 2.0 && r.chance(0.28) {
            let s = if r.chance(0.5) { 1.0 } else { -1.0 };
            let th = r.range(0.35, 0.7);
            a += s * th;
            pending = -s * th * r.range(0.6, 1.0);
        }
        let t = i as f32 / n as f32;
        // knots: now and then a sharp change of direction (per length)
        let knot = 0.55 - 0.3 * smoothstep(4.0, 14.0, w);
        let k = if r.chance(0.018 * step) { r.normal() * knot } else { r.normal() * kink };
        a += k;
        // heavy wood keeps to the direction it set out in
        a += wrap(ang - a) * 0.12 * smoothstep(3.0, 10.0, w);
        // a mild pull upward for thin wood, sideways for heavy limbs; long
        // thin shoots sag a little
        let up = -FRAC_PI_2;
        if w > 7.0 {
            let side = if a.cos() < 0.0 { -std::f32::consts::PI + 0.55 } else { -0.55 };
            a += wrap(side - a) * 0.05;
        } else {
            a += wrap(up - a) * 0.035;
        }
        // wood grows out and up: a branch that turns downward is pulled back
        let down = a.sin();
        if down > 0.2 {
            a += wrap(up - a).signum() * (down - 0.2) * 0.5;
        }
        let d = len / n as f32;
        let (x0, y0) = *pts.last().unwrap();
        pts.push((x0 + d * a.cos(), y0 + d * a.sin()));
        // swellings at knots
        let knot = 1.0 + 0.1 * r.normal().clamp(-1.5, 1.5) * smoothstep(1.5, 5.0, w);
        let dt = if dead { 1.0 - 0.45 * t * t } else { 1.0 };
        ws.push(w * (1.0 - taper * t) * knot * dt);
    }
    let wend = *ws.last().unwrap();
    // laterals along the branch
    let spacing = 2.6 * w + 5.0;
    let nl = (len / spacing).floor() as usize;
    let mut side = if r.chance(0.5) { 1.0 } else { -1.0 };
    let mut kids = Vec::new();
    for j in 0..nl {
        let t = ((j as f32 + r.range(0.3, 0.9)) / nl as f32).min(0.95);
        if dead && t > 0.8 {
            continue;
        }
        let i = ((t * n as f32) as usize).min(n - 1);
        let (x0, y0) = pts[i];
        let (x1, y1) = pts[i + 1];
        let dir = (y1 - y0).atan2(x1 - x0);
        let wi = ws[i];
        let cw = wi * r.range(0.35, 0.68);
        if r.chance(0.12) {
            side = -side;
            continue;
        }
        let ca = dir + side * r.range(0.5, 1.15);
        side = -side;
        kids.push(((x0 + (x1 - x0) * 0.5, y0 + (y1 - y0) * 0.5), ca, cw));
    }
    let tip = *pts.last().unwrap();
    let end_dir = {
        let (x0, y0) = pts[n - 1];
        (tip.1 - y0).atan2(tip.0 - x0)
    };
    out.push(Br { pts, ws, depth, dead });
    for (q, ca, cw) in kids {
        if cw >= wmin {
            let l = branch_len(cw, r);
            grow(out, r, q, ca, cw, l, depth + 1, wmin);
        }
    }
    if !dead && wend >= wmin * 1.2 {
        // fork at the tip: one stronger leader and one weaker branch
        let w1 = wend * r.range(0.78, 0.92);
        let w2 = wend * r.range(0.45, 0.72);
        let s = if r.chance(0.5) { 1.0 } else { -1.0 };
        let a1 = end_dir + s * r.range(0.1, 0.4);
        let a2 = end_dir - s * r.range(0.35, 0.8);
        let l1 = branch_len(w1, r);
        grow(out, r, tip, a1, w1, l1, depth + 1, wmin);
        if w2 >= wmin {
            let l2 = branch_len(w2, r);
            grow(out, r, tip, a2, w2, l2, depth + 1, wmin);
        }
    }
}

fn wrap(a: f32) -> f32 {
    let t = std::f32::consts::TAU;
    (a + std::f32::consts::PI).rem_euclid(t) - std::f32::consts::PI
}

fn branch_len(w: f32, r: &mut Rng) -> f32 {
    11.0 * w.powf(0.72) * r.range(0.75, 1.25) + 3.0
}

/// The whole oak: a short massive trunk on the rise, three heavy limbs
/// spreading from its crown, a long low limb reaching right, a broken stub.
fn oak(seed: u64) -> Vec<Br> {
    let mut r = Rng::new(seed);
    let mut out = Vec::new();
    // trunk: flared at the foot, leaning a little, with a bulge
    let base = (337.0, 557.0);
    let tpts: Vec<(f32, f32)> = (0..=10)
        .map(|i| {
            let t = i as f32 / 10.0;
            (base.0 + 5.0 * (t * 3.0).sin() - 4.0 * t, base.1 - 153.0 * t)
        })
        .collect();
    let tws: Vec<f32> = (0..=10)
        .map(|i| {
            let t = i as f32 / 10.0;
            34.0 + 18.0 * (1.0 - t / 0.2).max(0.0).powi(2) - 4.0 * t + 3.5 * (t * 9.0).sin() + 5.0 * smoothstep(0.8, 1.0, t)
        })
        .collect();
    out.push(Br { pts: tpts, ws: tws, depth: 0, dead: false });
    let top = (338.0, 418.0);
    let limbs = [
        ((top.0 - 8.0, top.1 + 4.0), -2.45, 17.0, 150.0),
        ((top.0 - 2.0, top.1), -1.85, 20.0, 125.0),
        ((top.0 + 8.0, top.1 + 2.0), -0.95, 17.0, 140.0),
        ((top.0 + 12.0, top.1 + 42.0), -0.28, 11.0, 150.0),
        ((top.0 - 12.0, top.1 + 26.0), -2.8, 9.0, 120.0),
    ];
    for (k, (p, a, w, l)) in limbs.into_iter().enumerate() {
        // the leader is stag-headed: dead above, only bare spiky wood
        let wmin = if k == 1 { 1.3 } else { WMIN };
        grow(&mut out, &mut r, p, a, w, l, 1, wmin);
    }
    // a broken stub low on the left of the trunk
    out.push(Br {
        pts: vec![(323.0, 486.0), (313.0, 480.5), (305.0, 478.5), (300.0, 476.0)],
        ws: vec![9.0, 7.0, 4.5, 1.6],
        depth: 1,
        dead: true,
    });
    out
}
// ckpt: end

fn main() {
    let o = Run::new("r15_p2");
    let mut st = Style::friedrich();
    // a patchy, warm whitish top ground over the two knifed earth layers
    st.ground[2] = Ground { color: hex("#dccfb6"), hiding: 0.85, um: 55.0, stiff: 0.4, apply: Apply::Brush };
    let pal = st.palette.clone();
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = c.height();

    let nk = Fbm::new(11, 3, 1.0);
    let nw = Fbm::new(12, 3, 1.0);
    let ncl = Fbm::new(13, 3, 1.0);
    let knoll = f.per_column(move |x| knoll_y(x, &nk));
    let woods = f.per_column(move |x| woods_y(x, &nw));
    let far = f.per_column(move |x| far_y(x, &nw));

    // ---------------------------------------------------------------- drawing
    if o.stage("drawing", &mut c, &mut rng) {
        let hb = Lead::pencil("HB").unwrap();
        let h2 = Lead::pencil("2H").unwrap();
        let mut worn = 0.0;
        // the rise
        let pts: Vec<(f32, f32)> = (0..=24).map(|i| {
            let x = -10.0 + i as f32 * 30.0;
            (x, knoll(x))
        }).collect();
        let m = graphite::hand_line(&pts, &[0.35, 0.45, 0.4, 0.3], true, false, 0.25, 1);
        worn += c.draw(&hb, &m, worn, 1);
        // far snow line and woods
        let pts: Vec<(f32, f32)> = (0..=22).map(|i| {
            let x = 300.0 + i as f32 * 32.0;
            (x, woods(x))
        }).collect();
        let m = graphite::hand_line(&pts, &[0.3, 0.3], true, false, 0.3, 2);
        worn += c.draw(&h2, &m, worn, 2);
        let m = graphite::hand_line(&[(300.0, HOR), (1010.0, HOR)], &[0.25, 0.3], false, true, 0.0, 3);
        worn += c.draw(&h2, &m, worn, 3);
        // church
        for (k, seg) in [
            [(742.0, HOR - 52.0), (742.0, HOR - 8.0)],
            [(754.0, HOR - 52.0), (754.0, HOR - 8.0)],
            [(748.0, HOR - 88.0), (741.0, HOR - 52.0)],
            [(748.0, HOR - 88.0), (755.0, HOR - 52.0)],
            [(754.0, HOR - 38.0), (800.0, HOR - 38.0)],
        ]
        .iter()
        .enumerate()
        {
            let m = graphite::hand_line(seg, &[0.4, 0.4], false, true, 0.0, 10 + k as u64);
            worn += c.draw(&h2, &m, worn, 10 + k as u64);
        }
        // the oak's trunk and the spring of its main limbs
        let trunk = [
            vec![(318.0, 552.0), (322.0, 500.0), (318.0, 452.0), (326.0, 410.0)],
            vec![(356.0, 552.0), (352.0, 500.0), (356.0, 456.0), (350.0, 414.0)],
        ];
        for (k, t) in trunk.iter().enumerate() {
            let m = graphite::hand_line(t, &[0.5, 0.45, 0.4], true, false, 0.3, 20 + k as u64);
            worn += c.draw(&hb, &m, worn, 20 + k as u64);
        }
        // path
        let pts: Vec<(f32, f32)> = (0..=12).map(|i| {
            let y = 714.0 - i as f32 * 15.0;
            (path_x(y), y)
        }).collect();
        let m = graphite::hand_line(&pts, &[0.3, 0.2], true, false, 0.3, 30);
        let _ = c.draw(&h2, &m, worn, 30);
        c.fix_drawing(None);
    }

    // ------------------------------------------------------------------- sky
    let sky = Mask::from_fn(f, move |x, y| {
        let top = woods(x).min(knoll(x));
        if y < top + 6.0 { 1.0 } else { 0.0 }
    });
    let full_sky = move |x: f32, y: f32| {
        let (v, a) = cloud_at(x, y, &ncl);
        lerpc(sky_col(x, y), cloud_col(x, y, a), v)
    };
    if o.stage("sky", &mut c, &mut rng) {
        // lean, full paint, long near-horizontal strokes worked top to bottom
        let lay = st
            .broad()
            .mixed(&pal, 0.3)
            .color(sky_col)
            .angle(|x, _| 0.04 * ((x - 500.0) / 500.0))
            .angle_jitter(0.05)
            .length(160.0, 340.0)
            .coverage(4.0)
            .sweep(FRAC_PI_2);
        c.work(&sky, &lay, 101);
        // the stratus bands laid into the wet sky
        let clouds = Mask::from_fn(f, move |x, y| cloud_at(x, y, &ncl).0);
        let cl = Handling::new(Tool { lay: 0.8, push: 0.03, ..Tool::filbert(6.0) })
            .mixed(&pal, 0.35)
            .color(move |x, y| {
                let (_, a) = cloud_at(x, y, &ncl);
                cloud_col(x, y, a)
            })
            .angle(|_, _| 0.0)
            .angle_jitter(0.03)
            .curve(0.02, 0.2)
            .length(40.0, 130.0)
            .coverage(2.2)
            .pressure(0.5, 0.75)
            .dips(2, 0.6, 0.6)
            .clip(true)
            .threshold(0.15);
        c.work(&clouds, &cl, 102);
        // stipple into the wet sky before blending: the badger then fuses
        // the touches into the laid paint
        let sp = Stipple::new(Tool::stippler(2.6))
            .mixed(&pal, 0.35)
            .color(full_sky)
            .coverage(|_, _| 1.2)
            .cluster(0.0, None)
            .pressure(0.35, 0.6)
            .dips(8, 0.5, 0.5);
        c.stipple(&sky, &sp, 105);
        // two crossing badger passes, horizontal
        let bl = st.blend().unwrap().angle(|_, _| 0.0);
        c.work(&sky, &bl, 103);
        c.work(&sky, &st.blend().unwrap().angle(|_, _| 0.03).cross(0.12), 104);
    }

    // ------------------------------------------------------------------ glow
    if o.stage("glow", &mut c, &mut rng) {
        // a transparent warm glaze deepening the afterglow above the woods
        let g = Pigment::transparent(hex("#eaa66a"));
        c.glaze(&g, Some(&sky), |x, y| {
            let t = smoothstep(300.0, HOR, y);
            let gx = (-((x - GLOW_X) / 360.0).powi(2)).exp();
            0.32 * t * t * (0.35 + 0.65 * gx)
        });
    }

    // -------------------------------------------------------------- distance
    // the woods lie behind the rise: nothing of them under it
    let woods_m = Mask::from_fn(f, move |x, y| {
        let top = woods(x);
        smoothstep(top - 0.8, top + 0.8, y) * smoothstep(far(x) + 4.0, far(x) + 1.0, y) * smoothstep(knoll(x) + 2.0, knoll(x) - 1.0, y)
    })
    .roughen(21, 5.0, 1.6, 0.9);
    // a Gothic church: west tower with a steep spire, a high-roofed nave, a
    // lower choir, a ridge turret
    let church = Mask::from_shape(
        f,
        Shape::new()
            .poly(&[(742.0, HOR - 52.0), (754.0, HOR - 52.0), (754.0, HOR - 6.0), (742.0, HOR - 6.0)])
            .poly(&[(740.5, HOR - 51.0), (744.0, HOR - 55.0), (748.0, HOR - 89.0), (752.0, HOR - 55.0), (755.5, HOR - 51.0)])
            .poly(&[(754.0, HOR - 30.0), (762.0, HOR - 42.0), (792.0, HOR - 42.0), (800.0, HOR - 30.0), (800.0, HOR - 6.0), (754.0, HOR - 6.0)])
            .poly(&[(800.0, HOR - 26.0), (806.0, HOR - 34.0), (813.0, HOR - 26.0), (815.0, HOR - 20.0), (815.0, HOR - 6.0), (800.0, HOR - 6.0)])
            .poly(&[(775.0, HOR - 41.0), (777.0, HOR - 54.0), (779.0, HOR - 41.0)]),
    );
    let distance = woods_m.clone().union(&church);
    if o.stage("distance", &mut c, &mut rng) {
        // violet-grey woods, hazier toward their foot
        let wc = move |x: f32, y: f32| {
            let top = woods(x);
            let d = ((y - top) / (HOR - top).max(1.0)).clamp(0.0, 1.0);
            let g = (-((x - GLOW_X) / 250.0).powi(2)).exp();
            let c0 = lerpc(hex("#6f6b7b"), hex("#7c6f78"), g);
            lerpc(c0, hex("#a79ea5"), 0.55 * d * d)
        };
        // short upright touches of a small round: trees, not rocks
        let body = Handling::new(Tool { push: 0.04, ..Tool::round_sable(3.2) })
            .mixed(&pal, 0.25)
            .color(wc)
            .angle(|_, _| -FRAC_PI_2)
            .angle_jitter(0.25)
            .length(5.0, 14.0)
            .coverage(3.5)
            .pressure(0.5, 0.75)
            .dips(3, 0.55, 0.6)
            .clip(true)
            .threshold(0.1);
        c.work(&woods_m, &body, 201);
        // the church, a shade darker than the woods, drawn with the sable
        let ch = st
            .detail()
            .color(|_, y| lerpc(hex("#6d6874"), hex("#8f8791"), smoothstep(HOR - 25.0, HOR, y)))
            .angle(|_, _| -FRAC_PI_2)
            .length(4.0, 12.0)
            .coverage(4.0)
            .threshold(0.05);
        c.work(&church, &ch, 202);
        // mist: stippled into the wet woods, lighter toward the snow line
        let mist = Mask::from_fn(f, move |x, y| {
            let top = woods(x).min(HOR - 40.0);
            smoothstep(top, HOR - 2.0, y) * smoothstep(HOR + 3.0, HOR, y)
        });
        let sp = Stipple::new(Tool::stippler(2.4))
            .mixed(&pal, 0.4)
            .color(|x, _| lerpc(hex("#b9aeb0"), hex("#d7bca8"), (-((x - GLOW_X) / 250.0).powi(2)).exp()))
            .coverage(move |_, y| 1.3 * smoothstep(HOR - 22.0, HOR - 2.0, y))
            .cluster(0.0, None)
            .pressure(0.3, 0.55)
            .dips(8, 0.45, 0.5);
        c.stipple(&mist.clone().mul(&distance.clone().offset(1.0).blur(1.0)), &sp, 203);
        let bl = st.blend().unwrap().angle(|_, _| 0.0).length(40.0, 90.0).coverage(1.5).pressure(0.2, 0.3);
        c.work(&mist.clone().mul(&woods_m), &bl, 204);
    }

    // ------------------------------------------------------------------ snow
    let ground = Mask::from_fn(f, move |x, y| {
        let top = knoll(x).min(far(x));
        smoothstep(top - 0.7, top + 0.7, y)
    });
    let in_knoll = move |x: f32, y: f32| y > knoll(x) && x < 760.0;
    let snow_col = move |x: f32, y: f32| -> Rgb {
        if in_knoll(x, y) {
            let d = y - knoll(x);
            let depth = ((y - 470.0) / (h - 470.0)).clamp(0.0, 1.0);
            // the crest takes the light of the whole sky; the face turned to
            // us is in the blue shade of dusk
            let top = lerpc(hex("#cbcad3"), hex("#c3c4d0"), smoothstep(0.0, 400.0, x));
            let slope = lerpc(top, hex("#a5aabc"), smoothstep(0.0, 40.0, d));
            lerpc(slope, hex("#81859a"), smoothstep(0.3, 1.0, depth))
        } else {
            let t = ((y - HOR) / (h - HOR)).clamp(0.0, 1.0);
            let g = (-((x - GLOW_X) / 300.0).powi(2)).exp();
            // the far snow gives back the afterglow
            let far = lerpc(hex("#cdc3be"), hex("#d9c6b3"), g);
            gradient(
                &[(0.0, far), (0.1, hex("#c6c0c3")), (0.4, hex("#afafbd")), (1.0, hex("#8c8ea3"))],
                t,
                Mix::Linear,
            )
        }
    };
    if o.stage("snow", &mut c, &mut rng) {
        // broad passes following the lie of the land: the rise's slope on
        // the left, level on the plain; clipped, so no stroke runs up into
        // the woods
        let ang = move |x: f32, y: f32| {
            if in_knoll(x, y) {
                let s = (knoll(x + 20.0) - knoll(x - 20.0)) / 40.0;
                0.7 * s.atan() * smoothstep(120.0, 0.0, y - knoll(x))
            } else {
                0.0
            }
        };
        let lay = st
            .broad()
            .mixed(&pal, 0.22)
            .color(snow_col)
            .angle(ang)
            .angle_jitter(0.04)
            .length(90.0, 220.0)
            .coverage(3.5)
            .curve(0.03, 0.2)
            .clip(true)
            .sweep(FRAC_PI_2);
        c.work(&ground, &lay, 301);
        // the rise's crest restated along the slope, into the wet: a lit edge
        let crest = Mask::from_fn(f, move |x, y| {
            let d = y - knoll(x);
            if x > 760.0 { 0.0 } else { smoothstep(-0.7, 0.7, d) * smoothstep(12.0, 4.0, d) }
        });
        let cr = st
            .body()
            .color_over(|_, _, under| shift(under, 0.03, 0.0, 0.008))
            .angle(ang)
            .length(30.0, 80.0)
            .coverage(2.2)
            .pressure(0.45, 0.65)
            .clip(true)
            .threshold(0.1);
        c.work(&crest, &cr, 302);
        // two level badger passes even out the laid strokes
        let bl = st.blend().unwrap().angle(ang).pressure(0.3, 0.4).coverage(2.5);
        c.work(&ground, &bl, 305);
        c.work(&ground, &st.blend().unwrap().angle(move |x, y| ang(x, y) + 0.04).cross(0.1).pressure(0.25, 0.35).coverage(2.0), 308);
    }

    // ------------------------------------------------------------------ path
    // a trodden path from the lower edge to the church: its left wall faces
    // the afterglow (lighter), its right wall is turned away (blue), the
    // floor is trampled grey
    let path_dir = move |y: f32| {
        let d = path_x(y - 5.0) - path_x(y + 5.0);
        (-10.0f32).atan2(d)
    };
    let path_part = move |lo: f32, hi: f32| {
        Mask::from_fn(f, move |x, y| {
            let hw = path_half(y);
            if hw <= 0.3 {
                return 0.0;
            }
            let u = (x - path_x(y)) / hw;
            smoothstep(lo - 0.2, lo + 0.2, u) * smoothstep(hi + 0.2, hi - 0.2, u)
        })
        .roughen(31, 12.0, 1.2, 0.6)
    };
    let floor = path_part(-0.55, 0.45);
    let wall_l = path_part(-1.05, -0.55);
    let wall_r = path_part(0.45, 1.0);
    if o.stage("path", &mut c, &mut rng) {
        c.dry();
        let along = move |_: f32, y: f32| path_dir(y);
        let mk = |dl: f32, db: f32| {
            Handling::new(Tool { push: 0.04, ..Tool::round_sable(4.0) })
                .mixed(&pal, 0.2)
                .color_over(move |_, _, under| shift(under, dl, 0.0, db))
                .angle(along)
                .angle_jitter(0.06)
                .length(8.0, 26.0)
                .coverage(2.6)
                .pressure(0.5, 0.75)
                .dips(3, 0.6, 0.6)
                .clip(true)
                .threshold(0.12)
        };
        c.work(&floor, &mk(-0.025, -0.01), 401);
        c.work(&wall_l, &mk(0.012, 0.004), 402);
        c.work(&wall_r, &mk(-0.04, -0.016), 403);
        // footprints on the floor: small blue-grey touches, alternating,
        // closer together with distance
        let mut r = Rng::new(o.seed + 60);
        let mut y = 712.0;
        let mut side = 1.0;
        let mut k = 0u64;
        while y > 548.0 {
            let hw = path_half(y);
            let x = path_x(y) + side * hw * 0.22 + r.normal() * hw * 0.05;
            let under = c.under(x, y, 1.5);
            let col = shift(under, -0.06, 0.0, -0.025);
            let tool = Tool { point: 0.5, ..Tool::round_sable((hw * 0.35).clamp(0.8, 4.0)) };
            let mut b = Held::new(tool, 7000 + k);
            b.load(pal.paint(col, 0.2), 0.5);
            let len = (hw * 0.45).max(0.7);
            let a = path_dir(y);
            c.drag(&mut b, &Gesture::line((x - 0.5 * len * a.cos(), y - 0.5 * len * a.sin()), (x + 0.5 * len * a.cos(), y + 0.5 * len * a.sin())).pressure(0.6, 0.4), None);
            side = -side;
            y -= (hw * 0.55).max(1.2) * r.range(0.8, 1.2);
            k += 1;
        }
    }

    // ------------------------------------------------------------------- oak
    let tree = oak(std::env::var("OAK_SEED").ok().and_then(|v| v.parse().ok()).unwrap_or(o.seed * 12 + 1));
    if let Ok(path) = std::env::var("OAK_SVG") {
        // a line drawing of the tree's geometry, for planning
        let mut svg = String::from("<svg xmlns='http://www.w3.org/2000/svg' width='1000' height='714' viewBox='0 0 1000 714'><rect width='1000' height='714' fill='#ddd'/>");
        for b in &tree {
            for i in 0..b.pts.len() - 1 {
                svg += &format!("<line x1='{:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' stroke='{}' stroke-width='{:.2}' stroke-linecap='round'/>", b.pts[i].0, b.pts[i].1, b.pts[i + 1].0, b.pts[i + 1].1, if b.dead { "#822" } else { "#222" }, b.ws[i]);
            }
        }
        svg += "</svg>";
        std::fs::write(path, svg).unwrap();
        eprintln!("{} branches, {} under 2.6", tree.len(), tree.iter().filter(|b| b.ws[0] < 2.6).count());
        return;
    }
    let limb_min = 2.6;
    let limbs = {
        let mut sh = Shape::new();
        // each segment its own part, with a disc at every joint: a ribbon
        // drawn through a sharp elbow twists on itself and its fill cancels
        for b in tree.iter().filter(|b| b.ws[0] >= limb_min) {
            for i in 0..b.pts.len() - 1 {
                sh = sh.add(Shape::new().ribbon(&b.pts[i..=i + 1], &b.ws[i..=i + 1]));
                sh = sh.add(Shape::new().circle(b.pts[i].0, b.pts[i].1, 0.5 * b.ws[i]));
            }
        }
        // nothing of the wood below the snow line at the foot
        Mask::from_shape(f, sh).mul_fn(|_, y| smoothstep(559.0, 553.0, y))
    };
    // the direction of the wood at every point of the limbs, from the
    // thickest limb over it (a coarse grid, 2 units a cell)
    let grain: std::sync::Arc<Vec<f32>> = {
        let (gw, gh) = (500usize, 360usize);
        let mut ang = vec![0.0f32; gw * gh];
        let mut wid = vec![0.0f32; gw * gh];
        for b in tree.iter().filter(|b| b.ws[0] >= limb_min) {
            for i in 0..b.pts.len() - 1 {
                let (a, c2) = (b.pts[i], b.pts[i + 1]);
                let d = (c2.1 - a.1).atan2(c2.0 - a.0);
                let w = b.ws[i];
                let len = ((c2.0 - a.0).hypot(c2.1 - a.1) / 1.0).ceil() as usize;
                for k in 0..=len {
                    let t = k as f32 / len.max(1) as f32;
                    let (px, py) = (a.0 + (c2.0 - a.0) * t, a.1 + (c2.1 - a.1) * t);
                    let r = (0.5 * w + 2.0) / 2.0;
                    let (cx, cy) = (px / 2.0, py / 2.0);
                    for gy in (cy - r).floor() as i32..=(cy + r).ceil() as i32 {
                        for gx in (cx - r).floor() as i32..=(cx + r).ceil() as i32 {
                            if gx < 0 || gy < 0 || gx >= gw as i32 || gy >= gh as i32 {
                                continue;
                            }
                            let j = gy as usize * gw + gx as usize;
                            if w > wid[j] {
                                wid[j] = w;
                                ang[j] = d;
                            }
                        }
                    }
                }
            }
        }
        std::sync::Arc::new(ang)
    };
    let grain_at = {
        let g = grain.clone();
        move |x: f32, y: f32| {
            let gx = ((x / 2.0) as i32).clamp(0, 499) as usize;
            let gy = ((y / 2.0) as i32).clamp(0, 359) as usize;
            g[gy * 500 + gx]
        }
    };
    // bark: dark warm grey; the side that faces the open upper-left sky a
    // cooler, lighter grey
    let bark = |u: f32, w: f32| -> Rgb {
        // dark all across: the sky-lit rim comes later, on its own mask
        let lit = smoothstep(0.3, 0.9, -u);
        let c0 = lerpc(hex("#302a27"), hex("#3a3431"), smoothstep(20.0, 3.0, w));
        lerpc(c0, hex("#48423f"), 0.4 * lit)
    };
    if o.stage("oak", &mut c, &mut rng) {
        // the tree goes on over the dry sky and snow
        c.dry();
        eprintln!("oak: {} branches", tree.len());
        let mut order: Vec<&Br> = tree.iter().filter(|b| b.ws[0] >= limb_min).collect();
        order.sort_by(|a, b| b.ws[0].total_cmp(&a.ws[0]));
        let mut k = 0u64;
        for b in order {
            let w0 = b.ws[0];
            // strokes along the limb, side by side across it
            let n = ((w0 / 4.5).ceil() as usize).clamp(1, 9);
            let tw = (w0 / n as f32 * 1.9).clamp(2.5, 9.0);
            for j in 0..n {
                let u = if n == 1 { 0.0 } else { -1.0 + 2.0 * (j as f32 + 0.5) / n as f32 };
                let pts: Vec<(f32, f32)> = (0..b.pts.len())
                    .map(|i| {
                        let (a, bb) = if i + 1 < b.pts.len() { (b.pts[i], b.pts[i + 1]) } else { (b.pts[i - 1], b.pts[i]) };
                        let d = (bb.1 - a.1).atan2(bb.0 - a.0);
                        // normal pointing to the left of travel; flip so -u is up-left
                        let (mut nx, mut ny) = (-d.sin(), d.cos());
                        if nx + ny > 0.0 {
                            nx = -nx;
                            ny = -ny;
                        }
                        let off = -u * 0.5 * b.ws[i] * 0.8;
                        (b.pts[i].0 + nx * off, b.pts[i].1 + ny * off)
                    })
                    .collect();
                let mut br = Held::new(Tool { push: 0.05, ..Tool::filbert(tw) }, 4000 + k);
                br.load(Paint::body(bark(u, w0)).with_stiff(0.8), 0.85);
                let g = Gesture::new(pts).pressure(0.8, 0.65).ramps(0.05, 0.2).shake(0.6);
                c.drag(&mut br, &g, Some(&limbs));
                k += 1;
            }
        }
        // where limbs cross and join, their strokes lifted each other's wet
        // paint: state the whole of the wood once more, along the grain
        let unify = Handling::new(Tool { push: 0.04, ..Tool::filbert(4.0) })
            .mixed(&pal, 0.15)
            .color(|_, _| hex("#342e2b"))
            .angle(grain_at.clone())
            .angle_jitter(0.05)
            .length(10.0, 30.0)
            .coverage(1.4)
            .pressure(0.55, 0.8)
            .dips(2, 0.6, 0.6)
            .clip(true)
            .threshold(0.3);
        c.work(&limbs, &unify, 460);
    }

    let twig_paint = |w: f32| -> Rgb { lerpc(hex("#312b28"), hex("#433c3a"), smoothstep(2.5, 0.4, w)) };
    if o.stage("twigs", &mut c, &mut rng) {
        let mut order: Vec<&Br> = tree.iter().filter(|b| b.ws[0] < limb_min).collect();
        order.sort_by(|a, b| b.ws[0].total_cmp(&a.ws[0]));
        eprintln!("twigs: {}", order.len());
        for (k, b) in order.iter().enumerate() {
            let w0 = b.ws[0];
            let w1 = *b.ws.last().unwrap();
            let tool = Tool { point: 0.7, push: 0.02, ..Tool::round_sable((w0 * 1.5).max(1.0)) };
            let (p0, p1) = (tool.pressure_for(w0), tool.pressure_for(w1 * 0.7));
            let mut br = Held::new(tool, 9000 + k as u64);
            br.load(Paint::body(twig_paint(w0)).with_stiff(0.6), 0.9);
            let g = Gesture::new(b.pts.clone()).pressure(p0, p1).ramps(0.03, 0.35).shake(0.4);
            c.drag(&mut br, &g, None);
        }
    }

    // ----------------------------------------------------------------- bark
    let nb = Fbm::new(18, 3, 1.0);
    if o.stage("bark", &mut c, &mut rng) {
        // fissures: thin darker strokes along the wood; ridges: broken
        // lighter ones, mostly on the side toward the open sky
        let fis = Handling::new(Tool { point: 0.5, push: 0.02, ..Tool::round_sable(1.6) })
            .mixed(&pal, 0.15)
            .color_over(|_, _, under| shift(under, -0.035, 0.0, -0.004))
            .angle(grain_at.clone())
            .angle_jitter(0.08)
            .length(6.0, 20.0)
            .curve(0.06, 0.4)
            .coverage(0.9)
            .pressure(0.4, 0.7)
            .dips(4, 0.6, 0.6)
            .clip(true)
            .fill(false)
            .threshold(0.5);
        c.work(&limbs, &fis, 451);
        let ridge = Handling::new(Tool { point: 0.5, push: 0.02, ..Tool::round_sable(1.4) })
            .mixed(&pal, 0.15)
            .color_over(move |x, y, under| shift(under, 0.014 + 0.01 * nb.get(x / 8.0, y / 8.0), -0.002, -0.004))
            .angle(grain_at.clone())
            .angle_jitter(0.1)
            .length(4.0, 12.0)
            .coverage(0.35)
            .broken(0.4)
            .pressure(0.3, 0.55)
            .dips(4, 0.5, 0.6)
            .clip(true)
            .fill(false)
            .threshold(0.5);
        c.work(&limbs, &ridge, 452);
        // the sky-lit rim: the upper-left edge of every limb, where the wood
        // turns to the open sky, a cool grey laid along the grain
        let lim = std::sync::Arc::new(limbs.clone());
        let rim = {
            let l = lim.clone();
            Mask::from_fn(f, move |x, y| {
                let inside = l.sample(x, y);
                let out = 1.0 - l.sample(x - 1.6, y - 1.9);
                inside * out
            })
            .blur(0.4)
        };
        let rl = Handling::new(Tool { point: 0.6, push: 0.02, ..Tool::round_sable(1.8) })
            .mixed(&pal, 0.15)
            .color(|x, _| lerpc(hex("#5d5b63"), hex("#6a6064"), smoothstep(300.0, 650.0, x)))
            .angle(grain_at.clone())
            .angle_jitter(0.05)
            .length(8.0, 26.0)
            .coverage(1.6)
            .broken(0.25)
            .pressure(0.45, 0.7)
            .dips(3, 0.6, 0.6)
            .clip(true)
            .fill(false)
            .threshold(0.3);
        c.work(&rim, &rl, 455);
        // a knot-hole low on the trunk: a dark hollow with a thick lip
        let hole = Mask::from_shape(f, Shape::new().ellipse(346.0, 471.0, 2.6, 4.6)).roughen(41, 3.0, 0.8, 0.4);
        let lip = Mask::from_shape(f, Shape::new().ellipse(345.5, 470.0, 4.6, 7.0)).roughen(42, 4.0, 1.2, 0.6).subtract(&hole);
        let lp = st.detail().color(|_, _| hex("#433d3b")).angle(|_, _| -FRAC_PI_2).length(3.0, 8.0).coverage(2.5).threshold(0.3);
        c.work(&lip.mul_fn(|x, y| smoothstep(0.0, 6.0, (344.5 - x) + (469.0 - y) + 4.0)), &lp, 453);
        let hp = st.detail().color(|_, _| hex("#1d1a1a")).angle(|_, _| -FRAC_PI_2).length(2.0, 6.0).coverage(3.5).threshold(0.1);
        c.work(&hole, &hp, 454);
    }

    // ------------------------------------------------------- snow on the oak
    // snow lies along the upper side of every limb that isn't too steep,
    // broken where the wood turns up
    let lodged: Vec<(Vec<(f32, f32)>, Vec<f32>)> = {
        let mut v = Vec::new();
        for b in tree.iter().filter(|b| b.ws[0] >= 1.6 && b.depth > 0) {
            let mut run: Vec<(f32, f32)> = Vec::new();
            let mut wr: Vec<f32> = Vec::new();
            for i in 0..b.pts.len() - 1 {
                let (a, c2) = (b.pts[i], b.pts[i + 1]);
                let d = (c2.1 - a.1).atan2(c2.0 - a.0);
                let flat = d.sin().abs() < 0.62;
                if flat {
                    // the upward normal
                    let (mut nx, mut ny) = (-d.sin(), d.cos());
                    if ny > 0.0 {
                        nx = -nx;
                        ny = -ny;
                    }
                    let off = 0.36 * b.ws[i];
                    if run.is_empty() {
                        run.push((a.0 + nx * off, a.1 + ny * off));
                        wr.push(b.ws[i]);
                    }
                    run.push((c2.0 + nx * off, c2.1 + ny * off));
                    wr.push(b.ws[i + 1]);
                } else if run.len() >= 3 {
                    v.push((std::mem::take(&mut run), std::mem::take(&mut wr)));
                } else {
                    run.clear();
                    wr.clear();
                }
            }
            if run.len() >= 3 {
                v.push((run, wr));
            }
        }
        v
    };
    if o.stage("snow on the oak", &mut c, &mut rng) {
        let mut r = Rng::new(o.seed + 70);
        for (k, (pts, ws)) in lodged.iter().enumerate() {
            if r.chance(0.25) {
                continue;
            }
            let w = ws.iter().copied().fold(0.0, f32::max);
            let tool = Tool { point: 0.6, push: 0.02, ..Tool::round_sable((0.42 * w).clamp(0.7, 7.0)) };
            let p = tool.pressure_for((0.34 * w).max(0.5));
            let mut b = Held::new(tool, 12000 + k as u64);
            // bluish in the dusk, a touch warmer where the glow reaches
            let (mx, my) = pts[pts.len() / 2];
            let col = lerpc(hex("#c9cad6"), hex("#d8cdc8"), smoothstep(250.0, 700.0, mx) * smoothstep(150.0, 450.0, my));
            b.load(pal.paint(col, 0.15), 0.8);
            let g = Gesture::new(pts.clone()).pressure(p * 0.7, p * 0.5).swell(vec![0.6, 1.1, 1.0, 0.7]).ramps(0.15, 0.4).shake(0.5);
            c.drag(&mut b, &g, None);
        }
    }

    // ----------------------------------------------------------- oak's foot
    // a drift banked round the foot of the trunk, bluer in the hollow
    // against the bark
    let nf = Fbm::new(16, 3, 1.0);
    // the drift's top: banked up against the trunk (higher on the left,
    // where the wind laid it), rising and falling with small hummocks
    let foot_y = move |x: f32| {
        let dx = x - 337.0;
        let bank = 7.0 * (-(dx / 24.0).powi(2)).exp() * (1.0 + 0.35 * (-dx / 20.0).tanh());
        553.0 - bank + 0.12 * dx.abs() + 1.4 * nf.get(x / 9.0, 0.3) + 0.7 * nf.get(x / 3.0, 4.1)
    };
    let foot = Mask::from_fn(f, move |x, y| {
        let dx = (x - 337.0).abs();
        if dx > 80.0 {
            return 0.0;
        }
        smoothstep(-0.6, 0.6, y - foot_y(x)) * smoothstep(585.0, 568.0, y) * smoothstep(58.0, 34.0, dx)
    });
    if o.stage("foot", &mut c, &mut rng) {
        let fc = move |x: f32, y: f32| {
            let base = snow_col(x, y);
            let top = smoothstep(6.0, 0.0, y - foot_y(x));
            let hollow = smoothstep(28.0, 16.0, (x - 337.0).abs()) * smoothstep(1.0, 8.0, y - foot_y(x));
            shift(base, 0.02 * top - 0.012 * hollow, 0.0, 0.005 * top - 0.006 * hollow)
        };
        // two opaque passes of lean body colour, level strokes; no blender
        // over the bark, which would drag it into the snow
        for (k, cov) in [(501u64, 3.0f32), (502, 2.0)] {
            let h2 = st
                .body()
                .mixed(&pal, 0.1)
                .color(fc)
                .angle(move |x, _| 0.25 * ((337.0 - x) / 40.0).tanh())
                .angle_jitter(0.1)
                .length(10.0, 26.0)
                .coverage(cov)
                .pressure(0.55, 0.8)
                .dips(1, 0.7, 0.6)
                .clip(true)
                .threshold(0.08);
            c.work(&foot, &h2, k);
        }
    }

    // ------------------------------------------------------------ the posts
    // an old fence line running away across the plain on the right: a few
    // leaning posts, each with a cap of snow
    let posts: Vec<(f32, f32, f32, f32)> = {
        let mut r = Rng::new(o.seed + 90);
        let (ax, ay) = (992.0f32, 688.0f32);
        let (vx, vy) = (812.0f32, HOR); // where the line would meet the far snow
        let mut v = Vec::new();
        let mut s = 0.0f32;
        // equal steps on the ground: 1/(y - HOR) steps evenly
        let inv0 = 1.0 / (ay - HOR);
        for i in 0..13 {
            let inv = inv0 + i as f32 * 0.0026;
            let y = HOR + 1.0 / inv;
            let t = (ay - y) / (ay - vy);
            let x = ax + (vx - ax) * t;
            s += 1.0;
            if r.chance(0.15) && i > 2 {
                continue; // a post gone
            }
            let hgt = (y - HOR) * 0.2 * r.range(0.85, 1.1);
            v.push((x, y, hgt, r.normal() * 0.08));
        }
        let _ = s;
        v
    };
    if o.stage("posts", &mut c, &mut rng) {
        for (k, &(x, y, hgt, lean)) in posts.iter().enumerate() {
            let w = (hgt * 0.13).max(0.8);
            let top = (x + hgt * lean.sin(), y - hgt * lean.cos());
            // the post, dark weathered wood
            let tool = Tool { push: 0.03, ..Tool::round_sable(w * 1.2) };
            let p = tool.pressure_for(w);
            let mut b = Held::new(tool, 30000 + k as u64);
            b.load(pal.paint(hex("#3d3533"), 0.15), 0.9);
            c.drag(&mut b, &Gesture::line((x, y + 0.6), top).pressure(p, p * 0.9).ramps(0.02, 0.1).shake(0.3), None);
            // a lighter, cooler edge toward the open sky
            let tool = Tool { point: 0.7, ..Tool::round_sable((w * 0.5).max(0.6)) };
            let mut b = Held::new(tool, 30100 + k as u64);
            b.load(pal.paint(hex("#77747c"), 0.15), 0.6);
            let off = -0.35 * w;
            c.drag(&mut b, &Gesture::line((x + off, y - 0.1 * hgt), (top.0 + off, top.1 + 0.1 * hgt)).pressure(0.35, 0.25).shake(0.3), None);
            // a cap of snow
            let mut b = Held::new(Tool { point: 0.5, ..Tool::round_sable(w * 1.1) }, 30200 + k as u64);
            b.load(pal.paint(hex("#cfd0da"), 0.15), 0.8);
            c.drag(&mut b, &Gesture::line((top.0 - 0.45 * w, top.1 + 0.2), (top.0 + 0.45 * w, top.1 + 0.3)).pressure(0.6, 0.5), None);
            // snow banked at its foot
            let mut b = Held::new(Tool { point: 0.4, ..Tool::round_sable(w * 2.2) }, 30300 + k as u64);
            b.load(pal.paint(shift(c.under(x + 3.0 * w, y, 1.0), 0.01, 0.0, 0.0), 0.15), 0.7);
            c.drag(&mut b, &Gesture::line((x - 1.4 * w, y + 0.8), (x + 1.4 * w, y + 0.6)).pressure(0.5, 0.4), None);
        }
    }

    // ----------------------------------------------------------- vegetation
    // dry grass and weeds through the snow, in tufts: thick on the rise and
    // along the path, sparse on the plain, smaller with distance
    let tufts: Vec<(f32, f32, f32)> = {
        let mut r = Rng::new(o.seed + 80);
        let mut v = Vec::new();
        let ok = |x: f32, y: f32| {
            let top = knoll(x).min(far(x));
            y > top + 2.0
                && y < 716.0
                && ((x - path_x(y)) / path_half(y).max(0.5)).abs() > 1.15
                && foot.sample(x, y) < 0.2
        };
        // patches where the snow lies thin over rough grass: on the rise,
        // along the path and the fence, few on the open plain
        let mut centres: Vec<(f32, f32, f32)> = Vec::new();
        let mut tries = 0;
        while centres.len() < 44 && tries < 5000 {
            tries += 1;
            let x = r.range(-20.0, 1020.0);
            let y = r.range(534.0, 720.0);
            if !ok(x, y) {
                continue;
            }
            let near = (y - HOR) / 190.0;
            let w = if in_knoll(x, y) { 1.0 } else { 0.25 }
                + 0.9 * smoothstep(5.0, 1.5, ((x - path_x(y)) / path_half(y).max(1.0)).abs());
            if r.f() < w * 0.5 {
                centres.push((x, y, near));
            }
        }
        for &(cx, cy, near) in &centres {
            let n = 1 + (r.f() * (2.0 + 6.0 * near)) as usize;
            let spread = 4.0 + 26.0 * near;
            for _ in 0..n {
                let x = cx + r.normal() * spread;
                let y = cy + r.normal() * spread * 0.15;
                if ok(x, y) {
                    v.push((x, y, ((y - HOR) / 190.0).clamp(0.0, 1.0)));
                }
            }
        }
        // a few big clumps at the lower edge, to stand in front of it all
        for &(x, y) in &[(52.0, 706.0), (84.0, 712.0), (170.0, 716.0), (872.0, 709.0), (905.0, 714.0), (470.0, 717.0)] {
            v.push((x, y, 1.12));
        }
        // weeds along the fence line
        for &(x, y, _, _) in &posts {
            for _ in 0..2 {
                let (dx, dy) = (r.normal() * 6.0 * (y - HOR) / 190.0 + 1.0, r.range(-0.5, 1.5));
                if ok(x + dx, y + dy) {
                    v.push((x + dx, y + dy, ((y + dy - HOR) / 190.0).clamp(0.0, 1.0)));
                }
            }
        }
        v
    };
    if o.stage("grass", &mut c, &mut rng) {
        eprintln!("tufts: {}", tufts.len());
        let mut r = Rng::new(o.seed + 81);
        // dry grass in the dusk: grey-browns, a few straw-coloured stems
        let cols = [hex("#5e5347"), hex("#6d604f"), hex("#4b433d"), hex("#7f6f5a"), hex("#57514d"), hex("#453d38")];
        let mut k = 0u64;
        for &(x, y, near) in &tufts {
            let size = 2.5 + 24.0 * near.powf(1.6) * r.range(0.7, 1.2);
            // a small bluish hollow where the snow has sunk round the tuft
            let mut b = Held::new(Tool { point: 0.3, ..Tool::round_sable((size * 0.5).max(1.0)) }, 19000 + k);
            b.load(pal.paint(shift(c.under(x, y + 0.5, 1.5), -0.035, 0.0, -0.014), 0.2), 0.4);
            c.drag(&mut b, &Gesture::line((x - size * 0.3, y + 0.3), (x + size * 0.3, y + 0.2)).pressure(0.35, 0.3), None);
            k += 1;
            let blades = 4 + (r.f() * (4.0 + 14.0 * near)) as usize;
            let lean = r.normal() * 0.2;
            let tone = cols[(r.f() * cols.len() as f32) as usize % cols.len()];
            for j in 0..blades {
                let col = if r.chance(0.3) { cols[(r.f() * cols.len() as f32) as usize % cols.len()] } else { tone };
                let seed_stalk = j == 0 && r.chance(0.35);
                let len = size * if seed_stalk { r.range(1.3, 1.7) } else { r.range(0.35, 1.0) };
                let a = -FRAC_PI_2 + lean + r.normal() * 0.32;
                // most blades arch a little; some are broken over
                let bend = if r.chance(0.12) { r.range(0.9, 1.5) * lean.signum().max(0.0).mul_add(2.0, -1.0) } else { r.normal() * 0.15 + lean * 0.6 };
                let x0 = x + r.normal() * size * 0.1;
                let p1 = (x0 + 0.5 * len * a.cos(), y + 0.5 * len * a.sin());
                let a2 = a + bend;
                let p2 = (p1.0 + 0.5 * len * a2.cos(), p1.1 + 0.5 * len * a2.sin());
                let w = (0.2 + 0.45 * near).max(0.3);
                let tool = Tool { point: 0.95, push: 0.01, ..Tool::round_sable(w * 1.6) };
                let p0 = tool.pressure_for(w);
                let mut b = Held::new(tool, 20000 + k);
                b.load(pal.paint(col, 0.15), 0.7);
                c.drag(&mut b, &Gesture::new(vec![(x0, y + 0.4), p1, p2]).pressure(p0, 0.04).ramps(0.04, 0.6).shake(0.3), None);
                k += 1;
                if seed_stalk && near > 0.3 {
                    // a small dark seed head
                    let mut b = Held::new(Tool { point: 0.6, ..Tool::round_sable(w * 3.0) }, 20000 + k);
                    b.load(pal.paint(hex("#3f3833"), 0.15), 0.7);
                    let q = (p2.0 - 0.15 * len * a2.cos(), p2.1 - 0.15 * len * a2.sin());
                    c.drag(&mut b, &Gesture::line(q, p2).pressure(0.5, 0.2), None);
                    k += 1;
                }
            }
        }
    }

    // ------------------------------------------------------ figure and crows
    // a walker in a dark greatcoat and hat, seen from behind, with a stick,
    // on the path toward the church
    let fy = 554.0f32;
    let fx = path_x(fy) - 0.5;
    let fh = 30.0f32;
    let fp = |u: f32, v: f32| (fx + u * fh, fy - v * fh);
    let figure = Mask::from_shape(
        f,
        Shape::new()
            // coat, a little wider at the hem, swinging with the step
            .poly(&[fp(-0.11, 0.8), fp(0.11, 0.8), fp(0.14, 0.62), fp(0.17, 0.26), fp(0.02, 0.23), fp(-0.16, 0.27), fp(-0.14, 0.6)])
            // legs, one striding
            .poly(&[fp(-0.08, 0.27), fp(-0.02, 0.27), fp(-0.05, 0.0), fp(-0.1, 0.0)])
            .poly(&[fp(0.02, 0.27), fp(0.09, 0.27), fp(0.12, 0.03), fp(0.07, 0.02)])
            // head and a low round hat
            .ellipse(fx, fy - 0.86 * fh, 0.065 * fh, 0.075 * fh)
            .poly(&[fp(-0.12, 0.905), fp(0.12, 0.905), fp(0.11, 0.93), fp(-0.11, 0.93)])
            .poly(&[fp(-0.065, 0.925), fp(0.065, 0.925), fp(0.055, 1.0), fp(-0.055, 1.0)])
            // stick
            .poly(&[fp(0.15, 0.56), fp(0.165, 0.56), fp(0.265, 0.0), fp(0.25, 0.0)]),
    );
    // crows: sitting on level limbs in the oak, two more flying
    let crows: Vec<(f32, f32, f32, bool)> = {
        let mut r = Rng::new(o.seed + 100);
        let mut v = Vec::new();
        let mut cands: Vec<(f32, f32)> = Vec::new();
        for b in tree.iter().filter(|b| b.ws[0] >= 2.0 && b.ws[0] <= 7.0) {
            for i in 0..b.pts.len() - 1 {
                let (a, c2) = (b.pts[i], b.pts[i + 1]);
                let d = (c2.1 - a.1).atan2(c2.0 - a.0);
                if d.sin().abs() < 0.2 && a.1 < 420.0 {
                    cands.push((a.0, a.1 - b.ws[i] * 0.5));
                }
            }
        }
        for _ in 0..40 {
            if v.len() >= 4 || cands.is_empty() {
                break;
            }
            let (x, y) = cands[(r.f() * cands.len() as f32) as usize % cands.len()];
            if v.iter().all(|&(px, py, _, _): &(f32, f32, f32, bool)| (px - x).hypot(py - y) > 60.0) {
                v.push((x, y, if r.chance(0.5) { 1.0 } else { -1.0 }, false));
            }
        }
        v.push((742.0, 286.0, 1.0, true));
        v.push((778.0, 309.0, -1.0, true));
        v
    };
    let crow_mask = {
        let mut sh = Shape::new();
        for &(x, y, dir, flying) in &crows {
            let s = if flying { 1.7f32 } else { 1.5 };
            if flying {
                // wings raised in a shallow M, seen from the side
                let q = |u: f32, v: f32| (x + dir * u * s, y - v * s);
                // the wings bow up from the body and droop to the tips: an M
                sh = sh
                    .poly(&[
                        q(-7.5, 0.6),
                        q(-4.0, 3.0),
                        q(-1.2, 1.4),
                        q(0.0, 1.9),
                        q(1.2, 1.4),
                        q(4.0, 3.0),
                        q(7.5, 0.6),
                        q(3.8, 1.9),
                        q(0.9, 0.0),
                        q(-0.9, 0.0),
                        q(-3.8, 1.9),
                    ])
                    .ellipse(x, y + 0.2 * s, 1.1 * s, 1.5 * s);
            } else {
                // perched: body tilted, tail down behind, head and beak forward
                let q = |u: f32, v: f32| (x + dir * u * s, y - v * s);
                sh = sh
                    .ellipse(x + dir * 0.3 * s, y - 3.0 * s, 2.6 * s, 3.4 * s)
                    .ellipse(x + dir * 1.6 * s, y - 6.6 * s, 1.5 * s, 1.4 * s)
                    .poly(&[q(2.8, 6.9), q(4.8, 6.4), q(2.8, 6.0)])
                    .poly(&[q(-1.2, 1.2), q(-3.4, -2.2), q(-2.2, -2.4), q(0.4, 0.4)])
                    .poly(&[q(-0.4, 0.3), q(-0.1, -0.3), q(0.9, -0.3), q(0.7, 0.3)]);
            }
        }
        Mask::from_shape(f, sh)
    };
    if o.stage("figure", &mut c, &mut rng) {
        let dark = st
            .detail()
            .color(|x, _| lerpc(hex("#29252a"), hex("#302a2c"), smoothstep(700.0, 730.0, x)))
            .angle(|_, _| -FRAC_PI_2)
            .length(2.0, 6.0)
            .coverage(4.0)
            .threshold(0.05);
        c.work(&figure, &dark, 601);
        // the sky catches the edge of his coat and hat on the side away from the glow
        let edge = {
            let fm = std::sync::Arc::new(figure.clone());
            Mask::from_fn(f, move |x, y| fm.sample(x, y) * (1.0 - fm.sample(x - 0.9, y - 0.5)))
        };
        let rim = st.detail().color(|_, _| hex("#4e4a55")).angle(|_, _| -FRAC_PI_2).length(2.0, 5.0).coverage(1.5).pressure(0.4, 0.6).threshold(0.3);
        c.work(&edge, &rim, 604);
        // the shadow he casts is lost in the dusk; a faint blue touch at his feet
        let mut b = Held::new(Tool { point: 0.5, ..Tool::round_sable(2.0) }, 602);
        b.load(pal.paint(hex("#8f90a4"), 0.2), 0.5);
        c.drag(&mut b, &Gesture::line((fx - 5.0, fy + 0.6), (fx + 6.0, fy + 0.4)).pressure(0.35, 0.3), None);
        let cr = st
            .detail()
            .color(|_, _| hex("#1f1c1e"))
            .length(1.5, 4.0)
            .coverage(4.0)
            .threshold(0.05);
        c.work(&crow_mask, &cr, 603);
    }

    // --------------------------------------------------------- final glazes
    // Carus's note of Friedrich's advice: a dark glaze over the picture,
    // deeper toward the edges; here a thin warm-grey veil at the corners
    if o.stage("veil", &mut c, &mut rng) {
        // the evening star over the afterglow, a single pale touch
        let halo = Mask::from_shape(f, Shape::new().circle(866.0, 262.0, 7.0)).blur(5.0);
        c.glaze(&Pigment::semi(hex("#f3ecd9")), Some(&halo), |_, _| 0.22);
        let mut b = Held::new(Tool { point: 0.6, ..Tool::round_sable(2.6) }, 700);
        b.load(Paint::body(hex("#f8f4e8")), 0.9);
        c.touch(&mut b, &Touch::at(866.0, 262.0).pressure(0.75), None);
        // the near snow sinks into the blue of the evening
        c.glaze(&Pigment::transparent(hex("#7c7f9a")), None, |_, y| 0.22 * smoothstep(600.0, 714.0, y));
        let g = Pigment::transparent(hex("#6d6270"));
        c.glaze(&g, None, move |x, y| {
            let dx = (x - 520.0) / 520.0;
            let dy = (y - 360.0) / 380.0;
            let r2 = dx * dx + dy * dy;
            0.22 * smoothstep(0.45, 1.3, r2)
        });
    }

    let fin = Finish {
        varnish_coats: 0.22,
        varnish_vary: 0.06,
        cracks: Some(Cracks { width_um: Some(4.0), dirt: 0.12, veil: 0.15, hierarchy: 0.5, patchy: 0.6, depth_um: 8.0, cupping_um: 6.0, ..Cracks::aged(o.seed + 5) }),
        ..Finish::aged(st.relief)
    };
    o.finish(&mut c, &mut rng, &fin);
}
