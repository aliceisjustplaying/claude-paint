use super::chemistry::Kinetics;
use crate::wet::{LAT, Prop};
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "replay-effects",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Recipe {
    pub color: [f32; LAT],
    pub parts: Vec<(usize, f32)>,
    pub properties: Prop,
    #[cfg_attr(feature = "replay-effects", serde(with = "super::wire::kinetics"))]
    pub kinetics: Kinetics,
    pub binders: [f64; 5],
}

#[derive(Default)]
pub(crate) struct RecipePool {
    recipes: HashMap<u64, Vec<Arc<Recipe>>>,
}

impl RecipePool {
    pub(crate) fn intern(&mut self, recipe: Arc<Recipe>) -> Arc<Recipe> {
        let mut hash = DefaultHasher::new();
        for value in recipe.color.iter().chain(&recipe.properties) {
            (if *value == 0.0 { 0 } else { value.to_bits() }).hash(&mut hash);
        }
        recipe.parts.len().hash(&mut hash);
        for (id, value) in &recipe.parts {
            id.hash(&mut hash);
            (if *value == 0.0 { 0 } else { value.to_bits() }).hash(&mut hash);
        }
        for value in super::store::kinetics_values(recipe.kinetics).into_iter().chain(recipe.binders) {
            (if value == 0.0 { 0 } else { value.to_bits() }).hash(&mut hash);
        }
        [recipe.kinetics.binder_code, recipe.kinetics.drier_code,
            recipe.kinetics.provenance_code, recipe.kinetics.revision].hash(&mut hash);
        let candidates = self.recipes.entry(hash.finish()).or_default();
        if let Some(existing) = candidates.iter().find(|existing| ***existing == *recipe) {
            return existing.clone();
        }
        candidates.push(recipe.clone());
        recipe
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(
    feature = "replay-effects",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct MechanicalHistory {
    pub tension_n_m: [f32; 3],
    pub moment_n: [f32; 3],
    pub interface_damage: f32,
    pub provenance: u32,
    pub coverage: f32,
}

impl Default for MechanicalHistory {
    fn default() -> Self {
        Self {
            tension_n_m: [0.0; 3],
            moment_n: [0.0; 3],
            interface_damage: 0.0,
            provenance: 0,
            coverage: 1.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "replay-effects",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Fragment {
    pub reference_area: f64,
    pub volume: f64,
    pub pigment: f64,
    pub binder: f64,
    pub solvent: f64,
    pub oxygen: f64,
    pub oxidation: f64,
    pub network: f64,
    pub inhibitor: f64,
    #[cfg_attr(feature = "replay-effects", serde(with = "super::wire::shared_recipe"))]
    pub recipe: Arc<Recipe>,
    pub history: MechanicalHistory,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bristle::{Held, Tool, feed};
    use crate::canvas::Canvas;
    use crate::wet::{Latent, Paint, mix_into};

    fn parcel(network: f64, solubility: f64) -> Latent {
        let paint = Paint::body([0.2, 0.3, 0.4]);
        let mut latent = paint.latent();
        latent.material = Some(MaterialPayload {
            fragments: vec![Fragment {
                reference_area: 1.0,
                volume: 1.0,
                pigment: 10.0,
                binder: 15.0,
                solvent: 0.0,
                oxygen: 0.4,
                oxidation: network * 0.8,
                network,
                inhibitor: 0.15,
                recipe: Arc::new(Recipe {
                    binders: [1.0, 0.0, 0.0, 0.0, 0.0],
                    color: latent.color,
                    parts: vec![(0, 1.0)],
                    properties: paint.prop(),
                    kinetics: Kinetics {
                        oxygen_solubility: solubility,
                        solvent_solubility: 0.4,
                        diffusivity_um2_s: 1.0,
                        cured_diffusivity_ratio: 0.01,
                        air_transfer_um_s: 1.0,
                        oxidation_rate_s: 0.001,
                        autocatalysis: 0.0,
                        oxygen_demand: 1.0,
                        inhibitor_rate_s: 0.001,
                        inhibitor_oxygen_demand: 1.0,
                        inhibition_strength: 1.0,
                        network_rate_s: 0.001,
                        solvent_evaporation_rate_s: 0.001,
                        binder_code: 1,
                        drier_code: 0,
                        provenance_code: 1,
                        revision: 1,
                    },
                }),
                history: MechanicalHistory::default(),
            }],
        });
        latent
    }

    fn values(inventory: Inventory) -> [f64; 7] {
        [
            inventory.pigment,
            inventory.binder,
            inventory.solvent,
            inventory.oxygen,
            inventory.oxidized_binder,
            inventory.networked_binder,
            inventory.inhibitor,
        ]
    }

    fn inventory(lat: &Latent, volume: f64) -> [f64; 7] {
        lat.material
            .as_ref()
            .map_or([0.0; 7], |m| values(m.scaled(volume).inventory()))
    }

    fn conserved(expected: [f64; 7], actual: [f64; 7]) {
        for (a, b) in expected.into_iter().zip(actual) {
            assert!((a - b).abs() <= 2e-5 * a.abs().max(1.0), "{a} != {b}");
        }
    }

    #[test]
    fn cloned_fragments_share_recipes_and_interning_uses_all_contents() {
        let source = parcel(0.2, 0.4).material.unwrap().fragments.remove(0);
        assert!(Arc::ptr_eq(&source.recipe, &source.clone().recipe));
        let mut pool = RecipePool::default();
        let mut first = (*source.recipe).clone();
        first.color[0] = 0.0;
        let a = pool.intern(Arc::new(first.clone()));
        first.color[0] = -0.0;
        let b = pool.intern(Arc::new(first.clone()));
        assert!(Arc::ptr_eq(&a, &b));
        first.kinetics.network_rate_s *= 2.0;
        let c = pool.intern(Arc::new(first));
        assert!(!Arc::ptr_eq(&a, &c));
        assert_ne!(a, c);
    }

    #[test]
    fn fresh_legacy_latents_have_no_material_payload() {
        assert!(Latent::default().material.is_none());
        assert!(Paint::body([0.2; 3]).latent().material.is_none());
    }

    #[test]
    fn aged_mixture_preserves_capacity_and_binder_weighted_inventories() {
        let (mut left, right) = (parcel(0.8, 0.2), parcel(0.1, 1.3));
        let expected =
            std::array::from_fn(|k| inventory(&left, 2.0)[k] + inventory(&right, 3.0)[k]);
        let (mut volume, mut hide) = (2.0, Paint::body([0.2; 3]).prop());
        let incoming_hide = hide;
        mix_into(
            &mut volume,
            &mut left,
            &mut hide,
            3.0,
            &right,
            incoming_hide,
        );
        conserved(expected, inventory(&left, volume as f64));
        assert_eq!(left.material.as_ref().unwrap().fragments[0].network, 0.8);
        assert_eq!(left.material.as_ref().unwrap().fragments[1].network, 0.1);
    }

    #[test]
    fn top_removal_exposes_the_existing_lower_cure() {
        let hide = Paint::body([0.2; 3]).prop();
        let (_, latent, _) = Latent::from_layers(vec![
            crate::wet::Layer {
                share: 0.6,
                lat: parcel(0.9, 0.2),
                hide,
            },
            crate::wet::Layer {
                share: 0.4,
                lat: parcel(0.1, 0.2),
                hide,
            },
        ]);
        let (below, top) = latent.split_top(1.0, hide, 0.4, false);
        let (v, exposed, _) = Latent::from_layers(below);
        let (t, removed, _) = Latent::from_layers(top);
        assert!((v - 0.6).abs() < 1e-6 && (t - 0.4).abs() < 1e-6);
        assert_eq!(exposed.material.as_ref().unwrap().fragments[0].network, 0.9);
        assert_eq!(
            removed
                .material
                .as_ref()
                .unwrap()
                .fragments
                .last()
                .unwrap()
                .network,
            0.1
        );
        conserved(
            inventory(&latent, 1.0),
            std::array::from_fn(|k| {
                inventory(&exposed, v as f64)[k] + inventory(&removed, t as f64)[k]
            }),
        );
    }

    #[test]
    fn repeated_actual_mixing_coalesces_only_compatible_constituents() {
        let mut mixed = parcel(0.08, 0.2).material.unwrap();
        let fresh = parcel(0.1, 0.2).material.unwrap();
        let before = values(mixed.inventory());
        let incoming = values(fresh.inventory());
        mixed.mix_in(&fresh, 3.0);
        conserved(
            std::array::from_fn(|k| before[k] + 3.0 * incoming[k]),
            values(mixed.inventory()),
        );
        assert_eq!(mixed.fragments.len(), 1);
        assert!((mixed.fragments[0].network - 0.095).abs() < 1e-14);
        let expected = values(mixed.inventory());
        for _ in 0..128 {
            let same = mixed.clone();
            mixed.blend(&same, 0.37);
            assert_eq!(mixed.fragments.len(), 1);
        }
        conserved(expected, values(mixed.inventory()));
        let mut stressed = fresh.clone();
        for _ in 0..32 {
            let (_, mut skim) = mixed.split_top_with_geometry(1e-20, 50.0);
            skim.scale(mixed.volume() / skim.volume());
            mixed.blend(&skim, 0.5);
            assert_eq!(mixed.fragments.len(), 1);
            assert_eq!(mixed.fragments[0].reference_area, 1.0);
        }
        conserved(expected, values(mixed.inventory()));
        stressed.fragments[0].history.tension_n_m[0] = 1.0;
        mixed.mix_in(&stressed, 1.0);
        assert_eq!(mixed.fragments.len(), 2);
        let mut ordered = parcel(0.8, 0.2).material.unwrap();
        ordered.append(&fresh, 1.0);
        assert_eq!(ordered.fragments.len(), 2);
        assert_eq!(ordered.fragments[0].network, 0.8);
        assert_eq!(ordered.fragments[1].network, 0.1);
        ordered.mix_in(&fresh, 1.0);
        assert_eq!(ordered.fragments[0].network, 0.8);
        assert_eq!(ordered.fragments.len(), 2);
    }

    #[test]
    fn feed_and_wipe_partition_the_retained_material() {
        let mut held = Held::new(Tool::filbert(10.0), 7);
        held.load_material(Paint::body([0.2; 3]), 0.5, parcel(0.7, 0.2).material);
        let total = |held: &Held| {
            held.bristles.iter().fold([0.0; 7], |sum, b| {
                std::array::from_fn(|k| sum[k] + inventory(&b.lat, b.vol as f64)[k])
            })
        };
        let before = total(&held);
        feed(&mut held.bristles, 0.6);
        conserved(before, total(&held));
        let wiped = values(held.wipe_material(0.4).inventory());
        conserved(before, std::array::from_fn(|k| total(&held)[k] + wiped[k]));
        held.wipe(1.0);
        assert!(held.bristles.iter().all(|b| b.lat.material.is_none()));
    }

    #[test]
    fn native_pickup_and_optimized_plough_keep_the_source_complement() {
        let mut canvas = Canvas::new(16, 1.0, [0.8; 3]).with_engine(7);
        let hide = Paint::body([0.2; 3]).prop();
        canvas.wet.vol[0] = 1.0;
        canvas.wet.lat[0] = parcel(0.8, 0.2);
        canvas.wet.hide[0] = hide;
        let expected = inventory(&canvas.wet.lat[0], 1.0);
        let surf = canvas.surf();
        unsafe {
            let picked = surf.pickup_lat(0, 0.25).unwrap();
            let properties = surf.take(0, 0.25);
            let film = crate::engine7::add::Film::of(&surf);
            film.add(
                1,
                0.25,
                &crate::engine7::add::Paint7::new(false, false, &picked, properties, 0.0),
            );
        }
        conserved(
            expected,
            std::array::from_fn(|k| {
                inventory(&canvas.wet.lat[0], canvas.wet.vol[0] as f64)[k]
                    + inventory(&canvas.wet.lat[1], canvas.wet.vol[1] as f64)[k]
            }),
        );
        assert_eq!(
            canvas.wet.lat[1].material.as_ref().unwrap().fragments[0].network,
            0.8
        );
    }

    #[test]
    fn fractional_solvent_flow_keeps_material_in_the_source() {
        let mut canvas = Canvas::new(16, 1.0, [0.8; 3])
            .with_size_mm(16.0)
            .with_engine(5);
        canvas.wet.ensure_solvent();
        canvas
            .wet
            .material_sinks
            .resize(canvas.wet.vol.len(), Vec::new());
        let source = 8 * canvas.f.w + 8;
        canvas.wet.vol[source] = 2.0;
        canvas.wet.solv[source] = crate::surface::COAT_UM;
        canvas.wet.lat[source] = parcel(0.0, 0.2);
        canvas.wet.hide[source] = Paint::body([0.2; 3]).prop();
        let expected = inventory(&canvas.wet.lat[source], 2.0);
        canvas.spread(0.1, &mut vec![source]);
        assert!(canvas.wet.vol[source] > 0.0 && canvas.wet.vol[source] < 2.0);
        let actual = canvas
            .wet
            .lat
            .iter()
            .zip(&canvas.wet.vol)
            .fold([0.0; 7], |sum, (lat, v)| {
                std::array::from_fn(|k| sum[k] + inventory(lat, *v as f64)[k])
            });
        conserved(expected, actual);
    }

    #[test]
    fn depth_split_preserves_force_and_moment_about_the_original_plane() {
        let mut material = parcel(0.8, 0.2).material.unwrap();
        material.fragments[0].history.tension_n_m = [0.5, 0.3, 0.1];
        material.fragments[0].history.moment_n = [1e-6, 2e-6, -1e-6];
        let (below, top) = material.split_top_with_geometry(0.25, 100.0);
        let (a, b, original) = (
            &below.fragments[0].history,
            &top.fragments[0].history,
            &material.fragments[0].history,
        );
        for k in 0..3 {
            assert!((a.tension_n_m[k] + b.tension_n_m[k] - original.tension_n_m[k]).abs() < 1e-6);
            let moment = a.moment_n[k] as f64 + b.moment_n[k] as f64
                - a.tension_n_m[k] as f64 * 12.5e-6
                + b.tension_n_m[k] as f64 * 37.5e-6;
            assert!((moment - original.moment_n[k] as f64).abs() < 1e-11);
        }
    }

    #[test]
    fn drainage_and_evaporation_remove_only_the_named_species() {
        let mut latent = parcel(0.7, 0.2);
        latent.material.as_mut().unwrap().fragments[0].solvent = 5.0;
        let before = inventory(&latent, 1.0);
        let sink = latent.drain_material(1.0, 0.94, 0.0, 0.44 / 0.94).unwrap();
        let removed = values(sink.material.inventory());
        assert_eq!(removed[0], 0.0);
        assert_eq!(removed[2], 0.0);
        conserved(
            before,
            std::array::from_fn(|k| inventory(&latent, 0.94)[k] + removed[k]),
        );
        let material = latent.material.as_mut().unwrap();
        let before = values(material.inventory());
        let solvent = values(material.remove_solvent(1.0).inventory());
        assert_eq!(solvent[0], 0.0);
        assert_eq!(solvent[1], 0.0);
        conserved(
            before,
            std::array::from_fn(|k| values(material.inventory())[k] + solvent[k]),
        );
    }

    #[test]
    fn exhausted_oxygen_carriers_leave_a_valid_pigment_residue() {
        for drain in [false, true] {
            let mut material = parcel(0.2, 0.4).material.unwrap();
            let before = values(material.inventory());
            let removed = if drain {
                material.drain_binder(15.0, 0.0, 1.0)
            } else {
                material.remove_binder(15.0)
            };
            assert!(material.fragments.iter().all(|f| f.binder == 0.0 && f.oxygen == 0.0));
            conserved(before, std::array::from_fn(|k| values(material.inventory())[k] + values(removed.inventory())[k]));
        }
        let mut material = parcel(0.2, 0.4).material.unwrap();
        material.fragments[0].solvent = 5.0;
        material.remove_binder(15.0);
        assert_eq!(material.fragments[0].oxygen, 0.4);
        let before = values(material.inventory());
        let removed = material.remove_solvent(5.0);
        assert_eq!(material.fragments[0].oxygen, 0.0);
        conserved(before, std::array::from_fn(|k| values(material.inventory())[k] + values(removed.inventory())[k]));
    }

    #[test]
    fn contact_coverage_preserves_inventories_and_coherent_geometry() {
        let mut material = parcel(0.0, 0.4).material.unwrap();
        material.append(&parcel(0.8, 0.4).material.unwrap(), 1.0);
        let mut stressed = parcel(0.0, 0.4).material.unwrap();
        stressed.fragments[0].history.tension_n_m = [1.0; 3];
        material.append(&stressed, 1.0);
        let before = material.inventory();
        let coherent = material.fragments[1..].to_vec();
        material.set_mobile_coverage(0.25);
        assert_eq!(material.fragments[0].history.coverage, 0.25);
        assert_eq!(material.fragments[1..], coherent);
        assert_eq!(material.inventory(), before);
    }

    #[test]
    fn rag_refold_retains_the_mobile_material_inside_the_cloth() {
        let mut rag = crate::rag::Rag::new(20.0, 7);
        rag.mobile.mm3 = 0.5;
        rag.mobile.lat = parcel(0.8, 0.2);
        let before = inventory(&rag.mobile.lat, 0.5);
        rag.refold(&mut crate::tally::Tally::default());
        assert!(rag.mobile.lat.material.is_none());
        conserved(before, values(rag.absorbed.unwrap().inventory()));
    }
}

impl Fragment {
    pub(crate) fn properties(&self) -> Prop {
        let mut prop = self.recipe.properties;
        let q = prop[8] as f64;
        if self.pigment > 0.0 && q > 0.0 && q < 1.0 {
            prop[4] = (self.binder / self.pigment * (1.0 - q) / q) as f32;
            let reference = (1.0 - q) / (1.0 - q + q * self.recipe.properties[4] as f64);
            prop[0] *= (self.pigment / (self.pigment + self.binder) / reference) as f32;
        }
        prop
    }

    pub fn scale(&mut self, factor: f64) {
        self.reference_area *= factor;
        self.volume *= factor;
        self.pigment *= factor;
        self.binder *= factor;
        self.solvent *= factor;
    }

    pub fn oxygen_capacity(&self) -> f64 {
        self.binder * self.recipe.kinetics.oxygen_solubility
            + self.solvent * self.recipe.kinetics.solvent_solubility
    }

    fn sliced_history(&self, start: f64, end: f64, um_per_volume: f64) -> MechanicalHistory {
        let mut history = self.history;
        let height = self.volume * um_per_volume * 1e-6 / history.coverage as f64;
        assert!(
            height.is_finite() && height > 0.0,
            "mechanical depth split requires positive packet geometry"
        );
        let a = (start - 0.5) * height;
        let b = (end - 0.5) * height;
        let fraction = end - start;
        for k in 0..3 {
            history.tension_n_m[k] = (self.history.tension_n_m[k] as f64 * fraction
                + 6.0 * self.history.moment_n[k] as f64 * (b * b - a * a) / height.powi(3))
                as f32;
            history.moment_n[k] = (self.history.moment_n[k] as f64 * fraction.powi(3)) as f32;
        }
        history
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "replay-effects", derive(serde::Serialize))]
pub struct MaterialPayload {
    pub fragments: Vec<Fragment>,
}

#[cfg(feature = "replay-effects")]
impl<'de> serde::Deserialize<'de> for MaterialPayload {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Wire {
            fragments: Vec<Fragment>,
        }
        let material = Wire::deserialize(deserializer)?;
        for fragment in &material.fragments {
            let history = fragment.history;
            if !fragment.reference_area.is_finite()
                || fragment.reference_area <= 0.0
                || [
                    fragment.volume,
                    fragment.pigment,
                    fragment.binder,
                    fragment.solvent,
                ]
                .iter()
                .any(|v| !v.is_finite() || *v < 0.0)
                || [
                    fragment.oxygen,
                    fragment.oxidation,
                    fragment.network,
                    fragment.inhibitor,
                ]
                .iter()
                .any(|v| !v.is_finite() || !(0.0..=1.0).contains(v))
                || history
                    .tension_n_m
                    .iter()
                    .chain(&history.moment_n)
                    .any(|v| !v.is_finite())
                || !history.coverage.is_finite()
                || !(0.0..=1.0).contains(&history.coverage)
                || history.coverage == 0.0
                || !history.interface_damage.is_finite()
                || !(0.0..=1.0).contains(&history.interface_damage)
                || fragment
                    .recipe
                    .color
                    .iter()
                    .chain(&fragment.recipe.properties)
                    .any(|v| !v.is_finite())
                || fragment
                    .recipe
                    .parts
                    .iter()
                    .any(|p| !p.1.is_finite() || p.1 < 0.0)
                || fragment
                    .recipe
                    .binders
                    .iter()
                    .any(|v| !v.is_finite() || *v < 0.0)
                || (fragment.recipe.binders.iter().sum::<f64>() - 1.0).abs() > 1e-12
            {
                return Err(serde::de::Error::custom("invalid chemical tool payload"));
            }
        }
        let mut fragments = material.fragments;
        let mut recipes = RecipePool::default();
        for fragment in &mut fragments {
            fragment.recipe = recipes.intern(fragment.recipe.clone());
        }
        Ok(Self { fragments })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Inventory {
    pub pigment: f64,
    pub binder: f64,
    pub solvent: f64,
    pub oxygen: f64,
    pub oxidized_binder: f64,
    pub networked_binder: f64,
    pub inhibitor: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SinkKind {
    Ground,
    Evaporated,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialSink {
    pub kind: SinkKind,
    pub material: MaterialPayload,
}

impl MaterialPayload {
    pub fn volume(&self) -> f64 {
        self.fragments.iter().map(|f| f.volume).sum()
    }

    pub fn inventory(&self) -> Inventory {
        let mut total = Inventory::default();
        for f in &self.fragments {
            total.pigment += f.pigment;
            total.binder += f.binder;
            total.solvent += f.solvent;
            total.oxygen += f.oxygen * f.oxygen_capacity();
            total.oxidized_binder += f.oxidation * f.binder;
            total.networked_binder += f.network * f.binder;
            total.inhibitor += f.inhibitor * f.binder;
        }
        total
    }

    pub fn scale(&mut self, factor: f64) {
        if factor <= 0.0 {
            self.fragments.clear();
            return;
        }
        for fragment in &mut self.fragments {
            fragment.scale(factor);
        }
    }

    pub fn scaled(&self, factor: f64) -> Self {
        let mut material = self.clone();
        material.scale(factor);
        material
    }

    pub fn append(&mut self, other: &Self, factor: f64) {
        if factor <= 0.0 {
            return;
        }
        for source in &other.fragments {
            let mut fragment = source.clone();
            fragment.scale(factor);
            if let Some(last) = self.fragments.last_mut() {
                let proportional = |a: f64, b: f64| {
                    let left = a * fragment.volume;
                    let right = b * last.volume;
                    (left - right).abs() <= 4.0 * f64::EPSILON * (left.abs() + right.abs())
                };
                if last.volume > 0.0
                    && fragment.volume > 0.0
                    && last.recipe == fragment.recipe
                    && last.history == fragment.history
                    && last.history.tension_n_m == [0.0; 3]
                    && last.history.moment_n == [0.0; 3]
                    && last.oxygen == fragment.oxygen
                    && last.oxidation == fragment.oxidation
                    && last.network == fragment.network
                    && last.inhibitor == fragment.inhibitor
                    && proportional(last.pigment, fragment.pigment)
                    && proportional(last.binder, fragment.binder)
                    && proportional(last.solvent, fragment.solvent)
                {
                    last.reference_area = 1.0;
                    last.volume += fragment.volume;
                    last.pigment += fragment.pigment;
                    last.binder += fragment.binder;
                    last.solvent += fragment.solvent;
                    continue;
                }
            }
            self.fragments.push(fragment);
        }
    }

    pub fn blend(&mut self, other: &Self, weight: f64) {
        self.scale(1.0 - weight);
        self.mix_in(other, weight);
    }

    pub(crate) fn set_mobile_coverage(&mut self, coverage: f32) {
        assert!(coverage.is_finite() && coverage > 0.0 && coverage <= 1.0);
        let gel = super::constitutive::Parameters::default().gel_network;
        for fragment in &mut self.fragments {
            if fragment.network <= gel
                && fragment.history.tension_n_m == [0.0; 3]
                && fragment.history.moment_n == [0.0; 3]
            {
                fragment.history.coverage = coverage;
            }
        }
    }

    pub fn mix_in(&mut self, other: &Self, factor: f64) {
        if factor <= 0.0 {
            return;
        }
        for source in &other.fragments {
            let mut fragment = source.clone();
            fragment.scale(factor);
            let gel = super::constitutive::Parameters::default().gel_network;
            let compatible = self.fragments.iter_mut().find(|f| {
                f.history == fragment.history
                    && f.history.tension_n_m == [0.0; 3]
                    && f.history.moment_n == [0.0; 3]
                    && ((f.binder > 0.0 && fragment.binder > 0.0
                        && f.network <= gel && fragment.network <= gel)
                        || (f.recipe == fragment.recipe
                            && f.oxygen == fragment.oxygen
                            && f.oxidation == fragment.oxidation
                            && f.network == fragment.network
                            && f.inhibitor == fragment.inhibitor))
            });
            if let Some(existing) = compatible {
                if existing.recipe != fragment.recipe {
                    *existing = super::mixture::mix_mobile(
                        &[existing.clone(), fragment],
                        1.0,
                        existing.history,
                    ).expect("unsupported homogeneous material mixture");
                    continue;
                }
                let weighted = |a: f64, b: f64, wa: f64, wb: f64| {
                    if wa + wb > 0.0 {
                        (a * wa + b * wb) / (wa + wb)
                    } else {
                        a
                    }
                };
                existing.oxygen = weighted(
                    existing.oxygen,
                    fragment.oxygen,
                    existing.oxygen_capacity(),
                    fragment.oxygen_capacity(),
                );
                existing.oxidation = weighted(
                    existing.oxidation,
                    fragment.oxidation,
                    existing.binder,
                    fragment.binder,
                );
                existing.network = weighted(
                    existing.network,
                    fragment.network,
                    existing.binder,
                    fragment.binder,
                );
                existing.inhibitor = weighted(
                    existing.inhibitor,
                    fragment.inhibitor,
                    existing.binder,
                    fragment.binder,
                );
                existing.reference_area = 1.0;
                existing.volume += fragment.volume;
                existing.pigment += fragment.pigment;
                existing.binder += fragment.binder;
                existing.solvent += fragment.solvent;
            } else {
                self.fragments.push(fragment);
            }
        }
    }

    pub fn split_top(&self, fraction: f64) -> (Self, Self) {
        self.split(fraction, None)
    }

    pub fn split_top_with_geometry(&self, fraction: f64, um_per_volume: f64) -> (Self, Self) {
        self.split(fraction, Some(um_per_volume))
    }

    fn split(&self, fraction: f64, um_per_volume: Option<f64>) -> (Self, Self) {
        let mut below = self.clone();
        let mut top = Self::default();
        let mut left = fraction.clamp(0.0, self.volume());
        while left > 0.0 {
            let Some(mut fragment) = below.fragments.pop() else {
                break;
            };
            if fragment.volume <= 0.0 {
                continue;
            }
            let take = left.min(fragment.volume);
            left -= take;
            if take < fragment.volume {
                let mut rest = fragment.clone();
                let boundary = 1.0 - take / fragment.volume;
                if let Some(geometry) = um_per_volume {
                    rest.history = fragment.sliced_history(0.0, boundary, geometry);
                    fragment.history = fragment.sliced_history(boundary, 1.0, geometry);
                }
                rest.scale(boundary);
                if um_per_volume.is_some() {
                    rest.reference_area = fragment.reference_area;
                }
                below.fragments.push(rest);
            }
            let reference_area = fragment.reference_area;
            fragment.scale(take / fragment.volume);
            if um_per_volume.is_some() {
                fragment.reference_area = reference_area;
            }
            top.fragments.push(fragment);
        }
        top.fragments.reverse();
        (below, top)
    }

    pub fn remove_binder(&mut self, amount: f64) -> Self {
        self.remove_species(amount, false)
    }

    pub fn drain_binder(&mut self, amount: f64, start: f64, reached: f64) -> Self {
        let (through, above) = self.split_top_with_geometry(
            (self.volume() - start - reached).max(0.0),
            crate::surface::COAT_UM as f64,
        );
        let (below, mut contact) =
            through.split_top_with_geometry(reached, crate::surface::COAT_UM as f64);
        let binder: f64 = contact.fragments.iter().map(|f| f.binder).sum();
        let fraction = if binder > 0.0 {
            (amount / binder).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let mut removed = Self::default();
        for fragment in &mut contact.fragments {
            let take = fragment.binder * fraction;
            if take <= 0.0 {
                continue;
            }
            let mut out = fragment.clone();
            out.volume = take / crate::surface::COAT_UM as f64;
            out.pigment = 0.0;
            out.binder = take;
            out.solvent = 0.0;
            fragment.binder -= take;
            if fragment.oxygen_capacity() == 0.0 {
                fragment.oxygen = 0.0;
            }
            fragment.volume = (fragment.volume - out.volume).max(0.0);
            removed.fragments.push(out);
        }
        *self = below;
        self.append(&contact, 1.0);
        self.append(&above, 1.0);
        removed
    }

    pub fn remove_solvent(&mut self, amount: f64) -> Self {
        self.remove_species(amount, true)
    }

    fn remove_species(&mut self, amount: f64, solvent: bool) -> Self {
        let mut removed = Self::default();
        let mut left = amount.max(0.0);
        for fragment in &mut self.fragments {
            let available = if solvent {
                fragment.solvent
            } else {
                fragment.binder
            };
            let take = left.min(available);
            if take <= 0.0 {
                continue;
            }
            let mut out = fragment.clone();
            out.volume = 0.0;
            out.pigment = 0.0;
            out.binder = if solvent { 0.0 } else { take };
            out.solvent = if solvent { take } else { 0.0 };
            if solvent {
                fragment.solvent -= take;
            } else {
                fragment.binder -= take;
            }
            if fragment.oxygen_capacity() == 0.0 {
                fragment.oxygen = 0.0;
            }
            removed.fragments.push(out);
            left -= take;
        }
        removed
    }
}
