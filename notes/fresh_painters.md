# Amnesia experiment: three fresh painters (winter, coast, mountains)

Three agents that never saw earlier paintings each composed an original
Friedrich from the engine + research notes (worktree `claude-paint-fresh`,
programs saved in `paintings/src/bin/fresh_*.rs` there). Sessions died in a
power loss before their final reports; findings below are mined from their
reasoning, plus the user's review.

## What worked
- Compositions: all three found genuinely Friedrich-like pictures (cross on a
  rock with firs and a wanderer; dusk shore with brig, figure, anchor; winter
  cross with dead oak, spruces, Rückenfigur, church in fog). Winter best.
- Snow, fog/mist, small grasses, birds; the cross; spruces beside it.

## Shared failures (user + painters)
1. **Too straight, too neat.** Horizontal everything: sky strokes, sea
   ripples ("ruler-like"), ridges as horizontal bands with hard edges. Not
   enough entropy in stroke geometry and placement.
2. **The same sky three times.** All converged on Style::broad + blend
   defaults: the style, not the painter, is making the sky.
3. **Paint color means "appearance over white".** Semi-transparent paint
   dries darker/yellower than the target over any non-white underlayer:
   stipple dots darker than the sky ("digital rain"), glazes darkening,
   seams where palette recipes switch between low-hiding smalt mixes and
   lead-white mixes (coast). Painters hand-rolled canvas sampling to match
   tones. Needs KM-aware "what will this look like here" mixing.
4. **No stippling.** Friedrich's actual sky/mist technique; attempts failed
   (see 3) and were dropped.
5. **Solid forms have no light model.** Rocks read as macarons, loaves,
   haystacks, beetles, soap bars; painters built their own height fields and
   shading. Need a form/planes lighting helper and a rock motif.
6. **Oak incoherent.** Twigs scattered all over, not a growing hierarchy;
   sausage joints; blunt broken limbs; dangling roots.
7. **Coverage.** Ground shows through (red/orange/white specks), strokes
   aren't seeded past canvas/mask edges.
8. **Brush quirks.** Hog push makes hollow outlines on short/retraced
   strokes; big round dabs show each bristle ("frogspawn"); tiny marks look
   fine at 1000px but become beads at 3200px; blender in shuffled tile order
   drags paint across gradients (painters worked top-to-bottom in bands).
9. **Motif APIs.** Take raw colors not palette/Paint; spruce top doesn't
   taper; figures: no women, few poses, leg/boot gaps.
10. **Workflow.** Crop-region render at full resolution; stage checkpoints
    (serialize canvas to resume); preview-vs-full mismatch; mask ops
    (distance, offset, facing the light).
11. **Detail.** Friedrich's pictures are full of tiny particular things; the
    API makes broad area work easy and detail laborious, so agents paint
    broad strokes.
