//! "Wayside Cross in the Snow at Evening" — an original winter picture in the
//! manner of Caspar David Friedrich (written from notes/research, never from
//! images).
//!
//! A snow-covered rise fills the foreground; on its crest, on the picture's
//! central axis, stands a weathered wooden cross between two young spruces.
//! Below it, with his back to us, a traveler in a cape leans on his stick
//! and looks up. On the left a dead oak with broken limbs, crows in it. Beyond
//! the rise the land drops away into evening fog: rows of spruces rise out of
//! it, and far off the ghost of a church tower. The sky after sunset: grey
//! blue above, mauve, then a pale lemon glow over the fog.
//!
//! Order of work (CATS p.127, NG p.56): bought ground, graphite underdrawing,
//! a very thin underpainting, sky, distance to foreground, figures last, then
//! the grasses over the finished snow and the finish.

use paint::color::{Mix, gradient, mix};
use oak::Oak;
use paint::{Canvas, Gesture, Hand, Handling, Held, Mark, Mask, Orient, Paint, Rgb, Rng, Shape, Style, Tool, hex, smoothstep};
use paintings::run::{Finish, Run};

const ASPECT: f32 = 1.45;
/// Far edge of the snow plain, lost in the fog.
const HORIZON: f32 = 455.0;
/// The cross stands on the central axis.
const AXIS: f32 = 500.0;

/// The crest of the foreground rise (y in units for x).
fn crest(x: f32) -> f32 {
    600.0 - 96.0 * (-((x - AXIS) / 330.0).powi(2)).exp() - 5.0 * (x * 0.013 + 1.0).sin() - 2.5 * (x * 0.047).sin()
}

fn crest_slope(x: f32) -> f32 {
    (crest(x + 1.0) - crest(x - 1.0)) * 0.5
}

/// The evening sky: grey blue at the top, mauve, then a pale lemon glow over
/// the fog, strongest a little right of center where the sun went down.
fn sky(x: f32, y: f32) -> Rgb {
    let t = (y / HORIZON).clamp(0.0, 1.0);
    let base = gradient(
        &[(0.0, hex("#868ea4")), (0.25, hex("#9a9fb2")), (0.5, hex("#bba9b8")), (0.7, hex("#dcc3b8")), (0.86, hex("#ecd4b8")), (1.0, hex("#f0dcc0"))],
        t,
        Mix::Light,
    );
    let g = (-((x - 610.0) / 280.0).powi(2)).exp() * smoothstep(0.5, 1.0, t);
    mix(base, hex("#f4d6a6"), g * 0.6, Mix::Light)
}

/// Fog color: the sky's light, a little greyer.
fn fog(x: f32, y: f32) -> Rgb {
    mix(sky(x, HORIZON), hex("#cfc8c0"), 0.35 + 0.25 * smoothstep(HORIZON - 40.0, HORIZON + 60.0, y), Mix::Light)
}

/// Snow drifts on the rise: (offset below the crest, x from, x to, phase).
const DRIFTS: [(f32, f32, f32, f32); 5] =
    [(38.0, -60.0, 400.0, 0.3), (58.0, 560.0, 1060.0, 1.7), (96.0, 140.0, 720.0, 2.9), (128.0, 640.0, 1060.0, 4.1), (150.0, -60.0, 330.0, 5.3)];

fn drift_y(d: &(f32, f32, f32, f32), x: f32) -> f32 {
    crest(x) + d.0 + 9.0 * (x * 0.009 + d.3).sin() + 4.0 * (x * 0.023 + 2.0 * d.3).sin()
}

/// How strongly a drift shows at x (it rises out of the slope and sinks back).
fn drift_fade(d: &(f32, f32, f32, f32), x: f32) -> f32 {
    smoothstep(d.1, d.1 + 90.0, x) * (1.0 - smoothstep(d.2 - 90.0, d.2, x))
}

/// The color of the snow on the rise.
fn snow_col(x: f32, y: f32) -> Rgb {
    let d = (y - crest(x)).max(0.0);
    let mut c = gradient(
        &[(0.0, hex("#e7dccb")), (0.1, hex("#cfc9ca")), (0.35, hex("#bcbac4")), (0.7, hex("#a2a4b6")), (1.0, hex("#898ca2"))],
        (d / 200.0).clamp(0.0, 1.0),
        Mix::Light,
    );
    for dr in DRIFTS.iter() {
        let fade = drift_fade(dr, x);
        if fade <= 0.0 {
            continue;
        }
        let s = y - drift_y(dr, x);
        // the face toward us, in shadow, lightening down to the trough
        let sh = if s > 0.0 { (-s / 22.0).exp() * smoothstep(0.0, 2.0, s) } else { 0.0 };
        // the top of the drift, catching the light
        let li = smoothstep(-14.0, -2.0, s) * (1.0 - smoothstep(-2.0, 1.0, s));
        c = mix(c, hex("#8e93ab"), sh * 0.75 * fade, Mix::Light);
        c = mix(c, hex("#e2d9ca"), li * 0.55 * fade, Mix::Light);
    }
    c
}

/// Strokes run along the rise and along the drifts.
fn snow_angle(x: f32, y: f32) -> f32 {
    let d = (y - crest(x)).max(0.0);
    (crest_slope(x) * (1.0 - smoothstep(0.0, 170.0, d))).atan()
}

/// One brush, loaded once, one gesture.
#[allow(clippy::too_many_arguments)]
fn stroke(c: &mut Canvas, tool: Tool, paint: Paint, load: f32, pts: Vec<(f32, f32)>, p: (f32, f32), ramps: (f32, f32), seed: u64) {
    let mut h = Held::new(tool, seed);
    h.load(paint, load);
    c.drag(&mut h, &Gesture::new(pts).pressure(p.0, p.1).ramps(ramps.0, ramps.1).orient(Orient::Across), None);
}

