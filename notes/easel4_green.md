# easel4_green: "Summer Morning above the River Valley"

Session log: `paintings/lua/easel4_green.lua`. Renders: `out/easel4_green.png` (1000 px),
`out/easel4_green_full.png` (3200 px).

## The picture and why

A summer morning, full daylight, seen from a grassy height. On the brow at the left
stands a single broad oak in full leaf; to its right a lone erratic boulder lies in the
grass; between them a small dark figure, seen from behind, stands looking out. Below
the brow the ground drops away over the tops of a wood into a wide, flat, green river
valley: a patchwork of meadows and fields with hedgerows and copses, a river winding
out to a village with a church spire, and low blue ranges under the haze at the
horizon. Above, a tall summer sky with a few cumulus.

What it draws on in Friedrich (from what is known of his motifs and method, not from
any one picture):
- **The height and the jump.** A near foreground zone and a far distance with the
  middle ground hidden (here behind the brow and the dark wood): his habit of cutting
  out the transition so the distance opens like a view.
- **The single oak** as a portrait of a tree (Carus's point about specific tree forms,
  not "foliage"), gnarled, standing against the sky.
- **The erratic boulder** (a glacial "Hünenstein" of the kind in his Rügen and
  Pomeranian studies) and the **figure from behind** (Rückenfigur), small, dark, still.
- **The church spire in the distance**: a small particular that carries meaning.
- **Materials**: the 1820 palette with greens (Prussian blue, green earth, chrome
  yellow, ochres), greens mixed from the sky's blues and the foreground's yellows
  (Field's rule quoted in friedrich_materials.md §9), a pencil underdrawing in two
  passes (2H searching, 2B firm), the sky laid first, trees painted on the painted sky,
  grass last as fine upturning strokes (NG p.56).

## How I worked (stage by stage)

1. `canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.4, seed=23}`.
2. A world: horizon y=296, eye 28 m above the valley floor (standing on a height),
   sun from the left behind (azimuth -118, elevation 36).
3. Underdrawing: 2H sketch of horizon, brow, oak, boulder, figure, village, river; a 2B
   restatement of the main contours.
4. Sky from `w:sky` + `w:clouds` (three cumulus, a far bank, thin stratus), broad and
   blended. A day later the lit cumulus tops were scumbled with warm white and the
   bellies glazed a little darker, then blended. (First try: a stipple and a hard-masked
   glaze made speckle and gray slabs; undone.)
5. Three hazed ranges; the valley floor colored per point from world coordinates: a
   rotated Worley grid gives field parcels, each a different green/straw, hazed by
   `w:aerial(Z)`. First try had 140 m parcels (far too big); redone at 55 x 95 m.
6. River as `w:ribbon` in world meters; hedgerows along field edges (a per-pixel mask
   using `w:to_ground`), only beyond 280 m, broken by noise.
7. Village: small hand-built rects and polys scaled with `w:height` (units per meter),
   church with tower and spire, a few trees; hazed.
8. The hill: an open `outline{char="soft"}` along a hand-placed brow, `:below()` as
   mask, painted in a gradient from sunlit yellow-green at the crest to cooler green.
9. Woodland just beyond the brow (tree tops seen from above): a lobed outline with
   irregular heights and gaps, dark hatch, lit clumps. Took three tries (caterpillar;
   too sparse; right).
10. Oak (`tree{habit="oak", seed=3}` chosen from four seeds previewed with `show`):
    thick limbs as `ribbon(l.pts, l.w)` masks in body color with a lit left edge, thin
    limbs as strokes, foliage dark then lit, a day later the trunk restated.
11. Boulder as a `body.ellipsoid` with cuts and roughness, lit by `form{}`; the figure
    as `body_of{}`.
12. Shadows (oak, stone), a darker foreground band, stone cracks and lichen, then a
    `sward` over the hill: 5,282 tufts, 18,783 blades, 258 flowers.

(continued below as the refinement goes on)

## How the easel felt

(see end of file)

## Friction

(see end of file)
