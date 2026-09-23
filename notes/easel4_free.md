# easel4_free: "Evening on the Heath: Dead Oak and Hünengrab"

Session log: `paintings/lua/easel4_free.lua`. Renders: `out/easel4_free.png` (1000 px),
`out/easel4_free_full.png` (3200 px).

## The picture and why

Late autumn, after sunset, on a flat heath near Greifswald. A low burial mound
rises across the middle of the picture. On it stand a dead oak with two great
splayed limbs and a dolmen (Hünengrab): a heavy capstone on three uprights, a
fallen stone beside it. The afterglow shows through the chamber under the
capstone. A small wanderer in a long coat and low cap, staff in hand, stands on
the crest left of the oak, seen from behind against the glow. A waxing
crescent hangs high in the cooling sky, two ravens fly home, and the towers of
Greifswald stand small and violet on the far horizon. A sandy track winds out
of the dark foreground up to the stones, past a few granite erratics.

What it draws on (from what is known of his motifs and habits, not any one
picture):
- Hünengräber and dead oaks as emblems of death and the pagan past, and the
  Greifswald skyline as the Christian home on the horizon.
- The Rückenfigur: a small figure seen from behind, standing between us and
  the light.
- A low horizon under a huge twilight sky, with the land nearly in silhouette.
  The sky runs from deep blue at the top through a pale greenish zone to yellow
  and a low rose band, crossed by thin, level cloud bars.
- A crescent moon lit on the side facing the sun below the horizon.
- The method in `notes/research/friedrich_materials.md`: pencil drawing on a
  warm, colored ground (the style's red-ocher ground stays visible under the
  paint in places, as in *Two Men Contemplating the Moon* [KÖR p.284]); a thin,
  blended sky, then stippled for smooth gradations [CATS p.127; NG p.56];
  short hatched strokes for heather and juniper [NG pp.49–50]; the fine
  particulars (twigs, grass flicks, figure, ravens) last [NG p.56; CATS p.130];
  and Friedrich's advice to Carus to glaze a twilight picture darker toward the
  edges [MET PDF p.35], used here as a gentle glaze over the ground and the far
  corners.
- The sky palette is limited to lead white, pale smalt, cobalt blue, the
  ochres, chrome yellow, vermilion and umber (the post-1820 palette); the
  land adds bone black.

## How I worked, stage by stage

1. **Canvas** (`friedrich`, aspect 1.45). The ground came out as a warm red-brown.
   I kept it as the mid-tone for a twilight picture.
2. **Drawing.** I placed the mound, oak, dolmen, moon, figure and town as
   overlays first (`try` + `show`). The first oak (seed 8) was tall and spindly.
   I laid five seeds side by side as overlays and chose seed 11 for its two
   splayed limbs and broken top. The first dolmen looked like a little table.
   I redrew it with `outline{char="broken"}` and replaced chunk 2 with
   `easel edit 2`. Then came a 2H pass for the horizon (ruled), the mound and
   the path, and an HB pass for the oak's limbs to order 2, the stones and the
   town.
3. **Sky.** I wrote my own gradient: I chose it over `w:sky{}` because the
   physical afterglow was too even from side to side. The glow is centered at
   x≈610, right of the oak. A broad pass and a blend left the red ground in
   blotches, so I stippled over it wet into wet and blended again. The first
   cloud bars were ruled and parallel, so I undid them. The second try uses
   ribbons that swell and taper, roughened, with the lit rim only on the
   underside (a mask of "cloud here but not just below").
4. **Ground.** The first pass overshot the mound line by about 10 units and
   left a comb of stroke ends against the sky; I redid it with `clip=`.
   The first colors went by canvas y, so the near mound took the far violet
   haze. I redid the colors with a "bump" term so the mound counts as near.
5. **Oak.** Limbs wider than 2.6 units became ribbons painted along the limb.
   The foot flares into the mound, and thin limbs are pressured strokes.
   The tree generator gave only 65 limbs, too sparse for a Friedrich oak, so I
   wrote a small zigzag branch function: segments with random elbows, forking
   into hooked twigs at the ends. The first version was hairline-thin and
   invisible, so I redid it thicker (1155 branches and twigs).