/// The traveler, seen from behind: a long dark greatcoat to below the knee,
/// the soft beret of the altdeutsche Tracht, a staff in his right hand.
/// Written as brush gestures in a local frame (u across, v up from the
/// soles, in figure heights). Back to front: staff, legs, coat, arms, head,
/// hat, then a lean rim of light on the side toward the glow.
fn traveler(c: &mut Canvas, pal: &paint::Palette, at: (f32, f32), size: f32, seed: u64) {
    let mut h = Hand::new(at, size, seed);
    h.tremor = 0.002;
    let coat = pal.paint(hex("#23221f"), 0.05).with_hiding(0.97).with_stiff(0.9);
    let dark = pal.paint(hex("#161412"), 0.1).with_hiding(0.97).with_stiff(0.5);
    let hair = pal.paint(hex("#2b2219"), 0.1).with_hiding(0.95).with_stiff(0.45);
    // the staff, planted a little ahead and to the right
    let mut st = h.take(Tool::rigger, 0.013, dark, 1.0);
    h.mark(c, &mut st, Mark { pts: &[(0.135, 0.56), (0.15, 0.28), (0.165, 0.0)], pressure: (0.9, 0.8), ramps: (0.0, 0.05) }, None);
    // legs: boots and trousers in one pull each, from the knee to the heel,
    // pressing harder at the heel so the boot swells
    let mut lg = h.take(Tool::round_sable, 0.05, dark, 1.0);
    for u in [-0.03f32, 0.032] {
        h.mark(c, &mut lg, Mark { pts: &[(u, 0.34), (u * 1.05, 0.15), (u * 1.1, 0.005)], pressure: (0.8, 1.0), ramps: (0.0, 0.0) }, None);
    }
    c.dry();
    // the greatcoat: long strokes from the shoulders to the hem, pressing
    // harder toward the hem so the skirt widens
    let mut cb = h.take(Tool::round_sable, 0.075, coat, 1.0);
    for (i, (u0, u1)) in [(-0.05f32, -0.09f32), (0.05, 0.092), (-0.02, -0.035), (0.02, 0.04), (0.0, 0.0)].iter().enumerate() {
        if i % 2 == 0 {
            cb.reload(coat, 1.0);
        }
        let mid = (u0 + u1) * 0.5 + h.rng.normal() * 0.003;
        h.mark(c, &mut cb, Mark { pts: &[(*u0, 0.8), (mid, 0.55), (*u1, 0.27)], pressure: (0.7, 1.0), ramps: (0.05, 0.03) }, None);
    }
    cb.reload(coat, 1.0);
    // shoulders, and the hem squared off
    h.line(c, &mut cb, &[(-0.09, 0.755), (-0.045, 0.815), (0.045, 0.815), (0.09, 0.755)], 0.7, 0.7);
    h.line(c, &mut cb, &[(-0.1, 0.285), (0.0, 0.27), (0.1, 0.285)], 0.75, 0.75);
    // arms: the left hangs at his side, the right reaches forward to the staff
    let mut ab = h.take(Tool::round_sable, 0.045, coat, 1.0);
    h.line(c, &mut ab, &[(-0.085, 0.77), (-0.1, 0.62), (-0.095, 0.48)], 0.8, 0.6);
    h.line(c, &mut ab, &[(0.08, 0.77), (0.115, 0.65), (0.13, 0.55)], 0.8, 0.6);
    // the collar standing up round the neck
    let mut cl = h.take(Tool::round_sable, 0.04, coat, 0.8);
    h.line(c, &mut cl, &[(-0.035, 0.82), (0.0, 0.845), (0.035, 0.82)], 0.8, 0.8);
    c.dry();
    // head, hair at the nape, and the beret resting on it, tipped a little
    let mut hd = h.take(Tool::round_sable, 0.058, hair, 0.5);
    h.dab(c, &mut hd, 0.002, 0.875, 0.05, std::f32::consts::FRAC_PI_2, 1.0);
    let mut be = h.take(Tool::round_sable, 0.04, dark, 0.5);
    h.mark(c, &mut be, Mark { pts: &[(-0.068, 0.905), (0.0, 0.925), (0.066, 0.912)], pressure: (0.85, 0.9), ramps: (0.05, 0.2) }, None);
    c.dry();
    // the glow finds the right shoulder and the right edge of the coat
    let rim = pal.paint(hex("#6f6254"), 0.4).with_hiding(0.55).with_stiff(0.4);
    let mut rb = h.take(Tool::round_sable, 0.014, rim, 0.25);
    h.mark(c, &mut rb, Mark { pts: &[(0.02, 0.83), (0.07, 0.81), (0.1, 0.75)], pressure: (0.4, 0.25), ramps: (0.2, 0.5) }, None);
    h.mark(c, &mut rb, Mark { pts: &[(0.1, 0.62), (0.105, 0.45), (0.11, 0.3)], pressure: (0.3, 0.15), ramps: (0.3, 0.6) }, None);
}

/// A spruce as Friedrich draws it: a slender dark spire, the branches short
/// hatched strokes drooping from the stem, a few longer ones breaking the
/// outline, the leader a single thin flick (NG p.50: firs in short hatched
/// strokes). Written in a local frame, `height` in units. `paint_at(y)` is
/// the paint the painter mixes for branches at canvas height y (a tree
/// standing in mist gets paler toward its foot); he goes back to the
/// palette every few tiers.
fn spruce(c: &mut Canvas, base: (f32, f32), height: f32, paint_at: &dyn Fn(f32) -> Paint, seed: u64) {
    let mut h = Hand::new(base, height, seed);
    h.tremor = 0.003;
    let slim = h.rng.range(0.17, 0.23);
    let tw = (1.6 / height).max(0.012);
    let top = paint_at(base.1 - height);
    let mut t = h.take(Tool::round_sable, tw, top, 0.7);
    h.mark(c, &mut t, Mark { pts: &[(0.0, 0.3), (0.001, 0.6), (0.0, 0.97)], pressure: (0.9, 0.35), ramps: (0.0, 0.3) }, None);
    let bw = (1.2 / height).max(0.018);
    let mut b = h.take(Tool::round_sable, bw, top, 0.6);
    let tiers = (height / 1.4).clamp(14.0, 80.0) as usize;
    for i in 0..tiers {
        let v = 0.96 - (i as f32 + h.rng.range(0.0, 0.7)) / tiers as f32 * 0.93;
        if i % 3 == 0 {
            b.reload(paint_at(base.1 - v * height), 0.6);
        }
        for s in [-1.0f32, 1.0] {
            if h.rng.chance(0.12) {
                continue;
            }
            let long = if h.rng.chance(0.1) { 1.35 } else { 1.0 };
            let reach = (0.01 + slim * (1.0 - v).powf(1.05)) * h.rng.range(0.6, 1.1) * long;
            let droop = reach * h.rng.range(0.35, 0.7);
            let pts = [(s * 0.002, v + 0.004), (s * reach * 0.55, v - droop * 0.6), (s * reach, v - droop)];
            h.mark(c, &mut b, Mark { pts: &pts, pressure: (0.85, 0.25), ramps: (0.0, 0.5) }, None);
        }
    }
    h.mark(c, &mut t, Mark { pts: &[(0.0, 0.9), (0.0015, 1.02)], pressure: (0.6, 0.15), ramps: (0.0, 0.6) }, None);
}

/// A blender that is wiped after every stroke.
fn clean_blend(st: &Style) -> Handling<'_> {
    st.blend().expect("Friedrich blends").dips(1, 0.0, 0.95)
}

/// One layer of sky, worked in horizontal bands from the top down, each band
/// fused with the badger before the next is started; then left to dry.
fn sky_layer(c: &mut Canvas, st: &Style, col: &(dyn Fn(f32, f32) -> Rgb + Sync), medium: f32, bottom: f32, seed: u64) {
    let f = c.frame();
    let m = Mask::from_fn(f, move |_, y| if y < bottom { 1.0 } else { 0.0 });
    let h = st.broad().color(col).angle(|_, _| 0.0).coverage(5.0).medium(medium).dips(1, 0.55, 0.7).length(60.0, 160.0);
    c.work(&m, &h, seed);
    let along = clean_blend(st).pressure(0.3, 0.4);
    for k in 0..4 {
        c.work(&m, &along, seed * 10 + k);
    }
    c.dry();
}

