# Craft record: r15_p3 (materials and tools)

Each entry gives the operation, the effect I saw in the paint, and my
explanation or uncertainty. Canvas: 440 mm linen, 1000 units wide, aspect
1.3, 2400 px (1 unit = 0.44 mm = 2.4 px). Ground: `Style::friedrich()` with
the top layer replaced by a brushed whitish #d9c9ae, 55 µm, stiff 0.4.

## Sky

1. **One lean layer (filbert 22, medium 0.3, coverage 4, swept down),
   stipple into it wet, then three badger passes (pressure 0.35–0.45,
   0.25–0.35, 0.2–0.28).**
   - *Effect:* at the whole view the upper sky carried pale wisps. At 1:1
     they were a lattice with the weave's pitch: bare pale ground on the
     thread crowns, darker paint between. The third, lightest pass made
     them stronger, not weaker.
   - *Explanation:* the badger lays nothing and picks up (pickup 0.15).
     Its bristles touch the crowns first, so every pass takes a little film
     from the tops and leaves it in the hollows. On a lean single layer the
     crowns are soon near bare, and the whitish ground shows through in the
     weave pattern.
2. **The same first layer with two badger passes, dried, then a second
   thin layer (medium 0.42, coverage 3) and two badger passes.**
   - *Effect:* a smooth, luminous gradation with only faint cirrus-like
     unevenness. No lattice.
   - *Explanation:* wherever the second layer is lifted or thin, it now
     shows the first layer's colour, not the ground. The contrast of each
     flaw drops to the small difference between two aims at the same
     target.
3. **Stratus bands laid into the wet first layer before the stipple and
   badger passes.**
   - *Effect:* smeared, smoky pink-violet patches with bristle streaks.
   - *Explanation:* the full-sky badger passes (40 units wide, 120–300
     long) drag the band paint across much more than its width.
4. **Bands laid into the wet second layer *after* its badger passes, with
   one light local badger pass (pressure 0.18–0.26, over the band mask
   dilated by 6 units).**
   - *Effect:* soft-edged bands keeping fine bristle striation along their
     length, which reads well at 1:1.
   - *Explanation:* the local blender fuses the band edges into the wet sky
     without dragging them across it. The bristle ridges of the 6-unit
     filbert survive along the stroke.
5. **Band geometry.**
   - *Effect:*
     - Spindle bands (thickness ∝ sin(πt)^0.6, 3.5–6 units) read as fat
       stripes.
     - Thinner (sin^1.1) they read as floating "cigars".
     - Broken by a noise gate of about 80-unit period they became Morse
       dashes (as r15_p2 found).
     - Long strands (400–650 units, thickness ∝ sin^0.8 × noise 0.35–1.35)
       with a gate of about 270-unit period, mostly on, read as dawn
       stratus.
   - *Explanation:* the eye reads the shape of a mark's ends. Frequent
     rounded ends make objects; rare, tapering ends make a layer.
6. **Asymmetric band edges (3.6 units soft above, 1.8 below) and a lit
   lower rim (round 2.4, salmon near the glow, grey-lilac away), into the
   wet band paint.**
   - *Effect:* subtle at the whole view; at 1:1 the lower edges are
     slightly warmer and firmer.
   - *Explanation:* the rim paint mixes into the wet band paint under it,
     so its step in value is small. That suits light from below the
     horizon.
7. **Transparent #e9a66b at 0.22 coat over the glow, fading to 0 at
   y = 300, and on the head of its path in the sea.**
   - *Effect:* a warm apricot band with no pooling streaks. At 0.26 it
     began to look like a postcard.
   - *Explanation:* consistent with r15_p1 and r15_p2: below about 0.3
     coat the film's extra depth in the weave hollows stays below
     visibility.

## Sea

8. **Broad level lay clipped to a mask whose edge is a 1-unit smoothstep
   at the horizon, and a level badger pass (cross 0.02, clipped).**
   - *Effect:* a crisp, straight, found horizon at 1:1 with no overrun into
     the sky.
   - *Explanation:* the clip multiplies every bristle's contact by the
     mask, so the edge is the mask's edge. The blender, clipped too, can't
     drag the sea into the sky.
