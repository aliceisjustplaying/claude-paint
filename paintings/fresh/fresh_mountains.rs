//! Morning on the Ridge (in the manner of Caspar David Friedrich).
//!
//! An original composition: a wide dawn view over the Riesengebirge from a
//! granite tor. Long ridges recede in veils of morning mist, each paler and
//! bluer than the one before; the sun has not yet cleared the farthest ridge
//! and only its glow stands in the saddle. On the right the tor rises dark
//! against the light, and on its top stand a plain wooden cross and spruces
//! (the evergreen and the cross: faith that outlasts). On the ledge at the
//! left a single wanderer, seen from behind, faces the dawn. There is no
//! middle ground: the near rock drops straight into the misted distance.
//!
//! Order of work as Friedrich's: bought ground, underdrawing, a very thin
//! underpainting, sky, distance to foreground, figures last, finish.

use paint::color::{Mix, gradient, mix};
use paint::{Apply, Canvas, Fbm, Gesture, Ground, Handling, Held, Mask, Orient, Palette, Rgb, Rng, Shape, Style, Tool, hex, smoothstep};
use paintings::run::{Finish, Run};

const ASPECT: f32 = 1.6;
/// Where the sun is about to rise (just below the far saddle).
const SUN: (f32, f32) = (420.0, 350.0);
/// Eye level: block tops below it are seen from above, above it from below.
const EYE: f32 = 352.0;

// ---------------------------------------------------------------- geometry

struct Ridge {
    base: f32,
    amp: f32,
    n: Fbm,
    fine: Fbm,
    /// (x, height, half width) of rounded summits / cones.
    peaks: Vec<(f32, f32, f32)>,
    col: Rgb,
    mist: Rgb,
    /// Distance below the crest over which the ridge dissolves into mist.
    fade: f32,
    /// How far below the crest the ridge is painted at all.
    depth: f32,
    /// Forest on the crest: spire height (0 = bare).
    forest: f32,
    /// Linear rise of the whole ridge (units per unit x; negative rises right).
    tilt: f32,
    mistn: Fbm,
}

impl Ridge {
    fn crest(&self, x: f32) -> f32 {
        let mut y = self.base + self.tilt * (x - 500.0) - self.amp * self.n.get(x, 13.7) - self.amp * 0.18 * self.fine.get(x, 3.1);
        for &(px, ph, pw) in &self.peaks {
            let d = ((x - px).powi(2) + (0.18 * pw).powi(2)).sqrt() / pw;
            y -= ph * (1.0 - d).max(0.0).powf(1.25);
        }
        y
    }
    fn slope(&self, x: f32) -> f32 {
        (self.crest(x + 3.0) - self.crest(x - 3.0)) / 6.0
    }
    fn color(&self, x: f32, y: f32) -> Rgb {
        let d = y - self.crest(x);
        // the mist lies unevenly: here it climbs high, there the slope shows
        let fade = self.fade * (1.0 + 0.45 * self.mistn.get(x, 5.0));
        let t = smoothstep(fade * 0.15, fade, d);
        // the glow of the sun behind the far ridges warms the haze near it
        let g = (-((x - SUN.0) / 240.0).powi(2)).exp();
        let mist = mix(self.mist, hex("#eedcb6"), g * 0.5, Mix::Light);
        mix(self.col, mist, t.powf(0.9), Mix::Light)
    }
}

fn curve(f: impl Fn(f32) -> f32) -> Vec<(f32, f32)> {
    (0..=216)
        .map(|i| {
            let x = -40.0 + i as f32 * 5.0;
            (x, f(x))
        })
        .collect()
}

/// A granite block of a tor: a boxy, rounded, weathered slab.
struct Block {
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    p: f32,
    tilt: f32,
    n: Fbm,
    pts: Vec<(f32, f32)>,
}

impl Block {
    fn new(cx: f32, cy: f32, rx: f32, ry: f32, tilt: f32, seed: u32) -> Self {
        Self::shaped(cx, cy, rx, ry, tilt, seed, 4.5 + (seed % 3) as f32 * 0.8)
    }
    /// `p`: superellipse exponent (2 = oval boulder, 5 = boxy block).
    fn shaped(cx: f32, cy: f32, rx: f32, ry: f32, tilt: f32, seed: u32, p: f32) -> Self {
        let n = Fbm::new(seed, 4, 1.1);
        let mut b = Block { cx, cy, rx, ry, p, tilt, n, pts: vec![] };
        let (st, ct) = tilt.sin_cos();
        b.pts = (0..64)
            .map(|i| {
                let a = i as f32 / 64.0 * std::f32::consts::TAU;
                let r = b.radius(a);
                let (lx, ly) = (rx * r * a.cos(), ry * r * a.sin());
                (cx + lx * ct - ly * st, cy + lx * st + ly * ct)
            })
            .collect();
        b
    }
    /// Boundary radius (in normalized units) in direction `a`.
    fn radius(&self, a: f32) -> f32 {
        let (ca, sa) = (a.cos(), a.sin());
        let p = self.p;
        let r = 1.0 / (ca.abs().powf(p) + sa.abs().powf(p)).powf(1.0 / p);
        r * (1.0 + 0.06 * self.n.get(ca * 1.6, sa * 1.6) + 0.02 * self.n.get(ca * 6.0 + 9.0, sa * 6.0))
    }
    fn inside(&self, x: f32, y: f32) -> bool {
        let (u, v) = self.uv(x, y);
        let a = v.atan2(u);
        (u * u + v * v).sqrt() < self.radius(a)
    }
    fn uv(&self, x: f32, y: f32) -> (f32, f32) {
        let (st, ct) = self.tilt.sin_cos();
        let (dx, dy) = (x - self.cx, y - self.cy);
        ((dx * ct + dy * st) / self.rx, (-dx * st + dy * ct) / self.ry)
    }
}

// ------------------------------------------------------------------ colors

fn sky(x: f32, y: f32) -> Rgb {
    let t = (y / 350.0).clamp(0.0, 1.0);
    let base = gradient(
        &[
            (0.0, hex("#5f7aa6")),
            (0.28, hex("#8a9ec0")),
            (0.5, hex("#b5b3c6")),
            (0.68, hex("#dcbfbb")),
            (0.84, hex("#eed2b0")),
            (1.0, hex("#f3dfb4")),
        ],
        t,
        Mix::Light,
    );
    let dx = (x - SUN.0) / 360.0;
    let dy = (y - SUN.1) / 160.0;
    let g = (-(dx * dx + dy * dy)).exp();
    mix(base, hex("#f6e3b2"), g * 0.75, Mix::Light)
}

