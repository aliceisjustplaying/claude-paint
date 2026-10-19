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
const RESERVE_BYTES: u64 = 64 << 10;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Object {
    pub id: String,
    pub bytes: u64,
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
    pub checkpoint: Option<Object>,
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
    let bytes = bounded(&dir.join("revisions").join(format!("{id}.json")), MAX_MANIFEST_BYTES)?;
    if digest("easel-revision/1", &bytes) != id {
        return Err(invalid("revision integrity check failed"));
    }
    let m: Manifest = serde_json::from_slice(&bytes).map_err(|e| invalid(e.to_string()))?;
    if m.schema != 1 || m.state_schema != "easel-save-1" || m.renderer != "legacy-look-1" || m.resumable
        || !valid_id(&m.source_sha256) || !valid_id(&m.runtime_sha256)
        || m.parent.as_ref().is_some_and(|s| !valid_id(s))
        || !valid_id(&m.source.id) || m.source.bytes > MAX_SOURCE_BYTES || m.source_bytes != m.source.bytes
        || m.checkpoint.as_ref().is_some_and(|o| !valid_id(&o.id) || o.bytes > MAX_STATE_BYTES)
        || m.renderable != m.checkpoint.is_some() || m.finishable != m.renderable
        || m.window.is_some() != m.renderable || m.um_per_px_bits.is_some() != m.renderable
        || !(16..=9600).contains(&m.painting_width) || !paint::supported_engine(m.engine)
    {
        return Err(invalid("unsupported or inconsistent revision manifest"));
    }
    Ok(m)
}

fn object_path(dir: &Path, id: &str) -> PathBuf {
    dir.join("objects").join(format!("{id}.zst"))
}

fn verify_object(dir: &Path, object: &Object, kind: &str, limit: u64, collect: bool) -> io::Result<Vec<u8>> {
    if !valid_id(&object.id) || object.bytes > limit {
        return Err(invalid("invalid object ID or object budget"));
    }
    let mut reader = zstd::stream::read::Decoder::new(File::open(object_path(dir, &object.id))?)?;
    reader.window_log_max(23)?;
    let mut hash = HashCount::new(kind, object.bytes);
    let mut result = Vec::new();
    let mut buffer = [0; 65536];
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 { break }
        hash.write_all(&buffer[..n])?;
        if collect { result.extend_from_slice(&buffer[..n]); }
    }
    if hash.bytes != object.bytes || hash.id() != object.id {
        return Err(invalid("object integrity check failed"));
    }
    Ok(result)
}

fn verify_revision(dir: &Path, m: &Manifest) -> io::Result<String> {
    let source = verify_object(dir, &m.source, "easel-source/1", MAX_SOURCE_BYTES, true)?;
    if format!("{:x}", Sha256::digest(&source)) != m.source_sha256 || crate::session::parse_program(std::str::from_utf8(&source).map_err(|_| invalid("source isn't UTF-8"))?).len() != m.chunks {
        return Err(invalid("source prefix integrity check failed"));
    }
    if let Some(checkpoint) = &m.checkpoint {
        verify_object(dir, checkpoint, "easel-checkpoint/1", MAX_STATE_BYTES, false)?;
        let mut reader = zstd::stream::read::Decoder::new(File::open(object_path(dir, &checkpoint.id))?)?;
        reader.window_log_max(23)?;
        let header = paint::checkpoint::read_header(&mut reader)?;
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
        let mut state_hash = HashCount::new("easel-checkpoint/1", MAX_STATE_BYTES);
        let has_canvas = save::write_to(session, source, &mut state_hash)?;
        let checkpoint = has_canvas.then(|| Object { id: state_hash.id(), bytes: state_hash.bytes });
        let source_object = Object { id: digest("easel-source/1", source.as_bytes()), bytes: source.len() as u64 };
        if let Some((_, old)) = &self.current && old.source.id == source_object.id {
            if old.checkpoint.as_ref().map(|o| &o.id) != checkpoint.as_ref().map(|o| &o.id) { return Err(invalid("material changed without a journal transaction")); }
            return Ok(());
        }
        let st = session.st.borrow();
        let m = Manifest {
            schema: 1, parent: self.current.as_ref().map(|(id, _)| id.clone()),
            source: source_object, source_sha256: format!("{:x}", Sha256::digest(source.as_bytes())), source_bytes: source.len() as u64,
            chunks: session.log.len(), checkpoint, state_schema: "easel-save-1".into(), runtime_sha256: self.runtime.clone(), renderer: "legacy-look-1".into(),
            engine: st.tubes.engine, pigment_box: st.tubes.name.into(), painting_width: st.width,
            window: st.canvas.as_ref().map(|c| { let f = c.window(); [f.w, f.h, f.x0, f.y0, f.full_w, f.full_h] }),
            um_per_px_bits: st.canvas.as_ref().map(|c| (c.px_mm() * 1000.0).to_bits()),
            renderable: has_canvas, finishable: has_canvas, resumable: false,
        };
        drop(st);
        let bytes = serde_json::to_vec(&m).map_err(|e| invalid(e.to_string()))?;
        if bytes.len() as u64 > MAX_MANIFEST_BYTES { return Err(invalid("manifest exceeds its byte budget")); }
        let required = zstd::zstd_safe::compress_bound(source.len()) as u64
            + zstd::zstd_safe::compress_bound(state_hash.bytes as usize) as u64 + RESERVE_BYTES;
        if self.disk_bytes()?.checked_add(required).is_none_or(|n| n > self.budget) { return Err(invalid("insufficient transaction space in the store disk budget")); }
        let result = (|| {
            self.write_object(&m.source, "easel-source/1", "source.synced", step, |w| w.write_all(source.as_bytes()))?;
            if let Some(object) = &m.checkpoint {
                self.write_object(object, "easel-checkpoint/1", "checkpoint.synced", step, |w| save::write_to(session, source, w).map(|_| ()))?;
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
                if let Some(c) = m.checkpoint { objects.insert(format!("{}.zst", c.id)); }
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

    pub fn checkpoint(&self) -> io::Result<PathBuf> {
        self.manifest.checkpoint.as_ref().map(|c| object_path(&self.dir, &c.id)).ok_or_else(|| invalid("committed revision has no canvas yet"))
    }
}
