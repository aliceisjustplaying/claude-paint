//! Stippling study: Friedrich's sky and mist technique [NG p.56; CATS p.127].
//!
//! Top: an evening sky laid in thin with a soft filbert in horizontal
//! strokes, then stippled in two passes (a coarser pass into the wet lay-in
//! aimed at the sky's tones, then, dry, a finer and lighter pass that grows
//! denser toward the glow at the horizon). The strip at the far left is the
//! lay-in alone, for comparison; the next strip has only the first pass.
//! Bottom: a dark ridge, let dry, with mist stippled over its foot, dense in the valley and thinning upward in banks.
//!
//!   cargo paint study_stipple                      1000px
//!   cargo paint study_stipple -- --full            3200px (compare crops)
//!   PAINT_DEBUG=1 ...                              touch counts and timings

use paint::{Mask, Stipple, Tool, Fbm, Style, hex, smoothstep};
use paint::color::{Mix, mix};

const ASPECT: f32 = 1.6;
const HORIZON: f32 = 390.0;
/// Left edges of the "lay-in only" and "first pass only" strips' ends.
const STRIP0: f32 = 120.0;
const STRIP1: f32 = 240.0;

fn main() {
    let run = paintings::run::Run::new("study_stipple");
    let st = Style::friedrich();
    let pal = &st.palette;
    let mut c = st.prepare(run.width, ASPECT, run.seed);
    let f = c.frame();
    let h = c.height();

    // the sky Friedrich might have wanted: cool blue-gray above, through a
    // pale greenish middle to a warm glow at the horizon
    let sky = |_x: f32, y: f32| {
        let t = (y / HORIZON).clamp(0.0, 1.0);
        paint::gradient(&[(0.0, hex("#6f84a8")), (0.45, hex("#a7b4bf")), (0.8, hex("#dcd4b8")), (1.0, hex("#ecd9a8"))], t, Mix::Light)
    };
    let ridge_top = move |x: f32| HORIZON - 40.0 + 30.0 * (x / 180.0).sin() * (x / 410.0).cos() - 25.0 * smoothstep(500.0, 900.0, x);
    let sky_m = Mask::from_fn(f, |_, y| 1.0 - smoothstep(HORIZON + 10.0, HORIZON + 20.0, y));

    // ---- lay-in: thin paint, horizontal strokes, a shade darker and duller
    // than the sky will end
    let lay = st.broad().color(move |x, y| mix(sky(x, y), hex("#6a6f7a"), 0.07, Mix::Light)).angle(|_, _| 0.0).coverage(4.5).medium(0.25).dips(1, 0.6, 0.6);
    c.work(&sky_m, &lay, 11);
    if let Some(b) = st.blend() {
        c.work(&sky_m, &b.angle(|_, _| 0.0), 14);
    }
    if run.stage(&mut c, "lay-in") {
        return;
    }

    // ---- first pass: a stippler of ~1 mm into the wet lay-in, aimed at
    // the sky's own tones; it breaks the strokes and fuses with the wet paint
    let pass1_m = sky_m.clone().mul_fn(|x, _| smoothstep(STRIP0, STRIP0 + 4.0, x));
    let s1 = Stipple::new(Tool::stippler(3.0))
        .mixed(pal, 0.45)
        .color(sky)
        .coverage(|_, _| 2.2)
        .pressure(0.5, 0.85)
        .dips(20, 0.4, 0.5);
    c.stipple(&pass1_m, &s1, 12);
    c.dry();
    if run.stage(&mut c, "pass 1") {
        return;
    }

    // ---- second pass, dry: finer and lighter, denser toward the glow
    let pass2_m = sky_m.clone().mul_fn(|x, _| smoothstep(STRIP1, STRIP1 + 4.0, x));
    let glow = move |x: f32, y: f32| mix(sky(x, y), hex("#f2e2b4"), 0.06 + 0.22 * smoothstep(150.0, HORIZON, y), Mix::Light);
    let s2 = Stipple::new(Tool::stippler(1.6))
        .mixed(pal, 0.5)
        .color(glow)
        .coverage(|_, y| 0.6 + 1.6 * smoothstep(60.0, HORIZON, y))
        .pressure(0.5, 0.85)
        .dips(24, 0.35, 0.6);
    c.stipple(&pass2_m, &s2, 13);
    c.dry();
    if run.stage(&mut c, "pass 2") {
        return;
    }

    // ---- the ridge: dark, cool, laid in body paint
    let ridge_m = Mask::from_fn(f, move |x, y| smoothstep(ridge_top(x) - 1.5, ridge_top(x) + 1.5, y));
    let ridge = st.body().color(|_, y| mix(hex("#2c3336"), hex("#3a4038"), smoothstep(HORIZON, h, y), Mix::Pigment)).angle(|_, _| 0.1).angle_jitter(0.3).length(20.0, 50.0).coverage(5.0);
    c.work(&ridge_m, &ridge, 21);
    c.dry();
    if run.stage(&mut c, "ridge") {
        return;
    }

    // ---- mist over the foot of the ridge: banks thinning upward
    let banks = Fbm::new(31, 4, 220.0);
    let valley = h - 70.0;
    let mist_cov = move |x: f32, y: f32| {
        let up = (valley - y) / 150.0 + 0.35 * banks.get(x * 0.6, y * 2.5);
        (1.0 - smoothstep(0.0, 1.0, up)) * 5.0
    };
    let mist_m = ridge_m.clone().mul_fn(|x, _| smoothstep(STRIP0, STRIP0 + 4.0, x));
    let mist = Stipple::new(Tool::stippler(2.2))
        .mixed(pal, 0.7)
        .color(|_, y| mix(hex("#b7bcc0"), hex("#cfcbbd"), smoothstep(valley - 150.0, valley, y), Mix::Light))
        .coverage(mist_cov)
        .pressure(0.5, 0.85)
        .dips(16, 0.3, 0.7)
        .aim(false);
    c.stipple(&mist_m, &mist, 22);
    c.dry();
    if run.stage(&mut c, "mist") {
        return;
    }

    c.relief(st.relief.0, st.relief.1);
    run.save(&mut c);
}