fn main() {
    let run = Run::new("fresh_winter");
    let st = Style::friedrich();
    let pal = &st.palette;
    let mut c = st.prepare(run.width, ASPECT, run.seed);
    let f = c.frame();
    let h = c.height();

    // ---- underdrawing: graphite lines, ruled where the thing is ruled
    let graphite = pal.paint(hex("#555049"), 0.55);
    let pencil = Tool { width: 0.9, length: 4.0, ..Tool::rigger(0.9) };
    let crest_line: Vec<(f32, f32)> = (0..=40).map(|i| (i as f32 * 25.0, crest(i as f32 * 25.0))).collect();
    stroke(&mut c, pencil.clone(), graphite, 0.25, crest_line, (0.45, 0.4), (0.05, 0.05), 1);
    stroke(&mut c, pencil.clone(), graphite, 0.2, vec![(0.0, HORIZON), (1000.0, HORIZON + 2.0)], (0.35, 0.35), (0.05, 0.05), 2);
    // the cross, ruled
    let cross_base = (AXIS + 2.0, crest(AXIS) + 4.0);
    let cross_top = (AXIS - 3.0, 296.0);
    stroke(&mut c, pencil.clone(), graphite, 0.25, vec![cross_base, cross_top], (0.5, 0.5), (0.02, 0.02), 3);
    stroke(&mut c, pencil.clone(), graphite, 0.25, vec![(AXIS - 40.0, 333.0), (AXIS + 38.0, 332.0)], (0.5, 0.5), (0.02, 0.02), 4);
    // oak axis, figure, spruce axes
    stroke(&mut c, pencil.clone(), graphite, 0.2, vec![(190.0, crest(190.0)), (182.0, 420.0), (200.0, 300.0)], (0.4, 0.3), (0.05, 0.1), 5);
    stroke(&mut c, pencil.clone(), graphite, 0.2, vec![(462.0, 612.0), (466.0, 668.0)], (0.4, 0.4), (0.05, 0.1), 6);
    c.dry();

    // ---- underpainting: a very thin violet layer in the sky (as in the
    // sky of The Cross in the Mountains, KÖR fig. 6), thin warm grey on the land
    let sky_m = Mask::from_fn(f, |_, y| 1.0 - smoothstep(HORIZON + 30.0, HORIZON + 60.0, y));
    let under_sky = st.broad().color(|_, y| mix(hex("#8a7f95"), hex("#b59c9a"), y / HORIZON, Mix::Light)).medium(0.8).dips(2, 0.25, 0.5).coverage(1.8);
    c.work(&sky_m, &under_sky, 11);
    let land_m = Mask::from_fn(f, |_, y| smoothstep(HORIZON + 10.0, HORIZON + 40.0, y));
    let under_land = st.broad().color(|_, _| hex("#9c948c")).medium(0.8).dips(2, 0.25, 0.5).coverage(1.8);
    c.work(&land_m, &under_land, 12);
    c.dry();

    // ---- sky: long horizontal strokes, fused with the badger, then a stipple
    // first layer: the darker, bluer sky everywhere, laid fairly full;
    // second layer: the light, thinner, over it. Worked in bands from the
    // top down, so the brush only carries the neighboring color.
    sky_layer(&mut c, &st, &|x, y| mix(sky(x, y), hex("#7f7f98"), 0.08 * (1.0 - y / HORIZON), Mix::Light), 0.3, HORIZON + 50.0, 21);
    sky_layer(&mut c, &st, &sky, 0.3, HORIZON + 50.0, 26);
    // the young moon, a thin crescent, its lit side turned down toward
    // where the sun has gone: two pulls of a small round from the belly out
    // to each horn, pressing less as they go
    let (mx, my, mr) = (812.0f32, 118.0f32, 10.0f32);
    let moon = pal.paint(hex("#f3ecd2"), 0.12);
    let belly = 2.35f32; // radians: lower left
    let mut k = 0u64;
    // arcs from the outer rim inward, each shorter: thick at the belly,
    // tapering to the horns
    for (ri, (dr, span)) in [(0.0f32, 1.5f32), (1.1, 1.15), (2.1, 0.75)].iter().enumerate() {
        for dir in [-1.0f32, 1.0] {
            let pts: Vec<(f32, f32)> = (0..=10)
                .map(|i| {
                    let a = belly + dir * span * i as f32 / 10.0;
                    (mx + a.cos() * (mr - dr), my + a.sin() * (mr - dr))
                })
                .collect();
            let w = if ri == 0 { 1.5 } else { 1.3 };
            stroke(&mut c, Tool::round_sable(w), moon, 0.7, pts, (0.8, 0.15), (0.0, 0.6), 24 + k);
            k += 1;
        }
    }
    c.dry();

    // ---- distance: faint far hills, the snow plain, the fog
    let plain_m = Mask::from_fn(f, |x, y| smoothstep(HORIZON - 2.0, HORIZON + 2.0, y) * (1.0 - smoothstep(crest(x) + 30.0, crest(x) + 50.0, y)));
    let plain = st.broad().color(|x, y| mix(fog(x, y), hex("#b9b8c2"), smoothstep(HORIZON, 560.0, y) * 0.6, Mix::Light)).medium(0.25).coverage(4.0).dips(1, 0.5, 0.7).angle(|_, _| 0.0);
    c.work(&plain_m, &plain, 32);
    c.work(&plain_m, &plain, 35);
    { let b = clean_blend(&st);
        c.work(&plain_m, &b, 33);
    }
    c.dry();

    // the church tower far off in the fog
    let tx = 655.0;
    let tower = Shape::new()
        .poly(&[(tx - 4.0, HORIZON + 2.0), (tx - 4.0, 395.0), (tx - 5.0, 393.0), (tx, 362.0), (tx + 5.0, 393.0), (tx + 4.0, 395.0), (tx + 4.0, HORIZON + 2.0)])
        .poly(&[(tx + 4.0, HORIZON + 2.0), (tx + 4.0, 428.0), (tx + 21.0, 407.0), (tx + 38.0, 428.0), (tx + 38.0, HORIZON + 2.0)]);
    let tower_m = Mask::from_shape(f, tower);
    let tower_paint = st.detail().color(|x, y| mix(fog(x, y), hex("#8c8a98"), 0.4 * (1.0 - smoothstep(405.0, HORIZON, y) * 0.7), Mix::Light)).medium(0.35).angle(|_, _| -1.5707);
    c.work(&tower_m, &tower_paint, 34);
    c.dry();

    // far rows of spruces rising out of the fog, back to front: each tree
    // mixed paler toward its foot, then the mist stippled over the feet of
    // the row. The living wood stands on the right; on the left, only a few
    // faint trees far off, so the dead oak stands alone against the mist.
    let mut rng = Rng::new(40);
    for row in 0..3 {
        let depth = row as f32 / 2.0; // 0 far .. 1 near
        let base_y = HORIZON + 4.0 + depth * 26.0;
        let groups: &[(f32, f32, f32)] = if row == 0 { &[(680.0, 1000.0, 1.0)] } else { &[(700.0, 1000.0, 1.0)] };
        let dark = mix(fog(500.0, base_y), hex("#343a38"), 0.35 + 0.55 * depth, Mix::Light);
        let lift = 38.0 + 30.0 * depth;
        let paint_at = move |y: f32| {
            let k = smoothstep(base_y - lift, base_y + 2.0, y);
            pal.paint(mix(dark, fog(700.0, y), k * 0.92, Mix::Light), 0.2)
        };
        for &(x0, x1, dens) in groups {
            let n = ((x1 - x0) / (5.0 + depth * 9.0) * dens) as usize;
            for _ in 0..n {
                let x = rng.range(x0, x1);
                // the wood is highest toward its middle
                let mid = ((x - 700.0) / 300.0).clamp(0.0, 1.0);
                let hump = if x0 < 200.0 { 0.35 } else { (1.0 - (2.0 * mid - 1.0).powi(2)).max(0.0).sqrt() };
                let ht = (28.0 + 70.0 * depth) * (0.4 + 0.75 * hump) * rng.range(0.65, 1.15);
                spruce(&mut c, (x, base_y + rng.range(-3.0, 4.0)), ht, &paint_at, rng.next_u64());
            }
        }
        c.dry();
        let top = base_y - 70.0;
        let fog_m = Mask::from_fn(f, move |_, y| smoothstep(top, top + 10.0, y) * (1.0 - smoothstep(base_y + 25.0, base_y + 45.0, y)));
        // the mist: a veil of thin pale paint in long, narrow, horizontal
        // strokes, loaded more toward the ground, then fused with the badger
        let veil_tool = Tool { stiffness: 0.3, lay: 0.8, pickup: 0.06, push: 0.03, ragged: 0.25, ..Tool::filbert(18.0) };
        for pass in 0..1u64 {
            let veil = Handling::new(veil_tool.clone())
                .mixed(pal, 0.55)
                .mix_jitter(0.03)
                .color(fog)
                .length(90.0, 220.0)
                .coverage(2.5)
                .pressure(0.45, 0.65)
                .dips(1, 0.3, 0.8)
                .ramps(0.25, 0.4)
                .angle(|_, _| 0.0)
                .angle_jitter(0.015)
                .load_at(move |_, y| smoothstep(top, base_y + 6.0, y).powf(2.0) * 0.45)
                .threshold(0.05);
            c.work(&fog_m, &veil, 47 + row as u64 * 10 + pass);
            let b = clean_blend(&st).pressure(0.3, 0.4);
            c.work(&fog_m, &b, 48 + row as u64 * 10 + pass);
            c.work(&fog_m, &b, 49 + row as u64 * 10 + pass);
            c.dry();
        }
        c.dry();
    }

    // ---- the foreground rise: snow in shadow (the light is behind the
    // fog), cool lavender grey, long drifts whose tops catch the light and
    // whose faces toward us are blue, a warm lit edge along the crest
    let fg_m = Mask::from_fn(f, |x, y| smoothstep(crest(x) - 1.0, crest(x) + 1.0, y));
    let snow = st.body().color(snow_col).angle(snow_angle).length(30.0, 80.0).medium(0.25).coverage(5.0);
    c.work(&fg_m, &snow, 51);
    {
        // fused near the crest; lower down the strokes are left to show
        let near_crest = Mask::from_fn(f, |x, y| smoothstep(crest(x) - 1.0, crest(x) + 1.0, y) * (1.0 - smoothstep(crest(x) + 50.0, crest(x) + 90.0, y)));
        let b = clean_blend(&st).pressure(0.3, 0.4).angle(snow_angle).length(60.0, 160.0);
        c.work(&near_crest, &b, 52);
        let light = clean_blend(&st).pressure(0.15, 0.25).angle(snow_angle).length(40.0, 90.0).coverage(1.2);
        c.work(&fg_m, &light, 55);
    }
    c.dry();
    // slight impasto of the foreground snow (NG p.50): lead white laid in
    // long strokes along the crest and the tops of the drifts
    let mut r = Rng::new(53);
    let warm = |x: f32| mix(hex("#ebe3d3"), hex("#f1dfc2"), (-((x - 610.0) / 250.0).powi(2)).exp(), Mix::Light);
    let mut lines: Vec<(f32, f32, f32, f32)> = vec![(-20.0, 1020.0, 2.5, 6.0)];
    for d in DRIFTS {
        lines.push((d.1, d.2, -2.0, 7.0));
    }
    for (li, &(x0, x1, off, w)) in lines.iter().enumerate() {
        let mut x = x0 + r.range(0.0, 30.0);
        while x < x1 {
            let len = r.range(70.0, 190.0);
            let xe = (x + len).min(x1);
            let n = ((xe - x) / 12.0).ceil().max(2.0) as usize;
            let pts: Vec<(f32, f32)> = (0..=n)
                .map(|k| {
                    let xx = x + (xe - x) * k as f32 / n as f32;
                    let yy = if li == 0 { crest(xx) + off } else { drift_y(&DRIFTS[li - 1], xx) + off };
                    (xx, yy + r.normal() * 0.6)
                })
                .collect();
            let (mx, my) = pts[pts.len() / 2];
            let paint = if li == 0 { pal.paint(mix(snow_col(mx, my + 3.0), warm(x), 0.7, Mix::Light), 0.1) } else { pal.paint(mix(snow_col(mx, my + 6.0), warm(x), 0.55, Mix::Light), 0.12) };
            let fade = if li == 0 { 1.0 } else { drift_fade(&DRIFTS[li - 1], x + len * 0.5) };
            if fade > 0.2 {
                let p0 = r.range(0.5, 0.85) * fade;
                stroke(&mut c, Tool { ragged: 0.45, ..Tool::filbert(w * r.range(0.7, 1.1)) }, paint, r.range(0.35, 0.6), pts, (p0, p0 * r.range(0.3, 0.8)), (0.25, 0.45), r.next_u64());
            }
            // the next pull starts back inside this one, so the light runs on
            x = xe - r.range(20.0, 45.0);
        }
    }
    // soften the drift lights into the wet snow around them, and pass the
    // badger once, lightly, along the crest so the lit edge runs on
    {
        let drift_m = Mask::from_fn(f, |x, y| smoothstep(crest(x) + 15.0, crest(x) + 25.0, y));
        let b = clean_blend(&st).pressure(0.3, 0.4).angle(snow_angle).length(40.0, 100.0).coverage(2.0);
        c.work(&drift_m, &b, 54);
        let crest_m = Mask::from_fn(f, |x, y| smoothstep(crest(x) - 2.0, crest(x) + 1.0, y) * (1.0 - smoothstep(crest(x) + 8.0, crest(x) + 14.0, y)));
        let bc = clean_blend(&st).pressure(0.2, 0.3).angle(|x, _| crest_slope(x).atan()).length(60.0, 140.0).coverage(1.5).threshold(0.2);
        c.work(&crest_m, &bc, 56);
    }
    c.dry();

    // ---- the dead oak on the left
    let mut oak = Oak::new((190.0, crest(190.0) + 6.0), 400.0, 17);
    oak.gnarl = 0.85;
    oak.broken = 0.32;
    oak.lean = -0.05;
    oak.depth = 5;
    oak.roots = 3;
    let bark = pal.mix(hex("#2b2522")).color;
    let bark_light = pal.mix(hex("#8e8472")).color;
    oak.paint(&mut c, bark, Some(bark_light), 18);
    c.dry();
    // snow lying along the tops of the heavier limbs
    let snow_paint = pal.paint(hex("#d9d3cb"), 0.2);
    let mut r = Rng::new(19);
    for l in oak.grow().iter().filter(|l| l.w[0] > 1.6 && l.depth <= 3) {
        // runs of consecutive limb segments that lie more across than up:
        // one pull of a small round along the top of each run
        let mut run_pts: Vec<(f32, f32)> = Vec::new();
        let mut run_w = f32::MAX;
        let n = l.pts.len();
        for i in 0..n {
            let flat_here = if i + 1 < n {
                let (a, b) = (l.pts[i], l.pts[i + 1]);
                let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                let len = (dx * dx + dy * dy).sqrt().max(1e-3);
                dx.abs() / len > 0.6
            } else {
                false
            };
            // offset toward the upper side of the limb at this point
            let (a, b) = (l.pts[i.saturating_sub(1)], l.pts[(i + 1).min(n - 1)]);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let len = (dx * dx + dy * dy).sqrt().max(1e-3);
            let (nx, ny) = (-dy / len, dx / len);
            let (nx, ny) = if ny < 0.0 { (nx, ny) } else { (-nx, -ny) };
            let p = (l.pts[i].0 + nx * l.w[i] * 0.4, l.pts[i].1 + ny * l.w[i] * 0.4);
            if flat_here || !run_pts.is_empty() {
                run_pts.push(p);
                run_w = run_w.min(l.w[i]);
            }
            if !flat_here && run_pts.len() >= 2 {
                if r.chance(0.6) {
                    let w = (run_w * 0.3).max(0.7);
                    stroke(&mut c, Tool { ragged: 0.5, ..Tool::round_sable(w) }, snow_paint, 0.35, run_pts.clone(), (0.45, 0.15), (0.25, 0.5), r.next_u64());
                }
                run_pts.clear();
                run_w = f32::MAX;
            } else if !flat_here {
                run_pts.clear();
            }
        }
    }
    c.dry();
    // snow drifted against the foot of the oak, over the roots: the bank is
    // cut in with the body brush along its upper edge and worked out into
    // the snow of the rise below
    let bank_top = |x: f32| crest(x) - 1.5 - 10.0 * (-((x - 195.0) / 42.0).powi(2)).exp();
    let bank_m = Mask::from_fn(f, move |x, y| {
        let side = smoothstep(95.0, 125.0, x) * (1.0 - smoothstep(290.0, 320.0, x));
        let top = bank_top(x);
        side * smoothstep(top - 0.8, top + 0.8, y) * (1.0 - smoothstep(crest(x) + 40.0, crest(x) + 60.0, y))
    });
    let bank = st.body().color(snow_col).angle(snow_angle).length(15.0, 40.0).medium(0.2).coverage(4.0).clip(true).threshold(0.2);
    c.work(&bank_m, &bank, 20);
    {
        let b = clean_blend(&st).pressure(0.25, 0.35).length(30.0, 60.0).coverage(2.0);
        c.work(&bank_m, &b, 21);
    }
    c.dry();
    let rim_light = pal.paint(hex("#e7decd"), 0.1);
    let pts: Vec<(f32, f32)> = (0..=17).map(|i| { let x = 128.0 + i as f32 * 10.0; (x, bank_top(x) + 1.5) }).collect();
    stroke(&mut c, Tool { ragged: 0.4, ..Tool::filbert(3.5) }, rim_light, 0.5, pts, (0.6, 0.35), (0.3, 0.4), 22);
    c.dry();

    // ---- the cross between two young spruces: squared beams, laid with a
    // small flat, each in two pulls
    let wood = pal.paint(hex("#2e2722"), 0.1);
    // (a flat that barely ploughs: a hog flat pushes its own wet paint to
    // the edges of the beam and leaves it hollow)
    let wood_tool = |w: f32| Tool { stiffness: 0.45, ragged: 0.12, lay: 1.2, push: 0.02, pickup: 0.04, ..Tool::hog_flat(w) };
    for k in 0..2 {
        stroke(&mut c, wood_tool(6.5), wood, 1.0, vec![cross_base, (AXIS - 0.5, 420.0), cross_top], (0.95, 0.9), (0.0, 0.0), 61 + k);
        stroke(&mut c, wood_tool(5.5), wood, 1.0, vec![(AXIS - 40.0, 333.0), (AXIS + 38.0, 332.0)], (0.9, 0.9), (0.0, 0.0), 63 + k);
        c.dry();
    }
    // the glow catches the right edge of the upright, barely
    let rim = pal.paint(hex("#7a6a58"), 0.45);
    stroke(&mut c, Tool::round_sable(1.1), rim, 0.3, vec![(AXIS + 4.0, crest(AXIS) - 4.0), (AXIS + 2.0, 420.0), (AXIS - 0.2, 338.0)], (0.45, 0.3), (0.2, 0.3), 65);
    let snow_top = pal.paint(hex("#ede5d6"), 0.1);
    stroke(&mut c, Tool::round_sable(2.2), snow_top, 0.6, vec![(AXIS - 39.0, 329.8), (AXIS + 37.0, 328.8)], (0.7, 0.6), (0.1, 0.2), 66);
    stroke(&mut c, Tool::round_sable(2.4), snow_top, 0.6, vec![(AXIS - 5.5, 294.5), (AXIS - 0.5, 294.5)], (0.8, 0.6), (0.1, 0.2), 67);
    let spruce_paint = pal.paint(hex("#1f2622"), 0.1);
    spruce(&mut c, (AXIS - 52.0, crest(AXIS - 52.0) + 4.0), 95.0, &|_| spruce_paint, 68);
    spruce(&mut c, (AXIS + 55.0, crest(AXIS + 55.0) + 4.0), 118.0, &|_| spruce_paint, 69);
    c.dry();
    // snow heaped at the feet of the cross and the spruces
    let heap = pal.paint(hex("#e4dbcc"), 0.12);
    for (k, &(x, w)) in [(AXIS + 1.0, 12.0f32), (AXIS - 52.0, 16.0), (AXIS + 55.0, 18.0)].iter().enumerate() {
        let y = crest(x) + 3.0;
        stroke(&mut c, Tool::round_sable(5.0), heap, 0.6, vec![(x - w, y + 1.5), (x, y - 1.0), (x + w, y + 1.5)], (0.75, 0.6), (0.2, 0.3), 70 + k as u64);
    }
    c.dry();

    // ---- figures last: the traveler, and the crows
    // where he stands the snow is trodden: a blue hollow under his feet
    stroke(&mut c, Tool::round_sable(3.5), pal.paint(hex("#858aa0"), 0.25), 0.45, vec![(449.0, 666.5), (459.0, 667.0), (470.0, 666.5)], (0.55, 0.35), (0.2, 0.4), 70);
    c.dry();
    traveler(&mut c, pal, (458.0, 666.0), 80.0, 71);
    let crow = pal.paint(hex("#191715"), 0.1);
    let mut r = Rng::new(72);
    // perched on the ends of the outer limbs of the oak
    let mut perches: Vec<(f32, f32)> = oak
        .grow()
        .iter()
        .filter(|l| l.depth == 2 || l.depth == 3)
        .map(|l| {
            let n = l.pts.len();
            let (a, b) = (l.pts[n - 2], l.pts[n - 1]);
            (a.0 + (b.0 - a.0) * 0.6, a.1 + (b.1 - a.1) * 0.6)
        })
        .filter(|p| p.1 < 330.0 && p.1 > 150.0)
        .collect();
    perches.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let picks: Vec<(f32, f32)> = [1usize, perches.len() / 2, perches.len().saturating_sub(3)].iter().filter_map(|&i| perches.get(i).copied()).collect();
    for (x, y) in picks {
        let s = 1.0;
        stroke(&mut c, Tool::round_sable(2.6 * s), crow, 0.9, vec![(x, y - 5.0 * s), (x + 0.6, y - 1.0)], (1.0, 0.9), (0.0, 0.2), r.next_u64());
        stroke(&mut c, Tool::round_sable(1.5 * s), crow, 0.9, vec![(x + 0.2, y - 6.2 * s), (x - 1.8, y - 5.6 * s)], (0.9, 0.6), (0.0, 0.3), r.next_u64());
        stroke(&mut c, Tool::round_sable(1.3 * s), crow, 0.9, vec![(x + 0.6, y - 1.0), (x + 1.6, y + 3.0 * s)], (0.9, 0.3), (0.0, 0.4), r.next_u64());
    }
    // two flying, far off over the fog: a shallow M, wings a hair thick
    for &(x, y, s) in &[(770.0f32, 214.0f32, 1.0f32), (803.0, 199.0, 0.8)] {
        let t = Tool::round_sable(0.9 * s);
        stroke(&mut c, t.clone(), crow, 0.7, vec![(x - 8.0 * s, y + 1.0 * s), (x - 4.5 * s, y - 2.2 * s), (x, y)], (0.25, 0.6), (0.1, 0.1), r.next_u64());
        stroke(&mut c, t, crow, 0.7, vec![(x, y), (x + 4.5 * s, y - 2.4 * s), (x + 8.5 * s, y + 0.5 * s)], (0.6, 0.25), (0.1, 0.1), r.next_u64());
        stroke(&mut c, Tool::round_sable(1.6 * s), crow, 0.7, vec![(x - 0.8 * s, y + 0.3), (x + 0.8 * s, y + 0.5)], (0.7, 0.7), (0.1, 0.1), r.next_u64());
    }
    c.dry();

    // ---- grass over the finished snow with fine upturning strokes (NG
    // p.56), in clumps: along the crest where the wind blew the snow thin,
    // round the foot of the oak, and in the near corners
    let grass = [pal.paint(hex("#5a4a38"), 0.2), pal.paint(hex("#3b3129"), 0.2), pal.paint(hex("#76644c"), 0.2)];
    let mut r = Rng::new(81);
    let mut clumps: Vec<(f32, f32, f32, usize)> = Vec::new();
    for _ in 0..14 {
        let x = r.range(0.0, 1000.0);
        if (x - AXIS).abs() < 70.0 {
            continue;
        }
        clumps.push((x, crest(x) + r.range(1.0, 5.0), r.range(0.6, 1.0), r.range(5.0, 14.0) as usize));
    }
    for _ in 0..5 {
        let x = 190.0 + r.normal() * 35.0;
        clumps.push((x, crest(x) + r.range(4.0, 14.0), r.range(0.7, 1.1), r.range(6.0, 14.0) as usize));
    }
    for _ in 0..10 {
        let x = if r.chance(0.5) { r.range(0.0, 190.0) } else { r.range(760.0, 1000.0) };
        clumps.push((x, r.range(h - 60.0, h + 5.0), r.range(0.9, 1.5), r.range(8.0, 22.0) as usize));
    }
    for (x, y, s, k) in clumps {
        let near = smoothstep(500.0, h, y);
        for _ in 0..k {
            let len = (5.0 + 20.0 * near) * s * r.range(0.5, 1.2);
            let lean = r.normal() * 0.35 + 0.1;
            let x0 = x + r.normal() * (4.0 + 8.0 * near) * s;
            let y0 = y + r.normal().abs() * 2.0 * near;
            let pts = vec![(x0, y0), (x0 + lean * len * 0.3, y0 - len * 0.55), (x0 + lean * len, y0 - len)];
            let g = grass[r.next_u64() as usize % 3];
            let w = 0.35 + 0.45 * near;
            stroke(&mut c, Tool { width: w, ..Tool::rigger(w) }, g, 0.6, pts, (0.8, 0.1), (0.0, 0.6), r.next_u64());
        }
    }
    c.dry();

    run.finish(&mut c, &mut (), &Finish::aged(st.relief));
}

