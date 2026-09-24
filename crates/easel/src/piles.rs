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

const KEYS: &[&str] = &["n", "over", "colors", "pal", "medium", "coats", "aim", "mix", "overlap", "patch", "vary", "batch", "dirty", "seed"];

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
    let mut set = match o.get::<Value>("colors")? {
        Value::Nil => {
            let n = o.get::<Option<usize>>("n")?.unwrap_or(5);
            if !(1..=16).contains(&n) {
                return err(format!("piles: n = {n}: a palette holds 1 to 16 piles"));
            }
            PileSet::from_field(fld, &over, n, 3.0, opts)
        }
        Value::Table(t) => {
            let cs: Vec<Rgb> = t.sequence_values::<Value>().map(|v| rgb_of(&v?)).collect::<Result<_>>()?;
            if cs.is_empty() || cs.len() > 16 {
                return err("piles: colors={...} wants 1 to 16 colors");
            }
            PileSet::new(cs, fld, opts)
        }
        v => return err(format!("piles: colors wants a list of colors, got {}", v.type_name())),
    };
    if o.get::<Option<bool>>("mix")?.unwrap_or(true) {
        let pal = palette_of(st, o.get::<Value>("pal")?)?;
        let medium = num(&o, "medium")?.unwrap_or(sty.thin_medium).clamp(0.0, 1.0);
        // aimed like a broad pass lays it, unless asked otherwise
        let coats = match (o.get::<Value>("aim")?, num(&o, "coats")?) {
            (Value::String(s), _) if &*s.to_str()? == "masstone" => None,
            (Value::Nil, Some(c)) | (Value::String(_), Some(c)) => Some(c.max(0.05)),
            (Value::String(s), None) if &*s.to_str()? == "laid" => Some(sty.broad().laid_coats()),
            (Value::Nil, None) => Some(sty.broad().laid_coats()),
            (Value::Number(n), _) => Some((n as f32).max(0.05)),
            (Value::Integer(n), _) => Some((n as f32).max(0.05)),
            _ => return err("piles: aim = \"laid\", \"masstone\" or a number of coats"),
        };
        // the painter looks at the canvas and knifes the piles: each is a
        // new pile on the palette (hand time)
        time::verb(st, Verb::Pass, |s| {
            let c = s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?;
            set.mix(c, &over, &pal, medium, coats, Marks::Blunt, 6.0);
            for p in &set.piles {
                s.hand.piles.trip(c.tally_mut(), p.want);
            }
            Ok(())
        })?;
    }
    Ok(PilesU(Arc::new(set)))
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
