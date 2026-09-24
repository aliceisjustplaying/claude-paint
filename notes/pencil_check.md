# Pencil check (round 6, branch `r6-pencil`)

Alice saw "an outline" around the rocks in several 3200px crops and asked
whether it was left over from the pencil underdrawing
(`crates/paint/src/graphite.rs`, `notes/pencil.md`).

**Answer: no.** Take the drawing out of a log and the rock edges come out
the same to within 1/255 at 3200px. The outline in the lab rock crops
comes from the painter's recipe. A dark **contact glaze** lays the band
along the foot, and a **blurred cast-shadow glaze** that spills onto the
rock's bottom pixel or two lays the thin line on the silhouette. Neither
one is an engine bug and neither one is pencil. No engine change, so
there's no test. The benchmarks are untouched because no code changed.

Evidence sheet (lossless, 1:1 crops): `notes/pencil_check/alice_sheet.png`
(JPEG preview `alice_sheet.jpg`).

## 1. Which logs draw

Pencil verbs (`pencil(`, `chalk(`, `:sketch`, `:line`, `:rule`, `:hatch`,
`erase`, `fix`) appear only in the loop logs, and only in their first
chunks: `notes/loops/l1_green`, `l1_near`, `l2_green`, `l2_near`,
`l3_green`, `l3_near`, `l4_near` and `l5_near`. None of the round 6 lab
logs (`notes/lab/*.lua`, `notes/lab2/*.lua`) draw, including
`rock_A.lua` and `rock_B.lua`, the source of the committed 3200 rock crops
`notes/lab/rock_A_crop.jpg` and `rock_B_crop.jpg`.

## 2. With and without the drawing (l5_near, l3_green)

The "no pencil" copies comment out the pencil lines in the first 70 lines
and keep the shape tables (`BOULDER`, `STUMP`, `WOODTOP`). No later chunk
uses `drawing_guide()` or `drawing_mask()`, so removing them changes
nothing else:

```sh
perl -pe 'if ($. <= 70) { s/^((h|b|b2|h2) = pencil.*|(h|b|b2|h2):(sketch|line|rule|hatch)\(.*|fix\(\).*)$/-- NOPENCIL $1/ }' \
  notes/loops/l5_near.lua > l5_near_nopencil.lua      # 21 lines out; l3_green: 20
easel run l5_near.lua --width 3200 --crop 170,300,620,520
easel run l3_green.lua --width 3200 --crop 360,430,500,525
```

| render | pixels | max diff (0–255) | pixels > 4 |
|---|---|---|---|
| l5_near, whole, 1000px | 769,000 | 41 | 7 (in the sky along the wood-edge line, not on the rock) |
| l3_green, whole, 1000px | 714,000 | 33 | 11 |
| l5_near boulder, 3200 crop | 1,013,760 | 37 | 2 |
| l3_green boulder, 3200 crop | 136,192 | 8 | 12 |

Line profiles (luminance, median of 5 px) across the l5_near boulder, with
the drawing and without. Its contour was drawn 2H at pressure 0.3, then
3B at 0.6–0.75:

```
left edge, row 422 (x):  57: 214.1 | 214.1   58: 140.3 | 140.3   59: 137.1 | 137.1   (diff 0.0)
top edge, col 768 (y):   66:  43.6 |  43.6   67: 123.8 | 123.8   68: 198.1 | 198.1   (diff 0.0)
```

So the drawing is hidden under these rocks, as the engine means it to be
("body color hides it", `notes/pencil.md`). Every rock passage in these
logs is body color (`coverage` 3–4.5). The pencil shows faintly in only a
few sky pixels over the wood-edge line. It isn't too strong, too dark or
showing through paint that should hide it. So there's nothing to fix and
no sketchbook pitfall to write. If anything, the drawing does almost
nothing to these finished pictures. It's a guide and doesn't survive into
the paint.

What reads as an outline on the l5_near boulder
(`notes/pencil_check/l5_near_boulder_3200.jpg`): the snow cap stops a few
pixels short of the silhouette. That leaves a thin tan rim of rock paint
between the snow and the dark wood, with the same values with and without
the drawing. It's paint, not graphite.

## 3. What the outline in the lab rock crops is (for the glitch stream)

`notes/lab/rock_A.lua` chunk 5 (copied from `seat_rock` in
`paintings/lua/rocks_in.lua:46`):

```lua
glaze(rk:cast():blur(1.2), {color=SHADOWCOL, coats=0.3})     -- (b) the thin line
glaze(rk:contact(), {color="#2c2a2a", coats=0.35})           -- (a) the dark band
```

rock_A crop at 3200 (`--crop 440,380,760,620`). Each variant changes one
line of the log:

| variant | vs as-logged: max diff | where |
|---|---|---|
| without `relief()` | 7 | scattered; not an edge |
| without `varnish{}` | 46 | everywhere (an even tint), not an edge |
| without the contact glaze | 58 | a band along the foot |

(a) **The contact glaze** (`Rock::contact`, `crates/paint/src/rock.rs:1253`)
is by design a dark seam `inside` units deep along the foot, plus a
shadow just below it. At 0.35 coats of `#2c2a2a` it's the broad dark
band along the foot (sheet row 3, left). The sketchbook already warns
against it ("never: a contact glaze ring, a dark line along the foot",
`notes/lab/rock.md`). rock_B, the new way, doesn't glaze it.

(b) **The thin line on the silhouette.** `Rock::cast` is zero on the rock
(`rock.rs:1305`: `*v *= 1.0 - s`), but the recipe's `:blur(1.2)` spreads
it back over the rock's last pixel or two. A dark glaze over the lighter
rock comes out darker than the shaded ground next to it. The line isn't
there after chunk 4, and it appears when the glazes go on. Profile at
column 700, rows 505–508 (luminance, median of 7 px across x 697–703):

```
                         rock (505)  edge (506)  edge (507)  ground (508)
after chunk 4 (no seat)     78.5        77.8        76.9        91.1   <- clean step
no contact glaze            74.0        66.8        57.6        64.3   <- 1–2 px line, ~17 below the rock
  + cast clipped to ground  74.0        72.8        64.8        64.3   <- line gone (edge = ground)
```

`glaze(rk:cast():blur(1.2) * (-rk:mask()), ...)` clears it
(`notes/pencil_check/rockA_foot_edge_as_logged.jpg` compared with
`rockA_foot_edge_cast_clipped.jpg`). The same recipe is in
`paintings/lua/rocks_in.lua` `seat_rock`, so every rock seated that way
has it. The right flank of rock_A and rock_B (row 205 across the
silhouette) has no rim: a clean step from about 76 to 105. Varnish
darkens both sides alike by about 5.

The fix belongs in the recipe, or in a `cast{blur=}` that blurs before it
zeroes the silhouette. That's the glitch stream's call, so I left it alone
here.

## Files

- `notes/pencil_check/alice_sheet.png`: row 1 is l5_near with the drawing,
  without it and the difference ×8. Row 2 is the same for l3_green. Row 3
  is rock_A's foot as logged, without the contact glaze and with the cast
  glaze also kept off the rock.
- `notes/pencil_check/alice_sheet.jpg`, `l5_near_boulder_3200.jpg`,
  `rockA_foot_edge_as_logged.jpg`, `rockA_foot_edge_cast_clipped.jpg`:
  previews made with `scripts/peek`.
