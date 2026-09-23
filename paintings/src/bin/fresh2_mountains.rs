//! Daybreak in the Riesengebirge: a wanderer by a granite tor.
//!
//! An original composition in the manner of Caspar David Friedrich (see
//! notes/fresh2_mountains.md for the why). We stand on a dark granite knoll
//! on the Silesian side of the Riesengebirge before sunrise. A wanderer,
//! seen from behind, stands by a weathered tor and looks east across valleys
//! still full of mist to the ranges and the far cone of the Schneekoppe,
//! behind which the sky is growing yellow. Knieholz (dwarf mountain pine)
//! crouches on the knoll; a few wind-thinned spruces stand at its right
//! edge, one of them dead.
//!
//! Order of work (Friedrich's, as far as it is known): primed canvas bought
//! ready, a pencil drawing, a very thin lay-in, then one or two thin layers:
//! skies, mist and far hills stippled; firs hatched; grass flicked upward
//! last; the figure and small details at the end.
//!
//!   cargo paint fresh2_mountains                  1000px
//!   cargo paint fresh2_mountains -- --full        3200px

use paint::color::{Mix, luminance, mix};
use paint::form::{Light, Sample, aerial};
use paint::{Fbm, Form, Gesture, Held, Mask, Orient, Rgb, Ridge, Rng, Sdf, Stipple, Style, Tool, Touch, gradient, hex, smoothstep};
use paintings::run::{Finish, Run};

const ASPECT: f32 = 1.45;
/// Where the sun will rise: behind the far range, a little right of center.
const SUN: (f32, f32) = (655.0, 318.0);

