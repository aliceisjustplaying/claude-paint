//! Steam on pastel and sifted pastel (`--@ steam-sift`; notes/cassatt_support.md gap 6;
//! paint's `steam`).
//!
//! - `steam(pts, {seconds=, distance=})`: a boiling kettle's jet moved along the points (units)
//!   at an even pace, the pass taking `seconds` (one point: held there), `distance` mm from the
//!   spout (200 if not given). Each point of the sheet is steamed for as long as the jet's
//!   steam share is on it, by the jet's own spread (paint's `steam_exposure`): at 200 mm the
//!   share halves 22 mm off the axis and nothing condenses past 47 mm. The vapour condenses on
//!   the cooler sheet and pastel: the pastel's
//!   pores take the water first, the rest stands and sinks into the sheet, the gum dissolves,
//!   and as it dries on the clock the gum binds the loose pastel there: it smudges and lifts
//!   less. Nothing moves sideways unless the wet pastel is then worked (`b:wash`). Returns the
//!   water condensed on the jet's track (µm) and the face's temperature there (°C).
//! - `sift(stick or pile, pts, {amount=, grain=, sieve=})`: `amount` mm³ of the stick (it
//!   shortens) or of a pile's color, crushed and rubbed through a sieve of `grain` mm aperture
//!   (0.5 if not given) and `sieve` mm across (60), held over the points (units) and moved
//!   along them at an even pace, with no stroke: the crumbs fall straight down under its mesh,
//!   and what falls beside the sheet is lost. On a dry sheet the crumbs lie loose and `tap`,
//!   `blow` or tilting (`tap{g=1}`) take most of them; on a damp one (steamed, or a damp
//!   sheet's own water) they drink the water, stick, and gather into clusters as it dries, the
//!   more water the larger, and the gum binds them. Returns a table: `laid`, `off`, `fines`,
//!   `loose`, `stuck`, `caught` (mm³), `cluster`, `film`, `crumb` (µm).
//!
//! Neither takes a mask, a function or a field: where they act is where the hand moves the
//! kettle or the sieve. Both need the paper-and-pastel switch's sheet; `steam` needs the wet
//! pastel switch too.

use crate::api::{S, check_keys, err, no_canvas, num, points};
use crate::time::{self, Verb};
use mlua::{Lua, Result, Table, Value};

/// The kettle's distance if `steam` names none, mm [E: a gesture; Degas played the jet over
/// the sheet from a hand's span or so].
const STEAM_DISTANCE_MM: f64 = 200.0;
/// The sieve's aperture if `sift` names none, mm [E: a kitchen or tea strainer's 0.3–1 mm].
const SIFT_GRAIN_MM: f64 = 0.5;
/// The sieve's mesh across if `sift` names none, mm [E: a tea strainer's bowl, 50–80 mm].
const SIEVE_MM: f64 = 60.0;
/// A stick can't be crushed shorter than this, mm (what the fingers hold: as `b:dip` grinds it).
const HOLD_MM: f32 = 5.0;

const STEAM_HELP: &str = "steam({{x, y}, ...}, {seconds=<the pass; one point: held there>, distance=<mm from the spout, 200>}): the kettle moved along the points";
const SIFT_HELP: &str = "sift(stick or pile, {{x, y}, ...}, {amount=<mm³ crushed>, grain=<aperture mm, 0.5>, sieve=<mm across, 60>}): the sieve held over the points";

