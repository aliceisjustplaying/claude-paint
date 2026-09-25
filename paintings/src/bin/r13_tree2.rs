//! One tree in winter: an old pedunculate oak, stag-headed, standing alone
//! on a low rise of snow under a pale overcast sky. Seen from low down (the
//! horizon crosses the lower trunk, as Friedrich marked it on his studies
//! drawn sitting on the ground). Two of its upper limbs are dead and bare
//! of bark; a lower limb ends in a clean sawn-looking stump; the living
//! crown is a dense net of crooked short shoots. Wet snow lies along the
//! tops of the level limbs and in the forks.
//!
//!   cargo paint r13_tree2 -- --full --width 2400
//!   cargo paint r13_tree2 -- --full --width 2400 --crop 380,560,620,760
//!
//! Order of work (as the sources give it): underdrawing of the trunk and
//! scaffold limbs in pencil, sky (lay-in, fused, stippled), the snowfield,
//! then the tree over the finished sky, thick to thin, the snow on the
//! limbs, and the small things last (weeds through the snow, dead leaves).

use paint::graphite::{Lead, Mark};
use paint::{Canvas, Fbm, Gesture, Held, Mask, Mix, Rgb, Rng, Shape, Stipple, Style, Tool, Touch, gradient, hex, smoothstep};
use std::f32::consts::{FRAC_PI_2, PI};

const ASPECT: f32 = 0.8; // 1000 x 1250 units
const HORIZON: f32 = 1004.0;
/// Where the trunk meets the snow (centre).
const FOOT: (f32, f32) = (486.0, 1128.0);
/// Twigs thinner than this are drawn as single pointed strokes.
const BODY_W: f32 = 2.4;

fn mix(a: Rgb, b: Rgb, t: f32) -> Rgb {
    paint::color::mix(a, b, t.clamp(0.0, 1.0), Mix::Pigment)
}

/// One branch as drawn: a polyline with a full width at each point.
#[derive(Clone)]
struct Br {
    pts: Vec<(f32, f32)>,
    w: Vec<f32>,
    dead: bool,
    /// ends in a live growing tip (lifted off to a point)
    tip: bool,
}

impl Br {
    /// Unit direction at point i.
    fn dir(&self, i: usize) -> (f32, f32) {
        let n = self.pts.len();
        let (a, e) = (self.pts[i.saturating_sub(1)], self.pts[(i + 1).min(n - 1)]);
        let (dx, dy) = (e.0 - a.0, e.1 - a.1);
        let l = (dx * dx + dy * dy).sqrt().max(1e-4);
        (dx / l, dy / l)
    }
    /// The point at i moved across the branch by `u` half-widths (u = 1 is
    /// the edge on the left of travel... in canvas terms the normal (-dy, dx)).
    fn across(&self, i: usize, u: f32) -> (f32, f32) {
        let (dx, dy) = self.dir(i);
        let off = u * self.w[i] * 0.5;
        (self.pts[i].0 - dy * off, self.pts[i].1 + dx * off)
    }
}

fn ang_diff(to: f32, from: f32) -> f32 {
    let mut d = to - from;
    while d > PI {
        d -= 2.0 * PI;
    }
    while d < -PI {
        d += 2.0 * PI;
    }
    d
}

/// Grow a branch from `p` at angle `a` (canvas angle, y down) and width `w`
/// (units): a crooked oak shoot. Side branches leave at internodes whose
/// length grows with the width; each fork takes its share of the parent's
/// cross-section (Leonardo, exponent `DELTA`); the parent kinks away from
/// the child (sympodial takeover), so an oak's line goes in elbows, not
/// waves. Thin shoots turn upward, heavy level ones sag. At a live tip the
/// buds cluster: two or three short shoots from one point.
fn grow(rng: &mut Rng, out: &mut Vec<Br>, p: (f32, f32), a0: f32, w0: f32, dead: bool, vigor: f32) {
    const DELTA: f32 = 2.0;
    let wmin = 0.32;
    if out.len() > 40_000 {
        return;
    }
    let mut pts = vec![p];
    let mut ws = vec![w0];
    let mut a = a0;
    let mut w = w0;
    let mut pos = p;
    let mut to_fork = internode(w, rng);
    let mut kids: Vec<((f32, f32), f32, f32)> = vec![];
    let mut tip = !dead;
    let mut guard = 0;
    loop {
        guard += 1;
        if guard > 500 {
            break;
        }
        let step = (1.4 + 0.8 * w).min(9.0);
        // small wander, and now and then an elbow
        a += rng.normal() * (0.02 + 0.07 / (1.0 + 0.5 * w));
        if rng.chance(0.05) {
            a += rng.range(-0.5, 0.5) / (1.0 + 0.15 * w);
        }
        // tropism: thin shoots turn up toward the light, heavy near-level
        // limbs sag a little under their own weight
        let up = -FRAC_PI_2;
        if w < 5.0 {
            a += ang_diff(up, a) * 0.025 * vigor;
        } else if a.cos().abs() > 0.5 {
            let down = if a.cos() > 0.0 { 0.0 } else { PI };
            a += ang_diff(down, a) * 0.012;
        }
        pos = (pos.0 + step * a.cos(), pos.1 + step * a.sin());
        w *= (-step * 0.004).exp();
        pts.push(pos);
        ws.push(w);
        if pos.1 > FOOT.1 - 30.0 || pos.0 < -40.0 || pos.0 > 1040.0 || pos.1 < -40.0 {
            tip = false;
            break;
        }
        to_fork -= step;
        if to_fork <= 0.0 {
            to_fork = internode(w, rng);
            // an old oak sheds much of its fine growth: some forks abort
            if w < 1.4 && rng.chance(0.15 + (1.0 - vigor) * 0.25) {
                continue;
            }
            let r = rng.range(0.45, 0.85);
            let wc = w * r;
            let wp = (w.powf(DELTA) - wc.powf(DELTA)).max(0.0).powf(1.0 / DELTA).max(wc * 0.85);
            let side = if rng.chance(0.5) { 1.0 } else { -1.0 };
            let spread = rng.range(0.5, 1.1) * (0.75 + 0.25 * r);
            let kink = rng.range(0.15, 0.45) * (1.0 - 0.3 * r);
            kids.push((pos, a + side * spread, wc));
            a -= side * kink;
            w = wp;
        }
        if w < wmin {
            break;
        }
        if dead && w < 3.0 && rng.chance(0.2) {
            break; // dead wood breaks off: a stub
        }
    }
    // clustered buds at a live tip: short shoots from one point
    if tip && w0 < 3.0 && rng.chance(0.55) && out.len() < 40_000 {
        let q = *pts.last().unwrap();
        let wl = *ws.last().unwrap();
        for _ in 0..rng.range(1.0, 3.0) as usize {
            let aa = a + rng.range(-0.8, 0.8);
            let l = rng.range(3.0, 9.0);
            let e = (q.0 + l * aa.cos(), q.1 + l * aa.sin() - l * 0.2);
            out.push(Br { pts: vec![q, ((q.0 + e.0) * 0.5 + rng.range(-0.6, 0.6), (q.1 + e.1) * 0.5), e], w: vec![wl * 0.9, wl * 0.7, wl * 0.5], dead: false, tip: true });
        }
    }
    out.push(Br { pts, w: ws, dead, tip });
    for (q, ka, kw) in kids {
        if kw < wmin {
            continue;
        }
        grow(rng, out, q, ka, kw, dead, vigor);
    }
}

