//! Running a painting from the command line: options, stages (timing,
//! stopping early to look, checkpoints to resume from), crop renders, and
//! the finishing recipe (varnish, cracks, raking light) paintings share.
//!
//!   cargo paint <name>                       1000px preview → out/<name>.png
//!   cargo paint <name> -- --full             3200px         → out/<name>_full.png
//!   cargo paint <name> -- --width 1600 --seed 7 --out path.png
//!   cargo paint <name> -- --full --crop 600,380,760,500
//!                                            only that window (units), at full
//!                                            resolution → out/<name>_full_crop.png
//!   cargo paint <name> -- --stop sky         save right after the "sky" stage
//!   cargo paint <name> -- --ckpt             save a checkpoint after every stage
//!   cargo paint <name> -- --resume mist      start from the "mist" checkpoint
//!   cargo paint <name> -- --no-cracks        skip craquelure in the finish
//!
//! A painting is written as stages; each stage's painting goes inside its
//! block, so a resumed run can skip it:
//!
//! ```ignore
//! let o = Run::new("my_painting");
//! let mut rng = Rng::new(o.seed);
//! let mut c = o.canvas(|| st.prepare(o.width, 1.4, o.seed));
//! let sky = Mask::from_fn(c.frame(), ...);        // outside stages: always runs
//! if o.stage("sky", &mut c, &mut rng) {
//!     c.work(&sky, ...);                          // skipped when resuming later
//! }
//! if o.stage("land", &mut c, &mut rng) { ... }
//! o.finish(&mut c, &mut rng, &Finish::aged(st.relief));
//! ```
//!
//! Rules for stages: paint only inside stage blocks (code between them runs
//! on every run, resumed or not, so keep it to masks, fields and constants);
//! state that a stage hands to later ones travels in the canvas or in the
//! `Keep` value passed to `stage` (usually the painting's `Rng`; pass the
//! same one to `end` or `finish`, which close the last stage).
//!
//! Checkpoints (`--ckpt`) go to `out/<stem>.<stage>.ckpt` (stem of the output
//! file). One holds the whole canvas state after its stage, wet paint
//! included, plus the `Keep` state, the width, seed and crop, and hashes of
//! the code that produced it: the painting's source up to the end of that
//! stage, the helpers in `paintings/src` and the engine. `--resume` refuses a
//! checkpoint whose code has changed since (pass `--stale-ok` to use it
//! anyway); changes after the stage are what resuming is for. "Up to the end
//! of the stage" is the source before the call that ends it (the next
//! `stage`, `end` or `finish`) when that call comes later in the same file;
//! otherwise (a `stage` call in a loop, which ends itself on the next
//! iteration, or calls in different files) the stage's body can't be told
//! apart by position, and the whole of both files is hashed: any edit to
//! the painting then makes that checkpoint stale. Helper functions defined
//! below `main` in the painting file are not covered by the prefix hash.

mod source;

use paint::{Canvas, Cracks, Crop, Fbm, Pigment, Rgb, Rng, hex};
use std::cell::RefCell;
use std::panic::Location;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// State a painting carries from stage to stage besides the canvas (saved in
/// checkpoints).
pub trait Keep {
    fn keep(&self) -> Vec<u8>;
    /// Restore from bytes `keep` wrote; an error (leaving `self` as it was)
    /// if they can't be its.
    fn restore(&mut self, b: &[u8]) -> Result<(), String>;
}

impl Keep for () {
    fn keep(&self) -> Vec<u8> {
        Vec::new()
    }
    fn restore(&mut self, b: &[u8]) -> Result<(), String> {
        if b.is_empty() { Ok(()) } else { Err(format!("{} bytes of state where none was kept", b.len())) }
    }
}

impl Keep for Rng {
    fn keep(&self) -> Vec<u8> {
        self.state().to_le_bytes().to_vec()
    }
    fn restore(&mut self, b: &[u8]) -> Result<(), String> {
        let b: [u8; 8] = b.try_into().map_err(|_| format!("{} bytes of state for an Rng (want 8)", b.len()))?;
        *self = Rng::from_state(u64::from_le_bytes(b));
        Ok(())
    }
}

impl<A: Keep, B: Keep> Keep for (A, B) {
    fn keep(&self) -> Vec<u8> {
        let a = self.0.keep();
        let mut v = (a.len() as u64).to_le_bytes().to_vec();
        v.extend(a);
        v.extend(self.1.keep());
        v
    }
    fn restore(&mut self, b: &[u8]) -> Result<(), String> {
        let (n, rest) = b.split_first_chunk::<8>().ok_or("state too short for a pair")?;
        let n = usize::try_from(u64::from_le_bytes(*n)).ok().filter(|&n| n <= rest.len()).ok_or("pair state is malformed")?;
        let (a, b) = rest.split_at(n);
        // restore both or neither
        let old = self.0.keep();
        self.0.restore(a)?;
        self.1.restore(b).inspect_err(|_| {
            let _ = self.0.restore(&old);
        })
    }
}

