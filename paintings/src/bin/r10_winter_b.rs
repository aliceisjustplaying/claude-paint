//! Winter evening by a frozen pond (r10, arm 2, painter b).
//!
//! A flat snowfield under the afterglow of a winter sunset. A stag-headed
//! oak stands on a low rise on the left, black against the glow, snow
//! lying along the tops of its limbs; three young spruces heavy with snow
//! on the right bank; a frozen pond between them catching the last light;
//! far off, a low blue line of hills with a village and its church spire
//! just showing in the haze; a fir wood dark on the right horizon. One man
//! in a dark coat walks the path along the pond toward the church. A thin
//! crescent moon and the evening star. Dry grass and reeds stand through
//! the snow in the foreground, laid last.
//!
//!   cargo paint r10_winter_b
//!   cargo paint r10_winter_b -- --full
//!   cargo paint r10_winter_b -- --full --crop 100,230,420,620

use paint::color::{Mix, mix};
use paint::fir::{self, Fir, FirHabit};
use paint::{Gesture, Habit, Held, Mask, Palette, Paint, Rgb, Rng, Shape, Stipple, Style, Tool, Touch, hex, smoothstep};
use paint::Fbm;
use paintings::run::{Finish, Run};
use std::f32::consts::FRAC_PI_2;

const ASPECT: f32 = 1000.0 / 700.0;
/// The far horizon: where the snowfield meets the far hills.
const HZ: f32 = 452.0;
/// Where the glow of the set sun is strongest on the horizon.
const GLOW_X: f32 = 395.0;

fn sky_col(x: f32, y: f32) -> Rgb {
    let t = (y / HZ).clamp(0.0, 1.0);
    let base = paint::gradient(
        &[
            (0.0, hex("#4f5a78")),
            (0.3, hex("#6f7894")),
            (0.55, hex("#9c9aae")),
            (0.74, hex("#c3b1b0")),
            (0.88, hex("#dfc6a6")),
            (1.0, hex("#ecdcb6")),
        ],
        t,
        Mix::Light,
    );
    // the glow: warm and strong over the sun's place, cooling off to the sides
    let dx = (x - GLOW_X) / 420.0;
    let g = (-(dx * dx)).exp() * smoothstep(0.45, 1.0, t);
    let side = mix(base, hex("#b7adb6"), 0.35 * (1.0 - (-(dx * dx)).exp()) * smoothstep(0.6, 1.0, t), Mix::Light);
    mix(side, hex("#f0d9a8"), 0.35 * g, Mix::Light)
}

/// Top of the far hills (units), low and long, a little higher on the left.
fn hills_top(n: &Fbm, x: f32) -> f32 {
    let swell = 9.0 * (-((x - 170.0) / 260.0).powi(2)).exp() + 5.0 * (-((x - 560.0) / 180.0).powi(2)).exp();
    HZ - 3.0 - swell - 4.0 * n.get01(x * 0.8, 0.5) - 1.2 * n.get(x * 5.0, 3.0)
}

/// Skyline of the far fir wood on the right: spiky tops.
fn wood_top(n: &Fbm, x: f32) -> f32 {
    if x < 640.0 {
        return 1e9;
    }
    let ramp = smoothstep(640.0, 720.0, x);
    let base = HZ - 2.0 - ramp * (18.0 + 8.0 * n.get01(x * 0.5, 1.0));
    // each tree a small spire
    let cell = 7.5;
    let k = (x / cell).floor();
    let f = x / cell - k;
    let hgt = 6.0 + 9.0 * ((k * 12.9898).sin() * 43758.545).fract().abs();
    let spire = hgt * (1.0 - (2.0 * f - 1.0).abs()).powf(1.6);
    base - ramp * spire
}

/// The near ground's edge seen against the far snowfield: the low rise
/// under the oak on the left.
fn rise_top(x: f32) -> f32 {
    605.0 - 48.0 * (-((x - 175.0) / 170.0).powi(2)).exp() - 10.0 * (-((x - 20.0) / 90.0).powi(2)).exp()
}

/// The pond's outline.
fn pond_shape() -> Shape {
    Shape::new().smooth_poly(&[
        (300.0, 536.0),
        (380.0, 527.0),
        (470.0, 523.0),
        (575.0, 525.0),
        (680.0, 531.0),
        (748.0, 541.0),
        (730.0, 556.0),
        (640.0, 565.0),
        (520.0, 568.0),
        (410.0, 564.0),
        (330.0, 556.0),
        (292.0, 546.0),
    ])
}

/// Snow light: the snow takes the sky's color, facing up to the cool zenith
/// and, where it tilts toward the glow, the warm horizon.
fn snow_col(n: &Fbm, x: f32, y: f32) -> Rgb {
    let depth = smoothstep(HZ, 700.0, y); // 0 far, 1 near
    let far = mix(hex("#cdc9cf"), hex("#dfd3c3"), (-(((x - GLOW_X) / 300.0).powi(2))).exp(), Mix::Light);
    let near = hex("#b0b3c6");
    let base = mix(far, near, depth.powf(0.8), Mix::Light);
    // drifts: slow undulation, lit crests toward the glow, blue troughs
    let d = n.get(x * 0.35, y * 2.6);
    let crest = smoothstep(0.05, 0.45, d) * depth;
    let trough = smoothstep(0.0, -0.45, d) * depth;
    let lit = mix(base, hex("#e3dcd4"), 0.45 * crest, Mix::Light);
    mix(lit, hex("#959bb3"), 0.3 * trough, Mix::Light)
}

