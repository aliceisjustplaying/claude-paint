# Glitch census (round 6, branch `r6-glitch`)

The owner's most repeated complaint at 3200 px is small digital-looking artifacts: dots on
the lit rock, the same dots in the lime's crown, outlines and "lining" on the rocks, sky and
water "glitches", "lines that are not blended properly" and "orangey, earthy, browny lines"
(`notes/round6/alice_review.md`). This round reproduced each passage on main and on the wet
engine (`r6-wet` at `abacd02`, built in scratch; that worktree was not touched), wrote a
detector for the classes, traced each class to its cause and fixed the engine bug that was
still live.

**Sheets for the owner (lossless PNG, 1:1 pixels):**
- `notes/glitch/alice_census.png`: the named passages at 3200, 1:1, each with the detector's
  overlay beside it (11 rows, numbered as in the table below).
- `notes/glitch/alice_fixes.png`: the engine fix (P1), before (main) over after (this branch):
  two windows of the lab 2 lime's crown at 3200 1:1, and the one place the 1000 px
  benchmarks moved visibly (l3_green's hedgerows, 4× pixels).

## The catalog

| class | where (sheet row) | cause | status |
|---|---|---|---|
| **V** brown dots and worms on the lit rock, outline along the rock's contour, dots in l3_green | lab 2 rock Q (1); lab 1 rock B's contour (8); l3_green; every pre-fix lossless sheet in `notes/round6/wet_experiment/lossless/` | engine: the varnish pooled at dried stroke edges (`notes/varnish.md`) | **fixed on main before this round.** Re-rendered here: the dots and the contour outline are gone on main and on r6-wet (detector: 3,739 fleck px per Mpx on the pre-fix lab 1 rock B, 9.3 now in a similar window) |
| **P1** pale gray-blue pinholes in a dark hatched crown | lab 2 lime P (3) | engine: a pointed tool's `cover` (its hairs' share of the pixel) stayed the sliver the hairs had touched while leveling poured 50-130 µm of paint into the pixel, so the paint sat on a sliver and the sky showed beside it | **fixed here** (`wet::bead_cover`), see below |
| **P2** single-pixel light specks left in the same crown | lab 2 lime P (3, after the fix) | recipe: bare crevices, a few µm of paint, 200 µm below the stiff hatch ridges around them, over a dry pale sky | recipe (sketchbook §13). An engine rule that closes 1-px bare slits between thick wet films (Taylor-Michael) was tried and reverted: it did nothing here (these are thin films, not bare pixels) and moved l5_near at 1000 |
| **G** orange slivers along long level strokes | lab 1 water B (6, 7), sky B (4: a few flecks) | recipe: bare Friedrich imprimatura (orange) left between thin long strokes of the water and sky; soft-edged, it's paint (it's there before the finish) | recipe (sketchbook §13) |
| **T** doubled pale "tramlines" along the waterline sheen | lab 1 water B (6, 7), main and r6-wet alike | engine, wet-on-wet: a round 3 stroke laid into open paint, then a level `blend()` along it, gives two pale lines with a darker trough (`notes/lab/water.md` item 2: without the blend it is one hard wire; likely the blender's plough splitting the line). It is paint, not relief: it is there without the finish chunk | **for the wet stream** (below) |
| **W1** 1-px rings of bare ground round small painted ellipses | lab 1 sky B wet repaint (5) | engine, r6-wet only: after the first broad sky pass (`sky_B_wet.lua` chunks 1-2, even without its blend); main at the same stage has none | **for the wet stream** (below) |
| **C** an even, ruled dark line across the lit rock | lab 1 rock B (8) | recipe: the lit pass is clipped off `rk:cracks()`, so the crack is a channel of the shadow paint of constant width. There is no pencil in this log; it is not graphite | recipe (sketchbook §13) |
| **S** dark outlines round old strokes inside the boulder's snow cap | l5_near (9) | recipe with an open engine question: thin snow (load 0.6, medium 0.32) dragged over the dry, ridged rock; the crests (+178 µm over their surroundings) end nearly bare. Not the relief lighting (there with `relief()` removed), not the blend (there without `blend(cap)`). At 1000 it reads as the snow torn by the grain the painter asked for | recipe (sketchbook §13); engine question below |
| **M** crunchy mottle in quiet sky, "JPEG vibes" | lab 1 sky A (`notes/round6/jpeg_check/`) | recipe: a stipple over the weave (checked lossless before this round) | recipe (known); the detector measures it (mottle share 0.81 for sky A, 0.017 for sky B) |
| light dots in the wood | l3_green (10, 11) | the painter's own lights (touches) in a dark mass; on r6-wet they are larger and separate (as `notes/wet.md` says) | not a glitch: a painting decision |
| hairline grass | l3_green (10) | recipe: dark wiry blades of equal weight | recipe (known, sketchbook §8) |

Graphite showing through thin paint is another stream's question; nothing in these
passages traced to it (lab 1 rock B has no drawing at all).

## The engine fix: P1, paint can't stand on a sliver of a pixel

`crates/paint/src/wet.rs`, `bead_cover`: when a film bakes (`drying.rs`, `bake`) and in the
wet look (`look_px`, so what the painter samples is what dries), the share of the pixel the
paint covers is at least `coats·COAT_UM / (BEAD_ASPECT·px_um)`: paint on a sliver of a pixel
stands no taller than half the pixel is wide (`BEAD_ASPECT` 0.5; stroke ridges are 0.1-0.3
of their width). The rule is linear in the paint, so a hairline's fringe keeps its share
wherever it falls between pixel centers (the tip tests `translated_pointed_marks_look_alike`
and `pointed_marks_are_resolution_independent` pass; a first version with a square root
broke the first), and at 1000 px (0.2-0.9 mm pixels) it takes a film over 100 µm on the
sliver to act.

What I measured before fixing: the lime's pale specks sat on paint, not on bare pixels
(median +54 µm of new surface under them, the dry surface there not a pit: −1 µm), and they
read gray, not sky. Forcing full cover at the bake (a throwaway switch) removed them all; a
histogram of the hatch's bake at 3200 had ~2,000 pixels holding ≥ 20 µm on under a quarter
of the pixel and ~20,000 with ≥ 50 µm on 75-99%.

Tests (fail before, pass after):
- `tests::thick_paint_from_a_pointed_hatch_hides_the_ground_at_full_size`: a dark
  `round_sable(1.6)` hatch (coverage 2.4) over a pale ground at 0.1 mm/px; pixels under
  ≥ 30 µm of dark paint that still show more ground than paint: 33 of 63,520 before, ≤ 3
  after.
- `wet::tests::bead_cover_floors_only_impossible_beads` (unit).

Effect on the lime at 3200 (`notes/glitch/alice_fixes.png`): pale pixels (25+ levels over
their 7×7 median) in the crown fell from 11.2 to 6.0 per 1,000; the detector's pinholes
from 591 to 428. The larger gray blobs are gone; what remains is P2.

**Benchmarks** (`easel run <log> --width 1000`, main `9c0294b` against this branch, both
release builds): not byte-identical, as intended where the bug acts at 1000.
- `notes/loops/l5_near.lua`: mean abs difference 0.021/255, max 127; 5,769 pixels differ,
  487 by more than 4 levels (scattered thin-paint edges over the whole canvas).
- `notes/loops/l3_green.lua`: mean 0.088/255, max 94; 40,800 pixels differ, 4,244 by more
  than 4, almost all in the band of hedgerows and far woods (y 300-400), darker (the fine
  strokes' fringes cover their pixels); 186 pixels got lighter. Side by side at 4× it is
  hard to see (`alice_fixes.png`, right).
- The golden scene and the release `hand_time` replay hashes are unchanged.

## For the wet stream (r6-wet, not fixed here)

- **W1, rings.** `notes/wet/repaints/sky_B_wet.lua` chunks 1-2 (chunk 2 without its
  `blend` shows it too) at `--width 3200 --crop 380,400,780,590`: crisp 1-px rings of bare
  orange ground around painted ellipses about 12 × 5 px, e.g. at crop pixels (50-110,
  40-80); main at the same stage has none. The detector counts them as flecks: 198 fleck px
  per Mpx on the wet sky against 13 on main's. A ring of bare ground round paint means the
  ellipse's rim lost everything down to the ground (or was never reached) while its inside
  was painted: look at what the exchange/film rewrite does at the edge of a bristle's
  contact (pickup of a glancing bristle, `GLANCE`) in a single pass over bare ground.
- **T, tramlines.** Also on main. `notes/lab/water_B.lua` chunk 7 at `--crop
  380,520,700,660`, crop pixels (350-550, 170-230): a thin pale stroke laid into open paint
  and a level `blend()` along it becomes two pale lines with a darker trough. A soft blender
  run along a wet line should widen and soften it. My guess (not measured): the plough
  (`push`, the paint set down at `rb + 1` px either side of each bristle's track) splits it.

## The detector: `scripts/glitch.py`

```sh
uv run scripts/glitch.py out/crop.png --out overlay.png          # counts; overlay: render | marks
uv run scripts/glitch.py out/crop.png --json > marks.json        # every mark, with position and color
uv run scripts/glitch.py render.png --scale 1.0                  # a 1000 px render (sizes scale with it)
```

Everything is measured in OKLab against the pixel's neighborhood (a 9 px median at 3200):
- **specks**: blobs up to ~12 px that differ by ΔE ≥ 0.05 while the ring around them is
  quiet. *Visible* specks still stand out after a 1 px blur (the eye at 1:1 hardly sees a
  lone pixel; it sees a 2 × 2 blob). *Pinholes* are visible specks lighter and grayer than
  their neighborhood: an underlayer showing through. Red circles on the overlay.
- **flecks**: patches up to ~80 px of an alien hue (OKLab a, b off by 0.035): ground
  slivers, rings, pooled varnish. Magenta.
- **lines**: thin (≤ 2.5 px), long (≥ 8 px) runs; cyan along a value edge (outlines,
  lining), blue elsewhere (hairlines).
- **mottle**: the share of quiet 24 × 24 windows whose fine band-pass texture is strong.
  Yellow.

Calibration on the owner's passages (3200 crops; per Mpx where marked):

| render | visible specks | pinholes | flecks px/Mpx | mottle share |
|---|---|---|---|---|
| lab 1 rock B before the varnish fix (lossless sheet, 400 × 287 units) | 1,151 | 10 | 3,739 | 0.39 |
| lab 1 rock B, main now (380-780 × 270-560, a similar window) | 174 | 14 | 9.3 | 0.13 |
| l3_green before the varnish fix (lossless sheet, 400 × 227 units) | 3,642 | 193 | 4,019 | 0.79 |
| l3_green, main now (440-760 × 430-580) | 492 | 18 | 324 | 0.37 |
| lab 2 lime P, main / this branch | 1,470 / 1,514 | 591 / 428 | 1,214 / 1,124 | 0.69 / 0.66 |
| lab 1 sky A / sky B (jpeg_check lossless) | 9 / 8 | | 15 / 57 | **0.81 / 0.017** |
| lab 1 sky B, main / r6-wet repaint | 34 / 85 | 0 | **13 / 198** | 0.017 / 0.036 |

Limits: it finds candidates, not verdicts. Grass blades, lit leaf touches and snow flecks
are real marks and count as specks; texture in a lit rock face counts as mottle. Compare a
passage with itself (before and after, main and a branch), not paintings with each other.
Clustered blobs larger than ~12 px escape the speck class (the lime's gray blobs mostly did,
so its pinhole count understates the fix).

## Tools added

- `easel run ... --dump-surface out.f32`: the dried surface height (µm, little-endian f32,
  row by row) under the saved pixels, for lining up relief with a render.
  `Canvas::kept_surface_um()` gives the same in Rust.

## What's left

- P2: bare crevices between stiff hatch ridges over a pale layer (recipe; an engine answer
  would be capillary closing of thin-film crevices, which needs leveling to act below the
  yield floor at 0.1 mm scales).
- S: whether a bristle dragged over dry ridges should strip the fresh film from their crests
  (plough and pickup scale with contact weight, highest on the crests) or catch paint there
  as a dry brush does. Measure deposit against plough per pixel on a crest, at 1000 and
  3200.
- W1 and T for the wet stream (above).
- The detector's classes overlap with real marks; a per-passage mask (quiet sky, rock face)
  would make its counts sharper.
