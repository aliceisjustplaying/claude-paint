//! How trees grow: a botanical growth model that returns a skeleton (limbs
//! as polylines with widths), not paint. What a tree looks like when painted
//! belongs to the painter; this module only knows how wood grows.
//!
//! The model follows Palubicki et al., "Self-organizing tree models for image
//! synthesis" (SIGGRAPH 2009), in 3D, projected onto the picture plane:
//!
//! - a tree is a hierarchy of internodes (metamers) ending in nodes that
//!   carry buds: one apical bud at the tip of each shoot and lateral buds in
//!   the leaf axils, placed by phyllotaxy (a spiral divergence angle, or
//!   whorls at the end of each annual shoot as in spruce);
//! - every year each bud gets light from a shadow grid (shadow propagation:
//!   foliage casts a pyramid of shade below it), light is summed from the
//!   buds to the base, and the tree's vigor is handed back out from the base
//!   by the Borchert–Honda rule: at each fork the continuing axis gets the
//!   share `λ·Qm / (λ·Qm + (1−λ)·ΣQl)` (apical dominance `λ`);
//! - a bud with enough vigor breaks and grows a shoot of several internodes;
//!   each new internode keeps its direction, turns toward the least shade
//!   (phototropism), up or down by gravitropism (per branch order: leaders
//!   up, spruce limbs level, birch twigs weeping), and wanders a little;
//! - when a terminal bud aborts (as oak terminal buds often do) the strongest
//!   of the buds clustered below it takes over: sympodial growth, the
//!   zig-zag of old oak limbs; dormant buds on old wood may break later
//!   (reiteration);
//! - branches that earn too little light for their size are shed, leaving a
//!   stub on the parent if they were thick;
//! - widths follow the pipe model (Leonardo's rule): `w^k = Σ w_child^k`
//!   with `k` about 2–2.5, from equal twigs up; wood never shrinks, so a
//!   limb that lost its twigs stays as thick as it grew.
//!
//! A dead or dying tree is the same tree after decline: branches die (the
//! top first: stag-headed), dead wood loses its twigs year by year leaving
//! short claws, and dead limbs break, ending in a jagged break.
//!
//! Limbs of the returned skeleton follow the thickest path through each fork
//! (the way the eye reads a limb), so a sympodial limb is one zig-zag
//! polyline. Species are data (`Habit`), not code paths. Deterministic by
//! seed.

use crate::canvas::Frame;
use crate::mask::Mask;
use crate::noise::Fbm;
use rayon::prelude::*;
use crate::rng::Rng;
use crate::shape::Shape;
use std::f32::consts::PI;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct V3 {
    x: f32,
    y: f32,
    z: f32,
}

