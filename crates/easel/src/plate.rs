//! Plates among the named surfaces (`--@ plate`; notes/plan-intaglio.md §2–§3, phase P2).
//!
//! `plate{name=, metal=, temper=, size={w, h}, thickness=, bevel=}` makes a copper or zinc
//! plate beside the painting, a surface of its own (surfaces.rs), at the painting's pitch: the
//! sheet it will print on lies at that pitch (plan-intaglio I1). Its state is paint's
//! `plate::Plate`, a statistical plate: per cell the openings binned by width with their
//! depth, the burr and its side, the surface's level; and the tools' paths, from which a crop
//! is drawn again at its own scale. Its canvas holds nothing but its clock and the hand time
//! spent on it.
//!
//! A plate in hand (`use(k)`) is worked with its own tools, from the physics of
//! notes/research/intaglio_physics.md and plate_physics.md:
//! - `n = needle{tip=, cone=}` and `k:drypoint(n, pts, {force=, alt=, azimuth=, speed=})`
//! - `k:burnish(pts, {force=, radius=, passes=, speed=})`
//! - `k:scrape(pts, {force=, width=, passes=, speed=})`
//! - `k:roulette(pts, {pitch=, tooth=, cone=, force=, speed=})`
//! - `k:feel(x, y)`, what a fingernail finds there.
//!
//! Paint, pencil and pastel are refused on it (time.rs `verb_plate`). `look --surface <plate>`
//! shows it as metal (Fresnel reflection from its optical constants), its relief lit by the
//! light of `--mode relief` or `--light`; `probe` gives its lines' width and depth and its
//! burr. No ink, wiping or press yet (phase P3), which read the cells' openings and burr.

use crate::api::{S, check_keys, err, fmt_num, num};
use crate::surfaces::{Kind, Own, Resting, Slot, SurfaceU};
use mlua::{Lua, Result, Table, UserData, UserDataFields, UserDataMethods, Value};
use paint::plate::{Metal, Op, Plate, Point, Report, Temper};
use std::rc::Rc;

/// The needle's point radius when none is given (µm) [E: IP §1.3's 10–50 µm for a needle
/// sharpened on a stone, its middle; no measurement exists].
pub const NEEDLE_TIP_UM: f32 = 25.0;
/// The needle's cone half-angle when none is given (degrees): IP §1.2's drypoint needle,
/// 15–30°, at the blunter end a needle of steel is ground to.
pub const NEEDLE_CONE_DEG: f32 = 30.0;
/// The hand's speed with a drypoint needle (mm/s) [E: plan-intaglio §3.1's 20–40 mm/s, its
/// middle: it is hard work, slower than a pencil]; the burnisher and scraper take it too.
pub const TOOL_MM_S: f32 = 30.0;

const PLATE_HELP: &str = "plate{name=\"<name>\", metal=\"copper\" | \"zinc\", temper=\"annealed\" | \"eighth-hard\" | \"half-hard\" (copper), size={<width mm>, <height mm>}, thickness=<mm>, bevel=<degrees>}: a plate beside the painting (use(k) to work it)";
const NEEDLE_HELP: &str = "needle{tip=<point radius µm>, cone=<half-angle degrees>}: a drypoint needle";

/// A drypoint needle: its point's radius (µm) and its cone's half-angle (degrees).
#[derive(Clone, Copy)]
pub struct NeedleU {
    pub tip: f32,
    pub cone: f32,
}

impl UserData for NeedleU {
    fn add_fields<F: UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("tip", |_, n| Ok(n.tip));
        f.add_field_method_get("cone", |_, n| Ok(n.cone));
    }
}

/// The switch's verbs: `plate{}` and `needle{}` (a plate's tools are its methods: `methods`).
pub(crate) fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    {
        let st = st.clone();
        g.set("plate", lua.create_function(move |_, o: Option<Table>| make(&st, o))?)?;
    }
    g.set("needle", lua.create_function(|_, o: Option<Table>| {
        let mut n = NeedleU { tip: NEEDLE_TIP_UM, cone: NEEDLE_CONE_DEG };
        if let Some(o) = o {
            check_keys(&o, &["tip", "cone"], "needle")?;
            if let Some(t) = num(&o, "tip")? {
                n.tip = t;
            }
            if let Some(c) = num(&o, "cone")? {
                n.cone = c;
            }
        }
        if !(0.0..=200.0).contains(&n.tip) {
            return err(format!("needle: tip is the point's radius in µm, 0 to 200 (a sharpened needle 10–50)\n{NEEDLE_HELP}"));
        }
        if !(5.0..=60.0).contains(&n.cone) {
            return err(format!("needle: cone is the half-angle of its point in degrees, 5 to 60 (a drypoint needle 15–30)\n{NEEDLE_HELP}"));
        }
        Ok(n)
    })?)?;
    Ok(())
}

