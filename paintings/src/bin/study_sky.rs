//! The air: one landscape (a sea horizon with ranges running out into it,
//! seen from a cliff top) under three skies, and a panel of ranges built
//! the old way beside the new. Everything is painted with brushes and
//! stipple by this study; the engine (`atmos`) only says how the one sun
//! lights the sky, the clouds and the air in front of the ranges.
//!
//!   1. morning: the sun low on the left, a cloud bank on the horizon, a
//!      few heaps, uneven haze and a haze layer
//!   2. overcast with a break: a stratus deck, the sun high on the right
//!      behind it, one break with the blue and a lit edge
//!   3. twilight after sunset: the sun 3.5° down ahead right; the Earth's
//!      shadow and the Belt of Venus opposite, the afterglow over the sun's
//!      place, high clouds lit from below
//!   4. ranges: left, the old way (even spacing in depth, parallel to the
//!      picture, similar peaks); right, `atmos::Ranges` (uneven spacing,
//!      oblique runs, peaks, domes, plateaus, saddles and cliffs, ranges
//!      that end, stratified haze and valley mist)
//!
//!   cargo paint study_sky
//!   cargo paint study_sky -- --fields          the engine's fields unpainted (a diagnostic)
//!   cargo paint study_sky -- --only morning     one panel
//!   cargo paint study_sky -- --full --crop 0,0,500,330

use paint::atmos::{Cloud, CloudField, Clouds, Haze, RangeLayer, Ranges, Sky, SkyField};
use paint::color::{Mix, mix};
use paint::form::Sample;
use paint::scene::{Sun, World};
use paint::{Canvas, Fbm, Form, Mask, Rgb, Stipple, Style, Tool, hex, smoothstep};

const PANEL_H: f32 = 300.0;
const GAP: f32 = 12.0;
const ASPECT: f32 = 1000.0 / (4.0 * PANEL_H + 3.0 * GAP);
/// Eye height (m): a cliff top over the sea.
const EYE: f32 = 30.0;

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Morning,
    Overcast,
    Twilight,
}

struct Panel {
    name: &'static str,
    kind: Kind,
    sun: Sun,
}

fn panels() -> [Panel; 3] {
    [
        Panel { name: "morning", kind: Kind::Morning, sun: Sun::deg(-19.0, 4.5) },
        Panel { name: "overcast", kind: Kind::Overcast, sun: Sun::deg(28.0, 24.0) },
        Panel { name: "twilight", kind: Kind::Twilight, sun: Sun::deg(14.0, -3.5) },
    ]
}

fn world(r: [f32; 4], sun: Sun) -> World {
    World::new(r, r[1] + r[3] * 0.6, EYE).fov(r[2], 58.0).sun(sun).visibility(30_000.0)
}

fn sky_for(kind: Kind, sun: Sun) -> Sky {
    match kind {
        Kind::Morning => Sky::new(sun).haze(2.6).uneven(0.6, 30_000.0, 11).layer(900.0, 500.0, 1.6, 0.8, 3),
        Kind::Overcast => Sky::new(sun).haze(3.5).uneven(0.4, 20_000.0, 5).overcast(0.25),
        Kind::Twilight => Sky::new(sun).haze(1.8).uneven(0.5, 40_000.0, 21),
    }
}

fn clouds_for(kind: Kind) -> Clouds {
    match kind {
        Kind::Morning => Clouds::new(vec![
            // a bank lying on the horizon, heavier on the right
            Cloud::bank(-2_000.0, 30_000.0, 38_000.0, 9_000.0, 700.0, 2_600.0, 4).density(0.035),
            // a few heaps, one near, uneven
            Cloud::cumulus(3_600.0, 10_000.0, 1_100.0, 2_600.0, 1_300.0, 8),
            Cloud::cumulus(-2_200.0, 16_000.0, 1_300.0, 1_600.0, 800.0, 9).soft(0.2),
            Cloud::stratus(3_800.0, 500.0, 0.34, 12).density(0.006).soft(0.5).wind(0.4, 5.0).reach(3_000.0, 40_000.0),
        ]),
        Kind::Overcast => Clouds::new(vec![Cloud::stratus(1_300.0, 700.0, 0.97, 31).density(0.02).soft(0.3).wind(-0.5, 1.6).heap(4_000.0).breaks(14_000.0, 0.22)]),
        Kind::Twilight => Clouds::new(vec![
            // high thin clouds, still in the sun
            Cloud::stratus(8_500.0, 700.0, 0.4, 41).density(0.004).soft(0.45).wind(0.2, 6.0).reach(8_000.0, 90_000.0),
            // low cloud already in the Earth's shadow
            Cloud::bank(-30_000.0, -6_000.0, 30_000.0, 8_000.0, 500.0, 1_500.0, 44).density(0.03),
        ]),
    }
}

