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
//! Every verb that marks the canvas, reads the clock or moves it goes
//! through `verb`, which keeps these rules in one place (`Verb`).
//!
//! A painter works in sittings of a few hours and rests between them:
//! `sitting{hours=3}` starts one, `rest(hours)` steps away while the paint
//! sets. A sitting that runs over its hours is reported, not cut short (see
//! `note_overrun`).

use crate::api::{S, Studio, err, num, span};
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
    /// The piles mixed on the palette this sitting: held brushes load from
    /// them and covering passes dip into them (`Canvas::work_with`).
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
    /// A new sitting starts at `clock`: a clean palette.
    fn begin(&mut self, clock: f64) {
        self.start = clock;
        self.sittings += 1;
        self.warned = 0;
        self.piles = Piles::default();
    }
}

/// What a verb does with the painting's clock (see `verb`).
pub enum Verb {
    /// Marks made one at a time (`b:stroke`, `b:touch`): their hand time
    /// goes on the clock once a `GRAIN_MIN` has piled up.
    Marks,
    /// A covering verb (`work`, `stipple`, pencil lines): all its hand time
    /// is on the clock when it ends (a long pass clocks its slices as it
    /// goes; the engine's `set_hand_time`).
    Pass,
    /// A verb that reads the clock or starts from it (`clock`, `drying`,
    /// `timesheet`, `sitting`, `hand_time`): the hand time owed goes on the
    /// clock first.
    Query,
    /// A verb that makes the clock jump (`wait`, `dry`, `rest`, `glaze`,
    /// `varnish`, `cracks`, `relief`): the hand time owed goes on the clock
    /// first, in the sitting it was spent in. Then the jump is taken into
    /// the clock: reported (`note`: the verb and what dried, for a wait the
    /// painter didn't ask for) and a rest by `rest`. The verb's own hand
    /// time (a glaze's brushing) comes after the jump, in the new sitting.
    Jump { rest: Rest, note: Option<(&'static str, &'static str)> },
}

/// When a jump of the clock is a rest (the next mark starts a new sitting).
pub enum Rest {
    /// A wait of this many minutes was asked for: a rest if `REST_MIN` or
    /// more.
    Waited(f64),
    /// A jump of `REST_MIN` or more.
    IfLong,
    /// Always, unless nothing was painted this sitting yet (`dry`: waiting
    /// for everything to dry is a rest, however short).
    IfPainted,
    /// Always (`rest`).
    Always,
}

/// Run a verb `f` on the studio under the hand clock's rules for its kind
/// (`Verb`). With hand time off no hand time is put on the clock, but jumps
/// are still reported and rests still start sittings.
pub fn verb<R>(st: &S, kind: Verb, f: impl FnOnce(&mut Studio) -> Result<R>) -> Result<R> {
    if matches!(kind, Verb::Query | Verb::Jump { .. }) {
        flush(st, true);
    }
    let r = {
        let mut g = st.borrow_mut();
        let s = &mut *g;
        let r = f(s)?;
        if let Verb::Jump { rest, note } = &kind
            && let Some(c) = s.canvas.as_ref()
        {
            let now = c.clock() - s.clock0;
            let jumped = now - s.clock;
            s.clock = now;
            if let Some((verb, what)) = note
                && jumped > 0.5
            {
                let note = format!("{verb}: waited {} for {what} to dry (clock {now:.0} min)\n", span(jumped));
                s.out.push_str(&note);
            }
            let rests = match rest {
                Rest::Waited(m) => *m >= REST_MIN,
                Rest::IfLong => jumped >= REST_MIN,
                Rest::IfPainted => now - s.hand.start > 1e-9,
                Rest::Always => true,
            };
            if rests {
                s.hand.begin(now);
            }
        }
        r
    };
    match kind {
        Verb::Marks => flush(st, false),
        Verb::Pass | Verb::Jump { .. } => flush(st, true),
        Verb::Query => {}
    }
    Ok(r)
}

