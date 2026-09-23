//! Study sheet for the Rückenfiguren (trees: see `study_trees`). Every
//! figure is painted in paints mixed on the Friedrich palette.

use paint::{Paint, Style, hex};
use paintings::figures::{self, Gown, WomanPose};

fn main() {
    let o = paintings::run::Run::new("study_motifs");
    let st = Style::friedrich();
    let pal = &st.palette;
    let mut c = st.prepare(o.width, 1.6, 1);
    let h = c.height();
    // stiff body color for clothes, thinner for hair and hands, lean light
    let body = |hx: &str| Paint { hiding: 0.97, stiff: 1.0, ..pal.paint(hex(hx), 0.0) };
    let thin = |hx: &str| pal.paint(hex(hx), 0.15);
    let rim = pal.paint(hex("#8f8a78"), 0.2);
    let fh = 190.0;

    // top row: the men
    let gy = h * 0.46;
    let sh = figures::man_in_cape(&mut c, (400.0, gy), fh, body("#3a3f33"), body("#15130f"), thin("#2a2119"), Some(rim), 2);
    figures::youth_in_frock(&mut c, (335.0, gy - 8.0), fh * 0.99, body("#394034"), body("#15130f"), thin("#33281c"), body("#d6cfbd"), Some(sh), Some(rim), 3);
    figures::wanderer(&mut c, (640.0, gy), fh, body("#2c3230"), body("#15130f"), thin("#6b5234"), 0.06, Some(rim), 5);
    // the wanderer again, small, as a figure in a landscape (44 units)
    figures::wanderer(&mut c, (860.0, gy), 44.0, body("#2c3230"), body("#15130f"), thin("#6b5234"), 0.06, Some(rim), 4);

    // bottom row: women in the three poses, with and without a shawl
    let gy = h * 0.96;
    let gown = Gown { dress: body("#1d211e"), shawl: Some(body("#4a2a22")), hair: thin("#33251a"), skin: thin("#b89478"), rim: Some(rim) };
    figures::woman(&mut c, (120.0, gy), fh, &gown, WomanPose::Standing, 6);
    figures::woman(&mut c, (380.0, gy), fh, &Gown { dress: body("#2a2d35"), shawl: None, ..gown }, WomanPose::ArmsOpen, 7);
    // at a window: the sill is a dark bar, painted first
    figures::woman(&mut c, (640.0, gy), fh, &Gown { dress: body("#34402f"), shawl: None, ..gown }, WomanPose::AtSill(0.6), 8);
    figures::woman(&mut c, (860.0, gy), 60.0, &gown, WomanPose::ArmsOpen, 9);

    c.relief(0.5, 0.0);
    o.save(&mut c);
}
