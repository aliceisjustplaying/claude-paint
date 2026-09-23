//! Winter Evening with a Ruined Choir: an original picture in the manner of
//! Caspar David Friedrich (notes/fresh2_winter.md).
//!
//!   cargo paint fresh2_winter                  1000px
//!   cargo paint fresh2_winter -- --full        3200px

use paint::color::{Mix, mix, to_oklab, from_oklab};
use paint::{Apply, Fbm, Gesture, Ground, Held, Mask, Orient, Paint, Palette, Rgb, Rng, Shape, Stipple, Style, Tool, Touch, gradient, hex, smoothstep};
use paintings::run::{Finish, Run};

const ASPECT: f32 = 1.385;
/// Where the far snow meets the foot of the wooded ridge.
const HORIZON: f32 = 452.0;

fn main() {
    let o = Run::new("fresh2_winter");
    let mut rng = Rng::new(o.seed);
    // the light winter ground of the London *Winter Landscape*: chalk and
    // lead white with a little umber and ochre, two layers, the upper
    // lighter and cooler [NG p.55]; the top one brushed [KÖR p.284]
    let st = Style {
        width_mm: 450.0,
        ground: vec![
            Ground { color: hex("#c4b69c"), hiding: 0.8, um: 90.0, stiff: 0.3, apply: Apply::Knife { texture: 0.3 } },
            Ground { color: hex("#d5cfc1"), hiding: 0.8, um: 50.0, stiff: 0.4, apply: Apply::Brush },
        ],
        palette: Palette::friedrich_early(),
        ..Style::friedrich()
    };
    let pal = &st.palette;
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = c.height();

    // ---------------------------------------------------------------- geometry
    let ridge_n = Fbm::new(3, 4, 140.0);
    // the far wooded ridge: long and low, rising a little to the right,
    // a soft dip where the ruin stands
    let ridge = f.per_column(|x| {
        HORIZON - 24.0 - 12.0 * smoothstep(560.0, 1000.0, x) - 8.0 * smoothstep(320.0, 0.0, x) + 6.0 * ridge_n.get(x, 0.0) + 1.5 * ridge_n.get(x * 4.0, 9.0)
            + 10.0 * (-((x - 480.0) / 110.0).powi(2)).exp()
    });
    // the snowfield's edge against the ridge, not ruled
    let snow_n = Fbm::new(5, 3, 90.0);
    let snow_top = f.per_column(|x| HORIZON + 1.5 * snow_n.get(x, 3.0));

    // knolls: the oak's to the left, the spruces' rise to the right
    let knoll = |x: f32, y: f32| -> f32 {
        let a = 42.0 * (-((x - 262.0) / 190.0).powi(2) - ((y - 612.0) / 50.0).powi(2)).exp();
        let b = 26.0 * (-((x - 810.0) / 160.0).powi(2) - ((y - 556.0) / 36.0).powi(2)).exp();
        a + b
    };
    let drift = Fbm::new(9, 4, 150.0);
    // snow surface height (picture units, up = +): drifts elongated across
    // the picture, flattening with distance
    let snow_ht = |x: f32, y: f32| -> f32 {
        let d = ((y - HORIZON) / (h - HORIZON)).clamp(0.0, 1.0);
        knoll(x, y) + (1.5 + 9.0 * d) * drift.get(x * 0.22, y * 2.4)
    };
    // light on the snow: from the sky, strongest from the afterglow low to
    // the left; faces tipped up and toward it are lighter
    let snow_lit = |x: f32, y: f32| -> f32 {
        let e = 3.0;
        let gy = (snow_ht(x, y + e) - snow_ht(x, y - e)) / (2.0 * e);
        let gx = (snow_ht(x + e, y) - snow_ht(x - e, y)) / (2.0 * e);
        // contre-jour: the glow is beyond the horizon, so a face that falls
        // toward the viewer (gy < 0) is turned from it and in shade; a face
        // turned to the left catches the afterglow
        (2.0 * gy + 0.7 * gx).clamp(-1.0, 1.0)
    };
    let sky_col = |_x: f32, y: f32| -> Rgb {
        let t = (y / HORIZON).clamp(0.0, 1.0);
        gradient(
            &[(0.0, hex("#7b7f92")), (0.3, hex("#9c9ba8")), (0.58, hex("#bdb3b3")), (0.8, hex("#d6c9b3")), (0.93, hex("#e1d6ba")), (1.0, hex("#ddd5bf"))],
            t,
            Mix::Light,
        )
    };
    let snow_col = |x: f32, y: f32| -> Rgb {
        let d = ((y - HORIZON) / (h - HORIZON)).clamp(0.0, 1.0);
        // far snow takes the horizon's warmth; near snow is cooler, bluer
        let base = gradient(&[(0.0, hex("#d7d2c6")), (0.25, hex("#c9c8c6")), (0.6, hex("#b5b7c0")), (1.0, hex("#a2a5b3"))], d, Mix::Light);
        let l = snow_lit(x, y);
        if l >= 0.0 {
            mix(base, hex("#e4ddcf"), 0.6 * l, Mix::Light)
        } else {
            mix(base, hex("#9095a8"), 0.6 * -l, Mix::Light)
        }
    };

    let sky_m = Mask::from_fn(f, |x, y| 1.0 - smoothstep(ridge(x) + 6.0, ridge(x) + 10.0, y));
    let ridge_m = Mask::from_fn(f, |x, y| smoothstep(ridge(x) - 0.8, ridge(x) + 0.8, y) * (1.0 - smoothstep(snow_top(x) + 3.0, snow_top(x) + 5.0, y)));
    let snow_m = Mask::from_fn(f, |x, y| smoothstep(snow_top(x) - 0.8, snow_top(x) + 0.8, y));

    // ------------------------------------------------------------------ sky
    if o.stage("sky", &mut c, &mut rng) {
        // a thin lay-in, a shade duller than it will end, long level arcs
        let lay = st
            .broad()
            .color(move |x, y| mix(sky_col(x, y), hex("#8f8a8c"), 0.06, Mix::Light))
            .angle(|_, _| 0.0)
            .coverage(4.0)
            .medium(0.35);
        c.work(&sky_m, &lay, 11);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.angle(|_, _| 0.0), 12);
        }
        // stippled into the wet lay-in with a small soft round, aimed at
        // the sky's own tones: breaks the strokes, fuses with them
        let s1 = Stipple::new(Tool::stippler(2.6)).mixed(pal, 0.45).color(sky_col).coverage(|_, _| 2.0).pressure(0.5, 0.85).dips(20, 0.4, 0.5);
        c.stipple(&sky_m, &s1, 13);
        c.dry();
        // dry: a finer, lighter stipple building the glow toward the horizon
        let glow = move |x: f32, y: f32| mix(sky_col(x, y), hex("#efe4c4"), 0.05 + 0.2 * smoothstep(220.0, HORIZON, y), Mix::Light);
        let s2 = Stipple::new(Tool::stippler(1.5))
            .mixed(pal, 0.5)
            .color(glow)
            .coverage(|_, y| 2.0 * smoothstep(120.0, HORIZON - 30.0, y))
            .pressure(0.45, 0.8)
            .dips(24, 0.35, 0.6);
        c.stipple(&sky_m, &s2, 14);
        c.dry();
    }

    // ----------------------------------------------------------------- moon
    let moon = (668.0f32, 118.0f32);
    // the crescent: the lit disc less the dark one, lit toward the lower
    // right where the sun has gone down
    let (mr, dark_off) = (10.5f32, (-4.2f32, -3.4f32));
    let crescent = Mask::from_shape(f, Shape::new().circle(moon.0, moon.1, mr)).subtract(&Mask::from_shape(f, Shape::new().circle(moon.0 + dark_off.0, moon.1 + dark_off.1, mr * 0.97)));
    if o.stage("moon", &mut c, &mut rng) {
        // a faint glow around it first, stippled: fine touches a little
        // lighter than the sky, thinning out with distance
        let glow_m = Mask::from_fn(f, |x, y| if ((x - moon.0).powi(2) + (y - moon.1).powi(2)).sqrt() < 60.0 { 1.0 } else { 0.0 });
        let glow = Stipple::new(Tool::stippler(1.3))
            .mixed(pal, 0.5)
            .color(move |x, y| {
                let d = ((x - moon.0).powi(2) + (y - moon.1).powi(2)).sqrt();
                let mut l = to_oklab(sky_col(x, y));
                l[0] += 0.045 * (-(d / 30.0).powi(2)).exp();
                from_oklab(l)
            })
            .coverage(move |x, y| {
                let d = ((x - moon.0).powi(2) + (y - moon.1).powi(2)).sqrt();
                1.8 * (1.0 - smoothstep(12.0, 58.0, d))
            })
            .pressure(0.4, 0.75)
            .dips(20, 0.35, 0.6);
        c.stipple(&glow_m, &glow, 17);
        c.dry();
        // the crescent filled with small touches of a round, clipped to its
        // shape so the horns come to points
        let paint = pal.mix(hex("#efe8cc")).paint(0.12).with_hiding(0.93);
        let mut b = Held::new(Tool::round_sable(1.6), rng.next_u64());
        for k in 0..70 {
            if k % 10 == 0 {
                b.reload(paint, 0.7);
            }
            let a = rng.range(-0.9, 2.5);
            let rr = mr * rng.range(0.55, 0.95);
            let p = (moon.0 + rr * a.cos(), moon.1 + rr * a.sin());
            c.drag(&mut b, &Gesture::new(vec![p, (p.0 - 1.4 * a.sin(), p.1 + 1.4 * a.cos())]).pressure(0.8, 0.6).ramps(0.1, 0.3), Some(&crescent));
        }
        c.dry();
    }

    // ------------------------------------------------------------- distance
    if o.stage("ridge", &mut c, &mut rng) {
        // the wooded ridge: a flat blue-gray band, short level hatching,
        // darker at its crest where the trees stand against the sky
        let rc = |x: f32, y: f32| -> Rgb {
            let t = smoothstep(ridge(x), HORIZON + 2.0, y);
            mix(hex("#7c8091"), hex("#a3a3ac"), t, Mix::Light)
        };
        let hd = st.hatch().color(rc).angle(|_, _| 0.05).angle_jitter(0.25).length(4.0, 10.0).coverage(3.2).medium(0.3);
        c.work(&ridge_m, &hd, 21);
        // the treetops along the crest: tiny upright touches breaking the line
        let crest = Mask::from_fn(f, |x, y| {
            let d = y - ridge(x);
            smoothstep(-5.0, -2.0, d) * (1.0 - smoothstep(1.0, 4.0, d))
        });
        let tops = Stipple::new(Tool::stippler(1.4)).mixed(pal, 0.35).color(|_, _| hex("#7a7e8e")).coverage(|x, _| 0.9 + 0.6 * ridge_n.get(x * 6.0, 2.0)).pressure(0.4, 0.8).drag(1.2, Some(-std::f32::consts::FRAC_PI_2)).dips(20, 0.4, 0.6).aim(false);
        c.stipple(&crest, &tops, 22);
        c.dry();
    }

    // the ruined choir: gable wall with a tall lancet, a lower side wall
    // with two narrow ones, the tops broken
    let ruin_top = || -> Vec<(f32, f32)> {
        vec![
            (462.0, 452.0),
            (462.0, 392.0),
            (466.0, 385.0),
            (470.0, 388.0),
            (476.0, 370.0),
            (486.0, 356.0),
            (491.0, 351.0),
            (495.0, 356.0),
            (499.0, 360.0),
            (503.0, 366.0),
            (507.0, 364.0),
            (512.0, 378.0),
            (516.0, 386.0),
            (521.0, 389.0),
            (527.0, 394.0),
            (533.0, 392.0),
            (538.0, 399.0),
            (545.0, 401.0),
            (551.0, 408.0),
            (556.0, 407.0),
            (560.0, 415.0),
            (562.0, 452.0),
        ]
    };
    let ruin_shape = || -> Shape { Shape::new().poly(&ruin_top()) };
    let lancet = |cx: f32, w: f32, y0: f32, y1: f32| -> Shape {
        // a pointed arch: straight jambs, two arcs meeting at the apex
        let mut pts = vec![(cx - w / 2.0, y1), (cx - w / 2.0, y0 + w * 0.9)];
        for i in 0..=8 {
            let t = i as f32 / 8.0;
            pts.push((cx - w / 2.0 + w / 2.0 * t, y0 + w * 0.9 * (1.0 - (t * std::f32::consts::FRAC_PI_2).sin())));
        }
        for i in 1..=8 {
            let t = i as f32 / 8.0;
            pts.push((cx + w / 2.0 * t, y0 + w * 0.9 * (1.0 - ((1.0 - t) * std::f32::consts::FRAC_PI_2).sin())));
        }
        pts.push((cx + w / 2.0, y1));
        Shape::new().poly(&pts)
    };
    let ruin_m = Mask::from_shape(f, ruin_shape())
        .subtract(&Mask::from_shape(f, lancet(489.0, 15.0, 376.0, 438.0)))
        .subtract(&Mask::from_shape(f, lancet(535.0, 5.5, 408.0, 436.0)))
        .subtract(&Mask::from_shape(f, lancet(550.0, 5.0, 414.0, 436.0)));
    if o.stage("ruin", &mut c, &mut rng) {
        // one flat dusky tone, a touch darker than the ridge, cooler at the
        // foot where the mist will lie; the gable face a hair lighter
        let col = |x: f32, y: f32| -> Rgb {
            let side = smoothstep(519.0, 523.0, x);
            let base = mix(hex("#6d6c7a"), hex("#77737c"), side, Mix::Light);
            mix(base, hex("#8f8f99"), smoothstep(405.0, 452.0, y) * 0.6, Mix::Light)
        };
        let hd = st.detail().color(col).angle(|_, _| -std::f32::consts::FRAC_PI_2).angle_jitter(0.15).length(4.0, 12.0).coverage(3.5).medium(0.2);
        c.work(&ruin_m, &hd, 31);
        c.dry();
        // weathering: stone patched lighter and darker, broken courses
        let weather = Fbm::new(41, 3, 14.0);
        let patches = ruin_m.clone().mul_fn(|x, y| smoothstep(0.1, 0.4, weather.get(x, y * 0.7).abs()));
        let hd = st.detail().color(|x, y| if weather.get(x, y * 0.7) > 0.0 { hex("#76737d") } else { hex("#696873") }).angle(|_, _| 0.0).length(2.0, 6.0).coverage(0.8).medium(0.45).clip(true);
        c.work(&patches, &hd, 32);
        let course = pal.mix(hex("#5c5c68")).paint(0.3).with_hiding(0.5);
        let mut b = Held::new(st.line_tool(0.35), rng.next_u64());
        for _ in 0..16 {
            let y = rng.range(392.0, 448.0);
            let x0 = rng.range(462.0, 555.0);
            let len = rng.range(4.0, 14.0);
            b.reload(course, 0.4);
            c.drag(&mut b, &Gesture::new(vec![(x0, y), (x0 + len, y + rng.normal() * 0.3)]).pressure(0.5, 0.3).ramps(0.2, 0.3), Some(&ruin_m));
        }
        // the reveals: the inner face of each lancet's left jamb catches
        // the afterglow, a thin lighter strip
        let reveal = pal.mix(hex("#8c8790")).paint(0.2);
        for (cx, w, y0, y1) in [(489.0f32, 15.0f32, 376.0f32, 438.0f32), (535.0, 5.5, 408.0, 436.0), (550.0, 5.0, 414.0, 436.0)] {
            let mut b = Held::new(Tool::round_sable((w * 0.12).max(0.6)), rng.next_u64());
            b.load(reveal, 0.6);
            let x = cx + w / 2.0 + w * 0.08;
            c.drag(&mut b, &Gesture::new(vec![(x, y1), (x, y0 + w * 0.9), (cx + w * 0.2, y0 + w * 0.15)]).pressure(0.7, 0.4).ramps(0.1, 0.4), Some(&ruin_m));
        }
        // snow on the broken tops and the sills: thin pale lines where the
        // wall's top is near level
        let snow = pal.mix(hex("#cfcbc6")).paint(0.15).with_hiding(0.9);
        let tops = ruin_top();
        let mut b = Held::new(Tool::round_sable(1.0), rng.next_u64());
        for w in tops.windows(2) {
            let (a, e) = (w[0], w[1]);
            let (dx, dy) = (e.0 - a.0, e.1 - a.1);
            if dx.abs() < 1.0 || (dy / dx).abs() > 1.1 || rng.f() < 0.2 {
                continue;
            }
            b.reload(snow, 0.5);
            c.drag(&mut b, &Gesture::new(vec![(a.0, a.1 + 0.5), (e.0, e.1 + 0.5)]).pressure(0.55, 0.45).ramps(0.2, 0.3), None);
        }
        for (cx, w, y1) in [(489.0f32, 15.0f32, 438.0f32), (535.0, 5.5, 436.0), (550.0, 5.0, 436.0)] {
            b.reload(snow, 0.4);
            c.drag(&mut b, &Gesture::new(vec![(cx - w / 2.0, y1 - 0.3), (cx + w / 2.0, y1 - 0.1)]).pressure(0.5, 0.4).ramps(0.2, 0.3), None);
        }
        c.dry();
    }

    if o.stage("mist", &mut c, &mut rng) {
        // mist lying on the valley at the ridge's foot, in banks, rising
        // into the ruin's lower half: a veil built by density
        let banks = Fbm::new(31, 4, 200.0);
        let mist_m = Mask::from_fn(f, |_, y| smoothstep(360.0, 380.0, y) * (1.0 - smoothstep(HORIZON + 8.0, HORIZON + 14.0, y)));
        let cov = move |x: f32, y: f32| {
            let up = (HORIZON + 2.0 - y) / 34.0 + 0.4 * banks.get(x * 0.5, y * 3.0);
            (1.0 - smoothstep(0.0, 1.0, up)) * 2.6
        };
        let mist = Stipple::new(Tool::stippler(2.0)).mixed(pal, 0.7).color(|_, _| hex("#c9c6c3")).coverage(cov).pressure(0.45, 0.8).dips(16, 0.3, 0.7).aim(false);
        c.stipple(&mist_m, &mist, 41);
        c.dry();
    }

    // ----------------------------------------------------------------- snow
    if o.stage("snow", &mut c, &mut rng) {
        // lay-in: lean, the strokes following the lie of the drifts (level,
        // swinging), a shade darker than the snow will end
        let lay = st
            .broad()
            .color(move |x, y| mix(snow_col(x, y), hex("#8e8e98"), 0.08, Mix::Light))
            .angle(move |x, y| {
                let e = 3.0;
                let gx = (snow_ht(x + e, y) - snow_ht(x - e, y)) / (2.0 * e);
                (0.35 * gx).clamp(-0.3, 0.3)
            })
            .coverage(3.5)
            .medium(0.3);
        c.work(&snow_m, &lay, 51);
        if let Some(b) = st.blend() {
            c.work(&snow_m, &b.angle(|_, _| 0.0).coverage(2.0), 52);
        }
        c.dry();
        // body: the snow in stiffer lead-white paint, wrist strokes along
        // the drifts, heavier and whiter in the lit foreground
        let body = st
            .body()
            .color(snow_col)
            .angle(move |x, y| {
                let e = 3.0;
                let gx = (snow_ht(x + e, y) - snow_ht(x - e, y)) / (2.0 * e);
                (0.35 * gx).clamp(-0.35, 0.35)
            })
            .length(25.0, 80.0)
            .mix_jitter(0.03)
            .coverage(2.2)
            .medium(0.18)
            .load_at(move |_, y| 0.7 + 0.5 * smoothstep(HORIZON, h, y));
        c.work(&snow_m, &body, 53);
        c.dry();
    }

    // ---------------------------------------------------------------- brook
    // a frozen brook winding from the foreground back toward the ruin
    let brook: Vec<(f32, f32)> = vec![
        (735.0, 740.0),
        (662.0, 716.0),
        (575.0, 700.0),
        (528.0, 682.0),
        (546.0, 661.0),
        (612.0, 644.0),
        (636.0, 626.0),
        (604.0, 609.0),
        (548.0, 597.0),
        (523.0, 583.0),
        (538.0, 569.0),
        (576.0, 558.0),
        (586.0, 546.0),
        (561.0, 536.0),
        (530.0, 527.0),
        (521.0, 516.0),
        (535.0, 506.0),
        (547.0, 497.0),
        (535.0, 488.0),
        (516.0, 480.0),
        (509.0, 472.0),
        (512.0, 464.0),
        (506.0, 457.0),
    ];
    // world width shrinks with distance; on the picture a reach running
    // across is foreshortened, a reach running into depth is not
    let brook_w: Vec<f32> = (0..brook.len())
        .map(|i| {
            let p = brook[i];
            let (a, b) = (brook[i.saturating_sub(1)], brook[(i + 1).min(brook.len() - 1)]);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let l = (dx * dx + dy * dy).sqrt().max(1e-3);
            let d = ((p.1 - HORIZON) / (h - HORIZON)).clamp(0.0, 1.0);
            let fs = 0.12 + 0.3 * d;
            (1.5 + 40.0 * d.powf(1.5)) * ((dy / l).abs() + fs * (dx / l).abs()).min(1.0) + 0.8
        })
        .collect();
    let brook_m = Mask::from_shape(f, Shape::new().ribbon(&brook, &brook_w)).mul(&snow_m);
    if o.stage("brook", &mut c, &mut rng) {
        // the ice: the sky near the horizon mirrored dully (you see the sky
        // low down in ice seen at a slant), grayed; strokes level
        let ice = |x: f32, y: f32| -> Rgb {
            let d = ((y - HORIZON) / (h - HORIZON)).clamp(0.0, 1.0);
            let refl = sky_col(x, HORIZON * (0.97 - 0.45 * d));
            mix(mix(refl, hex("#9aa0b2"), 0.5, Mix::Light), hex("#555a69"), 0.42 + 0.22 * d, Mix::Pigment)
        };
        let hd = st.detail().color(ice).angle(|_, _| 0.0).angle_jitter(0.12).length(5.0, 18.0).coverage(3.0).medium(0.25).clip(true);
        c.work(&brook_m, &hd, 61);
        c.dry();
        // open water in a few reaches: black, with the sky's light in a
        // thin streak down its middle
        let open = Fbm::new(19, 2, 55.0);
        // an open lead down the middle of the ice, never bank to bank
        let lead_w: Vec<f32> = brook_w.iter().map(|w| w * 0.4).collect();
        let open_m = Mask::from_shape(f, Shape::new().ribbon(&brook, &lead_w)).mul_fn(|x, y| smoothstep(0.22, 0.34, open.get(x, y * 1.5)) * smoothstep(480.0, 520.0, y)).roughen(29, 8.0, 0.35, 0.08).mul(&brook_m);
        let water = |x: f32, y: f32| mix(hex("#34373f"), sky_col(x, HORIZON * 0.8), 0.22, Mix::Pigment);
        let hd = st.detail().color(water).angle(|_, _| 0.0).length(4.0, 14.0).coverage(3.0).medium(0.2).clip(true);
        c.work(&open_m, &hd, 62);
        c.dry();
        // the far bank: snow overhanging the ice casts a thin blue-gray
        // shadow line along the upper edge, broken
        let edge = pal.mix(hex("#5d6172")).paint(0.2);
        let mut b = Held::new(st.line_tool(1.0), rng.next_u64());
        for i in 0..brook.len() - 1 {
            if rng.f() < 0.25 {
                continue;
            }
            let (a, e) = (brook[i], brook[i + 1]);
            let (wa, we) = (brook_w[i], brook_w[i + 1]);
            // the upper side of the ribbon
            let (dx, dy) = (e.0 - a.0, e.1 - a.1);
            let l = (dx * dx + dy * dy).sqrt().max(1e-3);
            let mut nrm = (-dy / l, dx / l);
            if nrm.1 > 0.0 {
                nrm = (-nrm.0, -nrm.1);
            }
            let t0 = rng.range(0.0, 0.3);
            let t1 = rng.range(0.7, 1.0);
            let pa = (a.0 + dx * t0 + nrm.0 * wa * 0.45, a.1 + dy * t0 + nrm.1 * wa * 0.45);
            let pe = (a.0 + dx * t1 + nrm.0 * we * 0.45, a.1 + dy * t1 + nrm.1 * we * 0.45);
            let d = ((a.1 - HORIZON) / (h - HORIZON)).clamp(0.0, 1.0);
            b.tool = st.line_tool(0.5 + 1.6 * d);
            b.reload(edge, 0.7);
            c.drag(&mut b, &Gesture::new(vec![pa, ((pa.0 + pe.0) * 0.5 + rng.normal() * 0.6, (pa.1 + pe.1) * 0.5 + rng.normal() * 0.4), pe]).pressure(0.7, 0.5).ramps(0.2, 0.4), None);
        }
        // snow drifted over the ice here and there
        let crust = Fbm::new(23, 3, 30.0);
        let crust_m = brook_m.clone().mul_fn(|x, y| smoothstep(0.15, 0.35, crust.get(x * 0.7, y * 2.0)));
        let sn = Stipple::new(Tool::stippler(2.4)).mixed(pal, 0.3).color(move |x, y| snow_col(x, y)).coverage(|_, _| 1.4).pressure(0.5, 0.9).drag(2.0, Some(0.0)).dips(12, 0.5, 0.5).aim(false);
        c.stipple(&crust_m, &sn, 63);
        c.dry();
    }

    // ------------------------------------------------------------------ oak
    let oak_base = (262.0, 598.0);
    let ev = |k: &str, d: f32| std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d);
    let oak = paint::Habit {
        lean: ev("OAK_LEAN", -0.06),
        decline: ev("OAK_DECLINE", 0.6),
        decay: ev("OAK_DECAY", 0.3),
        breakage: ev("OAK_BREAK", 0.25),
        years: ev("OAK_YEARS", 26.0) as u32,
        ..paint::Habit::dead_oak()
    }
    .grow(oak_base, ev("OAK_H", 390.0), ev("OAK_SEED", 44.0) as u64 + o.seed);
    // gnarl: old oak wood doesn't run smooth, it kinks every few inches. One
    // displacement field for every point, so twigs stay where they spring
    let oak = prune(gnarl(oak, 1.3, 9.0, 71));
    if o.stage("oak", &mut c, &mut rng) {
        let dark = pal.mix(hex("#2b2622")).paint(0.25);
        let dead = pal.mix(hex("#3b3531")).paint(0.25);
        wood(&mut c, &oak, dark, dead, 0.3, &mut rng);
        c.dry();
        // snow lying along the upper side of the limbs that are level
        // enough to hold it; broken, never on the fine twigs
        let snow = pal.mix(hex("#dcd9d3")).paint(0.12).with_hiding(0.95);
        let snow_sh = pal.mix(hex("#aeb1bf")).paint(0.12).with_hiding(0.9);
        limb_snow(&mut c, &oak, snow, snow_sh, &mut rng);
        // the trunk stands in the snow: a low drift laid across its foot in
        // the snow's own color, a little bluer on the shadow side
        let mut b = Held::new(Tool::filbert(5.0), rng.next_u64());
        for k in 0..4 {
            let x0 = oak_base.0 - 16.0 + k as f32 * 8.0 + rng.normal() * 1.5;
            let y0 = oak_base.1 - 1.0 + rng.normal() * 0.8;
            let col = mix(snow_col(x0, y0 + 4.0), hex("#9ea3b5"), if k >= 2 { 0.3 } else { 0.0 }, Mix::Light);
            b.reload(pal.mix(col).paint(0.2).with_hiding(0.95), 0.5);
            c.drag(&mut b, &Gesture::new(vec![(x0, y0 + 1.5), (x0 + 5.0, y0 - 0.8), (x0 + 11.0, y0 + 1.2)]).pressure(0.7, 0.5).ramps(0.2, 0.4), None);
        }
        c.dry();
    }

    // -------------------------------------------------------------- spruces
    let spruces = [(742.0, 548.0, 92.0), (776.0, 542.0, 128.0), (806.0, 545.0, 74.0), (834.0, 541.0, 108.0), (866.0, 546.0, 60.0), (889.0, 549.0, 40.0), (712.0, 553.0, 34.0)];
    if o.stage("spruces", &mut c, &mut rng) {
        let needles = pal.mix(hex("#1f2522")).paint(0.15);
        let snow = pal.mix(hex("#d8d6d2")).paint(0.12).with_hiding(0.95);
        let snow_sh = pal.mix(hex("#a3a8b8")).paint(0.12).with_hiding(0.9);
        // back to front: the far (small) ones first
        let mut order: Vec<_> = spruces.iter().collect();
        order.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        for &&(x, y, ht) in &order {
            spruce(&mut c, (x, y), ht, needles, snow, snow_sh, &mut rng);
        }
        c.dry();
    }

    // --------------------------------------------------------------- figure
    let walker = (590.0, 592.0);
    if o.stage("figure", &mut c, &mut rng) {
        let coat = pal.mix(hex("#2a2729")).paint(0.15);
        let hat = pal.mix(hex("#1e1c1c")).paint(0.15);
        let skin = pal.mix(hex("#8f7a6a")).paint(0.15);
        let shadow = pal.mix(hex("#6e7387")).paint(0.2);
        let rim = pal.mix(hex("#9c8b78")).paint(0.15);
        walker_fig(&mut c, walker, 34.0, coat, hat, skin, shadow, rim, &mut rng);
        c.dry();
    }

    // ---------------------------------------------------------------- crows
    if o.stage("crows", &mut c, &mut rng) {
        let black = pal.mix(hex("#1d1b1b")).paint(0.1);
        // perched on the oak's upper limbs
        let mut perches: Vec<(f32, f32)> = oak.limbs.iter().filter(|l| l.order >= 1 && l.order <= 2 && !l.is_empty() && l.w[0] > 1.2).map(|l| l.pts[l.pts.len() * 2 / 3]).filter(|p| p.1 < oak_base.1 - 180.0).collect();
        perches.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let n = perches.len();
        for (k, p) in perches.into_iter().enumerate() {
            if k % 3 != 1 || k > n * 2 / 3 {
                continue;
            }
            crow(&mut c, (p.0, p.1 - 0.5), 5.0, black, false, &mut rng);
        }
        // two in flight toward the ruin
        crow(&mut c, (418.0, 300.0), 6.0, black, true, &mut rng);
        crow(&mut c, (447.0, 318.0), 4.5, black, true, &mut rng);
    }

    // ---------------------------------------------------------------- fence
    // an old paling fence running back from the right foreground: posts
    // leaning this way and that, one broken, rails sagging or gone; snow
    // on every top
    let posts: Vec<(f32, f32, f32)> = {
        let mut v = Vec::new();
        let (a, e) = ((975.0f32, 716.0f32), (705.0f32, 588.0f32));
        let n = 8;
        for i in 0..n {
            // spacing shrinks with distance
            let t = 1.0 - (1.0 - i as f32 / (n - 1) as f32).powf(1.0);
            let t = 1.0 - (1.0 - t) * (1.0 - t * 0.35);
            let p = (a.0 + (e.0 - a.0) * t, a.1 + (e.1 - a.1) * t);
            let d = ((p.1 - HORIZON) / (h - HORIZON)).clamp(0.0, 1.0);
            v.push((p.0, p.1, 13.0 + 52.0 * d.powf(2.0)));
        }
        v
    };
    if o.stage("fence", &mut c, &mut rng) {
        let wood_p = pal.mix(hex("#3a332e")).paint(0.2);
        let lit = pal.mix(hex("#857563")).paint(0.2).with_hiding(0.7);
        let snow = pal.mix(hex("#dad7d0")).paint(0.12).with_hiding(0.95);
        let mut tops: Vec<(f32, f32)> = Vec::new();
        for (i, &(x, y, ht)) in posts.iter().enumerate().rev() {
            let broken = i == 3;
            let ht = if broken { ht * 0.45 } else { ht * rng.range(0.85, 1.1) };
            let lean = rng.normal() * 0.08 + 0.03;
            let w = ht * 0.085;
            let top = (x + lean * ht, y - ht);
            let mut b = Held::new(Tool { ragged: 0.4, ..Tool::round_sable(w) }, rng.next_u64());
            b.load(wood_p, 0.8);
            c.drag(&mut b, &Gesture::new(vec![(x, y + w * 0.4), (x + lean * ht * 0.5 + rng.normal() * 0.3, y - ht * 0.5), top]).pressure(0.95, if broken { 0.8 } else { 0.7 }).ramps(0.0, 0.08).shake(0.5), None);
            // the left edge catches the afterglow
            let mut lb = Held::new(Tool::round_sable((w * 0.25).max(0.4)), rng.next_u64());
            lb.load(lit, 0.35);
            let off = -w * 0.3;
            c.drag(&mut lb, &Gesture::new(vec![(x + off, y - ht * 0.15), (top.0 + off, top.1 + w * 0.6)]).pressure(0.5, 0.3).ramps(0.3, 0.3), None);
            // snow cap
            let mut sb = Held::new(Tool::round_sable(w * 0.9), rng.next_u64());
            sb.load(snow, 0.6);
            c.drag(&mut sb, &Gesture::new(vec![(top.0 - w * 0.55, top.1 + w * 0.2), (top.0, top.1 - w * 0.25), (top.0 + w * 0.55, top.1 + w * 0.25)]).pressure(0.8, 0.6).ramps(0.1, 0.3), None);
            tops.push(top);
        }
        tops.reverse();
        // rails: between some posts, one rail a little below the tops,
        // sagging; one hangs broken from a post
        for i in 0..posts.len() - 1 {
            if i == 2 || i == 5 {
                continue;
            }
            let (a, e) = (tops[i], tops[i + 1]);
            let ha = posts[i].2;
            let he = posts[i + 1].2;
            let (a, e) = ((a.0, a.1 + ha * 0.25), (e.0, e.1 + he * 0.25));
            let w = (ha * 0.04).max(0.5);
            let sag = ha * 0.04;
            let mut b = Held::new(Tool::round_sable(w), rng.next_u64());
            b.load(wood_p, 0.8);
            let m = ((a.0 + e.0) * 0.5, (a.1 + e.1) * 0.5 + sag);
            c.drag(&mut b, &Gesture::new(vec![a, m, e]).pressure(0.9, 0.7).ramps(0.05, 0.1).shake(0.4), None);
            let mut sb = Held::new(Tool::rigger(w * 0.7), rng.next_u64());
            sb.load(snow, 0.6);
            let up = w * 0.55;
            c.drag(&mut sb, &Gesture::new(vec![(a.0, a.1 - up), (m.0, m.1 - up), (e.0, e.1 - up)]).pressure(0.6, 0.5).ramps(0.2, 0.2), None);
        }
        // the broken rail: from the 3rd post down into the snow
        let (a, ha) = (tops[2], posts[2].2);
        let mut b = Held::new(Tool::round_sable((ha * 0.04).max(0.5)), rng.next_u64());
        b.load(wood_p, 0.8);
        c.drag(&mut b, &Gesture::new(vec![(a.0, a.1 + ha * 0.25), (a.0 - ha * 0.4, a.1 + ha * 0.6), (a.0 - ha * 0.75, posts[2].1 - 1.0)]).pressure(0.9, 0.7).ramps(0.05, 0.3), None);
        // snow drifted against the posts' feet
        for &(x, y, ht) in &posts {
            let col = snow_col(x, y);
            let mut sb = Held::new(Tool::filbert(ht * 0.12), rng.next_u64());
            sb.load(pal.mix(col).paint(0.2), 0.5);
            c.drag(&mut sb, &Gesture::new(vec![(x - ht * 0.12, y + 0.5), (x + ht * 0.02, y - ht * 0.03), (x + ht * 0.14, y + 0.7)]).pressure(0.7, 0.5).ramps(0.2, 0.4), None);
        }
        c.dry();
    }

    // --------------------------------------------------------------- grasses
    if o.stage("grasses", &mut c, &mut rng) {
        // dry grasses and reeds laid over the finished snow in fine upturning
        // strokes [NG p.56]: tufts along the brook, at the foot of the knoll
        // and across the foreground, bigger toward the viewer
        let tufts = Fbm::new(77, 3, 150.0);
        let hues = [pal.mix(hex("#5b4c38")).paint(0.2), pal.mix(hex("#766246")).paint(0.2), pal.mix(hex("#3e342a")).paint(0.2), pal.mix(hex("#8a7a5c")).paint(0.2)];
        let mut b = Held::new(st.line_tool(0.8), rng.next_u64());
        let mut placed = 0;
        for _ in 0..6000 {
            let x = rng.range(-10.0, 1010.0);
            let y = rng.range(HORIZON + 30.0, h + 5.0);
            let d = ((y - HORIZON) / (h - HORIZON)).clamp(0.0, 1.0);
            // near the brook
            let near_brook = brook.iter().zip(&brook_w).map(|(p, w)| ((p.0 - x).powi(2) + (p.1 - y).powi(2)).sqrt() - w * 0.5).fold(f32::MAX, f32::min);
            // patches where the snow lies thin over a rise, the brook's
            // banks, and the near foreground; almost nothing far away
            let patch = smoothstep(0.62, 0.8, tufts.get01(x * 0.6, y * 2.4));
            let p = 0.004 + 1.4 * patch * (0.2 + 0.8 * d) + 0.7 * (1.0 - smoothstep(2.0, 14.0, near_brook)) * (0.3 + 0.7 * d) + 0.5 * smoothstep(0.85, 1.0, d) * tufts.get01(x * 3.0, 5.0);
            if rng.f() > p * 0.5 || brook_m.sample(x, y) > 0.3 || (x - oak_base.0).abs() < 8.0 && (y - oak_base.1).abs() < 5.0 {
                continue;
            }
            placed += 1;
            let blades = 2 + (rng.f() * 5.0) as usize;
            let tall = (4.0 + 22.0 * d.powf(1.4)) * rng.range(0.6, 1.3);
            let paint = hues[(rng.f() * hues.len() as f32) as usize % hues.len()];
            b.tool = st.line_tool(0.3 + 0.5 * d);
            b.reload(paint, 0.6);
            for _ in 0..blades {
                let lean = rng.normal() * 0.3 + 0.08;
                let len = tall * rng.range(0.5, 1.1);
                let x0 = x + rng.normal() * tall * 0.12;
                let bend = rng.normal() * 0.15 + lean * 0.5;
                let pts = vec![(x0, y), (x0 + lean * len * 0.45, y - len * 0.55), (x0 + (lean + bend) * len, y - len)];
                c.drag(&mut b, &Gesture::new(pts).pressure(0.75, 0.04).ramps(0.05, 0.6).shake(0.8), None);
            }
        }
        eprintln!("  grasses: {placed} tufts");
    }

    // ----------------------------------------------------------------- veil
    // Friedrich's advice to Carus: a dark glaze over the whole picture
    // except the moon, growing darker toward the edges [MET PDF p.35]. Here
    // very thin, cool-brown, heaviest in the lower corners; never zero
    // anywhere (see FRICTION: a glaze's edge where the thickness reaches 0
    // shows as a line)
    if o.stage("veil", &mut c, &mut rng) {
        let (cx, cy) = (520.0f32, 400.0f32);
        c.glaze(&paint::Pigment::with_hiding(hex("#8e8a8a"), 0.04), None, |x, y| {
            let dx = (x - cx) / 620.0;
            let dy = (y - cy) / 420.0;
            let r = (dx * dx + dy * dy).sqrt();
            let m = ((x - moon.0).powi(2) + (y - moon.1).powi(2)).sqrt();
            let away = smoothstep(10.0, 70.0, m);
            (0.06 + 0.5 * smoothstep(0.45, 1.3, r) + 0.18 * smoothstep(HORIZON + 80.0, h, y)) * (0.75 + 0.25 * away)
        });
    }

    let fin = Finish {
        // thin, walnut-oil paint on a 140 µm chalk ground: fine cracks, not
        // weave-bound, little grime (a well-kept small picture)
        cracks: Some(paint::Cracks { island_mm: 2.6, ground_um: 140.0, width_um: 18.0, depth_um: 9.0, cupping_um: 6.0, dirt: 0.12, corners: true, seed: 0 }),
        varnish_coats: 0.3,
        ..Finish::aged(st.relief)
    };
    o.finish(&mut c, &mut rng, &fin);
    let _ = (to_oklab, from_oklab, Touch::at, Orient::Across, Paint::body);
}

