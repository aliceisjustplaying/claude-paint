//! r13_tree1: one tree in winter. An old pedunculate oak, stag-headed (its
//! top limbs dead and broken, one sawn off), standing alone in snow under a
//! pale winter sky. After Friedrich's habits: a precise pencil drawing of the
//! tree on the warm ground, a thin stippled sky, the tree painted on the
//! finished sky, the trunk in body color worked along the form, limbs and
//! twigs pressed where they leave their parent and lifted off to the tip,
//! snow laid in bands on the upper sides of limbs, grass flicked up last.
//!
//!   cargo paint r13_tree1 -- --full --width 2400
//!   cargo paint r13_tree1 -- --full --width 2400 --crop 300,300,600,600

use paint::color::{Mix, mix};
use paint::graphite::hand_line;
use paint::{Fbm, Gesture, Held, Lead, Mask, Rgb, Rng, Stipple, Style, Tool, Touch, hex, smoothstep};
use paintings::run::{Finish, Run};

/// Canvas width / height: an upright picture for a whole tree.
const ASPECT: f32 = 0.8;
/// The horizon (units from the top): low, as he set it.
const HORIZON: f32 = 1010.0;
/// Where the trunk stands in the snow.
const BASE: (f32, f32) = (478.0, 1000.0);
/// Widths (units) at which a limb changes hands: above THICK the limb is
/// worked as an area; between FINE and THICK, a pointed round; below, the rigger.
const THICK: f32 = 3.2;
const FINE: f32 = 0.95;

