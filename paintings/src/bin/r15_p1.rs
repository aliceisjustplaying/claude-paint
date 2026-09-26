//! r15_p1: "Evening over a Misty Valley in the Mountains" (after the manner
//! of Caspar David Friedrich; an original composition).
//!
//! Late October, some minutes after sunset. From a dark heath knoll with two
//! firs the ground falls away into a valley filled with mist; four ranges
//! step back, each paler and nearer the color of the air, to a long far
//! chain with a rounded summit against the afterglow. A thin crescent moon
//! hangs in the cooling sky. On a ledge at the right a small figure, seen
//! from behind, looks out over the mist.

use paint::color::{Mix, gradient};
use paint::{Fbm, Lead, Mask, Palette, Rng, Style, graphite, hex, smoothstep, Apply, Ground, Rgb};
use paintings::run::Run;

const AR: f32 = 1.45;

fn bump(x: f32, c: f32, w: f32) -> f32 {
    (-((x - c) / w).powi(2)).exp()
}

/// Crest lines (y of the top edge at x), far to near.
fn r4(x: f32) -> f32 {
    let n = Fbm::new(41, 4, 70.0);
    414.0 - 36.0 * bump(x, 652.0, 78.0) - 12.0 * bump(x, 548.0, 40.0) - 9.0 * bump(x, 160.0, 110.0) + 5.0 * n.get(x, 3.0) + 1.5 * Fbm::new(42, 3, 14.0).get(x, 7.0)
}
fn r3(x: f32) -> f32 {
    let n = Fbm::new(43, 4, 55.0);
    450.0 - 26.0 * bump(x, 300.0, 95.0) - 18.0 * bump(x, 70.0, 70.0) - 8.0 * bump(x, 520.0, 60.0) + 14.0 * smoothstep(560.0, 760.0, x) + 6.0 * n.get(x, 5.0) + 1.5 * Fbm::new(44, 3, 12.0).get(x, 9.0)
}
fn r2(x: f32) -> f32 {
    let n = Fbm::new(45, 4, 50.0);
    498.0 - 54.0 * bump(x, 905.0, 125.0) - 22.0 * bump(x, 760.0, 55.0) + 46.0 * smoothstep(600.0, 380.0, x) + 7.0 * n.get(x, 11.0) + 2.0 * Fbm::new(46, 3, 10.0).get(x, 2.0)
}
fn r1(x: f32) -> f32 {
    let n = Fbm::new(47, 4, 45.0);
    // a wooded ridge on the left: a ragged fringe of small firs on its crest
    let fringe = Fbm::new(48, 2, 3.5).get(x, 1.0).abs();
    522.0 - 34.0 * bump(x, 120.0, 150.0) + 34.0 * smoothstep(330.0, 520.0, x) + 50.0 * smoothstep(470.0, 650.0, x) + 6.0 * n.get(x, 13.0) - 5.0 * fringe
}
/// The foreground crest: a knoll at the left (the firs), a saddle, and a
/// ledge at the right where the figure stands.
fn fg(x: f32) -> f32 {
    let n = Fbm::new(49, 4, 60.0);
    604.0 - 60.0 * bump(x, 205.0, 150.0) - 24.0 * bump(x, 715.0, 62.0) - 34.0 * smoothstep(860.0, 1010.0, x) + 4.0 * n.get(x, 17.0) + 1.2 * Fbm::new(50, 3, 8.0).get(x, 3.0)
}

const SUN_X: f32 = 470.0;
const MOON: (f32, f32) = (742.0, 142.0);
const FIGURE_X: f32 = 712.0;

/// The sky's look on the canvas: dusky blue at the top, through a pale
/// greenish band, to lemon and a warm apricot glow over the far chain,
/// strongest above where the sun went down.
fn sky(x: f32, y: f32) -> Rgb {
    let hgt = 1000.0 / AR;
    let t = (y / 420.0).clamp(0.0, 1.0);
    let cool = gradient(
        &[
            (0.0, hex("#56678a")),
            (0.22, hex("#7688a8")),
            (0.45, hex("#a3b0bb")),
            (0.66, hex("#cfcdb7")),
            (0.82, hex("#e3d4ae")),
            (1.0, hex("#e9cfa0")),
        ],
        t,
        Mix::Light,
    );
    let glow = bump(x, SUN_X, 330.0) * smoothstep(250.0, 420.0, y);
    let warm = gradient(&[(0.0, hex("#ecd8a6")), (1.0, hex("#efbd8b"))], smoothstep(330.0, 425.0, y), Mix::Light);
    let _ = hgt;
    paint::color::mix(cool, warm, 0.85 * glow, Mix::Light)
}

