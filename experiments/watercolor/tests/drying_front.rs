use watercolor_pilot::{Watercolor, TICK_NS};

#[test]
fn a_thinning_film_does_not_make_a_threshold_capture_seam() {
    let mut base = Watercolor::new_at(32, 32, 7, 0.05);
    base.paper.height.fill(0.5);
    base.paper.sizing.fill(0.6);
    base.paper.capacity.fill(0.22);
    base.paper.clump.fill(1.0);
    base.capillary.fill(0.22);
    base.wet_front.fill(true);
    base.active.coverage.fill(1.0);
    base.suspended.fill([0.004, 0.0, 0.0]);
    base.ledger.pigment_added[0] = 0.004 * 1024.0;
    let mut previous: Option<f64> = None;
    let mut largest_jump = 0.0_f64;
    for step in 0..=400 {
        let water = 0.0003 + step as f64 * 0.000001;
        let mut s = base.clone();
        s.water.fill(water);
        s.ledger.water_added = (water + 0.22) * 1024.0;
        s.wait_ns(TICK_NS);
        let cell = 16 * 32 + 16;
        let captured = s.active.deposited[cell][0] + s.active.bound[cell][0];
        if let Some(p) = previous {
            largest_jump = largest_jump.max((captured - p).abs());
        }
        previous = Some(captured);
        assert!(s.balance().pigment_error[0].abs() < 1e-10);
        assert!(s.balance().water_error.abs() < 1e-8);
    }
    assert!(largest_jump < 0.004 * 0.02, "capture jump {largest_jump}");
}

#[test]
fn pore_water_does_not_keep_a_dried_surface_front_open() {
    let mut s = Watercolor::new_at(16, 16, 7, 0.05);
    s.capillary.fill(0.1);
    s.ledger.water_added = 0.1 * 256.0;
    s.wet_front.fill(true);
    s.flux_x.fill(1.0);
    s.flux_y.fill(1.0);
    s.wait_ns(TICK_NS);
    assert!(s.capillary.iter().any(|v| *v > 0.0));
    assert!(s.water.iter().all(|v| *v == 0.0));
    assert!(s.flux_x.iter().chain(&s.flux_y).all(|v| *v == 0.0));
    assert!(s.balance().water_error.abs() < 1e-10);
}

#[test]
fn contact_line_pinning_does_not_amplify_an_interior_checkerboard() {
    let mut s = Watercolor::new_at(64, 64, 7, 0.05);
    s.paper.height.fill(0.5);
    s.paper.sizing.fill(0.6);
    s.paper.capacity.fill(0.22);
    s.paper.clump.fill(1.0);
    s.capillary.fill(0.22);
    s.wet_front.fill(true);
    s.active.coverage.fill(1.0);
    for y in 0..64 {
        for x in 0..64 {
            let sign = if (x + y) % 2 == 0 { 1.0 } else { -1.0 };
            s.water[y * 64 + x] = 0.008 + sign * 1e-8;
        }
    }
    s.ledger.water_added = s.water.iter().sum::<f64>() + 0.22 * 4096.0;
    s.wait_ns(TICK_NS);
    let mut amplitude = 0.0;
    for y in 16..48 {
        for x in 16..48 {
            let sign = if (x + y) % 2 == 0 { 1.0 } else { -1.0 };
            amplitude += sign * s.water[y * 64 + x] / 1024.0;
        }
    }
    assert!(
        amplitude.abs() <= 1.001e-8,
        "checkerboard amplitude {amplitude}"
    );
    assert!(s.balance().water_error.abs() < 1e-8);
}
