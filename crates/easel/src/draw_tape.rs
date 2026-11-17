//! Masking tape and a cut stencil on a painting (paint's tape.rs), in a
//! painting with the gestures-only switch (session.rs `GESTURES_ONLY_MARK`).
//!
//! ```lua
//! tape({{500, 0}, {500, 1000}}, {width=24})   -- a strip, mm wide, along the points
//! stencil({{400, 400}, {600, 400}, {500, 600}})   -- film over the canvas, this shape cut out
//! lift_tape()                                  -- every piece lifted, with the paint on it
//! ```

use crate::api::{S, check_keys, err, num, points};
use mlua::{Lua, Result, Table, Value};

/// A strip's width when none is given, mm: the commonest roll (1 in).
const TAPE_WIDTH_MM: f32 = 24.0;
/// The hand pressing a strip down along its line, mm/s (an ESTIMATE: a
/// hand runs a thumb along a strip at a few centimeters a second), plus a
/// couple of seconds to tear and place it.
const TAPE_HAND_MM_S: f32 = 40.0;

/// Refuse `call` while tape lies on the picture: what it would do there is
/// not modeled (graphite and pastel on tape, waiting with paint on it).
pub(crate) fn no_tape(st: &S, call: &str) -> Result<()> {
    if st.borrow().canvas.as_ref().is_some_and(|c| c.has_tape()) {
        return err(format!("{call}: tape lies on the picture; lift it first (lift_tape())"));
    }
    Ok(())
}

pub fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    // tape(pts, {width=mm}): a strip of masking tape pressed down along the points
    {
        let st1 = st.clone();
        g.set("tape", lua.create_function(move |_, (p, o): (Value, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, &["width"], "tape")?;
            }
            let pts = points(&p)?;
            let width_mm = o.as_ref().map(|o| num(o, "width")).transpose()?.flatten().unwrap_or(TAPE_WIDTH_MM);
            crate::time::verb(&st1, crate::time::Verb::Pass, |s| {
                let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
                let mm2 = c.lay_tape(&pts, paint::tape::Cut::Strip { width_mm }).map_err(|e| mlua::Error::runtime(format!("tape: {e}")))?;
                c.tally_mut().secs += 2.0 + (mm2 / width_mm / TAPE_HAND_MM_S) as f64;
                Ok(mm2)
            })
        })?)?;
    }
    // stencil(pts): a sheet of stencil film laid over the picture with the
    // closed shape through the points cut out of it
    {
        let st1 = st.clone();
        g.set("stencil", lua.create_function(move |_, p: Value| {
            let pts = points(&p)?;
            crate::time::verb(&st1, crate::time::Verb::Pass, |s| {
                let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
                let mm2 = c.lay_tape(&pts, paint::tape::Cut::Stencil).map_err(|e| mlua::Error::runtime(format!("stencil: {e}")))?;
                c.tally_mut().secs += 5.0;
                Ok(mm2)
            })
        })?)?;
    }
    // lift_tape(): every piece of tape and stencil lifted with the paint on
    // it; returns the wet paint it took, mm³
    {
        let st1 = st.clone();
        g.set("lift_tape", lua.create_function(move |_, ()| {
            crate::time::verb(&st1, crate::time::Verb::Pass, |s| {
                let c = s.canvas.as_mut().ok_or_else(|| mlua::Error::runtime("no canvas yet: call canvas{} first"))?;
                let mm3 = c.lift_tape().ok_or_else(|| mlua::Error::runtime("lift_tape: no tape lies on the picture"))?;
                c.tally_mut().secs += 3.0;
                Ok(mm3)
            })
        })?)?;
    }
    Ok(())
}
