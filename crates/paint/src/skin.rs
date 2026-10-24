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
//! straight from the tube holds, as Ranquet's two films did. The gel
//! holds elastically only to its yield strain, a few percent: past it the
//! paste flows from the troughs to the crests, and what holds the skin
//! then is its yield stress `τ_y` as a plug flow through the depth it
//! has, a pressure `π τ_y / q H` under a thin bulk and `π τ_y` under a
//! deep one, which on a wrinkle of rms slope `σ` is a membrane force
//! `2√2 τ_y / (q σ min(1, qH))`; the skin beats the lesser of this and
//! the gel's elastic hold. Under a thin bulk the yield amplitude is under
//! a micrometer (the layer shears `6 A / q H²`), so the brush marks a
//! skin cures over have yielded it already and the yield stress is what
//! it beats: a few hundred micrometers of medium-rich paint wrinkle where
//! the elastic layer would have held them at a stiffness growing as
//! `1 / H³`, and a yield stress ten times theirs holds at that thickness.
//! Since the hold falls as the wrinkle grows, a wrinkle that starts runs
//! until the slopes have the compression, and the skin takes the
//! wavelength that needs the least at the slopes it has: on an elastic
//! gel `q_c` above, on a yielded one where bending balances the plug
//! flow, `(√2 τ_y / B σ)^(1/3)` on deep bulk and `(2√2 τ_y / B H σ)^(1/4)`
//! on thin. The slopes a skin has at its wavelength are measured in the
//! pass (`Packet::slope`, brush marks and wrinkle together), the pass
//! draws at that measurement, and a skin not measured yet asks for a
//! pass when it could wrinkle on a yielded bulk. The wrinkle is drawn at
//! the band of wavelengths nearest its own (`BAND_LAMBDA`), and the
//! threshold is the plate's at the wavenumber drawn. The gel
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
//! along them, and along the roughness every surface has, `SEED_UM`, which
//! runs in domains of their own heading, `DOMAIN_UM`) and
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
//! compression (`surface::SIGMA`). Each place's wavelength strays from its
//! skin's by a seeded factor (`JITTER`), and some ridges gather into folds
//! faster than others (`FOLD_SPREAD`). Within a pass the folds form one at
//! a time, as cracks do: each runs along its ridge and takes the
//! compression off the skin beside it, so a few long folds stand over calm
//! ground (`FOLD_REACH`). A wavelength under `FINEST` pixels
//! is below what the picture draws cleanly: the skin is drawn there if it
//! beats the threshold there (every wavelength the compression beats is
//! unstable; the least one only grows fastest).
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
//! wrinkles only around skin whose wavelength's worth asks for `GROW` of
//! slope or more, and there a pass settles every wavelength that asks at
//! all; skin that only has to be measured is measured and not drawn, and
//! twice a day (`DRAW_ALL`) a pass draws whatever any skin asks for. So
//! the slow creep of a skin that has long since wrinkled costs a pass over
//! its packets, not the blurs, and a pass runs over the tiles around the
//! skin that asks, not over the whole picture.
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
/// The bands a wrinkle is drawn at (blur radius, px): each one up to four,
/// then two to an octave. A wavelength between two is drawn at both, by its
/// logarithm (`Site::mix`).
const BANDS: [u8; 8] = [1, 2, 3, 4, 6, 8, 12, 16];
/// The least growth of a wrinkle's rms slope, were it starting from
/// level, that is drawn in one tick (6°, some 35 µm of amplitude at a
/// 2 mm wavelength); slower growth is saved up for a tick that has this
/// much (or for one of `DRAW_ALL`'s), and a pass settles every wavelength
/// that asks at all around it, so a skin that creeps costs a pass over its
/// packets, not the blurs.
const GROW: f32 = 0.1;
/// Every so many ticks a pass draws whatever the skin asks for, however
/// little: growth short of `GROW` waits for a tick that has that much,
/// and a skin that never asks for that much again would keep it until its
/// bulk cures through. Twice a day.
const DRAW_ALL: u64 = 2;
/// Roughness every surface has, µm, at the wrinkle's own wavelength and an
/// octave each side (dust, pigment lumps, a knife's chatter), so a skin
/// cured over a level film still has imperfections to buckle from, and a
/// skin over brush marks buckles from them as well as from the marks: a
/// noise seeded by the place and the film, so a replay is a replay. By eye.
const SEED_UM: f32 = 1.0;
/// Ripples of the wrinkle's own wavelength in that roughness, µm, each
/// patch of `DOMAIN_MM` running its own way, whatever wavelength a place
/// of it is drawn at: a skin buckles in domains that each pick a heading,
/// and its ridges turn where two meet (as a zinc drawdown's do, Gnemmi).
/// By eye: stronger than a knife's marks, so the patches' ridges run on
/// through them.
const DOMAIN_UM: f32 = 40.0;
const DOMAIN_MM: f32 = 12.0;
/// How far, mm, the ripples are pushed about by a smooth field over
/// `WARP_SCALE_MM`: their ridges bend and their spacing wanders (no skin
/// buckles as evenly as a sinusoid); and how unevenly they stand, the
/// standard deviation of their height over the same. By eye.
const WARP_MM: f32 = 1.2;
const WARP_SCALE_MM: f32 = 5.0;
const RIDGE_SPREAD: f32 = 0.5;
/// How much a skin's wavelength strays from place to place beyond what its
/// thickness and its paste make of it: the standard deviation of its
/// logarithm, over patches of `JITTER_MM` (a film is never as even as its
/// averages). By eye.
const JITTER: f32 = 0.4;
const JITTER_MM: [f32; 3] = [8.0, 4.0, 2.0];
/// The finest wavelength drawn, px: under `BAND_LAMBDA` the finest band's
/// pattern is drawn by its Gaussians alone, down to here (finer, its ridges
/// alias into a hatching of the grid).
const FINEST: f32 = 4.0;
/// How unevenly the ridges of one wavelength gather into folds: the standard
/// deviation of the logarithm of how fast each grows, over patches of
/// `FOLD_LAMBDAS` wavelengths (the paste and the skin over it are uneven at
/// every scale), so some ridges take the compression and the ground between
/// them stays flat. By eye.
const FOLD_SPREAD: f32 = 1.2;
const FOLD_LAMBDAS: f32 = 12.0;
/// How far a fold takes the compression off the skin beside it, in
/// wavelengths: the half width of the share of it it relieves (a Lorentzian
/// cut where it falls to a tenth, as a crack relieves the stress beside it,
/// `crate::crack`). By eye.
const FOLD_REACH: f32 = 2.5;
/// How compressed the skin must be for a fold to run on through it, as a
/// share of what wrinkles it, over a wavelength or so: a fold starts only
/// where the skin is past that, as a crack starts at half again the stress
/// it runs on at (`crate::crack`), and runs on further. By eye.
const FOLD_RUN: f32 = 0.5;
/// How far a running fold turns toward the way the ridges run, per cell.
const FOLD_STEER: f32 = 0.5;
/// Of the pattern's `SETTLE` steps, how many it takes around its folds.
const FOLD_SETTLE: usize = 8;
/// How wide a fold is, in wavelengths: the skin it gathers its slopes into.
const FOLD_WIDTH: f32 = 0.6;
/// How tall a fold may stand, as a multiple of the amplitude of a sinusoid
/// of the same root mean square slope: a fold gathers its slopes into a
/// ridge taller than a ripple's (the cap is there so the wall of a stroke at
/// a skin's edge is not a wrinkle). By eye.
const TALL: f32 = 3.5;
const JITTER_SEED: u64 = 0x7a11_5eed;
const FOLD_SEED: u64 = 0xf01d_5eed;
/// Steps a wrinkle's shape takes to settle, and the gain of a step: how
/// far past its threshold the skin is, as Swift–Hohenberg's ε (well past
/// it: the pattern saturates into ridges a wavelength apart, as a skin
/// pressed hard enough to yield its paste does). The more steps, the
/// longer its ridges grow and the more they follow the marks it cured
/// over. By eye.
const SETTLE: usize = 24;
const SETTLE_GAIN: f32 = 4.0;
/// The flow index n of the paste under a skin: a pigment paste thins as
/// it flows (Herschel–Bulkley, stress τ_y + K γ̇ⁿ with n under 1; pastes
/// and paints run 0.3–0.8). Estimate.
const FLOW_INDEX: f32 = 0.5;
/// How many times farther than a wrinkle's window (`2 r`) a fold gathers
/// the compression from: the wrinkles around it relax as it grows (period
/// doubling and quadrupling before a fold; Brau et al. 2011), and its
/// slopes are measured, and spent, over that much skin. By eye.
const FOLD: usize = 4;
/// How many wavelengths the unevenness that bends a wrinkle's ridges
/// spans: a herringbone's ridges jog every few wavelengths (Audoly and
/// Boudaoud 2008), and a skin on uneven paint bends them where the paint
/// lies higher or lower. By eye.
const BEND: usize = 3;
/// How unevenly the medium mixed into a paint is spread through its film:
/// one standard deviation, as a share of the medium (or, blotted, of the
/// oil drawn out). A pile thinned with a knife on the palette is streaked
/// fatter and leaner; paint as it left the tube is milled even. Estimate:
/// Ranquet's lead-white films go from wrinkled to flat between 32 and
/// 42 vol% of pigment, and three tenths of the medium of a pile thinned half
/// with it moves its paint's yield stress between 9 and 43 Pa.
pub const OIL_SPREAD: f32 = 0.3;
/// How unevenly a film cures through: the standard deviation of the log of
/// its paint's drying rate from place to place. Driers and drier pigments
/// are spread by hand as unevenly as the medium, and the air, the warmth
/// and the ground under a film are not the same everywhere (Golden;
/// Ranquet). Estimate.
pub const DRIER_SPREAD: f32 = 0.5;
/// The sizes, mm, of the streaks and patches a film's medium and drier
/// vary over, largest first, each as uneven as the others: a knife's mixing
/// streaks are centimeters long, its lumps under a millimeter.
const PATCH_MM: [f32; 6] = [16.0, 8.0, 4.0, 2.0, 1.0, 0.5];
/// Minutes between the gel times of two films for their streaks to be
/// unrelated: paint laid a day later was mixed afresh.
const FILM_MIN: f32 = 24.0 * 60.0;
const OIL_SEED: u64 = 0x0011_5eed;
const DRIER_SEED: u64 = 0xd41e_5eed;
const NONE: u32 = u32::MAX;
/// The tiles a tick looks for skin that could wrinkle in, px.
const TILE: usize = 32;
/// A packet's `slope` once a pass has measured it and found none at its
/// wavelength (0: not measured yet).
const FLAT: f32 = -1.0;

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
    /// The rms slope of the relief over it at its wrinkle's wavelength,
    /// brush marks and wrinkle together, as the last pass measured it (0:
    /// not measured yet; `FLAT`: measured, and none there). What the
    /// yielded bulk under its skin holds against is this.
    pub slope: f32,
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
    /// (negative: how far the skin is from wrinkling), at the slopes the
    /// skin was last measured with; 0 while it has none measured.
    delta: f32,
    /// The two bands its wavelength is drawn between (`BANDS`).
    band: u8,
    next: u8,
    /// The share of the wrinkle drawn at `next`: where its wavelength lies
    /// between the two, by its logarithm.
    mix: f32,
    /// Its wavelength, in `BAND_LAMBDA`s (under 1: finer than the finest
    /// band, which draws it).
    lam: f32,
    /// The plate's thickness, µm: every cured layer down to the soft one.
    plate: f32,
    /// The least displacement the bulk allows, µm (none left under the skin).
    floor: f32,
    /// Not measured yet, and the skin could wrinkle on a yielded bulk: a
    /// pass measures it and draws at the measurement.
    measure: bool,
    /// Past its threshold with the yielded bulk the lesser hold, and the
    /// hold it would shed by taking its ask is itself `GROW` of ask: the
    /// wrinkle runs, so a pass draws it without waiting for `GROW` of ask.
    runs: bool,
    /// The plate's compression less the oil's surface tension, N/m.
    nf: f32,
    /// What bending costs at the band's wavenumber, `B q²`, N/m.
    bend: f32,
    /// The gel's elastic hold at that wavenumber, N/m.
    hold: f32,
    /// The plate's extensional stiffness `C`, N/m.
    c: f32,
    /// The most mean square slope the soft bulk has room for.
    cap: f32,
    /// The yielded bulk's hold, N/m, per unit of the slope it is divided by.
    yielded: f32,
    /// How high the wrinkle has to be here, as a share of its root mean
    /// square height around, for the skin to press the paste under it past
    /// its yield: the yielded hold over the plate's drive at the slopes
    /// measured, more under a trough than a crest where the bulk is thin (it
    /// holds harder where there is less of it); 0 on an elastic gel, which
    /// gives under the whole wrinkle at once.
    gives: f32,
}