fn main() {
    let o = Run::new("fresh_mountains");
    let mut st = Style::friedrich();
    // a Dresden ground whose top layer is a patchy, lead-white-rich buff
    // (KÖR p.284) rather than the reddish ocher of the moon pictures: this is
    // a dawn picture and wants the light of the ground under the sky
    st.ground[2] = Ground { color: hex("#dccdb2"), hiding: 0.8, um: 55.0, stiff: 0.4, apply: Apply::Brush };
    let pal: Palette = st.palette.clone();
    let mut c = st.prepare(o.width, ASPECT, o.seed);
    let f = c.frame();
    let hgt = c.height();
    if o.stage(&mut c, "ground") {
        return;
    }

    // ------------------------------------------------------------ the plan
    #[allow(clippy::too_many_arguments)]
    let ridge = |base: f32, amp: f32, seed: u32, period: f32, peaks: Vec<(f32, f32, f32)>, col: &str, mist: &str, fade: f32, depth: f32, forest: f32, tilt: f32| Ridge {
        tilt,
        mistn: Fbm::new(seed + 90, 3, 150.0),
        base,
        amp,
        n: Fbm::new(seed, 3, period),
        fine: Fbm::new(seed + 50, 3, period * 0.15),
        peaks,
        col: hex(col),
        mist: hex(mist),
        fade,
        depth,
        forest,
    };
    let ridges = vec![
        // the farthest chain with its cone
        ridge(340.0, 9.0, 3, 380.0, vec![(205.0, 38.0, 125.0), (640.0, 9.0, 160.0)], "#aea3b3", "#dccfc5", 34.0, 70.0, 0.0, 0.0),
        ridge(357.0, 12.0, 5, 300.0, vec![(70.0, 12.0, 110.0), (560.0, 11.0, 90.0)], "#9588a0", "#d5c8c1", 32.0, 75.0, 0.0, 0.01),
        ridge(378.0, 14.0, 7, 260.0, vec![(330.0, 15.0, 120.0)], "#7b738e", "#cdc0ba", 38.0, 80.0, 0.0, -0.012),
        ridge(404.0, 15.0, 9, 230.0, vec![(140.0, 14.0, 90.0), (560.0, 16.0, 110.0)], "#5f5b75", "#c4b8b1", 46.0, 95.0, 1.8, 0.015),
        ridge(438.0, 15.0, 11, 200.0, vec![(520.0, 12.0, 120.0), (250.0, -46.0, 180.0)], "#46465c", "#baaea7", 58.0, 110.0, 2.6, -0.03),
        ridge(468.0, 12.0, 13, 180.0, vec![(700.0, 34.0, 260.0), (260.0, -42.0, 220.0)], "#2f3244", "#afa39c", 70.0, 120.0, 3.6, -0.07),
    ];

    // the foreground ledge the wanderer stands on, running into the tor
    let ledge_pts: Vec<(f32, f32)> = vec![
        (-20.0, 520.0),
        (60.0, 515.0),
        (140.0, 507.0),
        (220.0, 501.0),
        (290.0, 498.0),
        (340.0, 500.0),
        (420.0, 507.0),
        (490.0, 503.0),
        (560.0, 494.0),
        (640.0, 484.0),
        (1020.0, 470.0),
    ];
    let ledge = Mask::from_shape(f, Shape::new().below(&ledge_pts, hgt + 20.0));
    // the crag: a jagged granite summit, steep toward the light, with a
    // gentler shoulder to the right where the spruces stand
    let mut grng = Rng::new(17);
    // (a few corners jut out as overhanging blocks: the steps and notches)
    let crag_ctrl = [
        (470.0, 507.0),
        (540.0, 496.0),
        (590.0, 489.0),
        (598.0, 476.0),
        (612.0, 467.0),
        (611.0, 452.0),
        (628.0, 440.0),
        (645.0, 432.0),
        (644.0, 414.0),
        (656.0, 399.0),
        (676.0, 390.0),
        (695.0, 386.0),
        (694.0, 367.0),
        (706.0, 352.0),
        (726.0, 344.0),
        (730.0, 326.0),
        (744.0, 316.0),
        (768.0, 309.0),
        (778.0, 297.0),
        (798.0, 289.0),
        (816.0, 276.0),
        (834.0, 279.0),
        (846.0, 292.0),
        (856.0, 289.0),
        (868.0, 301.0),
        (884.0, 302.0),
        (898.0, 311.0),
        (922.0, 318.0),
        (930.0, 330.0),
        (955.0, 332.0),
        (978.0, 344.0),
        (1030.0, 348.0),
    ];
    let mut outline = jag(&crag_ctrl, 0.07, 4, &mut grng);
    let top_line = outline.clone();
    // the foot of the crag follows the plateau, a little below its rim
    for i in (0..=24).rev() {
        let x = 470.0 + i as f32 * (1030.0 - 470.0) / 24.0;
        outline.push((x, ledge_y(&ledge_pts, x) + 10.0));
    }
    let crag0 = Mask::from_shape(f, Shape::new().poly(&outline));
    // weathered boulders perched on the summit and the steps (a boulder
    // field, as on the Riesengebirge tops)
    let perched: Vec<Block> = [(812.0f32, 8.0f32, 5.5f32, 0.1f32), (838.0, 10.0, 6.0, -0.08), (853.0, 6.0, 4.5, 0.05), (760.0, 14.0, 7.0, 0.1), (668.0, 12.0, 7.0, -0.05), (905.0, 16.0, 8.0, 0.08), (720.0, 11.0, 6.0, 0.0)]
        .iter()
        .enumerate()
        .map(|(k, &(x, rx, ry, t))| Block::shaped(x, top_of(&crag0, x) - ry * 0.55, rx, ry, t, 50 + k as u32, 2.4 + 0.3 * (k % 3) as f32))
        .collect();
    let crag = {
        let mut sh = Shape::new();
        for b in &perched {
            sh = sh.add(Shape::new().smooth_poly(&b.pts));
        }
        crag0.clone().union(&Mask::from_shape(f, sh))
    };
    let fg = ledge.clone().union(&crag);
    // ledges across the crag face: each the top of a step in the rock
    let ledges: Vec<Vec<(f32, f32)>> = [
        vec![(606.0, 469.0), (650.0, 466.0), (700.0, 470.0), (748.0, 476.0)],
        vec![(652.0, 399.0), (690.0, 402.0), (738.0, 410.0)],
        vec![(703.0, 353.0), (742.0, 357.0), (785.0, 364.0), (812.0, 370.0)],
        vec![(764.0, 305.0), (790.0, 310.0), (810.0, 317.0)],
        vec![(800.0, 432.0), (865.0, 426.0), (935.0, 433.0), (1010.0, 428.0)],
        vec![(760.0, 500.0), (850.0, 493.0), (930.0, 498.0), (1010.0, 490.0)],
    ]
    .iter()
    .map(|l| jag(l, 0.07, 3, &mut grng))
    .collect();
    let cross_x = 826.0;
    let cross_at = (cross_x, top_of(&crag, cross_x) + 4.0);
    let figure_at = (262.0, 500.0);

    // ------------------------------------------------------ underdrawing
    // graphite-like thin gray lines: the ridge crests, the blocks, the cross
    {
        let lead = pal.paint(hex("#5d5a58"), 0.55);
        let mut pencil = Held::new(Tool { width: 0.7, ..Tool::rigger(0.7) }, 5);
        for r in &ridges {
            let pts = curve(|x| r.crest(x));
            for seg in pts.chunks(30) {
                pencil.reload(lead, 0.25);
                c.drag(&mut pencil, &Gesture::new(seg.to_vec()).pressure(0.35, 0.3).shake(1.2), None);
            }
        }
        for seg in top_line.chunks(24).chain(ledges.iter().map(|l| &l[..])) {
            pencil.reload(lead, 0.3);
            c.drag(&mut pencil, &Gesture::new(seg.to_vec()).pressure(0.4, 0.35), None);
        }
        pencil.reload(lead, 0.3);
        c.drag(&mut pencil, &Gesture::line(cross_at, (cross_at.0, cross_at.1 - 150.0)).pressure(0.4, 0.4).shake(0.3), None);
        c.drag(&mut pencil, &Gesture::line((cross_at.0 - 32.0, cross_at.1 - 118.0), (cross_at.0 + 32.0, cross_at.1 - 118.0)).pressure(0.4, 0.4).shake(0.3), None);
        c.dry();
    }
    if o.stage(&mut c, "drawing") {
        return;
    }

    // ------------------------------------------------ thin underpainting
    // a warm umber wash over the foreground mass only
    {
        let h = st.body().color(|_, y| mix(hex("#4a3a2c"), hex("#3a2f28"), smoothstep(450.0, 600.0, y), Mix::Pigment)).medium(0.6).clip(true).threshold(0.05).length(20.0, 50.0);
        c.work(&fg, &h, 21);
        c.dry();
    }
    if o.stage(&mut c, "underpaint") {
        return;
    }

    // ---------------------------------------------------------------- sky
    // two layers, as in the Monk's sky: a deeper, cooler first layer, then
    // the lighter one over it, each fused with the badger; then pale light
    // over the glow
    {
        let sky_mask = Mask::from_fn(f, |_, y| if y < 480.0 { 1.0 } else { 0.0 });
        let deeper = |x: f32, y: f32| {
            let s = sky(x, y);
            mix(s, hex("#7d86a3"), 0.12 * (1.0 - smoothstep(150.0, 350.0, y)), Mix::Pigment)
        };
        let h1 = st.broad().color(deeper).coverage(3.0).medium(0.25);
        c.work(&sky_mask, &h1, 31);
        if let Some(b) = st.blend() {
            c.work(&sky_mask, &b, 32);
            c.work(&sky_mask, &b, 132);
        }
        c.dry();
        let h2 = st.broad().color(sky).coverage(3.2).medium(0.22).mix_jitter(0.03);
        c.work(&sky_mask, &h2, 33);
        if let Some(b) = st.blend() {
            for p in 0..st.blend_passes + 1 {
                c.work(&sky_mask, &b, 34 + p as u64);
            }
        }
        c.dry();
        // a third, lead-white-rich layer over the glow only, as in the Monk's
        // sky (light blue, then pink and white over the center, CATS p.127):
        // it goes on where the brush carries more, and is fused
        let glow = |x: f32, y: f32| {
            let dx = (x - SUN.0) / 330.0;
            let dy = (y - SUN.1) / 120.0;
            (-(dx * dx + dy * dy)).exp()
        };
        let glow_mask = Mask::from_fn(f, |x, y| if y < 470.0 && glow(x, y) > 0.08 { 1.0 } else { 0.0 });
        let h3 = st.broad()
            .color(|x, y| mix(sky(x, y), hex("#f6eed8"), 0.35 * glow(x, y), Mix::Light))
            .coverage(2.5)
            .medium(0.3)
            .mix_jitter(0.03)
            .load_at(glow);
        c.work(&glow_mask, &h3, 37);
        if let Some(b) = st.blend() {
            c.work(&glow_mask, &b, 38);
            c.work(&glow_mask, &b, 39);
        }
        c.dry();
    }
    if o.stage(&mut c, "sky") {
        return;
    }

    // ------------------------------------------------------------- clouds
    // long thin bars, lit from below by the sun still under the ridge
    {
        let bars: [(f32, f32, f32, f32); 5] = [
            (40.0, 430.0, 70.0, 7.0),
            (520.0, 1010.0, 118.0, 9.0),
            (-10.0, 300.0, 162.0, 6.0),
            (230.0, 590.0, 212.0, 5.0),
            (660.0, 900.0, 246.0, 4.0),
        ];
        for (i, &(x0, x1, y, th)) in bars.iter().enumerate() {
            let n = (th / 2.0).ceil() as usize + 1;
            let mut b = Held::new(Tool { ragged: 0.45, ..Tool::filbert(5.0) }, 100 + i as u64);
            let wob = Fbm::new(200 + i as u32, 3, 90.0);
            for k in 0..n {
                let s = k as f32 / (n - 1).max(1) as f32; // 0 top .. 1 bottom
                let yy = y - th * 0.5 + s * th;
                let around = sky(0.5 * (x0 + x1), yy);
                let body = mix(hex("#9f96a8"), hex("#e3c2b0"), smoothstep(0.3, 1.0, s), Mix::Light);
                let col = mix(around, body, 0.45, Mix::Light);
                b.reload(pal.paint(col, 0.55), 0.25);
                let inset = th * 3.0 * (1.0 - (2.0 * s - 1.0).abs()).max(0.0);
                let pts: Vec<(f32, f32)> = (0..=12)
                    .map(|j| {
                        let t = j as f32 / 12.0;
                        let x = x0 + (x1 - x0) * t - inset * (0.5 - t);
                        (x, yy + wob.get(x, y) * 5.0 + (t - 0.5).powi(2) * 8.0)
                    })
                    .collect();
                c.drag(&mut b, &Gesture::new(pts).pressure(0.55, 0.4).ramps(0.3, 0.4).orient(Orient::Across), None);
            }
            let mut bl = Held::new(Tool { pickup: 0.15, ..Tool::badger(16.0) }, 150 + i as u64);
            for k in 0..2 {
                let yy = y - th * 0.3 + k as f32 * th * 0.6;
                let pts: Vec<(f32, f32)> = (0..=8).map(|j| (x0 + (x1 - x0) * j as f32 / 8.0, yy + wob.get(x0 + (x1 - x0) * j as f32 / 8.0, y) * 5.0)).collect();
                c.drag(&mut bl, &Gesture::new(pts).pressure(0.35, 0.3), None);
                bl.wipe(0.9);
            }
        }
        c.dry();
    }
    // the morning star above the glow: one touch of lead white
    {
        let mut b = Held::new(Tool::round_sable(2.4), 170);
        b.load(pal.paint(hex("#f4efe4"), 0.1), 0.6);
        c.drag(&mut b, &Gesture::line((548.0, 118.0), (548.3, 119.2)).pressure(0.75, 0.6).ramps(0.0, 0.3).shake(0.0), None);
        c.dry();
    }
    if o.stage(&mut c, "clouds") {
        return;
    }

    // ----------------------------------------------- the ridges, far to near
    for (k, r) in ridges.iter().enumerate() {
        let sc = f.scale;
        let m = Mask::from_fn(f, |x, y| {
            let d = y - r.crest(x);
            ((d * sc + 0.5).clamp(0.0, 1.0)) * (1.0 - smoothstep(r.depth * 0.7, r.depth, d))
        });
        let hd = Handling::new(Tool { lay: 0.9, ..Tool::filbert(6.0 + k as f32) })
            .mixed(&pal, 0.38 - 0.05 * k as f32)
            .mix_jitter(0.04)
            .color(|x, y| r.color(x, y))
            .angle(|x, _| r.slope(x).atan() * 0.7)
            .angle_jitter(0.05)
            .length(30.0, 80.0)
            .coverage(3.2 + 0.2 * k as f32)
            .pressure(0.55, 0.8)
            .dips(2, 0.5, 0.6)
            .clip(true)
            .threshold(0.05);
        c.work(&m, &hd, 40 + k as u64);
        if let Some(b) = st.blend() {
            let b = b.clip(true).threshold(0.05).pressure(0.3, 0.4).length(60.0, 160.0).angle(|x, _| r.slope(x).atan() * 0.7);
            c.work(&m, &b, 60 + k as u64);
            c.work(&m, &b, 160 + k as u64);
        }
        c.dry();
        // forest on the nearer crests: small spires in the ridge's own dark
        if r.forest > 0.0 {
            let mut rng = Rng::new(80 + k as u64);
            let dark = pal.paint(mix(r.col, hex("#1c2026"), 0.25, Mix::Pigment), 0.35);
            let woods = Fbm::new(70 + k as u32, 2, 160.0);
            let canopy = Fbm::new(75 + k as u32, 3, 25.0);
            let in_wood = |x: f32| woods.get(x, 1.0) > -0.15;
            // first the body of the wood: a band along the crest, lean
            let mut band = Held::new(Tool { ragged: 0.5, ..Tool::round_sable(r.forest * 0.9) }, 85 + k as u64);
            let mut x = -10.0;
            while x < 1010.0 {
                while x < 1010.0 && !in_wood(x) {
                    x += 2.0;
                }
                let mut run = vec![];
                while x < 1010.0 && in_wood(x) {
                    run.push((x, r.crest(x) + r.forest * 0.35));
                    x += 3.0;
                }
                if run.len() > 2 {
                    band.reload(dark, 0.4);
                    c.drag(&mut band, &Gesture::new(run).pressure(0.6, 0.6).ramps(0.1, 0.1).shake(0.2), None);
                }
            }
            // then the crowns: close, overlapping spires, each a tapering
            // upward touch of a fine brush, heights rising and falling
            let mut x = -10.0;
            let mut n = 0;
            while x < 1010.0 {
                x += rng.range(0.25, 0.7) * r.forest * 0.5;
                if !in_wood(x) {
                    continue;
                }
                let hh = r.forest * (0.8 + 0.5 * canopy.get(x, 2.0)) * rng.range(0.7, 1.3);
                let w = (hh * 0.42).max(0.5);
                let mut b = Held::new(Tool { stiffness: 0.3, ragged: 0.1, length: w * 0.3, ..Tool::round_sable(w) }, 9000 + n);
                b.load(dark, 0.3);
                n += 1;
                let y0 = r.crest(x) + r.forest * 0.5;
                let lean = rng.normal() * 0.15;
                let pts = vec![(x, y0), (x + lean * 0.5, y0 - hh * 0.5), (x + lean, y0 - hh)];
                c.drag(&mut b, &Gesture::new(pts).pressure(0.85, 0.02).ramps(0.0, 0.95).shake(0.2), None);
            }
            c.dry();
        }
    }
    if o.stage(&mut c, "distance") {
        return;
    }

    // ---------------------------------------------------------- foreground
    // the ledge: dark earth, turf and heather against the misted valley
    {
        let mot = Fbm::new(310, 4, 60.0);
        let h = st.body()
            .color(|x, y| {
                let d = y - ledge_y(&ledge_pts, x) + 14.0 * mot.get(x * 2.0, 7.0);
                let m = mot.get01(x, y);
                // the plateau near its rim takes the sky; it darkens toward us
                let rim = mix(hex("#57553f"), hex("#6a6450"), m, Mix::Pigment);
                let top = mix(hex("#3a3a2e"), hex("#4b4634"), m, Mix::Pigment);
                let s = mix(rim, top, smoothstep(0.0, 22.0, d), Mix::Pigment);
                mix(s, hex("#1f1e1a"), smoothstep(20.0, 115.0, d), Mix::Pigment)
            })
            .angle(|_, _| 0.04)
            .coverage(3.5)
            .medium(0.15)
            .clip(true)
            .threshold(0.05);
        c.work(&ledge, &h, 70);
        c.dry();
        // heather and turf: short stippled clumps, reddish-brown and dark olive
        let clumps = Handling::new(Tool { ragged: 0.6, ..Tool::filbert(2.8) })
            .mixed(&pal, 0.2)
            .mix_jitter(0.12)
            .color(|x, y| {
                let k = mot.get01(x * 3.0, y * 3.0);
                let c0 = mix(hex("#302f24"), hex("#3d3328"), k, Mix::Pigment);
                let d = y - ledge_y(&ledge_pts, x) + 14.0 * mot.get(x * 2.0, 7.0);
                let c0 = mix(c0, hex("#57533f"), (1.0 - smoothstep(0.0, 15.0, d)) * 0.35, Mix::Pigment);
                mix(c0, hex("#1d1a17"), smoothstep(15.0, 115.0, d), Mix::Pigment)
            })
            .length(3.0, 7.0)
            .coverage(1.2)
            .pressure(0.5, 0.8)
            .dips(3, 0.5, 0.5)
            .angle(|_, _| -1.4)
            .angle_jitter(0.5)
            .clip(true)
            .threshold(0.3);
        c.work(&ledge, &clumps, 71);
        c.dry();
        // the nearest ground darker still, toward the lower edge (the
        // repoussoir; Friedrich's advice to darken toward the edges)
        let low = Mask::from_fn(f, |x, y| smoothstep(30.0, 110.0, y - ledge_y(&ledge_pts, x) + 25.0 * mot.get(x * 1.5, 3.0)));
        let deep = st.body()
            .color(|x, y| mix(hex("#221f1b"), hex("#141211"), smoothstep(560.0, 625.0, y) + 0.2 * mot.get(x, y), Mix::Pigment))
            .angle(|x, _| 0.03 + 0.25 * mot.get(x, 9.0))
            .angle_jitter(0.15)
            .length(12.0, 35.0)
            .coverage(3.0)
            .medium(0.3)
            .load_at(|x, y| 0.25 + 0.75 * smoothstep(40.0, 120.0, y - ledge_y(&ledge_pts, x)))
            .threshold(0.05);
        c.work(&low, &deep, 72);
        // and the very edge, closed with a few long pulls of a wide brush
        let mut edge = Held::new(Tool::filbert(14.0), 73);
        for k in 0..6 {
            edge.reload(pal.paint(hex("#151311"), 0.3), 0.7);
            let x0 = -20.0 + k as f32 * 180.0;
            c.drag(&mut edge, &Gesture::line((x0, hgt - 2.5), (x0 + 220.0, hgt - 3.0)).pressure(0.8, 0.8).ramps(0.05, 0.05), None);
        }
        c.dry();
        // stones lying on the plateau, seen from above
        let stones = vec![
            Block::new(96.0, 523.0, 18.0, 6.0, 0.05, 31),
            Block::new(122.0, 519.0, 8.0, 3.5, -0.1, 32),
            Block::new(402.0, 514.0, 14.0, 5.0, -0.04, 33),
            Block::new(455.0, 531.0, 20.0, 7.0, 0.06, 34),
            Block::new(205.0, 544.0, 11.0, 4.0, 0.0, 37),
        ];
        paint_blocks(&mut c, &st, &pal, &stones, 600, 0.4);
    }
    // the crag: one dark mass against the dawn (Friedrich's earthly dark
    // under the luminous sky), then the light it catches, its ledges and
    // fissures, lichen, dwarf pine
    {
        let mot = Fbm::new(320, 4, 36.0);
        let grain = Fbm::new(321, 3, 8.0);
        let strata = Fbm::new(322, 3, 90.0);
        let joints = Fbm::new(323, 3, 1.0);
        let dome = crag.clone().blur(16.0);
        // the painter's idea of the rock's relief (height toward the viewer):
        // a rounded mass, stepped by the bedding, split by vertical joints
        let relief = |x: f32, y: f32| -> f32 {
            let d = dome.data[f.index(x, y)];
            let yy = y + 0.1 * (x - 800.0) + 10.0 * strata.get(x, y);
            let q = yy / 21.0;
            // a sawtooth: each bed leans back a little, then a sharp riser
            let step = smoothstep(0.72, 1.0, q.fract()) - q.fract();
            let vj = joints.get(x / 7.0, y / 60.0);
            40.0 * d + 5.0 * step * d + 3.0 * vj * d + 2.0 * grain.get(x, y)
        };
        let normal = |x: f32, y: f32| -> (f32, f32, f32) {
            let e = 1.5;
            let dx = (relief(x + e, y) - relief(x - e, y)) / (2.0 * e);
            let dy = (relief(x, y + e) - relief(x, y - e)) / (2.0 * e);
            let l = (dx * dx + dy * dy + 1.0).sqrt();
            (-dx / l, -dy / l, 1.0 / l)
        };
        // the light it would catch: sky from above (cool), the glow from the
        // left and behind (warm); faces toward us stay in shadow
        let rock = |x: f32, y: f32| -> Rgb {
            let (nx, ny, nz) = normal(x, y);
            let m = mot.get01(x, y) * 0.7 + grain.get01(x, y) * 0.3;
            let mut s = mix(hex("#1d1a1a"), hex("#302b28"), m, Mix::Pigment);
            s = mix(s, hex("#3a3530"), smoothstep(440.0, 520.0, y) * 0.35, Mix::Pigment);
            let up = (-ny).max(0.0);
            let glow = (-nx * 0.95 - nz * 0.1).max(0.0);
            s = mix(s, hex("#55545c"), (up.powf(1.4) * 0.9).min(0.6), Mix::Pigment);
            mix(s, hex("#5f4936"), (glow.powf(1.3) * 0.9).min(0.6), Mix::Pigment)
        };
        let body = st.body()
            .color(rock)
            .angle(|x, y| if normal(x, y).0.abs() > 0.45 { 1.45 } else { -0.1 })
            .angle_jitter(0.12)
            .length(8.0, 24.0)
            .coverage(3.6)
            .medium(0.12)
            .clip(true)
            .threshold(0.05);
        c.work(&crag, &body, 80);
        if let Some(bl) = st.blend() {
            let bl = bl.clip(true).threshold(0.05).pressure(0.2, 0.3).length(12.0, 30.0).angle(|_, _| -0.1);
            c.work(&crag, &bl, 81);
        }
        c.dry();
        paint_blocks(&mut c, &st, &pal, &perched, 700, 0.6);

        let dark = pal.paint(hex("#141110"), 0.2);
        let mut rng = Rng::new(333);
        // fissures: from the lit steps down, wandering, tapering
        let mut fb = Held::new(Tool { ragged: 0.4, ..Tool::round_sable(1.2) }, 370);
        let mut steps_lit: Vec<(f32, f32)> = vec![];
        let mut tries = 0;
        while steps_lit.len() < 60 && tries < 20000 {
            tries += 1;
            let (x, y) = (rng.range(590.0, 1000.0), rng.range(285.0, 500.0));
            if crag0.data[f.index(x, y)] > 0.5 && crag0.data[f.index(x, y - 8.0)] > 0.5 && -normal(x, y).1 > 0.45 {
                steps_lit.push((x, y));
            }
        }
        for &(x0, y0) in steps_lit.iter().take(16) {
            let len = rng.range(22.0, 60.0);
            let line = jag(&[(x0, y0 + 3.0), (x0 + rng.normal() * 4.0, y0 + 3.0 + len)], 0.06, 3, &mut rng);
            fb.reload(dark, 0.55);
            c.drag(&mut fb, &Gesture::new(line).pressure(0.75, 0.3).ramps(0.05, 0.5), Some(&crag));
        }
        // the silhouette catches the glow: a hair of warm light along the
        // edges that face it
        let rim = pal.paint(hex("#c29f7b"), 0.45);
        let mut rb = Held::new(Tool::round_sable(1.1), 380);
        let mut run: Vec<(f32, f32)> = vec![];
        for w in top_line.windows(2) {
            let (p, q) = (w[0], w[1]);
            let (dx, dy) = (q.0 - p.0, q.1 - p.1);
            let l = (dx * dx + dy * dy).sqrt().max(1e-4);
            let (nx, ny) = (dy / l, -dx / l);
            if -nx * 0.7 - ny * 0.6 > 0.25 && p.0 > 560.0 {
                run.push((p.0 - nx * 1.0, p.1 - ny * 1.0));
            } else if run.len() > 2 {
                rb.reload(rim, 0.25);
                c.drag(&mut rb, &Gesture::new(std::mem::take(&mut run)).pressure(0.4, 0.25).ramps(0.2, 0.4), Some(&crag));
            } else {
                run.clear();
            }
        }
        c.dry();
        // lichen: sparse flat flecks of gray-green and ochre where light falls
        let mut lb = Held::new(Tool { ragged: 0.5, ..Tool::round_sable(2.0) }, 390);
        let mut k = 0;
        let mut tries = 0;
        while k < 120 && tries < 20000 {
            tries += 1;
            let x = rng.range(560.0, 1000.0);
            let y = rng.range(280.0, 520.0);
            if crag.data[f.index(x, y)] < 0.5 || (-normal(x, y).1 < 0.3 && rng.chance(0.85)) {
                continue;
            }
            if k % 3 == 0 {
                let col = if rng.chance(0.5) { hex("#4c4e44") } else { hex("#584b39") };
                lb.reload(pal.paint(col, 0.45), 0.2);
            }
            k += 1;
            let l = rng.range(1.5, 3.5);
            c.drag(&mut lb, &Gesture::line((x, y), (x + l, y + rng.normal() * 0.4)).pressure(0.45, 0.3).ramps(0.2, 0.4), Some(&crag));
        }
        c.dry();
        // dwarf mountain pine (Knieholz) crouching on the ledges and at the foot
        for (k, &(x, y)) in steps_lit.iter().skip(16).take(9).enumerate() {
            knieholz(&mut c, &pal, (x, y + 1.0), rng.range(7.0, 15.0), 400 + k as u64);
        }
        for (k, &(x, r)) in [(520.0f32, 22.0f32), (470.0, 12.0), (598.0, 14.0), (985.0, 16.0)].iter().enumerate() {
            let y = top_of(&fg, x) + 2.0;
            knieholz(&mut c, &pal, (x, y), r, 420 + k as u64);
        }
        // and along the crag's foot, where it meets the plateau
        for (k, &(x, r)) in [(660.0f32, 18.0f32), (735.0, 12.0), (820.0, 22.0), (905.0, 14.0), (960.0, 18.0)].iter().enumerate() {
            knieholz(&mut c, &pal, (x, ledge_y(&ledge_pts, x) + 9.0), r, 430 + k as u64);
        }
        c.dry();
    }
    if o.stage(&mut c, "tor") {
        return;
    }

    // ------------------------------------------------ spruces and the cross
    let tree_dark = pal.mix(hex("#1d221f")).color;
    let tree_rim = pal.mix(hex("#7a6a52")).color;
    for (i, &(x, hh)) in [(912.0, 176.0), (952.0, 122.0), (988.0, 150.0), (745.0, 40.0), (870.0, 62.0)].iter().enumerate() {
        let y = top_of(&crag, x) + 5.0;
        spruce(&mut c, &pal, (x, y), hh, tree_dark, tree_rim, 700 + i as u64);
    }
    c.dry();
    cross(&mut c, &pal, cross_at, 150.0, 800);
    c.dry();
    if o.stage(&mut c, "trees") {
        return;
    }

    // ----------------------------------------------------------- the figure
    {
        let cape = pal.mix(hex("#2b2d29")).color;
        let dark = pal.mix(hex("#161412")).color;
        let hair = pal.mix(hex("#3a2c20")).color;
        let rim = pal.mix(hex("#9a8466")).color;
        paintings::figures::man_in_cape(&mut c, figure_at, 50.0, cape, dark, hair, Some(rim), 900);
        c.dry();
    }
    if o.stage(&mut c, "figure") {
        return;
    }

    // ------------------------------------------ grasses, last, upturned
    {
        let mut rng = Rng::new(1000);
        let mut b = Held::new(Tool::round_sable(1.0), 1001);
        let mut k = 0;
        while k < 900 {
            let x = rng.range(-10.0, 1010.0);
            let top = ledge_y(&ledge_pts, x);
            let y = top + rng.range(0.0, 1.0f32).powf(1.6) * (hgt - top + 10.0) + 1.0;
            if crag.data[f.index(x, y - 2.0)] > 0.5 {
                continue;
            }
            k += 1;
            let near = ((y - top) / (hgt - top)).clamp(0.0, 1.0);
            let lit = rng.chance(0.2) && y < top + 20.0;
            let col = if lit {
                hex("#686044")
            } else {
                mix(hex("#2a2a20"), hex("#11100e"), smoothstep(0.1, 0.6, near), Mix::Pigment)
            };
            if k % 4 == 0 {
                b.reload(pal.paint(col, 0.25), 0.5);
            }
            // tufts grow with nearness (perspective)
            let len = rng.range(5.0, 12.0) * (1.0 + 2.2 * near);
            let lean = rng.normal() * 0.3 + 0.1;
            let pts = vec![(x, y), (x + lean * len * 0.5, y - len * 0.6), (x + lean * len, y - len)];
            c.drag(&mut b, &Gesture::new(pts).pressure(0.7, 0.1).ramps(0.0, 0.6), None);
        }
        c.dry();
    }
    if o.stage(&mut c, "grass") {
        return;
    }

    // a lighter, younger varnish than the stock aged finish: the Friedrichs
    // cleaned in 2013–16 read cooler (SMB-blog)
    let fin = Finish { varnish_coats: 0.25, ..Finish::aged(st.relief) };
    o.finish(&mut c, &fin);
}

