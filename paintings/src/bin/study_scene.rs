//! One world, one sun: the same small shore under three suns, painted with
//! brushes from the engine's `scene` (camera, ground, water, one sun, cast
//! shadows, contact, reflections). Everything here (the colors, the order of
//! passes, the brushes) is this study's painting; the engine only says where
//! things are and how the one sun lights them.
//!
//!   top:    early morning, the sun low on the left (long shadows to the right)
//!   middle: noon, the sun high behind the painter's left shoulder
//!   bottom: dawn before sunrise, the glow over the sea ahead right; no cast
//!           shadows, only contact and reflections
//!
//! In each: two fishermen's poles standing in the shallows, a granite
//! boulder on the beach, a woman at the water's edge, a trodden path
//! running back to her, a far low shore.
//!
//!   cargo paint study_scene
//!   cargo paint study_scene -- --only morning --full --crop 380,120,900,330   figure, shadow, boulder at 3200px
//!   cargo paint study_scene -- --masks        the engine's fields unpainted (value; cast blue, contact red, reflections green)

use paint::color::{Mix, mix};
use paint::form::Sample;
use paint::scene::{BodyId, Sun, View, Water, World};
use paint::{Canvas, Fbm, Mask, Rgb, Sdf, Shape, Style, Tool, hex, smoothstep};
use paintings::figures::{self, Gown, WomanPose};
use paintings::rocks::{self, Stone};

const ASPECT: f32 = 1000.0 / 1320.0;
const PANEL_H: f32 = 430.0;
const GAP: f32 = 15.0;

/// One of the three lights, and the colors a painter mixes for it.
struct Light {
    name: &'static str,
    sun: Sun,
    sky_top: Rgb,
    sky_low: Rgb,
    /// The glow over the sun's place on the horizon.
    glow: Rgb,
    glow_amt: f32,
    sand_lit: Rgb,
    sand_shade: Rgb,
    water: Rgb,
    shore: Rgb,
    stone: Stone,
    wood_lit: Rgb,
    wood_shade: Rgb,
    ripple: f32,
}

fn lights() -> [Light; 3] {
    [
        Light {
            name: "morning",
            sun: Sun::deg(-72.0, 9.0),
            sky_top: hex("#8397ad"),
            sky_low: hex("#e2d6b9"),
            glow: hex("#f1dcae"),
            glow_amt: 0.55,
            sand_lit: hex("#cdb286"),
            sand_shade: hex("#6f6d6f"),
            water: hex("#3f4a4d"),
            shore: hex("#5d6660"),
            stone: Stone { light: hex("#c4ad8c"), ..Stone::granite() },
            wood_lit: hex("#9c8065"),
            wood_shade: hex("#312b27"),
            ripple: 0.035,
        },
        Light {
            name: "noon",
            sun: Sun::deg(-145.0, 58.0),
            sky_top: hex("#6485ab"),
            sky_low: hex("#c8d2d5"),
            glow: hex("#e4e6de"),
            glow_amt: 0.0,
            sand_lit: hex("#d6caa6"),
            sand_shade: hex("#80806f"),
            water: hex("#3e5055"),
            shore: hex("#6c7a74"),
            stone: Stone::granite(),
            wood_lit: hex("#a08a70"),
            wood_shade: hex("#3a332d"),
            ripple: 0.035,
        },
        Light {
            name: "dawn",
            sun: Sun::deg(18.0, -4.0),
            sky_top: hex("#56627b"),
            sky_low: hex("#b9a7a2"),
            glow: hex("#efcb95"),
            glow_amt: 0.9,
            sand_lit: hex("#9a8672"),
            sand_shade: hex("#5b5654"),
            water: hex("#2f3439"),
            shore: hex("#3a3b40"),
            stone: Stone {
                light: hex("#8f7a68"),
                half: hex("#6a605a"),
                shadow: hex("#45444b"),
                core: hex("#2c2b2f"),
                bounce: hex("#6b5a4c"),
                crevice: hex("#1a191b"),
            },
            wood_lit: hex("#7d6552"),
            wood_shade: hex("#27242a"),
            ripple: 0.012,
        },
    ]
}

