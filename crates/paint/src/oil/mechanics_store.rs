use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::Instant;

use super::chemistry::Cell;
use super::constitutive::{self, Parameters};
use super::mechanics::{self, *};
use super::store::{ColumnData, ColumnRef, Formulation};

#[derive(Clone, Debug)]
pub struct NativeColumn {
    pub address: u64,
    pub data: ColumnData,
    pub formulations: Vec<Arc<Formulation>>,
}

impl NativeColumn {
    pub fn read(address: u64, column: ColumnRef<'_>) -> Self {
        Self {
            address,
            data: column.to_owned(),
            formulations: column.formulations.to_vec(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Inventory {
    pub pigment_m3: f64,
    pub binder_m3: f64,
    pub solvent_m3: f64,
    pub oxygen: f64,
    pub oxidized_binder_m3: f64,
    pub network_binder_m3: f64,
    pub inhibitor_binder_m3: f64,
}

impl Inventory {
    pub fn volume_m3(self) -> f64 {
        self.pigment_m3 + self.binder_m3 + self.solvent_m3
    }

    fn scaled(self, scale: f64) -> Self {
        Self {
            pigment_m3: self.pigment_m3 * scale,
            binder_m3: self.binder_m3 * scale,
            solvent_m3: self.solvent_m3 * scale,
            oxygen: self.oxygen * scale,
            oxidized_binder_m3: self.oxidized_binder_m3 * scale,
            network_binder_m3: self.network_binder_m3 * scale,
            inhibitor_binder_m3: self.inhibitor_binder_m3 * scale,
        }
    }

    fn add(&mut self, other: Self) {
        self.pigment_m3 += other.pigment_m3;
        self.binder_m3 += other.binder_m3;
        self.solvent_m3 += other.solvent_m3;
        self.oxygen += other.oxygen;
        self.oxidized_binder_m3 += other.oxidized_binder_m3;
        self.network_binder_m3 += other.network_binder_m3;
        self.inhibitor_binder_m3 += other.inhibitor_binder_m3;
    }

    fn from_cell(cell: Cell, formulation: &Formulation, area_m2: f64) -> Self {
        let scale = area_m2 * 1.0e-6;
        let binder = cell.binder_um as f64 * scale;
        Self {
            pigment_m3: cell.pigment_um as f64 * scale,
            binder_m3: binder,
            solvent_m3: cell.solvent_um as f64 * scale,
            oxygen: cell.oxygen_inventory(&formulation.kinetics()) * scale,
            oxidized_binder_m3: binder * cell.oxidation as f64,
            network_binder_m3: binder * cell.network as f64,
            inhibitor_binder_m3: binder * cell.inhibitor as f64,
        }
    }

    pub fn cell(
        self,
        formulation: &Formulation,
        area_m2: f64,
        recipe: u32,
    ) -> Result<Cell, AdapterError> {
        if !area_m2.is_finite()
            || area_m2 <= 0.0
            || self.values().iter().any(|x| !x.is_finite() || *x < 0.0)
        {
            return Err(AdapterError::InvalidInventory);
        }
        let k = formulation.kinetics();
        let capacity =
            self.binder_m3 * k.oxygen_solubility + self.solvent_m3 * k.solvent_solubility;
        let fraction = |amount: f64, carrier: f64| -> Result<f32, AdapterError> {
            if amount > carrier * (1.0 + 64.0 * f64::EPSILON) || (carrier == 0.0 && amount != 0.0) {
                return Err(AdapterError::InvalidInventory);
            }
            Ok(if carrier > 0.0 {
                (amount / carrier).min(1.0) as f32
            } else {
                0.0
            })
        };
        let scale = 1.0e6 / area_m2;
        let cell = Cell {
            pigment_um: (self.pigment_m3 * scale) as f32,
            binder_um: (self.binder_m3 * scale) as f32,
            solvent_um: (self.solvent_m3 * scale) as f32,
            oxygen: fraction(self.oxygen, capacity)?,
            oxidation: fraction(self.oxidized_binder_m3, self.binder_m3)?,
            network: fraction(self.network_binder_m3, self.binder_m3)?,
            inhibitor: fraction(self.inhibitor_binder_m3, self.binder_m3)?,
            recipe,
        };
        if !cell.thickness_um().is_finite() || cell.thickness_um() <= 0.0 {
            return Err(AdapterError::InvalidInventory);
        }
        Ok(cell)
    }

    pub fn values(self) -> [f64; 7] {
        [
            self.pigment_m3,
            self.binder_m3,
            self.solvent_m3,
            self.oxygen,
            self.oxidized_binder_m3,
            self.network_binder_m3,
            self.inhibitor_binder_m3,
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Cohort {
    pub id: u64,
    pub formulation: Arc<Formulation>,
    pub inventory: Inventory,
    pub force_area_n_m3: Tensor,
    pub central_moment_area_n_m2: Tensor,
    pub chemical_force_area_n_m3: Tensor,
    pub chemical_central_moment_area_n_m2: Tensor,
}

impl Cohort {
    fn scaled(&self, fraction: f64) -> Self {
        Self {
            id: self.id,
            formulation: self.formulation.clone(),
            inventory: self.inventory.scaled(fraction),
            force_area_n_m3: self.force_area_n_m3.map(|x| x * fraction),
            central_moment_area_n_m2: self.central_moment_area_n_m2.map(|x| x * fraction),
            chemical_force_area_n_m3: self.chemical_force_area_n_m3.map(|x| x * fraction),
            chemical_central_moment_area_n_m2: self
                .chemical_central_moment_area_n_m2
                .map(|x| x * fraction),
        }
    }

    fn add(&mut self, other: &Self) -> Result<(), AdapterError> {
        if self.id != other.id || self.formulation != other.formulation {
            return Err(AdapterError::InvalidInventory);
        }
        self.inventory.add(other.inventory);
        for a in 0..3 {
            self.force_area_n_m3[a] += other.force_area_n_m3[a];
            self.central_moment_area_n_m2[a] += other.central_moment_area_n_m2[a];
            self.chemical_force_area_n_m3[a] += other.chemical_force_area_n_m3[a];
            self.chemical_central_moment_area_n_m2[a] += other.chemical_central_moment_area_n_m2[a];
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct MaterialCell {
    pub id: u32,
    pub native_column: usize,
    pub native_depth: usize,
    pub packet: usize,
    pub lower_vertices: [usize; 3],
    pub upper_vertices: [usize; 3],
    pub sheet_vertices: Option<[usize; 3]>,
    pub transport_vertices: Option<[usize; 3]>,
    pub cohorts: Vec<Cohort>,
}

#[derive(Clone, Debug)]
pub struct PatchCheckpoint {
    pub revision: u32,
    pub parameters: Parameters,
    pub geometry: RemappedPatch,
    pub material: Vec<MaterialCell>,
    pub native: Vec<NativeColumn>,
    pub changed_packets: BTreeSet<(usize, usize)>,
    pub next_cohort: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AdapterError {
    Mechanics(MechanicsError),
    InvalidInventory,
    InvalidNativeLayout,
    MissingCohortHistory {
        column: u64,
        packet: usize,
    },
    OccupancyRequired {
        column: u64,
        packet: usize,
        coverage: f32,
    },
    RefineTopology {
        column: u64,
        depth: usize,
    },
    UnsupportedInterface {
        column: u64,
        packet: usize,
    },
    UnknownGeneration,
}

impl From<MechanicsError> for AdapterError {
    fn from(value: MechanicsError) -> Self {
        Self::Mechanics(value)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NativeGrid {
    pub shape: [usize; 2],
    pub origin_m: [f64; 2],
    pub pixel_m: [f64; 2],
    pub physical_walls: [bool; 4],
    pub surface_tension_n_m: f64,
}

fn triangle_geometry(
    vertices: [usize; 3],
    patch: &RemappedPatch,
) -> Result<(f64, [[f64; 2]; 3]), AdapterError> {
    let p = vertices.map(|v| patch.vertices[v].position_m);
    let det = (p[1][0] - p[0][0]) * (p[2][1] - p[0][1]) - (p[1][1] - p[0][1]) * (p[2][0] - p[0][0]);
    if !det.is_finite() || det == 0.0 {
        return Err(AdapterError::InvalidNativeLayout);
    }
    Ok((
        0.5 * det.abs(),
        [
            [(p[1][1] - p[2][1]) / det, (p[2][0] - p[1][0]) / det],
            [(p[2][1] - p[0][1]) / det, (p[0][0] - p[2][0]) / det],
            [(p[0][1] - p[1][1]) / det, (p[1][0] - p[0][0]) / det],
        ],
    ))
}

fn mean_height(vertices: [usize; 3], patch: &RemappedPatch) -> f64 {
    vertices
        .iter()
        .map(|&v| patch.vertices[v].position_m[2] / 3.0)
        .sum()
}

fn material_phase(cell: &MaterialCell, parameters: &Parameters) -> Result<bool, AdapterError> {
    let mut coherent = None;
    for c in &cell.cohorts {
        let state = c.inventory.cell(&c.formulation, 1.0, 0)?;
        let material = constitutive::evaluate(&state, &c.formulation, parameters)
            .map_err(|_| AdapterError::InvalidInventory)?;
        let value = material.young_pa > 0.0;
        if coherent.is_some_and(|previous| previous != value) {
            return Err(AdapterError::RefineTopology {
                column: cell.native_column as u64,
                depth: cell.native_depth,
            });
        }
        coherent = Some(value);
    }
    coherent.ok_or(AdapterError::InvalidInventory)
}

impl PatchCheckpoint {
    pub fn from_full_coverage(
        native: Vec<NativeColumn>,
        grid: NativeGrid,
        parameters: Parameters,
    ) -> Result<Self, AdapterError> {
        parameters
            .validate()
            .map_err(|_| AdapterError::InvalidInventory)?;
        let [nx, ny] = grid.shape;
        if nx == 0
            || ny == 0
            || nx.checked_mul(ny) != Some(native.len())
            || grid.pixel_m.iter().any(|x| !x.is_finite() || *x <= 0.0)
            || grid.origin_m.iter().any(|x| !x.is_finite())
            || !grid.surface_tension_n_m.is_finite()
            || grid.surface_tension_n_m < 0.0
        {
            return Err(AdapterError::InvalidNativeLayout);
        }
        let first = &native[0].data;
        let layers = first.cells.len();
        if layers == 0 {
            return Err(AdapterError::InvalidNativeLayout);
        }
        let mut packets = vec![usize::MAX; layers];
        for (packet, p) in first.packets.iter().enumerate() {
            for depth in p.first_cell as usize..(p.first_cell as usize + p.cell_count as usize) {
                if depth >= layers || packets[depth] != usize::MAX {
                    return Err(AdapterError::InvalidNativeLayout);
                }
                packets[depth] = packet;
            }
        }
        if packets.contains(&usize::MAX) {
            return Err(AdapterError::InvalidNativeLayout);
        }
        let mut phase = vec![false; layers];
        for (column, input) in native.iter().enumerate() {
            let data = &input.data;
            if data.cells.len() != layers
                || data.packets.len() != first.packets.len()
                || data.coverage.len() != data.packets.len()
                || data.state.free_solvent_um != 0.0
                || data.coatings.iter().any(|c| c.before_packet != 0)
            {
                return Err(AdapterError::InvalidNativeLayout);
            }
            for (packet, p) in data.packets.iter().enumerate() {
                if data.coverage[packet] != 1.0 {
                    return Err(AdapterError::OccupancyRequired {
                        column: input.address,
                        packet,
                        coverage: data.coverage[packet],
                    });
                }
                if p.interface_damage != 0.0 {
                    return Err(AdapterError::UnsupportedInterface {
                        column: input.address,
                        packet,
                    });
                }
                if p.first_cell != first.packets[packet].first_cell
                    || p.cell_count != first.packets[packet].cell_count
                {
                    return Err(AdapterError::InvalidNativeLayout);
                }
            }
            for (depth, cell) in data.cells.iter().enumerate() {
                let recipe = input
                    .formulations
                    .get(cell.recipe as usize)
                    .ok_or(AdapterError::InvalidInventory)?;
                let m = constitutive::evaluate(cell, recipe, &parameters)
                    .map_err(|_| AdapterError::InvalidInventory)?;
                if cell.thickness_um() <= 0.0 {
                    return Err(AdapterError::InvalidInventory);
                }
                let value = m.young_pa > 0.0;
                if column == 0 {
                    phase[depth] = value;
                } else if phase[depth] != value {
                    return Err(AdapterError::RefineTopology {
                        column: input.address,
                        depth,
                    });
                }
            }
        }
        let corners = (nx + 1) * (ny + 1);
        let nxy = corners + native.len();
        let mut xy = Vec::with_capacity(nxy);
        for y in 0..=ny {
            for x in 0..=nx {
                xy.push([
                    grid.origin_m[0] + x as f64 * grid.pixel_m[0],
                    grid.origin_m[1] + y as f64 * grid.pixel_m[1],
                ]);
            }
        }
        for y in 0..ny {
            for x in 0..nx {
                xy.push([
                    grid.origin_m[0] + (x as f64 + 0.5) * grid.pixel_m[0],
                    grid.origin_m[1] + (y as f64 + 0.5) * grid.pixel_m[1],
                ]);
            }
        }
        let reconstruct = |means: &[f64]| -> Result<Vec<f64>, AdapterError> {
            let mut z = vec![0.0; nxy];
            for y in 0..=ny {
                for x in 0..=nx {
                    let mut count = 0;
                    for cy in y.saturating_sub(1)..=y.min(ny - 1) {
                        for cx in x.saturating_sub(1)..=x.min(nx - 1) {
                            z[y * (nx + 1) + x] += means[cy * nx + cx];
                            count += 1;
                        }
                    }
                    z[y * (nx + 1) + x] /= count as f64;
                }
            }
            for y in 0..ny {
                for x in 0..nx {
                    z[corners + y * nx + x] = 3.0 * means[y * nx + x]
                        - 0.5
                            * (z[y * (nx + 1) + x]
                                + z[y * (nx + 1) + x + 1]
                                + z[(y + 1) * (nx + 1) + x]
                                + z[(y + 1) * (nx + 1) + x + 1]);
                }
            }
            if z.iter().any(|x| !x.is_finite()) {
                return Err(AdapterError::InvalidNativeLayout);
            }
            Ok(z)
        };
        let support: Vec<_> = native
            .iter()
            .map(|c| {
                (c.data.state.support_z_um as f64
                    + c.data
                        .coatings
                        .iter()
                        .map(|c| c.thickness_um as f64)
                        .sum::<f64>())
                    * 1.0e-6
            })
            .collect();
        let mut heights = vec![reconstruct(&support)?];
        for depth in 0..layers {
            let h: Vec<_> = native
                .iter()
                .map(|c| c.data.cells[depth].thickness_um() * 1.0e-6)
                .collect();
            let h = reconstruct(&h)?;
            if h.iter().any(|x| *x <= 0.0) {
                return Err(AdapterError::Mechanics(MechanicsError::RefineMesh));
            }
            heights.push(
                heights
                    .last()
                    .unwrap()
                    .iter()
                    .zip(h)
                    .map(|(z, h)| z + h)
                    .collect(),
            );
        }
        let mut geometry = RemappedPatch {
            surface_tension_n_m: grid.surface_tension_n_m,
            ..RemappedPatch::default()
        };
        let mut edge_nodes = BTreeSet::new();
        let mut artificial_nodes = BTreeSet::new();
        for (node, &[x, y]) in xy.iter().enumerate() {
            let sides = [
                x == grid.origin_m[0],
                x == grid.origin_m[0] + nx as f64 * grid.pixel_m[0],
                y == grid.origin_m[1],
                y == grid.origin_m[1] + ny as f64 * grid.pixel_m[1],
            ];
            if sides.iter().any(|x| *x) {
                edge_nodes.insert(node);
            }
            if (0..4).any(|s| sides[s] && !grid.physical_walls[s]) {
                artificial_nodes.insert(node);
            }
        }
        for depth in 0..=layers {
            for node in 0..nxy {
                let fixed = depth == 0 || edge_nodes.contains(&node);
                geometry.vertices.push(Vertex {
                    position_m: [xy[node][0], xy[node][1], heights[depth][node]],
                    fixed: [fixed; 3],
                });
            }
        }
        let mut lateral = Vec::new();
        for y in 0..ny {
            for x in 0..nx {
                let a = y * (nx + 1) + x;
                let b = a + 1;
                let d = a + nx + 1;
                let c = d + 1;
                let m = corners + y * nx + x;
                for mut triangle in [[a, b, m], [b, c, m], [c, d, m], [d, a, m]] {
                    triangle.sort();
                    lateral.push((y * nx + x, triangle));
                }
            }
        }
        let mut material = Vec::new();
        let mut cohort_ids = vec![vec![0; layers]; native.len()];
        let mut next_cohort = 0;
        for row in &mut cohort_ids {
            for id in row {
                *id = next_cohort;
                next_cohort += 1;
            }
        }
        let mut sheet_nodes = vec![None; layers];
        let mut transport_nodes = vec![None; layers];
        let mut trace_samples = Vec::new();
        let mut trace_triangles = Vec::new();
        let mut depth = 0;
        while depth < layers {
            if !phase[depth] {
                depth += 1;
                continue;
            }
            let lo = depth;
            while depth < layers && phase[depth] {
                depth += 1;
            }
            let hi = depth;
            let base = geometry.vertices.len();
            for node in 0..nxy {
                geometry.vertices.push(Vertex {
                    position_m: [
                        xy[node][0],
                        xy[node][1],
                        0.5 * (heights[lo][node] + heights[hi][node]),
                    ],
                    fixed: [lo == 0 || edge_nodes.contains(&node); 3],
                });
                for interface in lo..=hi {
                    trace_samples.push((interface * nxy + node, base + node));
                }
                if artificial_nodes.contains(&node) {
                    geometry.collar_vertices.push(base + node);
                }
            }
            for layer in lo..hi {
                sheet_nodes[layer] = Some(base);
                let transport_base = geometry.vertices.len();
                transport_nodes[layer] = Some(transport_base);
                for node in 0..nxy {
                    geometry.vertices.push(Vertex {
                        position_m: [
                            xy[node][0],
                            xy[node][1],
                            0.5 * (heights[layer][node] + heights[layer + 1][node]),
                        ],
                        fixed: [lo == 0 || edge_nodes.contains(&node); 3],
                    });
                    trace_samples.push((transport_base + node, base + node));
                }
            }
            for &(_, triangle) in &lateral {
                trace_triangles.push(triangle.map(|v| base + v));
            }
        }
        trace_samples.sort();
        trace_samples.dedup();
        for depth in 0..layers {
            for &(column, triangle) in &lateral {
                let lower = triangle.map(|v| depth * nxy + v);
                let upper = triangle.map(|v| (depth + 1) * nxy + v);
                let (area, _) = triangle_geometry(lower, &geometry)?;
                let h = mean_height(upper, &geometry) - mean_height(lower, &geometry);
                let input = &native[column];
                let cell = input.data.cells[depth];
                let recipe = input.formulations[cell.recipe as usize].clone();
                let mut inventory = Inventory::from_cell(cell, &recipe, area);
                inventory = inventory.scaled(h / (cell.thickness_um() * 1.0e-6));
                let packet = packets[depth];
                let p = input.data.packets[packet];
                let mut force = [0.0; 3];
                let mut moment = [0.0; 3];
                if p.tension_n_m.iter().chain(&p.moment_n).any(|x| *x != 0.0) {
                    let start = p.first_cell as usize;
                    let end = start + p.cell_count as usize;
                    if (start..end).filter(|&d| phase[d]).count() != 1 {
                        return Err(AdapterError::MissingCohortHistory {
                            column: input.address,
                            packet,
                        });
                    }
                    if phase[depth] {
                        let packet_mid = 0.5
                            * (mean_height(triangle.map(|v| start * nxy + v), &geometry)
                                + mean_height(triangle.map(|v| end * nxy + v), &geometry));
                        let mid =
                            0.5 * (mean_height(lower, &geometry) + mean_height(upper, &geometry));
                        force = p.tension_n_m.map(|x| x as f64 * area);
                        moment = std::array::from_fn(|a| {
                            (p.moment_n[a] as f64 + (packet_mid - mid) * p.tension_n_m[a] as f64)
                                * area
                        });
                    }
                }
                material.push(MaterialCell {
                    id: material.len() as u32,
                    native_column: column,
                    native_depth: depth,
                    packet,
                    lower_vertices: lower,
                    upper_vertices: upper,
                    sheet_vertices: sheet_nodes[depth].map(|base| triangle.map(|v| base + v)),
                    transport_vertices: transport_nodes[depth]
                        .map(|base| triangle.map(|v| base + v)),
                    cohorts: vec![Cohort {
                        id: cohort_ids[column][depth],
                        formulation: recipe,
                        inventory,
                        force_area_n_m3: force,
                        central_moment_area_n_m2: moment,
                        chemical_force_area_n_m3: [0.0; 3],
                        chemical_central_moment_area_n_m2: [0.0; 3],
                    }],
                });
            }
        }
        let mut result = Self {
            revision: 1,
            parameters,
            geometry,
            material,
            native,
            changed_packets: BTreeSet::new(),
            next_cohort,
        };
        result.rebuild_material()?;
        result.geometry.bending_modes =
            mechanics::controls::initialize_bending_modes(result.geometry.view())?;
        result.geometry.constraints =
            coherent_trace_constraints(result.geometry.view(), &trace_triangles, &trace_samples)?;
        result.build_boundary(&edge_nodes, &artificial_nodes, nxy, layers)?;
        Ok(result)
    }

    fn rebuild_material(&mut self) -> Result<(), AdapterError> {
        self.geometry.cells.clear();
        self.geometry.skins.clear();
        self.geometry.skin_material.clear();
        for cell in &self.material {
            let (area, _) = triangle_geometry(cell.lower_vertices, &self.geometry)?;
            let lo = mean_height(cell.lower_vertices, &self.geometry);
            let hi = mean_height(cell.upper_vertices, &self.geometry);
            let volume: f64 = cell.cohorts.iter().map(|c| c.inventory.volume_m3()).sum();
            let coherent = material_phase(cell, &self.parameters)?;
            if coherent != cell.sheet_vertices.is_some() {
                return Err(AdapterError::RefineTopology {
                    column: self.native[cell.native_column].address,
                    depth: cell.native_depth,
                });
            }
            if let Some(vertices) = cell.sheet_vertices {
                let reference = mean_height(vertices, &self.geometry);
                let center = 0.5 * (lo + hi) - reference;
                self.geometry.skin_material.push(SkinMaterial {
                    cell: cell.id,
                    volume_m3: volume,
                });
                for c in &cell.cohorts {
                    let state = c.inventory.cell(&c.formulation, area, 0)?;
                    let m = constitutive::evaluate(&state, &c.formulation, &self.parameters)
                        .map_err(|_| AdapterError::InvalidInventory)?;
                    let fraction = c.inventory.volume_m3() / volume;
                    let tangent = integrate_skin(
                        &[SkinCell {
                            z_lo_m: lo - reference,
                            z_hi_m: hi - reference,
                            young_pa: m.young_pa * fraction,
                            poisson: m.poisson,
                        }],
                        0.0,
                    )?;
                    self.geometry.skins.push(PlateTriangle {
                        vertices,
                        packet_id: c.id,
                        material_cell: cell.id,
                        thickness_m: hi - lo,
                        tangent,
                        history: PlateHistory {
                            tension_n_m: c.force_area_n_m3.map(|x| x / area),
                            moment_n: std::array::from_fn(|a| {
                                (c.central_moment_area_n_m2[a] + center * c.force_area_n_m3[a])
                                    / area
                            }),
                        },
                        chemical_force_increment_n_m: c.chemical_force_area_n_m3.map(|x| x / area),
                        chemical_moment_increment_n: std::array::from_fn(|a| {
                            (c.chemical_central_moment_area_n_m2[a]
                                + center * c.chemical_force_area_n_m3[a])
                                / area
                        }),
                        relaxation_s: m.relaxation_seconds,
                        exposed: false,
                    });
                }
            } else {
                let mut eta = 0.0;
                let mut yield_pa = 0.0;
                for c in &cell.cohorts {
                    if c.force_area_n_m3
                        .iter()
                        .chain(&c.central_moment_area_n_m2)
                        .any(|x| *x != 0.0)
                    {
                        return Err(AdapterError::InvalidInventory);
                    }
                    let state = c.inventory.cell(&c.formulation, area, 0)?;
                    let m = constitutive::evaluate(&state, &c.formulation, &self.parameters)
                        .map_err(|_| AdapterError::InvalidInventory)?;
                    let weight = c.inventory.volume_m3() / volume;
                    eta += weight * m.plastic_viscosity_pa_s;
                    yield_pa += weight * m.shear_yield_pa;
                }
                let [a, b, c] = cell.lower_vertices;
                let [d, e, f] = cell.upper_vertices;
                for mut vertices in [[a, b, c, f], [a, b, e, f], [a, d, e, f]] {
                    let p = vertices.map(|v| self.geometry.vertices[v].position_m);
                    let u = std::array::from_fn::<_, 3, _>(|i| p[1][i] - p[0][i]);
                    let v = std::array::from_fn::<_, 3, _>(|i| p[2][i] - p[0][i]);
                    let w = std::array::from_fn::<_, 3, _>(|i| p[3][i] - p[0][i]);
                    let det = u[0] * (v[1] * w[2] - v[2] * w[1])
                        - u[1] * (v[0] * w[2] - v[2] * w[0])
                        + u[2] * (v[0] * w[1] - v[1] * w[0]);
                    if det < 0.0 {
                        vertices.swap(0, 1);
                    }
                    self.geometry.cells.push(FlowCell {
                        vertices,
                        material_cell: cell.id,
                        eta_pa_s: eta,
                        yield_pa,
                    });
                }
            }
        }
        Ok(())
    }

    fn build_boundary(
        &mut self,
        edge_nodes: &BTreeSet<usize>,
        artificial: &BTreeSet<usize>,
        nxy: usize,
        layers: usize,
    ) -> Result<(), AdapterError> {
        self.geometry.boundary.clear();
        self.geometry.exposed_surfaces.clear();
        self.geometry.support_surfaces.clear();
        self.geometry.skin_faces.clear();
        let mut faces = BTreeMap::<[usize; 3], usize>::new();
        for cell in &self.geometry.cells {
            for opposite in 0..4 {
                let vertices: Vec<_> = (0..4)
                    .filter(|&j| j != opposite)
                    .map(|j| cell.vertices[j])
                    .collect();
                let mut key = [vertices[0], vertices[1], vertices[2]];
                key.sort();
                *faces.entry(key).or_default() += 1;
            }
        }
        let mut skins = BTreeSet::new();
        let mut edges = BTreeMap::<(usize, usize, usize), Vec<(u32, usize)>>::new();
        for cell in &self.material {
            if let Some(sheet) = cell.sheet_vertices {
                skins.insert({
                    let mut v = cell.lower_vertices;
                    v.sort();
                    v
                });
                skins.insert({
                    let mut v = cell.upper_vertices;
                    v.sort();
                    v
                });
                for opposite in 0..3 {
                    let a = sheet[(opposite + 1) % 3].min(sheet[(opposite + 2) % 3]);
                    let b = sheet[(opposite + 1) % 3].max(sheet[(opposite + 2) % 3]);
                    edges
                        .entry((cell.native_depth, a, b))
                        .or_default()
                        .push((cell.id, opposite));
                }
            }
            if cell.native_depth == 0 {
                self.geometry.support_surfaces.push(cell.lower_vertices);
            }
            if cell.native_depth + 1 == layers {
                self.geometry.exposed_surfaces.push(cell.upper_vertices);
            }
        }
        for (vertices, count) in faces {
            if count == 1 {
                let kind = if vertices.iter().all(|&v| v < nxy) {
                    BoundaryKind::Support
                } else if skins.contains(&vertices) {
                    BoundaryKind::Skin
                } else if vertices.iter().all(|&v| v / nxy == layers) {
                    BoundaryKind::Exposed
                } else if vertices.iter().all(|v| artificial.contains(&(v % nxy))) {
                    BoundaryKind::Artificial
                } else if vertices.iter().all(|v| edge_nodes.contains(&(v % nxy))) {
                    BoundaryKind::Wall
                } else {
                    return Err(AdapterError::InvalidNativeLayout);
                };
                self.geometry.boundary.push(BoundaryFace { vertices, kind });
            }
        }
        let by_id: BTreeMap<_, _> = self.material.iter().map(|c| (c.id, c)).collect();
        for ((_, a, b), adjacent) in edges {
            if adjacent.len() > 2 {
                return Err(AdapterError::InvalidNativeLayout);
            }
            let (id, opposite) = adjacent[0];
            let cell = by_id[&id];
            let sheet = cell.sheet_vertices.unwrap();
            let (_, gradient) = triangle_geometry(sheet, &self.geometry)?;
            let mut thickness = [0.0; 2];
            for (i, v) in [a, b].into_iter().enumerate() {
                let j = sheet
                    .iter()
                    .position(|&n| n == v)
                    .ok_or(AdapterError::InvalidNativeLayout)?;
                thickness[i] = self.geometry.vertices[cell.upper_vertices[j]].position_m[2]
                    - self.geometry.vertices[cell.lower_vertices[j]].position_m[2];
            }
            let transport = cell
                .transport_vertices
                .ok_or(AdapterError::InvalidNativeLayout)?;
            let va = transport[sheet
                .iter()
                .position(|&v| v == a)
                .ok_or(AdapterError::InvalidNativeLayout)?];
            let vb = transport[sheet
                .iter()
                .position(|&v| v == b)
                .ok_or(AdapterError::InvalidNativeLayout)?];
            self.geometry.skin_faces.push(SkinFace {
                low_cell: id,
                high_cell: adjacent.get(1).map(|x| x.0),
                velocity_vertices: [va, vb],
                thickness_m: thickness,
                normal_xy: gradient[opposite].map(|x| -x),
            });
        }
        Ok(())
    }

    pub fn inventory(&self) -> Inventory {
        let mut result = Inventory::default();
        for cell in &self.material {
            for cohort in &cell.cohorts {
                result.add(cohort.inventory);
            }
        }
        result
    }
}

fn rotate_tensor(t: Tensor, angle: f64) -> Tensor {
    if angle == 0.0 {
        return t;
    }
    let (s, c) = angle.sin_cos();
    [
        c * c * t[0] + s * s * t[1] - 2.0 * c * s * t[2],
        s * s * t[0] + c * c * t[1] + 2.0 * c * s * t[2],
        c * s * (t[0] - t[1]) + (c * c - s * s) * t[2],
    ]
}

fn move_geometry(
    source: PatchView<'_>,
    trial: &MechanicsTrial,
    mapped: &mut RemappedPatch,
) -> Result<(), AdapterError> {
    let mut slopes = vec![[0.0; 2]; source.vertices.len()];
    let mut weights = vec![0.0; source.vertices.len()];
    let mut seen = BTreeSet::new();
    let old = RemappedPatch {
        vertices: source.vertices.to_vec(),
        ..RemappedPatch::default()
    };
    let geometry = source
        .skins
        .iter()
        .map(|s| s.vertices)
        .chain(source.exposed_surfaces.iter().copied())
        .chain(
            source
                .boundary
                .iter()
                .filter(|b| b.kind == BoundaryKind::Exposed || b.kind == BoundaryKind::Skin)
                .map(|b| b.vertices),
        );
    for triangle in geometry {
        let mut key = triangle;
        key.sort();
        if !seen.insert(key) {
            continue;
        }
        let (area, gradient) = triangle_geometry(triangle, &old)?;
        let slope: [f64; 2] = std::array::from_fn(|a| {
            (0..3)
                .map(|j| gradient[j][a] * source.vertices[triangle[j]].position_m[2])
                .sum()
        });
        for v in triangle {
            for a in 0..2 {
                slopes[v][a] += area * slope[a];
            }
            weights[v] += area;
        }
    }
    for i in 0..mapped.vertices.len() {
        if weights[i] > 0.0 {
            let d = trial.vertex_displacement_m[i];
            mapped.vertices[i].position_m[2] +=
                d[2] - (slopes[i][0] * d[0] + slopes[i][1] * d[1]) / weights[i];
        }
    }
    Ok(())
}

struct ProvisionalRemap {
    states: BTreeMap<u64, PatchCheckpoint>,
    next_generation: u64,
    failure: Option<AdapterError>,
}

impl ProvisionalRemap {
    fn map(
        &mut self,
        source: PatchView<'_>,
        trial: &MechanicsTrial,
    ) -> Result<RemappedPatch, AdapterError> {
        let before = self
            .states
            .get(&source.generation)
            .ok_or(AdapterError::UnknownGeneration)?;
        if trial.input_generation != source.generation
            || trial.vertex_displacement_m.len() != source.vertices.len()
            || trial.history.len() != source.skins.len()
        {
            return Err(AdapterError::UnknownGeneration);
        }
        let mut after = before.clone();
        after.geometry.vertices = source.vertices.to_vec();
        move_geometry(source, trial, &mut after.geometry)?;
        let by_id: BTreeMap<_, _> = before
            .material
            .iter()
            .enumerate()
            .map(|(i, c)| (c.id, i))
            .collect();
        let energy: BTreeMap<_, _> = source
            .skins
            .iter()
            .enumerate()
            .map(|(i, s)| ((s.material_cell, s.packet_id), i))
            .collect();
        let mut moved = before.material.clone();
        for (index, cell) in moved.iter_mut().enumerate() {
            let previous = &before.material[index];
            let old_mid = 0.5
                * (mean_height(previous.lower_vertices, &before.geometry)
                    + mean_height(previous.upper_vertices, &before.geometry));
            if let Some(sheet) = previous.sheet_vertices {
                let transport = previous
                    .transport_vertices
                    .ok_or(AdapterError::InvalidNativeLayout)?;
                let (area, gradient) = triangle_geometry(sheet, &before.geometry)?;
                let reference = mean_height(sheet, &before.geometry);
                let mut displacement_gradient = [[0.0; 2]; 2];
                for a in 0..2 {
                    for b in 0..2 {
                        displacement_gradient[a][b] = (0..3)
                            .map(|j| gradient[j][b] * trial.vertex_displacement_m[transport[j]][a])
                            .sum();
                    }
                }
                let f = [
                    [
                        1.0 + displacement_gradient[0][0],
                        displacement_gradient[0][1],
                    ],
                    [
                        displacement_gradient[1][0],
                        1.0 + displacement_gradient[1][1],
                    ],
                ];
                if f[0][0] * f[1][1] - f[0][1] * f[1][0] <= 0.0 {
                    return Err(AdapterError::Mechanics(
                        MechanicsError::SmallSlopeModelInvalid,
                    ));
                }
                let angle = (f[1][0] - f[0][1]).atan2(f[0][0] + f[1][1]);
                let dz = transport
                    .iter()
                    .map(|&v| trial.vertex_displacement_m[v][2] / 3.0)
                    .sum::<f64>();
                for c in &mut cell.cohorts {
                    let i = *energy
                        .get(&(cell.id, c.id))
                        .ok_or(AdapterError::InvalidInventory)?;
                    let h = trial.history[i];
                    if h != source.skins[i].history
                        || trial.vertex_displacement_m[transport[0]] != [0.0; 3]
                        || trial.vertex_displacement_m[transport[1]] != [0.0; 3]
                        || trial.vertex_displacement_m[transport[2]] != [0.0; 3]
                    {
                        after
                            .changed_packets
                            .insert((cell.native_column, cell.packet));
                    }
                    c.force_area_n_m3 = rotate_tensor(h.tension_n_m.map(|x| x * area), angle);
                    c.central_moment_area_n_m2 = rotate_tensor(
                        std::array::from_fn(|a| {
                            (h.moment_n[a] + (reference + dz) * h.tension_n_m[a]) * area
                        }),
                        angle,
                    );
                    c.chemical_force_area_n_m3 = rotate_tensor(
                        source.skins[i]
                            .chemical_force_increment_n_m
                            .map(|x| x * area),
                        angle,
                    );
                    c.chemical_central_moment_area_n_m2 = rotate_tensor(
                        std::array::from_fn(|a| {
                            (source.skins[i].chemical_moment_increment_n[a]
                                + (reference + dz)
                                    * source.skins[i].chemical_force_increment_n_m[a])
                                * area
                        }),
                        angle,
                    );
                }
            } else {
                for c in &mut cell.cohorts {
                    for a in 0..3 {
                        c.central_moment_area_n_m2[a] += old_mid * c.force_area_n_m3[a];
                        c.chemical_central_moment_area_n_m2[a] +=
                            old_mid * c.chemical_force_area_n_m3[a];
                    }
                }
            }
        }
        let transfers: Vec<_> = trial
            .transfers
            .iter()
            .map(|f| (f.low_cell, f.high_cell, f.volume_m3))
            .chain(
                trial
                    .skin_transfers
                    .iter()
                    .map(|f| (f.low_cell, f.high_cell, f.volume_m3)),
            )
            .collect();
        let mut outgoing = BTreeMap::<u32, f64>::new();
        let volumes: BTreeMap<_, _> = moved
            .iter()
            .map(|c| {
                (
                    c.id,
                    c.cohorts
                        .iter()
                        .map(|c| c.inventory.volume_m3())
                        .sum::<f64>(),
                )
            })
            .collect();
        for &(low, high, volume) in &transfers {
            if !volume.is_finite() || !by_id.contains_key(&low) || !by_id.contains_key(&high) {
                return Err(AdapterError::InvalidInventory);
            }
            if volume == 0.0 {
                continue;
            }
            *outgoing
                .entry(if volume > 0.0 { low } else { high })
                .or_default() += volume.abs();
            for id in [low, high] {
                let c = &moved[by_id[&id]];
                after.changed_packets.insert((c.native_column, c.packet));
            }
        }
        for cell in &mut after.material {
            let fraction = outgoing.get(&cell.id).copied().unwrap_or(0.0) / volumes[&cell.id];
            if !fraction.is_finite() || !(0.0..1.0).contains(&fraction) {
                return Err(AdapterError::InvalidInventory);
            }
            cell.cohorts = moved[by_id[&cell.id]]
                .cohorts
                .iter()
                .map(|c| c.scaled(1.0 - fraction))
                .collect();
        }
        for (low, high, volume) in transfers {
            if volume == 0.0 {
                continue;
            }
            let (donor, receiver) = if volume > 0.0 {
                (low, high)
            } else {
                (high, low)
            };
            let fraction = volume.abs() / volumes[&donor];
            for cohort in &moved[by_id[&donor]].cohorts {
                let incoming = cohort.scaled(fraction);
                let destination = &mut after.material[by_id[&receiver]].cohorts;
                if let Some(existing) = destination.iter_mut().find(|c| c.id == incoming.id) {
                    existing.add(&incoming)?;
                } else {
                    destination.push(incoming);
                }
            }
        }
        let expected = before.inventory().values();
        let actual = after.inventory().values();
        for a in 0..7 {
            if (actual[a] - expected[a]).abs()
                > 256.0 * f64::EPSILON * actual[a].abs().max(expected[a].abs())
            {
                return Err(AdapterError::InvalidInventory);
            }
        }
        for cell in &mut after.material {
            cell.cohorts.sort_by_key(|c| c.id);
            let mid = 0.5
                * (mean_height(cell.lower_vertices, &after.geometry)
                    + mean_height(cell.upper_vertices, &after.geometry));
            for c in &mut cell.cohorts {
                for a in 0..3 {
                    c.central_moment_area_n_m2[a] -= mid * c.force_area_n_m3[a];
                    c.chemical_central_moment_area_n_m2[a] -= mid * c.chemical_force_area_n_m3[a];
                }
            }
            if let Some(transport) = cell.transport_vertices {
                for j in 0..3 {
                    after.geometry.vertices[transport[j]].position_m[2] = 0.5
                        * (after.geometry.vertices[cell.lower_vertices[j]].position_m[2]
                            + after.geometry.vertices[cell.upper_vertices[j]].position_m[2]);
                }
            }
        }
        after.rebuild_material()?;
        for face in &mut after.geometry.skin_faces {
            let cell = &after.material[by_id[&face.low_cell]];
            let transport = cell
                .transport_vertices
                .ok_or(AdapterError::InvalidNativeLayout)?;
            for i in 0..2 {
                let j = transport
                    .iter()
                    .position(|&v| v == face.velocity_vertices[i])
                    .ok_or(AdapterError::InvalidNativeLayout)?;
                face.thickness_m[i] = after.geometry.vertices[cell.upper_vertices[j]].position_m[2]
                    - after.geometry.vertices[cell.lower_vertices[j]].position_m[2];
            }
        }
        let trial_modes: BTreeMap<_, _> = trial
            .bending_modes
            .iter()
            .map(|m| {
                (
                    (
                        m.vertices[0].min(m.vertices[1]),
                        m.vertices[0].max(m.vertices[1]),
                    ),
                    m.normal_slope,
                )
            })
            .collect();
        after.geometry.bending_modes =
            mechanics::controls::initialize_bending_modes(after.geometry.view())?;
        for mode in &mut after.geometry.bending_modes {
            mode.normal_slope = *trial_modes
                .get(&(
                    mode.vertices[0].min(mode.vertices[1]),
                    mode.vertices[0].max(mode.vertices[1]),
                ))
                .ok_or(AdapterError::InvalidNativeLayout)?;
        }
        let triangles: Vec<_> = after
            .material
            .iter()
            .filter_map(|c| c.sheet_vertices)
            .collect();
        let samples: Result<Vec<_>, _> = source
            .constraints
            .iter()
            .map(|c| {
                c.components[0]
                    .iter()
                    .find_map(|t| match t.dof {
                        VelocityDof::Translation {
                            vertex,
                            component: 0,
                        } if t.coefficient == 1.0 => Some((c.vertex, vertex)),
                        _ => None,
                    })
                    .ok_or(AdapterError::InvalidNativeLayout)
            })
            .collect();
        after.geometry.constraints =
            coherent_trace_constraints(after.geometry.view(), &triangles, &samples?)?;
        after.geometry.generation = self.next_generation;
        self.next_generation = self
            .next_generation
            .checked_add(1)
            .ok_or(AdapterError::UnknownGeneration)?;
        let mapped = after.geometry.clone();
        self.states.insert(mapped.generation, after);
        Ok(mapped)
    }
}

impl ConservativeRemap for ProvisionalRemap {
    fn remap(
        &mut self,
        source: PatchView<'_>,
        trial: &MechanicsTrial,
    ) -> Result<RemappedPatch, MechanicsError> {
        self.map(source, trial).map_err(|e| {
            let mechanical = match e {
                AdapterError::Mechanics(e) => e,
                _ => MechanicsError::InvalidMaterial,
            };
            self.failure = Some(e);
            mechanical
        })
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SolveCost {
    pub active_patches: usize,
    pub material_cells: usize,
    pub retained_cohorts: usize,
    pub tetrahedra: usize,
    pub elapsed_seconds: f64,
}

#[derive(Clone, Debug)]
pub struct AcceptedPatch {
    pub state: PatchCheckpoint,
    pub trial: MechanicsTrial,
    pub cost: SolveCost,
}

#[derive(Clone, Debug)]
pub enum PatchAttempt {
    Accepted(AcceptedPatch),
    Subdivide(SubdivisionReason, SolveCost),
}

pub fn attempt_patch(
    state: &PatchCheckpoint,
    interval: StepInterval,
    accuracy: &MechanicsAccuracy,
    scratch: &mut MechanicsScratch,
) -> Result<PatchAttempt, AdapterError> {
    let start = Instant::now();
    let mut remap = ProvisionalRemap {
        states: BTreeMap::from([(state.geometry.generation, state.clone())]),
        next_generation: state
            .geometry
            .generation
            .checked_add(1)
            .ok_or(AdapterError::UnknownGeneration)?,
        failure: None,
    };
    let attempt = mechanics::attempt_step(
        state.geometry.view(),
        interval,
        accuracy,
        scratch,
        &mut remap,
    );
    let cost = SolveCost {
        active_patches: 1,
        material_cells: state.material.len(),
        retained_cohorts: state.material.iter().map(|c| c.cohorts.len()).sum(),
        tetrahedra: state.geometry.cells.len(),
        elapsed_seconds: start.elapsed().as_secs_f64(),
    };
    if let Some(error) = remap.failure {
        return Err(error);
    }
    match attempt? {
        StepAttempt::Accepted(accepted) => {
            let state = remap
                .states
                .remove(&accepted.remapped.generation)
                .ok_or(AdapterError::UnknownGeneration)?;
            Ok(PatchAttempt::Accepted(AcceptedPatch {
                state,
                trial: accepted.trial,
                cost,
            }))
        }
        StepAttempt::Subdivide(reason) => Ok(PatchAttempt::Subdivide(reason, cost)),
    }
}
