//! "Two Poplars at a Pond, Evening": an original picture in the manner of
//! Caspar David Friedrich, from knowledge only (notes/round7/arm2/notes.md).
//!
//! The order: a level horizon, most of the canvas given to the air; on the
//! axis two Lombardy poplars on a low far bank, one whole, one with its top
//! broken off; the still pond repeats them and the sky; a near shore of reeds
//! that rises a little to both corners (a curve opening upward) and stops the
//! eye. The sun has gone down behind the trees; a young moon, veiled.
//!
//!   cargo paint pond_poplars                  1000px
//!   cargo paint pond_poplars -- --full        3200px

use paint::color::{Mix, mix};
use paint::graphite::{Lead, Mark as Line};
use paint::{Fbm, Gesture, Held, Mask, Palette, Rgb, Rng, Shape, Stipple, Style, Tool, gradient, hex, smoothstep};
use paintings::run::{Finish, Run};
use std::f32::consts::{FRAC_PI_2, PI};

const ASPECT: f32 = 1.4;
/// The top of the far bank against the sky: ruled level.
const HZ: f32 = 436.0;
/// The water's edge under the far bank: the mirror's hinge.
const WL: f32 = 452.0;

fn main() {
    let o = Run::new("pond_poplars");
    let mut rng = Rng::new(o.seed);
    let st = Style::friedrich();
    let pal = &st.palette;
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let (w, h) = (c.width(), c.height());

    // ------------------------------------------------------------ the world
    // the sky: after sunset, clear, deep blue-gray above, the glow low and
    // a little right of the trees where the sun went down
    let glow_x = 590.0f32;
    let sky_col = move |x: f32, y: f32| -> Rgb {
        let g = (-((x - glow_x) / 420.0).powi(2)).exp();
        let t = (y / HZ).clamp(0.0, 1.0);
        // the glow lifts the warm bands higher over the sun's place
        let t = (t + 0.07 * g * t).min(1.0);
        gradient(
            &[
                (0.0, hex("#3a4258")),
                (0.18, hex("#4c566e")),
                (0.38, hex("#737a8c")),
                (0.56, hex("#a09c9f")),
                (0.72, hex("#c5b49d")),
                (0.86, hex("#dcc59c")),
                (0.95, hex("#e8d4a6")),
                (1.0, hex("#ecdaae")),
            ],
            t,
            Mix::Pigment,
        )
    };
    // the far bank: a low meadow ruled level at the top, a few low bushes
    let bank_n = Fbm::new(21, 4, 60.0);
    let bank_top = f.per_column(move |x| {
        let bush = |cx: f32, r: f32, ht: f32| ht * (1.0 - ((x - cx) / r).powi(2)).max(0.0).sqrt();
        let b = bush(96.0, 38.0, 9.0).max(bush(150.0, 22.0, 6.0)).max(bush(842.0, 30.0, 8.0)).max(bush(905.0, 44.0, 11.0)).max(bush(962.0, 18.0, 5.0));
        HZ - b * (0.8 + 0.4 * bank_n.get01(x * 3.0, 1.0)) + 0.6 * bank_n.get(x, 0.0)
    });
    let water_edge = f.per_column(move |x| WL + 1.2 * bank_n.get(x * 2.0, 7.0));
    // the near shore: dark, rising to both corners
    let shore_n = Fbm::new(33, 4, 80.0);
    let shore = f.per_column(move |x| {
        let u = (x - 500.0) / 500.0;
        660.0 - 34.0 * u * u - 12.0 * smoothstep(0.55, 1.0, u.abs()) + 6.0 * shore_n.get(x, 0.0) + 2.0 * shore_n.get(x * 5.0, 3.0)
    });
    // a far range, just showing over the bank to the left, low and pale
    let hill_n = Fbm::new(41, 4, 150.0);
    let hills = f.per_column(move |x| HZ + 3.0 - 24.0 * smoothstep(440.0, 70.0, x) - 13.0 * smoothstep(690.0, 990.0, x) + (4.0 * hill_n.get(x, 0.0) + 1.2 * hill_n.get(x * 4.0, 2.0)) * (smoothstep(440.0, 300.0, x) + smoothstep(690.0, 820.0, x)));

    let sky_m = Mask::from_fn(f, |x, y| 1.0 - smoothstep(bank_top(x) + 3.0, bank_top(x) + 6.0, y));
    let hills_m = Mask::from_fn(f, |x, y| smoothstep(hills(x) - 0.7, hills(x) + 0.7, y) * (1.0 - smoothstep(HZ + 2.0, HZ + 4.0, y)));
    let bank_m = Mask::from_fn(f, |x, y| smoothstep(bank_top(x) - 0.7, bank_top(x) + 0.7, y) * (1.0 - smoothstep(water_edge(x) - 0.6, water_edge(x) + 0.6, y)));
    let water_m = Mask::from_fn(f, |x, y| smoothstep(water_edge(x) - 0.6, water_edge(x) + 0.6, y) * (1.0 - smoothstep(shore(x) + 4.0, shore(x) + 7.0, y)));
    let shore_m = Mask::from_fn(f, |x, y| smoothstep(shore(x) - 0.8, shore(x) + 0.8, y));

    // the water: the sky mirrored about the hinge, seen at a slant (so the
    // low sky, stretched), darkening toward the viewer
    let water_col = move |x: f32, y: f32| -> Rgb {
        let d = (y - WL).max(0.0);
        let refl = sky_col(x, HZ - 6.0 - d * 1.9);
        let near = smoothstep(WL, 700.0, y);
        mix(mix(refl, hex("#6d6a6a"), 0.18, Mix::Pigment), hex("#2e3239"), 0.1 + 0.6 * near * near, Mix::Pigment)
    };

    // the two poplars
    let tall = Poplar { x: 492.0, foot: 444.0, ht: 322.0, hw: 31.0, lean: -0.012, seed: 5, broken: false };
    let short = Poplar { x: 561.0, foot: 446.0, ht: 236.0, hw: 27.0, lean: 0.02, seed: 9, broken: true };
    let trees = [tall, short];
    let tree_m: Vec<Mask> = trees.iter().map(|p| Mask::from_shape(f, Shape::new().poly(&p.outline()))).collect();
    let trees_m = tree_m[0].clone().union(&tree_m[1]);

    // ------------------------------------------------------------ drawing
    if o.stage("drawing", &mut c, &mut rng) {
        // the search in a hard lead, then what I'm sure of restated in 3B;
        // the horizon ruled, the shore and the trees freehand
        let hb = Lead::pencil("2H").unwrap();
        let soft = Lead::pencil("3B").unwrap();
        let mut worn = 0.0;
        let mut line = |c: &mut paint::Canvas, lead: &Lead, pts: Vec<(f32, f32)>, p: f32, rng: &mut Rng| {
            let n = pts.len();
            let pressure: Vec<f32> = (0..n).map(|i| p * (0.75 + 0.35 * rng.f()) * if i == 0 || i + 1 == n { 0.5 } else { 1.0 }).collect();
            worn += c.draw(lead, &Line { pts, pressure }, worn, rng.next_u64());
        };
        // horizon: ruled, in two pulls with a small overlap
        line(&mut c, &hb, vec![(10.0, HZ + 0.3), (520.0, HZ)], 0.5, &mut rng);
        line(&mut c, &hb, vec![(500.0, HZ + 0.2), (992.0, HZ - 0.2)], 0.5, &mut rng);
        line(&mut c, &hb, (0..=40).map(|i| { let x = i as f32 * 25.0; (x, water_edge(x) + rng.normal() * 0.3) }).collect(), 0.45, &mut rng);
        line(&mut c, &hb, (0..=40).map(|i| { let x = i as f32 * 25.0; (x, shore(x) + rng.normal() * 0.6) }).collect(), 0.45, &mut rng);
        // the axis, faint (rubbed off later by the paint over it)
        line(&mut c, &hb, vec![(500.0, 40.0), (500.0, 690.0)], 0.25, &mut rng);
        for p in &trees {
            let o = p.outline();
            let half = o.len() / 2;
            line(&mut c, &hb, o[..half].to_vec(), 0.5, &mut rng);
            line(&mut c, &hb, o[half..].to_vec(), 0.5, &mut rng);
            // restated: the flanks in 3B, broken where unsure
            let mut k = 0;
            while k + 6 < o.len() {
                let len = 4 + (rng.f() * 8.0) as usize;
                let e = (k + len).min(o.len() - 1);
                if rng.f() < 0.7 {
                    line(&mut c, &soft, o[k..=e].to_vec(), 0.55, &mut rng);
                }
                k = e + (rng.f() * 3.0) as usize;
            }
            // the reflection's axis and length
            line(&mut c, &hb, vec![(p.x, WL + 2.0), (p.x + rng.normal(), (WL + (WL - p.foot) + p.ht * 0.9).min(h - 30.0))], 0.3, &mut rng);
        }
        // the moon's place
        let (mx, my) = MOON;
        line(&mut c, &hb, (0..=10).map(|i| { let a = 0.6 + i as f32 * 0.28; (mx + 9.0 * a.cos(), my + 9.0 * a.sin()) }).collect(), 0.35, &mut rng);
    }

    // ------------------------------------------------------------ lay-in
    if o.stage("lay-in", &mut c, &mut rng) {
        // a thin brown wash for the values, in one color: lean in the sky
        // (a touch only, low), heavier on the trees, bank and near shore;
        // the water a middle value, darker toward me
        let whole = Mask::full(f);
        let tm = &trees_m;
        let depth = move |x: f32, y: f32| {
            let land = smoothstep(HZ - 4.0, HZ + 4.0, y) * (1.0 - smoothstep(WL - 2.0, WL + 3.0, y));
            let water = smoothstep(WL, h, y);
            let near = smoothstep(620.0, 690.0, y);
            0.12 + 0.2 * smoothstep(200.0, HZ, y) + 1.4 * land + 0.8 * water + 1.2 * near + 1.6 * tm.sample(x, y)
        };
        c.work(&whole, &st.glaze(0.88).color(|_, _| hex("#5e4430")).angle(|_, _| 0.0).load_at(depth), 101);
        c.dry();
        // the trees' dark in the same brown, a brush along their growth
        let tree_under = st.body().color(|_, _| hex("#3d2d22")).angle(|_, _| -FRAC_PI_2).angle_jitter(0.2).length(10.0, 30.0).coverage(2.0).medium(0.55).load(0.5).clip(true);
        c.work(&trees_m.clone().erode(7.0), &tree_under, 102);
        c.dry();
    }

    // ------------------------------------------------------------ sky
    if o.stage("sky", &mut c, &mut rng) {
        // first layer: lean, long level arcs, a shade duller than the end,
        // fused with the badger while open
        let spal = pal.only(&["lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth", "bone black"]);
        let lay = st
            .broad()
            .mixed(&spal, 0.35)
            .color(move |x, y| mix(sky_col(x, y), hex("#8a8584"), 0.08, Mix::Pigment))
            .angle(|_, _| 0.0)
            .coverage(4.0)
            .medium(0.33);
        c.work(&sky_m, &lay, 11);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.angle(|_, _| 0.0), 12);
        }
        c.wait(4.0 * 60.0);
        // stippled over the set lay-in, aimed at the sky's own tones
        let s1 = Stipple::new(Tool::stippler(2.4)).mixed(&spal, 0.5).color(sky_col).coverage(|_, _| 2.4).pressure(0.4, 0.8).dips(18, 0.35, 0.6);
        c.stipple(&sky_m, &s1, 13);
        c.dry();
        // next day: a finer stipple to build the glow down to the bank,
        // barely lighter than the field
        let glow = move |x: f32, y: f32| mix(sky_col(x, y), hex("#f1e2b8"), 0.12 * smoothstep(250.0, HZ, y) * (-((x - glow_x) / 380.0).powi(2)).exp(), Mix::Light);
        let s2 = Stipple::new(Tool::stippler(1.5)).mixed(&spal, 0.55).color(glow).coverage(|_, y| 0.6 + 1.5 * smoothstep(180.0, HZ - 20.0, y)).pressure(0.4, 0.8).dips(22, 0.35, 0.6);
        c.stipple(&sky_m, &s2, 14);
        c.dry();
    }

    // ------------------------------------------------------------ clouds
    if o.stage("clouds", &mut c, &mut rng) {
        // three strands of evening cloud at different heights, none alike:
        // a long low one lying in the glow, a shorter thin one above it to
        // the right, a faint drawn-out streak high on the left
        let spal = pal.only(&["lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth", "bone black"]);
        let n = Fbm::new(61, 3, 220.0);
        // (y, thickness, x0, x1, strength, tint)
        let strands: [(f32, f32, f32, f32, f32); 4] = [(352.0, 9.0, -40.0, 690.0, 0.9), (316.0, 5.0, 520.0, 1040.0, 0.6), (380.0, 4.0, 700.0, 960.0, 0.45), (148.0, 7.0, -60.0, 420.0, 0.35)];
        let cov = move |x: f32, y: f32| -> f32 {
            let mut v = 0.0f32;
            for (k, &(cy, th, x0, x1, a)) in strands.iter().enumerate() {
                let kf = k as f32;
                // the strand sags and thickens irregularly along its length
                let wob = 5.0 * n.get(x * 0.5, kf * 40.0) + 0.012 * (x - x0) * (kf - 1.5);
                let thick = th * (0.4 + 1.1 * n.get01(x * 1.4, 90.0 + kf * 30.0));
                let d = (y - cy - wob) / thick;
                // flat-bottomed: the underside sharper than the top
                let across = if d > 0.0 { 1.0 - smoothstep(0.1, 1.2, d) } else { 1.0 - smoothstep(0.0, 1.8, -d) };
                let ends = smoothstep(x0, x0 + 120.0, x) * (1.0 - smoothstep(x1 - 160.0, x1, x));
                let breaks = smoothstep(0.28, 0.55, n.get01(x * 0.9, 200.0 + kf * 50.0));
                v = v.max(a * across * ends * breaks);
            }
            2.4 * v
        };
        let cloud_m = Mask::from_fn(f, |x, y| if cov(x, y) > 0.03 { 1.0 } else { 0.0 }).mul(&sky_m);
        // the strands against the glow are darker than it, mauve-gray,
        // their undersides warmed a little by the light under them
        let cc = move |x: f32, y: f32| {
            let s = sky_col(x, y);
            let low = smoothstep(200.0, 380.0, y);
            mix(s, mix(hex("#6f6674"), hex("#8e7e7c"), low, Mix::Pigment), 0.2 + 0.08 * low, Mix::Pigment)
        };
        let cl = Stipple::new(Tool::stippler(1.7)).mixed(&spal, 0.5).color(cc).coverage(move |x, y| 0.8 * cov(x, y)).pressure(0.4, 0.8).drag(2.2, Some(0.0)).dips(20, 0.35, 0.6);
        c.stipple(&cloud_m, &cl, 15);
        c.dry();
        // the lit lower lip of the long low strand: a few lean level touches
        let lip = pal.mix(hex("#e6cfa0")).paint(0.45).with_hiding(0.6);
        let mut b = Held::new(Tool::round_sable(1.6), 16);
        let mut x = 60.0;
        while x < 640.0 {
            let len = rng.range(18.0, 60.0);
            let y = 352.0 + 5.0 * n.get(x * 0.5, 0.0) + 5.5 + rng.normal() * 1.0;
            if cov(x, y - 3.0) > 0.5 && rng.f() < 0.7 {
                b.reload(lip, rng.range(0.25, 0.4));
                c.drag(&mut b, &Gesture::new(vec![(x, y), (x + len * 0.5, y + rng.normal() * 0.4), (x + len, y + rng.normal() * 0.5)]).pressure(rng.range(0.3, 0.5), 0.15).ramps(0.3, 0.5), Some(&sky_m));
            }
            x += len + rng.range(6.0, 40.0);
        }
        c.dry();
    }

    // ------------------------------------------------------------ moon
    if o.stage("moon", &mut c, &mut rng) {
        let (mx, my) = MOON;
        let mr = 8.0f32;
        // a young moon, lit from below right where the sun is; half lost in
        // the high streak of cloud
        let crescent = Mask::from_shape(f, Shape::new().circle(mx, my, mr)).subtract(&Mask::from_shape(f, Shape::new().circle(mx - 3.4, my - 2.6, mr * 0.95)));
        let paint = pal.mix(hex("#ece2c0")).paint(0.3).with_hiding(0.9);
        let mut b = Held::new(Tool::round_sable(1.2), 21);
        let mut k = 0;
        while k < 150 {
            let a = rng.range(-0.8, 2.4);
            let rr = mr * rng.range(0.55, 1.0);
            let p = (mx + rr * a.cos(), my + rr * a.sin());
            if crescent.sample(p.0, p.1) < 0.5 {
                continue;
            }
            if k % 12 == 0 {
                b.reload(paint, 0.4);
            }
            k += 1;
            let t = (-a.sin(), a.cos());
            c.drag(&mut b, &Gesture::new(vec![(p.0 - t.0 * 0.9, p.1 - t.1 * 0.9), (p.0 + t.0 * 0.9, p.1 + t.1 * 0.9)]).pressure(0.75, 0.6).ramps(0.1, 0.2), Some(&crescent));
        }
        c.dry();
        // veiled: the high cloud's tone scumbled lightly back over its lower horn
        let veil = Mask::from_fn(f, move |x, y| {
            let d = ((x - mx).powi(2) + (y - my).powi(2)).sqrt();
            (1.0 - smoothstep(mr * 0.6, mr * 2.2, d)) * smoothstep(my - 2.0, my + 5.0, y)
        });
        let vs = Stipple::new(Tool::stippler(1.3)).mixed(pal, 0.6).color(move |x, y| mix(sky_col(x, y), hex("#7c7582"), 0.25, Mix::Pigment)).coverage(|_, _| 1.0).pressure(0.35, 0.7).drag(1.6, Some(0.0)).dips(20, 0.3, 0.6).aim(false);
        c.stipple(&veil, &vs, 22);
        c.dry();
    }

    // ------------------------------------------------------------ distance
    if o.stage("hills", &mut c, &mut rng) {
        // barely a shape: a step darker and cooler than the glow, stippled
        let hc = move |x: f32, y: f32| {
            let t = smoothstep(hills(x), HZ, y);
            mix(mix(sky_col(x, y), hex("#7f7f93"), 0.4, Mix::Pigment), sky_col(x, HZ - 2.0), 0.35 * t, Mix::Pigment)
        };
        let hs = Stipple::new(Tool::stippler(1.6)).mixed(pal, 0.45).color(hc).coverage(|_, _| 2.3).pressure(0.45, 0.85).dips(20, 0.4, 0.6);
        c.stipple(&hills_m, &hs, 31);
        c.dry();
    }

    // ------------------------------------------------------------ water
    if o.stage("water", &mut c, &mut rng) {
        // whole level strokes, no flat brush: a round-ended filbert swung
        // along the water, in the water's own tones, then fused level
        let wpal = pal.only(&["lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth", "bone black"]);
        let lay = st.broad().mixed(&wpal, 0.35).color(water_col).angle(|_, _| 0.0).angle_jitter(0.01).curve(0.01, 0.05).drift(0.03, 400.0).broken(0.0).tail(0.0).coverage(4.2).medium(0.32);
        c.work(&water_m, &lay, 51);
        if let Some(b) = st.blend() {
            c.work(&water_m, &b.angle(|_, _| 0.0).cross(0.03), 52);
        }
        c.wait(5.0 * 60.0);
        // a veil of the same tones stippled over it once set, then laid
        // level with the badger while that is open
        let sv = Stipple::new(Tool::stippler(2.2)).mixed(&wpal, 0.5).color(water_col).coverage(|_, _| 2.4).pressure(0.4, 0.8).dips(18, 0.35, 0.6);
        c.stipple(&water_m, &sv, 53);
        if let Some(b) = st.blend() {
            c.work(&water_m, &b.angle(|_, _| 0.0).cross(0.004).length(80.0, 220.0).coverage(2.0), 54);
        }
        c.dry();
    }

    if o.stage("bank", &mut c, &mut rng) {
        // the meadow bank against the light: nearly one dark, olive-gray,
        // short level hatching; the bushes hatched upright
        let bc = move |x: f32, y: f32| {
            let t = smoothstep(bank_top(x), WL, y);
            mix(hex("#4a4a42"), hex("#34352f"), t, Mix::Pigment)
        };
        // its top first: level pulls of a small round along the line, the
        // way the eye reads it against the glow; then the body hatched in
        // below it, down under where the reflection will start
        let mut tb = Held::new(Tool::round_sable(1.7), 40);
        let mut x = rng.range(-20.0, -2.0);
        while x < 1005.0 {
            let l = rng.range(30.0, 80.0);
            tb.reload(pal.mix(bc(x, HZ + 1.0)).paint(0.25), rng.range(0.5, 0.7));
            let pts: Vec<(f32, f32)> = (0..=8).map(|i| { let xx = x + l * i as f32 / 8.0; (xx, bank_top(xx) + 0.9 + rng.normal() * 0.12) }).collect();
            c.drag(&mut tb, &Gesture::new(pts).pressure(rng.range(0.6, 0.85), 0.5).ramps(0.1, 0.2).shake(0.3), None);
            x += l * rng.range(0.8, 0.97);
        }
        let bank_body = Mask::from_fn(f, move |x, y| smoothstep(bank_top(x) + 1.0, bank_top(x) + 2.2, y) * (1.0 - smoothstep(water_edge(x) + 2.0, water_edge(x) + 3.0, y)));
        let hd = st.body().color(bc).angle(|_, _| 0.0).angle_jitter(0.08).length(18.0, 55.0).coverage(3.6).medium(0.22);
        c.work(&bank_body, &hd, 41);
        // the bushes: a small round, short strokes upward and out
        let bushes = Mask::from_fn(f, move |x, y| smoothstep(bank_top(x) - 0.5, bank_top(x) + 0.5, y) * (1.0 - smoothstep(HZ - 1.0, HZ + 1.5, y)) * smoothstep(HZ - 1.2, HZ - 2.5, bank_top(x)));
        let bh = st.hatch().color(|_, _| hex("#3e3f39")).angle(|_, _| -1.2).angle_jitter(0.5).length(2.5, 6.0).coverage(3.0).medium(0.25);
        c.work(&bushes, &bh, 42);
        c.dry();
        // the top of the bank: the glow catches the grass tips, a broken
        // lean line; and where the bank meets the water, a darker lip
        let tip = pal.mix(hex("#75715f")).paint(0.3).with_hiding(0.6);
        let lipc = pal.mix(hex("#2a2b27")).paint(0.25);
        let mut b = Held::new(Tool::round_sable(1.0), 43);
        let mut x = 2.0;
        while x < 998.0 {
            let len = rng.range(8.0, 40.0);
            if rng.f() < 0.55 {
                b.reload(tip, rng.range(0.2, 0.35));
                let pts: Vec<(f32, f32)> = (0..=4).map(|i| { let xx = x + len * i as f32 / 4.0; (xx, bank_top(xx) + 1.3 + rng.normal() * 0.3) }).collect();
                c.drag(&mut b, &Gesture::new(pts).pressure(rng.range(0.3, 0.5), 0.2).ramps(0.3, 0.5), Some(&bank_m));
            }
            x += len + rng.range(3.0, 30.0);
        }
        let mut b = Held::new(Tool::round_sable(1.6), 44);
        let mut x = -5.0;
        while x < 1000.0 {
            let len = rng.range(20.0, 70.0);
            b.reload(lipc, 0.5);
            let pts: Vec<(f32, f32)> = (0..=5).map(|i| { let xx = x + len * i as f32 / 5.0; (xx, water_edge(xx) - 1.6 + rng.normal() * 0.3) }).collect();
            c.drag(&mut b, &Gesture::new(pts).pressure(rng.range(0.5, 0.8), 0.3).ramps(0.2, 0.4), None);
            x += len * rng.range(0.8, 1.1);
        }
        c.dry();
        // the bank's reflection: a dark band under the edge, its lower edge
        // lost in level strokes; the bushes hang down in it
        // row by row, level pulls of a round, each row as long as the hand
        // makes it; the lower rows thinner and broken, lost in the water
        let mut b = Held::new(Tool::round_sable(2.2), 55);
        let mut row = 0;
        let mut y = WL - 1.6;
        while y < WL + 24.0 {
            let mut x = rng.range(-30.0, -5.0);
            while x < 1000.0 {
                let l = rng.range(25.0, 90.0);
                let xm = x + l * 0.5;
                let depth = (water_edge(xm) - bank_top(xm)) * 0.95;
                let d = y - water_edge(xm);
                let fade = 1.0 - smoothstep(depth * 0.55, depth + 2.0, d);
                if fade > 0.05 && rng.f() < 0.35 + 0.65 * fade {
                    let col = mix(hex("#2d2e2a"), water_col(xm, y + 6.0), 0.12 + 0.55 * (1.0 - fade), Mix::Pigment);
                    b.reload(pal.mix(col).paint(0.28).with_hiding(0.9), rng.range(0.4, 0.65));
                    let yy = y + rng.normal() * 0.25;
                    c.drag(&mut b, &Gesture::new(vec![(x, yy), (xm, yy + rng.normal() * 0.2), (x + l, yy + rng.normal() * 0.25)]).pressure(rng.range(0.5, 0.8) * (0.6 + 0.4 * fade), 0.4).ramps(0.15, 0.25), None);
                }
                x += l * rng.range(0.75, 1.05);
            }
            row += 1;
            y += rng.range(1.3, 1.9);
        }
        let _ = row;
        c.dry();
    }

    // ------------------------------------------------------------ poplars
    if o.stage("poplars", &mut c, &mut rng) {
        for p in trees.iter() {
            poplar(&mut c, &st, p, &sky_col, &mut rng);
        }
        c.dry();
    }

    if o.stage("reflections", &mut c, &mut rng) {
        for p in &trees {
            reflection(&mut c, pal, p, &water_col, &shore, &mut rng);
        }
        c.dry();
        // wind lines: a few long pale level streaks across the dark, where
        // a breath of air roughens the surface and shows the sky
        let streak = |y: f32| pal.mix(mix(water_col(500.0, y), hex("#c9bfa9"), 0.35, Mix::Pigment)).paint(0.35).with_hiding(0.55);
        let mut b = Held::new(st.line_tool(0.7), 61);
        for &(y0, x0, len) in &[(478.0f32, 380.0f32, 230.0f32), (503.0, 440.0, 170.0), (531.0, 300.0, 340.0), (566.0, 470.0, 150.0), (590.0, 520.0, 200.0)] {
            let mut x = x0;
            while x < x0 + len {
                let l = rng.range(10.0, 45.0);
                b.reload(streak(y0), rng.range(0.25, 0.4));
                let y = y0 + rng.normal() * 0.4;
                b.tool = st.line_tool(0.5 + 0.8 * smoothstep(WL, 620.0, y0));
                c.drag(&mut b, &Gesture::new(vec![(x, y), (x + l * 0.5, y + rng.normal() * 0.2), (x + l, y + rng.normal() * 0.3)]).pressure(rng.range(0.35, 0.6), 0.15).ramps(0.3, 0.5), Some(&water_m));
                x += l + rng.range(2.0, 18.0);
            }
        }
        c.dry();
    }


    // ------------------------------------------------------------ near shore
    if o.stage("shore", &mut c, &mut rng) {
        let sn = Fbm::new(81, 4, 60.0);
        let sc = move |x: f32, y: f32| {
            let t = smoothstep(shore(x), h, y);
            let base = mix(hex("#3a342b"), hex("#1f1c18"), t, Mix::Pigment);
            mix(base, hex("#4b4535"), 0.3 * sn.get01(x, y * 2.0), Mix::Pigment)
        };
        let body = st.body().color(sc).angle(move |x, _| ((shore(x + 5.0) - shore(x - 5.0)) / 10.0).atan()).angle_jitter(0.3).length(14.0, 45.0).coverage(3.4).medium(0.2);
        c.work(&shore_m, &body, 81);
        c.dry();
        // the water's lip at the near shore: a thin dull light where the
        // wet mud takes the sky, broken
        let wet = pal.mix(hex("#4e4a40")).paint(0.3).with_hiding(0.5);
        let mut b = Held::new(Tool::round_sable(1.3), 82);
        let mut x = 0.0;
        while x < 1000.0 {
            let len = rng.range(12.0, 50.0);
            if rng.f() < 0.3 {
                b.reload(wet, 0.25);
                let pts: Vec<(f32, f32)> = (0..=4).map(|i| { let xx = x + len * i as f32 / 4.0; (xx, shore(xx) + 1.0 + rng.normal() * 0.3) }).collect();
                c.drag(&mut b, &Gesture::new(pts).pressure(rng.range(0.35, 0.55), 0.2).ramps(0.3, 0.5), None);
            }
            x += len + rng.range(4.0, 40.0);
        }
        c.dry();
    }

    if o.stage("reeds", &mut c, &mut rng) {
        reeds(&mut c, pal, &shore, h, &water_col, &mut rng);
        c.dry();
    }

    // ------------------------------------------------------------ veil
    if o.stage("veil", &mut c, &mut rng) {
        // a thin darkening glaze, heavier toward the corners and the bottom,
        // never zero, kept off the glow
        c.glaze(&paint::Pigment::with_hiding(hex("#6e6258"), 0.04), None, move |x, y| {
            let dx = (x - 520.0) / 600.0;
            let dy = (y - 400.0) / 420.0;
            let r = (dx * dx + dy * dy).sqrt();
            let glow = (-((x - glow_x) / 300.0).powi(2) - ((y - HZ) / 80.0).powi(2)).exp();
            (0.06 + 0.55 * smoothstep(0.4, 1.3, r) + 0.2 * smoothstep(620.0, h, y)) * (1.0 - 0.7 * glow)
        });
    }

    let _ = w;
    o.finish(&mut c, &mut rng, &Finish::aged(st.relief));
}

