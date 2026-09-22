//! Study sheet for the oak and Rückenfigur generators.

use paint::{Canvas, Coat, Figure, Hat, Mix, Oak, Side, gradient, hex};

fn main() {
    let t0 = std::time::Instant::now();
    let o = paint::cli::opts("study_generators");
    let mut c = Canvas::new(o.width, 1.6, hex("#d9d2bf"));
    let h = c.height();
    c.paint(None, 1.0, Mix::Pigment, |_, y| {
        gradient(&[(0.0, hex("#9aa3a6")), (1.0, hex("#e2d6b4"))], y / h, Mix::Pigment)
    });

    // top row: oaks with different settings
    let ground = h * 0.5;
    for (i, (gnarl, broken, lean, depth)) in
        [(0.4, 0.05, 0.0, 6), (0.7, 0.12, 0.1, 6), (0.9, 0.3, -0.25, 6), (0.6, 0.1, 0.0, 7), (1.0, 0.25, 0.35, 5), (0.5, 0.0, -0.05, 7)]
            .iter()
            .enumerate()
    {
        let x = 90.0 + i as f32 * 165.0;
        let mut oak = Oak::new((x, ground), 250.0, 11 + i as u64 * 7);
        oak.gnarl = *gnarl;
        oak.broken = *broken;
        oak.lean = *lean;
        oak.depth = *depth;
        oak.roots = if i == 4 { 4 } else { 0 };
        oak.paint(&mut c, hex("#2a2420"), Some(hex("#8a7f6a")), 100 + i as u64);
    }

    // bottom row: figures
    let gy = h * 0.95;
    let fh = 120.0;
    let mut a = Figure::new((120.0, gy), fh);
    a.hat = Hat::Beret;
    a.stick = Some(Side::Right);
    a.paint(&mut c, hex("#23262a"), Some((hex("#8f8a78"), 1.0)), 1);

    let mut b = Figure::new((300.0, gy), fh * 0.97);
    b.coat = Coat::Cloak;
    b.hat = Hat::Cap;
    b.paint(&mut c, hex("#3a2f28"), Some((hex("#8f8a78"), 1.0)), 2);

    // two men: the left rests a hand on the right one's shoulder
    let mut r = Figure::new((560.0, gy), fh);
    r.hat = Hat::Beret;
    r.lean = 0.03;
    let mut l = Figure::new((500.0, gy), fh * 1.02);
    l.coat = Coat::Long;
    l.hat = Hat::None;
    l.right_hand = Some(r.shoulder(Side::Left));
    l.lean = 0.05;
    l.paint(&mut c, hex("#2b2d29"), Some((hex("#8f8a78"), 1.0)), 3);
    r.paint(&mut c, hex("#262320"), Some((hex("#8f8a78"), 1.0)), 4);

    let mut w = Figure::new((760.0, gy), fh);
    w.hat = Hat::TopHat;
    w.coat = Coat::Short;
    w.stance = 0.18;
    w.left_hand = Some(w.at(-0.1, 0.62)); // hand on hip
    w.paint(&mut c, hex("#1f2326"), Some((hex("#8f8a78"), 1.0)), 5);

    let mut m = Figure::new((900.0, gy), fh);
    m.coat = Coat::Habit;
    m.right_hand = Some(m.at(0.02, 0.86)); // hand to chin
    m.paint(&mut c, hex("#1a1817"), None, 6);

    c.relief(0.5, 0.0);
    c.save(&o.out).unwrap();
    paint::cli::done(&o, t0);
}
