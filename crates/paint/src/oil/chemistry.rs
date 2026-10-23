use std::cell::RefCell;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cell {
    pub pigment_um: f32,
    pub binder_um: f32,
    pub solvent_um: f32,
    pub oxygen: f32,
    pub oxidation: f32,
    pub network: f32,
    pub inhibitor: f32,
    pub recipe: u32,
}

impl Cell {
    pub fn thickness_um(&self) -> f64 {
        self.pigment_um as f64 + self.binder_um as f64 + self.solvent_um as f64
    }

    pub fn oxygen_inventory(&self, material: &Kinetics) -> f64 {
        (self.binder_um as f64 * material.oxygen_solubility
            + self.solvent_um as f64 * material.solvent_solubility)
            * self.oxygen as f64
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Kinetics {
    pub oxygen_solubility: f64,
    pub solvent_solubility: f64,
    pub diffusivity_um2_s: f64,
    pub cured_diffusivity_ratio: f64,
    pub air_transfer_um_s: f64,
    pub oxidation_rate_s: f64,
    pub autocatalysis: f64,
    pub oxygen_demand: f64,
    pub inhibitor_rate_s: f64,
    pub inhibitor_oxygen_demand: f64,
    pub inhibition_strength: f64,
    pub network_rate_s: f64,
    pub solvent_evaporation_rate_s: f64,
    pub binder_code: u32,
    pub drier_code: u32,
    pub provenance_code: u32,
    pub revision: u32,
}

impl Default for Kinetics {
    fn default() -> Self {
        Self {
            oxygen_solubility: 0.002,
            solvent_solubility: 0.002,
            diffusivity_um2_s: 2.0,
            cured_diffusivity_ratio: 0.1,
            air_transfer_um_s: 0.1,
            oxidation_rate_s: 0.00002,
            autocatalysis: 2.0,
            oxygen_demand: 0.16,
            inhibitor_rate_s: 0.00001,
            inhibitor_oxygen_demand: 0.001,
            inhibition_strength: 12.0,
            network_rate_s: 0.00002,
            solvent_evaporation_rate_s: 0.0,
            binder_code: 0,
            drier_code: 0,
            provenance_code: 0,
            revision: 1,
        }
    }
}

impl Kinetics {
    pub fn validate(&self) -> Result<(), ChemistryError> {
        let nonnegative = [
            self.solvent_solubility,
            self.diffusivity_um2_s,
            self.air_transfer_um_s,
            self.oxidation_rate_s,
            self.autocatalysis,
            self.oxygen_demand,
            self.inhibitor_rate_s,
            self.inhibitor_oxygen_demand,
            self.inhibition_strength,
            self.network_rate_s,
            self.solvent_evaporation_rate_s,
        ];
        if nonnegative.iter().any(|v| !v.is_finite() || *v < 0.0)
            || !self.oxygen_solubility.is_finite()
            || self.oxygen_solubility <= 0.0
            || !self.cured_diffusivity_ratio.is_finite()
            || self.cured_diffusivity_ratio <= 0.0
            || self.cured_diffusivity_ratio > 1.0
            || !(self.oxidation_rate_s * (1.0 + self.autocatalysis)).is_finite()
        {
            return Err(ChemistryError::InvalidKinetics);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChemistryError {
    InvalidTime,
    InvalidAirActivity,
    InvalidKinetics,
    InvalidCell(usize),
    MissingRecipe(usize),
    NumericalFailure,
    InvalidObservation,
    InvalidFixture,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OxygenBudget {
    pub initial_oxygen: f64,
    pub final_oxygen: f64,
    pub air_exchange: f64,
    pub oxidation_consumed: f64,
    pub inhibitor_consumed: f64,
    pub evaporation_oxygen_outflow: f64,
    pub solvent_evaporated_um: f64,
    pub rounding_delta: f64,
    pub balance_error: f64,
    pub substeps: u64,
}

#[derive(Clone, Debug, Default)]
struct WorkCell {
    index: usize,
    pigment: f64,
    binder: f64,
    solvent: f64,
    oxygen: f64,
    oxidation: f64,
    network: f64,
    inhibitor: f64,
    capacity: f64,
    permeability: f64,
    right: f64,
    diagonal: f64,
    upper: f64,
    rhs: f64,
}

impl WorkCell {
    fn thickness(&self) -> f64 {
        self.pigment + self.binder + self.solvent
    }
}

#[derive(Clone, Debug, Default)]
pub struct Scratch {
    work: Vec<WorkCell>,
}

impl Scratch {
    pub fn with_capacity(cells: usize) -> Self {
        Self {
            work: Vec::with_capacity(cells),
        }
    }

    pub fn capacity(&self) -> usize {
        self.work.capacity()
    }
}

thread_local! {
    static SCRATCH: RefCell<Scratch> = RefCell::new(Scratch::default());
}

pub fn advance_column(
    cells: &mut [Cell],
    kinetics: &[Kinetics],
    dt_seconds: f64,
    air_activity: f64,
) -> Result<OxygenBudget, ChemistryError> {
    SCRATCH.with(|scratch| {
        advance_column_with_scratch(
            cells,
            kinetics,
            dt_seconds,
            air_activity,
            &mut scratch.borrow_mut(),
        )
    })
}

pub fn advance_column_with_scratch(
    cells: &mut [Cell],
    kinetics: &[Kinetics],
    dt_seconds: f64,
    air_activity: f64,
    scratch: &mut Scratch,
) -> Result<OxygenBudget, ChemistryError> {
    if !dt_seconds.is_finite() || dt_seconds < 0.0 {
        return Err(ChemistryError::InvalidTime);
    }
    if !air_activity.is_finite() || !(0.0..=1.0).contains(&air_activity) {
        return Err(ChemistryError::InvalidAirActivity);
    }
    scratch.work.clear();
    let mut budget = OxygenBudget::default();
    let mut step_limit: f64 = 600.0;
    for (index, cell) in cells.iter().enumerate() {
        let material = kinetics
            .get(cell.recipe as usize)
            .ok_or(ChemistryError::MissingRecipe(index))?;
        material.validate()?;
        if [cell.pigment_um, cell.binder_um, cell.solvent_um]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.0)
            || [cell.oxygen, cell.oxidation, cell.network, cell.inhibitor]
                .iter()
                .any(|v| !v.is_finite() || !(0.0..=1.0).contains(v))
            || (cell.binder_um == 0.0 && cell.solvent_um == 0.0 && cell.oxygen != 0.0)
            || !(cell.binder_um as f64 * material.oxygen_solubility
                + cell.solvent_um as f64 * material.solvent_solubility)
                .is_finite()
        {
            return Err(ChemistryError::InvalidCell(index));
        }
        let rate = (material.oxidation_rate_s * (1.0 + material.autocatalysis))
            .max(material.inhibitor_rate_s)
            .max(material.network_rate_s)
            .max(material.solvent_evaporation_rate_s);
        if rate > 0.0 {
            step_limit = step_limit.min(0.05 / rate);
        }
        budget.initial_oxygen += cell.oxygen_inventory(material);
        if cell.thickness_um() > 0.0 {
            scratch.work.push(WorkCell {
                index,
                pigment: cell.pigment_um as f64,
                binder: cell.binder_um as f64,
                solvent: cell.solvent_um as f64,
                oxygen: cell.oxygen as f64,
                oxidation: cell.oxidation as f64,
                network: cell.network as f64,
                inhibitor: cell.inhibitor as f64,
                ..WorkCell::default()
            });
        }
    }
    let work = &mut scratch.work;
    let n = work.len();
    let mut remaining = dt_seconds;
    while remaining > 0.0 && n > 0 {
        let dt = remaining.min(step_limit);
        if dt <= 0.0 || (remaining > dt && remaining - dt == remaining) {
            return Err(ChemistryError::NumericalFailure);
        }
        for w in work.iter_mut() {
            let m = &kinetics[cells[w.index].recipe as usize];
            w.capacity = w.binder * m.oxygen_solubility + w.solvent * m.solvent_solubility;
            w.permeability =
                m.diffusivity_um2_s * m.cured_diffusivity_ratio.powf(w.network) * w.capacity
                    / w.thickness();
            w.right = 0.0;
            if !w.capacity.is_finite() || !w.permeability.is_finite() {
                return Err(ChemistryError::NumericalFailure);
            }
        }
        for j in 0..n.saturating_sub(1) {
            if work[j].permeability > 0.0 && work[j + 1].permeability > 0.0 {
                work[j].right = 1.0
                    / (0.5 * work[j].thickness() / work[j].permeability
                        + 0.5 * work[j + 1].thickness() / work[j + 1].permeability);
            }
        }
        let top = &work[n - 1];
        let m = &kinetics[cells[top.index].recipe as usize];
        let surface = m.air_transfer_um_s * top.capacity / top.thickness();
        let air_conductance = if surface > 0.0 && top.permeability > 0.0 {
            1.0 / (1.0 / surface + 0.5 * top.thickness() / top.permeability)
        } else {
            0.0
        };
        for j in 0..n {
            let left = if j == 0 { 0.0 } else { work[j - 1].right };
            let right = work[j].right + if j + 1 == n { air_conductance } else { 0.0 };
            let w = &mut work[j];
            w.diagonal = w.capacity + dt * (left + right);
            w.upper = -dt * w.right;
            w.rhs = w.capacity * w.oxygen
                + if j + 1 == n {
                    dt * air_conductance * air_activity
                } else {
                    0.0
                };
            if w.capacity == 0.0 {
                w.diagonal = 1.0;
            }
            if j > 0 {
                let factor = -dt * left / work[j - 1].diagonal;
                work[j].diagonal -= factor * work[j - 1].upper;
                work[j].rhs -= factor * work[j - 1].rhs;
            }
        }
        work[n - 1].oxygen = work[n - 1].rhs / work[n - 1].diagonal;
        for j in (0..n - 1).rev() {
            work[j].oxygen = (work[j].rhs - work[j].upper * work[j + 1].oxygen) / work[j].diagonal;
        }
        budget.air_exchange += dt * air_conductance * (air_activity - work[n - 1].oxygen);
        for w in work.iter_mut() {
            let m = &kinetics[cells[w.index].recipe as usize];
            if w.capacity > 0.0 && w.binder > 0.0 && w.oxygen > 0.0 {
                let before = w.oxygen;
                let mut lo = 0.0;
                let mut hi = before;
                for _ in 0..48 {
                    let activity = 0.5 * (lo + hi);
                    let (dx, di) = reaction_extents(w, m, dt, activity);
                    let demand = w.binder * (m.oxygen_demand * dx + m.inhibitor_oxygen_demand * di);
                    if demand <= w.capacity * (before - activity) {
                        lo = activity;
                    } else {
                        hi = activity;
                    }
                }
                let (dx, di) = reaction_extents(w, m, dt, lo);
                let oxidation = w.binder * m.oxygen_demand * dx;
                let induction = w.binder * m.inhibitor_oxygen_demand * di;
                w.oxygen = before - (oxidation + induction) / w.capacity;
                w.oxidation += dx;
                w.inhibitor -= di;
                budget.oxidation_consumed += oxidation;
                budget.inhibitor_consumed += induction;
            }
            w.network += (w.oxidation - w.network).max(0.0) * -(-m.network_rate_s * dt).exp_m1();
        }
        let top = &mut work[n - 1];
        let m = &kinetics[cells[top.index].recipe as usize];
        let evaporated = top.solvent * -(-m.solvent_evaporation_rate_s * dt).exp_m1();
        top.solvent -= evaporated;
        budget.solvent_evaporated_um += evaporated;
        budget.evaporation_oxygen_outflow += evaporated * m.solvent_solubility * top.oxygen;
        if top.binder == 0.0 && top.solvent == 0.0 {
            top.oxygen = 0.0;
        }
        if work.iter().any(|w| {
            [w.oxygen, w.oxidation, w.network, w.inhibitor]
                .iter()
                .any(|v| !v.is_finite() || *v < -1e-12 || *v > 1.0 + 1e-12)
        }) {
            return Err(ChemistryError::NumericalFailure);
        }
        remaining -= dt;
        budget.substeps += 1;
    }
    let mut unrounded = 0.0;
    for w in work.iter() {
        let m = &kinetics[cells[w.index].recipe as usize];
        unrounded += (w.binder * m.oxygen_solubility + w.solvent * m.solvent_solubility) * w.oxygen;
    }
    for w in work.iter() {
        let cell = &mut cells[w.index];
        cell.oxygen = w.oxygen.clamp(0.0, 1.0) as f32;
        cell.oxidation = w.oxidation.clamp(0.0, 1.0) as f32;
        cell.network = w.network.clamp(0.0, 1.0) as f32;
        cell.inhibitor = w.inhibitor.clamp(0.0, 1.0) as f32;
        cell.solvent_um = w.solvent as f32;
        budget.final_oxygen += cell.oxygen_inventory(&kinetics[cell.recipe as usize]);
    }
    budget.rounding_delta = budget.final_oxygen - unrounded;
    budget.balance_error = unrounded - budget.initial_oxygen - budget.air_exchange
        + budget.oxidation_consumed
        + budget.inhibitor_consumed
        + budget.evaporation_oxygen_outflow;
    Ok(budget)
}

fn reaction_extents(w: &WorkCell, m: &Kinetics, dt: f64, activity: f64) -> (f64, f64) {
    let di = w.inhibitor * -(-m.inhibitor_rate_s * activity * dt).exp_m1();
    let inhibition = (-m.inhibition_strength * (w.inhibitor - di)).exp();
    let rate = m.oxidation_rate_s * (1.0 + m.autocatalysis * w.oxidation) * inhibition;
    let dx = (1.0 - w.oxidation) * -(-rate * activity * dt).exp_m1();
    (dx, di)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ObservationModel {
    pub contact_depth_um: f64,
    pub pressure_depth_um: f64,
    pub set_to_touch_network: f64,
    pub dry_to_touch_network: f64,
    pub dry_hard_network: f64,
    pub golden_touch_network: f64,
    pub maximum_solvent_fraction: f64,
}

impl Default for ObservationModel {
    fn default() -> Self {
        Self {
            contact_depth_um: 10.0,
            pressure_depth_um: 100.0,
            set_to_touch_network: 0.15,
            dry_to_touch_network: 0.35,
            dry_hard_network: 0.65,
            golden_touch_network: 0.35,
            maximum_solvent_fraction: 0.05,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Observations {
    pub set_to_touch: bool,
    pub dry_to_touch: bool,
    pub dry_hard: bool,
    pub golden_touch: bool,
    pub surface_network: f64,
    pub pressure_network: f64,
    pub minimum_network: f64,
}

pub fn observe(cells: &[Cell], model: &ObservationModel) -> Result<Observations, ChemistryError> {
    if !model.contact_depth_um.is_finite()
        || model.contact_depth_um <= 0.0
        || !model.pressure_depth_um.is_finite()
        || model.pressure_depth_um < model.contact_depth_um
        || [
            model.set_to_touch_network,
            model.dry_to_touch_network,
            model.dry_hard_network,
            model.golden_touch_network,
            model.maximum_solvent_fraction,
        ]
        .iter()
        .any(|v| !v.is_finite() || !(0.0..=1.0).contains(v))
        || model.set_to_touch_network > model.dry_to_touch_network
        || model.dry_to_touch_network > model.dry_hard_network
    {
        return Err(ChemistryError::InvalidObservation);
    }
    let mut depth: f64 = 0.0;
    let mut surface = 0.0;
    let mut pressure = 0.0;
    let mut solvent = 0.0;
    let mut minimum: f64 = 1.0;
    for (index, cell) in cells.iter().enumerate().rev() {
        if [cell.pigment_um, cell.binder_um, cell.solvent_um]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.0)
            || !cell.network.is_finite()
            || !(0.0..=1.0).contains(&cell.network)
        {
            return Err(ChemistryError::InvalidCell(index));
        }
        let h = cell.thickness_um();
        if h == 0.0 {
            continue;
        }
        let contact = h.min((model.contact_depth_um - depth).max(0.0));
        let press = h.min((model.pressure_depth_um - depth).max(0.0));
        surface += contact * cell.network as f64;
        pressure += press * cell.network as f64;
        solvent += contact * cell.solvent_um as f64 / h;
        minimum = minimum.min(cell.network as f64);
        depth += h;
    }
    if depth == 0.0 {
        return Ok(Observations::default());
    }
    surface /= depth.min(model.contact_depth_um);
    pressure /= depth.min(model.pressure_depth_um);
    solvent /= depth.min(model.contact_depth_um);
    let solvent_ok = solvent <= model.maximum_solvent_fraction;
    Ok(Observations {
        set_to_touch: solvent_ok && surface >= model.set_to_touch_network,
        dry_to_touch: solvent_ok && surface >= model.dry_to_touch_network,
        dry_hard: solvent_ok
            && surface >= model.dry_to_touch_network
            && pressure >= model.dry_hard_network,
        golden_touch: solvent_ok && surface >= model.golden_touch_network,
        surface_network: surface,
        pressure_network: pressure,
        minimum_network: minimum,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixtureId {
    S3Lwlo450,
    S3Ublo450,
    S3Lwso450,
    S3Ubso450,
    S2Golden76,
    S2Golden254,
}

pub const FIXTURES: [FixtureId; 6] = [
    FixtureId::S3Lwlo450,
    FixtureId::S3Ublo450,
    FixtureId::S3Lwso450,
    FixtureId::S3Ubso450,
    FixtureId::S2Golden76,
    FixtureId::S2Golden254,
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fixture {
    pub id: FixtureId,
    pub thickness_um: f64,
    pub binder_volume_fraction: f64,
    pub kinetics: Kinetics,
    pub initial_inhibitor: f32,
    pub observations: ObservationModel,
}

impl FixtureId {
    pub fn fixture(self) -> Fixture {
        let (h, pigment_mass, pigment_density, binder_code, provenance, oxidation, induction) =
            match self {
                Self::S3Lwlo450 => (450.0, 0.66, 6.81, 1, 1, 0.00003, 0.00003),
                Self::S3Ublo450 => (450.0, 0.61, 2.35, 1, 2, 0.00003, 0.000008),
                Self::S3Lwso450 => (450.0, 0.77, 6.81, 2, 3, 0.00002, 0.00002),
                Self::S3Ubso450 => (450.0, 0.49, 2.35, 2, 4, 0.00003, 0.000008),
                Self::S2Golden76 => (76.2, 0.0, 1.0, 1, 5, 0.00004, 0.00004),
                Self::S2Golden254 => (254.0, 0.0, 1.0, 1, 5, 0.00004, 0.00004),
            };
        let oil = (1.0 - pigment_mass) / 0.93;
        let binder_volume_fraction = if pigment_mass > 0.0 {
            oil / (oil + pigment_mass / pigment_density)
        } else {
            0.65
        };
        Fixture {
            id: self,
            thickness_um: h,
            binder_volume_fraction,
            kinetics: Kinetics {
                binder_code,
                drier_code: if pigment_density == 6.81 { 1 } else { 0 },
                provenance_code: provenance,
                oxidation_rate_s: oxidation,
                inhibitor_rate_s: induction,
                ..Kinetics::default()
            },
            initial_inhibitor: 1.0,
            observations: ObservationModel::default(),
        }
    }
}

impl Fixture {
    pub fn column(&self, count: usize) -> Result<Vec<Cell>, ChemistryError> {
        if count == 0
            || !self.thickness_um.is_finite()
            || self.thickness_um <= 0.0
            || !self.binder_volume_fraction.is_finite()
            || !(0.0..=1.0).contains(&self.binder_volume_fraction)
            || !self.initial_inhibitor.is_finite()
            || !(0.0..=1.0).contains(&self.initial_inhibitor)
        {
            return Err(ChemistryError::InvalidFixture);
        }
        self.kinetics.validate()?;
        let h = self.thickness_um / count as f64;
        Ok(vec![
            Cell {
                pigment_um: (h * (1.0 - self.binder_volume_fraction)) as f32,
                binder_um: (h * self.binder_volume_fraction) as f32,
                inhibitor: self.initial_inhibitor,
                ..Cell::default()
            };
            count
        ])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endpoint {
    SetToTouch,
    DryToTouch,
    DryHard,
    GoldenTouch,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CalendarTarget {
    pub fixture: FixtureId,
    pub endpoint: Endpoint,
    pub earliest_hours: Option<f64>,
    pub latest_hours: f64,
}

pub const CALENDAR_TARGETS: [CalendarTarget; 14] = [
    CalendarTarget {
        fixture: FixtureId::S3Lwlo450,
        endpoint: Endpoint::SetToTouch,
        earliest_hours: Some(63.0),
        latest_hours: 63.0,
    },
    CalendarTarget {
        fixture: FixtureId::S3Lwlo450,
        endpoint: Endpoint::DryToTouch,
        earliest_hours: Some(90.0),
        latest_hours: 90.0,
    },
    CalendarTarget {
        fixture: FixtureId::S3Lwlo450,
        endpoint: Endpoint::DryHard,
        earliest_hours: Some(134.0),
        latest_hours: 134.0,
    },
    CalendarTarget {
        fixture: FixtureId::S3Ublo450,
        endpoint: Endpoint::SetToTouch,
        earliest_hours: Some(213.0),
        latest_hours: 213.0,
    },
    CalendarTarget {
        fixture: FixtureId::S3Ublo450,
        endpoint: Endpoint::DryToTouch,
        earliest_hours: Some(242.0),
        latest_hours: 242.0,
    },
    CalendarTarget {
        fixture: FixtureId::S3Ublo450,
        endpoint: Endpoint::DryHard,
        earliest_hours: Some(284.0),
        latest_hours: 284.0,
    },
    CalendarTarget {
        fixture: FixtureId::S3Lwso450,
        endpoint: Endpoint::SetToTouch,
        earliest_hours: Some(112.0),
        latest_hours: 112.0,
    },
    CalendarTarget {
        fixture: FixtureId::S3Lwso450,
        endpoint: Endpoint::DryToTouch,
        earliest_hours: Some(157.0),
        latest_hours: 157.0,
    },
    CalendarTarget {
        fixture: FixtureId::S3Lwso450,
        endpoint: Endpoint::DryHard,
        earliest_hours: Some(207.0),
        latest_hours: 207.0,
    },
    CalendarTarget {
        fixture: FixtureId::S3Ubso450,
        endpoint: Endpoint::SetToTouch,
        earliest_hours: Some(234.0),
        latest_hours: 234.0,
    },
    CalendarTarget {
        fixture: FixtureId::S3Ubso450,
        endpoint: Endpoint::DryToTouch,
        earliest_hours: Some(261.0),
        latest_hours: 261.0,
    },
    CalendarTarget {
        fixture: FixtureId::S3Ubso450,
        endpoint: Endpoint::DryHard,
        earliest_hours: Some(305.0),
        latest_hours: 305.0,
    },
    CalendarTarget {
        fixture: FixtureId::S2Golden76,
        endpoint: Endpoint::GoldenTouch,
        earliest_hours: None,
        latest_hours: 48.0,
    },
    CalendarTarget {
        fixture: FixtureId::S2Golden254,
        endpoint: Endpoint::GoldenTouch,
        earliest_hours: Some(96.0),
        latest_hours: 144.0,
    },
];

impl CalendarTarget {
    pub fn residual_hours(&self, predicted: f64) -> f64 {
        if !predicted.is_finite() {
            return f64::INFINITY;
        }
        if let Some(earliest) = self.earliest_hours {
            if predicted < earliest {
                return predicted - earliest;
            }
        }
        (predicted - self.latest_hours).max(0.0)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CalendarPrediction {
    pub set_to_touch_hours: Option<f64>,
    pub dry_to_touch_hours: Option<f64>,
    pub dry_hard_hours: Option<f64>,
    pub golden_touch_hours: Option<f64>,
}

impl CalendarPrediction {
    pub fn at(&self, endpoint: Endpoint) -> Option<f64> {
        match endpoint {
            Endpoint::SetToTouch => self.set_to_touch_hours,
            Endpoint::DryToTouch => self.dry_to_touch_hours,
            Endpoint::DryHard => self.dry_hard_hours,
            Endpoint::GoldenTouch => self.golden_touch_hours,
        }
    }

    pub fn loss(&self, fixture: FixtureId) -> f64 {
        CALENDAR_TARGETS
            .iter()
            .filter(|t| t.fixture == fixture)
            .map(|t| {
                t.residual_hours(self.at(t.endpoint).unwrap_or(f64::INFINITY))
                    .powi(2)
            })
            .sum()
    }
}

pub fn predict_calendar(
    fixture: &Fixture,
    cells: usize,
    sample_seconds: f64,
    horizon_hours: f64,
) -> Result<CalendarPrediction, ChemistryError> {
    if !sample_seconds.is_finite()
        || sample_seconds <= 0.0
        || !horizon_hours.is_finite()
        || horizon_hours <= 0.0
        || !(horizon_hours * 3600.0).is_finite()
    {
        return Err(ChemistryError::InvalidTime);
    }
    let mut column = fixture.column(cells)?;
    let mut scratch = Scratch::with_capacity(cells);
    let mut prediction = CalendarPrediction::default();
    let mut elapsed = 0.0;
    let horizon = horizon_hours * 3600.0;
    loop {
        let observed = observe(&column, &fixture.observations)?;
        for (passed, time) in [
            (observed.set_to_touch, &mut prediction.set_to_touch_hours),
            (observed.dry_to_touch, &mut prediction.dry_to_touch_hours),
            (observed.dry_hard, &mut prediction.dry_hard_hours),
            (observed.golden_touch, &mut prediction.golden_touch_hours),
        ] {
            if passed && time.is_none() {
                *time = Some(elapsed / 3600.0);
            }
        }
        if elapsed >= horizon
            || prediction.dry_hard_hours.is_some() && prediction.golden_touch_hours.is_some()
        {
            break;
        }
        let dt = sample_seconds.min(horizon - elapsed);
        if elapsed + dt == elapsed {
            return Err(ChemistryError::NumericalFailure);
        }
        advance_column_with_scratch(&mut column, &[fixture.kinetics], dt, 1.0, &mut scratch)?;
        elapsed += dt;
    }
    Ok(prediction)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inert() -> Kinetics {
        Kinetics {
            oxidation_rate_s: 0.0,
            inhibitor_rate_s: 0.0,
            network_rate_s: 0.0,
            ..Kinetics::default()
        }
    }

    fn cell(binder: f32, oxygen: f32) -> Cell {
        Cell {
            binder_um: binder,
            oxygen,
            ..Cell::default()
        }
    }

    fn balanced(b: OxygenBudget) {
        let scale = b.initial_oxygen.abs()
            + b.air_exchange.abs()
            + b.oxidation_consumed
            + b.inhibitor_consumed
            + 1.0;
        assert!(b.balance_error.abs() < 1e-10 * scale, "{b:?}");
        let receipt = b.final_oxygen - b.initial_oxygen - b.air_exchange
            + b.oxidation_consumed
            + b.inhibitor_consumed
            + b.evaporation_oxygen_outflow
            - b.rounding_delta;
        assert!(receipt.abs() < 1e-10 * scale, "{b:?}");
    }

    #[test]
    fn cell_layout_is_32_bytes() {
        assert_eq!(std::mem::size_of::<Cell>(), 32);
        assert_eq!(std::mem::align_of::<Cell>(), 4);
    }

    #[test]
    fn finite_air_resistance_uses_surface_half_cell() {
        let m = inert();
        let mut cells = [cell(10.0, 0.0)];
        let dt = 100.0;
        let g = m.oxygen_solubility / (1.0 / m.air_transfer_um_s + 5.0 / m.diffusivity_um2_s);
        let expected = dt * g / (10.0 * m.oxygen_solubility + dt * g);
        let budget = advance_column(&mut cells, &[m], dt, 1.0).unwrap();
        assert!((cells[0].oxygen as f64 - expected).abs() < 1e-7);
        assert!(cells[0].oxygen < 1.0);
        balanced(budget);
    }

    #[test]
    fn partitioned_closed_transport_conserves_inventory() {
        let a = Kinetics {
            air_transfer_um_s: 0.0,
            diffusivity_um2_s: 100.0,
            ..inert()
        };
        let b = Kinetics {
            oxygen_solubility: 0.008,
            ..a
        };
        let mut cells = [
            cell(10.0, 1.0),
            Cell {
                recipe: 1,
                ..cell(30.0, 0.0)
            },
        ];
        let budget = advance_column(&mut cells, &[a, b], 10000.0, 0.0).unwrap();
        assert!((cells[0].oxygen - cells[1].oxygen).abs() < 1e-6);
        assert!((cells[0].oxygen as f64 - 0.02 / 0.26).abs() < 1e-6);
        assert!(
            (cells[1].oxygen as f64 * b.oxygen_solubility
                / (cells[0].oxygen as f64 * a.oxygen_solubility)
                - 4.0)
                .abs()
                < 1e-5
        );
        assert_eq!(budget.air_exchange, 0.0);
        balanced(budget);
    }

    #[test]
    fn harmonic_interface_resistance_is_conservative() {
        let a = Kinetics {
            air_transfer_um_s: 0.0,
            diffusivity_um2_s: 2.0,
            ..inert()
        };
        let b = Kinetics {
            diffusivity_um2_s: 0.02,
            oxygen_solubility: 0.008,
            ..a
        };
        let mut cells = [
            cell(10.0, 1.0),
            Cell {
                recipe: 1,
                ..cell(20.0, 0.0)
            },
        ];
        let g = 1.0
            / (5.0 / (a.diffusivity_um2_s * a.oxygen_solubility)
                + 10.0 / (b.diffusivity_um2_s * b.oxygen_solubility));
        let dt = 10.0;
        let c0 = 10.0 * a.oxygen_solubility;
        let c1 = 20.0 * b.oxygen_solubility;
        let flux = dt * g / (1.0 + dt * g * (1.0 / c0 + 1.0 / c1));
        balanced(advance_column(&mut cells, &[a, b], dt, 0.0).unwrap());
        assert!((cells[0].oxygen as f64 - (1.0 - flux / c0)).abs() < 1e-7);
        assert!((cells[1].oxygen as f64 - flux / c1).abs() < 1e-7);
    }

    #[test]
    fn air_can_remove_dissolved_oxygen() {
        let mut cells = [cell(20.0, 1.0)];
        let budget = advance_column(&mut cells, &[inert()], 600.0, 0.0).unwrap();
        assert!(budget.air_exchange < 0.0);
        assert!(cells[0].oxygen < 1.0);
        balanced(budget);
    }

    #[test]
    fn reaction_bounds_and_oxygen_receipt() {
        let mut cells = FixtureId::S3Lwlo450.fixture().column(16).unwrap();
        let initial = cells.clone();
        let budget = advance_column(&mut cells, &[Kinetics::default()], 86400.0, 1.0).unwrap();
        for (before, after) in initial.iter().zip(&cells) {
            assert_eq!(before.pigment_um, after.pigment_um);
            assert_eq!(before.binder_um, after.binder_um);
            for state in [
                after.oxygen,
                after.oxidation,
                after.network,
                after.inhibitor,
            ] {
                assert!((0.0..=1.0).contains(&state));
            }
            assert!(after.network <= after.oxidation);
        }
        assert!(budget.oxidation_consumed > 0.0);
        assert!(budget.inhibitor_consumed > 0.0);
        balanced(budget);
    }

    #[test]
    fn skin_does_not_freeze_depth_chemistry() {
        let m = Kinetics {
            diffusivity_um2_s: 0.2,
            oxidation_rate_s: 0.0001,
            network_rate_s: 0.0001,
            ..Kinetics::default()
        };
        let mut cells = vec![cell(15.0, 0.0); 30];
        balanced(advance_column(&mut cells, &[m], 4.0 * 86400.0, 1.0).unwrap());
        assert!(cells[29].network > cells[0].network + 0.1);
        let before = cells.clone();
        balanced(advance_column(&mut cells, &[m], 4.0 * 86400.0, 1.0).unwrap());
        assert!(cells[25].network > before[25].network);
        assert!(cells[0].network < cells[29].network);
    }

    #[test]
    fn induction_and_network_are_distinct_from_oxidation() {
        let m = Kinetics {
            air_transfer_um_s: 0.0,
            ..Kinetics::default()
        };
        let mut inhibited = [Cell {
            inhibitor: 1.0,
            ..cell(10.0, 1.0)
        }];
        let mut uninhibited = [cell(10.0, 1.0)];
        balanced(advance_column(&mut inhibited, &[m], 3600.0, 0.0).unwrap());
        balanced(advance_column(&mut uninhibited, &[m], 3600.0, 0.0).unwrap());
        assert!(inhibited[0].oxidation < uninhibited[0].oxidation);
        assert!(inhibited[0].inhibitor < 1.0);
        let mut oxidized = [Cell {
            oxidation: 0.9,
            network: 0.4,
            ..cell(10.0, 0.0)
        }];
        advance_column(&mut oxidized, &[m], 86400.0, 0.0).unwrap();
        assert_eq!(oxidized[0].oxidation, 0.9);
        assert!(oxidized[0].network > 0.4);
    }

    #[test]
    fn inventory_and_drier_rates_are_not_stiffness() {
        let m = Kinetics {
            air_transfer_um_s: 0.0,
            ..Kinetics::default()
        };
        let mut lean = [Cell {
            pigment_um: 10.0,
            ..cell(10.0, 1.0)
        }];
        let mut fat = [Cell {
            pigment_um: 10.0,
            ..cell(20.0, 1.0)
        }];
        let a = advance_column(&mut lean, &[m], 3600.0, 0.0).unwrap();
        let b = advance_column(&mut fat, &[m], 3600.0, 0.0).unwrap();
        assert!((b.oxidation_consumed / a.oxidation_consumed - 2.0).abs() < 1e-9);
        let mut catalyzed = [cell(10.0, 1.0)];
        let fast = Kinetics {
            oxidation_rate_s: 2.0 * m.oxidation_rate_s,
            ..m
        };
        advance_column(&mut catalyzed, &[fast], 3600.0, 0.0).unwrap();
        assert!(catalyzed[0].oxidation > lean[0].oxidation);
    }

    #[test]
    fn evaporation_has_a_separate_oxygen_receipt() {
        let m = Kinetics {
            diffusivity_um2_s: 0.0,
            solvent_evaporation_rate_s: 0.001,
            ..inert()
        };
        let mut cells = [Cell {
            pigment_um: 1.0,
            solvent_um: 10.0,
            oxygen: 1.0,
            ..Cell::default()
        }];
        let budget = advance_column(&mut cells, &[m], 1000.0, 0.0).unwrap();
        assert!(budget.solvent_evaporated_um > 0.0);
        assert!(budget.evaporation_oxygen_outflow > 0.0);
        assert_eq!(cells[0].pigment_um, 1.0);
        assert_eq!(cells[0].binder_um, 0.0);
        balanced(budget);
    }

    #[test]
    fn zero_cells_and_scratch_reuse() {
        let m = inert();
        let mut cells = [Cell::default(), cell(10.0, 0.0), Cell::default()];
        let mut scratch = Scratch::with_capacity(3);
        let cap = scratch.capacity();
        balanced(advance_column_with_scratch(&mut cells, &[m], 3600.0, 1.0, &mut scratch).unwrap());
        assert_eq!(scratch.capacity(), cap);
        assert_eq!(cells[0], Cell::default());
        assert_eq!(cells[2], Cell::default());
        assert_eq!(
            advance_column(&mut [], &[], 1.0, 1.0).unwrap(),
            OxygenBudget::default()
        );
        let before = cells;
        advance_column(&mut cells, &[m], 0.0, 1.0).unwrap();
        assert_eq!(cells, before);
    }

    #[test]
    fn invalid_input_is_atomic() {
        let mut cells = [
            cell(10.0, 0.0),
            Cell {
                recipe: 1,
                ..cell(10.0, 0.0)
            },
        ];
        let before = cells;
        assert_eq!(
            advance_column(&mut cells, &[inert()], 3600.0, 1.0),
            Err(ChemistryError::MissingRecipe(1))
        );
        assert_eq!(cells, before);
        assert_eq!(
            advance_column(&mut cells, &[inert()], f64::NAN, 1.0),
            Err(ChemistryError::InvalidTime)
        );
        assert_eq!(
            advance_column(&mut cells, &[inert()], 1.0, -1.0),
            Err(ChemistryError::InvalidAirActivity)
        );
        assert_eq!(
            Kinetics {
                oxygen_demand: -1.0,
                ..inert()
            }
            .validate(),
            Err(ChemistryError::InvalidKinetics)
        );
    }

    #[test]
    fn separate_touch_hard_and_depth_operators() {
        let model = ObservationModel::default();
        let cells = [
            cell(440.0, 0.0),
            Cell {
                oxidation: 0.9,
                network: 0.8,
                ..cell(10.0, 0.0)
            },
        ];
        let observed = observe(&cells, &model).unwrap();
        assert!(observed.set_to_touch && observed.dry_to_touch);
        assert!(!observed.dry_hard);
        assert_eq!(observed.minimum_network, 0.0);
        let mut cells = cells;
        cells[1].network = 0.2;
        let observed = observe(&cells, &model).unwrap();
        assert!(observed.set_to_touch);
        assert!(!observed.dry_to_touch);
        assert!(!observed.dry_hard);
    }

    #[test]
    fn calendar_fixture_source_observations() {
        assert_eq!(CALENDAR_TARGETS.len(), 14);
        for id in FIXTURES {
            let fixture = id.fixture();
            let column = fixture.column(18).unwrap();
            let total: f64 = column.iter().map(Cell::thickness_um).sum();
            assert!((total - fixture.thickness_um).abs() < 0.0001);
            assert_eq!(
                fixture.kinetics.binder_code,
                if matches!(id, FixtureId::S3Lwso450 | FixtureId::S3Ubso450) {
                    2
                } else {
                    1
                }
            );
            for target in CALENDAR_TARGETS.iter().filter(|t| t.fixture == id) {
                assert_eq!(target.residual_hours(target.latest_hours), 0.0);
                assert_eq!(target.residual_hours(target.latest_hours + 1.0), 1.0);
            }
        }
        assert_eq!(CALENDAR_TARGETS[0].latest_hours, 63.0);
        assert_eq!(CALENDAR_TARGETS[11].latest_hours, 305.0);
        assert_eq!(CALENDAR_TARGETS[12].earliest_hours, None);
        assert_eq!(CALENDAR_TARGETS[12].residual_hours(24.0), 0.0);
        assert_eq!(CALENDAR_TARGETS[13].residual_hours(120.0), 0.0);
        assert_eq!(CALENDAR_TARGETS[13].residual_hours(95.0), -1.0);
    }

    #[test]
    fn caller_partition_only_adds_storage_rounding() {
        let fixture = FixtureId::S3Lwlo450.fixture();
        let mut whole = fixture.column(8).unwrap();
        let mut split = whole.clone();
        advance_column(&mut whole, &[fixture.kinetics], 3600.0, 1.0).unwrap();
        for _ in 0..6 {
            advance_column(&mut split, &[fixture.kinetics], 600.0, 1.0).unwrap();
        }
        for (a, b) in whole.iter().zip(&split) {
            assert!((a.oxygen - b.oxygen).abs() < 5e-7);
            assert!((a.oxidation - b.oxidation).abs() < 5e-7);
            assert!((a.network - b.network).abs() < 5e-7);
            assert!((a.inhibitor - b.inhibitor).abs() < 5e-7);
        }
    }
}
