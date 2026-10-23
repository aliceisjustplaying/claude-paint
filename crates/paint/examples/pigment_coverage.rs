use image::{Rgb as Pixel, RgbImage};
use paint::{Canvas, Knife, Mixture, Palette, Rgb, Tube, color::linear_to_srgb};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

const WIDTH: usize = 80;
const HEIGHT: usize = 48;
const LABEL: usize = 300;
const COLUMN: usize = 106;
const ROW: usize = 62;
const TOP: usize = 126;
const LIGHT: (f32, f32, f32) = (115.0, 55.0, 1.0);
const MEASURED: [&str; 15] = [
    "lead white", "zinc white", "bone black", "cobalt blue", "ultramarine blue",
    "Prussian blue", "vermilion", "orange vermilion", "Chinese vermilion",
    "chrome yellow", "lemon chrome", "viridian", "emerald green", "red lead",
    "carmine lake",
];

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let output = args.next().ok_or("usage: pigment_coverage OUTPUT_DIRECTORY [ENGINE]")?;
    let engine = args.next().map(|v| v.into_string().map_err(|_| "engine must be UTF-8")?.parse::<u32>().map_err(|_| "engine must be an integer")).transpose()?.unwrap_or(paint::PIGMENT_MIXER);
    if !paint::supported_engine(engine) {
        return Err("engine must be a supported engine version".into());
    }
    if args.next().is_some() {
        return Err("usage: pigment_coverage OUTPUT_DIRECTORY [ENGINE]".into());
    }
    let output = Path::new(&output);
    fs::create_dir_all(output)?;
    let catalog = paint::palette::catalog();
    let mut boxes: Vec<_> = Palette::box_names()
        .into_iter()
        .map(|name| Palette::named_box(name).expect("listed box"))
        .collect();
    let covered: BTreeSet<_> = boxes.iter().flat_map(|p| p.tubes.iter().map(|t| t.name)).collect();
    let missing: Vec<_> = catalog.iter().filter(|t| !covered.contains(t.name)).cloned().collect();
    if !missing.is_empty() {
        boxes.push(Palette::new("every", missing).named("catalog"));
    }
    let mut csv = BufWriter::new(File::create(output.join("swatches.csv"))?);
    writeln!(csv, "sheet,box,tube,pigment,measured_oil_reference,column,reference,tube_volume,reference_volume,medium,requested_gap_um,ground_linear,engine,light_azimuth,light_elevation,light_strength,width_px,height_px,width_mm,roi_x0,roi_y0,roi_x1,roi_y1,pile_r,pile_g,pile_b,seen_r,seen_g,seen_b,gallery_r,gallery_g,gallery_b,gallery_srgb_r,gallery_srgb_g,gallery_srgb_b,wet_um_mean,wet_um_min,wet_um_max,covered_fraction,checkpoint_sha256")?;
    let sheets = boxes.len();
    for palette in boxes {
        render_sheet(palette, &catalog, output, &mut csv, engine)?;
    }
    csv.flush()?;
    println!(
        "{sheets} sheets; {} catalog tubes ({} without measured oil references); {}",
        catalog.len(),
        catalog.iter().filter(|t| !MEASURED.contains(&t.name)).count(),
        output.join("swatches.csv").display()
    );
    Ok(())
}

