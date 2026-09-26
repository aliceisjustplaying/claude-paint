//! Looking at the canvas: small JPEGs an agent can read. The look shows
//! the paint on the canvas as it is now (the dry picture with the wet
//! paint on it as laid), whole or a window of it, and a few ways a painter
//! looks at a picture: in grays, squinting, in a mirror, and where the
//! paint is wet or dry (as a painter feels it with a knuckle). A grid in
//! canvas units can be laid over the look, as a painter squares up a
//! drawing; it never touches the canvas.

use paint::color::{linear_to_srgb, luminance};
use paint::{Canvas, Rgb};
use std::path::Path;

#[derive(Default)]
pub struct View {
    /// Crop in canvas units (x0, y0, x1, y1).
    pub crop: Option<[f32; 4]>,
    pub value: bool,
    pub squint: bool,
    pub mirror: bool,
    /// Where the paint is in drying: open, setting, tacky and dry in false
    /// color over the picture, dimmed to gray.
    pub wet: bool,
    /// Longest side of the image, px (None: 1000).
    pub size: Option<usize>,
    /// Coordinate grid: Some(0) picks the step from the zoom.
    pub grid: Option<f32>,
}

const LOOK_ARGS: &str = "--crop x0,y0,x1,y1 --mode value,squint,mirror,wet --grid [step] --size N";

fn is_num(s: Option<&String>) -> bool {
    s.is_some_and(|s| s.parse::<f32>().is_ok())
}

impl View {
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
                    let p: Vec<f32> = s.split(',').map(|t| t.trim().parse::<f32>()).collect::<std::result::Result<_, _>>().map_err(|_| format!("--crop {s}: want x0,y0,x1,y1 in units"))?;
                    if p.len() != 4 {
                        return Err(format!("--crop {s}: want x0,y0,x1,y1 in units"));
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
                            "wet" | "drying" | "stages" => v.wet = true,
                            o => return Err(format!("--mode {o}: normal, value, squint, mirror, wet (comma-separated)")),
                        }
                    }
                }
                "--value" => v.value = true,
                "--squint" => v.squint = true,
                "--mirror" => v.mirror = true,
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
                o => return Err(format!("look: unknown argument {o:?} ({LOOK_ARGS})")),
            }
            i += 1;
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
    if (v - v.round()).abs() < 1e-3 { format!("{}", v.round() as i64) } else { format!("{v:.1}") }
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

