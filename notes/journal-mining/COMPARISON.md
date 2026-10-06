# Opus and Sol syntheses compared

Two independent passes over the same 57 journals: Claude Opus 5.5 (`extract-*.md`, `PROPOSALS.md`) and
GPT-6.1 Sol, high reasoning (`sol-extract-*.md`, `SOL_PROPOSALS.md`). Sol did not see the Opus files.

## Both found (independently)

| lesson | Opus | Sol |
|---|---|---|
| Protect fine structure: paint it last or subtract it from every later background mask, and recheck after `grow()` | #4 | #1 (ranked first) |
| Blend discipline: passage-shaped masks, keep contrasting or protected shapes out | #2 | #2, adds "look after one pass before blending again" |
| A repair that keeps leaving seams needs a different method, not a smaller patch | #1, #7 | #4 |
| Check a mask before painting through it (`m:at`) | #11 | #6 |
| Check wetness before working over or beside paint | #3 | #7 |
| Glazes and scumbles: trial the actual veil; hand names and low load don't guarantee it | #5, #9 | #10 |
| Judge a mix laid in place; start a repair match from the logged recipe | #6 | #3, #8 |
| Replace techniques.md's stale "sketch sessions" bullet with the scratch canvas | yes | yes |

## Where they differ

- **Whole field versus patch.** Opus proposes "repaint the whole field, not a patch" as a rule (and as a
  brief sentence). Sol refuses the blanket rule: engine-5 local repairs worked (a thinner tint on a sky,
  paint-studio-d95ea6:74-85; exact-recipe local matches, paint-studio-49b647:52,57). Opus's own extract C
  notes the same d95ea6 success. **Sol's framing is better supported.**
- **Absolutes and numbers.** Opus's text includes numbers (glaze pigment share about 0.6 or less, medium
  0.7 or more, load 0.35 or less; thinner about 0.5 for scumbles) and categorical rules ("pale over dark
  only on dry paint"). Sol excludes exact loads, waits and coverage targets and "always dark on dry", citing
  successful wet modeling (paint-studio-24be5f:15-16; paint-studio-aef22d:61-62). **Sol's caution fits the
  evidence; the numbers belong in tests, not prompts.**
- **The rag.** Opus proposes "Undo, almost" (spirits rag lifts wet paint off a dry layer). Sol agrees the
  rag works but records residue and failed "perfect removal" claims, and lists it under tests.

## Only Sol found

- Coverage problems have three causes to tell apart: gaps between strokes, pigment that hides poorly and a
  starving deposit; `fill=true` closes gaps with dabs that may be unwanted (#5).
- Marks turning into beads, lozenges or pickets: fix scale, direction and the large form before
  multiplying marks (#9).
- Handoffs: keep paths, recipes and protection masks in globals; check saved paths before redrawing (#11).
- A gallery-light surface check before finishing (#14).
- **A real contradiction in what painters are told**, confirmed: techniques.md says "oil species and
  ground absorbency are not drying controls in this easel" (techniques.md:199-200), while the guide says
  poppy oil "dries much slower" (easel_guide.md:111-112) and an absorbent ground makes thin paint "quick to
  set" (easel_guide.md:67-70). Which is true of the current engine needs a small check.
- Qualifying techniques.md's "full value range" as an absolute for a free-subject brief.

## Only Opus found

- "Keep a correction within one value step; accents near their surroundings' value" (Sol touches this only
  inside its mark-pattern diagnosis).

## Recommended merge

Take Sol's structure and caution (diagnostic checks, no absolutes, no numbers) for the proposed text. Add
Opus's "within a value step" line. Keep both models' specific recipes (thinner scumble veils, glaze
pigment share, the rag's limits, dry interlocking hatches) as small-scene tests first. Fix the oil and
absorbency contradiction once a test settles it.
