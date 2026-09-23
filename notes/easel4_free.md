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

## HOW THE EASEL FELT

(filled in at the end)

## FRICTION

(filled in at the end)

## Critique

(filled in at the end)