pub struct Run {
    pub width: usize,
    pub seed: u64,
    pub out: PathBuf,
    /// out/<name>[_full][_crop]: checkpoints are named after it.
    stem: PathBuf,
    /// The crop window (units) and its margin, if this is a crop render.
    pub crop: Option<Crop>,
    name: String,
    /// The painting's source file (where `Run::new` was called), if known.
    file: Option<&'static str>,
    stop: Option<String>,
    no_cracks: bool,
    ckpt: bool,
    resume: Option<String>,
    stale_ok: bool,
    t0: Instant,
    st: RefCell<Stages>,
}

#[derive(Default)]
struct Stages {
    /// The stage in progress, whether it is being painted (false: skipped
    /// on the way to the resume point) and where it began.
    current: Option<(String, bool, &'static Location<'static>)>,
    t_prev: f32,
    /// A loaded checkpoint not yet reached: its stage (as a `key`), header
    /// and file.
    pending: Option<(String, Header, PathBuf)>,
    names: Vec<String>,
}

/// Checkpoint header: key=value lines.
struct Header(Vec<(String, String)>);

impl Header {
    fn get(&self, k: &str) -> &str {
        self.0.iter().find(|(a, _)| a == k).map_or("", |(_, v)| v.as_str())
    }
    fn text(&self) -> String {
        self.0.iter().map(|(k, v)| format!("{k}={v}\n")).collect()
    }
    fn parse(s: &str) -> Self {
        Header(s.lines().filter_map(|l| l.split_once('=')).map(|(k, v)| (k.to_string(), v.to_string())).collect())
    }
}

fn die(msg: &str) -> ! {
    if cfg!(test) {
        panic!("error: {msg}");
    }
    eprintln!("error: {msg}");
    std::process::exit(2)
}

/// `--stop` reached: the image is saved.
fn stop_here() -> ! {
    if cfg!(test) {
        panic!("stopped");
    }
    std::process::exit(0)
}

fn fnv(h: &mut u64, bytes: &[u8]) {
    for &b in bytes {
        *h ^= b as u64;
        *h = h.wrapping_mul(0x100_0000_01b3);
    }
}

/// Hash of the Rust sources in `dir` and its subdirectories (except
/// `bin`: the paintings themselves are hashed stage by stage), sorted by path.
fn hash_dir(dir: &Path) -> String {
    fn walk(d: &Path, out: &mut Vec<PathBuf>) {
        for e in std::fs::read_dir(d).into_iter().flatten().flatten() {
            let p = e.path();
            if p.is_dir() {
                if p.file_name().is_some_and(|n| n != "bin") {
                    walk(&p, out);
                }
            } else if p.extension().is_some_and(|e| e == "rs") {
                out.push(p);
            }
        }
    }
    let mut files = Vec::new();
    walk(dir, &mut files);
    files.sort();
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for f in files {
        fnv(&mut h, f.strip_prefix(dir).unwrap_or(&f).as_os_str().as_encoded_bytes());
        fnv(&mut h, &std::fs::read(&f).unwrap_or_default());
    }
    format!("{h:016x}")
}

fn root() -> PathBuf {
    let r = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    r.canonicalize().unwrap_or(r)
}

/// Stage names as they appear in file names.
fn slug(stage: &str) -> String {
    stage.replace([' ', '/'], "_")
}

/// Stage names as they are compared (`--stop`, `--resume`, tags, file
/// names): case, spaces, underscores, hyphens and slashes don't matter, so
/// `--stop far_range`, `--stop "Far Range"` and `far-range` all mean the
/// stage "far range".
pub fn key(stage: &str) -> String {
    stage.trim().to_lowercase().replace([' ', '/', '-'], "_")
}

fn read_source(file: &str) -> String {
    std::fs::read_to_string(root().join(file)).or_else(|_| std::fs::read_to_string(file)).unwrap_or_default()
}

/// The fallback fingerprint, for a stage call not followed by its block:
/// the source lines before `end` (the call that ends the stage) if it comes
/// later in the same file; otherwise the position of the ending call says
/// nothing about what the body was, so both files are hashed whole.
fn hash_src_with(start: (&str, u32), end: (&str, u32), read: impl Fn(&str) -> String) -> String {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    if start.0 == end.0 && end.1 > start.1 {
        for l in read(end.0).lines().take(end.1 as usize - 1) {
            fnv(&mut h, l.as_bytes());
            fnv(&mut h, b"\n");
        }
        return format!("{h:016x}");
    }
    fnv(&mut h, b"whole\n");
    for f in [start.0, end.0] {
        fnv(&mut h, f.as_bytes());
        fnv(&mut h, read(f).as_bytes());
    }
    format!("w{h:016x}")
}

/// Fingerprint of the painting code a stage's checkpoint depends on (the
/// staleness model; see the module docs and notes/workflow.md). For a stage
/// written as `if o.stage(..) { body }`:
/// - the source up to the closing brace of its own block (everything that
///   ran before the stage ended, minus the code between blocks that comes
///   after it: that is setup for later stages);
/// - the source after the item (`fn main`) the stage is in: helper
///   functions below `main`;
/// - in both, lines tagged `// ckpt: from <later stage>` (or regions
///   between `// ckpt: from <later stage>` and `// ckpt: end`) are left out:
///   the painter's word that they only matter from that stage on.
///
/// `before` holds the keys of the stages up to and including this one.
/// A stage call not followed by its block falls back to `hash_src_with`.
fn hash_stage(start: (&str, u32, u32), end: (&str, u32), before: &[String], read: impl Fn(&str) -> String) -> Result<String, String> {
    let text = read(start.0);
    let Some((blk, item)) = source::stage_block(&text, start.1, start.2) else {
        return Ok(hash_src_with((start.0, start.1), end, read));
    };
    let known: Vec<String> = source::declared_stages(&text).0.iter().map(|n| key(n)).collect();
    let lines: Vec<(&str, Option<source::Tag>)> = text.split('\n').zip(source::tags(&text)).collect();
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    fnv(&mut h, b"stage block v2\n");
    let n = lines.len();
    for part in [&lines[..(blk + 1).min(n)], &lines[(item + 1).min(n)..]] {
        for l in source::tagged_lines(part, before, &known, key)? {
            fnv(&mut h, l.as_bytes());
            fnv(&mut h, b"\n");
        }
        fnv(&mut h, b"-- after the item --\n");
    }
    Ok(format!("b{h:016x}"))
}

