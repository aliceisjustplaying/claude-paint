//! r12_tree1: one tree in winter. An old oak, stag-headed (its top dead and
//! broken, a live crown kept below it), standing alone in snow under a
//! pale winter sky, in the manner of C. D. Friedrich, from what is known of
//! his materials and method (notes/research/friedrich_materials.md,
//! trees.md); no pictures.
//!
//!   cargo paint r12_tree1 -- --full --width 2400
//!   cargo paint r12_tree1 -- --full --width 2400 --crop x0,y0,x1,y1
//!
//! Order of work, as he worked: bought warm ground; a pencil drawing of the
//! tree and horizon; the sky laid thin and stippled; the snow field; the
//! tree painted over the finished sky [ALF p.346]; its light and bark once
//! the dark has set; snow laid on the limbs; small things last (grass over
//! the finished snow [NG p.56]).

use paint::color::mix;
use paint::graphite::hand_line;
use paint::{Fbm, Gesture, Habit, Held, Lead, Limb, Mask, Mix, Orient, Paint, Rng, Skeleton, Stipple, Style, Tool, Touch, gradient, hex, smoothstep, Canvas};

const ASPECT: f32 = 0.78;
const HORIZON: f32 = 1000.0;
const BASE: (f32, f32) = (452.0, 1118.0);
const TREE_H: f32 = 860.0;
/// light from a veiled low sun, upper left (canvas direction)
const LIGHT: (f32, f32) = (-0.8, -0.6);

fn envf(k: &str, d: f32) -> f32 {
    std::env::var(k).ok().and_then(|s| s.parse().ok()).unwrap_or(d)
}

/// The oak's way of growing: an open-grown oak given room on all sides for
/// a long life: weak apical control, wide-angled limbs, a broad crown.
fn habit(decline: f32, years: u32) -> Habit {
    let o = Habit::oak();
    Habit {
        decline,
        decay: 0.5,
        breakage: 0.3,
        lean: -0.04,
        years,
        flat: envf("FLAT", 0.75),
        apical: envf("APICAL", 0.5),
        branch_angle: envf("BANG", 1.15),
        tropism: [envf("TR0", 0.1), envf("TR1", -0.02), -0.05, -0.08],
        trunk: 0.065,
        ..o
    }
}

fn tree_seed() -> u64 {
    std::env::var("TREE_SEED").ok().and_then(|s| s.parse().ok()).unwrap_or(1)
}

