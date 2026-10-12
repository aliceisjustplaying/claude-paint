//! Manual diagnostic for the fork-specific engine-6 drying-crack prototype.
//!
//! Inject this file as `defects::issue5_probe` into a disposable snapshot
//! of davidfwatson/ai-painting at b9dfeb2. It is deliberately not an
//! upstream test: it accesses private state and reports observations,
//! without asserting the prototype's physical assumptions as goldens.
//!
//! Set ISSUE5_OUT to a writable directory and run this one test. PNGs use
//! `seen()` without `Canvas::save()`, because save() advances drying.

use super::*;
use crate::color::{hex, linear_to_srgb};
use crate::wet::Paint;
use image::{Rgb, RgbImage};
use std::io::{self, Cursor, Write};
use std::path::{Path, PathBuf};

const SIZE: usize = 320;
const DAY: f32 = 24.0 * 60.0;
const THREE_YEARS: f32 = 3.0 * 365.0 * DAY;
const PAINT_BOX: (f32, f32, f32, f32) = (100.0, 100.0, 900.0, 900.0);
const MEASURE_BOX: (f32, f32, f32, f32) = (150.0, 150.0, 850.0, 850.0);

fn flat(engine: u32) -> Canvas {
    Canvas::new_window(SIZE, 1.0, hex("#d8cdb4"), None)
        .with_size_mm(80.0)
        .with_upright(false)
        .with_engine(engine)
}

fn dark_undercoat() -> Paint {
    // At 200 µm this is below the prototype's wrinkle critical thickness
    // (237.59 µm). Oil stays at its default: the model uses stiffness as
    // its fatness proxy, so these are synthetic material settings.
    Paint::body(hex("#201c18"))
        .with_stiff(0.30)
        .with_drying(0.5)
}

fn light_topcoat() -> Paint {
    Paint::body(hex("#ece6d6")).with_stiff(0.8).with_drying(2.0)
}

/// Lay a uniform film as the prototype's existing private tests do.
/// This isolates layer timing from brush load, gravity, and solvent.
fn film(c: &mut Canvas, paint: Paint, coats: f32, stroke: u32) {
    let f = c.f;
    let lat = paint.latent();
    let (x0, y0) = (
        f.index(PAINT_BOX.0, PAINT_BOX.1) % f.w,
        f.index(PAINT_BOX.0, PAINT_BOX.1) / f.w,
    );
    let (x1, y1) = (
        f.index(PAINT_BOX.2, PAINT_BOX.3) % f.w,
        f.index(PAINT_BOX.2, PAINT_BOX.3) / f.w,
    );
    for y in y0..y1 {
        for x in x0..x1 {
            let i = y * f.w + x;
            c.wet.vol[i] = coats;
            c.wet.lat[i] = lat;
            c.wet.hide[i] = paint.prop();
            c.wet.stroke[i] = stroke;
        }
    }
    c.wet.current = stroke;
    c.wet.touch(x0, y0, x1, y1);
}

fn undercoat(engine: u32) -> Canvas {
    let mut c = flat(engine);
    film(&mut c, dark_undercoat(), 8.0, 1);
    // Prototype touch-dry time for this uniform film is 30.438 days.
    c.wait(32.0 * DAY);
    c
}

fn topcoat(mut c: Canvas) -> Canvas {
    film(&mut c, light_topcoat(), 1.0, 2);
    c
}

fn luminance(p: crate::color::Rgb) -> f32 {
    0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2]
}

fn measured_indices(c: &Canvas) -> Vec<usize> {
    let f = c.f;
    let (x0, y0) = (
        f.index(MEASURE_BOX.0, MEASURE_BOX.1) % f.w,
        f.index(MEASURE_BOX.0, MEASURE_BOX.1) / f.w,
    );
    let (x1, y1) = (
        f.index(MEASURE_BOX.2, MEASURE_BOX.3) % f.w,
        f.index(MEASURE_BOX.2, MEASURE_BOX.3) / f.w,
    );
    (y0..y1)
        .flat_map(|y| (x0..x1).map(move |x| y * f.w + x))
        .collect()
}

fn png(c: &Canvas) -> RgbImage {
    let seen = c.seen();
    let buf = seen
        .iter()
        .flat_map(|p| p.map(|v| (linear_to_srgb(v) * 255.0).round().clamp(0.0, 255.0) as u8))
        .collect();
    RgbImage::from_raw(c.f.w as u32, c.f.h as u32, buf)
        .expect("RGB buffer matches canvas dimensions")
}

fn save_png(image: &RgbImage, path: impl AsRef<Path>) -> io::Result<()> {
    image.save(path).map_err(io::Error::other)
}

