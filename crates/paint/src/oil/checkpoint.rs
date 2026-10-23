use std::cmp::Ordering;
use std::io::{self, Read, Write};
use std::mem::size_of;
use std::sync::Arc;

use super::chemistry::{Cell, Kinetics};
use super::store::{
    ClockState, Column, Environment, FilmStore, Formulation, InertCoating, Packet, PixelState,
    SparseCoating, Tile, kinetics_values, validate_view,
};
use crate::wet::{LAT, Latent};

const MAGIC: &[u8; 8] = b"OILCOL02";
const COATED_MAGIC: &[u8; 8] = b"OILCOL03";
const HEADER_BYTES: u64 = 136;
const TILE_BYTES: u64 = 88;
const PIXEL_BYTES: u64 = 236;
const RECIPE_BYTES: u64 = (LAT as u64 + 10) * 4 + 18 * 8 + 4 * 4 + 8;

#[derive(Clone, Copy, Debug)]
pub struct LoadBudget {
    pub max_raw_bytes: u64,
    pub max_resident_bytes: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StoreCounts {
    pub tiles: u64,
    pub cells: u64,
    pub packets: u64,
    pub formulations: u64,
    pub parts: u64,
    pub coatings: u64,
    pub raw_bytes: u64,
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn add(a: u64, b: u64) -> io::Result<u64> {
    a.checked_add(b)
        .ok_or_else(|| invalid("store size overflow"))
}
fn mul(a: u64, b: u64) -> io::Result<u64> {
    a.checked_mul(b)
        .ok_or_else(|| invalid("store size overflow"))
}
fn native(n: u64) -> io::Result<usize> {
    usize::try_from(n).map_err(|_| invalid("store count exceeds address space"))
}

fn raw_size(pixels: u64, c: StoreCounts) -> io::Result<u64> {
    let coated = c.coatings > 0;
    let mut result = HEADER_BYTES + if coated { 8 } else { 0 };
    for (n, stride) in [
        (c.tiles, TILE_BYTES + if coated { 8 } else { 0 }),
        (pixels, PIXEL_BYTES),
        (c.cells, 32),
        (c.packets, 44),
        (c.formulations, RECIPE_BYTES),
        (c.parts, 12),
        (c.coatings, 44),
    ] {
        result = add(result, mul(n, stride)?)?;
    }
    Ok(result)
}

pub fn resident_size(pixels: u64, c: StoreCounts) -> io::Result<u64> {
    let mut result = size_of::<FilmStore>() as u64;
    for (n, stride) in [
        (
            c.tiles,
            (size_of::<Arc<Tile>>() + size_of::<Tile>() + 2 * size_of::<usize>()) as u64,
        ),
        (
            pixels,
            (size_of::<Column>() + 2 * size_of::<PixelState>()) as u64,
        ),
        (c.cells, 2 * size_of::<Cell>() as u64),
        (
            c.packets,
            2 * (size_of::<Packet>() + size_of::<f32>()) as u64,
        ),
        (
            c.formulations,
            (size_of::<Formulation>() + 4 * size_of::<usize>()) as u64,
        ),
        (c.parts, size_of::<(usize, f32)>() as u64),
        (c.coatings, 2 * size_of::<SparseCoating>() as u64),
    ] {
        result = add(result, mul(n, stride)?)?;
    }
    Ok(result)
}

fn reserve<T>(n: u64) -> io::Result<Vec<T>> {
    let mut result = Vec::new();
    result
        .try_reserve_exact(native(n)?)
        .map_err(|_| invalid("store allocation failed"))?;
    Ok(result)
}

fn formulation_cmp(a: &Formulation, b: &Formulation) -> Ordering {
    for (x, y) in a.binders().into_iter().zip(b.binders()) {
        let order = x.total_cmp(&y);
        if order != Ordering::Equal {
            return order;
        }
    }
    for (x, y) in a
        .latent()
        .color
        .iter()
        .chain(a.prop().iter())
        .zip(b.latent().color.iter().chain(b.prop().iter()))
    {
        let order = x.partial_cmp(y).unwrap_or(Ordering::Equal);
        if order != Ordering::Equal {
            return order;
        }
    }
    for (x, y) in a.latent().parts.iter().zip(&b.latent().parts) {
        let order =
            x.0.cmp(&y.0)
                .then_with(|| x.1.partial_cmp(&y.1).unwrap_or(Ordering::Equal));
        if order != Ordering::Equal {
            return order;
        }
    }
    let order = a.latent().parts.len().cmp(&b.latent().parts.len());
    if order != Ordering::Equal {
        return order;
    }
    for (x, y) in kinetics_values(a.kinetics())
        .iter()
        .zip(kinetics_values(b.kinetics()))
    {
        let order = x.partial_cmp(&y).unwrap_or(Ordering::Equal);
        if order != Ordering::Equal {
            return order;
        }
    }
    let a = a.kinetics();
    let b = b.kinetics();
    [a.binder_code, a.drier_code, a.provenance_code, a.revision].cmp(&[
        b.binder_code,
        b.drier_code,
        b.provenance_code,
        b.revision,
    ])
}

fn put_u32(w: &mut impl Write, v: u32) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
fn put_u64(w: &mut impl Write, v: u64) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
fn put_f32(w: &mut impl Write, v: f32) -> io::Result<()> {
    put_u32(w, if v == 0.0 { 0 } else { v.to_bits() })
}
fn put_f64(w: &mut impl Write, v: f64) -> io::Result<()> {
    put_u64(w, if v == 0.0 { 0 } else { v.to_bits() })
}

fn put_clock(w: &mut impl Write, c: ClockState) -> io::Result<()> {
    for v in [c.seconds, c.committed_seconds, c.pending_seconds] {
        put_f64(w, v)?;
    }
    put_u64(w, c.step)?;
    put_u64(w, c.seed)
}

fn put_state(w: &mut impl Write, s: &PixelState) -> io::Result<()> {
    for v in [s.support_z_um, s.film, s.gloss, s.absorb, s.ground_oil]
        .iter()
        .chain(s.reflectance.iter())
        .chain(s.residual.iter())
        .chain(s.color.iter())
    {
        put_f32(w, *v)?;
    }
    for v in [s.stroke, s.touched, s.laid_id] {
        put_u32(w, v)?;
    }
    for v in [s.floor, s.laid, s.free_solvent_um] {
        put_f32(w, v)?;
    }
    Ok(())
}

fn put_formulation(w: &mut impl Write, f: &Formulation) -> io::Result<()> {
    for v in f.latent().color {
        put_f32(w, v)?;
    }
    for v in f.prop() {
        put_f32(w, v)?;
    }
    for v in kinetics_values(f.kinetics()) {
        put_f64(w, v)?;
    }
    let k = f.kinetics();
    for v in [k.binder_code, k.drier_code, k.provenance_code, k.revision] {
        put_u32(w, v)?;
    }
    for v in f.binders() {
        put_f64(w, v)?;
    }
    put_u64(w, f.latent().parts.len() as u64)?;
    for (id, amount) in &f.latent().parts {
        put_u64(w, *id as u64)?;
        put_f32(w, *amount)?;
    }
    Ok(())
}

impl FilmStore {
    pub fn checkpoint_counts(&self) -> io::Result<StoreCounts> {
        let mut c = StoreCounts {
            tiles: self.tiles.len() as u64,
            ..StoreCounts::default()
        };
        for t in &self.tiles {
            c.coatings = add(c.coatings, t.coatings.len() as u64)?;
            for col in &t.columns {
                c.cells = add(c.cells, col.cell_count as u64)?;
                c.packets = add(c.packets, col.packet_count as u64)?;
            }
            c.formulations = add(c.formulations, t.formulations.len() as u64)?;
            for f in &t.formulations {
                c.parts = add(c.parts, f.latent().parts.len() as u64)?;
            }
        }
        c.raw_bytes = raw_size(mul(self.width as u64, self.height as u64)?, c)?;
        Ok(c)
    }

    pub fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        if !self.clock.valid() || !environment_valid(self.environment) {
            return Err(invalid("invalid store metadata"));
        }
        let counts = self.checkpoint_counts()?;
        let coated = counts.coatings > 0;
        w.write_all(if coated { COATED_MAGIC } else { MAGIC })?;
        put_u32(w, self.width)?;
        put_u32(w, self.height)?;
        put_f64(w, self.pixel_area)?;
        put_clock(w, self.clock)?;
        for v in [
            self.environment.temperature_k,
            self.environment.oxygen_activity,
            self.environment.relative_humidity,
        ] {
            put_f64(w, v)?;
        }
        for v in [
            counts.tiles,
            counts.cells,
            counts.packets,
            counts.formulations,
            counts.parts,
        ] {
            put_u64(w, v)?;
        }
        if coated {
            put_u64(w, counts.coatings)?;
        }
        put_u64(w, counts.raw_bytes)?;
        for (id, t) in self.tiles.iter().enumerate() {
            if !t.clock.valid() {
                return Err(invalid("invalid tile clock"));
            }
            put_u64(w, id as u64)?;
            put_u32(w, t.width)?;
            put_u32(w, t.height)?;
            let nc = t.columns.iter().map(|c| c.cell_count as u64).sum::<u64>();
            let np = t.columns.iter().map(|c| c.packet_count as u64).sum::<u64>();
            for v in [nc, np, t.formulations.len() as u64] {
                put_u64(w, v)?;
            }
            if coated {
                put_u64(w, t.coatings.len() as u64)?;
            }
            put_clock(w, t.clock)?;
            put_u64(w, t.generation)?;
            let mut order: Vec<usize> = (0..t.formulations.len()).collect();
            order.sort_by(|a, b| formulation_cmp(&t.formulations[*a], &t.formulations[*b]));
            let mut remap = vec![0u32; order.len()];
            for (new, old) in order.iter().enumerate() {
                remap[*old] = new as u32;
            }
            for old in order {
                put_formulation(w, &t.formulations[old])?;
            }
            let mut cell_offset = 0u32;
            let mut packet_offset = 0u32;
            for (i, c) in t.columns.iter().enumerate() {
                validate_view(&t.read(i), t.formulations.len())
                    .map_err(|_| invalid("invalid material column"))?;
                for v in [cell_offset, c.cell_count, packet_offset, c.packet_count] {
                    put_u32(w, v)?;
                }
                put_state(w, &t.states[i])?;
                cell_offset = cell_offset
                    .checked_add(c.cell_count)
                    .ok_or_else(|| invalid("tile arena overflow"))?;
                packet_offset = packet_offset
                    .checked_add(c.packet_count)
                    .ok_or_else(|| invalid("tile arena overflow"))?;
            }
            for i in 0..t.columns.len() {
                for c in t.read(i).cells {
                    for v in [
                        c.pigment_um,
                        c.binder_um,
                        c.solvent_um,
                        c.oxygen,
                        c.oxidation,
                        c.network,
                        c.inhibitor,
                    ] {
                        put_f32(w, v)?;
                    }
                    put_u32(w, remap[c.recipe as usize])?;
                }
            }
            let mut offset = 0u32;
            for i in 0..t.columns.len() {
                let column = t.read(i);
                for (p, cover) in column.packets.iter().zip(column.coverage) {
                    put_u32(w, p.first_cell - column.first_cell + offset)?;
                    put_u32(w, p.cell_count)?;
                    for v in p
                        .tension_n_m
                        .iter()
                        .chain(p.moment_n.iter())
                        .chain(std::iter::once(&p.interface_damage))
                    {
                        put_f32(w, *v)?;
                    }
                    put_u32(w, p.provenance)?;
                    put_f32(w, *cover)?;
                }
                offset += column.cells.len() as u32;
            }
            for (index, entry) in t.coatings.iter().enumerate() {
                if entry.pixel as usize >= t.columns.len()
                    || index > 0 && t.coatings[index - 1].pixel > entry.pixel
                {
                    return Err(invalid("invalid sparse coating column"));
                }
                put_u32(w, entry.pixel)?;
                put_u32(w, 1)?;
                put_u32(w, entry.coating.before_packet)?;
                for value in entry
                    .coating
                    .k
                    .iter()
                    .chain(&entry.coating.s)
                    .chain([&entry.coating.optical_coats, &entry.coating.thickness_um])
                {
                    put_f32(w, *value)?;
                }
            }
        }
        Ok(())
    }

    pub fn read_from(r: &mut impl Read, budget: LoadBudget) -> io::Result<Self> {
        let mut r = Decoder {
            reader: r,
            remaining: budget.max_raw_bytes,
            consumed: 0,
        };
        let coated = match &r.bytes::<8>()? {
            MAGIC => false,
            COATED_MAGIC => true,
            _ => return Err(invalid("unknown material schema")),
        };
        let width = r.u32()?;
        let height = r.u32()?;
        let pixel_area = r.f64()?;
        if width == 0 || height == 0 || !pixel_area.is_finite() || pixel_area <= 0.0 {
            return Err(invalid("invalid material dimensions"));
        }
        if !(pixel_area * f32::MAX as f64 * width as f64 * height as f64).is_finite() {
            return Err(invalid("absolute inventory scale overflow"));
        }
        let clock = r.clock()?;
        let environment = Environment {
            temperature_k: r.f64()?,
            oxygen_activity: r.f64()?,
            relative_humidity: r.f64()?,
        };
        if !environment_valid(environment) {
            return Err(invalid("invalid material environment"));
        }
        let declared = StoreCounts {
            tiles: r.u64()?,
            cells: r.u64()?,
            packets: r.u64()?,
            formulations: r.u64()?,
            parts: r.u64()?,
            coatings: if coated { r.u64()? } else { 0 },
            raw_bytes: r.u64()?,
        };
        let pixels = mul(width as u64, height as u64)?;
        let nx = width.div_ceil(32);
        let ny = height.div_ceil(32);
        if declared.tiles != mul(nx as u64, ny as u64)?
            || declared.raw_bytes != raw_size(pixels, declared)?
            || declared.raw_bytes > budget.max_raw_bytes
            || resident_size(pixels, declared)? > budget.max_resident_bytes
            || declared.packets > declared.cells
            || coated && declared.coatings == 0
        {
            return Err(invalid("material aggregate budget or counts invalid"));
        }
        r.remaining = declared
            .raw_bytes
            .checked_sub(r.consumed)
            .ok_or_else(|| invalid("invalid store byte count"))?;
        let mut tiles = reserve(declared.tiles)?;
        let mut actual = StoreCounts {
            tiles: declared.tiles,
            raw_bytes: declared.raw_bytes,
            ..StoreCounts::default()
        };
        for id in 0..declared.tiles {
            if r.u64()? != id {
                return Err(invalid("duplicate or unordered material tile"));
            }
            let tw = r.u32()?;
            let th = r.u32()?;
            let x = id % nx as u64;
            let y = id / nx as u64;
            if tw != (width - x as u32 * 32).min(32) || th != (height - y as u32 * 32).min(32) {
                return Err(invalid("invalid tile extent"));
            }
            let nc = r.u64()?;
            let np = r.u64()?;
            let nr = r.u64()?;
            let ni = if coated { r.u64()? } else { 0 };
            actual.cells = add(actual.cells, nc)?;
            actual.packets = add(actual.packets, np)?;
            actual.formulations = add(actual.formulations, nr)?;
            actual.coatings = add(actual.coatings, ni)?;
            if actual.cells > declared.cells
                || actual.packets > declared.packets
                || actual.formulations > declared.formulations
                || actual.coatings > declared.coatings
                || nc > u32::MAX as u64
                || np > u32::MAX as u64
                || nr > u32::MAX as u64
                || np > nc
            {
                return Err(invalid("tile counts exceed declared totals"));
            }
            let clock = r.clock()?;
            let generation = r.u64()?;
            let mut formulations: Vec<Arc<Formulation>> = reserve(nr)?;
            for _ in 0..nr {
                let f = r.formulation(&mut actual.parts, declared.parts)?;
                if formulations
                    .last()
                    .is_some_and(|previous| formulation_cmp(previous, &f) != Ordering::Less)
                {
                    return Err(invalid("duplicate or unordered formulation"));
                }
                formulations.push(Arc::new(f));
            }
            let count = tw as u64 * th as u64;
            let mut columns = reserve(count)?;
            let mut states = reserve(count)?;
            let mut cell_end = 0u64;
            let mut packet_end = 0u64;
            for _ in 0..count {
                let col = Column {
                    first_cell: r.u32()?,
                    cell_count: r.u32()?,
                    first_packet: r.u32()?,
                    packet_count: r.u32()?,
                };
                if col.first_cell as u64 != cell_end || col.first_packet as u64 != packet_end {
                    return Err(invalid("overlapping column ranges"));
                }
                cell_end = add(cell_end, col.cell_count as u64)?;
                packet_end = add(packet_end, col.packet_count as u64)?;
                if cell_end > nc || packet_end > np {
                    return Err(invalid("column exceeds tile arenas"));
                }
                columns.push(col);
                states.push(r.state()?);
            }
            if cell_end != nc || packet_end != np {
                return Err(invalid("unowned material records"));
            }
            let mut cells = reserve(nc)?;
            for _ in 0..nc {
                cells.push(Cell {
                    pigment_um: r.f32()?,
                    binder_um: r.f32()?,
                    solvent_um: r.f32()?,
                    oxygen: r.f32()?,
                    oxidation: r.f32()?,
                    network: r.f32()?,
                    inhibitor: r.f32()?,
                    recipe: r.u32()?,
                });
            }
            let mut packets = reserve(np)?;
            let mut coverage = reserve(np)?;
            for _ in 0..np {
                packets.push(Packet {
                    first_cell: r.u32()?,
                    cell_count: r.u32()?,
                    tension_n_m: [r.f32()?, r.f32()?, r.f32()?],
                    moment_n: [r.f32()?, r.f32()?, r.f32()?],
                    interface_damage: r.f32()?,
                    provenance: r.u32()?,
                });
                coverage.push(r.f32()?);
            }
            let mut coatings: Vec<SparseCoating> = reserve(ni)?;
            for _ in 0..ni {
                let pixel = r.u32()?;
                if pixel as u64 >= count || coatings.last().is_some_and(|entry| entry.pixel > pixel)
                {
                    return Err(invalid("invalid sparse coating column"));
                }
                if r.u32()? != 1 {
                    return Err(invalid("unknown inert coating tag"));
                }
                let coating = InertCoating {
                    before_packet: r.u32()?,
                    k: [r.f32()?, r.f32()?, r.f32()?],
                    s: [r.f32()?, r.f32()?, r.f32()?],
                    optical_coats: r.f32()?,
                    thickness_um: r.f32()?,
                };
                coatings.push(SparseCoating { pixel, coating });
            }
            let tile = Tile {
                width: tw,
                height: th,
                columns,
                cells,
                packets,
                coverage,
                coatings,
                formulations,
                states,
                clock,
                generation,
            };
            for (i, col) in tile.columns.iter().enumerate() {
                let mut expected = col.first_cell as u64;
                let end = add(expected, col.cell_count as u64)?;
                for p in tile.read(i).packets {
                    if p.first_cell as u64 != expected {
                        return Err(invalid("nonpartitioning packet range"));
                    }
                    expected = add(expected, p.cell_count as u64)?;
                    if expected > end {
                        return Err(invalid("packet exceeds owning column"));
                    }
                }
                if expected != end {
                    return Err(invalid("missing column packet"));
                }
                validate_view(&tile.read(i), tile.formulations.len())
                    .map_err(|_| invalid("invalid material record"))?;
            }
            tiles.push(Arc::new(tile));
        }
        if actual != declared || r.remaining != 0 || r.consumed != declared.raw_bytes {
            return Err(invalid("material totals or byte count mismatch"));
        }
        Ok(Self {
            width,
            height,
            pixel_area,
            tiles,
            clock,
            environment,
        })
    }
}

fn environment_valid(e: Environment) -> bool {
    e.temperature_k.is_finite()
        && e.temperature_k > 0.0
        && e.oxygen_activity.is_finite()
        && (0.0..=1.0).contains(&e.oxygen_activity)
        && e.relative_humidity.is_finite()
        && (0.0..=1.0).contains(&e.relative_humidity)
}

struct Decoder<'a, R> {
    reader: &'a mut R,
    remaining: u64,
    consumed: u64,
}

