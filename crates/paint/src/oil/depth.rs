use super::chemistry::{Cell, Kinetics, Scratch, advance_column_with_scratch};
use super::constitutive::{Parameters, RefinementAccuracy, integrate_column};
use super::store::{ColumnData, Formulation, MaterialError};
use std::sync::Arc;

const STATE_ERROR: f64 = 1e-6;
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
                let mut upper = upper;
                let values = intensive(&cell);
                let left = (i > a && column.cells[i - 1].recipe == cell.recipe)
                    .then(|| &column.cells[i - 1]);
                let right = (i + 1 < b && column.cells[i + 1].recipe == cell.recipe)
                    .then(|| &column.cells[i + 1]);
                let h = cell.thickness_um();
                let mut delta = [0.0; 4];
                for k in 0..4 {
                    let dl = left
                        .map(|c| (values[k] - intensive(c)[k]) / (0.5 * (h + c.thickness_um())));
                    let dr = right
                        .map(|c| (intensive(c)[k] - values[k]) / (0.5 * (h + c.thickness_um())));
                    let slope = match (dl, dr) {
                        (Some(l), Some(r)) if l * r > 0.0 => {
                            l.signum() * ((l + r).abs() * 0.5).min(2.0 * l.abs()).min(2.0 * r.abs())
                        }
                        (Some(s), None) | (None, Some(s)) => s,
                        _ => 0.0,
                    };
                    delta[k] = (slope * h * 0.25).clamp(
                        -values[k].min(1.0 - values[k]),
                        values[k].min(1.0 - values[k]),
                    );
                }
                if cell.network <= cell.oxidation {
                    let difference = (delta[1] - delta[2]).abs();
                    let gap = values[1] - values[2];
                    if difference > gap {
                        let scale = gap / difference;
                        delta[1] *= scale;
                        delta[2] *= scale;
                    }
                }
                lower.oxygen = (values[0] - delta[0]) as f32;
                upper.oxygen = (values[0] + delta[0]) as f32;
                lower.oxidation = (values[1] - delta[1]) as f32;
                upper.oxidation = (values[1] + delta[1]) as f32;
                lower.network = (values[2] - delta[2]) as f32;
                upper.network = (values[2] + delta[2]) as f32;
                lower.inhibitor = (values[3] - delta[3]) as f32;
                upper.inhibitor = (values[3] + delta[3]) as f32;
                lower.clear_empty_oxygen();
                upper.clear_empty_oxygen();
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
    let mut recipes = Vec::new();
    let mut adjusted = Vec::new();
    if start.cells.iter().any(|cell| cell.solvent_um > 0.0) {
        for cell in &start.cells {
            let recipe = cell.recipe;
            if !recipes.contains(&recipe) {
                let mut material = *kinetics
                    .get(recipe as usize)
                    .ok_or(MaterialError::Invalid("missing depth kinetics"))?;
                material.solvent_evaporation_rate_s /=
                    1.0 + paint_um / crate::thinner::TAU_DOUBLING_UM;
                adjusted.push(material);
                recipes.push(recipe);
            }
        }
    }
    let advance_chemistry = |cells: &mut [Cell], scratch: &mut Scratch| {
        if recipes.is_empty() {
            return advance_column_with_scratch(cells, kinetics, seconds, air, scratch);
        }
        for cell in cells.iter_mut() {
            cell.recipe = recipes
                .iter()
                .position(|&recipe| recipe == cell.recipe)
                .unwrap() as u32;
        }
        let result = advance_column_with_scratch(cells, &adjusted, seconds, air, scratch);
        for cell in cells {
            cell.recipe = recipes[cell.recipe as usize];
        }
        result
    };
    let mut accepted = loop {
        let mut coarse = start.clone();
        advance_chemistry(&mut coarse.cells, scratch)
            .map_err(|_| MaterialError::Invalid("material chemistry step failed"))?;
        let flags: Vec<_> = start
            .cells
            .iter()
            .map(|cell| cell.thickness_um() > 0.0)
            .collect();
        let mut refined = split(&start, &flags)?;
        advance_chemistry(&mut refined.cells, scratch)
            .map_err(|_| MaterialError::Invalid("refined material chemistry step failed"))?;
        let needs_rigidity_refinement =
            rigidity_error(&coarse.cells, &refined.cells, formulations)? > RIGIDITY_ERROR;
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
                || (needs_rigidity_refinement
                    && rigidity_error(std::slice::from_ref(cell), &[a, b], formulations)?
                        > RIGIDITY_ERROR);
        }
        let coupled = coupling
            .map(|coupling| coupling.evaluate(&coarse, &refined, formulations))
            .transpose()?;
        let needs_coupled_refinement = coupled.is_some_and(|accuracy| {
            accuracy.threshold_sign_changed || accuracy.maximum_relative() > RIGIDITY_ERROR
        });
        if needs_coupled_refinement
            || (needs_rigidity_refinement && refine.iter().all(|flag| !flag))
        {
            for (flag, &positive) in refine.iter_mut().zip(&flags) {
                *flag = positive;
            }
        }
        while refine.iter().any(|flag| *flag) && refine.iter().any(|flag| !*flag) {
            let mut trial = split(&start, &refine)?;
            advance_chemistry(&mut trial.cells, scratch)
                .map_err(|_| MaterialError::Invalid("selective material chemistry step failed"))?;
            let mut trial_cursor = 0;
            let mut reference_cursor = 0;
            let mut expand = refine.clone();
            let mut unresolved = false;
            for i in 0..start.cells.len() {
                let reference = if flags[i] {
                    let value = merge(
                        refined.cells[reference_cursor],
                        refined.cells[reference_cursor + 1],
                        kinetics,
                    );
                    reference_cursor += 2;
                    value
                } else {
                    let value = refined.cells[reference_cursor];
                    reference_cursor += 1;
                    value
                };
                let candidate = if refine[i] {
                    let value = merge(
                        trial.cells[trial_cursor],
                        trial.cells[trial_cursor + 1],
                        kinetics,
                    );
                    trial_cursor += 2;
                    value
                } else {
                    let value = trial.cells[trial_cursor];
                    trial_cursor += 1;
                    value
                };
                if intensive(&candidate)
                    .into_iter()
                    .zip(intensive(&reference))
                    .any(|(x, y)| (x - y).abs() > STATE_ERROR * 0.25)
                    || (candidate.solvent_um as f64 - reference.solvent_um as f64).abs()
                        > STATE_ERROR * 0.25 * start.cells[i].thickness_um()
                {
                    unresolved = true;
                    expand[i] = true;
                    if i > 0 {
                        expand[i - 1] = true;
                    }
                    if i + 1 < expand.len() {
                        expand[i + 1] = true;
                    }
                }
            }
            for (flag, &positive) in expand.iter_mut().zip(&flags) {
                *flag &= positive;
            }
            if expand == refine {
                if !unresolved {
                    break;
                }
                for i in 0..refine.len() {
                    if refine[i] {
                        if i > 0 {
                            expand[i - 1] = flags[i - 1];
                        }
                        if i + 1 < expand.len() {
                            expand[i + 1] = flags[i + 1];
                        }
                    }
                }
                if expand == refine {
                    expand.clone_from(&flags);
                }
            }
            refine = expand;
        }
        loop {
            let mut changed = false;
            for i in 0..start.cells.len().saturating_sub(1) {
                if start.cells[i].recipe != start.cells[i + 1].recipe
                    || start
                        .packets
                        .iter()
                        .any(|packet| packet.first_cell as usize == i + 1)
                {
                    continue;
                }
                let a = start.cells[i].thickness_um() / if refine[i] { 2.0 } else { 1.0 };
                let b = start.cells[i + 1].thickness_um() / if refine[i + 1] { 2.0 } else { 1.0 };
                if a > 2.0 * b && b > 0.0 && !refine[i] {
                    refine[i] = true;
                    changed = true;
                }
                if b > 2.0 * a && a > 0.0 && !refine[i + 1] {
                    refine[i + 1] = true;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        if refine.iter().all(|flag| !flag) {
            break coarse;
        }
        start = split(&start, &refine)?;
    };
    let unmerged = accepted.clone();
    let old = std::mem::take(&mut accepted.cells);
    let mut merged = Vec::new();
    for packet in &mut accepted.packets {
        let a = packet.first_cell as usize;
        let b = a + packet.cell_count as usize;
        packet.first_cell =
            u32::try_from(accepted.cells.len()).map_err(|_| MaterialError::Capacity)?;
        let unstressed = packet.tension_n_m == [0.0; 3] && packet.moment_n == [0.0; 3];
        let mut i = a;
        while i < b {
            let mut cell = old[i];
            let combine = unstressed
                && i + 1 < b
                && cell.thickness_um() > 0.0
                && cell.recipe == old[i + 1].recipe
                && cell.thickness_um() == old[i + 1].thickness_um()
                && (accepted.cells.len() == packet.first_cell as usize
                    || accepted.cells.last().is_none_or(|left| {
                        left.recipe != cell.recipe || cell.thickness_um() <= left.thickness_um()
                    }))
                && (i + 2 >= b
                    || old[i + 2].recipe != cell.recipe
                    || cell.thickness_um() <= old[i + 2].thickness_um())
                && intensive(&cell)
                    .into_iter()
                    .zip(intensive(&old[i + 1]))
                    .enumerate()
                    .all(|(k, (x, y))| {
                        (x - y).abs()
                            <= if k < 4 {
                                FRONT_VARIATION
                            } else {
                                STATE_ERROR * 0.25
                            }
                    })
                && rigidity_error(
                    &[merge(cell, old[i + 1], kinetics)],
                    &[cell, old[i + 1]],
                    formulations,
                )? <= RIGIDITY_ERROR;
            if combine {
                cell = merge(cell, old[i + 1], kinetics);
                i += 1;
            }
            accepted.cells.push(cell);
            merged.push(combine);
            i += 1;
        }
        packet.cell_count = u32::try_from(accepted.cells.len() - packet.first_cell as usize)
            .map_err(|_| MaterialError::Capacity)?;
    }
    let reconstructed = split(&accepted, &merged)?;
    let coarsening_state_error = reconstructed
        .cells
        .iter()
        .zip(&unmerged.cells)
        .any(|(a, b)| {
            intensive(a)
                .into_iter()
                .zip(intensive(b))
                .any(|(x, y)| (x - y).abs() > STATE_ERROR * 0.25)
        });
    if accepted.cells.len() != unmerged.cells.len()
        && (coarsening_state_error
            || rigidity_error(&accepted.cells, &unmerged.cells, formulations)? > RIGIDITY_ERROR
            || coupling
                .map(|coupling| coupling.evaluate(&accepted, &unmerged, formulations))
                .transpose()?
                .is_some_and(|accuracy| {
                    accuracy.threshold_sign_changed || accuracy.maximum_relative() > RIGIDITY_ERROR
                }))
    {
        accepted = unmerged;
    }
    for (packet, &coverage) in accepted.packets.iter().zip(&accepted.coverage) {
        let a = packet.first_cell as usize;
        for cell in &mut accepted.cells[a..a + packet.cell_count as usize] {
            cell.pigment_um *= coverage;
            cell.binder_um *= coverage;
            cell.solvent_um *= coverage;
            cell.clear_empty_oxygen();
        }
    }
    *column = accepted;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::chemistry::{
        CALENDAR_TARGETS, CalendarPrediction, FixtureId, observe, predict_calendar,
    };
    use super::super::constitutive::{AccuracyFloors, Rigidity, refinement_accuracy};
    use super::super::store::Packet;
    use super::*;
    use std::cell::Cell as Counter;

    fn formulation(kinetics: Kinetics) -> Arc<Formulation> {
        Arc::new(
            Formulation::new(
                crate::wet::Latent::default(),
                crate::Paint::body([0.5; 3]).prop(),
                kinetics,
            )
            .unwrap(),
        )
    }

    fn column(cells: Vec<Cell>) -> ColumnData {
        ColumnData {
            packets: vec![Packet {
                cell_count: cells.len() as u32,
                ..Packet::default()
            }],
            cells,
            coverage: vec![1.0],
            ..ColumnData::default()
        }
    }

    #[test]
    fn linear_prolongation_preserves_inventories_bounds_and_packet_history() {
        assert_eq!(std::mem::size_of::<Cell>(), 32);
        assert_eq!(std::mem::size_of::<Packet>(), 40);
        let kinetics = [Kinetics::default(); 2];
        let mut source = column(
            (0..6)
                .map(|i| Cell {
                    pigment_um: 4.0,
                    binder_um: 8.0,
                    solvent_um: 2.0,
                    oxygen: 0.1 + i as f32 * 0.1,
                    oxidation: 0.2 + i as f32 * 0.1,
                    network: 0.15 + i as f32 * 0.1,
                    inhibitor: 0.8 - i as f32 * 0.1,
                    recipe: if i < 3 { 0 } else { 1 },
                })
                .collect(),
        );
        source.packets[0].cell_count = 3;
        source.packets[0].tension_n_m = [0.2, -0.3, 0.1];
        source.packets[0].moment_n = [1e-5, -2e-5, 3e-5];
        source.packets.push(Packet {
            first_cell: 3,
            cell_count: 3,
            ..source.packets[0]
        });
        source.coverage.push(0.4);
        let refined = split(&source, &[true; 6]).unwrap();
        assert!(refined.cells[2].oxygen < source.cells[1].oxygen);
        for (i, pair) in refined.cells.chunks_exact(2).enumerate() {
            let restored = merge(pair[0], pair[1], &kinetics);
            let original = source.cells[i];
            assert_eq!(
                [restored.pigment_um, restored.binder_um, restored.solvent_um],
                [original.pigment_um, original.binder_um, original.solvent_um]
            );
            assert!(
                (restored.oxygen_inventory(&kinetics[original.recipe as usize])
                    - original.oxygen_inventory(&kinetics[original.recipe as usize]))
                .abs()
                    < 1e-8
            );
            for (x, y) in intensive(&restored).into_iter().zip(intensive(&original)) {
                assert!((x - y).abs() < 1e-7);
            }
            for child in pair {
                assert_eq!(child.recipe, original.recipe);
                assert!(child.network <= child.oxidation);
                assert!(
                    intensive(child)[..4]
                        .iter()
                        .all(|x| (0.0..=1.0).contains(x))
                );
            }
        }
        for (a, b) in refined.packets.iter().zip(&source.packets) {
            assert_eq!(a.tension_n_m, b.tension_n_m);
            assert_eq!(a.moment_n, b.moment_n);
            assert_eq!(a.provenance, b.provenance);
            assert_eq!(a.cell_count, b.cell_count * 2);
        }
        assert_eq!(refined.coverage, source.coverage);
        let mut isolated = source.clone();
        isolated.cells[3].recipe = 0;
        isolated.cells[3].oxygen = 1.0;
        assert_eq!(
            split(&isolated, &[true; 6]).unwrap().cells[..6],
            refined.cells[..6]
        );
        isolated.cells[2].network = isolated.cells[2].oxidation;
        for child in &split(&isolated, &[true; 6]).unwrap().cells {
            assert!(child.network <= child.oxidation);
        }
    }

    #[test]
    fn sparse_solvent_adjustment_matches_original_chemistry_and_preserves_recipe_ids() {
        struct Recipes;
        impl CoupledAccuracy for Recipes {
            fn evaluate(
                &self,
                coarse: &ColumnData,
                refined: &ColumnData,
                formulations: &[Arc<Formulation>],
            ) -> Result<RefinementAccuracy, MaterialError> {
                assert_eq!(formulations.len(), 42);
                assert!(
                    coarse
                        .cells
                        .iter()
                        .chain(&refined.cells)
                        .all(|cell| [17, 41].contains(&cell.recipe))
                );
                Ok(RefinementAccuracy::default())
            }
        }
        let material = Kinetics {
            diffusivity_um2_s: 0.0,
            air_transfer_um_s: 0.0,
            oxidation_rate_s: 0.0,
            network_rate_s: 0.0,
            inhibitor_rate_s: 0.0,
            solvent_evaporation_rate_s: 1e-4,
            ..Kinetics::default()
        };
        let mut kinetics = vec![
            Kinetics {
                oxidation_rate_s: f64::NAN,
                ..material
            };
            42
        ];
        kinetics[17] = material;
        kinetics[41] = material;
        let formulations = vec![formulation(material); 42];
        for solvent in [0.0, 10.0] {
            let mut source = column(
                [17, 41]
                    .map(|recipe| Cell {
                        pigment_um: 10.0,
                        binder_um: 20.0,
                        solvent_um: solvent,
                        oxygen: 0.4,
                        oxidation: 0.6,
                        network: 0.4,
                        inhibitor: 0.2,
                        recipe,
                    })
                    .to_vec(),
            );
            source.packets[0].tension_n_m = [0.1; 3];
            let mut adjusted = kinetics.clone();
            for material in &mut adjusted {
                material.solvent_evaporation_rate_s /= 1.0 + 60.0 / crate::thinner::TAU_DOUBLING_UM;
            }
            advance(
                &mut source,
                &formulations,
                &kinetics,
                600.0,
                1.0,
                &mut Scratch::default(),
                Some(&Recipes),
            )
            .unwrap();
            let mut expected: Vec<_> = source
                .cells
                .iter()
                .map(|cell| Cell {
                    pigment_um: cell.pigment_um,
                    binder_um: cell.binder_um,
                    solvent_um: solvent * (cell.pigment_um / 10.0),
                    oxygen: 0.4,
                    oxidation: 0.6,
                    network: 0.4,
                    inhibitor: 0.2,
                    recipe: cell.recipe,
                })
                .collect();
            advance_column_with_scratch(
                &mut expected,
                &adjusted,
                600.0,
                1.0,
                &mut Scratch::default(),
            )
            .unwrap();
            for recipe in [17, 41] {
                let actual = source
                    .cells
                    .iter()
                    .filter(|cell| cell.recipe == recipe)
                    .copied()
                    .reduce(|a, b| merge(a, b, &kinetics))
                    .unwrap();
                let expected = expected
                    .iter()
                    .filter(|cell| cell.recipe == recipe)
                    .copied()
                    .reduce(|a, b| merge(a, b, &kinetics))
                    .unwrap();
                assert!((actual.solvent_um - expected.solvent_um).abs() < 1e-5);
                for (a, b) in intensive(&actual).into_iter().zip(intensive(&expected)) {
                    assert!((a - b).abs() < 1e-6);
                }
            }
        }
    }

    #[test]
    fn adaptive_calendar_resolves_nonlocal_diffusion_and_thick_golden_film() {
        for id in [FixtureId::S3Lwlo450, FixtureId::S2Golden254] {
            let fixture = id.fixture();
            let references =
                [96, 192].map(|n| predict_calendar(&fixture, n, 600.0, 400.0).unwrap());
            let mut prop = crate::Paint::body([0.5; 3]).prop();
            prop[8] = fixture.binder_volume_fraction as f32;
            let formulations = [Arc::new(
                Formulation::new(crate::wet::Latent::default(), prop, fixture.kinetics).unwrap(),
            )];
            let mut adaptive = column(fixture.column(1).unwrap());
            let mut scratch = Scratch::default();
            let mut prediction = CalendarPrediction::default();
            for step in 1..=2400 {
                advance(
                    &mut adaptive,
                    &formulations,
                    &[fixture.kinetics],
                    600.0,
                    1.0,
                    &mut scratch,
                    None,
                )
                .unwrap();
                let observation = observe(&adaptive.cells, &fixture.observations).unwrap();
                for (met, endpoint) in [
                    (observation.set_to_touch, &mut prediction.set_to_touch_hours),
                    (observation.dry_to_touch, &mut prediction.dry_to_touch_hours),
                    (observation.dry_hard, &mut prediction.dry_hard_hours),
                    (observation.golden_touch, &mut prediction.golden_touch_hours),
                ] {
                    if met && endpoint.is_none() {
                        *endpoint = Some(step as f64 / 6.0);
                    }
                }
                if prediction.dry_hard_hours.is_some() {
                    break;
                }
            }
            for target in CALENDAR_TARGETS
                .iter()
                .filter(|target| target.fixture == id)
            {
                let actual = prediction.at(target.endpoint).unwrap();
                assert!(
                    target.residual_hours(actual).abs() <= 1.0 / 6.0 + 1e-10,
                    "{id:?} {:?}: {actual}",
                    target.endpoint
                );
                for reference in references {
                    assert!(
                        (actual - reference.at(target.endpoint).unwrap()).abs()
                            <= 1.0 / 6.0 + 1e-10,
                        "{id:?} {:?}: {actual}",
                        target.endpoint
                    );
                }
            }
        }
    }

    struct GrowthAccuracy {
        tension: f64,
        calls: Counter<usize>,
        sign_changes: Counter<usize>,
    }

    fn growth(r: &Rigidity, tension: f64) -> f64 {
        super::super::mechanics::coupled_linear_growth_oracle(
            3000.0,
            100e-6,
            1000.0,
            &super::super::mechanics::PlateTangent {
                a: r.a,
                b: r.b,
                d: r.d,
                z_ref_m: r.z_ref_m,
            },
            tension,
            0.03,
        )
        .unwrap()[0]
    }

    impl CoupledAccuracy for GrowthAccuracy {
        fn evaluate(
            &self,
            coarse: &ColumnData,
            refined: &ColumnData,
            formulations: &[Arc<Formulation>],
        ) -> Result<RefinementAccuracy, MaterialError> {
            self.calls.set(self.calls.get() + 1);
            let a = integrate_column(&coarse.cells, formulations, &Parameters::default()).unwrap();
            let b = integrate_column(&refined.cells, formulations, &Parameters::default()).unwrap();
            let accuracy = refinement_accuracy(
                &a,
                &b,
                [growth(&a, self.tension), growth(&b, self.tension)],
                AccuracyFloors {
                    extension_n_m: 1e-10,
                    bending_n_m: 1e-20,
                    threshold: 1e-9,
                },
            )
            .unwrap();
            if accuracy.threshold_sign_changed {
                self.sign_changes.set(self.sign_changes.get() + 1);
            }
            Ok(accuracy)
        }
    }

    #[test]
    fn real_coupled_growth_threshold_controls_acceptance() {
        let kinetics = Kinetics {
            oxidation_rate_s: 0.0,
            network_rate_s: 0.0,
            inhibitor_rate_s: 0.0,
            ..Kinetics::default()
        };
        let formulations = [formulation(kinetics)];
        let mut source = column(
            (0..4)
                .map(|i| Cell {
                    pigment_um: 40.0,
                    binder_um: 60.0,
                    oxygen: 1.0,
                    oxidation: 0.9,
                    network: 0.4 + i as f32 * 0.1,
                    ..Cell::default()
                })
                .collect(),
        );
        let refined = split(&source, &[true; 4]).unwrap();
        let a = integrate_column(&source.cells, &formulations, &Parameters::default()).unwrap();
        let b = integrate_column(&refined.cells, &formulations, &Parameters::default()).unwrap();
        let critical =
            |r: &Rigidity| -0.03 - 3000.0_f64.powi(2) * (r.d[0][0] - r.b[0][0].powi(2) / r.a[0][0]);
        let coupling = GrowthAccuracy {
            tension: 0.5 * (critical(&a) + critical(&b)),
            calls: Counter::new(0),
            sign_changes: Counter::new(0),
        };
        assert!(
            coupling
                .evaluate(&source, &refined, &formulations)
                .unwrap()
                .threshold_sign_changed
        );
        source.packets[0].tension_n_m = [coupling.tension as f32, 0.0, 0.0];
        source.packets[0].moment_n = [1e-6, 2e-6, 3e-6];
        let history = source.packets[0];
        advance(
            &mut source,
            &formulations,
            &[kinetics],
            0.0,
            1.0,
            &mut Scratch::default(),
            Some(&coupling),
        )
        .unwrap();
        let reference = split(&source, &vec![true; source.cells.len()]).unwrap();
        let accepted = coupling
            .evaluate(&source, &reference, &formulations)
            .unwrap();
        assert!(!accepted.threshold_sign_changed);
        assert!(accepted.maximum_relative() <= RIGIDITY_ERROR);
        assert!(coupling.calls.get() > 2);
        assert!(coupling.sign_changes.get() > 0);
        assert_eq!(source.packets[0].tension_n_m, history.tension_n_m);
        assert_eq!(source.packets[0].moment_n, history.moment_n);
    }
}
