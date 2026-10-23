//! Oil films that cure from the top down, and the skin that wrinkles.
//!
//! A film of oil paint takes up oxygen at its surface. Where the paint is
//! thin, the whole film oxidizes together and `drying` tells the story:
//! open, setting, tacky, touch-dry. Where it is thick, the surface cures
//! into a skin while the paint under it stays soft for days or months: an
//! 80 µm alkyd film has a mobility gradient from its air side down
//! (Mallégol et al. 2002, NMR depth profiles), a 1.5 mm titanium-white
//! disk takes up almost no oxygen for three weeks and a tube bead of raw
//! sienna is still wet inside after a year (Golden, "Weighing In on the
//! Drying of Oils"). That skin is the material that wrinkles: Ranquet et
//! al. (2023, Nat. Commun.) saw wrinkles on 1 mm lead-white films whose
//! paint had a yield stress of 58–75 Pa and none on one at 3200 Pa, and
//! attribute them to the oxidizing skin expanding over soft oil. A skin
//! under compression on a soft foundation buckles (Huang and Suo 2002,
//! J. Appl. Phys.; Allen 1969 for an elastic foundation): the softer the
//! paint under it, the smaller the compression that does it.
//!
//! Material revision 1 of engine 9 (`Canvas::with_material`) keeps, for
//! every film that has baked into the dry picture (`drying::bake`), a
//! `Packet`: its thickness, how deep its cure front has reached, the
//! compression its skin has built and how far wrinkling has moved its
//! soft bulk. Covering a film doesn't end it: the packet stays under the
//! new one, its front slowed by the cover (oxygen has to get through), its
//! bulk as soft as it was. None of this changes the surface endpoints
//! `drying` reports (touch-dry stays touch-dry); it changes the relief,
//! and only where a skin lies on paint that is still soft.
//!
//! **The front.** Oxygen diffuses through the cured skin to the paint
//! under it, where it is consumed: a moving front, as in the oxidation of
//! silicon (Deal and Grove 1965). With `u` the depth of material the
//! oxygen crosses (the packet's own skin plus whatever lies on it),
//! `du/dt = v0 / (1 + u / OXYGEN_UM)`: `v0` is the rate an uncovered
//! surface cures through at (reaction-limited, from the paint's own
//! touch-dry calendar: `THROUGH`), `OXYGEN_UM` the depth of skin that
//! halves it. It integrates exactly: `u + u² / 2 OXYGEN_UM` grows linearly
//! in time.
//!
//! **The skin's compression.** Skin that has just cured keeps oxidizing
//! and expands by a free strain `EXPAND × binder share` over `EXPAND_MIN`
//! minutes (the mass of a film keeps rising for days after it is
//! touch-dry, Golden). The skin is held by the film around it, so the
//! expansion becomes a compressive membrane force `n` (N/m, positive in
//! compression). Material that cures later joins the skin stress-free: the
//! packet keeps the thickness `x` of skin that hasn't expanded yet, and
//! `n` grows as that reserve expands. Shrinkage would be tension, and
//! tension doesn't wrinkle: the sign is the hypothesis Ranquet's
//! observations support, with an unmeasured magnitude (`EXPAND` is an
//! estimate; no measurement of the skin's strain exists).
//!
//! **The wrinkle.** Where a skin of thickness `t` (every cured layer from
//! the surface down to the first soft one) lies on soft bulk of depth
//! `H`, the skin is a plate of bending rigidity `B = E t³ / 12 (1 − ν²)`
//! and extensional stiffness `C = E t / (1 − ν²)` under the compression
//! `N` of its stacked packets, and the bulk is a gel of shear modulus
//! `G` (`MODULUS_PER_YIELD` times its yield stress, as a paste yields at
//! a few percent strain). A sinusoid of wavenumber `q` on the plate is
//! held down by the gel with a stiffness `2 G q` when the bulk is deep
//! (a half-space) and `12 G / H³ q²` when it is thinner than the
//! wavelength (bulk sheared between the skin and the paint under it, as
//! in lubrication); the plate buckles where `N q² > B q⁴` plus that, and
//! the wavenumber that needs the least compression, `q_c`, is where the
//! wrinkle forms: on deep bulk `(G / B)^(1/3)` at `N_c = 3 B^(1/3)
//! G^(2/3)` (Allen 1969; Huang, Hong and Suo 2005 in their notation), on
//! thin bulk `(24 G / B H³)^(1/6)`. The threshold is a strain,
//! `N_c / C`: about 0.1% on deep medium-rich paint (10 Pa yield) and
//! 4.4% on stiff tube paint (3000 Pa), against the 1.35% a tube-paint
//! skin reaches (`EXPAND`): medium-rich paint wrinkles, tube paint
//! straight from the tube holds, as Ranquet's two films did. The wrinkle
//! is drawn at the band of wavelengths nearest `q_c` (`BAND_LAMBDA`),
//! and the threshold is the plate's at the wavenumber drawn. The gel
//! relaxes in a fraction of a second and the compression builds over
//! days, so the wrinkle is always at its equilibrium: past the threshold
//! the plate buckles until the slopes it has made have taken up the
//! compression, `C ⟨|∇w|²⟩ / 2 = N − N_c` (von Kármán, averaged over a
//! wavelength). Each tick the skin asks for that much more mean square
//! slope, `Δ`, and no more than a wrinkle as deep as its soft bulk has
//! (a trough stops at the paint under it). A wavelength of skin acts
//! together: what a pixel asks for counts averaged over its wavelength,
//! with the paint around it that is cured through, or bare, asking
//! nothing, so the skin over a soft spot smaller than a wavelength is
//! held by the cured paint around it; over a soft pixel with a
//! cured-through neighbor it asks nothing of its own, and moves with the
//! wrinkle around it. The pattern it gets is the relief it cured over,
//! band-passed at its own wavelength (a difference of box blurs: the
//! brush marks are the imperfections a wrinkle grows from, and it lies
//! along them; a level film has `SEED_UM` of roughness to start from) and
//! scaled to unit rms slope, and the bulk takes `a` of it such that
//! `(ρ + a)² − ρ² = Δ`, with `ρ` the pattern's projection on the slopes
//! already there (so a wrinkle that is there deepens, a new wavelength
//! adds in quadrature). The slopes the skin got relieve the compression
//! of each of its layers by that layer's share of `C`; a layer that
//! cures later joins in the shape it finds, stress-free, so a wrinkle
//! never comes back out: the bulk's displacement `w` is the packet's
//! memory of it. The bulk moves sideways under the skin: the surface
//! rises where it gathers, sinks where it leaves, never below the paint
//! under it and always the same total, so no pigment is made or lost.
//! The plate's thickness is what lies above each packet's front, and when
//! the front reaches the bottom the soft bulk is gone and the wrinkle
//! stays as it is. The surface tension of the oil counts against the
//! compression (`surface::SIGMA`). A wrinkle whose wavelength is under
//! two pixels is below the picture's resolution and isn't drawn.
//!
//! A skin that thickens under compression wrinkles more than one of its
//! final thickness would: the thin early skin buckles at large slopes for
//! little compression, and the material that cures after it locks them
//! in. A 600 µm medium-rich film ends with slopes of about 0.25 rms
//! (some 100 µm rms of relief at the brush marks' spacing); a 1 mm one
//! whose skin forms faster, 0.15.
//!
//! **The clock.** Fronts, skins and wrinkles step every six hours of the
//! painting's clock (`TICK_MIN`), never at a wait's own ends: a wait
//! split into any parts leaves the same relief as the whole. A tick draws
//! wrinkles only when a wavelength's worth of skin asks for `GROW` of
//! slope or more, and a pass settles every wavelength that asks, so the
//! slow creep of a skin that has long since wrinkled costs a pass over
//! the packets, not the blurs: a wait of months draws its wrinkles in a
//! handful of passes.
//!
//! Engines 1 to 8 and engine-9 logs without `--@ material` have no
//! packets and paint as they did. The coefficients below are estimates
//! from the sources named; none is a measured constant of a named paint.

