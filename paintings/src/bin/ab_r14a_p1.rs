//! r14_p1: a ploughed field rising to a crest on an October morning, the
//! towers of a small Baltic town standing out of the mist in the lowland
//! beyond, a cart track running over the dip in the rise toward them.
//!
//!   cargo paint r14_p1 -- --full --width 2400
//!   cargo paint r14_p1 -- --full --width 2400 --crop x0,y0,x1,y1

use paint::atmos::{Cloud, Clouds, Sky, SkyField};
use paint::color::{Mix, luminance, mix};
use paint::scene::{Sun, World};
use paint::{Canvas, Fbm, Gesture, Palette, Sdf, Handling, Held, Mask, Rgb, Rng, Shape, Stipple, Style, Tool, hex, smoothstep};
use paintings::rocks::{self, Stone};
use paintings::run::{Finish, Run};

const ASPECT: f32 = 1.4;
const H: f32 = 1000.0 / ASPECT;
/// Eye level on the canvas.
const HZ: f32 = 452.0;
const EYE: f32 = 1.6;
/// The furrows' heading on the ground (radians from straight ahead, + to
/// the right): away and to the left.
const PHI: f32 = -0.12;
/// Ridge to ridge (m).
const FURROW: f32 = 0.36;
/// Distance of the town (m).
const TOWN_Z: f32 = 3500.0;

/// Where the rise crests (m away) and how high (m), along X: low in a
/// saddle left of center where the track goes over, rising to a shoulder on
/// the right.
fn crest_z(x: f32) -> f32 {
    60.0 - 10.0 * smoothstep(-2.0, 22.0, x) + 3.0 * (x * 0.09 + 1.0).sin()
}
fn crest_h(x: f32) -> f32 {
    -0.14 + 1.5 * smoothstep(-6.0, 20.0, x) + 0.5 * (1.0 - smoothstep(-28.0, -14.0, x)) + 0.05 * (x * 0.21).sin() + 0.04 * (x * 0.53 + 2.0).sin()
}
/// The ground: rising from the painter's feet to a rounded crest, then
/// falling away to the flat lowland 17 m below.
fn ground(x: f32, z: f32) -> f32 {
    let (zc, hc) = (crest_z(x), crest_h(x));
    let lumps = 0.05 * (x * 0.31 + 1.0).sin() * (z * 0.23).sin() + 0.03 * (x * 0.9).sin() * (z * 0.7 + 2.0).sin();
    if z <= zc {
        let t = (z / zc).clamp(0.0, 1.0);
        hc * (1.0 - (1.0 - t) * (1.0 - t)) + lumps * t
    } else {
        let s = ((z - zc) / 340.0).clamp(0.0, 1.0);
        hc - 17.0 * s * s * (3.0 - 2.0 * s)
    }
}

fn world() -> World {
    World::new([0.0, 0.0, 1000.0, H], HZ, EYE).fov(1000.0, 45.0).ground(ground).sun(Sun::deg(-58.0, 5.0)).visibility(9000.0)
}

/// The top of the field on the canvas (its crest seen against what lies
/// beyond), per column.
fn crest_y(w: &World, x: f32) -> f32 {
    let mut best = f32::MAX;
    let mut z = 4.0;
    while z < 420.0 {
        let wx = (x - w.cx) * z / w.focal;
        let y = HZ + w.focal * (EYE - ground(wx, z)) / z;
        best = best.min(y);
        z += 0.25 + z * 0.004;
    }
    best
}

/// The far country's skyline above the horizon: open flat land with low
/// woods in unequal stretches (units above HZ).
fn far_rise(x: f32, n: &Fbm) -> f32 {
    let woods = [(20.0, 118.0, 3.4), (150.0, 196.0, 2.0), (236.0, 300.0, 4.2), (520.0, 585.0, 2.4), (640.0, 700.0, 3.0), (760.0, 1000.0, 3.8)];
    let mut r: f32 = 0.5 + 0.4 * n.get(x * 0.2, 0.0);
    for (a, b, h) in woods {
        let k = smoothstep(a, a + 10.0, x) * (1.0 - smoothstep(b - 8.0, b, x));
        let crown = h * (0.7 + 0.4 * n.get01(x * 1.9, 3.0) + 0.3 * n.get(x * 6.0, 7.0));
        r = r.max(k * crown);
    }
    r
}

/// The furrows at a ground point: (across-coordinate in furrows, the local
/// heading on the ground).
fn furrow_u(x: f32, z: f32) -> f32 {
    // the plough bends a little over the rise
    let bend = 1.6 * (z * 0.05 + 0.4).sin() + 0.5 * (x * 0.04).sin();
    (x * PHI.cos() - z * PHI.sin() + bend) / FURROW
}
fn furrow_head(z: f32) -> f32 {
    PHI + 0.08 * (z * 0.05 + 0.4).cos()
}
/// The ground X of furrow `u` (in furrows) at depth `z`.
fn furrow_x(u: f32, z: f32) -> f32 {
    let mut x = (u * FURROW + z * PHI.sin()) / PHI.cos();
    for _ in 0..4 {
        x -= (furrow_u(x, z) - u) * FURROW / PHI.cos();
    }
    x
}

/// The cart track: its middle (X m) at depth z, from the right edge over
/// the saddle.
fn track_x(z: f32) -> f32 {
    5.6 - 0.24 * z - 0.0008 * (z - 8.0) * (z - 8.0) + 0.25 * (z * 0.15).sin()
}
const TRACK_W: f32 = 2.4;