/// The shore: land sloping gently down into the sea along a wavering line
/// 10–13 m out, coming in on the right where the boulder lies at the edge.
fn shore_z(x: f32) -> f32 {
    12.5 + 1.8 * (x * 0.42 + 0.3).sin() + 0.7 * (x * 1.7).sin() + 0.25 * (x * 4.1 + 1.0).sin() - 1.2 * smoothstep(1.0, 4.0, x)
}

fn ground(x: f32, z: f32) -> f32 {
    let s = shore_z(x);
    let beach = 0.05 + 0.03 * (x * 1.3).sin() * (z * 0.9).cos() + 0.012 * (s - z).max(0.0);
    let bed = -0.7;
    let g = beach + (bed - beach) * smoothstep(s - 1.0, s + 2.5, z);
    // a tide pool in the sand in front of the woman
    let (u, v) = ((x + 0.1) / 1.5, (z - 8.4) / 0.6);
    g - 0.12 * (-(u * u + v * v)).exp()
}

struct Scene {
    world: World,
    boulder: BodyId,
    poles: [BodyId; 2],
    woman: BodyId,
    path: Vec<(f32, f32)>,
}

fn build(r: [f32; 4], sun: Sun, ripple: f32) -> Scene {
    let horizon = r[1] + r[3] * 0.36;
    let mut world = World::new(r, horizon, 1.6)
        .fov(r[2], 55.0)
        .ground(ground)
        .water(Water::new(0.0).ripple(ripple, 1.4, 0.3, 7))
        .sun(sun)
        .backdrop(700.0)
        .visibility(1800.0);
    // a granite erratic on the beach, sunk a hand deep: a rounded block
    // fused with a flatter cap, turned, broken, weathered
    let s = world.spot_bed(3.1, 9.4);
    let c = s.p(0.0, 0.4, 0.0);
    let rock = Sdf::block(c, s.size(1.35, 1.0, 1.1), s.m(0.36))
        .union(Sdf::ellipsoid(s.p(-0.15, 0.8, 0.05), s.size(0.55, 0.25, 0.45)), s.m(0.22))
        .turn(c, 0.6, 0.06, -0.08)
        .rough(s.m(0.07), s.m(0.7), 3, false)
        .cut(s.p(0.0, 0.95, 0.0), [-0.25, -1.0, 0.2], 10, s.m(0.04))
        .cut(s.p(0.5, 0.45, 0.0), [1.0, -0.2, 0.4], 11, s.m(0.03))
        .rough(s.m(0.006), s.m(0.12), 4, true);
    let boulder = world.place(s, rock);
    // two poles standing in the shallows, leaning a little
    let mut poles = [0; 2];
    for (i, (x, z, h, lean)) in [(-4.2f32, 21.0f32, 3.4f32, -0.04f32), (-2.3, 24.5, 3.0, 0.05)].into_iter().enumerate() {
        let s = world.spot_bed(x, z);
        let depth = -s.at[1];
        let pole = Sdf::block(s.p(0.0, (h + depth) * 0.5, 0.0), s.size(0.13, h + depth, 0.13), s.m(0.05)).turn(s.p(0.0, 0.0, 0.0), 0.0, 0.0, lean);
        poles[i] = world.place(s, pole);
    }
    // the woman at the water's edge: a proxy (she is written with gestures),
    // so she still casts her shadow, stands in her contact and shows in the water
    let s = world.spot_at(0.0, 9.6);
    let fig = Sdf::ellipsoid(s.p(0.0, 0.45, 0.0), s.size(0.24, 0.46, 0.22))
        .union(Sdf::ellipsoid(s.p(0.0, 1.1, 0.0), s.size(0.2, 0.36, 0.14)), s.m(0.08))
        .union(Sdf::ellipsoid(s.p(0.0, 1.57, 0.0), s.size(0.1, 0.13, 0.11)), s.m(0.03));
    let woman = world.proxy(s, fig);
    let path = vec![(-2.6, 4.5), (-2.3, 6.5), (-1.6, 8.6), (-0.4, 9.7)];
    Scene { world, boulder, poles, woman, path }
}