fn draw_grid(img: &mut Img, m: &Map, step: f32, fs: i64) {
    let ppu = m.s * m.kx;
    let major = if step > 0.0 { step } else { nice_step(90.0 / ppu) };
    let minor = [5.0, 4.0, 2.0].into_iter().map(|d| major / d).find(|s| s * ppu >= 14.0);
    let (u0, v0) = m.from(0.0, 0.0);
    let (u1, v1) = m.from(img.w as f32, img.h as f32);
    let (ua, ub) = (u0.min(u1), u0.max(u1));
    let (va, vb) = (v0.min(v1), v0.max(v1));
    let thick = if fs >= 3 { 2 } else { 1 };
    let mut lines = |st: f32, alpha: f32, w: i64| {
        let mut k = (ua / st).ceil();
        while k * st <= ub {
            let (ox, _) = m.to(k * st, 0.0);
            for dx in 0..w {
                for y in 0..img.h as i64 {
                    grid_line(img, ox.floor() as i64 + dx, y, alpha);
                }
            }
            k += 1.0;
        }
        let mut k = (va / st).ceil();
        while k * st <= vb {
            let (_, oy) = m.to(0.0, k * st);
            for dy in 0..w {
                for x in 0..img.w as i64 {
                    grid_line(img, x, oy.floor() as i64 + dy, alpha);
                }
            }
            k += 1.0;
        }
    };
    if let Some(mi) = minor {
        lines(mi, 0.2, 1);
    }
    lines(major, 0.55, thick);
    // labels along the top and left edges
    let lab: Rgb = [0.95, 0.95, 0.85];
    let mut k = (ua / major).ceil();
    let mut last = -1000;
    let mut xs = Vec::new();
    while k * major <= ub {
        xs.push(k * major);
        k += 1.0;
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
    let mut k = (va / major).ceil();
    let mut last = 7 * fs + 4;
    while k * major <= vb {
        let (_, oy) = m.to(0.0, k * major);
        let y = oy.round() as i64 + 3;
        if y > last && y < img.h as i64 - 16 * fs {
            img.text(2, y, &fmt_units(k * major), fs, lab);
            last = y + 9 * fs;
        }
        k += 1.0;
    }
    let legend = match minor {
        Some(mi) => format!("GRID {} / {} UNITS", fmt_units(major), fmt_units(mi)),
        None => format!("GRID {} UNITS", fmt_units(major)),
    };
    img.text(2, img.h as i64 - 7 * fs - 2, &legend, fs, lab);
}

/// Render what the painter asked to see and write it as a JPEG.
pub fn look(c: &Canvas, v: &View, out: &Path) -> std::result::Result<(usize, usize), String> {
    let f = c.window();
    let px: Vec<Rgb> = c.seen();
    let px = if v.wet { stage_colors(c, &px) } else { px };
    // crop: units -> whole-canvas pixels -> pixels of the held window
    let (wx0, wy0, wx1, wy1) = (f.x0, f.y0, f.x0 + f.w, f.y0 + f.h);
    let (x0, y0, x1, y1) = match v.crop {
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
    let (cw, ch) = (x1 - x0, y1 - y0);
    let long = cw.max(ch);
    let size = v.size.unwrap_or(1000);
    // fit to size: average down, or enlarge by whole pixels (so pixels stay honest)
    let (ow, oh, img): (usize, usize, Vec<Rgb>) = if long > size {
        let k = size as f32 / long as f32;
        let (ow, oh) = (((cw as f32 * k).round() as usize).max(1), ((ch as f32 * k).round() as usize).max(1));
        let mut img = vec![[0.0f32; 3]; ow * oh];
        for oy in 0..oh {
            let (sy0, sy1) = (y0 + oy * ch / oh, (y0 + (oy + 1) * ch / oh).max(y0 + oy * ch / oh + 1));
            for ox in 0..ow {
                let (sx0, sx1) = (x0 + ox * cw / ow, (x0 + (ox + 1) * cw / ow).max(x0 + ox * cw / ow + 1));
                let mut acc = [0.0f32; 3];
                for y in sy0..sy1 {
                    for x in sx0..sx1 {
                        let p = px[(y - wy0) * f.w + x - wx0];
                        for q in 0..3 {
                            acc[q] += p[q];
                        }
                    }
                }
                let n = ((sy1 - sy0) * (sx1 - sx0)) as f32;
                img[oy * ow + ox] = [acc[0] / n, acc[1] / n, acc[2] / n];
            }
        }
        (ow, oh, img)
    } else {
        // round the enlargement up while the image stays within 1.6 × size
        let up = size.div_ceil(long);
        let k = if long * up * 5 <= size * 8 { up } else { (size / long).max(1) };
        let (ow, oh) = (cw * k, ch * k);
        let img = (0..ow * oh).map(|i| px[(y0 - wy0 + (i / ow) / k) * f.w + x0 - wx0 + (i % ow) / k]).collect();
        (ow, oh, img)
    };
    let mut img = img;
    if v.squint {
        // half-closed eyes: detail goes, the big shapes and values stay
        img = blur(&img, ow, oh, (ow.max(oh) as f32 * 0.012).max(2.0));
    }
    if v.value && !v.wet {
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
    let map = Map { s: f.scale, px0: x0 as f32, py0: y0 as f32, kx: ow as f32 / cw as f32, ky: oh as f32 / ch as f32, ow, mirror: v.mirror };
    let fs = if ow.max(oh) >= 1500 { 3 } else { 2 };
    let mut im = Img { w: ow, h: oh, px: img };
    if let Some(step) = v.grid {
        draw_grid(&mut im, &map, step, fs);
    }
    if v.wet {
        // the legend, top left
        let (mut x, y) = (4 * fs, 4 * fs);
        for (i, name) in ["OPEN", "SETTING", "TACKY", "DRY"].iter().enumerate() {
            x += im.text(x, y, name, fs, STAGE_LEGEND[i]) + 2 * fs;
        }
    }
    let buf: Vec<u8> = im.px.iter().flat_map(|p| p.map(|c| (linear_to_srgb(c) * 255.0).round().clamp(0.0, 255.0) as u8)).collect();
    if let Some(d) = out.parent() {
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    // quality 95 with full-resolution color (4:4:4), so thin colored
    // strokes and edges keep their color
    let file = std::fs::File::create(out).map_err(|e| e.to_string())?;
    let mut enc = jpeg_encoder::Encoder::new(std::io::BufWriter::new(file), 95);
    enc.set_sampling_factor(jpeg_encoder::SamplingFactor::R_4_4_4);
    let (w16, h16) = (u16::try_from(ow).map_err(|_| "look: image too wide")?, u16::try_from(oh).map_err(|_| "look: image too tall")?);
    enc.encode(&buf, w16, h16, jpeg_encoder::ColorType::Rgb).map_err(|e| e.to_string())?;
    Ok((ow, oh))
}

/// False colors for the drying stages (linear RGB): open, setting, tacky,
/// and dry (none: the dimmed picture shows through).
const STAGE_TINT: [Option<Rgb>; 4] = [Some([0.02, 0.22, 1.0]), Some([0.05, 0.75, 0.12]), Some([1.0, 0.36, 0.0]), None];
const STAGE_LEGEND: [Rgb; 4] = [[0.25, 0.5, 1.0], [0.2, 0.9, 0.25], [1.0, 0.5, 0.1], [0.55, 0.55, 0.55]];

/// The picture dimmed to gray, with each pixel tinted by where its paint
/// is in drying (`--mode wet`).
fn stage_colors(c: &Canvas, px: &[Rgb]) -> Vec<Rgb> {
    let st = c.stages();
    px.iter()
        .zip(st)
        .map(|(p, s)| {
            // dimmed gray keeps the picture's values; the tint is scaled by
            // them too, so forms still read through the color
            let g = 0.015 + 0.4 * luminance(*p);
            match STAGE_TINT[s as usize] {
                Some(t) => {
                    let k = 0.25 + 1.6 * g;
                    [g + (t[0] * k - g) * 0.5, g + (t[1] * k - g) * 0.5, g + (t[2] * k - g) * 0.5]
                }
                None => [g; 3],
            }
        })
        .collect()
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
    use crate::session::{Session, root};

    const W: usize = 160;

    fn bits(c: &Canvas) -> Vec<u32> {
        c.seen().iter().flat_map(|p| p.map(f32::to_bits)).chain(c.surface_um().iter().map(|v| v.to_bits())).collect()
    }

    const CANVAS: &str = r#"canvas{size=440, aspect=1.5, linen=15, seed=3, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=120, apply="knife"}}}"#;

    #[test]
    fn the_wet_look_shows_open_setting_tacky_and_dry() {
        let mut s = Session::new(W).unwrap();
        s.run(CANVAS).unwrap();
        // the left half laid yesterday, the right half just now; the top band stays bare
        s.run(r#"p = pile{{"lead white", 4}, {"raw umber", 1}, medium=0.3}
                  work(rect(0, 200, 480, 460), {hand="broad", pile=p}); wait(20 * 60)
                  work(rect(520, 200, 480, 460), {hand="broad", pile=p})"#).unwrap();
        let c = s.canvas().unwrap().clone();
        let f = c.window();
        let at = |x: f32, y: f32| f.index(x, y);
        let st = c.stages();
        assert_eq!(st[at(760.0, 420.0)], paint::Stage::Open);
        assert!(matches!(st[at(240.0, 420.0)], paint::Stage::Tacky | paint::Stage::Setting | paint::Stage::Dry), "{:?}", st[at(240.0, 420.0)]);
        assert_eq!(st[at(500.0, 60.0)], paint::Stage::Dry, "bare ground");
        let px = stage_colors(&c, &c.seen());
        let (open, bare) = (px[at(760.0, 420.0)], px[at(500.0, 60.0)]);
        assert!(open[2] > 2.0 * open[0], "open is blue: {open:?}");
        assert!((bare[0] - bare[2]).abs() < 1e-6 && bare[0] < 0.4, "dry is the picture dimmed to gray: {bare:?}");
        // the look itself, with its legend, leaves the canvas alone
        let before = bits(&c);
        let v = View::parse(&["--mode".into(), "wet,squint".into()]).unwrap();
        assert!(v.wet && v.squint);
        let out = root().join("target/easel-look-test/wet.jpg");
        assert_eq!(look(&c, &v, &out).unwrap(), (1120, 749));
        assert!(out.exists());
        assert_eq!(before, bits(&c));
    }

    #[test]
    fn looks_draw_the_grid_without_touching_the_canvas() {
        let mut s = Session::new(W).unwrap();
        s.run(CANVAS).unwrap();
        s.run(r#"b = brush("round", 4); b:load(pile{{"bone black", 1}}, 0.9); b:stroke({{100, 450}, {700, 400}})"#).unwrap();
        let dir = root().join("target/easel-look-test");
        let c = s.canvas().unwrap().clone();
        let before = bits(&c);
        let v = View::parse(&[]).unwrap();
        assert_eq!(look(&c, &v, &dir.join("plain.jpg")).unwrap(), (1120, 749), "enlarged 7x (rounded up)");
        let args: Vec<String> = ["--grid", "100", "--crop", "100,50,600,500", "--mirror"].iter().map(|s| s.to_string()).collect();
        let v = View::parse(&args).unwrap();
        assert_eq!(v.grid, Some(100.0));
        let (w, h) = look(&c, &v, &dir.join("grid.jpg")).unwrap();
        assert_eq!((w, h), (1040, 936), "a 500 x 450 unit crop at 0.16 px/unit (80 x 72 px), enlarged 13x (rounded up)");
        assert_eq!(before, bits(&c));
    }
}