impl Site {
    /// The share of the wrinkle drawn at `band`.
    fn share(&self, band: u8) -> f32 {
        if band == self.band {
            1.0 - self.mix
        } else if band == self.next {
            self.mix
        } else {
            0.0
        }
    }

    /// The mean square slope the skin asks for over what it has, with the
    /// bulk holding at the lesser of its elastic and its yielded hold at
    /// slopes `slope` (none measured, or none at all: the elastic one).
    fn ask(&self, slope: f32) -> f32 {
        let hold = if slope > 0.0 { self.hold.min(self.yielded / slope) } else { self.hold };
        (2.0 * (self.nf - self.bend - hold) / self.c).min(self.cap)
    }
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

/// What the band's filter (the relief blurred twice at radius `r`, less
/// blurred twice at `2 r`) leaves of a sinusoid at the band's own
/// wavelength, `BAND_LAMBDA r`: the slope it measures, over the relief's.
fn band_gain(r: usize) -> f32 {
    let lambda = BAND_LAMBDA * r as f32;
    let twice = |width: usize| {
        let g = (std::f32::consts::PI * width as f32 / lambda).sin() / (width as f32 * (std::f32::consts::PI / lambda).sin());
        g * g
    };
    (twice(2 * r + 1) - twice(4 * r + 1)).max(0.05)
}

/// The boxes (tiles, end-exclusive) of the marked tiles of a `tw` × `th`
/// grid, each with `m` tiles around it: marks within reach of each other
/// share a box, and boxes that overlap are one.
fn patches(marks: &[bool], tw: usize, th: usize, m: usize) -> Vec<(usize, usize, usize, usize)> {
    let mut boxes: Vec<(usize, usize, usize, usize)> = Vec::new();
    let mut seen = vec![false; tw * th];
    for i in 0..tw * th {
        if !marks[i] || seen[i] {
            continue;
        }
        let mut b = (tw, th, 0, 0);
        let mut todo = vec![i];
        seen[i] = true;
        while let Some(j) = todo.pop() {
            let (x, y) = (j % tw, j / tw);
            b = (b.0.min(x), b.1.min(y), b.2.max(x + 1), b.3.max(y + 1));
            for v in y.saturating_sub(2 * m)..(y + 2 * m + 1).min(th) {
                for u in x.saturating_sub(2 * m)..(x + 2 * m + 1).min(tw) {
                    if marks[v * tw + u] && !seen[v * tw + u] {
                        seen[v * tw + u] = true;
                        todo.push(v * tw + u);
                    }
                }
            }
        }
        boxes.push((b.0.saturating_sub(m), b.1.saturating_sub(m), (b.2 + m).min(tw), (b.3 + m).min(th)));
    }
    while let Some((a, b)) = (0..boxes.len()).flat_map(|a| (a + 1..boxes.len()).map(move |b| (a, b))).find(|&(a, b)| {
        let (p, q) = (boxes[a], boxes[b]);
        p.0 < q.2 && q.0 < p.2 && p.1 < q.3 && q.1 < p.3
    }) {
        let q = boxes.swap_remove(b);
        let p = &mut boxes[a];
        *p = (p.0.min(q.0), p.1.min(q.1), p.2.max(q.2), p.3.max(q.3));
    }
    boxes.sort_unstable_by_key(|b| (b.1, b.0));
    boxes
}

/// `src` (`w` × `h`) blurred by a Gaussian of `scale` × `sigma` px,
/// `sigma` each pixel's own (it weighs its neighbors by it), rows then
/// columns, the edges held, over the box `bx` of `out` (the rest of it
/// left as it was).
fn gauss_into(src: &[f32], w: usize, h: usize, bx: (usize, usize, usize, usize), sigma: &[f32], scale: f32, tmp: &mut Vec<f32>, out: &mut Vec<f32>) {
    // (a run of pixels at once: each the sum of `tap(j)` at its place,
    // weighed by e^(−j²/2s²) with its own s, the weights by their ratios
    // e^(−(2j+1)/2s²), over three of the widest s in the run)
    fn run<'a>(o: &mut [f32], sigma: &[f32], scale: f32, tap: impl Fn(isize) -> &'a [f32]) {
        let n = o.len();
        let (mut norm, mut wt, mut step, mut aa) = (vec![1.0f32; n], vec![1.0f32; n], vec![0.0f32; n], vec![0.0f32; n]);
        for i in 0..n {
            let s = scale * sigma[i];
            step[i] = (-0.5 / (s * s)).exp();
            aa[i] = step[i] * step[i];
        }
        o.copy_from_slice(&tap(0)[..n]);
        for j in 1..=(3.0 * scale * sigma.iter().fold(0.0f32, |m, s| m.max(*s))).ceil() as isize {
            let (p, m) = (&tap(j)[..n], &tap(-j)[..n]);
            for i in 0..n {
                wt[i] *= step[i];
                step[i] *= aa[i];
                o[i] += wt[i] * (p[i] + m[i]);
                norm[i] += 2.0 * wt[i];
            }
        }
        o.iter_mut().zip(&norm).for_each(|(v, n)| *v /= n);
    }
    let (x0, y0, x1, y1) = bx;
    let reach = (3.0 * scale * sigma.iter().fold(0.0f32, |m, s| m.max(*s))).ceil() as usize;
    tmp.resize(w * h, 0.0);
    out.resize(w * h, 0.0);
    let (t0, t1) = (y0.saturating_sub(reach), (y1 + reach).min(h));
    tmp[t0 * w..t1 * w].par_chunks_mut(w).enumerate().for_each(|(j, row)| {
        let y = t0 + j;
        let s = &src[y * w..(y + 1) * w];
        let held: Vec<f32> = (x0 as isize - reach as isize..(x1 + reach) as isize).map(|x| s[x.clamp(0, w as isize - 1) as usize]).collect();
        run(&mut row[x0..x1], &sigma[y * w + x0..y * w + x1], scale, |i| &held[(reach as isize + i) as usize..]);
    });
    let tmp = &*tmp;
    out[y0 * w..y1 * w].par_chunks_mut(w).enumerate().for_each(|(j, row)| {
        let y = y0 + j;
        run(&mut row[x0..x1], &sigma[y * w + x0..y * w + x1], scale, |i| &tmp[(y as isize + i).clamp(0, h as isize - 1) as usize * w + x0..]);
    });
}