fn snapshot(
    c: &Canvas,
    case: &str,
    stage: &str,
    top_elapsed_min: f32,
    out: &Path,
    csv: &mut impl Write,
) -> io::Result<RgbImage> {
    let seen = c.seen();
    let indices = measured_indices(c);
    let mean_lum = indices
        .iter()
        .map(|&i| luminance(seen[i]) as f64)
        .sum::<f64>()
        / indices.len() as f64;
    let dark_fraction = indices
        .iter()
        .filter(|&&i| luminance(seen[i]) < 0.4)
        .count() as f64
        / indices.len() as f64;
    let soft_sum = c.fork.soft.iter().map(|p| p[0] as f64).sum::<f64>();
    let soft_max = c.fork.soft.iter().map(|p| p[0]).fold(0.0f32, f32::max);
    let open = c.fork.cracks.iter().filter(|p| p.shown > 0.0).count();
    let shown_sum = c.fork.cracks.iter().map(|p| p.shown as f64).sum::<f64>();
    writeln!(
        csv,
        "{case},{stage},{},{},{top_elapsed_min},{},{soft_sum:.9},{soft_max:.9},{},{open},{shown_sum:.9},{mean_lum:.9},{dark_fraction:.9}",
        c.engine,
        c.clock(),
        c.wet.clock.px.len(),
        c.fork.cracks.len()
    )?;
    let im = png(c);
    save_png(&im, out.join(format!("{case}-{stage}.png")))?;
    Ok(im)
}

fn checkpoint(c: &Canvas) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    c.write_state(&mut bytes, "")?;
    Ok(bytes)
}

fn max_color_delta(a: &Canvas, b: &Canvas) -> f32 {
    a.seen()
        .iter()
        .zip(b.seen())
        .flat_map(|(p, q)| (0..3).map(move |k| (p[k] - q[k]).abs()))
        .fold(0.0, f32::max)
}

fn compare(case: &str, a: &Canvas, b: &Canvas, csv: &mut impl Write) -> io::Result<()> {
    let checkpoints_equal = checkpoint(a)? == checkpoint(b)?;
    let seen_equal = a.seen() == b.seen();
    let png_equal = png(a).as_raw() == png(b).as_raw();
    let height_delta = a
        .height
        .iter()
        .zip(&b.height)
        .map(|(p, q)| (p - q).abs())
        .fold(0.0f32, f32::max);
    writeln!(
        csv,
        "{case},{checkpoints_equal},{seen_equal},{png_equal},{:.9},{height_delta:.9}",
        max_color_delta(a, b)
    )
}

fn threads_run(threads: usize) -> io::Result<Canvas> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .map_err(io::Error::other)?;
    Ok(pool.install(|| {
        let mut c = topcoat(undercoat(6));
        c.wait(5.0 * DAY);
        c
    }))
}