fn mix1(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn main() {
    let o = Run::new("fresh2_mountains");
    let st = Style::friedrich();
    let pal = &st.palette;
    // for the ranges and the mist, a family set out on its own: whites,
    // blues, ochre, umber, black; no vermilion or red earth to fleck them
    let cool = st.palette.only(&["lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "bone black"]);
    let cool = &cool;
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let (w, h) = (c.width(), c.height());

    // ------------------------------------------------------------ geometry
    // crests, far to near (y of the skyline at x)
    let n_far = Fbm::new(11, 4, 160.0);
    let n_far = &n_far;
    let n_far2 = Fbm::new(12, 3, 30.0);
    let n_far2 = &n_far2;
    let far_crest = f.per_column(move |x: f32| {
        // a long, low range; the Schneekoppe a blunt cone right of center
        let cone = 62.0 * (1.0 - ((x - 668.0) / 95.0).powi(2)).max(0.0).powf(1.3);
        let shoulder = 20.0 * (1.0 - ((x - 560.0) / 150.0).powi(2)).max(0.0);
        318.0 - cone - shoulder + 7.0 * n_far.get(x, 0.0) + 1.5 * n_far2.get(x, 3.0) + 10.0 * smoothstep(850.0, 1000.0, x) - 12.0 * smoothstep(300.0, 0.0, x)
    });
    let far_crest = &far_crest;
    let n_mid = Fbm::new(21, 4, 200.0);
    let n_mid = &n_mid;
    let n_mid2 = Fbm::new(22, 3, 25.0);
    let n_mid2 = &n_mid2;
    let mid_crest = f.per_column(move |x: f32| {
        // a range that rises to the left and falls away to the right
        372.0 - 38.0 * smoothstep(620.0, 90.0, x) + 16.0 * n_mid.get(x, 1.0) + 2.5 * n_mid2.get(x, 1.0) + 14.0 * smoothstep(700.0, 1000.0, x)
    });
    let mid_crest = &mid_crest;
    let n_near = Fbm::new(31, 4, 150.0);
    let n_near = &n_near;
    let n_near2 = Fbm::new(32, 3, 18.0);
    let n_near2 = &n_near2;
    let near_crest = f.per_column(move |x: f32| {
        // a wooded foothill ridge, higher at the right
        432.0 - 30.0 * smoothstep(420.0, 900.0, x) + 12.0 * n_near.get(x, 2.0) + 2.0 * n_near2.get(x, 2.0) + 10.0 * smoothstep(250.0, 0.0, x)
    });
    let near_crest = &near_crest;
    // the knoll we stand on: rising from the lower left, a crown at x≈300,
    // falling away to the right into the valley
    let n_kn = Fbm::new(41, 4, 120.0);
    let n_kn = &n_kn;
    let n_kn2 = Fbm::new(42, 3, 14.0);
    let n_kn2 = &n_kn2;
    let knoll = f.per_column(move |x: f32| {
        let crown = 505.0 - 40.0 * (-((x - 290.0) / 230.0).powi(2)).exp();
        let fall = 150.0 * smoothstep(420.0, 820.0, x).powf(1.2);
        (crown + fall + 7.0 * n_kn.get(x, 5.0) + 1.8 * n_kn2.get(x, 5.0)).min(h + 5.0)
    });
    let knoll = &knoll;

    let sky_m = Mask::from_fn(f, |x, y| 1.0 - smoothstep(far_crest(x) + 4.0, far_crest(x) + 12.0, y));
    let knoll_m = Mask::from_fn(f, |x, y| smoothstep(knoll(x) - 0.8, knoll(x) + 0.8, y));
    // paint nothing pale where the dark knoll will go: whatever lies under
    // it shows in the gaps of its body color (a few units of overlap at the
    // edge so no light line opens along it)
    let off_knoll = move |x: f32, y: f32| 1.0 - smoothstep(knoll(x) + 3.0, knoll(x) + 6.0, y);
    let off_knoll = &off_knoll;

    // the tor: a pile of weathered granite blocks on the crown of the knoll
    let tor_base = (222.0, knoll(222.0) + 2.0);

    // the figure's feet, right of the tor, on the crown
    let fig_x = 336.0;
    let fig = (fig_x, knoll(fig_x) + 1.5);
    let fig_h = 58.0;

    // ------------------------------------------------------------- colors
    let sky = move |x: f32, y: f32| {
        let t = (y / (SUN.1 + 10.0)).clamp(0.0, 1.0);
        let base = gradient(
            &[(0.0, hex("#6f7f99")), (0.3, hex("#8f9cad")), (0.55, hex("#b8b6b6")), (0.72, hex("#d6c3b3")), (0.86, hex("#e7d3ae")), (1.0, hex("#efdca8"))],
            t,
            Mix::Light,
        );
        let d = ((x - SUN.0) / 330.0).powi(2) + ((y - SUN.1) / 120.0).powi(2);
        mix(base, hex("#f4e3b0"), 0.55 * (-d).exp(), Mix::Light)
    };
    let sky = &sky;
    // the valley air at dawn: cool and pale, warmer toward the sun
    let air = move |x: f32, y: f32| {
        let warm = (-((x - SUN.0) / 300.0).powi(2)).exp() * (1.0 - smoothstep(330.0, 480.0, y));
        mix(hex("#bdbcc0"), hex("#e2d4bb"), 0.8 * warm, Mix::Light)
    };
    let air = &air;

    let billow = Fbm::new(75, 4, 60.0);
    let billow = &billow;
    let sea_top = move |x: f32| near_crest(x) + 44.0 + 22.0 * smoothstep(900.0, 450.0, x) + 14.0 * billow.get(x, 0.0);
    let sea_top = &sea_top;

    // ---------------------------------------------------------------- stages
    if o.stage("drawing", &mut c, &mut rng) {
        // the pencil drawing: range lines, the knoll, the tor and the figure,
        // in a faint first pass (graphite was his, and lines "still partly
        // shimmer through" the thin paint [CATS p.132])
        let lead = pal.paint(hex("#4a4744"), 0.55).with_hiding(0.35);
        let mut pencil = Held::new(Tool { width: 0.55, ..Tool::round_sable(0.55) }, 7);
        let mut line = |c: &mut paint::Canvas, crest: &dyn Fn(f32) -> f32, x0: f32, x1: f32, rng: &mut Rng| {
            let mut x = x0;
            while x < x1 {
                // the pencil is lifted every so often
                let len = rng.range(40.0, 110.0);
                let pts: Vec<(f32, f32)> = (0..=12).map(|i| {
                    let xx = x + len * i as f32 / 12.0;
                    (xx, crest(xx) + rng.normal() * 0.25)
                }).collect();
                pencil.load(lead, 0.5);
                c.drag(&mut pencil, &Gesture::new(pts).pressure(0.5, 0.35).ramps(0.1, 0.2).shake(0.6), None);
                x += len + rng.range(-4.0, 6.0);
            }
        };
        line(&mut c, &|x| far_crest(x), 0.0, w, &mut rng);
        line(&mut c, &|x| mid_crest(x), 0.0, w, &mut rng);
        line(&mut c, &|x| near_crest(x), 0.0, w, &mut rng);
        line(&mut c, &|x| knoll(x), 0.0, w, &mut rng);
        // the tor and the figure: a few marks
        let (tx, ty) = tor_base;
        for (a, b) in [((tx - 42.0, ty), (tx - 36.0, ty - 38.0)), ((tx - 36.0, ty - 38.0), (tx + 10.0, ty - 58.0)), ((tx + 10.0, ty - 58.0), (tx + 44.0, ty - 30.0)), ((tx + 44.0, ty - 30.0), (tx + 50.0, ty))] {
            pencil.load(lead, 0.5);
            c.drag(&mut pencil, &Gesture::line(a, b).pressure(0.5, 0.4).shake(0.8), None);
        }
        pencil.load(lead, 0.5);
        c.drag(&mut pencil, &Gesture::line(fig, (fig.0 + 0.5, fig.1 - fig_h)).pressure(0.5, 0.4), None);
        c.dry();
    }

    if o.stage("underpainting", &mut c, &mut rng) {
        // "a very thin underpainting" [CATS p.127]: the dark lower half is
        // washed in thin umber and blue-gray first, so the pale ground never
        // flashes through the body color later
        let low = Mask::from_fn(f, |x, y| smoothstep(near_crest(x) + 2.0, near_crest(x) + 6.0, y));
        let hd = st
            .broad()
            .color(move |x, y| if y > knoll(x) + 3.0 { hex("#3a2f25") } else { mix(hex("#4a4d56"), hex("#6d6d72"), smoothstep(near_crest(x) + 40.0, near_crest(x) + 140.0, y), Mix::Pigment) })
            .by_masstone()
            .angle(move |x, _| (knoll(x + 4.0) - knoll(x - 4.0)).atan2(8.0) * 0.6)
            .angle_jitter(0.1)
            .length(50.0, 140.0)
            .coverage(3.0)
            .medium(0.55)
            .clip(true)
            .threshold(0.3);
        c.work(&low, &hd, 91);
        // and a cool violet-gray under the far and middle ranges, so no warm
        // ground specks through their stipple
        let ranges = Mask::from_fn(f, |x, y| smoothstep(far_crest(x) + 3.0, far_crest(x) + 8.0, y) * (1.0 - smoothstep(near_crest(x) + 2.0, near_crest(x) + 6.0, y)));
        let hd = st.broad().palette(cool).color(move |x, y| mix(hex("#9894a2"), hex("#77798a"), smoothstep(far_crest(x), near_crest(x), y), Mix::Pigment)).by_masstone().angle(|_, _| 0.0).angle_jitter(0.05).length(50.0, 140.0).coverage(4.5).medium(0.55).clip(true).threshold(0.3);
        c.work(&ranges, &hd, 92);
        c.dry();
    }

    if o.stage("sky", &mut c, &mut rng) {
        // a thin lay-in, long level arcs from the elbow, fused top to bottom
        let lay = st
            .broad()
            .color(move |x, y| mix(sky(x, y), hex("#7a7672"), 0.05, Mix::Light))
            .angle(|_, _| 0.0)
            .angle_jitter(0.03)
            .curve(0.04, 0.3)
            .coverage(4.0)
            .medium(0.3)
            .dips(1, 0.6, 0.6)
            .clip(true);
        c.work(&sky_m, &lay, 101);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.clip(true).angle(|_, _| 0.0), 102);
        }
        // first stipple into the wet lay-in: it breaks the strokes
        let s1 = Stipple::new(Tool::stippler(3.0)).mixed(pal, 0.45).color(sky).coverage(|_, _| 2.0).pressure(0.5, 0.85).dips(20, 0.4, 0.5);
        c.stipple(&sky_m, &s1, 103);
        c.dry();
    }

    // the clouds: a long, thin bank of stratus lying across the dawn, and
    // wisps above; their undersides catch the light
    let cl = Fbm::new(51, 4, 180.0);
    let cl = &cl;
    let cl2 = Fbm::new(52, 3, 40.0);
    let cl2 = &cl2;
    let cloud = move |x: f32, y: f32| {
        let band = |yc: f32, th: f32| (-((y - yc - 10.0 * cl.get(x * 0.4, 7.0)) / th).powi(2)).exp();
        let a = band(236.0, 7.0) * smoothstep(0.35, 0.65, cl.get01(x * 0.35, 1.0)) * smoothstep(120.0, 300.0, x);
        let b = band(198.0, 5.0) * smoothstep(0.5, 0.75, cl.get01(x * 0.3 + 300.0, 2.0)) * 0.8;
        let c2 = band(132.0, 9.0) * smoothstep(0.55, 0.8, cl.get01(x * 0.25 + 900.0, 3.0)) * 0.6;
        ((a + b + c2) * (0.7 + 0.5 * cl2.get01(x * 0.5, y * 2.0))).clamp(0.0, 1.0)
    };
    let cloud = &cloud;
    if o.stage("sky light", &mut c, &mut rng) {
        // second pass, dry: finer and paler, denser toward the glow
        let glow = move |x: f32, y: f32| mix(sky(x, y), hex("#f6e6b8"), 0.05 + 0.25 * smoothstep(160.0, SUN.1, y), Mix::Light);
        let s2 = Stipple::new(Tool::stippler(1.7)).mixed(pal, 0.5).color(glow).coverage(|_, y| 0.5 + 1.8 * smoothstep(60.0, SUN.1, y)).pressure(0.5, 0.85).dips(24, 0.35, 0.6);
        c.stipple(&sky_m, &s2, 111);
        c.dry();
        // the clouds, stippled: violet-gray bodies, rose-gold lower edges
        let cm = Mask::from_fn(f, |x, y| smoothstep(0.08, 0.3, cloud(x, y)));
        let ccol = move |x: f32, y: f32| {
            let under = smoothstep(-2.0, 5.0, y - 236.0 - 10.0 * cl.get(x * 0.4, 7.0)) + smoothstep(-2.0, 4.0, y - 198.0 - 10.0 * cl.get(x * 0.4, 7.0)) * 0.6;
            let warm = (-((x - SUN.0) / 280.0).powi(2)).exp();
            let body = mix(hex("#9d97a3"), hex("#b7a4a4"), warm, Mix::Light);
            mix(body, hex("#ebc9a8"), (under * (0.4 + 0.5 * warm)).min(0.85), Mix::Light)
        };
        // laid in first with a soft brush, thin, in long level strokes, and
        // fused, so the stipple softens a body instead of being the body
        let cm_soft = Mask::from_fn(f, |x, y| cloud(x, y));
        let lay = st.glaze(0.6).color(ccol).aim(0.6).angle(|_, _| 0.0).angle_jitter(0.02).length(40.0, 120.0).coverage(2.5).tool_width(9.0).load_at(|x, y| cloud(x, y)).clip(true).threshold(0.12);
        c.work(&cm_soft, &lay, 113);
        if let Some(b) = st.blend() {
            c.work(&cm_soft, &b.clip(true).angle(|_, _| 0.0).pressure(0.25, 0.35).length(40.0, 120.0).threshold(0.05), 114);
        }
        c.dry();
        let s3 = Stipple::new(Tool::stippler(2.4)).mixed(pal, 0.55).color(ccol).coverage(move |x, y| 1.6 * cloud(x, y)).pressure(0.45, 0.8).drag(0.9, Some(0.0)).dips(18, 0.35, 0.6);
        c.stipple(&cm, &s3, 112);
        c.dry();
    }

    // ------------------------------------------------------------- ranges
    let mut form = Form::new(f);
    let far = Ridge::new(0.0, w, |x| far_crest(x), 140.0, 61).lean(1.1, 0.8).gullies(14.0, 0.45).fan(1.0).z0(-600.0);
    let mid = Ridge::new(0.0, w, |x| mid_crest(x), 150.0, 62).lean(0.95, 0.8).gullies(30.0, 0.4).fan(1.0).z0(-300.0);
    let near = Ridge::new(0.0, w, |x| near_crest(x), 280.0, 63).lean(0.8, 0.8).gullies(34.0, 0.5).fan(1.0).z0(-100.0);
    let id_far = form.add_at(&far, &|_, _, _| 4.0);
    let id_mid = form.add_at(&mid, &|_, _, _| 2.4);
    let id_near = form.add_at(&near, &|_, _, _| 1.3);
    // dawn light from behind the ranges, up and to the right: the faces
    // turned to us are in shadow, the slopes that face the sky pick up a
    // little of it
    form.light(Light::new((0.55, -0.6), -0.35).ambient(0.35).penumbra(0.1).across_parts(false));
    let form = &form;
    let range_col = move |x: f32, y: f32, s: &Sample, base: Rgb, lit: Rgb, vis: f32| {
        let v = s.shade.value.clamp(0.0, 1.0);
        let col = mix(base, lit, smoothstep(0.2, 0.75, v), Mix::Pigment);
        mix(col, air(x, y), aerial(s.dist, vis), Mix::Light)
    };
    let range_col = &range_col;

    if o.stage("far range", &mut c, &mut rng) {
        // the far range and the Schneekoppe: laid in thin, then stippled,
        // barely darker than the sky
        let sil = form.silhouette(&[id_far], |_| 1.2);
        let fcol = move |x: f32, y: f32| match form.sample(x, y).filter(|s| s.part == id_far) {
            Some(s) => range_col(x, y, &s, hex("#6f6c86"), hex("#a8a0ad"), 9.0),
            None => mix(hex("#a39eae"), air(x, y), 0.2, Mix::Light),
        };
        let fcol = &fcol;
        let hd = st.body().palette(cool).color(fcol).angle(|x, y| form.fall(x, y)).angle_jitter(0.15).length(8.0, 26.0).coverage(3.0).medium(0.4).tool_width(5.0).clip(true).threshold(0.2);
        c.work(&sil, &hd, 201);
        let sp = Stipple::new(Tool::stippler(2.2)).mixed(cool, 0.5).color(fcol).coverage(|_, _| 2.6).pressure(0.45, 0.8).dips(18, 0.4, 0.5).clip(true);
        c.stipple(&sil, &sp, 202);
        c.dry();
    }
    let _ = &mut rng;
    // the mist lying in the valleys: where it lies thickest, at the foot of
    // each range, thinning upward in banks
    let banks = Fbm::new(71, 4, 170.0);
    let banks = &banks;
    let mist_at = move |x: f32, y: f32, foot: f32, reach: f32| {
        let up = (foot - y) / reach + 0.35 * banks.get(x * 0.5, y * 2.2);
        1.0 - smoothstep(0.0, 1.0, up)
    };
    let mist_at = &mist_at;
    if o.stage("far mist", &mut c, &mut rng) {
        let foot = |x: f32| mid_crest(x) + 4.0;
        let mm = Mask::from_fn(f, move |x, y| if y < foot(x) + 10.0 && y > far_crest(x) - 30.0 { 1.0 } else { 0.0 });
        let m = Stipple::new(Tool::stippler(2.4)).mixed(cool, 0.7).color(move |x, y| mix(air(x, y), hex("#efe3c8"), 0.3, Mix::Light)).coverage(move |x, y| 2.8 * mist_at(x, y, foot(x), 38.0)).pressure(0.5, 0.85).dips(16, 0.3, 0.7).aim(false);
        c.stipple(&mm, &m, 211);
        c.dry();
    }

    if o.stage("mid range", &mut c, &mut rng) {
        let sil = form.silhouette(&[id_mid], |_| 0.8);
        let mcol = move |x: f32, y: f32| match form.sample(x, y).filter(|s| s.part == id_mid) {
            Some(s) => {
                // spurs catch the sky, gullies stay in shadow
                let b = form.bend(x, y, 2.0);
                let base = range_col(x, y, &s, hex("#394058"), hex("#62667c"), 8.0);
                let base = mix(base, hex("#3c4158"), smoothstep(0.05, 0.3, -b) * 0.5, Mix::Pigment);
                mix(base, hex("#8a8a9c"), smoothstep(0.05, 0.3, b) * 0.35, Mix::Pigment)
            }
            None => hex("#6c6e84"),
        };
        let mcol = &mcol;
        let hd = st.body().palette(cool).color(mcol).angle(|x, y| form.fall(x, y)).angle_jitter(0.12).length(8.0, 28.0).coverage(3.0).medium(0.35).tool_width(5.0).clip(true).threshold(0.2);
        c.work(&sil, &hd, 221);
        let sp = Stipple::new(Tool::stippler(2.0)).mixed(cool, 0.45).color(mcol).coverage(|_, _| 2.4).pressure(0.45, 0.8).dips(18, 0.4, 0.5).clip(true);
        c.stipple(&sil, &sp, 222);
        c.dry();
        // mist at its foot, rising in front of the near range's crest
        let foot = |x: f32| near_crest(x) + 6.0;
        let mm = Mask::from_fn(f, move |x, y| if y < foot(x) + 10.0 && y > mid_crest(x) - 20.0 { 1.0 } else { 0.0 });
        let m = Stipple::new(Tool::stippler(2.4)).mixed(cool, 0.7).color(move |x, y| mix(air(x, y), hex("#ece2cc"), 0.25, Mix::Light)).coverage(move |x, y| 2.4 * mist_at(x, y, foot(x), 26.0)).pressure(0.5, 0.85).dips(16, 0.3, 0.7).aim(false);
        c.stipple(&mm, &m, 223);
        if let Some(b) = st.blend() {
            c.work(&mm, &b.clip(true).angle(|_, _| 0.0).pressure(0.22, 0.3).coverage(2.0).length(50.0, 140.0), 224);
        }
        c.dry();
    }

    // forest on the near range: dark spruce woods over most of it, broken by
    // clearings; the woods' edge against the mist is a serrated line of tips
    let woods = Fbm::new(81, 4, 70.0);
    let woods = &woods;
    if o.stage("near range", &mut c, &mut rng) {
        // (not under the knoll: its gray showed in the gaps of the knoll's
        // body color as pale flakes)
        let sil = form.silhouette(&[id_near], |_| 0.5).mul_fn(|x, y| off_knoll(x, y));
        let forest = move |x: f32, y: f32| smoothstep(0.38, 0.5, woods.get01(x, y * 1.4) + 0.15 * smoothstep(0.0, 40.0, y - near_crest(x)));
        let forest = &forest;
        let ncol = move |x: f32, y: f32| match form.sample(x, y).filter(|s| s.part == id_near) {
            Some(s) => {
                let rock = range_col(x, y, &s, hex("#3f4655"), hex("#6f7275"), 4.0);
                let wood = mix(hex("#2b3438"), air(x, y), aerial(s.dist, 4.0) * 0.9, Mix::Light);
                mix(rock, wood, forest(x, y), Mix::Pigment)
            }
            None => hex("#4a5058"),
        };
        let ncol = &ncol;
        let hd = st.body().palette(cool).color(ncol).angle(|x, y| form.fall(x, y)).angle_jitter(0.15).length(7.0, 22.0).coverage(3.2).medium(0.3).tool_width(4.5).clip(true).threshold(0.2);
        c.work(&sil, &hd, 231);
        // the woods: short hatching, upright, "like a closely woven textile"
        let wm = Mask::from_fn(f, move |x, y| forest(x, y) * (1.0 - smoothstep(near_crest(x) + 14.0, near_crest(x) + 26.0, y))).mul(&sil);
        let hatch = st.hatch().color(move |x, y| mix(ncol(x, y), hex("#1f2528"), 0.1, Mix::Pigment)).angle(|_, _| -1.5).cross(0.15).length(2.5, 6.0).coverage(2.6).tool_width(1.3).clip(true);
        c.work(&wm, &hatch, 232);
        c.dry();
        // spruce tips along the crest where the wood reaches it
        let mut r = Rng::new(233);
        let mut b = Held::new(Tool::rigger(0.25), 234);
        let mut x = 0.0;
        while x < w {
            // spruces stand in groups, with gaps
            x += if r.chance(0.15) { r.range(4.0, 12.0) } else { r.range(1.2, 3.0) };
            let yc = near_crest(x);
            if forest(x, yc + 2.0) < 0.5 {
                continue;
            }
            let tall = r.range(2.5, 8.0) * if r.chance(0.1) { 1.5 } else { 1.0 };
            let col = mix(ncol(x, yc + 3.0), hex("#262d31"), 0.55, Mix::Pigment);
            b.load(pal.paint(col, 0.2), 0.5);
            // a narrow cone: a hair-thin stem lifted to a point, and short
            // level boughs, longer toward the foot
            let g = Gesture::new(vec![(x, yc + 2.0), (x + r.normal() * 0.1, yc - tall * 0.6), (x + r.normal() * 0.1, yc - tall)]).pressure(0.5, 0.0).ramps(0.0, 0.9).shake(0.2);
            c.drag(&mut b, &g, None);
            let tiers = 3 + (tall / 2.0) as usize;
            for k in 0..tiers {
                let v = (k as f32 + 0.5) / tiers as f32;
                let yy = yc + 1.0 - v * tall * 0.85;
                let reach = tall * 0.22 * (1.0 - v) + 0.25;
                c.drag(&mut b, &Gesture::new(vec![(x - reach, yy + reach * 0.3), (x, yy - 0.1), (x + reach, yy + reach * 0.3)]).pressure(0.35, 0.35).ramps(0.2, 0.2).shake(0.2), None);
            }
        }
        c.dry();
        // mist at the foot of the near range, coming up out of the valley
        // between it and the knoll
        let foot = |x: f32| (knoll(x) + 20.0).min(h);
        // a veil brushed level, thin, its density carried by the load, and
        // fused: stippled, it read as static over the dark hatching
        let dens = move |x: f32, y: f32| mist_at(x, y, foot(x).min(near_crest(x) + 95.0), 55.0);
        let mm = Mask::from_fn(f, move |x, y| if y > near_crest(x) - 10.0 { off_knoll(x, y) * smoothstep(0.02, 0.1, dens(x, y)) } else { 0.0 });
        let hd = st.broad().color(move |x, y| mix(air(x, y), hex("#e9dfca"), 0.2, Mix::Light)).by_masstone().angle(|_, _| 0.0).angle_jitter(0.03).length(50.0, 150.0).coverage(3.0).medium(0.6).load_at(dens).clip(true).threshold(0.05).tool_width(12.0);
        c.work(&mm, &hd, 235);
        if let Some(b) = st.blend() {
            c.work(&mm, &b.clip(true).angle(|_, _| 0.0).pressure(0.25, 0.35).coverage(2.5).length(50.0, 140.0), 236);
        }
        c.dry();
    }
    
    // the sea of mist filling the valley below the near range, right of the
    // knoll: bright on top where the dawn reaches it, its upper edge
    // breaking into banks against the dark woods
    let sea = move |x: f32, y: f32| {
        // dense at its top surface, where the dawn lights it; thinner and
        // streaked lower down, where the woods show through
        let body = mist_at(x, y, sea_top(x) + 26.0, 34.0);
        let streak = billow.get01(x * 0.35, y * 3.0);
        let thin = 0.55 + 0.45 * smoothstep(0.35, 0.7, streak);
        body * mix1(1.0, thin, smoothstep(sea_top(x) + 30.0, sea_top(x) + 90.0, y))
    };
    let sea = &sea;
    let sea_col = move |x: f32, y: f32| {
        let warm = (-((x - SUN.0) / 260.0).powi(2)).exp();
        let top = mix(hex("#c9c7c8"), hex("#ebdfc6"), warm, Mix::Light);
        // billows: the rounded tops catch the dawn, the hollows between
        // them are cool and gray
        let b = billow.get01(x * 0.7, (y - sea_top(x)) * 2.4 + 50.0);
        let top = mix(top, mix(hex("#a7a8b2"), top, 0.35, Mix::Light), smoothstep(0.45, 0.75, b) * smoothstep(sea_top(x) - 4.0, sea_top(x) + 20.0, y), Mix::Light);
        // lower down, toward us, the mist lies in the knoll's shadow
        mix(top, hex("#9c9fa8"), 0.6 * smoothstep(sea_top(x) + 40.0, h, y), Mix::Light)
    };
    let sea_col = &sea_col;
    if o.stage("valley mist", &mut c, &mut rng) {
        let sm = Mask::from_fn(f, move |x, y| if sea(x, y) > 0.04 { off_knoll(x, y) } else { 0.0 });
        // laid in with long level strokes, thin, and fused
        let hd = st.broad().color(sea_col).angle(|_, _| 0.0).angle_jitter(0.04).curve(0.03, 0.3).length(60.0, 180.0).coverage(3.0).medium(0.5).load_at(move |x, y| sea(x, y)).dips(1, 0.5, 0.6).clip(true).threshold(0.5);
        c.work(&sm, &hd, 251);
        if let Some(b) = st.blend() {
            c.work(&sm, &b.clip(true).angle(|_, _| 0.0).pressure(0.3, 0.4), 252);
        }
        c.dry();
        // then stippled, the banks built by density at the upper edge
        let m = Stipple::new(Tool::stippler(2.6)).mixed(cool, 0.6).color(sea_col).coverage(move |x, y| 2.4 * sea(x, y)).pressure(0.5, 0.85).dips(18, 0.35, 0.6);
        c.stipple(&sm, &m, 253);
        c.dry();
    }

    // -------------------------------------------------------------- knoll
    let tuft = Fbm::new(91, 4, 40.0);
    let tuft = &tuft;
    let knoll_col = move |x: f32, y: f32| {
        // contre-jour: the knoll is dark; the ground turned up to the sky a
        // little lighter near the crest; heather browns and grass olives
        let near_top = 1.0 - smoothstep(0.0, 45.0, y - knoll(x));
        let base = mix(hex("#221f1a"), hex("#35301f"), tuft.get01(x, y * 2.0), Mix::Pigment);
        let base = mix(base, hex("#4a4230"), 0.55 * near_top, Mix::Pigment);
        mix(base, hex("#171512"), smoothstep(540.0, h, y) * 0.6, Mix::Pigment)
    };
    let knoll_col = &knoll_col;
    if o.stage("knoll", &mut c, &mut rng) {
        // body color in strokes that follow the swell of the ground
        let slope = move |x: f32| (knoll(x + 3.0) - knoll(x - 3.0)).atan2(6.0);
        let hd = st.body().color(knoll_col).angle(move |x, y| slope(x) * (1.0 - smoothstep(0.0, 120.0, y - knoll(x)) * 0.6)).angle_jitter(0.25).length(14.0, 45.0).coverage(3.5).medium(0.25).clip(true).threshold(0.2);
        c.work(&knoll_m, &hd, 301);
        c.dry();
    }

    if o.stage("heather", &mut c, &mut rng) {
        // rust-brown heather and bilberry in patches, hatched upright; and
        // pale dead grass on the crest where the sky light catches it
        let hp = Fbm::new(97, 4, 26.0);
        let hp = &hp;
        let hp2 = Fbm::new(98, 3, 5.0);
        let hp2 = &hp2;
        let hm = Mask::from_fn(f, move |x, y| smoothstep(0.62, 0.68, hp.get01(x, y * 1.8) + 0.2 * (hp2.get01(x, y * 1.5) - 0.5)) * smoothstep(4.0, 14.0, y - knoll(x))).mul(&knoll_m);
        let hcol = move |x: f32, y: f32| {
            let near_top = 1.0 - smoothstep(0.0, 60.0, y - knoll(x));
            mix(mix(hex("#33281f"), hex("#433126"), hp.get01(x * 3.0, y * 3.0), Mix::Pigment), hex("#1d1813"), 0.5 * (1.0 - near_top), Mix::Pigment)
        };
        let hd = st.hatch().color(hcol).angle(|_, _| -1.45).cross(0.3).length(2.5, 6.0).coverage(1.6).tool_width(1.6).clip(true);
        c.work(&hm, &hd, 305);
        let crest = Mask::from_fn(f, move |x, y| smoothstep(-1.0, 1.0, y - knoll(x)) * (1.0 - smoothstep(2.0, 9.0 + 5.0 * hp.get01(x, 0.0), y - knoll(x))) * smoothstep(560.0, 430.0, x));
        let hd = st.hatch().color(move |x, _| mix(hex("#4a4331"), hex("#5c5339"), smoothstep(200.0, 450.0, x), Mix::Pigment)).angle(|_, _| -1.5).cross(0.2).length(2.0, 5.0).coverage(1.3).tool_width(1.1).clip(true);
        c.work(&crest, &hd, 306);
        c.dry();
        // stones lying in the grass: granite, a dark underside and a pale
        // upper edge, each a couple of touches of a small brush
        let mut r = Rng::new(307);
        let mut b = Held::new(Tool::round_sable(2.2), 308);
        let mut t = Held::new(Tool::round_sable(1.2), 309);
        for _ in 0..22 {
            let x = r.range(20.0, 640.0);
            let d = r.range(6.0, 1.0).powi(2) * 6.0 + 4.0;
            let y = knoll(x) + d;
            if y > h - 5.0 || (x - tor_base.0).abs() < 70.0 && d < 20.0 {
                continue;
            }
            let sz = r.range(1.5, 4.5) * (1.0 + y / h);
            b.load(pal.paint(hex("#1b1917"), 0.2), 0.6);
            c.drag(&mut b, &Gesture::new(vec![(x - sz, y), (x, y + sz * 0.2), (x + sz, y)]).pressure(0.8, 0.7).shake(0.4), None);
            t.load(pal.paint(hex("#433f37"), 0.2), 0.35);
            c.drag(&mut t, &Gesture::new(vec![(x - sz * 0.8, y - sz * 0.3), (x - sz * 0.1, y - sz * 0.55), (x + sz * 0.5, y - sz * 0.35)]).pressure(0.6, 0.2).ramps(0.1, 0.5).shake(0.4), None);
        }
        c.dry();
    }

    if o.stage("tor", &mut c, &mut rng) {
        let (bx, by) = tor_base;
        paint_granite(&mut c, &st, &tor(bx, by), 0.3, 1.0, 1.0, &|_| by, knoll_m.clone(), knoll_col, 311);
        // the outcrop in the near foreground, rising out of the turf
        let seat = move |x: f32| h - 14.0 + 10.0 * (x / 60.0).sin() + 0.08 * x;
        paint_granite(&mut c, &st, &outcrop(h), 0.05, 2.2, 0.0, &seat, knoll_m.clone(), knoll_col, 312);
    }

    // knieholz: low dark mats of dwarf pine on the knoll's shoulders
    let pine = Fbm::new(95, 4, 30.0);
    let pine2 = Fbm::new(96, 3, 5.0);
    let pine2 = &pine2;
    let pine = &pine;
    let kn_cover = move |x: f32, y: f32| {
        let d = y - knoll(x);
        let band = smoothstep(2.0, 10.0, d) * (1.0 - smoothstep(60.0, 140.0, d));
        let away = smoothstep(30.0, 60.0, (x - tor_base.0).abs()) * smoothstep(10.0, 28.0, (x - fig.0).abs());
        // ragged at the edges: the mats are made of single bushes
        let rag = 0.22 * (pine2.get01(x, y * 1.5) - 0.5);
        band * away * smoothstep(0.55, 0.62, pine.get01(x, y * 1.8) + rag)
    };
    let kn_cover = &kn_cover;
    if o.stage("knieholz", &mut c, &mut rng) {
        let km = Mask::from_fn(f, move |x, y| smoothstep(0.1, 0.3, kn_cover(x, y))).mul(&knoll_m);
        let kcol = move |x: f32, y: f32| {
            let top = smoothstep(0.3, 0.7, pine.get01(x, y * 1.8 - 3.0));
            mix(hex("#1d2219"), hex("#3a4128"), 0.6 * (1.0 - top) * (1.0 - smoothstep(0.0, 120.0, y - knoll(x))), Mix::Pigment)
        };
        let kcol = &kcol;
        let hatch = st.hatch().color(kcol).angle(|_, _| -0.5).cross(0.6).length(2.0, 5.0).coverage(2.2).tool_width(1.5).clip(true);
        c.work(&km, &hatch, 321);
        c.dry();
    }

    if o.stage("spruces", &mut c, &mut rng) {
        // wind-thinned spruces on the right shoulder of the knoll, lower down
        let mut r = Rng::new(331);
        let trees = [(468.0, 72.0, false), (489.0, 104.0, false), (503.0, 58.0, true), (521.0, 86.0, false), (556.0, 48.0, false)];
        for (i, &(x, ht, dead)) in trees.iter().enumerate() {
            let base = (x, knoll(x) + 3.0);
            spruce(&mut c, pal, base, ht, dead, &mut r, 340 + i as u64);
        }
    }

    if o.stage("slope trees", &mut c, &mut rng) {
        // lower down the knoll's right flank, smaller spruces going down
        // toward the mist
        let mut r = Rng::new(401);
        let mut x = 585.0;
        let mut i = 0;
        while x < 760.0 {
            let base = (x, knoll(x) + r.range(8.0, 40.0));
            let ht = r.range(26.0, 52.0) * (1.0 - 0.3 * smoothstep(600.0, 760.0, x));
            spruce(&mut c, pal, base, ht, r.chance(0.12), &mut r, 410 + i);
            x += r.range(9.0, 26.0);
            i += 1;
        }
        // (I tried mist lapping up over the flank here, stippled and then
        // brushed; both turned the dark flank into mottled lichen. The dark
        // knoll stays crisp against the fog.)
        c.dry();
    }

    if o.stage("figure", &mut c, &mut rng) {
        wanderer(&mut c, pal, fig, fig_h, 351);
    }

    if o.stage("grass", &mut c, &mut rng) {
        // "fine upturning" strokes laid over the finished ground, last
        let mut r = Rng::new(361);
        let mut b = Held::new(Tool::rigger(0.5), 362);
        let mut n = 0;
        let mut x = 0.0;
        while x < w {
            x += r.range(0.6, 2.2);
            let y0 = knoll(x);
            // grass on the crest line against the sky/mist, and in the body
            for _ in 0..3 {
                let d = r.range(-1.0, 1.0).powi(2) * 120.0;
                let y = y0 + d + r.range(0.0, 3.0);
                if y > h || kn_cover(x, y) > 0.2 || (x - tor_base.0).abs() < 45.0 && y < tor_base.1 + 4.0 {
                    continue;
                }
                let near_top = 1.0 - smoothstep(0.0, 40.0, y - y0);
                let len = r.range(3.0, 8.0) * (1.0 + 0.8 * smoothstep(0.0, 180.0, y - y0));
                let lean = r.normal() * 0.35 + 0.25;
                let col = if r.chance(0.35 * near_top * near_top) { hex("#8a7d58") } else if r.chance(0.5) { mix(mix(hex("#302c20"), hex("#4a4430"), r.range(0.0, 1.0), Mix::Pigment), hex("#5a5238"), near_top, Mix::Pigment) } else { hex("#15130f") };
                if n % 6 == 0 {
                    b.load(pal.paint(col, 0.15), 0.5);
                } else {
                    b.reload(pal.paint(col, 0.15), 0.3);
                }
                n += 1;
                let tip = (x + lean.sin() * len, y - lean.cos() * len);
                let midp = (x + lean.sin() * len * 0.45 + r.normal() * 0.3, y - lean.cos() * len * 0.5);
                c.drag(&mut b, &Gesture::new(vec![(x, y), midp, tip]).pressure(0.7, 0.0).swell(vec![0.4, 1.0, 0.7]).ramps(0.35, 0.6).shake(0.5), None);
            }
        }
        // tufts on the skyline of the knoll: backlit, dark against the
        // mist, rooted just below the edge and breaking it, in clumps
        let mut x = 0.0;
        let tufts = Fbm::new(365, 3, 25.0);
        let mut fine = Held::new(Tool::rigger(0.3), 366);
        while x < 800.0 {
            x += r.range(0.4, 1.6);
            if tufts.get01(x, 0.0) < 0.45 || (x - tor_base.0).abs() < 60.0 {
                continue;
            }
            let y = knoll(x) + r.range(0.8, 2.5);
            let len = r.range(2.5, 7.0) * (0.6 + tufts.get01(x, 9.0));
            let lean = r.normal() * 0.3 + 0.2;
            fine.load(pal.paint(if r.chance(0.2) { hex("#6a5f42") } else { hex("#1f1c16") }, 0.15), 0.4);
            let tip = (x + lean.sin() * len, y - lean.cos() * len);
            // set down lightly at the root, pressed into the blade, lifted off
            c.drag(&mut fine, &Gesture::new(vec![(x, y), (x + lean.sin() * len * 0.4, y - len * 0.5), tip]).pressure(0.7, 0.0).swell(vec![0.4, 1.0, 0.7]).ramps(0.4, 0.6).shake(0.5), None);
        }
        c.dry();
    }

    if o.stage("moon", &mut c, &mut rng) {
        // the old moon, a thin waning crescent, rising ahead of the sun: its
        // lit limb turned down toward the light still below the ranges
        let m = (452.0f32, 118.0f32);
        let r = 8.0f32;
        let to_sun = (SUN.1 - m.1).atan2(SUN.0 - m.0);
        // the lune: the disc less the same disc moved away from the sun
        let off = 0.42 * r;
        let (ix, iy) = (m.0 - off * to_sun.cos(), m.1 - off * to_sun.sin());
        let lune_m = Mask::from_fn(f, move |x, y| {
            let d0 = ((x - m.0).powi(2) + (y - m.1).powi(2)).sqrt();
            let d1 = ((x - ix).powi(2) + (y - iy).powi(2)).sqrt();
            (1.0 - smoothstep(r - 0.35, r + 0.35, d0)) * smoothstep(r - 0.35, r + 0.35, d1)
        });
        // filled with a small sable, strokes running round the limb, clipped
        // to the lune so the horns come to points
        let hd = st
            .detail()
            .color(|_, _| hex("#ebe4cd"))
            .medium(0.35)
            .angle(move |x, y| (y - m.1).atan2(x - m.0) + std::f32::consts::FRAC_PI_2)
            .angle_jitter(0.05)
            .length(2.0, 5.0)
            .coverage(3.2)
            .tool_width(1.0)
            .clip(true)
            .threshold(0.3);
        c.work(&lune_m, &hd, 381);
        c.dry();
    }

    if o.stage("birds", &mut c, &mut rng) {
        // a few birds far off over the valley, going toward the light
        let mut r = Rng::new(371);
        let mut b = Held::new(Tool::rigger(0.4), 372);
        for &(x, y, s) in &[(546.0, 263.0, 3.4), (559.0, 256.0, 3.0), (569.0, 260.0, 2.6), (585.0, 252.0, 2.2)] {
            b.load(pal.paint(hex("#4a4650"), 0.2), 0.5);
            // each wing a curved flick from the body out to its tip; one
            // bird with wings down, the others up
            let wing = if r.chance(0.25) { -0.2 } else { r.range(0.3, 0.55) };
            c.drag(&mut b, &Gesture::new(vec![(x, y), (x - s * 0.45, y - s * (wing * 0.6 + 0.12)), (x - s, y - s * wing)]).pressure(0.7, 0.05).ramps(0.0, 0.5).shake(0.2), None);
            c.drag(&mut b, &Gesture::new(vec![(x, y), (x + s * 0.45, y - s * (wing * 0.6 + 0.12)), (x + s, y - s * wing * 0.9)]).pressure(0.7, 0.05).ramps(0.0, 0.5).shake(0.2), None);
        }
        // the chapel of St. Laurentius on the summit of the Schneekoppe: a
        // tiny round nub with a lantern, barely darker than the mountain
        let cx = (640..700).map(|x| (x as f32, far_crest(x as f32))).fold((668.0, 1e9f32), |a, p| if p.1 < a.1 { p } else { a });
        let mut t = Held::new(Tool::round_sable(0.9), 373);
        t.load(pal.paint(hex("#6e6878"), 0.2), 0.5);
        c.drag(&mut t, &Gesture::new(vec![(cx.0 - 1.3, cx.1 + 0.6), (cx.0 + 1.3, cx.1 + 0.6)]).pressure(0.9, 0.9).shake(0.1), None);
        let mut t2 = Held::new(Tool::rigger(0.3), 374);
        t2.load(pal.paint(hex("#6a6474"), 0.2), 0.5);
        c.drag(&mut t2, &Gesture::new(vec![(cx.0, cx.1 + 0.2), (cx.0, cx.1 - 1.6)]).pressure(0.8, 0.1).ramps(0.0, 0.6).shake(0.1), None);
        c.dry();
    }

    // an old canvas, but its cracks drawn at their real width (the aged
    // preset's hairlines are a whole dark pixel at 1000px), over the ground
    // this canvas really has (240 µm: the cracks go their own way, not the
    // weave's)
    let fin = Finish { cracks: Some(paint::Cracks { width_um: 40.0, depth_um: 20.0, dirt: 0.3, ground_um: st.ground.iter().map(|g| g.um).sum(), ..paint::Cracks::aged(0) }), ..Finish::aged(st.relief) };
    o.finish(&mut c, &mut rng, &fin);
    let _ = luminance;
}

trait ToolWidth {
    fn tool_width(self, w: f32) -> Self;
}
impl ToolWidth for paint::Handling<'_> {
    /// The handling's brush at another size (the engine has no builder for
    /// it; see FRICTION in the notes).
    fn tool_width(mut self, w: f32) -> Self {
        self.tool.length *= w / self.tool.width;
        self.tool.width = w;
        self
    }
}