/// A smooth field of zero mean and unit spread over the canvas (`xm`, `ym`
/// mm) and the gel time of a film (`born` minutes): an octave per size in
/// `mm`, each a value noise that passes to a fresh one over `FILM_MIN` of
/// gel time.
fn grain(xm: f32, ym: f32, born: f32, mm: &[f32], seed: u64) -> f32 {
    let t = born / FILM_MIN;
    let cell = t.floor();
    let f = t - cell;
    let f = f * f * (3.0 - 2.0 * f);
    let step = 0x9e37_79b9_u64;
    let at = seed.wrapping_add((cell as i64 as u64).wrapping_mul(step));
    let mut sum = 0.0f32;
    for (o, &mm) in mm.iter().enumerate() {
        let s = at.wrapping_add(o as u64 * 0x1_0000);
        let now = vnoise(xm / mm, ym / mm, s) - 0.5;
        let next = vnoise(xm / mm, ym / mm, s.wrapping_add(step)) - 0.5;
        sum += now + (next - now) * f;
    }
    // (a value noise spreads 0.214: bilinear, smoothstep weights; two of
    // them blended at f spread as √((1 − f)² + f²) of one)
    sum / (0.214 * (mm.len() as f32 * ((1.0 - f) * (1.0 - f) + f * f)).sqrt())
}

/// Ripples `lam` mm apart in patches `DOMAIN_MM` across, each patch's own
/// heading and phase (`seed`), passing from one patch to the next where
/// they meet: about the center of each of the four patches nearest (`xm`,
/// `ym`), its ripple, weighed by how near, read at a place pushed about by
/// `WARP_MM` and as high as `RIDGE_SPREAD` has it there. About unit
/// amplitude.
fn domains(xm: f32, ym: f32, lam: f32, seed: u64) -> f32 {
    let field = |s: u64| (vnoise(xm / WARP_SCALE_MM, ym / WARP_SCALE_MM, seed ^ s) - 0.5) / 0.214;
    let height = (1.0 + RIDGE_SPREAD * field(0x4e1d)).max(0.0);
    let (xm, ym) = (xm + WARP_MM * field(0x7a2b), ym + WARP_MM * field(0x7a2c));
    let cell = DOMAIN_MM;
    let (fx, fy) = (xm / cell - 0.5, ym / cell - 0.5);
    let (i0, j0) = (fx.floor(), fy.floor());
    let ease = |t: f32| t * t * (3.0 - 2.0 * t);
    let (tx, ty) = (ease(ease(fx - i0)), ease(ease(fy - j0)));
    let mut sum = 0.0f32;
    for (di, dj, wt) in [(0, 0, (1.0 - tx) * (1.0 - ty)), (1, 0, tx * (1.0 - ty)), (0, 1, (1.0 - tx) * ty), (1, 1, tx * ty)] {
        let (i, j) = (i0 as i64 + di, j0 as i64 + dj);
        let heading = std::f32::consts::PI * crate::rng::hash2(i, j, seed);
        let phase = std::f32::consts::TAU * crate::rng::hash2(i, j, seed ^ 0x9e37_79b9);
        let (cx, cy) = ((i as f32 + 0.5) * cell, (j as f32 + 0.5) * cell);
        sum += wt * (std::f32::consts::TAU / lam * ((xm - cx) * heading.cos() + (ym - cy) * heading.sin()) + phase).cos();
    }
    sum * height
}

