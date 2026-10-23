use std::cell::Cell;

thread_local! { static MATERIAL: Cell<bool> = const { Cell::new(false) }; }

pub fn material_schema() -> bool {
    MATERIAL.with(Cell::get)
}

pub fn with_material_schema<R>(enabled: bool, work: impl FnOnce() -> R) -> R {
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            MATERIAL.with(|mode| mode.set(self.0));
        }
    }
    let restore = Restore(MATERIAL.with(|mode| mode.replace(enabled)));
    let result = work();
    drop(restore);
    result
}

pub(crate) mod shared_recipe {
    use super::super::exchange::Recipe;
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;

    pub fn serialize<S: serde::Serializer>(value: &Arc<Recipe>, serializer: S) -> Result<S::Ok, S::Error> {
        value.as_ref().serialize(serializer)
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Arc<Recipe>, D::Error> {
        Recipe::deserialize(deserializer).map(Arc::new)
    }
}

pub(crate) mod kinetics {
    use super::super::chemistry::Kinetics;
    use serde::{Deserialize, Serialize};

    pub fn serialize<S: serde::Serializer>(
        value: &Kinetics,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        (
            super::super::store::kinetics_values(*value),
            [
                value.binder_code,
                value.drier_code,
                value.provenance_code,
                value.revision,
            ],
        )
            .serialize(serializer)
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Kinetics, D::Error> {
        let (v, codes) = <([f64; 13], [u32; 4])>::deserialize(deserializer)?;
        let kinetics = Kinetics {
            oxygen_solubility: v[0],
            solvent_solubility: v[1],
            diffusivity_um2_s: v[2],
            cured_diffusivity_ratio: v[3],
            air_transfer_um_s: v[4],
            oxidation_rate_s: v[5],
            autocatalysis: v[6],
            oxygen_demand: v[7],
            inhibitor_rate_s: v[8],
            inhibitor_oxygen_demand: v[9],
            inhibition_strength: v[10],
            network_rate_s: v[11],
            solvent_evaporation_rate_s: v[12],
            binder_code: codes[0],
            drier_code: codes[1],
            provenance_code: codes[2],
            revision: codes[3],
        };
        kinetics
            .validate()
            .map_err(|_| serde::de::Error::custom("invalid tool kinetics"))?;
        Ok(kinetics)
    }
}
