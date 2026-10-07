//! One locked owner of a live session, its painting log and committed witness.
//! A successful chunk is committed only after both files have been saved.

use crate::{
    painting_log::PaintingLog,
    replay, save,
    session::{self, Ran, Session},
};
use std::{
    fs::{self, File, OpenOptions},
    io::{Seek, Write},
    path::{Path, PathBuf},
    time::Instant,
};

/// Files may be reset or replayed only while this ownership is held. `open`'s
/// transport can pass the already locked file it handed to the server on stdin.
pub(crate) struct SessionFiles {
    directory: PathBuf,
    name: String,
    _lock: File,
}

pub(crate) fn lock_path(directory: &Path, name: &str) -> PathBuf {
    directory.join("out/easel").join(name).join("lock")
}

pub(crate) fn try_lock(directory: &Path, name: &str) -> Result<Option<File>, String> {
    let p = lock_path(directory, name);
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&p)
        .map_err(|e| format!("{}: {e}", p.display()))?;
    match f.try_lock() {
        Ok(()) => Ok(Some(f)),
        Err(std::fs::TryLockError::WouldBlock) => Ok(None),
        Err(std::fs::TryLockError::Error(e)) => Err(format!("{}: {e}", p.display())),
    }
}

impl SessionFiles {
    pub(crate) fn acquire(
        directory: &Path,
        name: String,
        inherited: Option<File>,
    ) -> Result<Self, String> {
        crate::valid_name(&name)?;
        fs::create_dir_all(directory.join("out/easel").join(&name)).map_err(|e| e.to_string())?;
        let p = lock_path(directory, &name);
        let lock = match inherited {
            Some(f) => {
                use std::os::unix::fs::MetadataExt;
                let (held, want) = (
                    f.metadata().map_err(|e| e.to_string())?,
                    fs::metadata(&p).map_err(|e| e.to_string())?,
                );
                if (held.dev(), held.ino()) != (want.dev(), want.ino()) {
                    return Err(format!(
                        "{} is set but stdin isn't {}",
                        crate::LOCK_ON_STDIN,
                        p.display()
                    ));
                }
                f.try_lock().map_err(|e| format!("{}: {e}", p.display()))?;
                f
            }
            None => try_lock(directory, &name)?.ok_or_else(|| {
                let pid = fs::read_to_string(&p).unwrap_or_default();
                format!(
                    "another easel serves session {name:?} (pid {}); nothing ran",
                    pid.lines().next().unwrap_or("")
                )
            })?,
        };
        let mut f = &lock;
        let _ = f
            .set_len(0)
            .and_then(|_| f.rewind())
            .and_then(|_| writeln!(f, "{}", std::process::id()));
        Ok(Self {
            directory: directory.to_path_buf(),
            name,
            _lock: lock,
        })
    }

    /// Socketless scratch ownership is distinguished from a primary server
    /// starting up, so `open` rejects it instead of waiting for a nonexistent socket.
    pub(crate) fn mark_scratch(&self) -> Result<(), String> {
        let mut f = &self._lock;
        f.set_len(0)
            .and_then(|_| f.rewind())
            .and_then(|_| writeln!(f, "{}\nscratch", std::process::id()))
            .map_err(|e| e.to_string())
    }

    pub(crate) fn dir(&self) -> PathBuf {
        self.directory.join("out/easel").join(&self.name)
    }
    pub(crate) fn log_path(&self) -> PathBuf {
        self.directory
            .join("paintings/lua")
            .join(format!("{}.lua", self.name))
    }
    fn witness(&self) -> PathBuf {
        self.dir().join("committed.lua")
    }
    pub(crate) fn exists(&self) -> bool {
        self.log_path().exists() || self.witness().exists()
    }

