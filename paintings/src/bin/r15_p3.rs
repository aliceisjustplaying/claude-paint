//! r15_p3: "Morning on the Baltic shore" — before sunrise, a calm sea
//! under a pale sky; a waning crescent high to the left, the glow of the
//! coming sun right of centre, a woman at the water's edge, net poles in
//! front of a low dune, a granite boulder in the right foreground.

use paint::color::Mix;
use paint::{Apply, Ground, Lead, Mask, Rng, Stipple, Style, Tool, gradient, graphite, hex, smoothstep};
use paintings::run::Run;
use std::f32::consts::FRAC_PI_2;

const ASPECT: f32 = 1.3;
const HZ: f32 = 468.0; // horizon (eye level)
const GX: f32 = 640.0; // the glow's centre on the horizon

// ckpt: from sky
/// The sky as wanted on the canvas at (x, y), y above the horizon.
fn sky(x: f32, y: f32) -> paint::Rgb {
    let t = (y / HZ).clamp(0.0, 1.05);
    let base = gradient(
        &[
            (0.0, hex("#627089")),
            (0.30, hex("#848ea2")),
            (0.58, hex("#aeaca9")),
            (0.80, hex("#d2c6af")),
            (0.93, hex("#e3d1ad")),
            (1.0, hex("#e8d2a8")),
        ],
        t,
        Mix::Pigment,
    );
    // the glow of the sun still below the horizon, right of centre
    let dx = (x - GX) / 300.0;
    let g = (-dx * dx).exp() * smoothstep(0.5, 1.0, t);
    let warm = paint::color::mix(base, hex("#efc393"), 0.55 * g, Mix::Pigment);
    // cooler, a little rosy, far from it (the left horizon)
    let cool = smoothstep(420.0, 60.0, x) * smoothstep(0.55, 1.0, t);
    paint::color::mix(warm, hex("#c3b9b9"), 0.45 * cool, Mix::Pigment)
}
// ckpt: end

