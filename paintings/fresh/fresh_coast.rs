//! "Evening on the Baltic Shore, with Anchor" — an original painting in the
//! manner of Caspar David Friedrich (Dresden practice, c. 1820–25).
//!
//! The afterglow of a sunset that has already gone below the sea, left of
//! center; a thin waxing crescent moon high on the right with the evening
//! star low in the glow. A brig stands out to sea on the axis of the moon.
//! On the shore a woman in a dark dress, seen from behind, watches it go; in
//! the dark foreground an old anchor lies half sunk among granite erratics.
//! (Anchor = hope; ship = the passage of life; moon = the promise beyond it;
//! the figure turned away so we look with her — Friedrich's Rückenfigur.)
//!
//! Order of work follows notes/research/friedrich_materials.md: bought
//! ground, graphite underdrawing, very thin dead coloring, sky, distance to
//! foreground, figure last, then a darkening glaze toward the edges (his
//! advice to Carus) and varnish.

use paint::color::{Mix, gradient, mix};
use paint::{Canvas, Frame, Gesture, Held, Mask, Orient, Paint, Rgb, Rng, Shape, Style, Tool, hex, smoothstep};
use paintings::run::{Finish, Run};

const ASPECT: f32 = 1.4;
/// Sea horizon (units from the top).
const HOR: f32 = 440.0;
/// Where the sunset glow sits (x).
const GX: f32 = 360.0;
/// The moon.
const MOON: (f32, f32) = (655.0, 128.0);
/// The brig (waterline center).
const SHIP: (f32, f32) = (668.0, 474.0);

fn hgt() -> f32 {
    1000.0 / ASPECT
}

/// The waterline: where the last thin wave runs out on the sand.
fn shore_y(x: f32) -> f32 {
    578.0 + 10.0 * (x * 0.004 + 0.7).sin() + 5.0 * (x * 0.013).sin() - 0.012 * (x - 500.0)
}

/// Afterglow sky: deep dusky blue overhead, through a cool violet-grey, to a
/// rose and then a pale yellow band on the horizon; warmest above the place
/// the sun went down.
fn sky(x: f32, y: f32) -> Rgb {
    let t = (y / HOR).clamp(0.0, 1.0);
    let base = gradient(
        &[
            (0.0, hex("#384463")),
            (0.2, hex("#525e7e")),
            (0.4, hex("#7c819e")),
            (0.58, hex("#a09bae")),
            (0.74, hex("#cda9a2")),
            (0.9, hex("#e2c39c")),
            (1.0, hex("#ecd6a4")),
        ],
        t,
        Mix::Light,
    );
    let d = (x - GX) / 420.0;
    let glow = (-d * d).exp() * smoothstep(0.45, 1.0, t);
    let warm = mix(base, hex("#f1d29a"), glow * 0.55, Mix::Light);
    // where the sun went down: a closer, brighter glow on the horizon
    let d2 = (x - GX) / 190.0;
    let core = (-d2 * d2).exp() * smoothstep(0.78, 1.0, t);
    let warm = mix(warm, hex("#f8e6b4"), core * 0.7, Mix::Light);
    // far from the glow, the low sky is greyer and cooler
    let cool = (1.0 - (-d * d * 0.6).exp()) * smoothstep(0.5, 1.0, t);
    let c = mix(warm, hex("#a9a3ab"), cool * 0.35, Mix::Light);
    // mixed a little deeper toward the upper corners (the darkening toward
    // the edges Friedrich recommended to Carus, put into the paint itself)
    let e = ((x - 470.0) / 560.0).powi(2) + ((y - 380.0) / 420.0).powi(2);
    mix(c, hex("#2c3448"), smoothstep(0.6, 1.5, e) * 0.35, Mix::Light)
}

/// The sea: near the horizon it mirrors the low sky, darker; toward the shore
/// a deep cool slate. A lighter column lies under the glow.
fn sea(x: f32, y: f32) -> Rgb {
    let t = ((y - HOR) / (shore_y(x) - HOR)).clamp(0.0, 1.0);
    let base = gradient(
        &[(0.0, hex("#a39891")), (0.1, hex("#7c7c83")), (0.4, hex("#505a67")), (1.0, hex("#2c343e"))],
        t,
        Mix::Light,
    );
    let d = (x - GX) / (60.0 + 110.0 * t);
    let col = (-d * d).exp() * (1.0 - t).powf(0.8);
    mix(base, hex("#e2cc9e"), col * 0.6, Mix::Light)
}

/// The shore: wet sand at the waterline taking a little sky, then darker
/// sand and shingle, darkest at the bottom edge.
fn shore(x: f32, y: f32) -> Rgb {
    let t = ((y - shore_y(x)) / (hgt() - shore_y(x))).clamp(0.0, 1.0);
    gradient(
        &[(0.0, hex("#7a746c")), (0.08, hex("#5d554a")), (0.4, hex("#43392e")), (1.0, hex("#231d17"))],
        t,
        Mix::Light,
    )
}

