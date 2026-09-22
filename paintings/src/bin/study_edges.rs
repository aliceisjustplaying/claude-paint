//! Edges: the same forms painted two ways. Left: body strokes hard-clipped
//! to a stencil mask. Right: body strokes that stop short of the edge, then
//! the edge cut in with a small round brush along the outline, the way a
//! painter does it.

use paint::{Mask, Style, Tool, hex, smoothstep};

fn main() {
    let o = paintings::run::Run::new("study_edges");
    let st = Style::friedrich();
    let mut c = st.prepare(o.width, 2.0, o.seed);
    let (w, h) = (c.width(), c.height());
    let f = c.frame();
    // a pale sky over everything first
    let all = Mask::from_fn(f, |_, _| 1.0);
    c.work(&all, &st.broad().color(|_, y| if y < 250.0 { hex("#c9c4b4") } else { hex("#ddd3bb") }).angle(|_, _| 0.0), 1);
    c.dry();
    for (half, cut) in [(0.0f32, false), (0.5, true)] {
        let x0 = half * w;
        // a hill with a bumpy ridge, a round rock, a church spire
        let ridge = move |x: f32| h * 0.62 - 40.0 * ((x - x0) / w * 12.0).sin().abs() - 25.0 * ((x - x0) / w * 31.0).sin();
        let hill = Mask::from_fn(f, move |x, y| if x >= x0 && x < x0 + w * 0.5 { smoothstep(-0.6, 0.6, y - ridge(x)) } else { 0.0 });
        let (rx, ry, rr) = (x0 + w * 0.15, h * 0.33, 45.0);
        let rock = Mask::from_fn(f, move |x, y| smoothstep(0.6, -0.6, ((x - rx).powi(2) + (y - ry).powi(2)).sqrt() - rr));
        let sx = x0 + w * 0.36;
        let spire = Mask::from_fn(f, move |x, y| {
            let top = h * 0.2;
            let half_w = if y < h * 0.3 { 7.0 * (y - top).max(0.0) / (h * 0.1) } else { 7.0 };
            if y > top && y < h * 0.62 { smoothstep(0.6, -0.6, (x - sx).abs() - half_w) } else { 0.0 }
        });
        for (k, (m, col)) in [(&hill, "#3a3a3e"), (&rock, "#4a4034"), (&spire, "#55565e")].into_iter().enumerate() {
            let mut hd = st.body().color(move |_, _| hex(col)).angle(|_, _| 0.0).threshold(0.5);
            hd = if cut { hd.cut_in(Tool::round_sable(2.2)) } else { hd.clip(true) };
            c.work(m, &hd, 10 + k as u64);
        }
        c.dry();
    }
    c.relief(0.3, 0.02);
    o.save(&mut c);
}
