//! A live painting: a Lua state over a canvas and the chunks run so far
//! (the log, which is also the replayable program). A chunk that fails
//! changes nothing; a chunk that succeeds is part of the painting for good.

use crate::api::{self, Studio};
use mlua::{Function, Lua, StdLib, Table, Value};
use paint::{Canvas, Held, Palette, Style};
use std::alloc::Layout;
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::ffi::c_void;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::mem::ManuallyDrop;
use std::rc::Rc;
use std::time::{Duration, Instant};

/// Marks the start of a chunk in a session file.
pub const MARK: &str = "--@ chunk";
/// Names the box a painting is painted from, in the head of its session file
/// (before the first chunk). A log without it was painted from the default
/// box (`paint::palette::DEFAULT_BOX`): every log before round 20.
pub const BOX_MARK: &str = "--@ box";
/// Names the engine version a painting is painted with (`paint::ENGINE`), in
/// the head of its session file. A log without it was painted with engine 1:
/// every log before the version was recorded.
pub const ENGINE_MARK: &str = "--@ engine";
/// Marks a sketch: a painting painted at `SKETCH_WIDTH`, not the live width.
/// A log replays at the width it was painted at, whatever its file is named.
pub const SKETCH_MARK: &str = "--@ sketch";
pub const TILES_MARK: &str = "--@ work-tiles";
/// Marks a painting whose `body_of` limbs join the nearest other limb, and
/// only from within their own width (`paint::Outline::body`): every new
/// painting. A log without it replays its limbs joined to the spine.
pub const LIMBS_MARK: &str = "--@ limbs-join-nearest";
/// A sketch's width (px): a quarter of the live width, some 16 times faster,
/// for trying out a composition before the painting.
pub const SKETCH_WIDTH: usize = crate::LIVE_WIDTH / 4;

/// What the engine version (`paint::ENGINE`) changes in the easel's Lua: from
/// engine 3, `pairs` and `next` walk every table in a fixed order, not only
/// tables with object keys (prelude.lua `ordered`), so a failed chunk's
/// rollback puts back all a later chunk can see and needs no rebuild from the
/// log (heap.lua `restore`). Older logs were painted with Lua's own order for
/// tables keyed by values and replay with it.
pub fn canonical_tables(engine: u32) -> bool {
    engine >= 3
}

/// The longest a chunk of a live session may run (the longest of 2,502 painters' chunks on
/// 2026-09-27 took 94 s). A chunk that runs longer is stopped like a failed one: nothing it
/// did is kept. Replays have no limit: a chunk in the log succeeded once and must replay the
/// same on a slower or busier machine.
pub const CHUNK_LIMIT: Duration = Duration::from_secs(600);

/// The head line naming the width a painting is painted at: a log without
/// one was painted at `LEGACY_WIDTH` (or is a sketch, `SKETCH_MARK`).
pub const WIDTH_MARK: &str = "--@ width";
pub const LEGACY_WIDTH: usize = crate::LIVE_WIDTH;
/// The widths a painting may name.
pub const WIDTHS: [usize; 2] = [2400, 4800];

/// A chunk's limit at a width: `CHUNK_LIMIT` at the legacy width, scaled with the pixel count
/// above it, so a chunk that fit at 2400 px still fits at 4800.
pub fn chunk_limit_for(width: usize) -> Duration {
    if width > LEGACY_WIDTH { CHUNK_LIMIT * ((width * width).div_ceil(LEGACY_WIDTH * LEGACY_WIDTH)) as u32 } else { CHUNK_LIMIT }
}
/// Lua instructions between two looks at the clock.
const HOOK_EVERY: u32 = 1_000_000;

pub struct Chunk {
    pub src: String,
}

/// Everything a failed chunk could have changed, taken before it runs.
struct Snap {
    canvas: Option<Canvas>,
    style: Option<Rc<Style>>,
    setup: Option<String>,
    canvas_src: Option<String>,
    seed: u64,
    clock: f64,
    clock0: f64,
    hand: crate::time::Hand,
    board: crate::board::Board,
    /// The last world view made (depth options resolve against it).
    view: Option<crate::world::ViewU>,
    /// The Lua heap (heap.lua's snapshot).
    heap: Table,
    brushes: Vec<(Rc<RefCell<Held>>, Held)>,
    knives: Vec<(Rc<RefCell<paint::Knife>>, paint::Knife)>,
    /// The rags in the hand, as the brushes.
    rags: Vec<(Rc<RefCell<paint::rag::Rag>>, paint::rag::Rag)>,
}

pub struct Session {
    /// Closed by hand in `drop`: the state is created with a fixed hash seed
    /// (see `fixed_lua`), which mlua doesn't own.
    pub lua: ManuallyDrop<Lua>,
    state: *mut mlua::ffi::lua_State,
    pub st: Rc<RefCell<Studio>>,
    pub log: Vec<Chunk>,
    /// A disposable replay (`easel run`, `check`): no snapshot, since a
    /// failure ends it. A live session snapshots before every chunk so a
    /// failure can roll back.
    replay: bool,
    /// When the running chunk must stop (live sessions only), and how long a chunk may run.
    deadline: Rc<Cell<Option<Instant>>>,
    pub chunk_limit: Duration,
    /// heap.lua's snap and restore, prelude.lua's per-chunk reset, the
    /// private objects snapshots skip and its after-chunk check (dropped
    /// before the state is closed).
    heap: Option<(Function, Function)>,
    prelude: Option<(Function, Table, Function, Function)>,
    /// A failed chunk left tables different after restoration. Their entries are
    /// back but maybe not their layout (which Lua gives a program no way to
    /// set), so `pairs` could walk them in another order than a replay of the
    /// log, or `#` find another border: the state is rebuilt from the log
    /// (`rebuild`) before the next chunk. From engine 3 (`canonical_tables`)
    /// only `#` can differ, so only a changed table with more than one border
    /// (holes before its last integer key) makes a session stale.
    pub stale: bool,
    /// Stale because putting back what a failed chunk did failed (not just a layout).
    pub unrestored: bool,
    /// Every global as the last successful chunk left it, with the chunk that last assigned it
    /// (0: the easel's own). `run` updates it by comparing the globals after each chunk with
    /// it, in a live session and a replay alike, so a reopen or rebuild gives the same answer.
    globals: BTreeMap<String, (Value, usize)>,
    /// The easel's own globals (its verbs, prelude.lua's replacements, Lua's libraries).
    own: BTreeMap<String, Value>,
    /// Chunks run before this session's own log (a session restored from a save,
    /// `save::read`): its next chunk is chunk `chunks_before + log.len() + 1`, as at the end
    /// of the log it was saved from. 0 for every other session.
    pub chunks_before: usize,
    /// Creation serials of the state's objects (outlives the state).
    _serials: Box<Serials>,
}

