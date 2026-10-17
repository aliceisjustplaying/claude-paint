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
//!   `contact_cache::ContactCache::refresh`.
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

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// The engine version that selects engine 7.
pub const ENGINE: u32 = 7;

/// A global (whole-canvas) pixel rectangle `(x0, y0, x1, y1)`, end-exclusive,
/// as `bristle::Rect`. `None` where one is optional means no pixels.
pub type Rect = (usize, usize, usize, usize);

/// Whether a canvas of engine `v` paints with engine 7's code (engines 7
/// and 8: `crate::REACH_END`).
#[inline]
pub fn active(v: u32) -> bool {
    v >= ENGINE
}

/// Whether `E7_STATS=1` is set: engine 7 then prints its `E7BATCH` and
/// `E7NODE` lines on stderr (format in CONTRACTS-e7.md).
pub fn stats() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("E7_STATS").is_some_and(|v| v == "1"))
}

/// One batch's scheduling counts (`E7_STATS` only): each batch owns its
/// own, so canvases painting at the same time don't mix them.
#[derive(Default)]
pub(crate) struct BatchStats {
    /// Tiles whose predecessors are done and that haven't finished
    /// (queued or running), and the most there were.
    ready: AtomicUsize,
    peak_ready: AtomicUsize,
    /// Tiles running, and the most at once.
    running: AtomicUsize,
    peak_running: AtomicUsize,
    /// Every tile's time, summed (ns).
    tile_ns: AtomicU64,
}

impl BatchStats {
    /// `n` more tiles are ready to run.
    pub(crate) fn ready(&self, n: usize) {
        let now = self.ready.fetch_add(n, Ordering::Relaxed) + n;
        self.peak_ready.fetch_max(now, Ordering::Relaxed);
    }

    /// Run one ready tile's `work`, counted and timed. It stays ready
    /// until `work` returns.
    pub(crate) fn tile<T>(&self, work: impl FnOnce() -> T) -> T {
        let now = self.running.fetch_add(1, Ordering::Relaxed) + 1;
        self.peak_running.fetch_max(now, Ordering::Relaxed);
        let t = std::time::Instant::now();
        let v = work();
        self.tile_ns.fetch_add(t.elapsed().as_nanos() as u64, Ordering::Relaxed);
        self.running.fetch_sub(1, Ordering::Relaxed);
        self.ready.fetch_sub(1, Ordering::Relaxed);
        v
    }

    /// Print the batch's `E7BATCH` line: `tiles` painted in `wall` seconds.
    pub(crate) fn print(&self, tiles: usize, wall: f64) {
        static BATCH: AtomicUsize = AtomicUsize::new(0);
        let b = BATCH.fetch_add(1, Ordering::Relaxed) + 1;
        let tile_secs = self.tile_ns.load(Ordering::Relaxed) as f64 * 1e-9;
        eprintln!("E7BATCH	{b}	{tiles}	{}	{}	{wall:.6}	{tile_secs:.6}", self.peak_ready.load(Ordering::Relaxed), self.peak_running.load(Ordering::Relaxed));
    }
}

/// Print a `work` stroke's `E7NODE` line (`E7_STATS` only). Seeds and
/// times per stroke aren't recorded yet: `-`.
pub(crate) fn trace_stroke(sf: &crate::bristle::Surf, held: &crate::bristle::Held, g: &crate::bristle::Gesture, id: u32) {
    let r = |r: Option<Rect>| r.map_or("-".to_string(), |(x0, y0, x1, y1)| format!("{x0},{y0},{x1},{y1}"));
    let exact = Footprint::exact(held, g, id, sf.scale, sf.fw, sf.fh);
    let conservative = Footprint::conservative(&held.tool, g, sf.scale, sf.fw, sf.fh);
    eprintln!("E7NODE	{id}	-	-	-	{}	{}", r(exact), r(conservative));
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