/// (port, 2026-09-26) The oak as round 1's engine had it: `paint::Oak`,
/// crates/paint/src/tree.rs at dddd9af, removed from the engine in 26bdfb6
/// (growth replaced it). Copied here unchanged but for its imports, so the
/// painter's oak is the one they painted with (notes/wet_merge/PORTS.md).
mod oak {
    // Gnarled oaks in the manner of Friedrich (e.g. "Oak Tree in the Snow",
    // "The Abbey in the Oakwood"): a tall leader that keeps going up, crooked
    // side limbs made of straight lengths joined by sharp elbows, broken stubs,
    // an open crown, and short claw-like twigs all along the outer limbs.
    // Grown recursively, then painted with simulated brushes: each limb is one
    // drag of a round sable sized to it, twigs with a rigger, light with a
    // dry-brush pass on the lit side.

    use paint::{Canvas, Gesture, Held, Mask, Orient, Paint, Rgb, Rng, Shape, Tool};
    use std::f32::consts::{FRAC_PI_2, PI};

    #[derive(Clone, Debug)]
    pub struct Limb {
        pub pts: Vec<(f32, f32)>,
        /// Full width at each point (units).
        pub w: Vec<f32>,
        pub depth: u32,
    }

    #[derive(Clone, Debug)]
    pub struct Oak {
        /// Base of the trunk (units).
        pub base: (f32, f32),
        /// Rough overall height (units).
        pub height: f32,
        /// Trunk width at the base (units).
        pub trunk: f32,
        /// Lean of the trunk in radians (+ = right).
        pub lean: f32,
        /// 0 = straight, 1 = very gnarled.
        pub gnarl: f32,
        /// Branching depth (4–6).
        pub depth: u32,
        /// Probability a limb is broken off (dead oak ≈ 0.3).
        pub broken: f32,
        /// Twig clusters per unit of outer limb length.
        pub twigs: f32,
        /// Exposed roots at the base.
        pub roots: u32,
        pub seed: u64,
    }