fn internode(w: f32, rng: &mut Rng) -> f32 {
    (3.5 + 2.4 * w).min(70.0) * rng.range(0.55, 1.45)
}

/// Resample a hand-placed polyline smoothly every `step` units with widths
/// interpolated along its length.
fn smooth(ctrl: &[(f32, f32, f32)], step: f32) -> (Vec<(f32, f32)>, Vec<f32>) {
    let pts: Vec<(f32, f32)> = ctrl.iter().map(|c| (c.0, c.1)).collect();
    let dense = paint::graphite::resample(&pts, true, step);
    let mut cl = vec![0.0f32];
    for i in 1..ctrl.len() {
        cl.push(cl[i - 1] + ((ctrl[i].0 - ctrl[i - 1].0).powi(2) + (ctrl[i].1 - ctrl[i - 1].1).powi(2)).sqrt());
    }
    let mut dl = vec![0.0f32];
    for i in 1..dense.len() {
        dl.push(dl[i - 1] + ((dense[i].0 - dense[i - 1].0).powi(2) + (dense[i].1 - dense[i - 1].1).powi(2)).sqrt());
    }
    let (ct, dt) = (*cl.last().unwrap(), dl.last().unwrap().max(1e-3));
    let ws = dl
        .iter()
        .map(|&d| {
            let s = d / dt * ct;
            let k = cl.partition_point(|&c| c <= s).clamp(1, ctrl.len() - 1);
            let t = ((s - cl[k - 1]) / (cl[k] - cl[k - 1]).max(1e-3)).clamp(0.0, 1.0);
            ctrl[k - 1].2 + (ctrl[k].2 - ctrl[k - 1].2) * t
        })
        .collect();
    (dense, ws)
}

/// The oak: the trunk and scaffold limbs placed by hand (the "study from
/// nature"), everything finer grown from them.
struct Tree {
    trunk: Br,
    /// the trunk's outline (closed), root flare included
    outline: Vec<(f32, f32)>,
    limbs: Vec<Br>,
    all: Vec<Br>,
    /// the sawn stump of a lost limb
    stump: Br,
    /// forks where snow collects: (point, width)
    crotches: Vec<((f32, f32), f32)>,
}

const FORK_Y: f32 = 780.0;

