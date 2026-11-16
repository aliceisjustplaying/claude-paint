//! Several named surfaces in one session (`--@ surfaces`; notes/plan-intaglio.md §1, phase P1).
//!
//! A painting's session holds its canvas, `"painting"`, made by `canvas{}`. With the switch it
//! may make further surfaces beside it: `sheet{name=, size=, aspect=, paper={...}}`, a sheet of
//! paper of its own size and make (a study to trace from, a proof to print on, later a plate).
//! They live in the one Lua state and on the one clock: a printer moves between them, and the
//! minutes that pass at one of them pass for all.
//!
//! **One is current.** Every drawing verb acts on the current surface, as it acted on the canvas;
//! `use(s)` (or `use("name")`) makes another current and `use("painting")` goes back. A session
//! that never names a surface has only the painting, current, and runs exactly as before.
//!
//! **What a surface keeps of its own.** Its canvas (paint, drawing, pastel, paper, the hand time
//! counted on it), its style, its `canvas{}`/`sheet{}` setup, its world view and its clock's
//! offset. Shared by all: the studio clock, the hand (brushes, knives, rags, the palette board
//! and its piles) and the session's randomness.
//!
//! **Time passes for all of them.** The current surface is aged by hand time and `wait` as the
//! canvas always was. One put aside (`Resting`) keeps the studio clock it was put aside at
//! (`since`); when it is taken up again (`use`) it waits out the time since, all at once, as
//! paint left on a sheet does: `Canvas::wait`, the drying model of time passing with the hand
//! away, whose result is the same however the wait is split by what happens elsewhere. A look at
//! a resting surface sees a copy of it waited up to now (`Session::show`) and leaves the surface
//! as it was, so a look changes nothing a replay of the log wouldn't.
//!
//! **Memory (PR #311's one canvas at a time).** Only the current surface's canvas is in memory.
//! Every other rests packed (`Packed`: its checkpoint bytes compressed in an anonymous temporary
//! file, with the few buffers `Canvas::pack` keeps beside them). A packed surface is shared, not
//! copied, by the snapshot a chunk takes for its rollback: a surface a failed chunk took up is
//! put back from the very file it rested in, so the rollback of a surface is taken lazily, the
//! first time a chunk wakes it, at no cost.
//!
//! **Checkpoints.** A session that has named a surface saves all of them in one multi-surface
//! checkpoint (paint's `checkpoint::write_surfaces`, `PAINTSRF` version 1); one that hasn't
//! saves its canvas exactly as before.
//!
//! **Between sessions (plan-intaglio I6).** The artist and the printer are two sessions, each
//! seeing only what it is handed. `handoff(s, "plates/key.surface")` takes a surface out of its
//! session into a file in the studio, with its whole state and the sender's clock;
//! `receive("plates/key.surface", {after=, sha256=})` takes it into another, aged by the `after`
//! minutes it spent in transit, which the receiving log names (never the wall clock), and by the
//! content hash it was handed, which a replay checks it gets again.

use crate::api::{S, Studio};
use mlua::{Lua, MetaMethod, Result, Table, UserData, UserDataFields, UserDataMethods, Value};
use paint::Canvas;
use paint::checkpoint::SurfaceEntry;
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Instant;

/// The painting's surface: the canvas `canvas{}` makes.
pub const MAIN: &str = "painting";
/// A handed-off surface's file extension.
pub const HANDOFF_EXT: &str = "surface";
/// The first line of a handed-off surface's header.
const HANDOFF_MAGIC: &str = "easel handoff 1";

/// What a surface is (a multi-surface checkpoint's kind word).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// The canvas `canvas{}` made.
    Painting,
    /// A sheet of paper (`sheet{}`).
    Sheet,
    /// A copper or zinc plate (`plate{}`, `--@ plate`: plate.rs). Its entry's payload is the
    /// plate's state (paint's `PLAT` section); its canvas keeps its clock.
    Plate,
}

impl Kind {
    pub fn word(self) -> u64 {
        match self {
            Kind::Painting => 0,
            Kind::Sheet => 1,
            Kind::Plate => 2,
        }
    }

    pub fn of(word: u64) -> std::result::Result<Kind, String> {
        match word {
            0 => Ok(Kind::Painting),
            1 => Ok(Kind::Sheet),
            2 => Ok(Kind::Plate),
            k => Err(format!("a surface of kind {k}, which this build doesn't know")),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Kind::Painting => "painting",
            Kind::Sheet => "sheet",
            Kind::Plate => "plate",
        }
    }
}

/// A canvas put away: its state packed (`Canvas::pack`) and compressed into an anonymous
/// temporary file, not a copy of it in memory. A copy of a painted 2400 px engine-9 canvas is
/// some 4 GiB beside the canvas itself; packed and compressed it is a few hundred MiB, in the
/// page cache, which the kernel writes out under memory pressure instead of swapping the
/// painting. It is read back as often as asked (`restore`): one file serves a resting surface
/// and the rollback of the chunk that takes it up.
pub(crate) struct Packed {
    file: std::fs::File,
    aside: paint::checkpoint::Aside,
    /// A resting surface's (`rest`): the large buffers `Canvas::pack` keeps aside, moved to
    /// a file of their own (`Aside::spill`), so what it keeps in memory is a few scalars.
    spill: Option<std::fs::File>,
}

impl Packed {
    pub(crate) fn take(canvas: &Canvas) -> std::io::Result<Self> {
        let t0 = Instant::now();
        let mut packer = zstd::stream::write::Encoder::new(tempfile::tempfile()?, 1)?;
        // (a painted 2400 px canvas packs to a gigabyte: compressed on every core the
        // painting has, it is a second or two before each chunk, not six)
        packer.multithread(rayon::current_num_threads() as u32)?;
        let aside = canvas.pack(&mut packer)?;
        let file = packer.finish()?;
        eprintln!("canvas put aside: {} bytes, {:.2}s", file.metadata()?.len(), t0.elapsed().as_secs_f64());
        Ok(Self { file, aside, spill: None })
    }

    /// `take`, for a surface put aside for a while (it rests until it is used): the large
    /// buffers kept beside the packed state go to disk too. (A chunk's rollback snapshot, gone
    /// when the chunk ends, keeps them in memory, as PR #311 has it.)
    pub(crate) fn rest(canvas: &Canvas) -> std::io::Result<Self> {
        let mut p = Self::take(canvas)?;
        let mut z = zstd::stream::write::Encoder::new(tempfile::tempfile()?, 1)?;
        z.multithread(rayon::current_num_threads() as u32)?;
        p.aside.spill(&mut z)?;
        p.spill = Some(z.finish()?);
        Ok(p)
    }

