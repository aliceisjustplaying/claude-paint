//! r11 study 1: one trunk against sky and snow, in the manner of Caspar
//! David Friedrich (from what is known of his materials and method, never
//! from pictures; see notes/r11_study1.md).
//!
//! A small portrait canvas (26 cm wide). An old oak trunk rises out of a
//! drift of snow and crosses a pale evening sky; the low sun is behind it,
//! a little to the left, so the trunk is seen against the glow and throws
//! its shadow toward us across the snow. The subject is the two meetings:
//! trunk against sky, trunk into snow.
//!
//! Order of work (Friedrich's, as far as it is known): bought ground,
//! graphite drawing, the sky laid thin and stippled, the far snow, the trunk
//! painted over the finished sky, then the snow piled against its foot and
//! the dry grass flicked up last over the snow [NG p.56; ALF p.346].
//!
//!   cargo paint r11_study1
//!   cargo paint r11_study1 -- --full --crop 330,850,640,1100   (the foot)
//!   cargo paint r11_study1 -- --full --crop 420,120,760,380    (the fork)

use paint::color::{Mix, mix};
use paint::graphite::hand_line;
use paint::{Fbm, Gesture, Held, Lead, Mask, Palette, Rng, Stipple, Style, Tool, Touch, gradient, hex, smoothstep};

const ASPECT: f32 = 0.8;
/// Where the far snow meets the sky.
const HORIZON: f32 = 905.0;
/// The sun, just under the horizon behind the trunk, a little left.
const SUN_X: f32 = 360.0;

/// One limb of the tree: a polyline with a radius at each point.
#[derive(Clone)]
struct Limb {
    pts: Vec<(f32, f32, f32)>,
}

impl Limb {
    fn new(pts: &[(f32, f32, f32)]) -> Self {
        Limb { pts: pts.to_vec() }
    }
    /// Distance from (x, y) to the limb's axis, the radius there, the axis
    /// direction there and the along-axis parameter (0..1 per segment + index).
    fn near(&self, x: f32, y: f32) -> (f32, f32, (f32, f32), f32) {
        let mut best = (f32::MAX, 0.0, (0.0, 1.0), 0.0);
        for i in 0..self.pts.len() - 1 {
            let (ax, ay, ar) = self.pts[i];
            let (bx, by, br) = self.pts[i + 1];
            let (dx, dy) = (bx - ax, by - ay);
            let l2 = dx * dx + dy * dy;
            let t = (((x - ax) * dx + (y - ay) * dy) / l2).clamp(0.0, 1.0);
            let (px, py) = (ax + dx * t, ay + dy * t);
            let d = ((x - px).powi(2) + (y - py).powi(2)).sqrt();
            let r = ar + (br - ar) * t;
            // compare by distance relative to the radius, so the thick
            // trunk wins over a thin limb running into it
            if d - r < best.0 - best.1 {
                let l = l2.sqrt();
                best = (d, r, (dx / l, dy / l), i as f32 + t);
            }
        }
        best
    }
    fn at(&self, s: f32) -> (f32, f32, f32) {
        let i = (s.floor() as usize).min(self.pts.len() - 2);
        let t = s - i as f32;
        let (ax, ay, ar) = self.pts[i];
        let (bx, by, br) = self.pts[i + 1];
        (ax + (bx - ax) * t, ay + (by - ay) * t, ar + (br - ar) * t)
    }
    fn dir(&self, s: f32) -> (f32, f32) {
        let i = (s.floor() as usize).min(self.pts.len() - 2);
        let (ax, ay, _) = self.pts[i];
        let (bx, by, _) = self.pts[i + 1];
        let l = ((bx - ax).powi(2) + (by - ay).powi(2)).sqrt();
        ((bx - ax) / l, (by - ay) / l)
    }
    fn segs(&self) -> f32 {
        (self.pts.len() - 1) as f32
    }
}

