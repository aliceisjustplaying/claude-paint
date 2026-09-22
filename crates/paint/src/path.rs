//! Path geometry shared by the brushes.

/// Catmull-Rom resample so consecutive points are ~2px apart.
pub(crate) fn densify(p: &[(f32, f32)]) -> Vec<(f32, f32)> {
    let n = p.len();
    let mut out = Vec::new();
    for i in 0..n - 1 {
        let p0 = p[i.saturating_sub(1)];
        let p1 = p[i];
        let p2 = p[i + 1];
        let p3 = p[(i + 2).min(n - 1)];
        let len = ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt();
        let k = ((len / 2.0).ceil() as usize).max(1);
        for j in 0..k {
            let t = j as f32 / k as f32;
            let (t2, t3) = (t * t, t * t * t);
            let cr = |a: f32, b: f32, c: f32, d: f32| {
                0.5 * (2.0 * b + (-a + c) * t + (2.0 * a - 5.0 * b + 4.0 * c - d) * t2
                    + (-a + 3.0 * b - 3.0 * c + d) * t3)
            };
            out.push((cr(p0.0, p1.0, p2.0, p3.0), cr(p0.1, p1.1, p2.1, p3.1)));
        }
    }
    out.push(p[n - 1]);
    out
}