#[derive(Debug)]
pub struct Ran {
    pub out: String,
    /// Seconds the machine took to run it.
    pub secs: f64,
}

impl Session {
    /// A session painting from the default box (tests).
    #[cfg(all(test, tube_box))]
    pub fn new(width: usize) -> mlua::Result<Self> {
        Self::with_box(width, Palette::tube_box())
    }

    /// A session painting from `tubes` (a box: `Palette::named_box`).
    pub fn with_box(width: usize, tubes: Palette) -> mlua::Result<Self> {
        if !hash_seed_fixed() {
            return Err(mlua::Error::runtime(
                "this Lua was built with a random hash seed, so `pairs` order would differ between runs and replays would not be exact; build with CFLAGS=\"-Dluai_makeseed()=0x5eedu\" (see .cargo/config.toml)",
            ));
        }
        let libs = StdLib::TABLE | StdLib::STRING | StdLib::MATH | StdLib::UTF8;
        let (lua, state, serials) = fixed_lua(libs)?;
        // no file or OS access for paintings
        for k in ["dofile", "loadfile", "require", "collectgarbage"] {
            lua.globals().raw_set(k, Value::Nil)?;
        }
        // a private debug library for heap.lua, then gone from the globals
        unsafe {
            mlua::ffi::luaL_requiref(state, c"debug".as_ptr(), mlua::ffi::luaopen_debug, 1);
            mlua::ffi::lua_pop(state, 1);
        }
        let dbg: Table = lua.globals().get("debug")?;
        lua.globals().raw_set("debug", Value::Nil)?;
        let canon = canonical_tables(tubes.engine);
        let probe = if canon { layout_probe(&lua)? } else { None };
        let (snap_f, restore_f): (Function, Function) = lua.load(include_str!("heap.lua")).set_name("heap.lua").call((dbg.clone(), canon, probe))?;
        let id = serials.id_fn(&lua)?;
        let getmt: Function = dbg.get("getmetatable")?;
        let getinfo: Function = dbg.get("getinfo")?;
        // an error of the easel's own, for the prelude to know them by
        let fail = lua.create_function(|_, ()| Err::<(), _>(mlua::Error::runtime("")))?;
        let prelude: (Function, Table, Function, Function) = lua.load(include_str!("prelude.lua")).set_name("prelude.lua").call((id, getmt, getinfo, fail, canon))?;
        let st = Rc::new(RefCell::new(Studio::new(width, tubes)));
        api::install(&lua, st.clone())?;
        let deadline = Rc::new(Cell::new(None::<Instant>));
        let d = deadline.clone();
        let limit_min = chunk_limit_for(width).as_secs() / 60;
        lua.set_hook(mlua::HookTriggers::new().every_nth_instruction(HOOK_EVERY), move |_, _| match d.get() {
            Some(t) if Instant::now() > t => Err(mlua::Error::runtime(format!(
                "the chunk ran longer than {limit_min} minutes and was stopped; nothing it did was kept"
            ))),
            _ => Ok(mlua::VmState::Continue),
        })?;
        let own = global_values(&lua)?;
        let globals = own.iter().map(|(k, v)| (k.clone(), (v.clone(), 0))).collect();
        Ok(Session { lua: ManuallyDrop::new(lua), state, st, log: Vec::new(), replay: false, deadline, chunk_limit: chunk_limit_for(width), heap: Some((snap_f, restore_f)), prelude: Some(prelude), stale: false, unrestored: false, globals, own, chunks_before: 0, _serials: serials })
    }

    /// A session that replays a program from the default box (tests;
    /// `easel run` and `check` use `replay_with`).
    #[cfg(all(test, tube_box))]
    pub fn replay(width: usize) -> mlua::Result<Self> {
        Self::replay_with(width, Palette::tube_box())
    }

    /// A session that replays a program painted from `tubes`: a failed chunk
    /// ends it, so it keeps no snapshot. (`easel run` and `check`: the replay
    /// build.)
    #[cfg(any(feature = "replay", all(test, tube_box)))]
    pub fn replay_with(width: usize, tubes: Palette) -> mlua::Result<Self> {
        let mut s = Self::with_box(width, tubes)?;
        s.replay = true;
        Ok(s)
    }

    /// The box this session paints from.
    #[cfg(feature = "replay")]
    pub fn tube_box(&self) -> Palette {
        (*self.st.borrow().tubes).clone()
    }

    /// Replaying (reopening a log): no snapshot before each chunk, since a failed chunk ends
    /// the reopen anyway, and no time limit. Off again once the log is in.
    pub fn set_replaying(&mut self, on: bool) {
        self.replay = on;
    }

    fn snap(&mut self) -> mlua::Result<Snap> {
        let (snap_f, _) = self.heap.as_ref().unwrap();
        let strings = self.lua.load("return getmetatable('')").eval::<Value>().ok();
        let skip = self.prelude.as_ref().unwrap().1.clone();
        let heap: Table = snap_f.call((skip, self.lua.globals(), strings))?;
        let mut s = self.st.borrow_mut();
        let brushes = s.live_brushes().into_iter().map(|b| {
            let h = b.borrow().clone();
            (b, h)
        }).collect();
        let knives = s.live_knives().into_iter().map(|k| {
            let h = k.borrow().clone();
            (k, h)
        }).collect();
        let rags = s.live_rags().into_iter().map(|r| {
            let v = *r.borrow();
            (r, v)
        }).collect();
        Ok(Snap { canvas: s.canvas.clone(), style: s.style.clone(), setup: s.setup.clone(), canvas_src: s.canvas_src.clone(), seed: s.seed, clock: s.clock, clock0: s.clock0, hand: s.hand.clone(), board: s.board.clone(), view: s.view.clone(), heap, brushes, knives, rags })
    }