const MOON: (f32, f32) = (262.0, 152.0);

// ======================================================================
// My hand: a Lombardy poplar, its reflection, the reeds.

#[derive(Clone, Copy)]
struct Poplar {
    x: f32,
    foot: f32,
    ht: f32,
    /// half the crown's greatest width
    hw: f32,
    lean: f32,
    seed: u32,
    /// top broken off, a dead leader standing out of it
    broken: bool,
}

impl Poplar {
    /// Where the crown starts (share of the height from the foot).
    const CB: f32 = 0.085;

    /// The axis at height share t.
    fn axis(&self, t: f32) -> f32 {
        let n = Fbm::new(self.seed + 100, 2, 120.0);
        self.x + self.lean * t * self.ht + 1.5 * n.get(t * self.ht, 0.0) * t
    }

    fn y(&self, t: f32) -> f32 {
        self.foot - t * self.ht
    }

    /// The crown's top (share): a broken tree ends low and flat-ish.
    fn top(&self) -> f32 {
        if self.broken { 0.8 } else { 1.0 }
    }

    /// Half-width at t on side s (-1 left, 1 right): widest a third of the
    /// way up, tapering to a spire; lobed where the sprays stand out, the
    /// two sides not alike.
    fn half(&self, t: f32, s: f32) -> f32 {
        if t < Self::CB {
            return 0.0;
        }
        let u = ((t - Self::CB) / (1.0 - Self::CB)).clamp(0.0, 1.0);
        let peak = 0.3;
        let prof = if u < peak { (u / peak).powf(0.45) } else { ((1.0 - u) / (1.0 - peak)).powf(0.8) };
        let n1 = Fbm::new(self.seed + if s < 0.0 { 1 } else { 2 }, 3, 1.0);
        let lobes = 1.0 + 0.3 * n1.get(t * self.ht / 26.0, 0.0) + 0.12 * n1.get(t * self.ht / 7.0, 5.0);
        let mut hw = self.hw * prof * lobes;
        if self.broken {
            // the crown stops in a ragged, rounded top
            let e = self.top();
            let k = ((e - t) / 0.08).clamp(0.0, 1.0);
            hw *= k.sqrt().max(0.0) * (0.8 + 0.25 * n1.get(t * 30.0, 9.0));
            if t > e {
                return 0.0;
            }
        }
        hw.max(0.0)
    }