/// A granite erratic: outline (clockwise), and planes blocked in over the
/// dark body: (polygon, tone, stroke angle).
struct Rock {
    pts: Vec<(f32, f32)>,
    planes: Vec<(Vec<(f32, f32)>, &'static str, f32)>,
}

fn rocks() -> Vec<Rock> {
    let h = hgt() + 5.0;
    let v = std::f32::consts::FRAC_PI_2;
    vec![
        // the big boulder at the right foreground, cut by the frame
        Rock {
            pts: vec![(808.0, h), (800.0, 676.0), (806.0, 648.0), (826.0, 626.0), (852.0, 612.0), (884.0, 605.0), (912.0, 601.0), (944.0, 592.0), (978.0, 595.0), (1006.0, 604.0), (1006.0, h)],
            planes: vec![
                (vec![(820.0, 630.0), (852.0, 610.0), (884.0, 603.0), (912.0, 599.0), (944.0, 590.0), (978.0, 593.0), (1008.0, 602.0), (1008.0, 626.0), (970.0, 621.0), (930.0, 630.0), (890.0, 634.0), (850.0, 646.0), (826.0, 650.0)], "#5e5854", 0.05),
                (vec![(798.0, 676.0), (804.0, 648.0), (824.0, 626.0), (836.0, 652.0), (832.0, 690.0), (812.0, 716.0)], "#4a423b", v - 0.35),
                (vec![(890.0, 636.0), (930.0, 632.0), (970.0, 623.0), (1008.0, 628.0), (1008.0, 662.0), (950.0, 657.0), (900.0, 664.0)], "#3b3530", 0.1),
            ],
        },
        // a lower one in front of it
        Rock {
            pts: vec![(730.0, h), (734.0, 697.0), (752.0, 682.0), (780.0, 676.0), (806.0, 678.0), (828.0, 690.0), (840.0, 706.0), (842.0, h)],
            planes: vec![(vec![(738.0, 690.0), (752.0, 680.0), (780.0, 674.0), (806.0, 676.0), (830.0, 690.0), (820.0, 697.0), (790.0, 692.0), (760.0, 697.0)], "#57504a", 0.0)],
        },
        // left group
        Rock {
            pts: vec![(-6.0, h), (-6.0, 626.0), (22.0, 616.0), (52.0, 611.0), (80.0, 617.0), (104.0, 634.0), (118.0, 656.0), (124.0, 684.0), (120.0, h)],
            planes: vec![
                (vec![(-8.0, 624.0), (22.0, 614.0), (52.0, 609.0), (80.0, 615.0), (106.0, 634.0), (96.0, 645.0), (60.0, 635.0), (24.0, 638.0), (-8.0, 644.0)], "#625c56", -0.05),
                (vec![(104.0, 632.0), (120.0, 656.0), (126.0, 684.0), (122.0, 716.0), (104.0, 700.0), (98.0, 660.0), (92.0, 642.0)], "#4b423a", v + 0.3),
            ],
        },
        Rock {
            pts: vec![(98.0, h), (102.0, 690.0), (118.0, 674.0), (142.0, 667.0), (168.0, 670.0), (190.0, 684.0), (200.0, h)],
            planes: vec![(vec![(106.0, 684.0), (118.0, 672.0), (142.0, 665.0), (168.0, 668.0), (192.0, 684.0), (170.0, 688.0), (140.0, 684.0), (118.0, 690.0)], "#59524b", 0.0)],
        },
        // low stones out near the waterline
        Rock { pts: vec![(416.0, 611.0), (421.0, 606.0), (433.0, 603.0), (447.0, 604.0), (456.0, 607.0), (460.0, 611.0)], planes: vec![] },
        Rock { pts: vec![(598.0, 605.0), (603.0, 601.5), (613.0, 600.0), (621.0, 602.0), (625.0, 605.5)], planes: vec![] },
        Rock { pts: vec![(262.0, 617.0), (268.0, 612.0), (279.0, 610.0), (291.0, 612.5), (296.0, 617.5)], planes: vec![] },
    ]
}

/// Long evening stratus: bands with a wandering center line and ragged ends.
/// `density` 0..1; `over` paints them over a sky color: violet-grey bodies,
/// with warm lit undersides near the glow.
struct Clouds {
    wob: paint::Fbm,
    gap: paint::Fbm,
}

/// (center y, x from, x to, half thickness)
const BANDS: [(f32, f32, f32, f32); 8] = [
    (356.0, -60.0, 520.0, 3.6),
    (371.0, 180.0, 840.0, 2.2),
    (318.0, 440.0, 1060.0, 2.8),
    (334.0, -40.0, 250.0, 2.2),
    (242.0, 60.0, 480.0, 2.4),
    (398.0, 600.0, 1060.0, 2.2),
    (412.0, 150.0, 470.0, 1.4),
    (286.0, 640.0, 980.0, 1.8),
];

impl Clouds {
    fn new(seed: u64) -> Self {
        Clouds { wob: paint::Fbm::new(seed as u32 + 301, 3, 260.0), gap: paint::Fbm::new(seed as u32 + 302, 4, 90.0) }
    }
    fn density(&self, x: f32, y: f32) -> f32 {
        let mut d: f32 = 0.0;
        for (k, &(yc, x0, x1, th)) in BANDS.iter().enumerate() {
            let yy = yc + 6.0 * self.wob.get(x, k as f32 * 97.0);
            let th = th * (0.7 + 0.6 * self.gap.get01(x * 0.5, k as f32 * 131.0));
            let across = (-((y - yy) / th).powi(2)).exp();
            let along = smoothstep(x0, x0 + 90.0, x) * (1.0 - smoothstep(x1 - 90.0, x1, x));
            let broken = smoothstep(0.25, 0.6, self.gap.get01(x, yc));
            d = d.max(across * along * broken);
        }
        d
    }
    fn over(&self, x: f32, y: f32, base: Rgb) -> Rgb {
        let d = self.density(x, y);
        if d <= 0.0 {
            return base;
        }
        // underside (below the band center) catches the glow
        let below = self.density(x, y - 3.0) - d;
        let near = (-((x - GX) / 280.0).powi(2)).exp() * smoothstep(250.0, 420.0, y);
        let body = mix(base, hex("#5a5268"), 0.6, Mix::Light);
        let lit = mix(body, hex("#d99a7a"), (below.max(0.0) * 3.0).min(1.0) * near * 0.7, Mix::Light);
        mix(base, lit, d, Mix::Light)
    }
}

fn rock_mask(fr: Frame, r: &Rock) -> Mask {
    Mask::from_shape(fr, Shape::new().smooth_poly(&r.pts))
}

/// The painter's eye at the palette: mix, try the pile against what is on the
/// canvas (`sub`, laid about `coats` thick), see how far off it looks, and
/// aim the next mix that much the other way. Three rounds. This keeps a
/// long gradient continuous even where the nearest-color recipe flips
/// between a transparent and an opaque mixture.
fn aim(pal: &paint::Palette, want: Rgb, sub: Rgb, medium: f32, coats: f32) -> Rgb {
    use paint::color::{from_oklab, to_oklab};
    let w = to_oklab(want);
    let mut t = w;
    for _ in 0..3 {
        let pt = pal.mix(from_oklab(t)).paint(medium);
        let got = to_oklab(paint::Pigment::with_hiding(pt.color, pt.hiding).over(sub, coats));
        for i in 0..3 {
            t[i] += 0.8 * (w[i] - got[i]);
        }
        t[0] = t[0].clamp(0.0, 1.0);
    }
    from_oklab(t)
}

/// Drag a brush through canvas points.
fn stroke(c: &mut Canvas, b: &mut Held, pts: &[(f32, f32)], p0: f32, p1: f32, clip: Option<&Mask>) {
    let g = Gesture::new(pts.to_vec()).pressure(p0, p1).ramps(0.1, 0.25).orient(Orient::Across);
    c.drag(b, &g, clip);
}

fn main() {
    let run = Run::new("fresh_coast");
    let st = Style::friedrich();
    let pal = &st.palette;
    let h = hgt();
    let mut c = st.prepare(run.width, ASPECT, run.seed);
    let fr = c.frame();
    let mut rng = Rng::new(run.seed * 977 + 5);
    if std::env::args().any(|a| a == "--gradient") {
        let ground = hex("#a9785a");
        let sh = |c: Rgb| format!("#{:02x}{:02x}{:02x}", (paint::color::linear_to_srgb(c[0]) * 255.0) as u8, (paint::color::linear_to_srgb(c[1]) * 255.0) as u8, (paint::color::linear_to_srgb(c[2]) * 255.0) as u8);
        let sky_pal = paint::Palette::new("Friedrich sky", pal.tubes.iter().filter(|t| t.name != "pale smalt").cloned().collect());
        for k in 0..=22 {
            let y = k as f32 * 20.0;
            let col = sky(500.0, y);
            let m = sky_pal.mix(aim(&sky_pal, col, ground, st.thin_medium, 1.2));
            let pt = m.paint(st.thin_medium);
            let pg = paint::Pigment::with_hiding(pt.color, pt.hiding);
            eprintln!("y {y:4} want {} hide {:.2} over ground 1 coat {} 2 coats {}  {}", sh(col), pt.hiding, sh(pg.over(ground, 1.0)), sh(pg.over(ground, 2.0)), sky_pal.recipe(&m));
        }
        return;
    }
    if std::env::args().any(|a| a == "--recipes") {
        for (name, col) in [
            ("sky top", sky(500.0, 0.0)),
            ("sky 0.3", sky(500.0, 130.0)),
            ("sky 0.6", sky(500.0, 260.0)),
            ("sky 0.8 glow", sky(GX, 350.0)),
            ("sky hor glow", sky(GX, 435.0)),
            ("sky hor right", sky(900.0, 435.0)),
            ("sea hor", sea(500.0, 445.0)),
            ("sea mid", sea(500.0, 500.0)),
            ("sea near", sea(500.0, 570.0)),
            ("shore", shore(500.0, 620.0)),
            ("shawl", hex("#43241e")),
            ("dress", hex("#1f2320")),
            ("hair", hex("#3b2a1c")),
        ] {
            let m = pal.mix(col);
            let s = |c: Rgb| format!("#{:02x}{:02x}{:02x}", (paint::color::linear_to_srgb(c[0]) * 255.0) as u8, (paint::color::linear_to_srgb(c[1]) * 255.0) as u8, (paint::color::linear_to_srgb(c[2]) * 255.0) as u8);
            eprintln!("{name:14} want {} got {} err {:.3} hide {:.2} {}", s(col), s(m.color), m.error, m.hiding, pal.recipe(&m));
            let pt = m.paint(0.15);
            let pg = paint::Pigment::with_hiding(pt.color, pt.hiding);
            eprintln!("     over dark x=0.5,1,2,4: {} {} {} {}", s(pg.over(hex("#1f2320"), 0.5)), s(pg.over(hex("#1f2320"), 1.0)), s(pg.over(hex("#1f2320"), 2.0)), s(pg.over(hex("#1f2320"), 4.0)));
        }
        return;
    }
    if run.stage(&mut c, "ground") {
        return;
    }

    // ---- underdrawing: graphite, a faint first outline, then firmer; the
    // horizon ruled against a straightedge
    {
        let graphite = Paint { color: hex("#3b3834"), hiding: 0.35, stiff: 0.3 };
        let mut pen = Held::new(st.line_tool(0.55), 11);
        pen.load(graphite, 0.5);
        let g = Gesture::line((0.0, HOR), (1000.0, HOR + 0.6)).pressure(0.35, 0.35).shake(0.15);
        c.drag(&mut pen, &g, None);
        // waterline
        let wl: Vec<(f32, f32)> = (0..=20).map(|i| (i as f32 * 50.0, shore_y(i as f32 * 50.0))).collect();
        pen.load(graphite, 0.4);
        stroke(&mut c, &mut pen, &wl, 0.3, 0.3, None);
        for r in rocks() {
            pen.load(graphite, 0.4);
            let mut p = r.pts.clone();
            p.push(p[0]);
            stroke(&mut c, &mut pen, &p, 0.35, 0.3, None);
        }
        // the brig, the figure, the anchor: placed with a few lines
        pen.load(graphite, 0.4);
        stroke(&mut c, &mut pen, &[(SHIP.0 - 22.0, SHIP.1), (SHIP.0 + 20.0, SHIP.1 - 1.0)], 0.35, 0.3, None);
        stroke(&mut c, &mut pen, &[(SHIP.0 - 6.0, SHIP.1), (SHIP.0 - 6.0, SHIP.1 - 64.0)], 0.3, 0.3, None);
        stroke(&mut c, &mut pen, &[(SHIP.0 + 8.0, SHIP.1), (SHIP.0 + 8.0, SHIP.1 - 58.0)], 0.3, 0.3, None);
        stroke(&mut c, &mut pen, &[(548.0, 604.0), (548.0, 540.0)], 0.3, 0.3, None);
        stroke(&mut c, &mut pen, &[(300.0, 690.0), (360.0, 632.0)], 0.35, 0.3, None);
        c.dry();
    }
    if run.stage(&mut c, "drawing") {
        return;
    }

    // ---- dead coloring: a very thin warm-brown underpainting of the land
    // and a thin cool one of the sea; the sky is left to the ground
    {
        let land = Mask::from_fn(fr, |x, y| smoothstep(shore_y(x) - 4.0, shore_y(x) + 4.0, y));
        let hd = st.broad().color(|_, y| mix(hex("#5a4636"), hex("#2e241c"), (y - 580.0) / 140.0, Mix::Light)).medium(0.7).coverage(1.6);
        c.work(&land, &hd, 21);
        let water = Mask::from_fn(fr, |x, y| smoothstep(HOR - 2.0, HOR + 3.0, y) * (1.0 - smoothstep(shore_y(x) - 2.0, shore_y(x) + 4.0, y)));
        let hd = st.broad().color(|x, y| mix(sea(x, y), hex("#4a4a50"), 0.12, Mix::Light)).medium(0.6).coverage(2.0).angle(|_, _| 0.0).angle_jitter(0.005);
        c.work(&water, &hd, 22);
        c.dry();
    }
    if run.stage(&mut c, "dead") {
        return;
    }

    // ---- sky: two thin layers, as in the Monk's sky (a first darker layer,
    // then the lighter ones over it). Horizontal strokes of a soft filbert,
    // the cloud bands laid into the wet second layer, all fused with the
    // badger; then a fine stipple of nearly the same tones to break the
    // strokes into the dotted texture of his skies.
    {
        let sky_m = Mask::from_fn(fr, |_, y| 1.0 - smoothstep(HOR + 1.0, HOR + 6.0, y));
        // for the sky he mixes his cobalt with white (after ~1820 cobalt had
        // largely replaced smalt: ALF pp.341, 348; NG p.56)
        let sky_pal = paint::Palette::new("Friedrich sky", pal.tubes.iter().filter(|t| t.name != "pale smalt").cloned().collect());
        let sp = &sky_pal;
        let snap0 = c.pixels().to_vec();
        let m1 = st.thin_medium;
        let first = st.broad().mixed(sp, m1).color(move |x, y| aim(sp, mix(sky(x, y), hex("#56607a"), 0.2, Mix::Light), snap0[fr.index(x, y)], m1, 1.2)).angle(|_, _| 0.0)
            .angle_jitter(0.01).length(60.0, 160.0).coverage(3.0).dips(1, 0.55, 0.6);
        c.work(&sky_m, &first, 31);
        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.angle(|_, _| 0.0), 32);
        }
        c.dry();
        let cl = Clouds::new(run.seed);
        let skyc = |x: f32, y: f32| cl.over(x, y, sky(x, y));
        let snap1 = c.pixels().to_vec();
        let aimed = |x: f32, y: f32| aim(sp, skyc(x, y), snap1[fr.index(x, y)], 0.3, 1.2);
        let second = st.broad().mixed(sp, 0.3).color(aimed).angle(|_, _| 0.0)
            .angle_jitter(0.01).length(50.0, 140.0).coverage(3.5).dips(1, 0.6, 0.7);
        c.work(&sky_m, &second, 33);
        for pass in 0..st.blend_passes {
            if let Some(b) = st.blend() {
                c.work(&sky_m, &b.angle(|_, _| 0.0).pressure(0.3 + 0.1 * pass as f32, 0.45), 35 + 100 * pass as u64);
            }
        }
        // the cloud streaks, into the still wet sky: a smaller filbert
        // following the bands, then fused once, lightly
        let cm = Mask::from_fn(fr, |x, y| smoothstep(0.08, 0.3, cl.density(x, y)));
        let streaks = paint::Handling::new(Tool { ragged: 0.5, lay: 0.8, ..Tool::filbert(4.0) })
            .mixed(sp, 0.3).mix_jitter(0.04).color(aimed).angle(|_, _| 0.0).angle_jitter(0.008)
            .length(30.0, 90.0).coverage(3.0).pressure(0.5, 0.75).dips(1, 0.55, 0.8).ramps(0.3, 0.4).threshold(0.45);
        c.work(&cm, &streaks, 34);
        if let Some(b) = st.blend() {
            c.work(&cm.clone().blur(8.0), &b.angle(|_, _| 0.0).pressure(0.2, 0.3).coverage(1.2).threshold(0.1), 36);
        }
        c.dry();
        // stipple: a small soft round dabbed all over in lean paint, each
        // pile mixed to match the tone already on the canvas at that spot and
        // a breath lighter (the painter looks, then mixes), so it breaks the
        // strokes into a fine scatter without spotting the sky
        let snap = c.pixels().to_vec();
        let look = move |x: f32, y: f32| {
            let p = snap[fr.index(x, y)];
            aim(pal, mix(p, hex("#f2ead8"), 0.03, Mix::Light), p, 0.65, 0.6)
        };
        let stip = paint::Handling::new(Tool { ragged: 0.1, ..Tool::round_sable(1.3) })
            .mixed(pal, 0.65)
            .mix_jitter(0.0)
            .jitter(0.0, 0.0)
            .color(look)
            .length(0.6, 1.2)
            .coverage(0.6)
            .pressure(0.45, 0.7)
            .dips(4, 0.25, 0.5)
            .ramps(0.3, 0.5)
            .angle_jitter(1.5);
        // (only in the warm low sky: in the thin dark blues above, every
        // dot dried darker than the passage around it, see the wishlist)
        let low = sky_m.clone().mul_fn(|_, y| smoothstep(250.0, 300.0, y));
        c.work(&low, &stip, 37);
        c.dry();
    }
    if run.stage(&mut c, "sky") {
        return;
    }

    // ---- the crescent moon and the evening star. The sickle is drawn with
    // a small soft round in a few arcs along the lit limb (toward the lower
    // left, where the sun went down): the outer arc longest, inner arcs
    // shorter, so the horns taper to points.
    {
        let (mx, my) = MOON;
        let r = 12.5;
        let face = 2.5f32; // radians: toward the lower left (y down)
        let mut b = Held::new(Tool { ragged: 0.05, ..Tool::round_sable(2.6) }, 51);
        for (k, &(rr, span, p)) in [(r - 1.0, 1.45f32, 0.7f32), (r - 2.1, 1.25, 0.75), (r - 3.2, 0.98, 0.7), (r - 4.1, 0.62, 0.6)].iter().enumerate() {
            b.reload(pal.paint(hex(if k == 0 { "#f6efd6" } else { "#efe6c8" }), 0.1), 0.7);
            let pts: Vec<(f32, f32)> = (0..=12)
                .map(|i| {
                    let a = face - span + 2.0 * span * i as f32 / 12.0;
                    (mx + a.cos() * rr, my + a.sin() * rr)
                })
                .collect();
            c.drag(&mut b, &Gesture::new(pts).pressure(p, p).ramps(0.35, 0.35).shake(0.1), None);
        }
        c.dry();
        // evening star
        let mut s = Held::new(Tool::round_sable(2.4), 52);
        s.load(pal.paint(hex("#fbf3dc"), 0.1), 0.8);
        c.drag(&mut s, &Gesture::new(vec![(292.0, 296.0), (292.3, 296.4)]).pressure(0.8, 0.8).ramps(0.0, 0.3).shake(0.0), None);
        c.dry();
    }
    if run.stage(&mut c, "clouds") {
        return;
    }

    // ---- the far coast: a low blue strip on the left horizon
    {
        let coast = |x: f32| HOR - 9.0 * smoothstep(250.0, 30.0, x) - 3.0 * (x * 0.03).sin().abs() * smoothstep(250.0, 60.0, x);
        let m = Mask::from_fn(fr, move |x, y| {
            let top = coast(x);
            if x > 260.0 { 0.0 } else { smoothstep(top - 0.8, top + 0.8, y) * (1.0 - smoothstep(HOR + 0.5, HOR + 2.0, y)) }
        });
        let hd = st.detail().color(|_, _| hex("#8a8698")).medium(0.4).angle(|_, _| 0.0).length(10.0, 30.0).clip(true);
        c.work(&m, &hd, 60);
        c.dry();
    }

    // ---- the sea: horizontal strokes, lightest at the horizon; then the
    // swells as long lighter lines, closer together in the distance
    {
        let water = Mask::from_fn(fr, |x, y| smoothstep(HOR - 0.5, HOR + 1.5, y) * (1.0 - smoothstep(shore_y(x), shore_y(x) + 4.0, y)));
        let snap = c.pixels().to_vec();
        let hd = st.broad().color(move |x, y| aim(pal, sea(x, y), snap[fr.index(x, y)], 0.35, 1.2)).angle(|_, _| 0.0).angle_jitter(0.01).length(100.0, 260.0).coverage(3.0).medium(0.35);
        c.work(&water, &hd, 70);
        if let Some(b) = st.blend() {
            c.work(&water, &b.angle(|_, _| 0.0).pressure(0.35, 0.45), 71);
        }
        c.dry();
        let snap = c.pixels().to_vec();
        let seac = move |x: f32, y: f32| aim(pal, sea(x, y), snap[fr.index(x, y)], 0.28, 1.2);
        let hd = st.broad().color(&seac).angle(|_, _| 0.0).angle_jitter(0.006).length(50.0, 150.0).coverage(3.5).medium(0.28).dips(1, 0.6, 0.7);
        c.work(&water, &hd, 73);
        // the far sea, right under the horizon: a narrow even band laid with
        // a smaller brush, cut clean against the sky
        let far = Mask::from_fn(fr, |_, y| smoothstep(HOR - 0.3, HOR + 0.6, y) * (1.0 - smoothstep(HOR + 14.0, HOR + 22.0, y)));
        let hd = paint::Handling::new(Tool { ragged: 0.2, ..Tool::filbert(6.0) }).mixed(pal, 0.25).mix_jitter(0.03)
            .color(&seac).angle(|_, _| 0.0).angle_jitter(0.003).length(60.0, 180.0).coverage(4.0).pressure(0.6, 0.8)
            .dips(1, 0.6, 0.8).clip(true).threshold(0.05);
        c.work(&far, &hd, 75);
        if let Some(b) = st.blend() {
            c.work(&water, &b.angle(|_, _| 0.0).pressure(0.35, 0.45), 74);
        }
        c.dry();
        // swells: thin light lines of reflected sky and darker troughs,
        // close together in the distance, wider apart near; densest and
        // brightest in the glow's path
        let mut rg = Held::new(Tool::round_sable(1.4), 72);
        let mut y = HOR + 2.5;
        let mut k = 0;
        while y < 574.0 {
            let t = (y - HOR) / 140.0;
            let n = 5 + (rng.f() * 5.0) as usize;
            for _ in 0..n {
                // lines gather under the glow
                let x0 = if rng.chance(0.45) { GX + rng.normal() * (40.0 + 90.0 * t) - 60.0 } else { rng.range(-80.0, 980.0) };
                let len = rng.range(12.0, 70.0) * (0.5 + 1.2 * t);
                let xm = x0 + len * 0.5;
                let inpath = (-((xm - GX) / (60.0 + 110.0 * t)).powi(2)).exp();
                let light = rng.chance(0.7 + 0.25 * inpath);
                let col = if light {
                    mix(sea(xm, y), hex("#efdcb2"), (0.15 + 0.5 * inpath) * (1.0 - 0.6 * t), Mix::Light)
                } else {
                    mix(sea(xm, y), hex("#262b33"), 0.15 + 0.1 * t, Mix::Light)
                };
                rg.tool = Tool { ragged: 0.4, ..Tool::round_sable(0.6 + 2.0 * t) };
                rg.reload(pal.paint(col, 0.4), 0.45);
                let pts = vec![(x0, y), (xm, y + rng.normal() * 0.3), (x0 + len, y + rng.normal() * 0.3)];
                let g = Gesture::new(pts).pressure(rng.range(0.3, 0.5), 0.25).ramps(0.4, 0.5).shake(0.5);
                c.drag(&mut rg, &g, Some(&water));
            }
            y += 0.9 + 9.0 * t * t + rng.range(0.0, 1.5);
            k += 1;
        }
        let _ = k;
        c.dry();
    }
    if run.stage(&mut c, "sea") {
        return;
    }

    // ---- the shore: sand, wet at the waterline, then darker shingle
    {
        let land = Mask::from_fn(fr, |x, y| smoothstep(shore_y(x) - 1.5, shore_y(x) + 2.0, y));
        let hd = st.body().color(shore).angle(|_, _| 0.0).angle_jitter(0.05).length(30.0, 90.0).coverage(2.5).medium(0.3).clip(true).threshold(0.2);
        c.work(&land, &hd, 80);
        c.dry();
        let hd = st.body().color(shore).angle(|_, _| 0.0).angle_jitter(0.05).length(25.0, 70.0).coverage(3.5).medium(0.15).clip(true).threshold(0.2);
        c.work(&land, &hd, 83);
        if let Some(b) = st.blend() {
            c.work(&land, &b.angle(|_, _| 0.0).pressure(0.3, 0.45).coverage(1.5).clip(true), 87);
        }
        c.dry();
        // dry brush: a stiff flat with little lean paint, barely pressing,
        // so it catches only the tooth: the grain of the sand
        let db = paint::Handling::new(Tool { ragged: 0.6, ..Tool::hog_flat(5.0) })
            .mixed(pal, 0.2)
            .color(|x, y| mix(shore(x, y), hex("#8a7e6c"), 0.18, Mix::Light))
            .angle(|_, _| 0.0)
            .angle_jitter(0.05)
            .length(40.0, 120.0)
            .coverage(0.8)
            .pressure(0.12, 0.22)
            .ramps(0.3, 0.4)
            .dips(2, 0.35, 0.8)
            .threshold(0.5);
        c.work(&land.clone().mul_fn(|x, y| smoothstep(shore_y(x) + 14.0, shore_y(x) + 30.0, y)), &db, 88);
        // the wet strand at the water's edge takes the sky: a glassy band,
        // laid in horizontally and fused into the sand below
        let wet = Mask::from_fn(fr, |x, y| {
            let s0 = shore_y(x);
            smoothstep(s0 - 1.0, s0 + 1.5, y) * (1.0 - smoothstep(s0 + 5.0, s0 + 13.0 + 4.0 * (x * 0.011).sin(), y))
        });
        let hd = st.broad().color(|x, y| {
            let d = (y - shore_y(x)) / 16.0;
            mix(mix(sky(x, HOR - 60.0 - 200.0 * d), hex("#6d665c"), 0.5 + 0.4 * d, Mix::Light), sea(x, 470.0), 0.25, Mix::Light)
        }).angle(|_, _| 0.0).angle_jitter(0.004).length(30.0, 90.0).coverage(3.5).medium(0.3).pressure(0.5, 0.7).threshold(0.2).clip(true);
        let hd = paint::Handling { tool: Tool { ragged: 0.3, ..Tool::filbert(6.0) }, ..hd };
        c.work(&wet, &hd, 84);
        if let Some(b) = st.blend() {
            let b = paint::Handling { tool: Tool { pickup: 0.12, ..Tool::badger(12.0) }, ..b };
            c.work(&wet.clone().blur(3.0), &b.angle(|_, _| 0.0).pressure(0.3, 0.45).length(40.0, 120.0).threshold(0.1).clip(true), 85);
        }
        c.dry();
        // the last thin wave: a light line along the water's edge
        let mut b = Held::new(Tool::round_sable(1.6), 81);
        let pts: Vec<(f32, f32)> = (0..=25).map(|i| (i as f32 * 40.0 - 10.0, shore_y(i as f32 * 40.0 - 10.0) + 1.0)).collect();
        for seg in pts.windows(4).step_by(3) {
            b.reload(pal.paint(hex("#c6bca8"), 0.3), 0.5);
            stroke(&mut c, &mut b, seg, 0.4, 0.3, None);
        }
        // the wrack line: a broken dark band of dried seaweed where the
        // last high water left it, wandering along the strand
        let wrack = |x: f32| shore_y(x) + 30.0 + 7.0 * (x * 0.017 + 1.3).sin() + 3.0 * (x * 0.051).sin();
        let mut wb = Held::new(Tool { ragged: 0.7, ..Tool::round_sable(1.8) }, 86);
        let mut x = -10.0;
        while x < 1010.0 {
            let len = rng.range(12.0, 60.0);
            if rng.chance(0.75) {
                wb.reload(pal.paint(mix(hex("#302a20"), hex("#4a3f2e"), rng.f(), Mix::Light), 0.35), 0.45);
                let pts: Vec<(f32, f32)> = (0..=4).map(|i| {
                    let xx = x + len * i as f32 / 4.0;
                    (xx, wrack(xx) + rng.normal() * 0.8)
                }).collect();
                c.drag(&mut wb, &Gesture::new(pts).pressure(rng.range(0.3, 0.6), rng.range(0.15, 0.4)).ramps(0.3, 0.5), None);
            }
            x += len + rng.range(0.0, 14.0);
        }
        // shingle: pebbles, each a flat darker touch with a lighter top
        // edge, wider than tall, growing larger toward the bottom edge
        let mut sb = Held::new(Tool::round_sable(3.0), 82);
        for _ in 0..220 {
            let x = rng.range(0.0, 1000.0);
            let y0s = shore_y(x) + 26.0;
            let t = rng.f().powf(0.6);
            let y = y0s + (h - y0s) * t;
            let sz = 1.0 + 4.5 * t * rng.range(0.6, 1.3);
            let base = shore(x, y);
            let wide = sz * rng.range(0.6, 1.2);
            sb.tool = Tool { ragged: 0.3, ..Tool::round_sable(sz * 0.7) };
            sb.reload(pal.paint(mix(base, hex("#1b1612"), rng.range(0.15, 0.4), Mix::Light), 0.2), 0.5);
            c.drag(&mut sb, &Gesture::new(vec![(x - wide, y), (x + wide, y + rng.normal() * 0.3)]).pressure(0.6, 0.55).ramps(0.15, 0.3), None);
            if rng.chance(0.7) {
                sb.tool = Tool { ragged: 0.4, ..Tool::round_sable(sz * 0.35) };
                sb.reload(pal.paint(mix(base, hex("#8f8272"), rng.range(0.2, 0.4), Mix::Light), 0.35), 0.35);
                c.drag(&mut sb, &Gesture::new(vec![(x - wide * 0.8, y - sz * 0.25), (x + wide * 0.5, y - sz * 0.3)]).pressure(0.45, 0.3).ramps(0.15, 0.4), None);
            }
        }
        c.dry();
    }

    // ---- the brig, standing out to sea, and a far sail on the horizon
    paint_ship(&mut c, st.palette.clone(), SHIP, 1.0, 90);
    paint_ship(&mut c, st.palette.clone(), (176.0, HOR + 1.5), 0.28, 91);
    if run.stage(&mut c, "ships") {
        return;
    }

    // ---- boulders: dark granite erratics seen against the light. The body
    // in a dark tone, then the planes blocked in (the top taking the cool
    // sky, the flank turned to the glow a little warmer), fused lightly with
    // a small badger; then the grain, cracks and the dark seam at the sand.
    for (k, r) in rocks().iter().enumerate() {
        let m = rock_mask(fr, r);
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for &(x, y) in &r.pts {
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x);
            y1 = y1.max(y.min(h));
        }
        let rh = (y1 - y0).max(1.0);
        let rw = (x1 - x0).max(1.0);
        let small = r.planes.is_empty();
        let col = move |x: f32, y: f32| {
            let v = ((y - y0) / rh).clamp(0.0, 1.0);
            let side = ((x - x0) / rw).clamp(0.0, 1.0);
            let dark = if rh < 20.0 {
                gradient(&[(0.0, hex("#4d4640")), (1.0, hex("#2c2621"))], v, Mix::Light)
            } else {
                gradient(&[(0.0, hex("#4a433d")), (0.3, hex("#35302b")), (1.0, hex("#1a1612"))], v, Mix::Light)
            };
            mix(dark, hex("#16120f"), side * 0.2, Mix::Light)
        };
        let tool_w = if small { 2.5 } else { 7.0 };
        let hd = paint::Handling::new(Tool { lay: 0.8, push: 0.08, ragged: 0.35, ..Tool::filbert(tool_w) })
            .mixed(pal, 0.15)
            .mix_jitter(0.08)
            .color(col)
            .length(tool_w * 2.0, tool_w * 6.0)
            .coverage(3.5)
            .pressure(0.6, 0.9)
            .dips(2, 0.6, 0.6)
            .angle(|_, _| 1.35)
            .angle_jitter(0.25)
            .clip(true)
            .threshold(0.15);
        c.work(&m, &hd, 100 + k as u64);
        for (j, (poly, tone, ang)) in r.planes.iter().enumerate() {
            let pm = Mask::from_shape(fr, Shape::new().poly(poly)).blur(1.5).mul(&m);
            let base = hex(tone);
            let ang = *ang;
            let ph = paint::Handling::new(Tool { lay: 0.7, push: 0.06, ragged: 0.45, ..Tool::filbert(5.0) })
                .mixed(pal, 0.25)
                .mix_jitter(0.1)
                .color(move |x, _| mix(base, hex("#2a2521"), 0.25 * (((x * 0.13).sin() + 1.0) * 0.5), Mix::Light))
                .length(10.0, 30.0)
                .coverage(2.5)
                .pressure(0.5, 0.8)
                .dips(2, 0.5, 0.6)
                .angle(move |_, _| ang)
                .angle_jitter(0.2)
                .clip(true)
                .threshold(0.3);
            c.work(&pm, &ph, 170 + (k * 10 + j) as u64);
        }
        let bl = paint::Handling::new(Tool { pickup: 0.1, ..Tool::badger(tool_w * 1.6) })
            .length(tool_w * 2.0, tool_w * 5.0).coverage(1.0).pressure(0.25, 0.4).dips(3, 0.0, 0.9).blender()
            .angle(|_, _| 0.3).angle_jitter(0.8).clip(true).threshold(0.2);
        c.work(&m, &bl, 130 + k as u64);
        c.dry();
        if small {
            // a low stone: its shadow on the wet sand, and a thin lit top
            let mut b = Held::new(Tool { ragged: 0.4, ..Tool::round_sable(2.0) }, 140 + k as u64);
            b.load(pal.paint(hex("#221d18"), 0.2), 0.6);
            c.drag(&mut b, &Gesture::new(vec![(x0 + 1.0, y1 + 0.8), (x1 + 3.0, y1 + 1.2)]).pressure(0.6, 0.3).ramps(0.1, 0.4), None);
            let top: Vec<(f32, f32)> = r.pts[1..r.pts.len() - 1].iter().map(|&(x, y)| (x, y + 1.0)).collect();
            b.tool = Tool { ragged: 0.5, ..Tool::round_sable(1.2) };
            b.reload(pal.paint(hex("#524c46"), 0.55), 0.25);
            c.drag(&mut b, &Gesture::new(top).pressure(0.4, 0.25).ramps(0.2, 0.5), Some(&m));
        } else {
            // cracks and seams: a few dark rigger lines
            let mut rg = Held::new(Tool::rigger(0.9), 150 + k as u64);
            for _ in 0..3 {
                rg.reload(pal.paint(hex("#120f0c"), 0.2), 0.7);
                let sx = rng.range(x0 + rw * 0.2, x1 - rw * 0.2);
                let sy = rng.range(y0 + rh * 0.2, y0 + rh * 0.6);
                let pts = vec![(sx, sy), (sx + rng.range(-8.0, 8.0), sy + rh * 0.15), (sx + rng.range(-12.0, 12.0), sy + rh * 0.3)];
                c.drag(&mut rg, &Gesture::new(pts).pressure(0.6, 0.2).ramps(0.1, 0.5), Some(&m));
            }
            // the grain of the granite: small lean touches, light and dark
            let mut sp = Held::new(Tool { ragged: 0.3, ..Tool::round_sable(1.4) }, 160 + k as u64);
            for _ in 0..(rw * rh / 50.0) as usize {
                let (px, py) = (rng.range(x0, x1), rng.range(y0, y1));
                if m.data[fr.index(px, py)] < 0.9 {
                    continue;
                }
                let here = c.sample(px, py);
                let col = if rng.chance(0.5) { mix(here, hex("#8a8078"), 0.3, Mix::Light) } else { mix(here, hex("#0f0c0a"), 0.4, Mix::Light) };
                sp.reload(pal.paint(col, 0.4), 0.35);
                let a = rng.range(0.0, 6.28);
                c.drag(&mut sp, &Gesture::new(vec![(px, py), (px + a.cos() * 1.0, py + a.sin() * 1.0)]).pressure(0.5, 0.4).ramps(0.1, 0.4), Some(&m));
            }
        }
        c.dry();
    }
    if run.stage(&mut c, "rocks") {
        return;
    }

    // ---- the anchor, half sunk in the shingle
    paint_anchor(&mut c, pal, 92);

    // ---- the figure, last
    paint_woman(&mut c, pal, (548.0, 597.0), 66.0, 93);
    if run.stage(&mut c, "figure") {
        return;
    }

    run.finish(&mut c, &Finish::aged((0.25, 0.02)));
}

