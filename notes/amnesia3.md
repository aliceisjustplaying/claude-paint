# Amnesia round 3 (2026-09-23 afternoon): three painters at the easel

The first round painted live at the easel, in Lua 5.5, with round 3's whole
toolbox: drying over time, pointed tips, one world with one sun, a physical
sky, clouds, ranges, greens, foliage, meadows and look-and-fill coverage.
Clean worktrees (no existing paintings, no figure motifs, no round notes).
Three briefs, to test convergence against steering (brief:
`notes/amnesia3_brief.md`).

| painter | brief | picture | time |
|---|---|---|---|
| free | entirely theirs | *Dolmen on the Baltic Shore at Evening*: a Hünengrab on a dune barrow after sunset, two bare oaks, a small figure seen from behind, two sails, a crescent moon and the evening star | 45 min, 38 chunks |
| green | daylight, green season, no moon/twilight | *The Oak on the Common, Summer Morning*: a lone oak in leaf with a dead snag, a shepherd and five sheep, a track, a far church, a pale range, cumulus | 52 min |
| near | a near subject, no moon, no figure from behind | a bedded sandstone block with a young birch growing from a crack, moss and heather on its ledges, rust bracken, dark spruces, crows | 61 min, 26 chunks |

Session logs (replayable), notes and previews (1000px + 3200px crops) are in
`notes/amnesia3/`. Each log replays byte-identically (`easel check`).

## Convergence
Left free, the painter chose twilight, a crescent moon, the evening star, a
figure seen from behind and a dead oak: the same set as all three round-2
painters. Steered, the others made a daylight summer common and a close
autumn rock study without them. So the convergence is a default, not a
ceiling: Claude's idea of "a Friedrich" is that set, and briefs move it.

## How the easel felt (the user's key question)
All three said the same thing in different words:
- **The look–judge–undo loop felt like painting.** "Most of my chunks were
  fine as code and wrong as pictures" (free). Undo made trying cheap: the
  green painter tried three sets of groves and two skies; the near painter
  used `--mode value,squint` to see the rock didn't read and darkened
  everything around it, "a painter's answer, not a programmer's".
- **The physics felt like paint.** A second thin sky over a dry first layer
  turned a flecked sky luminous; stones laid on wet grass picked up green
  until the painter waited a day; cracks drawn into open paint broke into
  dashes; a brush ran dry along a load. "Paint has consequences here, and
  time is a real tool" (green).
- **Drawing things felt like programming.** "A sheep is two ellipses, a
  thistle is a list of offsets" (green). "A painter draws it; I compiled
  it" (near, on four CSG attempts at the rock). "Every mark was a
  coordinate I estimated from a JPEG; I never touched the picture" (near).
  The easel is "much better feedback, much worse drawing" than a program
  (green): painting-like for surfaces, air and light, programming-like for
  things.

## Fixed during integration (this round)
- **Hidden clock jumps** (free #1, green #1/#18, near #5/#10): `glaze()`
  dries the paint under it first (physically right), which moved the
  canvas's clock without telling the easel; the next `wait` then jumped by
  weeks. The easel now reports it (`glaze: waited 9.8 days for the paint
  under it to dry`) and syncs its clock after every chunk.
- **`easel close` overwrote a hand-edited log** (green #10): an edited file
  is now kept as `<name>.edited-N.lua` with a note.

## Shared friction (next round)
1. **Drawing shapes.** No way to draw a shape by eye; every figure, sheep,
   rock and frond is coordinates and CSG. Candidates: a pencil underdrawing
   the painter sketches, looks at and corrects before painting over
   (Friedrich drew first, in graphite; friedrich_materials.md §3); a look
   overlay with a coordinate grid and a probe; shape helpers that take a few
   drawn points and add the irregularity (a "hand" for outlines).
2. **Occlusion by hand.** "Keeping things behind other things is all manual
   masking" (free #3); shadow masks come out as rings (near #2). The world
   knows depth: offer painters "everything in front of X" masks and soft
   cast-shadow masks from bodies placed in the world.
3. **Editing an early chunk** means close, edit, replay (27–110 s). Needed:
   replace chunk N and replay from a checkpoint; keep undone code visible.
4. **Scale traps**: `w:height` vs canvas-unit motifs, `eye=` absolute, `s:size`
   radii, `to_ground` nil at the edge (free #4, green #3/#5/#6).
5. **Defaults too even, dense or bright** (near #3/#4, green #2/#7/#9): sward
   covers everything; ranges boxy; stroke-end blobs square off silhouettes;
   scumble opaque; stipple reads as frost.
6. **Silent no-ops**: `form:part()` returns 0 off the form; low-pressure
   touches lay almost nothing; long strokes run dry early (green #15–17).
7. **Detail at 1000px**: a unit is a pixel, so particulars vanish; a live
   3200px crop (paint a window at full resolution without replaying
   everything) is the most-wanted feature (free).
8. Motif shapes from growth: 10-limb old oaks, spiky roots, savanna crowns
   (free #9/#10, green #8); spruce tiers can't be lit (near #7).
