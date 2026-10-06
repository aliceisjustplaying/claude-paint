# Wet-paint failures seen by tonight's lab painters (for r6-wet)

Three painters worked wet on purpose on the current main engine. Evidence
crops (read them with the Read tool; they're JPEGs):
- ~/src/a/claude-paint-r6-lab1/notes/lab/wet/*.jpg and notes/lab/foliage.md
  "Wet-paint misbehavior" (7 items with film thicknesses and clock times),
  notes/lab/rock.md
- ~/src/a/claude-paint-r6-lab2/notes/lab/sky_wet*.jpg, water_wet*.jpg and
  notes/lab/sky.md, notes/lab/water.md
- ~/src/a/claude-paint-r6-lab3/notes/lab/baretree_wet/*.jpg and
  notes/lab/baretree.md
The lab logs are there too (notes/lab/*_B.lua) if you want to replay a
passage on your branch.

Condensed, grouped by likely mechanism:

**Lifting/ploughing through to the ground (reported by all three):**
- A clean blender or badger run along a wet seam exposes orange ground
  specks (lab1 foliage and rock, lab3 baretree: thin open sky wiped down to
  the red ground, 15 µm left).
- A glaze-hand stroke over thin open sky lifted it to the ground (lab3).
- A filbert laid into thin wet sky scraped it to the ground (lab2
  sky_wet1_filbert_plough).
- A light laid into an open dark ploughs a camouflage mottle with ground
  showing, and heaps paint: probes read 2,774–3,477 µm of wet film at one
  spot (lab1 item 3). Paint heaped instead of spread is suspicious for a
  conservation or pickup bug.

**Pushed aside instead of mixed:**
- A dark over open light paint pushes the light to the stroke edges as a
  pale lace ridge; it doesn't mix into a grayed dark (lab1 item 1, lab3
  "dark wood into open sky churns mottled gray").
- A dark stroke through wet light drags the light 40–60 units at nearly
  full strength; long lights across wet darks the same (lab2
  sky_wet2_strip_into_open, water_wet1_unclipped_lights). Real paint drags
  a little, then the brush's own paint wins.
- A thin stroke into wet water leaves two parallel pale lines (tramlines)
  instead of one soft line (lab2 water_wet2_sheen_tramlines): ploughed
  bristle-edge ridges?

**Setting and tacky:**
- A dark over SETTING light is a translucent gray streak or leaves pale
  streaks; over dry paint it covers fully (lab1 item 6, lab2
  sky_wet3_strip_into_setting).
- Sky laid into setting dark: lumpy white cotton puffs, 246–625 µm thick
  (lab1 item 4).
- A thin veil over TACKY paint lies as a flat, hard-edged slab exactly the
  shape of its mask (lab1 item 5); glaze-hand strokes over tacky sky craze
  into a crackle mosaic (lab3); dark on tacky skips with pale flecks (lab3).
- Glaze-hand strokes on DRY sky lift each other, leaving a net of dark
  rims (lab3 tone_try7_glaze_on_dry_rims).

**What worked wet:** lights laid into SETTING dark (15–30 h) came out soft
and leafy (lab1); a bank and its reflection laid as one wet dark with the
water brought up to it (lab2 water B) read well.

Please check these against your audit: which ones you already reproduced
and fixed, which are new, and fix the worst remaining ones (lifting to the
ground and pushing-aside first). Add before/after crops for each one you
fix, and update notes/wet.md and your final report.