// ======================================================================
// My hand for wood, spruces, the figure and crows.

/// Full-pressure footprint width relative to the tool's, at pressure p
/// (from the footprint formula in bristle::drag_on).
fn spread(p: f32, splay: f32) -> f32 {
    (0.45 + 0.55 * p) * (1.0 + splay * (p - 0.5))
}

fn pressure_for(ratio: f32, splay: f32) -> f32 {
    let target = ratio.clamp(0.0, 1.0) * spread(1.0, splay);
    let (mut lo, mut hi) = (0.0f32, 1.0f32);
    for _ in 0..18 {
        let m = 0.5 * (lo + hi);
        if spread(m, splay) < target { lo = m } else { hi = m }
    }
    0.5 * (lo + hi)
}

fn arclen(pts: &[(f32, f32)]) -> Vec<f32> {
    let mut a = vec![0.0; pts.len()];
    for i in 1..pts.len() {
        a[i] = a[i - 1] + ((pts[i].0 - pts[i - 1].0).powi(2) + (pts[i].1 - pts[i - 1].1).powi(2)).sqrt();
    }
    a
}

/// The wood of a grown tree, trunk first: each limb pulled from where it
/// springs toward its tip, handed from brush to smaller brush where it
/// thins; the new brush sets down in the last one's wet end. Twigs finer
/// than `finest` are painted at that width, starved.
fn wood(c: &mut paint::Canvas, sk: &paint::Skeleton, live: Paint, dead: Paint, finest: f32, rng: &mut Rng) {
    // buttress roots are under the snow
    // buttress roots are under the snow, and low sprouts off the bole are
    // left out (they read as a boot at this size)
    let foot = sk.base.1 - sk.height * 0.12;
    let _ = foot;
    for l in sk.limbs.iter().filter(|l| !l.is_empty()) {
        let n = l.pts.len();
        let w: Vec<f32> = l.w.iter().map(|w| w.max(finest)).collect();
        let arc = arclen(&l.pts);
        let mut cuts = vec![0usize];
        for i in 1..n {
            let a = *cuts.last().unwrap();
            if (w[i] < w[a] * 0.45 || arc[i] - arc[a] > 60.0 + 10.0 * w[a] || i == l.dead_from) && i + 1 < n {
                cuts.push(i);
            }
        }
        cuts.push(n - 1);
        for k in 0..cuts.len() - 1 {
            let (a, b) = (cuts[k], cuts[k + 1]);
            let last = k + 2 == cuts.len();
            let ov = w[a] * 1.5;
            let a0 = if k == 0 { a } else { (0..a).rev().find(|&i| arc[a] - arc[i] >= ov).unwrap_or(0) };
            let pts = l.pts[a0..=b].to_vec();
            if pts.len() < 2 {
                continue;
            }
            let tool = if w[a] > 1.5 { Tool { ragged: 0.35, ..Tool::round_sable(w[a] / spread(1.0, 0.45)) } } else { let tw = w[a] / spread(1.0, 0.6); Tool { length: tw * 3.0, ..Tool::rigger(tw) } };
            let p1 = pressure_for(w[b] / w[a], tool.splay).max(0.05);
            let total = (arc[b] - arc[a0]).max(1e-3);
            let release = if last { if l.broken { 0.05 } else { (w[a].min(5.0) * 2.5 / total).clamp(0.15, 0.6) } } else { 0.05 };
            let paint = if l.dead_at(a) { dead } else { live };
            let thin = (l.w[a] / (2.0 * finest)).clamp(0.25, 1.0);
            let mut held = Held::new(tool, rng.next_u64());
            held.load(paint.with_hiding(paint.hiding() * (0.45 + 0.55 * thin)), 0.9 * thin.sqrt());
            c.drag(&mut held, &Gesture::new(pts).pressure(1.0, p1).ramps(0.0, release).shake(0.7), None);
        }
    }
}

