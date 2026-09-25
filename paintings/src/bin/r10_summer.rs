//! "Summer Afternoon: the Lime Tree on the Rise" (r10_summer). An original
//! painting in the manner of Caspar David Friedrich, not a copy: made from
//! what is known of his materials, method and motifs, with no reference
//! image. See notes/r10_summer.md.
//!
//! - a flat, wide land of meadows under a high luminous sky (the Greifswald
//!   meadows, the plain near Dresden), the horizon low, the picture built in
//!   level bands;
//! - one lime tree, whole, on a low rise left of center: the tree as a
//!   figure; under it two people seen from behind (Rückenfiguren) looking
//!   over the meadows to a far town whose church towers rise from the
//!   horizon;
//! - pollard willows along a ditch and haycocks on a mown strip: the
//!   particular, small, exact things of a summer day;
//! - order of work: warm ground bought primed, thin underpainting for
//!   values, sky laid and fused, clouds stippled, distance stippled cool,
//!   meadows lean and dried, trees after the sky, figures, grass last in
//!   fine upturning strokes.

use paint::broadleaf::{Season, Species, Tree};
use paint::{Canvas, Fbm, Gesture, Held, Mask, Mix, Orient, Palette, Rgb, Rng, Stipple, Style, Sward, Tool, Touch, Wind, gradient, hex, smoothstep};
use paintings::run::Finish;

/// Toward the sun: afternoon, high, from the left and a little in front.
const SUN: [f32; 3] = [-0.62, -0.62, 0.45];

fn mixc(a: Rgb, b: Rgb, t: f32) -> Rgb {
    gradient(&[(0.0, a), (1.0, b)], t.clamp(0.0, 1.0), Mix::Pigment)
}