fn main() {
    let o = Run::new("r10_winter_b");
    let st = Style { palette: Palette::friedrich_early_greens(), ..Style::friedrich() };
    let pal = &st.palette;
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = c.height();
    let n = Fbm::new(7, 4, 60.0);
    let n2 = Fbm::new(19, 3, 18.0);

    // ------------------------------------------------------------- sky
    let sky = Mask::from_fn(f, |_, y| if y < HZ + 6.0 { 1.0 } else { 0.0 });
    if o.stage("sky", &mut c, &mut rng) {
        // thin underpainting, level arcs, fused top to bottom
        let hd = st.broad().color(sky_col).angle(|_, _| 0.0).angle_jitter(0.08).length(50.0, 160.0).coverage(4.0).medium(0.35).clip(true);
        c.work(&sky, &hd, 11);
        if let Some(b) = st.blend() {
            c.work(&sky, &b.angle(|_, _| 0.0), 12);
        }
        c.dry();
        // stipple: the sky again in small touches, finer toward the glow
        let s1 = Stipple::new(Tool::stippler(2.6)).mixed(pal, 0.45).color(sky_col).coverage(|_, _| 1.8).pressure(0.45, 0.8).dips(20, 0.4, 0.5).cluster(0.25, None).clip(true);
        c.stipple(&sky, &s1, 13);
        c.dry();
        let low = Mask::from_fn(f, |_, y| smoothstep(HZ * 0.45, HZ * 0.8, y) * if y < HZ + 6.0 { 1.0 } else { 0.0 });
        let glow = |x: f32, y: f32| mix(sky_col(x, y), hex("#f3e2bb"), 0.25, Mix::Light);
        let s2 = Stipple::new(Tool::stippler(1.6)).mixed(pal, 0.5).color(glow).coverage(|_, _| 1.4).pressure(0.4, 0.75).dips(24, 0.35, 0.6).clip(true);
        c.stipple(&low, &s2, 14);
        c.dry();
        // long thin streaks of cloud low over the glow, gray-violet against
        // it, warmer underneath where the set sun still reaches
        let wisp = Fbm::new(23, 4, 90.0);
        let streaks = Mask::from_fn(f, move |x, y| {
            let band = smoothstep(250.0, 310.0, y) * smoothstep(425.0, 380.0, y);
            let breaks = smoothstep(0.35, 0.6, wisp.get01(x * 0.02 + 50.0, y * 0.05));
            band * breaks * smoothstep(0.64, 0.82, wisp.get01(x * 0.1, y * 2.6))
        });
        let cloud = move |x: f32, y: f32| {
            let under = smoothstep(0.62, 0.78, wisp.get01(x * 0.1, (y + 3.0) * 2.6));
            mix(mix(sky_col(x, y), hex("#a098a6"), 0.4, Mix::Light), hex("#d8bfae"), 0.25 * (1.0 - under), Mix::Light)
        };
        let hd = st.body().color(cloud).angle(|_, _| -0.015).angle_jitter(0.03).length(30.0, 110.0).coverage(1.8).medium(0.4).pressure(0.25, 0.5).clip(true).threshold(0.3);
        c.work(&streaks, &hd, 15);
        if let Some(b) = st.blend() {
            c.work(&streaks.clone().dilate(4.0).blur(3.0), &b.angle(|_, _| 0.0).length(20.0, 60.0), 16);
        }
        c.dry();
    }

    // ------------------------------------------------- moon and evening star
    if o.stage("moon", &mut c, &mut rng) {
        // a thin crescent, lit from the set sun below left
        let (mx, my, r) = (668.0f32, 118.0f32, 9.5f32);
        let moon = Mask::from_fn(f, |x, y| {
            let a = ((x - mx).powi(2) + (y - my).powi(2)).sqrt();
            let b = ((x - mx - 3.6).powi(2) + (y - my + 3.0).powi(2)).sqrt();
            smoothstep(r + 0.4, r - 0.4, a) * smoothstep(r - 0.6, r + 0.6, b)
        });
        let mut b = Held::new(Tool { point: 0.7, ..Tool::round_sable(1.8) }, 21);
        let lead = pal.paint(hex("#f2ead0"), 0.1);
        // strokes following the crescent's curve, pressed in the middle
        for k in 0..5 {
            b.reload(lead, 0.9);
            let off = k as f32 * 0.55;
            let pts: Vec<(f32, f32)> = (0..=12)
                .map(|i| {
                    let a = 0.6 + (i as f32 / 12.0) * 2.9; // from right-bottom around the lower left to the top
                    let rr = r - 0.9 - off;
                    (mx + rr * a.cos(), my + rr * a.sin())
                })
                .collect();
            c.drag(&mut b, &Gesture::new(pts).pressure(0.15, 0.1).swell(vec![0.6, 1.4, 1.8, 1.4, 0.6]).ramps(0.2, 0.3).shake(0.2), Some(&moon.clone().dilate(0.6)));
        }
        // a faint halo, stippled
        let halo = Mask::from_fn(f, |x, y| {
            let d = ((x - mx).powi(2) + (y - my).powi(2)).sqrt();
            smoothstep(34.0, 12.0, d) * smoothstep(r - 1.0, r + 1.5, d)
        });
        let hs = Stipple::new(Tool::stippler(1.5)).mixed(pal, 0.6).color(|x, y| mix(sky_col(x, y), hex("#c9c6c6"), 0.3, Mix::Light)).coverage(|_, _| 1.0).pressure(0.35, 0.6).dips(20, 0.3, 0.6).clip(true);
        c.stipple(&halo, &hs, 22);
        // Venus, low over the glow
        let mut s = Held::new(Tool::round_sable(1.6), 23);
        s.load(pal.paint(hex("#fbf4df"), 0.05), 0.8);
        c.touch(&mut s, &Touch::at(548.0, 262.0).pressure(0.55), None);
        c.dry();
    }

    // ------------------------------------------ far hills, village, fir wood
    let far = Mask::from_fn(f, |x, y| if y > hills_top(&n, x) && y < HZ + 3.0 { 1.0 } else { 0.0 }).blur(0.6);
    let far_col = move |x: f32, y: f32| {
        let t = smoothstep(HZ - 18.0, HZ + 2.0, y);
        let hill = mix(hex("#7f8499"), hex("#9b98a6"), t, Mix::Light);
        mix(hill, sky_col(x, HZ - 4.0), 0.25, Mix::Light)
    };
    if o.stage("far", &mut c, &mut rng) {
        let hd = st.body().color(far_col).angle(|_, _| 0.0).angle_jitter(0.15).length(10.0, 36.0).coverage(3.0).pressure(0.45, 0.7).medium(0.3).clip(true).threshold(0.3);
        c.work(&far, &hd, 31);
        let sp = Stipple::new(Tool::stippler(1.4)).mixed(pal, 0.45).color(far_col).coverage(|_, _| 1.6).pressure(0.4, 0.7).dips(16, 0.35, 0.6).clip(true);
        c.stipple(&far, &sp, 32);
        c.dry();
        // the village: roofs and the church, a shade darker than the hills
        let town = hex("#6f7288");
        let mut b = Held::new(Tool { point: 0.8, ..Tool::round_sable(1.4) }, 33);
        let roofs = [(258.0f32, 4.0f32, 3.2f32), (268.0, 5.0, 4.0), (281.0, 3.5, 3.0), (318.0, 5.0, 3.6), (330.0, 4.0, 3.0), (341.0, 3.0, 2.4)];
        let ground_y = |x: f32| hills_top(&n, x) + 3.0;
        for &(x, hw, ht) in &roofs {
            b.reload(pal.paint(town, 0.2), 0.7);
            let gy = ground_y(x) + 1.0;
            // gable: up one side, down the other, then the wall filled in
            c.drag(&mut b, &Gesture::new(vec![(x - hw, gy - ht * 0.5), (x, gy - ht - 1.5), (x + hw, gy - ht * 0.5)]).pressure(0.5, 0.5).shake(0.3), None);
            c.drag(&mut b, &Gesture::line((x - hw, gy - ht * 0.4), (x + hw, gy - ht * 0.4)).pressure(0.7, 0.7).shake(0.3), None);
            c.drag(&mut b, &Gesture::line((x - hw + 0.5, gy - 0.5), (x + hw - 0.5, gy - 0.5)).pressure(0.7, 0.7).shake(0.3), None);
        }
        // the church: nave and tower with a tall spire
        let (cx, cy) = (300.0f32, ground_y(300.0) + 1.0);
        b.reload(pal.paint(town, 0.2), 0.9);
        for k in 0..4 {
            let yy = cy - 1.0 - k as f32 * 1.4;
            c.drag(&mut b, &Gesture::line((cx - 1.5, yy), (cx + 11.0, yy)).pressure(0.7, 0.7).shake(0.3), None);
        }
        b.reload(pal.paint(town, 0.2), 0.9);
        for k in 0..3 {
            let xx = cx - 1.5 + k as f32 * 1.1;
            c.drag(&mut b, &Gesture::line((xx, cy), (xx, cy - 13.0)).pressure(0.7, 0.7).shake(0.3), None);
        }
        b.reload(pal.paint(town, 0.2), 0.9);
        c.drag(&mut b, &Gesture::new(vec![(cx - 1.2, cy - 12.5), (cx - 0.2, cy - 20.0), (cx - 0.3, cy - 26.0)]).pressure(0.7, 0.0).ramps(0.05, 0.7).shake(0.2), None);
        c.drag(&mut b, &Gesture::new(vec![(cx + 1.2, cy - 12.5), (cx + 0.2, cy - 20.0), (cx - 0.3, cy - 26.0)]).pressure(0.7, 0.0).ramps(0.05, 0.7).shake(0.2), None);
        c.dry();

        // the fir wood on the right horizon: short vertical hatching, dark
        // and blue with distance
        let wood = Mask::from_fn(f, |x, y| if y > wood_top(&n, x) && y < HZ + 4.0 { 1.0 } else { 0.0 }).blur(0.5);
        let wood_col = |x: f32, y: f32| {
            let t = smoothstep(HZ - 30.0, HZ + 2.0, y);
            let d = mix(hex("#454a5c"), hex("#5c6072"), t, Mix::Light);
            mix(d, sky_col(x, HZ - 10.0), 0.12, Mix::Light)
        };
        let hd = st.hatch().color(wood_col).angle(|_, _| FRAC_PI_2).angle_jitter(0.2).length(3.0, 9.0).coverage(3.2).medium(0.25).clip(true).threshold(0.3);
        c.work(&wood, &hd, 34);
        // the spires of the tree tops written with the point
        let mut r = Held::new(Tool { point: 1.0, ..Tool::rigger(0.7) }, 35);
        let mut x = 700.0;
        while x < 1000.0 {
            let top = wood_top(&n, x);
            r.reload(pal.paint(wood_col(x, top), 0.2), 0.6);
            c.drag(&mut r, &Gesture::line((x, top + 7.0), (x + 0.2, top - 1.5)).pressure(0.6, 0.05).ramps(0.05, 0.6).shake(0.4), None);
            x += 3.0 + 6.0 * rng.f();
        }
        c.dry();
    }

    // ------------------------------------------------------- the snowfield
    let pond = Mask::from_shape(f, pond_shape()).roughen(41, 40.0, 1.6, 1.0);
    let land = Mask::from_fn(f, |_, y| if y >= HZ - 1.0 { 1.0 } else { 0.0 }).subtract(&far).subtract(&pond);
    if o.stage("snow", &mut c, &mut rng) {
        // a warm-gray underpainting of the far field, then snow in level
        // strokes, bowed into the drifts in front
        let snow = move |x: f32, y: f32| snow_col(&n, x, y);
        let dir = move |x: f32, y: f32| 0.25 * n.get(x * 0.3 + 40.0, y * 0.9) * smoothstep(HZ + 30.0, 650.0, y);
        let hd = st.broad().color(snow).angle(dir).angle_jitter(0.06).length(40.0, 130.0).coverage(4.0).medium(0.2).clip(true);
        c.work(&land, &hd, 41);
        if let Some(b) = st.blend() {
            c.work(&land, &b.angle(|_, _| 0.0), 42);
        }
        c.dry();
        // body of the snow in stiffer lead white: the drifts' lit faces
        let near = land.clone().mul_fn(|_, y| smoothstep(HZ + 20.0, 620.0, y));
        let hd = st.body().color(snow).angle(dir).angle_jitter(0.1).length(14.0, 50.0).coverage(2.6).medium(0.12).pressure(0.5, 0.85).clip(true).threshold(0.2);
        c.work(&near, &hd, 43);
        // the far field stippled, so it lies flat and holds light
        let farsnow = land.clone().mul_fn(|_, y| 1.0 - smoothstep(HZ + 15.0, HZ + 70.0, y));
        let sp = Stipple::new(Tool::stippler(1.6)).mixed(pal, 0.45).color(snow).coverage(|_, _| 1.5).pressure(0.4, 0.75).drag(1.0, Some(0.0)).dips(18, 0.35, 0.6).clip(true);
        c.stipple(&farsnow, &sp, 44);
        c.dry();
        // the rise under the oak: its crest against the far field, a cool
        // shadow on its far side, lit toward the glow
        let rise = Mask::from_fn(f, |x, y| smoothstep(rise_top(x) - 0.8, rise_top(x) + 0.8, y) * if x < 420.0 { 1.0 } else { 0.0 }).mul_fn(|x, _| 1.0 - smoothstep(330.0, 420.0, x));
        let rise_col = move |x: f32, y: f32| {
            let d = y - rise_top(x);
            let base = snow_col(&n, x, y);
            let crest = mix(base, hex("#e6dccd"), 0.5 * smoothstep(12.0, 0.0, d), Mix::Light);
            mix(crest, hex("#9fa3ba"), 0.3 * smoothstep(0.0, 60.0, d) * smoothstep(280.0, 60.0, x), Mix::Light)
        };
        let hd = st.body().color(rise_col).angle(move |x, _| -0.25 * ((x - 175.0) / 170.0).clamp(-1.0, 1.0)).angle_jitter(0.12).length(12.0, 40.0).coverage(2.8).medium(0.15).clip(true).threshold(0.2);
        c.work(&rise, &hd, 45);
        c.dry();
        // blue shadow troughs of the near drifts, glazed thin
        let trough = land.clone().mul_fn(move |x, y| smoothstep(0.0, -0.4, n.get(x * 0.35, y * 2.6)) * smoothstep(HZ + 60.0, 650.0, y));
        let hd = st.body().color_over(|_, _, u| paint::shift(u, -0.035, 0.0, -0.018)).angle(dir).angle_jitter(0.1).length(16.0, 50.0).coverage(1.8).medium(0.35).pressure(0.4, 0.7).clip(true).threshold(0.25);
        c.work(&trough, &hd, 46);
        c.dry();
    }

    // ------------------------------------------------------- the frozen pond
    if o.stage("pond", &mut c, &mut rng) {
        // ice: a dull mirror of the glow, grayer near, with drifted snow
        let ice = move |x: f32, y: f32| {
            let t = smoothstep(522.0, 568.0, y);
            // nearer ice mirrors higher, cooler sky
            let sky = sky_col(x, HZ - 50.0 - 190.0 * t);
            let ice = mix(sky, hex("#8e94a8"), 0.4 + 0.25 * t, Mix::Light);
            mix(ice, hex("#c4bfc2"), 0.35 * n2.get01(x * 0.25, y * 3.0).powi(2), Mix::Light)
        };
        let hd = st.body().color(ice).angle(|_, _| 0.0).angle_jitter(0.03).length(20.0, 70.0).coverage(3.4).medium(0.25).clip(true).threshold(0.3);
        c.work(&pond, &hd, 51);
        if let Some(b) = st.blend() {
            c.work(&pond, &b.angle(|_, _| 0.0).length(20.0, 60.0), 52);
        }
        c.dry();
        // wind-laid snow streaks across the ice
        let streak = pond.clone().mul_fn(move |x, y| smoothstep(0.55, 0.8, n2.get01(x * 0.15, y * 4.0)));
        let hd = paint::Handling::new(Tool::round_sable(2.4)).mixed(pal, 0.15).color(move |x, y| snow_col(&n, x, y)).angle(|_, _| -0.03).angle_jitter(0.04).length(8.0, 30.0).coverage(1.4).pressure(0.3, 0.6).clip(true).threshold(0.4);
        c.work(&streak, &hd, 53);
        // under the far bank the ice lies in the snow's shadow: a cool band
        let under = pond.rim(3.5, 2.5).mul_fn(|_, y| smoothstep(546.0, 532.0, y));
        let hd = paint::Handling::new(Tool::round_sable(2.6)).mixed(pal, 0.3).color_over(|_, _, u| paint::shift(u, -0.07, -0.004, -0.02)).angle(|_, _| 0.0).angle_jitter(0.05).length(10.0, 30.0).coverage(1.6).pressure(0.35, 0.6).clip(true).threshold(0.3);
        c.work(&under, &hd, 55);
        // the snow's lip over the ice's edge, broken, level strokes
        let lip = pond.rim(2.5, 1.5).mul_fn(move |x, y| smoothstep(0.35, 0.6, n2.get01(x * 0.4, y)));
        let hd = paint::Handling::new(Tool::round_sable(2.2)).mixed(pal, 0.15).color(move |x, y| snow_col(&n, x, y)).angle(|_, _| 0.0).angle_jitter(0.08).length(6.0, 20.0).coverage(1.6).pressure(0.35, 0.65).clip(false).threshold(0.4);
        c.work(&lip, &hd, 54);
        c.dry();
    }

    // ----------------------------------------------------------- the oak
    let oak_seed: u64 = std::env::var("OAK_SEED").ok().and_then(|v| v.parse().ok()).unwrap_or(3);
    let oak = Habit { years: 32, lean: -0.04, decline: 0.45, decay: 0.18, breakage: 0.15, trunk: 0.075, ..Habit::dead_oak() }.grow((176.0, 574.0), 360.0, oak_seed);
    if o.stage("oak", &mut c, &mut rng) {
        let bark = pal.paint(hex("#2b2624"), 0.25);
        let dead = pal.paint(hex("#3a3634"), 0.25);
        let lit = pal.paint(hex("#5b4c44"), 0.3);
        paint_limbs(&mut c, &oak, bark, dead, &mut rng);
        c.dry();
        // the glow finds the right-hand side of the trunk and big limbs
        let mut b = Held::new(Tool { point: 0.6, ..Tool::round_sable(1.4) }, 61);
        for l in oak.limbs.iter().filter(|l| l.order <= 1 && l.w[0] > 2.5) {
            for i in 0..l.pts.len().saturating_sub(1) {
                if l.w[i] < 2.5 || rng.f() < 0.35 {
                    continue;
                }
                let (dx, dy) = l.dir(i);
                // the side facing right (toward the glow)
                let nx = if dy.abs() > 0.3 { dy.signum() * -dy } else { 0.0 };
                let _ = nx;
                let off = 0.32 * l.w[i];
                let (a, e) = (l.pts[i], l.pts[i + 1]);
                b.reload(lit, 0.4);
                c.drag(&mut b, &Gesture::line((a.0 + off * dy.abs(), a.1 - off * dx * 0.0), (e.0 + off * dy.abs(), e.1)).pressure(0.3, 0.25).ramps(0.2, 0.3).shake(0.5), None);
            }
        }
        c.dry();
        // bark: lean, dry gray drags down the trunk and the big limbs,
        // lichen and weathered wood catching the sky
        let gray = pal.paint(hex("#6a6664"), 0.1);
        let mut d = Held::new(Tool { ragged: 0.6, ..Tool::round_sable(2.2) }, 63);
        for l in oak.limbs.iter().filter(|l| l.order == 0 || (l.order == 1 && l.w[0] > 6.0)) {
            let m = l.pts.len();
            for _ in 0..(if l.order == 0 { 14 } else { 4 }) {
                let a = (rng.f() * (m as f32 - 3.0)).max(0.0) as usize;
                let e = (a + 2 + (rng.f() * 4.0) as usize).min(m - 1);
                let side = rng.range(-0.38, 0.38);
                let pts: Vec<(f32, f32)> = (a..=e)
                    .map(|k| {
                        let (dx, dy) = l.dir(k.min(m - 2));
                        (l.pts[k].0 - dy * side * l.w[k], l.pts[k].1 + dx * side * l.w[k])
                    })
                    .collect();
                d.reload(gray, 0.18);
                c.drag(&mut d, &Gesture::new(pts).pressure(0.35, 0.2).ramps(0.2, 0.4).shake(0.8), None);
            }
        }
        c.dry();
        // snow along the tops of the thicker limbs
        snow_on_limbs(&mut c, pal, &oak, &mut rng);
        // snow drifted over the root flare, heaped against the trunk
        let (bx, by) = oak.base;
        let drift = Mask::from_fn(f, move |x, y| {
            let dx = (x - bx) / 46.0;
            let top = by - 5.0 + 9.0 * dx * dx - 2.0 * (x * 0.4).sin();
            smoothstep(top - 0.6, top + 0.8, y) * smoothstep(1.3, 0.8, dx.abs()) * smoothstep(by + 16.0, by + 8.0, y)
        });
        let hd = paint::Handling::new(Tool::filbert(4.0)).mixed(pal, 0.12).color(move |x, y| mix(snow_col(&n, x, y), hex("#e2dbd3"), 0.35 * smoothstep(bx - 20.0, bx + 30.0, x), Mix::Light)).angle(|_, _| 0.0).angle_jitter(0.2).length(5.0, 16.0).coverage(2.6).clip(true).threshold(0.3);
        c.work(&drift, &hd, 64);
        // three crows in the dead top, one on the wing
        let crow = pal.paint(hex("#1d1a19"), 0.1);
        let tips = oak.tips();
        let mut s = Held::new(Tool { point: 0.9, ..Tool::round_sable(2.0) }, 62);
        let high: Vec<&(f32, f32)> = tips.iter().filter(|p| p.1 < 330.0).collect();
        for k in 0..2.min(high.len()) {
            let p = high[(k * 7 + 3) % high.len()];
            s.reload(crow, 0.6);
            c.touch(&mut s, &Touch::at(p.0, p.1 - 1.6).pressure(0.7).drag(0.2, -0.4), None);
            c.drag(&mut s, &Gesture::line((p.0, p.1 - 2.4), (p.0 + 0.8, p.1 + 1.6)).pressure(0.6, 0.2).shake(0.2), None);
            c.drag(&mut s, &Gesture::line((p.0 - 0.8, p.1 - 3.2), (p.0 - 1.9, p.1 - 3.5)).pressure(0.4, 0.05).ramps(0.1, 0.6).shake(0.2), None);
        }
        let (fx, fy) = (340.0f32, 250.0f32);
        s.reload(crow, 0.6);
        for side in [-1.0f32, 1.0] {
            c.drag(&mut s, &Gesture::new(vec![(fx, fy), (fx + side * 3.5, fy - 1.8), (fx + side * 7.0, fy - 0.6)]).pressure(0.55, 0.0).ramps(0.1, 0.7).shake(0.4), None);
        }
        c.touch(&mut s, &Touch::at(fx, fy + 0.3).pressure(0.5), None);
        c.dry();
    }

    // ------------------------------------------------------- the spruces
    let spruces = [(812.0f32, 616.0f32, 190.0f32, 36.0f32, 5u64), (866.0, 607.0, 132.0, 27.0, 6), (768.0, 604.0, 96.0, 20.0, 7)];
    if o.stage("spruces", &mut c, &mut rng) {
        for &(x, base, ht, hw, seed) in spruces.iter().rev() {
            let env = fir::envelope_for("young", (x, base - ht), base - 0.06 * ht, hw, seed);
            let tree = Fir::grow(&env, Some((x, base)), &FirHabit { dead_below: 0.02, ..FirHabit::young() }, (-0.6, -0.2, 0.3), seed);
            paint_spruce(&mut c, pal, &tree, &mut rng);
        }
        // a fold of snow at their feet
        let foot = Mask::from_fn(f, |x, y| {
            let d = ((x - 820.0) / 110.0).powi(2) + ((y - 616.0) / 9.0).powi(2);
            smoothstep(1.0, 0.6, d)
        });
        let hd = st.body().color(move |x, y| mix(snow_col(&n, x, y), hex("#dcd6d2"), 0.3, Mix::Light)).angle(|_, _| 0.0).angle_jitter(0.1).length(8.0, 26.0).coverage(2.0).medium(0.12).clip(true).threshold(0.35);
        c.work(&foot, &hd, 71);
        c.dry();
    }

    // ------------------------------------ a stone half buried in the snow
    let (sx, sy) = (338.0f32, 652.0f32);
    let stone = Mask::from_fn(f, move |x, y| {
        let dx = (x - sx) / 40.0;
        let dy = (y - sy) / 16.0;
        if dx.abs() > 1.5 || dy.abs() > 1.5 {
            return 0.0;
        }
        let lump = dx * dx + dy * dy * (if dy < 0.0 { (1.0 + 0.4 * dx).max(0.5) } else { 1.0 });
        smoothstep(1.05, 0.95, lump)
    })
    .roughen(71, 14.0, 1.8, 0.6);
    if o.stage("stone", &mut c, &mut rng) {
        // the rock: dark, cool where it faces the sky, strokes round its form
        let rock = move |x: f32, y: f32| {
            let up = smoothstep(sy + 10.0, sy - 12.0, y);
            let v = n2.get01(x * 2.0, y * 2.0);
            mix(mix(hex("#3b3835"), hex("#6a6a73"), 0.6 * up, Mix::Pigment), hex("#524a42"), 0.4 * v, Mix::Pigment)
        };
        let hd = paint::Handling::new(Tool::filbert(3.5)).mixed(pal, 0.15).color(rock).angle(move |x, y| (y - sy).atan2(x - sx) + FRAC_PI_2).angle_jitter(0.3).length(4.0, 12.0).coverage(3.2).clip(true).threshold(0.3);
        c.work(&stone, &hd, 72);
        c.dry();
        // the snow cap, thick, lying over the top and hanging over its edge
        let cap = Mask::from_fn(f, move |x, y| {
            let top = sy - 4.0 + 3.0 * ((x - sx) / 40.0).powi(2) * 4.0 + 1.5 * (x * 0.3).sin();
            smoothstep(top + 0.8, top - 0.8, y)
        })
        .mul(&stone.clone().dilate(1.4));
        let capc = move |x: f32, y: f32| {
            let lit = smoothstep(sx - 30.0, sx + 20.0, x);
            mix(hex("#bfc0cf"), hex("#e7dfd6"), 0.3 + 0.6 * lit * smoothstep(sy + 2.0, sy - 14.0, y), Mix::Light)
        };
        let hd = paint::Handling::new(Tool::filbert(4.0)).mixed(pal, 0.1).color(capc).angle(|_, _| -0.1).angle_jitter(0.25).length(5.0, 14.0).coverage(3.0).clip(true).threshold(0.3);
        c.work(&cap, &hd, 73);
        // snow banked against its foot, and a cool shadow off its left side
        let bank = Mask::from_fn(f, move |x, y| {
            let dx = (x - sx) / 48.0;
            let top = sy + 11.0 - 2.0 * (x * 0.2).sin() + 3.0 * dx * dx;
            smoothstep(top - 0.6, top + 0.6, y) * smoothstep(1.2, 0.9, dx.abs()) * smoothstep(sy + 26.0, sy + 16.0, y)
        });
        let hd = paint::Handling::new(Tool::filbert(4.0)).mixed(pal, 0.12).color(move |x, y| snow_col(&n, x, y)).angle(|_, _| 0.0).angle_jitter(0.15).length(6.0, 18.0).coverage(2.4).clip(true).threshold(0.3);
        c.work(&bank, &hd, 74);
        c.dry();
    }

    // -------------------------------------------- the walker and his tracks
    let (wx, wy) = (548.0f32, 590.0f32);
    if o.stage("figure", &mut c, &mut rng) {
        // tracks: a line of small blue-gray hollows curving in from the front
        let mut s = Held::new(Tool::round_sable(2.2), 81);
        let path = |t: f32| -> (f32, f32) {
            let x = wx + 70.0 * (1.0 - t).powi(2) + 12.0 * (t * 4.0).sin() * (1.0 - t);
            let y = wy + 2.0 + (h - wy + 20.0) * (1.0 - t).powf(1.6);
            (x, y)
        };
        let mut t = 0.02;
        let mut k = 0;
        while t < 0.985 {
            let (x, y) = path(t);
            let scale = 0.35 + 0.65 * smoothstep(wy, h, y);
            let side = if k % 2 == 0 { -1.0 } else { 1.0 };
            let x = x + side * 1.8 * scale;
            let col = paint::shift(snow_col(&n, x, y), -0.09, 0.0, -0.035);
            s.reload(pal.paint(col, 0.25), 0.5);
            c.touch(&mut s, &Touch::at(x, y).pressure(0.25 + 0.5 * scale).drag(0.0, 1.0 * scale), None);
            t += 0.012 + 0.03 * (1.0 - scale);
            k += 1;
        }
        c.dry();
        paint_walker(&mut c, pal, (wx, wy), 30.0);
        c.dry();
    }

    // --------------------------------------------- grass and reeds, last
    if o.stage("grass", &mut c, &mut rng) {
        let dry_grass = [hex("#6b5a43"), hex("#4b4034"), hex("#7c6a4f"), hex("#3a332c")];
        let mut rg = Held::new(Tool { point: 1.0, ..Tool::rigger(0.7) }, 91);
        let mut rs = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.3) }, 92);
        // tufts along the front, on the rise under the oak, at the pond's edge
        let mut tufts: Vec<(f32, f32, f32)> = vec![];
        // an old field edge under the snow, running in from the front
        // right toward the pond: the grass stands along it
        for k in 0..26 {
            let t = (k as f32 / 25.0 + rng.range(-0.015, 0.015)).clamp(0.0, 1.0);
            let x = 1010.0 - 330.0 * t + 25.0 * (t * 5.0).sin();
            let y = 700.0 - 120.0 * t.powf(0.8);
            if rng.f() < 0.8 {
                tufts.push((x, y, 1.1 - 0.7 * t));
            }
        }
        // a few clumps in the front, in groups
        for _ in 0..7 {
            let (cx, cy) = (rng.f() * 620.0, 650.0 + rng.f() * 45.0);
            for _ in 0..(2 + (rng.f() * 4.0) as usize) {
                tufts.push((cx + rng.range(-25.0, 25.0), cy + rng.range(-5.0, 5.0), 0.7 + 0.6 * rng.f()));
            }
        }
        for _ in 0..16 {
            let x = 40.0 + rng.f() * 300.0;
            tufts.push((x, rise_top(x) + 3.0 + rng.f() * 30.0, 0.5 + 0.3 * rng.f()));
        }
        for _ in 0..18 {
            // reeds at the pond's near edge
            let a = rng.f() * std::f32::consts::PI;
            let x = 520.0 - 225.0 * a.cos();
            let y = 548.0 + 20.0 * a.sin() + 2.0;
            tufts.push((x, y, 0.35 + 0.2 * rng.f()));
        }
        for &(x, y, s) in &tufts {
            if (x - wx).abs() < 14.0 && (y - wy).abs() < 20.0 {
                continue;
            }
            let blades = 3 + (rng.f() * 8.0 * s) as usize;
            for _ in 0..blades {
                let bx = x + rng.range(-7.0, 7.0) * s;
                let by = y + rng.range(-2.0, 2.0);
                let ht = s * (10.0 + 26.0 * rng.f());
                let lean = rng.range(-0.45, 0.45) + 0.15;
                let col = dry_grass[(rng.f() * dry_grass.len() as f32) as usize % dry_grass.len()];
                let pts = vec![(bx, by), (bx + lean * ht * 0.25, by - ht * 0.5), (bx + lean * ht * 0.8, by - ht)];
                let hb = if ht > 18.0 && rng.f() < 0.4 { &mut rs } else { &mut rg };
                hb.reload(pal.paint(col, 0.15), 0.55);
                c.drag(hb, &Gesture::new(pts).pressure(0.7, 0.0).ramps(0.04, 0.85).shake(0.5), None);
            }
        }
        c.dry();
    }

    // ---------------------- a dark glaze growing toward the edges; varnish
    if o.stage("glaze", &mut c, &mut rng) {
        let umber = pal.only(&["raw umber", "bone black", "smalt"]).mix(hex("#2c2a2e")).paint(0.9).pigment();
        let (w, hh) = (1000.0f32, h);
        c.glaze(&umber, None, move |x, y| {
            let dx = (x - GLOW_X) / (0.62 * w);
            let dy = (y - HZ + 30.0) / (0.75 * hh);
            let r = (dx * dx + dy * dy).sqrt();
            1.4 * smoothstep(0.5, 1.2, r) + 0.45 * smoothstep(0.35, 0.0, y / hh)
        });
    }
    o.finish(&mut c, &mut rng, &Finish::aged(st.relief));
}

