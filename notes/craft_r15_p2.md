# Craft record: r15_p2 (materials and tools)

Each entry gives the operation, the effect I saw in the paint, and my
explanation or uncertainty. Canvas: 440 mm linen, 1000 units wide, 2400 px
(1 unit = 0.44 mm = 2.4 px). Ground: `Style::friedrich()` with the top
layer replaced by a brushed whitish #dccfb6, 55 µm, stiff 0.4.

## Sky

1. **Thin stratus bands (half-thickness 3–6 units) laid with a filbert 6
   at pressure 0.35–0.6, load 0.45, clipped to soft band masks, into the
   wet sky.**
   - *Effect:* broken dotted dashes, like Morse code, instead of soft
     cloud.
   - *Explanation:* at that pressure the mark is only about 3–4 units
     wide. The lightly pressed bristles reach only the crowns of the
     ground's texture, so the film breaks up along the stroke. The clip
     mask's soft falloff thins it further at the band's edges.
   - *Fix:* bands 5–11 units, pressure 0.5–0.75, load 0.6, then stipple
     and badger across them. They became soft, continuous streaks.

2. **Stipple (tip 2.6, cluster 0, dips every 8) into the wet sky *after*
   two badger passes.**
   - *Effect:* pale, lacy clumps across the upper sky at 1:1.
   - *Explanation (partly uncertain):* each dip is aimed at the target
     over what the canvas shows. Where the laid and blended film came out
     darker than the field, every touch in that patch is lighter than its
     surroundings and stays a separate mark. Nothing fused it afterward.

3. **The same stipple *before* the badger passes.**
   - *Effect:* the lace turned into soft, lighter, wind-drawn wisps, like
     high cirrus.
   - *Explanation:* the clean badger picks up and drags wet paint about a
     stroke's width, so it smears each touch into its neighbours along
     the stroke. The unevenness of the touches survives, but as streaks
     in the blending direction rather than as dots.

4. **Transparent #eaa66a glaze, up to 0.32 coat, deepest at the horizon
   and centred on the glow.**
   - *Effect:* a warm apricot band with no visible pooling streaks.
   - *Explanation:* at this depth the film is thin enough that its
     pooling in the weave's hollows stays below visibility, consistent
     with r15_p1's observation that trouble began above about 0.5 coat.

## Distance

5. **Woods as short upright touches (round 3.2, length 5–14, angle −π/2)
   into a band with a roughened top.**
   - *Effect:* reads as distant trees with a faint vertical grain. The
     earlier level filbert strokes read as a crumbly rock ledge.
   - *Explanation:* the direction of the strokes carries the texture; at
     this size the eye takes vertical texture for trunks.

6. **A church drawn as polygons and painted with the sable (`detail`,
   clipped).**
   - *Effect:* a crisp silhouette. The first version (tower 8 wide and 76
     tall) read as a minaret. A tower of 12, a steep spire, a high nave
     roof, a lower choir and a ridge turret read as Gothic.
   - *Explanation:* this is geometry, not paint. At 30–90 units the
     proportions carry all of it.

## Snow

7. **A drift height field seen in perspective (fBm in ground coordinates:
   X/(y − horizon), 1/(y − horizon)), its slope shifting the colour; broad
   lay; stipple; badger along the slope.**
   - *Effect:* the whole field read as fog, or as a sea of cloud seen from
     above.
   - *Explanation:* the blender moved the colour variations into soft
     billows with no edges. Soft billowy value shapes with no objects
     standing on them are how clouds look, not ground.

8. **Crisp drift ridges: body strokes of darker colour clipped to a mask
   with a hard top edge (the crest) and a soft bottom, `color_over`
   darker by 0.06 L.**
   - *Effect:* thin dark lines across the plain, like ripples on a lake.
   - *Explanation:* the bands were narrow (depth 0.11 × distance below the
     horizon) and evenly spaced. Each was one clipped film with the
     mask's 1.2-unit edge. Long, parallel, even-width marks read as
     water. Widening them and softening the crest edge made them read as
     long swells, but they were still too banded. I dropped them.

9. **The final snow: broad lean filbert (medium 0.22, coverage 3.5),
   clipped, swept down, then two level badger passes (0.3–0.4, then
   0.25–0.35, coverage 2.5 and 2.0), no stipple.**
   - *Effect:* smooth, and it reads as snow once grass, posts, the path
     and the tree's foot stand on it. A few faint lighter oval blotches
     remain at 1:1.
   - *Explanation:* the blotches are at the scale of a stroke start
     (`lay` puts the thickest film at the loaded start). Over a light
     ground, thick and thin films differ in tone. The blender evens
     variation within about a stroke's width but not the ends.