impl V3 {
    const UP: V3 = V3 { x: 0.0, y: 1.0, z: 0.0 };
    fn new(x: f32, y: f32, z: f32) -> Self {
        V3 { x, y, z }
    }
    fn add(self, o: V3) -> V3 {
        V3::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
    fn sub(self, o: V3) -> V3 {
        V3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
    fn mul(self, k: f32) -> V3 {
        V3::new(self.x * k, self.y * k, self.z * k)
    }
    fn dot(self, o: V3) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    fn cross(self, o: V3) -> V3 {
        V3::new(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
    }
    fn len(self) -> f32 {
        self.dot(self).sqrt()
    }
    fn norm(self) -> V3 {
        let l = self.len();
        if l > 1e-9 { self.mul(1.0 / l) } else { V3::UP }
    }
    /// Two unit vectors perpendicular to `self` (unit) and to each other.
    fn basis(self) -> (V3, V3) {
        let h = if self.y.abs() < 0.9 { V3::UP } else { V3::new(1.0, 0.0, 0.0) };
        let a = self.cross(h).norm();
        (a, self.cross(a).norm())
    }
}

/// A species' way of growing, as numbers. See `Habit::oak`, `dead_oak`,
/// `birch`, `spruce`; tweak fields for individuals.
#[derive(Clone, Debug)]
pub struct Habit {
    /// Years of growth.
    pub years: u32,
    /// Apical dominance λ (0.5 = none, 0.7 = strong leader).
    pub apical: f32,
    /// Vigor per unit of light gathered (sets how fast the tree grows).
    pub vigor: f32,
    /// Most internodes a shoot can make in one year.
    pub shoot_max: u32,
    /// Internode length relative to the first year's (young shoots on high
    /// orders are shorter: length × `shorten^order`).
    pub shorten: f32,
    /// Angle between a lateral bud and its parent axis (radians).
    pub branch_angle: f32,
    /// Phyllotactic divergence between successive nodes (radians; 2.4 ≈
    /// 137.5° spiral, π = distichous).
    pub divergence: f32,
    /// Lateral buds in each leaf axil along a shoot (0 = none).
    pub node_buds: u32,
    /// Extra lateral buds clustered at the end of each shoot (oak's cluster,
    /// spruce's whorl), for the leader and for side shoots.
    pub tip_buds: (u32, u32),
    /// Probability per year that a terminal bud aborts and the buds below it
    /// take over (sympodial growth: zig-zag limbs).
    pub abort: f32,
    /// Years a lateral bud stays able to break.
    pub bud_life: u32,
    /// Probability a bud stays dormant indefinitely instead (reiteration:
    /// epicormic shoots on old wood).
    pub dormant: f32,
    /// Gravitropism by branch order 0, 1, 2, 3+: + bends shoots up, − down
    /// (weeping), 0 keeps their angle.
    pub tropism: [f32; 4],
    /// How strongly shoots turn toward the least shade.
    pub photo: f32,
    /// Shade tolerance: the shadow at which a bud gets no light (oak and
    /// birch need light, spruce grows on in shade).
    pub shade: f32,
    /// How much a shoot wanders, per internode.
    pub wander: f32,
    /// 0 = a round crown, 1 = grown in the picture plane only.
    pub flat: f32,
    /// Branches earning less light than this per internode are shed.
    pub shed: f32,
    /// Shed branches thicker than this (fraction of the parent) leave a stub.
    pub stub: f32,
    /// Trunk width at the base / tree height.
    pub trunk: f32,
    /// Width of the youngest twigs / tree height. The pipe-model exponent
    /// `k` (`w^k = Σ w_child^k`) is solved from trunk, twig and the number
    /// of tips; it is reported as `Skeleton::pipe`.
    pub twig: f32,
    /// Root flare: extra width at the foot.
    pub flare: f32,
    /// Buttress roots spreading into the ground.
    pub roots: u32,
    /// Lean of the young trunk (radians, + = right).
    pub lean: f32,
    /// Decline after growth: 0 = alive, 1 = long dead (fraction of limbs
    /// dead, the top first).
    pub decline: f32,
    /// How much of the fine wood dead limbs have lost (0..1).
    pub decay: f32,
    /// Probability a dead limb has broken off somewhere along it; a quarter
    /// of that for live limbs (storms).
    pub breakage: f32,
    /// How the species carries its leaves (see `Skeleton::foliage`).
    pub leaf: Leafing,
}

impl Habit {
    /// An open-grown oak: low apical control, twisting sympodial limbs,
    /// clustered buds, a broad crown on a short thick bole.
    pub fn oak() -> Self {
        Habit {
            years: 24,
            apical: 0.53,
            vigor: 2.2,
            shoot_max: 4,
            shorten: 0.9,
            branch_angle: 1.0,
            divergence: 2.51,
            node_buds: 1,
            tip_buds: (2, 1),
            abort: 0.5,
            bud_life: 2,
            dormant: 0.03,
            tropism: [0.2, 0.0, -0.03, -0.06],
            photo: 0.45,
            shade: 3.2,
            wander: 0.16,
            flat: 0.55,
            shed: 0.15,
            stub: 0.5,
            trunk: 0.06,
            twig: 0.0012,
            flare: 0.25,
            roots: 3,
            lean: 0.0,
            decline: 0.0,
            decay: 0.0,
            breakage: 0.08,
            leaf: Leafing::oak(),
        }
    }

    /// Friedrich's dead oak: the same oak long dead, stag-headed, the fine
    /// wood gone, big limbs broken.
    pub fn dead_oak() -> Self {
        Habit { decline: 0.8, decay: 0.6, breakage: 0.35, ..Self::oak() }
    }

    /// A birch: slender, a clear leader, fine twigs hanging from the outer
    /// limbs.
    pub fn birch() -> Self {
        Habit {
            years: 20,
            apical: 0.6,
            vigor: 2.4,
            shoot_max: 5,
            shorten: 0.85,
            branch_angle: 0.7,
            divergence: 2.4,
            node_buds: 1,
            tip_buds: (1, 1),
            abort: 0.05,
            bud_life: 1,
            dormant: 0.0,
            tropism: [0.6, 0.12, -0.25, -0.5],
            photo: 0.3,
            shade: 3.0,
            wander: 0.18,
            flat: 0.5,
            shed: 0.14,
            stub: 0.5,
            trunk: 0.03,
            twig: 0.0008,
            flare: 0.1,
            roots: 0,
            lean: 0.0,
            decline: 0.0,
            decay: 0.0,
            breakage: 0.03,
            leaf: Leafing::birch(),
        }
    }

    /// A spruce: a strong straight leader, whorls of level limbs at the end
    /// of each year's shoot, their tips turning up, side shoots flat.
    pub fn spruce() -> Self {
        Habit {
            years: 26,
            apical: 0.8,
            vigor: 2.0,
            shoot_max: 3,
            shorten: 0.7,
            branch_angle: 1.35,
            divergence: 2.4,
            node_buds: 0,
            tip_buds: (5, 2),
            abort: 0.0,
            bud_life: 1,
            dormant: 0.0,
            tropism: [1.2, -0.03, -0.1, -0.2],
            photo: 0.08,
            shade: 6.0,
            wander: 0.08,
            flat: 0.4,
            shed: 0.08,
            stub: 0.6,
            trunk: 0.035,
            twig: 0.0012,
            flare: 0.25,
            roots: 0,
            lean: 0.0,
            decline: 0.0,
            decay: 0.0,
            breakage: 0.0,
            leaf: Leafing::spruce(),
        }
    }

    /// A beech: smooth grey bole, a broad dome; leaves two-ranked
    /// (distichous) on long level sprays, layered, casting deep shade, so
    /// it tolerates shade and keeps inner limbs longer.
    pub fn beech() -> Self {
        Habit {
            years: 24,
            apical: 0.57,
            vigor: 2.2,
            shoot_max: 4,
            shorten: 0.88,
            branch_angle: 0.85,
            divergence: PI,
            node_buds: 1,
            tip_buds: (1, 1),
            abort: 0.1,
            bud_life: 2,
            dormant: 0.01,
            tropism: [0.35, 0.08, -0.08, -0.12],
            photo: 0.35,
            shade: 4.5,
            wander: 0.1,
            flat: 0.5,
            shed: 0.1,
            stub: 0.5,
            trunk: 0.045,
            twig: 0.001,
            flare: 0.3,
            roots: 2,
            lean: 0.0,
            decline: 0.0,
            decay: 0.0,
            breakage: 0.03,
            leaf: Leafing::beech(),
        }
    }

    /// A black alder (of wet meadows and ditches): a clear stem through a
    /// narrow oval crown, limbs spreading at a wide angle, dense dark
    /// foliage.
    pub fn alder() -> Self {
        Habit {
            years: 20,
            apical: 0.68,
            vigor: 2.2,
            shoot_max: 4,
            shorten: 0.8,
            branch_angle: 1.05,
            divergence: 2.09,
            node_buds: 1,
            tip_buds: (1, 1),
            abort: 0.05,
            bud_life: 2,
            dormant: 0.02,
            tropism: [0.9, 0.05, -0.04, -0.08],
            photo: 0.3,
            shade: 3.6,
            wander: 0.14,
            flat: 0.5,
            shed: 0.14,
            stub: 0.5,
            trunk: 0.035,
            twig: 0.001,
            flare: 0.15,
            roots: 1,
            lean: 0.0,
            decline: 0.0,
            decay: 0.0,
            breakage: 0.05,
            leaf: Leafing::alder(),
        }
    }

    /// A white willow (of meadow ditches and banks): low control, limbs
    /// rising steeply from a short leaning bole, the young shoots long,
    /// their ends arching over; narrow leaves in loose streamers.
    pub fn willow() -> Self {
        Habit {
            years: 18,
            apical: 0.52,
            vigor: 2.6,
            shoot_max: 5,
            shorten: 0.9,
            branch_angle: 0.55,
            divergence: 2.4,
            node_buds: 1,
            tip_buds: (2, 1),
            abort: 0.25,
            bud_life: 2,
            dormant: 0.06,
            tropism: [0.4, 0.3, 0.05, -0.3],
            photo: 0.35,
            shade: 3.0,
            wander: 0.2,
            flat: 0.55,
            shed: 0.14,
            stub: 0.5,
            trunk: 0.06,
            twig: 0.0009,
            flare: 0.25,
            roots: 1,
            lean: 0.1,
            decline: 0.0,
            decay: 0.0,
            breakage: 0.12,
            leaf: Leafing::willow(),
        }
    }

    /// Grow a tree standing at `base` (canvas units, the foot of the trunk),
    /// `height` units tall.
    pub fn grow(&self, base: (f32, f32), height: f32, seed: u64) -> Skeleton {
        Grower::new(self, seed).run().skeleton(self, base, height, seed)
    }
}

/// One limb as the eye reads it: from where it springs to its tip.
#[derive(Clone, Debug)]
pub struct Limb {
    /// Canvas units, base to tip.
    pub pts: Vec<(f32, f32)>,
    /// Depth at each point (units, + toward the viewer).
    pub z: Vec<f32>,
    /// Full width at each point (units).
    pub w: Vec<f32>,
    /// 0 = trunk, 1 = a main limb, 2 = its branches, ...
    pub order: u32,
    /// Index of the limb this one springs from, and the index into its
    /// `pts` where it does.
    pub parent: Option<usize>,
    pub at: usize,
    /// Dead wood all along, from where it springs. A limb can also die
    /// part way: see `dead_from`.
    pub dead: bool,
    /// Index into `pts` from which the limb is dead wood to its tip (the
    /// top of a stag-headed leader, a live limb's dead end): segments
    /// `pts[i]..pts[i + 1]` with `i >= dead_from` are dead. 0 when `dead`;
    /// `pts.len()` when it is alive throughout.
    pub dead_from: usize,
    /// Ends in a break (jagged, blunt) rather than a growing tip.
    pub broken: bool,
    /// A buttress root running into the ground.
    pub root: bool,
    /// Age in years of the wood at each point (the internode ending
    /// there): leaves grow on the young wood (`Leafing::years`).
    pub age: Vec<u32>,
}

impl Limb {
    pub fn len(&self) -> f32 {
        self.pts.windows(2).map(|p| dist(p[0], p[1])).sum()
    }
    pub fn is_empty(&self) -> bool {
        self.pts.len() < 2
    }
    /// Whether the segment from `pts[i]` to `pts[i + 1]` is dead wood.
    pub fn dead_at(&self, i: usize) -> bool {
        i >= self.dead_from
    }
    /// Direction of travel at point `i` (unit).
    pub fn dir(&self, i: usize) -> (f32, f32) {
        let n = self.pts.len();
        let a = self.pts[i.saturating_sub(1)];
        let b = self.pts[(i + 1).min(n - 1)];
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let d = (dx * dx + dy * dy).sqrt().max(1e-6);
        (dx / d, dy / d)
    }
}

/// A grown tree: limbs ordered parents first (trunk, main limbs, ... twigs).
#[derive(Clone, Debug)]
pub struct Skeleton {
    pub limbs: Vec<Limb>,
    pub base: (f32, f32),
    pub height: f32,
    /// The pipe-model exponent the widths follow (Leonardo's rule).
    pub pipe: f32,
    /// The species' leafing (from `Habit::leaf`), for `foliage`.
    pub leaf: Leafing,
}

impl Skeleton {
    /// Silhouette mask of the limbs (for glazes, mist, clipping).
    pub fn mask(&self, f: Frame) -> Mask {
        let mut shape = Shape::new();
        for l in self.limbs.iter().filter(|l| !l.is_empty()) {
            shape = shape.ribbon(&l.pts, &l.w);
        }
        Mask::from_shape(f, shape)
    }

    /// Tips of live twigs (where birds perch, where snow does not lie).
    pub fn tips(&self) -> Vec<(f32, f32)> {
        self.limbs.iter().filter(|l| !l.broken && !l.root && !l.is_empty()).map(|l| *l.pts.last().unwrap()).collect()
    }

    /// Bounding box (x0, y0, x1, y1) in canvas units.
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        let mut b = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for l in &self.limbs {
            for p in &l.pts {
                b = (b.0.min(p.0), b.1.min(p.1), b.2.max(p.0), b.3.max(p.1));
            }
        }
        b
    }
}

#[derive(Clone, Debug)]
struct Bud {
    dir: V3,
    apical: bool,
    born: u32,
    q: f32,
    v: f32,
    /// Stays able to break past `bud_life` (dormant, for reiteration).
    sleeper: bool,
}

#[derive(Clone, Debug)]
struct Node {
    p: V3,
    parent: usize,
    /// Continues its parent's shoot (not the first node of a side branch).
    main: bool,
    kids: Vec<usize>,
    order: u32,
    born: u32,
    /// Phyllotactic phase of the next lateral bud on this axis.
    phase: f32,
    buds: Vec<Bud>,
    alive: bool,
    dead: bool,
    broken: bool,
    /// Largest number of tips the subtree ever carried (wood never shrinks).
    wood: f32,
    q: f32,
    v: f32,
    count: f32,
}

const NONE: usize = usize::MAX;
const G: usize = 160;

struct Grid {
    s: Vec<f32>,
    /// World position of cell (0, 0, 0).
    o: V3,
}

impl Grid {
    fn idx(&self, p: V3) -> Option<usize> {
        let (i, j, k) = ((p.x - self.o.x).floor(), (p.y - self.o.y).floor(), (p.z - self.o.z).floor());
        if i < 0.0 || j < 0.0 || k < 0.0 || i >= G as f32 || j >= G as f32 || k >= G as f32 {
            return None;
        }
        Some((j as usize * G + k as usize) * G + i as usize)
    }
    fn at(&self, p: V3) -> f32 {
        self.idx(p).map(|i| self.s[i]).unwrap_or(0.0)
    }
    /// Foliage at `p` shades a pyramid below it.
    fn cast(&mut self, p: V3, sign: f32) {
        let (ci, cj, ck) = ((p.x - self.o.x).floor() as i32, (p.y - self.o.y).floor() as i32, (p.z - self.o.z).floor() as i32);
        for q in 0..6i32 {
            let j = cj - q;
            if j < 0 || j >= G as i32 {
                continue;
            }
            let a = sign * 1.0 * 0.55f32.powi(q);
            for dk in -q..=q {
                let k = ck + dk;
                if k < 0 || k >= G as i32 {
                    continue;
                }
                for di in -q..=q {
                    let i = ci + di;
                    if i < 0 || i >= G as i32 {
                        continue;
                    }
                    self.s[(j as usize * G + k as usize) * G + i as usize] += a;
                }
            }
        }
    }
}


struct Grower<'a> {
    h: &'a Habit,
    nodes: Vec<Node>,
    rng: Rng,
    grid: Grid,
    year: u32,
    /// Stubs of shed branches: (parent node, direction, wood).
    stubs: Vec<(usize, V3, f32)>,
}

impl<'a> Grower<'a> {
    fn new(h: &'a Habit, seed: u64) -> Self {
        let mut rng = Rng::new(seed ^ 0x9e37_79b9_7f4a_7c15);
        let up = V3::new(h.lean.sin(), h.lean.cos(), 0.0);
        let root = Node {
            p: V3::default(),
            parent: NONE,
            main: true,
            kids: vec![],
            order: 0,
            born: 0,
            phase: rng.range(0.0, 2.0 * PI),
            buds: vec![Bud { dir: up, apical: true, born: 0, q: 0.0, v: 0.0, sleeper: false }],
            alive: true,
            dead: false,
            broken: false,
            wood: 1.0,
            q: 0.0,
            v: 0.0,
            count: 0.0,
        };
        let grid = Grid { s: vec![0.0; G * G * G], o: V3::new(-(G as f32) * 0.5, -4.0, -(G as f32) * 0.5) };
        Grower { h, nodes: vec![root], rng, grid, year: 0, stubs: vec![] }
    }