/// Paint a pile of granite blocks, bottom first (later blocks stand in
/// front of / on top of earlier ones). Each block is a rounded box lit by the
/// dawn sky on its top (when seen from above), by the glow at the left on its
/// left end, and in shadow toward the viewer; the blocks part by value, not
/// by outline. Then the seams they rest on, lit edges and a few fissures.
fn paint_blocks(c: &mut Canvas, st: &Style, pal: &Palette, blocks: &[Block], seed: u64, lit: f32) {
    let f = c.frame();
    let front = |x: f32, y: f32| blocks.iter().rposition(|b| b.inside(x, y));
    let mot = Fbm::new(320, 4, 30.0);
    let grain = Fbm::new(321, 3, 9.0);
    let mut rng = Rng::new(seed);
    let dark = pal.paint(hex("#1a1715"), 0.2);
    let rim = pal.paint(hex("#b99a7c"), 0.45);
    let cool = pal.paint(hex("#7e7b84"), 0.45);
    for (i, b) in blocks.iter().enumerate() {
        // what of this block shows: its shape less the blocks in front
        let mut vis = Mask::from_shape(f, Shape::new().smooth_poly(&b.pts));
        for o2 in &blocks[i + 1..] {
            vis = vis.subtract(&Mask::from_shape(f, Shape::new().smooth_poly(&o2.pts)));
        }
        let seen_from_above = b.cy - b.ry * 0.6 > EYE;
        let shade = rng.f();
        let col = |x: f32, y: f32| -> Rgb {
            let (u, v) = b.uv(x, y);
            let m = mot.get01(x, y) * 0.7 + grain.get01(x, y) * 0.3;
            // each block its own stone: a little lighter or darker
            let face = mix(hex("#221e1b"), hex("#342d27"), (m * 0.8 + shade * 0.5).min(1.0), Mix::Pigment);
            // rounding toward the edges: normal turns left / up / down
            let left = smoothstep(-0.6, -0.97, u);
            let top = smoothstep(-0.65, -0.95, v);
            let bottom = smoothstep(0.7, 0.98, v);
            let mut s = mix(face, hex("#4d3c2f"), left * 0.5 * lit, Mix::Pigment);
            s = mix(s, hex("#171413"), bottom * 0.5, Mix::Pigment);
            if seen_from_above {
                let sky_lit = mix(hex("#57555c"), hex("#655a4e"), left, Mix::Pigment);
                s = mix(s, sky_lit, top * 0.8 * lit, Mix::Pigment);
            } else {
                s = mix(s, hex("#3c3634"), top * 0.35 * lit, Mix::Pigment);
            }
            s
        };
        let (tilt, cx, cy, rx, ry) = (b.tilt, b.cx, b.cy, b.rx, b.ry);
        let h = st.body()
            .color(col)
            .angle(move |x, y| {
                let (u, v) = ((x - cx) / rx, (y - cy) / ry);
                // strokes follow the rounding: along the top and bottom,
                // down the ends
                let side = smoothstep(0.55, 0.95, u.abs()) * (1.0 - smoothstep(0.6, 0.95, v.abs()));
                tilt + 0.25 * u * v + side * 1.3 * u.signum()
            })
            .length(8.0, 26.0)
            .coverage(3.5)
            .medium(0.12)
            .clip(true)
            .threshold(0.05);
        c.work(&vis, &h, seed + 1 + i as u64);
        if let Some(bl) = st.blend() {
            let bl = bl.clip(true).threshold(0.05).pressure(0.25, 0.35).length(15.0, 40.0).angle(move |_, _| tilt);
            c.work(&vis, &bl, seed + 101 + i as u64);
        }
        c.dry();
        // the shadowed seam where the block rests: only its flat underside
        let n = b.pts.len();
        let covered = |p: (f32, f32)| blocks[i + 1..].iter().any(|o2| o2.inside(p.0, p.1));
        let resting = |p: (f32, f32)| blocks[..i].iter().any(|o2| o2.inside(p.0, p.1 + 3.0));
        let mut jb = Held::new(Tool { ragged: 0.5, ..Tool::round_sable(1.8) }, seed + 201 + i as u64);
        let mut run: Vec<(f32, f32)> = vec![];
        let seam = |c: &mut Canvas, run: &mut Vec<(f32, f32)>, jb: &mut Held| {
            if run.len() > 3 {
                jb.reload(dark, 0.45);
                c.drag(jb, &Gesture::new(std::mem::take(run)).pressure(0.7, 0.4).ramps(0.3, 0.4), None);
            }
            run.clear();
        };
        for j in 0..n {
            let p = b.pts[j];
            let (_, v) = b.uv(p.0, p.1);
            if v > 0.8 && resting(p) && !covered(p) {
                run.push((p.0, p.1 - 0.5));
            } else {
                seam(c, &mut run, &mut jb);
            }
        }
        seam(c, &mut run, &mut jb);
        // light caught along the upper edge: warm toward the dawn, cool on top
        let mut rb = Held::new(Tool::round_sable(1.3), seed + 301 + i as u64);
        let mut last_warm = false;
        for j in 0..=n {
            let p = b.pts[j % n];
            let (u, v) = b.uv(p.0, p.1);
            let facing = v < -0.75 || (u < -0.85 && v < 0.3);
            if facing && !covered(p) {
                run.push((p.0 + 0.8, p.1 + 1.2));
                last_warm = u < -0.6;
            } else if run.len() > 3 {
                rb.reload(if last_warm { rim } else { cool }, 0.2 * lit * lit);
                c.drag(&mut rb, &Gesture::new(std::mem::take(&mut run)).pressure(0.4, 0.2).ramps(0.25, 0.45), Some(&vis));
            } else {
                run.clear();
            }
        }
        if run.len() > 3 {
            rb.reload(cool, 0.2 * lit * lit);
            c.drag(&mut rb, &Gesture::new(std::mem::take(&mut run)).pressure(0.4, 0.2).ramps(0.25, 0.45), Some(&vis));
        }
        run.clear();
        // a weathering fissure or two, short and tapering
        let mut fb = Held::new(Tool::round_sable(1.1), seed + 401 + i as u64);
        for _ in 0..(b.rx / 50.0) as usize + 1 {
            let x = b.cx + rng.range(-0.6, 0.6) * b.rx;
            let y0 = b.cy - b.ry * rng.range(0.5, 0.9);
            if front(x, y0) != Some(i) || rng.chance(0.3) {
                continue;
            }
            fb.reload(dark, 0.35);
            let len = b.ry * rng.range(0.5, 1.2);
            let pts = vec![(x, y0), (x + rng.normal() * 1.2, y0 + len * 0.5), (x + rng.normal() * 2.0, y0 + len)];
            c.drag(&mut fb, &Gesture::new(pts).pressure(0.5, 0.1).ramps(0.1, 0.6), Some(&vis));
        }
        // a vertical joint through the bigger blocks: the granite splits
        // into cubes, the seams open a little and hold shadow
        if b.rx > 45.0 && lit > 0.9 {
            let mut vj = Held::new(Tool { ragged: 0.5, ..Tool::round_sable(2.0) }, seed + 501 + i as u64);
            let x = b.cx + rng.range(-0.35, 0.45) * b.rx;
            let (st_, ct_) = b.tilt.sin_cos();
            let pts: Vec<(f32, f32)> = (0..=4)
                .map(|k| {
                    let v = -0.95 + 1.9 * k as f32 / 4.0;
                    let (lx, ly) = (x - b.cx + rng.normal() * 1.5, v * b.ry);
                    (b.cx + lx * ct_ - ly * st_, b.cy + lx * st_ + ly * ct_)
                })
                .collect();
            vj.reload(dark, 0.5);
            c.drag(&mut vj, &Gesture::new(pts).pressure(0.75, 0.6).ramps(0.1, 0.2), Some(&vis));
        }
        c.dry();
    }
}

