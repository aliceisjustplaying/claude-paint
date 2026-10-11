use watercolor_pilot::{compose_km, Watercolor, PIGMENTS, TICK_NS};

fn wet() -> Watercolor {
    let mut s = Watercolor::new(24, 18, 47);
    for y in 3..15 {
        for x in 3..21 {
            s.add(y * 24 + x, 0.16, [0.006, 0.004, 0.007]);
        }
    }
    s
}

fn check(s: &Watercolor) {
    let b = s.balance();
    assert!(
        b.water_error.abs() < 1e-9 * s.ledger.water_added.max(1.0),
        "{b:?}"
    );
    for p in 0..PIGMENTS {
        assert!(
            b.pigment_error[p].abs() < 1e-10 * s.ledger.pigment_added[p].max(1.0),
            "{b:?}"
        );
    }
    assert!(s
        .water
        .iter()
        .chain(&s.capillary)
        .all(|v| v.is_finite() && *v >= 0.0));
    assert!(s
        .suspended
        .iter()
        .flatten()
        .all(|v| v.is_finite() && *v >= 0.0));
    for g in std::iter::once(&s.active).chain(&s.glazes) {
        assert!(g
            .deposited
            .iter()
            .chain(&g.bound)
            .flatten()
            .all(|v| v.is_finite() && *v >= 0.0));
    }
    for i in 0..s.water.len() {
        assert!(s.capillary[i] <= s.paper.capacity[i] + 1e-12);
    }
}

#[test]
fn conservation_and_positivity_through_drying_rewetting_and_lifting() {
    let mut s = wet();
    for _ in 0..3000 {
        s.wait_ns(TICK_NS);
        check(&s);
    }
    assert!(s.ledger.water_evaporated > s.ledger.water_added * 0.99);
    s.finish_glaze().unwrap();
    for y in 6..12 {
        for x in 6..18 {
            let i = y * s.width + x;
            s.add(i, 0.07, [0.0; PIGMENTS]);
        }
    }
    s.wait_seconds(0.5);
    for y in 6..12 {
        for x in 6..18 {
            s.lift(y * s.width + x, 0.95);
        }
    }
    for _ in 0..200 {
        s.wait_ns(TICK_NS);
        check(&s);
    }
}

#[test]
fn arbitrary_nanosecond_partitions_and_restore_are_bit_identical() {
    let mut long = wet();
    let mut split = long.clone();
    long.wait_ns(9_777_123_456);
    split.wait_ns(123_456_789);
    split = Watercolor::restore(&split.snapshot()).unwrap();
    split.wait_ns(9_653_666_667);
    assert_eq!(long, split);
    assert_eq!(long.snapshot(), split.snapshot());
    assert_eq!(long.rgb(false), split.rgb(false));
    assert_eq!(long.rgb(true), split.rgb(true));
}

#[test]
fn presentation_frames_do_not_change_physics() {
    let mut long = wet();
    let mut sampled = long.clone();
    long.wait_seconds(8.0);
    for _ in 0..128 {
        sampled.wait_seconds(0.0625);
        let before = sampled.snapshot();
        sampled.rgb(false);
        sampled.rgb(true);
        assert_eq!(before, sampled.snapshot());
    }
    assert_eq!(long, sampled);
    let mut observed = wet();
    let mut times = Vec::new();
    let mut frames = Vec::new();
    observed.wait_ns_observed(123_456_789, 2, |s| {
        times.push(s.tick);
        frames.push(s.snapshot());
    });
    observed = Watercolor::restore(&observed.snapshot()).unwrap();
    observed.wait_ns_observed(7_876_543_211, 2, |s| {
        times.push(s.tick);
        frames.push(s.snapshot());
    });
    assert_eq!(long, observed);
    assert_eq!(times, (2..=128).step_by(2).collect::<Vec<_>>());
    let mut direct = wet();
    let mut direct_frames = Vec::new();
    direct.wait_ns_observed(8_000_000_000, 2, |s| direct_frames.push(s.snapshot()));
    assert_eq!(direct_frames, frames);
}

#[test]
fn donor_limits_survive_extreme_puddle_and_flux() {
    let mut s = Watercolor::new(12, 12, 5);
    for i in 0..144 {
        s.add(i, 0.0001, [1e-5; PIGMENTS]);
    }
    s.add(65, 25.0, [0.8, 0.6, 0.2]);
    s.flux_x[65] = 1e7;
    s.flux_y[65] = -1e7;
    for _ in 0..100 {
        s.wait_ns(TICK_NS);
        check(&s);
    }
}

