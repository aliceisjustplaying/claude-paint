# Round 8, arm 3: a winter landscape at the easel

## Composition and why

Working title: *Winter Morning by a Frozen Pond*.

A low horizon under a tall dawn sky that runs from cold gray-violet at the
top to a pale rose and straw band at the horizon (the pale mauve sky of the
London *Winter Landscape*, smalt with a few red iron-oxide particles, NG
p.56). Across the middle distance lies a frozen pond, its ice a pale mirror
of the sky; beyond it a dark band of firs and, faint in the morning mist,
the spire of a village church (the church as the far goal is a Friedrich
constant). On the left a snow bank rises into the foreground and carries a
dead oak whose crooked limbs cut the sky (the oak-in-snow motif), with a
few young firs painted in short hatched strokes (NG pp.49-50). One small
dark figure walks away from us along a trodden path toward the church:
a Rückenfigur, small enough that the landscape stays the subject. Dry grass
and reed stalks are flicked upward over the finished snow last (NG p.56).

Materials: `friedrich_early` (smalt, lead white, earths, bone black) on a
640 mm canvas, since the smalt skies of c.1811 suit a winter morning.

## Method, stage by stage

Session `pond` at the easel, 1000px, hand time off. Source:
`paintings/lua/pond.lua` (24 chunks).

1. **Ground and drawing.** `friedrich_early`, 640 mm wide, aspect 1.4. A
   2H pass: the horizon ruled, the bank, both pond shores, the wood's
   skyline, the spire as a ruled vertical, the oak's trunk line, the
   figure and the path, all sketched lightly.
2. **Sky.** One broad pass of a gradient I mixed by eye (gray-violet at the
   top, rose, then straw at the horizon, warmer under where the sun will
   rise), from a restricted pile set (lead white, both smalts, red earth,
   ochre, vermilion, umber). My first try with `blend` after it left pale,
   thin, lifted patches like cloudlets. I undid it, laid the paint heavier
   with no blend and fused it with a wet-in-wet stipple.
3. **Snow and ice.** The far flat snow and the bank as body paint, the
   bank's strokes running along its slope, the ice as a broad pass with
   a warm reflected glow under the dawn. The first bank was too dark and
   lavender and read as a distant mountain. I undid it and repainted it
   lighter.
4. **Distance.** Low wooded hills stippled at the horizon (after drying;
   see friction), the fir wood on the right from `fir_wood` (three rows,
   hazed), a small Gothic church and spire cut in with a pointed round,
   then morning mist stippled up over the far shore.
5. **The oak.** `tree_in` oak, winter, `detail=0.3`: the stout wood as body
   paint with a lit flank toward the glow, the middle wood with
   `paint_wood`, and the fine wood stroked by me from `wood_strokes` with
   a pointed round lifting to hairlines. Three tries: see friction. Later,
   snow along the level limbs, an umber glaze deepening the wood against
   the sky, bark furrows, and the foot buried in drifted snow touch by
   touch.
6. **Young firs** on the crest with `fir{}` (spire, young, old), made
   airier than the default (`gap`, lighter body hatch) so they have tiers
   and drooping boughs with sky between. Stems carried down, snow laid on
   the upper faces of boughs.
7. **The path and the walker.** A trodden path winding down to the ice:
   a thin blue glaze with footprints as short strokes. The figure from
   behind (greatcoat, low hat, a stick), cut in with a pointed round, a
   touch of dawn on his right shoulder, his boots sunk in snow.
8. **Particulars.** Faint strata in the sky, drawn by hand as long
   half-dry filbert strokes. Swept clear ice as a soft glaze. Reeds at
   the shore. Dry grass clumps and weed stalks flicked upward over the
   finished snow, larger near. A glaze of blue shade on the lee sides of
   soft drifts. A last stipple veil and a thin smalt glaze darkening
   toward the top (Friedrich's advice to Carus about edges).
9. **Finish.** A day's wait, then varnish, cracks and relief.

## FRICTION

1. **`blend` right after a broad sky pass lifts the paint into pale,
   thin blotches** (22 µm against 50 µm around them) that read as
   cloudlets or ground showing through. Workaround: no blend; a heavier
   first pass (coverage 5), then a wet-in-wet stipple to fuse.
2. **Body strokes over tacky or dry paint make a lacy net of loop
   outlines** instead of a covering film: the far hills over a tacky sky
   and the trodden path over dry snow both came out as a web of stroke
   rims with the old paint showing in the cells. It looks like nothing a
   brush makes. Workaround: stipple or glaze for anything laid over
   earlier paint; `dry()` first.
3. **`tree_in` winter oak reads as wire and claws.** Default
   `paint_wood` bands gave thorny spikes, beaded dotted limbs (the
   1.2–3.5 band) and looping limbs; `kink` made claw-like wedges. A pointed
   rigger at `pressure=0.04` gave dotted chains of beads. Workaround: my
   own strokes from `wood_strokes{max=1.2}` with a pointed round pressed
   by `pressure_for(w)` and lifting to 0, `every=2, load=1` on the middle
   band, another seed. Still smooth and tube-like at 3200: every limb has
   the same even tone.
4. **The trunk flares into a round bulb below its foot** (`t:wood(3.5)`
   paints past the given foot), so a tree planted on a slope shows a
   rounded "bottle" end on the snow. Burying it took four chunks: masked
   `work` made boxes, a stipple patch made a pale oval, touches sampling
   neighbors picked up the sky, and a glaze over `t:wood()` left a gray
   ghost disc over the snow. What finally worked was opaque lumpy snow
   touches along a hand-drawn drift line.
5. **Fir crowns end above their `foot`** and the dense default body
   (hatch at `f.hatch*1.5`, coverage 2.6) makes blobs with a round knob at
   the apex; firs floated above the snow. Workaround: `gap`, `pad`,
   `droop`; a thin hatch; the leader stroked down to the foot by hand.
6. **Masks built from `rect`s leave their straight sides in the paint**
   even after `:blur(4)`: the snow patches at the tree feet came out as
   visible boxes. A painter wouldn't have a rectangle there at all.
7. **Dry-brush with a flat at low pressure lays dashed double lines**
   (ice streaks), ruled and even; higher pressure gave hard parallel
   edges. Workaround: a glaze of a stretched noise mask.
8. **Scattered small marks read as confetti.** Grass tufts placed by noise
   thresholds, even clumped, came out as evenly sprinkled yellow sprigs
   at 1000px. Workaround: 19 hand-placed clumps of different sizes, big
   near, with glazed hollows at their feet.
9. **`work{fill=false}` isn't exposed** though the README talks about
   `Handling::fill`: the error lists the options and `fill` isn't one.
10. **`fir:paint` takes no `pal`**, unlike `work`: the palette restriction
    can't be carried into the motif verbs.
11. **`fir_wood{foot=}` wants points, not a function**, where `below()` and
    others take a function of x.
12. **Probing with `oak:mask()` inside a loop is slow** (15 s for 2,600
    calls: the mask is rebuilt each time). Keep masks in locals.
13. **`glaze` silently dries the whole canvas first** (days on the
    clock). That's right for a glaze, but it means any wet-in-wet work
    planned after it in the same sitting is gone.
14. **Pale specks of ground in body passages** (warm dots in the snow at
    3200) where strokes left pinholes over the ochre ground.
15. **The bank crest never reads** against the flat snow beyond, because
    both come from the same values; there's no tool problem here, just a
    reminder that snow needs a planned value step at every overlap.