/// The oak: trunk, leader, a limb crossing the sky to the right, a broken
/// stub on the left and a smaller limb high on the left.
fn tree() -> Vec<Limb> {
    vec![
        // trunk, from under the snow up to the fork
        Limb::new(&[(482.0, 1030.0, 74.0), (476.0, 985.0, 66.0), (468.0, 930.0, 57.0), (463.0, 850.0, 52.0), (470.0, 720.0, 49.0), (458.0, 590.0, 46.0), (452.0, 470.0, 44.0), (461.0, 370.0, 41.0), (452.0, 290.0, 38.0)]),
        // the leader, leaning left, out at the top
        Limb::new(&[(452.0, 300.0, 36.0), (440.0, 215.0, 31.0), (424.0, 140.0, 27.0), (421.0, 60.0, 24.0), (408.0, -30.0, 22.0)]),
        // the limb crossing the sky to the right, crooked
        Limb::new(&[(462.0, 335.0, 25.0), (540.0, 268.0, 19.0), (604.0, 250.0, 15.5), (668.0, 196.0, 12.5), (752.0, 170.0, 9.5), (838.0, 118.0, 7.0), (930.0, 96.0, 5.0), (1030.0, 52.0, 3.5)]),
        // a broken stub, left, dead
        Limb::new(&[(446.0, 540.0, 19.0), (398.0, 505.0, 14.0), (366.0, 492.0, 11.5)]),
        // a smaller limb high on the left
        Limb::new(&[(430.0, 175.0, 13.0), (372.0, 128.0, 9.0), (318.0, 110.0, 6.5), (262.0, 70.0, 4.5)]),
        // root flare, left and right, going under the snow
        Limb::new(&[(452.0, 935.0, 20.0), (426.0, 978.0, 22.0), (408.0, 1004.0, 18.0)]),
        Limb::new(&[(492.0, 935.0, 18.0), (520.0, 972.0, 19.0), (536.0, 1000.0, 15.0)]),
    ]
}

/// The drift piled against the foot of the trunk: the snow surface's y at x.
fn drift_top(x: f32) -> f32 {
    let heap = 24.0 * (-((x - 470.0) / 120.0).powi(2)).exp();
    let lean = 8.0 * (-((x - 535.0) / 45.0).powi(2)).exp();
    // the snow's edge against the bark is ragged at a small scale
    let rag = 2.2 * ((x / 5.3).sin() * 0.5 + (x / 3.1 + 1.7).sin() * 0.3 + (x / 11.0).cos() * 0.6);
    1000.0 - heap - lean + 5.0 * ((x / 47.0).sin() * 0.6 + (x / 23.0).cos() * 0.4) + rag
}

