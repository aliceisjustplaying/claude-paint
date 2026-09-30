//! Looking at the canvas: bounded PNGs an agent can read. The look shows
//! the paint on the canvas as it is now (the dry picture with the wet
//! paint on it as laid), whole or a window of it, and a few ways a painter
//! looks at a picture: in grays, squinting and in a mirror. A grid in
//! canvas units can be laid over the look, as a painter squares up a
//! drawing; it never touches the canvas.

use image::ImageEncoder;
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
    /// Whole-view longest side, px (None: 1000, capped at 1600). Crops stay 1:1.
    pub size: Option<usize>,
    /// Coordinate grid: Some(0) picks the step from the zoom.
    pub grid: Option<f32>,
}

const LOOK_ARGS: &str = "--crop x0,y0,x1,y1 --mode value,squint,mirror --grid [step] --size N";

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
                    let p: Vec<f32> = s
                        .split(',')
                        .map(|t| t.trim().parse::<f32>())
                        .collect::<std::result::Result<_, _>>()
                        .map_err(|_| format!("--crop {s}: want x0,y0,x1,y1 in units"))?;
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
                            o => return Err(format!("--mode {o}: normal, value, squint, mirror (comma-separated)")),
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

/// Render a PNG to `out`: whole views fit 1600 px and 3 MB; crops stay native, at most 1200 px per side.
pub fn look(c: &Canvas, v: &View, out: &Path) -> std::result::Result<(usize, usize), String> {
    let (w, h, png) = render(c, v)?;
    if let Some(d) = out.parent().filter(|d| !d.as_os_str().is_empty()) {
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    std::fs::write(out, png).map_err(|e| e.to_string())?;
    Ok((w, h))
}

/// `look`'s PNG bytes and size, written nowhere.
pub fn render(c: &Canvas, v: &View) -> std::result::Result<(usize, usize, Vec<u8>), String> {
    let f = c.window();
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
    if v.crop.is_some() && (cw > 1200 || ch > 1200) {
        return Err("--crop exceeds 1200 pixels per side; choose a smaller crop (crops stay 1:1)".into());
    }
    if long == 0 || cw == 0 || ch == 0 {
        return Err("look: canvas is empty".into());
    }
    let mut size = if v.crop.is_some() { long } else { v.size.unwrap_or(1000).clamp(1, 1600) };
    let px = c.seen();
    loop {
        // Average down from the original canvas on each attempt; never enlarge.
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
        if let Some(step) = v.grid {
            draw_grid(&mut im, &map, step, fs)?;
        }
        let buf: Vec<u8> = im.px.iter().flat_map(|p| p.map(|c| (linear_to_srgb(c) * 255.0).round().clamp(0.0, 255.0) as u8)).collect();
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
    #[cfg(tube_box)]
    use crate::session::Session;
    use std::path::PathBuf;

    fn out_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/easel-look-test")
    }

    #[cfg(tube_box)]
    const W: usize = 160;

    #[cfg(tube_box)]
    fn bits(c: &Canvas) -> Vec<u32> {
        c.seen()
            .iter()
            .flat_map(|p| p.map(f32::to_bits))
            .chain(c.surface_um().iter().map(|v| v.to_bits()))
            .collect()
    }

    #[cfg(tube_box)]
    const CANVAS: &str = r#"canvas{size=440, aspect=1.5, linen=15, seed=3, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=120, apply="knife"}}}"#;

    #[test]
    #[cfg(tube_box)]
    fn looks_draw_the_grid_without_touching_the_canvas() {
        let mut s = Session::new(W).unwrap();
        s.run(CANVAS).unwrap();
        s.run(r#"b = brush("round", 4); b:load(pile{{"bone black", 1}}, 0.9); b:stroke({{100, 450}, {700, 400}})"#)
            .unwrap();
        let dir = out_dir();
        let c = s.canvas().unwrap().clone();
        let before = bits(&c);
        let v = View::parse(&[]).unwrap();
        assert_eq!(look(&c, &v, &dir.join("plain.png")).unwrap(), (160, 107), "never enlarged");
        let args: Vec<String> = ["--grid", "100", "--crop", "100,50,600,500", "--mirror"].iter().map(|s| s.to_string()).collect();
        let v = View::parse(&args).unwrap();
        assert_eq!(v.grid, Some(100.0));
        let (w, h) = look(&c, &v, &dir.join("grid.png")).unwrap();
        assert_eq!((w, h), (80, 72), "crop stays at one output pixel per canvas pixel");
        assert_eq!(before, bits(&c));
    }
    #[test]
    fn grids_finer_than_a_pixel_are_refused() {
        let c = Canvas::new_window(100, 1.0, [0.2; 3], None);
        let out = out_dir().join("fine-grid.png");
        let grid = |g: f32| look(&c, &View { grid: Some(g), ..View::default() }, &out);
        let err = grid(1e-6).expect_err("a grid finer than a pixel");
        assert!(err.contains("want at least 10.00 units"), "{err}");
        assert_eq!(grid(10.0).unwrap(), (100, 100), "one line per pixel still draws");
    }

    #[test]
    fn whole_looks_are_bounded_pngs_without_enlargement() {
        for (name, width, aspect, size, expected) in [
            ("small", 40, 2.0, None, (40, 20)),
            ("wide", 2000, 2.0, Some(9999), (1600, 800)),
            ("tall", 100, 0.05, Some(9999), (80, 1600)),
            ("requested", 100, 2.0, Some(40), (40, 20)),
        ] {
            let c = Canvas::new_window(width, aspect, [0.2; 3], None);
            let out = out_dir().join(format!("{name}.png"));
            assert_eq!(look(&c, &View { size, ..View::default() }, &out).unwrap(), expected);
            let bytes = std::fs::read(out).unwrap();
            assert_eq!(image::guess_format(&bytes).unwrap(), image::ImageFormat::Png);
            let decoded = image::load_from_memory(&bytes).unwrap();
            assert_eq!((decoded.width() as usize, decoded.height() as usize), expected);
        }
    }

    #[test]
    fn crops_keep_native_pixels_and_reject_either_oversize_axis() {
        let mut c = Canvas::new_window(1300, 1.0, [0.0; 3], None);
        c.apply(|x, y, _| if x < 500.0 && y < 500.0 { [1.0, 0.0, 0.0] } else { [0.0, 0.0, 1.0] });
        let out = out_dir().join("native-crop.png");
        for size in [None, Some(40), Some(9999)] {
            let v = View {
                crop: Some([0.0, 0.0, 1200.0 / 1.3, 1200.0 / 1.3]),
                size,
                ..View::default()
            };
            assert_eq!(look(&c, &v, &out).unwrap(), (1200, 1200));
            let decoded = image::load_from_memory(&std::fs::read(&out).unwrap()).unwrap().to_rgb8();
            assert_eq!(decoded.dimensions(), (1200, 1200));
            assert_eq!(decoded.get_pixel(100, 100).0, [255, 0, 0]);
            assert_eq!(decoded.get_pixel(1000, 1000).0, [0, 0, 255]);
        }
        for crop in [[0.0, 0.0, 1201.0 / 1.3, 100.0], [0.0, 0.0, 100.0, 1201.0 / 1.3]] {
            let v = View {
                crop: Some(crop),
                ..View::default()
            };
            let err = look(&c, &v, &out).expect_err("oversize crop must not silently scale");
            assert!(err.contains("1200"), "{err}");
        }
    }

    #[test]
    fn whole_pngs_reduce_dimensions_to_meet_the_byte_budget() {
        use image::ImageEncoder;
        let mut c = Canvas::new_window(1600, 1.0, [0.0; 3], None);
        c.apply(|x, y, _| {
            let mut n = (x.to_bits() as u64) << 32 | y.to_bits() as u64;
            std::array::from_fn(|_| {
                n = n.wrapping_add(0x9e3779b97f4a7c15);
                let mut z = n;
                z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
                z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
                paint::color::srgb_to_linear(((z ^ (z >> 31)) & 255) as f32 / 255.0)
            })
        });
        let raw: Vec<u8> = c.seen().iter().flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round() as u8)).collect();
        let mut full = Vec::new();
        image::codecs::png::PngEncoder::new(&mut full)
            .write_image(&raw, 1600, 1600, image::ExtendedColorType::Rgb8)
            .unwrap();
        assert!(full.len() > 3_000_000, "fixture must exercise byte-budget reduction");
        let out = out_dir().join("byte-budget.png");
        let (w, h) = look(
            &c,
            &View {
                size: Some(1600),
                ..View::default()
            },
            &out,
        )
        .unwrap();
        let bytes = std::fs::read(out).unwrap();
        assert_eq!(image::guess_format(&bytes).unwrap(), image::ImageFormat::Png);
        assert!(bytes.len() <= 3_000_000, "{} bytes", bytes.len());
        assert!(w < 1600 && h == w, "must reduce dimensions, got {w}x{h}");
        let decoded = image::load_from_memory(&bytes).unwrap();
        assert_eq!((decoded.width() as usize, decoded.height() as usize), (w, h));
    }
}