fn crop_text(c: &Option<Crop>) -> String {
    match c {
        None => "none".into(),
        Some(c) => format!("{},{},{},{} margin {}", c.units[0], c.units[1], c.units[2], c.units[3], c.margin),
    }
}

impl Run {
    /// Options from the command line. Call it from the painting's own file:
    /// it reads that file's stage names to check `--stop` and `--resume`.
    #[track_caller]
    pub fn new(name: &str) -> Self {
        Self::from_args(name, std::env::args().collect(), Some(Location::caller().file()))
    }

    fn from_args(name: &str, args: Vec<String>, file: Option<&'static str>) -> Self {
        let get = |flag: &str| args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1));
        let has = |flag: &str| args.iter().any(|a| a == flag);
        let full = has("--full");
        let crop = get("--crop").map(|s| {
            let v: Vec<f32> = s.split(',').map(|t| t.trim().parse().unwrap_or_else(|_| die(&format!("--crop {s}: want x0,y0,x1,y1 in units")))).collect();
            if v.len() != 4 || v[2] <= v[0] || v[3] <= v[1] {
                die(&format!("--crop {s}: want x0,y0,x1,y1 in units with x1 > x0, y1 > y0"));
            }
            let margin = get("--margin").and_then(|m| m.parse().ok()).unwrap_or(DEFAULT_MARGIN);
            Crop { units: [v[0], v[1], v[2], v[3]], margin }
        });
        paint::set_crop(crop);
        let root = root().join("out");
        let stem = format!("{name}{}{}", if full { "_full" } else { "" }, if crop.is_some() { "_crop" } else { "" });
        let out = match get("--out") {
            Some(p) => p.into(),
            None => root.join(format!("{stem}.png")),
        };
        let stem = root.join(stem);
        let o = Run {
            width: get("--width").and_then(|s| s.parse().ok()).unwrap_or(if full { 3200 } else { 1000 }),
            seed: get("--seed").and_then(|s| s.parse().ok()).unwrap_or(1),
            out,
            stem,
            crop,
            name: name.to_string(),
            file,
            stop: get("--stop").cloned(),
            no_cracks: has("--no-cracks"),
            ckpt: has("--ckpt"),
            resume: get("--resume").cloned(),
            stale_ok: has("--stale-ok"),
            t0: Instant::now(),
            st: RefCell::new(Stages::default()),
        };
        if let Some(c) = &o.crop {
            eprintln!("crop {} (units) at {}px", crop_text(&Some(*c)), o.width);
        }
        o.check_stage_flags();
        o
    }

    /// The stages the painting's source declares with literal names, and
    /// whether some are named another way (then the list is incomplete).
    fn declared(&self) -> (Vec<String>, bool) {
        match self.file.map(read_source).filter(|s| !s.is_empty()) {
            Some(src) => source::declared_stages(&src),
            None => (Vec::new(), true),
        }
    }

    /// An unknown `--stop` or `--resume` stage is an error before anything
    /// is painted (when the painting names all its stages literally;
    /// otherwise it is caught when the run ends).
    fn check_stage_flags(&self) {
        let (names, dynamic) = self.declared();
        if dynamic {
            return;
        }
        for (flag, v) in [("--stop", &self.stop), ("--resume", &self.resume)] {
            if let Some(v) = v
                && !names.iter().any(|n| key(n) == key(v))
            {
                die(&format!("{flag} {v}: this painting has no stage by that name (stages: {})", list(&names)));
            }
        }
    }

    fn stops_at(&self, stage: &str) -> bool {
        self.stop.as_deref().is_some_and(|s| key(s) == key(stage))
    }

    /// The checkpoint file for `--resume stage`: `ckpt_path`, or the one
    /// whose stage matches by `key` (`--resume "Far Range"` finds
    /// `.far_range.ckpt`).
    fn find_ckpt(&self, stage: &str) -> PathBuf {
        let exact = self.ckpt_path(stage);
        if exact.exists() {
            return exact;
        }
        let stem = self.stem.file_name().unwrap().to_string_lossy().to_string();
        let dir = self.stem.parent().map(Path::to_path_buf).unwrap_or_default();
        let mut have: Vec<(String, PathBuf)> = std::fs::read_dir(&dir)
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|e| {
                let n = e.file_name().to_string_lossy().to_string();
                let st = n.strip_prefix(&format!("{stem}."))?.strip_suffix(".ckpt")?.to_string();
                Some((st, e.path()))
            })
            .collect();
        have.sort();
        if let Some((_, p)) = have.iter().find(|(s, _)| key(s) == key(stage)) {
            return p.clone();
        }
        let saved = if have.is_empty() { "none".to_string() } else { have.iter().map(|(s, _)| s.as_str()).collect::<Vec<_>>().join(", ") };
        die(&format!("--resume {stage}: no checkpoint {} (saved for this run: {saved}); run once with --ckpt first", exact.display()))
    }

    /// Where the checkpoint after `stage` lives: out/<stem>.<stage>.ckpt
    /// (whatever `--out` says).
    pub fn ckpt_path(&self, stage: &str) -> PathBuf {
        let stem = self.stem.file_name().unwrap().to_string_lossy().to_string();
        self.stem.with_file_name(format!("{stem}.{}.ckpt", slug(stage)))
    }

    /// The canvas: made by `make` (a fresh, primed one), or on `--resume`
    /// loaded from the checkpoint (then `make` doesn't run).
    pub fn canvas(&self, make: impl FnOnce() -> Canvas) -> Canvas {
        let Some(stage) = &self.resume else { return make() };
        let path = self.find_ckpt(stage);
        let file = std::fs::File::open(&path).unwrap_or_else(|e| die(&format!("--resume {stage}: can't open {} ({e}); run once with --ckpt first", path.display())));
        let (c, text) = Canvas::read_state(&mut std::io::BufReader::new(file)).unwrap_or_else(|e| die(&format!("{}: {e}", path.display())));
        let h = Header::parse(&text);
        // a checkpoint is only this run's if it was painted the same way
        for (k, want) in [("name", self.name.clone()), ("width", self.width.to_string()), ("seed", self.seed.to_string()), ("crop", crop_text(&self.crop))] {
            if h.get(k) != want {
                die(&format!("{}: made with {k} = {}, this run has {want}", path.display(), h.get(k)));
            }
        }
        self.st.borrow_mut().pending = Some((key(stage), h, path));
        c
    }

    /// Begin the stage `name` (and end the one before it). Returns true if
    /// the stage should be painted, false if a resumed run skips it (it is
    /// in the checkpoint). Ending a stage reports its time, writes its
    /// checkpoint (`--ckpt`) and stops the run if asked (`--stop`).
    #[track_caller]
    pub fn stage(&self, name: &str, c: &mut Canvas, state: &mut dyn Keep) -> bool {
        let loc = Location::caller();
        self.end_stage(c, state, loc);
        let mut st = self.st.borrow_mut();
        if let Some(n) = st.names.iter().find(|n| key(n) == key(name)) {
            die(&format!("two stages are named \"{name}\"{}", if n == name { String::new() } else { format!(" (and \"{n}\", the same to --stop and --resume)") }));
        }
        st.names.push(name.to_string());
        let paint = st.pending.is_none();
        st.current = Some((name.to_string(), paint, loc));
        paint
    }

    fn end_stage(&self, c: &mut Canvas, state: &mut dyn Keep, loc: &Location) {
        let Some((name, painted, start)) = self.st.borrow_mut().current.take() else { return };
        let before: Vec<String> = self.st.borrow().names.iter().map(|n| key(n)).collect();
        let src = hash_stage((start.file(), start.line(), start.column()), (loc.file(), loc.line()), &before, read_source).unwrap_or_else(|e| die(&format!("{}: {e}", start.file())));
        let t = self.t0.elapsed().as_secs_f32();
        if !painted {
            let pending = self.st.borrow_mut().pending.take();
            let Some((target, h, path)) = pending else { unreachable!() };
            if target != key(&name) {
                if self.stops_at(&name) {
                    die(&format!("--stop {}: stage \"{name}\" comes before the resume point \"{}\", so there is nothing to paint up to it", self.stop.as_deref().unwrap_or(""), h.get("stage")));
                }
                eprintln!("  {name:<10}   (in checkpoint)");
                self.st.borrow_mut().pending = Some((target, h, path));
                return;
            }
            // the resume point: the code that painted the checkpoint must be
            // the code we have now
            let mut stale = Vec::new();
            if h.get("src") != src {
                stale.push(format!("the painting's code for stage \"{name}\" (its block and everything before it)"));
            }
            if h.get("lib") != hash_dir(&root().join("paintings/src")) {
                stale.push("paintings/src helpers".into());
            }
            if h.get("engine") != hash_dir(&root().join("crates/paint/src")) {
                stale.push("the engine (crates/paint/src)".into());
            }
            let bytes = unhex(h.get("state")).unwrap_or_else(|| die(&format!("checkpoint \"{name}\": its saved state is not hex")));
            if !stale.is_empty() {
                let msg = format!("checkpoint \"{name}\" is stale: {} changed since it was saved", stale.join(", "));
                if !self.stale_ok {
                    die(&format!("{msg}. Re-run with --ckpt to refresh it, or pass --stale-ok to use it anyway (with --ckpt too, that adopts it for the current code)."));
                }
                if self.ckpt {
                    // the painter vouches for it: adopt it for the current
                    // code, so the next resume needs no --stale-ok
                    let fresh = self.header(&name, src.clone(), &bytes);
                    let fresh = Header(fresh.0.into_iter().chain([("adopted".to_string(), h.get("saved").to_string())]).collect());
                    self.write_ckpt(c, &path, &fresh.text()).unwrap_or_else(|e| die(&format!("refreshing {}: {e}", path.display())));
                    eprintln!("warning: {msg}; using it anyway (--stale-ok) and refreshed it for the current code (--ckpt)");
                } else {
                    eprintln!("warning: {msg}; using it anyway (--stale-ok). Not refreshed: add --ckpt to adopt it for the current code");
                }
            }
            if let Err(e) = state.restore(&bytes) {
                die(&format!("checkpoint \"{name}\": can't restore the painting's state ({e}). Re-run with --ckpt to refresh it."));
            }
            let age = h.get("saved").parse::<u64>().ok().and_then(|s| std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).ok().map(|n| n.as_secs().saturating_sub(s)));
            eprintln!("  {name:<10}   (in checkpoint) resumed{}", age.map_or(String::new(), |a| format!(", saved {} ago", ago(a))));
            self.st.borrow_mut().t_prev = t;
            if self.stops_at(&name) {
                eprintln!("  stopped at \"{name}\", the stage resumed from: nothing painted, this is its checkpoint as saved");
                self.save(c);
                stop_here();
            }
            return;
        }
        let dt = t - self.st.borrow().t_prev;
        self.st.borrow_mut().t_prev = t;
        eprintln!("  {name:<10} {t:>7.2}s  (+{dt:.2}s)");
        if self.ckpt {
            let h = self.header(&name, src, &state.keep());
            let path = self.ckpt_path(&name);
            let t1 = Instant::now();
            self.write_ckpt(c, &path, &h.text()).unwrap_or_else(|e| die(&format!("writing {}: {e}", path.display())));
            eprintln!("  {:<10}   checkpoint {} ({:.0} MB, {:.2}s)", "", path.display(), std::fs::metadata(&path).map_or(0.0, |m| m.len() as f64 / 1e6), t1.elapsed().as_secs_f32());
        }
        if self.stops_at(&name) {
            self.save(c);
            stop_here();
        }
    }

    fn header(&self, stage: &str, src: String, state: &[u8]) -> Header {
        Header(vec![
            ("name".into(), self.name.clone()),
            ("stage".into(), stage.to_string()),
            ("width".into(), self.width.to_string()),
            ("seed".into(), self.seed.to_string()),
            ("crop".into(), crop_text(&self.crop)),
            ("src".into(), src),
            ("lib".into(), hash_dir(&root().join("paintings/src"))),
            ("engine".into(), hash_dir(&root().join("crates/paint/src"))),
            ("state".into(), hexs(state)),
            ("saved".into(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs()).to_string()),
        ])
    }

    fn write_ckpt(&self, c: &Canvas, path: &Path, header: &str) -> std::io::Result<()> {
        if let Some(d) = path.parent() {
            std::fs::create_dir_all(d)?;
        }
        let tmp = path.with_extension("ckpt.part");
        {
            let mut w = std::io::BufWriter::with_capacity(1 << 20, std::fs::File::create(&tmp)?);
            c.write_state(&mut w, header)?;
            use std::io::Write;
            w.flush()?;
        }
        std::fs::rename(&tmp, path)
    }

    /// End the last stage (for programs that finish without `finish`):
    /// its checkpoint (with `state`, the `Keep` value the stages share),
    /// `--stop`, and the check that `--resume` found its stage.
    #[track_caller]
    pub fn end(&self, c: &mut Canvas, state: &mut dyn Keep) {
        self.end_at(c, state, Location::caller());
    }

    fn end_at(&self, c: &mut Canvas, state: &mut dyn Keep, loc: &Location) {
        self.end_stage(c, state, loc);
        let names = list(&self.st.borrow().names);
        if self.st.borrow().pending.is_some() {
            die(&format!("--resume {}: this painting has no stage by that name (stages: {names})", self.resume.as_deref().unwrap_or("")));
        }
        // a --stop that matched has exited by now
        if let Some(s) = &self.stop {
            die(&format!("--stop {s}: this painting has no stage by that name (stages: {names})"));
        }
    }

    /// Finish the painting (see `Finish`) and save it. Ends the last stage
    /// (see `end`).
    #[track_caller]
    pub fn finish(&self, c: &mut Canvas, state: &mut dyn Keep, f: &Finish) {
        let loc = Location::caller();
        self.end_at(c, state, loc);
        self.st.borrow_mut().current = Some(("finish".into(), true, loc));
        c.dry();
        let var = Fbm::new(self.seed as u32 + 98, 3, 400.0);
        let (base, vary) = (f.varnish_coats, f.varnish_vary);
        c.glaze(&Pigment::varnish(f.varnish), None, |x, y| base + vary * var.get(x, y));
        if let (Some(k), false) = (f.cracks, self.no_cracks) {
            c.crack(&Cracks { seed: k.seed.wrapping_add(self.seed), ..k });
        }
        c.relief(f.relief.0, f.relief.1);
        let t = self.t0.elapsed().as_secs_f32();
        eprintln!("  {:<10} {t:>7.2}s  (+{:.2}s)", "finish", t - self.st.borrow().t_prev);
        self.save(c);
    }

    pub fn save(&self, c: &mut Canvas) {
        c.save(&self.out).unwrap();
        let p = self.out.canonicalize().unwrap_or(self.out.clone());
        let win = c.window();
        let what = if self.crop.is_some() { format!("{}px wide canvas, crop {}", self.width, crop_text(&self.crop)) } else { format!("{}px", win.w) };
        eprintln!("wrote {} ({what}) in {:.1}s", p.display(), self.t0.elapsed().as_secs_f32());
    }
}

