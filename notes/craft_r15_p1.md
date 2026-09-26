# Craft record: r15_p1 (materials and tools)

Each entry gives the operation, the effect I saw in the paint, and my
explanation or uncertainty. Canvas: 440 mm linen, 1000 units wide, 2400 px
(1 unit = 0.44 mm = 2.4 px).

## Ground

1. **A brushed whitish top ground (#d9ccb2, 55 µm, stiff 0.4) over two knifed
   layers (110 + 70 µm).**
   - *Effect:* a warm light ground with faint drifting bristle striations.
     Thin sky paint over it stays luminous; the striations show as a slight
     directional grain in the thinnest passages (low sky, mist).
   - *Explanation:* 180 µm of knifed ground levels most of the weave, so the
     visible micro-texture comes from the brushed layer, as the research notes
     say for his grounds [KÖR p.284]. The weave was not fully gone, though
     (see 9).

## Drawing

2. **`graphite::hand_line` with 2H along the crests, HB for firs and figure.**
   - *Effect:* the 2H lines broke up on the tooth and were barely visible
     through the lean sky layer. HB fir axes showed clearly as grey lines in the
     glow until the firs covered them.
   - *Explanation:* the harder lead lays less and sits on the crowns. KM
     compositing of a thin sky film over graphite lets it "shimmer through"
     [CATS p.132]. Body colour (the firs) hides it.

## Sky

3. **Broad filbert, lean (medium 0.3), coverage 4, strokes 160–340 units swept
   top to bottom, aimed at a gradient; two badger passes.**
   - *Effect:* a first attempt at medium 0.45 and coverage 3 left violet-blue
     strokes with pale ground flashing between them (thin film, gaps). Leaner,
     fuller paint with the second, crossing badger pass gave a continuous layer
     whose tonal unevenness reads as faint high cloud.
   - *Explanation:* at medium 0.45 the film's K and S are scaled by 0.55, so
     one coat hides the light ground poorly and every gap or thin edge shows.
     The blender moves wet paint only a stroke's width. It evens stroke-scale
     variation but leaves variation at the passage scale (~1.6 stroke lengths),
     which is what survives as the cirrus.

4. **Stipple over the dried sky (tip 2.6, coverage 1.3, dips every 20).**
   - *Effect:* the gradient improved at the passage scale, but at 1:1 it was a
     lace of lighter, worm-like clumps.
   - *Explanation:* over dry paint each touch sits on top, so the passage is a
     mosaic of discrete films. Each dip's pile is aimed at the centroid of the
     ~20 touches it serves, so neighbouring groups differ slightly in tone. The
     default `cluster` 0.15 also groups touches. Where the first layer was darker
     than the target, the touches were lighter and stood out.

5. **The same stipple into the wet sky (`cluster(0, None)`, dips every 8,
   coverage 1.4).**
   - *Effect:* a fine even grain; the lace almost vanished. A few faint pale
     flecks remain.
   - *Explanation:* touches into open paint lift some of it and split their
     film into it, so each mark is partly the mixture of the two. More frequent
     dips shrink each pile's patch. This is closer to what the notes describe as
     his stippled skies, where the thin paint pools in the ground's texture.

6. **Transparent glaze of #e9a265 up to 0.4 coat over the low sky (after
   drying).**
   - *Effect:* the pale apricot deepened to a warm orange band above the far
     chain without losing the gradient. At 0.55 coat it became a hot,
     postcard orange.
   - *Explanation:* a transparent layer multiplies the light reflected from the
     pale sky below it (KM with low S). Over a light underlayer that gives
     saturation without darkening much, so the depth is sensitive to amount.

## Ranges and mist

7. **Each range: body filbert clipped to the crest band, strokes along the
   crest's slope (`atan` of its derivative × 0.8), colour field from crest colour
   down into mist colour.**
   - *Effect:* crisp, found crest edges with slight bristle build-up (a hair
     darker at the very edge), and smooth down-slope gradients.
   - *Explanation:* with `clip(true)` each bristle's contact is multiplied by
     the mask, so the edge is the mask's 1.6-unit smoothstep. Strokes ending at
     the edge plough a little paint there, which gives the faint dark line.

