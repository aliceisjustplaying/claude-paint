# What the journals teach: proposed "how to paint" improvements

Sources: the three extractions in this folder (`extract-A.md` r16-r17 engine 1, `extract-B.md` r18-r20
engine 1, `extract-C.md` engines 2-5 incl. the r31 smoke painter), 57 journals in all. Citations are
`<studio>:<line>` as in those files. "Recurs" counts the batches (A, B, C) where a lesson shows up
independently; a lesson in all three survived every engine change.

Status: proposals for Alice. Nothing in the brief, guide, techniques or studio notes has changed.

## The ranked lessons

| # | lesson | recurs | already said? |
|---|---|---|---|
| 1 | Correct a field by repainting all of it up to real edges with its original piles, over dry paint; local patches on a graded, thin or blended field dry as visible boxes | A B C | no |
| 2 | Blend only inside one wet tone, with a passage-shaped mask whose edge lies where the paint already matches; one pass; never a rectangle over contrasting wet paint or touching darks | A B C | partly (guide: clip) |
| 3 | Pale over dark, and dark corrections, only on dry paint: dark into wet pale comes out pale and ghosts; tacky paint breaks a pale into patches. Check `drying()` at several points first | A B C | partly (glazes and scumbles only) |
| 4 | Paint fine structure (twigs, figures, objects) last, keep its paths in globals and subtract it from every later background mask (after any `grow()`) | A B C | no |
| 5 | Glazes: low-hiding tubes, no white or earths, a small pigment share (about 0.6 parts or less), medium 0.7 or more and a low load (0.35 or less); trial first. Whole-canvas "unifying" glazes went wrong | A C | partly (medium only) |
| 6 | Judge a mixture's value against the passage it will lie on, not on the board: white-heavy piles go pale and lose hue, earth and black piles go near-black, thin lines over pale go lighter and warmer. Lay a small value ladder and read it in `mode: "value"` | B C (A: mix deeper when matching) | partly (studio notes 6) |
| 7 | When the same spot fails twice, stop and change method (or drop the motif); compare at whole-canvas size | B C (A: rescue cycles) | yes, brief ("If the repair recreates the problem…") |
| 8 | Keep corrections within one value step of their surroundings; small accents near their surroundings' value (pale blades and rim lights read as toothpicks and cracks); soften a seam by hatching ragged fingers of the neighboring recipes over dry paint, not by blending or glazing | B C | no |
| 9 | Scumbles with `hand="scumble"` or light lead-white mixes laid opaque dabs, blobs or rectangles, not a broken veil (engines 2-5); veils that integrated came from `thinner` about 0.55 in several light passes | C (7 journals) | contradicts techniques ("Scumble") |
| 10 | The spirits rag is the undo for wet paint over a dry layer; not on tacky or thick medium-free paint; it leaves a streak where a wipe ends; a fresh face per pass | C | contradicts techniques ("No undo") |
| 11 | Verify a new mask before painting through it (`print(m:at(x, y))` at a few points, `m:area()`); curves must span the canvas | A C | no |
| 12 | In evening and backlit pictures plan the dark land and foreground from the start; check the squint early | A | partly ("Plan in values") |

Engine-1 lessons I left out as likely stale or engine-specific: blend radius effects of the old 40-unit
blender (A6), `edge="lost"` reach (B10), `load_at` not softening opaque paint (B5), `world{}` scale (A7).
Each needs a check against the current engine before it goes anywhere.

The scratch canvas (r31 smoke only): the painter planned trials but ran them on the painting in sitting 1,
then used the scratch canvas for every new mix from sitting 2 on, once it had a specific unresolved
passage to test (r31smoke citations in extract C). One painter is one case.

## Proposed text

### techniques.md (shipped in every studio)

Replace "Scumble" in "Glazing and scumbling":

> - **Scumble**: light paint dragged thin over a dry darker passage, broken by the surface: air, haze,
>   light on water. Painters here found that `hand="scumble"` and pale lead-white mixes at a light load
>   tend to lay opaque dabs rather than a veil; the veils that blended into the passage came from paint
>   thinned with `thinner` (around 0.5) laid in several light passes. Try it small first.

Add to "Glazing and scumbling":

> - **A glaze is mostly medium.** Low-hiding tubes only (lakes, the transparent blues and greens), no
>   white and no earths, a small pigment share, medium 0.7 or more and a low load. Too much pigment lays
>   saturated bars. A glaze over the whole canvas to "unify" it has gone wrong more often than right.

Replace "No undo" in "Keeping control", and add three bullets:

> - **Undo, almost.** Wet paint over a dry layer comes off with a rag dampened with spirits, down to the
>   dry layer. It won't lift tacky paint or thick paint without medium, it leaves a streak where a wipe
>   ends, and it wants a fresh face each pass. Dry paint stays: paint over it.
> - **Repair a whole field, not a patch.** A patch of fresh paint on a graded, thin or blended field dries
>   as a visible box. Let it dry, then repaint the whole field up to its real edges with its original
>   piles (the log has their recipes).
> - **Blend inside one wet tone.** Blend with a mask shaped like the passage, its edge where the paint
>   already matches, once. A rectangle blended over contrasting wet paint leaves its outline; a blender
>   that touches a fresh dark drags it.
> - **Paint fine structure last.** Twigs, figures and small objects go on after the passages behind them;
>   keep their paths in globals and subtract them from every later background mask.
> - **Check a new mask** with `print(m:at(x, y))` at a few points before painting through it.

Add to "Time":

> - **Pale over dark, and dark corrections, only on dry paint.** Into open paint they mix and go pale or
>   ghost; on tacky paint they break into patches. Check `drying()` at several points first.

Add to "Color: broken, not mixed flat":

> - **Judge a mix where it will lie.** On the canvas, white-heavy piles go paler and lose hue, earth and
>   black piles go nearly black and thin lines over a pale field go lighter and warmer. A few candidate
>   strokes read with `mode: "value"` settle it.
> - **Keep a correction within a value step** of what surrounds it, and small accents near their
>   surroundings' value: a pale blade or rim light against a dark reads as a crack. To soften a seam,
>   hatch ragged strokes of the two neighboring colors across it over dry paint.

Replace the stale "Sketch sessions" bullet (the painter's studio now has a scratch canvas):

> - **The scratch canvas.** Try a mix, a stroke, a glaze or a drying time on the scratch canvas beside
>   the painting (`scratch: true`) before it goes on the painting: lay the underlayer it will go over and
>   let it dry as long.

### The brief

The brief's "Working" section already carries the process lessons (inspect after each campaign; a repair
that repeats the problem means a new method; trials before repeating an unfamiliar mix). One addition
would carry the two most repeated failures to every painter, whichever notes it reads:

> Repair a passage by repainting the whole of it up to its real edges over dry paint, rather than
> patching it, and blend only within one wet tone.

## Open questions

- Items 9 and 10 contradict text painters already get. They rest on painters' observations, not on a
  measured engine check; a small scene per claim would confirm them before the text ships.
- Whether these edits go into the next round only (`round-31` and later) or are also shown to current
  painters in a return sitting.
