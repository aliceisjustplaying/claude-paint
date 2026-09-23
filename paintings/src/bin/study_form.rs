//! Form and light: solids built from the engine's form primitives and
//! painted with brushes by the rock motifs in `paintings::rocks`.
//!
//!   top left: a granite boulder on the ground, lit from the upper left
//!   top right: a sandstone outcrop, stacked blocks seen a little from above
//!   bottom left: a Rügen chalk cliff with pinnacles, a farther cliff, the sea
//!   bottom right: three mountain ranges receding into haze, mist at the feet
//!
//!   cargo paint study_form                       the sheet at 1000px
//!   cargo paint study_form -- --only ranges      one motif over the whole canvas
//!   cargo paint study_form -- --grisaille        the light model's values, unpainted (a diagnostic)

use paint::color::{Mix, mix};
use paint::form::{Light, Sample, aerial};
use paint::{Canvas, Fbm, Form, Mask, Rgb, Style, hex, smoothstep};
use paintings::rocks::{self, Air, Rect, Solid, Stone};

const ASPECT: f32 = 1000.0 / 700.0;

fn sky_panel(c: &mut Canvas, st: &Style, r: Rect, top: Rgb, low: Rgb, seed: u64) {
    let m = Mask::from_fn(c.frame(), |x, y| if r.contains(x, y) { 1.0 } else { 0.0 });
    let k = r.w / 500.0;
    let hd = st
        .broad()
        .color(move |_, y| mix(top, low, smoothstep(r.y, r.y + r.h * 0.7, y), Mix::Light))
        .angle(|_, _| 0.0)
        .length(60.0 * k, 160.0 * k)
        .coverage(4.0)
        .medium(0.3)
        .clip(true)
        .threshold(0.5);
    c.work(&m, &hd, seed);
    if let Some(b) = st.blend() {
        c.work(&m, &b.clip(true).angle(|_, _| 0.0), seed + 1);
    }
    c.dry();
}

fn grass() -> Stone {
    Stone {
        light: hex("#8d8656"),
        half: hex("#6d6843"),
        shadow: hex("#454a37"),
        core: hex("#2d2f24"),
        bounce: hex("#56503a"),
        crevice: hex("#1f1f18"),
    }
}

fn grisaille(c: &mut Canvas, form: &Form, r: Rect) {
    c.apply(|x, y, p| {
        if !r.contains(x, y) {
            return p;
        }
        match form.sample(x, y) {
            Some(s) => {
                // value as a painter's value scale (perceptual), not light
                let v = (s.shade.value * 0.85 + 0.05).powf(2.2);
                mix([v; 3], [0.55, 0.58, 0.62], aerial(s.dist, 4.0) * 0.6, Mix::Linear)
            }
            None => [0.55, 0.58, 0.62],
        }
    });
}

/// Boulder on the ground.
fn boulder(c: &mut Canvas, st: &Style, r: Rect, gris: bool) {
    let mut form = Form::new(c.frame());
    let horizon = r.y + r.h * 0.5;
    let g = rocks::ground(&mut form, r, horizon, 11);
    let b = rocks::boulder(&mut form, r, horizon, 12);
    form.light(Light::new((-1.0, -0.75), 0.55).penumbra(0.05).ambient(0.18));
    if gris {
        return grisaille(c, &form, r);
    }
    sky_panel(c, st, r, hex("#9fa7ad"), hex("#d9d2bd"), 100);
    let k = r.w / 500.0;
    let air = Air { color: hex("#c9c6b8"), visibility: 4.0 };
    let tufts = Fbm::new(5, 3, 30.0 * k);
    let ground_col = move |x: f32, y: f32, s: &Sample| air.over(mix(grass().at(&s.shade), hex("#5b4b35"), 0.4 * tufts.get01(x, y * 2.0), Mix::Pigment), s.dist);
    rocks::paint_solid(c, st, &form, &Solid { parts: vec![g], color: &ground_col, soft: &|_| 0.5, scale: k, accents: 0.0, seed: 110 });
    let lichen = Fbm::new(7, 4, 25.0 * k);
    let stone = Stone::granite();
    let rock_col = move |x: f32, y: f32, s: &Sample| {
        let base = stone.at(&s.shade);
        // gray-green lichen on the planes that face the sky
        let up = smoothstep(-0.1, -0.6, s.n[1]) * smoothstep(0.45, 0.75, lichen.get01(x, y));
        mix(base, mix(base, hex("#7d8466"), 0.6, Mix::Pigment), up, Mix::Pigment)
    };
    rocks::paint_solid(c, st, &form, &Solid { parts: vec![b], color: &rock_col, soft: &|s| 0.3 + 1.5 * (1.0 - s.n[2]).max(0.0), scale: k, accents: 1.0, seed: 120 });
}