/// Midpoint displacement: roughen a polyline the way rock edges break,
/// `rough` relative to each segment's length.
fn jag(pts: &[(f32, f32)], rough: f32, levels: usize, rng: &mut Rng) -> Vec<(f32, f32)> {
    let mut p = pts.to_vec();
    for _ in 0..levels {
        let mut q = Vec::with_capacity(p.len() * 2);
        for w in p.windows(2) {
            let (a, b) = (w[0], w[1]);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let l = (dx * dx + dy * dy).sqrt().max(1e-4);
            let (nx, ny) = (-dy / l, dx / l);
            let t = rng.range(0.35, 0.65);
            let o = rng.normal() * rough * l;
            q.push(a);
            q.push((a.0 + dx * t + nx * o, a.1 + dy * t + ny * o));
        }
        q.push(*p.last().unwrap());
        p = q;
    }
    p
}

/// The first row (from the top) where a mask is solid at `x`.
fn top_of(m: &Mask, x: f32) -> f32 {
    let f = m.f;
    let mut y = 0.0;
    while y < f.height() && m.data[f.index(x, y)] < 0.5 {
        y += 0.5;
    }
    y
}

fn ledge_y(pts: &[(f32, f32)], x: f32) -> f32 {
    for w in pts.windows(2) {
        if x >= w[0].0 && x <= w[1].0 {
            let t = (x - w[0].0) / (w[1].0 - w[0].0);
            return w[0].1 + (w[1].1 - w[0].1) * t;
        }
    }
    pts[pts.len() - 1].1
}

