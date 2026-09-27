//! Finishing: varnish, cracks and relief over the whole dry picture.
//!
//! Only in builds with the `finish` feature (developers and the outside
//! runner). The painter build has none of these verbs: finishing is applied
//! after the session, by `scripts/finish_painting`, so that it can't become
//! the painter's way of ending.

use crate::api::{S, check_keys, err, no_canvas, num, style};
use crate::time::{self, Verb};
use mlua::{Lua, Result, Table};
use paint::{Canvas, Cracks, Fbm, Mask, Pigment, hex};

/// The varnish: a mastic resin film as it looks once it has aged (it only
/// absorbs, warming the darks without veiling them).
const MASTIC: &str = "#e6d3a4";

/// Brushing a glaze or a varnish over `m` (None: the whole canvas) is hand
/// time, priced by its area (counted; on the clock with hand time on).
fn brushed(c: &mut Canvas, m: Option<&Mask>) {
    let f = c.frame();
    let mm2 = match m {
        Some(m) => m.data.iter().map(|&v| v as f64).sum::<f64>() / (m.f.scale as f64).powi(2),
        None => (f.width() * f.height()) as f64,
    } * (c.mm_per_unit() as f64).powi(2);
    c.tally_mut().glaze(mm2);
}

/// The share of the canvas whose paint isn't touch-dry yet (0: all dry).
fn wet_share(c: &Canvas) -> f64 {
    let sh = c.stage_shares();
    sh[0] + sh[1] + sh[2]
}

/// Whole-picture finishing requires every film to be touch-dry.
fn needs_dry(st: &S, what: &str) -> Result<()> {
    let s = st.borrow();
    let c = s.canvas.as_ref().ok_or_else(no_canvas)?;
    let w = wet_share(c);
    if w > 0.0 {
        let pc = if w < 0.005 { "under 1%".to_string() } else { format!("{:.0}%", 100.0 * w) };
        return err(format!("{what}: the paint is not all dry yet ({pc} of the canvas is still open, setting or tacky; drying(x, y) tells where): wait first"));
    }
    Ok(())
}

pub(crate) fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    let st1 = st.clone();
    g.set("varnish", lua.create_function(move |_, o: Option<Table>| {
        let (coats, vary, seed) = match &o {
            None => (0.4, 0.12, 98),
            Some(o) => {
                check_keys(o, &["coats", "vary", "seed"], "varnish")?;
                (num(o, "coats")?.unwrap_or(0.4), num(o, "vary")?.unwrap_or(0.12), o.get::<Option<u32>>("seed")?.unwrap_or(98))
            }
        };
        // brushed over the whole canvas once it is dry
        needs_dry(&st1, "varnish")?;
        time::verb(&st1, Verb::Pass, |s| {
            let var = Fbm::new(s.seed as u32 + seed, 3, 400.0);
            let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
            c.glaze(&Pigment::varnish(hex(MASTIC)), None, |x, y| coats + vary * var.get(x, y));
            brushed(c, None);
            Ok(())
        })
    })?)?;
    let st1 = st.clone();
    g.set("cracks", lua.create_function(move |_, o: Option<Table>| {
        let mut k = Cracks::aged(st1.borrow().seed);
        if let Some(o) = &o {
            check_keys(o, &["island_mm", "ground_um", "width_um", "depth_um", "cupping_um", "dirt", "corners", "vary", "veil", "hierarchy", "patchy", "grain", "grime", "seed"], "cracks")?;
            // unset: fitted to this canvas's ground (Cracks::aged)
            k.island_mm = num(o, "island_mm")?.or(k.island_mm);
            k.ground_um = num(o, "ground_um")?.or(k.ground_um);
            k.width_um = num(o, "width_um")?.or(k.width_um);
            macro_rules! over {
                ($($f:ident),*) => {$( if let Some(v) = num(o, stringify!($f))? { k.$f = v; } )*};
            }
            over!(depth_um, cupping_um, dirt, vary, veil, hierarchy, patchy, grain, grime);
            if let Some(c) = o.get::<Option<bool>>("corners")? {
                k.corners = c;
            }
            if let Some(sd) = o.get::<Option<u64>>("seed")? {
                k.seed = sd;
            }
        }
        needs_dry(&st1, "cracks")?;
        time::verb(&st1, Verb::Query, |s| {
            s.canvas.as_mut().ok_or_else(no_canvas)?.crack(&k);
            Ok(())
        })
    })?)?;
    let st1 = st.clone();
    g.set("relief", lua.create_function(move |_, (strength, gloss): (Option<f32>, Option<f32>)| {
        let sty = style(&st1)?;
        needs_dry(&st1, "relief")?;
        time::verb(&st1, Verb::Query, |s| {
            s.canvas.as_mut().ok_or_else(no_canvas)?.relief(strength.unwrap_or(sty.relief.0), gloss.unwrap_or(sty.relief.1));
            Ok(())
        })
    })?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::session::Session;

    // Finishing verbs brush over the whole picture, so every film must be
    // touch-dry first, however small the wet part; after a long wait they run.
    #[test]
    fn finishing_waits_for_the_whole_picture_to_dry() {
        let mut s = Session::new(100).unwrap();
        s.run(r#"canvas{size=300, aspect=1, linen=15,
                ground={{pile={{"lead white",1}},um=100,apply="knife"}}}"#).unwrap();
        s.run(r#"b = brush("flat", 20); b:load(pile{{"bone black",1}}); b:stroke({100,200,400,200})"#).unwrap();
        for verb in ["varnish()", "cracks()", "relief()"] {
            let e = s.run(verb).unwrap_err();
            assert!(e.contains("not all dry yet") && e.contains("wait first"), "{verb}: {e}");
        }
        s.run("wait(90*24*60); varnish()").unwrap();
    }
}