/// Default margin around a crop window, units: painted but not saved. It
/// gives paint leveling, the brushes' feel for the surface and strokes
/// entering the window their context (see notes/workflow.md for how the
/// difference to a whole render falls with it).
pub const DEFAULT_MARGIN: f32 = 40.0;

fn list(names: &[String]) -> String {
    if names.is_empty() { "none".into() } else { names.iter().map(|n| format!("\"{n}\"")).collect::<Vec<_>>().join(", ") }
}

fn ago(s: u64) -> String {
    match s {
        0..=119 => format!("{s}s"),
        120..=7199 => format!("{}min", s / 60),
        _ => format!("{}h", s / 3600),
    }
}

fn hexs(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn unhex(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 || !s.is_ascii() {
        return None;
    }
    (0..s.len() / 2).map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).ok()).collect()
}

/// How a painting is finished: aged varnish (a clear, yellowing film,
/// uneven), craquelure, then the surface lit by raking light.
pub struct Finish {
    pub varnish: Rgb,
    /// Varnish thickness in coats, and how much it varies across the canvas.
    pub varnish_coats: f32,
    pub varnish_vary: f32,
    /// Craquelure, if the surface has cracked (its seed is offset by the
    /// run's seed).
    pub cracks: Option<Cracks>,
    /// Relief lighting strength and gloss.
    pub relief: (f32, f32),
}