/// The granite tor: three weathered blocks stacked, the typical
/// Riesengebirge "woolsack" weathering, rounded at the edges, split by
/// joints. Contre-jour: dark, with a thin light along the edges that face
/// the dawn.
/// The tor: weathered granite blocks stacked, the typical Riesengebirge
/// "woolsack" weathering, rounded at the edges, split by joints.
fn tor(bx: f32, by: f32) -> Sdf {
    let z = 0.0;
    // sizes are whole extents; the lowest blocks are sunk into the turf
    let low = Sdf::block([bx + 6.0, by - 9.0, z], [106.0, 32.0, 46.0], 7.0).turn([bx, by, z], 0.22, 0.2, 0.02).rough(2.6, 26.0, 1, false);
    let mid = Sdf::block([bx + 9.0, by - 36.0, z - 3.0], [66.0, 24.0, 38.0], 6.0).turn([bx, by - 36.0, z], -0.15, 0.28, -0.06).rough(2.2, 20.0, 2, false);
    let top = Sdf::block([bx + 16.0, by - 55.0, z - 5.0], [38.0, 16.0, 26.0], 6.0).turn([bx + 16.0, by - 55.0, z], 0.25, 0.25, 0.1).rough(1.6, 14.0, 3, false);
    let side = Sdf::block([bx - 54.0, by - 4.0, z + 8.0], [32.0, 19.0, 26.0], 6.0).turn([bx - 54.0, by, z], 0.45, 0.2, -0.16).rough(1.6, 14.0, 4, false);
    let low = low.cut([bx - 14.0, by - 10.0, z + 20.0], [0.3, -1.0, 0.25], 10, 3.0);
    low.union(mid, 2.5).union(top, 2.0).union(side, 2.0).rough(0.6, 5.0, 5, true)
}

