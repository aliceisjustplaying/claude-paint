use std::fmt;
use std::sync::Arc;

use super::chemistry::{Cell, Kinetics};
use super::transfer::{Fragment, MaterialTransfer, TransferCell};
use crate::wet::{Latent, Prop};

pub const TILE_SIZE: usize = 32;

#[derive(Clone, Debug, PartialEq)]
pub struct Formulation {
    latent: Latent,
    prop: Prop,
    kinetics: Kinetics,
    binders: [f64; 5],
}

impl Formulation {
    pub fn new(latent: Latent, prop: Prop, kinetics: Kinetics) -> Result<Self, MaterialError> {
        if kinetics.binder_code > 5 {
            return Err(MaterialError::Invalid(
                "mixed binders require explicit fractions",
            ));
        }
        let mut binders = [0.0; 5];
        binders[kinetics.binder_code.saturating_sub(1) as usize] = 1.0;
        Self::new_with_binders(latent, prop, kinetics, binders)
    }

    pub fn new_with_binders(
        mut latent: Latent,
        prop: Prop,
        kinetics: Kinetics,
        mut binders: [f64; 5],
    ) -> Result<Self, MaterialError> {
        let total = binders.iter().sum::<f64>();
        if !total.is_finite() || total <= 0.0 || binders.iter().any(|v| !v.is_finite() || *v < 0.0)
        {
            return Err(MaterialError::Invalid("invalid binder fractions"));
        }
        for v in &mut binders {
            if *v == 0.0 {
                *v = 0.0;
            } else if (total - 1.0).abs() > 1e-12 {
                *v /= total;
            }
        }
        latent.layers.clear();
        latent.material = None;
        latent
            .parts
            .sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.total_cmp(&b.1)));
        let mut end = 0;
        for i in 0..latent.parts.len() {
            let (id, amount) = latent.parts[i];
            if !amount.is_finite() || amount < 0.0 {
                return Err(MaterialError::Invalid("invalid pigment fraction"));
            }
            if amount == 0.0 {
                continue;
            }
            if end > 0 && latent.parts[end - 1].0 == id {
                latent.parts[end - 1].1 += amount;
            } else {
                latent.parts[end] = (id, amount);
                end += 1;
            }
        }
        latent.parts.truncate(end);
        let result = Self {
            latent,
            prop,
            kinetics,
            binders,
        };
        if !result.valid() {
            return Err(MaterialError::Invalid("invalid formulation"));
        }
        Ok(result)
    }

    pub fn latent(&self) -> &Latent {
        &self.latent
    }
    pub fn prop(&self) -> Prop {
        self.prop
    }
    pub fn kinetics(&self) -> Kinetics {
        self.kinetics
    }

    pub fn binders(&self) -> [f64; 5] {
        self.binders
    }

    pub(crate) fn valid(&self) -> bool {
        self.latent.layers.is_empty()
            && self.latent.material.is_none()
            && self.binders.iter().all(|v| v.is_finite() && *v >= 0.0)
            && (self.binders.iter().sum::<f64>() - 1.0).abs() <= 1e-12
            && self
                .latent
                .color
                .iter()
                .chain(self.prop.iter())
                .all(|v| v.is_finite())
            && self
                .latent
                .parts
                .iter()
                .all(|p| p.1.is_finite() && p.1 > 0.0)
            && self.latent.parts.windows(2).all(|p| p[0].0 < p[1].0)
            && kinetics_values(self.kinetics)
                .iter()
                .all(|v| v.is_finite() && *v >= 0.0)
            && self.kinetics.validate().is_ok()
    }
}

pub(crate) fn kinetics_values(k: Kinetics) -> [f64; 13] {
    [
        k.oxygen_solubility,
        k.solvent_solubility,
        k.diffusivity_um2_s,
        k.cured_diffusivity_ratio,
        k.air_transfer_um_s,
        k.oxidation_rate_s,
        k.autocatalysis,
        k.oxygen_demand,
        k.inhibitor_rate_s,
        k.inhibitor_oxygen_demand,
        k.inhibition_strength,
        k.network_rate_s,
        k.solvent_evaporation_rate_s,
    ]
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MaterialError {
    Invalid(&'static str),
    Unsupported(&'static str),
    OutOfBounds,
    Capacity,
}

impl fmt::Display for MaterialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(s) => f.write_str(s),
            Self::Unsupported(s) => write!(f, "unsupported material operation: {s}"),
            Self::OutOfBounds => f.write_str("material pixel out of bounds"),
            Self::Capacity => f.write_str("material arena capacity exceeded"),
        }
    }
}

