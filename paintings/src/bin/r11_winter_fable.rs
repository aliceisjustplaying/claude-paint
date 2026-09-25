//! Winter dusk on the marsh (an original picture in the manner of
//! Caspar David Friedrich, Dresden c. 1820).
//!
//! A frozen marsh on the plain north of Dresden, half an hour after sunset.
//! The afterglow lies low on the left; above it the sky cools through a
//! pale yellow-gray to a cold blue-gray overhead. A low wooded ridge lies
//! along the horizon in mist, with a village church spire just right of
//! center, half lost. In the middle ground a frozen pool mirrors the sky
//! dully, reeds and tussocks of dead grass poking through the snow at its
//! edge. A dead oak, stag-headed, leans from the left over the snow with
//! snow lying on its big limbs; two small spruces stand on the right. One
//! wanderer, seen from behind, has stopped on the snow between them and
//! looks toward the spire; his tracks come up from the bottom edge. Three
//! crows over the oak, a thin crescent moon low in the afterglow.
//!
//!   cargo paint r11_winter_fable                 1000px → out/r11_winter_fable.png
//!   cargo paint r11_winter_fable -- --full       3200px → out/r11_winter_fable_full.png
//!   cargo paint r11_winter_fable -- --full --crop 120,300,420,660   the oak
//!
//! Stages: drawing, sky, far, snow, pool, oak, spruces, grass, figure,
//! birds (then the finish).

use paint::atmos::{Sky, SkyField};
use paint::color::{Mix, mix, to_oklab, from_oklab};
use paint::scene::{Sun, World};
use paint::graphite::{Mark as Pencil, resample};
use paint::{Fbm, Gesture, Habit, Held, Lead, Mask, Paint, Rng, Stipple, Style, Tool, Touch, hex, shift, smoothstep};
use paintings::run::{Finish, Run};

const ASPECT: f32 = 1.4;
const H: f32 = 1000.0 / ASPECT; // 714.3
/// The horizon: a low one, the sky takes most of the picture.
const HZ: f32 = 440.0;