fn sky_col(w: &World, l: &Light, x: f32, y: f32) -> Rgb {
    // by angle above the horizon, then the glow over the sun's place
    let a = ((w.horizon - y) / w.focal).atan().max(0.0);
    let base = mix(l.sky_low, l.sky_top, smoothstep(0.0, 0.45, a).powf(0.7), Mix::Light);
    let Some((gx, _)) = w.sun_canvas() else { return base };
    let dx = (x - gx) / w.focal;
    let g = l.glow_amt * (-(dx * dx) / 0.12).exp() * (-a / 0.12).exp();
    mix(base, l.glow, g.min(1.0), Mix::Light)
}

/// The far low shore across the water, a painted backdrop: its top edge.
fn far_top(w: &World, hills: &Fbm, x: f32) -> f32 {
    let u = x / w.focal;
    w.horizon - w.focal * (4.0 + 14.0 * hills.get01(u * 3.0, 0.3).powf(1.5) + 2.0 * hills.get01(u * 25.0, 3.1)) / w.backdrop
}

fn panel_mask(c: &Canvas, r: [f32; 4]) -> Mask {
    Mask::from_fn(c.frame(), move |x, y| if x >= r[0] && y >= r[1] && x < r[0] + r[2] && y < r[1] + r[3] { 1.0 } else { 0.0 })
}

/// The engine's fields unpainted (a diagnostic): value of the light on
/// ground and solids, cast shadow in blue, contact in red, reflections green.
fn masks(c: &mut Canvas, r: [f32; 4], l: &Light) {
    let sc = Scene::new_in(r, l);
    let w = &sc.world;
    let view = w.view(c.frame());
    let refl = view.reflections(&[]);
    let seam = view.contact(0.22);
    let ff = w.form_of(c.frame(), &[sc.woman]);
    c.apply(|x, y, p| {
        if !w.sees(x, y) {
            return p;
        }
        let pt = view.at(x, y);
        let v = match pt.what {
            paint::scene::What::Sky => [0.8, 0.85, 0.9],
            _ => {
                let v = pt.shade.value * 0.8 + 0.1;
                let v = match ff.sample(x, y) {
                    Some(q) => q.shade.value * 0.8 + 0.1,
                    None => v,
                };
                [v, v, v]
            }
        };
        let cast = if w.sun.up() { view.cast(x, y) } else { 0.0 };
        let (rr, ct) = (refl.sample(x, y), seam.sample(x, y));
        [v[0] * (1.0 - 0.6 * cast) + 0.5 * ct, v[1] * (1.0 - 0.6 * cast) * (1.0 - 0.5 * ct) + 0.3 * rr, v[2] * (1.0 - 0.2 * cast) * (1.0 - 0.5 * ct)]
    });
}

