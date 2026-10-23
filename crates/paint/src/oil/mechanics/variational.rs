use super::*;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

type Row = Vec<(usize, f64)>;
const ROOT_TWO: f64 = std::f64::consts::SQRT_2;

#[derive(Clone, Default)]
struct SparseMatrix {
    offsets: Vec<usize>,
    columns: Vec<usize>,
    values: Vec<f64>,
    diagonal: Vec<usize>,
    factor: Vec<f64>,
    factored: bool,
}

impl SparseMatrix {
    fn row_value(&self, row: usize, x: &[f64]) -> f64 {
        (self.offsets[row]..self.offsets[row + 1])
            .map(|p| self.values[p] * x[self.columns[p]])
            .sum()
    }

    fn apply(&self, x: &[f64], out: &mut [f64]) {
        for (i, value) in out.iter_mut().enumerate() {
            *value = self.row_value(i, x);
        }
    }

    fn add(&mut self, i: usize, j: usize, value: f64) {
        if value == 0.0 {
            return;
        }
        let start = self.offsets[i];
        let end = self.offsets[i + 1];
        let position = self.columns[start..end]
            .binary_search(&j)
            .expect("assembled mechanics sparsity");
        self.values[start + position] += value;
    }

    fn factor(&mut self) {
        self.factor.resize(self.values.len(), 0.0);
        self.factor.fill(0.0);
        self.factored = false;
        for i in 0..self.diagonal.len() {
            for p in self.offsets[i]..self.diagonal[i] {
                let j = self.columns[p];
                let mut value = self.values[p];
                let mut a = self.offsets[i];
                let mut b = self.offsets[j];
                while a < p && b < self.diagonal[j] {
                    let ca = self.columns[a];
                    let cb = self.columns[b];
                    if ca == cb {
                        value -= self.factor[a] * self.factor[b];
                        a += 1;
                        b += 1;
                    } else if ca < cb {
                        a += 1;
                    } else {
                        b += 1;
                    }
                }
                self.factor[p] = value / self.factor[self.diagonal[j]];
            }
            let value = self.values[self.diagonal[i]]
                - (self.offsets[i]..self.diagonal[i])
                    .map(|p| self.factor[p] * self.factor[p])
                    .sum::<f64>();
            if !value.is_finite() || value <= 0.0 {
                return;
            }
            self.factor[self.diagonal[i]] = value.sqrt();
        }
        self.factored = true;
    }

    fn precondition(&self, rhs: &[f64], out: &mut [f64]) {
        if !self.factored {
            for i in 0..out.len() {
                out[i] = rhs[i] / self.values[self.diagonal[i]].abs().max(f64::MIN_POSITIVE);
            }
            return;
        }
        for i in 0..out.len() {
            let value = rhs[i]
                - (self.offsets[i]..self.diagonal[i])
                    .map(|p| self.factor[p] * out[self.columns[p]])
                    .sum::<f64>();
            out[i] = value / self.factor[self.diagonal[i]];
        }
        for i in (0..out.len()).rev() {
            out[i] /= self.factor[self.diagonal[i]];
            for p in self.offsets[i]..self.diagonal[i] {
                out[self.columns[p]] -= self.factor[p] * out[i];
            }
        }
    }
}

#[derive(Clone, Default)]
struct Face {
    vertices: [usize; 3],
    adjacent: Vec<(usize, usize)>,
    normal: [f64; 3],
    area: f64,
    bubble: Option<usize>,
    kind: Option<BoundaryKind>,
}

#[derive(Clone)]
struct Quadrature {
    rows: [Row; 6],
    divergence: Row,
    weight: f64,
    eta: f64,
    yield_pa: f64,
}

#[derive(Clone)]
pub(super) struct Triangle {
    vertices: [usize; 3],
    gradient: [[f64; 2]; 3],
    pub(super) area: f64,
    pub(super) slope: [f64; 2],
    curvature: [Row; 3],
}

#[derive(Default)]
pub(super) struct Work {
    faces: Vec<Face>,
    quadrature: Vec<Quadrature>,
    quadrature_ranges: Vec<std::ops::Range<usize>>,
    divergence: Vec<Row>,
    volumes: Vec<f64>,
    skins: Vec<Triangle>,
    exposed: Vec<Triangle>,
    fixed: Vec<bool>,
    area_scale: Vec<f64>,
    velocity: Vec<f64>,
    split: Vec<[f64; 6]>,
    dual: Vec<[f64; 6]>,
    rates: Vec<[f64; 6]>,
    pressure: Vec<f64>,
    gradient: Vec<f64>,
    direction: Vec<f64>,
    residual: Vec<f64>,
    conjugate: Vec<f64>,
    product: Vec<f64>,
    trial: Vec<f64>,
    rotations: Vec<(BendingMode, usize, f64)>,
    fluid: SparseMatrix,
    newton: SparseMatrix,
    rhs: Vec<f64>,
    constraint_force: Vec<f64>,
    split_force: Vec<f64>,
    preconditioned: Vec<f64>,
    newton_steps: usize,
    cg_steps: usize,
    backtracks: usize,
    preconditioner_fallbacks: usize,
    inner_residual_pa: f64,
    divergence_ratio: f64,
    lift: Vec<Row>,
    pub(super) last_diagnostics: Option<StepDiagnostics>,
}

enum TrialAttempt {
    Solved(MechanicsTrial),
    Subdivide(SubdivisionReason),
}

impl Work {
    fn physical_velocity<'a>(&self, velocity: &'a [f64]) -> Cow<'a, [f64]> {
        if self.lift.is_empty() {
            Cow::Borrowed(velocity)
        } else {
            Cow::Owned(self.lift.iter().map(|r| row_value(r, velocity)).collect())
        }
    }

    fn pull_row(&self, row: &Row) -> Row {
        if self.lift.is_empty() {
            return row.clone();
        }
        let mut result = Vec::new();
        for &(i, a) in row {
            for &(j, b) in &self.lift[i] {
                result.push((j, a * b));
            }
        }
        compress(&mut result);
        result
    }
}

