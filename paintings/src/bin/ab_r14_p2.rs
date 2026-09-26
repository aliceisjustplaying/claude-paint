//! r14_p2: Evening over the flat country. The sun has just gone down behind
//! a small town on the far edge of a flat coastal plain; a lagoon under the
//! glow, a stream winding to it through meadows that step down toward us, a
//! dark rise with two figures seen from behind, a young crescent above the
//! afterglow.
//!
//!   cargo paint r14_p2 -- --full --width 2400
//!   cargo paint r14_p2 -- --full --width 2400 --crop 300,300,600,500

use paint::color::{Mix, gradient, mix};
use paint::{Cracks, Fbm, Gesture, Handling, Held, Mask, Pigment, Rgb, Rng, Stipple, Style, Tool, hex, shift, smoothstep};
use paintings::figures::{self, Gown, WomanPose};
use paintings::run::{Finish, Run};
use std::f32::consts::FRAC_PI_2;

const ASPECT: f32 = 1.4;
/// The horizon (units from the top).
const HZ: f32 = 468.0;
/// Where the sun went down (x), a little left of the town's middle.
const GX: f32 = 405.0;

/// The crescent: center and radius.
const MOON: (f32, f32, f32) = (655.0, 178.0, 8.0);

/// A handling with another brush in the hand.
trait InHand {
    fn tool(self, t: Tool) -> Self;
}
impl InHand for Handling<'_> {
    fn tool(mut self, t: Tool) -> Self {
        self.tool = t;
        self
    }
}

/// The look of the evening sky at a point: bands that wander a little,
/// warm and pale at the horizon under the glow, a greenish cream band, a
/// breath of violet above the glow, then smalt blue toward the top.
fn sky_at(n: &Fbm, x: f32, y: f32) -> Rgb {
    let t = ((HZ - y) / HZ).clamp(0.0, 1.0) + 0.03 * n.get(x * 0.5, y * 1.5) + 0.02 * n.get(x * 3.0 + 400.0, y * 0.5);
    let g = (-((x - GX) / 300.0).powi(2)).exp();
    let g2 = (-((x - GX) / 140.0).powi(2)).exp();
    let low = mix(hex("#d8c6a6"), hex("#f0cc92"), g, Mix::Light);
    let low = mix(low, hex("#f0b676"), g2 * 0.6, Mix::Light);
    let band = mix(hex("#dbd1b0"), hex("#eed9a6"), g, Mix::Light);
    let s = gradient(
        &[
            (0.0, low),
            (0.07, band),
            (0.2, mix(hex("#d9d7ba"), hex("#e4dbb6"), g, Mix::Light)),
            (0.36, hex("#c2c6bf")),
            (0.55, hex("#a0afc2")),
            (0.78, hex("#7a8fb4")),
            (1.0, hex("#61779f")),
        ],
        t,
        Mix::Light,
    );
    // the purple light, high over the place of the sun
    let p = smoothstep(0.3, 0.45, t) * (1.0 - smoothstep(0.5, 0.72, t)) * (-((x - GX) / 420.0).powi(2)).exp();
    mix(s, hex("#c3b1b8"), 0.3 * p, Mix::Light)
}

/// Streaks of cloud: (alpha, how far down the streak, is it low).
fn streaks(n: &Fbm, x: f32, y: f32) -> (f32, f32, f32) {
    // (middle y, x from, x to, thickness)
    const S: [(f32, f32, f32, f32); 12] = [
        (443.0, 170.0, 540.0, 4.2),
        (431.0, 480.0, 780.0, 3.2),
        (409.0, 240.0, 430.0, 2.8),
        (396.0, 560.0, 930.0, 4.0),
        (452.0, 760.0, 1010.0, 2.6),
        (352.0, 60.0, 360.0, 2.8),
        (306.0, 630.0, 840.0, 2.0),
        // high wisps, still in the sun
        (262.0, 90.0, 430.0, 1.3),
        (228.0, 470.0, 880.0, 1.5),
        (171.0, 180.0, 400.0, 1.1),
        (283.0, 720.0, 1000.0, 1.2),
        (118.0, 560.0, 820.0, 1.3),
    ];
    let mut a: f32 = 0.0;
    let mut low = 0.0;
    let mut lowness = 0.0;
    for (i, &(cy, x0, x1, th)) in S.iter().enumerate() {
        if x < x0 - 30.0 || x > x1 + 30.0 {
            continue;
        }
        let k = i as f32 * 97.0;
        let u = ((x - x0) / (x1 - x0)).clamp(0.0, 1.0);
        let taper = (smoothstep(0.0, 0.3, u) * (1.0 - smoothstep(0.65, 1.0, u))).powf(0.7);
        let rag = 0.5 + 0.5 * n.get(x * 1.2 + k, k);
        let thick = th * taper * (0.5 + 1.0 * rag);
        let mid = cy + 4.0 * n.get(x * 0.4 + k, 3.0 * k) + 8.0 * (u - 0.5).powi(2);
        let d = y - mid;
        if thick <= 0.05 {
            continue;
        }
        let v = 1.0 - smoothstep(thick * 0.45, thick * 1.5, d.abs());
        if v > a {
            a = v;
            low = (d / thick).clamp(-1.0, 1.0);
            lowness = smoothstep(370.0, 400.0, cy);
            if cy < 300.0 {
                a *= 0.5;
            }
        }
    }
    (a, low, lowness)
}

/// The sky with its cloud streaks, as the painter wants it to look.
fn sky_look(n: &Fbm, x: f32, y: f32) -> Rgb {
    let base = sky_at(n, x, y);
    let (a, low, lowness) = streaks(n, x, y);
    if a <= 0.0 {
        return base;
    }
    let g = (-((x - GX) / 260.0).powi(2)).exp();
    // low streaks are already in the shadow of the earth, backlit, their
    // undersides touched with the glow; high ones still take the sun (rose)
    let body = mix(hex("#968b95"), hex("#877a88"), g, Mix::Light);
    let under = mix(body, hex("#edb47a"), smoothstep(0.25, 0.9, low) * (0.35 + 0.6 * g), Mix::Light);
    let high = mix(hex("#d4b6b0"), hex("#c9a9a8"), smoothstep(-1.0, 1.0, low), Mix::Light);
    let cl = mix(high, under, lowness, Mix::Light);
    mix(base, cl, a * 0.85, Mix::Light)
}