fn build_tree(seed: u64) -> Tree {
    let mut rng = Rng::new(seed);
    let (fx, fy) = FOOT;
    // trunk axis: short, stout bole with a lean to the right and a twist
    let (tp, tw) = smooth(&[(fx, fy + 20.0, 124.0), (fx + 1.0, fy - 30.0, 110.0), (fx + 6.0, fy - 120.0, 100.0), (fx + 4.0, fy - 210.0, 98.0), (fx + 12.0, fy - 290.0, 104.0), (fx + 18.0, FORK_Y, 112.0)], 3.0);
    let trunk = Br { pts: tp.clone(), w: tw.clone(), dead: false, tip: false };
    // outline: the two sides with burrs and the buttress roots spreading
    // into the snow
    let bump = Fbm::new(seed as u32 + 1, 3, 40.0);
    let mut left = vec![];
    let mut right = vec![];
    for i in (0..tp.len()).step_by(3) {
        let (x, y) = tp[i];
        let wv = tw[i] * 0.5;
        // concave flare: the buttress roots spread into the snow
        let flare = ((y - (fy + 20.0)) / 22.0).exp().min(1.0);
        let l = wv * (1.0 + 1.1 * flare) + 5.0 * bump.get(0.0, y);
        let r = wv * (1.0 + 0.8 * flare) + 5.0 * bump.get(50.0, y);
        left.push((x - l, y));
        right.push((x + r, y));
    }
    let mut outline = left.clone();
    right.reverse();
    outline.extend(right);
    let top = (fx + 18.0, FORK_Y);
    // scaffold limbs (x, y, width); dead ones silver and bare
    let defs: Vec<(Vec<(f32, f32, f32)>, bool, f32)> = vec![
        // A: the great left bough: out, rising, then level and twisting
        (vec![(top.0 - 30.0, top.1 + 10.0, 50.0), (430.0, 735.0, 40.0), (370.0, 690.0, 32.0), (318.0, 668.0, 26.0), (262.0, 612.0, 21.0), (200.0, 590.0, 16.0), (150.0, 548.0, 11.0), (100.0, 530.0, 7.0)], false, 1.0),
        // B: centre-left leader, alive low, dead above (stag-head)
        (vec![(top.0 - 12.0, top.1, 52.0), (482.0, 690.0, 42.0), (470.0, 590.0, 34.0), (446.0, 500.0, 28.0), (440.0, 430.0, 24.0)], false, 0.85),
        (vec![(440.0, 432.0, 24.0), (428.0, 360.0, 19.0), (404.0, 290.0, 15.0), (415.0, 225.0, 11.0), (402.0, 170.0, 8.0), (408.0, 140.0, 6.0)], true, 0.0),
        // C: the heavy right elbow, nearly level: holds the most snow
        (vec![(top.0 + 26.0, top.1 + 14.0, 48.0), (600.0, 752.0, 38.0), (672.0, 728.0, 31.0), (748.0, 690.0, 25.0), (812.0, 676.0, 19.0), (870.0, 640.0, 13.0), (925.0, 610.0, 8.0)], false, 1.0),
        // D: centre-right riser, dead and broken off short
        (vec![(top.0 + 10.0, top.1 - 6.0, 44.0), (560.0, 680.0, 36.0), (586.0, 580.0, 30.0), (606.0, 500.0, 25.0)], false, 0.9),
        (vec![(606.0, 502.0, 25.0), (618.0, 430.0, 21.0), (636.0, 372.0, 19.0)], true, 0.0),
        // E: a lower right limb, twisting up (a reiterated sub-crown)
        (vec![(fx + 50.0, FORK_Y + 50.0, 30.0), (590.0, 800.0, 24.0), (650.0, 790.0, 19.0), (700.0, 760.0, 15.0), (736.0, 720.0, 11.0)], false, 1.1),
    ];
    let mut limbs = vec![];
    let mut all = vec![];
    let mut crotches = vec![(top, 60.0)];
    for (ctrl, dead, vigor) in &defs {
        let (p, w) = smooth(ctrl, 3.0);
        let br = Br { pts: p.clone(), w: w.clone(), dead: *dead, tip: false };
        // side branches along the scaffold: alternate sides, acrotonic
        // (more and stronger toward its outer end)
        let n = p.len();
        let mut s = rng.range(10.0, 24.0);
        let mut acc = 0.0;
        let mut side = if rng.chance(0.5) { 1.0 } else { -1.0 };
        for i in 1..n {
            acc += ((p[i].0 - p[i - 1].0).powi(2) + (p[i].1 - p[i - 1].1).powi(2)).sqrt();
            if acc < s {
                continue;
            }
            acc = 0.0;
            let t = i as f32 / n as f32;
            s = (w[i] * rng.range(0.8, 1.8)).max(9.0);
            let a = (p[i].1 - p[i - 1].1).atan2(p[i].0 - p[i - 1].0);
            if *dead {
                if rng.chance(0.4) {
                    let (ka, kw) = (a + side * rng.range(0.5, 1.1), w[i] * rng.range(0.2, 0.45));
                    grow(&mut rng, &mut all, p[i], ka, kw, true, 0.0);
                }
            } else {
                let share = rng.range(0.25, 0.6) * (0.55 + 0.7 * t);
                let ka = a + side * rng.range(0.55, 1.2);
                let kw = (w[i] * share).max(0.6);
                if kw > 8.0 {
                    crotches.push((p[i], kw));
                }
                grow(&mut rng, &mut all, p[i], ka, kw, false, *vigor);
            }
            side = -side;
        }
        if !*dead {
            let a = (p[n - 1].1 - p[n - 2].1).atan2(p[n - 1].0 - p[n - 2].0);
            grow(&mut rng, &mut all, p[n - 1], a, w[n - 1], false, *vigor);
        }
        limbs.push(br);
    }
    // epicormic shoots on the old trunk: short straight sprouts
    for k in 0..10 {
        let y = fy - 170.0 - k as f32 * 18.0 + rng.range(-8.0, 8.0);
        let side = if k % 2 == 0 { -1.0 } else { 1.0 };
        let x = fx + 8.0 + side * 50.0;
        let a = -FRAC_PI_2 + side * rng.range(0.3, 0.8);
        let kw = rng.range(0.6, 1.2);
        grow(&mut rng, &mut all, (x, y), a, kw, false, 1.3);
    }
    // the sawn stump of a lost limb, left side of the bole: a short thick
    // cylinder out and up from the bole, cut clean
    let i = tp.iter().position(|p| p.1 < fy - 236.0).unwrap_or(0);
    let base = (tp[i].0 - tw[i] * 0.36, tp[i].1);
    let a = -PI + 0.5; // out to the left and up
    let (len, sw) = (30.0, 30.0);
    let (sp, sws) = smooth(&[(base.0 + 8.0, base.1 + 6.0, sw * 1.3), (base.0 - len * 0.5 * a.cos().abs(), base.1 + len * 0.5 * a.sin(), sw * 1.05), (base.0 - len * a.cos().abs(), base.1 + len * a.sin(), sw)], 2.0);
    let stump = Br { pts: sp, w: sws, dead: false, tip: false };
    Tree { trunk, outline, limbs, all, stump, crotches }
}

/// Paint one branch as a tapering pointed stroke (or a few, for long ones).
fn twig(c: &mut Canvas, hb: &mut Held, b: &Br, col: paint::Paint, rng: &mut Rng) {
    let n = b.pts.len();
    if n < 2 {
        return;
    }
    // pieces of up to ~40 points: a hand renews the stroke along a long twig
    let mut i0 = 0;
    while i0 < n - 1 {
        let i1 = (i0 + 40).min(n - 1);
        let pts: Vec<(f32, f32)> = b.pts[i0..=i1].to_vec();
        let knots: Vec<f32> = (i0..=i1).step_by(((i1 - i0) / 6).max(1)).map(|i| hb.tool.pressure_for(b.w[i].max(0.25)).max(0.02)).collect();
        let last = i1 == n - 1;
        let rel = if last && b.tip { 0.25 } else { 0.05 };
        hb.reload(col.clone(), 0.55 + 0.2 * rng.f());
        c.drag(hb, &Gesture::new(pts).pressure(1.0, 1.0).swell(knots).ramps(0.04, rel).shake(0.35), None);
        i0 = i1;
    }
}

