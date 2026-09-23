# Loop 1: pale halos around dark motifs

Defect (blind critic, easel3_free): "pale halos outline the stones, the
figure and the trunk, so they look matted in (probably tool)". It was a tool
defect: brush contact near thick paint. It is fixed in the engine in commit
`8b54213`.

## Reproduction

`target/release/easel run notes/amnesia3/easel3_free.lua --width 3200
--crop 200,420,560,540`. Images are in `notes/loop1_halo/`.
`before_3200_crop.jpg` and `before_figure.jpg` show a soft pale rim, about
5 units wide, around the stones, the figure and the dead oak against the sea.

Bisecting by truncating the log at a chunk:
- After chunk 32 (`bisect_after_chunk32.jpg`) there is no halo; the old sea is
  an even mauve-gray.
- After chunk 33 (`bisect_after_chunk33.jpg`), the sea veil
  (`stipple(veil, {width=2.2, color=seacol2, coverage=2.6, ...})`) darkens
  the open sea but leaves a band next to each motif almost untouched. The
  chunk 34 blend then softens that band into a glow, and the chunk 35 glints
  skip it too.

Mean of R, G and B (0–255) along row 482 (3200 crop pixel y=198), eastward
from the right-hand upright, sampled every 1.9 units:

| | at the stone edge → 6 units out |
|---|---|
| after chunk 32 (no veil yet) | 110 111 111 111 111 |
| finished painting, old engine | 105 103 93 82 78 |
| finished painting, fixed | 97 86 87 83 78 |

## Diagnosis

It was not the masks. Probes show `veil` = 1.0 from 1 unit off the stone,
exactly as the painter wrote it (`stonesil:grow(1)`). It was not the aim
either. The dips next to the stone judged the pale sea correctly (`seen`
≈ 0.15–0.16, `cv` 2.6), and a variant without clipping gave the same profile.

The cause is the contact base in `Canvas::surf()` (`crates/paint/src/bristle.rs`):

```rust
let low = box_blur(&box_blur(&self.height, w, h, r), w, h, r);   // r = 1.5 mm
*b = (0.5 + (hgt - lo) / (2.0 * TOOTH_UM)).clamp(-0.2, 1.3);
```

The stones are body paint about 1000 µm high and the thin sea is about
350 µm. The plain mean lifts the level over the sea next to the step: at
x=427 the level was 598 against a height of 357, so base ≈ −1.5, clamped to
−0.2. The sea there reads as a valley about 2r wide (r = 3.4 units on the
440 mm `friedrich` canvas), and bristles don't touch it. A pressed touch
lays paint in proportion to contact (`touch = sum_w / sum_cov`), so the
veil, blend and glints all thin out there and the paler old sea shows.

## Fix

`contact_level()` in `bristle.rs` replaces the plain mean with a separable
running median (a window of about 1.5r along the rows, then the columns) and a
light box blur. Weave and brush-mark relief sit about the median as they did
about the mean. At a step, a pixel's window is mostly its own side, so the
step no longer lifts the level at its foot. At x=427 the level is now 394
(base ≈ 0.19); from x=428 on it is within about 20 µm of the sea (base ≈ 0.5).
Only the corner right at the foot of the step is missed. The level is
rebuilt only when the height changes (drying, settle), not per stroke.

Regression test: `stipple::tests::veil_reaches_its_tone_up_to_dark_motifs`.
It uses dark bars on 0.6 mm plateaus on a 440 mm canvas and a veil clipped
1.5 units off them. Mean darkening (OKLab L) 1.5–4 units from the bars
against the open field is 0.043 vs 0.208 with the old level and 0.198 vs
0.208 with the fix.

`cargo test --workspace` passes. The golden fingerprint was re-recorded
because the contact level changes wherever the canvas has relief, so every
scene's pixels move.

## What remains

- A thin rim of 1–1.5 units remains where the painter's program cut the
  motif out of the veil (`- stonesil:grow(1)`, `- figure:grow(1.5)`). That
  was the painter's choice.
- The dead oak's branch still has a visible rim, and that one is mostly the
  mask. At y=487.5, `oakB:mask()` runs from x≈462 to 466.3 but the dark paint
  covers only 463–465. With `grow(1.5)` the veil starts at 467–468.5, so it
  leaves a 3.5-unit gap. Next step: check whether `tree:mask()` should hug the
  painted limbs (the ribbon's soft rim) instead of their outer width.
- Defect 2 (thick embossed snow rims, easel4_near) was not attempted. The
  program is not in this branch; only `notes/amnesia4/*.jpg` is. Untested
  hypothesis: `relief()` lights a tall plateau's edge as a hard ridge. The
  old mean also made a plateau's rim read as a peak, so later strokes caught
  it. The median removes the second effect; the first needs a look at the
  relief lighting of a paint step (for example, limit the slope a step can
  show, or light the dried film's own surface rather than its total height).
