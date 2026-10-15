//! A bounded working window for engine 7's exchange (lane STIL-68).
//!
//! Contract (frozen, CONTRACTS-e7.md): a window owns a mutable `interior`
//! and reads, never writes, a `read_halo` around it; `generation` counts the
//! ordered contact snapshots it has taken. Its scratch is reusable and
//! bounded by `CAPACITY` pixels; a contact larger than that runs the
//! unchanged engine-5 kernel before any mutation. The kernel keeps one
//! window per thread (`with`).

use super::Rect;
use std::cell::RefCell;

/// The most pixels a window's scratch holds.
pub const CAPACITY: usize = 1024;

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
}

thread_local! {
    static WINDOW: RefCell<Window> = RefCell::new(Window { weights: Vec::with_capacity(CAPACITY), ..Window::default() });
}

/// Run `f` with this thread's window.
pub fn with<R>(f: impl FnOnce(&mut Window) -> R) -> R {
    WINDOW.with(|w| f(&mut w.borrow_mut()))
}