/// Reports synthetic behavior and implementation lifecycle checks. The
/// only failures here are execution/I/O failures, not physical assertions.
#[test]
fn study() -> io::Result<()> {
    let out = PathBuf::from(std::env::var_os("ISSUE5_OUT").ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "set ISSUE5_OUT to an output directory",
        )
    })?);
    std::fs::create_dir_all(&out)?;
    let mut observations = std::fs::File::create(out.join("observations.csv"))?;
    writeln!(
        observations,
        "case,stage,engine,clock_min,top_elapsed_min,clock_pixels,soft_sum_pixel_um,soft_max_um,active_crack_pixels,active_open_pixels,active_shown_sum,central_mean_linear_luminance,central_fraction_linear_luminance_below_0_4"
    )?;
    let mut comparisons = std::fs::File::create(out.join("comparisons.csv"))?;
    writeln!(
        comparisons,
        "case,full_checkpoint_bytes_equal,seen_linear_pixels_equal,seen_png_pixels_equal,max_seen_linear_channel_delta,max_height_um_delta"
    )?;

    // Main contact sheet: columns soft undercoat / three-years-hardened
    // undercoat / engine 5; rows fresh (60 min) / five days / sixty days.
    let soft = undercoat(6);
    let mut hardened = undercoat(6);
    hardened.wait(THREE_YEARS);
    let engine5 = undercoat(5);
    let mut contact =
        RgbImage::from_pixel((3 * SIZE) as u32, (3 * SIZE) as u32, Rgb([255, 255, 255]));
    for (col, (case, under)) in [("soft", soft), ("hardened", hardened), ("engine5", engine5)]
        .into_iter()
        .enumerate()
    {
        snapshot(&under, case, "undercoat", 0.0, &out, &mut observations)?;
        let mut c = topcoat(under);
        let mut elapsed = 0.0;
        for (row, (stage, target)) in [("fresh", 60.0), ("5d", 5.0 * DAY), ("60d", 60.0 * DAY)]
            .into_iter()
            .enumerate()
        {
            c.wait(target - elapsed);
            elapsed = target;
            let im = snapshot(&c, case, stage, elapsed, &out, &mut observations)?;
            for y in 0..SIZE as u32 {
                for x in 0..SIZE as u32 {
                    contact.put_pixel(
                        col as u32 * SIZE as u32 + x,
                        row as u32 * SIZE as u32 + y,
                        *im.get_pixel(x, y),
                    );
                }
            }
        }
    }
    save_png(&contact, out.join("contact-sheet.png"))?;

    // First dry() has no per-pixel clock to carry softness. wait(0) before
    // dry() initializes it without advancing time; otherwise settings match.
    let mut direct = flat(6);
    film(&mut direct, dark_undercoat(), 8.0, 1);
    direct.dry();
    snapshot(&direct, "first-dry", "direct", 0.0, &out, &mut observations)?;
    let mut initialized = flat(6);
    film(&mut initialized, dark_undercoat(), 8.0, 1);
    initialized.wait(0.0);
    initialized.dry();
    snapshot(
        &initialized,
        "first-dry",
        "wait0-then-dry",
        0.0,
        &out,
        &mut observations,
    )?;
    compare(
        "first-dry-direct-vs-wait0",
        &direct,
        &initialized,
        &mut comparisons,
    )?;

    // Same total top-film wait, different subdivision. Active records retire
    // after opening; the final image/luminance also measures persistent gaps.
    let mut one = topcoat(undercoat(6));
    one.wait(THREE_YEARS);
    snapshot(
        &one,
        "large-wait",
        "one-3y",
        THREE_YEARS,
        &out,
        &mut observations,
    )?;
    let mut split = topcoat(undercoat(6));
    split.wait(2.0 * DAY);
    snapshot(
        &split,
        "large-wait",
        "split-2d",
        2.0 * DAY,
        &out,
        &mut observations,
    )?;
    split.wait(THREE_YEARS - 2.0 * DAY);
    snapshot(
        &split,
        "large-wait",
        "split-3y",
        THREE_YEARS,
        &out,
        &mut observations,
    )?;
    compare(
        "one-3y-vs-2d-plus-remainder",
        &one,
        &split,
        &mut comparisons,
    )?;

    // Save while cracks exist and are visibly opening, then compare complete
    // serialized states and seen pixels after identical further waits.
    let mut uninterrupted = topcoat(undercoat(6));
    uninterrupted.wait(5.0 * DAY);
    snapshot(
        &uninterrupted,
        "checkpoint",
        "before-5d",
        5.0 * DAY,
        &out,
        &mut observations,
    )?;
    let saved = checkpoint(&uninterrupted)?;
    let (mut resumed, _) = Canvas::read_state(&mut Cursor::new(saved))?;
    snapshot(
        &resumed,
        "checkpoint",
        "loaded-5d",
        5.0 * DAY,
        &out,
        &mut observations,
    )?;
    compare(
        "checkpoint-immediate-roundtrip",
        &uninterrupted,
        &resumed,
        &mut comparisons,
    )?;
    uninterrupted.wait(55.0 * DAY);
    resumed.wait(55.0 * DAY);
    snapshot(
        &uninterrupted,
        "checkpoint",
        "uninterrupted-60d",
        60.0 * DAY,
        &out,
        &mut observations,
    )?;
    snapshot(
        &resumed,
        "checkpoint",
        "resumed-60d",
        60.0 * DAY,
        &out,
        &mut observations,
    )?;
    compare(
        "checkpoint-after-55d-more",
        &uninterrupted,
        &resumed,
        &mut comparisons,
    )?;

    let one_thread = threads_run(1)?;
    let four_threads = threads_run(4)?;
    snapshot(
        &one_thread,
        "threads",
        "one-5d",
        5.0 * DAY,
        &out,
        &mut observations,
    )?;
    snapshot(
        &four_threads,
        "threads",
        "four-5d",
        5.0 * DAY,
        &out,
        &mut observations,
    )?;
    compare(
        "one-thread-vs-four-threads-5d",
        &one_thread,
        &four_threads,
        &mut comparisons,
    )?;

    println!("Diagnostic outputs: {}", out.display());
    println!(
        "Wrinkle severity at 200 um: {}",
        wrinkling(200.0, 0.30, 0.5)
    );
    println!(
        "This is a synthetic fork diagnostic; it does not validate physical constants or upstream behavior."
    );
    Ok(())
}
