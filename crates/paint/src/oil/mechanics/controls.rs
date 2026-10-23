use super::*;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Control {
    Growing,
    NoCompression,
    Tension,
    SubcriticalCompression,
    SolidBulk,
    NoCoherentSkin,
    ThinFullyCoherent,
    BelowYield,
}

pub const CONTROLS: [Control; 8] = [
    Control::Growing,
    Control::NoCompression,
    Control::Tension,
    Control::SubcriticalCompression,
    Control::SolidBulk,
    Control::NoCoherentSkin,
    Control::ThinFullyCoherent,
    Control::BelowYield,
];

#[derive(Clone, Copy, Debug)]
pub struct Parameters {
    pub shape: [usize; 3],
    pub width_m: f64,
    pub gap_m: f64,
    pub skin_m: f64,
    pub thin_film_m: f64,
    pub young_pa: f64,
    pub poisson: f64,
    pub eta_pa_s: f64,
    pub yield_pa: f64,
    pub arrest_yield_pa: f64,
    pub surface_tension_n_m: f64,
    pub expansive_strain: f64,
    pub relief_m: f64,
    pub seed_cycles: [usize; 2],
}

pub fn proposed_parameters() -> Parameters {
    Parameters {
        shape: [8, 8, 3],
        width_m: 0.4e-3,
        gap_m: 150.0e-6,
        skin_m: 25.0e-6,
        thin_film_m: 12.0e-6,
        young_pa: 2.0e6,
        poisson: 0.45,
        eta_pa_s: 1.0e5,
        yield_pa: 75.0,
        arrest_yield_pa: 1.0e5,
        surface_tension_n_m: 0.03,
        expansive_strain: 0.04,
        relief_m: 3.0e-6,
        seed_cycles: [1, 1],
    }
}

pub fn proposed_accuracy() -> MechanicsAccuracy {
    MechanicsAccuracy {
        divergence_error_s_inv: 1.0e-9,
        traction_error_pa: 0.01,
        relative_time_error: 0.01,
        absolute_displacement_error_m: 1.0e-10,
        maximum_resolved_slope: 0.3,
        maximum_resolved_strain: 0.1,
        maximum_donor_fraction: 0.1,
        quadrature_order: 4,
        maximum_iterations: 4000,
        linear_iterations: 100,
        penalty_pa_s: 2.0e5,
        divergence_penalty_pa_s: 2.0e6,
    }
}