/// Dwarf mountain pine: a low, dense crouching clump of short dark strokes
/// radiating up and out, with a few lit needles on the dawn side.
fn knieholz(c: &mut Canvas, pal: &Palette, at: (f32, f32), r: f32, seed: u64) {
    let mut rng = Rng::new(seed);
    let dark = pal.paint(hex("#1c211c"), 0.25);
    let mid = pal.paint(hex("#2c3326"), 0.3);
    let mut b = Held::new(Tool { ragged: 0.5, ..Tool::round_sable(1.6) }, seed);
    for k in 0..(r * 5.0) as usize {
        if k % 6 == 0 {
            b.reload(if rng.chance(0.7) { dark } else { mid }, 0.5);
        }
        let u = rng.range(-1.0, 1.0);
        let top = (1.0 - u * u).max(0.0).sqrt() * r * 0.55;
        let x = at.0 + u * r;
        let y = at.1 - rng.range(0.0, 1.0) * top;
        let a = -std::f32::consts::FRAC_PI_2 + u * 0.9 + rng.normal() * 0.3;
        let l = rng.range(2.0, 5.0);
        c.drag(&mut b, &Gesture::line((x, y + 1.0), (x + a.cos() * l, y + a.sin() * l)).pressure(0.8, 0.2).ramps(0.0, 0.6), None);
    }
    c.dry();
    let lit = pal.paint(hex("#6b6446"), 0.4);
    let mut l = Held::new(Tool::round_sable(1.0), seed + 1);
    l.load(lit, 0.3);
    for _ in 0..(r * 0.8) as usize {
        let u = rng.range(-1.0, -0.2);
        let top = (1.0 - u * u).max(0.0).sqrt() * r * 0.55;
        let x = at.0 + u * r;
        let y = at.1 - top * rng.range(0.6, 1.0);
        c.drag(&mut l, &Gesture::line((x, y), (x - 1.5, y - 2.0)).pressure(0.5, 0.2).ramps(0.0, 0.6), None);
    }
}

