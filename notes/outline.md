# Drawn outlines (round 4, stream `outline`)

The painters in amnesia round 3 could paint but not draw: "a sheep is two
ellipses, a thistle is a list of offsets" (notes/amnesia3/easel3_green.md);
"the rock was a CSG problem ... A painter draws it; I compiled it. The
bracken and the crows were drawn in code a few points at a time, the way
you'd write SVG by hand" (notes/amnesia3/easel3_near.md); silhouettes came
out "too smooth" or "cork" (notes/amnesia3/easel3_free.md). This stream adds
a drawing hand: a few rough points become a line a hand drew, and that line
is both the mask's edge and the path a brush follows.

## What changed

- `crates/paint/src/outline.rs` (new): `Outline`, `Character`, `Bone`.
  Geometry only; deterministic by seed; independent of canvas resolution.
  - The line: a centripetal Catmull-Rom curve through the points (no cusps
    or loops), split at corners. Corners are marked by the painter or found
    from the angle (over 60°). A "straight" span between two corners bows a
    little, as a hand's does.
  - The hand: the line moves in and out along its outward normal by a slow
    wobble. By character it also gets straight facets with kinks and chips
    in broken stretches between quiet ones (rock), or rounded lobes in two
    sizes with lognormal widths, taller in groups (foliage, wool, a wood).
    Closed lines take their noise around a circle, so there is no seam.
    Lobes of different heights share the dip where they meet, and a closed
    line's first and last lobes share one, so the line has no step at a
    join or at the seam (review 4, #8). Each lobe is a rounded bulge over
    a baseline running straight from dip to dip.
  - Strokes: the hand lifts at corners and every so often between them. It
    leaves small gaps or overlaps the last stroke, sometimes runs past a
    corner (bending off a little), drifts slightly off the line and restates
    stretches (searching). Pressure varies along the line and is heavier
    where the line faces down (the shadowed underside).
  - Masks: `mask` fills the closed lines, so the edge is exactly the drawn
    line. It is crisp or softened by the character, soft in some places and
    lost in others. Open lines give `below`/`above`, and any line gives
    `band`.
  - `Outline::body(bones, blend, ...)`: a silhouette from a skeleton. Each
    bone is a curve with widths; the spine and limbs join by a smooth
    minimum. The contour comes from marching squares on its own grid in
    canvas units. A limb that starts outside the spine reaches into it.
    Lobes and facets shrink where the body is thin (legs don't turn into
    caterpillars). Holes smaller than (0.08·scale)² are filled; outlined,
    the chink between an arm and a coat read as a buttonhole.
  - `offset(d)`: a parallel outline, the contour of the signed distance on
    the same kind of grid, redrawn by the same hand with less irregularity.
  - `gestures` and `paint`: the strokes as brush gestures, with each
    stroke's pressure along it as a `swell`.
- `crates/easel/src/draw_outline.rs` (new): `outline{}` and `body_of{}` and
  the outline object's methods.
- Shared files, minimal: `crates/paint/src/lib.rs` (+2 lines: module and
  re-export), `crates/easel/src/api.rs` (+1 line: install),
  `crates/easel/src/main.rs` (+1 line: `mod draw_outline`).
- `crates/easel/README.md`: a section "Drawing: outlines and bodies".
- `paintings/lua/outline.lua`: the study.
- Tests: 5 unit tests in outline.rs cover determinism, the line passing
  near the points and corners, the mask edge being the line, a sheep
  skeleton giving one silhouette with sane extents, offsets growing and
  shrinking, and an open line's `below` and `band`. `cargo test -p paint`
  passes (125 passed, 11 ignored) and so does `cargo test -p easel`. The
  golden fingerprint did not change (nothing existing was touched).

## The API a painter uses

```lua
-- seven rough points; "c" marks a corner
rock = outline{{300,600,"c"}, {312,524}, {354,458,"c"}, {424,446}, {490,424,"c"}, {560,504,"c"}, {572,600,"c"},
               char="broken", seed=3}
m = rock:mask()
work(m, {hand="body", color=..., clip=m})          -- clip=m keeps the fill inside the drawn line
rim = m - rock:inset(7):mask()                      -- a strip just inside the edge
b = brush{kind="round", width=4}; b:load("#2e2b26", 0.9)
rock:paint(b, {pressure=1, dip={"#2e2b26", 0.8}, every=4})

-- a tree line: an open line, lobes 24 units wide, the wood below it
tl = outline{{-10,372}, {140,352}, {300,362}, {470,334}, {640,356}, {820,342}, {1010,360},
             open=true, char="soft", lobe=24}
wood = tl:below(440)

-- a sheep from five spine points and four legs; the fleece is the same
-- spine without the head, so head and legs are the silhouette minus it
sheep = body_of{spine={rump, back, shoulder, poll, nose}, widths={8,10,8.5,3.6,2.4},
                limbs={{hip_top, hoof, widths={1.3,0.9}}, ...}, char="soft"}
fleece = body_of{spine={rump, back, shoulder}, widths={8.4,10.4,9}}:mask() * sheep:mask()
dark = sheep:mask() - fleece

-- a bracken pinna: three points, drawn as a small body whose lobes are its pinnules
px, py, tx, ty, nx, ny = rachis:at(t)
pinna = body_of{spine={{px,py}, mid, tip}, widths={w*0.7, w, w*0.25}, char="soft", lobe=w*0.55, edge=0}
```

Characters: `firm` (the default for `outline`), `searching`, `broken` and
`soft` (the default for `body_of`). Options: `amount=` (irregularity
×; 0 = a clean curve, also with an explicit `lobe=`), `lobe=` (units), `edge=` (the mask's soft edge in
units; 0 = crisp), `size=` (the hand's scale), `corners=` (indices, `true`
or `false`), `corner_angle=`, `closed=`/`open=`. Methods: `mask`, `below`,
`above`, `band(w, taper)`, `inset(d)`, `offset(d)`, `paint(brush, {...})`,
`path(i)`, `paths()`, `strokes()`, `at(t)`, `length()`, `corners()`,
`.scale`, `.ramps`.

For the pencil stream: `o:path()` and `o:strokes()` are plain Lua lists
(`{{x, y}, ...}`; strokes are `{pts=, pressure=}`) and `o.ramps` gives the
character's attack and release. In Rust, `Outline::strokes` and
`Outline::lines` are public `Vec`s of points and pressures.

### How big the irregularity is

Every length in a `Character` is a fraction of the hand's scale,
`hand_scale(size) = size^0.7 · 100^0.3`, where `size` is the diagonal of the
points' extent. That equals the size at 100 units and grows more slowly
above it: a small thing is drawn with the fingers, a big one with the arm.
A 300-unit rock (scale 228) gets a firm wobble of about 2 units; a 28-unit sheep (scale 48) gets
lobes of about 3 units.

## Evidence

- `notes/outline/outline_1000.jpg`: the whole study at 1000px.
- `notes/outline/outline_3200_rocks.jpg`, `..._searching.jpg` and
  `..._broken.jpg`: the rocks at 3200px.
- `notes/outline/outline_3200_sheep.jpg`, `..._figure_bracken.jpg` and
  `..._treeline.jpg`.
- Render: `easel run paintings/lua/outline.lua` takes 21–46 s at 1000px
  (depending on machine load) and 103–210 s at 3200px. All the outline
  geometry is a small part of that (chunk 3, three rocks with fills and rims:
  2.2 s).

What I saw, judged as a painter would:

- **Broken rock:** reads as a drawn rock. Quiet runs alternate with chipped
  stretches, and the heavy pointed-brush line thins and swells. The first
  version had even chips all around and looked like torn paper; the slow
  envelope along the line fixed that.
- **Searching rock:** reads as a sketch. Tapered strokes restate each other
  slightly off the line and run past the corners.
- **Firm rock:** the weakest of the three. The spans between marked corners
  bow slightly and the line swells, but the silhouette still looks cut from
  paper: closer to computed than drawn. That is partly the brief (a sure
  contour) and partly that seven points with five corners make a polygon.
- **Tree line:** reads as a far wood of rounded crowns with a soft top edge.
  After the two-size lobes it no longer looks like a row of identical
  scallops. At 1000px it still leans toward "broccoli": every crown is
  round, and nothing is a spire.
- **Sheep:** read as black-faced sheep, two grazing. The woolly edge is
  lobes fitted to the body's thickness, and the legs are clean. The first
  version (rectangles cutting head and legs) looked computed; building the
  fleece from the same skeleton fixed it.
- **Standing figure:** reads as a Friedrich figure from behind in a long
  coat. It is symmetric and a bit stiff (the skeleton is). The cast
  shadow is the contour laid flat and redrawn as a soft outline, then
  glazed. It reads as a low-sun shadow.
- **Bracken:** the best result. Three points per rachis, three per pinna,
  and the soft lobes become pinnules. It reads as dead bracken, not as a
  fishbone of strokes (the version with pinnae as single strokes did look
  like a fishbone).

### Engine API changes (review 4)

- `Character::irregularity` (1 by default) is the amplitude multiplier.
  `Character::amount(k)` multiplies it and no longer rewrites the
  amplitudes, so a lobe or facet set after `amount` is scaled too
  (`amount=0` with `lobe=24` is a clean curve, #6).
  `Character::applied()` returns the amplitudes as drawn. Outputs for
  existing characters are unchanged: the same products are formed in the
  same order.
- `Outline::is_open()`: true if the outline has lines and none is closed.
  An inset that consumes the shape has no lines. It is not open, and its
  `mask` is empty, so `o:mask() - o:inset(d):mask()` keeps the shape
  (#5). The Lua `:mask()` guard should test `is_open()` instead of "has
  no closed line"; that binding is the easel fixer's.

## Known issues

- `firm` with many marked corners still looks geometric (see above).
- Lobes are always rounded: there is no spiky character for conifers, grass
  tufts or thistles.
- `closed` defaults to true when there are three or more points. A
  three-point stem needs `open=true`; without it the stem closes into a
  triangle. `mask()` on an open line errors and points to `below`/`above`,
  but painting an accidentally closed line just draws the loop.
- Without `clip=m`, `work` body strokes overrun the drawn line (the
  existing `hug` behavior), so the fill and the contour disagree. The
  README example uses `clip=m`.
- A soft `edge` uses `Mask::soften`, a full-canvas distance transform
  (about 0.3–0.5 s at 3200px each). For many small shapes (70 pinnae), pass
  `edge=0`.
- `offset` evaluates the distance on a grid of scale/300 units by brute
  force (grid nodes × line segments): fast for motifs, but it could take
  about a second for a whole-canvas outline.
- The stroke plan knows nothing about the brush: a long contour with one
  load runs dry. `dip=` and `every=` handle that.

## What I would do next

- A `spiky` character (tapered points instead of lobes) for spruces, grass
  and thistles.
- `outline_of(mask)`: redraw any mask's edge (a form silhouette, a
  `ridge`) with a hand. The marching squares and the hand are already
  there; it needs only the mask sampled on the contour grid.
- Character by position: `char=function(x, y)` or per-span characters
  (firm here, lost there) for found-and-lost edges.
- An asymmetric skeleton helper for figures (weight on one leg, a turned
  head), so a figure isn't a symmetric post.
