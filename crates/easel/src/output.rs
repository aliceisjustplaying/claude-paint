#[cfg(feature = "replay")]
use image::RgbImage;
use paint::Rgb;
use paint::color::linear_to_srgb;
use rayon::prelude::*;
use std::sync::LazyLock;

static SRGB: LazyLock<[u8; 65536]> = LazyLock::new(|| std::array::from_fn(|i| exact_channel(i as f32 / 65535.0)));

fn exact_channel(value: f32) -> u8 {
    (linear_to_srgb(value) * 255.0).round().clamp(0.0, 255.0) as u8
}

pub fn rgb8(pixels: &[Rgb], width: usize) -> Vec<u8> {
    rgb8_region(pixels, width, [0, 0, width, pixels.len() / width])
}

pub fn rgb8_exact(pixels: &[Rgb], width: usize) -> Vec<u8> {
    rgb8_exact_region(pixels, width, [0, 0, width, pixels.len() / width])
}

pub fn rgb8_exact_region(pixels: &[Rgb], width: usize, rect: [usize; 4]) -> Vec<u8> {
    convert(pixels, width, rect, exact_channel)
}

pub fn rgb8_region(pixels: &[Rgb], width: usize, rect: [usize; 4]) -> Vec<u8> {
    let table = &*SRGB;
    convert(pixels, width, rect, |value| table[(value.clamp(0.0, 1.0) * 65535.0).round() as usize])
}

fn convert(pixels: &[Rgb], width: usize, [x0, y0, x1, y1]: [usize; 4], channel: impl Fn(f32) -> u8 + Sync) -> Vec<u8> {
    let row_bytes = (x1 - x0) * 3;
    let mut bytes = vec![0; row_bytes * (y1 - y0)];
    bytes.par_chunks_mut(row_bytes).enumerate().for_each(|(y, row)| {
        let source = &pixels[(y + y0) * width + x0..(y + y0) * width + x1];
        for (dest, pixel) in row.chunks_exact_mut(3).zip(source) {
            for q in 0..3 {
                dest[q] = channel(pixel[q]);
            }
        }
    });
    bytes
}

pub fn average_linear(pixels: &[Rgb], width: usize, [x0, y0, x1, y1]: [usize; 4], ow: usize, oh: usize) -> Vec<Rgb> {
    let (cw, ch) = (x1 - x0, y1 - y0);
    let mut image = vec![[0.0; 3]; ow * oh];
    image.par_chunks_mut(ow).enumerate().for_each(|(oy, row)| {
        let (sy0, sy1) = (y0 + oy * ch / oh, (y0 + (oy + 1) * ch / oh).max(y0 + oy * ch / oh + 1));
        for (ox, pixel) in row.iter_mut().enumerate() {
            let (sx0, sx1) = (x0 + ox * cw / ow, (x0 + (ox + 1) * cw / ow).max(x0 + ox * cw / ow + 1));
            let mut acc = [0.0; 3];
            for y in sy0..sy1 {
                for x in sx0..sx1 {
                    let p = pixels[y * width + x];
                    for q in 0..3 {
                        acc[q] += p[q];
                    }
                }
            }
            let n = ((sy1 - sy0) * (sx1 - sx0)) as f32;
            *pixel = [acc[0] / n, acc[1] / n, acc[2] / n];
        }
    });
    image
}

#[cfg(feature = "replay")]
pub fn thumbnail(image: &RgbImage, width: u32, height: u32) -> RgbImage {
    let (iw, ih) = image.dimensions();
    if width == 0 || height == 0 || iw < width || ih < height {
        return image::imageops::thumbnail(image, width, height);
    }
    let (xr, yr) = (iw as f32 / width as f32, ih as f32 / height as f32);
    let row_bytes = width as usize * 3;
    let mut bytes = vec![0; row_bytes * height as usize];
    bytes.par_chunks_mut(row_bytes * 16).enumerate().for_each(|(stripe, dest)| {
        for (local_y, row) in dest.chunks_exact_mut(row_bytes).enumerate() {
            let y = stripe * 16 + local_y;
            let bottomf = y as f32 * yr;
            let bottom = (bottomf.ceil() as u32).min(ih - 1);
            let top = ((bottomf + yr).ceil() as u32).clamp(bottom, ih);
            for (x, pixel) in row.chunks_exact_mut(3).enumerate() {
                let leftf = x as f32 * xr;
                let left = (leftf.ceil() as u32).min(iw - 1);
                let right = ((leftf + xr).ceil() as u32).clamp(left, iw);
                let mut sum = [0u32; 3];
                for sy in bottom..top {
                    for sx in left..right {
                        let p = image.get_pixel(sx, sy).0;
                        for q in 0..3 {
                            sum[q] += p[q] as u32;
                        }
                    }
                }
                let n = (right - left) * (top - bottom);
                for q in 0..3 {
                    pixel[q] = ((sum[q] + n / 2) / n) as u8;
                }
            }
        }
    });
    RgbImage::from_raw(width, height, bytes).unwrap()
}