    /// The canvas as it was packed, bit for bit. The file stays, for another restore.
    pub(crate) fn restore(&self) -> std::io::Result<Canvas> {
        use std::io::Seek;
        let t0 = Instant::now();
        let mut aside = self.aside.clone();
        if let Some(mut s) = self.spill.as_ref() {
            s.rewind()?;
            aside.unspill(&mut zstd::stream::read::Decoder::new(s)?)?;
        }
        let mut f = &self.file;
        f.rewind()?;
        let c = Canvas::unpack(&mut zstd::stream::read::Decoder::new(f)?, aside)?;
        eprintln!("canvas put back: {:.2}s", t0.elapsed().as_secs_f64());
        Ok(c)
    }

    /// The canvas's checkpoint (`write_state` with an empty header) as it was packed: its
    /// bytes, decompressed, without making the canvas again (`Canvas::pack` writes exactly
    /// them).
    pub(crate) fn copy_state(&self, w: &mut dyn Write) -> std::io::Result<()> {
        use std::io::Seek;
        let mut f = &self.file;
        f.rewind()?;
        std::io::copy(&mut zstd::stream::read::Decoder::new(f)?, w)?;
        Ok(())
    }

    /// What it costs while it rests: bytes on disk (compressed, in the page cache) and bytes of
    /// memory (the buffers kept beside them).
    pub(crate) fn cost(&self) -> (u64, usize) {
        let len = |f: &std::fs::File| f.metadata().map(|m| m.len()).unwrap_or(0);
        (len(&self.file) + self.spill.as_ref().map_or(0, len), self.aside.heap_bytes())
    }

    #[cfg(test)]
    pub(crate) fn damage(&self) {
        self.file.set_len(0).unwrap();
    }
}

/// The studio's fields that belong to a surface (the current one's are `Studio`'s own).
#[derive(Clone, Default)]
pub struct Own {
    pub style: Option<Rc<paint::Style>>,
    pub setup: Option<String>,
    pub canvas_src: Option<String>,
    pub recording_ground: u64,
    /// The surface's canvas clock when the studio clock read 0 (`Studio::clock0`).
    pub clock0: f64,
    pub view: Option<crate::world::ViewU>,
    /// A plate's state (plate.rs): None for the painting and sheets.
    pub plate: Option<Rc<paint::plate::Plate>>,
}

impl Own {
    pub(crate) fn take(s: &mut Studio) -> Own {
        Own { style: s.style.take(), setup: s.setup.take(), canvas_src: s.canvas_src.take(), recording_ground: s.recording_ground, clock0: s.clock0, view: s.view.take(), plate: s.plate.take() }
    }

    pub(crate) fn put(self, s: &mut Studio) {
        s.style = self.style;
        s.setup = self.setup;
        s.canvas_src = self.canvas_src;
        s.recording_ground = self.recording_ground;
        s.clock0 = self.clock0;
        s.view = self.view;
        s.plate = self.plate;
    }

    /// The current surface's, as the studio holds them (a copy; the world view is left out,
    /// as a checkpoint leaves it out: `fields`).
    fn of(s: &Studio) -> Own {
        Own { style: s.style.clone(), setup: s.setup.clone(), canvas_src: s.canvas_src.clone(), recording_ground: s.recording_ground, clock0: s.clock0, view: None, plate: s.plate.clone() }
    }

    /// Its entry's payload in a multi-surface checkpoint: a plate's state, or nothing.
    pub(crate) fn payload(&self) -> std::io::Result<Vec<u8>> {
        let mut b = Vec::new();
        if let Some(p) = &self.plate {
            p.write(&mut b)?;
        }
        Ok(b)
    }
}

/// A surface put aside.
#[derive(Clone)]
pub struct Resting {
    pub own: Own,
    /// Its canvas, packed (shared with the rollback snapshot of a chunk that takes it up).
    pub canvas: Option<Rc<Packed>>,
    /// The studio clock it has been aged to (minutes): when it is taken up again it waits out
    /// the time since. (A surface received from another session is aged to before it arrived:
    /// `receive`.)
    pub since: f64,
}

/// One surface of the session.
#[derive(Clone)]
pub struct Slot {
    pub name: String,
    pub kind: Kind,
    /// None for the current surface (its state is the studio's own).
    pub rest: Option<Resting>,
}

/// The session's surfaces: none named (the painting alone, as every session before the
/// switch), or every surface in the order made, the painting first.
#[derive(Clone, Default)]
pub struct Surfaces {
    pub slots: Vec<Slot>,
    pub current: usize,
}

impl Surfaces {
    /// Whether the session has named a surface besides the painting (and so keeps them all:
    /// its saves and checkpoints are multi-surface ones from then on).
    pub fn named(&self) -> bool {
        !self.slots.is_empty()
    }

    pub fn current_name(&self) -> &str {
        self.slots.get(self.current).map_or(MAIN, |s| s.name.as_str())
    }

    pub fn main_current(&self) -> bool {
        self.current_name() == MAIN
    }

    pub fn find(&self, name: &str) -> Option<usize> {
        self.slots.iter().position(|s| s.name == name)
    }

    /// Every surface's name, the painting first.
    pub fn names(&self) -> Vec<String> {
        if self.named() { self.slots.iter().map(|s| s.name.clone()).collect() } else { vec![MAIN.to_string()] }
    }

    /// Which surfaces there are and which is current, with each resting one's clocks: what a
    /// replay checkpoint's effects check beside the studio's state (replay_checkpoint.rs).
    /// None while none is named, so a session with the painting alone checks what it did.
    pub fn identity(&self) -> Option<Vec<u8>> {
        if !self.named() {
            return None;
        }
        let mut b = Vec::new();
        b.extend_from_slice(&(self.current as u64).to_le_bytes());
        for s in &self.slots {
            b.extend_from_slice(&(s.name.len() as u64).to_le_bytes());
            b.extend_from_slice(s.name.as_bytes());
            b.extend_from_slice(&s.kind.word().to_le_bytes());
            if let Some(r) = &s.rest {
                b.extend_from_slice(&r.since.to_bits().to_le_bytes());
                b.extend_from_slice(&r.own.clock0.to_bits().to_le_bytes());
            }
        }
        Some(b)
    }

