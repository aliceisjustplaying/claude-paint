//! Study: a summer meadow in daylight, a leafy oak and a hedge, in the
//! greens Friedrich could mix (notes/green.md). The engine grows the wood
//! and the leaves (`Habit::grow`, `Skeleton::foliage`) and seeds the grass
//! (`Sward::grow`); this study paints them with its own habits:
//!
//! - sky first, thin and fused, the far woods stippled into it cool and
//!   pale, the meadow laid in lean and dried;
//! - a leaf mass is built dark to light: the whole mass in a deep cool
//!   green in small leaf-sized touches, then the parts that face the sun in
//!   warmer mid greens, then the clumps that catch full light in a few
//!   crisp yellow-green touches; the sky left showing through the gaps;
//! - grass last, in fine upturning strokes laid over the dried meadow
//!   [NG p.56], far tufts first; flowers a touch each.
//!
//!   cargo paint study_green                         1000px
//!   cargo paint study_green -- --full --crop 560,160,900,520   the oak at 3200px
//!   cargo paint study_green -- --geom               the geometry only (diagnostic)

use paint::{Canvas, Fbm, Gesture, Habit, Held, Leafing, Mask, Mix, Orient, Palette, Rgb, Rng, Skeleton, Stipple, Style, Sward, Tool, Touch, Wind, gradient, hex, smoothstep};
use paintings::trees::{self, Bark, TreeHand};

/// Toward the sun (canvas axes: x right, y down, z toward the viewer): high
/// summer sun from the upper left, a little in front.
const SUN: (f32, f32, f32) = (-0.55, -0.75, 0.35);

