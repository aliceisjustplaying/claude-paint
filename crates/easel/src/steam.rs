//! Steam on pastel and sifted pastel (`--@ steam-sift`; notes/cassatt_support.md gap 6;
//! paint's `steam`).
//!
//! - `steam(mask, {seconds=, distance=})`: a boiling kettle's jet played over the mask (nil:
//!   the whole sheet), `seconds` at each point of it (a soft edge less), `distance` mm from the
//!   spout (200 if not given). The vapour condenses on the cooler sheet and pastel: the pastel's
//!   pores take the water first, the rest stands and sinks into the sheet, the gum dissolves,
//!   and as it dries on the clock the gum binds the loose pastel there: it smudges and lifts
//!   less. Nothing moves sideways unless the wet pastel is then worked (`b:wash`). Returns the
//!   water condensed (µm, where the mask is full) and the face's temperature (°C).
//! - `sift(stick or pile, mask, {amount=, grain=})`: `amount` mm³ of the stick (it shortens) or
//!   of a pile's color, crushed and rubbed through a sieve of `grain` mm (0.5 if not given) over
//!   the mask (nil: all of it), with no stroke. On a dry sheet the crumbs lie loose and `tap`,
//!   `blow` or tilting (`tap{g=1}`) take most of them; on a damp one (steamed, or a damp
//!   sheet's own water) they drink the water, stick, and gather into clusters as it dries, the
//!   more water the larger, and the gum binds them. Returns a table: `laid`, `fines`, `loose`,
//!   `stuck`, `caught` (mm³), `cluster`, `film`, `crumb` (µm).
//!
//! Both need the paper-and-pastel switch's sheet; `steam` needs the wet pastel switch too.

use crate::api::{S, check_keys, err, no_canvas, num, paint_mask_of};
use crate::time::{self, Verb};
use mlua::{Lua, Result, Table, Value};

/// The kettle's distance if `steam` names none, mm [E: a gesture; Degas played the jet over
/// the sheet from a hand's span or so].
const STEAM_DISTANCE_MM: f64 = 200.0;
/// The sieve if `sift` names none, mm [E: a kitchen or tea strainer's 0.3–1 mm].
const SIFT_GRAIN_MM: f64 = 0.5;
/// A stick can't be crushed shorter than this, mm (what the fingers hold: as `b:dip` grinds it).
const HOLD_MM: f32 = 5.0;

const STEAM_HELP: &str = "steam(mask, {seconds=<at each point>, distance=<mm from the spout, 200>})";
const SIFT_HELP: &str = "sift(stick or pile, mask, {amount=<mm³ crushed>, grain=<sieve mm, 0.5>})";

/// What `sift` reports (and a replay records).
#[derive(serde::Serialize, serde::Deserialize)]
struct Sifted {
    laid: f64,
    fines: f64,
    loose: f64,
    stuck: f64,
    caught: f64,
    cluster: f64,
    film: f64,
    crumb: f64,
}

fn opt(o: &Option<Table>, k: &str) -> Result<Option<f64>> {
    Ok(match o {
        Some(o) => num(o, k)?.map(|v| v as f64),
        None => None,
    })
}

fn mask(st: &S, v: &Value) -> Result<Option<std::rc::Rc<paint::Mask>>> {
    match v {
        Value::Nil => Ok(None),
        v => paint_mask_of(st, v).map(Some),
    }
}