use crate::canvas::Canvas;
use crate::drying::{GEL, Pace, lean_of, rate};
use crate::surface::{COAT_UM, SIGMA, rheology_at, vnoise};
use rayon::prelude::*;

/// The material revision new engine-9 paintings are painted with
/// (`Canvas::with_material`): 0 is the single-cure film engine 9 was
/// released with, 1 the film cured from the top of this module.
pub const MATERIAL: u32 = 1;

/// Depth of cured skin that halves the rate a front advances at, µm:
/// oxygen crossing a thicker skin arrives slower. Estimate from Golden's
/// no-drier titanium white: 76 µm drawdowns touch-dry by day 2 and 254 µm
/// at days 4–6, while 1.5 mm disks gained almost no mass in three weeks,
/// so a front moving some 25 µm a day in thin films reaches 150–200 µm in
/// 21 days; with `u + u²/2ℓ = v0 t` that is ℓ of 30–60 µm, with the thick
/// disk's own uncertainty read generously.
pub const OXYGEN_UM: f32 = 80.0;
/// How far a one-coat film has cured through when its surface is
/// touch-dry, as a share of its thickness: an 80 µm alkyd film was
/// surface-dry at 6.5–7 h and through-dry at 11–12 h (Mallégol et al.
/// 2002, Table 1), so a front crosses it at about 0.6 of its surface
/// time. Estimate; the engine's own touch-dry calendar stays as it is.
pub const THROUGH: f32 = 0.6;
/// Young's modulus of fresh skin, Pa. Ranquet's hardness assay rises two
/// decades from under 0.01 N/mm² to its plateau as the film forms; young
/// oil films measure a few MPa to tens of MPa in tension. Estimate: with
/// the gel's modulus it sets the strain a skin wrinkles at and the
/// wavelength it takes (`(B / G)^(1/3)`).
pub const SKIN_PA: f32 = 5.0e6;
/// Poisson's ratio of the skin (a rubbery network).
const NU: f32 = 0.4;
/// Free expansion of a fully oxidized skin per unit of its binder's share
/// of the paint's volume: tube paint at 54% oil expands 1.35%, an oily
/// glaze at 85% about 2.1%. Estimate: Ranquet attributes the wrinkles to
/// the skin's expansion but doesn't measure it, and a drying oil's mass
/// gain (10–15%) comes with a rise in density, so the net swelling is
/// small; this sits between the strain that wrinkles Ranquet's 58–75 Pa
/// films (0.3–0.4%) and the one that would wrinkle the 3200 Pa film
/// (4.6%), with room on both sides. It sets the wrinkle's wavelength
/// (`2π √(2 B / N)`), not its amplitude.
pub const EXPAND: f32 = 0.025;
/// Minutes over which a freshly cured skin takes up its expansion: a
/// film's mass keeps rising for days after it is touch-dry (Golden). Two
/// days.
const EXPAND_MIN: f32 = 2.0 * 24.0 * 60.0;
/// The gel's shear modulus as a multiple of its yield stress: a paste
/// yields at a few percent strain (3–10% for yield-stress fluids), so its
/// modulus is 10–30 times its yield stress. 20.
pub const MODULUS_PER_YIELD: f32 = 20.0;
/// The clock the fronts and wrinkles step on: every six hours of the
/// painting's clock. A front moves a fraction of a micrometer an hour
/// and a skin's compression builds over days, so the step loses nothing
/// the picture shows; a 90-day wait is 360 steps over the packets.
pub const TICK_MIN: f64 = 360.0;
/// Bulk thinner than this, µm, is cured through.
const DRY_UM: f32 = 0.05;
/// Depth of bulk, in wavenumbers (`q H`), past which the gel under a skin
/// answers as a half-space (`2 G q`) rather than a thin layer sheared
/// between the skin and the paint under it (`12 G / H³ q²`): 6^(1/3).
const DEEP: f32 = 1.817;
/// Wavelength of a wrinkle, px, over the radius of the box blur that
/// draws its pattern: the relief blurred twice at that radius, less the
/// relief blurred twice at double it, is a difference of Gaussians whose
/// band peaks at that wavelength.
const BAND_LAMBDA: f32 = 5.3;
/// The widest band, px (a wavelength of 85 px: 25 mm at the native
/// picture; skins on soft bulk don't get there).
const BAND_MAX: usize = 16;
/// The least growth of a wrinkle's rms slope, were it starting from
/// level, that is drawn in one tick (6°, some 35 µm of amplitude at a
/// 2 mm wavelength); slower growth is saved up for a tick that has this
/// much, and a pass settles every wavelength that asks at all, so a skin
/// that creeps costs a pass over its packets, not the blurs, and a wait
/// of months draws its wrinkles in a handful of passes.
const GROW: f32 = 0.1;
/// Micro-roughness every surface has, µm, at the wrinkle's own wavelength,
/// so a skin cured over a perfectly level film still has an imperfection
/// to buckle from (a deterministic noise, so a replay is a replay).
const SEED_UM: f32 = 0.2;
const NONE: u32 = u32::MAX;

