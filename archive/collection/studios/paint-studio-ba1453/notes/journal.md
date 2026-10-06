- day 1, 09:00: Painting 1: still life, dark ground-interior, raking light from upper left-behind.
  Canvas 762mm, 4:3, W=1000 H=750, linen 26/22. Ground came out a warm off-white, very smooth
  (the knife at 140+70um levelled the weave completely) — so no weave relief to break strokes over.
  I wanted a darker, thinner ground but canvas{} can only be set once. Fine: this is Tonn's 2026
  timelapse setup, a light ground with opaque paint from the first stroke.
  Plan: chalk/pencil drawing → thin dark imprimatura over the whole surface → opaque wet-in-wet block-in
  → refine, deepen, final accents. Palette: lead white, ochre, TOY, cad yellow, cad red, burnt sienna,
  raw umber, bone black, ultramarine, green earth, alizarin.
- day 1, 17:30: State after ~40 chunks. What works: the dark key (wall 0.25, table 0.18), the bowl's
  silhouette and dark interior, the quinces' light/shadow logic, the low warm palette.
  What doesn't: the bottle is bright cobalt (early ultramarine layers still bleeding through —
  must be covered opaquely with a green-black); the bowl is too pale and flat at ~0.6 (needs to
  come down to ~0.42 and warm); the cloth reads as a pale stone, not linen — needs warm value
  (~0.42), hard fold edges and a lit ridge; the wall and table are flat, no light gradient yet;
  no cast shadows from jug or bowl.

  Technique learned (important, cost me several chunks):
  - hand="body" (filbert 9, 20-60 unit strokes) lays discrete worm-like dashes. Stroke length must
    be much larger than brush width or the passage reads as lumps. Use hand="broad" with
    filbert 16-22 and 60-170 unit strokes, then blend.
  - Every work pass should end with blend() or the strokes stay visible as dashes.
  - clip=true piles paint up at the mask edge -> a saturated bright fringe. Use edge="found" to
    stop it, or cover the base again.
  - Transparent blues (ultramarine, cerulean, TOY) over yellow give greens. Shadows over a warm
    base must be opaque: bone black + burnt sienna + raw umber, blue only in small amounts.
  - My luminance estimates run ~0.15 too high; mix darker than the value you want.