fn main() {
    let o = paintings::run::Run::new("r11_study1");
    // a small study canvas: 26 cm wide (the style's brushes are in canvas
    // units, so they come out finer here, which suits the scale)
    let st = Style { width_mm: 260.0, ..Style::friedrich() };
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = c.height();

    let pal = &st.palette;
    let sky_pal = pal.only(&["lead white", "pale smalt", "cobalt blue", "yellow ochre", "chrome yellow", "vermilion"]);
    let snow_pal = pal.only(&["lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "vermilion"]);
    let bark_pal = pal.only(&["bone black", "raw umber", "red earth", "yellow ochre", "lead white", "cobalt blue"]);

    let limbs = tree();
    let bark_n = Fbm::new(71, 4, 40.0);
    let knob = Fbm::new(72, 3, 9.0);
    let burl = Fbm::new(73, 2, 110.0);
    // the tree's silhouette: capsules along each limb, the edge broken by
    // bark knobs and plates
    let tree_cov = {
        let limbs = limbs.clone();
        move |x: f32, y: f32| -> f32 {
            let mut m: f32 = 0.0;
            for l in &limbs {
                let (d, r, _, _) = l.near(x, y);
                if d > r + 12.0 {
                    continue;
                }
                let r = r * (1.0 + 0.09 * bark_n.get(x, y) + 0.06 * burl.get(x, y).max(0.0)) + 0.9 * knob.get(x, y) * (r / 40.0).min(1.0).sqrt();
                m = m.max(smoothstep(r + 0.9, r - 0.9, d));
            }
            // it goes down into the snow: nothing far below the drift's crest
            m * (1.0 - smoothstep(drift_top(x) + 14.0, drift_top(x) + 22.0, y))
        }
    };
    let tree_m = Mask::from_fn(f, &tree_cov);

    // the sky, as I want it to end: cool gray-blue at the top, a pale
    // greenish middle, a warm glow low down, hottest behind the trunk
    let sky = |x: f32, y: f32| -> paint::Rgb {
        let t = (y / HORIZON).clamp(0.0, 1.0);
        let base = gradient(&[(0.0, hex("#7f8ba2")), (0.3, hex("#a3abb6")), (0.58, hex("#cdd0c3")), (0.82, hex("#e7dcb4")), (0.95, hex("#eccf9c")), (1.0, hex("#e6bf92"))], t, Mix::Light);
        let glow = (-((x - SUN_X) / 260.0).powi(2)).exp() * smoothstep(0.55, 1.0, t);
        mix(base, hex("#f3dca6"), 0.35 * glow, Mix::Light)
    };
    let sky_m = Mask::from_fn(f, |_, y| 1.0 - smoothstep(HORIZON + 6.0, HORIZON + 14.0, y));

    // the snow, as it should end: lit by the sky, so pale and cool where it
    // faces up, warmed near the horizon by the glow, a cool blue-gray in the
    // trunk's long shadow coming toward us
    let swell = Fbm::new(81, 3, 160.0);
    let shadow = |x: f32, y: f32| -> f32 {
        // from the foot of the trunk (470, 1000) toward the viewer, right
        if y < 985.0 {
            return 0.0;
        }
        let t = (y - 985.0) / (h - 985.0);
        let wob = 6.0 * ((y / 37.0).sin() + 0.6 * (y / 13.0 + x / 50.0).sin());
        let cx = 478.0 + t * 230.0 + wob;
        let half = 30.0 + t * 48.0;
        smoothstep(half + 10.0 + 24.0 * t, half - 8.0, (x - cx).abs()) * (0.75 + 0.25 * (1.0 - t))
    };
    let snow = move |x: f32, y: f32| -> paint::Rgb {
        let t = ((y - HORIZON) / (h - HORIZON)).clamp(0.0, 1.0);
        let base = gradient(&[(0.0, hex("#d2c7b3")), (0.1, hex("#c2bfb8")), (0.45, hex("#aeb1b7")), (1.0, hex("#979eaa"))], t, Mix::Light);
        let lit = mix(base, hex("#d9d2c4"), 0.4 * swell.get01(x * 0.7, y * 2.2) * (1.0 - t * 0.5), Mix::Light);
        mix(lit, hex("#858d9c"), 0.7 * shadow(x, y), Mix::Light)
    };
    let snow_m = Mask::from_fn(f, |_, y| smoothstep(HORIZON - 4.0, HORIZON + 4.0, y));

    // ------------------------------------------------------------------
    if o.stage("drawing", &mut c, &mut rng) {
        // graphite: the horizon ruled faint, the trunk's two edges drawn
        // in a firmer second pass, the limbs, the drift's crest
        let faint = Lead::pencil("H").unwrap();
        let firm = Lead::pencil("HB").unwrap();
        let hz = hand_line(&[(0.0, HORIZON + 1.0), (1000.0, HORIZON - 1.0)], &[0.35, 0.3], false, true, 0.0, 1);
        c.draw(&faint, &hz, 0.0, 1);
        let mut worn = 0.0;
        for (k, l) in limbs.iter().enumerate() {
            for side in [-1.0f32, 1.0] {
                let n = 24;
                let pts: Vec<(f32, f32)> = (0..=n)
                    .map(|i| {
                        let s = l.segs() * i as f32 / n as f32;
                        let (x, y, r) = l.at(s);
                        let (dx, dy) = l.dir(s);
                        (x - dy * r * side, y + dx * r * side)
                    })
                    .collect();
                let m = hand_line(&pts, &[0.45, 0.6, 0.5, 0.3], true, false, 0.8, 10 + k as u64 * 2 + (side > 0.0) as u64);
                worn += c.draw(&firm, &m, worn, 20 + k as u64);
            }
        }
        let crest: Vec<(f32, f32)> = (0..=20).map(|i| {
            let x = 330.0 + i as f32 * 14.0;
            (x, drift_top(x))
        }).collect();
        let m = hand_line(&crest, &[0.3, 0.4, 0.3], true, false, 0.6, 5);
        c.draw(&faint, &m, 0.0, 6);
    }

    if o.stage("sky", &mut c, &mut rng) {
        // thin paint in long, nearly level strokes from the elbow, a shade
        // duller than the sky will end; then fused with the badger
        let lay = st.broad()
            .palette(&sky_pal)
            .color(move |x, y| mix(sky(x, y), hex("#8a8a86"), 0.06, Mix::Light))
            .angle(|x, y| 0.03 * ((x + y) / 300.0).sin())
            .coverage(4.0)
            .medium(0.3)
            .length(120.0, 300.0);
        c.work(&sky_m, &lay, 11);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.angle(|_, _| 0.0), 12);
        }
    }

    if o.stage("stipple", &mut c, &mut rng) {
        // first pass into the wet lay-in, aimed at the sky's own tones
        let s1 = Stipple::new(Tool::stippler(3.0))
            .mixed(&sky_pal, 0.45)
            .color(sky)
            .coverage(|_, _| 2.0)
            .pressure(0.5, 0.85)
            .dips(20, 0.4, 0.5);
        c.stipple(&sky_m, &s1, 13);
        c.dry();
        // a finer, lighter pass, dry, thickening toward the glow
        let glow = move |x: f32, y: f32| mix(sky(x, y), hex("#f6e4b6"), 0.05 + 0.2 * smoothstep(350.0, HORIZON, y) * (-((x - SUN_X) / 330.0).powi(2)).exp(), Mix::Light);
        let s2 = Stipple::new(Tool::stippler(1.7))
            .mixed(&sky_pal, 0.5)
            .color(glow)
            .coverage(|_, y| 0.6 + 1.8 * smoothstep(150.0, HORIZON, y))
            .pressure(0.45, 0.85)
            .dips(24, 0.35, 0.6);
        c.stipple(&sky_m, &s2, 14);
        c.dry();
    }

    if o.stage("far", &mut c, &mut rng) {
        // far off, a low line of bushes and a rise breaking the horizon,
        // stippled faint and cool into the dry sky
        let rise = Fbm::new(91, 4, 60.0);
        let top = move |x: f32| HORIZON - 6.0 - 9.0 * rise.get01(x, 3.0).powi(2) * smoothstep(0.0, 250.0, x) - 14.0 * smoothstep(620.0, 990.0, x) * rise.get01(x * 3.0, 9.0);
        let far_m = Mask::from_fn(f, move |x, y| smoothstep(top(x) - 1.0, top(x) + 1.5, y) * (1.0 - smoothstep(HORIZON + 3.0, HORIZON + 10.0, y)));
        let far = Stipple::new(Tool::stippler(1.2))
            .mixed(&snow_pal, 0.55)
            .color(|x, _| mix(hex("#aeaaa6"), hex("#c2b39e"), (-((x - SUN_X) / 200.0).powi(2)).exp(), Mix::Light))
            .coverage(|_, _| 3.2)
            .pressure(0.55, 0.8)
            .dips(16, 0.4, 0.6);
        c.stipple(&far_m, &far, 21);
        c.dry();
    }

    if o.stage("snow", &mut c, &mut rng) {
        // the snow plain: body paint in low, level strokes that follow the
        // ground, lead white rich; the shadow painted into it wet
        let lay = st.body()
            .palette(&snow_pal)
            .color(snow)
            .angle(move |x, y| 0.04 * ((x - 470.0) / 400.0) * smoothstep(HORIZON, h, y))
            .angle_jitter(0.08)
            .length(40.0, 110.0)
            .coverage(3.5)
            .medium(0.15);
        c.work(&snow_m, &lay, 31);
        // the far edge where snow meets sky: a clean pass of the snow's own
        // color along the horizon with a smaller brush
        let edge_m = Mask::from_fn(f, |_, y| smoothstep(HORIZON - 3.0, HORIZON + 2.0, y) * (1.0 - smoothstep(HORIZON + 18.0, HORIZON + 30.0, y)));
        let edge = st.detail().palette(&snow_pal).color(snow).angle(|_, _| 0.0).length(20.0, 50.0).clip(true).coverage(2.0);
        c.work(&edge_m, &edge, 32);
        c.dry();
    }

    if o.stage("trunk", &mut c, &mut rng) {
        // over the finished sky: the tree laid in a warm dark, thin, the
        // strokes running along each limb, cut in at the silhouette
        let limbs2 = limbs.clone();
        let along = move |x: f32, y: f32| {
            let mut best = (f32::MAX, 0.0f32);
            for l in &limbs2 {
                let (d, r, (dx, dy), _) = l.near(x, y);
                if d - r < best.0 {
                    best = (d - r, dy.atan2(dx));
                }
            }
            best.1
        };
        let under = st.body()
            .palette(&bark_pal)
            .color(|x, _| mix(hex("#2e2620"), hex("#3a3129"), smoothstep(520.0, 400.0, x), Mix::Pigment))
            .angle(along)
            .angle_jitter(0.06)
            .length(25.0, 70.0)
            .coverage(3.0)
            .cut_in(Tool::round_sable(3.0))
            .medium(0.15);
        c.work(&tree_m, &under, 41);
        c.wait(40.0);
    }

    if o.stage("bark", &mut c, &mut rng) {
        // the oak's bark: long fissures and plates, drawn along the axis with
        // a small round, darkest down the shaded right side; cool sky light
        // on the left flank; a warm rim where the glow wraps the edge
        let mut dark = Held::new(Tool { point: 0.7, ..Tool::round_sable(2.4) }, 51);
        let mut pale = Held::new(Tool { point: 0.3, ..Tool::filbert(3.5) }, 52);
        let mut plate = Held::new(Tool { ragged: 0.6, ..Tool::filbert(4.5) }, 54);
        let clip = tree_m.clone().dilate(0.8);
        for (k, l) in limbs.iter().enumerate() {
            let len: f32 = (0..l.pts.len() - 1).map(|i| {
                let (a, b) = (l.pts[i], l.pts[i + 1]);
                ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt()
            }).sum();
            let rmean = l.pts.iter().map(|p| p.2).sum::<f32>() / l.pts.len() as f32;
            let n = (len * rmean / 30.0) as usize + 4;
            for _ in 0..n {
                let s0 = rng.f() * l.segs();
                let (x0, y0, r0) = l.at(s0);
                if r0 < 5.0 || y0 > 1015.0 {
                    continue;
                }
                let u = rng.range(-0.95, 0.95);
                let seg = rng.range(8.0, 28.0) * (r0 / 40.0).clamp(0.4, 1.0);
                let (dx, dy) = l.dir(s0);
                let wob = rng.range(-1.0, 1.0);
                let pts: Vec<(f32, f32)> = (0..5).map(|i| {
                    let t = i as f32 / 4.0 - 0.5;
                    let w = u + 0.015 * wob * (t * 5.0).sin();
                    let along = t * seg;
                    (x0 + dx * along - dy * r0 * w, y0 + dy * along + dx * r0 * w)
                }).collect();
                // u < 0 is the left flank (normal (-dy, dx) with dy < 0 → +x... )
                let (px, _) = pts[2];
                let left = px < l.at(s0).0;
                let flank = ((px - x0).abs() / r0.max(1.0)).min(1.0);
                let roll = rng.f();
                if roll < 0.3 {
                    // a fissure: dark, drawn with the point
                    let col = if left { hex("#221b16") } else { hex("#17130f") };
                    dark.reload(bark_pal.paint(col, 0.1), 0.6);
                    c.drag(&mut dark, &Gesture::new(pts).pressure(0.5, 0.2).ramps(0.1, 0.5).shake(0.5), Some(&clip));
                } else if roll < 0.88 || !left {
                    // a plate: a small filbert nearly dry, dragged along the
                    // axis, so it catches the tops of the weave and breaks
                    let col = if left { mix(hex("#362e27"), hex("#433d37"), flank, Mix::Pigment) } else { mix(hex("#312821"), hex("#29221c"), flank, Mix::Pigment) };
                    plate.reload(bark_pal.paint(col, 0.05), rng.range(0.18, 0.32));
                    c.drag(&mut plate, &Gesture::new(pts).pressure(0.35, 0.25).ramps(0.1, 0.3).shake(0.4), Some(&clip));
                } else {
                    // sky light on the left flank, gray, a little green (lichen)
                    let col = if rng.f() < 0.35 { hex("#646857") } else { hex("#5a5955") };
                    pale.reload(bark_pal.paint(col, 0.05), 0.25);
                    c.drag(&mut pale, &Gesture::new(pts).pressure(0.35, 0.2).ramps(0.2, 0.4).shake(0.6), Some(&clip));
                }
            }
            let _ = k;
        }
        // the glow wrapping the left edge of trunk and leader, a thin warm
        // line against the sky, broken
        let mut rim = Held::new(Tool { point: 0.85, ..Tool::round_sable(1.2) }, 53);
        for &li in &[0usize, 1] {
            let l = &limbs[li];
            let mut s = 0.1;
            while s < l.segs() - 0.05 {
                let ds = rng.range(0.25, 0.6);
                let pts: Vec<(f32, f32)> = (0..4).map(|i| {
                    let ss = (s + ds * i as f32 / 3.0).min(l.segs());
                    let (x, y, r) = l.at(ss);
                    let (dx, dy) = l.dir(ss);
                    let side = if dy < 0.0 { 1.0 } else { -1.0 };
                    (x + dy * r * 0.9 * side, y - dx * r * 0.9 * side)
                }).collect();
                let (_, y, _) = l.at(s);
                if y < 985.0 && rng.f() < 0.7 {
                    let warm = smoothstep(200.0, 900.0, y);
                    rim.reload(bark_pal.paint(mix(hex("#6a6258"), hex("#8b6c4e"), warm, Mix::Pigment), 0.25), 0.35);
                    c.drag(&mut rim, &Gesture::new(pts).pressure(0.3, 0.1).ramps(0.2, 0.6).shake(0.4), Some(&clip));
                }
                s += ds + rng.range(0.05, 0.3);
            }
        }
        c.wait(60.0);
    }

    if o.stage("twigs", &mut c, &mut rng) {
        // the thin wood against the sky: crooked, angular (an oak turns at
        // each bud), pressed where it leaves the limb, lifted off at the tip
        let mut b = Held::new(Tool { point: 0.9, ..Tool::rigger(1.4) }, 61);
        let mut stack: Vec<((f32, f32), f32, f32, f32, u32)> = Vec::new();
        // shoots from the thin ends of the limbs and along the right limb
        let right = &limbs[2];
        for i in 0..24 {
            let s = 1.0 + i as f32 * (right.segs() - 1.2) / 23.0;
            let (x, y, r) = right.at(s);
            let (dx, dy) = right.dir(s);
            let up = rng.f() < 0.72;
            let a = dy.atan2(dx) + if up { -rng.range(0.5, 1.1) } else { rng.range(0.5, 1.0) };
            stack.push(((x, y - if up { r * 0.7 } else { -r * 0.7 }), a, rng.range(40.0, 110.0) * (1.0 - s / 9.0).max(0.4), (r / 10.0).clamp(0.35, 0.8), 0));
        }
        let hi = &limbs[4];
        for i in 0..6 {
            let s = 0.6 + i as f32 * (hi.segs() - 0.6) / 5.0;
            let (x, y, r) = hi.at(s);
            let a = -1.57 + rng.range(-1.1, 0.5);
            stack.push(((x, y - r * 0.5), a, rng.range(30.0, 80.0), 0.5, 0));
        }
        // the tip of the high left limb and of the stub's broken end: a few
        stack.push(((262.0, 70.0), -2.6, 70.0, 0.45, 0));
        let lead = &limbs[1];
        for i in 0..5 {
            let s = 0.8 + i as f32 * 0.6;
            let (x, y, r) = lead.at(s);
            let side = if i % 2 == 0 { -1.0 } else { 1.0 };
            stack.push(((x + side * r * 0.8, y), -1.57 + side * rng.range(0.6, 1.2), rng.range(35.0, 80.0), 0.55, 0));
        }
        let mut n = 0;
        while let Some((p, a, len, pr, depth)) = stack.pop() {
            n += 1;
            if n > 2500 {
                break;
            }
            // two or three straight-ish pieces with a turn at each bud
            let pieces = 2 + (rng.f() * 2.0) as usize;
            let mut pts = vec![p];
            let mut ang = a;
            let mut q = p;
            for _ in 0..pieces {
                ang += rng.range(-0.45, 0.45);
                let l = len / pieces as f32 * rng.range(0.7, 1.3);
                q = (q.0 + ang.cos() * l, q.1 + ang.sin() * l);
                pts.push(q);
            }
            let col = if depth == 0 { hex("#2a231d") } else { hex("#3a3330") };
            b.reload(bark_pal.paint(col, 0.15), 0.6);
            let tip = if len > 12.0 && depth < 5 { pr * 0.45 } else { 0.0 };
            c.drag(&mut b, &Gesture::new(pts.clone()).pressure(pr, tip).ramps(0.03, 0.35).shake(0.5), None);
            if len > 12.0 && depth < 5 {
                let kids = 1 + (rng.f() * 2.2) as usize;
                for _ in 0..kids {
                    let j = 1 + (rng.f() * (pts.len() - 1) as f32) as usize;
                    let j = j.min(pts.len() - 1);
                    let side = if rng.f() < 0.5 { -1.0 } else { 1.0 };
                    let base_ang = (pts[j].1 - pts[j - 1].1).atan2(pts[j].0 - pts[j - 1].0);
                    // oak twigs tend upward
                    let mut na = base_ang + side * rng.range(0.4, 1.0);
                    na = na * 0.75 + (-1.57) * 0.25;
                    let at = pr + (tip - pr) * j as f32 / (pts.len() - 1) as f32;
                    stack.push((pts[j], na, len * rng.range(0.45, 0.65), (at * 0.9).max(0.22), depth + 1));
                }
            }
        }
        c.dry();
    }

    if o.stage("drift", &mut c, &mut rng) {
        // the snow heaped against the foot, over the trunk's bottom: body
        // paint, lead white, laid along the drift's curve
        let drift_m = Mask::from_fn(f, |x, y| {
            let top = drift_top(x);
            let spread = (-((x - 475.0) / 120.0).powi(4)).exp();
            smoothstep(top - 1.2, top + 1.2, y) * (1.0 - smoothstep(top + 20.0, top + 30.0 + 40.0 * spread, y)) * smoothstep(0.08, 0.3, spread)
        });
        let drift_col = move |x: f32, y: f32| -> paint::Rgb {
            // the heap's crest faces the sky: pale, warm where the glow
            // catches it; its face toward us is in the trunk's shadow
            let below = ((y - drift_top(x)) / 40.0).clamp(0.0, 1.0);
            let crest = mix(hex("#d8d0c0"), snow(x, y), 0.4 + 0.6 * below, Mix::Light);
            // right against the bark the snow is in the trunk's own shade
            let foot = (-((x - 478.0) / 75.0).powi(4)).exp() * (1.0 - smoothstep(2.0, 12.0, y - drift_top(x)));
            let crest = mix(crest, hex("#9199a6"), 0.5 * foot, Mix::Light);
            mix(crest, hex("#858d9c"), 0.6 * shadow(x, y + 12.0) * smoothstep(0.1, 0.6, below), Mix::Light)
        };
        let slope = |x: f32, _y: f32| {
            let d = drift_top(x + 3.0) - drift_top(x - 3.0);
            (d / 6.0).atan()
        };
        let lay = st.body()
            .palette(&snow_pal)
            .color(drift_col)
            .angle(slope)
            .angle_jitter(0.1)
            .length(30.0, 70.0)
            .coverage(3.0)
            .medium(0.12);
        c.work(&drift_m, &lay, 71);
        c.dry();
    }

    if o.stage("light", &mut c, &mut rng) {
        // the last light on the snow: a few crisp lead-white touches, a
        // little thick, on the drift crest and the far rim; the cast shadow
        // deepened with a thin smalt veil
        let mut w = Held::new(Tool { point: 0.4, ..Tool::round_sable(3.0) }, 81);
        let lw = Palette::only(&snow_pal, &["lead white", "yellow ochre", "pale smalt"]);
        for _ in 0..14 {
            let x = rng.range(300.0, 660.0);
            if (x - 475.0).abs() < 95.0 {
                continue;
            }
            let l = rng.range(14.0, 34.0);
            let off = rng.range(1.0, 4.0);
            let pts: Vec<(f32, f32)> = (0..5).map(|i| {
                let xx = x + l * i as f32 / 4.0;
                (xx, drift_top(xx) + off + 0.8 * (i as f32 - 2.0).abs())
            }).collect();
            w.reload(lw.paint(hex("#d5cdbd"), 0.05), 0.4);
            c.drag(&mut w, &Gesture::new(pts).pressure(0.45, 0.1).ramps(0.3, 0.6).shake(0.4), None);
        }
        let sh_m = Mask::from_fn(f, move |x, y| shadow(x, y) * smoothstep(975.0, 1010.0, y));
        let veil = st.glaze(0.9).palette(&snow_pal).color(|_, _| hex("#6f7fa0")).angle(|_, _| 1.1).length(60.0, 160.0).load_at(|_, _| 0.25);
        c.work(&sh_m, &veil, 82);
        c.dry();
    }

    if o.stage("grass", &mut c, &mut rng) {
        // dry grass and a few stalks through the snow, flicked up last,
        // thickest near the foot and along a hollow to the left
        let mut b = Held::new(Tool { point: 0.9, ..Tool::rigger(1.0) }, 91);
        let mut t = Held::new(Tool { point: 0.9, ..Tool::round_sable(1.2) }, 92);
        for k in 0..160 {
            let near = rng.f() < 0.55;
            let x = if near { 470.0 + rng.normal() * 110.0 } else { rng.range(20.0, 980.0) };
            let yb = if near { drift_top(x) + rng.range(-2.0, 18.0) } else { rng.range(HORIZON + 40.0, h - 20.0) };
            if tree_cov(x, yb - 4.0) > 0.3 {
                continue;
            }
            let depth = ((yb - HORIZON) / (h - HORIZON)).clamp(0.05, 1.0);
            let ht = rng.range(8.0, 38.0) * (0.3 + depth);
            let lean = rng.range(-0.35, 0.35);
            let col = if rng.f() < 0.5 { hex("#6b5a3f") } else { hex("#8a7652") };
            let hb = if k % 2 == 0 { &mut b } else { &mut t };
            hb.reload(bark_pal.paint(col, 0.1), 0.6);
            let pts = vec![(x, yb), (x + lean * ht * 0.3, yb - ht * 0.55), (x + lean * ht, yb - ht)];
            c.drag(hb, &Gesture::new(pts).pressure(0.55 * (0.5 + depth), 0.0).ramps(0.05, 0.85).shake(0.5), None);
        }
        let _ = Touch::at(0.0, 0.0);
    }

    o.end(&mut c, &mut rng);
    c.relief(st.relief.0, st.relief.1);
    o.save(&mut c);
}