fn main() {
    let o = Run::new("r15_p3");
    let mut st = Style::friedrich();
    // a patchy whitish top ground (lead white, a little ochre), brushed
    st.ground[2] = Ground { color: hex("#d9c9ae"), hiding: 0.8, um: 55.0, stiff: 0.4, apply: Apply::Brush };
    let pal = st.palette.clone();
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = c.height();

    // ------------------------------------------------------------ drawing
    if o.stage("drawing", &mut c, &mut rng) {
        let hard = Lead::pencil("2H").unwrap();
        let hb = Lead::pencil("HB").unwrap();
        let mut worn = 0.0;
        // the horizon, ruled
        let m = graphite::hand_line(&[(0.0, HZ), (1000.0, HZ)], &[0.35, 0.4, 0.35], false, true, 0.0, 3);
        worn += c.draw(&hard, &m, worn, 3);
        // water's edge and the dune
        let edge: Vec<(f32, f32)> = (0..=10).map(|i| {
            let x = 120.0 + 88.0 * i as f32;
            (x, shore(x))
        }).collect();
        let m = graphite::hand_line(&edge, &[0.3, 0.35, 0.3], true, false, 0.3, 4);
        worn += c.draw(&hard, &m, worn, 4);
        let m = graphite::hand_line(&dune_pts(), &[0.3, 0.4, 0.35], true, false, 0.4, 5);
        worn += c.draw(&hard, &m, worn, 5);
        // boulder, figure and poles in HB
        let mut w2 = 0.0;
        let m = graphite::hand_line(&boulder_pts(), &[0.4, 0.5, 0.45, 0.4], true, false, 0.4, 6);
        w2 += c.draw(&hb, &m, w2, 6);
        for &(px, top, foot) in POLES.iter() {
            let m = graphite::hand_line(&[(px, foot), (px + 1.5, top)], &[0.45, 0.35], false, true, 0.0, 7);
            w2 += c.draw(&hb, &m, w2, 7);
        }
        let m = graphite::hand_line(&[(FIG_X - 9.0, FIG_FOOT), (FIG_X - 5.0, FIG_HEAD + 14.0), (FIG_X, FIG_HEAD), (FIG_X + 5.0, FIG_HEAD + 14.0), (FIG_X + 11.0, FIG_FOOT)], &[0.4, 0.5, 0.4], true, false, 0.2, 8);
        let _ = c.draw(&hb, &m, w2, 8);
        let _ = worn;
    }

    // ------------------------------------------------------------ sky
    let sky_m = Mask::from_fn(f, |_, y| smoothstep(HZ + 30.0, HZ + 10.0, y));
    if o.stage("sky", &mut c, &mut rng) {
        let lay = st
            .broad()
            .mixed(&pal, 0.3)
            .color(|x, y| sky(x, y))
            .length(160.0, 340.0)
            .coverage(4.0)
            .angle(|_, y| 0.03 * (1.0 - y / HZ))
            .curve(0.02, 0.1)
            .sweep(FRAC_PI_2);
        c.work(&sky_m, &lay, 11);
        // stipple into the wet sky, before the blender, for the grain
        let sp = Stipple::new(Tool::stippler(2.6))
            .mixed(&pal, 0.35)
            .color(|x, y| sky(x, y))
            .coverage(|_, _| 1.4)
            .cluster(0.0, None)
            .dips(8, 0.5, 0.5);
        c.stipple(&sky_m, &sp, 12);
        let bl = st.blend().unwrap().pressure(0.35, 0.45).cross(0.12);
        c.work(&sky_m, &bl, 13);
        let bl = st.blend().unwrap().pressure(0.25, 0.35).coverage(2.0).cross(0.08);
        c.work(&sky_m, &bl, 14);
    }

    // ------------------------------------------------------------ second sky layer, clouds
    if o.stage("sky2", &mut c, &mut rng) {
        c.dry();
        // a second thin layer over the dry first: its gaps now show sky, not ground
        let lay = st
            .broad()
            .mixed(&pal, 0.42)
            .color(|x, y| sky(x, y))
            .length(180.0, 360.0)
            .coverage(3.0)
            .angle(|_, y| 0.03 * (1.0 - y / HZ))
            .curve(0.02, 0.1)
            .sweep(FRAC_PI_2);
        c.work(&sky_m, &lay, 21);
        let bl = st.blend().unwrap().pressure(0.3, 0.4).cross(0.1);
        c.work(&sky_m, &bl, 22);
        let bl = st.blend().unwrap().pressure(0.22, 0.3).coverage(2.0).cross(0.06);
        c.work(&sky_m, &bl, 23);
        // long thin stratus bands into the wet layer: lit salmon from below
        // near the glow, violet-grey away from it
        let bands = Mask::from_fn(f, |x, y| band_at(x, y).0);
        let clouds = paint::Handling::new(Tool { lay: 0.6, ..Tool::filbert(6.0) })
            .mixed(&pal, 0.3)
            .color(|x, y| band_at(x, y).1)
            .length(30.0, 110.0)
            .angle(|x, y| band_at(x, y).2)
            .angle_jitter(0.03)
            .curve(0.02, 0.1)
            .pressure(0.5, 0.75)
            .dips(3, 0.6, 0.5)
            .coverage(2.2)
            .clip(true);
        c.work(&bands, &clouds, 24);
        // their lower rims catch the light of the sun still below the horizon
        let rims = Mask::from_fn(f, |x, y| band_rim(x, y).0);
        let rim = paint::Handling::new(Tool { point: 0.5, ..Tool::round_sable(2.4) })
            .mixed(&pal, 0.25)
            .color(|x, y| band_rim(x, y).1)
            .length(20.0, 70.0)
            .angle(|x, y| band_at(x, y).2)
            .angle_jitter(0.02)
            .pressure(0.4, 0.6)
            .ramps(0.25, 0.35)
            .coverage(1.4)
            .fill(false)
            .clip(true);
        c.work(&rims, &rim, 26);
        // one light pass of the badger along them only
        let near = bands.dilate(6.0).blur(3.0);
        let bl = st.blend().unwrap().pressure(0.18, 0.26).coverage(1.5).cross(0.03).length(60.0, 160.0);
        c.work(&near, &bl, 25);
    }

    // ------------------------------------------------------------ sea
    // Calm water mirrors the sky: at a depression below the horizon it
    // shows the sky from higher up (the facets of the swell tilt toward
    // the eye), darkened by its own body colour as the view steepens.
    let sea_col = move |x: f32, y: f32| -> paint::Rgb {
        let dy = (y - HZ).max(0.0);
        let refl = sky(x, (HZ - 2.0 * dy - 6.0).max(0.0));
        let r = 0.92 - 0.35 * smoothstep(0.0, 110.0, dy);
        let mut col = paint::color::mix(hex("#454a53"), refl, r, Mix::Pigment);
        // the far water: a slightly darker, cooler band under the horizon
        let far = smoothstep(18.0, 2.0, dy);
        col = paint::color::mix(col, hex("#8e8f9a"), 0.45 * far, Mix::Pigment);
        // the glow's path, widening toward us
        let w = 14.0 + 0.8 * dy;
        let p = (-((x - GX) / w).powi(2)).exp() * (1.0 - 0.55 * smoothstep(0.0, 90.0, dy));
        paint::color::mix(col, hex("#ecd3aa"), 0.8 * p, Mix::Pigment)
    };
    let sea_m = Mask::from_fn(f, move |x, y| smoothstep(HZ - 0.5, HZ + 0.5, y) * smoothstep(shore(x) + 14.0, shore(x) + 6.0, y));
    if o.stage("sea", &mut c, &mut rng) {
        c.dry();
        let lay = st
            .broad()
            .mixed(&pal, 0.3)
            .color(sea_col)
            .length(90.0, 240.0)
            .coverage(3.5)
            .angle(|_, _| 0.0)
            .angle_jitter(0.01)
            .curve(0.01, 0.05)
            .drift(0.02, 300.0)
            .clip(true)
            .sweep(FRAC_PI_2);
        c.work(&sea_m, &lay, 31);
        let bl = st.blend().unwrap().pressure(0.3, 0.4).cross(0.02).angle(|_, _| 0.0).curve(0.01, 0.0);
        c.work(&sea_m, &bl, 32);
    }

    // ripples: short level strokes, finer and closer toward the horizon;
    // facets turned up mirror the darker sky above, those turned to us the
    // bright horizon (the glitter under the glow)
    if o.stage("ripples", &mut c, &mut rng) {
        c.dry();
        let bands: [(f32, f32, f32, f32, f32); 4] = [
            (0.0, 14.0, 0.9, 4.0, 12.0),
            (12.0, 34.0, 1.3, 7.0, 22.0),
            (32.0, 62.0, 1.8, 10.0, 34.0),
            (60.0, 140.0, 2.4, 14.0, 50.0),
        ];
        for (k, &(a, b, w, l0, l1)) in bands.iter().enumerate() {
            let m = sea_m.clone().mul_fn(move |_, y| smoothstep(HZ + a - 2.0, HZ + a + 2.0, y) * smoothstep(HZ + b + 2.0, HZ + b - 2.0, y));
            let dark = paint::Handling::new(Tool { point: 0.5, ..Tool::round_sable(w) })
                .mixed(&pal, 0.3)
                .color_over(|_, _, u| paint::shift(u, -0.045, -0.004, -0.012))
                .length(l0, l1)
                .angle(|_, _| 0.0)
                .angle_jitter(0.015)
                .curve(0.03, 0.0)
                .pressure(0.35, 0.6)
                .ramps(0.3, 0.4)
                .coverage(0.35)
                .clump(0.6)
                .fill(false)
                .clip(true);
            c.work(&m, &dark, 40 + k as u64);
            let glint = paint::Handling::new(Tool { point: 0.5, ..Tool::round_sable(w) })
                .mixed(&pal, 0.25)
                .color_over(|_, _, u| paint::shift(u, 0.05, 0.004, 0.018))
                .length(l0 * 0.8, l1 * 0.8)
                .angle(|_, _| 0.0)
                .angle_jitter(0.015)
                .curve(0.02, 0.0)
                .pressure(0.35, 0.6)
                .ramps(0.3, 0.4)
                .coverage(0.5)
                .clump(0.5)
                .fill(false)
                .clip(true)
                .load_at(move |x, y| {
                    let w = 14.0 + 0.8 * (y - HZ);
                    0.15 + 0.85 * (-((x - GX) / (1.3 * w)).powi(2)).exp()
                });
            let path = m.clone().mul_fn(move |x, y| {
                let w = 14.0 + 0.8 * (y - HZ);
                0.25 + 0.75 * (-((x - GX) / (1.3 * w)).powi(2)).exp()
            });
            c.work(&path, &glint, 50 + k as u64);
        }
    }

    // ------------------------------------------------------------ beach and dune
    let dune_y = f.per_column(|x| {
        // the bank as painted: lower and longer than drawn, its foot running
        // out into the beach
        let p = [(-10.0f32, 512.0f32), (60.0, 516.0), (130.0, 527.0), (200.0, 546.0), (260.0, 568.0), (320.0, 592.0), (380.0, 613.0), (440.0, 632.0), (500.0, 648.0), (560.0, 660.0), (620.0, 668.0)];
        let mut y = 1e4;
        for s in p.windows(2) {
            if x >= s[0].0 && x <= s[1].0 {
                let t = (x - s[0].0) / (s[1].0 - s[0].0);
                let t = t * t * (3.0 - 2.0 * t);
                y = s[0].1 + t * (s[1].1 - s[0].1);
            }
        }
        y
    });
    // the water's edge, scalloped by the last small waves
    let wob = paint::Fbm::new(71, 3, 45.0);
    let edge = f.per_column(move |x| shore(x) + 2.2 * wob.get(x, 3.0) + 1.2 * wob.get(x * 3.1, 9.0));
    let beach_m = Mask::from_fn(f, move |x, y| smoothstep(edge(x) - 0.6, edge(x) + 0.6, y));
    // a found crest against the sea, a soft one against the beach, and the
    // foot fading out to the right
    // (roughen re-thresholds a mask to a hard edge, so the crest is
    // roughened first and the soft fade of the foot multiplied in after)
    let dune_m = Mask::from_fn(f, move |x, y| smoothstep(dune_y(x) - 0.8, dune_y(x) + 0.8, y))
        .roughen(61, 14.0, 1.5, 0.8)
        .mul_fn(move |x, y| smoothstep(600.0, 380.0, x + 0.5 * (y - 640.0)));
    let beach_col = move |x: f32, y: f32| -> paint::Rgb {
        let d = (y - edge(x)).max(0.0);
        // the wet sand at the edge still mirrors the sky, darker
        let wet = smoothstep(16.0, 1.0, d);
        let t = ((y - 545.0) / 224.0).clamp(0.0, 1.0);
        let sand = gradient(&[(0.0, hex("#9b9385")), (0.2, hex("#8a8173")), (0.55, hex("#655c50")), (1.0, hex("#3e3831"))], t, Mix::Pigment);
        let mirror = paint::color::mix(sea_col(x, edge(x) - 4.0 - 0.6 * d), hex("#6f6c6a"), 0.45, Mix::Pigment);
        paint::color::mix(sand, mirror, 0.75 * wet, Mix::Pigment)
    };
    if o.stage("beach", &mut c, &mut rng) {
        c.dry();
        let lay = st
            .body()
            .mixed(&pal, 0.25)
            .color(beach_col)
            .length(30.0, 90.0)
            .angle(|x, _| -0.05 + 0.1 * (x / 1000.0))
            .curve(0.05, 0.2)
            .coverage(2.8)
            .clip(true);
        c.work(&beach_m, &lay, 61);
        let bl = st.blend().unwrap().pressure(0.25, 0.35).cross(0.06).angle(|_, _| 0.0).coverage(2.0);
        c.work(&beach_m, &bl, 62);
        // the dune bank over it: brown sand under sparse grass, strokes down its slope
        let dune = st
            .body()
            .mixed(&pal, 0.2)
            .color(move |x, y| {
                let d = (y - dune_y(x)).max(0.0);
                let dc = gradient(&[(0.0, hex("#6c624f")), (0.12, hex("#554c3d")), (0.5, hex("#453e33")), (1.0, hex("#35302a"))], (d / 200.0).clamp(0.0, 1.0), Mix::Pigment);
                let k = smoothstep(560.0, 330.0, x + 0.5 * (y - 640.0));
                paint::color::mix(beach_col(x, y), dc, k, Mix::Pigment)
            })
            .length(24.0, 70.0)
            .angle(|x, _| 0.3 + 0.35 * (x / 450.0))
            .curve(0.06, 0.25)
            .coverage(3.6)
            .dips(1, 0.8, 0.5)
            .clip(true);
        c.work(&dune_m, &dune, 63);
        let bl = st.blend().unwrap().pressure(0.22, 0.3).cross(0.1).angle(|x, _| 0.3 + 0.35 * (x / 450.0)).coverage(1.6);
        c.work(&dune_m, &bl, 64);
    }

    // the beach worked: a lap of foam at the edge, tide ridges, wrack, pebbles
    if o.stage("shore", &mut c, &mut rng) {
        c.wait(240.0);
        // the lapping wave's thin bright lip, broken, and its shadow just below
        let lip = Mask::from_fn(f, move |x, y| {
            let e = edge(x) - 1.2;
            smoothstep(1.3, 0.3, (y - e).abs()) * smoothstep(0.25, 0.5, wob.get01(x * 2.3, 40.0)) * smoothstep(dune_y(x) + 2.0, dune_y(x) - 4.0, y)
        });
        let foam = paint::Handling::new(Tool { point: 0.6, ..Tool::round_sable(1.2) })
            .mixed(&pal, 0.25)
            .color_over(|_, _, u| paint::shift(u, 0.07, -0.002, 0.004))
            .length(8.0, 40.0)
            .angle(|_, _| 0.0)
            .pressure(0.35, 0.6)
            .ramps(0.3, 0.4)
            .coverage(1.6)
            .clip(true);
        c.work(&lip, &foam, 71);
        let shadow = Mask::from_fn(f, move |x, y| smoothstep(1.4, 0.3, (y - edge(x) - 1.6).abs()));
        let sh = paint::Handling::new(Tool { point: 0.6, ..Tool::round_sable(1.4) })
            .mixed(&pal, 0.25)
            .color_over(|_, _, u| paint::shift(u, -0.05, 0.0, -0.01))
            .length(10.0, 50.0)
            .angle(|_, _| 0.0)
            .pressure(0.3, 0.5)
            .ramps(0.3, 0.4)
            .coverage(0.9)
            .fill(false)
            .clip(true);
        c.work(&shadow, &sh, 72);
        // faint tide ridges, level, spaced wider toward us
        let ridge_m = beach_m.clone().mul_fn(move |x, y| {
            let d = y - edge(x);
            let s = (y - HZ).max(1.0);
            let ph = (1.0 / s) * 5200.0 + 0.8 * wob.get(x * 0.5, 70.0);
            smoothstep(12.0, 30.0, d) * smoothstep(0.55, 0.85, ph.sin() * 0.5 + 0.5)
        }).mul(&dune_m.clone().invert());
        let rid = paint::Handling::new(Tool::round_sable(2.4))
            .mixed(&pal, 0.25)
            .color_over(|_, y, u| paint::shift(u, 0.018 - 0.01 * ((y - 560.0) / 200.0).clamp(0.0, 1.0), 0.0, 0.004))
            .length(20.0, 70.0)
            .angle(|_, _| 0.0)
            .angle_jitter(0.02)
            .pressure(0.3, 0.55)
            .coverage(0.8)
            .fill(false)
            .clip(true);
        c.work(&ridge_m, &rid, 73);
        // the wrack line: dry weed thrown up by the last high water
        let wr = move |x: f32| edge(x) + 24.0 + 0.03 * (x - 300.0).max(0.0) + 9.0 * wob.get(x * 0.45, 90.0) + 3.0 * wob.get(x * 2.0, 95.0);
        let wrack_m = Mask::from_fn(f, move |x, y| smoothstep(1.8, 0.6, (y - wr(x)).abs()) * smoothstep(0.42, 0.62, wob.get01(x * 1.1, 120.0))).mul(&dune_m.clone().invert());
        let wk = paint::Handling::new(Tool { point: 0.4, ..Tool::round_sable(2.0) })
            .mixed(&pal, 0.2)
            .color_over(|_, _, u| paint::shift(u, -0.055, 0.0, 0.01))
            .length(4.0, 14.0)
            .angle(|_, _| 0.0)
            .angle_jitter(0.12)
            .curve(0.08, 0.2)
            .pressure(0.35, 0.65)
            .coverage(1.2)
            .fill(false)
            .clip(true);
        c.work(&wrack_m, &wk, 74);
        // pebbles, denser and larger toward us
        let peb_m = beach_m.clone().mul(&dune_m.clone().invert()).mul_fn(move |x, y| smoothstep(edge(x) + 20.0, edge(x) + 60.0, y));
        let sp = Stipple::new(Tool::stippler(2.4))
            .mixed(&pal, 0.2)
            .color_over(|_, _, u| paint::shift(u, -0.07, 0.0, -0.004))
            .coverage(|_, y| 0.05 + 0.25 * ((y - 600.0) / 170.0).clamp(0.0, 1.0))
            .cluster(0.5, Some(30.0))
            .pressure(0.3, 0.8)
            .aim(false);
        c.stipple(&peb_m, &sp, 75);
        c.wait(60.0);
        let sp = Stipple::new(Tool::stippler(1.4))
            .mixed(&pal, 0.2)
            .color_over(|_, _, u| paint::shift(u, 0.05, -0.002, -0.006))
            .coverage(|_, y| 0.03 + 0.12 * ((y - 600.0) / 170.0).clamp(0.0, 1.0))
            .cluster(0.5, Some(30.0))
            .pressure(0.25, 0.55)
            .aim(false);
        c.stipple(&peb_m, &sp, 76);
    }

    // ------------------------------------------------------------ pools
    // shallow water left by the tide lies in the hollows of the sand: flat,
    // it mirrors the sky at the elevation opposite its depression, and the
    // wet sand carries the glow's light on toward us
    let pools: [(f32, f32, f32, f32); 3] = [(512.0, 590.0, 44.0, 1.5), (716.0, 603.0, 66.0, 1.9), (905.0, 566.0, 48.0, 1.3)];
    let pool_m = {
        let n = paint::Fbm::new(151, 3, 25.0);
        Mask::from_fn(f, move |x, y| {
            let mut v = 0.0f32;
            for &(cx, cy, hl, hh) in pools.iter() {
                let u = (x - cx) / hl;
                if u.abs() >= 1.0 {
                    continue;
                }
                let half = hh * (1.0 - u * u).powf(0.7) * (0.7 + 0.6 * n.get01(x, cy));
                let c = cy + 1.2 * n.get(x * 0.8, cy + 50.0);
                v = v.max(smoothstep(half + 0.9, half - 0.7, (y - c).abs()) * smoothstep(1.0, 0.6, u.abs()));
            }
            v
        })
    };
    if o.stage("pools", &mut c, &mut rng) {
        c.dry();
        let refl = move |x: f32, y: f32| paint::color::mix(hex("#5c5752"), sky(x, (2.0 * HZ - y).max(0.0)), 0.6, Mix::Pigment);
        let pl = paint::Handling::new(Tool { lay: 0.6, ..Tool::filbert(3.5) })
            .mixed(&pal, 0.2)
            .color(refl)
            .length(15.0, 50.0)
            .angle(|_, _| 0.0)
            .angle_jitter(0.01)
            .curve(0.01, 0.0)
            .pressure(0.55, 0.8)
            .coverage(3.0)
            .clip(true);
        c.work(&pool_m, &pl, 161);
        // the glow's light carried across the wet sand, right of the figure
        let streak = Mask::from_fn(f, move |x, y| {
            let d = y - edge(x);
            let w = 16.0 + 0.5 * d.max(0.0);
            smoothstep(-1.0, 1.0, d) * smoothstep(34.0, 4.0, d) * (-((x - GX + 6.0) / w).powi(2)).exp()
        });
        let st_ = paint::Handling::new(Tool { point: 0.4, ..Tool::round_sable(2.4) })
            .mixed(&pal, 0.25)
            .color_over(|_, _, u| paint::shift(u, 0.06, 0.006, 0.02))
            .length(8.0, 30.0)
            .angle(|_, _| 0.0)
            .pressure(0.35, 0.6)
            .coverage(1.2)
            .fill(false)
            .clip(true);
        c.work(&streak, &st_, 162);
    }

    // ------------------------------------------------------------ the boulder
    // a granite erratic, bedded in the sand; the light comes from the glow
    // behind it to the left, the sky lights its upper planes
    let bury = move |x: f32| 740.0 + 9.0 * wob.get(x * 0.8, 150.0) + 3.0 * wob.get(x * 3.0, 160.0) + 0.03 * (x - 850.0);
    if o.stage("boulder", &mut c, &mut rng) {
        use paint::{Form, Light, Sdf};
        c.dry();
        let mut form = Form::new(f);
        // the main mass high left of centre, a lower shoulder to the right
        let rock = Sdf::ellipsoid([815.0, 708.0, 0.0], [140.0, 118.0, 110.0])
            .turn([815.0, 708.0, 0.0], 0.2, 0.0, 0.18)
            .union(Sdf::ellipsoid([955.0, 718.0, -10.0], [95.0, 62.0, 80.0]).turn([955.0, 718.0, 0.0], -0.3, 0.0, -0.1), 26.0)
            .cut([800.0, 612.0, 30.0], [-0.25, -0.85, 0.45], 2, 10.0)
            .cut([705.0, 690.0, 40.0], [-0.8, -0.1, 0.6], 3, 16.0)
            .cut([870.0, 700.0, 95.0], [0.15, 0.05, 1.0], 4, 12.0)
            .rough(7.0, 80.0, 9, true)
            .rough(2.5, 20.0, 10, false);
        let id = form.add(&rock, 30.0);
        let small = Sdf::ellipsoid([585.0, 726.0, 0.0], [46.0, 22.0, 30.0]).turn([585.0, 726.0, 0.0], 0.0, 0.0, -0.12).cut([580.0, 712.0, 10.0], [-0.2, -0.9, 0.4], 2, 6.0).rough(3.0, 26.0, 12, true);
        let id2 = form.add(&small, 30.0);
        form.light(Light::new((-1.0, -0.35), -0.5).ambient(0.35).penumbra(0.12));
        let shade_col = |s: &paint::form::Sample| -> paint::Rgb {
            let sky = s.shade.sky;
            // the face toward us in shadow, a little cool from the sky
            // behind the viewer; only the crown sees the bright sky
            let mut col = paint::color::mix(hex("#2f2a26"), hex("#3b3937"), smoothstep(0.2, 0.6, s.n[2]), Mix::Pigment);
            col = paint::color::mix(col, hex("#5f5b56"), 0.85 * smoothstep(0.62, 0.95, sky), Mix::Pigment);
            paint::color::mix(col, hex("#8f7c6c"), 0.75 * smoothstep(0.45, 0.85, s.shade.direct), Mix::Pigment)
        };
        let fs = &form;
        let sil = form.silhouette(&[id, id2], |_| 0.7).mul_fn(move |x, y| smoothstep(bury(x) + 1.0, bury(x) - 1.0, y));
        let body = paint::Handling::new(Tool { lay: 0.7, ..Tool::filbert(6.0) })
            .mixed(&pal, 0.15)
            .color(move |x, y| fs.sample(x, y).map(|s| shade_col(&s)).unwrap_or(hex("#2e2a27")))
            .angle(move |x, y| fs.across(x, y))
            .angle_jitter(0.12)
            .length(12.0, 40.0)
            .curve(0.1, 0.3)
            .pressure(0.6, 0.9)
            .coverage(3.0)
            .clip(true);
        c.work(&sil, &body, 81);
        // a light blend along the planes, then broken texture across them
        let bl = st.blend().unwrap().pressure(0.2, 0.28).coverage(1.4).angle(move |x, y| fs.across(x, y)).length(30.0, 80.0);
        c.work(&sil, &bl, 82);
        c.wait(300.0);
        let tex = paint::Handling::new(Tool { point: 0.5, ..Tool::round_sable(2.2) })
            .mixed(&pal, 0.15)
            .color_over(|x, y, u| paint::shift(u, 0.035 * (paint::noise::rand01((x * 0.4) as i32, (y * 0.4) as i32, 5) - 0.55), 0.0, 0.0))
            .angle(move |x, y| fs.fall(x, y))
            .angle_jitter(0.5)
            .length(3.0, 12.0)
            .curve(0.2, 0.5)
            .pressure(0.35, 0.7)
            .coverage(0.9)
            .fill(false)
            .clip(true);
        c.work(&sil, &tex, 83);
        // the lit edge toward the glow
        let rim = form.mask(move |s| if s.part == id || s.part == id2 { smoothstep(0.5, 0.8, s.shade.direct) } else { 0.0 }).mul(&sil);
        let rl = paint::Handling::new(Tool { point: 0.6, ..Tool::round_sable(2.0) })
            .mixed(&pal, 0.15)
            .color_over(|_, _, u| paint::shift(u, 0.05, 0.006, 0.014))
            .angle(move |x, y| fs.across(x, y))
            .length(6.0, 20.0)
            .pressure(0.35, 0.6)
            .coverage(1.2)
            .fill(false)
            .clip(true);
        c.work(&rim, &rl, 84);

        // sand thrown against its foot, and its soft shadow toward us
        c.wait(120.0);
        let foot = Mask::from_fn(f, move |x, y| {
            let inr = smoothstep(670.0, 700.0, x) + smoothstep(640.0, 620.0, x) * smoothstep(530.0, 545.0, x);
            smoothstep(bury(x) - 4.0, bury(x) + 2.0, y) * smoothstep(bury(x) + 30.0, bury(x) + 12.0, y) * inr.min(1.0)
        });
        let sand = st.body().mixed(&pal, 0.2).color(beach_col).angle(|_, _| 0.0).length(15.0, 45.0).coverage(2.0).clip(true);
        c.work(&foot, &sand, 85);
        let shadow = Mask::from_fn(f, move |x, y| {
            let d = y - bury(x);
            smoothstep(-2.0, 4.0, d) * smoothstep(26.0, 6.0, d) * smoothstep(690.0, 760.0, x)
        })
        .blur(4.0);
        let sh = st.body().mixed(&pal, 0.3).color_over(|_, _, u| paint::shift(u, -0.05, 0.0, -0.01)).angle(|_, _| 0.0).length(20.0, 60.0).coverage(1.5).fill(false).clip(true).pressure(0.4, 0.6);
        c.work(&shadow, &sh, 86);
    }

    // ------------------------------------------------------------ moon
    // the old moon, a thin crescent turned down-right toward the sun below
    // the horizon
    let (mx, my, mr) = (300.0f32, 150.0f32, 11.0f32);
    if o.stage("moon", &mut c, &mut rng) {
        c.dry();
        let halo = Mask::from_shape(f, paint::Shape::new().circle(mx, my, mr * 2.2)).blur(10.0);
        c.glaze(&paint::Pigment::semi(hex("#cfd2d6")), Some(&halo), |_, _| 0.12);
        let (ux, uy) = ((GX - mx), (HZ + 60.0 - my));
        let l = (ux * ux + uy * uy).sqrt();
        let (ux, uy) = (ux / l, uy / l);
        let disc = Mask::from_shape(f, paint::Shape::new().circle(mx, my, mr));
        let bite = Mask::from_shape(f, paint::Shape::new().circle(mx - 0.38 * mr * ux, my - 0.38 * mr * uy, mr * 0.97));
        let crescent = disc.clone().subtract(&bite).blur(0.4);
        let mut b = paint::Held::new(Tool { point: 0.4, ..Tool::round_sable(2.6) }, 91);
        b.load(pal.paint(hex("#f1ecd8"), 0.1), 0.9);
        // strokes along the arc of the crescent
        for k in 0..3 {
            let rr = mr * (0.92 - 0.1 * k as f32);
            let a0 = uy.atan2(ux);
            let pts: Vec<(f32, f32)> = (0..=12).map(|i| {
                let a = a0 - 1.35 + 2.7 * i as f32 / 12.0;
                (mx + rr * a.cos(), my + rr * a.sin())
            }).collect();
            c.drag(&mut b, &paint::Gesture::new(pts).pressure(0.45, 0.45).ramps(0.2, 0.2), Some(&crescent));
            b.reload(pal.paint(hex("#f1ecd8"), 0.1), 0.9);
        }
        // the ashen light on the dark part, barely there
        let ash = disc.clone().subtract(&crescent).mul(&disc.erode(0.6));
        c.work(&ash, &paint::Handling::new(Tool::round_sable(3.0)).mixed(&pal, 0.4).color_over(|_, _, u| paint::shift(u, 0.005, 0.0, 0.0)).length(4.0, 8.0).coverage(1.0).clip(true).pressure(0.25, 0.35), 92);
    }

    // ------------------------------------------------------------ the ship
    // a brig hull-down on the horizon, under the glow's right flank
    let ship_m = {
        let (sx, sy, k) = (832.0f32, HZ + 0.8, 1.15f32);
        let p = |pts: &[(f32, f32)]| -> Vec<(f32, f32)> { pts.iter().map(|&(x, y)| (sx + k * x, sy + k * y)).collect() };
        let hull = paint::Shape::new().poly(&p(&[(-11.0, -3.2), (12.0, -3.6), (9.0, 0.4), (-9.0, 0.4)]));
        let fore = paint::Shape::new().poly(&p(&[(1.5, -2.8), (2.2, -27.0), (9.5, -25.0), (10.5, -2.8)]));
        let main = paint::Shape::new().poly(&p(&[(-9.5, -2.8), (-8.0, -30.0), (0.5, -28.5), (0.8, -2.8)]));
        let jib = paint::Shape::new().poly(&p(&[(11.0, -3.0), (11.8, -22.0), (19.0, -3.0)]));
        let masts = paint::Shape::new().poly(&p(&[(-8.6, -34.0), (-7.9, -34.0), (-7.9, -3.0), (-8.6, -3.0)])).add(paint::Shape::new().poly(&p(&[(2.4, -31.0), (3.1, -31.0), (3.1, -3.0), (2.4, -3.0)])));
        Mask::from_shape(f, hull.add(fore).add(main).add(jib).add(masts)).blur(0.35)
    };
    if o.stage("ship", &mut c, &mut rng) {
        c.dry();
        let sh = st.detail().mixed(&pal, 0.15).by_masstone().color(|x, y| if y > HZ - 4.0 { hex("#3a373c") } else if x > 836.0 { hex("#6a6265") } else { hex("#5d575e") }).dips(1, 1.0, 0.3).pressure(0.75, 0.95).coverage(6.0).threshold(0.05).length(3.0, 8.0).angle(|_, _| -FRAC_PI_2);
        c.work(&ship_m, &sh, 101);
    }

    // ------------------------------------------------------------ poles and nets
    let net_m = {
        let (a, b) = (POLES[0], POLES[1]);
        let top = move |x: f32| {
            let t = ((x - a.0) / (b.0 - a.0)).clamp(0.0, 1.0);
            (a.1 + 12.0) + t * ((b.1 + 10.0) - (a.1 + 12.0)) + 10.0 * (std::f32::consts::PI * t).sin()
        };
        let n = paint::Fbm::new(111, 3, 20.0);
        Mask::from_fn(f, move |x, y| {
            if x < a.0 + 0.8 || x > b.0 + 0.6 {
                return 0.0;
            }
            let t = ((x - a.0) / (b.0 - a.0)).clamp(0.0, 1.0);
            let bot = top(x) + 34.0 + 22.0 * (std::f32::consts::PI * t).sin().powf(0.5) + 8.0 * n.get(x, 0.0);
            smoothstep(top(x) - 0.6, top(x) + 0.6, y) * smoothstep(bot + 1.0, bot - 3.0, y)
        })
    };
    if o.stage("poles", &mut c, &mut rng) {
        c.dry();
        // the net: a thin dark veil of crossing twine, then its head rope
        let mesh = paint::Handling::new(Tool { point: 0.9, ..Tool::rigger(0.5) })
            .mixed(&pal, 0.2)
            .color(|_, _| hex("#3b3833"))
            .length(8.0, 24.0)
            .angle(|_, _| FRAC_PI_2)
            .cross(0.6)
            .angle_jitter(0.12)
            .curve(0.12, 0.4)
            .pressure(0.25, 0.45)
            .coverage(0.22)
            .fill(false)
            .clip(true);
        c.work(&net_m, &mesh, 111);
        // the veil of the net's mass: a transparent dark glaze, deeper in
        // the folds where it hangs doubled
        let fold = paint::Fbm::new(117, 2, 9.0);
        let folds = net_m.clone().mul_fn(move |x, y| 0.45 + 0.55 * smoothstep(0.35, 0.75, fold.get01(x * 1.6, y * 0.12)));
        c.glaze(&paint::Pigment::transparent(hex("#4b453d")), Some(&folds), |_, _| 0.55);

        // the poles: weathered spars, thicker at the foot, a pointed top
        let mut b = paint::Held::new(Tool { point: 0.5, ..Tool::round_sable(3.2) }, 113);
        for (k, &(px, top, foot)) in POLES.iter().enumerate() {
            b.reload(pal.paint(hex("#2f2b27"), 0.15), 0.9);
            let lean = [1.6, -1.2, 9.0][k];
            let top = top + [0.0, 0.0, 34.0][k];
            let g = paint::Gesture::new(vec![(px + lean, top), (px + 0.5 * lean, (top + foot) * 0.5), (px, foot)])
                .pressure(0.5, 0.85)
                .ramps(0.15, 0.05)
                .shake(0.4);
            c.drag(&mut b, &g, None);
        }
        // head rope and a limp bundle of net on the third pole
        let mut r = paint::Held::new(Tool { point: 0.9, ..Tool::rigger(0.9) }, 114);
        r.reload(pal.paint(hex("#34302b"), 0.15), 1.0);
        let (a, bb) = (POLES[0], POLES[1]);
        let rope: Vec<(f32, f32)> = (0..=10).map(|i| {
            let t = i as f32 / 10.0;
            (a.0 + t * (bb.0 - a.0), (a.1 + 12.0) + t * ((bb.1 + 10.0) - (a.1 + 12.0)) + 10.0 * (std::f32::consts::PI * t).sin())
        }).collect();
        c.drag(&mut r, &paint::Gesture::new(rope).pressure(0.6, 0.6), None);
    }

    // ------------------------------------------------------------ the woman at the water's edge
    let (fx, fy) = (602.0f32, 553.0f32);
    let fig_body = [
        (-2.2f32, -80.5f32), (-6.5, -78.5), (-9.4, -75.5), (-10.8, -70.0), (-10.9, -62.0), (-10.0, -55.0), (-8.4, -50.0), (-9.2, -45.0),
        (-11.4, -32.0), (-13.6, -16.0), (-15.2, -3.0), (-14.0, 0.3), (14.3, 0.3), (15.0, -3.0), (13.2, -16.0), (11.0, -32.0), (9.0, -45.0),
        (8.2, -50.0), (9.8, -55.0), (10.7, -62.0), (10.6, -70.0), (9.2, -75.5), (6.3, -78.5), (2.2, -80.5),
    ];
    let fig_shape = move |dy: f32, flip: bool| {
        let pts: Vec<(f32, f32)> = fig_body.iter().map(|&(x, y)| (fx + x, fy + dy + if flip { -y } else { y })).collect();
        let hy = if flip { fy + dy + 86.0 } else { fy + dy - 86.0 };
        paint::Shape::new().smooth_poly(&pts).add(paint::Shape::new().ellipse(fx + 0.6, hy, 4.7, 5.8))
    };
    let fig_m = Mask::from_shape(f, fig_shape(0.0, false));
    let refl_m = Mask::from_shape(f, fig_shape(0.0, true)).mul_fn(move |_, y| 0.7 * smoothstep(fy + 26.0, fy + 0.5, y)).blur(0.8);
    if o.stage("figure", &mut c, &mut rng) {
        c.dry();
        // her faint reflection in the wet sand first
        c.glaze(&paint::Pigment::transparent(hex("#3d3935")), Some(&refl_m), |_, _| 0.7);
        // where she stands: a small dark contact in the wet sand
        let contact = Mask::from_shape(f, paint::Shape::new().ellipse(fx, fy + 0.6, 16.0, 1.6)).blur(0.8);
        c.glaze(&paint::Pigment::transparent(hex("#2e2b29")), Some(&contact), |_, _| 0.9);
        let dress = st
            .detail()
            .mixed(&pal, 0.12)
            // a dark red shawl in a V down her back, a near-black dress
            .color(move |x, y| {
                if y - fy < -80.0 {
                    hex("#221c1a")
                } else if y - fy < -72.0 + 24.0 * (1.0 - (x - fx - 0.3).abs() / 10.5) {
                    hex("#3f2322")
                } else {
                    hex("#262422")
                }
            })
            .coverage(4.0)
            .threshold(0.05)
            .length(4.0, 14.0)
            .angle(|_, _| FRAC_PI_2)
            .angle_jitter(0.15);
        c.work(&fig_m, &dress, 122);
        // the glow catches her right edge
        let rim = fig_m.clone().subtract(&Mask::from_fn(f, move |x, y| fig_m_sample(&fig_body, fx - 1.5, fy + 0.8, x, y)));
        c.work(&rim, &paint::Handling::new(Tool { point: 0.7, ..Tool::round_sable(1.0) }).mixed(&pal, 0.15).color(|_, _| hex("#8f6c58")).length(4.0, 14.0).angle(|_, _| FRAC_PI_2).coverage(1.6).clip(true).pressure(0.35, 0.55).fill(false), 123);
        // her hair gathered in a knot at the nape
        let bun = Mask::from_shape(f, paint::Shape::new().ellipse(fx + 0.4, fy - 83.2, 3.0, 2.3));
        c.work(&bun, &st.detail().mixed(&pal, 0.15).by_masstone().color(|_, _| hex("#1f1b19")).coverage(4.0).threshold(0.05).length(2.0, 5.0), 124);
    }

    // ------------------------------------------------------------ grass on the dune
    // marram in tufts along the crest, where it stands against the sea, and
    // thinner down the bank; each blade one stroke of a pointed round
    if o.stage("grass", &mut c, &mut rng) {
        c.dry();
        let mut g = Rng::new(o.seed + 700);
        let mut b = paint::Held::new(Tool { point: 0.95, ..Tool::round_sable(1.0) }, 131);
        let dark = pal.paint(hex("#302e24"), 0.15);
        let lit = pal.paint(hex("#555039"), 0.15);
        let mut n = 0;
        let mut tuft = |c: &mut paint::Canvas, b: &mut paint::Held, g: &mut Rng, x0: f32, y0: f32, size: f32| {
            let blades = (6.0 + 14.0 * g.f()) as usize;
            for _ in 0..blades {
                n += 1;
                if n % 5 == 0 {
                    b.reload(if g.chance(0.12) { lit.clone() } else { dark.clone() }, 0.8);
                }
                let x = x0 + g.normal() * 2.5 * size;
                let y = y0 + g.f() * 2.0;
                let len = size * (6.0 + 16.0 * g.f());
                let a = -FRAC_PI_2 + 0.28 * g.normal() + 0.15;
                let bend = 0.25 * g.normal() + 0.12;
                let mid = (x + 0.5 * len * a.cos(), y + 0.5 * len * a.sin());
                let tip = (x + len * (a + bend).cos(), y + len * (a + bend).sin());
                let w = 0.35 + 0.4 * g.f() * size;
                let tool = Tool { point: 0.95, ..Tool::round_sable(1.0) };
                let p0 = tool.pressure_for(w);
                let ge = paint::Gesture::new(vec![(x, y), mid, tip]).pressure(p0, 0.04).ramps(0.05, 0.6).shake(0.3);
                c.drag(b, &ge, None);
            }
        };
        b.reload(dark.clone(), 0.8);
        // along the crest
        let mut x = -5.0;
        while x < 470.0 {
            let y = dune_y(x);
            let size = 1.0 + 0.3 * g.f() - 0.25 * smoothstep(250.0, 470.0, x);
            let yy = y + 2.0 + 3.0 * g.f();
            tuft(&mut c, &mut b, &mut g, x, yy, size);
            x += 4.0 + 16.0 * g.f() * g.f();
        }
        // down the bank, larger toward us
        for _ in 0..26 {
            let cx = g.range(-10.0, 440.0);
            let cy = g.range(dune_y(cx) + 10.0, 780.0);
            let spread = 6.0 + 0.12 * (cy - 500.0);
            for _ in 0..(2 + (g.f() * 7.0) as usize) {
                let x = cx + g.normal() * spread;
                let y = cy + 0.5 * g.normal() * spread;
                if dune_m.sample(x, y) < 0.9 || y < dune_y(x) + 6.0 {
                    continue;
                }
                let size = 0.8 + 1.4 * ((y - 520.0) / 250.0).clamp(0.0, 1.0);
                tuft(&mut c, &mut b, &mut g, x, y, size);
            }
        }
    }

    // ------------------------------------------------------------ gulls
    if o.stage("gulls", &mut c, &mut rng) {
        let mut b = paint::Held::new(Tool { point: 0.9, ..Tool::round_sable(1.2) }, 141);
        for &(x, y, s, tilt) in &[(716.0f32, 262.0f32, 7.0f32, 0.1f32), (742.0, 281.0, 5.5, -0.15), (455.0, 350.0, 3.2, 0.05)] {
            b.reload(pal.paint(hex("#4c4a52"), 0.15), 0.7);
            let (ct, st_) = (tilt.cos(), tilt.sin());
            let r = |u: f32, v: f32| (x + u * ct - v * st_, y + u * st_ + v * ct);
            // an M: wings bowed up from the body, drooping to the tips
            let wing = |d: f32| vec![r(0.0, 0.0), r(0.35 * s * d, -0.35 * s), r(0.7 * s * d, -0.3 * s), r(1.0 * s * d, 0.1 * s)];
            let p0 = Tool { point: 0.9, ..Tool::round_sable(1.2) }.pressure_for(0.9);
            c.drag(&mut b, &paint::Gesture::new(wing(-1.0)).pressure(p0, 0.05).ramps(0.05, 0.5), None);
            c.drag(&mut b, &paint::Gesture::new(wing(1.0)).pressure(p0, 0.05).ramps(0.05, 0.5), None);
        }
    }

    // ------------------------------------------------------------ glazes
    // the last layers: the glow deepened where the sun will come, the
    // upper corners and the foreground darkened toward the edges
    if o.stage("glazes", &mut c, &mut rng) {
        c.dry();
        let glow = Mask::from_fn(f, |x, y| {
            let g = (-((x - GX) / 260.0).powi(2)).exp();
            let above = smoothstep(300.0, 460.0, y) * smoothstep(HZ + 0.5, HZ - 0.5, y);
            let path = smoothstep(HZ - 0.5, HZ + 0.5, y) * smoothstep(HZ + 60.0, HZ, y) * (-((x - GX) / 90.0).powi(2)).exp() * 0.5;
            g * (above + path)
        });
        c.glaze(&paint::Pigment::transparent(hex("#e9a66b")), Some(&glow), |_, _| 0.22);
        let h = c.height();
        let edges = Mask::from_fn(f, move |x, y| {
            let top = smoothstep(260.0, 0.0, y) * (0.35 + 0.65 * ((x - 500.0).abs() / 500.0).powf(1.5));
            let bot = smoothstep(560.0, h, y) * (0.55 + 0.45 * ((x - 560.0).abs() / 560.0).min(1.0));
            (top + bot).min(1.0)
        });
        c.glaze(&paint::Pigment::transparent(hex("#4f4a52")), Some(&edges), |_, y| if y < HZ { 0.2 } else { 0.55 });
        // the bank in the shade of dawn: umber, deeper down and to the left,
        // leaving the crest its skylight
        let bank = dune_m.clone().mul_fn(move |x, y| smoothstep(dune_y(x) + 2.0, dune_y(x) + 40.0, y) * (0.5 + 0.5 * smoothstep(420.0, 0.0, x)) * smoothstep(600.0, 280.0, x + 0.3 * (y - 640.0)));
        c.glaze(&paint::Pigment::transparent(hex("#5b4a37")), Some(&bank), |_, _| 0.45);
    }

    let fin = paintings::run::Finish {
        varnish_coats: 0.22,
        varnish_vary: 0.06,
        cracks: Some(paint::Cracks { width_um: Some(4.0), dirt: 0.12, veil: 0.15, hierarchy: 0.5, patchy: 0.6, depth_um: 8.0, cupping_um: 6.0, ..paint::Cracks::aged(0) }),
        ..paintings::run::Finish::aged(st.relief)
    };
    o.finish(&mut c, &mut rng, &fin);
    let _ = h;
}

