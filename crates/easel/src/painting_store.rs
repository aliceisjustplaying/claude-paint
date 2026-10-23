use crate::{save, session::Session};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub const DEFAULT_DISK_BYTES: u64 = 2 << 30;
const MAX_STATE_BYTES: u64 = 1 << 30;
const MAX_SOURCE_BYTES: u64 = 16 << 20;
const MAX_MANIFEST_BYTES: u64 = 16 << 10;
const CHECKPOINT_CHUNK_BYTES: u64 = 16 << 20;
const MAX_CHUNKED_STATE_BYTES: u64 = 64 << 30;
const MAX_CHECKPOINT_CHUNKS: usize = (MAX_CHUNKED_STATE_BYTES / CHECKPOINT_CHUNK_BYTES) as usize;
const MAX_CHUNKED_MANIFEST_BYTES: u64 = 512 << 10;
const RESERVE_BYTES: u64 = 64 << 10;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Object {
    pub id: String,
    pub bytes: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChunkedCheckpoint {
    pub id: String,
    pub bytes: u64,
    pub chunks: Vec<Object>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Checkpoint {
    Single(Object),
    Chunked(ChunkedCheckpoint),
}

impl Checkpoint {
    fn id(&self) -> &str {
        match self { Self::Single(o) => &o.id, Self::Chunked(c) => &c.id }
    }

    fn objects(&self) -> &[Object] {
        match self { Self::Single(o) => std::slice::from_ref(o), Self::Chunked(c) => &c.chunks }
    }

    fn bytes(&self) -> u64 {
        match self { Self::Single(o) => o.bytes, Self::Chunked(c) => c.bytes }
    }

    fn validate(&self, schema: u32) -> io::Result<()> {
        match self {
            Self::Single(o) if schema == 1 && valid_id(&o.id) && o.bytes <= MAX_STATE_BYTES => Ok(()),
            Self::Chunked(c) if schema == 2 => {
                if !valid_id(&c.id) || c.bytes == 0 || c.bytes > MAX_CHUNKED_STATE_BYTES
                    || c.chunks.is_empty() || c.chunks.len() > MAX_CHECKPOINT_CHUNKS
                    || c.chunks.len() as u64 != c.bytes.div_ceil(CHECKPOINT_CHUNK_BYTES)
                { return Err(invalid("invalid chunked checkpoint budget or count")); }
                let mut total = 0u64;
                for (i, o) in c.chunks.iter().enumerate() {
                    let expected = if i + 1 == c.chunks.len() {
                        c.bytes.checked_sub(total).ok_or_else(|| invalid("checkpoint length overflow"))?
                    } else { CHECKPOINT_CHUNK_BYTES };
                    if !valid_id(&o.id) || o.bytes == 0 || o.bytes > CHECKPOINT_CHUNK_BYTES || o.bytes != expected {
                        return Err(invalid("invalid checkpoint chunk length"));
                    }
                    total = total.checked_add(o.bytes).ok_or_else(|| invalid("checkpoint length overflow"))?;
                }
                if total != c.bytes { return Err(invalid("checkpoint aggregate length mismatch")); }
                Ok(())
            }
            _ => Err(invalid("checkpoint does not match its revision schema")),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema: u32,
    pub parent: Option<String>,
    pub source: Object,
    pub source_sha256: String,
    pub source_bytes: u64,
    pub chunks: usize,
    pub checkpoint: Option<Checkpoint>,
    pub state_schema: String,
    pub runtime_sha256: String,
    pub renderer: String,
    pub engine: u32,
    pub pigment_box: String,
    pub painting_width: usize,
    pub window: Option<[usize; 6]>,
    pub um_per_px_bits: Option<u32>,
    pub renderable: bool,
    pub finishable: bool,
    pub resumable: bool,
}

pub struct PaintingStore {
    dir: PathBuf,
    _writer: File,
    gate: File,
    pub current: Option<(String, Manifest)>,
    budget: u64,
    runtime: String,
    poisoned: bool,
}

pub struct ReadRevision {
    dir: PathBuf,
    _gate: File,
    pub id: String,
    pub manifest: Manifest,
    pub source: String,
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn valid_id(id: &str) -> bool {
    id.len() == 64 && id.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}

fn digest(kind: &str, bytes: &[u8]) -> String {
    let mut hash = Sha256::new();
    hash.update(kind.as_bytes());
    hash.update([0]);
    hash.update(bytes);
    format!("{:x}", hash.finalize())
}

fn bounded(path: &Path, limit: u64) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    File::open(path)?.take(limit + 1).read_to_end(&mut data)?;
    if data.len() as u64 > limit {
        return Err(invalid("store metadata exceeds its byte budget"));
    }
    Ok(data)
}

fn root(dir: &Path) -> io::Result<Option<String>> {
    match bounded(&dir.join("CURRENT"), 65) {
        Ok(bytes) => {
            if bytes == b"none\n" { return Ok(None); }
            let text = std::str::from_utf8(&bytes).map_err(|_| invalid("invalid current revision"))?;
            let id = text.strip_suffix('\n').filter(|s| valid_id(s)).ok_or_else(|| invalid("invalid current revision"))?;
            Ok(Some(id.into()))
        }
        Err(e) => Err(e),
    }
}

fn manifest(dir: &Path, id: &str) -> io::Result<Manifest> {
    if !valid_id(id) {
        return Err(invalid("invalid manifest ID"));
    }
    let bytes = bounded(&dir.join("revisions").join(format!("{id}.json")), MAX_CHUNKED_MANIFEST_BYTES)?;
    if digest("easel-revision/1", &bytes) != id {
        return Err(invalid("revision integrity check failed"));
    }
    let m: Manifest = serde_json::from_slice(&bytes).map_err(|e| invalid(e.to_string()))?;
    let manifest_limit = match m.schema {
        1 => MAX_MANIFEST_BYTES,
        2 if m.engine >= 9 => MAX_CHUNKED_MANIFEST_BYTES,
        _ => return Err(invalid("unsupported revision schema")),
    };
    if bytes.len() as u64 > manifest_limit || m.state_schema != "easel-save-1" || m.renderer != "legacy-look-1" || m.resumable
        || !valid_id(&m.source_sha256) || !valid_id(&m.runtime_sha256)
        || m.parent.as_ref().is_some_and(|s| !valid_id(s))
        || !valid_id(&m.source.id) || m.source.bytes > MAX_SOURCE_BYTES || m.source_bytes != m.source.bytes
        || m.renderable != m.checkpoint.is_some() || m.finishable != m.renderable
        || m.window.is_some() != m.renderable || m.um_per_px_bits.is_some() != m.renderable
        || !(16..=9600).contains(&m.painting_width) || !paint::supported_engine(m.engine)
    {
        return Err(invalid("unsupported or inconsistent revision manifest"));
    }
    if let Some(checkpoint) = &m.checkpoint { checkpoint.validate(m.schema)?; }
    Ok(m)
}

fn object_path(dir: &Path, id: &str) -> PathBuf {
    dir.join("objects").join(format!("{id}.zst"))
}

fn verify_object(dir: &Path, object: &Object, kind: &str, limit: u64, collect: bool) -> io::Result<Vec<u8>> {
    if !valid_id(&object.id) || object.bytes > limit {
        return Err(invalid("invalid object ID or object budget"));
    }
    let mut reader = ObjectReader::new(dir, object, kind)?;
    let mut result = Vec::new();
    if collect { io::copy(&mut reader, &mut result)?; }
    else { io::copy(&mut reader, &mut io::sink())?; }
    Ok(result)
}

fn verify_revision(dir: &Path, m: &Manifest) -> io::Result<String> {
    let source = verify_object(dir, &m.source, "easel-source/1", MAX_SOURCE_BYTES, true)?;
    if format!("{:x}", Sha256::digest(&source)) != m.source_sha256 || crate::session::parse_program(std::str::from_utf8(&source).map_err(|_| invalid("source isn't UTF-8"))?).len() != m.chunks {
        return Err(invalid("source prefix integrity check failed"));
    }
    if let Some(checkpoint) = &m.checkpoint {
        let mut reader = CheckpointReader::new(dir, checkpoint, m.schema)?;
        let header = paint::checkpoint::read_header(&mut reader)?;
        io::copy(&mut reader, &mut io::sink())?;
        let mut lines = header.lines();
        if lines.next() != Some("easel save 1") { return Err(invalid("checkpoint has no finishable save envelope")); }
        let fields: std::collections::BTreeMap<_, _> = lines.filter_map(|l| l.split_once('=')).collect();
        if fields.get("log_fnv").and_then(|v| u64::from_str_radix(v, 16).ok()) != Some(save::fnv1a(&source))
            || fields.get("chunks").and_then(|v| v.parse::<usize>().ok()) != Some(m.chunks)
            || fields.get("engine").and_then(|v| v.parse::<u32>().ok()) != Some(m.engine)
            || fields.get("box").copied() != Some(m.pigment_box.as_str())
            || fields.get("width").and_then(|v| v.parse::<usize>().ok()) != Some(m.painting_width)
        { return Err(invalid("checkpoint and accepted source do not agree")); }
    }
    String::from_utf8(source).map_err(|_| invalid("source isn't UTF-8"))
}

fn sync_dir(path: &Path) -> io::Result<()> {
    File::open(path)?.sync_all()
}

fn protect_output(dir: &Path, path: &Path) -> io::Result<()> {
    use std::os::unix::fs::MetadataExt;
    if crate::normalized_path(path)?.starts_with(crate::normalized_path(dir)?) {
        return Err(invalid("output must be outside the painting store"));
    }
    let output = match std::fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e),
    };
    for directory in [dir.to_path_buf(), dir.join("objects"), dir.join("revisions")] {
        for entry in std::fs::read_dir(directory)? {
            let metadata = std::fs::metadata(entry?.path())?;
            if metadata.is_file() && (metadata.dev(), metadata.ino()) == (output.dev(), output.ino()) {
                return Err(invalid("output aliases a painting store file"));
            }
        }
    }
    Ok(())
}

pub fn atomic_output(path: &Path, render: impl FnOnce(&Path) -> Result<(), String>) -> Result<(), String> {
    if image::ImageFormat::from_path(path).map_err(|e| e.to_string())? != image::ImageFormat::Png {
        return Err("committed-state output must be a PNG".into());
    }
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let dir = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let mut opened = None;
    for _ in 0..128 {
        let part = dir.join(format!(".pending-{}-{}.png", std::process::id(), NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)));
        match OpenOptions::new().write(true).create_new(true).open(&part) {
            Ok(file) => { opened = Some((part, file)); break }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e.to_string()),
        }
    }
    let (part, file) = opened.ok_or("cannot create exclusive output staging file")?;
    let result = (|| {
        render(&part)?;
        file.sync_all().map_err(|e| e.to_string())?;
        std::fs::rename(&part, path).map_err(|e| e.to_string())?;
        sync_dir(dir).map_err(|e| e.to_string())
    })();
    if result.is_err() { let _ = std::fs::remove_file(part); }
    result
}

