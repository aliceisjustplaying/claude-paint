//! r13_tree3: one tree in winter. An old pedunculate oak, stag-headed (its
//! top limbs dead and broken, silver-gray), standing alone in a snowfield
//! under a low overcast winter sky. Seen from low down, the way Friedrich
//! drew his trees sitting on the ground, so the horizon crosses the lower
//! trunk [trees.md §5.2, BUSCH-V pp.77–78].
//!
//!   cargo paint r13_tree3 -- --full --width 2400
//!   cargo paint r13_tree3 -- --full --width 2400 --crop 300,400,600,700
//!
//! Order (Friedrich's): sky laid thin and stippled, the far field, the snow,
//! then the tree painted over the finished sky [ALF p.346], thick to thin:
//! trunk and limbs as strokes along their length, branches with a pointed
//! round, twigs with a pointed rigger lifted off to the bud; then the snow
//! lying on the limbs; last the grass and small things over the snow
//! [NG p.56].

use paint::color::{Mix, mix};
use paint::{Fbm, Gesture, Held, Mask, Rgb, Rng, Stipple, Style, Tool, Touch, gradient, hex, smoothstep};
use std::f32::consts::{FRAC_PI_2, PI};

const ASPECT: f32 = 0.8; // 1000 x 1250 units
const HZ: f32 = 1012.0; // horizon
const BASE: (f32, f32) = (478.0, 1112.0); // where the trunk meets the snow

// ------------------------------------------------------------------ tree

/// One axis of the tree: a polyline with its diameter at each point.
#[derive(Clone)]
struct Axis {
    pts: Vec<(f32, f32)>,
    w: Vec<f32>,
    order: u32,
    dead: bool,
    /// Ends in a break (a stub), not a bud.
    broken: bool,
    /// Straight upright water shoot (epicormic).
    shoot: bool,
}

struct Grower {
    rng: Rng,
    axes: Vec<Axis>,
    dmin: f32,
}

impl Grower {
    fn step_len(d: f32) -> f32 {
        2.6 * d.powf(0.72) + 3.6
    }

    /// Grow one axis from `p` heading `a` (radians, canvas: -PI/2 is up)
    /// with diameter `d`, spawning side axes (queued).
    fn axis(&mut self, p: (f32, f32), a: f32, d: f32, order: u32, dead: bool, target: Option<(f32, f32)>, queue: &mut Vec<((f32, f32), f32, f32, u32, bool)>) {
        let mut pts = vec![p];
        let mut ws = vec![d];
        let mut target = target;
        let mut run_len = 0.0;
        let dead_len = self.rng.range(50.0, 230.0) * (d / 12.0).clamp(0.5, 1.4);
        let (mut p, mut a, mut d) = (p, a, d);
        let mut side = if self.rng.f() < 0.5 { 1.0 } else { -1.0 };
        let mut steps = 0;
        let mut broken = false;
        // dead wood: no fine twigs, broken ends
        let dstop = if dead { 2.2 + 3.0 * self.rng.f() } else { self.dmin };
        loop {
            steps += 1;
            let l = Self::step_len(d) * self.rng.range(0.7, 1.3);
            // crooked oak: zigzag bends, bigger for thinner wood
            let bend = if order == 0 { 0.06 } else { 0.16 + 0.22 * (1.0 - (d / 20.0).min(1.0)) };
            a += self.rng.normal() * bend;
            // tropisms: a scaffold limb heads for where the painter put it;
            // other wood grows out from the crown's middle and up, filling
            // a dome; thin wood reaching past the dome stops growing
            let up = -FRAC_PI_2;
            let (vx, vy) = ((p.0 - CROWN.0) / CROWN.2, (p.1 - CROWN.1) / if p.1 < CROWN.1 { CROWN.3 } else { CROWN.4 });
            match target {
                Some(t) if d > 5.0 && ((t.0 - p.0).powi(2) + (t.1 - p.1).powi(2)).sqrt() > 70.0 => {
                    let at = (t.1 - p.1).atan2(t.0 - p.0);
                    a += angle_diff(at, a) * 0.16;
                }
                _ => {
                    target = None;
                    let ao = vy.atan2(vx);
                    let want = ao + angle_diff(up, ao) * 0.3;
                    a += angle_diff(want, a) * if dead { 0.03 } else { 0.06 };
                }
            }
            if vx * vx + vy * vy > 1.0 && d < 8.0 {
                d *= 0.72;
            }
            if p.1 > HZ - 150.0 {
                a += angle_diff(up, a) * 0.25;
            }
            p = (p.0 + a.cos() * l, p.1 + a.sin() * l);
            // between forks the wood thins too: twigs shed along the way
            // (cladoptosis) leave their pipes behind
            d *= 0.978;
            run_len += l;
            if dead && run_len > dead_len {
                broken = true;
                break;
            }
            pts.push(p);
            ws.push(d);
            if d < dstop || steps > 200 {
                if dead {
                    broken = true;
                }
                break;
            }
            // a side axis at this node?
            // the leader dies back: above y 470 the old top is dead wood
            // (stag-headed), carried on as a dead axis
            if target.is_some() && !dead && p.1 < DEAD_ABOVE && d > 6.0 && order == 1 && p.0 > 380.0 && p.0 < 600.0 {
                queue.push((p, a, d, 2, true));
                break;
            }
            let trunk_clear = false;
            let chance = if d > 8.0 { 0.7 } else { 0.62 };
            if !trunk_clear && self.rng.f() < chance {
                // codominant fork now and then, else a smaller side branch
                let r = if order == 0 {
                    self.rng.range(0.6, 0.85)
                } else if d > 8.0 {
                    self.rng.range(0.14, 0.42)
                } else if self.rng.f() < 0.18 {
                    self.rng.range(0.62, 0.8)
                } else {
                    self.rng.range(0.3, 0.62)
                };
                let dc = d * r;
                let dn = (d * d - dc * dc).max(0.0).sqrt();
                let spread = if order == 0 { self.rng.range(0.8, 1.25) } else { self.rng.range(0.4, 0.95) };
                let ac = a + side * spread;
                // the continuing axis leans away (sympodial zigzag)
                a -= side * spread * r * 0.45;
                side = -side;
                // dead wood: some limbs of the upper crown are dead
                let child_dead = dead || (order <= 2 && dc > 5.0 && p.1 < 470.0 && self.rng.f() < 0.35);
                if dc >= self.dmin * 0.9 {
                    queue.push((p, ac, dc, order + 1, child_dead));
                }
                d = dn;
                // a limb broken off: the axis ends in a stub
                if order >= 1 && d > 6.0 && d < 16.0 && self.rng.f() < 0.05 {
                    broken = true;
                    // the break: a short jagged end
                    pts.push((p.0 + a.cos() * d * 0.5, p.1 + a.sin() * d * 0.5));
                    ws.push(d * 0.85);
                    break;
                }
            }
        }
        self.axes.push(Axis { pts, w: ws, order, dead, broken, shoot: false });
    }
}