fn main() {
    let o = paintings::run::Run::new("r12_tree1");
    let st = Style::friedrich();
    let pal = &st.palette;
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = c.height();

    // the tree: an old open-grown oak, long past its prime: the top dead
    // and broken, the lower crown alive (retrenchment [ATF; HTC])
    let oak = habit(0.12, 34).grow(BASE, TREE_H, tree_seed());
    eprintln!("oak: {} limbs, bounds {:?}, pipe {:.2}", oak.limbs.len(), oak.bounds(), oak.pipe);
    if let Ok(dir) = std::env::var("PROBE") {
        // a sketchbook of candidate oaks: skeleton silhouettes, 12 seeds a
        // sheet, written as PGM for choosing (not a painting)
        let (cw, chh) = (250usize, 320usize);
        let fr = paint::Frame::new(cw, chh, cw as f32 / 1000.0);
        let dec: f32 = std::env::var("DEC").ok().and_then(|s| s.parse().ok()).unwrap_or(0.15);
        let yrs: u32 = std::env::var("YRS").ok().and_then(|s| s.parse().ok()).unwrap_or(30);
        let s0: u64 = std::env::var("S0").ok().and_then(|s| s.parse().ok()).unwrap_or(1);
        let mut sheet = vec![255u8; cw * 6 * chh * 2];
        for k in 0..12u64 {
            let sk = habit(dec, yrs).grow(BASE, TREE_H, s0 + k);
            let bb = sk.bounds();
            eprintln!("seed {}: {} limbs {} dead x {:.0}..{:.0}", s0 + k, sk.limbs.len(), sk.limbs.iter().filter(|l| l.dead).count(), bb.0, bb.2);
            let m = sk.mask(fr);
            let (ox, oy) = ((k as usize % 6) * cw, (k as usize / 6) * chh);
            for y in 0..chh {
                for x in 0..cw {
                    let v = m.at(y * cw + x);
                    sheet[(oy + y) * cw * 6 + ox + x] = (255.0 * (1.0 - v)) as u8;
                }
            }
        }
        let mut out = format!("P5\n{} {}\n255\n", cw * 6, chh * 2).into_bytes();
        out.extend_from_slice(&sheet);
        std::fs::write(format!("{dir}/sheet_{dec}_{yrs}_{s0}.pgm"), out).unwrap();
        return;
    }

    // ---- the sky: cool grey-blue above, through pale grey to a warm
    // cream at the horizon, with a faint rose band just above the land
    let veil = Fbm::new(33, 5, 260.0);
    let sky = move |x: f32, y: f32| {
        let t = (y / HORIZON).clamp(0.0, 1.0);
        // long flat veils of high cloud, lighter and darker by turns,
        // thinning toward the horizon; the veiled sun is low at the left
        let v = veil.get(x * 0.22 + 40.0 * (y / 300.0).sin(), y * 1.6);
        let band = v * 0.22 * (1.0 - smoothstep(0.55, 0.95, t));
        let sun = (1.0 - smoothstep(0.0, 700.0, ((x + 150.0).powi(2) * 0.5 + (y - 820.0).powi(2) * 2.0).sqrt())) * 0.35;
        let base = gradient(
            &[(0.0, hex("#76808f")), (0.3, hex("#939ba6")), (0.62, hex("#bdbfbe")), (0.86, hex("#ddd5c6")), (0.95, hex("#e7d6c6")), (1.0, hex("#ece0c8"))],
            t,
            Mix::Light,
        );
        let base = if band > 0.0 { mix(base, hex("#dcd8cf"), band, Mix::Light) } else { mix(base, hex("#6d7482"), -band * 0.8, Mix::Light) };
        mix(base, hex("#f0e2c6"), sun, Mix::Light)
    };
    let sky_m = Mask::from_fn(f, |_, y| 1.0 - smoothstep(HORIZON + 2.0, HORIZON + 8.0, y));

    // far land: a thin dark band of distant woods on the horizon
    let far_n = Fbm::new(41, 4, 60.0);
    let far_seg = Fbm::new(43, 3, 180.0);
    let far_top = move |x: f32| {
        // woods in stretches, with open country between; crowns bump
        let on = smoothstep(0.08, 0.22, far_seg.get(x, 7.0));
        HORIZON - 1.0 - on * (1.5 + 5.0 * (0.5 + far_n.get(x, 0.0)).clamp(0.0, 1.2))
    };
    let far_m = Mask::from_fn(f, move |x, y| smoothstep(far_top(x) - 0.8, far_top(x) + 0.8, y) * (1.0 - smoothstep(HORIZON + 3.0, HORIZON + 6.0, y)));

    // the snow field
    let snow_m = Mask::from_fn(f, |_, y| smoothstep(HORIZON + 1.0, HORIZON + 3.0, y));
    let drift = Fbm::new(52, 4, 140.0);
    let snow = move |x: f32, y: f32| {
        // depth: 0 at the horizon, 1 at the bottom edge
        let d = ((y - HORIZON) / (h - HORIZON)).clamp(0.0, 1.0);
        // gentle undulations, stretched flat by perspective
        let u = drift.get(x * 0.5, (y - HORIZON) * 2.2 / (0.2 + d));
        let lit = hex("#eeeadd");
        let cool = hex("#b9bfc6");
        let warm_far = hex("#dcd6ca");
        let mut col = mix(warm_far, lit, smoothstep(0.0, 0.25, d), Mix::Light);
        col = mix(col, cool, (0.55 * smoothstep(0.0, 0.5, -u) * smoothstep(0.0, 0.15, d)).min(0.55), Mix::Light);
        // long low drifts across the field: the far side of each rise
        // turned from the light
        let rise = ((y - HORIZON) / (6.0 + 60.0 * d) + 0.8 * drift.get(x * 0.08, 3.0 + d * 5.0)).sin();
        col = mix(col, hex("#c2c6cb"), 0.3 * smoothstep(0.5, 0.95, rise), Mix::Light);
        // the oak's shadow, thrown right and a little toward us across
        // the snow from its foot, soft (the sun is veiled)
        let (bx, by) = BASE;
        let along = (x - bx) * 0.95 + (y - by) * 0.3;
        let across = -(x - bx) * 0.3 + (y - by) * 0.95;
        let reach = smoothstep(-10.0, 30.0, along) * (1.0 - smoothstep(150.0, 520.0, along));
        let width = 16.0 + along.max(0.0) * 0.06;
        let sh = reach * (1.0 - smoothstep(width * 0.4, width * 1.5, across.abs())) * 0.85;
        // the lower snow around the foot, a slight hollow on the shadow side
        let foot = (1.0 - smoothstep(20.0, 70.0, ((x - bx - 25.0).powi(2) / 4.0 + (y - by - 6.0).powi(2)).sqrt())) * 0.35;
        mix(col, hex("#a9b0bb"), (sh + foot).min(0.65), Mix::Light)
    };

    if o.stage("drawing", &mut c, &mut rng) {
        // the drawing on the ground: the horizon against the ruler, the
        // trunk and big limbs searched in with an HB pencil
        let lead = Lead::pencil("HB").unwrap();
        let mut worn = 0.0;
        let m = hand_line(&[(20.0, HORIZON), (980.0, HORIZON)], &[0.35], false, true, 0.0, 3);
        worn += c.draw(&lead, &m, worn, 3);
        for (k, l) in oak.limbs.iter().enumerate().filter(|(_, l)| l.order <= 2 && !l.is_empty() && l.w[0] > 3.0) {
            let s = 100 + k as u64;
            // the outline of the wood on both sides, lightly
            for side in [-1.0f32, 1.0] {
                let pts: Vec<(f32, f32)> = (0..l.pts.len())
                    .map(|i| {
                        let d = l.dir(i);
                        let n = (-d.1, d.0);
                        (l.pts[i].0 + side * n.0 * l.w[i] * 0.5, l.pts[i].1 + side * n.1 * l.w[i] * 0.5)
                    })
                    .collect();
                let m = hand_line(&pts, &[0.35, 0.28], false, false, 0.35, s + (side > 0.0) as u64);
                worn += c.draw(&lead, &m, worn, s);
                if worn > 900.0 {
                    worn = 0.0; // sharpened
                }
            }
        }
    }

    if o.stage("sky", &mut c, &mut rng) {
        // thin lay-in in long horizontal strokes, a shade duller than it
        // will end, fused with the badger
        let lay = st
            .broad()
            .color(move |x, y| mix(sky(x, y), hex("#70737a"), 0.06, Mix::Light))
            .angle(|_, y| 0.03 * (y / 200.0).sin())
            .coverage(4.0)
            .medium(0.3)
            .dips(1, 0.6, 0.6);
        c.work(&sky_m, &lay, 11);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.angle(|_, _| 0.0), 14);
        }
        // first stipple into the wet lay-in, aimed at the sky's tones
        let s1 = Stipple::new(Tool::stippler(3.0)).mixed(pal, 0.45).color(sky).coverage(|_, _| 2.0).pressure(0.5, 0.85).dips(20, 0.4, 0.5);
        c.stipple(&sky_m, &s1, 12);
        c.dry();
        // second, dry: finer and lighter, denser toward the horizon glow
        let glow = move |x: f32, y: f32| mix(sky(x, y), hex("#f1e6cc"), 0.04 + 0.2 * smoothstep(350.0, HORIZON, y), Mix::Light);
        let s2 = Stipple::new(Tool::stippler(1.6)).mixed(pal, 0.5).color(glow).coverage(|_, y| 0.6 + 1.6 * smoothstep(150.0, HORIZON, y)).pressure(0.45, 0.8).dips(24, 0.35, 0.6);
        c.stipple(&sky_m, &s2, 13);
        c.dry();
    }

    if o.stage("land", &mut c, &mut rng) {
        // the snow field: lead white, a touch of ochre in the light and
        // smalt-grey in the hollows, in flat horizontal strokes, fuller
        // (a slight impasto) toward us [NG p.50]
        let lay = st
            .body()
            .color(snow)
            .angle(|_, _| 0.0)
            .angle_jitter(0.05)
            .length(30.0, 90.0)
            .curve(0.03, 0.1)
            .coverage(3.5)
            .medium(0.18);
        c.work(&snow_m, &lay, 21);
        if let Some(b) = st.blend() {
            c.work(&snow_m, &b.angle(|_, _| 0.0).pressure(0.3, 0.4), 22);
        }
        c.dry();
        // far woods on the horizon: short upright hatching of a dull
        // blue-grey, the tone of distance
        let far = st.hatch().color(|x, _| mix(hex("#979ba4"), hex("#a5a5aa"), smoothstep(200.0, 800.0, x), Mix::Light)).angle(|_, _| -1.5).angle_jitter(0.4).length(1.5, 4.0).coverage(2.2).fill(false);
        c.work(&far_m, &far, 23);
        c.dry();
    }

    // paints for the tree
    let bark_dark = pal.mix(hex("#2f2a25")).paint(0.22);
    let bark_mid = pal.mix(hex("#575149")).paint(0.22);
    let bark_lit = pal.mix(hex("#8c877d")).paint(0.25);
    let dead_mid = pal.mix(hex("#6d6860")).paint(0.22);
    let dead_lit = pal.mix(hex("#8e897f")).paint(0.25);
    let wood = pal.mix(hex("#948a78")).paint(0.2);
    let tm = oak.mask(f);

    if o.stage("tree", &mut c, &mut rng) {
        // over the dry sky, trunk first, every limb pulled out of the wet
        // paint of its parent
        let mut r = Rng::new(tree_seed() + 1000);
        for l in oak.limbs.iter().filter(|l| !l.is_empty()) {
            paint_limb(&mut c, l, bark_dark, bark_mid, dead_mid, &mut r);
        }
        // back into the wet: the lit side of the bigger wood, and the dark
        // under the limbs, so each reads as round
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && l.w[0] > 3.0 && !l.root) {
            model_limb(&mut c, l, bark_lit, dead_lit, bark_dark, &mut r);
        }
        // the outer crown: last year's twigs, crooked and clustered at the
        // tips (oak buds crowd at the shoot tip), a haze, not a mass
        let twig = bark_mid.with_hiding(bark_mid.hiding() * 0.7);
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && !l.root && l.dead_from == l.pts.len() && !l.broken) {
            twigs(&mut c, l, twig, &mut r);
        }
        for l in oak.limbs.iter().filter(|l| l.broken && !l.is_empty()) {
            break_end(&mut c, l, dead_mid, wood, &mut r);
        }
        c.dry();
    }

    if o.stage("bark", &mut c, &mut rng) {
        // the old oak's bark: fissures in long broken strokes along the
        // trunk and big limbs, and lean grey on the ridges between them
        let mut r = Rng::new(tree_seed() + 2000);
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && l.w[0] > 9.0 && !l.root) {
            bark(&mut c, l, bark_dark, bark_lit, dead_lit, &tm, &mut r);
        }
        c.dry();
    }

    if o.stage("snow on wood", &mut c, &mut rng) {
        let snow_lit = pal.mix(hex("#f3efe6")).paint(0.12);
        let snow_cool = pal.mix(hex("#c9ccd0")).paint(0.14);
        let mut r = Rng::new(tree_seed() + 3000);
        let lumps = Fbm::new(77, 3, 18.0);
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && !l.root) {
            snow_on_limb(&mut c, l, snow_lit, snow_cool, &lumps, &mut r);
        }
        // crotches hold a little heap
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && !l.root && l.order >= 1) {
            if let Some(p) = l.parent {
                let par = &oak.limbs[p];
                if par.w[l.at.min(par.w.len() - 1)] > 8.0 && l.w[0] > 5.0 && r.f() < 0.35 {
                    crotch(&mut c, par, l, snow_lit, &mut r);
                }
            }
        }
        c.dry();
    }

    if o.stage("foot", &mut c, &mut rng) {
        // the snow banked against the foot and over the roots, then dry
        // grass through it, flicked up over the finished snow [NG p.56]
        let mut r = Rng::new(tree_seed() + 4000);
        let (bx, by) = BASE;
        // the root buttresses first: the trunk swells as it meets the
        // ground and its roots run out and down under the snow
        let tw = oak.limbs[0].w[0];
        let flare = [(-1.0f32, 0.6f32, 1.05f32, 12.0f32), (-1.0, 0.25, 0.6, 9.0), (1.0, 0.55, 1.12, 13.0), (1.0, 0.2, 0.7, 8.0)];
        for (k, &(side, from, to, w)) in flare.iter().enumerate() {
            let a = (bx + side * tw * from * 0.5, by - 26.0 - 3.0 * k as f32);
            // the buttress bulges out before it turns down into the ground
            let m = (bx + side * tw * (from + to) * 0.4, by - 8.0);
            let e = (bx + side * tw * to * 0.62, by + 6.0);
            let paint = if side < 0.0 { bark_mid } else { bark_dark };
            let mut b = Held::new(Tool { point: 0.7, ragged: 0.3, ..Tool::round_sable(w) }, r.next_u64());
            b.load(paint, 0.9);
            c.drag(&mut b, &Gesture::new(vec![a, m, e]).pressure(0.9, 0.55).ramps(0.0, 0.15).shake(0.6), None);
        }
        c.dry();
        let edge = Fbm::new(61, 4, 13.0);
        // the bank rises against the trunk, higher on the windward left
        // and in a tongue up the left buttress; its top edge is uneven
        let top_at = move |x: f32| {
            let dx = (x - bx - 12.0) / 120.0;
            by - 4.0 - 9.0 * (1.0 - (dx * 2.2).powi(2)).max(0.0) + 14.0 * edge.get(x, 0.0) - 7.0 * (-((x - bx + 22.0) / 9.0).powi(2)).exp()
        };
        let mound = Mask::from_fn(f, move |x, y| {
            let dx = (x - bx - 12.0) / 120.0;
            let top = top_at(x);
            let bottom = by + 26.0 - 20.0 * dx.abs();
            smoothstep(top - 1.0, top + 1.5, y) * (1.0 - smoothstep(bottom - 6.0, bottom + 6.0, y)) * (1.0 - smoothstep(0.85, 1.0, dx.abs()))
        });
        let bank = st
            .body()
            .color(move |x, y| {
                let right = smoothstep(bx - 10.0, bx + 60.0, x);
                let lit = mix(hex("#f0ece2"), hex("#e6e2d8"), smoothstep(by - 20.0, by + 20.0, y), Mix::Light);
                mix(lit, hex("#a9b0ba"), 0.55 * right * (1.0 - smoothstep(bx + 60.0, bx + 130.0, x)), Mix::Light)
            })
            .angle(|_, _| 0.0)
            .angle_jitter(0.12)
            .length(8.0, 26.0)
            .curve(0.1, 0.2)
            .coverage(4.0)
            .medium(0.15)
            .clip(true);
        c.work(&mound, &bank, 41);
        // the snow's shadow tucked in under the trunk on the shade side
        let cool = pal.mix(hex("#a3aab5")).paint(0.15);
        let mut sb = Held::new(Tool { ragged: 0.4, ..Tool::round_sable(5.0) }, r.next_u64());
        for k in 0..3 {
            sb.load(cool, 0.45);
            let x0 = bx + 6.0 + k as f32 * 9.0;
            let pts: Vec<(f32, f32)> = (0..4).map(|j| {
                let x = x0 + j as f32 * 4.0;
                (x, top_at(x) + 2.0)
            }).collect();
            c.drag(&mut sb, &Gesture::new(pts).pressure(0.55, 0.15).ramps(0.2, 0.6).shake(0.5), None);
        }
        fallen(&mut c, pal, &mut r);
        grass(&mut c, pal, &mut r);
        c.dry();
    }

    if o.stage("leaves", &mut c, &mut rng) {
        // last year's brown leaves, kept on a few low live twigs through
        // winter (marcescence), as a museum text says of his snowy oak
        // [CDF-EICH]
        let mut r = Rng::new(tree_seed() + 5000);
        leaves(&mut c, pal, &oak, &mut r);
        c.dry();
    }

    if o.stage("crows", &mut c, &mut rng) {
        // crows in the dead wood, and two far off over the fields
        let mut r = Rng::new(tree_seed() + 6000);
        let black = pal.mix(hex("#1c1b1e")).paint(0.15);
        // perches: the broken ends of the biggest dead limbs
        let mut dead: Vec<&Limb> = oak.limbs.iter().filter(|l| l.broken && l.dead_from < l.pts.len() && !l.root && l.w[l.w.len() - 1] > 3.0).collect();
        // far out against the sky, where a crow is seen
        let out = |l: &&Limb| (l.pts[l.pts.len() - 1].0 - BASE.0).abs();
        dead.sort_by(|a, b| out(b).total_cmp(&out(a)));
        let mut used: Vec<(f32, f32)> = vec![];
        let mut k = 0;
        for l in dead.iter() {
            let e = l.pts[l.pts.len() - 1];
            if k >= 2 || used.iter().any(|u| (u.0 - e.0).hypot(u.1 - e.1) < 80.0) {
                continue;
            }
            used.push(e);
            k += 1;
            let k = k - 1;
            let n = l.pts.len();
            let i = n.saturating_sub(2).max(1);
            let d = l.dir(i);
            let up = if -d.0 < 0.0 { (d.1, -d.0) } else { (-d.1, d.0) };
            let at = (l.pts[i].0 + up.0 * l.w[i] * 0.5, l.pts[i].1 + up.1 * l.w[i] * 0.5);
            crow_perched(&mut c, at, if k == 0 { -1.0 } else { 1.0 }, 15.0, black, &mut r);
        }
        crow_flying(&mut c, (140.0, 520.0), 9.0, black, &mut r);
        crow_flying(&mut c, (176.0, 548.0), 7.0, black, &mut r);
        c.dry();
    }

    o.end(&mut c, &mut rng);
    c.relief(st.relief.0, st.relief.1);
    o.save(&mut c);
}

