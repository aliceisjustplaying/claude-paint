# Round 8, arm 2: a winter landscape in the manner of Friedrich

Program: `paintings/src/bin/winter_ruin.rs` (`cargo paint winter_ruin`).

## Composition and why

A flat, snowed-over field at the end of a winter afternoon under a low,
overcast sky that clears to a pale cold band at the horizon. Left of
center, in the middle distance, the ruined gable of a Gothic choir stands
alone, one tall lancet window with its tracery open to the sky, the base
lost in ground mist. Right of center a small stand of spruces rises out of
the same mist in a roughly symmetrical group (the tallest in the middle).
In the right foreground a bare, contorted oak grows from a snow bank, its
limbs carrying lines of snow; on the left bank a single glacial boulder
with a snow cap and dry grass. A trodden track curves from the foreground
toward the ruin; on it one small traveler in a dark coat with a stick walks
away from us toward the ruin, leaving footprints. A few crows.

What it draws on (from text only, `notes/research/friedrich_materials.md`
and general knowledge of his motifs): the ruined Gothic church (Eldena, the
Oybin choir) as the image of the old faith; spruces as the evergreen hope
rising out of snow and mist; the dead or bare oak as death/the pagan past;
the Rückenfigur walking into the picture toward the church; snow, mist and a
low horizon with a large, quiet sky; near-symmetry and a strict separation
of foreground and distance without a middle ground to walk through.
Palette as in *Winter Landscape* (NG): lead white snow, smalt/cobalt grays,
a pale mauve sky with a touch of red earth, grass in bone black, ocher and
blue, laid in "fine upturning strokes" over finished snow.

## Working method, stage by stage

Canvas: `Style::friedrich()` (Dresden-type reddish ocher grounds, 1820
palette: lead white, cobalt, smalt, ochers, chrome yellow, vermilion, bone
black), 1.4:1, about 44 × 31 cm. Every paint is mixed from those tubes
(`pal.paint`, or handlings `mixed` from the palette). Stages:

1. **sky**: broad filbert lay-in in level strokes, a shade duller than
   the end, fused with the badger; a coarse stipple into the wet paint;
   then (still wet) about a dozen long lean stratus streaks with a soft
   filbert, gray-violet undersides and a few warm lit ones low; dried; a
   fine, lighter stipple that thickens toward the pale band at the horizon.
2. **far**: the low far rises in body color, a sparse darker stipple for
   the woods on them, the crest fused into the sky with the badger.
3. **ruin**: the gable wall's shape is my own geometry (a pointed
   equilateral lancet, a gable broken down on the right in steps along the
   courses, stepped buttresses), roughened. Laid in with short level body
   strokes, then stone by stone, course by course, a lean touch of a small
   filbert a little lighter or darker; faint course lines; the reveal of
   the window in shade; the fallen tracery (a stump of the mullion, the
   springing of one sub-arch); the shaded thickness of the wall at its
   broken end; light on the gable's slope; snow on the sill, the step tops
   and the buttress offsets (found by looking down the mask for the stone).
4. **spruces**: ten trees by hand, far and pale first: stem lifted to a
   point, then tier by tier each bough as one sagging stroke with the
   needles hatched down from it in short pointed strokes, snow on the
   upper side of the outer boughs.
5. **mist**: a veil of pale lean paint in level strokes with a small soft
   filbert (deep at the feet, thin above, the top edge broken by noise),
   then a stipple, fused level with the badger.
6. **snow**: body color in long near-level strokes, cool and a little
   darker toward us, the far field picking up the glow, rolls and the two
   near banks modeled (lit crests, bluer faces toward us), lightly fused.
7. **track**: two ruts pulled in broken pieces, narrowing as they recede.
8. **boulder**: an erratic with two smaller stones, body strokes turned
   with the form, dark cracks, a few lit planes, a soft shadow under the
   snow's lip, the snow cap laid by hand in stiff lead white, stroke over
   stroke along the crown, and drifted snow at the foot.
9. **oak**: grown by my own recursive hand (angular elbows every knot,
   limbs never hanging below level, twigs turning up in short hooks, a few
   broken ends, a torn-off stub), a hand-drawn thick trunk dividing at a
   third of its height; painted limb by limb, thickest first, several
   parallel pulls for the trunk, a pointed sable then a pointed rigger for
   the thin wood; its cool shadow thrown toward us first; snow banked at
   its foot; lean lighter streaks on the trunk; snow along the upper side of
   the level limbs.
10. **figure**: a traveler from behind in a long dark coat, hat, stick,
    mid-stride; his faint shadow toward us; footprints back down the track.
11. **grass**: dry grass in "fine upturning strokes" over the finished snow
    [NG p.56], clumps on the banks and a few in the field, pointed rigger,
    grays, umbers and dull ochers.
12. **crows**: three in the air, one on the snow.
13. finish: aged varnish, craquelure, relief light (`Finish::aged`).

## FRICTION

1. **A thin band of body color over dark paint came out empty or
   toothed.** The boulder's snow cap as `c.work(&cap_mask, body())` on a
   band 4–13 units high laid almost nothing (first try: a jagged row of
   "teeth" at its lower edge; after reshaping: nothing visible). The planner
   and `threshold`/`clip` don't suit a narrow curved band. Workaround: hand
   strokes along a contour I scanned from the mask, 5 layers from the crown
   down.
2. **A badger over a wet stipple lifts the sky to the red ground.** Fusing
   the sky after the wet stipple and streaks left rust-orange streaks over
   the whole sky (visible at 1000px). A hog filbert drawn into the wet
   lay-in did the same. Workaround: no badger after the stipple; streaks
   with a custom soft brush (`push 0, pickup 0.03, stiffness 0.3`).
   Physically plausible (a pink ground showing through is sourced for *The
   Sea of Ice*), but it's hard to predict and not what I asked for.
3. **One RNG through the whole painting.** Any added draw in an early
   stage re-shapes every later motif (the oak changed completely three
   times while I worked on the sky). Workaround: a separate `Rng` per
   motif (the oak's seed chosen from 5 tries, `OAK=n` env var).
4. **Checkpoints stale on any helper change, and the first stage is the
   slow one.** A helper above `main` stales every stage, so each sky or
   helper edit is a whole rerender (25–55 s at 1000px on the shared
   machine). Workaround: nested `fn`s inside the stage blocks that use them.
5. **Style handlings can't change their brush.** `st.glaze()` uses a
   26-unit filbert; a mist veil whose depth ramps over ~50 units showed
   each stroke's hard top: a rectangular veil across the ruin. There is no
   `.tool(..)` on `Handling`; I rebuilt the handling by hand with a 7-unit
   filbert.
6. **No edge queries on a mask.** To set snow on ledges I scan
   `mask.sample` down a column (`top_at`, a per-column `top_y`). And after
   `Mask::roughen` the edge is no longer where my geometry says, so strokes
   placed from the geometry floated off the wall.
7. **Crops aren't cheap here.** A 3200px crop of the ruin took 24 s, the
   same as a whole 1000px preview: every mask (whole canvas, per-pixel
   closures with noise) is built for the whole 3200px canvas each run.
8. **Aged finish corners.** The corner craquelure makes strong arcs in the
   bottom corners at 1000px; the only control is dropping cracks.
9. Minor: no integer draw on `Rng` (`below`); I used `range` and cast.
