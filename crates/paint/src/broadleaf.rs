//! Broadleaved trees (oak, beech, lime, birch, willow) grown into a crown
//! the painter draws, and groups of field trees in depth from a few drawn
//! crowns. Geometry only, as in `growth` and `fir`: limbs as polylines with
//! widths, leaf clumps on the grown twigs, the light on them, and the short
//! hooked touches a pointed brush lays on them.
//!
//! How it grows (space colonization, Runions, Lane and Prusinkiewicz,
//! "Modeling trees with a space colonization algorithm", 2007):
//!
//! - attraction points fill the drawn crown (in depth too: the crown is a
//!   solid about as deep as it is wide), thinned by a low noise so the crown
//!   has its own gaps where the limb masses part;
//! - the trunk the painter draws is the first wood. Only a few nodes on it
//!   may sprout (where the scaffold limbs leave it, and the fork at its top),
//!   so limbs come from the trunk, not from everywhere;
//! - every round each attraction point pulls the nearest node within reach;
//!   a pulled node grows one step toward the mean of its pulls, bent by the
//!   species' habit (oak: outward and level, crooked, with sympodial kinks
//!   that persist down a limb; beech: rising and smooth; birch: a leading
//!   stem and hanging twigs; lime: dense to the shell; willow: rods rising
//!   from a pollard head); points reached by wood die;
//! - widths follow the pipe model from equal twigs down to the trunk width
//!   the species has; limbs follow the thickest path through each fork;
//! - fine twigs, too small for the attraction grid, are added at the tips
//!   (and, for birch and willow, hanging along the thin wood);
//! - leaves come as clumps on the leafy twigs, sized by species and season
//!   (spring small, autumn thinned and turned, winter bare but for a few dead
//!   leaves an oak or beech keeps), lit by facing on the crown and shaded by
//!   the clumps between them and the sun.
//!
//! Deterministic by seed. What a painter draws of the wood (which limbs,
//! the strokes along them, the undrawn twigs' tone) is in `wood`.

mod wood;
pub use wood::WoodStroke;

use crate::noise::Fbm;
use crate::{Frame, Mask, Rng, Shape, lerp, smoothstep};
use rayon::prelude::*;
use std::collections::HashMap;
use std::f32::consts::{PI, TAU};

type V3 = [f32; 3];

fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn mul(a: V3, k: f32) -> V3 {
    [a[0] * k, a[1] * k, a[2] * k]
}
fn dot(a: V3, b: V3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn len(a: V3) -> f32 {
    dot(a, a).sqrt()
}
fn unit(a: V3) -> V3 {
    let l = len(a).max(1e-6);
    mul(a, 1.0 / l)
}

/// How a species grows and carries its leaves (the drawn crown sets size
/// and outline). Lengths are fractions of the crown's height unless noted.
#[derive(Clone, Debug)]
pub struct Species {
    pub name: String,
    /// Growth step (one internode of the model).
    pub step: f32,
    /// Attraction points per step² of crown area.
    pub density: f32,
    /// Reach of an attraction point, in steps; and the distance at which
    /// wood kills it.
    pub influence: f32,
    pub kill: f32,
    /// Tropisms added to each step: + up (- down), + outward from the axis.
    pub up: f32,
    pub out: f32,
    /// Random turn per step, and chance per step of a sympodial kink (a
    /// lasting change of direction, as when an oak's terminal bud aborts).
    pub crook: f32,
    pub kink: f32,
    /// How much a shoot keeps its last direction.
    pub inertia: f32,
    /// Attraction points crowd toward the crown's shell (0 even .. 1 shell).
    pub shell: f32,
    /// Gaps in the crown: how much a low noise thins the points, and its
    /// size as a fraction of the crown's width.
    pub voids: f32,
    pub void_size: f32,
    /// Crown depth as a fraction of its width.
    pub depth: f32,
    /// Scaffold limbs leaving the trunk inside the crown (min, max); 0 for a
    /// pollard head (everything from the top of the trunk).
    pub scaffold: (u32, u32),
    /// Trunk width at the foot.
    pub trunk: f32,
    /// Twig width (units at a crown 400 tall; scaled).
    pub twig_w: f32,
    /// Smoothing passes on the wood (0 crooked .. 4 smooth).
    pub smooth: u32,
    /// Twigs per tip, their length (steps), spread (radians), droop
    /// (pendulous) and zigzag; twigs hanging along thin wood per step.
    pub twigs: f32,
    pub twig_len: f32,
    pub twig_spread: f32,
    pub twig_droop: f32,
    pub twig_zig: f32,
    pub twig_along: f32,
    /// Leaf clump radius, height/width, how far it hangs (radii), how solid.
    pub clump: f32,
    pub squash: f32,
    pub hang: f32,
    pub fill: f32,
    /// Irregularity of a clump's outline.
    pub ragged: f32,
    /// Clumps per step of leafy twig.
    pub leafiness: f32,
    /// Wood thinner than this many twig widths carries leaves.
    pub leafy_w: f32,
    /// Leaf touch length, width (of length), hook (radians), and how the
    /// touches lie: `droop` 0 any way .. 1 hanging; `flat` level sprays.
    pub touch: f32,
    pub touch_w: f32,
    pub hook: f32,
    pub droop: f32,
    pub flat: f32,
    /// Touches per clump.
    pub touches: f32,
    /// Share of winter clumps kept as dead leaves (young oak, beech).
    pub marcescent: f32,
    /// How far the trunk goes on into the crown as a leader (share of the
    /// crown's height from its base) if the drawn trunk stops short.
    pub leader: f32,
    /// Pipe-model exponent (higher: limbs stay stouter against the trunk).
    pub pipe: f32,
    /// Angular wood (0: smooth curves, a beech's rising limbs). Any value
    /// above 0 switches on the angular habit (`is_angular`): an oak's limbs
    /// run fairly straight between their nodes and change direction at them
    /// (elbows, sympodial zigzags), no shoot turns back more than about 70
    /// degrees off its heading, twigs stand well off their limb and zigzag
    /// away from it, and the wood is stroked straight from node to node.
    /// The size of the value only sets how much is straightened: wiggles
    /// smaller than `angular` × half a model step between two forks go.
    /// (Oak 1; lime 0.5 has the whole habit with half the oak's straightening.)
    pub angular: f32,
}

impl Species {
    /// The angular habit is on (see `angular`).
    pub fn is_angular(&self) -> bool {
        self.angular > 0.0
    }

    /// Pedunculate oak: a broad crown of crooked, level limbs, sympodial
    /// zigzags, lobed leaf masses with sky between them.
    pub fn oak() -> Self {
        Species {
            name: "oak".into(),
            step: 0.024,
            density: 0.55,
            influence: 7.0,
            kill: 1.6,
            up: -0.12,
            out: 0.7,
            crook: 0.45,
            kink: 0.1,
            inertia: 0.35,
            shell: 0.45,
            voids: 0.55,
            void_size: 0.3,
            depth: 0.8,
            scaffold: (2, 4),
            trunk: 0.075,
            twig_w: 0.55,
            smooth: 1,
            twigs: 3.2,
            twig_len: 1.1,
            twig_spread: 0.9,
            twig_droop: 0.0,
            twig_zig: 0.45,
            twig_along: 0.35,
            clump: 0.026,
            squash: 0.72,
            hang: 0.2,
            fill: 0.85,
            ragged: 0.6,
            leafiness: 0.9,
            leafy_w: 3.2,
            touch: 0.013,
            touch_w: 0.4,
            hook: 0.9,
            droop: 0.25,
            flat: 0.1,
            touches: 7.0,
            marcescent: 0.025,
            leader: 0.3,
            pipe: 2.7,
            angular: 1.0,
        }
    }
    /// Beech: smooth gray limbs rising in a fan, level layered sprays, a
    /// dense crown.
    pub fn beech() -> Self {
        Species {
            name: "beech".into(),
            leader: 0.22,
            pipe: 2.5,
            angular: 0.0,
            up: 0.35,
            out: 0.15,
            crook: 0.12,
            kink: 0.0,
            inertia: 0.6,
            shell: 0.55,
            voids: 0.35,
            void_size: 0.25,
            scaffold: (2, 4),
            trunk: 0.065,
            smooth: 3,
            twigs: 3.0,
            twig_len: 1.2,
            twig_spread: 0.7,
            twig_zig: 0.3,
            twig_along: 0.4,
            clump: 0.028,
            squash: 0.5,
            hang: 0.15,
            fill: 0.95,
            ragged: 0.35,
            leafiness: 1.1,
            touch: 0.012,
            touch_w: 0.38,
            hook: 0.45,
            droop: 0.1,
            flat: 0.75,
            touches: 7.0,
            marcescent: 0.1,
            ..Self::oak()
        }
    }
    /// Small-leaved lime: a tall dense dome, many fine limbs, leaves to the
    /// shell.
    pub fn lime() -> Self {
        Species {
            name: "lime".into(),
            leader: 0.55,
            pipe: 2.4,
            angular: 0.5,
            density: 0.7,
            up: 0.3,
            out: 0.1,
            crook: 0.18,
            kink: 0.02,
            inertia: 0.5,
            shell: 0.75,
            voids: 0.2,
            void_size: 0.22,
            scaffold: (4, 7),
            trunk: 0.06,
            smooth: 2,
            twigs: 3.2,
            twig_len: 1.0,
            twig_spread: 0.9,
            twig_zig: 0.35,
            twig_along: 0.5,
            clump: 0.024,
            squash: 0.8,
            hang: 0.2,
            fill: 0.95,
            ragged: 0.4,
            leafiness: 1.25,
            touch: 0.011,
            touch_w: 0.45,
            hook: 0.6,
            droop: 0.3,
            flat: 0.2,
            touches: 7.0,
            marcescent: 0.0,
            ..Self::oak()
        }
    }
    /// Silver birch: a leading stem, thin rising limbs, long hanging twigs,
    /// small leaves the light goes through.
    pub fn birch() -> Self {
        Species {
            name: "birch".into(),
            leader: 0.6,
            pipe: 2.4,
            angular: 0.0,
            step: 0.022,
            density: 0.45,
            up: 0.45,
            out: 0.05,
            crook: 0.2,
            kink: 0.02,
            inertia: 0.55,
            shell: 0.3,
            voids: 0.45,
            void_size: 0.28,
            depth: 0.7,
            scaffold: (5, 9),
            trunk: 0.04,
            twig_w: 0.4,
            smooth: 2,
            twigs: 2.0,
            twig_len: 2.8,
            twig_spread: 0.5,
            twig_droop: 1.0,
            twig_zig: 0.15,
            twig_along: 0.9,
            clump: 0.017,
            squash: 1.3,
            hang: 0.6,
            fill: 0.55,
            ragged: 0.4,
            leafiness: 1.1,
            leafy_w: 2.6,
            touch: 0.009,
            touch_w: 0.5,
            hook: 0.4,
            droop: 0.8,
            flat: 0.0,
            touches: 6.0,
            marcescent: 0.0,
            ..Self::oak()
        }
    }
    /// A pollard willow: a short thick trunk ending in a knuckled head, and
    /// straight rods rising from it with narrow leaves in hanging tufts.
    pub fn willow() -> Self {
        Species {
            name: "willow".into(),
            leader: 0.0,
            pipe: 2.6,
            angular: 0.0,
            step: 0.026,
            density: 0.5,
            influence: 8.0,
            up: 0.9,
            out: 0.12,
            crook: 0.06,
            kink: 0.0,
            inertia: 0.8,
            shell: 0.35,
            voids: 0.3,
            void_size: 0.2,
            depth: 0.8,
            scaffold: (0, 0),
            trunk: 0.13,
            twig_w: 0.5,
            smooth: 3,
            twigs: 1.5,
            twig_len: 1.6,
            twig_spread: 0.25,
            twig_droop: 0.0,
            twig_zig: 0.05,
            twig_along: 0.3,
            clump: 0.022,
            squash: 1.4,
            hang: 0.45,
            fill: 0.7,
            ragged: 0.5,
            leafiness: 1.0,
            leafy_w: 3.0,
            touch: 0.016,
            touch_w: 0.3,
            hook: 0.35,
            droop: 0.55,
            flat: 0.0,
            touches: 6.0,
            marcescent: 0.0,
            ..Self::oak()
        }
    }
    pub fn named(name: &str) -> Option<Self> {
        Some(match name {
            "oak" => Self::oak(),
            "beech" => Self::beech(),
            "lime" | "linden" => Self::lime(),
            "birch" => Self::birch(),
            "willow" | "pollard" => Self::willow(),
            _ => return None,
        })
    }
}

/// The time of year, as what the leaves do.
#[derive(Clone, Copy, Debug)]
pub struct Season {
    /// Share of the summer's leaves on the tree (0 bare).
    pub leaf: f32,
    /// Size of the leaf clumps against summer's.
    pub size: f32,
    /// How far the leaves have turned (0 green .. 1 all turned).
    pub turn: f32,
    /// Winter: bare, but for the dead leaves some species keep.
    pub winter: bool,
}

impl Season {
    pub fn named(name: &str) -> Option<Self> {
        Some(match name {
            "summer" => Season { leaf: 1.0, size: 1.0, turn: 0.0, winter: false },
            "spring" => Season { leaf: 0.8, size: 0.62, turn: 0.0, winter: false },
            "autumn" | "fall" => Season { leaf: 0.62, size: 0.9, turn: 0.65, winter: false },
            "late_autumn" | "late_fall" => Season { leaf: 0.28, size: 0.85, turn: 1.0, winter: false },
            "winter" | "bare" => Season { leaf: 0.0, size: 0.9, turn: 1.0, winter: true },
            _ => return None,
        })
    }
}

/// One limb, twig or the trunk: from where it leaves its parent to its tip.
#[derive(Clone, Debug)]
pub struct Limb {
    pub pts: Vec<(f32, f32)>,
    /// Full width at each point.
    pub w: Vec<f32>,
    /// Depth at each point (units, + toward the viewer).
    pub z: Vec<f32>,
    /// 0 the trunk, 1 scaffold limbs, ...
    pub order: u32,
    pub parent: Option<usize>,
    /// A fine twig added past the model's resolution.
    pub twig: bool,
    /// The fine twig that leads on from the tip of its limb (the limb's own
    /// continuation, not a side twig).
    pub lead: bool,
}

/// A clump of leaves on a twig.
#[derive(Clone, Copy, Debug)]
pub struct Clump {
    pub at: (f32, f32),
    pub z: f32,
    /// Depth in the crown, -1 back .. 1 front.
    pub depth: f32,
    /// Half width; half height is `r * squash`.
    pub r: f32,
    pub squash: f32,
    pub tilt: f32,
    pub fill: f32,
    /// Light it catches (facing on the crown, less the shade).
    pub lit: f32,
    /// Shade cast on it by clumps between it and the sun.
    pub shade: f32,
    /// How far its leaves have turned (autumn), 0..1.
    pub turn: f32,
    /// Dead leaves kept in winter.
    pub dead: bool,
    pub limb: usize,
}

/// A hooked touch of a pointed brush: one leaf or a small bunch.
#[derive(Clone, Debug)]
pub struct Touch {
    pub pts: [(f32, f32); 3],
    pub w: f32,
    pub lit: f32,
    pub z: f32,
    pub turn: f32,
    pub dead: bool,
    pub clump: usize,
}

/// A grown broadleaved tree.
#[derive(Clone, Debug)]
pub struct Tree {
    pub species: String,
    /// Trunk first; parents before children.
    pub limbs: Vec<Limb>,
    /// Back to front.
    pub clumps: Vec<Clump>,
    /// Back to front.
    pub touches: Vec<Touch>,
    pub crown: Vec<(f32, f32)>,
    pub foot: (f32, f32),
    /// Where the trunk ends (the fork or pollard head).
    pub fork: (f32, f32),
    /// Crown height and the model's step (units).
    pub height: f32,
    pub step: f32,
    /// Mean clump radius and a good touch width (units).
    pub grain: f32,
    pub touch_w: f32,
    pub sun: V3,
    pub seed: u64,
    /// The width of the finest wood the model grows (units: the species'
    /// `twig_w` at this tree's size); wood under about twice this is fine
    /// wood (see `drawn`).
    pub twig_w: f32,
    /// The wood was grown angular (`Species::is_angular`): straight runs
    /// between nodes, which strokes keep (see `wood_strokes`).
    pub angular: bool,
    ragged: f32,
}

fn inside(poly: &[(f32, f32)], x: f32, y: f32) -> bool {
    let n = poly.len();
    let mut c = false;
    let mut j = n - 1;
    for i in 0..n {
        let (a, b) = (poly[i], poly[j]);
        if (a.1 > y) != (b.1 > y) && x < (b.0 - a.0) * (y - a.1) / (b.1 - a.1) + a.0 {
            c = !c;
        }
        j = i;
    }
    c
}

/// Distance from (x, y) to the polygon's edge.
fn edge_dist(poly: &[(f32, f32)], x: f32, y: f32) -> f32 {
    let n = poly.len();
    let mut best = f32::MAX;
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let t = (((x - a.0) * dx + (y - a.1) * dy) / (dx * dx + dy * dy).max(1e-9)).clamp(0.0, 1.0);
        let (px, py) = (a.0 + t * dx - x, a.1 + t * dy - y);
        best = best.min(px * px + py * py);
    }
    best.sqrt()
}

/// The horizontal extent of the polygon at height y (outermost crossings).
fn span(poly: &[(f32, f32)], y: f32) -> Option<(f32, f32)> {
    let n = poly.len();
    let (mut lo, mut hi) = (f32::MAX, f32::MIN);
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        if (a.1 <= y && b.1 > y) || (b.1 <= y && a.1 > y) {
            let x = a.0 + (y - a.1) / (b.1 - a.1) * (b.0 - a.0);
            lo = lo.min(x);
            hi = hi.max(x);
        }
    }
    (hi > lo).then_some((lo, hi))
}

/// Chaikin corner cutting (a few drawn points become a rounded crown).
fn soften(poly: &[(f32, f32)], passes: usize) -> Vec<(f32, f32)> {
    let mut p = poly.to_vec();
    for _ in 0..passes {
        let n = p.len();
        let mut q = Vec::with_capacity(2 * n);
        for i in 0..n {
            let (a, b) = (p[i], p[(i + 1) % n]);
            q.push((0.75 * a.0 + 0.25 * b.0, 0.75 * a.1 + 0.25 * b.1));
            q.push((0.25 * a.0 + 0.75 * b.0, 0.25 * a.1 + 0.75 * b.1));
        }
        p = q;
    }
    p
}

