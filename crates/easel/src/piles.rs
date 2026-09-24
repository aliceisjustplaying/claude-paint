//! `piles{}`: mix a few piles for a color field, and paint from them.
//!
//!   sky = piles(skycol, {n=6, over=skyM})     -- piles along the field's range
//!   work(skyM, {hand="broad", color=sky, ...}) -- each stroke dips into one
//!   blend(skyM, {...})                         -- the steps softened on the canvas
//!
//! The piles are knifed when `piles` is called, from the palette's tubes,
//! each aimed at how it will look over what is on the canvas where it goes
//! (paint::piles). A pile set is a color like any other: `color=` in
//! `work` (strokes load from the pile's recipe, batch by batch), `stipple`
//! and the rest (the field stepped to the piles' looks).

use crate::api::{Col, S, check_keys, color_field, err, frame, mask_of, num, palette_of, rgb_of, seed_of, style};
use crate::time::{self, Verb};
use mlua::{Lua, MetaMethod, Result, Table, UserData, UserDataMethods, Value};
use paint::palette::Marks;
use paint::piles::{PileOpts, PileSet};
use paint::{Mask, Rgb};
use std::rc::Rc;
use std::sync::Arc;

/// A pile set on the palette (immutable once mixed).
pub struct PilesU(pub Arc<PileSet>);

impl UserData for PilesU {
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        // p:colors() -> the piles' looks, dark to light (for a field chosen
        // by piles{}; in the painter's order for colors={...})
        m.add_method("colors", |_, p, ()| Ok(p.0.piles.iter().map(|q| Col(q.want)).collect::<Vec<_>>()));
        // p:recipes() -> "look: tube parts" per pile
        m.add_method("recipes", |_, p, ()| Ok(p.0.describe()));
        // p:at(x, y) -> the look of the pile a stroke centered there dips into
        m.add_method("at", |_, p, (x, y): (f32, f32)| Ok(Col(p.0.stepped(x, y))));
        // p:pick(x, y) -> which pile (1-based)
        m.add_method("pick", |_, p, (x, y): (f32, f32)| Ok(p.0.pick(x, y) + 1));
        // p:field(x, y) -> the field the piles were mixed for, there
        m.add_method("field", |_, p, (x, y): (f32, f32)| Ok(Col(p.0.field_at(x, y))));
        m.add_meta_method(MetaMethod::Len, |_, p, ()| Ok(p.0.piles.len()));
        m.add_meta_method(MetaMethod::ToString, |_, p, ()| Ok(format!("piles({}):\n  {}", p.0.piles.len(), p.0.describe().join("\n  "))));
    }
}

const KEYS: &[&str] = &["n", "over", "colors", "pal", "medium", "coats", "aim", "hand", "tool", "coverage", "load", "mix", "overlap", "patch", "vary", "batch", "dirty", "seed"];