// ---------------------------------------------------------------- helpers

fn cumulative(pts: &[(f32, f32)]) -> Vec<f32> {
    let mut a = vec![0.0f32; pts.len()];
    for i in 1..pts.len() {
        a[i] = a[i - 1] + ((pts[i].0 - pts[i - 1].0).powi(2) + (pts[i].1 - pts[i - 1].1).powi(2)).sqrt();
    }
    a
}

/// A limb resampled every `step` units along its length, straight between
/// its nodes (the elbows of a sympodial oak limb stay elbows), with widths
/// and deadness carried along.
fn resample(l: &Limb, step: f32) -> (Vec<(f32, f32)>, Vec<f32>, Vec<bool>) {
    let arc = cumulative(&l.pts);
    let total = *arc.last().unwrap();
    let n = ((total / step).ceil() as usize).max(1);
    let mut pts = Vec::with_capacity(n + 1);
    let mut w = Vec::with_capacity(n + 1);
    let mut dead = Vec::with_capacity(n + 1);
    let mut j = 0;
    for k in 0..=n {
        let t = total * k as f32 / n as f32;
        while j + 2 < l.pts.len() && arc[j + 1] < t {
            j += 1;
        }
        let seg = (arc[j + 1] - arc[j]).max(1e-6);
        let u = ((t - arc[j]) / seg).clamp(0.0, 1.0);
        pts.push((l.pts[j].0 + (l.pts[j + 1].0 - l.pts[j].0) * u, l.pts[j].1 + (l.pts[j + 1].1 - l.pts[j].1) * u));
        w.push(l.w[j] + (l.w[j + 1] - l.w[j]) * u);
        dead.push(l.dead_at(j));
    }
    (pts, w, dead)
}