fn main() {
    let o = Run::new("r13_tree1");
    let st = Style::friedrich();
    let pal = &st.palette;
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = c.height();

    // ---- the tree, as a set of limbs (built once, the same every run)
    let tree = Tree::grow(o.seed + 5);
    eprintln!("tree: {} limbs, {} segments", tree.limbs.len(), tree.limbs.iter().map(|l| l.pts.len()).sum::<usize>());
    if let Ok(p) = std::env::var("TREE_PGM") {
        tree.pgm(&p, h);
        return;
    }

    // the snow's surface: a low swell where the oak stands, a long drift
    let land_n = Fbm::new(o.seed as u32 + 3, 4, 260.0);
    let snow_top = f.per_column(move |x| {
        let swell = 22.0 * (-((x - BASE.0) / 240.0).powi(2)).exp();
        HORIZON + 6.0 + 8.0 * land_n.get(x, 0.0) + 10.0 * smoothstep(0.0, 1000.0, 1000.0 - x) - swell
    });
    let sky_m = Mask::from_fn(f, move |x, y| 1.0 - smoothstep(snow_top(x) - 14.0, snow_top(x) - 10.0, y));
    let snow_m = Mask::from_fn(f, move |x, y| smoothstep(snow_top(x) - 1.2, snow_top(x) + 1.2, y));

    // ---- the drawing: the tree's frame in pencil, the snow line faint
    if o.stage("drawing", &mut c, &mut rng) {
        let hb = Lead::pencil("HB").unwrap();
        let hard = Lead::pencil("H").unwrap();
        let mut worn = 0.0;
        let mut k = 0u64;
        for l in tree.limbs.iter().filter(|l| l.w[0] > 1.6) {
            // both contours of thick limbs, the axis of thinner ones
            let lead = if l.w[0] > 6.0 { &hb } else { &hard };
            let sides: &[f32] = if l.w[0] > 6.0 { &[-0.5, 0.5] } else { &[0.0] };
            for &s in sides {
                let pts: Vec<(f32, f32)> = (0..l.pts.len()).map(|i| l.offset(i, s)).collect();
                let prof: Vec<f32> = l.w.iter().map(|w| (0.25 + 0.04 * w).min(0.6)).collect();
                let m = hand_line(&pts, &prof, true, false, 0.35, o.seed + k);
                worn += c.draw(lead, &m, worn, o.seed + 1000 + k);
                if worn > 200.0 {
                    worn = 0.0;
                }
                k += 1;
            }
        }
        let pts: Vec<(f32, f32)> = (0..=40).map(|i| {
            let x = i as f32 * 25.0;
            (x, snow_top(x))
        }).collect();
        let m = hand_line(&pts, &[0.25, 0.3, 0.2], true, false, 0.5, o.seed + 77);
        c.draw(&hard, &m, 0.0, o.seed + 78);
    }

    // ---- the sky: a cold gray-blue above, paler and warmer down to a low
    // band of haze over the far woods
    let clouds = Fbm::new(o.seed as u32 + 11, 5, 300.0);
    let sky = move |x: f32, y: f32| {
        let t = (y / HORIZON).clamp(0.0, 1.0);
        // long, low strata of cloud, darker, stretched across
        let s = clouds.get(x * 0.35, y * 2.2);
        let band = smoothstep(0.05, 0.35, s) * smoothstep(0.15, 0.4, t) * (1.0 - smoothstep(0.78, 0.92, t));
        let base = paint::gradient(
            &[(0.0, hex("#76818d")), (0.3, hex("#8f98a0")), (0.62, hex("#b3b5b1")), (0.86, hex("#d6cfbd")), (0.95, hex("#dcd2bc")), (1.0, hex("#c9c3b6"))],
            t,
            Mix::Light,
        );
        mix(base, hex("#8e8c8f"), 0.35 * band, Mix::Light)
    };
    if o.stage("sky", &mut c, &mut rng) {
        let lay = st.broad().color(move |x, y| mix(sky(x, y), hex("#6c6e74"), 0.06, Mix::Light)).angle(|_, _| 0.03).coverage(4.5).medium(0.3).dips(1, 0.6, 0.6);
        c.work(&sky_m, &lay, 11);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.angle(|_, _| 0.0), 12);
        }
        let s1 = Stipple::new(Tool::stippler(2.6)).mixed(pal, 0.45).color(sky).coverage(|_, _| 2.0).pressure(0.5, 0.85).dips(20, 0.4, 0.5);
        c.stipple(&sky_m, &s1, 13);
        c.dry();
    }

    if o.stage("sky light", &mut c, &mut rng) {
        // dry: a finer, lighter stipple, thickening toward the pale band
        let glow = move |x: f32, y: f32| mix(sky(x, y), hex("#ece2c6"), 0.05 + 0.2 * smoothstep(420.0, 930.0, y), Mix::Light);
        let s2 = Stipple::new(Tool::stippler(1.5)).mixed(pal, 0.5).color(glow).coverage(|_, y| 0.4 + 1.7 * smoothstep(250.0, 930.0, y)).pressure(0.45, 0.8).dips(24, 0.35, 0.6);
        c.stipple(&sky_m, &s2, 14);
        c.dry();
    }

    // far woods: a low, broken band of blue-gray at the horizon
    let wood_n = Fbm::new(o.seed as u32 + 21, 5, 40.0);
    let woods_top = f.per_column(move |x| {
        let lump = 7.0 + 9.0 * wood_n.get(x, 3.0) + 4.0 * wood_n.get(x * 4.0, 9.0);
        snow_top(x) - 9.0 - lump.max(0.0) * smoothstep(-0.15, 0.25, wood_n.get(x * 0.25, 1.0))
    });
    let woods_m = Mask::from_fn(f, move |x, y| smoothstep(woods_top(x) - 0.8, woods_top(x) + 0.8, y) * (1.0 - smoothstep(snow_top(x) + 2.0, snow_top(x) + 4.0, y)));
    if o.stage("land", &mut c, &mut rng) {
        let woods = st.hatch().color(|x, y| mix(hex("#a3a5a8"), hex("#8f9095"), smoothstep(985.0, 1005.0, y) * (0.6 + 0.4 * smoothstep(0.0, 400.0, x)), Mix::Light)).angle(|_, _| -1.45).cross(0.3).length(3.0, 8.0).coverage(3.0);
        c.work(&woods_m, &woods, 21);
        c.dry();
        // the snow: lead white in body, lit from the low light at left and
        // cooler toward the right and in the hollows; strokes lie with the ground
        let lie = Fbm::new(o.seed as u32 + 23, 4, 120.0);
        let snow_col = move |x: f32, y: f32| {
            let depth = smoothstep(HORIZON, h, y);
            let cool = 0.5 + 0.5 * lie.get(x * 0.5, y * 2.0);
            let lit = paint::gradient(&[(0.0, hex("#dcd8cc")), (0.5, hex("#e6e2d6")), (1.0, hex("#ece8dc"))], depth, Mix::Light);
            mix(lit, hex("#b6bcc2"), 0.35 * cool * (0.4 + 0.6 * smoothstep(300.0, 1000.0, x)), Mix::Light)
        };
        let snow = st.body().color(snow_col).angle(|_, _| 0.02).angle_jitter(0.08).length(25.0, 80.0).coverage(4.0).curve(0.03, 0.2);
        c.work(&snow_m, &snow, 22);
        c.wait(20.0);
        // the snow lies in low waves: long cool hollows between them, a
        // bluer gray under the gray sky, fainter far off
        let waves = Fbm::new(o.seed as u32 + 25, 4, 200.0);
        let hollows = Mask::from_fn(f, move |x, y| {
            let v = waves.get(x * 0.18, y * 2.6);
            smoothstep(0.0, 0.4, v) * snow_m.sample(x, y) * smoothstep(snow_top(x) + 6.0, snow_top(x) + 30.0, y)
        });
        let cool = st.body().color(move |x, y| mix(hex("#d3d5d4"), hex("#c1c6cb"), smoothstep(HORIZON, h, y) * smoothstep(200.0, 900.0, x), Mix::Light)).angle(|_, _| 0.03).length(40.0, 110.0).coverage(1.8).medium(0.45);
        c.work(&hollows, &cool, 24);
        c.dry();
    }

    // ---- the tree
    let thick = tree.thick_mask(f);
    let dir = tree.clone();
    let tree_col = tree.clone();
    if o.stage("trunk", &mut c, &mut rng) {
        // body color worked along the limb, darker on the shadow side, the
        // lit side grayer with lichen
        let body = paint::Handling::new(Tool { lay: 0.8, ..Tool::filbert(5.0) })
            .mixed(pal, st.body_medium)
            .mix_jitter(st.mix_jitter * 1.4)
            .pressure(0.6, 0.9)
            .dips(2, 0.56, 0.6)
            .drift(0.1, 60.0)
            .tail(0.15)
            .broken(0.1)
            .swell(0.22)
            .color(move |x, y| tree_col.bark(x, y))
            .angle(move |x, y| dir.angle_at(x, y))
            .angle_jitter(0.06)
            .length(10.0, 38.0)
            .coverage(4.5)
            .clip(true)
            .curve(0.04, 0.3);
        c.work(&thick, &body, 31);
        c.wait(30.0);
        // bark: the fissures of an old oak run lengthwise and break into
        // long blocks; drawn with a small round, darker, into the wet paint
        let mut b = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.6) }, 32);
        let mut r = Rng::new(o.seed + 33);
        let dark = pal.paint(hex("#221d19"), 0.15);
        let light = pal.paint(hex("#8a8779"), 0.15);
        for l in tree.limbs.iter().filter(|l| l.w[0] > 7.0) {
            let n = (l.w[0] / 3.2) as usize;
            for j in 0..n {
                let across = -0.42 + 0.84 * (j as f32 + r.range(0.1, 0.9)) / n as f32;
                // fissures in lengths of 15-50 units along the limb
                let mut i = below(&mut r, 3);
                while i + 2 < l.pts.len() && l.w[i] > 4.0 {
                    let run = 2 + below(&mut r, 5);
                    let e = (i + run).min(l.pts.len() - 1);
                    let pts: Vec<(f32, f32)> = (i..=e).map(|k| {
                        let (x, y) = l.offset(k, across);
                        (x + r.range(-0.4, 0.4), y)
                    }).collect();
                    let lit = tree.light_at_offset(l, i, across);
                    if r.f() < 0.72 {
                        b.reload(if lit > 0.62 && r.f() < 0.5 { light.clone() } else { dark.clone() }, 0.7);
                        let w = (l.w[i] * 0.05).clamp(0.5, 2.2);
                        let p = b.tool.pressure_for(w);
                        c.drag(&mut b, &Gesture::new(pts).pressure(p, p * 0.6).ramps(0.2, 0.4).shake(0.7), Some(&thick));
                    }
                    i = e + 1 + below(&mut r, 3);
                }
            }
        }
        c.dry();
    }

    if o.stage("limbs", &mut c, &mut rng) {
        let mut r = Rng::new(o.seed + 41);
        for (li, l) in tree.limbs.iter().enumerate() {
            // the part of this limb between THICK and FINE
            let i0 = l.w.iter().position(|&w| w < THICK + 0.6).unwrap_or(l.w.len());
            let i1 = l.w.iter().position(|&w| w < FINE).unwrap_or(l.w.len());
            if i1 <= i0 + 1 {
                continue;
            }
            let i0 = i0.saturating_sub(1);
            let mut rb = Held::new(round_for(l.w[i0]), 42 + li as u64);
            paint_run(&mut c, &mut rb, pal, l, i0, i1, &mut r, &tree);
        }
        c.wait(10.0);
    }

    if o.stage("twigs", &mut c, &mut rng) {
        let mut r = Rng::new(o.seed + 51);
        let mut rg = Held::new(Tool { point: 1.0, ..Tool::rigger(1.2) }, 52);
        for l in tree.limbs.iter() {
            let i0 = l.w.iter().position(|&w| w < FINE + 0.25).unwrap_or(l.w.len()).saturating_sub(1);
            if i0 + 1 >= l.pts.len() {
                continue;
            }
            paint_run(&mut c, &mut rg, pal, l, i0, l.pts.len(), &mut r, &tree);
        }
        c.dry();
    }

    if o.stage("snow on tree", &mut c, &mut rng) {
        let mut r = Rng::new(o.seed + 61);
        let mut b = Held::new(Tool { point: 0.6, ..Tool::round_sable(3.0) }, 62);
        let white = pal.paint(hex("#ece8de"), 0.05).with_stiff(0.9);
        let blue = pal.paint(hex("#c4c8cc"), 0.05);
        for l in tree.limbs.iter().filter(|l| l.order != 99 && (l.w[0] > 1.3 && !l.dead || l.w[0] > 2.5)) {
            let mut i = 0;
            while i + 1 < l.pts.len() {
                let (dx, dy) = l.dir(i);
                let flat = 1.0 - dy.abs() / (dx.abs() + dy.abs() + 1e-6) * 1.4;
                if flat < -0.05 || l.w[i] < 1.6 {
                    i += 1;
                    continue;
                }
                // a band of snow of 2-8 segments, then a gap where it slid off
                let run = 2 + below(&mut r, 6);
                let e = (i + run).min(l.pts.len() - 1);
                if r.f() < 0.3 + 0.55 * flat * smoothstep(1.5, 6.0, l.w[i]).max(0.4) {
                    let up = if dx >= 0.0 { -0.5 } else { 0.5 };
                    let pts: Vec<(f32, f32)> = (i..=e).map(|k| {
                        let (x, y) = l.offset(k, up * 0.92);
                        (x, y - 0.25 * l.w[k].min(8.0) * 0.25)
                    }).collect();
                    let w = (l.w[i] * 0.3).clamp(0.7, 5.0) * (0.45 + 0.6 * flat.max(0.0)) * r.range(0.7, 1.1);
                    b = Held::new(Tool { point: 0.6, ..round_for(w) }, r.next_u64());
                    b.reload(if r.f() < 0.2 { blue.clone() } else { white.clone() }, 0.9);
                    let p = b.tool.pressure_for(w);
                    c.drag(&mut b, &Gesture::new(pts).pressure(p * 0.8, p * 0.5).ramps(0.25, 0.45).shake(0.6), None);
                }
                i = e + 1 + below(&mut r, 4);
            }
        }
        // snow lodged in the great crotch where the boughs part: a few
        // short strokes piled on each other, cool underneath
        let top = *tree.limbs[0].pts.last().unwrap();
        for k in 0..9 {
            let x = top.0 + r.range(-16.0, 16.0);
            let y = top.1 + 12.0 + r.range(-5.0, 4.0) + (x - top.0).abs() * 0.3;
            let ww = r.range(2.5, 5.0);
            b = Held::new(Tool { point: 0.4, ..round_for(ww) }, 600 + k);
            b.reload(if k < 3 { blue.clone() } else { white.clone() }, 0.9);
            let p = b.tool.pressure_for(ww);
            let len = r.range(5.0, 11.0);
            c.drag(&mut b, &Gesture::new(vec![(x - len * 0.5, y + 0.8), (x, y - 1.0), (x + len * 0.5, y + 0.6)]).pressure(p, p * 0.6).ramps(0.3, 0.4).shake(0.6), None);
        }
        c.dry();
    }

    if o.stage("foot", &mut c, &mut rng) {
        // the snow drifted against the foot of the trunk, its shadow side
        // cool, and the ground's own low shadows; then grass and weeds
        // flicked up through it, last
        let mut r = Rng::new(o.seed + 71);
        let (bx, by) = BASE;
        // the drift: a mound piled round the foot, higher on the weather
        // (left) side; worked with the snow's own strokes along its contour
        let mound = move |x: f32| {
            let u = (x - bx) / 95.0;
            let bump = (1.0 - u * u).max(0.0).powf(1.5);
            by + 4.0 - (7.0 + 3.0 * (-u).max(0.0)) * bump
        };
        let drift_m = Mask::from_fn(f, move |x, y| {
            let u = ((x - bx) / 100.0).abs();
            smoothstep(mound(x) - 0.7, mound(x) + 0.7, y) * (1.0 - smoothstep(by + 6.0, by + 20.0, y)) * (1.0 - smoothstep(0.55, 1.0, u))
        });
        let drift = st
            .body()
            .color(move |x, y| {
                let side = smoothstep(-20.0, 50.0, x - bx);
                let under = 1.0 - smoothstep(mound(x), mound(x) + 6.0, y);
                mix(hex("#e9e5d9"), hex("#c3c8cd"), (0.55 * side + 0.35 * under * side).min(0.9), Mix::Light)
            })
            .angle(move |x, _| {
                let d = mound(x + 1.0) - mound(x - 1.0);
                (d * 0.5).atan()
            })
            .length(10.0, 30.0)
            .coverage(4.0)
            .clip(true);
        c.work(&drift_m, &drift, 72);
        // the trunk's cool shadow where it meets the drift
        let mut b = Held::new(Tool::filbert(4.0), 73);
        for k in 0..8 {
            let x = bx + 6.0 + k as f32 * 4.5 + r.range(-1.0, 1.0);
            b.reload(pal.paint(hex("#9fa6ae"), 0.2), 0.5);
            let y = mound(x) + 1.5;
            c.drag(&mut b, &Gesture::line((x - 5.0, y + 0.5), (x + 6.0, y + 1.0)).pressure(0.45, 0.3).shake(0.7), None);
        }
        // grass: fine upturned strokes, in clumps, over the snow
        let mut g = Held::new(Tool { point: 1.0, ..Tool::rigger(0.9) }, 73);
        let grass = [hex("#6a5e4c"), hex("#857657"), hex("#4a4238"), hex("#9a8c6c")];
        // in patches: where a ditch or bank lies under the snow, the dry
        // grass stands through it; elsewhere a few stalks alone
        let mut clumps: Vec<(f32, f32, usize)> = vec![];
        for &(px, py, spread, count) in &[(150.0f32, 1150.0f32, 120.0f32, 14usize), (820.0, 1185.0, 150.0, 16), (620.0, 1060.0, 90.0, 8), (300.0, 1045.0, 70.0, 6), (40.0, 1225.0, 60.0, 7)] {
            for _ in 0..count {
                let x = px + r.normal() * spread * 0.5;
                let y = (py + r.normal() * spread * 0.12).max(snow_top(x) + 8.0).min(h - 6.0);
                clumps.push((x, y, 3 + below(&mut r, 8)));
            }
        }
        for _ in 0..10 {
            let x = r.range(10.0, 990.0);
            let y0 = snow_top(x) + 10.0;
            clumps.push((x, y0 + (h - y0 - 10.0) * r.f(), 1 + below(&mut r, 3)));
        }
        for (cx, cy, n) in clumps {
            let near = smoothstep(HORIZON, h, cy);
            for _ in 0..n {
                let x = cx + r.range(-4.0, 4.0) * (1.0 + 2.0 * near);
                let y = cy + r.range(-1.5, 1.5);
                let ht = (6.0 + 22.0 * r.f()) * (0.4 + 1.1 * near);
                let lean = r.range(-0.5, 0.5);
                g.reload(pal.paint(grass[below(&mut r, 4)], 0.2), 0.6);
                let pts = vec![(x, y), (x + lean * ht * 0.3, y - ht * 0.5), (x + lean * ht, y - ht)];
                c.drag(&mut g, &Gesture::new(pts).pressure(0.55 + 0.3 * near, 0.0).ramps(0.05, 0.85).shake(0.6), None);
            }
        }
        // a fallen limb from the dead crown, lying half in the snow
        let mut fl = Held::new(Tool { point: 1.0, ..Tool::round_sable(4.0) }, 74);
        let lx = bx + 170.0;
        let ly = snow_top(lx) + 95.0;
        let limb: Vec<(f32, f32)> = (0..8).map(|i| (lx + i as f32 * 16.0, ly - i as f32 * 2.2 + (i as f32 * 1.3).sin() * 1.8)).collect();
        // its shadow in the snow first, cool, just below
        let mut sh = Held::new(Tool::filbert(5.0), 77);
        sh.reload(pal.paint(hex("#aeb4bb"), 0.25), 0.6);
        c.drag(&mut sh, &Gesture::new(limb.iter().map(|&(x, y)| (x + 3.0, y + 3.5)).collect()).pressure(0.5, 0.3).shake(0.6), None);
        fl.reload(pal.paint(hex("#4a443c"), 0.1), 1.0);
        c.drag(&mut fl, &Gesture::new(limb.clone()).pressure(0.8, 0.35).ramps(0.05, 0.3).shake(0.6), None);
        let mut fl = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.8) }, 76);
        for &(i, a, len) in &[(2usize, -2.2f32, 18.0f32), (4, -0.9, 14.0), (6, -2.5, 11.0), (5, 0.4, 8.0)] {
            let (x, y) = limb[i];
            fl.reload(pal.paint(hex("#51493f"), 0.1), 0.8);
            c.drag(&mut fl, &Gesture::new(vec![(x, y), (x + a.cos() * len * 0.5, y + a.sin() * len * 0.5), (x + a.cos() * len, y + a.sin() * len - 2.0)]).pressure(0.7, 0.0).ramps(0.05, 0.7).shake(0.6), None);
        }
        // snow along its top, and drifted over it in two places
        let mut sw = Held::new(Tool::round_sable(2.5), 75);
        sw.reload(pal.paint(hex("#ede9df"), 0.05), 0.9);
        c.drag(&mut sw, &Gesture::new(limb[..5].iter().map(|&(x, y)| (x, y - 2.4)).collect()).pressure(0.6, 0.3).ramps(0.2, 0.3).shake(0.6), None);
        let mut sd = Held::new(Tool::filbert(6.0), 78);
        for &i in &[1usize, 5] {
            let (x, y) = limb[i];
            sd.reload(pal.paint(hex("#e6e2d7"), 0.1), 0.9);
            c.drag(&mut sd, &Gesture::new(vec![(x - 8.0, y + 1.0), (x, y - 2.5), (x + 9.0, y + 1.5)]).pressure(0.7, 0.5).shake(0.6), None);
        }
        c.dry();
    }

    o.finish(&mut c, &mut rng, &Finish { varnish: hex("#ffffff"), varnish_coats: 0.0, varnish_vary: 0.0, cracks: None, relief: st.relief });
}