fn temp(dir: &Path) -> io::Result<(PathBuf, File)> {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    for _ in 0..128 {
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = dir.join(format!(".pending-{}-{n}", std::process::id()));
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((path, file)),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
    Err(invalid("no exclusive temporary file available"))
}

struct HashCount {
    hash: Sha256,
    bytes: u64,
    limit: u64,
}

impl HashCount {
    fn new(kind: &str, limit: u64) -> Self {
        let mut hash = Sha256::new();
        hash.update(kind.as_bytes());
        hash.update([0]);
        Self { hash, bytes: 0, limit }
    }

    fn id(&self) -> String { format!("{:x}", self.hash.clone().finalize()) }
}

impl Write for HashCount {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let length = self.bytes.checked_add(bytes.len() as u64).filter(|n| *n <= self.limit).ok_or_else(|| invalid("state exceeds the raw byte budget"))?;
        self.hash.update(bytes);
        self.bytes = length;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

struct ChunkCounter {
    aggregate: HashCount,
    chunk: HashCount,
    chunks: Vec<Object>,
}

impl ChunkCounter {
    fn new() -> Self {
        Self {
            aggregate: HashCount::new("easel-checkpoint/2", MAX_CHUNKED_STATE_BYTES),
            chunk: HashCount::new("easel-checkpoint-chunk/2", CHECKPOINT_CHUNK_BYTES),
            chunks: Vec::new(),
        }
    }

    fn end_chunk(&mut self) {
        self.chunks.push(Object { id: self.chunk.id(), bytes: self.chunk.bytes });
        self.chunk = HashCount::new("easel-checkpoint-chunk/2", CHECKPOINT_CHUNK_BYTES);
    }

    fn finish(mut self) -> ChunkedCheckpoint {
        if self.chunk.bytes != 0 { self.end_chunk(); }
        ChunkedCheckpoint { id: self.aggregate.id(), bytes: self.aggregate.bytes, chunks: self.chunks }
    }
}

impl Write for ChunkCounter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let n = bytes.len().min((CHECKPOINT_CHUNK_BYTES - self.chunk.bytes) as usize);
        self.aggregate.write_all(&bytes[..n])?;
        self.chunk.write_all(&bytes[..n])?;
        if self.chunk.bytes == CHECKPOINT_CHUNK_BYTES { self.end_chunk(); }
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

struct ChunkWriter<'a> {
    buffer: Vec<u8>,
    write_chunk: &'a mut dyn FnMut(&[u8]) -> io::Result<()>,
}

impl<'a> ChunkWriter<'a> {
    fn new(write_chunk: &'a mut dyn FnMut(&[u8]) -> io::Result<()>) -> Self {
        Self { buffer: Vec::with_capacity(CHECKPOINT_CHUNK_BYTES as usize), write_chunk }
    }

    fn end_chunk(&mut self) -> io::Result<()> {
        (self.write_chunk)(&self.buffer)?;
        self.buffer.clear();
        Ok(())
    }

    fn finish(mut self) -> io::Result<()> {
        if !self.buffer.is_empty() { self.end_chunk()?; }
        Ok(())
    }
}

impl Write for ChunkWriter<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let n = bytes.len().min(CHECKPOINT_CHUNK_BYTES as usize - self.buffer.len());
        self.buffer.extend_from_slice(&bytes[..n]);
        if self.buffer.len() as u64 == CHECKPOINT_CHUNK_BYTES { self.end_chunk()?; }
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

struct ObjectReader {
    decoder: zstd::stream::read::Decoder<'static, io::BufReader<File>>,
    hash: HashCount,
    object: Object,
    done: bool,
}

impl ObjectReader {
    fn new(dir: &Path, object: &Object, kind: &str) -> io::Result<Self> {
        let mut decoder = zstd::stream::read::Decoder::new(File::open(object_path(dir, &object.id))?)?;
        decoder.window_log_max(23)?;
        Ok(Self { decoder, hash: HashCount::new(kind, object.bytes), object: object.clone(), done: false })
    }
}

impl Read for ObjectReader {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        if bytes.is_empty() || self.done { return Ok(0); }
        let n = self.decoder.read(bytes)?;
        self.hash.write_all(&bytes[..n])?;
        if n == 0 {
            if self.hash.bytes != self.object.bytes || self.hash.id() != self.object.id {
                return Err(invalid("object integrity check failed"));
            }
            self.done = true;
        }
        Ok(n)
    }
}

