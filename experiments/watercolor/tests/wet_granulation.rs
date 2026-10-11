use watercolor_pilot::{Watercolor, PIGMENTS, TICK_NS};

fn valley_response(depth: f64, coat: f64) -> [f64; PIGMENTS] {
    let mut s = Watercolor::new_at(32, 24, 71, 0.05);
    s.paper.sizing.fill(0.6);
    s.paper.capacity.fill(0.22);
    s.capillary.fill(0.22);
    s.ledger.water_added = 0.22 * 768.0;
    for i in 0..768 {
        s.paper.height[i] = if i % 32 < 16 { 0.2 } else { 0.8 };
        s.add(i, depth, [0.0007; PIGMENTS]);
        s.active.bound[i][0] = coat;
        s.ledger.pigment_added[0] += coat;
    }
    s.wet_front.fill(false);
    s.wait_ns(TICK_NS);
    let low = 12 * 32 + 8;
    let high = 12 * 32 + 24;
    assert!(s.balance().water_error.abs() < 1e-8);
    assert!(s.balance().pigment_error.iter().all(|e| e.abs() < 1e-10));
    std::array::from_fn(|p| {
        (s.active.deposited[low][p] + s.active.bound[low][p]
            - s.active.deposited[high][p]
            - s.active.bound[high][p])
            / 0.0007
    })
}

#[test]
fn wet_settling_is_pigment_specific_and_fades_in_pools_and_filled_valleys() {
    let pale = valley_response(0.006, 0.0);
    let pooled = valley_response(0.3, 0.0);
    let filled = valley_response(0.006, 0.02);
    println!("valley response: pale {pale:?}, pooled {pooled:?}, filled {filled:?}");
    assert!(pale[0] > pale[1] * 10.0);
    assert!(pale[2] > pale[1] * 10.0);
    for p in [0, 2] {
        assert!(pooled[p] < pale[p] * 0.3);
        assert!(filled[p] < pale[p] * 0.15);
    }
}

#[test]
fn fine_contact_tooth_does_not_print_a_wet_checkerboard() {
    let mut s = Watercolor::new_at(32, 24, 71, 0.05);
    s.paper.sizing.fill(0.6);
    s.paper.capacity.fill(0.22);
    s.capillary.fill(0.22);
    s.ledger.water_added = 0.22 * 768.0;
    for y in 0..24 {
        for x in 0..32 {
            let i = y * 32 + x;
            s.paper.height[i] = if (x + y) % 2 == 0 { 0.2 } else { 0.8 };
            s.add(i, 0.006, [0.0007, 0.0, 0.0]);
        }
    }
    let tooth = s.paper.height.clone();
    s.wet_front.fill(false);
    s.wait_ns(TICK_NS);
    assert_eq!(s.paper.height, tooth);
    let mut contrast = 0.0;
    for y in 6..18 {
        for x in 6..26 {
            let i = y * 32 + x;
            let sign = if (x + y) % 2 == 0 { 1.0 } else { -1.0 };
            contrast += sign * (s.active.deposited[i][0] + s.active.bound[i][0]) / 240.0;
        }
    }
    assert!(contrast.abs() < 1e-12, "wet fine-tooth contrast {contrast}");
}