/// One baked film at one pixel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Packet {
    /// Thickness it baked at, µm (the material; the bulk wrinkling has
    /// redistributed is counted in `w`).
    pub h: f32,
    /// Depth of its cure front below its top, µm: the skin. `s ≥ h + w`:
    /// cured through.
    pub s: f32,
    /// Skin that hasn't taken up its expansion yet, µm.
    pub x: f32,
    /// Compressive membrane force of its skin, N/m: what its expansion
    /// has built, less what the wrinkles it has taken have relieved.
    pub n: f32,
    /// How much bulk wrinkling has gathered here (or taken away, < 0),
    /// µm: the film's thickness is `h + w`.
    pub w: f32,
    /// Minutes on the clock when its surface gelled (its front starts then).
    pub born: f32,
    /// Rate its front advances at uncovered, µm per minute.
    pub v0: f32,
    /// Stiffness of the paint (`wet::Prop`'s): its bulk's viscosity and
    /// yield stress (`surface::rheology_at`).
    pub stiff: f32,
    /// Share of its volume that is binder: its skin's expansion.
    pub binder: f32,
    /// The packet under it, or `NONE`.
    pub below: u32,
}

/// Every pixel's baked films, top down.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Film {
    /// The top packet of each buffer pixel (`NONE`: bare, or every film
    /// cured through), empty until the first film bakes.
    pub top: Vec<u32>,
    pub pool: Vec<Packet>,
    free: Vec<u32>,
    /// Packets in use.
    pub live: usize,
    /// The clock minute of the last tick.
    pub ticked: f64,
}

impl Film {
    fn push(&mut self, p: Packet) -> u32 {
        self.live += 1;
        match self.free.pop() {
            Some(i) => {
                self.pool[i as usize] = p;
                i
            }
            None => {
                self.pool.push(p);
                (self.pool.len() - 1) as u32
            }
        }
    }

    /// The packets of pixel `i`, top down.
    pub(crate) fn column(&self, i: usize) -> impl Iterator<Item = &Packet> {
        let mut at = self.top.get(i).copied().unwrap_or(NONE);
        std::iter::from_fn(move || {
            if at == NONE {
                return None;
            }
            let p = &self.pool[at as usize];
            at = p.below;
            Some(p)
        })
    }

    /// Rebuild from per-pixel columns (top down), as a checkpoint stores them.
    pub(crate) fn from_columns(columns: Vec<Vec<Packet>>, ticked: f64) -> Film {
        let mut f = Film { top: vec![NONE; columns.len()], pool: Vec::new(), free: Vec::new(), live: 0, ticked };
        for (i, column) in columns.into_iter().enumerate() {
            let mut below = NONE;
            for p in column.into_iter().rev() {
                below = f.push(Packet { below, ..p });
            }
            f.top[i] = below;
        }
        f
    }
}

/// What the wrinkle does at one pixel this tick.
#[derive(Clone, Copy)]
struct Site {
    /// The packet whose bulk moves (`NONE`: nothing to do here).
    at: u32,
    /// How much the mean square slope of the wrinkle has to grow by
    /// (negative: how far the skin is from wrinkling).
    delta: f32,
    /// The band its wavelength is drawn in (blur radius, px).
    band: u8,
    /// The least displacement the bulk allows, µm (none left under the skin).
    floor: f32,
}

/// Minutes to touch-dry of one coat of the paint `hd`, as `drying` has it.
fn one_coat_minutes(engine: u32, drain: bool, hd: &crate::wet::Prop) -> f32 {
    Pace::of(engine).to_dry(0.0, rate(1.0, lean_of(drain, hd), hd[2])).max(1.0)
}

