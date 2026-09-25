//! r12_tree2: one old oak in winter, in the manner of C. D. Friedrich.
//!
//! An old pedunculate oak standing alone on a snowfield under a low winter
//! sky: stag-headed (its top dead and grey, a limb broken in a storm and
//! lying below), the living lower crown ending in a net of fine twigs, a few
//! of last year's brown leaves still hanging on the low twigs, snow lying in
//! ridges along the upper sides of the level limbs and in the forks.
//!
//!   cargo paint r12_tree2 -- --full --width 2400
//!   cargo paint r12_tree2 -- --full --width 2400 --crop x0,y0,x1,y1
//!   cargo paint r12_tree2 -- --full --width 2400 --ckpt / --resume <stage>
//!
//! Order of work (Friedrich's, as far as it is known): ground bought
//! primed; sky laid in thin, fused, then stippled; the land; the tree
//! painted on the finished sky [ALF p.346]; details last (grass over the
//! finished snow [NG p.56]).

use paint::color::{Mix, mix};
use paint::{Canvas, Handling, Fbm, Gesture, Habit, Held, Limb, Mask, Orient, Paint, Palette, Rng, Skeleton, Stipple, Style, Tool, Touch, hex, smoothstep};

const ASPECT: f32 = 0.8; // portrait: 1000 x 1250 units
const HORIZON: f32 = 1012.0;
const BASE: (f32, f32) = (492.0, 1084.0);
const TREE_H: f32 = 880.0;
const TREE_SEED: u64 = 12;
/// Wood wider than this (units) is laid in as a form, not dragged.
const THICK: f32 = 6.0;

fn habit() -> Habit {
    Habit { years: 30, lean: -0.03, decline: 0.18, decay: 0.4, breakage: 0.18, twig: 0.0010, apical: 0.56, ..Habit::oak() }
}