    impl Oak {
        pub fn new(base: (f32, f32), height: f32, seed: u64) -> Self {
            Oak { base, height, trunk: height * 0.06, lean: 0.0, gnarl: 0.7, depth: 5, broken: 0.18, twigs: 0.14, roots: 2, seed }
        }

        pub fn grow(&self) -> Vec<Limb> {
            let mut rng = Rng::new(self.seed);
            let mut out = Vec::new();
            self.leader(&mut out, &mut rng);
            for i in 0..self.roots {
                let side = if i % 2 == 0 { 1.0 } else { -1.0 };
                let a = FRAC_PI_2 - side * rng.range(1.1, 1.45); // out and a little down
                let start = (self.base.0 + side * self.trunk * 0.25, self.base.1 - self.trunk * 0.15);
                let rl = self.height * rng.range(0.05, 0.1);
                let w = self.trunk * 0.55;
                self.limb(&mut out, &mut rng, start, a, rl, w, self.depth, 0.0, false);
            }
            out
        }

        /// The trunk continues upward as a leader, thinning, throwing side limbs.
        fn leader(&self, out: &mut Vec<Limb>, rng: &mut Rng) {
            // the trunk runs up to where it splits into two or three main limbs
            let total = self.height * rng.range(0.38, 0.55);
            let nseg = 5;
            let seg = total / nseg as f32;
            let (mut x, mut y) = self.base;
            let mut a = -FRAC_PI_2 + self.lean;
            let mut pts = vec![(x, y)];
            let mut ws = vec![self.trunk * 1.25]; // flare at the foot
            let mut kids = Vec::new();
            let mut side = if rng.chance(0.5) { 1.0 } else { -1.0 };
            for i in 1..=nseg {
                let t = i as f32 / nseg as f32;
                if i > 1 && rng.chance(0.35 * self.gnarl) {
                    a += rng.range(0.15, 0.4) * if rng.chance(0.5) { 1.0 } else { -1.0 };
                }
                a += angle_diff(-FRAC_PI_2 + self.lean, a) * 0.35;
                x += a.cos() * seg;
                y += a.sin() * seg;
                let w = self.trunk * (1.0 - 0.3 * t);
                pts.push((x, y));
                ws.push(w);
                // limbs start a third of the way up, alternating sides
                if t > 0.45 && t < 0.95 && rng.chance(0.6) {
                    let n = 1;
                    for _ in 0..n {
                        side = -side;
                        let a2 = -FRAC_PI_2 + side * rng.range(0.7, 1.45);
                        let len = self.height * rng.range(0.22, 0.42) * (1.1 - 0.5 * t);
                        kids.push(((x, y), a2, len, w * rng.range(0.5, 0.8), side));
                    }
                }
                // stubs of lost limbs
                if rng.chance(0.3 * self.broken / 0.18) {
                    let s = if rng.chance(0.5) { 1.0 } else { -1.0 };
                    let a2 = -FRAC_PI_2 + s * rng.range(0.9, 1.6);
                    let l = w * rng.range(0.8, 2.2);
                    out.push(Limb { pts: vec![(x, y), (x + a2.cos() * l, y + a2.sin() * l)], w: vec![w * 0.45, w * 0.4], depth: 1 });
                }
            }
            // the split: main limbs going up and out
            let n = if rng.chance(0.4) { 3 } else { 2 };
            let w_top = *ws.last().unwrap();
            for k in 0..n {
                let u = if n == 1 { 0.0 } else { k as f32 / (n - 1) as f32 * 2.0 - 1.0 };
                let a2 = a + u * rng.range(0.35, 0.7) + rng.normal() * 0.1;
                let s = if u == 0.0 { side_of(a2 + 0.001) } else { u.signum() };
                let len = self.height * rng.range(0.3, 0.45);
                kids.push(((x, y), a2, len, w_top * rng.range(0.6, 0.8), s));
            }
            out.push(Limb { pts, w: ws, depth: 0 });
            for (p, a2, l2, w2, s) in kids {
                self.limb(out, rng, p, a2, l2, w2, 1, s, true);
            }
        }

