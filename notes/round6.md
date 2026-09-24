# Round 6, night 1 (2026-09-24): what happened while you slept

Read this first. Everything here waits for your eye. The critic panels
(2 × Gemini 3.8 Flash + 2 × gpt-6-astra, blind, shuffled) are diagnostic
only: they name defects, they don't decide. Their briefs, answers and keys
are in `notes/round6/panels/`.

## Decisions waiting for you (in order of payoff)
1. **Relief strength**: `notes/round6/relief/` (README + sheets). Pick off,
   vlow 0.06, low 0.12 or current 0.2; the winner becomes the Friedrich
   default. My suggestion to check: vlow or low.
2. **Merge r6-wet?** Real wet-on-wet; changes how every painting looks.
   See §3 below and `notes/round6/wet/`.
3. **Merge r6-oak?** Bare trees connected, angular and tapering; changes
   the `trees_in` study, not the benchmarks. See §4.
4. **The lab** (`notes/lab/`): which versions look good to you? See §2.

## What's on main now
All tests pass (debug; the golden is recorded in debug, so a `--release`
run reports a golden mismatch by design). The two benchmark logs render
byte-identical to before the night.
- **Hand in time** (merged from `r6-time`, opt-in): every stroke, touch
  and palette trip costs hand time (Fitts and steering laws, tapping rate,
  estimated reload and mixing times); with `canvas{..., hand=true}` the
  paint ages while the hand works, long passes in 15-minute slices;
  `sitting{}`, `rest()`, `timesheet()`; `easel look --mode wet` shows open,
  setting, tacky and dry in false color. Existing logs replay unchanged.
  Details: `notes/time.md`, `crates/easel/README.md` "Hand time and
  sittings", evidence `notes/time/`. For scale, l5_near is about 73 hours
  of hand time (53 of them in the fir wood), l3_green about 52.
- **Easel root fix**: the easel finds its checkout from the working
  directory (`EASEL_ROOT` overrides). Worktrees with a copied `target/`
  had been writing logs, renders and the default session into main, and
  one lab painter's `easel undo` hit another painter's session (nothing
  was lost).
- **The lab studies** and the relief sheets (notes only).

## 1. Relief A/B
The relief lighting makes the canvas weave grid and the stroke-ridge
creases. At 0.2 the green sky is covered in embossed diagonal creases and
the tree's sky holes get raised outlines; at 0.06 they're nearly gone; off
looks like a print. (Correction: the knife-cut hatching below was the
varnish, fixed and merged in `d5c53f0`.) It does NOT make the knife-cut hatching on the near
rock, the pale rims around objects, the fog rectangle in the near wood or
the pale halos in the green tree's holes: those are identical at every
strength. So relief is the "grooves", not the whole plateau.

## 2. The lab (`notes/lab/`)
Five single-subject studies, each painted on an identical setup chunk: A
the old way (sketchbook recipes as written, `dry()` first, objects
finished inside their masks), B the Round 6 way (masses first, value
families across objects, painting across mask edges, wet interaction,
economy). Foliage and the bare tree got a third try, C, after the first
panel. Each subject has `S.md` (what was done, the painter's verdict,
ceilings, wet-paint failures, sketchbook candidates), logs, whole images
and 3200 crops.

| study | panel 1 (A vs B) | panel 2 (A/B/C) | my eye |
|---|---|---|---|
| water | **B 4–0** | | B clearly: the bank and its reflection went down as one wet dark; A is a cut-out with a knife-edged mirror copy |
| sky | **B 4–0** | | B: cloud banks laid into the wet sky sit in it; A's clouds are cutouts on an even gradient |
| rock | **B 4–0** | | B is grounded (shadow side, foot and cast shadow as one dark), but its lit face is a flat, wormy slab |
| foliage | split 2–2 | **A first 3/4**, C second, B last | C at 1000 (real overlapping masses with light on top), but it repeats like broccoli; at 3200 all three have a near-black shade and punched holes |
| bare tree | **A 4–0** | **C first 3/4**, B last 4/4 | C: A's connected network, pruned and tapered, with far limbs lighter; B's twig tone reads as steel wool |

What the lab says:
- **Masses first and value families across objects work** where the
  subject is a few big shapes (water, sky, rock): unanimous.
- **Economy by tone fails for fine structure.** Indicating twigs as a
  tone read as fur to every critic; economy by *selection* (fewer,
  varied, tapering drawn twigs: bare tree C) won.
- **Foliage is unsolved.** The old detailed recipe still edges out the
  new ones with critics; the shared defect in every version is the shade
  half collapsing into one dark slab and holes that look punched.
- The recurring defects across all studies (all four critics): razor-cut
  edges and cutouts; hairline grass and twig "wire"; repeated grooves or
  embossing that don't follow form (the relief was at 0.2 in every study);
  big darks with no internal values.

## 3. Wet-on-wet (branch `r6-wet`, not merged)
Three passes (details: branch `notes/wet.md`, evidence `notes/wet/`).

**The cause.** Each wet pixel held one mixture, so anything laid into
open paint was blended into it by volume whatever the brush or load: a
loaded light touched into a wet dark came out 0.57 clean. And the plough
pushed a fixed share of the film aside on every bristle pass however thin
the film, scraping thin wet darks to the ground (81–90% of a track). The
sketchbook's "translucent rock", "fog band" and "plowed river" pitfalls
were these engine bugs, not painter misuse.