/// Sandstone outcrop.
fn outcrop(c: &mut Canvas, st: &Style, r: Rect, gris: bool) {
    let mut form = Form::new(c.frame());
    let horizon = r.y + r.h * 0.62;
    let g = rocks::ground(&mut form, r, horizon, 21);
    let parts = rocks::outcrop(&mut form, r, 22);
    form.light(Light::new((-1.0, -0.8), 0.5).penumbra(0.04).ambient(0.2));
    if gris {
        return grisaille(c, &form, r);
    }
    sky_panel(c, st, r, hex("#8d9aa8"), hex("#dcd3bd"), 200);
    let k = r.w / 500.0;
    let air = Air { color: hex("#cbc6b6"), visibility: 4.0 };
    let tufts = Fbm::new(25, 3, 30.0 * k);
    let ground_col = move |x: f32, y: f32, s: &Sample| air.over(mix(grass().at(&s.shade), hex("#5b4b35"), 0.4 * tufts.get01(x, y * 2.0), Mix::Pigment), s.dist);
    rocks::paint_solid(c, st, &form, &Solid { parts: vec![g], color: &ground_col, soft: &|_| 0.5, scale: k, accents: 0.0, seed: 210 });
    let stain = Fbm::new(27, 4, 40.0 * k);
    let stone = Stone::sandstone();
    let rock_col = move |x: f32, y: f32, s: &Sample| {
        let base = stone.at(&s.shade);
        // iron staining in streaks down the faces, darker weathering crust
        let streak = stain.get01(x * 3.0, y * 0.4);
        let base = mix(base, mix(base, hex("#6b4a30"), 0.5, Mix::Pigment), smoothstep(0.55, 0.8, streak), Mix::Pigment);
        mix(base, mix(base, hex("#4a4640"), 0.5, Mix::Pigment), 0.5 * smoothstep(0.5, 0.8, stain.get01(x, y)), Mix::Pigment)
    };
    rocks::paint_solid(c, st, &form, &Solid { parts, color: &rock_col, soft: &|_| 0.3, scale: k, accents: 1.2, seed: 220 });
}

/// Chalk cliff at the sea.
fn cliff(c: &mut Canvas, st: &Style, r: Rect, gris: bool) {
    let mut form = Form::new(c.frame());
    let (near, far) = rocks::chalk_cliff(&mut form, r, 31);
    form.light(Light::new((-0.9, -0.55), 0.75).penumbra(0.05).ambient(0.25).across_parts(false));
    if gris {
        return grisaille(c, &form, r);
    }
    let k = r.w / 500.0;
    sky_panel(c, st, r, hex("#8ea3b5"), hex("#e3dcc8"), 300);
    // the sea below the horizon
    let horizon = r.y + r.h * 0.52;
    let sea = Mask::from_fn(c.frame(), |x, y| if r.contains(x, y) && y > horizon { 1.0 } else { 0.0 });
    let hd = st
        .body()
        .color(move |_, y| mix(hex("#8d9c9e"), hex("#4f6468"), smoothstep(horizon, r.y + r.h, y), Mix::Pigment))
        .angle(|_, _| 0.0)
        .angle_jitter(0.04)
        .length(40.0 * k, 110.0 * k)
        .clip(true)
        .threshold(0.5);
    c.work(&sea, &hd, 310);
    c.dry();
    let air = Air { color: hex("#d4d2c8"), visibility: 3.0 };
    let stone = Stone::chalk();
    let tops = Fbm::new(33, 3, 12.0 * k);
    let form_ref = &form;
    let chalk_col = move |x: f32, y: f32, s: &Sample| {
        let base = stone.at(&s.shade);
        // beech wood and turf on the brink, clinging a little way down
        let below = y - top_of(form_ref, x, y, s.part);
        let green = 1.0 - smoothstep(3.0 * k, (7.0 + 9.0 * tops.get01(x, y)) * k, below);
        let wood = mix(hex("#5d6a3a"), hex("#2f3a24"), 1.0 - s.shade.lit(0.2), Mix::Pigment);
        // the scree at the foot: grayer, warmer
        let scree = smoothstep(r.y + r.h * 0.78, r.y + r.h * 0.9, y);
        let base = mix(base, mix(base, hex("#a39a88"), 0.6, Mix::Pigment), scree, Mix::Pigment);
        air.over(mix(base, wood, green, Mix::Pigment), s.dist)
    };
    rocks::paint_solid(c, st, &form, &Solid { parts: vec![far], color: &chalk_col, soft: &|_| 1.5 * k, scale: k * 0.8, accents: 0.4, seed: 330 });
    rocks::paint_solid(c, st, &form, &Solid { parts: vec![near], color: &chalk_col, soft: &|_| 0.4 * k, scale: k, accents: 1.0, seed: 320 });
}