/// A spruce as Friedrich places it: a straight dark spire of drooping
/// tiers, the trunk first, then tiers from the top down; the side toward the
/// dawn gets a few lean touches of light on the branch tips.
fn spruce(c: &mut Canvas, pal: &Palette, base: (f32, f32), height: f32, dark: Rgb, rim: Rgb, seed: u64) {
    let mut h = paint::Hand::new(base, height, seed);
    h.tremor = 0.002;
    let p = paint::Paint { color: dark, hiding: 0.92, stiff: 0.6 };
    let mut t = h.take(Tool::round_sable, 0.014, p, 0.6);
    h.mark(c, &mut t, paint::Mark { pts: &[(0.0, 0.0), (0.002, 0.5), (0.0, 1.0)], pressure: (0.9, 0.3), ramps: (0.0, 0.3) }, None);
    let mut b = h.take(Tool::round_sable, 0.026, p, 0.6);
    // near the top the tiers are short twigs: a finer brush, so the crown
    // ends in a point, not a knob
    let mut fine = h.take(Tool::round_sable, 0.011, p, 0.5);
    let tiers = 26 + (height / 10.0).min(20.0) as usize;
    let slim = h.rng.range(0.15, 0.2);
    let mut tips = vec![];
    for i in 0..tiers {
        let v = 0.97 - (i as f32 + h.rng.range(0.0, 0.6)) / tiers as f32 * 0.9;
        let reach = (0.012 + slim * (1.0 - v)) * h.rng.range(0.7, 1.1);
        if i % 3 == 0 {
            b.reload(p, 0.6);
            fine.reload(p, 0.5);
        }
        let brush = if v > 0.82 { &mut fine } else { &mut b };
        for s in [-1.0f32, 1.0] {
            let droop = reach * h.rng.range(0.3, 0.5);
            let pts = [(s * 0.004, v), (s * reach * 0.55, v - droop), (s * reach, v - droop * 0.75)];
            h.mark(c, brush, paint::Mark { pts: &pts, pressure: (0.9, 0.35), ramps: (0.0, 0.5) }, None);
            if s < 0.0 {
                tips.push((s * reach * 0.8, v - droop * 0.7));
            }
        }
    }
    c.dry();
    // the leader: a fine pointed flick above the top tier
    let mut ld = h.take(Tool::rigger, 0.005, p, 0.5);
    h.mark(c, &mut ld, paint::Mark { pts: &[(0.0, 0.9), (0.0005, 0.97), (0.001, 1.035)], pressure: (0.9, 0.0), ramps: (0.0, 0.8) }, None);
    // a few lean hairs of light along the tips that face the glow
    let lp = pal.paint(mix(dark, rim, 0.6, Mix::Pigment), 0.5);
    let mut l = h.take(Tool::rigger, 0.004, lp, 0.15);
    for (k, &(u, v)) in tips.iter().enumerate() {
        if k % 3 != 0 {
            continue;
        }
        l.reload(lp, 0.15);
        h.mark(c, &mut l, paint::Mark { pts: &[(u + 0.035, v + 0.006), (u + 0.015, v + 0.002), (u, v)], pressure: (0.3, 0.15), ramps: (0.3, 0.5) }, None);
    }
}