struct CheckpointReader {
    dir: PathBuf,
    checkpoint: Checkpoint,
    next: usize,
    reader: Option<ObjectReader>,
    hash: Option<HashCount>,
    done: bool,
}

impl CheckpointReader {
    fn new(dir: &Path, checkpoint: &Checkpoint, schema: u32) -> io::Result<Self> {
        checkpoint.validate(schema)?;
        Ok(Self { dir: dir.into(), checkpoint: checkpoint.clone(), next: 0, reader: None,
            hash: (schema == 2).then(|| HashCount::new("easel-checkpoint/2", checkpoint.bytes())), done: false })
    }
}

impl Read for CheckpointReader {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        if bytes.is_empty() || self.done { return Ok(0); }
        loop {
            if let Some(reader) = &mut self.reader {
                let n = reader.read(bytes)?;
                if n != 0 {
                    if let Some(hash) = &mut self.hash { hash.write_all(&bytes[..n])?; }
                    return Ok(n);
                }
                self.reader = None;
            }
            let Some(object) = self.checkpoint.objects().get(self.next) else {
                if let Some(hash) = &self.hash
                    && (hash.bytes != self.checkpoint.bytes() || hash.id() != self.checkpoint.id())
                {
                    return Err(invalid("checkpoint aggregate integrity check failed"));
                }
                self.done = true;
                return Ok(0);
            };
            let kind = match &self.checkpoint {
                Checkpoint::Single(_) => "easel-checkpoint/1",
                Checkpoint::Chunked(_) => "easel-checkpoint-chunk/2",
            };
            self.reader = Some(ObjectReader::new(&self.dir, object, kind)?);
            self.next += 1;
        }
    }
}

struct LimitedFile {
    file: File,
    remaining: u64,
}

impl Write for LimitedFile {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() as u64 > self.remaining { return Err(invalid("store disk budget exhausted")); }
        let n = self.file.write(bytes)?;
        self.remaining -= n as u64;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> { self.file.flush() }
}

struct CheckedEncoder<'a> {
    encoder: zstd::stream::write::Encoder<'a, LimitedFile>,
    hash: HashCount,
}

impl Write for CheckedEncoder<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let n = self.encoder.write(bytes)?;
        self.hash.write_all(&bytes[..n])?;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> { self.encoder.flush() }
}