/// The tree stands in snow: buttress roots are under it, and low sprouts
/// off the bole (and everything growing from them) are left out; they read
/// as a boot at this size. Parent indices are remapped.
fn prune(mut sk: paint::Skeleton) -> paint::Skeleton {
    let foot = sk.base.1 - sk.height * 0.12;
    let mut gone = vec![false; sk.limbs.len()];
    for i in 0..sk.limbs.len() {
        let l = &sk.limbs[i];
        gone[i] = l.root || (l.order == 1 && l.pts[0].1 > foot) || l.parent.is_some_and(|p| gone[p]);
    }
    let mut map = vec![usize::MAX; sk.limbs.len()];
    let mut k = 0;
    for i in 0..gone.len() {
        if !gone[i] {
            map[i] = k;
            k += 1;
        }
    }
    let limbs = std::mem::take(&mut sk.limbs);
    sk.limbs = limbs
        .into_iter()
        .enumerate()
        .filter(|(i, _)| !gone[*i])
        .map(|(_, mut l)| {
            l.parent = l.parent.map(|p| map[p]);
            l
        })
        .collect();
    sk
}

/// Kink a skeleton's limbs with a displacement field of short period,
/// fading out at the foot so the trunk stands in its own snow.
fn gnarl(mut sk: paint::Skeleton, amp: f32, period: f32, seed: u32) -> paint::Skeleton {
    let nx = Fbm::new(seed, 2, period);
    let ny = Fbm::new(seed + 1, 2, period);
    let base = sk.base;
    for l in &mut sk.limbs {
        for (i, p) in l.pts.iter_mut().enumerate() {
            let up = smoothstep(0.0, 40.0, base.1 - p.1);
            let a = amp * up * (0.6 + 0.4 * (l.w[i] / 4.0).min(1.0));
            *p = (p.0 + a * nx.get(p.0, p.1), p.1 + a * ny.get(p.0, p.1));
        }
    }
    sk
}