/// A low granite outcrop breaking through the turf in the near foreground,
/// cut by the frame: two broad slabs and a split boulder.
fn outcrop(h: f32) -> Sdf {
    let z = 60.0;
    let a = Sdf::block([50.0, h - 14.0, z], [220.0, 64.0, 110.0], 26.0).turn([50.0, h, z], 0.3, 0.22, -0.08).rough(7.0, 45.0, 11, false);
    let b = Sdf::block([178.0, h - 2.0, z + 10.0], [100.0, 38.0, 64.0], 16.0).turn([178.0, h, z], -0.25, 0.2, 0.06).rough(5.0, 32.0, 12, false);
    let b = b.cut([200.0, h - 20.0, z + 30.0], [1.0, -0.3, 0.4], 11, 2.0);
    a.union(b, 3.0).rough(0.8, 6.0, 13, true)
}

/// Paint a granite solid in contre-jour: dark, with light only on the
/// planes that face the dawn sky; joints drawn in; its foot tucked back
/// into the turf along `seat(x)` (the ground line it rises from). `k`
/// scales the brushes (1 = the tor).
#[allow(clippy::too_many_arguments)]
fn paint_granite(c: &mut paint::Canvas, st: &Style, rock: &Sdf, dist: f32, k: f32, sheen: f32, seat: &(dyn Fn(f32) -> f32 + Sync), ground: Mask, ground_col: &(dyn Fn(f32, f32) -> Rgb + Sync), seed: u64) {
    let pal = &st.palette;
    let f = c.frame();
    let mut form = Form::new(f);
    let id = form.add(rock, dist);
    form.light(Light::new((0.7, -0.55), -0.25).ambient(0.32).penumbra(0.08));
    let form = &form;
    let sil = form.silhouette(&[id], |_| 0.35);
    let lichen = Fbm::new(seed as u32, 4, 9.0);
    let lichen = &lichen;
    let col = move |x: f32, y: f32| match form.sample(x, y).filter(|s| s.part == id) {
        Some(s) => {
            let v = s.shade.value;
            let dark = mix(hex("#1e1c1d"), hex("#3a383c"), s.shade.sky * 0.8, Mix::Pigment);
            let dark = mix(dark, hex("#332b24"), s.shade.bounce.min(0.6), Mix::Pigment);
            let lit = mix(hex("#4d4943"), hex("#8f8270"), s.shade.direct.powf(1.5), Mix::Pigment);
            let base = mix(dark, lit, s.shade.lit(0.15) * 0.85 + 0.1 * v, Mix::Pigment);
            // lichen on the tops
            let up = smoothstep(-0.2, -0.7, s.n[1]) * smoothstep(0.5, 0.75, lichen.get01(x, y));
            let base = mix(base, hex("#5e6048"), up * 0.6, Mix::Pigment);
            // a near rock in contre-jour is darker than a far one
            mix(base, hex("#15130f"), (1.0 - sheen) * 0.8 * (1.0 - 0.5 * s.shade.direct), Mix::Pigment)
        }
        None => ground_col(x, y),
    };
    let fall = |x: f32, y: f32| form.fall(x, y);
    // dark lay-in down the planes
    let hd = paint::Handling::new(Tool { width: 3.2 * k, ..st.body.clone() }).mixed(pal, 0.3).color(&col).angle(fall).angle_jitter(0.2).length(4.0 * k, 12.0 * k).coverage(3.2).pressure(0.6, 0.9).dips(2, 0.55, 0.6).clip(true).threshold(0.2);
    c.work(&sil, &hd, seed);
    // the lit planes and edges, stiffer
    let lit = form.mask(|s| if s.part == id { s.shade.lit(0.12) } else { 0.0 }).mul(&sil);
    let hd = paint::Handling::new(Tool { width: 1.8 * k, lay: 1.1, ..st.body.clone() }).mixed(pal, 0.15).color(&col).angle(fall).angle_jitter(0.15).length(2.5 * k, 8.0 * k).coverage(2.4).pressure(0.6, 0.9).dips(2, 0.6, 0.7).clip(true).threshold(0.3);
    c.work(&lit, &hd, seed + 1);
    // joints: dark lines where the blocks meet
    let sp = 1.5 * k;
    let joints = Mask::from_fn(f, |x, y| if form.part(x, y) == id { smoothstep(0.2, 0.6, -form.bend(x, y, sp)) } else { 0.0 }).mul(&sil.erode(0.8));
    let hd = paint::Handling::new(Tool::round_sable(0.9 * k)).mixed(pal, 0.15).color(|_, _| hex("#1a1817")).angle(|x, y| form.edge_angle(x, y, sp)).angle_jitter(0.1).length(2.0 * k, 6.0 * k).coverage(1.0).pressure(0.5, 0.8).dips(3, 0.6, 0.8).clip(true).threshold(0.35);
    c.work(&joints, &hd, seed + 2);
    c.dry();
    // the tor sits in the ground: tuck the knoll's color back over its foot
    let foot = Mask::from_fn(f, move |x, y| if form.part(x, y) == id || form.part(x, y - 6.0) == id { smoothstep(seat(x) - 5.0, seat(x) + 2.0, y) } else { 0.0 }).mul(&ground);
    let hd = st.body().color(ground_col).angle(|_, _| 0.0).length(6.0 * k, 16.0 * k).coverage(1.5).tool_width(3.0 * k).clip(true);
    c.work(&foot, &hd, seed + 3);
    c.dry();
}