/// The metal and temper a `plate{}` names, from the period box (plan-intaglio I5: an 1890s
/// Paris print box has copper and zinc; steel facing comes later).
fn metal_of(o: &Table) -> Result<(Metal, Temper)> {
    let metal = match o.get::<Option<String>>("metal")?.as_deref() {
        Some("copper") => Metal::Copper,
        Some("zinc") => Metal::Zinc,
        Some(m) => return err(format!("plate: metal={m:?}: an 1890s Paris print box has copper and zinc plates (steel facing is put on a copper plate later)")),
        None => return err(format!("plate: metal= (\"copper\" or \"zinc\")\n{PLATE_HELP}")),
    };
    let temper = match (metal, o.get::<Option<String>>("temper")?.as_deref()) {
        // (printmakers' copper is eighth- to half-hard: IP §1.1)
        (Metal::Copper, None | Some("half-hard")) => Temper::HalfHard,
        (Metal::Copper, Some("eighth-hard")) => Temper::EighthHard,
        (Metal::Copper, Some("annealed")) => Temper::Annealed,
        (Metal::Zinc, None | Some("rolled")) => Temper::Rolled,
        (_, Some(t)) => return err(format!("plate: temper={t:?}: copper is \"annealed\", \"eighth-hard\" or \"half-hard\"; zinc is sold rolled")),
    };
    Ok((metal, temper))
}

fn make(st: &S, o: Option<Table>) -> Result<SurfaceU> {
    let o = o.ok_or_else(|| mlua::Error::runtime(PLATE_HELP))?;
    check_keys(&o, &["name", "metal", "temper", "size", "thickness", "bevel"], "plate")?;
    let name: String = o.get::<Option<String>>("name")?.ok_or_else(|| mlua::Error::runtime(format!("plate: name= (what the plate is called: use(\"<name>\"), look --surface <name>)\n{PLATE_HELP}")))?;
    crate::surfaces::check_new(&st.borrow(), "plate", &name)?;
    let (metal, temper) = metal_of(&o)?;
    let (wmm, hmm) = crate::api::pair(&o, "size")?.ok_or_else(|| mlua::Error::runtime(format!("plate: size={{width, height}} in mm\n{PLATE_HELP}")))?;
    if !(20.0..=1000.0).contains(&wmm) || !(20.0..=1000.0).contains(&hmm) {
        return err("plate: size is {width, height} in mm, each 20 to 1000");
    }
    // (the plan's example plate: 1.3 mm, its edge filed at 45°, IP §6.3)
    let thickness = num(&o, "thickness")?.unwrap_or(1.3);
    let bevel = num(&o, "bevel")?.unwrap_or(45.0);
    if !(0.3..=5.0).contains(&thickness) {
        return err("plate: thickness is mm, 0.3 to 5");
    }
    if !(10.0..=90.0).contains(&bevel) {
        return err("plate: bevel is the edge's filed angle in degrees, 10 to 90");
    }
    let tubes = st.borrow().tubes.clone();
    // laid out in mm at the painting's pitch, as a sheet (plan-intaglio I1)
    let (width, painting_mm) = {
        let s = st.borrow();
        let painting = if s.surfaces.main_current() { s.style.clone() } else { s.surfaces.slots[0].rest.as_ref().and_then(|r| r.own.style.clone()) };
        (s.width, painting.map(|p| p.width_mm).ok_or_else(crate::api::no_canvas)?)
    };
    let px = ((width as f64 * wmm as f64 / painting_mm as f64).round() as usize).max(16);
    let most = crate::session::WIDTHS[1];
    if px > most {
        return err(format!("plate: {wmm} mm at the painting's {} mm a pixel is {px} px across; a plate is at most {most} px", fmt_num(painting_mm / width as f32)));
    }
    let aspect = wmm / hmm;
    let mut c = paint::Canvas::new_window(px, aspect, Plate::bare_rgb(metal), None).with_engine(tubes.engine).with_size_mm(wmm).with_drain(tubes.drain).with_oils(tubes.oils);
    c.set_hand_time(Some(crate::time::SLICE_MIN));
    let mut plate = Plate::new(metal, temper, wmm, hmm, thickness, bevel, px, c.window().h);
    // (the contact laws of a painting begun with them: kept with the plate, `--@ intaglio-contact`)
    plate.contact = tubes.plate_contact;
    let h = c.height();
    let packed = Rc::new(crate::surfaces::Packed::rest(&c).map_err(|e| mlua::Error::runtime(format!("plate: the plate couldn't be put aside: {e}")))?);
    let recording_ground = crate::live_frames::ground_signature(&c);
    let clock = c.clock();
    drop(c);
    let mut s = st.borrow_mut();
    let own = Own {
        style: Some(Rc::new(paint::Style { name: "oil", width_mm: wmm, ..paint::Style::oil_with((*tubes).clone()) })),
        setup: Some(format!("plate {name:?}: {} {}, {} x {} mm, {} mm thick", metal.name(), temper.name(), fmt_num(wmm), fmt_num(hmm), fmt_num(thickness))),
        canvas_src: Some(format!("plate{}", crate::api::lua_src(&Value::Table(o.clone()))?)),
        recording_ground,
        clock0: clock - s.clock,
        view: None,
        plate: Some(Rc::new(plate)),
    };
    crate::surfaces::name_the_painting(&mut s);
    let since = s.clock;
    s.surfaces.slots.push(Slot { name: name.clone(), kind: Kind::Plate, rest: Some(Resting { own, canvas: Some(packed), since }) });
    Ok(SurfaceU { name, kind: Kind::Plate, size_mm: wmm, h })
}