fn main() {
    let o = Run::new("r11_winter_fable");
    let st = Style::friedrich();
    let pal = &st.palette;
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();

    // ---------------------------------------------------------------- world
    // One world, one sun: set 3.5° below the horizon, a little left of
    // straight ahead, so the afterglow sits left of center and the right
    // side of the sky is already colder.
    let sun = Sun::deg(-24.0, -3.6);
    let w = World::new([0.0, 0.0, 1000.0, H], HZ, 1.7).fov(1000.0, 52.0).sun(sun).visibility(9_000.0);
    let sf = SkyField::new(Sky::new(sun).haze(2.4).uneven(0.55, 25_000.0, 17), &w, 6.0).exposure(0.92);
    let sf = &sf;

    // The sky as I want to paint it: the physical sky for its structure
    // (where the glow is, how fast it cools), pulled toward Friedrich's
    // winter tints: a pale straw-pink glow, gray-violet above it, a cold
    // smalt-gray zenith. All mixed on the palette from lead white, smalt,
    // ochre, a little vermilion.
    let glow_x = 380.0f32;
    let sky_col = move |x: f32, y: f32| {
        let y = y.min(HZ - 0.5);
        let phys = sf.at(x, y);
        let t = ((HZ - y) / HZ).clamp(0.0, 1.0);
        // distance from the glow, in a squashed ellipse (the glow is wider
        // than it is tall)
        let d = (((x - glow_x) / 520.0).powi(2) + ((HZ - y) / 190.0).powi(2)).sqrt();
        let g = (1.0 - smoothstep(0.0, 1.0, d)).powf(1.4);
        let zenith = hex("#8592a3");
        let mid = hex("#b5b5b4");
        let low = hex("#d8d0bb");
        let glow = hex("#e9d2a6");
        let hand = mix(mix(mix(zenith, mid, smoothstep(0.0, 0.7, 1.0 - t), Mix::Light), low, smoothstep(0.5, 1.0, 1.0 - t), Mix::Light), glow, g, Mix::Light);
        // the physics decides where it's brighter and how the right side
        // goes cold; the hand decides the tints
        let pl = to_oklab(phys);
        let mut hl = to_oklab(hand);
        hl[0] = 0.72 * hl[0] + 0.28 * pl[0] + 0.02;
        hl[1] = 0.8 * hl[1] + 0.2 * pl[1];
        hl[2] = 0.8 * hl[2] + 0.2 * pl[2];
        from_oklab(hl)
    };
    let airlight = move |x: f32| sky_col(x, HZ - 6.0);

    // masks that every run needs
    let sky = Mask::from_fn(f, |_, y| if y < HZ + 1.0 { 1.0 } else { 0.0 });

    // ------------------------------------------------------------- drawing
    // Friedrich drew the composition on the ground in graphite, the
    // straights ruled: the horizon, the spire, the oak's trunk, the pool.
    if o.stage("drawing", &mut c, &mut rng) {
        let hard = Lead::pencil("2H").unwrap();
        let soft = Lead::pencil("HB").unwrap();
        let mut worn = 0.0;
        // ruled horizon
        worn += c.draw(&hard, &pencil(&[(0.0, HZ), (1000.0, HZ)], false, 0.35, 0.35), worn, 1);
        // the spire, ruled
        let sx = 618.0;
        for &(a, b) in &[((sx - 5.0, HZ), (sx - 5.0, HZ - 14.0)), ((sx + 5.0, HZ), (sx + 5.0, HZ - 14.0)), ((sx - 5.0, HZ - 14.0), (sx, HZ - 40.0)), ((sx + 5.0, HZ - 14.0), (sx, HZ - 40.0))] {
            worn += c.draw(&hard, &pencil(&[a, b], false, 0.4, 0.4), worn, 2);
        }
        // the pool's edge, freehand
        let pool: Vec<(f32, f32)> = (0..=40).map(|i| { let a = i as f32 / 40.0 * std::f32::consts::TAU; (560.0 + 235.0 * a.cos(), 560.0 + 40.0 * a.sin()) }).collect();
        worn += c.draw(&soft, &pencil(&pool, true, 0.3, 0.3), worn, 3);
        // the oak's trunk and lean, freehand
        let trunk = [(205.0, 645.0), (212.0, 560.0), (222.0, 470.0), (238.0, 400.0)];
        c.draw(&soft, &pencil(&trunk, true, 0.4, 0.2), worn, 4);
    }

    // ----------------------------------------------------------------- sky
    if o.stage("sky", &mut c, &mut rng) {
        // lay-in: thin, long elbow arcs, mostly level, wandering; fused
        // top to bottom with the badger; then dried and stippled twice
        // (Friedrich's skies are stippled, no visible strokes [NG p.56])
        let hd = st.broad().color(sky_col).angle(|x, _| 0.04 * ((x - 500.0) / 500.0)).angle_jitter(0.1).length(50.0, 140.0).coverage(3.6).medium(0.38).clip(true).threshold(0.3);
        c.work(&sky, &hd, 11);
        if let Some(b) = st.blend() {
            c.work(&sky, &b.clip(true).angle(|_, _| 0.0).angle_jitter(0.15), 12);
        }
        c.dry();
        let sp = Stipple::new(Tool::stippler(2.6)).mixed(pal, 0.45).color(sky_col).coverage(|_, _| 1.7).pressure(0.45, 0.8).dips(18, 0.4, 0.5).cluster(0.3, None).clip(true);
        c.stipple(&sky, &sp, 13);
        c.dry();
        // a finer, lighter stipple that thickens toward the glow, thinning
        // out overhead (fade keeps it from reading as snow on the blue)
        let sp = Stipple::new(Tool::stippler(1.7)).mixed(pal, 0.5).color(move |x, y| shift(sky_col(x, y), 0.035, 0.0, 0.01)).coverage(move |x, y| {
            let d = (((x - glow_x) / 560.0).powi(2) + ((HZ - y) / 260.0).powi(2)).sqrt();
            1.6 * (1.0 - smoothstep(0.2, 1.3, d)) + 0.3
        }).pressure(0.4, 0.75).dips(22, 0.35, 0.6).clip(true);
        c.stipple(&sky, &sp, 14);
        c.dry();
        // the moon: a thin waxing crescent low in the afterglow, right of
        // the glow, small (4 mm on a 44 cm canvas). Painted with the point
        // of a round sable: one arc pressed in the middle, lifted at the
        // horns; a second, lighter arc inside it for the earthshine edge.
        let (mx, my, r) = (662.0, 232.0, 8.5);
        let mut b = Held::new(Tool::round_sable(1.6), 21);
        b.load(pal.paint(hex("#f1ead6"), 0.05).with_hiding(0.98), 1.0);
        let arc: Vec<(f32, f32)> = (0..=14).map(|i| { let a = -1.35 + 2.7 * i as f32 / 14.0; (mx + r * a.sin() * 1.0 + 0.4, my - r * a.cos()) }).collect();
        c.drag(&mut b, &Gesture::new(arc).pressure(0.15, 0.15).swell(vec![0.2, 0.6, 1.0, 0.6, 0.2]).ramps(0.1, 0.1).shake(0.3), None);
        c.dry();
    }

    // ----------------------------------------------------------------- far
    // A low wooded ridge along the horizon, in mist; the spire. Setup only.
    let ridge_n = Fbm::new(31, 4, 260.0);
    let ridge_top = f.per_column(move |x| HZ - 6.0 - 16.0 * (0.5 + 0.5 * ridge_n.get(x, 3.0)) * (1.0 - 0.6 * smoothstep(650.0, 1000.0, x)) - 9.0 * smoothstep(60.0, 300.0, x) * (1.0 - smoothstep(330.0, 560.0, x)) - 5.0 * smoothstep(700.0, 820.0, x) * (1.0 - smoothstep(830.0, 1000.0, x)));
    let ridge = Mask::from_fn(f, |x, y| if y >= ridge_top(x) && y < HZ + 2.0 { 1.0 } else { 0.0 }).roughen(5, 36.0, 3.0, 1.2);
    let wood_n = Fbm::new(32, 3, 70.0);
    if o.stage("far", &mut c, &mut rng) {
        // the wooded ridge: a blue-gray only a little darker than the sky
        // at the horizon, laid thin and stippled (the far hills are stippled)
        let rc = move |x: f32, y: f32| {
            let a = airlight(x);
            let wood = hex("#676b75");
            // clumps of trees darker, gaps between them paler
            let clump = 0.5 + 0.5 * wood_n.get(x, y * 0.5);
            let t = 0.22 + 0.2 * smoothstep(HZ - 22.0, HZ - 4.0, y) + 0.16 * clump;
            mix(a, wood, t * (1.0 - 0.3 * smoothstep(600.0, 1000.0, x)), Mix::Light)
        };
        let hd = st.body().color(rc).angle(|_, _| 0.0).angle_jitter(0.2).length(8.0, 22.0).coverage(2.2).medium(0.4).pressure(0.4, 0.7).clip(true).threshold(0.25);
        c.work(&ridge, &hd, 31);
        let sp = Stipple::new(Tool::stippler(1.7)).mixed(pal, 0.45).color(rc).coverage(|_, _| 1.5).pressure(0.4, 0.7).dips(16, 0.35, 0.6).clip(true);
        c.stipple(&ridge, &sp, 32);
        c.dry();
        // the spire: a little darker than the ridge, thin. Nave, tower, a
        // steep spire drawn with the point in three strokes.
        let sx = 618.0;
        let spire_col = mix(airlight(sx), hex("#4e4f56"), 0.62, Mix::Light);
        let mut b = Held::new(Tool::round_sable(2.4), 33);
        let p = pal.paint(spire_col, 0.3);
        b.load(p, 0.9);
        // nave, right of the tower
        c.drag(&mut b, &Gesture::new(vec![(sx + 4.0, HZ - 1.0), (sx + 4.0, HZ - 9.0), (sx + 22.0, HZ - 8.0), (sx + 22.0, HZ - 1.0)]).pressure(0.7, 0.7).ramps(0.05, 0.05).shake(0.3), None);
        // tower
        for k in 0..3 {
            let xx = sx - 3.5 + 3.5 * k as f32;
            c.drag(&mut b, &Gesture::line((xx, HZ - 1.0), (xx, HZ - 16.0)).pressure(0.7, 0.6).ramps(0.03, 0.05).shake(0.3), None);
        }
        // spire: two strokes lifted to the point
        let mut r = Held::new(Tool::round_sable(1.4), 34);
        r.load(p, 0.9);
        c.drag(&mut r, &Gesture::new(vec![(sx - 5.0, HZ - 15.0), (sx - 1.5, HZ - 30.0), (sx, HZ - 43.0)]).pressure(0.8, 0.0).ramps(0.05, 0.6).shake(0.4), None);
        c.drag(&mut r, &Gesture::new(vec![(sx + 5.0, HZ - 15.0), (sx + 1.5, HZ - 30.0), (sx, HZ - 43.0)]).pressure(0.8, 0.0).ramps(0.05, 0.6).shake(0.4), None);
        c.drag(&mut b, &Gesture::line((sx - 4.0, HZ - 16.0), (sx + 4.0, HZ - 16.0)).pressure(0.6, 0.6).ramps(0.05, 0.05).shake(0.3), None);
        c.dry();
        // mist lying on the plain: a veil built by density, densest at
        // the horizon, thinning up over the ridge and down over the far snow
        let mist_col = mix(airlight(500.0), hex("#dcd8cf"), 0.6, Mix::Light);
        let mist_n = Fbm::new(37, 3, 180.0);
        let band = Mask::from_fn(f, |x, y| {
            let cov = if y < HZ { smoothstep(HZ - 30.0, HZ - 6.0, y) } else { 1.0 - smoothstep(HZ + 2.0, HZ + 40.0, y) };
            (cov * (0.8 + 0.3 * mist_n.get(x, y * 2.0))).clamp(0.0, 1.0)
        });
        // (a stipple veil here read as salt at 3200px: pale beads on the
        // ridge. A scumble of lead white, thin, brushed level and fused,
        // lies over it like air.)
        let hd = st.glaze(0.62).color(move |_, _| mist_col).angle(|_, _| 0.0).angle_jitter(0.06).length(60.0, 200.0).coverage(2.2).load_at({ let band = &band; move |x, y| 0.25 + 0.25 * band.sample(x, y) * (0.8 + 0.3 * mist_n.get(x + 500.0, y * 2.0)) }).clip(true).threshold(0.15);
        c.work(&band, &hd, 35);
        if let Some(b) = st.blend() {
            c.work(&band, &b.clip(true).angle(|_, _| 0.0).angle_jitter(0.05), 36);
        }
        c.dry();
    }

    // ---------------------------------------------------------------- snow
    // The snow plain: gentle hummocks (a height field), lit by the glow
    // from the left and front, so the slopes facing the glow are warm and
    // the rest reflect the cold sky. Cooler and paler with distance.
    let hum = Fbm::new(41, 4, 140.0);
    let hum2 = Fbm::new(42, 3, 40.0);
    let height = move |x: f32, y: f32| {
        // perspective: hummocks foreshorten toward the horizon
        let d = ((y - HZ).max(1.0) / (H - HZ)).clamp(0.0, 1.0);
        let sx = x / (0.35 + 0.65 * d);
        let sy = (y - HZ) / (0.25 + 0.75 * d) * 1.8;
        hum.get(sx, sy) * (0.6 + 0.4 * d) + 0.35 * hum2.get(sx, sy) * d
    };
    let slope = move |x: f32, y: f32| {
        let e = 2.0;
        ((height(x + e, y) - height(x - e, y)) / (2.0 * e), (height(x, y + e) - height(x, y - e)) / (2.0 * e))
    };
    let snow_col = move |x: f32, y: f32| {
        let d = ((y - HZ) / (H - HZ)).clamp(0.0, 1.0);
        let (gx, gy) = slope(x, y);
        // the glow is left and ahead: a slope rising to the left (gx > 0,
        // height increases with x means facing left... take -gx) faces it
        let face = (-gx * 9.0 - gy * 6.0).clamp(-1.0, 1.0);
        let lit = hex("#e8e2d3");
        let cold = hex("#c8ccd4");
        let shade = hex("#a6adbc");
        let near = mix(mix(cold, lit, smoothstep(-0.2, 0.8, face), Mix::Pigment), shade, smoothstep(0.1, 0.9, -face), Mix::Pigment);
        // far snow takes the sky's gray and the mist
        let far = mix(hex("#c9c6c0"), airlight(x), 0.3, Mix::Light);
        mix(far, near, smoothstep(0.0, 0.55, d).powf(0.8), Mix::Pigment)
    };
    // the snow family: no red earth or vermilion in it, so no pink piles
    let snow_pal = pal.only(&["lead white", "pale smalt", "yellow ochre", "raw umber", "cobalt blue"]);
    let snow_pal = &snow_pal;
    let plain = Mask::from_fn(f, |_, y| if y >= HZ - 1.0 { 1.0 } else { 0.0 });
    if o.stage("snow", &mut c, &mut rng) {
        // lead white with a little smalt and ochre, opaque, strokes
        // following the hummocks' contours; a little more body near the
        // bottom (the slight impasto of foreground snow [NG p.50])
        // a thin opaque underpainting first, so the warm ground is out
        let hd = st.broad().palette(snow_pal).color(move |x, y| shift(snow_col(x, y), -0.02, 0.0, 0.0)).by_masstone().angle(|_, _| 0.0).angle_jitter(0.2).length(40.0, 120.0).coverage(3.0).medium(0.2).clip(true).threshold(0.3);
        c.work(&plain, &hd, 40);
        c.dry();
        let hd = st.broad()
            .palette(snow_pal)
            .color(snow_col)
            .angle(move |x, y| { let (gx, gy) = slope(x, y); (-gx).atan2(gy) + std::f32::consts::FRAC_PI_2 })
            .angle_jitter(0.15)
            .length(30.0, 110.0)
            .coverage(3.0)
            .medium(0.22)
            .load_at(move |_, y| 0.45 + 0.35 * smoothstep(HZ, H, y))
            .pressure(0.5, 0.85)
            .clip(true)
            .threshold(0.3);
        c.work(&plain, &hd, 41);
        // a second, shorter pass in body color where the hummocks turn
        let hd = st.body().palette(snow_pal).color(snow_col).angle(move |x, y| { let (gx, gy) = slope(x, y); (-gx).atan2(gy) + std::f32::consts::FRAC_PI_2 }).angle_jitter(0.2).length(14.0, 40.0).coverage(1.6).medium(0.18).pressure(0.5, 0.9).clip(true).threshold(0.3);
        let turning = Mask::from_fn(f, |x, y| { let (gx, gy) = slope(x, y); (smoothstep(0.03, 0.12, (gx * gx + gy * gy).sqrt()) * smoothstep(HZ + 20.0, HZ + 120.0, y)).clamp(0.0, 1.0) });
        c.work(&turning, &hd, 42);
        if let Some(b) = st.blend() {
            let farsnow = Mask::from_fn(f, |_, y| if y >= HZ - 1.0 { 1.0 - smoothstep(HZ + 60.0, HZ + 160.0, y) } else { 0.0 });
            c.work(&farsnow, &b.clip(true).angle(|_, _| 0.0).angle_jitter(0.1).coverage(2.0), 43);
        }
        c.dry();
        // far snow stippled into the mist, as the far hills are
        let farsnow = Mask::from_fn(f, |_, y| if y >= HZ - 1.0 { 1.0 - smoothstep(HZ + 30.0, HZ + 110.0, y) } else { 0.0 });
        let sp = Stipple::new(Tool::stippler(2.0)).mixed(snow_pal, 0.45).color(snow_col).coverage(|_, _| 1.3).pressure(0.4, 0.7).dips(16, 0.35, 0.6).clip(true);
        c.stipple(&farsnow, &sp, 44);
        c.dry();
    }

    // ---------------------------------------------------------------- pool
    // The frozen pool: an oval in the middle ground, its edge ragged with
    // snow blown over the ice. It mirrors the sky dully.
    let pool_n = Fbm::new(51, 3, 90.0);
    let pool = Mask::from_fn(f, |x, y| {
        let dx = (x - 560.0) / 235.0;
        let dy = (y - 560.0) / 40.0;
        let r = (dx * dx + dy * dy).sqrt() + 0.18 * pool_n.get(x, y * 3.0);
        1.0 - smoothstep(0.92, 1.0, r)
    });
    if o.stage("pool", &mut c, &mut rng) {
        let ice = move |x: f32, y: f32| {
            let m = sky_col(x, (2.0 * HZ - y).max(2.0) * 0.55 + 60.0);
            let d = mix(m, hex("#7d8290"), 0.45, Mix::Light);
            // ice is paler where snow dusts it, near the edge
            let dx = (x - 560.0) / 235.0;
            let dy = (y - 560.0) / 40.0;
            let r = (dx * dx + dy * dy).sqrt();
            mix(d, hex("#c9cbcf"), 0.5 * smoothstep(0.6, 1.0, r) + 0.15 * (0.5 + 0.5 * pool_n.get(x * 2.0, y)), Mix::Pigment)
        };
        let hd = st.broad().color(ice).angle(|_, _| 0.0).angle_jitter(0.04).length(40.0, 160.0).coverage(3.0).medium(0.35).pressure(0.45, 0.7).clip(true).threshold(0.3);
        c.work(&pool, &hd, 51);
        if let Some(b) = st.blend() {
            c.work(&pool, &b.clip(true).angle(|_, _| 0.0).angle_jitter(0.05), 52);
        }
        c.dry();
        // a paler rim where snow lies over the ice edge, in short level touches
        let rim = pool.rim(6.0, 3.0);
        let hd = st.detail().color(move |x, y| mix(ice(x, y), snow_col(x, y), 0.6, Mix::Pigment)).angle(|_, _| 0.0).angle_jitter(0.3).length(3.0, 9.0).coverage(1.2).clip(true);
        c.work(&rim, &hd, 53);
        c.dry();
    }

    // ----------------------------------------------------------------- oak
    // The dead oak: grown, then painted limb by limb with round sables and
    // a rigger, pressure from the limb's width; snow lies on the upper
    // side of the near-level limbs.
    let oak = Habit { lean: 0.09, decay: 0.5, decline: 0.75, breakage: 0.3, ..Habit::dead_oak() }.grow((207.0, 648.0), 330.0, o.seed + 7);
    if o.stage("oak", &mut c, &mut rng) {
        let dark = pal.paint(hex("#2a221c"), 0.2);
        let dead = pal.paint(hex("#332c27"), 0.2);
        let light = pal.paint(hex("#5e5044"), 0.3);
        paint_tree(&mut c, &oak, dark, dead, &mut rng);
        c.dry();
        // lit side: the glow is left; lean streaks down the left of the
        // big limbs
        for l in oak.limbs.iter().filter(|l| l.w[0] > 2.2 && !l.root && l.pts.len() > 2) {
            let n = l.pts.len();
            let mut b = Held::new(Tool::round_sable((l.w[0] * 0.3).clamp(1.0, 2.6)), rng.next_u64());
            b.load(light.with_hiding(0.6), 0.6);
            let pts: Vec<(f32, f32)> = (0..n).map(|i| { let d = l.dir(i); let nr = (-d.1, d.0); let s = if nr.0 < 0.0 { 1.0 } else { -1.0 }; (l.pts[i].0 + s * nr.0 * l.w[i] * 0.3, l.pts[i].1 + s * nr.1 * l.w[i] * 0.3) }).collect();
            c.drag(&mut b, &Gesture::new(pts).pressure(0.5, 0.2).ramps(0.2, 0.4).shake(0.7), None);
        }
        c.dry();
        // snow lying on the limbs: along the upper side of every run of
        // segments that lies nearer level than vertical, one short stroke
        // of lead white dragged along the limb, offset up by 0.4 of its
        // width, pressed harder on thicker wood; broken where the run
        // steepens. Separate touches read as beads at 3200px; a dragged
        // stroke lies on the wood.
        let snow = pal.paint(hex("#ece8dd"), 0.08).with_hiding(0.97);
        for l in oak.limbs.iter().filter(|l| !l.root && l.w[0] > 0.9) {
            let n = l.pts.len();
            let mut run: Vec<(f32, f32, f32)> = vec![]; // x, y, width
            let flush = |c: &mut paint::Canvas, run: &mut Vec<(f32, f32, f32)>, rng: &mut Rng| {
                if run.len() >= 2 {
                    let wm = run.iter().map(|r| r.2).fold(0.0f32, f32::max);
                    let mut b = Held::new(Tool { ragged: 0.4, ..Tool::round_sable((wm * 0.45).clamp(0.9, 3.2)) }, rng.next_u64());
                    b.load(snow, 0.9);
                    let pts: Vec<(f32, f32)> = run.iter().map(|r| (r.0, r.1)).collect();
                    let p = (0.35 + 0.5 * (wm / 6.0).clamp(0.2, 1.0)).min(0.9);
                    let k = run.len().min(6);
                    let swell: Vec<f32> = (0..k).map(|_| rng.range(0.4, 1.1)).collect();
                    c.drag(&mut b, &Gesture::new(pts).pressure(p, p * 0.6).ramps(0.15, 0.3).swell(swell).shake(0.7), None);
                }
                run.clear();
            };
            for i in 0..n.saturating_sub(1) {
                let (a, e) = (l.pts[i], l.pts[i + 1]);
                let d = (e.0 - a.0, e.1 - a.1);
                let len = (d.0 * d.0 + d.1 * d.1).sqrt();
                if len < 0.3 { continue; }
                let level = 1.0 - (d.1.abs() / len).powf(0.7); // 1 = level, 0 = vertical
                if level < 0.4 || rng.f() < 0.12 {
                    flush(&mut c, &mut run, &mut rng);
                    continue;
                }
                let wdt = l.w[i];
                let up = (-d.1 / len, d.0 / len);
                let up = if up.1 > 0.0 { (-up.0, -up.1) } else { up }; // canvas y down: up is negative y
                if run.is_empty() {
                    run.push((a.0 + up.0 * wdt * 0.4, a.1 + up.1 * wdt * 0.4, wdt));
                }
                run.push((e.0 + up.0 * wdt * 0.4, e.1 + up.1 * wdt * 0.4, wdt));
            }
            flush(&mut c, &mut run, &mut rng);
        }
        c.dry();
    }

    // ------------------------------------------------------------- spruces
    if o.stage("spruces", &mut c, &mut rng) {
        let needle = pal.paint(hex("#1a201d"), 0.12);
        let needle_lit = pal.paint(hex("#2c3330"), 0.15);
        let snow = pal.paint(hex("#e7e5dc"), 0.08).with_hiding(0.97);
        spruce(&mut c, (795.0, 612.0), 172.0, needle, needle_lit, snow, &mut rng);
        spruce(&mut c, (846.0, 632.0), 118.0, needle, needle_lit, snow, &mut rng);
        // a third, smaller, further back and paler
        let far_needle = pal.paint(mix(hex("#1a201d"), airlight(870.0), 0.3, Mix::Light), 0.2);
        let far_lit = pal.paint(mix(hex("#2c3330"), airlight(870.0), 0.35, Mix::Light), 0.2);
        spruce(&mut c, (905.0, 560.0), 64.0, far_needle, far_lit, snow, &mut rng);
        c.dry();
    }

    // --------------------------------------------------------------- grass
    // Dead grass and reeds through the snow: fine upturning strokes laid
    // over the finished snow, last [NG p.56]. Clumps along the pool's edge,
    // around the oak's foot and scattered over the near snow.
    if o.stage("grass", &mut c, &mut rng) {
        let straw = hex("#7a6a48");
        let dark = hex("#3f3629");
        let mut rg = Held::new(Tool::rigger(0.7), 61);
        let mut sb = Held::new(Tool::round_sable(1.3), 62);
        let mut clumps: Vec<(f32, f32, f32, usize)> = vec![];
        // around the pool
        for i in 0..18 {
            let a = 0.15 + std::f32::consts::TAU * i as f32 / 18.0 + rng.range(-0.15, 0.15);
            let r = 1.0 + rng.range(0.0, 0.12);
            let (x, y) = (560.0 + 235.0 * r * a.cos(), 560.0 + 40.0 * r * a.sin() + 2.0);
            if a.sin() < -0.3 && rng.f() < 0.5 { continue; } // fewer on the far edge
            clumps.push((x, y, 8.0 + 14.0 * rng.f() * (0.5 + 0.5 * a.sin().max(0.0)), 5 + (rng.f() * 8.0) as usize));
        }
        // the oak's foot
        for _ in 0..7 {
            clumps.push((207.0 + rng.range(-40.0, 45.0), 650.0 + rng.range(-4.0, 10.0), 12.0 + 12.0 * rng.f(), 6 + (rng.f() * 8.0) as usize));
        }
        // scattered
        for _ in 0..16 {
            let y = HZ + 70.0 + (H - HZ - 80.0) * rng.f().powf(0.7);
            let x = rng.range(10.0, 990.0);
            let d = (y - HZ) / (H - HZ);
            clumps.push((x, y, (4.0 + 16.0 * d) * (0.6 + 0.6 * rng.f()), 3 + (rng.f() * 6.0 * d) as usize));
        }
        for (x, y, ht, n) in clumps {
            let d = ((y - HZ) / (H - HZ)).clamp(0.0, 1.0);
            let base_col = mix(mix(straw, dark, rng.range(0.2, 0.85), Mix::Pigment), airlight(x), 0.5 * (1.0 - d).powi(2), Mix::Light);
            for k in 0..n {
                let hb = if k % 3 == 2 { &mut sb } else { &mut rg };
                let col = shift(base_col, rng.range(-0.05, 0.05), 0.0, rng.range(-0.01, 0.02));
                hb.reload(Paint::body(col), 0.7);
                let x0 = x + rng.range(-ht * 0.35, ht * 0.35);
                let y0 = y + rng.range(-2.0, 2.0);
                let h = ht * rng.range(0.5, 1.1);
                let lean = rng.range(-0.7, 0.7);
                let pts = vec![(x0, y0), (x0 + lean * h * 0.25, y0 - h * 0.55), (x0 + lean * h, y0 - h)];
                c.drag(hb, &Gesture::new(pts).pressure(0.75, 0.0).ramps(0.05, 0.85).shake(0.6), None);
            }
            // a bent, seeded reed head on some
            if ht > 12.0 && rng.f() < 0.5 {
                rg.reload(Paint::body(dark), 0.6);
                let x0 = x + rng.range(-3.0, 3.0);
                let h = ht * 1.15;
                let lean = rng.range(-0.5, 0.5);
                let pts = vec![(x0, y), (x0 + lean * h * 0.4, y - h * 0.6), (x0 + lean * h, y - h), (x0 + lean * h + 3.0 * lean.signum(), y - h + 2.0)];
                c.drag(&mut rg, &Gesture::new(pts).pressure(0.7, 0.3).ramps(0.05, 0.3).shake(0.6), None);
            }
        }
        c.dry();
    }

    // -------------------------------------------------------------- figure
    // The wanderer, from behind, stopped on the snow, looking toward the
    // spire; his tracks come up from the bottom edge.
    if o.stage("figure", &mut c, &mut rng) {
        // (the world's own figure height here would put his head on the
        // horizon, eye height being his; Friedrich's figures are small in
        // their plains, so he is taken as standing farther off than the
        // pool's near edge suggests)
        let (fx, fy) = (462.0, 634.0);
        let ht = 54.0;
        let coat = pal.paint(hex("#23201d"), 0.15);
        let coat_lit = pal.paint(hex("#3a3531"), 0.2);
        let hat = pal.paint(hex("#1a1816"), 0.15);
        let skin = pal.paint(hex("#a88a6a"), 0.2);
        wanderer(&mut c, (fx, fy), ht, coat, coat_lit, hat, skin, &mut rng);
        c.dry();
        // tracks: pairs of small dents receding from the bottom edge to
        // his feet, spacing shrinking with distance; a dent is a touch of
        // the snow's shadow color pulled short toward the walker
        let mut b = Held::new(Tool::round_sable(1.6), 71);
        let mut y = H + 4.0;
        let mut k = 0;
        while y > fy + 2.0 {
            let d = ((y - HZ) / (H - HZ)).clamp(0.05, 1.0);
            let step = 9.0 * d * d + 1.5;
            let sway = if k % 2 == 0 { -1.0 } else { 1.0 };
            let wob = 0.4 * ((k as f32) * 1.7).sin();
            let x = fx + 2.0 + (y - fy) / (H - fy) * -46.0 + sway * (2.6 * d + 0.6) + wob;
            let under = c.under(x, y, 2.0);
            b.reload(pal.paint(shift(under, -0.17, 0.0, -0.035), 0.2), 0.8);
            let sz = (3.2 * d + 0.6).min(3.8);
            // a dent: a short stroke pulled toward the walker, heavier at the heel
            c.drag(&mut b, &Gesture::line((x - sway * 0.3, y + sz * 0.5), (x + sway * 0.2, y - sz * 0.5)).pressure((0.35 + 0.6 * d).min(0.95), (0.2 + 0.4 * d).min(0.7)).ramps(0.1, 0.3).shake(0.5), None);
            y -= step;
            k += 1;
        }
        c.dry();
    }

    // --------------------------------------------------------------- birds
    if o.stage("birds", &mut c, &mut rng) {
        let dark = Paint::body(hex("#26221f"));
        let mut b = Held::new(Tool::round_sable(1.4), 81);
        for &(x, y, sz, lift) in &[(330.0f32, 285.0f32, 7.0f32, 0.1f32), (372.0, 262.0, 5.5, -0.2), (296.0, 248.0, 4.5, 0.25)] {
            b.reload(dark, 0.8);
            for side in [-1.0f32, 1.0] {
                let tip = (x + side * sz, y - sz * (0.4 + lift * side * 0.5));
                let mid = (x + side * sz * 0.45, y - sz * 0.32);
                c.drag(&mut b, &Gesture::new(vec![(x, y), mid, tip]).pressure(0.7, 0.0).ramps(0.1, 0.75).shake(0.6), None);
            }
            c.touch(&mut b, &Touch::at(x, y + 0.4).pressure(0.4), None);
        }
        // one more on the oak, on a dead limb tip, a hunched dark touch
        if let Some(t) = oak.tips().into_iter().filter(|t| t.1 < 470.0 && t.0 > 230.0).max_by(|a, b| a.0.total_cmp(&b.0)) {
            b.reload(dark, 0.9);
            c.touch(&mut b, &Touch::at(t.0, t.1 - 2.5).pressure(0.7).drag(2.2, -0.3), None);
            c.touch(&mut b, &Touch::at(t.0 + 1.6, t.1 - 4.2).pressure(0.45), None);
        }
        c.dry();
    }

    o.finish(&mut c, &mut rng, &Finish::aged(st.relief));
}