    fn run(mut self) -> Self {
        for y in 1..=self.h.years {
            self.year = y;
            self.step();
        }
        self
    }

    fn live(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.nodes.len()).filter(|&i| self.nodes[i].alive)
    }

    fn step(&mut self) {
        let h = self.h.clone();
        // shade: foliage on the young wood (the last three years' shoots)
        self.grid.s.iter_mut().for_each(|s| *s = 0.0);
        let leafy: Vec<V3> = self.live().filter(|&i| self.nodes[i].born + 3 >= self.year).map(|i| self.nodes[i].p).collect();
        for p in &leafy {
            self.grid.cast(*p, 1.0);
        }
        // light at each bud (its own foliage excluded)
        for i in 0..self.nodes.len() {
            if !self.nodes[i].alive {
                continue;
            }
            let p = self.nodes[i].p;
            let own = if self.nodes[i].born + 3 >= self.year { 1.0 } else { 0.0 };
            for b in self.nodes[i].buds.iter_mut() {
                let s = self.grid.at(p.add(b.dir.mul(0.7)));
                b.q = (1.0 - (s - own).max(0.0) / h.shade).clamp(0.0, 1.0);
            }
        }
        // terminal buds abort now and then; lateral buds die when old
        let year = self.year;
        for i in 0..self.nodes.len() {
            if !self.nodes[i].alive {
                continue;
            }
            let order = self.nodes[i].order;
            let abort = if order == 0 { h.abort * 0.5 } else { h.abort };
            let mut buds = std::mem::take(&mut self.nodes[i].buds);
            // a terminal bud aborts only where buds below can take over
            let heirs = buds.iter().any(|b| !b.apical);
            buds.retain(|b| {
                if b.apical {
                    !(heirs && self.rng.chance(abort))
                } else {
                    b.sleeper || year <= b.born + h.bud_life
                }
            });
            self.nodes[i].buds = buds;
        }
        // light gathered, tips to base
        for i in (0..self.nodes.len()).rev() {
            if !self.nodes[i].alive {
                continue;
            }
            let own: f32 = self.nodes[i].buds.iter().map(|b| b.q).sum();
            let kids: f32 = self.nodes[i].kids.iter().map(|&k| self.nodes[k].q).sum();
            let cnt: f32 = 1.0 + self.nodes[i].kids.iter().map(|&k| self.nodes[k].count).sum::<f32>();
            self.nodes[i].q = own + kids;
            self.nodes[i].count = cnt;
        }
        // shed branches that do not pay their way
        for i in 1..self.nodes.len() {
            let n = &self.nodes[i];
            if !n.alive || n.main || n.born + 3 > self.year || n.count < 3.0 {
                continue;
            }
            if n.q / n.count < h.shed {
                let (par, dir, wood) = (n.parent, n.p.sub(self.nodes[n.parent].p).norm(), n.wood);
                self.stubs.push((par, dir, wood));
                self.kill(i);
                self.nodes[par].kids.retain(|&k| k != i);
            }
        }
        // vigor, base to tips (Borchert–Honda)
        self.nodes[0].v = h.vigor * self.nodes[0].q;
        let lam = h.apical;
        for i in 0..self.nodes.len() {
            if !self.nodes[i].alive {
                continue;
            }
            let v = self.nodes[i].v;
            let main_kid = self.nodes[i].kids.iter().copied().find(|&k| self.nodes[k].main);
            let qm = match main_kid {
                Some(k) => self.nodes[k].q,
                None => self.nodes[i].buds.iter().filter(|b| b.apical).map(|b| b.q).sum(),
            };
            let ql: f32 = self.nodes[i].kids.iter().filter(|&&k| !self.nodes[k].main).map(|&k| self.nodes[k].q).sum::<f32>()
                + self.nodes[i].buds.iter().filter(|b| !b.apical).map(|b| b.q).sum::<f32>();
            let d = lam * qm + (1.0 - lam) * ql;
            if d <= 1e-9 {
                continue;
            }
            let kids = self.nodes[i].kids.clone();
            for k in kids {
                let share = if self.nodes[k].main { lam } else { 1.0 - lam };
                self.nodes[k].v = v * share * self.nodes[k].q / d;
            }
            for b in self.nodes[i].buds.iter_mut() {
                let share = if b.apical { lam } else { 1.0 - lam };
                b.v = v * share * b.q / d;
            }
        }
        // buds with vigor enough break into shoots
        let n0 = self.nodes.len();
        for i in 0..n0 {
            if !self.nodes[i].alive || self.nodes[i].buds.is_empty() {
                continue;
            }
            let buds = std::mem::take(&mut self.nodes[i].buds);
            let mut keep = vec![];
            for b in buds {
                let n = (b.v.floor() as u32).min(h.shoot_max);
                if n == 0 {
                    keep.push(b);
                    continue;
                }
                self.shoot(i, &b, n);
            }
            self.nodes[i].buds = keep;
        }
        // wood: tips carried, never shrinking
        for i in (0..self.nodes.len()).rev() {
            if !self.nodes[i].alive {
                continue;
            }
            let tips = if self.nodes[i].kids.is_empty() { 1.0 } else { self.nodes[i].kids.iter().map(|&k| self.nodes[k].wood).sum() };
            self.nodes[i].wood = self.nodes[i].wood.max(tips);
        }
    }

    fn kill(&mut self, i: usize) {
        let mut stack = vec![i];
        while let Some(j) = stack.pop() {
            self.nodes[j].alive = false;
            stack.extend(self.nodes[j].kids.iter().copied());
        }
    }

    /// Grow a shoot of `n` internodes from bud `b` on node `from`.
    fn shoot(&mut self, from: usize, b: &Bud, n: u32) {
        let h = self.h;
        let order = if b.apical { self.nodes[from].order } else { self.nodes[from].order + 1 };
        let trop = h.tropism[(order as usize).min(3)];
        let len = h.shorten.powi(order as i32);
        let mut d = b.dir;
        let mut prev = from;
        let mut phase = if b.apical { self.nodes[from].phase } else { self.rng.range(0.0, 2.0 * PI) };
        for m in 0..n {
            let p0 = self.nodes[prev].p;
            if m > 0 || b.apical {
                // phototropism: down the shade gradient
                let e = 1.0;
                let gx = self.grid.at(p0.add(V3::new(e, 0.0, 0.0))) - self.grid.at(p0.sub(V3::new(e, 0.0, 0.0)));
                let gy = self.grid.at(p0.add(V3::new(0.0, e, 0.0))) - self.grid.at(p0.sub(V3::new(0.0, e, 0.0)));
                let gz = self.grid.at(p0.add(V3::new(0.0, 0.0, e))) - self.grid.at(p0.sub(V3::new(0.0, 0.0, e)));
                let light = V3::new(-gx, -gy, -gz);
                let light = if light.len() > 1e-4 { light.norm() } else { V3::default() };
                let r = V3::new(self.rng.normal(), self.rng.normal(), self.rng.normal()).mul(h.wander);
                d = d.add(light.mul(h.photo)).add(V3::UP.mul(trop)).add(r).norm();
            } else {
                let r = V3::new(self.rng.normal(), self.rng.normal(), self.rng.normal()).mul(h.wander * 0.5);
                d = d.add(r).norm();
            }
            d.z *= 1.0 - h.flat;
            d = d.norm();
            let p = p0.add(d.mul(len));
            let idx = self.nodes.len();
            let node = Node {
                p,
                parent: prev,
                main: m > 0 || b.apical,
                kids: vec![],
                order,
                born: self.year,
                phase,
                buds: vec![],
                alive: true,
                dead: false,
                broken: false,
                wood: 1.0,
                q: 0.0,
                v: 0.0,
                count: 1.0,
            };
            self.nodes.push(node);
            self.nodes[prev].kids.push(idx);
            let last = m + 1 == n;
            let mut lat = h.node_buds;
            if last {
                lat += if order == 0 { h.tip_buds.0 } else { h.tip_buds.1 };
            }
            let (u, w) = d.basis();
            let mut buds = vec![];
            for k in 0..lat {
                // a whorl spreads evenly; single buds follow the spiral
                let az = if last && lat > 1 { phase + k as f32 * 2.0 * PI / lat as f32 } else { phase };
                phase += h.divergence;
                let side = u.mul(az.cos()).add(w.mul(az.sin()));
                let ang = h.branch_angle * self.rng.range(0.8, 1.2);
                let bd = d.mul(ang.cos()).add(side.mul(ang.sin())).norm();
                let sleeper = self.rng.chance(h.dormant);
                buds.push(Bud { dir: bd, apical: false, born: self.year, q: 0.0, v: 0.0, sleeper });
            }
            if last {
                buds.push(Bud { dir: d, apical: true, born: self.year, q: 0.0, v: 0.0, sleeper: false });
            }
            self.nodes[idx].buds = buds;
            self.nodes[idx].phase = phase;
            prev = idx;
        }
    }

