//! Pastel as a stick (engine 6). Sources: notes/research/pastel_stick_tribology.md,
//! paper_surface.md, dry_pigment_optics.md.
//!
//! - **The stick is a solid**: a round stick, a square one or a pencil's
//!   sharpened core, cut by the facets it has worn. Held at an altitude
//!   (its angle to the paper), an azimuth (where its upper end points) and
//!   a roll (turned about its own axis), its lowest surface over each pixel
//!   is found by casting a ray up through it.
//! - **It rests on the tooth.** Under the hand's force F it sinks into the
//!   paper's micro-relief until the material it meets carries F plastically
//!   (Bowden–Tabor: the soft pastel yields, so the real contact area is
//!   F/σp, σp its hardness). The material ratio a pixel offers at an
//!   overlap o below its top envelope is the paper's bearing curve,
//!   1 − e^(−o/µ) for pores of exponential depth with mean µ (Dodson).
//! - **It wears where it touches** (Archard): the volume abraded is
//!   K·F·s/σp, i.e. at each pixel K × (real contact area) × (distance slid),
//!   linear in load and distance with no threshold (as measured for graphite
//!   on paper). What it abrades is left there, as particles about h_p thick:
//!   the area they cover follows from the volume (Poisson coverage). Grey
//!   level is coverage, as measured.
//! - **The tooth fills.** Pastel in the tooth lies in the pores from the
//!   bottom up; loose pastel is a third body the stick slides on without
//!   abrading, so as the pores fill there is less paper left to file the
//!   stick and a stroke lays less, until the tooth refuses it.
//!   **Fixative** binds what is there: bound particles are a hard, rough
//!   crust the stick abrades again (more weakly than paper), so a fixed
//!   layer takes more pastel; the resin that wets them darkens them a
//!   little (index matching: Kubelka–Munk with less scattering).
//! - **The stick wears to a facet**: the abraded volume comes off where it
//!   touches, flat against the paper, so a stick held one way grows a
//!   facet, and rolled or tilted it cuts a new, sharp-edged one.
//! - **The finger** (`Canvas::rub`) picks up loose pastel and lays it down
//!   along its path, pressing it into the pores (fine grains cover more
//!   per volume); it doesn't move bound pastel.
//!
//! Numbers marked as estimates in the research notes are the defaults here;
//! none of them is a measurement of pastel itself (none was found).

use crate::canvas::Canvas;
use crate::color::Rgb;
use crate::graphite::{Cell, cover, uncover};

/// The stick's cross-section.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Section {
    Round { d_mm: f32 },
    Square { side_mm: f32 },
}

/// A pastel stick (or a pastel pencil's core).
#[derive(Clone, Debug, PartialEq)]
pub struct Stick {
    pub section: Section,
    pub length_mm: f32,
    /// Facets in the stick's own frame (its axis along +z, the drawing end
    /// at z = 0): the stick lies where n·x ≤ d for every facet [nx, ny, nz, d]
    /// (n outward, unit; d in mm).
    pub facets: Vec<[f32; 4]>,
    /// Hardness σp, MPa (soft pastel 1–10, hard 10–50).
    pub hardness: f32,
    /// Archard wear coefficient against paper (friable compacts 0.01–0.1).
    pub wear: f32,
    /// The dry colour of the stick: R∞ of its pigments in air (linear RGB).
    pub color: Rgb,
}

/// Thickness of the particles a stroke leaves (µm): what turns a volume of
/// pastel into the area it covers (particles 1–20 µm, in agglomerates).
pub const PARTICLE_UM: f32 = 6.0;
/// Roughness of a layer of pastel particles, µm (their size).
const CRUST_UM: f32 = 3.0;
/// A bound (fixed) crust abrades the stick this much as the paper does.
const CRUST_WEAR: f32 = 0.5;
/// The facet in contact: the stick's surface within this much (mm) of its
/// lowest point counts as the face the wear comes off.
const FACE_MM: f32 = 0.02;
/// Most facets a stick keeps (older, near-parallel ones merge).
const MAX_FACETS: usize = 48;
/// How long a hand takes to come down to its force and to lift off, s
/// (30–100 ms and 20–80 ms: the 2–5 Hz bandwidth of a hand's force; the
/// force always starts from nothing at landing).
pub const LAND_S: f32 = 0.05;
pub const LIFT_S: f32 = 0.04;
/// Engine 7: the share of what a contact abrades that comes away as crumbs
/// riding under the stick's face (a third body) rather than staying on the
/// asperity it came from. Measured: a stroke covers about 6.6 times its real
/// contact area (Archambault: "debris spreads, fragments and collects in
/// texture"), so about 1/6.6 stays where it was abraded.
pub const SHED: f32 = 1.0 - 1.0 / 6.6;
/// Engine 7: crumbs settle where the face passes within a crumb's size of
/// the surface, µm (soft pastel crumbs on toothy paper: a few µm to 100–300).
pub const CRUMB_UM: f32 = 150.0;
/// Engine 7: the smallest crumb counted, µm across (finer grains go with it).
const CRUMB_MIN_UM: f32 = 5.0;
/// Engine 7: how far a crumb rides under the face before it settles, mm (an
/// estimate; the finger's smear runs 1–10 mm).
const SETTLE_MM: f32 = 3.0;

impl Stick {
    /// Hardness (MPa) and wear coefficient of a stick of softness `soft`
    /// (0 hard pastel .. 1 very soft): σp from 30 down to 2 MPa
    /// (log-spaced), K from 0.01 up to 0.06.
    fn mechanics(soft: f32) -> (f32, f32) {
        let s = soft.clamp(0.0, 1.0);
        let h = 10f32.powf(crate::lerp(30f32.log10(), 2f32.log10(), s));
        (h, crate::lerp(0.01, 0.06, s))
    }

    /// A round soft pastel, Ø `d_mm` (11–13 mm), its end cut flat with the
    /// edge eased (a narrow 45° chamfer), as it comes.
    pub fn round(color: Rgb, soft: f32, d_mm: f32) -> Stick {
        let r = 0.5 * d_mm;
        let mut facets = vec![[0.0, 0.0, -1.0, 0.0]];
        let c = std::f32::consts::FRAC_1_SQRT_2;
        for k in 0..12 {
            let a = k as f32 * std::f32::consts::TAU / 12.0;
            // the plane through the rim 0.4 mm up and 0.4 mm in
            facets.push([c * a.cos(), c * a.sin(), -c, c * (r - 0.4)]);
        }
        let (hardness, wear) = Stick::mechanics(soft);
        Stick { section: Section::Round { d_mm }, length_mm: 65.0, facets, hardness, wear, color }
    }

