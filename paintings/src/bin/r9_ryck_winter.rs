//! "Winter Morning on the Ryck": an original painting in the manner of
//! Caspar David Friedrich, made from what is known of his materials and
//! habits, never from pictures.
//!
//! The frozen river below Greifswald on a still, veiled morning: the sun
//! low in the east behind haze, a row of pollard willows marching along the
//! right bank to the flat horizon, the town's towers faint in the distance,
//! a post mill on the left, one man seen from behind on the bank. Notes and
//! friction list: notes/round9/arm2/notes.md.

use paint::{Fbm, Gesture, Held, Mask, Mix, Orient, Paint, Rgb, Rng, Style, Tool, Touch, gradient, hex, smoothstep};
use paintings::run::Finish;

const ASPECT: f32 = 1.45;

/// A brush loaded by hand from a pile mixed on the palette.
fn pile(st: &Style, col: &str, hiding: f32, stiff: f32) -> Paint {
    Paint::new(st.palette.mix(hex(col)).color, hiding, stiff)
}

fn main() {
    let o = paintings::run::Run::new("r9_ryck_winter");
    let mut rng = Rng::new(o.seed);
    let seed = o.seed as u32;
    let st = Style::friedrich();

    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let (w, h) = (c.width(), c.height());
    let f = c.frame();

    // ---- the lay of the land (units, y down). A flat coastal plain: the
    // horizon low, a little below the middle of the picture's height
    let yh = h * 0.615;
    let hn = Fbm::new(seed + 1, 4, 120.0);
    // the far edge of the land is not ruled: very slight swells, tree lines
    let horizon = f.per_column(move |x: f32| yh + 1.2 * hn.get(x, 0.0));
    // depth t = 0 at the horizon, 1 at the bottom edge (the ground plane
    // seen in perspective: widths and heights scale with t)
    let depth = move |y: f32| ((y - yh) / (h - yh)).clamp(0.0, 1.0);
    let rn = Fbm::new(seed + 2, 4, 40.0);
    // the river comes out from under the sun on the left of the horizon and
    // winds toward us, swinging out in a slow bend before it reaches the
    // bottom edge
    let river_c = move |y: f32| {
        let t = depth(y);
        262.0 + 60.0 * t + 120.0 * (std::f32::consts::PI * t.powf(0.75)).sin()
    };
    let river_hw = move |y: f32| {
        let t = depth(y);
        1.5 + 150.0 * t.powf(1.15)
    };
    // the banks wander a little: a bank is never a ruled curve
    let bank_l = move |y: f32| river_c(y) - river_hw(y) + 16.0 * depth(y) * rn.get(y * 1.3, 1.0);
    let bank_r = move |y: f32| river_c(y) + river_hw(y) + 16.0 * depth(y) * rn.get(y * 1.3, 9.0);
    // the path with its willows runs straight to the town
    let path_x = move |t: f32| 596.0 + 250.0 * t;
    let row_x = move |t: f32| 604.0 + 380.0 * t;
    let soft = |d: f32, e: f32| smoothstep(-e, e, d);
    let sky = Mask::from_fn(f, |x, y| 1.0 - soft(y - horizon(x), 0.7));
    let land = Mask::from_fn(f, |x, y| soft(y - horizon(x), 0.7));
    let ice = Mask::from_fn(f, |x, y| {
        if y < yh {
            return 0.0;
        }
        let e = 0.8 + 3.0 * depth(y);
        soft(x - bank_l(y), e) * (1.0 - soft(x - bank_r(y), e)) * soft(y - horizon(x) - 0.5, 0.6)
    });
    let snow_fields = Mask::from_fn(f, |x, y| land.sample(x, y) * (1.0 - ice.sample(x, y)));

    // the sun: low in the east (left), behind the haze, just above the land
    let (sx, sy) = (w * 0.27, yh - 58.0);

    if o.stage("underpainting", &mut c, &mut rng) {
        // a thin warm-brown underpainting for the values: barely there in
        // the sky, heavier on the land and in the willows' corner
        let whole = Mask::full(f);
        let under = |x: f32, y: f32| 0.15 + 0.5 * smoothstep(yh - 20.0, yh + 60.0, y) + 0.3 * smoothstep(500.0, 900.0, x) * smoothstep(yh, h, y);
        c.work(&whole, &st.glaze(0.9).color(|_, _| hex("#6f5139")).angle(|_, _| 0.0).load_at(under), o.seed * 100 + 1);
        if let Some(b) = st.blend() {
            c.work(&whole, &b.angle(|_, _| 0.0), o.seed * 100 + 2);
        }
        c.dry();
    }

    // ---- the sky of a still frost morning: a cool, pale grey-blue high up
    // (smalt, white, a touch of black), warming to a pale straw and rose
    // near the land, brightest in the haze where the sun is
    let sky_stops: [(f32, Rgb); 6] = [
        (0.00, hex("#8e97a4")),
        (0.22, hex("#a5aab2")),
        (0.42, hex("#bdbcbb")),
        (0.55, hex("#d2c7ba")),
        (0.61, hex("#dccdb8")),
        (0.66, hex("#d8c8b6")),
    ];
    let drift = Fbm::new(seed + 10, 4, 400.0);
    let sky_color = move |x: f32, y: f32| {
        let base = gradient(&sky_stops, (y / h + 0.01 * drift.get(x * 0.4, y)).clamp(0.0, 0.66), Mix::Pigment);
        // the glow of the veiled sun: wide, soft, warmer and lighter
        let d = (((x - sx) / 1.6).powi(2) + (y - sy).powi(2)).sqrt();
        let g = (-(d / 150.0).powi(2)).exp() * 0.8;
        gradient(&[(0.0, base), (1.0, hex("#eee2c8"))], g, Mix::Pigment)
    };
    let sky_angle = move |x: f32, y: f32| 0.02 * drift.get(x, y * 3.0);
    if o.stage("sky", &mut c, &mut rng) {
        c.work(&sky, &st.broad().color(sky_color).angle(sky_angle).medium(0.3).load(0.85).pressure(0.65, 0.9).coverage(4.5), o.seed * 100 + 10);
        c.work(&sky, &st.broad().color(sky_color).angle(sky_angle).coverage(2.0).length(90.0, 240.0).medium(0.5), o.seed * 100 + 11);
        // long bands of stratus, laid wet into the wet sky with a filbert: a
        // little darker and bluer high up, faintly lit rose low down
        let mut fil = Held::new(Tool { lay: 0.5, ..Tool::filbert(11.0) }, 12);
        for k in 0..14 {
            let v = rng.range(0.06, 0.5);
            let y = h * v;
            let x0 = rng.range(-150.0, w * 0.85);
            let len = rng.range(160.0, 480.0);
            let col = if v < 0.3 { gradient(&[(0.0, hex("#7f8895")), (1.0, hex("#9ea1a8"))], v / 0.3, Mix::Pigment) } else { gradient(&[(0.0, hex("#b1afb0")), (1.0, hex("#cdc3b8"))], (v - 0.3) / 0.2, Mix::Pigment) };
            fil.reload(Paint::new(st.palette.mix(col).color, 0.55, 0.45), 0.7 * 0.5);
            let sag = rng.range(-4.0, 4.0);
            let n = 5;
            let pts: Vec<(f32, f32)> = (0..=n).map(|j| {
                let u = j as f32 / n as f32;
                (x0 + len * u, y + sag * (u * 3.1).sin() + rng.range(-1.0, 1.0))
            }).collect();
            let p0 = rng.range(0.3, 0.55);
            c.drag(&mut fil, &Gesture::new(pts).pressure(p0, p0 * 0.6).ramps(0.25, 0.35).swell(vec![0.8, 1.1, 0.9, 1.05]).orient(Orient::Across), Some(&sky));
            let _ = k;
        }
        // the sun's place: pale warm touches curling round it, melted by the
        // badger into the haze; no disc, the sun is behind the veil
        let mut glow = Held::new(Tool::filbert(9.0), 13);
        for k in 0..14 {
            let a0 = k as f32 * 0.47 + rng.range(-0.2, 0.2);
            let r = rng.range(6.0, 34.0);
            let pts: Vec<(f32, f32)> = (0..5).map(|j| {
                let a = a0 + j as f32 * 0.2;
                (sx + 1.5 * r * a.cos(), sy + r * a.sin())
            }).collect();
            glow.reload(pile(&st, "#f1e8d2", 0.6, 0.4), 0.6 * 0.4);
            c.drag(&mut glow, &Gesture::new(pts).pressure(0.5, 0.35).ramps(0.3, 0.4), Some(&sky));
        }
        for k in 0..st.blend_passes {
            if let Some(b) = st.blend() {
                c.work(&sky, &b.angle(sky_angle), o.seed * 100 + 14 + k as u64);
            }
        }
        c.dry();
    }

    if o.stage("distance", &mut c, &mut rng) {
        // the far fields: pale snow in thin level strokes, a cool violet grey
        // that lightens toward the viewer, the sun's side a little warmer
        let band = Mask::from_fn(f, |x, y| land.sample(x, y) * (1.0 - smoothstep(yh + 18.0, yh + 40.0, y)));
        let far_c = |x: f32, y: f32| {
            let t = ((y - yh) / 30.0).clamp(0.0, 1.0);
            let base = gradient(&[(0.0, hex("#aaa6ae")), (1.0, hex("#c4c2c6"))], t, Mix::Pigment);
            let warm = (-((x - sx) / 180.0).powi(2)).exp() * 0.35;
            gradient(&[(0.0, base), (1.0, hex("#d4c8bc"))], warm, Mix::Pigment)
        };
        c.work(&band, &st.body().color(far_c).angle(|_, _| 0.0).angle_jitter(0.02).length(40.0, 120.0).coverage(4.0).clip(true).threshold(0.05), o.seed * 100 + 20);
        c.dry();
        // tree lines and hedges on the horizon: short level dabs of a grey
        // violet, broken, some rising into the tufts of far copses
        let mut sab = Held::new(Tool::round_sable(1.6), 21);
        let mut x = -10.0;
        while x < w + 10.0 {
            let gap = rng.range(8.0, 70.0);
            let run = rng.range(10.0, 90.0);
            if (x - 560.0).abs() > 150.0 || rng.f() < 0.4 {
                let yy = horizon(x) + rng.range(-0.3, 0.8);
                let dark = if (x - sx).abs() < 160.0 { "#9d97a0" } else { "#8a8795" };
                sab.reload(pile(&st, dark, 0.8, 0.5), 0.5);
                let mut xx = x;
                while xx < x + run {
                    let seg = rng.range(3.0, 14.0);
                    let hump = rng.range(0.4, 2.4);
                    let pts = vec![(xx, yy), (xx + seg * 0.5, yy - hump), (xx + seg, yy - rng.range(0.0, 0.8))];
                    c.drag(&mut sab, &Gesture::new(pts).pressure(rng.range(0.35, 0.7), rng.range(0.3, 0.6)).ramps(0.2, 0.3), None);
                    xx += seg * rng.range(0.6, 1.1);
                }
                // a copse: a few taller round tufts rising from the line
                if rng.f() < 0.35 {
                    let cx = x + rng.range(0.0, run);
                    for _ in 0..rng.range(3.0, 7.0) as u32 {
                        let tx = cx + rng.range(-7.0, 7.0);
                        let th = rng.range(3.0, 8.0);
                        c.touch(&mut sab, &Touch::at(tx, horizon(tx) - th * 0.5).pressure(rng.range(0.4, 0.8)).drag(rng.range(-0.6, 0.6), th), None);
                    }
                }
            }
            x += run + gap;
        }
        c.dry();
    }

    // ---- the town on the horizon: Greifswald's three towers, far off, paler
    // than the tree lines (they stand in more air): x positions
    let town = (620.0f32, 745.0f32);
    if o.stage("town", &mut c, &mut rng) {
        let air = pile(&st, "#9f9aa3", 0.85, 0.6);
        let roofs = pile(&st, "#a29ca4", 0.85, 0.6);
        // the roofs: a low broken band, gable after gable
        let mut r = Held::new(Tool::round_sable(1.4), 30);
        r.load(roofs, 0.6);
        let mut x = town.0 - 20.0;
        while x < town.1 + 25.0 {
            let gw = rng.range(3.0, 8.0);
            let gh = rng.range(2.0, 5.0);
            let base = horizon(x) + 0.5;
            c.drag(&mut r, &Gesture::new(vec![(x, base), (x + gw * 0.5, base - gh), (x + gw, base)]).pressure(0.7, 0.6).ramps(0.1, 0.2), None);
            c.touch(&mut r, &Touch::at(x + gw * 0.5, base - gh * 0.4).pressure(0.8).drag(gw * 0.4, 0.0), None);
            x += gw * rng.range(0.7, 1.3);
            if rng.f() < 0.1 {
                r.reload(roofs, 0.6);
            }
        }
        // a tower: a shaft in two or three touches, then the spire as a
        // stroke pulled up to a point off the pointed sable
        let mut tw = Held::new(Tool { point: 1.0, ..Tool::round_sable(2.6) }, 31);
        let tower = |c: &mut paint::Canvas, tw: &mut Held, x: f32, shaft: f32, width: f32, spire: f32, blunt: bool| {
            let b = horizon(x) + 1.0;
            tw.reload(air, 0.8);
            for k in 0..3 {
                let dx = (k as f32 - 1.0) * width * 0.3;
                c.drag(tw, &Gesture::new(vec![(x + dx, b), (x + dx * 0.95, b - shaft)]).pressure(0.8, 0.75).ramps(0.05, 0.1).shake(0.4), None);
            }
            if blunt {
                // a low pyramid roof with a small lantern
                c.drag(tw, &Gesture::new(vec![(x - width * 0.5, b - shaft), (x, b - shaft - spire * 0.4), (x + width * 0.5, b - shaft)]).pressure(0.5, 0.5).ramps(0.1, 0.1), None);
                c.drag(tw, &Gesture::new(vec![(x, b - shaft - spire * 0.3), (x, b - shaft - spire)]).pressure(0.35, 0.02).ramps(0.0, 0.8), None);
            } else {
                c.drag(tw, &Gesture::new(vec![(x, b - shaft + 1.0), (x, b - shaft - spire)]).pressure(0.75, 0.0).ramps(0.0, 0.95), None);
            }
        };
        // St. Nikolai: tall, with its stepped spire; St. Marien: the heavy
        // blunt tower; St. Jakobi: slimmer, to the right
        tower(&mut c, &mut tw, 655.0, 19.0, 4.2, 18.0, false);
        tower(&mut c, &mut tw, 690.0, 15.0, 5.0, 7.0, true);
        tower(&mut c, &mut tw, 728.0, 13.0, 3.2, 13.0, false);
        // a post mill on the left, far off: a box on a post and four sails
        let (mx, mb) = (148.0, horizon(148.0) + 0.5);
        let mill = pile(&st, "#908b96", 0.85, 0.6);
        let mut ms = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.6) }, 32);
        ms.load(mill, 0.8);
        c.drag(&mut ms, &Gesture::new(vec![(mx, mb), (mx, mb - 7.0)]).pressure(0.5, 0.5), None);
        c.drag(&mut ms, &Gesture::new(vec![(mx - 1.3, mb - 7.0), (mx - 1.3, mb - 12.5)]).pressure(1.0, 1.0).ramps(0.0, 0.0), None);
        c.drag(&mut ms, &Gesture::new(vec![(mx + 1.2, mb - 7.0), (mx + 1.2, mb - 12.5)]).pressure(1.0, 1.0).ramps(0.0, 0.0), None);
        let hub = (mx + 2.6, mb - 10.5);
        for k in 0..4 {
            let a = 0.35 + k as f32 * std::f32::consts::FRAC_PI_2;
            let l = 10.0;
            c.drag(&mut ms, &Gesture::new(vec![hub, (hub.0 + l * a.cos(), hub.1 + l * a.sin())]).pressure(0.45, 0.3).ramps(0.05, 0.3).orient(Orient::Along), None);
        }
        c.dry();
    }

    if o.stage("fields", &mut c, &mut rng) {
        // the snow of the near fields in body color: level strokes far off,
        // following the gentle roll of the ground nearer. The snow is light
        // but never white: cool in its shadows, warm where the low sun lies
        let sn = Fbm::new(seed + 40, 4, 110.0);
        let near_fields = Mask::from_fn(f, |x, y| snow_fields.sample(x, y) * smoothstep(yh + 14.0, yh + 30.0, y));
        let snow_c = move |x: f32, y: f32| {
            let t = depth(y);
            let base = gradient(&[(0.0, hex("#c9c6c8")), (0.35, hex("#d8d4cf")), (0.7, hex("#dcd8d0")), (1.0, hex("#cfccca"))], t, Mix::Pigment);
            // soft undulations of the drifts: shadowed troughs, sunlit crests
            let u = sn.get(x * 0.7, y * 2.2);
            let shade = gradient(&[(0.0, hex("#aeb0bb")), (0.5, base), (1.0, hex("#e6dccb"))], (0.5 + 0.9 * u).clamp(0.0, 1.0), Mix::Pigment);
            let v = gradient(&[(0.0, base), (1.0, shade)], 0.75, Mix::Pigment);
            // the near foot of the picture lies in the shade of a drift
            let near = smoothstep(h * 0.88, h, y) * (0.6 + 0.4 * smoothstep(-0.2, 0.3, sn.get(x * 0.3, 7.0)));
            gradient(&[(0.0, v), (1.0, hex("#a9abb6"))], near * 0.8, Mix::Pigment)
        };
        let slope = move |x: f32, y: f32| 0.06 * sn.get(x * 0.5, y) + 0.04 * (x - 500.0) / 500.0 * depth(y);
        c.work(&near_fields, &st.body().color(snow_c).angle(slope).angle_jitter(0.05).length(30.0, 110.0).coverage(4.0).load(0.75).clip(true).threshold(0.05), o.seed * 100 + 40);
        if let Some(b) = st.blend() {
            c.work(&near_fields, &b.angle(|_, _| 0.0).pressure(0.3, 0.4).coverage(1.5), o.seed * 100 + 41);
        }
        c.dry();
    }

    if o.stage("ice", &mut c, &mut rng) {
        // the frozen river: grey ice under a skin of blown snow, the pale
        // sky and the veiled sun's light lying in it along the far reach
        let isn = Fbm::new(seed + 50, 4, 70.0);
        let ice_c = move |x: f32, y: f32| {
            let t = depth(y);
            // clear ice is dark and takes the sky's color: the grey-blue of
            // the zenith near us, the pale haze at the far reach
            let base = gradient(&[(0.0, hex("#c9c1b6")), (0.3, hex("#a9a9ad")), (1.0, hex("#7f8791"))], t, Mix::Pigment);
            // the veiled sun lies in the far ice as a long warm sheen
            let sheen = (-((x - sx - 90.0 * t) / (18.0 + 120.0 * t)).powi(2)).exp() * (1.0 - t).powf(1.4) * 0.9;
            let lit = gradient(&[(0.0, base), (1.0, hex("#e6dac4"))], sheen, Mix::Pigment);
            // wind-blown snow over the ice in long flat tongues
            let s = smoothstep(0.05, 0.4, isn.get(x * 0.25, y * 3.0) + 0.25 * isn.get(x * 1.1, y * 5.0));
            gradient(&[(0.0, lit), (1.0, hex("#d4d1ce"))], s * 0.85, Mix::Pigment)
        };
        c.work(&ice, &st.broad().color(ice_c).angle(|_, _| 0.0).angle_jitter(0.03).length(40.0, 160.0).medium(0.3).load(0.6).coverage(4.0).clip(true), o.seed * 100 + 50);
        if let Some(b) = st.blend() {
            c.work(&ice, &b.angle(|_, _| 0.0), o.seed * 100 + 51);
        }
        c.dry();
        // snow blown over the ice: long flat tongues dragged out from either
        // bank with a filbert of stiff snow color, leaving a winding channel
        // of dark clear ice down the middle; at the far reach they close up
        let mut tg = Held::new(Tool { lay: 0.8, ..Tool::filbert(6.0) }, 53);
        let mut y = yh + 3.0;
        while y < h + 6.0 {
            let t = depth(y);
            let (l, r) = (bank_l(y), bank_r(y));
            let wid = r - l;
            for side in [0usize, 1] {
                if rng.f() < 0.45 {
                    continue;
                }
                let reach = wid * rng.range(0.1, 0.6) * rng.range(0.5, 1.0) * (if t < 0.12 { 1.6 } else { 1.0 });
                let (x0, dir) = if side == 0 { (l - 2.0 * t, 1.0) } else { (r + 2.0 * t, -1.0) };
                tg.tool.width = 1.2 + 11.0 * t;
                let col = if rng.f() < 0.5 { "#cfccca" } else { "#bdbdc2" };
                tg.reload(pile(&st, col, 0.75, 0.7), rng.range(0.35, 0.7));
                // the wind came from the west (left): the drifts trail
                // downwind, the left bank's out into the ice and down
                let sag = (if side == 0 { rng.range(1.0, 5.0) } else { rng.range(-3.0, 1.0) }) * (0.3 + t) * reach / 30.0;
                let pts: Vec<(f32, f32)> = (0..5).map(|j| {
                    let u = j as f32 / 4.0;
                    (x0 + dir * reach * u, y + sag * u * u + rng.range(-0.3, 0.3) * t)
                }).collect();
                c.drag(&mut tg, &Gesture::new(pts).pressure(rng.range(0.55, 0.9), rng.range(0.05, 0.3)).ramps(0.05, rng.range(0.4, 0.8)).swell(vec![1.0, 0.8, 1.1]), Some(&ice));
            }
            y += (1.5 + 13.0 * t) * rng.range(0.4, 1.8);
        }
        // softened while wet with light level strokes of the badger
        if let Some(b) = st.blend() {
            c.work(&ice, &b.angle(|_, _| 0.0).pressure(0.3, 0.4).coverage(1.5), o.seed * 100 + 55);
        }
        // the veiled sun's light on the far reach: a few level, lean,
        // barely pressed strokes of pale warm light, melted into the ice
        let mut sh = Held::new(Tool { lay: 0.5, stiffness: 0.3, ..Tool::filbert(4.0) }, 54);
        for _ in 0..16 {
            let t = rng.range(0.0, 0.3f32).powf(1.3);
            let y = yh + 1.0 + t * (h - yh);
            let (l, r) = (bank_l(y), bank_r(y));
            let x0 = rng.range(l, r - 4.0);
            sh.tool.width = 1.0 + 6.0 * t;
            sh.reload(pile(&st, "#e9dec8", 0.5, 0.4), 0.5);
            c.drag(&mut sh, &Gesture::new(vec![(x0, y), (x0 + (r - l) * 0.3, y + 0.2), (x0 + (r - l) * 0.6, y)]).pressure(0.4, 0.1).ramps(0.3, 0.5), Some(&ice));
        }
        c.dry();
        // cracks and the dark seams where the ice meets the banks: fine
        // hairlines, pulled across the river in long flat arcs
        let mut hl = Held::new(Tool { point: 1.0, ..Tool::rigger(0.9) }, 52);
        for _ in 0..28 {
            let y = yh + (h - yh) * rng.range(0.08, 1.0f32).powf(1.3);
            let (l, r) = (bank_l(y), bank_r(y));
            let x0 = rng.range(l, r);
            let len = (r - l) * rng.range(0.15, 0.6);
            hl.reload(pile(&st, "#6c7179", 0.7, 0.5), 0.35);
            let sag = rng.range(-2.0, 2.0) * (0.3 + depth(y));
            let mut pts = Vec::new();
            let mut xx = x0;
            let mut yy = y;
            for _ in 0..6 {
                pts.push((xx, yy));
                xx += len / 5.0;
                yy += sag * 0.3 + rng.range(-0.8, 0.8) * depth(y);
            }
            c.drag(&mut hl, &Gesture::new(pts).pressure(rng.range(0.15, 0.35), 0.05).ramps(0.2, 0.5).orient(Orient::Along), Some(&ice));
        }
        c.dry();
    }

    // ---- the pollard willows along the right bank, from the front one to
    // the smallest at the horizon (painted far to near, so the near ones
    // stand over the far)
    let willow_ts = [0.03f32, 0.045, 0.065, 0.092, 0.13, 0.18, 0.25, 0.345, 0.47, 0.64];
    let mut willows: Vec<(f32, f32, f32)> = willow_ts
        .iter()
        .enumerate()
        .map(|(i, &t)| {
            let y = yh + t * (h - yh) + (i as f32 * 1.7).sin() * 2.0 * t;
            // planted by hand, not by a surveyor: each a little off the line
            let x = row_x(t) + 14.0 * t * (i as f32 * 2.3).cos();
            let ht = (330.0 + 30.0 * (i as f32 * 1.9).sin()) * t;
            (x, y, ht)
        })
        .collect();
    // and one old pollard close by on the left bank, near the frame: its
    // crown stands dark against the sky, a repoussoir
    willows.push((78.0, h - 12.0, 540.0));
    // the figure: on the path, a way ahead of us, walking toward the town
    let ft = 0.5f32;
    let (fx, fy, fh) = (path_x(ft) + 4.0, yh + ft * (h - yh), 108.0 * ft);

    if o.stage("banks", &mut c, &mut rng) {
        // the banks: where the snow of the fields breaks over the ice there
        // is a dark lip (the undercut earth, old reeds), drawn down each bank
        // from the far end toward us in runs of a few strokes, the brush
        // lifting and setting down again, wider as the bank comes nearer
        let mut edge = Held::new(Tool { point: 0.7, ..Tool::round_sable(2.0) }, 60);
        for side in [0usize, 1] {
            let mut y = yh + 1.0;
            while y < h + 4.0 {
                let t = depth(y);
                let run = (6.0 + 70.0 * t) * rng.range(0.5, 1.2);
                edge.tool.width = 0.6 + 4.5 * t;
                edge.reload(pile(&st, if side == 0 { "#7c7771" } else { "#6c6760" }, 0.8, 0.5), 0.45);
                let dir = if side == 0 { -1.0 } else { 1.0 };
                let bx = |yy: f32| if side == 0 { bank_l(yy) } else { bank_r(yy) };
                let pts: Vec<(f32, f32)> = (0..5).map(|j| {
                    let yy = y + run * j as f32 / 4.0;
                    (bx(yy) + dir * (0.3 + 1.2 * depth(yy)) * rng.range(-0.3, 1.0), yy)
                }).collect();
                if rng.f() < 0.35 { y += run; continue; }
                c.drag(&mut edge, &Gesture::new(pts).pressure(rng.range(0.25, 0.7), rng.range(0.1, 0.6)).ramps(0.15, 0.3).swell(vec![1.0, rng.range(0.6, 1.2), 0.9]), None);
                y += run * rng.range(0.8, 1.15);
            }
        }
        c.dry();
        // snow dragged back over the dark lip from the field side, so the
        // edge is lost and found
        let mut sn = Held::new(Tool { lay: 0.8, ..Tool::filbert(5.0) }, 61);
        for side in [0usize, 1] {
            let mut y = yh + 6.0;
            while y < h + 4.0 {
                let t = depth(y);
                sn.tool.width = 1.5 + 9.0 * t;
                sn.reload(pile(&st, "#d6d2cb", 0.9, 0.8), 0.6);
                let dir = if side == 0 { -1.0 } else { 1.0 };
                let x = if side == 0 { bank_l(y) } else { bank_r(y) } + dir * (1.0 + 4.0 * t);
                let l = (4.0 + 26.0 * t) * rng.range(0.6, 1.3);
                if rng.f() < 0.6 {
                    c.drag(&mut sn, &Gesture::new(vec![(x + dir * l, y + 1.0 + 3.0 * t), (x + dir * l * 0.4, y + 0.3), (x - dir * l * 0.08 * rng.f(), y)]).pressure(rng.range(0.4, 0.8), 0.2).ramps(0.1, 0.5), None);
                }
                y += (2.0 + 16.0 * t) * rng.range(0.7, 1.4);
            }
        }
        c.dry();
        c.dry();
    }

    if o.stage("boat", &mut c, &mut rng) {
        // a flat river boat frozen in at the left bank, bow toward us,
        // snow lying in it, and its mooring post
        let bt = 0.44f32;
        let by = yh + bt * (h - yh);
        let l = 50.0 * bt; // half length on the picture plane
        let bx = bank_l(by) + l * 0.7;
        let d = 13.0 * bt; // depth of the side
        let mut hull = Held::new(Tool { point: 0.5, ..Tool::round_sable(2.2 * bt + 0.8) }, 75);
        // the dark side of the hull: strokes along it, the stern end rising
        for k in 0..5 {
            let v = k as f32 / 4.0;
            hull.reload(pile(&st, if k < 2 { "#3b332c" } else { "#2c2723" }, 0.92, 0.7), 0.7);
            let yy = by - d + d * v;
            c.drag(&mut hull, &Gesture::new(vec![(bx - l, yy - 2.0 * bt * (1.0 - v)), (bx, yy + 0.5 * v), (bx + l * (0.95 - 0.1 * v), yy - 3.0 * bt * (1.0 - v))]).pressure(0.8, 0.7).ramps(0.1, 0.2).shake(0.6), None);
        }
        // the gunwale catching the light, then the snow heaped inside
        hull.reload(pile(&st, "#7a6d60", 0.9, 0.7), 0.6);
        c.drag(&mut hull, &Gesture::new(vec![(bx - l, by - d - 2.3 * bt), (bx, by - d), (bx + l * 0.95, by - d - 3.0 * bt)]).pressure(0.5, 0.45).ramps(0.1, 0.2), None);
        let mut sn = Held::new(Tool { lay: 0.9, ..Tool::filbert(3.0 * bt + 1.0) }, 76);
        for k in 0..4 {
            sn.reload(pile(&st, if k % 2 == 0 { "#dcd6cc" } else { "#c9c6c6" }, 0.95, 0.9), 0.7);
            let yy = by - d - (1.5 + k as f32) * bt;
            c.drag(&mut sn, &Gesture::new(vec![(bx - l * 0.85, yy), (bx - l * 0.1, yy - 1.5 * bt), (bx + l * 0.8, yy - 0.5 * bt)]).pressure(0.6, 0.5).ramps(0.2, 0.3), None);
        }
        // snow drifted against the hull, burying its foot
        for k in 0..3 {
            sn.reload(pile(&st, "#d0cdca", 0.92, 0.85), 0.6);
            let x0 = bx - l + k as f32 * l * 0.8;
            c.drag(&mut sn, &Gesture::new(vec![(x0 - l * 0.3, by + 1.5 * bt), (x0, by - 0.5 * bt), (x0 + l * 0.4, by + 1.0 * bt)]).pressure(0.7, 0.3).ramps(0.1, 0.5), None);
        }
        // the post, leaning a little, a cap of snow
        let mut post = Held::new(Tool { point: 0.5, ..Tool::round_sable(3.0 * bt + 0.8) }, 77);
        post.load(pile(&st, "#2e2823", 0.92, 0.7), 0.8);
        let px = bx - l - 8.0 * bt;
        c.drag(&mut post, &Gesture::new(vec![(px, by + 1.0), (px - 1.0 * bt, by - 30.0 * bt)]).pressure(0.9, 0.8).ramps(0.05, 0.05).shake(0.5), None);
        sn.reload(pile(&st, "#e0dbd2", 0.95, 0.9), 0.6);
        c.touch(&mut sn, &Touch::at(px - 1.0 * bt, by - 30.5 * bt).pressure(0.6).drag(1.5 * bt, 0.0), None);
        // the rope down to the bow, a sagging hairline
        let mut rp = Held::new(Tool { point: 1.0, ..Tool::rigger(0.5) }, 78);
        rp.load(pile(&st, "#4a4038", 0.8, 0.5), 0.5);
        c.drag(&mut rp, &Gesture::new(vec![(px - 0.6 * bt, by - 24.0 * bt), (px + 5.0 * bt, by - 12.0 * bt), (bx - l + 2.0, by - d - 1.0)]).pressure(0.35, 0.3).orient(Orient::Along), None);
        c.dry();
    }

    if o.stage("shadows", &mut c, &mut rng) {
        // the low sun throws the willows' shadows long across the snow,
        // away from the light: along the line from the sun's point on the
        // horizon through each foot, toward us. Laid as a thin cool glaze
        // (smalt and a little black in much medium) over the dry snow, the
        // way a shadow on snow is only the snow, darker and bluer. The
        // sledge ruts along the path go in with the same glaze.
        let sn = Fbm::new(seed + 71, 3, 30.0);
        let shapes: Vec<(f32, f32, f32, f32, f32)> = willows
            .iter()
            .map(|&(x, y, ht)| {
                let (dx, dy) = (x - sx, y - yh);
                let n = (dx * dx + dy * dy).sqrt();
                (x, y, dx / n, dy / n, ht)
            })
            .collect();
        let shade = Mask::from_fn(f, |px, py| {
            let mut m = 0.0f32;
            for &(x, y, ux, uy, ht) in &shapes {
                let t = depth(y);
                let (rx, ry) = (px - x, py - y);
                let along = rx * ux + ry * uy;
                let across = (-rx * uy + ry * ux).abs();
                let len = ht * 0.95;
                if along < -2.0 || along > len * 1.2 {
                    continue;
                }
                // the trunk's shadow, then the crown's thinner, broken one
                let u = along / len;
                let half = (1.0 + 7.0 * t) * (1.0 + 0.6 * smoothstep(0.35, 0.7, u)) * (1.0 + 0.4 * sn.get(px, py));
                let fade = 1.0 - smoothstep(0.75, 1.15, u);
                let crown = if u > 0.4 { 0.55 + 0.35 * sn.get(px * 2.0, py * 2.0) } else { 1.0 };
                m = m.max((1.0 - smoothstep(half * 0.6, half * 1.2, across)) * fade * crown * smoothstep(-2.0, 0.5, along));
            }
            // the sledge ruts: two lines along the path, lost in places
            let t = depth(py);
            if t > 0.02 {
                for side in [-1.0f32, 1.0] {
                    let cx = path_x(t) + side * 11.0 * t + 1.5 * t * sn.get(py * 0.5, side);
                    let d = (px - cx).abs();
                    let lost = smoothstep(0.0, 0.25, sn.get(py * 0.4, 3.0 + side) + 0.15 * sn.get(py * 2.0, side));
                    m = m.max(0.45 * (1.0 - smoothstep(0.2 + 0.8 * t, 0.5 + 1.8 * t, d)) * lost);
                }
            }
            m * snow_fields.sample(px, py)
        });
        c.work(&shade, &st.glaze(0.85).color(|_, _| hex("#76809a")).angle(move |x, y| (y - yh).atan2(x - sx)).length(20.0, 80.0).coverage(3.0).load_at(|_, _| 0.6).clip(true), o.seed * 100 + 70);
        c.dry();
    }

    if o.stage("willows", &mut c, &mut rng) {
        for (i, &(x, y, ht)) in willows.iter().enumerate() {
            willow(&mut c, &st, &mut rng, x, y, ht, sx, i as u64);
        }
        c.dry();
    }

    if o.stage("details", &mut c, &mut rng) {
        // dry grass and reeds pushing through the snow, flicked up last with
        // the pointed rigger, "fine upturning strokes": thickest in the near
        // right corner and along the banks, few and tiny far off
        let mut rig = Held::new(Tool { point: 1.0, ..Tool::rigger(0.8) }, 80);
        let grasses = ["#6d5a44", "#7b6750", "#5a4a3a", "#8a7658", "#4b4036"];
        let mut spots: Vec<(f32, f32)> = Vec::new();
        for _ in 0..55 {
            // clumps along the right bank
            let y = yh + (h - yh) * rng.range(0.05, 1.0f32).powf(0.8);
            spots.push((bank_r(y) + rng.range(-2.0, 25.0) * (0.2 + depth(y)), y));
        }
        for _ in 0..60 {
            // and the left bank's reeds
            let y = yh + (h - yh) * rng.range(0.05, 1.0f32).powf(0.8);
            spots.push((bank_l(y) - rng.range(-2.0, 30.0) * (0.2 + depth(y)), y));
        }
        for _ in 0..34 {
            // and the near right corner, and the path's verges
            spots.push((rng.range(620.0, w + 10.0), rng.range(h * 0.84, h + 5.0)));
        }
        for _ in 0..50 {
            let t = rng.range(0.05, 1.0f32).powf(0.7);
            let side = if rng.f() < 0.5 { -1.0 } else { 1.0 };
            spots.push((path_x(t) + side * (18.0 + rng.range(0.0, 30.0)) * t, yh + t * (h - yh)));
        }
        spots.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        for (gx, gy) in spots {
            if ice.sample(gx, gy) > 0.5 {
                continue;
            }
            let t = depth(gy);
            rig.tool.width = 0.35 + 1.0 * t;
            rig.reload(pile(&st, grasses[((rng.f() * grasses.len() as f32) as usize).min(grasses.len() - 1)], 0.85, 0.6), 0.6);
            let n = (3.0 + 16.0 * t * rng.f() * rng.f()) as u32 + 1;
            for _ in 0..n {
                let x = gx + rng.normal() * 3.0 * (0.3 + 1.5 * t);
                let hg = rng.range(1.5, 8.0) * rng.range(0.5, 1.2) * (0.25 + 2.2 * t);
                let lean = rng.range(-0.35, 0.35) * hg + 0.1 * hg;
                let pts = vec![(x, gy + 0.5), (x + lean * 0.3, gy - hg * 0.5), (x + lean, gy - hg)];
                c.drag(&mut rig, &Gesture::new(pts).pressure(rng.range(0.5, 0.9), 0.05).ramps(0.03, 0.7).orient(Orient::Along), None);
            }
        }
        // crows: a few in the air over the far fields, dark gestures, and a
        // pair sitting on the head of the nearest willow
        let mut cr = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.6) }, 81);
        cr.load(pile(&st, "#2a2624", 0.95, 0.7), 0.9);
        for &(bx, by, sz) in &[(430.0f32, 250.0f32, 5.5f32), (468.0, 232.0, 4.5), (395.0, 282.0, 4.0), (520.0, 300.0, 3.2)] {
            let flap = rng.range(-0.4, 0.4);
            c.drag(&mut cr, &Gesture::new(vec![(bx - sz, by - sz * (0.35 + flap)), (bx - sz * 0.4, by - sz * 0.1), (bx, by)]).pressure(0.2, 0.7).ramps(0.3, 0.1), None);
            c.drag(&mut cr, &Gesture::new(vec![(bx, by), (bx + sz * 0.4, by - sz * 0.15), (bx + sz, by - sz * (0.4 - flap))]).pressure(0.7, 0.2).ramps(0.1, 0.3), None);
        }
        c.dry();
    }

    if o.stage("figure", &mut c, &mut rng) {
        man_from_behind(&mut c, &st, &mut rng, fx, fy, fh);
        c.dry();
    }

    o.finish(&mut c, &mut rng, &Finish::aged(st.relief));
}

