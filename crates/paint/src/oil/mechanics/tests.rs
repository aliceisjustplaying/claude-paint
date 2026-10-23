use super::*;

fn close(actual: f64, expected: f64, relative: f64) {
    assert!(
        (actual - expected).abs() <= relative * expected.abs().max(f64::MIN_POSITIVE),
        "{actual:e} != {expected:e}"
    );
}

#[test]
fn oil_mechanics_depth_rigidity_and_force_signs() {
    let e = 2.0e6;
    let h = 25.0e-6;
    let nu = 0.45;
    let one = integrate_skin(
        &[SkinCell {
            z_lo_m: 0.0,
            z_hi_m: h,
            young_pa: e,
            poisson: nu,
        }],
        0.5 * h,
    )
    .unwrap();
    close(one.a[0][0], e * h / (1.0 - nu * nu), 1.0e-14);
    close(
        one.d[0][0],
        e * h.powi(3) / (12.0 * (1.0 - nu * nu)),
        1.0e-14,
    );
    assert_eq!(one.b, [[0.0; 3]; 3]);
    let split = integrate_skin(
        &[
            SkinCell {
                z_lo_m: 0.0,
                z_hi_m: 0.3 * h,
                young_pa: e,
                poisson: nu,
            },
            SkinCell {
                z_lo_m: 0.3 * h,
                z_hi_m: h,
                young_pa: e,
                poisson: nu,
            },
        ],
        0.5 * h,
    )
    .unwrap();
    close(split.d[0][0], one.d[0][0], 1.0e-14);
    let retained = PlateHistory {
        tension_n_m: [-1.0, -2.0, 0.3],
        moment_n: [0.01, -0.02, 0.003],
    };
    assert_eq!(
        advance_history(&one, retained, [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], 1.0),
        retained
    );
    let stiffer = integrate_skin(
        &[SkinCell {
            z_lo_m: 0.0,
            z_hi_m: h,
            young_pa: 2.0 * e,
            poisson: nu,
        }],
        0.5 * h,
    )
    .unwrap();
    assert_eq!(
        advance_history(
            &stiffer, retained, [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], 1.0
        ),
        retained
    );
    let relaxed = advance_history(&one, retained, [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], 0.5);
    assert_eq!(relaxed.tension_n_m, retained.tension_n_m.map(|v| v * 0.5));
    assert_eq!(relaxed.moment_n, retained.moment_n.map(|v| v * 0.5));
    let free = [0.01, 0.01, 0.0];
    let chemical: Tensor = std::array::from_fn(|i| (0..3).map(|j| one.a[i][j] * free[j]).sum());
    let expansion = advance_history(
        &one,
        PlateHistory::default(),
        chemical,
        [0.0; 3],
        [0.0; 3],
        [0.0; 3],
        1.0,
    );
    let shrinkage = advance_history(
        &one,
        PlateHistory::default(),
        chemical.map(|v| -v),
        [0.0; 3],
        [0.0; 3],
        [0.0; 3],
        1.0,
    );
    assert!(expansion.tension_n_m[0] < 0.0 && shrinkage.tension_n_m[0] > 0.0);
    close(
        expansion.tension_n_m[0],
        -e * h * free[0] / (1.0 - nu),
        1.0e-14,
    );
    let consumed = advance_history(&one, expansion, [0.0; 3], [0.0; 3], free, [0.0; 3], 1.0);
    assert!(consumed.tension_n_m[0].abs() < 1.0e-14);
    let heterogeneous = integrate_skin(
        &[
            SkinCell {
                z_lo_m: 0.0,
                z_hi_m: h,
                young_pa: e,
                poisson: nu,
            },
            SkinCell {
                z_lo_m: h,
                z_hi_m: 2.0 * h,
                young_pa: 3.0 * e,
                poisson: 0.2,
            },
        ],
        0.0,
    )
    .unwrap();
    assert!(heterogeneous.b[0][0] > 0.0 && heterogeneous.d[0][0] > one.d[0][0]);
}

