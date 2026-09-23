//! A live painting: a Lua state over a canvas, the chunks run so far (the
//! log, which is also the replayable program) and snapshots for undo.

use crate::api::{self, Studio};
use mlua::{Function, Lua, StdLib, Table, Value};
use paint::{Canvas, Held, Style};
use std::alloc::Layout;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, VecDeque};
use std::ffi::c_void;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::mem::ManuallyDrop;
use std::rc::Rc;
use std::time::Instant;

/// Marks the start of a chunk in a session file.
pub const MARK: &str = "--@ chunk";

pub struct Chunk {
    pub src: String,
    /// Painting clock (minutes) when the chunk started.
    pub clock: f64,
    pub secs: f64,
}

/// Everything a failed or undone chunk could have changed.
struct Snap {
    canvas: Option<Canvas>,
    style: Option<Rc<Style>>,
    setup: Option<String>,
    seed: u64,
    clock: f64,
    clock0: f64,
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
    snaps: VecDeque<Snap>,
    pub undo_depth: usize,
    /// A disposable replay (`easel run`, `check`): no snapshots, since a
    /// failure ends it. A live session always snapshots before a chunk,
    /// even at undo depth 0, so a failure can roll back.
    replay: bool,
    /// heap.lua's snap and restore, prelude.lua's per-chunk reset and the
    /// private objects snapshots skip (dropped before the state is closed).
    heap: Option<(Function, Function)>,
    prelude: Option<(Function, Table)>,
    /// Creation serials of the state's objects (outlives the state).
    _serials: Box<Serials>,
    /// Time spent snapshotting and restoring the Lua heap (s), for status.
    pub heap_secs: (f64, f64),
}

#[derive(Debug)]
pub struct Ran {
    pub out: String,
    pub secs: f64,
    pub field_secs: f64,
}

impl Session {
    pub fn new(width: usize, undo_depth: usize) -> mlua::Result<Self> {
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
        let prelude: (Function, Table) = lua.load(include_str!("prelude.lua")).set_name("prelude.lua").call((id, getmt))?;
        let st = Rc::new(RefCell::new(Studio::new(width)));
        api::install(&lua, st.clone())?;
        Ok(Session { lua: ManuallyDrop::new(lua), state, st, log: Vec::new(), snaps: VecDeque::new(), undo_depth, replay: false, heap: Some((snap_f, restore_f)), prelude: Some(prelude), _serials: serials, heap_secs: (0.0, 0.0) })
    }

    /// A session that replays a program: a failed chunk ends it, so it
    /// keeps no snapshots.
    pub fn replay(width: usize) -> mlua::Result<Self> {
        let mut s = Self::new(width, 0)?;
        s.replay = true;
        Ok(s)
    }

    fn snap(&mut self) -> mlua::Result<Snap> {
        let t0 = Instant::now();
        let (snap_f, _) = self.heap.as_ref().unwrap();
        let strings = self.lua.load("return getmetatable('')").eval::<Value>().ok();
        let skip = self.prelude.as_ref().unwrap().1.clone();
        let heap: Table = snap_f.call((skip, self.lua.globals(), strings))?;
        self.heap_secs.0 += t0.elapsed().as_secs_f64();
        let mut s = self.st.borrow_mut();
        let brushes = s.live_brushes().into_iter().map(|b| {
            let h = b.borrow().clone();
            (b, h)
        }).collect();
        Ok(Snap { canvas: s.canvas.clone(), style: s.style.clone(), setup: s.setup.clone(), seed: s.seed, clock: s.clock, clock0: s.clock0, heap, brushes })
    }

    fn restore(&mut self, snap: Snap) -> mlua::Result<()> {
        let t0 = Instant::now();
        let (_, restore_f) = self.heap.as_ref().unwrap();
        restore_f.call::<()>(snap.heap)?;
        self.heap_secs.1 += t0.elapsed().as_secs_f64();
        for (b, h) in snap.brushes {
            *b.borrow_mut() = h;
        }
        let mut s = self.st.borrow_mut();
        s.canvas = snap.canvas;
        s.style = snap.style;
        s.setup = snap.setup;
        s.seed = snap.seed;
        s.clock = snap.clock;
        s.clock0 = snap.clock0;
        Ok(())
    }

