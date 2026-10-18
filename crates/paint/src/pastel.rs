//! Pastel as a stick. Sources: notes/research/pastel_stick_tribology.md,
//! paper_surface.md, dry_pigment_optics.md, pastel_blending.md (the finger).
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
//!   the area they cover follows from the volume (Poisson coverage). Gray
//!   level is coverage, as measured.
//! - **Most of it comes away as crumbs** (`SHED`): about 1/6.6 stays on the
//!   asperity it came from (a stroke covers about 6.6 times its real
//!   contact); the rest rides under the face and settles into the paper
//!   under it within a crumb's size, along and beside the contact
//!   (`lay_crumbs`). What is still carried at the lift drops there.
//! - **A sheet laid on the picture** (`sheet.rs`) is a surface like any
//!   other, raised by its caliper: what the stick lays and sheds over it
//!   stays on it.
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
//!   per volume); it doesn't move bound pastel. Over an area
//!   (`Canvas::rub_area`) it is driven along lanes that cover a mask, in
//!   loose circles (a fingertip) or straight strokes (a stump), pressing
//!   the force × the mask (sources: pastel_blending.md).
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
    /// The dry color of the stick: R∞ of its pigments in air (linear RGB).
    pub color: Rgb,
    pub material: Material,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub particle_um: f32,
    pub shed: f32,
    pub lift: f32,
    pub binder: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self { particle_um: PARTICLE_UM, shed: SHED, lift: 0.55, binder: 0.0 }
    }
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
/// The share of what a contact abrades that comes away as crumbs riding
/// under the stick's face (a third body) rather than staying on the
/// asperity it came from. Measured: a stroke covers about 6.6 times its real
/// contact area (Archambault: "debris spreads, fragments and collects in
/// texture"), so about 1/6.6 stays where it was abraded.
pub const SHED: f32 = 1.0 - 1.0 / 6.6;
/// Crumbs settle where the face passes within a crumb's size of the
/// surface, µm (soft pastel crumbs on toothy paper: a few µm to 100–300).
pub const CRUMB_UM: f32 = 150.0;
/// The smallest crumb counted, µm across (finer grains go with it).
pub(crate) const CRUMB_MIN_UM: f32 = 5.0;
/// How far a crumb rides under the face before it settles, mm (an
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
        Stick { section: Section::Round { d_mm }, length_mm: 65.0, facets, hardness, wear, color, material: Material::default() }
    }

    /// A square hard pastel, `side_mm` (about 6.35 mm), its end cut square.
    pub fn square(color: Rgb, soft: f32, side_mm: f32) -> Stick {
        let (hardness, wear) = Stick::mechanics(soft);
        Stick { section: Section::Square { side_mm }, length_mm: 75.0, facets: vec![[0.0, 0.0, -1.0, 0.0]], hardness, wear, color, material: Material::default() }
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
        Stick { section: Section::Round { d_mm }, length_mm: 120.0, facets, hardness, wear, color, material: Material::default() }
    }

    pub fn charcoal(kind: &str, grade: &str, thick: bool) -> Option<Stick> {
        let (gray, hardness, wear, shed, particle_um, lift, binder) = match (kind, grade) {
            ("vine", _) => (0.055, 1.2, 0.09, 0.94, 9.0, 0.95, 0.0),
            ("willow", _) => (0.032, 1.5, 0.08, 0.92, 6.0, 0.92, 0.0),
            ("compressed", _) => (0.006, 5.0, 0.045, 0.55, 4.0, 0.32, 0.30),
            ("pencil", "soft") => (0.008, 4.0, 0.05, 0.65, 4.0, 0.40, 0.20),
            ("pencil", "medium") => (0.014, 9.0, 0.032, 0.48, 4.0, 0.30, 0.35),
            ("pencil", "hard") => (0.025, 18.0, 0.018, 0.32, 4.0, 0.22, 0.50),
            _ => return None,
        };
        let mut s = if kind == "pencil" { Stick::pencil([gray; 3], 0.5, 4.0) } else { Stick::round([gray; 3], 1.0, if kind == "compressed" || thick { 8.0 } else { 3.0 }) };
        s.hardness = hardness;
        s.wear = wear;
        s.material = Material { particle_um, shed, lift, binder };
        Some(s)
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

/// Settle `vol` (µm over a pixel, summed) of crumbs of `color` into a bed of
/// pixels under the face. Each crumb is one agglomerate, its area drawn from
/// P(A) ∝ A^(-3/2) up to `dmax` µm across; it falls into one pixel of the
/// bed, the deeper hollows taking more (they collect in the texture). The
/// face going over presses it flat, so it covers what its volume (a
/// sphere's) makes as particles PARTICLE_UM thick, within that pixel; it is
/// loose pastel in the pores. Crumbs that fall on wet paint are lost in it;
/// returns how much.
#[allow(clippy::too_many_arguments)]
pub(crate) fn lay_crumbs(d: &mut crate::graphite::Drawing, film: &[f32], pxs: &mut [Rgb], bed: &[(usize, f32, bool)], vol: f32, color: Rgb, rng: &mut crate::rng::Rng, dmax: f32, px_um2: f32) -> f32 {
    lay_material_crumbs(d, film, pxs, bed, vol, color, rng, dmax, px_um2, Material::default())
}

#[allow(clippy::too_many_arguments)]
fn lay_material_crumbs(d: &mut crate::graphite::Drawing, film: &[f32], pxs: &mut [Rgb], bed: &[(usize, f32, bool)], vol: f32, color: Rgb, rng: &mut crate::rng::Rng, dmax: f32, px_um2: f32, material: Material) -> f32 {
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
        let q = 1.0 - (-dv / material.particle_um).exp();
        let under_px = uncover(pxs[i], c.a, c.r);
        let old = c.a * (1.0 - q);
        let w = old + q;
        c.r = [0, 1, 2].map(|k| ((old * c.r[k] + q * color[k]) / w.max(1e-6)).clamp(0.0, 1.0));
        c.lift = (old * c.lift + q * material.lift) / w.max(1e-6);
        c.a = (c.a + (1.0 - c.a) * q).min(0.995);
        c.loose += dv;
        if material.binder > 0.0 {
            c.loose -= dv * material.binder;
            c.bound += dv * material.binder;
            c.floor = (c.floor + (1.0 - c.floor) * q * material.binder).min(c.a);
        }
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
    pub fn charcoal_powder(&mut self, mask: &crate::Mask, depth_um: f32, seed: u64) -> f32 {
        self.check_mask(mask);
        if !(depth_um > 0.0 && depth_um.is_finite()) {
            return 0.0;
        }
        let f = self.f;
        let mut bed = Vec::new();
        let mut caught = Vec::new();
        let mut volume = 0.0;
        for i in 0..self.px.len() {
            let weight = mask.data[f.whole_index(i)].clamp(0.0, 1.0);
            if weight <= 0.0 {
                continue;
            }
            if self.sheet_over(i) {
                caught.push((i, weight * depth_um));
            } else {
                volume += weight * depth_um;
                bed.push((i, weight, self.wet.vol[i] > 1e-5));
            }
        }
        let color = [0.04; 3];
        self.sheet_catch(&caught, color);
        self.drawing_mut();
        let film = std::mem::take(&mut self.film);
        let mut pxs = std::mem::take(&mut self.px);
        let px_um2 = (self.px_mm() * 1000.0).powi(2);
        let d = self.drawing_mut();
        let lost = lay_material_crumbs(d, &film, &mut pxs, &bed, volume, color, &mut crate::rng::Rng::new(seed), 100.0, px_um2, Material { lift: 0.95, ..Material::default() });
        d.sticks |= volume > lost;
        self.film = film;
        self.px = pxs;
        (volume - lost) * px_um2 * 1e-9
    }

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
                let tau = std::f32::consts::TAU;
                let mut d = (q % tau - p % tau) % tau;
                if d > std::f32::consts::PI {
                    d -= std::f32::consts::TAU;
                }
                if d < -std::f32::consts::PI {
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
        // the crumbs riding under the face: volume (µm over a pixel) and where
        // the face last was over the paper (pixels, their gaps µm, wet) and
        // over a sheet laid on it (pixels, gaps)
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
            // where a sheet lies, what lands is the sheet's
            let onsheet: Vec<bool> = under.iter().map(|&(i, _)| self.sheet_over(i)).collect();
            let mut caught: Vec<(usize, f32)> = Vec::new();
            let mut worn_um3 = 0.0f32;
            let mut wet_um3 = 0.0f32;
            {
                let d = self.drawing_mut();
                // (a stick drawing's layout once pastel lands in the tooth:
                // its cells' loose and bound pastel are kept, `Drawing::sticks`)
                let mut laid = false;
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
                        carried += dv * stick.material.shed;
                        dv *= 1.0 - stick.material.shed;
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
                    // most of it comes away as crumbs under the face
                    carried += dv * stick.material.shed;
                    dv *= 1.0 - stick.material.shed;
                    if wetv[j] {
                        wet_um3 += dv;
                        continue;
                    }
                    let q = 1.0 - (-dv / stick.material.particle_um).exp();
                    let under_px = uncover(pxs[i], c.a, c.r);
                    let old = c.a * (1.0 - q);
                    let a1 = (c.a + (1.0 - c.a) * q).min(0.995);
                    let w = old + q;
                    c.r = [0, 1, 2].map(|k| ((old * c.r[k] + q * stick.color[k]) / w.max(1e-6)).clamp(0.0, 1.0));
                    c.lift = (old * c.lift + q * stick.material.lift) / w.max(1e-6);
                    c.a = a1;
                    c.loose += dv;
                    if stick.material.binder > 0.0 {
                        c.loose -= dv * stick.material.binder;
                        c.bound += dv * stick.material.binder;
                        c.floor = (c.floor + (1.0 - c.floor) * q * stick.material.binder).min(c.a);
                    }
                    laid = true;
                    pxs[i] = cover(under_px, c.a, c.r);
                }
                // the bed the crumbs can settle in: the paper (or the sheet)
                // under the face, not in contact, within a crumb's size of it
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
                let (gp, gs) = (last_bed.iter().map(|b| b.1).sum::<f32>(), last_sheet_bed.iter().map(|b| b.1).sum::<f32>());
                // (no bed under the face: the crumbs ride on to the next step)
                let give = if gp + gs > 0.0 { carried * settle } else { 0.0 };
                // (the share that falls on the sheet, by the beds' depths)
                let to_sheet = if gp + gs > 0.0 { give * gs / (gp + gs) } else { 0.0 };
                // (crumbs that settle on wet paint aren't laid in the tooth)
                laid |= give - to_sheet > 0.0 && last_bed.iter().any(|b| !b.2);
                wet_um3 += lay_material_crumbs(d, &film, &mut pxs, &last_bed, give - to_sheet, stick.color, &mut rng, crumb_max, px_um2, stick.material);
                for &(i, g) in &last_sheet_bed {
                    caught.push((i, to_sheet * g / gs));
                }
                carried -= give;
                d.sticks |= laid;
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
        if carried > 0.0 && !last_sheet_bed.is_empty() {
            // lifted off over the sheet: its share of what the face still
            // carried drops on it, by the beds' depths
            let (gp, gs) = (last_bed.iter().map(|b| b.1).sum::<f32>(), last_sheet_bed.iter().map(|b| b.1).sum::<f32>());
            let to_sheet = carried * gs / (gp + gs).max(1e-9);
            let caught: Vec<(usize, f32)> = last_sheet_bed.iter().map(|&(i, g)| (i, to_sheet * g / gs.max(1e-9))).collect();
            self.sheet_catch(&caught, stick.color);
            carried -= to_sheet;
        }
        if carried > 0.0 && !last_bed.is_empty() {
            // lifted off: what the face still carried drops where it was
            let a_px_mm2 = px * px;
            let film = std::mem::take(&mut self.film);
            let mut pxs = std::mem::take(&mut self.px);
            let lost = lay_material_crumbs(self.drawing_mut(), &film, &mut pxs, &last_bed, carried, stick.color, &mut rng, crumb_max, px_um2, stick.material);
            self.px = pxs;
            self.film = film;
            out.volume_mm3 -= lost * a_px_mm2 * 1e-3;
            out.on_wet += lost * a_px_mm2 * 1e-3;
            carried = 0.0;
        }
        if carried > 0.0 {
            // (no bed at the lift either: those crumbs fall clear of the
            // picture, not laid)
            out.volume_mm3 -= carried * px * px * 1e-3;
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

    /// Fixative over `m` (coverage 0..1, or all): the loose
    /// pastel bound (a crust the next stroke files again), and darkened as
    /// the resin wets the particles: their scattering falls by the share
    /// wetted (`wet` 0..1, 0.15 for a light spray) times what index matching
    /// takes (to about a fifth, for particles of n ≈ 2 in resin of n ≈ 1.5).
    pub fn fix_pastel(&mut self, m: Option<&crate::Mask>, wet: f32) {
        let f = self.f;
        let w = wet.clamp(0.0, 1.0);
        let Some(d) = self.drawing.as_mut() else { return };
        // (a sheet laid on the picture keeps the spray off what it covers)
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
    /// its pad (a fingertip's contact, `finger_mm2`: 135 mm² at 1 N, growing
    /// as F^0.4 up to 2 N and hardly at all past it; a stump of a few mm²:
    /// `pad_mm2`) picks up a share of the loose pastel it passes over (less
    /// as a fingertip's skin loads) and lays a share of what it carries down
    /// again, as fine grains pressed into the pores (they cover more for
    /// their volume). Bound pastel stays. Returns the hand time (s).
    pub fn rub(&mut self, path: &[(f32, f32)], force: f32, pad_mm2: Option<f32>, speed: f32) -> f32 {
        if path.len() < 2 {
            return 0.0;
        }
        let mmu = self.mm_per_unit;
        let area = pad_area(force, pad_mm2);
        let r_mm = (area / std::f32::consts::PI).sqrt();
        let mut secs = 0.0;
        let mut last = (path[0].0 * mmu, path[0].1 * mmu);
        // (nothing drawn: nothing to move, only the hand's time along the path)
        if self.drawing.is_none() {
            let len: f32 = path.windows(2).map(|w| ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt() * mmu).sum();
            return len / speed.max(1.0);
        }
        // (the pad starts clean, and what it still carries at the end goes
        // with the finger)
        let mut pad = Pad::new(area, pad_mm2.is_none());
        self.rubbing(|r| {
            for w in path.windows(2) {
                let (a, b) = ((w[0].0 * mmu, w[0].1 * mmu), (w[1].0 * mmu, w[1].1 * mmu));
                let len = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
                let n = ((len / (r_mm * 0.5)).ceil() as usize).max(1);
                for k in 1..=n {
                    let t = k as f32 / n as f32;
                    let c = (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
                    secs += ((c.0 - last.0).powi(2) + (c.1 - last.1).powi(2)).sqrt() / speed.max(1.0);
                    last = c;
                    r.press(&mut pad, c, r_mm);
                }
            }
        });
        secs
    }

    /// A finger or a stump worked over an area (`m` above a trace): along
    /// lanes across the mask at `angle_deg`, `SMUDGE_LANE` of the width a
    /// lane sweeps apart (the full pad's, plus a circle's), either in loose
    /// circles drifting along each lane, lane after lane back and forth (a
    /// fingertip's default), or in straight strokes there and back along
    /// each lane (a stump's: Orme 2017, Sousa & Buchanan 1999 [P]); each
    /// further pass turned by `SMUDGE_CROSS_DEG`. The pad
    /// presses the force × the mask's value under its center (a soft edge is
    /// a lighter touch: with a fingertip, a smaller pad) and is lifted where
    /// that falls below `SMUDGE_MIN_FORCE` or the mask ends; each piece the
    /// finger is down for is `rub`'s pad, a step of half its radius on. One
    /// finger does it all: what it picks up it carries on. Refused (nothing
    /// done) when the mask is not this canvas's, a number is out of range,
    /// or the work is too large. Defaults and their sources: the `SMUDGE_*`,
    /// `FINGER_*` and `STUMP_*` constants (notes/research/pastel_blending.md).
    pub fn rub_area(&mut self, m: &crate::Mask, o: AreaRub) -> Result<Rubbed, String> {
        let whole = self.f.whole();
        if m.f.w != whole.w || m.f.h != whole.h || m.data.len() != whole.w * whole.h {
            return Err("smudge: the mask is not this canvas's".into());
        }
        if o.pad_mm2.is_some_and(|p| !(p.is_finite() && (0.5..=400.0).contains(&p))) {
            return Err("smudge: pad is the contact in mm² (0.5 to 400)".into());
        }
        let stump = o.pad_mm2.is_some_and(|p| p <= STUMP_MAX_MM2);
        let force = o.force.unwrap_or(if stump { STUMP_FORCE } else { FINGER_FORCE });
        if !(force.is_finite() && (SMUDGE_MIN_FORCE..=10.0).contains(&force)) {
            return Err(format!("smudge: force is newtons ({SMUDGE_MIN_FORCE} to 10)"));
        }
        let pattern = o.pattern.unwrap_or(if stump { Pattern::Strokes } else { Pattern::Circles });
        let circle_mm = if stump { STUMP_CIRCLE_MM } else { FINGER_CIRCLE_MM };
        let speed = o.speed.unwrap_or(match pattern {
            Pattern::Circles => std::f32::consts::PI * circle_mm * SMUDGE_CIRCLE_HZ,
            Pattern::Strokes if stump => STUMP_SPEED,
            Pattern::Strokes => FINGER_SPEED,
        });
        if !(speed.is_finite() && speed > 0.0) {
            return Err("smudge: speed is mm/s, a number above 0".into());
        }
        let angle_deg = o.angle_deg.unwrap_or(SMUDGE_ANGLE_DEG);
        if !angle_deg.is_finite() || angle_deg.abs() > 360.0 {
            return Err("smudge: angle is degrees (-360 to 360)".into());
        }
        let passes = o.passes.unwrap_or(SMUDGE_PASSES);
        if !(1..=SMUDGE_MAX_PASSES).contains(&passes) {
            return Err(format!("smudge: passes is a whole number, 1 to {SMUDGE_MAX_PASSES}"));
        }
        let mmu = self.mm_per_unit;
        let px = self.px_mm();
        let pi = std::f32::consts::PI;
        let pad_mm2 = o.pad_mm2;
        let full_area = pad_area(force, pad_mm2);
        let r_full = (full_area / pi).sqrt();
        let speed = speed.max(1.0);
        // the lanes: straight strokes run at the hand's speed; circles drift
        // along the lane at SMUDGE_DRIFT while the pad goes round at `speed`
        let (radius, drift) = match pattern {
            Pattern::Strokes => (0.0, speed),
            Pattern::Circles => (0.5 * circle_mm, SMUDGE_DRIFT),
        };
        // a lane sweeps the pad's width, and in circles the circle's too
        let plan = Plan { spacing: (2.0 * r_full + 2.0 * radius) * SMUDGE_LANE / mmu, angle_deg, passes, radius: radius / mmu, turns: if radius > 0.0 { speed / radius * mmu / drift } else { 0.0 } };
        // the finger is down where the mask is above a trace and presses at
        // least the least force a pad is modeled at
        let down = |v: f32| v > SMUDGE_TRACE && force * v >= SMUDGE_MIN_FORCE;
        let (lanes, pieces) = area_lanes(m, &plan, down)?;
        // the work: pad placements (every half radius, the lightest touch's
        // the closest) times the pixels under the full pad
        let lightest = force * SMUDGE_TRACE.max(SMUDGE_MIN_FORCE / force);
        let r_lo = (pad_area(lightest, pad_mm2) / pi).sqrt();
        let len_mm: f32 = pieces.iter().map(|p| (p.s1 - p.s0) * lanes[p.lane].max_rate() * mmu).sum();
        let visits = (len_mm / (0.5 * r_lo) + pieces.len() as f32) * (pi * r_full * r_full / (px * px)).max(1.0);
        if pieces.len() > SMUDGE_MAX_STROKES || visits > SMUDGE_MAX_VISITS {
            return Err(format!(
                "smudge: too much to rub in one go ({} strokes over {:.0} mm); smudge a smaller mask, with fewer passes or a larger pad",
                pieces.len(),
                len_mm
            ));
        }
        let at = |p: (f32, f32)| {
            let f = force * m.sample(p.0, p.1);
            (pad_area(f, pad_mm2) / pi).sqrt()
        };
        // the hand: each piece lands and lifts (`rub` counts only the
        // sliding); between pieces of one lane the hand keeps its motion in
        // the air, and to the next lane it moves straight at `speed`
        let walk = |press: &mut dyn FnMut((f32, f32), f32)| -> f32 {
            let mut secs = 0.0f32;
            let mut prev: Option<&Piece> = None;
            for p in &pieces {
                let lane = &lanes[p.lane];
                if let Some(q) = prev {
                    secs += if q.lane == p.lane {
                        (p.s0 - q.s1).abs() * mmu / drift
                    } else {
                        let (a, b) = (lanes[q.lane].at(q.s1), lane.at(p.s0));
                        ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt() * mmu / speed
                    };
                }
                prev = Some(p);
                secs += LAND_S + LIFT_S;
                let mut s = p.s0;
                while s < p.s1 {
                    // a step of half the pad's radius along the path
                    let ds = 0.5 * at(lane.at(s)) / mmu / lane.rate(s);
                    let ds = if ds >= p.s1 - s { p.s1 - s } else { ds };
                    s = if ds >= p.s1 - s { p.s1 } else { s + ds };
                    secs += ds * mmu / drift;
                    let c = lane.at(s);
                    press((c.0 * mmu, c.1 * mmu), at(c));
                }
            }
            secs
        };
        if self.drawing.is_none() {
            return Ok(Rubbed { secs: walk(&mut |_, _| {}), strokes: pieces.len(), carried_mm3: 0.0, pattern });
        }
        let mut pad = Pad::new(full_area, pad_mm2.is_none());
        let secs = self.rubbing(|r| walk(&mut |c, r_mm| r.press(&mut pad, c, r_mm)));
        Ok(Rubbed { secs, strokes: pieces.len(), carried_mm3: pad.load * 1e-3, pattern })
    }

    /// Run `go` with the drawing open for a finger: the pixels and films
    /// taken out of the canvas while it works, and the layout marked a stick
    /// drawing's if it moved any pastel.
    fn rubbing<R>(&mut self, go: impl FnOnce(&mut Rubbing) -> R) -> R {
        let f = self.f;
        let px = self.px_mm();
        self.drawing_mut();
        let film = std::mem::take(&mut self.film);
        let mut pxs = std::mem::take(&mut self.px);
        let out;
        {
            let wet: Vec<bool> = self.wet.vol.iter().map(|&v| v > 1e-5).collect();
            // (the sheet, where it lies, keeps the finger off the picture)
            let shv: Option<Vec<bool>> = self.sheet.as_ref().map(|s| s.over.clone());
            let d = self.drawing.as_deref_mut().expect("drawing_mut made it");
            let mut r = Rubbing { d, f, px, wet: &wet, sheet: shv.as_deref(), film: &film, pxs: &mut pxs, moved: false };
            out = go(&mut r);
            // (a stick drawing's layout once the finger has moved its pastel)
            let moved = r.moved;
            r.d.sticks |= moved;
        }
        self.px = pxs;
        self.film = film;
        out
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

// ---------------------------------------------------------------- the finger
//
// Sources: notes/research/pastel_blending.md (cited by its section; [M]
// measured, [S] a standard's or vendor's parameter, [P] a practitioner's
// or museum's statement, [E] an estimate) and pastel_stick_tribology.md §6.

/// A fingertip's contact at 1 N, mm² (Dzidek et al. 2017: 124–151 [M];
/// Liu et al. 2018 on paper 107 [M], within the spread between people,
/// Tomlinson 2009 [M]: blending §2.1), growing with the force as
/// F^`FINGER_EXP` (Liu 2018: 0.42 below 2 N; consensus 0.36–0.42 [M]).
pub const FINGER_MM2: f32 = 135.0;
pub const FINGER_EXP: f32 = 0.4;
/// Past this force (N) the contact hardly grows: as F^`FINGER_EXP_HIGH`
/// (Liu 2018: 0.14 at 3–24 N [M]; Tomlinson 2009: a plateau at 2.0–5.5 N
/// [M]: blending §2.1), from the area at the knee (178 mm²), so the law is
/// continuous; 223 mm² at 10 N.
pub const FINGER_KNEE_N: f32 = 2.0;
pub const FINGER_EXP_HIGH: f32 = 0.14;
/// The least force the finger's pad is modeled at, N (`rub` floors the
/// force here). An area smudge lifts the finger where its graded force
/// falls below it.
pub const SMUDGE_MIN_FORCE: f32 = 0.05;
/// The share of the loose pastel under the pad picked up per pass: a soft,
/// oily skin-like pad sliding over a porous surface took 22.2 ± 2.3 % of a
/// dry residue a contact, with no dependence on force or dwell (Amaral et
/// al. 2022 [M]: blending §3.1, §6.8).
const PICK: f32 = 0.22;
/// As a fingertip's skin loads, its pick-up falls to about a third
/// (consecutive presses cut transfer about 3×: Rodes et al. 2001 [M],
/// abstract only: blending §3.1).
const PICK_LOADED: f32 = 1.0 / 3.0;
/// What skin holds when loaded, as pastel µm over its contact: 0.45 mg/cm²
/// (soil on skin 0.21–0.74 mg/cm², Ferguson et al. 2009 [M]; for pastel
/// [E]) at 1250 kg/m³ (a crumb's, `sheet.rs`). A stump's paper: no data;
/// it is not loaded this way.
const SKIN_HOLDS_UM: f32 = 0.45e-2 / 1.25 * 1e3;
/// The share of the pad's load laid down per pass (0.3–0.5 [E], from Sousa
/// & Buchanan 1999's blender, which lays 0.5 and keeps part: blending §6.8).
const LAY: f32 = 0.4;

/// `smudge` along a path: its defaults, N and mm/s (estimates, kept as
/// they were; an area takes the research's below).
pub const SMUDGE_PATH_FORCE: f32 = 1.0;
pub const SMUDGE_PATH_SPEED: f32 = 40.0;

/// A pad of at most this (mm²) is a stump: rigid, its contact set by its
/// shape (flank 10–60 mm², point 1–5 [E]: blending §2.5, §6.2).
pub const STUMP_MAX_MM2: f32 = 60.0;
/// The hand's force on a fingertip working an area, N: tactile exploration
/// at 0.49–0.64 N (Smith et al. 2002 [M]), blending "light" (Orme 2017 [P]);
/// 0.3–1 N [E] (blending §2.2, §6.3).
pub const FINGER_FORCE: f32 = 0.6;
/// A stump's, N: 0.5–2 [E], on pen forces of 0.8–3.2 N [M] (§2.2, §6.3).
pub const STUMP_FORCE: f32 = 1.0;
/// A fingertip's straight strokes, mm/s: free exploration at 77–86 mm/s
/// (Smith et al. 2002 [M]; finger 30–100 [E]: §2.3, §6.4).
pub const FINGER_SPEED: f32 = 80.0;
/// A stump's, mm/s: 20–60 [E] (§6.4).
pub const STUMP_SPEED: f32 = 40.0;
/// Loose circles: a fingertip's 15 mm across (10–20 [E]), a stump's 5 mm
/// (small circles for detail, 3–8 [E]), at 2 turns a second (1.5–3 [E]);
/// "loose circular movements" for a first blend, "small circular
/// movements" for detail (Orme 2017 [P]; Caran d'Ache [P]: §1.2, §6.5). A
/// fingertip goes round at π × 15 × 2 ≈ 94 mm/s, inside the measured
/// 30–100 mm/s.
pub const FINGER_CIRCLE_MM: f32 = 15.0;
pub const STUMP_CIRCLE_MM: f32 = 5.0;
pub const SMUDGE_CIRCLE_HZ: f32 = 2.0;
/// The circles drift along their lane at this, mm/s (5–20 [E]: §2.3, §6.5).
pub const SMUDGE_DRIFT: f32 = 10.0;
/// Lanes are this share of the width a lane sweeps apart (the full pad's
/// width; in circles, plus the circle's diameter), so a point gets at least
/// two contacts a traverse (≤ 0.5 [E]: §6.6).
pub const SMUDGE_LANE: f32 = 0.5;
/// Passes over the area (1–3 [E]; a stump rubbed one way then across,
/// Sousa & Buchanan 1999 [P]: §1.2, §6.7).
pub const SMUDGE_PASSES: u32 = 2;
/// Each further pass is turned by this, degrees (the second at about 90°
/// to the first: Sousa & Buchanan 1999 [P]; perpendicular at color
/// junctions [P]: §6.7).
pub const SMUDGE_CROSS_DEG: f32 = 90.0;
/// The first pass's lanes run at this angle, degrees (0 to the right, 90
/// down the canvas, as a stick's azimuth). No source: an estimate [E]
/// (practitioners pull along the form: give `angle=`).
pub const SMUDGE_ANGLE_DEG: f32 = 30.0;

/// Where an area smudge's pad goes down: where the mask is above this (a
/// trace, as graded hatching: `graphite::hatch_marks_graded`).
pub const SMUDGE_TRACE: f32 = 0.04;
/// Most passes an area smudge takes.
pub const SMUDGE_MAX_PASSES: u32 = 8;
/// The work an area smudge may do (not physics: a bound on time, as
/// `Knife::check_path` bounds a path): pieces the finger is down for,
/// pixels under the pad summed over its placements, and mask samples
/// planning the lanes.
pub const SMUDGE_MAX_STROKES: usize = 20_000;
pub const SMUDGE_MAX_VISITS: f32 = 1.0e9;
pub const SMUDGE_MAX_SAMPLES: f64 = 5.0e7;

/// How a pad works an area.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pattern {
    /// Loose circles drifting along each lane (a fingertip's default).
    Circles,
    /// Straight strokes there and back along each lane (a stump's
    /// default: rubbed back and forth, Orme 2017, Sousa & Buchanan 1999
    /// [P]: blending §1.2).
    Strokes,
}

/// An area smudge's settings: any left out take the defaults above (by
/// the tool: a `pad_mm2` at most `STUMP_MAX_MM2` is a stump).
#[derive(Clone, Copy, Debug, Default)]
pub struct AreaRub {
    pub force: Option<f32>,
    pub pad_mm2: Option<f32>,
    /// mm/s: along a stroke, or round a circle.
    pub speed: Option<f32>,
    pub angle_deg: Option<f32>,
    pub passes: Option<u32>,
    pub pattern: Option<Pattern>,
}

/// What an area smudge did.
#[derive(Clone, Copy, Debug)]
pub struct Rubbed {
    /// Hand time, s.
    pub secs: f32,
    /// Pieces the finger was down for.
    pub strokes: usize,
    /// Pastel still on the finger at the end, mm³ (it goes with it).
    pub carried_mm3: f32,
    pub pattern: Pattern,
}

/// The pad's contact at `force` N, mm²: a fingertip's, or a stump's `pad`.
pub fn pad_area(force: f32, pad: Option<f32>) -> f32 {
    pad.unwrap_or_else(|| {
        if force <= FINGER_KNEE_N {
            FINGER_MM2 * force.max(SMUDGE_MIN_FORCE).powf(FINGER_EXP)
        } else {
            FINGER_MM2 * FINGER_KNEE_N.powf(FINGER_EXP) * (force / FINGER_KNEE_N).powf(FINGER_EXP_HIGH)
        }
    })
}

/// What a finger's pad carries: volume (µm·mm²) and its mean color, and
/// what its skin holds when loaded (µm·mm²; 0 for a stump).
#[derive(Clone, Copy, Default)]
struct Pad {
    load: f32,
    col: [f32; 3],
    holds: f32,
}

impl Pad {
    /// A clean pad of `area` mm², skin (`skin`) or a stump's paper.
    fn new(area: f32, skin: bool) -> Pad {
        Pad { holds: if skin { SKIN_HOLDS_UM * area } else { 0.0 }, ..Pad::default() }
    }
}

/// The drawing open for a finger (`Canvas::rubbing`).
struct Rubbing<'a> {
    d: &'a mut crate::graphite::Drawing,
    f: crate::Frame,
    px: f32,
    wet: &'a [bool],
    sheet: Option<&'a [bool]>,
    film: &'a [f32],
    pxs: &'a mut [Rgb],
    moved: bool,
}

impl Rubbing<'_> {
    /// The pad (radius `r_mm`) at `c` (mm), a step of half its radius on:
    /// it picks up from the loose pastel under it (a skin pad less as it
    /// loads) and lays a share of its load over it, as fine grains.
    fn press(&mut self, pad: &mut Pad, c: (f32, f32), r_mm: f32) {
        let (f, px) = (self.f, self.px);
        let (x0, x1) = (((c.0 - r_mm) / px).floor().max(f.x0 as f32) as usize, (((c.0 + r_mm) / px).ceil() as usize).min(f.x0 + f.w));
        let (y0, y1) = (((c.1 - r_mm) / px).floor().max(f.y0 as f32) as usize, (((c.1 + r_mm) / px).ceil() as usize).min(f.y0 + f.h));
        let mut under = Vec::new();
        for y in y0..y1 {
            for x in x0..x1 {
                let (wx, wy) = ((x as f32 + 0.5) * px, (y as f32 + 0.5) * px);
                if (wx - c.0).powi(2) + (wy - c.1).powi(2) <= r_mm * r_mm {
                    if !self.sheet.is_some_and(|s| s[(y - f.y0) * f.w + (x - f.x0)]) {
                        under.push((y - f.y0) * f.w + (x - f.x0));
                    }
                }
            }
        }
        if under.is_empty() {
            return;
        }
        let a_px = px * px;
        // pick up from the loose pastel under the pad: a step of half
        // its radius is a quarter of a pass over a point
        let share = 0.25;
        let pick = if pad.holds > 0.0 { PICK * (1.0 - (1.0 - PICK_LOADED) * (pad.load / pad.holds).min(1.0)) } else { PICK };
        for &i in &under {
            let cell = &mut self.d.cells[i];
            if cell.loose <= 0.0 || self.wet[i] || self.film[i] > cell.film + 1e-4 {
                continue;
            }
            let take = cell.loose * pick * share;
            let total = pad.load + take * a_px;
            for q in 0..3 {
                pad.col[q] = (pad.col[q] * pad.load + cell.r[q] * take * a_px) / total.max(1e-9);
            }
            pad.load = total;
            let under_px = uncover(self.pxs[i], cell.a, cell.r);
            cell.a = cell.floor + (cell.a - cell.floor).max(0.0) * (1.0 - take / cell.loose);
            cell.loose -= take;
            self.pxs[i] = cover(under_px, cell.a, cell.r);
            self.moved |= take > 0.0;
        }
        // lay a share of the load over the pad, as fine grains
        let give = pad.load * LAY * share;
        if give <= 0.0 {
            return;
        }
        let per = give / (under.len() as f32 * a_px);
        for &i in &under {
            let cell = &mut self.d.cells[i];
            if self.wet[i] {
                continue;
            }
            if cell.film < 0.0 || self.film[i] > cell.film + 1e-4 {
                *cell = Cell { film: self.film[i], ..Cell::default() };
            }
            // fine grains: a third of a stroke's particles' thickness
            let q = 1.0 - (-per / (PARTICLE_UM / 3.0)).exp();
            let under_px = uncover(self.pxs[i], cell.a, cell.r);
            let old = cell.a * (1.0 - q);
            let wsum = old + q;
            cell.r = [0, 1, 2].map(|k| ((old * cell.r[k] + q * pad.col[k]) / wsum.max(1e-6)).clamp(0.0, 1.0));
            cell.lift = (old * cell.lift + q * 0.55) / wsum.max(1e-6);
            cell.a = (cell.a + (1.0 - cell.a) * q).min(0.995);
            cell.loose += per;
            self.pxs[i] = cover(under_px, cell.a, cell.r);
        }
        pad.load -= give;
    }
}

/// How the lanes of an area smudge are laid (units).
struct Plan {
    /// Between lanes.
    spacing: f32,
    angle_deg: f32,
    passes: u32,
    /// The circles' radius (0: straight strokes).
    radius: f32,
    /// The circles' turning, radians per unit the lane advances.
    turns: f32,
}

/// One lane (units): the pad's center at `s` along it is `at(s)`, on a
/// circle of `radius` turning `turns` radians per unit, or on the lane's
/// line itself.
struct Lane {
    o: (f32, f32),
    dir: (f32, f32),
    nrm: (f32, f32),
    radius: f32,
    turns: f32,
}

impl Lane {
    fn at(&self, s: f32) -> (f32, f32) {
        let (c, n) = if self.radius > 0.0 {
            let a = self.turns * s;
            (s + self.radius * a.cos(), self.radius * a.sin())
        } else {
            (s, 0.0)
        };
        (self.o.0 + self.dir.0 * c + self.nrm.0 * n, self.o.1 + self.dir.1 * c + self.nrm.1 * n)
    }
    /// How fast the pad moves as the lane advances (units per unit).
    fn rate(&self, s: f32) -> f32 {
        if self.radius > 0.0 {
            let (rk, a) = (self.radius * self.turns, self.turns * s);
            ((1.0 - rk * a.sin()).powi(2) + (rk * a.cos()).powi(2)).sqrt().max(1e-3)
        } else {
            1.0
        }
    }
    fn max_rate(&self) -> f32 {
        1.0 + self.radius * self.turns
    }
}

/// A piece of a lane the pad is down for, from `s0` to `s1`.
struct Piece {
    lane: usize,
    s0: f32,
    s1: f32,
}

/// The lanes of an area smudge over `m` (units, whole canvas) and the
/// pieces of them the pad is down for: for each pass, lanes `spacing`
/// apart at the pass's angle across the mask's bounding box (the first and
/// last on its edges; circles run a radius past its ends), circles back
/// and forth from lane to lane, strokes there and back along each lane,
/// each cut where `down` stops holding for the mask's value under the
/// pad's center (sampled every half pixel of the pad's path). Refused when
/// the samples would be too many.
fn area_lanes(m: &crate::Mask, p: &Plan, down: impl Fn(f32) -> bool) -> Result<(Vec<Lane>, Vec<Piece>), String> {
    let f = m.f;
    let (w, h) = (f.width(), f.height());
    let half = 0.5 / f.scale;
    let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for (i, &v) in m.data.iter().enumerate() {
        if v > SMUDGE_TRACE {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            x0 = x0.min(x - half);
            y0 = y0.min(y - half);
            x1 = x1.max(x + half);
            y1 = y1.max(y + half);
        }
    }
    let (mut lanes, mut pieces) = (Vec::new(), Vec::new());
    if x0 > x1 {
        return Ok((lanes, pieces));
    }
    let mut work = 0.0f64;
    for k in 0..p.passes {
        let th = (p.angle_deg + SMUDGE_CROSS_DEG * k as f32).to_radians();
        let (ca, sa) = (th.cos(), th.sin());
        let (mut u0, mut u1, mut v0, mut v1) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
        for &(x, y) in &[(x0, y0), (x1, y0), (x0, y1), (x1, y1)] {
            let (u, v) = (x * ca + y * sa, -x * sa + y * ca);
            u0 = u0.min(u);
            u1 = u1.max(u);
            v0 = v0.min(v);
            v1 = v1.max(v);
        }
        let (u0, u1) = (u0 - p.radius, u1 + p.radius);
        let n = ((v1 - v0) / p.spacing.max(1e-3)).ceil().max(0.0) as usize + 1;
        let len = u1 - u0;
        let ds = half / (1.0 + p.radius * p.turns);
        let samples = (len / ds).ceil() as usize + 1;
        work += n as f64 * samples as f64;
        if work > SMUDGE_MAX_SAMPLES {
            return Err(format!("smudge: too much to rub in one go ({} lanes across the mask); smudge a smaller mask, with fewer passes or a larger pad", n * (k as usize + 1)));
        }
        let dv = if n > 1 { (v1 - v0) / (n - 1) as f32 } else { 0.0 };
        let xy = |u: f32, v: f32| (u * ca - v * sa, u * sa + v * ca);
        for li in 0..n {
            let v = if n > 1 { v0 + dv * li as f32 } else { 0.5 * (v0 + v1) };
            // circles: every other lane runs the other way; strokes: each
            // lane is rubbed there and back, so the next starts where the
            // last began
            let there_and_back = p.radius <= 0.0;
            let lane = if there_and_back || li % 2 == 0 {
                Lane { o: xy(u0, v), dir: (ca, sa), nrm: (-sa, ca), radius: p.radius, turns: p.turns }
            } else {
                Lane { o: xy(u1, v), dir: (-ca, -sa), nrm: (sa, -ca), radius: p.radius, turns: p.turns }
            };
            let li = lanes.len();
            let inside = |s: f32| {
                let (x, y) = lane.at(s);
                x >= 0.0 && y >= 0.0 && x < w && y < h && down(m.sample(x, y))
            };
            let mut run: Option<f32> = None;
            for j in 0..=samples {
                let s = (ds * j as f32).min(len);
                let ins = j < samples && inside(s);
                match (run, ins) {
                    (None, true) => run = Some(s),
                    (Some(a), false) => {
                        let b = (s - ds).max(a);
                        if b > a {
                            pieces.push(Piece { lane: li, s0: a, s1: b });
                        }
                        run = None;
                    }
                    _ => {}
                }
            }
            if there_and_back {
                // the way back along the same line: the same pieces, from
                // the far end
                let back = Lane { o: lane.at(len), dir: (-lane.dir.0, -lane.dir.1), nrm: (-lane.nrm.0, -lane.nrm.1), radius: 0.0, turns: 0.0 };
                let there: Vec<(f32, f32)> = pieces.iter().filter(|q| q.lane == li).map(|q| (q.s0, q.s1)).collect();
                lanes.push(lane);
                let bi = lanes.len();
                pieces.extend(there.iter().rev().map(|&(s0, s1)| Piece { lane: bi, s0: len - s1, s1: len - s0 }));
                lanes.push(back);
            } else {
                lanes.push(lane);
            }
        }
    }
    Ok((lanes, pieces))
}

/// A pigment's refractive index (mean of its principal indices), by tube

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paper::Paper;

    #[test]
    fn charcoal_sticks_wear_and_natural_charcoal_lifts_more_readily() {
        let mut retained = Vec::new();
        for (kind, grade, thick) in [("vine", "", false), ("vine", "", true), ("willow", "", false), ("willow", "", true), ("compressed", "", false), ("pencil", "soft", false), ("pencil", "medium", false), ("pencil", "hard", false)] {
            let mut c = Canvas::new_window(120, 1.0, [0.55; 3], None).with_size_mm(60.0).with_paper(Paper::preset("toned laid", 4).unwrap().0);
            let mut stick = Stick::charcoal(kind, grade, thick).unwrap();
            let facets = stick.facets.clone();
            let laid = line(&mut c, &mut stick, 500.0, 1.5, 60.0);
            assert!(laid.volume_mm3 > 0.0, "{kind} {grade} lays material");
            assert_ne!(stick.facets, facets, "{kind} {grade} wears");
            let d = c.drawing.as_ref().unwrap();
            let loose: f32 = d.cells.iter().map(|p| p.loose).sum();
            let bound: f32 = d.cells.iter().map(|p| p.bound).sum();
            assert!(loose > 0.0);
            assert_eq!(bound > 0.0, stick.material.binder > 0.0);
            let before = darkening(&c, 0.0, 1000.0);
            let mask = crate::Mask::full(c.window());
            for _ in 0..4 { c.erase(&mask, 1.0); }
            let after = darkening(&c, 0.0, 1000.0);
            assert!(after < before, "{kind} {grade} can be lifted");
            retained.push(after / before);
            c.fix_drawing(None);
            c.fix_pastel(None, 0.15);
            let fixed: Vec<_> = c.drawing.as_ref().unwrap().cells.iter().map(|c| (c.a, c.bound)).collect();
            c.erase(&mask, 1.0);
            let after: Vec<_> = c.drawing.as_ref().unwrap().cells.iter().map(|c| (c.a, c.bound)).collect();
            assert_eq!(after, fixed);
        }
        assert!(retained[0] < retained[4], "vine lifts more than compressed: {retained:?}");
        assert!(retained[2] < retained[4], "willow lifts more than compressed: {retained:?}");
    }

    #[test]
    fn charcoal_powder_enters_the_existing_stump_brush_and_eraser_paths() {
        let mut c = Canvas::new_window(120, 1.0, [0.55; 3], None).with_size_mm(60.0).with_paper(Paper::preset("newsprint", 4).unwrap().0);
        let mask = crate::Mask::from_fn(c.window(), |x,y| if (200.0..400.0).contains(&x) && (350.0..650.0).contains(&y) { 1.0 } else { 0.0 });
        let blank = c.pixels().to_vec();
        let volume = c.charcoal_powder(&mask, 1.0, 7);
        assert!(volume > 0.0);
        assert_ne!(c.pixels(), blank);
        for (i, px) in c.pixels().iter().enumerate() {
            if mask.data[i] == 0.0 { assert_eq!(*px, blank[i]); }
        }
        let before = c.pixels().to_vec();
        c.rub(&[(250.0,500.0),(650.0,500.0)], 1.0, Some(4.0), 40.0);
        assert_ne!(c.pixels(), before);
        assert!(c.drawing.as_ref().unwrap().cells.iter().enumerate().any(|(i,p)| mask.data[i] == 0.0 && p.loose > 0.0));
        let mut held = 0.0;
        c.dust_pastel(100.0, 5.0, &[(200.0,450.0),(700.0,450.0)], &mut held, 10.0);
        assert!(held > 0.0, "the brush collects powder");
        let before = darkening(&c, 0.0, 1000.0);
        c.erase(&crate::Mask::full(c.window()), 1.0);
        assert!(darkening(&c, 0.0, 1000.0) < before);
    }

    fn sheet() -> Canvas {
        Canvas::new_window(600, 1.0, [0.55; 3], None).with_size_mm(150.0).with_paper(Paper::drawing(4))
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

    #[test]
    fn finite_extreme_angles_finish_a_stroke() {
        for (a, b) in [(0.0, 1e12_f32.to_radians()), (f32::MAX, -f32::MAX)] {
            let mut c = Canvas::new_window(40, 1.0, [0.55; 3], None).with_size_mm(40.0);
            let mut st = Stick::round([0.02; 3], 0.8, 2.0);
            let pts = [
                StrokePoint { x: 450.0, y: 500.0, pose: Pose { az: a, roll: -a, ..pose(1.0, 80.0) }, speed: 50.0 },
                StrokePoint { x: 550.0, y: 500.0, pose: Pose { az: b, roll: -b, ..pose(1.0, 80.0) }, speed: 50.0 },
            ];
            let laid = c.stick_stroke(&mut st, &pts);
            assert!(laid.secs > 0.0 && laid.secs.is_finite());
            assert!(laid.volume_mm3.is_finite());
            assert!(c.pixels().iter().flatten().all(|v| v.is_finite()));
        }
    }

    #[test]
    fn smudge_lightens_its_source_and_makes_erasable_deposits() {
        let mut c = Canvas::new_window(40, 1.0, [0.55; 3], None).with_size_mm(40.0);
        let i = 20 * 40 + 20;
        let source = Cell { a: 0.8, r: [0.02; 3], lift: 0.55, floor: 0.3, film: 0.0, loose: 12.0, bound: 8.0 };
        c.drawing_mut().cells[i] = source;
        c.px[i] = cover(c.px[i], source.a, source.r);
        let before = c.px[i];
        c.rub(&[(475.0, 500.0), (500.0, 500.0)], 1.0, Some(std::f32::consts::PI * 4.0), 40.0);
        let cells = &c.drawing.as_ref().unwrap().cells;
        assert!(cells[i].a < source.a && cells[i].a >= source.floor);
        assert!(cells[i].loose < source.loose);
        assert_eq!(cells[i].bound, source.bound);
        assert_eq!(cells[i].floor, source.floor);
        assert!(c.px[i][0] > before[0]);
        let deposits: Vec<_> = cells.iter().enumerate().filter(|&(j, cell)| j != i && cell.a > 0.0).map(|(j, cell)| {
            assert!((cell.lift - 0.55).abs() < 1e-6);
            (j, cell.a, cell.loose)
        }).collect();
        assert!(!deposits.is_empty());
        c.erase(&crate::Mask::full(c.window().whole()), 1.0);
        let cells = &c.drawing.as_ref().unwrap().cells;
        for (j, coverage, loose) in deposits {
            assert!(cells[j].a < coverage);
            assert!(cells[j].loose < loose);
        }
        assert_eq!(cells[i].bound, source.bound);
        assert!(cells[i].a >= source.floor);
    }

    #[test]
    fn erasing_loose_pastel_preserves_bound_pigment_and_releases_the_tooth() {
        for (coverage, floor) in [(0.8, 0.0), (0.99, 0.9), (0.995, 0.995)] {
            let mut c = Canvas::new_window(4, 1.0, [0.55; 3], None).with_size_mm(4.0);
            let source = Cell { a: coverage, r: [0.02; 3], lift: 0.55, floor, film: 0.0, loose: 12.0, bound: 8.0 };
            c.drawing_mut().cells[0] = source;
            c.px[0] = cover(c.px[0], source.a, source.r);
            c.erase(&crate::Mask::full(c.window().whole()), 1.0);
            let cell = c.drawing.as_ref().unwrap().cells[0];
            assert_eq!(cell.bound, source.bound);
            assert!(cell.a >= floor);
            if floor > 0.0 {
                assert_eq!(cell.a, floor);
                assert_eq!(cell.loose, 0.0);
            } else {
                assert!((cell.loose / source.loose - cell.a / source.a).abs() < 1e-6);
                assert!(cell.loose < source.loose);
            }
        }
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

    /// The pixels a stick held at full force touches along the line from
    /// x0 to x1 at `y` (with a stroke's worn stick, about all its contacts:
    /// the stroke's force tapers at its ends, and its facet grew on the way).
    fn contacts(c: &Canvas, st: &Stick, y: f32, x0: f32, x1: f32, force: f32, alt: f32) -> std::collections::BTreeSet<usize> {
        let f = c.window();
        let mut out = std::collections::BTreeSet::new();
        let step = 1.0 / f.scale;
        let mut x = x0;
        while x <= x1 {
            let sp = StrokePoint { x, y, pose: pose(force, alt), speed: 50.0 };
            if let Some(s) = c.seat_stick(st, &sp) {
                for (j, &(i, b)) in s.under.iter().enumerate() {
                    let (hs, mu, k) = s.surf[j];
                    if seat(hs - b * 1000.0 + s.delta, mu, k, st.hardness).1 > 0.0 {
                        out.insert(i);
                    }
                }
            }
            x += step;
        }
        out
    }

    fn loose_mm3(c: &Canvas, keep: impl Fn(usize) -> bool) -> f32 {
        let a = c.px_mm() * c.px_mm();
        c.drawing.as_ref().map_or(0.0, |d| d.cells.iter().enumerate().filter(|&(i, _)| keep(i)).map(|(_, c)| c.loose).sum::<f32>() * a * 1e-3)
    }

    /// Crumbs: most of what a contact abrades (SHED) rides under the face
    /// and settles into the paper along and beside it, so a stroke's pastel
    /// lies on half as many pixels again as the stick touches (all the worn
    /// stick would touch at full force along the stroke), wider than its
    /// contact on both sides, with over 40% of its volume off it; and all
    /// of it is accounted for: what lies in the tooth is what the stroke
    /// says it laid.
    #[test]
    fn crumbs_spread_a_stroke_beyond_its_contact() {
        let mut c = sheet();
        let mut st = Stick::round([0.05; 3], 0.8, 12.0);
        let pts = [StrokePoint { x: 300.0, y: 500.0, pose: pose(2.0, 60.0), speed: 50.0 }, StrokePoint { x: 700.0, y: 500.0, pose: pose(2.0, 60.0), speed: 50.0 }];
        let out = c.stick_stroke(&mut st, &pts);
        assert!(out.volume_mm3 > 0.0 && out.on_wet == 0.0);
        let laid = loose_mm3(&c, |_| true);
        assert!((laid - out.volume_mm3).abs() < 1e-4 * out.volume_mm3, "conserved: {laid} in the tooth, {} laid", out.volume_mm3);
        let touch = contacts(&c, &st, 500.0, 300.0, 700.0, 2.0, 60.0);
        let f = c.window();
        let d = c.drawing.as_ref().unwrap();
        let has: Vec<usize> = (0..d.cells.len()).filter(|&i| d.cells[i].loose > 0.0).collect();
        let (r0, r1) = (touch.iter().map(|i| i / f.w).min().unwrap(), touch.iter().map(|i| i / f.w).max().unwrap());
        let (above, below) = (has.iter().any(|i| i / f.w < r0), has.iter().any(|i| i / f.w > r1));
        let off_share = loose_mm3(&c, |i| !touch.contains(&i)) / laid;
        assert!(2 * has.len() > 3 * touch.len(), "pastel on {} pixels, the stick touched {}", has.len(), touch.len());
        assert!(above && below, "crumbs settle beside the contact on both sides");
        assert!(off_share > 0.4, "crumbs carry pastel off the contact: {off_share}");
    }

    /// Crumbs that settle on wet paint are lost in it, not laid: with wet
    /// paint beside the stroke (none where the stick touches), the dry
    /// paper gets exactly what it got without it, the wet paint none, and
    /// what the crumbs would have laid there is reported lost.
    #[test]
    fn crumbs_on_wet_paint_are_lost() {
        let pts = [StrokePoint { x: 300.0, y: 500.0, pose: pose(2.0, 60.0), speed: 50.0 }, StrokePoint { x: 700.0, y: 500.0, pose: pose(2.0, 60.0), speed: 50.0 }];
        let (mut dry, mut wet) = (sheet(), sheet());
        let mut s1 = Stick::round([0.05; 3], 0.8, 12.0);
        let mut s2 = s1.clone();
        // (where the stick touches: from a stroke on a third sheet)
        let mut probe = sheet();
        let mut s0 = s1.clone();
        probe.stick_stroke(&mut s0, &pts);
        let touch = contacts(&probe, &s0, 500.0, 300.0, 700.0, 2.0, 60.0);
        let f = wet.window();
        let (r0, r1) = (touch.iter().map(|i| i / f.w).min().unwrap(), touch.iter().map(|i| i / f.w).max().unwrap());
        // wet paint everywhere but the contact's rows and one more each side
        // (a film's worth of paint marked wet, the surface left as it was)
        let is_wet = |i: usize| i / f.w + 1 < r0 || i / f.w > r1 + 1;
        for i in 0..wet.wet.vol.len() {
            if is_wet(i) {
                wet.wet.vol[i] = 1.0;
            }
        }
        let a = dry.stick_stroke(&mut s1, &pts);
        let b = wet.stick_stroke(&mut s2, &pts);
        assert_eq!(s1, s2, "the stick wears the same");
        let (dd, dw) = (dry.drawing.as_ref().unwrap(), wet.drawing.as_ref().unwrap());
        for i in 0..dd.cells.len() {
            if is_wet(i) {
                assert!(dw.cells[i].loose == 0.0 && dw.cells[i].a == 0.0, "laid on wet paint at {i}");
            } else {
                assert!(dd.cells[i].loose == dw.cells[i].loose && dd.cells[i].a == dw.cells[i].a, "the dry paper differs at {i}");
            }
        }
        let beside = loose_mm3(&dry, is_wet);
        assert!(beside > 0.005 * a.volume_mm3, "crumbs settle beside the contact: {beside} of {}", a.volume_mm3);
        let lost = b.on_wet * (b.volume_mm3 / (1.0 - b.on_wet));
        assert!((a.volume_mm3 - b.volume_mm3 - beside).abs() < 1e-3 * a.volume_mm3, "laid {} vs {} with {beside} beside", a.volume_mm3, b.volume_mm3);
        assert!((lost - beside).abs() < 1e-3 * a.volume_mm3, "lost {lost}, the crumbs beside {beside}");
    }

    /// The eraser lifts loose pastel with what it lifts: the volume in the
    /// tooth falls with the coverage, so a full, erased passage takes pastel
    /// again; fixed pastel stays.
    #[test]
    fn the_eraser_lifts_loose_pastel() {
        let mut c = sheet();
        let mut st = Stick::round([0.1, 0.1, 0.5], 0.8, 12.0);
        for _ in 0..4 {
            line(&mut c, &mut st, 500.0, 3.0, 55.0);
        }
        let mut kept = c.clone();
        let mut fixed = c.clone();
        fixed.fix_pastel(None, 0.15);
        let before = c.drawing.clone().unwrap();
        let full = crate::Mask::full(c.frame());
        c.erase(&full, 1.0);
        let after = c.drawing.as_ref().unwrap();
        let (mut v0, mut v1) = (0.0f32, 0.0f32);
        for (p, q) in before.cells.iter().zip(&after.cells) {
            if p.loose > 0.0 {
                assert!((q.loose - p.loose * q.a / p.a).abs() <= 1e-5 * p.loose, "loose follows coverage: {} → {} as {} → {}", p.loose, q.loose, p.a, q.a);
                v0 += p.loose;
                v1 += q.loose;
            }
        }
        assert!(v1 < 0.8 * v0, "lifted: {v0} → {v1}");
        let fixed_before = fixed.drawing.clone().unwrap();
        fixed.erase(&full, 1.0);
        assert!(fixed.drawing.as_ref().unwrap().cells.iter().zip(&fixed_before.cells).all(|(p, q)| p.bound == q.bound && p.loose == q.loose), "fixed pastel stays");
        let (on_erased, on_full) = (line(&mut c, &mut st.clone(), 500.0, 3.0, 55.0).volume_mm3, line(&mut kept, &mut st, 500.0, 3.0, 55.0).volume_mm3);
        assert!(on_erased > 1.2 * on_full, "an erased passage takes more: {on_erased} vs {on_full}");
    }

    #[test]
    fn the_eraser_lifts_loose_pastel_over_fixed_coverage() {
        for coverage in [0.9, 0.995] {
            let mut c = Canvas::new_window(32, 1.0, [0.55; 3], None).with_size_mm(8.0);
            let i = c.pixels().len() / 2;
            c.drawing_mut().cells[i] = Cell { a: coverage, r: [0.1; 3], lift: 0.55, loose: 10.0, ..Cell::default() };
            c.px[i] = cover(c.px[i], coverage, [0.1; 3]);
            c.fix_pastel(None, 0.0);
            let px_um2 = (c.px_mm() * 1000.0).powi(2);
            lay_crumbs(c.drawing.as_mut().unwrap(), &c.film, &mut c.px, &[(i, 1.0, false)], 5.0, [0.1; 3], &mut crate::rng::Rng::new(7), 150.0, px_um2);
            let before = c.drawing.as_ref().unwrap().cells[i];
            assert_eq!(before.floor, coverage);
            assert!(before.loose > 0.0);
            if coverage == 0.995 {
                assert_eq!(before.a, coverage, "new crumbs at the coverage cap");
            } else {
                assert!(before.a > coverage, "new crumbs above the fixed floor");
            }
            let full = crate::Mask::full(c.frame());
            c.erase(&full, 0.0);
            assert_eq!(c.drawing.as_ref().unwrap().cells[i].loose, before.loose);
            for _ in 0..8 {
                c.erase(&full, 1.0);
            }
            let after = c.drawing.as_ref().unwrap().cells[i];
            assert!(after.loose < 0.01 * before.loose, "fixed coverage {coverage} must not trap loose pastel: {} → {}", before.loose, after.loose);
            assert_eq!(after.bound, before.bound);
            assert_eq!(after.floor, coverage);
            assert_eq!(after.a, coverage);
        }
    }

    // ------------------------------------------------ smudging an area

    /// A small sheet (75 mm, 300 px: 0.25 mm a pixel) with two bands of
    /// loose stick pastel touching at y = 400: red above, blue below, from
    /// x = 150 to 850.
    fn bands() -> Canvas {
        let mut c = Canvas::new_window(300, 1.0, [0.55; 3], None).with_size_mm(75.0).with_paper(Paper::drawing(4));
        let mut red = Stick::round([0.6, 0.04, 0.03], 0.8, 12.0);
        let mut blue = Stick::round([0.03, 0.05, 0.6], 0.8, 12.0);
        for k in 0..5 {
            for (st, y) in [(&mut red, 305.0 + 20.0 * k as f32), (&mut blue, 405.0 + 20.0 * k as f32)] {
                let pts = [StrokePoint { x: 150.0, y, pose: pose(3.0, 80.0), speed: 50.0 }, StrokePoint { x: 850.0, y, pose: pose(3.0, 80.0), speed: 50.0 }];
                c.stick_stroke(st, &pts);
            }
        }
        c
    }

    fn rect_mask(c: &Canvas, x0: f32, y0: f32, x1: f32, y1: f32) -> crate::Mask {
        crate::Mask::from_fn(c.frame(), |x, y| if x >= x0 && x < x1 && y >= y0 && y < y1 { 1.0 } else { 0.0 })
    }

    /// The pastel in the tooth, loose and bound, µm·mm² (f64 sums).
    fn tooth(c: &Canvas) -> (f64, f64) {
        let a = (c.px_mm() * c.px_mm()) as f64;
        let d = c.drawing.as_ref().unwrap();
        (d.cells.iter().map(|c| c.loose as f64).sum::<f64>() * a, d.cells.iter().map(|c| c.bound as f64).sum::<f64>() * a)
    }

    /// An area smudge is the finger driven back and forth over the mask:
    /// over two touching bands it carries each color into the other across
    /// the boundary; pixels beyond the pad's reach from the mask are
    /// untouched, bit for bit; the hand time is the walk's (the same on a
    /// blank sheet), at least every stroke's landing and lift.
    #[test]
    fn an_area_smudge_carries_each_color_into_the_other_inside_the_mask() {
        let mut c = bands();
        let before = c.pixels().to_vec();
        // the pastel's own color in the tooth near the boundary (the cells'
        // flakes, weighted by what they cover: the paper and how much
        // pastel lies there aside)
        let hue = |c: &Canvas, y0: f32, y1: f32, g: fn(Rgb) -> f32| {
            let f = c.window();
            let d = c.drawing.as_ref().unwrap();
            let (mut s, mut w) = (0.0f32, 0.0f32);
            for (i, cell) in d.cells.iter().enumerate() {
                let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
                if (350.0..650.0).contains(&x) && (y0..y1).contains(&y) {
                    s += cell.a * g(cell.r);
                    w += cell.a;
                }
            }
            s / w.max(1e-9)
        };
        let (blue, red): (fn(Rgb) -> f32, fn(Rgb) -> f32) = (|p| p[2] - p[0], |p| p[0] - p[2]);
        // (inside each band, away from the boundary its crumbs already mix)
        let (blue_in_red0, red_in_blue0) = (hue(&c, 320.0, 370.0, blue), hue(&c, 430.0, 480.0, red));
        let m = rect_mask(&c, 300.0, 250.0, 700.0, 550.0);
        let out = c.rub_area(&m, AreaRub::default()).unwrap();
        assert_eq!(out.pattern, Pattern::Circles, "a fingertip works in circles");
        let (blue_in_red1, red_in_blue1) = (hue(&c, 320.0, 370.0, blue), hue(&c, 430.0, 480.0, red));
        assert!(blue_in_red1 > blue_in_red0 + 0.005, "blue carried into the red: {blue_in_red0} → {blue_in_red1}");
        assert!(red_in_blue1 > red_in_blue0 + 0.005, "red carried into the blue: {red_in_blue0} → {red_in_blue1}");
        // the pad's reach: its radius at the finger's force (5.9 mm at 0.6
        // N, 79 units: its center stays on the mask) and a pixel
        let reach = (pad_area(FINGER_FORCE, None) / std::f32::consts::PI).sqrt() / c.mm_per_unit + 2.0 / c.window().scale;
        let f = c.window();
        let (mut far, mut changed) = (0usize, 0usize);
        for (i, (p, q)) in before.iter().zip(c.pixels()).enumerate() {
            let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
            if x < 300.0 - reach || x > 700.0 + reach || y < 250.0 - reach || y > 550.0 + reach {
                assert!(p == q, "touched beyond the pad's reach at ({x}, {y})");
                far += 1;
            } else if p != q {
                changed += 1;
            }
        }
        assert!(far > 10_000 && changed > 5_000, "{far} far, {changed} changed");
        let blank = Canvas::new_window(300, 1.0, [0.55; 3], None).with_size_mm(75.0).with_paper(Paper::drawing(4)).rub_area(&m, AreaRub::default()).unwrap();
        assert_eq!((blank.secs, blank.strokes), (out.secs, out.strokes), "the hand's walk doesn't depend on the pastel");
        assert!(out.strokes >= 4 && out.secs > out.strokes as f32 * (LAND_S + LIFT_S), "{out:?}");
    }

    /// What the finger moves is conserved: the loose pastel in the tooth
    /// before is what is in it after plus what the finger still carries
    /// (carried from stroke to stroke, it leaves with the finger); bound
    /// pastel is untouched.
    #[test]
    fn an_area_smudge_conserves_the_pastel_with_what_the_finger_carries() {
        let mut c = bands();
        let (l0, b0) = tooth(&c);
        let m = rect_mask(&c, 300.0, 250.0, 700.0, 550.0);
        let out = c.rub_area(&m, AreaRub { force: Some(2.0), ..AreaRub::default() }).unwrap();
        let (l1, b1) = tooth(&c);
        let carried = out.carried_mm3 as f64 * 1e3;
        assert!(carried > 0.0 && l1 < l0, "the finger took some: {carried}");
        assert!((l1 + carried - l0).abs() < 1e-4 * l0, "loose {l0} → {l1} + carried {carried}");
        assert_eq!(b0, b1);
        // a stump too
        let (l0, _) = tooth(&c);
        let out = c.rub_area(&m, AreaRub { pad_mm2: Some(4.0), angle_deg: Some(-20.0), passes: Some(1), ..AreaRub::default() }).unwrap();
        assert_eq!(out.pattern, Pattern::Strokes, "a stump works in strokes");
        let (l1, _) = tooth(&c);
        let carried = out.carried_mm3 as f64 * 1e3;
        assert!((l1 + carried - l0).abs() < 1e-4 * l0, "stump: loose {l0} → {l1} + carried {carried}");
    }

    /// Fixed pastel stays: over a fixed passage the finger finds nothing
    /// loose, and nothing changes.
    #[test]
    fn an_area_smudge_leaves_fixed_pastel() {
        let mut c = bands();
        c.fix_pastel(None, 0.15);
        let (px, cells) = (c.pixels().to_vec(), c.drawing.as_ref().unwrap().cells.iter().map(|c| (c.loose, c.bound, c.a)).collect::<Vec<_>>());
        let out = c.rub_area(&rect_mask(&c, 300.0, 250.0, 700.0, 550.0), AreaRub { force: Some(3.0), ..AreaRub::default() }).unwrap();
        assert!(out.carried_mm3 == 0.0 && out.strokes > 0);
        assert!(c.pixels() == &px[..], "the pixels are as they were");
        assert!(c.drawing.as_ref().unwrap().cells.iter().map(|c| (c.loose, c.bound, c.a)).collect::<Vec<_>>() == cells, "the tooth is as it was");
    }

    /// A soft-edged mask is a lighter touch at its edge: straight strokes
    /// along the ramp (angle 90°, one pass) press the force × the mask's
    /// value, a fingertip's pad shrinks with it while the lanes are spaced
    /// for the full pad, and below the least force the finger lifts: at the
    /// edge less loose pastel moves (is picked up or laid) than in the
    /// middle. (In circles, whose many contacts overlap, the edge moves
    /// nearly as much: pick-up hardly depends on force.)
    #[test]
    fn a_soft_edged_mask_moves_less_at_its_edge() {
        let mut c = bands();
        let before = c.drawing.as_ref().unwrap().cells.iter().map(|c| c.loose).collect::<Vec<_>>();
        let m = crate::Mask::from_fn(c.frame(), |x, y| if (250.0..550.0).contains(&y) && x < 800.0 { ((x - 200.0) / 300.0).clamp(0.0, 1.0) } else { 0.0 });
        let out = c.rub_area(&m, AreaRub { force: Some(1.0), angle_deg: Some(90.0), passes: Some(1), pattern: Some(Pattern::Strokes), ..AreaRub::default() }).unwrap();
        assert!(out.strokes > 0);
        let f = c.window();
        let d = c.drawing.as_ref().unwrap();
        let moved = |x0: f32, x1: f32| {
            let (mut s, mut n) = (0.0f32, 0usize);
            for (i, (p, q)) in before.iter().zip(&d.cells).enumerate() {
                let (x, y) = (f.ux(i % f.w), f.uy(i / f.w));
                if x >= x0 && x < x1 && (320.0..480.0).contains(&y) {
                    s += (q.loose - p).abs();
                    n += 1;
                }
            }
            s / n as f32
        };
        let (edge, middle) = (moved(230.0, 300.0), moved(600.0, 700.0));
        assert!(middle > 0.0 && edge < 0.75 * middle, "edge (mask 0.1–0.33) {edge} µm, middle {middle} µm: {}", edge / middle);
    }

    /// Area smudges refuse what they can't do, plainly, and do nothing.
    #[test]
    fn an_area_smudge_refuses_bad_values() {
        let mut c = bands();
        let m = rect_mask(&c, 300.0, 250.0, 700.0, 550.0);
        let px = c.pixels().to_vec();
        let o = AreaRub::default();
        for (r, want) in [
            (c.clone().rub_area(&m, AreaRub { force: Some(f32::NAN), ..o }), "force"),
            (c.clone().rub_area(&m, AreaRub { pad_mm2: Some(0.0), ..o }), "pad"),
            (c.clone().rub_area(&m, AreaRub { speed: Some(0.0), ..o }), "speed"),
            (c.clone().rub_area(&m, AreaRub { angle_deg: Some(f32::INFINITY), ..o }), "angle"),
            (c.clone().rub_area(&m, AreaRub { passes: Some(0), ..o }), "passes"),
            (c.clone().rub_area(&crate::Mask::full(crate::Frame::new(10, 10, 0.01)), o), "not this canvas's"),
            // (a stump over the whole of a 2 m sheet, eight times)
            (Canvas::new_window(300, 1.0, [0.55; 3], None).with_size_mm(2000.0).rub_area(&crate::Mask::full(c.frame()), AreaRub { pad_mm2: Some(0.5), passes: Some(8), ..o }), "too much"),
        ] {
            let e = r.unwrap_err();
            assert!(e.contains(want), "{want}: {e}");
        }
        assert!(c.rub_area(&crate::Mask::empty(c.frame()), o).unwrap().strokes == 0);
        assert!(c.pixels() == &px[..]);
    }

    /// A fingertip's contact: 135 mm² at 1 N growing as F^0.4 up to 2 N,
    /// then hardly (F^0.14), with no step at the knee; a stump's is its own.
    #[test]
    fn the_finger_contact_levels_off_past_2_n() {
        assert!((pad_area(1.0, None) - 135.0).abs() < 1e-3);
        let knee = pad_area(2.0, None);
        assert!((knee - 178.1).abs() < 0.1, "{knee}");
        assert!((pad_area(2.0001, None) - knee).abs() < 0.01);
        assert!((pad_area(10.0, None) - 223.1).abs() < 0.5, "{}", pad_area(10.0, None));
        assert_eq!(pad_area(10.0, Some(4.0)), 4.0);
    }
}