10. **Stipple into the wet snow (tip 2.8, coverage 1.1–1.3, cluster 0)
    followed by only a light badger (0.22–0.3).**
    - *Effect:* pale lace patches, like foam.
    - *Explanation:* the same as 2. A light blender doesn't fuse the
      touches. I dropped stipple in the snow rather than blend harder.

11. **Unclipped broad snow strokes along the far snow line.**
    - *Effect:* a whitish rim painted up over the dark woods.
    - *Explanation:* `hug` moves stroke centres to the region's edge and
      unclipped strokes overrun it by design. `clip(true)` fixed it.

12. **Lean snow (0.22 medium, coverage 3.5) over a dark band of woods
    paint mistakenly left under the rise.**
    - *Effect:* the dark band showed through as a grey stripe.
    - *Explanation:* lead white at medium 0.22 has its K and S scaled by
      0.78. A few coats of it over near-black don't reach full hiding.
      Masking the woods off the rise solved it; more paint would have
      been the painter's other answer.

## Path

13. **Three clipped passes of a round 4 along the path's direction
    (`color_over` shifts: floor −0.025 L, left wall +0.012 L, right wall
    −0.04 L), after `dry`, on roughened masks.**
    - *Effect:* at the first contrasts (−0.075 on the right wall) and
      with a nearly straight course, the path was a hard blue canal or a
      ski track. Softer shifts, a Catmull-Rom course through eight
      control points, a narrower width (0.075 × distance below the
      horizon) and a fade before the woods made it a trodden track.
    - *Explanation:* over dry paint each stroke sits on top as a separate
      film. The clip gives an exact, ruled edge. A roughened mask (period
      12, amount 1.2) breaks it.

