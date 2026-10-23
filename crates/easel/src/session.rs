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
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
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
/// A log's head line: its knife's paint laid on wet paint stays on top of it
/// (`paint::Canvas::with_knife_skim`). Every new painting has it.
pub const KNIFE_SKIM_MARK: &str = "--@ knife-skim";
/// Marks a sketch: a painting painted at `SKETCH_WIDTH`, not the live width.
/// A log replays at the width it was painted at, whatever its file is named.
pub const SKETCH_MARK: &str = "--@ sketch";
pub const TILES_MARK: &str = "--@ work-tiles";
/// Marks a painting whose `body_of` limbs join the nearest other limb, and
/// only from within their own width (`paint::Outline::body`): every new
/// painting. A log without it replays its limbs joined to the spine.
pub const LIMBS_MARK: &str = "--@ limbs-join-nearest";
/// Marks a painting whose `b:reload` refuses a nonfinite amount, as `b:load`
/// does: every new painting. A log without it replays its NaN reload.
pub const RELOAD_FINITE_MARK: &str = "--@ reload-finite";
pub const CARRIED_PAINT_MARK: &str = "--@ palette-carried-paint";
/// A log's head line: its `glaze` and `scumble` passes lay a thin film,
/// their coverage the share of the mask it reaches
/// (`paint::Canvas::with_glaze_film`). Every new painting has it; it
/// requires engine 7, 8 or 9.
pub const GLAZE_FILM_MARK: &str = "--@ glaze-film";
pub const LIQUID_FILM_LEVELS_MARK: &str = "--@ liquid-film-levels";
pub const SEALED_DRAWING_GUIDE_MARK: &str = "--@ sealed-drawing-guide";
pub const DRAWING_OPTICS_MARK: &str = "--@ drawing-optics";
/// Marks a painting whose full palette board refuses a new heap instead of scraping off the
/// oldest (board.rs): every new painting. A log without it replays with the oldest scraped off.
pub const FULL_PALETTE_MARK: &str = "--@ full-palette-refuses";
/// Marks a painting whose blender and glaze meet their masks as the mask says
/// (`paint::Palette::blend_at_masks`): every new painting. A log without it
/// replays `soften` as a fresh ramp on the mask's 0.5 line and its unclipped
/// blends and glazes at their strokes' full length.
pub const BLEND_MARK: &str = "--@ blend-at-masks";
/// A log's head line: its blender lays the wet paint it picks up only where
/// wet paint was, and a little way past it (`paint::Canvas::with_blend_feather`).
/// Every new painting has it.
pub const BLEND_FEATHER_MARK: &str = "--@ blend-feather";
/// A log's head line: a brush's paint laid on wet paint stays on top of it
/// (`paint::Canvas::with_brush_skim`). Every new painting has it.
pub const BRUSH_SKIM_MARK: &str = "--@ brush-skim";
/// A log's head line: its rag keeps what it lifts and a mask wipe stays
/// inside its mask (`paint::Canvas::with_rag_lift`). Every new painting has
/// it.
pub const RAG_LIFT_MARK: &str = "--@ rag-lift";
/// Marks a painting whose `pastel(p)` takes its color from the pile's heap as it is
/// now, added to and dirtied (draw_pencil.rs): every new painting. A log without it
/// replays its sticks in the colors of the piles as first knifed.
pub const PASTEL_HEAP_MARK: &str = "--@ pastel-current-heap";
/// Marks a painting whose calls agree with each other: a brush table refuses an option it
/// doesn't know, a brush's load refuses an amount outside 0..1 as a knife's does, a recipe or
/// mix whose parts overflow is refused, a pastel stick's side strokes keep its strokes' upper
/// limits and it refuses the pencil's sharpen and width, a view's `behind` and `front` pick as a
/// pass's `behind=` does, and a rag wipes a faint mask as it wipes a full one, lifting by
/// the mask's value: every new painting. A log without it replays as it was painted.
pub const CALLS_AGREE_MARK: &str = "--@ calls-agree";
/// The drain switch, in the head of a session file: a painting whose log
/// has it is painted with an absorbent ground that drains paint as its
/// pigment packs, and blotting that stops at the drained floor (paint's
/// `Canvas::with_drain`). A new painting has it; a log without it was
/// painted before the drain and replays as it was painted. It names no
/// engine of its own: it rides on engine 5 and later (`box_for`), and the
/// box carries it (`Palette::drain`).
pub const DRAIN_MARK: &str = "--@ drain";
/// The paper-and-pastel switch, in the head of a session file: a painting
/// whose log has it may be on a sheet of paper (`canvas{paper=}`, paint's
/// `paper`), and what comes with paper is its too. A new painting has it; a
/// log without it was painted before paper and replays as it was painted,
/// with none of it. It names no engine of its own: it rides on engine 5
/// and later (`box_for`).
pub const PAPER_MARK: &str = "--@ paper";
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

/// How far the chunk a live session is running has got, for `status` while it runs (main.rs
/// `serving_status`). A process runs one chunk at a time; replays leave it alone. Written on
/// the session's thread, read on the thread that answers `status`.
pub struct Progress {
    /// The chunk's lines, and the line of it running when last seen (0: not seen yet). The
    /// line is looked at with the clock (`HOOK_EVERY`), so not inside one long verb.
    lines: AtomicUsize,
    line: AtomicUsize,
    /// Strokes and touches (`paint::Tally`) and the painting's clock (minutes, f64 bits) when
    /// the chunk began and after its last verb.
    marks0: AtomicU64,
    marks: AtomicU64,
    clock0: AtomicU64,
    clock: AtomicU64,
    /// The chunk's number in the log once it is kept there (0: not yet), while the easel
    /// finishes its reply (a look, a frame).
    kept: AtomicUsize,
}

pub static PROGRESS: Progress = Progress { lines: AtomicUsize::new(0), line: AtomicUsize::new(0), marks0: AtomicU64::new(0), marks: AtomicU64::new(0), clock0: AtomicU64::new(0), clock: AtomicU64::new(0), kept: AtomicUsize::new(0) };

impl Progress {
    fn begin(&self, lines: usize, st: &Studio) {
        let marks = st.canvas.as_ref().map_or(0, |c| { let t = c.tally(); t.strokes + t.touches });
        self.lines.store(lines, Ordering::Relaxed);
        self.line.store(0, Ordering::Relaxed);
        self.marks0.store(marks, Ordering::Relaxed);
        self.marks.store(marks, Ordering::Relaxed);
        self.clock0.store(st.clock.to_bits(), Ordering::Relaxed);
        self.clock.store(st.clock.to_bits(), Ordering::Relaxed);
    }

    /// Before a chunk is asked for: nothing of the last one's is reported.
    pub fn clear(&self) {
        self.lines.store(0, Ordering::Relaxed);
        self.line.store(0, Ordering::Relaxed);
        self.marks0.store(0, Ordering::Relaxed);
        self.marks.store(0, Ordering::Relaxed);
        self.clock0.store(0, Ordering::Relaxed);
        self.clock.store(0, Ordering::Relaxed);
        self.kept.store(0, Ordering::Relaxed);
    }

    /// The chunk is in the log as chunk `n` (main.rs `serve_cmd`).
    pub fn keep(&self, n: usize) {
        self.kept.store(n, Ordering::Release);
    }

    /// The running chunk's number in the log, once it is kept there.
    pub fn kept(&self) -> usize {
        self.kept.load(Ordering::Acquire)
    }

    /// After a verb of a live chunk (time.rs `verb`).
    pub fn verb(&self, st: &Studio) {
        if let Some(c) = st.canvas.as_ref() {
            let t = c.tally();
            self.marks.store(t.strokes + t.touches, Ordering::Relaxed);
        }
        self.clock.store(st.clock.to_bits(), Ordering::Relaxed);
    }

    /// The running chunk's progress, `elapsed` after it began, as `status` answers it.
    pub fn report(&self, elapsed: Duration) -> String {
        let kept = self.kept();
        if kept > 0 {
            return format!("the chunk is in the log (chunk {kept}); the easel is finishing its reply, running for {}", minutes_seconds(elapsed.as_secs_f64()));
        }
        let (line, lines) = (self.line.load(Ordering::Relaxed), self.lines.load(Ordering::Relaxed));
        let marks = self.marks.load(Ordering::Relaxed).saturating_sub(self.marks0.load(Ordering::Relaxed));
        let clock = f64::from_bits(self.clock.load(Ordering::Relaxed));
        let spent = (clock - f64::from_bits(self.clock0.load(Ordering::Relaxed))).max(0.0) * 60.0;
        if lines == 0 {
            return format!("painting a chunk: starting, running for {}", minutes_seconds(elapsed.as_secs_f64()));
        }
        let at = if line > 0 { format!("at line {line} of {lines}") } else { format!("{lines} lines") };
        format!(
            "painting a chunk ({at}): {marks} strokes and touches so far, {} on the painting's clock (now {}), running for {}",
            minutes_seconds(spent), crate::time::time_of_day(clock), minutes_seconds(elapsed.as_secs_f64())
        )
    }
}

/// `14 min 20 s`, `40 s`.
fn minutes_seconds(secs: f64) -> String {
    let s = secs.round() as u64;
    if s < 60 { format!("{s} s") } else { format!("{} min {} s", s / 60, s % 60) }
}

pub struct Chunk {
    pub src: String,
    pub mark_effects: u64,
}

