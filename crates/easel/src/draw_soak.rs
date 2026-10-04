//! A raw canvas in Lua (engine: `paint::soak`): `canvas{raw="cotton duck"}`
//! sets up the bare cloth instead of a ground (in api.rs, with the rest of
//! `canvas{}`), and `soaked(x, y)` says what is in the cloth there.

use crate::api::{S, err};
use crate::time::{self, Verb};
use mlua::{Lua, Result, Table, Value};

/// Raw canvases are engine 3's: an older log's `canvas{}` takes no `raw=`.
pub fn has_raw(engine: u32) -> bool {
    engine >= 3
}

/// The cloth `canvas{raw=}` names (None: a primed canvas), checked against
/// its `ground=`: a raw canvas has none (an empty table is none).
pub fn fabric_of(o: &Table) -> Result<Option<paint::Fabric>> {
    let Some(n) = o.get::<Option<String>>("raw")? else { return Ok(None) };
    let f = paint::Fabric::named(&n).ok_or_else(|| mlua::Error::runtime(format!("canvas: raw= names the cloth: {}", paint::Fabric::names())))?;
    match o.get::<Value>("ground")? {
        Value::Nil => Ok(Some(f)),
        Value::Table(t) if t.is_empty() => Ok(Some(f)),
        _ => err("canvas: a raw canvas has no ground (leave ground= out)"),
    }
}

pub fn install(lua: &Lua, st: S) -> Result<()> {
    // soaked(x, y): what is in the cloth there, in words
    let st1 = st.clone();
    lua.globals().set("soaked", lua.create_function(move |_, (x, y): (f32, f32)| {
        time::verb(&st1, Verb::Query, |s| {
            let c = s.canvas.as_ref().ok_or_else(crate::api::no_canvas)?;
            Ok(c.soaked_at(x, y).unwrap_or_else(|| if c.is_raw() { "outside the canvas".into() } else { "primed (nothing soaks in)".into() }))
        })
    })?)?;
    Ok(())
}