fn piles(st: &S, field: Value, o: Table) -> Result<PilesU> {
    let f = frame(st)?;
    let sty = style(st)?;
    check_keys(&o, KEYS, "piles")?;
    let over: Rc<Mask> = match o.get::<Value>("over")? {
        Value::Nil => Rc::new(Mask::full(f)),
        v => mask_of(&v)?,
    };
    // the field, read over the whole canvas (a pile set serves any pass)
    let fld = grid(color_field(st, &field, (0.0, 0.0, f.width(), f.height()))?, f.width(), f.height());
    let d = PileOpts::default();
    let opt = |k: &str, dflt: f32| -> Result<f32> { Ok(num(&o, k)?.unwrap_or(dflt).max(0.0)) };
    let opts = PileOpts {
        overlap: opt("overlap", d.overlap)?.min(1.0),
        patch: opt("patch", d.patch)?.max(1.0),
        vary: opt("vary", d.vary)?.min(0.5),
        batch: opt("batch", d.batch)?.max(1.0),
        dirty: opt("dirty", d.dirty)?.min(0.5),
        seed: seed_of(st, &o)?,
    };
    // the painter's own piles, or n chosen from the field
    let given: Option<Vec<Rgb>> = match o.get::<Value>("colors")? {
        Value::Nil => None,
        Value::Table(t) => {
            let cs: Vec<Rgb> = t.sequence_values::<Value>().map(|v| rgb_of(&v?)).collect::<Result<_>>()?;
            if cs.is_empty() || cs.len() > 16 {
                return err("piles: colors={...} wants 1 to 16 colors");
            }
            Some(cs)
        }
        v => return err(format!("piles: colors wants a list of colors, got {}", v.type_name())),
    };
    let n = o.get::<Option<usize>>("n")?.unwrap_or(5);
    if !(1..=16).contains(&n) {
        return err(format!("piles: n = {n}: a palette holds 1 to 16 piles"));
    }
    let mix = o.get::<Option<bool>>("mix")?.unwrap_or(true);
    if !mix {
        let set = match given {
            Some(cs) => PileSet::new(cs, fld, opts),
            None => PileSet::from_field(fld, &over, n, 3.0, opts),
        };
        return Ok(PilesU(Arc::new(set)));
    }
    {
        let pal = palette_of(st, o.get::<Value>("pal")?)?;
        let medium = num(&o, "medium")?.unwrap_or(sty.thin_medium).clamp(0.0, 1.0);
        // aimed at the thickness the pass they are mixed for lays (as
        // `work` estimates it: `Handling::laid_coats`), unless asked otherwise
        let pass = || -> Result<(f32, Marks)> {
            let mut h = match o.get::<Option<String>>("hand")?.as_deref().unwrap_or("broad") {
                "broad" => sty.broad(),
                "body" => sty.body(),
                "detail" => sty.detail(),
                "hatch" => sty.hatch(),
                "scumble" => sty.scumble(),
                "glaze" => sty.glaze(num(&o, "medium")?.unwrap_or(0.9)),
                h => return err(format!("piles: hand {h:?}: broad, body, detail, hatch, glaze or scumble")),
            };
            if let Some(t) = o.get::<Option<Value>>("tool")? {
                h.tool = crate::api::tool_of(&t)?;
            }
            if let Some(c) = num(&o, "coverage")? {
                h = h.coverage(c);
            }
            if let Some(l) = num(&o, "load")? {
                h = h.load(l);
            }
            Ok((h.laid_coats(), Marks::of(&h.tool)))
        };
        let (laid, marks) = pass()?;
        let coats = match (o.get::<Value>("aim")?, num(&o, "coats")?) {
            (Value::String(s), _) if &*s.to_str()? == "masstone" => None,
            (Value::Nil, Some(c)) | (Value::String(_), Some(c)) => Some(c.max(0.05)),
            (Value::String(s), None) if &*s.to_str()? == "laid" => Some(laid),
            (Value::Nil, None) => Some(laid),
            (Value::Number(n), _) => Some((n as f32).max(0.05)),
            (Value::Integer(n), _) => Some((n as f32).max(0.05)),
            _ => return err("piles: aim = \"laid\", \"masstone\" or a number of coats"),
        };
        // the painter looks at the canvas and knifes the piles: each is a
        // new pile on the palette (hand time)
        let set = time::verb(st, Verb::Pass, |s| {
            let c = s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?;
            let set = match given {
                Some(cs) => {
                    let mut set = PileSet::new(cs, fld, opts);
                    set.mix(c, &over, &pal, medium, coats, marks, 6.0);
                    set
                }
                None => PileSet::by_paint(fld, c, &over, n, &pal, medium, coats, marks, 5.0, opts),
            };
            for p in &set.piles {
                s.hand.piles.trip(c.tally_mut(), p.want);
            }
            Ok(set)
        })?;
        Ok(PilesU(Arc::new(set)))
    }
}

/// The field read every `STEP` units into a plain grid (bilinear between
/// nodes), which the engine's threads can share.
fn grid(fld: crate::api::FieldBox<Rgb>, w: f32, h: f32) -> Box<dyn Fn(f32, f32) -> Rgb + Send + Sync> {
    const STEP: f32 = 2.0;
    let (nx, ny) = ((w / STEP).ceil() as usize + 1, (h / STEP).ceil() as usize + 1);
    let v: Vec<Rgb> = (0..ny).flat_map(|j| (0..nx).map(move |i| (i, j))).map(|(i, j)| fld((i as f32 * STEP).min(w - 0.01), (j as f32 * STEP).min(h - 0.01))).collect();
    Box::new(move |x, y| {
        let (u, t) = ((x / STEP).clamp(0.0, (nx - 1) as f32), (y / STEP).clamp(0.0, (ny - 1) as f32));
        let (i, j) = ((u as usize).min(nx - 2), (t as usize).min(ny - 2));
        let (a, b) = (u - i as f32, t - j as f32);
        let at = |i: usize, j: usize| v[j * nx + i];
        let (p, q, r, s) = (at(i, j), at(i + 1, j), at(i, j + 1), at(i + 1, j + 1));
        std::array::from_fn(|k| (p[k] * (1.0 - a) + q[k] * a) * (1.0 - b) + (r[k] * (1.0 - a) + s[k] * a) * b)
    })
}