impl PaintingStore {
    pub fn runtime_identity() -> io::Result<String> {
        let mut file = File::open(std::env::current_exe()?)?;
        let mut hash = Sha256::new();
        let mut bytes = [0; 65536];
        loop {
            let n = file.read(&mut bytes)?;
            if n == 0 { break }
            hash.update(&bytes[..n]);
        }
        Ok(format!("{:x}", hash.finalize()))
    }

    pub fn budget(value: Option<&str>) -> io::Result<u64> {
        match value {
            None => Ok(DEFAULT_DISK_BYTES),
            Some(value) => value.parse::<u64>().ok().filter(|v| (1..=65536).contains(v)).map(|v| v << 20).ok_or_else(|| invalid("state disk budget must be 1–65536 MiB")),
        }
    }

    pub fn validate_source(&self, expected: &str) -> io::Result<()> {
        let (id, _) = self.current.as_ref().ok_or_else(|| invalid("no accepted source revision"))?;
        if root(&self.dir)?.as_ref() != Some(id) { return Err(invalid("current revision changed unexpectedly")); }
        let m = manifest(&self.dir, id)?;
        if m.source_sha256 != format!("{:x}", Sha256::digest(expected.as_bytes())) || m.source_bytes != expected.len() as u64 {
            return Err(invalid("source is not the committed revision's prefix"));
        }
        Ok(())
    }

    pub fn protect_output(&self, path: &Path) -> io::Result<()> {
        protect_output(&self.dir, path)
    }

    pub fn open(dir: PathBuf, budget: u64, runtime: String) -> io::Result<Self> {
        if !valid_id(&runtime) { return Err(invalid("runtime identity isn't SHA-256")); }
        let dir = crate::normalized_path(&dir)?;
        let existing = dir.ancestors().find(|p| p.exists()).ok_or_else(|| invalid("store has no existing ancestor"))?.to_path_buf();
        std::fs::create_dir_all(&dir)?;
        for name in ["objects", "revisions"] {
            let path = dir.join(name);
            std::fs::create_dir_all(&path)?;
            if std::fs::symlink_metadata(path)?.file_type().is_symlink() { return Err(invalid("store directories cannot be symlinks")); }
        }
        let writer = OpenOptions::new().read(true).write(true).create(true).truncate(false).open(dir.join("writer.lock"))?;
        writer.try_lock().map_err(|_| invalid("another writer owns the painting store"))?;
        let gate = OpenOptions::new().read(true).write(true).create(true).truncate(false).open(dir.join("readers.lock"))?;
        gate.lock()?;
        if !dir.join("CURRENT").exists() {
            if std::fs::read_dir(dir.join("objects"))?.next().is_some() || std::fs::read_dir(dir.join("revisions"))?.next().is_some() {
                return Err(invalid("painting store lost its current pointer; accepted objects will not be collected"));
            }
            let (pending, mut file) = temp(&dir)?;
            let result = (|| {
                file.write_all(b"none\n")?;
                file.sync_all()?;
                std::fs::rename(&pending, dir.join("CURRENT"))
            })();
            if result.is_err() { let _ = std::fs::remove_file(pending); }
            result?;
        }
        sync_dir(&dir)?;
        for ancestor in dir.ancestors() {
            sync_dir(ancestor)?;
            if ancestor == existing { break }
        }
        let current = root(&dir)?.map(|id| manifest(&dir, &id).map(|m| (id, m))).transpose()?;
        if let Some((_, m)) = &current {
            verify_revision(&dir, m)?;
            if m.runtime_sha256 != runtime { return Err(invalid("recorded painter runtime differs; use the original runtime to continue")); }
        }
        let mut store = Self { dir, _writer: writer, gate, current, budget, runtime, poisoned: false };
        store.prune()?;
        if store.disk_bytes()? > budget { return Err(invalid("painting store exceeds its disk budget")); }
        store.gate.unlock()?;
        Ok(store)
    }

    pub fn source(&self) -> io::Result<Option<String>> {
        self.current.as_ref().map(|(_, m)| verify_revision(&self.dir, m)).transpose()
    }

    pub fn publish(&mut self, session: &Session, source: &str) -> io::Result<()> {
        self.publish_observed(session, source, &mut |_| Ok(()))
    }

