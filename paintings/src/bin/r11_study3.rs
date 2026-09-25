//! r11_study3: one old trunk rising out of snow against a pale evening sky.
//! A small study after Friedrich's materials and habits (notes/r11_study3.md).
//!
//!   cargo paint r11_study3                        1000px
//!   cargo paint r11_study3 -- --full              3200px
//!   cargo paint r11_study3 -- --full --crop 330,1030,640,1250   the foot in the snow
//!   cargo paint r11_study3 -- --full --crop 360,380,760,620     trunk, limb and sky
//!
//! Order of work, as he worked: bought ground, a pencil drawing, the sky laid
//! thin and stippled, the far snow, the snow field, then the tree painted on
//! the finished sky, snow brought back over its foot, shadows, and the grass
//! stalks last over the finished snow [NG p.56; ALF p.346].

use paint::color::{Mix, mix};
use paint::graphite::sketch_marks;
use paint::{Fbm, Gesture, Held, Lead, Mask, Paint, Rng, Stipple, Style, Tool, Touch, gradient, hex, shift, smoothstep};
use std::f32::consts::FRAC_PI_2;

const ASPECT: f32 = 0.75;
/// where the trunk stands in the snow (units)
const BASE_Y: f32 = 1150.0;

/// the trunk's axis: leaning a little right as it rises, with a slow wander
fn axis(y: f32) -> f32 {
    430.0 + 0.055 * (BASE_Y - y) + 9.0 * (y / 190.0).sin() + 5.0 * (y / 71.0 + 1.3).sin()
}
/// half the trunk's width at height y: flaring into the roots near the snow
fn half_w(y: f32) -> f32 {
    let t = (y / BASE_Y).clamp(0.0, 1.2);
    let flare = smoothstep(900.0, 1170.0, y);
    let foot = smoothstep(1060.0, 1175.0, y);
    27.0 + 15.0 * t + 26.0 * flare * flare + 20.0 * foot * foot * foot
}
fn bump(y: f32, at: f32, size: f32, reach: f32) -> f32 {
    size * (-((y - at) / reach).powi(2)).exp()
}
/// the trunk's two edges: an old oak is not a pole; burls, the collar
/// under the limb, a hollow where a branch was lost long ago
fn left_edge(y: f32) -> f32 {
    axis(y) - half_w(y) - bump(y, 612.0, 7.0, 26.0) - bump(y, 742.0, 6.0, 20.0) - bump(y, 1040.0, 9.0, 60.0) + bump(y, 880.0, 4.0, 30.0) - bump(y, 240.0, 3.5, 40.0)
}
fn right_edge(y: f32) -> f32 {
    axis(y) + half_w(y) + bump(y, 522.0, 12.0, 34.0) + bump(y, 830.0, 8.0, 22.0) - bump(y, 700.0, 4.0, 40.0) + bump(y, 1080.0, 10.0, 50.0) + bump(y, 150.0, 3.0, 25.0)
}
/// the snow surface far off (a low rise, a shallow dip)
fn far_snow(x: f32) -> f32 {
    968.0 + 9.0 * (x / 230.0 + 0.4).sin() - 16.0 * smoothstep(620.0, 1000.0, x) + 4.0 * (x / 57.0).sin()
}
/// where the snow meets the trunk: drift piled on the left (windward), a
/// scoop on the right
fn contact(x: f32) -> f32 {
    let u = (x - axis(BASE_Y)) / half_w(BASE_Y);
    BASE_Y - 22.0 + 30.0 * smoothstep(-1.2, 1.1, u) - 10.0 * (-(u * u) * 1.5).exp() + 3.0 * (x / 13.0).sin() * (x / 31.0 + 0.7).cos() - 6.0 * (-((u + 0.35) / 0.2).powi(2)).exp()
}

/// the big limb leaving the trunk to the right, and a broken stub to the left
const LIMB: [(f32, f32); 8] = [(468.0, 505.0), (555.0, 446.0), (640.0, 421.0), (702.0, 380.0), (790.0, 356.0), (878.0, 298.0), (955.0, 276.0), (1040.0, 226.0)];
const LIMB_W: [f32; 8] = [44.0, 32.0, 27.0, 23.0, 19.0, 16.0, 13.0, 11.0];
/// an old dead branch high on the left, crooked, broken off
const DEAD: [(f32, f32); 6] = [(472.0, 176.0), (424.0, 146.0), (392.0, 134.0), (352.0, 100.0), (318.0, 90.0), (284.0, 62.0)];
const DEAD_W: [f32; 6] = [22.0, 15.0, 12.5, 10.0, 8.0, 6.5];
const STUB: [(f32, f32); 3] = [(420.0, 742.0), (372.0, 706.0), (338.0, 690.0)];
const STUB_W: [f32; 3] = [30.0, 22.0, 17.0];

