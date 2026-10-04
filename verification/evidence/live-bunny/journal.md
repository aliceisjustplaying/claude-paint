- day 1, 13:35: ## Sitting 1 — day 1, first block-in (14 paint calls)

  Canvas: `canvas{size=400, aspect=1.25, linen={16,16}, seed=7}` → 1000 × 800 units.
  Ground: lead white 3 / raw umber 1.1 / yellow ochre 0.6, 38 µm, knife, texture 0.3.
  It dried **much lighter and warmer than I judged** — everything after had to go down harder than planned. Watch this next time.

  ### Globals to reuse

  **Brushes** — `bBroad` filbert 22 (backgrounds) · `bBody` filbert 9 (masses) · `bSoft` filbert 26 soft (glazes) · `bRound` round 6 point 0.9 (belly highlight) · `bRigger` rigger 2.4 point 1 (edges, free lines — unused so far) · `bBlender` badger 40 (unused so far).

  **Piles** (the palette holds only 16, and I now have ~26 globals, so the early ones may be scraped — just re-mix):
  - wall: `pWall2` (smalt 1.6, raw umber 2.2, lead white 0.8) is the one on the canvas; `pWall2L` lit upper-left; `pWallG` medium 0.35 for killing streaks. `pWall`, `pWallD` were duds (too light).
  - wood: `pWood2` (umber 4, bone black 1.3, vermilion 0.5, ochre 0.3) is the table; `pWoodD` glaze for shadows; `pWood3` the lit back-left band. `pWood`, `pWoodL` read as terracotta — don't reuse.
  - jug: `pJugM2` (cobalt 2.6, smalt 1.6, umber 0.9) the mass · `pJugDk` (Prussian 1.4, cobalt 1.6, umber 0.8, medium 0.1) core shadow · `pJugLt` (cobalt 0.8, pale smalt 1.2, lead white 3) light · `pJugW` warm bounce glaze · `pJugDg` medium 0.35 sink.
  - cloth: `pCloth2` (lead white 3, ochre 1.4, umber 0.35) the mass · `pClothD2` fold shadow · `pClothW` warm fold shadow, better than the cold `pClothD`.
  - orange: `pOrange` mass · `pOrangeL` light · `pOrangeD` core.
  - `pCast` deepest darks, `rCloth` the rag (loaded — `refold()` before trusting it).

  **Geometry kept as globals**: `tableEdge` (fn), `mWall, mWallLit, mTable, mTableLit, mTableDk, mCast`, `mCloth2` (angular fold shape), `f1 f2 f3 fFront` / `l1 l2 l3` (fold shadow + light ribbons), `mJug` = body + handle, `mJugDkM, mJugLtM, mJugLt2, mJugHi, mJugW, mHTop, mHDk, mRim, mRimU, mFoot, mJugCast`, `mOr, mOrCore, mOrLit, mOrFoot`.

  ### Where I stopped
  Masses and the colour key are in: quiet mid-grey wall, dark warm table, blue jug with its shadow right, cream cloth with an orange on it. The jug reads, the light comes from the upper left, cast shadows and contact shadows are placed.

  ### What I know is wrong, for next time
  1. **Everything is streaky.** I kept under-painting: the wall needed `coverage≈2.8` + `fill=true` to close up, and even then the bristle hatching shows. Start solid, glaze for effect — don't model on a thin film.
  2. **The cloth is a bright oval blob** with scribbly folds, not fabric. Needs a redrawn silhouette with real corners, fewer/bigger fold planes, and crisp fold edges.
  3. **The orange is too small and too flat** — no convincing core shadow or reflected light; it needs to be bigger and sit into the cloth.
  4. **The jug's silhouette is pawn-like** (pinched neck, weak belly). Its left side is still too light for its weight. The handle's loop closed up.
  5. **No crisp edge anywhere in the painting.** `bRigger` and `edge="found"` strokes are the next thing to try.
  6. The handle gap, the hazy fringe at the cloth's top-left, and the orange streak along the back of the table still need cleaning.

  ### Easel gotchas that cost me calls
  - `poly{{...}, ...}, true)` is a **syntax error** — put the points in a variable and call `poly(pts, true)`.
  - `work{..., cross=true}` → "boolean to f32" error; `cross` wants a number. Use two passes at different angles instead.
  - `pencil:line` has no `passes` (only `:sketch` does).
  - A chunk that errors changes nothing, including globals — a failed chunk means the masks it defined are still nil next time.