    fn publish_observed(&mut self, session: &Session, source: &str, step: &mut dyn FnMut(&str) -> io::Result<()>) -> io::Result<()> {
        if self.poisoned { return Err(invalid("store publication failed; reopen before retrying")); }
        if source.len() as u64 > MAX_SOURCE_BYTES { return Err(invalid("source exceeds its byte budget")); }
        if root(&self.dir)? != self.current.as_ref().map(|(id, _)| id.clone()) { return Err(invalid("current revision changed unexpectedly")); }
        let previous = self.source()?;
        if previous.as_ref().is_some_and(|p| !source.starts_with(p)) { return Err(invalid("source would rewrite the accepted prefix")); }
        let source_object = Object { id: digest("easel-source/1", source.as_bytes()), bytes: source.len() as u64 };
        let schema = if session.st.borrow().tubes.engine >= 9
            && !self.current.as_ref().is_some_and(|(_, m)| m.schema == 1 && m.source.id == source_object.id)
        { 2 } else { 1 };
        let (has_canvas, checkpoint) = if schema == 1 {
            let mut hash = HashCount::new("easel-checkpoint/1", MAX_STATE_BYTES);
            let has_canvas = save::write_to(session, source, &mut hash)?;
            (has_canvas, has_canvas.then(|| Checkpoint::Single(Object { id: hash.id(), bytes: hash.bytes })))
        } else {
            let mut counter = ChunkCounter::new();
            let has_canvas = save::write_to(session, source, &mut counter)?;
            (has_canvas, has_canvas.then(|| Checkpoint::Chunked(counter.finish())))
        };
        if let Some(checkpoint) = &checkpoint { checkpoint.validate(schema)?; }
        if let Some((_, old)) = &self.current && old.source.id == source_object.id {
            if old.checkpoint.as_ref().map(Checkpoint::id) != checkpoint.as_ref().map(Checkpoint::id) { return Err(invalid("material changed without a journal transaction")); }
            return Ok(());
        }
        let st = session.st.borrow();
        let m = Manifest {
            schema, parent: self.current.as_ref().map(|(id, _)| id.clone()),
            source: source_object, source_sha256: format!("{:x}", Sha256::digest(source.as_bytes())), source_bytes: source.len() as u64,
            chunks: session.log.len(), checkpoint, state_schema: "easel-save-1".into(), runtime_sha256: self.runtime.clone(), renderer: "legacy-look-1".into(),
            engine: st.tubes.engine, pigment_box: st.tubes.name.into(), painting_width: st.width,
            window: st.canvas.as_ref().map(|c| { let f = c.window(); [f.w, f.h, f.x0, f.y0, f.full_w, f.full_h] }),
            um_per_px_bits: st.canvas.as_ref().map(|c| (c.px_mm() * 1000.0).to_bits()),
            renderable: has_canvas, finishable: has_canvas, resumable: false,
        };
        drop(st);
        let bytes = serde_json::to_vec(&m).map_err(|e| invalid(e.to_string()))?;
        let manifest_limit = if schema == 1 { MAX_MANIFEST_BYTES } else { MAX_CHUNKED_MANIFEST_BYTES };
        if bytes.len() as u64 > manifest_limit { return Err(invalid("manifest exceeds its byte budget")); }
        let mut required = zstd::zstd_safe::compress_bound(source.len()) as u64 + RESERVE_BYTES;
        if schema == 1 {
            required += zstd::zstd_safe::compress_bound(m.checkpoint.as_ref().map_or(0, Checkpoint::bytes) as usize) as u64;
        } else {
            required += bytes.len() as u64;
            if let Some(checkpoint) = &m.checkpoint {
                for object in checkpoint.objects() {
                    required = required.checked_add(zstd::zstd_safe::compress_bound(object.bytes as usize) as u64)
                        .ok_or_else(|| invalid("transaction byte budget overflow"))?;
                }
            }
        }
        if self.disk_bytes()?.checked_add(required).is_none_or(|n| n > self.budget) { return Err(invalid("insufficient transaction space in the store disk budget")); }
        let result = (|| {
            self.write_object(&m.source, "easel-source/1", "source.synced", step, |w| w.write_all(source.as_bytes()))?;
            match &m.checkpoint {
                Some(Checkpoint::Single(object)) => {
                    self.write_object(object, "easel-checkpoint/1", "checkpoint.synced", step, |w| save::write_to(session, source, w).map(|_| ()))?;
                }
                Some(Checkpoint::Chunked(checkpoint)) => {
                    self.write_chunks(checkpoint, step, |w| save::write_to(session, source, w).map(|_| ()))?;
                }
                None => {}
            }
            sync_dir(&self.dir.join("objects"))?;
            step("objects.synced")?;
            let id = digest("easel-revision/1", &bytes);
            self.install(&self.dir.join("revisions").join(format!("{id}.json")), &bytes, "manifest.synced", step)?;
            sync_dir(&self.dir.join("revisions"))?;
            step("manifest.installed")?;
            self.gate.lock()?;
            if root(&self.dir)? != m.parent { return Err(invalid("current revision changed before publication")); }
            let (pending, mut file) = temp(&self.dir)?;
            let pointer_result = (|| {
                writeln!(file, "{id}")?;
                file.sync_all()?;
                step("pointer.synced")?;
                std::fs::rename(&pending, self.dir.join("CURRENT"))?;
                self.current = Some((id, m));
                step("pointer.installed")?;
                sync_dir(&self.dir)?;
                step("pointer.directory.synced")
            })();
            if pointer_result.is_err() { let _ = std::fs::remove_file(pending); }
            pointer_result?;
            if let Err(e) = self.prune() { eprintln!("state store: revision is committed but garbage collection was deferred: {e}"); }
            self.gate.unlock()?;
            Ok(())
        })();
        if result.is_err() {
            self.poisoned = true;
            let _ = self.gate.unlock();
        }
        result
    }

