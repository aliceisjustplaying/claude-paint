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
//! o.finish(&mut c, &Finish::aged(st.relief));
//! ```
//!
//! Rules for stages: paint only inside stage blocks (code between them runs
//! on every run, resumed or not, so keep it to masks, fields and constants);
//! state that a stage hands to later ones travels in the canvas or in the
//! `Keep` value passed to `stage` (usually the painting's `Rng`).
//!
//! Checkpoints (`--ckpt`) go to `out/<stem>.<stage>.ckpt` (stem of the output
//! file). One holds the whole canvas state after its stage, wet paint
//! included, plus the `Keep` state, the width, seed and crop, and hashes of
//! the code that produced it: the painting's source up to the end of that
//! stage, the helpers in `paintings/src` and the engine. `--resume` refuses a
//! checkpoint whose code has changed since (pass `--stale-ok` to use it
//! anyway); changes after the stage are what resuming is for. Helper
//! functions defined below `main` in the painting file are not covered.

use paint::{Canvas, Cracks, Crop, Fbm, Pigment, Rgb, Rng, hex};
use std::cell::RefCell;
use std::panic::Location;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// State a painting carries from stage to stage besides the canvas (saved in
/// checkpoints).
pub trait Keep {
    fn keep(&self) -> Vec<u8>;
    fn restore(&mut self, b: &[u8]);
}

impl Keep for () {
    fn keep(&self) -> Vec<u8> {
        Vec::new()
    }
    fn restore(&mut self, _: &[u8]) {}
}

impl Keep for Rng {
    fn keep(&self) -> Vec<u8> {
        self.state().to_le_bytes().to_vec()
    }
    fn restore(&mut self, b: &[u8]) {
        *self = Rng::from_state(u64::from_le_bytes(b[..8].try_into().expect("rng state")));
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
    fn restore(&mut self, b: &[u8]) {
        let n = u64::from_le_bytes(b[..8].try_into().unwrap()) as usize;
        self.0.restore(&b[8..8 + n]);
        self.1.restore(&b[8 + n..]);
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
    /// The stage in progress and whether it is being painted (false: skipped
    /// on the way to the resume point).
    current: Option<(String, bool)>,
    t_prev: f32,
    /// A loaded checkpoint not yet reached: its stage and header.
    pending: Option<(String, Header)>,
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
    eprintln!("error: {msg}");
    std::process::exit(2)
}

fn fnv(h: &mut u64, bytes: &[u8]) {
    for &b in bytes {
        *h ^= b as u64;
        *h = h.wrapping_mul(0x100_0000_01b3);
    }
}

/// Hash of the Rust sources in `dir` (not recursive), sorted by name.
fn hash_dir(dir: &Path) -> String {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir).map(|d| d.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "rs")).collect()).unwrap_or_default();
    files.sort();
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for f in files {
        fnv(&mut h, f.file_name().unwrap().as_encoded_bytes());
        fnv(&mut h, &std::fs::read(&f).unwrap_or_default());
    }
    format!("{h:016x}")
}

fn root() -> PathBuf {
    let r = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    r.canonicalize().unwrap_or(r)
}

/// Stage names as they appear in file names and on the command line.
fn slug(stage: &str) -> String {
    stage.replace([' ', '/'], "_")
}

/// Hash of a source file's lines before `line` (the code that ran before).
fn hash_prefix(loc: &Location) -> String {
    let p = root().join(loc.file());
    let text = std::fs::read_to_string(&p).or_else(|_| std::fs::read_to_string(loc.file())).unwrap_or_default();
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for l in text.lines().take(loc.line().saturating_sub(1) as usize) {
        fnv(&mut h, l.as_bytes());
        fnv(&mut h, b"\n");
    }
    format!("{h:016x}")
}

fn crop_text(c: &Option<Crop>) -> String {
    match c {
        None => "none".into(),
        Some(c) => format!("{},{},{},{} margin {}", c.units[0], c.units[1], c.units[2], c.units[3], c.margin),
    }
}