// ckpt: from far
/// A height profile along x, tabulated finely.
struct Profile {
    x0: f32,
    step: f32,
    v: Vec<f32>,
}

impl Profile {
    fn new(x0: f32, x1: f32, step: f32, g: impl Fn(f32) -> f32) -> Self {
        let n = ((x1 - x0) / step) as usize + 2;
        Profile { x0, step, v: (0..n).map(|i| g(x0 + i as f32 * step)).collect() }
    }
    fn at(&self, x: f32) -> f32 {
        let t = (x - self.x0) / self.step;
        if t < 0.0 || t >= (self.v.len() - 1) as f32 {
            return 0.0;
        }
        let i = t as usize;
        let f = t - i as f32;
        self.v[i] * (1.0 - f) + self.v[i + 1] * f
    }
}

/// The town's roofline, height above the horizon at x: houses (gable ends
/// and broadsides), trees and poplars, the great church with its west
/// tower and spire, a gate tower, a second church with a baroque cap.
fn town_profile() -> Profile {
    let mut r = Rng::new(4711);
    // houses
    let mut houses: Vec<(f32, f32, f32, f32, bool)> = vec![];
    let mut x = 272.0;
    while x < 598.0 {
        let hw = r.range(3.0, 6.5);
        let cx = x + hw;
        let edge = smoothstep(272.0, 300.0, cx) * (1.0 - smoothstep(565.0, 598.0, cx));
        let eave = (r.range(3.5, 7.0) * (0.5 + 0.5 * edge)).max(2.0);
        let gable = r.f() < 0.55;
        let ridge = eave + if gable { hw * r.range(0.9, 1.3) } else { r.range(2.0, 4.0) };
        houses.push((cx, hw, eave, ridge, gable));
        x += hw * 2.0 + r.range(-1.5, 2.5);
    }
    // trees: (x, half width, top, lumpy seed); poplars are narrow and tall
    let trees: Vec<(f32, f32, f32, f32)> = vec![
        (284.0, 7.0, 9.0, 1.0),
        (318.0, 6.0, 12.5, 2.0),
        (352.0, 8.0, 11.0, 3.0),
        (455.0, 5.5, 13.0, 4.0),
        (488.0, 7.5, 12.0, 5.0),
        (553.0, 9.0, 13.5, 6.0),
        (583.0, 6.0, 8.0, 7.0),
        (295.0, 2.2, 22.0, 8.0),
        (299.5, 1.8, 18.0, 9.0),
        (566.0, 2.3, 24.0, 10.0),
    ];
    let lump = Fbm::new(33, 3, 6.0);
    Profile::new(250.0, 620.0, 0.05, move |x| {
        let mut h: f32 = 0.0;
        for &(cx, hw, eave, ridge, gable) in &houses {
            let d = (x - cx).abs();
            if d < hw {
                let r = if gable { eave + (ridge - eave) * (1.0 - d / hw) } else { eave + (ridge - eave) * ((hw - d) / 2.5).min(1.0) };
                h = h.max(r);
            }
        }
        for &(cx, hw, top, s) in &trees {
            let d = (x - cx) / hw;
            if d.abs() < 1.0 {
                let shape = (1.0 - d * d).max(0.0).powf(if hw < 3.0 { 0.35 } else { 0.5 });
                h = h.max(top * shape * (0.85 + 0.25 * lump.get(x * 1.5, s * 10.0)));
            }
        }
        // the great church: nave with a steep roof and hipped ends
        if (381.0..=429.0).contains(&x) {
            h = h.max(20.0 + 10.0 * ((x - 381.0) / 5.0).min((429.0 - x) / 5.0).min(1.0));
        }
        // a ridge turret
        if (x - 412.0).abs() < 0.7 {
            h = h.max(41.0);
        }
        if (x - 412.0).abs() < 1.6 {
            h = h.max(33.0);
        }
        // choir, lower, with a rounded apse
        if (429.0..=445.0).contains(&x) {
            h = h.max(16.0 + 7.0 * ((445.0 - x) / 6.0).clamp(0.0, 1.0).sqrt());
        }
        // the west tower in two stages, corner pinnacles, the spire
        if (367.0..=381.0).contains(&x) {
            h = h.max(56.0);
        }
        if (368.2..=379.8).contains(&x) {
            h = h.max(63.0);
        }
        for px in [368.8f32, 379.2] {
            if (x - px).abs() < 0.6 {
                h = h.max(67.5);
            }
        }
        let sd = (x - 374.0).abs();
        if sd < 4.6 {
            let t = 1.0 - sd / 4.6;
            h = h.max(63.0 + 40.0 * t.powf(1.6));
        }
        if sd < 0.35 {
            h = h.max(107.0);
        }
        // a gate tower with a pointed roof
        if (464.0..=470.0).contains(&x) {
            h = h.max(22.0);
        }
        let gd = (x - 467.0).abs();
        if gd < 3.8 {
            h = h.max(22.0 + 9.0 * (1.0 - gd / 3.8));
        }
        // the second church: square tower, bell cap, lantern, needle
        if (510.0..=521.0).contains(&x) {
            h = h.max(44.0);
        }
        let bd = (x - 515.5).abs();
        if bd < 6.2 {
            h = h.max(44.0 + 8.0 * (1.0 - (bd / 6.2).powi(2)).max(0.0).powf(0.7));
        }
        if bd < 1.8 {
            h = h.max(57.0);
        }
        if bd < 2.6 && bd >= 1.8 {
            h = h.max(53.0);
        }
        if bd < 0.45 {
            h = h.max(63.5);
        }
        if (521.0..=549.0).contains(&x) {
            h = h.max(14.0 + 8.0 * ((x - 521.0) / 4.0).min((549.0 - x) / 4.0).min(1.0));
        }
        h
    })
}