    /// A square hard pastel, `side_mm` (about 6.35 mm), its end cut square.
    pub fn square(color: Rgb, soft: f32, side_mm: f32) -> Stick {
        let (hardness, wear) = Stick::mechanics(soft);
        Stick { section: Section::Square { side_mm }, length_mm: 75.0, facets: vec![[0.0, 0.0, -1.0, 0.0]], hardness, wear, color }
    }

    /// A pastel pencil's core, Ø `d_mm` (4.3–4.7), sharpened by a blade to a
    /// point: eight cuts at `taper` (the angle between a cut and the axis,
    /// radians), the very point a little blunt.
    pub fn pencil(color: Rgb, soft: f32, d_mm: f32) -> Stick {
        let taper: f32 = 0.22;
        let mut facets = vec![[0.0, 0.0, -1.0, -0.15]];
        let (s, c) = (taper.cos(), taper.sin());
        for k in 0..8 {
            let a = k as f32 * std::f32::consts::TAU / 8.0 + 0.2;
            // a plane through the point, leaning `taper` off the axis
            facets.push([s * a.cos(), s * a.sin(), -c, 0.0]);
        }
        let (hardness, wear) = Stick::mechanics(soft);
        Stick { section: Section::Round { d_mm }, length_mm: 120.0, facets, hardness, wear, color }
    }

    fn radius(&self) -> f32 {
        match self.section {
            Section::Round { d_mm } => 0.5 * d_mm,
            Section::Square { side_mm } => std::f32::consts::FRAC_1_SQRT_2 * side_mm,
        }
    }

    /// The lowest point of the stick on the vertical line through `q0`
    /// (body coordinates of the line's foot) going along `e` (the world's
    /// up in body coordinates): the parameter t (mm up from the foot), or
    /// None if the line misses it.
    fn ray(&self, q0: [f32; 3], e: [f32; 3]) -> Option<f32> {
        let (mut lo, mut hi) = (f32::NEG_INFINITY, f32::INFINITY);
        let mut half = |a: f32, b: f32| {
            // a·t ≤ b
            if a.abs() < 1e-9 {
                if b < 0.0 {
                    lo = f32::INFINITY;
                }
            } else if a > 0.0 {
                hi = hi.min(b / a);
            } else {
                lo = lo.max(b / a);
            }
        };
        for f in &self.facets {
            let ne = f[0] * e[0] + f[1] * e[1] + f[2] * e[2];
            let nq = f[0] * q0[0] + f[1] * q0[1] + f[2] * q0[2];
            half(ne, f[3] - nq);
        }
        // the far end
        half(e[2], self.length_mm - q0[2]);
        match self.section {
            Section::Square { side_mm } => {
                let s = 0.5 * side_mm;
                half(e[0], s - q0[0]);
                half(-e[0], s + q0[0]);
                half(e[1], s - q0[1]);
                half(-e[1], s + q0[1]);
            }
            Section::Round { d_mm } => {
                let r = 0.5 * d_mm;
                let a = e[0] * e[0] + e[1] * e[1];
                let b = 2.0 * (q0[0] * e[0] + q0[1] * e[1]);
                let c = q0[0] * q0[0] + q0[1] * q0[1] - r * r;
                if a < 1e-9 {
                    if c > 0.0 {
                        return None;
                    }
                } else {
                    let disc = b * b - 4.0 * a * c;
                    if disc < 0.0 {
                        return None;
                    }
                    let sq = disc.sqrt();
                    lo = lo.max((-b - sq) / (2.0 * a));
                    hi = hi.min((-b + sq) / (2.0 * a));
                }
            }
        }
        (lo <= hi && lo.is_finite()).then_some(lo)
    }

    /// Take `depth` mm off the stick flat against the paper, whose downward
    /// normal in the stick's frame is `n` and whose lowest point is at
    /// support `d` (n·x ≤ d is the stick now).
    fn wear_face(&mut self, n: [f32; 3], d: f32, depth: f32) {
        let nd = d - depth;
        // a facet already lying this way (within ~4°) is worn further
        if let Some(f) = self.facets.iter_mut().find(|f| f[0] * n[0] + f[1] * n[1] + f[2] * n[2] > 0.9976) {
            f[3] = f[3].min(nd);
            return;
        }
        self.facets.push([n[0], n[1], n[2], nd]);
        if self.facets.len() > MAX_FACETS {
            // the oldest worn facet (not the end's first cut) gives way
            self.facets.remove(1);
        }
    }
}

/// How a stick is held at a point of a stroke.
#[derive(Clone, Copy, Debug)]
pub struct Pose {
    /// Normal force, N.
    pub force: f32,
    /// Altitude: the stick's angle to the paper, radians (π/2 upright).
    pub alt: f32,
    /// Azimuth: where its upper end points, radians in canvas directions
    /// (0 to the right, π/2 down the canvas).
    pub az: f32,
    /// Roll about its own axis, radians.
    pub roll: f32,
}