fn main() {
    let o = paintings::run::Run::new("r10_summer");
    let mut rng = Rng::new(o.seed);
    let seed = o.seed;
    let st = Style { palette: Palette::friedrich_1820_greens(), ..Style::friedrich() };
    let pal = &st.palette;
    let sky_pal = pal.only(&["lead white", "cobalt blue", "pale smalt", "yellow ochre", "red earth"]);
    let mut c = o.canvas(|| st.prepare(o.width, 1.42, seed));
    let (w, h) = (c.width(), c.height());
    let f = c.frame();
    let soft = |d: f32, e: f32| smoothstep(-e, e, d);

    // ---- the lay of the land (units, y down)
    let horizon = h * 0.635;
    let s32 = seed as u32;
    let nb = Fbm::new(s32 + 1, 3, 150.0);
    let nt = Fbm::new(s32 + 2, 4, 7.0);
    // far woods on the horizon: low belts of trees with open land between
    let far_top = move |x: f32| {
        let belt = smoothstep(0.38, 0.6, nb.get01(x, 0.0));
        horizon - 0.8 - belt * (4.0 + 5.0 * nt.get01(x, 0.0))
    };
    // the rise the lime stands on, and the foreground bank
    let nr = Fbm::new(s32 + 3, 4, 90.0);
    let rise = move |x: f32| h * 0.8 - 34.0 * (-((x - 300.0) / 260.0).powi(2)).exp() + 3.0 * nr.get(x, 0.0) + 0.012 * (x - 500.0);
    let (far_top, rise) = (f.per_column(far_top), f.per_column(rise));
    let sky = Mask::from_fn(f, |x, y| 1.0 - soft(y - far_top(x), 0.4));
    let far = Mask::from_fn(f, |x, y| soft(y - far_top(x), 0.4) * (1.0 - soft(y - horizon - 1.2, 0.4)));
    let plain = Mask::from_fn(f, |x, y| soft(y - horizon - 1.2, 0.4) * (1.0 - soft(y - rise(x), 0.8)));
    let bank = Mask::from_fn(f, |x, y| soft(y - rise(x), 0.8));
    let land = Mask::from_fn(f, |x, y| soft(y - far_top(x), 0.4));
    // depth across the plain: 0 at the horizon, 1 at the rise
    let depth = move |x: f32, y: f32| ((y - horizon) / (rise(x) - horizon).max(1.0)).clamp(0.0, 1.0);

    // ---- underpainting: thin warm brown for the values, the land darker
    if o.stage("ground", &mut c, &mut rng) {
        let whole = Mask::from_fn(f, |_, _| 1.0);
        let under = move |x: f32, y: f32| 0.2 + 0.9 * smoothstep(horizon - 5.0, horizon + 30.0, y) + 1.2 * smoothstep(rise(x) - 10.0, rise(x) + 60.0, y);
        c.work(&whole, &st.glaze(0.9).color(|_, _| hex("#7a5a3c")).angle(|_, _| 0.0).angle_jitter(0.04).load_at(under), seed * 100 + 90);
        if let Some(b) = st.blend() {
            c.work(&whole, &b.angle(|_, _| 0.0), seed * 100 + 91);
        }
        c.dry();
    }

    // ---- sky: a high summer afternoon, cool clear blue above, warm and pale
    // toward the horizon, a little warmer on the sun's side (left)
    let sky_stops: [(f32, Rgb); 5] = [(0.0, hex("#6f8cb4")), (0.3, hex("#8fa6c2")), (0.6, hex("#b9c4cc")), (0.85, hex("#dcd9c8")), (1.0, hex("#e8dfc4"))];
    let drift = Fbm::new(s32 + 10, 4, 400.0);
    let sky_color = move |x: f32, y: f32| {
        let t = (y / horizon + 0.02 * drift.get(x * 0.3, y) - 0.05 * (1.0 - x / w) * (y / horizon)).clamp(0.0, 1.0);
        gradient(&sky_stops, t, Mix::Pigment)
    };
    // summer clouds: soft level streaks of fair-weather cloud lying one
    // above another, thicker and brighter high up, thinner and closer
    // together toward the horizon (perspective); a little heaped on top
    let nc = Fbm::new(s32 + 11, 5, 60.0);
    let nh = Fbm::new(s32 + 12, 4, 140.0);
    // (center y, x from, x to, thickness, seed offset)
    let banks: [(f32, f32, f32, f32, f32); 6] = [
        (150.0, 380.0, 1080.0, 15.0, 0.0),
        (212.0, -80.0, 470.0, 11.0, 7.0),
        (262.0, 560.0, 1040.0, 9.0, 13.0),
        (318.0, 150.0, 640.0, 7.0, 21.0),
        (360.0, 640.0, 1060.0, 5.0, 29.0),
        (398.0, -40.0, 380.0, 4.0, 37.0),
    ];
    let cloud = move |x: f32, y: f32| {
        let mut m = 0.0f32;
        for &(cy, a, b, th, so) in &banks {
            if (y - cy).abs() > th * 4.0 {
                continue;
            }
            let span = smoothstep(a, a + 160.0, x) * (1.0 - smoothstep(b - 160.0, b, x));
            let thick = th * (0.3 + 1.0 * nh.get01(x, so * 10.0)) * span;
            let off = 0.6 * th * nh.get(x * 0.5, so * 10.0 + 5.0);
            // heaped above, flatter below
            let d = y - cy - off + 0.35 * th * nc.get(x * 0.8, y + so * 17.0);
            let k = if d < 0.0 { d / (thick * 1.3 + 0.01) } else { d / (thick * 0.6 + 0.01) };
            let body = (-k * k).exp() * smoothstep(0.3, 0.55, nc.get01(x * 0.35, so * 7.0 + y * 0.1));
            m = m.max(body);
        }
        m
    };
    // lit on top, a greyer shade underneath
    let cloud_lit = move |x: f32, y: f32| {
        let a = cloud(x, y - 4.0);
        let b = cloud(x, y + 4.0);
        (0.55 + 1.2 * (b - a)).clamp(0.0, 1.0)
    };
    if o.stage("sky", &mut c, &mut rng) {
        c.work(&sky, &st.broad().mixed(&sky_pal, 0.28).color(sky_color).angle(move |x, y| 0.02 * drift.get(x, y * 3.0)).load(0.62).coverage(4.5), seed * 100 + 1);
        if let Some(b) = st.blend() {
            let b = b.angle(|_, _| 0.0);
            for k in 0..st.blend_passes {
                c.work(&sky, &b, seed * 100 + 12 + k as u64);
            }
        }
        c.dry();
        // the clouds on the dry sky: bellies first, grey-violet, then the
        // lit tops warm white, stippled, and fused a little where they lie
        // the clouds laid wet into the wet sky: bellies first, grey-violet,
        // then the lit tops warm white, stippled
        let belly = Stipple::new(Tool::stippler(3.4)).mixed(&sky_pal, 0.35).color(move |_, y| mixc(hex("#9d9fae"), hex("#c2bab0"), y / horizon)).coverage(move |x, y| 1.4 * cloud(x, y) * (1.0 - cloud_lit(x, y)).powf(0.7)).pressure(0.4, 0.75).feather(0.8);
        c.stipple(&sky, &belly, seed * 100 + 5);
        let tops = Stipple::new(Tool::stippler(3.0)).mixed(&sky_pal, 0.3).color(|_, _| hex("#f6f0e2")).coverage(move |x, y| 2.0 * cloud(x, y) * cloud_lit(x, y)).pressure(0.4, 0.8).feather(0.8);
        c.stipple(&sky, &tops, seed * 100 + 6);
        if let Some(b) = st.blend() {
            let cm = Mask::from_fn(f, move |x, y| smoothstep(0.02, 0.2, cloud(x, y) + cloud(x, y - 6.0)));
            let b = b.angle(|_, _| 0.0).pressure(0.3, 0.5);
            c.work(&cm, &b, seed * 100 + 2);
        }
        c.dry();
        // the lit crests again, dry, so the heaps keep their edges
        let crest = Stipple::new(Tool::stippler(2.2)).mixed(&sky_pal, 0.3).color(|_, _| hex("#f7f1e3")).coverage(move |x, y| 0.9 * (cloud(x, y) * cloud_lit(x, y) - 0.3).max(0.0)).pressure(0.3, 0.6);
        c.stipple(&sky, &crest, seed * 100 + 8);
        c.dry();
        // a second, finer stipple on the sky: the air's grain, lighter
        // toward the horizon
        let fine = Stipple::new(Tool::stippler(1.8)).mixed(&sky_pal, 0.5).color(move |x, y| {
            let s = sky_color(x, y);
            let k = cloud(x, y);
            mixc(s, hex("#f3eee2"), 0.12 + 0.7 * k * cloud_lit(x, y))
        }).coverage(move |_, y| 0.35 + 0.5 * (y / horizon)).pressure(0.3, 0.6);
        c.stipple(&sky, &fine, seed * 100 + 7);
        c.dry();
    }

    // ---- the distance: far woods cool blue-green, the town, a windmill
    let town_x = 705.0;
    if o.stage("distance", &mut c, &mut rng) {
        let far_c = move |x: f32, _y: f32| mixc(hex("#8a98a8"), hex("#7f9098"), nb.get01(x * 2.0, 1.0));
        c.work(&far, &st.body().mixed(pal, 0.35).color(far_c).angle(|_, _| 0.0).length(8.0, 25.0).coverage(4.0).clip(true).threshold(0.2), seed * 100 + 10);
        let sp = Stipple::new(Tool::stippler(1.6)).mixed(pal, 0.45).color(move |x, y| mixc(far_c(x, y), hex("#6f8088"), 0.4)).coverage(|_, _| 0.6).clip(true);
        c.stipple(&far, &sp, seed * 100 + 11);
        c.dry();
        town(&mut c, pal, town_x, horizon, &mut rng);
        c.dry();
    }

    // ---- the plain: meadows in level bands, mown strips paler and warmer,
    // cooler and bluer toward the horizon, deeper toward us
    let nm = Fbm::new(s32 + 20, 4, 120.0);
    let strips = Fbm::new(s32 + 21, 2, 300.0);
    // mown strips: a function of depth in perspective (bands that widen
    // toward us), skewed a little so they don't run dead level
    let mown = move |x: f32, y: f32| {
        let d = depth(x, y);
        let z = 1.0 / (0.08 + d); // distance-like
        let band = (z * 2.6 + 0.6 * strips.get(x * 0.4, 0.0) + 0.0015 * x).sin();
        // far off the strips run together into one tone
        let m = smoothstep(0.15, 0.6, band);
        let fade = smoothstep(0.03, 0.2, d);
        m * fade + 0.35 * (1.0 - fade)
    };
    let plain_c = move |x: f32, y: f32| {
        let d = depth(x, y);
        let green = gradient(&[(0.0, hex("#98a497")), (0.12, hex("#8a9677")), (0.45, hex("#6f7c45")), (1.0, hex("#5e6a35"))], d, Mix::Pigment);
        let hay = gradient(&[(0.0, hex("#aeab93")), (0.12, hex("#b0ab80")), (0.5, hex("#aaa066")), (1.0, hex("#a0935a"))], d, Mix::Pigment);
        let base = mixc(green, hay, mown(x, y) * 0.8);
        let k = 1.0 + 0.06 * nm.get(x, y * 3.0);
        [base[0] * k, base[1] * k, base[2] * k]
    };
    if o.stage("plain", &mut c, &mut rng) {
        c.work(&plain, &st.body().mixed(pal, 0.3).color(plain_c).angle(|_, _| 0.0).angle_jitter(0.05).length(15.0, 60.0).coverage(4.0).clip(true), seed * 100 + 20);
        c.dry();
        // the far meadow stippled into the air: cool, the strokes lost
        let farplain = Mask::from_fn(f, move |x, y| plain.sample(x, y) * (1.0 - smoothstep(0.1, 0.3, depth(x, y))));
        let sp = Stipple::new(Tool::stippler(1.4)).mixed(pal, 0.45).color(move |x, y| mixc(plain_c(x, y), hex("#a9b3aa"), 0.3)).coverage(|_, _| 0.6);
        c.stipple(&farplain, &sp, seed * 100 + 21);
        c.dry();
    }

    // ---- the ditch with pollard willows, running from the left into the
    // plain, and haycocks on the nearest mown strip
    let ditch = |t: f32| -> (f32, f32) {
        // t 0 near (left), 1 far
        let x = 40.0 + 560.0 * t.powf(0.8);
        let y = horizon + 10.0 + 70.0 * (1.0 - t).powf(1.8);
        (x, y)
    };
    let summer = Greens { shade: hex("#1f2a1c"), mid: hex("#3a4724"), light: hex("#667433"), sun: hex("#a4a258") };
    if o.stage("middle", &mut c, &mut rng) {
        // a copse on the plain to the right, far, and a hedgerow of bushes
        // with a few trees along a field edge in the middle distance
        let copse_air = 0.72;
        let g = summer.far(copse_air);
        let mut spots: Vec<(f32, f32, f32, bool)> = vec![];
        let mut x: f32 = 830.0;
        while x < 1010.0 {
            let ht = rng.range(22.0, 38.0) * (1.0 - 0.3 * ((x - 900.0) / 110.0).abs());
            spots.push((x, horizon + 3.0 + rng.range(-1.0, 1.0), ht, rng.chance(0.3)));
            x += rng.range(9.0, 18.0);
        }
        // a lone field tree left of the town
        spots.push((560.0, horizon + 4.0, 26.0, false));
        for (i, &(x, y, ht, lime)) in spots.iter().enumerate() {
            let (rw, rh) = if lime { (ht * 0.3, ht * 0.45) } else { (ht * 0.38, ht * 0.4) };
            let _ = i;
            small_tree(&mut c, pal, (x, y - ht + rh), rw, rh, (x, y), &g, &mut rng);
        }
        c.dry();
        let hy = |x: f32| horizon + 24.0 + 0.012 * (x - 700.0);
        let g2 = summer.far(0.5);
        let mut x = 470.0;
        let mut k = 0;
        while x < 1010.0 {
            let big = rng.chance(0.12);
            let ht = if big { rng.range(22.0, 30.0) } else { rng.range(6.0, 10.0) };
            let (rw, rh) = if big { (ht * 0.36, ht * 0.4) } else { (ht * 0.9, ht * 0.5) };
            let y = hy(x);
            small_tree(&mut c, pal, (x, y - rh * 0.9), rw, rh, (x, y), &g2, &mut rng);
            let _ = k;
            x += if big { rng.range(10.0, 16.0) } else { rng.range(8.0, 13.0) };
            k += 1;
        }
        c.dry();
        // the ditch itself: a thin line of dark water glinting with sky
        let mut dw = Held::new(Tool { point: 0.8, ..Tool::round_sable(1.2) }, 301);
        let pts: Vec<(f32, f32)> = (0..=24).map(|k| ditch(k as f32 / 24.0)).map(|(x, y)| (x, y + 2.0)).collect();
        dw.load(pal.paint(hex("#4f5a4c"), 0.3), 0.8);
        c.drag(&mut dw, &Gesture::new(pts.clone()).pressure(0.9, 0.2).ramps(0.05, 0.6).orient(Orient::Across), None);
        let mut gl = Held::new(Tool { point: 0.9, ..Tool::round_sable(0.7) }, 302);
        gl.load(pal.paint(hex("#b8c4cc"), 0.2), 0.8);
        let pts2: Vec<(f32, f32)> = pts.iter().take(16).map(|&(x, y)| (x, y - 0.3)).collect();
        c.drag(&mut gl, &Gesture::new(pts2).pressure(0.6, 0.1).ramps(0.1, 0.5).orient(Orient::Across), None);
        c.dry();
        // haycocks
        let mut cocks = vec![];
        for i in 0..9 {
            let x = 470.0 + i as f32 * 55.0 + rng.range(-10.0, 10.0);
            let y = horizon + 38.0 + 0.02 * (x - 600.0) + rng.range(-2.0, 2.0);
            cocks.push((x, y, 7.5 + rng.range(-1.0, 1.0)));
        }
        for i in 0..7 {
            let x = 520.0 + i as f32 * 40.0 + rng.range(-6.0, 6.0);
            let y = horizon + 30.0 + 0.012 * (x - 700.0) + rng.range(-1.0, 1.0);
            cocks.push((x, y, 4.2 + rng.range(-0.6, 0.6)));
        }
        cocks.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        for (x, y, s) in cocks {
            haycock(&mut c, pal, (x, y), s, &mut rng);
        }
        // willows: far to near, smaller with distance
        let n = 9;
        for k in (0..n).rev() {
            let t = (k as f32 + 0.3 + rng.range(-0.15, 0.15)) / n as f32;
            let (x, y) = ditch(t);
            let s = 0.25 + 0.75 * (1.0 - t).powf(1.6);
            willow(&mut c, pal, (x + rng.range(-3.0, 3.0), y), 70.0 * s, seed * 50 + k as u64, &mut rng, 1.0 - t);
        }
        c.dry();
    }

    // ---- the bank: the rise and the foreground, laid lean and dried
    let nbk = Fbm::new(s32 + 30, 4, 60.0);
    let bank_c = move |x: f32, y: f32| {
        let t = ((y - rise(x)) / (h - rise(x)).max(1.0)).clamp(0.0, 1.0);
        let base = gradient(&[(0.0, hex("#5f6634")), (0.35, hex("#4b5229")), (1.0, hex("#343a1f"))], t, Mix::Pigment);
        let k = 1.0 + 0.1 * nbk.get(x, y * 2.0);
        [base[0] * k, base[1] * k * 1.01, base[2] * k]
    };
    // the lime and where it stands
    let lime_foot = (300.0, rise(300.0) + 2.0);
    let crown: Vec<(f32, f32)> = {
        let (cx, top, bot, rw) = (300.0, 58.0, 440.0, 190.0);
        let nn = Fbm::new(s32 + 40, 3, 1.3);
        (0..32).map(|i| {
            let a = i as f32 / 32.0 * std::f32::consts::TAU;
            let cy = (top + bot) * 0.5;
            let ry = (bot - top) * 0.5;
            // a lime's crown: a tall dome, a little pointed at the top,
            // broadest below the middle; the lowest boughs hang at the
            // sides and the bottom lifts in the middle over the trunk
            let (ca, sa) = (a.cos(), a.sin());
            let r = 1.0 + 0.07 * nn.get(ca * 2.0, sa * 2.0);
            let sx = ca * rw * r * (1.0 + 0.12 * sa.max(0.0)) * (1.0 - 0.22 * (-sa).max(0.0).powi(3));
            let lift = if sa > 0.0 { 0.9 - 0.2 * (1.0 - ca.abs()).powi(2) } else { 1.0 };
            let sy = sa * ry * r * lift;
            (cx + sx, cy + sy)
        }).collect()
    };
    let trunk_line = vec![lime_foot, (298.0, lime_foot.1 - 60.0), (296.0, lime_foot.1 - 140.0)];
    let lime = Tree::grow(&crown, Some(&trunk_line), &Species { voids: 0.12, ..Species::lime() }, &Season::named("summer").unwrap(), SUN, seed * 13 + 5);
    eprintln!("lime: {} limbs, {} clumps, {} touches, grain {:.2} touch_w {:.2}", lime.limbs.len(), lime.clumps.len(), lime.touches.len(), lime.grain, lime.touch_w);
    // the lime's shadow on the grass: thrown to the right and toward us
    let shadow = {
        // the sun is behind us and to the left: the shadow falls away from
        // us and to the right, a long flattened pool on the rise and the
        // meadow behind it
        let (sx, sy) = (lime_foot.0 + 150.0, lime_foot.1 - 4.0);
        Mask::from_fn(f, move |x, y| {
            let nx = (x - sx) / 175.0;
            let ny = (y - sy - 0.03 * (x - sx)) / 13.0;
            let n = 0.5 * nbk.get(x * 1.5, y * 3.0);
            (1.0 - smoothstep(0.3, 1.1, nx * nx + ny * ny + n)) * soft(y - horizon - 8.0, 2.0)
        })
    };
    // a footpath worn through the grass, from the bottom edge up the rise
    // to where the two stand: the centerline in perspective, narrowing
    let path_x = move |y: f32| {
        let t = ((y - rise(410.0)) / (h - rise(410.0))).clamp(0.0, 1.0);
        410.0 + 230.0 * t.powf(1.3) + 30.0 * (t * 5.0).sin() * t
    };
    let np = Fbm::new(s32 + 31, 4, 12.0);
    let path = Mask::from_fn(f, move |x, y| {
        let t = ((y - rise(410.0)) / (h - rise(410.0))).clamp(0.0, 1.0);
        let half = 1.5 + 17.0 * t.powf(1.2);
        let d = (x - path_x(y)).abs() - half * (1.0 + 0.25 * np.get(x, y));
        (1.0 - smoothstep(-1.0, 1.5, d)) * soft(y - rise(x) - 1.0, 1.0)
    });
    if o.stage("bank", &mut c, &mut rng) {
        c.work(&bank, &st.body().mixed(pal, 0.3).color(bank_c).angle(move |x, _| 0.6 * ((rise(x + 3.0) - rise(x - 3.0)) / 6.0).atan()).angle_jitter(0.12).length(12.0, 45.0).coverage(4.0).clip(true), seed * 100 + 30);
        c.dry();
        // the path: sandy earth, worn pale in the middle, laid along it
        let path_c = move |x: f32, y: f32| mixc(hex("#636040"), hex("#8a8260"), 0.5 + 0.5 * np.get(x * 0.5, y) - 0.4 * ((x - path_x(y)).abs() / 30.0).min(1.0));
        c.work(&path, &st.body().mixed(pal, 0.3).color(path_c).angle(move |_, y| ((path_x(y + 3.0) - path_x(y - 3.0)) / 6.0).atan() * -1.0 + std::f32::consts::FRAC_PI_2).angle_jitter(0.1).length(6.0, 20.0).coverage(3.5).clip(true).threshold(0.3), seed * 100 + 32);
        c.dry();
        // the shadow stippled in, its density following the shade, so its
        // edge is a thinning of touches, not a stroke's end
        let shade_sp = Stipple::new(Tool::stippler(2.6)).mixed(pal, 0.35).color(move |x, y| {
            let g = if y > rise(x) { bank_c(x, y) } else { plain_c(x, y) };
            [g[0] * 0.62, g[1] * 0.68, g[2] * 0.66]
        }).coverage(|x, y| 1.6 * shadow.sample(x, y)).pressure(0.35, 0.75);
        c.stipple(&land, &shade_sp, seed * 100 + 31);
        c.dry();
    }

    // ---- the lime: wood where it shows, then the leaves dark to light
    if o.stage("lime", &mut c, &mut rng) {
        paint_lime(&mut c, &st, pal, &lime, &summer, seed * 100 + 40);
        c.dry();
    }

    // ---- the two figures under the lime's edge, seen from behind
    if o.stage("figures", &mut c, &mut rng) {
        let fx = 405.0;
        couple(&mut c, pal, (fx, rise(fx) + 1.0), 40.0, seed * 100 + 50);
        c.dry();
    }

    // ---- grass: fine upturning strokes, far tufts first; flowers
    if o.stage("grass", &mut c, &mut rng) {
        let meadow_near = Mask::from_fn(f, |x, y| soft(y - rise(x) - 1.0, 0.8) * (1.0 - 0.6 * path.sample(x, y)));
        let sward = Sward {
            horizon: rise(300.0) - 60.0,
            near: h,
            height: 24.0,
            spacing: 4.6,
            thin: 0.5,
            patch: 0.75,
            patch_size: 70.0,
            smallest: 0.8,
            flowers: 0.035,
            kinds: 3,
            wind: Wind { lean: 0.1, gust: 0.15, period: 150.0, seed: seed + 5 },
            ..Sward::default()
        };
        let tufts = sward.grow(&meadow_near, seed * 3 + 1);
        eprintln!("{} tufts", tufts.len());
        grass(&mut c, pal, &tufts, &bank_c, &shadow, seed * 100 + 60);
        c.dry();
    }

    // ---- the particular plants of the foreground: tall grasses gone to
    // seed, dock, yarrow and a thistle by the path, each its own
    if o.stage("plants", &mut c, &mut rng) {
        let spots = [(60.0f32, 0.0f32), (180.0, 1.0), (520.0, 2.0), (700.0, 0.0), (760.0, 3.0), (880.0, 1.0), (965.0, 0.0), (610.0, 0.0), (330.0, 2.0)];
        for &(x, kind) in &spots {
            let y = h - rng.range(8.0, 70.0);
            let g = bank_c(x, y);
            match kind as u32 {
                0 => tall_grass(&mut c, pal, (x, y), rng.range(45.0, 75.0), g, &mut rng),
                1 => dock(&mut c, pal, (x, y), rng.range(16.0, 24.0), g, &mut rng),
                2 => yarrow(&mut c, pal, (x, y), rng.range(28.0, 42.0), g, &mut rng),
                _ => thistle(&mut c, pal, (x, y), rng.range(30.0, 40.0), g, &mut rng),
            }
        }
        c.dry();
    }
    let _ = lime_foot;
    o.finish(&mut c, &mut rng, &Finish::aged(st.relief));
}

