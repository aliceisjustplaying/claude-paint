use crate::{
    COAT_UM, Canvas, Frame, Gesture, Held, Mask, Rgb, Tally,
    pigment::Pigment,
    rag::{
        Rag,
        transport::{Material, SolventLedger},
    },
    wet::over_share,
};

pub const MAX_BURIED_LAYERS: usize = 4;
pub const WORK_DEPTH_UM: f64 = 160.0;

#[derive(Clone)]
pub(crate) struct Layers {
    pub(crate) buried: Vec<Vec<Material>>,
    work_um: Vec<f64>,
    opened_mm3: Vec<f64>,
}

impl Layers {
    fn compact(&mut self, i: usize) {
        while self.buried[i].len() > MAX_BURIED_LAYERS {
            let oldest = self.buried[i].remove(0);
            self.buried[i][0].add(oldest);
        }
    }
}

#[derive(Clone)]
pub struct WorkingSurface {
    canvas: Canvas,
}

fn field_material(c: &Canvas, i: usize) -> Material {
    let area = (c.px_mm() as f64).powi(2);
    Material::picked(
        c.wet.vol[i] as f64 * area * COAT_UM as f64 / 1000.0,
        c.wet.solv.get(i).copied().unwrap_or(0.0) as f64 * area / 1000.0,
        c.wet.lat[i],
        c.wet.hide[i],
        c.wet.clock.px.get(i).map_or(0.0, |p| p.cure),
    )
}

impl Canvas {
    pub(crate) fn buried_height_um(&self, i: usize) -> f32 {
        self.wet.working_layers.as_ref().map_or(0.0, |layers| {
            (layers.buried[i]
                .iter()
                .map(|m| m.paint_mm3 + m.solvent_mm3)
                .sum::<f64>()
                * 1000.0
                / (self.px_mm() as f64).powi(2)) as f32
        })
    }

    pub(crate) fn begin_working_pass(&mut self) {
        let mut layers = self.wet.working_layers.take().unwrap();
        self.wet.ensure_solvent();
        for i in 0..self.wet.vol.len() {
            if self.wet.vol[i] > 0.0 {
                layers.buried[i].push(field_material(self, i));
                self.wet.solv[i] = 0.0;
            }
            self.wet.vol[i] = 0.0;
            self.wet.cover[i] = 1.0;
            self.wet.stroke[i] = 0;
            self.wet.touched[i] = 0;
            self.wet.floor[i] = 0.0;
            if let Some(p) = self.wet.clock.px.get_mut(i) {
                p.cure = 0.0;
                p.seen = 0.0;
            }
        }
        layers.work_um.fill(0.0);
        layers.opened_mm3.fill(0.0);
        self.wet.working_layers = Some(layers);
    }

    pub(crate) fn open_working_layer(
        &mut self,
        i: usize,
        pressure: f32,
        exposure: f64,
        secs: f32,
        slip_mm: f32,
        width_mm: f32,
    ) {
        let area = (self.px_mm() as f64).powi(2);
        let Some(layers) = &mut self.wet.working_layers else {
            return;
        };
        let shear = if slip_mm > 0.0 {
            1.0 + (slip_mm / width_mm).clamp(0.0, 1.0) as f64
        } else {
            0.35 * secs as f64 / crate::rag::pace::BLOT_PRESS
        };
        layers.work_um[i] +=
            (0.5 + WORK_DEPTH_UM * (pressure.clamp(0.0, 1.0) as f64).powi(3)) * exposure * shear;
        let mut wanted = (layers.work_um[i] * area / 1000.0 - layers.opened_mm3[i]).max(0.0);
        let mut got = Material::default();
        while wanted > 1e-12 {
            let Some(top) = layers.buried[i].last_mut() else {
                break;
            };
            let part = top.take((wanted / top.paint_mm3).min(1.0));
            wanted -= part.paint_mm3;
            layers.opened_mm3[i] += part.paint_mm3;
            got.add(part);
            if top.paint_mm3 <= 0.0 {
                layers.buried[i].pop();
            }
        }
        layers.compact(i);
        if got.paint_mm3 <= 0.0 {
            return;
        }
        let mut total = field_material(self, i);
        total.add(got);
        self.wet.vol[i] = (total.paint_mm3 * 1000.0 / area / COAT_UM as f64) as f32;
        self.wet.solv[i] = (total.solvent_mm3 * 1000.0 / area) as f32;
        self.wet.lat[i] = total.latent_moment.map(|m| (m / total.paint_mm3) as f32);
        self.wet.hide[i] = total.property_moment.map(|m| (m / total.paint_mm3) as f32);
        self.wet.clock.px[i].cure = (total.cure_moment / total.paint_mm3) as f32;
    }
}