/// A brig seen from the quarter, standing out to sea (heading right, away):
/// dark hull with its sheer and a wale, two masts with three square sails
/// each, a jib, yards and stays. Against the glow the sails are a dim warm
/// grey, lit along their left edges. `s` scales it (1 = the near brig).
fn paint_ship(c: &mut Canvas, pal: paint::Palette, at: (f32, f32), s: f32, seed: u64) {
    let (x, y) = at;
    let fr = c.frame();
    let mut rng = Rng::new(seed);
    let line = pal.paint(hex("#27231f"), 0.15);
    let lit = pal.paint(hex("#a89680"), 0.3);
    let p = |u: f32, v: f32| (x + u * s, y - v * s);
    // hull: cut in to its shape with a small round, then the wale
    let hull_pts = [p(-24.0, 8.0), p(-10.0, 6.2), p(8.0, 6.0), p(22.0, 7.4), p(29.0, 9.0), p(24.0, 2.5), p(14.0, 0.0), p(-14.0, 0.0), p(-21.0, 2.0)];
    let hm = Mask::from_shape(fr, Shape::new().smooth_poly(&hull_pts));
    let hd = paint::Handling::new(Tool { ragged: 0.2, ..Tool::round_sable(2.2 * s.max(0.5)) })
        .mixed(&pal, 0.1).color(|_, _| hex("#1e1a17")).angle(|_, _| 0.0).length(6.0 * s, 16.0 * s).coverage(4.0)
        .pressure(0.7, 0.9).dips(2, 0.8, 0.6).clip(true).threshold(0.2);
    c.work(&hm, &hd, seed + 30);
    c.dry();
    if s > 0.5 {
        let mut w = Held::new(Tool { ragged: 0.4, ..Tool::round_sable(0.9 * s) }, seed + 31);
        w.load(pal.paint(hex("#5a4a3c"), 0.3), 0.5);
        c.drag(&mut w, &Gesture::new(vec![p(-22.0, 6.4), p(-8.0, 4.8), p(10.0, 4.6), p(25.0, 6.4)]).pressure(0.5, 0.35).ramps(0.2, 0.4).shake(0.3), Some(&hm));
    }
    // the reflection: broken dark touches under the hull
    let mut r = Held::new(Tool::round_sable(1.6 * s.max(0.4)), seed + 9);
    for i in 0..5 {
        r.reload(pal.paint(hex("#3a3a40"), 0.3), 0.5);
        let yy = y + (1.0 + i as f32 * 1.6) * s;
        let w = (16.0 - i as f32 * 2.5) * s;
        let x0 = x - w + rng.normal() * 1.5 * s;
        c.drag(&mut r, &Gesture::new(vec![(x0, yy), (x0 + 2.0 * w * rng.range(0.6, 1.0), yy)]).pressure(0.5, 0.35).ramps(0.2, 0.4), None);
    }
    // masts (main at the left, fore at the right), with a slight rake
    let masts = [(-6.0f32, 88.0f32), (11.0, 82.0)];
    let mut m = Held::new(Tool::rigger(0.9 * s.max(0.45)), seed + 1);
    for &(mx, top) in &masts {
        m.reload(line, 0.9);
        c.drag(&mut m, &Gesture::new(vec![p(mx, 5.0), p(mx - 1.0, top)]).pressure(0.7, 0.4).ramps(0.0, 0.2).shake(0.15), None);
    }
    c.dry();
    // square sails: course, topsail, topgallant on each mast; each a
    // trapezoid (the lower yard is longer) with a bellied foot, cut in and
    // filled with short vertical strokes modeled from the lit left edge to
    // the shadowed right, then fused lightly
    let tiers = [(0.09f32, 0.38f32, 9.8f32), (0.37, 0.63, 8.4), (0.62, 0.8, 6.0)];
    for (mi, &(mx, top)) in masts.iter().enumerate() {
        for (ti, &(a, bb, half)) in tiers.iter().enumerate() {
            let rake = |v: f32| mx - v / top;
            let (head, foot) = (top * bb, top * a);
            let (hw, fw) = (half * 0.9, half * 1.08);
            let (hx, fx) = (rake(head), rake(foot));
            let poly = [
                p(hx - hw, head),
                p(hx + hw, head),
                p(fx + fw, foot + 0.6),
                p(fx + fw * 0.5, foot - 0.5),
                p(fx, foot - 1.1),
                p(fx - fw * 0.5, foot - 0.5),
                p(fx - fw, foot + 0.6),
            ];
            let sm = Mask::from_shape(fr, Shape::new().poly(&poly));
            let (l, r_) = (x + (fx - fw) * s, x + (fx + fw) * s);
            let shade = mi as f32 * 0.22 + ti as f32 * 0.04;
            let col = move |px: f32, _py: f32| {
                let u = ((px - l) / (r_ - l)).clamp(0.0, 1.0);
                // light on the left, a darker hollow right of the middle,
                // the far edge a touch lighter again (the belly turning)
                let t = gradient(&[(0.0, hex("#72685c")), (0.25, hex("#564e46")), (0.7, hex("#3a3531")), (1.0, hex("#45403a"))], u, Mix::Light);
                mix(t, hex("#2a2623"), shade, Mix::Light)
            };
            let fill = paint::Handling::new(Tool { ragged: 0.25, ..Tool::round_sable(2.4 * s.max(0.4)) })
                .mixed(&pal, 0.08).mix_jitter(0.015).color(col).angle(|_, _| std::f32::consts::FRAC_PI_2).angle_jitter(0.08)
                .length(4.0 * s, 10.0 * s).coverage(5.0).pressure(0.7, 0.9).dips(2, 0.8, 0.7).clip(true).threshold(0.15);
            c.work(&sm, &fill, seed + 40 + (mi * 3 + ti) as u64);
            let bl = paint::Handling::new(Tool { pickup: 0.08, ..Tool::badger(4.0 * s.max(0.4)) })
                .length(4.0 * s, 9.0 * s).coverage(2.0).pressure(0.35, 0.45).dips(4, 0.0, 0.9).blender()
                .angle(|_, _| std::f32::consts::FRAC_PI_2).clip(true).threshold(0.2);
            c.work(&sm, &bl, seed + 60 + (mi * 3 + ti) as u64);
        }
    }
    c.dry();
    // jib and fore staysail forward of the foremast: triangles cut in
    let jib = [p(12.0, 76.0), p(33.0, 9.0), p(18.0, 12.0)];
    let jm = Mask::from_shape(fr, Shape::new().poly(&jib));
    let fill = paint::Handling::new(Tool { ragged: 0.25, ..Tool::round_sable(2.0 * s.max(0.4)) })
        .mixed(&pal, 0.08).color(|_, _| hex("#4a443e")).angle(|_, _| 1.2).length(4.0 * s, 10.0 * s).coverage(5.0)
        .pressure(0.7, 0.9).dips(2, 0.8, 0.7).clip(true).threshold(0.15);
    c.work(&jm, &fill, seed + 70);
    c.dry();
    // yards, stays, a pennant, and the glow along the sails' left edges
    let mut rg = Held::new(Tool::rigger(0.4 * s.max(0.7)), seed + 4);
    for &(mx, top) in &masts {
        for &(a, half) in &[(0.38f32, 9.8f32 * 0.95), (0.63, 8.4 * 0.95), (0.8, 6.0 * 0.95)] {
            rg.reload(line, 0.6);
            let v = top * a;
            let cx = mx - v / top;
            c.drag(&mut rg, &Gesture::line(p(cx - half, v - 0.3), p(cx + half, v + 0.2)).pressure(0.6, 0.6).ramps(0.0, 0.0).shake(0.1), None);
        }
    }
    rg.reload(line, 0.5);
    for (a, b2) in [(p(10.0, 82.0), p(33.0, 9.0)), (p(-7.0, 88.0), p(-23.0, 7.5)), (p(-7.0, 88.0), p(10.0, 82.0)), (p(-7.0, 70.0), p(-20.0, 7.0))] {
        c.drag(&mut rg, &Gesture::line(a, b2).pressure(0.45, 0.45).ramps(0.0, 0.0).shake(0.1), None);
    }
    let mut pn = Held::new(Tool::round_sable(1.3 * s.max(0.6)), seed + 5);
    pn.load(pal.paint(hex("#4e2a22"), 0.2), 0.6);
    c.drag(&mut pn, &Gesture::new(vec![p(-7.0, 88.0), p(-1.0, 89.5), p(5.0, 88.2)]).pressure(0.55, 0.12).ramps(0.0, 0.6), None);
    if s > 0.5 {
        let mut lb = Held::new(Tool { ragged: 0.5, ..Tool::round_sable(1.2 * s) }, seed + 6);
        let (mx, top) = masts[0];
        for &(a, bb, half) in &tiers {
            lb.reload(lit, 0.4);
            let (head, foot) = (top * bb, top * a);
            let ex0 = mx - head / top - half * 0.9 + 0.7;
            let ex1 = mx - foot / top - half * 1.08 + 0.7;
            c.drag(&mut lb, &Gesture::new(vec![p(ex0, head - 0.8), p(0.5 * (ex0 + ex1) + 0.4, 0.5 * (head + foot)), p(ex1, foot + 1.0)]).pressure(0.4, 0.3).ramps(0.1, 0.3), None);
        }
    }
    c.dry();
}

