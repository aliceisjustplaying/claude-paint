//! Morning on the Shore at Arkona (fresh2_coast): an original picture in the
//! manner of Caspar David Friedrich, painted from knowledge only.
//!
//! The Baltic just before sunrise. A flat shore of damp sand runs across the
//! bottom third; the sea is still and dark under a sky that climbs from a
//! pale lemon glow at the horizon through rose and pearl to a cool slate
//! blue. A waning crescent hangs over the place where the sun will come up.
//! Three fishermen's net poles stand on the beach a little left of center,
//! a net hung to dry between two of them; a big erratic boulder lies in the
//! left foreground; a woman in a dark gown and shawl stands at the water's
//! edge, her back to us, looking toward the glow and a brig on the horizon.
//!
//!   cargo paint fresh2_coast                      1000px
//!   cargo paint fresh2_coast -- --full            3200px

use paint::color::{Mix, mix, from_oklab, to_oklab};
use paint::form::{Light, aerial};
use paint::{Canvas, Fbm, Form, Gesture, Held, Mask, Orient, Paint, Palette, Rgb, Rng, Sdf, Stipple, Style, Tool, Touch, gradient, hex, smoothstep};
use paintings::run::{Finish, Run};

const ASPECT: f32 = 1.4;
const HORIZON: f32 = 408.0;
/// Where the sun will rise (x), below the horizon.
const SUN_X: f32 = 640.0;
const MOON: (f32, f32, f32) = (598.0, 168.0, 10.5);

// ------------------------------------------------------------------ helpers

/// One drag of a held brush through canvas points.
#[allow(clippy::too_many_arguments)]
fn stroke(c: &mut Canvas, b: &mut Held, pts: &[(f32, f32)], p0: f32, p1: f32, ramps: (f32, f32), clip: Option<&Mask>) {
    let g = Gesture::new(pts.to_vec()).pressure(p0, p1).ramps(ramps.0, ramps.1).orient(Orient::Across);
    c.drag(b, &g, clip);
}

fn held(tool: Tool, paint: Paint, load: f32, seed: u64) -> Held {
    let mut h = Held::new(tool, seed);
    h.load(paint, load);
    h
}

/// Lighten/darken in OKLab L.
fn lift(c: Rgb, dl: f32) -> Rgb {
    let mut l = to_oklab(c);
    l[0] = (l[0] + dl).clamp(0.0, 1.0);
    from_oklab(l)
}

