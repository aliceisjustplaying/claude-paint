//! Old Oak in Snow, a study: one retrenched oak standing alone on a low
//! rise of snow under a still, clouded winter sky, after a wet snowfall
//! near freezing. In the manner of Caspar David Friedrich, from what is
//! known of his materials and method (notes/research/), not from pictures.
//!
//!   cargo paint r12_tree3 -- --full --width 2400
//!   cargo paint r12_tree3 -- --full --width 2400 --crop x0,y0,x1,y1
//!
//! Notes: notes/r12_tree3.md

use paint::color::{Mix, mix};
use paint::graphite::hand_line;
use paint::{Apply, Fbm, Gesture, Ground, Habit, Held, Lead, Limb, Mask, Paint, Palette, Rgb, Rng, Skeleton, Stipple, Style, Tool, gradient, hex, smoothstep};
use paintings::run::{Finish, Run};

/// width / height: an upright small canvas, 440 × 560 mm
const ASPECT: f32 = 0.786;
/// Where the far snow meets the sky (units from the top).
const HORIZON: f32 = 1010.0;

fn main() {
    let o = Run::new("r12_tree3");
    let mut rng = Rng::new(o.seed);
    // a light bought ground for a winter picture: a warm ochre-and-chalk
    // lower layer to level the weave, a cooler lead-white layer brushed on
    // top whose striations show through thin paint [NG p.55; KÖR p.284]
    let st = Style {
        width_mm: 440.0,
        ground: vec![
            Ground { color: hex("#b89a74"), hiding: 0.8, um: 100.0, stiff: 0.3, apply: Apply::Knife { texture: 0.3 } },
            Ground { color: hex("#d6cfc0"), hiding: 0.8, um: 55.0, stiff: 0.4, apply: Apply::Brush },
        ],
        palette: Palette::friedrich_1820(),
        ..Style::friedrich()
    };
    let pal = &st.palette;
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = c.height();

    // ------------------------------------------------------------ the tree
    // an ancient oak retrenching: its top has died back into grey antlers
    // (stag-headed), the lower crown still lives and ends in a fine net of
    // shoots [ATF; HTC; ROL17]. Grown, not drawn: the engine knows botany.
    let ev = |k: &str, d: f32| std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d);
    let far_n = Fbm::new(3, 4, 160.0);
    let bx = 500.0f32;
    // the rise the oak stands on: its crest under the oak
    let rise = move |x: f32| -> f32 {
        let k = 34.0 * (-((x - bx - 30.0) / 260.0).powi(2)).exp();
        HORIZON + 26.0 - k + 3.0 * far_n.get(x * 0.8, 20.0)
    };
    let base = (bx, rise(bx) + 4.0);
    let oak_h = ev("OAK_H", 800.0);
    let oak = Habit {
        years: ev("OAK_YEARS", 34.0) as u32,
        lean: -0.05,
        decline: ev("OAK_DECLINE", 0.22),
        decay: ev("OAK_DECAY", 0.35),
        breakage: ev("OAK_BREAK", 0.22),
        flat: 0.62,
        trunk: ev("OAK_TRUNK", 0.045),
        twig: ev("OAK_TWIG", 0.0012),
        apical: ev("OAK_APICAL", 0.53),
        vigor: ev("OAK_VIGOR", 2.2),
        shed: ev("OAK_SHED", 0.06),
        node_buds: ev("OAK_BUDS", 2.0) as u32,
        ..Habit::oak()
    }
    .grow(base, oak_h, ev("OAK_SEED", 5.0) as u64 + o.seed);
    let oak = gnarl(oak, 1.6, 11.0, 5);
    {
        let dead = oak.limbs.iter().filter(|l| l.dead).count();
        let fine = oak.limbs.iter().filter(|l| l.order >= 3).count();
        let b = oak.bounds();
        eprintln!("oak: {} limbs ({} dead, {} order>=3), trunk {:.1}, pipe {:.2}, bounds {:?}", oak.limbs.len(), dead, fine, oak.limbs[0].w[0], oak.pipe, b);
        if let Ok(out) = std::env::var("SKEL_ONLY") {
            // a diagnostic silhouette of the grown wood, not the painting
            let mut sc = paint::Canvas::new(700, ASPECT, hex("#ffffff"));
            let m = oak.mask(sc.frame());
            sc.glaze(&paint::Pigment::with_hiding(hex("#000000"), 1.0), Some(&m), |_, _| 4.0);
            sc.save(out).unwrap();
            return;
        }
    }

    // the far edge of the snow
    let snow_top = move |x: f32| HORIZON + 2.0 * far_n.get(x, 1.0) + 0.8 * far_n.get(x * 5.0, 7.0);
    let sky_col = move |x: f32, y: f32| -> Rgb {
        // a still clouded day toward late afternoon: a heavy grey-violet
        // above, clearing low down to a pale yellowish band where the sun
        // stands behind thin cloud, low on the left
        let t = (y / HORIZON).clamp(0.0, 1.0);
        let base = gradient(
            &[(0.0, hex("#75798a")), (0.22, hex("#878a99")), (0.45, hex("#a3a2ab")), (0.66, hex("#bdb6b3")), (0.82, hex("#d3c8b6")), (0.93, hex("#ddd3bb")), (1.0, hex("#d4cfc2"))],
            t,
            Mix::Light,
        );
        // the glow is left of the tree
        let g = (-((x - 150.0) / 420.0).powi(2)).exp() * smoothstep(0.45, 0.95, t);
        mix(base, hex("#e6dcbf"), 0.35 * g, Mix::Light)
    };
    let snow_col = move |x: f32, y: f32| -> Rgb {
        let d = ((y - HORIZON) / (h - HORIZON)).clamp(0.0, 1.0);
        let base = gradient(&[(0.0, hex("#d2cec6")), (0.2, hex("#c6c5c6")), (0.6, hex("#b8b9c2")), (1.0, hex("#aaacba"))], d, Mix::Light);
        // the rise's face turned to the glow (left of its crest) is lighter
        let e = 4.0;
        let slope = (rise(x + e) - rise(x - e)) / (2.0 * e);
        let lit = (-slope * 3.0).clamp(-1.0, 1.0) * (1.0 - smoothstep(rise(x) + 40.0, rise(x) + 110.0, y));
        if lit > 0.0 { mix(base, hex("#e2dccf"), 0.45 * lit, Mix::Light) } else { mix(base, hex("#9d9bb2"), 0.4 * -lit, Mix::Light) }
    };

    let sky_m = Mask::from_fn(f, move |x, y| 1.0 - smoothstep(snow_top(x) + 3.0, snow_top(x) + 6.0, y));
    let snow_m = Mask::from_fn(f, move |x, y| smoothstep(snow_top(x) - 1.0, snow_top(x) + 1.0, y));
    let land_m = Mask::from_fn(f, move |x, y| {
        // a far low wood on the horizon, only at the right, a band of mist
        let top = HORIZON - 7.0 - 5.0 * far_n.get01(x * 3.0, 40.0) - 4.0 * smoothstep(560.0, 1000.0, x);
        smoothstep(top - 0.6, top + 0.6, y) * (1.0 - smoothstep(snow_top(x) + 1.0, snow_top(x) + 3.0, y)) * smoothstep(540.0, 640.0, x)
    });

    // --------------------------------------------------------- underdrawing
    if o.stage("drawing", &mut c, &mut rng) {
        // graphite on the ground: the horizon ruled, the trunk's contours
        // and the big limbs drawn from the study, firmly; the twigs left to
        // the brush [CATS pp.128, 131; NG p.49]
        let hb = Lead::pencil("HB").unwrap();
        let h2 = Lead::pencil("2H").unwrap();
        let mut worn = 0.0;
        worn += c.draw(&h2, &hand_line(&[(0.0, HORIZON), (1000.0, HORIZON)], &[0.3, 0.3], false, true, 0.0, 1), worn, 2);
        let mut k = 10u64;
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && !l.root && l.w[0] > 3.0) {
            // both contours of the limb, where it is wide enough to have two
            for side in [-1.0f32, 1.0] {
                let pts: Vec<(f32, f32)> = (0..l.pts.len())
                    .take_while(|&i| l.w[i] > 2.2)
                    .map(|i| {
                        let d = l.dir(i);
                        (l.pts[i].0 - d.1 * side * l.w[i] * 0.5, l.pts[i].1 + d.0 * side * l.w[i] * 0.5)
                    })
                    .collect();
                if pts.len() >= 2 {
                    k += 1;
                    worn += c.draw(&hb, &hand_line(&pts, &[0.35, 0.45, 0.3], true, false, 0.4, k), worn, k);
                }
            }
        }
        let _ = worn;
    }

    // ------------------------------------------------------------------ sky
    if o.stage("sky", &mut c, &mut rng) {
        // a thin lean lay-in, level strokes, a shade duller than it ends
        let lay = st.broad().color(move |x, y| mix(sky_col(x, y), hex("#8e8b8e"), 0.07, Mix::Light)).angle(|_, _| 0.0).angle_jitter(0.06).coverage(4.0).medium(0.4);
        c.work(&sky_m, &lay, 11);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.angle(|_, _| 0.0), 12);
        }
        // stippled into it wet, aimed at the sky's own tones
        let s1 = Stipple::new(Tool::stippler(2.4)).mixed(pal, 0.45).color(sky_col).coverage(|_, _| 2.0).pressure(0.5, 0.85).dips(20, 0.4, 0.5);
        c.stipple(&sky_m, &s1, 13);
        c.dry();
        // a long low bank of cloud, darker and a little violet, lying
        // across the upper sky and thinning to the left where the light is
        let bank = Fbm::new(21, 4, 220.0);
        let cloud = move |x: f32, y: f32| -> f32 {
            let c1 = 200.0 + 50.0 * bank.get(x * 0.5, 3.0) + 0.12 * (x - 500.0);
            let d = (y - c1) / (70.0 + 30.0 * bank.get01(x, 9.0));
            let body = (1.0 - smoothstep(0.2, 1.0, d.abs())) * smoothstep(0.35, 0.6, bank.get01(x * 1.4, y * 2.0 + 50.0));
            body * smoothstep(80.0, 420.0, x)
        };
        let cl_m = Mask::from_fn(f, move |x, y| if cloud(x, y) > 0.03 { 1.0 } else { 0.0 });
        let cs = Stipple::new(Tool::stippler(2.0))
            .mixed(pal, 0.5)
            .color(move |x, y| mix(sky_col(x, y), hex("#6c6c7c"), 0.3, Mix::Light))
            .coverage(move |x, y| 2.2 * cloud(x, y))
            .pressure(0.45, 0.8)
            .drag(2.0, Some(0.08))
            .dips(20, 0.35, 0.6);
        c.stipple(&cl_m, &cs, 14);
        c.dry();
        // dry: finer, lighter touches building the glow in the low sky
        let glow = move |x: f32, y: f32| mix(sky_col(x, y), hex("#ece2c6"), 0.08 + 0.18 * smoothstep(600.0, HORIZON, y) * (-((x - 170.0) / 380.0).powi(2)).exp(), Mix::Light);
        let s2 = Stipple::new(Tool::stippler(1.4)).mixed(pal, 0.5).color(glow).coverage(|_, y| 1.8 * smoothstep(420.0, HORIZON - 20.0, y)).pressure(0.45, 0.8).dips(24, 0.35, 0.6);
        c.stipple(&sky_m, &s2, 15);
        c.dry();
    }

    // ------------------------------------------------------------- distance
    if o.stage("far", &mut c, &mut rng) {
        // the far wood: a cool blue-grey stipple barely darker than the sky
        let fs = Stipple::new(Tool::stippler(1.5))
            .mixed(pal, 0.45)
            .color(move |x, y| mix(sky_col(x, y), hex("#7f8397"), 0.38, Mix::Light))
            .coverage(|_, _| 2.2)
            .pressure(0.45, 0.8)
            .drag(1.0, Some(-std::f32::consts::FRAC_PI_2))
            .dips(20, 0.4, 0.6);
        c.stipple(&land_m, &fs, 21);
        c.dry();
    }

    // ----------------------------------------------------------------- snow
    if o.stage("snow", &mut c, &mut rng) {
        let slope_angle = move |x: f32, _y: f32| {
            let e = 4.0;
            (-(rise(x + e) - rise(x - e)) / (2.0 * e)).atan().clamp(-0.25, 0.25) * 0.8
        };
        let lay = st.broad().color(move |x, y| mix(snow_col(x, y), hex("#8f8e98"), 0.08, Mix::Light)).angle(slope_angle).coverage(3.5).medium(0.3).clip(true);
        c.work(&snow_m, &lay, 31);
        if let Some(b) = st.blend() {
            c.work(&snow_m, &b.angle(|_, _| 0.0).coverage(2.0), 32);
        }
        c.dry();
        // body: lead white snow, stiffer, a little raised in the near snow
        let body = st.body().color(snow_col).angle(slope_angle).length(25.0, 90.0).mix_jitter(0.03).coverage(2.2).medium(0.16).clip(true).load_at(move |_, y| 0.7 + 0.5 * smoothstep(HORIZON, h, y));
        c.work(&snow_m, &body, 33);
        c.dry();
        // the seam against the sky, stippled so no dark line shows
        let seam = Mask::from_fn(f, move |x, y| {
            let d = y - snow_top(x);
            smoothstep(-5.0, -2.0, d) * (1.0 - smoothstep(3.0, 7.0, d))
        });
        let ss = Stipple::new(Tool::stippler(1.5)).mixed(pal, 0.4).color(move |x, y| mix(snow_col(x, y + 5.0), sky_col(x, HORIZON - 4.0), 0.4, Mix::Light)).coverage(|_, _| 1.8).pressure(0.45, 0.85).dips(20, 0.4, 0.6).aim(false);
        c.stipple(&seam, &ss, 34);
        c.dry();
    }

    // ------------------------------------------------------------------ oak
    let bark_dark = pal.mix(hex("#2a2826")).paint(0.25);
    let bark_dead = pal.mix(hex("#4a4642")).paint(0.2).with_hiding(0.9);
    if o.stage("wood", &mut c, &mut rng) {
        // the snow line cuts the bole and the roots run in under the snow
        let sl = Fbm::new(88, 2, 14.0);
        let above = Mask::from_fn(f, move |x, y| 1.0 - smoothstep(-0.4, 0.4, y - (rise(x) + 3.0 + 3.0 * sl.get(x, 0.0))));
        // the bole and big limbs first, in long strokes laid along them
        // side by side; then every limb pulled from where it springs to its
        // tip, handed down to finer brushes as it thins
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && l.w[0] > 9.0) {
            broad_limb(&mut c, l, bark_dark, bark_dead, Some(&above), &mut rng);
        }
        for l in oak.limbs.iter().filter(|l| !l.is_empty()) {
            limb(&mut c, l, bark_dark, bark_dead, 0.28, Some(&above), &mut rng);
        }
        // the growth model's tips each stand for a cluster of shoots: the
        // last years' twigs added by hand at every live tip, forking and
        // lifting off to hairlines (he added twigs to his studies too)
        let twig = pal.mix(hex("#3b342e")).paint(0.3);
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && !l.root && !l.broken && l.dead_from == l.pts.len() && l.order >= 2) {
            sprays(&mut c, l, twig, &mut rng);
        }
        // an old oak's limbs carry short shoots all along them, stubby
        // twigs of a year or two that never grew out [ROL17; WT]
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && !l.root && l.w[0] > 1.0 && l.w[0] < 14.0) {
            short_shoots(&mut c, l, twig, bark_dead, &mut rng);
        }
        for l in oak.limbs.iter().filter(|l| l.broken && !l.is_empty() && l.w[l.w.len() - 1] > 0.8) {
            splinters(&mut c, l, bark_dead, &mut rng);
        }
        c.dry();
    }

    if o.stage("bark", &mut c, &mut rng) {
        // oak bark: grey, fissured into long blocks [JRC-Q; VT oak]. Over
        // the dry dark, lean broken strokes a shade lighter along the wood
        // where the bark's ridges catch the sky, fuller on the side turned
        // to the glow (left); a cooler, paler grey on the dead wood
        let oak_m = oak.mask(f);
        let ridge_lit = pal.mix(hex("#655f58")).paint(0.3).with_hiding(0.4);
        let ridge_dead = pal.mix(hex("#77726c")).paint(0.3).with_hiding(0.45);
        let rim = pal.mix(hex("#7c7771")).paint(0.3).with_hiding(0.35);
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && !l.root && l.w[0] > 2.2) {
            bark(&mut c, l, ridge_lit, ridge_dead, rim, &oak_m, &mut rng);
        }
        c.dry();
        // fissures: the dark cracks between the ridges, fine rigger lines
        // wandering up the bole and the thick limbs, broken off
        let fissure = pal.mix(hex("#221d1a")).paint(0.25);
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && !l.root && l.w[0] > 6.0) {
            fissures(&mut c, l, fissure, &oak_m, &mut rng);
        }
        c.dry();
    }

    if o.stage("snow on wood", &mut c, &mut rng) {
        // wet snow near 0°C clings: it lies in heaps on rough bark [MIL64
        // p.6], along the upper side of limbs near level and in the forks;
        // steep limbs and springy twigs hold none [MIL66]
        let snow = pal.mix(hex("#d2cfc9")).paint(0.12).with_hiding(0.85);
        let snow_sh = pal.mix(hex("#a6a8b6")).paint(0.12).with_hiding(0.85);
        limb_snow(&mut c, &oak, snow, snow_sh, &mut rng);
        c.dry();
    }

    if o.stage("leaves", &mut c, &mut rng) {
        // young and low oak wood keeps last year's dead leaves through the
        // winter [CDF-EICH]: a few curled brown leaves on the live shoots
        // of the lower crown, none in the dead top
        let leaf = pal.mix(hex("#6e4b2e")).paint(0.25);
        let leaf_lt = pal.mix(hex("#94704a")).paint(0.25);
        let mut b = Held::new(Tool { point: 0.8, ..Tool::round_sable(1.6) }, rng.next_u64());
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && !l.root && l.order >= 3 && l.dead_from > 0) {
            let n = l.pts.len();
            for i in 1..n.min(l.dead_from + 1) {
                let p = l.pts[i];
                let low = smoothstep(base.1 - oak_h * 0.35, base.1 - oak_h * 0.75, p.1);
                if l.w[i] > 1.2 || rng.f() > 0.07 * (1.0 - low) {
                    continue;
                }
                let paint = if rng.f() < 0.3 { leaf_lt } else { leaf };
                b.reload(paint, 0.5);
                // a leaf hangs from the shoot: a short curled drop
                let a = std::f32::consts::FRAC_PI_2 + rng.normal() * 0.7;
                let len = rng.range(1.6, 3.2);
                let q = (p.0 + a.cos() * len, p.1 + a.sin() * len);
                let m = ((p.0 + q.0) * 0.5 + rng.normal() * 0.5, (p.1 + q.1) * 0.5);
                c.drag(&mut b, &Gesture::new(vec![p, m, q]).pressure(0.35, 0.6).ramps(0.2, 0.5).shake(0.5), None);
            }
        }
        c.dry();
    }

    if o.stage("foot", &mut c, &mut rng) {
        let w0 = oak.limbs[0].w[0];
        // the foot swells into buttress roots that dive under the snow:
        // strokes pulled down and out from the bole, pressed as they go
        let mut fb = Held::new(Tool { ragged: 0.3, ..Tool::round_sable(w0 * 0.3) }, rng.next_u64());
        for (side, reach, drop) in [(-1.0f32, 0.95f32, 1.0f32), (1.0, 0.85, 0.9), (-0.35, 0.5, 1.1), (0.45, 0.55, 1.05)] {
            fb.reload(bark_dark, 0.9);
            let top = (base.0 + side * w0 * 0.25, base.1 - w0 * 1.3);
            let mid = (base.0 + side * w0 * (0.3 + 0.25 * reach), base.1 - w0 * 0.45);
            let end = (base.0 + side * w0 * (0.45 + 0.6 * reach), rise(base.0 + side * w0 * (0.45 + 0.6 * reach)) + 2.5 * drop);
            c.drag(&mut fb, &Gesture::new(vec![top, mid, end]).pressure(0.7, 0.95).ramps(0.0, 0.1).shake(0.5), None);
        }
        c.dry();
        // the snow drifted against the bole, stippled over the root ends in
        // the lit snow's colour, thickest against the wood
        let dn = Fbm::new(93, 2, 7.0);
        let top = move |x: f32| rise(x) - 7.0 + 3.5 * dn.get(x, 1.0) + 0.05 * (x - base.0).powi(2) / w0;
        let drift_m = Mask::from_fn(f, move |x, y| if (x - base.0).abs() < w0 * 2.2 && y > top(x) - 2.0 && y < rise(x) + 14.0 { 1.0 } else { 0.0 });
        let ds = Stipple::new(Tool::stippler(1.6))
            .mixed(pal, 0.3)
            .color(move |x, y| mix(snow_col(x, y), hex("#e0dbd1"), 0.3, Mix::Light))
            .coverage(move |x, y| {
                let ends = 1.0 - smoothstep(0.8 * w0, 2.1 * w0, (x - base.0).abs());
                3.6 * smoothstep(top(x) - 1.0, top(x) + 1.5, y) * ends * (1.0 - smoothstep(rise(x) + 4.0, rise(x) + 12.0, y))
            })
            .pressure(0.55, 0.9)
            .dips(16, 0.5, 0.5)
            .aim(false);
        c.stipple(&drift_m, &ds, 51);
        c.dry();
        // oaks shed small twigs cleanly [RUST; WP-CLAD]: a few lying on the
        // snow under the crown, fallen after the snow
        let twig = pal.mix(hex("#3a332c")).paint(0.25);
        let mut b = Held::new(Tool { point: 1.0, ..Tool::rigger(0.7) }, rng.next_u64());
        for _ in 0..9 {
            let x = base.0 + rng.normal() * 150.0;
            let y = rise(x) + rng.range(8.0, 90.0);
            let a = rng.range(-0.5, 0.5) + if rng.f() < 0.5 { 0.0 } else { std::f32::consts::PI };
            let len = rng.range(10.0, 22.0) * (0.6 + 0.6 * (y - HORIZON) / (h - HORIZON));
            let e = (x + a.cos() * len, y + a.sin() * len * 0.4);
            b.reload(twig, 0.5);
            c.drag(&mut b, &Gesture::new(vec![(x, y), ((x + e.0) * 0.5, (y + e.1) * 0.5 + rng.normal() * 0.5), e]).pressure(0.5, 0.05).ramps(0.05, 0.5).shake(0.8), None);
            // a side shoot or two
            for _ in 0..ri(&mut rng, 0, 3) {
                let t = rng.range(0.3, 0.8);
                let s = (x + (e.0 - x) * t, y + (e.1 - y) * t);
                let a2 = a + rng.range(-0.9, 0.9);
                let l2 = len * rng.range(0.2, 0.45);
                c.drag(&mut b, &Gesture::line(s, (s.0 + a2.cos() * l2, s.1 + a2.sin() * l2 * 0.5)).pressure(0.35, 0.02).ramps(0.05, 0.6), None);
            }
        }
        // dry grass stalks through the snow, fine upturning strokes laid
        // over the finished snow [NG p.56], in tufts
        let grass = pal.mix(hex("#7a6a52")).paint(0.25);
        let grass_d = pal.mix(hex("#4f453a")).paint(0.25);
        let mut g = Held::new(Tool { point: 1.0, ..Tool::rigger(0.6) }, rng.next_u64());
        for _ in 0..26 {
            let x = rng.range(40.0, 960.0);
            if (x - base.0).abs() < 50.0 {
                continue;
            }
            let y = rise(x) + rng.range(4.0, 170.0);
            let near = ((y - HORIZON) / (h - HORIZON)).clamp(0.0, 1.0);
            for _ in 0..ri(&mut rng, 3, 9) {
                let p = (x + rng.normal() * 4.0 * (0.5 + near), y + rng.normal() * 1.0);
                let a = -std::f32::consts::FRAC_PI_2 + rng.normal() * 0.35;
                let len = rng.range(5.0, 14.0) * (0.5 + near);
                let q = (p.0 + a.cos() * len, p.1 + a.sin() * len);
                let bend = rng.normal() * 0.2 * len;
                g.reload(if rng.f() < 0.5 { grass } else { grass_d }, 0.5);
                c.drag(&mut g, &Gesture::new(vec![p, ((p.0 + q.0) * 0.5 + bend * 0.4, (p.1 + q.1) * 0.5), (q.0 + bend, q.1)]).pressure(0.45, 0.02).ramps(0.05, 0.6).shake(0.7), None);
            }
        }
        c.dry();
    }

    // no aged finish: the paint as it left the easel, only the raking light
    let fin = Finish { varnish_coats: 0.0, varnish_vary: 0.0, cracks: None, ..Finish::aged(st.relief) };
    o.finish(&mut c, &mut rng, &fin);
}

