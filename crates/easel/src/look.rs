//! Looking at the canvas: bounded PNGs an agent can read. The look shows
//! the paint on the canvas as it is now (the dry picture with the wet
//! paint on it as laid), whole or a window of it, and a few ways a painter
//! looks at a picture: in grays, squinting and in a mirror. A grid in
//! canvas units can be laid over the look, as a painter squares up a
//! drawing; it never touches the canvas.

use image::ImageEncoder;
use paint::color::luminance;
use paint::{Canvas, Rgb};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct View {
    /// Crop in canvas units (x0, y0, x1, y1).
    pub crop: Option<[f32; 4]>,
    pub value: bool,
    pub squint: bool,
    pub mirror: bool,
    /// Longest side, px (whole: 1000, capped at 1600; crop: capped at 1200).
    pub size: Option<usize>,
    /// Coordinate grid: Some(0) picks the step from the zoom.
    pub grid: Option<f32>,
    /// Raking light on the paint's relief (wet and dry): (azimuth, elevation)
    /// in degrees, see `Canvas::seen_lit`.
    pub light: Option<(f32, f32)>,
    /// The palette instead of the canvas (`palette`).
    pub palette: bool,
    pub mask: Vec<String>,
    pub overlays: Vec<Arc<paint::Mask>>,
}

const LOOK_ARGS: &str = "--crop x0,y0,x1,y1 --mode value,squint,mirror,relief,gallery --light azimuth,elevation --grid [step] --size N --mask name[,name] | --palette";

/// The relief look's default light: from the upper left, raking at 25°.
const RAKING: (f32, f32) = (135.0, 25.0);

fn is_num(s: Option<&String>) -> bool {
    s.is_some_and(|s| s.parse::<f32>().is_ok())
}

impl View {
    pub fn display_eligible(&self) -> bool {
        !(self.value || self.squint || self.mirror || self.light.is_some() || self.palette)
    }

    /// Parse look arguments (see `LOOK_ARGS`).
    pub fn parse(args: &[String]) -> std::result::Result<View, String> {
        let mut v = View::default();
        let mut i = 0;
        while i < args.len() {
            let a = args[i].as_str();
            let mut next = || {
                i += 1;
                args.get(i).cloned().ok_or(format!("{a} needs a value"))
            };
            match a {
                "--crop" => {
                    let s = next()?;
                    let p: Vec<f32> = s
                        .split(',')
                        .map(|t| t.trim().parse::<f32>())
                        .collect::<std::result::Result<_, _>>()
                        .map_err(|_| format!("--crop {s}: want four numeric canvas-unit coordinates x0,y0,x1,y1 (opposite corners, not width/height or pixels); example: --crop 100,100,200,200"))?;
                    if p.len() != 4 || p.iter().any(|n| !n.is_finite()) {
                        return Err(format!("--crop {s}: want four numeric canvas-unit coordinates x0,y0,x1,y1 (opposite corners, not width/height or pixels); example: --crop 100,100,200,200"));
                    }
                    v.crop = Some([p[0].min(p[2]), p[1].min(p[3]), p[0].max(p[2]), p[1].max(p[3])]);
                }
                "--mode" => {
                    for m in next()?.split(',') {
                        match m.trim() {
                            "normal" | "" => {}
                            "value" | "gray" => v.value = true,
                            "squint" | "blur" => v.squint = true,
                            "mirror" => v.mirror = true,
                            "relief" | "raking" => v.light = Some(v.light.unwrap_or(RAKING)),
                            "gallery" => v.light = Some(v.light.unwrap_or(crate::GALLERY_LIGHT)),
                            o => return Err(format!("--mode {o}: normal, value, squint, mirror, relief, gallery (comma-separated)")),
                        }
                    }
                }
                "--value" => v.value = true,
                "--squint" => v.squint = true,
                "--mirror" => v.mirror = true,
                "--light" => v.light = Some(crate::light_of(&next()?).map_err(|e| format!("--{e}"))?),
                "--size" => v.size = Some(next()?.parse().map_err(|_| "--size N (px)".to_string())?),
                "--grid" => {
                    v.grid = Some(0.0);
                    if is_num(args.get(i + 1)) {
                        let s: f32 = args[i + 1].parse().unwrap();
                        i += 1;
                        if s.is_nan() || s <= 0.0 {
                            return Err("--grid step: want > 0 units".into());
                        }
                        v.grid = Some(s);
                    }
                }
                "--palette" => v.palette = true,
                "--mask" => {
                    for name in next()?.split(',') {
                        let name = name.trim();
                        if name.is_empty() {
                            return Err("--mask needs a mask name (or several, comma-separated)".into());
                        }
                        v.mask.push(name.to_string());
                    }
                }
                o => return Err(format!("look: unknown argument {o:?} ({LOOK_ARGS})")),
            }
            i += 1;
        }
        if v.palette && (v.crop.is_some() || v.value || v.squint || v.mirror || v.light.is_some() || v.size.is_some() || v.grid.is_some() || !v.mask.is_empty()) {
            return Err("look: --palette takes no other option".into());
        }
        Ok(v)
    }
}