/// Thumbnails of candidate trees (the painter's pencil studies before
/// choosing one), written as a PGM contact sheet: `--probe <first seed>`.
fn probe(first: u64) {
    let (tw, th) = (250usize, 312usize);
    let (cols, rows) = (6usize, 3usize);
    let mut img = vec![255u8; tw * cols * th * rows];
    for k in 0..cols * rows {
        let seed = first + k as u64;
        let sk = habit().grow((500.0, 1150.0), 1000.0, seed);
        let fr = paint::Frame::new(tw, th, tw as f32 / 1000.0);
        let m = sk.mask(fr);
        let dead = sk.limbs.iter().filter(|l| l.dead).count();
        eprintln!("seed {seed}: {} limbs, {dead} dead, bounds {:?}", sk.limbs.len(), sk.bounds());
        let (cx, cy) = (k % cols, k / cols);
        for y in 0..th {
            for x in 0..tw {
                let v = m.data[y * tw + x];
                img[(cy * th + y) * tw * cols + cx * tw + x] = (255.0 * (1.0 - v.clamp(0.0, 1.0))) as u8;
            }
        }
    }
    let mut out = format!("P5 {} {} 255\n", tw * cols, th * rows).into_bytes();
    out.extend(img);
    std::fs::write("out/r12_tree2_probe.pgm", out).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--probe") {
        probe(args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1));
        return;
    }
    let o = paintings::run::Run::new("r12_tree2");
    let st = Style::friedrich();
    let pal = &st.palette;
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = c.height();

    // ---- the tree, grown before anything is painted (it is the drawing)

    // ---- the lie of the land: a far flat line at the horizon, the snow
    // rising gently toward the tree and toward us
    let far = Fbm::new(7, 4, 90.0);
    let land_top = move |x: f32| HORIZON + 2.0 * far.get(x, 0.0) + 5.0 * smoothstep(200.0, 0.0, x) * 0.0;
    let sky_m = Mask::from_fn(f, move |x, y| 1.0 - smoothstep(land_top(x) - 1.0, land_top(x) + 3.0, y));
    let land_m = Mask::from_fn(f, move |x, y| smoothstep(land_top(x) - 2.0, land_top(x) + 1.0, y));

    // The sky: leaden grey-blue overhead, a paler grey lower, and near the
    // horizon a thin cold yellow where the overcast opens (late afternoon).
    let sky = |_x: f32, y: f32| {
        let t = (y / HORIZON).clamp(0.0, 1.0);
        paint::gradient(
            &[(0.0, hex("#7d858f")), (0.35, hex("#98a0a6")), (0.7, hex("#b9bab4")), (0.9, hex("#d6cfb6")), (1.0, hex("#e2d6b4"))],
            t,
            Mix::Light,
        )
    };
    if o.stage("sky", &mut c, &mut rng) {
        // lay-in: thin, long horizontal strokes, a shade duller than the end
        let lay = st
            .broad()
            .color(move |x, y| mix(sky(x, y), hex("#6c7078"), 0.06, Mix::Light))
            .angle(|_, y| 0.03 * (y / 300.0).sin())
            .coverage(4.0)
            .medium(0.3)
            .dips(1, 0.6, 0.6);
        c.work(&sky_m, &lay, 11);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.angle(|_, _| 0.0), 12);
        }
        // first stipple into the wet lay-in: breaks the strokes
        let s1 = Stipple::new(Tool::stippler(3.0)).mixed(pal, 0.45).color(sky).coverage(|_, _| 2.0).pressure(0.5, 0.85).dips(20, 0.4, 0.5);
        c.stipple(&sky_m, &s1, 13);
        c.dry();
    }

    if o.stage("sky stipple", &mut c, &mut rng) {
        // dry: finer and lighter, denser toward the light at the horizon;
        // a faint band of cloud edge lighter at a third of the way down
        let bands = Fbm::new(41, 4, 260.0);
        let glow = move |x: f32, y: f32| {
            let b = 0.5 + 0.5 * bands.get(x * 0.35, y * 2.2);
            mix(sky(x, y), hex("#ece2c4"), 0.04 + 0.2 * smoothstep(500.0, HORIZON, y) + 0.06 * b, Mix::Light)
        };
        let s2 = Stipple::new(Tool::stippler(1.6))
            .mixed(pal, 0.5)
            .color(glow)
            .coverage(move |x, y| 0.6 + 1.6 * smoothstep(250.0, HORIZON, y) + 0.5 * bands.get(x * 0.35, y * 2.2).max(0.0))
            .pressure(0.45, 0.8)
            .dips(24, 0.35, 0.6);
        c.stipple(&sky_m, &s2, 14);
        c.dry();
    }

    // far land: a low line of woods and fields, far off, blue-grey
    let woods = Fbm::new(9, 5, 30.0);
    let wood_top = move |x: f32| HORIZON - 5.0 - 5.0 * (woods.get(x, 0.0) * 1.6).max(-0.3) * smoothstep(0.0, 60.0, x.min(1000.0 - x)) * (0.4 + 0.6 * smoothstep(600.0, 900.0, x));
    let woods_m = Mask::from_fn(f, move |x, y| smoothstep(wood_top(x) - 0.8, wood_top(x) + 0.8, y) * (1.0 - smoothstep(HORIZON + 2.0, HORIZON + 5.0, y)));
    if o.stage("far", &mut c, &mut rng) {
        let far_p = st.detail().color(|_, _| hex("#7c7f86")).angle(|_, _| 0.0).length(3.0, 8.0).coverage(2.5).medium(0.35);
        c.work(&woods_m, &far_p, 21);
        c.dry();
    }

    // the snowfield
    let drift = Fbm::new(17, 4, 180.0);
    let snow_col = move |x: f32, y: f32| {
        let t = ((y - HORIZON) / (h - HORIZON)).clamp(0.0, 1.0);
        // far snow takes the sky's grey; near snow is whiter, with cool
        // shallow hollows between drifts
        let base = paint::gradient(&[(0.0, hex("#c6c4bb")), (0.25, hex("#dcd9cf")), (1.0, hex("#e6e3da"))], t, Mix::Light);
        let hollow = (drift.get(x * 0.5, y * 3.0) * 1.4).clamp(-1.0, 1.0);
        let base = if hollow < 0.0 { mix(base, hex("#a9adb4"), -hollow * 0.35 * (0.3 + t), Mix::Light) } else { mix(base, hex("#efece3"), hollow * 0.25 * t, Mix::Light) };
        // the tree's soft shadow under overcast light, and snow heaped round the foot
        let dx = (x - BASE.0) / 120.0;
        let dy = (y - BASE.1 - 6.0) / 18.0;
        let sh = (-(dx * dx + dy * dy)).exp();
        mix(base, hex("#9aa0a8"), 0.35 * sh, Mix::Light)
    };
    if o.stage("snow", &mut c, &mut rng) {
        let lay = st.body().color(snow_col).angle(|x, y| 0.04 * ((x + y) / 150.0).sin()).length(30.0, 90.0).coverage(3.0).medium(0.15);
        c.work(&land_m, &lay, 31);
        // fused a little, horizontally, so the drifts read as soft
        if let Some(b) = st.blend() {
            c.work(&land_m, &b.angle(|_, _| 0.0).pressure(0.3, 0.4), 32);
        }
        c.dry();
    }

    // ---- the tree (grown here, after the land: its drawing)
    let oak = habit().grow(BASE, TREE_H, TREE_SEED);
    let segs = Segs::of(&oak, THICK);
    let thick = Mask::from_shape(f, {
        let mut sh = paint::Shape::new();
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && !l.root && l.w[0] > THICK) {
            let n = l.pts.iter().zip(&l.w).take_while(|(_, w)| **w > THICK * 0.8).count().max(2).min(l.pts.len());
            sh = sh.ribbon(&l.pts[..n], &l.w[..n]);
        }
        sh
    })
    .mul_fn(|_, y| 1.0 - smoothstep(BASE.1 + 4.0, BASE.1 + 8.0, y));
    let bark_dark = pal.mix(hex("#2b2621")).paint(0.22);
    let bark_dead = pal.mix(hex("#57534c")).paint(0.25);
    if o.stage("tree", &mut c, &mut rng) {
        // the bole and the big limbs laid in as forms: body strokes along
        // the wood, a little lighter on the side toward the light (upper
        // left, the overcast's brightest quarter), dead wood grey
        let sg = &segs;
        let wood = Handling::new(Tool { lay: 0.8, ..Tool::filbert(6.0) })
            .mixed(pal, 0.18)
            .pressure(0.6, 0.9)
            .dips(2, 0.6, 0.6)
            .curve(0.06, 0.3)
            .tail(0.15)
            .broken(0.1)
            .swell(0.2)
            .color(move |x, y| {
                let q = sg.near(x, y);
                let base = if q.dead { hex("#5a564f") } else { hex("#2d2823") };
                let lit = (-q.u).clamp(0.0, 1.0) * if q.dead { 0.35 } else { 0.22 };
                mix(base, hex("#8a8680"), lit, Mix::Light)
            })
            .angle(move |x, y| sg.near(x, y).angle)
            .angle_jitter(0.08)
            .length(12.0, 40.0)
            .coverage(3.5)
            .medium(0.18);
        c.work(&thick, &wood, 41);
        let mut r = Rng::new(o.seed + 501);
        // then every limb from where it leaves the laid-in wood to its tip
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && !l.root) {
            limb(&mut c, l, bark_dark, bark_dead, &mut r);
        }
    }

    if o.stage("twigs", &mut c, &mut rng) {
        // the net of fine twigs each modeled tip stands for: short crooked
        // shoots (sympodial elbows), a few side twiglets, lean paint
        let mut r = Rng::new(o.seed + 601);
        let twig = pal.mix(hex("#3a332c")).paint(0.3);
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && !l.root && !l.dead && !l.broken) {
            let n = l.pts.len();
            let tipw = l.w[n - 1];
            if tipw > 2.0 {
                continue;
            }
            let d = l.dir(n - 1);
            let a0 = d.1.atan2(d.0);
            let k = 2 + (r.f() * 3.0) as usize;
            for _ in 0..k {
                let a = a0 + r.range(-0.9, 0.9);
                shoot(&mut c, l.pts[n - 1], a, r.range(7.0, 20.0), 0.5, twig, 2, &mut r);
            }
            // side twigs along the outer part of thin limbs
            let arc = cumulative(&l.pts);
            let tot = arc[n - 1];
            let mut s0 = tot * 0.35 + r.range(0.0, 8.0);
            let mut side = if r.f() < 0.5 { 1.0 } else { -1.0 };
            while s0 < tot - 3.0 {
                let i = arc.iter().position(|&v| v >= s0).unwrap_or(n - 1).min(n - 1);
                if l.w[i] < 2.2 {
                    let dd = l.dir(i);
                    let a = dd.1.atan2(dd.0) + side * r.range(0.5, 1.1);
                    shoot(&mut c, l.pts[i], a, r.range(5.0, 14.0), 0.45, twig, 1, &mut r);
                }
                side = -side;
                s0 += r.range(5.0, 11.0);
            }
        }
        c.dry();
    }

    if o.stage("bark", &mut c, &mut rng) {
        // oak bark: grey ridges broken into long blocks between dark
        // fissures, drawn along the wood with a small lean sable
        let mut r = Rng::new(o.seed + 701);
        let ridge = pal.mix(hex("#6e6a62")).paint(0.3);
        let fissure = pal.mix(hex("#1d1916")).paint(0.2);
        let moss = pal.mix(hex("#4d5539")).paint(0.35);
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && !l.root && l.w[0] > 5.0) {
            let n = l.pts.len();
            let arc = cumulative(&l.pts);
            let tot = arc[n - 1];
            let across = (l.w[0] / 2.2) as usize + 2;
            for j in 0..across * 3 {
                let u = r.range(-0.85, 0.85);
                let s_a = r.range(0.0, tot);
                let len = r.range(6.0, 22.0) * (l.w[0] / 20.0).clamp(0.5, 1.5);
                let pts: Vec<(f32, f32)> = (0..5)
                    .filter_map(|t| {
                        let sv = s_a + len * t as f32 / 4.0;
                        let i = arc.iter().position(|&v| v >= sv)?;
                        if l.w[i] < 4.0 {
                            return None;
                        }
                        let d = l.dir(i);
                        let nn = (-d.1, d.0);
                        let wob = r.normal() * 0.04;
                        Some((l.pts[i].0 + nn.0 * l.w[i] * 0.5 * (u + wob), l.pts[i].1 + nn.1 * l.w[i] * 0.5 * (u + wob)))
                    })
                    .collect();
                if pts.len() < 2 {
                    continue;
                }
                // light ridges toward the lit (left) side, fissures everywhere
                let lit_side = pts[0].0 < segs.near(pts[0].0, pts[0].1).cx;
                let (p, tw, load) = if j % 3 == 0 {
                    (fissure, 0.9, 0.7)
                } else if lit_side || r.f() < 0.25 {
                    (ridge, 0.8, 0.25)
                } else if !l.dead && r.f() < 0.3 {
                    (moss, 1.0, 0.25)
                } else {
                    (fissure, 0.7, 0.5)
                };
                let mut held = Held::new(Tool { point: 0.8, ragged: 0.5, ..Tool::round_sable(tw * 1.6) }, r.next_u64());
                held.load(p, load);
                c.drag(&mut held, &Gesture::new(pts).pressure(r.range(0.4, 0.7), 0.15).ramps(0.1, 0.4).shake(0.6), Some(&thick));
            }
        }
        c.dry();
    }

    if o.stage("snow on the tree", &mut c, &mut rng) {
        // snow lies as a ridge on the upper side of level limbs thick enough
        // to hold it, in the forks, and in heaps in the rough bark on the
        // windward (left) side of the bole; steep and thin wood holds none
        let mut r = Rng::new(o.seed + 801);
        let white = pal.mix(hex("#f0eee6")).paint(0.08);
        let blue = pal.mix(hex("#c9ccd0")).paint(0.1);
        let lie = Fbm::new(23, 3, 25.0);
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && !l.root && l.w[0] > 1.3) {
            let n = l.pts.len();
            let mut run: Vec<(f32, f32)> = Vec::new();
            let mut run_w = 0.0f32;
            for i in 0..n {
                let d = l.dir(i);
                let level = 1.0 - d.1.abs();
                let lies = level > 0.45 && l.w[i] > 1.2 && lie.get(l.pts[i].0, l.pts[i].1) > -0.25;
                if lies {
                    let nn = if d.0 >= 0.0 { (d.1, -d.0) } else { (-d.1, d.0) }; // upward normal
                    let s_w = (l.w[i] * 0.5 * smoothstep(0.45, 0.9, level)).clamp(0.6, 6.0);
                    run.push((l.pts[i].0 + nn.0 * (l.w[i] * 0.5 + s_w * 0.15), l.pts[i].1 + nn.1 * (l.w[i] * 0.5 + s_w * 0.15)));
                    run_w = run_w.max(s_w);
                }
                if (!lies || i + 1 == n) && run.len() >= 2 {
                    let mut held = Held::new(Tool { point: 0.5, ..Tool::round_sable(run_w * 1.1) }, r.next_u64());
                    held.load(white, 0.9);
                    c.drag(&mut held, &Gesture::new(run.clone()).pressure(0.8, 0.5).ramps(0.25, 0.35).shake(0.5), None);
                }
                if !lies {
                    run.clear();
                    run_w = 0.0;
                }
            }
            // a fork: a small cushion of snow in the crotch
            if let Some(pi) = l.parent {
                let par = &oak.limbs[pi];
                if l.w[0] > 3.0 && par.w[l.at.min(par.w.len() - 1)] > 5.0 && r.f() < 0.7 {
                    let q = par.pts[l.at.min(par.pts.len() - 1)];
                    let mut held = Held::new(Tool::round_sable((l.w[0] * 0.8).min(7.0)), r.next_u64());
                    held.load(white, 0.8);
                    c.touch(&mut held, &Touch::at(q.0, q.1 - l.w[0] * 0.3).pressure(0.6), None);
                }
            }
        }
        // heaps in the bark on the windward side of the bole
        let trunk = &oak.limbs[0];
        for i in 0..trunk.pts.len() {
            if trunk.w[i] < 8.0 {
                break;
            }
            let (x, y) = trunk.pts[i];
            for _ in 0..3 {
                let u = r.range(0.45, 0.95);
                let py = y + r.range(-4.0, 4.0);
                let px = x - trunk.w[i] * 0.5 * u;
                let mut held = Held::new(Tool { point: 0.6, ragged: 0.5, ..Tool::round_sable(r.range(1.2, 2.6)) }, r.next_u64());
                held.load(if r.f() < 0.7 { white } else { blue }, 0.6);
                let len = r.range(2.0, 7.0);
                c.drag(&mut held, &Gesture::new(vec![(px, py), (px + r.normal() * 0.4, py + len)]).pressure(0.6, 0.3).ramps(0.2, 0.5).shake(0.8), Some(&thick));
            }
        }
        c.dry();
    }

    // the foot of the tree: snow heaped round it, the drift banked on the
    // windward side
    let mound = Fbm::new(29, 3, 40.0);
    let tw0 = oak.limbs[0].w[0];
    let foot_m = Mask::from_fn(f, move |x, y| {
        let dx = (x - BASE.0 + 6.0) / (tw0 * 1.5);
        let top = BASE.1 - 4.0 - 9.0 * (1.0 - dx * dx).max(0.0) - 5.0 * (1.0 - ((x - BASE.0 + tw0 * 0.6) / 16.0).powi(2)).max(0.0) + 2.5 * mound.get(x, 0.0);
        smoothstep(top - 1.0, top + 1.0, y) * (1.0 - smoothstep(BASE.1 + 18.0, BASE.1 + 30.0, y)) * (1.0 - smoothstep(0.8, 1.2, dx.abs() * 0.5))
    });
    if o.stage("foot", &mut c, &mut rng) {
        let heap = Handling::new(Tool::round_sable(4.0)).mixed(pal, 0.1).pressure(0.7, 0.95).dips(3, 0.7, 0.8).clip(true).threshold(0.1).curve(0.04, 0.2).color(move |x, y| mix(snow_col(x, y), hex("#eeebe2"), 0.5 * smoothstep(BASE.1 + 12.0, BASE.1 - 10.0, y), Mix::Light)).angle(|_, _| 0.0).length(6.0, 18.0).coverage(3.0).medium(0.1);
        c.work(&foot_m, &heap, 51);
        c.dry();
    }

    if o.stage("fallen limb", &mut c, &mut rng) {
        // the limb the storm broke off, lying on the snow to the right,
        // half sunk: grey dead wood, snow along its top, a cool shadow below
        let mut r = Rng::new(o.seed + 901);
        let dead = pal.mix(hex("#4d4943")).paint(0.25);
        let shadow = pal.mix(hex("#a3a8b0")).paint(0.4);
        let white = pal.mix(hex("#efede6")).paint(0.08);
        let main = vec![(612.0, 1101.0), (650.0, 1098.0), (690.0, 1101.0), (726.0, 1097.0), (752.0, 1099.0)];
        let mut hs = Held::new(Tool::round_sable(5.0), r.next_u64());
        hs.load(shadow, 0.6);
        let sh: Vec<(f32, f32)> = main.iter().map(|p| (p.0 + 2.0, p.1 + 3.5)).collect();
        c.drag(&mut hs, &Gesture::new(sh).pressure(0.6, 0.3).ramps(0.2, 0.3).shake(0.5), None);
        let mut hm = Held::new(Tool { point: 0.4, ..Tool::round_sable(4.5) }, r.next_u64());
        hm.load(dead, 0.9);
        c.drag(&mut hm, &Gesture::new(main.clone()).pressure(0.9, 0.35).ramps(0.0, 0.4).shake(0.6), None);
        let claws = [((650.0, 1098.0), -2.3, 16.0), ((690.0, 1101.0), -1.1, 22.0), ((726.0, 1097.0), -0.7, 14.0), ((752.0, 1099.0), -2.8, 9.0)];
        for &(p, a, len) in &claws {
            shoot(&mut c, p, a, len, 0.8, dead, 1, &mut r);
        }
        // the broken end: a pale splintered face
        let wood = pal.mix(hex("#a39883")).paint(0.2);
        let mut hw = Held::new(Tool::round_sable(1.6), r.next_u64());
        hw.load(wood, 0.6);
        c.drag(&mut hw, &Gesture::new(vec![(611.0, 1099.0), (612.5, 1103.0)]).pressure(0.7, 0.4), None);
        let mut hsn = Held::new(Tool { point: 0.5, ..Tool::round_sable(2.2) }, r.next_u64());
        hsn.load(white, 0.8);
        let top: Vec<(f32, f32)> = main.iter().map(|p| (p.0, p.1 - 2.2)).collect();
        c.drag(&mut hsn, &Gesture::new(top).pressure(0.7, 0.4).ramps(0.2, 0.3).shake(0.8), None);
        c.dry();
    }

    if o.stage("grass", &mut c, &mut rng) {
        // dry grass and a few dead stalks through the snow, flicked up last
        // with fine upturning strokes [NG p.56], in tufts, more near us
        let mut r = Rng::new(o.seed + 1001);
        let straw = [pal.mix(hex("#8c7b58")).paint(0.2), pal.mix(hex("#6e604a")).paint(0.2), pal.mix(hex("#a08d66")).paint(0.2), pal.mix(hex("#4f463a")).paint(0.2)];
        let tufts = 70;
        for k in 0..tufts {
            let y = HORIZON + 25.0 + (h - HORIZON - 30.0) * r.f().powf(0.7);
            let near = ((y - HORIZON) / (h - HORIZON)).clamp(0.0, 1.0);
            let x = if k < 12 { BASE.0 + r.range(-70.0, 90.0) } else { r.range(10.0, 990.0) };
            let y = if k < 12 { BASE.1 + r.range(2.0, 30.0) } else { y };
            let blades = 3 + (r.f() * 8.0 * (0.4 + near)) as usize;
            for _ in 0..blades {
                let ht = r.range(4.0, 16.0) * (0.35 + near);
                let lean = r.range(-0.45, 0.45);
                let fx = x + r.normal() * 2.0 * (0.4 + near);
                let pts = vec![(fx, y), (fx + lean * ht * 0.3, y - ht * 0.55), (fx + lean * ht, y - ht)];
                let mut held = Held::new(Tool { point: 1.0, ..Tool::rigger(0.9 * (0.5 + near)) }, r.next_u64());
                held.load(straw[(r.f() * 4.0) as usize % 4], 0.7);
                c.drag(&mut held, &Gesture::new(pts).pressure(0.7, 0.0).ramps(0.05, 0.8).shake(0.5), None);
            }
        }
        c.dry();
    }

    if o.stage("leaves", &mut c, &mut rng) {
        // last year's leaves, still hanging on the young low twigs
        // (marcescence), rust brown, a few per tip, none in the dead top
        let mut r = Rng::new(o.seed + 1101);
        let leaf = [pal.mix(hex("#7b4b2c")).paint(0.15), pal.mix(hex("#8f6038")).paint(0.15), pal.mix(hex("#5e3d27")).paint(0.15)];
        for l in oak.limbs.iter().filter(|l| !l.is_empty() && !l.root && !l.dead && !l.broken) {
            let tip = *l.pts.last().unwrap();
            let low = smoothstep(BASE.1 - TREE_H * 0.35, BASE.1 - TREE_H * 0.62, tip.1);
            if r.f() > 0.55 * (1.0 - low) || l.w[l.w.len() - 1] > 1.5 {
                continue;
            }
            for _ in 0..(1 + (r.f() * 3.0) as usize) {
                let p = (tip.0 + r.normal() * 5.0, tip.1 + r.normal() * 4.0 + 2.0);
                let a = r.range(0.0, 6.28);
                let len = r.range(1.6, 3.2);
                let mut held = Held::new(Tool { point: 0.7, ..Tool::round_sable(1.6) }, r.next_u64());
                held.load(leaf[(r.f() * 3.0) as usize % 3], 0.7);
                c.drag(&mut held, &Gesture::new(vec![p, (p.0 + a.cos() * len, p.1 + a.sin() * len)]).pressure(0.8, 0.3).ramps(0.1, 0.5), None);
            }
        }
        c.dry();
    }

    o.end(&mut c, &mut rng);
    c.relief(st.relief.0, st.relief.1);
    o.save(&mut c);
    let _ = (&land_m, &oak, pal as &Palette, &Touch::at(0.0, 0.0), Orient::Across);
}

