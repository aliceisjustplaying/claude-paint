# Studio notes: sources and receipts (for the owner)

Checked against `main` at 4abe5b4. `crates/paint/src` and `crates/easel/src`
are unchanged since the round 12 branch point (`git diff --stat $(git merge-base
r12-tree1 main) main -- crates/paint/src crates/easel/src` is empty). So r12–r15
reports describe today's engine. r10 and r11 predate the crack and twig fixes,
and r10 also predates the dry-rims fix (`git merge-base --is-ancestor`), so their
reports were only kept where a later report or the code confirms them.

Paths: painter notes are `<branch>:notes/<file>`; `FR` = `notes/round15/pN_final_reply.md`
on main. Code paths are relative to `crates/`. The easel README is
`crates/easel/README.md` (ER); the repo README is `README.md` (RR).

## Facts

| # | Fact (short) | Sources | Receipt |
|---|---|---|---|
| 1 | Vertical threads closer and more even; weft has slubs | supports r15_p1 lim. 1 | `paint/src/surface.rs:39-41` "vertical (warp) and horizontal (weft)"; `:52` "the warp is more even than the weft"; `:475` "warp even, weft with slubs"; `paint/src/style.rs:116` warp 15/cm, weft 13/cm |
| 2 | Warm reddish-brown top ground over knife-laid red earth; early style pale top | r11_winter_astra FRICTION 1; r12_tree1 FRICTION 9 | `style.rs:117-121` grounds `#9a5a36` knife, `#b08457` knife, `#a9785a` brush; `:148-152` early top `#d8c7ab`; ER:325 "thin paint lets it show through … body color hides it" |
| 3 | Light paint at light pressure over wet dark catches only ground tops; dark net like cracks | r13_tree2 Friction 1 | `style.rs:118-119` `Apply::Knife { texture: 0.35 }`; `bristle.rs:1201-1206` contact weighted by surface height, "soft hair bends down into the valleys … stiff hog bristles ride on the peaks" |
| 4 | Plough leaves specks of bare ground; denser pass covers | r14_p1 FRICTION 7 | `bristle.rs:113` push "Fraction of wet paint a moving bristle ploughs aside"; `bristle.rs:1928-1931` test doc "A loaded brush covers a passage it means to cover" |
| 5 | Paint over its own masstone looks the same at any thickness | (supports r10_summer 17) | `paint/src/pigment.rs:10-12` "a coat of paint over paint of its own masstone looks the same at any thickness, so a mark matched to the field it sits in disappears into it" |
| 6 | Thin dark/earth films over pale read lighter, warmer, more saturated than masstone | r14_p3 FRICTION 9; r13_tree3 FRICTION 7 | KM layer compositing `pigment.rs:177` `over(sub, x)` (partial hiding at small thickness). The two reports were made with aimed mixing on; the direction (lighter over pale) follows from KM, "warmer, more saturated" is from the two reports |
| 7 | Thin fringe of a dark stroke over light dries as warm brown halo | r12_tree3 FRICTION 4 | `notes/fixes/dry_rims/README.md` "The tan rim of a pointed dark stroke … not the same cause, not fixed"; `notes/craft_trees.md` Trunks: "A thin dark edge over pale sky dried as a tan or brown halo" **[firm]** |
| 8 | Lead white hides ~0.82, smalts ~0.3; pale lead-white veil scumbles | r14_p2 FRICTION 4 | `paint/src/palette.rs:227-229` `tube("lead white", …, 0.82, …)`, smalt 0.3, pale smalt 0.35 (hiding = contrast ratio, `pigment.rs:29-31`) |
| 9 | Blunt by default; blunt mark 0.45–1.0 × width | r13_tree3 FRICTION 1; r11_winter_fable 16; r11_winter_astra 2 | `bristle.rs:119-129` "0 for every preset"; `:293` `half = width*0.5*(0.45 + 0.55*p)*(1 + splay*(p-0.5))`; ER:309 |
| 10 | Pointed at light pressure = hairline; width follows pressure; lifted stroke reaches end of path | r14_p1 FRICTION 5; r12_tree2 FRICTION 6 | `bristle.rs:295` `rho` from pressure for pointed tools; `:123-126` "pressed lightly only the point touches (a hairline)"; `notes/fixes/twigs/README.md` "lifted strokes paint to the end of their path" (merged a8f1b55) |
| 11 | Brush keeps paint; most laid at start; long wide drag runs dry, starved bands | r12_tree2 FRICTION 1; r10_winter_a 11 | `bristle.rs:108` run "Stroke length over which a load runs down"; `:110` lay "thickness laid at the start of a fully loaded stroke"; ER:317-318 "several strokes from one load run dry naturally"; `notes/craft_trees.md` Trunks **[firm]** |
| 12 | Loaded hair wets hollows, near-empty skims tops; soft hair into valleys, stiff on peaks | (mechanism for 3, 13, 14) | `bristle.rs:1155-1166` "a well-loaded blunt brush wets the shallow hollows … a nearly dry one drags over the peaks only (dry brush, broken color)"; `:1201-1203` |
| 13 | Dry brush breaks only at very low loads; else lines/slabs (reported) | r12_tree1 FRICTION 11 (load 0.04–0.09 needed); r14_p2 FRICTION 10 | `bristle.rs:1164` `wet = smoothstep(0.1, 0.8, vol/full)`. Numbers from the reports; marked reported |
| 14 | Pointed brush under ~¼ load stops wetting hollows, line breaks; dry tip splits | r11_winter_fable 14 (rigger < 0.5 beads) | `bristle.rs:1165` `wick = point * smoothstep(0.02, 0.25, vol/full)`; `:126-129` "a tip run dry loses its point and splits" |
| 15 | A touch is one continuous near-round patch growing with pressure | r12_tree1 FRICTION 6; r14_p3 FRICTION 10 | `bristle.rs:1396-1404` "the paint between them bridges the gaps, so the patch is continuous"; `:1468` `touch_rb` |
| 16 | clip = exact outline weighted by mask; detail and blend clipped by default | r15_p1 lim. 4 / FR p1 4; r10_summer 3 | `bristle.rs:1206` `Clip::Mask(m) => cov * … * m.data[..]`; `style.rs:302` detail `.clip(true)`; `:382` blend `.clip(true)`; `handling.rs:186` `clip: false` default; ER "Edges": "one crisp, even line along the whole contour" |
| 17 | Unclipped strokes overrun edges; wide brush spills out of a band | r12_tree3 FRICTION 3; r14_p2 FRICTION 5; r15_p1 lim. 5; r15_p2 lim. 8; r10_winter_a 13 | `handling.rs:186` `clip: false`; `:1099-1103` unclipped strokes seeded outside brush in |
| 18 | Square ends of unclipped strokes notch edges | r12_tree2 FRICTION 2 | report on current engine |
| 19 | Mask value only seeds strokes (threshold 0.3, detail 0.1); soft mask ≠ soft edge | r10_summer FRICTION 5 | `handling.rs:116-117` "Minimum mask value for a stroke center", `:191` 0.3, `:544`; `style.rs:303` detail 0.1; the mask's value is read nowhere else unclipped (`handling.rs:544, 587, 1089, 1117`) |
| 20 | work fills gaps at coverage ≥ 1.5 and load ≥ 0.25; single strokes don't | r15_p2 lim. 5 / FR p2 5 | `handling.rs:107-111`; `:1169-1176` `FILL_FROM = 1.5`, `FILL_LOAD = 0.25` |
| 21 | Short strokes along a ~1-unit band = dashed line | r15_p1 lim. 9 / FR p1 9 | report on current engine |
| 22 | Parallel strokes breaking together = row of ends, lobed edge | r13_tree3 FRICTION 2 | report on current engine |
| 23 | cut_in with a round on a curve = string of bumps | r14_p1 FRICTION 12 | report on current engine; `cut_in` in `api.rs:819` WORK_KEYS |
| 24 | Loaded starts of broad strokes stay as lighter blotches in a smooth lean passage | r15_p2 lim. 6 / FR p2 7 | `bristle.rs:110` lay at start of a loaded stroke; report on current engine |
| 25 | Brush lifts and mixes open paint; clean brush lifts less as it sets, nothing from set/dry | r15_p2 lim. 9 | `drying.rs:658-659` "A clean brush lifts wet paint, less as it sets, and nothing from set or dry paint"; `bristle.rs:1290-1296` pickup; `wet.rs:178-180` |
| 26 | Light touches into wet dark dissolve; on set paint they stay | r14_p1 FRICTION 11; r14_p3 FRICTION 10 | `bristle.rs:1403-1404` "it lifts some of the wet paint under it, so touches into wet paint blend and dirty the brush" |
| 27 | Pale stroke through wet dark: streaked, dirtier, less opaque | r12_tree3 FRICTION 11 | same mechanism as 25 |
| 28 | Dark stroke into wet pale lifts it, comes out paler | r15_p2 lim. 9 / FR p2 9 | same mechanism as 25 |
| 29 | A passage against a wet neighbor drags it in | r12_tree1 FRICTION 10 | same mechanism as 25 |
| 30 | Crossing strokes in wet paint thin each other at crossings | r15_p2 lim. 5 / FR p2 5 | `wet.rs:178-180` floor "one pass lifts only part of the film"; report |
| 31 | Tacky surface grabs, stick and slip | (mechanism) | `drying.rs:11-13`; `:54` `GRAB`; `:118-120` "Stick and slip … its paint comes off in patches" |
| 32 | Stipple into wet fuses; on dry stays separate | r15_p1 lim. 2 / FR p1 2 | `stipple.rs:15-17` "it deposits and lifts wet paint, so stippling into a wet lay-in fuses softly" |
| 33 | One dip = one patch (24); per-dip mix varies; patchwork on dry | r15_p1 lim. 2 | `stipple.rs:117` `dip_every: 24`; `:64-66` mix_jitter per dip; `:437-443` "a load serves about one patch" |
| 34 | Coverage = touches per point; touch covers π(0.4w)² | (definition) | `stipple.rs:48-50` |
| 35 | Contrast with the under layer decides how a stipple reads | r12_tree2 FRICTION 9; r11_winter_fable 6 | report on current engine (r12) |
| 36 | Coverage < 1: lighter touches (feather) | (supports 35) | `stipple.rs:86-89`, default `feather: 0.6` (`:125`); `api.rs:1193` |
| 37 | Row of touches along thin line = beads; dragged stroke = ridge | r11_winter_fable 11 | `notes/craft_trees.md` "Separate white touches beaded … A continuous ridge along the upper edge … read as snow lying on wood" **[firm]** (restated without the subject) |
| 38 | Blender fuses open paint; can't move set or dry paint | r12_tree1 FRICTION 10 | `drying.rs:658`; `notes/fixes/dry_rims/README.md` "'A blender can't move dry paint' … that's physics, not a bug" |
| 39 | blend clipped by default; unclipped drags wet paint across edge | (tool fact) | `style.rs:362-367` doc; `:382` |
| 40 | Stipple into wet then blend = fused, only strongest traces; after blend = lace | r10_summer FRICTION 1; r15_p2 lim. 7 / FR p2 7 | reports; mechanism `stipple.rs:15-17` |
| 41 | Repeated blending of one lean layer lifts off weave tops, pale lattice | r15_p3 lim. 7 / FR p3 7 | blender `pickup: 0.15` (`style.rs:126`), lift `bristle.rs:1296` |
| 42 | Blending spreads thin spots into soft blotches | r14_p2 FRICTION 12 | report on current engine |
| 43 | glaze() waits for touch-dry, time on the clock | (tool fact) | `easel/src/api.rs:1734-1738` "a glaze goes over dry paint: the painter waits …"; `paint/src/canvas.rs:405` `self.dry()` |
| 44 | Poured glaze levels, drains off tops, pools in hollows | r15_p1 lim. 1 | `canvas.rs:406-407` "a thin fluid film that levels and pools in the hollows"; `surface.rs:358-390` `settle_film` (convex drains, concave gathers) |
| 45 | Deep poured glaze over dark = vertical weave streaks (semi 1.2–1.6, transparent 0.6) | r15_p1 lim. 1 / FR p1 1 | report on current engine; mechanism 44 plus warp 1 |
| 46 | Poured glaze tints each point relative to itself, follows mask, no brush marks; 0.05 lands, < 0.007 fades | r15_p3 lim. 4 / FR p3 4; r12_tree3 FRICTION 12 | `canvas.rs:429-432` `pigment.over(*p, …)` per pixel; ER:475-476 |
| 47 | Brushed glaze by masstone, unclipped; over dry pale: on tops, speckles, lighter (reported) | r15_p1 lim. 3 / FR p1 3 ("I don't know why it lightened") | `style.rs:333-342` (doc and `pub fn glaze` at :339) `.by_masstone()`, no clip. Outcome unexplained; marked reported |
| 48 | Masks are whole-canvas; a function nonzero elsewhere paints there | r10_winter_b FRICTION 2 | ER:265 "Masks are coverage maps of the whole canvas" |
| 49 | Shape masks crisp; roughen re-thresholds at 0.5 | r13_tree2 Friction 7; r15_p3 lim. 6 / FR p3 6 | `mask.rs:72-103` "the old ramp is replaced, not added to … a pixel farther … keeps its side" |
| 50 | ribbon has a round disc at every point incl. ends | r15_p2 lim. 7 / FR p2 6; r13_tree2 Friction 6 | `paint/src/shape.rs:102-135` quads plus `from_circle` at every point; `easel/src/api.rs:1684-1693` |
| 51 | Two softened masks leave a seam; m and -m don't | r15_p1 lim. 6 / FR p1 6 | report; `-m` is invert (ER Masks) |
| 52 | Mask-bounded passage shows a cut edge where its paint differs from the neighbor | r13_tree3 FRICTION 5 | report |
| 53 | Stages; gel ~1.8 h / ~3.6 h; touch-dry ~1 day; bone black, lakes days; thick/fat slower | (tool fact) | `drying.rs:5-15`, `:19-22`, `:36-53` (`TOUCH_DRY_MIN`, `GEL` "~1.8 h … ~3.6 h", `THICK`, `FAT`); tests `:617-626` |
| 54 | wait ages the whole canvas at once | r12_tree1 FRICTION 10 ("the only remedy is c.dry() (the whole canvas)") | `api.rs:1758-1768` `wait` → `Canvas::wait`; `drying.rs:18-22` rate per pixel by pigment, thickness, oil |
| 55 | Hand time: 15-minute slices; earlier passage still open in a sitting | (tool fact) | ER:1310-1311, ER:1327-1328; `easel/src/time.rs:31` `SLICE_MIN = 15.0` |
| 56 | Varnish: yellowish 0.4 coats, absorbs only; warms darks, grays blues, warms pale cools | r14_p3 FRICTION 1; r10_winter_b FRICTION 6 | `api.rs:1791-1809` default `#e6d3a4`, 0.4 coats; `pigment.rs:139-143` "a clear film that only absorbs (yellows) … warms the darks without veiling them" |
| 57 | width_um sets strength; dirt, grime a little; depth, cupping only relief | r10_winter_a 14; r14_p1 FRICTION 4 (post-fix); r15_p1 lim. 10 (post-fix) | `paint/src/crack.rs:61-68` "To quiet a network, narrow it"; `:87-91`; `notes/fixes/cracks/README.md` |
| 58 | Crack reads by contrast; faint light line in darks; invisible in mid tones as dark as its fill | (fix note) | `notes/fixes/cracks/README.md` "The mid-tone crossover"; ER cracks paragraph "in darks a crack is a faint light line" |
| 59 | Graphite: no take on wet paint, sealed by paint, shows through thin paint | r15_p3 lim. 3 / FR p3 3 | ER:325-351 |
| 60 | Different widths = different paintings at small scale; faults show only at full grain | r10_summer FRICTION 15, 16; r10_winter_b FRICTION 9 | `notes/workflow.md:132-135` "work accepts a stroke center by reading the mask … every later mark differ"; RR:43 "a 1000px render is a different painting" |