/// A spruce as I paint one small: the stem in one stroke lifted toward the
/// top, then the tiers hatched from the stem outward and down, shorter
/// toward the top; the dead one is a gray stem with a few bare claws.
fn spruce(c: &mut paint::Canvas, pal: &paint::Palette, (x, y): (f32, f32), ht: f32, dead: bool, r: &mut Rng, seed: u64) {
    let dark = pal.paint(hex("#1b201c"), 0.2);
    let gray = pal.paint(hex("#5a544c"), 0.2);
    let lean = r.normal() * 0.02;
    let at = |v: f32| (x + lean * v * ht + (v * 9.0).sin() * 0.3, y - v * ht);
    let mut stem = Held::new(Tool::round_sable((ht * 0.028).max(0.8)), seed);
    stem.load(if dead { gray } else { dark }, 0.6);
    let pts: Vec<(f32, f32)> = (0..=8).map(|i| at(i as f32 / 8.0)).collect();
    c.drag(&mut stem, &Gesture::new(pts).pressure(0.9, 0.15).ramps(0.0, 0.5).shake(0.4), None);
    if dead {
        let mut b = Held::new(Tool::rigger(0.5), seed + 1);
        for i in 0..9 {
            let v = 0.25 + 0.7 * i as f32 / 9.0 + r.range(-0.03, 0.03);
            let s = if i % 2 == 0 { -1.0 } else { 1.0 };
            let reach = ht * (0.14 * (1.0 - v) + 0.03) * r.range(0.6, 1.2);
            let (px, py) = at(v);
            b.load(gray, 0.5);
            c.drag(&mut b, &Gesture::new(vec![(px, py), (px + s * reach * 0.6, py + reach * 0.15), (px + s * reach, py + reach * r.range(-0.1, 0.4))]).pressure(0.8, 0.1).ramps(0.0, 0.6), None);
        }
        return;
    }
    // wind from the west has thinned the left side
    let mut b = Held::new(Tool::round_sable((ht * 0.018).max(0.6)), seed + 2);
    let mut fine = Held::new(Tool::rigger((ht * 0.009).max(0.35)), seed + 3);
    let tiers = (ht / 2.6) as usize;
    for i in 0..tiers {
        let v = 0.08 + 0.9 * (i as f32 + r.range(0.0, 0.7)) / tiers as f32;
        let reach = ht * (0.2 * (1.0 - v).powf(0.9) + 0.01) * r.range(0.7, 1.15);
        let (px, py) = at(v);
        for s in [-1.0f32, 1.0] {
            let reach = if s < 0.0 { reach * r.range(0.45, 0.9) } else { reach };
            if s < 0.0 && r.chance(0.2) {
                continue;
            }
            let brush = if reach < ht * 0.05 { &mut fine } else { &mut b };
            if i % 2 == 0 {
                brush.load(dark, 0.55);
            }
            // out and down, then the tip lifting a little; each tier a few
            // hatched strokes, the bough's needles hanging from it
            let droop = reach * r.range(0.25, 0.45);
            c.drag(brush, &Gesture::new(vec![(px, py), (px + s * reach * 0.55, py + droop), (px + s * reach, py + droop * 0.7)]).pressure(0.85, 0.25).ramps(0.0, 0.5).shake(0.6), None);
            let k = (reach / 2.5) as usize;
            for j in 0..k {
                let t = (j as f32 + 0.5) / k as f32;
                let (qx, qy) = (px + s * reach * t, py + droop * (1.6 * t - 0.8 * t * t) * 0.9);
                let l = reach * 0.25 * (1.0 - 0.5 * t) + 1.0;
                c.drag(brush, &Gesture::new(vec![(qx, qy), (qx + s * l * 0.2, qy + l)]).pressure(0.6, 0.1).ramps(0.0, 0.6).orient(Orient::Across).shake(0.5), None);
            }
        }
    }
    // the leader
    fine.load(dark, 0.5);
    let (tx, ty) = at(1.0);
    c.drag(&mut fine, &Gesture::new(vec![at(0.92), (tx, ty - 2.0)]).pressure(0.7, 0.0).ramps(0.0, 0.8), None);
}