/// The town far off: a low bank of roofs and three church towers with
/// tall spires, a windmill apart; blue-grey in the air.
fn town(c: &mut Canvas, pal: &Palette, x0: f32, hz: f32, rng: &mut Rng) {
    let col = hex("#7b8594");
    let dark = hex("#6d7684");
    // roofs: short level strokes with a few gables
    let mut r = Held::new(Tool::round_sable(1.1), rng.next_u64());
    let mut x = x0 - 72.0;
    while x < x0 + 80.0 {
        let wdt = rng.range(4.0, 9.0);
        let top = hz - 1.5 - rng.range(0.5, 3.0);
        let gable = rng.chance(0.35);
        let tone = mixc(col, dark, rng.range(0.0, 1.0));
        r.reload(pal.paint(tone, 0.2).with_hiding(0.85), 0.7);
        let mut u = x;
        while u < x + wdt {
            let peak = if gable { 1.8 * (1.0 - ((u - x) / wdt * 2.0 - 1.0).abs()) } else { 0.0 };
            c.drag(&mut r, &Gesture::new(vec![(u, hz + 0.5), (u, top - peak)]).pressure(0.8, 0.8).ramps(0.0, 0.05).orient(Orient::Across), None);
            u += 0.8;
        }
        x += wdt + if rng.chance(0.3) { rng.range(2.0, 6.0) } else { 0.0 };
    }
    // three churches: (x, tower height, spire height, tower width)
    for &(dx, th, sh, tw) in &[(-38.0f32, 16.0f32, 20.0f32, 3.2f32), (6.0, 21.0, 26.0, 3.8), (52.0, 12.0, 13.0, 2.8)] {
        let x = x0 + dx;
        let mut t = Held::new(Tool::round_sable(tw), rng.next_u64());
        t.load(pal.paint(dark, 0.15).with_hiding(0.9), 0.9);
        c.drag(&mut t, &Gesture::new(vec![(x, hz), (x, hz - th)]).pressure(0.9, 0.9).ramps(0.0, 0.05).orient(Orient::Across), None);
        // the lit side of the tower (sun from the left)
        let mut l = Held::new(Tool::round_sable(tw * 0.35), rng.next_u64());
        l.load(pal.paint(hex("#a3a8ad"), 0.15), 0.6);
        c.drag(&mut l, &Gesture::new(vec![(x - tw * 0.3, hz - 1.0), (x - tw * 0.3, hz - th + 0.5)]).pressure(0.6, 0.6).ramps(0.1, 0.1).orient(Orient::Across), None);
        let mut s = Held::new(Tool { point: 1.0, ..Tool::round_sable(tw * 0.9) }, rng.next_u64());
        s.load(pal.paint(dark, 0.15).with_hiding(0.9), 0.8);
        c.drag(&mut s, &Gesture::new(vec![(x, hz - th + 0.5), (x, hz - th - sh * 0.5), (x, hz - th - sh)]).pressure(0.9, 0.02).ramps(0.0, 0.9).orient(Orient::Across), None);
    }
    // a windmill on a slight rise to the right
    let (mx, my) = (x0 + 175.0, hz - 1.0);
    let mut m = Held::new(Tool::round_sable(2.4), rng.next_u64());
    m.load(pal.paint(dark, 0.15).with_hiding(0.9), 0.8);
    c.drag(&mut m, &Gesture::new(vec![(mx, my), (mx + 0.3, my - 7.0)]).pressure(1.0, 0.7).ramps(0.0, 0.1).orient(Orient::Across), None);
    let mut sail = Held::new(Tool { point: 1.0, ..Tool::rigger(0.5) }, rng.next_u64());
    for k in 0..4 {
        let a = 0.35 + k as f32 * std::f32::consts::FRAC_PI_2;
        let (hx, hy) = (mx + 0.3, my - 7.2);
        sail.reload(pal.paint(dark, 0.15), 0.8);
        c.drag(&mut sail, &Gesture::new(vec![(hx, hy), (hx + 7.0 * a.cos(), hy - 7.0 * a.sin())]).pressure(0.8, 0.4).ramps(0.0, 0.2).orient(Orient::Across), None);
    }
}