/// A plain wooden cross: the upright set into a cleft, the crossbar pegged
/// on, a thin line of dawn light down the edge that faces the sun.
fn cross(c: &mut Canvas, pal: &Palette, at: (f32, f32), height: f32, seed: u64) {
    let wood = pal.paint(hex("#211c18"), 0.15);
    let mut b = Held::new(Tool { ragged: 0.2, ..Tool::round_sable(4.0) }, seed);
    for k in 0..2 {
        b.reload(wood, 0.9);
        let dx = k as f32 * 0.8 - 0.4;
        c.drag(&mut b, &Gesture::line((at.0 + dx, at.1 + 2.0), (at.0 + dx + 0.6, at.1 - height)).pressure(0.9, 0.85).ramps(0.02, 0.02).shake(0.3), None);
    }
    let bar_y = at.1 - height * 0.78;
    let half = height * 0.22;
    for k in 0..2 {
        b.reload(wood, 0.9);
        let dy = k as f32 * 0.8 - 0.4;
        c.drag(&mut b, &Gesture::line((at.0 - half, bar_y + dy + 0.8), (at.0 + half, bar_y + dy)).pressure(0.85, 0.85).ramps(0.03, 0.03).shake(0.3).orient(Orient::Across), None);
    }
    c.dry();
    let light = pal.paint(hex("#c9a47d"), 0.4);
    let mut r = Held::new(Tool::round_sable(1.0), seed + 1);
    r.load(light, 0.35);
    c.drag(&mut r, &Gesture::line((at.0 - 1.7, bar_y + 3.0), (at.0 - 1.5, at.1 - 20.0)).pressure(0.5, 0.2).ramps(0.1, 0.5).shake(0.3), None);
    r.reload(light, 0.35);
    c.drag(&mut r, &Gesture::line((at.0 - 1.8, at.1 - height + 2.0), (at.0 - 1.7, bar_y - 3.0)).pressure(0.45, 0.3).ramps(0.1, 0.3).shake(0.3), None);
    r.reload(light, 0.3);
    c.drag(&mut r, &Gesture::line((at.0 - half + 1.0, bar_y - 1.8), (at.0 - 3.0, bar_y - 1.9)).pressure(0.4, 0.2).ramps(0.1, 0.5).shake(0.3), None);
}
