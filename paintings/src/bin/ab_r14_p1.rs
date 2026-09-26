//! r14_p1: "Morning in the Mountains" — an original landscape in the manner
//! of Caspar David Friedrich (see notes/r14_p1.md).
//!
//! From a granite summit before sunrise: the glow low on the left over
//! ranges that step back in value to the horizon, a sea of mist filling the
//! valleys between them, two dark wooded ridges rising out of it, and on
//! the near summit, among heath and boulders, a group of spruces standing
//! against the mist on the right.
//!
//!   cargo paint r14_p1 -- --full --width 2400
//!   cargo paint r14_p1 -- --full --width 2400 --crop x0,y0,x1,y1
//!   cargo paint r14_p1 -- --full --width 2400 --ckpt / --resume <stage>

use paint::color::{Mix, mix};
use paint::{Apply, Fbm, Form, Gesture, Ground, Held, Light, Mask, Palette, Rgb, Sdf, Stipple, Style, Tool, gradient, hex, smoothstep};
use paintings::run::{Finish, Run};
use std::f32::consts::PI;

const ASPECT: f32 = 1.42;
const H: f32 = 1000.0 / ASPECT;
/// The eye's level: the farthest ranges sink to it.
const HORIZON: f32 = 322.0;
/// Where the sun will come up (canvas x).
const GLOW_X: f32 = 300.0;

// ------------------------------------------------------------- the sky

fn sky_col(bands: &Fbm, x: f32, y: f32) -> Rgb {
    // height above the horizon as a fraction of the sky, wandered a little
    // by unequal horizontal bands (a hand doesn't lay a formula)
    let t0 = ((HORIZON - y) / HORIZON).clamp(0.0, 1.0);
    let t = (t0 + 0.035 * bands.get(x * 0.35, y * 4.0) * smoothstep(0.02, 0.2, t0)).clamp(0.0, 1.0);
    let base = gradient(
        &[
            (0.00, hex("#dca77e")),
            (0.035, hex("#e6bf90")),
            (0.09, hex("#e8d0a4")),
            (0.20, hex("#dcd5bb")),
            (0.36, hex("#c3c6c1")),
            (0.58, hex("#a2adbd")),
            (1.00, hex("#72849f")),
        ],
        t,
        Mix::Light,
    );
    let dx = (x - GLOW_X) / 330.0;
    let g = (-(dx * dx)).exp();
    // the glow over the sun's place: warm, falling off faster upward
    let glow = g * (-t / 0.16).exp();
    let c = mix(base, hex("#f0cc92"), 0.55 * glow, Mix::Light);
    // away from the glow, the low sky goes rose-lilac
    let away = (1.0 - g) * (-t / 0.12).exp();
    mix(c, hex("#c6a9a6"), 0.55 * away, Mix::Light)
}

/// Cloud streaks lying in the upper sky, level and thin: (alpha, under)
/// where `under` is 1 on the side that faces the glow (below).
fn streaks(n: &Fbm, x: f32, y: f32) -> (f32, f32) {
    // (center y, x0, x1, thickness, tilt)
    let list: [(f32, f32, f32, f32, f32); 6] = [
        (62.0, 430.0, 1060.0, 9.0, -0.02),
        (92.0, 560.0, 980.0, 6.0, -0.015),
        (128.0, -40.0, 380.0, 7.0, 0.02),
        (150.0, 620.0, 1040.0, 11.0, -0.01),
        (205.0, 740.0, 1050.0, 7.0, 0.0),
        (262.0, -30.0, 240.0, 5.0, 0.01),
    ];
    let mut a: f32 = 0.0;
    let mut under: f32 = 0.0;
    for (k, &(yc, x0, x1, th, tilt)) in list.iter().enumerate() {
        let yc = yc + tilt * (x - x0) + 6.0 * n.get(x * 0.4, k as f32 * 7.0);
        let env = smoothstep(x0, x0 + 120.0, x) * (1.0 - smoothstep(x1 - 150.0, x1, x));
        let th = 1.7 * th * (0.6 + 0.8 * n.get01(x * 0.8, k as f32 * 3.0 + 1.0));
        let d = (y - yc) / th;
        let v = (-(d * d)).exp() * env * smoothstep(0.25, 0.65, n.get01(x * 1.6, y * 0.6 + k as f32 * 5.0));
        if v > a {
            a = v;
            under = smoothstep(-0.3, 0.9, d);
        }
    }
    (a.min(1.0), under)
}

fn sky_with_clouds(bands: &Fbm, cn: &Fbm, x: f32, y: f32) -> Rgb {
    let s = sky_col(bands, x, y);
    let (a, under) = streaks(cn, x, y);
    if a <= 0.0 {
        return s;
    }
    // the bodies are a violet-gray darker than the sky round them; their
    // undersides take the glow
    let t = ((HORIZON - y) / HORIZON).clamp(0.0, 1.0);
    let body = mix(hex("#8f8c9e"), hex("#a69aa4"), smoothstep(0.3, 0.8, 1.0 - t), Mix::Light);
    let warm = mix(hex("#d7ad9a"), hex("#e2bd95"), smoothstep(0.3, 0.8, 1.0 - t), Mix::Light);
    let cl = mix(body, warm, under * 0.8, Mix::Light);
    mix(s, cl, 0.85 * a, Mix::Light)
}

// ---------------------------------------------------------- the ranges
// ckpt: from far

/// One step back into the distance: a crest, the range's own color there,
/// where its foot goes into the mist, and where it sinks altogether.
struct Layer<'a> {
    crest: &'a (dyn Fn(f32) -> f32 + Sync),
    /// the range's color at its crest (before the mist), by x
    body: &'a (dyn Fn(f32) -> Rgb + Sync),
    /// how deep (units below the crest) the mist starts and is complete
    depth: (f32, f32),
    /// 0 standing clear, 1 sunk in the mist, by x
    sink: &'a (dyn Fn(f32) -> f32 + Sync),
    /// painted down to (y at x): past the next layer's crest
    bottom: &'a (dyn Fn(f32) -> f32 + Sync),
    seed: u64,
}