8. **Mist glazed (semi, 1.2–1.6 coats) over the dried ranges.**
   - *Effect:* soft and atmospheric at a distance, but at 1:1 there were
     regular vertical streaks ~3.6 px apart (≈ 0.67 mm), like rain.
   - *Explanation:* the glaze levels and pools in the surface hollows. The
     spacing matches the 15 threads/cm warp, so the weave relief still comes
     through the 235 µm ground and the paint over it. A deep poured film is
     thicker in every interstice, so it prints the threads. I am unsure why the
     vertical threads dominate over the horizontal (13/cm) ones. Perhaps the warp
     crowns are higher, or the brushed top ground's striations happened to run
     mostly across them here.

9. **Mist stippled onto the dried ranges instead.**
   - *Effect:* white "snow" dots on the dark range paint.
   - *Explanation:* as in 4. Over a dark dry field each light touch is a
     separate opaque fleck.

10. **Mist stippled into the wet range, then badger along the slope, dry,
    faint semi glaze (0.3 × amount).**
    - *Effect:* the mist reads as mist: soft, level wisps, with the range
      dissolving downward. No streaks and no dots.
    - *Explanation:* wet-into-wet touches fuse with the range paint, and the
      blender drags the fused paint along the slope. The veil left to glaze is
      thin enough that its pooling contrast is below visibility.

11. **Transparent violet glazes along the far chain (0.3) and third range
    (0.45) crests, fading 45 units down the flank.**
    - *Effect:* the far chain set back against the deepened glow and the ranges
      kept their order of darkness. The first masks, softened separately, left a
      pale seam along the crest; overlapped, they left a dark rim.
    - *Explanation:* where two soft ramps don't sum to 1, the sum of glaze
      depths dips or bumps along the boundary. Complementary masks (s and 1 − s)
      fixed it.