- day 1, 18:14: ## Sitting 2 — day 1, refinement (6 paint calls)

  ### What I set out to do
  Fix the three faults from sitting 1: the jug's haloed contour and pawn silhouette, the cloth as a bright oval with scribbled folds, and the flat orange.

  ### What worked
  - **`form` + `body.ellipsoid` is the answer for anything round.** `f = form{ {body.ellipsoid({630,596,0},{64,60,57}), dist=0.3}, light={from={-1,-0.7}, front=0.5, ambient=0.2} }`, then `f:lit{parts={1}, soft=0.1}` and `f:shadow{parts={1}}` gave the orange an exact terminator and core shadow for free. It ran first time inside `pcall`. **Use this for the jug's belly and for any future round form** — I built that jug's form-shadow by hand and it is much worse.
  - **Scrubbing a halo works**: `halo = (m:grow(26) - mNew):blur(7)`, then repaint it with the *surround's* pile (`pWall2` over `mWall`, `pWood2` over `mTable`). Do it **after** the last light pass, or the next pass puts the halo back.
  - New cloth silhouette `mCloth3` (13 points, corners, `:blur(3)`) with fold ribbons reads as fabric; the orange sitting into it with a contact shadow works.

  ### The mistake that cost me two calls — brushes carry paint between chunks
  I laid the jug's folds and the orange's contacts, then used `hand="broad"` for the cloth. `bBroad` still held blue from the jug and orange from the fruit: its long strokes dragged both across the wall in spikes and smeared the orange flat into the cloth. The jug grew a bottle-brush halo and the orange was destroyed.

  **Rules now:**
  1. `b:wipe(0.9)` at the top of any repair chunk, and `b:reload(p, load)` **before every colour change** — a brush that is loaded with the wrong colour will carry it past `edge=`.
  2. Never use `broad` next to another colour. Long strokes overrun the mask by up to their whole length; only `detail`/`blend` and `clip=true`/`clip=mask` hold.
  3. `clip=true` really does stop the bristles at the mask edge — use it on every pass that must not spill.

  Repair procedure that worked (call 5): wipe all brushes → scrub the wall and wood over a wide band with wall/wood piles → repaint each mass solid (`coverage≈2.6, fill=true, clip=true`) → re-lay the modelling. The buried streaks never came back.

  ### Where the painting stands
  Value structure is right and it reads at a squint: dark warm table, quiet mid wall, blue jug lit from the upper left with a proper core shadow and open handle, cream cloth with three fold planes, orange with real roundness. The jug now has weight.

  ### Still wrong, for sitting 3
  1. **A chalky pale patch on the jug's lower-left belly** — the light pass plus `mJugHi2` highlight is the brightest thing in the picture and sits in the wrong place. It reads as a hole, not a highlight. Knock it back with `pJugM2` glaze and put a small crisp highlight higher on the shoulder instead.
  2. **Blue still feathers onto the wood and the wall** at the jug's lower left, and the left contour is dry-brushed and scratchy. Needs one more halo scrub plus `edge="found"` passes on the contour.
  3. **Still no crisp edge anywhere.** I have `bRigger`, `outline{pts=..., char="firm"}` + `o:paint(brush, {dip={p,0.6}})`, and `edge={found=, soft=, lost=, period=40}` for running crisp/soft round a contour — none used yet. Start with the jug's silhouette and the cloth's front edge.
  4. The jug's neck is still a little long and its rim is flat; the cloth's folds are good but mottled (the `fill` dabs read as cottage cheese up close) — try longer `body` strokes at lower coverage, or the rag, instead of more `fill`.
  5. The wall's texture is busy enough to compete with the jug; consider a final thin glaze to sink it.

  ### New globals
  `mJug2` (the fuller jug) · `mJugDk2, mJugLt3, mJugHi2, mFoot2, mRim2, mRimU2, mJugW2, mJugSh2` · `mCloth3` + `clothPts2`, `crest1, crest2, trough1, trough2, lip` · `mOrNew` · `pClothLt, pOrangeR, pOrangeC, pOrangeH` · `scuff, halo, halo2, halo3` (repair bands).

  Still true from sitting 1: the ground dries lighter and warmer than it looks wet; start solid and glaze for effect; the palette holds 16 piles and I have ~30 globals.
