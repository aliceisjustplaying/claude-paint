use crate::api::S;
use crate::session::Session;
use bincode::Options;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, Instant};

const MAGIC: &[u8; 8] = b"EASLRP01";

#[derive(Serialize, Deserialize)]
struct State {
    seed: u64,
    calls: u64,
    chunk: u64,
    clock: f64,
    clock0: f64,
    rng: u64,
    hand: crate::time::Hand,
    board: crate::board::Board,
    out: String,
    brushes: Vec<paint::Held>,
    knives: Vec<paint::Knife>,
    rags: Vec<paint::rag::Rag>,
}

impl State {
    fn capture(st: &S) -> Self {
        let mut s = st.borrow_mut();
        Self {
            seed: s.seed, calls: s.calls, chunk: s.chunk, clock: s.clock, clock0: s.clock0,
            rng: s.rng.state(), hand: s.hand.clone(), board: s.board.clone(), out: s.out.clone(),
            brushes: s.live_brushes().iter().map(|b| b.borrow().clone()).collect(),
            knives: s.live_knives().iter().map(|b| b.borrow().clone()).collect(),
            rags: s.live_rags().iter().map(|b| b.borrow().clone()).collect(),
        }
    }

    fn apply(self, st: &S) -> Result<(), String> {
        let mut s = st.borrow_mut();
        let brushes = s.live_brushes();
        let knives = s.live_knives();
        let rags = s.live_rags();
        if (brushes.len(), knives.len(), rags.len()) != (self.brushes.len(), self.knives.len(), self.rags.len()) {
            return Err("replay checkpoint: tool registry differs; refusing reconstruction".into());
        }
        for (b, value) in brushes.iter().zip(self.brushes) { *b.borrow_mut() = value; }
        for (b, value) in knives.iter().zip(self.knives) { *b.borrow_mut() = value; }
        for (b, value) in rags.iter().zip(self.rags) { *b.borrow_mut() = value; }
        s.seed = self.seed;
        s.calls = self.calls;
        s.chunk = self.chunk;
        s.clock = self.clock;
        s.clock0 = self.clock0;
        s.rng = paint::Rng::from_state(self.rng);
        s.hand = self.hand;
        s.board = self.board;
        s.out = self.out;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct Event {
    tag: String,
    result_type: String,
    before: Option<[u8; 32]>,
    state: Option<State>,
    outcome: Outcome,
}

#[derive(Serialize, Deserialize)]
enum Outcome { Value(Vec<u8>), RuntimeError(String) }

#[derive(Default)]
struct Events {
    file: Option<RefCell<std::fs::File>>,
    len: usize,
    end: u64,
    cursor: u64,
}

impl Events {
    fn len(&self) -> usize { self.len }

    fn push(&mut self, bytes: Vec<u8>) -> Result<(), String> {
        if self.file.is_none() {
            self.file = Some(RefCell::new(tempfile::tempfile().map_err(|e| e.to_string())?));
        }
        let file = self.file.as_mut().unwrap().get_mut();
        file.seek(SeekFrom::Start(self.end)).map_err(|e| e.to_string())?;
        file.write_all(&(bytes.len() as u64).to_le_bytes()).and_then(|_| file.write_all(&bytes)).map_err(|e| e.to_string())?;
        self.end = file.stream_position().map_err(|e| e.to_string())?;
        self.len += 1;
        Ok(())
    }

    fn next(&mut self) -> Result<Option<Vec<u8>>, String> {
        if self.cursor == self.end { return Ok(None); }
        let file = self.file.as_mut().unwrap().get_mut();
        file.seek(SeekFrom::Start(self.cursor)).map_err(|e| e.to_string())?;
        let bytes = Self::read(file)?;
        self.cursor = file.stream_position().map_err(|e| e.to_string())?;
        Ok(Some(bytes))
    }

    fn read(file: &mut std::fs::File) -> Result<Vec<u8>, String> {
        let mut length = [0; 8];
        file.read_exact(&mut length).map_err(|e| e.to_string())?;
        let length = u64::from_le_bytes(length);
        if length > 4 << 30 { return Err("replay checkpoint: oversized effect event".into()); }
        let mut bytes = vec![0; length as usize];
        file.read_exact(&mut bytes).map_err(|e| e.to_string())?;
        Ok(bytes)
    }

    fn truncate(&mut self, len: usize, end: u64, cursor: u64) -> Result<(), String> {
        if let Some(file) = &self.file {
            file.borrow_mut().set_len(end).map_err(|e| e.to_string())?;
        }
        self.len = len;
        self.end = end;
        self.cursor = cursor;
        Ok(())
    }
}

impl Serialize for Events {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::{Error, SerializeSeq};
        let mut sequence = serializer.serialize_seq(Some(self.len))?;
        if let Some(file) = &self.file {
            let mut file = file.borrow_mut();
            file.seek(SeekFrom::Start(0)).map_err(S::Error::custom)?;
            for _ in 0..self.len {
                let bytes = Self::read(&mut file).map_err(S::Error::custom)?;
                sequence.serialize_element(&bytes)?;
            }
        }
        sequence.end()
    }
}

impl<'de> Deserialize<'de> for Events {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Events;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { f.write_str("replay effect events") }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut sequence: A) -> Result<Events, A::Error> {
                use serde::de::Error;
                let mut events = Events::default();
                while let Some(bytes) = sequence.next_element::<Vec<u8>>()? {
                    events.push(bytes).map_err(A::Error::custom)?;
                }
                Ok(events)
            }
        }
        deserializer.deserialize_seq(Visitor)
    }
}

#[derive(Default)]
struct Trace {
    enabled: bool,
    playback: bool,
    nested: bool,
    events: Events,
    at: usize,
    failure: Option<String>,
    brushes: Vec<Rc<RefCell<paint::Held>>>,
    knives: Vec<Rc<RefCell<paint::Knife>>>,
    rags: Vec<Rc<RefCell<paint::rag::Rag>>>,
}

thread_local! {
    static TRACE: RefCell<Trace> = RefCell::new(Trace::default());
}

fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, String> {
    bincode::serialize(value).map_err(|e| format!("replay checkpoint encoding: {e}"))
}

fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, String> {
    bincode::DefaultOptions::new().with_fixint_encoding().with_limit(4 << 30)
        .reject_trailing_bytes().deserialize(bytes).map_err(|e| format!("replay checkpoint decoding: {e}"))
}

