//! "Moonrise over a Misty Valley": an original painting in the manner of
//! Caspar David Friedrich, not a copy of any work. No reference images; it is
//! composed from what is known of his vocabulary and his method:
//!
//! - two companions seen from behind (Rückenfigur) on a dark foreground ledge,
//!   a dead oak beside them, evergreens beyond: death and hope side by side;
//! - a valley filled with mist, far ridges in bands of cool color, a waxing
//!   crescent and the evening star in the afterglow;
//! - the order of work of the Friedrich style profile: reddish ground and
//!   brown priming, brown underpainting for values, sky laid in and fused wet,
//!   dried, then distance to foreground, figures last, thin layers throughout.
//!
//! Every mark is a simulated brush in wet paint or a Kubelka–Munk layer.

use paint::{Fbm, Gesture, Held, Mask, Mix, Oak, Orient, Paint, Rgb, Rng, Style, Tool, gradient, hex, smoothstep};
use paintings::run::Finish;
use paintings::{figures, trees};

fn main() {
    let o = paintings::run::Run::new("friedrich_moonrise_valley");
    let mut rng = Rng::new(o.seed);
    let seed = o.seed as u32;
    let st = Style::friedrich();
    // every color is mixed on the palette from Friedrich's own tube paints
    let tube_mix = |h: &str| st.palette.mix(hex(h)).color;

    // ---- ground and priming
    let mut c = o.canvas(|| st.prepare(o.width, 1.4, o.seed));
    let (w, h) = (c.width(), c.height());
    let f = c.frame();

    // ---- the lay of the land (units; y down)
    let n1 = Fbm::new(seed + 1, 5, 180.0);
    let n2 = Fbm::new(seed + 2, 5, 120.0);
    let n3 = Fbm::new(seed + 3, 4, 90.0);
    // far ridge: long, gentle, a saddle in the middle
    let ridge1 = |x: f32| h * 0.555 + 14.0 * n1.get(x, 0.0) - 18.0 * (-((x - 780.0) / 160.0).powi(2)).exp() + 10.0 * (-((x - 420.0) / 120.0).powi(2)).exp();
    // nearer ridge: lower, more broken, sloping down to the right
    let ridge2 = |x: f32| h * 0.615 + 12.0 * n2.get(x, 0.0) + 0.03 * (x - 500.0) - 14.0 * (-((x - 250.0) / 140.0).powi(2)).exp();
    // the mist lies in the valley between ridge2 and the ledge
    let mist_top = h * 0.64;
    // the foreground ledge: high on the left, breaking off to the right
    // breaking off in steps: short steep faces between ledges of rock
    let n4 = Fbm::new(seed + 4, 4, 30.0);
    let ledge = |x: f32| {
        let steps = 22.0 * smoothstep(392.0, 404.0, x) + 55.0 * smoothstep(438.0, 452.0, x) + 70.0 * smoothstep(505.0, 522.0, x) + 90.0 * smoothstep(575.0, 600.0, x);
        h * 0.745 + 6.0 * n3.get(x, 0.0) + 2.5 * n4.get(x, 0.0) + steps - 10.0 * (-((x - 130.0) / 90.0).powi(2)).exp()
    };
    // the masks ask for these at every pixel: evaluate once per column
    let (ridge1, ridge2, ledge) = (f.per_column(ridge1), f.per_column(ridge2), f.per_column(ledge));
    let soft = |d: f32, e: f32| smoothstep(-e, e, d);
    let sky = Mask::from_fn(f, |x, y| 1.0 - soft(y - ridge1(x), 0.6));
    let far = Mask::from_fn(f, |x, y| soft(y - ridge1(x), 0.6) * (1.0 - soft(y - ridge2(x), 0.6)));
    let near = Mask::from_fn(f, |x, y| soft(y - ridge2(x), 0.6) * (1.0 - soft(y - ledge(x), 0.8)));
    let rock = Mask::from_fn(f, |x, y| soft(y - ledge(x), 0.8));

    if o.stage("ground", &mut c, &mut rng) {
        // ---- underpainting: thin brown for the values, dark below
        // a thin brown underpainting brushed over everything for the values,
        // loaded heavier where the land will be dark, fused, left to dry
        let whole = Mask::from_fn(f, |_, _| 1.0);
        let under = |x: f32, y: f32| 0.25 + 2.2 * smoothstep(ledge(x) - 20.0, ledge(x) + 40.0, y) + 0.6 * smoothstep(h * 0.55, h * 0.75, y);
        c.work(&whole, &st.glaze(0.9).color(|_, _| hex("#6a4c34")).angle(|_, _| 0.0).load_at(under), o.seed * 100 + 90);
        if let Some(b) = st.blend() {
            c.work(&whole, &b, o.seed * 100 + 91);
        }
        c.dry();
    }

    // ---- sky: twilight in bands, deep blue-grey above, the afterglow low
    let sky_stops: [(f32, Rgb); 7] = [
        (0.00, hex("#34405a")),
        (0.14, hex("#4d5a72")),
        (0.30, hex("#7d8190")),
        (0.42, hex("#b3a79c")),
        (0.50, hex("#d8bf98")),
        (0.55, hex("#e9d2a4")),
        (0.60, hex("#efdcae")),
    ];
    let drift = Fbm::new(seed + 10, 4, 500.0);
    let sky_color = |x: f32, y: f32| gradient(&sky_stops, (y / h + 0.012 * drift.get(x * 0.3, y)).clamp(0.0, 0.6), Mix::Pigment);
    let sky_angle = |x: f32, y: f32| 0.015 * drift.get(x, y * 3.0);
    if o.stage("sky lay", &mut c, &mut rng) {
        c.work(&sky, &st.broad().color(sky_color).angle(sky_angle).medium(0.3).load(0.64).pressure(0.7, 0.9).coverage(4.0), o.seed * 100 + 1);
        c.work(&sky, &st.broad().color(sky_color).angle(sky_angle).coverage(2.0).length(60.0, 150.0).medium(0.55), o.seed * 100 + 2);
    }
    // the moon's place: the sky is prepared for it, the moon painted later
    let (mx, my, mr) = (w * 0.7, h * 0.3, 9.5);
    if o.stage("sky", &mut c, &mut rng) {
        // thin cloud streaks, laid wet into the sky with a filbert and lean paint:
        // dark undersides high up, warm lit ones low in the afterglow
        let mut fil = Held::new(Tool { width: 7.0, ..Tool::filbert(7.0) }, 31);
        for i in 0..16 {
            let y = h * rng.range(0.1, 0.5);
            let x0 = rng.range(-100.0, w * 0.8);
            let len = rng.range(120.0, 380.0);
            let v = y / h;
            let col = if v < 0.3 { gradient(&[(0.0, hex("#2c3448")), (1.0, hex("#5d5f6c"))], v / 0.3, Mix::Pigment) } else { gradient(&[(0.0, hex("#8f7f7a")), (1.0, hex("#e2c69c"))], (v - 0.3) / 0.2, Mix::Pigment) };
            fil.reload(Paint { color: st.palette.mix(col).color, hiding: 0.45, stiff: 0.5 }, 0.8 * 0.5);
            let sag = rng.range(-3.0, 3.0);
            let pts = vec![(x0, y), (x0 + len * 0.35, y + sag), (x0 + len * 0.7, y + sag * 0.5 - 1.5), (x0 + len, y - 1.0)];
            c.drag(&mut fil, &Gesture::new(pts).pressure(rng.range(0.35, 0.6), rng.range(0.2, 0.4)).ramps(0.2, 0.35).orient(Orient::Across), Some(&sky));
            let _ = i;
        }
        // where the moon will be: while the sky is wet, a few pale touches around
        // it, curling with the ring, which the badger then melts into a glow
        let mut glow = Held::new(Tool::filbert(7.0), 33);
        for k in 0..10 {
            let a0 = k as f32 * 0.63 + rng.range(-0.2, 0.2);
            let r = mr * rng.range(0.6, 2.6);
            let pts: Vec<(f32, f32)> = (0..5).map(|j| {
                let a = a0 + j as f32 * 0.18;
                (mx + r * a.cos(), my + r * a.sin())
            }).collect();
            glow.reload(Paint { color: tube_mix("#dcdad2"), hiding: 0.7, stiff: 0.4 }, 0.7 * 0.4);
            c.drag(&mut glow, &Gesture::new(pts).pressure(0.5, 0.4).ramps(0.3, 0.4).orient(Orient::Across), Some(&sky));
        }
        if let Some(b) = st.blend() {
            let b = b.angle(sky_angle);
            for k in 0..st.blend_passes {
                c.work(&sky, &b, o.seed * 100 + 3 + k as u64);
            }
        }
        c.dry();
    }

    if o.stage("moon", &mut c, &mut rng) {
        // ---- moon and evening star: after the sky is dry
        // the crescent in a few touches of thick pale paint along the lit limb
        // (toward the set sun, below right): the limb, an inner stroke where the
        // crescent is widest, then the brightest touch
        let lit = 0.45f32;
        let arc = |rad: f32, span: f32, n: usize| -> Vec<(f32, f32)> {
            (0..=n).map(|k| {
                let a = lit - span + 2.0 * span * k as f32 / n as f32;
                (mx + rad * a.cos(), my + rad * a.sin())
            }).collect()
        };
        let mut cres = Held::new(Tool::round_sable(mr * 0.24), 42);
        cres.load(Paint { color: tube_mix("#e8ddb6"), hiding: 0.95, stiff: 0.8 }, 1.0 * 0.8);
        c.drag(&mut cres, &Gesture::new(arc(mr * 0.88, 1.45, 16)).pressure(0.85, 0.85).ramps(0.5, 0.5).orient(Orient::Across), None);
        cres.reload(Paint { color: tube_mix("#eee5c2"), hiding: 0.95, stiff: 0.8 }, 1.0 * 0.8);
        c.drag(&mut cres, &Gesture::new(arc(mr * 0.74, 0.8, 10)).pressure(0.6, 0.6).ramps(0.5, 0.5).orient(Orient::Across), None);
        let mut hi = Held::new(Tool::round_sable(mr * 0.16), 44);
        hi.load(Paint { color: tube_mix("#f8f2d8"), hiding: 0.97, stiff: 0.9 }, 1.0 * 0.9);
        c.drag(&mut hi, &Gesture::new(arc(mr * 0.76, 0.45, 6)).pressure(0.8, 0.8).ramps(0.4, 0.4).orient(Orient::Across), None);
        let mut star = Held::new(Tool::round_sable(1.6), 43);
        star.load(Paint { color: tube_mix("#f6f0d8"), hiding: 0.95, stiff: 1.0 }, 1.0 * 1.2);
        let (sx, sy) = (w * 0.77, h * 0.2);
        c.drag(&mut star, &Gesture::new(vec![(sx, sy - 0.4), (sx, sy + 0.4)]).pressure(1.0, 1.0).ramps(0.0, 0.0), None);
        c.dry();
    }

    if o.stage("ridges", &mut c, &mut rng) {
        // ---- the far ridge: cool violet, lighter at the top where it meets glow
        let ridge_color = |_x: f32, y: f32| gradient(&[(0.0, hex("#6e6f86")), (1.0, hex("#5a5c70"))], ((y - h * 0.53) / (h * 0.09)).clamp(0.0, 1.0), Mix::Pigment);
        c.work(&far, &st.body().color(ridge_color).angle(|_, _| 0.0).angle_jitter(0.05).length(30.0, 90.0).coverage(6.0).threshold(0.05).clip(true), o.seed * 100 + 10);
        // the nearer ridge, darker and bluer, with a forest edge
        let near_color = |_x: f32, y: f32| gradient(&[(0.0, hex("#44495c")), (1.0, hex("#3a3e4c"))], ((y - h * 0.6) / (h * 0.12)).clamp(0.0, 1.0), Mix::Pigment);
        c.work(&near, &st.body().color(near_color).angle(|_, _| 0.0).angle_jitter(0.06).length(25.0, 80.0).coverage(6.0).threshold(0.05).clip(true), o.seed * 100 + 11);
        c.dry();
        // spruce tops along the ridge line, tiny, softened by distance
        for i in 0..70 {
            let x = rng.range(0.0, w);
            let _ = i;
            let hgt = rng.range(5.0, 11.0);
            trees::spruce(&mut c, (x, ridge2(x) + hgt * 0.35), hgt, tube_mix("#353846"), rng.next_u64());
        }
    }

    if o.stage("mist", &mut c, &mut rng) {
        // ---- the valley: first the mist itself in body color, glowing with the
        // afterglow near the far side, fused with the badger and let dry
        // the top of the fog is uneven: it climbs the foot of the near ridge in
        // soft tongues, so the ridge dissolves into it rather than sitting on it
        let mt_n = Fbm::new(seed + 62, 4, 200.0);
        let mt = f.per_column(|x: f32| mist_top - 4.0 + 16.0 * mt_n.get(x, 0.0));
        let mist_c = |x: f32, y: f32| {
            let t = ((y - mist_top) / (h * 0.32)).clamp(0.0, 1.0);
            let base = gradient(&[(0.0, hex("#bdb4aa")), (0.35, hex("#a9a3a0")), (1.0, hex("#72706f"))], t, Mix::Pigment);
            let k = 1.0 + 0.06 * mt_n.get(x * 0.5, y * 2.5);
            [base[0] * k, base[1] * k, base[2] * k]
        };
        let valley = Mask::from_fn(f, |x, y| (1.0 - soft(y - ledge(x), 0.8)) * smoothstep(mt(x) - 40.0, mt(x) + 16.0, y));
        c.work(&valley, &st.broad().color(mist_c).angle(|_, _| 0.0).angle_jitter(0.02).length(80.0, 200.0).medium(0.4).load(0.3).pressure(0.6, 0.8).coverage(3.0).clip(true), o.seed * 100 + 20);
        if let Some(b) = st.blend() {
            c.work(&valley, &b.angle(|_, _| 0.0), o.seed * 100 + 21);
        }
        c.dry();
        // wisps drifting across the foot of the near ridge: long, lean, barely
        // pressed strokes of the mist color
        let mut wisp = Held::new(Tool::filbert(9.0), 71);
        for _ in 0..14 {
            let x0 = rng.range(-80.0, w * 0.9);
            let y = mt(x0) - rng.range(4.0, 26.0);
            let len = rng.range(90.0, 260.0);
            wisp.reload(Paint { color: tube_mix("#a9a29f"), hiding: 0.2, stiff: 0.25 }, 0.5 * 0.25);
            let pts = vec![(x0, y), (x0 + len * 0.5, y + rng.range(-2.0, 2.0)), (x0 + len, y + rng.range(-2.0, 3.0))];
            c.drag(&mut wisp, &Gesture::new(pts).pressure(rng.range(0.25, 0.4), 0.15).ramps(0.4, 0.5).orient(Orient::Across), None);
        }
        // fuse the wisps into the air with the badger while they are wet
        if let Some(b) = st.blend() {
            let band = Mask::from_fn(f, |x, y| smoothstep(mt(x) - 50.0, mt(x) - 32.0, y) * (1.0 - smoothstep(mt(x) + 2.0, mt(x) + 14.0, y)));
            c.work(&band, &b.angle(|_, _| 0.0), o.seed * 100 + 22);
        }
        c.dry();
        // spruces standing in the valley in loose groups, the far ones paler
        let mut spots = Vec::new();
        for _ in 0..9 {
            let gx = rng.range(380.0, w + 30.0);
            let gy = mist_top + rng.range(10.0, 90.0);
            for _ in 0..rng.range(3.0, 9.0) as u32 {
                let x = gx + rng.normal() * 22.0;
                let y = gy + rng.normal() * 6.0;
                if y < ledge(x) - 6.0 {
                    spots.push((x, y));
                }
            }
        }
        spots.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        for (x, y) in &spots {
            let depth = ((y - mist_top) / 90.0).clamp(0.0, 1.0);
            let hgt = 26.0 + 40.0 * depth + rng.range(-6.0, 10.0);
            let col = gradient(&[(0.0, hex("#565a62")), (1.0, hex("#23272a"))], depth, Mix::Pigment);
            trees::spruce(&mut c, (*x, *y), hgt, col, rng.next_u64());
        }
        // a church in the mist, far right: tower and spire, two sable strokes
        let (cx, cy) = (w * 0.86, mist_top + 10.0);
        let mut sp = Held::new(Tool::round_sable(3.0), 51);
        sp.load(Paint { color: tube_mix("#62636a"), hiding: 0.9, stiff: 0.6 }, 1.0 * 0.6);
        c.drag(&mut sp, &Gesture::new(vec![(cx, cy), (cx, cy - 18.0)]).pressure(1.0, 1.0).ramps(0.0, 0.0), None);
        let mut sp2 = Held::new(Tool::round_sable(2.6), 52);
        sp2.load(Paint { color: tube_mix("#62636a"), hiding: 0.9, stiff: 0.6 }, 1.0 * 0.6);
        c.drag(&mut sp2, &Gesture::new(vec![(cx, cy - 17.0), (cx, cy - 31.0)]).pressure(1.0, 0.1).ramps(0.0, 0.9), None);
        c.dry();
        // the fog bank lies in level layers: a pale semi-opaque KM layer, thin at
        // the tree tops, thick at their feet, drifting in horizontal bands
        let mist_n = Fbm::new(seed + 60, 5, 160.0);
        let bands = Fbm::new(seed + 61, 3, 60.0);
        // scumbled on in level strokes, heavier toward the valley floor, then
        // fused with the badger so the tree feet dissolve into it
        let depth = |x: f32, y: f32| {
            let d = (y - mist_top + 10.0) / 60.0;
            let layered = d + 0.25 * bands.get(x * 0.08, y * 1.5);
            (1.6 * smoothstep(0.0, 1.4, layered).powf(1.5)) * (0.85 + 0.3 * mist_n.get(x * 0.5, y * 2.0))
        };
        c.work(&valley, &st.glaze(0.6).color(|_, _| hex("#b4aea8")).angle(|_, _| 0.0).coverage(3.5).load_at(depth), o.seed * 100 + 23);
        if let Some(b) = st.blend() {
            c.work(&valley, &b.angle(|_, _| 0.0), o.seed * 100 + 24);
        }
        c.dry();
    }

    if o.stage("ledge", &mut c, &mut rng) {
        // ---- the ledge: dark earth and rock, strokes following the slope
        let slope = |x: f32| ((ledge(x + 4.0) - ledge(x - 4.0)) / 8.0).atan();
        let rn = Fbm::new(seed + 70, 5, 70.0);
        let rock_color = |x: f32, y: f32| {
            let t = ((y - ledge(x)) / 90.0).clamp(0.0, 1.0);
            let base = gradient(&[(0.0, hex("#4a4034")), (0.25, hex("#2f2922")), (1.0, hex("#1c1814"))], t, Mix::Pigment);
            let k = 1.0 + 0.18 * rn.get(x, y);
            [base[0] * k, base[1] * k, base[2] * k]
        };
        c.work(&rock, &st.body().color(rock_color).angle(|x, _| slope(x) * 0.8).angle_jitter(0.35).length(12.0, 40.0).coverage(4.0).threshold(0.2).clip(true), o.seed * 100 + 30);
        // the lit top edge and rock planes: dry-brush with a hog, barely pressing
        let edge_c = |x: f32, y: f32| {
            let t = ((y - ledge(x)) / 25.0).clamp(0.0, 1.0);
            gradient(&[(0.0, hex("#5c5144")), (1.0, hex("#3d342b"))], t, Mix::Pigment)
        };
        let lip = Mask::from_fn(f, |x, y| soft(y - ledge(x), 0.8) * (1.0 - soft(y - ledge(x) - 30.0, 10.0)));
        c.work(&lip, &st.body().color(edge_c).angle(|x, _| slope(x)).angle_jitter(0.08).length(25.0, 70.0).coverage(0.7).pressure(0.3, 0.45).dips(3, 0.5 * 0.7, 0.9).threshold(0.3).clip(true), o.seed * 100 + 31);
        c.dry();
        // grass on the ledge, flicked up with the rigger, dark against the mist
        let mut rig = Held::new(st.line_tool(0.5), 61);
        for _ in 0..40 {
            let gx = rng.range(0.0, 540.0);
            let gy = ledge(gx) + rng.range(0.5, 3.0);
            rig.reload(Paint { color: tube_mix("#2a241d"), hiding: 0.85, stiff: 0.6 }, 1.0 * 0.6);
            for _ in 0..rng.range(2.0, 7.0) as u32 {
                let x = gx + rng.range(-4.0, 4.0);
                let hg = rng.range(3.0, 9.0);
                let lean = rng.range(-3.0, 3.0);
                c.drag(&mut rig, &Gesture::new(vec![(x, gy), (x + lean * 0.4, gy - hg * 0.55), (x + lean, gy - hg)]).pressure(0.8, 0.1).ramps(0.02, 0.7).orient(Orient::Along), None);
            }
        }
    }

    if o.stage("oak", &mut c, &mut rng) {
        // ---- the dead oak on the ledge, reaching out over the valley
        let mut oak = Oak::new((115.0, ledge(115.0) + 3.0), h * 0.62, o.seed * 7 + 3);
        oak.gnarl = 0.85;
        oak.broken = 0.3;
        oak.lean = 0.12;
        oak.depth = 5;
        oak.roots = 3;
        oak.paint(&mut c, tube_mix("#1f1a16"), Some(tube_mix("#3a322b")), o.seed * 100 + 40);
        c.dry();
    }

    if o.stage("figures", &mut c, &mut rng) {
        // ---- the two companions, near the edge, looking toward the moon
        let fh = h * 0.12;
        let bx = 330.0;
        let sh = figures::man_in_cape(&mut c, (bx + 26.0, ledge(bx + 26.0) + 1.5), fh, tube_mix("#1d201b"), tube_mix("#110f0d"), tube_mix("#221b15"), Some(tube_mix("#b9a582")), o.seed * 100 + 50);
        figures::youth_in_frock(&mut c, (bx, ledge(bx) - 1.0), fh * 0.98, tube_mix("#1e221c"), tube_mix("#110f0d"), tube_mix("#2a2118"), tube_mix("#b8b09c"), Some(sh), Some(tube_mix("#b9a582")), o.seed * 100 + 51);
    }

    // ---- finish: clear aged varnish, cracks, the surface in raking light
    let ground_um = st.ground.iter().map(|g| g.um).sum();
    let cracks = paint::Cracks { island_mm: 3.5, ground_um, dirt: 0.4, ..paint::Cracks::aged(0) };
    o.finish(&mut c, &Finish { cracks: Some(cracks), ..Finish::aged(st.relief) });
}