/// The brush for a mark `w` units wide: round sable for wood, pointed
/// rigger for twigs.
fn brush_for(w: f32) -> Tool {
    if w > 2.2 {
        Tool { ragged: 0.3, point: 0.4, ..Tool::round_sable(w * 1.05) }
    } else {
        let t = Tool { point: 1.0, ..Tool::rigger(w.max(0.5) * 1.15) };
        Tool { length: t.width * 4.0, ..t }
    }
}

fn cumulative(pts: &[(f32, f32)]) -> Vec<f32> {
    let mut arc = vec![0.0f32; pts.len()];
    for i in 1..pts.len() {
        arc[i] = arc[i - 1] + ((pts[i].0 - pts[i - 1].0).powi(2) + (pts[i].1 - pts[i - 1].1).powi(2)).sqrt();
    }
    arc
}

/// One limb from where it springs to its tip: a brush per stretch of taper,
/// each set down wet into the end of the last.
fn limb(c: &mut Canvas, l: &Limb, live: Paint, dead: Paint, rng: &mut Rng) {
    let n0 = l.pts.len();
    // the part wider than THICK is already laid in as a form; start a
    // little inside it
    let s0 = l.w.iter().position(|w| *w < THICK * 1.1).unwrap_or(n0 - 1);
    if s0 + 1 >= n0 {
        return;
    }
    let pts_all = &l.pts[s0..];
    let n = pts_all.len();
    let w: Vec<f32> = l.w[s0..].iter().map(|w| w.max(0.45)).collect();
    let arc = cumulative(pts_all);
    let dead_from = l.dead_from.saturating_sub(s0);
    let mut cuts = vec![0usize];
    for i in 1..n {
        let a = *cuts.last().unwrap();
        let run = brush_for(w[a]).run * 0.6;
        if (w[i] < w[a] * 0.5 || arc[i] - arc[a] > run || i == dead_from) && i + 1 < n {
            cuts.push(i);
        }
    }
    cuts.push(n - 1);
    for k in 0..cuts.len() - 1 {
        let (a, b) = (cuts[k], cuts[k + 1]);
        let last = k + 2 == cuts.len();
        let ov = w[a] * 1.5;
        let a0 = if k == 0 { a } else { (0..a).rev().find(|&i| arc[a] - arc[i] >= ov).unwrap_or(0) };
        let b0 = if last { b } else { (b + 1..n).find(|&i| arc[i] - arc[b] >= ov).unwrap_or(n - 1) };
        let pts: Vec<(f32, f32)> = pts_all[a0..=b0].to_vec();
        if pts.len() < 2 {
            continue;
        }
        let tool = brush_for(w[a]);
        let total = (arc[b0] - arc[a0]).max(1e-3);
        let p0 = tool.pressure_for(w[a]).max(0.3);
        let p1 = tool.pressure_for(w[b0]).max(0.04);
        let release = if last {
            if l.broken { 0.05 } else { (w[a].min(6.0) * 3.0 / total).clamp(0.2, 0.7) }
        } else {
            ((arc[b0] - arc[b]) / total).clamp(0.02, 0.5)
        };
        let paint = if a >= dead_from { dead } else { live };
        let mut held = Held::new(tool, rng.next_u64());
        // the finest twigs lean and dry: a haze, not a mass
        let thin = (w[a] / 1.2).clamp(0.35, 1.0);
        held.load(paint.with_hiding(paint.hiding() * (0.5 + 0.5 * thin)), 0.95 * thin.sqrt());
        let g = Gesture::new(pts).pressure(p0, p1).ramps(0.0, release).orient(Orient::Across).shake(0.7);
        c.drag(&mut held, &g, None);
    }
}

