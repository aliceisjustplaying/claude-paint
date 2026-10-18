//! A hand in time: every mark costs the time a hand takes to make it, and
//! the paint ages while the hand works.
//!
//! The engine counts what the brushes do and prices it in seconds of hand
//! time (`paint::tally`). The easel puts that time on the painting's clock
//! as it is spent, with the drying model (`Canvas::wait`): after every
//! covering verb (`work`, `blend`, `stipple`, `varnish`, pencil lines) and,
//! for strokes and touches made one at a time (`b:stroke`, `b:touch` and
//! the verbs built on them), whenever a minute has piled up, and at the end
//! of every chunk. Hand time is always on. `wait(minutes)` lets time pass
//! with the hand away from the canvas.
//!
//! Every verb that marks the canvas, reads the clock or moves it goes
//! through `verb`, which keeps these rules in one place (`Verb`).

use crate::api::{S, Studio};
use paint::tally::Piles;

/// A long pass (`work`, `stipple`) is painted in slices of this much hand
/// time (minutes), the paint ageing between them: a passage that takes an
/// hour has set at its first strokes by the time the hand reaches the last.
pub const SLICE_MIN: f32 = 15.0;
/// Hand time is put on the clock once this much (minutes) has piled up
/// from strokes made one at a time (a covering verb always puts its own).
pub const GRAIN_MIN: f64 = 1.0;
/// The time of day the painting starts at (minutes after midnight).
pub const START_MIN: f64 = 9.0 * 60.0;

/// The hand's state: part of the studio, snapshotted with it (a failed
/// chunk rolls it back).
#[derive(Clone, Debug, Default)]
pub struct Hand {
    /// The piles on the palette: held brushes load from them and covering
    /// passes dip into them (`Canvas::work_with`); a pile knifed with
    /// `pile{}` is put here, so a dip into it is a reload.
    pub piles: Piles,
}

/// What a verb does with the painting's clock (see `verb`).
pub enum Verb {
    /// Marks made one at a time (`b:stroke`, `b:touch`): their hand time
    /// goes on the clock once a `GRAIN_MIN` has piled up.
    Marks,
    /// A covering verb (`work`, `stipple`, pencil lines, `varnish`): all its
    /// hand time is on the clock when it ends (a long pass clocks its slices
    /// as it goes; the engine's `set_hand_time`).
    Pass,
    /// A verb that reads the canvas's state (`drying`): the hand time owed
    /// goes on the clock first.
    Query,
    /// `wait`: the hand time owed goes on the clock first, then the wait.
    Wait,
}

thread_local! {
    /// Set while a dry verb runs (`verb_dry`): one the sheet is modeled for.
    static DRY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// `verb`, for a tool a paper mask is modeled for (paint's `sheet.rs`): a
/// pastel stick, the finger, a dry brush. Other marking verbs refuse while a
/// sheet lies on the picture.
pub fn verb_dry<R>(st: &S, kind: Verb, f: impl FnOnce(&mut Studio) -> mlua::Result<R>) -> mlua::Result<R> {
    // (put back as it was however the verb ends: a chunk's engine panic is
    // caught and the session goes on)
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            DRY.with(|d| d.set(self.0));
        }
    }
    let _restore = Restore(DRY.with(|d| d.replace(true)));
    verb(st, kind, f)
}

/// Run a verb `f` on the studio under the hand clock's rules for its kind.
pub fn verb<R>(st: &S, kind: Verb, f: impl FnOnce(&mut Studio) -> mlua::Result<R>) -> mlua::Result<R> {
    if matches!(kind, Verb::Query | Verb::Wait) {
        flush(st, true);
    }
    if matches!(kind, Verb::Marks | Verb::Pass) && !DRY.with(|d| d.get()) && st.borrow().canvas.as_ref().is_some_and(|c| c.has_sheet()) {
        return Err(mlua::Error::runtime("a sheet of paper lies on the picture: only pastel, the finger, a dry brush (b:dust), the eraser and fixative are modeled with one down; lift it first (lift_sheet())"));
    }
    let r = {
        let mut g = st.borrow_mut();
        let s = &mut *g;
        let r = f(s)?;
        if let (Verb::Wait, Some(c)) = (&kind, s.canvas.as_ref()) {
            s.clock = c.clock() - s.clock0;
        }
        r
    };
    match kind {
        Verb::Marks => flush(st, false),
        Verb::Pass | Verb::Wait => flush(st, true),
        Verb::Query => {}
    }
    #[cfg(feature = "replay")]
    if let Some(c) = st.borrow().canvas.as_ref() {
        crate::frames::after_verb(c);
    }
    if st.borrow().live {
        crate::session::PROGRESS.verb(&st.borrow());
    }
    Ok(r)
}

/// Put the hand time spent and not yet clocked on the clock once a
/// `GRAIN_MIN` has piled up, or always with `force`. A long pass has
/// already put all but its last slice on the clock as it went (the engine's
/// `set_hand_time`); this puts the rest. (The chunk's end calls it; verbs
/// go through `verb`.)
pub(crate) fn flush(st: &S, force: bool) {
    let mut g = st.borrow_mut();
    let s = &mut *g;
    let Some(c) = s.canvas.as_mut() else { return };
    if !force && c.hand_owed_secs() / 60.0 < GRAIN_MIN {
        return;
    }
    c.clock_hand_min();
    s.clock = c.clock() - s.clock0;
}

/// Hand time on: the paint ages while the hand works. The engine's opt-in
/// `PAINT_COARSE_AGING=1` bounds longer covering slices by material change;
/// the mark grain, verb flushes and chunk-end flush are unchanged.
pub fn start(c: &mut paint::Canvas) {
    c.set_hand_time(Some(SLICE_MIN));
    #[cfg(feature = "replay")]
    crate::frames::begin(c);
}

/// A held brush went to the palette for a pile of this color: a reload if
/// the pile is on the palette, else it is knifed again.
pub fn trip(st: &S, color: paint::Rgb) {
    let mut g = st.borrow_mut();
    let s = &mut *g;
    if let Some(c) = s.canvas.as_mut() {
        s.hand.piles.trip(c.tally_mut(), color);
    }
}

/// A pile knifed on the palette: the mixing time, and the pile there for
/// later dips.
pub fn knife(st: &S, color: paint::Rgb) {
    let mut g = st.borrow_mut();
    let s = &mut *g;
    if let Some(c) = s.canvas.as_mut() {
        s.hand.piles.knife(c.tally_mut(), color);
    }
}

/// The time of day, as a painter at the easel would know it: the day of
/// the painting (1 on the day it was begun) and the hour, `clock` minutes
/// after it was begun at `START_MIN`.
pub fn time_of_day(clock: f64) -> String {
    let t = START_MIN + clock.max(0.0);
    let day = (t / 1440.0).floor() as u64 + 1;
    let m = (t % 1440.0).floor() as u64;
    format!("day {day}, {:02}:{:02}", m / 60, m % 60)
}