/// The mist, wherever it lies: lit from the sky above, warmer and lighter
/// toward the glow and far off, grayer and cooler as it comes nearer.
fn mist_col(x: f32, y: f32) -> Rgb {
    let g = (-((x - GLOW_X) / 360.0).powi(2)).exp();
    let near = smoothstep(HORIZON + 10.0, 640.0, y);
    let far = mix(hex("#c2b0a8"), hex("#d4bea7"), g, Mix::Light);
    let mid = mix(hex("#aea6aa"), hex("#c1b2a5"), g, Mix::Light);
    let low = mix(hex("#94939e"), hex("#a39b9c"), g, Mix::Light);
    let m = if near < 0.5 { mix(far, mid, near * 2.0, Mix::Light) } else { mix(mid, low, near * 2.0 - 1.0, Mix::Light) };
    // long banks: lit tops a little lighter, the hollows between cooler;
    // billows whose tops take the sky and whose undersides go gray
    let bn = Fbm::new(61, 3, 120.0);
    let banks = bn.get(x * 0.25, y * 2.2);
    let bill = Fbm::new(62, 3, 45.0);
    let b0 = bill.get(x * 0.45, y * 1.4);
    let lift = (b0 - bill.get(x * 0.45, (y + 5.0) * 1.4)) * 1.5;
    // stronger nearer, where the mist is close enough to show its billows
    let k = 1.0 + 1.6 * smoothstep(HORIZON + 60.0, 520.0, y);
    let dl = 0.022 * banks + 0.03 * k * lift.clamp(-1.0, 1.0);
    paint::shift(m, dl, 0.0, -0.008 * (-dl).max(0.0) * 10.0)
}

/// A ridge half drowned in the mist sea, only its back showing.
fn ghost(n: &Fbm, x: f32) -> f32 {
    let hump = 16.0 * (1.0 - ((x - 610.0) / 190.0).powi(2)).max(0.0).powf(0.8);
    444.0 - hump + 3.0 * n.get(x * 2.0, 14.0) + 0.6 * firs_edge(x, 2.2, 3.0) * smoothstep(440.0, 520.0, x) * (1.0 - smoothstep(740.0, 800.0, x))
}

fn far1(n: &Fbm, x: f32) -> f32 {
    // the farthest: long and low, a few worn summits
    let bumps = 5.0 * (-((x - 120.0) / 80.0).powi(2)).exp() + 8.0 * (-((x - 640.0) / 110.0).powi(2)).exp() + 4.0 * (-((x - 880.0) / 60.0).powi(2)).exp();
    HORIZON + 5.0 - bumps - 3.0 * n.get01(x * 1.5, 0.3) - 1.0 * n.get(x * 8.0, 0.9)
}
fn far2(n: &Fbm, x: f32) -> f32 {
    // a long ridge rising from the left to a cone (the way the Schneekoppe
    // stands at the end of its ridge), just right of where the sun will
    // come up, then falling steeply into the mist
    let ridge = HORIZON + 9.0 - 13.0 * smoothstep(0.0, 300.0, x) - 7.0 * (-((x - 95.0) / 55.0).powi(2)).exp();
    // asymmetric: a long flank toward the glow, a steeper one away; the
    // top worn round
    let u = if x < 362.0 { (x - 362.0) / 165.0 } else { (x - 362.0) / 92.0 };
    let cone = if u.abs() < 1.0 { 27.0 * (0.45 * (1.0 - u.abs()).powf(1.3) + 0.55 * (1.0 - u * u).max(0.0).powf(1.1)) } else { 0.0 };
    let ridge = ridge - 5.0 * (-((x - 228.0) / 40.0).powi(2)).exp();
    ridge - cone + 2.5 * n.get(x * 2.0, 1.1) + 1.0 * n.get(x * 9.0, 2.0) + 34.0 * smoothstep(420.0, 600.0, x)
}
fn far3(n: &Fbm, x: f32) -> f32 {
    // a jagged range coming in from the right, its summit off center
    let peak = 34.0 * (1.0 - ((x - 810.0) / 170.0).abs()).max(0.0).powf(1.4);
    let shoulder = 10.0 * (1.0 - ((x - 660.0) / 90.0).powi(2)).max(0.0);
    HORIZON + 64.0 - 22.0 * smoothstep(470.0, 640.0, x) - peak.max(shoulder) + 4.0 * n.get(x * 2.5, 3.3) + 1.8 * n.get(x * 11.0, 4.0)
}

/// The two wooded ridges rising out of the mist: crest y at canvas x.
fn ridge_l(n: &Fbm, x: f32) -> f32 {
    let base = 398.0 + 0.1 * x + 60.0 * smoothstep(230.0, 480.0, x);
    // where the ridge sinks into the mist its fir tops go with it (a
    // serrated mist edge would cut dark teeth into the ridge behind)
    base + 5.0 * n.get(x * 1.7, 5.0) + firs_edge(x, 3.1, 6.0) * (1.0 - smoothstep(280.0, 420.0, x))
}
fn ridge_r(n: &Fbm, x: f32) -> f32 {
    let base = 468.0 - 42.0 * smoothstep(500.0, 760.0, x) - 12.0 * smoothstep(800.0, 1000.0, x);
    base + 4.0 * n.get(x * 1.9, 6.0) + firs_edge(x, 2.5, 5.0) * smoothstep(520.0, 640.0, x)
}
/// A serrated line of small fir tops along a crest (negative = up).
fn firs_edge(x: f32, spacing: f32, h: f32) -> f32 {
    let k = (x / spacing).floor();
    let u = x / spacing - k;
    let r = |i: f32| ((i * 12.9898).sin() * 43758.545).fract().abs();
    // in groups, with open gaps
    let grp = ((x / 37.0).sin() * 0.5 + 0.5) * ((x / 13.0 + 1.0).sin() * 0.5 + 0.5);
    let hk = h * (0.35 + 0.65 * r(k)) * if r(k + 0.5) < 0.25 || grp < 0.12 { 0.15 } else { 1.0 };
    -hk * (1.0 - (2.0 * u - 1.0).abs()).powf(1.3)
}

/// The near summit's edge: y at canvas x.
fn fg_top(n: &Fbm, x: f32) -> f32 {
    let slope = 614.0 - 0.13 * x - 40.0 * smoothstep(600.0, 820.0, x);
    let knoll = 28.0 * (-((x - 150.0) / 110.0).powi(2)).exp();
    slope - knoll + 5.0 * n.get(x * 1.3, 8.0) + 2.0 * n.get(x * 6.0, 9.0)
}