fn main() {
    let o = paintings::run::Run::new("study_green");
    let mut rng = Rng::new(o.seed);
    let seed = o.seed;
    let st = Style { palette: Palette::friedrich_1820_greens(), ..Style::friedrich() };
    let pal = &st.palette;
    // the sky's own family of paints, set out apart from the greens
    let sky_pal = pal.only(&["lead white", "cobalt blue", "pale smalt", "yellow ochre", "red earth"]);
    let mut c = o.canvas(|| st.prepare(o.width, 1.4, seed));
    let (w, h) = (c.width(), c.height());
    let f = c.frame();

    // ---- the lay of the land
    let horizon = h * 0.6;
    let nf = Fbm::new(seed as u32 + 1, 5, 60.0);
    let nt = Fbm::new(seed as u32 + 3, 3, 9.0);
    let belts = Fbm::new(seed as u32 + 4, 3, 140.0);
    // far woods: belts of trees along the horizon, their crowns a row of
    // small bumps, with open fields between the belts
    let far_top = move |x: f32| {
        let belt = smoothstep(0.35, 0.55, belts.get01(x, 0.0));
        horizon - 1.5 - belt * (6.0 + 9.0 * nf.get01(x, 0.0) + 4.0 * nt.get01(x, 0.0)) - 8.0 * (-((x - 260.0) / 90.0).powi(2)).exp()
    };
    let ng = Fbm::new(seed as u32 + 2, 4, 200.0);
    let meadow_top = move |x: f32| horizon + 2.0 + 3.0 * ng.get(x, 0.0);
    let (far_top, meadow_top) = (f.per_column(far_top), f.per_column(meadow_top));
    let soft = |d: f32, e: f32| smoothstep(-e, e, d);
    let sky = Mask::from_fn(f, |x, y| 1.0 - soft(y - far_top(x), 0.5));
    let far = Mask::from_fn(f, |x, y| soft(y - far_top(x), 0.5) * (1.0 - soft(y - meadow_top(x), 0.5)));
    let meadow = Mask::from_fn(f, |x, y| soft(y - meadow_top(x), 0.5));

    // ---- what grows there (geometry: botany, not paint)
    let oak_base = (690.0, h * 0.8);
    let oak = Habit { lean: -0.05, ..Habit::oak() }.grow(oak_base, h * 0.5, seed * 7 + 23);
    let oak_leaves = oak.foliage(SUN, seed * 7 + 3);
    // the hedge: a row of young alders and beeches along a ditch, set close
    let hedge_y = h * 0.705;
    let mut hedge: Vec<Skeleton> = vec![];
    let mut x = -20.0;
    let mut k = 0u64;
    while x < 560.0 {
        let ht = rng.range(38.0, 62.0) * (1.0 - 0.25 * (x / 560.0));
        let hab = if k % 3 == 1 { Habit { years: 15, ..Habit::beech() } } else { Habit { years: 15, ..Habit::alder() } };
        hedge.push(hab.grow((x, hedge_y + rng.range(-2.0, 2.0)), ht, seed * 31 + k));
        x += rng.range(9.0, 16.0);
        k += 1;
    }
    // hedge leaves: clipped dense and a little bigger than a tree's
    let hedge_leaves: Vec<_> = hedge
        .iter()
        .enumerate()
        .map(|(i, sk)| sk.foliage_with(&Leafing { clump: sk.leaf.clump * 2.2, spacing: 1.3, inner: 0.15, spray: 0.5, bare: 0.02, fill: 0.95, ..sk.leaf }, SUN, seed * 41 + i as u64))
        .collect();
    let meadow_near = Mask::from_fn(f, |x, y| soft(y - meadow_top(x) - 4.0, 0.5));
    let sward = Sward {
        horizon,
        near: h,
        height: 26.0,
        spacing: 3.4,
        thin: 0.55,
        smallest: 0.7,
        flowers: 0.05,
        kinds: 3,
        wind: Wind { lean: 0.12, gust: 0.18, period: 140.0, seed: seed + 5 },
        ..Sward::default()
    };
    let tufts = sward.grow(&meadow_near, seed * 3 + 1);
    eprintln!("oak: {} limbs, {} clumps; hedge: {} shrubs, {} clumps; {} tufts", oak.limbs.len(), oak_leaves.clumps.len(), hedge.len(), hedge_leaves.iter().map(|l| l.clumps.len()).sum::<usize>(), tufts.len());

    if std::env::args().any(|a| a == "--geom") {
        geometry(&mut c, &oak, &oak_leaves, &hedge_leaves, &tufts);
        o.save(&mut c);
        return;
    }

    // ---- sky: summer daylight, pale blue high, warm pale at the horizon
    let sky_stops: [(f32, Rgb); 4] = [(0.0, hex("#7e95b6")), (0.35, hex("#a3b3c6")), (0.75, hex("#d6d8d0")), (0.9, hex("#e6e2d2"))];
    let sky_color = move |_: f32, y: f32| gradient(&sky_stops, y / horizon, Mix::Pigment);
    if o.stage("sky", &mut c, &mut rng) {
        c.work(&sky, &st.broad().mixed(&sky_pal, 0.25).color(sky_color).angle(|_, _| 0.03).load(0.6).coverage(4.5), seed * 100 + 1);
        // a few soft banks of summer cloud, stippled pale into the wet sky
        let nc = Fbm::new(seed as u32 + 9, 5, 160.0);
        let cloud = move |x: f32, y: f32| smoothstep(0.58, 0.8, nc.get01(x * 0.6, y * 1.8)) * smoothstep(0.05, 0.3, y / horizon) * (1.0 - smoothstep(0.6, 0.9, y / horizon));
        let sp = Stipple::new(Tool::stippler(5.0)).mixed(&sky_pal, 0.4).color(|_, y| gradient(&[(0.0, hex("#dfe0dc")), (1.0, hex("#ece6d6"))], y / horizon, Mix::Pigment)).coverage(move |x, y| 1.2 * cloud(x, y)).pressure(0.3, 0.7).feather(0.9);
        c.stipple(&sky, &sp, seed * 100 + 6);
        if let Some(b) = st.blend() {
            let b = b.angle(|_, _| 0.0);
            for k in 0..st.blend_passes {
                c.work(&sky, &b, seed * 100 + 2 + k as u64);
            }
        }
        c.dry();
    }

    // ---- the far woods and the distant meadow: cool, pale, stippled
    if o.stage("distance", &mut c, &mut rng) {
        let far_c = |x: f32, y: f32| {
            let t = ((y - (horizon - 25.0)) / 30.0).clamp(0.0, 1.0);
            gradient(&[(0.0, hex("#8d9aa0")), (1.0, hex("#7f8e7c"))], t + 0.1 * (x / w - 0.5), Mix::Pigment)
        };
        c.work(&far, &st.body().color(far_c).angle(|_, _| 0.0).length(10.0, 30.0).coverage(4.0).clip(true).threshold(0.2), seed * 100 + 10);
        let sp = Stipple::new(Tool::stippler(2.2)).mixed(pal, 0.4).color(move |x, y| {
            let base = far_c(x, y);
            [base[0] * 0.85, base[1] * 0.9, base[2] * 0.85]
        });
        c.stipple(&far, &sp.coverage(|_, _| 0.8).clip(true), seed * 100 + 11);
        c.dry();
    }

    // ---- the meadow laid in: lean paint, warmer and deeper toward us,
    // lush patches deeper green, sunnier patches yellower
    let nl = Fbm::new(seed as u32 + 12, 4, 110.0);
    let meadow_c = move |x: f32, y: f32| {
        let t = ((y - horizon) / (h - horizon)).clamp(0.0, 1.0);
        let base = gradient(&[(0.0, hex("#a4aa84")), (0.25, hex("#8e9a5c")), (0.6, hex("#6d7e3e")), (1.0, hex("#56662f"))], t, Mix::Pigment);
        let lush = nl.get(x, y * 2.5);
        gradient(&[(0.0, [base[0] * 0.8, base[1] * 0.86, base[2] * 0.8]), (0.5, base), (1.0, [base[0] * 1.12, base[1] * 1.06, base[2] * 0.9])], 0.5 + 0.8 * lush, Mix::Pigment)
    };
    if o.stage("meadow", &mut c, &mut rng) {
        c.work(&meadow, &st.body().color(&meadow_c).angle(|_, _| 0.0).angle_jitter(0.15).length(20.0, 60.0).coverage(4.0).medium(0.35), seed * 100 + 20);
        // the oak's shadow on the grass, falling away from the sun
        let (sx, sy) = (oak_base.0 + 70.0, oak_base.1 + 8.0);
        let shadow = Mask::from_fn(f, move |x, y| (1.0 - smoothstep(0.7, 1.0, ((x - sx) / 120.0).powi(2) + ((y - sy) / 16.0).powi(2))) * smoothstep(horizon, horizon + 10.0, y));
        c.dry();
        // and the shadows of clouds drifting over the meadow: long soft bands
        let ncs = Fbm::new(seed as u32 + 14, 3, 260.0);
        let clouds = move |x: f32, y: f32| smoothstep(0.52, 0.68, ncs.get01(x * 0.5, y * 3.0));
        c.work(&meadow, &st.glaze(0.85).color(|_, _| hex("#34452e")).angle(|_, _| 0.0).load_at(move |x, y| 2.2 * shadow.sample(x, y) + 0.9 * clouds(x, y)), seed * 100 + 21);
        c.dry();
    }

    // ---- the hedge: a few dark stems, then leaves dark to light
    if o.stage("hedge", &mut c, &mut rng) {
        let greens = Greens { shade: hex("#34422f"), mid: hex("#56673a"), light: hex("#8c9750"), sun: hex("#aeab68") };
        for (i, fo) in hedge_leaves.iter().enumerate() {
            leaves(&mut c, pal, fo, &greens, 0.9, seed * 100 + 40 + i as u64);
        }
        c.dry();
    }

    // ---- the oak: wood, then the leaf masses
    if o.stage("oak", &mut c, &mut rng) {
        let bark = Bark { dark: pal.mix(hex("#2a251d")).paint(0.3), dead: None, light: Some(pal.mix(hex("#6e6a58")).paint(0.35)), wood: Some(pal.mix(hex("#a39479")).paint(0.25)) };
        // the painter paints only the wood that shows: trunk and main limbs,
        // the twigs lost in leaves
        let wood = Skeleton { limbs: oak.limbs.iter().filter(|l| l.order <= 1 || l.w[0] > 2.0).cloned().collect(), ..oak.clone() };
        trees::tree(&mut c, &wood, &bark, &TreeHand { light_from: (SUN.0, SUN.1), finest: 0.4, ..Default::default() }, seed * 100 + 50);
        let greens = Greens { shade: hex("#2e3d2a"), mid: hex("#56692f"), light: hex("#93a14a"), sun: hex("#c4bf6c") };
        leaves(&mut c, pal, &oak_leaves, &greens, 1.0, seed * 100 + 51);
        c.dry();
    }

    // ---- grass: fine upturning strokes over the dried meadow, far first
    if o.stage("grass", &mut c, &mut rng) {
        grass(&mut c, pal, &tufts, &meadow_c, seed * 100 + 60);
        c.dry();
    }
    c.relief(0.5, 0.0);
    o.save(&mut c);
}

