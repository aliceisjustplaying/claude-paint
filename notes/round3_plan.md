# Round 3 plan (2026-09-23 day)

User's direction after amnesia round 2 (notes/amnesia2.md): more entropy
(skies and ranges are "too neat"; the ranges read as waves), more detail
everywhere, especially foregrounds, a pointed-tip brush, a crack revamp,
consistent light (the coast had two light logics), solids grounded (the
floating tor block, the pasted-on boulder). Convergence of Claudes on the
same picture (twilight, crescent moon, figure seen from behind, birds) is an
outcome to observe, not a bug. Big new idea: painting should feel like
painting: a live Lua easel (REPL) whose session log is the replayable
painting, and time (drying) as a first-class verb.

## The user's standing instruction (read after any context compaction)
"Take your time. It's okay if this work takes several auto-compactions.
What counts is a result that feels like you poured real love into it."
Quality over speed: look at every render, judge it like a painter, fix what
reads as digital, and don't merge anything I haven't looked at.

## Integrator notes (for resuming after compaction)
- Scratch dir: ~/tmp/paint-overnight-8ce44b40 (briefs: common.md,
  easel_brief.md, amnesia_brief.md, review_brief_r3.md: neutral wording, the
  first review tripped a cybersecurity content filter on Astra).
- Worktrees: ../claude-paint-<branch>; merge each into main after viewing its
  evidence, run `cargo test -p paint` + build all bins, push.
- Easel must use Lua 5.5 (mlua 0.12 `lua55` + `vendored`); switch at merge if needed.
- Coast #17 (curved drag lays nothing) may need a bristle.rs fix after `tip` merges.
- Reviews: openai-codex/gpt-6-astra, thinking medium, fast off.
- Amnesia worktrees from round 2 (../claude-paint-amnesia-*) hold full-res PNGs; keep.

## Batch 1 (running)
| stream | branch | owns |
|---|---|---|
| easel: live Lua session, replayable log, studio views | `easel` | new crates/easel, paintings/lua |
| pointed-tip brush (round/rigger point, pressure→width, taper, no beads) | `tip` | bristle.rs |
| crack revamp (from the style's ground, resolution-aware, irregular) | `cracks` | crack.rs, run.rs Finish |

## Batch 2 (now)
| stream | branch | owns |
|---|---|---|
| time: open/tacky/touch-dry model, `wait(minutes)`; glaze near-zero cutoff | `drying` | wet.rs, surface.rs, canvas.rs dry/glaze |
| scene: one world, one sun, ground plane + horizon, contact and cast shadows, reflections | `scene` | new scene.rs (uses form.rs) |
| fixes (paint): aim along the stroke, coverage at mask edges, blender clipping, canvas-aware color fields, stipple contrast | `fixes-paint` | handling.rs, stipple.rs, style.rs, palette.rs aim |
| fixes (UX): stage names/--stop/staleness, closure ergonomics, roughen units, Sdf docs, curved-drag bug diagnosis | `fixes-ux` | run.rs stage code, noise.rs, canvas per_column, mask.rs roughen, form.rs docs |

## Batch 3 (after batch 1)
| atmosphere: sky light structure, cloud volumes, noise toolkit (warp etc.), irregular spacing | `atmosphere` | new atmos.rs, noise.rs |
| green: sourced greens for the palette, foliage masses on growth skeletons, meadows | `green` | palette.rs tubes, growth.rs |

## Then
Easel bindings catch-up for batch 2–3 features; adversarial review
(gpt-6-astra, medium, fast off; neutral wording to avoid content filters);
fixes; amnesia round 3 **at the easel in Lua**, varied briefs (one fully
free; others steered off twilight/moon/Rückenfigur, e.g. daylight, summer
green, a close study); stop and evaluate.

## Status log
- Easel uses Lua 5.5 (mlua 0.12 `lua55` + `vendored`; Lua 5.5.1 is current), user's choice.
- Batch 1 (easel, tip, cracks) and batch 2 (drying, scene, fixes-paint, fixes-ux) running. Batch 3 (atmosphere, green) waits for batch 1.
- cracks merged: organic web fitted to the ground (was a brick grid); open: cracks invisible in darks, density still even across skies.
- fixes-ux merged (stage UX, Copy fields, roughen units, form docs). Coast #17 = NaN gesture point: reject non-finite points in bristle.rs after tip merges (test crates/paint/tests/curved_drag_nan.rs, ignored).
- easel v1 done (LuaJIT via mlua 0.10: read brief before the Lua 5.5 edit). Resumed as easel-2: switch to Lua 5.5/mlua 0.12, merge main, expose form, exact rollback.
- drying merged (checkpoint PAINTCK3 combines drying state + cracks' ground; resume verified byte-identical).
- scene merged: one sun, shadows/contact/reflections consistent in all three study panels. Its 'glaze at thickness 5 wipes to ground' report doesn't reproduce on main after drying's settle NaN fix (probe: glaze darkens toward its own color).
- atmosphere launched (sky light model, cloud volumes, noise toolkit, ranges that don't read as waves). Running: tip, fixes-paint, green, easel-2, atmosphere.
- green merged: sourced greens + palettes, foliage clumps, Sward. Weak painting: savanna umbrella oak (young-oak habit), leaf masses as flat cut-outs at 3200px, uniform round light dabs. Follow-up after tip: mature oak habit; foliage edges as pointed-tip hooked marks.