impl<R: Read> Decoder<'_, R> {
    fn bytes<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        self.remaining = self
            .remaining
            .checked_sub(N as u64)
            .ok_or_else(|| invalid("material section exceeds byte budget"))?;
        let mut bytes = [0; N];
        self.reader.read_exact(&mut bytes)?;
        self.consumed += N as u64;
        Ok(bytes)
    }
    fn u32(&mut self) -> io::Result<u32> {
        Ok(u32::from_le_bytes(self.bytes()?))
    }
    fn u64(&mut self) -> io::Result<u64> {
        Ok(u64::from_le_bytes(self.bytes()?))
    }
    fn f32(&mut self) -> io::Result<f32> {
        let value = f32::from_bits(self.u32()?);
        if value.is_finite() {
            Ok(value)
        } else {
            Err(invalid("nonfinite material value"))
        }
    }
    fn f64(&mut self) -> io::Result<f64> {
        let value = f64::from_bits(self.u64()?);
        if value.is_finite() {
            Ok(value)
        } else {
            Err(invalid("nonfinite material value"))
        }
    }
    fn clock(&mut self) -> io::Result<ClockState> {
        let c = ClockState {
            seconds: self.f64()?,
            committed_seconds: self.f64()?,
            pending_seconds: self.f64()?,
            step: self.u64()?,
            seed: self.u64()?,
        };
        if c.valid() {
            Ok(c)
        } else {
            Err(invalid("invalid canonical chemistry clock"))
        }
    }
    fn state(&mut self) -> io::Result<PixelState> {
        let support_z_um = self.f32()?;
        let film = self.f32()?;
        let gloss = self.f32()?;
        let absorb = self.f32()?;
        let ground_oil = self.f32()?;
        let mut reflectance = [0.0; 38];
        let mut residual = [0.0; 3];
        let mut color = [0.0; 3];
        for v in &mut reflectance {
            *v = self.f32()?;
        }
        for v in &mut residual {
            *v = self.f32()?;
        }
        for v in &mut color {
            *v = self.f32()?;
        }
        let state = PixelState {
            support_z_um,
            film,
            gloss,
            absorb,
            ground_oil,
            reflectance,
            residual,
            color,
            stroke: self.u32()?,
            touched: self.u32()?,
            laid_id: self.u32()?,
            floor: self.f32()?,
            laid: self.f32()?,
            free_solvent_um: self.f32()?,
        };
        if state.valid() {
            Ok(state)
        } else {
            Err(invalid("invalid support or solvent inventory"))
        }
    }
    fn formulation(&mut self, total_parts: &mut u64, max_parts: u64) -> io::Result<Formulation> {
        let mut color = [0.0; LAT];
        let mut prop = [0.0; 10];
        for v in &mut color {
            *v = self.f32()?;
        }
        for v in &mut prop {
            *v = self.f32()?;
        }
        let k = Kinetics {
            oxygen_solubility: self.f64()?,
            solvent_solubility: self.f64()?,
            diffusivity_um2_s: self.f64()?,
            cured_diffusivity_ratio: self.f64()?,
            air_transfer_um_s: self.f64()?,
            oxidation_rate_s: self.f64()?,
            autocatalysis: self.f64()?,
            oxygen_demand: self.f64()?,
            inhibitor_rate_s: self.f64()?,
            inhibitor_oxygen_demand: self.f64()?,
            inhibition_strength: self.f64()?,
            network_rate_s: self.f64()?,
            solvent_evaporation_rate_s: self.f64()?,
            binder_code: self.u32()?,
            drier_code: self.u32()?,
            provenance_code: self.u32()?,
            revision: self.u32()?,
        };
        let mut binders = [0.0; 5];
        for v in &mut binders {
            *v = self.f64()?;
        }
        if binders.iter().any(|v| *v < 0.0) || (binders.iter().sum::<f64>() - 1.0).abs() > 1e-12 {
            return Err(invalid("noncanonical binder fractions"));
        }
        let n = self.u64()?;
        *total_parts = add(*total_parts, n)?;
        if *total_parts > max_parts || mul(n, 12)? > self.remaining {
            return Err(invalid("pigment parts exceed declared budget"));
        }
        let mut parts: Vec<(usize, f32)> = reserve(n)?;
        for _ in 0..n {
            let id = native(self.u64()?)?;
            let amount = self.f32()?;
            if amount <= 0.0 || parts.last().is_some_and(|p| p.0 >= id) {
                return Err(invalid("noncanonical pigment parts"));
            }
            parts.push((id, amount));
        }
        let mut latent = Latent::from_color(color);
        latent.parts = parts;
        Formulation::new_with_binders(latent, prop, k, binders)
            .map_err(|_| invalid("invalid formulation parameters"))
    }
}

