//! Finishing: varnish, cracks and relief over the whole dry picture.
//!
//! Available only in developer mode and the outside runner. Painter mode
//! has none of these verbs: finishing is applied
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
    let w = time::read("wet_share", || {
        let s = st.borrow();
        Ok(wet_share(s.canvas.as_ref().ok_or_else(no_canvas)?))
    })?;
    if w > 0.0 {
        let pc = if w < 0.005 { "under 1%".to_string() } else { format!("{:.0}%", 100.0 * w) };
        return err(format!("{what}: the paint is not all dry yet ({pc} of the canvas is still open, setting or tacky; drying(x, y) tells where): wait first"));
    }
    Ok(())
}

/// The finishing chunk `scripts/finish_painting` appends to a painter's log (and `easel
/// finish` runs on a save): wait a month at a time until every film is touch-dry (the
/// finishing verbs refuse wet paint), for at most ten years, then varnish (`coats` as given,
/// a number), crack and light the relief, each if asked. Byte for byte the chunk the script
/// wrote before it asked the easel for it.
pub fn chunk(coats: &str, varnish: bool, cracks: bool, relief: bool) -> String {
    if !(varnish || cracks || relief) {
        return String::new();
    }
    let mut s = String::from(
        "-- finishing, applied after the session by scripts/finish_painting
local function when_dry(f)
  for _ = 1, 120 do
    local ok, e = pcall(f)
    if ok then return end
    if not tostring(e):find('not all dry', 1, true) then error(e, 0) end
    wait(30 * 24 * 60)
  end
  error('still not dry after ten years', 0)
end",
    );
    if varnish {
        s.push_str(&format!("\nwhen_dry(function() varnish{{coats={coats}}} end)"));
    }
    if cracks {
        s.push_str("\nwhen_dry(function() cracks{} end)");
    }
    if relief {
        s.push_str("\nwhen_dry(function() relief() end)");
    }
    s
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
        if !(coats.is_finite() && vary.is_finite() && coats >= 0.0 && vary >= 0.0) {
            return err("varnish: coats and vary must be nonnegative and finite");
        }
        if st1.borrow().canvas.as_ref().is_some_and(|canvas| canvas.material_revision() == 1) { time::flush(&st1, true)?; }
        // brushed over the whole canvas once it is dry
        needs_dry(&st1, "varnish")?;
        time::verb(&st1, Verb::Pass, |s| {
            let var = Fbm::new(s.seed as u32 + seed, 3, 400.0);
            let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
            let laid = coats > 0.0 || vary > 0.0;
            if c.material_revision() == 1 && laid { brushed(c, None); }
            c.try_glaze(&Pigment::varnish(hex(MASTIC)), None, |x, y| coats + vary * var.get(x, y)).map_err(mlua::Error::external)?;
            // (a varnished surface is glossy: engine 4 shows no matte veil;
            // no coats, no varnish, and from engine 4 not the hand's time for
            // brushing it: an older log keeps the time it always had)
            if laid {
                c.varnished();
            }
            if c.material_revision() == 0 && (laid || c.engine() < 4) {
                brushed(c, None);
            }
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
        }).inspect(|_| st1.borrow_mut().mark_effects += 1)
    })?)?;
    let st1 = st.clone();
    g.set("relief", lua.create_function(move |_, (strength, gloss): (Option<f32>, Option<f32>)| {
        let sty = style(&st1)?;
        needs_dry(&st1, "relief")?;
        time::verb(&st1, Verb::Query, |s| {
            s.canvas.as_mut().ok_or_else(no_canvas)?.relief(strength.unwrap_or(sty.relief.0), gloss.unwrap_or(sty.relief.1));
            Ok(())
        }).inspect(|_| st1.borrow_mut().mark_effects += 1)
    })?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::session::Session;
    use paint::oil::chemistry::{Cell, Kinetics};
    use paint::oil::store::{DepositMode, Formulation};
    use paint::oil::transfer::{Fragment, MaterialTransfer, TransferCell};
    use std::sync::Arc;

    #[test]
    fn material_varnish_settles_prior_and_application_hand_time_before_sealing() {
        let mut palette = paint::Palette::tube_box(); palette.engine = 9; palette.material_revision = 1;
        let mut session = Session::with_box(3, palette).unwrap();
        session.run("canvas{size=50,aspect=1,linen=15}").unwrap();
        {
            let mut studio = session.st.borrow_mut();
            let canvas = studio.canvas.as_mut().unwrap();
            canvas.set_hand_time(Some(0.25));
            let paint = paint::Paint::body([0.2, 0.3, 0.4]);
            let recipe = Arc::new(Formulation::new(paint.latent(), paint.prop(), Kinetics::default()).unwrap());
            canvas.edit_material(|store| {
                let area = store.pixel_area();
                let mut fragment = Fragment::new(vec![TransferCell::from_cell(Cell { pigment_um: 10.0, binder_um: 10.0, oxygen: 1.0, oxidation: 1.0, network: 1.0, ..Cell::default() }, recipe, area)], 1.0);
                fragment.reference_area = area;
                store.deposit(4, MaterialTransfer { fragments: vec![fragment], free_solvent: 0.0 }, DepositMode::Cover)
            }).unwrap();
            canvas.tally_mut().glaze(1.0);
            assert!(canvas.hand_owed_secs() > 0.0);
        }
        let before = session.canvas().unwrap().clock();
        session.run("varnish{coats=0.4,vary=0}; relief(0.2,0.1)").unwrap();
        let canvas = session.canvas().unwrap();
        assert!(canvas.clock() > before);
        assert_eq!(canvas.hand_owed_secs(), 0.0);
        assert_eq!(canvas.material_store().unwrap().read_column(4).unwrap().inert_coatings().count(), 1);
        let mut retained = Vec::new(); canvas.material_store().unwrap().write_to(&mut retained).unwrap();
        drop(canvas);
        assert!(session.run("wait(1)").unwrap_err().contains("unsupported material operation"));
        let mut after = Vec::new(); session.canvas().unwrap().material_store().unwrap().write_to(&mut after).unwrap();
        assert_eq!(after, retained);
    }
}