pub fn fixture(control: Control, p: Parameters) -> Result<RemappedPatch, MechanicsError> {
    let [nx, ny, nz] = p.shape;
    if nx < 2
        || ny < 2
        || nz == 0
        || !p.width_m.is_finite()
        || p.width_m <= 0.0
        || !p.relief_m.is_finite()
        || p.relief_m.abs() >= p.gap_m
    {
        return Err(MechanicsError::InvalidMesh);
    }
    let solid = matches!(control, Control::SolidBulk | Control::ThinFullyCoherent);
    if !solid && (nx < 8 * p.seed_cycles[0].max(1) || ny < 8 * p.seed_cycles[1].max(1)) {
        return Err(MechanicsError::RefineMesh);
    }
    let depth = if control == Control::ThinFullyCoherent {
        p.thin_film_m
    } else {
        p.gap_m
    };
    let index = |x: usize, y: usize, z: usize| (z * (ny + 1) + y) * (nx + 1) + x;
    let mut patch = RemappedPatch {
        generation: 0,
        surface_tension_n_m: p.surface_tension_n_m,
        ..RemappedPatch::default()
    };
    for z in 0..=nz {
        for y in 0..=ny {
            for x in 0..=nx {
                let rx = x as f64 / nx as f64;
                let ry = y as f64 / ny as f64;
                let relief = p.relief_m
                    * (std::f64::consts::PI * p.seed_cycles[0] as f64 * rx)
                        .sin()
                        .powi(2)
                    * (std::f64::consts::PI * p.seed_cycles[1] as f64 * ry)
                        .sin()
                        .powi(2);
                let height = depth + relief;
                let fixed = solid || z == 0 || x == 0 || y == 0 || x == nx || y == ny;
                patch.vertices.push(Vertex {
                    position_m: [
                        p.width_m * rx,
                        p.width_m * ry,
                        height * z as f64 / nz as f64,
                    ],
                    fixed: [fixed; 3],
                });
            }
        }
    }
    let mut cells = Vec::new();
    for z in 0..nz {
        for y in 0..ny {
            for x in 0..nx {
                let a = index(x, y, z);
                let b = index(x + 1, y, z);
                let c = index(x + 1, y + 1, z);
                let d = index(x, y + 1, z);
                let e = index(x, y, z + 1);
                let f = index(x + 1, y, z + 1);
                let g = index(x + 1, y + 1, z + 1);
                let h = index(x, y + 1, z + 1);
                for vertices in [
                    [a, b, c, g],
                    [a, c, d, g],
                    [a, d, h, g],
                    [a, h, e, g],
                    [a, e, f, g],
                    [a, f, b, g],
                ] {
                    cells.push(FlowCell {
                        vertices,
                        material_cell: ((z * ny + y) * nx + x) as u32,
                        eta_pa_s: p.eta_pa_s,
                        yield_pa: match control {
                            Control::NoCompression
                            | Control::Tension
                            | Control::SubcriticalCompression => 0.0,
                            Control::BelowYield => p.arrest_yield_pa,
                            _ => p.yield_pa,
                        },
                    });
                }
            }
        }
    }
    let mut faces = BTreeMap::<[usize; 3], usize>::new();
    for c in &cells {
        for opposite in 0..4 {
            let mut v: Vec<_> = (0..4)
                .filter(|&j| j != opposite)
                .map(|j| c.vertices[j])
                .collect();
            v.sort();
            *faces.entry([v[0], v[1], v[2]]).or_default() += 1;
        }
    }
    let skin_thickness = match control {
        Control::ThinFullyCoherent => p.thin_film_m,
        Control::SolidBulk => p.gap_m + p.skin_m,
        _ => p.skin_m,
    };
    let tangent = integrate_skin(
        &[SkinCell {
            z_lo_m: -0.5 * skin_thickness,
            z_hi_m: 0.5 * skin_thickness,
            young_pa: p.young_pa,
            poisson: p.poisson,
        }],
        0.0,
    )?;
    let c = tangent.a[0][0] + tangent.a[0][1];
    let tension = match control {
        Control::NoCompression | Control::NoCoherentSkin => 0.0,
        Control::Tension => c * p.expansive_strain,
        Control::SubcriticalCompression => -0.9 * p.surface_tension_n_m,
        _ => -c * p.expansive_strain,
    };
    for (v, n) in faces {
        if n == 1 {
            let z: Vec<_> = v.iter().map(|&i| i / ((nx + 1) * (ny + 1))).collect();
            let kind = if z.iter().all(|&z| z == nz) {
                BoundaryKind::Exposed
            } else if z.iter().all(|&z| z == 0) {
                BoundaryKind::Support
            } else {
                BoundaryKind::Wall
            };
            if kind == BoundaryKind::Exposed && control != Control::NoCoherentSkin {
                patch.skins.push(PlateTriangle {
                    vertices: v,
                    packet_id: 1,
                    material_cell: (nx * ny * nz + patch.skins.len()) as u32,
                    thickness_m: skin_thickness,
                    tangent,
                    history: PlateHistory {
                        tension_n_m: [tension, tension, 0.0],
                        moment_n: [0.0; 3],
                    },
                    chemical_force_increment_n_m: [0.0; 3],
                    chemical_moment_increment_n: [0.0; 3],
                    relaxation_s: f64::INFINITY,
                    exposed: true,
                });
            }
            if !solid {
                patch.boundary.push(BoundaryFace { vertices: v, kind });
            }
        }
    }
    if !solid {
        patch.cells = cells;
    }
    patch.bending_modes = initialize_bending_modes(patch.view())?;
    Ok(patch)
}

pub fn initialize_bending_modes(patch: PatchView<'_>) -> Result<Vec<BendingMode>, MechanicsError> {
    let mut physical = BTreeMap::<(usize, usize), BTreeMap<[usize; 3], (f64, f64)>>::new();
    let mut edges = std::collections::BTreeSet::new();
    for s in patch.skins {
        let t = super::variational::triangle(s.vertices, patch)?;
        for opposite in 0..3 {
            let a = s.vertices[(opposite + 1) % 3].min(s.vertices[(opposite + 2) % 3]);
            let b = s.vertices[(opposite + 1) % 3].max(s.vertices[(opposite + 2) % 3]);
            let pa = patch.vertices[a].position_m;
            let pb = patch.vertices[b].position_m;
            let length = (pb[0] - pa[0]).hypot(pb[1] - pa[1]);
            let slope = (t.slope[0] * (pb[1] - pa[1]) - t.slope[1] * (pb[0] - pa[0])) / length;
            let mut triangle = s.vertices;
            triangle.sort();
            physical
                .entry((a, b))
                .or_default()
                .insert(triangle, (t.area * slope, t.area));
            edges.insert((s.packet_id, a, b));
        }
    }
    Ok(edges
        .into_iter()
        .map(|(packet_id, a, b)| {
            let (slope, area) = physical[&(a, b)]
                .values()
                .fold((0.0, 0.0), |(s, a), (ds, da)| (s + ds, a + da));
            BendingMode {
                packet_id,
                vertices: [a, b],
                normal_slope: slope / area,
            }
        })
        .collect())
}

