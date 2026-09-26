//! "Winter Twilight with Megalith and Spruces" (R11 Winter Flash):
//! An original painting in the manner of Caspar David Friedrich.
//!
//! Sourced historical and artistic vocabulary:
//! - An ancient megalithic stone tomb (Hünengrab / dolmen) in a snowbound landscape,
//!   echoing Friedrich's "Hünengrab im Schnee" (1807) and "Walk at Dusk" (c. 1830).
//! - The dialogue between the ancient gnarled, stag-headed bare oak (bodily death,
//!   fallen antiquity) and resilient evergreen spruces (steadfast faith, eternal life).
//! - Double-layer ground: warm ochre underlayer, cool lead-white brushed top layer.
//! - Twilight sky stippled in thin layers: cool slate-blue and mauve zenith fading into
//!   a luminous winter afterglow along the low horizon [NG p.56; CATS p.127].
//! - Continuous modeled snowfield with wind-drifts, frozen tarn, and cool violet shadows.
//! - Fine upturned frozen grasses piercing the snow crust [NG p.56].
//! - Solitary wanderer (Rückenfigur) contemplating the ancient stones in the fading light.
//! - Distant crows wheeling above the bare oak ("The Tree of Crows", 1822) and
//!   the faint sickle of the crescent moon in the evening sky.

use paint::color::{Mix, mix};
use paint::fir::{Fir, FirHabit, Kind as FirKind};
use paint::form::Light;
use paint::{
    Apply, Canvas, Fbm, Form, Gesture, Ground, Habit, Held, Mask,
    Orient, Paint, Palette, Rgb, Rng, Sdf, Skeleton, Stipple, Style, Tool, Touch,
    gradient, hex, smoothstep,
};
use paintings::run::{Finish, Run};

const ASPECT: f32 = 1.40;
/// Horizon line in canvas units (y down).
const HORIZON: f32 = 414.0;