fn resample(line: &[(f32, f32)], step: f32) -> Vec<(f32, f32)> {
    let mut out = vec![line[0]];
    let mut carry = 0.0;
    for w in line.windows(2) {
        let (a, b) = (w[0], w[1]);
        let l = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
        let mut t = step - carry;
        while t <= l {
            out.push((lerp(a.0, b.0, t / l), lerp(a.1, b.1, t / l)));
            t += step;
        }
        carry = l - (t - step);
    }
    let last = *line.last().unwrap();
    let e = *out.last().unwrap();
    if ((last.0 - e.0).powi(2) + (last.1 - e.1).powi(2)).sqrt() > step * 0.3 {
        out.push(last);
    }
    out
}

/// Straighten each run of nodes between joints (the root, trunk nodes,
/// forks and tips): keep the nodes that stand off the run's chord by more
/// than `eps` (Douglas-Peucker), and put the others back on the straight
/// segments between the kept ones, at the same share of the length.
fn straighten(nodes: &mut [Node], eps: f32) {
    let n = nodes.len();
    let mut kids: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n {
        if nodes[i].parent != ROOT {
            kids[nodes[i].parent].push(i);
        }
    }
    let joint = |i: usize, nodes: &[Node]| nodes[i].parent == ROOT || nodes[i].trunk || kids[i].len() != 1;
    fn dp(p: &[V3], a: usize, b: usize, eps: f32, keep: &mut [bool]) {
        if b <= a + 1 {
            return;
        }
        let ab = sub(p[b], p[a]);
        let l2 = dot(ab, ab).max(1e-9);
        let (mut best, mut at) = (0.0, a);
        for (k, q) in p.iter().enumerate().take(b).skip(a + 1) {
            let t = (dot(sub(*q, p[a]), ab) / l2).clamp(0.0, 1.0);
            let dd = len(sub(*q, add(p[a], mul(ab, t))));
            if dd > best {
                best = dd;
                at = k;
            }
        }
        if best > eps {
            keep[at] = true;
            dp(p, a, at, eps, keep);
            dp(p, at, b, eps, keep);
        }
    }
    for j in 0..n {
        if !joint(j, nodes) {
            continue;
        }
        for &c in &kids[j] {
            let mut run = vec![j];
            let mut cur = c;
            loop {
                run.push(cur);
                if joint(cur, nodes) {
                    break;
                }
                cur = kids[cur][0];
            }
            if run.len() < 3 {
                continue;
            }
            let p: Vec<V3> = run.iter().map(|&i| nodes[i].p).collect();
            let mut keep = vec![false; p.len()];
            keep[0] = true;
            *keep.last_mut().unwrap() = true;
            dp(&p, 0, p.len() - 1, eps, &mut keep);
            let mut arc = vec![0.0f32; p.len()];
            for k in 1..p.len() {
                arc[k] = arc[k - 1] + len(sub(p[k], p[k - 1]));
            }
            let mut a = 0;
            for k in 1..p.len() {
                if !keep[k] {
                    continue;
                }
                for m in a + 1..k {
                    let t = ((arc[m] - arc[a]) / (arc[k] - arc[a]).max(1e-6)).clamp(0.0, 1.0);
                    nodes[run[m]].p = add(p[a], mul(sub(p[k], p[a]), t));
                }
                a = k;
            }
        }
    }
}

struct Node {
    p: V3,
    parent: usize,
    dir: V3,
    /// The shoot's heading: its direction over the last few steps.
    head: V3,
    bias: V3,
    grow: bool,
    kids: u8,
    trunk: bool,
}

const ROOT: usize = usize::MAX;

impl Tree {
    /// Grow a tree of `sp` into the closed `crown` (canvas units), from the
    /// `trunk` line (foot first; default: from under the crown up to a
    /// third of the way into it). `sun` points toward the sun (x right, y
    /// down, z toward the viewer).
    pub fn grow(crown: &[(f32, f32)], trunk: Option<&[(f32, f32)]>, sp: &Species, season: &Season, sun: V3, seed: u64) -> Tree {
        // grown in the crown's own frame (quantized), so the same crown
        // drawn elsewhere grows the same tree
        let ox = crown.iter().map(|p| p.0).fold(f32::MAX, f32::min).floor();
        let oy = crown.iter().map(|p| p.1).fold(f32::MAX, f32::min).floor();
        let q = |p: &(f32, f32)| (((p.0 - ox) * 64.0).round() / 64.0, ((p.1 - oy) * 64.0).round() / 64.0);
        let c: Vec<(f32, f32)> = crown.iter().map(q).collect();
        let t: Option<Vec<(f32, f32)>> = trunk.map(|t| t.iter().map(q).collect());
        let mut tree = Self::grow_local(&c, t.as_deref(), sp, season, sun, seed);
        tree.shift(ox, oy);
        tree
    }

    fn shift(&mut self, dx: f32, dy: f32) {
        let m = |p: &mut (f32, f32)| {
            p.0 += dx;
            p.1 += dy;
        };
        for l in &mut self.limbs {
            l.pts.iter_mut().for_each(m);
        }
        for c in &mut self.clumps {
            m(&mut c.at);
        }
        for t in &mut self.touches {
            t.pts.iter_mut().for_each(m);
        }
        self.crown.iter_mut().for_each(m);
        m(&mut self.foot);
        m(&mut self.fork);
    }