/// The switch's verbs, installed beside the paper-and-pastel switch's only when the log has
/// the switch, so an older log sees exactly the globals it saw.
pub(crate) fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    {
        let st = st.clone();
        g.set("steam", lua.create_function(move |_, (m, o): (Value, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, &["seconds", "distance"], "steam")?;
            }
            let secs = opt(&o, "seconds")?.ok_or_else(|| mlua::Error::runtime(format!("steam: seconds= (how long each point is in the steam)\n{STEAM_HELP}")))?;
            if !(secs > 0.0 && secs <= 600.0) {
                return err(format!("steam: seconds={secs}: want 0 to 600 at each point\n{STEAM_HELP}"));
            }
            let dist = opt(&o, "distance")?.unwrap_or(STEAM_DISTANCE_MM);
            if !(100.0..=600.0).contains(&dist) {
                return err(format!("steam: distance={dist}: want 100 to 600 mm from the spout (nearer than ten spouts across the jet hasn't mixed)\n{STEAM_HELP}"));
            }
            let m = mask(&st, &m)?;
            time::verb_dry(&st, Verb::Pass, |s| {
                let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
                let r = c.steam(m.as_deref(), secs, dist).map_err(mlua::Error::runtime)?;
                c.tally_mut().secs += r.secs;
                Ok((r.water_um, r.face_c))
            })
        })?)?;
    }
    {
        let st = st.clone();
        g.set("sift", lua.create_function(move |_, (src, m, o): (Value, Value, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, &["amount", "grain"], "sift")?;
            }
            let amount = opt(&o, "amount")?.ok_or_else(|| mlua::Error::runtime(format!("sift: amount= (mm³ of pastel crushed)\n{SIFT_HELP}")))?;
            if !(amount > 0.0 && amount <= 1e5) {
                return err(format!("sift: amount={amount}: want mm³ of pastel, more than 0\n{SIFT_HELP}"));
            }
            let grain = opt(&o, "grain")?.unwrap_or(SIFT_GRAIN_MM);
            if !(0.05..=5.0).contains(&grain) {
                return err(format!("sift: grain={grain}: the sieve's aperture, 0.05 to 5 mm\n{SIFT_HELP}"));
            }
            // a stick (it shortens) or a pile (a stick of its color, crushed)
            let (stick, table) = match &src {
                Value::Table(t) if t.get::<Option<String>>("kind")?.as_deref() == Some("stick") => {
                    let mut s = crate::api::draw_pencil::stick_of(t)?;
                    let area = match s.section {
                        paint::pastel::Section::Round { d_mm } => std::f32::consts::FRAC_PI_4 * d_mm * d_mm,
                        paint::pastel::Section::Square { side_mm } => side_mm * side_mm,
                    };
                    let cut = amount as f32 / area;
                    if s.length_mm - cut < HOLD_MM {
                        return err(format!("sift: the stick is {:.0} mm long; crushing {amount} mm³ ({cut:.1} mm of it) would leave less than {HOLD_MM:.0} mm to hold", s.length_mm));
                    }
                    s.length_mm -= cut;
                    (s, Some(t.clone()))
                }
                v => {
                    let pile = crate::api::pile_of(v, "sift").map_err(|_| mlua::Error::runtime(format!("sift: the first argument is a pastel stick (pastel(pile)) or a pile\n{SIFT_HELP}")))?;
                    let tubes = st.borrow().tubes.clone();
                    let mut s = paint::pastel::Stick::round(tubes.dry_color(&pile.mix.parts), 0.75, 12.0);
                    if tubes.wet_pastel && tubes.drawing_optics {
                        s.index = tubes.dry_index(&pile.mix.parts);
                    }
                    (s, None)
                }
            };
            let m = mask(&st, &m)?;
            let r = time::verb_dry(&st, Verb::Pass, |s| {
                let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
                // (the fall's draw follows the painting's clock: a replay falls the same)
                let seed = c.now_min().to_bits() ^ amount.to_bits();
                let r = c.sift(m.as_deref(), amount, grain, &stick, seed).map_err(mlua::Error::runtime)?;
                // (crushing and sifting: as grinding pastel, mixing a pile)
                c.tally_mut().secs += paint::tally::pace::REMIX;
                Ok(Sifted { laid: r.laid_mm3, fines: r.fines_mm3, loose: r.loose_mm3, stuck: r.stuck_mm3, caught: r.caught_mm3, cluster: r.cluster_um, film: r.film_um, crumb: r.crumb_um })
            })?;
            if let Some(t) = table {
                t.set("length", stick.length_mm)?;
            }
            Ok(r)
        })?)?;
    }
    Ok(())
}

impl mlua::IntoLua for Sifted {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        let t = lua.create_table()?;
        t.set("laid", self.laid)?;
        t.set("fines", self.fines)?;
        t.set("loose", self.loose)?;
        t.set("stuck", self.stuck)?;
        t.set("caught", self.caught)?;
        t.set("cluster", self.cluster)?;
        t.set("film", self.film)?;
        t.set("crumb", self.crumb)?;
        Ok(Value::Table(t))
    }
}

#[cfg(test)]
mod tests;