fn main() {
    let o = Run::new("r11_winter_flash");
    let mut rng = Rng::new(o.seed);

    // Friedrich's light winter ground: warm lower earth layer leveled with knife,
    // cool lead-white and chalk upper layer brushed [NG p.55; KÖR p.284].
    let st = Style {
        name: "Friedrich Winter",
        width_mm: 450.0,
        ground: vec![
            Ground {
                color: hex("#b89b78"),
                hiding: 0.85,
                um: 90.0,
                stiff: 0.3,
                apply: Apply::Knife { texture: 0.3 },
            },
            Ground {
                color: hex("#dcd6c8"),
                hiding: 0.85,
                um: 60.0,
                stiff: 0.4,
                apply: Apply::Brush,
            },
        ],
        palette: Palette::friedrich_early(),
        relief: (0.06, 0.006),
        ..Style::friedrich()
    };
    let pal = &st.palette;
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let (_w, h) = (c.width(), c.height());

    // ---------------------------------------------------------------- geometry
    // Far mountain ridge (Riesengebirge profile)
    let ridge_n = Fbm::new(o.seed as u32 + 11, 4, 160.0);
    let ridge = f.per_column(|x| {
        HORIZON - 26.0 + 12.0 * ridge_n.get(x, 0.0)
            - 16.0 * (-((x - 720.0) / 180.0).powi(2)).exp()
            + 8.0 * (-((x - 300.0) / 140.0).powi(2)).exp()
    });

    // Masks for major spatial zones
    let sky_m = Mask::from_fn(f, |x, y| 1.0 - smoothstep(ridge(x) - 0.5, ridge(x) + 1.5, y));
    let ridge_m = Mask::from_fn(f, |x, y| {
        smoothstep(ridge(x) - 1.0, ridge(x) + 1.0, y)
            * (1.0 - smoothstep(HORIZON + 2.0, HORIZON + 5.0, y))
    });
    // The continuous snowfield covers everything from the horizon line down
    let snow_all_m = Mask::from_fn(f, |_, y| smoothstep(HORIZON - 1.5, HORIZON + 1.0, y));

    // Sky color function: cool slate-blue zenith -> mauve -> pale ochre afterglow
    let sky_stops: [(f32, Rgb); 6] = [
        (0.00, hex("#47596f")), // slate blue
        (0.22, hex("#6a7788")), // atmospheric blue-gray
        (0.45, hex("#99939e")), // pale mauve
        (0.70, hex("#c4b7a4")), // warm transition
        (0.88, hex("#e2cfab")), // luminous evening glow
        (1.00, hex("#ebd9b5")), // horizon amber-rose
    ];
    let sky_drift = Fbm::new(o.seed as u32 + 21, 3, 350.0);
    let sky_color = move |x: f32, y: f32| -> Rgb {
        let t = (y / HORIZON + 0.02 * sky_drift.get(x * 0.4, y)).clamp(0.0, 1.0);
        gradient(&sky_stops, t, Mix::Light)
    };

    // ------------------------------------------------------------- stage: ground
    if o.stage("ground", &mut c, &mut rng) {
        // Friedrich's thin warm underpainting: establishes values and spatial masses [CATS p.127]
        let whole = Mask::from_fn(f, |_, _| 1.0);
        let under_val = move |x: f32, y: f32| {
            0.15 + 1.8 * smoothstep(HORIZON - 10.0, h, y)
                 + 0.9 * (-((x - 350.0) / 200.0).powi(2) - ((y - 530.0) / 120.0).powi(2)).exp()
        };
        let under_glaze = st.glaze(0.85)
            .color(|_, _| hex("#564230"))
            .angle(|_, _| 0.0)
            .load_at(under_val);
        c.work(&whole, &under_glaze, o.seed * 37 + 1);
        if let Some(b) = st.blend() {
            c.work(&whole, &b, o.seed * 37 + 2);
        }
        c.dry();
    }

    // ------------------------------------------------------------- stage: sky lay
    if o.stage("sky lay", &mut c, &mut rng) {
        // Broad thin lay-in, horizontal strokes
        let lay = st.broad()
            .color(move |x, y| mix(sky_color(x, y), hex("#75757d"), 0.05, Mix::Light))
            .angle(|_, _| 0.0)
            .coverage(4.2)
            .medium(0.35);
        c.work(&sky_m, &lay, o.seed * 43 + 1);

        if let Some(b) = st.blend() {
            c.work(&sky_m, &b.angle(|_, _| 0.0), o.seed * 43 + 2);
        }

        // Wet-into-wet stippling: breaks strokes and fuses layers into Friedrich's fine vibration
        let s1 = Stipple::new(Tool::stippler(2.8))
            .mixed(pal, 0.45)
            .color(sky_color)
            .coverage(|_, _| 2.2)
            .pressure(0.5, 0.85)
            .dips(22, 0.4, 0.5);
        c.stipple(&sky_m, &s1, o.seed * 43 + 3);
        c.dry();
    }

    // ------------------------------------------------------------- stage: sky glow
    if o.stage("sky glow", &mut c, &mut rng) {
        // Dry stipple pass: finer and lighter, intensifying the warm afterglow
        let glow_color = move |x: f32, y: f32| {
            mix(sky_color(x, y), hex("#f0deb8"), 0.08 + 0.25 * smoothstep(180.0, HORIZON, y), Mix::Light)
        };
        let s2 = Stipple::new(Tool::stippler(1.5))
            .mixed(pal, 0.52)
            .color(glow_color)
            .coverage(|_, y| 2.4 * smoothstep(140.0, HORIZON - 10.0, y))
            .pressure(0.45, 0.8)
            .dips(26, 0.35, 0.6);
        c.stipple(&sky_m, &s2, o.seed * 47 + 1);

        // Faint sickle of the waxing crescent moon in the quiet evening sky
        paint_crescent_moon(&mut c, pal, (670.0, 185.0), 7.5, o.seed * 47 + 7);
        c.dry();
    }

    // ------------------------------------------------------------- stage: far hills
    if o.stage("far hills", &mut c, &mut rng) {
        // Distant mountain ridge in cold atmospheric blue-gray
        let ridge_col = |_, y: f32| {
            let t = ((y - (HORIZON - 35.0)) / 40.0).clamp(0.0, 1.0);
            gradient(
                &[(0.0, hex("#5f6777")), (0.5, hex("#565d6c")), (1.0, hex("#474d5b"))],
                t,
                Mix::Light,
            )
        };
        let ridge_handling = st.body()
            .color(ridge_col)
            .angle(|_, _| 0.05)
            .angle_jitter(0.1)
            .length(25.0, 60.0)
            .coverage(4.0)
            .medium(0.25);
        c.work(&ridge_m, &ridge_handling, o.seed * 53 + 1);

        // Valley mist stippled across the foot of the mountains
        let mist_n = Fbm::new(o.seed as u32 + 57, 4, 180.0);
        let mist_cov = move |x: f32, y: f32| {
            let base = smoothstep(HORIZON - 22.0, HORIZON + 6.0, y);
            let banks = 0.3 * mist_n.get(x * 0.8, y * 2.0);
            (base + banks).clamp(0.0, 1.0) * 2.8
        };
        let mist = Stipple::new(Tool::stippler(2.2))
            .mixed(pal, 0.65)
            .color(|_, _| hex("#cfcad0"))
            .coverage(mist_cov)
            .pressure(0.4, 0.8)
            .dips(18, 0.3, 0.6)
            .aim(false);
        c.stipple(&ridge_m, &mist, o.seed * 53 + 2);
        c.dry();
    }

    // ------------------------------------------------------------- stage: snow field
    if o.stage("snow field", &mut c, &mut rng) {
        // Unified, continuous snow landscape from horizon to foreground
        let drift = Fbm::new(o.seed as u32 + 61, 4, 120.0);
        let knoll_fbm = Fbm::new(o.seed as u32 + 65, 3, 90.0);

        let snow_color = move |x: f32, y: f32| -> Rgb {
            let d = ((y - HORIZON) / (h - HORIZON)).clamp(0.0, 1.0);

            // Frozen tarn depression on the right
            let tarn_dx = (x - 700.0) / 135.0;
            let tarn_dy = (y - 468.0) / 34.0;
            let in_tarn = (1.0 - (tarn_dx * tarn_dx + tarn_dy * tarn_dy)).clamp(0.0, 1.0);

            // Central knoll elevation under the dolmen
            let knoll_h = (-((x - 350.0) / 210.0).powi(2) - ((y - 530.0) / 130.0).powi(2)).exp();

            // Color gradient with distance: warm horizon glow down to cool foreground blue-slate
            let base_snow = gradient(
                &[
                    (0.0, hex("#ded8c8")), // warm horizon glow
                    (0.2, hex("#cecbd0")),
                    (0.5, hex("#b4b6c6")),
                    (0.8, hex("#9ea2b5")),
                    (1.0, hex("#8d91a5")), // foreground cool shadow
                ],
                d,
                Mix::Light,
            );

            // Wind drift lighting: subtle slope variations
            let slope = drift.get(x * 0.24, y * 1.9) + 0.3 * knoll_fbm.get(x * 0.5, y * 0.5);
            let mut col = if slope >= 0.0 {
                mix(base_snow, hex("#ece6d6"), 0.55 * slope, Mix::Light)
            } else {
                mix(base_snow, hex("#7e829a"), 0.5 * -slope, Mix::Light)
            };

            // Knoll crest catches warm skim light from horizon
            if knoll_h > 0.3 {
                col = mix(col, hex("#ebe4d3"), (knoll_h - 0.3) * 0.5, Mix::Light);
            }

            // Directional soft shadow behind the dolmen on the knoll
            let dolmen_sh = smoothstep(350.0, 420.0, x) * (1.0 - smoothstep(460.0, 540.0, x))
                * smoothstep(520.0, 560.0, y) * (1.0 - smoothstep(610.0, 660.0, y));
            if dolmen_sh > 0.0 {
                col = mix(col, hex("#747990"), dolmen_sh * 0.65, Mix::Light);
            }

            // Soft shadow behind the oak
            let oak_sh = smoothstep(180.0, 230.0, x) * (1.0 - smoothstep(300.0, 370.0, x))
                * smoothstep(620.0, 640.0, y) * (1.0 - smoothstep(670.0, 710.0, y));
            if oak_sh > 0.0 {
                col = mix(col, hex("#747990"), oak_sh * 0.6, Mix::Light);
            }

            // If inside tarn, blend in flat icy reflection
            if in_tarn > 0.0 {
                let ice_col = mix(hex("#606874"), hex("#b4aea5"), 0.5, Mix::Light);
                mix(col, ice_col, in_tarn.powf(0.8) * 0.88, Mix::Light)
            } else {
                col
            }
        };

        let snow_handling = st.body()
            .color(snow_color)
            .angle(|x, y| {
                let knoll_dx = (x - 350.0) / 200.0;
                let knoll_dy = (y - 530.0) / 100.0;
                let angle_turn = -0.08 * knoll_dx * (-knoll_dy * knoll_dy).exp();
                0.02 + angle_turn
            })
            .angle_jitter(0.12)
            .length(30.0, 80.0)
            .coverage(4.8)
            .medium(0.25);
        c.work(&snow_all_m, &snow_handling, o.seed * 67 + 1);

        // Distant Gothic ruined chapel arch on the far hill
        paint_ruin(&mut c, pal, 222.0, 412.0, 38.0, o.seed * 71 + 3);
        c.dry();
    }

    // ------------------------------------------------------------- stage: spruces
    if o.stage("spruces", &mut c, &mut rng) {
        // Natural clustered grove of evergreen Norway spruces by the tarn
        let spruce_specs = [
            // Distant cluster (far bank of tarn, smaller)
            (615.0, 452.0, 76.0),
            (642.0, 448.0, 90.0),
            (670.0, 446.0, 82.0),
            // Midground group
            (700.0, 465.0, 136.0),
            (735.0, 458.0, 162.0),
            (775.0, 468.0, 140.0),
            (815.0, 476.0, 116.0),
            (850.0, 486.0, 94.0),
            // Near sentinel spruce (tall, prominent)
            (790.0, 502.0, 170.0),
        ];

        for (i, &(sx, sy, sh)) in spruce_specs.iter().enumerate() {
            let s_seed = o.seed * 109 + (i as u64) * 19;
            paint_spruce(&mut c, pal, (sx, sy), sh, s_seed);
        }
        c.dry();
    }

    // Form setup for monumental dolmen and surrounding erratics
    let upright1 = Sdf::block([295.0, 520.0, 20.0], [44.0, 95.0, 48.0], 8.0)
        .rough(2.8, 22.0, (o.seed + 201) as u32, true);
    let upright2 = Sdf::block([405.0, 525.0, -10.0], [46.0, 100.0, 50.0], 8.0)
        .rough(3.0, 24.0, (o.seed + 202) as u32, true);
    let upright3 = Sdf::block([350.0, 480.0, -35.0], [42.0, 105.0, 44.0], 7.0)
        .rough(2.5, 20.0, (o.seed + 203) as u32, true);
    let capstone = Sdf::block([350.0, 452.0, 5.0], [165.0, 42.0, 100.0], 10.0)
        .rough(3.2, 26.0, (o.seed + 204) as u32, true)
        .turn([350.0, 452.0, 5.0], 0.06, -0.05, 0.04);

    let boulder1 = Sdf::ellipsoid([215.0, 605.0, 25.0], [48.0, 36.0, 42.0])
        .rough(2.8, 16.0, (o.seed + 205) as u32, true);
    let boulder2 = Sdf::block([460.0, 600.0, 15.0], [52.0, 38.0, 44.0], 9.0)
        .rough(2.5, 18.0, (o.seed + 206) as u32, true);
    let boulder3 = Sdf::ellipsoid([540.0, 625.0, 10.0], [36.0, 26.0, 30.0])
        .rough(2.2, 14.0, (o.seed + 207) as u32, true);

    let mut stone_form = Form::new(f);
    stone_form.add(&upright1, 46.0);
    stone_form.add(&upright2, 49.0);
    stone_form.add(&upright3, 53.0);
    stone_form.add(&capstone, 45.0);
    stone_form.add(&boulder1, 40.0);
    stone_form.add(&boulder2, 42.0);
    stone_form.add(&boulder3, 44.0);

    let stone_light = Light::new((-0.2, -0.3), -0.4)
        .ambient(0.24)
        .bounce(0.42, [-0.1, 0.8, 0.3])
        .penumbra(0.08);
    stone_form.light(stone_light);
    let stones_m = stone_form.mask(|_| 1.0);

    // ------------------------------------------------------------- stage: dolmen & rocks
    if o.stage("dolmen & rocks", &mut c, &mut rng) {
        let stone_color = |x: f32, y: f32| -> Rgb {
            if let Some(s) = stone_form.sample(x, y) {
                let sh = &s.shade;
                let core = hex("#262329");
                let shadow = hex("#4e505e");
                let bounce = hex("#7c6e5c");
                let half = hex("#8a8276");
                let lit = hex("#bfb4a2");

                let dark = mix(core, shadow, sh.sky, Mix::Pigment);
                let dark = mix(dark, bounce, (sh.bounce * 0.95).min(0.6), Mix::Pigment);
                let light = mix(half, lit, sh.direct.powf(0.8), Mix::Pigment);
                mix(dark, light, sh.lit(0.12), Mix::Pigment)
            } else {
                hex("#383640")
            }
        };

        let fall_angle = |x: f32, y: f32| stone_form.sample(x, y).map_or(0.0, |s| s.fall());
        let stone_handling = st.body()
            .color(stone_color)
            .angle(fall_angle)
            .angle_jitter(0.12)
            .length(14.0, 32.0)
            .coverage(4.8)
            .medium(0.22);
        c.work(&stones_m, &stone_handling, o.seed * 83 + 1);

        // Deepen the interior chamber of the megalithic tomb (between uprights 1 and 2)
        let chamber_pts = vec![
            (330.0, 485.0),
            (350.0, 520.0),
            (370.0, 485.0),
        ];
        let mut ch_held = Held::new(Tool::round_sable(22.0), rng.next_u64());
        ch_held.load(pal.paint(hex("#141216"), 0.15).with_hiding(0.98), 0.9);
        c.drag(&mut ch_held, &Gesture::new(chamber_pts).pressure(0.9, 0.9).orient(Orient::Across), Some(&stones_m));

        // Plane breaks and crevices
        let breaks_m = stone_form.edges(0.45, 1.2, 1.8);
        let dark_crevice = pal.paint(hex("#121014"), 0.15).with_hiding(0.98);
        let lit_edge = pal.paint(hex("#c2b19e"), 0.2).with_hiding(0.85);

        for _ in 0..90 {
            let rx = rng.range(200.0, 560.0);
            let ry = rng.range(420.0, 640.0);
            if breaks_m.sample(rx, ry) > 0.4 {
                let b = stone_form.bend(rx, ry, 2.0);
                if b > 0.22 {
                    let mut a_held = Held::new(Tool::rigger(0.45), rng.next_u64());
                    a_held.load(lit_edge, 0.75);
                    c.touch(&mut a_held, &Touch::at(rx, ry).pressure(0.7), None);
                } else if b < -0.22 {
                    let mut j_held = Held::new(Tool::rigger(0.45), rng.next_u64());
                    j_held.load(dark_crevice, 0.85);
                    c.touch(&mut j_held, &Touch::at(rx, ry).pressure(0.8), None);
                }
            }
        }
        c.dry();
    }

    // Grow the ancient bare oak skeleton with vigorous, spreading crown
    let oak_sk = Habit {
        years: 28,
        vigor: 2.4,
        shoot_max: 5,
        branch_angle: 0.95,
        abort: 0.42,
        decline: 0.32,
        decay: 0.15,
        breakage: 0.14,
        lean: 0.13,
        trunk: 0.065,
        flare: 0.45,
        roots: 4,
        flat: 0.65,
        ..Habit::oak()
    }.grow((165.0, 630.0), 510.0, o.seed + 77);

    // ------------------------------------------------------------- stage: ancient oak
    if o.stage("ancient oak", &mut c, &mut rng) {
        paint_oak(&mut c, pal, &oak_sk, (165.0, 630.0), o.seed * 97 + 1);
        c.dry();
    }

    // ------------------------------------------------------------- stage: snow on motifs
    if o.stage("snow on motifs", &mut c, &mut rng) {
        let snow_cap_paint = pal.paint(hex("#ede8dd"), 0.15).with_hiding(0.96);
        let snow_crest_paint = pal.paint(hex("#f3efe6"), 0.1).with_hiding(0.98);
        let snow_shade_paint = pal.paint(hex("#979cb0"), 0.2).with_hiding(0.9);

        // 1. Thick sculpted snow cornice covering the top face of the dolmen capstone
        let cap_pts = vec![
            (262.0, 435.0),
            (305.0, 421.0),
            (350.0, 416.0),
            (395.0, 422.0),
            (438.0, 437.0),
        ];
        let mut sn_held = Held::new(Tool::round_sable(15.0), rng.next_u64());
        sn_held.load(snow_cap_paint, 1.0);
        c.drag(
            &mut sn_held,
            &Gesture::new(cap_pts)
                .pressure(0.75, 0.95)
                .ramps(0.2, 0.2)
                .orient(Orient::Across),
            None,
        );

        // Crisp windward crest highlight on the capstone snow cornice
        let crest_pts = vec![
            (275.0, 429.0),
            (345.0, 417.0),
            (425.0, 431.0),
        ];
        let mut cr_held = Held::new(Tool::rigger(1.2), rng.next_u64());
        cr_held.load(snow_crest_paint, 0.85);
        c.drag(&mut cr_held, &Gesture::new(crest_pts).pressure(0.6, 0.4).ramps(0.1, 0.2).orient(Orient::Across), None);

        // Soft undercut shadow beneath the capstone snow overhang
        let mut sn_sh_held = Held::new(Tool::rigger(1.4), rng.next_u64());
        sn_sh_held.load(snow_shade_paint, 0.7);
        c.drag(
            &mut sn_sh_held,
            &Gesture::new(vec![(266.0, 442.0), (350.0, 428.0), (434.0, 443.0)])
                .pressure(0.6, 0.6)
                .ramps(0.1, 0.1)
                .orient(Orient::Across),
            None,
        );

        // 2. Natural snow blankets draping over the upper dome of each erratic boulder
        for &(bx, _by, top_y, bw) in &[
            (215.0, 605.0, 570.0, 48.0), // Boulder 1 (top at y = 570)
            (460.0, 600.0, 563.0, 52.0), // Boulder 2 (top at y = 563)
            (540.0, 625.0, 600.0, 36.0), // Boulder 3 (top at y = 600)
        ] {
            let pts = vec![
                (bx - bw * 0.44, top_y + 8.0),
                (bx - bw * 0.15, top_y + 1.5),
                (bx + bw * 0.15, top_y + 1.5),
                (bx + bw * 0.44, top_y + 7.5),
            ];
            let mut b_held = Held::new(Tool::round_sable(bw * 0.28), rng.next_u64());
            b_held.load(snow_cap_paint, 0.95);
            c.drag(&mut b_held, &Gesture::new(pts).pressure(0.6, 0.85).ramps(0.2, 0.2).orient(Orient::Across), None);

            // Lit crest along top
            let cr_pts = vec![
                (bx - bw * 0.25, top_y + 2.0),
                (bx, top_y + 0.5),
                (bx + bw * 0.25, top_y + 1.8),
            ];
            let mut c_held = Held::new(Tool::rigger(1.0), rng.next_u64());
            c_held.load(snow_crest_paint, 0.85);
            c.drag(&mut c_held, &Gesture::new(cr_pts).pressure(0.6, 0.4).ramps(0.1, 0.1).orient(Orient::Across), None);
        }

        c.dry();
    }

    // ------------------------------------------------------------- stage: wanderer & details
    if o.stage("wanderer & details", &mut c, &mut rng) {
        // Solitary Wanderer (Rückenfigur) standing on the snow crest to the right
        paint_wanderer(&mut c, pal, (730.0, 582.0), 66.0, o.seed * 151 + 7);

        // Flock of distant crows soaring in the winter twilight sky above the bare oak
        paint_crows(&mut c, pal, (210.0, 200.0), o.seed * 157 + 3);

        // Frozen winter grasses and reeds piercing through the snow
        let grass_clusters = [
            (200.0, 638.0, 22, 17.0), // base of oak
            (240.0, 622.0, 14, 15.0), // near left boulder
            (290.0, 565.0, 18, 16.0), // foot of left upright
            (415.0, 578.0, 24, 18.0), // foot of right upright
            (465.0, 626.0, 16, 14.0), // near center boulder
            (535.0, 634.0, 14, 13.0), // near right boulder
            (620.0, 515.0, 12, 15.0), // tarn bank
            (685.0, 592.0, 16, 16.0), // near wanderer
        ];
        paint_grasses(&mut c, pal, &grass_clusters, o.seed * 163 + 11);
        c.dry();
    }

    o.finish(&mut c, &mut rng, &Finish::aged(st.relief));
}

