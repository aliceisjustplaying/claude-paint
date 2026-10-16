//! A bounded working window for engine 7's exchange (lane STIL-68).
//!
//! Contract (frozen, CONTRACTS-e7.md): a window owns a mutable `interior`
//! and reads, never writes, a `read_halo` around it; `generation` counts the
//! ordered contact snapshots it has taken. Its scratch is reusable and
//! bounded by `CAPACITY` pixels; a contact larger than that runs the
//! unchanged engine-5 kernel before any mutation. The kernel keeps one
//! window per thread (`with`).
//!
//! For one contact the window holds the capsule's contact list (the pixels
//! a hair touches, in raster order, with their weights in `weights`) and
//! the paint its plough moves, gathered per destination pixel of the
//! interior. Nothing is cleared between contacts: a cell belongs to the
//! contact whose `generation` it carries.

use super::Rect;
use std::cell::RefCell;

/// The most pixels a window's scratch holds.
pub const CAPACITY: usize = 1024;

/// Layout of a packed paint sum (`Acc::a`, and the pickup's): volume,
/// then volume times each of the latent color, the properties and the
/// cure, then the cover and the solvent (µm) the paint brings.
pub(crate) const PACK: usize = 16;
pub(crate) const VOL: usize = 0;
pub(crate) const LAT0: usize = 1;
pub(crate) const HIDE0: usize = 8;
pub(crate) const CURE: usize = 13;
pub(crate) const COVER: usize = 14;
pub(crate) const SOLV: usize = 15;

/// A pixel the capsule touches: its buffer index, its whole-canvas position
/// and how fluid its paint is (its weight is the same entry of `weights`).
#[derive(Clone, Copy)]
pub(crate) struct Hit {
    pub i: usize,
    pub x: u32,
    pub y: u32,
    pub fl: f32,
}

/// Paint ploughed into one interior pixel during one contact: its buffer
/// index, the clip's acceptance there (read once) and the packed sum.
#[derive(Clone, Copy)]
pub(crate) struct Acc {
    pub j: usize,
    pub k: f32,
    pub a: [f32; PACK],
}

#[derive(Default)]
pub struct Window {
    /// Pixels this contact may change (global, end-exclusive).
    pub interior: Rect,
    /// Pixels it may read: the interior and its halo (global, end-exclusive).
    pub read_halo: Rect,
    /// Ordered contact snapshots taken so far.
    pub generation: u64,
    /// Per-pixel contact weights, reused across contacts (≤ `CAPACITY`).
    pub weights: Vec<f32>,
    /// The touched pixels, in raster order, parallel to `weights`.
    pub(crate) hits: Vec<Hit>,
    /// The interior's plough cells, row-major, and the generation each was
    /// last started in.
    cells: Vec<Acc>,
    stamp: Vec<u64>,
    /// Cells started this contact, in the order they were first hit.
    used: Vec<u32>,
}

impl Window {
    /// Start a contact that reads `read_halo` and writes `interior`.
    pub(crate) fn begin(&mut self, read_halo: Rect, interior: Rect) {
        self.read_halo = read_halo;
        self.interior = interior;
        self.generation += 1;
        self.hits.clear();
        self.weights.clear();
        self.used.clear();
        let n = interior.2.saturating_sub(interior.0) * interior.3.saturating_sub(interior.1);
        if self.cells.len() < n {
            self.cells.resize(n, Acc { j: 0, k: 0.0, a: [0.0; PACK] });
            self.stamp.resize(n, 0);
        }
    }

    /// The plough cell of interior pixel (`x`, `y`). Met for the first
    /// time this contact, it starts empty at buffer index `j`, with the
    /// acceptance `k()` gives.
    #[inline]
    pub(crate) fn cell(&mut self, x: usize, y: usize, j: usize, k: impl FnOnce() -> f32) -> &mut Acc {
        let r = self.interior;
        debug_assert!(x >= r.0 && y >= r.1 && x < r.2 && y < r.3, "plough cell ({x}, {y}) outside the interior {r:?}");
        let c = (y - r.1) * (r.2 - r.0) + (x - r.0);
        if self.stamp[c] != self.generation {
            self.stamp[c] = self.generation;
            self.cells[c] = Acc { j, k: k(), a: [0.0; PACK] };
            self.used.push(c as u32);
        }
        &mut self.cells[c]
    }

    /// The cells hit this contact, in first-hit order.
    pub(crate) fn hit_cells(&self) -> impl Iterator<Item = &Acc> + '_ {
        self.used.iter().map(|&c| &self.cells[c as usize])
    }
}

thread_local! {
    static WINDOW: RefCell<Window> = RefCell::new(Window { weights: Vec::with_capacity(CAPACITY), hits: Vec::with_capacity(CAPACITY), ..Window::default() });
}

/// Run `f` with this thread's window.
pub fn with<R>(f: impl FnOnce(&mut Window) -> R) -> R {
    WINDOW.with(|w| f(&mut w.borrow_mut()))
}