fn paint_panel(c: &mut Canvas, st: &Style, r: [f32; 4], l: &Light, seed: u64) {
    let f = c.frame();
    let pal = &st.palette;
    let sc = Scene::new_in(r, l);
    let w = &sc.world;
    let view = w.view(f);
    let k = 1.0;
    let hills = Fbm::new(seed as u32 + 5, 4, 1.0);

    // 1. the sky, level strokes, fused
    let sky = view.sky().mul(&panel_mask(c, r));
    let hd = st
        .broad()
        .color(|x, y| sky_col(w, l, x, y))
        .angle(|_, _| 0.0)
        .angle_jitter(0.05)
        .length(40.0 * k, 120.0 * k)
        .coverage(4.0)
        .medium(0.3)
        .clip(true)
        .threshold(0.4);
    c.work(&sky, &hd, seed);
    if let Some(b) = st.blend() {
        c.work(&sky, &b.clip(true).angle(|_, _| 0.0), seed + 1);
    }
    c.dry();

    // 2. the far shore, lost in the air
    let top = |x: f32| far_top(w, &hills, x);
    let far = Mask::from_fn(f, |x, y| if w.sees(x, y) && y > top(x) && y < w.horizon + 0.6 { 1.0 } else { 0.0 }).blur(0.8);
    let far_col = |x: f32, _y: f32| mix(l.shore, sky_col(w, l, x, w.horizon - 2.0), 0.35, Mix::Light);
    let hd = st.body().color(far_col).angle(|_, _| 0.0).length(15.0, 45.0).coverage(3.0).clip(true).threshold(0.3);
    c.work(&far, &hd, seed + 2);
    c.dry();

    // 3. the water: what it mirrors (sky, far shore) as strongly as the
    //    angle lets it, over the water's own dark body; level strokes
    let water = view.water();
    let mirrored = |x: f32, y: f32| -> Rgb {
        let Some(m) = view.mirror(x, y) else { return l.water };
        let (sx, sy) = m.src;
        let seen = if m.body.is_some() {
            sky_col(w, l, sx, w.horizon - 20.0)
        } else if sy > top(sx) && sy < w.horizon + 0.6 {
            far_col(sx, sy)
        } else {
            sky_col(w, l, sx, sy)
        };
        mix(l.water, seen, 0.3 + 0.7 * m.fresnel.sqrt(), Mix::Pigment)
    };
    let hd = st.body().color(mirrored).angle(|_, _| 0.0).angle_jitter(0.03).length(20.0, 70.0).coverage(3.4).clip(true).threshold(0.3);
    c.work(&water, &hd, seed + 3);
    c.dry();

    // 4. the sand: lit and shaded by the one sun, the cast shadows laid in
    //    with it (the world traced them); wet and darker at the water's edge
    let land = view.land();
    let grain = Fbm::new(seed as u32 + 9, 3, 25.0);
    let sand = |x: f32, y: f32| -> Rgb {
        let p = view.at(x, y);
        let lit = if w.sun.up() { smoothstep(0.0, 0.12, p.shade.turn) } else { 0.0 };
        let base = mix(l.sand_shade, l.sand_lit, lit, Mix::Pigment);
        let wet = 1.0 - smoothstep(0.02, 0.1, w.ground_at(p.at[0], p.at[2]));
        let base = mix(base, mix(base, l.water, 0.55, Mix::Pigment), wet, Mix::Pigment);
        let base = mix(base, mix(base, hex("#5a4d3e"), 0.4, Mix::Pigment), 0.5 * grain.get01(x, y * 3.0), Mix::Pigment);
        mix(base, sky_col(w, l, x, w.horizon - 1.0), w.aerial(p.dist), Mix::Light)
    };
    let hd = st.body().color(sand).angle(|_, _| 0.0).angle_jitter(0.12).length(12.0, 40.0).coverage(3.4).clip(true).threshold(0.3);
    c.work(&land, &hd, seed + 4);
    // the trodden path, narrowing as it runs back, strokes along it
    let path = Mask::from_shape(f, w.ribbon(&sc.path, |t| 1.1 - 0.4 * t)).blur(1.2).mul(&land);
    let line = w.line(&sc.path);
    let along = |x: f32, y: f32| {
        let i = line.iter().enumerate().min_by(|a, b| ((a.1.0 - x).powi(2) + (a.1.1 - y).powi(2)).total_cmp(&((b.1.0 - x).powi(2) + (b.1.1 - y).powi(2)))).map_or(0, |v| v.0);
        let (a, b) = (line[i.saturating_sub(1)], line[(i + 1).min(line.len() - 1)]);
        (b.1 - a.1).atan2(b.0 - a.0)
    };
    let trod = |x: f32, y: f32| mix(sand(x, y), hex("#6b5b49"), 0.3, Mix::Pigment);
    let hd = st.body().color(trod).angle(along).angle_jitter(0.1).length(8.0, 22.0).coverage(2.4).clip(true).threshold(0.35);
    c.work(&path, &hd, seed + 5);
    c.dry();
    // the cast shadows, as their own pass: the sand as the sky alone lights
    // it, stroked the way each shadow runs away from the one sun
    if w.sun.up() {
        // on the sand only: clear water shows a shadow barely at all
        let sh = view.shadows().map(|v| smoothstep(0.15, 0.6, v)).mul(&land);
        let dark = |x: f32, y: f32| {
            let p = view.at(x, y);
            let wet = 1.0 - smoothstep(0.02, 0.1, w.ground_at(p.at[0], p.at[2]));
            let base = if p.what == paint::scene::What::Water { mix(l.water, l.sky_top, 0.25, Mix::Pigment) } else { l.sand_shade };
            mix(base, mix(base, l.water, 0.4, Mix::Pigment), wet, Mix::Pigment)
        };
        let hd = st.body().color(dark).angle(|x, y| w.shadow_angle(x, y).unwrap_or(0.0)).angle_jitter(0.06).length(6.0, 20.0).coverage(3.0).clip(true).threshold(0.35);
        c.work(&sh, &hd, seed + 7);
        c.dry();
    }

    // 5. the boulder, plane by plane, lit by the world's light
    let stone = l.stone;
    let bpart = view.part(sc.boulder);
    let rock_col = |_: f32, _: f32, s: &Sample| mix(stone.at(&s.shade), sky_col(w, l, 500.0, w.horizon - 1.0), w.aerial(s.dist), Mix::Light);
    let bs = w.bodies[sc.boulder].spot;
    rocks::paint_solid(c, st, &view.form, &rocks::Solid { parts: vec![bpart], color: &rock_col, soft: &|_| 0.8, scale: bs.m(1.5) / 300.0, accents: 0.8, seed: seed + 6 });

    // 6. the poles: dark wood, then the side the sun (or the glow) finds
    for (i, &p) in sc.poles.iter().enumerate() {
        let part = view.part(p);
        let s = w.bodies[p].spot;
        let sil = view.form.silhouette(&[part], |_| 0.25);
        let wood = |x: f32, y: f32| match view.form.sample(x, y) {
            Some(q) => mix(l.wood_shade, l.wood_lit, q.shade.direct.powf(0.6) + 0.5 * q.shade.bounce, Mix::Pigment),
            None => l.wood_shade,
        };
        let tool = Tool::round_sable(s.m(0.11));
        let hd = paint::Handling::new(tool.clone()).mixed(pal, 0.2).color(wood).angle(|_, _| std::f32::consts::FRAC_PI_2).length(s.m(0.6), s.m(1.4)).coverage(3.0).clip(true).threshold(0.2);
        c.work(&sil, &hd, seed + 10 + i as u64);
        let lit = view.form.mask(|q| if q.part == part { smoothstep(0.08, 0.3, q.shade.direct) } else { 0.0 }).mul(&sil);
        let hd = paint::Handling::new(Tool::round_sable(s.m(0.05))).mixed(pal, 0.15).color(wood).angle(|_, _| std::f32::consts::FRAC_PI_2).length(s.m(0.4), s.m(1.0)).coverage(2.0).clip(true).threshold(0.3);
        c.work(&lit, &hd, seed + 20 + i as u64);
    }
    c.dry();

    // 7. what the water mirrors of the poles, the stone and the woman:
    //    their own colors as the one sun lights their undersides, darker,
    //    broken by the ripples
    let refl = view.reflections(&[]);
    let mcol = |x: f32, y: f32| -> Rgb {
        let Some(m) = view.mirror(x, y) else { return l.water };
        let seen = match m.body {
            Some(b) if b == sc.boulder => stone.at(&m.shade),
            Some(b) if b == sc.woman => hex("#2a2521"),
            Some(_) => mix(l.wood_shade, l.wood_lit, m.shade.direct.powf(0.6), Mix::Pigment),
            None => sky_col(w, l, m.src.0, m.src.1),
        };
        mix(l.water, seen, 0.45 + 0.4 * m.fresnel.sqrt(), Mix::Pigment)
    };
    let ang = if l.ripple > 0.02 { 0.0 } else { std::f32::consts::FRAC_PI_2 };
    let hd = paint::Handling::new(Tool::round_sable(2.2)).mixed(pal, 0.25).color(mcol).angle(move |_, _| ang).angle_jitter(0.05).length(3.0, 9.0).coverage(3.0).clip(true).threshold(0.3);
    c.work(&refl, &hd, seed + 30);
    c.dry();

    // 8. the woman, written with gestures at her size for that spot, and a
    //    touch of light on whatever side of her the one sun finds
    let ws = w.bodies[sc.woman].spot;
    let gown = Gown {
        dress: pal.paint(hex("#2b2622"), 0.12),
        shawl: Some(pal.only(&["red earth", "raw umber", "bone black"]).paint(hex("#43292a"), 0.12)),
        hair: pal.paint(hex("#2c231c"), 0.1),
        skin: pal.paint(hex("#6a5647"), 0.1),
        rim: None,
    };
    figures::woman(c, (ws.x, ws.y), ws.m(1.7), &gown, WomanPose::Standing, seed + 40);
    let ff = w.form_of(f, &[sc.woman]);
    // dark cloth in the light is a lighter dark, not a cream: the lit side
    // of the gown, then a thin edge where the light grazes her outline
    let sil = ff.silhouette(&[1], |_| 0.3).erode(ws.m(0.015));
    let key = if w.sun.up() { l.sand_lit } else { l.glow };
    let lit = ff.mask(|q| smoothstep(0.1, 0.4, q.shade.direct)).mul(&sil);
    let cloth = mix(hex("#2b2622"), key, 0.1, Mix::Pigment);
    let hd = paint::Handling::new(Tool::round_sable(ws.m(0.05))).mixed(pal, 0.2).color(move |_, _| cloth).angle(|_, _| std::f32::consts::FRAC_PI_2).length(ws.m(0.15), ws.m(0.4)).coverage(1.4).clip(true).threshold(0.4);
    c.work(&lit, &hd, seed + 41);
    let edge = lit.clone().subtract(&sil.erode(ws.m(0.03)));
    let rim = mix(hex("#2b2622"), key, 0.4, Mix::Pigment);
    let hd = paint::Handling::new(Tool::round_sable(ws.m(0.02))).mixed(pal, 0.15).color(move |_, _| rim).angle(|_, _| std::f32::consts::FRAC_PI_2).length(ws.m(0.08), ws.m(0.25)).coverage(1.2).clip(true).threshold(0.4);
    c.work(&edge, &hd, seed + 42);
    c.dry();

    // 9. the seams that ground them: a transparent umber glaze where the
    //    stone, the poles and her feet meet the sand and the water
    let seam = view.contact(0.22).map(|v| if v < 0.04 { 0.0 } else { v });
    let umber = pal.only(&["raw umber", "bone black"]).mix(hex("#2e2924")).paint(0.85).pigment();
    let amt = if w.sun.up() { 1.1 } else { 1.6 };
    c.glaze(&umber, Some(&seam), move |_, _| amt);
    c.dry();
}