fn seg_dist(p: (f32, f32), a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let t = (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
    let (qx, qy) = (a.0 + dx * t, a.1 + dy * t);
    (((p.0 - qx).powi(2) + (p.1 - qy).powi(2)).sqrt(), t)
}
/// signed coverage of a tapered polyline limb
fn limb_cov(x: f32, y: f32, pts: &[(f32, f32)], w: &[f32]) -> (f32, f32) {
    let (v, a, _) = limb_side(x, y, pts, w);
    (v, a)
}
/// coverage, direction, and which side of the limb (-1 underside .. 1 top)
fn limb_side(x: f32, y: f32, pts: &[(f32, f32)], w: &[f32]) -> (f32, f32, f32) {
    let mut best = 0.0f32;
    let mut ang = 0.0;
    let mut side = 0.0;
    let mut bestd = f32::MAX;
    let last = pts.len() - 2;
    for i in 0..pts.len() - 1 {
        let (d, t) = seg_dist((x, y), pts[i], pts[i + 1]);
        let hw = 0.5 * (w[i] + (w[i + 1] - w[i]) * t);
        let mut v = smoothstep(hw + 0.8, hw - 0.8, d);
        if i == last {
            // a limb broken off ends in a cut, not a rounded cap
            let (dx, dy) = (pts[i + 1].0 - pts[i].0, pts[i + 1].1 - pts[i].1);
            let l = (dx * dx + dy * dy).sqrt();
            let along = ((x - pts[i + 1].0) * dx + (y - pts[i + 1].1) * dy) / l;
            v *= smoothstep(0.8, -0.8, along);
        }
        if v > best || (v == best && d < bestd) {
            best = v;
            bestd = d;
            let (dx, dy) = (pts[i + 1].0 - pts[i].0, pts[i + 1].1 - pts[i].1);
            ang = dy.atan2(dx);
            // cross product sign: above the line (smaller y) is the top
            let cr = (dx * (y - pts[i].1) - dy * (x - pts[i].0)) / (dx * dx + dy * dy).sqrt();
            side = (-cr / hw.max(1.0)).clamp(-1.0, 1.0);
        }
    }
    (best, ang, side)
}

fn main() {
    let o = paintings::run::Run::new("r11_study3");
    // a small canvas: about 26 × 35 cm, like his small winter pieces
    let st = Style { width_mm: 260.0, ..Style::friedrich() };
    let pal = &st.palette;
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = c.height();

    // ---- the sky he wants: cool gray-blue above, a pale greenish band, a
    // warm glow low down, a breath of rose just above the snow
    let sky_col = |_x: f32, y: f32| {
        let t = (y / 975.0).clamp(0.0, 1.0);
        gradient(
            &[(0.0, hex("#7d8ba0")), (0.3, hex("#9eaab2")), (0.58, hex("#c3c7ba")), (0.8, hex("#e0d6b8")), (0.93, hex("#ead2ae")), (1.0, hex("#e2c6ad"))],
            t,
            Mix::Light,
        )
    };
    let sky_m = Mask::from_fn(f, |x, y| 1.0 - smoothstep(far_snow(x) + 6.0, far_snow(x) + 14.0, y));
    let land_m = Mask::from_fn(f, |x, y| smoothstep(far_snow(x) - 1.0, far_snow(x) + 1.0, y));

    if o.stage("drawing", &mut c, &mut rng) {
        // a light pencil drawing: the trunk's two edges, the limb, the stub
        // and the line of the far snow, gone over twice as he did
        let lead = Lead::pencil("2H").unwrap();
        let soft = Lead::pencil("HB").unwrap();
        let mut worn = 0.0;
        let left: Vec<(f32, f32)> = (0..=24).map(|i| {
            let y = -10.0 + i as f32 * (BASE_Y + 10.0) / 24.0;
            (axis(y) - half_w(y), y)
        }).collect();
        let right: Vec<(f32, f32)> = left.iter().map(|&(_, y)| (axis(y) + half_w(y), y)).collect();
        let horizon: Vec<(f32, f32)> = (0..=40).map(|i| (i as f32 * 25.0, far_snow(i as f32 * 25.0))).collect();
        let snowline: Vec<(f32, f32)> = (0..=10).map(|i| {
            let x = axis(BASE_Y) - 80.0 + i as f32 * 16.0;
            (x, contact(x))
        }).collect();
        for (k, pts) in [left, right, LIMB.to_vec(), STUB.to_vec(), horizon, snowline].iter().enumerate() {
            for m in sketch_marks(pts, 0.35, 2, 1.2, true, 0.4, o.seed + 40 + k as u64) {
                worn += c.draw(if k < 2 { &soft } else { &lead }, &m, worn, o.seed + k as u64);
            }
        }
    }

    if o.stage("sky", &mut c, &mut rng) {
        // thin lay-in, strokes across with the elbow, a shade duller than
        // the sky will end; then fused with the badger top to bottom
        let sky_pal = st.palette.only(&["lead white", "cobalt blue", "yellow ochre", "red earth", "raw umber"]);
        let lay = st
            .broad()
            .palette(&sky_pal)
            .color(move |x, y| mix(sky_col(x, y), hex("#8d8a86"), 0.06, Mix::Light))
            .angle(|x, y| 0.04 * ((x + y) / 300.0).sin())
            .coverage(4.0)
            .medium(0.4)
            .dips(1, 0.6, 0.6);
        c.work(&sky_m, &lay, 11);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.angle(|_, _| 0.0), 12);
        }
        // stipple into the wet lay-in with a ~1 mm tip, aimed at the sky
        let s1 = Stipple::new(Tool::stippler(4.2))
            .mixed(&sky_pal, 0.45)
            .color(sky_col)
            .coverage(|_, _| 2.0)
            .pressure(0.45, 0.85)
            .dips(20, 0.4, 0.5);
        c.stipple(&sky_m, &s1, 13);
        c.dry();
        // dry, a finer and lighter pass, thickening toward the glow
        let glow = move |x: f32, y: f32| mix(sky_col(x, y), hex("#f1dfb8"), 0.05 + 0.2 * smoothstep(420.0, 940.0, y), Mix::Light);
        let s2 = Stipple::new(Tool::stippler(2.4))
            .mixed(&sky_pal, 0.5)
            .color(glow)
            .coverage(|_, y| 0.4 + 1.8 * smoothstep(250.0, 930.0, y))
            .pressure(0.45, 0.85)
            .dips(24, 0.35, 0.6);
        c.stipple(&sky_m, &s2, 14);
        c.dry();
        // three thin streaks of evening cloud low in the glow, a little
        // grayer and rosier than the sky, stippled so they have no edges
        let streak_n = Fbm::new(55, 4, 1.0);
        let streaks = Mask::from_fn(f, move |x, y| {
            let mut v = 0.0f32;
            for &(yc, th, x0, x1) in &[(612.0f32, 10.0f32, 520.0f32, 1060.0f32), (668.0, 7.0, -40.0, 420.0), (705.0, 5.0, 610.0, 940.0)] {
                let wob = 6.0 * streak_n.get(x / 180.0, yc) + 2.0 * streak_n.get(x / 40.0, yc + 9.0);
                let d = (y - yc - wob - 0.02 * (x - x0)).abs();
                let t = th * (0.5 + 0.8 * (0.5 + 0.5 * streak_n.get(x / 90.0, yc + 3.0)));
                let along = smoothstep(x0, x0 + 90.0, x) * (1.0 - smoothstep(x1 - 120.0, x1, x));
                // thicker and thinner along its length, torn in places
                let body = smoothstep(-0.35, 0.3, streak_n.get(x / 60.0, yc + 17.0));
                let fl = if y < yc + wob + 0.02 * (x - x0) { 1.0 } else { 0.6 };
                v = v.max(smoothstep(t * fl + 4.0, 0.0, d) * along * body);
            }
            v
        });
        // a soft small filbert, thin paint, drawn along the streaks
        let cloud = paint::Handling::new(Tool { lay: 0.45, ragged: 0.5, stiffness: 0.35, ..Tool::filbert(8.0) })
            .mixed(&sky_pal, 0.6)
            .color_over(|_, _, u| shift(u, -0.03, 0.007, -0.004))
            .angle(|_, _| 0.02)
            .angle_jitter(0.03)
            .curve(0.03, 0.2)
            .length(40.0, 140.0)
            .coverage(1.8)
            .pressure(0.3, 0.55)
            .dips(2, 0.35, 0.5)
            .swell(0.3)
            .broken(0.25)
            .ramps(0.3, 0.5)
            .fill(false);
        c.work(&streaks, &cloud, 15);
        c.dry();
    }

    // far snow: bluer, in the evening shade; the field nearer is lighter
    // the snow's own relief: low drifts, long across and shallow, closer
    // together toward the horizon; lit where they rise away from us toward
    // the glow, shaded where they fall toward us
    let drift = Fbm::new(81, 4, 1.0);
    let snow_h = move |x: f32, y: f32| {
        let near = smoothstep(far_snow(x), h + 60.0, y);
        let s = 0.18 + 0.82 * near;
        drift.get(x / (260.0 * s), y / (48.0 * s * s + 6.0))
    };
    // the trunk's shadow, thrown toward us by the low sun behind it and
    // widening as it comes near, its edges wavering over the drifts: mixed
    // into the snow as it is laid, as a painter mixes shadowed snow
    let sh_n = Fbm::new(33, 3, 40.0);
    let shadow_fp = move |x: f32, y: f32| {
        let ax = axis(BASE_Y);
        let t = ((y - BASE_Y + 20.0) / (h - BASE_Y + 20.0)).clamp(0.0, 1.2);
        let cx = ax + 14.0 + 45.0 * t + 6.0 * sh_n.get(y * 0.3, 1.0);
        let hw = 44.0 + 50.0 * t + 7.0 * sh_n.get(x * 0.4, y * 1.5) + 5.0 * snow_h(x, y);
        let soft = 3.0 + 9.0 * t;
        smoothstep(hw + soft, hw - soft, (x - cx).abs()) * smoothstep(contact(x) - 30.0, contact(x) - 5.0, y)
    };
    let snow_col = move |x: f32, y: f32| {
        let t = smoothstep(far_snow(x), h, y);
        let base = gradient(&[(0.0, hex("#a9a8ae")), (0.2, hex("#b3b1b3")), (0.7, hex("#bab6b4")), (1.0, hex("#bfbab5"))], t, Mix::Light);
        // slope toward the horizon: rising away from us = lit by the glow
        let rise = (snow_h(x, y + 3.0) - snow_h(x, y - 3.0)) / 6.0;
        let near = smoothstep(far_snow(x), h, y);
        let k = (rise * (30.0 + 60.0 * near)).clamp(-1.0, 1.0);
        let lit = if k > 0.0 { shift(base, 0.045 * k, 0.001 * k, 0.01 * k) } else { shift(base, 0.04 * k, 0.0, 0.007 * k) };
        let sh = shadow_fp(x, y);
        // in the shadow the drifts still show, but only just
        mix(lit, shift(mix(base, lit, 0.35, Mix::Light), -0.13, 0.002, -0.022), sh, Mix::Light)
    };
    // far woods: two low copses on the snow, their tops ragged with bare
    // crowns, the right one farther and paler
    let far_band = Fbm::new(71, 4, 9.0);
    let far_slow = Fbm::new(72, 3, 70.0);
    let copse = |x: f32| {
        let a = smoothstep(120.0, 175.0, x) * (1.0 - smoothstep(290.0, 350.0, x));
        let b = 0.7 * smoothstep(650.0, 700.0, x) * (1.0 - smoothstep(820.0, 905.0, x));
        a.max(b)
    };
    let hedge_m = Mask::from_fn(f, move |x, y| {
        let env = copse(x);
        let crowns = (0.5 + 0.5 * far_band.get(x, 0.0)).powf(0.7) * (0.6 + 0.4 * far_slow.get(x, 5.0));
        let top = far_snow(x) + 1.0 - env * (3.0 + 16.0 * crowns);
        (env > 0.02) as u8 as f32 * smoothstep(top - 0.6, top + 0.6, y) * (1.0 - smoothstep(far_snow(x) + 2.0, far_snow(x) + 4.0, y))
    });

    if o.stage("snow", &mut c, &mut rng) {
        let snow_pal = st.palette.only(&["lead white", "cobalt blue", "yellow ochre", "red earth", "raw umber", "bone black"]);
        // the field: body color in strokes that lie along the surface,
        // longer far off, shorter and fuller near
        let lay = st
            .body()
            .palette(&snow_pal)
            .color(snow_col)
            .angle(|x, y| 0.08 * ((x / 140.0) + y / 90.0).sin())
            .length(30.0, 110.0)
            .coverage(3.5)
            .medium(0.15);
        c.work(&land_m, &lay, 21);
        if let Some(b) = st.blend() {
            let far = land_m.clone().mul_fn(|x, y| 1.0 - smoothstep(far_snow(x) + 60.0, far_snow(x) + 120.0, y));
            c.work(&far, &b.angle(|_, _| 0.0).pressure(0.3, 0.4), 22);
        }
        c.dry();
        // the far woods: a low broken line, stippled small and cool, into
        // dry snow (into the wet snow the touches picked up lead white and
        // went milky)
        let hedge = Stipple::new(Tool::stippler(1.6))
            .mixed(&snow_pal, 0.3)
            .color(|x, _| if x < 500.0 { hex("#5f606b") } else { hex("#7a7a84") })
            .coverage(|_, _| 3.2)
            .pressure(0.5, 0.8)
            .dips(16, 0.45, 0.6)
            .fade(0.0);
        c.stipple(&hedge_m, &hedge, 23);
        c.dry();
    }

    // ---- the tree
    let bark_n = Fbm::new(5, 4, 30.0);
    let trunk_m = Mask::from_fn(f, move |x, y| {
        let l = left_edge(y) - 2.0 * bark_n.get(y * 0.2, 3.0);
        let r = right_edge(y) + 2.0 * bark_n.get(y * 0.2, 9.0);
        smoothstep(l - 0.7, l + 0.7, x) * smoothstep(r + 0.7, r - 0.7, x) * (1.0 - smoothstep(contact(x) + 1.0, contact(x) + 4.0, y))
    })
    .roughen(9, 6.0, 1.1, 0.7);
    // the limb and the stub start at the trunk's edge (a painter paints
    // the limb out of the trunk, not across it)
    let limb_m = Mask::from_fn(f, |x, y| {
        let v = limb_cov(x, y, &LIMB, &LIMB_W).0 * smoothstep(right_edge(y) - 8.0, right_edge(y) - 3.0, x);
        let w = limb_cov(x, y, &STUB, &STUB_W).0.max(limb_cov(x, y, &DEAD, &DEAD_W).0) * smoothstep(left_edge(y) + 8.0, left_edge(y) + 3.0, x);
        // the crotch: the limb's wood swells into the trunk's, no corner
        let fx = (x - (right_edge(478.0) + 1.0)) / 12.0;
        let fy = (y - 478.0) / 26.0;
        let fil = smoothstep(1.05, 0.95, (fx * fx + fy * fy).sqrt()) * smoothstep(right_edge(y) - 8.0, right_edge(y) - 3.0, x);
        let sx = (x - (left_edge(722.0) + 1.0)) / 8.0;
        let sy = (y - 722.0) / 16.0;
        let sfil = smoothstep(1.05, 0.95, (sx * sx + sy * sy).sqrt()) * smoothstep(left_edge(y) + 8.0, left_edge(y) + 3.0, x);
        v.max(w).max(fil).max(sfil)
    })
    .roughen(10, 7.0, 0.9, 0.7);
    // the tree's own light: the evening sky behind and to the left; the
    // left edge takes a cool sky-rim, the right falls into the darkest dark
    let bark = move |x: f32, y: f32| {
        let u = ((x - axis(y)) / half_w(y)).clamp(-1.2, 1.2);
        let rim = smoothstep(-0.55, -0.95, u);
        let core = mix(hex("#2d2824"), hex("#1f1b18"), smoothstep(-0.2, 0.9, u), Mix::Pigment);
        let lich = 0.5 + 0.5 * bark_n.get(x * 0.5, y * 0.12);
        let c1 = mix(core, hex("#4b4d4f"), rim * 0.75, Mix::Pigment);
        mix(c1, hex("#4a4c3f"), 0.35 * smoothstep(0.62, 0.85, lich) * (1.0 - smoothstep(-0.2, 0.4, u)), Mix::Pigment)
    };

    if o.stage("trunk", &mut c, &mut rng) {
        let dark_pal = st.palette.only(&["lead white", "cobalt blue", "yellow ochre", "raw umber", "bone black", "red earth"]);
        // body color along the trunk, strokes following it up
        let body = st
            .body()
            .palette(&dark_pal)
            .color(bark)
            .angle(|_, _| FRAC_PI_2 + 0.055)
            .angle_jitter(0.08)
            .length(25.0, 80.0)
            .coverage(3.2)
            .medium(0.18)
            .clip(true);
        c.work(&trunk_m, &body, 31);
        let limb = st
            .body()
            .palette(&dark_pal)
            .color(|x, y| {
                let (v1, _, s1) = limb_side(x, y, &LIMB, &LIMB_W);
                let (v2, _, s2) = limb_side(x, y, &STUB, &STUB_W);
                let (v3, _, s3) = limb_side(x, y, &DEAD, &DEAD_W);
                let side = if v1 >= v2.max(v3) { s1 } else if v2 >= v3 { s2 } else { s3 };
                // the top takes the sky, the underside the dark
                let top = smoothstep(0.1, 0.85, side);
                let core = mix(hex("#2b2622"), hex("#1d1a17"), smoothstep(0.2, -0.8, side), Mix::Pigment);
                mix(core, hex("#474a4f"), 0.6 * top, Mix::Pigment)
            })
            .angle(|x, y| {
                let (v1, a1) = limb_cov(x, y, &LIMB, &LIMB_W);
                let (v2, a2) = limb_cov(x, y, &STUB, &STUB_W);
                let (v3, a3) = limb_cov(x, y, &DEAD, &DEAD_W);
                if v1 >= v2.max(v3) { a1 } else if v2 >= v3 { a2 } else { a3 }
            })
            .length(15.0, 50.0)
            .coverage(3.2)
            .medium(0.18)
            .clip(true);
        c.work(&limb_m, &limb, 32);
        c.wait(30.0);
    }

    if o.stage("bark", &mut c, &mut rng) {
        // fissures: the point of a round sable drawn up the trunk in broken,
        // wavering lines, darker than the body, into the still soft paint
        let mut r = Rng::new(o.seed + 301);
        let mut sab = Held::new(Tool { point: 1.0, ..Tool::round_sable(3.2) }, 301);
        let fiss = pal.paint(hex("#100e0d"), 0.12);
        let ridge = pal.paint(hex("#3a3632"), 0.12);
        let rim = pal.paint(hex("#6a6d72"), 0.1);
        for k in 0..220 {
            let u = r.range(-0.92, 0.92);
            let y0 = r.range(-20.0, BASE_Y - 10.0);
            let len = r.range(18.0, 70.0);
            let pts: Vec<(f32, f32)> = (0..6)
                .map(|i| {
                    let y = y0 - len * i as f32 / 5.0;
                    let uu = u + 0.06 * (y / 37.0 + k as f32).sin();
                    (axis(y) + uu * half_w(y), y)
                })
                .collect();
            let lighter = k % 3 == 2;
            sab.reload(if lighter { ridge } else { fiss }, 0.7);
            let p = if lighter { 0.28 } else { r.range(0.3, 0.6) };
            c.drag(&mut sab, &Gesture::new(pts).pressure(p, p * 0.3).ramps(0.1, 0.5).shake(0.7), Some(&trunk_m));
        }
        // oak bark is blocky: short cross cracks between the fissures
        let crack = pal.paint(hex("#191512"), 0.12);
        for _ in 0..260 {
            let y = r.range(0.0, BASE_Y - 20.0);
            let u = r.range(-0.9, 0.9);
            let x = axis(y) + u * half_w(y);
            let len = r.range(3.0, 9.0);
            let a = r.range(-0.5, 0.5);
            sab.reload(crack, 0.5);
            c.drag(&mut sab, &Gesture::new(vec![(x, y), (x + len * a.cos(), y + len * a.sin())]).pressure(0.3, 0.15).ramps(0.2, 0.5).shake(0.4), Some(&trunk_m));
        }
        c.wait(20.0);
        // lichen on the sky side: pale gray-green patches, stippled in clumps
        let lichen_n = Fbm::new(91, 4, 22.0);
        let lichen_m = trunk_m.clone().mul_fn(move |x, y| {
            let u = (x - axis(y)) / half_w(y);
            smoothstep(0.2, -0.7, u) * smoothstep(0.3, 0.5, lichen_n.get(x, y * 0.4)) * smoothstep(60.0, 300.0, y)
        });
        let dark_pal = st.palette.only(&["lead white", "cobalt blue", "yellow ochre", "raw umber", "bone black"]);
        let lichen = Stipple::new(Tool::stippler(1.5))
            .mixed(&dark_pal, 0.3)
            .color_over(|_, _, u| shift(u, 0.16, -0.012, 0.02))
            .coverage(|_, _| 0.9)
            .cluster(0.8, Some(4.0))
            .fade(0.0)
            .pressure(0.4, 0.8)
            .dips(12, 0.4, 0.6);
        c.stipple(&lichen_m, &lichen, 305);
        // the snow's light thrown back up onto the foot of the trunk
        let bounce_m = trunk_m.clone().mul_fn(|_, y| smoothstep(BASE_Y - 230.0, BASE_Y - 20.0, y));
        let bounce = Stipple::new(Tool::stippler(2.6))
            .mixed(&dark_pal, 0.45)
            .color_over(|_, _, u| shift(u, 0.07, -0.004, -0.012))
            .coverage(|_, y| 1.6 * smoothstep(BASE_Y - 230.0, BASE_Y - 20.0, y))
            .pressure(0.4, 0.8)
            .dips(12, 0.35, 0.6);
        c.stipple(&bounce_m, &bounce, 306);
        // the cool rim of sky light down the left edge, broken
        let mut rg = Held::new(Tool { point: 1.0, ..Tool::round_sable(2.4) }, 302);
        let mut y = 0.0;
        while y < BASE_Y - 60.0 {
            let len = r.range(30.0, 120.0);
            let pts: Vec<(f32, f32)> = (0..5)
                .map(|i| {
                    let yy = y + len * i as f32 / 4.0;
                    (axis(yy) - half_w(yy) + 3.5 + r.range(-0.6, 0.6), yy)
                })
                .collect();
            rg.reload(rim, 0.5);
            c.drag(&mut rg, &Gesture::new(pts).pressure(0.35, 0.15).ramps(0.2, 0.5).shake(0.5), Some(&trunk_m));
            y += len + r.range(10.0, 60.0);
        }
        // the break of the stub: torn splinters, one longer, and a little
        // pale split wood on the break
        let (ex, ey) = STUB[2];
        let (px, py) = STUB[1];
        let (dx, dy) = ((ex - px), (ey - py));
        let l = (dx * dx + dy * dy).sqrt();
        let (ux, uy) = (dx / l, dy / l);
        let (nx, ny) = (-uy, ux);
        let mut sp = Held::new(Tool { point: 1.0, ..Tool::round_sable(4.0) }, 303);
        for &(off, len, pr) in &[(-6.0f32, 7.0f32, 0.55f32), (-1.5, 16.0, 0.6), (3.5, 5.0, 0.5), (6.5, 9.0, 0.4)] {
            sp.reload(pal.paint(hex("#221e1b"), 0.12), 0.7);
            let a = (ex - ux * 4.0 + nx * off, ey - uy * 4.0 + ny * off);
            let b = (ex + ux * len + nx * off * 0.6, ey + uy * len + ny * off * 0.6);
            c.drag(&mut sp, &Gesture::new(vec![a, b]).pressure(pr, 0.0).ramps(0.05, 0.8).shake(0.4), None);
        }
        let mut w = Held::new(Tool { point: 1.0, ..Tool::round_sable(2.0) }, 304);
        w.load(pal.paint(hex("#7d725f"), 0.12), 0.5);
        c.drag(&mut w, &Gesture::new(vec![(ex + nx * 5.0, ey + ny * 5.0 - 1.0), (ex + ux * 3.0, ey + uy * 3.0 - 1.5), (ex - nx * 2.0 + ux * 6.0, ey - ny * 2.0 + uy * 6.0)]).pressure(0.45, 0.1).ramps(0.1, 0.6), None);
        c.wait(60.0);
    }

    if o.stage("twigs", &mut c, &mut rng) {
        // twigs off the limb and a few dead shoots off the trunk into the
        // sky: each pressed where it leaves and lifted off to its tip
        let mut r = Rng::new(o.seed + 401);
        let mut sab = Held::new(Tool { point: 1.0, ..Tool::round_sable(2.6) }, 401);
        let mut rig = Held::new(Tool { point: 1.0, ..Tool::rigger(1.0) }, 402);
        let twig_paint = pal.paint(hex("#2a2622"), 0.15);
        let mut stack: Vec<((f32, f32), f32, f32, f32)> = Vec::new();
        // off the limb, springing upward and outward
        for i in 1..LIMB.len() - 1 {
            let (x, y) = LIMB[i];
            stack.push(((x, y - LIMB_W[i] * 0.4), -1.35 + r.range(-0.3, 0.3), 70.0 + r.range(0.0, 80.0), 0.9));
            if r.f() < 0.6 {
                stack.push(((x + 20.0, y + LIMB_W[i] * 0.2), 0.4 + r.range(-0.2, 0.3), 50.0 + r.range(0.0, 50.0), 0.7));
            }
        }
        // off the dead branch: a few short dead twigs, stiff
        for i in 1..DEAD.len() - 1 {
            let (x, y) = DEAD[i];
            if r.f() < 0.7 {
                stack.push(((x, y - DEAD_W[i] * 0.4), -1.9 + r.range(-0.4, 0.4), 25.0 + r.range(0.0, 30.0), 0.7));
            }
        }
        // off the trunk: short epicormic shoots and two old dead branches
        for &(y, side, len) in &[(30.0f32, -1.0f32, 110.0f32), (300.0, -1.0, 90.0), (90.0, 1.0, 120.0), (255.0, 1.0, 75.0), (620.0, 1.0, 60.0), (860.0, -1.0, 45.0), (410.0, -1.0, 70.0)] {
            let x = axis(y) + side * (half_w(y) - 3.0);
            let a = if side < 0.0 { -2.5 } else { -0.7 } + r.range(-0.25, 0.25);
            stack.push(((x, y), a, len, 0.85));
        }
        let mut n = 0;
        while let Some((p, a, len, pr)) = stack.pop() {
            n += 1;
            if n > 400 {
                break;
            }
            let bend = r.range(-0.35, 0.35);
            let pts: Vec<(f32, f32)> = (0..5)
                .map(|i| {
                    let t = i as f32 / 4.0;
                    let aa = a + bend * t + 0.08 * (t * 9.0 + n as f32).sin();
                    (p.0 + aa.cos() * len * t, p.1 + aa.sin() * len * t)
                })
                .collect();
            let end = pts[4];
            let hb = if len > 55.0 { &mut sab } else { &mut rig };
            hb.reload(twig_paint, 0.8);
            c.drag(hb, &Gesture::new(pts.clone()).pressure(pr, 0.0).ramps(0.03, 0.75).shake(0.5), None);
            if len > 18.0 {
                let kids = if len > 50.0 { 3 } else { 2 };
                for j in 0..kids {
                    let fr = 0.35 + 0.55 * (j as f32 + r.f() * 0.6) / kids as f32;
                    let q = (p.0 + (end.0 - p.0) * fr, p.1 + (end.1 - p.1) * fr);
                    let side = if (j + n) % 2 == 0 { -1.0 } else { 1.0 };
                    // twigs of an oak: crooked, turning up
                    let na = a + side * r.range(0.35, 0.8) - 0.12;
                    stack.push((q, na, len * r.range(0.4, 0.62), pr * (1.0 - fr * 0.5) * 0.85));
                }
            }
        }
        c.dry();
    }

    if o.stage("foot", &mut c, &mut rng) {
        let snow_pal = st.palette.only(&["lead white", "cobalt blue", "yellow ochre", "red earth", "raw umber", "bone black"]);
        let _ = axis(BASE_Y);
        // where the trunk goes into the snow: a crease of shade right at
        // the bark, then the snow's lip pushed up against the dark and over
        // it in places, lead white from a small round; the drift on the
        // windward left piled higher
        let _ = &snow_pal;
        let mut r0 = Rng::new(o.seed + 510);
        let mut lipb = Held::new(Tool { point: 1.0, ..Tool::round_sable(4.5) }, 510);
        let lx0 = left_edge(BASE_Y) - 6.0;
        let lx1 = right_edge(BASE_Y) + 8.0;
        let crease = pal.paint(hex("#8f8e96"), 0.15);
        let lip = [pal.paint(hex("#c6c1ba"), 0.18), pal.paint(hex("#bcb8b4"), 0.2), pal.paint(hex("#cec8bf"), 0.15)];
        let mut x = lx0;
        while x < lx1 {
            let len = r0.range(10.0, 26.0);
            let pts: Vec<(f32, f32)> = (0..4).map(|i| { let xx = x + len * i as f32 / 3.0; (xx, contact(xx) + 5.5) }).collect();
            lipb.reload(crease, 0.5);
            c.drag(&mut lipb, &Gesture::new(pts).pressure(0.4, 0.3).ramps(0.2, 0.3).shake(0.5), None);
            x += len * r0.range(0.7, 1.0);
        }
        let mut x = lx0 - 8.0;
        while x < lx1 + 6.0 {
            let len = r0.range(6.0, 18.0);
            let up = r0.range(-1.0, 3.5);
            let pts: Vec<(f32, f32)> = (0..4).map(|i| { let xx = x + len * i as f32 / 3.0; (xx, contact(xx) + 1.5 - up * (1.0 - (2.0 * i as f32 / 3.0 - 1.0).powi(2))) }).collect();
            if r0.f() < 0.3 {
                x += len;
                continue;
            }
            lipb.reload(lip[(r0.f() * 2.999) as usize], 0.35);
            c.drag(&mut lipb, &Gesture::new(pts).pressure(r0.range(0.3, 0.55), 0.15).ramps(0.3, 0.5).shake(0.8), None);
            x += len * r0.range(0.6, 1.1);
        }
        // the drift on the left: a few rounded strokes swelling against the bark
        for k in 0..7 {
            let x = lx0 - 30.0 + k as f32 * 9.0 + r0.range(-3.0, 3.0);
            let y = contact(x) + 8.0 + r0.range(-3.0, 3.0);
            let pts = vec![(x - 14.0, y + 4.0), (x, y - 3.0 - k as f32 * 0.8), (x + 16.0, y - 1.0)];
            lipb.reload(lip[k % 3], 0.6);
            c.drag(&mut lipb, &Gesture::new(pts).pressure(0.55, 0.3).ramps(0.3, 0.4).shake(0.6), None);
        }
        let ax = axis(BASE_Y);
        // the snow crust on top of the limb and the stub: a broken line of
        // white laid with the side of a small round
        let mut r = Rng::new(o.seed + 501);
        let mut sab = Held::new(Tool { point: 1.0, ..Tool::round_sable(3.0) }, 501);
        let crust = pal.paint(hex("#cfcbc4"), 0.1);
        for (pts, ws) in [(&LIMB[..], &LIMB_W[..]), (&STUB[..], &STUB_W[..]), (&DEAD[..], &DEAD_W[..])] {
            for i in 0..pts.len() - 1 {
                if i == 0 && pts.len() > 3 {
                    continue;
                }
                let (a, b) = (pts[i], pts[i + 1]);
                for _lump in 0..3 {
                if r.f() < 0.35 {
                    continue;
                }
                let (w0, w1) = (ws[i], ws[i + 1]);
                let ang = (b.1 - a.1).atan2(b.0 - a.0);
                let (nx, ny) = (ang.sin(), -ang.cos()); // up side
                let t0 = r.range(0.0, 0.8);
                let t1 = (t0 + r.range(0.08, 0.25)).min(1.0);
                let pt = |t: f32| {
                    let hw = 0.5 * (w0 + (w1 - w0) * t) - 0.6;
                    (a.0 + (b.0 - a.0) * t + nx * hw, a.1 + (b.1 - a.1) * t + ny * hw)
                };
                sab.reload(crust, 0.8);
                c.drag(&mut sab, &Gesture::new(vec![pt(t0), pt((t0 + t1) * 0.5), pt(t1)]).pressure(0.62, 0.3).ramps(0.3, 0.4).shake(0.8), None);
                }
            }
        }
        // snow caught in the bark's hollows on the windward side, low down
        let mut tp = Held::new(Tool { point: 1.0, ..Tool::round_sable(2.2) }, 502);
        for _ in 0..0 {
            let y = r.range(1000.0, contact(ax - 40.0));
            let u = r.range(-1.0, -0.45);
            let x = axis(y) + u * half_w(y);
            tp.reload(crust, 0.5);
            c.drag(&mut tp, &Gesture::new(vec![(x, y), (x + r.range(2.0, 6.0), y - r.range(3.0, 9.0))]).pressure(0.45, 0.1).ramps(0.1, 0.6).shake(0.5), Some(&trunk_m));
        }
        c.wait(40.0);
    }

    if o.stage("shadow", &mut c, &mut rng) {
        let snow_pal = st.palette.only(&["lead white", "cobalt blue", "yellow ochre", "red earth", "raw umber", "bone black"]);
        let ax = axis(BASE_Y);
        // the scoop in the lee of the trunk: a bluer hollow
        let scoop_m = Mask::from_fn(f, move |x, y| {
            let dx = (x - (ax + 62.0)) / 46.0;
            let dy = (y - (contact(x) + 10.0)) / 10.0;
            smoothstep(1.0, 0.5, (dx * dx + dy * dy).sqrt())
        });
        let scoop = Stipple::new(Tool::stippler(3.0))
            .mixed(&snow_pal, 0.4)
            .color_over(|_, _, u| shift(u, -0.05, 0.0, -0.014))
            .coverage(|_, _| 2.0)
            .dips(14, 0.4, 0.6);
        c.stipple(&scoop_m, &scoop, 62);
        c.dry();
        // the snow's modeling near: long low drift crests catching the
        // glow, each with its furrow of shade below, laid with the side of a
        // small round, stiff paint, a little raised
        let mut r = Rng::new(o.seed + 601);
        let mut sab = Held::new(Tool { point: 1.0, ..Tool::round_sable(4.0) }, 601);
        let lit = pal.paint(hex("#c8c2b9"), 0.15);
        let lit2 = pal.paint(hex("#c1bcb5"), 0.18);
        let furrow = pal.paint(hex("#a8a7ac"), 0.25);
        let off_tree = Mask::from_fn(f, |_, _| 1.0).subtract(&trunk_m.dilate(1.5));
        for k in 0..30 {
            let y = BASE_Y - 60.0 + (h - BASE_Y + 60.0) * (k as f32 / 30.0).powf(0.8) + r.range(-6.0, 6.0);
            let x0 = r.range(-60.0, 1000.0);
            let near = smoothstep(far_snow(x0), h, y);
            let len = (40.0 + 140.0 * near) * r.range(0.6, 1.3);
            let sag = r.range(-0.08, 0.08) * len;
            let tilt = r.range(-0.05, 0.05);
            let pts: Vec<(f32, f32)> = (0..5)
                .map(|i| {
                    let t = i as f32 / 4.0;
                    (x0 + len * t, y + tilt * len * t + sag * (4.0 * t * (1.0 - t)))
                })
                .collect();
            if shadow_fp(x0 + len * 0.5, y) > 0.3 || y < far_snow(x0) + 12.0 {
                continue;
            }
            let p = 0.45 + 0.4 * near;
            sab.reload(if k % 3 == 0 { lit } else { lit2 }, 0.6 + 0.3 * near);
            c.drag(&mut sab, &Gesture::new(pts.clone()).pressure(p, p * 0.4).ramps(0.3, 0.5).shake(0.6), Some(&off_tree));
            let dn = 2.5 + 5.0 * near;
            let under: Vec<(f32, f32)> = pts.iter().skip(1).map(|&(x, y)| (x + 4.0, y + dn)).collect();
            sab.reload(furrow, 0.35);
            c.drag(&mut sab, &Gesture::new(under).pressure(p * 0.7, p * 0.2).ramps(0.3, 0.6).shake(0.6), Some(&off_tree));
        }
        // the far snow's edge takes the glow: a thin warm line along it
        let mut rg = Held::new(Tool { point: 1.0, ..Tool::round_sable(2.0) }, 602);
        let mut x = -10.0;
        while x < 1010.0 {
            let len = r.range(30.0, 110.0);
            if copse(x + len * 0.5) < 0.1 && (x + len < axis(960.0) - 70.0 || x > axis(960.0) + 50.0) {
                let pts: Vec<(f32, f32)> = (0..5).map(|i| { let xx = x + len * i as f32 / 4.0; (xx, far_snow(xx) + 1.2) }).collect();
                rg.reload(pal.paint(hex("#dccfbd"), 0.1), 0.5);
                c.drag(&mut rg, &Gesture::new(pts).pressure(0.35, 0.2).ramps(0.2, 0.3).shake(0.3), None);
            }
            x += len + r.range(5.0, 40.0);
        }
        c.wait(30.0);
    }

    if o.stage("grass", &mut c, &mut rng) {
        // dry stalks through the snow, fine upturning strokes laid last
        // over the finished snow [NG p.56]
        let mut r = Rng::new(o.seed + 701);
        let mut rig = Held::new(Tool { point: 1.0, ..Tool::rigger(0.9) }, 701);
        let stalk = [pal.paint(hex("#5c4e3a"), 0.12), pal.paint(hex("#433a2e"), 0.12), pal.paint(hex("#77674e"), 0.1)];
        let ax = axis(BASE_Y);
        let clumps: Vec<(f32, f32, usize)> = vec![(ax - 95.0, 0.0, 18), (ax + 110.0, 0.0, 12), (ax - 150.0, 0.0, 5), (170.0, 1240.0, 18), (760.0, 1210.0, 14), (880.0, 1290.0, 12), (80.0, 1090.0, 7), (600.0, 1060.0, 6), (330.0, 1300.0, 9)];
        for &(cx, cy, n) in &clumps {
            let cy = if cy == 0.0 { contact(cx) + 6.0 } else { cy };
            let scale = 0.45 + 0.75 * smoothstep(1000.0, 1320.0, cy);
            for _ in 0..n {
                let x = cx + r.range(-18.0, 18.0) * scale;
                let y = cy + r.range(-4.0, 4.0);
                let ht = r.range(18.0, 70.0) * scale;
                let lean = r.range(-0.45, 0.35);
                rig.reload(stalk[(r.f() * 2.999) as usize], 0.6);
                let pts = vec![(x, y), (x + lean * ht * 0.25, y - ht * 0.5), (x + lean * ht, y - ht)];
                c.drag(&mut rig, &Gesture::new(pts).pressure(0.75, 0.0).ramps(0.04, 0.85).shake(0.5), None);
                if r.f() < 0.25 {
                    // a seed head: a touch at the tip
                    rig.reload(stalk[1], 0.5);
                    c.touch(&mut rig, &Touch::at(x + lean * ht, y - ht).pressure(0.6), None);
                }
            }
        }
        let _: &Paint = &stalk[0];
    }

    o.end(&mut c, &mut rng);
    // no varnish, no cracks: the paint as it left the easel
    c.dry();
    c.relief(st.relief.0, st.relief.1);
    o.save(&mut c);
}