// ------------------------------------------------------------ geometry

// ckpt: from sky2
/// Stratus bands: (x0, x1, y at x0, half-thickness, tilt, strength).
const BANDS: [(f32, f32, f32, f32, f32, f32); 6] = [
    (-40.0, 700.0, 409.0, 2.8, -0.004, 0.6),
    (400.0, 1040.0, 399.0, 4.0, -0.008, 0.9),
    (590.0, 1040.0, 429.0, 2.2, -0.004, 0.8),
    (120.0, 520.0, 380.0, 2.0, 0.008, 0.4),
    (-40.0, 470.0, 300.0, 3.0, 0.012, 0.32),
    (560.0, 1040.0, 346.0, 2.6, -0.010, 0.42),
];

/// Cover, colour and stroke angle of the stratus bands at (x, y).
fn band_at(x: f32, y: f32) -> (f32, paint::Rgb, f32) {
    let n = paint::Fbm::new(41, 3, 60.0);
    let mut best = (0.0f32, 0.0f32);
    for (i, &(x0, x1, y0, th, tilt, k)) in BANDS.iter().enumerate() {
        if x < x0 - 5.0 || x > x1 + 5.0 {
            continue;
        }
        let t = ((x - x0) / (x1 - x0)).clamp(0.0, 1.0);
        let fi = i as f32 * 37.0;
        let half = th * (std::f32::consts::PI * t).sin().powf(0.8) * (0.35 + 1.0 * n.get01(x * 1.2, fi));
        let cy = y0 + tilt * (x - x0) + 2.5 * n.get(x * 0.7, fi + 300.0);
        let soft = if y < cy { 3.6 } else { 1.8 };
        // long strands broken into pieces of unequal length
        let gap = smoothstep(0.18, 0.4, n.get01(x / 4.5, fi + 500.0));
        let v = k * gap * smoothstep(half + soft, half - 1.6, (y - cy).abs());
        if v > best.0 {
            best = (v, tilt);
        }
    }
    let g = (-((x - GX) / 250.0).powi(2)).exp() * smoothstep(260.0, 400.0, y);
    let col = paint::color::mix(hex("#a29ba6"), hex("#d8ae9c"), 0.85 * g, Mix::Pigment);
    (best.0, col, best.1.atan())
}