    fn write_chunks(&self, checkpoint: &ChunkedCheckpoint, step: &mut dyn FnMut(&str) -> io::Result<()>, write: impl FnOnce(&mut ChunkWriter<'_>) -> io::Result<()>) -> io::Result<()> {
        let mut next = 0;
        let mut write_chunk = |bytes: &[u8]| {
            let object = checkpoint.chunks.get(next).ok_or_else(|| invalid("state grew while its chunks were being written"))?;
            if object.bytes != bytes.len() as u64 || object.id != digest("easel-checkpoint-chunk/2", bytes) {
                return Err(invalid("state changed while its chunks were being written"));
            }
            self.write_object(object, "easel-checkpoint-chunk/2", "checkpoint.synced", step, |w| w.write_all(bytes))?;
            next += 1;
            Ok(())
        };
        let mut writer = ChunkWriter::new(&mut write_chunk);
        write(&mut writer)?;
        writer.finish()?;
        if next != checkpoint.chunks.len() { return Err(invalid("state shrank while its chunks were being written")); }
        Ok(())
    }

    fn write_object(&self, object: &Object, kind: &str, phase: &str, step: &mut dyn FnMut(&str) -> io::Result<()>, write: impl FnOnce(&mut CheckedEncoder<'_>) -> io::Result<()>) -> io::Result<()> {
        let path = object_path(&self.dir, &object.id);
        if path.exists() {
            verify_object(&self.dir, object, kind, object.bytes, false)?;
            return Ok(());
        }
        let (pending, file) = temp(&self.dir.join("objects"))?;
        let result = (|| {
            let mut encoder = zstd::stream::write::Encoder::new(LimitedFile { file, remaining: zstd::zstd_safe::compress_bound(object.bytes as usize) as u64 }, 1)?;
            encoder.include_checksum(true)?;
            let mut writer = CheckedEncoder { encoder, hash: HashCount::new(kind, object.bytes) };
            write(&mut writer)?;
            if writer.hash.id() != object.id || writer.hash.bytes != object.bytes { return Err(invalid("state changed while its object was being written")); }
            let file = writer.encoder.finish()?.file;
            file.sync_all()?;
            use std::os::unix::fs::PermissionsExt;
            file.set_permissions(std::fs::Permissions::from_mode(0o400))?;
            file.sync_all()?;
            step(phase)?;
            std::fs::rename(&pending, path)
        })();
        if result.is_err() { let _ = std::fs::remove_file(pending); }
        result
    }

    fn install(&self, path: &Path, bytes: &[u8], phase: &str, step: &mut dyn FnMut(&str) -> io::Result<()>) -> io::Result<()> {
        let (pending, mut file) = temp(path.parent().unwrap())?;
        let result = (|| {
            file.write_all(bytes)?;
            file.sync_all()?;
            use std::os::unix::fs::PermissionsExt;
            file.set_permissions(std::fs::Permissions::from_mode(0o400))?;
            file.sync_all()?;
            step(phase)?;
            std::fs::rename(&pending, path)
        })();
        if result.is_err() { let _ = std::fs::remove_file(pending); }
        result
    }

    fn prune(&mut self) -> io::Result<()> {
        let mut revisions = BTreeSet::new();
        let mut objects = BTreeSet::new();
        if let Some((id, m)) = &self.current {
            revisions.insert(format!("{id}.json"));
            let mut retained = vec![m.clone()];
            if let Some(parent) = &m.parent {
                revisions.insert(format!("{parent}.json"));
                retained.push(manifest(&self.dir, parent)?);
            }
            for m in retained {
                objects.insert(format!("{}.zst", m.source.id));
                if let Some(c) = m.checkpoint {
                    for object in c.objects() { objects.insert(format!("{}.zst", object.id)); }
                }
            }
        }
        for (name, retained, extension) in [("objects", objects, ".zst"), ("revisions", revisions, ".json")] {
            let dir = self.dir.join(name);
            for entry in std::fs::read_dir(&dir)? {
                let entry = entry?;
                let name = entry.file_name().into_string().map_err(|_| invalid("unexpected store filename"))?;
                if !entry.file_type()?.is_file() { return Err(invalid("unexpected non-file in painting store")); }
                if name.starts_with(".pending-") || name.strip_suffix(extension).is_some_and(valid_id) && !retained.contains(&name) {
                    std::fs::remove_file(entry.path())?;
                } else if !retained.contains(&name) { return Err(invalid("unexpected file in painting store")); }
            }
            sync_dir(&dir)?;
        }
        for entry in std::fs::read_dir(&self.dir)? {
            let entry = entry?;
            if entry.file_name().to_str().is_some_and(|s| s.starts_with(".pending-")) && entry.file_type()?.is_file() { std::fs::remove_file(entry.path())?; }
        }
        sync_dir(&self.dir)
    }

    pub fn disk_bytes(&self) -> io::Result<u64> {
        let mut total = 0u64;
        for dir in [&self.dir, &self.dir.join("objects"), &self.dir.join("revisions")] {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let metadata = entry.metadata()?;
                if metadata.is_file() { total = total.checked_add(metadata.len()).ok_or_else(|| invalid("disk byte counter overflow"))?; }
            }
        }
        Ok(total)
    }
}

impl ReadRevision {
    pub fn protect_output(&self, path: &Path) -> io::Result<()> {
        protect_output(&self.dir, path)
    }

    pub fn open(dir: PathBuf) -> io::Result<Self> {
        let gate = File::open(dir.join("readers.lock"))?;
        gate.lock_shared()?;
        let id = root(&dir)?.ok_or_else(|| invalid("painting store has no committed revision"))?;
        let manifest = manifest(&dir, &id)?;
        let source = verify_revision(&dir, &manifest)?;
        Ok(Self { dir, _gate: gate, id, manifest, source })
    }

    pub fn checkpoint(&self) -> io::Result<impl Read + '_> {
        let checkpoint = self.manifest.checkpoint.as_ref().ok_or_else(|| invalid("committed revision has no canvas yet"))?;
        CheckpointReader::new(&self.dir, checkpoint, self.manifest.schema)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open_store(dir: &Path) -> PaintingStore {
        PaintingStore::open(dir.into(), DEFAULT_DISK_BYTES, "a".repeat(64)).unwrap()
    }