- day 1, 22:48: Damage log and repair plan (chunks ~55-70).
  Three self-inflicted wounds, all the same cause: I laid a passage over a large soft mask and it
  ran across objects in front of it. (1) A wall glaze with hand="glaze" is NOT clipped — it laid
  pale caterpillars over the bottle and the bowl; (2) knocking back shadow blobs with table_dark
  painted over the bowl's front and two quinces; (3) the bowl's lit pass covered the fruit.
  Rule from here: every object's mask must exclude the things in front of it
  (bowlfront = bowlm - bowl_in - q1 - q2), and I never repaint a plain region of wall or table
  without subtracting the objects.
  Also: hand="scumble" makes round dark dabs, not shadows — cast shadows want hand="broad",
  filbert 24, pressure ~0.3, low coverage.

  Next: model the bottle (it's still a flat silhouette), put a light pool on the table from the
  left and let the right fall away, clear the pale smear on the wall above the bowl, lay soft cast
  shadows with broad strokes, then a unifying glaze and the final accents.
- day 3, 23:23: Final state and what I'd do differently (chunk ~157, day 3 evening).

  The painting settles as a low-key still life: a dark green-black bottle at the back left, a warm
  bowl with two quinces, two loose quinces on the table, a folded linen running off the right edge.
  Wall 0.18, table 0.22 with the light falling from the left, bottle 0.10, bowl 0.30-0.45, quinces
  0.55-0.75 the only bright note. Squinting, the value structure holds and the step I was chasing
  is gone.

  Honest: the cloth is the weak passage — it reads as a pale slab, not linen — and the table still
  carries a raised, cobblestone relief from a glaze I laid far too lean. The forms are more graphic
  than I wanted: the quinces are cleanly turned but a little bald, the bowl's wall is soft.

  The easel lessons that cost me the most, in case I paint here again:
  - edge="lost"/"soft" with clip=true defeats the clipping: strokes run past the mask and paint the
    things standing in front. Use edge="found" for anything with an object behind it, and subtract
    objects from every background mask.
  - Medium above ~0.2 makes a pile lean and dry-brushing. My unifying glaze at medium 0.35 over
    everywhere() laid a reptile-skin crust over the whole painting that no medium-thickness coat
    could hide — only heavy opaque coats (coverage 3-4.5, pressure 0.65-0.8) displaced it, and even
    then the relief shows through as cobblestones.
  - Blurred masks under work() produce a visible cloud at the 0.3 contour. Gradients need either
    crisp masks with edge="soft" or many narrow bands, not one big blur.
  - blend() over a large mask smears object edges that touch it; keep blend masks well clear.
  - My value estimates from pile arithmetic ran about 0.15 light every time. Mix darker than you
    think and check by looking in "value".
- day 7, 05:24: day 7, ~01:00: Second session on the same picture. Big decisions and hard-won lessons.

  MAJOR CHANGE 1 — the cloth is gone. It read as a pale slab with a bright bone-like bar and I
  could not make it read as linen (three attempts: knock-back, three flat planes, then shadow).
  It was wrecking the right half of the canvas. Painted it back into the table and let the right
  side fall into quiet shadow. The picture is stronger without it: bottle / bowl of quinces /
  two loose quinces, everything to the right going dark.

  MAJOR CHANGE 2 — the fruit were rebuilt from scratch. They had gone bald and faceted with
  hard yellow caps. I knocked all four back to a dark olive mass, then built the light in four
  soft steps (outer zone -> highlight) with a blend between steps. They now read as smooth,
  lit-from-upper-left spheres in the bowl and on the table. The bowl is now a dark glazed vessel
  rather than a pale lens.

  THE TECHNIQUE LESSONS (cost me most of this session):
    - A pile renders MUCH lighter than its arithmetic value, especially at coverage 3-4 with thick
      paint. Mix roughly twice as dark as you think. I calibrated with four test swatches on bare
      table and should have done that first.
    - blend() drags paint across a mask and pulls the colour of anything it touches out into the
      surrounding paint. Blending a table mask that butted against a quince turned the fruit into
      a ghost smear. Rule: keep blend masks >=25 units clear of every object. No blending near
      anything.
    - edge="soft"/"lost" paints PAST the mask edge even with clip=true, leaving a pale halo around
      whatever you masked. Only edge="found" gives a real boundary. To get a soft transition,
      soften() the mask itself, not the edge.
    - hand="glaze" lays 120-300 unit strokes with two directions; on an object 40 units across it
      combs the surface. Glazes are only good on big empty areas (they worked well on the right
      -hand side).
    - hand="body" at filbert 8 lays worms, as I found on day 1. Still true.
    - The canvas was open for 3 days and every new stroke just MIXED into the wet paint instead
      of covering it - nothing I laid down showed. Wait until drying() says setting/tacky before
      you try to knock a passage back. I lost several chunks before I worked that out.
    - bowl_in includes the part of the quinces that stands in it. Painting bowl_in buries the fruit.
      The interior must be bowl_in - q1 - q2 (the day-1 rule again; I broke it).
    - blend() over a rect covering the whole foreground flattened it to a pale smear. On big areas,
      glaze and blend fight each other; pick one.
- day 8, 16:41: day 9, ~08:00: Finished (or as finished as it gets). Let the paint dry a long while between
  passes and rebuilt the picture properly.

  WHAT THE PICTURE IS NOW: a low-key still life. A dark green-black glass bottle at the back left,
  a wide shallow dark bowl with two quinces sitting in it, two more quinces on a warm brown table,
  and the whole right-hand side falling away into shadow. The light comes in from the upper left
  and dies out to the right. Wall about 0.08, table 0.20-0.30, bottle 0.06, bowl 0.10-0.20,
  quinces 0.45-0.72 — the fruit are the only light note in the picture, and they are all that
  is. Squinting, the value structure holds: dark band on top, mid band across, four small bright
  notes in the middle. The fruit read as quinces (downy, olive-gold, a stem dimple on each).

  The cloth is gone. The right side is empty table. I still think the picture is the better for it.

  WHAT WORKED, and is the thing I would write down first:
    - A very broad flat brush lays a clean, streak-free FIELD. bigb = {kind="filbert", width=34}
      with hand="broad", length 90-200, coverage 5 then 4 at two crossing angles, curve={0,0},
      edge="found" - that is how the wall and the table were finally made flat and even. Every
      other way I tried (glaze, body, small brushes, medium brushes) left visible strokes.
    - load_at= takes a function of (x,y) and is the only way to get a smooth gradient. coverage
      does NOT take a function (it errors). Three nested ellipses of a light pile gave three
      visible puddles; one pass with load_at falling off from the light gave a real gradient.
    - For a small object, a light or shadow must be a broad soft gradient over the whole form.
      A narrow ribbon of highlight (width 5-12) plus short strokes is a STRING OF BEADS, every
      time. I tried it three times on the bottle before giving up and lighting the whole left
      side instead.
    - build a fruit in this order: dark mass over the whole mask (coverage 4.5, filbert 7) ->
      warm shadow over the lower right -> four soft steps of light from the lit side inward ->
      blend INSIDE the mask between steps. That is the sequence that produced good fruit every
      time, and it is the only recipe here I would run again unchanged.

  THE MISTAKES I KEPT MAKING (all of them mine, none of them the easel's):
    - blend() on a mask that butts up against an object pulls the object's colour out into the
      surrounding paint. I ghosted the quinces with blend(table mask) three separate times and
      blend(bowlm) twice. bowlm CONTAINS the quinces. Never blend within 25 units of anything,
      and never over a mask that includes an object.
    - Painting a "background" mask without subtracting what stands in front of it. wallm runs
      behind the bowl, so a wall glaze painted over the top of the bowl and the tops of the two
      quinces in it. Same old rule, day 1, and I broke it twice more.
    - A unifying glaze at medium 0.28 over the whole canvas with a filbert 24 at low pressure
      put a mass of pale curls over the entire painting. I lost the objects and had to rebuild
      them. A "harmonising glaze" is not a free operation; if the surfaces are already even,
      do not touch them.
    - When a passage has to be undone, flatten it with the big brush at coverage 5-6 and clip to
      the mask GROWN by 5-7. Clipping to the mask itself leaves its soft rim untouched and the
      old paint survives as a fringe exactly where it is most visible.
    - If a mask does not contain what I think it contains, print its area and bounding box
      before painting on it. `m:band(0.55, 1.0)` remaps the mask's values and gave me a mask
      168000 sq units where I expected 25000.

  The honest verdict: the picture is quiet and low-key and the drawing is sound, but everything
  is too smooth. Tonn's surfaces are built out of visible, broken, varied strokes and mine are
  blended into soft gradients almost everywhere. The fruit are the only passage with any brush
  character. If I had another day I would stop modelling and start painting — leave the strokes
  alone — especially on the table and the bowl.
- day 59, 05:37: day 59, 06:00: Rebuilt the whole picture over nine days of painting time. What it is now: a low-key
  still life on a warm brown ground — a black-green glass bottle at the back left, a wide dark glazed
  bowl with two quinces sitting in it, two more quinces on the table, the light coming in from the
  upper left and dying away to the right. Wall ~0.17 flat warm brown, table 0.30 (lit front left) to
  0.12 (right), bottle 0.08 as a silhouette with one grey reflection down its left, bowl 0.12-0.22
  with a lit rim on its front left, quinces 0.45-0.70 and they are the only light in the picture.

  THE PICTURE, HONESTLY: the drawing and the value structure hold up squinting — dark band on top,
  mid table across, four small bright notes in the middle — and the objects sit on the table with real
  cast shadows. But it is soft everywhere: almost nothing has a found edge except the fruit, the
  brushwork is mostly blended out, and the whole thing sits in one narrow band of warm brown with the
  blue-black bottle as the only colour accident. It reads as a quiet, slightly over-resolved study,
  not as a boldly brushed Tonn. My day-9 note said "if I had another day I would stop modelling and
  start painting"; on this session I spent the day modelling again and never got there.

  THE BIG TECHNICAL FINDING, and it cost me about forty chunks: there is NO reliable way to make a
  soft gradient with `work` at this easel. I tried, in order —
    - `load_at` a positional function: with long strokes it lays STREAKS (each stroke starts loaded
      and runs dry, so a stroke started in the light leaves a bright ribbon), with short strokes it
      lays BLOTCHES. Low load values give dry-brush with hard edges.
    - `clip=` a soft gradient mask: bristles stop weighted by the mask value, which is patchy, not a
      gradient — visible as blobs at every scale.
    - `stipple` with `coverage` as a function: smooth ONCE (it was the best fruit I made all session),
      speckled the next four times, at any width, any cluster, any coverage.
    - `hug=false` and `edge="loose"` with a soft mask: mottled.
    - nested soft ellipses: the passes stop at the 0.3 contour, so you get a puddle with a hard rim.
  THE ONE COMBINATION THAT WORKS: work() with `load_at` (blotchy as it goes down) and then `blend(m)`
  inside the object mask. The blender fuses the blotches into a genuinely smooth modelled form. That
  is the only recipe for a small modelled object here, and I used it on the fruit and the bowl.
  And the opposite rule: for a FIELD, the only clean recipe is `hand="broad"` with a very flat
  filbert (42-62), one pile, constant coverage, `edge="found", clip=true` — an even field, no streaks.

  Other rules this session:
    - Paint onto SET paint or not at all. Two passes in one chunk on a wet passage came out mottled
      every time (the fruit went camo four separate times this way). One pass, then wait 18-22 h.
    - A "wash" of a transparent-ish pile over a dry passage dry-brushes and shows the underlayer;
      my cool tint on the wall turned it into torn paper. To change a whole field, flatten it with an
      OPAQUE pile at coverage 6-7 and stop.
    - A pile much darker than the one under it, laid at low coverage, does NOT knock back: it mottles
      (and it dries too transparent). Knock back means a full opaque cover at coverage 4-6.
    - After flattening a mass, the mask's soft rim survives as a pale fringe: grow the mask by 5-7
      before you paint, or flatten the ring outside the silhouette too. (I laid a ring of table colour
      around each quince twice.)
    - `outline{char="searching"}` with a big `lobe` is for a wide dry shape, never for a rim light.
    - `blend(m)` is the only smoother. It flattens, so run it last, and only inside the object mask.
    - Still true from day 1: edge="soft"/"lost" paints past the mask even with clip=true. I did it
      again with the cast shadows and wrecked the bowl and two quinces; the shadows had to be rebuilt.

  THE TRAPS I FELL INTO (all mine): I chased gradients for a whole session instead of accepting even
  fields; I kept re-painting areas that were already good and made them worse (the wall went from a
  clean light pool to an even field and back five times); and every time I "fixed" one passage with a
  new pile I broke its value and had to fix it again. The picture is better than the one I inherited —
  a real bottle, a real bowl, four quinces, a proper key of light and shadows that seat them — but it
  is over-smoothed, and I know exactly which pass to stop doing.
- day 86, 16:59: day 86, ~17:00: Final session on the same picture (days 59-86). Rebuilt the whole thing again, and this time it came together.

  WHAT IT IS NOW: a low-key still life. A dark green-black glass bottle at the back left, a wide dark glazed bowl with a bright lip and two quinces sitting in it, two more quinces on a warm brown table, the light coming in from the upper left and dying away to the right. Wall ~0.25 with a soft painterly grain, table ~0.30 (lit front left) to ~0.20 (right), bottle 0.06-0.12, bowl 0.08-0.14, quinces 0.40-0.70 and they are the only light in the picture. The two quinces in the bowl sit a step darker than the two on the table, so the bowl reads further back.

  WHAT ACTUALLY WORKED, in order of how much it mattered:
    - Flatten a field to a clean even colour FIRST: very flat filbert (40-44), hand="broad", coverage 3 at angle 0.08 then again at 1.48 with ruler=true, curve={0,0}, edge="found", clip=true, and wait(150) between. Every mess I made on the table and the wall came from NOT doing this first.
    - The bowl needed exactly ONE bright note, its lip, and one edge light on its front left. Nothing else. I tried modelling the belly three times and every version was worse than the black.
    - The fruit: dark base (coverage 4.5) -> mid over the whole fruit (2.8) -> light over a single offset ellipse crossing the fruit (2.0) -> dark crescent on the lower right (1.9) -> blend -> dark mottle at 0.9 with a filbert 7 -> deeper gold over that at 1.5 -> calyx and a couple of small highlights. That whole sequence, run without interruption, gives quinces. Running HALF of it gives potatoes.

  THE TECHNICAL LESSONS THAT COST ME THE MOST (day 59's notes were right about all of these, and I broke every one of them again):
    - edge="soft" is not a soft edge, it is a licence to paint as far past the mask as a stroke is long. With 200-unit strokes and a filbert 44 it sprayed the light right across the table and left black flakes. Soft transitions are made with MANY NARROW BANDS with found edges, not with edge="soft".
    - `grow()` after subtracting objects undoes the subtraction. I flattened the wall around a glow with (mask - obj):grow(12) and the 12-unit rim came back over the bottle's whole left side and sliced the tops off the two quinces in the bowl. If you grow a mask, subtract the objects AFTERWARD.
    - A pile renders far lighter and greyer than its arithmetic value. I chased dark shadows for a whole day with browns that rendered the same as the table. The swatch test (three piles side by side on the table, then look in "value") should be the first thing I do with any new pile.
    - A low-coverage pass (< 0.8) over a big area lays WORMS, whatever the hand. On a field it is only safe if the pile is within a hair of the colour under it. On a small object it is never safe.
    - lose() on a dark shape puts black spikes round it, like grass. It is for lost edges on light paint, not for softening a shadow.
    - blend() over a fruit mask is the only thing that turns blotches into a sphere; but blend then needs re-breaking with a mottle, or the fruit comes out bald.
    - Do not alternate. Twice this session I put a veil on the two quinces in the bowl to push them back, got worms, covered the worms with gold, got flat light discs, covered those with mottle, got worms again. Each repair cost more than the original pass would have.

  THE HONEST VERDICT: it is a quiet, low-key picture with sound drawing and a real key - the four quinces carry the light, the bottle and the bowl are dark anchors, and everything sits on the table with a shadow. It is still smoother than Tonn: the wall and the table are grained but even, the shadows are soft to the point of being implied rather than painted, and the mottling on the fruit is a touch close to camouflage. If I had another day the only thing I would do is break the fields harder with the flat brush and let the strokes stand.