/// The lower rims of the stratus bands, lit from the sun below the
/// horizon: membership and colour.
fn band_rim(x: f32, y: f32) -> (f32, paint::Rgb) {
    let n = paint::Fbm::new(41, 3, 60.0);
    let mut best = 0.0f32;
    for (i, &(x0, x1, y0, th, tilt, k)) in BANDS.iter().enumerate() {
        if x < x0 || x > x1 {
            continue;
        }
        let t = ((x - x0) / (x1 - x0)).clamp(0.0, 1.0);
        let fi = i as f32 * 37.0;
        let half = th * (std::f32::consts::PI * t).sin().powf(0.8) * (0.35 + 1.0 * n.get01(x * 1.2, fi));
        let cy = y0 + tilt * (x - x0) + 2.5 * n.get(x * 0.7, fi + 300.0);
        let d = y - cy;
        let gap = smoothstep(0.18, 0.4, n.get01(x / 4.5, fi + 500.0));
        let v = k * gap * smoothstep(0.1 * half, 0.5 * half, d) * smoothstep(half + 1.2, half - 0.6, d) * smoothstep(0.3, 0.6, n.get01(x * 2.5, fi + 80.0));
        best = best.max(v);
    }
    let g = (-((x - GX) / 230.0).powi(2)).exp();
    (best * (0.35 + 0.65 * g), paint::color::mix(hex("#bfb1b3"), hex("#ebc4a6"), g, Mix::Pigment))
}
// ckpt: end