/// A tree's limbs, each one continuous movement from where it springs to
/// its tip, trunk first; a finer brush takes over where a limb thins, set
/// down inside the wet end of the stroke before.
fn paint_limbs(c: &mut paint::Canvas, sk: &paint::Skeleton, bark: Paint, dead: Paint, rng: &mut Rng) {
    for (li, l) in sk.limbs.iter().enumerate() {
        let n = l.pts.len();
        if n < 2 {
            continue;
        }
        let mut i = 0;
        while i + 1 < n {
            // a run while the width stays within a brush's range
            let w0 = l.w[i].max(0.25);
            let mut j = i + 1;
            let mut len = 0.0;
            while j + 1 < n && l.w[j] > w0 * 0.45 && len < 70.0 && l.dead_at(j) == l.dead_at(i) {
                len += ((l.pts[j].0 - l.pts[j - 1].0).powi(2) + (l.pts[j].1 - l.pts[j - 1].1).powi(2)).sqrt();
                j += 1;
            }
            let w1 = l.w[j].max(0.2);
            let tool = if w0 > 1.6 { Tool { point: 0.5, ..Tool::round_sable(w0 * 1.15) } } else { Tool { point: 1.0, ..Tool::rigger((w0 * 1.1).max(0.45)) } };
            let (p0, p1) = (tool.pressure_for(w0), tool.pressure_for(w1));
            let tip = j == n - 1;
            let mut b = Held::new(tool, (li * 31 + i) as u64 + 600);
            let paint = if l.dead_at(i) { dead } else { bark };
            b.load(paint, if w0 > 1.6 { 1.0 } else { 0.7 });
            let pts: Vec<(f32, f32)> = l.pts[i..=j].to_vec();
            let release = if tip && !l.broken { 0.35 } else { 0.03 };
            c.drag(&mut b, &Gesture::new(pts).pressure(p0.max(0.05), if tip && !l.broken { 0.0 } else { p1.max(0.05) }).ramps(0.02, release).shake(0.35 + 0.2 * rng.f()), None);
            // the next brush sets down inside this stroke's wet end
            i = if j - i > 1 { j - 1 } else { j };
        }
    }
}