    /// The silhouette: up the left flank, down the right.
    fn outline(&self) -> Vec<(f32, f32)> {
        let n = 90;
        let t0 = Self::CB;
        let t1 = self.top();
        let mut pts = Vec::new();
        for i in 0..=n {
            let t = t0 + (t1 - t0) * i as f32 / n as f32;
            pts.push((self.axis(t) - self.half(t, -1.0), self.y(t)));
        }
        for i in (0..=n).rev() {
            let t = t0 + (t1 - t0) * i as f32 / n as f32;
            pts.push((self.axis(t) + self.half(t, 1.0), self.y(t)));
        }
        pts
    }
}

/// A Lombardy poplar against the evening light, built the way it grows:
/// the stem, then its many branches rising steeply from it, each carrying
/// its sprays of leaves in short upswept strokes; the silhouette comes from
/// where the sprays end, not from a drawn line. A dark core inside first
/// (dead color) so the sprays have a dark under them; then the sprays, from
/// three dark piles; a little cool light on the flank toward the open sky,
/// a warmer rim where the glow behind catches the right edge; the trunk.
fn poplar(c: &mut paint::Canvas, st: &Style, p: &Poplar, sky_col: &(impl Fn(f32, f32) -> Rgb + Sync), rng: &mut Rng) {
    let pal = &st.palette;
    let f = c.frame();
    let piles = [pal.mix(hex("#161915")).paint(0.2), pal.mix(hex("#1c201b")).paint(0.2), pal.mix(hex("#252820")).paint(0.22)];
    let edge_lit = pal.mix(hex("#3f4541")).paint(0.25).with_hiding(0.7);
    let warm_rim = pal.mix(hex("#51493a")).paint(0.25).with_hiding(0.6);
    let t1 = p.top();
    let t_of = |y: f32| (p.foot - y) / p.ht;

    // the stem up through the crown
    let stem = pal.mix(hex("#221f1c")).paint(0.2);
    let mut sb = Held::new(Tool::round_sable((p.hw * 0.1).max(1.2)), rng.next_u64());
    sb.load(stem, 0.8);
    let pts: Vec<(f32, f32)> = (0..=10).map(|i| { let t = i as f32 / 10.0 * 0.85 * t1; (p.axis(t), p.y(t) + 2.0) }).collect();
    c.drag(&mut sb, &Gesture::new(pts).pressure(0.9, 0.2).ramps(0.0, 0.6).shake(0.4), None);

    // dead color in the core only: the inner half of the crown, uneven
    let cn = Fbm::new(p.seed + 50, 3, 30.0);
    let core = Mask::from_fn(f, move |x, y| {
        let t = t_of(y);
        if t < Poplar::CB + 0.02 || t > t1 - 0.04 {
            return 0.0;
        }
        let k = (0.45 + 0.2 * cn.get(x, y)) * smoothstep(Poplar::CB, Poplar::CB + 0.035, t).sqrt();
        let (l, r) = (p.axis(t) - p.half(t, -1.0) * k, p.axis(t) + p.half(t, 1.0) * k);
        smoothstep(l - 1.0, l + 1.0, x) * (1.0 - smoothstep(r - 1.0, r + 1.0, x))
    });
    let body = st.body().color(|_, _| hex("#1d201c")).angle(|_, _| -FRAC_PI_2).angle_jitter(0.3).length(6.0, 18.0).coverage(2.6).medium(0.3).hug(false);
    c.work(&core, &body, rng.next_u64());
    c.wait(90.0);

    // the branches: from the stem, steeply up and out to the flank, then up
    // along it; each gets its sprays. Their number and spacing uneven, so
    // the crown has hollows and bulges and a few sky gaps between them.
    let bw = (p.hw * 0.055).clamp(1.1, 1.9);
    let mut brushes: Vec<Held> = (0..3).map(|_| Held::new(Tool::round_sable(bw * rng.range(0.85, 1.15)), rng.next_u64())).collect();
    let mut fine = Held::new(Tool::round_sable(bw * 0.65), rng.next_u64());
    let nb = (p.ht / 3.2) as usize;
    let mut sprays = 0usize;
    for i in 0..nb {
        let t0 = Poplar::CB + (t1 - Poplar::CB - 0.03) * (i as f32 + rng.range(0.0, 1.0)) / nb as f32;
        let s = if rng.f() < 0.5 { -1.0 } else { 1.0 };
        let rise = rng.range(0.07, 0.17) * (1.0 - 0.4 * t0);
        let tt = (t0 + rise).min(t1 - 0.005);
        let reach = rng.range(0.55, 1.08);
        let hw = p.half(tt, s);
        if hw < 1.0 {
            continue;
        }
        let a = (p.axis(t0), p.y(t0));
        let e = (p.axis(tt) + s * hw * reach, p.y(tt));
        // the branch bows outward, then turns up
        let m = (a.0 + (e.0 - a.0) * 0.75, a.1 + (e.1 - a.1) * 0.4);
        let path = |u: f32| -> (f32, f32) {
            let v = 1.0 - u;
            (v * v * a.0 + 2.0 * v * u * m.0 + u * u * e.0, v * v * a.1 + 2.0 * v * u * m.1 + u * u * e.1)
        };
        // how leafy this branch is: some are thin, leaving a hollow
        let leafy = rng.range(0.35, 1.0);
        let len = ((e.0 - a.0).powi(2) + (e.1 - a.1).powi(2)).sqrt();
        let n = (len * 1.4 * leafy) as usize + 2;
        for _ in 0..n {
            let u = rng.range(0.3, 1.0).powf(0.7);
            let (x0, y0) = path(u);
            let (x1, y1) = path((u + 0.05).min(1.0));
            let dir = (y1 - y0).atan2(x1 - x0);
            // sprays stand up off the branch, steeper at its end
            let up = -FRAC_PI_2 + s * rng.range(0.1, 0.5);
            let mut ang = dir + (up - dir) * rng.range(0.5, 0.9) + rng.normal() * 0.3;
            // low down some sprays hang instead of standing up
            if t0 < Poplar::CB + 0.12 && rng.f() < 0.4 {
                let r = rng.range(0.2, 0.9);
                ang = if s > 0.0 { r } else { PI - r };
            }
            let l = rng.range(3.0, 8.0) * (0.7 + 0.5 * u);
            let bend = rng.normal() * 0.3;
            let k = (rng.f() * 3.0) as usize % 3;
            let pile = if tt > 0.75 * t1 && rng.f() < 0.35 { piles[2] } else { piles[k] };
            let b = &mut brushes[k];
            if sprays % 7 == 0 || b.fullness() < 0.3 {
                b.reload(pile, rng.range(0.5, 0.75));
            }
            sprays += 1;
            let mid = (x0 + 0.5 * l * (ang + bend * 0.3).cos(), y0 + 0.5 * l * (ang + bend * 0.3).sin());
            let end = (x0 + l * (ang + bend).cos(), y0 + l * (ang + bend).sin());
            c.drag(b, &Gesture::new(vec![(x0, y0), mid, end]).pressure(rng.range(0.4, 0.75), 0.06).ramps(0.05, 0.6).shake(0.5), None);
        }
        // the branch's end: a few fine flicks past it, into the air
        for _ in 0..(1 + (rng.f() * 3.0 * leafy) as usize) {
            fine.reload(piles[(rng.f() * 2.0) as usize], 0.45);
            let ang = -FRAC_PI_2 + s * rng.range(0.2, 0.75);
            let l = rng.range(3.0, 7.5);
            let o = (e.0 + rng.normal() * 1.5, e.1 + rng.normal() * 2.0);
            c.drag(&mut fine, &Gesture::new(vec![o, (o.0 + l * 0.5 * ang.cos(), o.1 + l * 0.5 * ang.sin()), (o.0 + l * ang.cos(), o.1 + l * ang.sin() + 0.1 * l)]).pressure(0.65, 0.04).ramps(0.05, 0.7), None);
        }
        // low down, where the crown is thin, the branch itself shows
        if t0 < 0.3 && rng.f() < 0.5 {
            let mut wb = Held::new(Tool::round_sable(0.8), rng.next_u64());
            wb.load(stem, 0.6);
            c.drag(&mut wb, &Gesture::new((0..=4).map(|j| path(j as f32 / 4.0 * 0.6)).collect()).pressure(0.7, 0.2).ramps(0.0, 0.6), None);
        }
    }
    c.wait(60.0);

    // the top
    if !p.broken {
        // the leader: a few fine strokes drawn up into a slightly bent point
        let mut tip = Held::new(Tool::round_sable(bw * 0.5), rng.next_u64());
        let bent = rng.range(1.0, 3.0);
        for _ in 0..6 {
            tip.reload(piles[0], 0.5);
            let t = rng.range(0.9, 0.97);
            let x0 = p.axis(t) + rng.normal() * 0.8;
            c.drag(&mut tip, &Gesture::new(vec![(x0, p.y(t)), (p.axis(0.985) + bent * 0.5, p.y(0.985)), (p.axis(1.0) + bent + rng.normal() * 0.5, p.y(1.0) + rng.range(-1.0, 3.0))]).pressure(0.7, 0.05).ramps(0.05, 0.7), None);
        }
    } else {
        // the broken top: a dead leader standing out of the crown, gray
        // against the glow, a snapped end and two dead side shoots
        let dead = pal.mix(hex("#403c39")).paint(0.2);
        let mut lb = Held::new(Tool::round_sable((bw * 0.9).max(1.2)), rng.next_u64());
        lb.load(dead, 0.8);
        let base = (p.axis(t1 - 0.12), p.y(t1 - 0.12));
        let topp = (p.axis(t1) + 3.0, p.y(t1 + 0.1));
        let mid = ((base.0 + topp.0) * 0.5 - 1.0, (base.1 + topp.1) * 0.5);
        c.drag(&mut lb, &Gesture::new(vec![base, mid, topp]).pressure(0.9, 0.5).ramps(0.0, 0.1).shake(0.6), None);
        lb.reload(dead, 0.5);
        c.drag(&mut lb, &Gesture::new(vec![topp, (topp.0 + 3.5, topp.1 + 2.0)]).pressure(0.6, 0.4).ramps(0.0, 0.3), None);
        let mut tw = Held::new(Tool::round_sable(0.7), rng.next_u64());
        for (u, side, l) in [(0.45f32, -1.0f32, 11.0f32), (0.7, 1.0, 7.0)] {
            let b0 = (base.0 + (topp.0 - base.0) * u, base.1 + (topp.1 - base.1) * u);
            tw.load(dead, 0.5);
            c.drag(&mut tw, &Gesture::new(vec![b0, (b0.0 + side * l * 0.6, b0.1 - l * 0.45), (b0.0 + side * l, b0.1 - l * 0.6)]).pressure(0.7, 0.1).ramps(0.0, 0.6), None);
        }
        let lit = pal.mix(hex("#7a7268")).paint(0.25).with_hiding(0.6);
        let mut lt = Held::new(Tool::round_sable(0.6), rng.next_u64());
        lt.load(lit, 0.35);
        c.drag(&mut lt, &Gesture::new(vec![(topp.0 - 0.8, topp.1 + 2.0), (mid.0 - 0.9, mid.1), (base.0 - 0.9, base.1 - 4.0)]).pressure(0.5, 0.2).ramps(0.2, 0.5), None);
    }
    c.wait(60.0);

    // light: lean, few, broken; the cool one on the left upper sprays, the
    // warm rim on the right where the glow is
    let mut lb = Held::new(Tool::round_sable(bw * 0.7), rng.next_u64());
    for side in [-1.0f32, 1.0] {
        let paint = if side < 0.0 { edge_lit } else { warm_rim };
        let n = (p.ht * if side < 0.0 { 0.25 } else { 0.18 }) as usize;
        for i in 0..n {
            let t = rng.range(0.2, t1 - 0.03);
            let hw = p.half(t, side);
            if hw < 1.5 || rng.f() > 0.3 + 0.6 * t {
                continue;
            }
            let x0 = p.axis(t) + side * hw * rng.range(0.55, 0.9);
            let y0 = p.y(t);
            let a = -FRAC_PI_2 + side * rng.range(0.3, 0.7);
            let len = rng.range(2.5, 6.0);
            if i % 5 == 0 {
                lb.reload(paint, rng.range(0.2, 0.35));
            }
            c.drag(&mut lb, &Gesture::new(vec![(x0, y0), (x0 + len * a.cos(), y0 + len * a.sin())]).pressure(rng.range(0.3, 0.55), 0.08).ramps(0.1, 0.6), None);
        }
    }
    // a few sky holes where two branches' sprays don't meet
    let mut hb = Held::new(Tool::round_sable(bw * 0.55), rng.next_u64());
    for _ in 0..(p.ht / 28.0) as usize {
        let t = rng.range(0.3, t1 - 0.1);
        let s = if rng.f() < 0.5 { -1.0 } else { 1.0 };
        let hw = p.half(t, s);
        if hw < 6.0 {
            continue;
        }
        let x0 = p.axis(t) + s * hw * rng.range(0.6, 0.8);
        let y0 = p.y(t);
        let col = mix(sky_col(x0, y0), hex("#3a3c38"), 0.3, Mix::Pigment);
        hb.load(pal.mix(col).paint(0.3).with_hiding(0.85), 0.3);
        let a = -FRAC_PI_2 + s * 0.45;
        c.drag(&mut hb, &Gesture::new(vec![(x0, y0), (x0 + 1.6 * a.cos(), y0 + 1.6 * a.sin())]).pressure(0.55, 0.35).ramps(0.1, 0.4), None);
    }

    // the trunk below the crown: a little flare, a lean light on the left
    let trunk = pal.mix(hex("#1f1c1a")).paint(0.2);
    let tw = (p.hw * 0.15).max(1.8);
    let mut tb = Held::new(Tool::round_sable(tw), rng.next_u64());
    tb.load(trunk, 0.8);
    let top = (p.axis(Poplar::CB + 0.05), p.y(Poplar::CB + 0.05));
    c.drag(&mut tb, &Gesture::new(vec![(p.x - 0.4, p.foot + 1.5), (p.axis(Poplar::CB * 0.5) + 0.3, p.y(Poplar::CB * 0.5)), top]).pressure(0.95, 0.55).ramps(0.0, 0.3).shake(0.3), None);
    tb.reload(trunk, 0.5);
    c.drag(&mut tb, &Gesture::new(vec![(p.x - tw * 0.55, p.foot + 1.0), (p.x - tw * 0.2, p.foot - 4.0)]).pressure(0.7, 0.3).ramps(0.0, 0.3), None);
    let mut sh = Held::new(Tool::round_sable(0.8), rng.next_u64());
    for _ in 0..(3 + (rng.f() * 3.0) as usize) {
        let s = if rng.f() < 0.5 { -1.0 } else { 1.0 };
        let y0 = p.foot - rng.range(0.0, 8.0);
        let l = rng.range(6.0, 14.0);
        sh.load(piles[1], 0.5);
        c.drag(&mut sh, &Gesture::new(vec![(p.x + s * 1.0, y0), (p.x + s * l * 0.3, y0 - l * 0.6), (p.x + s * l * 0.45, y0 - l)]).pressure(0.7, 0.05).ramps(0.0, 0.6), None);
    }
}