/// A haycock: a round-shouldered heap, lit on the left, its shadow to the
/// right on the stubble.
fn haycock(c: &mut Canvas, pal: &Palette, (x, y): (f32, f32), s: f32, rng: &mut Rng) {
    let mut sh = Held::new(Tool::round_sable(s * 0.5), rng.next_u64());
    sh.load(pal.paint(hex("#6d6a3e"), 0.3), 0.6);
    c.drag(&mut sh, &Gesture::new(vec![(x, y + 0.2), (x + s * 1.4, y + 0.4)]).pressure(0.5, 0.3).ramps(0.1, 0.5).orient(Orient::Across), None);
    let mut b = Held::new(Tool::round_sable(s * 0.55), rng.next_u64());
    for k in 0..5 {
        let u = -0.45 + 0.9 * k as f32 / 4.0;
        let lit = 1.0 - (k as f32 / 4.0);
        let col = mixc(hex("#8a7a44"), hex("#d3bf7e"), lit * 0.9 + rng.range(-0.1, 0.1));
        b.reload(pal.paint(col, 0.2), 0.8);
        let top = y - s * (1.0 - 1.6 * u * u) * 0.95;
        c.drag(&mut b, &Gesture::new(vec![(x + u * s, y), (x + u * s * 0.9, top)]).pressure(0.8, 0.5).ramps(0.05, 0.4).orient(Orient::Across), None);
    }
}