/// The far shore: low woods and single trees along the horizon, broken by
/// open stretches (height above the horizon).
fn shore_profile(n: Fbm) -> Profile {
    Profile::new(-10.0, 1010.0, 0.1, move |x| {
        let v = n.get(x * 0.8, 50.0) + 0.3 * n.get(x * 2.5, 70.0);
        let woods = smoothstep(-0.15, 0.2, v) * (3.0 + 3.2 * (0.5 + 0.5 * n.get(x * 5.0, 7.0)) + 1.2 * n.get(x * 14.0, 9.0));
        let town = smoothstep(240.0, 275.0, x) * (1.0 - smoothstep(600.0, 640.0, x)) * 2.2;
        woods.max(town).max(0.0) + 1.3 + 0.5 * n.get(x * 7.0, 3.0)
    })
}

// ckpt: end

// ckpt: from water
/// The lagoon: how far its near shore lies below the horizon at x.
fn lagoon_w(n: &Fbm, x: f32) -> f32 {
    let w = 20.0 * (1.0 - smoothstep(20.0, 660.0, x)).powf(0.8) + 2.5 * n.get(x * 1.2, 90.0) + 1.5 * n.get(x * 5.0, 91.0);
    w.max(0.0)
}

// ckpt: end

// ckpt: from pools
/// Pools of standing water in the meadows (flooded hollows): each a long
/// level shape, (x middle, depth below the horizon, half length, half
/// depth). Laid out on the ground, so they are flat and wide.
const POOLS: [(f32, f32, f32, f32); 3] = [(215.0, 23.0, 105.0, 4.6), (505.0, 41.0, 48.0, 4.2), (330.0, 62.0, 40.0, 5.0)];

/// How far inside a pool a point is (>0 inside), in units of its depth.
fn pool_in(n: &Fbm, x: f32, y: f32) -> (f32, f32) {
    let mut best = (-1.0f32, 0.0f32);
    for (i, &(px, ps, hl, hd)) in POOLS.iter().enumerate() {
        let k = i as f32 * 71.0;
        let u = (x - px) / hl + 0.12 * n.get(x * 0.8 + k, k + 60.0);
        if u.abs() > 1.3 {
            continue;
        }
        // the shore: ragged along its length, pointed at the ends
        let env = (1.0 - u.abs().powf(2.4)).max(0.0);
        let top = HZ + ps - hd * env.powf(0.45) * (0.75 + 0.5 * n.get(x * 2.0 + k, k));
        let bot = HZ + ps + hd * env.powf(0.4) * (0.6 + 0.8 * (0.5 + 0.5 * n.get(x * 1.2 + k, k + 30.0))) + 0.9 * env.sqrt() * n.get(x * 4.0, k);
        let inside = (y - top).min(bot - y);
        if inside > best.0 {
            best = (inside, (y - top) / (bot - top).max(0.1));
        }
    }
    best
}

// ckpt: end

// ckpt: from rise
/// The foreground rise: its crest y at x.
fn crest(n: &Fbm, x: f32) -> f32 {
    // low on the left, a long rise, a knoll on the right
    const P: [(f32, f32); 9] = [(-20.0, 708.0), (250.0, 694.0), (450.0, 668.0), (580.0, 628.0), (680.0, 566.0), (750.0, 522.0), (800.0, 507.0), (870.0, 514.0), (1020.0, 546.0)];
    let mut y = P[P.len() - 1].1;
    for w in P.windows(2) {
        if x <= w[1].0 {
            let t = ((x - w[0].0) / (w[1].0 - w[0].0)).clamp(0.0, 1.0);
            y = w[0].1 + (w[1].1 - w[0].1) * t * t * (3.0 - 2.0 * t);
            break;
        }
    }
    y + 3.0 * n.get(x * 0.5, 20.0) + 0.7 * n.get(x * 2.0, 60.0)
}

// ckpt: end