// ============================================================================
// PAINTER HELPER FUNCTIONS (Written from Friedrich's sourced habits of hand)
// ============================================================================

/// Waxing crescent moon in the quiet evening sky.
fn paint_crescent_moon(c: &mut Canvas, pal: &Palette, at: (f32, f32), r: f32, seed: u64) {
    let mut rng = Rng::new(seed);
    let (mx, my) = at;
    let moon_col = pal.paint(hex("#eee9db"), 0.15).with_hiding(0.92);
    let glow_col = pal.paint(hex("#e5ddca"), 0.35).with_hiding(0.35);

    // Subtle ambient glow around the moon
    let mut gl_held = Held::new(Tool::round_sable(r * 2.2), rng.next_u64());
    gl_held.load(glow_col, 0.4);
    c.touch(&mut gl_held, &Touch::at(mx, my).pressure(0.4), None);

    // Waxing crescent sickle
    let arc_pts: Vec<(f32, f32)> = (0..8).map(|i| {
        let a = -1.2 + (i as f32) * 2.4 / 7.0;
        (mx + r * a.cos(), my + r * a.sin())
    }).collect();

    let mut m_held = Held::new(Tool::rigger(0.8), rng.next_u64());
    m_held.load(moon_col, 0.95);
    c.drag(&mut m_held, &Gesture::new(arc_pts).pressure(0.2, 0.8).ramps(0.2, 0.2).orient(Orient::Across), None);
}