/// A leaf mass's greens (the look wanted on the canvas).
struct Greens {
    shade: Rgb,
    mid: Rgb,
    light: Rgb,
    sun: Rgb,
}

fn mixc(a: Rgb, b: Rgb, t: f32) -> Rgb {
    gradient(&[(0.0, a), (1.0, b)], t.clamp(0.0, 1.0), Mix::Pigment)
}

/// A leaf mass, dark to light, in leaf-sized touches. `detail` scales the
/// number of touches.
fn leaves(c: &mut Canvas, pal: &Palette, fo: &paint::Foliage, g: &Greens, detail: f32, seed: u64) {
    let f = c.frame();
    let mask = fo.mask(f);
    let mut rng = Rng::new(seed);
    let grain = fo.grain();
    let clumps = fo.back_to_front();
    // touches land on leaves, not on sky
    let pick = |rng: &mut Rng, cl: &paint::Clump, bias: (f32, f32), spread: f32| -> Option<(f32, f32)> {
        for _ in 0..6 {
            let (u, v) = (rng.normal() * spread + bias.0, rng.normal() * spread + bias.1);
            let (x, y) = (cl.at.0 + u * cl.r, cl.at.1 + v * cl.r * cl.squash);
            if mask.sample(x, y) > 0.5 {
                return Some((x, y));
            }
        }
        None
    };
    let sun = fo.sun;
    let lean = |m: f32| m;
    // 1. the whole mass, dark: shade green, a little lighter where lit
    let tip = (grain * 0.55).max(1.0);
    let mut dark = Held::new(Tool { ragged: 0.4, ..Tool::round_sable(tip) }, rng.next_u64());
    for cl in &clumps {
        let n = ((cl.r * cl.r * cl.squash / (tip * tip) * 1.4 * detail) as usize).max(2);
        for _ in 0..n {
            let Some((x, y)) = pick(&mut rng, cl, (0.0, 0.0), 0.5) else { continue };
            let want = mixc(g.shade, g.mid, cl.lit * 0.8 + rng.normal() * 0.08);
            dark.reload(c.aim(pal, want, (x, y), tip * 0.5, lean(0.3), 1.0), 0.6);
            let a = rng.range(0.0, std::f32::consts::TAU);
            let d = tip * rng.range(0.2, 0.6);
            c.touch(&mut dark, &Touch::at(x, y).pressure(rng.range(0.45, 0.85)).drag(d * a.cos(), d * a.sin()).twist(rng.normal() * 0.4), None);
        }
    }
    c.dry();
    // 2. the sunward side of each lit clump: mid greens
    let tip2 = tip * 0.8;
    let mut mid = Held::new(Tool { ragged: 0.35, ..Tool::round_sable(tip2) }, rng.next_u64());
    let toward = (sun.0 * 0.45, sun.1 * 0.45);
    for cl in clumps.iter().filter(|cl| cl.lit > 0.15) {
        let n = ((cl.r * cl.r * cl.squash / (tip2 * tip2) * 1.1 * cl.lit * detail) as usize).max(1);
        for _ in 0..n {
            let Some((x, y)) = pick(&mut rng, cl, toward, 0.4) else { continue };
            let want = mixc(g.mid, g.light, (cl.lit - 0.15) * 1.4 + rng.normal() * 0.1);
            mid.reload(c.aim(pal, want, (x, y), tip2 * 0.5, 0.25, 1.0), 0.6);
            let a = rng.range(0.0, std::f32::consts::TAU);
            let d = tip2 * rng.range(0.2, 0.5);
            c.touch(&mut mid, &Touch::at(x, y).pressure(rng.range(0.4, 0.8)).drag(d * a.cos(), d * a.sin()), None);
        }
    }
    // 3. full light: a few crisp touches where a clump catches the sun
    let tip3 = tip * 0.55;
    let mut hi = Held::new(Tool::round_sable(tip3), rng.next_u64());
    for cl in clumps.iter().filter(|cl| cl.lit > 0.45) {
        let n = ((cl.r * cl.r / (tip3 * tip3) * 0.35 * (cl.lit - 0.4) * detail) as usize).max(1);
        for _ in 0..n {
            let Some((x, y)) = pick(&mut rng, cl, (sun.0 * 0.6, sun.1 * 0.6), 0.3) else { continue };
            let want = mixc(g.light, g.sun, (cl.lit - 0.45) * 2.0 + rng.normal() * 0.1);
            hi.reload(c.aim(pal, want, (x, y), tip3 * 0.5, 0.15, 1.0), 0.6);
            let a = rng.range(0.0, std::f32::consts::TAU);
            let d = tip3 * rng.range(0.1, 0.4);
            c.touch(&mut hi, &Touch::at(x, y).pressure(rng.range(0.35, 0.7)).drag(d * a.cos(), d * a.sin()), None);
        }
    }
}