thread_local! {
    /// The pass's buffers, kept between ticks: a pass over the picture
    /// allocates nothing.
    static SCRATCH: std::cell::RefCell<Vec<Vec<f32>>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// `n` empty buffers from the scratch (with their capacity), to `give` back.
fn take(n: usize) -> Vec<Vec<f32>> {
    let mut s = SCRATCH.with(|s| std::mem::take(&mut *s.borrow_mut()));
    s.resize_with(n, Vec::new);
    s.truncate(n);
    for b in &mut s {
        b.clear();
    }
    s
}

fn give(s: Vec<Vec<f32>>) {
    SCRATCH.with(|c| *c.borrow_mut() = s);
}

/// The binder's share of a paint's volume: its tubes' oil by volume, with
/// the medium added to it (`wet::Prop`'s oil, relative to tube paint).
fn binder_share(hd: &crate::wet::Prop) -> f32 {
    let (ov, oil) = (hd[8].clamp(0.05, 0.95), hd[4].max(0.0));
    (ov * oil / (1.0 - ov + ov * oil)).clamp(0.0, 1.0)
}

impl Canvas {
    /// Paint with material revision `rev` (`MATERIAL`; an easel log's
    /// `--@ material`): engine 9 only. 0 is the single-cure film engine 9
    /// was released with; a log without the line replays as it was.
    pub fn with_material(mut self, rev: u32) -> Self {
        self.material = if self.engine >= crate::PIGMENT_MIXER { rev.min(MATERIAL) } else { 0 };
        self
    }
    pub fn material(&self) -> u32 {
        self.material
    }

    /// Whether this canvas keeps its baked films as packets.
    #[inline]
    pub(crate) fn skins(&self) -> bool {
        self.material >= 1 && self.engine >= crate::PIGMENT_MIXER
    }

    /// A film `add_um` thick of cure `cure` bakes at pixel `i` (the clock
    /// will read `end` when this step is over): keep it as a packet on top
    /// of what's there. Its front starts when its surface gelled: back by
    /// the cure past the gel point at the set rate, or, dried before its
    /// gel point (`dry`), ahead by the open time it had left.
    pub(crate) fn skin_bake(&mut self, i: usize, add_um: f32, cure: f32, end: f64) {
        if !self.skins() || add_um <= 0.0 {
            return;
        }
        let n = self.f.w * self.f.h;
        if self.skin.top.len() != n {
            self.skin.top = vec![NONE; n];
            self.skin.ticked = (end / TICK_MIN).floor() * TICK_MIN - TICK_MIN;
        }
        let hd = self.wet.hide[i];
        let (pace, drain) = (Pace::of(self.engine), self.drain);
        let r = rate(self.wet.clock.px.get(i).map_or(0.0, |p| p.th), lean_of(drain, &hd), hd[2]);
        let born = if cure >= GEL { end - ((cure - GEL) / pace.set_rate(r)) as f64 } else { end + ((GEL - cure) / pace.open_rate(r)) as f64 };
        let v0 = THROUGH * COAT_UM / one_coat_minutes(self.engine, drain, &hd);
        let below = self.skin.top[i];
        let p = Packet { h: add_um, s: 0.0, x: 0.0, n: 0.0, w: 0.0, born: born as f32, v0, stiff: hd[1], binder: binder_share(&hd), below };
        let at = self.skin.push(p);
        self.skin.top[i] = at;
    }

    /// The clock moves from `start` to `end` minutes: step the fronts, the
    /// skins and the wrinkles at every whole hour in between.
    pub(crate) fn skin_advance(&mut self, start: f64, end: f64) {
        if !self.skins() || self.skin.live == 0 || end <= start {
            return;
        }
        let first = (start / TICK_MIN).floor() + 1.0;
        let last = (end / TICK_MIN).floor();
        let mut k = first;
        while k <= last {
            self.skin_tick(k * TICK_MIN);
            k += 1.0;
        }
    }

    /// Draw this tick's wrinkles. At each wavelength in use, the relief
    /// band-passed there and scaled to unit rms slope is the pattern; each
    /// pixel's bulk takes as much of it as brings the slopes to what its
    /// skin asks for, counting the slopes it already has, and the
    /// compression of every layer of its plate is relieved by what it
    /// took. `site` covers the box `bx` (buffer pixels); the pass works
    /// over `pb`, that box padded by the widest window.
    fn wrinkle(&mut self, site: &[Site], bx: (usize, usize, usize, usize), pb: (usize, usize, usize, usize), bands: Vec<u8>) {
        let w = self.f.w;
        let (pw, ph) = (pb.2 - pb.0, pb.3 - pb.1);
        let (bw, bh) = (bx.2 - bx.0, bx.3 - bx.1);
        let seed = 0x5a1d_c0de ^ self.f.full_w as u64;
        let dx_um = self.px_mm() * 1e3;
        let soft: Vec<usize> = (0..bw * bh).filter(|&k| site[k].at != NONE).collect();
        let padded = |k: usize| (bx.1 + k / bw - pb.1) * pw + bx.0 + k % bw - pb.0;
        let mut scratch = take(19);
        let mut buffers = scratch.drain(..);
        let mut buffer = |n: usize| {
            let mut b = buffers.next().unwrap();
            b.resize(n, 0.0);
            b
        };
        // the wrinkle already there (the soft bulk's displacement) and the
        // relief the skin cured over, without it, over the padded box
        let mut z = buffer(pw * ph);
        z.par_iter_mut().enumerate().for_each(|(k, z)| *z = self.height[(pb.1 + k / pw) * w + pb.0 + k % pw]);
        let mut wf = buffer(pw * ph);
        let mut need = buffer(pw * ph);
        let mut skin = buffer(pw * ph);
        for &k in &soft {
            let p = padded(k);
            wf[p] = self.skin.pool[site[k].at as usize].w;
            z[p] -= wf[p];
            need[p] = site[k].delta;
            skin[p] = 1.0;
        }
        let gradient = |v: &[f32], gx: &mut [f32], gy: &mut [f32]| {
            gx.par_chunks_mut(pw).zip(gy.par_chunks_mut(pw)).enumerate().for_each(|(y, (gx, gy))| {
                for x in 0..pw {
                    let at = |x: usize, y: usize| v[y.min(ph - 1) * pw + x.min(pw - 1)];
                    gx[x] = (at(x + 1, y) - at(x.saturating_sub(1), y)) / (2.0 * dx_um);
                    gy[x] = (at(x, y + 1) - at(x, y.saturating_sub(1))) / (2.0 * dx_um);
                }
            });
        };
        // the pattern only sees the skin: pixels without one are missing,
        // not a step down to the ground (a normalized convolution), and
        // slopes count only where every neighbor has a skin too
        let mut inner = buffer(pw * ph);
        inner.par_chunks_mut(pw).enumerate().for_each(|(y, row)| {
            for (x, v) in row.iter_mut().enumerate() {
                let at = |x: usize, y: usize| x < pw && y < ph && skin[y * pw + x] > 0.0;
                *v = (at(x, y) && at(x.wrapping_sub(1), y) && at(x + 1, y) && at(x, y.wrapping_sub(1)) && at(x, y + 1)) as u8 as f32;
            }
        });
        let (mut wx, mut wy) = (buffer(pw * ph), buffer(pw * ph));
        gradient(&wf, &mut wx, &mut wy);
        let mut dw = buffer(pw * ph);
        // the pixels the pass touches: the amount of the pattern each takes
        // and whether the wrinkle relieves its compression
        let mut took: Vec<(usize, f32, bool)> = Vec::new();
        let (mut tmp, mut mr, mut m2r, mut mi, mut near, mut far, mut rho) = (buffer(0), buffer(0), buffer(0), buffer(0), buffer(0), buffer(0), buffer(0));
        let (mut zr, mut px, mut py, mut dot) = (buffer(pw * ph), buffer(pw * ph), buffer(pw * ph), buffer(pw * ph));
        drop(buffers);
        for &band in &bands {
            let r = band as usize;
            let mut blur2 = |v: &[f32], r: usize, out: &mut Vec<f32>| {
                crate::surface::box_blur_into(v, pw, ph, r, &mut tmp);
                crate::surface::box_blur_into(&tmp, pw, ph, r, out);
            };
            let over = |num: &mut [f32], den: &[f32]| num.par_iter_mut().zip(den).for_each(|(n, d)| *n = if *d > 1e-6 { *n / d } else { 0.0 });
            blur2(&skin, r, &mut mr);
            blur2(&skin, 2 * r, &mut m2r);
            blur2(&inner, 2 * r, &mut mi);
            let cell = BAND_LAMBDA * r as f32 / 2.0;
            zr.par_chunks_mut(pw).enumerate().for_each(|(y, row)| {
                for (x, v) in row.iter_mut().enumerate() {
                    let k = y * pw + x;
                    let (gx, gy) = ((pb.0 + x + self.f.x0) as f32, (pb.1 + y + self.f.y0) as f32);
                    *v = skin[k] * (z[k] + SEED_UM * (vnoise(gx / cell, gy / cell, seed + r as u64) - 0.5));
                }
            });
            blur2(&zr, r, &mut near);
            over(&mut near, &mr);
            blur2(&zr, 2 * r, &mut far);
            over(&mut far, &m2r);
            near.par_iter_mut().zip(&far).for_each(|(n, f)| *n -= f);
            // the pattern's slopes: their mean square over a wavelength
            // (its scale), then its projection on the wrinkle already there
            gradient(&near, &mut px, &mut py);
            dot.par_iter_mut().zip(&px).zip(&py).zip(&inner).for_each(|(((d, x), y), m)| *d = m * (x * x + y * y));
            blur2(&dot, 2 * r, &mut far);
            over(&mut far, &mi);
            // (no slope within a window of the pattern, or no interior to
            // measure one on: nothing to scale, and the bulk there stays)
            far.par_iter_mut().for_each(|f| *f = if *f > 1e-12 { f.sqrt() } else { 0.0 });
            dot.par_iter_mut().zip(&px).zip(&py).zip(&wx).zip(&wy).zip(&far).zip(&inner).for_each(|((((((d, px), py), wx), wy), f), m)| *d = if *f > 0.0 { m * (px * wx + py * wy) / f } else { 0.0 });
            blur2(&dot, 2 * r, &mut rho);
            over(&mut rho, &mi);
            // a wrinkle is a wavelength's worth of skin acting together:
            // what the skin asks for, averaged over one (the cured paint
            // around a soft spot asks nothing, and holds it; paint short
            // of its threshold holds it too). Where that average is
            // positive the skin wrinkles, and the wrinkle relieves every
            // pixel within a wavelength of it that is past its threshold,
            // down to its threshold: one pass settles the wavelength (the
            // pixels short of their threshold keep their compression, so
            // the plate ends a little under the von Kármán balance where
            // soft and held paint interleave)
            blur2(&need, 2 * r, &mut dot);
            m2r.par_iter_mut().zip(&dot).zip(&skin).for_each(|((m, d), s)| *m = (*d > 0.0 && *s > 0.0) as u8 as f32);
            blur2(&m2r, 2 * r, &mut mr);
            // a sinusoid of unit rms slope has an amplitude of λ √2 / 2π:
            // the pattern is bounded by it, so the wall of a stroke at the
            // skin's edge is not a wrinkle's worth of relief
            let cap = BAND_LAMBDA * r as f32 * dx_um * std::f32::consts::FRAC_1_SQRT_2 / std::f32::consts::PI;
            for &k in &soft {
                let s = site[k];
                if s.band != band {
                    continue;
                }
                let p = padded(k);
                let delta = dot[p];
                let relieved = s.delta > 0.0 && mr[p] > 0.0;
                if delta <= 0.0 && !relieved {
                    continue;
                }
                // slopes add: (ρ + a)² − ρ² = Δ for the amount a of the
                // unit-slope pattern that brings the mean square slope up by
                // Δ, with ρ its projection on the slopes there; the wrinkle
                // deepens in the shape it has, so a runs the way ρ points (a
                // pixel with no pattern within its window, at the skin's
                // edge, stays where it is)
                let a = if delta > 0.0 && far[p] > 0.0 { ((rho[p] * rho[p] + delta).sqrt() - rho[p].abs()).copysign(rho[p]) } else { 0.0 };
                dw[p] = if a != 0.0 { a * (near[p] / far[p]).clamp(-cap, cap) } else { 0.0 };
                took.push((k, a, relieved));
            }
        }
        if !took.is_empty() {
            // the bulk only moves sideways: what the crests gather the
            // troughs gave, and a trough stops at the paint under it (the
            // shift that keeps the total is level, and changes no slope)
            let mean = (took.iter().map(|&(k, ..)| dw[padded(k)] as f64).sum::<f64>() / took.len() as f64) as f32;
            let mut surplus = 0.0f64;
            let mut free = 0usize;
            for &(k, ..) in &took {
                let p = padded(k);
                dw[p] -= mean;
                let least = site[k].floor - wf[p];
                if dw[p] < least {
                    surplus += (least - dw[p]) as f64;
                    dw[p] = least;
                } else {
                    free += 1;
                }
            }
            if surplus > 0.0 && free > 0 {
                let share = (surplus / free as f64) as f32;
                for &(k, ..) in &took {
                    let p = padded(k);
                    if dw[p] > site[k].floor - wf[p] {
                        dw[p] -= share;
                    }
                }
            }
            // the wrinkle relieves every layer of the plate over the pixels
            // it reaches, each by its share of the plate's stiffness (a
            // trough cut short at the paint under it is credited in full:
            // the skin there is held a little less than it would be)
            for &(k, _, relieved) in &took {
                let s = site[k];
                let i = (bx.1 + k / bw) * w + bx.0 + k % bw;
                let relief = if relieved { 0.5 * SKIN_PA / (1.0 - NU * NU) * s.delta * 1e-6 } else { 0.0 };
                let mut at = self.skin.top[i];
                while at != NONE {
                    let p = &mut self.skin.pool[at as usize];
                    let skin = if at == s.at { p.s } else { p.h + p.w };
                    p.n = (p.n - relief * skin).max(0.0);
                    if at == s.at {
                        break;
                    }
                    at = p.below;
                }
                let d = dw[padded(k)];
                if d != 0.0 {
                    self.skin.pool[s.at as usize].w += d;
                    self.height[i] += d;
                }
            }
            self.surf_gen += 1;
            self.base = None;
        }
        give(vec![z, wf, need, skin, inner, wx, wy, dw, tmp, mr, m2r, mi, near, far, rho, zr, px, py, dot]);
    }

    /// The box (buffer pixels, end-exclusive) of the pixels holding packets.
    fn skin_box(&self) -> Option<(usize, usize, usize, usize)> {
        let w = self.f.w;
        let rows: Vec<Option<(usize, usize)>> = self.skin.top.par_chunks(w).map(|row| {
            let mut span: Option<(usize, usize)> = None;
            for (x, &t) in row.iter().enumerate() {
                if t != NONE {
                    span = Some(span.map_or((x, x + 1), |(a, _)| (a, x + 1)));
                }
            }
            span
        }).collect();
        let mut bx: Option<(usize, usize, usize, usize)> = None;
        for (y, s) in rows.into_iter().enumerate() {
            if let Some((a, b)) = s {
                bx = Some(match bx {
                    None => (a, y, b, y + 1),
                    Some(r) => (r.0.min(a), r.1, r.2.max(b), y + 1),
                });
            }
        }
        bx
    }

    /// One tick, the clock at `now` minutes: every packet's front advances
    /// over the hour (from its birth, if that is later), its skin takes up
    /// expansion, then the skins on soft bulk wrinkle.
    fn skin_tick(&mut self, now: f64) {
        let w = self.f.w;
        let since = self.skin.ticked;
        self.skin.ticked = now;
        let Some(bx) = self.skin_box() else { return };
        let dx = self.px_mm() * 1e-3;
        let nyq = std::f32::consts::PI / dx;
        // 1. chemistry: what each packet's front has to cross, then every
        // front, reserve and force over the hour
        let mut above = vec![0.0f32; self.skin.pool.len()];
        for y in bx.1..bx.3 {
            for x in bx.0..bx.2 {
                let mut at = self.skin.top[y * w + x];
                let mut over = 0.0;
                while at != NONE {
                    let p = &self.skin.pool[at as usize];
                    above[at as usize] = over;
                    over += p.h + p.w;
                    at = p.below;
                }
            }
        }
        let dtm = (now - since).max(0.0) as f32;
        let e = (-dtm / EXPAND_MIN).exp();
        self.skin.pool.par_iter_mut().zip(&above).for_each(|(p, &above)| {
            let thick = p.h + p.w;
            if p.h <= 0.0 || p.s >= thick || dtm <= 0.0 {
                return;
            }
            let dt = (now as f32 - (since as f32).max(p.born)).max(0.0);
            let u0 = above + p.s;
            let g = u0 + u0 * u0 / (2.0 * OXYGEN_UM) + p.v0 * dt;
            let u1 = -OXYGEN_UM + (OXYGEN_UM * OXYGEN_UM + 2.0 * OXYGEN_UM * g).sqrt();
            let s1 = (u1 - above).clamp(p.s, thick);
            let rate = (s1 - p.s) / dtm;
            p.s = s1;
            let x_int = p.x * EXPAND_MIN * (1.0 - e) + rate * EXPAND_MIN * (dtm - EXPAND_MIN * (1.0 - e));
            p.x = p.x * e + rate * EXPAND_MIN * (1.0 - e);
            p.n += SKIN_PA / (1.0 - NU * NU) * EXPAND * p.binder * (x_int * 1e-6 / EXPAND_MIN);
        });
        // 2. at every pixel of the box: the plate (every cured layer down
        // to the first soft one), its compression, and whether the gel under
        // it gives: the critical wavenumber and force of the plate on it
        let (bw, bh) = (bx.2 - bx.0, bx.3 - bx.1);
        let mut site = vec![Site { at: NONE, delta: 0.0, band: 0, floor: 0.0 }; bw * bh];
        let (top, pool, engine) = (&self.skin.top, &self.skin.pool, self.engine);
        site.par_chunks_mut(bw).enumerate().for_each(|(y, row)| {
            for (x, site) in row.iter_mut().enumerate() {
                let mut at = top[(bx.1 + y) * w + bx.0 + x];
                let (mut plate, mut force) = (0.0f32, 0.0f32);
                let mut soft = NONE;
                while at != NONE {
                    let p = &pool[at as usize];
                    let thick = p.h + p.w;
                    force += p.n;
                    if p.s >= thick - DRY_UM {
                        plate += thick;
                        at = p.below;
                        continue;
                    }
                    plate += p.s;
                    soft = at;
                    break;
                }
                if soft == NONE || plate <= 0.0 {
                    continue;
                }
                let p = &pool[soft as usize];
                let t = plate * 1e-6;
                let b = SKIN_PA * t * t * t / (12.0 * (1.0 - NU * NU));
                let c = SKIN_PA * t / (1.0 - NU * NU);
                let g = MODULUS_PER_YIELD * rheology_at(engine, p.stiff).1;
                // the bulk's depth over a wavelength: the wrinkle only moves
                // it from trough to crest
                let hb = (p.h - p.s) * 1e-6;
                let nf = force - SIGMA;
                if nf <= 0.0 || p.h + p.w - p.s <= DRY_UM {
                    continue;
                }
                let qh = (g / b).cbrt();
                let qc = if qh * hb >= DEEP { qh } else { (24.0 * g / (b * hb * hb * hb)).powf(1.0 / 6.0).min(DEEP / hb) };
                // resolved on this grid (a wavelength of two pixels or more)?
                if qc > nyq {
                    continue;
                }
                // the wrinkle is drawn at the band nearest its wavelength,
                // and the threshold is the plate's at the wavenumber drawn
                let band = (2.0 * std::f32::consts::PI / (qc * dx) / BAND_LAMBDA).round().clamp(1.0, BAND_MAX as f32) as u8;
                let q = 2.0 * std::f32::consts::PI / (band as f32 * BAND_LAMBDA * dx);
                let nc = b * q * q + if q * hb >= DEEP { 2.0 * g / q } else { 12.0 * g / (hb * hb * hb * q * q * q * q) };
                // von Kármán: slopes of mean square 2 (N − N_c) / C take up
                // the compression past the threshold (short of it: how far
                // the pixel is from wrinkling, for its neighbors' sake). A
                // trough stops at the paint under the bulk, so the skin asks
                // for no more slope than a wrinkle as deep as its soft bulk
                // has, q² H² / 2: a film nearly cured through holds what
                // compression it has left.
                let room = (p.h + p.w - p.s) * 1e-6;
                let delta = (2.0 * (nf - nc) / c).min(0.5 * (q * room) * (q * room));
                *site = Site { at: soft, delta, band, floor: p.s - p.h + DRY_UM };
            }
        });
        // a skin asks for slopes only where it has soft bulk on every side
        // to make them with: over a soft pixel whose neighbor is cured
        // through, or bare, it is held, and asks nothing (its bulk still
        // moves with the wrinkle around it, and that relieves it)
        let lone: Vec<usize> = (0..bw * bh).into_par_iter().filter(|&k| {
            let (x, y) = (k % bw, k / bw);
            let soft = |x: usize, y: usize| x < bw && y < bh && site[y * bw + x].at != NONE;
            site[k].at != NONE && site[k].delta > 0.0 && !(soft(x.wrapping_sub(1), y) && soft(x + 1, y) && soft(x, y.wrapping_sub(1)) && soft(x, y + 1))
        }).collect();
        for k in lone {
            site[k].delta = 0.0;
        }
        // 3. the wrinkles: the relief band-passed at each wavelength in
        // use, scaled to unit rms slope, is the pattern; each pixel's bulk
        // takes the slope its skin asks for, over what it has. A wavelength
        // of skin acts together, so what a pixel asks for counts averaged
        // over its wavelength, with the paint around it that is cured
        // through, or bare, asking nothing: the skin over a soft spot
        // smaller than a wavelength is held by the cured paint around it.
        // The pass runs when that average is GROW² of slope or more.
        if site.iter().any(|s| s.at != NONE && s.delta >= GROW * GROW) {
            let pad = 2 * BAND_MAX;
            let pb = (bx.0.saturating_sub(pad), bx.1.saturating_sub(pad), (bx.2 + pad).min(w), (bx.3 + pad).min(self.f.h));
            let (pw, ph) = (pb.2 - pb.0, pb.3 - pb.1);
            let padded = |k: usize| (bx.1 + k / bw - pb.1) * pw + bx.0 + k % bw - pb.0;
            let mut scratch = take(3);
            let [need, tmp, avg] = &mut scratch[..] else { unreachable!() };
            need.resize(pw * ph, 0.0);
            for (k, s) in site.iter().enumerate() {
                if s.at != NONE {
                    need[padded(k)] = s.delta;
                }
            }
            let mut bands: Vec<u8> = site.iter().filter(|s| s.at != NONE).map(|s| s.band).collect();
            bands.sort_unstable();
            bands.dedup();
            let asks = bands.iter().any(|&band| {
                crate::surface::box_blur_into(need, pw, ph, 2 * band as usize, tmp);
                crate::surface::box_blur_into(tmp, pw, ph, 2 * band as usize, avg);
                site.iter().enumerate().any(|(k, s)| s.at != NONE && s.band == band && avg[padded(k)] >= GROW * GROW)
            });
            give(scratch);
            if asks {
                self.wrinkle(&site, bx, pb, bands);
            }
        }
        // 4. packets cured through from the bottom up are done: their
        // wrinkle is part of the picture, and the record goes. A cured
        // packet over soft bulk stays: it is the plate, and what the
        // oxygen crosses.
        let (top, pool, free) = (&mut self.skin.top, &mut self.skin.pool, &mut self.skin.free);
        let mut chain = Vec::new();
        for y in bx.1..bx.3 {
            for t in &mut top[y * w + bx.0..y * w + bx.2] {
                chain.clear();
                let mut at = *t;
                while at != NONE {
                    chain.push(at);
                    at = pool[at as usize].below;
                }
                while let Some(&bottom) = chain.last() {
                    let p = pool[bottom as usize];
                    if p.s < p.h + p.w - DRY_UM {
                        break;
                    }
                    chain.pop();
                    pool[bottom as usize].h = 0.0;
                    free.push(bottom);
                    self.skin.live -= 1;
                    match chain.last() {
                        Some(&upper) => pool[upper as usize].below = NONE,
                        None => *t = NONE,
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wet::Paint;

    /// A canvas of `w` × `h` pixels at a quarter millimeter each, painted
    /// with the new material.
    fn canvas(w: usize, h: usize) -> Canvas {
        Canvas::new(w, w as f32 / h as f32, [0.8; 3]).with_engine(9).with_size_mm(0.25 * w as f32).with_material(1)
    }

    /// Lay a wet film of `paint`, `coats` thick, over the pixel box.
    fn lay(c: &mut Canvas, paint: &Paint, coats: f32, bx: (usize, usize, usize, usize)) {
        let w = c.f.w;
        for y in bx.1..bx.3 {
            for x in bx.0..bx.2 {
                let i = y * w + x;
                c.wet.vol[i] = coats;
                c.wet.lat[i] = paint.latent();
                c.wet.hide[i] = paint.prop();
                c.wet.cover[i] = 1.0;
            }
        }
        c.wet.dirty = Some(match c.wet.dirty {
            None => bx,
            Some(d) => (d.0.min(bx.0), d.1.min(bx.1), d.2.max(bx.2), d.3.max(bx.3)),
        });
    }

    /// Brush furrows on the surface: a few µm of relief along x, the
    /// imperfections a skin cures over.
    fn furrow(c: &mut Canvas, um: f32, period: usize) {
        let w = c.f.w;
        for (i, h) in c.height.iter_mut().enumerate() {
            let x = i % w;
            *h += um * ((x % period) as f32 / period as f32 * std::f32::consts::TAU).sin();
        }
    }

    /// A body paint of stiffness `stiff` (`surface::rheology_at`: 0.1 is
    /// medium-rich at some 10 Pa yield, 1.0 tube paint at 3000 Pa).
    fn paint(stiff: f32) -> Paint {
        Paint::new([0.9; 3], 0.9, stiff).with_drying(2.0)
    }

    fn relief_rms(c: &Canvas, bx: (usize, usize, usize, usize)) -> f32 {
        let w = c.f.w;
        let px: Vec<f64> = (bx.1..bx.3).flat_map(|y| (bx.0..bx.2).map(move |x| (x, y))).map(|(x, y)| c.height[y * w + x] as f64).collect();
        let mean = px.iter().sum::<f64>() / px.len() as f64;
        (px.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / px.len() as f64).sqrt() as f32
    }

    fn total_paint(c: &Canvas) -> f64 {
        c.height.iter().map(|&h| h as f64).sum()
    }

    #[test]
    fn front_crosses_a_thin_film_and_stalls_in_a_thick_one() {
        let mut c = canvas(8, 8);
        let p = paint(0.9);
        lay(&mut c, &p, 1.0, (0, 0, 8, 4));
        lay(&mut c, &p, 40.0, (0, 4, 8, 8));
        c.dry();
        assert!(c.skin.live >= 32, "{}", c.skin.live);
        c.wait(3.0 * 24.0 * 60.0);
        // the thin film is cured through and gone; the thick one has a skin over soft paint
        assert!(c.skin.column(0).next().is_none(), "{:?}", c.skin.column(0).next());
        let thick = *c.skin.column(5 * 8).next().unwrap();
        assert!(thick.s > 20.0 && thick.s < 0.5 * thick.h, "front at {} µm of {}", thick.s, thick.h);
        // a year on it is still soft inside: diffusion-limited, ~√t
        c.wait(365.0 * 24.0 * 60.0);
        let later = *c.skin.column(5 * 8).next().unwrap();
        assert!(later.s > thick.s * 3.0 && later.s < later.h + later.w, "front at {} µm of {} after a year", later.s, later.h);
    }

    #[test]
    fn a_cover_slows_the_front_under_it_but_keeps_its_soft_bulk() {
        let mut c = canvas(8, 8);
        let base = Paint::new([0.2; 3], 0.9, 0.5).with_drying(0.4);
        lay(&mut c, &base, 20.0, (0, 0, 8, 8));
        c.dry();
        let mut bare = c.clone();
        lay(&mut c, &paint(0.9), 2.0, (0, 0, 8, 4));
        c.dry();
        bare.wait((c.clock() - bare.clock()) as f32);
        c.wait(10.0 * 24.0 * 60.0);
        bare.wait(10.0 * 24.0 * 60.0);
        let covered: Vec<_> = c.skin.column(0).copied().collect();
        let open = *bare.skin.column(0).next().unwrap();
        assert_eq!(covered.len(), 2, "the cover and the film under it: {covered:?}");
        assert!(covered[0].s >= covered[0].h - DRY_UM, "the thin cover cures through: {:?}", covered[0]);
        assert!(covered[1].s < open.s, "covered front {} vs open {}", covered[1].s, open.s);
        assert!(covered[1].s < covered[1].h, "the base is still soft under its cover: {:?}", covered[1]);
        let uncovered = *c.skin.column(5 * 8).next().unwrap();
        assert_eq!(uncovered.s, open.s, "the uncovered half dries as the bare canvas does");
    }

    /// The middle of the `bx` passage, away from its anchored rim.
    const INNER: (usize, usize, usize, usize) = (14, 14, 50, 50);

    /// What the packets add: `build` painted with the new material and
    /// with material 0 (no packets, the same wet paint and calendar), both
    /// waited `days` after it; the rms relief over `INNER` of each, legacy
    /// first, and the new one.
    fn paired(build: impl Fn(&mut Canvas), days: f32) -> (f32, f32, Canvas) {
        let mut legacy = canvas(64, 64).with_material(0);
        let mut c = canvas(64, 64);
        build(&mut legacy);
        build(&mut c);
        let paint_before = total_paint(&c);
        legacy.wait(days * 24.0 * 60.0);
        c.wait(days * 24.0 * 60.0);
        assert_eq!(legacy.clock(), c.clock());
        assert!((total_paint(&c) - paint_before).abs() < 1e-2 * paint_before.abs().sqrt(), "paint is conserved: {} vs {}", total_paint(&c), paint_before);
        (relief_rms(&legacy, INNER), relief_rms(&c, INNER), c)
    }

    #[test]
    fn a_thick_fat_film_wrinkles_and_a_stiff_or_thin_one_does_not() {
        let bx = (6, 6, 58, 58);
        let film = |paint: Paint, coats: f32| {
            move |c: &mut Canvas| {
                furrow(c, 3.0, 6);
                lay(c, &paint, coats, bx);
                c.dry();
            }
        };
        let (b, a, c) = paired(film(paint(0.1), 40.0), 6.0);
        assert!(a > 4.0 * b && a > 8.0, "a 1 mm medium-rich film wrinkles: relief {b} → {a} µm rms");
        let top = c.skin.column(32 * 64 + 32).next().unwrap();
        assert!(top.w.abs() > 1.0 && top.s < top.h + top.w, "bulk moved under a skin: {top:?}");
        let (b, a, _) = paired(film(paint(1.0), 40.0), 6.0);
        assert!(a < 1.5 * b, "stiff tube paint (3000 Pa yield) holds: {b} → {a}");
        let (b, a, _) = paired(film(paint(0.1), 2.0), 6.0);
        assert!((a - b).abs() < 0.5, "a thin medium-rich film has no soft bulk left to wrinkle: {b} → {a}");
    }

    #[test]
    fn wrinkles_are_the_same_however_the_wait_is_split() {
        let bx = (6, 6, 42, 42);
        let mut c = canvas(48, 48);
        furrow(&mut c, 3.0, 6);
        lay(&mut c, &paint(0.1), 40.0, bx);
        c.dry();
        let mut one = c.clone();
        one.wait(4.0 * 24.0 * 60.0);
        let mut many = c.clone();
        for _ in 0..96 {
            many.wait(60.0);
        }
        let mut odd = c.clone();
        for _ in 0..(4 * 24 * 60 / 37) {
            odd.wait(37.0);
        }
        odd.wait((4 * 24 * 60 % 37) as f32);
        assert_eq!(one.height, many.height);
        assert_eq!(one.height, odd.height);
        let inner = (14, 14, 34, 34);
        assert!(relief_rms(&one, inner) > 8.0, "{}", relief_rms(&one, inner));
    }

    #[test]
    fn a_fast_lean_cover_over_slow_fat_paint_wrinkles_and_lean_over_lean_does_not() {
        let bx = (6, 6, 58, 58);
        let lean = Paint::new([0.9; 3], 0.9, 0.9).with_drying(2.4);
        let slow = Paint::new([0.2; 3], 0.9, 0.1).with_drying(0.3);
        let layered = |first: &Paint, second: Option<&Paint>| {
            let (first, second) = (first.clone(), second.cloned());
            move |c: &mut Canvas| {
                furrow(c, 3.0, 6);
                lay(c, &first, 24.0, bx);
                c.dry();
                if let Some(second) = &second {
                    lay(c, second, 3.0, bx);
                    c.dry();
                }
            }
        };
        let (flat, covered, c) = paired(layered(&slow, Some(&lean)), 30.0);
        assert!(covered > 4.0 * flat && covered > 8.0, "a lean cover over slow fat paint wrinkles: {flat} → {covered}");
        let col: Vec<_> = c.skin.column(32 * 64 + 32).copied().collect();
        assert!(col.len() == 2 && col[0].s >= col[0].h && col[1].w.abs() > 1.0, "the cover is the plate, the fat paint under it moved: {col:?}");
        let (b, a, _) = paired(layered(&lean, Some(&lean)), 30.0);
        assert!(a < 1.5 * b, "lean over lean holds: {b} → {a}");
    }

    #[test]
    fn a_checkpoint_keeps_the_films_and_paints_on_as_before() {
        let bx = (2, 2, 14, 14);
        let mut c = canvas(16, 16);
        furrow(&mut c, 3.0, 6);
        lay(&mut c, &paint(0.1), 40.0, bx);
        c.dry();
        c.wait(3.0 * 24.0 * 60.0);
        lay(&mut c, &paint(0.9), 2.0, (2, 2, 14, 8));
        c.dry();
        assert!(c.skin.live > 0);
        let mut bytes = Vec::new();
        c.write_state(&mut bytes, "").unwrap();
        assert_eq!(&bytes[..8], b"PAINTC22");
        let (mut d, _) = Canvas::read_state(&mut &bytes[..]).unwrap();
        let films = |c: &Canvas| (0..256).map(|i| c.skin.column(i).map(|p| Packet { below: NONE, ..*p }).collect::<Vec<_>>()).collect::<Vec<_>>();
        assert_eq!(d.material(), 1);
        assert!(films(&d) == films(&c) && d.skin.live == c.skin.live && d.skin.ticked == c.skin.ticked && d.height == c.height, "{} films", d.skin.live);
        let mut again = Vec::new();
        d.write_state(&mut again, "").unwrap();
        assert!(again == bytes, "rewritten as it was read");
        c.wait(5.0 * 24.0 * 60.0);
        d.wait(5.0 * 24.0 * 60.0);
        assert!(d.height == c.height && films(&d) == films(&c));
        let mut legacy = Vec::new();
        canvas(4, 4).with_material(0).write_state(&mut legacy, "").unwrap();
        assert_eq!(&legacy[..8], b"PAINTC19");
    }

    #[test]
    fn legacy_material_keeps_no_packets() {
        let mut c = Canvas::new(8, 1.0, [0.8; 3]).with_engine(9).with_size_mm(2.0);
        lay(&mut c, &paint(0.1), 40.0, (0, 0, 8, 8));
        let before = c.height.clone();
        c.dry();
        c.wait(10.0 * 24.0 * 60.0);
        assert!(c.skin.top.is_empty());
        assert!(c.height.iter().zip(&before).all(|(a, b)| a - b == 40.0 * COAT_UM));
    }
}