    fn grow_local(crown: &[(f32, f32)], trunk: Option<&[(f32, f32)]>, sp: &Species, season: &Season, sun: V3, seed: u64) -> Tree {
        let mut rng = Rng::new(seed ^ 0x00a4_72ee);
        let crown = if crown.len() < 12 { soften(crown, 3) } else { crown.to_vec() };
        let (x0, y0, x1, y1) = crown.iter().fold((f32::MAX, f32::MAX, f32::MIN, f32::MIN), |b, p| (b.0.min(p.0), b.1.min(p.1), b.2.max(p.0), b.3.max(p.1)));
        let hc = (y1 - y0).max(4.0);
        let wc = (x1 - x0).max(4.0);
        let cx = 0.5 * (x0 + x1);
        let d = (sp.step * hc).max(0.5);
        let scale = hc / 400.0;
        let twig_w = (sp.twig_w * scale.sqrt()).max(0.2);
        let pollard = sp.scaffold.1 == 0;

        // the trunk: drawn, or made up
        let trunk_line: Vec<(f32, f32)> = match trunk {
            Some(t) if t.len() >= 2 => t.to_vec(),
            Some(t) if t.len() == 1 => {
                let top = if pollard { (t[0].0, y1 - 0.02 * hc) } else { (t[0].0 + rng.normal() * 0.03 * wc, lerp(y1, y0, 0.3)) };
                vec![t[0], top]
            }
            _ => {
                let foot = (cx + rng.normal() * 0.03 * wc, y1 + if pollard { 0.5 * hc } else { 0.45 * hc });
                let top = if pollard { (foot.0, y1 - 0.02 * hc) } else { (cx + rng.normal() * 0.04 * wc, lerp(y1, y0, 0.3)) };
                vec![foot, top]
            }
        };
        // the drawn trunk goes on into the crown as a crooked leader
        let mut trunk_line = trunk_line;
        let want_top = y1 - sp.leader * hc;
        let top = *trunk_line.last().unwrap();
        if !pollard && top.1 > want_top + d {
            let (bx, by) = (cx + rng.normal() * 0.05 * wc, want_top);
            let k = ((top.1 - by) / (2.5 * d)).ceil().max(1.0) as usize;
            let prev = trunk_line[trunk_line.len().saturating_sub(2)];
            let lean = (top.0 - prev.0) / (prev.1 - top.1).abs().max(1.0);
            for i in 1..=k {
                let t = i as f32 / k as f32;
                let x = lerp(top.0 + lean * (top.1 - by) * t, bx, t * t) + rng.normal() * sp.crook * d * 0.8;
                trunk_line.push((x, lerp(top.1, by, t)));
            }
        }
        let foot = trunk_line[0];
        let tpts = resample(&trunk_line, d);
        let mut nodes: Vec<Node> = Vec::new();
        let crook = Fbm::new(seed as u32 ^ 0x7c0c, 2, 6.0);
        for (i, p) in tpts.iter().enumerate() {
            let wob = if i > 0 && i + 1 < tpts.len() { crook.get(i as f32, 0.3) * d * 0.25 * sp.crook } else { 0.0 };
            nodes.push(Node { p: [p.0 + wob, p.1, 0.0], parent: if i == 0 { ROOT } else { i - 1 }, dir: [0.0, -1.0, 0.0], head: [0.0, -1.0, 0.0], bias: [0.0; 3], grow: false, kids: 1, trunk: true });
        }
        let nt = nodes.len();
        nodes[nt - 1].kids = 0;
        nodes[nt - 1].grow = true;
        if nt >= 2 {
            let (a, b) = (nodes[nt - 2].p, nodes[nt - 1].p);
            nodes[nt - 1].dir = unit(sub(b, a));
        }
        // scaffold limbs leave the trunk inside the crown
        let fork_y = nodes[nt - 1].p[1];
        let in_crown: Vec<usize> = (0..nt - 1).filter(|&i| nodes[i].p[1] < y1 - 0.02 * hc && nodes[i].p[1] > fork_y).collect();
        if pollard {
            for n in &mut nodes[nt.saturating_sub(3)..nt] {
                n.grow = true;
            }
        } else if !in_crown.is_empty() {
            let k = rng.range(sp.scaffold.0 as f32, sp.scaffold.1 as f32 + 1.0) as usize;
            let mut pick: Vec<usize> = vec![];
            for _ in 0..k * 4 {
                if pick.len() >= k {
                    break;
                }
                let i = in_crown[(rng.f() * in_crown.len() as f32) as usize % in_crown.len()];
                if pick.iter().all(|&j| (i as i32 - j as i32).abs() >= 2) {
                    pick.push(i);
                }
            }
            for i in pick {
                nodes[i].grow = true;
            }
        }

        // attraction points in the crown, in depth, with gaps
        let voids = Fbm::new(seed as u32 ^ 0x6a95, 3, sp.void_size * wc);
        let area = {
            let mut a = 0.0;
            let n = crown.len();
            for i in 0..n {
                let (p, q) = (crown[i], crown[(i + 1) % n]);
                a += p.0 * q.1 - q.0 * p.1;
            }
            (0.5 * a).abs()
        };
        let want = ((area / (d * d)) * sp.density).clamp(60.0, 5000.0) as usize;
        let mut attr: Vec<V3> = Vec::with_capacity(want);
        let mut tries = 0;
        while attr.len() < want && tries < want * 40 {
            tries += 1;
            let (x, y) = (rng.range(x0, x1), rng.range(y0, y1));
            if !inside(&crown, x, y) {
                continue;
            }
            let Some((l, r)) = span(&crown, y) else { continue };
            let hw = 0.5 * (r - l);
            let mid = 0.5 * (l + r);
            let zmax = sp.depth * (hw * hw - (x - mid).powi(2)).max(0.0).sqrt();
            let e = edge_dist(&crown, x, y) / (0.5 * wc.min(hc));
            let zs = if rng.chance(sp.shell) {
                let s = rng.range(0.7, 1.0);
                // deep inside the silhouette, only the front and back shells
                if e > 0.5 && rng.chance(sp.shell) { continue; }
                if rng.chance(0.5) { s } else { -s }
            } else {
                rng.range(-1.0, 1.0)
            };
            let z = zs * zmax;
            let v = voids.get(x - x0 + 0.6 * z, y - y0 - 0.3 * z);
            if sp.voids > 0.0 && rng.f() < smoothstep(0.35 - sp.voids * 0.6, 0.35, -v) * sp.voids * 1.6 {
                continue;
            }
            attr.push([x, y, z]);
        }

        // space colonization
        let di = sp.influence * d;
        let dk = sp.kill * d;
        let cell = di;
        let key = |p: V3| ((p[0] / cell).floor() as i32, (p[1] / cell).floor() as i32);
        let mut grid: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
        for (i, n) in nodes.iter().enumerate() {
            grid.entry(key(n.p)).or_default().push(i);
        }
        let mut alive = vec![true; attr.len()];
        let max_iter = ((hc + wc) / d * 2.5) as usize + 40;
        let mut idle = 0;
        for _ in 0..max_iter {
            let mut pull: HashMap<usize, (V3, u32)> = HashMap::new();
            let mut any = false;
            for (ai, a) in attr.iter().enumerate() {
                if !alive[ai] {
                    continue;
                }
                any = true;
                let (kx, ky) = key(*a);
                let mut best = (f32::MAX, ROOT);
                for gx in kx - 1..=kx + 1 {
                    for gy in ky - 1..=ky + 1 {
                        if let Some(v) = grid.get(&(gx, gy)) {
                            for &ni in v {
                                let dd = len(sub(*a, nodes[ni].p));
                                if dd < best.0 {
                                    best = (dd, ni);
                                }
                            }
                        }
                    }
                }
                if best.1 == ROOT {
                    continue;
                }
                if best.0 < dk {
                    alive[ai] = false;
                    continue;
                }
                if best.0 < di && nodes[best.1].grow {
                    let e = pull.entry(best.1).or_insert(([0.0; 3], 0));
                    e.0 = add(e.0, unit(sub(*a, nodes[best.1].p)));
                    e.1 += 1;
                }
            }
            if !any {
                break;
            }
            if pull.is_empty() {
                // nothing in reach: the leader grows on toward the crown
                idle += 1;
                if idle > 60 {
                    break;
                }
                let live: Vec<V3> = attr.iter().zip(&alive).filter(|(_, a)| **a).map(|(p, _)| *p).collect();
                let tip = (0..nodes.len()).rev().find(|&i| nodes[i].grow).unwrap_or(nodes.len() - 1);
                let near = live.iter().min_by(|a, b| len(sub(**a, nodes[tip].p)).total_cmp(&len(sub(**b, nodes[tip].p)))).copied();
                if let Some(t) = near {
                    let dir = unit(sub(t, nodes[tip].p));
                    let p = add(nodes[tip].p, mul(dir, d));
                    nodes[tip].kids += 1;
                    let i = nodes.len();
                    nodes.push(Node { p, parent: tip, dir, head: dir, bias: [0.0; 3], grow: true, kids: 0, trunk: false });
                    grid.entry(key(p)).or_default().push(i);
                }
                continue;
            }
            idle = 0;
            let mut ks: Vec<usize> = pull.keys().copied().collect();
            ks.sort_unstable();
            for ni in ks {
                let (s, _) = pull[&ni];
                let n = &nodes[ni];
                if n.kids >= if n.trunk { 2 } else { 3 } {
                    continue;
                }
                let mut dir = unit(s);
                let outv = unit([n.p[0] - cx, 0.0, n.p[2] * 0.6]);
                let r3 = unit([rng.normal(), rng.normal(), rng.normal()]);
                dir = add(dir, mul(n.dir, sp.inertia));
                dir = add(dir, [0.0, -sp.up, 0.0]);
                dir = add(dir, mul(outv, sp.out));
                dir = add(dir, mul(r3, sp.crook));
                let mut bias = mul(n.bias, 0.85);
                if rng.chance(sp.kink) {
                    // a sympodial kink: the shoot's bud aborts, a side bud takes over
                    bias = mul(unit([rng.normal(), rng.normal() * 0.6, rng.normal()]), 0.9);
                }
                dir = unit(add(dir, bias));
                // no shoot turns more than about 55 degrees in one step (no curls)
                if n.parent != ROOT && dot(dir, n.dir) < 0.57 {
                    dir = unit(add(dir, mul(n.dir, 0.9)));
                }
                // an angular shoot zigzags about its heading but doesn't
                // curl back on itself: no step more than about 70 degrees
                // off the way it has been going (a side shoot sets out anew)
                let lateral = n.kids > 0;
                if sp.is_angular() && !lateral && n.parent != ROOT && !n.trunk {
                    for _ in 0..4 {
                        if dot(dir, n.head) >= 0.34 {
                            break;
                        }
                        dir = unit(add(dir, mul(n.head, 0.6)));
                    }
                }
                let head = if lateral || n.trunk { dir } else { unit(add(mul(n.head, 0.7), mul(dir, 0.3))) };
                // no growing back into the trunk's foot or through the ground
                if dir[1] > 0.6 && !pollard {
                    dir[1] = 0.6;
                    dir = unit(dir);
                }
                let p = add(n.p, mul(dir, d));
                // skip a step that lands on a sibling
                let (kx, ky) = key(p);
                let dup = grid.get(&(kx, ky)).is_some_and(|v| v.iter().any(|&j| len(sub(nodes[j].p, p)) < 0.35 * d));
                if dup {
                    continue;
                }
                nodes[ni].kids += 1;
                let i = nodes.len();
                nodes.push(Node { p, parent: ni, dir, head, bias, grow: true, kids: 0, trunk: false });
                grid.entry(key(p)).or_default().push(i);
            }
        }

        // smooth the wood (not the trunk's foot, not forks)
        for _ in 0..sp.smooth {
            let n = nodes.len();
            let mut child = vec![ROOT; n];
            let mut count = vec![0u8; n];
            for (i, nd) in nodes.iter().enumerate() {
                if nd.parent != ROOT {
                    child[nd.parent] = i;
                    count[nd.parent] += 1;
                }
            }
            let old: Vec<V3> = nodes.iter().map(|n| n.p).collect();
            for i in 0..n {
                if nodes[i].parent == ROOT || count[i] != 1 || nodes[i].trunk {
                    continue;
                }
                let m = mul(add(old[nodes[i].parent], old[child[i]]), 0.5);
                nodes[i].p = add(mul(old[i], 0.5), mul(m, 0.5));
            }
        }

        // an angular wood (oak): the runs between forks straightened, the
        // turns kept at a few nodes (Douglas-Peucker on each run)
        if sp.is_angular() {
            straighten(&mut nodes, sp.angular * 0.5 * d);
        }

        // widths: the pipe model from equal twigs down, remapped so the
        // trunk has the species' width
        let n = nodes.len();
        let pexp = sp.pipe;
        let mut wp = vec![0.0f32; n];
        for i in (0..n).rev() {
            if wp[i] == 0.0 {
                wp[i] = twig_w.powf(pexp);
            }
            let p = nodes[i].parent;
            if p != ROOT {
                wp[p] += wp[i];
            }
        }
        let wraw: Vec<f32> = wp.iter().map(|v| v.powf(1.0 / pexp)).collect();
        let target = (sp.trunk * hc).max(twig_w * 2.0);
        let g = if wraw[0] > twig_w * 1.01 { (target / twig_w).ln() / (wraw[0] / twig_w).ln() } else { 1.0 };
        let mut width: Vec<f32> = wraw.iter().map(|&w| twig_w * (w / twig_w).max(1.0).powf(g)).collect();
        // the trunk tapers up from a flared foot
        let tlen = nodes[nt - 1].p[1] - nodes[0].p[1];
        for (i, w) in width.iter_mut().enumerate().take(nt) {
            let t = ((nodes[i].p[1] - nodes[0].p[1]) / tlen.min(-1.0)).clamp(0.0, 1.0);
            let flare = 1.0 + 0.55 * (-t * 16.0).exp();
            let floor = if nodes[i].p[1] > y1 - 0.05 * hc { target * (1.0 - 0.3 * t) } else { 0.0 };
            *w = w.max(floor) * flare;
        }

        // limbs: follow the thickest child through each fork
        let mut kids: Vec<Vec<usize>> = vec![vec![]; n];
        for i in 1..n {
            if nodes[i].parent != ROOT {
                kids[nodes[i].parent].push(i);
            }
        }
        let mut limbs: Vec<Limb> = Vec::new();
        let mut limb_of = vec![0usize; n];
        let mut stack: Vec<(usize, Option<usize>, u32)> = vec![(0, None, 0)];
        while let Some((start, parent, order)) = stack.pop() {
            let li = limbs.len();
            let mut pts = vec![];
            let mut w = vec![];
            let mut z = vec![];
            if nodes[start].parent != ROOT {
                let q = nodes[start].parent;
                pts.push((nodes[q].p[0], nodes[q].p[1]));
                w.push(width[start].min(width[q]));
                z.push(nodes[q].p[2]);
            }
            let mut cur = start;
            loop {
                limb_of[cur] = li;
                pts.push((nodes[cur].p[0], nodes[cur].p[1]));
                w.push(width[cur]);
                z.push(nodes[cur].p[2]);
                let ks = &kids[cur];
                if ks.is_empty() {
                    break;
                }
                let main = *ks.iter().max_by(|a, b| width[**a].total_cmp(&width[**b])).unwrap();
                for &k in ks {
                    if k != main {
                        stack.push((k, Some(li), order + 1));
                    }
                }
                cur = main;
            }
            limbs.push(Limb { pts, w, z, order, parent, twig: false, lead: false });
        }

        // fine twigs at the tips, and hanging along thin wood
        let nl = limbs.len();
        for li in 0..nl {
            let l = limbs[li].clone();
            let m = l.pts.len();
            if m < 2 || l.order == 0 {
                continue;
            }
            let sprout = |rng: &mut Rng, at: usize, k: f32, side: f32, lead: bool, limbs: &mut Vec<Limb>| {
                let p = l.pts[at];
                let q = l.pts[at.saturating_sub(1).min(m - 2)];
                let q2 = l.pts[(at.max(1)).min(m - 1)];
                let base = if at == 0 { (q2.0 - p.0, q2.1 - p.1) } else { (p.0 - q.0, p.1 - q.1) };
                let bl = (base.0 * base.0 + base.1 * base.1).sqrt().max(1e-6);
                // off to one side of the limb (a fishbone, not a starburst), or on along it
                // an angular twig stands well off its limb, not along it
                let lo = if sp.is_angular() { 0.7 } else { 0.45 };
                let mut a = base.1.atan2(base.0) + if side == 0.0 { rng.normal() * 0.25 * sp.twig_spread } else { side * sp.twig_spread * rng.range(lo, 1.0) + rng.normal() * 0.12 };
                let segs = 3;
                let tl = sp.twig_len * d * rng.range(0.6, 1.3) * k;
                let mut pts = vec![p];
                let mut z = vec![l.z[at]];
                let mut cur = p;
                for s in 0..segs {
                    let mut sa = if s % 2 == 0 { 1.0 } else { -1.0 };
                    // an angular twig's zigzag starts away from its limb
                    if sp.is_angular() && side != 0.0 {
                        sa *= side;
                    }
                    a += sp.twig_zig * sa * rng.range(0.3, 0.8);
                    let (mut dx, mut dy) = (a.cos(), a.sin());
                    // pendulous: bend toward hanging
                    let hang = sp.twig_droop * (s as f32 + 1.0) / segs as f32;
                    dx *= 1.0 - 0.8 * hang;
                    dy = lerp(dy, 1.0, hang);
                    let dl = (dx * dx + dy * dy).sqrt().max(1e-6);
                    cur = (cur.0 + dx / dl * tl / segs as f32, cur.1 + dy / dl * tl / segs as f32);
                    pts.push(cur);
                    z.push(l.z[at] + rng.normal() * tl * 0.2);
                }
                let _ = bl;
                let w0 = (l.w[at] * 0.6).min(twig_w * 0.9).max(0.15);
                let w: Vec<f32> = (0..pts.len()).map(|i| (w0 * (1.0 - 0.6 * i as f32 / segs as f32)).max(0.12)).collect();
                limbs.push(Limb { pts, w, z, order: l.order + 1, parent: Some(li), twig: true, lead });
            };
            // at the tip
            let k = (sp.twigs + rng.f()) as usize;
            for j in 0..k {
                let at = if j == 0 { m - 1 } else { (m - 1).saturating_sub(j).max(1) };
                let side = if j == 0 { 0.0 } else if j % 2 == 1 { 1.0 } else { -1.0 };
                sprout(&mut rng, at, if j == 0 { 1.0 } else { 0.85 }, side, j == 0, &mut limbs);
            }
            // along the thin wood
            for at in 1..m - 1 {
                if l.w[at] < twig_w * sp.leafy_w * 1.6 && rng.chance(sp.twig_along) {
                    let side = if rng.chance(0.5) { 1.0 } else { -1.0 };
                    sprout(&mut rng, at, 0.8, side, false, &mut limbs);
                }
            }
        }

        // leaves
        let r0 = sp.clump * hc * season.size;
        let mut clumps: Vec<Clump> = Vec::new();
        let keep = Fbm::new(seed as u32 ^ 0x1eaf, 2, 0.12 * wc);
        let zspan = (sp.depth * 0.5 * wc).max(1.0);
        let leafy = |w: f32| w < twig_w * sp.leafy_w;
        if r0 > 0.2 {
            for (li, l) in limbs.iter().enumerate() {
                if l.order == 0 {
                    continue;
                }
                let m = l.pts.len();
                let step = d / sp.leafiness.max(0.05);
                let mut acc = rng.range(0.0, step);
                for i in 1..m {
                    if !leafy(l.w[i]) {
                        continue;
                    }
                    let (a, b) = (l.pts[i - 1], l.pts[i]);
                    let sl = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
                    let mut t = acc;
                    while t < sl {
                        let f = t / sl.max(1e-6);
                        let (x, y) = (lerp(a.0, b.0, f), lerp(a.1, b.1, f));
                        let zz = lerp(l.z[i - 1], l.z[i], f);
                        t += step * rng.range(0.7, 1.3);
                        let upper = smoothstep(y1, y0, y);
                        let dead_keep = season.winter && rng.chance(sp.marcescent) && upper < 0.65;
                        if season.winter && !dead_keep {
                            continue;
                        }
                        if !season.winter {
                            // leaves fall in patches (whole twigs bare), not evenly
                            let k = 0.5 + 0.4 * keep.get(x - x0, y - y0) + 0.3 * (rng.f() - 0.5);
                            if season.leaf < 1.0 && k > season.leaf {
                                continue;
                            }
                        }
                        let r = r0 * rng.range(0.7, 1.25) * if dead_keep { 0.6 } else { 1.0 };
                        let (dx, dy) = ((b.0 - a.0) / sl.max(1e-6), (b.1 - a.1) / sl.max(1e-6));
                        let off = rng.normal() * 0.4 * r;
                        let cx2 = x - dy * off;
                        let cy2 = y + dx * off + sp.hang * r * sp.squash;
                        let along = dy.atan2(dx);
                        let along = if along > PI / 2.0 { along - PI } else if along < -PI / 2.0 { along + PI } else { along };
                        let tilt = if sp.squash < 1.0 { along * 0.4 } else { 0.0 } + rng.normal() * 0.15;
                        let outer = (edge_dist(&crown, cx2, cy2) / (0.25 * wc)).min(1.0);
                        let turn = if dead_keep { 1.0 } else { (season.turn * (0.55 + 0.45 * (1.0 - outer) + 0.35 * keep.get(y - y0, x - x0))).clamp(0.0, 1.0) };
                        clumps.push(Clump {
                            at: (cx2, cy2),
                            z: zz + rng.normal() * 0.3 * r,
                            depth: (zz / zspan).clamp(-1.0, 1.0),
                            r,
                            squash: sp.squash * rng.range(0.85, 1.15),
                            tilt,
                            fill: (sp.fill * rng.range(0.85, 1.1) * if dead_keep { 0.6 } else { lerp(0.75, 1.0, season.leaf) }).min(1.0),
                            lit: 0.0,
                            shade: 0.0,
                            turn,
                            dead: dead_keep,
                            limb: li,
                        });
                    }
                    acc = t - sl;
                }
            }
        }
        clumps.sort_by(|a, b| a.z.total_cmp(&b.z));
        let sun = unit(sun);
        light(&mut clumps, sun, (cx, 0.5 * (y0 + y1)), [0.5 * wc, 0.5 * hc, zspan]);

        let grain = if clumps.is_empty() { r0.max(1.0) } else { clumps.iter().map(|c| c.r).sum::<f32>() / clumps.len() as f32 };
        let tw = (sp.touch * hc * season.size.sqrt() * sp.touch_w).max(0.35);
        let mut tree = Tree {
            species: sp.name.clone(),
            limbs,
            clumps,
            touches: vec![],
            crown,
            foot,
            fork: (nodes[nt - 1].p[0], nodes[nt - 1].p[1]),
            height: hc,
            step: d,
            grain,
            touch_w: tw,
            sun,
            seed,
            twig_w,
            angular: sp.is_angular(),
            ragged: sp.ragged,
        };
        tree.touches = tree.lay_touches(sp, season, &mut rng);
        tree
    }