/// Snow along the tops of limbs: where a limb runs within ~50° of level,
/// a thin pale line sits on its upper edge, lit where it faces the
/// afterglow (left) and blue-gray where it doesn't; broken where wind took it.
fn limb_snow(c: &mut paint::Canvas, sk: &paint::Skeleton, snow: Paint, shade: Paint, rng: &mut Rng) {
    for l in sk.limbs.iter().filter(|l| !l.is_empty() && !l.root && l.order <= 3) {
        let n = l.pts.len();
        let mut run: Vec<(f32, f32)> = Vec::new();
        let mut widths: Vec<f32> = Vec::new();
        let flush = |c: &mut paint::Canvas, run: &mut Vec<(f32, f32)>, widths: &mut Vec<f32>, rng: &mut Rng| {
            let long = run.len() >= 2 && {
                let a = arclen(run);
                a[a.len() - 1] > 3.0 * widths.iter().cloned().fold(0.0, f32::max)
            };
            if long {
                let wm = widths.iter().sum::<f32>() / widths.len() as f32;
                let tw = (wm * 0.55).clamp(0.35, 3.5);
                let mut b = Held::new(if tw > 1.4 { Tool::round_sable(tw) } else { Tool::rigger(tw) }, rng.next_u64());
                let p = if rng.f() < 0.7 { snow } else { shade };
                b.load(p, 0.7);
                c.drag(&mut b, &Gesture::new(run.clone()).pressure(0.75, 0.45).swell(vec![0.7, 1.1, 0.9, 1.05]).ramps(0.15, 0.3).shake(0.5), None);
            }
            run.clear();
            widths.clear();
        };
        for i in 0..n {
            let d = l.dir(i);
            let level = d.1.abs() < 0.78;
            let hold = l.w[i] > 0.7 && level && rng.f() > 0.08;
            if hold {
                // the upper edge: the normal pointing up
                let nrm = if -d.0 < 0.0 { (d.1, -d.0) } else { (-d.1, d.0) };
                let nrm = if nrm.1 > 0.0 { (-nrm.0, -nrm.1) } else { nrm };
                let off = l.w[i] * 0.42;
                run.push((l.pts[i].0 + nrm.0 * off, l.pts[i].1 + nrm.1 * off));
                widths.push(l.w[i]);
            } else {
                flush(c, &mut run, &mut widths, rng);
            }
        }
        flush(c, &mut run, &mut widths, rng);
    }
}