// ---------------------------------------------------------------- rendering

/// Units -> output pixels (and back).
#[derive(Clone, Copy)]
struct Map {
    s: f32,
    px0: f32,
    py0: f32,
    kx: f32,
    ky: f32,
    ow: usize,
    mirror: bool,
}

impl Map {
    fn to(&self, x: f32, y: f32) -> (f32, f32) {
        let ox = (x * self.s - self.px0) * self.kx;
        let oy = (y * self.s - self.py0) * self.ky;
        (if self.mirror { self.ow as f32 - ox } else { ox }, oy)
    }
    fn from(&self, ox: f32, oy: f32) -> (f32, f32) {
        let ox = if self.mirror { self.ow as f32 - ox } else { ox };
        ((ox / self.kx + self.px0) / self.s, (oy / self.ky + self.py0) / self.s)
    }
}

struct Img {
    w: usize,
    h: usize,
    px: Vec<Rgb>,
}

const INK: Rgb = [0.0, 0.0, 0.0];

impl Img {
    fn blend(&mut self, x: i64, y: i64, c: Rgb, a: f32) {
        if x < 0 || y < 0 || x >= self.w as i64 || y >= self.h as i64 || a <= 0.0 {
            return;
        }
        let p = &mut self.px[y as usize * self.w + x as usize];
        let a = a.min(1.0);
        for q in 0..3 {
            p[q] += (c[q] - p[q]) * a;
        }
    }
    fn rect(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, c: Rgb, a: f32) {
        for y in y0..y1 {
            for x in x0..x1 {
                self.blend(x, y, c, a);
            }
        }
    }
    /// Text on a dark plate, top-left at (x, y); returns its width.
    fn text(&mut self, x: i64, y: i64, s: &str, fs: i64, c: Rgb) -> i64 {
        let n = s.chars().count() as i64;
        let (w, h) = (n * 4 * fs + fs, 7 * fs);
        self.rect(x, y, x + w, y + h, INK, 0.62);
        for (i, ch) in s.chars().enumerate() {
            let g = glyph(ch);
            for (row, bits) in g.iter().enumerate() {
                for col in 0..3 {
                    if bits & (4 >> col) != 0 {
                        let (gx, gy) = (x + fs + (i as i64 * 4 + col as i64) * fs, y + fs + row as i64 * fs);
                        self.rect(gx, gy, gx + fs, gy + fs, c, 1.0);
                    }
                }
            }
        }
        w
    }
}

/// A 3×5 pixel font (rows top to bottom; bit 4 = left column).
fn glyph(c: char) -> [u8; 5] {
    match c.to_ascii_uppercase() {
        '0' => [7, 5, 5, 5, 7],
        '1' => [2, 6, 2, 2, 7],
        '2' => [7, 1, 7, 4, 7],
        '3' => [7, 1, 3, 1, 7],
        '4' => [5, 5, 7, 1, 1],
        '5' => [7, 4, 7, 1, 7],
        '6' => [7, 4, 7, 5, 7],
        '7' => [7, 1, 1, 2, 2],
        '8' => [7, 5, 7, 5, 7],
        '9' => [7, 5, 7, 1, 7],
        '-' => [0, 0, 7, 0, 0],
        '+' => [0, 2, 7, 2, 0],
        '.' => [0, 0, 0, 0, 2],
        ',' => [0, 0, 0, 2, 4],
        ':' => [0, 2, 0, 2, 0],
        ';' => [0, 2, 0, 2, 4],
        '\'' => [2, 2, 0, 0, 0],
        '=' => [0, 7, 0, 7, 0],
        '/' => [1, 1, 2, 4, 4],
        '(' => [1, 2, 2, 2, 1],
        ')' => [4, 2, 2, 2, 4],
        '#' => [5, 7, 5, 7, 5],
        '_' => [0, 0, 0, 0, 7],
        '?' => [7, 1, 2, 0, 2],
        'A' => [2, 5, 7, 5, 5],
        'B' => [6, 5, 6, 5, 6],
        'C' => [3, 4, 4, 4, 3],
        'D' => [6, 5, 5, 5, 6],
        'E' => [7, 4, 6, 4, 7],
        'F' => [7, 4, 6, 4, 4],
        'G' => [3, 4, 5, 5, 3],
        'H' => [5, 5, 7, 5, 5],
        'I' => [7, 2, 2, 2, 7],
        'J' => [1, 1, 1, 5, 2],
        'K' => [5, 5, 6, 5, 5],
        'L' => [4, 4, 4, 4, 7],
        'M' => [5, 7, 7, 5, 5],
        'N' => [6, 5, 5, 5, 5],
        'O' => [2, 5, 5, 5, 2],
        'P' => [6, 5, 6, 4, 4],
        'Q' => [2, 5, 5, 6, 3],
        'R' => [6, 5, 6, 5, 5],
        'S' => [3, 4, 2, 1, 6],
        'T' => [7, 2, 2, 2, 2],
        'U' => [5, 5, 5, 5, 7],
        'V' => [5, 5, 5, 5, 2],
        'W' => [5, 5, 7, 7, 5],
        'X' => [5, 5, 2, 5, 5],
        'Y' => [5, 5, 2, 2, 2],
        'Z' => [7, 1, 2, 4, 7],
        _ => [0, 0, 0, 0, 0],
    }
}