    fn lay_touches(&self, sp: &Species, season: &Season, rng: &mut Rng) -> Vec<Touch> {
        let mut out = Vec::new();
        let tl = sp.touch * self.height * season.size.sqrt();
        let s = self.sun;
        for (ci, c) in self.clumps.iter().enumerate() {
            let n = ((sp.touches * c.fill * (c.r / self.grain.max(0.1)).powi(2)) + rng.f()) as usize;
            let (ct, st) = (c.tilt.cos(), c.tilt.sin());
            for _ in 0..n {
                // a point in the clump, more of them toward its rim
                let a = rng.range(0.0, TAU);
                let rr = rng.f().powf(0.35);
                let (u, v) = (a.cos() * rr, a.sin() * rr);
                let (ox, oy) = (u * c.r, v * c.r * c.squash);
                let p = (c.at.0 + ox * ct - oy * st, c.at.1 + ox * st + oy * ct);
                // how the leaf lies: outward from the clump, hanging, or level
                let outw = (u, v);
                let mut dx = outw.0 * (1.0 - sp.droop - sp.flat).max(0.0) + rng.normal() * 0.45;
                let mut dy = outw.1 * (1.0 - sp.droop - sp.flat).max(0.0) + sp.droop + rng.normal() * 0.3 * (1.0 - sp.flat);
                if sp.flat > 0.0 {
                    dx += sp.flat * if u >= 0.0 { 1.0 } else { -1.0 };
                    dy *= 1.0 - 0.6 * sp.flat;
                }
                let dl = (dx * dx + dy * dy).sqrt().max(1e-6);
                (dx, dy) = (dx / dl, dy / dl);
                let l = tl * rng.range(0.7, 1.3);
                let p1 = (p.0 + dx * l * 0.6, p.1 + dy * l * 0.6);
                let h = sp.hook * rng.range(0.4, 1.2) * if rng.chance(0.5) { 1.0 } else { -1.0 };
                let (hc, hs) = (h.cos(), h.sin());
                let (ex, ey) = (dx * hc - dy * hs, dx * hs + dy * hc);
                let p2 = (p1.0 + ex * l * 0.4, p1.1 + ey * l * 0.4);
                let w3 = (1.0 - u * u - v * v).max(0.0).sqrt();
                let local = (u * s[0] + v * s[1] + w3 * s[2]).clamp(-1.0, 1.0);
                let lit = (c.lit * (0.8 + 0.3 * local) + rng.normal() * 0.06).clamp(0.0, 1.0);
                out.push(Touch { pts: [p, p1, p2], w: self_touch_w(sp, self.height, season), lit, z: c.z + w3 * c.r * 0.5, turn: c.turn, dead: c.dead, clump: ci });
            }
        }
        out.sort_by(|a, b| a.z.total_cmp(&b.z));
        out
    }

