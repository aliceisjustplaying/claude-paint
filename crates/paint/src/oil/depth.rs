use super::chemistry::{Cell, Kinetics, Scratch, advance_column_with_scratch};
use super::constitutive::{Parameters, RefinementAccuracy, integrate_column};
use super::store::{ColumnData, Formulation, MaterialError};
use std::sync::Arc;

const STATE_ERROR: f64 = 1e-3;
const FRONT_VARIATION: f64 = 0.05;
const RIGIDITY_ERROR: f64 = 0.01;

pub trait CoupledAccuracy {
    fn evaluate(
        &self,
        coarse: &ColumnData,
        refined: &ColumnData,
        formulations: &[Arc<Formulation>],
    ) -> Result<RefinementAccuracy, MaterialError>;
}

fn rigidity_error(
    coarse: &[Cell],
    refined: &[Cell],
    formulations: &[Arc<Formulation>],
) -> Result<f64, MaterialError> {
    let parameters = Parameters::default();
    let mut a = integrate_column(coarse, formulations, &parameters)
        .map_err(|_| MaterialError::Invalid("invalid coarse rigidity"))?;
    let mut b = integrate_column(refined, formulations, &parameters)
        .map_err(|_| MaterialError::Invalid("invalid refined rigidity"))?;
    let reference = 0.5 * a.thickness_m.max(b.thickness_m);
    for rigidity in [&mut a, &mut b] {
        let delta = rigidity.z_ref_m - reference;
        for i in 0..3 {
            for j in 0..3 {
                rigidity.d[i][j] +=
                    2.0 * delta * rigidity.b[i][j] + delta * delta * rigidity.a[i][j];
                rigidity.b[i][j] += delta * rigidity.a[i][j];
            }
        }
        rigidity.z_ref_m = reference;
    }
    let norm = |m: &[[f64; 3]; 3]| {
        m.iter()
            .flatten()
            .copied()
            .map(f64::abs)
            .fold(0.0, f64::max)
    };
    let difference = |x: &[[f64; 3]; 3], y: &[[f64; 3]; 3]| {
        x.iter()
            .flatten()
            .zip(y.iter().flatten())
            .map(|(x, y)| (x - y).abs())
            .fold(0.0, f64::max)
    };
    let ratio = |error: f64, scale: f64| {
        if scale > 0.0 {
            error / scale
        } else if error == 0.0 {
            0.0
        } else {
            f64::INFINITY
        }
    };
    let extension = norm(&a.a).max(norm(&b.a));
    let bending = norm(&a.d).max(norm(&b.d));
    Ok(ratio(difference(&a.a, &b.a), extension)
        .max(ratio(
            difference(&a.b, &b.b),
            extension.sqrt() * bending.sqrt(),
        ))
        .max(ratio(difference(&a.d, &b.d), bending))
        .max(ratio(
            (a.neutral_bending_n_m - b.neutral_bending_n_m).abs(),
            a.neutral_bending_n_m.max(b.neutral_bending_n_m),
        )))
}

fn intensive(cell: &Cell) -> [f64; 6] {
    let thickness = cell.thickness_um().max(1e-30);
    [
        cell.oxygen as f64,
        cell.oxidation as f64,
        cell.network as f64,
        cell.inhibitor as f64,
        cell.pigment_um as f64 / thickness,
        cell.solvent_um as f64 / thickness,
    ]
}

fn merge(a: Cell, b: Cell, kinetics: &[Kinetics]) -> Cell {
    let material = &kinetics[a.recipe as usize];
    let oxygen_a = a.binder_um as f64 * material.oxygen_solubility
        + a.solvent_um as f64 * material.solvent_solubility;
    let oxygen_b = b.binder_um as f64 * material.oxygen_solubility
        + b.solvent_um as f64 * material.solvent_solubility;
    let weighted = |x: f32, y: f32, wa: f64, wb: f64| {
        if wa + wb > 0.0 {
            ((x as f64 * wa + y as f64 * wb) / (wa + wb)) as f32
        } else {
            (x + y) * 0.5
        }
    };
    Cell {
        pigment_um: a.pigment_um + b.pigment_um,
        binder_um: a.binder_um + b.binder_um,
        solvent_um: a.solvent_um + b.solvent_um,
        oxygen: weighted(a.oxygen, b.oxygen, oxygen_a, oxygen_b),
        oxidation: weighted(
            a.oxidation,
            b.oxidation,
            a.binder_um as f64,
            b.binder_um as f64,
        ),
        network: weighted(a.network, b.network, a.binder_um as f64, b.binder_um as f64),
        inhibitor: weighted(
            a.inhibitor,
            b.inhibitor,
            a.binder_um as f64,
            b.binder_um as f64,
        ),
        recipe: a.recipe,
    }
}

fn split(column: &ColumnData, refine: &[bool]) -> Result<ColumnData, MaterialError> {
    let mut result = column.clone();
    result.cells.clear();
    result
        .cells
        .try_reserve(column.cells.len())
        .map_err(|_| MaterialError::Capacity)?;
    for packet in &mut result.packets {
        let a = packet.first_cell as usize;
        let b = a + packet.cell_count as usize;
        packet.first_cell =
            u32::try_from(result.cells.len()).map_err(|_| MaterialError::Capacity)?;
        for i in a..b {
            let cell = column.cells[i];
            if refine[i] {
                let mut lower = cell;
                lower.pigment_um *= 0.5;
                lower.binder_um *= 0.5;
                lower.solvent_um *= 0.5;
                if lower.thickness_um() == 0.0 || lower.thickness_um() == cell.thickness_um() {
                    return Err(MaterialError::Invalid(
                        "depth accuracy exceeds inventory precision",
                    ));
                }
                let upper = Cell {
                    pigment_um: cell.pigment_um - lower.pigment_um,
                    binder_um: cell.binder_um - lower.binder_um,
                    solvent_um: cell.solvent_um - lower.solvent_um,
                    ..cell
                };
                result.cells.extend([lower, upper]);
            } else {
                result.cells.push(cell);
            }
        }
        packet.cell_count = u32::try_from(result.cells.len() - packet.first_cell as usize)
            .map_err(|_| MaterialError::Capacity)?;
    }
    Ok(result)
}

