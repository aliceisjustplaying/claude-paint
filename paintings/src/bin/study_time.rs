//! Time and drying. The painting has a clock (`c.wait(minutes)`), and what a
//! brush does depends on how far the paint under it has dried.
//!
//! Top row: the same wet-into-wet blend (a dark blue brought up into a warm
//! light field, then worked across the join with a clean brush), started
//! at 0 min, 30 min, 3 h and 24 h after the light field was laid.
//! Open paint blends; at 3 h the lead-white field is tacky, so the blue
//! catches and drags in broken patches; at 24 h it is touch-dry and the
//! blue sits on top with a crisp join.
//!
//! Bottom row: a light scumble dragged over an umber underlayer that is
//! tacky (3 h) and touch-dry (24 h); a pale glaze with a Gaussian falloff
//! (1/e radius 55 units) over a dark bone-black field.

use paint::drying::drier;
use paint::{Canvas, Gesture, Held, Mask, Orient, Paint, Pigment, Stage, Style, Tool, hex};

const TOP: (f32, f32) = (30.0, 330.0);
const BOT: (f32, f32) = (370.0, 640.0);

/// Panel `k` of `n` across the canvas: (x0, x1) in units.
fn panel(k: usize, n: usize) -> (f32, f32) {
    let w = 1000.0 / n as f32;
    (k as f32 * w + 20.0, (k + 1) as f32 * w - 20.0)
}

/// Lay a field, about one lean coat, with a hog flat in level, overlapping
/// strokes.
fn field(c: &mut Canvas, p: Paint, x: (f32, f32), y: (f32, f32), seed: u64) {
    let mut h = Held::new(Tool::hog_flat(14.0), seed);
    let mut yy = y.0 + 5.0;
    let mut k = 0;
    while yy < y.1 - 4.0 {
        if k % 2 == 0 {
            h.reload(p, 0.45);
        }
        c.drag(&mut h, &Gesture::new(vec![(x.0 + 2.0, yy), (x.1 - 2.0, yy + 1.5)]).pressure(0.85, 0.8).orient(Orient::Across), None);
        yy += 6.0;
        k += 1;
    }
}

/// The blend: dark blue brought up to the join, then a clean flat worked
/// up and down across it, then a light pass along it.
fn blend(c: &mut Canvas, x: (f32, f32), join: f32, seed: u64) {
    let blue = Paint::body(hex("#2c3a58")).with_drying(drier::SMALT);
    field(c, blue, x, (join, TOP.1), seed);
    let mut h = Held::new(Tool::hog_flat(10.0), seed + 1);
    let mut xx = x.0 + 6.0;
    let mut up = true;
    while xx < x.1 - 5.0 {
        let (a, b) = if up { (join + 28.0, join - 28.0) } else { (join - 28.0, join + 28.0) };
        c.drag(&mut h, &Gesture::new(vec![(xx, a), (xx + 2.0, b)]).pressure(0.6, 0.5).orient(Orient::Across), None);
        h.wipe(0.3);
        xx += 6.0;
        up = !up;
    }
    let mut soft = Held::new(Tool::badger(22.0), seed + 2);
    for dy in [-8.0, 0.0, 8.0] {
        c.drag(&mut soft, &Gesture::new(vec![(x.0 + 4.0, join + dy), (x.1 - 4.0, join + dy)]).pressure(0.35, 0.3), None);
    }
}

/// A scumble: a hog flat with a little stiff light paint, dragged lightly.
fn scumble(c: &mut Canvas, x: (f32, f32), y: (f32, f32), seed: u64) {
    let light = Paint::scumble(hex("#e4dcc4")).with_stiff(0.9).with_drying(drier::LEAD_WHITE);
    let mut h = Held::new(Tool::hog_flat(18.0), seed);
    let mut yy = y.0 + 25.0;
    while yy < y.1 - 20.0 {
        h.reload(light, 0.35);
        c.drag(&mut h, &Gesture::new(vec![(x.0 + 10.0, yy), (x.1 - 10.0, yy - 6.0)]).pressure(0.45, 0.35).orient(Orient::Across), None);
        yy += 30.0;
    }
}

