//! Engine 7: an opt-in, faster engine that paints the same Lua logs as
//! engine 5. A log opts in with the head line `--@ engine 7`; `crate::ENGINE`
//! (the default for a new painting) stays 5, and every log replays with the
//! engine its head names (`crate::SUPPORTED_ENGINES`). Engine 6 does not
//! exist.
//!
//! The hybrid rule: engine 7 is engine 5 for every verb and physical event
//! it doesn't reimplement here. Every `engine >= n` test in the crate is true
//! for 7 as for 5, so knife, rag, stipple, soak, pencil, varnish, solvent
//! flow, `wait`, bake and aging run engine 5's code unchanged. Engine 7
//! diverges only behind `engine7::active(canvas.engine)` (`Surf::e7` inside a
//! brush) at these central call sites, all owned by the integrator:
//!
//! - `bristle::drag_on` and the touch path call `exchange::exchange` instead
//!   of `bristle::exchange` (same arguments).
//! - `handling::finish_plan` sizes `work`'s stroke rectangles with
//!   `Footprint::planned` instead of `bristle::footprint`.
//! - `Canvas::surf` takes the brush contact surface from
//!   `contact_cache::BaseCache` (wired when that cache lands).
//!
//! Lane modules: `footprint` (STIL-67), `exchange` and `window` (STIL-68),
//! `add` (STIL-69), `contact_cache` (STIL-70). The contracts are frozen in
//! handoffs/engine-next/CONTRACTS-e7.md; this file and the signatures there
//! change only through the integrator (STIL-66).

#[allow(dead_code)]
pub(crate) mod add;
pub mod contact_cache;
pub mod exchange;
pub mod footprint;
pub mod window;

pub use footprint::Footprint;

/// The engine version that selects engine 7.
pub const ENGINE: u32 = 7;

/// A global (whole-canvas) pixel rectangle `(x0, y0, x1, y1)`, end-exclusive,
/// as `bristle::Rect`. `None` where one is optional means no pixels.
pub type Rect = (usize, usize, usize, usize);

/// Whether a canvas of engine `v` paints with engine 7's code.
#[inline]
pub fn active(v: u32) -> bool {
    v == ENGINE
}

/// Whether `E7_STATS=1` is set: engine 7 then prints its `E7BATCH` and
/// `E7NODE` lines on stderr (format in CONTRACTS-e7.md).
pub fn stats() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("E7_STATS").is_some_and(|v| v == "1"))
}

/// A stroke's seed from its logical position in the log: the log's seed,
/// the chunk, the verb call within the chunk and the stroke within the
/// verb. Never from a worker, a tile index or completion order, so tiling
/// and scheduling can change without changing any stroke's variation.
pub fn stroke_seed(log_seed: u64, chunk: u32, call: u32, stroke: u32) -> u64 {
    let mut h = log_seed ^ 0xE7E7_5EED_0000_0007;
    for v in [chunk as u64, call as u64, stroke as u64] {
        h = (h ^ v).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        h ^= h >> 29;
    }
    h
}

/// One unit of recorded work (a stroke, or a tile of strokes), for the
/// evidence trace and an optional later recorder. Not a scheduler: nothing
/// runs nodes, and no recorder is built unless the trace shows a width of
/// at least 3 with brush, palette, time and observation edges included.
#[derive(Clone, Debug)]
pub struct Node<T> {
    /// Every pixel the work reads or writes (`Footprint::exact`).
    pub exact: Option<Rect>,
    /// The legacy safety bound (`Footprint::conservative`).
    pub conservative: Option<Rect>,
    /// Simulated hand time (s) at its start and end.
    pub t0: f64,
    pub t1: f64,
    /// The stroke id it paints with (`Canvas::next_stroke_ids`).
    pub stroke_id: u32,
    /// `stroke_seed` of its logical position.
    pub seed: u64,
    pub payload: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stroke_seeds_depend_on_every_logical_position() {
        let s = stroke_seed(1, 2, 3, 4);
        assert_eq!(s, stroke_seed(1, 2, 3, 4));
        for other in [stroke_seed(0, 2, 3, 4), stroke_seed(1, 1, 3, 4), stroke_seed(1, 2, 2, 4), stroke_seed(1, 2, 3, 5), stroke_seed(1, 3, 2, 4)] {
            assert_ne!(s, other);
        }
    }

    #[test]
    fn only_seven_is_engine_seven_and_six_is_unsupported() {
        assert!(active(7) && !active(5) && !active(6));
        for v in [1, 2, 3, 4, 5, 7] {
            assert!(crate::supported_engine(v), "{v}");
        }
        for v in [0, 6, 8] {
            assert!(!crate::supported_engine(v), "{v}");
        }
        assert_eq!(crate::ENGINE, 5);
    }

    #[test]
    fn an_engine_7_canvas_reopens_as_engine_7_and_engine_6_does_not() {
        for (v, want) in [(7, Some(7)), (6, None)] {
            let c = crate::canvas::Canvas::new_window(2, 1.0, [0.1; 3], None).with_engine(v);
            let mut b = Vec::new();
            c.write_state(&mut b, "").unwrap();
            let back = crate::canvas::Canvas::read_state(&mut std::io::Cursor::new(b));
            assert_eq!(back.ok().map(|(d, _)| d.engine()), want, "engine {v}");
        }
    }

    /// Until a lane replaces a stub, engine 7 paints exactly as engine 5.
    #[test]
    fn the_pass_through_paints_as_engine_5() {
        use crate::bristle::{Gesture, Held, Tool};
        use crate::color::hex;
        use crate::wet::Paint;
        let paint = |v: u32| {
            let mut c = crate::canvas::Canvas::new(160, 1.0, hex("#e8e0d0")).with_size_mm(200.0).with_engine(v);
            for (k, tool) in [Tool::hog_flat(12.0), Tool::filbert(8.0), Tool { point: 1.0, ..Tool::round_sable(4.0) }].into_iter().enumerate() {
                let mut h = Held::new(tool, 3 + k as u64).with_engine(v);
                h.load(Paint::body(hex(["#1a1612", "#c03020", "#f0f0e8"][k])), 1.0);
                c.drag(&mut h, &Gesture::new(vec![(20.0, 40.0 + 30.0 * k as f32), (90.0, 70.0), (140.0, 30.0 + 20.0 * k as f32)]), None);
            }
            assert!(c.wet.vol.iter().filter(|&&v| v > 0.0).count() > 500, "engine {v} laid too little");
            c.wet.vol.iter().chain(c.height.iter()).map(|v| v.to_bits()).collect::<Vec<_>>()
        };
        assert_eq!(paint(7), paint(5));
    }
}
