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