    /// Put everything back as it was at `snap`. Returns how many Lua tables
    /// still differ in contents, traversal order or raw length after restoration (see `stale`).
    fn restore(&mut self, snap: &Snap) -> mlua::Result<usize> {
        let (_, restore_f) = self.heap.as_ref().unwrap();
        let mismatches = restore_f.call::<usize>(snap.heap.clone())?;
        for (b, h) in &snap.brushes {
            *b.borrow_mut() = h.clone();
        }
        for (k, h) in &snap.knives {
            *k.borrow_mut() = h.clone();
        }
        for (r, v) in &snap.rags {
            *r.borrow_mut() = *v;
        }
        let mut s = self.st.borrow_mut();
        s.canvas = snap.canvas.clone();
        s.style = snap.style.clone();
        s.setup = snap.setup.clone();
        s.canvas_src = snap.canvas_src.clone();
        s.seed = snap.seed;
        s.clock = snap.clock;
        s.clock0 = snap.clock0;
        s.hand = snap.hand.clone();
        s.board = snap.board.clone();
        s.view = snap.view.clone();
        Ok(mismatches)
    }

    /// Replace the state with a replay of the log in a fresh one: the state
    /// the log gives, exactly (see `stale`). Takes as long as a reopen.
    pub fn rebuild(&mut self) -> Result<(), String> {
        self.rebuild_with_progress(|_, _| {})
    }

    /// Reports completed chunks, including zero before starting the replay.
    pub fn rebuild_with_progress(&mut self, mut progress: impl FnMut(usize, usize)) -> Result<(), String> {
        progress(0, self.log.len());
        let (width, tubes) = {
            let s = self.st.borrow();
            (s.width, (*s.tubes).clone())
        };
        let mut fresh = Session::with_box(width, tubes).map_err(|e| e.to_string())?;
        fresh.chunk_limit = self.chunk_limit;
        fresh.replay = true;
        for (i, c) in self.log.iter().enumerate() {
            fresh.run(&c.src).map_err(|e| format!("rebuilding from the log failed at chunk {}: {e}", i + 1))?;
            progress(i + 1, self.log.len());
        }
        fresh.replay = self.replay;
        *self = fresh;
        Ok(())
    }