/// A number or a list of them along `n` points (interpolated by index), as pastel takes them.
fn along(o: Option<&Table>, key: &str, n: usize, default: f32) -> Result<Vec<f32>> {
    let Some(o) = o else { return Ok(vec![default; n]) };
    let v: Vec<f32> = match o.get::<Value>(key)? {
        Value::Nil => return Ok(vec![default; n]),
        Value::Integer(v) => vec![v as f32],
        Value::Number(v) => vec![v as f32],
        Value::Table(t) => t.sequence_values::<f32>().collect::<Result<_>>()?,
        o => return err(format!("{key}: want a number or a list, got {}", o.type_name())),
    };
    if v.is_empty() || v.iter().any(|x| !x.is_finite()) {
        return err(format!("{key}: a number, or a list of them along the stroke"));
    }
    Ok((0..n)
        .map(|i| {
            if v.len() == 1 || n == 1 {
                return v[0];
            }
            let u = i as f32 / (n - 1) as f32 * (v.len() - 1) as f32;
            let k = (u.floor() as usize).min(v.len() - 2);
            v[k] + (v[k + 1] - v[k]) * (u - k as f32)
        })
        .collect())
}

/// The plate `k` names, in hand and out of any bath (a plate lying in its acid is lifted out
/// before it is worked or felt: etch.rs): (its width in mm, its height in units).
pub(crate) fn in_hand(st: &S, k: &SurfaceU, verb: &str) -> Result<(f32, f32)> {
    let r = held(st, k, verb)?;
    let s = st.borrow();
    if let (Some(p), Some(c)) = (s.plate.as_ref(), s.canvas.as_ref())
        && let Some(e) = crate::etch::refuse_in_bath(p, c.clock(), verb, &k.name)
    {
        return Err(mlua::Error::runtime(e));
    }
    Ok(r)
}

/// The plate `k` names, in hand, in a bath or not: (its width in mm, its height in units).
pub(crate) fn held(st: &S, k: &SurfaceU, verb: &str) -> Result<(f32, f32)> {
    let s = st.borrow();
    if k.kind != Kind::Plate || !s.tubes.plate {
        return err(format!("{verb}: a plate's tool, and {:?} is a {}", k.name, k.kind.name()));
    }
    if s.surfaces.current_name() != k.name || s.plate.is_none() {
        return err(format!("{verb}: the plate {:?} isn't in hand: use({:?}) first", k.name, k.name));
    }
    Ok((k.size_mm, k.h))
}