/// Ruined gothic chapel gable and lancet arch on the distant hill.
fn paint_ruin(c: &mut Canvas, pal: &Palette, x0: f32, y_base: f32, h_ruin: f32, seed: u64) {
    let mut rng = Rng::new(seed);
    let stone = pal.paint(hex("#434652"), 0.2).with_hiding(0.95);
    let dark_crevice = pal.paint(hex("#272932"), 0.15).with_hiding(0.98);
    let snow = pal.paint(hex("#dad5cb"), 0.15).with_hiding(0.9);

    // Left wall pillar
    let mut left_held = Held::new(Tool::rigger(2.4), rng.next_u64());
    left_held.load(stone, 0.95);
    c.drag(
        &mut left_held,
        &Gesture::new(vec![(x0, y_base), (x0, y_base - h_ruin * 0.85)])
            .pressure(0.9, 0.7)
            .ramps(0.0, 0.1)
            .orient(Orient::Along),
        None,
    );

    // Right wall pillar & buttress
    let mut right_held = Held::new(Tool::rigger(2.8), rng.next_u64());
    right_held.load(stone, 0.95);
    c.drag(
        &mut right_held,
        &Gesture::new(vec![(x0 + 18.0, y_base), (x0 + 17.0, y_base - h_ruin * 0.95)])
            .pressure(0.9, 0.6)
            .ramps(0.0, 0.1)
            .orient(Orient::Along),
        None,
    );

    // Pointed gothic arch bridging across
    let arch_pts_l = vec![(x0 + 1.0, y_base - h_ruin * 0.6), (x0 + 8.5, y_base - h_ruin * 0.85)];
    let arch_pts_r = vec![(x0 + 16.0, y_base - h_ruin * 0.6), (x0 + 8.5, y_base - h_ruin * 0.85)];
    let mut arch_held = Held::new(Tool::rigger(1.4), rng.next_u64());
    arch_held.load(dark_crevice, 0.9);
    c.drag(&mut arch_held, &Gesture::new(arch_pts_l).pressure(0.7, 0.4).orient(Orient::Across), None);
    c.drag(&mut arch_held, &Gesture::new(arch_pts_r).pressure(0.7, 0.4).orient(Orient::Across), None);

    // Snow touches on ruined wall tops
    let mut sn_held = Held::new(Tool::rigger(0.8), rng.next_u64());
    sn_held.load(snow, 0.8);
    c.drag(&mut sn_held, &Gesture::new(vec![(x0 - 1.5, y_base - h_ruin * 0.86), (x0 + 1.5, y_base - h_ruin * 0.86)]).pressure(0.6, 0.6).orient(Orient::Across), None);
    c.drag(&mut sn_held, &Gesture::new(vec![(x0 + 15.5, y_base - h_ruin * 0.96), (x0 + 18.5, y_base - h_ruin * 0.96)]).pressure(0.6, 0.6).orient(Orient::Across), None);
}