## Dropped, and why

**Tools the easel doesn't have or the round removes**
- Checkpoint staleness, `// ckpt:` tags, `--stale-ok`, `--resume`, crop checkpoints, checkpoint sizes, stage timings, whole-render times, sky share of render time, env-var switches and pass isolation (r10a 2, 7; r10b 1, 7; r10s 10, 12, 15; r11 fable 7, 10, 15; r11 astra 3, top 5; r12 t1 2, 4, 8; r12 t2 5, 8, 10; r12 t3 5, 6, 10; r13 t1 4, 5; r13 t2 2, 3, 4; r13 t3 3, 4; r14 p1 1, 2, 9; r14 p2 1, 12 (switches part); r14 p3 1 (finish on every render), 3, 5; r15 p1 11; r15 p2 1, 10; r15 p3 1, 2, 3 (checkpoint part), 8): Rust runner only.
- Generators and structure tools: growth `Habit`/`Skeleton`, fir habits, broadleaf `Tree`, `Sward`, `atmos::Ranges`/`Sky`/`Clouds`, `Form` rocks, `World::height`, scene reflections and one water level, `Limb` normals, skeleton previews, `Tree::bounds`, figure helpers (r10a 1, 8; r10b 4, 10, 14; r10s 2, 4, 7, 9, 11, 14; r11 fable 4, 5; r11 astra top 4; r12 t1 1, 5, 6, 7; r12 t2 4, 7; r12 t3 1, 2, 7; r13 t1 1, 8; r13 t2 11, 12; r13 t3 6; r14 p1 3; r14 p2 9; r14 p3 2, 4, 8; r15 p1 7, 8; r15 p2 8): removed and/or subject-bound.
- Aimed mixing and color functions: aim against the wrong underlayer (r10a 12), aimed piles off hue (r10b 13; r11 fable 3; r11 astra top 2), half tone aimed over a dark (r14p2 11), `color_over` across two fields (r15p3 4), `detail` preset's assumed thickness (r15p3 5), wet stipple aimed over a blended field (r14p1 8), `Palette::paint` masstone vs `c.aim` (r10s 13), crescent hiding forced against aim (r11 fable 9): `pal:mix`/aim/color functions are removed. The underlying thin-film facts are kept as 5–8.
- `Mix::Pigment` gradients going gray between complements (r14p3 2): color math, removed.
- Rust API ergonomics: `Handling` has no `.tool()` and isn't `Clone`, `Held.tool` can't change size, `Mask` isn't `Copy`, two `Mark` types, no `Rng` integer range, `Gesture` per-point widths and `swell` knot spacing, NaN from `powf`, `Palette::only` panics and missing smalt (r10a 4, 5, 9; r10b 3, 5, 12; r11 fable 1, 2, 13; r12 t1 3; r12 t2 3; r12 t3 8; r13 t1 2, 3, 7; r13 t2 5; r14 p2 2, 3; r14 p3 7).
- Environment: missing notes, no PIL, `magick montage` fonts (r14p3 6, 11; r15p2 9).

