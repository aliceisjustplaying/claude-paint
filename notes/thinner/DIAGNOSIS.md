# Thinner, phase 1: why `load` doesn't thin paint

Measured on 3d0ea66 (engine 3), 2400 px (`LIVE_WIDTH`, crates/easel/src/main.rs:46).
Rig: the Inness box, a toned ground with black (bone black) and white (lead white)
bands dried 60 days, then one `body` pass per strip (`clip=true`), then 60 more days.
"Kept" = the card's black/white luminance difference after the dried pass / before.
Scratch probe, not committed.

## Today

| pile | load 0.1 | load 0.3 | load 0.6 |
|---|---|---|---|
| wet film per pass, over white | 12 µm | 45 µm | 95 µm |
| raw sienna, kept | 46% | 13% | 6% |
| lead white, kept | 30% | 9% | 4% |
| lead white 6 + yellow ochre 1, kept | 32% | 10% | 3% |

(At 600 px raw sienna keeps 55/16/4%: resolution moves these by up to 9 points.)

## Where the amount is set, and why load barely matters

1. A dip puts `amount × full` into each bristle (`Held::load`, crates/paint/src/bristle.rs:429-438).
2. The `body` hand re-dips every 2 strokes after wiping 60% (`.dips(2, 0.56, 0.6)`, crates/paint/src/style.rs:279; wipe and load at crates/paint/src/handling.rs:754-798) at coverage 2.5 (style.rs:276). A pass never runs dry.
3. Each stroke lays `br.vol * (1 - exp(-travel/run)) * touch` (crates/paint/src/bristle.rs:1277).

So paint per area is proportional to load (12/45/95 µm ≈ 0.5/1.8/3.8 coats of 25 µm, crates/paint/src/surface.rs:24), but always at tube concentration. Load changes thickness, not pigment per volume. Tube raw sienna (hiding 0.4, crates/paint/src/palette.rs:135; S 0.296 per coat) is past the optical knee by one coat. A uniform Kubelka–Munk layer over this card keeps 43% at 0.5 coat, 21% at 1, about 8% at 1.8 and under 2% at 3.8. Loads 0.3 and 0.6 both land on the flat part of the curve. That is the review's "80% of the way to masstone at either load".

## What the target needs

`medium` scales S per coat by (1 − m) at the same volume (`Mixture::paint`, palette.rs:493-499). Optically this matches laying a (1 − m) share of the pigment, because KM depends only on S·x and K·x. Measured with raw sienna, kept after drying:

| pigment share of today | load 0.3 | load 0.6 |
|---|---|---|
| 0.5 (medium 0.5) | 32% | 14% |
| 0.25 (medium 0.75) | **54%** | 25% |
| 0.15 (medium 0.85) | 64% | 43% |
| 0.10 (medium 0.9) | 71% | **52%** |

Keeping ≥ 50% needs ≤ 0.25 of today's pigment at load 0.3 and ≤ 0.10 at load 0.6. At both loads that is about 0.4 coat of tube paint: **about 10 µm of paint per area, whatever the load.**

- **Option A by itself fails.** If the stroke lays the same volume and the solvent then leaves, (1 − t) = 0.5 of the pigment stays: 32% / 14%. v1 added `run / (1 − t)`, reached 0.34 of the pigment and measured 37% / 18% at 2400 (v1 README).
- **Lowering raw sienna's hiding won't do it, and the source argues against it.** Even at half the pigment (1.9 coats at load 0.6), a uniform layer keeps 50% only if hiding is about 0.07 or lower. That is more transparent than transparent oxide yellow (0.2, palette.rs:131). Field/Salter §155 calls burnt sienna "richer, deeper, and more transparent than the raw earth", and the catalog gives burnt sienna 0.45 (palette.rs:163). So the source puts raw sienna's hiding above burnt sienna's, not below it. I propose no optical change.

## Proposal: a thinned load lays a liquid's film

Today a brush lays paste in proportion to what it carries: it wipes off a share per unit travel. A thin wash behaves differently. Its film is set by how much liquid stays between the hairs and the surface, not by how full the brush is. Proposed change, engine 3 only, for a bristle that carries solvent share t > 0:

1. **Liquid film.** In `exchange` (bristle.rs:1276-1278), deposit = min(today's paste law, h_w(t) × area swept). h_w(t) → ∞ as t → 0, so thinner 0 takes today's code path unchanged.
2. **Flash-off (option A).** Of what the bristle lays, only (1 − t) reaches the canvas (volume and pigment). The bristle loses all of it. The canvas gets no new state.
3. **Load sets reach, not thickness.** Paint the wash doesn't lay stays on the brush and is wiped at the next dip, as today.

Number to hit: a whole pass of thinner 0.5 lays at most about 20 µm wet, which leaves 10 µm of paint, at any load. That falls within the sourced ranges: ground layers of 1.5–30 µm (notes/research/oil_paint_physics.md:80), a glaze film of 20 µm (:39-40) and Inness's "thin washes or scrubs of color no thicker than water" (notes/research/inness_materials.md:89). h_w is the one new constant, and it is an estimate. Lead white would follow the same law: at 10 µm of paint, KM keeps about a third of the card (a veil, not a glaze).

Not modeled: a wet stage lasting minutes. The solvent is gone at deposit.

## Hazards for phase 3

- The `Debug` text of `Held` goes into `--state-digest` (crates/easel/src/main.rs:1194-1207). A new `Bristle` field would change engine-2 digests unless it stays out of that text.
- `pile` getters and `print` (crates/easel/src/api.rs:499, :517) must not show `thinner` before engine 3. This is the earlier `true 0.0` leak.
