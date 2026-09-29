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

/// Run a verb `f` on the studio under the hand clock's rules for its kind.
pub fn verb<R>(st: &S, kind: Verb, f: impl FnOnce(&mut Studio) -> mlua::Result<R>) -> mlua::Result<R> {
    if matches!(kind, Verb::Query | Verb::Wait) {
        flush(st, true);
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

/// Hand time on: the paint ages while the hand works.
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

// every test here paints from the default box
#[cfg(all(test, tube_box))]
mod tests {
    use crate::session::Session;
    use paint::tally::{pace, stroke_secs, touch_secs};

    const W: usize = 160;

    fn clock(s: &Session) -> f64 {
        s.st.borrow().clock
    }

    fn run(s: &mut Session, src: &str) -> String {
        s.run(src).unwrap_or_else(|e| panic!("{src}: {e}")).out
    }

    const CANVAS: &str = r#"canvas{size=440, aspect=1.5, linen=15, seed=2, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=120, apply="knife"}}}"#;

    #[test]
    fn the_clock_advances_by_the_hand_time_of_known_verbs() {
        let mut s = Session::new(W).unwrap();
        run(&mut s, CANVAS);
        let mm = s.canvas().unwrap().mm_per_unit() as f64;
        assert!((mm - 0.44).abs() < 1e-6, "440 mm wide: {mm}");
        assert_eq!(clock(&s), 0.0, "the ground is not the painter's time");
        // knifing a pile, a load from it, a stroke 300 units long with a
        // 4-unit round, a touch
        run(&mut s, r#"p = pile{{"bone black", 1}, {"raw umber", 2}}; b = brush("round", 4); b:load(p, 0.9); b:stroke({{100, 500}, {400, 500}}); b:touch(500, 300)"#);
        let want = (pace::REMIX + pace::RELOAD + stroke_secs(300.0 * mm, 4.0 * mm) + touch_secs()) / 60.0;
        assert!((clock(&s) - want).abs() < 1e-4, "clock {} want {want}", clock(&s));
        // the same pile again is a reload, a wipe is a wipe
        let t0 = clock(&s);
        run(&mut s, "b:reload(p, 0.9); b:wipe()");
        let want = (pace::RELOAD + pace::WIPE) / 60.0;
        assert!((clock(&s) - t0 - want).abs() < 1e-4, "{}", clock(&s) - t0);
        // a covering pass takes minutes of hand time
        let t0 = clock(&s);
        run(&mut s, r#"work(rect(100, 100, 300, 200), {hand="body", pile=pile{{"lead white", 4}, {"cobalt blue", 1}}})"#);
        assert!(clock(&s) - t0 > 5.0, "a body passage 300 × 200 units takes minutes: {}", clock(&s) - t0);
    }

    /// The paint ages while the hand works: a passage laid first has begun
    /// to set by the time a long spell of handwork after it is done, with
    /// no wait.
    #[test]
    fn hand_time_is_always_on() {
        let mut s = Session::new(W).unwrap();
        run(&mut s, CANVAS);
        assert!(s.canvas().unwrap().hand_time().is_some());
        run(&mut s, r#"work(rect(100, 100, 300, 200), {hand="body", pile=pile{{"lead white", 1}}, coverage=3})
                        b = brush("rigger", 0.5); b:load(pile{{"raw umber", 1}}, 0.9)
                        for i = 1, 600 do b:stroke({{500, 100 + i}, {900, 100 + i}}) end"#);
        assert!(clock(&s) > 60.0, "600 fine lines take the hand a while: {} min", clock(&s));
        let f = s.canvas().unwrap().frame();
        let st = s.canvas().unwrap().stages()[f.index(250.0, 200.0)];
        assert_ne!(st, paint::Stage::Open, "the first passage has begun to set");
    }

    /// `wait` returns the time of day; days are fine.
    #[test]
    fn wait_returns_the_time_of_day() {
        let mut s = Session::new(W).unwrap();
        run(&mut s, CANVAS);
        let out = run(&mut s, "print(wait(90)); print(wait(3 * 24 * 60))");
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 2, "{out}");
        assert!(lines[0].starts_with("day 1, 10:3"), "{out}");
        assert!(lines[1].starts_with("day 4, 10:3"), "{out}");
        assert_eq!(super::time_of_day(0.0), "day 1, 09:00");
        assert_eq!(super::time_of_day(15.0 * 60.0 + 7.5), "day 2, 00:07");
    }

    /// `wait` takes 0 to 10 years of minutes. Anything else (a huge number
    /// that would overflow the engine's clock, infinity, NaN, a negative) is
    /// refused before any time passes: the canvas and clock stay as they were.
    #[test]
    fn wait_refuses_what_is_not_a_span_of_up_to_ten_years() {
        let mut s = Session::new(W).unwrap();
        run(&mut s, CANVAS);
        run(&mut s, r#"work(rect(100, 100, 300, 200), {hand="body", pile=pile{{"lead white", 1}}, coverage=3})"#);
        let bits = |s: &Session| s.canvas().unwrap().seen().iter().flat_map(|p| p.map(f32::to_bits)).collect::<Vec<_>>();
        let (before, t0) = (bits(&s), clock(&s));
        for m in ["1e100", "1/0", "-1/0", "0/0", "-1", "5259600.5"] {
            let e = s.run(&format!("wait({m})")).unwrap_err();
            assert!(e.contains("5259600") && e.contains("10 years"), "wait({m}): {e}");
            assert_eq!(clock(&s), t0, "wait({m}) moved the clock");
            assert!(bits(&s) == before, "wait({m}) changed the canvas");
        }
        run(&mut s, "wait(5259600)");
        assert_eq!(clock(&s), t0 + 5259600.0);
    }
}