/// Snow lying along the upper side of limbs thick enough to hold it and
/// level enough for it to stay.
fn snow_on_limbs(c: &mut paint::Canvas, pal: &Palette, sk: &paint::Skeleton, rng: &mut Rng) {
    let snow_lit = pal.paint(hex("#ece6df"), 0.05).with_stiff(0.9);
    let snow_cool = pal.paint(hex("#c9cad6"), 0.08);
    for (li, l) in sk.limbs.iter().enumerate() {
        if l.order > 3 {
            continue;
        }
        let n = l.pts.len();
        let mut i = 0;
        while i + 1 < n {
            let w = l.w[i];
            let (dx, dy) = l.dir(i);
            if w < 0.9 || dy.abs() > 0.8 || rng.f() < 0.25 {
                i += 1;
                continue;
            }
            // a run of fairly level segments
            let mut j = i + 1;
            while j + 1 < n && l.dir(j).1.abs() < 0.8 && j - i < 4 {
                j += 1;
            }
            let ww = (0.55 * w).clamp(0.5, 3.0);
            // the normal pointing up (y down on the canvas)
            let (nx, ny) = if dx >= 0.0 { (dy, -dx) } else { (-dy, dx) };
            let pts: Vec<(f32, f32)> = (i..=j).map(|k| (l.pts[k].0 + nx * 0.42 * l.w[k], l.pts[k].1 + ny * 0.42 * l.w[k])).collect();
            let tool = if ww > 1.4 { Tool { point: 0.6, ..Tool::round_sable(ww * 1.2) } } else { Tool { point: 1.0, ..Tool::rigger(ww) } };
            let p = tool.pressure_for(ww);
            let mut b = Held::new(tool, (li * 17 + i) as u64 + 900);
            // warm where the glow finds it, cooler underneath the sky
            b.load(if rng.f() < 0.6 { snow_lit } else { snow_cool }, 0.8);
            c.drag(&mut b, &Gesture::new(pts).pressure(p * 0.6, p * 0.3).swell(vec![0.7, 1.2, 1.0, 0.6]).ramps(0.25, 0.4).shake(0.5), None);
            i = j + 1 + (rng.f() * 2.0) as usize;
        }
    }
}