fn failed(message: String) -> mlua::Error {
    TRACE.with(|t| { t.borrow_mut().failure.get_or_insert(message.clone()); });
    mlua::Error::runtime(message)
}

pub fn check() -> mlua::Result<()> {
    TRACE.with(|t| match &t.borrow().failure {
        Some(e) => Err(mlua::Error::runtime(e.clone())),
        None => Ok(()),
    })
}

pub fn chunk_end() {
    TRACE.with(|t| {
        let mut t = t.borrow_mut();
        t.brushes.clear();
        t.knives.clear();
        t.rags.clear();
    });
}

pub fn hold_tools(st: &S) {
    TRACE.with(|t| {
        let mut t = t.borrow_mut();
        if !t.enabled { return; }
        let mut s = st.borrow_mut();
        t.brushes = s.live_brushes();
        t.knives = s.live_knives();
        t.rags = s.live_rags();
    });
}

pub struct Mark {
    events: usize,
    end: u64,
    cursor: u64,
    at: usize,
    failure: Option<String>,
}

pub fn mark() -> Mark {
    TRACE.with(|t| {
        let t = t.borrow();
        Mark { events: t.events.len(), end: t.events.end, cursor: t.events.cursor, at: t.at, failure: t.failure.clone() }
    })
}

pub fn rollback(mark: Mark) {
    TRACE.with(|t| {
        let mut t = t.borrow_mut();
        let failure = t.events.truncate(mark.events, mark.end, mark.cursor).err();
        t.at = mark.at;
        t.failure = failure.or(mark.failure);
    });
}

pub fn effect<R: Serialize + DeserializeOwned>(st: &S, tag: &str, f: impl FnOnce() -> mlua::Result<R>) -> mlua::Result<R> {
    invoke(Some(st), tag, f)
}

pub fn read<R: Serialize + DeserializeOwned>(tag: &str, f: impl FnOnce() -> mlua::Result<R>) -> mlua::Result<R> {
    invoke(None, tag, f)
}