6. **Dolmen.** The first version was mushroom boulders with a light rim
   everywhere, like frosting, and black boxes in the chamber. I redid it as
   angular slabs. Only the capstone's top plane takes the zenith light. The
   chamber is left open, so the glow shows through under the capstone.
7. **Town and moon.** The town silhouette is the drawn polyline filled. The
   crescent is a disc minus an offset disc.
8. **Path.** The first try read as a railway (narrow, straight, light, even ruts),
   so I undid it. The second was the right S-curve but too light, so I undid
   it too. The third is darker.
9. **Heath and particulars.** The heath is hatched heather, short far away and
   longer near. Then came three granite erratics. The first version was
   invisible: dark on dark with a dotted rim. The redo has a lit top plane
   grading to a dark foot. Then junipers, a sward of dry grass with pale tips
   on the nearest tufts, and fine blades along the mound's crest to break its
   edge.
10. **Figure and ravens.** First I put him on the path by the dolmen. At 3200
    px he was a bottle shape below the mound line, lost against the stones. I
    moved him to the crest, where his head stands in the glow. He is a
    `body_of` with a spine from hem to head, legs, an arm to the staff and a
    cap brim.
11. **Finishing.** I stippled a thin veil over the upper sky to cover dark
    smudges, masked away from anything dark so the twigs survive. Then came
    the dark glaze over the ground and the far corners. The first one was a
    photographic vignette with a visible oval, so I undid it and redid it
    gentler. Varnish and relief last.
12. **Refining the foreground** (after the first full render). The foreground
    was a monotone dark field. The sward's blades were dark on dark and
    invisible, so I dropped the varnish chunk (`easel edit 18 --drop`) to
    work under it. The first try at particulars failed: heather clumps came
    out as flat-bottomed pink pills and the lit grass tips as bright white V
    marks spread evenly. The second try dulled the tips and clustered the
    tufts around eight centers, but the clumps were still pills, so I cut them.
    What stayed: 58 tufts of dry grass (about one blade in seven faintly lit),
    pebbles along the track and a fallen, weathered branch. Varnish again.

Time: about 90 minutes from the first command to the final render; 19 chunks
in the log, about a dozen undone or replaced on the way.

## HOW THE EASEL FELT

The best moments felt like painting. The sky went down, came out blotchy, and I
stippled into it wet and watched it close up. The glaze I laid too heavily
turned into a photo vignette and I took it off. Each chunk is one decision and
you see its result a second later, which is how working at a canvas feels: put
something down, step back, judge, keep or scrape. Undo is the palette knife,
and it's cheap enough that I used it about a dozen times without thinking twice.
`easel edit 2` to redraw the dolmen in the underdrawing after the sky existed
felt like a real painter's second thought.

The rest felt like programming, and more than I'd like. Every mark has a
coordinate. A "lit top plane" is a per-pixel mask expression
(`m:at(x,y) - m:at(x,y-4)`). The oak's twigs are a recursive function with
random elbows, not strokes I drew. The heath is a noise-driven color function.
Compared with writing a program from scratch, the easel is far better. The
physics (wet into wet, the ground showing through, a glaze waiting for the
paint to dry) produces the painterly accidents for you, and the look loop
catches mistakes in seconds. But the decisions are still made in numbers, and
judged in a 1000px JPEG.

**Could I draw?** Partly. Contours I could draw by eye. The dolmen stones,
the rocks, the path and the town skyline came from ten or so points placed
while looking at a grid, and `outline{char="broken"}` gave them a hand's
irregularity. The figure too: `body_of` with a spine and three limbs gave a
believable man in a long coat on the second try, once I'd looked at him at
3200px. The tree I could not draw. I chose among generated skeletons by
overlaying five seeds, and the extra branches are computed. The mound is a
formula. Grass and heather are computed scatters. Anything numerous (twigs,
blades, heather) I computed; anything singular (a stone, a man, a crescent) I
could draw. The worst failures were the computed scatters: the evenly spread
white grass tips and the pill-shaped clumps looked digital straight away.

What was slow: the first `--scale 3.2` look at a new window (33–49 s while
another render ran), and a chunk of 38 small `work` calls on small masks
(35 s, against 0.2–4 s for everything else).