    /// Run one chunk. On error nothing it did survives (canvas, globals,
    /// brushes, clock) and it is not logged.
    pub fn run(&mut self, src: &str) -> Result<Ran, String> {
        let src = src.trim_end().to_string();
        if src.lines().any(|l| l.trim_start().starts_with(MARK)) {
            return Err(format!("a chunk can't contain a line starting with {MARK:?} (it separates chunks in the log)"));
        }
        if src.trim().is_empty() {
            return Err("empty chunk".into());
        }
        // a live session snapshots even at undo depth 0 (to roll back a
        // failure); a replay needs none, since a failure ends it
        let snap = if !self.replay { Some(self.snap().map_err(|e| e.to_string())?) } else { None };
        let n = self.log.len() as u64 + 1;
        let clock = self.st.borrow().clock;
        self.prelude.as_ref().unwrap().0.call::<()>(()).map_err(|e| e.to_string())?;
        self.st.borrow_mut().begin(n);
        let t0 = Instant::now();
        let chunk = self.lua.load(src.as_str()).set_name(format!("chunk {n}"));
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| chunk.exec()));
        let secs = t0.elapsed().as_secs_f64();
        let (out, field_secs) = {
            let s = self.st.borrow();
            (s.out.clone(), s.field_secs)
        };
        let fail = match r {
            Ok(Ok(())) => None,
            Ok(Err(e)) => Some(clean_error(&e.to_string())),
            Err(p) => Some(format!("engine panic: {}", p.downcast_ref::<String>().cloned().or(p.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default())),
        };
        // masks and brushes hold memory Lua can't see: collect between chunks
        let _ = self.lua.gc_collect();
        if let Some(e) = fail {
            if let Some(snap) = snap {
                self.restore(snap).map_err(|e| e.to_string())?;
            }
            let mut msg = out;
            msg.push_str(&e);
            return Err(msg);
        }
        if let Some(snap) = snap.filter(|_| self.undo_depth > 0) {
            self.snaps.push_back(snap);
        }
        while self.snaps.len() > self.undo_depth {
            self.snaps.pop_front();
        }
        self.log.push(Chunk { src, clock, secs });
        Ok(Ran { out, secs, field_secs })
    }

    /// Take back the last `n` chunks.
    pub fn undo(&mut self, n: usize) -> Result<(), String> {
        if n == 0 {
            return Ok(());
        }
        if n > self.log.len() {
            return Err(format!("only {} chunks to undo", self.log.len()));
        }
        if n > self.snaps.len() {
            return Err(format!("can undo at most {} chunks (snapshots kept: open with --undo N for more)", self.snaps.len()));
        }
        let mut snap = None;
        for _ in 0..n {
            snap = self.snaps.pop_back();
            self.log.pop();
        }
        self.restore(snap.unwrap()).map_err(|e| e.to_string())
    }

    pub fn canvas(&self) -> Option<std::cell::Ref<'_, Canvas>> {
        std::cell::Ref::filter_map(self.st.borrow(), |s| s.canvas.as_ref()).ok()
    }

    /// The session as a replayable program.
    pub fn program(&self, name: &str) -> String {
        let mut s = String::new();
        let _ = writeln!(s, "-- easel session {name:?}: a painting replayed chunk by chunk.");
        let _ = writeln!(s, "--   easel run paintings/lua/{name}.lua [--width 3200]");
        let _ = writeln!(s, "-- Each {MARK:?} line starts one chunk as it was run at the easel (clock = painting minutes).");
        for (i, c) in self.log.iter().enumerate() {
            let _ = writeln!(s, "\n{MARK} {} · clock {}", i + 1, c.clock);
            s.push_str(&c.src);
            s.push('\n');
        }
        s
    }

    pub fn status(&self) -> String {
        let s = self.st.borrow();
        let wet = s.canvas.as_ref().map(|c| c.wet_total() > 1e-6).unwrap_or(false);
        let secs: f64 = self.log.iter().map(|c| c.secs).sum::<f64>() + 0.0;
        format!(
            "{} chunks · {}px · {} · clock {} min · {} · undo {} deep · painted {secs:.0}s · rollback bookkeeping {:.2}s",
            self.log.len(),
            s.width,
            s.setup.as_deref().unwrap_or("no canvas yet"),
            s.clock,
            if wet { "wet paint on the canvas" } else { "dry" },
            self.snaps.len(),
            self.heap_secs.0 + self.heap_secs.1
        )
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        // everything holding references into the state goes first
        self.snaps.clear();
        self.heap = None;
        self.prelude = None;
        let _ = self.lua.gc_collect();
        unsafe {
            ManuallyDrop::drop(&mut self.lua);
            mlua::ffi::lua_close(self.state);
        }
    }
}

