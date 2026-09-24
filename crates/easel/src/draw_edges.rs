//! Losing an edge: `lose(region, {...})` drags the neighbor's paint back
//! across the region's edge with a nearly dry brush, the way a painter
//! loses a contour after the passage is laid (the sky mixture pulled down
//! over a ridge, the water carried across a reflection's edge).
//!
//! Along the region's edge, where `where` says (0 keep .. 1 lose), short
//! strokes start out in the neighbor, cross the edge and lift off inside.
//! Each is loaded lightly with what lies on the canvas out in the neighbor,
//! where the stroke starts, dirtied with a little of the region's own
//! color (`mix`, 0.35: a brush that has been across the edge), mixed from
//! the palette in a lean medium, so it lays the neighbor's color thinner as
//! it runs out: over dry paint a scumble that breaks on the weave; into wet
//! paint it also picks up and drags what it crosses.
//!
//! ```lua
//! lose(rangeM, {where={lost=0.4, soft=0.6, period=60}, tool="filbert 4"})
//! lose(reflM, {where=function(x, y) return y > HZ + 20 and 1 or 0 end, angle=0})  -- level water strokes
//! ```

use crate::api::{S, check_keys, err, frame, num, pair, seed_of, support, tool_of};
use crate::time::{self, Verb};
use mlua::{Lua, Result, Table, Value};
use paint::rng::Rng;
use paint::{Gesture, Held};

const KEYS: &[&str] = &["where", "tool", "reach", "load", "pressure", "angle", "every", "medium", "pal", "seed", "mix"];

fn lose(lua: &Lua, st: &S, m: Value, o: Option<Table>) -> Result<usize> {
    let region = crate::api::mask_of(&m)?;
    let o = match o {
        Some(t) => t,
        None => lua.create_table()?,
    };
    check_keys(&o, KEYS, "lose")?;
    let f = frame(st)?;
    let tool = match o.get::<Option<Value>>("tool")? {
        Some(t) => tool_of(&t)?,
        None => paint::Tool::filbert(4.0),
    };
    tool.validate().map_err(mlua::Error::runtime)?;
    let w = tool.width;
    let seed = seed_of(st, &o)?;
    let b = support(Some(&region), f, 4.0 * w + 20.0);
    let q = match o.get::<Value>("where")? {
        Value::Nil => paint::mask::Mask::from_fn(f, |_, _| 1.0),
        v => crate::api::edge_quality(st, &v, &region, b, seed ^ 0x105E)?,
    };
    let (out_r, in_r) = pair(&o, "reach")?.unwrap_or((0.8 * w, 1.2 * w));
    if !(out_r >= 0.0 && in_r > 0.0) {
        return err("lose: reach={out, in} in units, out ≥ 0, in > 0");
    }
    let load = num(&o, "load")?.unwrap_or(0.2).clamp(0.01, 1.0);
    let (p0, p1) = pair(&o, "pressure")?.unwrap_or((0.35, 0.02));
    let every = o.get::<Option<f32>>("every")?.unwrap_or(1.2).max(0.2);
    let fixed_angle = num(&o, "angle")?;
    let pal = crate::api::palette_of(st, o.get("pal")?)?;
    let medium = num(&o, "medium")?.unwrap_or(0.45);
    // a dirty brush: the neighbor's mixture with some of the region's own
    let dirt = num(&o, "mix")?.unwrap_or(0.35).clamp(0.0, 1.0);

    // the edge, point by point (inward normals), a stroke every `every` brush widths
    let lines = paint::edge::contours(&region, (every * w).max(0.5));
    let mut rng = Rng::new(seed ^ 0x105E_ED6E);
    let mut plans: Vec<(Vec<(f32, f32)>, (f32, f32), (f32, f32))> = Vec::new();
    for line in &lines {
        for &((x, y), (nx, ny)) in line {
            let qv = q.sample(x, y).clamp(0.0, 1.0);
            let u = rng.range(0.0, 1.0);
            if u >= qv {
                continue;
            }
            // a slanting drag: mostly along the edge, crossing it at a shallow
            // angle (a fixed angle, pointed inward, if asked), each its own length
            let (tx, ty) = (-ny, nx);
            let side = if rng.range(0.0, 1.0) < 0.5 { 1.0 } else { -1.0 };
            let (dx, dy) = match fixed_angle {
                Some(a) => {
                    let (c, s) = (a.cos(), a.sin());
                    if c * nx + s * ny >= 0.0 { (c, s) } else { (-c, -s) }
                }
                None => {
                    let th = (0.45 + 0.15 * rng.normal()).clamp(0.2, 0.8);
                    let (c, s) = (th.cos(), th.sin());
                    (tx * side * c + nx * s, ty * side * c + ny * s)
                }
            };
            // how far across: out into the neighbor, in over the region
            let across = (dx * nx + dy * ny).max(0.15);
            let a = out_r * rng.range(0.5, 1.2) / across;
            let bb = in_r * rng.range(0.4, 1.1) * (0.5 + 0.5 * qv) / across;
            let (a, bb) = (a.min(6.0 * w), bb.min(8.0 * w));
            let start = (x - dx * a, y - dy * a);
            let end = (x + dx * bb, y + dy * bb);
            let bow = rng.normal() * 0.06 * (a + bb);
            let mid = (0.5 * (start.0 + end.0) - dy * bow, 0.5 * (start.1 + end.1) + dx * bow);
            // the neighbor's paint: what lies out past the start, off the edge
            let from = (start.0 - nx * w, start.1 - ny * w);
            let into = (x + nx * w, y + ny * w);
            plans.push((vec![start, mid, end], from, into));
        }
    }
    let mut held = Held::new(tool, seed ^ 0x5EED);
    let n = plans.len();
    for (pts, from, into) in plans {
        // aimed at the look over what's there where it crosses (as `b:load{at=}`)
        let (want, paint) = {
            let s = st.borrow();
            let cv = s.canvas.as_ref().ok_or_else(crate::api::no_canvas)?;
            let fw = cv.frame();
            let at = |p: (f32, f32)| cv.sample(p.0.clamp(0.0, fw.width() - 0.01), p.1.clamp(0.0, fw.height() - 0.01));
            let want = paint::color::mix(at(from), at(into), dirt, paint::Mix::Light);
            let mid = pts[pts.len() / 2];
            (want, cv.aim(&pal, want, mid, (0.5 * w).max(1.0), medium, 0.8))
        };
        held.wipe(0.9);
        held.load(paint, load);
        time::trip(st, want);
        let g = Gesture::new(pts).pressure(p0, p1).ramps(0.25, 0.6);
        time::verb(st, Verb::Marks, |s| {
            s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?.drag(&mut held, &g, None);
            Ok(())
        })?;
    }
    Ok(n)
}

pub fn install(lua: &Lua, st: S) -> Result<()> {
    let s1 = st.clone();
    lua.globals().set("lose", lua.create_function(move |lua, (m, o): (Value, Option<Table>)| lose(lua, &s1, m, o))?)?;
    Ok(())
}