/// A round grid step (units) of at least `min`.
fn nice_step(min: f32) -> f32 {
    let mut p = 10f32.powf(min.max(1e-3).log10().floor());
    loop {
        for m in [1.0, 2.0, 2.5, 5.0] {
            if m * p >= min {
                return m * p;
            }
        }
        p *= 10.0;
    }
}

fn fmt_units(v: f32) -> String {
    if (v - v.round()).abs() < 1e-3 {
        format!("{}", v.round() as i64)
    } else {
        format!("{v:.1}")
    }
}

/// Lines that read over light and dark paint: darken the light, lighten the dark.
fn grid_line(img: &mut Img, x: i64, y: i64, a: f32) {
    if x < 0 || y < 0 || x >= img.w as i64 || y >= img.h as i64 {
        return;
    }
    let l = luminance(img.px[y as usize * img.w + x as usize]);
    let c = if l > 0.16 { [0.0, 0.01, 0.03] } else { [0.75, 0.92, 1.0] };
    img.blend(x, y, c, a);
}

/// Grid lines and labels in output pixels. Lines closer than an output
/// pixel are refused (they'd be more lines than pixels).
fn draw_grid(img: &mut Img, m: &Map, step: f32, fs: i64) -> std::result::Result<(), String> {
    let ppu = m.s * m.kx;
    if step > 0.0 && step * m.s * m.kx.min(m.ky) < 1.0 {
        let min = 1.0 / (m.s * m.kx.min(m.ky));
        return Err(format!("--grid {step}: lines closer than a pixel of this look; want at least {min:.2} units"));
    }
    let major = if step > 0.0 { step } else { nice_step(90.0 / ppu) };
    let minor = [5.0, 4.0, 2.0].into_iter().map(|d| major / d).find(|s| s * ppu >= 14.0);
    let (u0, v0) = m.from(0.0, 0.0);
    let (u1, v1) = m.from(img.w as f32, img.h as f32);
    let (ua, ub) = (u0.min(u1), u0.max(u1));
    let (va, vb) = (v0.min(v1), v0.max(v1));
    let thick = if fs >= 3 { 2 } else { 1 };
    let mut lines = |st: f32, alpha: f32, w: i64| {
        let mut k = (ua / st).ceil() as i64;
        while k as f32 * st <= ub {
            let (ox, _) = m.to(k as f32 * st, 0.0);
            for dx in 0..w {
                for y in 0..img.h as i64 {
                    grid_line(img, ox.floor() as i64 + dx, y, alpha);
                }
            }
            k += 1;
        }
        let mut k = (va / st).ceil() as i64;
        while k as f32 * st <= vb {
            let (_, oy) = m.to(0.0, k as f32 * st);
            for dy in 0..w {
                for x in 0..img.w as i64 {
                    grid_line(img, x, oy.floor() as i64 + dy, alpha);
                }
            }
            k += 1;
        }
    };
    if let Some(mi) = minor {
        lines(mi, 0.2, 1);
    }
    lines(major, 0.55, thick);
    // labels along the top and left edges
    let lab: Rgb = [0.95, 0.95, 0.85];
    let mut k = (ua / major).ceil() as i64;
    let mut last = -1000;
    let mut xs = Vec::new();
    while k as f32 * major <= ub {
        xs.push(k as f32 * major);
        k += 1;
    }
    if m.mirror {
        xs.reverse();
    }
    for u in xs {
        let (ox, _) = m.to(u, 0.0);
        let x = ox.round() as i64 + 3;
        if x > last && x < img.w as i64 - 2 * fs {
            last = x + img.text(x, 2, &fmt_units(u), fs, lab) + 4 * fs;
        }
    }
    let mut k = (va / major).ceil() as i64;
    let mut last = 7 * fs + 4;
    while k as f32 * major <= vb {
        let (_, oy) = m.to(0.0, k as f32 * major);
        let y = oy.round() as i64 + 3;
        if y > last && y < img.h as i64 - 16 * fs {
            img.text(2, y, &fmt_units(k as f32 * major), fs, lab);
            last = y + 9 * fs;
        }
        k += 1;
    }
    let legend = match minor {
        Some(mi) => format!("GRID {} / {} UNITS", fmt_units(major), fmt_units(mi)),
        None => format!("GRID {} UNITS", fmt_units(major)),
    };
    img.text(2, img.h as i64 - 7 * fs - 2, &legend, fs, lab);
    Ok(())
}

