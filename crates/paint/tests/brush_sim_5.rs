#![cfg(feature = "brush-sim-5")]

use paint::{Canvas, Gesture, Handling, Held, Linen, Mask, Paint, Stipple, Tool, Touch, hex};

#[test]
fn sparse_hairs_and_edge_pileup_conserve_finite_material_at_edges_and_turns() {
    for engine in [18, 19, 20] {
        for tool in [Tool::hog_flat(85.0), Tool::filbert(30.0), Tool::badger(85.0), Tool::rigger(12.0)] {
            let mut c = Canvas::new(320, 2.0, hex("#bcb3a2")).with_size_mm(160.0).with_engine(engine).with_linen(Linen::fine(1906));
            let initial = c.material_total();
            let mut a = Held::new(tool, 73).with_engine(engine);
            let mut b = Held::new(Tool::filbert(85.0), 31).with_engine(engine);
            a.load(Paint::body(hex("#be5e32")).with_stiff(0.92).with_thinner(0.2), 0.9);
            b.load(Paint::body(hex("#456e8a")).with_stiff(0.92).with_thinner(0.5), 0.7);
            let input = (a.carried().0 + b.carried().0, a.carried().1 + b.carried().1);
            for y in [2.0, 250.0, 498.0] {
                c.drag(&mut a, &Gesture::line((-100.0, y), (1100.0, y)).pressure(0.7, 0.7), None);
            }
            c.drag(&mut b, &Gesture::new(vec![(450.0, -100.0), (600.0, 300.0), (350.0, 450.0), (550.0, 600.0)]).pressure(0.2, 0.95), None);
            c.touch(&mut b, &Touch::at(999.0, 499.0).drag(50.0, 30.0), None);
            let output = (c.material_total() - initial + a.carried().0 + b.carried().0, c.solvent_total() + a.carried().1 + b.carried().1);
            for (i, o) in [(input.0, output.0), (input.1, output.1)] { assert!((i - o).abs() < i * 0.002, "{engine}: {i} {o}"); }
            assert!(c.seen_lit(115.0, 55.0, 1.0).iter().flatten().all(|v| v.is_finite()));
        }
    }
}

#[test]
fn sparse_contact_history_and_canvas_checkpoint_continue_exactly() {
    for engine in [18, 19, 20] {
        let mut c = Canvas::new(320, 2.0, hex("#bcb3a2")).with_size_mm(160.0).with_engine(engine);
        let mut a = Held::new(Tool::hog_flat(85.0), 71).with_engine(engine);
        a.load(Paint::body(hex("#be5e32")), 0.7);
        c.drag(&mut a, &Gesture::line((100.0, 150.0), (800.0, 220.0)), None);
        let mut state = Vec::new();
        c.write_state(&mut state, "brush-sim-5").unwrap();
        let (mut restored, _) = Canvas::read_state(&mut state.as_slice()).unwrap();
        let mut b = a.clone();
        let g = Gesture::new(vec![(250.0, 420.0), (350.0, 250.0), (900.0, 280.0)]);
        c.drag(&mut a, &g, None);
        restored.drag(&mut b, &g, None);
        let mut continued = Vec::new();
        let mut resumed = Vec::new();
        c.write_state(&mut continued, "continued").unwrap();
        restored.write_state(&mut resumed, "continued").unwrap();
        assert_eq!(continued, resumed);
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
    }
}

#[test]
fn sparse_masked_work_and_stipple_are_thread_exact() {
    for engine in [18, 19, 20] {
        let render = |threads| rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap().install(|| {
            let mut c = Canvas::new(320, 2.0, hex("#bcb3a2")).with_size_mm(160.0).with_engine(engine);
            let m = Mask::from_fn(c.frame(), |x, y| if (50.0..900.0).contains(&x) && (80.0..430.0).contains(&y) { 1.0 } else { 0.0 });
            c.work(&m, &Handling::new(Tool::hog_flat(85.0)).length(80.0, 180.0).coverage(0.6).color(|_, _| hex("#be5e32")).paint(0.92, 1.0).load(0.6), 19);
            c.stipple(&m, &Stipple::new(Tool::filbert(45.0)).coverage(|_, _| 0.2).color(|_, _| hex("#456e8a")).paint(0.92, 1.0).dips(8, 0.08, 1.0), 31);
            let mut state = Vec::new();
            c.write_state(&mut state, "brush-sim-5").unwrap();
            state
        });
        assert_eq!(render(1), render(2));
    }
}

#[test]
fn irregular_broad_strokes_retain_loaded_capacity_and_account_for_unused_paint() {
    let draw = |engine, dips| {
        let mut c = Canvas::new(320, 2.0, hex("#bcb3a2")).with_size_mm(160.0).with_engine(engine);
        let mut h = Held::new(Tool::hog_flat(120.0), 73).with_engine(engine);
        for _ in 0..dips { h.load(Paint::body(hex("#202936")).with_stiff(0.92), 1.0); }
        let input = h.carried().0;
        c.drag(&mut h, &Gesture::line((80.0, 250.0), (920.0, 250.0)).pressure(0.85, 0.8), None);
        let deposited = c.material_total();
        assert!((deposited + h.carried().0 - input).abs() < input * 0.002);
        deposited
    };
    let baseline = draw(5, 1);
    for engine in [18, 19, 20] {
        let light = draw(engine, 1);
        let loaded = draw(engine, 8);
        assert!(loaded >= baseline, "{engine}: {loaded} below baseline {baseline}");
        assert!(loaded > light * 1.5);
    }
}