fn haze_for(kind: Kind) -> Haze {
    match kind {
        Kind::Morning => Haze::new(28_000.0).height(900.0).mist(140.0, 6.0, 90.0, 7),
        Kind::Overcast => Haze::new(20_000.0).height(1200.0),
        Kind::Twilight => Haze::new(35_000.0).height(1000.0).mist(90.0, 4.0, 60.0, 9),
    }
}

/// The sea level's canvas y at distance z (m): a range's foot.
fn foot(w: &World, z: f32) -> f32 {
    w.horizon + w.focal * (w.eye + RangeLayer::curvature(z)) / z
}

/// The ranges' form: one Ridge per layer, nearer in front, lit by the sun.
fn range_form(c: &Canvas, w: &World, layers: &[RangeLayer]) -> (Form, Vec<u16>) {
    let mut form = Form::new(c.frame());
    let (x0, x1) = (w.view[0], w.view[0] + w.view[2]);
    let mut ids = vec![];
    for l in layers {
        let fy = foot(w, l.z_at(0.0)) + 2.0;
        let top = (0..=20).map(|i| l.crest(w, x0 + (x1 - x0) * i as f32 / 20.0)).fold(f32::MAX, f32::min);
        let ridge = l.ridge(w, x0, x1, (fy - top).max(4.0) + 4.0, 260.0).base(fy).z0(-(l.k as f32) * 5000.0);
        let d = l.z / 1000.0;
        ids.push(form.add_at(&ridge, &move |_, _, _| d));
    }
    let light = w.light();
    form.light(light);
    (form, ids)
}

/// The stone color of a range at a point (before the air): lit and shadow
/// families from the one sun, warm where the low sun finds it.
fn stone(kind: Kind, s: &Sample, sunlight: Rgb) -> Rgb {
    let (shadow, lit) = match kind {
        Kind::Morning => (hex("#3b4046"), hex("#8a7a64")),
        Kind::Overcast => (hex("#3d4441"), hex("#5d625b")),
        Kind::Twilight => (hex("#2a2c35"), hex("#3a3840")),
    };
    let warm = mix(lit, [lit[0] * (0.6 + 0.8 * sunlight[0]), lit[1] * (0.7 + 0.5 * sunlight[1]), lit[2] * (0.6 + 0.4 * sunlight[2])], 0.5, Mix::Light);
    let v = s.shade.value.clamp(0.0, 1.0);
    mix(shadow, warm, smoothstep(0.15, 0.75, v), Mix::Pigment)
}

// ----------------------------------------------------------- diagnostic

fn fields(c: &mut Canvas, r: [f32; 4], p: &Panel) {
    let w = world(r, p.sun);
    let sf = SkyField::new(sky_for(p.kind, p.sun), &w, 6.0);
    let cf = clouds_for(p.kind).field(&sf, &w, 2.0);
    let air = haze_for(p.kind);
    let layers = Ranges::new(4_000.0, 50_000.0, 5, 17).heights(220.0, 2600.0).build(&w);
    let (form, ids) = range_form(c, &w, &layers);
    let sl = sf.sky.sunlight();
    c.apply(|x, y, px| {
        if !w.sees(x, y) {
            return px;
        }
        if y < w.horizon {
            let mut col = cf.color(&sf, x, y);
            if let Some(s) = form.sample(x, y)
                && let Some(k) = ids.iter().position(|&i| i == s.part)
            {
                let l = &layers[k];
                if y < foot(&w, l.z_at(0.0)) {
                    col = mix(stone(p.kind, &s, sl), sf.airlight(x), l.haze(&w, &air, x, y), Mix::Light);
                }
            }
            col
        } else {
            let z = w.focal * w.eye / (y - w.horizon).max(1e-3);
            let fres = 0.02 + 0.98 * (1.0 - ((y - w.horizon) / w.focal).atan().sin()).powi(5);
            let sea = mix(hex("#1d2a30"), sf.at(x, y), fres, Mix::Light);
            mix(sea, sf.airlight(x), air.loss(w.eye, z, 0.0, 0.0), Mix::Light)
        }
    });
}

// -------------------------------------------------------------- painting