/// The pixels to a canvas unit a crop is shown at (the 2400 px canvas's own).
const LOOK_SCALE: f32 = 2.4;

/// Render a PNG to `out`: whole views fit 1600 px and 3 MB; crops fit 1200 px per side.
pub fn look(c: &Canvas, v: &View, out: &Path) -> std::result::Result<(usize, usize), String> {
    let (w, h, png) = render(c, v)?;
    if let Some(d) = out.parent().filter(|d| !d.as_os_str().is_empty()) {
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    std::fs::write(out, png).map_err(|e| e.to_string())?;
    Ok((w, h))
}

pub struct DisplayPreset {
    pub name: &'static str,
    pub folder: &'static str,
    pub light: (f32, f32),
}

pub const DISPLAY_PRESETS: [DisplayPreset; 2] = [
    DisplayPreset { name: "gallery", folder: ".gallery", light: crate::GALLERY_LIGHT },
    DisplayPreset { name: "relief", folder: ".relief", light: RAKING },
];

pub fn companions(c: &Canvas, views: &[(View, PathBuf)]) {
    if !views.iter().any(|(v, _)| v.display_eligible()) {
        return;
    }
    let prepared = c.prepare_optics();
    for preset in DISPLAY_PRESETS {
        let seen = prepared.seen_lit(preset.light.0, preset.light.1, 1.0);
        for (v, out) in views.iter().filter(|(v, _)| v.display_eligible()) {
            let result = (|| -> Result<(), String> {
                let mut lit = v.clone();
                lit.light = Some(preset.light);
                let (_, _, png) = render_seen(c, &lit, Some(&seen))?;
                let dir = out.parent().unwrap_or_else(|| Path::new("")).join(preset.folder);
                let name = out.file_name().ok_or("display companion needs a PNG filename")?;
                std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
                std::fs::write(dir.join(name), png).map_err(|e| e.to_string())
            })();
            if let Err(e) = result {
                eprintln!("warning: couldn't save {} companion for {}: {e}", preset.name, out.display());
            }
        }
    }
}

/// `look`'s PNG bytes and size, written nowhere.
/// Labels written on a rendered look: (x, y as fractions of the image, text),
/// light text on a dark plate.
pub(crate) fn label(png: &[u8], labels: &[(f32, f32, String)]) -> std::result::Result<Vec<u8>, String> {
    let im = image::load_from_memory(png).map_err(|e| e.to_string())?.to_rgb8();
    let (w, h) = (im.width() as usize, im.height() as usize);
    let mut img = Img { w, h, px: im.pixels().map(|p| [p[0] as f32 / 255.0, p[1] as f32 / 255.0, p[2] as f32 / 255.0]).collect() };
    let fs = ((w / 520) as i64).max(1);
    for (fx, fy, s) in labels {
        img.text((fx * w as f32) as i64 + 2, (fy * h as f32) as i64, s, fs, [0.96, 0.95, 0.9]);
    }
    let buf: Vec<u8> = img.px.iter().flat_map(|c| c.iter().map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8)).collect();
    let mut out = Vec::new();
    image::codecs::png::PngEncoder::new(&mut out).write_image(&buf, w as u32, h as u32, image::ExtendedColorType::Rgb8).map_err(|e| e.to_string())?;
    Ok(out)
}

pub fn render(c: &Canvas, v: &View) -> std::result::Result<(usize, usize, Vec<u8>), String> {
    render_seen(c, v, None)
}