/// A pollard willow, my way: a short thick trunk leaning a little, a
/// knuckled head where it was cut back, and a sheaf of straight young rods
/// rising and fanning out from it, dressed in narrow grey-green leaves that
/// turn silver where the light catches their undersides.
fn willow(c: &mut Canvas, pal: &Palette, (x, y): (f32, f32), ht: f32, _seed: u64, rng: &mut Rng, near: f32) {
    let air = 1.0 - near; // farther = more air
    let tr = ht * 0.36;
    let lean = rng.range(-0.12, 0.12);
    let head = (x + lean * tr, y - tr);
    let bark = mixc(hex("#3a3528"), hex("#7e878a"), air * 0.85);
    let bark_l = mixc(hex("#77705a"), hex("#9aa0a2"), air * 0.85);
    let tw = (ht * 0.11).max(1.0);
    let mut b = Held::new(Tool::round_sable(tw), rng.next_u64());
    b.load(pal.paint(bark, 0.2), 0.9);
    c.drag(&mut b, &Gesture::new(vec![(x, y + 0.5), (x + lean * tr * 0.5 + 0.3, y - tr * 0.5), head]).pressure(1.0, 0.95).ramps(0.0, 0.1).orient(Orient::Across), None);
    // the knuckled head: a few short fat dabs
    for _ in 0..4 {
        let (hx, hy) = (head.0 + rng.range(-0.7, 0.7) * tw, head.1 + rng.range(-0.3, 0.2) * tw);
        c.touch(&mut b, &Touch::at(hx, hy).pressure(0.9).drag(rng.range(-0.4, 0.4) * tw, -0.3 * tw), None);
    }
    let mut lb = Held::new(Tool::round_sable(tw * 0.3), rng.next_u64());
    lb.load(pal.paint(bark_l, 0.2), 0.6);
    c.drag(&mut lb, &Gesture::new(vec![(x - tw * 0.3, y - 0.5), (head.0 - tw * 0.3, head.1 + 1.0)]).pressure(0.6, 0.5).ramps(0.2, 0.3).orient(Orient::Across), None);
    // rods and their leaves
    let rods = if near > 0.5 { 44 } else { 26 };
    let shade = mixc(hex("#46553a"), hex("#8b979a"), air * 0.75);
    let mid = mixc(hex("#6f7c55"), hex("#a0aaa8"), air * 0.75);
    let silver = mixc(hex("#b3b89c"), hex("#bcc2bd"), air * 0.75);
    let rw = (ht * 0.012).max(0.35);
    let mut rod = Held::new(Tool { point: 0.9, ..Tool::rigger(rw) }, rng.next_u64());
    let lw = (ht * 0.03).max(0.55);
    let mut leaf = Held::new(Tool { point: 0.8, ..Tool::round_sable(lw) }, rng.next_u64());
    let mut all: Vec<(f32, f32, f32)> = vec![];
    for k in 0..rods {
        let a = (k as f32 / (rods - 1) as f32 - 0.5) * 1.7 + rng.range(-0.15, 0.15);
        let len = ht * rng.range(0.35, 0.68) * (1.0 - 0.2 * a.abs());
        let bend = a * 0.5;
        let pts: Vec<(f32, f32)> = (0..=5).map(|j| {
            let s = j as f32 / 5.0;
            let ang = a + bend * s;
            (head.0 + (tw * 0.4) * a.sin() + len * s * ang.sin(), head.1 - len * s * ang.cos() * (1.0 - 0.15 * s * a.abs()))
        }).collect();
        rod.reload(pal.paint(mixc(bark, shade, 0.5), 0.25), 0.6);
        c.drag(&mut rod, &Gesture::new(pts.clone()).pressure(0.7, 0.1).ramps(0.05, 0.5).orient(Orient::Across), None);
        let n = (len / (lw * 0.8)) as usize;
        for j in 2..n {
            let s = j as f32 / n as f32;
            let i = ((s * 5.0) as usize).min(4);
            let fr = s * 5.0 - i as f32;
            let px = pts[i].0 + (pts[i + 1].0 - pts[i].0) * fr;
            let py = pts[i].1 + (pts[i + 1].1 - pts[i].1) * fr;
            all.push((px + rng.normal() * lw * 1.3, py + rng.normal() * lw * 0.9, a));
        }
    }
    // leaves back (shade) first, then lit: lit on the sun's side (left) and on top
    all.sort_by(|p, q| p.1.partial_cmp(&q.1).unwrap().reverse());
    for pass in 0..2 {
        for &(px, py, a) in &all {
            let side = ((head.0 - px) / (ht * 0.4)).clamp(-1.0, 1.0);
            let up = ((head.1 - py) / (ht * 0.6)).clamp(0.0, 1.0);
            let lit = (0.35 + 0.45 * side + 0.3 * up + rng.normal() * 0.15).clamp(0.0, 1.0);
            if pass == 0 && rng.chance(0.25) || pass == 1 && lit < 0.55 {
                continue;
            }
            let col = if pass == 0 { mixc(shade, mid, lit) } else if rng.chance(0.3) { silver } else { mixc(mid, silver, lit - 0.4) };
            leaf.reload(pal.paint(col, 0.25), 0.6);
            let dir = a * 0.6 + rng.normal() * 0.5;
            let l = lw * rng.range(1.2, 2.2);
            c.drag(&mut leaf, &Gesture::new(vec![(px, py), (px + l * 0.5 * dir.sin(), py - l * 0.5 * dir.cos() + l * 0.15), (px + l * dir.sin(), py - l * dir.cos() * 0.6 + l * 0.35)]).pressure(0.8, 0.1).ramps(0.05, 0.6).orient(Orient::Across), None);
        }
    }
}

/// Greens for a leaf mass: deep shade, mid, light, full sun.
#[derive(Clone, Copy)]
struct Greens {
    shade: Rgb,
    mid: Rgb,
    light: Rgb,
    sun: Rgb,
}