/// Everything a failed chunk could have changed, taken before it runs.
struct Snap {
    recording_ground: u64,
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
    #[cfg(test)]
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
        lua.set_hook(mlua::HookTriggers::new().every_nth_instruction(HOOK_EVERY), move |lua, _| match d.get() {
            Some(t) if Instant::now() > t => Err(mlua::Error::runtime(format!(
                "the chunk ran longer than {limit_min} minutes and was stopped; nothing it did was kept"
            ))),
            // a live chunk: the line of it running (its outermost frame; a function an
            // earlier chunk made is that chunk's)
            Some(_) => {
                let mut line = None;
                for level in 0..200 {
                    let Some(found) = lua.inspect_stack(level, |f| {
                        let chunk = f.source().source.is_some_and(|s| s.trim_start_matches(['=', '@']).starts_with("chunk "));
                        chunk.then(|| f.current_line()).flatten()
                    }) else { break };
                    line = found.or(line);
                }
                if let Some(l) = line {
                    PROGRESS.line.store(l, Ordering::Relaxed);
                }
                Ok(mlua::VmState::Continue)
            }
            None => Ok(mlua::VmState::Continue),
        })?;
        let own = global_values(&lua)?;
        let globals = own.iter().map(|(k, v)| (k.clone(), (v.clone(), 0))).collect();
        Ok(Session { lua: ManuallyDrop::new(lua), state, st, log: Vec::new(), replay: false, deadline, chunk_limit: chunk_limit_for(width), heap: Some((snap_f, restore_f)), prelude: Some(prelude), stale: false, unrestored: false, globals, own, chunks_before: 0, _serials: serials })
    }

    /// A session that replays a program from the default box (tests;
    /// `easel run` and `check` use `replay_with`).
    #[cfg(test)]
    pub fn replay(width: usize) -> mlua::Result<Self> {
        Self::replay_with(width, Palette::tube_box())
    }

    /// A session that replays a program painted from `tubes`: a failed chunk
    /// ends it, so it keeps no snapshot. (`easel run` and `check`: the replay
    /// build.)
    pub fn replay_with(width: usize, tubes: Palette) -> mlua::Result<Self> {
        let mut s = Self::with_box(width, tubes)?;
        s.replay = true;
        Ok(s)
    }

    /// Set the drain switch (`DRAIN_MARK`) on the box this session paints
    /// from (tests: a new painting's is on and a replay's is its log's,
    /// both set by `box_for`). Set before the first chunk runs.
    #[cfg(test)]
    pub fn with_drain(self, on: bool) -> Self {
        debug_assert!(self.log.is_empty(), "the drain switch is set before the first chunk");
        Rc::make_mut(&mut self.st.borrow_mut().tubes).drain = on;
        self
    }

    /// The box this session paints from.
    pub fn tube_box(&self) -> Palette {
        (*self.st.borrow().tubes).clone()
    }

    /// Set the paper-and-pastel switch (`PAPER_MARK`): a new painting's is
    /// on, a replay's is its log's (`logged_paper`). Set before the first
    /// chunk runs.
    pub fn with_paper(mut self, on: bool) -> mlua::Result<Self> {
        self.set_paper(on)?;
        Ok(self)
    }

    /// `with_paper` in place. Switched on, the session gets the switch's
    /// verbs (`api::install_paper`: `pastel`, `smudge`, `feel` and the
    /// stick's methods), which are then the easel's own globals as much as
    /// those `api::install` set; switched off it has none of them, so a log
    /// painted before the switch sees exactly the globals it saw.
    pub fn set_paper(&mut self, on: bool) -> mlua::Result<()> {
        debug_assert!(self.log.is_empty(), "the paper-and-pastel switch is set before the first chunk");
        let was = self.st.borrow().paper;
        match (was, on) {
            (false, true) => {
                self.st.borrow_mut().paper = true;
                api::install_paper(&self.lua, self.st.clone())?;
                // (no chunk has run: every global is the easel's own)
                self.own = global_values(&self.lua)?;
                self.globals = self.own.iter().map(|(k, v)| (k.clone(), (v.clone(), 0))).collect();
                Ok(())
            }
            (true, false) => Err(mlua::Error::runtime("the paper-and-pastel switch, once on, stays on")),
            _ => Ok(()),
        }
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
        let sr = &self._serials;
        sr.uncapped.set(true);
        sr.marking.set(true);
        let heap = snap_f.call::<Table>((skip, self.lua.globals(), strings));
        sr.marking.set(false);
        sr.uncapped.set(false);
        let heap = heap?;
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
            let v = r.borrow().clone();
            (r, v)
        }).collect();
        Ok(Snap { recording_ground: s.recording_ground, canvas: s.canvas.clone(), style: s.style.clone(), setup: s.setup.clone(), canvas_src: s.canvas_src.clone(), seed: s.seed, clock: s.clock, clock0: s.clock0, hand: s.hand.clone(), board: s.board.clone(), view: s.view.clone(), heap, brushes, knives, rags })
    }

    /// Put everything back as it was at `snap`. Returns how many Lua tables
    /// still differ in contents, traversal order or raw length after restoration (see `stale`).
    fn restore(&mut self, snap: &Snap) -> mlua::Result<usize> {
        let (_, restore_f) = self.heap.as_ref().unwrap();
        // not marking: what restore makes is the painting's again
        self._serials.uncapped.set(true);
        let mismatches = restore_f.call::<usize>(snap.heap.clone());
        self._serials.uncapped.set(false);
        let mismatches = mismatches?;
        for (b, h) in &snap.brushes {
            *b.borrow_mut() = h.clone();
        }
        for (k, h) in &snap.knives {
            *k.borrow_mut() = h.clone();
        }
        for (r, v) in &snap.rags {
            *r.borrow_mut() = v.clone();
        }
        let mut s = self.st.borrow_mut();
        s.canvas = snap.canvas.clone();
        s.style = snap.style.clone();
        s.setup = snap.setup.clone();
        s.canvas_src = snap.canvas_src.clone();
        s.recording_ground = snap.recording_ground;
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
        let mut fresh = Session::with_box(width, tubes).and_then(|s| s.with_paper(self.st.borrow().paper)).map_err(|e| e.to_string())?;
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
        let trace = (!self.replay).then(crate::replay_checkpoint::mark);
        crate::replay_checkpoint::hold_tools(&self.st);
        let mut snap = if !self.replay { Some(self.snap().map_err(|e| e.to_string())?) } else { None };
        let n = (self.chunks_before + self.log.len()) as u64 + 1;
        self.prelude.as_ref().unwrap().0.call::<()>(()).map_err(|e| e.to_string())?;
        self.st.borrow_mut().begin(n);
        let had_canvas = self.st.borrow().canvas.is_some();
        self.st.borrow_mut().live = !self.replay;
        if !self.replay {
            PROGRESS.begin(src.lines().count(), &self.st.borrow());
        }
        let t0 = Instant::now();
        // text only: mlua would take a chunk starting with Lua's binary signature as bytecode
        let chunk = self.lua.load(src.as_str()).set_name(format!("chunk {n}")).set_mode(mlua::chunk::ChunkMode::Text);
        let (check, guard) = { let p = self.prelude.as_ref().unwrap(); (p.2.clone(), p.3.clone()) };
        let deadline = (!self.replay).then(|| t0 + self.chunk_limit);
        self.deadline.set(deadline);
        self._serials.capped.set(false);
        // Everything that can fail before the chunk is kept is inside this guard: the chunk,
        // prelude.lua's check, putting its hand time on the clock and reading the globals it
        // left. A failure anywhere in it is a failed chunk, rolled back below.
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // through prelude.lua's guard: an error value is shown without its address
            guard.call::<()>(chunk.into_function()?)?;
            check.call::<()>(())?;
            // a paper mask is laid and lifted within one sitting's chunk (paint's sheet.rs)
            if self.st.borrow().canvas.as_ref().is_some_and(|c| c.has_sheet()) {
                return Err(mlua::Error::runtime("the chunk ended with a sheet laid on the picture: lift it (lift_sheet()) in the chunk that laid it"));
            }
            // the hand time the chunk spent goes on the clock before it ends
            crate::time::flush(&self.st, true);
            crate::replay_checkpoint::check()?;
            self.next_globals(n as usize)
        }));
        self.deadline.set(None);
        let secs = t0.elapsed().as_secs_f64();
        let feedback = crate::replay_checkpoint::effect(&self.st, "drying feedback", || {
            crate::pass_feedback::finish(&mut self.st.borrow_mut());
            Ok(())
        });
        let out = self.st.borrow().out.clone();
        let (mut fail, globals) = match r {
            Ok(Ok(g)) => (None, Some(g)),
            Ok(Err(e)) => (Some(clean_error(&e.to_string())), None),
            Err(p) => (Some(format!("engine panic: {}", panic_text(&p))), None),
        };
        if let Err(e) = feedback {
            fail.get_or_insert_with(|| clean_error(&e.to_string()));
        }
        let globals = if self._serials.capped.get() { None } else { globals };
        if fail.is_none() && self._serials.capped.get() {
            fail = Some(format!("the chunk asked for more memory than a painting may hold ({} GiB), so nothing it did was kept", LUA_MAX_BYTES >> 30));
        }
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
        crate::replay_checkpoint::chunk_end();
        self.deadline.set(deadline);
        let _ = self.lua.gc_collect();
        self.deadline.set(None);
        if let Some(e) = fail {
            if let Some(mark) = trace { crate::replay_checkpoint::rollback(mark); }
            let mut msg = out;
            msg.push_str(&e);
            return Err(msg);
        }
        let st = self.st.borrow();
        let mark_effects = st.mark_effects + u64::from(!had_canvas && st.canvas.is_some());
        self.log.push(Chunk { src, mark_effects });
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
        if self.st.borrow().tubes.knife_skim {
            let _ = writeln!(s, "{KNIFE_SKIM_MARK}");
        }
        if self.st.borrow().tubes.carried_paint {
            let _ = writeln!(s, "{CARRIED_PAINT_MARK}");
        }
        if self.st.borrow().tubes.limbs_join_nearest {
            let _ = writeln!(s, "{LIMBS_MARK}");
        }
        if self.st.borrow().tubes.reload_finite {
            let _ = writeln!(s, "{RELOAD_FINITE_MARK}");
        }
        if self.st.borrow().tubes.blend_feather {
            let _ = writeln!(s, "{BLEND_FEATHER_MARK}");
        }
        if self.st.borrow().tubes.brush_skim {
            let _ = writeln!(s, "{BRUSH_SKIM_MARK}");
        }
        if self.st.borrow().tubes.blend_at_masks {
            let _ = writeln!(s, "{BLEND_MARK}");
        }
        if self.st.borrow().tubes.glaze_film {
            let _ = writeln!(s, "{GLAZE_FILM_MARK}");
        }
        if self.st.borrow().tubes.liquid_film_levels {
            let _ = writeln!(s, "{LIQUID_FILM_LEVELS_MARK}");
        }
        if self.st.borrow().tubes.sealed_drawing_guide {
            let _ = writeln!(s, "{SEALED_DRAWING_GUIDE_MARK}");
        }
        if self.st.borrow().tubes.drawing_optics {
            let _ = writeln!(s, "{DRAWING_OPTICS_MARK}");
        }
        if self.st.borrow().tubes.rag_lift {
            let _ = writeln!(s, "{RAG_LIFT_MARK}");
        }
        if self.st.borrow().tubes.full_palette_refuses {
            let _ = writeln!(s, "{FULL_PALETTE_MARK}");
        }
        if self.st.borrow().tubes.pastel_current_heap {
            let _ = writeln!(s, "{PASTEL_HEAP_MARK}");
        }
        if self.st.borrow().tubes.calls_agree {
            let _ = writeln!(s, "{CALLS_AGREE_MARK}");
        }
        if self.st.borrow().width == SKETCH_WIDTH {
            let _ = writeln!(s, "{SKETCH_MARK}");
        }
        // only a painting width other than the legacy one is named (test sessions use small widths)
        let width = self.st.borrow().width;
        if width != LEGACY_WIDTH && WIDTHS.contains(&width) {
            let _ = writeln!(s, "{WIDTH_MARK} {width}");
        }
        // (our switches after main's head lines)
        if self.st.borrow().tubes.drain {
            let _ = writeln!(s, "{DRAIN_MARK}");
        }
        if self.st.borrow().paper {
            let _ = writeln!(s, "{PAPER_MARK}");
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

    pub fn mask(&self, name: &str) -> Result<std::sync::Arc<paint::Mask>, String> {
        let found = self.globals.get(name).filter(|(_, c)| *c > 0).map(|(v, _)| v);
        if let Some(Value::UserData(u)) = found && let Ok(m) = u.borrow::<api::M>() {
            return Ok(std::sync::Arc::new((*m.0).clone()));
        }
        Err(format!("look --mask {name}: no global mask with that name"))
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
                let _ = board.visit(p.heap, 0.0);
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

    /// The pastel stick a global (or a field of one: `P.glow`) holds, and how
    /// far it has been turned in the fingers (degrees): `look --hold`'s.
    pub fn held_stick(&self, name: &str) -> Result<(paint::pastel::Stick, f32), String> {
        let mut parts = name.split('.');
        let first = parts.next().unwrap_or("");
        let mut v: Value = self.lua.globals().raw_get(first).map_err(|e| e.to_string())?;
        for p in parts {
            v = match v {
                Value::Table(t) => t.raw_get(p).map_err(|e| e.to_string())?,
                _ => Value::Nil,
            };
        }
        match v {
            Value::Table(t) if t.raw_get::<Option<String>>("kind").ok().flatten().as_deref() == Some("stick") => {
                let s = crate::api::draw_pencil::stick_of(&t).map_err(|e| e.to_string())?;
                let turned = t.raw_get::<Option<f32>>("turned").ok().flatten().unwrap_or(0.0);
                Ok((s, turned))
            }
            _ => Err(format!("look --hold {name}: that holds no pastel stick")),
        }
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
    /// bytes the state holds now, headers included; past LUA_MAX_BYTES an
    /// allocation fails and Lua raises "not enough memory"
    used: Cell<usize>,
    cap: Cell<usize>,
    /// bytes of `used` in blocks the pre-chunk snapshot made (marked `SNAP`),
    /// counted as they are freed or resized: the cap leaves them out, so a
    /// chunk has the room a replay (which takes no snapshot) has
    held: Cell<usize>,
    /// while the easel snapshots or restores: no cap, so a painting the cap
    /// allowed can always be snapshotted and a failed chunk always put back
    uncapped: Cell<bool>,
    /// while the easel snapshots: new blocks are the snapshot's (`SNAP`)
    marking: Cell<bool>,
    /// set when the cap refused an allocation; `run` fails a chunk that hit
    /// it, even one that caught the error or that Lua's emergency collection
    /// recovered from, since only a live session's heap holds its snapshot
    capped: Cell<bool>,
    /// userdata and thread blocks: user address -> (serial, size)
    inner: RefCell<BTreeMap<usize, (i64, usize)>>,
}

const HDR: usize = 16;
/// The most a session's Lua state may hold. A chunk that builds something huge
/// (a giant string.rep, a loop filling a table) then errors and is rolled back
/// instead of swapping the machine. The snapshot a live session takes before
/// each chunk is outside it (`Serials::held`), so a live painting can hold
/// up to about twice this. Even so live and replayed heaps differ a little,
/// so a chunk that meets the cap fails however it handles that
/// (`Serials::capped`): no kept chunk ever met it.
const LUA_MAX_BYTES: usize = 2 << 30;
const PLAIN: i64 = 0;
const AT_START: i64 = 1;
const LISTED: i64 = 2;
/// With the kind: a block made while the easel snapshotted (`Serials::held`).
const SNAP: i64 = 4;

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
            if nsize.saturating_add(HDR) > h.room() {
                h.capped.set(true);
                return std::ptr::null_mut();
            }
            let b = std::alloc::alloc(layout(nsize));
            if b.is_null() {
                return b as *mut c_void;
            }
            h.used.set(h.used.get() + nsize + HDR);
            let snap = if h.marking.get() {
                h.held.set(h.held.get() + nsize + HDR);
                SNAP
            } else {
                0
            };
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
            *hd.add(1) = kind | snap;
            let p = b.add(HDR);
            if kind == LISTED {
                h.inner.borrow_mut().insert(p as usize, (n, nsize));
            }
            return p as *mut c_void;
        }
        let b = (ptr as *mut u8).sub(HDR);
        let kind = *(b as *const i64).add(1);
        if nsize == 0 {
            if kind & !SNAP == LISTED {
                h.inner.borrow_mut().remove(&(ptr as usize));
            }
            std::alloc::dealloc(b, layout(osize));
            h.used.set(h.used.get().saturating_sub(osize + HDR));
            if kind & SNAP != 0 {
                h.held.set(h.held.get().saturating_sub(osize + HDR));
            }
            return std::ptr::null_mut();
        }
        if nsize > osize && nsize - osize > h.room() {
            h.capped.set(true);
            return std::ptr::null_mut();
        }
        // only arrays and buffers are reallocated, never objects
        let nb = std::alloc::realloc(b, layout(osize), nsize + HDR);
        if nb.is_null() {
            return nb as *mut c_void;
        }
        h.used.set(h.used.get().saturating_sub(osize) + nsize);
        if kind & SNAP != 0 {
            h.held.set(h.held.get().saturating_sub(osize) + nsize);
        }
        nb.add(HDR) as *mut c_void
    }
}