fn dir_at(p: &[(f32, f32)], i: usize) -> (f32, f32) {
    let (a, b) = (p[i.saturating_sub(1)], p[(i + 1).min(p.len() - 1)]);
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l = (dx * dx + dy * dy).sqrt().max(1e-6);
    (dx / l, dy / l)
}

/// A pointed round sable (or, for the finest wood, a pointed rigger) whose
/// mark at pressure 0.85 is `w` units wide.
fn brush(w: f32) -> Tool {
    if w > 1.4 {
        let unit = Tool { point: 0.85, ..Tool::round_sable(1.0) };
        let k = w / unit.mark_width(0.85);
        Tool { point: 0.85, ragged: 0.25, ..Tool::round_sable(k) }
    } else {
        let unit = Tool { point: 0.9, ..Tool::rigger(1.0) };
        let k = w / unit.mark_width(0.8);
        Tool { point: 0.9, length: k * 4.0, ..Tool::rigger(k.max(0.2)) }
    }
}

/// Stroke along `pts` with widths `w` using as many brushes as the taper
/// needs; each new brush sets down inside the wet end of the last one.
/// `paint_at(i)` gives the paint for a section starting at `i`; `tip` =
/// lift off to a point at the end (a live tip) or stop blunt.
#[allow(clippy::too_many_arguments)]
fn taper_stroke(c: &mut Canvas, pts: &[(f32, f32)], w: &[f32], split: &[bool], paint_at: &dyn Fn(usize) -> Paint, load: &dyn Fn(f32) -> f32, tip: bool, shake: f32, rng: &mut Rng) {
    let n = pts.len();
    if n < 2 {
        return;
    }
    let arc = cumulative(pts);
    let mut cuts = vec![0usize];
    for i in 1..n {
        let a = *cuts.last().unwrap();
        let run = brush(w[a]).run * 0.75;
        if (w[i] < w[a] * 0.3 || arc[i] - arc[a] > run || split[i] != split[i - 1]) && i + 2 < n {
            cuts.push(i);
        }
    }
    cuts.push(n - 1);
    for k in 0..cuts.len() - 1 {
        let (a, b) = (cuts[k], cuts[k + 1]);
        let first = k == 0;
        let last = k + 2 == cuts.len();
        let ov = w[a] * 1.2 + 1.0;
        let a0 = if first { a } else { (0..a).rev().find(|&i| arc[a] - arc[i] >= ov).unwrap_or(0) };
        let b0 = if last { b } else { (b + 1..n).find(|&i| arc[i] - arc[b] >= ov * 0.5).unwrap_or(n - 1) };
        if b0 <= a0 {
            continue;
        }
        let tool = brush(w[a]);
        let sw: Vec<f32> = (a0..=b0).map(|i| tool.pressure_for(w[i].max(0.25)).clamp(0.04, 1.0)).collect();
        let release = if last && tip { 0.12 } else { 0.0 };
        let mut held = Held::new(tool, rng.next_u64());
        held.load(paint_at(a), load(w[a]));
        let g = Gesture::new(pts[a0..=b0].to_vec()).pressure(1.0, 1.0).swell(sw).ramps(0.0, release).orient(Orient::Across).shake(shake);
        c.drag(&mut held, &g, None);
    }
}

/// One limb in the dark body color, base to tip. The trunk and the big
/// limbs are laid in several strokes side by side along the wood (their
/// grain is the bark's grain), toned from the shadow side to the light.
fn paint_limb(c: &mut Canvas, l: &Limb, dark: Paint, mid: Paint, dead: Paint, rng: &mut Rng) {
    let finest = 0.3;
    let step = (l.w[0] * 0.25).clamp(0.6, 4.0);
    let (mut pts, mut w, mut deadv) = resample(l, step);
    if l.root {
        // roots dive under the snow within a short reach of the trunk
        let arc = cumulative(&pts);
        let keep = arc.iter().position(|&a| a > l.w[0] * 1.3).unwrap_or(pts.len()).max(2);
        pts.truncate(keep);
        w.truncate(keep);
        deadv.truncate(keep);
        for (k, x) in w.iter_mut().enumerate() {
            *x *= 1.0 - 0.75 * k as f32 / keep as f32;
        }
    }
    for x in w.iter_mut() {
        *x = x.max(finest);
    }
    let load = |wa: f32| (0.35 + 0.6 * (wa / 1.2).min(1.0)).min(0.95);
    let paint_for = |i: usize, base: Paint| if deadv[i] { dead } else { base };
    let bands = ((l.w[0] / 8.0).round() as usize).clamp(1, 7);
    if bands == 1 {
        let pa = |i: usize| paint_for(i, dark);
        // the finest twigs lean, a haze rather than a mass
        let lean = |i: usize| {
            let p = pa(i);
            let t = (w[i] / 0.9).clamp(0.35, 1.0);
            p.with_hiding(p.hiding() * t)
        };
        taper_stroke(c, &pts, &w, &deadv, &lean, &load, !l.broken, 0.55, rng);
        return;
    }
    // wide wood: bands across it, over the part that is wide; then the
    // limb continues in one stroke from inside the bands
    let thin_at = w.iter().position(|&x| x < 8.0 * 1.3).unwrap_or(w.len() - 1);
    let end = (thin_at + 3).min(pts.len() - 1);
    for j in 0..bands {
        let u = (j as f32 + 0.5) / bands as f32 - 0.5; // -0.5..0.5 across
        let bp: Vec<(f32, f32)> = (0..=end)
            .map(|i| {
                let d = dir_at(&pts, i);
                let nrm = (-d.1, d.0);
                let wob = rng.normal() * 0.04 * w[i] / bands as f32;
                (pts[i].0 + nrm.0 * (u * w[i] * 0.9 + wob), pts[i].1 + nrm.1 * (u * w[i] * 0.9 + wob))
            })
            .collect();
        // lit side: which side of the axis faces the light
        let d = dir_at(&pts, 0);
        let nrm = (-d.1, d.0);
        let face = (nrm.0 * LIGHT.0 + nrm.1 * LIGHT.1) * u * 2.0; // -1 shadow .. 1 light
        let base = if face > 0.25 { mid } else { dark };
        let bw: Vec<f32> = (0..=end).map(|i| (w[i] / bands as f32 * 1.7).max(finest)).collect();
        let pa = |i: usize| paint_for(i, base);
        taper_stroke(c, &bp, &bw, &deadv[..=end], &pa, &|_| 0.9, false, 0.4, rng);
    }
    if thin_at + 1 < pts.len() {
        let back = thin_at.saturating_sub(2);
        let pa = |i: usize| paint_for(i + back, dark);
        taper_stroke(c, &pts[back..], &w[back..], &deadv[back..], &pa, &load, !l.broken, 0.5, rng);
    }
}