/// Norway spruce (*Picea abies*) grown with botanical habit and painted with authentic needle hatches.
fn paint_spruce(
    c: &mut Canvas,
    pal: &Palette,
    at: (f32, f32),
    height: f32,
    seed: u64,
) {
    let mut rng = Rng::new(seed);
    let (sx, sy) = at;
    let env = [
        (sx, sy - height),
        (sx + height * 0.23, sy - height * 0.05),
        (sx - height * 0.23, sy - height * 0.05),
    ];
    let habit = FirHabit {
        fill: (0.75, 1.05),
        droop: 0.38,
        upturn: 0.32,
        ..FirHabit::spire()
    };
    let fir = Fir::grow(&env, Some((sx, sy)), &habit, (-0.2, -0.3, 0.4), seed);

    let dark_needles = pal.paint(hex("#131a15"), 0.1).with_hiding(0.97);
    let lit_needles = pal.paint(hex("#263428"), 0.12).with_hiding(0.94);
    let wood_dark = pal.paint(hex("#1b1714"), 0.1).with_hiding(0.98);
    let snow_paint = pal.paint(hex("#e5e2d8"), 0.12).with_hiding(0.9);
    let shade_paint = pal.paint(hex("#6a7082"), 0.35).with_hiding(0.4);

    // 0. Soft shadow pool under the spruce on the snow
    let sh_w = height * 0.20;
    let sh_pts = vec![(sx - sh_w * 0.5, sy + 1.0), (sx + sh_w * 0.6, sy + 1.5)];
    let mut sh_held = Held::new(Tool::round_sable(sh_w * 0.3), rng.next_u64());
    sh_held.load(shade_paint, 0.4);
    c.drag(&mut sh_held, &Gesture::new(sh_pts).pressure(0.5, 0.1).ramps(0.2, 0.3).orient(Orient::Across), None);

    // 1. Leader trunk: foot to apex, tapering continuously to a pointed rigger tip
    if fir.leader.len() >= 4 {
        let pts = &fir.leader; // foot to apex
        let n = pts.len();
        let mid = (n * 80) / 100;
        let w0 = fir.leader_w[0];
        let mut tr_held = Held::new(Tool::round_sable(w0.clamp(0.8, 3.2)), rng.next_u64());
        tr_held.load(wood_dark, 1.0);
        c.drag(&mut tr_held, &Gesture::new(pts[..mid].to_vec()).pressure(1.0, 0.25).ramps(0.0, 0.2).orient(Orient::Along), None);

        let mut tip_held = Held::new(Tool::rigger(0.35), rng.next_u64());
        tip_held.load(wood_dark, 0.8);
        c.drag(&mut tip_held, &Gesture::new(pts[mid..].to_vec()).pressure(0.4, 0.05).ramps(0.0, 0.5).orient(Orient::Along), None);
    }

    // 2. Boughs
    for b in &fir.boughs {
        if b.pts.len() < 2 { continue; }
        let bw = b.w[0].clamp(0.35, 1.8);
        let mut b_held = Held::new(Tool::rigger(bw), rng.next_u64());
        b_held.load(wood_dark, 0.85);
        c.drag(&mut b_held, &Gesture::new(b.pts.clone()).pressure(0.8, 0.2).ramps(0.0, 0.3).orient(Orient::Across), None);
    }

    // 3. Foliage needle hatches
    for h in &fir.strokes {
        let pts = h.pts.to_vec();
        let col = if h.lit > 0.5 { lit_needles } else { dark_needles };
        let hw = h.w.clamp(0.28, 0.65);
        let mut h_held = Held::new(Tool::rigger(hw), rng.next_u64());
        h_held.load(col, 0.75);
        c.drag(&mut h_held, &Gesture::new(pts).pressure(0.75, 0.1).ramps(0.0, 0.4).orient(Orient::Across), None);

        // Snow on top-facing hatches
        if h.kind == FirKind::Top && h.lit > 0.45 && rng.chance(0.65) {
            let sn_pts = vec![h.pts[0], h.pts[1]];
            let mut sn_held = Held::new(Tool::rigger(hw * 0.9), rng.next_u64());
            sn_held.load(snow_paint, 0.6);
            c.drag(&mut sn_held, &Gesture::new(sn_pts).pressure(0.5, 0.2).ramps(0.1, 0.3).orient(Orient::Across), None);
        }
    }

    // 4. Young apical needle cluster at the apex
    let apex_y = sy - height;
    for _ in 0..4 {
        let a = rng.range(-0.35, 0.35);
        let l = rng.range(2.0, 5.0);
        let pts = vec![(sx, apex_y + l * 0.5), (sx + a * l, apex_y - l * 0.4)];
        let mut n_held = Held::new(Tool::rigger(0.35), rng.next_u64());
        n_held.load(dark_needles, 0.85);
        c.drag(&mut n_held, &Gesture::new(pts).pressure(0.6, 0.1).ramps(0.0, 0.4).orient(Orient::Across), None);
    }
}