    /// Run one chunk. On error nothing it did survives (canvas, globals,
    /// brushes, clock) and it is not logged; on success it is in the log
    /// for good.
    pub fn run(&mut self, src: &str) -> Result<Ran, String> {
        let src = src.trim_end().to_string();
        if src.lines().any(|l| l.trim_start().starts_with(MARK)) {
            return Err(format!("a chunk can't contain a line starting with {MARK:?} (it separates chunks in the log)"));
        }
        if src.trim().is_empty() {
            return Err("empty chunk".into());
        }
        if self.stale {
            self.rebuild()?;
        }
        let mut snap = if !self.replay { Some(self.snap().map_err(|e| e.to_string())?) } else { None };
        let n = (self.chunks_before + self.log.len()) as u64 + 1;
        self.prelude.as_ref().unwrap().0.call::<()>(()).map_err(|e| e.to_string())?;
        self.st.borrow_mut().begin(n);
        let t0 = Instant::now();
        // text only: mlua would take a chunk starting with Lua's binary signature as bytecode
        let chunk = self.lua.load(src.as_str()).set_name(format!("chunk {n}")).set_mode(mlua::chunk::ChunkMode::Text);
        let (check, guard) = { let p = self.prelude.as_ref().unwrap(); (p.2.clone(), p.3.clone()) };
        let deadline = (!self.replay).then(|| t0 + self.chunk_limit);
        self.deadline.set(deadline);
        // Everything that can fail before the chunk is kept is inside this guard: the chunk,
        // prelude.lua's check, putting its hand time on the clock and reading the globals it
        // left. A failure anywhere in it is a failed chunk, rolled back below.
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // through prelude.lua's guard: an error value is shown without its address
            guard.call::<()>(chunk.into_function()?)?;
            check.call::<()>(())?;
            // the hand time the chunk spent goes on the clock before it ends
            crate::time::flush(&self.st, true);
            self.next_globals(n as usize)
        }));
        self.deadline.set(None);
        let secs = t0.elapsed().as_secs_f64();
        let out = self.st.borrow().out.clone();
        let (mut fail, globals) = match r {
            Ok(Ok(g)) => (None, Some(g)),
            Ok(Err(e)) => (Some(clean_error(&e.to_string())), None),
            Err(p) => (Some(format!("engine panic: {}", panic_text(&p))), None),
        };
        // Rollback must finish even after the chunk exhausted its deadline. The
        // private restore uses raw operations and runs no painter code; collection
        // below reinstates the deadline because it can run Lua work.
        // put back what a failed chunk did (no painter code runs in restore)
        if let Some(e) = fail.as_mut()
            && let Some(snap) = snap.take()
        {
            // A rollback that fails may have left anything half put back: the state is
            // no longer one the log gives, so it is rebuilt from the log (`stale`) before
            // anything paints on it.
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.restore(&snap))) {
                Ok(Ok(mismatches)) => self.stale = mismatches > 0,
                r => {
                    self.stale = true;
                    self.unrestored = true;
                    let why = match r {
                        Ok(Err(e)) => e.to_string(),
                        Err(p) => format!("engine panic: {}", panic_text(&p)),
                        Ok(Ok(_)) => unreachable!(),
                    };
                    e.push_str(&format!("\n(the easel couldn't put back what the chunk did ({why}), so it rebuilds the painting from its log before it paints again)"));
                }
            }
        }
        // before collecting: the values the chunk replaced aren't held here any more
        if let Some(g) = globals {
            self.globals = g;
        }
        // masks and brushes hold memory Lua can't see: collect between chunks, without the
        // snapshot, as a replay does, and within the chunk's time
        drop(snap);
        self.deadline.set(deadline);
        let _ = self.lua.gc_collect();
        self.deadline.set(None);
        if let Some(e) = fail {
            let mut msg = out;
            msg.push_str(&e);
            return Err(msg);
        }
        self.log.push(Chunk { src });
        Ok(Ran { out, secs })
    }

    pub fn canvas(&self) -> Option<std::cell::Ref<'_, Canvas>> {
        std::cell::Ref::filter_map(self.st.borrow(), |s| s.canvas.as_ref()).ok()
    }

    /// The session as a replayable program.
    pub fn program(&self, name: &str) -> String {
        let mut s = String::new();
        let _ = writeln!(s, "-- easel session {name:?}: a painting replayed chunk by chunk.");
        let _ = writeln!(s, "-- Each {MARK:?} line starts one chunk as it was run at the easel.");
        let tubes = self.st.borrow().tubes.name;
        if paint::palette::default_box() != Some(tubes) {
            let _ = writeln!(s, "{BOX_MARK} {tubes}");
        }
        let engine = self.st.borrow().tubes.engine;
        if engine != 1 {
            let _ = writeln!(s, "{ENGINE_MARK} {engine}");
        }
        if self.st.borrow().tubes.quarter_tiles {
            let _ = writeln!(s, "{TILES_MARK} quarter");
        }
        if self.st.borrow().tubes.adaptive_sweep {
            let _ = writeln!(s, "--@ adaptive-sweep");
        }
        if self.st.borrow().tubes.limbs_join_nearest {
            let _ = writeln!(s, "{LIMBS_MARK}");
        }
        if self.st.borrow().width == SKETCH_WIDTH {
            let _ = writeln!(s, "{SKETCH_MARK}");
        }
        // only a painting width other than the legacy one is named (test sessions use small widths)
        let width = self.st.borrow().width;
        if width != LEGACY_WIDTH && WIDTHS.contains(&width) {
            let _ = writeln!(s, "{WIDTH_MARK} {width}");
        }
        for (i, c) in self.log.iter().enumerate() {
            let _ = writeln!(s, "\n{MARK} {}", i + 1);
            s.push_str(&c.src);
            s.push('\n');
        }
        s
    }

    /// The globals as chunk `n` left them: the ones it set, changed or removed are its.
    fn next_globals(&self, n: usize) -> mlua::Result<BTreeMap<String, (Value, usize)>> {
        let now = global_values(&self.lua)?;
        let mut next = BTreeMap::new();
        for (k, v) in now {
            let chunk = match self.globals.get(&k) {
                Some((old, c)) if same(old, &v) => *c,
                // back to the easel's own value: the easel's again
                _ if self.own.get(&k).is_some_and(|o| same(o, &v)) => 0,
                _ => n,
            };
            next.insert(k, (v, chunk));
        }
        Ok(next)
    }

    /// The painting's globals, the most recently assigned last: one line each,
    /// `<chunk>\t<name>\t<what it holds>`, for a name a chunk can use (not the easel's own).
    pub fn globals(&self) -> String {
        let mut g: Vec<_> = self.globals.iter().filter(|(k, (_, c))| *c > 0 && is_name(k)).collect();
        g.sort_by_key(|(k, (_, c))| (*c, *k));
        let mut s = String::new();
        for (k, (v, c)) in g {
            let _ = writeln!(s, "{c}\t{k}\t{}", describe(v));
        }
        s
    }

    /// The piles the painting's globals hold, in `globals`' order: (name, recipe, the paint
    /// a brush loads from it before the hand's unevenness). Reads the userdata in Rust: runs
    /// no Lua and changes nothing (the palette look, look.rs `palette`).
    pub fn piles(&self) -> Vec<(String, String, paint::Paint)> {
        let mut g: Vec<_> = self.globals.iter().filter(|(k, (_, c))| *c > 0 && is_name(k)).collect();
        g.sort_by_key(|(k, (_, c))| (*c, *k));
        g.into_iter()
            .filter_map(|(k, (v, _))| match v {
                Value::UserData(u) => u.borrow::<api::PileU>().ok().map(|p| (k.clone(), p.recipe(), p.paint())),
                _ => None,
            })
            .collect()
    }

    /// What the global `name` puts on a knife held up to the canvas (the look
    /// `--hold`): a knife's own load, with its blade's length and how full it
    /// is, or a pile as its heap is on the board now, as a fresh full load on
    /// a blade 30 units long. Reads only: no Lua runs and nothing changes.
    pub fn held(&self, name: &str) -> Result<(paint::Paint, f32, f32), String> {
        let found = self.globals.get(name).filter(|(_, c)| *c > 0).map(|(v, _)| v);
        if let Some(Value::UserData(u)) = found {
            if let Ok(k) = u.borrow::<api::KnifeU>() {
                let k = k.k.borrow();
                return match k.paint() {
                    Some(p) => Ok((p, k.width, k.fullness().clamp(0.0, 1.0))),
                    None => Err(format!("look --hold {name}: the knife is clean; load it first (k:load(pile))")),
                };
            }
            if let Ok(p) = u.borrow::<api::PileU>() {
                // Project the same fresh visit as a knife load, without changing
                // the live board's dirt, ordering or last-carried paint.
                let st = self.st.borrow();
                let mut board = st.board.clone();
                board.visit(p.heap, 0.0);
                let mut projected = p.clone();
                if let Some(h) = board.heap(p.heap).filter(|h| h.changed) {
                    projected.mix = st.tubes.pile(h.fractions());
                    projected.mix.solvent = h.solvent;
                    projected.mix.oil_rate = h.oil_rate;
                    projected.medium = h.medium;
                    projected.thinner = projected.thinner.map(|_| h.thinner);
                }
                return Ok((projected.paint(), 30.0, 1.0));
            }
        }
        let mut names: Vec<&str> = self
            .globals
            .iter()
            .filter(|(k, (v, c))| *c > 0 && is_name(k) && matches!(v, Value::UserData(u) if u.borrow::<api::KnifeU>().is_ok() || u.borrow::<api::PileU>().is_ok()))
            .map(|(k, _)| k.as_str())
            .collect();
        names.sort();
        Err(format!("look --hold {name}: no global of that name holds a knife or a pile ({})", if names.is_empty() { "there is none yet".to_string() } else { names.join(", ") }))
    }

    /// The ground's color, from the `canvas{}` layers: each layer's paint over the raw linen
    /// and the layers under it at its thickness (`Paint::over`; the weave left out).
    pub fn ground_color(&self) -> Option<paint::Rgb> {
        let st = self.st.borrow();
        let sty = st.style.as_ref()?;
        let mut c = sty.raw;
        for g in &sty.ground {
            c = paint::Paint::new(g.color, g.hiding, g.stiff).over(c, g.um / paint::COAT_UM);
        }
        Some(c)
    }

    pub fn status(&self) -> String {
        let s = self.st.borrow();
        format!("{} chunks · {}px · {}", self.log.len(), s.width, s.setup.as_deref().unwrap_or("no canvas yet"))
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        // everything holding references into the state goes first
        self.heap = None;
        self.prelude = None;
        self.globals.clear();
        self.own.clear();
        let _ = self.lua.gc_collect();
        unsafe {
            ManuallyDrop::drop(&mut self.lua);
            mlua::ffi::lua_close(self.state);
        }
    }
}

/// The globals with a string key, read raw.
fn global_values(lua: &Lua) -> mlua::Result<BTreeMap<String, Value>> {
    let mut m = BTreeMap::new();
    lua.globals().for_each(|k: Value, v: Value| {
        if let Value::String(k) = k {
            m.insert(k.to_string_lossy(), v);
        }
        Ok(())
    })?;
    Ok(m)
}