#[test]
fn oil_mechanics_newtonian_and_bingham_limits() {
    let h = 150.0e-6;
    let eta = 1.0e5;
    let q = 1.0;
    close(
        newtonian_normal_mobility(q, h, eta).unwrap(),
        h.powi(3) * q * q / (3.0 * eta),
        1.0e-7,
    );
    close(
        newtonian_normal_mobility(1.0e8, h, eta).unwrap(),
        1.0 / (2.0 * eta * 1.0e8),
        1.0e-14,
    );
    let g = 1.0e6;
    close(
        bingham_parallel_flux(g, h, eta, 0.0, false).unwrap(),
        -h.powi(3) * g / (3.0 * eta),
        1.0e-14,
    );
    close(
        bingham_parallel_flux(g, h, eta, 0.0, true).unwrap(),
        -h.powi(3) * g / (12.0 * eta),
        1.0e-14,
    );
    assert_eq!(bingham_parallel_flux(g, h, eta, h * g, false).unwrap(), 0.0);
    assert_eq!(
        bingham_parallel_flux(g, h, eta, 0.5 * h * g, true).unwrap(),
        0.0
    );
    assert_eq!(
        bingham_parallel_flux(0.0, h, eta, 75.0, false).unwrap(),
        0.0
    );
    let y = 0.4;
    let flux = bingham_parallel_flux(g, h, eta, y * h * g, false).unwrap();
    close(
        flux,
        -h.powi(3) * g / (3.0 * eta) * (1.0 - 1.5 * y + 0.5 * y.powi(3)),
        1.0e-14,
    );
    assert!(flux * g < 0.0);
    let q = 1.86 / h;
    let finite = newtonian_normal_mobility(q, h, eta).unwrap();
    assert!(finite < h.powi(3) * q * q / (3.0 * eta));
    assert!(traction_free_linear_growth_oracle(q, h, eta, 1.0e-9, 0.0, 0.03).unwrap() < 0.0);
    assert!(traction_free_linear_growth_oracle(q, h, eta, 1.0e-9, 1.0, 0.03).unwrap() < 0.0);
    assert!(traction_free_linear_growth_oracle(q, h, eta, 1.0e-9, -1.0, 0.03).unwrap() > 0.0);
}

#[test]
fn oil_mechanics_matched_control_inputs() {
    let p = controls::proposed_parameters();
    let growing = controls::fixture(controls::Control::Growing, p).unwrap();
    for control in controls::CONTROLS {
        let fixture = controls::fixture(control, p).unwrap();
        match control {
            controls::Control::Growing => assert!(fixture.skins[0].history.tension_n_m[0] < 0.0),
            controls::Control::NoCompression => {
                assert_eq!(fixture.skins[0].history.tension_n_m, [0.0; 3]);
                assert!(fixture.cells.iter().all(|c| c.yield_pa == 0.0));
            }
            controls::Control::Tension => assert!(fixture.skins[0].history.tension_n_m[0] > 0.0),
            controls::Control::SubcriticalCompression => {
                assert!(fixture.skins[0].history.tension_n_m[0] + p.surface_tension_n_m > 0.0)
            }
            controls::Control::SolidBulk | controls::Control::ThinFullyCoherent => {
                assert!(fixture.cells.is_empty());
                assert!(fixture.vertices.iter().all(|v| v.fixed.iter().all(|x| *x)));
            }
            controls::Control::NoCoherentSkin => assert!(fixture.skins.is_empty()),
            controls::Control::BelowYield => assert!(
                fixture
                    .cells
                    .iter()
                    .all(|c| c.yield_pa == p.arrest_yield_pa)
            ),
        }
        if !matches!(control, controls::Control::ThinFullyCoherent) {
            assert_eq!(
                fixture
                    .vertices
                    .iter()
                    .map(|v| v.position_m)
                    .collect::<Vec<_>>(),
                growing
                    .vertices
                    .iter()
                    .map(|v| v.position_m)
                    .collect::<Vec<_>>()
            );
        }
    }
}