/// The stick's frame in the world (x right, y down the canvas, z up off
/// the paper): its axes x, y and z (the axis, toward its upper end).
fn frame(p: &Pose) -> [[f32; 3]; 3] {
    let (ca, sa) = (p.alt.cos(), p.alt.sin());
    let (cz, sz) = (p.az.cos(), p.az.sin());
    let a = [ca * cz, ca * sz, sa];
    let r0 = [-sa * cz, -sa * sz, ca];
    let r1 = cross(a, r0);
    let (cr, sr) = (p.roll.cos(), p.roll.sin());
    let bx = [cr * r0[0] + sr * r1[0], cr * r0[1] + sr * r1[1], cr * r0[2] + sr * r1[2]];
    let by = cross(a, bx);
    [bx, by, a]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// A point of a stroke: where (canvas units), the pose, and the speed (mm/s).
#[derive(Clone, Copy, Debug)]
pub struct StrokePoint {
    pub x: f32,
    pub y: f32,
    pub pose: Pose,
    pub speed: f32,
}

/// A stick come down on the surface (`Canvas::seat_stick`).
pub(crate) struct Seated {
    pub under: Vec<(usize, f32)>,
    pub surf: Vec<(f32, f32, f32)>,
    pub delta: f32,
    pub nb: [f32; 3],
}

/// What a stroke did.
#[derive(Clone, Copy, Debug, Default)]
pub struct Laid {
    /// Pastel laid, mm³.
    pub volume_mm3: f32,
    /// Seconds the hand took.
    pub secs: f32,
    /// Share of the abraded volume that fell on wet paint (it smears and is
    /// lost, not laid).
    pub on_wet: f32,
}

/// Engine 7: settle `vol` (µm over a pixel, summed) of crumbs of `color` into
/// a bed of pixels under the face. Each crumb is one agglomerate, its area
/// drawn from P(A) ∝ A^(-3/2) up to `dmax` µm across; it falls into one pixel
/// of the bed, the deeper hollows taking more (they collect in the
/// texture). The face going over presses it flat, so it covers what its
/// volume (a sphere's) makes as particles PARTICLE_UM thick, within that
/// pixel; it is loose pastel in the pores. Crumbs that fall on wet paint are lost in it;
/// returns how much.
#[allow(clippy::too_many_arguments)]
pub(crate) fn lay_crumbs(d: &mut crate::graphite::Drawing, film: &[f32], pxs: &mut [Rgb], bed: &[(usize, f32, bool)], vol: f32, color: Rgb, rng: &mut crate::rng::Rng, dmax: f32, px_um2: f32) -> f32 {
    let mut cum = Vec::with_capacity(bed.len());
    let mut total = 0.0f32;
    for &(_, g, _) in bed {
        total += g;
        cum.push(total);
    }
    if vol <= 0.0 || total <= 0.0 {
        return 0.0;
    }
    let (amin, amax) = (CRUMB_MIN_UM * CRUMB_MIN_UM, dmax * dmax);
    let r = (amin / amax).sqrt();
    let mut left = vol * px_um2; // µm³
    let mut lost = 0.0;
    while left > 0.0 {
        // inverse CDF of A^(-3/2) on [amin, amax]
        let u = rng.f();
        let a = amin / (1.0 - u * (1.0 - r)).powi(2);
        let dia = a.sqrt();
        let v = (std::f32::consts::FRAC_PI_6 * dia * dia * dia).min(left);
        left -= v;
        // where it falls: a pixel of the bed by its depth
        let t = rng.f() * total;
        let k = cum.partition_point(|&c| c < t).min(bed.len() - 1);
        let (i, _, wet) = bed[k];
        let dv = v / px_um2;
        if wet {
            lost += dv;
            continue;
        }
        let c = &mut d.cells[i];
        if c.film < 0.0 || film[i] > c.film + 1e-4 {
            *c = Cell { film: film[i], ..Cell::default() };
        }
        // pressed flat into the hollow by the face going over it: particles
        // about PARTICLE_UM thick, as the contact's own deposit
        let q = 1.0 - (-dv / PARTICLE_UM).exp();
        let under_px = uncover(pxs[i], c.a, c.r);
        let old = c.a * (1.0 - q);
        let w = old + q;
        c.r = [0, 1, 2].map(|k| ((old * c.r[k] + q * color[k]) / w.max(1e-6)).clamp(0.0, 1.0));
        c.lift = (old * c.lift + q * 0.55) / w.max(1e-6);
        c.a = (c.a + (1.0 - c.a) * q).min(0.995);
        c.loose += dv;
        pxs[i] = cover(under_px, c.a, c.r);
    }
    lost
}

/// The material ratio a surface of exponential pore depths (mean `mu`)
/// offers at an overlap `o` below its top envelope.
#[inline]
fn bearing(o: f32, mu: f32) -> f32 {
    if o <= 0.0 { 0.0 } else { 1.0 - (-o / mu.max(1e-3)).exp() }
}

/// The overlap left in the micro-relief at a pixel once the surface has
/// given (`seat`'s second value): above 0 the stick touches there.
pub(crate) fn seat_overlap(o: f32, mu: f32, k: f32, hard: f32) -> f32 {
    seat(o, mu, k, hard).1
}

/// A pixel where the stick overlaps the surface by `o` µm: the surface
/// gives (a Winkler spring, `k` MPa/µm) by u while the stick yields on the
/// micro-asperities it meets (pressure `hard` × the material ratio at the
/// overlap left, o − u), until the two carry the same pressure. Returns
/// (pressure MPa, overlap left µm).
#[inline]
fn seat(o: f32, mu: f32, k: f32, hard: f32) -> (f32, f32) {
    if o <= 0.0 {
        return (0.0, 0.0);
    }
    // g(u) = k u − hard·bearing(o − u): rising in u; root in [0, min(o, hard/k)]
    let (mut lo, mut hi) = (0.0f32, o.min(hard / k));
    for _ in 0..18 {
        let u = 0.5 * (lo + hi);
        if k * u < hard * bearing(o - u, mu) {
            lo = u;
        } else {
            hi = u;
        }
    }
    let u = 0.5 * (lo + hi);
    (k * u, o - u)
}

impl Canvas {
    /// Draw one stroke with `stick` along `pts` (resampled every pixel), the
    /// stick sinking into the tooth under each point's force, laying what
    /// it abrades and wearing. Returns what it laid and the hand time.
    pub fn stick_stroke(&mut self, stick: &mut Stick, pts: &[StrokePoint]) -> Laid {
        let mut out = Laid::default();
        if pts.len() < 2 {
            return out;
        }
        let mmu = self.mm_per_unit;
        let px = self.px_mm();
        let a_px_mm2 = px * px;
        // resample: one step per pixel of travel
        let mut s = vec![0.0f32];
        for w in pts.windows(2) {
            let d = ((w[1].x - w[0].x).powi(2) + (w[1].y - w[0].y).powi(2)).sqrt() * mmu;
            s.push(s.last().unwrap() + d);
        }
        let total = *s.last().unwrap();
        if total <= 0.0 {
            return out;
        }
        let steps = ((total / px).ceil() as usize).max(1);
        let ds = total / steps as f32;
        let at = |t: f32| -> StrokePoint {
            let k = match s.iter().position(|&v| v >= t) {
                Some(0) => 1,
                Some(k) => k,
                None => s.len() - 1,
            };
            let u = ((t - s[k - 1]) / (s[k] - s[k - 1]).max(1e-9)).clamp(0.0, 1.0);
            let (a, b) = (pts[k - 1], pts[k]);
            let l = |p: f32, q: f32| p + (q - p) * u;
            // angles interpolate the short way round
            let la = |p: f32, q: f32| {
                let mut d = q - p;
                while d > std::f32::consts::PI {
                    d -= std::f32::consts::TAU;
                }
                while d < -std::f32::consts::PI {
                    d += std::f32::consts::TAU;
                }
                p + d * u
            };
            StrokePoint {
                x: l(a.x, b.x),
                y: l(a.y, b.y),
                pose: Pose { force: l(a.pose.force, b.pose.force), alt: l(a.pose.alt, b.pose.alt), az: la(a.pose.az, b.pose.az), roll: la(a.pose.roll, b.pose.roll) },
                speed: l(a.speed, b.speed).max(1.0),
            }
        };
        let crumbs = self.engine >= 7;
        // the crumbs riding under the face: volume (µm over a pixel) and where
        // the face last was over the paper (pixels and their gaps, µm)
        let mut carried = 0.0f32;
        let mut last_bed: Vec<(usize, f32, bool)> = Vec::new();
        let mut last_sheet_bed: Vec<(usize, f32)> = Vec::new();
        let settle = 1.0 - (-ds / SETTLE_MM).exp();
        // the crumbs' sizes: a power law in area, P(A) ∝ A^(-3/2) (measured
        // over four decades), its upper cut-off larger for a softer stick
        // and a heavier hand (an estimate within the measured 100–300 µm)
        let soft = ((30.0 / stick.hardness).ln() / 15f32.ln()).clamp(0.0, 1.0);
        let fmean = pts.iter().map(|p| p.pose.force).sum::<f32>() / pts.len() as f32;
        let crumb_max = (80.0 + 220.0 * soft) * (fmean / 2.0).clamp(0.25, 2.0).powf(0.25);
        let mut rng = crate::rng::Rng::new((pts[0].x.to_bits() as u64) << 32 ^ pts[0].y.to_bits() as u64 ^ (steps as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15));
        let px_um2 = (px * 1000.0) * (px * 1000.0);
        for k in 0..=steps {
            let t = k as f32 * ds;
            let mut sp = at(t);
            out.secs += if k == 0 { 0.0 } else { ds / sp.speed };
            // the hand lands and lifts: its force rises from nothing and
            // falls to nothing over the time it takes, at this speed
            sp.pose.force *= crate::smoothstep(0.0, (LAND_S * sp.speed).max(1e-3), t) * crate::smoothstep(0.0, (LIFT_S * sp.speed).max(1e-3), total - t);
            if sp.pose.force <= 1e-4 {
                continue;
            }
            let Some(Seated { under, surf, delta, nb }) = self.seat_stick(stick, &sp) else { continue };
            // what it abrades where it touches, and lays there
            let ds_um = ds * 1000.0;
            self.drawing_mut();
            let film = std::mem::take(&mut self.film);
            let mut pxs = std::mem::take(&mut self.px);
            let wetv: Vec<bool> = under.iter().map(|&(i, _)| self.wet.vol[i] > 1e-5).collect();
            // engine 7: where a sheet lies, what lands is the sheet's
            let onsheet: Vec<bool> = under.iter().map(|&(i, _)| self.sheet_over(i)).collect();
            let mut caught: Vec<(usize, f32)> = Vec::new();
            let mut worn_um3 = 0.0f32;
            let mut wet_um3 = 0.0f32;
            {
                let d = self.drawing_mut();
                d.sticks = true;
                for (j, &(i, b)) in under.iter().enumerate() {
                    let (hs, mu, k) = surf[j];
                    // the overlap left in the micro-relief once the surface has given
                    let o = seat(hs - b * 1000.0 + delta, mu, k, stick.hardness).1;
                    if o <= 0.0 {
                        continue;
                    }
                    if onsheet[j] {
                        // on the sheet: it files the stick as fresh paper does, and keeps what it gets
                        let mut dv = ds_um * stick.wear * bearing(o, mu);
                        if dv <= 0.0 {
                            continue;
                        }
                        worn_um3 += dv;
                        if crumbs {
                            carried += dv * SHED;
                            dv *= 1.0 - SHED;
                        }
                        caught.push((i, dv));
                        continue;
                    }
                    let c = &mut d.cells[i];
                    if c.film < 0.0 || film[i] > c.film + 1e-4 {
                        // bare, or painted over since: a new layer on top
                        *c = Cell { film: film[i], ..Cell::default() };
                    }
                    let v = c.loose + c.bound;
                    // the pastel in the pores fills them from the bottom up,
                    // to this depth below the top envelope
                    let fill_at = if v <= 0.0 { f32::INFINITY } else { mu * (mu / v).max(1.0).ln() };
                    let a_paper = bearing(o.min(fill_at), mu);
                    let a_fill = if o > fill_at { (-fill_at / mu).exp() * bearing(o - fill_at, CRUST_UM) } else { 0.0 };
                    let bound_share = if v > 0.0 { c.bound / v } else { 0.0 };
                    let mut dv = ds_um * stick.wear * (a_paper + CRUST_WEAR * bound_share * a_fill);
                    if dv <= 0.0 {
                        continue;
                    }
                    worn_um3 += dv;
                    if crumbs {
                        // most of it comes away as crumbs under the face
                        carried += dv * SHED;
                        dv *= 1.0 - SHED;
                    }
                    if wetv[j] {
                        wet_um3 += dv;
                        continue;
                    }
                    let q = 1.0 - (-dv / PARTICLE_UM).exp();
                    let under_px = uncover(pxs[i], c.a, c.r);
                    let old = c.a * (1.0 - q);
                    let a1 = (c.a + (1.0 - c.a) * q).min(0.995);
                    let w = old + q;
                    c.r = [0, 1, 2].map(|k| ((old * c.r[k] + q * stick.color[k]) / w.max(1e-6)).clamp(0.0, 1.0));
                    c.lift = (old * c.lift + q * 0.55) / w.max(1e-6);
                    c.a = a1;
                    c.loose += dv;
                    pxs[i] = cover(under_px, c.a, c.r);
                }
                if crumbs {
                    // the bed the crumbs can settle in: the paper under the
                    // face, not in contact, within a crumb's size of it
                    last_bed.clear();
                    last_sheet_bed.clear();
                    for (j, &(i, b)) in under.iter().enumerate() {
                        let gap = b * 1000.0 - delta - surf[j].0;
                        if gap > 0.0 && gap < CRUMB_UM {
                            if onsheet[j] {
                                last_sheet_bed.push((i, gap));
                            } else {
                                last_bed.push((i, gap, wetv[j]));
                            }
                        }
                    }
                    let give = carried * settle;
                    let (gp, gs) = (last_bed.iter().map(|b| b.1).sum::<f32>(), last_sheet_bed.iter().map(|b| b.1).sum::<f32>());
                    let to_sheet = if gp + gs > 0.0 { give * gs / (gp + gs) } else { 0.0 };
                    wet_um3 += lay_crumbs(d, &film, &mut pxs, &last_bed, give - to_sheet, stick.color, &mut rng, crumb_max, px_um2);
                    for &(i, g) in &last_sheet_bed {
                        caught.push((i, to_sheet * g / gs));
                    }
                    carried -= give;
                }
            }
            self.px = pxs;
            self.film = film;
            if !caught.is_empty() {
                self.sheet_catch(&caught, stick.color);
            }
            // (µm of volume per area over mm² of pixel: 10⁻³ mm³)
            let worn_mm3 = worn_um3 * a_px_mm2 * 1e-3;
            out.volume_mm3 += worn_mm3 - wet_um3 * a_px_mm2 * 1e-3;
            out.on_wet += wet_um3 * a_px_mm2 * 1e-3;
            // the stick wears flat against the paper where it touched
            let bmin = under.iter().map(|&(_, b)| b).fold(f32::MAX, f32::min);
            let face = under.iter().filter(|&&(_, b)| b - bmin < FACE_MM).count().max(1) as f32 * a_px_mm2;
            stick.wear_face(nb, -bmin, worn_mm3 / face);
        }
        if crumbs && carried > 0.0 && !last_sheet_bed.is_empty() {
            // (the share that falls on the sheet, by the beds' depths)
            let (gp, gs) = (last_bed.iter().map(|b| b.1).sum::<f32>(), last_sheet_bed.iter().map(|b| b.1).sum::<f32>());
            let to_sheet = carried * gs / (gp + gs).max(1e-9);
            let caught: Vec<(usize, f32)> = last_sheet_bed.iter().map(|&(i, g)| (i, to_sheet * g / gs.max(1e-9))).collect();
            self.sheet_catch(&caught, stick.color);
            carried -= to_sheet;
        }
        if crumbs && carried > 0.0 && !last_bed.is_empty() {
            // lifted off: what the face still carried drops where it was
            let a_px_mm2 = px * px;
            let film = std::mem::take(&mut self.film);
            let mut pxs = std::mem::take(&mut self.px);
            let lost = lay_crumbs(self.drawing_mut(), &film, &mut pxs, &last_bed, carried, stick.color, &mut rng, crumb_max, px_um2);
            self.px = pxs;
            self.film = film;
            out.volume_mm3 -= lost * a_px_mm2 * 1e-3;
            out.on_wet += lost * a_px_mm2 * 1e-3;
        }
        if out.volume_mm3 + out.on_wet > 0.0 {
            out.on_wet /= out.volume_mm3 + out.on_wet;
        }
        out
    }

    /// Where `stick`, held at `sp` (its pose at full force), comes down on
    /// the surface: the pixels under its low part with the stick's
    /// underside over each (mm above its tip), the surface there (height µm,
    /// pore depth µm, give MPa/µm), how far it sank (µm) to carry the force,
    /// and its downward normal in its own frame. `None` off the canvas.
    pub(crate) fn seat_stick(&self, stick: &Stick, sp: &StrokePoint) -> Option<Seated> {
        let f = self.f;
        let mmu = self.mm_per_unit;
        let px = self.px_mm();
        let a_px_mm2 = px * px;
        let rmax = stick.radius();
        let fr = frame(&sp.pose);
        // the stick's tip in the world, mm
        let (ox, oy) = (sp.x * mmu, sp.y * mmu);
        // where its low part can be: the end and as far up the stick as
        // lies within a few mm of the paper (the whole length, laid flat)
        let reach_up = if sp.pose.alt.sin() < 1e-3 { stick.length_mm } else { (3.0 / sp.pose.alt.sin()).min(stick.length_mm) };
        let mut bb = [f32::MAX, f32::MAX, f32::MIN, f32::MIN];
        for zb in [0.0, reach_up] {
            for (cx, cy) in [(-1.0f32, -1.0f32), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
                let (xb, yb) = (cx * rmax, cy * rmax);
                let wx = ox + xb * fr[0][0] + yb * fr[1][0] + zb * fr[2][0];
                let wy = oy + xb * fr[0][1] + yb * fr[1][1] + zb * fr[2][1];
                bb = [bb[0].min(wx), bb[1].min(wy), bb[2].max(wx), bb[3].max(wy)];
            }
        }
        let to_px = |v: f32, o: usize, n: usize| ((v / px).floor() as isize - o as isize).clamp(0, n as isize) as usize;
        let (px0, px1) = (to_px(bb[0], f.x0, f.w), (to_px(bb[2], f.x0, f.w) + 1).min(f.w));
        let (py0, py1) = (to_px(bb[1], f.y0, f.h), (to_px(bb[3], f.y0, f.h) + 1).min(f.h));
        if px0 >= px1 || py0 >= py1 {
            return None;
        }
        // the world's up, and the stick's downward normal, in its frame
        let e = [fr[0][2], fr[1][2], fr[2][2]];
        let nb = [-e[0], -e[1], -e[2]];
        // the stick's underside over each pixel (mm above the tip's level)
        let mut under: Vec<(usize, f32)> = Vec::with_capacity((px1 - px0) * (py1 - py0));
        for py in py0..py1 {
            let wy = (py + f.y0) as f32 * px + 0.5 * px - oy;
            for pxx in px0..px1 {
                let wx = (pxx + f.x0) as f32 * px + 0.5 * px - ox;
                let q0 = [wx * fr[0][0] + wy * fr[0][1], wx * fr[1][0] + wy * fr[1][1], wx * fr[2][0] + wy * fr[2][1]];
                if let Some(b) = stick.ray(q0, e) {
                    under.push((py * f.w + pxx, b));
                }
            }
        }
        if under.is_empty() {
            return None;
        }
        // the surface it comes down on: µm, the pores' mean depth, and how
        // stiffly it gives
        let surf: Vec<(f32, f32, f32)> = under.iter().map(|&(i, _)| self.surface_point(i)).collect();
        // sink until the surface, giving, and the stick, yielding on what
        // it meets, carry the force between them
        let load = |delta: f32| -> f32 {
            let mut p = 0.0;
            for (j, &(_, b)) in under.iter().enumerate() {
                let (hs, mu, k) = surf[j];
                p += seat(hs - b * 1000.0 + delta, mu, k, stick.hardness).0;
            }
            p * a_px_mm2
        };
        let first = under.iter().zip(&surf).map(|(&(_, b), &(hs, _, _))| b * 1000.0 - hs).fold(f32::MAX, f32::min);
        let (mut lo, mut hi) = (first, first + 50.0);
        while load(hi) < sp.pose.force && hi - first < 5000.0 {
            hi = first + 2.0 * (hi - first);
        }
        for _ in 0..40 {
            let m = 0.5 * (lo + hi);
            if load(m) < sp.pose.force {
                lo = m;
            } else {
                hi = m;
            }
        }
        let delta = 0.5 * (lo + hi);
        Some(Seated { under, surf, delta, nb })
    }

    /// Fixative over `m` (coverage 0..1, or all), engine 6: the loose
    /// pastel bound (a crust the next stroke files again), and darkened as
    /// the resin wets the particles: their scattering falls by the share
    /// wetted (`wet` 0..1, 0.15 for a light spray) times what index matching
    /// takes (to about a fifth, for particles of n ≈ 2 in resin of n ≈ 1.5).
    pub fn fix_pastel(&mut self, m: Option<&crate::Mask>, wet: f32) {
        let f = self.f;
        let w = wet.clamp(0.0, 1.0);
        let Some(d) = self.drawing.as_mut() else { return };
        let sheet = self.sheet.as_ref();
        for (i, c) in d.cells.iter_mut().enumerate() {
            if m.is_some_and(|m| m.data[f.whole_index(i)] <= 0.5) || self.film[i] > c.film + 1e-4 || sheet.is_some_and(|s| s.over[i]) {
                continue;
            }
            c.floor = c.a;
            if c.loose > 0.0 {
                c.bound += c.loose;
                c.loose = 0.0;
                let under = uncover(self.px[i], c.a, c.r);
                let s_k = 1.0 - w + w / 5.0;
                c.r = c.r.map(|r| {
                    let r = r.clamp(1e-4, 0.9999);
                    let ks = (1.0 - r) * (1.0 - r) / (2.0 * r) / s_k;
                    1.0 + ks - (ks * ks + 2.0 * ks).sqrt()
                });
                self.px[i] = cover(under, c.a, c.r);
            }
        }
    }

    /// A finger (or a stump) drawn along `path` (units) pressing `force` N:
    /// its pad (a contact of about 135 mm² at 1 N, growing as F^0.4; a stump
    /// of a few mm²: `pad_mm2`) picks up a share of the loose pastel it
    /// passes over and lays a share of what it carries down again, as fine
    /// grains pressed into the pores (they cover more for their volume).
    /// Bound pastel stays. Returns the hand time (s).
    pub fn rub(&mut self, path: &[(f32, f32)], force: f32, pad_mm2: Option<f32>, speed: f32) -> f32 {
        if path.len() < 2 {
            return 0.0;
        }
        let f = self.f;
        let mmu = self.mm_per_unit;
        let px = self.px_mm();
        let area = pad_mm2.unwrap_or(135.0 * force.max(0.05).powf(0.4));
        let r_mm = (area / std::f32::consts::PI).sqrt();
        // what the pad carries: volume (µm·mm²) and its mean colour
        let (mut load, mut col) = (0.0f32, [0.0f32; 3]);
        // per pass under the pad: a share picked up, a share of the load laid
        // (skin picks up 10–30 % a pass: estimates)
        let (pick, lay) = (0.2f32, 0.25f32);
        let mut secs = 0.0;
        let mut last = (path[0].0 * mmu, path[0].1 * mmu);
        self.drawing_mut();
        let film = std::mem::take(&mut self.film);
        let mut pxs = std::mem::take(&mut self.px);
        {
            let wet: Vec<bool> = self.wet.vol.iter().map(|&v| v > 1e-5).collect();
            // (the sheet, where it lies, keeps the finger off the picture)
            let shv: Option<Vec<bool>> = self.sheet.as_ref().map(|s| s.over.clone());
            let d = self.drawing_mut();
            for w in path.windows(2) {
                let (a, b) = ((w[0].0 * mmu, w[0].1 * mmu), (w[1].0 * mmu, w[1].1 * mmu));
                let len = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
                let n = ((len / (r_mm * 0.5)).ceil() as usize).max(1);
                for k in 1..=n {
                    let t = k as f32 / n as f32;
                    let c = (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
                    secs += ((c.0 - last.0).powi(2) + (c.1 - last.1).powi(2)).sqrt() / speed.max(1.0);
                    last = c;
                    let (x0, x1) = (((c.0 - r_mm) / px).floor().max(f.x0 as f32) as usize, (((c.0 + r_mm) / px).ceil() as usize).min(f.x0 + f.w));
                    let (y0, y1) = (((c.1 - r_mm) / px).floor().max(f.y0 as f32) as usize, (((c.1 + r_mm) / px).ceil() as usize).min(f.y0 + f.h));
                    let mut under = Vec::new();
                    for y in y0..y1 {
                        for x in x0..x1 {
                            let (wx, wy) = ((x as f32 + 0.5) * px, (y as f32 + 0.5) * px);
                            if (wx - c.0).powi(2) + (wy - c.1).powi(2) <= r_mm * r_mm {
                                if !shv.as_ref().is_some_and(|s| s[(y - f.y0) * f.w + (x - f.x0)]) {
                                    under.push((y - f.y0) * f.w + (x - f.x0));
                                }
                            }
                        }
                    }
                    if under.is_empty() {
                        continue;
                    }
                    let a_px = px * px;
                    // pick up from the loose pastel under the pad: a step of half
                    // its radius is a quarter of a pass over a point
                    let share = 0.25;
                    for &i in &under {
                        let cell = &mut d.cells[i];
                        if cell.loose <= 0.0 || wet[i] || film[i] > cell.film + 1e-4 {
                            continue;
                        }
                        let take = cell.loose * pick * share;
                        let total = load + take * a_px;
                        for q in 0..3 {
                            col[q] = (col[q] * load + cell.r[q] * take * a_px) / total.max(1e-9);
                        }
                        load = total;
                        cell.loose -= take;
                    }
                    // lay a share of the load over the pad, as fine grains
                    let give = load * lay * share;
                    if give <= 0.0 {
                        continue;
                    }
                    let per = give / (under.len() as f32 * a_px);
                    for &i in &under {
                        let cell = &mut d.cells[i];
                        if wet[i] {
                            continue;
                        }
                        if cell.film < 0.0 || film[i] > cell.film + 1e-4 {
                            *cell = Cell { film: film[i], ..Cell::default() };
                        }
                        // fine grains: a third of a stroke's particles' thickness
                        let q = 1.0 - (-per / (PARTICLE_UM / 3.0)).exp();
                        let under_px = uncover(pxs[i], cell.a, cell.r);
                        let old = cell.a * (1.0 - q);
                        let wsum = old + q;
                        cell.r = [0, 1, 2].map(|k| ((old * cell.r[k] + q * col[k]) / wsum.max(1e-6)).clamp(0.0, 1.0));
                        cell.a = (cell.a + (1.0 - cell.a) * q).min(0.995);
                        cell.loose += per;
                        pxs[i] = cover(under_px, cell.a, cell.r);
                    }
                    load -= give;
                }
            }
            d.sticks = true;
        }
        self.px = pxs;
        self.film = film;
        secs
    }

    /// What a fingertip feels at canvas point (x, y): the surface (paper,
    /// paint and how far it has set), and the pastel in the tooth (none,
    /// some, the tooth half full, full; loose or fixed).
    pub fn feel(&self, x: f32, y: f32) -> String {
        let f = self.f;
        let (gx, gy) = ((x * f.scale).floor() as isize - f.x0 as isize, (y * f.scale).floor() as isize - f.y0 as isize);
        if gx < 0 || gy < 0 || gx >= f.w as isize || gy >= f.h as isize {
            return "off the canvas".into();
        }
        let i = gy as usize * f.w + gx as usize;
        let paint = self.film[i] > 1e-3 || self.wet.vol[i] > 1e-5;
        let mut out = if self.wet.vol[i] > 1e-5 {
            let stage = match self.drying_at(x, y) {
                crate::Stage::Open => "open",
                crate::Stage::Setting => "setting",
                crate::Stage::Tacky => "tacky",
                crate::Stage::Dry => "dry",
            };
            format!("wet paint, {stage}")
        } else if paint {
            if self.gloss[i] > 0.5 { "dry paint, smooth".to_string() } else { "dry paint, lean and matte".to_string() }
        } else if self.paper.is_some() {
            "paper".to_string()
        } else {
            "the ground".to_string()
        };
        if let Some(d) = &self.drawing {
            let c = &d.cells[i];
            let v = c.loose + c.bound;
            if v > 0.0 && self.film[i] <= c.film + 1e-4 {
                let mu = self.micro_um(i);
                let full = v / mu.max(1e-3);
                let tooth = if full >= 0.95 { "the tooth full" } else if full > 0.5 { "the tooth half full and more" } else if full > 0.2 { "the tooth taking it" } else { "a little in the tooth" };
                let fixed = if c.loose <= 0.0 { "fixed" } else if c.bound > 0.0 { "loose over fixed" } else { "loose" };
                out += &format!(": pastel, {fixed}, {tooth}");
            } else {
                out += ": the tooth open";
            }
        } else {
            out += ": the tooth open";
        }
        out
    }
}

/// A pigment's refractive index (mean of its principal indices), by tube
/// name (notes/research/dry_pigment_optics.md §1); 1.9 if not listed.
pub fn refractive_index(name: &str) -> f32 {
    match name {
        "lead white" => 2.0,
        "zinc white" => 2.02,
        "cobalt blue" => 1.74,
        "ultramarine blue" | "ultramarine ash" => 1.5,
        "cerulean blue" => 1.84,
        "Prussian blue" | "Antwerp blue" => 1.56,
        "emerald green" => 1.75,
        "viridian" => 1.8,
        "chrome yellow" | "lemon chrome" => 2.4,
        "orange chrome" => 2.55,
        "barium yellow" => 1.96,
        "strontium yellow" => 1.97,
        "zinc yellow" => 1.87,
        "pale cadmium" | "cadmium yellow" | "deep cadmium" => 2.5,
        "cadmium red" => 2.6,
        "Naples yellow" => 2.15,
        "Indian yellow" => 1.65,
        "yellow lake" => 1.55,
        "yellow ochre" | "brown ochre" => 2.1,
        "Mars yellow" => 2.2,
        "red earth" | "Mars red" | "Indian red" => 2.7,
        "raw sienna" => 2.0,
        "burnt sienna" | "Mars brown" | "Mars orange" => 2.3,
        "raw umber" => 2.1,
        "vermilion" | "orange vermilion" | "Chinese vermilion" => 3.0,
        "red lead" => 2.42,
        "rose madder" | "carmine lake" | "magenta" | "permanent alizarin" => 1.6,
        "cobalt violet" => 1.68,
        "bone black" | "bone brown" => 1.65,
        "vine black" => 1.9,
        "smalt" | "pale smalt" => 1.5,
        _ => 1.9,
    }
}

/// Transport scattering per unit volume of a 1 µm pigment in air, relative
/// (Mie, size-averaged: the research note's table, d = 1 µm), and the ratio
/// of its scattering dry to in fresh oil.
fn mie_air(n: f32) -> (f32, f32) {
    // (n, σ'_air µm⁻¹, S_air/S_oil) at d = 1 µm
    const T: [(f32, f32, f32); 12] = [
        (1.50, 0.68, 700.0),
        (1.56, 0.71, 43.0),
        (1.65, 0.73, 9.0),
        (1.70, 0.80, 5.8),
        (1.74, 0.75, 3.9),
        (1.84, 0.82, 2.5),
        (2.00, 0.88, 1.9),
        (2.15, 0.91, 1.7),
        (2.30, 0.89, 1.6),
        (2.40, 1.04, 1.8),
        (2.45, 0.95, 1.5),
        (3.00, 1.03, 1.4),
    ];
    if n <= T[0].0 {
        return (T[0].1, T[0].2);
    }
    for w in T.windows(2) {
        if n <= w[1].0 {
            let u = (n - w[0].0) / (w[1].0 - w[0].0);
            // the ratio spans decades: interpolate its log
            return (w[0].1 + (w[1].1 - w[0].1) * u, (w[0].2.ln() + (w[1].2.ln() - w[0].2.ln()) * u).exp());
        }
    }
    (T[11].1, T[11].2)
}

/// How pigments look dry, in a pastel stick: two-constant Kubelka–Munk
/// from each tube's masstone in oil (R∞ → K/S), its scattering in oil
/// (`s_oil`, per coat), and its refractive index. Absorption K stays; the
/// scattering rises with the index contrast against air: for n ≥ 1.9 by the
/// Mie ratio over its scattering in oil; below that, where oil all but
/// matches the pigment and its oil value says little, from the dry Mie
/// scattering itself, scaled to lead white's (n = 2: its oil S × 1.9).
/// `parts`: (masstone, s_oil, n, share by volume).
/// Engine 7: the air/oil scattering ratio of a carbon black, whose
/// absorption comes from carbon on (or in) its particles: bone black's
/// apatite matrix ×9 at 1 µm (dry_pigment_optics.md, table 2.2), a carbon
/// (vine) black's ×2.4 (§2.4: "a dry carbon black is a little greyer, never
/// pale"). None for any other tube.
pub fn carbon_ratio(name: &str) -> Option<f32> {
    match name {
        "bone black" | "bone brown" | "ivory black" => Some(9.0),
        "vine black" | "lamp black" => Some(2.4),
        _ => None,
    }
}

/// Each part is (masstone in oil, scattering in oil, refractive index,
/// share). A negative index is a carbon black's air/oil ratio
/// (`carbon_ratio`), taken as it is.
pub fn dry_color(parts: &[(Rgb, f32, f32, f32)], s_oil_white: f32) -> Rgb {
    let total: f32 = parts.iter().map(|p| p.3).sum::<f32>().max(1e-9);
    let (sigma_white, r_white) = mie_air(2.0);
    let s_dry_white = r_white * s_oil_white;
    let mut out = [0.0f32; 3];
    for (q, o) in out.iter_mut().enumerate() {
        let (mut k, mut s) = (0.0f32, 0.0f32);
        for &(m, s_oil, n, c) in parts {
            let r = m[q].clamp(1e-4, 0.9999);
            let ks = (1.0 - r) * (1.0 - r) / (2.0 * r);
            let s_dry = if n < 0.0 {
                -n * s_oil
            } else {
                let (sigma, ratio) = mie_air(n);
                if n >= 1.9 { ratio * s_oil } else { s_dry_white * sigma / sigma_white }
            };
            k += c / total * ks * s_oil;
            s += c / total * s_dry;
        }
        let ks = k / s.max(1e-9);
        *o = 1.0 + ks - (ks * ks + 2.0 * ks).sqrt();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paper::Paper;

    fn sheet() -> Canvas {
        Canvas::new_window(600, 1.0, [0.55; 3], None).with_size_mm(150.0).with_engine(6).with_paper(Paper::drawing(4))
    }

    fn pose(force: f32, alt_deg: f32) -> Pose {
        Pose { force, alt: alt_deg.to_radians(), az: 45f32.to_radians(), roll: 0.0 }
    }

    fn line(c: &mut Canvas, st: &mut Stick, y: f32, force: f32, alt: f32) -> Laid {
        let pts = [StrokePoint { x: 100.0, y, pose: pose(force, alt), speed: 50.0 }, StrokePoint { x: 900.0, y, pose: pose(force, alt), speed: 50.0 }];
        c.stick_stroke(st, &pts)
    }

    fn darkening(c: &Canvas, y0: f32, y1: f32) -> f32 {
        let f = c.window();
        let mut s = 0.0;
        for (i, p) in c.pixels().iter().enumerate() {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            if x > 200.0 && x < 800.0 && y >= y0 && y < y1 {
                s += 0.55 - p[0];
            }
        }
        s
    }

    /// Archard: a stroke lays in proportion to the force, and the stick
    /// shortens by what it lays.
    #[test]
    fn a_stroke_lays_in_proportion_to_its_force() {
        let black = [0.02f32; 3];
        let (mut c1, mut c2) = (sheet(), sheet());
        let (mut s1, mut s2) = (Stick::round(black, 0.7, 12.0), Stick::round(black, 0.7, 12.0));
        // (a first stroke to wear a facet, off the measured band)
        line(&mut c1, &mut s1, 900.0, 1.0, 60.0);
        line(&mut c2, &mut s2, 900.0, 1.0, 60.0);
        let (a, b) = (line(&mut c1, &mut s1, 400.0, 1.0, 60.0), line(&mut c2, &mut s2, 400.0, 3.0, 60.0));
        let r = b.volume_mm3 / a.volume_mm3;
        assert!(r > 2.0 && r < 4.0, "3 N lays {r}× what 1 N does");
        assert!(darkening(&c2, 300.0, 500.0) > darkening(&c1, 300.0, 500.0));
    }

    /// A stick held one way wears a facet parallel to the paper; rolled, it
    /// cuts a new one.
    #[test]
    fn the_stick_wears_to_a_facet() {
        let mut c = sheet();
        let mut st = Stick::round([0.5; 3], 0.8, 12.0);
        let n0 = st.facets.len();
        line(&mut c, &mut st, 300.0, 2.0, 50.0);
        let n1 = st.facets.len();
        assert_eq!(n1, n0 + 1, "a new facet from the first stroke");
        line(&mut c, &mut st, 500.0, 2.0, 50.0);
        assert_eq!(st.facets.len(), n1, "the same facet, worn further");
        let pts = [StrokePoint { x: 100.0, y: 700.0, pose: Pose { roll: 1.2, ..pose(2.0, 50.0) }, speed: 50.0 }, StrokePoint { x: 900.0, y: 700.0, pose: Pose { roll: 1.2, ..pose(2.0, 50.0) }, speed: 50.0 }];
        c.stick_stroke(&mut st, &pts);
        assert_eq!(st.facets.len(), n1 + 1, "rolled: a new facet");
    }

    /// The tooth fills: the same stroke over and over lays less each time;
    /// after fixing, it lays more again.
    #[test]
    fn the_tooth_fills_and_fixative_gives_it_back() {
        let mut c = sheet();
        let mut st = Stick::round([0.1, 0.1, 0.5], 0.8, 12.0);
        let mut v = Vec::new();
        for _ in 0..8 {
            v.push(line(&mut c, &mut st, 500.0, 3.0, 55.0).volume_mm3);
        }
        assert!(v[7] < 0.6 * v[1], "fills: {v:?}");
        c.fix_pastel(None, 0.15);
        let after = line(&mut c, &mut st, 500.0, 3.0, 55.0).volume_mm3;
        assert!(after > 1.3 * v[7], "fixed takes more: {} vs {}", after, v[7]);
    }

    /// Ultramarine is deep in oil and pale and bright dry; vermilion barely
    /// changes.
    #[test]
    fn dry_pigments_are_paler_by_their_index() {
        let ultra = [0.03f32, 0.04, 0.35];
        let verm = [0.62f32, 0.05, 0.03];
        let du = dry_color(&[(ultra, 0.3, 1.5, 1.0)], 1.0);
        let dv = dry_color(&[(verm, 1.0, 3.0, 1.0)], 1.0);
        assert!(du[2] > ultra[2] + 0.2, "ultramarine dry {du:?}");
        assert!((dv[0] - verm[0]).abs() < 0.15, "vermilion dry {dv:?}");
    }

    /// The finger moves loose pastel: a dark band smeared along the path
    /// darkens what lies past it, and bound pastel doesn't move.
    #[test]
    fn the_finger_moves_loose_pastel() {
        let mut c = sheet();
        let mut st = Stick::round([0.02; 3], 0.8, 12.0);
        for k in 0..6 {
            let x = 300.0 + 8.0 * k as f32;
            let pts = [StrokePoint { x, y: 200.0, pose: pose(3.0, 80.0), speed: 50.0 }, StrokePoint { x, y: 800.0, pose: pose(3.0, 80.0), speed: 50.0 }];
            c.stick_stroke(&mut st, &pts);
        }
        let before = darkening(&c, 450.0, 550.0);
        let mut d = c.clone();
        d.fix_pastel(None, 0.15);
        let fixed_before = d.pixels().to_vec();
        c.rub(&[(250.0, 500.0), (700.0, 500.0)], 1.0, None, 40.0);
        d.rub(&[(250.0, 500.0), (700.0, 500.0)], 1.0, None, 40.0);
        let f = c.window();
        let right = |c: &Canvas| (0..f.w).filter(|&x| f.ux(x) > 450.0 && f.ux(x) < 650.0).map(|x| 0.55 - c.pixels()[(500.0 * f.scale) as usize * f.w + x][0]).sum::<f32>();
        assert!(right(&c) > 0.0, "smeared past the band");
        assert!(d.pixels() == &fixed_before[..], "bound pastel stays");
        let _ = before;
    }
}