fn angle_diff(to: f32, from: f32) -> f32 {
    let mut d = to - from;
    while d > PI {
        d -= 2.0 * PI;
    }
    while d < -PI {
        d += 2.0 * PI;
    }
    d
}

/// The tree's seed: the painter looked at several and chose this one.
const TREE_SEED: u64 = 104;

/// The crown's dome: center, half width, half height above and below.
const CROWN: (f32, f32, f32, f32, f32) = (480.0, 520.0, 455.0, 410.0, 330.0);
/// Above this the leader is dead.
const DEAD_ABOVE: f32 = 470.0;

fn build_tree(seed: u64) -> Vec<Axis> {
    let mut g = Grower { rng: Rng::new(seed), axes: vec![], dmin: 0.42 };
    // the bole, drawn as the painter chose it: a short massive trunk leaning
    // a little, swelling where the limbs leave it, flared at the foot
    let bole = [(BASE.0, BASE.1), (476.0, 1060.0), (471.0, 1000.0), (466.0, 940.0), (463.0, 880.0), (466.0, 830.0), (472.0, 790.0)];
    let bw = [86.0, 66.0, 60.0, 58.0, 58.0, 61.0, 64.0];
    g.axes.push(Axis { pts: bole.to_vec(), w: bw.to_vec(), order: 0, dead: false, broken: false, shoot: false });
    // an old limb sawn or broken off the bole long ago: a stub with a
    // collar grown round it
    g.axes.push(Axis { pts: vec![(440.0, 905.0), (426.0, 897.0), (418.0, 895.0)], w: vec![20.0, 17.0, 16.0], order: 1, dead: true, broken: true, shoot: false });
    // the scaffold limbs (start, angle, diameter, where they head):
    // Leonardo's rule: their squares sum to about the bole's (58²)
    let limbs: [((f32, f32), f32, f32, (f32, f32)); 5] = [
        ((460.0, 815.0), -2.75, 24.0, (210.0, 690.0)), // low left, spreading
        ((466.0, 795.0), -2.05, 27.0, (290.0, 230.0)), // up left
        ((474.0, 790.0), -1.52, 27.0, (505.0, 140.0)), // the leader, dead above
        ((480.0, 795.0), -1.05, 24.0, (700.0, 250.0)), // up right
        ((486.0, 835.0), -0.35, 24.0, (800.0, 640.0)), // low right, long
    ];
    let mut queue = vec![];
    for &(p, a, d, t) in &limbs {
        g.axis(p, a, d, 1, false, Some(t), &mut queue);
        while let Some((p, a, d, order, dead)) = queue.pop() {
            g.axis(p, a, d, order, dead, None, &mut queue);
            if g.axes.len() > 60_000 {
                break;
            }
        }
    }
    // water shoots: straight thin rods from the bole and old limbs
    let mut rng = Rng::new(seed + 7);
    let mut shoots = vec![];
    for ax in g.axes.iter().filter(|a| a.order <= 2 && !a.dead) {
        for i in 1..ax.pts.len() {
            if ax.w[i] > 14.0 && rng.f() < 0.22 {
                let p = ax.pts[i];
                let side = if rng.f() < 0.5 { -1.0 } else { 1.0 };
                let n = 2 + (rng.f() * 3.0) as usize;
                for _ in 0..n {
                    let a0 = -FRAC_PI_2 + side * rng.range(0.15, 0.6);
                    let len = rng.range(14.0, 40.0);
                    let start = (p.0 + side * ax.w[i] * 0.45, p.1 + rng.range(-6.0, 6.0));
                    let k = 5;
                    let pts: Vec<(f32, f32)> = (0..=k)
                        .map(|j| {
                            let t = j as f32 / k as f32;
                            let aa = a0 + 0.25 * t * side * -1.0;
                            (start.0 + aa.cos() * len * t, start.1 + aa.sin() * len * t)
                        })
                        .collect();
                    let w: Vec<f32> = (0..=k).map(|j| 0.85 - 0.45 * j as f32 / k as f32).collect();
                    shoots.push(Axis { pts, w, order: 9, dead: false, broken: false, shoot: true });
                }
            }
        }
    }
    g.axes.extend(shoots);
    g.axes
}