/// One fine shoot: a crooked twig of a few elbows lifted off to a point,
/// with `depth` levels of side twiglets.
#[allow(clippy::too_many_arguments)]
fn shoot(c: &mut Canvas, from: (f32, f32), angle: f32, len: f32, pressure: f32, p: Paint, depth: u32, r: &mut Rng) {
    let k = 3 + (r.f() * 2.0) as usize;
    let mut pts = vec![from];
    let mut a = angle;
    let mut q = from;
    for _ in 0..k {
        // oak twigs turn at every bud: a crooked, elbowed line, a little
        // upward overall
        a += r.range(-0.45, 0.45);
        a = a * 0.9 + (-1.5708) * 0.1 * (a.sin() > -0.95) as i32 as f32;
        let step = len / k as f32 * r.range(0.7, 1.3);
        q = (q.0 + a.cos() * step, q.1 + a.sin() * step);
        pts.push(q);
    }
    let mut held = Held::new(Tool { point: 1.0, length: 3.0, ..Tool::rigger(0.9) }, r.next_u64());
    held.load(p.with_hiding(p.hiding() * 0.7), 0.6);
    c.drag(&mut held, &Gesture::new(pts.clone()).pressure(pressure, 0.0).ramps(0.0, 0.7).orient(Orient::Along).shake(0.4), None);
    if depth > 0 {
        let m = (r.f() * 2.5) as usize;
        for _ in 0..m {
            let i = 1 + (r.f() * (pts.len() - 2) as f32) as usize;
            let side = if r.f() < 0.5 { -1.0 } else { 1.0 };
            let d = (pts[i + 1].0 - pts[i].0, pts[i + 1].1 - pts[i].1);
            shoot(c, pts[i], d.1.atan2(d.0) + side * r.range(0.5, 1.0), len * r.range(0.3, 0.55), pressure * 0.6, p, depth - 1, r);
        }
    }
}