fn render_sheet(
    mut palette: Palette,
    catalog: &[Tube],
    output: &Path,
    csv: &mut impl Write,
    engine: u32,
) -> Result<(), Box<dyn Error>> {
    let rows = palette.tubes.len();
    let extra = ["lead white", "cobalt blue", "ultramarine blue"]
        .into_iter()
        .filter(|name| !palette.tubes.iter().any(|t| t.name == *name))
        .map(|name| catalog.iter().find(|t| t.name == name).expect("reference tube").clone())
        .collect();
    palette = palette.with(extra);
    palette.engine = engine;
    palette.glaze_film = engine >= paint::PIGMENT_MIXER;
    let index = |name| palette.tubes.iter().position(|t| t.name == name).expect("reference in palette");
    let white = index("lead white");
    let cobalt = index("cobalt blue");
    let ultramarine = index("ultramarine blue");
    let filename = format!("{}.png", palette.name.replace(' ', "-"));
    let mut sheet = RgbImage::from_pixel(
        (LABEL + 7 * COLUMN + 12) as u32,
        (TOP + rows * ROW + 12) as u32,
        Pixel([224, 224, 224]),
    );
    text(&mut sheet, 12, 12, palette.name, 2);
    text(&mut sheet, 12, 34, &format!("ENGINE {engine} / GALLERY 115,55,1 / FRESH WET PAINT / SMOOTH SEALED SUPPORT"), 1);
    text(&mut sheet, 12, 48, "RATIOS: TUBE:REFERENCE BY VOLUME / TINT REFERENCE: MEASURED LEAD WHITE", 1);
    text(&mut sheet, 12, 62, "MIX: MEASURED COBALT BLUE; COBALT ROW USES MEASURED ULTRAMARINE BLUE", 1);
    text(&mut sheet, 12, 76, "KNIFE GAP: 150 UM; GLAZE: 8 UM + 0.70 MEDIUM / CSV: CENTER-HALF MEANS", 1);
    text(&mut sheet, 12, 102, "TUBE", 2);
    for (column, (title, detail)) in [
        ("MASSTONE", "THICK"),
        ("TINT 1:1", "LEAD WHITE"),
        ("TINT 1:4", "LEAD WHITE"),
        ("TINT 1:16", "LEAD WHITE"),
        ("GLAZE", "BLACK 0.01"),
        ("GLAZE", "WHITE 0.90"),
        ("MIX 1:1", "BLUE"),
    ].into_iter().enumerate() {
        text(&mut sheet, LABEL + column * COLUMN, 99, title, 1);
        text(&mut sheet, LABEL + column * COLUMN, 112, detail, 1);
    }
    for row in 0..rows {
        let tube = &palette.tubes[row];
        let y = TOP + row * ROW;
        text(&mut sheet, 12, y + 8, tube.name, 2);
        let partner = if row == cobalt { ultramarine } else { cobalt };
        let partner_name = palette.tubes[partner].name;
        for column in 0..7 {
            let (name, reference, fraction, medium, gap, ground) = match column {
                0 => ("masstone", "", 0.0, 0.0, 150.0, 0.18),
                1 => ("tint_1_1", "lead white", 0.5, 0.0, 150.0, 0.18),
                2 => ("tint_1_4", "lead white", 4.0 / 5.0, 0.0, 150.0, 0.18),
                3 => ("tint_1_16", "lead white", 16.0 / 17.0, 0.0, 150.0, 0.18),
                4 => ("glaze_black", "", 0.0, 0.70, 8.0, 0.01),
                5 => ("glaze_white", "", 0.0, 0.70, 8.0, 0.90),
                _ => ("mix_1_1", partner_name, 0.5, 0.0, 150.0, 0.18),
            };
            let reference_index = if column == 6 { partner } else { white };
            let parts = if fraction == 0.0 || row == reference_index {
                vec![(row, 1.0)]
            } else {
                vec![(row, 1.0 - fraction), (reference_index, fraction)]
            };
            let pile = palette.pile(parts);
            let canvas = swatch(&pile, medium, gap, ground, engine);
            let seen = canvas.seen();
            let gallery = canvas.seen_lit(LIGHT.0, LIGHT.1, LIGHT.2);
            for py in 0..HEIGHT {
                for px in 0..WIDTH {
                    sheet.put_pixel(
                        (LABEL + column * COLUMN + px) as u32,
                        (y + py) as u32,
                        Pixel(gallery[py * WIDTH + px].map(channel)),
                    );
                }
            }
            for value in [filename.as_str(), palette.name, tube.name, tube.pigment] {
                write!(csv, "\"{}\",", value.replace('"', "\"\""))?;
            }
            write!(csv, "{},{name},", MEASURED.contains(&tube.name))?;
            write!(csv, "\"{}\",{:.9},{:.9},{medium},{gap},{ground},{engine},{},{},{},{WIDTH},{HEIGHT},50,{},{},{},{},",
                reference, 1.0 - fraction, fraction, LIGHT.0, LIGHT.1, LIGHT.2,
                WIDTH / 4, HEIGHT / 4, WIDTH * 3 / 4, HEIGHT * 3 / 4)?;
            let (mut unlit, mut lit) = ([0.0; 3], [0.0; 3]);
            let (mut wet_sum, mut wet_min, mut wet_max, mut covered, mut count) =
                (0.0f32, f32::INFINITY, 0.0f32, 0usize, 0usize);
            let frame = canvas.window();
            for py in HEIGHT / 4..HEIGHT * 3 / 4 {
                for px in WIDTH / 4..WIDTH * 3 / 4 {
                    let i = py * WIDTH + px;
                    for k in 0..3 {
                        unlit[k] += seen[i][k];
                        lit[k] += gallery[i][k];
                    }
                    let wet = canvas.wet_um(frame.ux(px), frame.uy(py));
                    assert!(wet.is_finite(), "nonfinite film thickness");
                    wet_sum += wet;
                    wet_min = wet_min.min(wet);
                    wet_max = wet_max.max(wet);
                    covered += usize::from(wet > 0.05);
                    count += 1;
                }
            }
            let unlit: Rgb = unlit.map(|v| v / count as f32);
            let lit: Rgb = lit.map(|v| v / count as f32);
            assert!(wet_sum > 0.0, "empty swatch: {} / {} / {name}", palette.name, tube.name);
            for rgb in [pile.color, unlit, lit, lit.map(|v| linear_to_srgb(v) * 255.0)] {
                for v in rgb {
                    assert!(v.is_finite(), "nonfinite swatch: {} / {} / {name}", palette.name, tube.name);
                    write!(csv, "{v:.9},")?;
                }
            }
            let mut checkpoint = Vec::new();
            canvas.write_state(&mut checkpoint, "pigment coverage regression")?;
            writeln!(csv, "{:.9},{wet_min:.9},{wet_max:.9},{:.9},{:x}", wet_sum / count as f32, covered as f32 / count as f32, Sha256::digest(&checkpoint))?;
        }
    }
    sheet.save(output.join(&filename))?;
    println!("{}: {rows} tubes", output.join(filename).display());
    Ok(())
}

