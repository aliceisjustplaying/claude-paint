use std::collections::BTreeMap;

#[path = "mechanics/controls.rs"]
pub mod controls;
#[path = "mechanics/variational.rs"]
mod variational;

#[cfg(test)]
#[path = "mechanics/tests.rs"]
mod tests;

pub type Tensor = [f64; 3];
pub type Matrix = [[f64; 3]; 3];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SkinCell {
    pub z_lo_m: f64,
    pub z_hi_m: f64,
    pub young_pa: f64,
    pub poisson: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PlateTangent {
    pub a: Matrix,
    pub b: Matrix,
    pub d: Matrix,
    pub z_ref_m: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PlateHistory {
    pub tension_n_m: Tensor,
    pub moment_n: Tensor,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub position_m: [f64; 3],
    pub fixed: [bool; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum VelocityDof {
    Translation { vertex: usize, component: usize },
    NormalSlope { vertices: [usize; 2] },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VelocityTerm {
    pub dof: VelocityDof,
    pub coefficient: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VertexConstraint {
    pub vertex: usize,
    pub components: [Vec<VelocityTerm>; 3],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlowCell {
    pub vertices: [usize; 4],
    pub material_cell: u32,
    pub eta_pa_s: f64,
    pub yield_pa: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundaryKind {
    Support,
    Wall,
    Exposed,
    Skin,
    Artificial,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundaryFace {
    pub vertices: [usize; 3],
    pub kind: BoundaryKind,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SkinFace {
    pub low_cell: u32,
    pub high_cell: Option<u32>,
    pub velocity_vertices: [usize; 2],
    pub thickness_m: [f64; 2],
    pub normal_xy: [f64; 2],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SkinMaterial {
    pub cell: u32,
    pub volume_m3: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlateTriangle {
    pub vertices: [usize; 3],
    pub packet_id: u64,
    pub material_cell: u32,
    pub thickness_m: f64,
    pub tangent: PlateTangent,
    pub history: PlateHistory,
    pub chemical_force_increment_n_m: Tensor,
    pub chemical_moment_increment_n: Tensor,
    pub relaxation_s: f64,
    pub exposed: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct PatchView<'a> {
    pub generation: u64,
    pub vertices: &'a [Vertex],
    pub cells: &'a [FlowCell],
    pub boundary: &'a [BoundaryFace],
    pub skins: &'a [PlateTriangle],
    pub bending_modes: &'a [BendingMode],
    pub constraints: &'a [VertexConstraint],
    pub collar_vertices: &'a [usize],
    pub skin_faces: &'a [SkinFace],
    pub skin_material: &'a [SkinMaterial],
    pub exposed_surfaces: &'a [[usize; 3]],
    pub support_surfaces: &'a [[usize; 3]],
    pub surface_tension_n_m: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BendingMode {
    pub packet_id: u64,
    pub vertices: [usize; 2],
    pub normal_slope: f64,
}

#[derive(Clone, Debug, Default)]
pub struct RemappedPatch {
    pub generation: u64,
    pub vertices: Vec<Vertex>,
    pub cells: Vec<FlowCell>,
    pub boundary: Vec<BoundaryFace>,
    pub skins: Vec<PlateTriangle>,
    pub bending_modes: Vec<BendingMode>,
    pub constraints: Vec<VertexConstraint>,
    pub collar_vertices: Vec<usize>,
    pub skin_faces: Vec<SkinFace>,
    pub skin_material: Vec<SkinMaterial>,
    pub exposed_surfaces: Vec<[usize; 3]>,
    pub support_surfaces: Vec<[usize; 3]>,
    pub surface_tension_n_m: f64,
}

impl RemappedPatch {
    pub fn view(&self) -> PatchView<'_> {
        PatchView {
            generation: self.generation,
            vertices: &self.vertices,
            cells: &self.cells,
            boundary: &self.boundary,
            skins: &self.skins,
            bending_modes: &self.bending_modes,
            constraints: &self.constraints,
            collar_vertices: &self.collar_vertices,
            skin_faces: &self.skin_faces,
            skin_material: &self.skin_material,
            exposed_surfaces: &self.exposed_surfaces,
            support_surfaces: &self.support_surfaces,
            surface_tension_n_m: self.surface_tension_n_m,
        }
    }
}

pub trait ConservativeRemap {
    fn remap(
        &mut self,
        source: PatchView<'_>,
        trial: &MechanicsTrial,
    ) -> Result<RemappedPatch, MechanicsError>;
}

pub fn coherent_trace_constraints(
    patch: PatchView<'_>,
    triangles: &[[usize; 3]],
    samples: &[(usize, usize)],
) -> Result<Vec<VertexConstraint>, MechanicsError> {
    variational::coherent_trace_constraints(patch, triangles, samples)
}

#[derive(Clone, Copy, Debug)]
pub struct StepInterval {
    pub start_s: f64,
    pub end_s: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct MechanicsAccuracy {
    pub divergence_error_s_inv: f64,
    pub traction_error_pa: f64,
    pub relative_time_error: f64,
    pub absolute_displacement_error_m: f64,
    pub maximum_resolved_slope: f64,
    pub maximum_resolved_strain: f64,
    pub maximum_donor_fraction: f64,
    pub quadrature_order: usize,
    pub maximum_iterations: usize,
    pub linear_iterations: usize,
    pub penalty_pa_s: f64,
    pub divergence_penalty_pa_s: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MechanicsError {
    InvalidMesh,
    InvalidMaterial,
    NonFiniteState,
    ExpandDomain,
    RefineMesh,
    SmallSlopeModelInvalid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubdivisionReason {
    TimeAccuracy,
    DonorDepletion,
    NonlinearConvergence,
    YieldResidual,
    Nonpenetration,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FaceTransfer {
    pub face: usize,
    pub low_cell: u32,
    pub high_cell: u32,
    pub volume_m3: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SkinTransfer {
    pub face: usize,
    pub low_cell: u32,
    pub high_cell: u32,
    pub volume_m3: f64,
}

#[derive(Clone, Debug, Default)]
pub struct StepDiagnostics {
    pub energy_increment_j: f64,
    pub viscous_dissipation_j: f64,
    pub plastic_dissipation_j: f64,
    pub divergence_error_s_inv: f64,
    pub yield_residual_pa: f64,
    pub stationarity_residual_pa: f64,
    pub maximum_slope: f64,
    pub maximum_strain: f64,
    pub maximum_donor_fraction: f64,
    pub estimated_displacement_error_m: f64,
    pub estimated_force_error_n_m: f64,
    pub estimated_moment_error_n: f64,
    pub kinematic_error_m: f64,
    pub volume_geometry_error_m: f64,
    pub pointwise_divergence_error_s_inv: f64,
    pub collar_traction_pa: f64,
    pub iterations: usize,
    pub newton_iterations: usize,
    pub krylov_iterations: usize,
    pub line_search_backtracks: usize,
    pub preconditioner_fallbacks: usize,
    pub inner_stationarity_residual_pa: f64,
    pub arrested: bool,
}

#[derive(Clone, Debug)]
pub struct MechanicsTrial {
    pub input_generation: u64,
    pub duration_s: f64,
    pub transfers: Vec<FaceTransfer>,
    pub skin_transfers: Vec<SkinTransfer>,
    pub history: Vec<PlateHistory>,
    pub bending_modes: Vec<BendingMode>,
    pub vertex_displacement_m: Vec<[f64; 3]>,
    pub end_cell_volumes_m3: Vec<(u32, f64)>,
    pub end_skin_volumes_m3: Vec<(u32, f64)>,
    pub diagnostics: StepDiagnostics,
}

#[derive(Clone, Debug)]
pub struct AcceptedStep {
    pub trial: MechanicsTrial,
    pub remapped: RemappedPatch,
}

#[derive(Clone, Debug)]
pub enum StepAttempt {
    Accepted(AcceptedStep),
    Subdivide(SubdivisionReason),
}

#[derive(Default)]
pub struct MechanicsScratch {
    work: variational::Work,
    comparison: variational::Work,
    pub last_diagnostics: Option<StepDiagnostics>,
}

pub fn integrate_skin(cells: &[SkinCell], z_ref_m: f64) -> Result<PlateTangent, MechanicsError> {
    if !z_ref_m.is_finite() {
        return Err(MechanicsError::InvalidMaterial);
    }
    let mut result = PlateTangent {
        z_ref_m,
        ..PlateTangent::default()
    };
    let mut previous = f64::NEG_INFINITY;
    for cell in cells {
        let h = cell.z_hi_m - cell.z_lo_m;
        if ![cell.z_lo_m, cell.z_hi_m, cell.young_pa, cell.poisson]
            .iter()
            .all(|v| v.is_finite())
            || h <= 0.0
            || cell.z_lo_m < previous
            || cell.young_pa < 0.0
            || cell.poisson <= -1.0
            || cell.poisson >= 0.5
        {
            return Err(MechanicsError::InvalidMaterial);
        }
        previous = cell.z_hi_m;
        let z = 0.5 * (cell.z_hi_m + cell.z_lo_m) - z_ref_m;
        let q = cell.young_pa / (1.0 - cell.poisson * cell.poisson);
        let matrix = [
            [q, q * cell.poisson, 0.0],
            [q * cell.poisson, q, 0.0],
            [0.0, 0.0, 0.5 * q * (1.0 - cell.poisson)],
        ];
        for i in 0..3 {
            for j in 0..3 {
                result.a[i][j] += h * matrix[i][j];
                result.b[i][j] += h * z * matrix[i][j];
                result.d[i][j] += (h * z * z + h * h * h / 12.0) * matrix[i][j];
            }
        }
    }
    Ok(result)
}

pub fn advance_history(
    tangent: &PlateTangent,
    retained: PlateHistory,
    chemical_force_increment_n_m: Tensor,
    chemical_moment_increment_n: Tensor,
    metric_increment: Tensor,
    curvature_increment: Tensor,
    relaxation_factor: f64,
) -> PlateHistory {
    let mut out = retained;
    for i in 0..3 {
        out.tension_n_m[i] =
            relaxation_factor * retained.tension_n_m[i] - chemical_force_increment_n_m[i];
        out.moment_n[i] = relaxation_factor * retained.moment_n[i] - chemical_moment_increment_n[i];
        for j in 0..3 {
            out.tension_n_m[i] +=
                tangent.a[i][j] * metric_increment[j] + tangent.b[i][j] * curvature_increment[j];
            out.moment_n[i] +=
                tangent.b[j][i] * metric_increment[j] + tangent.d[i][j] * curvature_increment[j];
        }
    }
    out
}

pub fn newtonian_normal_mobility(
    q_m_inv: f64,
    depth_m: f64,
    eta_pa_s: f64,
) -> Result<f64, MechanicsError> {
    if ![q_m_inv, depth_m, eta_pa_s].iter().all(|v| v.is_finite())
        || q_m_inv < 0.0
        || depth_m <= 0.0
        || eta_pa_s <= 0.0
    {
        return Err(MechanicsError::InvalidMaterial);
    }
    let a = q_m_inv * depth_m;
    if a == 0.0 {
        return Ok(0.0);
    }
    if a < 0.01 {
        let a2 = a * a;
        return Ok(depth_m * a2 / (3.0 * eta_pa_s) * (1.0 - 9.0 * a2 / 5.0 + 23.0 * a2 * a2 / 7.0));
    }
    let e = (-2.0 * a).exp();
    Ok((1.0 - e * e - 4.0 * a * e)
        / (2.0 * eta_pa_s * q_m_inv * (1.0 + e * e + (4.0 * a * a + 2.0) * e)))
}

pub fn newtonian_surface_mobility(
    q_m_inv: f64,
    depth_m: f64,
    eta_pa_s: f64,
) -> Result<[[f64; 2]; 2], MechanicsError> {
    let normal = newtonian_normal_mobility(q_m_inv, depth_m, eta_pa_s)?;
    let a = q_m_inv * depth_m;
    if a < 0.01 {
        let a2 = a * a;
        let longitudinal = depth_m / eta_pa_s * (1.0 - 5.0 * a2 / 3.0 + 46.0 * a2 * a2 / 15.0);
        let cross = -q_m_inv * depth_m * depth_m / (2.0 * eta_pa_s)
            * (1.0 - 2.0 * a2 + 11.0 * a2 * a2 / 3.0);
        return Ok([[longitudinal, cross], [cross, normal]]);
    }
    let e = (-2.0 * a).exp();
    let factor = 1.0 / (2.0 * eta_pa_s * q_m_inv * (1.0 + e * e + (4.0 * a * a + 2.0) * e));
    let longitudinal = factor * (1.0 - e * e + 4.0 * a * e);
    let cross = -factor * 4.0 * a * a * e;
    Ok([[longitudinal, cross], [cross, normal]])
}

pub fn coupled_linear_growth_oracle(
    q_m_inv: f64,
    depth_m: f64,
    eta_pa_s: f64,
    tangent: &PlateTangent,
    tension_n_m: f64,
    surface_tension_n_m: f64,
) -> Result<[f64; 2], MechanicsError> {
    if ![
        tangent.a[0][0],
        tangent.b[0][0],
        tangent.d[0][0],
        tension_n_m,
        surface_tension_n_m,
    ]
    .iter()
    .all(|v| v.is_finite())
        || tangent.a[0][0] <= 0.0
        || tangent.d[0][0] <= 0.0
        || tangent.a[0][0] * tangent.d[0][0] <= tangent.b[0][0].powi(2)
        || surface_tension_n_m < 0.0
    {
        return Err(MechanicsError::InvalidMaterial);
    }
    let m = newtonian_surface_mobility(q_m_inv, depth_m, eta_pa_s)?;
    let q2 = q_m_inv * q_m_inv;
    let h = [
        [tangent.a[0][0] * q2, -tangent.b[0][0] * q2 * q_m_inv],
        [
            -tangent.b[0][0] * q2 * q_m_inv,
            tangent.d[0][0] * q2 * q2 + (tension_n_m + surface_tension_n_m) * q2,
        ],
    ];
    let trace = -(m[0][0] * h[0][0] + 2.0 * m[0][1] * h[0][1] + m[1][1] * h[1][1]);
    let determinant =
        (m[0][0] * m[1][1] - m[0][1] * m[1][0]) * (h[0][0] * h[1][1] - h[0][1] * h[1][0]);
    let root = (trace * trace - 4.0 * determinant).max(0.0).sqrt();
    let mut rates = [0.5 * (trace + root), 0.5 * (trace - root)];
    if trace < 0.0 && rates[1] != 0.0 {
        rates[0] = determinant / rates[1];
    }
    if trace > 0.0 && rates[0] != 0.0 {
        rates[1] = determinant / rates[0];
    }
    if rates.iter().any(|v| !v.is_finite()) {
        return Err(MechanicsError::NonFiniteState);
    }
    Ok(rates)
}

pub fn bingham_parallel_flux(
    gradient_pa_m: f64,
    depth_m: f64,
    eta_pa_s: f64,
    yield_pa: f64,
    fixed_top: bool,
) -> Result<f64, MechanicsError> {
    if ![gradient_pa_m, depth_m, eta_pa_s, yield_pa]
        .iter()
        .all(|v| v.is_finite())
        || depth_m <= 0.0
        || eta_pa_s <= 0.0
        || yield_pa < 0.0
    {
        return Err(MechanicsError::InvalidMaterial);
    }
    if gradient_pa_m == 0.0 {
        return Ok(0.0);
    }
    let factor = if fixed_top { 2.0 } else { 1.0 };
    let y = factor * yield_pa / (depth_m * gradient_pa_m.abs());
    if y >= 1.0 {
        return Ok(0.0);
    }
    Ok(
        -depth_m.powi(3) * gradient_pa_m / (3.0 * eta_pa_s * factor * factor)
            * (1.0 - 1.5 * y + 0.5 * y.powi(3)),
    )
}

pub fn traction_free_linear_growth_oracle(
    q_m_inv: f64,
    depth_m: f64,
    eta_pa_s: f64,
    bending_n_m: f64,
    tension_n_m: f64,
    surface_tension_n_m: f64,
) -> Result<f64, MechanicsError> {
    if ![bending_n_m, tension_n_m, surface_tension_n_m]
        .iter()
        .all(|v| v.is_finite())
        || bending_n_m < 0.0
        || surface_tension_n_m < 0.0
    {
        return Err(MechanicsError::InvalidMaterial);
    }
    let q2 = q_m_inv * q_m_inv;
    Ok(-newtonian_normal_mobility(q_m_inv, depth_m, eta_pa_s)?
        * (bending_n_m * q2 * q2 + (tension_n_m + surface_tension_n_m) * q2))
}

pub fn attempt_step(
    patch: PatchView<'_>,
    interval: StepInterval,
    accuracy: &MechanicsAccuracy,
    scratch: &mut MechanicsScratch,
    remap: &mut impl ConservativeRemap,
) -> Result<StepAttempt, MechanicsError> {
    scratch.work.last_diagnostics = None;
    scratch.comparison.last_diagnostics = None;
    let result = variational::attempt_step(
        patch,
        interval,
        accuracy,
        &mut scratch.work,
        &mut scratch.comparison,
        remap,
    );
    scratch.last_diagnostics = match &result {
        Ok(StepAttempt::Accepted(step)) => Some(step.trial.diagnostics.clone()),
        _ => scratch
            .comparison
            .last_diagnostics
            .clone()
            .or_else(|| scratch.work.last_diagnostics.clone()),
    };
    result
}

fn volume_totals(patch: PatchView<'_>, volumes: &[f64]) -> BTreeMap<u32, f64> {
    let mut totals = BTreeMap::new();
    for (cell, volume) in patch.cells.iter().zip(volumes) {
        *totals.entry(cell.material_cell).or_default() += volume;
    }
    totals
}