/// Lua's rawequal, except that a NaN is the same as itself.
fn same(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.to_bits() == y.to_bits(),
        (Value::Integer(_), Value::Number(_)) | (Value::Number(_), Value::Integer(_)) => false,
        _ => a == b,
    }
}

fn is_name(k: &str) -> bool {
    let mut c = k.chars();
    c.next().is_some_and(|f| f == '_' || f.is_ascii_alphabetic()) && c.all(|c| c == '_' || c.is_ascii_alphanumeric())
}

/// What a global holds, in a few words. Runs no painter code: a table's is counted raw, and
/// only the engine's own `__tostring` describes a userdata.
fn describe(v: &Value) -> String {
    match v {
        Value::Nil => "nil".into(),
        Value::Boolean(b) => b.to_string(),
        Value::Integer(i) => format!("number {i}"),
        Value::Number(x) => format!("number {x}"),
        Value::String(s) => {
            let t = s.to_string_lossy();
            let n = t.chars().count();
            if n <= 60 { format!("string {t:?}") } else { format!("string {:?}… ({n} characters)", t.chars().take(40).collect::<String>()) }
        }
        Value::Table(t) => {
            let mut n = 0;
            let _ = t.for_each(|_: Value, _: Value| {
                n += 1;
                Ok(())
            });
            format!("table with {n} {}", if n == 1 { "entry" } else { "entries" })
        }
        Value::Function(f) => {
            let i = f.info();
            let src = i.source.as_deref().map(|s| s.trim_start_matches(['=', '@']));
            match (i.what, src, i.line_defined) {
                ("C", ..) => "function (the easel's)".into(),
                (_, Some(src), Some(line)) if src.starts_with("chunk ") => format!("function ({src}, line {line})"),
                _ => "function".into(),
            }
        }
        Value::UserData(u) => {
            let shown = u.metatable().and_then(|m| m.get::<Value>("__tostring")).ok().and_then(|f| match f {
                Value::Function(f) => f.call::<String>(u.clone()).ok(),
                _ => None,
            });
            shown.unwrap_or_else(|| {
                let t = u.type_name().map(|t| t.to_string_lossy()).unwrap_or_else(|_| "userdata".into());
                t.strip_suffix('U').unwrap_or(&t).to_lowercase()
            })
        }
        o => o.type_name().into(),
    }
}

/// Creation serials of a state's tables, closures, userdata and threads,
/// kept by its allocator. Lua hashes these objects by address, which
/// differs between processes; their serials give prelude.lua's `pairs` and
/// `next` an order that doesn't: the order the program created them in.
/// (Relative order is all that counts, so objects made by failed chunks,
/// snapshots or mlua itself do no harm.)
///
/// Every block carries a 16-byte header: the serial and how to find it. A
/// table or closure pointer is its block, so its serial sits just before
/// it; a userdata or thread pointer lies inside its block, so those blocks
/// are also listed by address.
pub struct Serials {
    next: Cell<i64>,
    /// userdata and thread blocks: user address -> (serial, size)
    inner: RefCell<BTreeMap<usize, (i64, usize)>>,
}

const HDR: usize = 16;
const PLAIN: i64 = 0;
const AT_START: i64 = 1;
const LISTED: i64 = 2;

fn layout(size: usize) -> Layout {
    // SAFETY (for callers): 16 is a power of two and Lua's sizes are far from isize::MAX
    unsafe { Layout::from_size_align_unchecked(size + HDR, HDR) }
}

unsafe extern "C" fn counting_alloc(ud: *mut c_void, ptr: *mut c_void, osize: usize, nsize: usize) -> *mut c_void {
    use mlua::ffi::{LUA_TFUNCTION, LUA_TTABLE, LUA_TTHREAD, LUA_TUSERDATA};
    // SAFETY: `ud` is the session's boxed Serials, which outlives the state;
    // Lua passes the block's true size as `osize` whenever `ptr` is a block,
    // and every block has the header this function put before it.
    unsafe {
        let h = &*(ud as *const Serials);
        if ptr.is_null() {
            if nsize == 0 {
                return std::ptr::null_mut();
            }
            let b = std::alloc::alloc(layout(nsize));
            if b.is_null() {
                return b as *mut c_void;
            }
            // a new object: `osize` is its type
            let kind = match osize as i32 {
                LUA_TTABLE | LUA_TFUNCTION => AT_START,
                LUA_TUSERDATA | LUA_TTHREAD => LISTED,
                _ => PLAIN,
            };
            let mut n = 0;
            if kind != PLAIN {
                n = h.next.get();
                h.next.set(n + 1);
            }
            let hd = b as *mut i64;
            *hd = n;
            *hd.add(1) = kind;
            let p = b.add(HDR);
            if kind == LISTED {
                h.inner.borrow_mut().insert(p as usize, (n, nsize));
            }
            return p as *mut c_void;
        }
        let b = (ptr as *mut u8).sub(HDR);
        if nsize == 0 {
            if *(b as *const i64).add(1) == LISTED {
                h.inner.borrow_mut().remove(&(ptr as usize));
            }
            std::alloc::dealloc(b, layout(osize));
            return std::ptr::null_mut();
        }
        // only arrays and buffers are reallocated, never objects
        let nb = std::alloc::realloc(b, layout(osize), nsize + HDR);
        if nb.is_null() { nb as *mut c_void } else { nb.add(HDR) as *mut c_void }
    }
}

impl Serials {
    /// `id(v)`: the serial of a table, closure, userdata or thread; nil for
    /// values and light C functions (library functions have no block).
    fn id_fn(&self, lua: &Lua) -> mlua::Result<Function> {
        let me = self as *const Serials;
        lua.create_function(move |_, v: Value| {
            let p = v.to_pointer() as usize;
            Ok(match &v {
                Value::Function(f) if f.info().what == "C" && f.info().num_upvalues == 0 => None,
                // SAFETY: a live table or closure is a block with a header
                Value::Table(_) | Value::Function(_) => Some(unsafe { *((p - HDR) as *const i64) }),
                Value::UserData(_) | Value::Thread(_) => {
                    // SAFETY: the Serials outlive the state, so this function
                    let inner = unsafe { &*me }.inner.borrow();
                    inner.range(..=p).next_back().filter(|(a, (_, len))| p < *a + *len).map(|(_, (n, _))| *n)
                }
                _ => None,
            })
        })
    }
}