What was confusing: `try --look` shows the canvas after rollback, so a paint
chunk can't be previewed. Also, stroke overshoot at a mask edge is only
stopped by `clip=`, not by the mask itself.

## FRICTION

1. **Strokes overshoot a mask's edge by up to ~10 units** even with the default
   `hug=true`. Painting the ground `below(mound)` left a comb of horizontal
   stroke ends in the sky over the mound. *Workaround:* `clip=ground` on every
   pass whose edge is a silhouette. The clipped edge is then crisp to the pixel
   and looks slightly stair-stepped when zoomed; grass flicks along the crest
   break it.
2. **`try --look` can't preview paint**: the look is taken after the
   rollback, so it shows only overlays. *Workaround:* `do`, look, `undo`.
3. **No mask translate.** Rim light on one side of a shape (a cloud's lit
   underside, a capstone's top plane, a rock's sky-lit top) needs "the shape
   minus itself shifted by (dx, dy)". `m:offset` grows the mask on every side.
   *Workaround:* per-pixel `mask(function(x, y) return m:at(x, y) - m:at(x, y - 4) end)`.
   It works but it's slow and wordy.
4. **`tree{habit="dead_oak"}` is sparse or collapses.** With `years=220` all
   five seeds became a bare pole with a few stubs. At default years one seed
   had 65 limbs and read as a sapling. *Workaround:* I picked seed 11 for its
   silhouette and wrote a zigzag branch-and-twig generator (1155 strokes).
   Some of its twigs curl into loops, which oak twigs don't do.
5. **The look file isn't always there when `look` returns.** Four times,
   reading the printed JPEG path right away gave ENOENT; a second later it
   was there. *Workaround:* `sleep 1–2` after `look`.
6. **Many small masks cost full-canvas time.** 38 heather clumps (an
   ellipse, a roughen, two `work` calls each) took 35 s. *Workaround:* I cut
   them. Stamping small shapes by brush would be cheaper.
7. **Thick limbs need ribbons.** A trunk that tapers from 24 to 5 units
   can't be one stroke. *Workaround:* `ribbon(l.pts, l.w)` per limb, unioned,
   and painted along the limb direction.
8. **Colors by distance without a world.** I drew the mound as a curve, not a
   `world` ground, so aerial perspective had to be faked by canvas y. That put
   the near mound in the far haze. *Workaround:* a hand-written "bump" term.
9. **The pencil barely shows on the red-brown `friedrich` ground at 1000px.**
   It was usable only in crops.
10. **`glaze` jumps the clock by weeks** (28.8 days here) to dry what's under
    it. That's correct, but it's a surprise when all you wanted was a veil.

## Critique

What works: the big design. The dark mound and its three emblems (the dead oak,
the dolmen with the glow through its chamber, the tiny wanderer) sit against a
calm, stippled afterglow, with the town and crescent as quiet counterweights on
the right. The value structure is right for a Friedrich twilight: light sky,
nearly silhouetted land. The oak's crown, with its elbows and twig clusters,
reads as a dead oak at a glance. The cloud bars are level and thin as he
paints them.

What doesn't:
- **The foreground is still too empty and too even.** Friedrich would have
  filled it with particular plants and stones in a clear order. Mine is a dark
  hatched field with a few tufts, a pale stick and two gray domes. The rocks
  are rounded blobs, not granite with planes and cracks.
- **The path is the most digital thing in the picture.** It's a smooth ribbon
  of even width with parallel ruts, still a little too light near the bottom.
- **The oak's twigs** curl into loops in places. The trunk is a smooth tube
  with almost no bark or hollows, and the foot is a blob.
- **The upper sky** still has soft dark smudges that aren't clouds, and there
  are pale scumbled patches at its left and right edges.
- **The mound** is a perfect smooth dome; its edge is too clean at close range.
- **The ravens** are too small to read at 1000px.
- The stones and figure are all right as silhouettes. At 3200px the figure
  is simple: a coat, a cap, a staff.

In short, a convincing arrangement of Friedrich's motifs and light, painted with
too little of his foreground patience.