/// An old stocked anchor lying on the shingle, tilted up against a stone,
/// one arm buried, its ring rising against the pale water's edge. Dark iron
/// with rust, a warm glint of the sky along its upper edges.
fn paint_anchor(c: &mut Canvas, pal: &paint::Palette, seed: u64) {
    let iron = pal.paint(hex("#211a16"), 0.12);
    let rust = pal.paint(hex("#44352c"), 0.4);
    let glint = pal.paint(hex("#a8977e"), 0.35);
    let wood = pal.paint(hex("#2e2620"), 0.12);
    let crown: (f32, f32) = (226.0, 694.0);
    let top: (f32, f32) = (318.0, 602.0);
    let (dx, dy) = (top.0 - crown.0, top.1 - crown.1);
    let len = (dx * dx + dy * dy).sqrt();
    let (ux, uy) = (dx / len, dy / len);
    let (nx, ny) = (-uy, ux);
    let along = |t: f32, o: f32| (crown.0 + dx * t + nx * o, crown.1 + dy * t + ny * o);
    // shank: two strokes side by side, tapering toward the ring
    let mut b = Held::new(Tool::round_sable(6.0), seed);
    for o in [-1.2f32, 1.2] {
        b.reload(iron, 0.9);
        c.drag(&mut b, &Gesture::new(vec![along(0.0, o), along(0.5, o * 0.8), along(1.0, o * 0.6)]).pressure(0.95, 0.7).ramps(0.03, 0.08).shake(0.3), None);
    }
    // the arm that shows: from the crown sweeping right and up, ending in
    // a spade-shaped fluke
    let arm = [crown, (crown.0 + 20.0, crown.1 + 7.0), (crown.0 + 44.0, crown.1 + 2.0), (crown.0 + 58.0, crown.1 - 12.0)];
    b.reload(iron, 0.9);
    c.drag(&mut b, &Gesture::new(arm.to_vec()).pressure(1.0, 0.7).ramps(0.03, 0.15).shake(0.3), None);
    let mut f = Held::new(Tool { ragged: 0.1, ..Tool::filbert(11.0) }, seed + 1);
    f.load(iron, 1.0);
    c.drag(&mut f, &Gesture::new(vec![(crown.0 + 50.0, crown.1 - 3.0), (crown.0 + 57.0, crown.1 - 14.0), (crown.0 + 63.0, crown.1 - 24.0)]).pressure(1.0, 0.4).ramps(0.0, 0.5).shake(0.2), None);
    // the other arm goes down into the shingle: only its root shows
    b.reload(iron, 0.7);
    c.drag(&mut b, &Gesture::new(vec![crown, (crown.0 - 14.0, crown.1 + 2.0)]).pressure(0.9, 0.4).ramps(0.0, 0.5), None);
    c.dry();
    // the wooden stock across the shank below the ring, one end resting on
    // the ground
    let mut w = Held::new(Tool { ragged: 0.4, ..Tool::round_sable(5.0) }, seed + 2);
    let sc = along(0.86, 0.0);
    let half = 34.0;
    for o in [-0.8f32, 0.8] {
        w.reload(wood, 0.9);
        c.drag(&mut w, &Gesture::new(vec![(sc.0 - nx * half + ux * o, sc.1 - ny * half + uy * o), (sc.0 + ux * o, sc.1 + uy * o), (sc.0 + nx * half * 0.9 + ux * o, sc.1 + ny * half * 0.9 + uy * o)]).pressure(0.9, 0.85).ramps(0.05, 0.1).shake(0.4), None);
    }
    // iron bands on the stock
    b.tool = Tool::round_sable(2.0);
    for t in [-0.55f32, 0.5] {
        b.reload(iron, 0.6);
        let p = (sc.0 + nx * half * t, sc.1 + ny * half * t);
        c.drag(&mut b, &Gesture::new(vec![(p.0 - ux * 3.0, p.1 - uy * 3.0), (p.0 + ux * 3.0, p.1 + uy * 3.0)]).pressure(0.8, 0.8).ramps(0.0, 0.0), None);
    }
    c.dry();
    // the ring: a loop drawn with a rigger through the eye
    let mut r = Held::new(Tool::rigger(1.8), seed + 3);
    r.load(iron, 1.0);
    let rc = (top.0 + ux * 7.0, top.1 + uy * 7.0);
    let ring: Vec<(f32, f32)> = (0..=14).map(|i| {
        let a = i as f32 / 14.0 * std::f32::consts::TAU + 2.2;
        (rc.0 + a.cos() * 9.0, rc.1 + a.sin() * 8.0)
    }).collect();
    c.drag(&mut r, &Gesture::new(ring).pressure(0.8, 0.8).ramps(0.03, 0.03).shake(0.3), None);
    c.dry();
    // rust along the shank and arm; a glint of sky along the upper edges
    let mut g = Held::new(Tool { ragged: 0.6, ..Tool::round_sable(2.2) }, seed + 4);
    g.load(rust, 0.5);
    c.drag(&mut g, &Gesture::new(vec![along(0.1, -1.0), along(0.5, -0.8), along(0.8, -0.5)]).pressure(0.35, 0.2).ramps(0.3, 0.5), None);
    g.reload(rust, 0.45);
    c.drag(&mut g, &Gesture::new(vec![(crown.0 + 22.0, crown.1 + 4.0), (crown.0 + 44.0, crown.1 - 1.0)]).pressure(0.4, 0.25).ramps(0.2, 0.4), None);
    g.tool = Tool { ragged: 0.5, ..Tool::round_sable(1.4) };
    g.reload(glint, 0.4);
    c.drag(&mut g, &Gesture::new(vec![along(0.35, -3.6), along(0.7, -3.2), along(0.95, -2.6)]).pressure(0.35, 0.15).ramps(0.3, 0.5), None);
    g.reload(glint, 0.35);
    c.drag(&mut g, &Gesture::new(vec![(sc.0 - nx * half * 0.9, sc.1 - ny * half * 0.9 - 2.0), (sc.0 - nx * half * 0.3, sc.1 - ny * half * 0.3 - 2.5)]).pressure(0.35, 0.2).ramps(0.3, 0.5), None);
    c.dry();
}

