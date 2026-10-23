use crate::Rgb;
use rayon::prelude::*;

pub(crate) fn light(
    pixels: &mut [Rgb],
    surf: &[f32],
    w: usize,
    h: usize,
    px_mm: f32,
    strength: f32,
    gloss: f32,
) {
    let l = {
        let v = [-0.58f32, -0.58, 0.57];
        let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        [v[0] / n, v[1] / n, v[2] / n]
    };
    let k = 0.5 / (px_mm * 1000.0);
    pixels.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for x in 0..w {
            let at = |xx: usize, yy: usize| surf[yy.min(h - 1) * w + xx.min(w - 1)];
            let dx = (at(x + 1, y) - at(x.saturating_sub(1), y)) * k;
            let dy = (at(x, y + 1) - at(x, y.saturating_sub(1))) * k;
            let g = (dx * dx + dy * dy).sqrt();
            let lim = 1.0 / (1.0 + g / 1.2);
            let (dx, dy) = (dx * lim, dy * lim);
            let n = {
                let v = [-dx, -dy, 1.0];
                let m = (v[0] * v[0] + v[1] * v[1] + 1.0).sqrt();
                [v[0] / m, v[1] / m, v[2] / m]
            };
            let ndl = n[0] * l[0] + n[1] * l[1] + n[2] * l[2];
            let shade = 1.0 + strength * (ndl / l[2] - 1.0);
            let hv = [l[0], l[1], l[2] + 1.0];
            let hm = (hv[0] * hv[0] + hv[1] * hv[1] + hv[2] * hv[2]).sqrt();
            let ndh = ((n[0] * hv[0] + n[1] * hv[1] + n[2] * hv[2]) / hm).max(0.0);
            let flat = (l[2] + 1.0) / hm;
            let spec = (gloss * (ndh.powf(60.0) - flat.powf(60.0)).max(0.0)).min(gloss * 0.2);
            let p = &mut row[x];
            for c in 0..3 {
                p[c] = (p[c] * shade + spec).max(0.0);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::oil::store::{DepositMode, MaterialError};
    use crate::{Canvas, Mask};

    fn canvas() -> Canvas {
        let mut canvas =
            Canvas::new_window_with_material(3, 1.0, [0.8; 3], None, 9, 1).with_size_mm(50.0);
        canvas
            .edit_material(|store| {
                for i in 0..9 {
                    store.with_column_mut(i, |column, _| {
                        column.state.support_z_um = if i % 3 == 1 { 10000.0 } else { 0.0 };
                        Ok(())
                    })?;
                    store.deposit(
                        i,
                        crate::oil::store::tests::paint(20.0 * store.pixel_area(), 1, 1.0),
                        DepositMode::Cover,
                    )?;
                }
                Ok(())
            })
            .unwrap();
        canvas
    }

    fn store_bytes(canvas: &Canvas) -> Vec<u8> {
        let mut bytes = Vec::new();
        canvas
            .material_store()
            .unwrap()
            .write_to(&mut bytes)
            .unwrap();
        bytes
    }

    #[test]
    fn material_relief_matches_the_lighting_operator_and_roundtrips_without_recoloring_paint() {
        let mut canvas = canvas();
        let before = store_bytes(&canvas);
        let unlit = canvas.seen();
        let pickup = canvas.sample(500.0, 500.0);
        assert_ne!(pickup, [0.8; 3]);
        let mut legacy =
            Canvas::new_window_with_material(3, 1.0, [0.8; 3], None, 9, 0).with_size_mm(50.0);
        legacy.px = (0..9).map(|i| canvas.material_look_px(i)).collect();
        legacy.height = canvas.wet_surface();
        legacy.gloss.fill(1.0);
        legacy.relief(0.7, 0.15);
        canvas.relief(0.7, 0.15);
        assert_eq!(canvas.seen(), legacy.seen());
        assert_ne!(canvas.seen(), unlit);
        assert_eq!(store_bytes(&canvas), before);
        assert_eq!(canvas.sample(500.0, 500.0), pickup);
        let mut saved = Vec::new();
        canvas.write_state(&mut saved, "relief").unwrap();
        assert_eq!(&saved[..8], b"PAINTC23");
        let (restored, header) = Canvas::read_state(&mut saved.as_slice()).unwrap();
        assert_eq!(header, "relief");
        assert_eq!(restored.material_relief, Some([0.7, 0.15]));
        assert_eq!(restored.seen(), canvas.seen());
        assert_eq!(store_bytes(&restored), before);
        assert_eq!(restored.sample(500.0, 500.0), pickup);
        let mut unlit = restored.clone();
        unlit.material_relief = None;
        let mut older = Vec::new();
        unlit.write_state(&mut older, "relief").unwrap();
        assert_eq!(&older[..8], b"PAINTC22");
        assert!(
            Canvas::read_state(&mut older.as_slice())
                .unwrap()
                .0
                .material_relief
                .is_none()
        );
        saved[54..58].copy_from_slice(&f32::NAN.to_le_bytes());
        assert!(Canvas::read_state(&mut saved.as_slice()).is_err());
    }

    #[test]
    fn arbitrary_material_rgb_edits_are_rejected_before_the_closure_runs() {
        let mut canvas = canvas();
        let before = store_bytes(&canvas);
        let color = canvas.seen();
        assert!(matches!(
            canvas.apply(|_, _, _| panic!("unsupported RGB callback ran")),
            Err(MaterialError::Unsupported(_))
        ));
        assert!(matches!(
            canvas.apply_masked(&Mask::full(canvas.frame()), |_, _, _, _| panic!(
                "unsupported masked RGB callback ran"
            )),
            Err(MaterialError::Unsupported(_))
        ));
        assert_eq!(store_bytes(&canvas), before);
        assert_eq!(canvas.seen(), color);
    }
}