9. **A mirror colour field: at depression d the sky at 2d above,
   reflectance 0.92 → 0.57, over a dark body colour.**
   - *Effect:* the far sea pale and warm, the near sea cooler and bluer. At
     2.4d the near sea turned a saturated blue; at 2.0d, with a greyer body
     colour, it sat right.
   - *Explanation:* at 2.4d the near water mirrors the zenith's slate blue.
     This is a choice about wave slope, not paint.
10. **Ripples: short level sable strokes in four depth bands (tool 0.9 →
    2.4, lengths 4–12 → 14–50, coverage 0.35 dark, 0.5 light), after
    drying.**
    - *Effect:* the water surface at once: fine and dense at the horizon,
      broader in front. The light strokes, loaded mostly in the glow's
      column (`load_at`), make a broken glitter path.
    - *Explanation:* over dry paint each stroke sits on top as a separate
      film with tapered ends (`ramps(0.3, 0.4)`). The perspective lives
      entirely in the band parameters.
11. **The lap of foam: a round 1.2 along the scalloped edge, `color_over`
    +0.07 L, broken by a noise gate; its shadow just below at −0.05 L.**
    - *Effect:* with an absolute near-white colour it was a dotted white
      line (like a road marking). Relative to the water under it, it reads
      as a thin wet lip.

## Beach and dune

12. **Tide pools: lens masks painted with the reflected sky at the
    mirrored elevation.**
    - *Effect:* with six pools at reflectance 0.82 they were bright
      paper cut-outs. Three thinner ones at 0.6 with softer ends sit in the
      sand.
    - *Explanation:* flat water is a better mirror than rippled sea, but a
      pool is a small, sharp-edged, uniform shape, so its contrast must
      stay low.
13. **The glow's path carried onto the wet sand: round 2.4, +0.06 L warm,
    in a column that widens toward us.**
    - *Effect:* the most useful single touch on the beach. It links the
      figure to the light.
14. **Dune colour blended toward the beach colour where the dune mask
    fades.**
    - *Effect:* without it, the dune ended in a hard slab edge even though
      the mask was soft. The darker colour gave the edge away.
15. **`Mask::roughen(seed, period, amount, edge)` on a mask with a wide
    soft fade.**
    - *Effect:* a hard diagonal edge where the fade crossed 0.5. After a
      glaze it read as a cut bank.
    - *Explanation:* roughen displaces and re-thresholds, with `edge` as the
      new transition width. It discards any softness wider than that.
      Roughen first, then multiply the soft fade in.
16. **Pencil under the dune.** The 2H crest line, drawn where the painted
    crest no longer is, showed through a body layer at coverage 2.8 and
    load 0.64 as a thin dark line, like a crack.
    - *Fix:* coverage 3.6 and load 0.8, and later a glaze, hid it.
    - *Explanation:* the graphite sits in the dry picture, and a thin body
      film composites over it by KM. This is the "shimmer through" of the
      research notes, but it reads as a flaw when it lies on nothing.
17. **Grass: tufts of 6–20 pointed-sable blades (point 0.95, width
    0.35–0.75, pressure `pressure_for(w)` → 0.04, release 0.6).**
    - *Effect:*
      - With 30 % of dips at #6f6a52 the tufts read as a light green
        lawn.
      - At #302e24 with 12 % #555039 they read as dune grass: dark against
        the sea along the crest.
      - Scattered one by one down the bank they were polka dots. In 26
        clustered patches they were better.
    - *Explanation:* the dune under them was #554c3d–#453e33. A blade
      lighter than its ground is seen as a separate bright thing.
18. **Umber transparent glaze (0.45) over the bank, deeper to the left and
    down, keeping the crest.**
    - *Effect:* the dune darkened to a repoussoir. The grass on it
      darkened too and nearly vanished, which suits a shaded bank.

## Boulder (`Form`)

