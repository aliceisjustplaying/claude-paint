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
