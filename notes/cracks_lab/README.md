# Cracks lab: pick by eye (Alice)
Round 10's frozen pond (C), finished six ways: everything identical except
the craquelure (re-finished from its last-stage checkpoint at 3200).
- `sheet_1to6.png`: each variant 1 to 6 as two 1:1 crops of the 3200
  render: the thin sky (left) and the dark tree against the warm sky
  (right). Look at it at 100% zoom; the cracks are fine lines.
- `whole_1..6.png`: each variant whole (scaled down from 3200).
Unlabeled on purpose; the key is added after Alice picks.
Variants use settings that already exist (crates/paint/src/crack.rs):
direction (`grain`), island size and opening (fewer cracks), and none.

## Alice's pick (round 1) and round 2
Alice: "4 and 5 are the best ones, with the caveat that especially 5 looks
digital, and yet the right direction vibe-wise". Key: 1 = fewer cracks,
2 = fewer + direction 0.6, 3 = none, 4 = as delivered (the painter's
narrow cracks: width 16 um, depth 14 um, dirt 0.25), 5 = as delivered +
direction (`grain` 0.6; default 0.15), 6 = direction 1.0.
Round 2 (`round2/`): the same painting with direction strengths between 4
and 5, unlabeled A-E (`sheet_AtoE.png`, `whole_A..E.png`). Astra and Fable
judge the same crops blind, as conservators.

Alice, after checking (she had taken "the Monk" to mean round 2's winter):
the real *Monk by the Sea*'s cracks flow broadly down and to the right,
and that is one pattern: other Friedrichs have other patterns. The
"very much digital-like" cracks she mentioned were round 2's winter
(ours), not the Monk's. So direction is real and should vary per painting;
what reads as digital is our cracks, not directionality as such. (The
round 2 judges' brief describes the Monk's flow correctly.)

Round 2 key: A = direction 0.6, B = as delivered (0.15), C = 0.25,
D = 0.35, E = 0.45. Astra (blind): C, B, D, E, A: "straight stretches are
not inherently wrong. The giveaway is repeated long stretches with similar
orientation, separated by comparable gaps ... like ladder rungs"; "dark
paint can crack too".
Alice: "yes ... this is why I always say entropy: anything that reads like
repetition reads digital" (principles.md 2b). Suspect in the model: the
fixed relaxation distance (D_r ~ S/2) gives even spacing; direction only
makes it visible. Next: vary spacing, width and density locally (several
fracture scales, dense patches and quiet areas), not the direction knob.