    /// Coverage (0..1) of clump k at (x, y), before its fill; and (u, v) in radii.
    fn cover(&self, k: usize, x: f32, y: f32) -> (f32, f32, f32) {
        let c = &self.clumps[k];
        let (dx, dy) = (x - c.at.0, y - c.at.1);
        let (ct, st) = (c.tilt.cos(), c.tilt.sin());
        let u = (dx * ct + dy * st) / c.r;
        let v = (-dx * st + dy * ct) / (c.r * c.squash);
        let d = (u * u + v * v).sqrt();
        let th = v.atan2(u);
        let h = crate::rng::hash2(k as i64, 17, self.seed);
        let h2 = crate::rng::hash2(k as i64, 29, self.seed);
        let lobe = 1.0 + self.ragged * 0.35 * (0.6 * (3.0 * th + h * TAU).sin() + 0.4 * (5.0 * th + h2 * TAU).sin());
        (1.0 - smoothstep(lobe * 0.55, lobe * 1.05, d), u, v)
    }

    fn px_box(&self, f: &Frame, k: usize) -> Option<(usize, usize, usize, usize)> {
        let c = &self.clumps[k];
        let rr = c.r * c.squash.max(1.0) * 1.4;
        let x0 = ((c.at.0 - rr) * f.scale).floor() as isize - f.x0 as isize;
        let x1 = ((c.at.0 + rr) * f.scale).ceil() as isize - f.x0 as isize;
        let y0 = ((c.at.1 - rr) * f.scale).floor() as isize - f.y0 as isize;
        let y1 = ((c.at.1 + rr) * f.scale).ceil() as isize - f.y0 as isize;
        let (x0, y0) = (x0.max(0), y0.max(0));
        let (x1, y1) = (x1.min(f.w as isize - 1), y1.min(f.h as isize - 1));
        (x1 >= x0 && y1 >= y0).then_some((x0 as usize, y0 as usize, x1 as usize, y1 as usize))
    }

    /// Where the leaves are, of the clumps `pick` takes: 1 on leaves, 0 on
    /// sky, the edges broken at leaf size.
    pub fn leaves_where(&self, f: Frame, pick: impl Fn(&Clump) -> bool) -> Mask {
        let mut cov = vec![0.0f32; f.w * f.h];
        for k in 0..self.clumps.len() {
            if !pick(&self.clumps[k]) {
                continue;
            }
            let Some((x0, y0, x1, y1)) = self.px_box(&f, k) else { continue };
            let fill = self.clumps[k].fill;
            for py in y0..=y1 {
                let y = (py + f.y0) as f32 / f.scale + 0.5 / f.scale;
                for px in x0..=x1 {
                    let x = (px + f.x0) as f32 / f.scale + 0.5 / f.scale;
                    let (c, _, _) = self.cover(k, x, y);
                    if c > 0.0 {
                        let i = py * f.w + px;
                        cov[i] = 1.0 - (1.0 - cov[i]) * (1.0 - c * fill);
                    }
                }
            }
        }
        let leaf = Fbm::new(self.seed as u32 ^ 0x1eaf, 3, (self.grain * 0.4).max(0.5));
        let inv = 1.0 / f.scale;
        let data = cov
            .par_iter()
            .enumerate()
            .map(|(i, &c)| {
                if c <= 0.0 {
                    return 0.0;
                }
                let (x, y) = (((i % f.w) + f.x0) as f32 * inv, ((i / f.w) + f.y0) as f32 * inv);
                smoothstep(0.42, 0.58, c + 0.35 * leaf.get(x, y))
            })
            .collect();
        Mask { f, data }
    }

    pub fn leaves(&self, f: Frame) -> Mask {
        self.leaves_where(f, |_| true)
    }

    /// Light on the leaves (0..1, 0 off them): the front clump over the ones
    /// behind, each rounded like a small ball of leaves.
    pub fn light(&self, f: Frame) -> Mask {
        let mut val = vec![0.0f32; f.w * f.h];
        let mut cov = vec![0.0f32; f.w * f.h];
        let s = self.sun;
        for k in 0..self.clumps.len() {
            let Some((x0, y0, x1, y1)) = self.px_box(&f, k) else { continue };
            let (lit, fill) = (self.clumps[k].lit, self.clumps[k].fill);
            for py in y0..=y1 {
                let y = (py + f.y0) as f32 / f.scale + 0.5 / f.scale;
                for px in x0..=x1 {
                    let x = (px + f.x0) as f32 / f.scale + 0.5 / f.scale;
                    let (c, u, v) = self.cover(k, x, y);
                    let a = fill * smoothstep(0.4, 0.6, c);
                    if a > 0.0 {
                        let w = (1.0 - u * u - v * v).max(0.0).sqrt();
                        let local = (u * s[0] + v * s[1] + w * s[2]).clamp(-1.0, 1.0);
                        let i = py * f.w + px;
                        val[i] = val[i] * (1.0 - a) + a * (lit * (0.8 + 0.3 * local)).clamp(0.0, 1.0);
                        cov[i] = cov[i] * (1.0 - a) + a;
                    }
                }
            }
        }
        for (v, c) in val.iter_mut().zip(&cov) {
            if *c > 0.0 {
                *v /= *c;
            }
        }
        Mask { f, data: val }.mul(&self.leaves(f))
    }