// ======================================================================
// My hand for the oak.

fn arclen(pts: &[(f32, f32)]) -> Vec<f32> {
    let mut a = vec![0.0; pts.len()];
    for i in 1..pts.len() {
        a[i] = a[i - 1] + ((pts[i].0 - pts[i - 1].0).powi(2) + (pts[i].1 - pts[i - 1].1).powi(2)).sqrt();
    }
    a
}

/// A point on the limb's contour: `u` from −0.5 (one edge) to 0.5 (the other).
fn across(l: &Limb, i: usize, u: f32) -> (f32, f32) {
    let d = l.dir(i);
    (l.pts[i].0 - d.1 * u * l.w[i], l.pts[i].1 + d.0 * u * l.w[i])
}

/// The bole and the thickest limbs: several strokes of a middling round
/// laid side by side along the wood, each following it from the foot as far
/// as the wood stays thick, converging as it tapers.
fn broad_limb(c: &mut paint::Canvas, l: &Limb, live: Paint, dead: Paint, clip: Option<&Mask>, rng: &mut Rng) {
    let n = l.pts.len();
    let w0 = l.w[0];
    let bw = (w0 / 4.0).clamp(3.0, 9.0);
    let lanes = ((w0 / (bw * 0.45)).ceil() as usize).max(2);
    // the lanes stop where the wood dies: the dead top is laid in its own
    // grey by `limb`, not dragged through wet dark
    let end = (0..n).find(|&i| l.w[i] < bw * 1.6).unwrap_or(n - 1).min(if l.dead_from == 0 { n - 1 } else { l.dead_from }).max(1);
    for k in 0..lanes {
        let u = -0.5 + bw * 0.55 / w0 + (1.0 - bw * 1.1 / w0) * k as f32 / (lanes - 1) as f32;
        let pts: Vec<(f32, f32)> = (0..=end).map(|i| across(l, i, u + rng.normal() * 0.01)).collect();
        let paint = if l.dead_at(0) { dead } else { live };
        let mut b = Held::new(Tool { ragged: 0.4, ..Tool::round_sable(bw) }, rng.next_u64());
        b.load(paint, 0.9);
        c.drag(&mut b, &Gesture::new(pts).pressure(1.0, 0.85).ramps(0.0, 0.2).shake(0.35), clip);
    }
}