// ------------------------------------------------------------------ motifs

/// A pencil line: the path resampled every unit, pressure from `p0` to `p1`.
fn pencil(pts: &[(f32, f32)], smooth: bool, p0: f32, p1: f32) -> Pencil {
    let pts = resample(pts, smooth, 1.0);
    let n = pts.len().max(2) as f32 - 1.0;
    let pressure = (0..pts.len()).map(|i| p0 + (p1 - p0) * i as f32 / n).collect();
    Pencil { pts, pressure }
}

/// A dead oak from its skeleton: each limb one movement from where it
/// springs to the tip, round sables for the wood, a rigger for the twigs,
/// pressure from the width at each end so it tapers; a live tip lifted to
/// a point, a broken one stopped short and split.
fn paint_tree(c: &mut paint::Canvas, sk: &paint::Skeleton, live: Paint, dead: Paint, rng: &mut Rng) {
    let finest = 0.4;
    for l in sk.limbs.iter().filter(|l| l.pts.len() >= 2) {
        let n = l.pts.len();
        let w0 = l.w[0].max(finest);
        let w1 = l.w[n - 1].max(finest);
        // section the limb where it thins past 40% of the start width
        let mut a = 0usize;
        while a + 1 < n {
            let wa = l.w[a].max(finest);
            let mut b = a + 1;
            while b + 1 < n && l.w[b].max(finest) > wa * 0.42 { b += 1; }
            // overlap back into the wet previous section
            let a0 = if a == 0 { 0 } else { a.saturating_sub(1) };
            let pts: Vec<(f32, f32)> = l.pts[a0..=b].to_vec();
            let tool = if wa > 1.7 { Tool { ragged: 0.3, ..Tool::round_sable(wa * 1.15) } } else { Tool { length: wa * 4.0, ..Tool::rigger(wa.max(0.5)) } };
            let p0 = tool.pressure_for(wa).clamp(0.2, 1.0);
            let p1 = tool.pressure_for(l.w[b].max(finest)).clamp(0.05, 1.0);
            let last = b + 1 >= n;
            // a section that hands over to the next brush ends at full
            // pressure (a release there tapers the limb to nothing at the
            // joint and the next, thinner brush can't fill it)
            let release = if last { if l.broken { 0.06 } else { 0.45 } } else { 0.0 };
            let paint = if l.dead_at(a) { dead } else { live };
            let thin = (wa / (2.0 * finest)).clamp(0.3, 1.0);
            let mut held = Held::new(tool, rng.next_u64());
            held.load(paint.with_hiding(paint.hiding() * (0.5 + 0.5 * thin)), 0.9 * thin.sqrt());
            let g = Gesture::new(pts).pressure(p0, if last && !l.broken { 0.0 } else { p1 }).ramps(if a == 0 { 0.02 } else { 0.0 }, release).shake(0.6);
            c.drag(&mut held, &g, None);
            a = b;
        }
        let _ = (w0, w1);
        // splinters at a break
        if l.broken && l.w[n - 1] > 0.8 {
            let end = l.pts[n - 1];
            let d = l.dir(n - 1);
            let nr = (-d.1, d.0);
            let wd = l.w[n - 1];
            let mut held = Held::new(Tool::rigger(0.6), rng.next_u64());
            for i in 0..3 {
                let off = (i as f32 - 1.0) * wd * 0.3 + rng.normal() * wd * 0.1;
                let len = wd * if i == 1 { rng.range(1.0, 2.0) } else { rng.range(0.4, 0.9) };
                let s = (end.0 - d.0 * wd * 0.5 + nr.0 * off, end.1 - d.1 * wd * 0.5 + nr.1 * off);
                let e = (s.0 + d.0 * len + nr.0 * rng.normal() * 0.3 * len, s.1 + d.1 * len + nr.1 * rng.normal() * 0.3 * len);
                held.reload(dead, 0.8);
                c.drag(&mut held, &Gesture::line(s, e).pressure(0.8, 0.0).ramps(0.0, 0.7).shake(0.5), None);
            }
        }
    }
}