/// A pollard willow at (x, y) (its foot), `ht` units tall to the tips of
/// its rods, lit from the left (the sun at x = `sun_x`). Painted as a
/// painter would: the squat trunk in a few vertical strokes, the swollen
/// head in dabs, the rods pulled up out of the head with a pointed brush,
/// snow on the head's upper side, then snow dragged over the foot.
fn willow(c: &mut paint::Canvas, st: &Style, rng: &mut Rng, x: f32, y: f32, ht: f32, sun_x: f32, id: u64) {
    let s = ht / 100.0; // scale: 1 at a 100-unit tree
    let trunk_h = ht * rng.range(0.34, 0.42);
    let tw = ht * rng.range(0.09, 0.12);
    let lean = rng.range(-0.08, 0.08);
    let kink = rng.range(-0.25, 0.25);
    let head = (x + lean * trunk_h, y - trunk_h);
    // far willows are paler and bluer: the air between
    let far = 1.0 - (ht / 220.0).clamp(0.0, 1.0);
    let bark_dark = gradient(&[(0.0, hex("#2d2620")), (1.0, hex("#7c7880"))], far.powf(1.3), Mix::Pigment);
    let bark_lit = gradient(&[(0.0, hex("#6b5b4a")), (1.0, hex("#8e8a90"))], far.powf(1.3), Mix::Pigment);
    let rod_col = gradient(&[(0.0, hex("#443830")), (1.0, hex("#8a8690"))], far.powf(1.1), Mix::Pigment);
    let lit_side = if sun_x < x { -1.0 } else { 1.0 };

    // the trunk: side by side strokes pulled up from the foot, a little
    // curved, the gnarled shaft swelling into the head
    let mut br = Held::new(Tool { point: 0.6, ..Tool::round_sable((tw * 0.35).max(0.8)) }, 900 + id * 17);
    let strokes = (4.0 + 6.0 * s.min(1.5)) as usize;
    for k in 0..strokes {
        let u = (k as f32 + 0.5) / strokes as f32 * 2.0 - 1.0;
        let lit = (u * lit_side).max(0.0);
        let col = gradient(&[(0.0, bark_dark), (1.0, bark_lit)], lit.powf(1.5) * 0.9, Mix::Pigment);
        br.reload(Paint::new(st.palette.mix(col).color, 0.9, 0.6), 0.7);
        // the shaft's girth up the trunk: a flared foot, a waist, the
        // swollen head where it has been cut back year after year
        let prof = [1.25, 0.95, 0.88, 1.0, 1.35];
        let wob = rng.range(-0.06, 0.06) * tw;
        let pts: Vec<(f32, f32)> = (0..5).map(|j| {
            let v = j as f32 / 4.0;
            let cx = x + lean * trunk_h * v + kink * (std::f32::consts::PI * v).sin() * tw;
            (cx + u * tw * 0.5 * prof[j] + wob, y + 1.5 * s - (trunk_h + 1.5 * s) * v)
        }).collect();
        c.drag(&mut br, &Gesture::new(pts).pressure(rng.range(0.75, 0.95), rng.range(0.7, 0.95)).ramps(0.05, 0.1), None);
    }
    // the head: a swollen, knuckled crown, dabbed and turned with the brush
    let mut dab = Held::new(Tool::round_sable((tw * 0.3).max(0.7)), 901 + id * 17);
    let knobs = (6.0 + 10.0 * s.min(1.5)) as usize;
    for _ in 0..knobs {
        let a = rng.range(-1.0, 1.0);
        let px = head.0 + a * tw * 0.8;
        let py = head.1 - rng.range(-0.15, 0.3) * tw;
        let lit = (a * lit_side).max(0.0);
        let col = gradient(&[(0.0, bark_dark), (1.0, bark_lit)], lit * 0.8, Mix::Pigment);
        dab.reload(Paint::new(st.palette.mix(col).color, 0.9, 0.7), 0.6);
        c.touch(&mut dab, &Touch::at(px, py).pressure(rng.range(0.6, 1.0)).drag(rng.range(-0.3, 0.3) * tw, -rng.range(0.1, 0.35) * tw), None);
    }
    // the bark: short, broken vertical strokes, darker fissures and a few
    // lights on the side toward the sun, following the shaft
    if ht > 40.0 {
        let mut bk = Held::new(Tool { point: 0.9, ..Tool::round_sable((tw * 0.07).max(0.4)) }, 905 + id * 17);
        for _ in 0..(12.0 * s) as usize {
            let u = rng.range(-0.9, 0.9);
            let v0 = rng.range(0.05, 0.85);
            let lit = (u * lit_side) > 0.35 && rng.f() < 0.5;
            let col = if lit { gradient(&[(0.0, bark_lit), (1.0, hex("#8f8272"))], 0.4, Mix::Pigment) } else { gradient(&[(0.0, bark_dark), (1.0, hex("#15120f"))], 0.5, Mix::Pigment) };
            bk.reload(Paint::new(st.palette.mix(col).color, 0.9, 0.6), 0.5);
            let at = |v: f32| (x + lean * trunk_h * v + kink * (std::f32::consts::PI * v).sin() * tw + u * tw * 0.45, y - trunk_h * v);
            let v1 = v0 + rng.range(0.06, 0.2);
            let (a, b) = (at(v0), at(v1));
            c.drag(&mut bk, &Gesture::new(vec![a, ((a.0 + b.0) * 0.5 + rng.range(-0.3, 0.3) * s, (a.1 + b.1) * 0.5), b]).pressure(rng.range(0.4, 0.8), 0.1).ramps(0.1, 0.5).orient(Orient::Along), None);
        }
    }
    // the rods: last year's shoots, straight and whippy, springing up and
    // out from the head in a fan; many, uneven, crossing, some bent; far
    // willows get fewer, softer ones (they read as a haze)
    let n = (14.0 + 110.0 * s.min(1.6)) as usize;
    let mut rod = Held::new(Tool { point: 1.0, ..Tool::rigger((0.22 + 0.3 * s).min(0.8)) }, 902 + id * 17);
    let rod_h = ht - trunk_h;
    for k in 0..n {
        if k % 6 == 0 {
            let tint = rng.range(-0.15, 0.15);
            let col = gradient(&[(0.0, rod_col), (1.0, if tint > 0.0 { hex("#76593f") } else { hex("#2e2824") })], tint.abs() * 2.0 * (1.0 - far), Mix::Pigment);
            rod.reload(Paint::new(st.palette.mix(col).color, 0.8, 0.5), 0.55);
        }
        // the fan: steep in the middle, spreading at the sides
        let spread = rng.normal() * 0.42;
        let a = -std::f32::consts::FRAC_PI_2 + spread + lean;
        let len = rod_h * rng.range(0.45, 1.0) * (1.0 - 0.35 * spread.abs());
        let root = (head.0 + rng.range(-0.8, 0.8) * tw, head.1 + rng.range(-0.1, 0.25) * tw);
        let bend = rng.range(-0.12, 0.12) + 0.1 * spread; // arching outward
        let pts: Vec<(f32, f32)> = (0..5).map(|j| {
            let u = j as f32 / 4.0;
            let aa = a + bend * u * u * 2.0;
            (root.0 + len * u * aa.cos(), root.1 + len * u * aa.sin())
        }).collect();
        let p0 = rng.range(0.55, 0.9) * (0.5 + 0.5 * (1.0 - far));
        c.drag(&mut rod, &Gesture::new(pts).pressure(p0, 0.0).ramps(0.02, rng.range(0.4, 0.8)).orient(Orient::Along), None);
    }
    // snow lying on the crown's upper side, catching the light
    let mut sn = Held::new(Tool { point: 0.8, ..Tool::round_sable((tw * 0.12).max(0.5)) }, 903 + id * 17);
    for _ in 0..(3.0 + 5.0 * s.min(1.5)) as usize {
        sn.reload(pile(st, "#dcd8d0", 0.9, 0.9), 0.45);
        let a = rng.range(-0.9, 0.9);
        let px = head.0 + a * tw * 0.8;
        let py = head.1 - tw * rng.range(0.2, 0.4);
        c.touch(&mut sn, &Touch::at(px, py).pressure(rng.range(0.3, 0.6)).drag(rng.range(0.15, 0.4) * tw, rng.range(-0.05, 0.05) * tw), None);
    }
    // the foot buried: snow dragged across the base of the trunk from the
    // side, a drift heaped against it
    let mut dr = Held::new(Tool { lay: 0.9, ..Tool::filbert((tw * 0.7).max(1.2)) }, 904 + id * 17);
    for k in 0..4 {
        dr.reload(pile(st, if k % 2 == 0 { "#d2cec8" } else { "#c2c1c6" }, 0.92, 0.85), 0.5);
        let yy = y + rng.range(-0.4, 1.0) * s * 2.0;
        let side = if k % 2 == 0 { -1.0 } else { 1.0 };
        let x0 = x + side * tw * rng.range(0.6, 1.0);
        c.drag(&mut dr, &Gesture::new(vec![(x0, yy + 1.0 * s), (x + side * tw * 0.2, yy - s * 1.5), (x - side * tw * rng.range(0.0, 0.3), yy - s * 0.6)]).pressure(0.8, 0.3).ramps(0.1, 0.5), None);
    }
}

