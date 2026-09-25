//! r11_study2: a study after Friedrich. One passage: a dark oak trunk rising
//! out of snow and crossing a pale evening sky, on a small canvas (24 × 30 cm).
//!
//! Method (notes/research/friedrich_materials.md): bought canvas with a warm
//! lower ground and a patchy light top ground; a graphite underdrawing; a thin
//! sky, laid in and stippled; the tree painted over the dry sky
//! ("trees were painted on the already painted sky", ALF p.346); lead-white
//! snow with a slight impasto in the foreground (NG p.50); dead grass flicked
//! up last over the finished snow (NG p.56). No varnish, no cracks.
//!
//!   cargo paint r11_study2                 1000px
//!   cargo paint r11_study2 -- --full       3200px

use paint::graphite::hand_line;
use paint::color::{Mix, mix};
use paint::{Apply, Fbm, Gesture, Ground, Held, Lead, Mask, Rng, Stipple, Style, Tool, Touch, gradient, hex, shift, smoothstep};
use std::f32::consts::{FRAC_PI_2, PI};

const ASPECT: f32 = 0.8; // 1000 × 1250 units, portrait
const HZ: f32 = 905.0; // the far edge of the snow field

type P = (f32, f32);

/// The trunk: centerline and half-width by height, bark bumps per side.
#[derive(Clone, Copy)]
struct Trunk {
    bumps_l: Fbm,
    bumps_r: Fbm,
    wander: Fbm,
}

impl Trunk {
    fn new() -> Self {
        Trunk { bumps_l: Fbm::new(71, 4, 38.0), bumps_r: Fbm::new(72, 4, 44.0), wander: Fbm::new(73, 3, 420.0) }
    }
    /// x of the axis at height y (it leans a little right as it rises, with a
    /// slow kink where the old leader was lost and a side limb took over)
    fn cx(&self, y: f32) -> f32 {
        let up = (1130.0 - y) / 1130.0;
        412.0 + 44.0 * up + 16.0 * (y / 230.0).sin() - 9.0 * smoothstep(380.0, 470.0, y) + 10.0 * self.wander.get(0.0, y)
    }
    fn slope(&self, y: f32) -> f32 {
        (self.cx(y + 2.0) - self.cx(y - 2.0)) / 4.0
    }
    /// half width: tapering up, flaring into the roots at the snow
    fn hw(&self, y: f32) -> f32 {
        let t = (y / 1130.0).clamp(0.0, 1.2);
        let flare = 26.0 * (-(1105.0 - y).max(0.0) / 55.0).exp();
        // a swelling where the limb leaves (the collar)
        let collar = 7.0 * (-((y - 440.0) / 40.0).powi(2)).exp();
        36.0 + 22.0 * t + flare + collar
    }
    fn left(&self, y: f32) -> f32 {
        self.cx(y) - self.hw(y) - 2.6 * self.bumps_l.get(3.0, y) - 1.2 * self.bumps_l.get(40.0, y * 3.0)
    }
    fn right(&self, y: f32) -> f32 {
        self.cx(y) + self.hw(y) + 2.2 * self.bumps_r.get(9.0, y) + 1.0 * self.bumps_r.get(50.0, y * 3.0)
    }
    /// -1 at the left edge, +1 at the right
    fn across(&self, x: f32, y: f32) -> f32 {
        let (l, r) = (self.left(y), self.right(y));
        (2.0 * (x - l) / (r - l).max(1.0) - 1.0).clamp(-1.5, 1.5)
    }
}

/// A limb: a polyline with a width at every point.
struct Limb {
    pts: Vec<P>,
    w: Vec<f32>,
}

impl Limb {
    /// distance from (x, y) to the limb's edge (negative inside) and the
    /// direction of the nearest segment
    fn sd(&self, x: f32, y: f32) -> (f32, f32) {
        let mut best = (f32::MAX, 0.0);
        for i in 0..self.pts.len() - 1 {
            let (a, b) = (self.pts[i], self.pts[i + 1]);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let l2 = dx * dx + dy * dy;
            let t = (((x - a.0) * dx + (y - a.1) * dy) / l2).clamp(0.0, 1.0);
            let (px, py) = (a.0 + dx * t, a.1 + dy * t);
            let d = ((x - px).powi(2) + (y - py).powi(2)).sqrt();
            let r = 0.5 * (self.w[i] + (self.w[i + 1] - self.w[i]) * t);
            if d - r < best.0 {
                best = (d - r, dy.atan2(dx));
            }
        }
        best
    }
}

