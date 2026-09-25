# Cracks: the variation that didn't reach the picture (branch fix-cracks)

## What to look at

All images are lossless PNGs at 1:1 (look at 100%): main on the left,
this branch on the right. The lab painting (round 10's frozen pond) is
finished with its own settings (`width_um` 16, `dirt` 0.25, `depth_um` 14,
the rest `Cracks::aged`). Nothing else differs.

- `lab_tree.png` (the dark tree against the warm sky): on main you see a
  few faint hairlines. On the branch you see the network: a few longer,
  darker cracks and finer ones between them, closing into cells of
  different sizes, busier in some passages than in others.
- `lab_sky.png` (the thin sky): the same in its lower half. The dark blue
  upper half shows no cracks on either side (see "What's left": there the
  crack's tone matches the paint's).
- `lab_corner.png` (the lower right corner): the corner cracks run
  diagonally across the corner, as designed (not rings). On the branch
  they are less dominant (generation 0, corners and bars, opens 1.5× a
  primary instead of 2×).
- `lab_network.png`: the crack network alone (each crack's opening ×4),
  the whole canvas at half size. On main the darkest lines are the corner
  hatching and the stretcher-bar lines, and large passages look nearly
  blank: their cracks exist but were drawn at a fraction of their opening.
  On the branch the canvas's own first cracks come close to the corner and
  bar lines, and those passages show their cracks. A few areas stay blank
  on both. The network also reads more evenly busy than on main (see
  `patchy` below).
- `winter_own.png` (round 2's winter, branch amnesia-winter, the warm sky
  right of the ruin): byte-identical on both sides. That painting asks for
  the old even web (`hierarchy` 0 and the rest of the variation off), which
  this fix doesn't touch.
- `winter_aged.png`: the same crop with the painting's own sizes but
  today's variation (`..Cracks::aged(0)`; a temporary edit, not committed):
  main vs the branch.

The cracks are still fine lines: at 3200 px a pixel is 0.14 mm and these
cracks are 5–50 µm wide. The fix doesn't widen anything by fiat. The
branch's cracks read about twice as strongly at the painter's
`width_um` 16 because main was thinning them by mistake (below). To
quiet them, use `width_um`.

## What was wrong

One bug: `hierarchy` opened each crack by the canvas's global generation
count instead of by the crack's own history. The network is grown in five
strength steps. Generation 0 only nucleates where stress concentrates: 91%
of its length on the lab canvas lies at the corners (58%) or along the
stretcher-bar edges (33%), and with corners and bars off it is empty. So:

- the widest cracks on the canvas were the corner hatching and the bar
  lines (hence Fable's "concentric arcs hugging all four corners");
- the canvas's real first cracks (generation 1) opened 1.16× instead of the
  documented 1.7×;
- with `patchy`, a tough passage that only began cracking in generation 3
  or 4 had its first long cracks drawn at a third of a primary or less.
  Those cracks split whole islands: the top tenth of generations 3 and 4
  release 0.90 and 0.82 of a primary's stress (0.67 and 0.48 without
  `patchy`);
- the generation decay was also multiplied by the released stress, which
  already falls each generation. The steps fell about 10× from generation 0
  to 3 instead of the documented 0.68 per generation, so the finer
  generations vanished. What remained visible was one generation, evenly
  spaced by the relaxation distance: Fable's "every crack a one-pixel
  hairline ~5 gray levels darker", "coverage 0.7–1.1% everywhere".

Because the later generations were near-invisible, `vary` (which drops
them in dark and thick paint) and `patchy` (which delays them) had little
left to change in the picture.

The fix (`rsegs`): a crack opens by the stress it released, over the
island it split: released² (`HIER_EXP`), so on an evenly aged canvas each
generation opens ~0.69, 0.41, 0.23 of a primary (the documented 0.68 per
step). The first crack through a tough passage opens like a primary,
whenever it formed. Cupping and the stretches where a crack closes follow
the same measure. With `hierarchy` 0 nothing changes (verified
byte-identical).

`depth_um` and `cupping_um` were not broken: they shape the relief, and the
finish lights the relief at the painting's strength (Friedrich: `relief`
0.06), which hides them. Under a full-strength light they show. `dirt`
works but moves a crack's tone only a little, because a crack is a small
part of its pixel. The `Cracks` docs now say this, which also answers r10
friction 14 ("which knob makes cracks less visible": `width_um`).

## Each setting, measured

The lab harness (`crates/paint/src/crack_lab.rs`, test only, ignored)
loads the lab's `glaze` checkpoint, replays `Run::finish` exactly (its
crops reproduce the lab's delivered crops bit for bit) and finishes it
once per variant (one setting changed from the painting's own). Regions
come from the paint before cracking: pale thin (linear luminance > 0.3,
paint film < 150 µm; 24% of the canvas), dark (< 0.04; 6.5%). Tones are
8-bit sRGB luma. "Changed" is the share of pixels whose tone moves by ≥ 2
levels against the painting's own finish. "Crack area" is the open crack's
share of the region.

| setting (vs the painting's own) | main | branch | verdict |
|---|---|---|---|
| (the painting's own) | pale thin: 1.50% of px > 6 levels darker; crack tone p50/p99 4.7/21.6 levels; crack area pale/dark 0.38%/0.38% | 3.85%; 7.4/33.9; 0.82%/0.67% | |
| `hierarchy` 0 | changed 2.67% (max 34) | changed 3.01% (max 48); crack tone p99 14.6 at 0 vs 33.9 at 1 | **was broken**, fixed |
| `vary` 0 | changed 5.55% (max 51); crack area in darks 0.39% vs 0.38% at 1 (no difference) | changed 9.33% (max 67); darks 0.88% at 0 vs 0.67% at 1 | worked on the network (drawn 6.0% of pale thin vs 3.2% of dark thick px), didn't reach the picture's darks; now does |
| `patchy` 0 | changed 6.28% (max 52); 5 mm tile coverage cv 0.35 vs 0.51 | changed 9.33% (max 68); cv 0.29 vs 0.32 | worked on the network. On main its quiet passages were blank mostly because their few long cracks were drawn too thin. Now they show as a few long cracks (cells of different sizes), which tile coverage barely separates |
| `grime` 0 | changed 1.30% (max 31); crack pixels in darks lighter than the paint: 100% at 1, 0% at 0 | changed 2.82% (max 34) | worked |
| `dirt` 0 / 1 (painter's 0.25) | changed 0.35% / 0.23% (max 14 / 9) | 1.45% / 1.00% (max 18 / 14) | works, weak by physics (a crack covers ~5–25% of its pixel and a clean crack is already a shadowed slot); documented |
| `depth_um` 0 / 40 (painter's 14) | changed 0.00% / 0.00% (max 1 / 2) | 0.00% / 0.00% | works as relief; hidden by the painting's light. At relief 1.0: main 0.25% / 0.71% (max 24 / 44), branch 1.5% / 3.2%; documented |
| `cupping_um` 0 / 40 (15) | changed 0.00% / 0.00% (max 0 / 1) | 0.00% / 0.00% | same. At relief 1.0: main 2.8% / 8.9%, branch 5.4% / 12.4%; documented |
| `corners` off | changed 5.29% (max 52); 96% of crack length within 5% of the diagonal from a corner runs within 30° of perpendicular to it | changed 8.78% (max 74); 96% | geometry worked (straight diagonal cracks, not rings); on main they were the widest cracks; now 1.5× a primary instead of 2× |
| `grain` 0.3 (for reference; default untouched) | changed 5.43% | 8.92% | not in scope |

Island sizes seen in the picture (islands are the areas between crack
pixels ≥ 2 levels; per 12 mm tile of pale paint, median island): on main
at least half the tiles show no closed island at all (the visible cracks
don't meet). Branch: median 6.4 mm, p90/p10 across tiles 2.6.

Openings per generation (the `first_cracks_open_widest` test canvas,
relative to `width_um`): main [2.22, 1.08, 0.52, 0.22, 0.08], branch
[2.55, 1.68, 0.94, 0.42, 0.15]. The length-weighted mean opening of the
lab network (before the local paint's say) is 0.52 on main, 1.10 on the
branch and 0.81 with `hierarchy` 0.

## Tests

- `crack::tests::first_cracks_open_widest_wherever_they_formed` (new). On
  a patchy canvas, the cracks that were first in their passage (no older
  crack within an island of where they started) must open > 0.7 × the
  primaries and > 1.5 × the cracks subdividing the same generation. Main:
  fails (generation 3: 0.34 vs primaries 1.12, subdividing 0.26;
  generation 4: 0.18 vs 0.12). Branch: passes (1.53 and 1.53 vs 1.72;
  subdividing 0.78 and 0.55).
- Existing crack tests all pass unchanged, including `first_cracks_open_widest`,
  `patchy_development_leaves_quiet_passages`, `corner_cracks_cross_the_diagonal`
  and `crop_cracks_like_the_whole`.
- `cargo test --workspace` and `cargo test --release -p easel --test
  hand_time` pass. No golden hash re-recorded: no golden replay calls
  `cracks`, so none moved. Clippy: no new warnings.
- The lab: `CRACK_LAB_CKPT=<r10-arm2>/out/r10_winter_b_full.glaze.ckpt
  CRACK_LAB_OUT=<dir> cargo test --release -p paint --lib crack_lab --
  --ignored --nocapture` (also `CRACK_LAB_ONLY`, `CRACK_LAB_RELIEF`,
  `CRACK_LAB_DUMP`).

## Causes (file:line on main, 0abe49e)

- `hierarchy`: `crates/paint/src/crack.rs:1154-1157`: `gf =
  HIER_DECAY.powf(generation)` and `open = ... HIER_PRIMARY * gf * wide *
  a.opening.clamp(0.5, 1.3)` (the global generation, and the decay counted
  twice); `:1157` cupping and `:1177` the closed stretches by generation.
  Generation 0 is sparse because the first strength step is 1.6 × the mean
  stress (`:230`, `R_FIRST`), which only corners and bars exceed. Fixed at
  `rsegs` (`HIER_EXP`).
- `depth_um`, `cupping_um`: their only use is the height change
  (`crack.rs:1304`), lit by `Canvas::relief` (`paintings/src/run.rs:611`)
  at the style's strength (`crates/paint/src/style.rs:136`, 0.06). Docs.
- `dirt`: `crack.rs:1342-1358`: with `grime` on, the soot is replaced by
  grime over the walls, in amount `dirt`, filling a slot that already
  keeps 35% (`:1480`, `SLOT`). Docs.

## What's left (not bugs of these settings; for Alice to decide)

- **Corner regularity.** The corner stress is a flat uniaxial plateau over
  half the corner zone (`crack.rs:489`), so the corner cracks are evenly
  spaced parallel chords, alike in all four corners. A concentration that
  falls off from the corner would space them out away from it (entropy from
  the process). That is a model change, not a freeze fix.
- **The mid-tone crossover.** Where the paint is about as dark as the
  crack's fill (the upper sky here, luma ~70–95), a crack neither darkens
  nor lightens its pixel. The network is there (`lab_network.png`); it has
  no contrast. This follows from the model (shadowed slot, pale ground in
  the walls, grime), consistent with "visibility is contrast".
- **Hairlines.** Crack widths are physical (5–50 µm here), far under a
  pixel (0.14 mm; ~0.2 mm from the next round), so every crack is a
  one-pixel line whose strength is its opening. With the fix that
  strength spans 2.8 to 34 levels in pale paint (p10 to p99 of crack
  pixels; main 2.4 to 22).
- **Even spacing.** The relaxation distance (D_r ≈ S/2) still sets one
  spacing per generation (Alice's note in `notes/cracks_lab/README.md`).
  It also limits `patchy` in the picture. A passage that began cracking
  late gets first cracks spaced like the primaries elsewhere, so it
  differs from its neighbors by lacking the finer subdivisions rather than
  by having fewer long cracks. On main those passages looked blank only
  because of the bug.
- **`vary` and `patchy` stack by generation count** (`crack.rs:1279`): a
  tough passage under dark paint can show no cracks at all.
- **`vary`'s thick paint** reaches only 0.4% of this painting: the film
  bookkeeping's thresholds (`THICK_UM`, 150–700 µm) were calibrated on
  another painting.
- **Amount.** At the painter's `width_um` 16 the branch's cracks read about
  2× stronger (crack area in pale thin paint 0.38% → 0.82%). The painter
  tuned 16 µm against main's too-steep hierarchy. With `hierarchy` 1 a
  primary opens `HIER_PRIMARY` (1.7) × `width_um`, although `width_um`'s
  doc calls it the primary's opening. Main's bug hid that (its canvas
  primaries opened ~1.1×). Whether 1.7 stays is a calibration for Alice,
  not part of this fix.
- **Direction** (diagonal, down-right, ~0.3): untouched, per the plan in
  `notes/HANDOFF.md`.
