use watercolor_pilot::{studio, Watercolor, TICK_NS};

#[test]
fn each_grid_preserves_partitioned_time_frames_and_restore() {
    for cell in [0.025, 0.05, 0.1] {
        let mut full = Watercolor::new_at(32, 24, 71, cell);
        for y in 4..20 {
            for x in 4..28 {
                full.add(y * 32 + x, 0.12, [0.004, 0.002, 0.001]);
            }
        }
        let mut split = full.clone();
        full.wait_ns(2_123_456_789);
        split.wait_ns(123_456_789);
        split = Watercolor::restore(&split.snapshot()).unwrap();
        split.wait_ns_observed(2_000_000_000, 2, |s| {
            s.rgb(false);
        });
        assert_eq!(full, split);
        assert_eq!(full.snapshot(), split.snapshot());
        assert!(full.balance().water_error.abs() < 1e-8);
        assert!(full.balance().pigment_error.iter().all(|e| e.abs() < 1e-10));
    }
}

#[test]
fn dry_fast_forward_has_the_same_clock_and_observed_state() {
    let mut full = Watercolor::new_at(20, 20, 1, 0.05);
    full.add(210, 0.001, [0.004, 0.0, 0.0]);
    let mut sampled = full.clone();
    full.wait_ns(4000 * TICK_NS + 19);
    sampled.wait_ns_observed(4000 * TICK_NS + 19, 317, |_| {});
    assert_eq!(full, sampled);
}

#[test]
fn native_export_has_one_output_pixel_per_simulation_cell() {
    let s = Watercolor::new_at(48, 32, 7, 0.05);
    let p = std::env::temp_dir().join(format!("wc-native-{}.png", std::process::id()));
    studio::render(&s, &p, "gallery", 48, None).unwrap();
    let rendered = image::open(&p).unwrap().to_rgb8();
    assert_eq!(rendered.dimensions(), (48, 32));
    assert_eq!(rendered.into_raw(), s.rgb(false));
    std::fs::remove_file(p).unwrap();
}

#[test]
fn invalid_cell_size_is_rejected_by_checkpoint_validation() {
    let mut s = Watercolor::new_at(16, 16, 7, 0.05);
    s.cell_mm = 0.0;
    assert!(Watercolor::restore(&s.snapshot()).is_err());
}