    /// Decline after growth: branches die (the top first), dead wood loses
    /// its twigs leaving claws, limbs break.
    fn decline(&mut self, max_y: f32) {
        let h = self.h;
        let n = self.nodes.len();
        let big = self.nodes[0].wood;
        // death spreads from the crown down; whole branches die at once
        if h.decline > 0.0 {
            for i in 1..n {
                if !self.nodes[i].alive {
                    continue;
                }
                let par = self.nodes[i].parent;
                if self.nodes[par].dead {
                    self.nodes[i].dead = true;
                    continue;
                }
                let t = self.nodes[i].p.y / max_y;
                let stag = self.nodes[i].order == 0 && t > 1.0 - 0.3 * h.decline;
                let branch_root = !self.nodes[i].main;
                let p = h.decline * (0.35 + 0.9 * t) * if self.nodes[i].order <= 1 { 0.8 } else { 0.5 };
                if stag || (branch_root && self.rng.chance(p.min(0.97))) {
                    self.nodes[i].dead = true;
                }
            }
            // dead wood loses the fine twigs; some leave a short claw
            let thin = 1.0 + h.decay * h.decay * 0.025 * big;
            for i in 1..n {
                if !self.nodes[i].alive || !self.nodes[i].dead || self.nodes[i].wood >= thin {
                    continue;
                }
                let par = self.nodes[i].parent;
                if self.nodes[par].wood < thin && self.nodes[par].dead {
                    continue; // removed with its parent
                }
                // i is the first thin node: keep a claw of one or two
                // internodes, or lose it altogether
                if self.rng.chance(0.55 * (1.0 - 0.5 * h.decay)) {
                    let mut j = i;
                    let keep = if self.rng.chance(0.5) { 1 } else { 2 };
                    for _ in 1..keep {
                        match self.nodes[j].kids.iter().copied().filter(|&k| self.nodes[k].alive).max_by(|&a, &b| self.nodes[a].wood.total_cmp(&self.nodes[b].wood)) {
                            Some(k) => {
                                // the claw goes on through its strongest
                                // twig; the others are lost
                                let kids = std::mem::replace(&mut self.nodes[j].kids, vec![k]);
                                for s in kids.into_iter().filter(|&s| s != k) {
                                    self.kill(s);
                                }
                                j = k;
                            }
                            None => break,
                        }
                    }
                    let kids = std::mem::take(&mut self.nodes[j].kids);
                    for k in kids {
                        self.kill(k);
                    }
                } else {
                    self.kill(i);
                    self.nodes[par].kids.retain(|&k| k != i);
                    if self.nodes[par].kids.is_empty() {
                        self.nodes[par].broken = true;
                    }
                }
            }
        }
        // limbs break: dead ones often, live ones in storms
        for i in 1..n {
            let nd = &self.nodes[i];
            if !nd.alive || nd.main || nd.wood < 0.01 * big + 3.0 {
                continue;
            }
            let p = if nd.dead { h.breakage } else { h.breakage * 0.25 };
            if !self.rng.chance(p) {
                continue;
            }
            // walk out along the limb and snap it somewhere
            let mut chain = vec![i];
            let mut j = i;
            while let Some(k) = self.nodes[j].kids.iter().copied().max_by(|&a, &b| self.nodes[a].wood.total_cmp(&self.nodes[b].wood)) {
                chain.push(k);
                j = k;
            }
            if chain.len() < 3 {
                continue;
            }
            let at = chain[((self.rng.range(0.15, 0.7) * chain.len() as f32) as usize).max(1)];
            let kids = std::mem::take(&mut self.nodes[at].kids);
            for k in kids {
                self.kill(k);
            }
            self.nodes[at].broken = true;
            self.nodes[at].buds.clear();
            // the break kills what remains of a live limb beyond its forks
            if !self.nodes[at].dead {
                let mut k = at;
                while k != i && self.nodes[k].kids.len() <= 1 {
                    self.nodes[k].dead = true;
                    k = self.nodes[k].parent;
                }
            }
        }
    }

    /// The limbs as chains of nodes, parents first: each follows the
    /// thickest path through every fork (a little in favor of going
    /// straight on). A limb's chain starts at the node it springs from.
    fn chains(&self) -> Vec<Chain> {
        let mut out: Vec<Chain> = vec![];
        let mut queue: std::collections::VecDeque<(usize, Option<usize>, usize, u32)> = std::collections::VecDeque::new();
        queue.push_back((0, None, 0, 0));
        while let Some((start, parent, at, order)) = queue.pop_front() {
            let mut chain = vec![];
            if start != 0 {
                chain.push(self.nodes[start].parent);
            }
            let mut j = start;
            loop {
                chain.push(j);
                let here = self.nodes[j].p.sub(self.nodes[if self.nodes[j].parent == NONE { j } else { self.nodes[j].parent }].p).norm();
                let next = self.nodes[j].kids.iter().copied().filter(|&k| self.nodes[k].alive).max_by(|&a, &b| {
                    let score = |k: usize| self.nodes[k].wood * (1.2 + 0.3 * self.nodes[k].p.sub(self.nodes[j].p).norm().dot(here));
                    score(a).total_cmp(&score(b))
                });
                for k in self.nodes[j].kids.iter().copied().filter(|&k| self.nodes[k].alive && Some(k) != next) {
                    queue.push_back((k, Some(out.len()), chain.len() - 1, order + 1));
                }
                match next {
                    Some(k) => j = k,
                    None => break,
                }
            }
            out.push(Chain { nodes: chain, start, parent, at, order });
        }
        out
    }

    fn skeleton(mut self, h: &Habit, base: (f32, f32), height: f32, seed: u64) -> Skeleton {
        let max_y0 = self.live().map(|i| self.nodes[i].p.y).fold(1e-3, f32::max);
        self.decline(max_y0);
        let max_y = self.live().map(|i| self.nodes[i].p.y).fold(1e-3, f32::max);
        let s = height / max_y;
        let trunk_w = h.trunk * height;
        let tip_w = (h.twig * height).min(trunk_w * 0.5);
        // Leonardo: w^k = Σ w_child^k from equal tips gives w = tip·N^(1/k);
        // k is whatever makes N tips add up to the trunk
        let pk = (self.nodes[0].wood.max(2.0).ln() / (trunk_w / tip_w).ln()).clamp(1.2, 3.5);
        let tip_w = trunk_w / self.nodes[0].wood.powf(1.0 / pk);
        let width = |wood: f32| tip_w * wood.powf(1.0 / pk);
        let proj = |p: V3| (base.0 + p.x * s, base.1 - p.y * s);
        let mut limbs: Vec<Limb> = vec![];
        for Chain { nodes: chain, start, parent, at, order } in self.chains() {
            let tip = *chain.last().unwrap();
            let mut pts = vec![];
            let mut z = vec![];
            let mut w = vec![];
            let mut age = vec![];
            for (ci, &nd) in chain.iter().enumerate() {
                age.push(self.year.saturating_sub(self.nodes[nd].born));
                pts.push(proj(self.nodes[nd].p));
                z.push(self.nodes[nd].p.z * s);
                // width of the internode leaving this point
                let out = chain.get(ci + 1).map(|&k| self.nodes[k].wood).unwrap_or(self.nodes[nd].wood * 0.7);
                let out = if ci == 0 && start != 0 { self.nodes[start].wood } else { out };
                w.push(width(out));
            }
            // a limb never thickens toward its tip
            for i in 1..w.len() {
                w[i] = w[i].min(w[i - 1]);
            }
            if start == 0 {
                // the foot flares into the roots
                for (i, wi) in w.iter_mut().enumerate() {
                    let y = self.nodes[chain[i]].p.y * s;
                    *wi *= 1.0 + h.flare * (-y / (trunk_w * 0.8)).exp();
                }
            }
            // the internode ending at chain[k + 1] is segment k; death
            // spreads outward, so along a limb it is dead from some point on
            let seg_dead: Vec<bool> = chain.windows(2).map(|p| self.nodes[p[1]].dead).collect();
            let dead_from = seg_dead.iter().position(|&d| d).unwrap_or(pts.len());
            debug_assert!(seg_dead[dead_from.min(seg_dead.len())..].iter().all(|&d| d), "live wood beyond dead wood");
            let dead = dead_from == 0;
            limbs.push(Limb { pts, z, w, order, parent, at, dead, dead_from, broken: self.nodes[tip].broken, root: false, age });
        }
        // stubs of branches shed long ago
        let mut rng = Rng::new(seed ^ 0x51ab);
        for &(par, dir, wood) in &self.stubs {
            if !self.nodes[par].alive {
                continue;
            }
            let pw = width(self.nodes[par].wood);
            let sw = width(wood);
            if sw < h.stub * pw || sw < 0.004 * height {
                continue;
            }
            let p0 = proj(self.nodes[par].p);
            // find the limb point nearest the stub's root
            let (li, pi) = nearest(&limbs, p0);
            let sw = sw.min(limbs[li].w[pi] * 0.8);
            let l = sw * rng.range(1.0, 2.5);
            let d2 = (dir.x, -dir.y);
            let dl = (d2.0 * d2.0 + d2.1 * d2.1).sqrt().max(0.2);
            let p1 = (p0.0 + d2.0 / dl * l, p0.1 + d2.1 / dl * l);
            limbs.push(Limb {
                pts: vec![p0, p1],
                z: vec![self.nodes[par].p.z * s; 2],
                w: vec![sw * 0.9, sw * 0.8],
                order: limbs[li].order + 1,
                parent: Some(li),
                at: pi,
                dead: true,
                dead_from: 0,
                broken: true,
                root: false,
                age: vec![h.years; 2],
            });
        }
        // buttress roots running out into the ground
        for r in 0..h.roots {
            let side = if r % 2 == 0 { 1.0 } else { -1.0 };
            let a = rng.range(0.25, 0.6); // below horizontal
            let l = trunk_w * rng.range(0.9, 1.8);
            let x0 = base.0 + side * trunk_w * rng.range(0.1, 0.35);
            let y0 = base.1 - trunk_w * rng.range(0.15, 0.35);
            let pts = vec![(x0, y0), (x0 + side * l * 0.5 * a.cos(), y0 + l * 0.5 * a.sin() * 0.8), (x0 + side * l * a.cos(), y0 + l * a.sin())];
            let w0 = trunk_w * rng.range(0.4, 0.6);
            limbs.push(Limb { pts, z: vec![0.0; 3], w: vec![w0, w0 * 0.55, w0 * 0.2], order: 1, parent: Some(0), at: 0, dead: false, dead_from: 3, broken: false, root: true, age: vec![h.years; 3] });
        }
        Skeleton { limbs, base, height, pipe: pk, leaf: h.leaf }
    }
}