#[test]
fn oil_mechanics_coupled_stokes_mobility_limits() {
    let h = 150.0e-6;
    let eta = 1.0e5;
    let q = 1.0;
    let m = newtonian_surface_mobility(q, h, eta).unwrap();
    close(m[0][0], h / eta, 1.0e-7);
    close(m[0][1], -q * h * h / (2.0 * eta), 1.0e-7);
    close(m[1][1], h.powi(3) * q * q / (3.0 * eta), 1.0e-7);
    close(
        m[1][1] - m[0][1] * m[1][0] / m[0][0],
        h.powi(3) * q * q / (12.0 * eta),
        1.0e-7,
    );
    let m = newtonian_surface_mobility(1.0e8, h, eta).unwrap();
    close(m[0][0], 1.0 / (2.0 * eta * 1.0e8), 1.0e-14);
    close(m[1][1], m[0][0], 1.0e-14);
    assert_eq!(m[0][1], 0.0);
    let tangent = integrate_skin(
        &[SkinCell {
            z_lo_m: -12.5e-6,
            z_hi_m: 12.5e-6,
            young_pa: 2.0e6,
            poisson: 0.45,
        }],
        0.0,
    )
    .unwrap();
    let q = 1.86 / h;
    let stable = coupled_linear_growth_oracle(q, h, eta, &tangent, 1.0, 0.03).unwrap();
    let unstable = coupled_linear_growth_oracle(q, h, eta, &tangent, -1.0, 0.03).unwrap();
    assert!(stable.iter().all(|v| *v < 0.0));
    assert!(unstable[0] > 0.0 && unstable[1] < 0.0);
}

#[test]
fn oil_mechanics_solid_retains_geometry_and_history() {
    let mut p = controls::proposed_parameters();
    p.shape = [2, 2, 1];
    for control in [
        controls::Control::SolidBulk,
        controls::Control::ThinFullyCoherent,
    ] {
        let input = controls::fixture(control, p).unwrap();
        let mut scratch = MechanicsScratch::default();
        let mut remap = controls::UniformFixtureRemap;
        let result = attempt_step(
            input.view(),
            StepInterval {
                start_s: 0.0,
                end_s: 1.0,
            },
            &controls::proposed_accuracy(),
            &mut scratch,
            &mut remap,
        )
        .unwrap();
        let StepAttempt::Accepted(step) = result else {
            panic!("solid control subdivided");
        };
        assert!(
            step.trial.transfers.is_empty()
                && step.trial.skin_transfers.iter().all(|f| f.volume_m3 == 0.0)
        );
        assert_eq!(input.vertices, step.remapped.vertices);
        assert_eq!(
            input.skins.iter().map(|s| s.history).collect::<Vec<_>>(),
            step.remapped
                .skins
                .iter()
                .map(|s| s.history)
                .collect::<Vec<_>>()
        );
        assert!(step.trial.diagnostics.arrested);
    }
}

#[test]
fn oil_mechanics_support_and_material_validation() {
    assert_eq!(
        integrate_skin(
            &[SkinCell {
                z_lo_m: 0.0,
                z_hi_m: 1.0,
                young_pa: 1.0,
                poisson: 0.5
            }],
            0.0
        ),
        Err(MechanicsError::InvalidMaterial)
    );
    assert!(newtonian_normal_mobility(1.0, 1.0, 0.0).is_err());
    let mut p = controls::proposed_parameters();
    p.shape = [8, 8, 1];
    let mut fixture = controls::fixture(controls::Control::NoCompression, p).unwrap();
    fixture.cells[0].eta_pa_s = 0.0;
    let result = attempt_step(
        fixture.view(),
        StepInterval {
            start_s: 0.0,
            end_s: 1.0,
        },
        &controls::proposed_accuracy(),
        &mut MechanicsScratch::default(),
        &mut controls::UniformFixtureRemap,
    );
    assert!(matches!(result, Err(MechanicsError::InvalidMaterial)));
}

