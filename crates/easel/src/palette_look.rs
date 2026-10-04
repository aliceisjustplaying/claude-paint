//! `look --palette`: the board as a painter sees it beside the easel. Every
//! heap on it is knifed out by the engine itself on a small board of its own:
//! a thick heap (the masstone, the body of the paint) and a smear dragged from
//! thick to thin across a dark stripe (the tint as it thins, how much it
//! hides), lit as the picture is in the gallery view, with its name.
//! Nothing here touches the painting.

use crate::board::Board;
use paint::{Canvas, Knife, Paint, Palette, hex};

const COLS: usize = 4;
/// A cell's size in board units (the board is 1000 units wide).
const CELL_W: f32 = 250.0;
const CELL_H: f32 = 230.0;

pub fn render(board: &Board, tubes: &Palette) -> Result<(usize, usize, Vec<u8>), String> {
    let heaps = board.live();
    if heaps.is_empty() {
        return Err("look --palette: no piles on the palette yet".into());
    }
    let rows = heaps.len().div_ceil(COLS);
    let aspect = 1000.0 / (rows as f32 * CELL_H);
    // a light, neutral board, so colors are judged against no color
    let mut c = Canvas::new(1600, aspect, hex("#d6d1c7")).with_size_mm(420.0).with_engine(tubes.engine);
    // the dark stripes, laid and dried first
    let dark = Paint::body(hex("#0d0c0c"));
    for n in 0..heaps.len() {
        let (cx, y) = ((n % COLS) as f32 * CELL_W, (n / COLS) as f32 * CELL_H + 140.0);
        // laid thick enough to be black, so a smear over it shows how much it hides
        for _ in 0..3 {
            let mut k = Knife::new(34.0);
            k.load(dark, 1.0);
            c.knife(&mut k, &[(cx + 100.0, y), (cx + 240.0, y)], (0.3, 0.3), Some(std::f32::consts::FRAC_PI_2), true, 0.0);
        }
    }
    c.dry();
    let mut labels = Vec::new();
    for (n, h) in heaps.iter().enumerate() {
        let (cx, cy) = ((n % COLS) as f32 * CELL_W, (n / COLS) as f32 * CELL_H);
        let mut mix = tubes.pile(h.fractions());
        mix.solvent = h.solvent;
        mix.oil_rate = h.oil_rate;
        let paint = mix.laid(h.medium);
        // the heap: a thick slab, the bead left standing where the knife lifts
        let mut k = Knife::new(46.0);
        k.load(paint, 1.0);
        c.knife(&mut k, &[(cx + 40.0, cy + 70.0), (cx + 70.0, cy + 76.0)], (0.12, 0.08), None, true, 0.25);
        // the smear: from thick to thin across the light board and the dark stripe
        let mut s = Knife::new(30.0);
        s.load(paint, 0.55);
        c.knife(&mut s, &[(cx + 120.0, cy + 60.0), (cx + 165.0, cy + 130.0), (cx + 215.0, cy + 205.0)], (0.35, 1.0), None, true, 0.05);
        let name = h.name.clone().unwrap_or_else(|| format!("pile {}", h.id));
        let mut tubes_line: Vec<(usize, f32)> = h.fractions();
        tubes_line.sort_by(|a, b| b.1.total_cmp(&a.1));
        let tl = tubes_line.iter().take(3).map(|&(i, f)| format!("{} {:.0}", short(tubes.tubes[i].name), 100.0 * f)).collect::<Vec<_>>().join(" ");
        let mut l = format!("{}. {}", n + 1, name);
        if h.dirt_share() > 0.005 {
            l += &format!(" ({:.0}% dirt)", 100.0 * h.dirt_share());
        }
        labels.push((cx / 1000.0, (cy + 4.0) / (rows as f32 * CELL_H), l));
        labels.push((cx / 1000.0, (cy + 210.0) / (rows as f32 * CELL_H), tl));
    }
    let v = crate::look::View { light: Some(crate::GALLERY_LIGHT), ..Default::default() };
    let (w, hgt, png) = crate::look::render(&c, &v)?;
    let png = crate::look::label(&png, &labels)?;
    Ok((w, hgt, png))
}

/// A tube's name short enough for a label ("ultramarine blue" -> "ultram").
fn short(name: &str) -> String {
    let words: Vec<&str> = name.split_whitespace().collect();
    match words.as_slice() {
        [one] => one.chars().take(7).collect(),
        [a, b, ..] if ["white", "blue", "yellow", "green", "lake", "black", "violet", "red"].contains(b) => format!("{}{}", a.chars().take(5).collect::<String>(), b.chars().next().unwrap_or(' ')),
        [a, ..] => a.chars().take(7).collect(),
        [] => String::new(),
    }
}