/// One limb's nodes (see `Grower::chains`).
struct Chain {
    nodes: Vec<usize>,
    start: usize,
    parent: Option<usize>,
    at: usize,
    order: u32,
}

fn nearest(limbs: &[Limb], p: (f32, f32)) -> (usize, usize) {
    let mut best = (0, 0, f32::MAX);
    for (li, l) in limbs.iter().enumerate() {
        for (pi, q) in l.pts.iter().enumerate() {
            let d = dist(*q, p);
            if d < best.2 {
                best = (li, pi, d);
            }
        }
    }
    (best.0, best.1)
}

fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

// ---------------------------------------------------------------------------
// Leaves

/// How a species carries its leaves, as numbers (botany, not paint). Leaves
/// grow on the young wood: each clump stands for a spray or bunch of leaves
/// on one short length of twig. See `Skeleton::foliage`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Leafing {
    /// Wood younger than this (years) carries leaves; 0 = bare (winter).
    pub years: u32,
    /// Radius of one clump / tree height.
    pub clump: f32,
    /// Distance between clumps along leafy wood, in clump radii.
    pub spacing: f32,
    /// Height / width of a clump: flat sprays < 1 (beech), hanging
    /// streamers > 1 (birch, willow).
    pub squash: f32,
    /// How far a clump hangs below its twig, in clump radii.
    pub droop: f32,
    /// How solid a clump is (0 airy, sky through it .. 1 none).
    pub fill: f32,
    /// Irregularity of a clump's outline (0 round .. 1 lobed).
    pub ragged: f32,
    /// Share of leafy twigs carrying no leaves (browsed, broken, dying
    /// back): the holes where sky shows through a crown.
    pub bare: f32,
    /// Leaves crowd toward the ends of shoots: clumps near a tip are up to
    /// `1 + tip` times bigger.
    pub tip: f32,
    /// Short leafy shoots along older live wood (spur shoots, epicormic
    /// sprigs), as a share of the young wood's density, on limbs thinner
    /// than `inner_w` × the trunk: fills the crown's shell instead of
    /// leaving leaves only at the ends of bare limbs.
    pub inner: f32,
    pub inner_w: f32,
    /// Extra clumps around each live shoot tip: a modeled tip stands for a
    /// cluster of real twigs the model does not grow (see `Habit::twig`),
    /// each with its leaves. Mean count per tip.
    pub spray: f32,
}

impl Leafing {
    /// Bare: winter, or a dead tree.
    pub fn none() -> Self {
        Leafing { years: 0, ..Self::oak() }
    }
    /// Pedunculate oak: leaves bunched at the shoot ends around the bud
    /// cluster, in irregular lobed masses with sky between them.
    pub fn oak() -> Self {
        Leafing { years: 3, clump: 0.024, spacing: 1.0, squash: 0.75, droop: 0.15, fill: 0.8, ragged: 0.55, bare: 0.12, tip: 0.5, inner: 0.45, inner_w: 0.3, spray: 2.5 }
    }
    /// Silver birch: small leaves along long hanging twigs, an airy crown
    /// the light passes through.
    pub fn birch() -> Self {
        Leafing { years: 3, clump: 0.018, spacing: 1.0, squash: 1.35, droop: 0.7, fill: 0.5, ragged: 0.35, bare: 0.18, tip: 0.2, inner: 0.25, inner_w: 0.2, spray: 1.5 }
    }
    /// Beech: two-ranked leaves on level sprays, layered, dense (deep shade).
    pub fn beech() -> Self {
        Leafing { years: 3, clump: 0.028, spacing: 0.9, squash: 0.45, droop: 0.2, fill: 0.95, ragged: 0.25, bare: 0.05, tip: 0.25, inner: 0.6, inner_w: 0.35, spray: 2.0 }
    }
    /// Black alder: dense, dark, evenly spread foliage.
    pub fn alder() -> Self {
        Leafing { years: 3, clump: 0.024, spacing: 0.9, squash: 0.85, droop: 0.1, fill: 0.9, ragged: 0.3, bare: 0.06, tip: 0.2, inner: 0.6, inner_w: 0.35, spray: 2.0 }
    }
    /// White willow: narrow leaves in loose, hanging streamers.
    pub fn willow() -> Self {
        Leafing { years: 3, clump: 0.022, spacing: 1.0, squash: 1.6, droop: 0.45, fill: 0.6, ragged: 0.45, bare: 0.1, tip: 0.3, inner: 0.35, inner_w: 0.25, spray: 1.5 }
    }
    /// Spruce: needles on several years of shoots, in flat dense sprays
    /// hanging from the limbs.
    pub fn spruce() -> Self {
        Leafing { years: 7, clump: 0.02, spacing: 0.8, squash: 0.5, droop: 0.35, fill: 0.9, ragged: 0.3, bare: 0.04, tip: 0.1, inner: 0.0, inner_w: 0.0, spray: 0.5 }
    }
}

/// One clump of leaves: an ellipse on the picture plane with depth.
#[derive(Clone, Copy, Debug)]
pub struct Clump {
    /// Center (canvas units) and depth (units, + toward the viewer).
    pub at: (f32, f32),
    pub z: f32,
    /// Half width (units); half height is `r * squash`.
    pub r: f32,
    pub squash: f32,
    /// Rotation of the clump's width axis from horizontal (radians).
    pub tilt: f32,
    /// How solid (see `Leafing::fill`).
    pub fill: f32,
    /// Light the clump catches, 0 (deep in shade) .. 1 (full sun on the
    /// sunward face of the crown).
    pub lit: f32,
    /// Shadow cast on it by clumps between it and the sun (0 none .. 1).
    pub shade: f32,
    /// The limb whose twig carries it, and the main limb (order ≤ 1) that
    /// carries that: the mass it belongs to, as the eye groups a crown.
    pub limb: usize,
    pub mass: usize,
}

/// A tree's leaves: clumps of leaves on the young wood, lit by the sun.
/// Geometry, not paint: `mask` is where leaves are (with the holes where
/// sky shows through), `lit` how much light each part catches, `gaps` the
/// holes inside the crown. The painter decides what to do with them.
#[derive(Clone, Debug)]
pub struct Foliage {
    pub clumps: Vec<Clump>,
    /// Unit vector toward the sun (canvas axes: x right, y down, z toward
    /// the viewer).
    pub sun: (f32, f32, f32),
    pub ragged: f32,
    seed: u64,
}

impl Skeleton {
    /// The leaves the species carries (`Habit::leaf`), lit from `sun`
    /// (toward the sun, canvas axes: x right, y down, z toward the viewer;
    /// normalized here). Only live wood carries leaves.
    pub fn foliage(&self, sun: (f32, f32, f32), seed: u64) -> Foliage {
        self.foliage_with(&self.leaf, sun, seed)
    }

    /// Foliage with another leafing (a thinner crown in drought, a sparse
    /// autumn, a hedge clipped dense).
    pub fn foliage_with(&self, leaf: &Leafing, sun: (f32, f32, f32), seed: u64) -> Foliage {
        let mut rng = Rng::new(seed ^ 0xf011_a6e5);
        let r0 = leaf.clump * self.height;
        let mut clumps = vec![];
        let n = self.limbs.len();
        let mut mass = vec![0usize; n];
        for (i, l) in self.limbs.iter().enumerate() {
            mass[i] = match l.parent {
                Some(p) if l.order > 1 => mass[p],
                _ => i,
            };
        }
        let trunk_w = self.limbs.first().map_or(1.0, |l| l.w[0]);
        if leaf.years > 0 && r0 > 0.0 {
            for (li, l) in self.limbs.iter().enumerate() {
                if l.root || l.is_empty() || (l.order >= 2 && rng.chance(leaf.bare)) {
                    continue;
                }
                let arc = {
                    let mut a = vec![0.0f32; l.pts.len()];
                    for i in 1..l.pts.len() {
                        a[i] = a[i - 1] + dist(l.pts[i - 1], l.pts[i]);
                    }
                    a
                };
                let total = *arc.last().unwrap();
                let mut next = rng.range(0.0, 1.0) * r0 * leaf.spacing;
                for i in 0..l.pts.len() - 1 {
                    if l.dead_at(i) {
                        break;
                    }
                    let seg = arc[i + 1] - arc[i];
                    let young = l.age[i + 1] < leaf.years;
                    let sprigs = !young && leaf.inner > 0.0 && l.w[i] < leaf.inner_w * trunk_w;
                    if !(young || sprigs) || seg <= 1e-6 {
                        next = next.max(arc[i + 1]);
                        continue;
                    }
                    while next < arc[i + 1] {
                        if !young && !rng.chance(leaf.inner) {
                            next += r0 * leaf.spacing * rng.range(0.7, 1.3);
                            continue;
                        }
                        let t = ((next - arc[i]) / seg).clamp(0.0, 1.0);
                        let (a, b) = (l.pts[i], l.pts[i + 1]);
                        let d = ((b.0 - a.0) / seg, (b.1 - a.1) / seg);
                        let to_tip = total - next;
                        let r = r0 * rng.range(0.7, 1.25) * (1.0 + leaf.tip * (-to_tip / (3.0 * r0)).exp());
                        let off = rng.normal() * 0.35 * r;
                        let x = a.0 + (b.0 - a.0) * t - d.1 * off;
                        let y = a.1 + (b.1 - a.1) * t + d.0 * off + leaf.droop * r * leaf.squash;
                        let z = l.z[i] + (l.z[i + 1] - l.z[i]) * t + rng.normal() * 0.3 * r;
                        // flat sprays lie partly along their twig; hanging
                        // ones hang
                        let along = d.1.atan2(d.0);
                        let along = if along > PI / 2.0 { along - PI } else if along < -PI / 2.0 { along + PI } else { along };
                        let tilt = if leaf.squash < 1.0 { along * 0.35 } else { 0.0 } + rng.normal() * 0.12;
                        clumps.push(Clump {
                            at: (x, y),
                            z,
                            r,
                            squash: leaf.squash * rng.range(0.85, 1.15),
                            tilt,
                            fill: (leaf.fill * rng.range(0.85, 1.1)).min(1.0),
                            lit: 0.0,
                            shade: 0.0,
                            limb: li,
                            mass: mass[li],
                        });
                        next += r0 * leaf.spacing * rng.range(0.7, 1.3);
                    }
                }
                // the unmodeled twigs around a live tip, each with leaves
                let last = l.pts.len() - 1;
                if l.dead_from >= last && !l.broken && l.age[last] < leaf.years && leaf.spray > 0.0 {
                    let d = l.dir(last);
                    let k = (leaf.spray + rng.f()).floor() as usize;
                    for _ in 0..k {
                        let r = r0 * rng.range(0.6, 1.0) * (1.0 + leaf.tip);
                        let reach = r0 * rng.range(0.6, 1.8);
                        let side = rng.normal();
                        let x = l.pts[last].0 + d.0 * reach * 0.6 - d.1 * side * reach;
                        let y = l.pts[last].1 + d.1 * reach * 0.6 + d.0 * side * reach + leaf.droop * r * leaf.squash;
                        let z = l.z[last] + rng.normal() * reach;
                        let tilt = rng.normal() * 0.2;
                        clumps.push(Clump { at: (x, y), z, r, squash: leaf.squash * rng.range(0.85, 1.15), tilt, fill: (leaf.fill * rng.range(0.85, 1.1)).min(1.0), lit: 0.0, shade: 0.0, limb: li, mass: mass[li] });
                    }
                }
            }
        }
        let mut f = Foliage { clumps, sun: norm3(sun), ragged: leaf.ragged, seed };
        f.light();
        f
    }
}