/// A woman in a long dark high-waisted dress, seen from behind, standing
/// still, a dusky red shawl over her shoulders with its point down her back,
/// hair gathered up. Local gestures in figure heights (u across, v up),
/// like `figures.rs`. Head about an eighth of her height.
fn paint_woman(c: &mut Canvas, pal: &paint::Palette, at: (f32, f32), size: f32, seed: u64) {
    use paint::{Hand, Mark};
    let mut h = Hand::new(at, size, seed);
    h.tremor = 0.002;
    let dress = pal.paint(hex("#1d211e"), 0.1);
    let shawl = pal.paint(hex("#432721"), 0.12);
    let shawl_lit = pal.paint(hex("#6a4034"), 0.2);
    let hair = pal.paint(hex("#33251a"), 0.15);
    // skirt: from the high waist down, flaring a little to the hem, which
    // just touches the sand
    let mut b = h.take(Tool::round_sable, 0.07, dress, 1.0);
    for (i, (u0, u1)) in [(-0.055f32, -0.115f32), (0.055, 0.115), (-0.02, -0.045), (0.02, 0.045), (0.0, 0.0)].iter().enumerate() {
        if i % 2 == 0 {
            b.reload(dress, 1.0);
        }
        h.mark(c, &mut b, Mark { pts: &[(*u0, 0.66), ((u0 + u1) * 0.5, 0.33), (*u1, 0.012)], pressure: (0.65, 1.0), ramps: (0.04, 0.04) }, None);
    }
    b.reload(dress, 1.0);
    h.line(c, &mut b, &[(-0.12, 0.018), (0.0, 0.008), (0.12, 0.018)], 0.8, 0.8);
    // bodice and sleeves (arms hanging close, slightly out at the elbow)
    for u in [-0.035f32, 0.035, 0.0] {
        b.reload(dress, 0.9);
        h.mark(c, &mut b, Mark { pts: &[(u * 1.15, 0.84), (u, 0.74), (u * 1.2, 0.64)], pressure: (0.75, 0.75), ramps: (0.05, 0.1) }, None);
    }
    let mut sl = h.take(Tool::round_sable, 0.035, dress, 0.9);
    for s in [-1.0f32, 1.0] {
        sl.reload(dress, 0.9);
        h.mark(c, &mut sl, Mark { pts: &[(s * 0.075, 0.83), (s * 0.092, 0.72), (s * 0.086, 0.6)], pressure: (0.8, 0.6), ramps: (0.05, 0.2) }, None);
    }
    c.dry();
    // shawl: across the shoulders, then drawn down to a point at the small
    // of the back; its ends fall over the upper arms
    let mut sh = h.take(Tool::round_sable, 0.045, shawl, 1.0);
    h.mark(c, &mut sh, Mark { pts: &[(-0.09, 0.82), (-0.045, 0.852), (0.045, 0.852), (0.09, 0.82)], pressure: (0.85, 0.85), ramps: (0.05, 0.1) }, None);
    for s in [-1.0f32, 1.0] {
        sh.reload(shawl, 1.0);
        h.mark(c, &mut sh, Mark { pts: &[(s * 0.07, 0.84), (s * 0.035, 0.76), (s * 0.006, 0.66)], pressure: (0.9, 0.35), ramps: (0.05, 0.35) }, None);
        sh.reload(shawl, 0.8);
        h.mark(c, &mut sh, Mark { pts: &[(s * 0.085, 0.83), (s * 0.095, 0.76)], pressure: (0.7, 0.4), ramps: (0.05, 0.4) }, None);
    }
    sh.reload(shawl, 0.8);
    h.mark(c, &mut sh, Mark { pts: &[(0.0, 0.84), (0.0, 0.72)], pressure: (0.8, 0.5), ramps: (0.05, 0.3) }, None);
    c.dry();
    // the glow catches the left edge of the shawl a little
    let mut lb = h.take(Tool::round_sable, 0.014, shawl_lit, 0.35);
    h.mark(c, &mut lb, Mark { pts: &[(-0.08, 0.835), (-0.04, 0.858), (0.0, 0.862)], pressure: (0.4, 0.2), ramps: (0.2, 0.5) }, None);
    // the head, and the hair gathered in a knot
    let mut hd = h.take(Tool::round_sable, 0.085, hair, 0.8);
    h.mark(c, &mut hd, Mark { pts: &[(0.0, 0.875), (0.0, 0.935)], pressure: (0.9, 0.85), ramps: (0.1, 0.3) }, None);
    let mut kn = h.take(Tool::round_sable, 0.045, hair, 0.7);
    h.dab(c, &mut kn, 0.004, 0.955, 0.025, 0.0, 0.8);
    c.dry();
}