/// `render`, from the canvas as already seen in the view's light (`seen`:
/// `Canvas::seen` or `seen_lit`), so several views of one canvas (a survey's
/// tiles) light it once.
pub fn render_seen(c: &Canvas, v: &View, seen: Option<&[Rgb]>) -> std::result::Result<(usize, usize, Vec<u8>), String> {
    let own;
    let px: &[Rgb] = match seen {
        Some(px) => px,
        None => {
            own = match v.light {
                Some((az, el)) => c.seen_lit(az, el, 1.0),
                None => c.seen(),
            };
            &own
        }
    };
    render_pixels(c.window(), v, px)
}

fn crop_bounds(f: paint::Frame, v: &View) -> Result<(usize, usize, usize, usize), String> {
    let (wx0, wy0, wx1, wy1) = (f.x0, f.y0, f.x0 + f.w, f.y0 + f.h);
    let bounds = match v.crop {
        None => (wx0, wy0, wx1, wy1),
        Some([a, b, cc, d]) => {
            let s = f.scale;
            let cl = |u: f32, lo: usize, hi: usize| ((u * s).round().max(0.0) as usize).clamp(lo, hi);
            let r = (cl(a, wx0, wx1), cl(b, wy0, wy1), cl(cc, wx0, wx1), cl(d, wy0, wy1));
            if r.2 <= r.0 || r.3 <= r.1 {
                return Err("--crop is empty or outside the canvas".into());
            }
            r
        }
    };
    Ok(bounds)
}

pub fn crop_scale(f: paint::Frame, v: &View, w: usize, h: usize) -> Result<String, String> {
    if v.crop.is_none() {
        return Ok(String::new());
    }
    let (x0, y0, x1, y1) = crop_bounds(f, v)?;
    Ok(format!("; scale {:.4} x {:.4} px/unit", w as f32 * f.scale / (x1 - x0) as f32, h as f32 * f.scale / (y1 - y0) as f32))
}

fn draw_masks(img: &mut Img, map: &Map, masks: &[Arc<paint::Mask>]) {
    let colors = [[1.0, 0.15, 0.65], [0.0, 0.85, 1.0], [0.85, 1.0, 0.1]];
    for (i, mask) in masks.iter().enumerate() {
        let color = colors[i % colors.len()];
        let sample = |x: i64, y: i64| {
            let (u, v) = map.from(x as f32 + 0.5, y as f32 + 0.5);
            mask.sample(u, v).clamp(0.0, 1.0)
        };
        for y in 0..img.h {
            for x in 0..img.w {
                let a = sample(x as i64, y as i64);
                img.blend(x as i64, y as i64, color, 0.14 * a);
                let inside = a >= 0.5;
                let edge = [(x as i64 - 1, y as i64), (x as i64 + 1, y as i64), (x as i64, y as i64 - 1), (x as i64, y as i64 + 1)].iter().any(|&(xx, yy)| {
                    (sample(xx, yy) >= 0.5) != inside
                });
                if edge {
                    img.blend(x as i64, y as i64, color, 0.9);
                }
            }
        }
    }
}