/// A Lua state with the hash seed Lua's own `luaL_newstate` gives it, which
/// is constant when Lua is built with `-Dluai_makeseed()=...` (see
/// .cargo/config.toml). mlua 0.12 seeds its states from `arc4random` on
/// macOS, so `pairs` order (and the layout of any hash table) would differ
/// between a live session and its replay. Its allocator numbers objects
/// (see `Serials`).
fn fixed_lua(libs: StdLib) -> mlua::Result<(Lua, *mut mlua::ffi::lua_State, Box<Serials>)> {
    use mlua::ffi;
    let serials = Box::new(Serials { next: Cell::new(1), inner: RefCell::new(BTreeMap::new()) });
    unsafe {
        let ud = &*serials as *const Serials as *mut c_void;
        let state = ffi::lua_newstate(counting_alloc, ud, ffi::luaL_makeseed_(std::ptr::null_mut()));
        if state.is_null() {
            return Err(mlua::Error::runtime("could not create a Lua state"));
        }
        ffi::luaL_requiref(state, c"_G".as_ptr(), ffi::luaopen_base, 1);
        ffi::lua_pop(state, 1);
        let lua = Lua::get_or_init_from_ptr(state).clone();
        lua.load_std_libs(libs)?;
        Ok((lua, state, serials))
    }
}

/// The head of Lua 5.5's `Table` (lobject.h): what `#t` reads besides the
/// entries is the array part's size and the length hint stored at `array`
/// (ltable.h `lenhint`, ltable.c `luaH_getn`).
#[repr(C)]
struct TableHead {
    next: *const c_void,
    tt: u8,
    marked: u8,
    flags: u8,
    lsizenode: u8,
    asize: u32,
    array: *const u32,
}

/// A table's array size and length hint, read without changing them (`#t`
/// would move the hint): with its entries, all `#t` depends on. heap.lua
/// compares them across a failed chunk for tables with more than one border.
fn table_layout(t: &Table) -> (u32, u32) {
    // SAFETY: for a table `lua_topointer` gives its `Table` (lapi.c), alive while `t` is;
    // `array` points at the hint when the array part isn't empty (ltable.c `luaH_resize`).
    unsafe {
        let h = &*(t.to_pointer() as *const TableHead);
        let hint = if h.asize > 0 && !h.array.is_null() { *h.array } else { 0 };
        (h.asize, hint)
    }
}

/// Whether `table_layout` reads this Lua's tables right, tried once in a
/// Lua of its own: known array sizes and hints (ltable.c `luaH_resize` sets
/// the hint to half the size; `#t` of a full array part sets it to the size).
fn table_layout_reads_right() -> bool {
    static OK: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *OK.get_or_init(|| {
        let probe = || -> mlua::Result<bool> {
            let (lua, state, _serials) = fixed_lua(StdLib::NONE)?;
            let t: Table = lua.load("return {1, 2, 3, 4, 5, 6, 7, 8}").eval()?;
            let before = table_layout(&t);
            let n = t.raw_len();
            let after = table_layout(&t);
            let e = lua.create_table()?;
            let empty = table_layout(&e);
            drop((t, e));
            drop(lua);
            unsafe { mlua::ffi::lua_close(state) };
            Ok((before, n, after, empty) == ((8, 4), 8, (8, 8), (0, 0)))
        };
        probe().unwrap_or(false)
    })
}

/// `table_layout` for heap.lua, if it reads this Lua's tables right;
/// otherwise none, and heap.lua takes every table with more than one border
/// as moved by a failed chunk.
fn layout_probe(lua: &Lua) -> mlua::Result<Option<Function>> {
    if !table_layout_reads_right() {
        return Ok(None);
    }
    Ok(Some(lua.create_function(|_, t: Table| Ok(table_layout(&t)))?))
}

/// The order a fresh Lua walks a table of string keys in: fixed when Lua
/// is built with a constant `luai_makeseed`, different run to run otherwise.
pub fn hash_probe() -> String {
    let probe = || -> mlua::Result<String> {
        let (lua, state, _serials) = fixed_lua(StdLib::NONE)?;
        let t = lua.create_table()?;
        for i in 0..32 {
            t.set(format!("k{}", i * 7919), i)?;
        }
        let mut o = String::new();
        for kv in t.pairs::<String, i64>() {
            o.push_str(&kv?.1.to_string());
            o.push(',');
        }
        drop(t);
        drop(lua);
        unsafe { mlua::ffi::lua_close(state) };
        Ok(o)
    };
    probe().unwrap_or_default()
}

/// `hash_probe()` with the seed in .cargo/config.toml.
const HASH_PROBE: &str = "31,24,5,8,28,22,12,6,19,26,4,20,1,3,30,11,0,15,14,10,9,17,7,27,2,18,13,21,16,29,23,25,";

fn hash_seed_fixed() -> bool {
    hash_probe() == HASH_PROBE
}