/// Paint a limb's points i0..i1 in strokes of at most ~50 units, pressed at
/// the thick end and lifted toward the tip, reloading between strokes.
fn paint_run(c: &mut paint::Canvas, b: &mut Held, pal: &paint::Palette, l: &Limb, i0: usize, i1: usize, r: &mut Rng, tree: &Tree) {
    let mut i = i0;
    let i1 = i1.min(l.pts.len());
    while i + 1 < i1 {
        let mut e = i + 1;
        let mut len = 0.0;
        while e + 1 < i1 && len < 45.0 + 20.0 * r.f() {
            len += dist(l.pts[e], l.pts[e + 1]);
            e += 1;
        }
        let pts: Vec<(f32, f32)> = l.pts[i..=e].to_vec();
        let (wa, wb) = (l.w[i], l.w[e]);
        let col = tree.limb_color(l, i, r);
        b.reload(pal.paint(col, if wa < 1.0 { 0.25 } else { 0.15 }), if wa < 1.0 { 0.55 } else { 0.8 });
        let pa = b.tool.pressure_for(wa);
        let pb = b.tool.pressure_for(wb.max(0.3));
        let tip = e + 1 >= l.pts.len();
        let knots: Vec<f32> = (0..=4).map(|k| 1.0 + r.range(-0.08, 0.08) * (k as f32 / 4.0)).collect();
        let g = Gesture::new(pts).pressure(pa, if tip { 0.0 } else { pb }).ramps(0.04, if tip { 0.6 } else { 0.15 }).swell(knots).shake(0.55);
        c.drag(b, &g, None);
        i = e;
    }
}

fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

/// One limb: a polyline from where it leaves its parent to its tip, with its
/// width (units) at each point.
#[derive(Clone)]
struct Limb {
    pts: Vec<(f32, f32)>,
    w: Vec<f32>,
    dead: bool,
    order: u32,
}

impl Limb {
    fn dir(&self, i: usize) -> (f32, f32) {
        let (a, b) = if i + 1 < self.pts.len() { (self.pts[i], self.pts[i + 1]) } else { (self.pts[i - 1], self.pts[i]) };
        let d = dist(a, b).max(1e-4);
        ((b.0 - a.0) / d, (b.1 - a.1) / d)
    }
    /// The point at fraction `s` of the half-width to the limb's left (-)
    /// or right (+) of its axis (looking along it).
    fn offset(&self, i: usize, s: f32) -> (f32, f32) {
        let (dx, dy) = self.dir(i);
        let (nx, ny) = (-dy, dx);
        (self.pts[i].0 + nx * s * self.w[i], self.pts[i].1 + ny * s * self.w[i])
    }
}

#[derive(Clone)]
struct Tree {
    limbs: Vec<Limb>,
    /// Segments of the thick limbs: (a, b, wa, wb, limb) and a grid over them.
    segs: Vec<((f32, f32), (f32, f32), f32, f32, usize)>,
    grid: Vec<Vec<u32>>,
    forks: Vec<(f32, f32, f32)>,
    bark_n: Fbm,
}