impl Scene {
    fn new_in(r: [f32; 4], l: &Light) -> Scene {
        build(r, l.sun, l.ripple)
    }
}

#[allow(dead_code)]
fn outline(v: &View, s: Shape) -> Mask {
    Mask::from_shape(v.f, s)
}

fn main() {
    let o = paintings::run::Run::new("study_scene");
    let st = Style::friedrich();
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let diag = std::env::args().any(|a| a == "--masks");
    if std::env::args().any(|a| a == "--probe") {
        let l = &lights()[0];
        let sc = Scene::new_in([0.0, 0.0, 1000.0, PANEL_H], l);
        let w = &sc.world;
        let v = w.view(c.frame());
        let s = w.bodies[sc.boulder].spot;
        for dy in [0.2f32, 0.5, 0.8] {
            for dx in [-0.6f32, -0.3, 0.0, 0.3, 0.6] {
                let (x, y) = (s.x + s.m(dx), s.y - s.m(dy));
                if let Some(q) = v.form.sample(x, y) {
                    let at = s.world([x, y, q.z]);
                    eprintln!("dx {dx} dy {dy}: n {:?} turn {:.2} cast {:.2} direct {:.2} at {:?} occl {:.2}", q.n, q.shade.turn, q.shade.cast, q.shade.direct, at, w.occlusion(at, paint::scene::to_world(q.n), 0.22, true));
                }
            }
        }
        return;
    }
    let args: Vec<String> = std::env::args().collect();
    let only = args.iter().position(|a| a == "--only").and_then(|i| args.get(i + 1)).cloned();
    for (i, l) in lights().iter().enumerate() {
        if only.as_deref().is_some_and(|n| n != l.name) {
            continue;
        }
        let r = [0.0, i as f32 * (PANEL_H + GAP), 1000.0, PANEL_H];
        if o.stage(l.name, &mut c, &mut ()) {
            if diag {
                masks(&mut c, r, l);
                continue;
            }
            paint_panel(&mut c, &st, r, l, 1000 * (i as u64 + 1));
        }
    }
    o.end(&mut c, &mut ());
    c.relief(0.25, 0.02);
    o.save(&mut c);
}