/// A tool's path in units into plate points (mm), with its options along it.
pub(crate) fn path(st: &S, k: &SurfaceU, verb: &str, pts: &Value, o: Option<&Table>, force: f32, max_force: f32, speed: f32) -> Result<Vec<Point>> {
    let (mm, h) = in_hand(st, k, verb)?;
    let pts = crate::api::points(pts)?;
    if pts.len() < 2 {
        return err(format!("{verb}: needs at least two points"));
    }
    if pts.iter().any(|&(x, y)| !(0.0..=1000.0).contains(&x) || !(0.0..=h).contains(&y)) {
        return err(format!("{verb}: the points lie on the plate: x 0 to 1000, y 0 to {}", fmt_num(h)));
    }
    let n = pts.len();
    let f = along(o, "force", n, force)?;
    let alt = along(o, "alt", n, 55.0)?;
    let az = along(o, "azimuth", n, 45.0)?;
    let speed = along(o, "speed", n, speed)?;
    if f.iter().any(|v| !(0.0..=max_force).contains(v)) {
        return err(format!("{verb}: force is newtons, 0 to {max_force}"));
    }
    if alt.iter().any(|v| !(0.0..=90.0).contains(v)) {
        return err(format!("{verb}: alt is the tool's angle to the plate, 0 to 90 degrees"));
    }
    if speed.iter().any(|v| !(1.0..=500.0).contains(v)) {
        return err(format!("{verb}: speed is mm/s, 1 to 500"));
    }
    let u = mm / 1000.0;
    Ok((0..n).map(|i| Point { x: pts[i].0 * u, y: pts[i].1 * u, force: f[i], alt: alt[i], az: az[i], speed: speed[i] }).collect())
}

fn int_opt(o: Option<&Table>, key: &str, default: u32, most: u32) -> Result<u32> {
    let v = o.map(|o| num(o, key)).transpose()?.flatten().unwrap_or(default as f32);
    if v.fract() != 0.0 || !(1.0..=most as f32).contains(&v) {
        return err(format!("{key}: a whole number, 1 to {most}"));
    }
    Ok(v as u32)
}

pub(crate) fn opt(o: Option<&Table>, key: &str, default: f32, lo: f32, hi: f32, what: &str) -> Result<f32> {
    let v = o.map(|o| num(o, key)).transpose()?.flatten().unwrap_or(default);
    if !(lo..=hi).contains(&v) {
        return err(format!("{key}: {what}, {lo} to {hi}"));
    }
    Ok(v)
}

/// A tool's report as it is recorded (replay checkpoints take a verb's result from the trace).
pub(crate) type Rep = (Vec<f64>, bool, bool);

fn rep_of(r: &Report) -> Rep {
    (vec![r.width_um, r.depth_um, r.burr_um, r.beta_deg, r.groove_um3, r.burr_um3, r.chip_um3, r.flattened_um3, r.length_mm, r.secs, r.pressure_mpa, r.scrape_um], r.cut, r.ploughed)
}

/// Apply `op` to the plate in hand, its hand time on the plate's clock. A plate under a coat
/// (a ground, varnish, rosin or a bite's state, `--@ etch`) is cleaned first.
fn apply(st: &S, op: Op) -> Result<Rep> {
    crate::time::verb_plate(st, crate::time::Verb::Pass, move |s| {
        let plate = Rc::make_mut(s.plate.as_mut().ok_or_else(|| mlua::Error::runtime("no plate in hand"))?);
        if plate.coated() {
            return Err(mlua::Error::runtime("the plate has a ground, varnish, rosin or the acid's work on it: clean it first (k:clean())"));
        }
        crate::ink::refuse_inked(plate).map_err(mlua::Error::runtime)?;
        let r = plate.apply(op);
        let c = s.canvas.as_mut().ok_or_else(crate::api::no_canvas)?;
        c.tally_mut().secs += r.secs;
        Ok(rep_of(&r))
    })
}

/// What a drypoint that cut a chip says: how much of its groove left as chip,
/// and the attack angle only when all of it did (`beta` is the widest
/// mark's, which may have ploughed where another part of the line cut).
fn chip_words(r: &Rep) -> String {
    let crit = fmt_num(paint::plate::BETA_C_DEG as f32);
    if !r.2 {
        return format!(
            "drypoint: the needle cut a chip all along (its face met the metal at {}°, past its {crit}°: held upright with a fine point, or pushed): a clean line with no burr\n",
            fmt_num(r.0[3].round() as f32)
        );
    }
    let share = if r.0[4] > 0.0 { r.0[6] / r.0[4] } else { 0.0 };
    let pct = if share < 0.01 { "under 1".to_string() } else { fmt_num((100.0 * share).round() as f32) };
    format!("drypoint: the needle cut a chip from {pct}% of the groove (where its face met the metal steeper than {crit}°: held upright or pushed, or the middle of a fine point) and ploughed the rest into burr\n")
}