12. **A 0.6-coat transparent glaze on the second range, then a brushed glaze
    (`st.glaze(0.9)`) instead.**
    - *Effect:* the poured glaze printed weave streaks as in 8. The brushed one
      speckled on the weave crowns and, after two badger passes, looked paler
      and bluish, like snow.
    - *Explanation, uncertain:* the soft filbert's thin medium-rich film touched
      mostly the crowns (light pressure clears only the tops), leaving the
      hollows as they were. The badger then redistributed a film too thin to
      darken much. I did not find why it came out lighter than the underlayer;
      possibly the aimed masstone plus residual scatter of lead white in the pile
      lightened it. I repainted the range darker (#525369, less mist) instead.

13. **Ridge made to dive into the mist (crest y raised by a smoothstep toward
    its end) rather than faded by an x-ramp on the mask.**
    - *Effect:* a natural spur sinking into fog. The x-ramp version ended in a
      hard vertical cut with a blender swirl and a light fleck at the tip.
    - *Explanation:* a side-faded mask still clips every stroke's crest edge
      where the mask is non-zero, so the silhouette ends abruptly. The blender,
      following the steep slope at the tip, stirred wet paint in a curl.

## Foreground, rocks, grass

14. **Body filbert along the ground's fall; unclipped first.**
    - *Effect:* strokes overshot the crest and one left a sliver like a log
      standing up from the knoll.
    - *Explanation:* unclipped strokes reach past the region's edge by design
      (`hug`). Clipping plus a sable pass along the rim gave a found edge.

15. **Heather: `hatch()` touches at −1.4 rad, lighter and warmer, coverage 0.9,
    into the wet heath.**
    - *Effect:* a lighter olive-rust band under the crest that models the
      ground turning up to the sky; with an OKLab lightness shift of 0.035 it
      became too light and grassy, so I settled on 0.022.
    - *Explanation:* small touches into wet paint mix partly with the dark
      heath. Their net lift is less than the shift, but it is spread over a band,
      so it reads strongly.

16. **Rocks from a `Form` (rough ridged blocks and ellipsoids), light
    `(-0.35, -1)` from behind, ambient 0.25; body filbert from the lit value.**
    - *Effect:* with a lit colour up to #79736c they read as pale pillows and
      a mattress-like plinth, pasted on. Darker (to #5d5852) and angular, they
      read as stone, but still sat *on* the heath.
    - *Explanation:* `shade.value` includes ambient × sky, so upward-facing
      planes of every rock get most of the light, whatever their mass. The
      silhouette is complete, so nothing sinks them.

17. **Sinking: per column, the rock mask's top and bottom found by scanning
    (`f.per_column`), the heath brushed back up to a wavering mid line, then
    grass blades along it.**
    - *Effect:* the rocks sit in the ground; their bottoms are lost in heath.
    - *Explanation:* this is geometry, not physics. The painter decides where
      the ground covers the stone.

18. **Grass as pointed-sable blades (`point` 0.9, 1.1 wide, pressure 0.45 →
    0.05, release 0.7).**
    - *Effect:* fine tapered blades that break the crest against the mist; some
      catch a warmer ochre.
    - *Explanation:* at light pressure only the cone tip touches, and the long
      release draws the mark to a point.

## Firs

19. **Hatch fill clipped to a tiered silhouette mask plus thin branch lines.**
    - *Effect:* stacked flat slabs with hard horizontal edges (the mask's own
      outline).
    - *Explanation:* `clip(true)` reproduces the mask edge exactly; the fill
      strokes can't make a softer edge than the mask.

20. **Wide hatched core with thin (2.0) pointed branch strokes.**
    - *Effect:* a solid black cone with spidery arms, like a dead tree.
    - *Explanation:* at pressure 0.3–0.6 a 2-unit pointed sable makes a mark
      well under a unit wide; the branches had no mass.

21. **Thin spine, then each branch as one stroke of a pointed round 2.4–4.6
    wide (`point` 0.6), pressure 0.55–0.9 → 0.08, release 0.7, sagging then
    lifting; twigs of a 1.2 round hanging from the outer two-thirds.**
    - *Effect:* spruce branches: heavy at the trunk, tapering to a point,
      drooping with a lifted tip, fringed below. Overlapping branches build a
      ragged, asymmetric silhouette with sky between the tiers.
    - *Explanation:* `mark_width` falls with pressure, and the cone narrows as
      the brush lifts, so a single stroke is a wedge. Per-tier and per-side
      random lengths, with 12 % short tiers and 22 % missing branches, break the
      symmetry.

22. **Trunk: a round scaled to the tree (1 + 0.045 × width), pressure 0.8 →
    0.02, `point` 1.**
    - *Effect:* with a fixed 2.6 width the tops were thick bare poles. Scaled
      and fully pointed, the trunk vanishes into the leader.

## Figure and moon

23. **Figure from polygon masks, painted with the sable (`detail`, coverage
    4, `threshold` 0.05).**
    - *Effect:* clean dark silhouette. A flat cap over the head read as a
      chimney; a bare head over rounded shoulders reads as a man.
    - *Explanation:* at 140 px tall the silhouette is everything. The rim
      light (#6c6670, pressure 0.35 → 0.1) is almost invisible at this size.

24. **Crescent: disc minus a disc offset 0.42 r away from the sun, sable
    touches of #f3ecd2, over a 0.18-coat pale halo glaze.**
    - *Effect:* a clean crescent with a soft glow; the thin lit limb faces the
      set sun.

## Finish

25. **`Cracks::aged` defaults.**
    - *Effect:* a strong dark network with islands of ~1 cm dominating the
      image.
    - *Explanation:* hierarchy 1 and patchy 1 let the first cracks open widest.
      Dirt 0.4 and grime fill them dark over light passages.

26. **Cracks at `width_um` 4, `dirt` 0.12, `veil` 0.15, `hierarchy` 0.5,
    `patchy` 0.6, depth 8 µm, cupping 6 µm; varnish 0.22 ± 0.06.**
    - *Effect:* faint hairlines visible only at 1:1 in the lights; the varnish
      barely warms the picture.

## Computation

27. **Timing and reproducibility.**
    - *Timing:* the whole render takes ~220 s. The sky (a broad pass plus two
      swept badger passes over 80 % of the canvas) takes 70 s. Each range takes
      ~18 s, the ground 37 s, the firs, figure, grass and moon under 1 s each.
    - *Iteration:* resuming from a checkpoint cut an iteration to 10–75 s. A crop
      with its own checkpoints was useful for the mist and sky tests.
    - *Reproducibility:* a from-scratch render was byte-identical to the render
      assembled from resumed checkpoints.