/// A young spruce: stem, then the needle pads as short hatched strokes of
/// the point, then snow heaped on the boughs.
fn paint_spruce(c: &mut paint::Canvas, pal: &Palette, t: &Fir, rng: &mut Rng) {
    let dark = pal.paint(hex("#1f2622"), 0.12);
    let mid = pal.paint(hex("#2e3830"), 0.12);
    let lit = pal.paint(hex("#4a5146"), 0.15);
    // stem
    let mut b = Held::new(Tool { point: 0.7, ..Tool::round_sable(t.leader_w[0].max(1.0) * 1.1) }, t.seed + 1);
    b.load(pal.paint(hex("#2a2420"), 0.2), 1.0);
    let p0 = b.tool.pressure_for(t.leader_w[0].max(0.8));
    c.drag(&mut b, &Gesture::new(t.leader.clone()).pressure(p0, 0.0).ramps(0.02, 0.4).shake(0.3), None);
    // needles
    let mut hb = Held::new(Tool { point: 1.0, ..Tool::round_sable(t.hatch.max(0.6) * 1.2) }, t.seed + 2);
    for (k, s) in t.strokes.iter().enumerate() {
        if k % 6 == 0 {
            let pick = if s.lit > 0.6 { lit } else if s.lit > 0.3 { mid } else { dark };
            hb.reload(pick, 0.7);
        }
        let p = hb.tool.pressure_for(s.w.max(0.4));
        c.drag(&mut hb, &Gesture::new(s.pts.to_vec()).pressure(p, p * 0.2).ramps(0.05, 0.6).shake(0.4), None);
    }
    c.dry();
    // snow: heaped on each bough's upper face, fullest along its middle
    let snow = pal.paint(hex("#e9e4de"), 0.05).with_stiff(0.95);
    let shade = pal.paint(hex("#b9bccb"), 0.08);
    let mut sb = Held::new(Tool { point: 0.5, ..Tool::round_sable((t.hatch * 2.2).max(1.4)) }, t.seed + 3);
    for bo in &t.boughs {
        if bo.dead || bo.minor || bo.pts.len() < 3 || rng.f() < 0.12 {
            continue;
        }
        let n = bo.pts.len();
        let a = (n as f32 * rng.range(0.15, 0.35)) as usize;
        let e = (n as f32 * rng.range(0.75, 0.98)) as usize;
        let pts: Vec<(f32, f32)> = (a..e.min(n)).map(|k| (bo.pts[k].0, bo.pts[k].1 - bo.pad[k].0 * 0.7)).collect();
        if pts.len() < 2 {
            continue;
        }
        sb.reload(if rng.f() < 0.7 { snow } else { shade }, 0.8);
        let p = 0.35 + 0.35 * (1.0 - bo.t * 0.5);
        c.drag(&mut sb, &Gesture::new(pts).pressure(p, p * 0.4).swell(vec![0.6, 1.3, 1.1, 0.5]).ramps(0.2, 0.5).shake(0.6), None);
    }
}