impl std::error::Error for MaterialError {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Column {
    pub first_cell: u32,
    pub cell_count: u32,
    pub first_packet: u32,
    pub packet_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Packet {
    pub first_cell: u32,
    pub cell_count: u32,
    pub tension_n_m: [f32; 3],
    pub moment_n: [f32; 3],
    pub interface_damage: f32,
    pub provenance: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ClockState {
    pub seconds: f64,
    pub committed_seconds: f64,
    pub pending_seconds: f64,
    pub step: u64,
    pub seed: u64,
}

impl ClockState {
    pub(crate) fn valid(self) -> bool {
        [self.seconds, self.committed_seconds, self.pending_seconds]
            .iter()
            .all(|v| v.is_finite() && *v >= 0.0)
            && self.committed_seconds <= self.seconds
            && self.seconds <= self.pending_seconds
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Environment {
    pub temperature_k: f64,
    pub oxygen_activity: f64,
    pub relative_humidity: f64,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            temperature_k: 293.15,
            oxygen_activity: 1.0,
            relative_humidity: 0.5,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PixelState {
    pub support_z_um: f32,
    pub film: f32,
    pub gloss: f32,
    pub absorb: f32,
    pub ground_oil: f32,
    pub reflectance: [f32; 38],
    pub residual: [f32; 3],
    pub color: [f32; 3],
    pub stroke: u32,
    pub touched: u32,
    pub laid_id: u32,
    pub floor: f32,
    pub laid: f32,
    pub free_solvent_um: f32,
}

impl Default for PixelState {
    fn default() -> Self {
        Self {
            support_z_um: 0.0,
            film: 0.0,
            gloss: 0.0,
            absorb: 0.0,
            ground_oil: 0.0,
            reflectance: [1.0; 38],
            residual: [0.0; 3],
            color: [1.0; 3],
            stroke: 0,
            touched: 0,
            laid_id: 0,
            floor: 0.0,
            laid: 0.0,
            free_solvent_um: 0.0,
        }
    }
}

impl PixelState {
    pub(crate) fn valid(&self) -> bool {
        [
            self.support_z_um,
            self.film,
            self.gloss,
            self.absorb,
            self.ground_oil,
            self.floor,
            self.laid,
            self.free_solvent_um,
        ]
        .iter()
        .chain(self.reflectance.iter())
        .chain(self.residual.iter())
        .chain(self.color.iter())
        .all(|v| v.is_finite())
            && [
                self.film,
                self.absorb,
                self.ground_oil,
                self.laid,
                self.free_solvent_um,
            ]
            .iter()
            .all(|v| *v >= 0.0)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ColumnData {
    pub cells: Vec<Cell>,
    pub packets: Vec<Packet>,
    pub coverage: Vec<f32>,
    pub state: PixelState,
    pub coatings: Vec<InertCoating>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InertCoating {
    pub before_packet: u32,
    pub k: [f32; 3],
    pub s: [f32; 3],
    pub optical_coats: f32,
    pub thickness_um: f32,
}

impl InertCoating {
    pub(crate) fn valid(&self, packets: usize) -> bool {
        self.before_packet as usize <= packets
            && self
                .k
                .iter()
                .chain(&self.s)
                .chain([&self.optical_coats, &self.thickness_um])
                .all(|v| v.is_finite() && *v >= 0.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SparseCoating {
    pub pixel: u32,
    pub coating: InertCoating,
}

pub struct ColumnRef<'a> {
    pub cells: &'a [Cell],
    pub packets: &'a [Packet],
    pub coverage: &'a [f32],
    pub formulations: &'a [Arc<Formulation>],
    pub state: &'a PixelState,
    pub first_cell: u32,
    pub(crate) coatings: &'a [SparseCoating],
}

impl ColumnRef<'_> {
    pub fn inert_coatings(&self) -> impl Iterator<Item = &InertCoating> {
        self.coatings.iter().map(|entry| &entry.coating)
    }

    pub fn to_owned(&self) -> ColumnData {
        let mut packets = self.packets.to_vec();
        for p in &mut packets {
            p.first_cell -= self.first_cell;
        }
        ColumnData {
            cells: self.cells.to_vec(),
            packets,
            coverage: self.coverage.to_vec(),
            state: self.state.clone(),
            coatings: self.inert_coatings().copied().collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RemovalMode {
    Mobile { max_network: f32 },
    Coherent,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TakeRequest {
    pub amount_um: f32,
    pub mode: RemovalMode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DepositMode {
    Cover,
    Entraining { depth_um: f32, max_network: f32 },
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ContactSample {
    pub height_um: f32,
    pub inventory_um: f32,
    pub mobile_um: f32,
    pub coverage: f32,
    pub oxygen: f32,
    pub oxidation: f32,
    pub network: f32,
    pub inhibitor: f32,
    pub free_solvent_um: f32,
}

#[derive(Clone, Debug)]
pub(crate) struct Tile {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) columns: Vec<Column>,
    pub(crate) cells: Vec<Cell>,
    pub(crate) packets: Vec<Packet>,
    pub(crate) coverage: Vec<f32>,
    pub(crate) live_cells: usize,
    pub(crate) live_packets: usize,
    pub(crate) coatings: Vec<SparseCoating>,
    pub(crate) formulations: Vec<Arc<Formulation>>,
    pub(crate) states: Vec<PixelState>,
    pub(crate) clock: ClockState,
    pub(crate) generation: u64,
}

impl Tile {
    fn new(width: u32, height: u32) -> Self {
        let n = (width * height) as usize;
        Self {
            width,
            height,
            columns: vec![Column::default(); n],
            cells: Vec::new(),
            packets: Vec::new(),
            coverage: Vec::new(),
            live_cells: 0,
            live_packets: 0,
            coatings: Vec::new(),
            formulations: Vec::new(),
            states: vec![PixelState::default(); n],
            clock: ClockState::default(),
            generation: 0,
        }
    }

    pub(crate) fn read(&self, pixel: usize) -> ColumnRef<'_> {
        let c = self.columns[pixel];
        let a = c.first_cell as usize;
        let b = c.first_packet as usize;
        let first_coating = self
            .coatings
            .partition_point(|entry| entry.pixel < pixel as u32);
        let end_coating = self
            .coatings
            .partition_point(|entry| entry.pixel <= pixel as u32);
        ColumnRef {
            cells: &self.cells[a..a + c.cell_count as usize],
            packets: &self.packets[b..b + c.packet_count as usize],
            coverage: &self.coverage[b..b + c.packet_count as usize],
            formulations: &self.formulations,
            state: &self.states[pixel],
            first_cell: c.first_cell,
            coatings: &self.coatings[first_coating..end_coating],
        }
    }

    fn intern(&mut self, formulation: Arc<Formulation>) -> Result<u32, MaterialError> {
        if let Some(i) = self.formulations.iter().position(|f| **f == *formulation) {
            return Ok(i as u32);
        }
        let i = u32::try_from(self.formulations.len()).map_err(|_| MaterialError::Capacity)?;
        self.formulations
            .try_reserve(1)
            .map_err(|_| MaterialError::Capacity)?;
        self.formulations.push(formulation);
        Ok(i)
    }

    fn replace(&mut self, pixel: usize, data: ColumnData) -> Result<(), MaterialError> {
        validate_column(&data, self.formulations.len())?;
        let nc = u32::try_from(data.cells.len()).map_err(|_| MaterialError::Capacity)?;
        let np = u32::try_from(data.packets.len()).map_err(|_| MaterialError::Capacity)?;
        let previous = self.columns[pixel];
        let live_cells = self.live_cells
            .checked_sub(previous.cell_count as usize)
            .and_then(|n| n.checked_add(data.cells.len()))
            .ok_or(MaterialError::Capacity)?;
        let live_packets = self.live_packets
            .checked_sub(previous.packet_count as usize)
            .and_then(|n| n.checked_add(data.packets.len()))
            .ok_or(MaterialError::Capacity)?;
        let in_place = previous.cell_count == nc && previous.packet_count == np;
        let a = if in_place { previous.first_cell } else {
            u32::try_from(self.cells.len()).map_err(|_| MaterialError::Capacity)?
        };
        let b = if in_place { previous.first_packet } else {
            u32::try_from(self.packets.len()).map_err(|_| MaterialError::Capacity)?
        };
        a.checked_add(nc).ok_or(MaterialError::Capacity)?;
        b.checked_add(np).ok_or(MaterialError::Capacity)?;
        if !in_place {
            self.cells
                .try_reserve(data.cells.len())
                .map_err(|_| MaterialError::Capacity)?;
            self.packets
                .try_reserve(data.packets.len())
                .map_err(|_| MaterialError::Capacity)?;
            self.coverage
                .try_reserve(data.coverage.len())
                .map_err(|_| MaterialError::Capacity)?;
        }
        let first_coating = self
            .coatings
            .partition_point(|entry| entry.pixel < pixel as u32);
        let end_coating = self
            .coatings
            .partition_point(|entry| entry.pixel <= pixel as u32);
        self.coatings
            .try_reserve(data.coatings.len().saturating_sub(end_coating - first_coating))
            .map_err(|_| MaterialError::Capacity)?;
        if data.coatings.len() == end_coating - first_coating {
            for (entry, coating) in self.coatings[first_coating..end_coating]
                .iter_mut().zip(data.coatings)
            {
                entry.coating = coating;
            }
        } else {
            self.coatings.splice(
                first_coating..end_coating,
                data.coatings.into_iter().map(|coating| SparseCoating {
                    pixel: pixel as u32,
                    coating,
                }),
            );
        }
        if in_place {
            self.cells[a as usize..a as usize + nc as usize].copy_from_slice(&data.cells);
            for (target, mut packet) in self.packets[b as usize..b as usize + np as usize]
                .iter_mut().zip(data.packets)
            {
                packet.first_cell += a;
                *target = packet;
            }
            self.coverage[b as usize..b as usize + np as usize].copy_from_slice(&data.coverage);
        } else {
            self.cells.extend(data.cells);
            self.packets.extend(data.packets.into_iter().map(|mut p| {
                p.first_cell += a;
                p
            }));
            self.coverage.extend(data.coverage);
        }
        self.live_cells = live_cells;
        self.live_packets = live_packets;
        self.states[pixel] = data.state;
        self.columns[pixel] = Column {
            first_cell: a,
            cell_count: nc,
            first_packet: b,
            packet_count: np,
        };
        self.generation = self.generation.wrapping_add(1);
        Ok(())
    }

    pub(crate) fn compact(&mut self) -> Result<(), MaterialError> {
        let nc = self.live_cells;
        let np = self.live_packets;
        let mut cells = Vec::new();
        let mut packets = Vec::new();
        let mut coverage = Vec::new();
        cells
            .try_reserve_exact(nc)
            .map_err(|_| MaterialError::Capacity)?;
        packets
            .try_reserve_exact(np)
            .map_err(|_| MaterialError::Capacity)?;
        coverage
            .try_reserve_exact(np)
            .map_err(|_| MaterialError::Capacity)?;
        for c in &mut self.columns {
            let a = c.first_cell as usize;
            let b = c.first_packet as usize;
            let new_a = cells.len() as u32;
            let new_b = packets.len() as u32;
            cells.extend_from_slice(&self.cells[a..a + c.cell_count as usize]);
            for p in &self.packets[b..b + c.packet_count as usize] {
                let mut p = *p;
                p.first_cell = p.first_cell - c.first_cell + new_a;
                packets.push(p);
            }
            coverage.extend_from_slice(&self.coverage[b..b + c.packet_count as usize]);
            c.first_cell = new_a;
            c.first_packet = new_b;
        }
        self.cells = cells;
        self.packets = packets;
        self.coverage = coverage;
        Ok(())
    }

    fn finish(&mut self) -> Result<(), MaterialError> {
        if self.cells.len() > self.live_cells.saturating_mul(2).saturating_add(1024) {
            self.compact()?;
        }
        Ok(())
    }
}

pub(crate) fn validate_cell(c: &Cell, recipes: usize) -> bool {
    [c.pigment_um, c.binder_um, c.solvent_um]
        .iter()
        .all(|v| v.is_finite() && *v >= 0.0)
        && [c.oxygen, c.oxidation, c.network, c.inhibitor]
            .iter()
            .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
        && (c.pigment_um as f64 + c.binder_um as f64 + c.solvent_um as f64) > 0.0
        && (c.recipe as usize) < recipes
}

pub(crate) fn validate_column(data: &ColumnData, recipes: usize) -> Result<(), MaterialError> {
    let extra_height = validate_coatings(data.coatings.iter(), data.packets.len())?;
    if (data.state.support_z_um as f64 + extra_height).abs() > f32::MAX as f64 {
        return Err(MaterialError::Invalid("coating height overflow"));
    }
    validate_view(
        &ColumnRef {
            cells: &data.cells,
            packets: &data.packets,
            coverage: &data.coverage,
            formulations: &[],
            state: &data.state,
            first_cell: 0,
            coatings: &[],
        },
        recipes,
    )?;
    let mut height =
        data.state.support_z_um as f64 + data.state.free_solvent_um as f64 + extra_height;
    for (packet, &coverage) in data.packets.iter().zip(&data.coverage) {
        let first = packet.first_cell as usize;
        height += data.cells[first..first + packet.cell_count as usize]
            .iter()
            .map(Cell::thickness_um)
            .sum::<f64>()
            / coverage as f64;
    }
    if !height.is_finite() || height.abs() > f32::MAX as f64 {
        return Err(MaterialError::Invalid("coating height overflow"));
    }
    Ok(())
}

pub(crate) fn validate_view(data: &ColumnRef<'_>, recipes: usize) -> Result<(), MaterialError> {
    if !data.state.valid()
        || data.coverage.len() != data.packets.len()
        || !data.cells.iter().all(|c| validate_cell(c, recipes))
    {
        return Err(MaterialError::Invalid("invalid column fields"));
    }
    let mut end = 0usize;
    let mut inventory = data.state.free_solvent_um as f64;
    let mut height = data.state.support_z_um as f64
        + data.state.free_solvent_um as f64
        + validate_coatings(data.inert_coatings(), data.packets.len())?;
    for (p, coverage) in data.packets.iter().zip(data.coverage) {
        let start = p
            .first_cell
            .checked_sub(data.first_cell)
            .ok_or(MaterialError::Invalid("packet precedes column"))? as usize;
        if start != end
            || p.cell_count == 0
            || !coverage.is_finite()
            || *coverage <= 0.0
            || *coverage > 1.0
            || !p
                .tension_n_m
                .iter()
                .chain(p.moment_n.iter())
                .chain(std::iter::once(&p.interface_damage))
                .all(|v| v.is_finite())
            || !(0.0..=1.0).contains(&p.interface_damage)
        {
            return Err(MaterialError::Invalid("invalid packet partition"));
        }
        end = end
            .checked_add(p.cell_count as usize)
            .ok_or(MaterialError::Capacity)?;
        if end > data.cells.len() {
            return Err(MaterialError::Invalid("packet exceeds column"));
        }
        let amount: f64 = data.cells[start..end]
            .iter()
            .map(|c| c.pigment_um as f64 + c.binder_um as f64 + c.solvent_um as f64)
            .sum();
        inventory += amount;
        height += amount / *coverage as f64;
    }
    if end != data.cells.len()
        || !inventory.is_finite()
        || inventory > f32::MAX as f64
        || !height.is_finite()
        || height.abs() > f32::MAX as f64
    {
        return Err(MaterialError::Invalid("column inventory overflow"));
    }
    Ok(())
}

fn validate_coatings<'a>(
    coatings: impl Iterator<Item = &'a InertCoating>,
    packets: usize,
) -> Result<f64, MaterialError> {
    let mut end = 0;
    let mut height = 0.0;
    for coating in coatings {
        if !coating.valid(packets) || coating.before_packet < end {
            return Err(MaterialError::Invalid("invalid ordered inert coating"));
        }
        end = coating.before_packet;
        height += coating.thickness_um as f64;
    }
    Ok(height)
}

#[derive(Clone, Debug)]
pub struct FilmStore {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) pixel_area: f64,
    pub(crate) tiles: Vec<Arc<Tile>>,
    pub clock: ClockState,
    pub environment: Environment,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StoreStatistics {
    pub tiles: usize,
    pub shared_tiles: usize,
    pub live_cells: usize,
    pub allocated_cells: usize,
    pub live_packets: usize,
    pub allocated_packets: usize,
    pub formulations: usize,
    pub pigment: f64,
    pub binder: f64,
    pub solvent: f64,
    pub ground_oil: f64,
}

impl FilmStore {
    pub fn new(width: u32, height: u32, pixel_area: f64) -> Result<Self, MaterialError> {
        if width == 0 || height == 0 || !pixel_area.is_finite() || pixel_area <= 0.0 {
            return Err(MaterialError::Invalid("invalid film dimensions"));
        }
        if !(pixel_area * f32::MAX as f64 * width as f64 * height as f64).is_finite() {
            return Err(MaterialError::Invalid("absolute inventory scale overflow"));
        }
        let nx = width.div_ceil(TILE_SIZE as u32);
        let ny = height.div_ceil(TILE_SIZE as u32);
        let count = (nx as usize)
            .checked_mul(ny as usize)
            .ok_or(MaterialError::Capacity)?;
        let mut tiles = Vec::new();
        tiles
            .try_reserve_exact(count)
            .map_err(|_| MaterialError::Capacity)?;
        for y in 0..ny {
            for x in 0..nx {
                tiles.push(Arc::new(Tile::new(
                    (width - x * 32).min(32),
                    (height - y * 32).min(32),
                )));
            }
        }
        Ok(Self {
            width,
            height,
            pixel_area,
            tiles,
            clock: ClockState::default(),
            environment: Environment::default(),
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn pixel_area(&self) -> f64 {
        self.pixel_area
    }
    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    pub fn check_clock_advance(&self) -> Result<(), MaterialError> {
        if self.tiles.iter().any(|tile| {
            !tile.coatings.is_empty()
                && tile
                    .columns
                    .iter()
                    .enumerate()
                    .any(|(i, column)| column.cell_count > 0 && !tile.read(i).coatings.is_empty())
        }) {
            return Err(MaterialError::Unsupported(
                "oxygen transport in an inert-coated reactive column",
            ));
        }
        if self.tiles.iter().any(|tile| !tile.coatings.is_empty())
            && self.statistics().solvent > 0.0
        {
            return Err(MaterialError::Unsupported(
                "solvent transport with an inert-coated canvas",
            ));
        }
        Ok(())
    }

    pub fn locate(&self, pixel: usize) -> Result<(usize, usize), MaterialError> {
        let n = (self.width as usize)
            .checked_mul(self.height as usize)
            .ok_or(MaterialError::Capacity)?;
        if pixel >= n {
            return Err(MaterialError::OutOfBounds);
        }
        let x = pixel % self.width as usize;
        let y = pixel / self.width as usize;
        let tile = y / 32 * self.width.div_ceil(32) as usize + x / 32;
        Ok((tile, y % 32 * self.tiles[tile].width as usize + x % 32))
    }

    pub fn read_column(&self, pixel: usize) -> Result<ColumnRef<'_>, MaterialError> {
        let (tile, local) = self.locate(pixel)?;
        Ok(self.tiles[tile].read(local))
    }

    pub fn surface_height(&self, pixel: usize) -> Result<f32, MaterialError> {
        let c = self.read_column(pixel)?;
        let mut height = c.state.support_z_um as f64 + c.state.free_solvent_um as f64;
        for (packet, &coverage) in c.packets.iter().zip(c.coverage) {
            let first = (packet.first_cell - c.first_cell) as usize;
            height += c.cells[first..first + packet.cell_count as usize]
                .iter()
                .map(Cell::thickness_um)
                .sum::<f64>()
                / coverage as f64;
        }
        height += c
            .inert_coatings()
            .map(|coating| coating.thickness_um as f64)
            .sum::<f64>();
        Ok(height as f32)
    }

    pub fn append_inert_coating(
        &mut self,
        pixel: usize,
        mut coating: InertCoating,
    ) -> Result<(), MaterialError> {
        self.with_column_mut(pixel, |column, _| {
            coating.before_packet =
                u32::try_from(column.packets.len()).map_err(|_| MaterialError::Capacity)?;
            if !coating.valid(column.packets.len()) {
                return Err(MaterialError::Invalid("invalid inert coating"));
            }
            if coating.optical_coats == 0.0 && coating.thickness_um == 0.0 {
                return Ok(());
            }
            column
                .coatings
                .try_reserve(1)
                .map_err(|_| MaterialError::Capacity)?;
            column.coatings.push(coating);
            Ok(())
        })
    }

    pub fn sample(&self, pixel: usize) -> Result<ContactSample, MaterialError> {
        self.sample_with_limit(pixel, 1.0)
    }

    pub fn sample_with_limit(
        &self,
        pixel: usize,
        max_network: f32,
    ) -> Result<ContactSample, MaterialError> {
        if !max_network.is_finite() || !(0.0..=1.0).contains(&max_network) {
            return Err(MaterialError::Invalid("invalid mobility limit"));
        }
        let c = self.read_column(pixel)?;
        if !c.coatings.is_empty() {
            return Err(MaterialError::Unsupported(
                "contact with an inert coating has no constitutive law",
            ));
        }
        let mut result = ContactSample {
            height_um: c.state.support_z_um,
            free_solvent_um: c.state.free_solvent_um,
            ..ContactSample::default()
        };
        let mut height = c.state.support_z_um as f64 + c.state.free_solvent_um as f64;
        let mut inventory = 0.0;
        for (p, coverage) in c.packets.iter().zip(c.coverage) {
            let a = (p.first_cell - c.first_cell) as usize;
            let amount: f64 = c.cells[a..a + p.cell_count as usize]
                .iter()
                .map(|v| v.pigment_um as f64 + v.binder_um as f64 + v.solvent_um as f64)
                .sum();
            inventory += amount;
            height += amount / *coverage as f64;
            result.coverage = *coverage;
        }
        for cell in c.cells.iter().rev() {
            if cell.network > max_network {
                break;
            }
            result.mobile_um += cell.pigment_um + cell.binder_um + cell.solvent_um;
        }
        if let Some(top) = c.cells.last() {
            result.oxygen = top.oxygen;
            result.oxidation = top.oxidation;
            result.network = top.network;
            result.inhibitor = top.inhibitor;
        }
        result.inventory_um = inventory as f32;
        result.height_um = height as f32;
        Ok(result)
    }

    pub fn edit_tile(&mut self, tile: usize) -> Result<FilmEdit<'_>, MaterialError> {
        let area = self.pixel_area;
        let arc = self.tiles.get_mut(tile).ok_or(MaterialError::OutOfBounds)?;
        Ok(FilmEdit {
            tile: Arc::make_mut(arc),
            pixel_area: area,
        })
    }

    pub fn edit_tiles(&mut self, ids: &[usize]) -> Result<Vec<FilmEdit<'_>>, MaterialError> {
        if ids.windows(2).any(|pair| pair[0] >= pair[1])
            || ids.last().is_some_and(|id| *id >= self.tiles.len())
        {
            return Err(MaterialError::Invalid(
                "tile leases must be unique, sorted and in bounds",
            ));
        }
        let mut leases = Vec::new();
        leases
            .try_reserve_exact(ids.len())
            .map_err(|_| MaterialError::Capacity)?;
        let mut next = ids.iter().copied().peekable();
        for (id, tile) in self.tiles.iter_mut().enumerate() {
            if next.peek() == Some(&id) {
                leases.push(FilmEdit {
                    tile: Arc::make_mut(tile),
                    pixel_area: self.pixel_area,
                });
                next.next();
            }
        }
        Ok(leases)
    }

    pub fn with_column_mut<R>(
        &mut self,
        pixel: usize,
        f: impl FnOnce(&mut ColumnData, &[Arc<Formulation>]) -> Result<R, MaterialError>,
    ) -> Result<R, MaterialError> {
        let (tile, local) = self.locate(pixel)?;
        self.edit_tile(tile)?.with_column_mut(local, f)
    }

    pub fn with_state_mut<R>(
        &mut self,
        pixel: usize,
        f: impl FnOnce(&mut PixelState) -> Result<R, MaterialError>,
    ) -> Result<R, MaterialError> {
        let (tile, local) = self.locate(pixel)?;
        self.edit_tile(tile)?.with_state_mut(local, f)
    }

    pub fn take_top(
        &mut self,
        pixel: usize,
        request: TakeRequest,
    ) -> Result<MaterialTransfer, MaterialError> {
        let (tile, local) = self.locate(pixel)?;
        self.edit_tile(tile)?.take_top(local, request)
    }

    pub fn deposit(
        &mut self,
        pixel: usize,
        material: MaterialTransfer,
        mode: DepositMode,
    ) -> Result<(), MaterialError> {
        let (tile, local) = self.locate(pixel)?;
        self.edit_tile(tile)?.deposit(local, material, mode)
    }

    pub fn take_solvent(
        &mut self,
        pixel: usize,
        amount_um: f32,
    ) -> Result<MaterialTransfer, MaterialError> {
        let (tile, local) = self.locate(pixel)?;
        self.edit_tile(tile)?.take_solvent(local, amount_um)
    }

    pub fn compact(&mut self) -> Result<(), MaterialError> {
        for tile in &mut self.tiles {
            Arc::make_mut(tile).compact()?;
        }
        Ok(())
    }

    pub fn statistics(&self) -> StoreStatistics {
        let mut s = StoreStatistics {
            tiles: self.tiles.len(),
            ..StoreStatistics::default()
        };
        for t in &self.tiles {
            s.shared_tiles += usize::from(Arc::strong_count(t) > 1);
            s.allocated_cells += t.cells.len();
            s.allocated_packets += t.packets.len();
            s.formulations += t.formulations.len();
            s.live_cells += t.live_cells;
            s.live_packets += t.live_packets;
            for i in 0..t.columns.len() {
                for cell in t.read(i).cells {
                    s.pigment += cell.pigment_um as f64 * self.pixel_area;
                    s.binder += cell.binder_um as f64 * self.pixel_area;
                    s.solvent += cell.solvent_um as f64 * self.pixel_area;
                }
                s.solvent += t.states[i].free_solvent_um as f64 * self.pixel_area;
                s.ground_oil += t.states[i].ground_oil as f64 * self.pixel_area;
            }
        }
        s
    }
}

pub struct FilmEdit<'a> {
    tile: &'a mut Tile,
    pixel_area: f64,
}

impl FilmEdit<'_> {
    pub fn width(&self) -> u32 {
        self.tile.width
    }
    pub fn height(&self) -> u32 {
        self.tile.height
    }
    pub fn column_count(&self) -> usize {
        self.tile.columns.len()
    }
    pub fn read_column(&self, pixel: usize) -> Result<ColumnRef<'_>, MaterialError> {
        if pixel >= self.column_count() {
            return Err(MaterialError::OutOfBounds);
        }
        Ok(self.tile.read(pixel))
    }
    pub fn clock(&self) -> ClockState {
        self.tile.clock
    }
    pub fn clock_mut(&mut self) -> &mut ClockState {
        &mut self.tile.clock
    }
    pub fn formulations(&self) -> &[Arc<Formulation>] {
        &self.tile.formulations
    }
    pub fn generation(&self) -> u64 {
        self.tile.generation
    }

    pub fn with_column_mut<R>(
        &mut self,
        pixel: usize,
        f: impl FnOnce(&mut ColumnData, &[Arc<Formulation>]) -> Result<R, MaterialError>,
    ) -> Result<R, MaterialError> {
        if pixel >= self.tile.columns.len() {
            return Err(MaterialError::OutOfBounds);
        }
        self.tile.finish()?;
        let mut data = self.tile.read(pixel).to_owned();
        let result = f(&mut data, &self.tile.formulations)?;
        self.tile.replace(pixel, data)?;
        Ok(result)
    }

    pub fn with_state_mut<R>(
        &mut self,
        pixel: usize,
        f: impl FnOnce(&mut PixelState) -> Result<R, MaterialError>,
    ) -> Result<R, MaterialError> {
        let column = self.read_column(pixel)?;
        let mut state = column.state.clone();
        let result = f(&mut state)?;
        validate_view(&ColumnRef { state: &state, ..column }, self.tile.formulations.len())?;
        self.tile.states[pixel] = state;
        self.tile.generation = self.tile.generation.wrapping_add(1);
        Ok(result)
    }

    pub fn take_top(
        &mut self,
        pixel: usize,
        request: TakeRequest,
    ) -> Result<MaterialTransfer, MaterialError> {
        if pixel >= self.tile.columns.len() {
            return Err(MaterialError::OutOfBounds);
        }
        if request.amount_um == 0.0 {
            return Ok(MaterialTransfer::default());
        }
        let area = self.pixel_area;
        self.with_column_mut(pixel, |data, recipes| {
            take_data(data, recipes, area, request)
        })
    }

    pub fn take_solvent(
        &mut self,
        pixel: usize,
        amount_um: f32,
    ) -> Result<MaterialTransfer, MaterialError> {
        if !amount_um.is_finite() || amount_um < 0.0 {
            return Err(MaterialError::Invalid("invalid solvent removal"));
        }
        let area = self.pixel_area;
        if !self.read_column(pixel)?.coatings.is_empty() {
            return Err(MaterialError::Unsupported(
                "solvent transport through an inert coating",
            ));
        }
        self.with_state_mut(pixel, |state| {
            let before = state.free_solvent_um;
            let after = before - before.min(amount_um);
            let free_solvent = (before as f64 - after as f64) * area;
            if !free_solvent.is_finite() {
                return Err(MaterialError::Invalid("solvent inventory overflow"));
            }
            state.free_solvent_um = after;
            Ok(MaterialTransfer {
                fragments: Vec::new(),
                free_solvent,
            })
        })
    }

    pub fn deposit(
        &mut self,
        pixel: usize,
        mut material: MaterialTransfer,
        mode: DepositMode,
    ) -> Result<(), MaterialError> {
        material.validate()?;
        if pixel >= self.tile.columns.len() {
            return Err(MaterialError::OutOfBounds);
        }
        if material.fragments.is_empty() && material.free_solvent == 0.0 {
            return Ok(());
        }
        self.tile.finish()?;
        let mut data = self.tile.read(pixel).to_owned();
        if !data.coatings.is_empty()
            && (material.free_solvent > 0.0 || matches!(mode, DepositMode::Entraining { .. }))
        {
            return Err(MaterialError::Unsupported(
                "entrainment or solvent transport through an inert coating",
            ));
        }
        if let DepositMode::Entraining {
            depth_um,
            max_network,
        } = mode
        {
            if material
                .fragments
                .iter()
                .flat_map(|p| &p.cells)
                .any(|c| c.network > max_network)
            {
                return Err(MaterialError::Invalid(
                    "coherent incoming material cannot be entrained",
                ));
            }
            let mut base = take_data(
                &mut data,
                &self.tile.formulations,
                self.pixel_area,
                TakeRequest {
                    amount_um: depth_um,
                    mode: RemovalMode::Mobile { max_network },
                },
            )?;
            base.append(material);
            base.coalesce_homogeneous(max_network)?;
            material = base;
        }
        let old_recipes = self.tile.formulations.len();
        let result = (|| {
            for fragment in material.fragments {
                let first_cell =
                    u32::try_from(data.cells.len()).map_err(|_| MaterialError::Capacity)?;
                let count =
                    u32::try_from(fragment.cells.len()).map_err(|_| MaterialError::Capacity)?;
                for cell in &fragment.cells {
                    let recipe = self.tile.intern(cell.formulation.clone())?;
                    let mut c = Cell {
                        pigment_um: (cell.pigment / self.pixel_area) as f32,
                        binder_um: (cell.binder / self.pixel_area) as f32,
                        solvent_um: (cell.solvent / self.pixel_area) as f32,
                        oxygen: cell.oxygen,
                        oxidation: cell.oxidation,
                        network: cell.network,
                        inhibitor: cell.inhibitor,
                        recipe,
                    };
                    c.clear_empty_oxygen();
                    if !validate_cell(&c, self.tile.formulations.len()) {
                        return Err(MaterialError::Invalid("deposition inventory overflow"));
                    }
                    data.cells.push(c);
                }
                data.packets.push(Packet {
                    first_cell,
                    cell_count: count,
                    tension_n_m: fragment
                        .tension_n_m
                        .map(|n| (n as f64 * fragment.reference_area / self.pixel_area) as f32),
                    moment_n: fragment.moment_n.map(|m| {
                        (m as f64 * fragment.reference_area / self.pixel_area
                            * fragment.reference_area / self.pixel_area) as f32
                    }),
                    interface_damage: fragment.interface_damage,
                    provenance: fragment.provenance,
                });
                data.coverage.push(fragment.coverage);
            }
            data.state.free_solvent_um += (material.free_solvent / self.pixel_area) as f32;
            self.tile.replace(pixel, data)
        })();
        if result.is_err() {
            self.tile.formulations.truncate(old_recipes);
        }
        result
    }
}

fn take_data(
    data: &mut ColumnData,
    recipes: &[Arc<Formulation>],
    area: f64,
    request: TakeRequest,
) -> Result<MaterialTransfer, MaterialError> {
    if request.amount_um > 0.0 && !data.coatings.is_empty() {
        return Err(MaterialError::Unsupported(
            "removal from an inert-coated column",
        ));
    }
    if !request.amount_um.is_finite() || request.amount_um < 0.0 {
        return Err(MaterialError::Invalid("invalid removal amount"));
    }
    if let RemovalMode::Mobile { max_network } = request.mode {
        if !max_network.is_finite() || !(0.0..=1.0).contains(&max_network) {
            return Err(MaterialError::Invalid("invalid mobility limit"));
        }
    }
    let mut remaining = request.amount_um as f64;
    let mut result = MaterialTransfer::default();
    while remaining > 0.0 {
        let Some(packet) = data.packets.last().copied() else {
            break;
        };
        let coverage = *data.coverage.last().unwrap();
        let original_volume: f64 = data.cells[packet.first_cell as usize..]
            .iter()
            .map(|c| c.pigment_um as f64 + c.binder_um as f64 + c.solvent_um as f64)
            .sum();
        let mut removed = Vec::new();
        let mut blocked = false;
        while data.cells.len() > packet.first_cell as usize && remaining > 0.0 {
            let cell = *data.cells.last().unwrap();
            if let RemovalMode::Mobile { max_network } = request.mode {
                if cell.network > max_network {
                    blocked = true;
                    break;
                }
            }
            let volume = cell.pigment_um as f64 + cell.binder_um as f64 + cell.solvent_um as f64;
            let fraction = (remaining / volume).min(1.0);
            let mut lifted =
                TransferCell::from_cell(cell, recipes[cell.recipe as usize].clone(), area);
            if fraction == 1.0 {
                data.cells.pop();
            } else {
                let rest = data.cells.last_mut().unwrap();
                rest.pigment_um = (cell.pigment_um as f64 * (1.0 - fraction)) as f32;
                rest.binder_um = (cell.binder_um as f64 * (1.0 - fraction)) as f32;
                rest.solvent_um = (cell.solvent_um as f64 * (1.0 - fraction)) as f32;
                rest.clear_empty_oxygen();
                lifted.pigment = (cell.pigment_um as f64 - rest.pigment_um as f64) * area;
                lifted.binder = (cell.binder_um as f64 - rest.binder_um as f64) * area;
                lifted.solvent = (cell.solvent_um as f64 - rest.solvent_um as f64) * area;
                if rest.pigment_um == 0.0 && rest.binder_um == 0.0 && rest.solvent_um == 0.0 {
                    data.cells.pop();
                }
            }
            let actual = lifted.volume() / area;
            if actual == 0.0 {
                blocked = true;
                break;
            }
            removed.push(lifted);
            remaining = (remaining - actual).max(0.0);
        }
        if !removed.is_empty() {
            removed.reverse();
            let retained_volume: f64 = data.cells[packet.first_cell as usize..]
                .iter()
                .map(|c| c.pigment_um as f64 + c.binder_um as f64 + c.solvent_um as f64)
                .sum();
            let fraction = retained_volume / original_volume;
            let thickness_m = original_volume / coverage as f64 * 1e-6;
            let mut lifted_packet = packet;
            let mut retained_packet = packet;
            for axis in 0..3 {
                let n = packet.tension_n_m[axis] as f64;
                let m = packet.moment_n[axis] as f64;
                let retained_n = n * fraction - 6.0 * m / thickness_m * fraction * (1.0 - fraction);
                retained_packet.tension_n_m[axis] = retained_n as f32;
                lifted_packet.tension_n_m[axis] = (n - retained_n) as f32;
                retained_packet.moment_n[axis] = (m * fraction.powi(3)) as f32;
                lifted_packet.moment_n[axis] = (m * (1.0 - fraction).powi(3)) as f32;
            }
            if let Some(remaining_packet) = data.packets.last_mut() {
                *remaining_packet = retained_packet;
            }
            result.fragments.push(Fragment::from_packet(
                lifted_packet,
                coverage,
                removed,
                area,
            ));
        }
        let count = data.cells.len() - packet.first_cell as usize;
        if count == 0 {
            data.packets.pop();
            data.coverage.pop();
        } else {
            data.packets.last_mut().unwrap().cell_count = count as u32;
        }
        if blocked {
            break;
        }
    }
    result.fragments.reverse();
    result.validate()?;
    Ok(result)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn recipe(code: u32) -> Arc<Formulation> {
        Arc::new(
            Formulation::new_with_binders(
                Latent::from_color([0.25; crate::wet::LAT]),
                [0.5; 10],
                Kinetics {
                    oxygen_solubility: 1.0 + code as f64,
                    solvent_solubility: 0.5,
                    diffusivity_um2_s: 0.01,
                    cured_diffusivity_ratio: 0.01,
                    air_transfer_um_s: 0.1,
                    oxidation_rate_s: 0.001,
                    autocatalysis: 1.0,
                    oxygen_demand: 1.0,
                    inhibitor_rate_s: 0.001,
                    inhibitor_oxygen_demand: 0.5,
                    inhibition_strength: 1.0,
                    network_rate_s: 0.001,
                    solvent_evaporation_rate_s: 0.001,
                    binder_code: code,
                    drier_code: 0,
                    provenance_code: code,
                    revision: 1,
                },
                [1.0, 0.0, 0.0, 0.0, 0.0],
            )
            .unwrap(),
        )
    }

    pub(crate) fn paint(amount: f64, code: u32, network: f32) -> MaterialTransfer {
        MaterialTransfer {
            fragments: vec![Fragment::new(
                vec![TransferCell {
                    pigment: amount * 0.5,
                    binder: amount * 0.5,
                    solvent: 0.0,
                    oxygen: 0.2,
                    oxidation: 0.3,
                    network,
                    inhibitor: 0.5,
                    formulation: recipe(code),
                }],
                0.75,
            )],
            free_solvent: 0.0,
        }
    }

    fn coherent(amount_um: f32) -> TakeRequest {
        TakeRequest {
            amount_um,
            mode: RemovalMode::Coherent,
        }
    }
    fn near(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5, "{a} != {b}");
    }

    #[test]
    fn material_record_layout() {
        assert_eq!(std::mem::size_of::<Column>(), 16);
        assert_eq!(std::mem::size_of::<Packet>(), 40);
        assert_eq!(std::mem::size_of::<Cell>(), 32);
        assert_eq!(std::mem::size_of::<PixelState>(), 220);
        assert_eq!(std::mem::align_of::<Cell>(), 4);
    }

    #[test]
    fn retained_layers_have_no_fixed_cap() {
        let mut s = FilmStore::new(1, 1, 1.0).unwrap();
        for i in 0..300 {
            s.deposit(0, paint(1.0, i, 0.8), DepositMode::Cover)
                .unwrap();
        }
        let c = s.read_column(0).unwrap();
        assert_eq!(c.cells.len(), 300);
        assert_eq!(c.packets.len(), 300);
        for (i, cell) in c.cells.iter().enumerate() {
            assert_eq!(
                c.formulations[cell.recipe as usize].kinetics().binder_code,
                i as u32
            );
            assert_eq!(cell.network, 0.8);
        }
        near(s.statistics().pigment + s.statistics().binder, 300.0);
    }

    #[test]
    fn unstressed_fragment_area_does_not_overflow_zero_moments() {
        let mut store = FilmStore::new(1, 1, 0.25).unwrap();
        let mut material = paint(1.0, 1, 0.0);
        material.fragments[0].reference_area = 1e200;
        store.deposit(0, material, DepositMode::Cover).unwrap();
        let packet = store.read_column(0).unwrap().packets[0];
        assert_eq!(packet.tension_n_m, [0.0; 3]);
        assert_eq!(packet.moment_n, [0.0; 3]);
    }

    #[test]
    fn top_split_exposes_the_retained_lower_chemistry() {
        let mut s = FilmStore::new(1, 1, 2.0).unwrap();
        s.deposit(0, paint(20.0, 1, 0.8), DepositMode::Cover)
            .unwrap();
        let mut top = paint(8.0, 2, 0.2);
        top.fragments[0].tension_n_m[2] = 0.13;
        top.fragments[0].interface_damage = 0.4;
        top.fragments[0].provenance = 92;
        s.deposit(0, top, DepositMode::Cover).unwrap();
        let lifted = s.take_top(0, coherent(1.0)).unwrap();
        near(lifted.volume(), 2.0);
        assert_eq!(lifted.fragments[0].interface_damage, 0.4);
        assert_eq!(lifted.fragments[0].provenance, 92);
        assert_eq!(lifted.fragments[0].cells[0].network, 0.2);
        assert_eq!(s.sample(0).unwrap().network, 0.2);
        let removed = s.take_top(0, coherent(3.0)).unwrap();
        near(removed.volume(), 6.0);
        assert_eq!(s.sample(0).unwrap().network, 0.8);
        assert_eq!(s.read_column(0).unwrap().cells.len(), 1);
    }

    #[test]
    fn transfer_conserves_inventory_across_pixel_areas() {
        let mut source = FilmStore::new(1, 1, 2.0).unwrap();
        let mut destination = FilmStore::new(1, 1, 0.5).unwrap();
        source
            .deposit(0, paint(24.0, 3, 0.7), DepositMode::Cover)
            .unwrap();
        source
            .deposit(
                0,
                MaterialTransfer {
                    free_solvent: 4.0,
                    ..MaterialTransfer::default()
                },
                DepositMode::Cover,
            )
            .unwrap();
        let before = source.statistics();
        let mut moved = source.take_top(0, coherent(4.0)).unwrap();
        moved.append(source.take_solvent(0, 1.0).unwrap());
        destination.deposit(0, moved, DepositMode::Cover).unwrap();
        let a = source.statistics();
        let b = destination.statistics();
        near(a.pigment + b.pigment, before.pigment);
        near(a.binder + b.binder, before.binder);
        near(a.solvent + b.solvent, before.solvent);
        assert_eq!(destination.sample(0).unwrap().network, 0.7);
        near(destination.sample(0).unwrap().inventory_um as f64, 16.0);
    }

    #[test]
    fn mixing_weights_chemistry_by_binder_and_oxygen_capacity() {
        let mut a = paint(10.0, 0, 0.2);
        let mut b = paint(8.0, 2, 0.6);
        let ca = &mut a.fragments[0].cells[0];
        ca.pigment = 8.0;
        ca.binder = 2.0;
        ca.oxygen = 0.2;
        ca.oxidation = 0.1;
        ca.inhibitor = 0.8;
        let cb = &mut b.fragments[0].cells[0];
        cb.pigment = 2.0;
        cb.binder = 6.0;
        cb.oxygen = 0.8;
        cb.oxidation = 0.9;
        cb.inhibitor = 0.4;
        a.append(b);
        let before = a.volume();
        a.mix_mobile(0.7).unwrap();
        near(a.volume(), before);
        for cell in a.fragments.iter().flat_map(|f| &f.cells) {
            near(cell.oxygen as f64, 0.74);
            near(cell.oxidation as f64, 0.7);
            near(cell.network as f64, 0.5);
            near(cell.inhibitor as f64, 0.5);
        }
        assert_ne!(
            a.fragments[0].cells[0].formulation,
            a.fragments[1].cells[0].formulation
        );
    }

    #[test]
    fn entrainment_never_mixes_the_inaccessible_base() {
        let mut s = FilmStore::new(1, 1, 1.0).unwrap();
        s.deposit(0, paint(10.0, 0, 0.9), DepositMode::Cover)
            .unwrap();
        let base = s.read_column(0).unwrap().cells[0];
        s.deposit(0, paint(4.0, 1, 0.4), DepositMode::Cover)
            .unwrap();
        s.deposit(
            0,
            paint(4.0, 1, 0.2),
            DepositMode::Entraining {
                depth_um: 100.0,
                max_network: 0.5,
            },
        )
        .unwrap();
        let c = s.read_column(0).unwrap();
        assert_eq!(c.cells[0], base);
        assert_eq!(c.cells.len(), 2);
        near(c.cells[1].network as f64, 0.3);
        near(s.statistics().binder + s.statistics().pigment, 18.0);
    }

    #[test]
    fn heterogeneous_entrainment_requires_an_explicit_composite_recipe() {
        let mut s = FilmStore::new(1, 1, 1.0).unwrap();
        s.deposit(0, paint(4.0, 1, 0.4), DepositMode::Cover)
            .unwrap();
        let before = s.read_column(0).unwrap().to_owned();
        assert!(
            s.deposit(
                0,
                paint(4.0, 2, 0.2),
                DepositMode::Entraining {
                    depth_um: 4.0,
                    max_network: 0.5
                }
            )
            .is_err()
        );
        assert_eq!(s.read_column(0).unwrap().to_owned(), before);
    }

    #[test]
    fn sub_ulp_pickup_never_invents_inventory() {
        let mut s = FilmStore::new(1, 1, 1.0).unwrap();
        s.deposit(0, paint(4.0, 1, 0.2), DepositMode::Cover)
            .unwrap();
        let before = s.statistics();
        let removed = s.take_top(0, coherent(1e-12)).unwrap();
        let after = s.statistics();
        assert_eq!(removed.volume(), 0.0);
        assert_eq!(before.pigment, after.pigment);
        assert_eq!(before.binder, after.binder);
    }

    #[test]
    fn top_cut_conserves_force_and_midplane_moment() {
        let mut s = FilmStore::new(1, 1, 2.0).unwrap();
        let mut material = paint(40.0, 1, 0.7);
        material.fragments[0].coverage = 1.0;
        s.deposit(0, material, DepositMode::Cover).unwrap();
        let n = [4.0, 2.0, -1.0];
        let m = [2e-5, -1e-5, 0.5e-5];
        s.with_column_mut(0, |c, _| {
            c.packets[0].tension_n_m = n;
            c.packets[0].moment_n = m;
            Ok(())
        })
        .unwrap();
        let removed = s.take_top(0, coherent(8.0)).unwrap();
        let low = s.read_column(0).unwrap().packets[0];
        let top = &removed.fragments[0];
        for axis in 0..3 {
            let force = low.tension_n_m[axis] as f64 + top.tension_n_m[axis] as f64;
            let moment = low.moment_n[axis] as f64 + top.moment_n[axis] as f64
                - 4e-6 * low.tension_n_m[axis] as f64
                + 6e-6 * top.tension_n_m[axis] as f64;
            assert!((force - n[axis] as f64).abs() < 1e-6);
            assert!((moment - m[axis] as f64).abs() < 1e-10);
            assert!((low.moment_n[axis] as f64 - m[axis] as f64 * 0.6f64.powi(3)).abs() < 1e-10);
        }
        let expected_n = top.tension_n_m;
        let expected_m = top.moment_n;
        let mut destination = FilmStore::new(1, 1, 1.0).unwrap();
        destination.deposit(0, removed, DepositMode::Cover).unwrap();
        let deposited = destination.read_column(0).unwrap().packets[0];
        for axis in 0..3 {
            near(
                deposited.tension_n_m[axis] as f64,
                expected_n[axis] as f64 * 2.0,
            );
            assert!(
                (deposited.moment_n[axis] as f64 - expected_m[axis] as f64 * 4.0).abs() < 1e-10
            );
        }
    }

    #[test]
    fn same_size_replacement_reuses_ranges_and_preserves_checkpoint_bytes() {
        let mut s = FilmStore::new(2, 1, 1.0).unwrap();
        s.deposit(0, paint(4.0, 1, 0.2), DepositMode::Cover).unwrap();
        s.deposit(1, paint(6.0, 2, 0.3), DepositMode::Cover).unwrap();
        let mut expected = s.clone();
        let ranges = s.tiles[0].columns.clone();
        let allocated = (s.tiles[0].cells.len(), s.tiles[0].packets.len(), s.tiles[0].coverage.len());
        s.with_column_mut(1, |column, _| {
            column.cells[0].network = 0.4;
            column.packets[0].tension_n_m[0] = 0.25;
            column.coverage[0] = 0.5;
            column.state.stroke = 17;
            Ok(())
        }).unwrap();
        let tile = Arc::make_mut(&mut expected.tiles[0]);
        let column = tile.columns[1];
        tile.cells[column.first_cell as usize].network = 0.4;
        tile.packets[column.first_packet as usize].tension_n_m[0] = 0.25;
        tile.coverage[column.first_packet as usize] = 0.5;
        tile.states[1].stroke = 17;
        tile.generation = tile.generation.wrapping_add(1);
        assert_eq!(s.tiles[0].columns, ranges);
        assert_eq!((s.tiles[0].cells.len(), s.tiles[0].packets.len(), s.tiles[0].coverage.len()), allocated);
        let mut actual_bytes = Vec::new();
        let mut expected_bytes = Vec::new();
        s.write_to(&mut actual_bytes).unwrap();
        expected.write_to(&mut expected_bytes).unwrap();
        assert_eq!(actual_bytes, expected_bytes);
        assert_eq!((s.tiles[0].live_cells, s.tiles[0].live_packets), (2, 2));
    }

    #[test]
    fn state_only_edits_are_atomic_and_leave_material_arenas_unchanged() {
        let mut s = FilmStore::new(1, 1, 1.0).unwrap();
        s.deposit(0, paint(4.0, 1, 0.2), DepositMode::Cover).unwrap();
        let before = s.read_column(0).unwrap().to_owned();
        let allocated = (s.tiles[0].cells.len(), s.tiles[0].packets.len());
        s.with_state_mut(0, |state| {
            state.stroke = 9;
            state.free_solvent_um = 2.0;
            Ok(())
        }).unwrap();
        let generation = s.tiles[0].generation;
        let mut bytes = Vec::new();
        s.write_to(&mut bytes).unwrap();
        assert!(s.with_state_mut(0, |state| {
            state.stroke = 10;
            Err::<(), _>(MaterialError::Invalid("failed state edit"))
        }).is_err());
        assert!(s.with_state_mut(0, |state| {
            state.free_solvent_um = f32::NAN;
            Ok(())
        }).is_err());
        let mut after_error = Vec::new();
        s.write_to(&mut after_error).unwrap();
        assert_eq!(after_error, bytes);
        assert_eq!(s.tiles[0].generation, generation);
        let removed = s.take_solvent(0, 1.0).unwrap();
        assert_eq!(removed.free_solvent, 1.0);
        assert_eq!((s.tiles[0].cells.len(), s.tiles[0].packets.len()), allocated);
        let after = s.read_column(0).unwrap();
        assert_eq!(after.cells, before.cells);
        assert_eq!(after.packets.len(), before.packets.len());
        assert_eq!(after.state.free_solvent_um, 1.0);
        assert_eq!((s.tiles[0].live_cells, s.tiles[0].live_packets), (1, 1));
    }

    #[test]
    fn live_counts_survive_growth_removal_compaction_and_load() {
        let mut s = FilmStore::new(2, 1, 1.0).unwrap();
        s.deposit(0, paint(4.0, 1, 0.2), DepositMode::Cover).unwrap();
        s.deposit(0, paint(4.0, 2, 0.3), DepositMode::Cover).unwrap();
        s.deposit(1, paint(4.0, 3, 0.4), DepositMode::Cover).unwrap();
        assert_eq!((s.tiles[0].live_cells, s.tiles[0].live_packets), (3, 3));
        s.take_top(0, coherent(4.0)).unwrap();
        s.compact().unwrap();
        assert_eq!((s.tiles[0].live_cells, s.tiles[0].live_packets), (2, 2));
        let mut bytes = Vec::new();
        s.write_to(&mut bytes).unwrap();
        let mut restored = FilmStore::read_from(&mut bytes.as_slice(), super::super::checkpoint::LoadBudget {
            max_raw_bytes: u64::MAX,
            max_resident_bytes: u64::MAX,
        }).unwrap();
        assert_eq!((restored.tiles[0].live_cells, restored.tiles[0].live_packets), (2, 2));
        restored.take_top(0, coherent(4.0)).unwrap();
        assert_eq!((restored.tiles[0].live_cells, restored.tiles[0].live_packets), (1, 1));
    }

    #[test]
    fn clone_rollback_detaches_only_the_edited_tile() {
        let mut live = FilmStore::new(65, 2, 1.0).unwrap();
        live.deposit(0, paint(8.0, 1, 0.2), DepositMode::Cover)
            .unwrap();
        let rollback = live.clone();
        assert!(Arc::ptr_eq(&live.tiles[0], &rollback.tiles[0]));
        live.deposit(1, paint(4.0, 2, 0.4), DepositMode::Cover)
            .unwrap();
        assert!(!Arc::ptr_eq(&live.tiles[0], &rollback.tiles[0]));
        assert!(Arc::ptr_eq(&live.tiles[1], &rollback.tiles[1]));
        assert_eq!(rollback.read_column(1).unwrap().cells.len(), 0);
        live = rollback.clone();
        assert_eq!(live.read_column(1).unwrap().cells.len(), 0);
        near(live.statistics().binder + live.statistics().pigment, 8.0);
    }

    #[test]
    fn tile_leases_exclude_duplicates_and_detach_before_parallel_work() {
        let mut s = FilmStore::new(64, 1, 1.0).unwrap();
        let rollback = s.clone();
        assert!(s.edit_tiles(&[0, 0]).is_err());
        assert!(s.edit_tiles(&[1, 0]).is_err());
        assert!(s.edit_tiles(&[2]).is_err());
        let mut leases = s.edit_tiles(&[0, 1]).unwrap();
        assert_eq!(leases.len(), 2);
        for (i, lease) in leases.iter_mut().enumerate() {
            lease
                .with_column_mut(0, |c, _| {
                    c.state.stroke = i as u32 + 7;
                    Ok(())
                })
                .unwrap();
        }
        drop(leases);
        assert_eq!(s.read_column(0).unwrap().state.stroke, 7);
        assert_eq!(s.read_column(32).unwrap().state.stroke, 8);
        assert_eq!(rollback.read_column(0).unwrap().state.stroke, 0);
        assert_eq!(rollback.read_column(32).unwrap().state.stroke, 0);
    }

    #[test]
    fn column_errors_leave_material_and_recipes_unchanged() {
        let mut s = FilmStore::new(2, 1, 1.0).unwrap();
        s.deposit(0, paint(4.0, 1, 0.2), DepositMode::Cover)
            .unwrap();
        let before = s.read_column(0).unwrap().to_owned();
        let result = s.with_column_mut(0, |c, _| {
            c.cells[0].recipe = u32::MAX;
            Ok(())
        });
        assert!(result.is_err());
        assert_eq!(s.read_column(0).unwrap().to_owned(), before);
        let mut invalid = paint(2.0, 99, 0.1);
        invalid.fragments[0].cells[0].pigment = f64::MAX;
        assert!(s.deposit(0, invalid, DepositMode::Cover).is_err());
        assert_eq!(s.tiles[0].formulations.len(), 1);
        assert_eq!(s.read_column(0).unwrap().to_owned(), before);
    }

    #[test]
    fn compaction_preserves_all_live_records_and_stable_recipe_keys() {
        let mut s = FilmStore::new(2, 1, 1.0).unwrap();
        for i in 0..12 {
            s.deposit(i % 2, paint(4.0, i as u32, 0.2), DepositMode::Cover)
                .unwrap();
        }
        s.take_top(0, coherent(3.0)).unwrap();
        s.with_column_mut(1, |c, _| {
            c.state.stroke = 81;
            c.state.ground_oil = 1.25;
            c.packets[0].tension_n_m[0] = 0.17;
            Ok(())
        })
        .unwrap();
        let a = s.read_column(0).unwrap().to_owned();
        let b = s.read_column(1).unwrap().to_owned();
        let stats = s.statistics();
        assert!(stats.allocated_cells > stats.live_cells);
        s.compact().unwrap();
        assert_eq!(s.read_column(0).unwrap().to_owned(), a);
        assert_eq!(s.read_column(1).unwrap().to_owned(), b);
        assert_eq!(s.statistics().allocated_cells, stats.live_cells);
        assert_eq!(s.statistics().allocated_packets, stats.live_packets);
        assert_eq!(s.statistics().formulations, stats.formulations);
    }

    #[test]
    fn transfer_split_preserves_resolved_recipes_and_history() {
        let mut transfer = paint(10.0, 7, 0.6);
        transfer.fragments[0].tension_n_m[0] = 0.2;
        transfer.fragments[0].interface_damage = 0.3;
        let part = transfer.split(0.25).unwrap();
        near(part.volume(), 2.5);
        near(transfer.volume(), 7.5);
        assert_eq!(part.fragments[0].cells[0].network, 0.6);
        assert_eq!(part.fragments[0].interface_damage, 0.3);
        assert!(Arc::ptr_eq(
            &part.fragments[0].cells[0].formulation,
            &transfer.fragments[0].cells[0].formulation
        ));
        assert!(transfer.mix_mobile(0.5).is_err());
    }
}