/// One limb from where it springs to its tip: pointed rounds for the wood,
/// a pointed rigger for the twigs, pressure following the limb's width so it
/// tapers without a step; each brush sets down in the last one's wet end. A
/// live tip lifts off to a hairline; a broken one stops short.
fn limb(c: &mut paint::Canvas, l: &Limb, live: Paint, dead: Paint, finest: f32, clip: Option<&Mask>, rng: &mut Rng) {
    let n = l.pts.len();
    let w: Vec<f32> = l.w.iter().map(|w| w.max(finest)).collect();
    let arc = arclen(&l.pts);
    let mut cuts = vec![0usize];
    for i in 1..n {
        let a = *cuts.last().unwrap();
        if (w[i] < w[a] * 0.5 || arc[i] - arc[a] > 45.0 + 8.0 * w[a] || i == l.dead_from) && i + 1 < n {
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
        let tool = if w[a] > 1.8 {
            Tool { point: 0.6, ragged: 0.3, ..Tool::round_sable(w[a] * 1.15) }
        } else {
            let tw = (w[a] * 1.4).max(0.5);
            Tool { point: 1.0, length: tw * 4.0, ..Tool::rigger(tw) }
        };
        let p0 = tool.pressure_for(w[a]).max(0.08);
        let p1 = if last && !l.broken { 0.0 } else { tool.pressure_for(w[b]).max(0.05) };
        let total = (arc[b] - arc[a0]).max(1e-3);
        let release = if last { if l.broken { 0.05 } else { (w[a].min(4.0) * 3.0 / total).clamp(0.2, 0.6) } } else { 0.05 };
        let paint = if l.dead_at(a) { dead } else { live };
        // the finest twigs lean: a haze of wood, not a mass
        let thin = (l.w[a] / (2.5 * finest)).clamp(0.35, 1.0);
        let mut held = Held::new(tool, rng.next_u64());
        held.load(paint.with_hiding(paint.hiding() * (0.5 + 0.5 * thin)), 0.85 * thin.sqrt());
        c.drag(&mut held, &Gesture::new(pts).pressure(p0, p1).ramps(0.0, release).shake(0.6), clip);
    }
}

/// The youngest shoots at a live tip: two to four short twigs forking off
/// the last nodes, each keeping the limb's heading and bending a little
/// toward the light above, some forking once more; a pointed rigger pressed
/// lightly and lifted off.
fn sprays(c: &mut paint::Canvas, l: &Limb, twig: Paint, rng: &mut Rng) {
    let n = l.pts.len();
    if n < 2 {
        return;
    }
    let mut b = Held::new(Tool { point: 1.0, length: 2.0, ..Tool::rigger(0.55) }, rng.next_u64());
    let shoots = ri(rng, 2, 5);
    for _ in 0..shoots {
        let i = n - 1 - (ri(rng, 0, 3) as usize).min(n - 2);
        let p = l.pts[i];
        let d = l.dir(i.min(n - 2));
        let a0 = d.1.atan2(d.0) + rng.normal() * 0.55;
        let len = rng.range(5.0, 13.0);
        let mut pts = vec![p];
        let mut a = a0;
        let mut q = p;
        for _ in 0..4 {
            // oak shoots zig at each node and turn a little up
            a += rng.normal() * 0.3;
            a += 0.12 * ((-std::f32::consts::FRAC_PI_2) - a).sin().signum() * 0.5;
            q = (q.0 + a.cos() * len / 4.0, q.1 + a.sin() * len / 4.0);
            pts.push(q);
        }
        b.reload(twig, 0.5);
        let p0 = b.tool.pressure_for(l.w[i].clamp(0.25, 0.5));
        c.drag(&mut b, &Gesture::new(pts.clone()).pressure(p0, 0.0).ramps(0.0, 0.6).shake(0.5), None);
        if rng.f() < 0.55 {
            let k = ri(rng, 1, 3) as usize;
            let s = pts[k];
            let a2 = a0 + if rng.f() < 0.5 { 0.6 } else { -0.6 } + rng.normal() * 0.2;
            let l2 = len * rng.range(0.3, 0.55);
            let e = (s.0 + a2.cos() * l2, s.1 + a2.sin() * l2);
            let m = ((s.0 + e.0) * 0.5 + rng.normal() * 0.3, (s.1 + e.1) * 0.5 + rng.normal() * 0.3);
            c.drag(&mut b, &Gesture::new(vec![s, m, e]).pressure(p0 * 0.8, 0.0).ramps(0.0, 0.7).shake(0.5), None);
        }
    }
}

/// Short stubby twigs along a limb, on either side, leaning out toward the
/// light above; on dead wood fewer, and grey.
fn short_shoots(c: &mut paint::Canvas, l: &Limb, live: Paint, dead: Paint, rng: &mut Rng) {
    let n = l.pts.len();
    let mut b = Held::new(Tool { point: 1.0, length: 2.0, ..Tool::rigger(0.5) }, rng.next_u64());
    for i in 1..n.saturating_sub(1) {
        let is_dead = l.dead_at(i);
        if rng.f() > if is_dead { 0.08 } else { 0.25 } || l.w[i] > 9.0 {
            continue;
        }
        let d = l.dir(i);
        let side = if rng.f() < 0.5 { 1.0 } else { -1.0 };
        let mut a = d.1.atan2(d.0) + side * rng.range(0.4, 0.85);
        // turn toward up a little
        let up = -std::f32::consts::FRAC_PI_2;
        a += 0.3 * (up - a).sin().clamp(-1.0, 1.0);
        let t = rng.f();
        let p = (l.pts[i].0 + (l.pts[i + 1].0 - l.pts[i].0) * t, l.pts[i].1 + (l.pts[i + 1].1 - l.pts[i].1) * t);
        let len = rng.range(2.0, 5.5) * if is_dead { 0.6 } else { 1.0 };
        let m = (p.0 + a.cos() * len * 0.5 + rng.normal() * 0.3, p.1 + a.sin() * len * 0.5 + rng.normal() * 0.3);
        let a2 = a + rng.normal() * 0.35;
        let e = (m.0 + a2.cos() * len * 0.5, m.1 + a2.sin() * len * 0.5);
        b.reload(if is_dead { dead } else { live }, 0.45);
        let p0 = b.tool.pressure_for(0.45);
        c.drag(&mut b, &Gesture::new(vec![p, m, e]).pressure(p0, 0.0).ramps(0.0, 0.65).shake(0.5), None);
    }
}

/// A broken end: three or four short flicks out of the stump, one longer,
/// in the dead wood's grey.
fn splinters(c: &mut paint::Canvas, l: &Limb, dead: Paint, rng: &mut Rng) {
    let n = l.pts.len();
    let e = l.pts[n - 1];
    let d = l.dir(n - 2);
    let w = l.w[n - 1];
    let mut b = Held::new(Tool { point: 1.0, ..Tool::rigger((w * 0.35).max(0.5)) }, rng.next_u64());
    for k in 0..ri(rng, 3, 5) {
        let u = rng.range(-0.4, 0.4);
        let s = (e.0 - d.1 * u * w - d.0 * w * 0.3, e.1 + d.0 * u * w - d.1 * w * 0.3);
        let len = w * if k == 0 { rng.range(1.2, 2.2) } else { rng.range(0.4, 1.0) };
        let a = rng.normal() * 0.25;
        let dir = (d.0 * a.cos() - d.1 * a.sin(), d.0 * a.sin() + d.1 * a.cos());
        b.reload(dead, 0.5);
        c.drag(&mut b, &Gesture::line(s, (s.0 + dir.0 * len, s.1 + dir.1 * len)).pressure(0.6, 0.0).ramps(0.0, 0.7), None);
    }
}

/// Lit bark: broken lean strokes along the limb, most on the side turned
/// to the glow; a rim down that edge on the thick wood.
fn bark(c: &mut paint::Canvas, l: &Limb, live: Paint, dead: Paint, rim: Paint, clip: &Mask, rng: &mut Rng) {
    let n = l.pts.len();
    if n < 3 {
        return;
    }
    let streaks = (l.w[0] * 0.7) as usize + 1;
    for _ in 0..streaks {
        // which side is toward the light (left): u < 0 when the limb's
        // normal (−dy, dx) points left
        let i0 = ri(rng, 0, (n as f32 * 0.85) as i64) as usize;
        let d = l.dir(i0);
        let left = if -d.1 < 0.0 { 1.0 } else { -1.0 };
        let u = left * rng.range(-0.15, 0.45).max(-0.45);
        let len = 1 + ri(rng, 0, 4) as usize;
        let e = (i0 + len).min(n - 1);
        if e <= i0 || l.w[i0] < 2.0 {
            continue;
        }
        let pts: Vec<(f32, f32)> = (i0..=e).map(|i| across(l, i, u + rng.normal() * 0.02)).collect();
        let tw = (l.w[i0] * rng.range(0.05, 0.12)).max(0.45);
        let mut b = Held::new(Tool { ragged: 0.7, ..Tool::round_sable(tw) }, rng.next_u64());
        b.load(if l.dead_at(i0) { dead } else { live }, rng.range(0.15, 0.35));
        c.drag(&mut b, &Gesture::new(pts).pressure(rng.range(0.4, 0.7), 0.2).ramps(0.2, 0.4).shake(0.9), Some(clip));
    }
    if l.w[0] > 5.0 {
        let d0 = l.dir(0);
        let left = if -d0.1 < 0.0 { 1.0 } else { -1.0 };
        // broken: a touch here and there down the lit edge, not a line
        let m = (0..n).take_while(|&i| l.w[i] > 3.0).count();
        let mut i = 0;
        while i + 1 < m {
            let e = (i + 1 + ri(rng, 0, 2) as usize).min(m - 1);
            if rng.f() < 0.55 {
                let pts: Vec<(f32, f32)> = (i..=e).map(|k| across(l, k, left * rng.range(0.36, 0.44))).collect();
                let mut b = Held::new(Tool { ragged: 0.7, ..Tool::round_sable((l.w[i] * 0.06).max(0.5)) }, rng.next_u64());
                b.load(rim, 0.25);
                c.drag(&mut b, &Gesture::new(pts).pressure(0.45, 0.2).ramps(0.25, 0.5).shake(0.9), Some(clip));
            }
            i = e + ri(rng, 0, 2) as usize;
        }
    }
}

/// Dark fissures running along the thick wood, wandering and broken.
fn fissures(c: &mut paint::Canvas, l: &Limb, dark: Paint, clip: &Mask, rng: &mut Rng) {
    let n = l.pts.len();
    let count = (l.w[0] * 0.8) as usize + 1;
    let mut b = Held::new(Tool { point: 1.0, ..Tool::rigger(0.8) }, rng.next_u64());
    for _ in 0..count {
        let i0 = ri(rng, 0, (n as f32 * 0.7) as i64) as usize;
        let e = (i0 + 2 + ri(rng, 0, 5) as usize).min(n - 1);
        if e <= i0 || l.w[i0] < 5.0 {
            continue;
        }
        let u = rng.range(-0.4, 0.4);
        let pts: Vec<(f32, f32)> = (i0..=e).map(|i| across(l, i, u + rng.normal() * 0.03)).collect();
        b.reload(dark, 0.5);
        c.drag(&mut b, &Gesture::new(pts).pressure(rng.range(0.3, 0.6), 0.1).ramps(0.1, 0.4).shake(1.0), Some(clip));
    }
}

/// Snow in heaps along the upper side of limbs near level, fatter on the
/// thick wood, gaps where it slid off; a blue-grey shade under each heap's
/// lip on the side away from the light.
fn limb_snow(c: &mut paint::Canvas, sk: &Skeleton, snow: Paint, shade: Paint, rng: &mut Rng) {
    for l in sk.limbs.iter().filter(|l| !l.is_empty() && !l.root && l.order <= 4) {
        let n = l.pts.len();
        let mut i = 0;
        while i + 1 < n {
            let d = l.dir(i);
            let level = d.1.abs() < 0.7;
            if !(l.w[i] > 2.0 && level) || rng.f() < 0.45 {
                i += 1;
                continue;
            }
            // a heap: a run of a few nodes
            let run = ri(rng, 1, 3) as usize;
            let e = (i + run).min(n - 1);
            let up = |k: usize| -> (f32, f32) {
                let d = l.dir(k);
                let nrm = (d.1, -d.0);
                let nrm = if nrm.1 > 0.0 { (-nrm.0, -nrm.1) } else { nrm };
                (l.pts[k].0 + nrm.0 * l.w[k] * 0.3, l.pts[k].1 + nrm.1 * l.w[k] * 0.3)
            };
            let pts: Vec<(f32, f32)> = (i..=e).map(up).collect();
            let wm = (i..=e).map(|k| l.w[k]).sum::<f32>() / (e - i + 1) as f32;
            let tw = (wm * rng.range(0.35, 0.6)).clamp(0.6, 5.0);
            // the shade first, a hair lower, then the lit top over it
            let mut b = Held::new(Tool { point: 0.7, ..Tool::round_sable(tw) }, rng.next_u64());
            b.load(shade, 0.6);
            let low: Vec<(f32, f32)> = pts.iter().map(|p| (p.0 + 0.2 * tw, p.1 + 0.15 * tw)).collect();
            if low.len() >= 2 {
                c.drag(&mut b, &Gesture::new(low).pressure(0.6, 0.4).ramps(0.2, 0.3).shake(0.5), None);
            }
            b.reload(snow, 0.8);
            let hi: Vec<(f32, f32)> = pts.iter().map(|p| (p.0 - 0.05 * tw, p.1 - 0.08 * tw)).collect();
            if hi.len() >= 2 {
                c.drag(&mut b, &Gesture::new(hi).pressure(0.7, 0.6).swell(vec![0.8, 1.1, 0.9, 1.0]).ramps(0.1, 0.15).shake(0.5), None);
            }
            i = e + 1 + ri(rng, 0, 3) as usize;
        }
    }
}

/// Kink a skeleton with a short-period displacement field: old oak wood
/// doesn't run smooth. One field for every point, so twigs stay attached;
/// faded out at the foot so the bole stands in its snow.
fn gnarl(mut sk: Skeleton, amp: f32, period: f32, seed: u32) -> Skeleton {
    let nx = Fbm::new(seed, 2, period);
    let ny = Fbm::new(seed + 1, 2, period);
    let base = sk.base;
    for l in &mut sk.limbs {
        for (i, p) in l.pts.iter_mut().enumerate() {
            let up = smoothstep(0.0, 60.0, base.1 - p.1);
            let a = amp * up * (0.5 + 0.5 * (l.w[i] / 5.0).min(1.0));
            *p = (p.0 + a * nx.get(p.0, p.1), p.1 + a * ny.get(p.0, p.1));
        }
    }
    sk
}

/// A whole number in lo..hi (hi excluded).
fn ri(rng: &mut Rng, lo: i64, hi: i64) -> i64 {
    if hi <= lo { lo } else { lo + ((rng.f() * (hi - lo) as f32) as i64).min(hi - lo - 1) }
}