#[cfg(test)]
mod tests {
    use super::super::store::tests::{paint, recipe};
    use super::super::store::{DepositMode, RemovalMode, TakeRequest};
    use super::super::transfer::{Fragment, MaterialTransfer, TransferCell};
    use super::*;

    fn budget() -> LoadBudget {
        LoadBudget {
            max_raw_bytes: 8 * 1024 * 1024,
            max_resident_bytes: 16 * 1024 * 1024,
        }
    }
    fn encoded(s: &FilmStore) -> Vec<u8> {
        let mut bytes = Vec::new();
        s.write_to(&mut bytes).unwrap();
        bytes
    }
    fn specimen() -> FilmStore {
        let mut s = FilmStore::new(2, 1, 0.75).unwrap();
        s.deposit(0, paint(12.0, 9, 0.7), DepositMode::Cover)
            .unwrap();
        s.deposit(0, paint(3.0, 2, 0.2), DepositMode::Cover)
            .unwrap();
        s.deposit(1, paint(6.0, 4, 0.4), DepositMode::Cover)
            .unwrap();
        s.with_column_mut(0, |c, _| {
            c.packets[0].tension_n_m[0] = 0.125;
            c.packets[0].interface_damage = 0.25;
            c.packets[0].provenance = 71;
            c.state.support_z_um = -4.0;
            c.state.ground_oil = 0.3;
            c.state.reflectance[17] = 0.4;
            c.state.stroke = 13;
            c.state.free_solvent_um = 1.5;
            Ok(())
        })
        .unwrap();
        s.clock = ClockState {
            seconds: 3.5,
            committed_seconds: 3.0,
            pending_seconds: 4.0,
            step: 19,
            seed: 188,
        };
        *s.edit_tile(0).unwrap().clock_mut() = s.clock;
        s
    }