const CELL: f32 = 12.0;
const GW: usize = 90;
const GH: usize = 110;

impl Tree {
    fn grow(seed: u64) -> Tree {
        let mut r = Rng::new(seed);
        let mut t = Tree { limbs: vec![], segs: vec![], grid: vec![vec![]; GW * GH], forks: vec![], bark_n: Fbm::new(seed as u32 + 9, 4, 30.0) };
        // the trunk: short and massive, leaning a little, with a root flare
        let mut pts = vec![];
        let mut w = vec![];
        let (mut x, mut y) = (BASE.0, BASE.1 + 6.0);
        let mut a = -1.57f32 + 0.05;
        let n = 32;
        for i in 0..=n {
            let t = i as f32 / n as f32;
            pts.push((x, y));
            // flare at the foot, the bole, a swelling where the boughs part
            // an old oak's bole is not a column: burrs and swellings
            let burr = 3.5 * (t * 23.0 + 1.3).sin() * (t * 7.0).cos() + 2.5 * (t * 41.0).sin();
            let wt = 54.0 + 30.0 * (-t * 16.0).exp() - 8.0 * t + 6.0 * smoothstep(0.8, 1.0, t) + burr;
            w.push(wt);
            a += r.range(-0.03, 0.03) - 0.004 * (a + 1.57 - 0.03) + 0.012 * (t * 9.0).sin();
            x += a.cos() * 8.5;
            y += a.sin() * 8.5;
        }
        t.limbs.push(Limb { pts, w, dead: false, order: 0 });
        let top = (x, y);
        t.forks.push((top.0, top.1, 40.0));
        // the boughs: four from the crown of the bole, spreading and
        // twisting; the two highest dead (a stag head), one sawn off
        let boughs: [(f32, f32, bool, f32); 6] = [
            (-2.8, 22.0, false, 1.5),  // very low left, near level, the longest
            (-2.2, 26.0, false, 1.3),  // left
            (-1.72, 24.0, true, 1.9),  // the old leader: dead, bleached, broken high
            (-1.38, 19.0, true, 1.7),  // a second dead limb beside it
            (-0.85, 28.0, false, 1.35), // right, the living crown
            (-0.2, 17.0, false, 0.8),  // low right: sawn off
        ];
        for (k, &(ang, bw, dead, reach)) in boughs.iter().enumerate() {
            let sx = top.0 + (k as f32 - 2.5) * 7.0;
            let sy = top.1 + 10.0 + (k as f32 - 2.5).abs() * 7.0;
            let sawn = k == 5;
            t.branch(&mut r, (sx, sy), ang, bw, 0, dead, reach, sawn);
        }
        // roots: the foot spreads into buttresses that run down into the snow
        for &(ang, rw) in &[(2.55f32, 26.0f32), (2.2, 18.0), (0.62, 24.0), (0.95, 16.0), (1.7, 20.0)] {
            let p = (BASE.0 + ang.cos() * 20.0, BASE.1 - 22.0);
            let mut pts = vec![p];
            let mut ws = vec![rw];
            let (mut x, mut y, mut a, mut w) = (p.0, p.1, ang, rw);
            for _ in 0..5 {
                a += r.range(-0.1, 0.1);
                x += a.cos() * 6.0;
                y += a.sin() * 6.0;
                w *= 0.8;
                pts.push((x, y));
                ws.push(w);
            }
            t.limbs.push(Limb { pts, w: ws, dead: false, order: 99 });
        }
        // epicormic shoots on the bole: a few thin sprouts from old wounds
        for _ in 0..7 {
            let i = 8 + below(&mut r, 20);
            let l = t.limbs[0].clone();
            let side = if r.f() < 0.5 { -0.5 } else { 0.5 };
            let p = l.offset(i, side * 0.95);
            let a = if side < 0.0 { -2.3 } else { -0.8 } + r.range(-0.3, 0.3);
            let ew = r.range(1.2, 2.2);
            t.branch(&mut r, p, a, ew, 3, false, 0.6, false);
        }
        t.index();
        t
    }