/// The paint `hd` as it is at the point (`xm`, `ym`) mm of a film that
/// gelled at minute `born`, not as the stroke mixed it on average: its
/// medium and its drier spread as unevenly as `OIL_SPREAD` and
/// `DRIER_SPREAD`. More medium here is a paste softer by the square of
/// what isn't medium, and fatter (`palette::Mixture::paint`).
fn uneven(hd: &crate::wet::Prop, xm: f32, ym: f32, born: f32) -> crate::wet::Prop {
    let mut p = *hd;
    let medium = if hd[4] >= 1.0 { (hd[4] - 1.0) / 1.5 } else { hd[4] - 1.0 };
    let here = medium + OIL_SPREAD * medium.abs() * grain(xm, ym, born, &PATCH_MM, OIL_SEED);
    let (k, kh) = (1.0 - medium, (1.0 - here).max(0.0));
    if k > 0.0 {
        p[1] = (hd[1] * (kh / k) * (kh / k)).min(1.0);
    }
    p[4] = if here >= 0.0 { 1.0 + 1.5 * here } else { (1.0 + here).max(0.1) };
    p[2] = hd[2] * (DRIER_SPREAD * grain(xm, ym, born, &PATCH_MM, DRIER_SEED)).exp();
    p
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
        let mm = self.px_mm();
        let here = uneven(&hd, (i % self.f.w + self.f.x0) as f32 * mm, (i / self.f.w + self.f.y0) as f32 * mm, born as f32);
        let v0 = THROUGH * COAT_UM / one_coat_minutes(self.engine, drain, &here);
        let below = self.skin.top[i];
        let p = Packet { h: add_um, s: 0.0, x: 0.0, n: 0.0, w: 0.0, born: born as f32, v0, stiff: here[1], binder: binder_share(&here), slope: 0.0, below };
        let at = self.skin.push(p);
        self.skin.top[i] = at;
    }

    /// The clock moves from `start` to `end` minutes: step the fronts, the
    /// skins and the wrinkles at every tick (`TICK_MIN`) in between.
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
    /// band-passed there, settled into ridges that bend until their slopes
    /// are as steep every way, each ridge grown as tall as the relief along
    /// it already stands (so a few gather into folds), scaled to unit rms
    /// slope over a fold's reach, is the pattern; each
    /// pixel's bulk takes as much of it as brings the slopes to what its
    /// skin asks for, counting the slopes it already has, and the
    /// compression of every layer of its plate is relieved by what it
    /// took. `site` covers the box `bx` (buffer pixels); the pass works
    /// over `pb`, that box padded by the widest window. Not `draw`: it only
    /// measures the slopes.
    fn wrinkle(&mut self, site: &[Site], bx: (usize, usize, usize, usize), pb: (usize, usize, usize, usize), bands: Vec<u8>, draw: bool) {
        let w = self.f.w;
        let (pw, ph) = (pb.2 - pb.0, pb.3 - pb.1);
        let (bw, bh) = (bx.2 - bx.0, bx.3 - bx.1);
        let seed = 0x5a1d_c0de ^ self.f.full_w as u64;
        let dx_um = self.px_mm() * 1e3;
        let soft: Vec<usize> = (0..bw * bh).filter(|&k| site[k].at != NONE).collect();
        let padded = |k: usize| (bx.1 + k / bw - pb.1) * pw + bx.0 + k % bw - pb.0;
        let mut scratch = take(24);
        let mut buffers = scratch.drain(..);
        let mut buffer = |n: usize| {
            let mut b = buffers.next().unwrap();
            b.resize(n, 0.0);
            b
        };
        // the wrinkle already there (the soft bulk's displacement) and the
        // imperfection it grows from, over the padded box: the plate's
        // mid-surface as it cured, the relief it cured over less half its
        // thickness (a plate is thinner, and its mid-surface higher, where
        // its paint cured slower)
        let mut z = buffer(pw * ph);
        z.par_iter_mut().enumerate().for_each(|(k, z)| *z = self.height[(pb.1 + k / pw) * w + pb.0 + k % pw]);
        let mut wf = buffer(pw * ph);
        let mut need = buffer(pw * ph);
        let mut press = buffer(pw * ph);
        let mut skin = buffer(pw * ph);
        let mut msq = buffer(pw * ph);
        let mm = self.px_mm();
        let mut born = vec![0.0f32; pw * ph];
        for &k in &soft {
            let p = padded(k);
            born[p] = self.skin.pool[site[k].at as usize].born;
            wf[p] = self.skin.pool[site[k].at as usize].w;
            z[p] -= wf[p] + 0.5 * site[k].plate;
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
        let mut took: Vec<(usize, f32, bool, f32)> = Vec::new();
        let (mut tmp, mut mr, mut m2r, mut mi, mut near, mut far, mut rho, mut wrinkled) = (buffer(0), buffer(0), buffer(0), buffer(0), buffer(0), buffer(0), buffer(0), buffer(0));
        let (mut zr, mut px, mut py, mut dot) = (buffer(pw * ph), buffer(pw * ph), buffer(pw * ph), buffer(pw * ph));
        let (mut shape, mut rms) = (buffer(0), buffer(0));
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
            zr.par_iter_mut().zip(&z).zip(&skin).for_each(|((v, z), s)| *v = s * z);
            blur2(&zr, r, &mut near);
            over(&mut near, &mr);
            blur2(&zr, 2 * r, &mut far);
            over(&mut far, &m2r);
            near.par_iter_mut().zip(&far).for_each(|(n, f)| *n -= f);
            // the imperfection's slopes, their root mean square over a
            // wavelength
            let rms_slope = |v: &[f32], px: &mut Vec<f32>, py: &mut Vec<f32>, dot: &mut Vec<f32>, out: &mut Vec<f32>, blur2: &mut dyn FnMut(&[f32], usize, &mut Vec<f32>), radius: usize, within: &mut Vec<f32>| {
                gradient(v, px, py);
                dot.par_iter_mut().zip(&*px).zip(&*py).zip(&inner).for_each(|(((d, x), y), m)| *d = m * (x * x + y * y));
                blur2(dot, radius, out);
                blur2(&inner, radius, within);
                over(out, within);
                // (no slope within a window, or no interior to measure one
                // on: nothing to scale, and the bulk there stays)
                out.par_iter_mut().for_each(|f| *f = if *f > 1e-12 { f.sqrt() } else { 0.0 });
            };
            rms_slope(&near, &mut px, &mut py, &mut dot, &mut far, &mut blur2, 2 * r, &mut rms);
            // the slopes the skin has at this wavelength, brush marks and
            // wrinkle together: what its yielded bulk holds against. The
            // measurement is kept on the packet, and the skin asks at it.
            dot.par_iter_mut().zip(&wx).zip(&wy).zip(&inner).for_each(|(((d, x), y), m)| *d = m * (x * x + y * y));
            blur2(&dot, 2 * r, &mut wrinkled);
            over(&mut wrinkled, &mi);
            // each pixel asks at this band its share of what it asks for;
            // one short of its threshold holds at every band
            let gain = band_gain(r);
            for &k in &soft {
                let (s, p) = (site[k], padded(k));
                let share = s.share(band);
                let ask = if share > 0.0 {
                    let slope = (far[p] / gain).powi(2) + wrinkled[p].max(0.0);
                    msq[p] += share * slope;
                    let hold = if slope > 0.0 { s.hold.min(s.yielded / slope.sqrt()) } else { s.hold };
                    press[p] = if s.cap > 0.0 && s.bend + hold > 0.0 { s.nf / (s.bend + hold) } else { 0.0 };
                    if s.cap > 0.0 { s.ask(slope.sqrt()) } else { 0.0 }
                } else {
                    press[p] = 0.0;
                    s.delta
                };
                need[p] = if ask > 0.0 { share * ask } else { ask };
            }
            // the imperfection the wrinkle grows from: the relief and the
            // roughness every surface has (`SEED_UM`, `DOMAIN_UM`), which
            // seeds where the ridges start and which way they run but is
            // not a slope the paste holds against
            if !draw {
                continue;
            }
            let lam_mm = BAND_LAMBDA * r as f32 * mm;
            let place = |k: usize| ((pb.0 + k % pw + self.f.x0) as f32 * mm, (pb.1 + k / pw + self.f.y0) as f32 * mm);
            zr.par_iter_mut().enumerate().for_each(|(k, v)| {
                if skin[k] == 0.0 {
                    *v = 0.0;
                    return;
                }
                let (xm, ym) = place(k);
                let film = seed ^ ((born[k] / FILM_MIN).floor() as i64 as u64).wrapping_mul(0x9e37_79b9);
                *v = z[k] + SEED_UM * grain(xm, ym, born[k], &[0.5 * lam_mm, lam_mm, 2.0 * lam_mm], seed) + DOMAIN_UM * domains(xm, ym, lam_mm, film);
            });
            blur2(&zr, r, &mut near);
            over(&mut near, &mr);
            blur2(&zr, 2 * r, &mut far);
            over(&mut far, &m2r);
            near.par_iter_mut().zip(&far).for_each(|(n, f)| *n -= f);
            // the shape the skin has at this band, its imperfection and its
            // wrinkle together: its slopes' root mean square over a
            // wavelength (its scale) and their projection on the wrinkle's
            // (the wrinkle as the pixels whose slopes count have it: one
            // held at the skin's edge moves with the wrinkle around it, not
            // with its own)
            zr.par_iter_mut().zip(&inner).zip(&wf).for_each(|((v, m), w)| *v = m * w);
            blur2(&inner, r, &mut rms);
            blur2(&zr, r, &mut shape);
            over(&mut shape, &rms);
            blur2(&zr, 2 * r, &mut rms);
            over(&mut rms, &mi);
            shape.par_iter_mut().zip(&rms).zip(&near).zip(&skin).for_each(|(((v, f), n), s)| *v = s * (*v + n - f));
            // a skin can't ripple two ways at one place without stretching
            // (a bump has Gaussian curvature; a ripple only bends about a
            // line), so each place keeps the one way its shape is
            // strongest in: the shape settles as under an effective
            // Swift–Hohenberg equation (Stoop et al. 2015), its wavelength
            // kept and its height saturating (a difference of
            // Gaussians at σ and 2σ peaks at a wavelength of 2πσ / 0.961;
            // Gaussians, unlike boxes, favor no direction)
            let sigma = BAND_LAMBDA * r as f32 * 0.961 / std::f32::consts::TAU;
            // (over the pixels drawn at this band, and as far around them
            // as their windows reach)
            let reach = 4 * r + 2 + (6.0 * sigma * (1.0 + 1.0 / r as f32)).ceil() as usize;
            let sb = soft.iter().filter(|&&k| site[k].share(band) > 0.0).fold((pw, ph, 0, 0), |b, &k| {
                let p = padded(k);
                let (x, y) = (p % pw, p / pw);
                (b.0.min(x.saturating_sub(reach)), b.1.min(y.saturating_sub(reach)), b.2.max((x + reach + 1).min(pw)), b.3.max((y + reach + 1).min(ph)))
            });
            let within = |k: usize| (sb.0..sb.2).contains(&(k % pw)) && (sb.1..sb.3).contains(&(k / pw));
            // (each place settles at its own wavelength, between the band's
            // neighbors': a skin's wavelength changes from place to place
            // with its thickness and its paste, and the spacing of its ridges
            // with it. The wavelengths of the pixels drawn at the band, over
            // two of its wavelengths around)
            let (mut lam, mut has) = (vec![0.0f32; pw * ph], vec![0.0f32; pw * ph]);
            for &k in &soft {
                let s = site[k];
                if s.share(band) > 0.0 {
                    let p = padded(k);
                    lam[p] = s.lam;
                    has[p] = 1.0;
                }
            }
            let (mut sig, mut weight) = (Vec::new(), Vec::new());
            blur2(&lam, 2 * r, &mut sig);
            blur2(&has, 2 * r, &mut weight);
            sig.par_iter_mut().zip(&weight).for_each(|(l, n)| *l = BAND_LAMBDA * 0.961 / std::f32::consts::TAU * if *n > 1e-6 { *l / n } else { r as f32 });
            gauss_into(&skin, pw, ph, sb, &sig, 1.0, &mut px, &mut zr);
            gauss_into(&skin, pw, ph, sb, &sig, 2.0, &mut px, &mut wrinkled);
            // (a band's pattern is the finest band's r times over: it
            // settles on a grid r times coarser, where it is the finest
            // band's, and is drawn back up)
            let (cw, ch) = ((sb.2 - sb.0).div_ceil(r), (sb.3 - sb.1).div_ceil(r));
            let down = |v: &[f32]| -> Vec<f32> {
                (0..cw * ch).into_par_iter().map(|c| {
                    let (x0, y0) = (sb.0 + c % cw * r, sb.1 + c / cw * r);
                    let (x1, y1) = ((x0 + r).min(sb.2), (y0 + r).min(sb.3));
                    let sum: f32 = (y0..y1).map(|y| v[y * pw + x0..y * pw + x1].iter().sum::<f32>()).sum();
                    sum / ((x1 - x0) * (y1 - y0)) as f32
                }).collect()
            };
            let box2 = |v: &[f32], out: &mut Vec<f32>, t: &mut Vec<f32>| {
                crate::surface::box_blur_into(v, cw, ch, 2, t);
                crate::surface::box_blur_into(t, cw, ch, 2, out);
            };
            // (the paste under the skin thins as it flows: it moves from
            // under a trough to under a crest at a rate that grows as the
            // pressure to the power 1/n (Herschel–Bulkley), and the plate
            // presses with N q² A, so a crest taller than those around it
            // grows faster than in proportion and takes the compression
            // first, as a skin past its onset gathers into folds while the
            // wrinkles beside them relax (Pocivavsek et al. 2008; Brau et
            // al. 2011). The settle says where the ridges run; each grows as
            // high as the relief under it already is, over that of `FOLD`
            // wavelengths around, to the power 1/n)
            let fold = 2 * r * FOLD;
            dot.par_iter_mut().zip(&shape).zip(&inner).for_each(|((d, v), m)| *d = m * v * v);
            blur2(&dot, r, &mut near);
            blur2(&inner, r, &mut far);
            over(&mut near, &far);
            blur2(&dot, fold, &mut rho);
            blur2(&inner, fold, &mut far);
            over(&mut rho, &far);
            // (and some ridges faster than others: `FOLD_SPREAD`)
            let grows: Vec<f32> = near.par_iter().zip(&rho).enumerate().map(|(k, (h, around))| {
                let g = if *around > 1e-12 { (h / around).powf(0.5 / FLOW_INDEX) } else { 1.0 };
                if skin[k] == 0.0 {
                    return g;
                }
                let (xm, ym) = place(k);
                g * (FOLD_SPREAD * grain(xm, ym, born[k], &[1.5 * FOLD_LAMBDAS * lam_mm, 0.75 * FOLD_LAMBDAS * lam_mm], FOLD_SEED)).exp()
            }).collect();
            let cm = down(&skin);
            let mut cv = down(&shape);
            let (cs, cb): (Vec<f32>, _) = (down(&sig).into_iter().map(|s| s / r as f32).collect(), (0, 0, cw, ch));
            let (mut ct, mut c1, mut c2, mut cn, mut cf, mut cr, mut cmr) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
            gauss_into(&cm, cw, ch, cb, &cs, 1.0, &mut ct, &mut c1);
            gauss_into(&cm, cw, ch, cb, &cs, 2.0, &mut ct, &mut c2);
            box2(&cm, &mut cmr, &mut ct);
            let mut settle = |cv: &mut Vec<f32>| {
                gauss_into(cv, cw, ch, cb, &cs, 1.0, &mut ct, &mut cn);
                gauss_into(cv, cw, ch, cb, &cs, 2.0, &mut ct, &mut cf);
                cn.par_iter_mut().zip(&cf).zip(&c1).zip(&c2).for_each(|(((n, f), a), b)| *n = if *a > 1e-6 && *b > 1e-6 { *n / a - f / b } else { 0.0 });
                cf.par_iter_mut().zip(&cn).zip(&cm).for_each(|((d, n), m)| *d = m * n * n);
                box2(&cf, &mut cr, &mut ct);
                cv.par_iter_mut().zip(&cn).zip(&cr).zip(&cmr).zip(&cm).for_each(|((((v, n), f), d), m)| {
                    let f = if *d > 1e-6 { f / d } else { 0.0 };
                    *v = if f > 1e-12 { m * (SETTLE_GAIN * n / f.sqrt()).tanh() } else { 0.0 };
                });
            };
            for _ in 0..SETTLE - FOLD_SETTLE {
                settle(&mut cv);
            }
            // (folds form one at a time, as cracks do (`crate::crack`): a
            // fold starts where the skin is most compressed and its relief
            // stands most out of its surroundings, runs along the way the
            // ridges run while the skin ahead is still compressed enough
            // (it runs on at less than it took to start), and takes the
            // compression off the skin beside it, gathering it into its own
            // slopes: one ridge between two troughs along its length. The
            // next fold starts only where enough compression is left, so a
            // few long folds stand over calm ground and later ones stop
            // where they meet them. The pattern settles its last
            // `FOLD_SETTLE` steps around the folds, so the ridges beside a
            // fold line up with it. On the band's coarse grid, a cell to a
            // pixel of the finest band)
            let mut fold_need = need.clone();
            {
                let n0 = down(&need.iter().map(|v| v.max(0.0)).collect::<Vec<f32>>());
                let (ig, sh) = (down(&grows), down(&shape));
                let (mut pressed, mut spare) = (Vec::new(), Vec::new());
                box2(&down(&press), &mut pressed, &mut spare);
                let grad = |c: usize| {
                    let (x, y) = (c % cw, c / cw);
                    let gx = 0.5 * (cv[y * cw + (x + 1).min(cw - 1)] - cv[y * cw + x.saturating_sub(1)]);
                    let gy = 0.5 * (cv[(y + 1).min(ch - 1) * cw + x] - cv[y.saturating_sub(1) * cw + x]);
                    (gx, gy)
                };
                let mut tensor: [Vec<f32>; 3] = [
                    (0..cw * ch).into_par_iter().map(|c| grad(c).0.powi(2)).collect(),
                    (0..cw * ch).into_par_iter().map(|c| grad(c).1.powi(2)).collect(),
                    (0..cw * ch).into_par_iter().map(|c| grad(c).0 * grad(c).1).collect(),
                ];
                for t in &mut tensor {
                    for _ in 0..2 {
                        let v = t.clone();
                        box2(&v, t, &mut spare);
                    }
                }
                let [txx, tyy, txy] = tensor;
                let tangent = |i: usize| {
                    let th = 0.5 * (2.0 * txy[i]).atan2(txx[i] - tyy[i]);
                    (-th.sin(), th.cos())
                };
                let mut left = n0.clone();
                let mut dep = vec![0.0f32; cw * ch];
                let mut owner = vec![0u32; cw * ch];
                let (mut near_fold, mut way) = (vec![f32::INFINITY; cw * ch], vec![0.0f32; cw * ch]);
                let l = FOLD_REACH * BAND_LAMBDA;
                let reach = 3.0 * l;
                let half = 0.5 * FOLD_WIDTH * BAND_LAMBDA;
                let relief = |d: f32| {
                    let x = d / l;
                    ((1.0 / (1.0 + x * x) - 0.1) / 0.9).max(0.0)
                };
                let cell = |x: f32, y: f32| (x >= 0.0 && y >= 0.0 && (x as usize) < cw && (y as usize) < ch).then(|| y as usize * cw + x as usize);
                // (it keeps to the crest of the relief it grows from, or the
                // bottom of a trough: of the places across it, where the
                // relief peaks, to within a cell)
                let crest = |x: f32, y: f32, hx: f32, hy: f32, up: f32| {
                    let f = |k: f32| {
                        let (fx, fy) = ((x - k * hy - 0.5).clamp(0.0, (cw - 1) as f32), (y + k * hx - 0.5).clamp(0.0, (ch - 1) as f32));
                        let (i, j, tx, ty) = (fx as usize, fy as usize, fx.fract(), fy.fract());
                        let at = |i: usize, j: usize| sh[j.min(ch - 1) * cw + i.min(cw - 1)];
                        let top = at(i, j) + (at(i + 1, j) - at(i, j)) * tx;
                        let bottom = at(i, j + 1) + (at(i + 1, j + 1) - at(i, j + 1)) * tx;
                        up * (top + (bottom - top) * ty)
                    };
                    let (a, b, c) = (f(-1.0), f(0.0), f(1.0));
                    let den = a - 2.0 * b + c;
                    let d = if den < 0.0 { (0.5 * (a - c) / den).clamp(-0.5, 0.5) } else { 0.0 };
                    (x - d * hy, y + d * hx)
                };
                let mut heap: std::collections::BinaryHeap<(u32, std::cmp::Reverse<u32>)> = (0..cw * ch).filter(|&c| cm[c] > 0.0 && pressed[c] >= 1.0).map(|c| ((pressed[c] * ig[c]).to_bits(), std::cmp::Reverse(c as u32))).collect();
                let (mut mark, mut best) = (vec![0u32; cw * ch], vec![0.0f32; cw * ch]);
                let (mut mark2, mut best2) = (vec![0u32; cw * ch], vec![0.0f32; cw * ch]);
                let mut seen = vec![0u32; cw * ch];
                let mut touched: Vec<usize> = Vec::new();
                let mut fold = 0u32;
                while let Some((rank, std::cmp::Reverse(c))) = heap.pop() {
                    let c = c as usize;
                    if owner[c] != 0 || pressed[c] < 1.0 {
                        continue;
                    }
                    let q = pressed[c] * ig[c];
                    if q.to_bits() < rank {
                        // (relieved since it was queued: queued again at its new rank)
                        heap.push((q.to_bits(), std::cmp::Reverse(c as u32)));
                        continue;
                    }
                    fold += 1;
                    let up = if sh[c] >= 0.0 { 1.0 } else { -1.0 };
                    let (t0, t1) = tangent(c);
                    let start = crest((c % cw) as f32 + 0.5, (c / cw) as f32 + 0.5, t0, t1, up);
                    let mut path = vec![start];
                    seen[c] = fold;
                    for side in [1.0f32, -1.0] {
                        let (mut x, mut y, mut hx, mut hy) = (start.0, start.1, side * t0, side * t1);
                        let mut run = Vec::new();
                        loop {
                            let (x1, y1) = crest(x + hx, y + hy, hx, hy, up);
                            let Some(j) = cell(x1, y1) else { break };
                            // (and stops where it meets another fold, or
                            // comes round to its own path)
                            if cm[j] == 0.0 || pressed[j] < FOLD_RUN || owner[j] != 0 || (seen[j] == fold && cell(x, y) != Some(j)) || run.len() > cw + ch {
                                break;
                            }
                            seen[j] = fold;
                            (x, y) = (x1, y1);
                            run.push((x, y));
                            // (it turns with the ridges, as a crack turns with
                            // the stress, a share of the way each step)
                            let (t0, t1) = tangent(j);
                            let (t0, t1) = if t0 * hx + t1 * hy < 0.0 { (-t0, -t1) } else { (t0, t1) };
                            let (nx, ny) = (hx + FOLD_STEER * (t0 - hx), hy + FOLD_STEER * (t1 - hy));
                            let n = (nx * nx + ny * ny).sqrt();
                            if n < 1e-6 {
                                break;
                            }
                            (hx, hy) = (nx / n, ny / n);
                        }
                        if side > 0.0 {
                            path.extend(run);
                        } else {
                            run.reverse();
                            path.splice(0..0, run);
                        }
                    }
                    // the distance of each place near it from its path:
                    // within a fold's width it is the fold's own, and what
                    // it relieves reaches along chords of its path (the
                    // relief reaches far past the jags of a path on the grid)
                    let near_path = |pts: &[(f32, f32)], within: f32, mark: &mut Vec<u32>, best: &mut Vec<f32>, touched: &mut Vec<usize>| {
                        touched.clear();
                        let segs: Vec<((f32, f32), (f32, f32))> = if pts.len() == 1 { vec![(pts[0], pts[0])] } else { pts.windows(2).map(|w| (w[0], w[1])).collect() };
                        for ((ax, ay), (bx, by)) in segs {
                            let (dx, dy) = (bx - ax, by - ay);
                            let l2 = dx * dx + dy * dy;
                            let (x0, x1) = ((ax.min(bx) - within).max(0.0) as usize, ((ax.max(bx) + within) as usize).min(cw - 1));
                            let (y0, y1) = ((ay.min(by) - within).max(0.0) as usize, ((ay.max(by) + within) as usize).min(ch - 1));
                            for yy in y0..=y1 {
                                for xx in x0..=x1 {
                                    let (cx, cy) = (xx as f32 + 0.5, yy as f32 + 0.5);
                                    let t = if l2 > 0.0 { (((cx - ax) * dx + (cy - ay) * dy) / l2).clamp(0.0, 1.0) } else { 0.0 };
                                    let (ex, ey) = (cx - ax - t * dx, cy - ay - t * dy);
                                    let d2 = ex * ex + ey * ey;
                                    if d2 >= within * within {
                                        continue;
                                    }
                                    let i = yy * cw + xx;
                                    if mark[i] != fold {
                                        mark[i] = fold;
                                        best[i] = d2;
                                        touched.push(i);
                                    } else if d2 < best[i] {
                                        best[i] = d2;
                                    }
                                }
                            }
                        }
                    };
                    let stride = ((0.5 * reach) as usize).max(1);
                    let mut knots: Vec<(f32, f32)> = path.iter().step_by(stride).copied().collect();
                    if (path.len() - 1) % stride != 0 {
                        knots.push(*path.last().unwrap());
                    }
                    near_path(&knots, reach, &mut mark, &mut best, &mut touched);
                    let mut released = 0.0f32;
                    for &i in &touched {
                        let off = relief(best[i].sqrt());
                        let take = left[i] * off;
                        left[i] -= take;
                        released += take;
                        pressed[i] *= 1.0 - off;
                    }
                    near_path(&path, BAND_LAMBDA, &mut mark2, &mut best2, &mut touched);
                    let mut foot = Vec::new();
                    for &i in &touched {
                        let d = best2[i].sqrt();
                        if d < near_fold[i] {
                            near_fold[i] = d;
                            way[i] = up;
                        }
                        if d <= half && owner[i] == 0 && cm[i] > 0.0 {
                            owner[i] = fold;
                            foot.push(i);
                        }
                    }
                    if !foot.is_empty() {
                        let each = released / foot.len() as f32;
                        for &i in &foot {
                            dep[i] += each;
                        }
                    }
                }
                // (a fold's ridge, with a trough either side of it, as
                // steep-sided as the settled pattern's, fading into the
                // pattern around it over the next half wavelength)
                let pin = |cv: &mut Vec<f32>| {
                    cv.par_iter_mut().zip(&near_fold).zip(&way).zip(&cm).for_each(|(((v, d), s), m)| {
                        let h = 0.5 * BAND_LAMBDA;
                        if *d >= BAND_LAMBDA {
                            return;
                        }
                        let w = if *d <= h { 1.0 } else { 0.5 * (1.0 + (std::f32::consts::PI * (d - h) / h).cos()) };
                        let c = (std::f32::consts::TAU * d / BAND_LAMBDA).cos();
                        *v += w * (m * s * c.signum() * c.abs().sqrt() - *v);
                    });
                };
                pin(&mut cv);
                for _ in 0..FOLD_SETTLE {
                    settle(&mut cv);
                    pin(&mut cv);
                }
                fold_need.par_chunks_mut(pw).enumerate().take(sb.3).skip(sb.1).for_each(|(y, row)| {
                    let j = (y - sb.1) / r;
                    for (x, v) in row.iter_mut().enumerate().take(sb.2).skip(sb.0) {
                        if skin[y * pw + x] == 0.0 {
                            continue;
                        }
                        let c = j * cw + (x - sb.0) / r;
                        // (a fold runs on into skin short of wrinkling, and
                        // wrinkles it with the compression it gathered)
                        let keep = if n0[c] > 0.0 { left[c] / n0[c] } else { 1.0 };
                        let v0 = if owner[c] != 0 { v.max(0.0) } else { *v };
                        *v = if v0 > 0.0 { v0 * keep } else { v0 } + dep[c];
                    }
                });
            }
            shape.par_chunks_mut(pw).enumerate().take(sb.3).skip(sb.1).for_each(|(y, row)| {
                let at = |i: usize, j: usize| cv[j.min(ch - 1) * cw + i.min(cw - 1)];
                let fy = (((y - sb.1) as f32 + 0.5) / r as f32 - 0.5).max(0.0);
                let (j, ty) = (fy as usize, fy.fract());
                for (x, v) in row.iter_mut().enumerate().take(sb.2).skip(sb.0) {
                    let fx = (((x - sb.0) as f32 + 0.5) / r as f32 - 0.5).max(0.0);
                    let (i, tx) = (fx as usize, fx.fract());
                    let top = at(i, j) + (at(i + 1, j) - at(i, j)) * tx;
                    let bottom = at(i, j + 1) + (at(i + 1, j + 1) - at(i, j + 1)) * tx;
                    *v = skin[y * pw + x] * (top + (bottom - top) * ty);
                }
            });
            // the skin is compressed the same every way, and a ripple
            // relieves it only across itself (½ w_x² takes up strain along
            // x, ½ w_y² along y): a skin rippled across a knife's drag is
            // still compressed along it, and its ridges bend, as a
            // herringbone's do (Chen and Hutchinson 2004; Audoly and
            // Boudaoud 2008), until their slopes are as steep both ways.
            // Each place moves across its ridges by as much as tilts them
            // to make up the lesser way's share of the slopes (their
            // tensor over two wavelengths), as the paint under the skin
            // lies higher or lower over `BEND` wavelengths: the ridges bend
            // where it was laid unevenly
            gradient(&shape, &mut px, &mut py);
            dot.par_iter_mut().zip(&px).zip(&inner).for_each(|((d, x), m)| *d = m * x * x);
            blur2(&dot, 4 * r, &mut near);
            dot.par_iter_mut().zip(&py).zip(&inner).for_each(|((d, y), m)| *d = m * y * y);
            blur2(&dot, 4 * r, &mut far);
            dot.par_iter_mut().zip(&px).zip(&py).zip(&inner).for_each(|(((d, x), y), m)| *d = m * x * y);
            blur2(&dot, 4 * r, &mut rho);
            let rb = BEND * r;
            let (mut a1, mut a2, mut s1, mut s2) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
            dot.par_iter_mut().zip(&z).zip(&skin).for_each(|((d, z), s)| *d = s * z);
            blur2(&dot, rb, &mut a1);
            blur2(&dot, 2 * rb, &mut a2);
            blur2(&skin, rb, &mut s1);
            blur2(&skin, 2 * rb, &mut s2);
            let lift: Vec<f32> = (0..pw * ph).into_par_iter().map(|k| if s1[k] > 1e-6 && s2[k] > 1e-6 { a1[k] / s1[k] - a2[k] / s2[k] } else { 0.0 }).collect();
            let (sum, count) = (sb.1..sb.3).flat_map(|y| (sb.0..sb.2).map(move |x| y * pw + x)).filter(|&k| skin[k] > 0.0).fold((0.0f64, 0usize), |(s, c), k| (s + (lift[k] as f64).powi(2), c + 1));
            let spread = if count > 0 { (sum / count as f64).sqrt() as f32 } else { 0.0 };
            // (a ridge's axis has no sign: each place moves across its
            // ridges the same way round as the band's ridges lie on the
            // whole, or a ridge would tear wherever its axis turns past
            // the grid's)
            let whole = (sb.1..sb.3).flat_map(|y| (sb.0..sb.2).map(move |x| y * pw + x)).filter(|&k| skin[k] > 0.0).fold((0.0f64, 0.0f64, 0.0f64), |(a, b, c), k| (a + near[k] as f64, b + far[k] as f64, c + rho[k] as f64));
            let th0 = (0.5 * (2.0 * whole.2).atan2(whole.0 - whole.1)) as f32;
            let settled = shape.clone();
            shape.par_chunks_mut(pw).enumerate().take(sb.3).skip(sb.1).for_each(|(y, row)| {
                for (x, v) in row.iter_mut().enumerate().take(sb.2).skip(sb.0) {
                    let k = y * pw + x;
                    let (a, b, c) = (near[k], far[k], rho[k]);
                    let half = (0.25 * (a - b) * (a - b) + c * c).sqrt();
                    let (big, small) = (0.5 * (a + b) + half, 0.5 * (a + b) - half);
                    if skin[k] == 0.0 || big <= 0.0 || spread <= 0.0 {
                        continue;
                    }
                    // (a ridge tilted by θ has the share sin² θ of its
                    // slopes along where it ran)
                    let sin2 = ((big - small.max(0.0)) / (2.0 * big)).clamp(0.0, 0.49);
                    let th = 0.5 * (2.0 * c).atan2(a - b);
                    let th = if (th - th0).cos() < 0.0 { th + std::f32::consts::PI } else { th };
                    let u = (sin2 / (1.0 - sin2)).sqrt() * rb as f32 * BAND_LAMBDA / std::f32::consts::TAU * lift[k] / spread;
                    let (fx, fy) = ((x as f32 + u * th.cos()).clamp(0.0, (pw - 1) as f32), (y as f32 + u * th.sin()).clamp(0.0, (ph - 1) as f32));
                    let (i, j, tx, ty) = (fx as usize, fy as usize, fx.fract(), fy.fract());
                    let at = |i: usize, j: usize| settled[j.min(ph - 1) * pw + i.min(pw - 1)];
                    let top = at(i, j) + (at(i + 1, j) - at(i, j)) * tx;
                    let bottom = at(i, j + 1) + (at(i + 1, j + 1) - at(i, j + 1)) * tx;
                    *v = skin[k] * (top + (bottom - top) * ty);
                }
            });
            // and is drawn at its own wavelengths alone: the saturated
            // ridges' edges are finer than the finest bands' pixels can hold
            gauss_into(&shape, pw, ph, sb, &sig, 1.0, &mut px, &mut near);
            gauss_into(&shape, pw, ph, sb, &sig, 2.0, &mut px, &mut far);
            shape.par_iter_mut().zip(&near).zip(&far).zip(&zr).zip(&wrinkled).zip(&skin).enumerate().for_each(|(k, (((((v, n), f), m1), m2), s))| {
                if within(k) {
                    *v = if *m1 > 1e-6 && *m2 > 1e-6 { s * (n / m1 - f / m2) } else { 0.0 };
                }
            });
            rms_slope(&shape, &mut px, &mut py, &mut dot, &mut far, &mut blur2, fold, &mut wrinkled);
            dot.par_iter_mut().zip(&px).zip(&py).zip(&wx).zip(&wy).zip(&far).zip(&inner).for_each(|((((((d, px), py), wx), wy), f), m)| *d = if *f > 0.0 { m * (px * wx + py * wy) / f } else { 0.0 });
            blur2(&dot, 2 * r, &mut rho);
            over(&mut rho, &mi);
            // a yielded paste gives way point by point, where the skin
            // presses on it past its yield stress: the plate presses in
            // proportion to the wrinkle's height there, so the paste gives
            // under the highest crests and the deepest troughs first and
            // holds between them (`Site::gives`, against the shape's root
            // mean square height over a wavelength). The skin makes only the
            // slopes the paste lets it make; the compression it can't spend
            // stays, and drives the paste harder next time, so a few folds
            // take the compression first and the skin between them follows
            // as it builds
            dot.par_iter_mut().zip(&shape).zip(&inner).for_each(|((d, v), m)| *d = m * v * v);
            blur2(&dot, 2 * r, &mut rms);
            over(&mut rms, &mi);
            for &k in &soft {
                let p = padded(k);
                let at = site[k].gives * rms[p].max(0.0).sqrt();
                shape[p] = shape[p].signum() * (shape[p].abs() - at).max(0.0);
            }
            rms_slope(&shape, &mut px, &mut py, &mut dot, &mut rms, &mut blur2, fold, &mut wrinkled);
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
            // (what each place draws: what it asks for, the folds having
            // gathered it from the skin around them)
            blur2(&fold_need, 2 * r, &mut dot);
            // a sinusoid of unit rms slope has an amplitude of λ √2 / 2π:
            // the pattern is bounded by `TALL` of it, so the wall of a
            // stroke at the skin's edge is not a wrinkle's worth of relief
            let cap = TALL * BAND_LAMBDA * r as f32 * dx_um * std::f32::consts::FRAC_1_SQRT_2 / std::f32::consts::PI;
            for &k in &soft {
                let s = site[k];
                if s.share(band) == 0.0 {
                    continue;
                }
                let p = padded(k);
                let delta = dot[p];
                let asked = need[p];
                let relieved = asked > 0.0 && mr[p] > 0.0;
                if delta <= 0.0 && !relieved {
                    continue;
                }
                // slopes add: (ρ + a)² − ρ² = Δ for the amount a of the
                // unit-slope shape that brings the mean square slope up by
                // Δ, with ρ its projection on the slopes there. A plate under
                // compression deflects along its imperfection, the same way
                // up, so the wrinkle only grows the way its shape points;
                // where the slopes there run against the shape (at the
                // skin's edge) it grows as from level, so a is continuous
                // across the skin wherever ρ passes zero (a pixel with no
                // shape within its window stays where it is)
                let rho = rho[p].max(0.0);
                let a = if delta > 0.0 && far[p] > 0.0 { (rho * rho + delta).sqrt() - rho } else { 0.0 };
                // (of the slopes asked for, the share the paste gave way to)
                let gave = if far[p] > 0.0 { (rms[p] / far[p]).min(1.0).powi(2) } else { 0.0 };
                if a != 0.0 {
                    dw[p] += a * (shape[p] / far[p]).clamp(-cap, cap);
                    msq[p] += gave * delta;
                }
                took.push((k, a, relieved, gave * asked));
            }
        }
        for &k in &soft {
            let m = msq[padded(k)].sqrt();
            self.skin.pool[site[k].at as usize].slope = if m > 0.0 { m } else { FLAT };
        }
        if !took.is_empty() {
            // the bulk only moves sideways: what the crests gather the
            // troughs gave, and a trough stops at the paint under it (the
            // shift that keeps the total is level, and changes no slope);
            // a pixel drawn at two bands moves once, by both
            let mut moved: Vec<usize> = took.iter().map(|&(k, ..)| k).collect();
            moved.sort_unstable();
            moved.dedup();
            let mean = (moved.iter().map(|&k| dw[padded(k)] as f64).sum::<f64>() / moved.len() as f64) as f32;
            let mut surplus = 0.0f64;
            let mut free = 0usize;
            for &k in &moved {
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
                for &k in &moved {
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
            for &(k, _, relieved, asked) in &took {
                let s = site[k];
                let i = (bx.1 + k / bw) * w + bx.0 + k % bw;
                let relief = if relieved { 0.5 * SKIN_PA / (1.0 - NU * NU) * asked * 1e-6 } else { 0.0 };
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
            }
            for &k in &moved {
                let d = dw[padded(k)];
                if d != 0.0 {
                    self.skin.pool[site[k].at as usize].w += d;
                    self.height[(bx.1 + k / bw) * w + bx.0 + k % bw] += d;
                }
            }
            self.surf_gen += 1;
            self.base = None;
        }
        give(vec![z, wf, need, press, skin, msq, inner, wx, wy, dw, tmp, mr, m2r, mi, near, far, rho, wrinkled, zr, px, py, dot, shape, rms]);
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
    /// over the tick (from its birth, if that is later), its skin takes up
    /// expansion, then the skins on soft bulk wrinkle.
    fn skin_tick(&mut self, now: f64) {
        let w = self.f.w;
        let since = self.skin.ticked;
        self.skin.ticked = now;
        let Some(bx) = self.skin_box() else { return };
        let dx = self.px_mm() * 1e-3;
        let nyq = std::f32::consts::PI / dx;
        // 1. chemistry: what each packet's front has to cross, then every
        // front, reserve and force over the tick
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
        let mut site = vec![Site { at: NONE, delta: 0.0, band: 0, next: 0, mix: 0.0, lam: 0.0, plate: 0.0, floor: 0.0, measure: false, runs: false, nf: 0.0, bend: 0.0, hold: 0.0, c: 0.0, cap: 0.0, yielded: 0.0, gives: 0.0 }; bw * bh];
        let (top, pool, engine) = (&self.skin.top, &self.skin.pool, self.engine);
        let (mm, x0, y0) = (self.px_mm(), self.f.x0, self.f.y0);
        site.par_chunks_mut(bw).enumerate().for_each(|(y, row)| {
            for (x, site) in row.iter_mut().enumerate() {
                let mut at = top[(bx.1 + y) * w + bx.0 + x];
                let (mut plate, mut force, mut over) = (0.0f32, 0.0f32, 0.0f32);
                let mut soft = NONE;
                while at != NONE {
                    let p = &pool[at as usize];
                    let thick = p.h + p.w;
                    force += p.n;
                    if p.s >= thick - DRY_UM {
                        plate += thick;
                        over = p.slope;
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
                // (a film that has just become the soft one under a skin that
                // cured through has the relief that skin was measured with)
                let slope = if p.slope != 0.0 { p.slope } else { over };
                let t = plate * 1e-6;
                let b = SKIN_PA * t * t * t / (12.0 * (1.0 - NU * NU));
                let c = SKIN_PA * t / (1.0 - NU * NU);
                let yield_pa = rheology_at(engine, p.stiff).1;
                let g = MODULUS_PER_YIELD * yield_pa;
                // the bulk's depth over a wavelength: the wrinkle only moves
                // it from trough to crest, so once the front is past the
                // film's own thickness the troughs have nothing left to give
                // and the wrinkle stays as it is, whatever a crest still holds
                let hb = (p.h - p.s) * 1e-6;
                let nf = force - SIGMA;
                if nf <= 0.0 || hb <= DRY_UM * 1e-6 || p.h + p.w - p.s <= DRY_UM {
                    continue;
                }
                // the gel's elastic hold at q, and the yielded paste's: a
                // plug flow from trough to crest through the depth it has,
                // a pressure of π τ_y / (q H) under a thin bulk and π τ_y
                // under a deep one, which the plate presses with N q² A on
                // a wrinkle of amplitude A (rms slope σ = q A / √2): a
                // membrane force of π τ_y / (√2 q σ min(1, qH)); the skin
                // beats the lesser of the two
                let elastic = |q: f32| if q * hb >= DEEP { 2.0 * g / q } else { 12.0 * g / (hb * hb * hb * q * q * q * q) };
                let yielded = |q: f32| std::f32::consts::FRAC_PI_2 * std::f32::consts::SQRT_2 * yield_pa / (q * (q * hb).min(1.0));
                let need = |q: f32| b * q * q + if slope > 0.0 { elastic(q).min(yielded(q) / slope) } else { elastic(q) };
                // the wavenumber that needs the least compression: on an
                // elastic gel (G / B)^(1/3) on deep bulk, (24 G / B H³)^(1/6)
                // on thin; on a yielded paste at the slopes measured, where
                // bending balances the plug flow, (π τ_y / 2√2 B σ)^(1/3) on
                // deep bulk and (π τ_y / √2 B H σ)^(1/4) on thin
                let qh = (g / b).cbrt();
                let mut qc = if qh * hb >= DEEP { qh } else { (24.0 * g / (b * hb * hb * hb)).powf(1.0 / 6.0).min(DEEP / hb) };
                if slope > 0.0 {
                    let deep = (std::f32::consts::FRAC_PI_4 * std::f32::consts::FRAC_1_SQRT_2 * yield_pa / (b * slope)).cbrt();
                    let thin = (std::f32::consts::FRAC_PI_2 * std::f32::consts::SQRT_2 * yield_pa / (b * hb * slope)).powf(0.25);
                    for q in [deep, thin] {
                        if q > 0.0 && q.is_finite() && need(q) < need(qc) {
                            qc = q;
                        }
                    }
                }
                // the wrinkle is drawn at its wavelength as a mix of the two
                // bands around it (by its logarithm), so a skin whose
                // wavelength changes from place to place passes from one
                // band to the next without a seam; the threshold is the
                // plate's at the wavenumber drawn. Each place strays from its
                // skin's wavelength (`JITTER`); under the finest band its
                // Gaussians alone draw it, down to `FINEST`, and the skin is
                // drawn there if it beats the threshold there (every
                // wavelength the compression beats is unstable; the least one
                // only grows fastest)
                let (xm, ym) = ((bx.0 + x + x0) as f32 * mm, (bx.1 + y + y0) as f32 * mm);
                let stray = (JITTER * grain(xm, ym, p.born, &JITTER_MM, JITTER_SEED)).exp();
                let at = (2.0 * std::f32::consts::PI / (qc.min(nyq) * dx) / BAND_LAMBDA * stray).clamp(FINEST / BAND_LAMBDA, BAND_MAX as f32);
                let i = BANDS.iter().rposition(|&b| b as f32 <= at).unwrap_or(0);
                let (band, next) = (BANDS[i], BANDS[(i + 1).min(BANDS.len() - 1)]);
                let mix = if next > band && at > band as f32 { (at / band as f32).ln() / (next as f32 / band as f32).ln() } else { 0.0 };
                let q = 2.0 * std::f32::consts::PI / (at * BAND_LAMBDA * dx);
                // von Kármán: slopes of mean square 2 (N − N_c) / C take up
                // the compression past the threshold (short of it: how far
                // the pixel is from wrinkling, for its neighbors' sake). A
                // trough stops at the paint under the bulk, so the skin asks
                // for no more slope than a wrinkle as deep as its soft bulk
                // has, q² H² / 2: a film nearly cured through holds what
                // compression it has left.
                let room = (p.h + p.w - p.s) * 1e-6;
                let mut s = Site { at: soft, delta: 0.0, band, next, mix, lam: at, plate, floor: p.s - p.h + DRY_UM, measure: false, runs: false, nf, bend: b * q * q, hold: elastic(q), c, cap: 0.5 * (q * room) * (q * room), yielded: yielded(q), gives: 0.0 };
                // a skin whose slopes are not measured asks nothing of its
                // neighbors yet; it asks for a pass when it could wrinkle
                // on a bulk that has yielded, and the pass draws at the
                // slopes it measures
                if slope > 0.0 {
                    s.delta = s.ask(slope);
                    let plateau = s.yielded / slope;
                    s.runs = s.delta > 0.0 && plateau < s.hold && 2.0 * plateau * (1.0 - slope / (slope * slope + s.delta).sqrt()) / c >= GROW * GROW;
                    if plateau < s.hold && nf > s.bend {
                        s.gives = (plateau / (nf - s.bend)).min(1.0) * (q * hb).min(1.0) / (q * room).min(1.0);
                    }
                } else if slope == 0.0 {
                    s.measure = (2.0 * (nf - s.bend) / c).min(s.cap) >= GROW * GROW;
                }
                *site = s;
            }
        });
        // a skin asks for slopes only where it has soft bulk on every side
        // to make them with: over a soft pixel whose neighbor is cured
        // through, or bare, it is held, and asks nothing (its bulk still
        // moves with the wrinkle around it, and that relieves it)
        let lone: Vec<usize> = (0..bw * bh).into_par_iter().filter(|&k| {
            let (x, y) = (k % bw, k / bw);
            let soft = |x: usize, y: usize| x < bw && y < bh && site[y * bw + x].at != NONE;
            site[k].at != NONE && !(soft(x.wrapping_sub(1), y) && soft(x + 1, y) && soft(x, y.wrapping_sub(1)) && soft(x, y + 1))
        }).collect();
        for k in lone {
            site[k] = Site { delta: 0.0, measure: false, runs: false, cap: 0.0, ..site[k] };
        }
        // 3. the wrinkles: the relief band-passed at each wavelength in
        // use, scaled to unit rms slope, is the pattern; each pixel's bulk
        // takes the slope its skin asks for, over what it has. A wavelength
        // of skin acts together, so what a pixel asks for counts averaged
        // over its wavelength, with the paint around it that is cured
        // through, or bare, asking nothing: the skin over a soft spot
        // smaller than a wavelength is held by the cured paint around it.
        // The pass runs when that average is GROW² of slope or more.
        let least = if (now / TICK_MIN).round() as u64 % DRAW_ALL == 0 { f32::MIN_POSITIVE } else { GROW * GROW };
        if site.iter().any(|s| s.at != NONE && (s.delta >= least || s.measure || s.runs)) {
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
            let mut bands: Vec<u8> = site.iter().filter(|s| s.at != NONE).flat_map(|s| [s.band, if s.mix > 0.0 { s.next } else { s.band }]).collect();
            bands.sort_unstable();
            bands.dedup();
            // (a pass runs where the skin asks for one, and over the skin
            // around it as far as its windows reach, found in tiles: there it
            // draws, and measures every skin; where the skin only has to be
            // measured, it measures and draws nothing. The rest keeps the
            // slopes it was last measured with, and what it asks for waits)
            let (tw, th) = (bw.div_ceil(TILE), bh.div_ceil(TILE));
            let mut asks = vec![0u8; tw * th];
            for (k, s) in site.iter().enumerate() {
                if s.at != NONE && s.measure {
                    asks[k / bw / TILE * tw + k % bw / TILE] = 1;
                }
            }
            // (what a place asks for, averaged over its wavelength, reaches
            // `GROW` only within a window of a place that asks for that much
            // itself: the averages are taken around those alone)
            let mut hot = vec![false; tw * th];
            for (k, s) in site.iter().enumerate() {
                if s.at != NONE && (s.delta >= least || s.runs) {
                    hot[k / bw / TILE * tw + k % bw / TILE] = true;
                }
            }
            for (tx0, ty0, tx1, ty1) in patches(&hot, tw, th, (8 * BAND_MAX).div_ceil(TILE)) {
                let (sx0, sy0, sx1, sy1) = (tx0 * TILE, ty0 * TILE, (tx1 * TILE).min(bw), (ty1 * TILE).min(bh));
                let (x0, y0) = (bx.0 + sx0 - pb.0, bx.1 + sy0 - pb.1);
                let (sw, sh) = (sx1 - sx0, sy1 - sy0);
                let part: Vec<f32> = (y0..y0 + sh).flat_map(|y| need[y * pw + x0..y * pw + x0 + sw].iter().copied()).collect();
                for &band in &bands {
                    crate::surface::box_blur_into(&part, sw, sh, 2 * band as usize, tmp);
                    crate::surface::box_blur_into(tmp, sw, sh, 2 * band as usize, avg);
                    for y in 0..sh {
                        for x in 0..sw {
                            let s = &site[(sy0 + y) * bw + sx0 + x];
                            if s.at != NONE && s.share(band) > 0.0 {
                                let a = avg[y * sw + x];
                                if a >= least || (s.runs && a > 0.0) {
                                    asks[(sy0 + y) / TILE * tw + (sx0 + x) / TILE] = 2;
                                }
                            }
                        }
                    }
                }
            }
            give(scratch);
            let marks: Vec<bool> = asks.iter().map(|&a| a > 0).collect();
            let m = (4 * BAND_MAX).div_ceil(TILE);
            let boxes = patches(&marks, tw, th, m);
            for (tx0, ty0, tx1, ty1) in boxes {
                let draw = (ty0..ty1).any(|ty| asks[ty * tw + tx0..ty * tw + tx1].contains(&2));
                let (sx0, sy0, sx1, sy1) = (tx0 * TILE, ty0 * TILE, (tx1 * TILE).min(bw), (ty1 * TILE).min(bh));
                let site: Vec<Site> = (sy0..sy1).flat_map(|y| site[y * bw + sx0..y * bw + sx1].iter().copied()).collect();
                let bx = (bx.0 + sx0, bx.1 + sy0, bx.0 + sx1, bx.1 + sy1);
                let pb = (bx.0.saturating_sub(pad), bx.1.saturating_sub(pad), (bx.2 + pad).min(w), (bx.3 + pad).min(self.f.h));
                let bands: Vec<u8> = bands.iter().copied().filter(|&band| site.iter().any(|s| s.at != NONE && s.share(band) > 0.0)).collect();
                self.wrinkle(&site, bx, pb, bands, draw);
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
        assert_eq!(uncovered.s, bare.skin.column(5 * 8).next().unwrap().s, "the uncovered half dries as the bare canvas does");
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
        // (over the middle of the film: a single pixel may lie where the
        // wrinkle's bulk crosses level)
        let tops: Vec<(f32, f32, f32)> = (24..41).flat_map(|y| (24..41).map(move |x| y * 64 + x)).filter_map(|k| c.skin.column(k).next().map(|t| (t.w, t.s, t.h))).collect();
        let moved = tops.iter().filter(|(w, s, h)| w.abs() > 1.0 && s < &(h + w)).count();
        assert!(2 * moved > tops.len(), "bulk moved under a skin: {moved} of {} pixels", tops.len());
        let (b, a, _) = paired(film(paint(1.0), 40.0), 6.0);
        assert!(a < 1.5 * b, "stiff tube paint (3000 Pa yield) holds: {b} → {a}");
        let (b, a, _) = paired(film(paint(0.1), 2.0), 6.0);
        assert!((a - b).abs() < 0.5, "a thin medium-rich film has no soft bulk left to wrinkle: {b} → {a}");
    }

    #[test]
    fn a_knife_slab_wrinkles_on_its_yielded_bulk_and_a_stiffer_one_holds() {
        let bx = (6, 6, 58, 58);
        let film = |paint: Paint, coats: f32| {
            move |c: &mut Canvas| {
                furrow(c, 3.0, 6);
                lay(c, &paint, coats, bx);
                c.dry();
            }
        };
        // 200 µm of medium-rich paint drying at lead white's pace (a coat
        // touch-dry in two days): less soft bulk than a wrinkle wavelength,
        // which an elastic layer would hold flat (22.8 N/m against 6.7 of
        // compression at day 20) and the yielded paste does not
        let slab = |stiff: f32| Paint::new([0.9; 3], 0.9, stiff).with_drying(0.8);
        let (b, a, c) = paired(film(slab(0.1), 8.0), 30.0);
        assert!(a > 3.0 * b && a > 5.0, "a 200 µm medium-rich slab wrinkles: relief {b} → {a} µm rms");
        let top = c.skin.column(32 * 64 + 32).next().unwrap();
        assert!(top.slope > 0.05 && top.w.abs() > 1.0, "measured slopes and moved bulk: {top:?}");
        // ten times the yield stress holds at that thickness
        let (b, a, _) = paired(film(slab(0.46), 8.0), 30.0);
        assert!(a < 1.5 * b, "a blotted slab (95 Pa yield) holds at 200 µm: {b} → {a}");
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