/// Town silhouettes (canvas units): towers, naves, roofs.
fn town_shapes() -> Vec<Vec<(f32, f32)>> {
    let b = HZ + 9.0; // below any mist top: the base is lost anyway
    let mut v = vec![];
    // St. Jacobi: a slim tower with a tall plain spire
    v.push(vec![(254.6, b), (254.6, 437.0), (254.9, 436.0), (256.3, 420.5), (257.7, 436.0), (258.0, 437.0), (258.0, b)]);
    v.push(vec![(258.0, b), (258.0, 444.5), (262.0, 440.5), (277.0, 440.8), (280.0, 444.5), (280.0, b)]);
    // St. Nikolai: the tall tower, octagonal stages, two bulbs, lantern, needle
    v.push(vec![
        (307.0, b),
        (307.0, 427.0),
        (307.6, 426.5),
        (307.6, 420.0),
        (306.9, 418.6),
        (307.4, 416.2),
        (308.1, 415.0),
        (308.4, 412.0),
        (307.8, 410.6),
        (308.4, 408.8),
        (309.1, 407.8),
        (309.2, 404.5),
        (309.6, 403.2),
        (309.8, 396.0),
        (310.0, 403.2),
        (310.4, 404.5),
        (310.5, 407.8),
        (311.2, 408.8),
        (311.8, 410.6),
        (311.2, 412.0),
        (311.5, 415.0),
        (312.2, 416.2),
        (312.7, 418.6),
        (312.0, 420.0),
        (312.0, 426.5),
        (312.6, 427.0),
        (312.6, b),
    ]);
    v.push(vec![(312.6, b), (312.6, 441.0), (316.0, 436.8), (336.0, 437.2), (339.5, 441.5), (339.5, b)]);
    // St. Marien: the broad squat tower with its low cap
    v.push(vec![(348.0, b), (348.0, 432.5), (349.2, 431.8), (351.4, 428.8), (352.0, 426.6), (352.6, 428.8), (354.8, 431.8), (356.0, 432.5), (356.0, b)]);
    v.push(vec![(356.0, b), (356.0, 442.0), (359.5, 438.6), (375.0, 439.0), (377.5, 442.5), (377.5, b)]);
    // the roofs between, gables and a low wall of houses
    let mut roofs = vec![(223.0, b), (223.0, 449.5)];
    let mut x = 223.0;
    let mut r = Rng::new(77);
    while x < 397.0 {
        let wd = r.range(4.0, 9.0);
        let h = r.range(2.6, 4.8) * (1.0 - 0.35 * smoothstep(375.0, 397.0, x) - 0.35 * (1.0 - smoothstep(223.0, 239.0, x)));
        roofs.push((x + 0.6, b - 9.0 - h + 1.2));
        roofs.push((x + wd * 0.5, b - 9.0 - h - r.range(0.0, 1.6)));
        roofs.push((x + wd - 0.6, b - 9.0 - h + 1.2));
        x += wd;
    }
    roofs.push((397.0, b));
    v.push(roofs);
    // a little nearer than first drawn: the towers want more weight
    let k = 1.28;
    let v: Vec<Vec<(f32, f32)>> = v.into_iter().map(|p| p.into_iter().map(|(x, y)| (310.0 + (x - 310.0) * k, 458.0 - (458.0 - y) * k)).collect()).collect();
    let mut v = v;
    // a village far off on the right, only its church
    v.push(vec![(868.0, HZ + 2.0), (868.0, 447.6), (868.6, 447.0), (869.3, 440.8), (870.0, 447.0), (870.6, 447.6), (870.6, HZ + 2.0)]);
    v.push(vec![(870.6, HZ + 2.0), (870.6, 449.2), (871.6, 448.2), (876.0, 448.4), (876.8, 449.4), (876.8, HZ + 2.0)]);
    // a windmill out on the flat to the right
    v.push(vec![(446.6, HZ + 6.5), (447.1, 452.0), (447.3, 449.6), (449.9, 449.6), (450.1, 452.0), (450.6, HZ + 6.5)]);
    v
}
/// The windmill's sails: four thin arms from the hub.
fn mill_sails() -> Vec<[(f32, f32); 2]> {
    let hub = (448.6, 449.4);
    [0.35f32, 1.92, 3.49, 5.06].iter().map(|&a| [hub, (hub.0 + 6.2 * a.cos(), hub.1 - 6.2 * a.sin())]).collect()
}

/// The heap of field stones gathered at the track side: (X, Z, half
/// width, height, half depth) in m.
const STONES: [(f32, f32, f32, f32, f32); 9] = [
    (4.2, 13.8, 0.52, 0.5, 0.44),
    (5.0, 14.3, 0.4, 0.34, 0.34),
    (3.55, 14.5, 0.36, 0.28, 0.3),
    (4.55, 13.1, 0.3, 0.22, 0.28),
    (5.65, 13.6, 0.28, 0.2, 0.25),
    (4.3, 15.1, 0.42, 0.36, 0.34),
    (3.0, 13.7, 0.2, 0.13, 0.18),
    (6.1, 14.6, 0.2, 0.14, 0.17),
    (-1.8, 8.3, 0.3, 0.15, 0.26),
];
/// The low sun's shadow of the stones on the ground (0..1): long, thrown
/// toward the painter and to the right.
fn stone_shadow(gx: f32, gz: f32) -> f32 {
    let (dx, dz) = (0.848f32, -0.529f32);
    let mut v: f32 = 0.0;
    for &(sx, sz, rx, h, _) in &STONES {
        let (px, pz) = (gx - sx, gz - sz);
        let t = px * dx + pz * dz;
        let lat = (px * dz - pz * dx).abs();
        let len = h * 11.0;
        let half = rx * (1.0 - 0.45 * (t / len).clamp(0.0, 1.0));
        let a = smoothstep(-rx * 0.6, 0.0, t) * (1.0 - smoothstep(len * 0.75, len, t)) * (1.0 - smoothstep(half * 0.8, half * 1.25, lat));
        v = v.max(a);
    }
    v
}

/// A crow on the ground at a canvas point, `s` units per meter, facing
/// left or right, head down or up.
fn crow(c: &mut Canvas, pal: &Palette, at: (f32, f32), s: f32, facing: f32, pecking: bool, seed: u64) {
    let mut r = Rng::new(seed);
    let p = |u: f32, v: f32| (at.0 + facing * u * s, at.1 - v * s);
    let black = pal.paint(hex("#18181c"), 0.12);
    let sheen = pal.paint(hex("#2c2e35"), 0.12);
    let mut b = Held::new(Tool { point: 1.0, ..Tool::round_sable((0.13 * s).max(0.6)) }, seed);
    let tilt = if pecking { -0.05 } else { 0.06 };
    // the body: from the tail's tip to the breast, the brush pressed as
    // the body fills out, then a second pull for the back's curve
    for (dv, p0, p1) in [(0.0f32, 0.15f32, 0.95f32), (0.025, 0.1, 0.7)] {
        b.reload(black, 1.0);
        let g = Gesture::new(vec![p(-0.23, 0.125 + dv), p(-0.1, 0.14 + dv + tilt * 0.3), p(0.02, 0.15 + dv + tilt * 0.7), p(0.09, 0.14 + dv + tilt)]).pressure(p0, p1).ramps(0.05, 0.25);
        c.drag(&mut b, &g, None);
    }
    let _ = r.range(0.0, 1.0);
    // head and beak
    let (hx, hy) = if pecking { (0.2, 0.06) } else { (0.17, 0.23) };
    b.reload(black, 1.0);
    c.drag(&mut b, &Gesture::new(vec![p(0.06, 0.15 + tilt), p(hx - 0.02, hy + 0.01)]).pressure(0.8, 0.65), None);
    let mut t = Held::new(Tool { point: 1.0, ..Tool::round_sable((0.05 * s).max(0.45)) }, seed + 1);
    t.reload(black, 1.0);
    let (bx, by) = if pecking { (hx + 0.05, hy - 0.06) } else { (hx + 0.09, hy - 0.02) };
    c.drag(&mut t, &Gesture::new(vec![p(hx, hy), p(bx, by)]).pressure(0.8, 0.05), None);
    // legs
    for lx in [-0.02f32, 0.04] {
        t.reload(black, 0.8);
        c.drag(&mut t, &Gesture::new(vec![p(lx, 0.1), p(lx + 0.01, 0.0)]).pressure(0.3, 0.2), None);
    }
    // the sky's light along the back
    let mut sh = Held::new(Tool::round_sable((0.025 * s).max(0.4)), seed + 2);
    sh.reload(sheen, 0.8);
    c.drag(&mut sh, &Gesture::new(vec![p(-0.16, 0.19), p(0.02, 0.2 + tilt * 0.5)]).pressure(0.3, 0.5), None);
}