fn invoke<R: Serialize + DeserializeOwned>(st: Option<&S>, tag: &str, f: impl FnOnce() -> mlua::Result<R>) -> mlua::Result<R> {
    let active = TRACE.with(|t| { let t = t.borrow(); t.enabled && !t.nested });
    if !active { return f(); }
    check()?;
    let before = st.map(|s| encode(&State::capture(s)).map(|b| Sha256::digest(b).into())).transpose().map_err(failed)?;
    let event = TRACE.with(|t| {
        let mut t = t.borrow_mut();
        if !t.playback { return Ok(None); }
        let bytes = t.events.next()?.ok_or_else(|| "replay checkpoint: exhausted effect trace".to_string())?;
        let raw = zstd::stream::decode_all(bytes.as_slice()).map_err(|e| format!("replay checkpoint trace: {e}"))?;
        let event: Event = decode(&raw)?;
        t.at += 1;
        Ok(Some(event))
    }).map_err(failed)?;
    if let Some(event) = event {
        if event.tag != tag || event.result_type != std::any::type_name::<R>() || event.before != before {
            return Err(failed(format!("replay checkpoint: effect {} ({tag}) differs; refusing reconstruction", TRACE.with(|t| t.borrow().at))));
        }
        if let (Some(s), Some(state)) = (st, event.state) { state.apply(s).map_err(failed)?; }
        return match event.outcome {
            Outcome::Value(value) => decode(&value).map_err(failed),
            Outcome::RuntimeError(message) => Err(mlua::Error::RuntimeError(message)),
        };
    }
    TRACE.with(|t| t.borrow_mut().nested = true);
    struct Nested;
    impl Drop for Nested { fn drop(&mut self) { TRACE.with(|t| t.borrow_mut().nested = false); } }
    let nested = Nested;
    let value = f();
    drop(nested);
    let outcome = match &value {
        Ok(v) => Outcome::Value(encode(v).map_err(failed)?),
        Err(mlua::Error::RuntimeError(message)) => Outcome::RuntimeError(message.clone()),
        Err(e) => {
            let _ = failed(format!("replay checkpoint: an engine effect failed ({tag}): {e}; refusing an incomplete trace"));
            return value;
        }
    };
    let event = Event {
        tag: tag.to_string(), result_type: std::any::type_name::<R>().to_string(), before,
        state: st.map(State::capture), outcome,
    };
    let bytes = zstd::stream::encode_all(encode(&event).map_err(failed)?.as_slice(), 1).map_err(|e| failed(e.to_string()))?;
    TRACE.with(|t| t.borrow_mut().events.push(bytes)).map_err(failed)?;
    value
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct Identity {
    build: [u8; 32],
    source: [u8; 32],
    width: usize,
    environment: Vec<(Vec<u8>, Vec<u8>)>,
}

impl Identity {
    fn new(source: &str, width: usize) -> Result<Self, String> {
        use std::os::unix::ffi::OsStrExt;
        let executable = std::env::current_exe().and_then(std::fs::read).map_err(|e| e.to_string())?;
        let mut environment: Vec<_> = std::env::vars_os().filter(|(k, _)| k.as_bytes().starts_with(b"PAINT_") || matches!(k.as_bytes(), b"EASEL_BOX" | b"EASEL_WIDTH"))
            .map(|(k, v)| (k.as_bytes().to_vec(), v.as_bytes().to_vec())).collect();
        environment.sort();
        Ok(Self { build: Sha256::digest(executable).into(), source: Sha256::digest(source.as_bytes()).into(), width, environment })
    }
}

#[derive(Serialize, Deserialize)]
struct Saved {
    identity: Identity,
    done: usize,
    events: Events,
    state: State,
    digests: Option<Vec<String>>,
}

pub(crate) fn atomic(path: &Path, f: impl FnOnce(&mut std::fs::File) -> Result<(), String>) -> Result<(), String> {
    let dir = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let mut opened = None;
    for serial in 0..128 {
        let mut part = path.as_os_str().to_os_string();
        part.push(format!(".{}-{serial}.part", std::process::id()));
        let part = PathBuf::from(part);
        match std::fs::OpenOptions::new().create_new(true).write(true).read(true).open(&part) {
            Ok(file) => { opened = Some((part, file)); break; }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(format!("{}: {e}", part.display())),
        }
    }
    let (part, mut file) = opened.ok_or_else(|| format!("{}: no exclusive temporary filename available", path.display()))?;
    let result = (|| {
        f(&mut file)?;
        file.sync_all().map_err(|e| e.to_string())?;
        std::fs::rename(&part, path).map_err(|e| e.to_string())?;
        std::fs::File::open(dir).and_then(|f| f.sync_all()).map_err(|e| e.to_string())
    })();
    if result.is_err() { let _ = std::fs::remove_file(&part); }
    result.map_err(|e| format!("{}: {e}", path.display()))
}

fn write_checkpoint(path: &Path, s: &Session, identity: &Identity, done: usize, digests: &Option<Vec<String>>, elapsed: f64) -> Result<(), String> {
    let started = Instant::now();
    check().map_err(|e| e.to_string())?;
    let state = State::capture(&s.st);
    let mut bytes = 0;
    atomic(path, |f| {
        f.write_all(MAGIC).and_then(|_| f.write_all(&[0; 40])).map_err(|e| e.to_string())?;
        {
            let mut writer = zstd::stream::write::Encoder::new(&mut *f, 1).map_err(|e| e.to_string())?;
            TRACE.with(|t| {
                #[derive(Serialize)]
                struct SavedRef<'a> { identity: &'a Identity, done: usize, events: &'a Events, state: &'a State, digests: &'a Option<Vec<String>> }
                bincode::serialize_into(&mut writer, &SavedRef { identity, done, events: &t.borrow().events, state: &state, digests }).map_err(|e| e.to_string())
            })?;
            bincode::serialize_into(&mut writer, &s.canvas().is_some()).map_err(|e| e.to_string())?;
            if let Some(c) = s.canvas() { c.write_state(&mut writer, "").map_err(|e| e.to_string())?; }
            writer.finish().map_err(|e| e.to_string())?;
        }
        bytes = f.stream_position().map_err(|e| e.to_string())?;
        f.seek(SeekFrom::Start(40)).and_then(|_| f.write_all(&elapsed.to_le_bytes())).map_err(|e| e.to_string())?;
        f.seek(SeekFrom::Start(40)).map_err(|e| e.to_string())?;
        let digest = hash_reader(&mut *f)?;
        f.seek(SeekFrom::Start(8)).and_then(|_| f.write_all(&digest)).map_err(|e| e.to_string())
    })?;
    eprintln!("replay checkpoint {}: {done} chunks, {bytes} bytes, {:.3}s write", path.display(), started.elapsed().as_secs_f64());
    Ok(())
}

fn reconstruct(s: &mut Session, chunks: &[String], saved: &mut Saved, canvas: Option<paint::Canvas>, progress: &dyn Fn(usize, usize)) -> Result<(), String> {
    TRACE.with(|t| *t.borrow_mut() = Trace { enabled: true, playback: true, events: std::mem::take(&mut saved.events), ..Trace::default() });
    for (i, chunk) in chunks.iter().take(saved.done).enumerate() {
        s.run(chunk).map_err(|e| format!("replay checkpoint: prefix chunk {} failed: {e}", i + 1))?;
        progress(i + 1, chunks.len());
    }
    check().map_err(|e| e.to_string())?;
    let complete = TRACE.with(|t| { let t = t.borrow(); t.at == t.events.len() });
    if !complete || encode(&State::capture(&s.st))? != encode(&saved.state)? {
        return Err("replay checkpoint: incomplete or divergent reconstructed state".into());
    }
    if s.canvas().is_some() != canvas.is_some() {
        return Err("replay checkpoint: reconstructed canvas differs".into());
    }
    s.st.borrow_mut().canvas = canvas;
    TRACE.with(|t| t.borrow_mut().playback = false);
    Ok(())
}

fn source_prefix(source: &str, done: usize) -> Result<String, String> {
    let chunks = crate::session::parse_program(source);
    if done == 0 || done > chunks.len() {
        return Err("replay checkpoint: invalid chunk cursor".into());
    }
    let head: String = source.split_inclusive('\n').take_while(|l| !l.trim_start().starts_with(crate::session::MARK)).collect();
    let mut prefix = head.trim_end().to_string();
    prefix.push('\n');
    for (i, chunk) in chunks.iter().take(done).enumerate() {
        prefix.push_str(&format!("\n{} {}\n", crate::session::MARK, i + 1));
        prefix.push_str(chunk);
        prefix.push('\n');
    }
    Ok(prefix)
}

pub struct Live {
    trace: Trace,
    identity: Result<Identity, String>,
    done: usize,
    written: Instant,
}

impl Live {
    pub fn new() -> Self {
        static IDENTITY: std::sync::OnceLock<Result<Identity, String>> = std::sync::OnceLock::new();
        Self { trace: Trace { enabled: true, ..Trace::default() }, identity: IDENTITY.get_or_init(|| Identity::new("", 0)).clone(), done: 0, written: Instant::now() }
    }

    pub(crate) fn identity(&self, source: &str, width: usize) -> Result<Identity, String> {
        let mut identity = self.identity.clone()?;
        identity.source = Sha256::digest(source.as_bytes()).into();
        identity.width = width;
        Ok(identity)
    }

    pub fn with<R>(&mut self, f: impl FnOnce() -> R) -> R {
        struct Active<'a>(&'a mut Trace);
        impl Drop for Active<'_> {
            fn drop(&mut self) { TRACE.with(|t| std::mem::swap(&mut *t.borrow_mut(), self.0)); }
        }
        TRACE.with(|t| std::mem::swap(&mut *t.borrow_mut(), &mut self.trace));
        let active = Active(&mut self.trace);
        let result = f();
        drop(active);
        result
    }

    pub fn resume(&mut self, path: &Path, source: &str, width: usize, s: &mut Session, chunks: &[String], name: &str, progress: &dyn Fn(usize, usize)) -> Result<usize, String> {
        let started = Instant::now();
        let modified = std::fs::metadata(path).and_then(|m| m.modified()).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                format!("replay checkpoint {}: no checkpoint found; refusing resume", path.display())
            } else {
                format!("replay checkpoint {}: {e}; refusing resume", path.display())
            }
        })?;
        let (mut saved, canvas, elapsed) = load(path)?;
        if saved.done == 0 || saved.done > chunks.len() || !elapsed.is_finite() || elapsed < 0.0 {
            return Err("replay checkpoint: invalid chunk cursor or elapsed time".into());
        }
        let identity = self.identity(&source_prefix(source, saved.done)?, width)?;
        if saved.identity != identity {
            return Err("replay checkpoint: source prefix, executable, width or engine environment differs; refusing resume".into());
        }
        self.with(|| -> Result<(), String> {
            reconstruct(s, chunks, &mut saved, canvas, progress)?;
            let prefix: [u8; 32] = Sha256::digest(s.program(name).as_bytes()).into();
            if prefix != saved.identity.source {
                return Err("replay checkpoint: reconstructed source prefix differs; refusing resume".into());
            }
            Ok(())
        })?;
        self.done = saved.done;
        self.written = Instant::now() - modified.elapsed().unwrap_or_default().min(Duration::from_secs(900));
        eprintln!("reconstructed {} chunks from {} in {:.3}s", saved.done, path.display(), started.elapsed().as_secs_f64());
        Ok(saved.done)
    }

    pub fn completed(&mut self, path: &Path, s: &Session, source: &str, width: usize) -> Result<(), String> {
        let done = s.log.len();
        if done == 0 || (done.saturating_sub(self.done) < 10 && self.written.elapsed() < Duration::from_secs(900)) {
            return Ok(());
        }
        let identity = self.identity(source, width)?;
        self.with(|| write_checkpoint(path, s, &identity, done, &None, 0.0))?;
        self.done = done;
        self.written = Instant::now();
        Ok(())
    }
}