/// The ancient bare oak tree: limb following, tapering sable/rigger, bark, splinters, crotch snow.
fn paint_oak(
    c: &mut Canvas,
    pal: &Palette,
    sk: &Skeleton,
    root_at: (f32, f32),
    seed: u64,
) {
    let mut rng = Rng::new(seed);
    let dark_bark = pal.paint(hex("#201b17"), 0.1).with_hiding(0.98);
    let dead_bark = pal.paint(hex("#544e48"), 0.15).with_hiding(0.95);
    let wood_splinter = pal.paint(hex("#9e907a"), 0.2).with_hiding(0.92);
    let lit_streak = pal.paint(hex("#827464"), 0.25).with_hiding(0.75);
    let snow_pillow = pal.paint(hex("#eae6dd"), 0.15).with_hiding(0.92);

    // 0. Spreading buttress roots anchoring the oak into the rocky snow knoll
    let (rx, ry) = root_at;
    let root_paths = [
        vec![(rx, ry), (rx - 25.0, ry + 12.0), (rx - 55.0, ry + 18.0)],
        vec![(rx, ry), (rx + 8.0, ry + 18.0), (rx + 18.0, ry + 28.0)],
        vec![(rx, ry), (rx + 28.0, ry + 8.0), (rx + 58.0, ry + 14.0)],
    ];
    for rp in root_paths {
        let mut r_held = Held::new(Tool::round_sable(5.5), rng.next_u64());
        r_held.load(dark_bark, 1.0);
        c.drag(&mut r_held, &Gesture::new(rp).pressure(1.0, 0.3).ramps(0.0, 0.3).orient(Orient::Along), None);
    }

    // Filter limbs to keep the oak crown framing the left and upper sky
    let limbs: Vec<_> = sk.limbs.iter().filter(|l| {
        if l.is_empty() { return false; }
        let tip = *l.pts.last().unwrap();
        // Skip stray branch that pokes into the sky directly above the capstone
        !(tip.0 > 260.0 && tip.0 < 430.0 && tip.1 < 440.0)
    }).collect();

    // 1. Draw each limb, base to tip
    for l in &limbs {
        let n = l.pts.len();
        if n < 2 { continue; }

        let w_base = l.w[0];
        let w_tip = *l.w.last().unwrap();

        let brush_w = w_base.clamp(0.4, 20.0);
        let tool = if brush_w > 1.8 {
            Tool::round_sable(brush_w * 0.85)
        } else {
            Tool::rigger(brush_w.max(0.35))
        };

        let paint = if l.dead || l.dead_from == 0 { dead_bark } else { dark_bark };
        let mut held = Held::new(tool, rng.next_u64());
        held.load(paint, 0.9);

        let p0 = 1.0;
        let p1 = (w_tip / w_base).clamp(0.1, 1.0);
        let g = Gesture::new(l.pts.clone())
            .pressure(p0, p1)
            .ramps(0.0, if l.broken { 0.0 } else { 0.35 })
            .orient(Orient::Across)
            .shake(0.35);
        c.drag(&mut held, &g, None);

        // Splintered break if limb is broken
        if l.broken {
            let tip = *l.pts.last().unwrap();
            let prev = l.pts[n - 2];
            let dir = (tip.0 - prev.0, tip.1 - prev.1);
            let d_len = (dir.0 * dir.0 + dir.1 * dir.1).sqrt().max(1e-4);
            let u_dir = (dir.0 / d_len, dir.1 / d_len);

            for s in 0..3 {
                let s_len = (w_tip * 1.8 * (0.8 + 0.4 * s as f32)).clamp(1.5, 8.0);
                let lateral = (s as f32 - 1.0) * w_tip * 0.4;
                let s_pts = vec![
                    tip,
                    (tip.0 + u_dir.0 * s_len + lateral, tip.1 + u_dir.1 * s_len),
                ];
                let col = if s == 1 { wood_splinter } else { dead_bark };
                let mut sp_held = Held::new(Tool::rigger(0.4), rng.next_u64());
                sp_held.load(col, 0.8);
                c.drag(
                    &mut sp_held,
                    &Gesture::new(s_pts).pressure(0.8, 0.1).ramps(0.0, 0.4).orient(Orient::Across),
                    None,
                );
            }
        }
    }

    c.dry();

    // 2. Lean dry-brush lit streaks on big limbs facing the twilight glow
    for l in limbs.iter().filter(|l| l.w[0] > 2.5 && !l.root) {
        let n = l.pts.len();
        if n < 3 { continue; }
        let lit_pts: Vec<(f32, f32)> = l.pts.iter().enumerate().map(|(i, &(x, y))| {
            let w = l.w[i];
            (x + w * 0.3, y)
        }).collect();

        let mut lit_held = Held::new(Tool::round_sable(1.2), rng.next_u64());
        lit_held.load(lit_streak, 0.45);
        let g = Gesture::new(lit_pts).pressure(0.5, 0.2).ramps(0.2, 0.3).orient(Orient::Across);
        c.drag(&mut lit_held, &g, None);
    }

    // 3. Snow settled in branch crotches and on horizontal limbs
    for l in limbs.iter().filter(|l| l.w[0] > 1.8 && !l.root) {
        let n = l.pts.len();
        for i in 1..n - 1 {
            let (p0, p1, p2) = (l.pts[i - 1], l.pts[i], l.pts[i + 1]);
            let dx = p2.0 - p0.0;
            let dy = p2.1 - p0.1;
            if dx.abs() > dy.abs() * 0.8 && rng.chance(0.65) {
                let w = l.w[i];
                let snow_pts = vec![
                    (p1.0 - dx * 0.3, p1.1 - w * 0.45),
                    (p1.0, p1.1 - w * 0.55),
                    (p1.0 + dx * 0.3, p1.1 - w * 0.45),
                ];
                let mut sn_held = Held::new(Tool::round_sable((w * 0.4).clamp(0.5, 2.5)), rng.next_u64());
                sn_held.load(snow_pillow, 0.7);
                c.drag(
                    &mut sn_held,
                    &Gesture::new(snow_pts).pressure(0.4, 0.6).ramps(0.2, 0.2).orient(Orient::Across),
                    None,
                );
            }
        }
    }
}

