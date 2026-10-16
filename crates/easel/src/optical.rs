use image::ImageEncoder;
use paint::render_snapshot::{RenderSnapshot, RENDERER};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn hex(bytes: &[u8]) -> String { bytes.iter().map(|b| format!("{b:02x}")).collect() }

fn id(text: &str) -> io::Result<[u8; 32]> {
    if text.len() != 64 || !text.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
        return Err(invalid("invalid optical identity"));
    }
    let mut bytes = [0; 32];
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&text[i * 2..i * 2 + 2], 16).map_err(|_| invalid("invalid optical identity"))?;
    }
    Ok(bytes)
}

fn runtime() -> io::Result<[u8; 32]> { id(&crate::painting_store::PaintingStore::runtime_identity()?) }

struct Limited<W> { writer: W, left: u64 }

impl<W: Write> Write for Limited<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() as u64 > self.left { return Err(invalid("optical output exceeds its byte budget")); }
        let n = self.writer.write(bytes)?;
        self.left -= n as u64;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> { self.writer.flush() }
}

fn atomic(path: &Path, budget: u64, write: impl FnOnce(&mut Limited<BufWriter<File>>) -> io::Result<()>) -> io::Result<()> {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let dir = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    std::fs::create_dir_all(dir)?;
    let mut opened = None;
    for _ in 0..128 {
        let part = dir.join(format!(".optical-{}-{}.part", std::process::id(), NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)));
        match OpenOptions::new().write(true).create_new(true).open(&part) {
            Ok(file) => { opened = Some((part, file)); break }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
    let (part, file) = opened.ok_or_else(|| invalid("cannot create exclusive optical staging file"))?;
    let result = (|| {
        let mut writer = Limited { writer: BufWriter::with_capacity(65536, file), left: budget };
        write(&mut writer)?;
        writer.flush()?;
        writer.writer.get_ref().sync_all()?;
        drop(writer);
        std::fs::rename(&part, path)?;
        File::open(dir)?.sync_all()
    })();
    if result.is_err() { let _ = std::fs::remove_file(part); }
    result
}

pub fn export(args: &[String]) -> Result<(), String> {
    let result = (|| -> io::Result<()> {
        if args.len() != 2 { return Err(invalid("snapshot-state <store directory> <out.view>")); }
        let dir = Path::new(&args[0]);
        let out = Path::new(&args[1]);
        if out.extension().is_none_or(|extension| extension != "view") {
            return Err(invalid("snapshot output must have the .view extension"));
        }
        if crate::normalized_path(out)?.starts_with(crate::normalized_path(dir)?) {
            return Err(invalid("snapshot output must be outside the committed store"));
        }
        let revision = crate::painting_store::ReadRevision::open(dir.into())?;
        revision.protect_output(out)?;
        let checkpoint = revision.checkpoint()?;
        if crate::same_file(&checkpoint, out).map_err(invalid)? { return Err(invalid("snapshot must not replace its material input")); }
        match File::open(out) {
            Ok(file) => {
                RenderSnapshot::read(&mut BufReader::with_capacity(65536, file))
                    .map_err(|e| invalid(format!("snapshot output already exists and is not a verified snapshot; refusing to replace journal exports or unrelated files: {e}")))?;
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
        let restored = crate::save::read(&checkpoint).map_err(invalid)?;
        let canvas = restored.session.canvas().ok_or_else(|| invalid("committed revision has no canvas"))?;
        let snapshot = RenderSnapshot::capture(&canvas, id(&revision.id)?, runtime()?)?;
        let (w, h) = snapshot.dimensions();
        atomic(out, (w * h) as u64 * 20 + 4096, |writer| snapshot.write(writer))?;
        eprintln!("wrote {} (revision {}, {}×{}, compact optics, no replay)", out.display(), revision.id, w, h);
        Ok(())
    })();
    result.map_err(|e| e.to_string())
}

fn load(path: &Path) -> io::Result<RenderSnapshot> {
    let snapshot = RenderSnapshot::read(&mut BufReader::with_capacity(65536, File::open(path)?))?;
    if snapshot.runtime() != runtime()? { return Err(invalid("snapshot renderer binary differs; use its recorded renderer or export a new snapshot")); }
    Ok(snapshot)
}

fn light(text: &str) -> io::Result<[u32; 3]> {
    let values: Vec<f32> = text.split(',').map(|v| v.parse::<f32>().map_err(|_| invalid("light needs azimuth,elevation,strength"))).collect::<io::Result<_>>()?;
    let [az, el, strength] = values.as_slice() else { return Err(invalid("light needs azimuth,elevation,strength")); };
    if !az.is_finite() || !el.is_finite() || !strength.is_finite() || !(0.0..=360.0).contains(az) || !(0.0..=90.0).contains(el) || !(0.0..=3.0).contains(strength) {
        return Err(invalid("light requires finite azimuth 0–360, elevation 0–90 and strength 0–3"));
    }
    Ok([az.to_bits(), el.to_bits(), strength.to_bits()])
}

fn crop(text: &str) -> io::Result<[usize; 4]> {
    text.split(',').map(|v| v.parse::<usize>().map_err(|_| invalid("crop needs four whole-canvas pixel coordinates"))).collect::<io::Result<Vec<_>>>()?.try_into().map_err(|_| invalid("crop needs four whole-canvas pixel coordinates"))
}

fn rect(snapshot: &RenderSnapshot, wanted: Option<[usize; 4]>) -> io::Result<[usize; 4]> {
    let f = snapshot.frame();
    let keep = snapshot.keep();
    let visible = [f.x0 + keep.0, f.y0 + keep.1, f.x0 + keep.2, f.y0 + keep.3];
    let r = wanted.unwrap_or(visible);
    if r[0] < visible[0] || r[1] < visible[1] || r[2] > visible[2] || r[3] > visible[3] || r[0] >= r[2] || r[1] >= r[3] {
        return Err(invalid("requested crop is not completely covered by the snapshot's visible pixels"));
    }
    Ok(r)
}

fn png_budget(r: [usize; 4]) -> u64 { ((r[2] - r[0]) * (r[3] - r[1])) as u64 * 4 + (1 << 20) }

fn png(snapshot: &RenderSnapshot, r: [usize; 4], light: Option<[u32; 3]>, writer: &mut impl Write) -> io::Result<()> {
    let f = snapshot.frame();
    let lit = light.map(|bits| { let [az, el, strength] = bits.map(f32::from_bits); snapshot.lit(az, el, strength) });
    let pixels = lit.as_deref().unwrap_or(snapshot.color());
    png_pixels(pixels, f, r, writer)
}

fn png_pixels(pixels: &[paint::Rgb], f: paint::Frame, r: [usize; 4], writer: &mut impl Write) -> io::Result<()> {
    let bytes = crate::output::rgb8_exact_region(pixels, f.w, [r[0] - f.x0, r[1] - f.y0, r[2] - f.x0, r[3] - f.y0]);
    image::codecs::png::PngEncoder::new(writer).write_image(&bytes, (r[2] - r[0]) as u32, (r[3] - r[1]) as u32, image::ExtendedColorType::Rgb8).map_err(|e| invalid(e.to_string()))
}

pub fn render(args: &[String]) -> Result<(), String> {
    let result = (|| -> io::Result<()> {
        if args.len() < 2 { return Err(invalid("render-snapshot <snapshot.view> <out.png> [--mode color|gallery|relief] [--light az,el,strength] [--crop x0,y0,x1,y1]")); }
        let input = Path::new(&args[0]);
        let out = Path::new(&args[1]);
        if image::ImageFormat::from_path(out).map_err(|e| invalid(e.to_string()))? != image::ImageFormat::Png {
            return Err(invalid("snapshot output must be a PNG"));
        }
        if crate::same_file(input, out).map_err(invalid)? { return Err(invalid("snapshot output must differ from its input")); }
        let mut mode = "gallery";
        let mut custom = None;
        let mut wanted = None;
        for pair in args[2..].chunks(2) {
            if pair.len() != 2 { return Err(invalid("snapshot option needs a value")); }
            match pair[0].as_str() {
                "--mode" => mode = &pair[1],
                "--light" => custom = Some(light(&pair[1])?),
                "--crop" => wanted = Some(crop(&pair[1])?),
                _ => return Err(invalid("unknown render-snapshot option")),
            }
        }
        let illumination = match mode {
            "color" if custom.is_none() => None,
            "gallery" => Some(custom.unwrap_or(light("115,55,1")?)),
            "relief" => Some(custom.unwrap_or(light("135,25,1")?)),
            _ => return Err(invalid("mode must be color, gallery or relief; color has no light")),
        };
        let snapshot = load(input)?;
        let r = rect(&snapshot, wanted)?;
        atomic(out, png_budget(r), |writer| png(&snapshot, r, illumination, writer))?;
        eprintln!("wrote {} (revision {}, {mode}, compact snapshot, no replay)", out.display(), hex(&snapshot.revision()));
        Ok(())
    })();
    result.map_err(|e| e.to_string())
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    schema: u32,
    snapshot: String,
    revision: String,
    runtime: String,
    renderer: String,
    encoder: String,
    crop: [usize; 4],
    gallery: [u32; 3],
    relief: [u32; 3],
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Png { id: String, bytes: u64 }

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Generation { config: Config, images: [Png; 3], parent: Option<String> }

fn hashed_file(path: &Path, limit: u64) -> io::Result<(String, u64)> {
    let mut reader = File::open(path)?;
    let mut hash = Sha256::new();
    let mut bytes = 0u64;
    let mut buffer = [0; 65536];
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 { break }
        bytes = bytes.checked_add(n as u64).filter(|v| *v <= limit).ok_or_else(|| invalid("cached PNG exceeds its byte budget"))?;
        hash.update(&buffer[..n]);
    }
    Ok((hex(&hash.finalize()), bytes))
}

fn generation(dir: &Path, name: &str) -> io::Result<Generation> {
    id(name)?;
    let mut bytes = Vec::new();
    File::open(dir.join(format!("{name}.json")))?.take(16385).read_to_end(&mut bytes)?;
    if bytes.len() > 16384 || hex(&Sha256::digest(&bytes)) != name { return Err(invalid("optical cache manifest integrity failed")); }
    let value: Generation = serde_json::from_slice(&bytes).map_err(|e| invalid(e.to_string()))?;
    id(&value.config.snapshot)?;
    id(&value.config.revision)?;
    id(&value.config.runtime)?;
    if value.config.schema != 1 || value.config.renderer.is_empty() || value.config.renderer.len() > 64 || !value.config.renderer.is_ascii()
        || value.config.encoder.is_empty() || value.config.encoder.len() > 64 || !value.config.encoder.is_ascii() {
        return Err(invalid("unsupported optical cache metadata"));
    }
    let r = value.config.crop;
    if r[0] >= r[2] || r[1] >= r[3] || r[2] > 9600 || r[3] > 9600 { return Err(invalid("invalid cached crop")); }
    if let Some(parent) = &value.parent { id(parent)?; }
    for png in &value.images {
        id(&png.id)?;
        if png.bytes == 0 || png.bytes > png_budget(r) { return Err(invalid("invalid cached PNG length")); }
        let (digest, bytes) = hashed_file(&dir.join(format!("{}.png", png.id)), png.bytes)?;
        if digest != png.id || bytes != png.bytes { return Err(invalid("cached PNG integrity failed")); }
    }
    Ok(value)
}

fn current(dir: &Path) -> io::Result<Option<String>> {
    let mut bytes = Vec::new();
    File::open(dir.join("CURRENT"))?.take(66).read_to_end(&mut bytes)?;
    if bytes == b"none\n" { return Ok(None); }
    let text = std::str::from_utf8(&bytes).map_err(|_| invalid("invalid optical cache root"))?;
    let text = text.strip_suffix('\n').ok_or_else(|| invalid("invalid optical cache root"))?;
    id(text)?;
    Ok(Some(text.into()))
}

fn collect(dir: &Path, root: Option<&str>) -> io::Result<u64> {
    let mut keep = BTreeSet::from(["writer.lock".into(), "CURRENT".into()]);
    if let Some(root) = root {
        let value = generation(dir, root)?;
        let mut retained = vec![(root.to_string(), value)];
        if let Some(parent) = &retained[0].1.parent { retained.push((parent.clone(), generation(dir, parent)?)); }
        for (name, generation) in retained {
            keep.insert(format!("{name}.json"));
            for png in generation.images { keep.insert(format!("{}.png", png.id)); }
        }
    }
    let mut bytes = 0u64;
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() { return Err(invalid("unexpected non-file in optical cache")); }
        let name = entry.file_name().into_string().map_err(|_| invalid("invalid optical cache filename"))?;
        if keep.contains(&name) { bytes = bytes.checked_add(entry.metadata()?.len()).ok_or_else(|| invalid("cache byte counter overflow"))?; }
        else if name.starts_with(".optical-") && name.ends_with(".part") || name.strip_suffix(".png").or_else(|| name.strip_suffix(".json")).is_some_and(|s| id(s).is_ok()) {
            std::fs::remove_file(entry.path())?;
        } else { return Err(invalid("unrelated file in optical cache; use a dedicated directory")); }
    }
    File::open(dir)?.sync_all()?;
    Ok(bytes)
}

pub fn companions(args: &[String]) -> Result<(), String> {
    let result = (|| -> io::Result<()> {
        if args.len() < 2 { return Err(invalid("snapshot-companions <snapshot.view> <cache directory> [--gallery-light az,el,strength] [--relief-light az,el,strength] [--crop x0,y0,x1,y1] [--cache-budget-mib N]")); }
        let input = Path::new(&args[0]);
        let dir = Path::new(&args[1]);
        if crate::normalized_path(input)?.starts_with(crate::normalized_path(dir)?) { return Err(invalid("snapshot input must be outside its dedicated companion cache")); }
        let mut gallery = light("115,55,1")?;
        let mut relief = light("135,25,1")?;
        let mut wanted = None;
        let mut budget = 512 << 20;
        for pair in args[2..].chunks(2) {
            if pair.len() != 2 { return Err(invalid("companion option needs a value")); }
            match pair[0].as_str() {
                "--gallery-light" => gallery = light(&pair[1])?,
                "--relief-light" => relief = light(&pair[1])?,
                "--crop" => wanted = Some(crop(&pair[1])?),
                "--cache-budget-mib" => budget = pair[1].parse::<u64>().ok().filter(|v| (1..=4096).contains(v)).map(|v| v << 20).ok_or_else(|| invalid("cache budget must be 1–4096 MiB"))?,
                _ => return Err(invalid("unknown snapshot-companions option")),
            }
        }
        let snapshot = load(input)?;
        let r = rect(&snapshot, wanted)?;
        let config = Config { schema: 1, snapshot: hex(&snapshot.content_id()), revision: hex(&snapshot.revision()), runtime: hex(&snapshot.runtime()), renderer: RENDERER.into(), encoder: "image-png-0.25".into(), crop: r, gallery, relief };
        std::fs::create_dir_all(dir)?;
        let lock = OpenOptions::new().read(true).write(true).create(true).truncate(false).open(dir.join("writer.lock"))?;
        lock.try_lock().map_err(|_| invalid("another companion writer owns this cache"))?;
        if !dir.join("CURRENT").exists() {
            if std::fs::read_dir(dir)?.any(|e| e.map_or(true, |e| e.file_name() != "writer.lock")) { return Err(invalid("optical cache lost its root; refusing collection")); }
            atomic(&dir.join("CURRENT"), 65, |w| w.write_all(b"none\n"))?;
        }
        let old = current(dir)?;
        let used = collect(dir, old.as_deref())?;
        if used > budget { return Err(invalid("existing optical cache exceeds its byte budget")); }
        let mut selected = None;
        if let Some(old) = &old {
            if generation(dir, old)?.config == config { selected = Some(old.clone()); }
        }
        if selected.is_none() {
            if used.checked_add(png_budget(r) * 3 + 65536).is_none_or(|v| v > budget) { return Err(invalid("insufficient companion transaction space")); }
            let prepared = snapshot.into_prepared_optics();
            let mut images = Vec::new();
            for illumination in [None, Some(gallery), Some(relief)] {
                let staged = dir.join(format!(".optical-{}-{}.part", std::process::id(), images.len()));
                if staged.exists() { return Err(invalid("companion staging collision")); }
                let lit = illumination.map(|bits| { let [az, el, strength] = bits.map(f32::from_bits); prepared.seen_lit(az, el, strength) });
                let pixels = lit.as_deref().unwrap_or(prepared.color());
                atomic(&staged, png_budget(r), |w| png_pixels(pixels, prepared.frame(), r, w))?;
                let (digest, bytes) = hashed_file(&staged, png_budget(r))?;
                std::fs::rename(&staged, dir.join(format!("{digest}.png")))?;
                images.push(Png { id: digest, bytes });
            }
            File::open(dir)?.sync_all()?;
            let value = Generation { config, images: images.try_into().map_err(|_| invalid("missing companion"))?, parent: old.clone() };
            let bytes = serde_json::to_vec(&value).map_err(|e| invalid(e.to_string()))?;
            let name = hex(&Sha256::digest(&bytes));
            atomic(&dir.join(format!("{name}.json")), 16384, |w| w.write_all(&bytes))?;
            atomic(&dir.join("CURRENT"), 65, |w| writeln!(w, "{name}"))?;
            collect(dir, Some(&name))?;
            selected = Some(name);
        }
        let selected = selected.unwrap();
        let value = generation(dir, &selected)?;
        let mut output = serde_json::Map::new();
        output.insert("revision".into(), value.config.revision.into());
        output.insert("snapshot".into(), value.config.snapshot.into());
        output.insert("generation".into(), selected.into());
        for (mode, image) in ["color", "gallery", "relief"].into_iter().zip(value.images) { output.insert(mode.into(), dir.join(format!("{}.png", image.id)).to_string_lossy().into_owned().into()); }
        println!("{}", serde_json::Value::Object(output));
        Ok(())
    })();
    result.map_err(|e| e.to_string())
}