fn main() {
    let o = Run::new("r14_p2");
    let st = Style::friedrich();
    let pal = &st.palette;
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = f.height();
    let n = Fbm::new(7, 4, 300.0);
    let sky_pal = pal.only(&["lead white", "pale smalt", "cobalt blue", "yellow ochre", "chrome yellow", "vermilion", "red earth", "raw umber"]);
    let sky = Mask::from_fn(f, |_, y| 1.0 - smoothstep(HZ + 6.0, HZ + 10.0, y));

    if o.stage("sky", &mut c, &mut rng) {
        // a thin dead color first, a little darker and duller than the sky
        // will be, so the thin ends of the lay-in show it and not the ground
        let hd = st.broad().palette(&sky_pal).color(|x, y| shift(sky_at(&n, x, y), -0.02, 0.0, -0.01)).angle(|_, _| 0.0).cross(0.3).length(50.0, 160.0).coverage(3.6).medium(0.2).clip(true);
        c.work(&sky, &hd, 10);
        c.dry();
        // lay-in: thin, long, nearly level arcs; the streaks are in the
        // color the brush carries, laid into the same wet paint
        let hd = st.broad().palette(&sky_pal).color(|x, y| sky_look(&n, x, y)).angle(|_, _| 0.0).angle_jitter(0.05).length(60.0, 200.0).coverage(4.2).medium(0.32).clip(true);
        c.work(&sky, &hd, 11);
        let hd = st.broad().palette(&sky_pal).color(|x, y| sky_look(&n, x, y)).angle(|_, _| 0.0).cross(0.12).length(40.0, 140.0).coverage(1.6).medium(0.45).clip(true);
        c.work(&sky, &hd, 12);
        if let Some(b) = st.blend() {
            c.work(&sky, &b.angle(|_, _| 0.0).angle_jitter(0.1), 13);
        }
        // the glow catching the undersides of the low streaks, laid thin
        // into the still wet sky with a small filbert, level
        let lit_under = Mask::from_fn(f, |x, y| {
            let (a, low, lowness) = streaks(&n, x, y);
            smoothstep(0.5, 0.75, a) * smoothstep(0.25, 0.7, low) * lowness * (-((x - GX) / 330.0).powi(2)).exp().max(0.25)
        });
        let hd = st
            .body()
            .tool(Tool { lay: 0.6, ..Tool::filbert(3.0) })
            .palette(&sky_pal)
            .color(|x, y| mix(sky_at(&n, x, y), hex("#f1b477"), 0.55, Mix::Light))
            .angle(|_, _| 0.0)
            .angle_jitter(0.03)
            .curve(0.01, 0.0)
            .length(8.0, 30.0)
            .coverage(1.6)
            .pressure(0.35, 0.55)
            .medium(0.45)
            .clip(true)
            .fill(false);
        c.work(&lit_under, &hd, 16);
        // stipple into the wet: each dip moves a little from what is there
        // toward the sky wanted (never a fixed mix)
        let sp = Stipple::new(Tool::stippler(3.0))
            .mixed(&sky_pal, 0.45)
            .color_over(|x, y, u| mix(u, sky_look(&n, x, y), 0.45, Mix::Light))
            .coverage(|_, _| 1.8)
            .pressure(0.45, 0.8)
            .dips(18, 0.4, 0.5)
            .cluster(0.3, None)
            .clip(true);
        c.stipple(&sky, &sp, 14);
        c.dry();
        // once dry: a finer stipple barely lighter than the field, denser
        // toward the glow
        let sp = Stipple::new(Tool::stippler(1.7))
            .mixed(&sky_pal, 0.5)
            .color_over(|_, _, u| shift(u, 0.012, 0.0, 0.002))
            .coverage(|x, y| 0.5 + 1.3 * smoothstep(120.0, HZ, y) * (0.4 + 0.6 * (-((x - GX) / 320.0).powi(2)).exp()))
            .pressure(0.4, 0.75)
            .dips(22, 0.35, 0.6)
            .cluster(0.35, None)
            .clip(true);
        c.stipple(&sky, &sp, 15);
        c.dry();
    }

    // ---- the young moon: lit limb toward where the sun went down
    let (mx, my, mr) = MOON;
    let (sx, sy) = (GX - mx, HZ + 80.0 - my);
    let sl = (sx * sx + sy * sy).sqrt();
    let (ux, uy) = (sx / sl, sy / sl);
    let crescent = Mask::from_fn(f, move |x, y| {
        let d0 = ((x - mx).powi(2) + (y - my).powi(2)).sqrt();
        let (dx, dy) = (x - (mx - ux * mr * 0.3), y - (my - uy * mr * 0.3));
        let d1 = (dx * dx + dy * dy).sqrt();
        (1.0 - smoothstep(mr - 0.6, mr + 0.4, d0)) * smoothstep(mr * 0.98 - 0.5, mr * 0.98 + 0.5, d1)
    });
    let old_moon = Mask::from_fn(f, move |x, y| {
        let d0 = ((x - mx).powi(2) + (y - my).powi(2)).sqrt();
        1.0 - smoothstep(mr - 0.8, mr + 0.3, d0)
    });
    if o.stage("moon", &mut c, &mut rng) {
        // the ashen light: the rest of the disc barely lighter than the sky
        let hd = st.detail().tool(Tool::round_sable(1.6)).color_over(|_, _, u| shift(u, 0.014, 0.002, 0.0)).angle(move |x, y| (y - my).atan2(x - mx) + FRAC_PI_2).angle_jitter(0.3).length(3.0, 8.0).coverage(2.4).pressure(0.5, 0.7).clip(true);
        c.work(&old_moon, &hd, 20);
        c.dry();
        let hd = st
            .detail()
            .tool(Tool::round_sable(1.5))
            .color(|_, _| hex("#efe6cb"))
            .angle(move |x, y| (y - my).atan2(x - mx) + FRAC_PI_2)
            .angle_jitter(0.15)
            .length(3.0, 9.0)
            .coverage(3.0)
            .pressure(0.55, 0.8)
            .clip(true);
        c.work(&crescent, &hd, 21);
        c.dry();
    }

    // ---- a few rooks going home toward the town, far off: each two wing
    // strokes pressed at the body and lifted toward the tips
    if o.stage("birds", &mut c, &mut rng) {
        let mut r = Rng::new(3131);
        let mut b = Held::new(Tool { point: 1.0, ..Tool::round_sable(0.9) }, 131);
        let flock = [(588.0f32, 318.0f32), (601.0, 311.0), (612.0, 322.0), (627.0, 308.0), (641.0, 317.0), (566.0, 329.0)];
        for &(x, y) in &flock {
            let sz = r.range(2.2, 3.4);
            let lift = r.range(-0.5, 0.4);
            b.reload(pal.paint(hex("#4a4650"), 0.1), 0.5);
            for side in [-1.0f32, 1.0] {
                let tip = (x + side * sz, y - sz * (0.3 + lift * side * 0.4));
                let mid = (x + side * sz * 0.45, y - sz * 0.32);
                c.drag(&mut b, &Gesture::new(vec![(x, y), mid, tip]).pressure(0.6, 0.0).ramps(0.1, 0.75).shake(0.4), None);
            }
        }
        c.dry();
    }

    // ---- far shore and the town on it
    let tp = town_profile();
    let town_h = |x: f32| tp.at(x);
    let sp_ = shore_profile(n);
    let shore_h = |x: f32| sp_.at(x);
    let town = Mask::from_fn(f, |x, y| {
        let th = town_h(x);
        if th <= 0.0 {
            return 0.0;
        }
        smoothstep(HZ - th - 0.35, HZ - th + 0.35, y) * (1.0 - smoothstep(HZ + 3.0, HZ + 4.0, y))
    });
    let shore = Mask::from_fn(f, |x, y| smoothstep(HZ - shore_h(x) - 0.6, HZ - shore_h(x) + 0.6, y) * (1.0 - smoothstep(HZ + 3.0, HZ + 4.0, y)));
    // the windmill on the far shore, left of the town: a post mill
    let (wmx, wmy) = (206.0, HZ - 1.5);
    let mill = Mask::from_fn(f, move |x, y| {
        let v = wmy - y;
        let body = (x - wmx).abs() < 3.3 && (5.5..=13.5).contains(&v);
        let roof = v > 13.5 && v < 15.8 && (x - wmx).abs() < 3.3 - (v - 13.5) * 1.1;
        let post = (x - wmx).abs() < 0.8 + (5.5 - v).max(0.0) * 0.45 && (0.0..=5.5).contains(&v);
        if body || roof || post { 1.0 } else { 0.0 }
    });
    if o.stage("far", &mut c, &mut rng) {
        // the far shore: a pale blue-gray strip, barely darker than the
        // glow, laid level; then the far woods on it in upright hatching
        let shore_col = |x: f32, _y: f32| mix(hex("#9d9492"), hex("#ad9f95"), (-((x - GX) / 250.0).powi(2)).exp(), Mix::Light);
        let hd = st.detail().tool(Tool::round_sable(1.3)).color(shore_col).angle(|_, _| 0.0).angle_jitter(0.1).length(4.0, 12.0).coverage(3.2).clip(true);
        c.work(&shore, &hd, 30);
        let woods = shore.clone().mul_fn(|x, y| smoothstep(HZ - 1.8, HZ - 2.6, y) * smoothstep(2.2, 3.0, shore_h(x)));
        let hd = st.hatch().tool(Tool::round_sable(1.3)).color(|x, y| shift(shore_col(x, y), -0.02, 0.0, 0.0)).angle(|_, _| 1.5).angle_jitter(0.3).length(2.0, 5.0).coverage(3.0).clip(true);
        c.work(&woods, &hd, 31);
        c.dry();
        // the town: backlit, cool, one flat dark, a little warmer and
        // lighter low down where the evening haze lies
        let hd = st
            .detail()
            .tool(Tool::round_sable(1.4))
            .color(|x, y| {
                let mist = smoothstep(HZ - 14.0, HZ, y);
                // the great church nearest and darkest; the roofs toward the
                // town's ends a little farther into the air
                let air = 0.22 * smoothstep(20.0, 170.0, (x - 400.0).abs());
                mix(hex("#686572"), hex("#948c8b"), mist * 0.75 + air + 0.1 * (-((x - GX) / 120.0).powi(2)).exp(), Mix::Light)
            })
            .angle(|x, y| if town_h(x) > 24.0 && y < HZ - 18.0 { FRAC_PI_2 } else { 0.08 })
            .angle_jitter(0.12)
            .length(2.0, 7.0)
            .coverage(3.4)
            .clip(true);
        c.work(&town, &hd, 32);
        let hd = st.detail().tool(Tool::round_sable(1.2)).color(|_, _| hex("#7a7479")).angle(|_, _| FRAC_PI_2).length(2.0, 5.0).coverage(3.0).clip(true);
        c.work(&mill, &hd, 33);
        c.dry();
        // the belfry openings: the evening seen through the towers, a few
        // narrow upright touches a step lighter than the stone
        let mut sb = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.0) }, 37);
        for &(x, y0, y1) in &[(371.6f32, HZ - 59.5, HZ - 53.5), (376.4, HZ - 59.5, HZ - 53.5), (513.4, HZ - 40.5, HZ - 35.5), (517.6, HZ - 40.5, HZ - 35.5)] {
            sb.reload(pal.paint(hex("#a3988f"), 0.15), 0.5);
            c.drag(&mut sb, &Gesture::new(vec![(x, y0), (x + 0.05, (y0 + y1) * 0.5), (x, y1)]).pressure(0.55, 0.45).ramps(0.1, 0.2).shake(0.2), None);
        }
        // the mill's sails: four arms, a lattice too fine to see, so each
        // is a narrow stroke out from the hub
        let mut b = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.1) }, 34);
        let hub = (wmx + 0.4, wmy - 11.0);
        for k in 0..4 {
            let a = 0.42 + k as f32 * FRAC_PI_2;
            b.reload(pal.paint(hex("#77717a"), 0.1), 0.7);
            let tip = (hub.0 + a.cos() * 12.5, hub.1 + a.sin() * 12.5);
            c.drag(&mut b, &Gesture::new(vec![hub, ((hub.0 + tip.0) * 0.5, (hub.1 + tip.1) * 0.5), tip]).pressure(0.75, 0.55).ramps(0.05, 0.3).shake(0.3), None);
        }
        c.dry();
        c.dry();
    }

    // ---- the lagoon under the glow: it gives back the sky, a little darker
    let lagoon = Mask::from_fn(f, |x, y| {
        let w = lagoon_w(&n, x);
        if w <= 0.2 {
            return 0.0;
        }
        smoothstep(HZ + 1.2, HZ + 2.2, y) * (1.0 - smoothstep(HZ + w - 0.6, HZ + w + 0.6, y))
    });
    let water_look = |x: f32, y: f32| {
        let d = (y - HZ).max(0.0);
        let s = sky_at(&n, x, HZ - 4.0 - d * 2.0);
        let th = town_h(x).max(shore_h(x) * 0.8);
        let refl = if th > 0.0 && d < th * 0.9 { (0.6 * (1.0 - d / (th * 0.9))).max(0.0) } else { 0.0 };
        let s = mix(s, hex("#7a7480"), refl, Mix::Light);
        shift(s, -0.05 - 0.004 * d, 0.0, -0.008)
    };
    if o.stage("water", &mut c, &mut rng) {
        let hd = st.body().color(water_look).angle(|_, _| 0.0).angle_jitter(0.015).curve(0.01, 0.0).length(20.0, 70.0).coverage(3.2).clip(true);
        c.work(&lagoon, &hd, 41);
        if let Some(b) = st.blend() {
            c.work(&lagoon, &b.tool(Tool { pickup: 0.15, run: 45.0, ..Tool::badger(12.0) }).angle(|_, _| 0.0).angle_jitter(0.01).length(40.0, 120.0), 42);
        }
        c.dry();
    }

    // ---- the meadows: value steps toward us, level strokes, larger near
    let land = Mask::from_fn(f, |x, y| {
        let w = lagoon_w(&n, x);
        let water = if w > 0.2 { 1.0 - smoothstep(HZ + w - 0.6, HZ + w + 0.6, y) } else { 0.0 };
        smoothstep(HZ + 1.0, HZ + 2.5, y) * (1.0 - water)
    });
    let land_look = move |x: f32, y: f32| {
        let d = (y - HZ).max(0.0);
        let t = (d / (h - HZ)).clamp(0.0, 1.0);
        // fields: bands that widen toward us
        let band = (d.sqrt() * 2.1 + 0.6 * n.get(x * 0.3, d)).sin();
        let base = gradient(&[(0.0, hex("#786c68")), (0.05, hex("#5f5550")), (0.2, hex("#4a413b")), (0.5, hex("#3a322d")), (1.0, hex("#2b2521"))], t, Mix::Light);
        let tint = if band > 0.35 { shift(base, 0.014, -0.004, 0.008) } else if band < -0.5 { shift(base, -0.012, 0.004, -0.003) } else { base };
        // the glow still reaches the far meadows
        let g = (-((x - GX) / 300.0).powi(2)).exp() * (1.0 - smoothstep(0.0, 0.12, t));
        mix(tint, hex("#9a8676"), g * 0.3, Mix::Light)
    };
    if o.stage("land", &mut c, &mut rng) {
        // a thin brown dead color first, so the lay-in's thin places show
        // earth, not the red ground
        let hd = st.body().tool(Tool { lay: 0.8, ..Tool::filbert(12.0) }).color(|x, y| shift(land_look(x, y), -0.03, 0.01, 0.0)).angle(|_, _| 0.0).cross(0.25).length(30.0, 80.0).coverage(2.6).medium(0.3).clip(true);
        c.work(&land, &hd, 50);
        c.dry();
        let far = land.clone().mul_fn(|_, y| 1.0 - smoothstep(HZ + 30.0, HZ + 50.0, y));
        let hd = st.body().tool(Tool { lay: 0.7, push: 0.06, ..Tool::filbert(6.0) }).color(land_look).angle(|_, _| 0.0).angle_jitter(0.03).curve(0.015, 0.1).length(10.0, 34.0).coverage(3.4).medium(0.25).mix_jitter(0.04).clip(true);
        c.work(&far, &hd, 51);
        let near = land.clone().mul_fn(|_, y| smoothstep(HZ + 25.0, HZ + 45.0, y));
        let hd = st.body().tool(Tool { lay: 0.75, push: 0.06, ..Tool::filbert(10.0) }).color(land_look).angle(|x, y| 0.04 * n.get(x, y)).angle_jitter(0.05).curve(0.02, 0.1).length(25.0, 80.0).coverage(3.6).medium(0.22).mix_jitter(0.03).clip(true);
        c.work(&near, &hd, 52);
        c.wait(120.0);
        // the meadow's grain: short strokes a step off the paint under them,
        // nearly level, larger toward us
        for (k, &(s0, s1, w, l0, l1)) in [(22.0f32, 70.0f32, 1.6f32, 3.0f32, 8.0f32), (60.0, 260.0, 2.8, 6.0, 16.0)].iter().enumerate() {
            let m = land.clone().mul_fn(move |_, y| smoothstep(HZ + s0, HZ + s0 + 10.0, y) * (1.0 - smoothstep(HZ + s1 - 10.0, HZ + s1, y)));
            let hd = st
                .hatch()
                .tool(Tool::round_sable(w))
                .color_over(|x, y, u| {
                    let k = n.get(x * 3.0, y * 8.0);
                    if k > 0.2 { shift(u, 0.018, 0.004, 0.012) } else { shift(u, -0.03, 0.0, -0.004) }
                })
                .angle(|x, y| 0.08 * n.get(x * 2.0, y * 2.0))
                .angle_jitter(0.12)
                .length(l0, l1)
                .coverage(1.3)
                .fill(false);
            c.work(&m, &hd, 53 + k as u64);
        }
        c.dry();
    }

    // ---- pools of standing water: the sky given back, a step darker,
    // the far shore's reflection a dark band along their far side
    let pools = Mask::from_fn(f, |x, y| smoothstep(0.05, 0.6, pool_in(&n, x, y).0));
    let pool_look = |x: f32, y: f32| {
        let s = (y - HZ).max(0.0);
        let sky = shift(sky_at(&n, x, HZ - 3.0 - s * 0.5), -0.07 - 0.0012 * s, 0.0, -0.008);
        let (_, v) = pool_in(&n, x, y);
        mix(sky, hex("#3e3733"), 0.8 * (1.0 - smoothstep(0.1, 0.4, v)), Mix::Light)
    };
    if o.stage("pools", &mut c, &mut rng) {
        let hd = st.body().tool(Tool { lay: 0.7, ..Tool::filbert(4.0) }).color(pool_look).angle(|_, _| 0.0).angle_jitter(0.01).curve(0.0, 0.0).length(10.0, 40.0).coverage(3.4).clip(true);
        c.work(&pools, &hd, 55);
        c.dry();
        // the meadow carried back over the shores in short level strokes,
        // so the water lies in the grass and not on it
        let lagoon_edge = Mask::from_fn(f, |x, y| {
            let w = lagoon_w(&n, x);
            if w <= 0.5 {
                return 0.0;
            }
            1.0 - smoothstep(0.6, 1.4, (y - HZ - w).abs())
        });
        let shore = pools.dilate(1.2).subtract(&pools.erode(0.5)).union(&lagoon_edge).mul_fn(|x, y| 0.4 + 0.6 * (0.5 + 0.5 * n.get(x * 6.0, y * 6.0)));
        let hd = st.detail().tool(Tool::round_sable(1.4)).color(land_look).angle(|_, _| 0.0).angle_jitter(0.15).length(2.0, 8.0).coverage(0.9).clip(true).fill(false);
        c.work(&shore, &hd, 57);
        c.dry();
    }

    // ---- a group of trees on the left, sized by their depth
    let grove = {
        // (x, crown middle depth below the horizon, half width, height)
        // (x, depth of the foot below the horizon, half width, height)
        const T: [(f32, f32, f32, f32); 3] = [(74.0, 64.0, 15.0, 46.0), (99.0, 62.0, 9.0, 31.0), (54.0, 66.0, 8.0, 21.0)];
        let lump = Fbm::new(55, 3, 7.0);
        let mut r = Rng::new(56);
        // each crown is a few masses of foliage, not a ball
        let mut blobs: Vec<(f32, f32, f32, f32)> = vec![];
        for &(tx, s, hw, ht) in T.iter() {
            let foot = HZ + s;
            let k = 3 + (hw / 4.0) as usize;
            for _ in 0..k {
                let bx = tx + r.range(-0.65, 0.65) * hw;
                let by = foot - ht * r.range(0.45, 0.85);
                blobs.push((bx, by, hw * r.range(0.35, 0.6), ht * r.range(0.16, 0.26)));
            }
            blobs.push((tx, foot - ht * 0.8, hw * 0.45, ht * 0.2));
        }
        Mask::from_fn(f, move |x, y| {
            let mut v: f32 = 0.0;
            for (i, &(bx, by, rx, ry)) in blobs.iter().enumerate() {
                let e = (((x - bx) / rx).powi(2) + ((y - by) / ry).powi(2)).sqrt();
                let r = 1.0 + 0.35 * lump.get(x * 2.0 + i as f32 * 50.0, y * 2.0);
                v = v.max(1.0 - smoothstep(r - 0.1, r + 0.05, e));
            }
            for &(tx, s, hw, ht) in T.iter() {
                let foot = HZ + s;
                if (x - tx - 0.03 * (foot - y)).abs() < 0.08 * hw + 0.3 && y > foot - ht * 0.6 && y < foot + 0.5 {
                    v = 1.0;
                }
            }
            // undergrowth: a low lumpy band from one side of the group to the other
            let (x0, x1) = (40.0, 112.0);
            if x > x0 && x < x1 {
                let t = (x - x0) / (x1 - x0);
                let foot = HZ + 66.0 - 4.0 * t;
                let ht = (3.0 + 3.5 * (0.5 + 0.5 * lump.get(x * 1.5, 900.0))) * (t * (1.0 - t) * 4.0).sqrt();
                v = v.max(smoothstep(foot - ht - 0.4, foot - ht + 0.4, y) * (1.0 - smoothstep(foot, foot + 0.8, y)));
            }
            v
        })
    };
    if o.stage("grove", &mut c, &mut rng) {
        // the grove: a dark mass first, then its edge broken by hatching
        let hd = st.hatch().tool(Tool::round_sable(1.6)).color(|x, y| mix(hex("#2f2b27"), hex("#4a403a"), 0.5 + 0.5 * n.get(x * 5.0, y * 5.0), Mix::Light)).angle(|_, _| -FRAC_PI_2).angle_jitter(0.6).length(2.0, 6.0).coverage(3.4).clip(true);
        c.work(&grove, &hd, 92);
        c.dry();
    }

    // ---- the rise in front: one dark, strokes along its slope
    let rise = Mask::from_fn(f, |x, y| smoothstep(crest(&n, x) - 0.8, crest(&n, x) + 0.8, y));
    let rise_look = move |x: f32, y: f32| {
        let d = y - crest(&n, x);
        let base = mix(hex("#2e2a23"), hex("#26221d"), smoothstep(30.0, 180.0, d), Mix::Light);
        // the top, turned to the sky, takes a little of its cool light
        let top = (1.0 - smoothstep(4.0, 45.0, d)) * smoothstep(560.0, 760.0, x);
        let base = mix(base, hex("#46423d"), 0.45 * top, Mix::Light);
        mix(base, hex("#3b342a"), 0.35 * (0.5 + 0.5 * n.get(x * 1.5, y * 2.0)), Mix::Light)
    };
    if o.stage("rise", &mut c, &mut rng) {
        let slope = move |x: f32| {
            let e = 3.0;
            (crest(&n, x + e) - crest(&n, x - e)).atan2(2.0 * e)
        };
        let hd = st.body().tool(Tool { lay: 0.8, ..Tool::filbert(10.0) }).color(rise_look).angle(move |x, _| slope(x)).angle_jitter(0.12).length(20.0, 60.0).coverage(3.2).clip(true);
        c.work(&rise, &hd, 61);
        c.wait(90.0);
        // heath: short upright hatching a step darker, some rust and gray
        // green, once the lay-in has set
        let heath = rise.clone().mul_fn(|x, y| smoothstep(0.0, 8.0, y - crest(&n, x)) * (0.55 + 0.45 * n.get(x * 2.0, y * 2.0)));
        let hd = st
            .hatch()
            .tool(Tool::round_sable(2.4))
            .color_over(|x, y, u| {
                let k = n.get(x * 6.0, y * 6.0);
                if k > 0.25 { shift(u, -0.02, 0.012, 0.012) } else if k < -0.25 { shift(u, -0.01, -0.01, 0.0) } else { shift(u, -0.03, 0.0, 0.0) }
            })
            .angle(|x, y| -FRAC_PI_2 + 0.3 * n.get(x * 4.0, y * 4.0))
            .angle_jitter(0.3)
            .length(4.0, 10.0)
            .coverage(1.2)
            .fill(false);
        c.work(&heath, &hd, 62);
        c.dry();
    }

    // ---- dry grass, flicked up last: tufts in clumps, larger near, mostly
    // dark blades, a few catching the sky on the skyline
    if o.stage("grass", &mut c, &mut rng) {
        let mut r = Rng::new(808);
        let mut b = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.3) }, 81);
        let dark = pal.paint(hex("#211d18"), 0.1);
        let mid = pal.paint(hex("#3e3528"), 0.1);
        let lit = pal.paint(hex("#5d4e3c"), 0.15);
        let mut tufts: Vec<(f32, f32, f32)> = vec![];
        // along the skyline: clumps and gaps, the edge made of grass
        let mut x = 330.0;
        while x < 1010.0 {
            let clump = (0.5 + 0.5 * (n.get(x * 2.5, 400.0) + 0.4 * n.get(x * 9.0, 410.0) + 0.5 * n.get(x * 0.8, 420.0))).clamp(0.0, 1.0);
            if r.f() < clump.powi(3) * 2.0 {
                let y = crest(&n, x) + r.range(0.5, 4.0);
                tufts.push((x, y, r.range(5.0, 12.0) * (0.6 + 0.9 * clump)));
            }
            x += r.range(1.5, 5.5);
        }
        // down the slope: patches of grass, bare between, bigger toward us
        for _ in 0..80 {
            let px = r.range(-20.0, 1020.0);
            let cy = crest(&n, px);
            let py = r.range(cy + 10.0, h + 10.0);
            let near = ((py - cy) / (h - cy).max(1.0)).clamp(0.0, 1.0);
            let rad = 10.0 + 30.0 * near;
            let count = 4 + (r.f() * 14.0 * (0.6 + near)) as usize;
            for _ in 0..count {
                let (dx, dy) = (r.normal() * rad, r.normal() * rad * 0.3);
                let (x, y) = (px + dx, py + dy);
                if y < crest(&n, x) + 6.0 {
                    continue;
                }
                tufts.push((x, y, r.range(7.0, 13.0) * (0.8 + 1.6 * near)));
            }
        }
        for (i, &(x, y, ht)) in tufts.iter().enumerate() {
            let on_sky = y - crest(&n, x) < 4.0;
            let blades = 3 + (r.f() * 6.0) as usize;
            let lean0 = r.range(-0.35, 0.35) + 0.15;
            for k in 0..blades {
                let p = if on_sky && r.f() < 0.25 * smoothstep(620.0, 760.0, x) {
                    lit
                } else if r.f() < 0.3 {
                    mid
                } else {
                    dark
                };
                if k % 2 == 0 || i % 7 == 0 {
                    b.reload(p, 0.6);
                }
                let l = ht * r.range(0.45, 1.1);
                let a = -FRAC_PI_2 + lean0 + r.range(-0.45, 0.45);
                let bend = r.range(-0.25, 0.25);
                let x0 = x + r.range(-ht * 0.12, ht * 0.12);
                let p1 = (x0 + a.cos() * l * 0.5, y + a.sin() * l * 0.5);
                let a2 = a + bend;
                let p2 = (p1.0 + a2.cos() * l * 0.5, p1.1 + a2.sin() * l * 0.5);
                c.drag(&mut b, &Gesture::new(vec![(x0, y), p1, p2]).pressure(r.range(0.55, 0.85), 0.0).ramps(0.03, 0.8).shake(0.5), None);
            }
        }
        c.dry();
    }

    // ---- the two figures on the rise, looking toward the town
    if o.stage("figures", &mut c, &mut rng) {
        let dark = pal.paint(hex("#1d1b19"), 0.05);
        let coat = pal.paint(hex("#2c2e2b"), 0.1);
        let hair = pal.paint(hex("#2a221c"), 0.1);
        let rim = Some(pal.paint(hex("#b79a78"), 0.2));
        let (fx, fy) = (792.0, crest(&n, 792.0) + 1.5);
        figures::wanderer(&mut c, (fx, fy), 62.0, coat, dark, hair, 0.0, rim, 71);
        let g = Gown { dress: pal.paint(hex("#2d2826"), 0.1), shawl: Some(pal.paint(hex("#3b2f2a"), 0.1)), hair, skin: pal.paint(hex("#4a3d34"), 0.1), rim };
        figures::woman(&mut c, (774.0, crest(&n, 774.0) + 1.5), 57.0, &g, WomanPose::Standing, 72);
        c.dry();
    }

    // ---- a few blades over their feet, so they stand in the grass
    if o.stage("feet", &mut c, &mut rng) {
        let mut r = Rng::new(1212);
        let mut b = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.1) }, 121);
        for &fx in &[768.0f32, 775.0, 781.0, 787.0, 793.0, 798.0] {
            for _ in 0..3 {
                let x = fx + r.range(-2.5, 2.5);
                let y = crest(&n, x) + r.range(1.0, 3.0);
                let l = r.range(3.0, 6.5);
                let a = -FRAC_PI_2 + r.range(-0.5, 0.5);
                b.reload(pal.paint(hex(if r.f() < 0.8 { "#221e19" } else { "#3e3528" }), 0.1), 0.5);
                c.drag(&mut b, &Gesture::new(vec![(x, y), (x + a.cos() * l * 0.5, y + a.sin() * l * 0.5), (x + (a + 0.15).cos() * l, y + (a + 0.15).sin() * l)]).pressure(0.6, 0.0).ramps(0.03, 0.8).shake(0.5), None);
            }
        }
        c.dry();
    }

    // ---- the last glaze, a shade deeper toward the edges (Friedrich to
    // Carus), an even film that wanders slowly
    if o.stage("glaze", &mut c, &mut rng) {
        let w = Fbm::new(91, 2, 500.0);
        let vig = Pigment::transparent(hex("#5e4c42"));
        c.glaze(&vig, None, move |x, y| {
            let dx = (x - GX) / 620.0;
            let dy = (y - HZ + 60.0) / 520.0;
            let r = (dx * dx + dy * dy).sqrt();
            (0.18 * smoothstep(0.35, 1.2, r) * (1.0 + 0.3 * w.get(x, y))).max(0.0)
        });
    }
    let fin = Finish { cracks: Some(Cracks { width_um: Some(9.0), dirt: 0.2, depth_um: 12.0, ..Cracks::aged(0) }), ..Finish::aged(st.relief) };
    o.finish(&mut c, &mut rng, &fin);
}
