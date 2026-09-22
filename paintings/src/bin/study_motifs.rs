//! Study sheet for the oak and Rückenfigur generators.

use paint::{Oak, Style, hex};
use paintings::figures;

fn main() {
    let o = paintings::run::Run::new("study_motifs");
    let mut c = Style::friedrich().prepare(o.width, 1.6, 1);
    let h = c.height();

    // top row: oaks with different settings
    let ground = h * 0.5;
    for (i, (gnarl, broken, lean, depth)) in
        [(0.5, 0.08, 0.0, 5), (0.7, 0.18, 0.08, 5), (0.9, 0.35, -0.15, 4), (0.7, 0.12, 0.0, 5), (1.0, 0.3, 0.2, 4), (0.6, 0.05, -0.05, 5)]
            .iter()
            .enumerate()
    {
        let x = 90.0 + i as f32 * 165.0;
        let mut oak = Oak::new((x, ground), 250.0, 11 + i as u64 * 7);
        oak.gnarl = *gnarl;
        oak.broken = *broken;
        oak.lean = *lean;
        oak.depth = *depth;
        oak.roots = if i == 4 { 4 } else { 2 };
        oak.paint(&mut c, hex("#2a2420"), Some(hex("#8a7f6a")), 100 + i as u64);
    }

    // bottom row: figures, each written as brush gestures
    let gy = h * 0.96;
    let fh = 190.0;
    let (robe, pale, rim) = (hex("#14120f"), hex("#a8986f"), hex("#8f8a78"));
    figures::monk(&mut c, (140.0, gy), fh, robe, pale, None, 1);
    // the two men: the youth, a little higher, leans on the older man
    let sh = figures::man_in_cape(&mut c, (520.0, gy), fh, hex("#3a3f33"), hex("#15130f"), hex("#2a2119"), Some(rim), 2);
    figures::youth_in_frock(&mut c, (455.0, gy - 8.0), fh * 0.99, hex("#394034"), hex("#15130f"), hex("#33281c"), hex("#d6cfbd"), Some(sh), Some(rim), 3);
    // the monk again, small, as he is in the painting (44 units)
    figures::monk(&mut c, (800.0, gy), 44.0, robe, pale, None, 4);

    c.relief(0.5, 0.0);
    o.save(&mut c);
}