    /// First free backup number, with ownership already established. Never
    /// remove a witness or rename a log on behalf of an unlocked scratch canvas.
    pub(crate) fn put_aside(&self) -> Result<Option<PathBuf>, String> {
        let lp = self.log_path();
        let aside = if lp.exists() {
            let mut n = 1;
            let to = loop {
                let p = self
                    .directory
                    .join("paintings/lua")
                    .join(format!("{}-{n}.lua", self.name));
                if !p.exists() {
                    break p;
                }
                n += 1;
            };
            fs::rename(&lp, &to).map_err(|e| format!("{}: {e}", lp.display()))?;
            Some(to)
        } else {
            None
        };
        match fs::remove_file(self.witness()) {
            Ok(()) => (),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(e) => return Err(e.to_string()),
        }
        Ok(aside)
    }

    /// On replay failure return the still-locked files: scratch can put them
    /// aside without releasing ownership between validation and reset.
    pub(crate) fn open(self, like: Option<&Session>) -> Result<PersistedSession, (Self, String)> {
        match self.load(like) {
            Ok((s, written, replayed)) => Ok(PersistedSession {
                files: self,
                s,
                written,
                replayed,
            }),
            Err(e) => Err((self, e)),
        }
    }

    fn load(&self, like: Option<&Session>) -> Result<(Session, String, usize), String> {
        let lp = self.log_path();
        let text = if self.exists() {
            Some(fs::read_to_string(&lp).map_err(|e| format!("session integrity: {e}"))?)
        } else {
            None
        };
        if let Some(t) = &text {
            self.integrity(t).validate()?;
        }
        let log = text.as_deref().map(PaintingLog::parse);
        let (tubes, width) = match (like, &text) {
            (Some(l), None) => ((*l.st.borrow().tubes).clone(), l.st.borrow().width),
            _ => (
                session::box_for(log.as_ref())?,
                crate::width_for(&self.name, log.as_ref()),
            ),
        };
        let s = Session::with_box(width, tubes).map_err(|e| e.to_string())?;
        if let Some(text) = text {
            let log = log.as_ref().unwrap();
            struct Resume {
                started: Instant,
                total: usize,
            }
            impl replay::Observer for Resume {
                fn before_chunk(&mut self, number: usize, total: usize) -> Result<(), String> {
                    self.started = Instant::now();
                    eprintln!("resuming chunk {number}/{total}");
                    Ok(())
                }
                fn chunk_end(
                    &mut self,
                    _s: &Session,
                    number: usize,
                    _ran: &Ran,
                ) -> Result<(), String> {
                    eprintln!(
                        "resumed chunk {number}/{} {:.2}s",
                        self.total,
                        self.started.elapsed().as_secs_f64()
                    );
                    Ok(())
                }
            }
            let s = replay::replay(log, || Ok(s), false, &mut Resume { started: Instant::now(), total: log.chunks.len() })
                .map_err(|e| match e {
                    replay::Error::Chunk { number, error } => format!("session integrity: replay failed at chunk {number}: {error}; refusing partial session"),
                    replay::Error::Session(e) | replay::Error::Observer(e) => e,
                })?;
            if s.program(&self.name) != log.render() {
                return Err(
                    "session integrity: noncanonical or incomplete log; refusing replay".into(),
                );
            }
            println!("resumed {} chunks from {}", s.log.len(), lp.display());
            Ok((s, text, log.chunks.len()))
        } else {
            let text = s.program(&self.name);
            self.initialize(&text)?;
            Ok((s, text, 0))
        }
    }

    /// Preserve bootstrap's existing failure behavior: an unfinished initial
    /// witness leaves the log on disk and the next open refuses it as incomplete.
    fn initialize(&self, text: &str) -> Result<(), String> {
        let p = self.log_path();
        fs::create_dir_all(p.parent().unwrap()).map_err(|e| e.to_string())?;
        let mut log = OpenOptions::new()
            .append(true)
            .create_new(true)
            .open(&p)
            .map_err(|e| format!("session integrity: {e}"))?;
        log.write_all(text.as_bytes())
            .and_then(|_| log.sync_all())
            .map_err(|e| format!("session integrity: {e}"))?;
        let pending = self.witness().with_extension("pending");
        let mut witness = File::create(&pending).map_err(|e| e.to_string())?;
        witness
            .write_all(text.as_bytes())
            .and_then(|_| witness.sync_all())
            .map_err(|e| e.to_string())?;
        fs::rename(pending, self.witness()).map_err(|e| e.to_string())
    }

