//! Pointed-tip study: small marks made with the point of a round sable or a
//! rigger, which should read at 1000px and at 3200px as the same marks
//! (same physical width and taper, no beads).
//!
//!   cargo paint study_tip                       1000px sheet
//!   cargo paint study_tip -- --full --crop 20,20,330,230     birds and rigging, 3200px
//!
//! Panels (units): birds (20..330, 20..230), rigging (340..660, 20..310),
//! signature (670..980, 60..300), grass (20..330, 360..650), twigs
//! (340..660, 340..650), spruce tips (670..980, 340..650).

use paint::{Gesture, Held, Mask, Paint, Rng, Style, Tool, Touch, hex};

fn main() {
    let o = paintings::run::Run::new("study_tip");
    let st = Style::friedrich();
    let mut rng = Rng::new(o.seed);
    let mut c = o.canvas(|| st.prepare(o.width, 1.5, o.seed));
    let h = c.height();

    // grounds for the marks: a pale evening sky above, a dull meadow below
    let sky = Mask::from_fn(c.frame(), |_, y| if y < 330.0 { 1.0 } else { 0.0 });
    let land = Mask::from_fn(c.frame(), |_, y| if y >= 330.0 { 1.0 } else { 0.0 });
    if o.stage("ground", &mut c, &mut rng) {
        c.work(&sky, &st.broad().color(|_, y| if y < 150.0 { hex("#b9c2c6") } else { hex("#e2d6b8") }).angle(|_, _| 0.0).coverage(2.5), 11);
        c.work(&land, &st.broad().color(|x, _| if x < 660.0 { hex("#8a8458") } else { hex("#a8a490") }).angle(|_, _| 0.0).coverage(2.5), 12);
        c.dry();
    }
    let dark = hex("#2a2620");
    let umber = hex("#3b2e22");
    let green = hex("#2f3a22");

    if o.stage("birds", &mut c, &mut rng) {
        // small birds: two wing flicks out from a body touch; a V is two
        // strokes pressed at the body and lifted off toward the wingtips
        let mut b = Held::new(Tool::round_sable(1.6), 1);
        for k in 0..9 {
            b.reload(Paint::body(dark), 0.8);
            let sz = 10.0 + 14.0 * rng.f();
            let (x, y) = (50.0 + (k % 3) as f32 * 95.0 + rng.range(-10.0, 10.0), 50.0 + (k / 3) as f32 * 60.0 + rng.range(-8.0, 8.0));
            let lift = rng.range(-0.4, 0.2);
            for side in [-1.0f32, 1.0] {
                let tip = (x + side * sz, y - sz * (0.35 + lift * side * 0.5));
                let mid = (x + side * sz * 0.45, y - sz * 0.35);
                c.drag(&mut b, &Gesture::new(vec![(x, y), mid, tip]).pressure(0.75, 0.0).ramps(0.1, 0.75).shake(0.6), None);
            }
            c.touch(&mut b, &Touch::at(x, y + 0.6).pressure(0.45), None);
        }
    }

    if o.stage("rigging", &mut c, &mut rng) {
        // a brig's masts and rigging: hairlines with a rigger, pressure light
        let mut hull = Held::new(Tool::round_sable(3.0), 2);
        hull.load(Paint::body(umber), 1.0);
        for k in 0..3 {
            let y = 280.0 + k as f32 * 5.0;
            c.drag(&mut hull, &Gesture::line((400.0 + k as f32 * 6.0, y), (600.0 - k as f32 * 8.0, y)).pressure(0.8, 0.8), None);
        }
        let mut rg = Held::new(Tool::rigger(0.5), 3);
        let masts = [(450.0f32, 60.0f32), (540.0, 80.0)];
        for &(mx, top) in &masts {
            rg.reload(Paint::body(umber), 1.0);
            c.drag(&mut rg, &Gesture::line((mx, 278.0), (mx + 3.0, top)).pressure(0.8, 0.5).ramps(0.02, 0.1).shake(0.4), None);
        }
        // stays and shrouds
        let lines = [
            ((450.0, 70.0), (380.0, 278.0)),
            ((450.0, 70.0), (410.0, 278.0)),
            ((450.0, 70.0), (543.0, 90.0)),
            ((543.0, 90.0), (620.0, 278.0)),
            ((543.0, 90.0), (590.0, 278.0)),
            ((452.0, 140.0), (543.0, 150.0)),
            ((452.0, 200.0), (420.0, 278.0)),
            ((542.0, 180.0), (570.0, 278.0)),
        ];
        for &(a, e) in &lines {
            rg.reload(Paint::body(dark), 0.6);
            c.drag(&mut rg, &Gesture::line(a, e).pressure(0.35, 0.3).ramps(0.05, 0.1).shake(0.3), None);
        }
        // yards: short strokes across the masts
        for &(mx, y, half) in &[(451.0f32, 110.0f32, 34.0f32), (451.0, 170.0, 44.0), (542.0, 125.0, 30.0), (541.5, 190.0, 40.0)] {
            rg.reload(Paint::body(umber), 1.0);
            c.drag(&mut rg, &Gesture::line((mx - half, y + 2.0), (mx + half, y - 1.0)).pressure(0.6, 0.6).ramps(0.1, 0.2).shake(0.4), None);
        }
    }

    if o.stage("signature", &mut c, &mut rng) {
        // a signature-like line: loops and hooks with pressure swelling on
        // the downstrokes and lifting to hairlines on the upstrokes
        let mut b = Held::new(Tool::round_sable(1.4), 4);
        b.load(Paint::body(dark), 1.0);
        let pts: Vec<(f32, f32)> = (0..90)
            .map(|i| {
                let t = i as f32 / 89.0;
                let x = 690.0 + t * 260.0 + (t * 40.0).sin() * 10.0;
                let y = 180.0 - (t * 20.0).cos() * 30.0 * (1.0 - 0.5 * t);
                (x, y)
            })
            .collect();
        let swell: Vec<f32> = (0..30).map(|i| if i % 3 == 1 { 1.0 } else { 0.35 }).collect();
        c.drag(&mut b, &Gesture::new(pts).pressure(0.8, 0.6).swell(swell).ramps(0.05, 0.2).shake(0.3), None);
        // an underline, flicked off
        b.reload(Paint::body(dark), 0.8);
        c.drag(&mut b, &Gesture::new(vec![(700.0, 250.0), (820.0, 256.0), (960.0, 244.0)]).pressure(0.8, 0.0).ramps(0.05, 0.8).shake(0.5), None);
    }

    if o.stage("grass", &mut c, &mut rng) {
        // blades flicked up from the root: pressed, then lifted off fast
        let mut b = Held::new(Tool::rigger(0.8), 5);
        let mut s = Held::new(Tool::round_sable(1.4), 6);
        for k in 0..70 {
            let hb = if k % 2 == 0 { &mut b } else { &mut s };
            hb.reload(Paint::body(if k % 3 == 0 { green } else { hex("#4a4a28") }), 0.7);
            let x = 40.0 + rng.f() * 270.0;
            let y = 620.0 + rng.range(-12.0, 12.0);
            let ht = 30.0 + 60.0 * rng.f();
            let lean = rng.range(-0.5, 0.5);
            let pts = vec![(x, y), (x + lean * ht * 0.3, y - ht * 0.5), (x + lean * ht, y - ht)];
            c.drag(hb, &Gesture::new(pts).pressure(0.8, 0.0).ramps(0.05, 0.85).shake(0.5), None);
        }
    }

    if o.stage("twigs", &mut c, &mut rng) {
        // bare twigs: each twig pressed where it leaves its limb and lifted
        // to its tip; side twigs from along it
        let mut b = Held::new(Tool::rigger(1.0), 7);
        let mut stack = vec![((500.0f32, 640.0f32), -1.57f32, 150.0f32, 0.9f32)];
        let mut n = 0;
        while let Some((p, a, len, pr)) = stack.pop() {
            n += 1;
            if n > 60 {
                break;
            }
            b.reload(Paint::body(umber), 0.8);
            let bend = rng.range(-0.3, 0.3);
            let pts: Vec<(f32, f32)> = (0..4)
                .map(|i| {
                    let t = i as f32 / 3.0;
                    let aa = a + bend * t;
                    (p.0 + aa.cos() * len * t, p.1 + aa.sin() * len * t)
                })
                .collect();
            let end = *pts.last().unwrap();
            c.drag(&mut b, &Gesture::new(pts.clone()).pressure(pr, 0.0).ramps(0.03, 0.8).shake(0.5), None);
            if len > 25.0 {
                for &f in &[0.45f32, 0.8] {
                    let q = (p.0 + (end.0 - p.0) * f, p.1 + (end.1 - p.1) * f);
                    let side = if rng.f() < 0.5 { -1.0 } else { 1.0 };
                    stack.push((q, a + side * rng.range(0.4, 0.8), len * rng.range(0.45, 0.6), pr * (1.0 - f * 0.6)));
                }
            }
        }
    }

    if o.stage("spruce", &mut c, &mut rng) {
        // spruce tips: a leader drawn up to a point, needle flicks hanging
        // off it to either side, shorter toward the top
        let mut b = Held::new(Tool::round_sable(1.2), 8);
        for &(x, base, ht) in &[(720.0f32, 640.0f32, 240.0f32), (820.0, 640.0, 280.0), (920.0, 640.0, 220.0)] {
            b.reload(Paint::body(green), 1.0);
            c.drag(&mut b, &Gesture::new(vec![(x, base), (x + 1.0, base - ht * 0.5), (x - 1.0, base - ht)]).pressure(0.9, 0.0).ramps(0.02, 0.6).shake(0.4), None);
            let n = 26;
            for i in 0..n {
                let t = i as f32 / n as f32;
                let y = base - ht * (0.08 + 0.9 * t);
                let reach = (1.0 - t) * 40.0 + 5.0;
                for side in [-1.0f32, 1.0] {
                    if i % 4 == 0 {
                        b.reload(Paint::body(green), 0.8);
                    }
                    let tip = (x + side * reach, y + reach * 0.35 + rng.range(-2.0, 2.0));
                    let mid = (x + side * reach * 0.5, y + reach * 0.05);
                    c.drag(&mut b, &Gesture::new(vec![(x, y), mid, tip]).pressure(0.7, 0.0).ramps(0.05, 0.8).shake(0.5), None);
                }
            }
        }
    }
    let _ = h;
    o.end(&mut c, &mut rng);
    c.relief(st.relief.0, st.relief.1);
    o.save(&mut c);
}