/// Paint one layer: its body in level strokes, the mist mixed into its own
/// paint below an uneven line, fused there while wet.
fn paint_layer(c: &mut paint::Canvas, st: &Style, pal: &Palette, l: &Layer, medium: f32, stroke: (f32, f32), carry: f32) {
    let f = c.frame();
    let (crest, bottom, body, sink) = (l.crest, l.bottom, l.body, l.sink);
    // what lies just above the crest before this range goes on (the sky, or
    // a farther range), read off the canvas every unit along it
    let above: Vec<Rgb> = (0..=1001).map(|i| c.sample(i as f32, crest(i as f32) - 3.5)).collect();
    let (d0, d1) = l.depth;
    let top_n = Fbm::new(l.seed as u32 + 50, 3, 70.0);
    let region = Mask::from_fn(f, move |x, y| smoothstep(crest(x) - 0.5, crest(x) + 0.5, y) * (1.0 - smoothstep(bottom(x) - 2.0, bottom(x), y)));
    // how far the mist has the range at a point
    // measured from the line the crest's small tops stand on, so the mist
    // line doesn't copy the fir tops (upside down) into the mist
    let foot_line = move |x: f32| (-4..=4).map(|k| crest(x + k as f32 * 0.8)).fold(f32::MIN, f32::max);
    let misted = move |x: f32, y: f32| {
        let d = y - foot_line(x);
        let a = d0 * (0.6 + 0.8 * top_n.get01(x, 0.5));
        let m = smoothstep(a, a + (d1 - d0), d);
        m.max(sink(x))
    };
    let col = move |x: f32, y: f32| mix(body(x), mist_col(x, y), misted(x, y), Mix::Light);
    let hd = st.body().palette(pal).color(col).angle(|_, _| 0.0).angle_jitter(0.08).length(stroke.0, stroke.1).coverage(4.0).pressure(0.45, 0.75).medium(medium).clip(true).threshold(0.3);
    c.work(&region, &hd, l.seed);
    if let Some(b) = st.blend() {
        // fuse where the mist takes the range and the mist itself, never the crest
        let soft = Mask::from_fn(f, move |x, y| region.sample(x, y) * smoothstep(0.05, 0.3, misted(x, y)) * smoothstep(crest(x) + 3.0, crest(x) + 7.0, y));
        c.work(&soft, &b.angle(|_, _| 0.0).length(40.0, 140.0), l.seed + 1);
    }
    if carry > 0.0 {
        // the sky taken from just above and laid back over the crest in
        // places, thin, in short strokes along it: the edge lost and found
        let lost = Fbm::new(l.seed as u32 + 70, 3, 60.0);
        let band = Mask::from_fn(f, move |x, y| {
            let d = y - crest(x);
            smoothstep(-1.2, -0.2, d) * (1.0 - smoothstep(1.5, 3.0, d)) * smoothstep(0.45, 0.65, lost.get01(x, 0.3)) * (1.0 - sink(x))
        });
        let above = &above;
        // a half tone between the sky and the range, not the sky itself
        let col = move |x: f32, _: f32, u: Rgb| mix(above[(x.round().clamp(0.0, 1001.0)) as usize], u, 0.65, Mix::Light);
        let hd = paint::Handling::new(Tool::filbert(4.0)).mixed(pal, 0.5).color_over(col).angle(|_, _| 0.0).angle_jitter(0.1).length(6.0, 18.0).coverage(1.4 * carry).pressure(0.35, 0.55).clip(true).threshold(0.2);
        c.work(&band, &hd, l.seed + 2);
    }
}
// ckpt: end

// ckpt: from spruces
/// One bough of a spruce: from the trunk at height `yb` out to one side.
struct Bough {
    side: f32,
    xs: f32,
    yb: f32,
    len: f32,
    droop: f32,
    up: f32,
    /// how far the twig curtain hangs below the bough (units, at the trunk)
    hang: f32,
    seed: f32,
}

impl Bough {
    /// The bough's line at t (0 trunk .. 1 tip).
    fn at(&self, t: f32) -> (f32, f32) {
        (self.xs + self.side * self.len * t, self.yb + self.droop * t.powf(1.6) - self.up * smoothstep(0.7, 1.0, t))
    }
    /// How much of the bough (with its hanging twigs) covers (x, y).
    fn cover(&self, x: f32, y: f32) -> f32 {
        let t = self.side * (x - self.xs) / self.len;
        if !(-0.02..=1.02).contains(&t) {
            return 0.0;
        }
        let t = t.clamp(0.0, 1.0);
        let (_, py) = self.at(t);
        // the curtain is ragged: twigs of different lengths side by side
        let r = |k: f32| ((k * 12.9898 + self.seed * 78.233).sin() * 43758.545).fract().abs();
        let k = (t * self.len / 1.6).floor();
        let rag = 0.45 + 0.55 * r(k);
        let above = 1.0 * (1.0 - 0.5 * t) + 0.4;
        let below = self.hang * (1.0 - 0.55 * t) * rag + 0.8;
        let d = y - py;
        let inside = smoothstep(-above - 0.5, -above + 0.5, d) * (1.0 - smoothstep(below - 0.6, below + 0.6, d));
        // the tip thins to nothing
        inside * (1.0 - smoothstep(0.9, 1.02, t))
    }
}