    fn integrity(&self, expected: &str) -> Integrity {
        Integrity {
            paths: [self.log_path(), self.witness()],
            expected: expected.to_string(),
        }
    }

    /// Prepare the witness, append and sync the log, then publish the witness.
    /// Until publication, errors truncate the append back to the committed prefix.
    /// A crash between writes still fails closed, rather than replaying a partial chunk.
    fn append(&self, previous: &str, text: &str) -> Result<(), String> {
        self.integrity(previous).validate()?;
        let prefix = previous;
        let suffix = text
            .strip_prefix(prefix)
            .ok_or("session integrity: log would rewrite history")?;
        let p = self.log_path();
        fs::create_dir_all(p.parent().unwrap()).map_err(|e| e.to_string())?;
        let pending = self.witness().with_extension("pending");
        let result = (|| {
            let mut witness = File::create(&pending).map_err(|e| e.to_string())?;
            witness
                .write_all(text.as_bytes())
                .and_then(|_| witness.sync_all())
                .map_err(|e| e.to_string())?;
            let mut opts = OpenOptions::new();
            opts.append(true);
            let mut log = opts
                .open(&p)
                .map_err(|e| format!("session integrity: {e}"))?;
            let mut published = false;
            let result = (|| {
                log.write_all(suffix.as_bytes())
                    .and_then(|_| log.sync_all())
                    .map_err(|e| e.to_string())?;
                fs::rename(&pending, self.witness()).map_err(|e| e.to_string())?;
                published = true;
                File::open(self.dir())
                    .and_then(|f| f.sync_all())
                    .map_err(|e| e.to_string())?;
                File::open(p.parent().unwrap())
                    .and_then(|f| f.sync_all())
                    .map_err(|e| e.to_string())
            })();
            if let Err(e) = result {
                let rollback = (|| -> std::io::Result<()> {
                    log.set_len(prefix.len() as u64)?;
                    log.sync_all()?;
                    if published {
                        let mut f = File::create(&pending)?;
                        f.write_all(prefix.as_bytes())?;
                        f.sync_all()?;
                        fs::rename(&pending, self.witness())?;
                        File::open(self.dir())?.sync_all()?;
                    }
                    Ok(())
                })();
                return Err(match rollback {
                    Ok(()) => e,
                    Err(why) => format!(
                        "{e}; the committed files couldn't be restored ({why}); the easel goes on only from the log it wrote"
                    ),
                });
            }
            Ok(())
        })();
        let _ = fs::remove_file(pending);
        result
    }
}

/// Immutable witness sent to the transport's status helper, never the Lua state.
pub(crate) struct Integrity {
    paths: [PathBuf; 2],
    expected: String,
}
impl Integrity {
    pub(crate) fn validate(&self) -> Result<(), String> {
        for p in &self.paths {
            let actual = fs::read(p).map_err(|e| format!("session integrity: {} can't be read ({e}). The easel goes on only from the log it wrote; nothing ran.", p.display()))?;
            if actual != self.expected.as_bytes() {
                return Err(format!(
                    "session integrity: {} differs from the log the session wrote. The easel goes on only from the log it wrote; nothing ran.",
                    p.display()
                ));
            }
        }
        Ok(())
    }
}

