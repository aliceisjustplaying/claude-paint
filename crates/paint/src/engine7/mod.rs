pub(crate) mod contact_cache;
pub(crate) mod exchange;
pub(crate) mod exchange_simd;
pub(crate) mod footprint;
pub(crate) mod window;

pub type Rect = (usize, usize, usize, usize);

pub(crate) struct Features {
    pub footprints: bool,
    pub kernel: bool,
    pub contact_cache: bool,
}

pub(crate) fn features() -> &'static Features {
    static FEATURES: std::sync::OnceLock<Features> = std::sync::OnceLock::new();
    FEATURES.get_or_init(|| Features {
        footprints: std::env::var("PAINT_ENGINE7_FOOTPRINTS").as_deref() != Ok("0"),
        kernel: std::env::var("PAINT_ENGINE7_KERNEL").as_deref() != Ok("0"),
        contact_cache: std::env::var("PAINT_ENGINE7_CACHE").as_deref() != Ok("0"),
    })
}

#[derive(Clone)]
pub(crate) struct ContactState {
    frame: crate::Frame,
    px_mm: f32,
    generation: u64,
    cache: contact_cache::BaseCache,
}

impl crate::Canvas {
    pub(crate) fn engine7_base(&mut self) -> *const f32 {
        let px_mm = self.px_mm();
        if self.contact7.as_ref().is_none_or(|state| state.frame != self.f || state.px_mm != px_mm) {
            self.contact7 = Some(ContactState {
                frame: self.f,
                px_mm,
                generation: self.surf_gen,
                cache: contact_cache::BaseCache::new(self.f, px_mm),
            });
        }
        let state = self.contact7.as_mut().unwrap();
        if state.generation != self.surf_gen {
            state.cache.invalidate_all();
            state.generation = self.surf_gen;
        }
        state.cache.rebuild(&self.height).as_ptr()
    }

    pub(crate) fn engine7_height_changed(&mut self, local: Rect) {
        if let Some(state) = &mut self.contact7 {
            if state.generation.wrapping_add(1) != self.surf_gen {
                state.cache.invalidate_all();
            } else {
                state.cache.invalidate((local.0 + self.f.x0, local.1 + self.f.y0, local.2 + self.f.x0, local.3 + self.f.y0));
            }
            state.generation = self.surf_gen;
        }
    }
}

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
    fn engine7_cache_tracks_local_unknown_and_scale_changes() {
        let mut canvas = crate::Canvas::new_window(520, 1.0, [0.3; 3], None).with_engine(7);
        let check = |canvas: &mut crate::Canvas| {
            canvas.engine7_base();
            let expected = super::contact_cache::BaseCache::new(canvas.f, canvas.px_mm()).rebuild(&canvas.height).to_vec();
            let actual = canvas.contact7.as_mut().unwrap().cache.rebuild(&canvas.height);
            assert!(actual.iter().zip(expected).all(|(a, b)| (a - b).abs() < 1e-4));
        };
        check(&mut canvas);
        canvas.height[255 * 520 + 255] = 300.0;
        canvas.surf_gen += 1;
        canvas.engine7_height_changed((255, 255, 256, 256));
        check(&mut canvas);
        canvas.height[511 * 520 + 511] = 500.0;
        canvas.surf_gen += 1;
        check(&mut canvas);
        canvas.mm_per_unit *= 0.1;
        check(&mut canvas);
    }

    #[test]
    fn engine7_work_is_independent_of_worker_count() {
        let render = |workers| rayon::ThreadPoolBuilder::new().num_threads(workers).build().unwrap().install(|| {
            let mut canvas = crate::Canvas::new_window(240, 1.5, [0.3; 3], None).with_engine(7);
            canvas.set_hand_time(Some(0.1));
            let mask = crate::Mask::full(canvas.frame());
            let handling = crate::Handling::new(crate::Tool::filbert(18.0))
                .color(|x, _| if x < 450.0 { [0.6, 0.2, 0.1] } else { [0.1, 0.2, 0.6] })
                .length(25.0, 60.0).coverage(1.5);
            canvas.work(&mask, &handling, 47);
            let mut bytes = Vec::new();
            canvas.write_state(&mut bytes, "").unwrap();
            bytes
        });
        assert!(render(1) == render(4));
    }

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