pub fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    // piles(field, {n=5, over=mask, colors={...}, pal=, medium=, aim=, coats=,
    //   mix=true, overlap=0.5, patch=45, vary=0.08, batch=130, dirty=0.12, seed=})
    g.set("piles", lua.create_function(move |lua, (field, o): (Value, Option<Table>)| {
        let o = match o {
            Some(o) => o,
            None => lua.create_table()?,
        };
        piles(&st, field, o)
    })?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::session::Session;

    const SKY: &str = r##"skyM = above(function(x) return 420 end)
        skycol = function(x, y) return mix(gradient({{0,"#47536c"},{0.5,"#8b8d9c"},{1,"#d2bd98"}}, y/420), "#ecd49e", 0.6*math.exp(-((x-600)/250)^2)*y/420) end"##;

    fn bits(s: &Session) -> Vec<u32> {
        s.canvas().unwrap().seen().iter().flat_map(|p| p.map(f32::to_bits)).collect()
    }

    fn session(chunks: &[&str]) -> Session {
        let mut s = Session::replay(200).unwrap();
        s.run(r#"canvas{style="friedrich", aspect=1.4, seed=11}"#).unwrap();
        s.run(SKY).unwrap();
        for c in chunks {
            s.run(c).unwrap();
        }
        s
    }

    #[test]
    fn a_sky_from_piles_steps_and_replays_exactly() {
        let paint = r##"P = piles(skycol, {n=5, over=skyM, coverage=3, seed=4})
            assert(#P == 5, #P)
            local c = P:colors()
            for i = 2, #c do assert(c[i].L > c[i-1].L, "dark to light") end
            -- stepped: along a column the look changes in a few steps, not every unit
            local seen, steps, last = {}, 0, nil
            for y = 5, 415, 5 do local h = P:at(300, y):hex() if h ~= last then steps = steps + 1 end last = h seen[h] = true end
            assert(steps >= 4 and steps <= 30, steps)
            assert(P:pick(300, 5) == 1 and P:pick(300, 415) >= 4, P:pick(300, 5) .. " " .. P:pick(300, 415))
            assert(#P:recipes() == 5 and P:recipes()[1]:find("#"), P:recipes()[1])
            work(skyM, {hand="broad", color=P, angle=0, coverage=3, seed=5})
            blend(skyM, {angle=0, coverage=1.2, seed=6})"##;
        let a = session(&[paint]);
        let b = session(&[paint]);
        assert_eq!(bits(&a), bits(&b), "deterministic");
        // a formula sky paints differently (the piles are what the strokes load)
        let c = session(&[r##"work(skyM, {hand="broad", color=skycol, angle=0, coverage=3, seed=5})
            blend(skyM, {angle=0, coverage=1.2, seed=6})"##]);
        assert_ne!(bits(&a), bits(&c));
    }

    #[test]
    fn piles_options_and_errors() {
        let s = session(&[]);
        let mut s = s;
        // the painter's own piles, unmixed: the field only says where each goes
        s.run(r##"Q = piles(skycol, {colors={"#505c75", "#a99c9d", "#d6c19c"}, mix=false, seed=1})
            assert(#Q == 3 and Q:recipes()[1]:find("not mixed"))
            stipple(skyM, {width=2, color=Q, coverage=0.3, seed=2})"##)
            .unwrap();
        for (bad, want) in [
            (r##"piles(skycol, {n=0})"##, "1 to 16"),
            (r##"piles(skycol, {colour="#fff"})"##, "unknown option"),
            (r##"piles(skycol, {aim="thick"})"##, "aim"),
            (r##"piles(skycol, {hand="wave"})"##, "hand"),
        ] {
            let e = s.run(bad).unwrap_err();
            assert!(e.contains(want), "{bad}: {e}");
        }
    }
}