fn norm3(v: (f32, f32, f32)) -> (f32, f32, f32) {
    let l = (v.0 * v.0 + v.1 * v.1 + v.2 * v.2).sqrt().max(1e-6);
    (v.0 / l, v.1 / l, v.2 / l)
}

impl Foliage {
    /// Light on each clump: its facing on the crown's envelope (the sunward
    /// side of the crown is lit, the far side and the inside dark), dimmed
    /// by the clumps between it and the sun.
    fn light(&mut self) {
        let n = self.clumps.len();
        if n == 0 {
            return;
        }
        let s = self.sun;
        let wsum: f32 = self.clumps.iter().map(|c| c.r * c.r).sum();
        let mean = |f: &dyn Fn(&Clump) -> f32| self.clumps.iter().map(|c| f(c) * c.r * c.r).sum::<f32>() / wsum;
        let (cx, cy, cz) = (mean(&|c| c.at.0), mean(&|c| c.at.1), mean(&|c| c.z));
        let rmax = self.clumps.iter().map(|c| c.r).fold(0.0, f32::max);
        let sx = (mean(&|c| (c.at.0 - cx).powi(2))).sqrt() * 1.7 + rmax;
        let sy = (mean(&|c| (c.at.1 - cy).powi(2))).sqrt() * 1.7 + rmax;
        let sz = (mean(&|c| (c.z - cz).powi(2))).sqrt() * 1.7 + rmax;
        let pos: Vec<(f32, f32, f32)> = self.clumps.iter().map(|c| (c.at.0, c.at.1, c.z)).collect();
        for i in 0..n {
            let c = self.clumps[i];
            let v = ((c.at.0 - cx) / sx, (c.at.1 - cy) / sy, (c.z - cz) / sz);
            let m = (v.0 * v.0 + v.1 * v.1 + v.2 * v.2).sqrt().max(1e-6);
            let facing = 0.5 + 0.5 * (v.0 * s.0 + v.1 * s.1 + v.2 * s.2) / m;
            let outer = crate::smoothstep(0.15, 0.85, m);
            // clumps between this one and the sun
            let mut through = 1.0f32;
            for (j, pj) in pos.iter().enumerate() {
                if j == i {
                    continue;
                }
                let d = (pj.0 - pos[i].0, pj.1 - pos[i].1, pj.2 - pos[i].2);
                let t = d.0 * s.0 + d.1 * s.1 + d.2 * s.2;
                if t <= 0.5 * c.r {
                    continue;
                }
                let perp2 = d.0 * d.0 + d.1 * d.1 + d.2 * d.2 - t * t;
                let rj = self.clumps[j].r;
                if perp2 < rj * rj * 0.8 {
                    through *= 1.0 - 0.45 * self.clumps[j].fill;
                }
            }
            let shade = 1.0 - through;
            self.clumps[i].shade = shade;
            self.clumps[i].lit = (facing.powf(1.3) * (0.4 + 0.6 * outer) * (1.0 - 0.8 * shade)).clamp(0.0, 1.0);
        }
    }

    /// Clumps from the back (far from the viewer) to the front.
    pub fn back_to_front(&self) -> Vec<&Clump> {
        let mut v: Vec<&Clump> = self.clumps.iter().collect();
        v.sort_by(|a, b| a.z.total_cmp(&b.z));
        v
    }

    /// Bounding box (x0, y0, x1, y1) of the clumps, units.
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        let mut b = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for c in &self.clumps {
            let (rx, ry) = (c.r * 1.4, c.r * c.squash * 1.4);
            b = (b.0.min(c.at.0 - rx), b.1.min(c.at.1 - ry), b.2.max(c.at.0 + rx), b.3.max(c.at.1 + ry));
        }
        b
    }

    /// Mean clump radius (units): the scale of the leaf masses.
    pub fn grain(&self) -> f32 {
        if self.clumps.is_empty() {
            return 1.0;
        }
        self.clumps.iter().map(|c| c.r).sum::<f32>() / self.clumps.len() as f32
    }

    /// Coverage (0..1) of clump `k` at (x, y) before its fill: 1 inside its
    /// lobed outline, soft over the last quarter of its radius; and the
    /// point in the clump's own frame (u, v in radii).
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
        let lobe = 1.0 + self.ragged * 0.32 * (0.6 * (3.0 * th + h * std::f32::consts::TAU).sin() + 0.4 * (5.0 * th + h2 * std::f32::consts::TAU).sin());
        (1.0 - crate::smoothstep(lobe * 0.5, lobe * 1.1, d), dx / c.r, dy / (c.r * c.squash))
    }

    /// Pixel ranges (buffer coordinates) a clump can touch in frame `f`.
    fn px_box(&self, f: &Frame, k: usize) -> Option<(usize, usize, usize, usize)> {
        let c = &self.clumps[k];
        let rr = c.r * c.squash.max(1.0) * (1.0 + self.ragged * 0.35);
        let x0 = ((c.at.0 - rr) * f.scale).floor() as isize - f.x0 as isize;
        let x1 = ((c.at.0 + rr) * f.scale).ceil() as isize - f.x0 as isize;
        let y0 = ((c.at.1 - rr) * f.scale).floor() as isize - f.y0 as isize;
        let y1 = ((c.at.1 + rr) * f.scale).ceil() as isize - f.y0 as isize;
        let (x0, y0) = (x0.max(0), y0.max(0));
        let (x1, y1) = (x1.min(f.w as isize - 1), y1.min(f.h as isize - 1));
        (x1 >= x0 && y1 >= y0).then_some((x0 as usize, y0 as usize, x1 as usize, y1 as usize))
    }

    /// Where the leaves are: 1 on leaves, 0 on sky, with the holes of an
    /// airy clump and the gaps between clumps (a leaf-sized noise breaks
    /// the edge of every partly covered place into leaves and sky).
    pub fn mask(&self, f: Frame) -> Mask {
        let mut cov = vec![0.0f32; f.w * f.h];
        for k in 0..self.clumps.len() {
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
        let leaf = Fbm::new(self.seed as u32 ^ 0x1eaf, 3, (self.grain() * 0.45).max(0.5));
        let inv = 1.0 / f.scale;
        let data = cov
            .par_iter()
            .enumerate()
            .map(|(i, &c)| {
                if c <= 0.0 {
                    return 0.0;
                }
                let (x, y) = (((i % f.w) + f.x0) as f32 * inv, ((i / f.w) + f.y0) as f32 * inv);
                crate::smoothstep(0.4, 0.6, c + 0.35 * leaf.get(x, y))
            })
            .collect();
        Mask { f, data }
    }

    /// Light on the leaves (0..1; 0 off them): each clump's `lit`, the front
    /// clump over the ones behind, rounded like a small ball of leaves (its
    /// sunward side lighter, its underside darker).
    pub fn lit(&self, f: Frame) -> Mask {
        let mut val = vec![0.0f32; f.w * f.h];
        let s = self.sun;
        let mut order: Vec<usize> = (0..self.clumps.len()).collect();
        order.sort_by(|&a, &b| self.clumps[a].z.total_cmp(&self.clumps[b].z));
        for k in order {
            let Some((x0, y0, x1, y1)) = self.px_box(&f, k) else { continue };
            let lit = self.clumps[k].lit;
            for py in y0..=y1 {
                let y = (py + f.y0) as f32 / f.scale + 0.5 / f.scale;
                for px in x0..=x1 {
                    let x = (px + f.x0) as f32 / f.scale + 0.5 / f.scale;
                    let (c, u, v) = self.cover(k, x, y);
                    if c > 0.5 {
                        let w = (1.0 - u * u - v * v).max(0.0).sqrt();
                        let local = (u * s.0 + v * s.1 + w * s.2).clamp(-1.0, 1.0);
                        val[py * f.w + px] = (lit * (0.82 + 0.25 * local)).clamp(0.0, 1.0);
                    }
                }
            }
        }
        Mask { f, data: val }.mul(&self.mask(f))
    }

    /// The crown's outline with its holes closed (the leaves' mask grown and
    /// shrunk by `reach` units).
    pub fn envelope(&self, f: Frame, reach: f32) -> Mask {
        self.mask(f).dilate(reach).erode(reach)
    }

    /// Holes inside the crown where sky shows through (`envelope` minus the
    /// leaves).
    pub fn gaps(&self, f: Frame, reach: f32) -> Mask {
        let m = self.mask(f);
        m.dilate(reach).erode(reach).subtract(&m)
    }
}

