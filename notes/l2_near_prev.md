# l2_near: loop 2 rework of the near winter (erratic, spruce, birch stump)

Critique being answered (total 26): worst = the snow at the boulder's base as five evenly
spaced identical white puffs; next = the wood as a flat black-green mass capped by a comb
of near-identical firs stepping down the slope; next = even horizontal striation over the sky
and snow (probably the tool, not addressed).

## What I changed and why

1. **Drift: puffs → one continuous bank (chunk 11, drift half rewritten; the cap is unchanged).**
   The puffs came from `drifth = 3 + 26*noise(period 70)`. A big amplitude over a short
   period, clipped at 0, makes separate round-topped tongues at a regular spacing, and their
   top color (`#eee6d4` 45%) sat whiter than the contact-shaded snow below them. Now the
   height is an **envelope drawn by hand plus small noise**:
   `env = 4 + 13·G(x,222,48) + 6·G(x,430,60) + 5·G(x,590,16) − 3·smoothstep(500,560,x)·(1−G(x,590,16))`
   (G = `exp(-((x-c)/s)^2)`), then `h = max(0, env + 4·n(period 95) + 2.2·n(period 19) + 0.8·n(period 6))`.
   The snow is piled on the windward left (sun and wind from the left), runs low and ragged
   along the front, nearly vanishes under the shaded flank and rises once more in the corner
   by the small stone. Its edge is roughened at `(1.2, 5)` instead of `(1.6, 7)`. The color
   is **the field's own** (`sample(x, b+12, 3)`): +0.012 L on the lit side, −0.07 L and bluer
   under the shade flank. A crease shade (−0.05 L over the 4 units under its top edge) lets
   the snow tuck under the rock. flat 6, coverage 3.4, medium 0.3, load 0.75, `hug=false`,
   blended twice.
2. **Contact shade into the crease (chunk 14).** Before, `contact_shadow * -footd:grow(2)`
   cut a hole where the drift was, which left a pale gap and then a detached dark stripe
   (L 0.78 at y 505, then 0.56 at y 515), and the stone floated. Now it's
   `vs:contact_shadow{reach=0.14}:blur(1.5) * -stm`, coats 0.22, with nothing subtracted.
   The shade runs up over the foot of the drift into the crease, so the stone sits in it.
3. **The comb broken (chunk 7).** `uneven(18, 14, 650, 0.85, 0.55, 5)` (more clumping). Right
   of x 300 the tips are now `randn(0, 30)` off the line, not ±12. Tree 12 stands 55 units
   above the others, every fifth tree from k 3 is a young one 25–60 lower, `halfw` is
   `rand(0.18, 0.32)`×height, and droop, lean and thickness vary. Every fourth tree right
   of 300 has a second spire grown in against it. The wood's edge now reads as clumps and
   gaps, not a sawtooth.
4. **The wood opened (new chunk 21, before the varnish).** At the wood's foot, a
   horizontal band of **dim shaded snow floor** (horizontal noise, period 38, `stretch={0.03, 4}`;
   y ≈ 313–354; colors from `#323b38` to `#6b706f`, going lighter toward the floor), with
   44 trunks (round 3.2, near-black, pressure 0.5→1, tops at y 250–315) standing in it and
   2–4 low drooping boughs per trunk (round 1.4, pressure 0.9→0.05) hanging into the gap.
   Behind it, a thin glaze veil in **vertical columns** (noise period 34, `stretch={π/2, 4}`,
   `mix(under, "#46524f", 0.35)`, y 120–320) as air between the back trees.
   First try failed: vertical noise columns of pale gray (`#58615f`–`#7a7f80`) read as
   white ghosts or smoke in the wood. The gaps have to be horizontal and at the floor, and
   darker than you think.

Renders: `out/l2_near.png` (1000) and `out/l2_near_full.png` (3200).

## SKETCHBOOK CANDIDATES (only if the critic's scores improve)

- **Snow banked against a stone: a hand-drawn envelope, not a noise ribbon** (replaces the
  loop 1 drift entry in §7, whose ceiling was "evenly spaced puffs"). Heap the drift where
  the wind would put it: a sum of 2–4 Gaussians (a big one on the windward end, 13 units;
  smaller ones, 5–6 units) on a base of 4 units, a negative term under the shaded flank,
  then small noise (4 units at period 95, 2 at period 19, 1 at period 6) and `max(0, ·)`.
  Color it from the field just below the foot (not a white top color), +0.012 L lit and
  −0.07 L cool in shade, with a −0.05 L crease under its top edge. *Pitfall:* big noise
  over a short period, clipped at zero (`3 + 26·n(period 70)`), gives evenly spaced round
  puffs, because clipped noise peaks read as separate objects.
- **Contact shade over the drift, not around it.** `contact_shadow{reach=0.14}:blur(1.5)`,
  coats 0.22, with nothing subtracted. Subtracting the drift (`-footd:grow(2)`) leaves a
  lit gap and a detached dark stripe, and the stone floats. (This replaces loop 1's
  "subtract drifts from contact_shadow" pitfall.)
- **Opening a black wood at its foot.** Paint a horizontal band of dim floor snow under
  the lowest boughs (horizontal stretched noise, lighter toward the ground, `#323b38` to
  `#6b706f`), then trunks (round 3.2, near-black) and drooping low boughs (round 1.4) over
  it, and a thin vertical-column glaze veil higher up for air. *Pitfall:* vertical pale
  patches in the mass read as ghosts or smoke, not gaps.
- **Breaking a wood-edge comb:** tip jitter of ±30 (not ±12) off the drawn line, one tree
  well above the line, a few young ones well below it, `halfw` factors from 0.18 to 0.32,
  and occasional twin spires 9–16 units apart.

## Still weak (checked at 3200, out/l2_near_full.png)

- The drift is one continuous bank now, but along the front (x 300–560) it's still a fairly
  even pale lip about 4–8 units high over a lilac shade band. The envelope needs a lower
  base (1–2, not 4) there, so the rock meets the snow directly in places.
- The wood's trunks taper to spikes at the top (round brush pressure 0.5→1 read upward). They
  should end under the boughs instead, or start at full pressure.
- The floor band in the wood is fairly even along its length at 1000 px and a little
  foggy. It needs more breaks where boughs come down to the snow.
- The trunks come out warm brown-black, not cool gray-black.
- The cap on the boulder still shows the pitted grain as white lace (the loop 1 pitfall).
- The striation over the sky and snow wasn't touched; it's probably the ground texture or
  the tool.