/// Creation serials of a state's tables, closures, userdata and threads,
/// kept by its allocator. Lua hashes these objects by address, which
/// differs between processes; their serials give prelude.lua's `pairs` and
/// `next` an order that doesn't: the order the program created them in.
/// (Relative order is all that counts, so objects made by failed chunks,
/// snapshots or mlua itself do no harm.)
pub struct Serials {
    next: Cell<i64>,
    /// block address -> (serial, size)
    live: RefCell<BTreeMap<usize, (i64, usize)>>,
}

const ALIGN: usize = 16;

unsafe extern "C" fn counting_alloc(ud: *mut c_void, ptr: *mut c_void, osize: usize, nsize: usize) -> *mut c_void {
    use mlua::ffi::{LUA_TFUNCTION, LUA_TTABLE, LUA_TTHREAD, LUA_TUSERDATA};
    // SAFETY: `ud` is the session's boxed Serials, which outlives the state;
    // Lua passes the block's true size as `osize` whenever `ptr` is a block.
    unsafe {
        let h = &*(ud as *const Serials);
        if nsize == 0 {
            if !ptr.is_null() {
                h.live.borrow_mut().remove(&(ptr as usize));
                std::alloc::dealloc(ptr as *mut u8, Layout::from_size_align_unchecked(osize, ALIGN));
            }
            return std::ptr::null_mut();
        }
        if ptr.is_null() {
            let p = std::alloc::alloc(Layout::from_size_align_unchecked(nsize, ALIGN));
            // a new object: `osize` is its type
            if !p.is_null() && matches!(osize as i32, LUA_TTABLE | LUA_TFUNCTION | LUA_TUSERDATA | LUA_TTHREAD) {
                let n = h.next.get();
                h.next.set(n + 1);
                h.live.borrow_mut().insert(p as usize, (n, nsize));
            }
            return p as *mut c_void;
        }
        // objects are never reallocated, only arrays and buffers
        std::alloc::realloc(ptr as *mut u8, Layout::from_size_align_unchecked(osize, ALIGN), nsize) as *mut c_void
    }
}

