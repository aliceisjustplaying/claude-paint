//! A small bare tree painted the way the round 10-11 painters did: each limb
//! one Gesture with a release ramp, children set on points of the parent's
//! path (its end included), each child pressed at the parent's linear
//! pressure there (r11-study1's final method).
use paint::*;

struct Rng(u64);
impl Rng {
    fn f(&mut self) -> f32 { self.0 ^= self.0 << 13; self.0 ^= self.0 >> 7; self.0 ^= self.0 << 17; (self.0 >> 40) as f32 / (1u64 << 24) as f32 }
    fn range(&mut self, a: f32, b: f32) -> f32 { a + (b - a) * self.f() }
}

fn main() {
    let out = std::env::args().nth(1).expect("out dir");
    let mut c = Canvas::new(3200, 2.0, hex("#d9d2c2"));
    let mut rng = Rng(0x9e3779b97f4a7c15);
    let bark = Paint::body(hex("#2a231d"));
    // (big limbs stay within one load of their brush: a brush run dry skips,
    // which is a different thing from the lift)
    // trunk and big limbs: a blunt round, pressed, lifted into the fork
    let mut trunk = Held::new(Tool::round_sable(9.0), 1);
    trunk.load(bark, 1.0);
    c.drag(&mut trunk, &Gesture::new(vec![(500.0, 470.0), (498.0, 420.0), (502.0, 370.0), (500.0, 330.0)]).pressure(1.0, 0.8).ramps(0.0, 0.05).shake(0.5), None);
    // (position, angle, length, pressure, depth)
    let mut stack: Vec<((f32, f32), f32, f32, f32, u32)> = vec![
        ((500.0, 332.0), -2.2, 85.0, 0.85, 0),
        ((500.0, 332.0), -1.0, 90.0, 0.85, 0),
        ((500.0, 335.0), -1.6, 80.0, 0.8, 0),
    ];
    // limbs: r11-astra's pointed sable; twigs: r11-study1's pointed rigger
    let mut limb = Held::new(Tool { point: 0.88, ..Tool::round_sable(4.0) }, 2);
    let mut twig = Held::new(Tool { point: 0.9, ..Tool::rigger(1.4) }, 3);
    let mut junctions: Vec<(f32, f32, u32)> = Vec::new();
    let mut n = 0;
    while let Some((p, a, len, pr, depth)) = stack.pop() {
        n += 1;
        if n > 1500 { break; }
        let pieces = 2 + (rng.f() * 2.0) as usize;
        let mut pts = vec![p];
        let (mut ang, mut q) = (a, p);
        for _ in 0..pieces {
            ang += rng.range(-0.45, 0.45);
            let l = len / pieces as f32 * rng.range(0.7, 1.3);
            q = (q.0 + ang.cos() * l, q.1 + ang.sin() * l);
            pts.push(q);
        }
        let parent = len > 12.0 && depth < 5;
        let tip = if parent { pr * 0.45 } else { 0.0 };
        let b = if depth < 1 { &mut limb } else { &mut twig };
        b.reload(bark, 0.8);
        c.drag(b, &Gesture::new(pts.clone()).pressure(pr, tip).ramps(0.03, 0.35).shake(0.5), None);
        if parent {
            let kids = 1 + (rng.f() * 2.2) as usize;
            for _ in 0..kids {
                let j = (1 + (rng.f() * (pts.len() - 1) as f32) as usize).min(pts.len() - 1);
                let side = if rng.f() < 0.5 { -1.0 } else { 1.0 };
                let base = (pts[j].1 - pts[j - 1].1).atan2(pts[j].0 - pts[j - 1].0);
                let na = (base + side * rng.range(0.4, 1.0)) * 0.75 + (-1.57) * 0.25;
                let at = pr + (tip - pr) * j as f32 / (pts.len() - 1) as f32;
                if j == pts.len() - 1 { junctions.push((pts[j].0, pts[j].1, depth)); }
                stack.push((pts[j], na, len * rng.range(0.45, 0.65), (at * 0.9).max(0.22), depth + 1));
            }
        }
    }
    c.save(format!("{out}/tree.png")).unwrap();
    let js: Vec<String> = junctions.iter().map(|j| format!("[{:.2},{:.2},{}]", j.0, j.1, j.2)).collect();
    std::fs::write(format!("{out}/junctions.json"), format!("[{}]", js.join(","))).unwrap();
    println!("{} strokes, {} children set on a parent's end", n, junctions.len());
}
