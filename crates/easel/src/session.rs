//! A live painting: a Lua state over a canvas, the chunks run so far (the
//! log, which is also the replayable program) and snapshots for undo.

use crate::api::{self, Studio};
use mlua::{Lua, LuaOptions, StdLib, Value};
use paint::{Canvas, Held, Style};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
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
    globals: Vec<(Value, Value)>,
    brushes: Vec<(Rc<RefCell<Held>>, Held)>,
}

pub struct Session {
    pub lua: Lua,
    pub st: Rc<RefCell<Studio>>,
    pub log: Vec<Chunk>,
    snaps: VecDeque<Snap>,
    pub undo_depth: usize,
}

pub struct Ran {
    pub out: String,
    pub secs: f64,
    pub field_secs: f64,
}

impl Session {
    pub fn new(width: usize, undo_depth: usize) -> mlua::Result<Self> {
        let libs = StdLib::TABLE | StdLib::STRING | StdLib::MATH | StdLib::BIT | StdLib::JIT;
        let lua = Lua::new_with(libs, LuaOptions::default())?;
        // no file or OS access for paintings
        for k in ["dofile", "loadfile", "require", "collectgarbage"] {
            lua.globals().raw_set(k, Value::Nil)?;
        }
        let st = Rc::new(RefCell::new(Studio::new(width)));
        api::install(&lua, st.clone())?;
        Ok(Session { lua, st, log: Vec::new(), snaps: VecDeque::new(), undo_depth, })
    }

    fn snap(&self) -> mlua::Result<Snap> {
        let mut s = self.st.borrow_mut();
        let globals = self.lua.globals().pairs::<Value, Value>().collect::<mlua::Result<Vec<_>>>()?;
        let brushes = s.live_brushes().into_iter().map(|b| {
            let h = b.borrow().clone();
            (b, h)
        }).collect();
        Ok(Snap { canvas: s.canvas.clone(), style: s.style.clone(), setup: s.setup.clone(), seed: s.seed, clock: s.clock, globals, brushes })
    }

    fn restore(&self, snap: Snap) -> mlua::Result<()> {
        let g = self.lua.globals();
        let now = g.clone().pairs::<Value, Value>().map(|kv| kv.map(|(k, _)| k)).collect::<mlua::Result<Vec<_>>>()?;
        for k in now {
            g.raw_set(k, Value::Nil)?;
        }
        for (k, v) in snap.globals {
            g.raw_set(k, v)?;
        }
        for (b, h) in snap.brushes {
            *b.borrow_mut() = h;
        }
        let mut s = self.st.borrow_mut();
        s.canvas = snap.canvas;
        s.style = snap.style;
        s.setup = snap.setup;
        s.seed = snap.seed;
        s.clock = snap.clock;
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
        let snap = self.snap().map_err(|e| e.to_string())?;
        let n = self.log.len() as u64 + 1;
        let clock = snap.clock;
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
        if let Some(e) = fail {
            self.restore(snap).map_err(|e| e.to_string())?;
            let mut msg = out;
            msg.push_str(&e);
            return Err(msg);
        }
        self.snaps.push_back(snap);
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
        let secs: f64 = self.log.iter().map(|c| c.secs).sum();
        format!(
            "{} chunks · {}px · {} · clock {} min · {} · undo {} deep · painted {secs:.0}s",
            self.log.len(),
            s.width,
            s.setup.as_deref().unwrap_or("no canvas yet"),
            s.clock,
            if wet { "wet paint on the canvas" } else { "dry" },
            self.snaps.len()
        )
    }
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
