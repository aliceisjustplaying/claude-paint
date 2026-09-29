//! Losing an edge: `lose(region, {pile=, ...})` drags a lightly loaded
//! brush across the region's edge from the outside in, after the passage
//! is laid.
//!
//! Along the region's edge, where `where` says (0 keep .. 1 lose), short
//! strokes start out in the neighbor, cross the edge and lift off inside,
//! each loaded lightly from the painter's pile, so it lays the pile thinner
//! as it runs out: over dry paint a scumble that breaks on the weave; into
//! wet paint it also picks up and drags what it crosses.
//!
//! ```lua
//! lose(m, {pile=p, where={lost=0.4, soft=0.6, period=60}, tool="filbert 4"})
//! ```

use crate::api::{S, check_keys, err, frame, num, pair, seed_of, support, tool_of};
use crate::time::{self, Verb};
use mlua::{Lua, Result, Table, Value};
use paint::rng::Rng;
use paint::{Gesture, Held};

/// A stroke across the edge: its points, where its paint comes from, where it crosses into.
type Planned = (Vec<(f32, f32)>, (f32, f32), (f32, f32));

const KEYS: &[&str] = &["pile", "where", "tool", "reach", "load", "pressure", "angle", "every", "seed"];

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
    let pile = crate::api::pile_of(&o.get::<Value>("pile")?, "lose")?;
    let tubes = st.borrow().tubes.clone();
    let jitter = crate::api::style(st)?.mix_jitter;

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
    for (pts, _from, _into) in plans {
        // a light load from the pile (remixed a little: a pile knifed by hand is uneven)
        let paint = tubes.remix(&pile.mix, jitter, &mut Rng::new(rng.next_u64())).laid(pile.medium);
        let want = pile.mix.color;
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

// every test here paints from the default box
#[cfg(all(test, tube_box))]
mod tests {
    use crate::session::Session;

    const SETUP: &str = r##"canvas{size=440, aspect=1.4, linen=15, seed=5, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=120, apply="knife"}}}
dark = pile{{"bone black", 1}, {"cobalt blue", 1}}
light = everywhere()
work(light, {hand="broad", pile=pile{{"lead white", 4}, {"yellow ochre", 1}}, coverage=3, clip=true})
wait(60 * 24 * 60)
shape = ellipse(500, 400, 220, 120)"##;

    fn dark_outside(s: &Session) -> usize {
        // pixels a few units outside the shape that the dark passage darkened
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
    /// it); lose drags a pile back across; a mask passed as clip= clips to
    /// that mask.
    #[test]
    fn edges_lose_and_the_clip_mask() {
        let mut a = Session::new(400).unwrap();
        a.run(SETUP).unwrap();
        a.run(r##"work(shape, {hand="body", pile=dark, coverage=3, clip=true})"##).unwrap();
        let stencil = dark_outside(&a);
        let mut g = Session::new(400).unwrap();
        g.run(SETUP).unwrap();
        g.run(r##"work(shape, {hand="body", pile=dark, coverage=3, clip=shape:grow(8)})"##).unwrap();
        let grown = dark_outside(&g);
        assert!(grown > 10, "a grown clip mask is used: {grown} of 60 points 5 units out darkened");
        let mut b = Session::new(400).unwrap();
        b.run(SETUP).unwrap();
        let r = b.run(r##"work(shape, {hand="body", pile=dark, coverage=3, edge="lost"})"##).unwrap();
        assert!(!r.out.contains("clip="), "{}", r.out);
        let lost = dark_outside(&b);
        assert!(stencil == 0 && lost > 10, "stencil {stencil}, lost {lost} of 60 points 5 units out darkened");
        // the other forms of edge=
        b.run(r##"work(shape, {hand="body", pile=dark, coverage=1, edge={found=0.5, soft=0.3, lost=0.2, period=30}})
                  work(shape, {hand="body", pile=dark, coverage=1, edge=function(x, y) return x / 1000 end})
                  work(shape, {hand="body", pile=dark, coverage=1, edge=0.3})"##)
            .unwrap();
        assert!(b.run(r##"work(shape, {hand="body", pile=dark, edge="blurry"})"##).is_err());
        assert!(b.run(r##"work(shape, {hand="body", pile=dark, edge="soft", cut_in="round 2"})"##).is_err());
        // lose: strokes along the edge where asked, none where not
        b.run("wait(60 * 24 * 60)").unwrap();
        let r = b.run(r##"print(lose(shape, {pile=pile{{"lead white", 4}, {"yellow ochre", 1}}, where=function(x, y) return x < 500 and 1 or 0 end, tool="filbert 4"}))"##).unwrap();
        let n: usize = r.out.trim().lines().next().unwrap().trim().parse().unwrap();
        assert!(n > 20, "{}", r.out);
        let r = b.run(r##"print(lose(shape, {pile=dark, where=0}))"##).unwrap();
        assert_eq!(r.out.trim().lines().next().unwrap().trim(), "0");
    }
}