/// A young spruce in snow: the stem, then tiers of drooping branches from
/// the top down, each a little longer (a straight-sided cone ending in a
/// spire); then snow laid on the upper side of the tiers, heavier low down,
/// lit on the left, bluish on the right; a little heap at the foot.
fn spruce(c: &mut paint::Canvas, base: (f32, f32), ht: f32, needles: Paint, snow: Paint, snow_sh: Paint, rng: &mut Rng) {
    let (x, y) = base;
    let top = y - ht;
    let reach_k = rng.range(0.19, 0.24);
    let mut stem = Held::new(Tool::round_sable((ht * 0.02).max(0.6)), rng.next_u64());
    stem.load(needles, 0.7);
    c.drag(&mut stem, &Gesture::new(vec![(x, y), (x + rng.normal() * 0.3, y - ht * 0.5), (x, top)]).pressure(1.0, 0.1).ramps(0.0, 0.4).shake(0.3), None);
    let tiers = (16.0 + ht / 5.0) as usize;
    let mut big = Held::new(Tool::round_sable((ht * 0.03).max(0.9)), rng.next_u64());
    let mut fine = Held::new(Tool::round_sable((ht * 0.014).max(0.45)), rng.next_u64());
    let mut hang = Held::new(Tool::round_sable((ht * 0.01).max(0.4)), rng.next_u64());
    let mut tier_pts: Vec<(f32, f32, f32, f32)> = Vec::new(); // (sx, sy, ex, ey)
    for i in 0..tiers {
        let v = (i as f32 + rng.range(0.0, 0.7)) / tiers as f32; // 0 top .. 1 bottom
        let ty = top + ht * (0.04 + 0.9 * v);
        let reach = ht * (0.01 + reach_k * v) * rng.range(0.8, 1.12);
        if i % 3 == 0 {
            big.reload(needles, 0.65);
            fine.reload(needles, 0.55);
            hang.reload(needles, 0.5);
        }
        let brush = if reach < ht * 0.06 { &mut fine } else { &mut big };
        for s in [-1.0f32, 1.0] {
            let droop = reach * rng.range(0.35, 0.55);
            let sx = x + s * 0.3;
            let e = (x + s * reach, ty + droop * 0.7);
            let m = (x + s * reach * 0.55, ty + droop);
            c.drag(brush, &Gesture::new(vec![(sx, ty), m, e]).pressure(0.95, 0.3).ramps(0.0, 0.5).shake(0.5), None);
            // a few hanging needles under the tier
            if reach > 4.0 {
                for _ in 0..(reach / 3.0) as usize {
                    let t = rng.range(0.2, 0.95);
                    let p = (x + s * reach * t, ty + droop * (0.4 + 0.6 * t) * 0.9);
                    let len = reach * rng.range(0.12, 0.25);
                    c.drag(&mut hang, &Gesture::new(vec![p, (p.0 + s * len * 0.3, p.1 + len)]).pressure(0.7, 0.1).ramps(0.0, 0.6), None);
                }
            }
            tier_pts.push((sx, ty, e.0, e.1));
        }
    }
    c.dry();
    // snow on the tiers: clumps lying on the upper side of a branch, flat
    // on top, ragged below, not on every tier; more on the lit (left) side
    let mut sb = Held::new(Tool::round_sable((ht * 0.014).max(0.5)), rng.next_u64());
    for &(sx, sy, ex, ey) in tier_pts.iter() {
        let v = (sy - top) / ht;
        let left = ex < sx;
        if rng.f() < 0.15 + 0.25 * (1.0 - v) + if left { 0.0 } else { 0.15 } {
            continue;
        }
        let p = if left { snow } else { snow_sh };
        sb.reload(p.with_hiding(0.85), rng.range(0.35, 0.6));
        let lerp = |t: f32| (sx + (ex - sx) * t, sy + (ey - sy) * t * t - 0.4 - ht * 0.005);
        let clumps = 1 + (rng.f() * 2.5) as usize;
        for _ in 0..clumps {
            let t = rng.range(0.2, 0.85);
            let len = rng.range(0.1, 0.3);
            let (p0, p1) = (lerp(t), lerp((t + len).min(0.95)));
            let sag = rng.range(0.3, 0.9);
            c.drag(&mut sb, &Gesture::new(vec![p0, ((p0.0 + p1.0) * 0.5, (p0.1 + p1.1) * 0.5 + sag * 0.5), p1]).pressure(rng.range(0.5, 0.9), rng.range(0.2, 0.5)).ramps(0.15, 0.5).shake(0.8), None);
        }
    }
    // snow at the foot, covering the stem's base
    let mut fb = Held::new(Tool::filbert((ht * 0.05).max(2.0)), rng.next_u64());
    fb.load(snow_sh.with_hiding(0.6), 0.4);
    c.drag(&mut fb, &Gesture::new(vec![(x - ht * 0.08, y + 0.8), (x + ht * 0.1, y + 0.3)]).pressure(0.45, 0.3).ramps(0.3, 0.5), None);
}

