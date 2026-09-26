//! r14_p3: Evening on the Baltic shore.
//!
//! The sun has gone down a little left of center; the afterglow lies along
//! the sea's rim and a thin young moon hangs above the sun's place. A stony
//! Rügen beach runs across the foreground; granite erratics lie at the
//! water's edge on the right, one far out in the shallows. A line of
//! fishermen's stakes walks out into the sea on the left, toward the glow.
//! A woman stands at the water's edge, her head just breaking the sea's rim,
//! looking at the place where the sun went down. A small galliot lies on
//! the horizon on the right.
//!
//!   cargo paint r14_p3 -- --full --width 2400
//!   cargo paint r14_p3 -- --full --width 2400 --crop x0,y0,x1,y1
//!   cargo paint r14_p3 -- --width 1000 --plan     the sky plan unpainted (a diagnostic)
//!   cargo paint r14_p3 -- --width 1000 --probe    stone normals; --probe-sea: sea targets

use paint::color::{Mix, mix};
use paint::form::Sample;
use paint::scene::{BodyId, Sun, Water, World};
use paint::{Fbm, Gesture, Held, Mask, Rgb, Rng, Sdf, Stipple, Style, Tool, Touch, hex, shift, smoothstep};
use paintings::figures::{self, Gown, WomanPose};
use paintings::rocks::{self, Stone};
use paintings::run::{Finish, Run};

const ASPECT: f32 = 1.4;
const HZ: f32 = 430.0;
const EYE: f32 = 1.5;
const SUN_AZ: f32 = -6.5;
const SUN_EL: f32 = -3.0;

// ckpt: from sea
/// Where the shore runs (z in m, by world x in m): farther on the left,
/// coming in toward the stones on the right.
fn shore(x: f32) -> f32 {
    15.5 - 0.32 * x + 1.4 * (x * 0.31 + 0.7).sin() + 0.8 * (x * 0.83 + 2.0).sin() + 0.35 * (x * 2.1).sin() + 0.12 * (x * 5.3 + 1.0).sin()
}

fn ground(x: f32, z: f32) -> f32 {
    let s = shore(x);
    let d = s - z; // > 0 on the beach
    let bumps = 0.05 * (x * 0.9 + z * 0.4).sin() * (z * 0.7 - x * 0.3).cos() + 0.02 * (x * 2.3).sin() * (z * 1.9).cos();
    let h = if d >= 0.0 { 0.02 + 0.022 * d + bumps * smoothstep(0.0, 3.0, d) } else { 0.02 + 0.12 * d.max(-8.0) };
    h
}

struct Scene {
    world: World,
    stones: Vec<BodyId>,
    stakes: Vec<BodyId>,
    woman: BodyId,
}