/// Back into the wet dark: a lighter stroke down the lit side, a darker
/// one down the shadow side, following the limb's taper.
fn model_limb(c: &mut Canvas, l: &Limb, lit: Paint, dead_lit: Paint, dark: Paint, rng: &mut Rng) {
    let (pts, w, deadv) = resample(l, (l.w[0] * 0.25).clamp(0.6, 4.0));
    let keep = w.iter().position(|&x| x < 2.2).unwrap_or(w.len());
    if keep < 3 {
        return;
    }
    let (pts, w, deadv) = (&pts[..keep], &w[..keep], &deadv[..keep]);
    for (side, paint, amt, u, width) in [(1.0f32, None, 0.45f32, 0.26f32, 0.32f32), (-1.0, Some(dark), 0.5, 0.3, 0.28)] {
        let sp: Vec<(f32, f32)> = (0..pts.len())
            .map(|i| {
                let d = dir_at(pts, i);
                let mut nrm = (-d.1, d.0);
                if (nrm.0 * LIGHT.0 + nrm.1 * LIGHT.1) * side < 0.0 {
                    nrm = (-nrm.0, -nrm.1);
                }
                (pts[i].0 + nrm.0 * w[i] * u, pts[i].1 + nrm.1 * w[i] * u)
            })
            .collect();
        let sw: Vec<f32> = w.iter().map(|x| (x * width).max(0.4)).collect();
        // how squarely the limb turns its side to the light
        let d = dir_at(pts, pts.len() / 2);
        let face = ((-d.1) * LIGHT.0 + d.0 * LIGHT.1).abs();
        let a = amt * (0.4 + 0.6 * face);
        let pa = |i: usize| paint.unwrap_or(if deadv[i] { dead_lit } else { lit });
        taper_stroke(c, &sp, &sw, deadv, &pa, &|_| a, true, 0.5, rng);
    }
}

/// A broken end: blunt, split into a few splinters, pale wood on the face.
fn break_end(c: &mut Canvas, l: &Limb, dead: Paint, wood: Paint, rng: &mut Rng) {
    let n = l.pts.len();
    let end = l.pts[n - 1];
    let d = l.dir(n - 1);
    let nrm = (-d.1, d.0);
    let w = l.w[n - 1];
    if w < 1.0 {
        return;
    }
    let k = 3 + (rng.f() * 3.0) as usize;
    let long = (rng.f() * k as f32) as usize;
    for i in 0..k {
        let off = (i as f32 / (k - 1) as f32 - 0.5) * w * 0.8 + rng.normal() * w * 0.06;
        let len = w * if i == long { rng.range(1.2, 2.6) } else { rng.range(0.2, 0.8) };
        let s = (end.0 - d.0 * w * 0.3 + nrm.0 * off, end.1 - d.1 * w * 0.3 + nrm.1 * off);
        let bend = rng.normal() * 0.2;
        let e = (s.0 + (d.0 + nrm.0 * bend) * len, s.1 + (d.1 + nrm.1 * bend) * len);
        let tw = (w * rng.range(0.2, 0.35)).max(0.5);
        let mut held = Held::new(brush(tw), rng.next_u64());
        held.load(dead, 0.8);
        c.drag(&mut held, &Gesture::new(vec![s, e]).pressure(0.85, 0.1).ramps(0.0, 0.7).shake(0.6), None);
    }
    let mut held = Held::new(Tool { point: 0.6, ..Tool::round_sable((w * 0.35).max(0.6)) }, rng.next_u64());
    held.load(wood, 0.5);
    let a = (end.0 - d.0 * w * 0.35 - nrm.0 * w * 0.3, end.1 - d.1 * w * 0.35 - nrm.1 * w * 0.3);
    let b = (end.0 - d.0 * w * 0.2 + nrm.0 * w * 0.25, end.1 - d.1 * w * 0.2 + nrm.1 * w * 0.25);
    c.drag(&mut held, &Gesture::new(vec![a, b]).pressure(0.6, 0.3).ramps(0.2, 0.5), None);
}

/// Oak bark on the dry dark: long broken fissure strokes along the wood
/// and lean light on the ridges, stronger on the lit side.
fn bark(c: &mut Canvas, l: &Limb, dark: Paint, lit: Paint, dead_lit: Paint, mask: &Mask, rng: &mut Rng) {
    let (pts, w, deadv) = resample(l, 2.0);
    let arc = cumulative(&pts);
    let total = *arc.last().unwrap();
    let wide = w.iter().position(|&x| x < 6.0).unwrap_or(w.len());
    if wide < 4 {
        return;
    }
    let count = ((total.min(arc[wide - 1]) * l.w[0]) / 90.0) as usize + 3;
    for _ in 0..count {
        let dark_mark = rng.f() < 0.72;
        // across the wood: ridges crowd to the lit side
        let u = if dark_mark { rng.range(-0.45, 0.45) } else { rng.range(-0.42, 0.25) };
        let start = rng.range(0.0, arc[wide - 1]);
        let len = rng.range(4.0, 18.0) * (0.5 + l.w[0] / 60.0);
        let i0 = arc.iter().position(|&a| a >= start).unwrap_or(0);
        let i1 = arc.iter().position(|&a| a >= start + len).unwrap_or(wide - 1).min(wide - 1);
        if i1 <= i0 + 1 {
            continue;
        }
        let side = {
            let d = dir_at(&pts, i0);
            let n = (-d.1, d.0);
            if n.0 * LIGHT.0 + n.1 * LIGHT.1 > 0.0 { 1.0 } else { -1.0 }
        };
        let sp: Vec<(f32, f32)> = (i0..=i1)
            .map(|i| {
                let d = dir_at(&pts, i);
                let n = (-d.1 * side, d.0 * side);
                let wig = rng.normal() * 0.25;
                (pts[i].0 + n.0 * (u * w[i] + wig), pts[i].1 + n.1 * (u * w[i] + wig))
            })
            .collect();
        let bw = if dark_mark { rng.range(0.5, 1.1) } else { rng.range(0.5, 1.2) } * (0.6 + l.w[0] / 50.0);
        let paint = if dark_mark { dark } else if deadv[i0] { dead_lit } else { lit.with_hiding(lit.hiding() * 0.6) };
        let lightness = if dark_mark { 0.6 } else { (0.05 + 0.2 * (u + 0.45)).clamp(0.04, 0.16) };
        let mut held = Held::new(Tool { point: 0.7, ragged: 0.5, ..Tool::round_sable(bw) }, rng.next_u64());
        held.load(paint, lightness);
        let g = Gesture::new(sp).pressure(rng.range(0.4, 0.8), rng.range(0.2, 0.5)).ramps(0.15, 0.4).shake(0.7);
        c.drag(&mut held, &g, Some(mask));
    }
}

