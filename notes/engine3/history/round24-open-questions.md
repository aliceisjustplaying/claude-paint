> HISTORICAL RECORD (2026-10-04): the current user decision is ENGINE 3, no painting launch and no expensive replays ever. Old paintings retain the pinned historical runtime. Use thinner spec draft 7, tooling brief v8 and the current handover. All engine-4, engine-1/2 compatibility-gate and replay-gate proposals below are superseded history, not instructions. Rag appearance is still unvalidated; the small rag/thinner study is required.

# Round 24: open design questions (settle at review, not mid-run)

Rule while agents run: integrity notes (weakened/skipped tests, shrunk fixtures, self-serving checks) are steered at once ("undo it, report honestly"). Design notes land here.

## drying
- Calibration basis: one basis for all pigments (ordinary brushed thin films, artist-guide ranges); Gamblin's 250 um titanium figure only as a check on THICK. Steered 2026-10-03; confirm in report.
- Source ranges: lead white workable 12+ h, tacky ~24 h, touch-dry 1-2 d; titanium 3-5 d; blacks 2-5 d; alizarin 7-14 d. Ordering test: lead white/umber before titanium/blacks, alizarin slowest.

## rag
- Solvent strength: a solvent-damp rag should lift nearly to the ground through `soaked` only; if it can't, what number is acceptable?
- Cloth state out of the painter's reach; spirits dip (added after a steer).
- All parameters are estimates; real calibration = by-eye against reference photos (R. Palesca wipe-out, Downing-White rub-out).

## thinner [SUPERSEDED by the thinner spec draft 4: engine 4, option B, locked target]
- Targets per pigment: lead white stays fairly opaque thinned (scumble); a thinned earth must let the ground show (imprimatura). Raw sienna target failed once; must be live, not skipped.
- Solvent evaporation separate from oil cure; set time a factor on engine drying (minutes to matte, ~1-few h to set).

## all
- New behavior only under the new engine version (engine 3 for round 24; engine 4 for the thinner); old logs replay identically, printed output included.

## at integration (from the failed-chunk count, 213 chunks, 2026-10-03)
- Guide, Lua section (+3 lines): `math.atan2` is gone, use `math.atan(y, x)` (4 failures); `%d` needs an integer, use `%.0f` or `math.floor` (4); lists/tables use `{}`: `{1, 2}`, `{x = 1}`, never `[1, 2]` or `{x: 1}` (3).
- Easel: the f32 conversion error ("error converting Lua function/nil/table to f32", 31 failures = 15%) should name the verb and argument ("stroke: pressure wants a number, got a function"). Error text only; replays unchanged (check printed output of old logs).
- Scripts: /tmp/fails.py, /tmp/classify.py, /tmp/verbs.py (move into the repo if kept).

## after integration: cheaper replay gate [SUPERSEDED by the speed brief v5: release builds, not iter]
- Store per-chunk state digests of 2-3 real logs (one short, one long, one engine-1) once in the repo (e.g. crates/easel/tests/golden/), so a replay check runs only the new binary (half the time).
- One script: replay those logs at 320 px with the iter profile and compare to the golden digests. Agents run the short one; integration runs all.
- Brief future agents to use it instead of building an old binary and replaying twice.

## rag: refine after the round 24 painting (user, 2026-10-03)
- By eye: "broadly right"; not sure the wiped passages read as Inness's veils. Judge on the real painting, then tune lift / stain / spirits.
- Dead source links cited in the rag branch (code comments, notes/rag/README.md, guide?): rpalescafineart.com wipe-out page (404) and susandowningwhiteclasses.com handout (doesn't load). Replace with the Reilly wipe-out page (thinkingaboutpainting.blogspot.com/2007/08/reilly-method-wipe-out.html, works) and the Clark's Inness texts (Autumn in Montclair c.1894: thin red layer wiped with a rag; New Jersey Landscape 1891: wiped areas, trees drawn with the brush end), or drop.

## external review, 2026-10-03 (findings on round-24 3d0ea66 + thinner snapshot)
- Fixing now: #1 sparse-table `#` after a failed chunk (repaints), #6 rag damp never evaporates, #7 drying tests fail without default features -> agent r24-fixes on round-24. #2 thinner getter on engine 2, #3 no solvent evaporation, #4 imprimatura test failing -> steered to the thinner agent.
- DECIDED (user: 100% historical, sent to r24-fixes): #5 raw vs burnt sienna. Engine 3 has raw sienna 2.3 (touch-dry 45 h) vs burnt sienna 1.2 (84.5 h), from modern manufacturer categories; Field/Salter 1869 §§50, 155 says calcining (burnt) improves sienna's drying. Options: burnt >= raw (historical), or keep modern order with a stated reason.
- Note: one pigment table + generic oil/solvent = one chosen formulation, not all oil painting (White, Pilc & Kirby 1998). Scumble = lightly charged brush over dry paint, unthinned (Parkhurst 1897): the lead-white-thinned check isn't a scumble test.

## thinner evaporation (user, 2026-10-03) [SUPERSEDED: option B now, in the thinner spec]
- Round 24: option A. Solvent flashes off at deposit (less pigment per area, thinner leaner film); no new canvas state. No minutes-long wet stage; documented.
- Overnight: option B. Per-pixel solvent that evaporates over minutes of painting time, separate from the oil's cure (Church, The Chemistry of Paints and Painting: solvent escape vs oil hardening). New canvas state, so the save file/checkpoint format must carry it; engine-gated; its own replay tests. Brief it after round 24 is merged.

## performance outlier (2026-10-03)
- paint-studio-3f29bf (Space Bunny) chunk 214 takes 1,569 s alone at 320 px (base build), ~half of the 56 min replay; at 2400 px live far longer. Look at what it does (loop? huge pass?) and whether the easel should report or cap such chunks. Don't use 3f29bf as a golden log.