/// A man seen from behind walking away up the brook: long dark coat,
/// a cap, a stick; his shadow a short blue smear on the snow.
fn walker_fig(c: &mut paint::Canvas, at: (f32, f32), size: f32, coat: Paint, hat: Paint, skin: Paint, shadow: Paint, rim: Paint, rng: &mut Rng) {
    let mut hd = paint::Hand::new(at, size, rng.next_u64());
    hd.tremor = 0.004;
    // shadow first, falling toward the viewer and right (the glow is behind)
    let mut s = hd.take(Tool::round_sable, 0.07, shadow, 0.7);
    hd.mark(c, &mut s, paint::Mark { pts: &[(-0.06, 0.005), (0.15, -0.02), (0.38, -0.05)], pressure: (0.9, 0.3), ramps: (0.05, 0.6) }, None);
    // legs: one striding back (lower), one forward, boots dark
    let mut b = hd.take(Tool::round_sable, 0.07, coat, 0.7);
    hd.line(c, &mut b, &[(-0.035, 0.34), (-0.05, 0.16), (-0.06, 0.01)], 0.9, 0.7);
    b.reload(coat, 0.7);
    hd.line(c, &mut b, &[(0.04, 0.34), (0.05, 0.18), (0.055, 0.04)], 0.9, 0.7);
    // the coat: shoulders to hem in several strokes, widening, the hem swinging
    let mut cb = hd.take(Tool::round_sable, 0.11, coat, 0.8);
    for (u0, u1) in [(-0.07, -0.12), (-0.02, -0.03), (0.03, 0.05), (0.075, 0.11)] {
        cb.reload(coat, 0.8);
        hd.line(c, &mut cb, &[(u0, 0.8), (u0 + (u1 - u0) * 0.5, 0.58), (u1, 0.3)], 0.95, 0.9);
    }
    // shoulders and collar
    cb.reload(coat, 0.8);
    hd.line(c, &mut cb, &[(-0.1, 0.77), (0.0, 0.81), (0.1, 0.77)], 0.9, 0.9);
    // arm with the stick (right), bent forward
    b.reload(coat, 0.7);
    hd.line(c, &mut b, &[(0.1, 0.76), (0.14, 0.6), (0.17, 0.5)], 0.9, 0.8);
    // head: a dab of dark hair under a cap (seen from behind), a hint of neck
    let mut sk = hd.take(Tool::round_sable, 0.05, skin, 0.5);
    hd.dab(c, &mut sk, 0.0, 0.83, 0.03, std::f32::consts::FRAC_PI_2, 0.7);
    let mut hb = hd.take(Tool::round_sable, 0.1, hat, 0.8);
    hd.mark(c, &mut hb, paint::Mark { pts: &[(-0.035, 0.87), (0.0, 0.905), (0.04, 0.88)], pressure: (1.0, 1.0), ramps: (0.0, 0.2) }, None);
    hb.reload(hat, 0.7);
    hd.mark(c, &mut hb, paint::Mark { pts: &[(-0.045, 0.93), (0.0, 0.955), (0.05, 0.925)], pressure: (0.8, 0.6), ramps: (0.1, 0.3) }, None);
    // the stick: a rigger line from the hand to the snow ahead
    let mut r = hd.take(Tool::rigger, 0.018, hat, 0.8);
    hd.mark(c, &mut r, paint::Mark { pts: &[(0.17, 0.52), (0.21, 0.26), (0.25, 0.0)], pressure: (0.8, 0.6), ramps: (0.05, 0.1) }, None);
    // the hat's brim, a little wider than the crown
    hb.reload(hat, 0.6);
    hd.mark(c, &mut hb, paint::Mark { pts: &[(-0.075, 0.905), (0.0, 0.9), (0.08, 0.905)], pressure: (0.5, 0.5), ramps: (0.2, 0.2) }, None);
    // the edge of the left shoulder and back catching the afterglow: a
    // lean touch, broken by the tooth of the coat's paint
    let mut rb = hd.take(Tool::round_sable, 0.025, rim, 0.3);
    hd.mark(c, &mut rb, paint::Mark { pts: &[(-0.095, 0.78), (-0.1, 0.66), (-0.115, 0.5)], pressure: (0.6, 0.2), ramps: (0.2, 0.6) }, None);
    let _ = rng;
}