fn table(lua: &Lua, r: &Rep) -> Result<Table> {
    let t = lua.create_table()?;
    let names = ["width", "depth", "burr", "beta", "groove", "burr_volume", "chip", "flattened", "length", "seconds", "pressure", "scraped"];
    for (k, v) in names.iter().zip(&r.0) {
        t.set(*k, *v)?;
    }
    t.set("cut", r.1)?;
    t.set("ploughed", r.2)?;
    Ok(t)
}

/// A plate's methods on its surface's userdata (surfaces.rs `SurfaceU`): its tools and `feel`.
pub(crate) fn methods<M: UserDataMethods<SurfaceU>>(m: &mut M) {
    // k:drypoint(n, pts, {force=, alt=, azimuth=, speed=}): the needle drawn along pts
    m.add_method("drypoint", |lua, k, (n, pts, o): (Value, Value, Option<Table>)| {
        let st = crate::api::studio(lua)?;
        let Value::UserData(n) = n else { return err(format!("drypoint: the needle first: k:drypoint(n, pts, {{...}})\n{NEEDLE_HELP}")) };
        let n = *n.borrow::<NeedleU>().map_err(|_| mlua::Error::runtime(format!("drypoint: the needle first\n{NEEDLE_HELP}")))?;
        if let Some(o) = &o {
            check_keys(o, &["force", "alt", "azimuth", "speed"], "drypoint")?;
        }
        // (a hand presses a needle with 0.5–5 N, IP §1.3)
        let pts = path(&st, k, "drypoint", &pts, o.as_ref(), 2.0, 20.0, TOOL_MM_S)?;
        let r = apply(&st, Op::Drypoint { tip_um: n.tip, cone_deg: n.cone, pts })?;
        if r.1 {
            st.borrow_mut().out.push_str(&chip_words(&r));
        }
        table(lua, &r)
    });
    // k:burnish(pts, {force=, radius=, passes=, speed=}): a burnisher rubbed along pts
    m.add_method("burnish", |lua, k, (pts, o): (Value, Option<Table>)| {
        let st = crate::api::studio(lua)?;
        if let Some(o) = &o {
            check_keys(o, &["force", "radius", "passes", "speed"], "burnish")?;
        }
        // (a rounded burnisher of 5–20 mm at a hand's 10–50 N: IP §1.7)
        let radius = opt(o.as_ref(), "radius", 10.0, 1.0, 50.0, "the burnisher's radius in mm")?;
        let passes = int_opt(o.as_ref(), "passes", 3, 1000)?;
        let pts = path(&st, k, "burnish", &pts, o.as_ref(), 30.0, 200.0, TOOL_MM_S)?;
        let r = apply(&st, Op::Burnish { radius_mm: radius, passes, pts })?;
        table(lua, &r)
    });
    // k:scrape(pts, {force=, width=, passes=, speed=}): a scraper drawn along pts, cutting
    m.add_method("scrape", |lua, k, (pts, o): (Value, Option<Table>)| {
        let st = crate::api::studio(lua)?;
        if let Some(o) = &o {
            check_keys(o, &["force", "width", "passes", "speed"], "scrape")?;
        }
        let width = opt(o.as_ref(), "width", 8.0, 0.5, 40.0, "the blade's width on the plate in mm")?;
        let passes = int_opt(o.as_ref(), "passes", 1, 1000)?;
        let pts = path(&st, k, "scrape", &pts, o.as_ref(), 5.0, 100.0, TOOL_MM_S)?;
        let r = apply(&st, Op::Scrape { width_mm: width, passes, pts })?;
        table(lua, &r)
    });
    // k:roulette(pts, {pitch=, tooth=, cone=, force=, speed=}): a toothed wheel rolled along pts
    m.add_method("roulette", |lua, k, (pts, o): (Value, Option<Table>)| {
        let st = crate::api::studio(lua)?;
        if let Some(o) = &o {
            check_keys(o, &["pitch", "tooth", "cone", "force", "speed"], "roulette")?;
        }
        let pitch = opt(o.as_ref(), "pitch", 0.2, 0.05, 5.0, "mm between the teeth")?;
        let tooth = opt(o.as_ref(), "tooth", 30.0, 0.0, 300.0, "a tooth's point radius in µm")?;
        // [E] a tooth's point: a roulette's teeth are cut as small pyramids; 45° until measured
        let cone = opt(o.as_ref(), "cone", 45.0, 5.0, 80.0, "a tooth's half-angle in degrees")?;
        let pts = path(&st, k, "roulette", &pts, o.as_ref(), 3.0, 50.0, TOOL_MM_S)?;
        let r = apply(&st, Op::Roulette { pitch_mm: pitch, tooth_um: tooth, cone_deg: cone, pts })?;
        table(lua, &r)
    });
    // k:feel(x, y): what a fingernail finds there, in words and numbers
    m.add_method("feel", |lua, k, (x, y): (f32, f32)| {
        let st = crate::api::studio(lua)?;
        let (mm, h) = in_hand(&st, k, "feel")?;
        if !(0.0..=1000.0).contains(&x) || !(0.0..=h).contains(&y) {
            return err(format!("feel: x 0 to 1000, y 0 to {} on the plate", fmt_num(h)));
        }
        let (nums, words) = crate::time::read("plate feel", || {
            let s = st.borrow();
            let p = s.plate.as_ref().ok_or_else(|| mlua::Error::runtime("no plate in hand"))?;
            let c = p.cell_at_mm((x * mm / 1000.0) as f64, (y * mm / 1000.0) as f64).unwrap_or_default();
            let (mut nums, mut words) = feel(&c);
            // under a coat (`--@ etch`): the ground, the varnish, the rosin
            if p.coats().is_some() {
                let (cx, cy) = (((x * mm / 1000.0) as f64 * 1000.0 / p.pitch_um()) as usize, ((y * mm / 1000.0) as f64 * 1000.0 / p.pitch_um()) as usize);
                let (more, n) = crate::etch::feel(p, cx, cy);
                words = if words == "smooth" && !more.is_empty() { more } else if more.is_empty() { words } else { format!("{more}; under it {words}") };
                nums.extend(n);
            }
            // ink on it (`--@ press`)
            if p.inking().is_some() {
                let (cx, cy) = (((x * mm / 1000.0) as f64 * 1000.0 / p.pitch_um()) as usize, ((y * mm / 1000.0) as f64 * 1000.0 / p.pitch_um()) as usize);
                let (more, n) = crate::ink::feel(p, cx, cy);
                if !more.is_empty() {
                    words = if words == "smooth" { more } else { format!("{more}; under it {words}") };
                }
                nums.extend(n);
            }
            Ok((nums, words))
        })?;
        let t = lua.create_table()?;
        // (the ground and rosin only on a coated plate: the ink's after whichever came before)
        let names: &[&str] = if nums.len() >= 10 { &["depth", "width", "open", "burr", "level", "ground", "rosin", "ink", "film", "cure"] } else if nums.len() == 8 { &["depth", "width", "open", "burr", "level", "ink", "film", "cure"] } else { &["depth", "width", "open", "burr", "level", "ground", "rosin"] };
        for (k, v) in names.iter().zip(&nums) {
            t.set(*k, *v)?;
        }
        t.set("words", words)?;
        Ok(t)
    });
    // the etching tools (etch.rs, `--@ etch`)
    crate::etch::methods(m);
    // ink and wiping (ink.rs, `--@ press`)
    crate::ink::methods(m);
}

