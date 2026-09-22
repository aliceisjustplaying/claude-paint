//! Foreground things, written as brush work: boulders, moss, grass, stones.
//! Friedrich's foregrounds are made of particular things painted with care
//! (rocks with lit planes and cracks, tufts, moss), not of texture.

use paint::{Canvas, Gesture, Held, Mask, Orient, Paint, Rgb, Rng, Shape, Tool};

fn body(color: Rgb) -> Paint {
    Paint { color, hiding: 0.95, body: 0.8 }
}

/// A boulder: blocked in with a filbert inside its outline, the top plane
/// dry-brushed lighter where it faces the sky, a crack or two, and a dark
/// line where it sits on the ground. `at` = center of the base.
#[allow(clippy::too_many_arguments)]
pub fn boulder(c: &mut Canvas, at: (f32, f32), w: f32, h: f32, dark: Rgb, top: Rgb, crack: Rgb, seed: u64) {
    let mut rng = Rng::new(seed);
    // outline: a rounded, faceted lump
    let n = 9;
    let pts: Vec<(f32, f32)> = (0..n)
        .map(|i| {
            let a = std::f32::consts::PI * (1.0 + i as f32 / (n - 1) as f32); // left → over the top → right
            let r = rng.range(0.8, 1.1);
            (at.0 + a.cos() * w * 0.5 * r, at.1 + a.sin() * h * r)
        })
        .chain([(at.0 + w * 0.5, at.1 + h * 0.08), (at.0 - w * 0.5, at.1 + h * 0.08)])
        .collect();
    let mask = Mask::from_shape(c.f, Shape::new().smooth_poly(&pts));
    // block-in: strokes across the rock, following its roundness
    let bw = (h * 0.35).clamp(1.2, 8.0);
    let mut b = Held::new(Tool::filbert(bw), rng.next_u64());
    let rows = ((h / (bw * 0.6)).ceil() as usize).max(2);
    for r in 0..=rows {
        let y = at.1 - h * (r as f32 / rows as f32) * 1.05;
        b.reload(body(dark), 1.0);
        let sag = rng.range(-0.3, 0.3) * bw;
        c.drag(&mut b, &Gesture::new(vec![(at.0 - w * 0.6, y + sag), (at.0, y - h * 0.08), (at.0 + w * 0.6, y - sag)]).pressure(0.85, 0.85).orient(Orient::Across), Some(&mask));
    }
    c.dry();
    // the top plane catches the sky: a lean light paint dragged lightly so
    // it breaks on the rough surface
    let mut hog = Held::new(Tool { ragged: 0.6, ..Tool::hog_flat((h * 0.3).clamp(1.0, 6.0)) }, rng.next_u64());
    for k in 0..3 {
        hog.reload(Paint { color: top, hiding: 0.7, body: 0.5 }, 0.7);
        let y = at.1 - h * (0.75 + 0.1 * k as f32 + rng.range(-0.05, 0.05));
        let x0 = at.0 - w * rng.range(0.25, 0.4);
        let x1 = at.0 + w * rng.range(0.15, 0.35);
        c.drag(&mut hog, &Gesture::new(vec![(x0, y + h * 0.05), ((x0 + x1) * 0.5, y - h * 0.04), (x1, y + h * 0.06)]).pressure(0.42, 0.3).orient(Orient::Across), Some(&mask));
    }
    // cracks: a rigger line or two down the face
    let mut rig = Held::new(Tool { splay: 0.15, stiffness: 0.3, ..Tool::rigger((h * 0.04).clamp(0.3, 1.0)) }, rng.next_u64());
    for _ in 0..rng.range(1.0, 3.0) as u32 {
        rig.reload(body(crack), 1.0);
        let x = at.0 + w * rng.range(-0.3, 0.3);
        let y0 = at.1 - h * rng.range(0.6, 0.95);
        let pts = vec![(x, y0), (x + w * rng.range(-0.08, 0.08), y0 + h * 0.3), (x + w * rng.range(-0.12, 0.12), at.1 - h * rng.range(0.0, 0.3))];
        c.drag(&mut rig, &Gesture::new(pts).pressure(0.8, 0.4).ramps(0.1, 0.5).orient(Orient::Across), Some(&mask));
    }
    // where it sits: a dark line along the foot
    let mut sab = Held::new(Tool::round_sable((h * 0.12).clamp(0.6, 3.0)), rng.next_u64());
    sab.load(body(crack), 1.0);
    c.drag(&mut sab, &Gesture::new(vec![(at.0 - w * 0.55, at.1 + h * 0.02), (at.0, at.1 + h * 0.06), (at.0 + w * 0.55, at.1 + h * 0.02)]).pressure(0.8, 0.6).ramps(0.3, 0.3).orient(Orient::Across), None);
}

/// Moss: a cluster of small stippled dabs of a round sable.
pub fn moss(c: &mut Canvas, at: (f32, f32), spread: (f32, f32), colors: &[Rgb], dab: f32, n: usize, seed: u64) {
    let mut rng = Rng::new(seed);
    let mut b = Held::new(Tool::round_sable(dab), rng.next_u64());
    for i in 0..n {
        if i % 6 == 0 {
            let col = colors[rng.range(0.0, colors.len() as f32 - 1e-3) as usize];
            b.reload(Paint { color: col, hiding: 0.85, body: 0.7 }, 0.8);
        }
        let x = at.0 + rng.normal() * spread.0;
        let y = at.1 + rng.normal() * spread.1;
        let a = rng.range(0.0, std::f32::consts::TAU);
        let l = dab * rng.range(0.2, 0.6);
        c.drag(&mut b, &Gesture::new(vec![(x, y), (x + a.cos() * l, y + a.sin() * l)]).pressure(rng.range(0.5, 0.9), 0.4).ramps(0.0, 0.5), None);
    }
}

/// A grass tuft: blades flicked upward from a base with a rigger.
pub fn tuft(c: &mut Canvas, rig: &mut Held, at: (f32, f32), height: f32, blades: usize, color: Paint, rng: &mut Rng) {
    rig.reload(color, 1.0);
    for _ in 0..blades {
        let x = at.0 + rng.normal() * height * 0.25;
        let hg = height * rng.range(0.5, 1.1);
        let lean = rng.normal() * height * 0.35;
        c.drag(rig, &Gesture::new(vec![(x, at.1), (x + lean * 0.35, at.1 - hg * 0.55), (x + lean, at.1 - hg)]).pressure(0.8, 0.1).ramps(0.02, 0.7).orient(Orient::Along), None);
    }
}