/// A crow: body, head and tail, perched (upright) or flying (a shallow M
/// of wings).
fn crow(c: &mut paint::Canvas, at: (f32, f32), size: f32, paint: Paint, flying: bool, rng: &mut Rng) {
    let (x, y) = at;
    let mut b = Held::new(Tool::round_sable(size * 0.32), rng.next_u64());
    b.load(paint, 0.8);
    if flying {
        // a thin shallow M: each wing pulled out from the body to a lifted
        // tip, a fine brush, the body a touch at the middle
        let s = size;
        let mut w = Held::new(Tool::round_sable(size * 0.16), rng.next_u64());
        let up = rng.range(0.25, 0.45) * s;
        for side in [-1.0f32, 1.0] {
            w.load(paint, 0.8);
            c.drag(&mut w, &Gesture::new(vec![(x, y), (x + side * s * 0.45, y - up), (x + side * s, y - up * 0.55)]).pressure(1.0, 0.25).ramps(0.0, 0.5), None);
        }
        b.load(paint, 0.5);
        c.drag(&mut b, &Gesture::new(vec![(x - s * 0.08, y + s * 0.02), (x + s * 0.1, y + s * 0.08)]).pressure(0.7, 0.5).ramps(0.0, 0.3), None);
    } else {
        let s = size;
        let lean = rng.normal() * 0.15;
        // body, hunched, tail hanging down
        c.drag(&mut b, &Gesture::new(vec![(x + lean * s, y - s * 0.95), (x, y - s * 0.5), (x - lean * s * 0.5, y + s * 0.1)]).pressure(0.8, 0.5).swell(vec![0.9, 1.3, 1.0, 0.6]).ramps(0.1, 0.4), None);
        // head and beak
        b.load(paint, 0.5);
        let dir = if rng.f() < 0.5 { -1.0 } else { 1.0 };
        c.drag(&mut b, &Gesture::new(vec![(x + lean * s, y - s * 1.0), (x + lean * s + dir * s * 0.35, y - s * 1.05)]).pressure(0.8, 0.2).ramps(0.0, 0.6), None);
    }
}