/// The wanderer, seen from behind: a long dark coat, bare head, a stick in
/// the right hand planted on the rock; he looks east toward the light. His
/// outline against the pale valley air is the one sharp thing in the
/// picture.
fn wanderer(c: &mut paint::Canvas, pal: &paint::Palette, (x, y): (f32, f32), s: f32, seed: u64) {
    let mut r = Rng::new(seed);
    let coat = pal.paint(hex("#1e2422"), 0.15);
    let coat_lit = pal.paint(hex("#3c4540"), 0.15);
    let trousers = pal.paint(hex("#2b2622"), 0.15);
    let hair = pal.paint(hex("#2a2019"), 0.15);
    let skin = pal.paint(hex("#b08a6c"), 0.1);
    let rim = pal.paint(hex("#c9b58f"), 0.1);
    let p = |u: f32, v: f32| (x + u * s, y - v * s);
    let drag = |c: &mut paint::Canvas, tool: Tool, paint: paint::Paint, pts: &[(f32, f32)], pr: (f32, f32), r: &mut Rng| {
        let mut b = Held::new(tool, r.next_u64());
        b.load(paint, 0.6);
        let pts: Vec<(f32, f32)> = pts.iter().map(|&(u, v)| p(u, v)).collect();
        c.drag(&mut b, &Gesture::new(pts).pressure(pr.0, pr.1).ramps(0.05, 0.25).shake(0.3), None);
    };
    let t = |w: f32| Tool::round_sable(w * s);
    // legs and boots
    drag(c, t(0.05), trousers, &[(-0.045, 0.40), (-0.05, 0.2), (-0.055, 0.02)], (0.9, 0.7), &mut r);
    drag(c, t(0.05), trousers, &[(0.04, 0.40), (0.05, 0.2), (0.06, 0.02)], (0.9, 0.7), &mut r);
    drag(c, t(0.045), coat, &[(-0.07, 0.015), (-0.04, 0.01)], (0.9, 0.9), &mut r);
    drag(c, t(0.045), coat, &[(0.04, 0.012), (0.08, 0.01)], (0.9, 0.9), &mut r);
    // the coat: from the shoulders down to below the knee, flaring
    for (u0, u1) in [(-0.1, -0.12), (-0.05, -0.07), (0.0, 0.0), (0.05, 0.07), (0.1, 0.12)] {
        drag(c, t(0.075), coat, &[(u0 * 0.85, 0.83), (u0, 0.6), (u1, 0.32)], (0.9, 0.9), &mut r);
    }
    // a second pass over the coat, the strokes laid between the first, so
    // no light shows between them
    for (u0, u1) in [(-0.075, -0.095), (-0.025, -0.035), (0.025, 0.035), (0.075, 0.095)] {
        drag(c, t(0.07), coat, &[(u0 * 0.85, 0.82), (u0, 0.58), (u1, 0.33)], (0.9, 0.85), &mut r);
    }
    // shoulders and collar
    drag(c, t(0.07), coat, &[(-0.13, 0.8), (0.0, 0.845), (0.13, 0.8)], (0.9, 0.9), &mut r);
    // arms: the left hangs, the right reaches out to the stick
    drag(c, t(0.05), coat, &[(-0.13, 0.8), (-0.15, 0.62), (-0.14, 0.47)], (0.9, 0.8), &mut r);
    drag(c, t(0.05), coat, &[(0.13, 0.8), (0.18, 0.66), (0.22, 0.54)], (0.9, 0.8), &mut r);
    drag(c, t(0.03), skin, &[(0.22, 0.54), (0.23, 0.52)], (0.8, 0.6), &mut r);
    drag(c, t(0.03), skin, &[(-0.14, 0.47), (-0.14, 0.45)], (0.7, 0.5), &mut r);
    // the stick, planted ahead and to the right
    drag(c, Tool::rigger(0.018 * s), pal.paint(hex("#3a2e22"), 0.1), &[(0.23, 0.62), (0.26, 0.3), (0.3, 0.0)], (0.9, 0.7), &mut r);
    // neck and head: bare, hair to the collar, the head a little turned
    drag(c, t(0.045), skin, &[(0.0, 0.84), (0.005, 0.87)], (0.8, 0.8), &mut r);
    drag(c, t(0.1), hair, &[(0.0, 0.88), (0.008, 0.93), (0.01, 0.96)], (0.95, 0.8), &mut r);
    drag(c, t(0.03), skin, &[(0.055, 0.915), (0.06, 0.93)], (0.5, 0.3), &mut r);
    // light from the dawn along the right edge of the figure: a thin rim
    drag(c, t(0.018), coat_lit, &[(0.14, 0.79), (0.16, 0.6), (0.13, 0.33)], (0.6, 0.3), &mut r);
    drag(c, t(0.012), rim, &[(0.125, 0.81), (0.14, 0.78)], (0.5, 0.2), &mut r);
    drag(c, t(0.012), rim, &[(0.05, 0.95), (0.055, 0.92)], (0.5, 0.2), &mut r);
    let _ = Touch::at(0.0, 0.0);
    c.dry();
}