    /// Grow a limb and, recursively, its side limbs. `reach` scales length.
    #[allow(clippy::too_many_arguments)]
    fn branch(&mut self, r: &mut Rng, p: (f32, f32), ang: f32, w0: f32, order: u32, dead: bool, reach: f32, sawn: bool) {
        if self.limbs.len() > 40_000 {
            return;
        }
        let mut pts = vec![p];
        let mut ws = vec![w0];
        let (mut x, mut y) = p;
        let mut a = ang;
        let mut home = ang;
        let mut w = w0;
        let mut kids: Vec<((f32, f32), f32, f32)> = vec![];
        let mut since = 0.0;
        let mut side = if r.f() < 0.5 { 1.0 } else { -1.0 };
        let internode = |w: f32| 6.0 + w * 2.0;
        let mut next = internode(w) * r.range(0.6, 1.2);
        // dead limbs end broken: they stop at a width well above a twig
        let stop_w = if sawn { w0 * 0.72 } else if dead && order > 0 { w0 * r.range(0.5, 0.8) } else if dead { w0 * r.range(0.25, 0.4) } else { 0.32 };
        let mut len = 0.0;
        let max_len = if sawn { 70.0 } else { reach * (26.0 + 16.0 * w0.powf(0.95)) };
        let step_of = |w: f32| (w * 0.55).clamp(1.2, 7.0);
        let mut guard = 0;
        while w > stop_w && len < max_len && guard < 400 {
            guard += 1;
            let step = step_of(w);
            // oak: crooked, the direction wandering and kinking; thick limbs
            // spread outward and lift at their ends; twigs rise
            let up = -1.57f32;
            // each limb holds to its own heading (which turns up a little
            // as it thins); dead wood, no longer growing, just holds
            home += 0.012 * angle_diff(up, home) * (if dead { 0.3 } else { 1.0 }) * (step / 3.0) / (1.0 + w * 0.5);
            let hold = if w > 8.0 { 0.12 } else if w > 2.0 { 0.07 } else { 0.035 };
            let want = a + hold * angle_diff(home, a);
            a = want + r.normal() * (0.05 + 0.14 / (1.0 + w * 0.4));
            x += a.cos() * step;
            y += a.sin() * step;
            len += step;
            since += step;
            // gradual taper between forks
            w *= 1.0 - (if dead { 0.0018 } else { 0.0022 }) * step;
            if since >= next && order < 9 {
                since = 0.0;
                next = internode(w) * r.range(0.6, 1.4);
                // a side limb: a quarter to two thirds of the mother's width;
                // the mother keeps area ~ w^1.9 - wc^1.9
                let frac = if w > 8.0 { r.range(0.22, 0.5) } else { r.range(0.4, 0.8) };
                let wc = w * frac;
                if dead {
                    // dead wood grows no more: it keeps only the stubs of
                    // limbs it had, and its line is crooked where they were
                    a += r.range(-0.35, 0.35);
                }
                if wc > 0.3 && !(dead && (r.f() < 0.6 || wc < 2.0)) {
                    let ca = a + side * r.range(0.45, 1.05);
                    kids.push(((x, y), ca, wc));
                    self.forks.push((x, y, wc));
                    let e = 1.9f32;
                    w = (w.powf(e) - wc.powf(e)).max(0.0).powf(1.0 / e).max(w * 0.6);
                    // the mother kinks away from the child
                    a -= side * r.range(0.08, 0.3) * (1.0 / (1.0 + w * 0.08));
                    side = -side;
                    if r.f() < 0.25 {
                        side = -side;
                    }
                }
            }
            pts.push((x, y));
            ws.push(w);
            if x < -50.0 || x > 1050.0 || y < 30.0 {
                break;
            }
        }
        // the tip: a living twig ends in a small cluster of short shoots
        if !dead && !sawn && w < 1.0 && order > 1 && r.f() < 0.45 {
            for _ in 0..(1 + below(r, 2)) {
                let ca = a + r.range(-0.45, 0.45);
                let l = r.range(2.0, 5.5);
                let q = (x + ca.cos() * l, y + ca.sin() * l);
                self.limbs.push(Limb { pts: vec![(x, y), ((x + q.0) * 0.5 + r.range(-0.6, 0.6), (y + q.1) * 0.5), q], w: vec![0.5, 0.4, 0.3], dead: false, order: order + 1 });
            }
        }
        let me = Limb { pts, w: ws, dead, order };
        self.limbs.push(me);
        for (q, ca, wc) in kids {
            // side limbs of a dead bough are dead too; a few live limbs die
            let kd = dead || (wc > 2.0 && r.f() < 0.06);
            self.branch(r, q, ca, wc, order + 1, kd, reach.max(0.7), false);
        }
    }