fn swatch(pile: &Mixture, medium: f32, gap_um: f32, ground: f32, engine: u32) -> Canvas {
    let mut canvas = Canvas::new_window(WIDTH, WIDTH as f32 / HEIGHT as f32, [ground; 3], None)
        .with_engine(engine)
        .with_glaze_film(engine >= paint::PIGMENT_MIXER)
        .with_size_mm(50.0);
    let mut knife = Knife::new(1000.0);
    knife.load(pile.paint(medium), 1.0);
    let pressure = 1.0 - (gap_um / 300.0).powf(2.0 / 3.0);
    let y = canvas.height() * 0.5;
    canvas.knife(&mut knife, &[(-150.0, y), (1150.0, y)], (pressure, pressure), None, true, 0.0);
    canvas
}

fn channel(value: f32) -> u8 {
    (linear_to_srgb(value) * 255.0).round().clamp(0.0, 255.0) as u8
}

fn text(image: &mut RgbImage, x: usize, y: usize, value: &str, scale: usize) {
    for (i, c) in value.to_ascii_uppercase().chars().enumerate() {
        let glyph: [u8; 7] = match c {
            'A' => [14, 17, 17, 31, 17, 17, 17],
            'B' => [30, 17, 17, 30, 17, 17, 30],
            'C' => [14, 17, 16, 16, 16, 17, 14],
            'D' => [30, 17, 17, 17, 17, 17, 30],
            'E' => [31, 16, 16, 30, 16, 16, 31],
            'F' => [31, 16, 16, 30, 16, 16, 16],
            'G' => [14, 17, 16, 23, 17, 17, 15],
            'H' => [17, 17, 17, 31, 17, 17, 17],
            'I' => [31, 4, 4, 4, 4, 4, 31],
            'J' => [7, 2, 2, 2, 18, 18, 12],
            'K' => [17, 18, 20, 24, 20, 18, 17],
            'L' => [16, 16, 16, 16, 16, 16, 31],
            'M' => [17, 27, 21, 21, 17, 17, 17],
            'N' => [17, 25, 21, 19, 17, 17, 17],
            'O' => [14, 17, 17, 17, 17, 17, 14],
            'P' => [30, 17, 17, 30, 16, 16, 16],
            'Q' => [14, 17, 17, 17, 21, 18, 13],
            'R' => [30, 17, 17, 30, 20, 18, 17],
            'S' => [15, 16, 16, 14, 1, 1, 30],
            'T' => [31, 4, 4, 4, 4, 4, 4],
            'U' => [17, 17, 17, 17, 17, 17, 14],
            'V' => [17, 17, 17, 17, 17, 10, 4],
            'W' => [17, 17, 17, 21, 21, 21, 10],
            'X' => [17, 17, 10, 4, 10, 17, 17],
            'Y' => [17, 17, 10, 4, 4, 4, 4],
            'Z' => [31, 1, 2, 4, 8, 16, 31],
            '0' => [14, 17, 19, 21, 25, 17, 14],
            '1' => [4, 12, 4, 4, 4, 4, 14],
            '2' => [14, 17, 1, 2, 4, 8, 31],
            '3' => [30, 1, 1, 14, 1, 1, 30],
            '4' => [2, 6, 10, 18, 31, 2, 2],
            '5' => [31, 16, 16, 30, 1, 1, 30],
            '6' => [14, 16, 16, 30, 17, 17, 14],
            '7' => [31, 1, 2, 4, 8, 8, 8],
            '8' => [14, 17, 17, 14, 17, 17, 14],
            '9' => [14, 17, 17, 15, 1, 1, 14],
            ':' => [0, 4, 4, 0, 4, 4, 0],
            '.' => [0, 0, 0, 0, 0, 4, 4],
            ',' => [0, 0, 0, 0, 4, 4, 8],
            '-' => [0, 0, 0, 31, 0, 0, 0],
            '/' => [1, 2, 2, 4, 8, 8, 16],
            '+' => [0, 4, 4, 31, 4, 4, 0],
            ';' => [0, 4, 4, 0, 4, 4, 8],
            '\'' => [4, 4, 8, 0, 0, 0, 0],
            _ => [0; 7],
        };
        for (gy, bits) in glyph.into_iter().enumerate() {
            for gx in 0..5 {
                if bits & (1 << (4 - gx)) == 0 {
                    continue;
                }
                for dy in 0..scale {
                    for dx in 0..scale {
                        let px = (x + (i * 6 + gx) * scale + dx) as u32;
                        let py = (y + gy * scale + dy) as u32;
                        if px < image.width() && py < image.height() {
                            image.put_pixel(px, py, Pixel([35, 35, 35]));
                        }
                    }
                }
            }
        }
    }
}