fn paint_sky(c: &mut Canvas, st: &Style, w: &World, sf: &SkyField, cf: &CloudField, r: [f32; 4], seed: u64) {
    let f = c.frame();
    let pal = &st.palette;
    let inside = move |x: f32, y: f32| (x.clamp(r[0] + 0.5, r[0] + r[2] - 0.5), y.clamp(r[1] + 0.5, r[1] + r[3] - 0.5));
    let hz = w.horizon;
    let sky = Mask::from_fn(f, |x, y| if w.sees(x, y) && y < hz + 1.0 { 1.0 } else { 0.0 });
    let col = |x: f32, y: f32| {
        let (x, y) = inside(x, y);
        cf.color(sf, x, y.min(hz - 0.5))
    };
    // lay-in: broad, loose, mostly level, fused
    let hd = st.broad().color(col).angle(|_, _| 0.0).angle_jitter(0.12).length(30.0, 90.0).coverage(3.6).medium(0.35).clip(true).threshold(0.3);
    c.work(&sky, &hd, seed);
    if let Some(b) = st.blend() {
        c.work(&sky, &b.clip(true).angle(|_, _| 0.0).angle_jitter(0.2), seed + 1);
    }
    c.dry();
    // stipple over it: the sky's structure again, in small touches
    let sp = Stipple::new(Tool::stippler(2.4)).mixed(pal, 0.45).color(col).coverage(|_, _| 1.6).pressure(0.45, 0.8).dips(18, 0.4, 0.5).cluster(0.3, None).clip(true);
    c.stipple(&sky, &sp, seed + 2);
    c.dry();
    // clouds: bodies stroked along their lying direction, the shadowed
    // bellies, then the lit edges in stiffer paint; soft edges fused
    let body = cf.mask(f, |p| smoothstep(0.25, 0.7, p.alpha)).mul(&sky);
    let cc = |x: f32, y: f32| {
        let (x, y) = inside(x, y);
        cf.color(sf, x, y.min(hz - 0.5))
    };
    let hd = st.body().color(cc).angle(|_, _| 0.0).angle_jitter(0.35).length(6.0, 22.0).coverage(2.4).pressure(0.5, 0.85).clip(true).threshold(0.25);
    c.work(&body, &hd, seed + 3);
    let lit = cf.mask(f, |p| smoothstep(0.4, 0.9, p.alpha) * smoothstep(0.35, 0.8, p.lit)).mul(&sky);
    let hd = paint::Handling::new(Tool::round_sable(2.6)).mixed(pal, 0.2).color(cc).angle(|_, _| -0.2).angle_jitter(0.6).length(2.5, 9.0).coverage(1.6).pressure(0.5, 0.9).clip(true).threshold(0.3);
    c.work(&lit, &hd, seed + 4);
    let thin = cf.mask(f, |p| smoothstep(0.02, 0.2, p.alpha) * (1.0 - smoothstep(0.4, 0.8, p.alpha))).mul(&sky);
    let sp = Stipple::new(Tool::stippler(1.8)).mixed(pal, 0.5).color(cc).coverage(|_, _| 1.4).pressure(0.4, 0.7).dips(16, 0.35, 0.6).clip(true);
    c.stipple(&thin, &sp, seed + 5);
    if let Some(b) = st.blend() {
        let soft = Mask::from_fn(f, |x, y| if y < hz { smoothstep(8.0, 30.0, cf.soft(x, y)) * smoothstep(0.02, 0.15, cf.alpha(x, y)) * (1.0 - smoothstep(0.85, 1.0, cf.alpha(x, y))) } else { 0.0 });
        c.work(&soft, &b.clip(true).angle(|_, _| 0.0).angle_jitter(0.4).length(10.0, 30.0), seed + 6);
    }
    c.dry();
}

