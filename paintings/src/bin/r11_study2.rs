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
        let flare = 30.0 * (-(1095.0 - y).max(0.0) / 75.0).exp();
        // a swelling where the limb leaves (the collar)
        let collar = 7.0 * (-((y - 440.0) / 40.0).powi(2)).exp();
        36.0 + 22.0 * t + flare + collar
    }
    fn left(&self, y: f32) -> f32 {
        // slow swellings, the callus lip round the old scar
        let callus = 6.0 * (-((y - 640.0) / 45.0).powi(2)).exp();
        self.cx(y) - self.hw(y) - 2.6 * self.bumps_l.get(3.0, y) - 1.2 * self.bumps_l.get(40.0, y * 3.0) - 4.5 * self.bumps_l.get(0.0, y * 0.3) - callus
    }
    fn right(&self, y: f32) -> f32 {
        // and a burl low on the shadow side
        let burl = 9.0 * (-((y - 815.0) / 32.0).powi(2)).exp();
        self.cx(y) + self.hw(y) + 2.2 * self.bumps_r.get(9.0, y) + 1.0 * self.bumps_r.get(50.0, y * 3.0) + 4.0 * self.bumps_r.get(0.0, y * 0.3) + burl
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
    /// distance from (x, y) to the limb's edge (negative inside), the
    /// direction of the nearest segment, and where across the limb the point
    /// lies (+1 on the upper edge, -1 on the lower)
    fn sd(&self, x: f32, y: f32) -> (f32, f32, f32) {
        let mut best = (f32::MAX, 0.0, 0.0);
        for i in 0..self.pts.len() - 1 {
            let (a, b) = (self.pts[i], self.pts[i + 1]);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let l2 = dx * dx + dy * dy;
            let t = (((x - a.0) * dx + (y - a.1) * dy) / l2).clamp(0.0, 1.0);
            let (px, py) = (a.0 + dx * t, a.1 + dy * t);
            let d = ((x - px).powi(2) + (y - py).powi(2)).sqrt();
            let r = 0.5 * (self.w[i] + (self.w[i + 1] - self.w[i]) * t);
            if d - r < best.0 {
                let ln = l2.sqrt();
                let (mut nx, mut ny) = (dy / ln, -dx / ln);
                if ny > 0.0 {
                    nx = -nx;
                    ny = -ny;
                }
                best = (d - r, dy.atan2(dx), ((x - px) * nx + (y - py) * ny) / r);
            }
        }
        best
    }
    /// The same limb through a smooth curve (Catmull–Rom), `k` points per span.
    fn smooth(self, k: usize) -> Limb {
        let n = self.pts.len();
        let at = |i: isize| self.pts[i.clamp(0, n as isize - 1) as usize];
        let mut pts = Vec::new();
        let mut w = Vec::new();
        for i in 0..n - 1 {
            let (p0, p1, p2, p3) = (at(i as isize - 1), at(i as isize), at(i as isize + 1), at(i as isize + 2));
            for j in 0..k {
                let t = j as f32 / k as f32;
                let (t2, t3) = (t * t, t * t * t);
                let cr = |a: f32, b: f32, c: f32, d: f32| 0.5 * (2.0 * b + (-a + c) * t + (2.0 * a - 5.0 * b + 4.0 * c - d) * t2 + (-a + 3.0 * b - 3.0 * c + d) * t3);
                pts.push((cr(p0.0, p1.0, p2.0, p3.0), cr(p0.1, p1.1, p2.1, p3.1)));
                w.push(self.w[i] + (self.w[i + 1] - self.w[i]) * t);
            }
        }
        pts.push(self.pts[n - 1]);
        w.push(self.w[n - 1]);
        Limb { pts, w }
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
        a += rng.range(-0.55, 0.55);
        // oak shoots bend up toward the light a little
        a += 0.08 * ((-FRAC_PI_2) - a).sin().signum() * 0.5;
        p = (p.0 + a.cos() * seg * rng.range(0.75, 1.2), p.1 + a.sin() * seg * rng.range(0.75, 1.2));
        pts.push(p);
    }
    out.push((pts.clone(), w));
    if depth == 0 {
        return;
    }
    let kids = 2 + (rng.f() * 2.5) as usize;
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
    }
    .smooth(6);
    let bough = Limb {
        pts: wobble(&[(640.0, 205.0), (700.0, 190.0), (760.0, 196.0), (830.0, 170.0), (900.0, 162.0), (960.0, 128.0), (1030.0, 118.0)], 4.0, &mut g),
        w: vec![13.0, 11.0, 10.0, 8.0, 7.0, 6.0, 5.0],
    }
    .smooth(6);
    let high = Limb {
        pts: wobble(&[(452.0, 150.0), (420.0, 110.0), (372.0, 76.0), (340.0, 30.0), (300.0, -20.0)], 3.0, &mut g),
        w: vec![22.0, 18.0, 16.0, 14.0, 13.0],
    }
    .smooth(6);
    // ckpt: from twigs
    let mut twig_list: Vec<(Vec<P>, f32)> = Vec::new();
    // twigs off the bough and the limb, crossing the sky: oak shoots, short
    // and angular, crowding toward the ends
    for (i, p) in bough.pts.iter().step_by(6).enumerate().skip(1) {
        let up = if i % 2 == 0 { -1.25 } else { -0.5 };
        twigs(*p, up + g.range(-0.3, 0.3), 60.0 + 40.0 * g.f(), 4.5, 3, &mut g, &mut twig_list);
        twigs((p.0 + 12.0, p.1 + 2.0), 0.55 + g.range(-0.3, 0.3), 40.0 + 30.0 * g.f(), 3.4, 2, &mut g, &mut twig_list);
    }
    for (i, p) in limb.pts.iter().step_by(6).enumerate().skip(2) {
        if i % 2 == 0 {
            twigs(*p, -2.3 + g.range(-0.3, 0.3), 55.0 + 30.0 * g.f(), 4.5, 3, &mut g, &mut twig_list);
        } else {
            twigs(*p, -0.25 + g.range(-0.3, 0.3), 60.0 + 40.0 * g.f(), 5.0, 3, &mut g, &mut twig_list);
        }
    }
    for p in high.pts.iter().step_by(6).skip(1) {
        twigs(*p, -2.1 + g.range(-0.4, 0.4), 50.0 + 30.0 * g.f(), 4.0, 3, &mut g, &mut twig_list);
        twigs(*p, -1.2 + g.range(-0.4, 0.4), 35.0 + 20.0 * g.f(), 3.0, 2, &mut g, &mut twig_list);
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
    // ckpt: end

    // snow surface around the base: the snow banks up against the trunk
    // (a tongue of drift heaped on the windward left, a root hump under the
    // snow to the right, small lobes along the lip)
    let lobes = Fbm::new(131, 4, 40.0);
    let bank = move |x: f32| {
        let d = x - tr.cx(1090.0);
        1104.0 - 30.0 * (-(d / 58.0).powi(2)).exp() - 14.0 * (-((d + 85.0) / 45.0).powi(2)).exp() - 9.0 * (-((d - 72.0) / 26.0).powi(2)).exp()
            + 0.02 * d.abs()
            // it climbs the windward side of the trunk and falls away from
            // the root that runs out under it on the right
            - 12.0 * (1.0 - smoothstep(-70.0, 30.0, d)) * (-(d / 110.0).powi(2)).exp()
            + 7.0 * (-((d - 38.0) / 16.0).powi(2)).exp()
            + 2.0 * lobes.get(x, 0.0) - 4.0 * lobes.get(x * 0.35, 5.0).max(0.0).powf(1.5)
    };

    // a root running out from the foot on the shadow side, diving under the
    // snow
    let root = Limb {
        pts: vec![(tr.right(1050.0) - 26.0, 1050.0), (tr.right(1070.0) + 6.0, 1072.0), (tr.right(1078.0) + 26.0, 1088.0), (tr.right(1080.0) + 44.0, 1108.0), (tr.right(1080.0) + 62.0, 1135.0)],
        w: vec![44.0, 30.0, 24.0, 22.0, 21.0],
    }
    .smooth(4);
    let limbs = [&limb, &bough, &high, &root];
    let under_snow = move |x: f32, y: f32| 1.0 - smoothstep(bank(x) + 0.5, bank(x) + 2.0, y);
    // masks
    let sky_m = Mask::from_fn(f, move |x, y| 1.0 - smoothstep(horizon(x) - 1.0, horizon(x) + 3.0, y));
    let snow_m = Mask::from_fn(f, move |x, y| smoothstep(horizon(x) - 1.5, horizon(x) + 1.5, y));
    let trunk_m = Mask::from_fn(f, move |x, y| {
        let (l, r) = (tr.left(y), tr.right(y));
        smoothstep(l - 0.7, l + 0.7, x) * (1.0 - smoothstep(r - 0.7, r + 0.7, x)) * (1.0 - smoothstep(bank(x) + 0.5, bank(x) + 2.0, y))
    });
    let limbs_m = Mask::from_fn(f, |x, y| {
        let mut m: f32 = 0.0;
        for l in &limbs {
            let (d, _, _) = l.sd(x, y);
            m = m.max(1.0 - smoothstep(-0.7, 0.7, d));
        }
        m * under_snow(x, y)
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
        if y < 1040.0 {
            return 0.0;
        }
        let t = ((y - 1075.0) / 175.0).max(0.0);
        let axis = tr.cx(1090.0) + 8.0 + t * 190.0 + 9.0 * drift.get(y * 0.3, 7.0);
        let half = 44.0 + t * 26.0;
        // the shadow's edges ride over the drift's small humps
        let d = (x - axis).abs() + 4.0 * drift.get(x * 1.2, y * 0.8) + 2.5 * lobes.get(x * 0.7, y);
        let fade = 1.0 - 0.35 * t; // it thins as it comes toward us
        (1.0 - smoothstep(half - 16.0, half + 10.0, d)) * smoothstep(bank(x) - 4.0, bank(x) + 12.0, y) * fade
    };
    let snow_col = move |x: f32, y: f32| {
        let d = ((y - HZ) / (h - HZ)).clamp(0.0, 1.0);
        let lit = mix(hex("#ddd2c2"), hex("#d6d3d2"), smoothstep(0.1, 0.6, d), Mix::Light);
        // the drift's broad swells: warm where a crest faces the glow,
        // lavender-blue in the troughs behind it
        let sw = drift.get(x * 0.45, y * 2.4);
        let hollow = (0.5 + 0.5 * sw).powf(1.6) * smoothstep(0.03, 0.35, d);
        let crest = smoothstep(-0.1, -0.5, sw) * smoothstep(0.05, 0.3, d);
        mix(mix(lit, hex("#e9dcc6"), 0.6 * crest, Mix::Light), hex("#a2a8bd"), 0.6 * hollow, Mix::Light)
    };
    if o.stage("snow", &mut c, &mut rng) {
        // lit snow: lead white warmed by the evening; far snow takes the
        // sky's glow and loses contrast, the foreground is whiter and cooler,
        // soft blue in the hollows of the drift
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
        let cx0 = tr.cx(1090.0);
        // how much of the mound is here: crisp along its lip, fading into the
        // field below and to the sides (so it is one snow with the field)
        let mound = move |x: f32, y: f32| {
            let below = y - bank(x);
            let side = (-((x - cx0) / 105.0).powi(2)).exp();
            smoothstep(-0.8, 0.8, below) * (-(below.max(0.0) / 38.0)).exp() * side
        };
        let bm = Mask::from_fn(f, move |x, y| (mound(x, y) * 1.6).min(1.0));
        let bcol = move |x: f32, y: f32| {
            let a = x - cx0;
            // the mound: its left flank faces the glow, its right turns away
            // into the tree's own shadow; blue deepest in the hollow against the bark
            let turn = smoothstep(-15.0, 50.0, a) * (1.0 - smoothstep(bank(x) + 10.0, bank(x) + 45.0, y));
            let close = (-(a / 60.0).powi(2)).exp() * (1.0 - smoothstep(bank(x), bank(x) + 14.0, y));
            let sh = (0.35 * turn + 0.2 * close).min(1.0);
            let lit = mix(snow_col(x, y), hex("#f2e6d2"), 0.6 * smoothstep(10.0, -70.0, a) * (1.0 - smoothstep(bank(x) + 5.0, bank(x) + 40.0, y)), Mix::Light);
            let m = mound(x, y);
            mix(snow_col(x, y), mix(lit, hex("#a7adc2"), sh, Mix::Light), m.min(1.0), Mix::Light)
        };
        // strokes follow the mound's surface: across it, turning down its flanks
        let hd = st.body().palette(&snow_pal).color(bcol).angle(move |x, _| { let a = (x - cx0) / 90.0; 0.45 * a.clamp(-1.0, 1.0) }).length(8.0, 28.0).coverage(3.5).medium(0.08).clip(true);
        c.work(&bm, &hd, 701);
        // the cast shadow, wet into the wet: blue-violet, cooler near the tree
        let sh_m = Mask::from_fn(f, move |x, y| shadow(x, y)).mul(&snow_m);
        let sh = st.body().palette(&snow_pal).color_over(|_, y, u| shift(u, -0.17 + 0.04 * smoothstep(1100.0, 1250.0, y), 0.004, -0.04)).angle(|_, _| 0.75).angle_jitter(0.2).length(40.0, 110.0).coverage(3.0).medium(0.2);
        c.work(&sh_m, &sh, 303);
        // fused with a soft badger along its length and across its edges,
        // wet into the wet snow, so the edge goes soft as it leaves the tree
        let fuse_m = Mask::from_fn(f, move |x, y| (shadow(x, y) * 3.0).min(1.0)).blur(18.0).mul(&snow_m);
        if let Some(b) = st.blend() {
            c.work(&fuse_m, &b.angle(|_, _| 0.75).length(60.0, 140.0).coverage(0.9).pressure(0.3, 0.4), 304);
        }
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
        // limbs: along their own direction, the upper side catching the
        // glow, the underside dark
        for (k, l) in limbs.iter().enumerate() {
            let lm = Mask::from_fn(f, |x, y| (1.0 - smoothstep(-0.7, 0.7, l.sd(x, y).0)) * under_snow(x, y));
            let lcol = |x: f32, y: f32| {
                let (_, _, a) = l.sd(x, y);
                let n = 0.5 + 0.5 * bark.get(x * 2.0, y * 2.0);
                let base = mix(hex("#27221f"), hex("#3a322c"), n, Mix::Light);
                mix(base, hex("#5a5046"), 0.8 * smoothstep(0.1, 0.9, a), Mix::Light)
            };
            let hd = st.body().palette(&bark_pal).color(lcol).angle(|x, y| l.sd(x, y).1).length(15.0, 50.0).coverage(3.2).medium(0.15).clip(true);
            c.work(&lm, &hd, 410 + k as u64);
        }
        // the trunk's body over the limbs' and root's starts, so they grow
        // out of it rather than lie on it
        let body = st.body().palette(&bark_pal).color(body_col).angle(move |_, y| (1.0f32).atan2(tr.slope(y))).length(25.0, 70.0).coverage(3.2).medium(0.15).clip(true);
        c.work(&trunk_m, &body, 402);
        // the fork: bark strokes pulled from the trunk out along the limb,
        // so the limb grows out of the trunk instead of lying on it
        let mut jb = Held::new(Tool::filbert(7.0), 420);
        for j in 0..14 {
            let off = rng.range(-0.8, 0.8);
            let start_y = limb.pts[0].1 + rng.range(20.0, 70.0);
            let s0 = (tr.cx(start_y) + rng.range(-10.0, 25.0), start_y);
            let mut pts = vec![s0];
            for i in (0..30).step_by(5) {
                let p = limb.pts[i];
                let r = 0.5 * limb.w[i];
                let (dx, dy) = (limb.pts[i + 1].0 - p.0, limb.pts[i + 1].1 - p.1);
                let ln = (dx * dx + dy * dy).sqrt();
                pts.push((p.0 + dy / ln * r * off, p.1 - dx / ln * r * off));
            }
            let col = mix(hex("#2b2622"), hex("#51483f"), smoothstep(0.0, 0.8, off), Mix::Light);
            jb.reload(bark_pal.paint(col, 0.15), 0.5);
            c.drag(&mut jb, &Gesture::new(pts).pressure(0.5, 0.25).ramps(0.2, 0.5).shake(0.4), Some(&tree_m));
            let _ = j;
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
        // the ridges between the fissures, on the lit side: long lean strokes
        // of warm grey dragged up the bark with a nearly dry filbert, so they
        // catch only the tops of the weave and break; more and lighter toward
        // the left, none on the shadow side
        let mut db = Held::new(Tool { lay: 0.7, ragged: 0.5, ..Tool::round_sable(3.2) }, 502);
        for k in 0..170 {
            let a = -1.0 + 1.2 * rng.f().powf(1.6);
            let y0 = rng.range(0.0, 1110.0);
            let len = rng.range(30.0, 110.0);
            let mut pts = Vec::new();
            let mut aa = a;
            let mut y = y0;
            while y > y0 - len {
                let (l, r) = (tr.left(y), tr.right(y));
                pts.push((l + (r - l) * (0.5 + 0.5 * aa), y));
                aa = (aa + rng.range(-0.04, 0.04)).clamp(-0.97, 0.3);
                y -= 10.0;
            }
            if pts.len() < 2 {
                continue;
            }
            let lit = smoothstep(0.3, -0.9, a);
            let col = mix(hex("#3b342d"), hex("#675b4e"), lit, Mix::Light);
            if k % 2 == 0 {
                db.reload(bark_pal.paint(col, 0.05), 0.3);
            }
            c.drag(&mut db, &Gesture::new(pts).pressure(rng.range(0.3, 0.55), 0.15).ramps(0.15, 0.3).shake(0.5), Some(&trunk_m));
        }
        // along the limbs: a few lean ridges on their upper sides and dark
        // cracks, following each limb
        for l in &limbs {
            let n = l.pts.len();
            for _ in 0..(n / 3) {
                let i0 = (rng.f() * (n - 8) as f32) as usize;
                let run = 4 + (rng.f() * 10.0) as usize;
                let lit = rng.f() < 0.6;
                let off = if lit { rng.range(0.2, 0.8) } else { rng.range(-0.6, 0.3) };
                let pts: Vec<P> = (i0..(i0 + run).min(n - 1)).map(|i| {
                    let p = l.pts[i];
                    let (dx, dy) = (l.pts[i + 1].0 - p.0, l.pts[i + 1].1 - p.1);
                    let ln = (dx * dx + dy * dy).sqrt().max(1e-3);
                    let (mut nx, mut ny) = (dy / ln, -dx / ln);
                    if ny > 0.0 { nx = -nx; ny = -ny; }
                    let r = 0.5 * l.w[i];
                    (p.0 + nx * r * off, p.1 + ny * r * off)
                }).collect();
                if pts.len() < 2 { continue; }
                if lit {
                    db.reload(bark_pal.paint(hex("#6c6052"), 0.05), 0.25);
                    c.drag(&mut db, &Gesture::new(pts).pressure(0.3, 0.15).ramps(0.2, 0.3).shake(0.5), Some(&tree_m));
                } else {
                    rg.reload(fiss, 0.6);
                    c.drag(&mut rg, &Gesture::new(pts).pressure(0.35, 0.2).ramps(0.2, 0.3).shake(0.5), Some(&tree_m));
                }
            }
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
        // where a limb was lost long ago: a healed scar, a swollen callus lip
        // around a dark hollow, the lip lit on its upper left
        let (sx, sy) = (tr.cx(640.0) - 14.0, 640.0);
        let mut sc = Held::new(Tool::round_sable(3.0), 505);
        sc.reload(bark_pal.paint(hex("#191614"), 0.1), 0.8);
        for k in 0..3 {
            let r = 5.0 - k as f32 * 1.5;
            let ring: Vec<P> = (0..9).map(|i| { let a = i as f32 / 8.0 * 2.0 * PI; (sx + a.cos() * r * 0.8, sy + a.sin() * r * 1.5) }).collect();
            c.drag(&mut sc, &Gesture::new(ring).pressure(0.8, 0.8).shake(0.3), Some(&trunk_m));
        }
        sc.reload(bark_pal.paint(hex("#7f7262"), 0.1), 0.6);
        let arc: Vec<P> = (0..7).map(|i| { let a = PI * 0.75 + i as f32 / 6.0 * PI * 0.95; (sx + a.cos() * 9.5, sy + a.sin() * 15.0) }).collect();
        c.drag(&mut sc, &Gesture::new(arc).pressure(0.55, 0.3).ramps(0.2, 0.4).shake(0.4), Some(&trunk_m));
        sc.reload(bark_pal.paint(hex("#221e1b"), 0.1), 0.6);
        let arc2: Vec<P> = (0..6).map(|i| { let a = -PI * 0.3 + i as f32 / 5.0 * PI * 0.9; (sx + a.cos() * 10.0, sy + a.sin() * 16.0) }).collect();
        c.drag(&mut sc, &Gesture::new(arc2).pressure(0.6, 0.3).ramps(0.2, 0.4).shake(0.4), Some(&trunk_m));
        // lichen: a few grey-green touches on the lit, north... lit side
        let mut lb = Held::new(Tool::stippler(3.0), 504);
        for _ in 0..14 {
            let y = rng.range(600.0, 1050.0);
            let a = rng.range(-0.9, -0.2);
            let (l, r) = (tr.left(y), tr.right(y));
            let x = l + (r - l) * (0.5 + 0.5 * a);
            lb.reload(bark_pal.paint(hex("#7c7c66"), 0.2), 0.3);
            for _ in 0..3 {
                c.touch(&mut lb, &Touch::at(x + rng.range(-3.0, 3.0), y + rng.range(-5.0, 5.0)).pressure(rng.range(0.2, 0.45)), Some(&trunk_m));
            }
        }
        c.dry();
    }

    // ------------------------------------------------------------ twigs
    if o.stage("twigs", &mut c, &mut rng) {
        let twig = bark_pal.paint(hex("#2e2824"), 0.1);
        let mut sab = Held::new(Tool { point: 1.0, ..Tool::round_sable(3.0) }, 601);
        let mut rgr = Held::new(Tool { point: 1.0, ..Tool::rigger(1.0) }, 602);
        let mut big = Held::new(Tool { point: 1.0, ..Tool::round_sable(5.5) }, 603);
        for (pts, w) in &twig_list {
            let b = if *w > 3.2 { &mut big } else if *w > 1.6 { &mut sab } else { &mut rgr };
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
        let cx0 = tr.cx(1090.0);
        let bcol = move |x: f32, y: f32| mix(mix(snow_col(x, y), hex("#efe3cf"), 0.5 * smoothstep(20.0, -40.0, x - cx0), Mix::Light), hex("#a3a9bf"), 0.6 * shadow(x, y).max(0.5 * smoothstep(0.0, 40.0, x - cx0)), Mix::Light);
        // the hollow where the snow has drawn back from the bark: a dark line
        // under the lip of the snow, broken
        let mut sab = Held::new(Tool::round_sable(2.0), 702);
        let dark = bark_pal.paint(hex("#23201e"), 0.1);
        let mut x = tr.cx(1080.0) - 5.0;
        while x < tr.right(1080.0) - 3.0 {
            let len = rng.range(5.0, 14.0);
            sab.reload(dark, 0.5);
            let y0 = bank(x) - 1.2;
            let y1 = bank(x + len) - 1.2;
            c.drag(&mut sab, &Gesture::new(vec![(x, y0), (x + len * 0.5, (y0 + y1) * 0.5 - rng.range(0.0, 1.0)), (x + len, y1)]).pressure(0.35, 0.1).ramps(0.2, 0.5).shake(0.6), None);
            x += len + rng.range(3.0, 14.0);
        }
        // the lip of the bank against the bark: short curved strokes of the
        // lit snow pulled up onto the trunk, broken, so the bark shows through
        // (long, flat, overlapping strokes laid along the line of the bank,
        // the brush lifting now and then a hair higher onto the bark)
        let mut lip = Held::new(Tool { lay: 0.9, ..Tool::filbert(6.0) }, 705);
        let mut x = tr.left(1080.0) - 16.0;
        while x < tr.right(1080.0) + 75.0 {
            let len = rng.range(14.0, 40.0);
            let col = bcol(x + len * 0.5, bank(x + len * 0.5) + 4.0);
            lip.reload(snow_pal.paint(shift(col, -0.02, 0.0, -0.005), 0.05), 0.6);
            let pts: Vec<P> = (0..5).map(|k| {
                let xx = x + len * k as f32 / 4.0;
                (xx, bank(xx) + 3.5 - rng.range(0.0, 2.2))
            }).collect();
            c.drag(&mut lip, &Gesture::new(pts).pressure(rng.range(0.35, 0.6), 0.25).ramps(0.3, 0.5).shake(0.6), None);
            x += len * rng.range(0.35, 0.7);
        }
        // here and there a little snow pushed up against the bark on the
        // windward left: short upward touches
        let mut tip = Held::new(Tool::round_sable(4.0), 707);
        for _ in 0..9 {
            let x = rng.range(tr.left(1070.0) + 2.0, tr.cx(1070.0) + 10.0);
            let y = bank(x) + 1.0;
            tip.reload(snow_pal.paint(shift(bcol(x, y + 3.0), -0.02, 0.0, 0.0), 0.05), 0.4);
            let ht = rng.range(1.0, 3.5);
            c.touch(&mut tip, &Touch::at(x, y - ht).pressure(rng.range(0.4, 0.7)).drag(rng.range(-1.0, 1.0), 1.5), None);
        }
        // a little snow caught in the bark's crevices just above the lip, on
        // the side it blew against
        let mut lip = Held::new(Tool::round_sable(1.4), 708);
        for _ in 0..5 {
            let y = rng.range(1015.0, 1070.0);
            let a = rng.range(-0.95, 0.1);
            let (l, r) = (tr.left(y), tr.right(y));
            let x = l + (r - l) * (0.5 + 0.5 * a);
            if y > bank(x) - 3.0 {
                continue;
            }
            lip.reload(snow_pal.paint(hex("#cfccca"), 0.05), 0.25);
            let len = rng.range(5.0, 11.0);
            c.drag(&mut lip, &Gesture::line((x, y), (x + rng.range(-0.8, 0.8), y - len)).pressure(0.3, 0.1).ramps(0.2, 0.6).shake(0.5), None);
        }
        // snow lying along the top of the long bough where it runs level:
        // a thin cushion sitting on the upper edge, broken where the wind
        // took it, bluish where it turns from the light
        let mut wb = Held::new(Tool::round_sable(2.0), 703);
        let n = bough.pts.len();
        let mut i = 2;
        while i < n - 4 {
            let run = 3 + (rng.f() * 7.0) as usize;
            let (a, b) = (bough.pts[i], bough.pts[(i + run).min(n - 1)]);
            let level = ((b.1 - a.1) / (b.0 - a.0).abs().max(1e-3)).abs() < 0.45;
            if level && rng.f() < 0.7 {
                let pts: Vec<P> = (i..(i + run).min(n - 1)).map(|k| {
                    let p = bough.pts[k];
                    let (dx, dy) = (bough.pts[k + 1].0 - p.0, bough.pts[k + 1].1 - p.1);
                    let ln = (dx * dx + dy * dy).sqrt().max(1e-3);
                    let (mut nx, mut ny) = (dy / ln, -dx / ln);
                    if ny > 0.0 { nx = -nx; ny = -ny; }
                    let r = 0.5 * bough.w[k];
                    (p.0 + nx * (r - 0.3), p.1 + ny * (r - 0.3))
                }).collect();
                let col = if rng.f() < 0.35 { hex("#b9bccb") } else { hex("#dcd6cc") };
                wb.reload(snow_pal.paint(col, 0.05), 0.5);
                if pts.len() >= 2 {
                    c.drag(&mut wb, &Gesture::new(pts).pressure(rng.range(0.35, 0.6), 0.2).ramps(0.3, 0.4).shake(0.4), None);
                }
            }
            i += run + (rng.f() * 4.0) as usize;
        }
        c.dry();
    }

    // ------------------------------------------------------------ grass
    if o.stage("grass", &mut c, &mut rng) {
        // dead grass and a few weed stems through the snow, flicked up last
        let mut rgr = Held::new(Tool { point: 1.0, ..Tool::rigger(1.0) }, 801);
        let mut sab = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.6) }, 802);
        let straw = [hex("#8a7248"), hex("#6d5a3c"), hex("#4e4232"), hex("#a08a5e")];
        // a few oak leaves the wind has brought down onto the snow, curled,
        // each with a hair of blue shadow under it
        let mut lf = Held::new(Tool { point: 1.0, ..Tool::round_sable(3.2) }, 803);
        for _ in 0..8 {
            let x = tr.cx(1100.0) + rng.range(-170.0, 200.0);
            let y = rng.range(1105.0, 1235.0);
            if y < bank(x) + 8.0 {
                continue;
            }
            let s = 0.7 + 0.5 * ((y - 1100.0) / 150.0);
            let a = rng.range(-0.6, 0.6);
            let (ca, sa) = (a.cos(), a.sin());
            let l = 9.0 * s;
            lf.reload(snow_pal.paint(hex("#a2a8bc"), 0.1), 0.4);
            c.drag(&mut lf, &Gesture::line((x - ca * l * 0.4, y + 2.0 * s), (x + ca * l * 0.5, y + sa * l * 0.5 + 2.5 * s)).pressure(0.5, 0.2).ramps(0.2, 0.5), None);
            let col = [hex("#6e4e32"), hex("#8a6440"), hex("#5a4230")][(rng.f() * 3.0) as usize % 3];
            lf.reload(snow_pal.paint(col, 0.1), 0.6);
            c.drag(&mut lf, &Gesture::new(vec![(x - ca * l * 0.5, y - sa * l * 0.5), (x, y - 1.5 * s), (x + ca * l * 0.5, y + sa * l * 0.5)]).pressure(0.75, 0.1).ramps(0.1, 0.7).shake(0.8), None);
            // the stalk
            c.drag(&mut lf, &Gesture::line((x - ca * l * 0.5, y - sa * l * 0.5), (x - ca * l * 0.75, y - sa * l * 0.75 + 1.0)).pressure(0.25, 0.0).ramps(0.1, 0.8), None);
        }
        // clumps: near the trunk and scattered, smaller with distance
        let mut clumps: Vec<(f32, f32, f32)> = vec![(330.0, 1102.0, 1.0), (560.0, 1106.0, 0.9), (610.0, 1118.0, 0.7), (270.0, 1150.0, 1.1), (180.0, 1190.0, 1.3), (820.0, 1170.0, 1.1), (900.0, 1225.0, 1.3)];
        for _ in 0..7 {
            let x = rng.range(20.0, 980.0);
            let y = rng.range(HZ + 15.0, 1240.0);
            let s = 0.3 + 1.0 * ((y - HZ) / (h - HZ));
            clumps.push((x, y, s));
        }
        for (cx, cy, s) in clumps {
            let n = (4.0 + 16.0 * s * rng.f()) as usize;
            for i in 0..n {
                let b = if i % 3 == 0 { &mut sab } else { &mut rgr };
                b.reload(snow_pal.paint(straw[(rng.f() * 4.0) as usize % 4], 0.1), 0.6);
                let x = cx + rng.range(-14.0, 14.0) * s;
                let y = cy + rng.range(-3.0, 3.0) * s;
                let ht = (10.0 + 30.0 * rng.f()) * s;
                let lean = rng.range(-0.6, 0.5);
                let r = rng.f();
                let pts = if r < 0.2 {
                    // bent over: the top half broken and hanging
                    let k = (x + lean * ht * 0.25, y - ht * 0.6);
                    let side = if lean < 0.0 { -1.0 } else { 1.0 };
                    vec![(x, y), k, (k.0 + side * ht * 0.35, k.1 + ht * 0.12)]
                } else if r < 0.35 {
                    // stubble: a short stiff stalk
                    vec![(x, y), (x + lean * 2.0, y - ht * 0.3)]
                } else {
                    vec![(x, y), (x + lean * ht * 0.3, y - ht * 0.55), (x + lean * ht, y - ht)]
                };
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
            }

    o.end(&mut c, &mut rng);
    c.relief(st.relief.0, st.relief.1);
    o.save(&mut c);
}