/// A flock of distant crows wheeling above the bare oak ("The Tree of Crows", 1822).
fn paint_crows(c: &mut Canvas, pal: &Palette, center: (f32, f32), seed: u64) {
    let mut rng = Rng::new(seed);
    let crow_paint = pal.paint(hex("#161418"), 0.1).with_hiding(0.98);

    let crow_positions = [
        (-40.0, -35.0, 2.4),
        (-15.0, -50.0, 1.8),
        (20.0, -42.0, 2.1),
        (55.0, -25.0, 1.6),
        (80.0, -10.0, 1.4),
        (-25.0, -15.0, 2.2),
        (35.0, -8.0, 1.9),
    ];

    for (dx, dy, span) in crow_positions {
        let cx = center.0 + dx + rng.range(-6.0, 6.0);
        let cy = center.1 + dy + rng.range(-4.0, 4.0);
        let bank = rng.range(-0.3, 0.3);

        let left_wing = vec![
            (cx - span, cy - span * (0.45 + bank)),
            (cx, cy),
        ];
        let right_wing = vec![
            (cx, cy),
            (cx + span, cy - span * (0.45 - bank)),
        ];

        let mut cr_held = Held::new(Tool::rigger(0.35), rng.next_u64());
        cr_held.load(crow_paint, 0.95);
        c.drag(&mut cr_held, &Gesture::new(left_wing).pressure(0.7, 0.9).ramps(0.2, 0.0).orient(Orient::Across), None);
        c.drag(&mut cr_held, &Gesture::new(right_wing).pressure(0.9, 0.7).ramps(0.0, 0.2).orient(Orient::Across), None);
        c.touch(&mut cr_held, &Touch::at(cx, cy).pressure(0.6), None);
    }
}