/// The poplar's reflection: laid in level strokes row by row down the
/// water, each row shifted by the small ripples, broken where they catch
/// the sky, fading into the water's own dark toward me.
fn reflection(c: &mut paint::Canvas, pal: &Palette, p: &Poplar, water_col: &(impl Fn(f32, f32) -> Rgb + Sync), shore: &(impl Fn(f32) -> f32 + Sync), rng: &mut Rng) {
    let mirror = WL + 1.0;
    let rip = Fbm::new(p.seed + 300, 3, 9.0);
    let gaps = Fbm::new(p.seed + 301, 3, 5.0);
    let t1 = p.top();
    let mut b = Held::new(Tool::round_sable(2.3), rng.next_u64());
    let mut y = mirror + 0.5;
    let mut k = 0;
    // the image of height t lies as far below the mirror as the tree's
    // point is above it (the bank lifts the tree a little)
    let lift = mirror - p.foot;
    loop {
        let t = (y - mirror - lift) / p.ht;
        if t > t1 || y > shore(p.x) + 2.0 {
            break;
        }
        let d = (y - mirror) / 200.0;
        let (lo, hi) = if t < Poplar::CB {
            // the trunk
            let tw = (p.hw * 0.14).max(1.6);
            (p.axis(t.max(0.0)) - tw * 0.5, p.axis(t.max(0.0)) + tw * 0.5)
        } else {
            (p.axis(t) - p.half(t, -1.0), p.axis(t) + p.half(t, 1.0))
        };
        // ripples displace a row sideways, more with distance
        let dx = (1.0 + 3.5 * d) * rip.get(0.0, y);
        let broken = gaps.get01(0.0, y * 1.2);
        let step = rng.range(0.9, 1.4) + 0.5 * d;
        if hi - lo > 0.8 && broken > 0.22 {
            let wc = water_col(p.x, y);
            let dark = mix(hex("#262a27"), wc, 0.18 + 0.35 * d + 0.3 * (1.0 - broken), Mix::Pigment);
            if k % 4 == 0 || b.fullness() < 0.3 {
                b.reload(pal.mix(dark).paint(0.28).with_hiding(0.9), rng.range(0.45, 0.7));
            }
            k += 1;
            // a row in one or two pulls, the ends ragged
            let (a0, a1) = (lo + dx + rng.normal() * 0.8, hi + dx + rng.normal() * 0.8);
            let split = rng.f() < 0.35 && a1 - a0 > 10.0;
            let pieces: Vec<(f32, f32)> = if split {
                let m = rng.range(a0 + 0.3 * (a1 - a0), a0 + 0.7 * (a1 - a0));
                vec![(a0, m - rng.range(0.5, 2.5)), (m + rng.range(0.5, 2.5), a1)]
            } else {
                vec![(a0, a1)]
            };
            for (u, v) in pieces {
                if v - u < 0.6 {
                    continue;
                }
                let yy = y + rng.normal() * 0.2;
                c.drag(&mut b, &Gesture::new(vec![(u, yy), ((u + v) * 0.5, yy + rng.normal() * 0.15), (v, yy)]).pressure(rng.range(0.55, 0.85), rng.range(0.4, 0.7)).ramps(0.1, 0.2), None);
            }
        }
        y += step;
    }
}