fn main() {
    let o = Run::new("r14_p1");
    let st = Style::friedrich();
    let pal = &st.palette;
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let w = world();
    let w = &w;
    let sun = w.sun;

    // ---------------------------------------------------------------- air
    let sky = Sky::new(sun).haze(3.4).uneven(0.5, 25_000.0, 11).layer(600.0, 400.0, 1.4, 0.7, 5);
    let sf = SkyField::new(sky, w, 6.0);
    let bal = sf.sky.sunlight();
    let sf = sf.balance(bal, 0.3).exposure(1.0);
    let clouds = Clouds::new(vec![
        // a high thin deck in long level streaks
        Cloud::stratus(5_500.0, 500.0, 0.3, 12).density(0.005).soft(0.6).wind(0.12, 8.0).reach(4_000.0, 60_000.0),
        // a low grey bank lying over the far country on the right
        Cloud::bank(6_000.0, 30_000.0, 24_000.0, 8_000.0, 400.0, 1_000.0, 4).density(0.012),
    ]);
    let cf = clouds.field(&sf, w, 3.0);
    let (sf, cf) = (&sf, &cf);
    let nfar = Fbm::new(o.seed as u32 + 3, 4, 60.0);
    let crest = f.per_column(move |x| crest_y(w, x));
    let far_top = f.per_column(move |x| HZ - far_rise(x, &nfar));
    let band = Fbm::new(o.seed as u32 + 17, 3, 1.0);
    // a painter's sky: the light's colors, quieted toward the grey of lead
    // white, smalt and a little ochre
    let sky_col = move |x: f32, y: f32| -> Rgb {
        let y = y.min(HZ - 0.5);
        // the clouds held back: thin veils in the sky, not cut-outs on it
        let base = mix(sf.at(x, y), cf.color(sf, x, y), 0.6, Mix::Light);
        let l = luminance(base);
        let grey = [l * 1.02, l * 1.0, l * 0.97];
        let base = mix(base, grey, 0.3, Mix::Light);
        let b = band.get(x * 0.0022, y * 0.02);
        let k = 1.0 + 0.03 * b;
        [base[0] * k, base[1] * k, base[2] * k]
    };

    if std::env::args().any(|a| a == "--probe") {
        let s = |p: Rgb| format!("{:02x}{:02x}{:02x}", (paint::color::linear_to_srgb(p[0]) * 255.0) as u8, (paint::color::linear_to_srgb(p[1]) * 255.0) as u8, (paint::color::linear_to_srgb(p[2]) * 255.0) as u8);
        for y in [20.0, 120.0, 250.0, 350.0, 420.0, 448.0] {
            let row: Vec<String> = [30.0, 250.0, 500.0, 750.0, 970.0].iter().map(|&x| s(sky_col(x, y))).collect();
            eprintln!("y {y:>5}: {}", row.join(" "));
        }
        let cs: Vec<i32> = (0..11).map(|i| crest(i as f32 * 99.9 + 0.05) as i32).collect();
        eprintln!("crest {cs:?}");
        eprintln!("airlight {} {} {}", s(sf.airlight(50.0)), s(sf.airlight(400.0)), s(sf.airlight(900.0)));
        return;
    }

    // ---------------------------------------------------------------- sky
    let sky_m = Mask::from_fn(f, move |_, y| if y < HZ + 6.0 { 1.0 } else { 0.0 });
    if o.stage("sky", &mut c, &mut rng) {
        // a thin dead color in the sky's own tones first, shorter strokes,
        // so no warm ground is left bare where the long strokes run dry
        let hd = st.broad().color(sky_col).angle(|_, _| 0.0).angle_jitter(0.1).length(30.0, 80.0).coverage(2.6).medium(0.55).pressure(0.5, 0.7).clip(true).threshold(0.3);
        c.work(&sky_m, &hd, 10);
        c.wait(30.0);
        let hd = st.broad().color(sky_col).angle(|_, _| 0.0).angle_jitter(0.06).length(60.0, 200.0).coverage(3.4).medium(0.4).clip(true).threshold(0.3);
        c.work(&sky_m, &hd, 11);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.clip(true).angle(|_, _| 0.0).angle_jitter(0.1), 12);
        }
        // stippled into the wet, the same color from the field beneath
        let sp = Stipple::new(Tool::stippler(2.6)).mixed(pal, 0.45).color(sky_col).coverage(|_, _| 1.3).pressure(0.4, 0.75).dips(18, 0.4, 0.5).cluster(0.35, None).clip(true);
        c.stipple(&sky_m, &sp, 13);
        c.wait(90.0);
        // the cloud streaks, laid level into the still-open sky
        let body = cf.mask(f, |p| smoothstep(0.12, 0.6, p.alpha)).mul(&sky_m);
        let hd = st.body().color(sky_col).angle(|_, _| 0.0).angle_jitter(0.08).length(20.0, 70.0).coverage(2.2).pressure(0.45, 0.75).medium(0.35).clip(true).threshold(0.25);
        c.work(&body, &hd, 14);
        if let Some(b) = st.blend() {
            let soft = cf.mask(f, |p| smoothstep(0.03, 0.2, p.alpha)).mul(&sky_m);
            c.work(&soft, &b.clip(true).angle(|_, _| 0.0).angle_jitter(0.1).length(40.0, 120.0).pressure(0.3, 0.4), 15);
        }
        c.dry();
        // dry: a finer stipple barely off the field, denser toward the glow
        let sp = Stipple::new(Tool::stippler(1.7)).mixed(pal, 0.5).color(sky_col).coverage(move |x, y| 0.7 + 0.6 * smoothstep(250.0, 440.0, y) * (1.0 - smoothstep(0.0, 700.0, x))).pressure(0.35, 0.7).dips(16, 0.35, 0.6).cluster(0.4, None).clip(true);
        c.stipple(&sky_m, &sp, 16);
        c.dry();
    }

    // ------------------------------------------------ the far country
    let far_m = Mask::from_fn(f, move |x, y| if y >= far_top(x) && y < crest(x) + 3.0 { 1.0 } else { 0.0 });
    // the mist lying on the lowland: brightest just under the horizon
    let mist_col = move |x: f32, y: f32| -> Rgb {
        let air = sky_col(x, HZ - 3.0);
        let warm = mix(air, hex("#e8dcc4"), 0.25 * (1.0 - smoothstep(0.0, 600.0, x)), Mix::Light);
        let d = smoothstep(HZ, HZ + 22.0, y);
        mix(warm, mix(air, hex("#8d8f8c"), 0.35, Mix::Pigment), 0.5 * d, Mix::Pigment)
    };
    let land_col = move |x: f32, y: f32| -> Rgb {
        let air = sky_col(x, HZ - 3.0);
        let woods = smoothstep(0.8, 2.0, HZ - far_top(x)) * (1.0 - smoothstep(HZ - 0.5, HZ + 2.5, y));
        let base = mix(hex("#6f6e66"), hex("#3e4447"), woods, Mix::Pigment);
        // nearer (lower) is less veiled
        let d = smoothstep(HZ, HZ + 30.0, y);
        mix(base, air, 0.78 - 0.3 * d, Mix::Light)
    };
    let town = town_shapes();
    let town_m = {
        let mut s = Shape::new();
        for p in &town {
            s = s.add(Shape::new().poly(p));
        }
        Mask::from_shape(f, s).mul_fn(move |x, y| if y < crest(x) + 1.0 { 1.0 } else { 0.0 })
    };
    if o.stage("far", &mut c, &mut rng) {
        let hd = st.body().color(land_col).angle(|_, _| 0.0).angle_jitter(0.03).length(15.0, 45.0).coverage(3.0).pressure(0.45, 0.7).clip(true).threshold(0.3);
        c.work(&far_m, &hd, 21);
        c.dry();
        // the town: one firm shape each, cut in with a small sable, the
        // faces toward the low sun a shade warmer
        let town_col = move |x: f32, y: f32| -> Rgb {
            let air = sky_col(x, HZ - 3.0);
            let stone = mix(hex("#5c5250"), hex("#4a4a52"), smoothstep(285.0, 375.0, x), Mix::Pigment);
            let k = 0.5 + 0.1 * smoothstep(400.0, 440.0, y) + 0.3 * smoothstep(440.0, 453.0, y);
            mix(stone, air, k, Mix::Light)
        };
        let hd = Handling::new(Tool::round_sable(1.1)).mixed(pal, 0.12).color(town_col).angle(|_, _| std::f32::consts::FRAC_PI_2).angle_jitter(0.05).length(2.0, 7.0).coverage(3.2).pressure(0.55, 0.85).dips(4, 0.8, 0.7).clip(true).threshold(0.35);
        c.work(&town_m, &hd, 22);
        let mut held = Held::new(Tool::round_sable(0.45), 23);
        for s in mill_sails() {
            held.reload(pal.paint(town_col(448.0, 448.0), 0.15), 0.8);
            c.drag(&mut held, &Gesture::new(vec![s[0], s[1]]).pressure(0.6, 0.35), None);
        }
        c.wait(20.0);
        // mist over the lowland and the town's foot, laid level wet into it
        let veil = Mask::from_fn(f, move |x, y| {
            if y >= crest(x) + 1.0 {
                return 0.0;
            }
            let top = HZ - 4.0 - 3.0 * (x * 0.013).sin();
            smoothstep(top - 5.0, top + 3.0, y)
        });
        let hd = st.broad().color(mist_col).angle(|_, _| 0.0).angle_jitter(0.02).length(40.0, 140.0).coverage(1.6).medium(0.7).pressure(0.35, 0.55).clip(true).threshold(0.1).fill(false);
        c.work(&veil, &hd, 24);
        if let Some(b) = st.blend() {
            c.work(&veil.clone().blur(2.0), &b.angle(|_, _| 0.0).length(50.0, 150.0).pressure(0.3, 0.4), 25);
        }
        c.dry();
    }

    // ---------------------------------------------------------- the field
    let field_m = Mask::from_fn(f, move |x, y| if y >= crest(x) - 0.3 { 1.0 } else { 0.0 });
    let clods = Fbm::new(o.seed as u32 + 31, 4, 1.0);
    let at_ground = move |x: f32, y: f32| w.to_ground(x, y.max(crest(x) + 0.4));
    // the earth's mean color at depth z (m): dark umber loam near, cooler
    // and paler up the rise, where the sky light lies along it
    let earth_mean = move |x: f32, gx: f32, gz: f32| -> Rgb {
        let n = clods.get(gx * 0.35, gz * 0.35);
        let c0 = mix(hex("#3a3029"), hex("#52443a"), 0.5 + 0.5 * n, Mix::Pigment);
        let c0 = mix(c0, hex("#2a241f"), 0.45 * (1.0 - smoothstep(6.0, 16.0, gz)), Mix::Pigment);
        let c0 = mix(c0, hex("#6a625a"), 0.4 * smoothstep(14.0, 58.0, gz), Mix::Pigment);
        let c0 = mix(c0, hex("#262328"), 0.55 * stone_shadow(gx, gz), Mix::Pigment);
        mix(c0, sky_col(x, HZ - 2.0), w.aerial(gz) + 0.16 * smoothstep(30.0, 70.0, gz), Mix::Light)
    };
    // the lay-in already knows the furrows where they come too close for a
    // line each: a faint ridge-and-trough banding along the brush
    let earth = move |x: f32, y: f32| -> Rgb {
        let Some(p) = at_ground(x, y) else { return hex("#4a4038") };
        let (gx, gz) = (p[0], p[2]);
        let m = earth_mean(x, gx, gz);
        let ph = furrow_u(gx, gz).rem_euclid(1.0);
        let lit = (ph - 0.34).abs() < 0.16;
        let dark = (ph - 0.82).abs() < 0.12;
        let k = smoothstep(10.0, 22.0, gz) * 0.8;
        if lit {
            mix(m, hex("#8a7a68"), 0.35 * k, Mix::Pigment)
        } else if dark {
            mix(m, hex("#211b17"), 0.35 * k, Mix::Pigment)
        } else {
            m
        }
    };
    let along = move |x: f32, y: f32| -> f32 {
        let Some(p) = at_ground(x, y) else { return 0.0 };
        let head = furrow_head(p[2]);
        let (dx, dz) = (head.sin() * 0.5, head.cos() * 0.5);
        let a = w.project([p[0], ground(p[0], p[2]), p[2]]).unwrap_or((x, y));
        let b = w.project([p[0] + dx, ground(p[0] + dx, p[2] + dz), p[2] + dz]).unwrap_or((x + 1.0, y));
        (b.1 - a.1).atan2(b.0 - a.0)
    };
    let zmap = Mask::from_fn(f, move |x, y| if y >= crest(x) - 0.3 { at_ground(x, y).map_or(80.0, |p| p[2]) } else { 1e4 });
    // the track: lateral offset (m) from its middle at a canvas point
    let lateral = move |x: f32, y: f32| -> f32 { at_ground(x, y).map_or(99.0, |p| p[0] - track_x(p[2])) };
    let lat_map = Mask::from_fn(f, move |x, y| if y >= crest(x) - 0.3 { lateral(x, y) } else { 99.0 });
    let track_m = lat_map.clone().map(|s| 1.0 - smoothstep(TRACK_W * 0.5 - 0.12, TRACK_W * 0.5 + 0.12, s.abs())).mul(&field_m);
    let bands: [(f32, f32, f32); 4] = [(0.0, 13.0, 7.0), (11.0, 24.0, 4.2), (21.0, 42.0, 2.4), (38.0, 1e3, 1.5)];
    if o.stage("field", &mut c, &mut rng) {
        // the dead color: loam laid along the furrows, brush by distance
        for (i, &(z0, z1, wd)) in bands.iter().enumerate() {
            let m = zmap.clone().map(move |z| if z >= z0 && z < z1 { 1.0 } else { 0.0 }).mul(&field_m);
            let tool = Tool { lay: 0.75, ..Tool::filbert(wd) };
            let hd = Handling::new(tool).mixed(pal, 0.2).mix_jitter(0.05).color(earth).angle(along).angle_jitter(0.04).length(wd * 4.0, wd * 12.0).coverage(3.0).pressure(0.6, 0.9).curve(0.02, 0.15).tail(0.15).broken(0.08).swell(0.2).dips(2, 0.6, 0.6).clip(true).threshold(0.2);
            c.work(&m, &hd, 40 + i as u64);
        }
        c.wait(40.0);
    }

    // ------------------------------------------------------- the furrows
    // each turned slice drawn along its length: the dark of the trough in
    // long pulls, then the face the low sun finds, broken into clods; the
    // brushes smaller up the rise
    let furrow_clip = field_m.clone().subtract(&track_m.clone().blur(0.8));
    if o.stage("furrows", &mut c, &mut rng) {
        let mut r = Rng::new(501);
        let (u_lo, u_hi) = {
            let mut lo = f32::MAX;
            let mut hi = f32::MIN;
            for &z in &[4.0f32, 10.0, 20.0, 45.0, 70.0] {
                for &x in &[-0.6f32, 0.6] {
                    let u = furrow_u(x * z, z);
                    lo = lo.min(u);
                    hi = hi.max(u);
                }
            }
            (lo.floor() - 2.0, hi.ceil() + 2.0)
        };
        let path = |u: f32, z0: f32, z1: f32| -> Vec<(f32, f32)> {
            let n = 10;
            (0..=n).filter_map(|i| {
                let zz = z0 + (z1 - z0) * i as f32 / n as f32;
                let gx = furrow_x(u, zz);
                w.project([gx, ground(gx, zz), zz])
            }).collect()
        };
        let off = |pts: &[(f32, f32)]| pts.iter().all(|p| p.0 < -5.0 || p.0 > 1005.0 || p.1 > H + 5.0);
        let mut trough = Held::new(Tool::round_sable(4.0), 502);
        let mut face = Held::new(Tool::filbert(4.0), 503);
        let mut k = u_lo;
        while k <= u_hi {
            // the trough: long pulls, a new stroke where the hand runs out
            let u = k + 0.82 + r.range(-0.03, 0.03);
            let mut z = 3.8 + r.range(0.0, 0.8);
            while z < 72.0 {
                let piece = (r.range(4.0, 9.0) * (z / 8.0).powf(0.9)).clamp(3.0, 30.0);
                let z1 = z + piece;
                let zm = 0.5 * (z + z1);
                let pts = path(u, z, z1);
                z = z1 - 0.15 * piece;
                let px = FURROW * w.scale_at(zm);
                if pts.len() < 2 || off(&pts) || px < 1.6 {
                    continue;
                }
                let (mx, _) = pts[pts.len() / 2];
                let mean = earth_mean(mx, furrow_x(u, zm), zm);
                let wd = (0.2 * px).clamp(0.55, 7.0);
                trough.tool = Tool::round_sable(wd);
                let dark = mix(mean, hex("#17120f"), 0.6 - 0.25 * smoothstep(20.0, 60.0, zm), Mix::Pigment);
                trough.reload(pal.paint(dark, 0.22), r.range(0.7, 1.0));
                let g = Gesture::new(pts).pressure(r.range(0.5, 0.85), r.range(0.35, 0.7)).swell(vec![1.0, r.range(0.7, 1.2), r.range(0.7, 1.2), 1.0]);
                c.drag(&mut trough, &g, Some(&furrow_clip));
            }
            // the lit face: clods and short runs along the ridge, some
            // warm in the sun, some cool where the sky finds their tops
            let mut z = 3.8 + r.range(0.0, 1.5);
            while z < 72.0 {
                let piece = r.range(0.6, 2.6) * (z / 8.0).powf(0.8) * (1.0 + 1.4 * smoothstep(14.0, 40.0, z));
                let z1 = z + piece;
                let zm = 0.5 * (z + z1);
                let gap = if r.chance(0.35 - 0.2 * smoothstep(14.0, 40.0, z)) { r.range(0.3, 1.4) * piece } else { -r.range(0.0, 0.15) * piece };
                let u = k + r.range(0.26, 0.42);
                let pts = path(u, z, z1);
                z = z1 + gap;
                let px = FURROW * w.scale_at(zm);
                if pts.len() < 2 || off(&pts) || px < 1.6 {
                    continue;
                }
                let (mx, _) = pts[pts.len() / 2];
                if stone_shadow(furrow_x(u, zm), zm) > 0.3 {
                    continue;
                }
                let mean = earth_mean(mx, furrow_x(u, zm), zm);
                let wd = (r.range(0.18, 0.34) * px).clamp(0.6, 12.0);
                let sunny = mix(hex("#8c6f52"), hex("#a08a72"), smoothstep(10.0, 50.0, zm), Mix::Pigment);
                let cool = hex("#77726e");
                let hue = if r.chance(0.12) { cool } else { sunny };
                let lit = mix(mean, hue, 0.36 + r.range(-0.14, 0.1) - 0.1 * (1.0 - smoothstep(5.0, 12.0, zm)), Mix::Pigment);
                // a drier brush dragged along the slice, so it catches the
                // lumps and leaves the dead color between them; near, a
                // run is two or three overlapping touches, not one bar
                let n = if px > 14.0 { r.range(1.0, 3.99) as usize } else { 1 };
                let len = pts.len();
                for j in 0..n {
                    let a = (j * len / n).min(len - 2);
                    let b = (((j + 1) * len / n) + 1).min(len);
                    let dv = r.range(-0.07, 0.07) * px;
                    let seg: Vec<(f32, f32)> = pts[a..b].iter().map(|&(x, y)| (x + dv * 0.3, y + dv)).collect();
                    face.tool = Tool { lay: 0.7, ragged: 0.8, ..Tool::filbert(wd * r.range(0.7, 1.25)) };
                    face.reload(pal.paint(lit, 0.16), if px > 14.0 { r.range(0.3, 0.75) } else { r.range(0.5, 0.9) });
                    let g = Gesture::new(seg).pressure(r.range(0.3, 0.75), r.range(0.2, 0.65)).ramps(0.25, 0.35).swell(vec![r.range(0.6, 1.2), r.range(0.5, 1.3), r.range(0.5, 1.2)]);
                    c.drag(&mut face, &g, Some(&furrow_clip));
                }
            }
            k += 1.0;
        }
        c.dry();
    }

    // --------------------------------------------------------- the track
    // two ruts, a grass hump between, the worn verges; water standing in
    // the ruts where they dip, the pale sky in it
    let ruts = lat_map.clone().map(|s| {
        let d = (s.abs() - 0.78).abs();
        1.0 - smoothstep(0.14, 0.24, d)
    }).mul(&track_m);
    let wet_n = Fbm::new(o.seed as u32 + 71, 3, 1.0);
    let puddle = move |x: f32, y: f32| -> f32 {
        let Some(p) = at_ground(x, y) else { return 0.0 };
        let s = p[0] - track_x(p[2]);
        let side = if s < 0.0 { 0.0 } else { 7.0 };
        let v = wet_n.get(p[2] * 0.22 + side, 0.5);
        let ragged = 0.05 * wet_n.get(p[2] * 1.7, 3.0 + side);
        let across = 1.0 - smoothstep(0.05, 0.13, (s.abs() - 0.78 + ragged).abs());
        smoothstep(0.42, 0.52, v) * across * smoothstep(14.0, 20.0, p[2]) * (1.0 - smoothstep(40.0, 48.0, p[2]))
    };
    let puddle_m = Mask::from_fn(f, move |x, y| if y >= crest(x) { puddle(x, y) } else { 0.0 }).mul(&track_m);
    let track_along = move |x: f32, y: f32| -> f32 {
        let Some(p) = at_ground(x, y) else { return 0.0 };
        let z = p[2];
        let a = w.project([track_x(z), ground(track_x(z), z), z]).unwrap_or((x, y));
        let b = w.project([track_x(z + 0.8), ground(track_x(z + 0.8), z + 0.8), z + 0.8]).unwrap_or((x, y - 1.0));
        (b.1 - a.1).atan2(b.0 - a.0)
    };
    let track_col = move |x: f32, y: f32| -> Rgb {
        let Some(p) = at_ground(x, y) else { return hex("#5a5048") };
        let s = (p[0] - track_x(p[2])).abs();
        let m = earth_mean(x, p[0], p[2]);
        // packed earth takes the sky's grey; the hump keeps its dry grass
        let packed = mix(m, hex("#7a746c"), 0.35, Mix::Pigment);
        let grass = mix(m, hex("#7c7456"), 0.45, Mix::Pigment);
        let rut = mix(m, hex("#2a2420"), 0.35, Mix::Pigment);
        let g = 1.0 - smoothstep(0.3, 0.45, s);
        let rr = 1.0 - smoothstep(0.1, 0.22, (s - 0.78).abs());
        mix(mix(packed, grass, g, Mix::Pigment), rut, rr, Mix::Pigment)
    };
    if o.stage("track", &mut c, &mut rng) {
        for (i, &(z0, z1, wd)) in bands.iter().enumerate() {
            let m = zmap.clone().map(move |z| if z >= z0 && z < z1 { 1.0 } else { 0.0 }).mul(&track_m);
            let tool = Tool { lay: 0.75, ..Tool::filbert(wd * 0.8) };
            let hd = Handling::new(tool).mixed(pal, 0.2).mix_jitter(0.05).color(track_col).angle(track_along).angle_jitter(0.05).length(wd * 3.0, wd * 9.0).coverage(3.0).pressure(0.55, 0.85).curve(0.02, 0.15).tail(0.15).broken(0.1).swell(0.2).dips(2, 0.6, 0.6).clip(true).threshold(0.25);
            c.work(&m, &hd, 60 + i as u64);
        }
        c.wait(30.0);
        // the ruts drawn along their length, like the furrows: two long
        // dark pulls, a new stroke where the hand runs out
        let mut r = Rng::new(701);
        let mut rut = Held::new(Tool::round_sable(3.0), 702);
        for side in [-0.78f32, 0.78] {
            let mut z = 12.5 + r.range(0.0, 1.0);
            while z < 68.0 {
                let piece = (r.range(3.0, 7.0) * (z / 12.0).powf(0.9)).clamp(2.5, 20.0);
                let z1 = z + piece;
                let pts: Vec<(f32, f32)> = (0..=10).filter_map(|i| {
                    let zz = z + piece * i as f32 / 10.0;
                    let gx = track_x(zz) + side + 0.05 * (zz * 0.7 + side * 3.0).sin();
                    w.project([gx, ground(gx, zz), zz])
                }).collect();
                let zm = 0.5 * (z + z1);
                z = z1 - 0.1 * piece;
                if pts.len() < 2 {
                    continue;
                }
                let px = w.scale_at(zm);
                let (mx, _) = pts[pts.len() / 2];
                let dark = mix(earth_mean(mx, track_x(zm) + side, zm), hex("#1e1916"), 0.45, Mix::Pigment);
                rut.tool = Tool::round_sable((0.22 * px).clamp(0.6, 9.0));
                rut.reload(pal.paint(dark, 0.2), r.range(0.6, 0.95));
                let g = Gesture::new(pts).pressure(r.range(0.45, 0.8), r.range(0.35, 0.7)).swell(vec![1.0, r.range(0.7, 1.2), r.range(0.7, 1.2), 1.0]);
                c.drag(&mut rut, &g, Some(&track_m));
            }
        }
        // dry grass on the hump between the ruts: fine upturned strokes,
        // in clumps, longer near
        let hump = lat_map.clone().map(|s| 1.0 - smoothstep(0.22, 0.4, s.abs())).mul(&track_m).mul(&zmap.clone().map(|z| 1.0 - smoothstep(26.0, 34.0, z)));
        let grass_col = move |x: f32, y: f32| -> Rgb {
            let Some(p) = at_ground(x, y) else { return hex("#6e6448") };
            mix(earth_mean(x, p[0], p[2]), hex("#8a7d58"), 0.5, Mix::Pigment)
        };
        let hd = Handling::new(Tool { point: 1.0, ..Tool::round_sable(0.9) }).mixed(pal, 0.15).color(grass_col).angle(|_, _| -std::f32::consts::FRAC_PI_2).angle_jitter(0.35).length(3.0, 9.0).coverage(0.9).clump(0.8).pressure(0.3, 0.7).ramps(0.1, 0.6).dips(4, 0.7, 0.6).clip(false).threshold(0.3).fill(false);
        c.work(&hump, &hd, 72);
        // the standing water: the sky it mirrors, laid level and stroked
        // along the rut
        let water_col = move |x: f32, y: f32| -> Rgb {
            let my = 2.0 * HZ - y;
            let sky = sky_col(x, my.max(40.0));
            mix(sky, hex("#3e3a36"), 0.6, Mix::Pigment)
        };
        let hd = Handling::new(Tool { lay: 0.8, ..Tool::filbert(1.6) }).mixed(pal, 0.15).color(water_col).angle(track_along).angle_jitter(0.02).length(3.0, 12.0).coverage(3.2).pressure(0.55, 0.8).clip(true).threshold(0.35);
        c.work(&puddle_m, &hd, 70);
        // their edges fused into the wet rut with a small soft brush
        let fuse = Handling::new(Tool::badger(3.0)).blender().angle(track_along).angle_jitter(0.05).length(4.0, 10.0).coverage(1.6).pressure(0.3, 0.45).dips(3, 0.0, 0.9).clip(true).threshold(0.1);
        c.work(&puddle_m.clone().blur(1.5), &fuse, 71);
        c.dry();
    }

    let _ = &ruts;

    // ---------------------------------------------------------- the clods
    // the near ground broken into clods: dark hollows and a few lit tops,
    // in clusters, smaller up the rise
    if o.stage("clods", &mut c, &mut rng) {
        let clod_col = move |dark: bool| {
            move |x: f32, y: f32| -> Rgb {
                let Some(p) = at_ground(x, y) else { return hex("#3a3029") };
                let m = earth_mean(x, p[0], p[2]);
                if dark { mix(m, hex("#1a1512"), 0.5, Mix::Pigment) } else { mix(m, hex("#8d7862"), 0.35, Mix::Pigment) }
            }
        };
        for (i, &(z0, z1, wd)) in bands.iter().enumerate().take(3) {
            let m = zmap.clone().map(move |z| if z >= z0 && z < z1 { 1.0 } else { 0.0 }).mul(&furrow_clip);
            let cov = [0.08f32, 0.08, 0.07][i];
            let sp = Stipple::new(Tool::stippler(wd * 0.6)).mixed(pal, 0.15).color(clod_col(true)).coverage(move |_, _| cov).pressure(0.2, 0.9).dips(12, 0.5, 0.5).cluster(0.7, None).clip(true);
            c.stipple(&m, &sp, 80 + i as u64);
            let sp = Stipple::new(Tool::stippler(wd * 0.35)).mixed(pal, 0.15).color(clod_col(false)).coverage(move |_, _| cov * 0.45).pressure(0.35, 0.75).dips(12, 0.5, 0.5).cluster(0.75, None).clip(true);
            c.stipple(&m, &sp, 90 + i as u64);
        }
        c.dry();
    }

    // ---------------------------------------------------------- the crest
    // the ploughed crest against the mist is not a line: the ridges stand
    // up end-on in small uneven teeth, painted with the crest's own paint
    // taken from just below each
    if o.stage("crest", &mut c, &mut rng) {
        let mut r = Rng::new(611);
        let mut tooth = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.6) }, 612);
        let mut x = 2.0;
        while x < 998.0 {
            let cy = crest(x);
            // the furrow spacing there, on the canvas
            let z = w.to_ground(x, cy + 0.6).map_or(60.0, |p| p[2]);
            let px = FURROW * w.scale_at(z);
            let step = px * r.range(0.5, 2.2);
            let skip = r.chance(0.4);
            if !skip {
                // a low lump: a clod or a ridge's end, broad and uneven
                let hgt = (0.1 * w.scale_at(z)) * r.range(0.1, 0.6);
                let under = c.sample(x, cy + 1.5);
                let col = mix(under, hex("#3b342e"), r.range(0.0, 0.2), Mix::Pigment);
                let wd = (px * r.range(0.25, 0.45)).clamp(0.7, 2.4);
                tooth.tool = Tool::round_sable(wd);
                tooth.reload(pal.paint(col, 0.15), 0.9);
                let run = px * r.range(0.4, 1.3);
                let g = Gesture::new(vec![(x - run * 0.5, cy + 0.6), (x, cy + wd * 0.5 - hgt), (x + run * 0.5, cy + 0.6)]).pressure(r.range(0.4, 0.75), r.range(0.3, 0.6)).ramps(0.2, 0.3);
                c.drag(&mut tooth, &g, None);
            }
            x += step;
        }
        c.wait(30.0);
    }

    // --------------------------------------------------------- the stones
    let mut ws = world();
    let mut ids = vec![];
    for (i, &(sx, sz, rx, h, rz)) in STONES.iter().enumerate() {
        let sp = ws.spot_at(sx, sz);
        let cc = sp.p(0.0, h * 0.42, 0.0);
        let sd = Sdf::ellipsoid(cc, sp.size(rx, h * 0.62, rz))
            .union(Sdf::block(sp.p(0.0, h * 0.25, 0.0), sp.size(rx * 1.5, h * 0.6, rz * 1.4), sp.m(rx * 0.5)), sp.m(rx * 0.3))
            .turn(cc, 0.3 + i as f32 * 0.7, 0.08, -0.05)
            .rough(sp.m(rx * 0.12), sp.m(rx * 1.2), 3, false)
            .cut(sp.p(rx * 0.3, h * 0.8, 0.0), [0.4, -1.0, 0.2], 20 + i as u16, sp.m(0.03));
        // the lone stone near the frame: a broken block, half in the earth
        let sd = if i == STONES.len() - 1 {
            Sdf::block(sp.p(0.0, h * 0.2, 0.0), sp.size(rx * 2.0, h * 1.6, rz * 2.0), sp.m(rx * 0.35))
                .turn(cc, 0.5, 0.12, -0.1)
                .cut(sp.p(-rx * 0.4, h * 0.9, 0.0), [-0.6, -1.0, 0.1], 40, sp.m(0.02))
                .cut(sp.p(rx * 0.6, h * 0.6, 0.0), [1.0, -0.4, 0.3], 41, sp.m(0.02))
                .rough(sp.m(rx * 0.1), sp.m(rx * 0.9), 3, false)
        } else {
            sd
        };
        ids.push(ws.place(sp, sd));
    }
    if o.stage("stones", &mut c, &mut rng) {
        let sform = ws.form_of(f, &ids);
        let stone = Stone { light: hex("#948a7c"), half: hex("#625d58"), shadow: hex("#39383c"), core: hex("#252427"), bounce: hex("#4a4038"), crevice: hex("#141315") };
        let col = move |_: f32, _: f32, q: &paint::form::Sample| stone.at(&q.shade);
        let parts: Vec<u16> = (1..=ids.len() as u16).collect();
        rocks::paint_solid(&mut c, &st, &sform, &rocks::Solid { parts, color: &col, soft: &|_| 0.5, scale: 0.35, accents: 0.15, seed: 101 });
        c.dry();
        // small field stones turned up by the plough, in loose drifts: a
        // dark body, the rim the low sun finds on the left, the long thin
        // shadow thrown toward the painter and to the right
        let mut r = Rng::new(151);
        let mut body = Held::new(Tool::round_sable(2.0), 152);
        let mut rim = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.0) }, 153);
        let centers: Vec<(f32, f32)> = (0..7).map(|_| (r.range(-0.4, 0.4), r.range(6.5, 22.0))).collect();
        for (ci, &(cxf, cz)) in centers.iter().enumerate() {
            let n = 1 + (r.f() * 5.0) as usize;
            for _ in 0..n {
                let z = cz + r.range(-1.8, 1.8);
                let x = cxf * z * 0.9 + r.range(-1.5, 1.5);
                let lat = (x - track_x(z)).abs();
                let heap = STONES[..8].iter().any(|&(sx, sz, ..)| (sx - x).abs() < 1.6 && (sz - z).abs() < 1.6);
                if lat < TRACK_W * 0.5 + 0.2 || heap || z < 6.0 {
                    continue;
                }
                let sp = w.spot_at(x, z);
                if sp.x < 3.0 || sp.x > 997.0 || sp.y > H - 2.0 {
                    continue;
                }
                let rad = r.range(0.035, 0.11);
                let px = sp.m(rad);
                let (bx, by) = (sp.x, sp.y - px * 0.35);
                // shadow first, on the ground
                let sh_to = w.project([x + 0.85 * rad * 3.5, ground(x + 0.85 * rad * 3.5, z - 0.53 * rad * 3.5), z - 0.53 * rad * 3.5]).unwrap_or((bx + px * 3.0, by + px));
                let under = c.sample(sh_to.0, sh_to.1);
                body.tool = Tool::round_sable((px * 0.9).max(0.6));
                body.reload(pal.paint(mix(under, hex("#1c1917"), 0.3, Mix::Pigment), 0.2), 0.7);
                c.drag(&mut body, &Gesture::new(vec![(bx, by + px * 0.3), sh_to]).pressure(0.45, 0.05).ramps(0.1, 0.6), None);
                // the stone
                let tone = mix(hex("#3c3936"), hex("#57514b"), r.f(), Mix::Pigment);
                body.tool = Tool::round_sable((px * 1.3).max(0.7));
                body.reload(pal.paint(tone, 0.12), 1.0);
                c.drag(&mut body, &Gesture::new(vec![(bx - px * 0.35, by), (bx + px * 0.35, by + px * 0.05)]).pressure(0.8, 0.7), None);
                // the lit rim, left and on top
                rim.tool = Tool { point: 1.0, ..Tool::round_sable((px * 0.3).max(0.45)) };
                let lit = mix(tone, mix(hex("#8f8374"), hex("#a39686"), r.f(), Mix::Pigment), 0.55, Mix::Pigment);
                rim.reload(pal.paint(lit, 0.12), 0.8);
                c.drag(&mut rim, &Gesture::new(vec![(bx - px * 0.55, by + px * 0.15), (bx - px * 0.3, by - px * 0.4), (bx + px * r.range(0.0, 0.3), by - px * 0.5)]).pressure(0.5, 0.15), None);
            }
            let _ = ci;
        }
        c.dry();
    }

    // ---------------------------------------------------------- the crows
    if o.stage("crows", &mut c, &mut rng) {
        let birds: [(f32, f32, f32, bool); 6] = [(-3.1, 13.0, 1.0, true), (-4.6, 21.0, -1.0, false), (-3.0, 23.2, 1.0, true), (-6.6, 25.0, 1.0, true), (-10.5, 44.0, -1.0, false), (-9.3, 46.5, 1.0, true)];
        for (i, &(bx, bz, face, peck)) in birds.iter().enumerate() {
            let sp = w.spot_at(bx, bz);
            crow(&mut c, pal, (sp.x, sp.y), sp.s, face, peck, 200 + i as u64);
        }
        c.dry();
        // the veil a painter lays over the finished picture, darker toward
        // the foot and the lower corners
        let vig = Mask::from_fn(f, |x, y| {
            let dx = (x - 420.0) / 620.0;
            let dy = ((y - 470.0) / 260.0).max(0.0);
            smoothstep(0.35, 1.25, (dx * dx + dy * dy).sqrt()) * smoothstep(HZ, HZ + 60.0, y)
        });
        let umber = pal.only(&["raw umber", "bone black"]).mix(hex("#2e2924")).paint(0.9).pigment();
        c.glaze(&umber, Some(&vig), |_, _| 3.0);
        c.dry();
    }
    // --------------------------------------------------------- the cranes
    // a skein of cranes going over, high and far, in a loose wavering line
    if o.stage("cranes", &mut c, &mut rng) {
        let mut r = Rng::new(801);
        let mut b = Held::new(Tool { point: 1.0, ..Tool::round_sable(0.9) }, 802);
        let n = 17;
        for i in 0..n {
            let t = i as f32 / (n - 1) as f32;
            // two arms of an uneven V, the lead bird at the left
            let arm = if i % 3 == 1 { -1.0 } else { 1.0 };
            let along = t * 150.0;
            let x = 640.0 + along + r.range(-3.0, 3.0);
            let y = 150.0 - 0.12 * along + arm * along * 0.16 + 4.0 * (t * 9.0).sin() + r.range(-2.0, 2.0);
            let span = r.range(4.2, 6.2) * (1.0 - 0.25 * t);
            let up = r.range(-1.2, 1.4);
            let col = mix(sky_col(x, y), hex("#2c2b30"), 0.62 + r.range(-0.06, 0.06), Mix::Pigment);
            b.tool = Tool { point: 1.0, ..Tool::round_sable((0.2 * span).max(0.6)) };
            b.reload(pal.paint(col, 0.12), 0.9);
            let pts = vec![(x - span * 0.5, y - up), (x - span * 0.18, y - 0.25), (x, y), (x + span * 0.18, y - 0.25), (x + span * 0.5, y - up * r.range(0.7, 1.1))];
            c.drag(&mut b, &Gesture::new(pts).pressure(0.15, 0.15).swell(vec![0.4, 1.4, 2.2, 1.4, 0.4]).ramps(0.1, 0.1), None);
            // the long neck forward, the legs trailing
            b.tool = Tool { point: 1.0, ..Tool::round_sable(0.5) };
            b.reload(pal.paint(col, 0.12), 0.8);
            c.drag(&mut b, &Gesture::new(vec![(x + 0.4, y + 0.1), (x - span * 0.32, y + 0.4)]).pressure(0.5, 0.2), None);
        }
        c.dry();
    }

    o.finish(&mut c, &mut rng, &Finish { cracks: Some(paint::Cracks { width_um: Some(15.0), grain: 0.55, ..paint::Cracks::aged(0) }), ..Finish::aged(st.relief) });
}