/// A spruce as I paint it: a leader drawn up to the point, then whorls of
/// limbs that droop and turn up at the tip, each hung with short hatched
/// needle strokes (Friedrich's firs are short hatched strokes
/// [NG pp.49–50]); the silhouette ragged, one or two limbs longer than
/// their whorl; snow on the upper side of the lower limbs.
#[allow(clippy::too_many_arguments)]
fn spruce(c: &mut paint::Canvas, (x, base): (f32, f32), ht: f32, needle: Paint, lit: Paint, snow: Paint, rng: &mut Rng) {
    let mut stem = Held::new(Tool::round_sable((ht * 0.014).max(0.8)), rng.next_u64());
    stem.load(needle, 1.0);
    c.drag(&mut stem, &Gesture::new(vec![(x, base), (x + 0.6, base - ht * 0.5), (x - 0.5, base - ht)]).pressure(0.9, 0.0).ramps(0.02, 0.5).shake(0.4), None);
    let mut b = Held::new(Tool::round_sable((ht * 0.008).max(0.7)), rng.next_u64());
    let mut rg = Held::new(Tool::rigger(0.6), rng.next_u64());
    let n = (ht / 5.5) as usize;
    // the whorls, top first so lower limbs hang over upper ones' needles
    for i in (0..n).rev() {
        let t = i as f32 / n as f32;
        let y = base - ht * (0.05 + 0.9 * t) + rng.range(-1.0, 1.0);
        let reach0 = (1.0 - t).powf(0.85) * ht * 0.21 + 2.0;
        for side in [-1.0f32, 1.0] {
            let limbs = if reach0 > 8.0 { 2 } else { 1 };
            for m in 0..limbs {
                let reach = reach0 * rng.range(0.6, 1.15) * if m == 1 { 0.7 } else { 1.0 };
                let yy = y + m as f32 * 2.0 + rng.range(-1.0, 1.0);
                let droop = rng.range(0.22, 0.42);
                let tip = (x + side * reach, yy + reach * droop - reach * 0.08);
                let mid = (x + side * reach * 0.55, yy + reach * droop * 0.7);
                let hb = if reach > 6.0 { &mut b } else { &mut rg };
                hb.reload(if side < 0.0 && rng.f() < 0.45 { lit } else { needle }, 0.85);
                c.drag(hb, &Gesture::new(vec![(x, yy), mid, tip]).pressure(0.8, 0.0).ramps(0.05, 0.7).shake(0.5), None);
                // needles hanging off the limb: short hatched flicks, down
                // and a little outward, denser near the stem
                let k = (reach / 2.2) as usize + 1;
                for j in 0..k {
                    let u = (j as f32 + rng.f()) / k as f32;
                    let px = x + side * reach * u;
                    let py = yy + reach * droop * (u * u * 0.7 + 0.3 * u) + rng.range(-0.6, 0.6);
                    let len = (reach * rng.range(0.22, 0.4) * (1.1 - 0.5 * u)).max(1.5);
                    if j % 3 == 0 { rg.reload(if side < 0.0 && rng.f() < 0.4 { lit } else { needle }, 0.75); }
                    let ang = side * rng.range(0.1, 0.45);
                    c.drag(&mut rg, &Gesture::new(vec![(px, py), (px + ang * len, py + len)]).pressure(0.65, 0.0).ramps(0.05, 0.7).shake(0.6), None);
                }
            }
        }
    }
    // snow lying on the upper side of the lower whorls
    let mut sn = Held::new(Tool::round_sable(1.6), rng.next_u64());
    for i in 0..n {
        let t = i as f32 / n as f32;
        if t > 0.75 || rng.f() < 0.3 { continue; }
        let y = base - ht * (0.05 + 0.9 * t);
        let reach = (1.0 - t).powf(0.85) * ht * 0.21 + 2.0;
        for side in [-1.0f32, 1.0] {
            let k = (reach / 3.5) as usize + 1;
            sn.reload(snow, 0.9);
            let droop = 0.32;
            for j in 0..k {
                let u = (j as f32 + 0.5) / k as f32;
                if rng.f() < 0.35 { continue; }
                let px = x + side * reach * u * rng.range(0.85, 1.0);
                let py = y + reach * droop * (u * u * 0.7 + 0.3 * u) - 1.0;
                c.touch(&mut sn, &Touch::at(px, py).pressure(0.3 + 0.45 * rng.f()).drag(1.5 + reach * 0.1, 0.0), None);
            }
        }
    }
    // the leader's tip: a lifted flick
    rg.reload(needle, 0.8);
    c.drag(&mut rg, &Gesture::new(vec![(x, base - ht * 0.92), (x - 0.8, base - ht - 3.0)]).pressure(0.6, 0.0).ramps(0.0, 0.8).shake(0.5), None);
}