#[test]
fn staining_resists_a_damp_brush() {
    let mut s = wet();
    s.wait_seconds(190.0);
    s.finish_glaze().unwrap();
    for i in 0..s.water.len() {
        s.add(i, 0.035, [0.0; PIGMENTS]);
    }
    for i in 0..s.water.len() {
        s.lift(i, 1.0);
    }
    let fraction: Vec<_> = (0..PIGMENTS)
        .map(|p| s.ledger.pigment_lifted[p] / s.ledger.pigment_added[p])
        .collect();
    assert!(fraction[0] > fraction[1] * 3.0, "{fraction:?}");
    check(&s);
}

#[test]
fn granulating_pigment_prefers_low_tooth() {
    let mut s = wet();
    s.wait_seconds(3.0);
    let mut low = [0.0; PIGMENTS];
    let mut high = [0.0; PIGMENTS];
    let (mut nl, mut nh) = (0.0, 0.0);
    for y in 5..13 {
        for x in 5..19 {
            let i = y * s.width + x;
            let h = s.paper.height[i];
            if h < 0.4 {
                nl += 1.0;
                for (p, value) in low.iter_mut().enumerate() {
                    *value += s.active.deposited[i][p] + s.active.bound[i][p];
                }
            }
            if h > 0.6 {
                nh += 1.0;
                for (p, value) in high.iter_mut().enumerate() {
                    *value += s.active.deposited[i][p] + s.active.bound[i][p];
                }
            }
        }
    }
    let ratio: Vec<_> = (0..PIGMENTS)
        .map(|p| (low[p] / nl) / (high[p] / nh))
        .collect();
    assert!(ratio[0] > ratio[1] * 1.2, "{ratio:?}");
}

#[test]
fn km_is_bounded_transparent_at_zero_and_ordered() {
    let paper = [0.92, 0.9, 0.85];
    assert_eq!(compose_km(paper, [0.0; 3]), paper);
    let blue_earth = compose_km(compose_km(paper, [0.025, 0.0, 0.0]), [0.0, 0.0, 0.025]);
    let earth_blue = compose_km(compose_km(paper, [0.0, 0.0, 0.025]), [0.025, 0.0, 0.0]);
    assert!(blue_earth
        .iter()
        .zip(earth_blue)
        .any(|(a, b)| (a - b).abs() > 0.01));
    for mass in [[0.0001; 3], [0.1; 3], [1e6; 3]] {
        assert!(compose_km(paper, mass)
            .iter()
            .all(|v| v.is_finite() && (0.0..=1.0).contains(v)));
    }
}

#[test]
fn dry_contact_uses_the_fiber_tooth() {
    let mut s = Watercolor::new(32, 24, 17);
    for i in 0..s.water.len() {
        s.dry_contact(i, [0.01, 0.0, 0.0], 0.3);
    }
    assert!(s.active.deposited.iter().any(|v| v[0] == 0.0));
    assert!(s.active.deposited.iter().any(|v| v[0] > 0.009));
    assert_eq!(s.ledger.water_added, 0.0);
    check(&s);
}

#[test]
fn wet_glaze_and_malformed_snapshot_are_refused() {
    let mut s = wet();
    assert!(s.finish_glaze().is_err());
    assert!(Watercolor::restore(b"{}").is_err());
    s.water.pop();
    assert!(Watercolor::restore(&s.snapshot()).is_err());
}

#[test]
fn a_pinned_wash_transports_pigment_to_the_evaporating_edge() {
    let mut s = Watercolor::new(40, 32, 3);
    s.paper.height.fill(0.5);
    s.paper.sizing.fill(0.65);
    s.paper.capacity.fill(0.22);
    s.paper.clump.fill(1.0);
    for y in 4..28 {
        for x in 4..36 {
            s.add(y * 40 + x, 0.15, [0.004, 0.0, 0.0]);
        }
    }
    s.wait_seconds(140.0);
    let mut edge = Vec::new();
    let mut center = Vec::new();
    for y in 4..28 {
        for x in 4..36 {
            let i = y * 40 + x;
            let mass = s.active.deposited[i][0] + s.active.bound[i][0];
            if x == 4 || x == 35 || y == 4 || y == 27 {
                edge.push(mass);
            } else if (12..28).contains(&x) && (10..22).contains(&y) {
                center.push(mass);
            }
        }
    }
    let edge = edge.iter().sum::<f64>() / edge.len() as f64;
    let center = center.iter().sum::<f64>() / center.len() as f64;
    assert!(edge > center * 1.1, "edge {edge} center {center}");
    check(&s);
}

#[test]
fn pure_water_has_a_capillary_wet_front_and_explicit_evaporation() {
    let mut s = Watercolor::new(20, 16, 9);
    let i = 8 * 20 + 10;
    s.add(i, 5.0, [0.0; PIGMENTS]);
    s.wait_seconds(30.0);
    assert!(s.capillary[i + 1] > 0.0);
    assert!(s.wet_front.iter().filter(|v| **v).count() > 1);
    assert!(s.ledger.water_evaporated > 0.0);
    assert_eq!(s.ledger.pigment_added, [0.0; PIGMENTS]);
    check(&s);
}