impl Serials {
    /// What the cap still allows a chunk to allocate.
    fn room(&self) -> usize {
        if self.uncapped.get() {
            return usize::MAX;
        }
        self.cap.get().saturating_sub(self.used.get().saturating_sub(self.held.get()))
    }

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
    let serials = Box::new(Serials { next: Cell::new(1), inner: RefCell::new(BTreeMap::new()), used: Cell::new(0), cap: Cell::new(LUA_MAX_BYTES), held: Cell::new(0), uncapped: Cell::new(false), marking: Cell::new(false), capped: Cell::new(false) });
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

/// Lua errors carry a traceback; the painter needs the first lines. A shape or number error
/// from one of the easel's functions says neither which function nor where the chunk called
/// it (`located`): both go in front of it.
fn clean_error(e: &str) -> String {
    let mut lines = e.lines();
    let out: Vec<&str> = lines.by_ref().take_while(|l| !l.trim_start().starts_with("stack traceback")).collect();
    let msg = out.join("\n");
    unaddressed(&located(&msg, lines.map(str::trim)).unwrap_or(msg))
}

/// `msg` with the easel function that raised it and the chunk line that called it in front
/// (`below at line 3: the curve: ...`), when it is a shape or number error that names
/// neither: a Lua value converted to the wrong type, or a point list, curve or number the
/// easel refused. `frames` are the traceback's, innermost first. A method's arguments are
/// counted as the painter wrote them, without the object before the colon. None for any
/// other error.
fn located<'a>(msg: &str, mut frames: impl Iterator<Item = &'a str>) -> Option<String> {
    let (kind, name) = frames.next()?.strip_prefix("[C]: in ")?.split_once(' ')?;
    let name = name.strip_prefix('\'')?.strip_suffix('\'')?;
    let line = frames.find_map(|f| f.strip_prefix("[string \"chunk ")?.split_once("\"]:")?.1.split_once(':').map(|l| l.0.to_string()));
    let at = match line {
        Some(l) => format!("{name} at line {l}"),
        None => name.to_string(),
    };
    let msg = msg.strip_prefix("runtime error: ").unwrap_or(msg);
    let converted = |m: &str| -> Option<String> {
        let (from, to) = m.strip_prefix("error converting Lua ")?.split_once(" to ")?;
        let to = to.split(' ').next()?;
        let want = match to {
            "f32" | "f64" => "a number",
            "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "usize" | "isize" => "a whole number",
            "table" | "Table" => "a table",
            "String" | "string" | "BorrowedStr" => "text",
            "bool" | "boolean" => "true or false",
            "function" | "Function" => "a function",
            o => o,
        };
        Some(format!("want {want}, got {from}"))
    };
    if let Some(rest) = msg.strip_prefix("bad argument #") {
        let n: usize = rest[..rest.find(|c: char| !c.is_ascii_digit())?].parse().ok()?;
        let n = if kind == "method" && n > 1 { n - 1 } else { n };
        let cause = rest.split_once(": ")?.1;
        return Some(format!("{at}: argument {n}: {}", converted(cause).unwrap_or_else(|| cause.to_string())));
    }
    if let Some(c) = converted(msg) {
        return Some(format!("{at}: {c}"));
    }
    let number = msg.split_once(": ").map_or(msg, |m| if m.0.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') { m.1 } else { msg });
    (msg.starts_with("points: ") || msg.starts_with("the curve: ") || number.starts_with("want a number, got ")).then(|| format!("{at}: {msg}"))
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
                return Err(format!("the log's line {l:?} names no engine this easel has ({ENGINE_MARK} <1 to {}, {}, {} or {}>)", paint::ENGINE, paint::engine7::ENGINE, paint::REACH_END, paint::PIGMENT_MIXER));
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

/// Whether a session file's head has the paper-and-pastel switch (a
/// `PAPER_MARK` line before the first chunk).
pub fn logged_paper(text: &str) -> bool {
    text.lines().map(str::trim).take_while(|l| !l.starts_with(MARK)).any(|l| l == PAPER_MARK)
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

/// Whether a session file's head has the drain switch (a `DRAIN_MARK` line
/// before the first chunk).
pub fn logged_drain(text: &str) -> bool {
    text.lines().map(str::trim).take_while(|l| !l.starts_with(MARK)).any(|l| l == DRAIN_MARK)
}

/// Where the box of a new painting is set: a file `box` next to the easel's
/// executable (one line, the box's name; a studio ships it as
/// `<studio>/bin/box`), else `EASEL_BOX`, in developer mode. Painter mode
/// takes only its studio setting. None: the default box. Conflicting
/// developer settings are an error.
pub fn configured_box() -> Result<Option<(String, String)>, String> {
    if let Some(name) = crate::studio::box_name() {
        return Ok(Some((name.to_string(), "the studio setting".to_string())));
    }
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
    if let Some(own) = crate::studio::box_name()
        && name != own
    {
        return Err("this studio paints from its own box only; nothing ran".into());
    }
    Palette::named_box(name).ok_or_else(|| format!("this easel has no box {name:?} (its boxes: {})", box_list()))
}

/// The boxes this easel has, quoted: `"tube box", "sargent"`.
fn box_list() -> String {
    Palette::box_names().iter().map(|b| format!("{b:?}")).collect::<Vec<_>>().join(", ")
}

/// The box a painting is painted from. A new painting (`log` None) takes the
/// configured box (`configured_box`), else the default. An existing one takes
/// the box its log names (none: the default), whatever easel replays it; if a
/// box is configured too, it must be the same one. The box carries the
/// engine version the painting is painted with: a new one engine 9,
/// an existing one its log's (`logged_engine`). New paintings' `body_of`
/// joins limbs to the nearest limb; an existing one's as its log says
/// (`LIMBS_MARK`), and likewise for the blender's feather
/// (`BLEND_FEATHER_MARK`), the brush's skim (`BRUSH_SKIM_MARK`) and
/// blends and glazes at masks (`BLEND_MARK`).
/// New paintings' full palette refuses a new heap, and their pastels take the color of
/// their pile's heap as it is now; an existing one's as its log says
/// (`FULL_PALETTE_MARK`, `PASTEL_HEAP_MARK`).
pub fn box_for(log: Option<&str>) -> Result<Palette, String> {
    let configured = configured_box()?;
    let Some(text) = log else {
        let mut b = find_box(configured.as_ref().map_or(Palette::fallback_box(), |c| c.0.as_str()))?;
        b.engine = paint::PIGMENT_MIXER;
        b.knife_skim = true;
        b.limbs_join_nearest = true;
        b.reload_finite = true;
        b.carried_paint = true;
        b.glaze_film = true;
        b.liquid_film_levels = true;
        b.sealed_drawing_guide = true;
        b.drawing_optics = true;
        b.full_palette_refuses = true;
        b.pastel_current_heap = true;
        b.calls_agree = true;
        b.blend_at_masks = true;
        b.blend_feather = true;
        b.brush_skim = true;
        b.rag_lift = true;
        b.drain = true;
        return Ok(b);
    };
    let engine = logged_engine(text)?;
    if logged_paper(text) && engine < 5 {
        return Err(format!("the log's {PAPER_MARK} line (the paper-and-pastel switch) is engine 5's and later's; this log names engine {engine}; nothing ran"));
    }
    let adaptive_sweep = text.lines().map(str::trim).take_while(|l| !l.starts_with(MARK)).any(|l| l == "--@ adaptive-sweep");
    if adaptive_sweep && engine != 5 {
        return Err("the log's adaptive sweep switch requires engine 5".into());
    }
    let head = |mark: &str| text.lines().map(str::trim).take_while(|l| !l.starts_with(MARK)).any(|l| l == mark);
    let (knife_skim, blend_feather, brush_skim) = (head(KNIFE_SKIM_MARK), head(BLEND_FEATHER_MARK), head(BRUSH_SKIM_MARK));
    if knife_skim && !matches!(engine, paint::engine7::ENGINE | paint::REACH_END | paint::PIGMENT_MIXER) {
        return Err(format!("the log's knife skim switch ({KNIFE_SKIM_MARK}) requires engine 7, 8 or 9"));
    }
    for (on, mark) in [(blend_feather, BLEND_FEATHER_MARK), (brush_skim, BRUSH_SKIM_MARK)] {
        if on && !matches!(engine, paint::engine7::ENGINE | paint::REACH_END | paint::PIGMENT_MIXER) {
            return Err(format!("the log's switch {mark:?} requires engine 7, 8 or 9"));
        }
    }
    let drain = logged_drain(text);
    if drain && engine < 5 {
        return Err(format!("the log's {DRAIN_MARK} line (the drain switch) is engine 5's and later's; this log names engine {engine}; nothing ran"));
    }
    let logged = logged_box(text)?;
    let name = logged.as_deref().unwrap_or(paint::palette::DEFAULT_BOX);
    if let Some((c, from)) = &configured
        && c != name
    {
        if crate::studio::painter() {
            return Err("the log wasn't painted from this studio's box; nothing ran".into());
        }
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
    b.knife_skim = knife_skim;
    b.carried_paint = head(CARRIED_PAINT_MARK);
    b.limbs_join_nearest = text.lines().map(str::trim).take_while(|l| !l.starts_with(MARK)).any(|l| l == LIMBS_MARK);
    b.reload_finite = head(RELOAD_FINITE_MARK);
    b.glaze_film = text.lines().map(str::trim).take_while(|l| !l.starts_with(MARK)).any(|l| l == GLAZE_FILM_MARK);
    b.liquid_film_levels = head(LIQUID_FILM_LEVELS_MARK);
    b.sealed_drawing_guide = head(SEALED_DRAWING_GUIDE_MARK);
    b.drawing_optics = head(DRAWING_OPTICS_MARK);
    if b.liquid_film_levels && engine < 4 {
        return Err(format!("the log's liquid film leveling switch ({LIQUID_FILM_LEVELS_MARK}) requires engine 4 or later"));
    }
    if b.glaze_film && !paint::engine7::active(engine) {
        return Err(format!("the log's glaze film switch ({GLAZE_FILM_MARK}) requires engine 7, 8 or 9"));
    }
    b.full_palette_refuses = text.lines().map(str::trim).take_while(|l| !l.starts_with(MARK)).any(|l| l == FULL_PALETTE_MARK);
    b.pastel_current_heap = head(PASTEL_HEAP_MARK);
    b.calls_agree = head(CALLS_AGREE_MARK);
    b.blend_at_masks = text.lines().map(str::trim).take_while(|l| !l.starts_with(MARK)).any(|l| l == BLEND_MARK);
    b.blend_feather = blend_feather;
    b.brush_skim = brush_skim;
    b.rag_lift = text.lines().map(str::trim).take_while(|l| !l.starts_with(MARK)).any(|l| l == RAG_LIFT_MARK);
    if b.rag_lift && !paint::engine7::active(engine) {
        return Err(format!("the log's rag lift switch ({RAG_LIFT_MARK}) requires engine 7, 8 or 9"));
    }
    b.drain = drain;
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
/// in `out/`) in developer mode: `EASEL_ROOT` if set, else the nearest directory at or above
/// the working directory that holds `crates/easel/Cargo.toml`, else the
/// checkout this binary was built from. Looking from the working directory
/// keeps git worktrees apart: a binary built in (or copied from) another
/// checkout still writes into the worktree it's run in. Painter mode always
/// uses the studio above the executable's directory.
pub fn root() -> PathBuf {
    if let Some(root) = crate::studio::root() {
        return root.to_path_buf();
    }
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

#[cfg(test)]
#[path = "../../paint/tests/support/image_tolerance.rs"]
mod image_tolerance;

#[cfg(test)]
mod tests {
    use super::*;

    fn checkpoint_base_format(bytes: &[u8]) -> u64 {
        assert_eq!(&bytes[..8], b"PAINTC19");
        let header = u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
        assert_eq!(u64::from_le_bytes(bytes[16 + header..24 + header].try_into().unwrap()), paint::mixer::identity());
        u64::from_le_bytes(bytes[24 + header..32 + header].try_into().unwrap())
    }

    #[test]
    fn rag_preparation_keeps_hand_time_without_mark_effects() {
        let mut s = Session::new(32).unwrap();
        s.run(r#"canvas{size=200,aspect=1.25,linen=15,raw="linen"}"#).unwrap();
        for (src, secs) in [
            ("r=rag()", paint::rag::pace::FRESH),
            ("r:dip()", paint::rag::pace::DIP),
            ("r:refold()", paint::rag::pace::REFOLD),
        ] {
            let before = s.canvas().unwrap().clock();
            let tally = s.canvas().unwrap().tally().secs;
            s.run(src).unwrap();
            assert_eq!(s.log.last().unwrap().mark_effects, 0, "{src}");
            assert_eq!(s.canvas().unwrap().tally().secs - tally, secs, "{src}");
            assert!((s.canvas().unwrap().clock() - before - secs / 60.0).abs() < 1e-7, "{src}");
        }
        s.run("r:blot(400,300); r:wipe({{100,300},{900,300}})").unwrap();
        assert_eq!(s.log.last().unwrap().mark_effects, 2);
    }

    /// A shape or number error says which of the easel's functions refused what and the
    /// chunk line that called it, a method's arguments counted after its colon; any other
    /// error is as Lua and the easel wrote it.
    #[test]
    fn shape_and_number_errors_name_the_function_argument_and_line() {
        let mut s = Session::new(64).unwrap();
        s.run(r#"canvas{size=200, aspect=1.25, linen=15, ground={{pile={{"lead white", 1}}, um=40, apply="knife"}}}"#).unwrap();
        for (src, said) in [
            ("m = below(1405)", "below at line 1: the curve: want points {{x, y}, ...} or {x1, y1, x2, y2, ...}, or a function(x) giving y; got integer"),
            ("local x = 1\nm = poly({{1, 2}, {3, 'x'}, {5, 6}})", "poly at line 2: points: point 2: want {x, y}, two numbers"),
            ("m = poly({1, 2, 3, {}, 5, 6})", "poly at line 1: points: entry 4: want a number"),
            ("m = rect(1, 2, 3, 4):grow(true)", "grow at line 1: argument 1: want a number, got boolean"),
            ("local function f() return rect(1, nil, 2, 2) end\nf()", "rect at line 1: argument 2: want a number, got nil"),
            ("p = pile{{'lead white', 1}} work(rect(1, 2, 30, 40), {pile=p, coverage=function() end})", "work at line 1: coverage: want a number, got function"),
            ("m = ribbon({{1, 2}, {3, 4}}, true)", "runtime error: ribbon: widths: want one width or a table of one per point, got boolean"),
            ("work(rect(1, 2, 3, 4), {pile=1})", "runtime error: work: want a pile (made with pile{...}), got integer"),
            ("x = nil + 1", r#"runtime error: [string "chunk 2"]:1: attempt to perform arithmetic on a nil value"#),
        ] {
            assert_eq!(s.run(src).unwrap_err(), said, "{src}");
        }
    }

    #[test]
    fn numeric_option_errors_read_the_option_once() {
        let mut s = Session::new(32).unwrap();
        let out = s.run(r#"
            n = 0
            local ok, e = pcall(palette, setmetatable({}, {
                __index = function()
                    n = n + 1
                    error("option unavailable")
                end
            }))
            assert(not ok and tostring(e):find("option unavailable", 1, true))
            print(n)
        "#).unwrap().out;
        assert_eq!(out, "1\n");
        let out = s.run(r#"
            n = 0
            local ok, e = pcall(palette, setmetatable({}, {
                __index = function()
                    n = n + 1
                    return true
                end
            }))
            assert(not ok and tostring(e):find("dirty: want a number, got boolean", 1, true))
            print(n)
            palette{dirty="0.5"}
        "#).unwrap().out;
        assert_eq!(out, "1\n");
    }

    #[test]
    fn accepted_chunks_count_executed_mark_effects_not_lua_names() {
        let mut s = Session::new(64).unwrap().with_paper(true).unwrap();
        for (src, effects) in [
            ("canvas{size=200,aspect=1.25,paper={grammage=200}}", 1),
            (r#"q=pile{{"vermilion",1}}; b=brush("round",4); b:load(q,0.9); p=pastel(q)"#, 0),
            ("jobs={function() b:touch(400,300) end}; alias=b.touch", 0),
            (r#"b["touch"](b,400,300,{pressure=0.6})"#, 1),
            ("jobs[1]()", 1),
            ("alias(b,400,300); jobs[1]()", 2),
            ("p:side({{100,400},{900,400}})", 1),
            ("smudge({{100,400},{900,400}})", 1),
            ("erase(rect(300,300,100,100))", 1),
            ("fix()", 1),
            ("print(wait(60)); print(drying(400,300)); print(tubes()); b:reload(q,0.3)", 0),
            ("pcall(function() b:touch() end)", 0),
        ] {
            s.run(src).unwrap();
            assert_eq!(s.log.last().unwrap().mark_effects, effects, "{src}");
        }
        let before: Vec<_> = s.log.iter().map(|c| c.mark_effects).collect();
        assert!(s.run("jobs[1](); error('rollback')").is_err());
        assert_eq!(s.log.iter().map(|c| c.mark_effects).collect::<Vec<_>>(), before);
        s.run("print('jobs[1]()')").unwrap();
        assert_eq!(s.log.last().unwrap().mark_effects, 0);
    }

    fn assert_canvas_close(actual: &Canvas, expected: &Canvas) {
        assert_eq!(actual.engine(), expected.engine());
        assert_eq!(actual.drain(), expected.drain());
        let (aw, ah, actual_surface) = actual.kept_surface_um();
        let (ew, eh, expected_surface) = expected.kept_surface_um();
        assert_eq!((aw, ah), (ew, eh));
        image_tolerance::assert_fields_close(actual_surface, expected_surface);
        for mode in ["normal", "gallery", "relief"] {
            let size = expected.window().w.max(expected.window().h).to_string();
            let view = crate::look::View::parse(&["--mode".into(), mode.into(), "--size".into(), size]).unwrap();
            let (_, _, actual) = crate::look::render(actual, &view).unwrap();
            let (_, _, expected) = crate::look::render(expected, &view).unwrap();
            image_tolerance::assert_close(&image::load_from_memory(&actual).unwrap().to_rgba8(), &image::load_from_memory(&expected).unwrap().to_rgba8());
        }
    }

    #[test]
    fn a_subnormal_blend_stays_finite_through_a_scrape_and_wait() {
        let log = "--@ box every\n--@ engine 8\n--@ knife-skim\n--@ limbs-join-nearest\n--@ blend-feather\n--@ brush-skim\n--@ blend-at-masks\n--@ glaze-film\n--@ liquid-film-levels\n--@ rag-lift\n--@ full-palette-refuses\n--@ drain\n";
        let mut s = Session::replay_with(320, box_for(Some(log)).unwrap()).unwrap();
        s.run(r#"canvas{size=700,aspect=4/3,linen={16,14},ground={{pile={{"lead white",1}},um=70,apply="knife"}}}
            p=pile{{"lead white",1}}
            m=rect(580,270,130,200)
            work(m,{hand="body",pile=p,tool={kind="filbert",width=8,lay=2.5},coverage=2.5,fill=true,clip=m})"#).unwrap();
        for chunk in [
            "soft=m*mask(function(x,y) return x<640 and 1 or 1e-40 end)\nblend(soft,{angle=0,seed=1})",
            "k=knife{width=8}\nk:scrape({{644,350},{644,390}},{pressure=1,angle=0})",
            "wait(1)",
        ] {
            s.run(chunk).unwrap();
            for field in s.canvas().unwrap().state_fields() {
                if let paint::state_dump::Values::F32(values) = field.values {
                    assert!(values.iter().all(|v| v.is_finite()), "{} after {chunk}", field.name);
                }
            }
        }
    }


    const CANVAS: &str = r#"canvas{size=440, aspect=1.5, linen=15, seed=2, ground={{pile={{"lead white", 3}, {"red earth", 1}}, um=110, apply="knife"}, {pile={{"lead white", 5}, {"yellow ochre", 1}}, um=60, apply="brush"}}}"#;

    /// The drain switch is read from a log's head only, needs engine 5, and
    /// rides on the box: a log with it replays, rebuilds and checkpoints the
    /// same draining canvas (version 14), its brushes printing the paint's
    /// packing; a log without it is as before (no mark, version 11, five
    /// properties printed). A new painting's box has it.
    #[test]
    fn the_drain_switch_comes_from_the_log_s_head() {
        assert!(logged_drain(&format!("{ENGINE_MARK} 5\n  {DRAIN_MARK}  \n{MARK} 1\n")));
        assert!(!logged_drain(&format!("{ENGINE_MARK} 5\n{MARK} 1\n{DRAIN_MARK}\n")));
        assert!(!logged_drain(&format!("{ENGINE_MARK} 5\n{DRAIN_MARK}-ish\n{MARK} 1\n")));
        let e = box_for(Some(&format!("{ENGINE_MARK} 4\n{DRAIN_MARK}\n{MARK} 1\n"))).unwrap_err();
        assert!(e.contains(DRAIN_MARK), "{e}");
        assert!(box_for(Some(&format!("{ENGINE_MARK} 5\n{DRAIN_MARK}\n{MARK} 1\n"))).unwrap().drain);
        assert!(!box_for(Some(&format!("{ENGINE_MARK} 5\n{MARK} 1\n"))).unwrap().drain);
        assert!(box_for(None).map(|b| b.drain).unwrap_or(true), "a new painting's box drains");
        let strokes = r#"b = brush("flat", 24)
            b:load(pile{{"raw umber", 1}}, 0.8)
            b:stroke({{100, 200}, {900, 200}})
            wait(30)"#;
        let state = |s: &Session| {
            let mut bytes = Vec::new();
            s.canvas().unwrap().write_state(&mut bytes, "").unwrap();
            bytes
        };
        for drain in [false, true] {
            let mut live = Session::new(96).unwrap().with_drain(drain);
            live.run(CANVAS).unwrap();
            live.run(strokes).unwrap();
            assert_eq!(live.canvas().unwrap().drain(), drain);
            let log = live.program("drain");
            assert_eq!(log.contains(&format!("{ENGINE_MARK} {}\n{DRAIN_MARK}\n\n{MARK} 1\n", paint::ENGINE)), drain, "{log}");
            assert_eq!(logged_drain(&log), drain);
            let mut replay = Session::replay_with(96, box_for(Some(&log)).unwrap()).unwrap();
            for chunk in parse_program(&log) {
                replay.run(&chunk).unwrap();
            }
            let bytes = state(&live);
            let original = live.canvas().unwrap().clone();
            assert_eq!(checkpoint_base_format(&bytes), if drain { 14 } else { 11 });
            assert_canvas_close(&replay.canvas().unwrap(), &original);
            assert_eq!(replay.program("drain"), log);
            let brushes = live.st.borrow_mut().live_brushes();
            let held = format!("{:?}", brushes[0].borrow());
            let hide = held.split("hide: [").nth(1).and_then(|h| h.split(']').next()).unwrap();
            assert_eq!(hide.matches(',').count(), if drain { 9 } else { 4 }, "drain {drain}: {hide}");
            live.rebuild().unwrap();
            assert_canvas_close(&live.canvas().unwrap(), &original);
            assert_eq!(live.program("drain"), log);
        }
    }
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

    /// An engine-2 painting replays as engine 2 with the current pigment mixer:
    /// a test sheet of lead white
    /// and bone black laid at different times over five days, lifted at
    /// every stage of drying and painted across, with waits of hours and
    /// days. Its log goes on naming engine 2.
    #[test]
    fn engine_2_logs_replay_as_before() {
        let (s, text) = replay_fixture("engine2/drying_sheet");
        assert_eq!(s.canvas().unwrap().engine(), 2);
        assert_eq!(s.program("painting"), text);
        for mode in ["normal", "gallery", "relief"] {
            let view = crate::look::View::parse(&["--mode".into(), mode.into(), "--size".into(), "320".into()]).unwrap();
            let (_, _, png) = crate::look::render(&s.canvas().unwrap(), &view).unwrap();
            let path = format!("{}/tests/engine2/drying_sheet_{mode}.png", env!("CARGO_MANIFEST_DIR"));
            if std::env::var_os("UPDATE_GOLDEN").is_some() {
                std::fs::write(&path, &png).unwrap();
            }
            let expected = image::open(path).unwrap().to_rgba8();
            image_tolerance::assert_close(&image::load_from_memory(&png).unwrap().to_rgba8(), &expected);
        }
    }

    /// A gesture keeps the pressure its points ask for up to its last point
    /// (engine 8): a limb drawn to where it forks keeps its width there, and
    /// meets the forks drawn from it. Before engine 8 the last 5% of every
    /// gesture lifted off, drawing the limb down to a dotted point short of
    /// the fork.
    #[test]
    fn a_gesture_keeps_its_width_to_its_last_point() {
        let text = "--@ engine 8\n";
        let mut s = Session::replay_with(2400, box_for(Some(text)).unwrap()).unwrap();
        s.run(r#"canvas{size=300, aspect=4, linen=16, seed=3, ground={{pile={{"lead white", 4}, {"yellow ochre", 0.3}}, um=80, apply="knife"}}}"#).unwrap();
        s.run(r#"local b = brush{kind="round", width=1.6, point=1, stiffness=0.5}
            b:load(pile{{"bone black", 1}, {"raw umber", 1}}, 0.6)
            b:gesture({{500, 220, 0.8}, {500, 150, 0.8}, {500, 80, 0.8}})"#).unwrap();
        let c = s.canvas().unwrap();
        let across = |y: f32| (-20..=20).map(|k| c.wet_um(500.0 + k as f32 * 0.1, y)).fold(0.0f32, f32::max);
        let (body, end) = (across(90.0), across(80.5));
        assert!(end >= 0.5 * body, "0.5 units before its end the gesture lays {end} µm, against {body} µm 10 units before it");
    }

    /// A new painting's blender feathers (`paint::Canvas::with_blend_feather`)
    /// and its brushes skim (`paint::Canvas::with_brush_skim`), and its log's
    /// head says so; a replay of that log and a save of it do too, and a log
    /// without the lines (every painting before them) does neither.
    /// Engines 7 and 8 have them.
    #[test]
    fn new_paintings_record_the_blend_feather_and_brush_skim_and_older_logs_replay_without_them() {
        let canvas = "canvas{size=440, aspect=1.25, linen=15, seed=7, ground={{pile={{\"lead white\", 3}, {\"yellow ochre\", 1}}, um=120, apply=\"knife\"}}}";
        let mut live = Session::with_box(240, box_for(None).unwrap()).unwrap();
        live.run(canvas).unwrap();
        let c = live.canvas().unwrap();
        assert!(c.blend_feather() && c.brush_skim());
        let text = live.program("p");
        for mark in [BLEND_FEATHER_MARK, BRUSH_SKIM_MARK] {
            assert!(text.contains(&format!("\n{mark}\n")), "{text}");
        }
        let mut saved = Vec::new();
        live.canvas().unwrap().write_state(&mut saved, "").unwrap();
        let (restored, _) = Canvas::read_state(&mut &saved[..]).unwrap();
        assert!(restored.blend_feather() && restored.brush_skim());
        for engine in [paint::engine7::ENGINE, paint::REACH_END] {
            for (feather, skim) in [(false, false), (false, true), (true, false), (true, true)] {
                let mut log = text.replace(&format!("{ENGINE_MARK} {}", paint::PIGMENT_MIXER), &format!("--@ engine {engine}"));
                for (on, mark) in [(feather, BLEND_FEATHER_MARK), (skim, BRUSH_SKIM_MARK)] {
                    if !on {
                        log = log.replace(&format!("{mark}\n"), "");
                    }
                }
                let mut replay = Session::replay_with(240, box_for(Some(&log)).unwrap()).unwrap();
                for chunk in parse_program(&log) {
                    replay.run(&chunk).unwrap();
                }
                let c = replay.canvas().unwrap();
                assert_eq!((c.blend_feather(), c.brush_skim()), (feather, skim));
                assert_eq!(replay.program("p"), log);
                let mut saved = Vec::new();
                c.write_state(&mut saved, "").unwrap();
                let (restored, _) = Canvas::read_state(&mut &saved[..]).unwrap();
                assert_eq!(restored.engine(), engine);
                assert_eq!((restored.blend_feather(), restored.brush_skim()), (feather, skim));
                assert!(restored.knife_skim());
            }
        }
        assert!(box_for(Some(&text.replace(&format!("{ENGINE_MARK} {}", paint::PIGMENT_MIXER), "--@ engine 5"))).is_err());
    }

    /// A new painting's knife skims (`paint::Canvas::with_knife_skim`) and
    /// its log's head says so; a replay of that log and a save of it skim
    /// too, and a log without the line (every painting before it) doesn't.
    /// Engines 7 and 8 can skim.
    #[test]
    fn new_paintings_record_the_knife_skim_and_older_logs_replay_without_it() {
        let canvas = "canvas{size=440, aspect=1.25, linen=15, seed=7, ground={{pile={{\"lead white\", 3}, {\"yellow ochre\", 1}}, um=120, apply=\"knife\"}}}";
        let mut live = Session::with_box(240, box_for(None).unwrap()).unwrap();
        live.run(canvas).unwrap();
        assert!(live.canvas().unwrap().knife_skim());
        let text = live.program("p");
        assert!(text.contains(&format!("\n{KNIFE_SKIM_MARK}\n")), "{text}");
        assert!(text.contains(&format!("{ENGINE_MARK} {}\n", paint::PIGMENT_MIXER)), "{text}");
        let mut saved = Vec::new();
        live.canvas().unwrap().write_state(&mut saved, "").unwrap();
        let restored = Canvas::read_state(&mut &saved[..]).unwrap().0;
        assert!(restored.knife_skim());
        assert_eq!(restored.engine(), paint::PIGMENT_MIXER);
        for (log, skims) in [
            (text.clone(), true),
            (text.replace(&format!("{KNIFE_SKIM_MARK}\n"), ""), false),
            (text.replace(&format!("{ENGINE_MARK} {}", paint::PIGMENT_MIXER), "--@ engine 7"), true),
            (text.replace(&format!("{ENGINE_MARK} {}", paint::PIGMENT_MIXER), "--@ engine 7").replace(&format!("{KNIFE_SKIM_MARK}\n"), ""), false),
        ] {
            let mut replay = Session::replay_with(240, box_for(Some(&log)).unwrap()).unwrap();
            for chunk in parse_program(&log) {
                replay.run(&chunk).unwrap();
            }
            assert_eq!(replay.canvas().unwrap().knife_skim(), skims);
            assert_eq!(replay.program("p"), log);
            let mut saved = Vec::new();
            replay.canvas().unwrap().write_state(&mut saved, "").unwrap();
            let restored = Canvas::read_state(&mut &saved[..]).unwrap().0;
            assert_eq!(restored.engine(), logged_engine(&log).unwrap());
            assert_eq!(restored.knife_skim(), skims);
        }
        assert!(box_for(Some(&text.replace(&format!("{ENGINE_MARK} {}", paint::PIGMENT_MIXER), "--@ engine 5"))).is_err());
    }

    /// A new painting's rag keeps what it lifts and stays inside a mask
    /// (`paint::Canvas::with_rag_lift`) and its log's head says so; a replay
    /// of that log and a save of it do too, and a log without the line (every
    /// painting before it) doesn't. Engines 7 and 8 have it.
    #[test]
    fn new_paintings_record_the_rag_lift_and_older_logs_replay_without_it() {
        let canvas = "canvas{size=440, aspect=1.25, linen=15, seed=7, ground={{pile={{\"lead white\", 3}, {\"yellow ochre\", 1}}, um=120, apply=\"knife\"}}}";
        let mut live = Session::with_box(240, box_for(None).unwrap()).unwrap();
        live.run(canvas).unwrap();
        assert!(live.canvas().unwrap().rag_lift());
        let text = live.program("p");
        assert!(text.contains(&format!("\n{RAG_LIFT_MARK}\n")), "{text}");
        let mut saved = Vec::new();
        live.canvas().unwrap().write_state(&mut saved, "").unwrap();
        assert!(Canvas::read_state(&mut &saved[..]).unwrap().0.rag_lift());
        for engine in [paint::engine7::ENGINE, paint::REACH_END] {
            for lifts in [false, true] {
                for other_flags in [false, true] {
                    let mut log = text.replace(&format!("{ENGINE_MARK} {}", paint::PIGMENT_MIXER), &format!("--@ engine {engine}"));
                    for (on, mark) in [(lifts, RAG_LIFT_MARK), (other_flags, KNIFE_SKIM_MARK), (other_flags, BLEND_FEATHER_MARK), (other_flags, BRUSH_SKIM_MARK)] {
                        if !on {
                            log = log.replace(&format!("{mark}\n"), "");
                        }
                    }
                    let mut replay = Session::replay_with(240, box_for(Some(&log)).unwrap()).unwrap();
                    for chunk in parse_program(&log) {
                        replay.run(&chunk).unwrap();
                    }
                    assert_eq!(replay.canvas().unwrap().rag_lift(), lifts);
                    assert_eq!(replay.program("p"), log);
                    let mut saved = Vec::new();
                    replay.canvas().unwrap().write_state(&mut saved, "").unwrap();
                    let (restored, _) = Canvas::read_state(&mut &saved[..]).unwrap();
                    assert_eq!(restored.engine(), engine);
                    assert_eq!(restored.rag_lift(), lifts, "rag lift's bit 39 must round-trip independently");
                    assert_eq!((restored.knife_skim(), restored.blend_feather(), restored.brush_skim()), (other_flags, other_flags, other_flags));
                }
            }
        }
        assert!(box_for(Some(&text.replace(&format!("{ENGINE_MARK} {}", paint::PIGMENT_MIXER), "--@ engine 5"))).is_err());
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

    #[test]
    fn mixing_and_adding_keep_the_oil_left_after_blotting() {
        for drain in [false, true] {
            let log = format!("--@ box sargent\n--@ engine 7\n{}", if drain { "--@ drain\n" } else { "" });
            let tubes = box_for(Some(&log)).unwrap();
            let zinc = tubes.tubes.iter().position(|tube| tube.name == "zinc white").unwrap();
            let blotted = tubes.pile(vec![(zinc, 1.0)]).paint(-0.5).oil - 1.0;
            let mut live = Session::with_box(32, tubes).unwrap();
            live.run("canvas{size=120, aspect=1, linen=15, seed=21, ground={{pile={{\"lead white\", 1}}, um=65, apply=\"knife\"}}}").unwrap();
            live.run(r#"p = pile{{"zinc white", 1}, blot=0.5}
                q = pile{{"zinc white", 1}}
                a = pile{{"zinc white", 1}, blot=0.5}
                a:add{{"zinc white", 1}}
                m = mix{{p, 1}, {q, 1}}
                n = mix{{a, 1}, {q, 1}}"#).unwrap();
            for (name, medium) in [("p", blotted), ("q", 0.0), ("a", blotted * 0.5), ("m", blotted * 0.5), ("n", blotted * 0.25)] {
                let pile = live.lua.globals().get::<mlua::AnyUserData>(name).unwrap().borrow::<crate::api::PileU>().unwrap().clone();
                let pile = crate::api::resolve(&live.st, pile);
                assert!((pile.medium - medium).abs() < 1e-6, "{name}, drain {drain}: stored {} instead of {medium}", pile.medium);
                assert!((pile.mix.paint(pile.medium).oil - (1.0 + medium)).abs() < 1e-6, "{name}, drain {drain}: its oil is conserved");
                assert!((live.st.borrow().board.heap(pile.heap).unwrap().medium - medium).abs() < 1e-6, "{name}: the heap carries the effective blotting");
            }
        }
    }

    #[test]
    fn new_paintings_record_liquid_film_leveling_and_old_logs_keep_it_off() {
        let mut live = Session::with_box(80, box_for(None).unwrap()).unwrap();
        live.run("canvas{size=440, aspect=1, linen=15, seed=7, ground={{pile={{\"lead white\", 1}}, um=120, apply=\"knife\"}}}").unwrap();
        assert!(live.canvas().unwrap().liquid_film_levels());
        let text = live.program("p");
        assert!(text.contains(&format!("\n{LIQUID_FILM_LEVELS_MARK}\n")));
        for (log, levels) in [(text.clone(), true), (text.replace(&format!("{LIQUID_FILM_LEVELS_MARK}\n"), ""), false)] {
            let b = box_for(Some(&log)).unwrap();
            assert_eq!(b.clone().liquid_film_levels, levels);
            assert_eq!(b.only(&["lead white"]).liquid_film_levels, levels);
            let mut replay = Session::replay_with(80, b).unwrap();
            for chunk in parse_program(&log) {
                replay.run(&chunk).unwrap();
            }
            assert_eq!(replay.canvas().unwrap().liquid_film_levels(), levels);
            assert_eq!(replay.program("p"), log);
            let mut saved = Vec::new();
            replay.canvas().unwrap().write_state(&mut saved, "").unwrap();
            let restored = Canvas::read_state(&mut &saved[..]).unwrap().0;
            assert_eq!(restored.liquid_film_levels(), levels);
            assert_eq!(restored.seen_lit(315.0, 55.0, 1.0), replay.canvas().unwrap().seen_lit(315.0, 55.0, 1.0));
            assert!(restored.glaze_film() && restored.rag_lift() && restored.knife_skim() && restored.blend_feather() && restored.brush_skim());
        }
        for engine in [4, 5, 7, 8] {
            assert!(box_for(Some(&format!("--@ engine {engine}\n{LIQUID_FILM_LEVELS_MARK}\n"))).unwrap().liquid_film_levels);
            assert!(!box_for(Some(&format!("--@ engine {engine}\n--@ chunk 1\n{LIQUID_FILM_LEVELS_MARK}\n"))).unwrap().liquid_film_levels);
        }
        assert!(box_for(Some(&format!("--@ engine 3\n{LIQUID_FILM_LEVELS_MARK}\n"))).is_err());
    }

    /// A new painting's glaze and scumble lay a thin film
    /// (`paint::Canvas::with_glaze_film`) and its log's head says so; a
    /// replay of that log and a save of it do too, and a log without the line
    /// (every painting before it) doesn't. Engines 7 and 8 have it.
    #[test]
    fn new_paintings_record_the_glaze_film_and_older_logs_replay_without_it() {
        let canvas = "canvas{size=440, aspect=1.25, linen=15, seed=7, ground={{pile={{\"lead white\", 3}, {\"yellow ochre\", 1}}, um=120, apply=\"knife\"}}}";
        let mut live = Session::with_box(240, box_for(None).unwrap()).unwrap();
        live.run(canvas).unwrap();
        assert!(live.canvas().unwrap().glaze_film());
        let text = live.program("p");
        assert!(text.contains(&format!("\n{GLAZE_FILM_MARK}\n")), "{text}");
        let mut saved = Vec::new();
        live.canvas().unwrap().write_state(&mut saved, "").unwrap();
        assert!(Canvas::read_state(&mut &saved[..]).unwrap().0.glaze_film());
        for (log, film) in [
            (text.clone(), true),
            (text.replace(&format!("{GLAZE_FILM_MARK}\n"), ""), false),
            (text.replace(&format!("{ENGINE_MARK} {}", paint::PIGMENT_MIXER), "--@ engine 7"), true),
            (text.replace(&format!("{ENGINE_MARK} {}", paint::PIGMENT_MIXER), "--@ engine 7").replace(&format!("{GLAZE_FILM_MARK}\n"), ""), false),
        ] {
            let mut replay = Session::replay_with(240, box_for(Some(&log)).unwrap()).unwrap();
            for chunk in parse_program(&log) {
                replay.run(&chunk).unwrap();
            }
            assert_eq!(replay.canvas().unwrap().glaze_film(), film);
            assert_eq!(replay.program("p"), log);
            let mut saved = Vec::new();
            replay.canvas().unwrap().write_state(&mut saved, "").unwrap();
            let (restored, _) = Canvas::read_state(&mut &saved[..]).unwrap();
            assert_eq!(restored.engine(), logged_engine(&log).unwrap());
            assert_eq!(restored.glaze_film(), film);
            assert!(restored.knife_skim() && restored.blend_feather() && restored.brush_skim());
        }
        assert!(box_for(Some(&text.replace(&format!("{ENGINE_MARK} {}", paint::PIGMENT_MIXER), "--@ engine 5"))).is_err());
    }

    /// A new painting 400 px wide (1000 × 800 units): a pale ground with
    /// `drawing` laid on it and left a month to dry, then `pass`. The
    /// CIE L* of every pixel before and after the pass, and the change in
    /// red minus blue (of 255).
    fn under_a_pass(drawing: &str, pass: &str) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        let mut s = Session::with_box(400, box_for(None).unwrap()).unwrap();
        s.run("canvas{size=440, aspect=1.25, linen=15, seed=7, ground={{pile={{\"lead white\", 3}, {\"yellow ochre\", 0.2}}, um=120, apply=\"knife\"}}}").unwrap();
        s.run(drawing).unwrap();
        s.run("wait(30*24*60)").unwrap();
        let lstar = |c: &[f32; 3]| {
            let y = paint::color::luminance(*c);
            if y > 216.0 / 24389.0 { 116.0 * y.cbrt() - 16.0 } else { 24389.0 / 27.0 * y }
        };
        let warm = |c: &[f32; 3]| 255.0 * (paint::color::linear_to_srgb(c[0]) - paint::color::linear_to_srgb(c[2]));
        let before = s.canvas().unwrap().seen();
        s.run(pass).unwrap();
        let after = s.canvas().unwrap().seen();
        (before.iter().map(lstar).collect(), after.iter().map(lstar).collect(), before.iter().zip(&after).map(|(b, a)| warm(a) - warm(b)).collect())
    }

    /// The pixels of that 400 × 320 canvas whose x lies in the middle 60
    /// units of a 100-unit bar: the `dark` bars, or the light ones between.
    fn bars(dark: bool) -> Vec<usize> {
        let mid = if dark { 20.0..80.0 } else { 120.0..180.0 };
        (8..312).flat_map(|y| (0..400).map(move |x| (y, x))).filter(|&(_, x)| mid.contains(&((x as f32 + 0.5) * 2.5 % 200.0))).map(|(y, x)| y * 400 + x).collect()
    }

    /// A glaze is a thin transparent film: over a dry drawing of dark bars a
    /// medium-rich warm pile (92% medium) brushed over the whole canvas
    /// shifts the color by about the same small amount everywhere and the
    /// drawing stays legible. Before the glaze film its strokes laid the
    /// pile's own pale color in streaks over the bars.
    #[test]
    fn a_glaze_tints_the_drawing_under_it_and_keeps_it_legible() {
        let drawing = "work(mask(function(x, y) return math.floor(x / 100) % 2 == 0 and 1 or 0 end), {hand=\"body\", pile=pile{{\"bone black\", 1}, {\"raw umber\", 1}}, coverage=3, fill=true, clip=true, angle=1.5708, seed=5})";
        let glaze = "work(everywhere(), {hand=\"glaze\", pile=pile{{\"lead white\", 0.8}, {\"yellow ochre\", 0.55}, {\"vermilion\", 0.12}, {\"bone black\", 0.1}, medium=0.92}, coverage=1, angle=0.05, seed=491})";
        let (l0, l1, warm) = under_a_pass(drawing, glaze);
        let mean = |v: &[f32], ix: &[usize]| ix.iter().map(|&i| v[i]).sum::<f32>() / ix.len() as f32;
        let (dark, light) = (bars(true), bars(false));
        let kept = (mean(&l1, &light) - mean(&l1, &dark)) / (mean(&l0, &light) - mean(&l0, &dark));
        let dl: Vec<f32> = light.iter().map(|&i| l1[i] - l0[i]).collect();
        let sd = (dl.iter().map(|d| d * d).sum::<f32>() / dl.len() as f32 - (dl.iter().sum::<f32>() / dl.len() as f32).powi(2)).sqrt();
        let shift = light.iter().chain(&dark).map(|&i| warm[i]).sum::<f32>() / (light.len() + dark.len()) as f32;
        eprintln!("glaze: contrast kept {kept}, L* change sd over the light bars {sd}, warm shift {shift}");
        assert!(kept > 0.8, "the drawing's contrast kept: {kept}");
        assert!(sd < 1.5, "the film is uneven over the light bars: L* change sd {sd}");
        assert!((2.0..12.0).contains(&shift), "the warm shift (R - B, of 255): {shift}");
    }

    /// A scumble is a broken veil: at coverage 0.5 a pale body pile scumbled
    /// over dry dark paint reaches about half of its rect and leaves the dark
    /// showing in the gaps, and where it lands the dark shows through it.
    /// Before the glaze film its scrubbing strokes closed over nearly all of
    /// it.
    #[test]
    fn a_scumble_at_half_coverage_leaves_gaps() {
        let drawing = "work(everywhere(), {hand=\"body\", pile=pile{{\"raw umber\", 1}, {\"bone black\", 0.3}, {\"lead white\", 0.1}}, coverage=3, fill=true, seed=5})";
        let scumble = "work(rect(100, 100, 800, 600), {hand=\"scumble\", pile=pile{{\"lead white\", 2}, {\"yellow ochre\", 0.4}, {\"raw umber\", 0.3}}, coverage=0.5, angle=0.2, seed=41})";
        let (l0, l1, _) = under_a_pass(drawing, scumble);
        let inside: Vec<usize> = (48..272).flat_map(|y| (48..352).map(move |x| y * 400 + x)).collect();
        let reached: Vec<usize> = inside.iter().copied().filter(|&i| l1[i] - l0[i] > 2.0).collect();
        let share = reached.len() as f32 / inside.len() as f32;
        let lift = reached.iter().map(|&i| l1[i] - l0[i]).sum::<f32>() / reached.len().max(1) as f32;
        let under = inside.iter().map(|&i| l0[i]).sum::<f32>() / inside.len() as f32;
        eprintln!("scumble: reached {share}, L* {under} up by {lift} where it landed");
        assert!((0.3..0.7).contains(&share), "the scumble reached {share} of its rect at coverage 0.5");
        assert!(lift < 0.75 * (80.0 - under), "where it lands it hides the dark: L* {under} up by {lift}");
    }
    /// The paper-and-pastel switch is read from a log's head only, needs
    /// engine 5, and is what lets `canvas{paper=}` set up a sheet: without it
    /// (an older painting's log) paper is refused by name; with it the log
    /// says so, and replays, rebuilds and checkpoints the same sheet.
    #[test]
    fn paper_comes_with_the_switch_in_the_log_s_head() {
        assert!(logged_paper(&format!("{ENGINE_MARK} 5\n  {PAPER_MARK}  \n{MARK} 1\n")));
        assert!(!logged_paper(&format!("{ENGINE_MARK} 5\n{MARK} 1\n{PAPER_MARK}\n")));
        assert!(!logged_paper(&format!("{ENGINE_MARK} 5\n{PAPER_MARK}-ish\n{MARK} 1\n")));
        let e = box_for(Some(&format!("{ENGINE_MARK} 4\n{PAPER_MARK}\n{MARK} 1\n"))).unwrap_err();
        assert!(e.contains(PAPER_MARK), "{e}");
        let sheet = r#"canvas{size=200, aspect=1.25, seed=3, paper={grammage=200, felt=false}}"#;
        let mut old = Session::new(96).unwrap();
        let e = old.run(sheet).unwrap_err();
        assert!(e.contains(PAPER_MARK), "{e}");
        old.run(CANVAS).unwrap();
        assert!(!logged_paper(&old.program("old")));
        let mut live = Session::new(96).unwrap().with_paper(true).unwrap();
        live.run(sheet).unwrap();
        let log = live.program("paper");
        assert!(log.contains(&format!("{ENGINE_MARK} {}\n{PAPER_MARK}\n\n{MARK} 1\n", paint::ENGINE)), "{log}");
        let mut replay = Session::replay_with(96, box_for(Some(&log)).unwrap()).unwrap().with_paper(logged_paper(&log)).unwrap();
        for chunk in parse_program(&log) {
            replay.run(&chunk).unwrap();
        }
        let state = |s: &Session| {
            let mut bytes = Vec::new();
            s.canvas().unwrap().write_state(&mut bytes, "").unwrap();
            bytes
        };
        let bytes = state(&live);
        let original = live.canvas().unwrap().clone();
        assert_eq!(checkpoint_base_format(&bytes), 13);
        assert_canvas_close(&replay.canvas().unwrap(), &original);
        assert!(replay.st.borrow().paper);
        assert_eq!(replay.program("paper"), log);
        live.rebuild().unwrap();
        assert_canvas_close(&live.canvas().unwrap(), &original);
        assert!(live.st.borrow().paper);
        assert_eq!(live.program("paper"), log);
    }

    /// A save keeps the paper-and-pastel switch and the sheet: read back
    /// (`save::read`), the session has the switch, the same canvas bit for
    /// bit (a sheet: version 13), paints on as the live one does, and writes
    /// the switch into its log's head.
    #[test]
    fn paper_survives_a_save() {
        let sheet = r#"canvas{size=200, aspect=1.25, seed=3, paper={grammage=200, felt=false}}"#;
        for setup in [CANVAS, sheet] {
            let path = std::env::temp_dir().join(format!("easel-paper-save-{}-{}.ckpt", std::process::id(), setup.len()));
            let mut live = Session::new(96).unwrap().with_paper(true).unwrap();
            live.run(setup).unwrap();
            live.run("b = brush(\"round\", 6); b:load(pile{{\"raw umber\", 1}}, 0.6); b:stroke({{100, 300}, {800, 320}})").unwrap();
            crate::save::write(&live, &live.program("t"), &path).unwrap();
            let mut restored = crate::save::read(&path).unwrap().session;
            let saved = |s: &Session| {
                let mut bytes = Vec::new();
                s.canvas().unwrap().write_state(&mut bytes, "").unwrap();
                bytes
            };
            assert!(restored.st.borrow().paper);
            assert_eq!(saved(&restored), saved(&live));
            assert_eq!(checkpoint_base_format(&saved(&live)) == 13, setup == sheet);
            restored.run("wait(1)").unwrap();
            live.run("wait(1)").unwrap();
            assert_canvas_close(&restored.canvas().unwrap(), &live.canvas().unwrap());
            assert!(logged_paper(&restored.program("t")));
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[cfg(test)]
mod progress {
    use super::*;

    #[test]
    fn a_kept_chunk_is_reported_as_in_the_log() {
        let p = Progress { lines: AtomicUsize::new(6), line: AtomicUsize::new(5), marks0: AtomicU64::new(0), marks: AtomicU64::new(0), clock0: AtomicU64::new(0), clock: AtomicU64::new(0), kept: AtomicUsize::new(0) };
        assert!(p.report(Duration::from_secs(3)).starts_with("painting a chunk (at line 5 of 6)"));
        p.keep(2);
        assert_eq!(p.report(Duration::from_secs(75)), "the chunk is in the log (chunk 2); the easel is finishing its reply, running for 1 min 15 s");
        p.clear();
        assert!(p.report(Duration::from_secs(1)).starts_with("painting a chunk: starting"));
    }
}

/// The Lua memory cap (LUA_MAX_BYTES, Serials::held, Serials::capped).
#[cfg(test)]
mod memory_cap_tests {
    use super::*;

    const W: usize = 160;

    const CANVAS: &str = r#"canvas{size=440, aspect=1.5, linen=15, seed=2, ground={{pile={{"lead white", 3}, {"red earth", 1}}, um=110, apply="knife"}, {pile={{"lead white", 5}, {"yellow ochre", 1}}, um=60, apply="brush"}}}"#;

    /// A chunk that asks for more memory than the cap errors and leaves the
    /// session as it was, instead of swapping the machine. 8 GB is past
    /// LUA_MAX_BYTES, a constant, so this never allocates it.
    #[test]
    fn a_runaway_allocation_errors() {
        let mut s = Session::new(W).unwrap();
        s.run(CANVAS).unwrap();
        let e = s.run("x = string.rep('x', 8 * 1024 * 1024 * 1024)").unwrap_err();
        assert!(e.contains("memory"), "{e}");
        s.run("assert(x == nil)").unwrap();
    }

    /// A chunk that catches the cap's error still fails: a live heap holds a
    /// snapshot a replay's doesn't, so what the cap refused live a replay
    /// might allow, and the chunk would paint differently there.
    #[test]
    fn a_caught_runaway_allocation_still_fails_the_chunk() {
        let mut s = Session::new(W).unwrap();
        s.run(CANVAS).unwrap();
        let e = s.run("ok = pcall(string.rep, 'x', 8 * 1024 * 1024 * 1024)").unwrap_err();
        assert!(e.contains("memory"), "{e}");
        assert!(!s.globals.contains_key("ok"), "the failed chunk's globals were kept");
        s.run("assert(ok == nil)").unwrap();
        assert_eq!(s.log.len(), 2, "the failed chunk was logged");
    }

    /// A painting that fits under the cap can still be snapshotted, so the
    /// next chunk runs (and can let the big table go) though heap and
    /// snapshot together are past the cap.
    #[test]
    fn a_painting_near_the_cap_can_still_paint() {
        let mut s = Session::new(W).unwrap();
        s.run(CANVAS).unwrap();
        s.run("t = {} for i = 1, 2000000 do t[i] = i end").unwrap();
        s._serials.cap.set(s._serials.used.get() + (1 << 20));
        s.run("assert(#t == 2000000) t = nil").unwrap();
        s.run("u = 1").unwrap();
        assert_eq!(s.log.len(), 4);
    }

    /// The snapshot's credit is the snapshot's blocks as they are now: its
    /// temporaries, once collected, are no longer credited, and none is left
    /// once the snapshot is gone.
    #[test]
    fn the_snapshot_credit_follows_its_blocks() {
        let mut s = Session::new(W).unwrap();
        s.run(CANVAS).unwrap();
        s.run("t = {} for i = 1, 20000 do t[i] = {i} end").unwrap();
        let snap = s.snap().unwrap();
        let taken = s._serials.held.get();
        assert!(taken > 0);
        s.lua.gc_collect().unwrap();
        assert!(s._serials.held.get() < taken, "the snapshot's collected temporaries are still credited");
        drop(snap);
        s.lua.gc_collect().unwrap();
        assert_eq!(s._serials.held.get(), 0, "credit left after the snapshot is gone");
    }
}
