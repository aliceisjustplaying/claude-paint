The painting is finished: **"Evening over a Misty Valley in the Mountains"**, in Friedrich's manner.
- **Scene:** late October, just after sunset, seen from a dark heath knoll. Four mountain ranges step back through a mist-filled valley to a far chain with one rounded summit against the afterglow.
- **Figures and sky:** two tall spruces and two young ones stand on the left. A small bare-headed figure with a staff stands on the right, seen from behind, against the palest mist. A thin crescent moon hangs above him, and two faint cloud streaks lie in the glow.

A fresh render from scratch came out byte-identical to the one built up from saved stages, so the program reproduces the picture exactly.

## Render paths
- Final render: `out/r15_p1_full.png` (2400 × 1655 px), from `cargo paint r15_p1 -- --full --width 2400`
- Program: `paintings/src/bin/r15_p1.rs`
- Notes: `notes/r15_p1.md` (composition, stages, limitations, critique) and `notes/craft_r15_p1.md` (materials and tools record)

## Tool limitations met, and workarounds
1. **Poured glazes printed the canvas weave.** A deep glaze (`Canvas::glaze`) pools in the thread hollows and showed as regular vertical streaks, like rain, over the dark ranges. I stippled the mist into each range while it was wet, kept glazes to at most 0.45 coat, and repainted one range darker instead of glazing it.
2. **Stippling on dry paint left blotches.** Each touch stays separate, and each refill of the brush covers about 20 touches at one tone, so the sky dried as lacy patches and the mist looked like snow. Stippling into the wet layer, with no clustering and a refill every 8 touches, made the touches fuse.
3. **A brushed glaze over a dry pale passage failed.** It went onto the thread tops only, speckled, and came out lighter than the paint under it. I don't know why it lightened. I dropped it and repainted the range darker.
4. **Clipping a fill to a mask reproduces the mask's outline exactly.** Clipped fills made the firs look like stacked slabs. I built the silhouettes from single tapering, drooping branch strokes of a pointed brush, with a thin clipped spine.
5. **Unclipped strokes overran the crest.** One left a sliver standing up from the knoll. Clipping, plus a fine brush along the edge, gave a clean crest.
6. **Two separately softened glaze masks left a seam.** They either left a light gap line or, when overlapped, a dark rim along the crests. Masks built as exact complements (`m` and `1 − m`) removed it.
7. **Rocks lit by the `Form` solids looked pasted onto the ground.** There is no way to set them into the ground. I measured each rock's height per column, brushed heath back over its lower half, and added grass along that line.
8. **There is no figure tool.** Hand-drawn polygons worked, but a head with a flat cap read as a chimney at 140 px, so the figure went bare-headed.
9. **Thin lit cloud edges came out as dashed white lines.** The stroke planner lays separate short strokes along a thin band. I dropped those edges.
10. **Default craquelure dominated the picture.** `Cracks::aged` made a heavy dark network, so I made the cracks narrow and clean with a weaker hierarchy.
11. **Checkpoints go stale too easily.** Any edit above a stage stales it, including removing an unused import. I used `--stale-ok --ckpt` once for that harmless edit and kept stage-specific helpers between the stages that use them.
12. **The badger swirled paint where a ridge band was cut off at its end.** It also left a light fleck. Reshaping the ridge to slope down into the mist fixed it.

The main remaining weaknesses: the upper sky is slightly blotchy, the clouds are schematic, and the foreground and rocks lack Friedrich's fine detail.
