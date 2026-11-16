//! Tracing a study onto another surface (`--@ trace`; notes/plan-intaglio.md §7, gap 4 of
//! notes/cassatt_support.md; engine: paint's `trace`).
//!
//! `trace(study, {onto=, by=, tool=, coat=, pressure=, force=, tip=, mirror=, through=, at=,
//! level=, seed=})` follows the study's drawn lines (its `drawing_guide()`) and lays them on
//! `onto`, by default the current surface:
//!
//! - **`by="transfer"`** (the default): the lines are traced onto a sheet whose back is rubbed
//!   with `coat=` (a pencil or chalk, or a pastel or charcoal stick), then retraced with a
//!   stylus (`tool=`: a ball `tip=` mm across, or a pencil's point) pressed with `pressure=`
//!   through it. The sheet is a natural tracing paper (`through="tracing"`, the default: traced
//!   twice by hand) or the study itself, its own back coated (`through="study"`: traced once).
//!   The line is faint, as wide as the ball's contact, and holds what left the coat. With no
//!   coat the stylus is blind: its line goes into the surface's guide only.
//! - **`by="window"`**: the two sheets are held to a window, and the lines are drawn straight
//!   onto `onto` with `tool=` (a pencil or chalk) at `pressure=`. That makes a real line, the
//!   tool's own.
//!
//! `mirror=true` flips the tracing over first, as for a plate (the print comes out the right
//! way round). `at={x, y}` puts the study's middle at that point of `onto` (units; by default
//! its middle). Both surfaces lie at the painting's pixel pitch, so a line keeps its width
//! in mm.
//!
//! **The surfaces.** The study is read where it rests (a look: nothing about it changes).
//! `onto` is taken up for the trace if it isn't current, and the surface current before is
//! taken up again after it.

use crate::api::{S, check_keys, err, num, points, seed_of};
use crate::surfaces::{name_of, no_surface, take_up};
use mlua::{Lua, Result, Table, Value};
use paint::trace::{self, Coat, Hand, Layer};

const KEYS: &[&str] = &["onto", "by", "tool", "coat", "pressure", "force", "tip", "mirror", "through", "at", "level", "seed"];
const HELP: &str = "trace(study, {onto=<surface>, by=\"transfer\"|\"window\", coat=<pencil, chalk or stick>, tool=\"stylus\"|<pencil>, pressure=0..1, mirror=false, through=\"tracing\"|\"study\", at={x, y}, level=0.25}): the study's drawn lines traced onto another surface";
/// A hand's pressure 0..1 as its force on the point, N (the pencil's `p:line` on a pastel:
/// draw_pencil.rs, `FORCE_AT_FULL`).
const FORCE_AT_FULL: f32 = 5.0;

/// The switch's verb, installed beside the named surfaces' (session.rs `TRACE_MARK`).
pub(crate) fn install(lua: &Lua, st: S) -> Result<()> {
    lua.globals().set("trace", lua.create_function(move |lua, (study, o): (Value, Option<Table>)| {
        let o = match o {
            Some(o) => o,
            None => lua.create_table()?,
        };
        trace_verb(lua, &st, &study, &o)
    })?)
}

/// How the lines go onto the surface.
#[derive(Clone, Copy, PartialEq)]
enum By {
    /// A coated back retraced with a stylus, by way of a sheet: tracing paper (traced twice by
    /// hand) or the study itself (once).
    Transfer { through_study: bool },
    /// Held to a window and drawn straight on.
    Window,
}

/// What the traced lines laid, for the Lua table it returns.
#[derive(serde::Serialize, serde::Deserialize)]
struct Traced {
    lines: usize,
    length_mm: f32,
    width_mm: f32,
    moved_mm3: f64,
    coat_mm3: f64,
    secs: f32,
    wet: usize,
    worn: f32,
}