/// Bend a polyline a little, point by point (a limb is never straight).
fn wobble(pts: &[P], amount: f32, rng: &mut Rng) -> Vec<P> {
    let n = pts.len();
    pts.iter()
        .enumerate()
        .map(|(i, &(x, y))| if i == 0 || i == n - 1 { (x, y) } else { (x + rng.range(-amount, amount), y + rng.range(-amount, amount)) })
        .collect()
}

/// Oak twigs: short zig-zag shoots, each springing from its parent at an
/// angle and turning at every node; the next order thinner.
fn twigs(from: P, dir: f32, len: f32, w: f32, depth: u32, rng: &mut Rng, out: &mut Vec<(Vec<P>, f32)>) {
    let nodes = 3 + (len / 18.0) as usize;
    let mut pts = vec![from];
    let mut a = dir;
    let mut p = from;
    let seg = len / nodes as f32;
    for _ in 0..nodes {
        a += rng.range(-0.45, 0.45);
        // oak shoots bend up toward the light a little
        a += 0.08 * ((-FRAC_PI_2) - a).sin().signum() * 0.5;
        p = (p.0 + a.cos() * seg * rng.range(0.75, 1.2), p.1 + a.sin() * seg * rng.range(0.75, 1.2));
        pts.push(p);
    }
    out.push((pts.clone(), w));
    if depth == 0 {
        return;
    }
    let kids = 2 + (rng.f() * 2.0) as usize;
    for _ in 0..kids {
        let i = 1 + (rng.f() * (pts.len() - 2) as f32) as usize;
        let side = if rng.f() < 0.5 { -1.0 } else { 1.0 };
        let q = pts[i];
        twigs(q, dir + side * rng.range(0.4, 0.95), len * rng.range(0.35, 0.6), w * 0.55, depth - 1, rng, out);
    }
}