pub struct Batch {
    checkpoint: Option<PathBuf>,
    progress: PathBuf,
    identity: Option<Identity>,
    pub done: usize,
    total: usize,
    until: usize,
    minutes: Option<f64>,
    elapsed_before: f64,
    started: Instant,
    pub digests: Option<Vec<String>>,
}

impl Drop for Batch {
    fn drop(&mut self) { TRACE.with(|t| *t.borrow_mut() = Trace::default()); }
}

impl Batch {
    pub fn open(args: &[String], source: &str, width: usize, s: &mut Session, chunks: &[String], out: &Path) -> Result<Self, String> {
        let checkpoint = crate::flag(args, "--replay-checkpoint").map(PathBuf::from);
        let until = crate::flag(args, "--until-chunk").map(|n| n.parse::<usize>().map_err(|_| "--until-chunk needs a positive chunk number".to_string())).transpose()?.unwrap_or(chunks.len());
        if until == 0 || until > chunks.len() { return Err(format!("--until-chunk must be in 1..={}", chunks.len())); }
        let minutes = crate::flag(args, "--for-minutes").map(|v| v.parse::<f64>().map_err(|_| "--for-minutes needs a positive duration".to_string())).transpose()?;
        if minutes.is_some_and(|m| !m.is_finite() || m <= 0.0) { return Err("--for-minutes needs a positive finite duration".into()); }
        if checkpoint.is_none() && (until < chunks.len() || minutes.is_some()) { return Err("batch limits require --replay-checkpoint".into()); }
        let progress = checkpoint.as_deref().unwrap_or(out).with_extension("replay-progress.json");
        let identity = (checkpoint.is_some() || crate::flag(args, "--frames-every").is_some()).then(|| Identity::new(source, width)).transpose()?;
        let mut batch = Self { checkpoint, progress, identity, done: 0, total: chunks.len(), until, minutes, elapsed_before: 0.0, started: Instant::now(), digests: crate::flag(args, "--state-digest").map(|_| Vec::new()) };
        if let Some(path) = batch.checkpoint.as_ref().filter(|p| p.exists()) {
            let (mut saved, canvas, elapsed_secs) = load(path)?;
            if Some(&saved.identity) != batch.identity.as_ref() { return Err("replay checkpoint: source, executable, width or engine environment differs; refusing resume".into()); }
            if saved.digests.is_some() != batch.digests.is_some() || saved.digests.as_ref().is_some_and(|d| d.len() != saved.done) {
                return Err("replay checkpoint: --state-digest must be kept consistently across batches".into());
            }
            if saved.done == 0 || saved.done > chunks.len() || saved.done > until || !elapsed_secs.is_finite() || elapsed_secs < 0.0 {
                return Err("replay checkpoint: invalid chunk cursor or elapsed time".into());
            }
            reconstruct(s, chunks, &mut saved, canvas, &|_, _| {})?;
            batch.done = saved.done;
            batch.elapsed_before = elapsed_secs;
            batch.digests = saved.digests;
            eprintln!("reconstructed {} chunks from {}", batch.done, path.display());
        } else if batch.checkpoint.is_some() {
            TRACE.with(|t| *t.borrow_mut() = Trace { enabled: true, ..Trace::default() });
        }
        Ok(batch)
    }

    pub fn begin(&self) -> Result<(), String> {
        self.publish(if self.done == self.total { "complete" } else if self.stopped() { "paused" } else { "running" }, self.done.checked_add(1).filter(|n| *n <= self.total && !self.stopped()))
    }

    pub(crate) fn identity(&self) -> &Identity { self.identity.as_ref().unwrap() }

    fn elapsed(&self) -> f64 { self.elapsed_before + self.started.elapsed().as_secs_f64() }

    pub fn begin_chunk(&self, n: usize) -> Result<(), String> { self.publish("running", Some(n)) }

    pub fn completed(&mut self, s: &Session, done: usize) -> Result<bool, String> {
        self.done = done;
        if let Some(path) = &self.checkpoint {
            write_checkpoint(path, s, self.identity.as_ref().unwrap(), done, &self.digests, self.elapsed())?;
        }
        let stop = done >= self.until || self.minutes.is_some_and(|m| self.started.elapsed().as_secs_f64() >= m * 60.0);
        self.publish(if done == self.total { "complete" } else if stop { "paused" } else { "running" }, if done < self.total && !stop { Some(done + 1) } else { None })?;
        Ok(stop)
    }

    pub fn stopped(&self) -> bool { self.done >= self.until || self.done == self.total }

    fn publish(&self, status: &str, current_chunk: Option<usize>) -> Result<(), String> {
        let elapsed = self.elapsed();
        let rate = if elapsed > 0.0 && self.done > 0 { Some(self.done as f64 / elapsed) } else { None };
        let eta = rate.map(|r| (self.total - self.done) as f64 / r);
        let value = serde_json::json!({ "version": 1, "status": status, "chunks_done": self.done, "chunks_total": self.total, "current_chunk": current_chunk, "elapsed_secs": elapsed, "chunks_per_second": rate, "eta_secs": eta });
        atomic(&self.progress, |f| serde_json::to_writer(f, &value).map_err(|e| e.to_string()))?;
        eprintln!("replay {status}: {}/{} chunks, current {}, {:.1}s elapsed, {} chunks/s, ETA {}s", self.done, self.total, current_chunk.map_or_else(|| "-".into(), |n| n.to_string()), elapsed, rate.map_or_else(|| "-".into(), |r| format!("{r:.4}")), eta.map_or_else(|| "-".into(), |e| format!("{e:.0}")));
        Ok(())
    }
}

