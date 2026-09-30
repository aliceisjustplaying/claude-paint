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

/// The longest a chunk of a live session may run (the longest of 2,502 painters' chunks on
/// 2026-09-27 took 94 s). A chunk that runs longer is stopped like a failed one: nothing it
/// did is kept. Replays have no limit: a chunk in the log succeeded once and must replay the
/// same on a slower or busier machine.
pub const CHUNK_LIMIT: Duration = Duration::from_secs(600);
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
    seed: u64,
    clock: f64,
    clock0: f64,
    hand: crate::time::Hand,
    /// The last world view made (depth options resolve against it).
    view: Option<crate::world::ViewU>,
    /// The Lua heap (heap.lua's snapshot).
    heap: Table,
    brushes: Vec<(Rc<RefCell<Held>>, Held)>,
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
    /// log: the state is rebuilt from the log (`rebuild`) before the next chunk.
    pub stale: bool,
    /// Stale because putting back what a failed chunk did failed (not just a layout).
    pub unrestored: bool,
    /// Every global as the last successful chunk left it, with the chunk that last assigned it
    /// (0: the easel's own). `run` updates it by comparing the globals after each chunk with
    /// it, in a live session and a replay alike, so a reopen or rebuild gives the same answer.
    globals: BTreeMap<String, (Value, usize)>,
    /// The easel's own globals (its verbs, prelude.lua's replacements, Lua's libraries).
    own: BTreeMap<String, Value>,
    /// Creation serials of the state's objects (outlives the state).
    _serials: Box<Serials>,
    /// Tests: the step of `run` that fails ("flush", "globals" or "restore").
    #[cfg(test)]
    pub fail_at: Cell<Option<&'static str>>,
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
        let (snap_f, restore_f): (Function, Function) = lua.load(include_str!("heap.lua")).set_name("heap.lua").call(dbg.clone())?;
        let id = serials.id_fn(&lua)?;
        let getmt: Function = dbg.get("getmetatable")?;
        let getinfo: Function = dbg.get("getinfo")?;
        // an error of the easel's own, for the prelude to know them by
        let fail = lua.create_function(|_, ()| Err::<(), _>(mlua::Error::runtime("")))?;
        let prelude: (Function, Table, Function, Function) = lua.load(include_str!("prelude.lua")).set_name("prelude.lua").call((id, getmt, getinfo, fail))?;
        let st = Rc::new(RefCell::new(Studio::new(width, tubes)));
        api::install(&lua, st.clone())?;
        let deadline = Rc::new(Cell::new(None::<Instant>));
        let d = deadline.clone();
        lua.set_hook(mlua::HookTriggers::new().every_nth_instruction(HOOK_EVERY), move |_, _| match d.get() {
            Some(t) if Instant::now() > t => Err(mlua::Error::runtime(format!(
                "the chunk ran longer than {} minutes and was stopped; nothing it did was kept",
                CHUNK_LIMIT.as_secs() / 60
            ))),
            _ => Ok(mlua::VmState::Continue),
        })?;
        let own = global_values(&lua)?;
        let globals = own.iter().map(|(k, v)| (k.clone(), (v.clone(), 0))).collect();
        Ok(Session { lua: ManuallyDrop::new(lua), state, st, log: Vec::new(), replay: false, deadline, chunk_limit: CHUNK_LIMIT, heap: Some((snap_f, restore_f)), prelude: Some(prelude), stale: false, unrestored: false, globals, own, _serials: serials, #[cfg(test)] fail_at: Cell::new(None) })
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
        Ok(Snap { canvas: s.canvas.clone(), style: s.style.clone(), setup: s.setup.clone(), seed: s.seed, clock: s.clock, clock0: s.clock0, hand: s.hand.clone(), view: s.view.clone(), heap, brushes })
    }

    /// Put everything back as it was at `snap`. Returns how many Lua tables
    /// still differ in contents, traversal order or raw length after restoration (see `stale`).
    fn restore(&mut self, snap: &Snap) -> mlua::Result<usize> {
        let (_, restore_f) = self.heap.as_ref().unwrap();
        let mismatches = restore_f.call::<usize>(snap.heap.clone())?;
        self.inject("restore")?;
        for (b, h) in &snap.brushes {
            *b.borrow_mut() = h.clone();
        }
        let mut s = self.st.borrow_mut();
        s.canvas = snap.canvas.clone();
        s.style = snap.style.clone();
        s.setup = snap.setup.clone();
        s.seed = snap.seed;
        s.clock = snap.clock;
        s.clock0 = snap.clock0;
        s.hand = snap.hand.clone();
        s.view = snap.view.clone();
        Ok(mismatches)
    }

    /// Tests: fail here if `fail_at` says so (nothing, outside tests).
    fn inject(&self, _at: &str) -> mlua::Result<()> {
        #[cfg(test)]
        if self.fail_at.get() == Some(_at) {
            return Err(mlua::Error::runtime(format!("injected failure: {_at}")));
        }
        Ok(())
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
        let n = self.log.len() as u64 + 1;
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
            if self.inject("flush").is_err() {
                panic!("injected failure: flush");
            }
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
        for (i, c) in self.log.iter().enumerate() {
            let _ = writeln!(s, "\n{MARK} {}", i + 1);
            s.push_str(&c.src);
            s.push('\n');
        }
        s
    }

    /// The globals as chunk `n` left them: the ones it set, changed or removed are its.
    fn next_globals(&self, n: usize) -> mlua::Result<BTreeMap<String, (Value, usize)>> {
        self.inject("globals")?;
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
            let v = rest.strip_prefix(' ').and_then(|v| v.trim().parse::<u32>().ok()).filter(|v| (1..=paint::ENGINE).contains(v));
            let Some(v) = v else {
                return Err(format!("the log's line {l:?} names no engine this easel has ({ENGINE_MARK} <1 to {}>)", paint::ENGINE));
            };
            if found.is_some() {
                return Err(format!("the log names its engine twice ({ENGINE_MARK} lines)"));
            }
            found = Some(v);
        }
    }
    Ok(found.unwrap_or(1))
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
/// engine version the painting is painted with: a new one the current
/// (`paint::ENGINE`), an existing one its log's (`logged_engine`).
pub fn box_for(log: Option<&str>) -> Result<Palette, String> {
    let configured = configured_box()?;
    let Some(text) = log else {
        return find_box(configured.as_ref().map_or(Palette::fallback_box(), |c| c.0.as_str()));
    };
    let engine = logged_engine(text)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(tube_box)]
    const W: usize = 160;

    #[cfg(tube_box)]
    fn bits(s: &Session) -> Vec<u32> {
        s.canvas().unwrap().seen().iter().flat_map(|p| p.map(f32::to_bits)).collect()
    }

    #[cfg(tube_box)]
    pub(crate) const CANVAS: &str = r#"canvas{size=440, aspect=1.5, linen=15, seed=2, ground={{pile={{"lead white", 3}, {"red earth", 1}}, um=110, apply="knife"}, {pile={{"lead white", 5}, {"yellow ochre", 1}}, um=60, apply="brush"}}}"#;

    #[cfg(tube_box)]
    const CHUNKS: [&str; 4] = [
        CANVAS,
        r#"p = pile{{"lead white", 6}, {"cobalt blue", 1}, {"yellow ochre", 0.5}, medium=0.3}
           work(ellipse(500, 300, 300, 150), {hand="broad", pile=p, angle=0, coverage=2})"#,
        r#"b = brush("round", 4); b:load(pile{{"bone black", 1}, {"raw umber", 2}}, 0.9)
           for i = 1, 5 do b:stroke({{100 + i*60, 500}, {130 + i*60 + rand(-10, 10), 420}}) end"#,
        r#"wait(90); stipple(rect(100, 380, 800, 200), {width=3, pile=pile{{"lead white", 1}}, coverage=1.5})"#,
    ];

    #[test]
    #[cfg(tube_box)]
    fn replay_is_exact_and_failures_roll_back() {
        let mut a = Session::new(W).unwrap();
        for (i, c) in CHUNKS.iter().enumerate() {
            a.run(c).unwrap();
            if i == 1 {
                // a failing chunk that painted and set globals first changes nothing
                let before = bits(&a);
                let e = a.run(r#"junk = 1; b0 = brush("flat", 6); b0:load(p); b0:stroke({0, 0, 900, 600}); work(everywhere(), {pile=p, colour="red"})"#).unwrap_err();
                assert!(e.contains("unknown option \"colour\""), "{e}");
                assert_eq!(before, bits(&a));
                assert!(a.run("assert(junk == nil and b0 == nil)").is_ok());
            }
        }
        assert_eq!(a.log.len(), CHUNKS.len() + 1);
        // the log replays to the same canvas, bit for bit
        let prog = a.program("t");
        let chunks = parse_program(&prog);
        assert_eq!(chunks.len(), CHUNKS.len() + 1);
        let mut b = Session::replay(W).unwrap();
        for c in &chunks {
            b.run(c).unwrap();
        }
        assert_eq!(bits(&a), bits(&b));
        assert_eq!(a.st.borrow().clock, b.st.borrow().clock);
    }

    #[test]
    #[cfg(tube_box)]
    fn globals_are_the_painting_s_as_the_state_holds_them_live_and_replayed() {
        let chunks = [
            CANVAS,
            "local dm = nil\ndm = 3\na = 1; b = 2\ndo\n  inner = \"x\"\nend\nif true then\n  deep = {1, 2, 3}\nend\nlocal s = [[\nfake = 1\n]]\n-- ghost = 2\n--[[\nphantom = 3\n]]\nfunction tree(x)\n  return x\nend\ngone = 1\nn = 0/0\nm = rect(0, 0, 10, 10)",
            "gone = nil; b = 2; a = 5",
        ];
        let mut live = Session::new(W).unwrap();
        for (i, c) in chunks.iter().enumerate() {
            live.run(c).unwrap();
            if i == 1 {
                live.run("c = 1; b = 7; error('no')").unwrap_err();
            }
        }
        // canvas{} sets W and H
        let want = "1\tH\tnumber 668.75\n1\tW\tnumber 1000\n2\tb\tnumber 2\n2\tdeep\ttable with 3 entries\n2\tinner\tstring \"x\"\n2\tm\tmask(88 sq units)\n2\tn\tnumber NaN\n2\ttree\tfunction (chunk 2, line 17)\n3\ta\tnumber 5\n";
        assert_eq!(live.globals(), want);
        let mut replay = Session::replay(W).unwrap();
        for c in parse_program(&live.program("t")) {
            replay.run(&c).unwrap();
        }
        assert_eq!(replay.globals(), want);
    }

    /// Lua loads precompiled chunks, which it doesn't verify (a crafted one can crash the
    /// easel): a painting loads text only, by every way in, and can't dump functions.
    #[test]
    #[cfg(tube_box)]
    fn binary_chunks_are_refused_and_text_load_still_works() {
        let mut s = Session::new(64).unwrap();
        // a benign binary chunk, dumped by another state of the same Lua
        let other = Lua::new();
        let bin = other.load("return 7").into_function().unwrap().dump(true);
        s.lua.globals().set("bin", s.lua.create_string(&bin).unwrap()).unwrap();
        let mut failed = Vec::new();
        for chunk in [
            "assert(string.dump == nil, 'string.dump is there')",
            "local f, e = load(bin); assert(f == nil and e:find('binary chunk'), e)",
            "for _, m in ipairs{'b', 'bt', 'tb'} do assert(load(bin, 'x', m) == nil, m) end",
            "local n = 0; local f = load(function() n = n + 1; if n == 1 then return bin end end); assert(f == nil)",
            // text loading is as Lua's, with and without an environment
            "assert(load('return 1 + 1')() == 2); x = 9; assert(load('return x')() == 9)",
            "assert(load('return x', 'c', 't', {x = 5})() == 5); assert(load('return x', 'c', nil, {x = 6})() == 6)",
            "assert(not pcall(load('return x', 'c', 't', nil)))",
            "assert(load('return 1', 'c', 'b') == nil)",
            // load's own helpers are its own: replacing the global pcall, string.find or
            // string.gsub hands a painting no function that loads bytecode
            "local got, real = {}, {pcall, string.find, string.gsub}; \
             local function spy(f) return function(g, ...) got[#got + 1] = g; return f(g, ...) end end; \
             pcall, string.find, string.gsub = spy(real[1]), spy(real[2]), spy(real[3]); \
             load('return 1'); load(bin); load('return (', 'c'); load('x', 'c', 1); \
             pcall, string.find, string.gsub = real[1], real[2], real[3]; \
             for _, g in ipairs(got) do assert(type(g) ~= 'function' or not pcall(g, bin), 'a load of bytecode leaked') end",
        ] {
            if let Err(e) = s.run(chunk) {
                failed.push(format!("{chunk}: {e}"));
            }
        }
        // a chunk sent to the easel is loaded as text: one starting like a binary chunk is
        // refused as one, not read as bytecode
        let e = s.run("\x1bLua = 1").unwrap_err();
        if !e.contains("attempt to load a binary chunk (mode is 't')") {
            failed.push(format!("\\x1bLua chunk loaded as binary: {e}"));
        }
        assert!(failed.is_empty(), "{failed:#?}");
    }

    /// Nothing a painting can print or branch on shows a memory address (which differs from
    /// process to process, so between the live easel and a replay of its log): objects print
    /// as `<type>: (hidden)`, `%p` is refused, and errors carrying objects say no address.
    #[test]
    #[cfg(tube_box)]
    fn no_memory_address_reaches_a_painting() {
        let shown = |s: &mut Session| {
            let mut text = String::new();
            for chunk in [
                "local t, f = {}, function() end\nprint(t, f, print, string, setmetatable({}, {__name = 'Thing'}))\nprint(tostring(t), tostring(f), tostring(print))",
                "local t = {}\nprint(string.format('%s|%8s|%-8s|%.3s', t, t, print, t), ('%s'):format(function() end))",
                "print(setmetatable({}, {__tostring = function() return 'mine' end}), tostring(1.5), tostring(nil), tostring('s'))",
                "for _, a in ipairs{{}, 's', 1} do local ok, e = pcall(string.format, '%p', a); assert(not ok, 'no %p'); print(e) end",
                "error({})",
                "error(function() end)",
                "error(setmetatable({}, {}))",
                "local t = {}; t = t .. 1",
            ] {
                match s.run(chunk) {
                    Ok(r) => text.push_str(&r.out),
                    Err(e) => text.push_str(&e),
                }
                text.push('\n');
            }
            text
        };
        let a = shown(&mut Session::new(64).unwrap());
        // another state, its objects elsewhere in memory
        let _elsewhere: Vec<Vec<u8>> = (0..1000).map(|i| vec![0; i]).collect();
        let b = shown(&mut Session::new(64).unwrap());
        let hexes = a.split(|c: char| !c.is_ascii_hexdigit()).filter(|w| w.len() >= 8).collect::<Vec<_>>();
        assert!(!a.contains("0x") && hexes.is_empty(), "an address shows:\n{a}");
        assert_eq!(a, b, "two processes show different text");
        for want in ["table: (hidden)\tfunction: (hidden)\tfunction: (hidden)\ttable: (hidden)\tThing: (hidden)", "mine\t1.5\tnil\ts", "table: (hidden)|table: (hidden)|function: (hidden)|tab"] {
            assert!(a.contains(want), "{want:?} not in\n{a}");
        }
    }

    /// An error object raised in a painter's function the easel calls (a mask's, a curve's)
    /// reaches the painting, caught or not, without an address either: Lua's conversion of
    /// it to text happens outside the painting's reach (mlua's handler), so it's the text
    /// that hides the address.
    #[test]
    #[cfg(tube_box)]
    fn no_memory_address_escapes_a_callback() {
        let raised = ["{}", "function() end", "setmetatable({}, {__name = 'Thing'})", "noise()"];
        let mut chunks = Vec::new();
        for v in raised {
            for call in ["mask(function() error(V) end)", "below(function() error(V) end)"] {
                let call = call.replace('V', v);
                chunks.push(format!("local ok, e = pcall(function() return {call} end); assert(not ok); print(e, tostring(e), string.format('%s|%-9s', e, e), ('%s'):format(e))"));
                chunks.push(call);
            }
        }
        for run in 0..100 {
            let mut s = Session::new(64).unwrap();
            s.run(r#"canvas{size=100, aspect=1, seed=5, linen=15, ground={{pile={{"lead white", 1}}, um=80, apply="knife"}}}"#).unwrap();
            let mut text = String::new();
            for c in &chunks {
                match s.run(c) {
                    Ok(r) => text.push_str(&r.out),
                    Err(e) => text.push_str(&e),
                }
                text.push('\n');
            }
            assert!(!text.contains(": 0x"), "run {run}: an address shows:\n{text}");
            if run == 0 {
                for want in ["table: (hidden)", "function: (hidden)", "Thing: (hidden)", "Noise: (hidden)"] {
                    assert!(text.contains(want), "{want:?} not in\n{text}");
                }
            }
        }
    }

    /// A chunk that ran but whose ending failed (putting its hand time on the clock,
    /// reading the globals it left) is a failed chunk: nothing it did is kept, and the
    /// session goes on as a replay of its log does.
    #[test]
    #[cfg(tube_box)]
    fn a_chunk_whose_ending_fails_changes_nothing() {
        for at in ["globals", "flush"] {
            let mut s = Session::new(W).unwrap();
            s.run(CHUNKS[0]).unwrap();
            s.run(CHUNKS[1]).unwrap();
            let (before, clock, globals) = (bits(&s), s.st.borrow().clock, s.globals());
            s.fail_at.set(Some(at));
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| s.run(r#"junk = 1; work(everywhere(), {hand="broad", pile=p, coverage=1})"#)));
            s.fail_at.set(None);
            let e = match r {
                Ok(r) => r.expect_err(at),
                Err(_) => panic!("{at}: the failure escaped the chunk's guard"),
            };
            assert!(e.contains(&format!("injected failure: {at}")), "{at}: {e}");
            assert!(before == bits(&s), "{at}: the canvas kept what the chunk painted");
            assert_eq!((clock, globals, s.log.len()), (s.st.borrow().clock, s.globals(), 2), "{at}");
            s.run("assert(junk == nil)").unwrap();
            s.run(CHUNKS[2]).unwrap();
            let mut b = Session::replay(W).unwrap();
            for c in parse_program(&s.program("t")) {
                b.run(&c).unwrap();
            }
            assert!(bits(&s) == bits(&b), "{at}: the session went on differently from its log");
        }
    }

    /// Putting back what a failed chunk did can fail too: the session then paints no more
    /// from that state, but rebuilds from its log first, and says so.
    #[test]
    #[cfg(tube_box)]
    fn a_failed_rollback_rebuilds_from_the_log_before_painting_again() {
        let mut s = Session::new(W).unwrap();
        s.run(CHUNKS[0]).unwrap();
        s.run(CHUNKS[1]).unwrap();
        let before = bits(&s);
        s.fail_at.set(Some("restore"));
        let e = s.run(r#"junk = 1; work(everywhere(), {hand="broad", pile=p, coverage=1}); error("no")"#).unwrap_err();
        s.fail_at.set(None);
        assert!(e.contains("no") && e.contains("couldn't put back"), "{e}");
        assert!(s.stale, "a session that couldn't roll back went on");
        s.run("assert(junk == nil)").unwrap();
        assert!(before == bits(&s), "the canvas kept what the failed chunk painted");
    }

    #[test]
    #[cfg(tube_box)]
    fn terrain_refuses_huge_and_empty_areas_before_allocating() {
        let mut s = Session::new(64).unwrap();
        let h = "height=function(x, y) return 0 end";
        let e = s.run(&format!("terrain{{area={{0, 0, 1e9, 1e9}}, {h}}}")).unwrap_err();
        assert!(e.contains("at most 4001 x 4001"), "{e}");
        let e = s.run(&format!("terrain{{area={{5, 5, 5, 50}}, {h}}}")).unwrap_err();
        assert!(e.contains("at least one step"), "{e}");
        s.run(&format!("t = terrain{{area={{0, 0, 20, 20}}, {h}}}")).unwrap();
    }

    #[test]
    #[cfg(tube_box)]
    fn a_live_chunk_over_the_limit_stops_and_keeps_nothing_but_a_replay_runs_it() {
        let busy = "x = 1; for i = 1, 3000000 do end; x = 2";
        let mut s = Session::new(64).unwrap();
        s.chunk_limit = Duration::ZERO;
        let e = s.run(busy).unwrap_err();
        assert!(e.contains("was stopped"), "{e}");
        assert!(s.log.is_empty());
        assert!(s.lua.globals().get::<Value>("x").unwrap().is_nil());
        s.set_replaying(true);
        s.run(busy).unwrap();
        assert_eq!(s.lua.globals().get::<i64>("x").unwrap(), 2);
    }

    #[test]
    #[cfg(tube_box)]
    fn rollback_restores_tables_and_upvalues_exactly() {
        let mut s = Session::new(W).unwrap();
        s.run(CHUNKS[0]).unwrap();
        s.run(r##"things = {1, 2, {x = 3}}; local n = 0; function bump() n = n + 1; return n end; setmetatable(things, {tag = "a"})"##).unwrap();
        // a failing chunk that edits old tables, an upvalue and a metatable
        let e = s.run(r##"things[1] = 99; things[3].x = nil; things[4] = {}; bump(); getmetatable(things).tag = "b"; string.custom = 1; error("stop")"##).unwrap_err();
        assert!(e.contains("stop"), "{e}");
        s.run(r##"assert(things[1] == 1 and things[3].x == 3 and things[4] == nil); assert(bump() == 1); assert(getmetatable(things).tag == "a"); assert(string.custom == nil)"##).unwrap();
        // and the live session still replays exactly
        let mut b = Session::replay(W).unwrap();
        for c in &s.log {
            b.run(&c.src).unwrap();
        }
        assert_eq!(bits(&s), bits(&b));
    }

    /// A failed chunk can leave a table laid out differently (grown, or refilled by the
    /// rollback) though its contents are back: `pairs` then walks it in another order than
    /// a replay of the log does, unless the session rebuilds from the log.
    #[test]
    #[cfg(tube_box)]
    fn after_a_failed_chunk_pairs_walks_tables_as_the_replay_does() {
        let order = "local o = {}; for k in pairs(t) do o[#o + 1] = k end; print(table.concat(o, ' '))";
        let mut s = Session::new(W).unwrap();
        let mut live = vec![s.run("t = {}; for i = 1, 30 do t['k' .. i] = i end").unwrap().out];
        // a failure that touched no earlier table costs no rebuild
        s.run("local u = {}; u.x = pencil(); error('stop')").unwrap_err();
        assert!(!s.stale);
        // Updating an existing array value restores both contents and next order;
        // it must not pay for an unnecessary replay.
        s.run("a = {1, 2, 3}").unwrap();
        live.push(String::new());
        s.run("a[2] = 99; error('stop')").unwrap_err();
        assert!(!s.stale, "an exactly restored table should not rebuild");
        assert_eq!(s.lua.globals().get::<Table>("a").unwrap().get::<i64>(2).unwrap(), 2);
        // Sparse-array length is observable too, even if next order is unchanged.
        live.push(s.run("sparse = {}; sparse[2] = 2; sparse_length = #sparse").unwrap().out);
        s.run("for i = 9, 13 do sparse[i] = i end; for i = 9, 13 do sparse[i] = nil end; sparse[2] = 99; error('stop')").unwrap_err();
        live.push(s.run("assert(#sparse == sparse_length)").unwrap().out);
        // grows the table and empties it again: the same contents, a bigger table
        s.run("for i = 1, 200 do t['x' .. i] = i end; for i = 1, 200 do t['x' .. i] = nil end; error('stop')").unwrap_err();
        live.push(s.run(order).unwrap().out);
        // leaves keys behind for the rollback to take out
        s.run("for i = 1, 200 do t['y' .. i] = i end; error('stop')").unwrap_err();
        live.push(s.run(order).unwrap().out);
        let mut b = Session::replay(W).unwrap();
        let replayed: Vec<String> = s.log.iter().map(|c| b.run(&c.src).unwrap().out).collect();
        assert_eq!(live, replayed);
    }

    /// The pencil's shared methods are out of a chunk's reach, so a failed chunk can't
    /// take one away (nor add one a replay wouldn't have).
    #[test]
    #[cfg(tube_box)]
    fn a_failed_chunk_cannot_change_pencil_methods() {
        let mut s = Session::new(W).unwrap();
        s.run("local p = pencil(); getmetatable(p).__index.line = nil; error('stop')").unwrap_err();
        s.run("assert(type(pencil().line) == 'function')").unwrap();
    }

    /// Finalizers and weak tables act when the collector gets to them, which differs between
    /// a live session (it holds a snapshot) and a replay: a painting can't use them.
    #[test]
    #[cfg(tube_box)]
    fn finalizers_and_weak_tables_are_refused() {
        let mut s = Session::new(W).unwrap();
        for mt in ["{__gc = function() end}", "{__mode = 'v'}"] {
            let e = s.run(&format!("w = setmetatable({{}}, {mt})")).unwrap_err();
            assert!(e.contains("__gc and __mode"), "{e}");
        }
        // nor put in a metatable after it is set
        s.run("mt = {}; w = setmetatable({}, mt)").unwrap();
        let e = s.run("mt.__mode = 'k'").unwrap_err();
        assert!(e.contains("__gc and __mode"), "{e}");
        s.run("assert(getmetatable(w).__mode == nil)").unwrap();
    }

    /// A failed chunk takes back what it did to a brush and the clock too.
    #[test]
    #[cfg(tube_box)]
    fn a_failed_chunk_leaves_brush_and_clock_alone() {
        let mut s = Session::new(W).unwrap();
        s.run(CHUNKS[0]).unwrap();
        s.run(r#"b = brush("round", 4); b:load(pile{{"bone black", 1}}, 0.9); full0 = b:fullness()"#).unwrap();
        let clock = s.st.borrow().clock;
        let before = bits(&s);
        let e = s.run("b:stroke({100, 300, 400, 320}); wait(600); error('stop')").unwrap_err();
        assert!(e.contains("stop"), "{e}");
        assert_eq!(before, bits(&s));
        assert_eq!(clock, s.st.borrow().clock);
        s.run("assert(b:fullness() == full0)").unwrap();
    }

    /// The order `pairs` and `next` walk a table keyed by tables, closures
    /// and userdata in, as printed by a fresh session.
    #[cfg(tube_box)]
    fn object_key_order() -> String {
        let mut s = Session::new(W).unwrap();
        s.run(&CANVAS.replace("aspect=1.5", "aspect=1")).unwrap();
        s.run("items = {}; for i = 1, 64 do items[{index = i}] = i end").unwrap();
        // garbage and failed chunks in between must not matter
        let _ = s.run("for i = 1, 500 do local _ = {i} end; items[{index = 0}] = 0; error('x')");
        s.run(r#"fs = {}; for i = 1, 16 do fs[function() return i end] = i end
                  ms = {}; for i = 1, 8 do ms[mask(function() return 1 end)] = i end
                  mixed = {10, 20, x = 1, y = 2, [2.5] = 3, [true] = 4}; for i = 1, 20 do mixed[{i}] = -i end"#).unwrap();
        s.run(r#"local o = {}
                  for k, v in pairs(items) do o[#o + 1] = v end
                  for k, v in pairs(fs) do o[#o + 1] = v end
                  for k, v in pairs(ms) do o[#o + 1] = v end
                  for k, v in pairs(mixed) do o[#o + 1] = tostring(v) end
                  local k, v = next(items); o[#o + 1] = 'first ' .. v
                  k, v = next(items, k); o[#o + 1] = 'second ' .. v
                  local n = 0; for k in next, items do n = n + 1 end; o[#o + 1] = 'n ' .. n
                  print(table.concat(o, ','))"#).unwrap();
        let out = s.st.borrow().out.clone();
        out
    }

    // tables keyed by objects walk in creation order, not address order
    #[test]
    #[cfg(tube_box)]
    fn object_keys_walk_in_creation_order() {
        let a = object_key_order();
        // a different heap layout in the same process
        let _pad: Vec<Session> = (0..3).map(|_| Session::new(W).unwrap()).collect();
        let b = object_key_order();
        assert_eq!(a, b);
        let first: String = (1..=64).map(|i| format!("{i},")).collect();
        assert!(a.starts_with(&first), "object keys in creation order: {a}");
        assert!(a.contains("first 1,second 2,n 64"), "{a}");
        // library functions have no serial: they go by name, after objects
        let mut s = Session::replay(W).unwrap();
        s.run("local t = {[math.sin] = 1, [math.cos] = 2, [string.rep] = 3, [{}] = 4, x = 5}; local o = {}; for k, v in pairs(t) do o[#o + 1] = v end; print(table.concat(o, ','))").unwrap();
        assert_eq!(s.st.borrow().out.trim_end(), "5,4,2,1,3");
    }

    #[test]
    #[cfg(tube_box)]
    fn plain_tables_keep_lua_order() {
        // tables without object keys walk in stock Lua's order (same seed),
        // so existing paintings replay as before
        let build = "t = {}; for i = 1, 40 do t['k' .. i * 7919] = i end; for i = 1, 10 do t[i * 0.5] = -i end; t[true] = 0";
        let (lua, state, _serials) = fixed_lua(StdLib::TABLE).unwrap();
        let want: String = lua.load(format!("{build}; local o = {{}}; for k, v in next, t do o[#o + 1] = v end; return table.concat(o, ',')")).eval().unwrap();
        drop(lua);
        unsafe { mlua::ffi::lua_close(state) };
        let mut s = Session::replay(W).unwrap();
        s.run(build).unwrap();
        s.run("local o = {}; for k, v in pairs(t) do o[#o + 1] = v end; print(table.concat(o, ','))").unwrap();
        assert_eq!(s.st.borrow().out.trim_end(), want);
        s.run("local o = {}; for k, v in next, t do o[#o + 1] = v end; print(table.concat(o, ','))").unwrap();
        assert_eq!(s.st.borrow().out.trim_end(), want);
        // __pairs is honored
        s.run("local p = setmetatable({}, {__pairs = function(t) return function(_, k) if not k then return 1, 'one' end end, t, nil end}); for k, v in pairs(p) do assert(k == 1 and v == 'one') end").unwrap();
    }

    // a gmatch iterator kept across chunks does not advance during a
    // failed chunk
    #[test]
    #[cfg(tube_box)]
    fn gmatch_iterators_roll_back() {
        let mut s = Session::new(W).unwrap();
        s.run(CANVAS).unwrap();
        s.run(r#"it = string.gmatch("red green blue", "%a+")"#).unwrap();
        let e = s.run(r#"assert(it() == "red"); error("stop")"#).unwrap_err();
        assert!(e.contains("stop"), "{e}");
        s.run(r#"assert(it() == "red")"#).unwrap();
        s.run(r#"assert(it() == "green")"#).unwrap();
        let _ = s.run("it(); error('stop')").unwrap_err();
        s.run(r#"w = it(); assert(w == "blue", w)"#).unwrap();
        // a method-call iterator and a word-pair iterator too
        s.run(r#"it2 = ("a=1, b=2"):gmatch("(%w+)=(%w+)")"#).unwrap();
        let _ = s.run("it2(); error('stop')").unwrap_err();
        s.run(r#"local k, v = it2(); assert(k == "a" and v == "1")"#).unwrap();
    }

    // the rollback-aware gmatch matches Lua's own, match for match
    #[test]
    #[cfg(tube_box)]
    fn gmatch_matches_lua() {
        let cases: &[(&str, &str, Option<i64>)] = &[
            ("red green blue", "%a+", None),
            ("hello world from Lua", "%a+", None),
            ("key=val, k2=v2", "(%w+)=(%w+)", None),
            ("abc", "", None),
            ("abc", "x*", None),
            ("a,b,,c,", "([^,]*)", None),
            ("one two three", "()%a+()", None),
            ("^a^b", "^%a", None),
            ("aaa", "a-", None),
            ("abcabc", "b", Some(3)),
            ("abcabc", "%a", Some(-2)),
            ("abc", "", Some(10)),
            ("abc", "%a", Some(0)),
            ("THE (quick) fox", "%((%a+)%)", None),
            ("x = 1.5e3", "%d+%.?%d*", None),
        ];
        let lua = mlua::Lua::new();
        let mut s = Session::replay(W).unwrap();
        let prog = |s: &str, p: &str, i: Option<i64>| {
            let init = i.map(|i| format!(", {i}")).unwrap_or_default();
            format!("local o = {{}}; for a, b in string.gmatch({s:?}, {p:?}{init}) do o[#o + 1] = tostring(a) .. '|' .. tostring(b) end; return table.concat(o, ' ')")
        };
        for (str_, pat, init) in cases {
            let want: String = lua.load(prog(str_, pat, *init)).eval().unwrap();
            s.run(&format!("print((function() {} end)())", prog(str_, pat, *init))).unwrap();
            let got = s.st.borrow().out.trim_end_matches('\n').to_string();
            assert_eq!(got, want, "gmatch({str_:?}, {pat:?}, {init:?})");
        }
        // errors are Lua's too
        let e = s.run("for _ in string.gmatch('abc', '%') do end").unwrap_err();
        assert!(e.contains("malformed pattern"), "{e}");
    }

    /// A painting from the default box names no box in its log: its log is
    /// what round 19 wrote, and the engine it is painted with.
    #[test]
    #[cfg(tube_box)]
    fn a_default_log_names_no_box() {
        let mut a = Session::new(W).unwrap();
        a.run(CANVAS).unwrap();
        let prog = a.program("t");
        let want = format!("-- easel session \"t\": a painting replayed chunk by chunk.\n-- Each \"--@ chunk\" line starts one chunk as it was run at the easel.\n--@ engine 2\n\n--@ chunk 1\n{CANVAS}\n");
        assert_eq!(prog, want);
        assert_eq!(logged_box(&prog).unwrap(), None);
        assert_eq!(box_for(Some(&prog)).map(|b| b.name), Ok(paint::palette::DEFAULT_BOX), "(EASEL_BOX set in the test's environment?)");
    }

    /// Only the log's head names its box: a `--@ box` line inside a chunk is
    /// the chunk's; two in the head, or one without a name, are errors.
    #[test]
    fn the_box_line_is_read_from_the_head_only() {
        let head = "-- easel session \"t\": a painting replayed chunk by chunk.\n-- Each \"--@ chunk\" line starts one chunk as it was run at the easel.\n";
        assert_eq!(logged_box(&format!("{head}--@ box inness\n\n--@ chunk 1\nx = 1\n")).unwrap().as_deref(), Some("inness"));
        assert_eq!(logged_box(&format!("{head}\n--@ chunk 1\n--@ box inness\nx = 1\n")).unwrap(), None);
        assert!(logged_box(&format!("{head}--@ box inness\n--@ box sargent\n--@ chunk 1\n")).unwrap_err().contains("twice"));
        assert!(logged_box(&format!("{head}--@ box\n--@ chunk 1\n")).unwrap_err().contains("names no box"));
        assert!(logged_box(&format!("{head}--@ boxes x\n--@ chunk 1\n")).unwrap_err().contains("names no box"));
        // the boxes of this build: the default box, or a painter's build's own
        let own = paint::palette::default_box().unwrap_or_else(|| Palette::box_names()[0]);
        assert!(find_box("no such box").unwrap_err().contains(&format!("its boxes: {own:?}")));
    }

    /// A painting from another box names it in its log; the log replays with
    /// that box to the same canvas, bit for bit, and the default box can't
    /// paint it.
    #[cfg(tube_box)]
    #[cfg(feature = "box-sargent")]
    #[test]
    fn a_box_is_named_in_the_log_and_replays_with_it() {
        let chunks = [CANVAS, r#"b = brush("round", 4); b:load(pile{{"rose madder", 1}, {"ultramarine blue", 2}, {"zinc white", 1}}, 0.9)
           for i = 1, 5 do b:stroke({{100 + i*60, 500}, {130 + i*60, 420}}) end
           work(ellipse(500, 300, 200, 100), {hand="glaze", pile=pile{{"viridian", 1}, medium=0.6}})"#];
        let mut a = Session::with_box(W, find_box("sargent").unwrap()).unwrap();
        for c in chunks {
            a.run(c).unwrap();
        }
        let prog = a.program("t");
        assert_eq!(prog.lines().nth(2), Some("--@ box sargent"), "{prog}");
        assert_eq!(logged_box(&prog).unwrap().as_deref(), Some("sargent"));
        let mut b = Session::replay_with(W, box_for(Some(&prog)).expect("(EASEL_BOX set in the test's environment?)")).unwrap();
        for c in parse_program(&prog) {
            b.run(&c).unwrap();
        }
        assert_eq!(bits(&a), bits(&b));
        assert_eq!(b.program("t"), prog);
        let mut d = Session::replay(W).unwrap();
        d.run(CANVAS).unwrap();
        let e = d.run(chunks[1]).unwrap_err();
        assert!(e.contains("no tube \"rose madder\""), "{e}");
    }

    /// The guide's tube table is the default box's.
    #[test]
    #[cfg(tube_box)]
    fn the_guide_shows_the_default_box() {
        let guide = include_str!("../../../notes/easel_guide.md");
        let table: String = guide.lines().skip_while(|l| !l.starts_with("| tube | pigment |")).take_while(|l| l.starts_with('|')).map(|l| format!("{l}\n")).collect();
        assert_eq!(table, Palette::tube_box().table());
    }

    // a view's form counts visible bodies only, not proxies
    #[test]
    #[cfg(tube_box)]
    fn view_form_counts_visible_parts_only() {
        let mut s = Session::replay(W).unwrap();
        s.run(CANVAS).unwrap();
        s.run(r#"local w = world{horizon=300}
                  local s = w:spot_at(0, 10)
                  w = w:proxy(s, body.ellipsoid(s:p(0, 1, 0), s:size(1, 1, 1)))
                  local v = w:view()
                  assert(v.form.parts == 0, 'proxy-only: ' .. v.form.parts)
                  assert(v:part(1) == 0)"#).unwrap();
        s.run(r#"local w = world{horizon=300}
                  local a, b = w:spot_at(-3, 12), w:spot_at(3, 12)
                  w = w:place(a, body.ellipsoid(a:p(0, 1, 0), a:size(1, 1, 1)))
                  w = w:proxy(a, body.ellipsoid(a:p(0, 5, 0), a:size(1, 1, 1)))
                  w = w:place(b, body.ellipsoid(b:p(0, 1, 0), b:size(1, 1, 1)))
                  local v = w:view()
                  assert(v.form.parts == 2, 'mixed: ' .. v.form.parts)
                  assert(v:part(1) == 1 and v:part(2) == 0 and v:part(3) == 2)"#).unwrap();
    }

    /// A new painting's log names the engine it is painted with; a log
    /// without the line was painted with engine 1, and replays and goes on
    /// as engine 1 (its log stays without the line).
    #[test]
    #[cfg(tube_box)]
    fn the_log_names_its_engine() {
        let mut a = Session::new(W).unwrap();
        a.run(CANVAS).unwrap();
        let line = format!("{ENGINE_MARK} {}\n", paint::ENGINE);
        let prog = a.program("t");
        assert!(prog.contains(&format!("{line}\n{MARK} 1\n")), "{prog}");
        assert_eq!(logged_engine(&prog), Ok(paint::ENGINE));
        let old = prog.replace(&line, "");
        let tubes = box_for(Some(&old)).expect("(EASEL_BOX set in the test's environment?)");
        let mut b = Session::with_box(W, tubes).unwrap();
        for c in parse_program(&old) {
            b.run(&c).unwrap();
        }
        assert_eq!(b.canvas().unwrap().engine(), 1);
        assert_eq!(b.program("t"), old);
        for bad in ["--@ engine 0", "--@ engine 99", "--@ engine two", "--@ engine 1\n--@ engine 1"] {
            assert!(logged_engine(&format!("{bad}\n{MARK} 1\n")).is_err(), "{bad}");
        }
    }

    /// Paintings from before the engine version was recorded (no `--@ engine`
    /// line) replay as engine 1, bit for bit as they did (recorded at 08f6324,
    /// before engine 2, at a small width to keep the test short): two
    /// studios' paintings (paint-studio-6399ad and -db6324), hand-timed, with
    /// paint worked over drying paint.
    #[test]
    #[cfg(feature = "replay")]
    fn logs_without_an_engine_line_replay_as_before() {
        for (name, want) in [("studio_6399ad", "4f3abae7eb080221"), ("studio_db6324", "009ec933d082a252")] {
            let text = std::fs::read_to_string(format!("{}/tests/engine1/{name}.lua", env!("CARGO_MANIFEST_DIR"))).unwrap();
            let mut s = Session::replay_with(320, box_for(Some(&text)).expect("(EASEL_BOX set in the test's environment?)")).unwrap();
            for (i, c) in parse_program(&text).iter().enumerate() {
                s.run(c).unwrap_or_else(|e| panic!("{name} chunk {}: {e}", i + 1));
            }
            let c = s.canvas().unwrap();
            let mut h = 0xcbf2_9ce4_8422_2325u64;
            for b in c.seen().iter().flat_map(|p| p.map(f32::to_bits)).chain(c.kept_surface_um().2.iter().map(|v| v.to_bits())) {
                h ^= b as u64;
                h = h.wrapping_mul(0x100_0000_01b3);
            }
            assert_eq!(format!("{h:016x}"), want, "{name} no longer replays as it did");
        }
    }
}