**The fix.** Two layers per wet pixel (the newest stroke's surface film
over the body), mixed by how the bristles work them (travel, pressure,
stiffness, load); pickup takes the surface first; the plough can't push a
film below what a bristle rides on and follows the paint's stiffness;
blenders drag into the film; a dirty brush tip (pass 3). Lights into a
wet dark: 0.57 / 0.38 / 0.89 clean (open / setting / dry) → 0.88 / 0.93 /
0.94; the river shows ground in 90% → 18–35% of its track. Most of the
failures the lab painters hit are fixed (glaze-hand strokes lifting thin
open sky to the ground 42% → 0.1%, crazing over tacky paint, dark rims
over dry paint, tramlines). The sketchbook's "`dry()` before any
passage" rule is replaced (on the branch) by when to work wet, tacky or
dry. Cost: 44 bytes more per pixel in the film (~315 MB at 3200; it was
52 before the maintenance round, ~373 MB, with 8 of them per-stroke
state copied into every undo snapshot), plus 4 bytes of stroke scratch
that snapshots and checkpoints don't carry (`notes/wet.md` §9);
checkpoint format 8.

**What it looks like** (panel 3 judged pass 2, not pass 3; main vs the branch on identical logs:
`notes/round6/panels/judge3/`). The branch wins **foliage C 4–0** (lights
no longer sink) and **l5_near 4–0** (the fog veil at the wood's foot is
gone and the spruces read as trees: `notes/round6/wet/l5_near_bench_cmp.jpg`,
main above). Main wins **rock B 4–0** and sky B, water B and l3_green
3–1. Both Astras told the engines apart blind in all six pairs and one
Gemini in five of six (corrected after review): on the branch later
marks stay discrete, firm and opaque; on main they're absorbed into the
wet paint. The branch loses where passages should melt: stepped cloud
tips, reflection "teeth", the rock's scalloped terminator, the l3_green
hedge's lights as polka dots (that log was tuned while lights sank).
Pass 3 (a dirty brush tip) moved the pictures only 1–2.5/255: the blunt
ends are the flat brush's own footprint stopping. Main hid them by
mixing every mark into the wet paint, which is also what sank lights and
muddied the wood.

**My read.** The physics on the branch is right and main's was wrong;
the branch is the better engine to paint on, but merging it makes the
two benchmarks look different (l5_near better, l3_green mixed) and the
next thing to fix is how a stroke ends: a flat lifting off rolls onto its
edge and drags, a round lifts to a point. Known open issues: paint heaps
where the wet film is already thick (a brush's contact grows with film
thickness); dark over setting paint gives out early; a spent, pressed hog
still shows ground at the end of a river stroke.

## 4. Bare trees (branch `r6-oak`, not merged)
The floating twigs were a painting bug, not geometry: the recipe painted
thick wood by local width but the wood passes selected limbs by base
width, so the thin outer end of every thick limb was never painted (9
limbs and 897 units of wood on the trees_in oak, with 188 limbs and twigs
leaving from them), and the brush faded out over the last half of each
stroke, where the twigs leave. Fixed, with a test that allows no loose
piece bigger than 6 px (the old code left 14 over 3 px). Then:
- `detail=` picks which fine twigs to draw (default 0.35 bare): long
  ones, leading twigs and ones at the crown edge, always with the wood
  they leave from.
- The rope look came from the brush's Catmull-Rom spline rounding every
  elbow into an S, random wandering in growth, and riggers that couldn't
  lay the widths. Oaks (and half-way limes) now grow angular: straight
  runs, turns at nodes (turning at nodes 43% → 80%), width falling at each
  fork, twigs ending in points. Beech and birch keep their habits.
- The twig tone (`t:twig_mass()`) is kept in the API but marked not
  recommended, with the panel's evidence.
- Benchmarks byte-identical; golden unchanged; the `trees_in` study
  changes. Evidence: branch `notes/oak/p2_*.jpg`. My eye: clearly better;
  at 3200 the short straight twig stubs lean a little thorny, and the
  birch's curled side twigs look odd.
- Not touched: the fir's needle pads (the fir's boughs do reach the stem;
  fixing the pads would change l5_near's spruce).

## 5. What to do next (my suggestion)
1. Pick the relief (5 minutes with the sheets).
2. Look at r6-oak's `notes/oak/p2_*.jpg`; if you like it, merge (no
   benchmark changes).
3. Look at `notes/round6/wet/` and the branch's `notes/wet/pass3_*.jpg`;
   if you like the direction, merge r6-wet, then fix stroke endings (the
   flat's lift-off) as the next engine step.
4. Lab round 2 on the merged engine, with hand time on: foliage (the
   unsolved one: internal values in the shade half, holes that aren't
   punched), a rock whose lit face isn't a flat slab, and the bare tree C
   recipe on the angular oak. Then back to the two benchmarks.

Branches: `r6-time` and `r6-lab1..3` are merged into main; `r6-wet` and
`r6-oak` are pushed to GitHub as branches, unmerged, waiting for you.
