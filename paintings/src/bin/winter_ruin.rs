//! "Winter Evening, the Way to the Ruined Choir": an original winter landscape
//! in the manner of Caspar David Friedrich, painted from what is known of his
//! materials, method and motifs (no pictures). See notes/round8/arm2/notes.md.
//!
//! Every motif (ruin, spruces, oak, boulder, traveler, crows, grass) is
//! written here as brush gestures; the engine gives the canvas, paint,
//! brushes and time.

use paint::color::mix;
use paint::{Canvas, Fbm, Gesture, Held, Mask, Mix, Rgb, Rng, Stipple, Style, Tool, gradient, hex, smoothstep};
use paintings::run::Finish;

/// Lighter (dv > 0) or darker, keeping the hue.
fn shift_l(c: Rgb, dv: f32) -> Rgb {
    let k = 1.0 + dv;
    [c[0] * k, c[1] * k, c[2] * k]
}

const ASPECT: f32 = 1.4;

fn main() {
    let o = paintings::run::Run::new("winter_ruin");
    let mut rng = Rng::new(o.seed);
    let seed = o.seed as u32;
    let st = Style::friedrich();
    let pal = &st.palette;
    let mut c = o.canvas(|| st.prepare(o.width, ASPECT, o.seed));
    let f = c.frame();
    let h = c.height();

    // ---- the lay of the land (units, y down)
    let n1 = Fbm::new(seed + 1, 4, 150.0);
    let n2 = Fbm::new(seed + 2, 5, 70.0);
    // the far edge of the snow field
    let hz = f.per_column(move |x: f32| 468.0 + 2.0 * n1.get(x, 0.0) - 3.0 * smoothstep(300.0, 700.0, x));
    // low wooded rises far off, left and right, almost lost in the haze
    let hill = f.per_column(move |x: f32| {
        468.0 - 3.0 - 11.0 * (-((x - 40.0) / 200.0).powi(2)).exp() - 9.0 * (-((x - 960.0) / 170.0).powi(2)).exp() - 4.0 * n2.get(x, 3.0).abs() - 3.0 * smoothstep(300.0, 700.0, x)
    });
    let soft = |d: f32, e: f32| smoothstep(-e, e, d);
    let sky = Mask::from_fn(f, move |x, y| 1.0 - soft(y - hill(x) - 3.0, 0.8));

    // ---- sky: a low overcast lid, violet gray above, opening to a pale cold
    // band of evening light near the horizon, faintly rose above it
    let sky_stops: [(f32, Rgb); 6] = [
        (0.00, hex("#5f6275")),
        (0.25, hex("#7c7c8c")),
        (0.45, hex("#a39ca2")),
        (0.58, hex("#c6b3ad")),
        (0.64, hex("#dac9b7")),
        (0.68, hex("#e4dac6")),
    ];
    let drift = Fbm::new(seed + 10, 4, 400.0);
    let sky_col = move |x: f32, y: f32| gradient(&sky_stops, (y / h + 0.02 * drift.get(x * 0.4, y * 1.5)).clamp(0.0, 0.68), Mix::Pigment);

    if o.stage("sky", &mut c, &mut rng) {
        // laid in thin with the broad filbert, level strokes a shade duller
        // than the end, fused with the badger, then stippled while wet
        let lay = st.broad().color(move |x, y| mix(sky_col(x, y), hex("#6f6a70"), 0.08, Mix::Pigment)).angle(move |x, y| 0.03 * drift.get(x, y * 3.0)).coverage(4.0).medium(0.35);
        c.work(&sky, &lay, o.seed * 100 + 1);
        if let Some(b) = st.blend() {
            c.work(&sky, &b.angle(|_, _| 0.0), o.seed * 100 + 2);
        }
        let s1 = Stipple::new(Tool::stippler(3.0)).mixed(pal, 0.45).color(sky_col).coverage(|_, _| 2.0).pressure(0.5, 0.85).dips(20, 0.4, 0.5);
        c.stipple(&sky, &s1, o.seed * 100 + 3);
        // stratus: long level streaks drawn into the wet lay-in with a
        // lean filbert, gray-violet undersides above the light band, a few
        // lit warm ones low down; the badger fuses them after
        // (a soft brush that neither ploughs nor lifts much: a hog filbert
        // drawn through the wet lay-in pushed it aside to the red ground)
        let mut fil = Held::new(Tool { ragged: 0.35, push: 0.0, pickup: 0.03, stiffness: 0.3, ..Tool::filbert(9.0) }, 21);
        for i in 0..13 {
            let v = rng.range(0.28, 0.6);
            let y = h * v;
            let len = rng.range(140.0, 460.0) * (0.6 + v);
            let x0 = rng.range(-150.0, 1000.0 - len * 0.4);
            let col = if v < 0.48 || i % 3 != 0 {
                mix(sky_col(x0, y), hex("#57586a"), rng.range(0.2, 0.45), Mix::Pigment)
            } else {
                mix(sky_col(x0, y), hex("#eadcc6"), rng.range(0.3, 0.6), Mix::Pigment)
            };
            fil.reload(pal.paint(col, 0.35), rng.range(0.45, 0.7));
            let sag = rng.range(-2.0, 2.0);
            let pts = vec![(x0, y), (x0 + len * 0.3, y + sag), (x0 + len * 0.65, y + sag * 0.4 + rng.range(-1.5, 1.5)), (x0 + len, y + rng.range(-2.0, 1.0))];
            c.drag(&mut fil, &Gesture::new(pts).pressure(rng.range(0.35, 0.6), rng.range(0.2, 0.4)).ramps(0.25, 0.4).shake(0.3), Some(&sky));
        }
        // (no badger over the stippled sky: it lifted the wet paint in
        // streaks down to the red ground)
        c.dry();
        // dry: a finer, lighter stipple, denser toward the light band
        let glow = move |x: f32, y: f32| mix(sky_col(x, y), hex("#efe4cc"), 0.05 + 0.25 * smoothstep(200.0, 450.0, y), Mix::Pigment);
        let s2 = Stipple::new(Tool::stippler(1.6)).mixed(pal, 0.5).color(glow).coverage(|_, y| 0.15 + 1.1 * smoothstep(250.0, 460.0, y)).pressure(0.45, 0.8).dips(24, 0.35, 0.6);
        c.stipple(&sky, &s2, o.seed * 100 + 4);
        c.dry();
    }

    // ---- the far rises: cool, pale, the haze between us and them
    let far = Mask::from_fn(f, move |x, y| soft(y - hill(x), 0.6) * (1.0 - soft(y - hz(x) - 2.0, 0.8)));
    if o.stage("far", &mut c, &mut rng) {
        let far_col = move |_x: f32, y: f32| gradient(&[(0.0, hex("#8f8b97")), (1.0, hex("#b3aba9"))], ((y - 445.0) / 25.0).clamp(0.0, 1.0), Mix::Pigment);
        c.work(&far, &st.body().color(far_col).angle(|_, _| 0.0).angle_jitter(0.05).length(20.0, 60.0).coverage(4.0).threshold(0.05).clip(true), o.seed * 100 + 10);
        c.dry();
        // the woods on them as a fine stipple of darker touches along the crest
        let crest = Mask::from_fn(f, move |x, y| soft(y - hill(x), 0.6) * (1.0 - smoothstep(hill(x) + 3.0, hill(x) + 10.0, y)) * (1.0 - smoothstep(380.0, 560.0, x) * (1.0 - smoothstep(620.0, 820.0, x))));
        let s = Stipple::new(Tool::stippler(1.4)).mixed(pal, 0.4).color(|_, _| hex("#7c7885")).coverage(|_, _| 0.45).pressure(0.4, 0.8).dips(18, 0.35, 0.6).aim(false);
        c.stipple(&crest, &s, o.seed * 100 + 11);
        // the crest lost into the sky: the badger drawn level across it
        if let Some(b) = st.blend() {
            let edge = Mask::from_fn(f, move |x, y| 1.0 - smoothstep(4.0, 9.0, (y - hill(x)).abs()));
            c.work(&edge, &b.angle(|_, _| 0.0).pressure(0.3, 0.4), o.seed * 100 + 12);
        }
        c.dry();
    }

    // ---- the ruined choir: a gable wall with one tall lancet window
    let (rx, rbase) = (268.0f32, 470.0f32);
    let rn = Fbm::new(seed + 20, 4, 18.0);
    // pointed (equilateral) arch opening: jambs cx±hw from `sill` up to
    // `spring`, the two arcs of radius 2·hw above
    let lancet = move |x: f32, y: f32, cx: f32, hw: f32, spring: f32, sill: f32| -> bool {
        if y > sill || (x - cx).abs() > hw {
            return false;
        }
        if y >= spring {
            return true;
        }
        let r = 2.0 * hw;
        let dl = ((x - (cx - hw)).powi(2) + (y - spring).powi(2)).sqrt();
        let dr = ((x - (cx + hw)).powi(2) + (y - spring).powi(2)).sqrt();
        dl <= r && dr <= r
    };
    // the broken top of the wall: the left of the gable stands, the right
    // is broken down in steps
    let wall_top = move |x: f32| {
        let gable = if x < rx { 250.0 + 0.62 * (rx - x) } else { 250.0 + 0.62 * (x - rx) };
        let broken = 300.0 + 26.0 * smoothstep(300.0, 318.0, x) + 30.0 * smoothstep(330.0, 336.0, x);
        let top = if x > rx + 6.0 { gable.max(broken - 36.0 + 0.9 * (x - rx)) } else { gable };
        // where the wall is broken it breaks along the courses: stepped,
        // block by block (courses 10.4 high, blocks ~14 long)
        if x > rx + 8.0 {
            let blk = (x / 14.0).floor();
            let jig = 4.0 * paint::rng::hash2(blk as i64, 7, 11);
            ((top + jig) / 10.4).ceil() * 10.4 + 1.5 * rn.get(x, 0.0)
        } else {
            top + 2.0 * rn.get(x, 0.0)
        }
    };
    let ruin_in = move |x: f32, y: f32| -> f32 {
        // wall body 196..342, buttresses stepping out at the corners
        let body = x > 198.0 && x < 340.0 && y > wall_top(x) && y < rbase + 4.0;
        let lbut = x > 184.0 && x <= 198.0 && y > 356.0 + 0.9 * (198.0 - x) && y < rbase + 4.0;
        let lbut2 = x > 176.0 && x <= 184.0 && y > 420.0 + 0.9 * (184.0 - x) && y < rbase + 4.0;
        let rbut = x >= 340.0 && x < 352.0 && y > 404.0 + 0.9 * (x - 340.0) && y < rbase + 4.0;
        let solid = body || lbut || lbut2 || rbut;
        // the great window, and a small lancet high in the gable
        let hole = lancet(x, y, rx, 23.0, 342.0, 432.0);
        if solid && !hole { 1.0 } else { 0.0 }
    };
    let ruin = Mask::from_fn(f, move |x, y| ruin_in(x, y)).roughen(seed + 21, 7.0, 1.4, 0.5);
    if o.stage("ruin", &mut c, &mut rng) {
        // the stone in half-light: warm gray, darker at the right where the
        // wall turns, lighter up where the glow reaches
        let stone = move |x: f32, y: f32| {
            let t = ((y - 250.0) / 220.0).clamp(0.0, 1.0);
            let base = gradient(&[(0.0, hex("#766d6c")), (0.6, hex("#665e60")), (1.0, hex("#817a7a"))], t, Mix::Pigment);
            let k = 1.0 + 0.14 * rn.get(x * 0.5, y * 0.9) - 0.12 * smoothstep(300.0, 350.0, x);
            [base[0] * k, base[1] * k, base[2] * k]
        };
        // laid in with the body filbert in short level strokes, like courses
        c.work(&ruin, &st.body().color(stone).angle(|_, _| 0.0).angle_jitter(0.1).length(8.0, 26.0).coverage(4.0).threshold(0.3).clip(true), o.seed * 100 + 20);
        c.dry();
        // detail with the sable: the window's reveal in shadow (the inner
        // face of the jamb on the right of the opening, as the light is from
        // the right), the mullion and the tracery, the courses of stone
        let mut sab = Held::new(Tool::round_sable(1.4), 201);
        let shade = pal.paint(hex("#4c4549"), 0.15);
        // the reveal: the left jamb and arch, seen at a slant
        let mut pts = Vec::new();
        for k in 0..=24 {
            let t = k as f32 / 24.0;
            // up the left jamb, around the left arc to the apex
            if t < 0.5 {
                pts.push((rx - 23.0 + 1.6, 432.0 - (432.0 - 342.0) * (t / 0.5)));
            } else {
                let a = (t - 0.5) / 0.5 * (std::f32::consts::PI / 3.0);
                pts.push((rx + 23.0 - 46.0 * a.cos() + 1.6 * (1.0 - t), 342.0 - 46.0 * a.sin()));
            }
        }
        sab.load(shade, 0.8);
        c.drag(&mut sab, &Gesture::new(pts).pressure(0.9, 0.7).ramps(0.05, 0.1).shake(0.3), None);
        // the tracery has fallen: a stump of the mullion on the sill, and
        // the springing of one sub-arch still hanging from the left jamb
        let tr = pal.paint(hex("#686061"), 0.1).with_hiding(0.95).with_stiff(0.8);
        let mut mul = Held::new(Tool::round_sable(2.6), 202);
        mul.load(tr, 0.9);
        c.drag(&mut mul, &Gesture::new(vec![(rx + 0.4, 430.0), (rx + 0.2, 414.0), (rx - 0.3, 402.0)]).pressure(0.8, 0.7).ramps(0.05, 0.05).shake(0.4), None);
        let mut ring = Held::new(Tool::round_sable(2.0), 203);
        let pts: Vec<(f32, f32)> = (0..=5).map(|k| {
            let a = std::f32::consts::PI * (k as f32 / 10.0);
            (rx - 11.5 - 11.5 * a.cos(), 358.0 - 14.0 * a.sin())
        }).collect();
        ring.reload(tr, 0.8);
        c.drag(&mut ring, &Gesture::new(pts).pressure(0.75, 0.55).ramps(0.1, 0.05).shake(0.3), None);
        // the thickness of the wall seen on its broken right end, in shade
        let mut side = Held::new(Tool::round_sable(3.0), 207);
        side.load(pal.paint(hex("#433d40"), 0.15), 0.7);
        c.drag(&mut side, &Gesture::new(vec![(339.0, 318.0), (338.5, 360.0), (339.2, 404.0)]).pressure(0.6, 0.6).ramps(0.1, 0.1).shake(0.5), Some(&ruin));
        // the stones: course by course, block by block, a touch of the
        // filbert each, a little lighter or darker than the wall, lean
        let mut blk = Held::new(Tool { ragged: 0.4, ..Tool::filbert(5.0) }, 208);
        let mut row = 0;
        let mut yy = 256.0f32;
        while yy < 468.0 {
            let mut xx = 176.0 + if row % 2 == 0 { 0.0 } else { 7.0 } + rng.range(-2.0, 2.0);
            while xx < 354.0 {
                let l = rng.range(9.0, 20.0);
                if ruin.sample(xx + l * 0.5, yy + 5.0) > 0.5 && rng.chance(0.7) {
                    let dv = rng.range(-0.09, 0.09);
                    let stn = shift_l(stone(xx, yy), dv);
                    blk.reload(pal.paint(stn, 0.35), rng.range(0.2, 0.4));
                    let y0 = yy + 5.2 + rng.range(-0.8, 0.8);
                    c.drag(&mut blk, &Gesture::new(vec![(xx + 1.0, y0), (xx + l * 0.5, y0 + rng.range(-0.5, 0.5)), (xx + l - 1.5, y0 + rng.range(-0.6, 0.6))]).pressure(rng.range(0.35, 0.6), rng.range(0.25, 0.5)).ramps(0.15, 0.3).shake(0.3), Some(&ruin));
                }
                xx += l + rng.range(0.5, 1.8);
            }
            yy += 10.4;
            row += 1;
        }
        // courses: a few faint broken level lines, and blocks in the jambs
        let mut line = Held::new(Tool { point: 1.0, ..Tool::round_sable(0.9) }, 204);
        for k in 0..60 {
            let y = 262.0 + (k % 20) as f32 * 10.2 + rng.range(-1.2, 1.2);
            let x0 = rng.range(180.0, 330.0);
            let len = rng.range(6.0, 34.0);
            let tone = if rng.chance(0.7) { hex("#4c4548") } else { hex("#86807e") };
            line.reload(pal.paint(tone, 0.3), 0.4);
            c.drag(&mut line, &Gesture::new(vec![(x0, y), (x0 + len * 0.5, y + rng.range(-0.6, 0.6)), (x0 + len, y + rng.range(-0.8, 0.8))]).pressure(0.3, 0.2).ramps(0.3, 0.4).shake(0.4), Some(&ruin));
        }
        // the light catching the gable's right slope and the broken edges
        let mut lit = Held::new(Tool::round_sable(1.2), 205);
        lit.load(pal.paint(hex("#a79e98"), 0.25), 0.5);
        let pts: Vec<(f32, f32)> = (0..=12).map(|k| {
            let x = rx - 60.0 + 60.0 * k as f32 / 12.0;
            (x, wall_top(x) + 1.2)
        }).collect();
        c.drag(&mut lit, &Gesture::new(pts).pressure(0.4, 0.3).ramps(0.2, 0.3).shake(0.3), Some(&ruin));
        c.dry();
        // snow lying on the sills, the buttress offsets and the broken top
        let mut sn = Held::new(Tool::round_sable(1.8), 206);
        let snow = pal.paint(hex("#dcdad8"), 0.05).with_hiding(0.95).with_stiff(0.9);
        // find the top of the stone at x, looking down from y0 (as the
        // eye finds a ledge): the first point inside the wall
        let top_at = |x: f32, y0: f32| -> Option<f32> {
            let mut y = y0;
            while y < rbase {
                if ruin.sample(x, y) > 0.5 {
                    return Some(y);
                }
                y += 0.3;
            }
            None
        };
        let mut ledges: Vec<Vec<(f32, f32)>> = vec![vec![(rx - 22.0, 431.0), (rx - 8.0, 430.5), (rx + 8.0, 431.2), (rx + 22.0, 430.8)]];
        // the flat tops of the broken steps and the buttress offsets
        for (xa, xb) in [(282.0f32, 296.0f32), (298.0, 309.0), (311.0, 322.0), (324.0, 336.0), (341.0, 351.0), (185.0, 197.0), (177.0, 184.0)] {
            let pts: Vec<(f32, f32)> = (0..=4).filter_map(|k| {
                let x = xa + (xb - xa) * k as f32 / 4.0;
                top_at(x, 240.0).map(|y| (x, y + 0.5))
            }).collect();
            if pts.len() >= 2 {
                ledges.push(pts);
            }
        }
        for l in ledges {
            sn.reload(snow, 0.7);
            c.drag(&mut sn, &Gesture::new(l).pressure(0.6, 0.45).ramps(0.1, 0.3).shake(0.4), None);
        }
        c.dry();
    }

    // ---- the spruces: a stand right of center, the tallest in the middle
    let stand: Vec<(f32, f32, f32, f32)> = vec![
        // x, base y, height, haze (0 near .. 1 lost)
        (612.0, 486.0, 118.0, 0.35),
        (578.0, 480.0, 70.0, 0.6),
        (646.0, 490.0, 172.0, 0.15),
        (676.0, 492.0, 236.0, 0.0),
        (703.0, 489.0, 160.0, 0.1),
        (732.0, 486.0, 122.0, 0.3),
        (758.0, 482.0, 84.0, 0.5),
        (790.0, 479.0, 52.0, 0.7),
        (536.0, 476.0, 40.0, 0.8),
        (826.0, 477.0, 34.0, 0.85),
    ];
    if o.stage("spruces", &mut c, &mut rng) {
        /// A spruce by hand: the stem first, lifted to a point at the
        /// leader; then tier by tier from the top down each bough as one
        /// stroke from the stem, sagging and lifting at the tip, with the
        /// needles hatched down from it in short strokes; snow laid on the
        /// upper side of the boughs last, while the dark is still soft.
        fn spruce(c: &mut Canvas, pal: &paint::Palette, base: (f32, f32), ht: f32, dark: Rgb, snowc: Rgb, rng: &mut Rng, seed: u64) {
            let (x, y) = base;
            let top = y - ht;
            let dk = pal.paint(dark, 0.1).with_hiding(0.93).with_stiff(0.7);
            let sw = (ht * 0.014).clamp(0.6, 3.0);
            let mut stem = Held::new(Tool { point: 1.0, ..Tool::round_sable(sw * 1.6) }, seed);
            stem.load(dk, 0.9);
            let lean = rng.range(-1.5, 1.5);
            c.drag(&mut stem, &Gesture::new(vec![(x, y), (x + lean * 0.5, y - ht * 0.5), (x + lean, top)]).pressure(0.9, 0.05).ramps(0.02, 0.6).shake(0.2), None);
            let nt = ((ht / 5.5) as usize).clamp(6, 44);
            let bw = (ht * 0.011).clamp(0.6, 2.4);
            let mut bough = Held::new(Tool { point: 0.6, ..Tool::round_sable(bw * 1.4) }, seed + 1);
            let mut needle = Held::new(Tool { point: 1.0, ..Tool::round_sable(bw * 1.2) }, seed + 2);
            let mut snow = Held::new(Tool { point: 0.5, ..Tool::round_sable(bw * 1.1) }, seed + 3);
            let sn = pal.paint(snowc, 0.05).with_hiding(0.95).with_stiff(0.9);
            let mut snow_strokes = Vec::new();
            for k in 0..nt {
                let t = (k as f32 + 0.6 + rng.range(-0.3, 0.3)) / nt as f32;
                let yy = top + ht * (0.02 + 0.84 * t);
                let sx = x + lean * (1.0 - (yy - top) / ht);
                // the crown is a narrow spire, fuller below the middle
                let reach = ht * (0.05 + 0.24 * t.powf(0.85)) * rng.range(0.7, 1.15);
                if k % 3 == 0 {
                    bough.reload(dk, 0.8);
                    needle.reload(dk, 0.8);
                }
                for side in [-1.0f32, 1.0] {
                    if rng.chance(0.08 + 0.1 * t) {
                        continue; // a gap: a bough missing
                    }
                    let r = reach * rng.range(0.75, 1.1);
                    let sag = r * rng.range(0.25, 0.45);
                    let lift = r * rng.range(0.02, 0.12);
                    let tip = (sx + side * r, yy + sag - lift);
                    let pts = vec![(sx, yy - 0.5), (sx + side * r * 0.4, yy + sag * 0.55), (sx + side * r * 0.8, yy + sag * 0.95), tip];
                    c.drag(&mut bough, &Gesture::new(pts.clone()).pressure(0.8, 0.2).ramps(0.05, 0.5).shake(0.3), None);
                    // needles: short strokes hanging from the bough, down and
                    // a little outward, longer toward the middle of the bough
                    let nn = ((r / (bw * 1.5)) as usize).clamp(2, 24);
                    for j in 0..nn {
                        let u = (j as f32 + rng.range(0.0, 0.9)) / nn as f32;
                        let bx = sx + side * r * u;
                        let by = yy + sag * (1.1 * u - 0.15 * u * u) - lift * u * u;
                        let l = ht * rng.range(0.018, 0.045) * (0.5 + (1.0 - (u - 0.5).abs() * 1.4).max(0.2));
                        let a = side * rng.range(0.1, 0.5);
                        c.drag(&mut needle, &Gesture::new(vec![(bx, by - 0.4), (bx + a * l * 0.5, by + l * 0.55), (bx + a * l, by + l)]).pressure(0.75, 0.05).ramps(0.03, 0.6), None);
                    }
                    // snow on the outer half of some boughs
                    if rng.chance(0.55) {
                        let u0 = rng.range(0.25, 0.6);
                        let u1 = (u0 + rng.range(0.2, 0.45)).min(0.98);
                        let seg: Vec<(f32, f32)> = (0..=4).map(|q| {
                            let u = u0 + (u1 - u0) * q as f32 / 4.0;
                            (sx + side * r * u, yy + sag * (1.1 * u - 0.15 * u * u) - lift * u * u - bw * 0.7)
                        }).collect();
                        snow_strokes.push(seg);
                    }
                }
            }
            for (i, s) in snow_strokes.into_iter().enumerate() {
                if i % 4 == 0 {
                    snow.reload(sn, 0.7);
                }
                c.drag(&mut snow, &Gesture::new(s).pressure(rng.range(0.45, 0.8), 0.15).ramps(0.1, 0.5).shake(0.3), None);
            }
        }
        // far ones first, the near dark ones over them
        let mut order: Vec<usize> = (0..stand.len()).collect();
        order.sort_by(|a, b| stand[*b].3.partial_cmp(&stand[*a].3).unwrap());
        for (n, i) in order.into_iter().enumerate() {
            let (x, y, ht, haze) = stand[i];
            let dark = mix(hex("#1d231f"), hex("#8c8990"), haze * 0.9, Mix::Pigment);
            let snowc = mix(hex("#d6d6d8"), hex("#b8b5bb"), haze, Mix::Pigment);
            spruce(&mut c, pal, (x, y), ht, dark, snowc, &mut rng, o.seed * 1000 + 300 + n as u64 * 10);
        }
        c.dry();
    }

    // ---- ground mist: lying in the field's far edge, the feet of the ruin
    // and of the spruces lost in it
    let mn = Fbm::new(seed + 30, 4, 180.0);
    if o.stage("mist", &mut c, &mut rng) {
        let depth = move |x: f32, y: f32| {
            let d = (y - (hz(x) - 50.0)) / 60.0 + 0.45 * mn.get(x * 0.8, y * 1.6);
            smoothstep(0.0, 1.0, d) * (1.0 - smoothstep(hz(x) + 6.0, hz(x) + 26.0, y))
        };
        let band = Mask::from_fn(f, move |x, y| smoothstep(hz(x) - 90.0, hz(x) - 30.0, y) * (1.0 - smoothstep(hz(x) + 14.0, hz(x) + 30.0, y)));
        let veil = paint::Handling::new(Tool { stiffness: 0.3, lay: 0.8, pickup: 0.08, ragged: 0.35, ..Tool::filbert(7.0) })
            .mixed(pal, 0.65)
            .by_masstone()
            .color(|_, _| hex("#cbc4c0"))
            .angle(|_, _| 0.0)
            .angle_jitter(0.04)
            .length(50.0, 160.0)
            .coverage(3.0)
            .pressure(0.4, 0.6)
            .dips(2, 0.3, 0.5)
            .curve(0.04, 0.2)
            .ramps(0.3, 0.4)
            .load_at(move |x, y| 1.3 * depth(x, y));
        c.work(&band, &veil, o.seed * 100 + 31);
        let s = Stipple::new(Tool::stippler(2.0)).mixed(pal, 0.7).color(|_, y| mix(hex("#c9bfbb"), hex("#d9d2cc"), smoothstep(420.0, 470.0, y), Mix::Pigment)).coverage(move |x, y| 1.6 * depth(x, y)).pressure(0.4, 0.7).dips(16, 0.25, 0.7).aim(false);
        c.stipple(&band, &s, o.seed * 100 + 30);
        if let Some(b) = st.blend() {
            c.work(&band, &b.angle(|_, _| 0.0).pressure(0.3, 0.4), o.seed * 100 + 32);
        }
        c.dry();
    }

    // ---- the snow field
    let field = Mask::from_fn(f, move |x, y| soft(y - hz(x) - 1.0, 0.8));
    let roll = Fbm::new(seed + 40, 4, 220.0);
    // near banks: left one with the boulder, right one with the oak
    let bank_l = f.per_column(move |x: f32| 560.0 + 0.0009 * (x - 20.0).powi(2) + 4.0 * n2.get(x * 2.0, 9.0));
    let bank_r = f.per_column(move |x: f32| 598.0 + 0.0011 * (x - 930.0).powi(2) + 3.0 * n2.get(x * 2.0, 17.0));
    let snow_col = move |x: f32, y: f32| {
        // far snow picks up the sky's glow; nearer, the cool of the overcast;
        // gentle rolls; banks' faces toward us a little bluer
        let d = ((y - 468.0) / 246.0).clamp(0.0, 1.0);
        let base = gradient(&[(0.0, hex("#d6cfcc")), (0.25, hex("#dcd9d8")), (0.6, hex("#cfd0d4")), (1.0, hex("#b9bcc5"))], d, Mix::Pigment);
        let r = roll.get(x * 0.6, y * 2.2);
        let face_l = smoothstep(bank_l(x) + 2.0, bank_l(x) + 30.0, y);
        let face_r = smoothstep(bank_r(x) + 2.0, bank_r(x) + 30.0, y);
        let face = face_l.max(face_r);
        let cool = mix(base, hex("#939cb0"), (0.6 * face + 0.35 * r.max(0.0) * (0.3 + d)).clamp(0.0, 0.75), Mix::Pigment);
        let crest = (1.0 - smoothstep(0.0, 4.0, (y - bank_l(x)).abs())).max(1.0 - smoothstep(0.0, 4.0, (y - bank_r(x)).abs()));
        mix(cool, hex("#efece6"), 0.55 * crest + 0.25 * (-r).max(0.0) * (1.0 - face), Mix::Pigment)
    };
    if o.stage("snow", &mut c, &mut rng) {
        c.work(&field, &st.body().color(snow_col).angle(move |x, y| 0.04 * roll.get(x, y)).angle_jitter(0.08).length(30.0, 90.0).coverage(3.5).medium(0.12).load(0.8), o.seed * 100 + 40);
        if let Some(b) = st.blend() {
            c.work(&field, &b.angle(|_, _| 0.0).pressure(0.25, 0.35), o.seed * 100 + 41);
        }
        c.dry();
    }

    if o.stage("track", &mut c, &mut rng) {
        // the trodden way, curving from us toward the ruin: two ruts, a hand
        // pulling a cool gray down them in pieces, narrower as they recede
        let path = |t: f32| -> (f32, f32) {
            let y = 714.0 + 10.0 - (724.0 - 471.0) * t.powf(0.8);
            let x = 560.0 - 250.0 * t + 70.0 * (std::f32::consts::PI * t).sin();
            (x, y)
        };
        let mut rut = Held::new(Tool { point: 0.4, ..Tool::round_sable(3.0) }, 401);
        let gray = pal.paint(hex("#b0b4c0"), 0.45);
        for side in [-1.0f32, 1.0] {
            let mut t = 0.0f32;
            while t < 0.97 {
                let t1 = (t + rng.range(0.03, 0.1)).min(0.98);
                let pts: Vec<(f32, f32)> = (0..=6).map(|k| {
                    let u = t + (t1 - t) * k as f32 / 6.0;
                    let (x, y) = path(u);
                    let half = 12.0 * (1.0 - u).powf(1.3) + 1.0;
                    (x + side * half, y)
                }).collect();
                let wk = 1.0 - t;
                rut.reload(gray, 0.3 + 0.25 * wk);
                c.drag(&mut rut, &Gesture::new(pts).pressure(0.15 + 0.4 * wk, 0.1 + 0.35 * wk).ramps(0.2, 0.3).shake(0.4), None);
                t = t1 + rng.range(0.01, 0.06);
            }
        }
        c.dry();
    }

    // ---- the boulder on the left bank
    let bx = 150.0f32;
    let bn = Fbm::new(seed + 50, 4, 30.0);
    let lump = move |x: f32, y: f32, cx: f32, cy: f32, rx_: f32, ry: f32| -> f32 {
        let a = (y - cy).atan2(x - cx);
        let r = 1.0 + 0.12 * bn.get(a.cos() * 40.0, a.sin() * 40.0) + 0.07 * (3.0 * a).sin() + 0.09 * (2.0 * a + 1.3).sin();
        let d = (((x - cx) / rx_).powi(2) + ((y - cy) / ry).powi(2)).sqrt() / r;
        // flat on the ground: cut at the snow line
        1.0 - smoothstep(0.97, 1.03, d)
    };
    let boulder_in = move |x: f32, y: f32| -> f32 {
        // a big erratic and two smaller stones leaning on it, sunk in snow
        let m = lump(x, y, bx - 8.0, 606.0, 70.0, 50.0).max(lump(x, y, bx + 52.0, 618.0, 38.0, 30.0)).max(lump(x, y, bx - 72.0, 628.0, 22.0, 15.0));
        m * (1.0 - smoothstep(636.0, 640.0, y))
    };
    let boulder = Mask::from_fn(f, boulder_in);
    if o.stage("boulder", &mut c, &mut rng) {
        let rockc = move |x: f32, y: f32| {
            // lit from behind and above: the top edge catches the sky, the
            // face toward us dark
            let t = ((y - 548.0) / 90.0).clamp(0.0, 1.0);
            let base = gradient(&[(0.0, hex("#5e5a5a")), (0.35, hex("#46423f")), (1.0, hex("#2e2b29"))], t, Mix::Pigment);
            let k = 1.0 + 0.2 * bn.get(x * 1.5, y * 1.5);
            [base[0] * k, base[1] * k, base[2] * k]
        };
        c.work(&boulder, &st.body().color(rockc).angle(move |x, _| 0.5 * ((x - bx) / 80.0)).angle_jitter(0.5).length(6.0, 20.0).coverage(4.0).threshold(0.2).clip(true), o.seed * 100 + 50);
        c.dry();
        // cracks and facets: a few dark strokes, a few lighter planes
        let mut sab = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.6) }, 501);
        for _ in 0..9 {
            let x0 = bx + rng.range(-60.0, 60.0);
            let y0 = 560.0 + rng.range(0.0, 60.0);
            if boulder.sample(x0, y0) < 0.5 {
                continue;
            }
            let a: f32 = rng.range(0.6, 2.4);
            let l = rng.range(8.0, 25.0);
            sab.reload(pal.paint(hex("#221f1d"), 0.2), 0.5);
            c.drag(&mut sab, &Gesture::new(vec![(x0, y0), (x0 + a.cos() * l * 0.5 + rng.range(-2.0, 2.0), y0 + a.sin() * l * 0.5), (x0 + a.cos() * l, y0 + a.sin() * l)]).pressure(0.6, 0.1).ramps(0.1, 0.5).shake(0.4), Some(&boulder));
        }
        let mut lit = Held::new(Tool::round_sable(3.0), 502);
        for _ in 0..7 {
            let x0 = bx + rng.range(-55.0, 45.0);
            let y0 = 566.0 + rng.range(0.0, 18.0);
            lit.reload(pal.paint(hex("#77716c"), 0.3), 0.35);
            let l = rng.range(8.0, 20.0);
            c.drag(&mut lit, &Gesture::new(vec![(x0, y0), (x0 + l, y0 + rng.range(-3.0, 3.0))]).pressure(0.35, 0.2).ramps(0.2, 0.4), Some(&boulder));
        }
        c.dry();
        // the snow cap: thick lead white laid on the top in a few strokes,
        // its lower edge wandering, a little overhanging
        // the top of the stones at each x, then the snow's depth on them:
        // thick on the crown, thinning off at the shoulders
        let top_y = f.per_column(move |x: f32| {
            let mut y = 540.0;
            while y < 640.0 && boulder_in(x, y) < 0.5 {
                y += 0.25;
            }
            y
        });
        let cap_bot = f.per_column(move |x: f32| {
            let t = top_y(x);
            if t >= 639.0 { return t; }
            t + (3.0 + 13.0 * (1.0 - ((t - 556.0) / 50.0).clamp(0.0, 1.0))) * (1.0 + 0.45 * bn.get(x * 4.0, 2.0))
        });
        let cap = Mask::from_fn(f, move |x, y| boulder_in(x, y + 2.5).max(boulder_in(x, y)) * (1.0 - smoothstep(cap_bot(x) - 1.0, cap_bot(x) + 1.0, y)));
        let _ = &cap;
        // laid by hand, in stiff lead white, stroke over stroke along the
        // stone's crown, from the top down to the lip, each a little shorter
        let mut capb = Held::new(Tool { lay: 1.3, ..Tool::filbert(6.0) }, 504);
        for layer in 0..5 {
            let mut x = bx - 95.0 + rng.range(0.0, 10.0);
            while x < bx + 92.0 {
                let x1 = x + rng.range(14.0, 34.0);
                let pts: Vec<(f32, f32)> = (0..=5).filter_map(|k| {
                    let xx = x + (x1 - x) * k as f32 / 5.0;
                    let (t, b) = (top_y(xx), cap_bot(xx));
                    if t >= 639.0 || (top_y(xx + 2.0) - top_y(xx - 2.0)).abs() > 3.2 { return None; }
                    let u = (layer as f32 + 0.5) / 5.0;
                    Some((xx, t - 1.2 + (b - t) * u * 0.8))
                }).collect();
                if pts.len() >= 2 {
                    let y = pts[0].1;
                    let col = mix(hex("#eeebe6"), hex("#c7cad3"), smoothstep(550.0, 590.0, y) * 0.7 + 0.2 * layer as f32 / 5.0, Mix::Pigment);
                    capb.reload(pal.paint(col, 0.04).with_hiding(0.97).with_stiff(0.95), rng.range(0.7, 1.0));
                    c.drag(&mut capb, &Gesture::new(pts).pressure(rng.range(0.55, 0.85), rng.range(0.35, 0.6)).ramps(0.1, 0.3).shake(0.3), None);
                }
                x = x1 + rng.range(-6.0, 2.0);
            }
        }
        // the shadow under the cap's overhanging lip, and rock showing
        // through where the snow has slid
        let mut lip = Held::new(Tool { point: 0.6, ..Tool::round_sable(2.2) }, 503);
        let mut x = bx - 92.0;
        while x < bx + 88.0 {
            let x1 = x + rng.range(30.0, 60.0);
            let pts: Vec<(f32, f32)> = (0..=4).map(|k| {
                let xx = x + (x1 - x) * k as f32 / 4.0;
                (xx, cap_bot(xx) + 1.6)
            }).collect();
            lip.reload(pal.paint(hex("#2a2725"), 0.3), 0.35);
            c.drag(&mut lip, &Gesture::new(pts).pressure(rng.range(0.3, 0.5), 0.25).ramps(0.2, 0.3).shake(0.2), Some(&boulder));
            x = x1 + rng.range(-4.0, 4.0);
        }
        for _ in 0..0 {
            let x0 = bx + rng.range(-40.0, 40.0);
            let y0 = cap_bot(x0) - rng.range(2.0, 8.0);
            lip.reload(pal.paint(hex("#4a4542"), 0.2), 0.35);
            c.drag(&mut lip, &Gesture::new(vec![(x0, y0), (x0 + rng.range(2.0, 6.0), y0 + rng.range(-1.0, 1.0))]).pressure(0.4, 0.2).ramps(0.2, 0.4), Some(&boulder));
        }
        // snow drifted against its foot
        let foot = Mask::from_fn(f, move |x, y| (1.0 - smoothstep(0.0, 6.0, (y - 638.0 + 4.0 * bn.get(x * 2.0, 5.0)).abs())) * (1.0 - smoothstep(70.0, 95.0, (x - bx).abs())));
        c.work(&foot, &st.body().color(|_, _| hex("#dcdde0")).angle(|_, _| 0.0).length(10.0, 30.0).coverage(3.0).medium(0.1).threshold(0.2).clip(true), o.seed * 100 + 52);
        c.dry();
    }

    // ---- the oak on the right bank, grown by hand: angular elbows, limbs
    // flung out sideways, a broken top, twigs in short hooks
    if o.stage("oak", &mut c, &mut rng) {
        struct Seg {
            pts: Vec<(f32, f32)>,
            w0: f32,
            w1: f32,
            broken: bool,
        }
        fn grow(out: &mut Vec<Seg>, at: (f32, f32), dir: f32, len: f32, w: f32, depth: u32, rng: &mut Rng) {
            let nseg = if depth == 0 { 5 } else { rng.range(2.0, 5.0) as usize };
            let mut p = at;
            let mut a = dir;
            let mut pts = vec![p];
            let mut kids = Vec::new();
            let wend = (w * rng.range(0.45, 0.65)).max(0.35);
            let broken = depth > 0 && depth < 4 && rng.chance(0.12);
            for s in 0..nseg {
                // an elbow at each knot: oaks change direction abruptly
                a += rng.normal() * (0.25 + 0.08 * depth as f32);
                // side limbs of a solitary oak spread level; twigs turn up
                let up = -std::f32::consts::FRAC_PI_2;
                let pull = if depth == 0 { 0.35 } else if depth >= 3 { 0.12 } else { -0.04 };
                a += (up - a) * pull;
                // limbs never hang below level (in y-down units, upward is
                // -π..0); twigs may droop a little
                let (lo, hi) = if depth <= 2 { (-std::f32::consts::PI + 0.15, -0.12) } else { (-std::f32::consts::PI - 0.35, 0.35) };
                a = a.clamp(lo, hi);
                let l = len / nseg as f32 * rng.range(0.7, 1.3);
                p = (p.0 + a.cos() * l, p.1 + a.sin() * l);
                pts.push(p);
                let ws = w + (wend - w) * (s as f32 + 1.0) / nseg as f32;
                if depth < 5 && !(broken && s + 1 == nseg) && s + 1 < nseg || (depth < 5 && s + 1 == nseg && !broken) {
                    let nk = if depth == 0 && s < 2 { 0 } else if depth >= 4 { rng.range(0.6, 2.4) as usize } else if depth == 3 { rng.range(1.0, 2.8) as usize } else { rng.range(1.0, 2.4) as usize };
                    for _ in 0..nk {
                        let side = if rng.chance(0.5) { -1.0 } else { 1.0 };
                        let da = side * if depth >= 3 { rng.range(0.5, 1.3) } else { rng.range(0.4, 0.95) };
                        kids.push((p, a + da, len * rng.range(0.45, 0.72), ws * rng.range(0.5, 0.8), depth + 1));
                    }
                }
            }
            out.push(Seg { pts, w0: w, w1: wend, broken });
            for (p, a, l, ww, d) in kids {
                if ww < 0.25 {
                    continue;
                }
                grow(out, p, a, l, ww, d, rng);
            }
        }
        let base = (905.0, 650.0);
        // its shadow first, thrown toward us by the light low behind it:
        // a cool breath on the snow, softened while wet
        let mut shb = Held::new(Tool { ragged: 0.4, ..Tool::filbert(14.0) }, 688);
        shb.load(pal.paint(hex("#a3aabb"), 0.55), 0.35);
        c.drag(&mut shb, &Gesture::new(vec![(base.0 - 2.0, base.1 + 2.0), (base.0 - 28.0, base.1 + 26.0), (base.0 - 70.0, base.1 + 64.0)]).pressure(0.45, 0.2).ramps(0.1, 0.6).shake(0.5), None);
        let mut segs = Vec::new();
        // the oak's own hand: its shape shouldn't change when other passages do
        let oseed: u64 = std::env::var("OAK").ok().and_then(|v| v.parse().ok()).unwrap_or(3);
        let mut rng = Rng::new(oseed * 7919 + 13);
        // the trunk, leaning left toward the field, and its first great limbs
        // the trunk by hand: short, thick, a slight twist, dividing at
        // about a third of the tree's height into heavy limbs
        let top = (base.0 - 6.0, base.1 - 150.0);
        segs.push(Seg { pts: vec![base, (base.0 + 3.0, base.1 - 45.0), (base.0 - 2.0, base.1 - 95.0), top, (top.0 - 1.0, top.1 - 14.0)], w0: 30.0, w1: 17.0, broken: true });
        use std::f32::consts::{FRAC_PI_2, PI};
        for (at, a, l, w) in [
            (top, -FRAC_PI_2 - 0.55, 170.0, 13.0),
            (top, -FRAC_PI_2 + 0.05, 200.0, 14.0),
            (top, -FRAC_PI_2 + 0.6, 140.0, 11.0),
            ((base.0 - 2.0, base.1 - 96.0), -PI + 0.5, 150.0, 9.0),
        ] {
            grow(&mut segs, at, a, l, w, 1, &mut rng);
        }
        // a limb torn off long ago: a blunt stub on the right of the trunk
        segs.push(Seg { pts: vec![(base.0 + 4.0, base.1 - 70.0), (base.0 + 16.0, base.1 - 82.0), (base.0 + 22.0, base.1 - 84.0)], w0: 8.0, w1: 6.0, broken: true });
        segs.sort_by(|a, b| b.w0.partial_cmp(&a.w0).unwrap());
        let bark = pal.paint(hex("#2b2622"), 0.12).with_hiding(0.95).with_stiff(0.7);
        let mut n = 0u64;
        let mut snowy = Vec::new();
        for s in &segs {
            n += 1;
            // the brush for this limb's width: several parallel pulls for the
            // trunk, one stroke for a limb, the pointed rigger for twigs
            let tool = if s.w0 > 4.0 {
                Tool::round_sable((s.w0 * 0.55).min(8.0))
            } else if s.w0 > 1.2 {
                Tool { point: 1.0, ..Tool::round_sable(s.w0 * 1.1) }
            } else {
                Tool { point: 1.0, ..Tool::rigger(s.w0 * 1.3) }
            };
            let mut held = Held::new(tool.clone(), 600 + n);
            let passes = if s.w0 > 4.0 { ((s.w0 / 3.5).ceil() as usize).max(2) } else { 1 };
            for q in 0..passes {
                let off = if passes > 1 { (q as f32 / (passes - 1) as f32 - 0.5) * s.w0 * 0.55 } else { 0.0 };
                let pts: Vec<(f32, f32)> = s.pts.iter().enumerate().map(|(i, p)| {
                    // offset across the limb, shrinking with its taper
                    let j = (i + 1).min(s.pts.len() - 1);
                    let k = i.saturating_sub(1);
                    let (dx, dy) = (s.pts[j].0 - s.pts[k].0, s.pts[j].1 - s.pts[k].1);
                    let l = (dx * dx + dy * dy).sqrt().max(1e-3);
                    let t = i as f32 / (s.pts.len() - 1) as f32;
                    let o2 = off * (1.0 - t * (1.0 - s.w1 / s.w0));
                    (p.0 - dy / l * o2, p.1 + dx / l * o2)
                }).collect();
                held.reload(bark, 0.9);
                let p0 = tool.pressure_for(if passes > 1 { tool.width } else { s.w0 });
                let p1 = if s.broken { p0 * if s.w0 > 20.0 { 0.8 } else { 0.5 } } else { tool.pressure_for(s.w1).min(p0) * 0.6 };
                c.drag(&mut held, &Gesture::new(pts).pressure(p0.max(0.15), p1.max(0.03)).ramps(0.02, if s.broken { 0.05 } else { 0.5 }).shake(0.25), None);
            }
            // snow will lie on the limbs that run level enough
            if s.w0 > 1.0 {
                for win in s.pts.windows(2) {
                    let (dx, dy) = (win[1].0 - win[0].0, win[1].1 - win[0].1);
                    let near_fork = ((win[0].0 - top.0).powi(2) + (win[0].1 - top.1).powi(2)).sqrt() < 16.0;
                    if dy.abs() < dx.abs() * 0.7 && !near_fork && rng.chance(0.6) {
                        snowy.push((win[0], win[1], s.w0));
                    }
                }
            }
        }
        c.wait(60.0);
        // snow banked against the foot of the trunk
        let mut drift = Held::new(Tool::filbert(6.0), 689);
        drift.load(pal.paint(hex("#d9dadf"), 0.08).with_hiding(0.95), 0.9);
        c.drag(&mut drift, &Gesture::new(vec![(base.0 - 26.0, base.1 + 2.0), (base.0 - 10.0, base.1 - 1.5), (base.0 + 6.0, base.1 - 0.5), (base.0 + 24.0, base.1 + 3.0)]).pressure(0.7, 0.5).ramps(0.2, 0.4).shake(0.4), None);
        // a lean lighter streak down the trunk's right, where the glow is
        let mut lt = Held::new(Tool { ragged: 0.5, ..Tool::round_sable(1.6) }, 690);
        for k in 0..3 {
            let dx = 7.0 + 2.5 * k as f32;
            lt.reload(pal.paint(hex("#4c4440"), 0.4), 0.25);
            let y0 = base.1 - rng.range(8.0, 30.0);
            let y1 = base.1 - rng.range(100.0, 145.0);
            c.drag(&mut lt, &Gesture::new(vec![(base.0 + dx, y0), (base.0 + dx - 1.0, (y0 + y1) * 0.5), (base.0 + dx - 5.0, y1)]).pressure(0.3, 0.15).ramps(0.3, 0.5).shake(0.6), None);
        }
        c.dry();
        // snow along the upper side of the level limbs
        let mut sn = Held::new(Tool { point: 0.5, ..Tool::round_sable(1.6) }, 691);
        let snow = pal.paint(hex("#e6e4e0"), 0.05).with_hiding(0.96).with_stiff(0.9);
        for (i, (a, b, w)) in snowy.into_iter().enumerate() {
            if i % 5 == 0 {
                sn.reload(snow, 0.7);
            }
            let up = w * 0.5;
            let u0 = rng.range(0.0, 0.3);
            let u1 = rng.range(0.6, 1.0);
            let p0 = (a.0 + (b.0 - a.0) * u0, a.1 + (b.1 - a.1) * u0 - up);
            let p1 = (a.0 + (b.0 - a.0) * u1, a.1 + (b.1 - a.1) * u1 - up);
            c.drag(&mut sn, &Gesture::new(vec![p0, ((p0.0 + p1.0) * 0.5, (p0.1 + p1.1) * 0.5 - 0.3), p1]).pressure((w * 0.25).clamp(0.3, 0.8), 0.15).ramps(0.1, 0.5).shake(0.3), None);
        }
        c.dry();
    }

    if o.stage("figure", &mut c, &mut rng) {
        // the traveler, from behind, walking up the track toward the ruin:
        // a long dark greatcoat, a hat, a stick in the right hand
        let (fx, fy) = (478.0f32, 552.0f32);
        let hh = 34.0f32;
        let coat = pal.paint(hex("#23262a"), 0.05).with_hiding(0.97).with_stiff(0.9);
        let dark = pal.paint(hex("#141312"), 0.05).with_hiding(0.97).with_stiff(0.9);
        let mut b = Held::new(Tool::round_sable(2.6), 701);
        // legs, one mid-stride, the boots dark
        b.load(dark, 0.9);
        c.drag(&mut b, &Gesture::new(vec![(fx - 2.0, fy - hh * 0.4), (fx - 2.4, fy - hh * 0.18), (fx - 2.6, fy - 0.3)]).pressure(0.7, 0.6).ramps(0.05, 0.05), None);
        c.drag(&mut b, &Gesture::new(vec![(fx + 2.2, fy - hh * 0.4), (fx + 2.8, fy - hh * 0.2), (fx + 3.8, fy - 2.2)]).pressure(0.65, 0.55).ramps(0.05, 0.05), None);
        // the coat: from the shoulders down, flaring, in vertical pulls
        let mut cb = Held::new(Tool::round_sable(3.2), 702);
        cb.load(coat, 1.0);
        for k in 0..7 {
            let u = k as f32 / 6.0 - 0.5;
            let top = (fx + u * hh * 0.2, fy - hh * 0.8 + (u * 2.0).powi(2) * 1.6);
            let bot = (fx + u * hh * 0.34 + 0.6, fy - hh * 0.3 + rng.range(-0.8, 0.8));
            c.drag(&mut cb, &Gesture::new(vec![top, ((top.0 + bot.0) * 0.5, (top.1 + bot.1) * 0.5), bot]).pressure(0.75, 0.8).ramps(0.05, 0.1), None);
        }
        // arms: the left hangs, the right reaches forward to the stick
        c.drag(&mut cb, &Gesture::new(vec![(fx - hh * 0.1, fy - hh * 0.76), (fx - hh * 0.13, fy - hh * 0.6), (fx - hh * 0.12, fy - hh * 0.47)]).pressure(0.6, 0.5).ramps(0.05, 0.2), None);
        c.drag(&mut cb, &Gesture::new(vec![(fx + hh * 0.1, fy - hh * 0.76), (fx + hh * 0.15, fy - hh * 0.62), (fx + hh * 0.2, fy - hh * 0.52)]).pressure(0.6, 0.5).ramps(0.05, 0.2), None);
        // the collar turned up, the head, the hat
        let mut hd = Held::new(Tool::round_sable(2.4), 703);
        hd.load(pal.paint(hex("#3a3431"), 0.05).with_hiding(0.97), 0.8);
        c.drag(&mut hd, &Gesture::new(vec![(fx - 0.2, fy - hh * 0.83), (fx, fy - hh * 0.88)]).pressure(0.8, 0.8).ramps(0.1, 0.1), None);
        hd.reload(dark, 0.9);
        c.drag(&mut hd, &Gesture::new(vec![(fx - hh * 0.08, fy - hh * 0.905), (fx + hh * 0.085, fy - hh * 0.9)]).pressure(0.45, 0.45).ramps(0.1, 0.2), None);
        c.drag(&mut hd, &Gesture::new(vec![(fx - hh * 0.035, fy - hh * 0.92), (fx + 0.2, fy - hh * 0.98), (fx + hh * 0.04, fy - hh * 0.92)]).pressure(0.7, 0.7).ramps(0.1, 0.1), None);
        // the stick
        let mut st_ = Held::new(Tool { point: 1.0, ..Tool::round_sable(0.9) }, 704);
        st_.load(pal.paint(hex("#2e2620"), 0.1), 0.7);
        c.drag(&mut st_, &Gesture::new(vec![(fx + hh * 0.21, fy - hh * 0.55), (fx + hh * 0.26, fy - hh * 0.25), (fx + hh * 0.3, fy + 0.5)]).pressure(0.8, 0.5).ramps(0.05, 0.1), None);
        c.dry();
        // his shadow falls toward us, faint and cool; footprints behind him
        let mut sh = Held::new(Tool::round_sable(3.0), 705);
        sh.load(pal.paint(hex("#aab0bf"), 0.4), 0.4);
        c.drag(&mut sh, &Gesture::new(vec![(fx, fy + 0.5), (fx + 3.0, fy + 7.0), (fx + 6.0, fy + 14.0)]).pressure(0.5, 0.25).ramps(0.1, 0.6), None);
        let mut fp = Held::new(Tool::round_sable(1.6), 706);
        let prints = pal.paint(hex("#a6abb8"), 0.3);
        let mut t = 0.0f32;
        let mut side = 1.0f32;
        while t < 1.0 {
            // the track from the figure back toward us
            let y = fy + 4.0 + t * 160.0;
            let x = fx + 16.0 * t + 50.0 * t * t;
            let s = 1.0 + 2.2 * t;
            if (t * 10.0) as i32 % 3 == 0 {
                fp.reload(prints, 0.5);
            }
            c.drag(&mut fp, &Gesture::new(vec![(x + side * 1.2 * s, y), (x + side * 1.2 * s + 0.3, y + 1.4 * s)]).pressure(0.3 + 0.2 * t, 0.3 + 0.2 * t).ramps(0.2, 0.3), None);
            t += 0.018 + 0.03 * t + rng.range(0.0, 0.01);
            side = -side;
        }
        c.dry();
    }

    if o.stage("grass", &mut c, &mut rng) {
        // dry grass through the snow, "fine upturning strokes" laid last
        // [NG p.56]: clumps along the banks, a few in the field
        let cols = [hex("#5e5242"), hex("#766a55"), hex("#34302b"), hex("#4b4439"), hex("#857a64"), hex("#3e3a36")];
        let mut rig = Held::new(Tool { point: 1.0, ..Tool::rigger(0.9) }, 801);
        let mut clumps: Vec<(f32, f32, f32)> = Vec::new();
        for _ in 0..28 {
            let x = rng.range(0.0, 420.0);
            let y = bank_l(x) + rng.range(-2.0, 60.0);
            if y > 700.0 || boulder.sample(x, y) > 0.1 {
                continue;
            }
            clumps.push((x, y, 1.0 + (y - 560.0) / 120.0));
        }
        for _ in 0..22 {
            let x = rng.range(700.0, 1000.0);
            let y = bank_r(x) + rng.range(-2.0, 50.0);
            if y > 710.0 {
                continue;
            }
            clumps.push((x, y, 1.0 + (y - 598.0) / 100.0));
        }
        for _ in 0..14 {
            let x = rng.range(300.0, 800.0);
            let y = rng.range(490.0, 620.0);
            clumps.push((x, y, 0.25 + (y - 480.0) / 200.0));
        }
        clumps.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        for (gx, gy, sc) in clumps {
            let nb = (rng.range(3.0, 12.0) * sc.min(1.5)) as usize + 2;
            let col = cols[(rng.range(0.0, cols.len() as f32) as usize).min(cols.len() - 1)];
            rig.reload(pal.paint(col, 0.2), 0.6);
            for _ in 0..nb {
                let x = gx + rng.range(-5.0, 5.0) * sc;
                let y = gy + rng.range(-1.0, 1.5) * sc;
                let hgt = rng.range(5.0, 18.0) * sc;
                let lean = rng.range(-0.5, 0.5) * hgt + 0.15 * hgt;
                let bend = rng.range(-0.2, 0.3) * hgt;
                c.drag(&mut rig, &Gesture::new(vec![(x, y), (x + lean * 0.3, y - hgt * 0.5), (x + lean * 0.7 + bend * 0.3, y - hgt * 0.85), (x + lean + bend, y - hgt)]).pressure(rng.range(0.4, 0.8), 0.03).ramps(0.02, 0.7), None);
            }
        }
    }

    if o.stage("crows", &mut c, &mut rng) {
        let black = pal.paint(hex("#161514"), 0.05).with_hiding(0.97).with_stiff(0.9);
        let mut b = Held::new(Tool { point: 1.0, ..Tool::round_sable(1.5) }, 901);
        // two in the air over the field, wings in different beats
        for &(x, y, s, beat) in &[(520.0f32, 210.0f32, 1.8f32, 0.4f32), (566.0, 246.0, 1.5, -0.3), (452.0, 168.0, 1.3, 0.1)] {
            b.reload(black, 0.7);
            c.drag(&mut b, &Gesture::new(vec![(x - 2.0 * s, y), (x + 2.2 * s, y + 0.3 * s)]).pressure(0.8, 0.5).ramps(0.1, 0.3), None);
            for side in [-1.0f32, 1.0] {
                let tip = (x + side * 7.0 * s, y - 3.5 * s * beat - 1.0 * s);
                c.drag(&mut b, &Gesture::new(vec![(x, y), (x + side * 3.0 * s, y - 2.5 * s * beat - 0.6 * s), tip]).pressure(0.55, 0.05).ramps(0.05, 0.6), None);
            }
        }
        // one on the snow by the track
        for &(x, y) in &[(418.0f32, 596.0f32)] {
            b.reload(black, 0.7);
            c.drag(&mut b, &Gesture::new(vec![(x - 3.0, y - 2.0), (x, y - 2.8), (x + 3.0, y - 2.2)]).pressure(1.0, 0.9).ramps(0.1, 0.2), None);
            c.drag(&mut b, &Gesture::new(vec![(x + 2.6, y - 2.6), (x + 4.2, y - 3.6)]).pressure(0.7, 0.4).ramps(0.1, 0.3), None);
            c.drag(&mut b, &Gesture::new(vec![(x - 2.5, y - 2.2), (x - 5.0, y - 1.2)]).pressure(0.6, 0.1).ramps(0.1, 0.5), None);
            c.drag(&mut b, &Gesture::new(vec![(x - 0.5, y - 1.0), (x - 0.4, y)]).pressure(0.2, 0.2).ramps(0.1, 0.1), None);
        }
        c.dry();
    }

    o.finish(&mut c, &mut rng, &Finish::aged(st.relief));
}