// ---------------------------------------------------------------------------
// Ground cover

/// Wind over a meadow: how far grass leans (radians from upright, + to
/// the right), in gusts.
#[derive(Clone, Copy, Debug)]
pub struct Wind {
    /// Mean lean (radians, + right).
    pub lean: f32,
    /// Gusts: lean varies by up to about this much from place to place.
    pub gust: f32,
    /// Size of a gust (units).
    pub period: f32,
    pub seed: u64,
}

impl Wind {
    pub fn calm() -> Self {
        Wind { lean: 0.0, gust: 0.08, period: 120.0, seed: 1 }
    }
    /// Lean at (x, y), radians.
    pub fn at(&self, x: f32, y: f32) -> f32 {
        self.lean + self.gust * (value_noise(x / self.period, y / self.period, self.seed) * 0.7 + value_noise(x * 2.3 / self.period, y * 2.3 / self.period, self.seed ^ 0x77) * 0.3)
    }
}

/// Smooth value noise in about [-1, 1].
fn value_noise(x: f32, y: f32, seed: u64) -> f32 {
    let (xi, yi) = (x.floor(), y.floor());
    let (tx, ty) = (x - xi, y - yi);
    let (sx, sy) = (tx * tx * (3.0 - 2.0 * tx), ty * ty * (3.0 - 2.0 * ty));
    let h = |i: f32, j: f32| crate::rng::hash2(i as i64, j as i64, seed) * 2.0 - 1.0;
    let a = h(xi, yi) + (h(xi + 1.0, yi) - h(xi, yi)) * sx;
    let b = h(xi, yi + 1.0) + (h(xi + 1.0, yi + 1.0) - h(xi, yi + 1.0)) * sx;
    a + (b - a) * sy
}

/// A meadow or grassy bank as a set of seeded tufts on a ground plane seen
/// in perspective: near the viewer tall tufts of many blades, toward the
/// horizon fewer and smaller ones, until they are too small to be marks.
/// Geometry, not paint: `grow` returns tufts (blades as curves, flower
/// heads) and the painter paints them.
#[derive(Clone, Copy, Debug)]
pub struct Sward {
    /// Canvas y of the horizon (marks shrink to nothing there) and of the
    /// near edge where they have full size.
    pub horizon: f32,
    pub near: f32,
    /// Tuft height at `near` (units).
    pub height: f32,
    /// Mean distance between tufts at `near` (units).
    pub spacing: f32,
    /// How fast marks thin out toward the horizon: the spacing grows as
    /// `scale^-thin` (0 = as many marks per area far as near, only smaller).
    pub thin: f32,
    /// No mark shorter than this (units): below it the painter paints the
    /// ground's tone, not tufts.
    pub smallest: f32,
    /// Blades per tuft near the viewer (fewer far away).
    pub blades: (u32, u32),
    /// Fan of the blades in a tuft (radians).
    pub fan: f32,
    /// How much blades bend over at the tip, with the wind (radians).
    pub curl: f32,
    /// Share of tufts carrying a flower head, where flowers grow.
    pub flowers: f32,
    /// Kinds of flowers (each grows in its own patches); `Flower::kind`.
    pub kinds: u32,
    /// Patchiness: tufts crowd in lush patches and thin in poor ones
    /// (0 even .. 1 strongly patchy), over `patch_size` units.
    pub patch: f32,
    pub patch_size: f32,
    pub wind: Wind,
}

impl Default for Sward {
    fn default() -> Self {
        Sward {
            horizon: 400.0,
            near: 666.0,
            height: 14.0,
            spacing: 4.0,
            thin: 0.5,
            smallest: 0.6,
            blades: (3, 7),
            fan: 0.7,
            curl: 0.35,
            flowers: 0.04,
            kinds: 3,
            patch: 0.6,
            patch_size: 90.0,
            wind: Wind::calm(),
        }
    }
}

/// A flower head on a tuft's tallest blade.
#[derive(Clone, Copy, Debug)]
pub struct Flower {
    pub at: (f32, f32),
    pub r: f32,
    pub kind: u32,
}

/// One tuft of grass: blades from a common foot.
#[derive(Clone, Debug)]
pub struct Tuft {
    /// Foot (canvas units).
    pub at: (f32, f32),
    /// Perspective scale there (1 at `near`, 0 at the horizon).
    pub scale: f32,
    pub height: f32,
    /// Lean (radians from upright, + right).
    pub lean: f32,
    /// Blades as quadratic curves: foot, control point, tip (units).
    pub blades: Vec<[(f32, f32); 3]>,
    pub flower: Option<Flower>,
    /// How lush the patch is here (0..1).
    pub lush: f32,
}

impl Sward {
    /// Perspective scale at canvas height `y`: 0 at the horizon, 1 at
    /// `near` (more below it).
    pub fn scale_at(&self, y: f32) -> f32 {
        ((y - self.horizon) / (self.near - self.horizon)).max(0.0)
    }

    /// Tufts inside `region` (the ground to cover, ≥ 0.5), far ones first
    /// (paint them in this order). Deterministic by seed; the tufts are
    /// scattered with a minimum distance (no rows, no clumps of darts) and
    /// crowd in lush patches.
    pub fn grow(&self, region: &Mask, seed: u64) -> Vec<Tuft> {
        let f = region.f;
        let mut rng = Rng::new(seed ^ 0x5a4d);
        // bounds of the region
        let (mut bx0, mut by0, mut bx1, mut by1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for (i, &v) in region.data.iter().enumerate() {
            if v >= 0.5 {
                let (x, y) = (((i % f.w) + f.x0) as f32 / f.scale, ((i / f.w) + f.y0) as f32 / f.scale);
                bx0 = bx0.min(x);
                bx1 = bx1.max(x);
                by0 = by0.min(y);
                by1 = by1.max(y);
            }
        }
        if bx1 < bx0 {
            return vec![];
        }
        let by0 = by0.max(self.horizon + 1e-3);
        let smin = self.scale_at(by0).max(1e-3);
        let smax = self.scale_at(by1).max(smin);
        let cell = |s: f32| self.spacing * s.max(1e-3).powf(-self.thin);
        let cmin = cell(smax);
        let n = ((bx1 - bx0) * (by1 - by0) / (cmin * cmin) * 1.6) as usize;
        let lush_at = |x: f32, y: f32| {
            let p = 0.5 + 0.5 * (value_noise(x / self.patch_size, y / (self.patch_size * 0.4), seed ^ 0x11) * 0.7 + value_noise(x * 3.1 / self.patch_size, y * 3.1 / (self.patch_size * 0.4), seed ^ 0x13) * 0.3);
            (1.0 - self.patch + self.patch * 1.6 * p).clamp(0.0, 1.0)
        };
        // a hash grid for the minimum distance
        let g = cmin * 0.5;
        let gw = (((bx1 - bx0) / g).ceil() as usize).max(1) + 1;
        let gh = (((by1 - by0) / g).ceil() as usize).max(1) + 1;
        let mut grid: Vec<Vec<(f32, f32)>> = vec![vec![]; gw * gh];
        let mut tufts = vec![];
        for _ in 0..n {
            let (x, y) = (rng.range(bx0, bx1), rng.range(by0, by1));
            let u = rng.f();
            let s = self.scale_at(y);
            if s <= 0.0 || region.sample(x, y) < 0.5 {
                continue;
            }
            let lush = lush_at(x, y);
            let c = cell(s);
            if u > (cmin / c).powi(2) * (0.35 + 0.65 * lush) {
                continue;
            }
            let rmin = 0.55 * c;
            let (gx, gy) = (((x - bx0) / g) as isize, ((y - by0) / g) as isize);
            let k = (rmin / g).ceil() as isize;
            let mut near = false;
            'outer: for j in (gy - k).max(0)..=(gy + k).min(gh as isize - 1) {
                for i in (gx - k).max(0)..=(gx + k).min(gw as isize - 1) {
                    if grid[j as usize * gw + i as usize].iter().any(|q| (q.0 - x).powi(2) + (q.1 - y).powi(2) < rmin * rmin) {
                        near = true;
                        break 'outer;
                    }
                }
            }
            if near {
                continue;
            }
            let height = self.height * s * rng.range(0.6, 1.3) * (0.6 + 0.6 * lush);
            if height < self.smallest {
                continue;
            }
            grid[gy as usize * gw + gx as usize].push((x, y));
            let lean = self.wind.at(x, y) + rng.normal() * 0.12;
            let nb = ((rng.range(self.blades.0 as f32, self.blades.1 as f32 + 1.0)) * (0.35 + 0.65 * s.min(1.0))).floor().max(1.0) as usize;
            let dir = |a: f32| (a.sin(), -a.cos());
            let mut blades = vec![];
            let mut tallest = (0.0f32, (x, y - height));
            for b in 0..nb {
                let fan = if nb > 1 { (b as f32 / (nb - 1) as f32 - 0.5) * self.fan } else { 0.0 };
                let a = lean + fan + rng.normal() * 0.1;
                let len = height * rng.range(0.55, 1.0);
                let foot = (x + rng.normal() * height * 0.06, y);
                let d0 = dir(a * 0.7);
                let bend = a + self.curl * (a.signum() * a.abs().min(0.6) / 0.6) * rng.range(0.5, 1.2);
                let d1 = dir(bend);
                let ctrl = (foot.0 + d0.0 * len * 0.55, foot.1 + d0.1 * len * 0.55);
                let tip = (ctrl.0 + d1.0 * len * 0.5, ctrl.1 + d1.1 * len * 0.5);
                if len > tallest.0 {
                    tallest = (len, tip);
                }
                blades.push([foot, ctrl, tip]);
            }
            let bloom = 0.5 + 0.5 * value_noise(x / (self.patch_size * 0.6), y / (self.patch_size * 0.25), seed ^ 0xf1);
            let flower = (self.kinds > 0 && rng.chance(self.flowers * 2.5 * crate::smoothstep(0.45, 0.85, bloom))).then(|| {
                let kn = 0.5 + 0.5 * value_noise(x / (self.patch_size * 0.8), y / (self.patch_size * 0.3), seed ^ 0xf3);
                Flower { at: tallest.1, r: (height * rng.range(0.05, 0.09)).max(self.smallest * 0.3), kind: ((kn * self.kinds as f32) as u32).min(self.kinds - 1) }
            });
            tufts.push(Tuft { at: (x, y), scale: s, height, lean, blades, flower, lush });
        }
        tufts.sort_by(|a, b| a.at.1.total_cmp(&b.at.1));
        tufts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_and_connected() {
        for h in [Habit::oak(), Habit::dead_oak(), Habit::birch(), Habit::spruce()] {
            let a = h.grow((500.0, 900.0), 400.0, 7);
            let b = h.grow((500.0, 900.0), 400.0, 7);
            assert_eq!(a.limbs.len(), b.limbs.len());
            assert!(a.limbs.len() > 20, "too few limbs: {}", a.limbs.len());
            let top = a.bounds().1;
            assert!((900.0 - top - 400.0).abs() < 1.0, "height {}", 900.0 - top);
            for (i, l) in a.limbs.iter().enumerate() {
                assert!(l.w.iter().all(|w| w.is_finite() && *w > 0.0));
                if let Some(p) = l.parent {
                    assert!(p < i, "parents come first");
                    if !l.root {
                        // a limb springs from a point on its parent
                        let q = a.limbs[p].pts[l.at];
                        assert!(dist(q, l.pts[0]) < 1e-3 || l.broken, "limb {i} detached");
                    }
                }
            }
        }
    }

    /// A dying tree as the grower left it (what `skeleton` starts from).
    fn declined(h: &Habit, seed: u64) -> Grower<'_> {
        let mut g = Grower::new(h, seed).run();
        let max_y = g.live().map(|i| g.nodes[i].p.y).fold(1e-3, f32::max);
        g.decline(max_y);
        g
    }

