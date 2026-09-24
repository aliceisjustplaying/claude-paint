//! A hand in time: marks cost the time a hand takes to make them, and the
//! paint ages while the hand works (notes/time.md).
//!
//! The engine counts what the brushes do and prices it in seconds of hand
//! time (`paint::tally`). With hand time on, the easel puts that time on the
//! painting's clock as it is spent, with the drying model (`Canvas::wait`):
//! after every covering verb (`work`, `blend`, `stipple`, `glaze`, pencil
//! lines) and, for strokes and touches made one at a time (`b:stroke`,
//! `b:touch` and the motif verbs built on them), whenever a minute has
//! piled up, and at the end of every chunk. It is off unless a painting
//! turns it on (`canvas{hand=true}` or `hand_time(true)`), so older logs
//! replay exactly as before.
//!
//! A painter works in sittings of a few hours and rests between them:
//! `sitting{hours=3}` starts one, `rest(hours)` steps away while the paint
//! sets. A sitting that runs over its hours is reported, not cut short (see
//! `note_overrun`).

use crate::api::{S, err, num, span};
use mlua::{Lua, Result, Table, Value};
use paint::tally::Piles;

/// A long pass (`work`, `stipple`) is painted in slices of this much hand
/// time (minutes), the paint ageing between them: a sky that takes an hour
/// has set at its first passages by the time the hand reaches the last [E].
pub const SLICE_MIN: f32 = 15.0;
/// Hand time is put on the clock once this much (minutes) has piled up
/// from strokes made one at a time (a covering verb always puts its own).
pub const GRAIN_MIN: f64 = 1.0;
/// A sitting's length unless the painting says otherwise (hours): "a few
/// hours, a couple of times a day" [E].
pub const SITTING_HOURS: f64 = 3.0;
/// A wait this long (minutes) or longer is a rest: the painter has left the
/// easel, and the next mark starts a new sitting.
pub const REST_MIN: f64 = 120.0;
/// `rest()` without hours: overnight [E].
pub const OVERNIGHT_H: f64 = 16.0;

/// The hand's clock: part of the studio, snapshotted with it (undo).
#[derive(Clone, Debug)]
pub struct Hand {
    /// The painting clock (minutes since `canvas{}`) when this sitting began.
    pub start: f64,
    /// This sitting's planned length, hours.
    pub hours: f64,
    /// Sittings so far (1 during the first).
    pub sittings: u32,
    /// The overrun of this sitting was reported (whole hours over).
    pub warned: u32,
    /// The piles mixed on the palette this sitting (held brushes' loads).
    pub piles: Piles,
    /// The ledger when `canvas{}` was set up (the grounds are the
    /// colorman's work, not the painter's).
    pub base: paint::Tally,
}

impl Default for Hand {
    fn default() -> Self {
        Hand { start: 0.0, hours: SITTING_HOURS, sittings: 1, warned: 0, piles: Piles::default(), base: paint::Tally::default() }
    }
}

impl Hand {
    /// After everything dried (`dry()`): a rest, unless nothing was painted
    /// in this sitting yet.
    pub fn begin_rested(&mut self, clock: f64) {
        if clock - self.start > 1e-9 {
            self.begin(clock);
        }
    }

    /// A new sitting starts at `clock`: a clean palette.
    fn begin(&mut self, clock: f64) {
        self.start = clock;
        self.sittings += 1;
        self.warned = 0;
        self.piles = Piles::default();
    }
}

/// Put the hand time spent and not yet clocked on the clock (hand time on)
/// once a `GRAIN_MIN` has piled up, or always with `force`. A long pass has
/// already put all but its last slice on the clock as it went (the engine's
/// `set_hand_time`); this puts the rest. With hand time off, nothing: the
/// ledger only counts.
pub fn flush(st: &S, force: bool) {
    let mut g = st.borrow_mut();
    let s = &mut *g;
    let Some(c) = s.canvas.as_mut() else { return };
    let owed = c.hand_owed() / 60.0;
    if !force && owed < GRAIN_MIN {
        return;
    }
    c.clock_hand();
    let now = c.clock() - s.clock0;
    if now != s.clock {
        s.clock = now;
        note_overrun(s);
    }
}

/// Hand time on or off.
pub fn set(c: &mut paint::Canvas, on: bool) {
    c.set_hand_time(on.then_some(SLICE_MIN));
}

/// A sitting that runs over its hours is reported once per hour over, not
/// ended: an automatic rest would fall wherever the hand happened to be
/// (halfway through a sky), when a painter finishes the passage while it is
/// wet and then stops. The painter decides where the break goes (`rest`).
fn note_overrun(s: &mut crate::api::Studio) {
    let over = s.clock - s.hand.start - s.hand.hours * 60.0;
    if over <= 0.0 {
        return;
    }
    let h = (over / 60.0).floor() as u32 + 1;
    if h > s.hand.warned {
        s.hand.warned = h;
        let note = format!(
            "sitting {}: {} at the easel, {} planned; finish the passage while it is open, then rest(hours)\n",
            s.hand.sittings,
            span(s.clock - s.hand.start),
            span(s.hand.hours * 60.0)
        );
        s.out.push_str(&note);
    }
}