/// What `sift` reports (and a replay records).
#[derive(serde::Serialize, serde::Deserialize)]
struct Sifted {
    laid: f64,
    off: f64,
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

/// Where the hand moves the kettle or the sieve: a list of points (units). A mask, a function
/// or nil (the whole sheet) is refused, saying so.
fn gesture(v: &Value, verb: &str, help: &str) -> Result<Vec<(f32, f32)>> {
    let what = match v {
        Value::Table(_) => None,
        Value::Nil => Some("nil (the whole sheet)"),
        Value::Function(_) => Some("a function"),
        Value::UserData(_) => Some("a mask or field"),
        o => Some(o.type_name()),
    };
    if let Some(what) = what {
        let tool = if verb == "steam" { "kettle" } else { "sieve" };
        return err(format!("{verb}: takes the points the hand moves the {tool} along, {{{{x, y}}, ...}}, not {what}: give the points it passes over (a row of points across a passage, a zigzag over a larger one)\n{help}"));
    }
    let pts = points(v).map_err(|e| mlua::Error::runtime(format!("{verb}: {e}\n{help}")))?;
    if pts.is_empty() || pts.len() > 10_000 || pts.iter().any(|p| !(p.0.is_finite() && p.1.is_finite() && p.0.abs() < 1e5 && p.1.abs() < 1e5)) {
        return err(format!("{verb}: 1 to 10000 points, finite (units)\n{help}"));
    }
    Ok(pts)
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
            let pts = gesture(&m, "steam", STEAM_HELP)?;
            let secs = opt(&o, "seconds")?.ok_or_else(|| mlua::Error::runtime(format!("steam: seconds= (how long the pass takes, or the jet is held at one point)\n{STEAM_HELP}")))?;
            if !(secs > 0.0 && secs <= 3600.0) {
                return err(format!("steam: seconds={secs}: want 0 to 3600 for the pass\n{STEAM_HELP}"));
            }
            let dist = opt(&o, "distance")?.unwrap_or(STEAM_DISTANCE_MM);
            if !(100.0..=600.0).contains(&dist) {
                return err(format!("steam: distance={dist}: want 100 to 600 mm from the spout (nearer than ten spouts across the jet hasn't mixed)\n{STEAM_HELP}"));
            }
            time::verb_dry(&st, Verb::Pass, |s| {
                let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
                let r = c.steam(&pts, secs, dist).map_err(mlua::Error::runtime)?;
                c.tally_mut().secs += r.secs;
                Ok((r.water_um, r.face_c))
            })
        })?)?;
    }
    {
        let st = st.clone();
        g.set("sift", lua.create_function(move |_, (src, m, o): (Value, Value, Option<Table>)| {
            if let Some(o) = &o {
                check_keys(o, &["amount", "grain", "sieve"], "sift")?;
            }
            let pts = gesture(&m, "sift", SIFT_HELP)?;
            let amount = opt(&o, "amount")?.ok_or_else(|| mlua::Error::runtime(format!("sift: amount= (mm³ of pastel crushed)\n{SIFT_HELP}")))?;
            if !(amount > 0.0 && amount <= 1e5) {
                return err(format!("sift: amount={amount}: want mm³ of pastel, more than 0\n{SIFT_HELP}"));
            }
            let grain = opt(&o, "grain")?.unwrap_or(SIFT_GRAIN_MM);
            if !(0.05..=5.0).contains(&grain) {
                return err(format!("sift: grain={grain}: the sieve's aperture, 0.05 to 5 mm\n{SIFT_HELP}"));
            }
            let sieve = opt(&o, "sieve")?.unwrap_or(SIEVE_MM);
            if !(10.0..=300.0).contains(&sieve) {
                return err(format!("sift: sieve={sieve}: the sieve's mesh across, 10 to 300 mm\n{SIFT_HELP}"));
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
            let r = time::verb_dry(&st, Verb::Pass, |s| {
                let c = s.canvas.as_mut().ok_or_else(no_canvas)?;
                // (the fall's draw follows the painting's clock: a replay falls the same)
                let seed = c.now_min().to_bits() ^ amount.to_bits();
                let r = c.sift(&pts, amount, grain, sieve, &stick, seed).map_err(mlua::Error::runtime)?;
                // (crushing and sifting: as grinding pastel, mixing a pile)
                c.tally_mut().secs += paint::tally::pace::REMIX;
                Ok(Sifted { laid: r.laid_mm3, off: r.off_mm3, fines: r.fines_mm3, loose: r.loose_mm3, stuck: r.stuck_mm3, caught: r.caught_mm3, cluster: r.cluster_um, film: r.film_um, crumb: r.crumb_um })
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
        t.set("off", self.off)?;
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