    fn session(engine: u32) -> Session {
        let tubes = crate::session::box_for(Some(&format!("--@ engine {engine}\n"))).unwrap();
        let mut session = Session::replay_with(16, tubes).unwrap();
        session.run(r#"canvas{size=100,aspect=1,linen=16,ground={{pile={{"lead white",1}},um=50,apply="knife"}}}"#).unwrap();
        session
    }

    fn specimen(writer: &mut impl Write) -> io::Result<()> {
        for value in [17, 34] {
            let block = [value; 65536];
            for _ in 0..CHECKPOINT_CHUNK_BYTES / block.len() as u64 { writer.write_all(&block)?; }
        }
        writer.write_all(&[51; 13])
    }

    fn check_stream(dir: &Path, checkpoint: &ChunkedCheckpoint) -> io::Result<u64> {
        io::copy(&mut CheckpointReader::new(dir, &Checkpoint::Chunked(checkpoint.clone()), 2)?, &mut io::sink())
    }

    #[test]
    fn chunk_counter_exceeds_one_gib_without_state_allocation() {
        let mut counter = ChunkCounter::new();
        let block = [0; 65536];
        for _ in 0..MAX_STATE_BYTES / block.len() as u64 { counter.write_all(&block).unwrap(); }
        counter.write_all(b"beyond").unwrap();
        let checkpoint = counter.finish();
        assert_eq!(checkpoint.bytes, MAX_STATE_BYTES + 6);
        assert_eq!(checkpoint.chunks.len(), 65);
        assert_eq!(checkpoint.chunks.last().unwrap().bytes, 6);
        Checkpoint::Chunked(checkpoint).validate(2).unwrap();
        let mut legacy = HashCount::new("easel-checkpoint/1", MAX_STATE_BYTES);
        legacy.bytes = MAX_STATE_BYTES;
        assert!(legacy.write_all(b"x").is_err());
        let mut counter = ChunkCounter::new();
        counter.aggregate.bytes = MAX_CHUNKED_STATE_BYTES - 1;
        assert!(counter.write_all(b"xx").is_err());
    }

    #[test]
    fn chunked_checkpoint_validates_counts_lengths_and_schema_limits() {
        let object = Object { id: "a".repeat(64), bytes: CHECKPOINT_CHUNK_BYTES };
        let maximum = ChunkedCheckpoint { id: "b".repeat(64), bytes: MAX_CHUNKED_STATE_BYTES, chunks: vec![object; MAX_CHECKPOINT_CHUNKS] };
        Checkpoint::Chunked(maximum.clone()).validate(2).unwrap();
        assert!(serde_json::to_vec(&maximum).unwrap().len() < MAX_CHUNKED_MANIFEST_BYTES as usize - 4096);
        for change in 0..7 {
            let mut bad = maximum.clone();
            match change {
                0 => bad.bytes += 1,
                1 => bad.chunks.push(bad.chunks[0].clone()),
                2 => { bad.chunks.pop(); }
                3 => bad.chunks[0].bytes -= 1,
                4 => bad.chunks.last_mut().unwrap().bytes = 0,
                5 => bad.chunks[0].id = "../escape".into(),
                _ => bad.bytes = 0,
            }
            assert!(Checkpoint::Chunked(bad).validate(2).is_err(), "change {change}");
        }
        assert!(Checkpoint::Chunked(maximum).validate(1).is_err());
        let legacy = Checkpoint::Single(Object { id: "c".repeat(64), bytes: MAX_STATE_BYTES });
        legacy.validate(1).unwrap();
        assert!(legacy.validate(2).is_err());
        assert!(Checkpoint::Single(Object { id: "c".repeat(64), bytes: MAX_STATE_BYTES + 1 }).validate(1).is_err());
    }

    #[test]
    fn chunked_checkpoint_stream_rejects_corruption_reordering_and_changed_writes() {
        let dir = tempfile::tempdir().unwrap();
        let store = open_store(dir.path());
        let mut counter = ChunkCounter::new();
        specimen(&mut counter).unwrap();
        let checkpoint = counter.finish();
        let mut synced = 0;
        assert!(store.write_chunks(&checkpoint, &mut |_| {
            synced += 1;
            if synced == 2 { return Err(io::Error::other("injected chunk failure")); }
            Ok(())
        }, |w| specimen(w)).is_err());
        assert!(object_path(dir.path(), &checkpoint.chunks[0].id).exists());
        assert!(!object_path(dir.path(), &checkpoint.chunks[1].id).exists());
        assert_eq!(root(dir.path()).unwrap(), None);
        drop(store);
        let store = open_store(dir.path());
        assert_eq!(std::fs::read_dir(dir.path().join("objects")).unwrap().count(), 0);
        store.write_chunks(&checkpoint, &mut |_| Ok(()), |w| specimen(w)).unwrap();
        assert_eq!(check_stream(dir.path(), &checkpoint).unwrap(), checkpoint.bytes);
        let mut reordered = checkpoint.clone();
        reordered.chunks.swap(0, 1);
        assert!(check_stream(dir.path(), &reordered).is_err());
        let mut wrong_hash = checkpoint.clone();
        wrong_hash.id = "0".repeat(64);
        assert!(check_stream(dir.path(), &wrong_hash).is_err());
        let last = object_path(dir.path(), &checkpoint.chunks.last().unwrap().id);
        let saved = std::fs::read(&last).unwrap();
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&last, std::fs::Permissions::from_mode(0o600)).unwrap();
        for raw in [&[51; 12][..], &[51; 14][..], &[52; 13][..]] {
            std::fs::write(&last, zstd::stream::encode_all(raw, 1).unwrap()).unwrap();
            assert!(check_stream(dir.path(), &checkpoint).is_err());
        }
        std::fs::write(&last, &saved[..saved.len() - 1]).unwrap();
        assert!(check_stream(dir.path(), &checkpoint).is_err());
        std::fs::write(&last, saved).unwrap();
        std::fs::remove_file(&last).unwrap();
        assert!(check_stream(dir.path(), &checkpoint).is_err());
        assert!(store.write_chunks(&checkpoint, &mut |_| Ok(()), |w| {
            let block = [34; 65536];
            for _ in 0..CHECKPOINT_CHUNK_BYTES / block.len() as u64 { w.write_all(&block)?; }
            Ok(())
        }).is_err());
        assert_eq!(root(dir.path()).unwrap(), None);
    }

