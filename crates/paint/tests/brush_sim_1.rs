#![cfg(feature = "brush-sim-1")]

use paint::{Canvas, Gesture, Handling, Held, Mask, Paint, Part, Stipple, Tool, Touch, hex};

fn canvas(engine: u32) -> Canvas {
    Canvas::new(320, 2.0, hex("#bcb3a2")).with_size_mm(160.0).with_engine(engine)
}

#[test]
fn normalized_loads_keep_the_baseline_budget() {
    for engine in [6, 7, 8] {
        for part in [Part::ALL, Part { side: -0.6, share: 0.4, streak: 0.7, seed: 17 }] {
            let p = Paint::body(hex("#be5e32")).with_stiff(0.92).with_thinner(0.2);
            let mut a = Held::new(Tool::hog_flat(85.0), 73).with_engine(5);
            let mut b = a.clone().with_engine(engine);
            a.load_part(p, 0.7, &part);
            b.load_part(p, 0.7, &part);
            for (x, y) in [(a.carried().0, b.carried().0), (a.carried().1, b.carried().1)] {
                assert!((x - y).abs() < x * 2e-6, "{engine}: {x} {y}");
            }
        }
    }
}

#[test]
fn wet_crossings_touches_and_edges_conserve_finite_paint() {
    for engine in [6, 7, 8] {
        for width in [12.0, 85.0, 170.0] {
            let mut c = canvas(engine);
            let initial = c.material_total();
            let mut a = Held::new(Tool::hog_flat(width), 73).with_engine(engine);
            let mut b = Held::new(Tool::filbert(width), 31).with_engine(engine);
            a.load(Paint::body(hex("#be5e32")).with_stiff(0.92), 0.9);
            b.load(Paint::body(hex("#456e8a")).with_stiff(0.92).with_thinner(0.2), 0.7);
            let input = a.carried().0 + b.carried().0;
            for y in [2.0, 250.0, 498.0] {
                c.drag(&mut a, &Gesture::line((-100.0, y), (1100.0, y)).pressure(0.7, 0.7), None);
            }
            c.drag(&mut b, &Gesture::line((450.0, -100.0), (550.0, 600.0)).pressure(0.2, 0.95), None);
            c.touch(&mut b, &Touch::at(999.0, 499.0).drag(50.0, 30.0), None);
            let output = c.material_total() - initial + a.carried().0 + b.carried().0;
            assert!((input - output).abs() < input * 0.002, "{engine}/{width}: {input} {output}");
            assert!(c.surface_um().iter().all(|h| h.is_finite()));
            assert!(c.seen_lit(115.0, 55.0, 1.0).iter().flatten().all(|v| v.is_finite()));
        }
    }
}

#[test]
fn repeated_edge_strokes_do_not_concentrate_into_piles() {
    for engine in [6, 7, 8] {
        let mut c = canvas(engine);
        for i in 0..24 {
            let mut h = Held::new(Tool::hog_flat(85.0), i + 73).with_engine(engine);
            h.load(Paint::body(hex("#be5e32")).with_stiff(0.92), 1.0);
            c.drag(&mut h, &Gesture::line((-100.0, 498.0), (1100.0, 498.0)).pressure(0.8, 0.8), None);
        }
        let f = c.frame();
        let peak = (0..f.w * f.h).map(|i| c.wet_um(f.ux(i % f.w), f.uy(i / f.w))).fold(0.0, f32::max);
        assert!(peak < 2500.0, "engine {engine}: {peak} um");
    }
}

#[test]
fn scheduled_work_is_thread_deterministic_and_checkpointed() {
    for engine in [6, 7, 8] {
        let render = |threads| {
            rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap().install(|| {
                let mut c = canvas(engine);
                let m = Mask::from_fn(c.frame(), |x, y| if (50.0..900.0).contains(&x) && (80.0..430.0).contains(&y) { 1.0 } else { 0.0 });
                let h = Handling::new(Tool::hog_flat(85.0)).length(80.0, 180.0).coverage(0.6).color(|_, _| hex("#be5e32")).paint(0.92, 1.0).load(0.6);
                c.work(&m, &h, 19);
                let sp = Stipple::new(Tool::filbert(45.0)).coverage(|_, _| 0.2).color(|_, _| hex("#456e8a")).paint(0.92, 1.0).dips(8, 0.08, 1.0);
                c.stipple(&m, &sp, 31);
                let mut state = Vec::new();
                c.write_state(&mut state, "brush-sim-1").unwrap();
                let (mut restored, _) = Canvas::read_state(&mut state.as_slice()).unwrap();
                assert_eq!(restored.engine(), engine);
                assert_eq!(c.seen_lit(115.0, 55.0, 1.0), restored.seen_lit(115.0, 55.0, 1.0));
                let mut a = Held::new(Tool::hog_flat(85.0), 71).with_engine(engine);
                a.load(Paint::body(hex("#be5e32")), 0.4);
                let mut b = a.clone();
                let g = Gesture::line((100.0, 240.0), (900.0, 280.0));
                c.drag(&mut a, &g, None);
                restored.drag(&mut b, &g, None);
                let mut continued = Vec::new();
                let mut resumed = Vec::new();
                c.write_state(&mut continued, "continued").unwrap();
                restored.write_state(&mut resumed, "continued").unwrap();
                assert_eq!(continued, resumed);
                state
            })
        };
        assert_eq!(render(1), render(2));
    }
}