pub(crate) struct PersistedSession {
    files: SessionFiles,
    s: Session,
    written: String,
    replayed: usize,
}
impl PersistedSession {
    pub(crate) fn session(&self) -> &Session {
        &self.s
    }
    pub(crate) fn name(&self) -> &str {
        &self.files.name
    }
    pub(crate) fn directory(&self) -> &Path {
        &self.files.directory
    }
    pub(crate) fn dir(&self) -> PathBuf {
        self.files.dir()
    }
    pub(crate) fn log_path(&self) -> PathBuf {
        self.files.log_path()
    }
    pub(crate) fn integrity(&self) -> Integrity {
        self.files.integrity(&self.written)
    }
    pub(crate) fn ready(&self) -> Result<(), String> {
        self.integrity().validate()?;
        if self.written != self.s.program(self.name()) {
            return Err("session integrity: uncommitted state; restart required".into());
        }
        Ok(())
    }
    pub(crate) fn into_files(self) -> SessionFiles {
        self.files
    }

    pub(crate) fn run(&mut self, src: &str) -> Result<Ran, String> {
        self.ready()?;
        if self.s.stale {
            self.rebuild_with_progress(|_, _| {})?;
        }
        let result = self.s.run_committing(src, |s| {
            self.files.append(&self.written, &s.program(&self.files.name)).map_err(|e| {
                format!("session integrity: the chunk's log update couldn't be saved ({e}); the chunk was not kept")
            })
        });
        match result {
            Ok(ran) => {
                self.written = self.s.program(self.name());
                Ok(ran)
            }
            Err(e) if self.s.unrestored => Err(format!(
                "{e}\n(the chunk failed and is not in the log; status reports the rebuild's progress and other commands must retry after it)"
            )),
            Err(e) if self.s.stale => Err(format!(
                "{e}\n(the chunk failed and changed nothing. It had changed tables from earlier chunks, and though what they hold is back, how they are laid out (which decides the order `pairs` walks them in, from engine 3 only the length `#` finds in a table with holes) can't be put back, so the easel now rebuilds the painting from its log, as a reopen does; status reports progress and other commands must retry after that)"
            )),
            Err(e) => Err(format!("{e}\n(the chunk failed and changed nothing)")),
        }
    }

    pub(crate) fn rebuild_with_progress(
        &mut self,
        progress: impl FnMut(usize, usize),
    ) -> Result<(), String> {
        self.ready()?;
        self.s.rebuild_with_progress(progress)?;
        self.replayed = self.s.log.len();
        Ok(())
    }

    pub(crate) fn close(&self, build: u64) -> Result<String, String> {
        self.files.append(&self.written, &self.written)?;
        let live = self
            .save_live(build)
            .unwrap_or_else(|e| format!("the live canvas couldn't be saved: {e}\n"));
        Ok(format!(
            "{live}closed; the session is in {}\n",
            self.log_path().display()
        ))
    }

    pub(crate) fn save_live(&self, build: u64) -> Result<String, String> {
        self.ready()?;
        let dir = self.dir();
        let (png, txt, ckpt) = (
            dir.join("live.png"),
            dir.join("live.txt"),
            dir.join("live.ckpt"),
        );
        let _ = fs::remove_file(&txt);
        let Some(c) = self.s.canvas() else {
            let _ = fs::remove_file(&png);
            let _ = fs::remove_file(&ckpt);
            return Ok(String::new());
        };
        crate::deliver(&c, &png)?;
        drop(c);
        if let Err(e) = save::write(&self.s, &self.written, &ckpt) {
            let _ = fs::remove_file(&ckpt);
            eprintln!("the save file couldn't be written: {e}");
        }
        fs::write(
            &txt,
            format!(
                "chunks {}\nreplayed {}\nbuild {:016x}\n",
                self.s.log.len(),
                self.replayed,
                build
            ),
        )
        .map_err(|e| format!("{}: {e}", txt.display()))?;
        Ok(format!("the live canvas is in {}\n", png.display()))
    }
}

#[cfg(all(test, feature = "replay"))]
#[path = "persisted_session_tests.rs"]
mod tests;