fn build() -> Scene {
    let h = 1000.0 / ASPECT;
    let mut world = World::new([0.0, 0.0, 1000.0, h], HZ, EYE)
        .fov(1000.0, 52.0)
        .ground(ground)
        .water(Water::new(0.0).ripple(0.03, 1.6, 0.4, 17))
        .sun(Sun::deg(SUN_AZ, SUN_EL))
        .backdrop(4000.0)
        .visibility(9000.0);
    let mut stones = vec![];
    // the big erratic at the water's edge, right: a high shoulder on the
    // left, its back broken off in one long slope falling to the right,
    // a split across the front
    {
        let s = world.spot_bed(2.75, 12.6);
        let c = s.p(0.0, 0.6, 0.0);
        let rock = Sdf::block(c, s.size(2.0, 1.6, 1.6), s.m(0.2))
            .union(Sdf::block(s.p(1.0, 0.3, -0.1), s.size(1.1, 0.7, 1.2), s.m(0.16)), s.m(0.15))
            .turn(c, 0.35, 0.04, -0.05)
            .rough(s.m(0.11), s.m(1.1), 3, false)
            // the broken back: a long plane sloping down to the right
            .cut(s.p(0.15, 1.1, 0.0), [0.55, -1.0, 0.15], 10, s.m(0.06))
            // the steep face toward the light, left
            .cut(s.p(-0.92, 0.8, 0.0), [-1.0, -0.35, 0.3], 11, s.m(0.05))
            // a notch in the front
            .cut(s.p(0.3, 0.35, 0.78), [0.2, -0.1, 1.0], 12, s.m(0.04))
            // a chip off the shoulder
            .cut(s.p(-0.55, 1.28, 0.1), [-0.5, -0.8, 0.2], 13, s.m(0.03))
            .rough(s.m(0.006), s.m(0.18), 4, true);
        stones.push(world.place(s, rock));
    }
    // a second, lower stone behind it, half in the water: a slab split
    // off, its top tilted toward the sea
    {
        let s = world.spot_bed(4.7, 15.6);
        let c = s.p(0.0, 0.3, 0.0);
        let rock = Sdf::block(c, s.size(1.8, 0.75, 1.3), s.m(0.14))
            .turn(c, -0.35, 0.12, 0.1)
            .rough(s.m(0.05), s.m(0.6), 7, false)
            .cut(s.p(0.3, 0.55, 0.0), [-0.45, -1.0, 0.15], 10, s.m(0.03))
            .cut(s.p(0.8, 0.3, 0.0), [1.0, -0.4, 0.3], 11, s.m(0.03))
            .rough(s.m(0.005), s.m(0.15), 8, true);
        stones.push(world.place(s, rock));
    }
    // a low broken slab in the sand, left of center, nearer
    {
        let s = world.spot_bed(-1.35, 8.4);
        let c = s.p(0.0, 0.14, 0.0);
        let rock = Sdf::block(c, s.size(0.95, 0.4, 0.7), s.m(0.1))
            .turn(c, 0.5, 0.08, 0.1)
            .cut(s.p(0.2, 0.3, 0.0), [0.4, -1.0, 0.35], 10, s.m(0.025))
            .cut(s.p(-0.45, 0.15, 0.0), [-1.0, -0.3, 0.4], 11, s.m(0.02))
            .rough(s.m(0.02), s.m(0.3), 9, false)
            .rough(s.m(0.004), s.m(0.1), 10, true);
        stones.push(world.place(s, rock));
    }
    // a flat stone far out in the shallows, and a smaller one
    for (i, &(x, z, wd, ht)) in [(7.5f32, 30.0f32, 2.4f32, 0.9f32), (-1.8, 34.0, 1.0, 0.45)].iter().enumerate() {
        let s = world.spot_bed(x, z);
        let c = s.p(0.0, 0.1, 0.0);
        let rock = Sdf::block(c, s.size(wd, ht, wd * 0.7), s.m(ht * 0.4))
            .turn(c, 0.2 + 0.5 * i as f32, 0.0, 0.03)
            .rough(s.m(0.08), s.m(0.7), 11 + i as u32, false);
        stones.push(world.place(s, rock));
    }
    // stones in the beach: a group on the left, a pair near me on the
    // right, a few alone; flattened, broken, sunk
    let beach_stones = [
        (-2.6f32, 6.9f32, 0.34f32, 0.55f32),
        (-2.15, 7.25, 0.2, 0.6),
        (-3.1, 7.6, 0.16, 0.7),
        (-1.2, 10.5, 0.14, 0.6),
        (1.55, 6.3, 0.26, 0.5),
        (1.95, 6.55, 0.13, 0.75),
    ];
    for (i, &(x, z, r, flat)) in beach_stones.iter().enumerate() {
        let s = world.spot_bed(x, z);
        let c = s.p(0.0, r * flat * 0.45, 0.0);
        let k = i as f32;
        let rock = Sdf::block(c, s.size(r * 2.0, r * flat * 1.6, r * 1.5), s.m(r * 0.35))
            .turn(c, 0.4 + 0.9 * k, 0.05, 0.12 * (k * 1.7).sin())
            .cut(s.p(r * 0.3 * (k * 2.3).sin(), r * flat * 0.9, 0.0), [0.3 * (k * 1.3).cos(), -1.0, 0.25], 10, s.m(r * 0.1))
            .rough(s.m(r * 0.1), s.m(r * 0.9), 20 + i as u32, false)
            .rough(s.m(r * 0.015), s.m(r * 0.3), 40 + i as u32, true);
        stones.push(world.place(s, rock));
    }
    // the stakes: a crooked line walking out and away to the left
    let mut stakes = vec![];
    let mut r = Rng::new(404);
    // (x, z, height above the water, lean): tall ones, stumps, a pair,
    // one leaning hard where the sea has worked it loose
    let line = [
        (-2.3f32, 17.5f32, 2.3f32, 0.03f32),
        (-3.3, 20.0, 1.9, -0.05),
        (-3.55, 20.4, 1.2, 0.1),
        (-5.6, 25.5, 2.4, 0.02),
        (-6.3, 28.0, 0.6, -0.04),
        (-8.9, 35.5, 2.0, -0.2),
        (-11.5, 44.0, 2.2, 0.05),
        (-12.3, 47.5, 1.6, 0.0),
        (-15.8, 60.0, 2.3, 0.06),
        (-17.5, 66.0, 1.1, -0.08),
    ];
    for &(x, z, above, lean) in &line {
        let s = world.spot_bed(x, z);
        let depth = -s.at[1];
        let hgt = above + depth + 0.2 * r.f();
        let w = 0.12 + 0.07 * r.f();
        let pole = Sdf::block(s.p(0.0, hgt * 0.5, 0.0), s.size(w, hgt, w), s.m(0.03)).turn(s.p(0.0, 0.0, 0.0), 0.0, 0.0, lean);
        stakes.push(world.place(s, pole));
    }
    // the woman at the water's edge (a proxy: she is written in gestures)
    let wx = -0.35;
    let s = world.spot_at(wx, shore(wx) - 0.7);
    let fig = Sdf::ellipsoid(s.p(0.0, 0.45, 0.0), s.size(0.26, 0.46, 0.22))
        .union(Sdf::ellipsoid(s.p(0.0, 1.1, 0.0), s.size(0.2, 0.36, 0.14)), s.m(0.08))
        .union(Sdf::ellipsoid(s.p(0.0, 1.55, 0.0), s.size(0.1, 0.13, 0.11)), s.m(0.03));
    let woman = world.proxy(s, fig);
    Scene { world, stones, stakes, woman }
}

// ckpt: end

/// The sun's place on the canvas x (it is below the rim).
fn sun_x(w: &World) -> f32 {
    w.sun_canvas().map_or(380.0, |p| p.0)
}

/// The evening sky as I mix it: a few piles by height above the rim, the
/// glow gathered over the sun's place and falling away sideways; thin
/// streaks of cloud lying level, dark against the glow low down, catching a
/// little rose higher up. Not a formula for light: a painter's plan of
/// piles, made uneven.
struct SkyPlan {
    gx: f32,
    streak: Fbm,
    wander: Fbm,
    band: Fbm,
}

