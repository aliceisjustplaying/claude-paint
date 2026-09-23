# Amnesia round 2 (2026-09-23 night): winter, coast, mountains

Same themes as round 1, so the two rounds compare directly. Three painters
that had never seen any painting from this project each got a clean
worktree: engine, research notes, tonight's feature notes and study
programs. They got no Moonrise, no monk2, no round-1 programs, no figure
motifs and no git history. They were not allowed to change the engine.
Brief: `notes/amnesia_brief.md`. Programs are archived (not built) in
`paintings/fresh2/`; each painter's full notes (composition, method,
friction, critique) are in `notes/amnesia2/fresh2_<theme>.md`, with 1000px
previews and 3200px crops beside them.

All three finished a complete painting with a full render in 54–57
minutes. Round 1's sessions were cut off by a power loss before their
final reports.

| theme | picture | files |
|---|---|---|
| winter | *Winter Evening with a Ruined Choir*: dusk, a Gothic ruin in mist, a stag-headed oak with snow on its limbs, spruces, a broken fence, a frozen brook and a walker seen from behind | `amnesia2/fresh2_winter.jpg`, `_3200_crop.jpg` |
| coast | *Morning on the Shore at Arkona*: before sunrise, fishermen's poles with a drying net, an erratic boulder, a woman at the water's edge, a brig on the horizon | `amnesia2/fresh2_coast.jpg`, `_3200_crop.jpg` |
| mountains | *Daybreak in the Riesengebirge*: a wanderer by a granite tor above a sea of fog, the Schneekoppe with its chapel, wind-thinned spruces | `amnesia2/fresh2_mountains.jpg`, `_3200_crop.jpg` |

## What improved since round 1
- **Skies**: all three skies are smooth stippled gradients with no ruled
  horizontal strokes. The coast painter: "no visible strokes, and the dry
  stipple gives it grain up close without salt." The skies also differ
  from each other (violet to straw, lemon through rose to slate, lemon to
  gray-blue). Round 1 produced the same sky three times.
- **Color hits its target**: the complaints about dark specks from
  "appearance over white" are gone. Aiming creates new, smaller problems
  (below).
- **Distance and mist**: ranges pale layer by layer, and the ruin and the
  Schneekoppe sit convincingly in haze.
- **Trees**: the dead oak is coherent and stag-headed. The spruces are
  still generic.
- **Solids**: the coast's erratic "reads as a heavy granite block ... no
  egg shape." The tor still reads a bit like stacked loaves.
- **Workflow**: crops and resume were used heavily. Iterating on details
  became affordable.

## Friction shared across painters (engine work for next time)
Numbers refer to items in each painter's FRICTION list (W = winter,
C = coast, M = mountains).

1. **Craquelure preset is wrong for this style and resolution-blind**
   (W1, C8, M1, M11). `Finish::aged` assumes a 60 µm ground while
   `Style::friedrich` primes 240 µm; 70 µm hairlines become full dark
   pixels at 1000px and a grid at 3200px. Derive it from the style's
   ground and render hairlines sub-pixel.
2. **Near-zero glaze amounts make hard edges and rectangles** (W11, C1).
   `Canvas::glaze` applies wherever the amount is > 0, and blurred masks
   and smooth falloffs leave tiny float remainders. Needs a physical
   cutoff (a film can't be thinner than a few µm; below that it's dry
   brush or nothing) and a check of `settle` dividing by tiny volumes.
3. **Blender and clipping** (C5, M14, M9). `Style::blend()` isn't
   clipped, so badger passes smear across mask edges; hidden layers leak
   under later passages and only show at 3200px.
4. **Coverage thins at mask edges; ground flecks through darks** (C2, C3,
   C12, M3, W15). Strokes seeded past edges helped, but edges of masked
   passages still thin out.
5. **Aim over contrasting paint** (C1, C2, C13, C15, W14). Thin light
   marks over cool darks come out orange or salmon. A stroke is judged at
   its center (a fleck of bare ground there skews the whole pile). Thin
   edges show their strongest tube. Aim should look along the stroke,
   weight by where the paint lands and prefer the painter's paint family.
6. **Color fields can't see the canvas** (W18, C6). Painters want "the
   snow as it actually is here, bluer" in a closure; `c.work` borrows the
   canvas. Offer a snapshot or a `color_over(|x, y, under| ..)` form.
7. **Stipple lighter than the field or over a dark reads as salt or
   static** (C9, M10). Needs density-dependent contrast (touches fade
   toward the underlayer where coverage thins) or a veil mode.
8. **Small marks become beads and blobs at 3200px** (M12, C14, W9).
   There's no pointed-tip model for small birds, spruce tips and grass.
   Needs a tapered-tip brush model (hairs converging to a point, which
   lifts off to a hairline) and a resolution-independent minimum mark.
9. **Checkpoint UX** (M4, M5, M18, W2, W20, C19). `--stop far_range`
   never matches the stage "far range"; `--resume X --stop X` runs to the
   end. Staleness is too eager (any edit above a stage invalidates it) and
   `--stale-ok` doesn't refresh. Needs normalized names, an error for
   unknown stage names and a clearer staleness model.
10. **Closure ergonomics** (W13, C7, M6). `Fbm` and `per_column` closures
    aren't `Copy`, so every field needs clone gymnastics.
11. **Growth and form motifs** (W4, W5, C11, M7, M8). Grown limbs are
    "smooth noodles" (elbows rounded off); roots and low sprouts look
    wrong. Ellipsoid boulders read as eggs. `Sdf::block` size means whole
    extents, which isn't documented where you call it. Contre-jour faces
    get too much light.
12. **Possible bug** (C17): a curved multi-point `drag` with a fine brush
    laid nothing, while straight strokes nearby did. There's a
    reproduction description in the coast notes.

## My read
This round shows the engine working as a painting app for agents: three
blind painters made three different, recognizably Friedrich-like pictures
in under an hour each. Nobody had to invert the optics by hand to make a
mark match its field, give up on stippling or fall back to ruler strokes
for a sky, which were round 1's main fights. The remaining look problems are mostly foreground
particularity (the beach and the snow field are the dullest passages; the
painters said so themselves), small marks at full resolution and the
crack preset laid over everything. The shared friction list above is the
next engine round.
