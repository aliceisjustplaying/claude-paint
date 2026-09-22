//! Debug probe: light badger / filbert drags over dried paint at full scale.
use paint::{Canvas, Gesture, Held, Orient, Paint, Tool, hex};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let tool = args.get(1).map(|s| s.as_str()).unwrap_or("badger");
    let p: f32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.4);
    let mut c = Canvas::new(3200, 4.0, hex("#9aa3a6")).with_weave(0.9, 0.35, 1);
    // a dried body layer
    let mut b = Held::new(Tool::filbert(40.0), 1);
    for i in 0..6 {
        b.reload(Paint::body(hex("#6d6f78")), 1.0);
        let y = 30.0 + i as f32 * 30.0;
        c.drag(&mut b, &Gesture::new(vec![(20.0, y), (980.0, y)]).pressure(0.9, 0.9).orient(Orient::Across), None);
    }
    c.dry();
    // light passes on top
    let mut t = match tool {
        "badger" => Held::new(Tool::badger(40.0), 2),
        _ => Held::new(Tool::filbert(9.0), 2),
    };
    for i in 0..4 {
        let y = 60.0 + i as f32 * 40.0;
        if tool != "badger" {
            t.reload(Paint { color: hex("#b0a9a3"), hiding: 0.3, stiff: 0.3 }, 0.6 * 0.3);
        }
        c.drag(&mut t, &Gesture::new(vec![(100.0, y), (900.0, y + 3.0)]).pressure(p, p).orient(Orient::Across), None);
    }
    c.dry();
    c.relief(0.2, 0.02);
    c.save("out/probe.png").unwrap();
}
