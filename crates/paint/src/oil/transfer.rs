use std::sync::Arc;

use super::chemistry::Cell;
use super::store::{Formulation, MaterialError, Packet};

#[derive(Clone, Debug, PartialEq)]
pub struct TransferCell {
    pub pigment: f64,
    pub binder: f64,
    pub solvent: f64,
    pub oxygen: f32,
    pub oxidation: f32,
    pub network: f32,
    pub inhibitor: f32,
    pub formulation: Arc<Formulation>,
}

impl TransferCell {
    pub fn from_cell(cell: Cell, formulation: Arc<Formulation>, area: f64) -> Self {
        Self {
            pigment: cell.pigment_um as f64 * area,
            binder: cell.binder_um as f64 * area,
            solvent: cell.solvent_um as f64 * area,
            oxygen: cell.oxygen,
            oxidation: cell.oxidation,
            network: cell.network,
            inhibitor: cell.inhibitor,
            formulation,
        }
    }

    pub fn volume(&self) -> f64 {
        self.pigment + self.binder + self.solvent
    }

    pub fn scale(&mut self, fraction: f64) {
        self.pigment *= fraction;
        self.binder *= fraction;
        self.solvent *= fraction;
    }

    pub(crate) fn valid(&self) -> bool {
        [self.pigment, self.binder, self.solvent]
            .iter()
            .all(|v| v.is_finite() && *v >= 0.0)
            && [self.oxygen, self.oxidation, self.network, self.inhibitor]
                .iter()
                .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
            && self.volume().is_finite()
            && self.volume() > 0.0
            && self.formulation.valid()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fragment {
    pub cells: Vec<TransferCell>,
    pub tension_n_m: [f32; 3],
    pub moment_n: [f32; 3],
    pub reference_area: f64,
    pub interface_damage: f32,
    pub provenance: u32,
    pub coverage: f32,
}

impl Fragment {
    pub fn new(cells: Vec<TransferCell>, coverage: f32) -> Self {
        Self {
            cells,
            coverage,
            tension_n_m: [0.0; 3],
            moment_n: [0.0; 3],
            reference_area: 1.0,
            interface_damage: 0.0,
            provenance: 0,
        }
    }

    pub(crate) fn from_packet(
        packet: Packet,
        coverage: f32,
        cells: Vec<TransferCell>,
        reference_area: f64,
    ) -> Self {
        Self {
            cells,
            coverage,
            tension_n_m: packet.tension_n_m,
            moment_n: packet.moment_n,
            reference_area,
            interface_damage: packet.interface_damage,
            provenance: packet.provenance,
        }
    }

    pub fn volume(&self) -> f64 {
        self.cells.iter().map(TransferCell::volume).sum()
    }

    pub(crate) fn valid(&self) -> bool {
        !self.cells.is_empty()
            && self.reference_area.is_finite()
            && self.reference_area > 0.0
            && self.cells.iter().all(TransferCell::valid)
            && self.volume().is_finite()
            && self.coverage.is_finite()
            && self.coverage > 0.0
            && self.coverage <= 1.0
            && self
                .tension_n_m
                .iter()
                .chain(self.moment_n.iter())
                .all(|v| v.is_finite())
            && self.interface_damage.is_finite()
            && (0.0..=1.0).contains(&self.interface_damage)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MaterialTransfer {
    pub fragments: Vec<Fragment>,
    pub free_solvent: f64,
}

impl MaterialTransfer {
    pub fn volume(&self) -> f64 {
        self.fragments.iter().map(Fragment::volume).sum::<f64>() + self.free_solvent
    }

    pub fn validate(&self) -> Result<(), MaterialError> {
        if self.fragments.iter().all(Fragment::valid)
            && self.free_solvent.is_finite()
            && self.free_solvent >= 0.0
            && self.volume().is_finite()
        {
            Ok(())
        } else {
            Err(MaterialError::Invalid("invalid material transfer"))
        }
    }

    pub fn append(&mut self, mut other: Self) {
        self.fragments.append(&mut other.fragments);
        self.free_solvent += other.free_solvent;
    }

    pub fn split(&mut self, fraction: f64) -> Result<Self, MaterialError> {
        if !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {
            return Err(MaterialError::Invalid("invalid transfer fraction"));
        }
        self.validate()?;
        if fraction == 1.0 {
            return Ok(std::mem::take(self));
        }
        if fraction == 0.0 {
            return Ok(Self::default());
        }
        if self.fragments.iter().any(|fragment| {
            fragment.reference_area * fraction == 0.0
                || fragment.reference_area * (1.0 - fraction) == 0.0
                || fragment.cells.iter().any(|cell| {
                    cell.pigment * fraction + cell.binder * fraction + cell.solvent * fraction
                        == 0.0
                        || cell.pigment * (1.0 - fraction)
                            + cell.binder * (1.0 - fraction)
                            + cell.solvent * (1.0 - fraction)
                            == 0.0
                })
        }) {
            return Err(MaterialError::Invalid("transfer split underflow"));
        }
        let mut result = self.clone();
        result.free_solvent *= fraction;
        self.free_solvent *= 1.0 - fraction;
        for fragment in &mut result.fragments {
            fragment.reference_area *= fraction;
            for cell in &mut fragment.cells {
                cell.scale(fraction);
            }
        }
        for fragment in &mut self.fragments {
            fragment.reference_area *= 1.0 - fraction;
            for cell in &mut fragment.cells {
                cell.scale(1.0 - fraction);
            }
        }
        Ok(result)
    }

    pub fn mix_mobile(&mut self, max_network: f32) -> Result<(), MaterialError> {
        self.validate()?;
        if !max_network.is_finite()
            || !(0.0..=1.0).contains(&max_network)
            || self
                .fragments
                .iter()
                .flat_map(|p| &p.cells)
                .any(|c| c.network > max_network)
        {
            return Err(MaterialError::Invalid(
                "mixing inaccessible coherent material",
            ));
        }
        if self.fragments.is_empty() {
            return Ok(());
        }
        let mut binder = 0.0;
        let mut body = 0.0;
        let mut capacity = 0.0;
        let mut oxygen = 0.0;
        let mut oxidation = 0.0;
        let mut network = 0.0;
        let mut inhibitor = 0.0;
        for fragment in &self.fragments {
            let weight: f64 = fragment.cells.iter().map(|c| c.pigment + c.binder).sum();
            body += weight;
            for cell in &fragment.cells {
                let k = cell.formulation.kinetics();
                let cap = cell.binder * k.oxygen_solubility + cell.solvent * k.solvent_solubility;
                capacity += cap;
                oxygen += cap * cell.oxygen as f64;
                binder += cell.binder;
                oxidation += cell.binder * cell.oxidation as f64;
                network += cell.binder * cell.network as f64;
                inhibitor += cell.binder * cell.inhibitor as f64;
            }
        }
        if ![
            binder, body, capacity, oxygen, oxidation, network, inhibitor,
        ]
        .iter()
        .all(|v| v.is_finite())
        {
            return Err(MaterialError::Invalid("mixing inventory overflow"));
        }
        for fragment in &mut self.fragments {
            for cell in &mut fragment.cells {
                if capacity > 0.0 {
                    cell.oxygen = (oxygen / capacity) as f32;
                }
                if binder > 0.0 {
                    cell.oxidation = (oxidation / binder) as f32;
                    cell.network = (network / binder) as f32;
                    cell.inhibitor = (inhibitor / binder) as f32;
                }
            }
        }
        Ok(())
    }

    pub fn coalesce_homogeneous(&mut self, max_network: f32) -> Result<(), MaterialError> {
        self.validate()?;
        if let Some(first) = self.fragments.first().and_then(|f| f.cells.first()) {
            if self
                .fragments
                .iter()
                .flat_map(|f| &f.cells)
                .any(|c| c.formulation != first.formulation)
            {
                return Err(MaterialError::Invalid(
                    "heterogeneous entrainment requires a composite formulation",
                ));
            }
        }
        self.mix_mobile(max_network)?;
        let mut fragments: Vec<Fragment> = Vec::new();
        for mut fragment in std::mem::take(&mut self.fragments) {
            let first = &fragment.cells[0];
            if fragment.cells.iter().any(|c| {
                c.oxygen != first.oxygen
                    || c.oxidation != first.oxidation
                    || c.network != first.network
                    || c.inhibitor != first.inhibitor
            }) {
                fragments.push(fragment);
                continue;
            }
            let mut cell = fragment.cells.remove(0);
            for other in fragment.cells.drain(..) {
                cell.pigment += other.pigment;
                cell.binder += other.binder;
                cell.solvent += other.solvent;
            }
            let unstressed = fragment.tension_n_m == [0.0; 3] && fragment.moment_n == [0.0; 3];
            if unstressed {
                fragment.reference_area = 1.0;
            }
            if let Some(existing) = fragments.iter_mut().find(|f| {
                unstressed
                    && f.tension_n_m == [0.0; 3]
                    && f.moment_n == [0.0; 3]
                    && f.provenance == fragment.provenance
                    && f.interface_damage == fragment.interface_damage
                    && f.cells.len() == 1
                    && f.cells[0].oxygen == cell.oxygen
                    && f.cells[0].oxidation == cell.oxidation
                    && f.cells[0].network == cell.network
                    && f.cells[0].inhibitor == cell.inhibitor
            }) {
                existing.cells[0].pigment += cell.pigment;
                existing.cells[0].binder += cell.binder;
                existing.cells[0].solvent += cell.solvent;
                existing.coverage = existing.coverage.max(fragment.coverage);
            } else {
                fragment.cells.push(cell);
                fragments.push(fragment);
            }
        }
        self.fragments = fragments;
        Ok(())
    }
}