#[test]
fn oil_mechanics_variational_patch_controls_and_transfers() {
    let mut p = controls::proposed_parameters();
    p.shape = [8, 8, 2];
    p.width_m = 0.4e-3;
    p.seed_cycles = [1, 1];
    let accuracy = controls::proposed_accuracy();
    let mut scratch = MechanicsScratch::default();
    for control in controls::CONTROLS {
        let patch = controls::fixture(control, p).unwrap();
        let top_center = 202;
        let result = attempt_step(
            patch.view(),
            StepInterval {
                start_s: 0.0,
                end_s: 1.0e-3,
            },
            &accuracy,
            &mut scratch,
            &mut controls::UniformFixtureRemap,
        )
        .unwrap();
        let StepAttempt::Accepted(step) = result else {
            panic!("{control:?}: {result:?}");
        };
        let dz = step.remapped.vertices[top_center].position_m[2]
            - patch.vertices[top_center].position_m[2];
        let resolved_change =
            step.trial.diagnostics.estimated_displacement_error_m + 1.0e-8 * p.relief_m;
        match control {
            controls::Control::Growing => {
                assert!(
                    dz > resolved_change,
                    "positive did not show resolved growth: {dz:e}, floor {resolved_change:e}"
                );
                assert!(step.trial.transfers.iter().any(|f| f.volume_m3 != 0.0));
                let before: f64 = patch
                    .skins
                    .iter()
                    .map(|s| s.history.tension_n_m[0] + s.history.tension_n_m[1])
                    .sum();
                let after: f64 = step
                    .remapped
                    .skins
                    .iter()
                    .map(|s| s.history.tension_n_m[0] + s.history.tension_n_m[1])
                    .sum();
                assert!(after > before, "metric did not consume compression");
            }
            controls::Control::NoCompression
            | controls::Control::Tension
            | controls::Control::SubcriticalCompression => {
                assert!(
                    dz < -resolved_change,
                    "{control:?} did not show resolved decay: {dz:e}, floor {resolved_change:e}"
                );
                assert!(step.trial.transfers.iter().any(|f| f.volume_m3 != 0.0));
            }
            controls::Control::SolidBulk
            | controls::Control::ThinFullyCoherent
            | controls::Control::BelowYield => {
                assert_eq!(dz, 0.0);
                assert!(step.trial.diagnostics.arrested);
                assert!(step.trial.transfers.iter().all(|f| f.volume_m3 == 0.0));
            }
            controls::Control::NoCoherentSkin => assert!(dz <= 0.0),
        }
        let d = &step.trial.diagnostics;
        assert!(d.viscous_dissipation_j >= 0.0 && d.plastic_dissipation_j >= 0.0);
        assert!(
            d.energy_increment_j + 0.5 * d.viscous_dissipation_j + d.plastic_dissipation_j
                <= accuracy.traction_error_pa
                    * p.width_m.powi(2)
                    * accuracy.absolute_displacement_error_m
        );
        let old = step
            .trial
            .end_cell_volumes_m3
            .iter()
            .map(|&(id, _)| {
                let mut v = 0.0;
                for c in patch.cells.iter().filter(|c| c.material_cell == id) {
                    let a = c.vertices.map(|i| patch.vertices[i].position_m);
                    let b = [a[1][0] - a[0][0], a[1][1] - a[0][1], a[1][2] - a[0][2]];
                    let q = [a[2][0] - a[0][0], a[2][1] - a[0][1], a[2][2] - a[0][2]];
                    let r = [a[3][0] - a[0][0], a[3][1] - a[0][1], a[3][2] - a[0][2]];
                    v += (b[0] * (q[1] * r[2] - q[2] * r[1])
                        + b[1] * (q[2] * r[0] - q[0] * r[2])
                        + b[2] * (q[0] * r[1] - q[1] * r[0]))
                        / 6.0;
                }
                (id, v)
            })
            .collect::<BTreeMap<_, _>>();
        let initial_volume: f64 = old.values().sum();
        let end_volume: f64 = step.trial.end_cell_volumes_m3.iter().map(|x| x.1).sum();
        if initial_volume > 0.0 {
            close(end_volume, initial_volume, 1.0e-13);
        }
        let mut tracer: BTreeMap<_, _> = old
            .iter()
            .map(|(&id, &v)| (id, if id % 2 == 0 { v } else { 0.0 }))
            .collect();
        let tracer_before: f64 = tracer.values().sum();
        for f in &step.trial.transfers {
            let donor = if f.volume_m3 >= 0.0 {
                f.low_cell
            } else {
                f.high_cell
            };
            let amount = f.volume_m3 * if donor % 2 == 0 { 1.0 } else { 0.0 };
            *tracer.get_mut(&f.low_cell).unwrap() -= amount;
            *tracer.get_mut(&f.high_cell).unwrap() += amount;
        }
        if tracer_before > 0.0 {
            close(tracer.values().sum(), tracer_before, 1.0e-13);
        }
        assert!(tracer.values().all(|v| *v >= 0.0));
    }
}