    /// One line for `status`: the surfaces and which is current (empty while none is named).
    pub fn status(&self) -> String {
        if !self.named() {
            return String::new();
        }
        let names: Vec<String> = self.slots.iter().enumerate().map(|(i, s)| if i == self.current { format!("{} (current)", s.name) } else { s.name.clone() }).collect();
        format!(" · surfaces: {}", names.join(", "))
    }
}

/// The surface the studio holds now goes aside and `to` is taken up (no time passes for it yet:
/// `wake_up` waits it up to the clock). Returns how long the surface taken up has rested
/// (minutes on the studio clock). Moves memory, not state: the current canvas is packed and let
/// go before the other is unpacked, so one canvas is in memory at a time. (A failure on the way
/// fails the chunk, whose rollback puts every surface back: session.rs `restore`.)
fn swap(s: &mut Studio, to: usize) -> std::io::Result<f64> {
    let from = s.surfaces.current;
    debug_assert!(from != to);
    let packed = s.canvas.as_ref().map(Packed::rest).transpose()?.map(Rc::new);
    let own = Own::take(s);
    s.surfaces.slots[from].rest = Some(Resting { own, canvas: packed, since: s.clock });
    s.canvas = None;
    let taken = s.surfaces.slots[to].rest.take().expect("a surface not current rests");
    s.canvas = taken.canvas.as_ref().map(|p| p.restore()).transpose()?;
    taken.own.put(s);
    s.surfaces.current = to;
    Ok(s.clock - taken.since)
}

/// The surface just taken up waits out the `rested` minutes it was put aside for, and its
/// clock is set to the studio's. An effect of its own (replay_checkpoint.rs): a replay
/// checkpoint's reconstruction records it rather than ageing the paint again.
fn wake_up(st: &S, rested: f64) -> Result<()> {
    crate::replay_checkpoint::effect(st, "surface wait", || {
        let mut g = st.borrow_mut();
        let s = &mut *g;
        if let Some(c) = s.canvas.as_mut() {
            if rested > 0.0 {
                c.wait(rested as f32);
            }
            // the studio clock is the one clock: the surface's is read against it from now on
            s.clock0 = c.clock() - s.clock;
        }
        Ok(())
    })
}

/// Make surface `to` current: the hand time spent on the one put aside goes on the clock
/// first, and the one taken up is aged to the clock.
fn take_up(st: &S, to: usize) -> Result<()> {
    crate::time::flush(st, true);
    let rested = swap(&mut st.borrow_mut(), to).map_err(|e| mlua::Error::runtime(format!("the surfaces couldn't be changed over: {e}")))?;
    wake_up(st, rested)
}

/// A surface's userdata: what `sheet{}` and `receive` return and `use` and `handoff` take.
#[derive(Clone)]
pub struct SurfaceU {
    pub name: String,
    pub kind: Kind,
    /// Its width (mm) and height (units: its width is 1000 units, as the painting's is).
    pub size_mm: f32,
    pub h: f32,
}

impl UserData for SurfaceU {
    fn add_fields<F: UserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("name", |_, s| Ok(s.name.clone()));
        f.add_field_method_get("kind", |_, s| Ok(s.kind.name()));
        f.add_field_method_get("size", |_, s| Ok(s.size_mm));
        f.add_field_method_get("W", |_, _| Ok(1000));
        f.add_field_method_get("H", |_, s| Ok(s.h));
    }

    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        // a plate's tools (plate.rs; `--@ plate`)
        crate::plate::methods(m);
        m.add_meta_method(MetaMethod::ToString, |_, s, ()| Ok(format!("{} {:?} ({} mm, 1000 x {} units)", s.kind.name(), s.name, crate::api::fmt_num(s.size_mm), crate::api::fmt_num(s.h))));
    }
}