/// Resample an axis piece to points no more than `step` apart.
fn densify(pts: &[(f32, f32)], w: &[f32], step: f32) -> (Vec<(f32, f32)>, Vec<f32>) {
    let mut p2 = vec![pts[0]];
    let mut w2 = vec![w[0]];
    for i in 1..pts.len() {
        let (a, b) = (pts[i - 1], pts[i]);
        let l = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
        let n = (l / step).ceil().max(1.0) as usize;
        for k in 1..=n {
            let t = k as f32 / n as f32;
            p2.push((a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t));
            w2.push(w[i - 1] + (w[i] - w[i - 1]) * t);
        }
    }
    (p2, w2)
}

/// Points offset across an axis by `s` (-1..1) of the local half width.
fn offset_line(pts: &[(f32, f32)], w: &[f32], s: f32) -> Vec<(f32, f32)> {
    let n = pts.len();
    (0..n)
        .map(|i| {
            let (a, b) = (pts[i.saturating_sub(1)], pts[(i + 1).min(n - 1)]);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let l = (dx * dx + dy * dy).sqrt().max(1e-4);
            let (nx, ny) = (-dy / l, dx / l);
            (pts[i].0 + nx * s * w[i] * 0.5, pts[i].1 + ny * s * w[i] * 0.5)
        })
        .collect()
}

/// Split an axis into runs where the diameter lies in [lo, hi).
fn runs(ax: &Axis, lo: f32, hi: f32) -> Vec<(Vec<(f32, f32)>, Vec<f32>)> {
    let mut out = vec![];
    let mut cur: (Vec<(f32, f32)>, Vec<f32>) = (vec![], vec![]);
    for i in 0..ax.pts.len() {
        let inside = ax.w[i] >= lo && ax.w[i] < hi;
        if inside {
            if cur.0.is_empty() && i > 0 {
                // start at the previous point so pieces join
                cur.0.push(ax.pts[i - 1]);
                cur.1.push(ax.w[i - 1].min(hi));
            }
            cur.0.push(ax.pts[i]);
            cur.1.push(ax.w[i]);
        } else if !cur.0.is_empty() {
            cur.0.push(ax.pts[i]);
            cur.1.push(ax.w[i].max(lo * 0.9));
            out.push(std::mem::take(&mut cur));
        }
    }
    if cur.0.len() >= 2 {
        out.push(cur);
    }
    out
}

/// A quick look at the skeleton (a diagnostic, not the painting): the axes
/// as dark capsules on white, 0.5 px per unit, as a PGM.
fn preview(axes: &[Axis], out: &str) {
    let (w, h) = (500usize, 625usize);
    let mut img = vec![255u8; w * h];
    for ax in axes {
        let (p, ws) = densify(&ax.pts, &ax.w, 1.0);
        for (i, &(x, y)) in p.iter().enumerate() {
            let r = (ws[i] * 0.25).max(0.3);
            let v = if ax.dead { 120u8 } else { 0 };
            let (x0, x1) = (((x * 0.5 - r).floor() as i64).max(0), ((x * 0.5 + r).ceil() as i64).min(w as i64 - 1));
            let (y0, y1) = (((y * 0.5 - r).floor() as i64).max(0), ((y * 0.5 + r).ceil() as i64).min(h as i64 - 1));
            for yy in y0..=y1 {
                for xx in x0..=x1 {
                    let d = ((xx as f32 + 0.5 - x * 0.5).powi(2) + (yy as f32 + 0.5 - y * 0.5).powi(2)).sqrt();
                    if d <= r + 0.35 {
                        let i = yy as usize * w + xx as usize;
                        img[i] = img[i].min(v.max((255.0 * (1.0 - (r + 0.35 - d).min(1.0)) ) as u8));
                    }
                }
            }
        }
    }
    let mut data = format!("P5\n{w} {h}\n255\n").into_bytes();
    data.extend(img);
    std::fs::write(out, data).unwrap();
}