14. **Footprints: alternating short touches of a round (width 0.35 × the
    path's half-width), spacing 0.55 × half-width, stepping up the
    path.**
    - *Effect:* a faint dotted rhythm that diminishes naturally with
      distance.

## The oak

15. **Geometry: a recursive branch generator (kinks per step ∝ √step,
    knots by chance per length, laterals every 2.6 w + 5 units, forks at
    the tip, taper 0.16–0.22), with a lower cut-off of 0.32 units.**
    - *Effects and fixes:*
      - Per-step kinks of fixed size made thin twigs curl, because their
        steps are short and the random walk accumulates. Scaling the kink
        with √step fixed it.
      - Heavy limbs curled into loops. A pull back toward the initial
        direction (0.12 per step for w > 3) and a pull up when heading
        down stopped that.
      - Oak character came from "elbows": a turn of 0.35–0.7 rad answered
        by 60–100 % of it on the next step, with probability 0.28 for
        wood over 2 units.
      - A stag-headed leader: a limb whose children under 1.3 units are
        not grown.

16. **Heavy limbs (≥ 2.6 units) as filbert drags along the limb, side by
    side, clipped to the union mask of all limbs.**
    - *Effect:*
      - Painted straight after the snow stage, into wet snow, the trunk
        came out pale grey and see-through. The brush picked up the wet
        lead-white paint and mixed it into the bark. `c.dry()` first
        fixed it.
      - With a light "lit side" colour on the outer strokes, the limbs
        looked like tubes. Where a thin limb joins a thick one, its
        offset strokes painted lit colour across the thick one, as pale
        streaks.
    - *Explanation:* the clip mask is the union, so it can't keep a stroke
      on its own limb. The lit edge needs its own mask (see 19).

17. **`Shape::ribbon` for a whole limb polyline.**
    - *Effect:* a gap in the leader with a jagged pale fringe.
    - *Explanation:* at a sharp elbow the ribbon's offset outline crosses
      itself, and the fill cancels in the twisted part. The stroke ends
      at the gap left bristle serrations.
    - *Fix:* one ribbon part per segment plus a disc at each joint;
      `Shape::add` parts never cancel.

18. **Crossing drags into wet paint at limb junctions.**
    - *Effect:* thin, pale, streaky patches at the trunk top and at forks.
    - *Explanation:* each bristle picks up a share of the wet paint it
      passes (pickup 0.18). Strokes from several limbs crossing the same
      spot thin the film, and single drags have no look-and-fill.
    - *Fix:* one `c.work` pass of dark bark (filbert 4, coverage 1.4,
      clipped) along a grain field over the whole limb mask.

19. **Sky-lit rim on a mask = limbs × (1 − limbs shifted 1.6, 1.9 units
    up-left), stroked along the grain with a round 1.8 in cool grey.**
    - *Effect:* a consistent thin cool edge on the upper-left of every
      limb, with no streaks at junctions.
    - *Explanation:* the edge is a property of the silhouette, not of
      each limb, so deriving it from the union mask is right.

20. **Bark texture: fissures (−0.035 L, round 1.6 with point 0.5,
    coverage 0.9) and ridges (+0.03–0.05 L, coverage 0.5), along the
    grain.**
    - *Effect:* at +0.03–0.05 the ridges made the limbs silvery, like
      birch or planks. At +0.014 ± 0.01 and coverage 0.35 they read as
      bark.
    - *Explanation:* on a dark field a small absolute lightness step is a
      large relative contrast.

21. **Twigs: one stroke each of a pointed round (point 0.7, width 1.5 ×
    root width), pressure from `pressure_for(root)` to
    `pressure_for(0.7 × tip)`, ramps (0.03, 0.35).**
    - *Effect:* tapering twigs with sharp points, heavier at the fork,
      lighter than the limbs where they are thinnest. 4,819 strokes took
      1.2 s.
    - *Explanation:* the cone narrows as pressure falls and on the lift.
      Marks under a pixel wide cover part of each pixel, so they look
      lighter over the pale sky, like real fine twigs against light.

22. **Snow on limbs: a pointed round (0.42 × limb width) along the
    upper-side offset (0.36 w) of runs of segments within ±38° of level,
    bluish #c9cad6.**
    - *Effect:* thin lines of snow on the level limbs, which read well.
      Short runs of one or two segments on the thick leader made little
      white "combs".
    - *Explanation:* a wide stroke over a very short path shows its
      bristle ends.
    - *Fix:* keep runs of at least three points and skip the trunk.

## Foot of the tree

23. **Drift at the foot: body strokes clipped to a mound mask, then a
    badger over it.**
    - *Effect:* a ghostly dark "reflection" of the trunk under the snow.
    - *Explanation:* the ribbon's round end cap reached about 26 units
      below the trunk's base. The snow body over it didn't hide it fully,
      and the badger dragged the dark paint down.
    - *Fix:* two opaque passes at medium 0.1, no badger, the trunk
      shortened, and the limb mask cut at the snow line.

## Vegetation, figure, birds

24. **Grass tufts: pointed round blades (point 0.95, width 0.2–0.65),
    pressure from `pressure_for(w)` to 0.04, release 0.6; a small blue
    hollow touch at each base; seed heads as a short dark stroke.**
    - *Effect:*
      - Evenly scattered small tufts in ochre read as confetti or
        seedlings.
      - Clustered (44 patch centres, spread ∝ distance) in grey-browns,
        with 4–22 blades and a few broken over, they read as dry grass
        through snow.
      - The blue hollow seats each tuft in the snow.
    - *Explanation:* the pattern is geometry. The hollow is a small value
      contact shadow, as in 23.

25. **Flying crows as a V (wings up at the tips).**
    - *Effect:* a "smile".
    - *Fix:* an M (wings bowed up from the body, drooping to the tips)
      reads as a bird at once.

26. **Walker: sable clipped to a polygon silhouette, then a rim mask
    (figure × (1 − figure shifted 0.9 units up-left)) in #4e4a55.**
    - *Effect:* the rim is barely visible at the whole view but takes the
      cut-out edge off at 1:1.

## Glazes and finish

27. **Semi-opaque #f3ecd9 halo glaze (0.22 coat, 7-unit blurred disc) and
    one touch of a round 2.6 at pressure 0.75 for the evening star.**
    - *Effect:* a small bright point with a soft glow, visible at the
      whole view. At round 1.6, pressure 0.5, it vanished.

28. **Transparent #7c7f9a up to 0.22 coat over the bottom 110 units, and
    #6d6270 up to 0.22 coat toward the corners.**
    - *Effect:* the near snow deepens to violet and the corners close in
      gently, with no streaks.

29. **Cracks at `width_um` 4, dirt 0.12, veil 0.15, hierarchy 0.5, patchy
    0.6; varnish 0.22 ± 0.06.**
    - *Effect:* hairlines visible only at 1:1 in the sky.

## Computation

30. **Timing.**
    - Whole render about 148 s: ground 36, sky 72, snow 23, oak with
      twigs 3.6, bark 4, the rest under 1 s each.
    - A resume from `path` or later takes 8–12 s.
    - The tree geometry (about 5,000 branches) is instant.
31. **Reproducibility.** A render resumed from the `twigs` checkpoint was
    byte-identical to the from-scratch render.
32. **Staleness.** Any edit above `main` stales everything, including a
    `use` line and helper functions. I tagged the tree generator
    `// ckpt: from oak`. Geometry shared with the drawing stage (the path)
    can't be tagged honestly, so edits to it cost a full render.
33. **Checkpoints** are 428 MB each at 2400 × 1714; fifteen stages
    filled 6 GB of `out/`.