impl Greens {
    fn at(&self, l: f32) -> Rgb {
        if l < 0.35 {
            mixc(self.shade, self.mid, l / 0.35)
        } else if l < 0.72 {
            mixc(self.mid, self.light, (l - 0.35) / 0.37)
        } else {
            mixc(self.light, self.sun, (l - 0.72) / 0.28)
        }
    }
    /// The same greens seen through `air` (0 near .. 1 far): paler, bluer,
    /// less contrast.
    fn far(&self, air: f32) -> Greens {
        let veil = hex("#9eaab0");
        Greens { shade: mixc(self.shade, veil, air * 0.8), mid: mixc(self.mid, veil, air * 0.7), light: mixc(self.light, veil, air * 0.6), sun: mixc(self.sun, veil, air * 0.5) }
    }
}

/// A small field tree or bush in the middle distance, too small for a
/// grown crown to tell: a trunk stroke, then the mass in small touches,
/// dark all over first, then the side toward the sun (upper left) lighter.
fn small_tree(c: &mut Canvas, pal: &Palette, (cx, cy): (f32, f32), rw: f32, rh: f32, foot: (f32, f32), g: &Greens, rng: &mut Rng) {
    let tw = (rw * 0.07).max(0.4);
    let mut b = Held::new(Tool { point: 0.6, ..Tool::round_sable(tw) }, rng.next_u64());
    b.load(pal.paint(mixc(hex("#3a3a30"), g.shade, 0.6), 0.2), 0.8);
    c.drag(&mut b, &Gesture::new(vec![foot, (foot.0 + 0.2, cy + rh * 0.55)]).pressure(0.8, 0.4).ramps(0.0, 0.4).orient(Orient::Across), None);
    let tip = (rw * 0.22).clamp(0.6, 2.4);
    let mut hd = Held::new(Tool { point: 0.5, ..Tool::round_sable(tip) }, rng.next_u64());
    let n = ((std::f32::consts::PI * rw * rh) / (tip * tip) * 2.2) as usize + 4;
    let lump = rng.range(0.0, 100.0);
    let inside = |u: f32, v: f32| {
        let a = v.atan2(u);
        u * u + v * v < 1.0 + 0.3 * (a * 3.0 + lump).sin() * 0.5 + 0.15 * (a * 5.0 + lump * 1.7).sin()
    };
    for pass in 0..2 {
        for _ in 0..(if pass == 0 { n } else { n / 2 }) {
            let (u, v) = (rng.range(-1.1, 1.1), rng.range(-1.1, 1.1));
            if !inside(u, v) {
                continue;
            }
            let lit = (0.45 - 0.45 * (u * 0.6 + v * 0.8) + rng.normal() * 0.12).clamp(0.0, 1.0);
            if pass == 1 && lit < 0.55 {
                continue;
            }
            let l = if pass == 0 { lit * 0.55 } else { lit };
            hd.reload(pal.paint(g.at(l), 0.25), 0.6);
            let (x, y) = (cx + u * rw, cy + v * rh);
            let a = -0.9 + rng.normal() * 0.6;
            let l = tip * rng.range(0.8, 1.6);
            c.drag(&mut hd, &Gesture::new(vec![(x, y), (x + l * 0.5 * a.cos(), y + l * 0.5 * a.sin() + 0.2 * l), (x + l * a.cos(), y + l * a.sin() + 0.4 * l)]).pressure(rng.range(0.7, 1.0), 0.3).ramps(0.05, 0.5).orient(Orient::Across), None);
        }
    }
}

/// The lime: trunk and the stout limbs that show, then the leaf mass in
/// three passes, dark to light, each touch a hooked leaf stroke.
fn paint_lime(c: &mut Canvas, st: &Style, pal: &Palette, t: &Tree, g: &Greens, seed: u64) {
    let f = c.frame();
    let mut rng = Rng::new(seed);
    // 1. wood: the grown wood strokes of stout wood only
    let bark_d = hex("#2e2a22");
    for (pass, ws) in t.wood_strokes(1.2, 99.0, 0.3).iter().flat_map(|w| [(0, w), (1, w)]) {
        let w0 = ws.w[0].max(0.6) * if pass == 0 { 1.0 } else { 0.7 };
        let mut held = Held::new(Tool { point: 0.6, ..Tool::round_sable(w0) }, rng.next_u64());
        held.load(pal.paint(if pass == 0 { bark_d } else { hex("#26231d") }, 0.4), 1.0);
        let p0 = held.tool.pressure_for(ws.w[0]).clamp(0.2, 1.0);
        let p1 = held.tool.pressure_for(*ws.w.last().unwrap()).clamp(0.05, 1.0);
        c.drag(&mut held, &Gesture::new(ws.pts.clone()).pressure(p0, p1).ramps(0.02, if ws.tip { 0.4 } else { 0.05 }).orient(Orient::Across), None);
    }
    c.dry();
    // lit bark on the trunk's sunward side: lean dry strokes up the grain,
    // grey-green, only where the trunk stands clear of the crown
    let tr = &t.limbs[0];
    let n = tr.pts.len();
    let clear = (0..n).take_while(|&i| tr.pts[i].1 > 430.0).count().max(2);
    for k in 0..4 {
        let off = 0.18 + 0.07 * k as f32;
        let mut lb = Held::new(Tool { point: 0.5, ..Tool::round_sable((tr.w[0] * 0.08).max(0.5)) }, rng.next_u64());
        lb.load(pal.paint(mixc(hex("#5c5a48"), hex("#7a7862"), rng.range(0.0, 1.0)), 0.15), 0.35);
        let pts: Vec<(f32, f32)> = tr.pts[..clear].iter().zip(&tr.w).map(|(&(x, y), &w)| (x - w * off + rng.normal() * 0.3, y)).collect();
        c.drag(&mut lb, &Gesture::new(pts).pressure(0.35, 0.2).ramps(0.2, 0.4).orient(Orient::Across), None);
    }
    // 2. leaves: the mass laid in dark, in short hatched strokes cut to it
    let mask = t.leaves(f);
    let lit = t.light(f);
    let sh = g.shade;
    let md = g.mid;
    c.work(&mask, &st.hatch().mixed(pal, 0.3).color(move |x, y| mixc(sh, md, lit.sample(x, y) * 0.8)).angle(|_, _| -1.0).cross(0.6).length(3.0, 7.0).threshold(0.7), seed ^ 0x51);
    c.dry();
    // 3. the hooked leaf touches, back to front, colored by the light they
    // catch: shade green, mid, sunlit yellow-green
    let tw = t.touch_w.max(0.5);
    let mut hd = Held::new(Tool { point: 0.8, ..Tool::round_sable(tw) }, rng.next_u64());
    for (i, tc) in t.touches.iter().enumerate() {
        let l = (tc.lit * 1.25 + 0.06 + rng.normal() * 0.08).clamp(0.0, 1.0);
        let want = g.at(l);
        if i % 3 == 0 {
            hd.reload(c.aim(pal, want, tc.pts[1], tw, 0.25, 1.0), 0.7);
        } else {
            hd.reload(pal.paint(want, 0.25), 0.7);
        }
        c.drag(&mut hd, &Gesture::new(tc.pts.to_vec()).pressure(0.85, 0.15).ramps(0.05, 0.6).orient(Orient::Across), None);
    }
}