/// Snow lying along the top of a limb: where it is near level and thick
/// enough to stop the flakes bouncing off, in heaps (oak bark is rough
/// [MIL64 p.6]), lit on top, a cooler grey on its lower edge.
fn snow_on_limb(c: &mut Canvas, l: &Limb, lit: Paint, cool: Paint, lumps: &Fbm, rng: &mut Rng) {
    let (pts, w, _) = resample(l, 1.0);
    let n = pts.len();
    if n < 3 {
        return;
    }
    // amount of snow per point
    let amt: Vec<f32> = (0..n)
        .map(|i| {
            let d = dir_at(&pts, i);
            let level = 1.0 - smoothstep(0.45, 0.8, d.1.abs());
            let thick = smoothstep(0.9, 3.0, w[i]);
            let lump = 0.5 + lumps.get(pts[i].0, pts[i].1);
            // the noise only thickens and thins the ridge; it drops out
            // where it is thinnest (bounced off, blown off)
            level * thick * (0.25 + 1.0 * smoothstep(0.15, 0.75, lump))
        })
        .collect();
    let mut i = 0;
    while i < n {
        if amt[i] < 0.15 {
            i += 1;
            continue;
        }
        let a = i;
        while i < n && amt[i] >= 0.15 {
            i += 1;
        }
        let b = i - 1;
        if b < a + 2 {
            continue;
        }
        // the upper edge of the wood: the normal that points up
        let up = |k: usize| {
            let d = dir_at(&pts, k);
            let nn = (-d.1, d.0);
            if nn.1 < 0.0 { nn } else { (-nn.0, -nn.1) }
        };
        let depth: Vec<f32> = (a..=b).map(|k| amt[k] * (w[k] * 0.45).min(4.5).max(0.6)).collect();
        let top: Vec<(f32, f32)> = (a..=b)
            .map(|k| {
                let u = up(k);
                let dd = depth[k - a];
                (pts[k].0 + u.0 * (w[k] * 0.5 + dd * 0.15), pts[k].1 + u.1 * (w[k] * 0.5 + dd * 0.15))
            })
            .collect();
        // a cool underside first, a touch lower, then the light on top
        let under: Vec<(f32, f32)> = (a..=b)
            .map(|k| {
                let u = up(k);
                let dd = depth[k - a];
                (pts[k].0 + u.0 * (w[k] * 0.5 - dd * 0.15), pts[k].1 + u.1 * (w[k] * 0.5 - dd * 0.15))
            })
            .collect();
        let widest = depth.iter().cloned().fold(0.0, f32::max);
        // the cool underside of the snow: one thin drag along the wood
        {
            let tool = Tool { point: 0.8, ragged: 0.35, ..Tool::round_sable((widest * 0.9).max(0.7)) };
            let sw: Vec<f32> = depth.iter().map(|&dd| tool.pressure_for((dd * 0.7).max(0.3)).clamp(0.05, 1.0)).collect();
            let mut held = Held::new(tool, rng.next_u64());
            held.load(cool, 0.5);
            let g = Gesture::new(under.clone()).pressure(1.0, 1.0).swell(sw).ramps(0.2, 0.3).shake(0.5);
            c.drag(&mut held, &g, None);
        }
        // the heaps: touches along the top, overlapping, each pushed a
        // little along the limb, bigger where the snow is deeper
        let mut held = Held::new(Tool { point: 0.5, ragged: 0.4, ..Tool::round_sable((widest * 1.2).max(0.8)) }, rng.next_u64());
        held.load(lit, 0.9);
        let mut k = 0usize;
        let mut dabs = 0;
        while k < top.len() {
            let dd = depth[k];
            let tool_w = (widest * 1.2).max(0.8);
            let want = (dd * rng.range(0.8, 1.25)).max(0.5);
            let p = Tool { point: 0.5, ..Tool::round_sable(tool_w) }.pressure_for(want).clamp(0.1, 1.0);
            let d = dir_at(&top, k);
            let push = rng.range(0.5, 1.5) * dd;
            c.touch(&mut held, &Touch::at(top[k].0, top[k].1 - dd * 0.1).pressure(p).drag(d.0 * push, d.1 * push), None);
            dabs += 1;
            if dabs % 6 == 0 {
                held.reload(lit, 0.9);
            }
            k += ((dd * rng.range(0.3, 0.6)).max(1.0)) as usize;
        }
    }
}

/// A little heap of snow in the fork where a branch leaves its parent.
fn crotch(c: &mut Canvas, par: &Limb, child: &Limb, lit: Paint, rng: &mut Rng) {
    let p = child.pts[0];
    let dc = child.dir(0);
    let i = child.at.min(par.pts.len() - 2);
    let dp = par.dir(i);
    // the fork opens upward between the two directions
    let bis = (dc.0 + dp.0, dc.1 + dp.1);
    if bis.1 > -0.3 {
        return;
    }
    let bl = (bis.0 * bis.0 + bis.1 * bis.1).sqrt().max(1e-6);
    let b = (bis.0 / bl, bis.1 / bl);
    let r = (child.w[0] * 0.6).clamp(0.8, 4.0);
    let at = (p.0 + b.0 * r * 0.8, p.1 + b.1 * r * 0.8);
    let mut held = Held::new(Tool { point: 0.5, ..Tool::round_sable(r * 1.6) }, rng.next_u64());
    held.load(lit, 0.8);
    c.touch(&mut held, &Touch::at(at.0, at.1).pressure(rng.range(0.5, 0.8)), None);
}