impl WorkingSurface {
    pub fn new(mut canvas: Canvas) -> Result<Self, &'static str> {
        if canvas.engine < 3
            || canvas.hand_time().is_some()
            || canvas.absorb_any
            || canvas.drawing.is_some()
            || canvas.f.w != canvas.f.full_w
            || canvas.f.h != canvas.f.full_h
            || canvas.wet.working_layers.is_some()
        {
            return Err(
                "working layers require a whole oil-ground engine-3+ canvas, no drawing or hand-time aging",
            );
        }
        if canvas
            .wet
            .vol
            .iter()
            .zip(&canvas.wet.cover)
            .any(|(&v, &cover)| !v.is_finite() || v < 0.0 || (v > 0.0 && cover != 1.0))
            || canvas.wet.solv.iter().any(|&v| !v.is_finite() || v < 0.0)
            || canvas
                .wet
                .lat
                .iter()
                .flatten()
                .chain(canvas.wet.hide.iter().flatten())
                .any(|v| !v.is_finite())
            || canvas.wet.clock.px.iter().enumerate().any(|(i, p)| {
                !p.cure.is_finite() || (canvas.wet.vol[i] > 0.0 && p.cure >= crate::drying::GEL)
            })
        {
            return Err("working layers require finite, fully covered open paint");
        }
        let n = canvas.wet.vol.len();
        canvas.wet.ensure_solvent();
        if canvas.wet.clock.px.is_empty() {
            canvas.wet.clock.px = vec![crate::drying::Px::FRESH; n];
        }
        canvas.wet.working_layers = Some(Box::new(Layers {
            buried: vec![Vec::new(); n],
            work_um: vec![0.0; n],
            opened_mm3: vec![0.0; n],
        }));
        Ok(Self { canvas })
    }

    pub fn lay_drag(
        &mut self,
        held: &mut Held,
        gesture: &Gesture,
        clip: Option<&Mask>,
    ) -> Result<(), &'static str> {
        let mut next = self.canvas.clone();
        next.begin_working_pass();
        let mut layers = next.wet.working_layers.take().unwrap();
        let protected = (next.px.clone(), next.height.clone());
        let area = (next.px_mm() as f64).powi(2);
        for (i, h) in next.height.iter_mut().enumerate() {
            *h += (layers.buried[i]
                .iter()
                .map(|m| m.paint_mm3 + m.solvent_mm3)
                .sum::<f64>()
                * 1000.0
                / area) as f32;
        }
        next.base = None;
        let mut brush = held.clone();
        next.drag(&mut brush, gesture, clip);
        if next.px != protected.0
            || next
                .wet
                .vol
                .iter()
                .zip(&next.wet.cover)
                .any(|(&v, &cover)| v > 0.0 && (!v.is_finite() || cover != 1.0))
        {
            return Err("layer deposition must preserve the baked picture and full coverage");
        }
        next.height = protected.1;
        next.base = None;
        for i in 0..next.wet.vol.len() {
            if next.wet.vol[i] > 0.0 {
                layers.compact(i);
            }
        }
        next.wet.working_layers = Some(layers);
        self.canvas = next;
        *held = brush;
        Ok(())
    }

    pub fn rag(&self, width: f32, seed: u64) -> Rag {
        Rag::new(width, seed).with_transport(&self.canvas)
    }
    pub fn dip(&mut self, rag: &mut Rag, amount: f32) {
        rag.dip(amount, self.canvas.now_min(), self.canvas.tally_mut());
    }
    pub fn refold(&mut self, rag: &mut Rag) {
        rag.refold_at(self.canvas.now_min(), self.canvas.tally_mut());
    }
    pub fn age_rag(&self, rag: &mut Rag) {
        rag.age_transport(&self.canvas);
    }
    pub fn wipe(&mut self, rag: &mut Rag, pts: &[(f32, f32)], pressure: &[f32], seed: u64) -> f64 {
        self.canvas.rag_wipe(rag, pts, pressure, seed)
    }
    pub fn frame(&self) -> Frame {
        self.canvas.frame()
    }
    pub fn tally(&self) -> Tally {
        self.canvas.tally()
    }
    pub fn px_mm(&self) -> f32 {
        self.canvas.px_mm()
    }
    pub fn mm_per_unit(&self) -> f32 {
        self.canvas.mm_per_unit()
    }
    pub fn protected_pixels(&self) -> &[Rgb] {
        self.canvas.pixels()
    }
    pub fn protected_surface_um(&self) -> &[f32] {
        self.canvas.surface_um()
    }
    pub fn solvent_ledger(&self) -> Option<SolventLedger> {
        self.canvas.rag_solvent_ledger()
    }

    pub fn inventory(&self) -> Material {
        let mut total = Material::default();
        for i in 0..self.canvas.wet.vol.len() {
            total.add(field_material(&self.canvas, i));
            for material in &self.canvas.wet.working_layers.as_ref().unwrap().buried[i] {
                total.add(*material);
            }
        }
        total
    }

    pub fn working_paint_mm3(&self) -> f64 {
        (0..self.canvas.wet.vol.len())
            .map(|i| field_material(&self.canvas, i).paint_mm3)
            .sum()
    }

    pub fn buried_paint_mm3(&self) -> f64 {
        self.inventory().paint_mm3 - self.working_paint_mm3()
    }
    pub fn opened_paint_mm3(&self) -> f64 {
        self.canvas
            .wet
            .working_layers
            .as_ref()
            .unwrap()
            .opened_mm3
            .iter()
            .sum()
    }

    fn projection(&self) -> (Canvas, Vec<Rgb>) {
        let mut image = self.canvas.clone();
        let layers = image.wet.working_layers.take().unwrap();
        let area = (image.px_mm() as f64).powi(2);
        let mut colors = self.canvas.px.clone();
        for (i, stack) in layers.buried.iter().enumerate() {
            for material in stack {
                let v = material.paint_mm3;
                if v <= 0.0 {
                    continue;
                }
                let coats = (v * 1000.0 / area / COAT_UM as f64) as f32;
                let lat = material.latent_moment.map(|m| (m / v) as f32);
                let color = mixbox::latent_to_linear_float_rgb(&lat);
                colors[i] = over_share(
                    Pigment::masstone(color, (material.property_moment[0] / v) as f32),
                    colors[i],
                    coats,
                    1.0,
                );
                image.wet.vol[i] += coats;
                image.wet.solv[i] += (material.solvent_mm3 * 1000.0 / area) as f32;
            }
            let coats = self.canvas.wet.vol[i];
            if stack.is_empty() {
                colors[i] = self.canvas.look_px(i);
            } else if coats > 0.0 {
                let color = mixbox::latent_to_linear_float_rgb(&self.canvas.wet.lat[i]);
                colors[i] = over_share(
                    Pigment::masstone(color, self.canvas.wet.hide[i][0]),
                    colors[i],
                    coats,
                    1.0,
                );
            }
            if image.engine >= 4 {
                let gloss = crate::lerp(
                    image.gloss[i],
                    1.0,
                    crate::smoothstep(0.0, 0.1, image.wet.vol[i]),
                );
                colors[i] = crate::canvas::haze(colors[i], gloss);
            }
        }
        (image, colors)
    }

    pub fn seen(&self) -> Vec<Rgb> {
        self.projection().1
    }
    pub fn seen_lit(&self, azimuth: f32, elevation: f32, strength: f32) -> Vec<Rgb> {
        let (image, colors) = self.projection();
        image.light_colors(colors, azimuth, elevation, strength)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Linen, Paint, Tool, color::luminance};

    fn base() -> Canvas {
        let mut c = Canvas::new(96, 1.0, [0.4, 0.3, 0.2])
            .with_engine(5)
            .with_size_mm(100.0)
            .with_linen(Linen::fine(23));
        c.wet.ensure_solvent();
        c.wet.vol.fill(0.8);
        c.wet
            .lat
            .fill(mixbox::linear_float_rgb_to_latent(&[0.04, 0.03, 0.02]));
        c.wet.hide.fill([1.4, 0.6, 1.0, 0.0, 0.8]);
        c.wet.clock.px = vec![crate::drying::Px::FRESH; c.wet.vol.len()];
        for p in &mut c.wet.clock.px {
            p.cure = 0.12;
        }
        c
    }

    fn passage() -> WorkingSurface {
        let mut w = WorkingSurface::new(base()).unwrap();
        w.canvas.begin_working_pass();
        w.canvas.wet.vol.fill(0.6);
        w.canvas
            .wet
            .lat
            .fill(mixbox::linear_float_rgb_to_latent(&[0.85, 0.76, 0.58]));
        w.canvas.wet.hide.fill([1.6, 0.4, 1.1, 0.0, 0.9]);
        w.canvas.wet.solv.fill(2.0);
        for p in &mut w.canvas.wet.clock.px {
            p.cure = 0.04;
        }
        w
    }

    fn total(w: &WorkingSurface, r: &Rag) -> Material {
        let mut sum = w.inventory();
        let cloth = r.transport.as_ref().unwrap();
        sum.add(cloth.inventory());
        sum.add(cloth.waste);
        sum.solvent_mm3 += cloth.evaporated_solvent_mm3 - cloth.solvent_input_mm3;
        if let Some(ledger) = w.solvent_ledger() {
            sum.solvent_mm3 += ledger.evaporated_mm3 + ledger.discarded_trace_mm3;
        }
        sum
    }

    fn close(a: Material, b: Material) {
        let near = |x: f64, y: f64| assert!((x - y).abs() < 3e-6 * x.abs().max(1.0), "{x} != {y}");
        near(a.paint_mm3, b.paint_mm3);
        near(a.solvent_mm3, b.solvent_mm3);
        near(a.cure_moment, b.cure_moment);
        for (x, y) in a.latent_moment.into_iter().zip(b.latent_moment) {
            near(x, y);
        }
        for (x, y) in a.property_moment.into_iter().zip(b.property_moment) {
            near(x, y);
        }
    }

    #[test]
    fn burying_a_pass_keeps_color_gallery_geometry_and_inventory() {
        let mut w = WorkingSurface::new(base()).unwrap();
        let before = (w.seen(), w.seen_lit(115.0, 55.0, 1.0), w.inventory());
        w.canvas.begin_working_pass();
        assert_eq!(before.0, w.seen());
        assert_eq!(before.1, w.seen_lit(115.0, 55.0, 1.0));
        close(before.2, w.inventory());
        assert!(w.buried_paint_mm3() > 0.0 && w.working_paint_mm3() == 0.0);
    }

    #[test]
    fn splitting_identical_paint_does_not_change_its_optics() {
        let mut w = WorkingSurface::new(base()).unwrap();
        let before = w.seen();
        w.canvas.begin_working_pass();
        for i in 0..w.canvas.wet.vol.len() {
            let m = w.canvas.wet.working_layers.as_mut().unwrap().buried[i][0].take(0.5);
            w.canvas.wet.vol[i] = 0.4;
            w.canvas.wet.lat[i] = m.latent_moment.map(|x| (x / m.paint_mm3) as f32);
            w.canvas.wet.hide[i] = m.property_moment.map(|x| (x / m.paint_mm3) as f32);
            w.canvas.wet.clock.px[i].cure = (m.cure_moment / m.paint_mm3) as f32;
        }
        for (a, b) in before.into_iter().zip(w.seen()) {
            for (x, y) in a.into_iter().zip(b) {
                assert!((x - y).abs() < 2e-6);
            }
        }
    }

    #[test]
    fn a_single_light_pass_over_a_dried_picture_matches_the_normal_brush() {
        let mut c = base();
        c.wet.touch(0, 0, 96, 96);
        c.dry();
        let mut w = WorkingSurface::new(c.clone()).unwrap();
        let mut b = Held::new(Tool::hog_flat(100.0), 71).with_engine(5);
        b.load(Paint::body([0.85, 0.76, 0.58]).with_stiff(0.7), 0.7);
        let mut normal = b.clone();
        let g = Gesture::line((150.0, 500.0), (850.0, 500.0))
            .pressure(0.2, 0.2)
            .shake(0.0);
        c.drag(&mut normal, &g, None);
        w.lay_drag(&mut b, &g, None).unwrap();
        assert_eq!(c.seen(), w.seen());
        assert_eq!(c.seen_lit(115.0, 55.0, 1.0), w.seen_lit(115.0, 55.0, 1.0));
        assert_eq!(c.pixels(), w.protected_pixels());
    }

    #[test]
    fn light_contact_keeps_the_dark_layer_separate_and_scrubbing_entrains_it() {
        let start = passage();
        let i = start.canvas.f.index(500.0, 500.0);
        let dark = start.canvas.wet.working_layers.as_ref().unwrap().buried[i][0];
        let mut light = start.clone();
        let mut lr = light.rag(126.0, 7);
        let initial = total(&light, &lr);
        light.wipe(&mut lr, &[(250.0, 500.0), (750.0, 500.0)], &[0.15], 41);
        close(initial, total(&light, &lr));
        assert_eq!(
            light.canvas.wet.working_layers.as_ref().unwrap().buried[i][0],
            dark
        );
        let mut scrub = start.clone();
        let mut sr = scrub.rag(126.0, 7);
        scrub.wipe(
            &mut sr,
            &[
                (250.0, 500.0),
                (750.0, 500.0),
                (250.0, 500.0),
                (750.0, 500.0),
            ],
            &[0.95],
            41,
        );
        close(initial, total(&scrub, &sr));
        let remaining = scrub.canvas.wet.working_layers.as_ref().unwrap().buried[i]
            .first()
            .map_or(0.0, |m| m.paint_mm3);
        assert!(remaining < 0.8 * dark.paint_mm3);
        assert!(luminance(scrub.seen()[i]) < luminance(light.seen()[i]));
        assert_eq!(light.protected_pixels(), scrub.protected_pixels());
        assert_eq!(light.protected_surface_um(), scrub.protected_surface_um());
    }

    #[test]
    fn layered_cloth_exchange_refold_and_redip_conserve_every_material_moment() {
        let mut w = passage();
        let mut r = w.rag(126.0, 7);
        let initial = total(&w, &r);
        let fixed = (
            w.protected_pixels().to_vec(),
            w.protected_surface_um().to_vec(),
        );
        w.dip(&mut r, 0.95);
        for pass in 0..3 {
            w.wipe(&mut r, &[(250.0, 500.0), (750.0, 500.0)], &[0.9], 41 + pass);
            close(initial, total(&w, &r));
        }
        for _ in 0..12 {
            w.refold(&mut r);
        }
        w.dip(&mut r, 0.6);
        w.wipe(&mut r, &[(650.0, 700.0), (850.0, 700.0)], &[0.5], 41);
        close(initial, total(&w, &r));
        assert_eq!(fixed.0, w.protected_pixels());
        assert_eq!(fixed.1, w.protected_surface_um());
        r.transport.as_mut().unwrap().discard();
        close(initial, total(&w, &r));
    }

    #[test]
    fn brush_deposition_has_one_owner_and_does_not_mix_the_buried_pass() {
        let mut w = WorkingSurface::new(base()).unwrap();
        let before = w.inventory();
        let fixed = (
            w.protected_pixels().to_vec(),
            w.protected_surface_um().to_vec(),
        );
        let mut b = Held::new(Tool::hog_flat(100.0), 19).with_engine(5);
        let paint = Paint::body([0.85, 0.76, 0.58]).with_stiff(0.4);
        b.load(paint, 0.3);
        let unit = (w.mm_per_unit() as f64).powi(2) * COAT_UM as f64 / 1000.0;
        let carried = b.carried();
        w.lay_drag(
            &mut b,
            &Gesture::line((150.0, 500.0), (850.0, 500.0))
                .pressure(0.2, 0.2)
                .shake(0.0),
            None,
        )
        .unwrap();
        let after = w.inventory();
        let left = b.carried();
        assert!(
            (before.paint_mm3 + carried.0 * unit - after.paint_mm3 - left.0 * unit).abs() < 1e-4
        );
        assert!(
            (before.solvent_mm3 + carried.1 * unit - after.solvent_mm3 - left.1 * unit).abs()
                < 1e-4
        );
        let mut buried = Material::default();
        for stack in &w.canvas.wet.working_layers.as_ref().unwrap().buried {
            for m in stack {
                buried.add(*m);
            }
        }
        close(before, buried);
        assert_eq!(fixed.0, w.protected_pixels());
        assert_eq!(fixed.1, w.protected_surface_um());
    }

    #[test]
    fn compaction_is_bounded_conservative_and_does_not_run_on_untouched_pixels() {
        let mut w = WorkingSurface::new(base()).unwrap();
        for _ in 0..5 {
            w.canvas.begin_working_pass();
            w.canvas.wet.vol[0] = 0.2;
            w.canvas.wet.lat[0] = mixbox::linear_float_rgb_to_latent(&[0.2, 0.3, 0.4]);
        }
        let before = w.inventory();
        let untouched = w.canvas.wet.working_layers.as_ref().unwrap().buried[1].clone();
        w.canvas.begin_working_pass();
        w.canvas.open_working_layer(0, 0.1, 0.01, 0.02, 3.0, 12.6);
        close(before, w.inventory());
        assert!(w.canvas.wet.working_layers.as_ref().unwrap().buried[0].len() <= MAX_BURIED_LAYERS);
        assert_eq!(
            untouched,
            w.canvas.wet.working_layers.as_ref().unwrap().buried[1]
        );
    }

    #[test]
    fn unsupported_aging_checkpoint_and_invalid_inputs_are_explicit() {
        let mut timed = base();
        timed.set_hand_time(Some(1.0));
        assert!(WorkingSurface::new(timed).is_err());
        let mut partial = base();
        partial.wet.cover[0] = 0.5;
        assert!(WorkingSurface::new(partial).is_err());
        let mut invalid = base();
        invalid.wet.lat[0][0] = f32::NAN;
        assert!(WorkingSurface::new(invalid).is_err());
        let mut w = WorkingSurface::new(base()).unwrap();
        let mut output = Vec::new();
        assert!(w.canvas.write_state(&mut output, "").is_err());
        assert!(output.is_empty());
        let before = w.inventory();
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| w.canvas.wait(1.0))).is_err()
        );
        close(before, w.inventory());
        let tally = w.tally();
        let colors = w.seen();
        let _ = w.seen_lit(115.0, 55.0, 1.0);
        assert_eq!(tally, w.tally());
        assert_eq!(colors, w.seen());
        close(before, w.inventory());
    }
}