/// A wanderer seen from behind, standing: a long dark coat to the calf,
/// narrow shoulders, a round hat, a stick in the right hand. Written as a
/// few gestures in a frame `ht` units tall: the coat as five downward
/// strokes side by side that widen a little to the hem, the middle one
/// heaviest; a rim of the glow's light down its left edge.
#[allow(clippy::too_many_arguments)]
fn wanderer(c: &mut paint::Canvas, (x, y): (f32, f32), ht: f32, coat: Paint, coat_lit: Paint, hat: Paint, skin: Paint, rng: &mut Rng) {
    let u = ht / 100.0;
    let cw = ht * 0.075; // one coat stroke's width
    let mut b = Held::new(Tool::round_sable(cw.max(1.2)), rng.next_u64());
    let sh = y - ht * 0.78; // shoulders
    let hem = y - ht * 0.2;
    // the coat narrows at the shoulders and flares to the hem; the outer
    // strokes start lower (the shoulders slope) and lean outward
    for k in [-2.0f32, 2.0, -1.0, 1.0, 0.0] {
        b.reload(if k <= -1.0 { coat_lit } else { coat }, 1.0);
        let top = (x + k * cw * 0.45, sh + k.abs() * u * 3.0);
        let bot = (x + k * cw * 1.05, hem - k.abs() * u * 1.5);
        c.drag(&mut b, &Gesture::new(vec![top, (x + k * cw * 0.6, y - ht * 0.55), bot]).pressure(0.85, 0.75).ramps(0.06, 0.08).shake(0.35), None);
    }
    // the hem, a level touch across the bottom of the coat
    b.reload(coat, 0.8);
    c.drag(&mut b, &Gesture::line((x - cw * 2.0, hem - u), (x + cw * 2.0, hem - u)).pressure(0.5, 0.5).ramps(0.1, 0.1).shake(0.4), None);
    // legs and boots below the hem
    let mut s = Held::new(Tool::round_sable((cw * 0.55).max(0.8)), rng.next_u64());
    for dx in [-0.55f32, 0.6] {
        s.reload(coat, 0.9);
        c.drag(&mut s, &Gesture::line((x + dx * cw, hem - u), (x + dx * cw * 1.1, y - u)).pressure(0.75, 0.65).ramps(0.05, 0.1).shake(0.4), None);
    }
    // neck, then the hat over it: crown and brim
    s.reload(skin, 0.8);
    c.touch(&mut s, &Touch::at(x, sh - ht * 0.05).pressure(0.6), None);
    let mut h = Held::new(Tool::round_sable((cw * 1.1).max(1.0)), rng.next_u64());
    h.reload(hat, 1.0);
    c.drag(&mut h, &Gesture::line((x, sh - ht * 0.06), (x, sh - ht * 0.17)).pressure(0.85, 0.7).ramps(0.1, 0.1).shake(0.3), None);
    s.reload(hat, 0.9);
    c.drag(&mut s, &Gesture::line((x - cw * 1.2, sh - ht * 0.09), (x + cw * 1.2, sh - ht * 0.09)).pressure(0.6, 0.6).ramps(0.1, 0.1).shake(0.4), None);
    // the stick: a hairline from the right hand to the snow
    let mut rg = Held::new(Tool::rigger(0.5), rng.next_u64());
    rg.reload(hat, 0.8);
    c.drag(&mut rg, &Gesture::line((x + cw * 1.9, y - ht * 0.5), (x + cw * 2.4, y + 0.5)).pressure(0.5, 0.4).ramps(0.05, 0.1).shake(0.3), None);
    // a rim of glow light on the left edge of the coat and hat
    let mut r = Held::new(Tool::rigger(0.6), rng.next_u64());
    r.reload(coat_lit.with_hiding(0.7), 0.6);
    c.drag(&mut r, &Gesture::new(vec![(x - cw * 1.6, sh + u), (x - cw * 1.75, y - ht * 0.45), (x - cw * 2.0, hem - u)]).pressure(0.45, 0.2).ramps(0.1, 0.3).shake(0.5), None);
}