/// The solitary wanderer (Rückenfigur) in traditional German traveler's dress with walking staff.
fn paint_wanderer(
    c: &mut Canvas,
    pal: &Palette,
    at: (f32, f32),
    height: f32,
    seed: u64,
) {
    let mut rng = Rng::new(seed);
    let cape_body = pal.paint(hex("#242723"), 0.1).with_hiding(0.98);
    let dark_boot = pal.paint(hex("#151312"), 0.1).with_hiding(0.98);
    let hat_paint = pal.paint(hex("#171615"), 0.1).with_hiding(0.98);
    let staff_paint = pal.paint(hex("#423525"), 0.15).with_hiding(0.95);
    let rim_light = pal.paint(hex("#9e9786"), 0.25).with_hiding(0.7);
    let shadow_snow = pal.paint(hex("#7d8498"), 0.4).with_hiding(0.45);
    let snow_dust = pal.paint(hex("#ded9cc"), 0.15).with_hiding(0.85);

    let (x0, y0) = at;

    // 1. Cast shadow on snow (trailing toward the right-forward)
    let sh_pts = vec![
        (x0, y0),
        (x0 + height * 0.35, y0 + height * 0.12),
        (x0 + height * 0.7, y0 + height * 0.22),
    ];
    let mut sh_held = Held::new(Tool::round_sable(height * 0.12), rng.next_u64());
    sh_held.load(shadow_snow, 0.6);
    c.drag(
        &mut sh_held,
        &Gesture::new(sh_pts).pressure(0.8, 0.2).ramps(0.1, 0.4).orient(Orient::Across),
        None,
    );

    // 2. Boots and legs
    let leg_w = (height * 0.055).clamp(0.6, 2.8);
    for dx in [-height * 0.035, height * 0.03] {
        let leg_pts = vec![
            (x0 + dx, y0 - height * 0.35),
            (x0 + dx, y0 - height * 0.1),
            (x0 + dx - height * 0.015, y0),
        ];
        let mut l_held = Held::new(Tool::round_sable(leg_w), rng.next_u64());
        l_held.load(dark_boot, 0.9);
        c.drag(&mut l_held, &Gesture::new(leg_pts).pressure(0.9, 0.8).orient(Orient::Along), None);
    }

    // 3. Walking staff (slender, planted firmly into the snow to the left)
    let staff_pts = vec![
        (x0 - height * 0.15, y0 - height * 0.72),
        (x0 - height * 0.17, y0 - height * 0.35),
        (x0 - height * 0.19, y0 + height * 0.02),
    ];
    let mut st_held = Held::new(Tool::rigger(0.42), rng.next_u64());
    st_held.load(staff_paint, 0.95);
    c.drag(&mut st_held, &Gesture::new(staff_pts).pressure(0.85, 0.75).orient(Orient::Along), None);

    // 4. Heavy woolen traveler's cape (smooth draped silhouette)
    let cape_pts = vec![
        (x0, y0 - height * 0.82),
        (x0 - height * 0.08, y0 - height * 0.55),
        (x0 - height * 0.11, y0 - height * 0.30),
    ];
    let mut c_held = Held::new(Tool::round_sable(height * 0.16), rng.next_u64());
    c_held.load(cape_body, 1.0);
    c.drag(&mut c_held, &Gesture::new(cape_pts).pressure(0.6, 0.95).ramps(0.1, 0.1).orient(Orient::Across), None);

    let cape_pts2 = vec![
        (x0, y0 - height * 0.82),
        (x0 + height * 0.07, y0 - height * 0.55),
        (x0 + height * 0.09, y0 - height * 0.30),
    ];
    let mut c_held2 = Held::new(Tool::round_sable(height * 0.16), rng.next_u64());
    c_held2.load(cape_body, 1.0);
    c.drag(&mut c_held2, &Gesture::new(cape_pts2).pressure(0.6, 0.95).ramps(0.1, 0.1).orient(Orient::Across), None);

    // Smooth hem line across bottom of cape
    let hem_pts = vec![
        (x0 - height * 0.11, y0 - height * 0.30),
        (x0, y0 - height * 0.29),
        (x0 + height * 0.09, y0 - height * 0.30),
    ];
    let mut hem_held = Held::new(Tool::round_sable(height * 0.08), rng.next_u64());
    hem_held.load(cape_body, 1.0);
    c.drag(&mut hem_held, &Gesture::new(hem_pts).pressure(0.8, 0.8).orient(Orient::Across), None);

    // 5. Head and Altdeutsche Tracht velvet cap / beret
    let mut hd_held = Held::new(Tool::round_sable(height * 0.08), rng.next_u64());
    hd_held.load(dark_boot, 0.85);
    c.touch(&mut hd_held, &Touch::at(x0, y0 - height * 0.88).pressure(0.75), None);

    let hat_pts = vec![
        (x0 - height * 0.08, y0 - height * 0.92),
        (x0, y0 - height * 0.935),
        (x0 + height * 0.07, y0 - height * 0.925),
    ];
    let mut hat_held = Held::new(Tool::round_sable(height * 0.07), rng.next_u64());
    hat_held.load(hat_paint, 0.95);
    c.drag(&mut hat_held, &Gesture::new(hat_pts).pressure(0.85, 0.7).orient(Orient::Across), None);

    // 6. Twilight rim light along shoulder and hat
    let rim_pts = vec![
        (x0 - height * 0.07, y0 - height * 0.93),
        (x0 - height * 0.06, y0 - height * 0.82),
        (x0 - height * 0.09, y0 - height * 0.55),
    ];
    let mut rim_held = Held::new(Tool::rigger(0.35), rng.next_u64());
    rim_held.load(rim_light, 0.5);
    c.drag(&mut rim_held, &Gesture::new(rim_pts).pressure(0.4, 0.3).ramps(0.1, 0.2).orient(Orient::Across), None);

    // 7. Small snow dust at boot feet
    let mut sn_held = Held::new(Tool::rigger(0.4), rng.next_u64());
    sn_held.load(snow_dust, 0.6);
    c.drag(&mut sn_held, &Gesture::new(vec![(x0 - height * 0.06, y0 + 1.0), (x0 + height * 0.05, y0 + 1.0)]).pressure(0.4, 0.4).orient(Orient::Across), None);
}

/// Frozen grasses and reeds piercing the snow (Friedrich's fine upturning strokes [NG p.56]).
fn paint_grasses(
    c: &mut Canvas,
    pal: &Palette,
    clusters: &[(f32, f32, usize, f32)],
    seed: u64,
) {
    let mut rng = Rng::new(seed);
    let umber = pal.paint(hex("#4e3b26"), 0.15).with_hiding(0.95);
    let ochre = pal.paint(hex("#7b643a"), 0.18).with_hiding(0.92);
    let dark = pal.paint(hex("#2a2016"), 0.15).with_hiding(0.96);

    for &(cx, cy, count, h) in clusters {
        for _ in 0..count {
            let x = cx + rng.range(-15.0, 15.0);
            let y = cy + rng.range(-4.0, 6.0);
            let gh = h * rng.range(0.7, 1.3);
            let lean = rng.range(-0.4, 0.4);
            let curve = rng.range(-0.3, 0.3);

            let tip_x = x + gh * (lean + curve);
            let mid_x = x + gh * lean * 0.4;
            let tip_y = y - gh;
            let mid_y = y - gh * 0.55;

            let pts = vec![(x, y), (mid_x, mid_y), (tip_x, tip_y)];
            let col = if rng.chance(0.5) { umber } else if rng.chance(0.6) { ochre } else { dark };
            let mut g_held = Held::new(Tool::rigger(0.35), rng.next_u64());
            g_held.load(col, 0.7);
            c.drag(
                &mut g_held,
                &Gesture::new(pts)
                    .pressure(0.8, 0.1)
                    .ramps(0.0, 0.4)
                    .orient(Orient::Across),
                None,
            );
        }
    }
}