/// Grass: every tuft's blades as fine strokes pulled up from the foot and
/// lifted off, far tufts first; a flower is a touch on the tallest blade.
fn grass(c: &mut Canvas, pal: &Palette, tufts: &[paint::Tuft], ground: &dyn Fn(f32, f32) -> Rgb, seed: u64) {
    let mut rng = Rng::new(seed);
    let mut held = Held::new(Tool::rigger(0.6), rng.next_u64());
    let mut flower = Held::new(Tool::round_sable(1.0), rng.next_u64());
    let sheen = Fbm::new(seed as u32 ^ 0x5ee, 3, 70.0);
    for t in tufts {
        let g = ground(t.at.0, t.at.1);
        // each tuft a shade off the ground: deeper in lush grass, lighter
        // and yellower where the wind turns the blades to the sun (in
        // patches, as gusts go over)
        let lit = (0.35 + 0.5 * sheen.get(t.at.0, t.at.1 * 3.0) + 0.2 * rng.normal() - 0.25 * t.lush).clamp(0.0, 1.0);
        let want = if lit < 0.55 {
            mixc([g[0] * 0.6, g[1] * 0.68, g[2] * 0.6], g, lit / 0.55)
        } else {
            mixc(g, mixc(g, hex("#b9b36e"), 0.6), (lit - 0.55) / 0.45)
        };
        let bw = (t.height * 0.07).clamp(0.3, 1.4);
        held.tool = Tool { length: bw * 5.0, ..Tool::rigger(bw) };
        let p = c.aim(pal, want, t.at, t.height * 0.3, 0.3, 1.0);
        for b in &t.blades {
            held.reload(p, 0.7);
            let pts: Vec<(f32, f32)> = (0..=4).map(|k| {
                let s = k as f32 / 4.0;
                let (a, m, e) = (b[0], b[1], b[2]);
                let q = |i: usize| -> f32 { let (a, m, e) = ([a.0, a.1][i], [m.0, m.1][i], [e.0, e.1][i]); (1.0 - s) * (1.0 - s) * a + 2.0 * s * (1.0 - s) * m + s * s * e };
                (q(0), q(1))
            }).collect();
            c.drag(&mut held, &Gesture::new(pts).pressure(0.8, 0.1).ramps(0.05, 0.6).orient(Orient::Across), None);
        }
        if let Some(fl) = t.flower {
            let col = match fl.kind { 0 => hex("#ece8d8"), 1 => hex("#e0bb2a"), _ => hex("#c0452c") };
            flower.tool = Tool::round_sable((fl.r * 2.0).max(0.5));
            flower.reload(pal.mix(col).paint(0.1), 0.8);
            c.touch(&mut flower, &Touch::at(fl.at.0, fl.at.1).pressure(0.6), None);
        }
    }
}