**Fixed since reported**
- Pale "confetti" from lean dry-brush strokes (r10a 3): r10-arm1 predates `fix-dry-rims`; consistent with the rim bug (`notes/fixes/dry_rims/README.md`). Not reported after the fix.
- Limb sections that "tapered to nothing at the joint" (r11 fable 12) and float-off twigs: `fix-twigs` (lifted strokes now reach the end of their path). Fact 10 states today's behavior.
- Crack weight/visibility complaints from r10 (r10a 6): the hierarchy bug is fixed (`fix-cracks`); the `width_um` answer survives as fact 57.

**Not true of today's engine**
- "`Shape::ribbon` self-intersects at sharp turns and leaves holes" (r15p2 2 / FR p2 2): `shape.rs:102-104` builds per-segment quads plus a disc at each point, each filled on its own, "so sharp bends never leave holes" (unchanged since 58f128e). Likely the painter's own path code. The round-cap part is kept as fact 50.

**Painter's own bug, not a tool fact**
- A mask's zero contour drawing a line (r14p2 6): the painter's `smoothstep` formula.
- Dark broad marks vanishing in a dark field (r10s 17): the painter's own note calls it a painter's error; fact 5 states the mechanism.

**Cut to stay within 60 (true but weaker, single reports, or unexplained)**
- Thin blue over the warm ground turning olive (r14p2 12): mechanism not verified; the blotch-spreading half is kept as 42.
- Wide rounds ploughing "ladder" ridges under relief light (r12t3 9; r11 fable 14; notes/motifs.md:172 lists the cause as "appears to be"): cause unconfirmed. Starved bands kept as 11.
- Clipped fill plus edge pass leaving a pale rim (r12t2 2): single report.
- Isolated heavy stipple touches as specks (r10s 16): pre-fix branch; folded into 60.
- Blender swirl and light fleck at a band's pointed end (r15p1 12): single report, mechanism unknown.
- Brushed glaze plus blender drawing dark squiggles (r14p1 10): single report.
- HB pencil barely visible on the reddish ground (r12t1 9): resolution-specific, single report.
- Default cracks too strong on a small canvas at 2400 px (r14p1 4; r15p1 10): resolution-specific and close to a verdict; `width_um` fact kept.

**Uncertain status in round 16's easel**
- "No behind for masks or gestures" (r14p1 6; r13t2 10): the easel has depth options (`visible`, `behind`, `at`, `view` in `api.rs:819-820`) tied to the world and view tools; unclear whether they survive the removal of generators. Dropped.
- `look --scale`, `--dried`, `--probe`: not stated because "no previews" may cover them. Fact 60 mentions only the resolution difference.
- Everything here is checked against `main`. If the round 16 easel changes defaults (`clip`, `threshold`, stipple `dips`, varnish defaults, hand time slices), facts 16, 19, 33, 55 and 56 need rechecking.