fn paint_sea(c: &mut Canvas, st: &Style, w: &World, sf: &SkyField, air: &Haze, kind: Kind, r: [f32; 4], seed: u64) {
    let f = c.frame();
    let pal = &st.palette;
    let hz = w.horizon;
    let inside = move |x: f32, y: f32| (x.clamp(r[0] + 0.5, r[0] + r[2] - 0.5), y.clamp(r[1] + 0.5, r[1] + r[3] - 0.5));
    let deep = match kind {
        Kind::Morning => hex("#1f2b31"),
        Kind::Overcast => hex("#26302f"),
        Kind::Twilight => hex("#171b24"),
    };
    let ripples = Fbm::new(seed as u32 + 3, 4, 40.0);
    let sea = Mask::from_fn(f, |x, y| if w.sees(x, y) && y >= hz - 0.5 { 1.0 } else { 0.0 });
    let col = move |x: f32, y: f32| {
        let (x, y) = inside(x, y);
        let y = y.max(hz + 0.3);
        let dy = y - hz;
        let z = w.focal * w.eye / dy;
        // Fresnel (Schlick) at the grazing angle, broken by the ripples'
        // tilt: the far sea mirrors the sky, the near sea shows its own dark
        let a = (dy / w.focal).atan() * (1.0 + 0.8 * ripples.get(x * 0.3, dy * 4.0 + y * 0.2));
        let fres = 0.02 + 0.98 * (1.0 - a.sin().clamp(0.0, 1.0)).powi(5);
        let m = sf.at(x, hz + dy * (1.0 + 0.6 * ripples.get(x, dy * 3.0).abs()));
        let s = mix(deep, m, fres, Mix::Light);
        mix(s, sf.airlight(x), air.loss(w.eye, z, 0.0, 0.0), Mix::Light)
    };
    let hd = st.broad().color(col).angle(|_, _| 0.0).angle_jitter(0.03).length(30.0, 110.0).coverage(3.4).medium(0.35).clip(true).threshold(0.3);
    c.work(&sea, &hd, seed);
    if let Some(b) = st.blend() {
        c.work(&sea, &b.clip(true).angle(|_, _| 0.0).angle_jitter(0.03), seed + 2);
    }
    // far water: level stipple, so the horizon is made of touches, not a rule
    let far = Mask::from_fn(f, |x, y| if w.sees(x, y) && y >= hz - 1.5 { 1.0 - smoothstep(hz + 3.0, hz + 18.0, y) } else { 0.0 });
    let sp = Stipple::new(Tool::stippler(1.8)).mixed(pal, 0.5).color(col).coverage(|_, _| 1.6).pressure(0.4, 0.75).drag(1.5, Some(0.0)).dips(18, 0.35, 0.6).clip(true);
    c.stipple(&far, &sp, seed + 1);
    c.dry();
}

#[allow(clippy::too_many_arguments)]
fn paint_ranges(c: &mut Canvas, st: &Style, w: &World, sf: &SkyField, air: &Haze, kind: Kind, layers: &[RangeLayer], r: [f32; 4], seed: u64) {
    let pal = &st.palette;
    let (form, ids) = range_form(c, w, layers);
    let form = &form;
    let sl = sf.sky.sunlight();
    let inside = move |x: f32, y: f32| (x.clamp(r[0] + 0.5, r[0] + r[2] - 0.5), y.clamp(r[1] + 0.5, r[1] + r[3] - 0.5));
    for (k, l) in layers.iter().enumerate().rev() {
        let id = ids[k];
        let fy = foot(w, l.z_at(0.0));
        let region = form.silhouette(&[id], |_| 0.5 + 0.02 * l.z / 1000.0).mul_fn(|x, y| if w.sees(x, y) && y < fy + 0.5 { 1.0 } else { 0.0 });
        let col = move |x: f32, y: f32| {
            let (x, y) = inside(x, y);
            let s = form.sample(x, y).filter(|s| s.part == id).or_else(|| form.sample(x, y + 2.0).filter(|s| s.part == id));
            let base = s.map_or(sf.airlight(x), |s| stone(kind, &s, sl));
            mix(base, sf.airlight(x), l.haze(w, air, x, y.min(fy)), Mix::Light)
        };
        let col = &col;
        let col = move |x: f32, y: f32| col(x, y);
        let a = l.haze(w, air, 500.0, l.crest(w, 500.0));
        if a < 0.55 {
            // near: strokes down the fall lines, then the lit spurs
            let hd = st.body().color(col).angle(|x, y| form.fall(x, y)).angle_jitter(0.15).length(4.0, 16.0).coverage(2.8).pressure(0.5, 0.85).clip(true).threshold(0.2);
            c.work(&region, &hd, seed + 10 * k as u64);
            let lit = form.mask(|s| if s.part == id { s.shade.lit(0.12) } else { 0.0 }).mul(&region);
            let hd = paint::Handling::new(Tool::round_sable(2.2)).mixed(pal, 0.2).color(col).angle(|x, y| form.fall(x, y)).angle_jitter(0.12).length(3.0, 9.0).coverage(1.4).pressure(0.5, 0.85).clip(true).threshold(0.3);
            c.work(&lit, &hd, seed + 10 * k as u64 + 1);
        } else {
            // far: laid thin, then stippled (Friedrich's distant hills)
            let hd = st.body().color(col).angle(|_, _| 0.0).angle_jitter(0.2).length(8.0, 24.0).coverage(2.4).pressure(0.45, 0.7).clip(true).threshold(0.2);
            c.work(&region, &hd, seed + 10 * k as u64);
            let sp = Stipple::new(Tool::stippler(1.6)).mixed(pal, 0.45).color(col).coverage(|_, _| 1.5).pressure(0.4, 0.75).dips(16, 0.35, 0.6).clip(true);
            c.stipple(&region, &sp, seed + 10 * k as u64 + 1);
        }
        c.dry();
    }
}