pub fn render_pixels(f: paint::Frame, v: &View, px: &[Rgb]) -> std::result::Result<(usize, usize, Vec<u8>), String> {
    if f.w.checked_mul(f.h) != Some(px.len()) {
        return Err("look: optical pixels do not match the captured frame".into());
    }
    let (wx0, wy0) = (f.x0, f.y0);
    let (x0, y0, x1, y1) = crop_bounds(f, v)?;
    let (cw, ch) = (x1 - x0, y1 - y0);
    let long = cw.max(ch);
    // a crop is shown at LOOK_SCALE px to a unit whatever the canvas's own width:
    // a painting begun at 4800 px looks to its painter as one begun at 2400 does
    let shown = |n: usize| if f.scale > LOOK_SCALE { ((n as f32 * LOOK_SCALE / f.scale).round() as usize).max(1) } else { n };
    if long == 0 || cw == 0 || ch == 0 {
        return Err("look: canvas is empty".into());
    }
    let mut size = if v.crop.is_some() { shown(long).min(v.size.unwrap_or(1200).clamp(1, 1200)) } else { v.size.unwrap_or(1000).clamp(1, 1600) };
    loop {
        // Average down from the original canvas on each attempt; never enlarge.
        let (ow, oh, img): (usize, usize, Vec<Rgb>) = if long > size {
            let k = size as f32 / long as f32;
            let (ow, oh) = (((cw as f32 * k).round() as usize).max(1), ((ch as f32 * k).round() as usize).max(1));
            let img = crate::output::average_linear(px, f.w, [x0 - wx0, y0 - wy0, x1 - wx0, y1 - wy0], ow, oh);
            (ow, oh, img)
        } else {
            let img = (0..cw * ch).map(|i| px[(y0 - wy0 + i / cw) * f.w + x0 - wx0 + i % cw]).collect();
            (cw, ch, img)
        };
        let mut img = img;
        if v.squint {
            // half-closed eyes: detail goes, the big shapes and values stay
            img = blur(&img, ow, oh, (ow.max(oh) as f32 * 0.012).max(2.0));
        }
        if v.value {
            for p in img.iter_mut() {
                let l = luminance(*p);
                *p = [l; 3];
            }
        }
        if v.mirror {
            for row in img.chunks_mut(ow) {
                row.reverse();
            }
        }
        // the aids, drawn over the picture in output pixels
        let map = Map {
            s: f.scale,
            px0: x0 as f32,
            py0: y0 as f32,
            kx: ow as f32 / cw as f32,
            ky: oh as f32 / ch as f32,
            ow,
            mirror: v.mirror,
        };
        let fs = if ow.max(oh) >= 1500 { 3 } else { 2 };
        let mut im = Img { w: ow, h: oh, px: img };
        draw_masks(&mut im, &map, &v.overlays);
        if let Some(step) = v.grid {
            draw_grid(&mut im, &map, step, fs)?;
        }
        let buf = crate::output::rgb8(&im.px, ow);
        let mut png = Vec::new();
        image::codecs::png::PngEncoder::new(&mut png)
            .write_image(&buf, ow as u32, oh as u32, image::ExtendedColorType::Rgb8)
            .map_err(|e| e.to_string())?;
        if v.crop.is_none() && png.len() > 3_000_000 {
            // Reduce both dimensions, then redraw aids at the new output resolution.
            // Strict progress guarantees termination, even for incompressible images.
            size = (ow.max(oh) * 4 / 5).max(1);
            continue;
        }
        return Ok((ow, oh, png));
    }
}

// ---------------------------------------------------------------- the palette

/// The palette look's thicknesses, in coats (`paint::COAT_UM`, 25 µm, is one): a pile laid
/// thick, one thin coat and a very thin one.
pub const THICK: f32 = 8.0;
pub const THIN: f32 = 0.48;
pub const VERY_THIN: f32 = 0.16;
/// The drawdown card under the last swatch: white, with a black stripe across it.
const CARD_WHITE: Rgb = [0.85, 0.85, 0.85];
const CARD_BLACK: Rgb = [0.01, 0.01, 0.01];

const SW: usize = 136;
const SH: usize = 48;
const LABEL_W: usize = 540;
const GAP: usize = 8;

/// One row's swatches, left to right: the pile laid thick over the ground, a thin and a
/// very thin coat over the ground, and the thin coat over the black-and-white card. Each is
/// the pile's own paint and `Paint::over`: nothing on the canvas is read or changed. A
/// thinned pile (`paint::thinner`) is shown as it dries: each coat (wet, as laid) leaves the
/// share `1 - thinner` of it once the solvent has gone, so its swatches are thinner.
pub fn swatches(p: &paint::Paint, ground: Rgb) -> [Rgb; 5] {
    let k = 1.0 - p.thinner();
    let (thick, thin, very) = if k < 1.0 { (THICK * k, THIN * k, VERY_THIN * k) } else { (THICK, THIN, VERY_THIN) };
    [p.over(ground, thick), p.over(ground, thin), p.over(ground, very), p.over(CARD_WHITE, thin), p.over(CARD_BLACK, thin)]
}