impl Serials {
    /// `id(v)`: the serial of an object (the block holding a userdata's
    /// memory), nil for values and library C functions.
    fn id_fn(&self, lua: &Lua) -> mlua::Result<Function> {
        let me = self as *const Serials;
        lua.create_function(move |_, v: Value| {
            let p = v.to_pointer() as usize;
            if p == 0 {
                return Ok(None);
            }
            // SAFETY: the Serials outlive the state, so this function
            let live = unsafe { &*me }.live.borrow();
            Ok(live.range(..=p).next_back().filter(|(a, (_, len))| p < *a + *len).map(|(_, (n, _))| *n))
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
    let serials = Box::new(Serials { next: Cell::new(1), live: RefCell::new(BTreeMap::new()) });
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

/// Lua errors carry a traceback; the painter needs the first lines.
fn clean_error(e: &str) -> String {
    let mut out = Vec::new();
    for l in e.lines() {
        if l.trim_start().starts_with("stack traceback") {
            break;
        }
        out.push(l);
    }
    out.join("\n")
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

pub fn root() -> PathBuf {
    let r = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    r.canonicalize().unwrap_or(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    const W: usize = 160;

    fn bits(s: &Session) -> Vec<u32> {
        s.canvas().unwrap().seen().iter().flat_map(|p| p.map(f32::to_bits)).collect()
    }

    const CHUNKS: [&str; 4] = [
        r##"canvas{style="friedrich", aspect=1.5, seed=2}"##,
        r##"sky = above(function(x) return 300 + 20*math.sin(x/80) end)
           work(sky, {hand="broad", color=function(x, y) return mix("#6f84a8", "#e0d4b0", y/300) end, angle=0, coverage=2})"##,
        r##"b = brush("round", 4); b:load("#303830", 0.9)
           for i = 1, 5 do b:stroke({{100 + i*60, 500}, {130 + i*60 + rand(-10, 10), 420}}) end"##,
        r##"wait(90); stipple(below(function(x) return 380 end), {width=3, color="#c8c6bc", coverage=1.5})"##,
    ];

    #[test]
    fn replay_is_exact_and_failures_roll_back() {
        let mut a = Session::new(W, 4).unwrap();
        for (i, c) in CHUNKS.iter().enumerate() {
            a.run(c).unwrap();
            if i == 1 {
                // a failing chunk that painted and set globals first changes nothing
                let before = bits(&a);
                let e = a.run(r##"junk = 1; b0 = brush("flat", 6); b0:load("#ff0000"); b0:stroke({0, 0, 900, 600}); work(everywhere(), {colour="#fff"})"##).unwrap_err();
                assert!(e.contains("unknown option \"colour\""), "{e}");
                assert_eq!(before, bits(&a));
                assert!(a.run("assert(junk == nil and b0 == nil)").is_ok());
                a.undo(1).unwrap();
            }
        }
        assert_eq!(a.log.len(), CHUNKS.len());
        assert_eq!(a.st.borrow().clock, 90.0, "painting minutes since canvas{{}}");
        // the log replays to the same canvas, bit for bit
        let prog = a.program("t");
        let chunks = parse_program(&prog);
        assert_eq!(chunks, CHUNKS.iter().map(|c| c.trim_end().to_string()).collect::<Vec<_>>());
        let mut b = Session::replay(W).unwrap();
        for c in &chunks {
            b.run(c).unwrap();
        }
        assert_eq!(bits(&a), bits(&b));
    }

    #[test]
    fn rollback_restores_tables_and_upvalues_exactly() {
        let mut s = Session::new(W, 4).unwrap();
        s.run(CHUNKS[0]).unwrap();
        s.run(r##"trees = {1, 2, {x = 3}}; local n = 0; function bump() n = n + 1; return n end; setmetatable(trees, {tag = "a"})"##).unwrap();
        // a failing chunk that edits old tables, an upvalue and a metatable
        let e = s.run(r##"trees[1] = 99; trees[3].x = nil; trees[4] = {}; bump(); getmetatable(trees).tag = "b"; string.custom = 1; error("stop")"##).unwrap_err();
        assert!(e.contains("stop"), "{e}");
        s.run(r##"assert(trees[1] == 1 and trees[3].x == 3 and trees[4] == nil); assert(bump() == 1); assert(getmetatable(trees).tag == "a"); assert(string.custom == nil)"##).unwrap();
        // undo takes the same care
        s.run(r##"trees[2] = "changed"; bump()"##).unwrap();
        s.undo(1).unwrap();
        s.run(r##"assert(trees[2] == 2); assert(bump() == 2)"##).unwrap();
        // and the live session still replays exactly
        let mut b = Session::replay(W).unwrap();
        for c in &s.log {
            b.run(&c.src).unwrap();
        }
        assert_eq!(bits(&s), bits(&b));
    }

    #[test]
    fn undo_restores_canvas_and_brush() {
        let mut s = Session::new(W, 4).unwrap();
        s.run(CHUNKS[0]).unwrap();
        s.run(r##"b = brush("round", 4); b:load("#303830", 0.9)"##).unwrap();
        // every verb is where the guide says (no module shadows another)
        s.run(r##"for _, v in ipairs{"relief", "varnish", "cracks", "terrain", "ridge", "form", "wait", "dry", "drying", "work", "stipple", "glaze", "blend", "tree"} do assert(type(_G[v]) == "function", v) end"##).unwrap();
        s.run(r##"assert(wait(30) == 30 and clock() == 30)"##).unwrap();
        s.undo(2).unwrap();
        let before = bits(&s);
        s.run("full0 = b:fullness(); b:stroke({100, 300, 400, 320}); assert(b:fullness() < full0)").unwrap();
        assert_ne!(before, bits(&s));
        s.undo(1).unwrap();
        assert_eq!(before, bits(&s));
        s.run("assert(full0 == nil); print(b:fullness())").unwrap();
        let full: f32 = s.st.borrow().out.trim().parse().unwrap();
        assert!(full > 0.5, "the brush got its paint back: {full}");
        assert!(s.undo(5).is_err());
    }

    // review 3, finding 1: `--undo 0` kept no pre-chunk snapshot, so a
    // failed chunk's paint and globals survived
    #[test]
    fn live_session_without_undo_still_rolls_back() {
        let mut s = Session::new(W, 0).unwrap();
        s.run("canvas{aspect=1.5, seed=2}; a = 1; t = {n = 1}").unwrap();
        let before = bits(&s);
        let e = s.run(r##"a = 2; t.n = 2; b = brush("round", 4); wait(30); glaze(everywhere(), {color="#ff0000", coats=0.5}); error("stop")"##).unwrap_err();
        assert!(e.contains("stop"), "{e}");
        assert_eq!(before, bits(&s), "the failed glaze is gone");
        s.run("assert(a == 1 and t.n == 1 and b == nil and clock() == 0)").unwrap();
        assert!(s.undo(1).is_err(), "no undo history at depth 0");
        let mut r = Session::replay(W).unwrap();
        for c in &s.log {
            r.run(&c.src).unwrap();
        }
        assert_eq!(bits(&s), bits(&r));
    }

    /// The order `pairs` and `next` walk a table keyed by tables, closures
    /// and userdata in, as printed by a fresh session.
    fn object_key_order() -> String {
        let mut s = Session::new(W, 2).unwrap();
        s.run("canvas{aspect=1, seed=7}; items = {}; for i = 1, 64 do items[{index = i}] = i end").unwrap();
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

    // review 3, finding 2: tables keyed by objects walked in address order
    #[test]
    fn object_keys_walk_in_creation_order() {
        let a = object_key_order();
        // a different heap layout in the same process
        let _pad: Vec<Session> = (0..3).map(|_| Session::new(W, 0).unwrap()).collect();
        let b = object_key_order();
        assert_eq!(a, b);
        let first: String = (1..=64).map(|i| format!("{i},")).collect();
        assert!(a.starts_with(&first), "object keys in creation order: {a}");
        assert!(a.contains("first 1,second 2,n 64"), "{a}");
    }

    #[test]
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

    // review 3, finding 3: a gmatch iterator kept across chunks advanced
    // during a failed chunk
    #[test]
    fn gmatch_iterators_roll_back() {
        let mut s = Session::new(W, 4).unwrap();
        s.run("canvas{aspect=1, seed=7}").unwrap();
        s.run(r#"it = string.gmatch("red green blue", "%a+")"#).unwrap();
        let e = s.run(r#"assert(it() == "red"); error("stop")"#).unwrap_err();
        assert!(e.contains("stop"), "{e}");
        s.run(r#"assert(it() == "red")"#).unwrap();
        s.run(r#"assert(it() == "green")"#).unwrap();
        s.undo(1).unwrap();
        s.run(r#"w = it(); assert(w == "green", w)"#).unwrap();
        // a method-call iterator and a word-pair iterator too
        s.run(r#"it2 = ("a=1, b=2"):gmatch("(%w+)=(%w+)")"#).unwrap();
        let _ = s.run("it2(); error('stop')").unwrap_err();
        s.run(r#"local k, v = it2(); assert(k == "a" and v == "1")"#).unwrap();
    }

    // the rollback-aware gmatch matches Lua's own, match for match
    #[test]
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

    // review 3, finding 4: a view's form counted proxies as visible parts
    #[test]
    fn view_form_counts_visible_parts_only() {
        let mut s = Session::replay(W).unwrap();
        s.run("canvas{aspect=1.5, seed=2}").unwrap();
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
}