#[derive(Default)]
pub struct UniformFixtureRemap;

impl ConservativeRemap for UniformFixtureRemap {
    fn remap(
        &mut self,
        source: PatchView<'_>,
        trial: &MechanicsTrial,
    ) -> Result<RemappedPatch, MechanicsError> {
        let mut result = RemappedPatch {
            generation: source.generation,
            vertices: source.vertices.to_vec(),
            cells: source.cells.to_vec(),
            boundary: source.boundary.to_vec(),
            skins: source.skins.to_vec(),
            bending_modes: trial.bending_modes.clone(),
            surface_tension_n_m: source.surface_tension_n_m,
        };
        let mut slopes = vec![[0.0; 2]; source.vertices.len()];
        let mut weights = vec![0.0; source.vertices.len()];
        let mut seen = std::collections::BTreeSet::new();
        for v in source.skins.iter().map(|s| s.vertices).chain(
            source
                .boundary
                .iter()
                .filter(|b| b.kind == BoundaryKind::Exposed)
                .map(|b| b.vertices),
        ) {
            let mut k = v;
            k.sort();
            if !seen.insert(k) {
                continue;
            }
            let t = super::variational::triangle(v, source)?;
            for &i in &v {
                for a in 0..2 {
                    slopes[i][a] += t.area * t.slope[a];
                }
                weights[i] += t.area;
            }
        }
        for i in 0..result.vertices.len() {
            if weights[i] == 0.0 {
                continue;
            }
            let d = trial.vertex_displacement_m[i];
            result.vertices[i].position_m[2] +=
                d[2] - (slopes[i][0] * d[0] + slopes[i][1] * d[1]) / weights[i];
        }
        let areas: Result<Vec<_>, _> = source
            .skins
            .iter()
            .map(|s| super::variational::triangle(s.vertices, source).map(|t| t.area))
            .collect();
        let areas = areas?;
        let mut volumes: Vec<_> = source
            .skins
            .iter()
            .zip(&areas)
            .map(|(s, a)| s.thickness_m * a)
            .collect();
        let old_volumes = volumes.clone();
        let mut forces: Vec<Tensor> = trial
            .history
            .iter()
            .zip(&areas)
            .map(|(h, a)| h.tension_n_m.map(|v| v * a))
            .collect();
        let mut moments: Vec<Tensor> = trial
            .history
            .iter()
            .zip(&areas)
            .map(|(h, a)| h.moment_n.map(|v| v * a))
            .collect();
        let mut touched = vec![false; source.skins.len()];
        for f in &trial.skin_transfers {
            if f.volume_m3 == 0.0 {
                continue;
            }
            touched[f.low_skin] = true;
            touched[f.high_skin] = true;
            let donor = if f.volume_m3 >= 0.0 {
                f.low_skin
            } else {
                f.high_skin
            };
            volumes[f.low_skin] -= f.volume_m3;
            volumes[f.high_skin] += f.volume_m3;
            for a in 0..3 {
                let force = f.volume_m3 * trial.history[donor].tension_n_m[a]
                    / source.skins[donor].thickness_m;
                let moment = f.volume_m3 * trial.history[donor].moment_n[a]
                    / source.skins[donor].thickness_m;
                forces[f.low_skin][a] -= force;
                forces[f.high_skin][a] += force;
                moments[f.low_skin][a] -= moment;
                moments[f.high_skin][a] += moment;
            }
        }
        for i in 0..result.skins.len() {
            if !touched[i] {
                result.skins[i].history = trial.history[i];
                continue;
            }
            if volumes[i] <= 0.0 {
                return Err(MechanicsError::InvalidMaterial);
            }
            let s = &mut result.skins[i];
            let ratio = volumes[i] / old_volumes[i];
            s.thickness_m = volumes[i] / areas[i];
            s.history = PlateHistory {
                tension_n_m: forces[i].map(|v| v / areas[i]),
                moment_n: moments[i].map(|v| v / areas[i]),
            };
            for a in 0..3 {
                for b in 0..3 {
                    s.tangent.a[a][b] *= ratio;
                    s.tangent.b[a][b] *= ratio * ratio;
                    s.tangent.d[a][b] *= ratio.powi(3);
                }
            }
        }
        Ok(result)
    }
}
