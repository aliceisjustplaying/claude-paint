# Thinner acceptance checks

Four of the thinner's acceptance checks are left, all in `scripts/test` (the
gate). Their answers and limits are unchanged; the full text of every check,
its review history and the files' approved hashes are in this file's history
(last at `de23782`).

- **Check 2, no thinner:** `scripts/thinner_check2 <easel>`. Without thinner,
  or with `thinner=0`, engine 3 leaves the before-change baseline
  (`notes/thinner/baseline/`, from unchanged af49348) as it was: every state
  field after every chunk of the six scenes, and their PNGs. The rag scene's
  mismatch is the registered known failure (`scripts/known_rag_baseline.py`).
  `crates/easel/tests/thinner/dump_solvent.lua` is its positive control.
- **Check 4, nothing vanishes:**
  `c04_a_stroke_and_a_wipe_account_for_all_paint_and_solvent`
  (`crates/paint/tests/thinner_physics.rs`).
- **Check 9, evaporation:**
  `c09_the_solvent_evaporates_and_the_film_loses_its_volume` (same file).
- **Check 13 (a), the pigments stay as they were:**
  `c13_every_pigment_value_is_af49348s` (`crates/paint/tests/thinner_pigments.rs`)
  against `crates/paint/tests/thinner/tubes_af49348.txt` and `tubes_current.txt`.
