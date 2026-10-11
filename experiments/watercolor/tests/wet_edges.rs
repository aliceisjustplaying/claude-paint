use watercolor_pilot::{Watercolor, PIGMENTS};

fn wash(damp: bool) -> Watercolor {
    let mut s = Watercolor::new_at(80, 64, 71, 0.05);
    s.paper.height.fill(0.5);
    s.paper.sizing.fill(0.65);
    s.paper.capacity.fill(0.22);
    if damp {
        s.capillary.fill(0.11);
        s.ledger.water_added = 0.11 * 80.0 * 64.0;
    }
    for y in 16..48 {
        for x in 16..64 {
            s.add(y * 80 + x, 0.12, [0.004, 0.0, 0.0]);
        }
    }
    s
}

fn conserved(s: &Watercolor) {
    let b = s.balance();
    assert!(b.water_error.abs() < 1e-8);
    assert!(b.pigment_error.iter().all(|e| e.abs() < 1e-10));
}

#[test]
fn painted_coverage_does_not_drive_the_water_edge() {
    let mut local = wash(false);
    let mut covered = local.clone();
    covered.active.coverage.fill(1.0);
    local.wait_seconds(12.0);
    covered.wait_seconds(12.0);
    assert_eq!(local.water, covered.water);
    assert_eq!(local.capillary, covered.capillary);
    assert_eq!(local.suspended, covered.suspended);
    assert_eq!(local.active.deposited, covered.active.deposited);
    assert_eq!(local.active.bound, covered.active.bound);
    conserved(&local);
    conserved(&covered);
}

#[test]
fn damp_paper_opens_a_conservative_feathering_front() {
    let mut dry = wash(false);
    let mut damp = wash(true);
    dry.wait_seconds(4.0);
    damp.wait_seconds(4.0);
    let outside = |s: &Watercolor| {
        (0..s.water.len())
            .filter(|i| !(16..64).contains(&(i % 80)) || !(16..48).contains(&(i / 80)))
            .map(|i| s.suspended[i][0] + s.active.deposited[i][0] + s.active.bound[i][0])
            .sum::<f64>()
    };
    assert!(outside(&damp) > 0.01);
    assert!(outside(&dry) < 1e-12);
    conserved(&dry);
    conserved(&damp);
    let mut split = wash(true);
    split.wait_seconds(1.25);
    split = Watercolor::restore(&split.snapshot()).unwrap();
    split.wait_seconds(2.75);
    assert_eq!(damp, split);
}

#[test]
fn wet_pigment_join_moves_across_a_material_dependent_front() {
    let mut s = Watercolor::new_at(80, 64, 71, 0.05);
    s.paper.height.fill(0.5);
    s.paper.capacity.fill(0.22);
    for y in 0..64 {
        for x in 0..80 {
            let i = y * 80 + x;
            s.paper.sizing[i] = if y < 32 { 0.3 } else { 0.85 };
            s.capillary[i] = 0.22;
            let mut pigments = [0.0; PIGMENTS];
            pigments[if x < 40 { 0 } else { 2 }] = 0.002;
            s.add(i, 0.12, pigments);
        }
    }
    s.ledger.water_added += 0.22 * 80.0 * 64.0;
    s.wait_seconds(4.0);
    let amount = |i: usize| s.suspended[i][0] + s.active.deposited[i][0] + s.active.bound[i][0];
    assert!(amount(16 * 80 + 44) > 1e-7);
    assert!(amount(16 * 80 + 44) > amount(48 * 80 + 44) * 1.1);
    conserved(&s);
}
