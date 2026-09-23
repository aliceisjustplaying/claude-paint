# l2_green: loop 2 rework (Friedrich, the wanderer over the Saxon plain)

Starting point: l1_green, critic Q3 score 20 (friedrich 4, paint 4, drawing 3, light 5,
detail 4). The integrator fixed the pale halos in the engine, so I skipped them and worked
on the figure/stone contact, the foliage, and the critic's main note that the picture read
as "sunny pastoral, puffy cumulus... not his stilled, emptied light". About 25 minutes.
Four new chunks (26 to 29), all before the varnish chunk (now 30).

## What changed and why

1. **Chunk 26, a stilled sky (the critic's main note on Friedrich and light).** The whole
   sky above the range crests, oak cut out, is overpainted in a `body` pass. It uses
   `color_over` to mix 82% toward a new field `calmsky(x, y)`: slate `#6f7f90` at the top,
   through `#8e9ba5` and `#bdbdb0`, to a warm `#dccfae` at the horizon. A Gaussian glow
   (`exp(-((x-700)/380)^2)`, toward `#e8d6ac`) sits over the village. Long banks come from
   a noise stretched 7x horizontally (darkened by `shift(-0.09, 0.004, -0.012)` at 0.6),
   and a second stretched noise lights their undersides toward the glow. Then a `blend`.
   The 18% of the old sky left under it keeps a ghost of uneven structure, so it doesn't
   read as a flat gradient. The puffy cumulus and the two stray pale discs are gone. A
   second pass with a detail round 2 closes the old sky left in the gap between
   `OAKALL` and the leaves.
2. **Chunk 27, oak crown clump by clump (critique "next": flat foliage blobs).** For each
   `lv.clumps` entry with r > 3.5: three short dark strokes on the lower-right arc (the
   underside), then 2 + 7·lit hooked leaf strokes on the upper-left arc (round 1.3 olive
   `#5d6a37`–`#77804a` where lit > 0.45, else round 1.6 `#34422a`–`#46532f`). Then about
   900 outward hooked strokes along `CROWN:rim(4, 1)`.
3. **Chunk 28, crown mass plus figure and stone contact (critique "worst").** A
   transparent `#1b2419` glaze at 0.5 coats rises across the crown on a diagonal
   smoothstep, so the crown turns as one mass (lit upper left, dark lower right). Chunk
   27 alone had lit every clump evenly. The figure gets a dark crease at his feet and a
   short cast shadow falling right, matching the oak's and the stone's (roughened ellipses,
   transparent `#26301f`, 0.6 coats). The stone gets a contact crease along its foot. Then
   160 small rigger blades go over his boots and the stone's foot.
4. **Chunk 29, evening light on the land (critique notes: "bright even greens").** One
   transparent warm umber glaze (`#4f4a33`, 0.42 coats), graded from 0.3 at the horizon
   to 1 at y 640 with a noisy edge. The noon greens sink into the same light as the sky,
   the far plain stays luminous, and the glow over the spire is now the lightest area in
   the picture.

Not done: the hedges and the field bushes are still stamped (critique "next" #2). The
stone is still loaf-shaped and now the palest thing on the land. The oak's limb structure
("rubbery, no species") is untouched. Some limbs in the crown are now covered by the lit
leaf strokes. A faint pale rim still hugs the crown in places at 1000 px, maybe the
relief pass. The figure is still small and plain at 3200. **At 3200 (`look --crop 300,420,500,520
--scale 3.2`) a pale green glow still surrounds the figure and the stone**, even after
the integrator's engine fix. Check whether it comes from the engine or the paint before
the next session.

## SKETCHBOOK CANDIDATES (add only if the critic's scores improve)

- **Re-keying a finished sky into a stilled Friedrich sky (sky ceiling, §2).** Don't repaint
  from scratch: one `body` pass with `color_over = mix(under, calmsky, 0.82)`, long level
  strokes (30 to 90, coverage 3.4, medium 0.3, `SKYPAL`), then `blend`. `calmsky` is a
  four-stop slate to warm-horizon gradient, plus a Gaussian glow over the focal point,
  plus banks from `noise{period=140, stretch={0, 7}}` darkened with
  `shift(-0.09, 0.004, -0.012)` at 0.6. The 18% of the old sky gives the uneven structure
  that plain gradients lack. Mask it to `1 - smoothstep(crest-8, crest-1, y)` under the
  lowest range crest, clipped, minus the tree. *Pitfall:* the gap between a grown tree
  mask and the leaves keeps the old sky as a halo: close it with a detail pass over
  `(tree:grow(3) - crown) * sky`, clipped off the crown.
- **Unifying the light with one graded transparent glaze.** After the sky changes key,
  `glaze(land, {color="#4f4a33", coats=0.42, pigment="transparent"})` with coverage 0.3
  at the horizon rising to 1 in the foreground. It turns a noon-green landscape into
  evening without touching the drawing.
- **Crown as one turning mass:** after clump-level strokes, a transparent near-black-green
  glaze (0.5 coats) on a diagonal smoothstep across the crown, away from the sun. Clump
  strokes alone light every clump the same way and flatten the whole.
- **Figure contact:** a dark crease at the feet, a short roughened cast shadow on the same
  side as the other shadows (transparent, 0.6 coats, `blur(1)`), then 30 to 40 tiny
  rigger blades over the boots.