/// A cell as a fingernail finds it: (mean depth µm, mean width µm, open share, burr height µm,
/// level µm) and in words.
fn feel(c: &paint::plate::Cell) -> (Vec<f64>, String) {
    let (d, w) = c.mean_depth_width();
    // (a V-groove's mean depth is half its depth)
    let deep = 2.0 * d;
    let (open, burr, level) = (c.open_share(), c.burr_height(), c.level);
    let mut words = Vec::new();
    if open > 0.0 && deep >= 1.0 {
        words.push(format!("a line about {} µm wide and {} µm deep", w.round(), deep.round()));
    }
    if burr >= 1.0 {
        words.push(format!("burr standing {} µm", burr.round()));
    }
    if level <= -1.0 {
        words.push(format!("a shallow dish {} µm down", (-level).round()));
    } else if level >= 1.0 {
        words.push(format!("metal pressed flat, {} µm proud", level.round()));
    }
    let words = if words.is_empty() { "smooth".to_string() } else { words.join(", ") };
    (vec![deep as f64, w as f64, open as f64, burr as f64, level as f64], words)
}

/// A look at a plate: its colour in the room's light, or lit on its relief, whole at its own
/// pitch or a crop drawn again from its vectors at the crop's (`Plate::rebuilt`).
/// A plate still in its bath is seen as it would come out now (`Plate::as_lifted`); it stays in.
pub(crate) fn render(c: &paint::Canvas, p: &Plate, v: &crate::look::View) -> std::result::Result<(usize, usize, Vec<u8>), String> {
    let lifted;
    let p = if p.in_bath().is_some() {
        lifted = p.as_lifted(c.clock());
        &lifted
    } else {
        p
    };
    let f = c.window();
    let (w, h) = p.size_px();
    if (f.w, f.h) != (w, h) {
        return Err("the plate's canvas isn't its size".into());
    }
    match v.crop {
        None => crate::look::render_pixels(f, v, &p.seen((0, 0, w, h), v.light)),
        Some([a, b, cc, d]) => {
            let k = ((crate::look::LOOK_SCALE / f.scale).ceil() as usize).clamp(1, 8);
            let s = f.scale * k as f32;
            let (fw, fh) = (w * k, h * k);
            let px = |u: f32, n: usize, m: i64| (((u * s).floor() as i64 + m).clamp(0, n as i64)) as usize;
            let (x0, x1) = (px(a.min(cc), fw, -2), px(a.max(cc), fw, 3));
            let (y0, y1) = (px(b.min(d), fh, -2), px(b.max(d), fh, 3));
            if x1 <= x0 || y1 <= y0 {
                return Err("--crop is empty or outside the plate".into());
            }
            let fine = p.rebuilt(k, (x0, y0, x1, y1));
            let seen = fine.seen((x0, y0, x1, y1), v.light);
            let frame = paint::Frame { w: x1 - x0, h: y1 - y0, scale: s, x0, y0, full_w: fw, full_h: fh };
            crate::look::render_pixels(frame, v, &seen)
        }
    }
}

