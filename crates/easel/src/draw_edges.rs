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

/// A stroke across the edge: its points, where its paint comes from, where it crosses into.
type Planned = (Vec<(f32, f32)>, (f32, f32), (f32, f32));

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
    let mut plans: Vec<Planned> = Vec::new();
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

#[cfg(test)]
mod tests {
    use crate::session::Session;

    const SETUP: &str = r##"canvas{style="friedrich", aspect=1.4, seed=5}
light = everywhere()
work(light, {hand="broad", color="#c9b99a", coverage=3, clip=true})
dry()
hill = ellipse(500, 400, 220, 120)"##;

    fn dark_outside(s: &Session) -> usize {
        // pixels a few units outside the hill that the dark passage darkened
        let c = s.canvas().unwrap();
        let f = c.frame();
        let mut n = 0;
        for k in 0..60 {
            let a = k as f32 / 60.0 * std::f32::consts::TAU;
            let (x, y) = (500.0 + 225.0 * a.cos(), 400.0 + 124.0 * a.sin());
            let p = c.seen()[f.index(x, y)];
            n += (p[0] + p[1] + p[2] < 0.9) as usize;
        }
        n
    }

    /// edge= carries a passage past its region's edge (the stencil stops on
    /// it); lose drags the neighbor back across; a mask passed as clip= is
    /// used (it used to be read as clip=true: the grown mask was ignored).
    #[test]
    fn edges_lose_and_the_clip_mask() {
        let mut a = Session::new(400, 2).unwrap();
        a.run(SETUP).unwrap();
        a.run(r##"work(hill, {hand="body", color="#2e2d33", coverage=3, clip=true})"##).unwrap();
        let stencil = dark_outside(&a);
        let mut g = Session::new(400, 2).unwrap();
        g.run(SETUP).unwrap();
        g.run(r##"work(hill, {hand="body", color="#2e2d33", coverage=3, clip=hill:grow(8)})"##).unwrap();
        let grown = dark_outside(&g);
        assert!(grown > 10, "a grown clip mask is used: {grown} of 60 points 5 units out darkened");
        let mut b = Session::new(400, 2).unwrap();
        b.run(SETUP).unwrap();
        let r = b.run(r##"work(hill, {hand="body", color="#2e2d33", coverage=3, edge="lost"})"##).unwrap();
        assert!(!r.out.contains("clip="), "{}", r.out);
        let lost = dark_outside(&b);
        assert!(stencil == 0 && lost > 10, "stencil {stencil}, lost {lost} of 60 points 5 units out darkened");
        // the other forms of edge=
        b.run(r##"work(hill, {hand="body", color="#2e2d33", coverage=1, edge={found=0.5, soft=0.3, lost=0.2, period=30}})
                  work(hill, {hand="body", color="#2e2d33", coverage=1, edge=function(x, y) return x / 1000 end})
                  work(hill, {hand="body", color="#2e2d33", coverage=1, edge=0.3})"##)
            .unwrap();
        assert!(b.run(r##"work(hill, {hand="body", color="#2e2d33", edge="blurry"})"##).is_err());
        assert!(b.run(r##"work(hill, {hand="body", color="#2e2d33", edge="soft", cut_in="round 2"})"##).is_err());
        // lose: strokes along the edge where asked, none where not
        b.run("dry()").unwrap();
        let r = b.run(r##"print(lose(hill, {where=function(x, y) return x < 500 and 1 or 0 end, tool="filbert 4"}))"##).unwrap();
        let n: usize = r.out.trim().lines().next().unwrap().trim().parse().unwrap();
        assert!(n > 20, "{}", r.out);
        let r = b.run(r##"print(lose(hill, {where=0}))"##).unwrap();
        assert_eq!(r.out.trim().lines().next().unwrap().trim(), "0");
    }
}