/// The water's edge.
fn shore(x: f32) -> f32 {
    548.0 + 50.0 * smoothstep(520.0, 120.0, x) - 4.0 * smoothstep(600.0, 1000.0, x)
}

/// The dune's crest, left.
fn dune_pts() -> Vec<(f32, f32)> {
    vec![(-10.0, 500.0), (40.0, 503.0), (100.0, 512.0), (160.0, 530.0), (220.0, 556.0), (280.0, 588.0), (330.0, 612.0), (380.0, 626.0)]
}

/// The big boulder's outline, right foreground.
fn boulder_pts() -> Vec<(f32, f32)> {
    vec![(690.0, 742.0), (700.0, 690.0), (728.0, 640.0), (775.0, 603.0), (835.0, 585.0), (900.0, 590.0), (955.0, 612.0), (1010.0, 640.0)]
}

// ckpt: from figure
/// Whether (x, y) lies inside the figure's body polygon placed at (fx, fy)
/// (a hard test for the rim light's offset copy).
fn fig_m_sample(body: &[(f32, f32)], fx: f32, fy: f32, x: f32, y: f32) -> f32 {
    let mut inside = false;
    let n = body.len();
    for i in 0..n {
        let (x0, y0) = (fx + body[i].0, fy + body[i].1);
        let (x1, y1) = (fx + body[(i + 1) % n].0, fy + body[(i + 1) % n].1);
        if (y0 > y) != (y1 > y) && x < x0 + (y - y0) / (y1 - y0) * (x1 - x0) {
            inside = !inside;
        }
    }
    let head = ((x - fx - 0.6) / 4.7).powi(2) + ((y - fy + 86.0) / 5.8).powi(2) < 1.0;
    if inside || head { 1.0 } else { 0.0 }
}
// ckpt: end
const POLES: [(f32, f32, f32); 3] = [(196.0, 418.0, 652.0), (252.0, 430.0, 640.0), (318.0, 452.0, 626.0)];
const FIG_X: f32 = 468.0;
const FIG_FOOT: f32 = 556.0;
const FIG_HEAD: f32 = 463.0;