/// A man seen from behind, standing on the bank: a dark greatcoat to the
/// calf, a hat, a stick. `(x, y)` is where his feet meet the snow, `ht` his
/// height to the crown of the hat.
fn man_from_behind(c: &mut paint::Canvas, st: &Style, rng: &mut Rng, x: f32, y: f32, ht: f32) {
    let u = ht / 100.0;
    let coat = pile(st, "#26282b", 0.95, 0.8);
    let coat_lit = pile(st, "#4a4a4c", 0.9, 0.8);
    let hat = pile(st, "#1c1b1a", 0.95, 0.8);
    let boots = pile(st, "#1f1b18", 0.95, 0.8);
    // legs and boots below the coat
    let mut b = Held::new(Tool::round_sable(4.5 * u), 91);
    b.load(boots, 0.8);
    for dx in [-4.0f32, 4.5] {
        c.drag(&mut b, &Gesture::new(vec![(x + dx * u, y - 16.0 * u), (x + dx * u * 1.05, y)]).pressure(0.9, 0.9).ramps(0.05, 0.05).shake(0.5), None);
    }
    // the coat: strokes down from the shoulders to the hem, the body widening
    // as the skirt of the coat falls; lit a little on its left (sun side)
    let mut fb = Held::new(Tool { lay: 0.9, ..Tool::filbert(8.0 * u) }, 92);
    let sh_y = y - 80.0 * u;
    let hem_y = y - 13.0 * u;
    // the back in five overlapping strokes, the skirt flaring below the
    // waist; the sun, low ahead on the left, only rims his left side
    for k in 0..5 {
        let v = (k as f32 + 0.5) / 5.0 * 2.0 - 1.0;
        fb.reload(coat, 0.85);
        let top = (x + v * 6.5 * u, sh_y + v.abs().powi(2) * 4.0 * u);
        let mid = (x + v * 8.5 * u + rng.range(-0.3, 0.3) * u, y - 50.0 * u);
        let bot = (x + v * 17.0 * u + rng.range(-0.6, 0.6) * u, hem_y + rng.range(-1.0, 1.0) * u - v * 1.5 * u);
        c.drag(&mut fb, &Gesture::new(vec![top, mid, bot]).pressure(0.9, 0.95).ramps(0.05, 0.1).shake(0.5), None);
    }
    // the arms hanging at his sides, the left one catching a little light
    let mut arm = Held::new(Tool { lay: 0.9, ..Tool::filbert(4.0 * u) }, 98);
    for side in [-1.0f32, 1.0] {
        arm.reload(if side < 0.0 { coat_lit } else { coat }, 0.8);
        c.drag(&mut arm, &Gesture::new(vec![(x + side * 9.0 * u, sh_y + 3.0 * u), (x + side * 11.5 * u, y - 55.0 * u), (x + side * 11.0 * u, y - 40.0 * u)]).pressure(0.8, 0.7).ramps(0.05, 0.2).shake(0.5), None);
    }
    // a fold or two down the back and the belt line, a shade darker
    let mut dk = Held::new(Tool { point: 1.0, ..Tool::round_sable(2.2 * u) }, 93);
    dk.load(hat, 0.6);
    c.drag(&mut dk, &Gesture::new(vec![(x + 1.0 * u, y - 58.0 * u), (x + 2.0 * u, y - 35.0 * u), (x + 3.5 * u, hem_y + 1.0 * u)]).pressure(0.5, 0.2).ramps(0.2, 0.5), None);
    c.drag(&mut dk, &Gesture::new(vec![(x - 12.0 * u, y - 52.0 * u), (x + 12.0 * u, y - 51.0 * u)]).pressure(0.35, 0.35).ramps(0.3, 0.3), None);
    // collar and head: the turned-up collar, the back of the head, the hat
    let mut hd = Held::new(Tool::round_sable(7.0 * u), 94);
    hd.load(pile(st, "#34302c", 0.95, 0.8), 0.8);
    c.touch(&mut hd, &Touch::at(x, sh_y - 3.0 * u).pressure(0.9).drag(0.0, -5.0 * u), None);
    hd.reload(pile(st, "#6e5a4a", 0.95, 0.8), 0.6);
    c.touch(&mut hd, &Touch::at(x, sh_y - 8.0 * u).pressure(0.7).drag(0.0, -2.0 * u), None);
    let mut hb = Held::new(Tool::round_sable(4.0 * u), 95);
    hb.load(hat, 0.9);
    // brim, then the crown of a tall hat
    c.drag(&mut hb, &Gesture::new(vec![(x - 7.5 * u, sh_y - 12.5 * u), (x, sh_y - 13.5 * u), (x + 7.5 * u, sh_y - 12.0 * u)]).pressure(0.6, 0.6).ramps(0.1, 0.1).orient(Orient::Along), None);
    for dx in [-3.0f32, 0.0, 3.0] {
        c.drag(&mut hb, &Gesture::new(vec![(x + dx * u, sh_y - 12.5 * u), (x + dx * 0.95 * u, sh_y - 20.0 * u)]).pressure(0.9, 0.9).ramps(0.0, 0.0), None);
    }
    // the stick, held out to the right, its foot in the snow
    let mut st_b = Held::new(Tool { point: 1.0, ..Tool::rigger(1.2 * u) }, 96);
    st_b.load(pile(st, "#3a2f26", 0.9, 0.6), 0.8);
    c.drag(&mut st_b, &Gesture::new(vec![(x + 11.5 * u, y - 40.0 * u), (x + 15.0 * u, y - 20.0 * u), (x + 18.0 * u, y + 1.0 * u)]).pressure(0.8, 0.7).ramps(0.05, 0.1).orient(Orient::Along), None);
    // snow over his feet and the stick's foot
    let mut sn = Held::new(Tool { lay: 0.9, ..Tool::filbert(6.0 * u) }, 97);
    for k in 0..3 {
        sn.reload(pile(st, if k == 1 { "#c3c2c6" } else { "#d6d2ca" }, 0.92, 0.85), 0.7);
        let yy = y + rng.range(-1.0, 1.5) * u;
        c.drag(&mut sn, &Gesture::new(vec![(x - 16.0 * u + k as f32 * 12.0 * u, yy + 1.0 * u), (x - 6.0 * u + k as f32 * 12.0 * u, yy - 1.0 * u), (x + 4.0 * u + k as f32 * 12.0 * u, yy)]).pressure(0.7, 0.3).ramps(0.1, 0.5), None);
    }
}