fn hash_reader(reader: &mut impl Read) -> Result<[u8; 32], String> {
    let mut hash = Sha256::new();
    let mut buffer = [0; 1 << 16];
    loop {
        let n = reader.read(&mut buffer).map_err(|e| e.to_string())?;
        if n == 0 { break; }
        hash.update(&buffer[..n]);
    }
    Ok(hash.finalize().into())
}

fn read_saved(reader: &mut impl Read, limit: u64) -> Result<Saved, String> {
    let options = bincode::DefaultOptions::new().with_fixint_encoding().with_limit(limit);
    let identity = options.deserialize_from(&mut *reader).map_err(|e| e.to_string())?;
    let done = options.deserialize_from(&mut *reader).map_err(|e| e.to_string())?;
    let count: usize = options.deserialize_from(&mut *reader).map_err(|e| e.to_string())?;
    let mut events = Vec::new();
    for _ in 0..count {
        events.push(options.deserialize_from(&mut *reader).map_err(|e| e.to_string())?);
    }
    let state = options.deserialize_from(&mut *reader).map_err(|e| e.to_string())?;
    let digests = options.deserialize_from(reader).map_err(|e| e.to_string())?;
    Ok(Saved { identity, done, events, state, digests })
}

fn load(path: &Path) -> Result<(Saved, Option<paint::Canvas>, f64), String> {
    let result = (|| {
        let mut f = std::fs::File::open(path).map_err(|e| e.to_string())?;
        let mut magic = [0; 8];
        let mut digest = [0; 32];
        f.read_exact(&mut magic).and_then(|_| f.read_exact(&mut digest)).map_err(|e| e.to_string())?;
        if &magic != MAGIC { return Err("not a resumable replay checkpoint (unsupported format)".into()); }
        if hash_reader(&mut f)? != digest { return Err("checkpoint checksum mismatch".into()); }
        f.seek(SeekFrom::Start(40)).map_err(|e| e.to_string())?;
        let mut elapsed = [0; 8];
        f.read_exact(&mut elapsed).map_err(|e| e.to_string())?;
        let mut reader = zstd::stream::read::Decoder::new(f).map_err(|e| e.to_string())?;
        let saved = read_saved(&mut reader, 4 << 30)?;
        let has_canvas: bool = bincode::deserialize_from(&mut reader).map_err(|e| e.to_string())?;
        let (canvas, header) = if has_canvas {
            let (canvas, header) = paint::Canvas::read_state(&mut reader).map_err(|e| e.to_string())?;
            (Some(canvas), header)
        } else { (None, String::new()) };
        let mut extra = [0];
        if !header.is_empty() || reader.read(&mut extra).map_err(|e| e.to_string())? != 0 { return Err("trailing or unexpected checkpoint data".into()); }
        Ok((saved, canvas, f64::from_le_bytes(elapsed)))
    })();
    result.map_err(|e: String| format!("replay checkpoint {}: {e}; refusing resume", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = r#"--@ box every
--@ engine 8
--@ knife-skim
--@ brush-skim
--@ blend-feather
--@ blend-at-masks
--@ glaze-film
--@ liquid-film-levels
--@ rag-lift
--@ full-palette-refuses
--@ drain
--@ chunk 1
counter = 0
local ok, e = pcall(function() drying(1,1) end)
assert(not ok and tostring(e):find("no canvas"))
--@ chunk 2
canvas{size=400, aspect=1, linen=15, seed=37, ground={{pile={{"lead white",1}},um=20,apply="knife"}}}
p = pile{{"red earth",1}}
b = brush{kind="round",width=20}
alias = b
local count = 0
function mark(x)
  count = count + 1
  counter = count
  if alias:fullness() < 0.4 then alias:load(p,0.8) end
  alias:stroke({{x,100},{x+130,500}}, {pressure=0.6})
end
mark(100)
--@ chunk 3
mark(250)
print(rand(), counter, drying(200,200))
--@ chunk 4
wait(83)
r = rag{width=20}
r:dip(0.6)
--@ chunk 5
mark(450)
r:wipe({{120,100},{240,400}})
print(r.damp, r.load)
k = knife{width=20}
k:load(p,0.5)
k:lay({{300,300},{400,400}})
h = pencil("2B")
h:line({{100,600},{150,640},{200,620}})
h:line({{200,620},{210,650}})
guide = drawing_guide()
--@ chunk 6
wait(7)
print(h.worn,guide:area())
mark(600)
r:refold()
k:scrape({{300,300},{400,400}})
--@ chunk 7
mark(720)
print(counter,rand(),drying(300,300))
"#;

    struct Temp(PathBuf);
    impl Temp {
        fn new() -> Self {
            static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let dir = std::env::temp_dir().join(format!("easel-replay-{}-{}", std::process::id(), NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)));
            std::fs::create_dir_all(&dir).unwrap();
            Self(dir)
        }
    }
    impl Drop for Temp { fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); } }

    fn session() -> Session { Session::replay_with(64, crate::session::box_for(Some(SOURCE)).unwrap()).unwrap() }
    fn args(dir: &Path, until: usize) -> Vec<String> {
        vec!["--replay-checkpoint".into(), dir.join("replay.ckpt").display().to_string(), "--until-chunk".into(), until.to_string()]
    }
    fn canvas(s: &Session) -> Vec<u8> {
        let mut bytes = Vec::new();
        s.canvas().unwrap().write_state(&mut bytes, "").unwrap();
        bytes
    }

    #[test]
    fn checkpoint_budget_is_per_record_not_per_history() {
        let limit = 64 << 10;
        let mut saved = Saved {
            identity: Identity { build: [1; 32], source: [2; 32], width: 64, environment: Vec::new() },
            done: 20,
            events: vec![vec![7; 8 << 10]; 20],
            state: State::capture(&session().st),
            digests: Some(vec!["kept".into()]),
        };
        let mut bytes = encode(&saved).unwrap();
        assert!(bytes.len() > limit as usize);
        bytes.push(0);
        let mut reader = bytes.as_slice();
        let restored = read_saved(&mut reader, limit).unwrap();
        assert_eq!(restored.identity, saved.identity);
        assert_eq!(restored.done, saved.done);
        assert_eq!(restored.events, saved.events);
        assert_eq!(encode(&restored.state).unwrap(), encode(&saved.state).unwrap());
        assert_eq!(restored.digests, saved.digests);
        assert_eq!(reader, [0]);

        saved.events = vec![vec![0; limit as usize]];
        assert!(read_saved(&mut encode(&saved).unwrap().as_slice(), limit).err().unwrap().contains("size limit"));
    }

    #[test]
    fn batch_identity_is_only_built_for_checkpoints_or_frames() {
        let dir = Temp::new();
        let chunks = crate::session::parse_program(SOURCE);
        for (args, expected) in [
            (Vec::new(), false),
            (args(&dir.0, chunks.len()), true),
            (vec!["--frames-every".into(), "2".into(), "--frames-dir".into(), dir.0.join("frames").display().to_string()], true),
        ] {
            let batch = Batch::open(&args, SOURCE, 64, &mut session(), &chunks, &dir.0.join("out.png")).unwrap();
            assert_eq!(batch.identity.is_some(), expected);
        }
    }

    #[test]
    fn checkpoint_resume_rejects_changed_build_width_and_environment() {
        let chunks = crate::session::parse_program(SOURCE);
        for change in 0..3 {
            let dir = Temp::new();
            let mut s = session();
            let mut batch = Batch::open(&args(&dir.0, 1), SOURCE, 64, &mut s, &chunks, &dir.0.join("out.png")).unwrap();
            let identity = batch.identity.as_mut().unwrap();
            match change {
                0 => identity.build[0] ^= 1,
                1 => identity.width += 1,
                _ => identity.environment.push((b"PAINT_TEST_IDENTITY".to_vec(), b"changed".to_vec())),
            }
            s.run(&chunks[0]).unwrap();
            batch.completed(&s, 1).unwrap();
            drop(batch);
            let mut restored = session();
            let result = Batch::open(&args(&dir.0, chunks.len()), SOURCE, 64, &mut restored, &chunks, &dir.0.join("out.png"));
            assert!(result.err().unwrap().contains("differs; refusing resume"));
            assert!(restored.log.is_empty());
        }
    }

    #[test]
    fn checkpoint_round_trip_reconstructs_closures_aliases_tools_rng_and_clock() {
        let dir = Temp::new();
        let chunks = crate::session::parse_program(SOURCE);
        let mut plain = session();
        for chunk in &chunks { plain.run(chunk).unwrap(); }
        let mut final_session = None;
        for until in [1, 3, 4, 5, chunks.len()] {
            let mut s = session();
            let mut batch = Batch::open(&args(&dir.0, until), SOURCE, 64, &mut s, &chunks, &dir.0.join("out.png")).unwrap();
            for (i, chunk) in chunks.iter().enumerate().skip(batch.done) {
                s.run(chunk).unwrap();
                if batch.completed(&s, i + 1).unwrap() { break; }
            }
            drop(batch);
            final_session = Some(s);
        }
        let s = final_session.unwrap();
        assert_eq!(canvas(&s), canvas(&plain));
        assert_eq!(encode(&State::capture(&s.st)).unwrap(), encode(&State::capture(&plain.st)).unwrap());
        assert_eq!(s.lua.globals().get::<usize>("counter").unwrap(), 5);
        assert_eq!(s.log.len(), chunks.len());
    }

    #[test]
    fn pastel_checkpoints_reconstruct_stick_wear_and_dry_tools() {
        let dir = Temp::new();
        let mut plain = session().with_paper(true).unwrap();
        for chunk in [
            "canvas{size=200, aspect=1.25, seed=3, paper={grammage=200}}",
            r#"p=pastel(pile{{"vermilion",1}}); b=brush("round",30); k=knife{width=20}"#,
            "p:stroke({{100,400},{900,400}}, {force=3,alt=55})",
            "lay_sheet(rect(0,0,500,800)); p:side({{100,400},{900,400}}); smudge({{100,400},{900,400}}); lift_sheet()",
            "b:dust({{200,400},{700,400}}); k:scrape({{200,400},{700,400}}, {edge=10}); blow(500,400); tap()",
            "smudge(rect(300,300,300,200), {pad=4,passes=1}); p:roll(45); p:stroke({{100,450},{900,450}}); print(feel(500,400))",
        ] { plain.run(chunk).unwrap(); }
        let source = plain.program("pastel");
        let chunks = crate::session::parse_program(&source);
        for until in [3, 5, chunks.len()] {
            let mut s = Session::replay_with(64, crate::session::box_for(Some(&source)).unwrap()).unwrap().with_paper(true).unwrap();
            let mut batch = Batch::open(&args(&dir.0, until), &source, 64, &mut s, &chunks, &dir.0.join("out.png")).unwrap();
            for (i, chunk) in chunks.iter().enumerate().skip(batch.done) {
                s.run(chunk).unwrap();
                if batch.completed(&s, i + 1).unwrap() { break; }
            }
            drop(batch);
            if until == chunks.len() {
                assert_eq!(canvas(&s), canvas(&plain));
                assert_eq!(encode(&State::capture(&s.st)).unwrap(), encode(&State::capture(&plain.st)).unwrap());
                assert_eq!(s.held_stick("p").unwrap(), plain.held_stick("p").unwrap());
                assert_eq!(s.program("pastel"), source);
            }
        }
    }

    #[test]
    fn corrupt_and_mismatched_checkpoints_are_refused() {
        let dir = Temp::new();
        let chunks = crate::session::parse_program(SOURCE);
        let mut s = session();
        let mut batch = Batch::open(&args(&dir.0, 2), SOURCE, 64, &mut s, &chunks, &dir.0.join("out.png")).unwrap();
        for (i, chunk) in chunks.iter().take(2).enumerate() { s.run(chunk).unwrap(); batch.completed(&s, i + 1).unwrap(); }
        drop(batch);
        let result = Batch::open(&args(&dir.0, chunks.len()), &format!("{SOURCE}\n-- changed"), 64, &mut session(), &chunks, &dir.0.join("out.png"));
        assert!(result.err().unwrap().contains("differs; refusing resume"));
        let path = dir.0.join("replay.ckpt");
        let mut bytes = std::fs::read(&path).unwrap();
        let last = bytes.len() - 1;
        bytes[last] ^= 1;
        std::fs::write(&path, bytes).unwrap();
        assert!(load(&path).err().unwrap().contains("checksum mismatch"));
    }

    #[test]
    fn progress_is_published_at_every_chunk_and_batch_boundary() {
        let dir = Temp::new();
        let chunks = crate::session::parse_program(SOURCE);
        let mut s = session();
        let mut batch = Batch::open(&args(&dir.0, 2), SOURCE, 64, &mut s, &chunks, &dir.0.join("out.png")).unwrap();
        let progress = dir.0.join("replay.replay-progress.json");
        assert!(!progress.exists());
        batch.begin().unwrap();
        let read = || serde_json::from_slice::<serde_json::Value>(&std::fs::read(&progress).unwrap()).unwrap();
        assert_eq!(read()["chunks_done"], 0);
        for (i, chunk) in chunks.iter().take(2).enumerate() {
            batch.begin_chunk(i + 1).unwrap();
            assert_eq!(read()["current_chunk"], i + 1);
            s.run(chunk).unwrap();
            batch.completed(&s, i + 1).unwrap();
            let value = read();
            assert_eq!(value["chunks_done"], i + 1);
            assert_eq!(value["chunks_total"], chunks.len());
            assert!(value["elapsed_secs"].as_f64().unwrap() > 0.0);
            assert!(value["chunks_per_second"].as_f64().unwrap() > 0.0);
            assert!(value["eta_secs"].as_f64().unwrap() > 0.0);
        }
        assert_eq!(read()["status"], "paused");
        assert!(read()["current_chunk"].is_null());
    }

    #[test]
    fn trace_failure_cannot_be_hidden_by_lua_pcall() {
        let mut s = session();
        TRACE.with(|t| *t.borrow_mut() = Trace { enabled: true, playback: true, ..Trace::default() });
        let result = s.run("pcall(function() wait(2) end)");
        TRACE.with(|t| *t.borrow_mut() = Trace::default());
        assert!(result.err().unwrap().contains("exhausted effect trace"));
    }

    #[test]
    fn live_checkpoint_cadence_and_failed_writes_preserve_last_success() {
        let dir = Temp::new();
        let path = dir.0.join("live.ckpt");
        let mut s = session();
        let mut live = Live::new();
        for done in 1..=10 {
            live.with(|| s.run("counter = (counter or 0) + 1")).unwrap();
            live.completed(&path, &s, &s.program("live"), 64).unwrap();
            assert_eq!(path.exists(), done == 10);
        }
        assert_eq!(load(&path).unwrap().0.done, 10);
        live.with(|| s.run("counter = (counter or 0) + 1")).unwrap();
        live.completed(&path, &s, &s.program("live"), 64).unwrap();
        assert_eq!(load(&path).unwrap().0.done, 10);
        live.written -= Duration::from_secs(900);
        let written = live.written;
        assert!(live.completed(&dir.0, &s, &s.program("live"), 64).is_err());
        assert_eq!(live.done, 10);
        assert_eq!(live.written, written);
        assert_eq!(load(&path).unwrap().0.done, 10);
        live.completed(&path, &s, &s.program("live"), 64).unwrap();
        assert_eq!(live.done, 11);
        assert!(live.written > written);
        assert_eq!(load(&path).unwrap().0.done, 11);
    }

    #[test]
    fn live_resume_accepts_appended_chunks_and_keeps_recording() {
        let dir = Temp::new();
        let path = dir.0.join("live.ckpt");
        let chunks = crate::session::parse_program(SOURCE);
        let mut s = session();
        let mut live = Live::new();
        for chunk in chunks.iter().take(3) { live.with(|| s.run(chunk)).unwrap(); }
        live.written -= Duration::from_secs(900);
        live.completed(&path, &s, &s.program("live"), 64).unwrap();
        let expected_canvas = canvas(&s);
        std::fs::File::open(&path).unwrap().set_times(std::fs::FileTimes::new().set_modified(std::time::SystemTime::now() - Duration::from_secs(900))).unwrap();
        for chunk in chunks.iter().skip(3) { live.with(|| s.run(chunk)).unwrap(); }
        let source = s.program("live");
        let chunks = crate::session::parse_program(&source);
        let mut restored = session();
        let mut resumed = Live::new();
        let progress = RefCell::new(Vec::new());
        assert_eq!(resumed.resume(&path, &source, 64, &mut restored, &chunks, "live", &|n, total| progress.borrow_mut().push((n, total))).unwrap(), 3);
        assert_eq!(*progress.borrow(), vec![(1, chunks.len()), (2, chunks.len()), (3, chunks.len())]);
        assert_eq!(canvas(&restored), expected_canvas);
        for (i, chunk) in chunks.iter().enumerate().skip(3) {
            resumed.with(|| restored.run(chunk)).unwrap();
            resumed.completed(&path, &restored, &restored.program("live"), 64).unwrap();
            if i == 3 { assert_eq!(load(&path).unwrap().0.done, 4); }
        }
        assert_eq!(canvas(&restored), canvas(&s));
        assert_eq!(encode(&State::capture(&restored.st)).unwrap(), encode(&State::capture(&s.st)).unwrap());
        assert_eq!(encode(&resumed.trace.events).unwrap(), encode(&live.trace.events).unwrap());
        assert!(!resumed.trace.playback);
        resumed.written -= Duration::from_secs(900);
        resumed.completed(&path, &restored, &source, 64).unwrap();
        assert_eq!(load(&path).unwrap().0.done, chunks.len());
    }

    #[test]
    fn live_checkpoint_reconstructs_temporary_tools_across_gc_schedules() {
        let dir = Temp::new();
        let path = dir.0.join("replay.ckpt");
        let mut s = session();
        s.lua.gc_stop();
        let mut live = Live::new();
        for chunk in [
            r#"canvas{size=100, aspect=1, linen=15, seed=3, ground={{pile={{"lead white",1}},um=20,apply="knife"}}}"#,
            "for i=1,100 do local r=rag{width=7}; r:dip(1); r:wipe({{100,100},{101,101}}) end",
        ] { live.with(|| s.run(chunk)).unwrap(); }
        live.written -= Duration::from_secs(900);
        let source = s.program("live");
        live.completed(&path, &s, &source, 64).unwrap();
        let mut restored = session();
        restored.lua.gc_set_mode(mlua::state::GcMode::Incremental(mlua::state::GcIncParams::default().pause(100).step_multiplier(1000).step_size(1)));
        restored.lua.gc_restart();
        let mut resumed = Live::new();
        let chunks = crate::session::parse_program(&source);
        assert_eq!(resumed.resume(&path, &source, 64, &mut restored, &chunks, "live", &|_, _| {}).unwrap(), 2);
        assert_eq!(canvas(&restored), canvas(&s));
        assert!(s.st.borrow_mut().live_rags().is_empty());
        assert!(restored.st.borrow_mut().live_rags().is_empty());
    }

    #[test]
    fn live_resume_rejects_missing_and_changed_prefix_before_reconstruction() {
        let dir = Temp::new();
        let path = dir.0.join("live.ckpt");
        let mut s = session();
        let mut live = Live::new();
        live.with(|| s.run("counter = 1")).unwrap();
        live.written -= Duration::from_secs(900);
        let source = s.program("live");
        live.completed(&path, &s, &source, 64).unwrap();
        let changed = source.replace("counter = 1", "counter = 2");
        let mut restored = session();
        let mut resumed = Live::new();
        let error = resumed.resume(&path, &changed, 64, &mut restored, &crate::session::parse_program(&changed), "live", &|_, _| panic!("must validate first")).unwrap_err();
        assert!(error.contains("source prefix, executable, width or engine environment differs"));
        assert!(restored.log.is_empty());
        let chunks = crate::session::parse_program(&source);
        assert!(resumed.resume(&path, &source, 65, &mut restored, &chunks, "live", &|_, _| panic!("must validate first")).unwrap_err().contains("differs"));
        let error = resumed.resume(&dir.0.join("missing"), &source, 64, &mut restored, &chunks, "live", &|_, _| {}).unwrap_err();
        assert!(error.contains("no checkpoint found") && error.contains("refusing resume"));
    }

    #[test]
    fn live_checkpoint_uses_its_startup_identity() {
        let dir = Temp::new();
        let path = dir.0.join("live.ckpt");
        let mut s = session();
        let mut live = Live::new();
        live.identity.as_mut().unwrap().build = [7; 32];
        live.identity.as_mut().unwrap().environment.push((b"PAINT_TEST_STARTUP".to_vec(), b"kept".to_vec()));
        live.with(|| s.run("counter = 1")).unwrap();
        live.written -= Duration::from_secs(900);
        live.completed(&path, &s, &s.program("live"), 64).unwrap();
        let saved = load(&path).unwrap().0;
        assert_eq!(saved.identity.build, [7; 32]);
        assert_eq!(saved.identity.environment, live.identity.as_ref().unwrap().environment);
    }

    #[test]
    fn live_scope_restores_both_traces_on_unwind() {
        let mut outer = Live::new();
        let mut inner = Live::new();
        outer.with(|| {
            read("outer", || Ok(1u32)).unwrap();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| inner.with(|| {
                read("inner", || Ok(2u32)).unwrap();
                panic!("unwind the trace scope");
            })));
            assert!(result.is_err());
            TRACE.with(|t| assert_eq!(t.borrow().events.len(), 1));
            read("outer again", || Ok(3u32)).unwrap();
        });
        assert_eq!(outer.trace.events.len(), 2);
        assert_eq!(inner.trace.events.len(), 1);
        TRACE.with(|t| assert!(!t.borrow().enabled));
    }

    #[test]
    fn effect_history_does_not_accumulate_in_memory() {
        const CHILD: &str = "EASEL_TEST_EFFECT_MEMORY";
        if std::env::var_os(CHILD).is_some() {
            let mut live = Live::new();
            let mut rng = paint::Rng::new(73);
            let mut payload = vec![0u8; 64 << 10];
            live.with(|| {
                for _ in 0..1024 {
                    for bytes in payload.chunks_exact_mut(8) {
                        bytes.copy_from_slice(&rng.next_u64().to_le_bytes());
                    }
                    read("memory", || Ok(payload.clone())).unwrap();
                }
            });
            assert_eq!(live.trace.events.len(), 1024);
            let output = std::process::Command::new(if cfg!(target_os = "macos") { "/bin/ps" } else { "ps" })
                .args(["-o", "rss=", "-p", &std::process::id().to_string()]).output().unwrap();
            assert!(output.status.success());
            let rss: usize = std::str::from_utf8(&output.stdout).unwrap().trim().parse().unwrap();
            assert!(rss < 48 << 10, "effect history retained {rss} KiB RSS for 64 MiB of payloads");
            return;
        }
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "replay_checkpoint::tests::effect_history_does_not_accumulate_in_memory", "--nocapture"])
            .env(CHILD, "1").output().unwrap();
        assert!(output.status.success(), "{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    }

    #[test]
    fn trace_rollback_discards_failed_events_and_restores_failure() {
        let mut live = Live::new();
        live.with(|| {
            read("kept", || Ok(1u32)).unwrap();
            let before = mark();
            read("discarded", || Ok(2u32)).unwrap();
            let _ = failed("discarded failure".into());
            assert!(check().is_err());
            rollback(before);
            check().unwrap();
            TRACE.with(|t| assert_eq!(t.borrow().events.len(), 1));
            let _ = failed("existing failure".into());
            let before = mark();
            TRACE.with(|t| t.borrow_mut().failure = Some("replaced".into()));
            rollback(before);
            assert!(check().unwrap_err().to_string().contains("existing failure"));
        });
    }

    #[test]
    fn exclusive_temporary_collisions_never_delete_existing_files() {
        let dir = Temp::new();
        let target = dir.0.join("progress.json");
        let part = |n| PathBuf::from(format!("{}.{}-{n}.part", target.display(), std::process::id()));
        std::fs::write(part(0), b"existing source").unwrap();
        atomic(&target, |f| f.write_all(b"new progress").map_err(|e| e.to_string())).unwrap();
        assert_eq!(std::fs::read(part(0)).unwrap(), b"existing source");
        assert_eq!(std::fs::read(&target).unwrap(), b"new progress");
        assert!(!part(1).exists());
        for n in 1..128 { std::fs::write(part(n), b"existing checkpoint").unwrap(); }
        assert!(atomic(&target, |_| Ok(())).is_err());
        for n in 0..128 { assert!(part(n).exists()); }
        assert_eq!(std::fs::read(&target).unwrap(), b"new progress");
    }

    #[test]
    fn unrelated_non_utf8_environment_is_ignored() {
        use std::os::unix::ffi::OsStringExt;
        if std::env::var_os("REPLAY_TEST_NON_UTF8").is_some() {
            Identity::new(SOURCE, 64).unwrap();
            return;
        }
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "replay_checkpoint::tests::unrelated_non_utf8_environment_is_ignored"])
            .env("REPLAY_TEST_NON_UTF8", std::ffi::OsString::from_vec(vec![0xff]))
            .status().unwrap();
        assert!(status.success());
    }
}