/// After the clock jumped by `minutes` (a `wait` or `dry`): a long one is a
/// rest, and the next mark starts a new sitting.
pub fn waited(st: &S, minutes: f64) {
    if minutes >= REST_MIN {
        let mut s = st.borrow_mut();
        let now = s.clock;
        s.hand.begin(now);
    }
}

/// A held brush went to the palette for `color`: a reload from a pile
/// already mixed this sitting, or a new pile.
pub fn trip(st: &S, color: paint::Rgb) {
    let mut g = st.borrow_mut();
    let s = &mut *g;
    if let Some(c) = s.canvas.as_mut() {
        s.hand.piles.trip(c.tally_mut(), color);
    }
}

/// The time line for `status` and the chunk reply: the clock, the sitting,
/// hand time and how much of the canvas is open, setting, tacky and dry.
pub fn summary(s: &crate::api::Studio) -> String {
    let Some(c) = s.canvas.as_ref() else { return "no canvas yet".into() };
    let sh = c.stage_shares();
    let pc = |x: f64| if x > 0.0 && x < 0.005 { "<1%".to_string() } else { format!("{:.0}%", 100.0 * x) };
    format!(
        "sitting {}: {} of {} · hand time {} · open {} setting {} tacky {} dry {}",
        s.hand.sittings,
        span(s.clock - s.hand.start),
        span(s.hand.hours * 60.0),
        if c.hand_time().is_some() { "on" } else { "off" },
        pc(sh[0]),
        pc(sh[1]),
        pc(sh[2]),
        pc(sh[3])
    )
}

pub fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    // hand_time(on): hand time on the clock or not; returns what it was
    {
        let st1 = st.clone();
        g.set("hand_time", lua.create_function(move |_, on: Option<bool>| {
            flush(&st1, true);
            let mut s = st1.borrow_mut();
            let Some(c) = s.canvas.as_mut() else {
                return err("no canvas yet: canvas{..., hand=true} or hand_time(true) after it");
            };
            let was = c.hand_time().is_some();
            set(c, on.unwrap_or(true));
            Ok(was)
        })?)?;
    }
    // sitting{hours=3} or sitting(3): a new sitting starts now (a clean
    // palette); returns the clock
    {
        let st1 = st.clone();
        g.set("sitting", lua.create_function(move |_, a: Value| {
            let hours = match &a {
                Value::Nil => None,
                Value::Table(t) => {
                    crate::api::check_keys(t, &["hours"], "sitting")?;
                    num(t, "hours")?.map(|h| h as f64)
                }
                Value::Integer(n) => Some(*n as f64),
                Value::Number(n) => Some(*n),
                o => return err(format!("sitting: want {{hours=3}} or a number of hours, got {}", o.type_name())),
            };
            if let Some(h) = hours
                && !(h > 0.0 && h <= 24.0)
            {
                return err("sitting: hours between 0 and 24");
            }
            flush(&st1, true);
            let mut s = st1.borrow_mut();
            if s.canvas.is_none() {
                return err("no canvas yet");
            }
            let now = s.clock;
            // the first sitting is under way from canvas{}: starting it
            // again before anything was painted just sets its hours
            if !(s.hand.sittings == 1 && now - s.hand.start < 1e-9) {
                s.hand.begin(now);
            }
            if let Some(h) = hours {
                s.hand.hours = h;
            }
            Ok(now)
        })?)?;
    }
    // rest(hours): step away from the easel; the paint sets meanwhile and
    // the next mark starts a new sitting. Returns the clock.
    {
        let st1 = st.clone();
        g.set("rest", lua.create_function(move |_, hours: Option<f64>| {
            let h = hours.unwrap_or(OVERNIGHT_H);
            if h.is_nan() || h < 0.0 {
                return err("rest(hours): want >= 0 (default: overnight, 16)");
            }
            flush(&st1, true);
            let mut g = st1.borrow_mut();
            let s = &mut *g;
            let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet"))?;
            c.wait((h * 60.0) as f32);
            s.clock = c.clock() - s.clock0;
            let now = s.clock;
            s.hand.begin(now);
            Ok(now)
        })?)?;
    }
    // timesheet(): {clock=, sitting=, sittings=, hours=, hand=, open=, setting=, tacky=, dry=, strokes=, touches=, reloads=, piles=, hand_min=}
    {
        let st1 = st.clone();
        g.set("timesheet", lua.create_function(move |lua, ()| {
            flush(&st1, true);
            let s = st1.borrow();
            let c = s.canvas.as_ref().ok_or_else(|| mlua::Error::runtime("no canvas yet"))?;
            let t: Table = lua.create_table()?;
            let sh = c.stage_shares();
            let k = c.tally().since(&s.hand.base);
            t.set("clock", s.clock)?;
            t.set("sitting", s.clock - s.hand.start)?;
            t.set("sittings", s.hand.sittings)?;
            t.set("hours", s.hand.hours)?;
            t.set("hand", c.hand_time().is_some())?;
            for (i, n) in ["open", "setting", "tacky", "dry"].iter().enumerate() {
                t.set(*n, sh[i])?;
            }
            t.set("strokes", k.strokes)?;
            t.set("touches", k.touches)?;
            t.set("reloads", k.reloads)?;
            t.set("piles", k.remixes)?;
            t.set("hand_min", k.minutes())?;
            Ok(t)
        })?)?;
    }
    Ok(())
}