/// Two people seen from behind, standing close, looking out over the
/// plain: a man in a dark green coat and a cap, a woman in a dark red
/// dress with a pale shawl, her hand on his shoulder. `s` is the man's
/// height in units; `at` is between their feet.
fn couple(c: &mut Canvas, pal: &Palette, at: (f32, f32), s: f32, seed: u64) {
    let mut rng = Rng::new(seed);
    let (x, y) = at;
    let p = |u: f32, v: f32| (x + u * s, y - v * s);
    let stroke = |c: &mut Canvas, col: Rgb, wid: f32, pts: &[(f32, f32)], p0: f32, p1: f32, rng: &mut Rng| {
        let mut h = Held::new(Tool { point: 0.5, ..Tool::round_sable(wid * s) }, rng.next_u64());
        h.load(pal.paint(col, 0.15).with_hiding(0.95), 0.9);
        let pts: Vec<(f32, f32)> = pts.iter().map(|&(u, v)| p(u, v)).collect();
        c.drag(&mut h, &Gesture::new(pts).pressure(p0, p1).ramps(0.05, 0.2).orient(Orient::Across), None);
    };
    // the man (right): legs, coat, shoulders, head with cap
    let coat = hex("#28301f");
    let coat_l = hex("#4a5536");
    let legs = hex("#1f1c17");
    let mx = 0.1;
    stroke(c, legs, 0.06, &[(mx - 0.04, 0.0), (mx - 0.035, 0.36)], 0.9, 0.9, &mut rng);
    stroke(c, legs, 0.06, &[(mx + 0.05, 0.0), (mx + 0.045, 0.36)], 0.9, 0.9, &mut rng);
    for k in 0..4 {
        let u = mx - 0.09 + 0.06 * k as f32;
        stroke(c, coat, 0.075, &[(u * 1.15 - mx * 0.15, 0.3), (u, 0.62), (u * 0.8 + mx * 0.2, 0.8)], 0.95, 0.85, &mut rng);
    }
    stroke(c, coat, 0.08, &[(mx - 0.11, 0.78), (mx + 0.11, 0.78)], 0.9, 0.9, &mut rng);
    // sunlit edge of the coat, left
    stroke(c, coat_l, 0.02, &[(mx - 0.1, 0.35), (mx - 0.11, 0.6), (mx - 0.1, 0.79)], 0.7, 0.5, &mut rng);
    // head and cap
    stroke(c, hex("#6b5440"), 0.075, &[(mx, 0.82), (mx, 0.9)], 1.0, 1.0, &mut rng);
    stroke(c, hex("#1a1a17"), 0.09, &[(mx - 0.04, 0.925), (mx + 0.045, 0.93)], 0.9, 0.9, &mut rng);
    // the woman (left): long dress flaring, shawl, hair knot
    let dress = hex("#5a2a22");
    let dress_l = hex("#8a4a36");
    let wx = -0.1;
    for k in 0..5 {
        let u = -0.1 + 0.05 * k as f32;
        stroke(c, dress, 0.07, &[(wx + u * 1.3, 0.0), (wx + u * 0.8, 0.35), (wx + u * 0.55, 0.62)], 0.95, 0.85, &mut rng);
    }
    stroke(c, dress_l, 0.02, &[(wx - 0.13, 0.02), (wx - 0.09, 0.3), (wx - 0.07, 0.6)], 0.7, 0.5, &mut rng);
    // shawl: pale, its point down the back
    stroke(c, hex("#d8d2bc"), 0.05, &[(wx - 0.08, 0.72), (wx, 0.5), (wx + 0.08, 0.72)], 0.9, 0.9, &mut rng);
    stroke(c, hex("#d8d2bc"), 0.06, &[(wx - 0.08, 0.72), (wx + 0.08, 0.72)], 0.9, 0.9, &mut rng);
    // her arm to his shoulder
    stroke(c, dress, 0.035, &[(wx + 0.07, 0.7), (mx - 0.02, 0.76)], 0.8, 0.7, &mut rng);
    // head, hair knot
    stroke(c, hex("#4a3424"), 0.07, &[(wx, 0.76), (wx, 0.84)], 1.0, 1.0, &mut rng);
    stroke(c, hex("#3a281c"), 0.045, &[(wx + 0.005, 0.855), (wx + 0.006, 0.88)], 1.0, 1.0, &mut rng);
}

/// Grass: every tuft's blades pulled up from the foot and lifted off;
/// deeper in the lime's shadow, lighter where the sun turns them.
fn grass(c: &mut Canvas, pal: &Palette, tufts: &[paint::Tuft], ground: &dyn Fn(f32, f32) -> Rgb, shadow: &Mask, seed: u64) {
    let mut rng = Rng::new(seed);
    let mut held = Held::new(Tool::rigger(0.6), rng.next_u64());
    let mut flower = Held::new(Tool::round_sable(1.0), rng.next_u64());
    let sheen = Fbm::new(seed as u32 ^ 0x5ee, 3, 80.0);
    for t in tufts {
        let g = ground(t.at.0, t.at.1);
        let shd = shadow.sample(t.at.0, t.at.1);
        let lit = ((0.4 + 0.5 * sheen.get(t.at.0, t.at.1 * 3.0) + 0.2 * rng.normal() - 0.2 * t.lush) * (1.0 - 0.8 * shd)).clamp(0.0, 1.0);
        let want = if lit < 0.5 { mixc([g[0] * 0.45, g[1] * 0.55, g[2] * 0.45], g, lit / 0.5) } else { mixc(g, hex("#b9b46a"), (lit - 0.5) * 1.4) };
        let bw = (t.height * 0.06).clamp(0.3, 1.3);
        held.tool = Tool { point: 0.8, length: bw * 5.0, ..Tool::rigger(bw) };
        let p = c.aim(pal, want, t.at, t.height * 0.3, 0.15, 1.2);
        for b in &t.blades {
            held.reload(p, 0.9);
            let pts: Vec<(f32, f32)> = (0..=4).map(|k| {
                let s = k as f32 / 4.0;
                let q = |a: f32, m: f32, e: f32| (1.0 - s) * (1.0 - s) * a + 2.0 * s * (1.0 - s) * m + s * s * e;
                (q(b[0].0, b[1].0, b[2].0), q(b[0].1, b[1].1, b[2].1))
            }).collect();
            c.drag(&mut held, &Gesture::new(pts).pressure(0.9, 0.1).ramps(0.05, 0.55).orient(Orient::Across), None);
        }
        if let Some(fl) = t.flower {
            let col = match fl.kind { 0 => hex("#ebe6d4"), 1 => hex("#dcb834"), _ => hex("#b8452e") };
            flower.tool = Tool::round_sable((fl.r * 2.0).max(0.5));
            flower.reload(pal.mix(col).paint(0.1), 0.8);
            c.touch(&mut flower, &Touch::at(fl.at.0, fl.at.1).pressure(0.6), None);
        }
    }
}

fn stroke(c: &mut Canvas, pal: &Palette, rng: &mut Rng, tool: Tool, col: Rgb, pts: Vec<(f32, f32)>, p: (f32, f32), ramps: (f32, f32)) {
    let mut h = Held::new(tool, rng.next_u64());
    h.load(pal.paint(col, 0.2).with_hiding(0.9), 0.8);
    c.drag(&mut h, &Gesture::new(pts).pressure(p.0, p.1).ramps(ramps.0, ramps.1).orient(Orient::Across), None);
}