    #[test]
    fn material_stream_roundtrip_preserves_clock_parameters_and_history() {
        let s = specimen();
        let bytes = encoded(&s);
        assert_eq!(s.checkpoint_counts().unwrap().raw_bytes, bytes.len() as u64);
        let restored = FilmStore::read_from(&mut bytes.as_slice(), budget()).unwrap();
        assert_eq!(encoded(&restored), bytes);
        assert_eq!(restored.clock, s.clock);
        assert_eq!(restored.environment, s.environment);
        assert_eq!(restored.sample(0).unwrap(), s.sample(0).unwrap());
        assert_eq!(restored.statistics().binder, s.statistics().binder);
        assert_eq!(
            restored.read_column(0).unwrap().state,
            s.read_column(0).unwrap().state
        );
        assert_eq!(restored.read_column(0).unwrap().packets[0].provenance, 71);
    }

    #[test]
    fn same_tile_edits_have_deterministic_wire_order() {
        let mut a = FilmStore::new(2, 1, 1.0).unwrap();
        let mut b = a.clone();
        a.deposit(0, paint(4.0, 8, 0.2), DepositMode::Cover)
            .unwrap();
        a.deposit(1, paint(6.0, 3, 0.4), DepositMode::Cover)
            .unwrap();
        b.deposit(1, paint(6.0, 3, 0.4), DepositMode::Cover)
            .unwrap();
        b.deposit(0, paint(4.0, 8, 0.2), DepositMode::Cover)
            .unwrap();
        assert_eq!(encoded(&a), encoded(&b));
        let first_key = a.read_column(0).unwrap().cells[0].recipe;
        a.deposit(1, paint(1.0, 0, 0.1), DepositMode::Cover)
            .unwrap();
        assert_eq!(a.read_column(0).unwrap().cells[0].recipe, first_key);
    }