        #[allow(clippy::too_many_arguments)]
        fn limb(
            &self,
            out: &mut Vec<Limb>,
            rng: &mut Rng,
            start: (f32, f32),
            angle: f32,
            len: f32,
            width: f32,
            depth: u32,
            side: f32,
            can_break: bool,
        ) {
            if depth > self.depth + 1 || width < 0.1 || len < 0.8 {
                return;
            }
            let broken = can_break && depth <= 3 && rng.chance(self.broken);
            let len = if broken { len * rng.range(0.2, 0.45) } else { len };
            // a few straight lengths joined by elbows
            let nseg = (2.0 + len / 18.0).clamp(2.0, 5.0) as usize;
            let mut pts = vec![start];
            let mut ws = vec![width];
            let mut a = angle;
            let (mut x, mut y) = start;
            // outer limbs turn back upward at elbows; some droop and recover
            let up = -FRAC_PI_2 + side * rng.range(0.35, 0.8);
            let mut kids = Vec::new();
            for i in 1..=nseg {
                let t = i as f32 / nseg as f32;
                let seg = len / nseg as f32 * rng.range(0.7, 1.3);
                x += a.cos() * seg;
                y += a.sin() * seg;
                let w = width * (1.0 - 0.55 * t);
                pts.push((x, y));
                ws.push(w);
                // elbow: a sharp turn, mostly back toward "up", sometimes away
                let elbow = rng.range(0.3, 0.95) * self.gnarl;
                let toward = angle_diff(up, a).signum();
                a += if rng.chance(0.72) { toward * elbow } else { -toward * elbow * 0.8 };
                if i < nseg && !broken && rng.chance(0.55) {
                    let s = if rng.chance(0.5) { 1.0 } else { -1.0 };
                    let a2 = a + s * rng.range(0.6, 1.3);
                    kids.push(((x, y), a2, len * rng.range(0.45, 0.7), w * rng.range(0.45, 0.7), side_of(a2)));
                }
            }
            let tip = (x, y);
            let tip_w = *ws.last().unwrap();
            out.push(Limb { pts: pts.clone(), w: ws.clone(), depth });
            if broken {
                return;
            }
            for (p, a2, l2, w2, s) in kids {
                self.limb(out, rng, p, a2, l2, w2, depth + 1, s, true);
            }
            if depth < self.depth {
                let l2 = len * rng.range(0.55, 0.8);
                let a2 = a + rng.normal() * 0.25;
                self.limb(out, rng, tip, a2, l2, tip_w * 0.8, depth + 1, side, true);
            }
            // claw-like twig clusters along the outer limbs
            if depth + 2 >= self.depth {
                let total: f32 = pts.windows(2).map(|p| dist(p[0], p[1])).sum();
                let n = (total * self.twigs * rng.range(0.6, 1.4)).round() as usize;
                for _ in 0..n {
                    let (bi, t) = pick_along(&pts, rng.f());
                    let p0 = lerp(pts[bi], pts[bi + 1], t);
                    let w0 = (ws[bi] * 0.4).clamp(0.1, 0.28);
                    let mut a2 = -FRAC_PI_2 + rng.normal() * 0.9;
                    let mut p = p0;
                    let mut tp = vec![p];
                    let mut tw = vec![w0];
                    let l = self.height * rng.range(0.015, 0.045);
                    for k in 0..3 {
                        p = (p.0 + a2.cos() * l / 3.0, p.1 + a2.sin() * l / 3.0);
                        tp.push(p);
                        tw.push(w0 * (0.7 - 0.2 * k as f32).max(0.2));
                        a2 += rng.range(0.3, 0.9) * if rng.chance(0.5) { 1.0 } else { -1.0 };
                    }
                    out.push(Limb { pts: tp, w: tw, depth: depth + 2 });
                }
            }
        }

