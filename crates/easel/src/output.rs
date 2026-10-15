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

#[cfg(test)]
mod tests {
    use super::*;

    fn pixels(width: usize, height: usize) -> Vec<Rgb> {
        (0..width * height).map(|i| [(i * 71 % 997) as f32 / 996.0, (i * 13 % 101) as f32 / 100.0, (i * 43 % 163) as f32 / 162.0]).collect()
    }

    #[test]
    fn exact_rows_keep_serial_bytes_and_crop_coordinates() {
        let pixels = pixels(29, 19);
        let expected: Vec<u8> = pixels.iter().flat_map(|p| p.map(exact_channel)).collect();
        for threads in [1, 4] {
            rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap().install(|| {
                assert_eq!(rgb8_exact(&pixels, 29), expected);
                let crop = rgb8_exact_region(&pixels, 29, [3, 5, 27, 18]);
                let expected_crop: Vec<u8> = (5..18).flat_map(|y| expected[y * 29 * 3 + 9..y * 29 * 3 + 81].iter().copied()).collect();
                assert_eq!(crop, expected_crop);
                assert_eq!(rgb8_region(&pixels, 29, [3, 5, 27, 18]), rgb8(&(5..18).flat_map(|y| pixels[y * 29 + 3..y * 29 + 27].iter().copied()).collect::<Vec<_>>(), 24));
            });
        }
    }

    #[test]
    fn lookup_stays_within_one_byte_at_every_bucket_edge() {
        let table = &*SRGB;
        for i in 0..65536 {
            let center = i as f32 / 65535.0;
            let low = ((i as f32 - 0.5) / 65535.0).max(0.0);
            let high = ((i as f32 + 0.5) / 65535.0).min(1.0);
            for value in [low.next_down(), low, low.next_up(), center, high.next_down(), high, high.next_up()] {
                let byte = table[(value.clamp(0.0, 1.0) * 65535.0).round() as usize];
                assert!(byte.abs_diff(exact_channel(value)) <= 1, "{value}");
            }
        }
        let special = [[f32::NEG_INFINITY, f32::INFINITY, f32::NAN], [-1.0, -0.0, 2.0]];
        assert_eq!(rgb8(&special, 2), rgb8_exact(&special, 2));
    }

    #[test]
    fn linear_averaging_keeps_each_pixels_serial_accumulation_order() {
        let pixels = pixels(29, 23);
        for (rect, ow, oh) in [([3, 5, 28, 21], 13, 7), ([3, 5, 28, 21], 25, 16), ([0, 0, 29, 23], 1, 1)] {
            let [x0, y0, x1, y1] = rect;
            let (cw, ch) = (x1 - x0, y1 - y0);
            let mut expected = Vec::new();
            for oy in 0..oh {
                for ox in 0..ow {
                    let mut acc = [0.0; 3];
                    let (sx0, sx1) = (x0 + ox * cw / ow, x0 + (ox + 1) * cw / ow);
                    let (sy0, sy1) = (y0 + oy * ch / oh, y0 + (oy + 1) * ch / oh);
                    for y in sy0..sy1 {
                        for x in sx0..sx1 {
                            for q in 0..3 {
                                acc[q] += pixels[y * 29 + x][q];
                            }
                        }
                    }
                    let n = ((sx1 - sx0) * (sy1 - sy0)) as f32;
                    expected.push(acc.map(|v| v / n));
                }
            }
            for threads in [1, 4] {
                rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap().install(|| {
                    assert_eq!(average_linear(&pixels, 29, rect, ow, oh), expected);
                });
            }
        }
    }

    #[cfg(feature = "replay")]
    #[test]
    fn striped_thumbnails_match_image_on_odd_sizes_and_stripe_boundaries() {
        for (iw, ih, ow, oh) in [(241, 157, 193, 127), (241, 157, 73, 49), (240, 160, 192, 128), (17, 33, 1, 17), (1, 49, 1, 17), (29, 19, 29, 19), (29, 19, 41, 31), (0, 0, 4, 3), (29, 19, 0, 0)] {
            let image = RgbImage::from_fn(iw, ih, |x, y| image::Rgb([(x * 71 + y * 13) as u8, (x * 19 + y * 61) as u8, (x * 43 + y * 31) as u8]));
            let expected = image::imageops::thumbnail(&image, ow, oh);
            for threads in [1, 4] {
                rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap().install(|| {
                    assert_eq!(thumbnail(&image, ow, oh), expected, "{iw}x{ih} -> {ow}x{oh}");
                });
            }
        }
    }
}