    #[test]
    fn canonical_formulations_use_all_parameters_and_strip_optical_layers() {
        let mut latent = recipe(1).latent().clone();
        latent.parts = vec![(7, 0.25), (2, 0.5), (7, 0.25)];
        latent.layers.push(crate::wet::Layer {
            share: 1.0,
            lat: Latent::from_color([1.0; LAT]),
            hide: [0.0; 10],
        });
        let formulation =
            Arc::new(Formulation::new(latent, [0.5; 10], recipe(1).kinetics()).unwrap());
        assert!(formulation.latent().layers.is_empty());
        assert_eq!(formulation.latent().parts, vec![(2, 0.5), (7, 0.5)]);
        let mut s = FilmStore::new(1, 1, 1.0).unwrap();
        let material = MaterialTransfer {
            fragments: vec![Fragment::new(
                vec![TransferCell {
                    pigment: 1.0,
                    binder: 1.0,
                    solvent: 0.0,
                    oxygen: 0.2,
                    oxidation: 0.3,
                    network: 0.4,
                    inhibitor: 0.5,
                    formulation: formulation.clone(),
                }],
                1.0,
            )],
            free_solvent: 0.0,
        };
        s.deposit(0, material.clone(), DepositMode::Cover).unwrap();
        s.deposit(0, material, DepositMode::Cover).unwrap();
        assert_eq!(s.statistics().formulations, 1);
        s.deposit(0, paint(1.0, 2, 0.2), DepositMode::Cover)
            .unwrap();
        assert_eq!(s.statistics().formulations, 2);
        let bytes = encoded(&s);
        let restored = FilmStore::read_from(&mut bytes.as_slice(), budget()).unwrap();
        assert_eq!(encoded(&restored), bytes);
    }