/// A few tall grass stalks gone to seed, bending a little, their heads
/// straw-colored and lit.
fn tall_grass(c: &mut Canvas, pal: &Palette, (x, y): (f32, f32), ht: f32, g: Rgb, rng: &mut Rng) {
    for _ in 0..rng.range(4.0, 8.0) as u32 {
        let x0 = x + rng.range(-6.0, 6.0);
        let hh = ht * rng.range(0.6, 1.0);
        let lean = rng.range(-0.25, 0.3);
        let pts: Vec<(f32, f32)> = (0..=6).map(|k| {
            let s = k as f32 / 6.0;
            (x0 + lean * hh * s * s, y - hh * s)
        }).collect();
        let stem = mixc([g[0] * 0.7, g[1] * 0.75, g[2] * 0.6], hex("#9c9463"), 0.5);
        stroke(c, pal, rng, Tool { point: 1.0, ..Tool::rigger(0.45) }, stem, pts.clone(), (0.8, 0.3), (0.05, 0.3));
        // the seed head: a narrow spike of short touches along the top fifth
        let (tx, ty) = *pts.last().unwrap();
        let (bx, by) = pts[4];
        for k in 0..9 {
            let s = k as f32 / 8.0;
            let (px, py) = (bx + (tx - bx) * s, by + (ty - by) * s);
            let col = mixc(hex("#8a7e52"), hex("#d2c48a"), rng.range(0.2, 1.0));
            let side = if k % 2 == 0 { -1.0 } else { 1.0 };
            stroke(c, pal, rng, Tool { point: 1.0, ..Tool::round_sable(0.9) }, col, vec![(px, py), (px + side * 1.2, py - 1.8)], (0.7, 0.2), (0.05, 0.5));
        }
    }
}

/// Dock: a rosette of broad long leaves, dark, the lit edges and midribs
/// picked out, a rusty seed stalk rising from it.
fn dock(c: &mut Canvas, pal: &Palette, (x, y): (f32, f32), s: f32, g: Rgb, rng: &mut Rng) {
    let dark = mixc([g[0] * 0.55, g[1] * 0.62, g[2] * 0.5], hex("#26301c"), 0.4);
    let lit = mixc(g, hex("#9ba25a"), 0.6);
    for k in 0..6 {
        let a = -std::f32::consts::PI * (0.12 + 0.76 * k as f32 / 5.0) + rng.range(-0.1, 0.1);
        let l = s * rng.range(0.7, 1.1);
        let (ex, ey) = (x + l * a.cos(), y + l * a.sin() * 0.6);
        let mid = (x + l * 0.5 * a.cos(), y + l * 0.5 * a.sin() * 0.6 - l * 0.12);
        stroke(c, pal, rng, Tool { point: 0.7, ..Tool::round_sable(s * 0.22) }, dark, vec![(x, y), mid, (ex, ey)], (0.9, 0.2), (0.1, 0.55));
        stroke(c, pal, rng, Tool { point: 1.0, ..Tool::rigger(0.35) }, lit, vec![(x, y - 0.5), mid, (ex, ey)], (0.6, 0.1), (0.1, 0.5));
    }
    let rust = hex("#6e3f24");
    let hh = s * 2.2;
    stroke(c, pal, rng, Tool { point: 1.0, ..Tool::rigger(0.6) }, rust, vec![(x, y - 2.0), (x + 1.0, y - hh * 0.5), (x + 2.5, y - hh)], (0.8, 0.3), (0.05, 0.3));
    for k in 0..14 {
        let t = 0.45 + 0.55 * k as f32 / 13.0;
        let (px, py) = (x + 2.5 * t * t, y - hh * t);
        let side = if k % 2 == 0 { -1.0 } else { 1.0 };
        let rc = mixc(rust, hex("#a0603a"), rng.range(0.0, 1.0));
        stroke(c, pal, rng, Tool::round_sable(1.0), rc, vec![(px, py), (px + side * 1.6, py + 0.6)], (0.8, 0.5), (0.05, 0.3));
    }
}

/// Yarrow: stiff stems with flat white flower heads, a few feathery leaves.
fn yarrow(c: &mut Canvas, pal: &Palette, (x, y): (f32, f32), ht: f32, g: Rgb, rng: &mut Rng) {
    for _ in 0..rng.range(2.0, 4.0) as u32 {
        let x0 = x + rng.range(-5.0, 5.0);
        let hh = ht * rng.range(0.7, 1.0);
        let tx = x0 + rng.range(-3.0, 3.0);
        let stem = mixc(g, hex("#4a5230"), 0.5);
        stroke(c, pal, rng, Tool { point: 1.0, ..Tool::rigger(0.5) }, stem, vec![(x0, y), (x0 + (tx - x0) * 0.4, y - hh * 0.5), (tx, y - hh)], (0.8, 0.5), (0.05, 0.2));
        // the umbel: a flat cluster of small touches, shaded below
        for k in 0..22 {
            let (u, v) = (rng.normal() * 3.2, rng.normal().abs() * 0.9);
            let col = if v > 0.9 { hex("#bdb8a4") } else { mixc(hex("#e9e5d6"), hex("#f6f2e6"), rng.range(0.0, 1.0)) };
            let _ = k;
            let mut f = Held::new(Tool::round_sable(0.9), rng.next_u64());
            f.load(pal.paint(col, 0.1), 0.8);
            c.touch(&mut f, &Touch::at(tx + u, y - hh + v - 0.5).pressure(0.7), None);
        }
        for k in 0..3 {
            let t = 0.2 + 0.2 * k as f32;
            let (px, py) = (x0 + (tx - x0) * t, y - hh * t);
            let side = if k % 2 == 0 { -1.0 } else { 1.0 };
            stroke(c, pal, rng, Tool { point: 1.0, ..Tool::rigger(0.4) }, stem, vec![(px, py), (px + side * 4.0, py - 3.0), (px + side * 7.0, py - 3.5)], (0.7, 0.1), (0.05, 0.6));
        }
    }
}

/// A thistle: a stiff stem, spiny leaves out from it, two purple heads.
fn thistle(c: &mut Canvas, pal: &Palette, (x, y): (f32, f32), ht: f32, g: Rgb, rng: &mut Rng) {
    let stem = mixc(g, hex("#56603e"), 0.5);
    let tx = x + rng.range(-2.0, 2.0);
    stroke(c, pal, rng, Tool { point: 0.8, ..Tool::round_sable(1.1) }, stem, vec![(x, y), (x + (tx - x) * 0.5, y - ht * 0.5), (tx, y - ht)], (0.9, 0.6), (0.05, 0.2));
    for k in 0..6 {
        let t = 0.12 + 0.13 * k as f32;
        let (px, py) = (x + (tx - x) * t, y - ht * t);
        let side = if k % 2 == 0 { -1.0 } else { 1.0 };
        let l = ht * (0.32 - 0.03 * k as f32);
        let lc = mixc(stem, hex("#8c9575"), 0.4);
        let pts: Vec<(f32, f32)> = (0..=4).map(|j| {
            let s = j as f32 / 4.0;
            let zig = if j % 2 == 1 { -1.6 } else { 0.0 };
            (px + side * l * s, py - l * 0.25 * s + zig + l * 0.2 * s * s)
        }).collect();
        stroke(c, pal, rng, Tool { point: 1.0, ..Tool::round_sable(1.6) }, lc, pts, (0.8, 0.1), (0.05, 0.6));
    }
    for &(dx, dy) in &[(0.0f32, 0.0f32), (5.0, 7.0)] {
        let (hx, hy) = (tx + dx, y - ht + dy);
        if dx > 0.0 {
            stroke(c, pal, rng, Tool { point: 1.0, ..Tool::rigger(0.5) }, stem, vec![(tx, hy + 6.0), (hx, hy + 1.0)], (0.7, 0.5), (0.05, 0.2));
        }
        stroke(c, pal, rng, Tool::round_sable(3.0), hex("#4e5a3a"), vec![(hx, hy + 1.5), (hx, hy - 0.5)], (0.9, 0.9), (0.0, 0.2));
        for k in 0..9 {
            let a = -std::f32::consts::PI * (0.15 + 0.7 * k as f32 / 8.0);
            let pc = mixc(hex("#7a3d6a"), hex("#b06a98"), rng.range(0.0, 1.0));
            stroke(c, pal, rng, Tool { point: 1.0, ..Tool::round_sable(0.8) }, pc, vec![(hx, hy - 0.8), (hx + 3.0 * a.cos(), hy - 0.8 + 3.2 * a.sin())], (0.8, 0.2), (0.05, 0.5));
        }
    }
}