fn main() {
    let o = paintings::run::Run::new("r13_tree2");
    let st = Style::friedrich();
    let pal = &st.palette;
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();

    let t0 = std::time::Instant::now();
    let tree = build_tree(o.seed + 11);
    eprintln!("  tree: {} grown branches ({:.1}s)", tree.all.len(), t0.elapsed().as_secs_f32());

    // ------------------------------------------------------------ geometry
    // the snowfield: a flat plain to the horizon, the tree's low rise in
    // front, falling away gently to both sides
    let rise_c = f.per_column(move |x: f32| {
        let d = (x - FOOT.0) / 300.0;
        HORIZON + 30.0 + 26.0 * (1.0 - (-d * d).exp()) + 3.0 * (x / 70.0).sin()
    });
    let far = Fbm::new(o.seed as u32 + 5, 4, 60.0);
    // a strip of distant wood on the horizon, broken
    let wood_top = f.per_column(move |x: f32| {
        let n = far.get(x, 0.0);
        let band = smoothstep(-0.35, 0.15, n) * (if x < 330.0 { 1.0 } else if x > 690.0 { 0.8 } else { 0.35 });
        HORIZON - 1.0 - band * (4.0 + 7.0 * smoothstep(-0.2, 0.4, n) + 5.0 * far.get(x * 4.0, 9.0).abs())
    });
    let sky_m = Mask::from_fn(f, |_, y| if y < HORIZON + 3.0 { 1.0 } else { 0.0 });
    let snow_m = Mask::from_fn(f, |_, y| if y >= HORIZON - 1.0 { 1.0 } else { 0.0 });

    // sky: a luminous gray overcast, slightly violet overhead, lightening
    // to a pale warm band low down; soft darker stratus drifts
    let drift = Fbm::new(o.seed as u32 + 3, 5, 260.0);
    let sky_stops: [(f32, Rgb); 5] = [(0.0, hex("#8a8d96")), (0.35, hex("#a4a4a6")), (0.7, hex("#c2beb4")), (0.9, hex("#d8cfbc")), (1.0, hex("#ddd2bd"))];
    let sky_col = move |x: f32, y: f32| {
        let t = (y / HORIZON).clamp(0.0, 1.0);
        let base = gradient(&sky_stops, t, Mix::Pigment);
        let s = drift.get(x * 0.35, y * 1.6);
        let k = smoothstep(0.05, 0.45, s) * (1.0 - smoothstep(0.45, 0.85, t)) * 0.55;
        mix(base, hex("#7e7f86"), k)
    };

    // snow: reflecting the gray sky; the plain grayer, the rise lighter,
    // long soft level drifts, bluish in their lee, toward us
    let snowvar = Fbm::new(o.seed as u32 + 7, 4, 120.0);
    let snow_col = move |x: f32, y: f32| {
        let r = rise_c(x);
        let near = smoothstep(r, 1250.0, y);
        let n = snowvar.get(x * 0.25, y * 2.2);
        let mut col = mix(hex("#e0ddd3"), hex("#d6d6d2"), near);
        // the lee of low drifts: bluish gray bands
        col = mix(col, hex("#b3b8be"), smoothstep(0.15, 0.55, n) * (0.25 + 0.4 * near));
        // the rise turned from the light on its right flank, and a shallow
        // hollow toward us at the lower right
        let slope = (rise_c(x + 4.0) - rise_c(x - 4.0)) / 8.0;
        col = mix(col, hex("#b9bec4"), smoothstep(0.0, 0.12, -slope) * 0.35 * (1.0 - near * 0.5));
        let hx = (x - 780.0) / 260.0;
        let hy = (y - 1190.0) / 80.0;
        col = mix(col, hex("#b0b6bd"), (-(hx * hx + hy * hy)).exp() * 0.45);
        // the tree's faint shade at its foot
        let dx = (x - FOOT.0) / 170.0;
        let dy = (y - FOOT.1 - 6.0) / 26.0;
        col = mix(col, hex("#a2a7ae"), (-(dx * dx + dy * dy)).exp() * 0.4);
        if y < r {
            // the plain beyond the rise, grayer and flat
            col = mix(hex("#c9c8c2"), hex("#d4d2ca"), smoothstep(HORIZON, r, y));
        }
        col
    };

    // ---------------------------------------------------------- drawing
    if o.stage("drawing", &mut c, &mut rng) {
        // pencil on the ground: the trunk's outline and the scaffolds'
        // sides, lightly; the horizon ruled
        let hb = Lead::pencil("HB").unwrap();
        let b2 = Lead::pencil("2B").unwrap();
        let mut worn = 0.0;
        let mut line = |c: &mut Canvas, lead: &Lead, pts: Vec<(f32, f32)>, p: f32, seed: u64| {
            let n = pts.len();
            let pr = (0..n).map(|i| p * (0.75 + 0.25 * ((i as f32 * 0.37 + seed as f32).sin()))).collect();
            worn += c.draw(lead, &Mark { pts, pressure: pr }, worn, seed);
        };
        let mut ol = tree.outline.clone();
        ol.push(ol[0]);
        line(&mut c, &hb, ol, 0.3, 1);
        for (k, l) in tree.limbs.iter().enumerate() {
            let n = l.pts.len();
            line(&mut c, &hb, (0..n).map(|i| l.across(i, 1.0)).collect(), 0.25, 10 + k as u64);
            line(&mut c, &hb, (0..n).map(|i| l.across(i, -1.0)).collect(), 0.25, 20 + k as u64);
        }
        line(&mut c, &b2, vec![(0.0, HORIZON), (1000.0, HORIZON)], 0.25, 40);
        line(&mut c, &hb, (0..=40).map(|i| (i as f32 * 25.0, rise_c(i as f32 * 25.0))).collect(), 0.2, 41);
    }

    // ---------------------------------------------------------------- sky
    if o.stage("sky", &mut c, &mut rng) {
        let hd = st.broad().color(sky_col).angle(|_, _| 0.0).angle_jitter(0.1).length(40.0, 120.0).coverage(3.4).medium(0.35).clip(true).threshold(0.3);
        c.work(&sky_m, &hd, 101);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.clip(true).angle(|_, _| 0.0).angle_jitter(0.15), 102);
        }
        c.dry();
        let sp = Stipple::new(Tool::stippler(2.2)).mixed(pal, 0.45).color(sky_col).coverage(|_, _| 1.5).pressure(0.4, 0.75).dips(18, 0.4, 0.5).cluster(0.3, None).clip(true);
        c.stipple(&sky_m, &sp, 103);
        c.dry();
    }

    // --------------------------------------------------------------- snow
    if o.stage("snow", &mut c, &mut rng) {
        let wood_band = Mask::from_fn(f, |x, y| if y > wood_top(x) && y < HORIZON + 1.5 { 1.0 } else { 0.0 });
        let hd = st.body().color(snow_col).angle(|x, _| 0.02 * (x / 200.0).sin()).angle_jitter(0.06).length(25.0, 80.0).coverage(3.0).medium(0.15).clip(true).threshold(0.3);
        c.work(&snow_m, &hd, 201);
        c.wait(20.0);
        // the rise's crest: a lighter edge where it meets the plain
        let crest = Mask::from_fn(f, |x, y| {
            let d = y - rise_c(x);
            if (-2.0..6.0).contains(&d) { 1.0 - smoothstep(0.0, 6.0, d) } else { 0.0 }
        });
        let crest_h = paint::Handling::new(Tool::round_sable(3.0)).mixed(pal, 0.1).color(|_, _| hex("#ebe8df")).angle(|_, _| 0.0).angle_jitter(0.05).length(15.0, 50.0).coverage(1.3).pressure(0.4, 0.7).clip(true).threshold(0.3).fill(false);
        c.work(&crest, &crest_h, 202);
        c.dry();
        let sp = Stipple::new(Tool::stippler(1.5))
            .mixed(pal, 0.3)
            .color(|x, y| mix(hex("#7a7e88"), hex("#9c9ea4"), smoothstep(-0.3, 0.3, far.get(x * 2.0, y)) * 0.7 + 0.3 * smoothstep(HORIZON - 12.0, HORIZON, y)))
            .coverage(|_, _| 1.4)
            .pressure(0.35, 0.7)
            .dips(14, 0.4, 0.5)
            .clip(true);
        c.stipple(&wood_band, &sp, 203);
        let sp = Stipple::new(Tool::stippler(3.0)).mixed(pal, 0.2).color(snow_col).coverage(|_, y| if y > HORIZON + 40.0 { 0.8 } else { 0.35 }).pressure(0.35, 0.7).dips(16, 0.4, 0.5).cluster(0.5, None).clip(true);
        c.stipple(&snow_m, &sp, 204);
        c.dry();
    }

    // ---------------------------------------------------------- tree body
    // the wood's colors (masstones to mix): an old oak's gray bark in
    // overcast light; the dead wood silver-gray
    let bark_dark = hex("#2b241f");
    let bark_mid = hex("#453c34");
    let bark_lit = hex("#857e73");
    let bark_cool = hex("#a09f9a");
    let dead_dark = hex("#5a5650");
    let dead_lit = hex("#b4aea2");
    let twig_col = hex("#3a322c");
    // lit from the brighter sky, upper left, diffuse
    let light = |nx: f32, ny: f32| smoothstep(-0.9, 0.9, -(nx * 0.75 + ny * 0.65));
    let bodies: Vec<&Br> = std::iter::once(&tree.trunk).chain(std::iter::once(&tree.stump)).chain(tree.limbs.iter()).chain(tree.all.iter().filter(|b| b.w[0] > BODY_W)).collect();

    if o.stage("trunk", &mut c, &mut rng) {
        let t1 = std::time::Instant::now();
        let mut shape = Shape::new().smooth_poly(&tree.outline);
        for b in bodies.iter().skip(1) {
            // only the thick part of each branch is a body
            let k = b.w.iter().position(|&w| w < BODY_W).unwrap_or(b.w.len());
            if k >= 2 {
                shape = shape.ribbon(&b.pts[..k], &b.w[..k]);
            }
        }
        let wood_m = Mask::from_shape(f, shape);
        eprintln!("  wood mask {:.1}s", t1.elapsed().as_secs_f32());
        // underpaint: dark umber through the whole wood, thin
        let hd = paint::Handling::new(Tool { lay: 0.7, ..Tool::filbert(7.0) })
            .mixed(pal, 0.3)
            .color(|_, _| bark_mid)
            .angle(|_, _| -FRAC_PI_2)
            .angle_jitter(0.4)
            .length(8.0, 24.0)
            .coverage(2.4)
            .clip(true)
            .threshold(0.35);
        c.work(&wood_m, &hd, 301);
        c.wait(10.0);
        // modeling: strokes along each limb, side by side across its width,
        // shadow side to lit side
        let mut r = Rng::new(o.seed + 303);
        let mut brushes: Vec<Held> = [1.5f32, 3.0, 6.0, 10.0].iter().enumerate().map(|(i, &wd)| Held::new(Tool { point: 0.6, ..Tool::round_sable(wd) }, 310 + i as u64)).collect();
        let bi_for = |wd: f32| if wd < 1.6 { 0 } else if wd < 3.5 { 1 } else if wd < 7.0 { 2 } else { 3 };
        for b in &bodies {
            let n = b.w.iter().position(|&w| w < BODY_W).unwrap_or(b.w.len());
            if n < 3 {
                continue;
            }
            let wmax = b.w[..n].iter().cloned().fold(0.0, f32::max);
            let lanes = ((wmax / 4.5).ceil() as usize).clamp(2, 26);
            for lane in 0..lanes {
                let u = ((lane as f32 + 0.5) / lanes as f32 * 2.0 - 1.0) * 0.92;
                let mut i0 = 0;
                while i0 < n - 1 {
                    let i1 = (i0 + (r.range(8.0, 22.0) as usize)).min(n - 1);
                    let seg: Vec<(f32, f32)> = (i0..=i1).map(|i| b.across(i, u)).collect();
                    let mid = (i0 + i1) / 2;
                    let wd = b.w[mid] / lanes as f32 * 1.7;
                    let (dx, dy) = b.dir(mid);
                    let lit = light(-dy * u.signum() * u.abs().sqrt(), dx * u.signum() * u.abs().sqrt());
                    let lit = (lit * (1.0 - 0.35 * u.abs().powi(3)) + r.range(-0.08, 0.08)).clamp(0.0, 1.0);
                    let col = if b.dead {
                        mix(dead_dark, dead_lit, lit)
                    } else {
                        let base = gradient(&[(0.0, bark_dark), (0.45, bark_mid), (0.85, bark_lit), (1.0, bark_cool)], lit, Mix::Pigment);
                        mix(base, hex("#6b6a5a"), 0.2 * r.f())
                    };
                    let hb = &mut brushes[bi_for(wd)];
                    let p = hb.tool.pressure_for(wd.min(hb.tool.width)).clamp(0.15, 1.0);
                    hb.reload(pal.paint(col, 0.2), 0.7);
                    c.drag(hb, &Gesture::new(seg).pressure(p, p * 0.9).ramps(0.1, 0.2).shake(0.4), Some(&wood_m));
                    i0 = i1.saturating_sub(1).max(i0 + 1);
                }
            }
        }
        c.wait(30.0);
        // bark: oak bark fissured into long blocks. Dark fissures drawn
        // along the trunk and big limbs with a pointed rigger, broken, and
        // the lit ridges between them picked out in gray
        let mut rig = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.8) }, 330);
        let mut ridge = Held::new(Tool { point: 0.8, ..Tool::round_sable(2.0) }, 331);
        for b in bodies.iter().filter(|b| b.w[0] > 12.0) {
            let n = b.pts.len();
            let k_end = b.w.iter().position(|&w| w < 10.0).unwrap_or(n);
            let lines = (b.w[0] / 2.6) as usize;
            for _ in 0..lines {
                let u = r.range(-0.95, 0.95);
                let mut i = r.range(0.0, 12.0) as usize;
                while i + 4 < k_end {
                    let len = r.range(6.0, 20.0) as usize;
                    let j = (i + len).min(k_end - 1);
                    let wob = r.range(-0.04, 0.04);
                    let seg: Vec<(f32, f32)> = (i..=j).map(|q| b.across(q, (u + wob * (q - i) as f32 / len as f32).clamp(-0.97, 0.97))).collect();
                    let (dx, dy) = b.dir(i);
                    let lit = light(-dy * u, dx * u);
                    if b.dead {
                        // dead wood: fine grain lines, few
                        if r.chance(0.4) {
                            rig.reload(pal.paint(hex("#6c665d"), 0.3), 0.5);
                            c.drag(&mut rig, &Gesture::new(seg).pressure(0.25, 0.15).ramps(0.2, 0.3).shake(0.3), Some(&wood_m));
                        }
                    } else {
                        rig.reload(pal.paint(mix(hex("#1d1916"), bark_dark, lit), 0.25), 0.7);
                        c.drag(&mut rig, &Gesture::new(seg.clone()).pressure(r.range(0.4, 0.75), 0.3).ramps(0.15, 0.3).shake(0.15), Some(&wood_m));
                        // the ridge beside it catches the light
                        if lit > 0.35 && r.chance(0.6) {
                            let sh: Vec<(f32, f32)> = (i..=j).map(|q| b.across(q, (u - 0.07).clamp(-0.97, 0.97))).collect();
                            ridge.reload(pal.paint(mix(bark_lit, hex("#9c9a94"), lit), 0.2), 0.5);
                            c.drag(&mut ridge, &Gesture::new(sh).pressure(0.35, 0.25).ramps(0.2, 0.3).shake(0.15), Some(&wood_m));
                        }
                    }
                    i = j + r.range(1.0, 5.0) as usize;
                }
            }
        }
        // the sawn stump's cut face: pale heartwood, a few rings, a dark
        // rim of bark
        {
            let b = &tree.stump;
            let n = b.pts.len();
            let e = b.pts[n - 1];
            let (dx, dy) = b.dir(n - 1);
            let (ra, rb) = (b.w[n - 1] * 0.2, b.w[n - 1] * 0.5);
            let ell = |k: f32, sc: f32| -> Vec<(f32, f32)> {
                (0..24)
                    .map(|i| {
                        let t = i as f32 / 24.0 * 2.0 * PI;
                        let (u, v) = (ra * sc * t.cos(), rb * sc * t.sin());
                        (e.0 + dx * (u + k) - dy * v, e.1 + dy * (u + k) + dx * v)
                    })
                    .collect()
            };
            let face = Mask::from_shape(f, Shape::new().smooth_poly(&ell(0.0, 1.0)));
            let mut fb = Held::new(Tool::round_sable(2.4), 341);
            for k in 0..9 {
                let t = k as f32 / 8.0;
                let v = (t * 2.0 - 1.0) * rb;
                let a = (e.0 - dy * v - dx * ra * 1.2, e.1 + dx * v - dy * ra * 1.2);
                let z = (e.0 - dy * v + dx * ra * 1.2, e.1 + dx * v + dy * ra * 1.2);
                fb.reload(pal.paint(mix(hex("#b8ab94"), hex("#8a7a66"), t), 0.15), 0.8);
                c.drag(&mut fb, &Gesture::line(a, z).pressure(0.8, 0.8).shake(0.2), Some(&face));
            }
            // growth rings round the pith, off-centre
            let mut rg = Held::new(Tool { point: 1.0, ..Tool::rigger(0.8) }, 344);
            for k in [0.25f32, 0.45, 0.65, 0.82] {
                let mut pts = ell(-0.08 * rb, k);
                pts.push(pts[0]);
                rg.reload(pal.paint(hex("#7a6a58"), 0.3), 0.4);
                c.drag(&mut rg, &Gesture::new(pts).pressure(0.25, 0.2).ramps(0.1, 0.1).shake(0.2), Some(&face));
            }
            let mut rim = ell(0.0, 1.0);
            rim.push(rim[0]);
            rg.reload(pal.paint(hex("#2c2520"), 0.2), 0.8);
            c.drag(&mut rg, &Gesture::new(rim).pressure(0.6, 0.6).ramps(0.05, 0.05).shake(0.3), None);
        }
        // an old wound: a dark hollow with rolled callus lips, low on the
        // bole where a limb rotted out
        let (hx, hy) = (FOOT.0 + 22.0, FOOT.1 - 120.0);
        let hol = Mask::from_shape(f, Shape::new().smooth_poly(&[(hx - 7.0, hy - 26.0), (hx + 4.0, hy - 22.0), (hx + 8.0, hy), (hx + 5.0, hy + 20.0), (hx - 3.0, hy + 26.0), (hx - 8.0, hy + 4.0)]));
        let mut hbru = Held::new(Tool::round_sable(4.0), 342);
        for k in 0..6 {
            let x = hx - 8.0 + k as f32 * 3.2;
            hbru.reload(pal.paint(hex("#15110f"), 0.1), 0.9);
            c.drag(&mut hbru, &Gesture::line((x, hy - 28.0), (x + 1.0, hy + 28.0)).pressure(0.9, 0.9).shake(0.3), Some(&hol));
        }
        for (side, col) in [(-1.0f32, hex("#8a857a")), (1.0, hex("#4c433a"))] {
            let pts: Vec<(f32, f32)> = (0..=8).map(|k| {
                let t = k as f32 / 8.0;
                let yy = hy - 28.0 + 56.0 * t;
                let wv = (PI * t).sin();
                (hx + side * (3.0 + 7.5 * wv), yy)
            }).collect();
            ridge.reload(pal.paint(col, 0.15), 0.8);
            c.drag(&mut ridge, &Gesture::new(pts).pressure(0.8, 0.5).ramps(0.2, 0.3).shake(0.3), None);
        }
        // lichen: gray-green touches on the lit side of the bole
        let mut lb = Held::new(Tool::stippler(2.4), 343);
        for _ in 0..70 {
            let i = r.range(10.0, tree.trunk.pts.len() as f32 - 5.0) as usize;
            let u = r.range(-0.95, -0.2);
            let (x, y) = tree.trunk.across(i, u);
            if wood_m.sample(x, y) < 0.5 {
                continue;
            }
            lb.reload(pal.paint(mix(hex("#7f866d"), hex("#9b9d8a"), r.f()), 0.3), 0.4);
            c.touch(&mut lb, &Touch::at(x, y).pressure(r.range(0.2, 0.5)), Some(&wood_m));
        }
        c.wait(20.0);
    }

    // ------------------------------------------------------------- twigs
    if o.stage("twigs", &mut c, &mut rng) {
        let mut r = Rng::new(o.seed + 401);
        let mut fine = Held::new(Tool { point: 1.0, ..Tool::rigger(1.2) }, 402);
        let mut mid = Held::new(Tool { point: 1.0, ..Tool::round_sable(2.6) }, 403);
        let mut n = 0;
        // thicker first, then the fine net over it
        let mut order: Vec<&Br> = tree.all.iter().collect();
        order.sort_by(|a, b| b.w[0].partial_cmp(&a.w[0]).unwrap());
        for b in order {
            let k = b.w.iter().position(|&w| w < BODY_W).unwrap_or(b.w.len());
            if k >= b.pts.len() - 1 {
                continue;
            }
            let start = k.saturating_sub(1);
            let part = Br { pts: b.pts[start..].to_vec(), w: b.w[start..].to_vec(), dead: b.dead, tip: b.tip };
            let col = if b.dead { mix(dead_dark, dead_lit, 0.4) } else { mix(twig_col, hex("#4d4540"), r.f() * 0.6) };
            let paint = pal.paint(col, 0.25);
            let hb = if part.w[0] > 1.3 { &mut mid } else { &mut fine };
            twig(&mut c, hb, &part, paint, &mut r);
            n += 1;
        }
        eprintln!("  {n} twigs");
        c.wait(30.0);
    }

    // ------------------------------------------------------ snow on limbs
    if o.stage("limb snow", &mut c, &mut rng) {
        // wet snow near freezing: a ridge along the upper side of limbs that
        // are near level and thick enough; none on steep wood; gone in
        // places where it slid off. Laid in two strokes: a cooler body
        // straddling the limb's top edge, then the lit crest above it
        let mut r = Rng::new(o.seed + 501);
        let lose = Fbm::new(o.seed as u32 + 502, 3, 40.0);
        let mut sb = Held::new(Tool { point: 0.5, ..Tool::round_sable(7.0) }, 503);
        let mut sm = Held::new(Tool { point: 0.8, ..Tool::round_sable(3.5) }, 504);
        let mut sf = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.6) }, 505);
        let white = hex("#f1eee6");
        let body = hex("#d3d5d6");
        let limbs: Vec<&Br> = tree.limbs.iter().chain(std::iter::once(&tree.stump)).chain(tree.all.iter()).collect();
        for b in limbs {
            let n = b.pts.len();
            let mut runs: Vec<Vec<(usize, f32)>> = vec![];
            let mut cur: Vec<(usize, f32)> = vec![];
            for i in 0..n {
                let (_, dy) = b.dir(i);
                let wv = b.w[i];
                let hold = (1.0 - smoothstep(0.3, 0.72, dy.abs())) * smoothstep(0.8, 3.5, wv);
                let (x, y) = b.pts[i];
                if hold > 0.12 && lose.get(x, y) > -0.2 {
                    cur.push((i, (wv * 0.5).min(8.0) * hold));
                } else if !cur.is_empty() {
                    runs.push(std::mem::take(&mut cur));
                }
            }
            if !cur.is_empty() {
                runs.push(cur);
            }
            for run in runs.into_iter().filter(|r| r.len() >= 3) {
                let tmax = run.iter().map(|p| p.1).fold(0.0, f32::max);
                // the upper side of the limb at each point
                let top = |i: usize, lift: f32| {
                    let (dx, dy) = b.dir(i);
                    let (nx, ny) = if dx > 0.0 { (dy, -dx) } else { (-dy, dx) };
                    let off = b.w[i] * 0.5 + lift;
                    (b.pts[i].0 + nx * off, b.pts[i].1 + ny * off)
                };
                for (pass, (col, k, lift)) in [(body, 1.0f32, 0.0f32), (white, 0.6, 0.35)].into_iter().enumerate() {
                    let hb = if tmax * k > 3.2 { &mut sb } else if tmax * k > 1.4 { &mut sm } else { &mut sf };
                    let pts: Vec<(f32, f32)> = run.iter().map(|&(i, t)| top(i, t * lift)).collect();
                    let knots: Vec<f32> = run.iter().step_by((run.len() / 6).max(1)).map(|&(_, t)| hb.tool.pressure_for((t * k).max(0.35))).collect();
                    hb.reload(pal.paint(mix(col, hex("#c6cacd"), r.f() * 0.15), 0.02).with_stiff(0.85), 1.0);
                    let _ = pass;
                    c.drag(hb, &Gesture::new(pts).pressure(1.0, 1.0).swell(knots).ramps(0.25, 0.3).shake(0.25), None);
                }
            }
        }
        // the main fork holds a heap, and the stump a cap
        let mut hb = Held::new(Tool::filbert(8.0), 506);
        let (fx0, fy0) = (FOOT.0 + 14.0, FORK_Y + 6.0);
        for k in 0..4 {
            let s = 22.0 - k as f32 * 4.0;
            hb.reload(pal.paint(if k < 2 { body } else { white }, 0.02).with_stiff(0.85), 1.0);
            c.drag(&mut hb, &Gesture::new(vec![(fx0 - s, fy0 - s * 0.1 - k as f32), (fx0, fy0 - s * 0.45 - k as f32 * 1.5), (fx0 + s, fy0 - s * 0.05 - k as f32)]).pressure(0.8, 0.7).ramps(0.25, 0.3).shake(0.3), None);
        }
        c.wait(30.0);
    }

    // ---------------------------------------------------- foot and details
    if o.stage("foot", &mut c, &mut rng) {
        let mut r = Rng::new(o.seed + 601);
        // snow drifted against the foot of the trunk: the mound covers the
        // root flare, higher on the windward left, its edge uneven
        let (fx, fy) = FOOT;
        let edge = Fbm::new(o.seed as u32 + 602, 3, 18.0);
        let mound_top = move |x: f32| {
            let d = (x - fx - 10.0) / 120.0;
            fy + 22.0 - 30.0 * (-d * d * 2.0).exp() - 6.0 * (-((x - fx + 60.0) / 30.0).powi(2)).exp() + 4.0 * edge.get(x, 0.0)
        };
        let mound = Mask::from_fn(f, move |x, y| if (x - fx).abs() < 240.0 { smoothstep(mound_top(x) - 0.8, mound_top(x) + 0.8, y) } else { 0.0 });
        let hd = paint::Handling::new(Tool::filbert(5.0))
            .mixed(pal, 0.08)
            .color(move |x, y| {
                let top = mound_top(x);
                // lit on top, cooler down its face toward us
                mix(hex("#eeebe2"), snow_col(x, y), smoothstep(top, top + 30.0, y))
            })
            .angle(move |x, _| (mound_top(x + 2.0) - mound_top(x - 2.0)).atan2(4.0))
            .angle_jitter(0.1)
            .length(8.0, 24.0)
            .coverage(2.6)
            .clip(true)
            .threshold(0.3);
        let near = Mask::from_fn(f, move |x, y| if (x - fx).abs() < 240.0 && y < mound_top(x) + 45.0 { 1.0 } else { 0.0 });
        c.work(&mound.clone().mul(&near), &hd, 603);
        c.wait(10.0);
        // the shadowed seam where the snow meets the bark
        let mut sm = Held::new(Tool { point: 1.0, ..Tool::round_sable(2.0) }, 607);
        let xs: Vec<f32> = (0..=40).map(|i| fx - 95.0 + i as f32 * 4.6).collect();
        let seam: Vec<(f32, f32)> = xs.iter().map(|&x| (x, mound_top(x) + 0.8)).collect();
        sm.reload(pal.paint(hex("#8f959c"), 0.1), 0.6);
        c.drag(&mut sm, &Gesture::new(seam).pressure(0.45, 0.45).ramps(0.2, 0.2).shake(0.3), None);
        // dry grass and weed stalks through the snow, in patches, more on
        // the rise and near the tree: fine upturning strokes over the snow
        let patch = Fbm::new(o.seed as u32 + 608, 3, 110.0);
        let mut g = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.2) }, 604);
        let mut tries = 0;
        let mut placed = 0;
        while placed < 120 && tries < 4000 {
            tries += 1;
            let x = r.range(10.0, 990.0);
            let y0 = rise_c(x) + 2.0;
            let yb = y0 + (1250.0 - y0 - 8.0) * r.f().powf(0.8);
            let p = smoothstep(0.1, 0.45, patch.get(x, yb * 2.0)) + 0.6 * (-((x - fx) / 120.0).powi(2) - ((yb - fy - 20.0) / 40.0).powi(2)).exp();
            if !r.chance(p * 0.5) || (x - fx).abs() < 60.0 && yb < fy + 25.0 {
                continue;
            }
            placed += 1;
            let near = smoothstep(HORIZON, 1250.0, yb);
            let clump = r.range(2.0, 7.0) as usize;
            for _ in 0..clump {
                let ht = r.range(3.0, 15.0) * (0.4 + 1.2 * near);
                let lean = r.range(-0.6, 0.5);
                let x0 = x + r.range(-4.0, 4.0) * (0.5 + near);
                let col = mix(hex("#4d3f30"), hex("#8a7657"), r.f());
                g.reload(pal.paint(col, 0.2), 0.5);
                let pts = vec![(x0, yb), (x0 + lean * ht * 0.25, yb - ht * 0.55), (x0 + lean * ht, yb - ht)];
                c.drag(&mut g, &Gesture::new(pts).pressure(r.range(0.25, 0.55), 0.02).ramps(0.05, 0.7).shake(0.4), None);
            }
        }
        // dead leaves still on some low live twigs (young wood keeps its
        // leaves till spring): small brown touches, a few clusters
        let mut lb = Held::new(Tool { point: 0.8, ..Tool::round_sable(2.2) }, 605);
        let low: Vec<&Br> = tree.all.iter().filter(|b| !b.dead && b.tip && b.w[0] < 1.2 && b.pts[0].1 > 700.0).collect();
        for b in low.iter().step_by(4).take(80) {
            let (x, y) = *b.pts.last().unwrap();
            for _ in 0..r.range(1.0, 4.0) as usize {
                let a = r.range(0.0, 2.0 * PI);
                let (lx, ly) = (x + 2.0 * a.cos(), y + 2.0 * a.sin() + 1.0);
                lb.reload(pal.paint(mix(hex("#6a462a"), hex("#8c6640"), r.f()), 0.2), 0.6);
                c.drag(&mut lb, &Gesture::new(vec![(lx, ly), (lx + 1.8 * a.cos(), ly + 2.6)]).pressure(0.55, 0.25).ramps(0.2, 0.4).shake(0.5), None);
            }
        }
        // a broken branch lying half sunk in the snow below the dead limb
        let mut bb = Held::new(Tool { point: 0.6, ..Tool::round_sable(3.0) }, 606);
        let (x0, y0) = (fx - 190.0, fy + 48.0);
        bb.reload(pal.paint(hex("#5a534b"), 0.2), 0.8);
        c.drag(&mut bb, &Gesture::new(vec![(x0, y0), (x0 + 30.0, y0 + 3.0), (x0 + 58.0, y0 + 1.0), (x0 + 84.0, y0 + 6.0)]).pressure(0.8, 0.35).ramps(0.1, 0.3).shake(0.4), None);
        for &(t, a, l) in &[(0.3f32, -2.4f32, 18.0f32), (0.6, -0.8, 14.0), (0.8, -2.0, 10.0)] {
            let (px, py) = (x0 + 84.0 * t, y0 + 3.0 * t);
            fine_twig(&mut c, &mut bb, (px, py), a, l, pal.paint(hex("#4a423b"), 0.25));
        }
        bb.reload(pal.paint(hex("#eeebe3"), 0.02).with_stiff(0.85), 0.9);
        c.drag(&mut bb, &Gesture::new(vec![(x0 + 5.0, y0 - 1.8), (x0 + 40.0, y0 + 0.6), (x0 + 74.0, y0 + 2.6)]).pressure(0.5, 0.35).ramps(0.2, 0.3), None);
        // its snow-filled hollow: a bluish lee on the far side
        sm.reload(pal.paint(hex("#aab0b7"), 0.1), 0.6);
        c.drag(&mut sm, &Gesture::new(vec![(x0 - 4.0, y0 + 4.0), (x0 + 40.0, y0 + 7.5), (x0 + 88.0, y0 + 10.0)]).pressure(0.5, 0.2).ramps(0.2, 0.4).shake(0.3), None);
    }
    o.end(&mut c, &mut rng);
    c.dry();
    c.relief(st.relief.0, st.relief.1);
    o.save(&mut c);
}

fn fine_twig(c: &mut Canvas, hb: &mut Held, p: (f32, f32), a: f32, l: f32, col: paint::Paint) {
    hb.reload(col, 0.5);
    let e = (p.0 + l * a.cos(), p.1 + l * a.sin());
    let m = ((p.0 + e.0) * 0.5 + 1.0, (p.1 + e.1) * 0.5);
    c.drag(hb, &Gesture::new(vec![p, m, e]).pressure(0.4, 0.05).ramps(0.05, 0.6).shake(0.4), None);
}