impl SkyPlan {
    fn new(gx: f32) -> Self {
        SkyPlan { gx, streak: Fbm::new(71, 5, 1.0), wander: Fbm::new(72, 3, 1.0), band: Fbm::new(73, 3, 1.0) }
    }
    /// Clear sky (no cloud).
    fn clear(&self, x: f32, y: f32) -> Rgb {
        let dx = (x - self.gx) / 1000.0;
        // height above the rim, 0 at the rim, 1 at the top edge; the bands
        // wander a little across the picture
        let h = ((HZ - y) / HZ + 0.025 * self.band.get(x / 420.0, 0.3)).max(0.0);
        let near = (-(dx * dx) / 0.09).exp(); // over the sun's place
        let near2 = (-(dx * dx) / 0.018).exp();
        let rim_warm = mix(hex("#c8a48f"), hex("#f0c78a"), near, Mix::Pigment);
        let rim_warm = mix(rim_warm, hex("#f4c282"), 0.85 * near2, Mix::Pigment);
        let yellow = mix(hex("#d8c2a6"), hex("#f0dfae"), near, Mix::Pigment);
        let pale = mix(hex("#bdb9bd"), hex("#d9dbc9"), near, Mix::Pigment);
        let lilac = hex("#9196b3");
        let top = hex("#55638e");
        // the glow reaches higher over the sun's place
        let k = 1.0 + 0.45 * near;
        let stops = [(0.0, rim_warm), (0.07 * k, yellow), (0.22 * k, pale), (0.5, lilac), (1.0, top)];
        let mut col = stops[0].1;
        for i in 0..stops.len() - 1 {
            let (a, ca) = stops[i];
            let (b, cb) = stops[i + 1];
            if h >= a {
                col = mix(ca, cb, smoothstep(0.0, 1.0, ((h - a) / (b - a)).min(1.0)), Mix::Light);
            }
        }
        col
    }
    /// How much cloud lies at a point (0..1): long level streaks in a few
    /// bands, thinning out and broken.
    fn cloud(&self, x: f32, y: f32) -> f32 {
        let h = (HZ - y) / HZ;
        if h <= 0.0 {
            return 0.0;
        }
        let yy = y + 14.0 * self.wander.get(x / 300.0, 1.7);
        // bands: a low one just over the rim, one at the glow's top, a
        // loose high one
        let bands = [(HZ - 15.0, 7.0, 0.7), (HZ - 40.0, 9.0, 0.85), (HZ - 84.0, 12.0, 0.45), (HZ - 232.0, 14.0, 0.6), (HZ - 268.0, 10.0, 0.45), (HZ - 150.0, 16.0, 0.1)];
        let mut a: f32 = 0.0;
        for (i, &(by, bw, amt)) in bands.iter().enumerate() {
            // each band wanders its own way, so they don't run parallel
            let yy = yy + 9.0 * self.wander.get(x / (180.0 + 60.0 * i as f32) + 5.3 * i as f32, 2.9 + 1.7 * i as f32);
            let t = (yy - by) / bw;
            let prof = (-t * t).exp();
            let n = self.streak.get(x / 260.0 + 3.1 * i as f32, yy / 5.5 + 1.3 * i as f32) * 0.6 + self.streak.get(x / 90.0, yy / 3.0 + 7.0 * i as f32) * 0.4;
            // the low streaks run long across the glow; the high ones break up
            let (lo, hi) = match i {
                0 => (-0.2, 0.3),
                1 => (-0.12, 0.3),
                2 => (0.0, 0.45),
                3 | 4 => (-0.15, 0.35),
                _ => (-0.05, 0.45),
            };
            // and each fades in and out along its length
            let along = if i < 3 { smoothstep(-0.35, 0.25, self.band.get(x / 230.0 + 11.0 * i as f32, 4.1 + 2.3 * i as f32)) } else { 1.0 };
            a = a.max(amt * prof * smoothstep(lo, hi, n) * along);
        }
        a.clamp(0.0, 1.0)
    }
    fn color(&self, x: f32, y: f32) -> Rgb {
        let clear = self.clear(x, y);
        let a = self.cloud(x, y);
        if a <= 0.0 {
            return clear;
        }
        let h = ((HZ - y) / HZ).max(0.0);
        let dx = (x - self.gx) / 1000.0;
        let near = (-(dx * dx) / 0.06).exp();
        // low: gray-violet in the Earth's shadow against the glow; the high
        // streaks are still in the sun: rose-gold on the glow side, cooling
        // to a lilac gray away from it
        let low = mix(hex("#6e6576"), hex("#77676e"), near, Mix::Pigment);
        let mid = mix(hex("#8a8596"), hex("#a08e94"), near, Mix::Pigment);
        let wide = (-(dx * dx) / 0.25).exp();
        let high = mix(hex("#9a90a2"), hex("#d0a89d"), wide, Mix::Light);
        let cl = mix(low, mid, smoothstep(0.08, 0.3, h), Mix::Pigment);
        let cl = mix(cl, high, smoothstep(0.42, 0.52, h), Mix::Light);
        mix(clear, cl, a * 0.8, Mix::Light)
    }
}

// ckpt: from sea
fn water_body() -> Rgb {
    hex("#262a36")
}
// ckpt: end