fn main() {
    let o = paintings::run::Run::new("r11_study2");
    // a small canvas, bought primed: warm lower grounds, a lighter, patchy
    // top ground brushed on (KÖR p.284: "a patchy whitish ground")
    let base = Style::friedrich();
    let mut ground = base.ground.clone();
    ground[2] = Ground { color: hex("#c9b597"), hiding: 0.7, um: 55.0, stiff: 0.35, apply: Apply::Brush };
    let st = Style { width_mm: 240.0, ground, ..base };
    let pal = &st.palette;
    let sky_pal = pal.only(&["lead white", "pale smalt", "cobalt blue", "yellow ochre", "vermilion", "raw umber"]);
    let snow_pal = pal.only(&["lead white", "pale smalt", "cobalt blue", "yellow ochre", "vermilion", "raw umber", "bone black"]);
    let bark_pal = pal.only(&["raw umber", "bone black", "yellow ochre", "red earth", "lead white", "pale smalt"]);
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = c.height();

    let tr = Trunk::new();
    let far = Fbm::new(81, 4, 60.0);
    let horizon = f.per_column(move |x| HZ + 5.0 * (x / 210.0).sin() - 9.0 * (1.0 - smoothstep(0.0, 380.0, x)) + 2.0 * far.get(x, 3.0));

    // limbs (geometry is fixed by its own generator so every stage sees it)
    let mut g = Rng::new(9);
    let limb = Limb {
        pts: wobble(&[(468.0, 452.0), (520.0, 392.0), (570.0, 318.0), (606.0, 240.0), (660.0, 172.0), (694.0, 96.0), (748.0, 30.0), (772.0, -30.0)], 5.0, &mut g),
        w: vec![44.0, 36.0, 32.0, 28.0, 25.0, 22.0, 20.0, 19.0],
    };
    let bough = Limb {
        pts: wobble(&[(640.0, 205.0), (700.0, 190.0), (760.0, 196.0), (830.0, 170.0), (900.0, 162.0), (960.0, 128.0), (1030.0, 118.0)], 4.0, &mut g),
        w: vec![13.0, 11.0, 10.0, 8.0, 7.0, 6.0, 5.0],
    };
    let high = Limb {
        pts: wobble(&[(452.0, 150.0), (420.0, 110.0), (372.0, 76.0), (340.0, 30.0), (300.0, -20.0)], 3.0, &mut g),
        w: vec![22.0, 18.0, 16.0, 14.0, 13.0],
    };
    // the broken stub on the shady... no, the lit side, low down
    let stub = Limb {
        pts: vec![(tr.cx(655.0) - 30.0, 655.0), (tr.cx(655.0) - 70.0, 625.0), (tr.cx(655.0) - 98.0, 596.0)],
        w: vec![30.0, 22.0, 17.0],
    };
    let limbs = [&limb, &bough, &high, &stub];
    let mut twig_list: Vec<(Vec<P>, f32)> = Vec::new();
    // twigs off the bough and the limb, crossing the sky
    for (i, p) in bough.pts.iter().enumerate().skip(1) {
        let up = if i % 2 == 0 { -1.2 } else { -0.4 };
        twigs(*p, up + g.range(-0.3, 0.3), 70.0 + 40.0 * g.f(), 3.2, 2, &mut g, &mut twig_list);
        if i > 2 {
            twigs((p.0 + 10.0, p.1 + 3.0), 0.5 + g.range(-0.3, 0.3), 50.0 + 30.0 * g.f(), 2.6, 1, &mut g, &mut twig_list);
        }
    }
    for (i, p) in limb.pts.iter().enumerate().skip(2) {
        if i % 2 == 0 {
            twigs(*p, -2.4 + g.range(-0.3, 0.3), 60.0 + 30.0 * g.f(), 3.0, 2, &mut g, &mut twig_list);
        } else {
            twigs(*p, -0.2 + g.range(-0.3, 0.3), 70.0 + 40.0 * g.f(), 3.4, 2, &mut g, &mut twig_list);
        }
    }
    for p in high.pts.iter().skip(1) {
        twigs(*p, -2.0 + g.range(-0.4, 0.4), 55.0 + 30.0 * g.f(), 2.8, 2, &mut g, &mut twig_list);
    }
    // a few epicormic shoots straight off the trunk (oaks carry them)
    for &y in &[300.0f32, 350.0, 560.0, 780.0] {
        let x = tr.right(y) - 1.0;
        twigs((x, y), -0.5 + g.range(-0.3, 0.3), 26.0 + 14.0 * g.f(), 2.2, 1, &mut g, &mut twig_list);
    }
    for &y in &[250.0f32, 520.0] {
        let x = tr.left(y) + 1.0;
        twigs((x, y), -2.5 + g.range(-0.3, 0.3), 24.0 + 12.0 * g.f(), 2.0, 1, &mut g, &mut twig_list);
    }

    // snow surface around the base: the snow banks up against the trunk
    let bank = move |x: f32| {
        let d = x - tr.cx(1090.0);
        1098.0 - 16.0 * (-(d / 90.0).powi(2)).exp() + 0.012 * d.abs()
    };

    // masks
    let sky_m = Mask::from_fn(f, move |x, y| 1.0 - smoothstep(horizon(x) - 1.0, horizon(x) + 3.0, y));
    let snow_m = Mask::from_fn(f, move |x, y| smoothstep(horizon(x) - 1.5, horizon(x) + 1.5, y));
    let trunk_m = Mask::from_fn(f, move |x, y| {
        if y > 1160.0 {
            return 0.0;
        }
        let (l, r) = (tr.left(y), tr.right(y));
        smoothstep(l - 0.7, l + 0.7, x) * (1.0 - smoothstep(r - 0.7, r + 0.7, x))
    });
    let limbs_m = Mask::from_fn(f, |x, y| {
        let mut m: f32 = 0.0;
        for l in &limbs {
            let (d, _) = l.sd(x, y);
            m = m.max(1.0 - smoothstep(-0.7, 0.7, d));
        }
        m
    });
    let tree_m = trunk_m.clone().union(&limbs_m);

    // ------------------------------------------------------------ drawing
    if o.stage("drawing", &mut c, &mut rng) {
        // 2H first, searching; the horizon ruled
        let hard = Lead::pencil("2H").unwrap();
        let soft = Lead::pencil("HB").unwrap();
        let hz: Vec<P> = vec![(0.0, horizon(0.0)), (1000.0, horizon(1000.0))];
        c.draw(&hard, &hand_line(&hz, &[0.25], false, true, 0.0, 1), 0.1, 1);
        for (k, side) in [0, 1].iter().enumerate() {
            let pts: Vec<P> = (0..40).map(|i| {
                let y = -10.0 + i as f32 * 29.0;
                (if *side == 0 { tr.left(y) } else { tr.right(y) }, y)
            }).collect();
            c.draw(&hard, &hand_line(&pts, &[0.2, 0.3, 0.25], true, false, 0.6, 10 + k as u64), 0.2, 10 + k as u64);
            c.draw(&soft, &hand_line(&pts, &[0.35, 0.45, 0.3], true, false, 0.4, 20 + k as u64), 0.2, 20 + k as u64);
        }
        for (k, l) in limbs.iter().enumerate() {
            c.draw(&soft, &hand_line(&l.pts, &[0.35, 0.25], true, false, 0.5, 30 + k as u64), 0.3, 30 + k as u64);
        }
        // the snow's edge against the trunk and the cast shadow, lightly
        let bk: Vec<P> = (0..12).map(|i| {
            let x = 300.0 + i as f32 * 25.0;
            (x, bank(x))
        }).collect();
        c.draw(&hard, &hand_line(&bk, &[0.2], true, false, 0.6, 41), 0.2, 41);
    }

    // ------------------------------------------------------------ sky
    // cool, pale blue-grey above, through a greyish pearl to a straw glow
    // low down and a faint rose just over the snow
    let sky = move |_x: f32, y: f32| {
        let t = (y / HZ).clamp(0.0, 1.0);
        gradient(
            &[(0.0, hex("#8e9cb2")), (0.3, hex("#a8b2bf")), (0.58, hex("#c9cbc5")), (0.8, hex("#e2d8bd")), (0.93, hex("#ecd5a8")), (1.0, hex("#e9c9a4"))],
            t,
            Mix::Light,
        )
    };
    if o.stage("sky", &mut c, &mut rng) {
        // thin lay-in, a little darker and duller than it will end, in long
        // horizontal elbow strokes, fused top to bottom
        let lay = st
            .broad()
            .palette(&sky_pal)
            .color(move |x, y| mix(sky(x, y), hex("#7d7f86"), 0.06, Mix::Light))
            .angle(|_, y| 0.03 * (y / 300.0).sin())
            .coverage(4.2)
            .length(90.0, 240.0)
            .medium(0.3);
        c.work(&sky_m, &lay, 101);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.angle(|_, _| 0.0), 102);
        }
        // first stipple into the wet: breaks the strokes, keeps the tones
        let s1 = Stipple::new(Tool::stippler(4.2)).mixed(&sky_pal, 0.45).color(sky).coverage(|_, _| 2.0).pressure(0.5, 0.85).dips(20, 0.4, 0.5);
        c.stipple(&sky_m, &s1, 103);
        c.dry();
        // second, dry: finer, lighter toward the glow
        let glow = move |x: f32, y: f32| mix(sky(x, y), hex("#f3e4bd"), 0.05 + 0.25 * smoothstep(380.0, HZ, y), Mix::Light);
        let s2 = Stipple::new(Tool::stippler(2.6)).mixed(&sky_pal, 0.5).color(glow).coverage(|_, y| 0.6 + 1.6 * smoothstep(200.0, HZ, y)).pressure(0.45, 0.8).dips(24, 0.35, 0.6);
        c.stipple(&sky_m, &s2, 104);
        c.dry();
    }

    // ------------------------------------------------------------ far woods
    let woods_top = f.per_column(move |x| {
        let n = Fbm::new(91, 5, 26.0);
        let body = smoothstep(560.0, 640.0, x) * (1.0 - smoothstep(960.0, 1010.0, x));
        let left = smoothstep(0.0, 40.0, x) * (1.0 - smoothstep(150.0, 230.0, x));
        let hgt = body * (9.0 + 6.0 * n.get(x, 0.0).abs() * 2.0) + left * (5.0 + 3.0 * n.get(x, 9.0));
        horizon(x) - hgt.max(0.0)
    });
    let woods_m = Mask::from_fn(f, move |x, y| {
        let top = woods_top(x);
        if top >= horizon(x) - 0.5 {
            return 0.0;
        }
        smoothstep(top - 0.6, top + 0.6, y) * (1.0 - smoothstep(horizon(x) + 1.0, horizon(x) + 3.0, y))
    });
    if o.stage("far", &mut c, &mut rng) {
        // far woods: a thin blue-grey band hatched upright, barely darker
        // than the glow, lost in the haze
        let hd = st.hatch().palette(&sky_pal).color(|_, _| hex("#a49c9c")).angle(|_, _| -FRAC_PI_2).length(3.0, 8.0).coverage(3.0).clip(true);
        c.work(&woods_m, &hd, 201);
        // single trees poking above the band: tiny upright ticks
        let mut b = Held::new(Tool::round_sable(1.2), 202);
        for i in 0..70 {
            let x = 570.0 + rng.f() * 390.0;
            let top = woods_top(x);
            if top >= horizon(x) - 2.0 {
                continue;
            }
            if i % 8 == 0 {
                b.reload(sky_pal.paint(hex("#958d90"), 0.3), 0.5);
            }
            let ht = 3.0 + 6.0 * rng.f();
            c.drag(&mut b, &Gesture::line((x, top + 3.0), (x + rng.range(-0.5, 0.5), top - ht)).pressure(0.5, 0.0).ramps(0.05, 0.7), None);
        }
        c.dry();
    }

    // ------------------------------------------------------------ snow
    let drift = Fbm::new(111, 4, 160.0);
    // the trunk's shadow comes toward us and to the right (the sun is low,
    // behind and left of the tree)
    let shadow = move |x: f32, y: f32| {
        if y < 1070.0 {
            return 0.0;
        }
        let t = (y - 1080.0) / 170.0;
        let axis = tr.cx(1090.0) + 12.0 + t * 210.0 + 10.0 * drift.get(y * 0.2, 7.0);
        let half = 48.0 + t * 30.0;
        let d = (x - axis).abs();
        (1.0 - smoothstep(half - 10.0, half + 8.0, d)) * smoothstep(1070.0, 1100.0, y)
    };
    if o.stage("snow", &mut c, &mut rng) {
        // lit snow: lead white warmed by the evening; far snow takes the
        // sky's glow and loses contrast, the foreground is whiter and cooler,
        // soft blue in the hollows of the drift
        let snow_col = move |x: f32, y: f32| {
            let d = ((y - HZ) / (h - HZ)).clamp(0.0, 1.0);
            let lit = mix(hex("#e7dccb"), hex("#ece8e0"), smoothstep(0.1, 0.6, d), Mix::Light);
            let hollow = (0.5 + 0.5 * drift.get(x * 0.5, y * 2.2)).powf(2.0) * smoothstep(0.05, 0.4, d);
            mix(lit, hex("#b7bccb"), 0.55 * hollow, Mix::Light)
        };
        let lay = st
            .body()
            .palette(&snow_pal)
            .color(snow_col)
            .angle(move |x, y| 0.04 * drift.get(x, y) + 0.02)
            .length(30.0, 90.0)
            .curve(0.05, 0.3)
            .coverage(3.5)
            .medium(0.15);
        c.work(&snow_m, &lay, 301);
        // slight impasto in the foreground snow (NG p.50): a second, stiffer
        // load in short strokes near us
        let fore = snow_m.clone().mul_fn(|_, y| smoothstep(1080.0, 1200.0, y));
        let imp = st.body().palette(&snow_pal).color(move |x, y| shift(snow_col(x, y), 0.02, 0.0, 0.0)).angle(move |x, y| 0.1 * drift.get(x, y)).length(15.0, 40.0).coverage(1.2).medium(0.05);
        c.work(&fore, &imp, 302);
        // the cast shadow, wet into the wet: blue-violet, cooler near the tree
        let sh_m = Mask::from_fn(f, move |x, y| shadow(x, y)).mul(&snow_m);
        let sh = st.body().palette(&snow_pal).color_over(|_, y, u| shift(u, -0.13 + 0.03 * smoothstep(1100.0, 1250.0, y), 0.0, -0.035)).angle(|_, _| 0.9).length(25.0, 60.0).coverage(2.4).medium(0.2);
        c.work(&sh_m, &sh, 303);
        c.dry();
    }

    // ------------------------------------------------------------ trunk
    let bark = Fbm::new(121, 5, 70.0);
    if o.stage("trunk", &mut c, &mut rng) {
        // a thin warm underpainting over the dry sky (one coat, umber)
        let under = st.body().palette(&bark_pal).color(|_, _| hex("#4a3d31")).angle(move |_, y| (1.0f32).atan2(tr.slope(y))).length(30.0, 80.0).coverage(2.5).medium(0.4).clip(true);
        c.work(&tree_m, &under, 401);
        c.dry();
        // body: dark, cooler to the right (away from the light), warm grey
        // on the left where the glow catches the bark
        let body_col = move |x: f32, y: f32| {
            let a = tr.across(x, y);
            let lit = smoothstep(0.1, -0.9, a);
            let n = 0.5 + 0.5 * bark.get(x * 3.0, y * 0.5);
            let dark = mix(hex("#2b2622"), hex("#3a322b"), n, Mix::Light);
            mix(dark, hex("#5d544a"), 0.75 * lit, Mix::Light)
        };
        let body = st.body().palette(&bark_pal).color(body_col).angle(move |_, y| (1.0f32).atan2(tr.slope(y))).length(25.0, 70.0).coverage(3.2).medium(0.15).clip(true);
        c.work(&trunk_m, &body, 402);
        // limbs: along their own direction
        let lcol = move |x: f32, y: f32| {
            let n = 0.5 + 0.5 * bark.get(x * 2.0, y * 2.0);
            mix(hex("#2a2521"), hex("#3e362f"), n, Mix::Light)
        };
        for (k, l) in limbs.iter().enumerate() {
            let lm = Mask::from_fn(f, |x, y| 1.0 - smoothstep(-0.7, 0.7, l.sd(x, y).0));
            let hd = st.body().palette(&bark_pal).color(lcol).angle(|x, y| l.sd(x, y).1).length(15.0, 50.0).coverage(3.2).medium(0.15).clip(true);
            c.work(&lm, &hd, 410 + k as u64);
        }
        c.dry();
    }

    // ------------------------------------------------------------ bark
    if o.stage("bark", &mut c, &mut rng) {
        // fissures: long dark wavy lines up the trunk, splitting and joining
        let mut rg = Held::new(Tool::round_sable(1.8), 501);
        let fiss = bark_pal.paint(hex("#1c1917"), 0.1);
        for k in 0..90 {
            let a0 = rng.range(-0.95, 0.95);
            let y0 = rng.range(-20.0, 1120.0);
            let len = rng.range(60.0, 220.0);
            let mut pts = Vec::new();
            let mut a = a0;
            let mut y = y0;
            while y > y0 - len {
                let l = tr.left(y);
                let r = tr.right(y);
                pts.push((l + (r - l) * (0.5 + 0.5 * a), y));
                a = (a + rng.range(-0.06, 0.06)).clamp(-0.97, 0.97);
                y -= 12.0;
            }
            if pts.len() < 2 {
                continue;
            }
            if k % 6 == 0 {
                rg.reload(fiss, 0.8);
            }
            let p = rng.range(0.35, 0.7);
            c.drag(&mut rg, &Gesture::new(pts).pressure(p, p * 0.6).ramps(0.1, 0.3).shake(0.5), Some(&trunk_m));
        }
        // plates on the lit side: short, lean, dry-brush strokes of warm
        // grey that catch only the tops of the weave
        let mut db = Held::new(Tool { lay: 0.5, ..Tool::filbert(5.0) }, 502);
        let plate = bark_pal.paint(hex("#7a6e60"), 0.05);
        for k in 0..260 {
            let y = rng.range(-10.0, 1110.0);
            let a = rng.range(-1.0, 0.15);
            let l = tr.left(y);
            let r = tr.right(y);
            let x = l + (r - l) * (0.5 + 0.5 * a);
            let len = rng.range(10.0, 34.0);
            let s = tr.slope(y);
            if k % 5 == 0 {
                db.reload(plate, 0.25);
            }
            c.drag(&mut db, &Gesture::line((x, y), (x - s * len, y - len)).pressure(0.3, 0.2).shake(0.4), Some(&trunk_m));
        }
        // the lit edge: the evening catches the left rim of the trunk
        let mut rim = Held::new(Tool::round_sable(2.4), 503);
        let rimp = bark_pal.paint(hex("#8f8272"), 0.1);
        let mut y = 1080.0;
        while y > -20.0 {
            let len = rng.range(25.0, 70.0);
            let pts: Vec<P> = (0..6).map(|i| {
                let yy = y - len * i as f32 / 5.0;
                (tr.left(yy) + 2.2 + rng.range(-0.5, 0.5), yy)
            }).collect();
            rim.reload(rimp, 0.5);
            c.drag(&mut rim, &Gesture::new(pts).pressure(0.35, 0.25).ramps(0.2, 0.4).shake(0.3), Some(&trunk_m));
            y -= len + rng.range(-5.0, 20.0);
        }
        // lichen: a few grey-green touches on the lit, north... lit side
        let mut lb = Held::new(Tool::stippler(3.0), 504);
        for _ in 0..40 {
            let y = rng.range(500.0, 1060.0);
            let a = rng.range(-0.9, -0.2);
            let (l, r) = (tr.left(y), tr.right(y));
            let x = l + (r - l) * (0.5 + 0.5 * a);
            lb.reload(bark_pal.paint(hex("#7c7c66"), 0.2), 0.3);
            for _ in 0..4 {
                c.touch(&mut lb, &Touch::at(x + rng.range(-4.0, 4.0), y + rng.range(-4.0, 4.0)).pressure(rng.range(0.3, 0.6)), Some(&trunk_m));
            }
        }
        c.dry();
    }

    // ------------------------------------------------------------ twigs
    if o.stage("twigs", &mut c, &mut rng) {
        let twig = bark_pal.paint(hex("#2e2824"), 0.1);
        let mut sab = Held::new(Tool { point: 1.0, ..Tool::round_sable(3.0) }, 601);
        let mut rgr = Held::new(Tool { point: 1.0, ..Tool::rigger(1.0) }, 602);
        for (pts, w) in &twig_list {
            let b = if *w > 2.5 { &mut sab } else { &mut rgr };
            b.reload(twig, 0.8);
            let p0 = (b.tool.pressure_for(*w)).clamp(0.2, 0.95);
            c.drag(b, &Gesture::new(pts.clone()).pressure(p0, 0.0).ramps(0.03, 0.75).shake(0.5), None);
        }
        c.dry();
    }

    // ------------------------------------------------------------ base
    if o.stage("base", &mut c, &mut rng) {
        // snow banked against the foot of the trunk, over the roots: the
        // lit left side warm, the right in blue shadow; the edge ragged
        let bm = Mask::from_fn(f, move |x, y| {
            let top = bank(x) + 2.5 * drift.get(x * 3.0, 0.0);
            smoothstep(top - 1.0, top + 1.0, y) * (1.0 - smoothstep(horizon(x) - 1.0, horizon(x), 0.0))
        })
        .mul_fn(|x, _| 1.0 - smoothstep(220.0, 300.0, (x - 440.0).abs()));
        let bcol = move |x: f32, y: f32| {
            let a = x - tr.cx(y.min(1100.0));
            let sh = smoothstep(-10.0, 30.0, a) * 0.7 + 0.3 * shadow(x, y);
            mix(hex("#ebe3d6"), hex("#a9afc3"), sh.min(1.0), Mix::Light)
        };
        let hd = st.body().palette(&snow_pal).color(bcol).angle(|x, _| if x < 440.0 { -0.25 } else { 0.25 }).length(10.0, 35.0).coverage(3.0).medium(0.08).clip(true);
        c.work(&bm, &hd, 701);
        // the hollow where the snow has drawn back from the bark: a dark line
        // under the lip of the snow, broken
        let mut sab = Held::new(Tool::round_sable(2.0), 702);
        let dark = bark_pal.paint(hex("#23201e"), 0.1);
        let mut x = tr.left(1090.0) - 6.0;
        while x < tr.right(1090.0) + 6.0 {
            let len = rng.range(8.0, 22.0);
            sab.reload(dark, 0.5);
            let y0 = bank(x) + 1.0;
            let y1 = bank(x + len) + 1.0;
            c.drag(&mut sab, &Gesture::line((x, y0 - 1.5), (x + len, y1 - 1.5)).pressure(0.5, 0.2).shake(0.6), None);
            x += len + rng.range(2.0, 10.0);
        }
        // snow caught on the stub's broken top and in the fork
        let mut wb = Held::new(Tool::round_sable(4.0), 703);
        let white = snow_pal.paint(hex("#eee9e0"), 0.05);
        let caps = [(stub.pts[2], 12.0f32), (stub.pts[1], 16.0), ((tr.cx(462.0) + 22.0, 458.0), 20.0), ((tr.left(1030.0) + 8.0, 1034.0), 14.0)];
        for (p, len) in caps {
            wb.reload(white, 0.8);
            c.drag(&mut wb, &Gesture::new(vec![(p.0 - len * 0.5, p.1 - 4.0), (p.0, p.1 - 6.5), (p.0 + len * 0.5, p.1 - 4.5)]).pressure(0.7, 0.4).ramps(0.15, 0.4).shake(0.3), None);
        }
        // the broken end of the stub: pale splintered wood
        let wood = bark_pal.paint(hex("#9a8a70"), 0.1);
        let e = stub.pts[2];
        let mut sp = Held::new(Tool::round_sable(1.6), 704);
        for k in 0..4 {
            sp.reload(wood, 0.5);
            let a = -2.4 + k as f32 * 0.22;
            c.drag(&mut sp, &Gesture::new(vec![(e.0 + 4.0, e.1 + 4.0), (e.0 + a.cos() * 9.0, e.1 + a.sin() * 9.0)]).pressure(0.5, 0.0).ramps(0.05, 0.7), None);
        }
        c.dry();
    }

    // ------------------------------------------------------------ grass
    if o.stage("grass", &mut c, &mut rng) {
        // dead grass and a few weed stems through the snow, flicked up last
        let mut rgr = Held::new(Tool { point: 1.0, ..Tool::rigger(1.0) }, 801);
        let mut sab = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.6) }, 802);
        let straw = [hex("#8a7248"), hex("#6d5a3c"), hex("#4e4232"), hex("#a08a5e")];
        // clumps: near the trunk and scattered, smaller with distance
        let mut clumps: Vec<(f32, f32, f32)> = vec![(330.0, 1102.0, 1.0), (560.0, 1106.0, 0.9), (610.0, 1118.0, 0.7), (270.0, 1150.0, 1.1), (180.0, 1190.0, 1.3), (820.0, 1170.0, 1.1), (900.0, 1225.0, 1.3)];
        for _ in 0..16 {
            let x = rng.range(20.0, 980.0);
            let y = rng.range(HZ + 15.0, 1240.0);
            let s = 0.3 + 1.0 * ((y - HZ) / (h - HZ));
            clumps.push((x, y, s));
        }
        for (cx, cy, s) in clumps {
            let n = (6.0 + 10.0 * s) as usize;
            for i in 0..n {
                let b = if i % 3 == 0 { &mut sab } else { &mut rgr };
                b.reload(snow_pal.paint(straw[(rng.f() * 4.0) as usize % 4], 0.1), 0.6);
                let x = cx + rng.range(-14.0, 14.0) * s;
                let y = cy + rng.range(-3.0, 3.0) * s;
                let ht = (10.0 + 30.0 * rng.f()) * s;
                let lean = rng.range(-0.6, 0.5);
                let pts = vec![(x, y), (x + lean * ht * 0.3, y - ht * 0.55), (x + lean * ht, y - ht)];
                c.drag(b, &Gesture::new(pts).pressure(0.6, 0.0).ramps(0.05, 0.85).shake(0.5), None);
            }
            // a weed stem with a seed head
            if s > 0.8 && rng.f() < 0.6 {
                let x = cx + rng.range(-8.0, 8.0);
                let ht = 50.0 + 30.0 * rng.f();
                sab.reload(snow_pal.paint(hex("#3e342a"), 0.1), 0.6);
                let top = (x + rng.range(-8.0, 8.0), cy - ht);
                c.drag(&mut sab, &Gesture::new(vec![(x, cy), ((x + top.0) * 0.5 + 2.0, cy - ht * 0.5), top]).pressure(0.45, 0.2).ramps(0.05, 0.3).shake(0.4), None);
                for _ in 0..5 {
                    c.touch(&mut sab, &Touch::at(top.0 + rng.range(-2.5, 2.5), top.1 + rng.range(-3.0, 3.0)).pressure(0.5), None);
                }
            }
        }
        // a thin cool line of shadow under each near clump would come here
        let _ = PI;
    }

    o.end(&mut c, &mut rng);
    c.relief(st.relief.0, st.relief.1);
    o.save(&mut c);
}
