#![cfg(feature = "brush-sim-2")]

use paint::{Canvas, Gesture, Handling, Held, Linen, Mask, Paint, Part, Stipple, Tool, Touch, hex};
use paint::color::luminance;
use paint::palette::Palette;

#[test]
fn finite_loads_and_wet_exchange_preserve_paint_and_solvent() {
    for engine in [9, 10, 11] {
        for width in [12.0, 85.0, 170.0] {
            let mut c = Canvas::new(320, 2.0, hex("#bcb3a2")).with_size_mm(160.0).with_engine(engine);
            let initial = c.material_total();
            let mut a = Held::new(Tool::hog_flat(width), 73).with_engine(engine);
            let mut b = Held::new(Tool::filbert(width), 31).with_engine(engine);
            let p = Paint::body(hex("#be5e32")).with_stiff(0.92).with_thinner(0.2);
            let part = Part { side: -0.6, share: 0.4, streak: 0.7, seed: 17 };
            let mut control = a.clone().with_engine(5);
            control.load_part(p, 0.9, &part);
            a.load_part(p, 0.9, &part);
            assert!((control.carried().0 - a.carried().0).abs() < control.carried().0 * 2e-6);
            b.load(Paint::body(hex("#456e8a")).with_stiff(0.92).with_thinner(0.5), 0.7);
            let input = (a.carried().0 + b.carried().0, a.carried().1 + b.carried().1);
            for y in [2.0, 250.0, 498.0] {
                c.drag(&mut a, &Gesture::line((-100.0, y), (1100.0, y)).pressure(0.7, 0.7), None);
            }
            c.drag(&mut b, &Gesture::line((450.0, -100.0), (550.0, 600.0)).pressure(0.2, 0.95), None);
            c.touch(&mut b, &Touch::at(999.0, 499.0).drag(50.0, 30.0), None);
            let output = (c.material_total() - initial + a.carried().0 + b.carried().0, c.solvent_total() + a.carried().1 + b.carried().1);
            for (i, o) in [(input.0, output.0), (input.1, output.1)] {
                assert!((i - o).abs() < i * 0.002, "{engine}/{width}: {i} {o}");
            }
            assert!(c.seen_lit(115.0, 55.0, 1.0).iter().flatten().all(|v| v.is_finite()));
        }
    }
}

#[test]
fn loaded_bone_black_reaches_a_deep_value_in_one_pass() {
    let mut c = Canvas::new(640, 2.0, hex("#eee8da")).with_size_mm(160.0).with_engine(10).with_linen(Linen::fine(1906));
    c.prime(hex("#eee8da"), 0.92, 100.0, 0.9, 0.3, 1906);
    let pal = Palette::tube_box();
    let black = pal.tubes.iter().position(|t| t.name == "bone black").unwrap();
    let mut h = Held::new(Tool::hog_flat(110.0), 73).with_engine(10);
    h.load(pal.pile(vec![(black, 1.0)]).paint(0.05), 0.9);
    c.drag(&mut h, &Gesture::line((110.0, 250.0), (930.0, 250.0)).pressure(0.85, 0.85), None);
    for (gallery, pixels) in [(false, c.seen()), (true, c.seen_lit(115.0, 55.0, 1.0))] {
        let mut ys = Vec::new();
        for y in 150..170 { for x in 160..480 { ys.push(luminance(pixels[y * 640 + x])); } }
        ys.sort_by(f32::total_cmp);
        if gallery {
            assert!(ys[ys.len() / 20] < 0.016, "gallery trough linear Y {}", ys[ys.len() / 20]);
            assert!(ys[ys.len() / 2] < 0.03, "gallery median linear Y {}", ys[ys.len() / 2]);
        } else {
            assert!(ys[ys.len() / 2] < 0.012, "color median linear Y {}", ys[ys.len() / 2]);
            assert!(ys[ys.len() * 95 / 100] < 0.013, "color p95 linear Y {}", ys[ys.len() * 95 / 100]);
        }
    }
}

#[test]
fn wet_pickup_carries_color_beyond_the_crossing_without_lifting_dry_paint() {
    let carry = |dry, pickup| {
        let mut c = Canvas::new(320, 2.0, hex("#bcb3a2")).with_size_mm(160.0).with_engine(10);
        let mut blue = Held::new(Tool::hog_flat(130.0), 31).with_engine(10);
        blue.load(Paint::body(hex("#456e8a")).with_stiff(0.92), 0.9);
        c.drag(&mut blue, &Gesture::line((360.0, 250.0), (610.0, 250.0)).pressure(0.85, 0.85), None);
        if dry {
            c.dry();
            assert_eq!(c.wet_total(), 0.0);
        }
        let mut tool = Tool::hog_flat(85.0);
        tool.pickup = pickup;
        let mut h = Held::new(tool, 73).with_engine(10);
        if !dry { h.load(Paint::body(hex("#be5e32")).with_stiff(0.92), 0.18); }
        c.drag(&mut h, &Gesture::line((120.0, 250.0), (930.0, 250.0)).pressure(0.65, 0.85), None);
        if dry { assert_eq!(h.carried().0, 0.0); }
        let color = c.under(800.0, 250.0, 8.0);
        color[2] - color[0]
    };
    let ordinary = Tool::hog_flat(85.0).pickup;
    let wet = carry(false, ordinary);
    let no_pickup = carry(false, 0.0);
    let dry = carry(true, ordinary);
    let dry_control = carry(true, 0.0);
    assert!(wet > no_pickup + 0.03, "wet carry {wet}, pickup disabled {no_pickup}");
    assert_eq!(dry.to_bits(), dry_control.to_bits(), "dry paint must remain unavailable to pickup");
}

#[test]
fn scheduled_work_and_stipple_are_thread_exact_and_resume_exactly() {
    for engine in [9, 10, 11] {
        let render = |threads| rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap().install(|| {
            let mut c = Canvas::new(320, 2.0, hex("#bcb3a2")).with_size_mm(160.0).with_engine(engine);
            let m = Mask::from_fn(c.frame(), |x, y| if (50.0..900.0).contains(&x) && (80.0..430.0).contains(&y) { 1.0 } else { 0.0 });
            c.work(&m, &Handling::new(Tool::hog_flat(85.0)).length(80.0, 180.0).coverage(0.6).color(|_, _| hex("#be5e32")).paint(0.92, 1.0).load(0.6), 19);
            c.stipple(&m, &Stipple::new(Tool::filbert(45.0)).coverage(|_, _| 0.2).color(|_, _| hex("#456e8a")).paint(0.92, 1.0).dips(8, 0.08, 1.0), 31);
            let mut state = Vec::new();
            c.write_state(&mut state, "brush-sim-2").unwrap();
            let (mut restored, _) = Canvas::read_state(&mut state.as_slice()).unwrap();
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
            continued
        });
        assert_eq!(render(1), render(2));
    }
}