fn main() {
    let o = Run::new("r14_p3");
    let st = Style::friedrich();
    let pal = &st.palette;
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let hgt = c.height();
    let sc = build();
    let w = &sc.world;
    let gx = sun_x(w);
    // ckpt: from veil
    if std::env::args().any(|a| a == "--probe") {
        let v = w.view(f);
        for &b in &sc.stones {
            let sp = w.bodies[b].spot;
            for dy in [0.05f32, 0.15, 0.3] {
                let (x, y) = (sp.x, sp.y - sp.m(dy));
                if let Some(q) = v.form.sample(x, y) {
                    eprintln!("stone {b} at ({x:.0},{y:.0}) n {:?} dist {:.2} aerial {:.3} part {}", q.n, q.dist, w.aerial(q.dist), q.part);
                }
            }
        }
        return;
    }
    // ckpt: end
    let plan = SkyPlan::new(gx);
    let plan = &plan;
    let inside = move |x: f32, y: f32| (x.clamp(0.5, 999.5), y.clamp(0.5, hgt - 0.5));
    let sky_col = move |x: f32, y: f32| {
        let (x, y) = inside(x, y);
        plan.color(x, y.min(HZ - 0.3))
    };

    // ckpt: from veil
    if std::env::args().any(|a| a == "--plan") {
        // the sky plan unpainted (a diagnostic, not the painting)
        c.apply(|x, y, p| if y < HZ { sky_col(x, y) } else { p });
        o.save(&mut c);
        return;
    }
    // ckpt: end

    // ------------------------------------------------------------- sky
    let sky = Mask::from_fn(f, |_, y| if y < HZ + 1.5 { 1.0 } else { 0.0 });
    if o.stage("sky", &mut c, &mut rng) {
        // a lean dead color in the sky's own tones, a shade darker, to kill
        // the warm ground where the thin sky will run thin
        let dead = move |x: f32, y: f32| shift(sky_col(x, y), -0.025, 0.0, -0.004);
        let hd = st.broad().color(dead).medium(0.12).angle(|_, _| 0.0).angle_jitter(0.15).cross(0.25).length(40.0, 110.0).coverage(3.0).clip(true).threshold(0.3);
        c.work(&sky, &hd, 100);
        c.dry();
        let hd = st.broad().color(sky_col).angle(|_, _| 0.0).angle_jitter(0.08).length(40.0, 130.0).coverage(4.0).medium(0.35).clip(true).threshold(0.3);
        c.work(&sky, &hd, 101);
        if let Some(b) = st.blend() {
            c.work(&sky, &b.clip(true).angle(|_, _| 0.0).angle_jitter(0.12), 102);
            // the upper sky once more: it is where strokes show most
            let upper = Mask::from_fn(f, |_, y| 1.0 - smoothstep(HZ - 200.0, HZ - 120.0, y));
            if let Some(b) = st.blend() {
                c.work(&upper, &b.clip(true).angle(|_, _| 0.0).angle_jitter(0.1), 105);
            }
        }
        // the streaks again, with a smaller filbert in long level strokes
        // into the wet sky, so they keep their line and still sit in it
        let streaks = Mask::from_fn(f, move |x, y| smoothstep(0.18, 0.5, plan.cloud(x, y)));
        let hd = paint::Handling::new(Tool { lay: 0.6, ..Tool::filbert(5.0) })
            .mixed(pal, st.thin_medium)
            .color(sky_col)
            .angle(|_, _| 0.0)
            .angle_jitter(0.03)
            .curve(0.02, 0.2)
            .length(30.0, 110.0)
            .coverage(1.6)
            .pressure(0.35, 0.55)
            .dips(2, 0.4, 0.5)
            .clip(true)
            .threshold(0.2)
            .fill(false);
        c.work(&streaks, &hd, 106);
        // into the wet: a stipple taken from the paint beneath, moved a little
        // toward the plan
        let sp = Stipple::new(Tool::stippler(2.4))
            .mixed(pal, 0.45)
            .color_over(move |x, y, u| mix(u, sky_col(x, y), 0.35, Mix::Pigment))
            .coverage(|_, _| 1.4)
            .pressure(0.45, 0.8)
            .dips(18, 0.4, 0.5)
            .cluster(0.3, None)
            .clip(true);
        c.stipple(&sky, &sp, 103);
        c.dry();
        // dry: a finer stipple, barely lighter, denser toward the glow
        let sp = Stipple::new(Tool::stippler(1.6))
            .mixed(pal, 0.5)
            .color_over(|_, _, u| shift(u, 0.012, 0.0, 0.004))
            .coverage(move |x, y| 0.6 + 1.2 * (-((x - gx) / 380.0).powi(2)).exp() * smoothstep(80.0, HZ, y))
            .pressure(0.4, 0.75)
            .dips(20, 0.35, 0.6)
            .clip(true);
        c.stipple(&sky, &sp, 104);
        c.dry();
    }

    // ------------------------------------------------------------- sea
    let view = w.view(f);
    let view = &view;
    let sea_col = move |x: f32, y: f32| -> Rgb {
        let (x, y) = inside(x, y);
        let Some(m) = view.mirror(x, y) else { return water_body() };
        let (sx, sy) = m.src;
        let seen = sky_col(sx, sy.min(HZ - 0.5));
        // the sea gives the sky back a step darker and a little cooler
        let seen = shift(seen, -0.12, -0.006, -0.03);
        // a reflection is light added over the water's own dark
        let sea = mix(water_body(), seen, (0.04 + 0.5 * m.fresnel.sqrt()).min(1.0), Mix::Light);
        // under the sun's place the ripples give back the glow: a broad soft
        // path, brightest near the rim
        let k = (-((x - gx) / (70.0 + 0.9 * (y - HZ))).powi(2)).exp() * (1.0 - smoothstep(HZ, HZ + 115.0, y));
        mix(sea, shift(sky_col(gx, HZ - 6.0), -0.14, 0.0, -0.01), 0.5 * k, Mix::Light)
    };
    let sea = Mask::from_fn(f, move |x, y| if y > HZ - 0.8 && view.at(x, y).what == paint::scene::What::Water { 1.0 } else { 0.0 });
    // ckpt: from veil
    if std::env::args().any(|a| a == "--probe-sea") {
        let enc = |v: f32| (255.0 * if v <= 0.0031308 { 12.92 * v } else { 1.055 * v.powf(1.0 / 2.4) - 0.055 }) as i32;
        let v = w.view(f);
        for yy in (600..700).step_by(4) {
            let row: String = (0..100).map(|i| match v.at(i as f32 * 10.0, yy as f32).what { paint::scene::What::Water => '~', paint::scene::What::Ground => '.', _ => '#' }).collect();
            eprintln!("{yy} {row}");
        }
        for &(x, y) in &[(500.0f32, 440.0f32), (500.0, 470.0), (600.0, 500.0), (380.0, 450.0), (900.0, 470.0)] {
            let t = sea_col(x, y);
            eprintln!("sea target at ({x},{y}): {:?}", t.map(enc));
        }
        return;
    }
    // ckpt: end
    if o.stage("sea", &mut c, &mut rng) {
        let hd = st.body().color(sea_col).angle(|_, _| 0.0).angle_jitter(0.02).curve(0.01, 0.1).length(25.0, 80.0).coverage(3.6).clip(true).threshold(0.3);
        c.work(&sea, &hd, 201);
        if let Some(b) = st.blend() {
            c.work(&sea, &b.clip(true).angle(|_, _| 0.0).angle_jitter(0.02).cross(0.03), 202);
        }
        // glints in the path: short broken level touches of the glow's
        // color, into the wet
        let path = Mask::from_fn(f, move |x, y| {
            let k = (-((x - gx) / (45.0 + 0.8 * (y - HZ))).powi(2)).exp() * (1.0 - smoothstep(HZ + 2.0, HZ + 80.0, y));
            smoothstep(0.3, 0.9, k)
        })
        .mul(&sea);
        let glint = move |_: f32, y: f32| shift(sky_col(gx, HZ - 4.0), -0.1 - 0.1 * smoothstep(HZ, HZ + 90.0, y), 0.0, -0.01);
        let hd = paint::Handling::new(Tool::round_sable(1.3)).mixed(pal, 0.2).color(glint).angle(|_, _| 0.0).angle_jitter(0.03).curve(0.0, 0.0).length(6.0, 26.0).coverage(0.07).pressure(0.25, 0.45).clip(true).threshold(0.2).fill(false);
        c.work(&path, &hd, 203);
        // the long swell: faint level lines, lighter where their backs take
        // the sky, closer together as they go back
        let mut r = Rng::new(204);
        let mut z = 22.0f32;
        let sea_m = &sea;
        while z < 900.0 {
            let y = HZ + 1072.0 * EYE / z * (1.0 - 0.0) ;
            let mut x = r.range(-20.0, 30.0);
            while x < 1000.0 {
                let len = r.range(40.0, 180.0) * (0.5 + 0.5 * (22.0 / z).sqrt());
                let yy = y + r.range(-0.4, 0.4);
                if sea_m.sample(x + len * 0.5, yy) > 0.5 {
                    let light = r.f() < 0.6;
                    let col = shift(sea_col(x + len * 0.5, yy), if light { 0.045 } else { -0.035 }, 0.0, -0.006);
                    let wdt = (0.5 + 1.6 * (22.0 / z)).min(1.8);
                    let mut b = Held::new(Tool::round_sable(wdt), 2040 + (z * 10.0) as u64 + x as u64);
                    b.load(pal.paint(col, 0.35), 0.5);
                    c.drag(&mut b, &Gesture::new(vec![(x, yy), (x + len * 0.5, yy + r.range(-0.3, 0.3)), (x + len, yy)]).pressure(0.3, 0.3).ramps(0.3, 0.4).shake(0.2), Some(sea_m));
                }
                x += len + r.range(20.0, 160.0);
            }
            z *= 1.18 + 0.12 * r.f();
        }
        c.dry();
    }

    // ------------------------------------------------------------- beach
    let land = view.land();
    let grain = Fbm::new(91, 4, 30.0);
    let bands = Fbm::new(92, 3, 1.0);
    let beach_col = move |x: f32, y: f32| -> Rgb {
        let (x, y) = inside(x, y);
        let p = view.at(x, y);
        let (wx, wz) = (p.at[0], p.at[2]);
        let d = shore(wx) - wz;
        let far = hex("#433936");
        let near = hex("#282120");
        let base = mix(far, near, smoothstep(12.0, 5.5, wz), Mix::Pigment);
        // the sand lies in low level ridges and troughs the wind made
        let band = bands.get(wx / 4.0, wz * 1.3);
        let base = shift(base, 0.03 * band, 0.0, 0.004 * band);
        let base = mix(base, hex("#4a403c"), 0.35 * grain.get01(x, y * 2.5), Mix::Pigment);
        // the tide's wrack: a dark broken line of weed
        let wr = (-((d - 3.0 - 0.6 * bands.get(wx / 2.0, 3.3)) / 0.45).powi(2)).exp() * smoothstep(-0.2, 0.3, bands.get(wx / 1.3, 7.1));
        let base = mix(base, hex("#1f1b19"), 0.8 * wr, Mix::Pigment);
        // the wet strip at the water's edge gives back the sky, darker
        let wet = (1.0 - smoothstep(0.1, 1.4 + 0.8 * bands.get01(wx / 1.7, 5.0), d)) * (0.55 + 0.45 * bands.get01(wx / 0.9, 9.0));
        // looking toward the glow, wet sand gives back the low sky like water,
        // a little duller
        let refl = mix(water_body(), shift(sky_col(x, HZ - (y - HZ) * 0.5), -0.12, -0.004, -0.03), 0.5, Mix::Light);
        mix(base, refl, 0.85 * wet, Mix::Pigment)
    };
    if o.stage("beach", &mut c, &mut rng) {
        let hd = st.body().color(beach_col).angle(|_, _| 0.0).angle_jitter(0.1).length(15.0, 45.0).coverage(3.4).clip(true).threshold(0.3);
        c.work(&land, &hd, 301);
        // wind ripples in the sand: thin broken lines a little darker than
        // the sand, lying across, closer together as they go back
        let mut r = Rng::new(305);
        let mut z = 5.8f32;
        while z < 14.5 {
            let mut x = -0.7 * z + r.range(0.0, 1.0);
            while x < 0.7 * z {
                let len = r.range(0.6, 2.8) * (0.6 + z * 0.06);
                let s0 = w.spot_at(x, z);
                if shore(x) - z > 1.6 && s0.y < 714.0 && !w.is_water(x, z) && !w.is_water(x + len, z) {
                    let pts: Vec<(f32, f32)> = (0..=5)
                        .map(|k| {
                            let xx = x + len * k as f32 / 5.0;
                            let zz = z + 0.06 * (xx * 2.1 + z).sin();
                            let p = w.spot_at(xx, zz);
                            (p.x, p.y)
                        })
                        .collect();
                    let col = shift(beach_col(s0.x, s0.y), -0.03 - 0.02 * r.f(), 0.0, -0.002);
                    let mut b = Held::new(Tool::round_sable(s0.m(0.03).max(0.6)), 3050 + (z * 100.0) as u64 + (x * 10.0) as u64);
                    b.load(pal.paint(col, 0.2), 0.6);
                    c.drag(&mut b, &Gesture::new(pts).pressure(0.35, 0.3).ramps(0.3, 0.4).shake(0.3), None);
                }
                x += len + r.range(0.3, 2.2);
            }
            z += 0.28 + 0.07 * z * r.range(0.6, 1.4);
        }
        c.dry();
        // pebbles: small, each one seen, a darker body with the sky on its
        // upper edge; in drifts, many near the wrack line, sizes by depth
        let mut r = Rng::new(306);
        let drift = Fbm::new(307, 3, 1.0);
        let mut n = 0u64;
        for _ in 0..14000 {
            let z = 5.6 + 9.0 * r.f().powf(1.6);
            let x = r.range(-0.62, 0.62) * z;
            let d = shore(x) - z;
            if d < 0.8 {
                continue;
            }
            let wrack = (-((d - 3.0) / 1.0).powi(2)).exp();
            let dens = 0.015 + 0.3 * smoothstep(0.5, 0.85, drift.get01(x / 1.8, z / 1.2)) + 0.45 * wrack;
            if r.f() > dens {
                continue;
            }
            let s0 = w.spot_at(x, z);
            if s0.y > 716.0 || w.is_water(x, z) || w.is_water(x - 0.15, z) || w.is_water(x + 0.15, z) {
                continue;
            }
            let m = 0.012 + 0.045 * r.f().powf(3.0);
            let size = s0.m(m);
            if size < 0.6 {
                continue;
            }
            // seen from eye height a pebble lying on the sand is a flat dash
            let flat = (EYE / z).clamp(0.12, 0.35) * r.range(1.0, 1.8);
            let base = beach_col(s0.x, s0.y);
            let k = r.f();
            let body = if k < 0.95 { shift(base, -0.06 - 0.05 * r.f(), 0.0, -0.004) } else { shift(base, 0.015 + 0.015 * r.f(), -0.003, -0.008) };
            let th = (size * flat * 1.1).max(0.5);
            let mut b = Held::new(Tool::round_sable(th), 30600 + n);
            b.load(pal.paint(body, 0.12), 0.6);
            let a = r.range(-0.12, 0.12);
            let (hx, hy) = (size * 0.8 * a.cos(), size * 0.8 * a.sin());
            c.drag(&mut b, &Gesture::line((s0.x - hx, s0.y - th * 0.5 - hy), (s0.x + hx, s0.y - th * 0.5 + hy)).pressure(0.75, 0.7).ramps(0.25, 0.3), None);
            if k < 0.95 && r.f() < 0.25 {
                let top = mix(body, hex("#66656c"), 0.4, Mix::Pigment);
                let mut b = Held::new(Tool::round_sable((th * 0.45).max(0.4)), 60600 + n);
                b.load(pal.paint(top, 0.15), 0.4);
                c.drag(&mut b, &Gesture::line((s0.x - hx * 0.7, s0.y - th * 0.95 - hy), (s0.x + hx * 0.4, s0.y - th * 0.95 + hy * 0.4)).pressure(0.5, 0.3).ramps(0.3, 0.5), None);
            }
            n += 1;
        }
        eprintln!("pebbles: {n}");
        c.dry();
    }

    // ------------------------------------------------------------- stones
    let dusk = Stone {
        light: hex("#8d8580"),
        half: hex("#625b58"),
        shadow: hex("#403d41"),
        core: hex("#29272a"),
        bounce: hex("#4b4139"),
        crevice: hex("#171516"),
    };
    if o.stage("stones", &mut c, &mut rng) {
        let parts: Vec<_> = sc.stones.iter().map(|&b| view.part(b)).collect();
        // contre-jour after sunset: the faces toward me are in the shadow of
        // the stone, lit only by the dim eastern sky behind me; the planes
        // that face up take the sky's cool light; the faces turned toward
        // the glow (left) take a little warmth; the foot the beach's dark
        let big = [view.part(sc.stones[0]), view.part(sc.stones[1])];
        let rock_col = move |_: f32, _: f32, s: &Sample| {
            let up = (-s.n[1]).max(0.0);
            let top = if big.contains(&s.part) { hex("#4f4f57") } else { mix(hex("#3b393e"), hex("#2c2729"), smoothstep(9.0, 6.2, s.dist), Mix::Pigment) };
            let left = (-s.n[0]).max(0.0);
            let shadow = hex("#28252a");
            let warm = hex("#483e3a");
            let mut col = mix(shadow, dusk.core, smoothstep(0.3, 0.9, s.n[2]) * 0.4, Mix::Pigment);
            col = mix(col, warm, 0.35 * smoothstep(0.35, 0.85, left) * (1.0 - up), Mix::Pigment);
            col = mix(col, top, 0.8 * smoothstep(0.3, 1.0, up).powf(1.4), Mix::Pigment);
            mix(col, sky_col(500.0, HZ - 2.0), w.aerial(s.dist), Mix::Light)
        };
        rocks::paint_solid(&mut c, &st, &view.form, &rocks::Solid { parts, color: &rock_col, soft: &|_| 0.6, scale: 0.7, accents: 0.3, seed: 401 });
        c.dry();
        // where they sit: the dark in the sand close round them, then the
        // sand (or the shallow water) carried back over the foot in short
        // level strokes, broken
        let seam = view.contact(0.25).map(|v| if v < 0.05 { 0.0 } else { v });
        let umber = pal.only(&["raw umber", "bone black"]).mix(hex("#221d1a")).paint(0.85).pigment();
        c.glaze(&umber, Some(&seam), |_, _| 1.4);
        // pebbles and sand lying against each foot, over the bottom of its
        // outline, so no stone ends in a ruled cut
        let form = &view.form;
        let mut r = Rng::new(403);
        let mut n = 0u64;
        for &bd in &sc.stones {
            let part = view.part(bd);
            let sp = w.bodies[bd].spot;
            let mut foot = vec![];
            let mut x = sp.x - sp.m(3.0);
            while x < sp.x + sp.m(3.0) {
                let mut y = sp.y + sp.m(0.4);
                let top = sp.y - sp.m(2.0);
                while y > top {
                    if form.sample(x, y).is_some_and(|q| q.part == part) {
                        foot.push((x, y));
                        break;
                    }
                    y -= 0.4;
                }
                x += 0.5;
            }
            if foot.is_empty() {
                continue;
            }
            let k = (foot.len() as f32 * 0.5 / sp.m(0.06).max(1.0)) as usize + 2;
            for _ in 0..k {
                let (fx, fy) = foot[((r.f() * foot.len() as f32) as usize).min(foot.len() - 1)];
                if view.at(fx, fy + 1.5).what == paint::scene::What::Water {
                    continue;
                }
                let size = sp.m(0.02 + 0.07 * r.f().powf(2.0));
                let th = (size * (EYE / sp.at[2]).clamp(0.12, 0.35) * 1.3).max(0.5);
                let base = beach_col(fx, fy + 2.0);
                let body = shift(base, -0.06 - 0.05 * r.f(), 0.0, -0.004);
                let mut b = Held::new(Tool::round_sable(th), 40300 + n);
                b.load(pal.paint(body, 0.12), 0.6);
                let yy = fy + th * r.range(-0.2, 0.5);
                c.drag(&mut b, &Gesture::line((fx - size * 0.8, yy), (fx + size * 0.8, yy + r.range(-0.2, 0.2))).pressure(0.75, 0.7).ramps(0.25, 0.3), None);
                n += 1;
            }
        }
        c.dry();
    }

    // ------------------------------------------------------------- stakes
    if o.stage("stakes", &mut c, &mut rng) {
        for (i, &b) in sc.stakes.iter().enumerate() {
            let part = view.part(b);
            let s = w.bodies[b].spot;
            // only what stands above the water: the rest is under it
            let wl = w.project([s.at[0], 0.0, s.at[2]]).map_or(s.y, |p| p.1);
            let sil = view.form.silhouette(&[part], |_| 0.2).mul(&Mask::from_fn(f, move |_, y| 1.0 - smoothstep(wl - 0.2, wl + 0.4, y)));
            let wood = hex("#2a2624");
            let tool = Tool::round_sable(s.m(0.1).max(0.8));
            let hd = paint::Handling::new(tool).mixed(pal, 0.15).color(move |_, _| wood).angle(|_, _| std::f32::consts::FRAC_PI_2).length(s.m(0.5), s.m(1.2)).coverage(3.0).clip(true).threshold(0.2);
            c.work(&sil, &hd, 501 + i as u64);
        }
        c.wait(20.0);
        // the wood: a few drags up each post along its own line, darker and
        // grayer, wobbling, so it is weathered timber and not a cut shape
        let form = &view.form;
        let mut r = Rng::new(530);
        for (i, &bd) in sc.stakes.iter().enumerate() {
            let part = view.part(bd);
            let sp = w.bodies[bd].spot;
            let wl = w.project([sp.at[0], 0.0, sp.at[2]]).map_or(sp.y, |p| p.1);
            let mut line = vec![];
            let mut y = wl - 0.5;
            loop {
                let (mut lo, mut hi) = (f32::MAX, f32::MIN);
                let mut x = sp.x - 30.0;
                while x < sp.x + 30.0 {
                    if form.sample(x, y).is_some_and(|q| q.part == part) {
                        lo = lo.min(x);
                        hi = hi.max(x);
                    }
                    x += 0.25;
                }
                if lo > hi {
                    break;
                }
                line.push(((lo + hi) * 0.5, y, (hi - lo) * 0.5));
                y -= 1.5;
            }
            if line.len() < 3 {
                continue;
            }
            let hw = line.iter().map(|p| p.2).sum::<f32>() / line.len() as f32;
            let sil = form.silhouette(&[part], |_| 0.2).mul(&Mask::from_fn(f, move |_, y| 1.0 - smoothstep(wl - 0.2, wl + 0.4, y)));
            for k in 0..3 {
                let off = hw * r.range(-0.45, 0.45);
                let col = [hex("#2d2825"), hex("#3a3531"), hex("#221e1d")][k];
                let mut b = Held::new(Tool::round_sable((hw * r.range(0.6, 1.0)).max(0.5)), 5300 + 10 * i as u64 + k as u64);
                b.load(pal.paint(col, 0.15), 0.6);
                let a = r.range(0.0, 0.3);
                let b0 = r.range(0.7, 1.0);
                let n = line.len();
                let pts: Vec<(f32, f32)> = line[(a * n as f32) as usize..((b0 * n as f32) as usize).max((a * n as f32) as usize + 2).min(n)]
                    .iter()
                    .map(|&(x, y, _)| (x + off, y))
                    .collect();
                if pts.len() >= 2 {
                    c.drag(&mut b, &Gesture::new(pts).pressure(0.5, 0.35).ramps(0.2, 0.3).shake(0.6), Some(&sil));
                }
            }
        }
        c.dry();
        // the side of each stake turned to the glow, and its weathered top,
        // take a little light
        let form = &view.form;
        let parts: Vec<_> = sc.stakes.iter().map(|&b| (view.part(b), w.bodies[b].spot.x)).collect();
        let parts = &parts;
        let rim = form
            .mask(move |q| {
                let Some(&(_, sx)) = parts.iter().find(|p| p.0 == q.part) else { return 0.0 };
                let toward = if sx < gx { q.n[0] } else { -q.n[0] };
                smoothstep(0.35, 0.8, toward).max(smoothstep(0.4, 0.9, -q.n[1]))
            });
        let rimc = move |x: f32, _: f32| mix(hex("#2a2624"), sky_col(x, HZ - 8.0), 0.3, Mix::Pigment);
        let hd = paint::Handling::new(Tool::round_sable(0.8)).mixed(pal, 0.15).color(rimc).angle(|_, _| std::f32::consts::FRAC_PI_2).length(4.0, 14.0).coverage(1.2).pressure(0.4, 0.6).clip(true).threshold(0.3);
        c.work(&rim, &hd, 515);
        // their images in the water, broken by the ripples
        // their images in the still water, mirrored about each stake's own
        // waterline, broken sideways by the ripples and fading with depth
        let lines: Vec<(u16, f32, f32)> = sc
            .stakes
            .iter()
            .map(|&b| {
                let sp = w.bodies[b].spot;
                let wl = w.project([sp.at[0], 0.0, sp.at[2]]).map_or(sp.y, |p| p.1);
                (view.part(b), wl, sp.s)
            })
            .collect();
        let lines = &lines;
        let ripple = Fbm::new(521, 3, 1.0);
        let form = &view.form;
        let refl = Mask::from_fn(f, move |x, y| {
            if view.at(x, y).what != paint::scene::What::Water {
                return 0.0;
            }
            let mut k: f32 = 0.0;
            for &(part, wl, sc_) in lines {
                let dy = y - wl;
                if dy <= 0.0 || dy > 3.0 * sc_ {
                    continue;
                }
                let wob = sc_ * 0.05 * ripple.get(x / (sc_ * 0.4), y / (sc_ * 0.05)) * (0.3 + dy / sc_);
                if form.sample(x + wob, wl - dy).is_some_and(|q| q.part == part) {
                    let fade = 1.0 - smoothstep(0.3 * sc_, 2.6 * sc_, dy);
                    // the ripples break the image into level bars
                    let bar = smoothstep(-0.35, 0.2, ripple.get(x / (sc_ * 1.5), y / (sc_ * 0.035)));
                    k = k.max(fade * (0.15 + 0.85 * bar));
                }
            }
            k
        });
        let mcol = move |x: f32, y: f32| -> Rgb {
            let base = sea_col(x, y);
            mix(base, hex("#29272c"), 0.72, Mix::Pigment)
        };
        let hd = paint::Handling::new(Tool::round_sable(1.0)).mixed(pal, 0.2).color(mcol).angle(|_, _| 0.0).angle_jitter(0.04).curve(0.0, 0.0).length(1.5, 5.0).coverage(2.2).clip(true).threshold(0.15);
        c.work(&refl, &hd, 520);
        c.dry();
    }

    // ------------------------------------------------------------- woman
    if o.stage("woman", &mut c, &mut rng) {
        let ws = w.bodies[sc.woman].spot;
        let gown = Gown {
            dress: pal.paint(hex("#252529"), 0.12),
            shawl: Some(pal.only(&["red earth", "raw umber", "bone black"]).paint(hex("#221a1b"), 0.12)),
            hair: pal.paint(hex("#221c18"), 0.1),
            skin: pal.paint(hex("#27232a"), 0.1),
            rim: Some(pal.paint(hex("#8f8272"), 0.3)),
        };
        figures::woman(&mut c, (ws.x, ws.y), ws.m(1.66), &gown, WomanPose::Standing, 601);
        c.wait(15.0);
        // what makes her this woman and not the pattern: one end of the
        // shawl hanging free on her right, lifted a little by the air off the
        // sea; her head turned a little toward the glow (the knot of hair
        // goes right, the cheek catches the rim); weight on her right foot,
        // so the hem dips on that side
        let mut h = paint::Hand::new((ws.x, ws.y), ws.m(1.66), 611);
        h.tremor = 0.002;
        let shawl = pal.only(&["red earth", "raw umber", "bone black"]).paint(hex("#221a1b"), 0.12);
        let mut b = h.take(Tool::round_sable, 0.03, shawl, 0.9);
        h.mark(&mut c, &mut b, paint::Mark { pts: &[(0.07, 0.83), (0.096, 0.74), (0.112, 0.63), (0.121, 0.585)], pressure: (0.8, 0.25), ramps: (0.05, 0.45) }, None);
        b.reload(shawl, 0.6);
        h.mark(&mut c, &mut b, paint::Mark { pts: &[(0.085, 0.78), (0.104, 0.68), (0.114, 0.61)], pressure: (0.6, 0.2), ramps: (0.1, 0.5) }, None);
        let hair = pal.paint(hex("#221c18"), 0.1);
        let mut b = h.take(Tool::round_sable, 0.026, hair, 0.7);
        h.dab(&mut c, &mut b, 0.014, 0.948, 0.022, 0.2, 0.7);
        let cheek = pal.paint(mix(hex("#252529"), hex("#8f8272"), 0.3, Mix::Pigment), 0.25);
        let mut b = h.take(Tool::round_sable, 0.008, cheek, 0.3);
        h.mark(&mut c, &mut b, paint::Mark { pts: &[(-0.036, 0.935), (-0.041, 0.91), (-0.036, 0.885)], pressure: (0.3, 0.2), ramps: (0.3, 0.4) }, None);
        let dress = pal.paint(hex("#252529"), 0.12);
        let mut b = h.take(Tool::round_sable, 0.03, dress, 0.9);
        h.mark(&mut c, &mut b, paint::Mark { pts: &[(-0.1, 0.02), (0.0, 0.006), (0.09, -0.004), (0.125, -0.006)], pressure: (0.7, 0.8), ramps: (0.1, 0.2) }, None);
        c.dry();
    }

    // ------------------------------------------------------------- moon
    if o.stage("moon", &mut c, &mut rng) {
        let (mx, my, r) = (gx + 175.0, 196.0, 7.6);
        // the lit limb faces the sun below the rim
        let to_sun = ((HZ + 40.0) - my).atan2(gx - mx);
        let mut b = Held::new(Tool::round_sable(1.7), 7);
        b.load(pal.paint(hex("#f1ead2"), 0.12), 1.0);
        let pts: Vec<(f32, f32)> = (0..=12)
            .map(|k| {
                let a = to_sun - 1.45 + 2.9 * k as f32 / 12.0;
                (mx + r * a.cos(), my + r * a.sin())
            })
            .collect();
        c.drag(&mut b, &Gesture::new(pts).pressure(0.18, 0.18).swell(vec![0.25, 0.85, 0.25]).ramps(0.3, 0.3), None);
        c.dry();
    }

    // ------------------------------------------------------------- ship
    if o.stage("ship", &mut c, &mut rng) {
        let (sx, sy) = (772.0, HZ + 0.6);
        let hull = pal.paint(hex("#4c4852"), 0.15);
        let sail = pal.paint(hex("#7f7a86"), 0.2);
        let mut b = Held::new(Tool::round_sable(1.1), 8);
        b.load(hull, 0.9);
        c.drag(&mut b, &Gesture::line((sx - 9.0, sy - 1.2), (sx + 8.0, sy - 1.5)).pressure(0.6, 0.6), None);
        let mut m = Held::new(Tool::rigger(0.35), 9);
        m.load(hull, 0.8);
        c.drag(&mut m, &Gesture::line((sx - 2.0, sy - 2.0), (sx - 2.3, sy - 21.0)).pressure(0.5, 0.3), None);
        c.drag(&mut m, &Gesture::line((sx + 4.5, sy - 2.0), (sx + 4.3, sy - 15.0)).pressure(0.5, 0.3), None);
        let mut s = Held::new(Tool::round_sable(1.0), 10);
        s.load(sail, 0.8);
        for k in 0..6 {
            let y = sy - 4.0 - 2.4 * k as f32;
            let half = 4.2 - 0.35 * k as f32;
            c.drag(&mut s, &Gesture::line((sx - 2.2 - half, y), (sx - 2.2 + half, y + 0.2)).pressure(0.55, 0.55), None);
        }
        for k in 0..4 {
            let y = sy - 4.0 - 2.4 * k as f32;
            let half = 3.0 - 0.35 * k as f32;
            c.drag(&mut s, &Gesture::line((sx + 4.4 - half, y), (sx + 4.4 + half, y + 0.2)).pressure(0.5, 0.5), None);
        }
        c.dry();
    }

    // ------------------------------------------------------------- gulls
    if o.stage("gulls", &mut c, &mut rng) {
        // three gulls going home over the water, dark against the glow:
        // two wing flicks lifted off toward the tips from a body touch
        let dark = pal.paint(hex("#554f58"), 0.15);
        for &(x, y, span, tilt, lift) in &[(640.0f32, 352.0f32, 7.5f32, 0.12f32, 0.25f32), (668.0, 366.0, 5.5, -0.08, -0.1), (598.0, 381.0, 4.2, 0.05, 0.4)] {
            let mut b = Held::new(Tool::round_sable(0.9), (x * 10.0) as u64);
            for side in [-1.0f32, 1.0] {
                b.reload(dark, 0.7);
                let up = span * (0.32 + lift * 0.5 * side) + tilt * side * span;
                let mid = (x + side * span * 0.45, y - span * 0.28 - tilt * side * span * 0.5);
                let tip = (x + side * span, y - up);
                c.drag(&mut b, &Gesture::new(vec![(x, y), mid, tip]).pressure(0.7, 0.0).ramps(0.1, 0.75).shake(0.4), None);
            }
            c.touch(&mut b, &Touch::at(x, y + 0.3).pressure(0.4), None);
        }
        c.dry();
    }

    // ------------------------------------------------------------- veil
    if o.stage("veil", &mut c, &mut rng) {
        // a neutral dark, not umber: an umber veil over the blue sky grays it
        let umber = pal.only(&["raw umber", "bone black", "cobalt blue"]).mix(hex("#26262b")).paint(0.85).pigment();
        let wander = Fbm::new(77, 3, 400.0);
        c.glaze(&umber, None, move |x, y| {
            let dx = (x - gx) / 700.0;
            let dy = (y - (HZ - 30.0)) / 520.0;
            let d = (dx * dx + dy * dy).sqrt();
            (0.25 * smoothstep(0.3, 1.1, d) + 0.3 * smoothstep(570.0, 714.0, y)) * (0.8 + 0.4 * wander.get01(x, y))
        });
    }

    // a quieter craquelure: at this size the default opening drew a dark
    // net over the pale sky
    let mut fin = Finish::aged(st.relief);
    fin.varnish_coats = 0.15;
    fin.cracks = Some(paint::Cracks { width_um: Some(9.0), dirt: 0.25, veil: 0.3, grime: 0.4, ..paint::Cracks::aged(0) });
    o.finish(&mut c, &mut rng, &fin);
}