impl Run {
    pub fn new(name: &str) -> Self {
        let args: Vec<String> = std::env::args().collect();
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
            stop: get("--stop").cloned(),
            no_cracks: has("--no-cracks"),
            ckpt: has("--ckpt"),
            resume: get("--resume").map(|s| slug(s)),
            stale_ok: has("--stale-ok"),
            t0: Instant::now(),
            st: RefCell::new(Stages::default()),
        };
        if let Some(c) = &o.crop {
            eprintln!("crop {} (units) at {}px", crop_text(&Some(*c)), o.width);
        }
        o
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
        let path = self.ckpt_path(stage);
        let file = std::fs::File::open(&path).unwrap_or_else(|e| die(&format!("--resume {stage}: can't open {} ({e}); run once with --ckpt first", path.display())));
        let (c, text) = Canvas::read_state(&mut std::io::BufReader::new(file)).unwrap_or_else(|e| die(&format!("{}: {e}", path.display())));
        let h = Header::parse(&text);
        // a checkpoint is only this run's if it was painted the same way
        for (k, want) in [("name", self.name.clone()), ("width", self.width.to_string()), ("seed", self.seed.to_string()), ("crop", crop_text(&self.crop))] {
            if h.get(k) != want {
                die(&format!("{}: made with {k} = {}, this run has {want}", path.display(), h.get(k)));
            }
        }
        self.st.borrow_mut().pending = Some((stage.clone(), h));
        c
    }

    /// Begin the stage `name` (and end the one before it). Returns true if
    /// the stage should be painted, false if a resumed run skips it (it is
    /// in the checkpoint). Ending a stage reports its time, writes its
    /// checkpoint (`--ckpt`) and stops the run if asked (`--stop`).
    #[track_caller]
    pub fn stage(&self, name: &str, c: &mut Canvas, state: &mut dyn Keep) -> bool {
        let loc = Location::caller();
        self.end_stage(c, Some(state), loc);
        let mut st = self.st.borrow_mut();
        if st.names.iter().any(|n| n == name) {
            die(&format!("two stages are named \"{name}\""));
        }
        st.names.push(name.to_string());
        let paint = st.pending.is_none();
        st.current = Some((name.to_string(), paint));
        paint
    }

    fn end_stage(&self, c: &mut Canvas, state: Option<&mut dyn Keep>, loc: &Location) {
        let Some((name, painted)) = self.st.borrow_mut().current.take() else { return };
        let t = self.t0.elapsed().as_secs_f32();
        if !painted {
            let pending = self.st.borrow_mut().pending.take();
            let Some((target, h)) = pending else { unreachable!() };
            if target != slug(&name) {
                eprintln!("  {name:<10}   (in checkpoint)");
                self.st.borrow_mut().pending = Some((target, h));
                return;
            }
            // the resume point: the code that painted the checkpoint must be
            // the code we have now
            let mut stale = Vec::new();
            if h.get("src") != hash_prefix(loc) {
                stale.push(format!("the painting's code up to the end of stage \"{name}\""));
            }
            if h.get("lib") != hash_dir(&root().join("paintings/src")) {
                stale.push("paintings/src helpers".into());
            }
            if h.get("engine") != hash_dir(&root().join("crates/paint/src")) {
                stale.push("the engine (crates/paint/src)".into());
            }
            if !stale.is_empty() {
                let msg = format!("checkpoint \"{name}\" is stale: {} changed since it was saved", stale.join(", "));
                if self.stale_ok {
                    eprintln!("warning: {msg}; using it anyway (--stale-ok)");
                } else {
                    die(&format!("{msg}. Re-run with --ckpt to refresh it, or pass --stale-ok to use it anyway."));
                }
            }
            if let Some(s) = state {
                s.restore(&unhex(h.get("state")));
            }
            let age = h.get("saved").parse::<u64>().ok().and_then(|s| std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).ok().map(|n| n.as_secs().saturating_sub(s)));
            eprintln!("  {name:<10}   (in checkpoint) resumed{}", age.map_or(String::new(), |a| format!(", saved {} ago", ago(a))));
            self.st.borrow_mut().t_prev = t;
            return;
        }
        let dt = t - self.st.borrow().t_prev;
        self.st.borrow_mut().t_prev = t;
        eprintln!("  {name:<10} {t:>7.2}s  (+{dt:.2}s)");
        if self.ckpt {
            let h = Header(vec![
                ("name".into(), self.name.clone()),
                ("stage".into(), name.clone()),
                ("width".into(), self.width.to_string()),
                ("seed".into(), self.seed.to_string()),
                ("crop".into(), crop_text(&self.crop)),
                ("src".into(), hash_prefix(loc)),
                ("lib".into(), hash_dir(&root().join("paintings/src"))),
                ("engine".into(), hash_dir(&root().join("crates/paint/src"))),
                ("state".into(), state.map_or(String::new(), |s| hexs(&s.keep()))),
                ("saved".into(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs()).to_string()),
            ]);
            let path = self.ckpt_path(&name);
            let t1 = Instant::now();
            self.write_ckpt(c, &path, &h.text()).unwrap_or_else(|e| die(&format!("writing {}: {e}", path.display())));
            eprintln!("  {:<10}   checkpoint {} ({:.0} MB, {:.2}s)", "", path.display(), std::fs::metadata(&path).map_or(0.0, |m| m.len() as f64 / 1e6), t1.elapsed().as_secs_f32());
        }
        if self.stop.as_deref() == Some(name.as_str()) {
            self.save(c);
            std::process::exit(0);
        }
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

    /// Finish the painting (see `Finish`) and save it. Ends the last stage.
    #[track_caller]
    pub fn finish(&self, c: &mut Canvas, f: &Finish) {
        let loc = Location::caller();
        self.end_stage(c, None, loc);
        if let Some((target, _)) = &self.st.borrow().pending {
            die(&format!("--resume {target}: this painting has no stage by that name (stages: {})", self.st.borrow().names.join(", ")));
        }
        self.st.borrow_mut().current = Some(("finish".into(), true));
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
pub const DEFAULT_MARGIN: f32 = 12.0;

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

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len() / 2).filter_map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).ok()).collect()
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
