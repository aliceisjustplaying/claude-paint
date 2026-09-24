//! Path geometry: polyline lengths shared by everything that walks a line
//! (trees, graphite, rocks, outlines, the easel's stroke planners) and the
//! brushes' resampling.

/// Length of a segment.
#[inline]
pub fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt()
}

/// Length of an open polyline (0 for fewer than two points).
pub fn length(p: &[(f32, f32)]) -> f32 {
    p.windows(2).map(|w| dist(w[0], w[1])).sum()
}

/// Cumulative length at each point of an open polyline (0 at the first;
/// empty for no points). Its last value is `length(p)`, bit for bit.
pub fn arclen(p: &[(f32, f32)]) -> Vec<f32> {
    let mut s = vec![0.0f32; p.len()];
    for i in 1..p.len() {
        s[i] = s[i - 1] + dist(p[i - 1], p[i]);
    }
    s
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_is_the_last_arclen_bit_for_bit() {
        let p: Vec<(f32, f32)> = (0..50).map(|i| { let t = i as f32 * 0.37; (t.cos() * 13.1 + t, t.sin() * 7.3) }).collect();
        let s = arclen(&p);
        assert_eq!(s.len(), p.len());
        assert_eq!(s[0], 0.0);
        assert_eq!(s.last().unwrap().to_bits(), length(&p).to_bits());
        assert!(s.windows(2).all(|w| w[1] >= w[0]));
        assert_eq!(length(&[(1.0, 1.0)]), 0.0);
        assert!(arclen(&[]).is_empty());
        assert_eq!(length(&[(0.0, 0.0), (3.0, 4.0), (3.0, 0.0)]), 9.0);
    }
}