/// Diagnostic only: the geometry as flat tones (not a painting).
fn geometry(c: &mut Canvas, oak: &Skeleton, fo: &paint::Foliage, hedge: &[paint::Foliage], tufts: &[paint::Tuft]) {
    let f = c.frame();
    c.apply(|_, _, _| [0.8, 0.82, 0.85]);
    let wood = oak.mask(f);
    c.apply_masked(&wood, |_, _, _, _| [0.1, 0.08, 0.06]);
    for h in hedge {
        let lit = h.lit(f);
        let m = h.mask(f);
        c.apply_masked(&m, |x, y, _, _| { let l = lit.sample(x, y); [0.05 + 0.4 * l, 0.12 + 0.5 * l, 0.05 + 0.1 * l] });
    }
    let lit = fo.lit(f);
    let m = fo.mask(f);
    c.apply_masked(&m, |x, y, _, _| { let l = lit.sample(x, y); [0.05 + 0.5 * l, 0.12 + 0.55 * l, 0.05 + 0.12 * l] });
    let mut pts = Mask::empty(f);
    for t in tufts {
        for b in &t.blades {
            for k in 0..=8 {
                let s = k as f32 / 8.0;
                let x = (1.0 - s) * (1.0 - s) * b[0].0 + 2.0 * s * (1.0 - s) * b[1].0 + s * s * b[2].0;
                let y = (1.0 - s) * (1.0 - s) * b[0].1 + 2.0 * s * (1.0 - s) * b[1].1 + s * s * b[2].1;
                if f.holds(x, y) {
                    let i = f.index(x, y);
                    pts.data[i] = 1.0;
                }
            }
        }
    }
    c.apply_masked(&pts, |_, _, _, _| [0.15, 0.3, 0.1]);
}