/// The piece that broke from the dead limb, lying half buried in the snow
/// before the tree: a crooked grey branch with stubs, snow along its top
/// and drifted over it in places.
fn fallen(c: &mut Canvas, pal: &paint::Palette, rng: &mut Rng) {
    let wood = pal.mix(hex("#5f5a53")).paint(0.2);
    let dark = pal.mix(hex("#34302b")).paint(0.2);
    let snow = pal.mix(hex("#efebe1")).paint(0.14);
    let cool = pal.mix(hex("#b4bac3")).paint(0.14);
    let pts = vec![(530.0, 1172.0), (575.0, 1179.0), (612.0, 1175.0), (640.0, 1186.0), (683.0, 1190.0), (712.0, 1186.0)];
    // the shadow under it on the snow first
    let mut sh = Held::new(Tool { ragged: 0.4, ..Tool::round_sable(5.0) }, rng.next_u64());
    sh.load(cool, 0.6);
    let low: Vec<(f32, f32)> = pts.iter().map(|p| (p.0 + 5.0, p.1 + 5.0)).collect();
    c.drag(&mut sh, &Gesture::new(low).pressure(0.8, 0.5).ramps(0.2, 0.4).shake(0.6), None);
    // the branch, thick end left, and its dark underside
    let mut b = Held::new(Tool { point: 0.7, ragged: 0.15, ..Tool::round_sable(10.0) }, rng.next_u64());
    b.load(wood, 0.9);
    c.drag(&mut b, &Gesture::new(pts.clone()).pressure(0.9, 0.35).ramps(0.06, 0.2).shake(0.6), None);
    let mut u = Held::new(Tool { point: 0.7, ..Tool::round_sable(2.5) }, rng.next_u64());
    u.load(dark, 0.7);
    let under: Vec<(f32, f32)> = pts.iter().map(|p| (p.0, p.1 + 2.6)).collect();
    c.drag(&mut u, &Gesture::new(under).pressure(0.8, 0.3).ramps(0.1, 0.3).shake(0.6), None);
    // stubs of its side branches, broken short
    for &(x, y, dx, dy) in &[(600.0f32, 1178.0f32, 6.0f32, -12.0f32), (640.0, 1182.0, -4.0, -9.0), (668.0, 1185.0, 9.0, -6.0)] {
        let mut t = Held::new(Tool { point: 0.8, ..Tool::round_sable(2.2) }, rng.next_u64());
        t.load(wood, 0.8);
        c.drag(&mut t, &Gesture::new(vec![(x, y), (x + dx * 0.6, y + dy * 0.5), (x + dx, y + dy)]).pressure(0.8, 0.2).ramps(0.0, 0.5).shake(0.6), None);
    }
    // snow along its top, and drifted over it at two places
    let top: Vec<(f32, f32)> = pts.iter().map(|p| (p.0, p.1 - 3.2)).collect();
    let mut s = Held::new(Tool { point: 0.5, ragged: 0.4, ..Tool::round_sable(2.6) }, rng.next_u64());
    s.load(snow, 0.8);
    c.drag(&mut s, &Gesture::new(top[..3].to_vec()).pressure(0.7, 0.5).ramps(0.2, 0.3).swell(vec![1.0, 0.6, 1.1, 0.8]).shake(0.6), None);
    for &(x, y, w) in &[(608.0f32, 1179.0f32, 12.0f32), (672.0, 1186.0, 9.0)] {
        let mut d = Held::new(Tool { ragged: 0.4, ..Tool::filbert(6.0) }, rng.next_u64());
        d.load(snow, 0.8);
        c.drag(&mut d, &Gesture::new(vec![(x - w * 0.5, y + 2.0), (x, y - 1.0), (x + w * 0.5, y + 2.5)]).pressure(0.7, 0.5).ramps(0.3, 0.4).shake(0.7), None);
    }
}

/// A crow sitting on a branch at `at` (its feet), facing left (-1) or
/// right (1): a body pressed in one short stroke, the head a touch, the
/// beak a flick, the tail a stroke drawn down and back.
fn crow_perched(c: &mut Canvas, at: (f32, f32), face: f32, size: f32, black: Paint, rng: &mut Rng) {
    let s = size;
    let body_c = (at.0, at.1 - s * 0.3);
    let mut b = Held::new(Tool { point: 0.6, ..Tool::round_sable(s * 0.42) }, rng.next_u64());
    b.load(black, 0.9);
    // body: from the tail root up to the shoulders, leaning forward
    let tail_root = (body_c.0 - face * s * 0.28, body_c.1 + s * 0.08);
    let shoulder = (body_c.0 + face * s * 0.22, body_c.1 - s * 0.12);
    c.drag(&mut b, &Gesture::new(vec![tail_root, body_c, shoulder]).pressure(0.75, 0.9).ramps(0.2, 0.2).shake(0.4), None);
    // head
    let head = (shoulder.0 + face * s * 0.08, shoulder.1 - s * 0.07);
    let mut h = Held::new(Tool { point: 0.5, ..Tool::round_sable(s * 0.3) }, rng.next_u64());
    h.load(black, 0.9);
    // the neck and head pulled up out of the wet body in one short stroke
    c.drag(&mut h, &Gesture::new(vec![shoulder, head]).pressure(0.9, 0.8).ramps(0.0, 0.3).shake(0.2), None);
    // beak
    let mut k = Held::new(Tool { point: 1.0, ..Tool::round_sable(s * 0.1) }, rng.next_u64());
    k.load(black, 0.8);
    c.drag(&mut k, &Gesture::new(vec![head, (head.0 + face * s * 0.22, head.1 + s * 0.03)]).pressure(0.8, 0.0).ramps(0.0, 0.8).shake(0.2), None);
    // tail: down and back
    let mut t = Held::new(Tool { point: 0.8, ..Tool::round_sable(s * 0.18) }, rng.next_u64());
    t.load(black, 0.8);
    c.drag(&mut t, &Gesture::new(vec![tail_root, (tail_root.0 - face * s * 0.3, tail_root.1 + s * 0.3)]).pressure(0.8, 0.4).ramps(0.0, 0.3).shake(0.3), None);
    // legs to the branch
    let mut l = Held::new(Tool { point: 1.0, ..Tool::rigger(s * 0.05) }, rng.next_u64());
    l.load(black, 0.7);
    for dx in [-0.05f32, 0.07] {
        c.drag(&mut l, &Gesture::line((body_c.0 + face * dx * s, body_c.1 + s * 0.12), (at.0 + face * dx * s * 1.2, at.1 + 0.3)).pressure(0.6, 0.5).shake(0.2), None);
    }
}

/// A crow flying far off: two wing strokes pressed at the body and lifted
/// toward the tips, a touch for the body.
fn crow_flying(c: &mut Canvas, at: (f32, f32), size: f32, black: Paint, rng: &mut Rng) {
    let mut b = Held::new(Tool { point: 1.0, ..Tool::round_sable(size * 0.2) }, rng.next_u64());
    let lift = rng.range(-0.3, 0.3);
    for side in [-1.0f32, 1.0] {
        b.reload(black, 0.8);
        let tip = (at.0 + side * size, at.1 - size * (0.25 + lift * side * 0.5));
        let mid = (at.0 + side * size * 0.45, at.1 - size * 0.32);
        c.drag(&mut b, &Gesture::new(vec![at, mid, tip]).pressure(0.8, 0.0).ramps(0.05, 0.75).shake(0.4), None);
    }
    c.touch(&mut b, &Touch::at(at.0, at.1 + 0.4).pressure(0.6), None);
}