19. **An ellipsoid with three cuts, lit from behind-left (−1, −0.35,
    front −0.5), ambient 0.35. Colour from `sky`, `n.z` and `direct`.**
    - *Effect:*
      - With a lit colour up to #75757b and sky ≥ 0.55, it was a pale
        pillow, as r15_p1 found.
      - With #5f5b56 only where sky ≥ 0.62, a face of #2f2a26–#3b3937 and
        the warm rim limited to direct ≥ 0.45, it reads as dark stone
        against the light.
      - Two unioned ellipsoids made a boxy loaf. A main mass high left of
        centre plus a lower shoulder gives a boulder's silhouette.
    - *Explanation:* an ellipsoid's upper half has a large area with a
      high `sky` term, so any light colour tied to `sky` covers most of
      the silhouette. Against a glow, only the crown and a thin rim may be
      light.
20. **Filbert along `form.across`, a light blend along it, then
    short sable marks along `form.fall` with ±0.035 L random shifts.**
    - *Effect:* granite-like broken texture on the crown at 1:1, invisible
      in the dark face at the whole view.

## Figure, net, ship, moon

21. **`color_over` darkening passes of vertical strokes that cross the
    horizon (the net's veil; the first figure reflection).**
    - *Effect:* lighter marks, where I wanted darker ones.
    - *Explanation:* each stroke's pile is aimed from the underlayer
      sampled along its path, weighted toward its loaded start (nine discs,
      median per OKLab channel). A stroke starting in the pale sky gets a
      pile a little darker than the sky, and that pile is much lighter than
      the sea where most of it is laid.
    - *Fix:* `Canvas::glaze` with `Pigment::transparent` and a mask. A
      glaze darkens every pixel relative to itself. For the net the depth
      varied by an fBm stretched vertically (folds); for the reflection it
      faded over 26 units below the feet.
22. **Grey sails with `detail()` aimed on the canvas (coverage 4).**
    - *Effect:* pale lavender sails, a "white yacht".
    - *Explanation:* a probe of `Palette::aim` showed that for a thickness
      of 8 coats (the round's 4.0 per load × overlap, clamped) the best
      pile was 85 % pale smalt, semi-transparent. On a 9-unit clipped
      shape the real film is nearer 1 coat or less, where that pile looks
      #ad9b94 to #cfbba3 over the sky.
    - *Fix:* `by_masstone()` and a full load. The sails became an even
      mid-grey.
23. **Figure body from a smooth polygon, sable at coverage 4, threshold
    0.05, colour by region (hair, shawl V, dress).**
    - *Effect:*
      - When the head took the shawl colour she read as a bald man in a
        brown coat. Separate near-black hair and a knot at the nape made
        her a woman.
      - Her mirrored reflection started 3 units below the hem, and she
        floated. Mirroring about the feet, plus a small dark contact
        glaze, seated her.
24. **Moon: three sable strokes along the arc, clipped to disc − disc
    offset 0.38 r away from the sun.**
    - *Effect:* a clean crescent with its horns turned away from the sun.
      The ashen light at +0.012 L was a visible pale textured disc; at
      +0.005 L and coverage 1 it is barely there.

## Checkpoints and computation

25. **Staleness details.**
    - Blank lines count: one untagged blank line left after a `// ckpt:
      end` staled all checkpoints.
    - A redundant-parentheses cleanup inside the shore block staled every
      stage from shore on.
    - A `// ckpt: from sky` region used only by sky2 needlessly staled
      sky.
    - Tagging `sky()` (above `main`) `from sky` kept the drawing
      checkpoint valid after later edits to the sky colours.
26. **Timing.**
    - Ground 38 s, each sky layer about 60 s, sea 11 s, beach 20 s,
      boulder 7 s, the rest under 2 s each; 203 s from scratch.
    - Resumes from `gulls` took about 5 s, from `ripples` about 40 s.
27. **Reproducibility.** A from-scratch render of the final code was
    byte-identical to the render assembled from resumed checkpoints
    (several adopted with `--stale-ok` after no-op edits).
28. **Checkpoints** were 443 MB through the beach stage and 549 MB from the
    shore stage on. I assume the difference is the state some later
    operation adds, but I didn't identify it. 17 stages took 7.6 GB.
