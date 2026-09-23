//! Looking at the canvas: small JPEGs an agent can read, with the studio's
//! viewing tricks (value, squint, mirror) and a preview of wet paint dried.

use paint::color::{linear_to_srgb, luminance};
use paint::{Canvas, Rgb};
use std::path::Path;

pub struct View {
    /// Crop in canvas units (x0, y0, x1, y1).
    pub crop: Option<[f32; 4]>,
    pub value: bool,
    pub squint: bool,
    pub mirror: bool,
    /// Show wet paint as it will look once it has leveled and dried.
    pub dried: bool,
    /// Also light the surface relief (implies `dried`).
    pub relief: Option<(f32, f32)>,
    /// Longest side of the image, px.
    pub size: usize,
}

impl Default for View {
    fn default() -> Self {
        View { crop: None, value: false, squint: false, mirror: false, dried: false, relief: None, size: 1000 }
    }
}

impl View {
    /// Parse look arguments: --crop x0,y0,x1,y1 --mode value,squint,mirror
    /// --dried (or --wet) --relief --size N.
    pub fn parse(args: &[String]) -> Result<View, String> {
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
                    let p: Vec<f32> = s.split(',').map(|t| t.trim().parse::<f32>()).collect::<Result<_, _>>().map_err(|_| format!("--crop {s}: want x0,y0,x1,y1 in units"))?;
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
                "--dried" | "--wet" | "--dry" => v.dried = true,
                "--relief" => {
                    v.dried = true;
                    v.relief = Some((f32::NAN, f32::NAN));
                }
                "--size" => v.size = next()?.parse().map_err(|_| "--size N (px)".to_string())?,
                o => return Err(format!("look: unknown argument {o:?} (--crop x0,y0,x1,y1 --mode value|squint|mirror --dried --relief --size N)")),
            }
            i += 1;
        }
        Ok(v)
    }
}

/// Render what the painter asked to see and write it as a JPEG.
pub fn look(c: &Canvas, relief_default: (f32, f32), v: &View, out: &Path) -> Result<(usize, usize), String> {
    let f = c.window();
    let px: Vec<Rgb> = if v.dried {
        let mut d = c.clone();
        d.dry();
        if let Some((s, g)) = v.relief {
            let (s, g) = if s.is_nan() { relief_default } else { (s, g) };
            d.relief(s, g);
        }
        d.pixels().to_vec()
    } else {
        c.seen()
    };
    // crop (units -> pixels of the held window)
    let (x0, y0, x1, y1) = match v.crop {
        None => (0, 0, f.w, f.h),
        Some([a, b, cc, d]) => {
            let s = f.scale;
            let cl = |u: f32, n: usize| ((u * s).round().max(0.0) as usize).min(n);
            let r = (cl(a, f.w), cl(b, f.h), cl(cc, f.w), cl(d, f.h));
            if r.2 <= r.0 || r.3 <= r.1 {
                return Err("--crop is empty or outside the canvas".into());
            }
            r
        }
    };
    let (cw, ch) = (x1 - x0, y1 - y0);
    // fit to size: average down, or enlarge by whole pixels (so pixels stay honest)
    let long = cw.max(ch);
    let (ow, oh, img): (usize, usize, Vec<Rgb>) = if long > v.size {
        let k = v.size as f32 / long as f32;
        let (ow, oh) = (((cw as f32 * k).round() as usize).max(1), ((ch as f32 * k).round() as usize).max(1));
        let mut img = vec![[0.0f32; 3]; ow * oh];
        for oy in 0..oh {
            let (sy0, sy1) = (y0 + oy * ch / oh, (y0 + (oy + 1) * ch / oh).max(y0 + oy * ch / oh + 1));
            for ox in 0..ow {
                let (sx0, sx1) = (x0 + ox * cw / ow, (x0 + (ox + 1) * cw / ow).max(x0 + ox * cw / ow + 1));
                let mut acc = [0.0f32; 3];
                for y in sy0..sy1 {
                    for x in sx0..sx1 {
                        let p = px[y * f.w + x];
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
        let k = (v.size / long).max(1);
        let (ow, oh) = (cw * k, ch * k);
        let img = (0..ow * oh).map(|i| px[(y0 + (i / ow) / k) * f.w + x0 + (i % ow) / k]).collect();
        (ow, oh, img)
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
    let buf: Vec<u8> = img.iter().flat_map(|p| p.map(|c| (linear_to_srgb(c) * 255.0).round().clamp(0.0, 255.0) as u8)).collect();
    if let Some(d) = out.parent() {
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    let file = std::fs::File::create(out).map_err(|e| e.to_string())?;
    let mut enc = image::codecs::jpeg::JpegEncoder::new_with_quality(std::io::BufWriter::new(file), 85);
    enc.encode(&buf, ow as u32, oh as u32, image::ExtendedColorType::Rgb8).map_err(|e| e.to_string())?;
    Ok((ow, oh))
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