    fn wood_shape(&self, mut shape: Shape, pick: impl Fn(&Limb) -> bool) -> Shape {
        for l in self.limbs.iter().filter(|l| pick(l)) {
            if l.pts.len() >= 2 {
                shape = shape.ribbon(&l.pts, &l.w);
            }
        }
        shape
    }

    /// The wood where it is between `lo` and `hi` wide: runs of each limb,
    /// so `wood(6, inf)` is the trunk and the stout lower limbs, not their
    /// thin ends.
    pub fn wood(&self, f: Frame, lo: f32, hi: f32) -> Mask {
        let mut shape = Shape::new();
        for l in &self.limbs {
            let n = l.pts.len();
            let mut i = 0;
            while i < n {
                if !(l.w[i] >= lo && l.w[i] < hi) {
                    i += 1;
                    continue;
                }
                let s = i;
                while i < n && l.w[i] >= lo && l.w[i] < hi {
                    i += 1;
                }
                // one point past the run on each side keeps the joins
                let (a, b) = (s.saturating_sub(1), (i + 1).min(n));
                if b - a >= 2 {
                    shape = shape.ribbon(&l.pts[a..b], &l.w[a..b]);
                }
            }
        }
        Mask::from_shape(f, shape)
    }

    pub fn trunk(&self, f: Frame) -> Mask {
        Mask::from_shape(f, self.wood_shape(Shape::new(), |l| l.order == 0))
    }

    /// Leaves and wood.
    pub fn mask(&self, f: Frame) -> Mask {
        self.leaves(f).union(&self.wood(f, 0.0, f32::MAX))
    }

    pub fn crown_mask(&self, f: Frame) -> Mask {
        Mask::from_shape(f, Shape::new().poly(&self.crown))
    }

    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        let mut b = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for l in &self.limbs {
            for p in &l.pts {
                b = (b.0.min(p.0), b.1.min(p.1), b.2.max(p.0), b.3.max(p.1));
            }
        }
        for c in &self.clumps {
            let r = c.r * 1.3;
            b = (b.0.min(c.at.0 - r), b.1.min(c.at.1 - r), b.2.max(c.at.0 + r), b.3.max(c.at.1 + r));
        }
        b
    }
}

fn self_touch_w(sp: &Species, hc: f32, season: &Season) -> f32 {
    (sp.touch * hc * season.size.sqrt() * sp.touch_w).max(0.35)
}

/// Light on each clump: its facing on the crown (the sunward side lit, the
/// far side and inside dark), less the shade of the clumps between it and
/// the sun (found on a grid across the sun's direction).
fn light(clumps: &mut [Clump], s: V3, c: (f32, f32), half: V3) {
    let n = clumps.len();
    if n == 0 {
        return;
    }
    // a basis across the sun
    let a = if s[1].abs() < 0.9 { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] };
    let u = unit([a[1] * s[2] - a[2] * s[1], a[2] * s[0] - a[0] * s[2], a[0] * s[1] - a[1] * s[0]]);
    let v = [s[1] * u[2] - s[2] * u[1], s[2] * u[0] - s[0] * u[2], s[0] * u[1] - s[1] * u[0]];
    let rmean = clumps.iter().map(|c| c.r).sum::<f32>() / n as f32;
    let cell = rmean * 1.5;
    let pos: Vec<V3> = clumps.iter().map(|c| [c.at.0, c.at.1, c.z]).collect();
    let mut grid: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
    for (i, p) in pos.iter().enumerate() {
        grid.entry(((dot(*p, u) / cell).floor() as i32, (dot(*p, v) / cell).floor() as i32)).or_default().push(i);
    }
    for i in 0..n {
        let p = pos[i];
        let (ku, kv) = ((dot(p, u) / cell).floor() as i32, (dot(p, v) / cell).floor() as i32);
        let mut through = 1.0f32;
        for gu in ku - 1..=ku + 1 {
            for gv in kv - 1..=kv + 1 {
                let Some(list) = grid.get(&(gu, gv)) else { continue };
                for &j in list {
                    if j == i {
                        continue;
                    }
                    let dd = sub(pos[j], p);
                    let t = dot(dd, s);
                    if t <= 0.6 * clumps[i].r {
                        continue;
                    }
                    let perp2 = dot(dd, dd) - t * t;
                    let rj = clumps[j].r;
                    if perp2 < rj * rj {
                        through *= 1.0 - 0.3 * clumps[j].fill;
                    }
                }
            }
        }
        let q = [(p[0] - c.0) / half[0], (p[1] - c.1) / half[1], p[2] / half[2]];
        let m = len(q).max(1e-6);
        let facing = 0.5 + 0.5 * dot(q, s) / m;
        let outer = smoothstep(0.2, 0.9, m);
        let shade = 1.0 - through;
        clumps[i].shade = shade;
        clumps[i].lit = (facing.powf(1.2) * (0.45 + 0.55 * outer) * (1.0 - 0.75 * shade)).clamp(0.0, 1.0);
    }
}

// ------------------------------------------------------------ field trees

/// A group of field trees in depth, grown from a few drawn crowns: the
/// drawn ones stand in front; others, made from them (flipped, stretched,
/// reshaped), stand behind, smaller, higher toward the horizon and hazier.
#[derive(Clone, Debug)]
pub struct Group {
    /// Back to front.
    pub trees: Vec<Tree>,
    /// Per tree: apparent size relative to the drawn ones, and air.
    pub scale: Vec<f32>,
    pub haze: Vec<f32>,
    pub horizon: f32,
}

#[derive(Clone, Debug)]
pub struct GroupSpec {
    /// Trees made behind the drawn ones.
    pub extra: usize,
    pub horizon: Option<f32>,
    /// How far back the made trees stand (1 + u * recede).
    pub recede: f32,
    /// Air per unit of distance (haze = 1 - exp(-(d - 1) * air)).
    pub air: f32,
    /// How far sideways the made trees spread, as a share of the drawn span.
    pub spread: f32,
    /// Share of the made trees drawn tall and narrow.
    pub narrow: f32,
    pub sun: V3,
}

impl Default for GroupSpec {
    fn default() -> Self {
        GroupSpec { extra: 3, horizon: None, recede: 1.5, air: 0.5, spread: 0.5, narrow: 0.3, sun: [-0.55, -0.75, 0.35] }
    }
}