pub fn advance(
    column: &mut ColumnData,
    formulations: &[Arc<Formulation>],
    kinetics: &[Kinetics],
    seconds: f64,
    air: f64,
    scratch: &mut Scratch,
    coupling: Option<&dyn CoupledAccuracy>,
) -> Result<(), MaterialError> {
    let mut start = column.clone();
    for (packet, &coverage) in start.packets.iter().zip(&start.coverage) {
        let a = packet.first_cell as usize;
        for cell in &mut start.cells[a..a + packet.cell_count as usize] {
            cell.pigment_um /= coverage;
            cell.binder_um /= coverage;
            cell.solvent_um /= coverage;
        }
    }
    let paint_um = start
        .cells
        .iter()
        .map(|cell| cell.pigment_um as f64 + cell.binder_um as f64)
        .sum::<f64>();
    let adjusted: Vec<_> = kinetics
        .iter()
        .copied()
        .map(|mut kinetics| {
            kinetics.solvent_evaporation_rate_s /= 1.0 + paint_um / crate::thinner::TAU_DOUBLING_UM;
            kinetics
        })
        .collect();
    let kinetics = &adjusted;
    let mut accepted = loop {
        let mut coarse = start.clone();
        advance_column_with_scratch(&mut coarse.cells, kinetics, seconds, air, scratch)
            .map_err(|_| MaterialError::Invalid("material chemistry step failed"))?;
        let flags: Vec<_> = start
            .cells
            .iter()
            .map(|cell| cell.thickness_um() > 0.0)
            .collect();
        let mut refined = split(&start, &flags)?;
        advance_column_with_scratch(&mut refined.cells, kinetics, seconds, air, scratch)
            .map_err(|_| MaterialError::Invalid("refined material chemistry step failed"))?;
        let mut cursor = 0;
        let mut refine = vec![false; start.cells.len()];
        for (i, cell) in coarse.cells.iter().enumerate() {
            if !flags[i] {
                cursor += 1;
                continue;
            }
            let a = refined.cells[cursor];
            let b = refined.cells[cursor + 1];
            cursor += 2;
            let average = merge(a, b, kinetics);
            let average_error = intensive(cell)
                .into_iter()
                .zip(intensive(&average))
                .map(|(x, y)| (x - y).abs())
                .fold(0.0, f64::max);
            let variation = intensive(&a)
                .into_iter()
                .zip(intensive(&b))
                .map(|(x, y)| (x - y).abs())
                .fold(0.0, f64::max);
            let solvent_error = (cell.solvent_um as f64 - average.solvent_um as f64).abs()
                / start.cells[i].thickness_um().max(1e-30);
            refine[i] = average_error > STATE_ERROR
                || variation > FRONT_VARIATION
                || solvent_error > STATE_ERROR
                || rigidity_error(std::slice::from_ref(cell), &[a, b], formulations)?
                    > RIGIDITY_ERROR;
        }
        let coupled = coupling
            .map(|coupling| coupling.evaluate(&coarse, &refined, formulations))
            .transpose()?;
        let needs_coupled_refinement = coupled.is_some_and(|accuracy| {
            accuracy.threshold_sign_changed || accuracy.maximum_relative() > RIGIDITY_ERROR
        });
        if needs_coupled_refinement
            || rigidity_error(&coarse.cells, &refined.cells, formulations)? > RIGIDITY_ERROR
        {
            for flag in &mut refine {
                *flag = true;
            }
        }
        if refine.iter().all(|flag| !flag) {
            break coarse;
        }
        start = split(&start, &refine)?;
    };
    let old = std::mem::take(&mut accepted.cells);
    for packet in &mut accepted.packets {
        let a = packet.first_cell as usize;
        let b = a + packet.cell_count as usize;
        packet.first_cell =
            u32::try_from(accepted.cells.len()).map_err(|_| MaterialError::Capacity)?;
        let unstressed = packet.tension_n_m == [0.0; 3] && packet.moment_n == [0.0; 3];
        let mut i = a;
        while i < b {
            let mut cell = old[i];
            if unstressed
                && i + 1 < b
                && cell.recipe == old[i + 1].recipe
                && intensive(&cell)
                    .into_iter()
                    .zip(intensive(&old[i + 1]))
                    .all(|(x, y)| (x - y).abs() <= STATE_ERROR * 0.25)
                && rigidity_error(
                    &[merge(cell, old[i + 1], kinetics)],
                    &[cell, old[i + 1]],
                    formulations,
                )? <= RIGIDITY_ERROR
            {
                cell = merge(cell, old[i + 1], kinetics);
                i += 1;
            }
            accepted.cells.push(cell);
            i += 1;
        }
        packet.cell_count = u32::try_from(accepted.cells.len() - packet.first_cell as usize)
            .map_err(|_| MaterialError::Capacity)?;
    }
    for (packet, &coverage) in accepted.packets.iter().zip(&accepted.coverage) {
        let a = packet.first_cell as usize;
        for cell in &mut accepted.cells[a..a + packet.cell_count as usize] {
            cell.pigment_um *= coverage;
            cell.binder_um *= coverage;
            cell.solvent_um *= coverage;
        }
    }
    *column = accepted;
    Ok(())
}