        /// Silhouette mask of the limbs (for glazing, mist, clipping).
        pub fn mask(&self, c: &Canvas, limbs: &[Limb]) -> Mask {
            let mut shape = Shape::new();
            for l in limbs {
                shape = shape.ribbon(&l.pts, &l.w);
            }
            Mask::from_shape(c.frame(), shape)
        }

        /// Paint the tree with simulated brushes: every limb a single drag of a
        /// round sable as wide as the limb (the bristles give bark streaks), twigs
        /// with a rigger, then a light dry-brush pass along the lit edge.
        pub fn paint(&self, c: &mut Canvas, dark: Rgb, light: Option<Rgb>, seed: u64) -> Mask {
            let limbs = self.grow();
            let mut rng = Rng::new(seed);
            // thick to thin, like a painter blocking the tree in
            let mut order: Vec<&Limb> = limbs.iter().collect();
            order.sort_by(|a, b| b.w[0].partial_cmp(&a.w[0]).unwrap());
            let paint = Paint::new(dark, 0.95, 0.9);
            for l in &order {
                let w0 = l.w[0];
                let w1 = *l.w.last().unwrap();
                let tool = if w0 < 0.9 {
                    Tool { width: w0 * 1.6, length: w0 * 3.0, splay: 0.15, stiffness: 0.3, ragged: 0.05, ..Tool::rigger(w0) }
                } else {
                    Tool { width: w0, length: w0 * 0.5, ragged: 0.35, ..Tool::round_sable(w0) }
                };
                let mut held = Held::new(tool, rng.next_u64());
                held.load(paint, 0.9);
                // pressure sets the width: full at the base, lighter to the tip
                let p1 = (w1 / w0).clamp(0.15, 1.0);
                let g = Gesture::new(l.pts.clone()).pressure(1.0, p1).ramps(0.0, if l.depth > 2 { 0.4 } else { 0.1 }).orient(Orient::Across);
                c.drag(&mut held, &g, None);
                // a stub from a broken limb ends blunt: dab the end
            }
            if let Some(lc) = light {
                // let the dark dry, then drag a lean light scumble along the lit
                // edge with hardly any pressure: it catches only the ridges of the
                // bark strokes and the canvas tooth, broken like real dry-brush
                c.dry();
                let mask = self.mask(c, &limbs);
                // model each big limb as a lit cylinder: a few thin lean streaks
                // laid along the limb on the side facing the light (upper left),
                // stepping from half-tone near the axis to the lightest near the
                // edge; low pressure so the canvas tooth and the bark ridges break
                // them up, and each covers a random stretch of the limb
                let light_dir = (-0.75f32, -0.66f32);
                for l in limbs.iter().filter(|l| l.w[0] > 2.0) {
                    let n = l.pts.len();
                    let normals: Vec<(f32, f32)> = (0..n)
                        .map(|i| {
                            let a = l.pts[i.saturating_sub(1)];
                            let b = l.pts[(i + 1).min(n - 1)];
                            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                            let d = (dx * dx + dy * dy).sqrt().max(1e-4);
                            let nn = (-dy / d, dx / d);
                            if nn.0 * light_dir.0 + nn.1 * light_dir.1 >= 0.0 { nn } else { (-nn.0, -nn.1) }
                        })
                        .collect();
                    // how squarely the limb faces the light: vertical limbs get a
                    // clear lit side, limbs pointing at the light hardly any
                    let facing = normals.iter().map(|nn| (nn.0 * light_dir.0 + nn.1 * light_dir.1).max(0.0)).sum::<f32>() / n as f32;
                    let streaks = 2 + (l.w[0] / 3.0).min(3.0) as usize;
                    for k in 0..streaks {
                        let u = 0.12 + 0.3 * (k as f32 + rng.range(0.0, 0.8)) / streaks as f32;
                        let tone = (0.25 + 0.75 * (k + 1) as f32 / streaks as f32) * (0.4 + 0.6 * facing);
                        let pts: Vec<(f32, f32)> = (0..n)
                            .map(|i| {
                                let w = l.w[i];
                                (l.pts[i].0 + normals[i].0 * w * u, l.pts[i].1 + normals[i].1 * w * u)
                            })
                            .collect();
                        // a random stretch of the limb
                        let a = rng.range(0.0, 0.4);
                        let b = (a + rng.range(0.35, 0.9)).min(1.0);
                        let sub = sub_path(&pts, a, b);
                        let col = paint::color::mix(dark, lc, tone, paint::color::Mix::Pigment);
                        let tool = Tool { ragged: 0.6, ..Tool::round_sable(l.w[0] * rng.range(0.1, 0.18)) };
                        let mut held = Held::new(tool, rng.next_u64());
                        held.load(Paint::new(col, 0.6, 0.4), rng.range(0.4, 0.8) * 0.4);
                        let g = Gesture::new(sub).pressure(rng.range(0.3, 0.5), rng.range(0.15, 0.3)).ramps(0.2, 0.4).orient(Orient::Across);
                        c.drag(&mut held, &g, Some(&mask));
                    }
                }
                return mask;
            }
            self.mask(c, &limbs)
        }
    }

    fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
        ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
    }

    fn lerp(a: (f32, f32), b: (f32, f32), t: f32) -> (f32, f32) {
        (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t)
    }

    /// Segment index and fraction at arc-length fraction `u` along a polyline.
    fn pick_along(pts: &[(f32, f32)], u: f32) -> (usize, f32) {
        let total: f32 = pts.windows(2).map(|p| dist(p[0], p[1])).sum();
        let mut target = u * total;
        for i in 0..pts.len() - 1 {
            let d = dist(pts[i], pts[i + 1]);
            if target <= d || i == pts.len() - 2 {
                return (i, (target / d.max(1e-6)).clamp(0.0, 1.0));
            }
            target -= d;
        }
        (0, 0.0)
    }

    fn angle_diff(target: f32, a: f32) -> f32 {
        let mut d = target - a;
        while d > PI {
            d -= 2.0 * PI;
        }
        while d < -PI {
            d += 2.0 * PI;
        }
        d
    }

    fn side_of(a: f32) -> f32 {
        if a.cos() >= 0.0 { 1.0 } else { -1.0 }
    }

    /// The part of a polyline between arc-length fractions `a` and `b`.
    fn sub_path(pts: &[(f32, f32)], a: f32, b: f32) -> Vec<(f32, f32)> {
        let (i0, t0) = pick_along(pts, a);
        let (i1, t1) = pick_along(pts, b);
        let mut out = vec![lerp(pts[i0], pts[i0 + 1], t0)];
        for p in &pts[i0 + 1..=i1] {
            out.push(*p);
        }
        out.push(lerp(pts[i1], pts[i1 + 1], t1));
        out
    }
}