impl Group {
    /// `crowns[i]` with `trunks[i]` (optional) and `species[i]`: the drawn
    /// trees; `foot` y where the drawn trees stand if no trunk is given.
    pub fn grow(crowns: &[Vec<(f32, f32)>], trunks: &[Option<Vec<(f32, f32)>>], species: &[Species], season: &Season, foot: Option<f32>, spec: &GroupSpec, seed: u64) -> Group {
        let mut rng = Rng::new(seed ^ 0x9e0_f1e1d);
        let mut trees: Vec<(f32, Tree, f32, f32)> = Vec::new();
        let mut feet = vec![];
        for (i, c) in crowns.iter().enumerate() {
            let sp = &species[i % species.len()];
            let y1 = c.iter().map(|p| p.1).fold(f32::MIN, f32::max);
            let y0 = c.iter().map(|p| p.1).fold(f32::MAX, f32::min);
            let cx = c.iter().map(|p| p.0).sum::<f32>() / c.len() as f32;
            let tr = trunks.get(i).cloned().flatten().or_else(|| foot.map(|fy| vec![(cx, fy)]));
            let t = Tree::grow(c, tr.as_deref(), sp, season, spec.sun, seed.wrapping_mul(131).wrapping_add(i as u64));
            feet.push((t.foot, y1 - y0, c.clone(), sp.clone()));
            trees.push((1.0, t, 1.0, 0.0));
        }
        let fy = feet.iter().map(|f| f.0.1).sum::<f32>() / feet.len().max(1) as f32;
        let mean_h = feet.iter().map(|f| f.1).sum::<f32>() / feet.len().max(1) as f32;
        let horizon = spec.horizon.unwrap_or(fy - 0.6 * mean_h);
        let xs0 = feet.iter().map(|f| f.0.0).fold(f32::MAX, f32::min);
        let xs1 = feet.iter().map(|f| f.0.0).fold(f32::MIN, f32::max);
        let spanx = (xs1 - xs0).max(mean_h);
        for k in 0..spec.extra {
            let (ft, _, c, sp) = &feet[rng.f().mul_add(feet.len() as f32, 0.0) as usize % feet.len()];
            let dist = 1.0 + rng.range(0.15, 1.0) * spec.recede;
            let s = 1.0 / dist;
            let x = lerp(xs0 - spec.spread * spanx, xs1 + spec.spread * spanx, rng.f());
            let nfy = horizon + (fy - horizon) * s;
            let flip = if rng.chance(0.5) { -1.0 } else { 1.0 };
            let narrow = rng.chance(spec.narrow);
            let sx = s * if narrow { rng.range(0.45, 0.65) } else { rng.range(0.8, 1.2) };
            let sy = s * if narrow { rng.range(1.05, 1.3) } else { rng.range(0.85, 1.1) };
            let wob = Fbm::new((seed as u32).wrapping_add(k as u32 * 977), 2, 0.9);
            let n = c.len();
            let cc: Vec<(f32, f32)> = c
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let a = i as f32 / n as f32 * TAU;
                    let r = 1.0 + 0.12 * wob.get(a.cos(), a.sin());
                    (x + flip * (p.0 - ft.0) * sx * r, nfy + (p.1 - ft.1) * sy * r)
                })
                .collect();
            let t = Tree::grow(&cc, Some(&[(x, nfy)]), sp, season, spec.sun, seed.wrapping_mul(977).wrapping_add(k as u64 + 100));
            let haze = 1.0 - (-(dist - 1.0) * spec.air).exp();
            trees.push((dist, t, s, haze));
        }
        trees.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.foot.1.total_cmp(&b.1.foot.1)));
        Group { scale: trees.iter().map(|t| t.2).collect(), haze: trees.iter().map(|t| t.3).collect(), trees: trees.into_iter().map(|t| t.1).collect(), horizon }
    }

    /// One faint, uneven shadow on the ground per tree, thrown away from the sun.
    pub fn shadow(&self, f: Frame) -> Mask {
        let mut shape = Shape::new();
        for t in &self.trees {
            let (x0, _, x1, _) = t.bounds();
            let w = (x1 - x0) * 0.55;
            let dx = -self.trees[0].sun[0].signum() * w * 0.45;
            let h = (w * 0.12).max(1.0);
            shape = shape.ellipse(t.foot.0 + dx, t.foot.1 + h * 0.3, w, h);
        }
        Mask::from_shape(f, shape)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(super) fn crown() -> Vec<(f32, f32)> {
        let mut v = vec![];
        for i in 0..32 {
            let a = i as f32 / 32.0 * TAU;
            v.push((500.0 + 190.0 * a.cos(), 260.0 + 140.0 * a.sin()));
        }
        v
    }

    #[test]
    fn an_oak_fills_its_drawn_crown_from_the_trunk() {
        let c = crown();
        let t = Tree::grow(&c, Some(&[(505.0, 560.0), (498.0, 330.0)]), &Species::oak(), &Season::named("summer").unwrap(), [-0.5, -0.7, 0.3], 3);
        assert!(t.limbs.len() > 60, "{}", t.limbs.len());
        assert_eq!(t.limbs[0].order, 0);
        assert!((t.limbs[0].pts[0].1 - 560.0).abs() < 1.0);
        // limbs reach the crown's edge on both sides and the top
        let (x0, y0, x1, _) = t.bounds();
        assert!(x0 < 360.0 && x1 > 640.0 && y0 < 150.0, "{:?}", t.bounds());
        // few tips outside the drawn crown
        let wood: Vec<&Limb> = t.limbs.iter().filter(|l| !l.twig && l.order > 0).collect();
        let out = wood.iter().filter(|l| { let p = *l.pts.last().unwrap(); !inside(&c, p.0, p.1) && edge_dist(&c, p.0, p.1) > t.step * 3.0 }).count();
        assert!(out * 10 < wood.len(), "{out} of {}", wood.len());
        assert!(t.clumps.len() > 300 && t.touches.len() > 1000, "{} {}", t.clumps.len(), t.touches.len());
        // the sunward side is lighter than the far side
        let left: Vec<f32> = t.clumps.iter().filter(|c| c.at.0 < 420.0).map(|c| c.lit).collect();
        let right: Vec<f32> = t.clumps.iter().filter(|c| c.at.0 > 580.0).map(|c| c.lit).collect();
        let m = |v: &[f32]| v.iter().sum::<f32>() / v.len() as f32;
        assert!(m(&left) > m(&right) + 0.1, "{} {}", m(&left), m(&right));
        // the trunk is the widest wood
        assert!(t.limbs[0].w[0] > 20.0, "{}", t.limbs[0].w[0]);
    }

    /// Straight runs (interior nodes that barely turn) and hooks (net
    /// turning over three segments beyond 120 degrees) on the model limbs.
    fn runs_and_hooks(t: &Tree) -> (f32, usize) {
        let (mut straight, mut inner, mut hooks) = (0, 0, 0);
        for l in t.limbs.iter().filter(|l| !l.twig && l.order > 0) {
            let a: Vec<f32> = l.pts.windows(2).map(|w| (w[1].1 - w[0].1).atan2(w[1].0 - w[0].0)).collect();
            let turn: Vec<f32> = a.windows(2).map(|w| { let mut d = w[1] - w[0]; while d > PI { d -= TAU; } while d < -PI { d += TAU; } d }).collect();
            inner += turn.len();
            straight += turn.iter().filter(|d| d.abs() < 0.05).count();
            hooks += turn.windows(3).filter(|w| (w[0] + w[1] + w[2]).abs() > 2.0 * PI / 3.0).count();
        }
        (straight as f32 / inner.max(1) as f32, hooks)
    }

    #[test]
    fn an_oak_is_angular_a_beech_smooth() {
        let c = crown();
        let trunk = [(505.0, 560.0), (498.0, 330.0)];
        let w = Season::named("winter").unwrap();
        let oak = Tree::grow(&c, Some(&trunk), &Species::oak(), &w, [-0.5, -0.7, 0.3], 3);
        let round = Tree::grow(&c, Some(&trunk), &Species { angular: 0.0, ..Species::oak() }, &w, [-0.5, -0.7, 0.3], 3);
        let (so, ho) = runs_and_hooks(&oak);
        let (sr, hr) = runs_and_hooks(&round);
        // the oak runs straight between a few turning nodes, and hooks back less
        assert!(so > 0.5 && sr < 0.25, "straight nodes: oak {so}, not angular {sr}");
        assert!(ho < hr, "hooks: oak {ho}, not angular {hr}");
        // a beech keeps its smooth curves
        let beech = Tree::grow(&c, Some(&trunk), &Species::beech(), &w, [-0.5, -0.7, 0.3], 3);
        assert!(!beech.angular && runs_and_hooks(&beech).0 < so - 0.2, "{}", runs_and_hooks(&beech).0);
        // wood thins at forks, and twigs end thinner than they start
        for l in oak.limbs.iter().filter(|l| l.twig) {
            assert!(l.w[l.w.len() - 1] < l.w[0]);
        }
    }

    #[test]
    fn winter_is_bare_and_seasons_thin() {
        let c = crown();
        let tr = [(505.0, 560.0), (498.0, 330.0)];
        let sum = Tree::grow(&c, Some(&tr), &Species::oak(), &Season::named("summer").unwrap(), [-0.5, -0.7, 0.3], 3);
        let aut = Tree::grow(&c, Some(&tr), &Species::oak(), &Season::named("autumn").unwrap(), [-0.5, -0.7, 0.3], 3);
        let win = Tree::grow(&c, Some(&tr), &Species::oak(), &Season::named("winter").unwrap(), [-0.5, -0.7, 0.3], 3);
        assert_eq!(sum.limbs.len(), win.limbs.len(), "the same wood in every season");
        // the same crown drawn elsewhere grows the same tree
        let moved: Vec<(f32, f32)> = c.iter().map(|p| (p.0 + 322.0, p.1 + 7.0)).collect();
        let tr2 = [(827.0, 567.0), (820.0, 337.0)];
        let win2 = Tree::grow(&moved, Some(&tr2), &Species::oak(), &Season::named("winter").unwrap(), [-0.5, -0.7, 0.3], 3);
        assert_eq!(win.limbs.len(), win2.limbs.len());
        let (a, b) = (win.limbs[5].pts[1], win2.limbs[5].pts[1]);
        assert!((a.0 + 322.0 - b.0).abs() < 0.01 && (a.1 + 7.0 - b.1).abs() < 0.01, "{a:?} {b:?}");
        assert!(aut.clumps.len() < sum.clumps.len() * 8 / 10);
        assert!(win.clumps.len() < sum.clumps.len() / 10);
        assert!(win.clumps.iter().all(|c| c.dead));
        assert!(aut.clumps.iter().map(|c| c.turn).sum::<f32>() > 0.3 * aut.clumps.len() as f32);
    }

    #[test]
    fn species_differ_and_a_group_recedes() {
        let c = crown();
        let s = Season::named("summer").unwrap();
        let birch = Tree::grow(&c, None, &Species::birch(), &s, [-0.5, -0.7, 0.3], 4);
        let oak = Tree::grow(&c, None, &Species::oak(), &s, [-0.5, -0.7, 0.3], 4);
        let hang = |t: &Tree| t.limbs.iter().filter(|l| l.twig).map(|l| l.pts[l.pts.len() - 1].1 - l.pts[0].1).sum::<f32>() / t.limbs.iter().filter(|l| l.twig).count() as f32;
        assert!(hang(&birch) > hang(&oak) + 2.0, "birch twigs hang: {} vs {}", hang(&birch), hang(&oak));
        assert!(birch.limbs[0].w[0] < oak.limbs[0].w[0]);
        let a = Tree::grow(&c, None, &Species::oak(), &s, [-0.5, -0.7, 0.3], 4);
        assert_eq!(a.touches.len(), oak.touches.len());
        let small: Vec<(f32, f32)> = c.iter().map(|p| (p.0 * 0.3, p.1 * 0.3 + 300.0)).collect();
        let g = Group::grow(&[small], &[None], &[Species::oak()], &s, Some(420.0), &GroupSpec { extra: 3, ..GroupSpec::default() }, 5);
        assert_eq!(g.trees.len(), 4);
        // back to front: hazier first, smaller
        assert!(g.haze[0] >= g.haze[3] && g.scale[0] <= g.scale[3]);
        assert_eq!(g.haze[3], 0.0);
    }
}