fn trace_verb(lua: &Lua, st: &S, study: &Value, o: &Table) -> Result<Table> {
    check_keys(o, KEYS, "trace")?;
    let from = name_of(study, "trace")?;
    let onto = match o.get::<Value>("onto")? {
        Value::Nil => st.borrow().surfaces.current_name().to_string(),
        v => name_of(&v, "trace onto")?,
    };
    let (to, was) = {
        let s = st.borrow();
        if s.canvas.is_none() && s.surfaces.main_current() {
            return Err(crate::api::no_canvas());
        }
        if s.surfaces.find(&from).is_none() {
            return Err(no_surface(&s.surfaces, "trace", &from));
        }
        let Some(to) = s.surfaces.find(&onto) else { return Err(no_surface(&s.surfaces, "trace onto", &onto)) };
        if from == onto {
            return err(format!("trace: the study and the surface it is traced onto are both {from:?}\n{HELP}"));
        }
        (to, s.surfaces.current)
    };
    let by = match o.get::<Option<String>>("by")?.as_deref() {
        None | Some("transfer") => {
            let through_study = match o.get::<Option<String>>("through")?.as_deref() {
                None | Some("tracing") => false,
                Some("study") => true,
                Some(t) => return err(format!("trace: through={t:?}: \"tracing\" (a tracing paper, traced from the study) or \"study\" (the study itself, its back coated)")),
            };
            By::Transfer { through_study }
        }
        Some("window") => {
            if o.get::<Value>("through")? != Value::Nil || o.get::<Value>("coat")? != Value::Nil || o.get::<Value>("tip")? != Value::Nil {
                return err("trace: at a window the lines are drawn straight on with tool=: no coat, tip or through");
            }
            By::Window
        }
        Some(b) => return err(format!("trace: by={b:?}: \"transfer\" (a coated back and a stylus) or \"window\" (drawn straight on, held to the light)")),
    };
    let pressure = num(o, "pressure")?.unwrap_or(0.5);
    if !(0.0..=1.0).contains(&pressure) {
        return err("trace: pressure is 0..1 (light 0.1, firm 0.5, heavy 1: 5 N)");
    }
    let force = match num(o, "force")? {
        Some(f) if !(0.0..=50.0).contains(&f) => return err("trace: force is the hand's on the stylus, N (0 to 50)"),
        Some(f) if by == By::Window => return err(format!("trace: at a window the tool draws at pressure=, not force= ({f} N)")),
        Some(f) => f,
        None => FORCE_AT_FULL * pressure,
    };
    let level = num(o, "level")?.unwrap_or(trace::LEVEL);
    if !(level > 0.0 && level <= 1.0) {
        return err("trace: level is the faintest line followed, as the guide reads it (0 to 1)");
    }
    let mirror = o.get::<Option<bool>>("mirror")?.unwrap_or(false);
    let at = match o.get::<Value>("at")? {
        Value::Nil => None,
        v => match points(&v)?.as_slice() {
            [p] => Some(*p),
            _ => return err("trace: at={x, y}: where the study's middle lies on the surface, in its units"),
        },
    };
    let seed = seed_of(st, o)?;
    // the tool: a stylus's ball, or a pencil's point
    let tool = o.get::<Value>("tool")?;
    let pencil = match &tool {
        Value::Table(t) if crate::api::draw_pencil::is_stick(st, t)? => return err("trace: tool= is a stylus or a pencil (or chalk); a pastel stick goes in coat="),
        Value::Table(t) => Some((t.clone(), crate::api::draw_pencil::lead_of(t)?)),
        Value::Nil => None,
        Value::String(s) if s.to_str()? == "stylus" => None,
        v => return err(format!("trace: tool= is \"stylus\" or a pencil, got {}", v.type_name())),
    };
    let worn: f32 = match &pencil {
        Some((t, _)) => t.get::<Option<f32>>("worn")?.unwrap_or(0.0),
        None => 0.0,
    };
    let tip_mm = match (num(o, "tip")?, &pencil) {
        (Some(_), Some(_)) => return err("trace: tip= is a stylus's ball; a pencil's point is its own"),
        (Some(t), None) if !(0.2..=5.0).contains(&t) => return err("trace: tip is the stylus's ball, mm across (0.2 to 5; tracing styluses are 1 to 3)"),
        (Some(t), None) => t,
        (None, Some((_, l))) => l.width_mm(worn),
        (None, None) => trace::STYLUS_MM,
    };
    if by == By::Window && pencil.is_none() {
        return err("trace: at a window the lines are drawn with tool= a pencil or chalk");
    }
    let coat = match o.get::<Value>("coat")? {
        Value::Nil => None,
        Value::Table(t) if crate::api::draw_pencil::is_stick(st, &t)? => Some(Coat::of_stick(&crate::api::draw_pencil::stick_of(&t)?)),
        Value::Table(t) => Some(Coat::of_lead(&crate::api::draw_pencil::lead_of(&t)?)),
        v => return err(format!("trace: coat= is what the back is rubbed with: a pencil, chalk or a stick, got {}", v.type_name())),
    };
    // the surface traced onto is taken up, and the one current before after
    if to != was {
        if st.borrow().canvas.as_ref().is_some_and(|c| c.has_sheet()) {
            return err("trace: a sheet of paper lies on the picture; lift it first (lift_sheet())");
        }
        take_up(st, to)?;
    }
    let lead = pencil.as_ref().map(|p| p.1.clone());
    let traced = crate::time::verb(st, crate::time::Verb::Pass, |s| {
        // the study, where it rests (read, not changed)
        let i = s.surfaces.find(&from).expect("the study is a surface");
        let packed = s.surfaces.slots[i].rest.as_ref().and_then(|r| r.canvas.clone()).ok_or_else(|| mlua::Error::runtime(format!("trace: {from:?} has no canvas")))?;
        let (guide, s_px_mm, s_paper) = {
            let c = packed.restore().map_err(|e| mlua::Error::runtime(format!("trace: the study couldn't be read: {e}")))?;
            (c.drawing_guide(), c.px_mm(), c.paper().copied())
        };
        let c = s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?;
        let back_paper = match by {
            By::Transfer { through_study: true } => match s_paper {
                Some(p) => Some(p),
                None => return err(format!("trace: {from:?} is a canvas, not a sheet: its back can't be coated (trace it by way of tracing paper)")),
            },
            By::Transfer { through_study: false } => Some(trace::tracing_paper(seed)),
            By::Window => None,
        };
        if by == By::Window && (c.paper().is_none() || s_paper.is_none()) {
            return err("trace: only sheets of paper show a drawing through them at a window");
        }
        let (paths, w_study) = trace::guide_paths(&guide, level);
        let f = c.window();
        let (t_px_mm, scale) = (c.px_mm(), f.scale);
        let k = s_px_mm / t_px_mm;
        let (sw, sh) = (guide.f.w as f32, guide.f.h as f32);
        let (cx, cy) = match at {
            Some((x, y)) => (x * scale, y * scale),
            None => (f.full_w as f32 / 2.0, f.full_h as f32 / 2.0),
        };
        let place = |(x, y): (f32, f32)| (cx + ((if mirror { sw - x } else { x }) - sw / 2.0) * k, cy + (y - sh / 2.0) * k);
        let back = (cx - sw * k / 2.0, cy - sh * k / 2.0, cx + sw * k / 2.0, cy + sh * k / 2.0);
        let hand = Hand { w_px: w_study * k, generations: if by == (By::Transfer { through_study: false }) { 2 } else { 1 } };
        let mut followed = Vec::with_capacity(paths.len());
        let (mut secs, mut length_px) = (0.0f32, 0.0f32);
        for (n, p) in paths.iter().enumerate() {
            let q: Vec<(f32, f32)> = p.iter().map(|&pt| place(pt)).collect();
            let len = paint::path::arclen(&q).last().copied().unwrap_or(0.0);
            secs += hand.secs(len);
            length_px += len;
            followed.push(hand.follow(&q, seed.wrapping_add(n as u64 * 0x9E37)));
        }
        let mut out = Traced { lines: paths.len(), length_mm: length_px * t_px_mm, width_mm: 0.0, moved_mm3: 0.0, coat_mm3: 0.0, secs, wet: 0, worn };
        match (by, back_paper, &lead) {
            (By::Window, _, Some(lead)) => {
                let p = vec![pressure];
                for (n, q) in followed.iter().enumerate() {
                    let pts: Vec<(f32, f32)> = q.iter().map(|&(x, y)| (x / scale, y / scale)).collect();
                    if pts.len() < 2 {
                        continue;
                    }
                    let s_along = paint::path::arclen(&pts);
                    let mark = paint::graphite::Mark { pressure: paint::graphite::pressure_along(&p, &s_along), pts };
                    out.worn += c.draw(lead, &mark, out.worn, seed.wrapping_add(0x51A7 + n as u64));
                }
                out.width_mm = lead.width_mm(out.worn);
            }
            (_, Some(bp), _) => {
                let mut stack = vec![Layer::of(&bp)];
                if let Some(tp) = c.paper() {
                    stack.push(Layer::of(tp));
                }
                let contact = trace::contact(tip_mm / 2.0, force, &stack);
                let laid = c.trace_transfer(&followed, back, contact, coat.map(|ct| (ct, bp.pore_um())));
                out.width_mm = contact.width_mm();
                out.moved_mm3 = laid.moved_mm3;
                out.coat_mm3 = laid.coat_mm3;
                out.wet = laid.wet;
            }
            _ => unreachable!("a window trace has its pencil, a transfer its back"),
        }
        c.tally_mut().secs += secs as f64;
        let what = match (by, coat) {
            (By::Window, _) => "drawn at the window".to_string(),
            (_, None) => "a blind stylus: the guide only".to_string(),
            (_, Some(_)) => format!("{} mm³ of the coat laid, {} mm³ held", fmt3(out.moved_mm3), fmt3(out.coat_mm3)),
        };
        s.out.push_str(&format!(
            "traced {} line{} ({} mm) of {from:?} onto {onto:?}{}: {} mm wide, {what}, {} s\n",
            out.lines,
            if out.lines == 1 { "" } else { "s" },
            crate::api::fmt_num(out.length_mm),
            if mirror { ", mirrored" } else { "" },
            crate::api::fmt_num((out.width_mm * 100.0).round() / 100.0),
            crate::api::fmt_num(out.secs.round()),
        ));
        if out.wet > 0 {
            s.out.push_str(&format!("  ({} px of the lines fell on wet paint and took nothing)\n", out.wet));
        }
        Ok(out)
    })?;
    if let Some((t, _)) = &pencil {
        if by == By::Window {
            t.set("worn", traced.worn)?;
        }
    }
    if to != was {
        take_up(st, was)?;
    }
    let r = lua.create_table()?;
    r.set("lines", traced.lines)?;
    r.set("length", traced.length_mm)?;
    r.set("width", traced.width_mm)?;
    r.set("moved", traced.moved_mm3)?;
    r.set("coat", traced.coat_mm3)?;
    r.set("seconds", traced.secs)?;
    Ok(r)
}

fn fmt3(v: f64) -> String {
    format!("{v:.3}")
}

#[cfg(test)]
mod tests;