fn main() {
    let o = Run::new("fresh2_coast");
    let st = Style::friedrich();
    let pal: &Palette = &st.palette;
    let sky_pal = pal.only(&["lead white", "pale smalt", "cobalt blue", "yellow ochre", "vermilion", "red earth", "chrome yellow"]);
    let sea_pal = pal.only(&["lead white", "pale smalt", "cobalt blue", "raw umber", "bone black", "yellow ochre"]);
    // sand, stones and wood: earths, white and black, no reds (aimed light
    // touches over a cool field otherwise reach for vermilion)
    let earth_pal = pal.only(&["lead white", "yellow ochre", "raw umber", "bone black", "pale smalt"]);
    let mut rng = Rng::new(o.seed ^ 0xC0A57);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = c.height();

    // ---------------------------------------------------------- the design
    let shore_n = Fbm::new(7, 3, 140.0);
    // the waterline: nearly level, a shallow bay in the middle, a spit
    // running out to the right
    let shore = f.per_column(move |x| 548.0 + 7.0 * (x / 210.0).sin() - 12.0 * smoothstep(620.0, 1000.0, x) + 5.0 * shore_n.get(x, 3.0) + 6.0 * (-(x - 470.0).powi(2) / 9000.0).exp());
    let shore = &shore;
    let sky_col = move |x: f32, y: f32| {
        let t = (y / HORIZON).clamp(0.0, 1.0);
        let base = gradient(
            &[(0.0, hex("#4f5a78")), (0.28, hex("#707b95")), (0.52, hex("#a4a7b0")), (0.72, hex("#cdbfb3")), (0.88, hex("#e6cfa9")), (1.0, hex("#efdeae"))],
            t,
            Mix::Light,
        );
        // the glow: brighter and warmer over the sun, cooler to the left
        let d = ((x - SUN_X) / 330.0).powi(2) + ((HORIZON - y) / 170.0).powi(2);
        let glow = (-d).exp();
        let cool = smoothstep(500.0, 0.0, x) * smoothstep(0.35, 1.0, t) * 0.25;
        let c1 = mix(base, hex("#f4e6b8"), 0.55 * glow, Mix::Light);
        mix(c1, hex("#b6aeb2"), cool, Mix::Light)
    };
    let beach_col = move |x: f32, y: f32| {
        let d = y - shore(x);
        let wet = gradient(&[(0.0, hex("#9e978c")), (1.0, hex("#7b7162"))], (d / 30.0).clamp(0.0, 1.0), Mix::Light);
        let dry = gradient(&[(0.0, hex("#8c7c62")), (0.45, hex("#6f604a")), (1.0, hex("#3a332b"))], ((y - 575.0) / (h - 575.0)).clamp(0.0, 1.0), Mix::Pigment);
        mix(wet, dry, smoothstep(10.0, 40.0, d), Mix::Light)
    };
    let sky_m = Mask::from_fn(f, |_, y| 1.0 - smoothstep(HORIZON + 4.0, HORIZON + 10.0, y));
    let sea_m = Mask::from_fn(f, |x, y| smoothstep(HORIZON - 3.0, HORIZON + 1.0, y) * (1.0 - smoothstep(shore(x) + 2.0, shore(x) + 8.0, y)));
    let beach_m = Mask::from_fn(f, |x, y| smoothstep(shore(x) - 5.0, shore(x) + 1.0, y));

    // the net poles: base (x, y), top y, lean (dx over the height)
    let poles: [((f32, f32), f32, f32); 3] = [((392.0, 606.0), 296.0, -4.0), ((458.0, 598.0), 318.0, 3.0), ((536.0, 590.0), 350.0, -2.0)];
    let pole_at = |i: usize, y: f32| {
        let ((bx, by), ty, lean) = poles[i];
        bx + lean * (by - y) / (by - ty)
    };
    // the net hangs from a rail lashed between poles 0 and 1
    // a rope between the two poles, sagging under the wet net
    let rail_y = |x: f32| {
        let t = ((x - 392.0) / (458.0 - 392.0)).clamp(0.0, 1.0);
        318.0 + 6.0 * t + 8.0 * (std::f32::consts::PI * t).sin()
    };
    let net_n = Fbm::new(19, 3, 9.0);
    // the net's lower edge: long in the middle, gathered up toward the
    // poles where its ends are tied, ragged where folds hang longer
    let hem = move |x: f32| {
        let t = ((x - 392.0) / (458.0 - 392.0)).clamp(0.0, 1.0);
        rail_y(x) + 22.0 + 42.0 * (std::f32::consts::PI * t).sin().powf(0.7) + 7.0 * net_n.get(x, 1.0) + 3.0 * (x / 3.1).sin()
    };
    let hem = &hem;

    // the woman at the water's edge
    let woman = (664.0, 553.0, 58.0);

    if o.stage("drawing", &mut c, &mut rng) {
        // graphite underdrawing: faint ruled horizon, the poles, the boulder,
        // the figure. Lean, gray, a hard fine point; it will shimmer through
        let lead = Paint::new(hex("#4a4744"), 0.5, 0.6);
        let mut pen = held(Tool { length: 1.2, ..Tool::rigger(0.5) }, lead, 0.35, 1);
        stroke(&mut c, &mut pen, &[(8.0, HORIZON), (330.0, HORIZON + 0.3), (670.0, HORIZON - 0.2), (992.0, HORIZON + 0.1)], 0.35, 0.3, (0.02, 0.02), None);
        for i in 0..3 {
            let ((_, by), ty, _) = poles[i];
            pen.reload(lead, 0.3);
            stroke(&mut c, &mut pen, &[(pole_at(i, by), by), (pole_at(i, ty), ty)], 0.35, 0.3, (0.05, 0.05), None);
        }
        pen.reload(lead, 0.3);
        let bould: Vec<(f32, f32)> = (0..=24).map(|k| {
            let a = std::f32::consts::PI * (1.0 + k as f32 / 24.0);
            (190.0 + 130.0 * a.cos(), 612.0 + 70.0 * a.sin() * if a.sin() < 0.0 { 1.0 } else { 0.2 })
        }).collect();
        stroke(&mut c, &mut pen, &bould, 0.3, 0.3, (0.05, 0.05), None);
        pen.reload(lead, 0.3);
        let (wx, wy, wh) = woman;
        stroke(&mut c, &mut pen, &[(wx - 7.0, wy), (wx - 4.0, wy - wh * 0.6), (wx, wy - wh), (wx + 4.0, wy - wh * 0.6), (wx + 7.5, wy)], 0.3, 0.3, (0.05, 0.05), None);
    }

    if o.stage("underpainting", &mut c, &mut rng) {
        // a very thin underpainting [CATS p.127]: the sea in a dull
        // blue-gray, the shore in umber, so the warm ground no longer
        // flickers through where later strokes skip or a blender lifts them
        let under_col = move |x: f32, y: f32| if y < shore(x) + 2.0 { hex("#4b5260") } else { mix(hex("#6d6150"), hex("#3a322a"), smoothstep(560.0, h, y), Mix::Pigment) };
        let land_m = Mask::from_fn(f, |_, y| smoothstep(HORIZON - 1.0, HORIZON + 2.0, y));
        let up = st
            .broad()
            .palette(&earth_pal)
            .color(under_col)
            .by_masstone()
            .angle(|_, _| 0.0)
            .coverage(3.0)
            .medium(0.55)
            .clip(true);
        c.work(&land_m, &up, 9);
        c.dry();
    }

    if o.stage("sky", &mut c, &mut rng) {
        // a thin lay-in, a shade duller than it will end, strokes swung
        // across in long elbow arcs; then fused lightly top to bottom
        let lay = st
            .broad()
            .palette(&sky_pal)
            .color(move |x, y| mix(sky_col(x, y), hex("#8a8078"), 0.06, Mix::Light))
            .angle(|x, y| 0.03 * ((x + y) / 300.0).sin())
            .drift(0.12, 400.0)
            .coverage(4.2)
            .medium(0.4)
            .clip(true);
        c.work(&sky_m, &lay, 11);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.angle(|_, _| 0.0).coverage(2.4), 12);
        }
        // first stipple into the wet lay-in: breaks the strokes, fuses
        let s1 = Stipple::new(Tool::stippler(3.2))
            .mixed(&sky_pal, 0.45)
            .color(sky_col)
            .coverage(|_, _| 2.0)
            .pressure(0.5, 0.85)
            .dips(20, 0.4, 0.5);
        c.stipple(&sky_m, &s1, 13);
        c.dry();
    }

    if o.stage("sky stipple", &mut c, &mut rng) {
        // dry: a finer, lighter stipple, denser toward the glow
        let glow = move |x: f32, y: f32| lift(sky_col(x, y), 0.015 + 0.03 * smoothstep(200.0, HORIZON, y));
        let s2 = Stipple::new(Tool::stippler(1.7))
            .mixed(&sky_pal, 0.5)
            .color(glow)
            .coverage(move |x, y| 0.4 + 1.8 * smoothstep(120.0, HORIZON, y) * (0.6 + 0.4 * (-((x - SUN_X) / 380.0).powi(2)).exp()))
            .pressure(0.45, 0.8)
            .dips(24, 0.35, 0.6);
        c.stipple(&sky_m, &s2, 21);
        c.dry();
    }

    if o.stage("clouds", &mut c, &mut rng) {
        // a few low banks of stratus lying over the glow: violet-gray bodies,
        // their undersides caught by the light from below the horizon
        let mut r = Rng::new(31);
        let banks: [(f32, f32, f32, f32); 6] = [
            (40.0, 318.0, 300.0, 5.0),
            (230.0, 352.0, 260.0, 3.5),
            (500.0, 338.0, 330.0, 4.5),
            (780.0, 300.0, 230.0, 6.0),
            (700.0, 372.0, 280.0, 3.0),
            (120.0, 380.0, 200.0, 2.5),
        ];
        let body = hex("#8a7f8a");
        let lit = hex("#f0d2a6");
        for (k, &(x0, y0, len, thick)) in banks.iter().enumerate() {
            let tool = Tool { lay: 0.5, ..Tool::filbert(thick * 2.2) };
            let n = 3 + (len / 90.0) as usize;
            for j in 0..n {
                let xs = x0 + r.range(0.0, len * 0.4);
                let l = len * r.range(0.35, 0.7);
                let y = y0 + r.range(-thick, thick) * 0.8;
                let under = c.under(xs + l * 0.5, y, 4.0);
                let want = mix(under, body, 0.35 + 0.25 * r.f(), Mix::Light);
                let p = c.aim(&sky_pal, want, (xs + l * 0.5, y), 4.0, 0.55, 0.8);
                let mut b = held(tool.clone(), p, 0.35, 100 + (k * 10 + j) as u64);
                let pts: Vec<(f32, f32)> = (0..6).map(|i| {
                    let t = i as f32 / 5.0;
                    (xs + t * l, y + (t * 3.1).sin() * thick * 0.4 - t * thick * 0.3)
                }).collect();
                stroke(&mut c, &mut b, &pts, 0.35, 0.2, (0.3, 0.4), None);
            }
            // the lit underside: a thin warm line along the bottom of the bank
            for j in 0..2 {
                let xs = x0 + r.range(0.05, 0.3) * len;
                let l = len * r.range(0.4, 0.6);
                let y = y0 + thick * 0.8 + r.range(-0.8, 0.8);
                let under = c.under(xs + l * 0.5, y, 3.0);
                let want = mix(under, lit, 0.6, Mix::Light);
                let p = c.aim(&sky_pal, want, (xs + l * 0.5, y), 2.0, 0.3, 1.0);
                let mut b = held(Tool::round_sable(thick * 0.8), p, 0.4, 200 + (k * 10 + j) as u64);
                let pts: Vec<(f32, f32)> = (0..5).map(|i| {
                    let t = i as f32 / 4.0;
                    (xs + t * l, y + (t * 2.7).sin() * 0.8)
                }).collect();
                stroke(&mut c, &mut b, &pts, 0.4, 0.15, (0.35, 0.5), None);
            }
        }
        // soften the banks into the sky, along them
        let bank_m = Mask::from_fn(f, |_, y| smoothstep(280.0, 300.0, y) * (1.0 - smoothstep(392.0, 402.0, y)));
        let soft = paint::Handling::new(Tool { pickup: 0.12, ..Tool::badger(18.0) })
            .blender()
            .angle(|_, _| 0.0)
            .length(60.0, 160.0)
            .coverage(1.2)
            .pressure(0.2, 0.3)
            .dips(3, 0.0, 0.9)
            .cross(0.05);
        c.work(&bank_m, &soft, 33);
        c.dry();
    }

    if o.stage("moon", &mut c, &mut rng) {
        // a waning crescent, its lit limb turned down toward the sun below
        // the horizon; pale warm white, laid with the tip of a small round
        let (mx, my, mr) = MOON;
        let lit_dir = ((SUN_X + 60.0 - mx), (HORIZON + 200.0 - my));
        let ln = (lit_dir.0 * lit_dir.0 + lit_dir.1 * lit_dir.1).sqrt();
        let (ux, uy) = (lit_dir.0 / ln, lit_dir.1 / ln);
        let cres = Mask::from_fn(f, move |x, y| {
            let d0 = ((x - mx).powi(2) + (y - my).powi(2)).sqrt();
            let d1 = ((x - mx + ux * mr * 0.55).powi(2) + (y - my + uy * mr * 0.55).powi(2)).sqrt();
            (1.0 - smoothstep(mr - 0.6, mr + 0.4, d0)) * smoothstep(mr * 0.98, mr * 1.08, d1)
        });
        // a faint halo stippled first
        let halo = Stipple::new(Tool::stippler(1.4))
            .mixed(&sky_pal, 0.6)
            .color(move |x, y| lift(sky_col(x, y), 0.05))
            .coverage(move |x, y| 1.6 * (1.0 - smoothstep(mr, mr * 3.2, ((x - mx).powi(2) + (y - my).powi(2)).sqrt())))
            .dips(20, 0.3, 0.6);
        let halo_m = Mask::from_fn(f, move |x, y| if ((x - mx).powi(2) + (y - my).powi(2)).sqrt() < mr * 3.5 { 1.0 } else { 0.0 });
        c.stipple(&halo_m, &halo, 41);
        let hd = paint::Handling::new(Tool::round_sable(1.8))
            .mixed(&sky_pal, 0.08)
            .color(|_, _| hex("#f6efd8"))
            .by_masstone()
            .length(2.0, 6.0)
            .coverage(4.0)
            .angle(move |_, _| uy.atan2(ux) + std::f32::consts::FRAC_PI_2)
            .pressure(0.6, 0.85)
            .dips(4, 0.6, 0.6)
            .clip(true)
            .threshold(0.15);
        c.work(&cres, &hd, 42);
        c.dry();
    }

    if o.stage("sea", &mut c, &mut rng) {
        // the sea: dark and cool at the horizon, the sky's glow in it nearer,
        // a path of light under the sun; long level strokes
        let sea_col = move |x: f32, y: f32| {
            let sh = shore(x);
            let t = ((y - HORIZON) / (sh - HORIZON)).clamp(0.0, 1.0);
            let base = gradient(&[(0.0, hex("#3c4556")), (0.12, hex("#4d5566")), (0.5, hex("#6b6f7a")), (0.85, hex("#8b8883")), (1.0, hex("#9c948a"))], t, Mix::Light);
            let path = (-((x - SUN_X) / (40.0 + 150.0 * t)).powi(2)).exp();
            mix(base, hex("#cdbb9c"), 0.45 * path * (0.3 + 0.7 * t), Mix::Light)
        };
        let lay = st
            .broad()
            .palette(&sea_pal)
            .color(sea_col)
            .angle(|_, _| 0.0)
            .angle_jitter(0.015)
            .curve(0.015, 0.2)
            .drift(0.03, 400.0)
            .length(90.0, 260.0)
            .coverage(4.0)
            .medium(0.35)
            .clip(true);
        c.work(&sea_m, &lay, 51);
        // fuse with a narrower badger kept inside the water (a 40-unit
        // badger swept level drags the dark horizon band up into the sky)
        let sea_blend_m = sea_m.clone().mul_fn(|_, y| smoothstep(HORIZON + 5.0, HORIZON + 14.0, y));
        let soft = paint::Handling::new(Tool { pickup: 0.07, run: 45.0, ..Tool::badger(16.0) })
            .blender()
            .angle(|_, _| 0.0)
            .angle_jitter(0.02)
            .cross(0.03)
            .length(120.0, 300.0)
            .coverage(2.2)
            .pressure(0.3, 0.42)
            .dips(3, 0.0, 0.9)
            .clip(true)
            .sweep(std::f32::consts::FRAC_PI_2);
        c.work(&sea_blend_m, &soft, 52);
        c.dry();
        // swell: long, faint, darker and lighter bands, closer together
        // toward the horizon (perspective), laid with a soft flat
        let mut r = Rng::new(53);
        let mut y = HORIZON + 3.0;
        let mut k = 0u64;
        while y < 548.0 {
            let t = (y - HORIZON) / 140.0;
            let gap = 2.0 + 16.0 * t * t + r.range(0.0, 3.0 + 6.0 * t);
            y += gap;
            let n = 1 + (r.f() * 3.0) as usize;
            for _ in 0..n {
                let xs = r.range(-60.0, 960.0);
                let l = r.range(60.0, 260.0) * (0.5 + t);
                let ym = y + r.range(-1.0, 1.0);
                if ym > shore(xs.clamp(0.0, 999.0)) - 4.0 {
                    continue;
                }
                let under = c.under(xs + l * 0.5, ym, 3.0);
                let dark = r.chance(0.55);
                let path = (-((xs + l * 0.5 - SUN_X) / (40.0 + 150.0 * t)).powi(2)).exp();
                let want = if dark { lift(under, -0.03 - 0.02 * t) } else { mix(lift(under, 0.05), hex("#d8cdb4"), 0.25 * path, Mix::Light) };
                // mixed by masstone: an aimed thin light over the cool sea
                // comes out salmon (the pile fights the blue of thin white)
                let p = sea_pal.paint(want, 0.3);
                let w = 0.8 + 2.5 * t;
                let mut b = held(Tool { lay: 0.6, ..Tool::filbert(w) }, p, 0.3, 5300 + k);
                k += 1;
                let pts: Vec<(f32, f32)> = (0..5).map(|i| {
                    let s = i as f32 / 4.0;
                    (xs + s * l, ym + (s * 5.0 + xs).sin() * 0.3 * w)
                }).collect();
                stroke(&mut c, &mut b, &pts, 0.45, 0.3, (0.25, 0.35), Some(&sea_m));
            }
        }
        // glitter under the glow: short pale dashes, denser and brighter
        // near the shore, a few on the dark far water
        for i in 0..90 {
            let t = r.f().powf(0.6);
            let y = HORIZON + 2.0 + t * (538.0 - HORIZON);
            let spread = 18.0 + 110.0 * t;
            let x = SUN_X + r.normal() * spread * 0.55;
            if y > shore(x.clamp(0.0, 999.0)) - 5.0 {
                continue;
            }
            let l = 3.0 + 14.0 * t * r.range(0.4, 1.2);
            let under = c.under(x, y, 1.5);
            let want = mix(lift(under, 0.05 + 0.04 * t), hex("#e9dcbc"), 0.15, Mix::Light);
            let p = sea_pal.paint(want, 0.3);
            let mut b = held(Tool { lay: 0.6, ..Tool::round_sable(0.4 + 0.6 * t) }, p, 0.35, 5600 + i);
            stroke(&mut c, &mut b, &[(x - l * 0.5, y), (x, y + 0.15), (x + l * 0.5, y)], 0.4, 0.2, (0.3, 0.5), None);
        }
        // the horizon: one crisp, slightly darker line, ruled, the eye rests on it
        let hp = c.aim(&sea_pal, hex("#353d4c"), (500.0, HORIZON + 1.0), 1.0, 0.2, 1.0);
        let mut b = held(Tool::round_sable(1.1), hp, 0.5, 5900);
        let mut x0 = -5.0f32;
        while x0 < 1000.0 {
            let x1 = x0 + r.range(110.0, 170.0);
            b.reload(hp, 0.4);
            stroke(&mut c, &mut b, &[(x0 - 4.0, HORIZON + 1.0), (x1, HORIZON + 1.0)], 0.5, 0.5, (0.04, 0.04), None);
            x0 = x1;
        }
        c.dry();
    }

    if o.stage("shore", &mut c, &mut rng) {
        // the beach: damp sand at the water's edge holds the sky; drier,
        // warmer sand behind; the foreground sinks into shadow
        let lay = st
            .body()
            .palette(&earth_pal)
            .color(beach_col)
            .angle(|x, _| -0.02 + 0.04 * (x / 260.0).sin())
            .angle_jitter(0.05)
            .length(40.0, 120.0)
            .curve(0.03, 0.2)
            .coverage(3.2)
            .medium(0.3)
            .clip(true);
        c.work(&beach_m, &lay, 61);
        if let Some(b) = st.blend() {
            c.work(&beach_m, &b.angle(|_, _| 0.0).coverage(1.4).pressure(0.3, 0.4), 62);
        }
        c.dry();
        // pools left by the tide in the damp sand, holding the sky: flat
        // lenses, the light of the horizon in them, a dark lip below
        let pools: [(f32, f32, f32, f32); 3] = [(262.0, 0.0, 46.0, 2.2), (748.0, 0.0, 64.0, 2.8), (938.0, 0.0, 30.0, 1.8)];
        let pn = Fbm::new(23, 3, 25.0);
        let pool_m = Mask::from_fn(f, move |x, y| {
            let mut m = 0.0f32;
            for &(px, _, rx, ry) in &pools {
                let py = shore(px.clamp(0.0, 999.0)) + 14.0 + ry * 2.0;
                let d = ((x - px) / rx).powi(2) + ((y - py) / ry).powi(2) + 0.35 * pn.get(x, y * 3.0);
                m = m.max(1.0 - smoothstep(0.8, 1.0, d));
            }
            m
        });
        let pool_col = move |x: f32, y: f32| {
            // what the pool mirrors: the sky low over the horizon, a little dimmer
            let refl = sky_col(x, HORIZON - 6.0 - 2.0 * (y - 560.0).max(0.0));
            lift(mix(refl, hex("#8f8c88"), 0.35, Mix::Light), -0.09)
        };
        let pool = st
            .detail()
            .palette(&sky_pal)
            .color(pool_col)
            .angle(|_, _| 0.0)
            .angle_jitter(0.01)
            .length(8.0, 30.0)
            .coverage(3.0)
            .medium(0.2);
        c.work(&pool_m, &pool, 64);
        c.dry();
        // ripple marks on the sand: short bowed strokes, light on their
        // sky-facing crests, a darker trough under each
        let mut r = Rng::new(65);
        let mut k = 0u64;
        for _ in 0..420 {
            let y = r.range(578.0, h - 3.0);
            let x = r.range(-5.0, 1005.0);
            let persp = ((y - 560.0) / (h - 560.0)).clamp(0.0, 1.0);
            if pool_m.sample(x, y) > 0.1 {
                continue;
            }
            let l = (5.0 + 26.0 * persp) * r.range(0.6, 1.3);
            let bow = r.range(-0.12, 0.12) * l;
            let light = r.chance(0.5);
            let under = c.under(x, y, 1.5);
            let want = if light { mix(lift(under, 0.05 + 0.02 * (1.0 - persp)), hex("#a9a092"), 0.15, Mix::Light) } else { lift(under, -0.05) };
            let p = if light { earth_pal.paint(want, 0.3) } else { c.aim(&earth_pal, want, (x, y), 1.0, 0.35, 0.8) };
            let w = 0.5 + 1.8 * persp;
            let mut b = held(Tool { lay: 0.55, ..Tool::round_sable(w) }, p, 0.35, 6400 + k);
            k += 1;
            stroke(&mut c, &mut b, &[(x - l * 0.5, y), (x, y + bow * 0.3), (x + l * 0.5, y + 0.05 * l * r.normal())], 0.45, 0.25, (0.3, 0.5), None);
        }
        // the lap of the water: a thin line of foam, broken, and the thin
        // sheet of wet sand just above it reflecting the glow
        let mut r = Rng::new(63);
        let mut x = -10.0f32;
        let mut k = 0;
        while x < 1005.0 {
            let l = r.range(20.0, 90.0);
            let xs = x;
            x += l + r.range(4.0, 40.0);
            let pts: Vec<(f32, f32)> = (0..5).map(|i| {
                let s = i as f32 / 4.0;
                let xx = xs + s * l;
                (xx, shore(xx.clamp(0.0, 999.0)) - 0.5 + (s * 3.0).sin() * 0.6)
            }).collect();
            let (mx, my) = pts[2];
            let path = (-((mx - SUN_X) / 200.0).powi(2)).exp();
            let under = c.under(mx, my, 2.0);
            let want = mix(lift(under, 0.08), hex("#dcd6c8"), 0.2 + 0.2 * path, Mix::Light);
            let p = sea_pal.paint(want, 0.35);
            let mut b = held(Tool { lay: 0.6, ..Tool::round_sable(r.range(0.6, 1.2)) }, p, 0.4, 6300 + k);
            k += 1;
            stroke(&mut c, &mut b, &pts, r.range(0.3, 0.55), r.range(0.1, 0.3), (0.3, 0.5), None);
        }
        // the wrack line: dark seaweed strewn along the last high water
        for i in 0..140 {
            let x = r.range(-5.0, 1005.0);
            let y = shore(x.clamp(0.0, 999.0)) + 18.0 + 6.0 * (x / 90.0).sin() + r.normal() * 2.5;
            let l = r.range(2.0, 9.0);
            let under = c.under(x, y, 2.0);
            let p = c.aim(&earth_pal, mix(under, hex("#2f2a22"), 0.55, Mix::Pigment), (x, y), 1.0, 0.2, 1.0);
            let mut b = held(Tool::round_sable(r.range(0.8, 1.6)), p, 0.45, 6500 + i);
            let a = r.normal() * 0.4;
            stroke(&mut c, &mut b, &[(x, y), (x + l * a.cos(), y + l * a.sin() * 0.4)], 0.6, 0.3, (0.1, 0.4), None);
        }
        // pebbles and small stones on the foreground sand: each a dark dab
        // with a light touch on its sunward (right) top
        for i in 0..260 {
            let y = r.range(585.0, h - 4.0);
            let x = r.range(-5.0, 1005.0);
            let persp = (y - 560.0) / (h - 560.0);
            let s = (0.8 + 3.6 * persp * persp) * r.range(0.5, 1.4);
            let under = c.under(x, y, s);
            let p = c.aim(&earth_pal, lift(mix(under, hex("#3b3630"), 0.5, Mix::Pigment), -0.03), (x, y), s, 0.2, 1.2);
            let mut b = held(Tool::round_sable(s * 1.2), p, 0.5, 6800 + i);
            c.touch(&mut b, &Touch::at(x, y).pressure(r.range(0.5, 0.9)).drag(s * 0.4, 0.0), None);
            if r.chance(0.7) {
                let under = c.under(x + s * 0.3, y - s * 0.3, s * 0.4);
                let p = earth_pal.paint(mix(lift(under, 0.1), hex("#b3a590"), 0.3, Mix::Light), 0.15);
                let mut b = held(Tool::round_sable(s * 0.55), p, 0.5, 7100 + i);
                c.touch(&mut b, &Touch::at(x + s * 0.25, y - s * 0.3).pressure(0.6).drag(s * 0.3, 0.0), None);
            }
        }
        c.dry();
    }

    if o.stage("rocks", &mut c, &mut rng) {
        // the erratic: a big rounded granite boulder, and a few stones in the
        // shallows on the right. The light comes low from the right and from
        // behind: the masses stay dark, their tops and right shoulders catch it
        let mut form = Form::new(f);
        let big = Sdf::ellipsoid([188.0, 606.0, 0.0], [128.0, 64.0, 70.0])
            .turn([188.0, 606.0, 0.0], 0.25, 0.15, 0.06)
            .rough(9.0, 110.0, 3, false)
            .cut([240.0, 560.0, 20.0], [0.45, -1.0, 0.2], 10, 5.0)
            .cut([90.0, 590.0, 0.0], [-1.0, -0.3, 0.35], 11, 4.0)
            .rough(1.2, 18.0, 4, true);
        let big_id = form.add(&big, 0.3);
        let low = Sdf::ellipsoid([352.0, 655.0, 40.0], [30.0, 13.0, 22.0])
            .turn([352.0, 655.0, 40.0], -0.3, 0.2, 0.1)
            .rough(3.0, 35.0, 5, false)
            .cut([352.0, 646.0, 50.0], [0.2, -1.0, 0.3], 12, 3.0);
        let low_id = form.add(&low, 0.25);
        let small = Sdf::ellipsoid([842.0, 539.0, -200.0], [24.0, 11.0, 16.0])
            .rough(2.5, 30.0, 6, false)
            .union(Sdf::ellipsoid([872.0, 541.0, -205.0], [12.0, 7.0, 10.0]).rough(1.5, 20.0, 7, false), 3.0)
            .union(Sdf::ellipsoid([805.0, 542.0, -210.0], [9.0, 5.0, 8.0]).rough(1.2, 16.0, 8, false), 2.0);
        let small_id = form.add(&small, 2.0);
        form.light(Light::new((0.85, -0.35), -0.25).ambient(0.3).bounce(0.3, [-0.3, 0.9, 0.3]).penumbra(0.08).across_parts(false));
        let stone = move |s: &paint::form::Sample| {
            let light = hex("#a88f72");
            let half = hex("#554b40");
            let shadow = hex("#34302b");
            let core = hex("#211e1b");
            let v = s.shade.value;
            let dir = s.shade.direct;
            let base = if dir > 0.02 { mix(half, light, smoothstep(0.02, 0.55, dir), Mix::Pigment) } else { mix(core, shadow, smoothstep(0.0, 0.5, v), Mix::Pigment) };
            // sky light on the tops, and the air on the far stones
            let top = mix(base, hex("#6e6f78"), 0.25 * s.shade.sky * (1.0 - dir), Mix::Light);
            mix(top, hex("#8e8a88"), aerial(s.dist, 6.0), Mix::Light)
        };
        // first their shadows on the sand: the light is low and behind them,
        // so the shadows reach toward us and a little left; soft, thin,
        // darkest where stone meets sand
        let sh_m = Mask::from_fn(f, |x, y| {
            let a = ((x - 170.0) / 150.0).powi(2) + ((y - 652.0) / 24.0).powi(2);
            let b = ((x - 338.0) / 40.0).powi(2) + ((y - 668.0) / 9.0).powi(2);
            let c2 = ((x - 832.0) / 42.0).powi(2) + ((y - 548.5) / 3.5).powi(2);
            (1.0 - smoothstep(0.55, 1.0, a.min(b).min(c2))).max(0.0)
        });
        let sh = paint::Handling::new(Tool { lay: 0.6, ..Tool::filbert(6.0) })
            .mixed(&earth_pal, 0.5)
            .color(move |x, y| mix(lift(beach_col(x, y), -0.12), hex("#555a66"), 0.15, Mix::Light))
            .angle(|_, _| -0.08)
            .length(20.0, 60.0)
            .coverage(2.0)
            .pressure(0.4, 0.6)
            .dips(2, 0.4, 0.6)
            .clip(true)
            .threshold(0.15);
        c.work(&sh_m, &sh, 70);
        for (id, seed, sc) in [(big_id, 71u64, 1.0f32), (low_id, 79, 0.5), (small_id, 75, 0.45)] {
            let sil = form.silhouette(&[id], |_| 0.6);
            let col = |x: f32, y: f32| match form.sample(x, y).filter(|s| s.part == id) {
                Some(s) => stone(&s),
                None => hex("#3a3632"),
            };
            let ang = |x: f32, y: f32| {
                let s = form.sample(x, y);
                match s {
                    Some(s) if s.n[1] < -0.6 => 0.0,
                    Some(s) => s.fall() + 0.3,
                    None => 0.0,
                }
            };
            // block in, dark, down the planes
            let hd = paint::Handling::new(Tool { width: 7.0 * sc, ..st.body.clone() })
                .mixed(&earth_pal, 0.3)
                .color(|x, y| lift(col(x, y), -0.06))
                .angle(ang)
                .angle_jitter(0.2)
                .length(10.0 * sc, 30.0 * sc)
                .coverage(3.0)
                .pressure(0.6, 0.9)
                .dips(2, 0.56, 0.6)
                .clip(true)
                .threshold(0.2);
            c.work(&sil, &hd, seed);
            // the lit planes in stiffer, lighter paint
            let lit = form.mask(|s| if s.part == id { s.shade.lit(0.15) } else { 0.0 }).mul(&sil);
            let hd = paint::Handling::new(Tool { width: 4.5 * sc, ..st.body.clone() })
                .mixed(&earth_pal, 0.15)
                .color(col)
                .angle(ang)
                .angle_jitter(0.15)
                .length(6.0 * sc, 18.0 * sc)
                .coverage(2.4)
                .pressure(0.6, 0.9)
                .dips(2, 0.6, 0.7)
                .clip(true)
                .threshold(0.3);
            c.work(&lit, &hd, seed + 1);
            // fuse lightly
            let soft = paint::Handling::new(Tool { pickup: 0.12, ..Tool::badger(8.0 * sc) })
                .blender()
                .angle(ang)
                .length(10.0 * sc, 25.0 * sc)
                .coverage(1.0)
                .pressure(0.2, 0.3)
                .dips(3, 0.0, 0.9)
                .clip(true);
            c.work(&sil.erode(2.0 * sc), &soft, seed + 2);
            c.dry();
            // crevices and lichen: dark cracks along the concave breaks,
            // then scattered pale-ochre lichen spots on the sky-facing top
            let joints = form.edges(0.8, 3.0 * sc, 2.5 * sc).mul(&Mask::from_fn(f, |x, y| smoothstep(0.3, 0.7, -form.bend(x, y, 2.5 * sc)))).mul(&sil.erode(1.5));
            let hd = paint::Handling::new(Tool::round_sable(1.4 * sc))
                .mixed(pal, 0.15)
                .color(|_, _| hex("#1d1a17"))
                .by_masstone()
                .angle(|x, y| form.edge_angle(x, y, 2.5 * sc))
                .length(4.0 * sc, 12.0 * sc)
                .coverage(0.8)
                .pressure(0.5, 0.8)
                .dips(3, 0.6, 0.7)
                .clip(true)
                .threshold(0.3);
            c.work(&joints, &hd, seed + 3);
        }
        // lichen on the big stone's top
        let mut r = Rng::new(77);
        for i in 0..90 {
            let x = r.range(80.0, 320.0);
            let y = r.range(548.0, 615.0);
            let Some(s) = form.sample(x, y).filter(|s| s.part == big_id) else { continue };
            if s.shade.sky < 0.65 {
                continue;
            }
            let under = c.under(x, y, 1.5);
            let want = mix(under, if s.shade.direct > 0.1 { hex("#b8a676") } else { hex("#6f6e5a") }, 0.45, Mix::Pigment);
            let p = c.aim(&earth_pal, want, (x, y), 1.5, 0.2, 1.0);
            let mut b = held(Tool::round_sable(r.range(1.0, 2.6)), p, 0.45, 7700 + i);
            c.touch(&mut b, &Touch::at(x, y).pressure(r.range(0.4, 0.8)).drag(r.range(-1.0, 1.0), r.range(-0.4, 0.4)), None);
        }
        // the small stones' reflections in the still water: short dark
        // horizontal strokes under them, broken
        for i in 0..14 {
            let x = r.range(792.0, 885.0);
            let y = 546.0 + r.range(0.0, 6.0);
            if y > shore(x) - 1.0 {
                continue;
            }
            let under = c.under(x, y, 2.0);
            let p = c.aim(&sea_pal, mix(under, hex("#3a3a3e"), 0.4, Mix::Light), (x, y), 1.5, 0.3, 1.0);
            let mut b = held(Tool::round_sable(1.3), p, 0.4, 7900 + i);
            let l = r.range(4.0, 12.0);
            stroke(&mut c, &mut b, &[(x - l * 0.5, y), (x + l * 0.5, y)], 0.5, 0.3, (0.2, 0.3), None);
        }
        drop(form);
        c.dry();
    }

    if o.stage("ships", &mut c, &mut rng) {
        // a brig hull-down on the horizon to the right, her sails catching
        // the light from below the horizon; a far sail to the left
        let hull = c.aim(&sea_pal, hex("#2e3038"), (770.0, HORIZON), 1.0, 0.15, 1.5);
        let mut b = held(Tool::round_sable(1.6), hull, 0.6, 81);
        stroke(&mut c, &mut b, &[(757.0, HORIZON - 0.6), (768.0, HORIZON - 1.2), (781.0, HORIZON - 0.8)], 0.7, 0.6, (0.05, 0.1), None);
        // masts
        let mast = c.aim(&sea_pal, hex("#3a3940"), (770.0, HORIZON - 15.0), 0.5, 0.15, 1.5);
        let mut m = held(Tool::rigger(0.35), mast, 0.5, 82);
        stroke(&mut c, &mut m, &[(765.0, HORIZON - 1.0), (765.2, HORIZON - 31.0)], 0.6, 0.3, (0.02, 0.3), None);
        m.reload(mast, 0.5);
        stroke(&mut c, &mut m, &[(774.0, HORIZON - 1.0), (774.1, HORIZON - 27.0)], 0.6, 0.3, (0.02, 0.3), None);
        m.reload(mast, 0.5);
        stroke(&mut c, &mut m, &[(781.0, HORIZON - 1.2), (791.0, HORIZON - 6.0)], 0.5, 0.3, (0.02, 0.3), None);
        // sails: stacked squares on each mast, shaded on the left, lit warm on the right
        for (mx, top, n) in [(765.0f32, 29.0f32, 4), (774.0, 25.0, 4)] {
            for s in 0..n {
                let y0 = HORIZON - top + s as f32 * (top - 5.0) / n as f32;
                let hh = (top - 5.0) / n as f32 * 0.85;
                let w = 3.2 + s as f32 * 0.9;
                for (side, col) in [(-1.0f32, hex("#9c948f")), (1.0, hex("#e9d4b0"))] {
                    let p = c.aim(&sky_pal, col, (mx + side * w * 0.5, y0 + hh * 0.5), 1.0, 0.1, 1.5);
                    let mut sb = held(Tool::round_sable(w * 0.8), p, 0.55, 83 + s as u64);
                    stroke(&mut c, &mut sb, &[(mx + side * w * 0.45, y0), (mx + side * w * 0.5, y0 + hh)], 0.55, 0.5, (0.1, 0.1), None);
                }
            }
        }
        // the far sail
        let far = c.aim(&sky_pal, hex("#b3aba6"), (232.0, HORIZON - 5.0), 1.0, 0.2, 1.2);
        let mut fb = held(Tool::round_sable(2.0), far, 0.5, 88);
        stroke(&mut c, &mut fb, &[(232.0, HORIZON - 10.0), (232.8, HORIZON - 1.5)], 0.6, 0.6, (0.1, 0.1), None);
        let fh = c.aim(&sea_pal, hex("#3a3f4a"), (232.0, HORIZON), 0.5, 0.15, 1.5);
        let mut fb = held(Tool::round_sable(0.9), fh, 0.5, 89);
        stroke(&mut c, &mut fb, &[(228.5, HORIZON - 0.6), (236.0, HORIZON - 0.6)], 0.6, 0.6, (0.1, 0.1), None);
        c.dry();
    }

    if o.stage("poles", &mut c, &mut rng) {
        // the net: a dark, thin veil hung from a rail, open to the sky;
        // folds in vertical strokes, the mesh a few fine crossing lines
        let rail0 = (pole_at(0, rail_y(pole_at(0, 320.0))) - 2.0, rail_y(392.0));
        let rail1 = (pole_at(1, rail_y(458.0)) + 2.0, rail_y(458.0));
        let net_m = Mask::from_fn(f, move |x, y| {
            if x < rail0.0 + 1.5 || x > rail1.0 - 1.5 {
                return 0.0;
            }
            let top = rail_y(x) + 1.0;
            smoothstep(top, top + 2.0, y) * (1.0 - smoothstep(hem(x) - 3.0, hem(x) + 1.0, y))
        });
        let net_col = hex("#3d3833");
        let veil = paint::Handling::new(Tool { lay: 0.45, ..Tool::round_sable(3.0) })
            .mixed(pal, 0.7)
            .color(move |_, _| net_col)
            .by_masstone()
            .angle(|x, _| std::f32::consts::FRAC_PI_2 + 0.05 * (x / 7.0).sin())
            .angle_jitter(0.08)
            .length(30.0, 90.0)
            .coverage(1.3)
            .pressure(0.3, 0.55)
            .dips(3, 0.35, 0.6)
            .clip(true)
            .threshold(0.2);
        c.work(&net_m, &veil, 91);
        // folds: darker verticals where the net bunches
        let mut r = Rng::new(92);
        let fold = pal.paint(hex("#2a2622"), 0.3);
        let mut b = held(Tool::round_sable(1.2), fold, 0.5, 93);
        let mut x = rail0.0 + 2.0;
        while x < rail1.0 - 2.0 {
            b.reload(fold, 0.45);
            let top = rail_y(x) + 1.0;
            let bot = hem(x) - r.range(0.0, 8.0);
            let sway = r.range(-2.0, 2.0);
            stroke(&mut c, &mut b, &[(x, top), (x + sway * 0.4, (top + bot) * 0.5), (x + sway, bot)], r.range(0.4, 0.7), 0.2, (0.05, 0.5), Some(&net_m));
            x += r.range(2.5, 7.0);
        }
        // the mesh: fine diagonal lines, lean
        let mesh = pal.paint(hex("#302b26"), 0.45);
        let mut fine = held(Tool::rigger(0.3), mesh, 0.35, 94);
        for k in 0..28 {
            fine.reload(mesh, 0.3);
            let s = if k % 2 == 0 { 1.0 } else { -1.0 };
            let x0 = rail0.0 + (k as f32 / 28.0) * (rail1.0 - rail0.0) + r.range(-3.0, 3.0);
            let y0 = rail_y(x0) + 2.0;
            stroke(&mut c, &mut fine, &[(x0, y0), (x0 + s * 30.0, y0 + 60.0), (x0 + s * 55.0, y0 + 140.0)], 0.35, 0.3, (0.1, 0.2), Some(&net_m));
        }
        c.dry();
        // the poles: weathered spars, dark, their right side catching the glow
        let pole_dark = hex("#2c2621");
        for i in 0..3 {
            let ((bx, by), ty, _) = poles[i];
            let w = 4.6 - i as f32 * 0.5;
            let p = c.aim(&earth_pal, pole_dark, (bx, (by + ty) * 0.5), w, 0.15, 1.5);
            let mut b = held(Tool { length: w * 0.5, ..Tool::round_sable(w) }, p, 0.7, 100 + i as u64);
            let n = 6;
            let mut pts: Vec<(f32, f32)> = (0..=n).map(|k| {
                let y = by - (by - ty) * k as f32 / n as f32;
                (pole_at(i, y) + r.normal() * 0.35, y)
            }).collect();
            // from the foot up in two loads, the second set down in the wet of the first
            let half = pts.split_off(n / 2);
            let mut first = pts.clone();
            first.push(half[0]);
            stroke(&mut c, &mut b, &first, 0.9, 0.85, (0.0, 0.05), None);
            b.reload(p, 0.7);
            let mut second = vec![pts[pts.len() - 1]];
            second.extend(half);
            stroke(&mut c, &mut b, &second, 0.85, 0.72, (0.0, 0.08), None);
            // the lit edge on the right
            let lit = c.aim(&earth_pal, hex("#9a8166"), (bx + w * 0.3, (by + ty) * 0.5), w * 0.2, 0.1, 1.5);
            let mut lb = held(Tool::round_sable(w * 0.3), lit, 0.5, 110 + i as u64);
            let edge: Vec<(f32, f32)> = (0..=6).map(|k| {
                let y = ty + 3.0 + (by - ty - 30.0) * k as f32 / 6.0;
                (pole_at(i, y) + w * 0.3, y)
            }).collect();
            stroke(&mut c, &mut lb, &edge, 0.5, 0.25, (0.1, 0.4), None);
        }
        // the rail, lashed on: a slightly bowed dark spar
        let rp = c.aim(&earth_pal, hex("#2e2823"), (425.0, rail_y(425.0)), 1.0, 0.15, 1.5);
        let mut rb = held(Tool::round_sable(1.4), rp, 0.6, 130);
        let rope: Vec<(f32, f32)> = (0..=8).map(|k| {
            let x = pole_at(0, 318.0) + (pole_at(1, 324.0) - pole_at(0, 318.0)) * k as f32 / 8.0;
            (x, rail_y(x) - 0.5)
        }).collect();
        stroke(&mut c, &mut rb, &rope, 0.7, 0.7, (0.05, 0.05), None);
        // lashings: a few small dark turns where the rail crosses the poles
        for (i, x) in [(0usize, pole_at(0, 318.0)), (1, pole_at(1, 324.0))] {
            let y = rail_y(x);
            let lp = pal.paint(hex("#221e1a"), 0.1);
            let mut lb = held(Tool::round_sable(1.0), lp, 0.5, 140 + i as u64);
            for k in 0..3 {
                let yy = y - 2.0 + k as f32 * 1.6;
                stroke(&mut c, &mut lb, &[(x - 3.0, yy - 0.8), (x + 3.0, yy + 0.8)], 0.6, 0.6, (0.1, 0.1), None);
            }
        }
        // a line of cork floats along the hem of the net
        for k in 0..9 {
            let x = rail0.0 + 4.0 + k as f32 * (rail1.0 - rail0.0 - 8.0) / 8.0 + r.range(-1.5, 1.5);
            let y = hem(x) - 3.0;
            let p = c.aim(&earth_pal, hex("#5a4632"), (x, y), 1.2, 0.1, 1.5);
            let mut fb = held(Tool::round_sable(2.2), p, 0.5, 150 + k);
            c.touch(&mut fb, &Touch::at(x, y).pressure(0.6).drag(1.0, 0.0), None);
        }
        // their shadows and reflections on the damp sand: the light is behind
        // them, so the shadows fall toward us, long and faint
        for i in 0..3 {
            let ((bx, by), _, _) = poles[i];
            let under = c.under(bx, by + 10.0, 2.0);
            let p = c.aim(&earth_pal, lift(under, -0.08), (bx, by + 10.0), 2.0, 0.4, 0.7);
            let mut sb = held(Tool { lay: 0.5, ..Tool::filbert(3.0) }, p, 0.35, 160 + i as u64);
            stroke(&mut c, &mut sb, &[(bx, by + 1.0), (bx - 10.0, by + 40.0), (bx - 22.0, by + 95.0)], 0.5, 0.15, (0.05, 0.6), None);
        }
        c.dry();
    }

    if o.stage("figure", &mut c, &mut rng) {
        // the woman: a dark, high-waisted gown to the ground, a dull red
        // shawl over her shoulders with its point down her back, hair in a
        // knot; seen from behind, still; a rim of light on her right side
        let (fx, fy, fh) = woman;
        let hand = paint::Hand::new((fx, fy), fh, 170);
        let gown = pal.paint(hex("#27231f"), 0.12);
        let shawl = pal.paint(hex("#4b2b24"), 0.12);
        let hair = pal.paint(hex("#3a2a1f"), 0.1);
        let skin = pal.paint(hex("#b58f75"), 0.1);
        let rim = pal.paint(hex("#c9a37c"), 0.1);
        let p = |u: f32, v: f32| hand.p(u, v);
        // skirt: strokes from the waist down, flaring to the hem
        let mut b = held(Tool::round_sable(0.05 * fh), gown, 0.7, 171);
        for k in 0..7 {
            let t = k as f32 / 6.0 - 0.5;
            b.reload(gown, 0.65);
            stroke(&mut c, &mut b, &[p(t * 0.1, 0.62), p(t * 0.16, 0.3), p(t * 0.22 + 0.005, 0.01)], 0.85, 0.95, (0.0, 0.05), None);
        }
        // hem, a slightly uneven bottom where the gown touches the sand
        b.reload(gown, 0.6);
        stroke(&mut c, &mut b, &[p(-0.115, 0.02), p(0.0, 0.0), p(0.12, 0.015)], 0.8, 0.8, (0.05, 0.05), None);
        // bodice and back
        let mut bb = held(Tool::round_sable(0.06 * fh), gown, 0.7, 172);
        for t in [-0.035f32, 0.0, 0.035] {
            bb.reload(gown, 0.6);
            stroke(&mut c, &mut bb, &[p(t * 1.2, 0.8), p(t, 0.6)], 0.85, 0.85, (0.02, 0.02), None);
        }
        // arms hanging close, slightly bent: dark sleeves
        let mut arm = held(Tool::round_sable(0.035 * fh), gown, 0.6, 173);
        stroke(&mut c, &mut arm, &[p(-0.065, 0.79), p(-0.085, 0.66), p(-0.075, 0.54)], 0.8, 0.7, (0.02, 0.1), None);
        arm.reload(gown, 0.6);
        stroke(&mut c, &mut arm, &[p(0.065, 0.79), p(0.085, 0.66), p(0.073, 0.54)], 0.8, 0.7, (0.02, 0.1), None);
        // shawl: over the shoulders and down to a point at the small of the back
        let mut sh = held(Tool::round_sable(0.05 * fh), shawl, 0.6, 174);
        stroke(&mut c, &mut sh, &[p(-0.075, 0.8), p(-0.03, 0.7), p(0.0, 0.6)], 0.85, 0.5, (0.02, 0.2), None);
        sh.reload(shawl, 0.6);
        stroke(&mut c, &mut sh, &[p(0.075, 0.8), p(0.03, 0.7), p(0.0, 0.6)], 0.85, 0.5, (0.02, 0.2), None);
        sh.reload(shawl, 0.6);
        stroke(&mut c, &mut sh, &[p(-0.07, 0.805), p(0.0, 0.815), p(0.07, 0.805)], 0.8, 0.8, (0.05, 0.05), None);
        // neck and head, hair gathered in a knot on the crown
        let mut nk = held(Tool::round_sable(0.03 * fh), skin, 0.5, 175);
        stroke(&mut c, &mut nk, &[p(0.0, 0.815), p(0.0, 0.855)], 0.7, 0.7, (0.05, 0.05), None);
        let mut hd = held(Tool::round_sable(0.075 * fh), hair, 0.6, 176);
        c.touch(&mut hd, &Touch::at(p(0.0, 0.9).0, p(0.0, 0.9).1).pressure(0.85), None);
        let mut kn = held(Tool::round_sable(0.045 * fh), hair, 0.5, 177);
        c.touch(&mut kn, &Touch::at(p(0.004, 0.945).0, p(0.004, 0.945).1).pressure(0.8), None);
        // the rim of light on her right (sunward) side
        let mut rb = held(Tool::round_sable(0.012 * fh), rim, 0.3, 178);
        stroke(&mut c, &mut rb, &[p(0.028, 0.935), p(0.036, 0.9)], 0.4, 0.2, (0.1, 0.4), None);
        rb.reload(rim, 0.25);
        stroke(&mut c, &mut rb, &[p(0.055, 0.81), p(0.078, 0.795), p(0.09, 0.75)], 0.35, 0.15, (0.1, 0.6), None);
        // her shadow on the sand, falling toward us and to the left
        let under = c.under(fx, fy + 6.0, 3.0);
        let sp = c.aim(&earth_pal, lift(under, -0.07), (fx, fy + 6.0), 3.0, 0.4, 0.7);
        let mut sb = held(Tool { lay: 0.5, ..Tool::filbert(0.12 * fh) }, sp, 0.35, 179);
        stroke(&mut c, &mut sb, &[(fx, fy + 0.5), (fx - 8.0, fy + 18.0), (fx - 16.0, fy + 40.0)], 0.5, 0.15, (0.05, 0.6), None);
        c.dry();
    }

    if o.stage("foreground", &mut c, &mut rng) {
        // flat stones in the near sand, then tufts of marram grass: fine
        // upturning strokes laid last [NG p.56], dark against the sand, a
        // few dry pale blades catching the light
        let mut r = Rng::new(200);
        let stones: [(f32, f32, f32); 7] = [(612.0, 688.0, 11.0), (640.0, 694.0, 6.0), (772.0, 676.0, 14.0), (806.0, 684.0, 7.0), (905.0, 702.0, 16.0), (470.0, 700.0, 8.0), (60.0, 700.0, 12.0)];
        for (i, &(x, y, s)) in stones.iter().enumerate() {
            let i = i as u64;
            let dark = c.aim(&earth_pal, hex("#2b2723"), (x, y), s, 0.15, 1.5);
            let mut b = held(Tool::filbert(s * 0.55), dark, 0.6, 2000 + i);
            stroke(&mut c, &mut b, &[(x - s * 0.5, y + s * 0.05), (x, y + s * 0.12), (x + s * 0.5, y)], 0.9, 0.8, (0.1, 0.2), None);
            b.reload(dark, 0.5);
            stroke(&mut c, &mut b, &[(x - s * 0.4, y - s * 0.12), (x + s * 0.1, y - s * 0.2), (x + s * 0.45, y - s * 0.1)], 0.8, 0.7, (0.1, 0.2), None);
            // the lit top, back and right
            let lit = earth_pal.paint(hex("#8f806c"), 0.15);
            let mut lb = held(Tool::round_sable(s * 0.14), lit, 0.45, 2100 + i);
            stroke(&mut c, &mut lb, &[(x - s * 0.1, y - s * 0.3), (x + s * 0.2, y - s * 0.3), (x + s * 0.48, y - s * 0.14)], 0.55, 0.3, (0.2, 0.5), None);
            // a thin shadow toward us
            let under = c.under(x, y + s * 0.4, 2.0);
            let sp = c.aim(&earth_pal, lift(under, -0.08), (x, y + s * 0.3), 2.0, 0.4, 0.8);
            let mut sb = held(Tool { lay: 0.5, ..Tool::filbert(s * 0.3) }, sp, 0.35, 2200 + i);
            stroke(&mut c, &mut sb, &[(x - s * 0.3, y + s * 0.25), (x - s * 0.9, y + s * 0.5)], 0.5, 0.2, (0.05, 0.5), None);
        }
        let tufts: [(f32, f32, f32, usize); 7] = [(34.0, 632.0, 26.0, 26), (58.0, 640.0, 18.0, 14), (302.0, 612.0, 14.0, 12), (948.0, 660.0, 30.0, 30), (985.0, 668.0, 22.0, 18), (880.0, 706.0, 20.0, 16), (12.0, 690.0, 34.0, 30)];
        let dark = earth_pal.paint(hex("#34322a"), 0.2);
        let olive = earth_pal.paint(hex("#4d4a36"), 0.2);
        let pale = earth_pal.paint(hex("#9a8b68"), 0.2);
        let mut k = 0u64;
        for &(x, y, hgt, n) in &tufts {
            for j in 0..n {
                let pick = r.f();
                let pnt = if pick < 0.45 { dark } else if pick < 0.8 { olive } else { pale };
                let mut b = held(Tool::rigger(r.range(0.35, 0.6)), pnt, 0.45, 2300 + k);
                k += 1;
                let bx = x + r.normal() * hgt * 0.18;
                let by = y + r.range(-1.5, 1.5);
                let ang = -std::f32::consts::FRAC_PI_2 + r.normal() * 0.45 + 0.12;
                let len = hgt * r.range(0.45, 1.05);
                let bend = r.range(0.05, 0.35) * if j % 3 == 0 { -1.0 } else { 1.0 };
                let (dx, dy) = (ang.cos(), ang.sin());
                let pts: Vec<(f32, f32)> = (0..4).map(|q| {
                    let t = q as f32 / 3.0;
                    let droop = bend * t * t * len;
                    (bx + dx * len * t + droop, by + dy * len * t + droop.abs() * 0.4)
                }).collect();
                stroke(&mut c, &mut b, &pts, 0.7, 0.05, (0.0, 0.7), None);
            }
        }
        c.dry();
    }

    if o.stage("gulls", &mut c, &mut rng) {
        // a few gulls, late touches: small bent strokes, dark against the
        // glow, pale against the upper sky
        let mut r = Rng::new(190);
        let gulls = [(352.0, 262.0, 6.0), (378.0, 248.0, 4.5), (318.0, 281.0, 3.8), (560.0, 300.0, 3.2), (880.0, 244.0, 4.0)];
        for (i, &(x, y, s)) in gulls.iter().enumerate() {
            let under = c.under(x, y, s);
            let want = lift(under, -0.3);
            let p = c.aim(&sky_pal, want, (x, y), 0.5, 0.1, 1.5);
            let mut b = held(Tool::round_sable(0.45 + s * 0.06), p, 0.5, 191 + i as u64);
            let tilt = r.normal() * 0.15;
            let wing = |sx: f32| -> Vec<(f32, f32)> {
                vec![(x, y), (x + sx * s * 0.45, y - s * (0.28 + tilt * sx)), (x + sx * s, y - s * (0.05 + tilt * sx))]
            };
            stroke(&mut c, &mut b, &wing(-1.0), 0.65, 0.2, (0.05, 0.5), None);
            b.reload(p, 0.5);
            stroke(&mut c, &mut b, &wing(1.0), 0.65, 0.2, (0.05, 0.5), None);
        }
        c.dry();
    }

    if o.stage("glaze", &mut c, &mut rng) {
        // a last thin warm veil to knit the foreground, darker toward the
        // bottom edge and the corners (Friedrich's advice to Carus)
        let umber = pal.mix(hex("#4a3a2c")).paint(0.9);
        let pig = umber.pigment();
        c.glaze(&pig, None, move |x, y| {
            let edge = smoothstep(560.0, h, y) * 0.5 + 0.35 * smoothstep(300.0, 0.0, x) * smoothstep(500.0, h, y) + 0.2 * smoothstep(700.0, 1000.0, x) * smoothstep(560.0, h, y);
            edge * 0.9
        });
    }

    let mut fin = Finish::aged(st.relief);
    // a finer, cleaner craquelure than the stock one: at 1000px the stock
    // network reads as a grid laid over the picture
    fin.cracks = Some(paint::Cracks { island_mm: 4.5, width_um: 45.0, depth_um: 22.0, cupping_um: 18.0, dirt: 0.3, ..paint::Cracks::aged(0) });
    o.finish(&mut c, &mut rng, &fin);
}