// ------------------------------------------------------------ colors

fn sky(x: f32, y: f32) -> Rgb {
    // a low overcast: gray with a breath of violet above, paling through a
    // cool pearl to a thin warm light lying on the horizon, a little
    // brighter to the left where the sun is behind the cloud
    let t = (y / HZ).clamp(0.0, 1.0);
    let base = gradient(
        &[(0.0, hex("#7d828d")), (0.3, hex("#979aa1")), (0.62, hex("#b9b8b4")), (0.86, hex("#d6cfbf")), (1.0, hex("#e2d8c0"))],
        t,
        Mix::Light,
    );
    let glow = smoothstep(700.0, 0.0, x) * smoothstep(0.35, 0.95, t) * 0.25;
    mix(base, hex("#ece2c8"), glow, Mix::Light)
}

fn main() {
    let o = paintings::run::Run::new("r13_tree3");
    let st = Style::friedrich();
    let pal = &st.palette;
    let mut rng = Rng::new(o.seed);
    if let Ok(out) = std::env::var("TREE_PREVIEW") {
        let k: u64 = std::env::var("TREE_SEED").ok().and_then(|v| v.parse().ok()).unwrap_or(TREE_SEED);
        preview(&build_tree(k), &out);
        return;
    }
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = c.height();

    // ---- sky
    let bands = Fbm::new(o.seed as u32 + 11, 4, 260.0);
    let skyc = move |x: f32, y: f32| {
        // long low bands of cloud, a little darker, lying level
        let b = bands.get(x * 0.35, y * 2.2);
        let k = smoothstep(0.1, 0.55, b) * 0.10 * smoothstep(HZ, 250.0, y);
        mix(sky(x, y), hex("#6f7280"), k, Mix::Light)
    };
    let sky_m = Mask::from_fn(f, |_, y| 1.0 - smoothstep(HZ + 4.0, HZ + 10.0, y));
    if o.stage("sky", &mut c, &mut rng) {
        let lay = st.broad().color(move |x, y| mix(skyc(x, y), hex("#7a7a80"), 0.05, Mix::Light)).angle(|_, _| 0.0).angle_jitter(0.08).coverage(4.2).medium(0.3);
        c.work(&sky_m, &lay, 11);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.angle(|_, _| 0.0), 12);
        }
        let s1 = Stipple::new(Tool::stippler(2.8)).mixed(pal, 0.45).color(skyc).coverage(|_, _| 2.0).pressure(0.5, 0.85).dips(20, 0.4, 0.5).cluster(0.25, None);
        c.stipple(&sky_m, &s1, 13);
        c.dry();
        // a finer, lighter pass, denser toward the light on the horizon
        let glow = move |x: f32, y: f32| mix(skyc(x, y), hex("#efe5cc"), 0.05 + 0.2 * smoothstep(550.0, HZ, y), Mix::Light);
        let s2 = Stipple::new(Tool::stippler(1.5)).mixed(pal, 0.5).color(glow).coverage(|_, y| 0.6 + 1.6 * smoothstep(300.0, HZ, y)).pressure(0.45, 0.8).dips(24, 0.35, 0.6);
        c.stipple(&sky_m, &s2, 14);
        c.dry();
    }

    // ---- the far field: the snow plain to the horizon, a far hedge line
    let hedge = Fbm::new(o.seed as u32 + 21, 4, 40.0);
    let hedge_top = move |x: f32| {
        let run = smoothstep(40.0, 120.0, x) * (1.0 - smoothstep(230.0, 330.0, x)) + smoothstep(690.0, 760.0, x) * (1.0 - smoothstep(930.0, 990.0, x)) * 0.8;
        HZ - run * (4.0 + 5.0 * hedge.get01(x, 3.0))
    };
    let snowc = {
        let mounds = Fbm::new(o.seed as u32 + 31, 5, 180.0);
        let fine = Fbm::new(o.seed as u32 + 32, 3, 30.0);
        move |x: f32, y: f32| {
            let near = smoothstep(HZ, h, y);
            // drifts: stretched level with distance
            let sx = x * (0.35 + 0.65 * near);
            let m = mounds.get(sx, y * 2.0);
            let slope = mounds.get(sx + 6.0, y * 2.0) - m; // lit from the left
            let lit = (0.5 - slope * 9.0).clamp(0.0, 1.0);
            let far = gradient(&[(0.0, hex("#c9c6c0")), (1.0, hex("#dcd8cf"))], near.sqrt(), Mix::Light);
            let shade = hex("#a9adb6");
            let light = hex("#ece7da");
            let mut col = mix(far, shade, (0.25 + 0.5 * (1.0 - lit)) * (0.3 + 0.7 * near) * (0.6 + 0.4 * m.abs()), Mix::Light);
            col = mix(col, light, lit * 0.55 * near, Mix::Light);
            // under the crown and around the foot the snow is a shade cooler
            let dtx = (x - BASE.0) / 380.0;
            let dty = (y - BASE.1 - 40.0) / 90.0;
            let under = (-(dtx * dtx + dty * dty)).exp() * 0.25;
            col = mix(col, hex("#9da3ae"), under + 0.04 * fine.get(x, y), Mix::Light);
            col
        }
    };
    let land_m = Mask::from_fn(f, move |_, y| smoothstep(HZ - 1.0, HZ + 1.5, y));
    if o.stage("snow", &mut c, &mut rng) {
        // snow laid in body color, level strokes far off, curving over the
        // drifts near to
        let lay = st.body().color(snowc).angle(move |x, y| 0.06 * ((x - 500.0) / 500.0) * smoothstep(HZ, h, y)).angle_jitter(0.12).length(30.0, 110.0).coverage(3.2).medium(0.15);
        c.work(&land_m, &lay, 21);
        // foreground: fuller paint, a slight impasto on the lit drifts
        let near_m = land_m.clone().mul_fn(move |_, y| smoothstep(HZ + 60.0, HZ + 160.0, y));
        let thick = st.body().color(snowc).angle(|_, _| 0.05).angle_jitter(0.25).length(12.0, 40.0).coverage(1.6).medium(0.05).curve(0.1, 0.3);
        c.work(&near_m, &thick, 22);
        c.dry();
        // far field stippled level so the horizon is touches, not a rule
        let far_m = Mask::from_fn(f, move |_, y| smoothstep(HZ - 2.0, HZ, y) * (1.0 - smoothstep(HZ + 10.0, HZ + 45.0, y)));
        let sp = Stipple::new(Tool::stippler(1.6)).mixed(pal, 0.5).color(snowc).coverage(|_, _| 1.5).pressure(0.4, 0.75).drag(1.4, Some(0.0)).dips(18, 0.35, 0.6);
        c.stipple(&far_m, &sp, 23);
        // the far hedge: bluish touches, very pale in the air
        let hedge_m = Mask::from_fn(f, move |x, y| if y >= hedge_top(x) - 0.3 && y <= HZ + 1.0 { 1.0 } else { 0.0 });
        let hc = |x: f32, _y: f32| mix(hex("#8e929a"), hex("#a8a9ab"), smoothstep(0.0, 1000.0, x), Mix::Light);
        let sp = Stipple::new(Tool::stippler(1.2)).mixed(pal, 0.35).color(hc).coverage(|_, _| 2.2).pressure(0.4, 0.7).dips(14, 0.35, 0.5).clip(true);
        c.stipple(&hedge_m, &sp, 24);
        c.dry();
    }

    // ---- the tree (geometry is built on every run; it draws nothing)
    let axes = build_tree(TREE_SEED);
    eprintln!("tree: {} axes, {} twig tips", axes.len(), axes.iter().filter(|a| !a.broken && !a.dead).count());

    // bark colors: dark warm gray-brown, grayer and lighter on the lit
    // (left, upper) side; dead wood silver-gray
    let bark = |s: f32, dead: bool, x: f32, y: f32| -> Rgb {
        // s: -1 (left edge) .. 1 (right edge)
        let lit = smoothstep(0.6, -0.8, s);
        let n = ((x * 0.37 + y * 0.11).sin() * 0.5 + 0.5) * 0.15;
        if dead {
            mix(hex("#4a4744"), hex("#8f8b84"), lit * 0.9 + n, Mix::Pigment)
        } else {
            mix(hex("#2a2521"), hex("#5d5a52"), lit * 0.8 + n, Mix::Pigment)
        }
    };

    if o.stage("limbs", &mut c, &mut rng) {
        // trunk and limbs thicker than ~4 units: strokes along their length,
        // side by side across the wood, the outer ones making the edge
        let mut pieces = vec![];
        for ax in &axes {
            for (p, w) in runs(ax, 4.0, 1e9) {
                pieces.push((p, w, ax.dead, ax.order));
            }
        }
        // thick first
        pieces.sort_by(|a, b| b.1[0].total_cmp(&a.1[0]));
        let mut hr = Rng::new(o.seed + 201);
        for (k, (p, w, dead, order)) in pieces.iter().enumerate() {
            let (p, w) = densify(p, w, 4.0);
            let wmax = w.iter().cloned().fold(0.0, f32::max);
            let tw = (wmax * 0.34).clamp(3.0, 11.0);
            let tool = if tw > 5.0 { Tool { lay: 0.8, ..Tool::filbert(tw) } } else { Tool::round_sable(tw) };
            let mut held = Held::new(tool.clone(), o.seed + 300 + k as u64);
            let n = ((wmax / (tw * 0.55)).ceil() as usize).max(2);
            // strokes across, from the shaded edge to the lit one
            for i in 0..n {
                let s = -1.0 + (2.0 * i as f32 + 1.0) / n as f32;
                let s = s * (1.0 - tw / wmax.max(tw) * 0.45);
                let line = offset_line(&p, &w, s);
                // break long runs into strokes of 25–90 units, overlapping
                let mut j = 0;
                while j + 1 < line.len() {
                    let len = (hr.range(6.0, 22.0) as usize).max(3);
                    let e = (j + len).min(line.len() - 1);
                    let seg: Vec<(f32, f32)> = line[j..=e].to_vec();
                    let (mx, my) = seg[seg.len() / 2];
                    let col = bark(s, *dead, mx, my);
                    let paint = pal.paint(col, 0.12);
                    held.reload(paint, 0.9);
                    let pr = if *order == 0 { 0.75 } else { 0.7 };
                    // strokes go up the wood (from the base toward the tip)
                    c.drag(&mut held, &Gesture::new(seg).pressure(pr, pr * 0.9).ramps(0.1, 0.2).shake(0.5), None);
                    j = if e + 1 >= line.len() { e } else { e.saturating_sub(1) };
                    if e == line.len() - 1 {
                        break;
                    }
                }
            }
        }
        c.wait(30.0);
        // bark: fissures in the bole and big limbs, dark, following the
        // wood; lit ridges between them dragged lightly over the tooth
        let mut fiss = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.6) }, o.seed + 401);
        let mut ridge = Held::new(Tool { lay: 0.5, ..Tool::round_sable(2.2) }, o.seed + 402);
        for ax in axes.iter().filter(|a| a.order <= 2) {
            for (p, w) in runs(ax, 9.0, 1e9) {
                let (p, w) = densify(&p, &w, 3.0);
                let wmax = w.iter().cloned().fold(0.0, f32::max);
                let n = (wmax / 3.2) as usize;
                for _ in 0..n {
                    let s = hr.range(-0.88, 0.88);
                    let line = offset_line(&p, &w, s);
                    let a = (hr.f() * (line.len() as f32 - 3.0)) as usize;
                    let len = hr.range(4.0, 14.0) as usize;
                    let e = (a + len).min(line.len() - 1);
                    if e <= a + 1 {
                        continue;
                    }
                    let seg: Vec<(f32, f32)> = line[a..=e].iter().map(|&(x, y)| (x + hr.range(-0.6, 0.6), y)).collect();
                    let lit = smoothstep(0.6, -0.8, s);
                    fiss.reload(pal.paint(mix(hex("#1b1714"), hex("#302b26"), lit, Mix::Pigment), 0.1), 0.8);
                    c.drag(&mut fiss, &Gesture::new(seg.clone()).pressure(0.55, 0.3).ramps(0.1, 0.4).shake(0.8), None);
                    if hr.f() < 0.55 && s < 0.4 {
                        let seg2: Vec<(f32, f32)> = seg.iter().map(|&(x, y)| (x + 1.6, y)).collect();
                        ridge.reload(pal.paint(if ax.dead { hex("#a29d94") } else { hex("#77746b") }, 0.05).with_stiff(0.9), 0.35);
                        c.drag(&mut ridge, &Gesture::new(seg2).pressure(0.3, 0.25).ramps(0.2, 0.3).shake(0.8), None);
                    }
                }
            }
        }
        c.dry();
    }

    if o.stage("branches", &mut c, &mut rng) {
        // branches 1.2–4 units: a pointed round, pressed to the width where
        // the branch leaves its parent, lifting toward its thinner end
        let mut held = Held::new(Tool { point: 1.0, ..Tool::round_sable(4.4) }, o.seed + 501);
        let mut hr = Rng::new(o.seed + 502);
        let mut pieces = vec![];
        for ax in axes.iter().filter(|a| !a.shoot) {
            for (p, w) in runs(ax, 1.2, 4.0) {
                pieces.push((p, w, ax.dead));
            }
        }
        pieces.sort_by(|a, b| b.1[0].total_cmp(&a.1[0]));
        let tool = held.tool.clone();
        for (p, w, dead) in &pieces {
            let (p, w) = densify(p, w, 3.0);
            let p0 = tool.pressure_for(w[0]).max(0.05);
            let swell: Vec<f32> = w.iter().step_by((w.len() / 8).max(1)).map(|&wi| tool.pressure_for(wi) / p0).collect();
            let s = hr.range(-0.6, 0.2);
            let col = bark(s, *dead, p[0].0, p[0].1);
            held.reload(pal.paint(col, 0.12), 0.8);
            c.drag(&mut held, &Gesture::new(p.clone()).pressure(p0, p0).swell(swell).ramps(0.03, 0.08).shake(0.35), None);
        }
        c.wait(20.0);
    }

    if o.stage("twigs", &mut c, &mut rng) {
        // the fine twigs: a pointed rigger, pressed where each leaves its
        // branch and lifted off to the bud; slightly warmer and lighter than
        // the limbs, the finest melting into the sky
        let mut held = Held::new(Tool { point: 1.0, ..Tool::rigger(1.4) }, o.seed + 601);
        let mut hr = Rng::new(o.seed + 602);
        let tool = held.tool.clone();
        let mut n = 0;
        for ax in axes.iter() {
            for (p, w) in runs(ax, 0.0, 1.2) {
                let (p, w) = densify(&p, &w, 2.5);
                let p0 = tool.pressure_for(w[0]).max(0.04);
                let p1 = tool.pressure_for(*w.last().unwrap()).max(0.02);
                let col = if ax.shoot {
                    hex("#4a3b30")
                } else if ax.dead {
                    hex("#6a6660")
                } else {
                    mix(hex("#302a26"), hex("#4b433b"), hr.f(), Mix::Pigment)
                };
                if n % 3 == 0 {
                    held.reload(pal.paint(col, 0.18), 0.7);
                }
                n += 1;
                c.drag(&mut held, &Gesture::new(p).pressure(p0, p1).ramps(0.02, 0.35).shake(0.45), None);
            }
        }
        eprintln!("twig strokes: {n}");
        c.dry();
    }

    if o.stage("snow on tree", &mut c, &mut rng) {
        // snow lies as a ridge on the upper side of near-level limbs thick
        // enough to hold it, heaps in the forks, little on steep or thin
        // wood [MIL64 pp.5–7; trees.md §4]
        let mut held = Held::new(Tool { point: 0.6, lay: 1.2, ..Tool::round_sable(3.0) }, o.seed + 701);
        let mut hr = Rng::new(o.seed + 702);
        let snow = |lit: f32| mix(hex("#c9ccd2"), hex("#f1ede2"), lit, Mix::Light);
        for ax in axes.iter().filter(|a| !a.shoot) {
            let (p, w) = densify(&ax.pts, &ax.w, 2.0);
            let mut i = 0;
            while i + 2 < p.len() {
                let (a, b) = (p[i], p[i + 1]);
                let ang = (b.1 - a.1).atan2(b.0 - a.0);
                let level = ang.cos().abs();
                let wi = w[i];
                // hold: level wood, thick enough
                let hold = smoothstep(0.55, 0.92, level) * smoothstep(1.4, 5.0, wi);
                if hold < 0.05 || hr.f() > hold * 0.85 {
                    i += 1;
                    continue;
                }
                // a band: a few points along the upper edge
                let len = ((hr.range(3.0, 10.0) * hold) as usize).max(2);
                let e = (i + len).min(p.len() - 1);
                let seg: Vec<(f32, f32)> = (i..=e)
                    .map(|j| {
                        let (q, r) = (p[j.saturating_sub(1)], p[(j + 1).min(p.len() - 1)]);
                        let (dx, dy) = (r.0 - q.0, r.1 - q.1);
                        let l = (dx * dx + dy * dy).sqrt().max(1e-4);
                        // the upward normal
                        let (mut nx, mut ny) = (-dy / l, dx / l);
                        if ny > 0.0 {
                            nx = -nx;
                            ny = -ny;
                        }
                        (p[j].0 + nx * w[j] * 0.42, p[j].1 + ny * w[j] * 0.42 - 0.3)
                    })
                    .collect();
                let thick = (wi * 0.28 + 0.8).min(3.2);
                let pr = held.tool.pressure_for(thick);
                let lit = 0.55 + 0.45 * hr.f();
                held.reload(pal.paint(snow(lit), 0.02).with_stiff(0.95), 0.8);
                c.drag(&mut held, &Gesture::new(seg).pressure(pr * 0.7, pr * 0.6).swell(vec![0.8, 1.2, 1.0, 0.7]).ramps(0.25, 0.4).shake(0.5), None);
                i = e + (hr.range(1.0, 6.0) as usize);
            }
        }
        // forks: small heaps where a side limb leaves a thick one
        let mut dab = Held::new(Tool::round_sable(3.5), o.seed + 703);
        for ax in axes.iter().filter(|a| a.order >= 1 && a.order <= 3 && !a.shoot) {
            let (x, y) = ax.pts[0];
            if ax.w[0] > 3.0 && hr.f() < 0.6 {
                dab.reload(pal.paint(snow(0.7 + 0.3 * hr.f()), 0.02).with_stiff(0.95), 0.9);
                c.touch(&mut dab, &Touch::at(x, y - ax.w[0] * 0.3).pressure(0.5 + 0.3 * hr.f()), None);
            }
        }
        c.wait(30.0);
    }

    if o.stage("foot", &mut c, &mut rng) {
        // snow banked against the foot of the trunk, a fallen limb half
        // buried, dry grass through the snow, laid last over it [NG p.56]
        let mut hr = Rng::new(o.seed + 801);
        let mut b = Held::new(Tool { lay: 1.1, ..Tool::filbert(6.0) }, o.seed + 802);
        for k in 0..16 {
            let t = k as f32 / 15.0;
            let x = BASE.0 - 48.0 + t * 96.0 + hr.range(-3.0, 3.0);
            let y = BASE.1 - 4.0 + 6.0 * (t - 0.5).abs() + hr.range(-2.0, 2.0);
            let lit = smoothstep(0.9, 0.0, t);
            b.reload(pal.paint(mix(hex("#aeb2ba"), hex("#e9e4d8"), lit, Mix::Light), 0.04).with_stiff(0.95), 0.9);
            c.drag(&mut b, &Gesture::new(vec![(x - 6.0, y + 1.0), (x, y - 1.5), (x + 7.0, y + 0.5)]).pressure(0.6, 0.5).ramps(0.2, 0.4).shake(0.6), None);
        }
        // the fallen limb, lying right of the tree, broken from the crown
        let limb: Vec<(f32, f32)> = vec![(560.0, 1150.0), (610.0, 1146.0), (655.0, 1149.0), (700.0, 1144.0), (735.0, 1147.0)];
        let mut lb = Held::new(Tool::round_sable(7.0), o.seed + 803);
        lb.reload(pal.paint(hex("#3b3631"), 0.1), 0.9);
        c.drag(&mut lb, &Gesture::new(limb.clone()).pressure(0.8, 0.45).ramps(0.05, 0.2).shake(0.5), None);
        let mut tw = Held::new(Tool { point: 1.0, ..Tool::rigger(1.6) }, o.seed + 804);
        for &(x, y, a, l) in &[(600.0, 1146.0, -2.2f32, 22.0f32), (648.0, 1147.0, -1.2, 18.0), (690.0, 1144.0, -2.0, 26.0), (720.0, 1146.0, -0.8, 14.0)] {
            tw.reload(pal.paint(hex("#3a332d"), 0.15), 0.7);
            let pts: Vec<(f32, f32)> = (0..5).map(|i| {
                let t = i as f32 / 4.0;
                let aa = a + 0.3 * t * hr.range(-1.0, 1.0);
                (x + aa.cos() * l * t, y + aa.sin() * l * t)
            }).collect();
            c.drag(&mut tw, &Gesture::new(pts).pressure(0.6, 0.1).ramps(0.03, 0.5).shake(0.5), None);
        }
        // snow on the fallen limb and drifted against it
        let mut sb = Held::new(Tool { point: 0.5, ..Tool::round_sable(3.0) }, o.seed + 805);
        sb.reload(pal.paint(hex("#ece8dc"), 0.02).with_stiff(0.95), 0.9);
        let top: Vec<(f32, f32)> = limb.iter().map(|&(x, y)| (x, y - 3.2)).collect();
        c.drag(&mut sb, &Gesture::new(top).pressure(0.55, 0.4).swell(vec![1.0, 0.6, 1.1, 0.8, 1.0]).ramps(0.1, 0.3).shake(0.6), None);
        // dry grass and weeds through the snow
        let mut gb = Held::new(Tool { point: 1.0, ..Tool::rigger(1.0) }, o.seed + 806);
        let tufts = [(210.0f32, 1180.0f32), (300.0, 1120.0), (395.0, 1128.0), (560.0, 1122.0), (640.0, 1200.0), (800.0, 1160.0), (905.0, 1215.0), (120.0, 1225.0), (470.0, 1215.0), (860.0, 1098.0), (150.0, 1080.0)];
        for &(tx, ty) in &tufts {
            let n = 5 + (hr.f() * 9.0) as usize;
            let scale = 0.5 + 0.7 * smoothstep(HZ, h, ty);
            for _ in 0..n {
                let x = tx + hr.range(-10.0, 10.0) * scale;
                let y = ty + hr.range(-2.0, 2.0);
                let ht = hr.range(6.0, 22.0) * scale;
                let lean = hr.range(-0.6, 0.6);
                let col = if hr.f() < 0.5 { hex("#8a7050") } else { hex("#5e4c3a") };
                gb.reload(pal.paint(col, 0.15), 0.6);
                let pts = vec![(x, y), (x + lean * ht * 0.3, y - ht * 0.55), (x + lean * ht, y - ht)];
                c.drag(&mut gb, &Gesture::new(pts).pressure(0.55, 0.0).ramps(0.05, 0.85).shake(0.5), None);
            }
        }
        c.dry();
    }

    o.end(&mut c, &mut rng);
    c.relief(st.relief.0, st.relief.1);
    o.save(&mut c);
}
