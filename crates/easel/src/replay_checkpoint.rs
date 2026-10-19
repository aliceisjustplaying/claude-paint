use crate::api::S;
use crate::session::Session;
use bincode::Options;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

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
            rags: s.live_rags().iter().map(|b| *b.borrow()).collect(),
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
struct Trace {
    enabled: bool,
    playback: bool,
    nested: bool,
    events: Vec<Vec<u8>>,
    at: usize,
    failure: Option<String>,
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
        let bytes = t.events.get(t.at).ok_or_else(|| "replay checkpoint: exhausted effect trace".to_string())?;
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
    TRACE.with(|t| t.borrow_mut().events.push(bytes));
    value
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Identity {
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
    events: Vec<Vec<u8>>,
    state: State,
    digests: Option<Vec<String>>,
}

fn atomic(path: &Path, f: impl FnOnce(&mut std::fs::File) -> Result<(), String>) -> Result<(), String> {
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

pub struct Batch {
    checkpoint: Option<PathBuf>,
    progress: PathBuf,
    identity: Identity,
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
    pub fn open(args: &[String], file: &Path, source: &str, width: usize, s: &mut Session, chunks: &[String], out: &Path) -> Result<Self, String> {
        let checkpoint = crate::flag(args, "--replay-checkpoint").map(PathBuf::from);
        let until = crate::flag(args, "--until-chunk").map(|n| n.parse::<usize>().map_err(|_| "--until-chunk needs a positive chunk number".to_string())).transpose()?.unwrap_or(chunks.len());
        if until == 0 || until > chunks.len() { return Err(format!("--until-chunk must be in 1..={}", chunks.len())); }
        let minutes = crate::flag(args, "--for-minutes").map(|v| v.parse::<f64>().map_err(|_| "--for-minutes needs a positive duration".to_string())).transpose()?;
        if minutes.is_some_and(|m| !m.is_finite() || m <= 0.0) { return Err("--for-minutes needs a positive finite duration".into()); }
        if checkpoint.is_none() && (until < chunks.len() || minutes.is_some()) { return Err("batch limits require --replay-checkpoint".into()); }
        let progress = checkpoint.as_deref().unwrap_or(out).with_extension("replay-progress.json");
        let mut outputs = vec![out.to_path_buf()];
        if args.iter().any(|a| a == "--look") { outputs.push(out.with_extension("look.png")); }
        for option in ["--checkpoint-out", "--state-digest", "--dump-surface"] {
            if let Some(p) = crate::flag(args, option) { outputs.push(PathBuf::from(p)); }
        }
        if crate::same_file(file, &progress)? { return Err("replay progress must differ from source".into()); }
        for p in &outputs {
            if crate::same_file(p, &progress)? { return Err("replay progress must differ from outputs".into()); }
        }
        if let Some(p) = &checkpoint {
            for other in [file, progress.as_path()].into_iter().chain(outputs.iter().map(|p| p.as_path())) {
                if crate::same_file(p, &other)? { return Err("--replay-checkpoint must differ from source, progress and outputs".into()); }
            }
        }
        let identity = Identity::new(source, width)?;
        let mut batch = Self { checkpoint, progress, identity, done: 0, total: chunks.len(), until, minutes, elapsed_before: 0.0, started: Instant::now(), digests: crate::flag(args, "--state-digest").map(|_| Vec::new()) };
        if let Some(path) = batch.checkpoint.as_ref().filter(|p| p.exists()) {
            let (saved, canvas, elapsed_secs) = load(path)?;
            if saved.identity != batch.identity { return Err("replay checkpoint: source, executable, width or engine environment differs; refusing resume".into()); }
            if saved.digests.is_some() != batch.digests.is_some() || saved.digests.as_ref().is_some_and(|d| d.len() != saved.done) {
                return Err("replay checkpoint: --state-digest must be kept consistently across batches".into());
            }
            if saved.done == 0 || saved.done > chunks.len() || saved.done > until || !elapsed_secs.is_finite() || elapsed_secs < 0.0 {
                return Err("replay checkpoint: invalid chunk cursor or elapsed time".into());
            }
            TRACE.with(|t| *t.borrow_mut() = Trace { enabled: true, playback: true, events: saved.events, ..Trace::default() });
            for (i, chunk) in chunks.iter().take(saved.done).enumerate() {
                s.run(chunk).map_err(|e| format!("replay checkpoint: prefix chunk {} failed: {e}", i + 1))?;
            }
            check().map_err(|e| e.to_string())?;
            let complete = TRACE.with(|t| { let t = t.borrow(); t.at == t.events.len() });
            if !complete || encode(&State::capture(&s.st))? != encode(&saved.state)? { return Err("replay checkpoint: incomplete or divergent reconstructed state".into()); }
            s.st.borrow_mut().canvas = canvas;
            TRACE.with(|t| t.borrow_mut().playback = false);
            batch.done = saved.done;
            batch.elapsed_before = elapsed_secs;
            batch.digests = saved.digests;
            eprintln!("reconstructed {} chunks from {}", batch.done, path.display());
        } else if batch.checkpoint.is_some() {
            TRACE.with(|t| *t.borrow_mut() = Trace { enabled: true, ..Trace::default() });
        }
        batch.publish(if batch.done == batch.total { "complete" } else if batch.stopped() { "paused" } else { "running" }, batch.done.checked_add(1).filter(|n| *n <= batch.total && !batch.stopped()))?;
        Ok(batch)
    }

    fn elapsed(&self) -> f64 { self.elapsed_before + self.started.elapsed().as_secs_f64() }

    pub fn begin_chunk(&self, n: usize) -> Result<(), String> { self.publish("running", Some(n)) }

    pub fn completed(&mut self, s: &Session, done: usize) -> Result<bool, String> {
        self.done = done;
        if let Some(path) = &self.checkpoint {
            check().map_err(|e| e.to_string())?;
            let state = State::capture(&s.st);
            atomic(path, |f| {
                f.write_all(MAGIC).and_then(|_| f.write_all(&[0; 40])).map_err(|e| e.to_string())?;
                {
                    let mut writer = zstd::stream::write::Encoder::new(&mut *f, 1).map_err(|e| e.to_string())?;
                    TRACE.with(|t| {
                        #[derive(Serialize)]
                        struct SavedRef<'a> { identity: &'a Identity, done: usize, events: &'a Vec<Vec<u8>>, state: &'a State, digests: &'a Option<Vec<String>> }
                        bincode::serialize_into(&mut writer, &SavedRef { identity: &self.identity, done, events: &t.borrow().events, state: &state, digests: &self.digests }).map_err(|e| e.to_string())
                    })?;
                    bincode::serialize_into(&mut writer, &s.canvas().is_some()).map_err(|e| e.to_string())?;
                    if let Some(c) = s.canvas() { c.write_state(&mut writer, "").map_err(|e| e.to_string())?; }
                    writer.finish().map_err(|e| e.to_string())?;
                }
                f.seek(SeekFrom::Start(40)).and_then(|_| f.write_all(&self.elapsed().to_le_bytes())).map_err(|e| e.to_string())?;
                f.seek(SeekFrom::Start(40)).map_err(|e| e.to_string())?;
                let digest = hash_reader(&mut *f)?;
                f.seek(SeekFrom::Start(8)).and_then(|_| f.write_all(&digest)).map_err(|e| e.to_string())
            })?;
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
        let saved = bincode::DefaultOptions::new().with_fixint_encoding().with_limit(4 << 30).deserialize_from(&mut reader).map_err(|e| e.to_string())?;
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
    fn checkpoint_round_trip_reconstructs_closures_aliases_tools_rng_and_clock() {
        let dir = Temp::new();
        let source = dir.0.join("painting.lua");
        let chunks = crate::session::parse_program(SOURCE);
        let mut plain = session();
        for chunk in &chunks { plain.run(chunk).unwrap(); }
        let mut final_session = None;
        for until in [1, 3, 4, 5, chunks.len()] {
            let mut s = session();
            let mut batch = Batch::open(&args(&dir.0, until), &source, SOURCE, 64, &mut s, &chunks, &dir.0.join("out.png")).unwrap();
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
    fn corrupt_and_mismatched_checkpoints_are_refused() {
        let dir = Temp::new();
        let source = dir.0.join("painting.lua");
        let chunks = crate::session::parse_program(SOURCE);
        let mut s = session();
        let mut batch = Batch::open(&args(&dir.0, 2), &source, SOURCE, 64, &mut s, &chunks, &dir.0.join("out.png")).unwrap();
        for (i, chunk) in chunks.iter().take(2).enumerate() { s.run(chunk).unwrap(); batch.completed(&s, i + 1).unwrap(); }
        drop(batch);
        let result = Batch::open(&args(&dir.0, chunks.len()), &source, &format!("{SOURCE}\n-- changed"), 64, &mut session(), &chunks, &dir.0.join("out.png"));
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
        let source = dir.0.join("painting.lua");
        let chunks = crate::session::parse_program(SOURCE);
        let mut s = session();
        let mut batch = Batch::open(&args(&dir.0, 2), &source, SOURCE, 64, &mut s, &chunks, &dir.0.join("out.png")).unwrap();
        let progress = dir.0.join("replay.replay-progress.json");
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
