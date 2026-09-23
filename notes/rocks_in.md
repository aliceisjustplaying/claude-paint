# Rocks grown from a drawn outline (branch `tool-rock`)

## Why

Rocks have been a repeated defect in every critic round (notes/review_scores_loop0..4_raw.md):
- "the boulder is split down the middle into a lit tan half and a flat
  purple half" (loop 4, three times)
- "a loaf with a sliced-off flat face" (loop 3)
- "a stack of rectangular blocks covered in a uniform wormy noise texture
  ... no volume" (loop 0)
- "a soft lumpy blob with a noise-like surface and no planes, fractures or
  bedding" (loop 2)

Painters needed four rounds of CSG per rock (ellipsoid, turn, rough, cut
and cut again). Sketchbook section 6 records the recipes and their
failures (eggs, cork, pillows, masonry).

## What landed

- **`crates/paint/src/rock.rs`** (new, geometry and light only). It adds
  `Rock::grow(outline, corners, cracks, plane_lines, &RockSpec, Light,
  seed)`, which infers a solid behind a drawn outline:
  - **The mass.** The outline is inflated into a pillow from its distance
    field. Granite gets a round profile (2.2). Sandstone and chalk get a
    boxy one (4 and 3.5): steep walls and a flat top. The mass turns away
    at the drawn line.
  - **Planes.** Each plane is tangent to the mass at a site, sunk a little
    and turned a little, so it holds a patch around its site. The patches
    meet in arrises. Rim planes sit on the outline's spans, which are split
    at the drawn corners, at corners found on the line and in long runs,
    so arrises run in from the corners. Face planes are spread over the
    inside, upper ones turned to the sky and lower ones to the ground.
    Sinks are skewed: a few deep cuts make the big planes and many shallow
    ones make facets on them. Planes are joined by a soft minimum, wide for
    weathered granite and narrow for fresh sandstone.
  - **Drawn lines.** A crack or plane line seeds two planes, one on each
    side, facing different ways. A crack also cuts a V joint (walls of
    different pitch) with a crevice at its bottom. Each concave corner of
    the outline grows a joint, since a notch in a silhouette is where a
    crack comes out. Sandstone adds vertical joints.
  - **Bedding (sandstone).** Beds are of uneven thickness, and each
    weathers back toward its base, so it overhangs and shades the bed
    below. Bed joints are returned as lines too. Chalk gets soft vertical
    flutes, and every kind gets lumps and grain.
  - **Light.** A `form::Light` gives `Shade` per point. Cast shadows are
    traced over the rock itself (cracks and overhangs) and onto a ground
    plane it stands on (`ground` foreshortening). The model adds reflected
    light and occlusion from the cavities (two scales plus the cracks).
    `RockSample` gives the core shadow (just past the terminator, where the
    bounce doesn't reach) and the reflected-light family.
  - **Masks:** `mask`, `lit`, `shadow`, `halftone`, `core`, `reflected`,
    `value`, `occlusion`, `cracks`, `up`, `plane(i)`, `arrises`, `contact`,
    `cast` and `snow`. The `snow` mask is the up faces broken by noise,
    plus a cap standing over the top edges.
  - **Fields:** `fall`, `across`, `plane_fall` (one direction per plane),
    `along_crack` and `bedding`. Also `levels`, `bend`, `plane_areas` and
    `lit_share`.
  - **Tests:** 4 unit tests (many planes and a terminator that isn't all
    or nothing; a crack is a dark occluded groove; deterministic, kinds
    differ and sandstone is bedded; a low sun casts the shadow to the
    right, and snow lies on top). There is also 1 ignored test that writes
    a grisaille (`ROCK_LOOK=dir cargo test -p paint --release --lib
    rock::look -- --ignored`), which I used to judge the geometry before
    painting.
- **`crates/easel/src/draw_rocks.rs`** (new): `rock{outline=, cracks=,
  planes=, corners=, kind=, sun=, seed=, ...overrides}` and the rock's
  methods and fields (see the README). `sun` takes `{x, y, z}`, a light
  table as for `form{}` or a `world{}` (its sun). `r:field(...)` returns a
  field that `work{angle=}` reads natively. It has 1 test.
- **Small shared touches:**
  - `api.rs`: the install line, plus a branch in `angle_field` for rock
    fields.
  - `main.rs`: `mod draw_rocks`.
  - `draw_outline.rs`: `outline_corners()` (additive).
  - `easel/src/form.rs`: `light_of` is now `pub(crate)`. Nothing else in
    form.rs changed, and neither did growth.rs or the tree files.
  - `paint/src/lib.rs`: `pub mod rock`.
- **README** (crates/easel/README.md, "Rocks from a drawn outline"): the
  API and two worked examples, an erratic in snow and a sandstone outcrop.
- **Sketchbook** section 6: a recipe at the top, with what decided the
  look.
- **Study:** `paintings/lua/rocks_in.lua`, in 8 chunks. `paint_rock` and
  `seat_rock` in chunk 2 are one painter's way of painting from the masks.
  There are four panels: a granite erratic on the heath; the same outline
  in snow at dusk with a low sun; a sandstone ledge with two drawn joints;
  and a scatter of six stones (granite and one sandstone, broken and firm
  outlines).

## The API a painter uses

```lua
o = outline{pts={{84,322,"c"},{92,282},{112,238,"c"},{152,206},{204,188,"c"},{252,196},{290,214,"c"},
                 {334,226},{372,256,"c"},{398,292},{404,320,"c"},{330,330},{210,334}}, char="broken", seed=4}
r = rock{outline=o, cracks={{{206,192},{222,226},{236,250},{244,292}}}, kind="granite", sun={-1, -0.5, 0.3}, seed=7}
local lo, hi = r:levels(0.03, 0.97)
work(r:mask(), {hand="body", angle=r:field("plane"), clip=r:mask(),
  color=function(x, y) return mix("#34322f", "#cdc6b6", smoothstep(lo, hi, r:value():at(x, y))) end})  -- cache r:value()
for _, s in ipairs(r.seams) do rb:stroke(s.pts, {pressure={0.8, 0.2}, clip=r:mask()}) end
glaze(r:cast(), {color="#3d3a30", coats=0.3}); glaze(r:contact(), {color="#2c2a2a", coats=0.35})
snow = r:snow{amount=0.7, depth=3}      -- or r:up(0.35, 0.7): the faces that turn up
```

## Evidence

- `notes/rocks_in/rocks_in_1000.jpg`: the whole study at 1000 px (about
  85 s on a busy machine).
- `notes/rocks_in/rocks_in_3200_erratic.jpg` and `..._3200_snow.jpg`: the
  crop `--width 3200 --crop 60,170,940,350` (218 s), over both erratics.
- Replay: `easel run paintings/lua/rocks_in.lua [--width 3200 --crop ...]`.
  The PNGs are in out/lua (git-ignored).

Judged against the critics' words:

- **"A pale smooth loaf":** answered for the erratic as a geometry
  question, partly as a painting. The first version was exactly this: 87 %
  lit, one gray dome. The planes were too soft, the sun came too much from
  the front, the whole rock was blended and I drew a symmetric dome. Now,
  at 1000 px, it reads as a stone: a lit shoulder at the left, a lit top
  plane, a crack running off at an angle, arrises from the corners and a
  darker back. Drawing the outline lopsided mattered as much as the tool.
- **"Split into a lit half and a flat purple shadow half along a hard
  vertical seam":** not fully answered. In both erratics the terminator is
  now a diagonal, broken arris with a crack near it, not a vertical seam.
  But at 3200 the plain erratic's shadow side is still one big, fairly
  even plane: the planes are in the geometry (the grisaille shows them),
  yet the paint doesn't separate them in the shadow. In snow, with a
  strong bounce from below, the shadow side is cool gray with reflected
  light and visible planes, far from flat purple. That needs
  `bounce=0.55`, which the README and sketchbook now say.
- **"A loaf with a flat cut face":** no single cut face anywhere, since
  planes are local by construction. An early version let tilted planes run
  below zero, and the clamp made flat gray wedges, the same defect in
  other words. Fixed.
- **"Flat rectangular blocks under a wormy noise texture; stacked
  masonry":** the sandstone ledge now reads as bedded rock. Beds step, each
  overhangs the one below with a shadow line under it, bed tops catch the
  light and the drawn joints cut through. It is busy with pale horizontal
  streaks from the lit-plane pass, and its stroke texture is a little
  striped. The first draft painted the joints as short detail strokes along
  the crack mask: dotted lines, which read like a crate. They are now one
  stroke per seam. The small sandstone stone in the scatter still leans
  toward "basket" (dense bed joints on a small stone): use `bed=` larger on
  small stones.
- **"Stones read as loaves, eggs":** the scatter reads as individual
  stones of different shapes, each with a lit top plane, a shadow side and
  a cast shadow on the sand. The smallest ones are close to plain lumps at
  1000 px, as expected at 20 units across.
- **Snow on stone ("blotches or lichen"):** snow lies on the top faces
  and stands over the edge as a cap, lit warm and shaded blue. But dark
  flecks show inside the snow at both 1000 and 3200 px. Softening the mask
  and painting in masstone at coverage 5 didn't remove them. See below.

## Known issues and next steps

- **Flecks in the snow on the rock.** These are dark dashes at 1000 and
  3200. Not solved: they are neither the mask's edge (softened) nor the
  cover (masstone, coverage 5). Suspects: the style's relief or grain over
  thick white, or the crack seams drawn earlier showing through. Next:
  paint the snow in a separate chunk after `dry()`, and bisect by dropping
  the seam strokes.
- **Shadow sides paint flat.** The geometry has planes there (see the
  grisaille), but a value-only color map compresses the shadow family into
  a narrow range. Next: a per-plane value offset in the painter's recipe,
  or `r:levels` for the shadow family separately (`r:shadow()`-weighted
  quantiles), so the core, the planes and the reflected light spread over
  more of the palette.
- **A dark speckled line along the foot** at 3200: the contact glaze plus
  the grown bottom seams. Lighter `contact` coats, or skip grown seams near
  the foot.
- **Cast shadow length.** Under a raking sun the shadow on the ground
  reaches far (correct for the ground model). Painters must clip it to
  their ground (`seat_rock` takes a panel mask). `ground=` sets the
  foreshortening.
- **Chalk** is only in the grisaille, not in a painting. Its flutes read
  as bark at close range.
- **Cost.** A rock is 0.1–0.5 s to grow and each mask is a full-canvas
  `Mask` (4 bytes per pixel). `paint_rock` builds about 8 masks per rock.
  The scatter of six stones took about 7 s at 1000 px.
- **Painting speed.** The 1000 px study takes 50–105 s, mostly the lit
  and shadow passes and glazes. No new hot spots were found.

## Commits

- `rock: a solid inferred from a drawn outline ...` (the module, tests)
- `easel: rock{} bindings (draw_rocks.rs) ...`
- `rock: planes cut tangent to an inflated mass ...` (no clamped flats)
- `rock: planes may run out through the silhouette ...` (skewed sinks,
  cast shadow from the rock's foot, `levels`)
- `rock: bed joints as seams, deeper undercut beds, calmer snow; rocks_in
  study`
- the final commit: README, sketchbook, these notes and the evidence

`cargo test --workspace` passes (see the final commit). The golden
fingerprint is untouched: nothing existing calls the new code, and
crates/paint/tests weren't changed.