#[cfg(test)]
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

    #[test]
    fn the_clock_advances_by_the_hand_time_of_known_verbs() {
        let mut s = Session::new(W, 2).unwrap();
        run(&mut s, r##"canvas{style="friedrich", aspect=1.5, seed=2, hand=true}"##);
        let mm = s.canvas().unwrap().mm_per_unit() as f64;
        assert!((mm - 0.44).abs() < 1e-6, "friedrich is 440 mm wide: {mm}");
        assert_eq!(clock(&s), 0.0, "the grounds are not the painter's time");
        // a new pile, a stroke 300 units long with a 4-unit round, a touch
        run(&mut s, r##"b = brush("round", 4); b:load("#303830", 0.9); b:stroke({{100, 500}, {400, 500}}); b:touch(500, 300)"##);
        let want = (pace::REMIX + pace::RELOAD + stroke_secs(300.0 * mm, 4.0 * mm) + touch_secs(4.0 * mm)) / 60.0;
        assert!((clock(&s) - want).abs() < 1e-4, "clock {} want {want}", clock(&s));
        // the same color again is a reload, a wipe is a wipe
        let t0 = clock(&s);
        run(&mut s, r##"b:reload("#303830", 0.9); b:wipe(); c1 = clock()"##);
        let want = (pace::RELOAD + pace::WIPE) / 60.0;
        assert!((clock(&s) - t0 - want).abs() < 1e-4, "{}", clock(&s) - t0);
        // a covering pass: all its hand time is on the clock when it ends
        let t0 = clock(&s);
        run(&mut s, r##"work(rect(100, 100, 300, 200), {hand="body", color="#8090a0"})
                        local t = timesheet(); dt = t.hand_min"##);
        let sheet: f64 = s.lua.globals().get("dt").unwrap();
        assert!((clock(&s) - sheet).abs() < 1e-3, "clock {} vs the timesheet's hand time {sheet}", clock(&s));
        assert!(clock(&s) - t0 > 5.0, "a body passage 300 × 200 units takes minutes: {}", clock(&s) - t0);
    }

    #[test]
    fn hand_time_off_takes_no_time_and_changes_nothing() {
        let paint = r##"b = brush("round", 4); b:load("#303830", 0.9); b:stroke({{100, 500}, {400, 500}})
                        work(rect(100, 100, 300, 200), {hand="body", color="#8090a0"})"##;
        let mut a = Session::new(W, 0).unwrap();
        run(&mut a, r##"canvas{style="friedrich", aspect=1.5, seed=2}"##);
        run(&mut a, paint);
        assert_eq!(clock(&a), 0.0);
        let mut b = Session::new(W, 0).unwrap();
        // (in the same chunk: a chunk's number seeds its randomness)
        run(&mut b, r##"canvas{style="friedrich", aspect=1.5, seed=2, hand=false}; hand_time(false)"##);
        run(&mut b, paint);
        let bits = |s: &Session| s.canvas().unwrap().seen().iter().flat_map(|p| p.map(f32::to_bits)).collect::<Vec<_>>();
        assert_eq!(bits(&a), bits(&b));
        // turning it on later doesn't bill the past
        run(&mut b, "hand_time(true)");
        assert_eq!(clock(&b), 0.0);
    }

    #[test]
    fn sittings_rest_and_report_their_overrun() {
        let mut s = Session::new(W, 2).unwrap();
        run(&mut s, r##"canvas{style="friedrich", aspect=1.5, seed=2, hand=true}; sitting{hours=0.05}"##);
        let out = run(&mut s, r##"work(rect(100, 100, 400, 300), {hand="body", color="#8090a0"})"##);
        assert!(out.contains("sitting 1:") && out.contains("then rest(hours)"), "a 3-minute sitting ran over: {out}");
        let st = s.status();
        assert!(st.contains("sitting 1:") && st.contains("hand time on") && st.contains("open "), "{st}");
        let before = clock(&s);
        run(&mut s, "rest(2); t = timesheet(); assert(t.sittings == 2 and t.sitting == 0, t.sitting)");
        assert!((clock(&s) - before - 120.0).abs() < 1e-3);
        // undo takes the sitting back too
        s.undo(1).unwrap();
        run(&mut s, "t = timesheet(); assert(t.sittings == 1, t.sittings)");
        // a long wait is a rest; a short one isn't
        run(&mut s, "wait(30); assert(timesheet().sittings == 1); wait(180); assert(timesheet().sittings == 2)");
    }
}