/// The top of the solid above a point (for the green cap).
fn top_of(form: &Form, x: f32, y: f32, part: u16) -> f32 {
    let mut yy = y;
    while yy > 0.0 && form.part(x, yy - 1.0) == part {
        yy -= 1.0;
    }
    yy
}

/// Three ranges receding.
fn ranges(c: &mut Canvas, st: &Style, r: Rect, gris: bool) {
    let mut form = Form::new(c.frame());
    let ids = rocks::ranges(&mut form, r, 41);
    form.light(Light::new((-1.0, -0.6), 0.5).penumbra(0.06).ambient(0.28).across_parts(false));
    if gris {
        return grisaille(c, &form, r);
    }
    let k = r.w / 500.0;
    sky_panel(c, st, r, hex("#8c9bb0"), hex("#e6d9bf"), 400);
    let air = Air { color: hex("#cfc9c0"), visibility: 6.0 };
    let stone = Stone::mountain();
    let woods = Fbm::new(45, 3, 30.0 * k);
    let col = move |x: f32, y: f32, s: &Sample| {
        let base = stone.at(&s.shade);
        // dark forest over the lower slopes of the nearer ranges
        let forest = smoothstep(0.45, 0.6, woods.get01(x, y)) * smoothstep(r.y + r.h * 0.55, r.y + r.h * 0.75, y);
        mix(base, mix(hex("#3b4434"), hex("#252b24"), 1.0 - s.shade.lit(0.2), Mix::Pigment), forest * 0.8, Mix::Pigment)
    };
    // valley mist lying in front of each range's foot, where the next
    // range rises (the range's own base line is hidden behind it)
    let feet = [r.y + r.h * 1.05, r.y + r.h * 0.64, r.y + r.h * 0.5];
    let fog = Fbm::new(47, 3, 60.0 * k);
    let mist = move |x: f32, y: f32, i: usize| {
        let foot = feet[i.min(2)];
        0.85 * smoothstep(foot - r.h * 0.17, foot, y + fog.get(x, y) * r.h * 0.05)
    };
    rocks::paint_ranges(c, st, &form, &ids, &air, &col, &mist, k, 410);
}

fn main() {
    let o = paintings::run::Run::new("study_form");
    let args: Vec<String> = std::env::args().collect();
    let only = args.iter().position(|a| a == "--only").and_then(|i| args.get(i + 1)).cloned();
    let gris = args.iter().any(|a| a == "--grisaille");
    let st = Style::friedrich();
    let mut c = st.prepare(o.width, ASPECT, o.seed);
    let (w, h) = (c.width(), c.height());
    let g = 8.0;
    let panels = [
        ("boulder", Rect { x: 0.0, y: 0.0, w: w * 0.5 - g * 0.5, h: h * 0.5 - g * 0.5 }),
        ("outcrop", Rect { x: w * 0.5 + g * 0.5, y: 0.0, w: w * 0.5 - g * 0.5, h: h * 0.5 - g * 0.5 }),
        ("cliff", Rect { x: 0.0, y: h * 0.5 + g * 0.5, w: w * 0.5 - g * 0.5, h: h * 0.5 - g * 0.5 }),
        ("ranges", Rect { x: w * 0.5 + g * 0.5, y: h * 0.5 + g * 0.5, w: w * 0.5 - g * 0.5, h: h * 0.5 - g * 0.5 }),
    ];
    let whole = Rect { x: 0.0, y: 0.0, w, h };
    for (name, r) in panels {
        if only.as_deref().is_some_and(|n| n != name) {
            continue;
        }
        let r = if only.is_some() { whole } else { r };
        match name {
            "boulder" => boulder(&mut c, &st, r, gris),
            "outcrop" => outcrop(&mut c, &st, r, gris),
            "cliff" => cliff(&mut c, &st, r, gris),
            _ => ranges(&mut c, &st, r, gris),
        }
        if o.stage(&mut c, name) {
            return;
        }
    }
    c.relief(0.25, 0.02);
    o.save(&mut c);
}