    /// A diagnostic: the skeleton as a grayscale image, 1 px per unit (the
    /// painter's thumbnail sketch, not part of the painting).
    fn pgm(&self, path: &str, h: f32) {
        let (w, hh) = (1000usize, h as usize);
        let mut img = vec![255u8; w * hh];
        for l in &self.limbs {
            for i in 0..l.pts.len().saturating_sub(1) {
                let (a, b) = (l.pts[i], l.pts[i + 1]);
                let n = (dist(a, b) * 3.0).ceil().max(1.0) as usize;
                for k in 0..=n {
                    let t = k as f32 / n as f32;
                    let (x, y) = (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
                    let r = (l.w[i] * 0.5).max(0.3);
                    let v = if l.dead { 140u8 } else { 0 };
                    for yy in ((y - r).floor() as i32)..=((y + r).ceil() as i32) {
                        for xx in ((x - r).floor() as i32)..=((x + r).ceil() as i32) {
                            if xx >= 0 && yy >= 0 && (xx as usize) < w && (yy as usize) < hh {
                                let d = ((xx as f32 + 0.5 - x).powi(2) + (yy as f32 + 0.5 - y).powi(2)).sqrt();
                                let cov = (r + 0.5 - d).clamp(0.0, 1.0);
                                let px = &mut img[yy as usize * w + xx as usize];
                                let nv = (255.0 - (255.0 - v as f32) * cov) as u8;
                                *px = (*px).min(nv);
                            }
                        }
                    }
                }
            }
        }
        let mut out = format!("P5\n{w} {hh}\n255\n").into_bytes();
        out.extend(img);
        std::fs::write(path, out).unwrap();
    }

    fn index(&mut self) {
        for (li, l) in self.limbs.iter().enumerate() {
            for i in 0..l.pts.len().saturating_sub(1) {
                if l.w[i] < THICK {
                    break;
                }
                let s = (l.pts[i], l.pts[i + 1], l.w[i], l.w[i + 1], li);
                let si = self.segs.len() as u32;
                self.segs.push(s);
                let rad = l.w[i] * 0.5 + 2.0;
                let (x0, x1) = (s.0.0.min(s.1.0) - rad, s.0.0.max(s.1.0) + rad);
                let (y0, y1) = (s.0.1.min(s.1.1) - rad, s.0.1.max(s.1.1) + rad);
                for gy in ((y0 / CELL).floor().max(0.0) as usize)..=((y1 / CELL).floor().max(0.0) as usize).min(GH - 1) {
                    for gx in ((x0 / CELL).floor().max(0.0) as usize)..=((x1 / CELL).floor().max(0.0) as usize).min(GW - 1) {
                        self.grid[gy * GW + gx].push(si);
                    }
                }
            }
        }
    }

    /// The nearest thick segment: (index, signed distance to the surface
    /// (+ inside, units), across offset -1..1, along 0..1).
    fn nearest(&self, x: f32, y: f32) -> Option<(usize, f32, f32)> {
        if x < 0.0 || y < 0.0 {
            return None;
        }
        let (gx, gy) = ((x / CELL) as usize, (y / CELL) as usize);
        if gx >= GW || gy >= GH {
            return None;
        }
        let mut best: Option<(usize, f32, f32)> = None;
        for &si in &self.grid[gy * GW + gx] {
            let (a, b, wa, wb, _) = self.segs[si as usize];
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let l2 = (dx * dx + dy * dy).max(1e-6);
            let t = (((x - a.0) * dx + (y - a.1) * dy) / l2).clamp(0.0, 1.0);
            let (px, py) = (a.0 + dx * t, a.1 + dy * t);
            let half = 0.5 * (wa + (wb - wa) * t);
            let d = ((x - px).powi(2) + (y - py).powi(2)).sqrt();
            let inside = half - d;
            // which side: the cross product of the direction and the offset
            let cross = dx * (y - py) - dy * (x - px);
            let across = (d / half.max(1e-3)).min(1.5) * cross.signum();
            if best.is_none_or(|bb| inside > bb.1) {
                best = Some((si as usize, inside, across));
            }
        }
        best
    }

    /// The thick limbs as a mask: their edges a little uneven, as bark is.
    fn thick_mask(&self, f: paint::Frame) -> Mask {
        let n = Fbm::new(77, 3, 6.0);
        Mask::from_fn(f, move |x, y| match self.nearest(x, y) {
            // the foot goes into the snow
            Some((_, inside, _)) => smoothstep(-0.35, 0.35, inside + 0.5 * n.get(x, y)) * (1.0 - smoothstep(BASE.1 + 2.0, BASE.1 + 5.0, y - 5.0 * n.get(x * 0.7, 0.0))),
            None => 0.0,
        })
    }

    fn angle_at(&self, x: f32, y: f32) -> f32 {
        match self.nearest(x, y) {
            Some((si, _, _)) => {
                let (a, b, ..) = self.segs[si];
                (b.1 - a.1).atan2(b.0 - a.0)
            }
            None => -1.57,
        }
    }

    /// The light on a round limb: from the upper left, soft (a winter day).
    fn light_across(&self, dx: f32, dy: f32, across: f32) -> f32 {
        // the surface normal in the picture plane, tilted toward the viewer
        let (nx, ny) = (-dy * across, dx * across);
        let z = (1.0 - across * across).max(0.0).sqrt();
        let (lx, ly, lz) = (-0.62, -0.45, 0.64);
        (nx * lx + ny * ly + z * lz).max(0.0)
    }

    fn light_at_offset(&self, l: &Limb, i: usize, across: f32) -> f32 {
        let (dx, dy) = l.dir(i);
        self.light_across(dx, dy, across * 2.0)
    }

    /// Bark color at a point of a thick limb.
    fn bark(&self, x: f32, y: f32) -> Rgb {
        let Some((si, _, across)) = self.nearest(x, y) else { return hex("#3a342d") };
        let (a, b, _, _, li) = self.segs[si];
        let d = dist(a, b).max(1e-4);
        let lit = self.light_across((b.0 - a.0) / d, (b.1 - a.1) / d, across.clamp(-1.0, 1.0));
        let dead = self.limbs[li].dead;
        let n = self.bark_n.get(x * 0.5, y * 0.12);
        let (sh, mid, hi) = if dead { (hex("#46423c"), hex("#6a665d"), hex("#96918a")) } else { (hex("#201c18"), hex("#36302a"), hex("#5d584b")) };
        let base = if lit < 0.5 { mix(sh, mid, lit / 0.5, Mix::Pigment) } else { mix(mid, hi, (lit - 0.5) / 0.5, Mix::Pigment) };
        // lichen, gray-green, on the lit, weather side of the bole
        let lichen = if !dead { smoothstep(0.1, 0.5, n) * smoothstep(0.45, 0.8, lit) * 0.45 } else { 0.0 };
        mix(base, hex("#7d8166"), lichen, Mix::Pigment)
    }

    fn limb_color(&self, l: &Limb, i: usize, r: &mut Rng) -> Rgb {
        let w = l.w[i];
        if l.dead {
            return mix(hex("#57534b"), hex("#77736a"), r.f() * 0.5, Mix::Pigment);
        }
        // thick: dark; the finest: warmer and a little lighter, as twigs
        // against a light sky are
        let t = smoothstep(0.4, 3.0, w);
        mix(hex("#4c433b"), hex("#2a2521"), t, Mix::Pigment)
    }
}

fn below(r: &mut Rng, n: usize) -> usize {
    ((r.f() * n as f32) as usize).min(n - 1)
}

/// A brush from the painter's set of pointed rounds: the smallest one that
/// makes a mark `w` wide without being pressed flat.
fn round_for(w: f32) -> Tool {
    let sizes = [1.0f32, 1.4, 2.0, 2.8, 3.8, 5.0, 6.5];
    let s = sizes.iter().copied().find(|&s| s >= w * 1.15).unwrap_or(6.5);
    Tool { point: 1.0, ..Tool::round_sable(s) }
}

fn angle_diff(to: f32, from: f32) -> f32 {
    let mut d = to - from;
    while d > std::f32::consts::PI {
        d -= std::f32::consts::TAU;
    }
    while d < -std::f32::consts::PI {
        d += std::f32::consts::TAU;
    }
    d
}