/// The reeds and sedge of the near shore, blade by blade: clumps thick at
/// the two corners, thin in the middle, dark against the water, a few tips
/// catching the glow; a few seed heads; a couple of stems broken over.
fn reeds(c: &mut paint::Canvas, pal: &Palette, shore: &(impl Fn(f32) -> f32 + Sync), h: f32, water_col: &(impl Fn(f32, f32) -> Rgb + Sync), rng: &mut Rng) {
    let dark = [pal.mix(hex("#1e1c17")).paint(0.25), pal.mix(hex("#2a271f")).paint(0.25), pal.mix(hex("#35311f")).paint(0.25)];
    let straw = [pal.mix(hex("#6b6043")).paint(0.25).with_hiding(0.8), pal.mix(hex("#8a7a55")).paint(0.25).with_hiding(0.8)];
    let lean_n = Fbm::new(91, 2, 300.0);
    // clump centers: many at the corners, a few scattered, none on the axis
    let mut clumps: Vec<(f32, f32)> = Vec::new();
    for _ in 0..34 {
        let side = if rng.f() < 0.5 { -1.0 } else { 1.0 };
        let u = rng.f().powf(1.6);
        let x = 500.0 + side * (500.0 - 460.0 * u) + rng.normal() * 10.0;
        if (x - 500.0).abs() < 90.0 {
            continue;
        }
        let dens = smoothstep(120.0, 480.0, (x - 500.0).abs());
        clumps.push((x, dens));
    }
    clumps.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut rig = Held::new(Tool::rigger(0.8), rng.next_u64());
    let mut n = 0;
    for &(cx, dens) in &clumps {
        let base_y = shore(cx) + rng.range(4.0, 30.0);
        let blades = (6.0 + 22.0 * dens * rng.range(0.5, 1.2)) as usize;
        let tallest = (30.0 + 80.0 * dens) * rng.range(0.6, 1.2);
        for _ in 0..blades {
            let x = cx + rng.normal() * (4.0 + 8.0 * dens);
            let y = (base_y + rng.normal() * 3.0).min(h + 4.0);
            let ht = tallest * rng.range(0.25, 1.0);
            let lean = 0.25 * lean_n.get(x, 0.0) + rng.normal() * 0.18;
            let curl = rng.normal() * 0.2;
            let tipx = x + ht * (lean + curl * 0.5);
            let pts = vec![(x, y), (x + ht * lean * 0.35, y - ht * 0.45), (x + ht * (lean * 0.8 + curl * 0.2), y - ht * 0.8), (tipx, y - ht)];
            // the tallest ones stand against the water: those tips are lit
            let lit_tip = ht > tallest * 0.7 && rng.f() < 0.35;
            if n % 6 == 0 {
                rig.tool = Tool::rigger(rng.range(0.6, 1.1));
            }
            rig.reload(dark[(rng.f() * 3.0) as usize % 3], 0.8);
            n += 1;
            c.drag(&mut rig, &Gesture::new(pts.clone()).pressure(rng.range(0.55, 0.85), 0.0).ramps(0.05, 0.75), None);
            if lit_tip {
                rig.reload(straw[(rng.f() * 2.0) as usize % 2], 0.4);
                let p2 = pts[2];
                c.drag(&mut rig, &Gesture::new(vec![p2, (tipx, y - ht)]).pressure(0.35, 0.0).ramps(0.1, 0.8), None);
            }
        }
        // a seed head (reed plume) on some tall clumps, against the water
        if dens > 0.4 && rng.f() < 0.5 {
            let ht = tallest * rng.range(0.9, 1.1);
            let lean = 0.2 * lean_n.get(cx, 0.0);
            let top = (cx + ht * lean, base_y - ht);
            rig.tool = Tool::rigger(0.6);
            rig.reload(dark[1], 0.8);
            c.drag(&mut rig, &Gesture::new(vec![(cx, base_y), (cx + ht * lean * 0.4, base_y - ht * 0.5), top]).pressure(0.6, 0.2).ramps(0.05, 0.3), None);
            // the plume nods to one side: fine hairs hung from the top,
            // brownish, a couple catching the glow
            let mut hb = Held::new(Tool::rigger(0.5), rng.next_u64());
            let nod = if rng.f() < 0.5 { -1.0 } else { 1.0 } * rng.range(0.5, 1.0);
            for j in 0..8 {
                let wc = mix(water_col(top.0, top.1), hex("#3a3226"), 0.55, Mix::Pigment);
                hb.reload(pal.mix(if j % 3 == 2 { wc } else { hex("#3a3226") }).paint(0.25), 0.4);
                let l = rng.range(6.0, 12.0);
                let s = rng.range(0.0, 3.0);
                let o = (top.0 + nod * s * 0.6, top.1 + s);
                c.drag(&mut hb, &Gesture::new(vec![o, (o.0 + nod * l * 0.5, o.1 + l * 0.25), (o.0 + nod * l * 0.8 + rng.normal(), o.1 + l * 0.8)]).pressure(0.6, 0.05).ramps(0.05, 0.7), None);
            }
        }
    }
    // two broken stems bent over the water
    for &cx in &[150.0f32, 872.0] {
        let y = shore(cx) + 8.0;
        let ht = rng.range(40.0, 60.0);
        let s = if cx < 500.0 { 1.0 } else { -1.0 };
        let knee = (cx + s * 4.0, y - ht * 0.6);
        let tip = (knee.0 + s * ht * 0.45, knee.1 + ht * 0.2);
        rig.tool = Tool::rigger(0.8);
        rig.reload(dark[2], 0.8);
        c.drag(&mut rig, &Gesture::new(vec![(cx, y), (cx + s * 2.0, y - ht * 0.3), knee]).pressure(0.7, 0.4).ramps(0.05, 0.2), None);
        rig.reload(straw[0], 0.5);
        c.drag(&mut rig, &Gesture::new(vec![knee, ((knee.0 + tip.0) * 0.5, knee.1 - 1.0), tip]).pressure(0.5, 0.0).ramps(0.05, 0.7), None);
    }
}