    #[test]
    fn formulations_enforce_the_chemistry_parameter_domain() {
        let f = recipe(1);
        let mut k = f.kinetics();
        k.cured_diffusivity_ratio = 2.0;
        assert!(Formulation::new(f.latent().clone(), f.prop(), k).is_err());
        k = f.kinetics();
        k.oxygen_solubility = -1.0;
        assert!(Formulation::new(f.latent().clone(), f.prop(), k).is_err());
    }

    #[test]
    fn wire_encoding_ignores_dead_arena_slots() {
        let mut s = specimen();
        s.take_top(
            0,
            TakeRequest {
                amount_um: 1.0,
                mode: RemovalMode::Coherent,
            },
        )
        .unwrap();
        let bytes = encoded(&s);
        assert!(s.statistics().allocated_cells > s.statistics().live_cells);
        s.compact().unwrap();
        assert_eq!(encoded(&s), bytes);
    }

    #[test]
    fn bounded_reader_rejects_corrupt_counts_before_payload_allocation() {
        struct HeaderOnly<'a> {
            data: &'a [u8],
            read: usize,
        }
        impl Read for HeaderOnly<'_> {
            fn read(&mut self, target: &mut [u8]) -> io::Result<usize> {
                let n = self.data.read(target)?;
                self.read += n;
                assert!(self.read <= HEADER_BYTES as usize);
                Ok(n)
            }
        }
        let mut bytes = encoded(&specimen());
        bytes[96..104].copy_from_slice(&u64::MAX.to_le_bytes());
        let mut source = HeaderOnly {
            data: &bytes,
            read: 0,
        };
        assert!(FilmStore::read_from(&mut source, budget()).is_err());
        assert_eq!(source.read, HEADER_BYTES as usize);
        let valid = encoded(&specimen());
        assert!(
            FilmStore::read_from(
                &mut valid.as_slice(),
                LoadBudget {
                    max_raw_bytes: valid.len() as u64 - 1,
                    ..budget()
                }
            )
            .is_err()
        );
        assert!(
            FilmStore::read_from(
                &mut valid.as_slice(),
                LoadBudget {
                    max_resident_bytes: 1,
                    ..budget()
                }
            )
            .is_err()
        );
    }

    #[test]
    fn malformed_ranges_recipes_chemistry_and_parts_are_rejected() {
        let mut s = FilmStore::new(2, 1, 1.0).unwrap();
        s.deposit(0, paint(4.0, 1, 0.2), DepositMode::Cover)
            .unwrap();
        s.deposit(1, paint(4.0, 1, 0.2), DepositMode::Cover)
            .unwrap();
        let bytes = encoded(&s);
        let columns = (HEADER_BYTES + TILE_BYTES + RECIPE_BYTES) as usize;
        let cells = columns + 2 * PIXEL_BYTES as usize;
        let packets = cells + 2 * 32;
        for (offset, value) in [
            (columns + PIXEL_BYTES as usize, 0u32),
            (cells + 28, 7u32),
            (cells + 16, 2.0f32.to_bits()),
            (cells + 12, f32::NAN.to_bits()),
            (packets, 1u32),
            (packets + 40, 0.0f32.to_bits()),
        ] {
            let mut corrupt = bytes.clone();
            corrupt[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
            assert!(
                FilmStore::read_from(&mut corrupt.as_slice(), budget()).is_err(),
                "offset {offset}"
            );
        }
        let mut corrupt = bytes.clone();
        let parts = (HEADER_BYTES + TILE_BYTES + RECIPE_BYTES - 8) as usize;
        corrupt[parts..parts + 8].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(FilmStore::read_from(&mut corrupt.as_slice(), budget()).is_err());
        for length in [0, 7, 135, 136, columns, cells, packets, bytes.len() - 1] {
            assert!(FilmStore::read_from(&mut &bytes[..length], budget()).is_err());
        }
    }

    #[test]
    fn material_reader_stops_at_its_section_boundary() {
        let s = specimen();
        let mut bytes = encoded(&s);
        bytes.extend_from_slice(b"outer-section");
        let mut reader = bytes.as_slice();
        FilmStore::read_from(&mut reader, budget()).unwrap();
        assert_eq!(reader, b"outer-section");
    }
}