/// A spruce written with the brush: a narrow dark core (the trunk and the
/// inner boughs), the boughs as drooping bands with their twigs hanging
/// under them like a curtain, tiers at uneven spacing with some boughs
/// missing or short so the sky shows through; hatched down and out along
/// the boughs; then twig ends pulled past the silhouette, sky light on the
/// tops of the boughs toward the glow, and the leader.
#[allow(clippy::too_many_arguments)]
fn spruce(c: &mut paint::Canvas, st: &Style, pal: &Palette, x0: f32, base: f32, hgt: f32, lean: f32, dark: Rgb, lit: Rgb, seed: u64) {
    let f = c.frame();
    let mut rng = paint::Rng::new(seed);
    let wob = seed as f32;
    let cx = move |v: f32| x0 + lean * hgt * v + 1.0 * (v * 9.0 + wob).sin();
    let wmax = 0.185 * hgt;
    let width = move |v: f32| wmax * (1.0 - v).max(0.0).powf(0.95) * (0.8 + 0.2 * smoothstep(0.0, 0.15, v));
    // the boughs, tier by tier at uneven spacing
    let mut boughs: Vec<Bough> = vec![];
    let mut v = 0.02 + 0.03 * rng.range(0.0, 1.0);
    while v < 0.95 {
        let step = (11.0 / hgt) * rng.range(0.55, 1.5) * (1.0 - 0.5 * v);
        for side in [-1.0f32, 1.0] {
            let r = rng.range(0.0, 1.0);
            let vv = v + rng.range(-0.3, 0.3) * step;
            // gaps and short boughs mostly in the upper half
            if r < 0.1 * smoothstep(0.2, 0.6, vv) {
                continue;
            }
            let len = width(vv) * if r < 0.25 * smoothstep(0.1, 0.5, vv) { rng.range(0.35, 0.6) } else { rng.range(0.8, 1.2) };
            if len < 1.0 {
                continue;
            }
            let droop = len * rng.range(0.12, 0.6) * (0.7 + 0.5 * (1.0 - vv));
            boughs.push(Bough {
                side,
                xs: cx(vv),
                yb: base - vv * hgt,
                len,
                droop,
                up: droop * rng.range(0.1, 0.4),
                hang: (2.5 + 5.5 * (len / wmax).min(1.0)) * rng.range(0.7, 1.3),
                seed: rng.range(0.0, 100.0),
            });
        }
        v += step;
    }
    boughs.sort_by(|a, b| a.yb.total_cmp(&b.yb));
    let reach = boughs.iter().map(|b| b.droop + b.hang + 2.0).fold(0.0, f32::max);
    let bs = &boughs;
    let sil = Mask::from_fn(f, move |x, y| {
        if y < base - hgt - 4.0 || y > base + 4.0 || (x - x0).abs() > wmax * 1.3 + 6.0 {
            return 0.0;
        }
        let vv = (base - y) / hgt;
        // the core: trunk and inner boughs
        // dense near the trunk; the sky shows only toward the bough ends
        let w = (0.3 + 0.1 * (vv * 37.0 + wob).sin()) * width(vv.clamp(0.0, 1.0)) + 0.9;
        let mut a: f32 = if (0.0..0.96).contains(&vv) { smoothstep(w + 0.5, w - 0.5, (x - cx(vv)).abs()) } else { 0.0 };
        // boughs whose curtain can reach this height
        let i0 = bs.partition_point(|b| b.yb < y - reach);
        for b in &bs[i0..] {
            if b.yb > y + 3.0 {
                break;
            }
            a = a.max(b.cover(x, y));
            if a >= 1.0 {
                break;
            }
        }
        a
    })
    .roughen(seed as u32, 2.5, 0.8, 0.4);
    // 1. the body: short strokes down and out along the boughs
    let hd = st
        .hatch()
        .palette(pal)
        .color(move |x, y| {
            let v = ((base - y) / hgt).clamp(0.0, 1.0);
            // the side toward the glow a shade cooler and lighter
            mix(dark, lit, if x < cx(v) { 0.1 } else { 0.0 }, Mix::Pigment)
        })
        .angle(move |x, y| {
            let v = ((base - y) / hgt).clamp(0.0, 1.0);
            if x > cx(v) { 0.7 } else { PI - 0.7 }
        })
        .angle_jitter(0.3)
        .length(2.5, 6.0)
        .coverage(3.2)
        .clip(true)
        .threshold(0.3);
    c.work(&sil, &hd, seed + 1);
    // 2. twig ends pulled down past the curtain's edge, and sky light on
    //    the tops of the boughs
    let twig_p = pal.paint(dark, 0.15);
    let lit_p = pal.paint(lit, 0.18);
    let mut th = Held::new(Tool::round_sable(1.0), seed + 3);
    let mut lh = Held::new(Tool::round_sable(1.1), seed + 4);
    th.load(twig_p, 0.7);
    lh.load(lit_p, 0.6);
    for b in bs.iter() {
        let n = (b.len / 2.2) as usize + 1;
        if n == 0 {
            continue;
        }
        for _ in 0..n {
            let t: f32 = rng.range(0.2, 0.98);
            let (px, py) = b.at(t);
            let hang = b.hang * (1.0 - 0.55 * t) * rng.range(0.7, 1.3);
            let a = PI / 2.0 - b.side * rng.range(0.05, 0.45);
            let tip = (px + a.cos() * (hang + 1.5), py + a.sin() * (hang + 1.5));
            if th.fullness() < 0.35 {
                th.reload(twig_p, 0.7);
            }
            c.drag(&mut th, &Gesture::new(vec![(px, py + hang * 0.4), tip]).pressure(0.6, 0.15).ramps(0.05, 0.5), None);
        }
    }
    // the sky light goes on when the dark has set a little, so it sits on
    // it instead of sinking into it
    c.wait(120.0);
    for b in bs.iter() {
        let toward = b.side < 0.0;
        if (toward && rng.range(0.0, 1.0) < 0.8) || rng.range(0.0, 1.0) < 0.2 {
            let t0 = rng.range(0.2, 0.5);
            let t1 = (t0 + rng.range(0.3, 0.5)).min(0.95);
            let pts: Vec<(f32, f32)> = (0..4)
                .map(|i| {
                    let t = t0 + (t1 - t0) * i as f32 / 3.0;
                    let (x, y) = b.at(t);
                    (x, y - 0.6)
                })
                .collect();
            if lh.fullness() < 0.3 {
                lh.reload(lit_p, 0.6);
            }
            c.drag(&mut lh, &Gesture::new(pts).pressure(0.55, 0.2).ramps(0.1, 0.5), None);
        }
    }
    // 3. the leader, a little crooked, with a tuft or two
    let dark_p = pal.paint(dark, 0.12);
    let mut rh = Held::new(Tool { point: 1.0, ..Tool::rigger(1.0) }, seed + 5);
    rh.load(dark_p, 0.8);
    let top = base - hgt;
    let pts = vec![(cx(0.86), base - 0.86 * hgt), (cx(0.94) + 0.4, base - 0.94 * hgt), (cx(1.0) - 0.3, top - 1.5)];
    c.drag(&mut rh, &Gesture::new(pts).pressure(0.9, 0.2).ramps(0.05, 0.4), None);
    for k in 0..3 {
        let v = 0.9 + 0.028 * k as f32;
        let y = base - v * hgt;
        for side in [-1.0f32, 1.0] {
            let l = 1.8 + rng.range(0.0, 2.0);
            c.drag(&mut th, &Gesture::new(vec![(cx(v), y), (cx(v) + side * l, y + l * 0.55)]).pressure(0.55, 0.1).ramps(0.05, 0.5), None);
        }
    }
}
// ckpt: end

// ---------------------------------------------------------------- main

// ckpt: from summit
/// The summit's granite: angular blocks, tilted and stacked, their edges
/// barely worn, weathered on the surface. Returns the form and the ids.
fn rock_form(f: paint::Frame) -> (Form, Vec<u16>) {
    let mut form = Form::new(f);
    #[allow(clippy::too_many_arguments)]
    let block = |cx: f32, cy: f32, sx: f32, sy: f32, sz: f32, yaw: f32, pitch: f32, roll: f32, seed: u32| {
        let cc = [cx, cy, 0.0];
        let r = |k: u32| (((seed * 7 + k) as f32 * 12.9898).sin() * 43758.545).fract().abs();
        Sdf::block(cc, [sx, sy, sz], sy * 0.1)
            .turn(cc, yaw, pitch, roll)
            .cut([cx - sx * (0.2 + 0.2 * r(1)), cy - sy * 0.38, 0.0], [-0.5 + r(2), -1.0, 0.3], 10, sy * 0.03)
            .cut([cx + sx * 0.42, cy - sy * 0.2, 0.0], [1.0, -0.6 + 0.4 * r(3), 0.2], 11, sy * 0.03)
            .cut([cx - sx * 0.45, cy - sy * 0.1, 0.0], [-1.0, -0.4 * r(4), 0.3], 12, sy * 0.03)
            .rough(sy * 0.022, sy * 0.5, seed, false)
            .rough(sy * 0.01, sy * 0.12, seed + 1, true)
    };
    let ids = vec![
        // the outcrop on the knoll at the left: a base block, a tilted block
        // on it, a small one wedged beside, a slab leaning on the left
        form.add(&block(165.0, 584.0, 175.0, 52.0, 110.0, 0.25, 0.3, 0.05, 1), 0.3),
        form.add(&block(132.0, 548.0, 96.0, 40.0, 80.0, -0.2, 0.25, -0.13, 2), 0.3),
        form.add(&block(206.0, 551.0, 58.0, 30.0, 50.0, 0.4, 0.3, 0.22, 3), 0.3),
        form.add(&block(66.0, 574.0, 58.0, 62.0, 45.0, 0.1, 0.2, -0.38, 4), 0.3),
        form.add(&block(290.0, 592.0, 70.0, 24.0, 50.0, 0.5, 0.3, 0.08, 5), 0.3),
        // one by the spruces
        form.add(&block(650.0, 525.0, 96.0, 32.0, 70.0, 0.45, 0.3, -0.06, 6), 0.3),
    ];
    (form, ids)
}
// ckpt: end