/// The thick wood as segments, for the direction of the strokes laying it
/// in and for which side of a limb a point is on.
struct Segs {
    s: Vec<((f32, f32), (f32, f32), f32, f32, bool)>,
}

struct Near {
    angle: f32,
    /// Across the limb: −1 at its left edge, +1 at its right.
    u: f32,
    dead: bool,
    /// The axis' x at this point.
    cx: f32,
}

impl Segs {
    fn of(sk: &Skeleton, min_w: f32) -> Segs {
        let mut s = Vec::new();
        for l in sk.limbs.iter().filter(|l| !l.is_empty() && !l.root) {
            for i in 0..l.pts.len() - 1 {
                if l.w[i] > min_w * 0.7 {
                    s.push((l.pts[i], l.pts[i + 1], l.w[i], l.w[i + 1], l.dead_at(i)));
                }
            }
        }
        Segs { s }
    }
    fn near(&self, x: f32, y: f32) -> Near {
        let mut best = (f32::MAX, Near { angle: -1.5708, u: 0.0, dead: false, cx: x });
        for &(a, b, wa, wb, dead) in &self.s {
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let l2 = (dx * dx + dy * dy).max(1e-6);
            let t = (((x - a.0) * dx + (y - a.1) * dy) / l2).clamp(0.0, 1.0);
            let (px, py) = (a.0 + dx * t, a.1 + dy * t);
            let w = wa + (wb - wa) * t;
            let d = ((x - px).powi(2) + (y - py).powi(2)).sqrt() / (0.5 * w).max(0.5);
            if d < best.0 {
                let len = l2.sqrt();
                // signed: left of the axis (smaller x for an upright limb) negative
                let side = if (x - px) * (-dy / len) + (y - py) * (dx / len) >= 0.0 { 1.0 } else { -1.0 };
                let across = if dy < 0.0 { -side } else { side };
                best = (d, Near { angle: dy.atan2(dx), u: across * d.min(1.0), dead, cx: px });
            }
        }
        best.1
    }
}

#[allow(dead_code)]
fn skel_info(sk: &Skeleton) {
    let b = sk.bounds();
    eprintln!("limbs {} bounds {:?} pipe {}", sk.limbs.len(), b, sk.pipe);
}