fn valid_name(n: &str) -> bool {
    !n.is_empty() && n.len() <= paint::checkpoint::SURFACE_NAME_MAX && n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

/// A surface's name from what a verb was given: the surface (what `sheet{}` returned) or its
/// name.
fn name_of(v: &Value, verb: &str) -> Result<String> {
    match v {
        Value::String(n) => Ok(n.to_str()?.to_string()),
        Value::UserData(u) if u.is::<SurfaceU>() => Ok(u.borrow::<SurfaceU>()?.name.clone()),
        o => crate::api::err(format!("{verb}: a surface (what sheet{{}} returned) or its name (\"painting\" for the canvas), got {}", o.type_name())),
    }
}

fn no_surface(s: &Surfaces, verb: &str, name: &str) -> mlua::Error {
    mlua::Error::runtime(format!("{verb}: there is no surface called {name:?} (there are: {})", s.names().join(", ")))
}

/// The session's surfaces start being kept by name: the painting is the first.
pub(crate) fn name_the_painting(s: &mut Studio) {
    if s.surfaces.slots.is_empty() {
        s.surfaces.slots.push(Slot { name: MAIN.into(), kind: Kind::Painting, rest: None });
        s.surfaces.current = 0;
    }
}

/// A new surface's checks: a canvas to lie beside, a name of its own, room for it.
pub(crate) fn check_new(s: &Studio, verb: &str, name: &str) -> Result<()> {
    if !valid_name(name) {
        return crate::api::err(format!("{verb}: name {name:?}: 1 to 64 letters, digits, _ or -"));
    }
    if name == MAIN {
        return crate::api::err(format!("{verb}: {MAIN:?} is the canvas canvas{{}} made: name the surface otherwise"));
    }
    if s.canvas.is_none() && s.surfaces.main_current() {
        return Err(crate::api::no_canvas());
    }
    if s.surfaces.find(name).is_some() {
        return crate::api::err(format!("{verb}: there is a surface called {name:?} already"));
    }
    if s.surfaces.slots.len().max(1) >= paint::checkpoint::SURFACES_MAX {
        return crate::api::err(format!("{verb}: a session holds at most {} surfaces", paint::checkpoint::SURFACES_MAX));
    }
    Ok(())
}

const SHEET_HELP: &str = "sheet{name=\"<name>\", size=<mm>, aspect=<width / height>, paper={grammage=, tone=, ...}, ground=?, seed=?}: a sheet of paper beside the painting, a surface of its own (use(s) to draw on it, use(\"painting\") to go back)";
const HANDOFF_HELP: &str = "handoff(s, \"<folder>/<name>.surface\"): the surface s leaves this session for a file in the studio, with its whole state, for another session to receive";
const RECEIVE_HELP: &str = "receive(\"<folder>/<name>.surface\", {after=<minutes in transit>, sha256=\"<the file's hash, as handoff gave it>\", name=?}): a surface handed off by another session, aged by the minutes it spent on the way";

/// The switch's verbs: `sheet{}`, `use`, `surfaces`, `handoff` and `receive`. Installed beside
/// the paper-and-pastel switch's (whose paper a sheet is made of), only when the log has the
/// switch, so an older log sees exactly the globals it saw.
pub(crate) fn install(lua: &Lua, st: S) -> Result<()> {
    let g = lua.globals();
    // sheet{name=, size=, aspect=, paper={...}, ground=, seed=}: a sheet of paper, a surface
    // of its own, put aside until it is used (the current surface stays current)
    {
        let st = st.clone();
        g.set("sheet", lua.create_function(move |_, o: Option<Table>| {
            let o = o.ok_or_else(|| mlua::Error::runtime(SHEET_HELP))?;
            crate::api::check_keys(&o, &["name", "size", "aspect", "paper", "ground", "seed"], "sheet")?;
            let name: String = o.get::<Option<String>>("name")?.ok_or_else(|| mlua::Error::runtime(format!("sheet: name= (what the sheet is called: use(\"<name>\"), look --surface <name>)\n{SHEET_HELP}")))?;
            check_new(&st.borrow(), "sheet", &name)?;
            let mm = crate::api::num(&o, "size")?.ok_or_else(|| mlua::Error::runtime(format!("sheet: size= (the sheet's width in mm)\n{SHEET_HELP}")))?;
            if !(20.0..=5000.0).contains(&mm) {
                return crate::api::err("sheet: size is the sheet's width in mm, 20 to 5000");
            }
            let aspect = crate::api::num(&o, "aspect")?.ok_or_else(|| mlua::Error::runtime(format!("sheet: aspect= (width / height)\n{SHEET_HELP}")))?;
            if !(0.2..=5.0).contains(&aspect) {
                return crate::api::err("sheet: aspect is width / height, between 0.2 and 5");
            }
            let seed = o.get::<Option<u64>>("seed")?.unwrap_or(1);
            let tubes = st.borrow().tubes.clone();
            let (paper, tone) = match o.get::<Value>("paper")? {
                Value::Table(t) => crate::api::paper_of(&tubes, &t, seed)?,
                Value::Nil => return crate::api::err(format!("sheet: paper= (the sheet's make: {{grammage=, tone=, ...}})\n{SHEET_HELP}")),
                v => return crate::api::err(format!("sheet: paper= is a table of the sheet's make, got {}", v.type_name())),
            };
            let ground = match o.get::<Value>("ground")? {
                Value::Nil => Vec::new(),
                g => crate::api::ground_of(&tubes, &g)?,
            };
            // laid out in mm at the painting's pitch (plan-intaglio I1): a line traced from one
            // sheet to another keeps its width in pixels
            let (width, painting_mm) = {
                let s = st.borrow();
                let painting = if s.surfaces.main_current() { s.style.clone() } else { s.surfaces.slots[0].rest.as_ref().and_then(|r| r.own.style.clone()) };
                (s.width, painting.map(|p| p.width_mm).ok_or_else(crate::api::no_canvas)?)
            };
            // (at least 16 px across, which only a test's tiny painting comes below)
            let px = ((width as f64 * mm as f64 / painting_mm as f64).round() as usize).max(16);
            let most = crate::session::WIDTHS[1];
            if px > most {
                return crate::api::err(format!("sheet: {mm} mm at the painting's {} mm a pixel is {px} px across; a sheet is at most {most} px", crate::api::fmt_num(painting_mm / width as f32)));
            }
            let sty = paint::Style { name: "oil", width_mm: mm, raw: tone, ground, ..paint::Style::oil_with((*tubes).clone()) };
            let mut c = sty.prepare_paper(px, aspect, seed, paper).with_drain(tubes.drain);
            // hand time counts on the sheet as on the canvas (not the painting's frames: time.rs)
            c.set_hand_time(Some(crate::time::SLICE_MIN));
            let h = c.height();
            let packed = Rc::new(Packed::rest(&c).map_err(|e| mlua::Error::runtime(format!("sheet: the sheet couldn't be put aside: {e}")))?);
            let recording_ground = crate::live_frames::ground_signature(&c);
            let clock = c.clock();
            drop(c);
            let mut s = st.borrow_mut();
            let own = Own {
                style: Some(Rc::new(sty)),
                setup: Some(format!("sheet {name:?}: size={}, aspect={aspect}, paper={} g/m², seed={seed}", crate::api::fmt_num(mm), crate::api::fmt_num(paper.grammage))),
                canvas_src: Some(format!("sheet{}", crate::api::lua_src(&Value::Table(o.clone()))?)),
                recording_ground,
                // (its clock reads the studio's from the start: `wake_up`)
                clock0: clock - s.clock,
                view: None,
                plate: None,
            };
            name_the_painting(&mut s);
            let since = s.clock;
            s.surfaces.slots.push(Slot { name: name.clone(), kind: Kind::Sheet, rest: Some(Resting { own, canvas: Some(packed), since }) });
            Ok(SurfaceU { name, kind: Kind::Sheet, size_mm: mm, h })
        })?)?;
    }
    // use(s) or use("name"): make that surface current; returns the one current before
    {
        let st = st.clone();
        g.set("use", lua.create_function(move |_, v: Value| {
            let name = name_of(&v, "use")?;
            let to = {
                let s = st.borrow();
                if s.canvas.is_none() && s.surfaces.main_current() {
                    return Err(crate::api::no_canvas());
                }
                let was = s.surfaces.current_name().to_string();
                if name == was {
                    return Ok(was);
                }
                let Some(to) = s.surfaces.find(&name) else { return Err(no_surface(&s.surfaces, "use", &name)) };
                if s.canvas.as_ref().is_some_and(|c| c.has_sheet()) {
                    return crate::api::err("use: a sheet of paper lies on the picture; lift it first (lift_sheet())");
                }
                to
            };
            let was = st.borrow().surfaces.current_name().to_string();
            take_up(&st, to)?;
            Ok(was)
        })?)?;
    }
    // surfaces(): the surfaces' names, the painting first, and the current one's
    {
        let st = st.clone();
        g.set("surfaces", lua.create_function(move |_, ()| {
            let s = st.borrow();
            Ok((s.surfaces.names(), s.surfaces.current_name().to_string()))
        })?)?;
    }
    // handoff(s, path): s leaves this session for a file; returns the file's sha256
    {
        let st = st.clone();
        g.set("handoff", lua.create_function(move |_, (v, path): (Value, Option<String>)| {
            let name = name_of(&v, "handoff")?;
            let path = path.ok_or_else(|| mlua::Error::runtime(HANDOFF_HELP))?;
            handoff(&st, &name, &path)
        })?)?;
    }
    // receive(path, {after=, sha256=, name=}): a surface another session handed off
    {
        let st = st.clone();
        g.set("receive", lua.create_function(move |_, (path, o): (Option<String>, Option<Table>)| {
            let path = path.ok_or_else(|| mlua::Error::runtime(RECEIVE_HELP))?;
            let o = o.ok_or_else(|| mlua::Error::runtime(RECEIVE_HELP))?;
            receive(&st, &path, &o)
        })?)?;
    }
    // plate{}: a copper or zinc plate among the surfaces (plate.rs), with its own switch
    if st.borrow().tubes.plate {
        crate::plate::install(lua, st)?;
    }
    Ok(())
}

// ---------------------------------------------------------------- the handoff

/// A handed-off surface's file, written beside where it goes (`part`), put in place when the
/// chunk that handed it off is kept (`commit`) and taken away if it fails (`discard`): a
/// surface handed off by a chunk that failed never left.
pub struct Pending {
    part: PathBuf,
    dest: PathBuf,
}

/// Writes through to a file and hashes what it writes.
struct Hashed<W: Write> {
    w: W,
    hash: Sha256,
}

impl<W: Write> Write for Hashed<W> {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        let n = self.w.write(b)?;
        self.hash.update(&b[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.w.flush()
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// Where a handed-off surface's file is, in the studio (as a picture `save` writes is: main.rs
/// `in_studio_file`), and whether that is a file of its own (no link, no second name).
fn handoff_path(verb: &str, path: &str) -> Result<PathBuf> {
    let p = crate::in_studio_file(Path::new(path), verb, HANDOFF_EXT, "a surface").map_err(mlua::Error::runtime)?;
    use std::os::unix::fs::MetadataExt;
    if std::fs::metadata(&p).is_ok_and(|m| !m.is_file() || m.nlink() > 1) {
        return crate::api::err(format!("{verb} {path}: that is not a file of its own (a folder, or a file with hard links)"));
    }
    Ok(p)
}

fn handoff(st: &S, name: &str, path: &str) -> Result<String> {
    if name == MAIN {
        return crate::api::err(format!("handoff: the painting stays in its session; hand off a surface beside it (sheet{{}})\n{HANDOFF_HELP}"));
    }
    let dest = handoff_path("handoff", path)?;
    let (at, live) = {
        let s = st.borrow();
        let Some(at) = s.surfaces.find(name) else { return Err(no_surface(&s.surfaces, "handoff", name)) };
        if at == s.surfaces.current && s.canvas.as_ref().is_some_and(|c| c.has_sheet()) {
            return crate::api::err("handoff: a sheet of paper lies on the picture; lift it first (lift_sheet())");
        }
        if s.handoffs.iter().any(|p| p.dest == dest) {
            return crate::api::err(format!("handoff {path}: this chunk hands another surface off there already"));
        }
        (at, s.live)
    };
    // (a live session hands off a new file; a replay finds the one it handed off before)
    if live && dest.exists() {
        return crate::api::err(format!("handoff {path}: a surface was handed off there already, which another session may have received: hand this one off under a name of its own"));
    }
    // a surface in hand is put down first: the painting is current again
    if at == st.borrow().surfaces.current {
        take_up(st, 0)?;
    }
    let slot = {
        let mut g = st.borrow_mut();
        let s = &mut *g;
        let slot = s.surfaces.slots.remove(at);
        if s.surfaces.current > at {
            s.surfaces.current -= 1;
        }
        slot
    };
    let rest = slot.rest.expect("a surface handed off rests");
    let packed = rest.canvas.clone().ok_or_else(|| mlua::Error::runtime("handoff: a surface without its canvas"))?;
    // the file is written as an effect: a replay checkpoint's reconstruction, whose canvases
    // are put back afterwards, takes the hash it gave rather than writing the file again
    let path = path.to_string();
    let name = name.to_string();
    crate::replay_checkpoint::effect(st, "handoff", || {
        let (clock, header) = {
            let s = st.borrow();
            let owed = s.clock - rest.since;
            let header = format!(
                "{HANDOFF_MAGIC}\nclock={:016x}\nowed={:016x}\nengine={}\nbox={}\nat={}\n",
                s.clock.to_bits(),
                owed.to_bits(),
                s.tubes.engine,
                s.tubes.name,
                crate::time::time_of_day(s.clock)
            );
            (s.clock, header)
        };
        let dir = dest.parent().ok_or_else(|| mlua::Error::runtime("handoff: no folder"))?;
        std::fs::create_dir_all(dir).map_err(|e| mlua::Error::runtime(format!("handoff {path}: {e}")))?;
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let file_name = dest.file_name().and_then(|n| n.to_str()).unwrap_or("surface");
        let part = dir.join(format!(".{file_name}.{}.{}.part", std::process::id(), NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)));
        let written = (|| -> std::io::Result<String> {
            let f = std::fs::OpenOptions::new().write(true).create_new(true).open(&part)?;
            let mut hashed = Hashed { w: std::io::BufWriter::with_capacity(1 << 20, f), hash: Sha256::new() };
            {
                // (one thread: the same surface makes the same bytes)
                let mut z = zstd::stream::write::Encoder::new(&mut hashed, 3)?;
                let entry = SurfaceEntry { name: name.clone(), kind: slot.kind.word(), fields: fields(&rest.own, None), payload: rest.own.payload()? };
                paint::checkpoint::write_surfaces(&mut z, &header, &name, &[entry], |_, w| packed.copy_state(w))?;
                z.finish()?;
            }
            hashed.w.flush()?;
            hashed.w.get_ref().sync_all()?;
            Ok(hex(&hashed.hash.finalize()))
        })();
        let sha = match written {
            Ok(h) => h,
            Err(e) => {
                let _ = std::fs::remove_file(&part);
                return crate::api::err(format!("handoff {path}: {e}"));
            }
        };
        let mut s = st.borrow_mut();
        s.handoffs.push(Pending { part, dest: dest.clone() });
        s.out.push_str(&format!(
            "handed off {name:?} at {} (clock {} min): {path} sha256={sha}\n  receive it with receive({path:?}, {{after=<minutes on the way>, sha256={sha:?}}})\n",
            crate::time::time_of_day(clock),
            crate::api::fmt_num(clock as f32)
        ));
        Ok(sha)
    })
}

/// Put the files the chunk handed off in place: it is kept. A live session's never replaces
/// a file (a surface handed off before may have been received); a replay's leaves the one it
/// handed off when it was painted as it is. Nothing is put in place if one can't be.
pub(crate) fn commit(st: &S, replay: bool) -> std::result::Result<(), String> {
    let pending = std::mem::take(&mut st.borrow_mut().handoffs);
    let mut placed: Vec<PathBuf> = Vec::new();
    let mut failed = None;
    for p in &pending {
        if failed.is_some() {
            break;
        }
        if replay && p.dest.exists() {
            continue;
        }
        // a second name, then the first let go: never over a file that is there
        match std::fs::hard_link(&p.part, &p.dest) {
            Ok(()) => placed.push(p.dest.clone()),
            Err(e) => failed = Some(format!("handoff {}: {e}", p.dest.display())),
        }
    }
    for p in &pending {
        let _ = std::fs::remove_file(&p.part);
    }
    if let Some(e) = failed {
        for d in placed {
            let _ = std::fs::remove_file(d);
        }
        return Err(e);
    }
    if let Some(dir) = pending.first().and_then(|p| p.dest.parent()) {
        let _ = std::fs::File::open(dir).and_then(|f| f.sync_all());
    }
    Ok(())
}

/// The chunk failed: the surfaces it handed off never left.
pub(crate) fn discard(st: &S) {
    for p in std::mem::take(&mut st.borrow_mut().handoffs) {
        let _ = std::fs::remove_file(&p.part);
    }
}

fn hash_file(p: &Path) -> std::io::Result<String> {
    let mut f = std::fs::File::open(p)?;
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(hex(&h.finalize()))
}

/// What a handed-off surface's file says of the session that sent it.
struct Sent {
    clock: f64,
    owed: f64,
    engine: u32,
}

fn sent(header: &str) -> std::result::Result<Sent, String> {
    let mut lines = header.lines();
    if lines.next() != Some(HANDOFF_MAGIC) {
        return Err(format!("not a handed-off surface (its header doesn't start {HANDOFF_MAGIC:?})"));
    }
    let h: std::collections::BTreeMap<&str, &str> = lines.filter_map(|l| l.split_once('=')).collect();
    let hex64 = |k: &str| h.get(k).and_then(|v| u64::from_str_radix(v, 16).ok()).map(f64::from_bits).filter(|v| v.is_finite()).ok_or_else(|| format!("its header has no {k}= (16 hex digits)"));
    let engine = h.get("engine").and_then(|v| v.parse().ok()).ok_or("its header names no engine")?;
    Ok(Sent { clock: hex64("clock")?, owed: hex64("owed")?, engine })
}

fn receive(st: &S, path: &str, o: &Table) -> Result<SurfaceU> {
    crate::api::check_keys(o, &["after", "sha256", "name"], "receive")?;
    let after = o.get::<Option<f64>>("after")?.ok_or_else(|| mlua::Error::runtime(format!("receive: after= (the minutes it spent on the way from the session that handed it off: the log names them, as the wall clock can't be replayed)\n{RECEIVE_HELP}")))?;
    if !(0.0..=crate::api::MAX_WAIT_MIN).contains(&after) {
        return crate::api::err(format!("receive: after={after}: want minutes from 0 to {} (10 years)", crate::api::MAX_WAIT_MIN));
    }
    let file = handoff_path("receive", path)?;
    if !file.is_file() {
        return crate::api::err(format!("receive {path}: no surface was handed off there"));
    }
    let sha = hash_file(&file).map_err(|e| mlua::Error::runtime(format!("receive {path}: {e}")))?;
    match o.get::<Option<String>>("sha256")? {
        None => return crate::api::err(format!("receive {path}: sha256= names what was handed off, so that a replay of this log receives the same: this file's is sha256={sha:?}")),
        Some(want) if want.to_ascii_lowercase() != sha => {
            return crate::api::err(format!("receive {path}: this is not the surface the log received: the file there now has sha256={sha:?}, not {want:?} (it was changed, or handed off again)"));
        }
        Some(_) => {}
    }
    let tubes = st.borrow().tubes.clone();
    let read = (|| -> std::result::Result<ReadBack, String> {
        let f = std::fs::File::open(&file).map_err(|e| e.to_string())?;
        let mut z = zstd::stream::read::Decoder::new(f).map_err(|e| e.to_string())?;
        read_all(&mut z).map_err(|e| e.to_string())
    })()
    .map_err(|e| mlua::Error::runtime(format!("receive {path}: {e}")))?;
    let from = sent(&read.header).map_err(|e| mlua::Error::runtime(format!("receive {path}: {e}")))?;
    if from.engine != tubes.engine {
        return crate::api::err(format!("receive {path}: it was painted with engine {}, this session with engine {}: a surface goes on with the engine it was made with", from.engine, tubes.engine));
    }
    if read.slots.len() != 1 {
        return crate::api::err(format!("receive {path}: a handed-off file holds one surface, this one {}", read.slots.len()));
    }
    let ReadBack { mut canvas, mut slots, current_fields, current_payload, .. } = read;
    let (sent_name, kind, _, _, _) = slots.pop().unwrap();
    if kind == Kind::Painting {
        return crate::api::err(format!("receive {path}: a painting isn't handed off"));
    }
    if kind == Kind::Plate && !tubes.plate {
        return crate::api::err(format!("receive {path}: a plate, and this painting has no plates (its log's head has no {:?} line)", crate::session::PLATE_MARK));
    }
    let plate = plate_of(kind, &current_payload).map_err(|e| mlua::Error::runtime(format!("receive {path}: {e}")))?;
    if plate.as_ref().is_some_and(|p| p.coats().is_some()) && !tubes.etch {
        return crate::api::err(format!("receive {path}: an etched plate, and this painting doesn't etch (its log's head has no {:?} line)", crate::session::ETCH_MARK));
    }
    let canvas = canvas.take().ok_or_else(|| mlua::Error::runtime(format!("receive {path}: the surface has no canvas")))?;
    let name = o.get::<Option<String>>("name")?.unwrap_or(sent_name.clone());
    check_new(&st.borrow(), "receive", &name)?;
    let (mut own, _) = read_fields(&current_fields, &tubes).map_err(|e| mlua::Error::runtime(format!("receive {path}: {e}")))?;
    own.plate = plate;
    let size_mm = own.style.as_ref().map_or(0.0, |s| s.width_mm);
    let h = canvas.height();
    let packed = Rc::new(Packed::rest(&canvas).map_err(|e| mlua::Error::runtime(format!("receive {path}: the surface couldn't be put aside: {e}")))?);
    drop(canvas);
    let mut s = st.borrow_mut();
    name_the_painting(&mut s);
    // aged to before it arrived by what it was owed in the sender and the minutes on the way:
    // taken up (or looked at), it waits them out with the time since, as one wait
    let since = s.clock - (from.owed + after);
    // (a plate handed off in its bath came in its tray: it bit on the way, and bites on here)
    let bathing = own.plate.as_ref().and_then(|p| p.in_bath()).map(|b| b.bath.acid.name());
    s.surfaces.slots.push(Slot { name: name.clone(), kind, rest: Some(Resting { own, canvas: Some(packed), since }) });
    s.out.push_str(&format!(
        "received {name:?} ({} {sent_name:?}, handed off at clock {} min of the session that sent it) after {} min on the way\n",
        kind.name(),
        crate::api::fmt_num(from.clock as f32),
        crate::api::fmt_num(after as f32)
    ));
    if let Some(acid) = bathing {
        s.out.push_str(&format!("  it came in its {acid} bath, and bit all the way: it bites on until it is lifted out (use({name:?}), then k:lift())\n"));
    }
    Ok(SurfaceU { name, kind, size_mm, h })
}

/// Every surface's plate, by name, as a check compares them (check.rs): the current one's as
/// it is, the others' as they rest. Empty while no plate is among the surfaces.
pub(crate) fn plates(s: &Studio) -> Vec<(String, Vec<u8>)> {
    let bytes = |p: &paint::plate::Plate| {
        let mut b = Vec::new();
        p.write(&mut b).expect("a plate writes to memory");
        b
    };
    let mut out = Vec::new();
    if let Some(p) = &s.plate {
        out.push((s.surfaces.current_name().to_string(), bytes(p)));
    }
    for slot in &s.surfaces.slots {
        if let Some(p) = slot.rest.as_ref().and_then(|r| r.own.plate.as_ref()) {
            out.push((slot.name.clone(), bytes(p)));
        }
    }
    out.sort();
    out
}

// ---------------------------------------------------------------- checkpoints

/// The fields a multi-surface checkpoint keeps of a surface: the studio clock it is aged to
/// (`since`: None for the current surface) and its own studio fields (`Own`: its clock's
/// offset, its setup, its style's width and relief). A resting surface's world view isn't
/// kept: a view is a Lua object, made again by the chunk that uses it (as the painting's isn't
/// kept by a save, save.rs).
pub(crate) fn fields(own: &Own, since: Option<f64>) -> String {
    let mut f = String::new();
    if let Some(t) = since {
        f.push_str(&format!("since={:016x}\n", t.to_bits()));
    }
    f.push_str(&format!("clock0={:016x}\n", own.clock0.to_bits()));
    if let Some(setup) = &own.setup {
        f.push_str(&format!("setup={}\n", setup.replace('\n', " ")));
    }
    if let Some(src) = &own.canvas_src {
        f.push_str(&format!("src={}\n", src.replace('\n', " ")));
    }
    if let Some(sty) = &own.style {
        f.push_str(&format!("mm={:08x}\n", sty.width_mm.to_bits()));
        f.push_str(&format!("relief={:08x},{:08x}\n", sty.relief.0.to_bits(), sty.relief.1.to_bits()));
    }
    f.push_str(&format!("ground={:016x}\n", own.recording_ground));
    f
}

/// `fields` read back: the surface's own fields (its style the box's oil style with the width
/// and relief it had, as a save keeps the painting's: save.rs) and the studio clock it is aged
/// to (None: it was current).
pub(crate) fn read_fields(text: &str, tubes: &paint::Palette) -> std::result::Result<(Own, Option<f64>), String> {
    let h: std::collections::BTreeMap<&str, &str> = text.lines().filter_map(|l| l.split_once('=')).collect();
    let hex64 = |k: &str| -> std::result::Result<Option<u64>, String> { h.get(k).map(|v| u64::from_str_radix(v, 16).map_err(|_| format!("a surface's {k}={v:?} isn't hex"))).transpose() };
    let hex32 = |v: &str, k: &str| u32::from_str_radix(v, 16).map(f32::from_bits).map_err(|_| format!("a surface's {k}={v:?} isn't a hex float"));
    let since = hex64("since")?.map(f64::from_bits);
    let clock0 = f64::from_bits(hex64("clock0")?.ok_or("a surface without its clock0=")?);
    let style = match h.get("relief") {
        None => None,
        Some(rel) => {
            let (a, b) = rel.split_once(',').ok_or_else(|| format!("a surface's relief={rel:?} isn't two hex floats"))?;
            let oil = paint::Style::oil_with(tubes.clone());
            let width_mm = match h.get("mm") {
                Some(v) => hex32(v, "mm")?,
                None => oil.width_mm,
            };
            Some(Rc::new(paint::Style { relief: (hex32(a, "relief")?, hex32(b, "relief")?), width_mm, ..oil }))
        }
    };
    let own = Own {
        style,
        setup: h.get("setup").map(|v| v.to_string()),
        canvas_src: h.get("src").map(|v| v.to_string()),
        recording_ground: hex64("ground")?.unwrap_or(0),
        clock0,
        view: None,
        // (a plate's is its entry's payload: `plate_of`)
        plate: None,
    };
    Ok((own, since))
}

/// Write every surface of a session that has named one into a multi-surface checkpoint, with
/// `header`: the current surface's canvas as it is in memory, every other as it rests packed
/// (copied, not made again: no second canvas in memory).
pub(crate) fn write_all(s: &Studio, header: &str, w: &mut impl Write) -> std::io::Result<()> {
    let sf = &s.surfaces;
    let entries: Vec<SurfaceEntry> = sf.slots.iter().enumerate().map(|(i, slot)| {
        let (fields, payload) = match &slot.rest {
            Some(r) => (fields(&r.own, Some(r.since)), r.own.payload()?),
            None => {
                let own = Own::of(s);
                (fields(&own, None), own.payload()?)
            }
        };
        debug_assert_eq!(slot.rest.is_none(), i == sf.current);
        Ok(SurfaceEntry { name: slot.name.clone(), kind: slot.kind.word(), fields, payload })
    }).collect::<std::io::Result<_>>()?;
    let missing = || std::io::Error::other("a surface without its canvas");
    paint::checkpoint::write_surfaces(w, header, sf.current_name(), &entries, |i, w| match &sf.slots[i].rest {
        None => s.canvas.as_ref().ok_or_else(missing)?.write_state(&mut &mut *w, ""),
        Some(r) => r.canvas.as_ref().ok_or_else(missing)?.copy_state(w),
    })
}

/// A multi-surface checkpoint read back (`read_all`): the current surface's canvas, every
/// other surface packed as it was read (one canvas in memory at a time).
pub(crate) struct ReadBack {
    pub header: String,
    pub canvas: Option<Canvas>,
    current: usize,
    current_fields: String,
    /// The current surface's payload (a plate's state, or nothing).
    current_payload: Vec<u8>,
    /// Each surface's name, kind, fields, (but the current one's) packed canvas and payload.
    slots: Vec<(String, Kind, String, Option<Rc<Packed>>, Vec<u8>)>,
}

/// A surface's plate from its entry's payload: a plate has one, no other surface has any.
fn plate_of(kind: Kind, payload: &[u8]) -> std::result::Result<Option<Rc<paint::plate::Plate>>, String> {
    match (kind, payload.is_empty()) {
        (Kind::Plate, false) => paint::plate::Plate::read(&mut std::io::Cursor::new(payload)).map(|p| Some(Rc::new(p))).map_err(|e| e.to_string()),
        (Kind::Plate, true) => Err("a plate without its state".into()),
        (_, true) => Ok(None),
        (k, false) => Err(format!("a {} with a payload it shouldn't have", k.name())),
    }
}

pub(crate) fn read_all(r: &mut impl Read) -> std::io::Result<ReadBack> {
    let mut slots = Vec::new();
    let mut canvas = None;
    let mut current_fields = None;
    let mut current_payload = Vec::new();
    let (header, current) = paint::checkpoint::read_surfaces(r, |e, c| {
        let kind = Kind::of(e.kind).map_err(std::io::Error::other)?;
        let rests = e.fields.lines().any(|l| l.starts_with("since="));
        let packed = if rests {
            Some(Rc::new(Packed::rest(&c)?))
        } else {
            if canvas.is_some() {
                return Err(std::io::Error::other("a multi-surface checkpoint with two current surfaces"));
            }
            canvas = Some(c);
            current_fields = Some(e.fields.clone());
            current_payload = e.payload.clone();
            None
        };
        slots.push((e.name, kind, e.fields, packed, e.payload));
        Ok(())
    })?;
    let at = slots.iter().position(|s| s.0 == current).ok_or_else(|| std::io::Error::other("the current surface isn't among them"))?;
    if slots[at].3.is_some() {
        return Err(std::io::Error::other("the current surface's entry says it rests"));
    }
    let current_fields = current_fields.ok_or_else(|| std::io::Error::other("no current surface"))?;
    Ok(ReadBack { header, canvas, current: at, current_fields, current_payload, slots })
}

impl ReadBack {
    /// The surfaces and the current one's own fields (its canvas is `canvas`), with the box's
    /// styles. The painting is the first, and only it.
    pub(crate) fn surfaces(&self, tubes: &paint::Palette) -> std::result::Result<(Surfaces, Own), String> {
        let mut slots = Vec::new();
        for (i, (name, kind, f, packed, payload)) in self.slots.iter().enumerate() {
            if (i == 0) != (name == MAIN && *kind == Kind::Painting) {
                return Err("the painting is a multi-surface checkpoint's first surface, and only it".into());
            }
            if *kind == Kind::Plate && !tubes.plate {
                return Err(format!("the surface {name:?} is a plate, and the painting has no plates (no {:?} in its head)", crate::session::PLATE_MARK));
            }
            let rest = match packed {
                Some(p) => {
                    let (mut own, since) = read_fields(f, tubes)?;
                    own.plate = plate_of(*kind, payload)?;
                    Some(Resting { own, canvas: Some(p.clone()), since: since.ok_or("a resting surface without its since=")? })
                }
                None => None,
            };
            slots.push(Slot { name: name.clone(), kind: *kind, rest });
        }
        let (mut own, _) = read_fields(&self.current_fields, tubes)?;
        own.plate = plate_of(self.slots[self.current].1, &self.current_payload)?;
        Ok((Surfaces { slots, current: self.current }, own))
    }

    /// The current surface's plate, as the checkpoint kept it (a replay checkpoint's
    /// reconstruction puts it in place: replay_checkpoint.rs).
    pub(crate) fn current_plate(&self) -> std::result::Result<Option<Rc<paint::plate::Plate>>, String> {
        plate_of(self.slots[self.current].1, &self.current_payload)
    }

    /// The resting surfaces' canvases into `into`, surfaces of the same names, kinds and
    /// current one, whose other fields are kept (a replay checkpoint's reconstruction:
    /// replay_checkpoint.rs, whose Lua made them again with their world views).
    pub(crate) fn canvases_into(self, into: &mut Surfaces) -> std::result::Result<(), String> {
        let same = into.slots.len() == self.slots.len()
            && into.current == self.current
            && into.slots.iter().zip(&self.slots).all(|(a, b)| a.name == b.0 && a.kind == b.1 && a.rest.is_some() == b.3.is_some());
        if !same {
            return Err("the surfaces the checkpoint holds aren't the reconstructed session's".into());
        }
        // (and their plates as it kept them: the reconstruction's tools took their effects from
        // the trace, and changed none)
        for (slot, (_, kind, _, packed, payload)) in into.slots.iter_mut().zip(self.slots) {
            if let (Some(r), Some(p)) = (slot.rest.as_mut(), packed) {
                r.canvas = Some(p);
                r.own.plate = plate_of(kind, &payload)?;
            }
        }
        Ok(())
    }

    /// Into a studio whose canvas is already this one's current surface's (save.rs).
    pub(crate) fn install(self, s: &mut Studio) -> std::result::Result<(), String> {
        let (surfaces, own) = self.surfaces(&s.tubes)?;
        own.put(s);
        s.surfaces = surfaces;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