/// Dry grass stalks through the snow: fine strokes flicked upward.
fn grass(c: &mut Canvas, pal: &paint::Palette, rng: &mut Rng) {
    let colors = [hex("#7a6844"), hex("#8d7a52"), hex("#5b4c35"), hex("#a08d63")];
    let paints: Vec<Paint> = colors.iter().map(|&h| pal.mix(h).paint(0.2)).collect();
    let tufts: Vec<(f32, f32, f32)> = {
        let mut v = vec![];
        // around the foot
        for _ in 0..14 {
            v.push((BASE.0 + rng.range(-70.0, 90.0), BASE.1 + rng.range(-2.0, 16.0), 1.0));
        }
        // in a few patches where the wind has blown the snow thin, larger
        // toward us
        for _ in 0..5 {
            let cy = rng.range(HORIZON + 30.0, 1260.0);
            let cx = rng.range(40.0, 960.0);
            let s0 = 0.4 + 1.1 * (cy - HORIZON) / 280.0;
            for _ in 0..(3 + (rng.f() * 6.0) as usize) {
                let y = cy + rng.normal() * 4.0 * s0;
                v.push((cx + rng.normal() * 25.0 * s0, y, s0 * rng.range(0.7, 1.1)));
            }
        }
        v
    };
    for (x, y, s) in tufts {
        let blades = 4 + (rng.f() * 8.0 * s) as usize;
        for _ in 0..blades {
            let paint = paints[(rng.f() * paints.len() as f32) as usize % paints.len()];
            let ht = rng.range(5.0, 16.0) * s;
            let lean = rng.range(-0.5, 0.5);
            let x0 = x + rng.normal() * 2.5 * s;
            let pts = vec![(x0, y), (x0 + lean * ht * 0.3, y - ht * 0.55), (x0 + lean * ht, y - ht)];
            let mut held = Held::new(Tool { point: 1.0, ..Tool::rigger((0.45 * s).max(0.3)) }, rng.next_u64());
            held.load(paint, 0.7);
            c.drag(&mut held, &Gesture::new(pts).pressure(0.75, 0.0).ramps(0.05, 0.85).shake(0.5), None);
        }
    }
}

/// Twig sprays from a live limb: from its tip and from along its last
/// stretch, short crooked twigs in clusters, each lifted off to a point,
/// a second, finer order off the longer ones.
fn twigs(c: &mut Canvas, l: &Limb, paint: Paint, rng: &mut Rng) {
    let n = l.pts.len();
    let wend = l.w[n - 1];
    if wend > 3.0 {
        return;
    }
    let arc = cumulative(&l.pts);
    let total = arc[n - 1];
    let spray = |c: &mut Canvas, from: (f32, f32), dir: (f32, f32), len: f32, w: f32, depth: u32, rng: &mut Rng| {
        let mut stack = vec![(from, dir, len, w, depth)];
        while let Some((p, d, len, w, depth)) = stack.pop() {
            // a crooked twig: two or three kinks
            let mut pts = vec![p];
            let mut q = p;
            let mut dd = d;
            let kinks = 3;
            for _ in 0..kinks {
                let a = rng.normal() * 0.28;
                let (ca, sa) = (a.cos(), a.sin());
                dd = (dd.0 * ca - dd.1 * sa, dd.0 * sa + dd.1 * ca);
                // twigs turn up a little toward the light
                dd = (dd.0, dd.1 - 0.08);
                let dl = (dd.0 * dd.0 + dd.1 * dd.1).sqrt();
                dd = (dd.0 / dl, dd.1 / dl);
                q = (q.0 + dd.0 * len / kinks as f32, q.1 + dd.1 * len / kinks as f32);
                pts.push(q);
            }
            let tool = brush(w);
            let p0 = tool.pressure_for(w).clamp(0.05, 1.0);
            let mut held = Held::new(tool, rng.next_u64());
            held.load(paint, 0.55);
            c.drag(&mut held, &Gesture::new(pts.clone()).pressure(p0, p0 * 0.25).ramps(0.0, 0.6).shake(0.5), None);
            if depth > 0 && len > 5.0 {
                let k = 1 + (rng.f() * 2.2) as usize;
                for _ in 0..k {
                    let at = pts[1 + (rng.f() * (pts.len() - 2) as f32) as usize];
                    let side = if rng.f() < 0.5 { -1.0 } else { 1.0 };
                    let a = side * rng.range(0.4, 0.9);
                    let (ca, sa) = (a.cos(), a.sin());
                    stack.push((at, (dd.0 * ca - dd.1 * sa, dd.0 * sa + dd.1 * ca), len * rng.range(0.4, 0.65), (w * 0.7).max(0.3), depth - 1));
                }
            }
        }
    };
    // a cluster at the tip
    let tip = l.pts[n - 1];
    let d = l.dir(n - 1);
    let k = 1 + (rng.f() * 3.0) as usize;
    for _ in 0..k {
        let a = rng.normal() * 0.35;
        let (ca, sa) = (a.cos(), a.sin());
        let dd = (d.0 * ca - d.1 * sa, d.0 * sa + d.1 * ca);
        spray(c, tip, dd, rng.range(8.0, 20.0), (wend * 0.8).clamp(0.3, 0.8), 2, rng);
    }
    // and from along the thin stretch of the limb, both sides: the net
    let thin_from = l.w.iter().position(|&x| x < 5.0).unwrap_or(n - 1);
    let span = total - arc[thin_from];
    let m = (span / 6.0) as usize;
    for _ in 0..m.min(34) {
        let t = total - rng.range(2.0, span.max(2.5));
        let i = arc.iter().position(|&a| a >= t).unwrap_or(n - 1).clamp(1, n - 1);
        let p = l.pts[i];
        let di = l.dir(i);
        let side = if rng.f() < 0.5 { -1.0 } else { 1.0 };
        let a = side * rng.range(0.5, 1.0);
        let (ca, sa) = (a.cos(), a.sin());
        let wi = l.w[i];
        spray(c, p, (di.0 * ca - di.1 * sa, di.0 * sa + di.1 * ca), rng.range(7.0, 16.0), (wi * 0.5).clamp(0.3, 0.7), 2, rng);
    }
}

/// Last year's leaves, a few on the low live twigs.
fn leaves(c: &mut Canvas, pal: &paint::Palette, oak: &Skeleton, rng: &mut Rng) {
    let browns = [pal.mix(hex("#7b5537")).paint(0.2), pal.mix(hex("#936a42")).paint(0.2), pal.mix(hex("#5a3f2b")).paint(0.2)];
    let (_, y0, _, y1) = oak.bounds();
    for l in oak.limbs.iter().filter(|l| l.order >= 3 && l.dead_from == l.pts.len() && !l.is_empty()) {
        let tip = *l.pts.last().unwrap();
        // low in the crown, and not everywhere
        let low = (tip.1 - y0) / (y1 - y0).max(1.0);
        if low < 0.55 || rng.f() > 0.35 {
            continue;
        }
        let k = 2 + (rng.f() * 4.0) as usize;
        for _ in 0..k {
            let at = (tip.0 + rng.normal() * 2.2, tip.1 + rng.normal() * 1.8 + 1.0);
            let paint = browns[(rng.f() * 3.0) as usize % 3];
            let mut held = Held::new(Tool { point: 0.7, ..Tool::round_sable(rng.range(1.4, 2.4)) }, rng.next_u64());
            held.load(paint, 0.7);
            let a = rng.range(0.0, std::f32::consts::TAU);
            let len = rng.range(1.2, 2.8);
            c.drag(&mut held, &Gesture::new(vec![at, (at.0 + a.cos() * len, at.1 + a.sin() * len + 0.6)]).pressure(0.7, 0.2).ramps(0.1, 0.5).shake(0.6), None);
        }
    }
}