// ckpt: from figure
/// A man seen from behind, standing still, a stick in his right hand,
/// looking out toward the glow: a long coat narrowing to the shoulders and
/// flaring a little at the hem, legs a little apart, a soft cap. Painted
/// dark against the mist; the side toward the glow takes a thin warm rim.
/// `at` is between his feet, `size` his height (units).
fn wanderer(c: &mut paint::Canvas, pal: &Palette, at: (f32, f32), size: f32, seed: u64) {
    let f = c.frame();
    let (ax, ay) = at;
    let cap = |u: f32, v: f32, a: (f32, f32), b: (f32, f32), r: f32| {
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let t = (((u - a.0) * dx + (v - a.1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
        let (px, py) = (a.0 + dx * t, a.1 + dy * t);
        r - ((u - px).powi(2) + (v - py).powi(2)).sqrt()
    };
    // signed "inside" distance in figure heights
    let body = move |u: f32, v: f32| -> f32 {
        let mut d: f32 = -1.0;
        // legs, a little apart
        d = d.max(cap(u, v, (-0.045, 0.0), (-0.03, 0.34), 0.034));
        d = d.max(cap(u, v, (0.05, 0.0), (0.03, 0.34), 0.034));
        // the coat
        if (0.27..0.86).contains(&v) {
            let hem = 0.27 + 0.012 * (u * 40.0).sin();
            let hw = if v < 0.72 { 0.138 - 0.05 * (v - 0.27) / 0.45 } else { 0.093 * (1.0 - ((v - 0.72) / 0.135).powi(2)).max(0.0).sqrt() };
            d = d.max((hw - u.abs()).min(v - hem));
        }
        // neck and head, turned a little toward the glow, a soft cap
        d = d.max(cap(u, v, (-0.004, 0.83), (-0.008, 0.88), 0.03));
        d = d.max(0.058 - (((u + 0.01) / 0.8).powi(2) + ((v - 0.925) / 1.1).powi(2)).sqrt());
        d = d.max(0.03 - (((u + 0.014) / 1.9).powi(2) + (v - 0.973).powi(2)).sqrt());
        // his right arm, a little out, the hand on the stick
        d = d.max(cap(u, v, (0.085, 0.77), (0.15, 0.53), 0.03));
        d
    };
    let px_u = 0.5 / size; // about a pixel at 2400 in figure heights
    let sil = Mask::from_fn(f, move |x, y| {
        let (u, v) = ((x - ax) / size, (ay - y) / size);
        if !(-0.3..=0.4).contains(&u) || !(-0.05..=1.1).contains(&v) {
            return 0.0;
        }
        smoothstep(-px_u, px_u, body(u, v))
    });
    let coat = hex("#1f2224");
    let hd = paint::Handling::new(Tool::round_sable(0.9))
        .mixed(pal, 0.12)
        .color(move |_, y| if y > ay - 0.3 * size { hex("#1b1a19") } else { coat })
        .angle(|_, _| PI / 2.0)
        .angle_jitter(0.15)
        .length(2.0, 5.0)
        .coverage(3.5)
        .pressure(0.6, 0.85)
        .clip(true)
        .threshold(0.25);
    c.work(&sil, &hd, seed);
    // the stick, from his hand to the rock
    let mut rh = Held::new(Tool { point: 0.6, ..Tool::rigger(0.45) }, seed + 1);
    rh.load(pal.paint(hex("#221d19"), 0.1), 0.8);
    let p = |u: f32, v: f32| (ax + u * size, ay - v * size);
    c.drag(&mut rh, &Gesture::new(vec![p(0.158, 0.6), p(0.19, 0.3), p(0.22, 0.0)]).pressure(0.7, 0.6).ramps(0.05, 0.1), None);
    // the rim toward the glow: the left edge of his coat, shoulder and head
    let rim = sil.rim(0.45, 0.2).mul_fn(move |x, y| {
        let (u, v) = ((x - ax) / size, (ay - y) / size);
        if u < 0.0 && v > 0.3 { 1.0 } else { 0.0 }
    });
    let hd = paint::Handling::new(Tool::round_sable(0.5))
        .mixed(pal, 0.15)
        .color(|_, _| hex("#6f6258"))
        .angle(|_, _| PI / 2.0)
        .angle_jitter(0.1)
        .length(1.5, 4.0)
        .coverage(1.5)
        .pressure(0.4, 0.7)
        .clip(true)
        .threshold(0.3);
    c.work(&rim, &hd, seed + 2);
}
// ckpt: end

fn main() {
    let o = Run::new("r14_p1");
    let mut st = Style::friedrich();
    // a Dresden ground: warm lower layers, then a patchy whitish top of lead
    // white in oil [KÖR p.284], so the pale sky keeps its coolness
    st.ground.push(Ground { color: hex("#d4c9b4"), hiding: 0.6, um: 30.0, stiff: 0.4, apply: Apply::Brush });
    let greens = Palette::friedrich_1820_greens();
    let mut rng = paint::Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();

    let bands = Fbm::new(11, 4, 260.0);
    let cn = Fbm::new(12, 4, 120.0);

    let sky_pal = st.palette.only(&["lead white", "cobalt blue", "pale smalt", "yellow ochre", "vermilion", "chrome yellow", "raw umber"]);
    // the sky is carried down past where the ranges will stand
    let sky = Mask::from_fn(f, move |_, y| 1.0 - smoothstep(HORIZON + 26.0, HORIZON + 30.0, y));

    if o.stage("sky", &mut c, &mut rng) {
        let col = move |x: f32, y: f32| sky_with_clouds(&bands, &cn, x, y.min(HORIZON + 2.0));
        // lay-in: long, nearly level strokes of thin paint, the clouds laid
        // in with it so they sit in the wet sky
        let hd = st.broad().palette(&sky_pal).color(col).angle(|_, _| 0.0).angle_jitter(0.04).length(60.0, 180.0).coverage(4.0).clip(true).threshold(0.3);
        c.work(&sky, &hd, 101);
        if let Some(b) = st.blend() {
            c.work(&sky, &b.angle(|_, _| 0.0), 103);
        }
        // stipple into the wet paint: breaks the strokes, keeps the tones
        let sp = Stipple::new(Tool::stippler(2.6)).mixed(&sky_pal, 0.45).color_over(move |x, y, u| mix(u, col(x, y), 0.3, Mix::Light)).coverage(|_, _| 1.4).pressure(0.45, 0.8).dips(18, 0.4, 0.5).cluster(0.3, None).clip(true);
        c.stipple(&sky, &sp, 104);
        c.dry();
        // dry: a finer stipple barely lighter than the field, denser toward the glow
        let sp = Stipple::new(Tool::stippler(1.6))
            .mixed(&sky_pal, 0.5)
            .color_over(|_, _, u| paint::shift(u, 0.018, 0.0, 0.004))
            .coverage(move |x, y| {
                let dx = (x - GLOW_X) / 380.0;
                1.8 * smoothstep(40.0, HORIZON, y) * (0.35 + 0.65 * (-(dx * dx)).exp())
            })
            .pressure(0.4, 0.75)
            .dips(22, 0.35, 0.6)
            .clip(true);
        c.stipple(&sky, &sp, 105);
        c.dry();
    }

    // ------------------------------------------------ the far ranges
    let far_pal = st.palette.only(&["lead white", "cobalt blue", "pale smalt", "red earth", "raw umber", "yellow ochre", "vermilion"]);
    let rn = Fbm::new(13, 4, 90.0);
    let t1 = f.per_column(move |x| far1(&rn, x));
    let t2 = f.per_column(move |x| far2(&rn, x));
    let t3 = f.per_column(move |x| far3(&rn, x));
    let rl = f.per_column(move |x| ridge_l(&rn, x));
    let rr = f.per_column(move |x| ridge_r(&rn, x));
    let fg = f.per_column(move |x| fg_top(&rn, x));
    let glowk = |x: f32| (-((x - GLOW_X) / 300.0).powi(2)).exp();
    let b1 = move |x: f32| mix(hex("#b5a2ad"), hex("#cdae9f"), glowk(x), Mix::Light);
    let b2 = move |x: f32| mix(hex("#958ca2"), hex("#a8979c"), glowk(x), Mix::Light);
    let b3 = move |x: f32| mix(hex("#6f768c"), hex("#817b8b"), glowk(x), Mix::Light);
    let s1 = |x: f32| 0.85 * smoothstep(860.0, 1000.0, x);
    let s2 = |x: f32| smoothstep(500.0, 700.0, x);
    let s3 = |x: f32| 1.0 - smoothstep(400.0, 580.0, x);
    let bot1 = move |x: f32| t2(x).max(t3(x).min(HORIZON + 40.0)).max(t1(x) + 20.0) + 10.0;
    let bot2 = move |x: f32| t3(x).max(t2(x) + 30.0) + 10.0;
    // the nearer layers are carried down to the summit: an edge buried
    // under the pale mist of a nearer range shows through it
    let bot3 = move |x: f32| fg(x) + 10.0;
    let tg = f.per_column(move |x| ghost(&rn, x));
    let bg = move |x: f32| mix(mist_col(x, ghost(&rn, x)), hex("#7f8292"), 0.4, Mix::Light);
    let sg = |x: f32| 1.0 - smoothstep(380.0, 470.0, x) + smoothstep(760.0, 840.0, x);
    let botg = move |x: f32| fg(x) + 10.0;
    if o.stage("far", &mut c, &mut rng) {
        // each range a step darker and cooler than the one behind it, its
        // foot lost in a warmer mist mixed into the range's own paint
        let l1 = Layer { crest: t1, body: &b1, depth: (6.0, 17.0), sink: &s1, bottom: &bot1, seed: 201 };
        paint_layer(&mut c, &st, &far_pal, &l1, 0.35, (14.0, 44.0), 1.0);
        c.dry();
        let l2 = Layer { crest: t2, body: &b2, depth: (15.0, 42.0), sink: &s2, bottom: &bot2, seed: 211 };
        paint_layer(&mut c, &st, &far_pal, &l2, 0.3, (14.0, 44.0), 0.7);
        c.dry();
        let l3 = Layer { crest: t3, body: &b3, depth: (18.0, 52.0), sink: &s3, bottom: &bot3, seed: 221 };
        paint_layer(&mut c, &st, &far_pal, &l3, 0.25, (12.0, 40.0), 0.6);
        c.dry();
        // a ridge half drowned in the mist sea
        let lg = Layer { crest: tg, body: &bg, depth: (3.0, 16.0), sink: &sg, bottom: &botg, seed: 231 };
        paint_layer(&mut c, &st, &far_pal, &lg, 0.3, (10.0, 36.0), 0.0);
        c.dry();
    }

    // the wooded ridges rising out of the mist, and the mist in front of them
    let wood_pal = greens.only(&["lead white", "Prussian blue", "cobalt blue", "raw umber", "bone black", "yellow ochre", "green earth", "pale smalt"]);
    let br = |_: f32| hex("#3f4753");
    let bl = |_: f32| hex("#39414b");
    let sr = |x: f32| 1.0 - smoothstep(470.0, 620.0, x);
    let sl = |x: f32| smoothstep(300.0, 470.0, x);
    let botr = move |x: f32| fg(x) + 10.0;
    let botl = move |x: f32| fg(x) + 10.0;
    if o.stage("ridges", &mut c, &mut rng) {
        let lr = Layer { crest: rr, body: &br, depth: (16.0, 55.0), sink: &sr, bottom: &botr, seed: 401 };
        paint_layer(&mut c, &st, &wood_pal, &lr, 0.2, (10.0, 36.0), 0.0);
        c.dry();
        let ll = Layer { crest: rl, body: &bl, depth: (20.0, 70.0), sink: &sl, bottom: &botl, seed: 411 };
        paint_layer(&mut c, &st, &wood_pal, &ll, 0.2, (10.0, 36.0), 0.0);
        c.dry();
    }

    // ------------------------------------------------ the near summit
    // granite blocks on it, lit from the glow behind and the sky above
    let fg_mask = Mask::from_fn(f, move |x, y| smoothstep(fg(x) - 0.6, fg(x) + 0.6, y));
    if o.stage("summit", &mut c, &mut rng) {
        let (mut form, ids) = rock_form(f);
        form.light(Light::new((-1.0, -0.8), -0.25).ambient(0.4).penumbra(0.1));
        let rocks = form.silhouette(&ids, |_| 0.5);
        let ground = fg_mask.clone().union(&rocks);
        let form = &form;
        let heath = Fbm::new(21, 4, 60.0);
        // one dark for ground and the rocks' shadow sides, laid together
        let earth = move |x: f32, y: f32| -> Rgb {
            let v = heath.get01(x * 1.2, y * 2.0);
            let a = mix(hex("#1e1f1c"), hex("#2b2922"), v, Mix::Pigment);
            // the upper slope turns to the sky and takes a little of it
            let e = 1.0 - smoothstep(fg(x) + 2.0, fg(x) + 60.0, y);
            mix(a, hex("#4a4c52"), 0.35 * e * e.sqrt(), Mix::Pigment)
        };
        let col = move |x: f32, y: f32| -> Rgb {
            match form.sample(x, y) {
                Some(s) => {
                    let v = s.shade.value.clamp(0.0, 1.0);
                    mix(hex("#252629"), hex("#35363b"), smoothstep(0.2, 0.8, v), Mix::Pigment)
                }
                None => earth(x, y),
            }
        };
        let hd = st.body().palette(&wood_pal).color(col).angle(move |x, y| if form.sample(x, y).is_some() { form.fall(x, y) } else { -0.12 }).angle_jitter(0.3).length(10.0, 34.0).coverage(3.4).pressure(0.55, 0.85).clip(true).threshold(0.3);
        c.work(&ground, &hd, 501);
        // the tops take the sky: a little lighter, cool, laid into the wet
        // dark in broken strokes across the plane
        let lit = form.mask(|s| smoothstep(0.45, 0.85, s.shade.value) * smoothstep(0.15, 0.6, -s.n[1])).mul(&rocks);
        let lcol = move |x: f32, y: f32| -> Rgb {
            match form.sample(x, y) {
                Some(s) => mix(hex("#35363b"), hex("#494b51"), smoothstep(0.45, 1.0, s.shade.value.clamp(0.0, 1.0)), Mix::Pigment),
                None => hex("#35363b"),
            }
        };
        let hd = st.body().palette(&wood_pal).color(lcol).angle(move |x, y| form.across(x, y)).angle_jitter(0.25).length(6.0, 18.0).coverage(1.6).broken(0.3).pressure(0.4, 0.7).clip(true).threshold(0.3);
        c.work(&lit, &hd, 502);
        // the edges turned toward the glow catch it: a thin warm rim, lost
        // and found along the outline
        let breaks = Fbm::new(23, 3, 25.0);
        let rim = rocks
            .rim(1.2, 0.5)
            .mul(&form.mask(|s| smoothstep(0.45, 0.9, -0.75 * s.n[0] - 0.65 * s.n[1])))
            .mul_fn(move |x, y| smoothstep(0.5, 0.65, breaks.get01(x, y)));
        let hd = st.detail().palette(&wood_pal).color(|_, _| hex("#5a524c")).angle(move |x, y| form.across(x, y)).angle_jitter(0.15).length(3.0, 9.0).coverage(1.6).pressure(0.4, 0.7).clip(true).threshold(0.3);
        c.work(&rim, &hd, 503);
        // a crack running off at an angle across the big block's face,
        // lost halfway, with a light lip above it
        let mut ch = Held::new(Tool { point: 1.0, ..Tool::round_sable(0.9) }, 504);
        ch.load(wood_pal.paint(hex("#18181a"), 0.1), 0.7);
        c.drag(&mut ch, &Gesture::new(vec![(176.0, 566.5), (180.0, 574.0), (179.0, 580.0), (186.0, 590.0)]).pressure(0.7, 0.05).ramps(0.05, 0.6), Some(&rocks));
        c.drag(&mut ch, &Gesture::new(vec![(120.0, 536.0), (127.0, 541.0), (131.0, 549.0)]).pressure(0.6, 0.05).ramps(0.05, 0.6), Some(&rocks));
        let mut lp = Held::new(Tool { point: 1.0, ..Tool::round_sable(0.7) }, 505);
        lp.load(wood_pal.paint(hex("#4b4c52"), 0.15), 0.5);
        c.drag(&mut lp, &Gesture::new(vec![(174.8, 566.8), (178.6, 573.6), (177.8, 578.0)]).pressure(0.5, 0.05).ramps(0.1, 0.6), Some(&rocks));
        // the heath: after an hour, when the dark has begun to set, short
        // upright strokes side by side in close darks (olive, the rust of
        // heather, a cool gray-green), patch by patch, larger nearer
        c.wait(60.0);
        let patches = Fbm::new(24, 3, 90.0);
        let hcol = move |x: f32, y: f32| -> Rgb {
            let p = patches.get01(x, y * 1.6);
            let q = patches.get01(x * 2.3 + 40.0, y * 3.0);
            let a = if p < 0.45 { mix(hex("#22221b"), hex("#2b201a"), smoothstep(0.2, 0.45, p), Mix::Pigment) } else { mix(hex("#2b201a"), hex("#222724"), smoothstep(0.5, 0.75, p), Mix::Pigment) };
            let e = 1.0 - smoothstep(fg(x) + 2.0, fg(x) + 60.0, y);
            mix(mix(a, hex("#302f26"), 0.35 * smoothstep(0.6, 0.9, q), Mix::Pigment), hex("#424347"), 0.3 * e * e.sqrt(), Mix::Pigment)
        };
        let open = ground.clone().subtract(&rocks.dilate(1.0));
        let far_h = open.clone().mul_fn(move |x, y| 1.0 - smoothstep(fg(x) + 40.0, fg(x) + 70.0, y));
        let near_h = open.mul_fn(move |x, y| smoothstep(fg(x) + 40.0, fg(x) + 70.0, y));
        let hd = st.hatch().palette(&wood_pal).color(hcol).angle(|_, _| -PI / 2.0).angle_jitter(0.35).length(2.5, 5.0).coverage(1.3).clump(0.7).clip(true).threshold(0.3);
        c.work(&far_h, &hd, 506);
        let hd = st.hatch().palette(&wood_pal).color(hcol).angle(|_, _| -PI / 2.0).angle_jitter(0.4).length(4.0, 10.0).coverage(1.3).clump(0.7).clip(true).threshold(0.3);
        c.work(&near_h, &hd, 507);
        c.dry();
    }

    // ------------------------------------------------ the spruces
    if o.stage("spruces", &mut c, &mut rng) {
        let dark = hex("#1a1f1c");
        let lit = hex("#7a8482");
        // back to front: the farther (right) ones first
        for (i, &(x, h, lean)) in [(962.0f32, 146.0f32, -0.02f32), (884.0, 264.0, 0.0), (806.0, 198.0, 0.02), (770.0, 322.0, -0.01), (704.0, 112.0, 0.02)].iter().enumerate() {
            let base = fg_top(&rn, x) + 16.0;
            spruce(&mut c, &st, &wood_pal, x, base, h, lean, dark, lit, 600 + 10 * i as u64);
        }
        c.dry();
    }

    // ------------------------------------------------ the man on the rocks
    if o.stage("figure", &mut c, &mut rng) {
        // he stands on the top of the tilted block, facing the glow
        let (form, _) = rock_form(f);
        let fx = 136.0;
        let mut fy = 470.0;
        while fy < 600.0 && form.sample(fx, fy).is_none() {
            fy += 0.25;
        }
        drop(form);
        eprintln!("figure feet at y {fy:.1}");
        wanderer(&mut c, &wood_pal, (fx, fy + 1.5), 33.0, 700);
        c.dry();
    }

    // ------------------------------------------------ heath and grass
    if o.stage("heath", &mut c, &mut rng) {
        let mut rg = paint::Rng::new(77);
        let clump = Fbm::new(31, 3, 40.0);
        let mut held = Held::new(Tool { point: 1.0, ..Tool::round_sable(0.9) }, 78);
        let deep = wood_pal.paint(hex("#1c1b16"), 0.15);
        let dry = wood_pal.paint(hex("#3a372b"), 0.15);
        // on the skyline the blades take the light of the mist behind
        let rim = wood_pal.paint(hex("#77736a"), 0.15);
        let mut n = 0;
        let tip = wood_pal.paint(hex("#5d5642"), 0.15);
        for _ in 0..3400 {
            let x = rg.range(-10.0, 1010.0);
            let top = fg_top(&rn, x);
            let near = rg.range(0.0, 1.0).powf(1.3);
            let y = top + 3.0 + near * (H + 5.0 - top);
            let cl = clump.get01(x, y * 1.5);
            if cl < 0.6 + 0.06 * (1.0 - near) {
                continue;
            }
            // a tuft: blades from one base, fanning, longer nearer
            let k = 3 + (rg.range(0.0, 1.0) * (3.0 + 7.0 * near)) as usize;
            let size = (3.0 + 16.0 * near * near) * rg.range(0.6, 1.3);
            let lean0 = rg.normal() * 0.2 + 0.1;
            for _ in 0..k {
                let bx = x + rg.normal() * size * 0.08;
                let len = size * rg.range(0.45, 1.1);
                let lean = lean0 + rg.normal() * 0.35;
                let bend = rg.normal() * 0.12;
                let r = rg.range(0.0, 1.0);
                // dark blades; a few catch the sky, more of them high up
                let lightish = 0.04 + 0.12 * (1.0 - near);
                let p = if y < top + 5.0 && r < 0.5 { rim } else if r < lightish { tip } else if r < lightish + 0.2 { dry } else { deep };
                held.reload(p, 0.5);
                let pts = vec![(bx, y), (bx + lean * len * 0.3, y - len * 0.5), (bx + (lean + bend) * len, y - len * (1.0 - 0.15 * bend.abs()))];
                c.drag(&mut held, &Gesture::new(pts).pressure(0.6 + 0.2 * near, 0.05).ramps(0.05, 0.6), None);
                n += 1;
            }
        }
        // tufts on the skyline, dark against the mist
        let mut x = 228.0;
        let mut th = Held::new(Tool { point: 1.0, ..Tool::round_sable(0.8) }, 79);
        let tuft = wood_pal.paint(hex("#26251f"), 0.12);
        while x < 720.0 {
            let top = fg_top(&rn, x);
            let k = 2 + (rg.range(0.0, 1.0) * 6.0) as usize;
            for _ in 0..k {
                let bx = x + rg.normal() * 2.0;
                let by = fg_top(&rn, bx) + rg.range(0.5, 2.5);
                let len = rg.range(3.0, 9.0) * if rg.range(0.0, 1.0) < 0.15 { 1.6 } else { 1.0 };
                let lean = rg.normal() * 0.35 + 0.1;
                th.reload(tuft, 0.5);
                c.drag(&mut th, &Gesture::new(vec![(bx, by), (bx + lean * len * 0.35, by - len * 0.5), (bx + lean * len, by - len)]).pressure(0.55, 0.05).ramps(0.05, 0.6), None);
                n += 1;
            }
            let _ = top;
            x += rg.range(10.0, 45.0);
        }
        eprintln!("grass: {n} blades");
        c.dry();
    }

    // ------------------------------------------------ the veil
    // Friedrich's advice to Carus: a glaze over the whole picture, growing
    // darker toward its edges [MET p.35]; here brushed thin and cool,
    // heavier the farther from the glow, and fused
    if o.stage("veil", &mut c, &mut rng) {
        c.dry();
        let veil_pal = st.palette.only(&["cobalt blue", "bone black", "raw umber"]);
        let w = move |x: f32, y: f32| {
            let dx = (x - GLOW_X) / 640.0;
            let dy = (y - HORIZON) / 430.0;
            smoothstep(0.4, 1.3, (dx * dx + dy * dy).sqrt())
        };
        // a film of cobalt and black in much medium: it levels and pools in
        // the hollows of the surface; its depth varies unevenly, as a
        // brushed glaze does, and no badger goes over it (that pushed the
        // fluid glaze into worm-like rims)
        let glz = veil_pal.mix(hex("#2b303d")).paint(0.9).pigment();
        let uneven = Fbm::new(81, 3, 160.0);
        c.glaze(&glz, None, move |x, y| 0.9 * w(x, y) * (0.75 + 0.5 * uneven.get01(x, y)));
        c.dry();
    }

    // ------------------------------------------------ the old moon
    // a thin waning crescent over the place where the sun will rise, its
    // lit limb turned down toward the sun under the horizon; laid last,
    // in short touches along the limb so the horns come to points
    if o.stage("moon", &mut c, &mut rng) {
        let (mx, my, r) = (452.0f32, 118.0f32, 6.2f32);
        let (sx, sy) = (GLOW_X - mx, HORIZON + 40.0 - my);
        let sl = (sx * sx + sy * sy).sqrt();
        let (ux, uy) = (sx / sl, sy / sl);
        let dd = 0.24 * r;
        let moon = Mask::from_fn(f, move |x, y| {
            let d0 = ((x - mx).powi(2) + (y - my).powi(2)).sqrt();
            let d1 = ((x - mx + ux * dd).powi(2) + (y - my + uy * dd).powi(2)).sqrt();
            smoothstep(r + 0.25, r - 0.25, d0) * smoothstep(r * 1.03 - 0.25, r * 1.03 + 0.25, d1)
        });
        let hd = paint::Handling::new(Tool { point: 0.8, ..Tool::round_sable(0.9) })
            .mixed(&sky_pal, 0.1)
            .color(|_, _| hex("#e8dfca"))
            .angle(move |x, y| (y - my).atan2(x - mx) + PI / 2.0)
            .angle_jitter(0.1)
            .length(2.0, 5.0)
            .coverage(3.0)
            .pressure(0.55, 0.8)
            .clip(true)
            .threshold(0.3);
        c.work(&moon, &hd, 901);
        c.dry();
    }

    let fin = Finish { cracks: Some(paint::Cracks { width_um: Some(12.0), dirt: 0.2, grime: 0.5, depth_um: 7.0, cupping_um: 6.0, corners: false, ..paint::Cracks::aged(0) }), ..Finish::aged(st.relief) };
    o.finish(&mut c, &mut rng, &fin);
}