fn prepare_constraints(patch: PatchView<'_>, w: &mut Work) -> Result<(), MechanicsError> {
    w.lift.clear();
    if patch.constraints.is_empty() {
        return Ok(());
    }
    let targets: BTreeSet<_> = patch.constraints.iter().map(|c| c.vertex).collect();
    if targets.len() != patch.constraints.len()
        || targets.iter().any(|&v| v >= patch.vertices.len())
    {
        return Err(MechanicsError::InvalidMesh);
    }
    w.lift = (0..w.velocity.len()).map(|i| vec![(i, 1.0)]).collect();
    let rotations: BTreeMap<_, _> = w
        .rotations
        .iter()
        .map(|(mode, i, length)| {
            (
                (
                    mode.vertices[0].min(mode.vertices[1]),
                    mode.vertices[0].max(mode.vertices[1]),
                ),
                (*i, *length),
            )
        })
        .collect();
    for c in patch.constraints {
        for a in 0..3 {
            let mut row = Vec::new();
            for term in &c.components[a] {
                if !term.coefficient.is_finite() {
                    return Err(MechanicsError::InvalidMesh);
                }
                let (index, coefficient) = match term.dof {
                    VelocityDof::Translation { vertex, component } => {
                        if vertex >= patch.vertices.len()
                            || component >= 3
                            || targets.contains(&vertex)
                        {
                            return Err(MechanicsError::InvalidMesh);
                        }
                        (3 * vertex + component, term.coefficient)
                    }
                    VelocityDof::NormalSlope { vertices: [a, b] } => {
                        let &(index, length) = rotations
                            .get(&(a.min(b), a.max(b)))
                            .ok_or(MechanicsError::InvalidMesh)?;
                        (index, term.coefficient / length)
                    }
                };
                if coefficient != 0.0 {
                    row.push((index, coefficient));
                }
            }
            compress(&mut row);
            if patch.vertices[c.vertex].fixed[a] && row.iter().any(|&(j, _)| !w.fixed[j]) {
                return Err(MechanicsError::InvalidMesh);
            }
            let slave = 3 * c.vertex + a;
            let area = w.area_scale[slave];
            w.area_scale[slave] = 0.0;
            for &(j, b) in &row {
                w.area_scale[j] += area * b.abs();
            }
            w.fixed[slave] = true;
            w.lift[slave] = row;
        }
    }
    for f in patch.skin_faces {
        if f.velocity_vertices
            .iter()
            .any(|&v| v >= patch.vertices.len())
            || f.thickness_m.iter().any(|h| !h.is_finite() || *h < 0.0)
            || f.normal_xy.iter().any(|x| !x.is_finite())
            || f.normal_xy[0].hypot(f.normal_xy[1]) == 0.0
        {
            return Err(MechanicsError::InvalidMesh);
        }
    }
    if patch
        .skin_material
        .iter()
        .any(|s| !s.volume_m3.is_finite() || s.volume_m3 < 0.0)
        || patch
            .exposed_surfaces
            .iter()
            .flatten()
            .any(|&v| v >= patch.vertices.len())
    {
        return Err(MechanicsError::InvalidMesh);
    }
    for q in &mut w.quadrature {
        for row in q.rows.iter_mut().chain(std::iter::once(&mut q.divergence)) {
            let mut result = Vec::new();
            for &(i, a) in row.iter() {
                for &(j, b) in &w.lift[i] {
                    result.push((j, a * b));
                }
            }
            compress(&mut result);
            *row = result;
        }
    }
    for row in &mut w.divergence {
        let mut result = Vec::new();
        for &(i, a) in row.iter() {
            for &(j, b) in &w.lift[i] {
                result.push((j, a * b));
            }
        }
        compress(&mut result);
        *row = result;
    }
    Ok(())
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn row_value(row: &Row, x: &[f64]) -> f64 {
    row.iter().map(|&(i, a)| a * x[i]).sum()
}
fn scatter(row: &Row, scale: f64, x: &mut [f64]) {
    for &(i, a) in row {
        x[i] += a * scale;
    }
}
fn compress(row: &mut Row) {
    row.sort_by_key(|v| v.0);
    let mut n = 0;
    for k in 0..row.len() {
        let item = row[k];
        if n > 0 && row[n - 1].0 == item.0 {
            row[n - 1].1 += item.1;
        } else {
            row[n] = item;
            n += 1;
        }
    }
    row.truncate(n);
    row.retain(|&(_, a)| a != 0.0);
}
fn key(mut v: [usize; 3]) -> [usize; 3] {
    v.sort();
    v
}
fn plus(a: Tensor, b: Tensor) -> Tensor {
    std::array::from_fn(|i| a[i] + b[i])
}

fn invert(mut a: [[f64; 6]; 6]) -> Result<[[f64; 6]; 6], MechanicsError> {
    let mut inverse = [[0.0; 6]; 6];
    for i in 0..6 {
        inverse[i][i] = 1.0;
    }
    for k in 0..6 {
        let pivot = (k..6)
            .max_by(|&i, &j| a[i][k].abs().total_cmp(&a[j][k].abs()))
            .ok_or(MechanicsError::InvalidMesh)?;
        if a[pivot][k] == 0.0 || !a[pivot][k].is_finite() {
            return Err(MechanicsError::InvalidMesh);
        }
        a.swap(k, pivot);
        inverse.swap(k, pivot);
        let scale = a[k][k];
        for j in 0..6 {
            a[k][j] /= scale;
            inverse[k][j] /= scale;
        }
        for i in 0..6 {
            if i != k {
                let scale = a[i][k];
                for j in 0..6 {
                    a[i][j] -= scale * a[k][j];
                    inverse[i][j] -= scale * inverse[k][j];
                }
            }
        }
    }
    Ok(inverse)
}

fn elastic_strain(s: &PlateTriangle, h: PlateHistory) -> f64 {
    let scales: [f64; 6] = std::array::from_fn(|i| {
        if i < 3 {
            s.tangent.a[i][i].sqrt()
        } else {
            s.tangent.d[i - 3][i - 3].sqrt()
        }
    });
    let matrix = std::array::from_fn(|i| {
        std::array::from_fn(|j| {
            let value = match (i < 3, j < 3) {
                (true, true) => s.tangent.a[i][j],
                (true, false) => s.tangent.b[i][j - 3],
                (false, true) => s.tangent.b[j][i - 3],
                (false, false) => s.tangent.d[i - 3][j - 3],
            };
            value / (scales[i] * scales[j])
        })
    });
    let Ok(inverse) = invert(matrix) else {
        return f64::INFINITY;
    };
    let rhs: [f64; 6] = std::array::from_fn(|i| {
        if i < 3 {
            h.tension_n_m[i] / scales[i]
        } else {
            h.moment_n[i - 3] / scales[i]
        }
    });
    let strain: [f64; 6] = std::array::from_fn(|i| dot(&inverse[i], &rhs) / scales[i]);
    [-0.5, 0.5]
        .iter()
        .map(|z| {
            let e: Tensor = std::array::from_fn(|i| strain[i] + z * s.thickness_m * strain[i + 3]);
            (e[0] * e[0] + e[1] * e[1] + 0.5 * e[2] * e[2]).sqrt()
        })
        .fold(0.0, f64::max)
}

fn constant_operator(w: &Work) -> SparseMatrix {
    let n = w.velocity.len();
    let mut rows: Vec<BTreeMap<usize, f64>> = (0..n).map(|i| BTreeMap::from([(i, 0.0)])).collect();
    for (ci, range) in w.quadrature_ranges.iter().enumerate() {
        let mut indices: Vec<_> = w.quadrature[range.clone()]
            .iter()
            .flat_map(|q| q.rows.iter())
            .flat_map(|row| row.iter().map(|x| x.0))
            .collect();
        indices.extend(w.divergence[ci].iter().map(|x| x.0));
        indices.sort();
        indices.dedup();
        let count = indices.len();
        let mut local = vec![0.0; count * count];
        for q in &w.quadrature[range.clone()] {
            for row in &q.rows {
                let terms: Vec<_> = row
                    .iter()
                    .map(|&(i, a)| (indices.binary_search(&i).expect("local strain degree"), a))
                    .collect();
                for &(i, a) in &terms {
                    for &(j, b) in &terms {
                        local[i * count + j] += q.weight * a * b;
                    }
                }
            }
        }
        let terms: Vec<_> = w.divergence[ci]
            .iter()
            .map(|&(i, a)| (indices.binary_search(&i).expect("local volume degree"), a))
            .collect();
        for &(i, a) in &terms {
            for &(j, b) in &terms {
                local[i * count + j] += w.divergence_ratio * w.volumes[ci] * a * b;
            }
        }
        for i in 0..count {
            for j in 0..count {
                let v = local[i * count + j];
                if v != 0.0 {
                    *rows[indices[i]].entry(indices[j]).or_default() += v;
                }
            }
        }
    }
    for t in &w.skins {
        let mut indices: Vec<_> = t
            .vertices
            .iter()
            .flat_map(|&i| [3 * i, 3 * i + 1, 3 * i + 2])
            .collect();
        indices.extend(t.curvature.iter().flat_map(|r| r.iter().map(|x| x.0)));
        if !w.lift.is_empty() {
            indices = indices
                .iter()
                .flat_map(|&i| w.lift[i].iter().map(|x| x.0))
                .collect();
        }
        indices.sort();
        indices.dedup();
        for &i in &indices {
            for &j in &indices {
                rows[i].entry(j).or_default();
            }
        }
    }
    for t in &w.exposed {
        for &i in &t.vertices {
            for &j in &t.vertices {
                let ri = w.pull_row(&vec![(3 * i + 2, 1.0)]);
                let rj = w.pull_row(&vec![(3 * j + 2, 1.0)]);
                for &(a, _) in &ri {
                    for &(b, _) in &rj {
                        rows[a].entry(b).or_default();
                    }
                }
            }
        }
    }
    let mut matrix = SparseMatrix::default();
    matrix.offsets.push(0);
    for (i, row) in rows.into_iter().enumerate() {
        for (j, v) in row {
            if j == i {
                matrix.diagonal.push(matrix.values.len());
            }
            matrix.columns.push(j);
            matrix.values.push(v);
        }
        matrix.offsets.push(matrix.values.len());
    }
    matrix
}

fn update_newton(patch: PatchView<'_>, w: &mut Work, dt: f64, rho: f64) {
    for (a, b) in w.newton.values.iter_mut().zip(&w.fluid.values) {
        *a = dt * rho * b;
    }
    let physical = w.physical_velocity(&w.velocity);
    for (s, t) in patch.skins.iter().zip(&w.skins) {
        let (e, k, _, raw_jac) = strains(t, &physical, dt);
        let jac = raw_jac.map(|row| w.pull_row(&row));
        let curvature = t.curvature.clone().map(|row| w.pull_row(&row));
        for a in 0..3 {
            for b in 0..3 {
                for &(i, x) in &jac[a] {
                    for &(j, y) in &jac[b] {
                        w.newton.add(i, j, t.area * s.tangent.a[a][b] * x * y);
                    }
                }
                for &(i, x) in &jac[a] {
                    for &(j, y) in &curvature[b] {
                        let v = dt * t.area * s.tangent.b[a][b] * x * y;
                        w.newton.add(i, j, v);
                        w.newton.add(j, i, v);
                    }
                }
                for &(i, x) in &curvature[a] {
                    for &(j, y) in &curvature[b] {
                        w.newton
                            .add(i, j, dt * dt * t.area * s.tangent.d[a][b] * x * y);
                    }
                }
            }
        }
        let force = history(s, e, k, dt).tension_n_m;
        for i in 0..3 {
            for j in 0..3 {
                let [ix, iy] = t.gradient[i];
                let [jx, jy] = t.gradient[j];
                let value = dt
                    * dt
                    * t.area
                    * (force[0] * ix * jx + force[1] * iy * jy + force[2] * (ix * jy + iy * jx));
                let ri = w.pull_row(&vec![(3 * t.vertices[i] + 2, 1.0)]);
                let rj = w.pull_row(&vec![(3 * t.vertices[j] + 2, 1.0)]);
                for &(a, x) in &ri {
                    for &(b, y) in &rj {
                        w.newton.add(a, b, value * x * y);
                    }
                }
            }
        }
    }
    for t in &w.exposed {
        for i in 0..3 {
            for j in 0..3 {
                let value = dt
                    * dt
                    * t.area
                    * patch.surface_tension_n_m
                    * (t.gradient[i][0] * t.gradient[j][0] + t.gradient[i][1] * t.gradient[j][1]);
                let ri = w.pull_row(&vec![(3 * t.vertices[i] + 2, 1.0)]);
                let rj = w.pull_row(&vec![(3 * t.vertices[j] + 2, 1.0)]);
                for &(a, x) in &ri {
                    for &(b, y) in &rj {
                        w.newton.add(a, b, value * x * y);
                    }
                }
            }
        }
    }
    for i in 0..w.fixed.len() {
        for p in w.newton.offsets[i]..w.newton.offsets[i + 1] {
            let j = w.newton.columns[p];
            if w.fixed[i] || w.fixed[j] {
                w.newton.values[p] = if i == j { 1.0 } else { 0.0 };
            }
        }
    }
    w.newton.factor();
}

fn tetra_geometry(p: [[f64; 3]; 4]) -> Result<(f64, [[f64; 3]; 4]), MechanicsError> {
    let a = sub(p[1], p[0]);
    let b = sub(p[2], p[0]);
    let c = sub(p[3], p[0]);
    let det = dot3(a, cross(b, c));
    if !det.is_finite() || det <= 0.0 {
        return Err(MechanicsError::InvalidMesh);
    }
    let mut g = [[0.0; 3]; 4];
    for (i, v) in [cross(b, c), cross(c, a), cross(a, b)].iter().enumerate() {
        for j in 0..3 {
            g[i + 1][j] = v[j] / det;
            g[0][j] -= g[i + 1][j];
        }
    }
    Ok((det / 6.0, g))
}

pub(super) fn triangle(
    vertices: [usize; 3],
    patch: PatchView<'_>,
) -> Result<Triangle, MechanicsError> {
    let p = vertices.map(|i| patch.vertices[i].position_m);
    let det = (p[1][0] - p[0][0]) * (p[2][1] - p[0][1]) - (p[1][1] - p[0][1]) * (p[2][0] - p[0][0]);
    if !det.is_finite() || det == 0.0 {
        return Err(MechanicsError::InvalidMesh);
    }
    let gradient = [
        [(p[1][1] - p[2][1]) / det, (p[2][0] - p[1][0]) / det],
        [(p[2][1] - p[0][1]) / det, (p[0][0] - p[2][0]) / det],
        [(p[0][1] - p[1][1]) / det, (p[1][0] - p[0][0]) / det],
    ];
    let slope = std::array::from_fn(|a| (0..3).map(|j| gradient[j][a] * p[j][2]).sum());
    Ok(Triangle {
        vertices,
        gradient,
        area: 0.5 * det.abs(),
        slope,
        curvature: std::array::from_fn(|_| Vec::new()),
    })
}

fn morley_interpolant(
    patch: PatchView<'_>,
    vertices: [usize; 3],
) -> Result<([[f64; 6]; 6], f64, [[usize; 2]; 3]), MechanicsError> {
    if vertices.iter().any(|&i| i >= patch.vertices.len()) {
        return Err(MechanicsError::InvalidMesh);
    }
    let p = vertices.map(|v| patch.vertices[v].position_m);
    let length = triangle(vertices, patch)?.area.sqrt();
    let mut matrix = [[0.0; 6]; 6];
    let mut edges = [[0; 2]; 3];
    for j in 0..3 {
        let x = (p[j][0] - p[0][0]) / length;
        let y = (p[j][1] - p[0][1]) / length;
        matrix[j] = [1.0, x, y, 0.5 * x * x, x * y, 0.5 * y * y];
        let a = vertices[(j + 1) % 3].min(vertices[(j + 2) % 3]);
        let b = vertices[(j + 1) % 3].max(vertices[(j + 2) % 3]);
        edges[j] = [a, b];
        let pa = patch.vertices[a].position_m;
        let pb = patch.vertices[b].position_m;
        let edge_length = (pb[0] - pa[0]).hypot(pb[1] - pa[1]);
        let normal = [
            (pb[1] - pa[1]) / edge_length,
            -(pb[0] - pa[0]) / edge_length,
        ];
        let x = (0.5 * (pa[0] + pb[0]) - p[0][0]) / length;
        let y = (0.5 * (pa[1] + pb[1]) - p[0][1]) / length;
        let scale = edge_length / length;
        matrix[j + 3] = [
            0.0,
            scale * normal[0],
            scale * normal[1],
            scale * x * normal[0],
            scale * (y * normal[0] + x * normal[1]),
            scale * y * normal[1],
        ];
    }
    Ok((invert(matrix)?, length, edges))
}

pub(super) fn coherent_trace_constraints(
    patch: PatchView<'_>,
    triangles: &[[usize; 3]],
    samples: &[(usize, usize)],
) -> Result<Vec<VertexConstraint>, MechanicsError> {
    let mut derivatives = BTreeMap::<usize, ([BTreeMap<VelocityDof, f64>; 4], f64)>::new();
    let mut seen = BTreeSet::new();
    for &vertices in triangles {
        if !seen.insert(key(vertices)) {
            continue;
        }
        let t = triangle(vertices, patch)?;
        let (inverse, length, edges) = morley_interpolant(patch, vertices)?;
        let p = vertices.map(|v| patch.vertices[v].position_m);
        for i in 0..3 {
            let entry = derivatives
                .entry(vertices[i])
                .or_insert_with(|| (std::array::from_fn(|_| BTreeMap::new()), 0.0));
            entry.1 += t.area;
            let x = (p[i][0] - p[0][0]) / length;
            let y = (p[i][1] - p[0][1]) / length;
            for j in 0..6 {
                let (dof, scale) = if j < 3 {
                    (
                        VelocityDof::Translation {
                            vertex: vertices[j],
                            component: 2,
                        },
                        1.0,
                    )
                } else {
                    let [a, b] = edges[j - 3];
                    let pa = patch.vertices[a].position_m;
                    let pb = patch.vertices[b].position_m;
                    (
                        VelocityDof::NormalSlope { vertices: [a, b] },
                        (pb[0] - pa[0]).hypot(pb[1] - pa[1]),
                    )
                };
                let gradient = [
                    (inverse[1][j] + x * inverse[3][j] + y * inverse[4][j]) / length,
                    (inverse[2][j] + x * inverse[4][j] + y * inverse[5][j]) / length,
                    (inverse[3][j] + inverse[5][j]) / (length * length),
                ];
                for a in 0..3 {
                    *entry.0[a].entry(dof).or_default() += t.area * scale * gradient[a];
                }
            }
            for j in 0..3 {
                for a in 0..2 {
                    *entry.0[3]
                        .entry(VelocityDof::Translation {
                            vertex: vertices[j],
                            component: a,
                        })
                        .or_default() += t.area * t.gradient[j][a];
                }
            }
        }
    }
    let mut result = Vec::with_capacity(samples.len());
    for &(slave, master) in samples {
        let a = patch
            .vertices
            .get(slave)
            .ok_or(MechanicsError::InvalidMesh)?
            .position_m;
        let b = patch
            .vertices
            .get(master)
            .ok_or(MechanicsError::InvalidMesh)?
            .position_m;
        if slave == master || a[0] != b[0] || a[1] != b[1] {
            return Err(MechanicsError::InvalidMesh);
        }
        let (rows, area) = derivatives
            .get(&master)
            .ok_or(MechanicsError::InvalidMesh)?;
        let z = a[2] - b[2];
        let mut components: [BTreeMap<VelocityDof, f64>; 3] = std::array::from_fn(|component| {
            BTreeMap::from([(
                VelocityDof::Translation {
                    vertex: master,
                    component,
                },
                1.0,
            )])
        });
        for component in 0..2 {
            for (&dof, &value) in &rows[component] {
                *components[component].entry(dof).or_default() -= z * value / area;
            }
        }
        for (&dof, &value) in &rows[3] {
            *components[2].entry(dof).or_default() -= z * value / area;
        }
        for (&dof, &value) in &rows[2] {
            *components[2].entry(dof).or_default() += 0.5 * z * z * value / area;
        }
        result.push(VertexConstraint {
            vertex: slave,
            components: components.map(|row| {
                row.into_iter()
                    .map(|(dof, coefficient)| VelocityTerm { dof, coefficient })
                    .collect()
            }),
        });
    }
    Ok(result)
}

fn gauss(order: usize) -> Vec<(f64, f64)> {
    let mut out = Vec::with_capacity(order);
    for i in 0..order {
        let mut z = (std::f64::consts::PI * (i as f64 + 0.75) / (order as f64 + 0.5)).cos();
        let mut derivative = 0.0;
        for _ in 0..32 {
            let (mut p0, mut p1) = (1.0, z);
            for n in 2..=order {
                let next = ((2 * n - 1) as f64 * z * p1 - (n - 1) as f64 * p0) / n as f64;
                p0 = p1;
                p1 = next;
            }
            derivative = order as f64 * (z * p1 - p0) / (z * z - 1.0);
            let dz = p1 / derivative;
            z -= dz;
            if dz.abs() <= 4.0 * f64::EPSILON {
                break;
            }
        }
        out.push((
            0.5 * (1.0 + z),
            1.0 / ((1.0 - z * z) * derivative * derivative),
        ));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn check(
    patch: PatchView<'_>,
    dt: f64,
    accuracy: &MechanicsAccuracy,
) -> Result<(), MechanicsError> {
    if !dt.is_finite()
        || dt <= 0.0
        || patch.vertices.is_empty()
        || !patch.surface_tension_n_m.is_finite()
        || patch.surface_tension_n_m < 0.0
    {
        return Err(MechanicsError::InvalidMaterial);
    }
    if ![
        accuracy.divergence_error_s_inv,
        accuracy.traction_error_pa,
        accuracy.relative_time_error,
        accuracy.absolute_displacement_error_m,
        accuracy.maximum_resolved_slope,
        accuracy.maximum_resolved_strain,
        accuracy.maximum_donor_fraction,
        accuracy.penalty_pa_s,
        accuracy.divergence_penalty_pa_s,
    ]
    .iter()
    .all(|x| x.is_finite() && *x > 0.0)
        || accuracy.maximum_donor_fraction >= 1.0
        || !(3..=8).contains(&accuracy.quadrature_order)
        || accuracy.maximum_iterations == 0
        || accuracy.linear_iterations == 0
    {
        return Err(MechanicsError::InvalidMaterial);
    }
    for v in patch.vertices {
        if !v.position_m.iter().all(|x| x.is_finite()) {
            return Err(MechanicsError::NonFiniteState);
        }
    }
    if patch
        .collar_vertices
        .iter()
        .any(|&v| v >= patch.vertices.len())
    {
        return Err(MechanicsError::InvalidMesh);
    }
    for c in patch.constraints {
        if c.vertex >= patch.vertices.len()
            || c.components.iter().flatten().any(|t| {
                t.vertex >= patch.vertices.len() || t.component >= 3 || !t.coefficient.is_finite()
            })
        {
            return Err(MechanicsError::InvalidMesh);
        }
    }
    for c in patch.cells {
        if c.vertices.iter().any(|&i| i >= patch.vertices.len()) {
            return Err(MechanicsError::InvalidMesh);
        }
        if !c.eta_pa_s.is_finite()
            || c.eta_pa_s <= 0.0
            || !c.yield_pa.is_finite()
            || c.yield_pa < 0.0
        {
            return Err(MechanicsError::InvalidMaterial);
        }
    }
    for s in patch.skins {
        if s.vertices.iter().any(|&i| i >= patch.vertices.len()) {
            return Err(MechanicsError::InvalidMesh);
        }
        if !s.thickness_m.is_finite()
            || s.thickness_m <= 0.0
            || !s.tangent.z_ref_m.is_finite()
            || !(s.relaxation_s > 0.0)
            || s.relaxation_s.is_nan()
            || !s
                .tangent
                .a
                .iter()
                .flatten()
                .chain(s.tangent.b.iter().flatten())
                .chain(s.tangent.d.iter().flatten())
                .chain(s.history.tension_n_m.iter())
                .chain(s.history.moment_n.iter())
                .chain(s.chemical_force_increment_n_m.iter())
                .chain(s.chemical_moment_increment_n.iter())
                .all(|x| x.is_finite())
        {
            return Err(MechanicsError::InvalidMaterial);
        }
        let mut block = [[0.0; 6]; 6];
        for i in 0..3 {
            for j in 0..3 {
                if s.tangent.a[i][j] != s.tangent.a[j][i] || s.tangent.d[i][j] != s.tangent.d[j][i]
                {
                    return Err(MechanicsError::InvalidMaterial);
                }
                block[i][j] = s.tangent.a[i][j];
                block[i][j + 3] = s.tangent.b[i][j];
                block[i + 3][j] = s.tangent.b[j][i];
                block[i + 3][j + 3] = s.tangent.d[i][j];
            }
        }
        for k in 0..6 {
            let pivot = block[k][k];
            if pivot <= 0.0 {
                return Err(MechanicsError::InvalidMaterial);
            }
            for i in k + 1..6 {
                for j in i..6 {
                    block[j][i] -= block[i][k] * block[j][k] / pivot;
                    block[i][j] = block[j][i];
                }
            }
        }
    }
    Ok(())
}

fn assemble(
    patch: PatchView<'_>,
    accuracy: &MechanicsAccuracy,
    w: &mut Work,
) -> Result<(), MechanicsError> {
    w.faces.clear();
    w.divergence.clear();
    w.volumes.clear();
    w.skins.clear();
    w.exposed.clear();
    w.rotations.clear();
    let mut quadrature_count = 0;
    w.divergence_ratio = accuracy.divergence_penalty_pa_s / accuracy.penalty_pa_s;
    if !w.divergence_ratio.is_finite() {
        return Err(MechanicsError::InvalidMaterial);
    }
    w.newton_steps = 0;
    w.cg_steps = 0;
    w.backtracks = 0;
    w.preconditioner_fallbacks = 0;
    w.inner_residual_pa = 0.0;
    w.quadrature_ranges.clear();
    let mut faces = BTreeMap::<[usize; 3], usize>::new();
    let mut boundaries = BTreeMap::new();
    for b in patch.boundary {
        if b.vertices.iter().any(|&i| i >= patch.vertices.len())
            || boundaries.insert(key(b.vertices), b.kind).is_some()
        {
            return Err(MechanicsError::InvalidMesh);
        }
    }
    let skin_faces: BTreeSet<_> = patch.skins.iter().map(|s| key(s.vertices)).collect();
    let constrained_vertices: BTreeSet<_> = patch.constraints.iter().map(|c| c.vertex).collect();
    let linked_masters: BTreeSet<_> = patch
        .constraints
        .iter()
        .flat_map(|c| {
            c.components.iter().flatten().filter_map(|t| match t.dof {
                VelocityDof::Translation { vertex, .. } => Some(vertex),
                VelocityDof::NormalSlope { .. } => None,
            })
        })
        .collect();
    for (i, c) in patch.cells.iter().enumerate() {
        let p = c.vertices.map(|j| patch.vertices[j].position_m);
        let (volume, _) = tetra_geometry(p)?;
        w.volumes.push(volume);
        for opposite in 0..4 {
            let fv: Vec<_> = (0..4)
                .filter(|&j| j != opposite)
                .map(|j| c.vertices[j])
                .collect();
            let fv = [fv[0], fv[1], fv[2]];
            let k = key(fv);
            let index = *faces.entry(k).or_insert_with(|| {
                let index = w.faces.len();
                let a = patch.vertices[fv[0]].position_m;
                let mut n = cross(
                    sub(patch.vertices[fv[1]].position_m, a),
                    sub(patch.vertices[fv[2]].position_m, a),
                );
                if dot3(n, sub(p[opposite], a)) > 0.0 {
                    for x in &mut n {
                        *x = -*x;
                    }
                }
                let size = dot3(n, n).sqrt();
                w.faces.push(Face {
                    vertices: fv,
                    adjacent: Vec::new(),
                    normal: n.map(|x| x / size),
                    area: 0.5 * size,
                    bubble: None,
                    kind: boundaries.get(&k).copied(),
                });
                index
            });
            w.faces[index].adjacent.push((i, opposite));
            if w.faces[index].adjacent.len() > 2 {
                return Err(MechanicsError::InvalidMesh);
            }
        }
    }
    let mut dofs = 3 * patch.vertices.len();
    for f in &mut w.faces {
        let k = key(f.vertices);
        if !f.area.is_finite() || f.area <= 0.0 {
            return Err(MechanicsError::InvalidMesh);
        }
        if f.adjacent.len() == 1 && f.kind.is_none() {
            return Err(MechanicsError::InvalidMesh);
        }
        if f.adjacent.len() == 2 && f.kind.is_some() && f.kind != Some(BoundaryKind::Skin) {
            return Err(MechanicsError::InvalidMesh);
        }
        if skin_faces.contains(&k) {
            f.kind = Some(BoundaryKind::Skin);
        }
        if f.kind == Some(BoundaryKind::Skin)
            && !skin_faces.contains(&k)
            && f.vertices.iter().any(|v| !constrained_vertices.contains(v))
        {
            return Err(MechanicsError::InvalidMesh);
        }
        if f.adjacent.len() == 2 && f.kind.is_none() {
            f.bubble = Some(dofs);
            dofs += 1;
        }
        if matches!(
            f.kind,
            Some(BoundaryKind::Support | BoundaryKind::Artificial)
        ) && f
            .vertices
            .iter()
            .any(|&i| !patch.vertices[i].fixed.iter().all(|x| *x))
        {
            return Err(MechanicsError::InvalidMesh);
        }
        if f.kind == Some(BoundaryKind::Wall)
            && f.vertices
                .iter()
                .any(|&v| !patch.vertices[v].fixed.iter().all(|x| *x))
        {
            let axis = (0..3)
                .find(|&a| {
                    f.vertices.iter().all(|&v| {
                        patch.vertices[v].position_m[a]
                            == patch.vertices[f.vertices[0]].position_m[a]
                    })
                })
                .ok_or(MechanicsError::InvalidMesh)?;
            if f.vertices.iter().any(|&v| !patch.vertices[v].fixed[axis]) {
                return Err(MechanicsError::InvalidMesh);
            }
        }
    }
    for k in boundaries.keys() {
        if !faces.contains_key(k) {
            return Err(MechanicsError::InvalidMesh);
        }
    }
    for s in patch.skins {
        if !faces.contains_key(&key(s.vertices))
            && s.vertices.iter().any(|&i| {
                !patch.vertices[i].fixed.iter().all(|v| *v) && !linked_masters.contains(&i)
            })
        {
            return Err(MechanicsError::InvalidMesh);
        }
    }
    w.fixed.clear();
    for v in patch.vertices {
        w.fixed.extend(v.fixed);
    }
    w.fixed.resize(dofs, false);
    w.area_scale.clear();
    w.area_scale.resize(dofs, 0.0);
    let nodes = gauss(accuracy.quadrature_order);
    for (ci, cell) in patch.cells.iter().enumerate() {
        let quadrature_begin = quadrature_count;
        let p = cell.vertices.map(|j| patch.vertices[j].position_m);
        let (volume, g) = tetra_geometry(p)?;
        let mut local_faces = Vec::with_capacity(4);
        for opposite in 0..4 {
            let fv: Vec<_> = (0..4)
                .filter(|&j| j != opposite)
                .map(|j| cell.vertices[j])
                .collect();
            local_faces.push(faces[&key([fv[0], fv[1], fv[2]])]);
        }
        let mut divergence = Vec::new();
        for j in 0..4 {
            for a in 0..3 {
                divergence.push((3 * cell.vertices[j] + a, g[j][a]));
                w.area_scale[3 * cell.vertices[j] + a] += volume.powf(2.0 / 3.0) / 4.0;
            }
        }
        for &fi in &local_faces {
            let f = &w.faces[fi];
            if let Some(dof) = f.bubble {
                let sign = if f.adjacent[0].0 == ci { 1.0 } else { -1.0 };
                divergence.push((dof, sign * f.area / volume));
                w.area_scale[dof] += f.area;
            }
        }
        compress(&mut divergence);
        w.divergence.push(divergence);
        for &(r, wr) in &nodes {
            for &(s, ws) in &nodes {
                for &(t, wt) in &nodes {
                    let lambda = [
                        (1.0 - r) * (1.0 - s) * (1.0 - t),
                        r,
                        (1.0 - r) * s,
                        (1.0 - r) * (1.0 - s) * t,
                    ];
                    let weight = 6.0 * volume * wr * ws * wt * (1.0 - r).powi(2) * (1.0 - s);
                    let mut basis = Vec::<(usize, [[f64; 3]; 3])>::with_capacity(16);
                    for j in 0..4 {
                        for a in 0..3 {
                            let mut grad = [[0.0; 3]; 3];
                            grad[a] = g[j];
                            basis.push((3 * cell.vertices[j] + a, grad));
                        }
                    }
                    for (opposite, &fi) in local_faces.iter().enumerate() {
                        let f = &w.faces[fi];
                        if let Some(dof) = f.bubble {
                            let js: Vec<_> = (0..4).filter(|&j| j != opposite).collect();
                            let gb: [f64; 3] = std::array::from_fn(|a| {
                                60.0 * (g[js[0]][a] * lambda[js[1]] * lambda[js[2]]
                                    + lambda[js[0]] * g[js[1]][a] * lambda[js[2]]
                                    + lambda[js[0]] * lambda[js[1]] * g[js[2]][a])
                            });
                            basis.push((dof, std::array::from_fn(|a| gb.map(|x| x * f.normal[a]))));
                        }
                    }
                    let mut rows: [Row; 6] = if quadrature_count < w.quadrature.len() {
                        let mut rows = std::mem::take(&mut w.quadrature[quadrature_count].rows);
                        for row in &mut rows {
                            row.clear();
                        }
                        rows
                    } else {
                        std::array::from_fn(|_| Vec::with_capacity(basis.len()))
                    };
                    for (i, grad) in basis {
                        let d = [
                            grad[0][0],
                            grad[1][1],
                            grad[2][2],
                            (grad[0][1] + grad[1][0]) / ROOT_TWO,
                            (grad[0][2] + grad[2][0]) / ROOT_TWO,
                            (grad[1][2] + grad[2][1]) / ROOT_TWO,
                        ];
                        for k in 0..6 {
                            if d[k] != 0.0 {
                                rows[k].push((i, d[k]));
                            }
                        }
                    }
                    let mut divergence = Vec::new();
                    for row in &rows[..3] {
                        divergence.extend_from_slice(row);
                    }
                    compress(&mut divergence);
                    for row in &mut rows[..3] {
                        row.extend(divergence.iter().map(|&(i, a)| (i, -a / 3.0)));
                        compress(row);
                    }
                    let q = Quadrature {
                        rows,
                        divergence,
                        weight,
                        eta: cell.eta_pa_s,
                        yield_pa: cell.yield_pa,
                    };
                    if quadrature_count < w.quadrature.len() {
                        w.quadrature[quadrature_count] = q;
                    } else {
                        w.quadrature.push(q);
                    }
                    quadrature_count += 1;
                }
            }
        }
        w.quadrature_ranges.push(quadrature_begin..quadrature_count);
    }
    w.quadrature.truncate(quadrature_count);
    for s in patch.skins {
        w.skins.push(triangle(s.vertices, patch)?);
    }
    let mut edges = BTreeMap::<(u64, usize, usize), Vec<(usize, usize)>>::new();
    for (i, s) in patch.skins.iter().enumerate() {
        for edge in 0..3 {
            let (a, b) = (s.vertices[(edge + 1) % 3], s.vertices[(edge + 2) % 3]);
            edges
                .entry((s.packet_id, a.min(b), a.max(b)))
                .or_default()
                .push((i, edge));
        }
    }
    let mut rotations = BTreeMap::new();
    let mut physical_rotations = BTreeMap::<(usize, usize), (usize, f64)>::new();
    let mut physical_areas = BTreeMap::<(usize, usize), BTreeMap<[usize; 3], f64>>::new();
    for (i, s) in patch.skins.iter().enumerate() {
        for opposite in 0..3 {
            let a = s.vertices[(opposite + 1) % 3].min(s.vertices[(opposite + 2) % 3]);
            let b = s.vertices[(opposite + 1) % 3].max(s.vertices[(opposite + 2) % 3]);
            physical_areas
                .entry((a, b))
                .or_default()
                .insert(key(s.vertices), w.skins[i].area);
        }
    }
    let modes: BTreeMap<_, _> = patch
        .bending_modes
        .iter()
        .map(|m| {
            (
                (
                    m.packet_id,
                    m.vertices[0].min(m.vertices[1]),
                    m.vertices[0].max(m.vertices[1]),
                ),
                *m,
            )
        })
        .collect();
    if modes.len() != patch.bending_modes.len() || modes.len() != edges.len() {
        return Err(MechanicsError::InvalidMesh);
    }
    for (&edge, adjacent) in &edges {
        if adjacent.len() > 2 {
            return Err(MechanicsError::InvalidMesh);
        }
        let (_, a, b) = edge;
        let pa = patch.vertices[a].position_m;
        let pb = patch.vertices[b].position_m;
        let edge_length = (pb[0] - pa[0]).hypot(pb[1] - pa[1]);
        let mode = *modes.get(&edge).ok_or(MechanicsError::InvalidMesh)?;
        if !mode.normal_slope.is_finite() {
            return Err(MechanicsError::NonFiniteState);
        }
        let area: f64 = physical_areas[&(a, b)].values().sum();
        let dof = if let Some(&(i, slope)) = physical_rotations.get(&(a, b)) {
            if slope != mode.normal_slope {
                return Err(MechanicsError::InvalidMesh);
            }
            i
        } else {
            let i = dofs;
            dofs += 1;
            physical_rotations.insert((a, b), (i, mode.normal_slope));
            w.fixed.push(
                patch.vertices[a].fixed.iter().all(|v| *v)
                    && patch.vertices[b].fixed.iter().all(|v| *v),
            );
            w.area_scale.push(area);
            i
        };
        rotations.insert(edge, dof);
        w.rotations.push((mode, dof, edge_length));
    }
    for (i, s) in patch.skins.iter().enumerate() {
        let (inverse, length, edges) = morley_interpolant(patch, s.vertices)?;
        let mut local = [0; 6];
        for j in 0..3 {
            local[j] = 3 * s.vertices[j] + 2;
            let [a, b] = edges[j];
            local[j + 3] = rotations[&(s.packet_id, a, b)];
        }
        for j in 0..6 {
            w.skins[i].curvature[0].push((local[j], inverse[3][j] / (length * length)));
            w.skins[i].curvature[1].push((local[j], inverse[5][j] / (length * length)));
            w.skins[i].curvature[2].push((local[j], 2.0 * inverse[4][j] / (length * length)));
        }
    }
    for s in &mut w.skins {
        for row in &mut s.curvature {
            compress(row);
        }
    }
    let mut seen = BTreeSet::new();
    for f in &w.faces {
        let k = key(f.vertices);
        let exposed = f.kind == Some(BoundaryKind::Exposed)
            || patch
                .skins
                .iter()
                .any(|s| s.exposed && key(s.vertices) == k);
        if exposed && seen.insert(k) {
            w.exposed.push(triangle(f.vertices, patch)?);
        }
    }
    for s in patch.skins {
        if s.exposed && seen.insert(key(s.vertices)) {
            w.exposed.push(triangle(s.vertices, patch)?);
        }
    }
    for &vertices in patch.exposed_surfaces {
        if seen.insert(key(vertices)) {
            w.exposed.push(triangle(vertices, patch)?);
        }
    }
    w.velocity.clear();
    w.velocity.resize(dofs, 0.0);
    w.split.clear();
    w.split.resize(w.quadrature.len(), [0.0; 6]);
    w.dual.clear();
    w.dual.resize(w.quadrature.len(), [0.0; 6]);
    w.rates.clear();
    w.rates.resize(w.quadrature.len(), [0.0; 6]);
    w.pressure.clear();
    w.pressure.resize(patch.cells.len(), 0.0);
    for v in [
        &mut w.gradient,
        &mut w.direction,
        &mut w.residual,
        &mut w.conjugate,
        &mut w.product,
        &mut w.trial,
        &mut w.rhs,
        &mut w.constraint_force,
        &mut w.split_force,
        &mut w.preconditioned,
    ] {
        v.clear();
        v.resize(dofs, 0.0);
    }
    prepare_constraints(patch, w)?;
    if patch
        .collar_vertices
        .iter()
        .any(|&v| v >= patch.vertices.len())
    {
        return Err(MechanicsError::InvalidMesh);
    }
    if w.area_scale
        .iter()
        .enumerate()
        .any(|(i, a)| !w.fixed[i] && *a == 0.0)
    {
        return Err(MechanicsError::InvalidMesh);
    }
    w.fluid = constant_operator(w);
    w.newton = w.fluid.clone();
    Ok(())
}

fn strains(t: &Triangle, velocity: &[f64], dt: f64) -> (Tensor, Tensor, [f64; 2], [Row; 3]) {
    let mut gu = [[0.0; 2]; 3];
    for i in 0..3 {
        for a in 0..3 {
            for b in 0..2 {
                gu[a][b] += dt * t.gradient[i][b] * velocity[3 * t.vertices[i] + a];
            }
        }
    }
    let slope = [t.slope[0] + gu[2][0], t.slope[1] + gu[2][1]];
    let e = [
        gu[0][0] + t.slope[0] * gu[2][0] + 0.5 * gu[2][0].powi(2),
        gu[1][1] + t.slope[1] * gu[2][1] + 0.5 * gu[2][1].powi(2),
        gu[0][1] + gu[1][0] + t.slope[0] * gu[2][1] + t.slope[1] * gu[2][0] + gu[2][0] * gu[2][1],
    ];
    let k = std::array::from_fn(|a| dt * row_value(&t.curvature[a], velocity));
    let mut jac: [Row; 3] = std::array::from_fn(|_| Vec::new());
    for i in 0..3 {
        let v = 3 * t.vertices[i];
        let [gx, gy] = t.gradient[i];
        jac[0].extend([(v, dt * gx), (v + 2, dt * slope[0] * gx)]);
        jac[1].extend([(v + 1, dt * gy), (v + 2, dt * slope[1] * gy)]);
        jac[2].extend([
            (v, dt * gy),
            (v + 1, dt * gx),
            (v + 2, dt * (slope[0] * gy + slope[1] * gx)),
        ]);
    }
    (e, k, slope, jac)
}

fn history(s: &PlateTriangle, e: Tensor, k: Tensor, dt: f64) -> PlateHistory {
    advance_history(
        &s.tangent,
        s.history,
        s.chemical_force_increment_n_m,
        s.chemical_moment_increment_n,
        e,
        k,
        (-dt / s.relaxation_s).exp(),
    )
}

fn elastic(
    patch: PatchView<'_>,
    w: &Work,
    velocity: &[f64],
    dt: f64,
    gradient: Option<&mut [f64]>,
) -> f64 {
    let physical = w.physical_velocity(velocity);
    if w.lift.is_empty() {
        return elastic_raw(patch, w, &physical, dt, gradient);
    }
    let Some(gradient) = gradient else {
        return elastic_raw(patch, w, &physical, dt, None);
    };
    let mut raw = vec![0.0; velocity.len()];
    let energy = elastic_raw(patch, w, &physical, dt, Some(&mut raw));
    for (i, &value) in raw.iter().enumerate() {
        scatter(&w.lift[i], value, gradient);
    }
    energy
}

fn elastic_raw(
    patch: PatchView<'_>,
    w: &Work,
    velocity: &[f64],
    dt: f64,
    mut gradient: Option<&mut [f64]>,
) -> f64 {
    let mut energy = 0.0;
    for (s, t) in patch.skins.iter().zip(&w.skins) {
        let (e, k, _, jac) = strains(t, velocity, dt);
        let h0 = history(s, [0.0; 3], [0.0; 3], dt);
        let h1 = history(s, e, k, dt);
        energy += t.area
            * 0.5
            * (dot(&e, &plus(h0.tension_n_m, h1.tension_n_m))
                + dot(&k, &plus(h0.moment_n, h1.moment_n)));
        if let Some(g) = gradient.as_deref_mut() {
            for a in 0..3 {
                scatter(&jac[a], t.area * h1.tension_n_m[a], g);
                scatter(&t.curvature[a], dt * t.area * h1.moment_n[a], g);
            }
        }
    }
    for t in &w.exposed {
        let mut delta = [0.0; 2];
        for i in 0..3 {
            for a in 0..2 {
                delta[a] += dt * t.gradient[i][a] * velocity[3 * t.vertices[i] + 2];
            }
        }
        for a in 0..2 {
            energy += patch.surface_tension_n_m
                * t.area
                * (t.slope[a] * delta[a] + 0.5 * delta[a] * delta[a]);
            if let Some(g) = gradient.as_deref_mut() {
                for i in 0..3 {
                    g[3 * t.vertices[i] + 2] += dt
                        * patch.surface_tension_n_m
                        * t.area
                        * t.gradient[i][a]
                        * (t.slope[a] + delta[a]);
                }
            }
        }
    }
    energy
}

fn augmented(
    patch: PatchView<'_>,
    w: &Work,
    v: &[f64],
    dt: f64,
    rho: f64,
    mut g: Option<&mut [f64]>,
) -> f64 {
    if let Some(g) = g.as_deref_mut() {
        g.fill(0.0);
    }
    let mut energy = elastic(patch, w, v, dt, g.as_deref_mut());
    for i in 0..v.len() {
        let av = w.fluid.row_value(i, v);
        energy += dt * rho * v[i] * (0.5 * av - w.rhs[i]);
        if let Some(g) = g.as_deref_mut() {
            g[i] += dt * rho * (av - w.rhs[i]);
        }
    }
    energy
}

fn hessian(
    _patch: PatchView<'_>,
    w: &Work,
    direction: &[f64],
    _dt: f64,
    _rho: f64,
    out: &mut [f64],
) {
    w.newton.apply(direction, out);
}

#[cfg(test)]
fn hessian_reference(
    patch: PatchView<'_>,
    w: &Work,
    direction: &[f64],
    dt: f64,
    rho: f64,
    out: &mut [f64],
) {
    assert!(w.lift.is_empty());
    out.fill(0.0);
    for q in &w.quadrature {
        for row in &q.rows {
            scatter(row, dt * rho * q.weight * row_value(row, direction), out);
        }
    }
    for (i, row) in w.divergence.iter().enumerate() {
        scatter(
            row,
            dt * rho * w.divergence_ratio * w.volumes[i] * row_value(row, direction),
            out,
        );
    }
    for (s, t) in patch.skins.iter().zip(&w.skins) {
        let (e, k, _, jac) = strains(t, &w.velocity, dt);
        let de = std::array::from_fn(|a| row_value(&jac[a], direction));
        let dk = std::array::from_fn(|a| dt * row_value(&t.curvature[a], direction));
        let dh = advance_history(
            &s.tangent,
            PlateHistory::default(),
            [0.0; 3],
            [0.0; 3],
            de,
            dk,
            1.0,
        );
        for a in 0..3 {
            scatter(&jac[a], t.area * dh.tension_n_m[a], out);
            scatter(&t.curvature[a], dt * t.area * dh.moment_n[a], out);
        }
        let force = history(s, e, k, dt).tension_n_m;
        let mut gd = [0.0; 2];
        for i in 0..3 {
            for a in 0..2 {
                gd[a] += dt * t.gradient[i][a] * direction[3 * t.vertices[i] + 2];
            }
        }
        let traction = [
            force[0] * gd[0] + force[2] * gd[1],
            force[2] * gd[0] + force[1] * gd[1],
        ];
        for i in 0..3 {
            out[3 * t.vertices[i] + 2] +=
                dt * t.area * (t.gradient[i][0] * traction[0] + t.gradient[i][1] * traction[1]);
        }
    }
    for t in &w.exposed {
        let mut gd = [0.0; 2];
        for i in 0..3 {
            for a in 0..2 {
                gd[a] += dt * t.gradient[i][a] * direction[3 * t.vertices[i] + 2];
            }
        }
        for i in 0..3 {
            out[3 * t.vertices[i] + 2] += dt
                * t.area
                * patch.surface_tension_n_m
                * (gd[0] * t.gradient[i][0] + gd[1] * t.gradient[i][1]);
        }
    }
    for i in 0..out.len() {
        if w.fixed[i] {
            out[i] = direction[i];
        }
    }
}

fn elastic_remainder(
    patch: PatchView<'_>,
    w: &Work,
    direction: &[f64],
    dt: f64,
    fraction: f64,
) -> f64 {
    let physical = w.physical_velocity(&w.velocity);
    let physical_direction = w.physical_velocity(direction);
    let mut remainder = 0.0;
    for (s, t) in patch.skins.iter().zip(&w.skins) {
        let (e, k, _, jac) = strains(t, &physical, dt);
        let mut gw = [0.0; 2];
        for i in 0..3 {
            for a in 0..2 {
                gw[a] += dt * t.gradient[i][a] * physical_direction[3 * t.vertices[i] + 2];
            }
        }
        let nonlinear = [
            0.5 * fraction * fraction * gw[0] * gw[0],
            0.5 * fraction * fraction * gw[1] * gw[1],
            fraction * fraction * gw[0] * gw[1],
        ];
        let de = std::array::from_fn(|a| {
            fraction * row_value(&jac[a], &physical_direction) + nonlinear[a]
        });
        let dk = std::array::from_fn(|a| {
            fraction * dt * row_value(&t.curvature[a], &physical_direction)
        });
        let h = history(s, e, k, dt);
        let dh = advance_history(
            &s.tangent,
            PlateHistory::default(),
            [0.0; 3],
            [0.0; 3],
            de,
            dk,
            1.0,
        );
        remainder += t.area
            * (dot(&h.tension_n_m, &nonlinear)
                + 0.5 * dot(&de, &dh.tension_n_m)
                + 0.5 * dot(&dk, &dh.moment_n));
    }
    for t in &w.exposed {
        let mut gw = [0.0; 2];
        for i in 0..3 {
            for a in 0..2 {
                gw[a] += dt * t.gradient[i][a] * physical_direction[3 * t.vertices[i] + 2];
            }
        }
        remainder += 0.5
            * fraction
            * fraction
            * patch.surface_tension_n_m
            * t.area
            * (gw[0] * gw[0] + gw[1] * gw[1]);
    }
    remainder
}

fn minimize(patch: PatchView<'_>, w: &mut Work, dt: f64, accuracy: &MechanicsAccuracy) -> bool {
    let rho = accuracy.penalty_pa_s;
    let inner_target = 0.1
        * accuracy
            .traction_error_pa
            .min(accuracy.divergence_penalty_pa_s * accuracy.divergence_error_s_inv);
    let mut gradient = std::mem::take(&mut w.gradient);
    let mut product = std::mem::take(&mut w.product);
    let mut preconditioned = std::mem::take(&mut w.preconditioned);
    let mut direction = std::mem::take(&mut w.direction);
    let mut residual = std::mem::take(&mut w.residual);
    let mut conjugate = std::mem::take(&mut w.conjugate);
    let mut trial = std::mem::take(&mut w.trial);
    let mut converged = false;
    for _ in 0..accuracy.linear_iterations {
        w.newton_steps += 1;
        augmented(patch, w, &w.velocity, dt, rho, Some(&mut gradient));
        let mut maximum: f64 = 0.0;
        for i in 0..gradient.len() {
            if w.fixed[i] {
                gradient[i] = 0.0;
            } else {
                maximum = maximum.max(gradient[i].abs() / (dt * w.area_scale[i]));
            }
        }
        if maximum <= inner_target {
            converged = true;
            break;
        }
        update_newton(patch, w, dt, rho);
        w.preconditioner_fallbacks += usize::from(!w.newton.factored);
        direction.fill(0.0);
        for i in 0..gradient.len() {
            residual[i] = -gradient[i];
        }
        w.newton.precondition(&residual, &mut preconditioned);
        conjugate.copy_from_slice(&preconditioned);
        let mut rz: f64 = dot(&residual, &preconditioned);
        let initial = rz;
        for _ in 0..accuracy.linear_iterations {
            w.cg_steps += 1;
            hessian(patch, w, &conjugate, dt, rho, &mut product);
            let curvature = dot(&conjugate, &product);
            if !curvature.is_finite() || curvature <= 0.0 {
                break;
            }
            let alpha = rz / curvature;
            for i in 0..direction.len() {
                direction[i] += alpha * conjugate[i];
                residual[i] -= alpha * product[i];
            }
            w.newton.precondition(&residual, &mut preconditioned);
            let next: f64 = dot(&residual, &preconditioned);
            if next <= initial * 0.0001 {
                break;
            }
            let beta = next / rz;
            for i in 0..conjugate.len() {
                conjugate[i] = preconditioned[i] + beta * conjugate[i];
            }
            rz = next;
        }
        let mut slope = dot(&gradient, &direction);
        if !slope.is_finite() || slope >= 0.0 {
            w.newton.precondition(&gradient, &mut preconditioned);
            for i in 0..direction.len() {
                direction[i] = -preconditioned[i];
            }
            slope = dot(&gradient, &direction);
        }
        w.fluid.apply(&direction, &mut product);
        let fluid_curvature = dot(&direction, &product);
        let mut fraction = 1.0;
        let mut accepted = false;
        for _ in 0..64 {
            for i in 0..trial.len() {
                trial[i] = w.velocity[i] + fraction * direction[i];
            }
            let delta = fraction * slope
                + 0.5 * fraction * fraction * dt * rho * fluid_curvature
                + elastic_remainder(patch, w, &direction, dt, fraction);
            if delta.is_finite() && delta <= 0.0001 * fraction * slope {
                w.velocity.copy_from_slice(&trial);
                accepted = true;
                break;
            }
            fraction *= 0.5;
            w.backtracks += 1;
        }
        if !accepted {
            break;
        }
    }
    augmented(patch, w, &w.velocity, dt, rho, Some(&mut gradient));
    w.inner_residual_pa = gradient
        .iter()
        .enumerate()
        .filter(|(i, _)| !w.fixed[*i])
        .map(|(i, g)| g.abs() / (dt * w.area_scale[i]))
        .fold(0.0, f64::max);
    converged |= w.inner_residual_pa <= inner_target;
    w.gradient = gradient;
    w.product = product;
    w.preconditioned = preconditioned;
    w.direction = direction;
    w.residual = residual;
    w.conjugate = conjugate;
    w.trial = trial;
    converged
}

fn diagnostics(patch: PatchView<'_>, w: &Work, dt: f64, rho: f64, exact: bool) -> StepDiagnostics {
    let physical = w.physical_velocity(&w.velocity);
    let mut d = StepDiagnostics {
        energy_increment_j: elastic(patch, w, &w.velocity, dt, None),
        newton_iterations: w.newton_steps,
        krylov_iterations: w.cg_steps,
        line_search_backtracks: w.backtracks,
        preconditioner_fallbacks: w.preconditioner_fallbacks,
        inner_stationarity_residual_pa: w.inner_residual_pa,
        ..StepDiagnostics::default()
    };
    let mut forces = vec![0.0; w.velocity.len()];
    elastic(patch, w, &w.velocity, dt, Some(&mut forces));
    if !exact {
        for i in 0..forces.len() {
            forces[i] += dt * rho * w.constraint_force[i];
        }
    }
    let zero = w.velocity.iter().all(|v| *v == 0.0);
    for (i, q) in w.quadrature.iter().enumerate() {
        let rate: [f64; 6] = if zero {
            [0.0; 6]
        } else if exact {
            std::array::from_fn(|a| row_value(&q.rows[a], &w.velocity))
        } else {
            w.rates[i]
        };
        let norm = dot(&rate, &rate).sqrt();
        if exact {
            d.pointwise_divergence_error_s_inv = d
                .pointwise_divergence_error_s_inv
                .max(row_value(&q.divergence, &w.velocity).abs());
        }
        d.viscous_dissipation_j += 2.0 * dt * q.weight * q.eta * norm * norm;
        d.plastic_dissipation_j += dt * q.weight * q.yield_pa * ROOT_TWO * norm;
        let stress: [f64; 6] = w.dual[i].map(|x| rho * x);
        let split_norm = dot(&w.split[i], &w.split[i]).sqrt();
        let constitutive: [f64; 6] =
            std::array::from_fn(|a| stress[a] - 2.0 * q.eta * w.split[i][a]);
        let yield_error = if split_norm > 0.0 {
            let r: [f64; 6] = std::array::from_fn(|a| {
                constitutive[a] - q.yield_pa * ROOT_TWO * w.split[i][a] / split_norm
            });
            dot(&r, &r).sqrt() / ROOT_TWO
        } else {
            (dot(&constitutive, &constitutive).sqrt() / ROOT_TWO - q.yield_pa).max(0.0)
        };
        let consistency: [f64; 6] = std::array::from_fn(|a| rho * (rate[a] - w.split[i][a]));
        d.yield_residual_pa = d
            .yield_residual_pa
            .max(yield_error + dot(&consistency, &consistency).sqrt() / ROOT_TWO);
        if exact {
            for a in 0..6 {
                scatter(&q.rows[a], dt * q.weight * stress[a], &mut forces);
            }
        }
    }
    for (i, row) in w.divergence.iter().enumerate() {
        d.divergence_error_s_inv = d
            .divergence_error_s_inv
            .max(row_value(row, &w.velocity).abs());
        if exact {
            scatter(row, dt * rho * w.volumes[i] * w.pressure[i], &mut forces);
        }
    }
    for i in 0..forces.len() {
        if !w.fixed[i] {
            d.stationarity_residual_pa = d
                .stationarity_residual_pa
                .max(forces[i].abs() / (dt * w.area_scale[i]));
        }
    }
    for f in &w.faces {
        if f.kind == Some(BoundaryKind::Artificial) {
            for &v in &f.vertices {
                for a in 0..3 {
                    d.collar_traction_pa = d
                        .collar_traction_pa
                        .max(forces[3 * v + a].abs() / (dt * w.area_scale[3 * v + a]));
                }
            }
            for &(mode, i, _) in &w.rotations {
                if mode.vertices.iter().all(|v| f.vertices.contains(v)) {
                    d.collar_traction_pa = d
                        .collar_traction_pa
                        .max(forces[i].abs() / (dt * w.area_scale[i]));
                }
            }
        }
    }
    for (s, t) in patch.skins.iter().zip(&w.skins) {
        let (e, k, slope, _) = strains(t, &physical, dt);
        d.maximum_slope = d.maximum_slope.max(slope[0].hypot(slope[1]));
        d.maximum_strain = d
            .maximum_strain
            .max((e[0] * e[0] + e[1] * e[1] + 0.5 * e[2] * e[2]).sqrt());
        d.maximum_strain = d
            .maximum_strain
            .max(elastic_strain(s, history(s, e, k, dt)));
    }
    for t in &w.exposed {
        let (_, _, slope, _) = strains(t, &physical, dt);
        d.maximum_slope = d.maximum_slope.max(slope[0].hypot(slope[1]));
    }
    for &(mode, i, length) in &w.rotations {
        d.maximum_slope = d
            .maximum_slope
            .max((mode.normal_slope + dt * w.velocity[i] / length).abs());
    }
    for &v in patch.collar_vertices {
        for a in 0..3 {
            let row = w.pull_row(&vec![(3 * v + a, 1.0)]);
            for &(i, _) in &row {
                if w.fixed[i] && w.area_scale[i] > 0.0 {
                    d.collar_traction_pa = d
                        .collar_traction_pa
                        .max(forces[i].abs() / (dt * w.area_scale[i]));
                }
            }
        }
        for &(mode, i, _) in &w.rotations {
            if mode.vertices.contains(&v) && w.fixed[i] {
                d.collar_traction_pa = d
                    .collar_traction_pa
                    .max(forces[i].abs() / (dt * w.area_scale[i]));
            }
        }
    }
    d.arrested = w.velocity.iter().all(|x| *x == 0.0);
    d
}

fn solve(
    patch: PatchView<'_>,
    dt: f64,
    accuracy: &MechanicsAccuracy,
    w: &mut Work,
) -> Result<TrialAttempt, MechanicsError> {
    assemble(patch, accuracy, w)?;
    let rho = accuracy.penalty_pa_s;
    for iteration in 0..accuracy.maximum_iterations {
        if !minimize(patch, w, dt, accuracy) {
            let mut d = diagnostics(patch, w, dt, rho, true);
            d.iterations = iteration + 1;
            w.last_diagnostics = Some(d);
            return Ok(TrialAttempt::Subdivide(
                SubdivisionReason::NonlinearConvergence,
            ));
        }
        w.fluid.apply(&w.velocity, &mut w.product);
        w.split_force.fill(0.0);
        for (i, q) in w.quadrature.iter().enumerate() {
            let strain: [f64; 6] = std::array::from_fn(|a| row_value(&q.rows[a], &w.velocity));
            w.rates[i] = strain;
            let a: [f64; 6] = std::array::from_fn(|j| strain[j] + w.dual[i][j]);
            let norm = dot(&a, &a).sqrt();
            let factor = if norm == 0.0 {
                0.0
            } else {
                (1.0 - ROOT_TWO * q.yield_pa / (rho * norm)).max(0.0) * rho / (rho + 2.0 * q.eta)
            };
            for j in 0..6 {
                w.split[i][j] = factor * a[j];
                w.dual[i][j] += strain[j] - w.split[i][j];
                scatter(&q.rows[j], q.weight * w.split[i][j], &mut w.split_force);
            }
        }
        for (i, row) in w.divergence.iter().enumerate() {
            w.pressure[i] += w.divergence_ratio * row_value(row, &w.velocity);
        }
        for i in 0..w.velocity.len() {
            w.constraint_force[i] += w.product[i] - w.split_force[i];
            w.rhs[i] = w.split_force[i] - w.constraint_force[i];
        }
        if w.split.iter().all(|z| z.iter().all(|x| *x == 0.0)) {
            let retained = w.velocity.clone();
            w.velocity.fill(0.0);
            let d = diagnostics(patch, w, dt, rho, false);
            if d.stationarity_residual_pa > accuracy.traction_error_pa
                || d.yield_residual_pa > accuracy.traction_error_pa
            {
                w.velocity = retained;
            }
        }
        let mut d = diagnostics(patch, w, dt, rho, false);
        d.iterations = iteration + 1;
        if ![
            d.energy_increment_j,
            d.yield_residual_pa,
            d.stationarity_residual_pa,
            d.divergence_error_s_inv,
        ]
        .iter()
        .all(|x| x.is_finite())
        {
            w.last_diagnostics = Some(diagnostics(patch, w, dt, rho, true));
            return Err(MechanicsError::NonFiniteState);
        }
        if d.divergence_error_s_inv <= accuracy.divergence_error_s_inv
            && d.yield_residual_pa <= accuracy.traction_error_pa
            && d.stationarity_residual_pa <= accuracy.traction_error_pa
        {
            d = diagnostics(patch, w, dt, rho, true);
            d.iterations = iteration + 1;
            if d.divergence_error_s_inv > accuracy.divergence_error_s_inv
                || d.yield_residual_pa > accuracy.traction_error_pa
                || d.stationarity_residual_pa > accuracy.traction_error_pa
            {
                continue;
            }
            w.last_diagnostics = Some(d.clone());
            if d.collar_traction_pa > accuracy.traction_error_pa {
                return Err(MechanicsError::ExpandDomain);
            }
            if d.maximum_slope > accuracy.maximum_resolved_slope
                || d.maximum_strain > accuracy.maximum_resolved_strain
            {
                return Err(MechanicsError::SmallSlopeModelInvalid);
            }
            return finish(patch, dt, accuracy, w, d);
        }
    }
    let mut d = diagnostics(patch, w, dt, rho, true);
    d.iterations = accuracy.maximum_iterations;
    w.last_diagnostics = Some(d);
    Ok(TrialAttempt::Subdivide(SubdivisionReason::YieldResidual))
}

fn finish(
    patch: PatchView<'_>,
    dt: f64,
    accuracy: &MechanicsAccuracy,
    w: &Work,
    mut diagnostics: StepDiagnostics,
) -> Result<TrialAttempt, MechanicsError> {
    let physical = w.physical_velocity(&w.velocity);
    let mut transfers = Vec::new();
    let mut totals = volume_totals(patch, &w.volumes);
    let initial = totals.clone();
    let mut outgoing = BTreeMap::<u32, f64>::new();
    for (fi, f) in w.faces.iter().enumerate() {
        if f.adjacent.len() != 2 || f.kind == Some(BoundaryKind::Skin) {
            continue;
        }
        let low = patch.cells[f.adjacent[0].0].material_cell;
        let high = patch.cells[f.adjacent[1].0].material_cell;
        if low == high {
            continue;
        }
        let mean: [f64; 3] =
            std::array::from_fn(|a| f.vertices.iter().map(|&v| physical[3 * v + a] / 3.0).sum());
        let volume =
            dt * f.area * (dot3(mean, f.normal) + f.bubble.map(|i| w.velocity[i]).unwrap_or(0.0));
        if !volume.is_finite() {
            return Err(MechanicsError::NonFiniteState);
        }
        *totals.get_mut(&low).ok_or(MechanicsError::InvalidMesh)? -= volume;
        *totals.get_mut(&high).ok_or(MechanicsError::InvalidMesh)? += volume;
        *outgoing
            .entry(if volume >= 0.0 { low } else { high })
            .or_default() += volume.abs();
        transfers.push(FaceTransfer {
            face: fi,
            low_cell: low,
            high_cell: high,
            volume_m3: volume,
        });
    }
    for (id, volume) in outgoing {
        diagnostics.maximum_donor_fraction = diagnostics
            .maximum_donor_fraction
            .max(volume / initial[&id]);
    }
    if diagnostics.maximum_donor_fraction > accuracy.maximum_donor_fraction
        || totals.values().any(|v| *v < 0.0)
    {
        return Ok(TrialAttempt::Subdivide(SubdivisionReason::DonorDepletion));
    }
    let mut skin_transfers = Vec::new();
    let mut skin_volumes: BTreeMap<u32, f64> = patch
        .skin_material
        .iter()
        .map(|s| (s.cell, s.volume_m3))
        .collect();
    if patch.skin_faces.is_empty() {
        let mut edges = BTreeMap::<(u64, usize, usize), Vec<(usize, usize)>>::new();
        for (i, s) in patch.skins.iter().enumerate() {
            for opposite in 0..3 {
                let (a, b) = (
                    s.vertices[(opposite + 1) % 3],
                    s.vertices[(opposite + 2) % 3],
                );
                edges
                    .entry((s.packet_id, a.min(b), a.max(b)))
                    .or_default()
                    .push((i, opposite));
            }
        }
        let mut skin_out = vec![0.0; patch.skins.len()];
        for ((_, a, b), adjacent) in edges {
            let (i, opposite) = adjacent[0];
            let t = &w.skins[i];
            let pa = patch.vertices[a].position_m;
            let pb = patch.vertices[b].position_m;
            let length = (pa[0] - pb[0]).hypot(pa[1] - pb[1]);
            let inward = t.gradient[opposite];
            let norm = inward[0].hypot(inward[1]);
            let speed = -0.5
                * ((physical[3 * a] + physical[3 * b]) * inward[0]
                    + (physical[3 * a + 1] + physical[3 * b + 1]) * inward[1])
                / norm;
            if adjacent.len() == 1 {
                if speed.abs() * dt > accuracy.absolute_displacement_error_m {
                    return Err(MechanicsError::ExpandDomain);
                }
                continue;
            }
            let j = adjacent[1].0;
            let donor = if speed >= 0.0 { i } else { j };
            let volume = dt * length * speed * patch.skins[donor].thickness_m;
            skin_out[donor] += volume.abs();
            skin_transfers.push(SkinTransfer {
                face: skin_transfers.len(),
                low_cell: patch.skins[i].material_cell,
                high_cell: patch.skins[j].material_cell,
                volume_m3: volume,
            });
        }
        for (i, volume) in skin_out.iter().enumerate() {
            if patch.skin_material.is_empty() {
                *skin_volumes
                    .entry(patch.skins[i].material_cell)
                    .or_default() += w.skins[i].area * patch.skins[i].thickness_m;
            }
            let fraction = volume / (w.skins[i].area * patch.skins[i].thickness_m);
            diagnostics.maximum_donor_fraction = diagnostics.maximum_donor_fraction.max(fraction);
            if fraction > accuracy.maximum_donor_fraction {
                return Ok(TrialAttempt::Subdivide(SubdivisionReason::DonorDepletion));
            }
        }
    } else {
        if skin_volumes.len() != patch.skin_material.len() {
            return Err(MechanicsError::InvalidMesh);
        }
        for (face, f) in patch.skin_faces.iter().enumerate() {
            let [a, b] = f.velocity_vertices;
            let pa = patch.vertices[a].position_m;
            let pb = patch.vertices[b].position_m;
            let length = (pa[0] - pb[0]).hypot(pa[1] - pb[1]);
            let norm = f.normal_xy[0].hypot(f.normal_xy[1]);
            let speed = |i: usize| {
                (physical[3 * i] * f.normal_xy[0] + physical[3 * i + 1] * f.normal_xy[1]) / norm
            };
            let (u, v) = (speed(a), speed(b));
            let [h, k] = f.thickness_m;
            let volume = dt * length * (2.0 * h * u + h * v + k * u + 2.0 * k * v) / 6.0;
            if !volume.is_finite() {
                return Err(MechanicsError::NonFiniteState);
            }
            let Some(high_cell) = f.high_cell else {
                if dt * u.abs().max(v.abs()) > accuracy.absolute_displacement_error_m {
                    return Err(MechanicsError::ExpandDomain);
                }
                continue;
            };
            if high_cell == f.low_cell {
                continue;
            }
            skin_transfers.push(SkinTransfer {
                face,
                low_cell: f.low_cell,
                high_cell,
                volume_m3: volume,
            });
        }
    }
    let initial_skin = skin_volumes.clone();
    let mut skin_outgoing = BTreeMap::<u32, f64>::new();
    for f in &skin_transfers {
        *skin_volumes
            .get_mut(&f.low_cell)
            .ok_or(MechanicsError::InvalidMesh)? -= f.volume_m3;
        *skin_volumes
            .get_mut(&f.high_cell)
            .ok_or(MechanicsError::InvalidMesh)? += f.volume_m3;
        *skin_outgoing
            .entry(if f.volume_m3 >= 0.0 {
                f.low_cell
            } else {
                f.high_cell
            })
            .or_default() += f.volume_m3.abs();
    }
    for (id, volume) in skin_outgoing {
        let available = initial_skin[&id];
        let fraction = if available > 0.0 {
            volume / available
        } else if volume == 0.0 {
            0.0
        } else {
            f64::INFINITY
        };
        diagnostics.maximum_donor_fraction = diagnostics.maximum_donor_fraction.max(fraction);
        if fraction > accuracy.maximum_donor_fraction {
            return Ok(TrialAttempt::Subdivide(SubdivisionReason::DonorDepletion));
        }
    }
    if skin_volumes.values().any(|v| *v < 0.0) {
        return Ok(TrialAttempt::Subdivide(SubdivisionReason::DonorDepletion));
    }
    let displacements: Vec<[f64; 3]> = (0..patch.vertices.len())
        .map(|i| std::array::from_fn(|a| dt * physical[3 * i + a]))
        .collect();
    for c in patch.cells {
        let p = c.vertices.map(|i| {
            std::array::from_fn(|a| patch.vertices[i].position_m[a] + displacements[i][a])
        });
        if tetra_geometry(p).is_err() {
            return Ok(TrialAttempt::Subdivide(SubdivisionReason::Nonpenetration));
        }
    }
    let mut support: Vec<_> = w
        .faces
        .iter()
        .filter(|f| f.kind == Some(BoundaryKind::Support))
        .map(|f| f.vertices)
        .collect();
    support.extend_from_slice(patch.support_surfaces);
    if support.is_empty() && !patch.cells.is_empty() {
        return Err(MechanicsError::InvalidMesh);
    }
    for (i, v) in patch.vertices.iter().enumerate() {
        let position: [f64; 3] = std::array::from_fn(|a| v.position_m[a] + displacements[i][a]);
        for f in &support {
            let p = f.map(|j| patch.vertices[j].position_m);
            let det = (p[1][0] - p[0][0]) * (p[2][1] - p[0][1])
                - (p[1][1] - p[0][1]) * (p[2][0] - p[0][0]);
            if det == 0.0 {
                return Err(MechanicsError::InvalidMesh);
            }
            let b = ((position[0] - p[0][0]) * (p[2][1] - p[0][1])
                - (position[1] - p[0][1]) * (p[2][0] - p[0][0]))
                / det;
            let c = ((p[1][0] - p[0][0]) * (position[1] - p[0][1])
                - (p[1][1] - p[0][1]) * (position[0] - p[0][0]))
                / det;
            if b >= 0.0 && c >= 0.0 && b + c <= 1.0 {
                let floor = (1.0 - b - c) * p[0][2] + b * p[1][2] + c * p[2][2];
                if position[2] < floor {
                    return Ok(TrialAttempt::Subdivide(SubdivisionReason::Nonpenetration));
                }
            }
        }
    }
    let history = patch
        .skins
        .iter()
        .zip(&w.skins)
        .map(|(s, t)| {
            let (e, k, _, _) = strains(t, &physical, dt);
            history(s, e, k, dt)
        })
        .collect();
    let bending_modes = w
        .rotations
        .iter()
        .map(|&(m, i, length)| BendingMode {
            normal_slope: m.normal_slope + dt * w.velocity[i] / length,
            ..m
        })
        .collect();
    Ok(TrialAttempt::Solved(MechanicsTrial {
        input_generation: patch.generation,
        duration_s: dt,
        transfers,
        skin_transfers,
        history,
        bending_modes,
        vertex_displacement_m: displacements,
        end_cell_volumes_m3: totals.into_iter().collect(),
        end_skin_volumes_m3: skin_volumes.into_iter().collect(),
        diagnostics,
    }))
}

fn remap_checked(
    patch: PatchView<'_>,
    trial: &mut MechanicsTrial,
    accuracy: &MechanicsAccuracy,
    remap: &mut impl ConservativeRemap,
) -> Result<RemappedPatch, MechanicsError> {
    let mapped = remap.remap(patch, trial)?;
    let next = mapped.view();
    check(next, 1.0, accuracy)?;
    if next.collar_vertices != patch.collar_vertices
        || next.constraints.len() != patch.constraints.len()
        || next
            .constraints
            .iter()
            .zip(patch.constraints)
            .any(|(a, b)| {
                a.vertex != b.vertex
                    || (0..3).any(|i| {
                        a.components[i].len() != b.components[i].len()
                            || a.components[i]
                                .iter()
                                .zip(&b.components[i])
                                .any(|(x, y)| x.dof != y.dof)
                    })
            })
    {
        return Err(MechanicsError::RefineMesh);
    }
    let mode_keys = |modes: &[BendingMode]| {
        modes
            .iter()
            .map(|m| {
                (
                    m.vertices[0].min(m.vertices[1]),
                    m.vertices[0].max(m.vertices[1]),
                )
            })
            .collect::<BTreeSet<_>>()
    };
    if mode_keys(next.bending_modes) != mode_keys(&trial.bending_modes)
        || next
            .bending_modes
            .iter()
            .any(|m| !m.normal_slope.is_finite())
    {
        return Err(MechanicsError::InvalidMesh);
    }
    if next.vertices.len() != patch.vertices.len()
        || next.cells.len() != patch.cells.len()
        || next
            .cells
            .iter()
            .zip(patch.cells)
            .any(|(a, b)| a.vertices != b.vertices || a.material_cell != b.material_cell)
    {
        return Err(MechanicsError::RefineMesh);
    }
    let skin_geometry = |skins: &[PlateTriangle]| {
        skins
            .iter()
            .map(|s| (s.material_cell, key(s.vertices)))
            .collect::<BTreeSet<_>>()
    };
    if skin_geometry(next.skins) != skin_geometry(patch.skins)
        || next.exposed_surfaces != patch.exposed_surfaces
        || next.support_surfaces != patch.support_surfaces
        || next.skin_faces.len() != patch.skin_faces.len()
        || next.skin_faces.iter().zip(patch.skin_faces).any(|(a, b)| {
            a.low_cell != b.low_cell
                || a.high_cell != b.high_cell
                || a.velocity_vertices != b.velocity_vertices
        })
    {
        return Err(MechanicsError::RefineMesh);
    }
    if next.boundary.len() != patch.boundary.len()
        || next
            .boundary
            .iter()
            .zip(patch.boundary)
            .any(|(a, b)| a != b)
    {
        return Err(MechanicsError::InvalidMesh);
    }
    for (a, b) in next.vertices.iter().zip(patch.vertices) {
        if a.fixed != b.fixed || (0..3).any(|i| b.fixed[i] && a.position_m[i] != b.position_m[i]) {
            return Err(MechanicsError::InvalidMesh);
        }
    }
    let volumes: Result<Vec<_>, _> = next
        .cells
        .iter()
        .map(|c| tetra_geometry(c.vertices.map(|i| next.vertices[i].position_m)).map(|v| v.0))
        .collect();
    let totals = volume_totals(next, &volumes?);
    for &(id, expected) in &trial.end_cell_volumes_m3 {
        let actual = *totals.get(&id).ok_or(MechanicsError::InvalidMesh)?;
        let area = expected.abs().max(actual.abs()).powf(2.0 / 3.0);
        let error = if area > 0.0 {
            (actual - expected).abs() / area
        } else {
            0.0
        };
        trial.diagnostics.volume_geometry_error_m =
            trial.diagnostics.volume_geometry_error_m.max(error);
        if error > accuracy.absolute_displacement_error_m {
            return Err(MechanicsError::RefineMesh);
        }
    }
    let skin_volumes: BTreeMap<_, _> = next
        .skin_material
        .iter()
        .map(|s| (s.cell, s.volume_m3))
        .collect();
    if !patch.skin_material.is_empty() {
        for &(id, expected) in &trial.end_skin_volumes_m3 {
            let actual = *skin_volumes.get(&id).ok_or(MechanicsError::InvalidMesh)?;
            if (actual - expected).abs() > 64.0 * f64::EPSILON * actual.abs().max(expected.abs()) {
                return Err(MechanicsError::InvalidMesh);
            }
        }
    }
    let mut slope_sum = vec![[0.0; 2]; patch.vertices.len()];
    let mut weights = vec![0.0; patch.vertices.len()];
    let mut seen = BTreeSet::new();
    for vertices in patch
        .skins
        .iter()
        .map(|s| s.vertices)
        .chain(
            patch
                .boundary
                .iter()
                .filter(|b| b.kind == BoundaryKind::Exposed)
                .map(|b| b.vertices),
        )
        .chain(patch.exposed_surfaces.iter().copied())
    {
        if !seen.insert(key(vertices)) {
            continue;
        }
        let t = triangle(vertices, patch)?;
        for &i in &vertices {
            for a in 0..2 {
                slope_sum[i][a] += t.area * t.slope[a];
            }
            weights[i] += t.area;
        }
    }
    let mut error: f64 = 0.0;
    for i in 0..patch.vertices.len() {
        if weights[i] == 0.0 {
            continue;
        }
        if next.vertices[i].position_m[0] != patch.vertices[i].position_m[0]
            || next.vertices[i].position_m[1] != patch.vertices[i].position_m[1]
        {
            return Err(MechanicsError::InvalidMesh);
        }
        let d = trial.vertex_displacement_m[i];
        let predicted = patch.vertices[i].position_m[2] + d[2]
            - (slope_sum[i][0] * d[0] + slope_sum[i][1] * d[1]) / weights[i];
        error = error.max((next.vertices[i].position_m[2] - predicted).abs());
    }
    trial.diagnostics.kinematic_error_m = error;
    if error > accuracy.absolute_displacement_error_m {
        return Err(MechanicsError::RefineMesh);
    }
    Ok(mapped)
}

pub(super) fn attempt_step(
    patch: PatchView<'_>,
    interval: StepInterval,
    accuracy: &MechanicsAccuracy,
    w: &mut Work,
    comparison: &mut Work,
    remap: &mut impl ConservativeRemap,
) -> Result<StepAttempt, MechanicsError> {
    let dt = interval.end_s - interval.start_s;
    if !interval.start_s.is_finite() || !interval.end_s.is_finite() {
        return Err(MechanicsError::InvalidMaterial);
    }
    check(patch, dt, accuracy)?;
    let mut full = match solve(patch, dt, accuracy, w)? {
        TrialAttempt::Solved(step) => step,
        TrialAttempt::Subdivide(reason) => return Ok(StepAttempt::Subdivide(reason)),
    };
    let full_remapped = remap_checked(patch, &mut full, accuracy, remap)?;
    let half_skins: Vec<_> = patch
        .skins
        .iter()
        .map(|s| {
            let fraction = 1.0 / (1.0 + (-0.5 * dt / s.relaxation_s).exp());
            PlateTriangle {
                chemical_force_increment_n_m: s.chemical_force_increment_n_m.map(|v| fraction * v),
                chemical_moment_increment_n: s.chemical_moment_increment_n.map(|v| fraction * v),
                ..*s
            }
        })
        .collect();
    let half_patch = PatchView {
        skins: &half_skins,
        ..patch
    };
    let mut first = match solve(half_patch, 0.5 * dt, accuracy, comparison)? {
        TrialAttempt::Solved(step) => step,
        TrialAttempt::Subdivide(reason) => return Ok(StepAttempt::Subdivide(reason)),
    };
    let middle = remap_checked(half_patch, &mut first, accuracy, remap)?;
    let mut second = match solve(middle.view(), 0.5 * dt, accuracy, comparison)? {
        TrialAttempt::Solved(step) => step,
        TrialAttempt::Subdivide(reason) => return Ok(StepAttempt::Subdivide(reason)),
    };
    let half_remapped = remap_checked(middle.view(), &mut second, accuracy, remap)?;
    let mut maximum: f64 = 0.0;
    let mut scale: f64 = 0.0;
    for ((f, h), initial) in full_remapped
        .vertices
        .iter()
        .zip(&half_remapped.vertices)
        .zip(patch.vertices)
    {
        for k in 0..3 {
            maximum = maximum.max((f.position_m[k] - h.position_m[k]).abs());
            scale = scale.max((f.position_m[k] - initial.position_m[k]).abs());
        }
    }
    full.diagnostics.estimated_displacement_error_m = maximum;
    if maximum > accuracy.absolute_displacement_error_m + accuracy.relative_time_error * scale {
        return Ok(StepAttempt::Subdivide(SubdivisionReason::TimeAccuracy));
    }
    let cohort_key = |s: &PlateTriangle| (s.packet_id, s.material_cell, key(s.vertices));
    let full_cohorts: BTreeMap<_, _> = full_remapped
        .skins
        .iter()
        .map(|s| (cohort_key(s), s))
        .collect();
    let half_cohorts: BTreeMap<_, _> = half_remapped
        .skins
        .iter()
        .map(|s| (cohort_key(s), s))
        .collect();
    let keys: BTreeSet<_> = full_cohorts
        .keys()
        .chain(half_cohorts.keys())
        .copied()
        .collect();
    let mut errors =
        BTreeMap::<(u32, [usize; 3]), ([f64; 3], [f64; 3], [f64; 3], [f64; 3], f64)>::new();
    for k in keys {
        let f = full_cohorts.get(&k).copied();
        let h = half_cohorts.get(&k).copied();
        let reference = f.or(h).ok_or(MechanicsError::InvalidMesh)?;
        let entry = errors.entry((k.1, k.2)).or_insert((
            [0.0; 3],
            [0.0; 3],
            [0.0; 3],
            [0.0; 3],
            reference.thickness_m,
        ));
        for a in 0..3 {
            let ft = f.map(|s| s.history.tension_n_m[a]).unwrap_or(0.0);
            let ht = h.map(|s| s.history.tension_n_m[a]).unwrap_or(0.0);
            let fm = f.map(|s| s.history.moment_n[a]).unwrap_or(0.0);
            let hm = h.map(|s| s.history.moment_n[a]).unwrap_or(0.0);
            entry.0[a] += (ft - ht).abs();
            entry.1[a] += (fm - hm).abs();
            entry.2[a] += ft.abs().max(ht.abs());
            entry.3[a] += fm.abs().max(hm.abs());
        }
    }
    for (_, (force_errors, moment_errors, forces, moments, thickness)) in errors {
        for a in 0..3 {
            let force_error = force_errors[a];
            let moment_error = moment_errors[a];
            full.diagnostics.estimated_force_error_n_m =
                full.diagnostics.estimated_force_error_n_m.max(force_error);
            full.diagnostics.estimated_moment_error_n =
                full.diagnostics.estimated_moment_error_n.max(moment_error);
            if force_error
                > accuracy.traction_error_pa * thickness + accuracy.relative_time_error * forces[a]
                || moment_error
                    > accuracy.traction_error_pa * thickness * thickness
                        + accuracy.relative_time_error * moments[a]
            {
                return Ok(StepAttempt::Subdivide(SubdivisionReason::TimeAccuracy));
            }
        }
    }
    Ok(StepAttempt::Accepted(AcceptedStep {
        trial: full,
        remapped: full_remapped,
    }))
}

#[cfg(test)]
mod numerical_tests {
    use super::*;

    #[test]
    fn oil_mechanics_kirchhoff_trace_reproduces_quadratic() {
        let mut patch = RemappedPatch::default();
        for z in [0.0, 0.2] {
            for [x, y] in [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]] {
                patch.vertices.push(Vertex {
                    position_m: [x, y, z],
                    fixed: [false; 3],
                });
            }
        }
        let constraints =
            coherent_trace_constraints(patch.view(), &[[0, 1, 2]], &[(3, 0), (4, 1), (5, 2)])
                .unwrap();
        let value = |dof| match dof {
            VelocityDof::Translation { vertex, component } => {
                let [x, y, _] = patch.vertices[vertex].position_m;
                [2.0 * x + 3.0 * y, -y, 0.5 * x * x + x * y + y * y][component]
            }
            VelocityDof::NormalSlope { vertices: [a, b] } => {
                let pa = patch.vertices[a].position_m;
                let pb = patch.vertices[b].position_m;
                let length = (pb[0] - pa[0]).hypot(pb[1] - pa[1]);
                let x = 0.5 * (pa[0] + pb[0]);
                let y = 0.5 * (pa[1] + pb[1]);
                ((x + y) * (pb[1] - pa[1]) - (x + 2.0 * y) * (pb[0] - pa[0])) / length
            }
        };
        for c in constraints {
            let [x, y, z] = patch.vertices[c.vertex].position_m;
            let actual: [f64; 3] = c
                .components
                .map(|row| row.iter().map(|t| t.coefficient * value(t.dof)).sum());
            let expected = [
                2.0 * x + 3.0 * y - z * (x + y),
                -y - z * (x + 2.0 * y),
                0.5 * x * x + x * y + y * y - z + 1.5 * z * z,
            ];
            for i in 0..3 {
                assert!((actual[i] - expected[i]).abs() < 1.0e-13);
            }
        }
    }

    #[test]
    fn oil_mechanics_trace_constraint_virtual_work() {
        let vertices = vec![
            Vertex {
                position_m: [0.0, 0.0, 0.0],
                fixed: [true; 3],
            },
            Vertex {
                position_m: [1.0, 0.0, 0.0],
                fixed: [true; 3],
            },
            Vertex {
                position_m: [0.0, 1.0, 0.0],
                fixed: [true; 3],
            },
            Vertex {
                position_m: [0.0, 0.0, 1.0],
                fixed: [true; 3],
            },
            Vertex {
                position_m: [0.0, 0.0, 1.1],
                fixed: [true; 3],
            },
        ];
        let cells = [FlowCell {
            vertices: [0, 1, 2, 3],
            material_cell: 0,
            eta_pa_s: 2.0,
            yield_pa: 0.0,
        }];
        let boundary = [[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]].map(|vertices| BoundaryFace {
            vertices,
            kind: BoundaryKind::Wall,
        });
        let tangent = integrate_skin(
            &[SkinCell {
                z_lo_m: -0.01,
                z_hi_m: 0.01,
                young_pa: 100.0,
                poisson: 0.3,
            }],
            0.0,
        )
        .unwrap();
        let skins = [PlateTriangle {
            vertices: [1, 2, 3],
            packet_id: 1,
            material_cell: 1,
            thickness_m: 0.02,
            tangent,
            history: PlateHistory {
                tension_n_m: [1.0, 2.0, 0.3],
                moment_n: [0.0; 3],
            },
            chemical_force_increment_n_m: [0.0; 3],
            chemical_moment_increment_n: [0.0; 3],
            relaxation_s: f64::INFINITY,
            exposed: true,
        }];
        let constraints = [VertexConstraint {
            vertex: 3,
            components: std::array::from_fn(|component| {
                vec![VelocityTerm {
                    dof: VelocityDof::Translation {
                        vertex: 4,
                        component,
                    },
                    coefficient: 1.0,
                }]
            }),
        }];
        let mut patch = PatchView {
            generation: 0,
            vertices: &vertices,
            cells: &cells,
            boundary: &boundary,
            skins: &skins,
            bending_modes: &[],
            constraints: &constraints,
            collar_vertices: &[],
            skin_faces: &[],
            skin_material: &[],
            exposed_surfaces: &[],
            support_surfaces: &[],
            surface_tension_n_m: 0.03,
        };
        let modes = controls::initialize_bending_modes(patch).unwrap();
        patch.bending_modes = &modes;
        let mut w = Work::default();
        assemble(patch, &controls::proposed_accuracy(), &mut w).unwrap();
        let mut v = vec![0.0; w.velocity.len()];
        v[12] = 1.0e-5;
        v[13] = -2.0e-5;
        v[14] = 3.0e-5;
        let physical = w.physical_velocity(&v);
        assert_eq!(&physical[9..12], &v[12..15]);
        let mut g = vec![0.0; v.len()];
        augmented(patch, &w, &v, 0.01, 2.0, Some(&mut g));
        assert_eq!(&g[9..12], &[0.0; 3]);
        let epsilon = 1.0e-7;
        for i in 12..15 {
            let mut plus = v.clone();
            let mut minus = v.clone();
            plus[i] += epsilon;
            minus[i] -= epsilon;
            let derivative = (augmented(patch, &w, &plus, 0.01, 2.0, None)
                - augmented(patch, &w, &minus, 0.01, 2.0, None))
                / (2.0 * epsilon);
            assert!((derivative - g[i]).abs() < 1.0e-7 * g[i].abs().max(1.0e-12));
        }
    }

    #[test]
    fn oil_mechanics_bonded_cohorts_share_bending_modes() {
        let mut p = controls::proposed_parameters();
        p.shape = [2, 2, 1];
        let one = controls::fixture(controls::Control::SolidBulk, p).unwrap();
        let mut split = one.clone();
        split.skins.clear();
        let h = p.gap_m + p.skin_m;
        for s in &one.skins {
            for (packet_id, lo, hi) in [(1, -0.5 * h, 0.0), (2, 0.0, 0.5 * h)] {
                let tangent = integrate_skin(
                    &[SkinCell {
                        z_lo_m: lo,
                        z_hi_m: hi,
                        young_pa: p.young_pa,
                        poisson: p.poisson,
                    }],
                    0.0,
                )
                .unwrap();
                split.skins.push(PlateTriangle {
                    packet_id,
                    thickness_m: 0.5 * h,
                    tangent,
                    history: PlateHistory {
                        tension_n_m: s.history.tension_n_m.map(|v| 0.5 * v),
                        moment_n: [0.0; 3],
                    },
                    ..*s
                });
            }
        }
        split.bending_modes = controls::initialize_bending_modes(split.view()).unwrap();
        let mut a = Work::default();
        let mut b = Work::default();
        let accuracy = controls::proposed_accuracy();
        assemble(one.view(), &accuracy, &mut a).unwrap();
        assemble(split.view(), &accuracy, &mut b).unwrap();
        assert_eq!(a.velocity.len(), b.velocity.len());
        let v: Vec<_> = (0..a.velocity.len())
            .map(|i| 1.0e-6 * (i as f64).sin())
            .collect();
        let mut ga = vec![0.0; v.len()];
        let mut gb = vec![0.0; v.len()];
        let ea = elastic(one.view(), &a, &v, 1.0e-4, Some(&mut ga));
        let eb = elastic(split.view(), &b, &v, 1.0e-4, Some(&mut gb));
        assert!((ea - eb).abs() < 1.0e-12 * ea.abs().max(eb.abs()));
        let scale = ga.iter().map(|x| x.abs()).fold(0.0, f64::max);
        assert!(
            ga.iter()
                .zip(&gb)
                .all(|(a, b)| (a - b).abs() < 1.0e-12 * scale)
        );
        a.fixed.fill(false);
        b.fixed.fill(false);
        update_newton(one.view(), &mut a, 1.0e-4, accuracy.penalty_pa_s);
        update_newton(split.view(), &mut b, 1.0e-4, accuracy.penalty_pa_s);
        let mut reference = vec![0.0; v.len()];
        hessian_reference(
            one.view(),
            &a,
            &v,
            1.0e-4,
            accuracy.penalty_pa_s,
            &mut reference,
        );
        a.newton.apply(&v, &mut ga);
        b.newton.apply(&v, &mut gb);
        let scale = reference.iter().map(|v| v.abs()).fold(0.0, f64::max);
        assert!(
            ga.iter()
                .zip(&reference)
                .all(|(a, b)| (a - b).abs() < 1.0e-12 * scale)
        );
        assert!(
            ga.iter()
                .zip(&gb)
                .all(|(a, b)| (a - b).abs() < 1.0e-12 * scale)
        );
    }

    #[test]
    fn oil_mechanics_finite_depth_strain_and_flux_integrals() {
        let vertices = [
            Vertex {
                position_m: [0.0, 0.0, 0.0],
                fixed: [true; 3],
            },
            Vertex {
                position_m: [1.0, 0.0, 0.0],
                fixed: [true; 3],
            },
            Vertex {
                position_m: [0.0, 1.0, 0.0],
                fixed: [true; 3],
            },
            Vertex {
                position_m: [0.0, 0.0, 1.0],
                fixed: [true; 3],
            },
        ];
        let cells = [FlowCell {
            vertices: [0, 1, 2, 3],
            material_cell: 0,
            eta_pa_s: 2.0,
            yield_pa: 0.0,
        }];
        let boundary = [[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]].map(|vertices| BoundaryFace {
            vertices,
            kind: BoundaryKind::Wall,
        });
        let patch = PatchView {
            generation: 0,
            vertices: &vertices,
            cells: &cells,
            boundary: &boundary,
            skins: &[],
            bending_modes: &[],
            constraints: &[],
            collar_vertices: &[],
            skin_faces: &[],
            skin_material: &[],
            exposed_surfaces: &[],
            support_surfaces: &[],
            surface_tension_n_m: 0.0,
        };
        let mut w = Work::default();
        assemble(patch, &controls::proposed_accuracy(), &mut w).unwrap();
        w.fixed.fill(false);
        update_newton(patch, &mut w, 1.0, 1.0);
        let volume: f64 = w.quadrature.iter().map(|q| q.weight).sum();
        assert!((volume - 1.0 / 6.0).abs() < 1.0e-14);
        for normal_extension in [false, true] {
            let mut v = vec![0.0; w.velocity.len()];
            for (i, p) in vertices.iter().enumerate() {
                if normal_extension {
                    v[3 * i] = p.position_m[0];
                    v[3 * i + 1] = -p.position_m[1];
                } else {
                    v[3 * i] = p.position_m[2];
                }
            }
            let dissipation: f64 = w
                .quadrature
                .iter()
                .map(|q| {
                    let strain: [f64; 6] = std::array::from_fn(|a| row_value(&q.rows[a], &v));
                    2.0 * q.eta * q.weight * dot(&strain, &strain)
                })
                .sum();
            let expected = if normal_extension {
                8.0 * volume
            } else {
                2.0 * volume
            };
            assert!((dissipation - expected).abs() < 1.0e-13 * expected);
            assert!(row_value(&w.divergence[0], &v).abs() < 1.0e-14);
            let mut reference = vec![0.0; v.len()];
            let mut cached = vec![0.0; v.len()];
            hessian_reference(patch, &w, &v, 1.0, 1.0, &mut reference);
            w.newton.apply(&v, &mut cached);
            let scale = reference.iter().map(|x| x.abs()).fold(0.0, f64::max);
            assert!(
                reference
                    .iter()
                    .zip(&cached)
                    .all(|(a, b)| (a - b).abs() < 1.0e-13 * scale)
            );
        }
    }
}