- day 1, 21:04: ## Sitting 3 — day 1, final sitting (5 paint calls)

  ### What I set out to do
  The four faults left from sitting 2: the chalky pale patch on the jug's belly, blue feathering onto wood and cloth at its foot, no crisp edge anywhere, and a wall texture that competes with the jug.

  ### What worked
  - **Tracing a silhouette out of the mask itself.** Looping rows and columns with `mJug2:at(x, y) > 0.5` gives the exact left, right and bottom contour as point lists. No more guessing points by eye. This is the best trick I found today:
    `for y = 276, 576, 12 do for x = 200, 500, 2 do if mJug2:at(x,y) > 0.5 then lx = x end end end`
  - **Edges are a band, not a line.** `outline{pts=..., char="firm", amount=0.15, size=3}` then `work(o:mask(), {hand="detail", tool=bEdge, ...})` with `bEdge = brush{kind="round", width=3.2, point=0.1}` lays a continuous two-unit edge. Intersect with `mJug2:shrink(1)` (or `:grow(2)` on the shadow side) and the edge cannot leak onto the wall. `clip=true` on top of that.
  - `blend(litM, {angle=1.35})` after the light pass knocks the chalky look off a light without losing it.
  - A **less chalky light pile**: `pJugLt2 = cobalt 0.9, pale smalt 1.5, lead white 2.2` (pJugLt's lead white 3 was the chalk). Still too light on the belly's lower left — see below.
  - The form scaffold on an **ellipsoid for the belly** (`{352,480,0},{104,110,76}`) gave a clean terminator, better than my hand masks. Worth building next time around the whole jug.

  ### What failed, and cost me a call
  `outline:o:paint(bRigger, ...)` with `amount=0.7` drew **wire**: a rigger hairline with long overshoots that flew off across the form and onto the wall (one crossed the belly horizontally). It read as pencil, not paint. Fix: `amount` down to 0.15, **and** paint `o:mask()` with `work` rather than stroking the line with a rigger. `o:paint` also rejects a table of pressures — `pressure={0.8,0.6,0.05}` raised "converting Lua table to f32"; use one number and split the polyline into two runs instead.
  A pointed rigger at low pressure on a narrow mask **beads** — dots rather than a line. A blunt round (`point=0.1`) is what an edge wants.

  ### Where it stands
  Reads at a squint and at arm's length: dark warm table, quiet grey wall, blue jug with weight and an open handle, cream cloth with fold planes, orange with real roundness. The jug's chalky hole is gone; it has a modelled belly, a lit shoulder, a small high highlight, and a contour that is found rather than lost. The orange and cloth are unchanged from sitting 2 apart from being cleaned round the edges.

  ### Still wrong, if there is ever a sitting 4
  1. The jug's **lower-left belly is still the palest thing in the picture** and its contour there is beaded. The belly wants a darker mass tone on the left and one confident edge, not a crust.
  2. **The wall's texture is the busiest thing in the painting** — a thin `pWallG` glaze over `mWall` at coverage ≈0.5 would sink it; I ran out of calls before trying it.
  3. The cloth's folds read as soft bands, not sharp fabric; `crest1`/`trough1` want `edge="found"` with a small brush, and the front lip needs one crisp dark line under it.
  4. The neck still reads a little long and the rim flat.

  ### New globals
  `pJugLt2` (the light to use from now on) · `bEdge` (blunt round 3.2, for contours) · `L`, `R`, `B` (traced jug contours, `R` bottom-to-top) · `Rs2` (right contour below the rim) · `eL`, `eR`, `mEdgeL`, `mEdgeR` (contour masks) · `litM4`, `band`, `haloA`.

  Still true from earlier sittings: the ground dries lighter and warmer than it looks wet; start solid and glaze for effect; `b:wipe(0.9)` at the top of every repair chunk and `reload` before every colour change; `o:paint` takes one pressure number.