/// Put the hand time spent and not yet clocked on the clock (hand time on)
/// once a `GRAIN_MIN` has piled up, or always with `force`. A long pass has
/// already put all but its last slice on the clock as it went (the engine's
/// `set_hand_time`); this puts the rest. With hand time off, nothing: the
/// ledger only counts. (The chunk's end calls it; verbs go through `verb`.)
pub(crate) fn flush(st: &S, force: bool) {
    let mut g = st.borrow_mut();
    let s = &mut *g;
    let Some(c) = s.canvas.as_mut() else { return };
    if !force && c.hand_owed_secs() / 60.0 < GRAIN_MIN {
        return;
    }
    c.clock_hand_min();
    // (every other move of the clock is a `Verb::Jump`, which takes it into
    // `s.clock`: what moved since is hand time)
    let now = c.clock() - s.clock0;
    let hand = now - s.clock;
    s.clock = now;
    if hand > 0.0 {
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
fn note_overrun(s: &mut Studio) {
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
            verb(&st1, Verb::Query, |s| {
                let Some(c) = s.canvas.as_mut() else {
                    return err("no canvas yet: canvas{..., hand=true} or hand_time(true) after it");
                };
                let was = c.hand_time().is_some();
                set(c, on.unwrap_or(true));
                Ok(was)
            })
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
            verb(&st1, Verb::Query, |s| {
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
            })
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
            verb(&st1, Verb::Jump { rest: Rest::Always, note: None }, |s| {
                s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet"))?.wait((h * 60.0) as f32);
                Ok(())
            })?;
            Ok(st1.borrow().clock)
        })?)?;
    }
    // timesheet(): {clock=, sitting=, sittings=, hours=, hand=, open=, setting=, tacky=, dry=, strokes=, touches=, reloads=, piles=, hand_min=}
    {
        let st1 = st.clone();
        g.set("timesheet", lua.create_function(move |lua, ()| {
            verb(&st1, Verb::Query, |s| {
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
            })
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

    /// The session's checkpoints carry the hand state: an edit (replaying
    /// from the nearest checkpoint) and an undo past the undo ring leave the
    /// clock, the sitting and the timesheet exactly as a fresh replay of the
    /// same log (review r6, finding 1).
    #[test]
    fn edits_and_undo_keep_the_hand_state_of_a_replay() {
        const LOG: [&str; 6] = [
            r##"canvas{style="friedrich", aspect=1.5, seed=2, hand=true}; sitting{hours=2}"##,
            r##"work(rect(100, 100, 300, 200), {hand="body", color="#8090a0"})"##,
            r##"b = brush("round", 3); b:load("#303830", 0.9); for i = 1, 30 do b:stroke({{100 + 20*i, 500}, {110 + 20*i, 440}}) end"##,
            "rest(3)",
            r##"stipple(rect(0, 380, 1000, 120), {width=3, color="#cfccc2", coverage=1.2})"##,
            r##"b:load("#6a5040", 0.9); b:stroke({{200, 300}, {600, 320}})"##,
        ];
        let sheet = |s: &mut Session| -> String {
            run(s, "local t = timesheet(); sheet = string.format('%.9f %d %.9f %.9f %d %d %.3f %.9f', t.clock, t.sittings, t.sitting, t.hours, t.strokes, t.touches, t.reloads, t.hand_min)");
            s.lua.globals().get::<String>("sheet").unwrap()
        };
        let bits = |s: &Session| s.canvas().unwrap().seen().iter().flat_map(|p| p.map(f32::to_bits)).collect::<Vec<_>>();
        let fresh = |log: &[&str]| {
            let mut r = Session::replay(W).unwrap();
            for c in log {
                run(&mut r, c);
            }
            r
        };
        let mut s = Session::new(W, 1).unwrap();
        s.keep = 3;
        for c in LOG {
            run(&mut s, c);
        }
        // edit chunk 3 (a checkpoint before it, chunks after it replayed)
        let new3 = r##"b = brush("round", 4); b:load("#303830", 0.9); for i = 1, 20 do b:stroke({{100 + 30*i, 520}, {110 + 30*i, 430}}) end"##;
        s.splice(3, 1, &[new3.to_string()]).unwrap();
        let mut want: Vec<&str> = LOG.to_vec();
        want[2] = new3;
        let mut r = fresh(&want);
        assert_eq!(clock(&s), clock(&r));
        assert_eq!(bits(&s), bits(&r));
        assert_eq!(sheet(&mut s), sheet(&mut r));
        // undo past the undo ring (replays from a checkpoint)
        s.undo(4).unwrap();
        let mut r = fresh(&want[..3]);
        assert_eq!(clock(&s), clock(&r));
        assert_eq!(sheet(&mut s), sheet(&mut r));
    }

    /// The time a finishing verb spent drying the paint is taken into the
    /// clock once: repeated queries in the same chunk see the same clock and
    /// the same sitting, never a new sitting per query or a negative one,
    /// and the drying is still reported once (review r6, finding 2).
    #[test]
    fn queries_after_a_finish_consume_its_drying_once() {
        for on in [false, true] {
            let mut s = Session::new(W, 0).unwrap();
            run(&mut s, &format!(r##"canvas{{style="friedrich", aspect=1.5, seed=2, hand={on}}}"##));
            run(&mut s, r##"work(rect(100, 100, 300, 200), {hand="body", color="#8090a0", coverage=4})"##);
            let out = run(
                &mut s,
                r##"varnish()
                    local a = timesheet(); local b = timesheet(); local c1, c2 = clock(), clock(); drying(200, 200); local c = timesheet()
                    assert(a.sittings == 2 and b.sittings == 2 and c.sittings == 2, a.sittings .. " " .. b.sittings .. " " .. c.sittings)
                    assert(a.sitting >= 0 and b.sitting == a.sitting and c.sitting == a.sitting, a.sitting .. " " .. b.sitting)
                    assert(c1 == c2 and c1 == a.clock and a.clock > 120, c1 .. " " .. c2 .. " " .. a.clock)"##,
            );
            assert_eq!(out.matches("varnish: waited").count(), 1, "hand {on}: {out}");
            let now = s.canvas().unwrap().clock();
            let c0 = s.st.borrow().clock0;
            assert_eq!(clock(&s), now - c0);
            // and the next chunk reports nothing more
            let out = run(&mut s, "assert(timesheet().sittings == 2)");
            assert!(!out.contains("waited"), "{out}");
        }
    }

    /// Time a finishing verb spends drying the paint isn't hand time: it is
    /// reported, starts a new sitting and never counts as an overrun (the
    /// new sitting holds only the varnish's brushing, with hand time on).
    #[test]
    fn drying_for_a_finish_is_not_time_at_the_easel() {
        for on in [false, true] {
            let mut s = Session::new(W, 0).unwrap();
            run(&mut s, &format!(r##"canvas{{style="friedrich", aspect=1.5, seed=2, hand={on}}}"##));
            run(&mut s, r##"work(rect(100, 100, 300, 200), {hand="body", color="#8090a0"})"##);
            let out = run(&mut s, "varnish()");
            assert!(out.contains("varnish: waited") && !out.contains("at the easel"), "hand {on}: {out}");
            run(&mut s, &format!("local t = timesheet(); assert(t.sittings == 2 and t.sitting {}, t.sitting)", if on { "> 0 and t.sitting < 2" } else { "== 0" }));
        }
    }

    /// A varnish is brushed over the whole canvas like a glaze: after the
    /// hand time owed before it goes on the clock (in the sitting it was
    /// spent in), the paint dries (a rest), and its brushing is hand time
    /// in the new sitting (thermos B4).
    #[test]
    fn varnish_is_hand_time_after_the_rest() {
        let mut s = Session::new(W, 0).unwrap();
        run(&mut s, r##"canvas{style="friedrich", aspect=1.5, seed=2, hand=true}"##);
        run(&mut s, r##"work(rect(100, 100, 300, 200), {hand="body", color="#8090a0"})"##);
        run(&mut s, r##"b = brush("round", 4); b:load("#303830", 0.9); b:stroke({{100, 500}, {400, 500}})
                        varnish(); local t = timesheet(); sitting, sittings = t.sitting, t.sittings"##);
        let c = s.canvas().unwrap();
        let f = c.frame();
        let brushing = (f.width() * f.height()) as f64 * (c.mm_per_unit() as f64).powi(2) / pace::GLAZE_MM2_S / 60.0;
        let sitting: f64 = s.lua.globals().get("sitting").unwrap();
        let sittings: u32 = s.lua.globals().get("sittings").unwrap();
        assert_eq!(sittings, 2);
        assert!((sitting - brushing).abs() < 1e-6, "the new sitting is the varnish's brushing: {sitting} vs {brushing} min");
    }

    /// One palette a sitting: passes and stipples dip into the piles the
    /// held brushes and earlier passes mixed (a reload, not a new mix), and
    /// a new sitting starts with a clean palette (thermos B5).
    #[test]
    fn a_sitting_mixes_on_one_palette() {
        let mut s = Session::new(W, 0).unwrap();
        run(&mut s, r##"canvas{style="friedrich", aspect=1.5, seed=2, hand=true}"##);
        run(
            &mut s,
            r##"b = brush("round", 4); b:load("#8090a0", 0.9)
                work(rect(100, 100, 300, 200), {hand="body", color="#8090a0", cut_in="round 2"})
                stipple(rect(100, 400, 300, 100), {width=3, color="#8090a0", coverage=1})
                work(rect(500, 100, 300, 200), {hand="body", color="#8090a0"})
                local t = timesheet(); assert(t.piles == 1, t.piles .. " piles")"##,
        );
        run(&mut s, r##"rest(3); b:load("#8090a0", 0.9); local t = timesheet(); assert(t.piles == 2, t.piles .. " piles")"##);
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