/// Up to `n` characters a line, broken at spaces where it can.
fn wrap(s: &str, n: usize) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    for w in s.split(' ') {
        if !cur.is_empty() && cur.chars().count() + 1 + w.chars().count() > n {
            out.push(std::mem::take(&mut cur));
        }
        if !cur.is_empty() {
            cur.push(' ');
        }
        cur.push_str(w);
        while cur.chars().count() > n {
            let rest: String = cur.chars().skip(n).collect();
            out.push(cur.chars().take(n).collect());
            cur = rest;
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// The palette look: a row for every pile in `rows` (the global's name, the recipe, the
/// pile's paint), its swatches (`swatches`) over this canvas's `ground`. A PNG, as `render`.
pub fn palette(rows: &[(String, String, paint::Paint)], ground: Rgb) -> std::result::Result<(usize, usize, Vec<u8>), String> {
    if rows.is_empty() {
        return Err("look: no pile on the palette: no global holds a pile (p = pile{...})".into());
    }
    let fs = 2i64;
    let line = (7 * fs + 2) as usize;
    let chars = (LABEL_W - 2 * fs as usize) / (4 * fs as usize) - 1;
    let labels: Vec<Vec<String>> = rows.iter().map(|(n, r, _)| [vec![n.clone()], wrap(r, chars)].concat()).collect();
    let heights: Vec<usize> = labels.iter().map(|l| (l.len() * line + 6).max(SH + 2 * GAP)).collect();
    let head = line + 2 * GAP;
    let w = LABEL_W + 4 * (SW + GAP) + GAP;
    let h = head + heights.iter().sum::<usize>();
    let mut im = Img { w, h, px: vec![[0.18, 0.18, 0.18]; w * h] };
    let x0 = LABEL_W as i64;
    let col = |k: usize| x0 + (k * (SW + GAP)) as i64;
    let titles = ["THICK", "12 UM ON GROUND", "4 UM ON GROUND", "12 UM ON CARD"];
    for (k, t) in titles.iter().enumerate() {
        im.text(col(k), GAP as i64, t, fs, [0.9; 3]);
    }
    let mut y = head as i64;
    for ((label, rh), (_, _, paint)) in labels.iter().zip(&heights).zip(rows) {
        for (j, l) in label.iter().enumerate() {
            im.text(GAP as i64, y + 3 + (j * line) as i64, l, fs, if j == 0 { [1.0; 3] } else { [0.75; 3] });
        }
        let sw = swatches(paint, ground);
        let sy = y + ((*rh - SH) / 2) as i64;
        for k in 0..3 {
            im.rect(col(k), sy, col(k) + SW as i64, sy + SH as i64, sw[k], 1.0);
        }
        // the card: white, a black stripe across its middle third
        let (cy0, cy1) = (sy + SH as i64 / 3, sy + 2 * SH as i64 / 3);
        im.rect(col(3), sy, col(3) + SW as i64, sy + SH as i64, sw[3], 1.0);
        im.rect(col(3), cy0, col(3) + SW as i64, cy1, sw[4], 1.0);
        y += *rh as i64;
    }
    let buf = crate::output::rgb8_exact(&im.px, w);
    let mut png = Vec::new();
    image::codecs::png::PngEncoder::new(&mut png).write_image(&buf, w as u32, h as u32, image::ExtendedColorType::Rgb8).map_err(|e| e.to_string())?;
    Ok((w, h, png))
}

/// Three box blurs ≈ a Gaussian of sd `r`.
fn blur(img: &[Rgb], w: usize, h: usize, r: f32) -> Vec<Rgb> {
    let k = ((r * 0.6).round() as usize).max(1);
    let mut a = img.to_vec();
    let mut b = a.clone();
    for _ in 0..3 {
        for y in 0..h {
            for x in 0..w {
                let (lo, hi) = (x.saturating_sub(k), (x + k).min(w - 1));
                let mut s = [0.0; 3];
                for i in lo..=hi {
                    for q in 0..3 {
                        s[q] += a[y * w + i][q];
                    }
                }
                let n = (hi - lo + 1) as f32;
                b[y * w + x] = [s[0] / n, s[1] / n, s[2] / n];
            }
        }
        for y in 0..h {
            for x in 0..w {
                let (lo, hi) = (y.saturating_sub(k), (y + k).min(h - 1));
                let mut s = [0.0; 3];
                for j in lo..=hi {
                    for q in 0..3 {
                        s[q] += b[j * w + x][q];
                    }
                }
                let n = (hi - lo + 1) as f32;
                a[y * w + x] = [s[0] / n, s[1] / n, s[2] / n];
            }
        }
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    fn view(args: &[&str]) -> View {
        View::parse(&args.iter().map(|s| s.to_string()).collect::<Vec<_>>()).unwrap()
    }

    #[test]
    fn crops_of_any_span_are_bounded_and_report_the_actual_scale() {
        for width in [600, 2400, 4800] {
            let f = paint::Frame::new(width, width / 5, width as f32 / 1000.0);
            let px = vec![[0.2, 0.3, 0.4]; f.w * f.h];
            let v = view(&["--crop", "0,0,1000,100"]);
            let (w, h, _) = render_pixels(f, &v, &px).unwrap();
            let expected = width.min(1200);
            assert_eq!((w, h), (expected, expected / 10));
            assert_eq!(crop_scale(f, &v, w, h).unwrap(), format!("; scale {:.4} x {:.4} px/unit", expected as f32 / 1000.0, expected as f32 / 1000.0));
            let small = view(&["--crop", "0,0,1000,100", "--size", "300"]);
            assert_eq!(render_pixels(f, &small, &px).unwrap().0, 300);
            let capped = view(&["--crop", "0,0,1000,100", "--size", "9999"]);
            assert_eq!(render_pixels(f, &capped, &px).unwrap().0, expected);
        }
        let f = paint::Frame::new(120, 2400, 2.4);
        let px = vec![[0.2; 3]; f.w * f.h];
        let v = view(&["--crop", "-10,-10,50,1000"]);
        let (w, h, _) = render_pixels(f, &v, &px).unwrap();
        assert_eq!((w, h), (60, 1200));
        assert_eq!(crop_scale(f, &v, w, h).unwrap(), "; scale 1.2000 x 1.2000 px/unit");
        assert!(render_pixels(f, &view(&["--crop", "100,0,200,100"]), &px).is_err());
    }

    #[test]
    fn fitting_crops_keep_their_pixels_and_whole_views_keep_their_size() {
        let f = paint::Frame::new(100, 60, 0.1);
        let px: Vec<Rgb> = (0..f.w * f.h).map(|i| [i as f32 / 6000.0, 0.2, 0.3]).collect();
        let v = view(&["--crop", "100,100,300,300"]);
        let (w, h, png) = render_pixels(f, &v, &px).unwrap();
        assert_eq!((w, h), (20, 20));
        let expected: Vec<Rgb> = (10..30).flat_map(|y| px[y * 100 + 10..y * 100 + 30].iter().copied()).collect();
        assert_eq!(image::load_from_memory(&png).unwrap().to_rgb8().as_raw(), &crate::output::rgb8(&expected, w));
        let f = paint::Frame::new(2400, 24, 2.4);
        let px = vec![[0.3; 3]; f.w * f.h];
        assert_eq!(render_pixels(f, &View::default(), &px).unwrap().0, 1000);
        assert_eq!(render_pixels(f, &view(&["--size", "2000"]), &px).unwrap().0, 1600);
        assert_eq!(render_pixels(f, &view(&["--crop", "0,0,500,10"]), &px).unwrap().0, 1200);
        assert_eq!(render_pixels(f, &view(&["--crop", "0,0,200,10"]), &px).unwrap().0, 480);
        assert!(crop_scale(f, &View::default(), 1000, 10).unwrap().is_empty());
    }

    #[test]
    fn masks_overlay_fill_and_outlines_in_crop_and_mirror_coordinates() {
        let f = paint::Frame::new(100, 60, 0.1);
        let px = vec![[0.2; 3]; f.w * f.h];
        let mut v = view(&["--mask", "shape, soft"]);
        assert_eq!(v.mask, ["shape", "soft"]);
        v.overlays = vec![
            Arc::new(paint::Mask::from_fn(f, |x, y| if (200.0..400.0).contains(&x) && (100.0..300.0).contains(&y) { 1.0 } else { 0.0 })),
            Arc::new(paint::Mask::from_fn(f, |x, y| if (600.0..800.0).contains(&x) && (100.0..300.0).contains(&y) { 0.25 } else { 0.0 })),
        ];
        let original = image::load_from_memory(&render_pixels(f, &View::default(), &px).unwrap().2).unwrap().to_rgb8();
        let shown = image::load_from_memory(&render_pixels(f, &v, &px).unwrap().2).unwrap().to_rgb8();
        assert_eq!(shown.get_pixel(5, 5), original.get_pixel(5, 5));
        assert_ne!(shown.get_pixel(30, 20), original.get_pixel(30, 20));
        assert_ne!(shown.get_pixel(70, 20), original.get_pixel(70, 20));
        assert!(shown.get_pixel(20, 20)[0] > shown.get_pixel(30, 20)[0]);
        let mut mirrored = v.clone();
        mirrored.mirror = true;
        let mirrored = image::load_from_memory(&render_pixels(f, &mirrored, &px).unwrap().2).unwrap().to_rgb8();
        assert_eq!(mirrored, image::imageops::flip_horizontal(&shown));
        v.crop = Some([100.0, 0.0, 500.0, 400.0]);
        let cropped = image::load_from_memory(&render_pixels(f, &v, &px).unwrap().2).unwrap().to_rgb8();
        assert_eq!(cropped, image::imageops::crop_imm(&shown, 10, 0, 40, 40).to_image());
        assert!(View::parse(&["--mask".into(), "a,".into()]).is_err());
        assert!(View::parse(&["--palette".into(), "--mask".into(), "shape".into()]).is_err());
    }
}