/// A man in a long dark coat and a hat, from behind, walking with a stick.
fn paint_walker(c: &mut paint::Canvas, pal: &Palette, (x, y): (f32, f32), size: f32) {
    let coat = pal.paint(hex("#26221f"), 0.12);
    let hat = pal.paint(hex("#1c1a18"), 0.1);
    let s = size;
    let mut b = Held::new(Tool { point: 0.6, ..Tool::round_sable(0.13 * s) }, 801);
    // legs, one a step behind
    b.load(coat, 0.8);
    c.drag(&mut b, &Gesture::line((x - 0.05 * s, y - 0.28 * s), (x - 0.07 * s, y)).pressure(0.35, 0.3).shake(0.2), None);
    c.drag(&mut b, &Gesture::line((x + 0.05 * s, y - 0.28 * s), (x + 0.09 * s, y - 0.03 * s)).pressure(0.35, 0.3).shake(0.2), None);
    // the coat: from the shoulders down, flaring to the hem
    b.reload(coat, 1.0);
    for k in -2..=2 {
        let u = k as f32 / 2.0;
        let top = (x + u * 0.085 * s, y - 0.8 * s + (u * u) * 0.04 * s);
        let hem = (x + u * 0.15 * s, y - 0.27 * s + 0.02 * s * (u * 3.0).sin());
        c.drag(&mut b, &Gesture::line(top, hem).pressure(0.6, 0.75).shake(0.25), None);
    }
    // head and hat
    b.reload(hat, 0.8);
    c.touch(&mut b, &Touch::at(x, y - 0.86 * s).pressure(0.55), None);
    c.drag(&mut b, &Gesture::line((x - 0.1 * s, y - 0.9 * s), (x + 0.1 * s, y - 0.905 * s)).pressure(0.35, 0.3).shake(0.2), None);
    c.drag(&mut b, &Gesture::line((x - 0.04 * s, y - 0.93 * s), (x + 0.05 * s, y - 0.935 * s)).pressure(0.5, 0.5).shake(0.2), None);
    // stick, held out forward right
    let mut r = Held::new(Tool { point: 1.0, ..Tool::rigger(0.5) }, 802);
    r.load(pal.paint(hex("#3a2f26"), 0.15), 0.8);
    c.drag(&mut r, &Gesture::line((x + 0.14 * s, y - 0.55 * s), (x + 0.25 * s, y + 0.01 * s)).pressure(0.5, 0.4).shake(0.3), None);
    // a thin light on the coat's glow side
    let mut e = Held::new(Tool { point: 1.0, ..Tool::rigger(0.45) }, 803);
    e.load(pal.paint(hex("#6d5f55"), 0.2), 0.5);
    c.drag(&mut e, &Gesture::line((x - 0.1 * s, y - 0.76 * s), (x - 0.14 * s, y - 0.32 * s)).pressure(0.3, 0.2).shake(0.3), None);
}