fn main() {
    let o = Run::new("r15_p1");
    // Friedrich's own late grounds: warm, knifed lower layers that level the
    // weave, under a patchy whitish lead-white top layer, brushed, whose
    // striations show through thin paint [KÖR p.284].
    let st = Style {
        ground: vec![
            Ground { color: hex("#9a5a36"), hiding: 0.8, um: 110.0, stiff: 0.25, apply: Apply::Knife { texture: 0.35 } },
            Ground { color: hex("#b8987a"), hiding: 0.8, um: 70.0, stiff: 0.3, apply: Apply::Knife { texture: 0.3 } },
            Ground { color: hex("#d9ccb2"), hiding: 0.78, um: 55.0, stiff: 0.4, apply: Apply::Brush },
        ],
        palette: Palette::friedrich_1820_greens(),
        ..Style::friedrich()
    };
    let pal_sky = Palette::friedrich_1820();
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, AR, o.seed));
    let f = c.frame();
    let hgt = c.height();

    if o.stage("ground", &mut c, &mut rng) {
        // (the ground is laid by `prepare`)
    }

    if o.stage("drawing", &mut c, &mut rng) {
        // A light 2H drawing of the crests, then HB for the firs, the ledge
        // and the figure.
        let h2 = Lead::pencil("2H").unwrap();
        let hb = Lead::pencil("HB").unwrap();
        let mut worn = 0.0;
        let crest = |g: fn(f32) -> f32, x0: f32, x1: f32| -> Vec<(f32, f32)> {
            let n = ((x1 - x0) / 6.0) as usize;
            (0..=n).map(|i| {
                let x = x0 + (x1 - x0) * i as f32 / n as f32;
                (x, g(x))
            }).collect()
        };
        let lines: [(fn(f32) -> f32, f32, f32, u64); 5] = [(r4, -5.0, 1005.0, 1), (r3, -5.0, 760.0, 2), (r2, 560.0, 1005.0, 3), (r1, -5.0, 480.0, 4), (fg, -5.0, 1005.0, 5)];
        for (g, x0, x1, s) in lines {
            let pts = crest(g, x0, x1);
            let prof: Vec<f32> = (0..8).map(|i| 0.28 + 0.1 * ((i as f32) * 1.3 + s as f32).sin()).collect();
            let m = graphite::hand_line(&pts, &prof, true, false, 0.25, s);
            worn += c.draw(&h2, &m, worn, s);
        }
        // firs: axes and the outer triangles
        for (x, base, top, half, s) in [(168.0, fg(168.0) + 4.0, 292.0, 30.0, 11u64), (222.0, fg(222.0) + 4.0, 372.0, 24.0, 12)] {
            let m = graphite::hand_line(&[(x, base), (x + 1.0, top)], &[0.5, 0.45, 0.4], false, true, 0.1, s);
            worn += c.draw(&hb, &m, worn, s);
            let m = graphite::hand_line(&[(x - half, base - 18.0), (x, top), (x + half, base - 18.0)], &[0.3, 0.35, 0.3], false, false, 0.2, s + 50);
            worn += c.draw(&hb, &m, worn, s + 50);
        }
        // the figure: a slim upright with head and shoulders
        let fx = FIGURE_X;
        let fy = fg(fx);
        let m = graphite::hand_line(&[(fx - 5.0, fy), (fx - 6.5, fy - 30.0), (fx - 5.5, fy - 43.0), (fx - 2.0, fy - 47.0), (fx + 2.0, fy - 47.0), (fx + 5.5, fy - 43.0), (fx + 6.5, fy - 30.0), (fx + 5.0, fy)], &[0.5, 0.55, 0.5], true, false, 0.1, 21);
        worn += c.draw(&hb, &m, worn, 21);
        let _ = worn;
    }

    let fore = Mask::from_fn(f, move |x, y| smoothstep(-2.0, 2.0, y - fg(x)));
    if o.stage("underpainting", &mut c, &mut rng) {
        // A very thin warm-brown underpainting over the foreground only:
        // raw umber and red earth, much thinned, broad strokes following
        // the fall of the ground.
        let under = st.glaze(0.75).color(|_, _| hex("#6b4a32")).angle(|x, _| if x < 450.0 { 0.25 } else { -0.12 }).coverage(2.2).length(90.0, 220.0);
        c.work(&fore, &under, 101);
        c.dry();
    }

    // the sky runs down under every range to the foreground crest: the
    // ranges and the mist are painted over it once it is dry
    let sky_m = Mask::from_fn(f, move |x, y| smoothstep(6.0, -2.0, y - fg(x)));
    if o.stage("sky", &mut c, &mut rng) {
        // Laid in full, lean paint in long, nearly level strokes, band by band
        // from the top, then fused with the badger in two crossing passes.
        let lay = st
            .broad()
            .mixed(&pal_sky, 0.3)
            .color(sky)
            .angle(|x, y| 0.03 * ((x - SUN_X) / 500.0) + 0.02 * (y / 200.0).sin())
            .curve(0.03, 0.2)
            .coverage(4.0)
            .length(160.0, 340.0)
            .pressure(0.65, 0.85)
            .dips(1, 0.5, 0.5)
            .sweep(std::f32::consts::FRAC_PI_2);
        c.work(&sky_m, &lay, 201);
        let blend = st.blend().unwrap().angle(|_, _| 0.0).clip(false);
        c.work(&sky_m, &blend, 202);
        let blend2 = st.blend().unwrap().angle(|_, _| 0.05).cross(0.35).pressure(0.3, 0.4).clip(false);
        c.work(&sky_m, &blend2, 203);
        // stippled into the wet layer with a fine tip, each small dip aimed so
        // the passage comes to the gradient; the touches lift and fuse with
        // the paint under them
        let sp = paint::Stipple::new(paint::Tool::stippler(2.4))
            .mixed(&pal_sky, 0.4)
            .color(sky)
            .coverage(|_, _| 1.4)
            .cluster(0.0, None)
            .pressure(0.35, 0.65)
            .dips(8, 0.45, 0.5);
        c.stipple(&sky_m, &sp, 204);
        let _ = hgt;
    }

    if o.stage("sky stipple", &mut c, &mut rng) {
        // (the stipple went into the wet sky; here it dries)
        c.dry();
    }

    // ---- the ranges: each brushed along its slope over the dry sky, fused,
    // dried, then veiled at its foot with mist before the next is laid
    fn mist_col(x: f32) -> Rgb {
        paint::color::mix(hex("#c4bcc2"), hex("#e2cfb4"), bump(x, SUN_X, 300.0), Mix::Light)
    }
    fn band(f: paint::Frame, crest: fn(f32) -> f32, depth: f32, xfade: (f32, f32)) -> Mask {
        let (a, b) = xfade;
        Mask::from_fn(f, move |x, y| {
            let d = y - crest(x);
            let side = if a == b { 1.0 } else { smoothstep(a, b, x) };
            smoothstep(-0.8, 0.8, d) * smoothstep(depth + 20.0, depth - 10.0, d) * side
        })
    }
    /// How thick the mist lies over the foot of a range (0..1).
    fn mist_k(crest: fn(f32) -> f32, start: f32, full: f32, depth: f32, seed: u32) -> impl Fn(f32, f32) -> f32 + Copy + Sync {
        let w = Fbm::new(seed, 4, 90.0);
        move |x: f32, y: f32| {
            let d = y - crest(x);
            let wisp = (0.7 + 0.45 * w.get(x * 0.25, y * 1.6)).clamp(0.0, 1.2);
            (smoothstep(start, full, d) * smoothstep(depth + 30.0, depth, d) * wisp).clamp(0.0, 1.0)
        }
    }
    /// A range: brushed along its slope, the mist stippled into its foot
    /// while it is wet (the touches lift and fuse with the range's paint),
    /// fused with the badger, dried, then a faint veil over the foot.
    #[allow(clippy::too_many_arguments)]
    fn range(c: &mut paint::Canvas, st: &Style, crest: fn(f32) -> f32, depth: f32, xfade: (f32, f32), top: Rgb, fall: f32, mist: (f32, f32, f32), seed: u64) {
        let f = c.frame();
        let m = band(f, crest, depth, xfade);
        let mod_n = Fbm::new(seed as u32, 4, 60.0);
        let field = move |x: f32, y: f32| {
            let t = ((y - crest(x)) / fall).clamp(0.0, 1.0).powf(0.8);
            let col = paint::color::mix(top, mist_col(x), t, Mix::Light);
            paint::shift(col, 0.018 * mod_n.get(x, y), 0.0, 0.0)
        };
        let slope = move |x: f32, _y: f32| ((crest(x + 6.0) - crest(x - 6.0)) / 12.0).atan() * 0.8;
        let lay = st.body().color(field).angle(slope).length(30.0, 90.0).coverage(3.2).clip(true).pressure(0.6, 0.85).curve(0.04, 0.2);
        c.work(&m, &lay, seed);
        let (start, full, amount) = mist;
        let k = mist_k(crest, start, full, depth, seed as u32 + 7);
        let km = Mask::from_fn(f, move |x, y| smoothstep(0.02, 0.1, k(x, y)));
        let pal = Palette::friedrich_1820();
        let sp = paint::Stipple::new(paint::Tool::stippler(2.6))
            .mixed(&pal, 0.4)
            .color_over(move |x, y, under| paint::color::mix(under, mist_col(x), (amount * k(x, y)).min(0.95), Mix::Light))
            .coverage(move |x, y| 0.3 + 1.6 * k(x, y))
            .pressure(0.4, 0.75)
            .dips(20, 0.5, 0.5);
        c.stipple(&km, &sp, seed + 3);
        let bl = st.blend().unwrap().angle(slope).length(60.0, 160.0).pressure(0.3, 0.42);
        c.work(&m.clone().union(&km), &bl, seed + 1);
        c.dry();
        let pig = paint::Pigment::semi(hex("#e4d8c8"));
        c.glaze(&pig, Some(&km), move |x, y| 0.3 * amount * k(x, y));
    }

    if o.stage("far chain", &mut c, &mut rng) {
        range(&mut c, &st, r4, 110.0, (0.0, 0.0), hex("#a09aae"), 60.0, (10.0, 45.0, 0.85), 301);
    }
    if o.stage("third range", &mut c, &mut rng) {
        range(&mut c, &st, r3, 110.0, (0.0, 0.0), hex("#8a889f"), 60.0, (12.0, 50.0, 0.88), 311);
    }
    if o.stage("second range", &mut c, &mut rng) {
        range(&mut c, &st, r2, 120.0, (360.0, 540.0), hex("#525369"), 78.0, (20.0, 72.0, 0.84), 321);
    }
    if o.stage("near ridge", &mut c, &mut rng) {
        range(&mut c, &st, r1, 120.0, (680.0, 560.0), hex("#474d5c"), 80.0, (20.0, 75.0, 0.94), 331);
    }

    // ---- a few thin level streaks of cloud low in the sky, violet-grey,
    // their undersides catching the glow
    let cloud_m = {
        let n = Fbm::new(91, 4, 40.0);
        let streaks: [(f32, f32, f32, f32, f32); 3] = [
            // x0, x1, y at x0, y at x1, half thickness
            (560.0, 1010.0, 356.0, 343.0, 3.6),
            (720.0, 960.0, 373.0, 369.0, 2.2),
            (-10.0, 300.0, 333.0, 341.0, 2.4),
        ];
        Mask::from_fn(f, move |x, y| {
            let mut v: f32 = 0.0;
            for &(x0, x1, y0, y1, t) in streaks.iter() {
                if x < x0 - 5.0 || x > x1 + 5.0 {
                    continue;
                }
                let u = ((x - x0) / (x1 - x0)).clamp(0.0, 1.0);
                let yc = y0 + (y1 - y0) * u + 3.0 * n.get(x, y0);
                let taper = smoothstep(0.0, 0.25, u) * smoothstep(1.0, 0.7, u);
                let th = t * taper * (0.7 + 0.6 * n.get01(x * 1.7, y0 * 3.0));
                v = v.max(smoothstep(th + 0.9, th - 0.9, (y - yc).abs() * (1.0 + 0.4 * ((y > yc) as i32 as f32))));
            }
            v * smoothstep(-2.0, -6.0, y - r4(x))
        })
    };
    if o.stage("clouds", &mut c, &mut rng) {
        // thin, greyish violet, fused into the sky with the badger while wet
        let glow = move |x: f32| bump(x, SUN_X, 330.0);
        let body = move |x: f32, _y: f32| paint::color::mix(hex("#aaa0a8"), hex("#c4aba3"), 0.7 * glow(x), Mix::Light);
        let lay = st.glaze(0.7).color(body).angle(|_, _| -0.02).length(40.0, 140.0).coverage(2.5).clip(true).threshold(0.2).pressure(0.4, 0.6);
        c.work(&cloud_m, &lay, 351);
        let bl = st.blend().unwrap().angle(|_, _| 0.0).length(60.0, 160.0).pressure(0.22, 0.32).clip(false);
        c.work(&cloud_m.clone().blur(5.0).map(|v| (v * 3.0).min(1.0)), &bl, 352);
    }

    // ---- the afterglow deepened: a thin transparent glaze of vermilion and
    // chrome yellow over the lowest sky, strongest where the sun went down
    if o.stage("afterglow", &mut c, &mut rng) {
        // (neighboring glazes meet on complementary masks: no seam, no rim)
        let below = |g: fn(f32) -> f32, x: f32, y: f32| smoothstep(-0.8, 0.8, y - g(x));
        let m = Mask::from_fn(f, move |x, y| 1.0 - below(r4, x, y));
        c.glaze(&paint::Pigment::transparent(hex("#e9a265")), Some(&m), move |x, y| {
            0.4 * (0.35 + 0.65 * bump(x, SUN_X, 260.0)) * smoothstep(320.0, 408.0, y)
        });
        // and the far chain set back against it: a thin violet glaze over
        // its crest, fading down its flank, stopping at the next range
        let chain = Mask::from_fn(f, move |x, y| below(r4, x, y) * (1.0 - below(r3, x, y)));
        c.glaze(&paint::Pigment::transparent(hex("#8c82a0")), Some(&chain), move |x, y| 0.3 * smoothstep(45.0, 2.0, y - r4(x)));
        // the third range deepened a step more, so the ranges keep their order
        let third = Mask::from_fn(f, move |x, y| below(r3, x, y) * smoothstep(0.5, -1.0, y - r2(x).min(r1(x))));
        c.glaze(&paint::Pigment::transparent(hex("#7f7896")), Some(&third), move |x, y| 0.45 * smoothstep(45.0, 2.0, y - r3(x)));
        c.dry();
    }

    // ---- the foreground heath and rocks
    let fslope = move |x: f32, _y: f32| ((fg(x + 8.0) - fg(x - 8.0)) / 16.0).atan() * 0.9;
    let heath_field = {
        let patch = Fbm::new(61, 4, 70.0);
        let heather = Fbm::new(62, 3, 35.0);
        move |x: f32, y: f32| {
            let d = ((y - fg(x)) / 70.0).clamp(0.0, 1.0);
            let base = gradient(&[(0.0, hex("#403c33")), (0.3, hex("#33302a")), (1.0, hex("#211e1b"))], d, Mix::Light);
            let h = smoothstep(0.2, 0.6, heather.get01(x, y));
            let base = paint::color::mix(base, hex("#4a3329"), 0.4 * h, Mix::Light);
            paint::shift(base, 0.025 * patch.get(x, y), 0.0, 0.004 * patch.get(y, x))
        }
    };
    if o.stage("foreground", &mut c, &mut rng) {
        // dark heath: umber and black with green earth, a little warmer where
        // heather grows, a little lighter along the crest where the ground
        // turns up to the sky; strokes follow the fall of the ground and stop
        // at the crest
        let lay = st.body().color(heath_field).angle(move |x, y| fslope(x, y) + 0.15 * (y / 40.0).sin()).length(25.0, 70.0).coverage(3.2).pressure(0.65, 0.9).clip(true);
        c.work(&fore, &lay, 401);
        // the crest drawn along with the tip of the brush, so the edge against
        // the mist is found
        let edge = fore.clone().subtract(&fore.erode(5.0));
        let tip = st.detail().color(heath_field).angle(fslope).length(10.0, 30.0).coverage(2.5).clip(true);
        c.work(&edge, &tip, 402);
        // heather: short upright touches, a little lighter and warmer, dense
        // under the crest and thinning down the slope, into the wet heath
        let heather_m = fore.clone().mul_fn(|x, y| smoothstep(80.0, 4.0, y - fg(x)));
        let hth = st.hatch().color(move |x, y| paint::shift(heath_field(x, y), 0.022, 0.006, 0.008)).angle(|_, _| -1.4).angle_jitter(0.35).length(3.0, 7.0).coverage(0.9).fill(false);
        c.work(&heather_m, &hth, 403);
    }

    // rocks: a ledge under the figure and a few boulders, lit only by the
    // sky from above and behind
    let (rock_m, rock_lit) = {
        let mut form = paint::Form::new(f);
        let gy = |x: f32| fg(x);
        let solids = [
            paint::Sdf::block([748.0, gy(748.0) + 14.0, -6.0], [38.0, 30.0, 34.0], 2.0).turn([748.0, gy(748.0) + 14.0, -6.0], -0.5, 0.25, 0.3).rough(2.2, 10.0, 72, true),
            paint::Sdf::block([676.0, gy(676.0) + 12.0, 6.0], [30.0, 24.0, 28.0], 2.0).turn([676.0, gy(676.0) + 12.0, 6.0], 0.6, 0.2, -0.35).rough(2.0, 9.0, 78, true),
            paint::Sdf::ellipsoid([298.0, gy(298.0) + 9.0, 0.0], [22.0, 13.0, 18.0]).rough(3.0, 11.0, 73, true),
            paint::Sdf::ellipsoid([58.0, gy(58.0) + 10.0, 0.0], [30.0, 14.0, 22.0]).rough(3.5, 13.0, 74, true),
            paint::Sdf::ellipsoid([880.0, 668.0, 20.0], [44.0, 18.0, 26.0]).rough(3.5, 14.0, 76, true),
        ];
        for s in solids.iter() {
            form.add(s, 5.0);
        }
        form.light(paint::Light::new((-0.35, -1.0), -0.2).ambient(0.25));
        let m = form.mask(|_| 1.0).blur(0.6);
        let lit = form.mask(|s| s.shade.value);
        (m, lit)
    };
    let (rock_line, rock_sink) = {
        let rm = &rock_m;
        let wob = Fbm::new(79, 3, 9.0);
        // per column: the rock's top and bottom, and the line the heath comes up to
        let line = f.per_column(move |x| {
            let (mut top, mut bot) = (f32::NAN, f32::NAN);
            let mut y = 480.0;
            while y < 690.0 {
                if rm.sample(x, y) > 0.5 {
                    if top.is_nan() {
                        top = y;
                    }
                    bot = y;
                }
                y += 0.5;
            }
            if top.is_nan() { 2000.0 } else { top + 0.5 * (bot - top) + 2.5 * wob.get(x, 1.0) }
        });
        let sink = rock_m.clone().mul_fn(move |x, y| smoothstep(-1.0, 1.0, y - line(x)));
        (line, sink)
    };
    if o.stage("rocks", &mut c, &mut rng) {
        let lit = &rock_lit;
        let tone = Fbm::new(77, 4, 16.0);
        let field = move |x: f32, y: f32| {
            let v = smoothstep(0.3, 0.95, lit.sample(x, y));
            let col = gradient(&[(0.0, hex("#221e1b")), (0.55, hex("#34302b")), (1.0, hex("#5d5852"))], v, Mix::Light);
            paint::shift(col, 0.03 * tone.get(x, y), 0.0, 0.0)
        };
        let lay = st.body().color(field).length(8.0, 26.0).coverage(3.5).clip(true).angle(|_, _| 0.1).cross(0.6).pressure(0.6, 0.85);
        c.work(&rock_m, &lay, 411);
        let det = st.detail().color(field).length(4.0, 12.0).coverage(1.2).angle(|_, _| -0.3).angle_jitter(0.5);
        c.work(&rock_m, &det, 412);
        // the rocks sink into the heath: it is brushed back over the lower
        // half of each, up to a wavering line
        let sink = &rock_sink;
        let heath = st.body().color(heath_field).angle(|_, _| -0.15).angle_jitter(0.3).length(6.0, 16.0).coverage(3.0).clip(true);
        c.work(sink, &heath, 413);
        let hth = st.hatch().color(move |x, y| paint::shift(heath_field(x, y), 0.022, 0.006, 0.008)).angle(|_, _| -1.4).angle_jitter(0.35).length(3.0, 7.0).coverage(1.2).fill(false);
        c.work(&sink.clone().dilate(3.0), &hth, 414);
    }

    // ---- the firs, painted on the dry sky and mist
    struct Fir {
        x: f32,
        base: f32,
        top: f32,
        w: f32,
        seed: u64,
    }
    let firs = [
        Fir { x: 166.0, base: fg(166.0) + 3.0, top: 286.0, w: 38.0, seed: 501 },
        Fir { x: 224.0, base: fg(224.0) + 3.0, top: 368.0, w: 30.0, seed: 502 },
        Fir { x: 122.0, base: fg(122.0) + 3.0, top: 468.0, w: 21.0, seed: 503 },
        Fir { x: 256.0, base: fg(256.0) + 3.0, top: 492.0, w: 17.0, seed: 504 },
    ];
    // a fir's silhouette: at each height a half width that grows downward,
    // tier by tier (the branches droop, so each tier is widest at its foot)
    fn fir_half(fr: &Fir, y: f32, side: u32) -> f32 {
        let h = ((y - fr.top) / (fr.base - fr.top)).clamp(0.0, 1.0);
        let tiers = 6.0 + (fr.base - fr.top) / 14.0;
        let ti = (h * tiers).floor() as i32;
        let tier = (h * tiers).fract();
        let nt = paint::noise::rand01(ti, side as i32, fr.seed as u32);
        let n = paint::noise::rand01((y * 0.9) as i32, 3 + side as i32, fr.seed as u32);
        let gap = if nt < 0.12 { 0.55 } else { 1.0 };
        fr.w * h.powf(0.8) * (0.45 + 0.55 * tier.powf(0.8)) * (0.7 + 0.6 * nt) * (0.88 + 0.24 * n) * gap + 0.7
    }
    let fir_core = Mask::from_fn(f, {
        let fs: Vec<(f32, f32, f32, f32, u64)> = firs.iter().map(|fr| (fr.x, fr.base, fr.top, fr.w, fr.seed)).collect();
        move |x, y| {
            let mut v: f32 = 0.0;
            for &(fx, base, top, w, _) in fs.iter() {
                if y < top || y > base + 2.0 {
                    continue;
                }
                let h = ((y - top) / (base - top)).clamp(0.0, 1.0);
                let hw = 0.1 * w * h.powf(0.9) + 0.5;
                v = v.max(smoothstep(hw + 0.8, hw - 0.8, (x - fx).abs()));
            }
            v
        }
    });
    if o.stage("firs", &mut c, &mut rng) {
        let dark = st.palette.paint(hex("#1d2320"), 0.12);
        let darker = st.palette.paint(hex("#171b19"), 0.12);
        // the dense core first, in short hatched strokes falling steeply
        let core = st.hatch().color(|_, _| hex("#1b201d")).angle(|_, _| 1.3).angle_jitter(0.3).length(3.0, 8.0).coverage(2.5).clip(true);
        c.work(&fir_core, &core, 510);
        for fr in firs.iter() {
            let mut r = Rng::new(fr.seed);
            let span = fr.base - fr.top;
            // the trunk
            let mut tr = paint::Held::new(paint::Tool { point: 1.0, ..paint::Tool::round_sable(1.0 + 0.045 * fr.w) }, fr.seed);
            tr.load(darker, 0.9);
            c.drag(&mut tr, &paint::Gesture::new(vec![(fr.x, fr.base + 2.0), (fr.x + 0.4, fr.top + 0.4 * span), (fr.x + 0.2, fr.top - 3.0)]).pressure(0.8, 0.02).ramps(0.02, 0.6), None);
            // branches, top to bottom: out from the trunk, sagging, the tip
            // lifting; from each, twigs hang in a fringe
            let n = (span / 2.2) as usize;
            let bw = (2.4 + 0.07 * fr.w).min(4.6);
            let mut br = paint::Held::new(paint::Tool { point: 0.6, ..paint::Tool::round_sable(bw) }, fr.seed + 1);
            let mut tw = paint::Held::new(paint::Tool { point: 1.0, ..paint::Tool::round_sable(1.2) }, fr.seed + 2);
            for i in 0..n {
                if i % 3 == 0 {
                    br.reload(dark, 0.8);
                    tw.reload(dark, 0.7);
                }
                let yy = fr.top + span * (i as f32 + r.range(0.0, 0.9)) / n as f32;
                let hrel = (yy - fr.top) / span;
                for side in [-1.0f32, 1.0] {
                    let hw = fir_half(fr, yy + 3.0, (side > 0.0) as u32);
                    if r.range(0.0, 1.0) < 0.22 {
                        continue;
                    }
                    let l = hw * r.range(0.7, 1.12) * if r.range(0.0, 1.0) < 0.08 { 1.3 } else { 1.0 };
                    let sag = l * r.range(0.18, 0.4);
                    let lift = l * r.range(0.0, 0.18);
                    let pts = vec![
                        (fr.x + side * 0.5, yy),
                        (fr.x + side * 0.4 * l, yy + 0.6 * sag),
                        (fr.x + side * 0.8 * l, yy + sag),
                        (fr.x + side * l, yy + sag - lift),
                    ];
                    let p0 = 0.55 + 0.35 * hrel;
                    c.drag(&mut br, &paint::Gesture::new(pts).pressure(p0, 0.08).ramps(0.03, 0.7).shake(0.7), None);
                    // hanging twigs along the outer two thirds
                    let k = (l / 2.2) as usize;
                    for j in 0..k {
                        let u = 0.3 + 0.65 * (j as f32 + r.range(0.0, 1.0)) / k.max(1) as f32;
                        let tx = fr.x + side * u * l;
                        let ty = yy + sag * (u * 1.25).min(1.0) - if u > 0.85 { lift } else { 0.0 };
                        let tl = r.range(1.5, 4.5) * (0.6 + 0.6 * hrel);
                        c.drag(&mut tw, &paint::Gesture::line((tx, ty), (tx + side * r.range(-0.3, 0.6), ty + tl)).pressure(0.35, 0.05).ramps(0.05, 0.6).shake(0.5), None);
                    }
                }
            }
        }
    }

    // ---- the figure, from behind, on the ledge
    let fx = FIGURE_X;
    let fb = fg(FIGURE_X) + 0.5;
    let lean = |v: f32| 0.018 * v; // x shift at height v above the feet
    let coat = Mask::from_shape(f, paint::Shape::new().smooth_poly(&[
        (-2.3, 48.6), (-6.0, 46.9), (-7.3, 43.6), (-7.5, 36.0), (-7.7, 29.5), (-8.7, 18.5), (0.4, 17.4), (8.9, 18.4), (7.8, 29.5), (7.6, 36.5), (7.2, 43.6), (6.0, 46.9), (2.3, 48.6),
    ].map(|(u, v)| (fx + u + lean(v), fb - v))));
    let legs = Mask::from_shape(f, paint::Shape::new()
        .poly(&[(fx - 5.06, fb - 19.55), (fx - 1.15, fb - 19.55), (fx - 1.84, fb - 0.57), (fx - 5.29, fb - 0.23)])
        .add(paint::Shape::new().poly(&[(fx + 1.15, fb - 19.55), (fx + 5.17, fb - 19.55), (fx + 5.52, fb - 0.23), (fx + 2.07, fb - 0.57)])));
    let head = Mask::from_shape(f, paint::Shape::new().ellipse(fx + 0.95, fb - 51.9, 3.3, 3.9).add(paint::Shape::new().ellipse(fx + 0.95, fb - 48.6, 1.9, 1.6)));
    if o.stage("figure", &mut c, &mut rng) {
        // a dark coat, darker legs and cap, laid with the sable
        let coat_col = |_: f32, y: f32| paint::color::mix(hex("#262a28"), hex("#1f2220"), smoothstep(fg(FIGURE_X) - 46.0, fg(FIGURE_X) - 18.0, y), Mix::Light);
        let d1 = st.detail().color(coat_col).angle(|_, _| 1.57).length(3.0, 9.0).coverage(4.0).clip(true).threshold(0.05);
        c.work(&coat, &d1, 601);
        let d2 = st.detail().color(|_, _| hex("#1d1c1b")).angle(|_, _| 1.57).length(3.0, 8.0).coverage(4.0).clip(true).threshold(0.05);
        c.work(&legs.clone().union(&head), &d2, 602);
        // a staff held in the right hand, planted on the ledge
        let mut rg = paint::Held::new(st.line_tool(0.9), 603);
        rg.load(st.palette.paint(hex("#2a2521"), 0.15), 0.9);
        c.drag(&mut rg, &paint::Gesture::line((fx + 7.1, fb - 28.8), (fx + 14.4, fb + 0.5)).pressure(0.7, 0.6), None);
        // the sky light along the left of the shoulders and the cap
        let mut rim = paint::Held::new(paint::Tool { point: 1.0, ..paint::Tool::round_sable(1.0) }, 604);
        rim.load(st.palette.paint(hex("#6c6670"), 0.2), 0.6);
        c.drag(&mut rim, &paint::Gesture::new(vec![(fx - 2.6, fb - 48.4), (fx - 5.3, fb - 46.6), (fx - 6.5, fb - 42.6), (fx - 6.7, fb - 36.0)]).pressure(0.35, 0.1), None);
        c.drag(&mut rim, &paint::Gesture::new(vec![(fx - 1.9, fb - 50.6), (fx - 1.7, fb - 53.6), (fx + 0.3, fb - 55.6), (fx + 2.2, fb - 55.6)]).pressure(0.3, 0.1), None);
    }

    // ---- grass along the crest, last, as fine upturning strokes
    if o.stage("grass", &mut c, &mut rng) {
        let mut r = Rng::new(701);
        let mut held = paint::Held::new(paint::Tool { point: 0.9, ..paint::Tool::round_sable(1.1) }, 702);
        let cols = [hex("#2e2b25"), hex("#3b352b"), hex("#5c5040"), hex("#6e5c43")];
        let mut x = -4.0;
        let mut k = 0;
        while x < 1004.0 {
            x += r.range(0.6, 2.4);
            // no grass on the rocks or at the figure's feet
            let y0 = fg(x) + r.range(0.5, 4.0);
            if rock_m.sample(x, y0 - 3.0) > 0.3 || (x - FIGURE_X).abs() < 12.0 {
                continue;
            }
            if k % 6 == 0 {
                let ci = if r.range(0.0, 1.0) < 0.2 { 2 + (r.range(0.0, 1.99) as usize) } else { r.range(0.0, 1.99) as usize };
                held.reload(st.palette.paint(cols[ci], 0.15), 0.6);
            }
            k += 1;
            let len = r.range(3.0, 11.0) * (0.6 + 0.4 * smoothstep(900.0, 0.0, (x - 400.0).abs()));
            let lean = r.range(-0.5, 0.5) + 0.15;
            let pts = vec![(x, y0), (x + lean * 0.4 * len, y0 - 0.55 * len), (x + lean * len, y0 - len)];
            c.drag(&mut held, &paint::Gesture::new(pts).pressure(0.45, 0.05).ramps(0.05, 0.7).shake(0.4), None);
        }
        // blades where the heath meets the rocks
        let mut x = 0.0;
        while x < 1000.0 {
            x += r.range(0.8, 2.2);
            let yl = rock_line(x);
            if yl > 1000.0 {
                continue;
            }
            if k % 6 == 0 {
                held.reload(st.palette.paint(cols[(r.range(0.0, 2.99)) as usize], 0.15), 0.6);
            }
            k += 1;
            let y0 = yl + r.range(0.5, 3.0);
            let len = r.range(2.5, 7.0);
            let lean = r.range(-0.5, 0.5);
            c.drag(&mut held, &paint::Gesture::new(vec![(x, y0), (x + lean * 0.4 * len, y0 - 0.55 * len), (x + lean * len, y0 - len)]).pressure(0.4, 0.05).ramps(0.05, 0.7).shake(0.4), None);
        }
        // sparse tufts down the slope, fewer and darker as the ground falls
        // into shadow
        for _ in 0..260 {
            let tx = r.range(0.0, 1000.0);
            let d = r.range(8.0, 70.0);
            let ty = fg(tx) + d;
            if rock_m.sample(tx, ty) > 0.2 || fir_core.sample(tx, ty - 2.0) > 0.2 {
                continue;
            }
            let ci = if d < 30.0 && r.range(0.0, 1.0) < 0.35 { 2 } else { r.range(0.0, 1.99) as usize };
            held.reload(st.palette.paint(paint::shift(cols[ci], -0.01 * d / 10.0, 0.0, 0.0), 0.15), 0.5);
            let blades = 3 + (r.range(0.0, 4.0) as usize);
            for _ in 0..blades {
                let bx = tx + r.range(-2.5, 2.5);
                let len = r.range(3.0, 8.0) * (1.0 - 0.4 * d / 70.0);
                let lean = r.range(-0.6, 0.6);
                c.drag(&mut held, &paint::Gesture::new(vec![(bx, ty), (bx + lean * 0.4 * len, ty - 0.55 * len), (bx + lean * len, ty - len)]).pressure(0.4, 0.05).ramps(0.05, 0.7).shake(0.4), None);
            }
        }
    }

    // ---- the moon: a thin crescent, lit on the side toward the set sun
    if o.stage("moon", &mut c, &mut rng) {
        let (mx, my) = MOON;
        let (dx, dy) = (SUN_X - mx, 420.0 - my);
        let dl = (dx * dx + dy * dy).sqrt();
        let (ux, uy) = (-dx / dl, -dy / dl);
        let r0 = 6.8;
        let cres = Mask::from_fn(f, move |x, y| {
            let a = ((x - mx).powi(2) + (y - my).powi(2)).sqrt();
            let b = ((x - mx - ux * 0.42 * r0).powi(2) + (y - my - uy * 0.42 * r0).powi(2)).sqrt();
            smoothstep(r0 + 0.4, r0 - 0.4, a) * smoothstep(r0 * 1.02 - 0.5, r0 * 1.02 + 0.5, b)
        });
        // a faint halo first, then the crescent in two touches of paint
        c.glaze(&paint::Pigment::semi(hex("#dfe0da")), None, move |x, y| {
            let a = ((x - mx).powi(2) + (y - my).powi(2)).sqrt();
            0.18 * (-(a / 22.0).powi(2)).exp()
        });
        let d = st.detail().color(|_, _| hex("#f3ecd2")).length(2.0, 6.0).coverage(5.0).clip(true).threshold(0.05).pressure(0.5, 0.7);
        c.work(&cres, &d, 801);
    }

    // ---- last: a dark glaze growing deeper toward the edges and corners,
    // sparing the moon (Friedrich's advice to Carus)
    if o.stage("last glaze", &mut c, &mut rng) {
        let hh = hgt;
        let (mx, my) = MOON;
        c.glaze(&paint::Pigment::transparent(hex("#4a4150")), None, move |x, y| {
            let u = (x / 1000.0 - 0.5) * 2.0;
            let v = (y / hh - 0.45) * 2.0;
            let r = (u * u * 0.8 + v * v).sqrt();
            let m = ((x - mx).powi(2) + (y - my).powi(2)).sqrt();
            0.55 * smoothstep(0.55, 1.45, r) * smoothstep(8.0, 40.0, m)
        });
    }

    // the finish: a thin, pale varnish (not the yellowed layers of later
    // restorers), the craquelure a thin ground and thin paint make, and the
    // surface lit from the upper left
    let cracks = paint::Cracks { width_um: Some(4.0), dirt: 0.12, veil: 0.15, hierarchy: 0.5, patchy: 0.6, depth_um: 8.0, cupping_um: 6.0, ..paint::Cracks::aged(0) };
    let fin = paintings::run::Finish { varnish_coats: 0.22, varnish_vary: 0.06, cracks: Some(cracks), ..paintings::run::Finish::aged(st.relief) };
    o.finish(&mut c, &mut rng, &fin);
}