fn say(c: &Canvas, t0: f64, what: &str, at: &[(&str, (f32, f32))]) {
    let s: Vec<String> = at.iter().map(|(n, p)| format!("{n} {:?}", c.drying_at(p.0, p.1))).collect();
    println!("{:>7.0} min  {what}: {}", c.clock() - t0, s.join(", "));
}

fn main() {
    let o = paintings::run::Run::new("study_time");
    let st = Style::friedrich();
    let mut c = st.prepare(o.width, 1000.0 / 670.0, o.seed);
    // (the ground was dried: the clock starts weeks in)
    let t0 = c.clock();
    let join = 0.5 * (TOP.0 + TOP.1);
    let warm = Paint::body(hex("#e2cfa6")).with_drying(drier::LEAD_WHITE);
    let umber = Paint::body(hex("#4a3a2a")).with_drying(drier::UMBER);
    let slate = Paint::body(hex("#343a44")).with_drying(drier::BONE_BLACK);
    // t = 0: the light fields of the blends, the scumbles' underlayer, the
    // glaze's dark field
    for k in 0..4 {
        field(&mut c, warm, panel(k, 4), (TOP.0, join + 6.0), 10 + k as u64);
    }
    for k in 0..2 {
        field(&mut c, umber, panel(k, 3), BOT, 20 + k as u64);
    }
    field(&mut c, slate, panel(2, 3), BOT, 30);
    let top = |k: usize| (0.5 * (panel(k, 4).0 + panel(k, 4).1), join - 40.0);
    let bot = |k: usize| (0.5 * (panel(k, 3).0 + panel(k, 3).1), 0.5 * (BOT.0 + BOT.1));
    say(&c, t0, "laid", &[("light", top(0)), ("umber", bot(0)), ("slate", bot(2))]);
    blend(&mut c, panel(0, 4), join, 100);
    c.wait(30.0);
    say(&c, t0, "30 min", &[("light", top(1))]);
    assert_eq!(c.drying_at(top(1).0, top(1).1), Stage::Open, "the light field is open at 30 min");
    blend(&mut c, panel(1, 4), join, 200);
    c.wait(150.0);
    say(&c, t0, "3 h", &[("light", top(2)), ("umber", bot(0)), ("slate", bot(2))]);
    // (the same at any width: a film dries by its thickness over a few
    // millimeters, `drying::FILM_MM`, not per pixel)
    assert_eq!(c.drying_at(top(2).0, top(2).1), Stage::Tacky, "the light field is tacky at 3 h");
    assert_eq!(c.drying_at(bot(0).0, bot(0).1), Stage::Tacky, "the umber is tacky at 3 h");
    assert_eq!(c.drying_at(bot(2).0, bot(2).1), Stage::Open, "the slate (bone black) is still open at 3 h");
    blend(&mut c, panel(2, 4), join, 300);
    scumble(&mut c, panel(0, 3), BOT, 400);
    c.wait(21.0 * 60.0);
    say(&c, t0, "next day", &[("light", top(3)), ("umber", bot(1)), ("slate", bot(2))]);
    assert_eq!([top(3), bot(1), bot(2)].map(|p| c.drying_at(p.0, p.1)), [Stage::Dry, Stage::Dry, Stage::Tacky], "next day: light and umber dry, slate tacky");
    blend(&mut c, panel(3, 4), join, 500);
    scumble(&mut c, panel(1, 3), BOT, 600);
    // the glaze: a pale veil over the slate with a long Gaussian falloff,
    // through a blurred panel mask (float residue out to the edges)
    let p = panel(2, 3);
    let m = Mask::from_fn(c.frame(), move |x, y| if x > p.0 && x < p.1 && y > BOT.0 && y < BOT.1 { 1.0 } else { 0.0 }).blur(2.0);
    let (gx, gy) = bot(2);
    c.glaze(&Pigment::semi(hex("#e8dcc0")), Some(&m), move |x, y| {
        let d = ((x - gx).powi(2) + (y - gy).powi(2)).sqrt();
        1.2 * (-(d / 55.0).powi(2)).exp()
    });
    say(&c, t0, "glazed (dry)", &[("slate", bot(2))]);
    c.relief(0.3, 0.02);
    o.save(&mut c);
}