    #[test]
    fn store_preserves_schema_one_and_streams_schema_two_saves() {
        for engine in [8, 9] {
            let dir = tempfile::tempdir().unwrap();
            let mut store = open_store(dir.path());
            let session = session(engine);
            let source = session.program("test");
            store.publish(&session, &source).unwrap();
            let id = store.current.as_ref().unwrap().0.clone();
            assert_eq!(store.current.as_ref().unwrap().1.schema, if engine == 8 { 1 } else { 2 });
            store.publish(&session, &source).unwrap();
            assert_eq!(store.current.as_ref().unwrap().0, id);
            drop(store);
            let revision = ReadRevision::open(dir.path().into()).unwrap();
            let restored = save::read_from(revision.checkpoint().unwrap(), dir.path()).unwrap();
            assert_eq!(restored.log_fnv, save::fnv1a(source.as_bytes()));
            assert_eq!(restored.chunks, session.log.len());
            assert_eq!(crate::state_digest_line(&restored.session, 1, 0.0).split(" brushes=").next(), crate::state_digest_line(&session, 1, 0.0).split(" brushes=").next());
            let output = tempfile::tempdir().unwrap();
            let png = output.path().join("state.png");
            crate::render_state(&[dir.path().display().to_string(), png.display().to_string(), "--mode".into(), "color".into(), "--look".into()]).unwrap();
            assert!(std::fs::read(png).unwrap().starts_with(b"\x89PNG"));
            #[cfg(feature = "render-snapshot")]
            {
                let view = output.path().join("state.view");
                crate::optical::export(&[dir.path().display().to_string(), view.display().to_string()]).unwrap();
                assert!(view.exists());
            }
            if engine == 8 {
                let bytes = serde_json::to_value(&revision.manifest).unwrap();
                assert!(bytes["checkpoint"].get("chunks").is_none());
                assert_eq!(bytes["checkpoint"].as_object().unwrap().len(), 2);
                let mut oversized = serde_json::to_vec(&revision.manifest).unwrap();
                oversized.resize(MAX_MANIFEST_BYTES as usize + 1, b' ');
                let id = digest("easel-revision/1", &oversized);
                std::fs::write(dir.path().join("revisions").join(format!("{id}.json")), oversized).unwrap();
                assert!(manifest(dir.path(), &id).is_err());
            }
        }
    }

    #[test]
    fn schema_one_engine_nine_is_readable_and_migrates_only_on_append() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = open_store(dir.path());
        let mut session = session(9);
        let source = session.program("test");
        store.publish(&session, &source).unwrap();
        let mut legacy = store.current.as_ref().unwrap().1.clone();
        let mut raw = Vec::new();
        save::write_to(&session, &source, &mut raw).unwrap();
        let object = Object { id: digest("easel-checkpoint/1", &raw), bytes: raw.len() as u64 };
        store.write_object(&object, "easel-checkpoint/1", "checkpoint.synced", &mut |_| Ok(()), |w| w.write_all(&raw)).unwrap();
        legacy.schema = 1;
        legacy.checkpoint = Some(Checkpoint::Single(object));
        let bytes = serde_json::to_vec(&legacy).unwrap();
        let id = digest("easel-revision/1", &bytes);
        store.install(&dir.path().join("revisions").join(format!("{id}.json")), &bytes, "manifest.synced", &mut |_| Ok(())).unwrap();
        std::fs::write(dir.path().join("CURRENT"), format!("{id}\n")).unwrap();
        drop(store);
        let revision = ReadRevision::open(dir.path().into()).unwrap();
        assert_eq!(revision.manifest.schema, 1);
        assert_eq!(save::read_from(revision.checkpoint().unwrap(), dir.path()).unwrap().chunks, 1);
        drop(revision);
        let mut store = open_store(dir.path());
        store.publish(&session, &source).unwrap();
        assert_eq!(store.current.as_ref().unwrap().0, id);
        session.run("wait(1)").unwrap();
        store.publish(&session, &session.program("test")).unwrap();
        let (_, current) = store.current.as_ref().unwrap();
        assert_eq!(current.schema, 2);
        assert_eq!(current.parent.as_ref(), Some(&id));
        assert!(matches!(manifest(dir.path(), &id).unwrap().checkpoint, Some(Checkpoint::Single(_))));
        drop(store);
        assert_eq!(ReadRevision::open(dir.path().into()).unwrap().manifest.schema, 2);
    }

    #[test]
    fn chunked_publication_is_atomic_and_retains_two_revisions_within_budget() {
        let phases = ["source.synced", "checkpoint.synced", "objects.synced", "manifest.synced", "manifest.installed", "pointer.synced", "pointer.installed", "pointer.directory.synced"];
        for phase in phases {
            let dir = tempfile::tempdir().unwrap();
            let mut store = open_store(dir.path());
            let mut session = session(9);
            let old_source = session.program("test");
            store.publish(&session, &old_source).unwrap();
            session.run("wait(1)").unwrap();
            let new_source = session.program("test");
            let mut reached = false;
            assert!(store.publish_observed(&session, &new_source, &mut |step| {
                if step == phase { reached = true; return Err(io::Error::other("injected failure")); }
                Ok(())
            }).is_err());
            assert!(reached, "{phase}");
            drop(store);
            let store = open_store(dir.path());
            let committed = if phase.starts_with("pointer.") && phase != "pointer.synced" { &new_source } else { &old_source };
            assert_eq!(store.source().unwrap().as_ref(), Some(committed));
            drop(store);
            let revision = ReadRevision::open(dir.path().into()).unwrap();
            assert_eq!(&revision.source, committed);
        }
        let dir = tempfile::tempdir().unwrap();
        let mut store = open_store(dir.path());
        let mut session = session(9);
        for _ in 0..3 {
            store.publish(&session, &session.program("test")).unwrap();
            session.run("wait(1)").unwrap();
        }
        assert_eq!(std::fs::read_dir(dir.path().join("revisions")).unwrap().count(), 2);
        let mut expected = BTreeSet::new();
        let (_, current) = store.current.as_ref().unwrap();
        for m in [current.clone(), manifest(dir.path(), current.parent.as_ref().unwrap()).unwrap()] {
            expected.insert(m.source.id);
            for o in m.checkpoint.unwrap().objects() { expected.insert(o.id.clone()); }
        }
        assert_eq!(std::fs::read_dir(dir.path().join("objects")).unwrap().count(), expected.len());
        let pointer = std::fs::read(dir.path().join("CURRENT")).unwrap();
        store.budget = store.disk_bytes().unwrap() + RESERVE_BYTES;
        assert!(store.publish(&session, &session.program("test")).is_err());
        assert_eq!(std::fs::read(dir.path().join("CURRENT")).unwrap(), pointer);
        assert!(store.disk_bytes().unwrap() <= store.budget);
    }
}