    /// Every exported segment carries its internode's live/dead state: the
    /// stag-headed top of a live leader is dead wood.
    #[test]
    fn limbs_keep_the_dead_wood_along_them() {
        let h = Habit::dead_oak();
        let mut part_dead = 0;
        for seed in [2, 7, 11] {
            let g = declined(&h, seed);
            let chains = g.chains();
            let s = h.grow((0.0, 0.0), 400.0, seed);
            for (i, c) in chains.iter().enumerate() {
                let l = &s.limbs[i];
                assert_eq!(l.pts.len(), c.nodes.len());
                for k in 0..c.nodes.len() - 1 {
                    assert_eq!(l.dead_at(k), g.nodes[c.nodes[k + 1]].dead, "seed {seed} limb {i} segment {k}");
                }
                assert_eq!(l.dead, l.dead_from == 0);
                part_dead += (l.dead_from > 0 && l.dead_from < l.pts.len()) as usize;
            }
            if seed == 7 {
                // the reviewer's case: 20 of the leader's 55 nodes are dead
                let trunk = &s.limbs[0];
                assert!(!trunk.dead && trunk.dead_from < trunk.pts.len(), "{} of {}", trunk.dead_from, trunk.pts.len());
                assert_eq!(trunk.pts.len() - 1 - trunk.dead_from, (1..chains[0].nodes.len()).filter(|&k| g.nodes[chains[0].nodes[k]].dead).count());
            }
        }
        assert!(part_dead > 0);
    }

    /// Dead wood keeps claws of at most two internodes past the width set
    /// by `decay`: no side shoots survive on a claw.
    #[test]
    fn dead_claws_are_short() {
        let h = Habit::dead_oak();
        for seed in [2, 7, 11, 23, 54, 61] {
            let g = declined(&h, seed);
            let thin = 1.0 + h.decay * h.decay * 0.025 * g.nodes[0].wood;
            let is_thin_dead = |i: usize| g.nodes[i].dead && g.nodes[i].wood < thin;
            let mut deepest = 0;
            for i in g.live().filter(|&i| i > 0 && is_thin_dead(i) && !is_thin_dead(g.nodes[i].parent)) {
                let mut stack = vec![(i, 1)];
                while let Some((j, d)) = stack.pop() {
                    deepest = deepest.max(d);
                    stack.extend(g.nodes[j].kids.iter().filter(|&&k| g.nodes[k].alive).map(|&k| (k, d + 1)));
                }
            }
            assert!(deepest <= 2, "seed {seed}: a thin dead subtree {deepest} deep");
        }
    }

    #[test]
    fn pipe_model_widths() {
        let s = Habit::oak().grow((0.0, 0.0), 300.0, 3);
        let trunk = &s.limbs[0];
        assert!(trunk.w[0] > 300.0 * 0.07);
        // children are never wider than the parent where they spring
        for l in s.limbs.iter().skip(1).filter(|l| !l.root) {
            let p = &s.limbs[l.parent.unwrap()];
            assert!(l.w[0] <= p.w[l.at] * 1.3 + 1e-3, "{} > {} (order {} broken {} dead {} at {}/{} parent order {})", l.w[0], p.w[l.at], l.order, l.broken, l.dead, l.at, p.pts.len(), p.order);
        }
    }
    /// Leaves grow on live young wood only, spread over the crown with sky
    /// between them, and the sunward side of the crown is lit.
    #[test]
    fn foliage_on_live_young_wood() {
        let f = Frame::new(500, 500, 0.5);
        for h in [Habit::oak(), Habit::birch(), Habit::beech(), Habit::alder(), Habit::willow()] {
            let sk = h.grow((500.0, 950.0), 700.0, 5);
            let sun = (-0.7, -0.6, 0.4);
            let fo = sk.foliage(sun, 3);
            let fo2 = sk.foliage(sun, 3);
            assert_eq!(fo.clumps.len(), fo2.clumps.len());
            assert!(fo.clumps.len() > 40, "{:?}: {} clumps", h.leaf, fo.clumps.len());
            for c in &fo.clumps {
                let l = &sk.limbs[c.limb];
                assert!(!l.dead && !l.root);
                assert!((0.0..=1.0).contains(&c.lit) && (0.0..=1.0).contains(&c.shade));
            }
            // sunward (left, upper) clumps catch more light than the far side
            let cx = fo.clumps.iter().map(|c| c.at.0).sum::<f32>() / fo.clumps.len() as f32;
            let side = |left: bool| {
                let v: Vec<f32> = fo.clumps.iter().filter(|c| (c.at.0 < cx) == left).map(|c| c.lit).collect();
                v.iter().sum::<f32>() / v.len().max(1) as f32
            };
            assert!(side(true) > side(false) + 0.05, "{} vs {}", side(true), side(false));
            // leaves with sky between them
            let m = fo.mask(f);
            let env = m.clone().dilate(fo.grain() * 1.5).erode(fo.grain() * 1.5);
            let (leaf, hull) = (m.data.iter().sum::<f32>(), env.data.iter().sum::<f32>());
            let open = 1.0 - leaf / hull;
            assert!(open > 0.02 && open < 0.6, "{:?}: sky through the crown {open}", h.leaf);
            let lit = fo.lit(f);
            assert!(lit.data.iter().zip(&m.data).all(|(l, m)| *l <= m + 1e-6));
        }
        // a dead oak's dead limbs are bare; winter is bare
        let sk = Habit::dead_oak().grow((500.0, 950.0), 700.0, 7);
        let fo = sk.foliage((-1.0, -1.0, 0.3), 1);
        assert!(fo.clumps.iter().all(|c| !sk.limbs[c.limb].dead));
        assert!(sk.foliage_with(&Leafing::none(), (0.0, -1.0, 0.0), 1).clumps.is_empty());
    }

    /// Tufts recede: smaller and fewer per area toward the horizon, far
    /// ones first, leaning with the wind.
    #[test]
    fn sward_recedes() {
        let f = Frame::new(500, 333, 0.5);
        let region = Mask::from_fn(f, |_, y| if y > 420.0 { 1.0 } else { 0.0 });
        let sw = Sward { horizon: 400.0, near: 666.0, wind: Wind { lean: 0.3, gust: 0.05, period: 100.0, seed: 2 }, ..Sward::default() };
        let t = sw.grow(&region, 9);
        let t2 = sw.grow(&region, 9);
        assert_eq!(t.len(), t2.len());
        assert!(t.len() > 500, "{}", t.len());
        assert!(t.windows(2).all(|w| w[0].at.1 <= w[1].at.1), "far first");
        let band = |y0: f32, y1: f32| {
            let v: Vec<&Tuft> = t.iter().filter(|q| q.at.1 >= y0 && q.at.1 < y1).collect();
            (v.len() as f32 / (y1 - y0), v.iter().map(|q| q.height).sum::<f32>() / v.len().max(1) as f32)
        };
        let (far_n, far_h) = band(440.0, 480.0);
        let (near_n, near_h) = band(610.0, 650.0);
        assert!(far_h < 0.5 * near_h, "heights {far_h} {near_h}");
        assert!(far_n < near_n, "marks per area {far_n} {near_n}");
        let mean_lean = t.iter().map(|q| q.lean).sum::<f32>() / t.len() as f32;
        assert!((mean_lean - 0.3).abs() < 0.1, "{mean_lean}");
        assert!(t.iter().any(|q| q.flower.is_some()));
        assert!(t.iter().all(|q| q.height >= sw.smallest && !q.blades.is_empty()));
    }
}