fn paint_panel(c: &mut Canvas, st: &Style, r: [f32; 4], p: &Panel, seed: u64) {
    let w = world(r, p.sun);
    let sf = SkyField::new(sky_for(p.kind, p.sun), &w, 6.0);
    let sf = match p.kind {
        Kind::Twilight => sf.exposure(0.9),
        Kind::Morning => {
            let bal = sf.sky.sunlight();
            sf.balance(bal, 0.25)
        }
        Kind::Overcast => sf,
    };
    let cf = clouds_for(p.kind).field(&sf, &w, 2.0);
    let air = haze_for(p.kind);
    let layers = Ranges::new(4_000.0, 50_000.0, 5, 17).heights(220.0, 2600.0).build(&w);
    paint_sky(c, st, &w, &sf, &cf, r, seed);
    paint_sea(c, st, &w, &sf, &air, p.kind, r, seed + 100);
    paint_ranges(c, st, &w, &sf, &air, p.kind, &layers, r, seed + 200);
}

/// Ranges old and new, side by side, under the morning sky.
fn paint_compare(c: &mut Canvas, st: &Style, y0: f32, seed: u64) {
    let sun = Sun::deg(-35.0, 9.0);
    for (i, old) in [true, false].into_iter().enumerate() {
        let r = [i as f32 * 503.0, y0, 497.0, PANEL_H];
        let w = World::new(r, r[1] + r[3] * 0.72, EYE).fov(r[2], 40.0).sun(sun).visibility(30_000.0);
        let sf = SkyField::new(Sky::new(sun).haze(2.5).uneven(0.5, 30_000.0, 11), &w, 6.0);
        let cf = Clouds::new(vec![]).field(&sf, &w, 4.0);
        let air = Haze::new(60_000.0).height(900.0).mist(120.0, 5.0, 80.0, 7);
        let rs = Ranges::new(2_500.0, 45_000.0, 7, 23).heights(260.0, 2600.0);
        let layers = if old { rs.regular(&w) } else { rs.build(&w) };
        paint_sky(c, st, &w, &sf, &cf, r, seed + 1000 * i as u64);
        paint_sea(c, st, &w, &sf, &air, Kind::Morning, r, seed + 1000 * i as u64 + 100);
        paint_ranges(c, st, &w, &sf, &air, Kind::Morning, &layers, r, seed + 1000 * i as u64 + 200);
    }
}

fn main() {
    let o = paintings::run::Run::new("study_sky");
    let st = Style::friedrich();
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let diag = std::env::args().any(|a| a == "--fields");
    if std::env::args().any(|a| a == "--probe") {
        let w = world([0.0, 0.0, 1000.0, PANEL_H], panels()[0].sun);
        for l in Ranges::new(4_000.0, 50_000.0, 5, 17).heights(220.0, 2600.0).build(&w) {
            let ys: Vec<i32> = (0..11).map(|i| l.crest(&w, i as f32 * 100.0) as i32).collect();
            eprintln!("k {} z {:.0} skew {:.2} floor {:.0} masses {:?}\n   crest {:?} foot {:.0}", l.k, l.z, l.skew, l.floor, l.masses.iter().map(|m| (m.kind, m.x as i32, m.half as i32, m.h as i32)).collect::<Vec<_>>(), ys, foot(&w, l.z));
        }
        return;
    }
    let args: Vec<String> = std::env::args().collect();
    let only = args.iter().position(|a| a == "--only").and_then(|i| args.get(i + 1)).cloned();
    for (i, p) in panels().iter().enumerate() {
        if only.as_deref().is_some_and(|n| n != p.name) {
            continue;
        }
        let r = [0.0, i as f32 * (PANEL_H + GAP), 1000.0, PANEL_H];
        if o.stage(p.name, &mut c, &mut ()) {
            if diag {
                fields(&mut c, r, p);
                continue;
            }
            paint_panel(&mut c, &st, r, p, 1000 * (i as u64 + 1));
        }
    }
    if only.as_deref().is_none_or(|n| n == "ranges") && o.stage("ranges", &mut c, &mut ()) && !diag {
        paint_compare(&mut c, &st, 3.0 * (PANEL_H + GAP), 9000);
    }
    o.end(&mut c, &mut ());
    c.relief(0.25, 0.02);
    o.save(&mut c);
}
