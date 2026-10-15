use super::*;
use crate::Crop;

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn owned_capture_survives_canvas_changes_and_drop() {
    assert_send_sync::<PreparedOptics>();
    let mut canvas = Canvas::new_window(32, 1.25, [0.3, 0.5, 0.2], None);
    canvas.wet.solv.resize(canvas.px.len(), 0.0);
    for i in 0..canvas.px.len() {
        canvas.height[i] = (i % 13) as f32 * 43.0;
        canvas.gloss[i] = (i % 5) as f32 * 0.2;
        canvas.wet.vol[i] = (i % 7) as f32 * 0.07;
        canvas.wet.solv[i] = (i % 11) as f32 * 3.0;
    }
    let expected_color = canvas.seen();
    let expected_gallery = canvas.seen_lit(115.0, 55.0, 1.0);
    let expected_relief = canvas.seen_lit(135.0, 25.0, 1.0);
    let prepared = canvas.prepare_optics();
    let frame = canvas.window();
    canvas.height.fill(0.0);
    canvas.px.fill([1.0; 3]);
    canvas.wet.vol.fill(0.0);
    drop(canvas);
    std::thread::spawn(move || {
        assert_eq!(prepared.frame(), frame);
        assert_eq!(prepared.dimensions(), (frame.w, frame.h));
        assert_eq!(prepared.color(), expected_color);
        assert_eq!(prepared.seen_lit(115.0, 55.0, 1.0), expected_gallery);
        assert_eq!(prepared.seen_lit(135.0, 25.0, 1.0), expected_relief);
    })
    .join()
    .unwrap();
}

#[test]
fn prepared_lighting_preserves_engines_and_cropped_frames() {
    for engine in 1..=5 {
        let mut canvas = Canvas::new_window(
            48,
            1.25,
            [0.3, 0.5, 0.2],
            Some(Crop {
                units: [250.0, 180.0, 700.0, 600.0],
                margin: 120.0,
            }),
        )
        .with_engine(engine);
        for i in 0..canvas.px.len() {
            canvas.height[i] = (i % 17) as f32 * 80.0;
            canvas.px[i] = [0.2, (i % 5) as f32 * 0.1, 0.4];
            canvas.gloss[i] = (i % 4) as f32 / 3.0;
        }
        let prepared = canvas.prepare_optics();
        assert_eq!(prepared.frame(), canvas.window());
        assert_eq!(prepared.color(), canvas.seen());
        for (az, el, strength) in [
            (115.0, 55.0, 1.0),
            (135.0, 25.0, 1.0),
            (45.0, 3.0, 0.4),
            (270.0, 89.0, 0.0),
        ] {
            assert_eq!(
                prepared.seen_lit(az, el, strength),
                canvas.seen_lit(az, el, strength)
            );
        }
        #[cfg(feature = "render-snapshot")]
        {
            let snapshot =
                crate::render_snapshot::RenderSnapshot::capture(&canvas, [0; 32], [0; 32]).unwrap();
            let from_snapshot = snapshot.into_prepared_optics();
            assert_eq!(from_snapshot.color(), prepared.color());
            assert_eq!(
                from_snapshot.seen_lit(115.0, 55.0, 1.0),
                prepared.seen_lit(115.0, 55.0, 1.0)
            );
        }
    }
}