impl Finish {
    /// An old varnished painting.
    pub fn aged(relief: (f32, f32)) -> Self {
        Finish { varnish: hex("#e6d3a4"), varnish_coats: 0.4, varnish_vary: 0.12, cracks: Some(Cracks::aged(0)), relief }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A stage in a loop is ended by its own call on the next iteration:
    /// its fingerprint must cover its body (review finding: editing only the
    /// loop body left the first iteration's checkpoint "fresh").
    #[test]
    fn loop_stages_hash_their_body() {
        let src = |body: &str| format!("fn main() {{\n    for name in [\"first\", \"last\"] {{\n        if o.stage(name, &mut c, &mut ()) {{\n            {body}\n        }}\n    }}\n    o.end(&mut c, &mut ());\n}}\n");
        let (a, b) = (src("c.apply(|_, _, _| [0.2; 3]);"), src("c.apply(|_, _, p| [p[0] + 0.3; 3]);"));
        let hash = |text: &str, start: u32, end: u32| hash_src_with(("p.rs", start), ("p.rs", end), |_| text.to_string());
        // "first" begins and ends at line 3; "last" ends at `end` on line 7
        assert_ne!(hash(&a, 3, 3), hash(&b, 3, 3));
        assert_ne!(hash(&a, 3, 7), hash(&b, 3, 7));
        // straight-line stages still only hash what comes before their end
        let c = format!("{a}// a later stage\n");
        assert_eq!(hash(&a, 3, 7), hash(&c, 3, 7));
        // ... and a stage ending in another file hashes both whole
        let two = |x: &str| hash_src_with(("p.rs", 3), ("q.rs", 1), |f| if f == "p.rs" { x.to_string() } else { String::new() });
        assert_ne!(two(&a), two(&b));
    }

    #[test]
    fn keep_restore_checks_lengths() {
        let mut r = Rng::new(3);
        r.next_u64();
        let mut q = Rng::new(9);
        q.restore(&r.keep()).unwrap();
        assert_eq!(q.state(), r.state());
        assert!(q.restore(&[]).is_err());
        assert!(q.restore(&[0; 9]).is_err());
        assert!(().restore(&[1]).is_err());
        let mut pair = (Rng::new(1), Rng::new(2));
        let kept = (r.clone(), Rng::new(5)).keep();
        pair.restore(&kept).unwrap();
        assert_eq!((pair.0.state(), pair.1.state()), (r.state(), Rng::new(5).state()));
        let before = pair.0.state();
        assert!(pair.restore(&kept[..kept.len() - 1]).is_err());
        assert_eq!(pair.0.state(), before, "a failed restore changes nothing");
        let mut long = kept.clone();
        long[..8].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(pair.restore(&long).is_err());
        assert_eq!(unhex("0a1"), None);
        assert_eq!(unhex("zz"), None);
        assert_eq!(unhex("0aff"), Some(vec![10, 255]));
    }

    fn run(dir: &Path, args: &[&str]) -> Run {
        let mut a = vec!["test".to_string(), "--width".into(), "4".into(), "--ckpt".into()];
        a.extend(args.iter().map(|s| s.to_string()));
        let mut o = Run::from_args("keep_test", a, None);
        o.stem = dir.join("keep_test");
        o.out = dir.join("keep_test.png");
        o
    }

    /// `end` (and `finish`) save the last stage's `Keep` state, and resuming
    /// from it restores it, also when a stage has been appended since (it
    /// used to be saved empty, and restoring then panicked).
    #[test]
    fn last_stage_keeps_its_state() {
        let base = std::env::var_os("TMPDIR").map(PathBuf::from).unwrap_or_else(|| root().join("target"));
        let dir = base.join(format!("run_keep_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let painted = {
            let o = run(&dir, &[]);
            let mut rng = Rng::new(o.seed);
            let mut c = o.canvas(|| Canvas::new_window(o.width, 1.0, [0.1; 3], None));
            for name in ["a", "last"] {
                if o.stage(name, &mut c, &mut rng) {
                    rng.next_u64();
                }
            }
            o.end(&mut c, &mut rng);
            rng.state()
        };
        let text = paint::checkpoint::read_header(&mut std::fs::File::open(o_path(&dir, "last")).unwrap()).unwrap();
        let st = Header::parse(&text);
        assert_eq!(st.get("state"), hexs(&painted.to_le_bytes()));
        // resumed (the code positions differ here, hence --stale-ok)
        let o = run(&dir, &["--resume", "last", "--stale-ok"]);
        let mut rng = Rng::new(o.seed);
        let mut c = o.canvas(|| unreachable!());
        for name in ["a", "last"] {
            assert!(!o.stage(name, &mut c, &mut rng));
        }
        o.end(&mut c, &mut rng);
        assert_eq!(rng.state(), painted);
        // a stage appended after the old last one
        let o = run(&dir, &["--resume", "last", "--stale-ok"]);
        let mut rng = Rng::new(o.seed);
        let mut c = o.canvas(|| unreachable!());
        for name in ["a", "last"] {
            assert!(!o.stage(name, &mut c, &mut rng));
        }
        assert!(o.stage("new", &mut c, &mut rng));
        assert_eq!(rng.state(), painted);
    }

    fn o_path(dir: &Path, stage: &str) -> PathBuf {
        dir.join(format!("keep_test.{stage}.ckpt"))
    }

    fn scratch(tag: &str) -> PathBuf {
        let base = std::env::var_os("TMPDIR").map(PathBuf::from).unwrap_or_else(|| root().join("target"));
        let dir = base.join(format!("run_{tag}_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// The panic message of `f` (`die` and `--stop` panic under test).
    fn outcome(f: impl FnOnce()) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
            Ok(()) => "ran to the end".into(),
            Err(e) => e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default(),
        }
    }

    /// Three stages, the middle one with a space in its name.
    fn three(o: &Run) {
        let mut rng = Rng::new(o.seed);
        let mut c = o.canvas(|| Canvas::new_window(o.width, 1.0, [0.1; 3], None));
        for name in ["a", "far range", "last"] {
            if o.stage(name, &mut c, &mut rng) {
                rng.next_u64();
            }
        }
        o.end(&mut c, &mut rng);
    }

    /// The same stages written at another place in the source: its
    /// checkpoints are stale for `three` and the other way round.
    fn three_edited(o: &Run) {
        let mut rng = Rng::new(o.seed);
        let mut c = o.canvas(|| Canvas::new_window(o.width, 1.0, [0.1; 3], None));
        for name in ["a", "far range", "last"] {
            if o.stage(name, &mut c, &mut rng) {
                rng.next_u64();
                // an edit
            }
        }
        o.end(&mut c, &mut rng);
    }

    /// Stage names match by `key`; `--resume X --stop X` stops; a `--stop`
    /// before the resume point or naming no stage is an error (mountains
    /// #5, winter #20).
    #[test]
    fn stop_and_resume_names() {
        assert_eq!(key("Far Range"), "far_range");
        assert_eq!(key("far-range"), key("far_range"));
        let dir = scratch("names");
        assert_eq!(outcome(|| three(&run(&dir, &[]))), "ran to the end");
        assert!(o_path(&dir, "far_range").exists());
        let _ = std::fs::remove_file(dir.join("keep_test.png"));
        assert_eq!(outcome(|| three(&run(&dir, &["--stop", "far_range"]))), "stopped");
        assert!(dir.join("keep_test.png").exists());
        // resumed and stopped at the same stage: saved as it was
        let _ = std::fs::remove_file(dir.join("keep_test.png"));
        assert_eq!(outcome(|| three(&run(&dir, &["--resume", "Far Range", "--stop", "far range"]))), "stopped");
        assert!(dir.join("keep_test.png").exists());
        let e = outcome(|| three(&run(&dir, &["--resume", "far_range", "--stop", "a"])));
        assert!(e.contains("comes before the resume point"), "{e}");
        let e = outcome(|| three(&run(&dir, &["--stop", "far"])));
        assert!(e.contains("--stop far: this painting has no stage") && e.contains("\"far range\""), "{e}");
        let e = outcome(|| three(&run(&dir, &["--resume", "nope"])));
        assert!(e.contains("no checkpoint") && e.contains("far_range"), "{e}");
    }

    /// With the painting's file known, an unknown stage name fails before
    /// anything is painted.
    #[test]
    fn unknown_stage_fails_early() {
        let dir = scratch("early");
        let f = dir.join("p.rs");
        std::fs::write(&f, "fn main() {\n    if o.stage(\"sky\", &mut c, &mut r) {}\n    if o.stage(\"far range\", &mut c, &mut r) {}\n}\n").unwrap();
        let file: &'static str = Box::leak(f.to_string_lossy().to_string().into_boxed_str());
        let args = |a: &[&str]| std::iter::once("t").chain(a.iter().copied()).map(String::from).collect::<Vec<_>>();
        let e = outcome(|| drop(Run::from_args("t", args(&["--stop", "far_rnge"]), Some(file))));
        assert!(e.contains("no stage by that name (stages: \"sky\", \"far range\")"), "{e}");
        assert_eq!(outcome(|| drop(Run::from_args("t", args(&["--stop", "FAR_RANGE"]), Some(file)))), "ran to the end");
        let e = outcome(|| drop(Run::from_args("t", args(&["--resume", "land"]), Some(file))));
        assert!(e.contains("--resume land"), "{e}");
    }

    /// `--stale-ok --ckpt` adopts the checkpoint it resumed from, so the
    /// next resume of the same code needs no `--stale-ok` (mountains #18);
    /// without `--ckpt` it is used, not refreshed.
    #[test]
    fn stale_ok_refreshes_with_ckpt() {
        let dir = scratch("refresh");
        three(&run(&dir, &[]));
        let e = outcome(|| three_edited(&run(&dir, &["--resume", "far range"])));
        assert!(e.contains("is stale"), "{e}");
        let no_ckpt = |args: &[&str]| {
            let mut o = run(&dir, args);
            o.ckpt = false;
            o
        };
        assert_eq!(outcome(|| three_edited(&no_ckpt(&["--resume", "far range", "--stale-ok"]))), "ran to the end");
        assert!(outcome(|| three_edited(&no_ckpt(&["--resume", "far range"]))).contains("is stale"), "not refreshed without --ckpt");
        assert_eq!(outcome(|| three_edited(&run(&dir, &["--resume", "far range", "--stale-ok"]))), "ran to the end");
        assert_eq!(outcome(|| three_edited(&no_ckpt(&["--resume", "far range"]))), "ran to the end");
        let h = Header::parse(&paint::checkpoint::read_header(&mut std::fs::File::open(o_path(&dir, "far_range")).unwrap()).unwrap());
        assert!(!h.get("adopted").is_empty());
    }

    /// The staleness model (winter #2, mountains #4, coast #19): a stage's
    /// fingerprint covers its block and what comes before it, and helpers
    /// after `main`; not the setup for later stages written between blocks,
    /// and not lines tagged for a later stage.
    #[test]
    fn stage_fingerprint_model() {
        let prog = |top: &str, between: &str, body: &str, helper: &str| {
            format!("const TOR: f32 = 1.0;\n{top}\nfn main() {{\n    let sky = 1;\n    if o.stage(\"sky\", &mut c, &mut r) {{\n        {body}\n    }}\n    {between}\n    if o.stage(\"oak\", &mut c, &mut r) {{\n        oak();\n    }}\n    o.end(&mut c, &mut r);\n}}\nfn helper() {{ {helper} }}\n")
        };
        let hash = |text: &str, stage: (u32, &[&str])| {
            let before: Vec<String> = stage.1.iter().map(|s| key(s)).collect();
            hash_stage(("p.rs", stage.0, 8), ("p.rs", stage.0 + 4), &before, |_| text.to_string())
        };
        let sky = (5, &["sky"][..]);
        let oak = (9, &["sky", "oak"][..]);
        let base = prog("", "let skel = grow(1);", "paint(sky);", "a()");
        let h0 = hash(&base, sky).unwrap();
        // geometry for the oak, built between the blocks: sky is unaffected
        let edited = prog("", "let skel = grow(2);", "paint(sky);", "a()");
        assert_eq!(hash(&edited, sky).unwrap(), h0);
        assert_ne!(hash(&edited, oak).unwrap(), hash(&base, oak).unwrap());
        // the sky's own body, and helpers after main, count
        assert_ne!(hash(&prog("", "let skel = grow(1);", "paint(sky * 2);", "a()"), sky).unwrap(), h0);
        assert_ne!(hash(&prog("", "let skel = grow(1);", "paint(sky);", "b()"), sky).unwrap(), h0);
        // a constant at the top counts, unless tagged for a later stage
        let top = |v: &str| prog(&format!("const X: f32 = {v}; // ckpt: from oak"), "", "paint(sky);", "a()");
        assert_eq!(hash(&top("1.0"), sky).unwrap(), hash(&top("2.0"), sky).unwrap());
        assert_ne!(hash(&top("1.0"), oak).unwrap(), hash(&top("2.0"), oak).unwrap());
        let region = |v: &str| prog(&format!("// ckpt: from Oak\nconst X: f32 = {v};\nconst Y: f32 = {v};\n// ckpt: end"), "", "paint(sky);", "a()");
        assert_eq!(hash(&region("1.0"), sky).unwrap(), hash(&region("2.0"), sky).unwrap());
        assert_ne!(hash(&region("1.0"), oak).unwrap(), hash(&region("2.0"), oak).unwrap());
        let untagged = |v: &str| prog(&format!("const X: f32 = {v};"), "", "paint(sky);", "a()");
        assert_ne!(hash(&untagged("1.0"), sky).unwrap(), hash(&untagged("2.0"), sky).unwrap());
        // a tag naming no stage is an error, not a silent hole
        let typo = prog("const X: f32 = 1.0; // ckpt: from oka", "", "paint(sky);", "a()");
        assert!(hash(&typo, sky).unwrap_err().contains("names no stage"));
    }
}