/// `probe` on a plate over cells (x0, y0, x1, y1), end-exclusive: its lines, burr and level; a
/// plate in its bath as it would come out at `now` (its clock), and the bath.
pub(crate) fn probe(p: &Plate, now: f64, place: &str, r: (usize, usize, usize, usize)) -> String {
    let bath = crate::etch::bath_line(p, now);
    let lifted;
    let p = if p.in_bath().is_some() {
        lifted = p.as_lifted(now);
        &lifted
    } else {
        p
    };
    let (x0, y0, x1, y1) = r;
    let n = (x1 - x0) * (y1 - y0);
    let (mut open, mut vol, mut deepest, mut widest, mut burr, mut tallest, mut level) = (0.0f64, 0.0f64, 0.0f32, 0.0f32, 0.0f64, 0.0f32, 0.0f64);
    for y in y0..y1 {
        for x in x0..x1 {
            let c = p.cell(x, y);
            open += c.open_share() as f64;
            vol += c.groove_vol() as f64;
            for (_, w, d) in c.openings() {
                deepest = deepest.max(2.0 * d);
                widest = widest.max(w);
            }
            burr += c.burr_area as f64;
            tallest = tallest.max(c.burr_height());
            level += c.level as f64;
        }
    }
    let nf = n as f64;
    let what = if n == 1 { place.to_string() } else { format!("{place}, over {} x {} cells", x1 - x0, y1 - y0) };
    let mut out = format!(
        "{what}\n{} plate, {} HV\nopen {}% · lines up to {} µm wide and {} µm deep · {} µm³ of openings a mm²\nburr over {}%, up to {} µm high\nlevel {} µm\n",
        p.metal.name(),
        fmt_num(p.vickers() as f32),
        (100.0 * open / nf).round(),
        widest.round(),
        deepest.round(),
        (vol / nf * 1e6).round(),
        (100.0 * burr / nf).round(),
        tallest.round(),
        fmt_num(((level / nf) * 10.0).round() as f32 / 10.0)
    );
    // its coats and bites (`--@ etch`)
    if p.coats().is_some() {
        out.push_str(&crate::etch::probe(p, r));
    }
    // its ink (`--@ press`), aged to its clock
    if let Some(k) = p.inking() {
        out.push_str(&crate::ink::probe(p, r, k.aged_to));
    }
    out.push_str(&bath);
    out
}

#[cfg(test)]
mod tests;