/// What a caught panic said.
fn panic_text(p: &Box<dyn std::any::Any + Send>) -> String {
    p.downcast_ref::<String>().cloned().or(p.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default()
}

/// Lua errors carry a traceback; the painter needs the first lines.
fn clean_error(e: &str) -> String {
    let mut out = Vec::new();
    for l in e.lines() {
        if l.trim_start().starts_with("stack traceback") {
            break;
        }
        out.push(l);
    }
    unaddressed(&out.join("\n"))
}

/// `<name>: 0x<8+ hex digits>` (Lua's text for an object without __tostring, which Lua
/// makes of an error object raised in a painter's function the easel called) as
/// `<name>: (hidden)`, as prelude.lua's `unaddressed`: the address differs from process
/// to process.
fn unaddressed(e: &str) -> String {
    let mut out = String::with_capacity(e.len());
    let mut rest = e;
    while let Some(i) = rest.find(": 0x") {
        let hex = rest[i + 4..].bytes().take_while(u8::is_ascii_hexdigit).count();
        out.push_str(&rest[..i + 2]);
        if hex >= 8 {
            out.push_str("(hidden)");
            rest = &rest[i + 4 + hex..];
        } else {
            rest = &rest[i + 2..];
        }
    }
    out.push_str(rest);
    out
}

/// The box a session file names in its head (`BOX_MARK` lines before the
/// first chunk): None if it names none (it was painted from the default box).
pub fn logged_box(text: &str) -> Result<Option<String>, String> {
    let mut found = None;
    for l in text.lines() {
        let l = l.trim();
        if l.starts_with(MARK) {
            break;
        }
        if let Some(rest) = l.strip_prefix(BOX_MARK) {
            let name = rest.trim();
            if name.is_empty() || !rest.starts_with(' ') {
                return Err(format!("the log's line {l:?} names no box ({BOX_MARK} <name>)"));
            }
            if found.is_some() {
                return Err(format!("the log names its box twice ({BOX_MARK} lines)"));
            }
            found = Some(name.to_string());
        }
    }
    Ok(found)
}

/// The engine version a session file names in its head (its `ENGINE_MARK`
/// line before the first chunk): 1 if it names none.
pub fn logged_engine(text: &str) -> Result<u32, String> {
    let mut found = None;
    for l in text.lines() {
        let l = l.trim();
        if l.starts_with(MARK) {
            break;
        }
        if let Some(rest) = l.strip_prefix(ENGINE_MARK) {
            let v = rest.strip_prefix(' ').and_then(|v| v.trim().parse::<u32>().ok()).filter(|&v| paint::supported_engine(v));
            let Some(v) = v else {
                return Err(format!("the log's line {l:?} names no engine this easel has ({ENGINE_MARK} <1 to {}, or {}>)", paint::ENGINE, paint::engine7::ENGINE));
            };
            if found.is_some() {
                return Err(format!("the log names its engine twice ({ENGINE_MARK} lines)"));
            }
            found = Some(v);
        }
    }
    Ok(found.unwrap_or(1))
}

/// The width a session file names in its head (its `WIDTH_MARK` line before
/// the first chunk): `LEGACY_WIDTH` if it names none.
pub fn logged_width(text: &str) -> Result<usize, String> {
    let mut found = None;
    for l in text.lines() {
        let l = l.trim();
        if l.starts_with(MARK) {
            break;
        }
        if let Some(rest) = l.strip_prefix(WIDTH_MARK) {
            let v = rest.strip_prefix(' ').and_then(|v| v.trim().parse::<usize>().ok()).filter(|v| WIDTHS.contains(v));
            let Some(v) = v else {
                return Err(format!("the log's line {l:?} names no width this easel paints at ({WIDTH_MARK} <{}>)", WIDTHS.map(|w| w.to_string()).join("|")));
            };
            if found.is_some() {
                return Err(format!("the log names its width twice ({WIDTH_MARK} lines)"));
            }
            found = Some(v);
        }
    }
    Ok(found.unwrap_or(LEGACY_WIDTH))
}

/// Whether a session file's head marks it a sketch (a `SKETCH_MARK` line
/// before the first chunk).
pub fn logged_sketch(text: &str) -> bool {
    text.lines().map(str::trim).take_while(|l| !l.starts_with(MARK)).any(|l| l == SKETCH_MARK)
}

pub fn logged_quarter_tiles(text: &str) -> Result<bool, String> {
    let mut found = false;
    for l in text.lines().map(str::trim).take_while(|l| !l.starts_with(MARK)) {
        if let Some(rest) = l.strip_prefix(TILES_MARK) {
            if rest.strip_prefix(' ').map(str::trim) != Some("quarter") || found {
                return Err(format!("the log's line {l:?} names no work tiles this easel has, or names them twice ({TILES_MARK} quarter, at most once)"));
            }
            found = true;
        }
    }
    if found && logged_engine(text)? != 5 {
        return Err(format!("the log asks for quarter work tiles ({TILES_MARK} quarter), but only engine 5 paints with them"));
    }
    Ok(found)
}

/// Where the box of a new painting is set: a file `box` next to the easel's
/// executable (one line, the box's name; a studio ships it as
/// `<studio>/bin/box`), else `EASEL_BOX`. None: neither is set (the default
/// box). The two naming different boxes is an error.
pub fn configured_box() -> Result<Option<(String, String)>, String> {
    let file = std::env::current_exe().ok().and_then(|e| e.canonicalize().ok()).and_then(|e| e.parent().map(|d| d.join("box")));
    let from_file = match file {
        Some(f) if f.exists() => {
            let text = std::fs::read_to_string(&f).map_err(|e| format!("{}: {e}", f.display()))?;
            let name = text.trim();
            if name.is_empty() || name.contains('\n') {
                return Err(format!("{} holds one line, the name of a box (it holds {text:?})", f.display()));
            }
            Some((name.to_string(), f.display().to_string()))
        }
        _ => None,
    };
    let from_env = std::env::var("EASEL_BOX").ok().map(|v| v.trim().to_string()).filter(|v| !v.is_empty()).map(|v| (v, "EASEL_BOX".to_string()));
    match (from_file, from_env) {
        (Some(f), Some(e)) if f.0 != e.0 => Err(format!("{} says box {:?} but EASEL_BOX says {:?}: set one, or both the same", f.1, f.0, e.0)),
        (Some(f), _) => Ok(Some(f)),
        (None, e) => Ok(e),
    }
}

/// The box called `name`, or an error naming the boxes this easel has.
pub fn find_box(name: &str) -> Result<Palette, String> {
    Palette::named_box(name).ok_or_else(|| format!("this easel has no box {name:?} (its boxes: {})", box_list()))
}

/// The boxes this easel has, quoted: `"tube box", "sargent"`.
fn box_list() -> String {
    Palette::box_names().iter().map(|b| format!("{b:?}")).collect::<Vec<_>>().join(", ")
}

/// The box a painting is painted from. A new painting (`log` None) takes the
/// configured box (`configured_box`), else the default (in a painter's build
/// for one box, which has no default box: that box). An existing one takes
/// the box its log names (none: the default), whatever easel replays it; if a
/// box is configured too, it must be the same one. The box carries the
/// engine version the painting is painted with: a new one engine 7,
/// an existing one its log's (`logged_engine`). New paintings' `body_of`
/// joins limbs to the nearest limb; an existing one's as its log says
/// (`LIMBS_MARK`).
pub fn box_for(log: Option<&str>) -> Result<Palette, String> {
    let configured = configured_box()?;
    let Some(text) = log else {
        let mut b = find_box(configured.as_ref().map_or(Palette::fallback_box(), |c| c.0.as_str()))?;
        b.engine = paint::engine7::ENGINE;
        b.limbs_join_nearest = true;
        return Ok(b);
    };
    let engine = logged_engine(text)?;
    let adaptive_sweep = text.lines().map(str::trim).take_while(|l| !l.starts_with(MARK)).any(|l| l == "--@ adaptive-sweep");
    if adaptive_sweep && engine != 5 {
        return Err("the log's adaptive sweep switch requires engine 5".into());
    }
    let logged = logged_box(text)?;
    let Some(name) = logged.as_deref().or(paint::palette::default_box()) else {
        return Err(format!("the log names no box, so it was painted from the default box, which this easel doesn't have (its boxes: {}); nothing ran", box_list()));
    };
    if let Some((c, from)) = &configured
        && c != name
    {
        let log_says = match &logged {
            Some(n) => format!("the log was painted from the box {n:?} (its {BOX_MARK} line)"),
            None => format!("the log names no box, so it was painted from the default box {name:?}"),
        };
        return Err(format!("{log_says}, but {from} says {c:?}: a painting is replayed with the box it was painted from; nothing ran"));
    }
    let mut b = find_box(name)?;
    b.engine = engine;
    b.quarter_tiles = logged_quarter_tiles(text)?;
    b.adaptive_sweep = adaptive_sweep;
    b.limbs_join_nearest = text.lines().map(str::trim).take_while(|l| !l.starts_with(MARK)).any(|l| l == LIMBS_MARK);
    Ok(b)
}

/// Split a session file into chunks.
pub fn parse_program(text: &str) -> Vec<String> {
    let mut chunks: Vec<String> = Vec::new();
    let mut cur: Option<String> = None;
    for l in text.lines() {
        if l.trim_start().starts_with(MARK) {
            if let Some(c) = cur.take() {
                chunks.push(c);
            }
            cur = Some(String::new());
        } else if let Some(c) = cur.as_mut() {
            c.push_str(l);
            c.push('\n');
        }
    }
    if let Some(c) = cur {
        chunks.push(c);
    }
    chunks.into_iter().map(|c| c.trim_end().to_string()).filter(|c| !c.trim().is_empty()).collect()
}

/// The checkout the easel works in (session logs in `paintings/lua`, renders
/// in `out/`) in the replay build: `EASEL_ROOT` if set, else the nearest directory at or above
/// the working directory that holds `crates/easel/Cargo.toml`, else the
/// checkout this binary was built from. Looking from the working directory
/// keeps git worktrees apart: a binary built in (or copied from) another
/// checkout still writes into the worktree it's run in.
#[cfg(feature = "replay")]
pub fn root() -> PathBuf {
    if let Some(r) = std::env::var_os("EASEL_ROOT").filter(|r| !r.is_empty()) {
        let r = PathBuf::from(r);
        return r.canonicalize().unwrap_or(r);
    }
    if let Ok(cwd) = std::env::current_dir() {
        let found = cwd.ancestors().find(|d| d.join("crates/easel/Cargo.toml").is_file()).map(Path::to_path_buf);
        if let Some(r) = found {
            return r.canonicalize().unwrap_or(r);
        }
    }
    let r = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    r.canonicalize().unwrap_or(r)
}

/// The studio the painter build works in: the parent of the directory
/// holding the executable (the studio ships it as `<studio>/bin/easel`),
/// wherever it is run from. Nothing in the environment moves it.
#[cfg(not(feature = "replay"))]
pub fn root() -> PathBuf {
    let exe = std::env::current_exe().and_then(|e| e.canonicalize()).unwrap_or_else(|e| panic!("easel: cannot locate its own executable: {e}"));
    exe.parent().and_then(Path::parent).unwrap_or_else(|| panic!("easel: {} is not inside <studio>/bin", exe.display())).to_path_buf()
}

#[cfg(all(test, feature = "replay"))]
mod tests {
    use super::*;

    /// Paintings from before the engine version was recorded (no `--@ engine`
    /// line) replay as engine 1, bit for bit as they did (recorded at 08f6324,
    /// before engine 2, at a small width to keep the test short): two
    /// studios' paintings (paint-studio-6399ad and -db6324), hand-timed, with
    /// paint worked over drying paint.
    // Replays two studios' whole paintings: not run by any test command (the overnight
    // plan of 2026-10-04 forbids painting replays in tests). scripts/tests/old_logs.sh
    // checks the same rule on a tiny engine-1 log and on the first three chunks of each
    // of these two logs, at 128 to 160 px; notes/speed/SKIPPED.md.
    #[test]
    #[ignore = "replays whole paintings: never run"]
    fn logs_without_an_engine_line_replay_as_before() {
        for (name, want) in [("studio_6399ad", "4f3abae7eb080221"), ("studio_db6324", "009ec933d082a252")] {
            let (s, _) = replay_fixture(&format!("engine1/{name}"));
            assert_eq!(canvas_hash(&s), want, "{name} no longer replays as it did");
        }
    }

    /// An engine-2 painting replays as engine 2, bit for bit as it did before
    /// engine 3 (recorded at 7b80cb0, round 23): a test sheet of lead white
    /// and bone black laid at different times over five days, lifted at
    /// every stage of drying and painted across, with waits of hours and
    /// days. Its log goes on naming engine 2.
    #[test]
    fn engine_2_logs_replay_as_before() {
        let (s, text) = replay_fixture("engine2/drying_sheet");
        assert_eq!(s.canvas().unwrap().engine(), 2);
        assert_eq!(s.program("painting"), text);
        let expected = if cfg!(all(target_os = "linux", target_arch = "x86_64")) { "137bcdd02d9ccf64" } else { "2d2bc0a5b6922d4a" };
        assert_eq!(canvas_hash(&s), expected, "the engine-2 drying sheet no longer replays as it did");
    }

    /// Replay `tests/<name>.lua` at 320 px with the box and engine its log
    /// names.
    fn replay_fixture(name: &str) -> (Session, String) {
        let text = std::fs::read_to_string(format!("{}/tests/{name}.lua", env!("CARGO_MANIFEST_DIR"))).unwrap();
        let mut s = Session::replay_with(320, box_for(Some(&text)).expect("(EASEL_BOX set in the test's environment?)")).unwrap();
        for (i, c) in parse_program(&text).iter().enumerate() {
            s.run(c).unwrap_or_else(|e| panic!("{name} chunk {}: {e}", i + 1));
        }
        (s, text)
    }

    /// FNV-1a of the canvas as seen and its surface relief.
    fn canvas_hash(s: &Session) -> String {
        let c = s.canvas().unwrap();
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for b in c.seen().iter().flat_map(|p| p.map(f32::to_bits)).chain(c.kept_surface_um().2.iter().map(|v| v.to_bits())) {
            h ^= b as u64;
            h = h.wrapping_mul(0x100_0000_01b3);
        }
        format!("{h:016x}")
    }
}
