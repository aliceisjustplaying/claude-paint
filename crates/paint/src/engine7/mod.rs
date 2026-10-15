pub(crate) mod contact_cache;
pub(crate) mod exchange;
pub(crate) mod exchange_simd;
pub(crate) mod footprint;
pub(crate) mod window;

pub type Rect = (usize, usize, usize, usize);

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub exact: Option<Rect>,
    pub conservative: Option<Rect>,
    pub t0: f64,
    pub t1: f64,
    pub stroke_id: u64,
    pub seed: u64,
    pub payload: T,
}

pub fn stroke_seed(log_seed: u64, chunk: u64, call: u64, stroke: u64) -> u64 {
    let mut seed = log_seed;
    for position in [chunk, call, stroke] {
        seed ^= position.wrapping_add(0x9e3779b97f4a7c15);
        seed = (seed ^ (seed >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        seed = (seed ^ (seed >> 27)).wrapping_mul(0x94d049bb133111eb);
        seed ^= seed >> 31;
    }
    seed
}

#[cfg(test)]
mod tests {
    #[test]
    fn engine7_is_supported_without_changing_the_default() {
        assert_eq!(crate::ENGINE, 5);
        assert!(crate::supports_engine(7));
        assert!(!crate::supports_engine(6));
        assert!(!crate::supports_engine(0));
        assert!(!crate::supports_engine(8));
    }

    #[test]
    fn engine7_checkpoint_roundtrip() {
        let canvas = crate::Canvas::new_window(16, 1.0, [0.3; 3], None).with_engine(7);
        let mut bytes = Vec::new();
        canvas.write_state(&mut bytes, "engine=7").unwrap();
        let (restored, _) = crate::Canvas::read_state(&mut bytes.as_slice()).unwrap();
        assert_eq!(restored.engine(), 7);
        assert_eq!(restored.seen(), canvas.seen());
    }

    #[cfg(feature = "render-snapshot")]
    #[test]
    fn engine7_optical_snapshot_roundtrip() {
        let canvas = crate::Canvas::new_window(16, 1.0, [0.3; 3], None).with_engine(7);
        let snapshot = crate::render_snapshot::RenderSnapshot::capture(&canvas, [1; 32], [2; 32]).unwrap();
        let mut bytes = Vec::new();
        snapshot.write(&mut bytes).unwrap();
        let restored = crate::render_snapshot::RenderSnapshot::read(&mut bytes.as_slice()).unwrap();
        assert_eq!(restored.lit(115.0, 55.0, 1.0), canvas.seen_lit(115.0, 55.0, 1.0));
    }
}
