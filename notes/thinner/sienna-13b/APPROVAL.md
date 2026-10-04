# Check 13 (b) restated: approval request

Lane D of the engine-3 plan (AGENT_BRIEF_V2 §5, REVIEW_RESPONSE "Sienna
13(b)"). Base: `d54b423`. Nothing here changes pigment constants, the
catalog's hiding (0.40 raw, 0.45 burnt), check 13 (a) or any engine code.

**Proposed change:** `sienna-13b.patch` in this directory, made with
`git diff` against `d54b423` plus this lane's palette comment commit. It
touches only the following files; all but `notes/speed/SKIPPED.md` are
protected:

| file | change |
|---|---|
| `crates/paint/tests/thinner_pigments.rs` | new (b) `c13_burnt_sienna_has_a_lower_contrast_ratio_than_raw_sienna_at_equal_film`; the old (b) becomes `c13_diagnostic_sienna_card_retained_absolute_substrate_difference` without its ordering assertion |
| `scripts/test_thinner_acceptance` | the expected-failure path is removed; (b) and the diagnostic are required in every mode, with `--show-output`; exit 0 when all pass (exit 3 is gone) |
| `scripts/tests/thinner_acceptance_runner.sh` | its cases follow: all ok → 0; (b) failing or not running → 1; the diagnostic failing → 1 (19 cases, all pass) |
| `notes/speed/test_lists/fast.tsv`, `all.tsv` | the thinner steps lose their known-failure columns (8 to 10) |
| `notes/thinner/ACCEPTANCE.md` | check 13 (b) restated, the run table and exit codes, the decision, three file hashes |
| `notes/speed/SKIPPED.md` | the test count (27 → 28) and the known-failure sentence |

Apply with `git apply notes/thinner/sienna-13b/sienna-13b.patch`. The
protected paths then need `scripts/golden_approve` approvals by a reviewer
other than this lane (notes/speed/SAFEGUARDS.md).

## The new requirement

At 3, 10, 25 and 30 µm of unthinned (wholly nonvolatile) paint, on two
named substrates (the black/80% white chart; the Sargent box's bone black
and lead white masstones), burnt sienna's RGB luminance contrast ratio
(Y over black ÷ Y over white) must be lower than raw sienna's by at least
1e-4. The films come from the engine's uniform-film optics (`Paint::over`).
No brush is involved. The thicknesses are those of the existing diagnostic
`look_sienna.rs`.

**The tolerance comes from the arithmetic.** The test file's comment
states the reasoning: f32 rounding plus libm last-place differences stay
near 1e-5, so 1e-4 is ten times that and about forty times smaller than
one 8-bit display step. It was set before the run, without reference to
the measured gaps (0.011 to 0.029). Afterward, an f64 port of the same
formulas differed from the f32 ratios by at most 9.2e-8 (EVIDENCE.md).
Ties within 1e-4 fail. The old test passed ties ("at least as well"), but
Field/Salter says "more transparent".

## Recommendation

Approve. The current (b) measures something else: the retained absolute
black/white difference on a brush-painted card. Burnt sienna's masstone
(Y 0.080) is darker than raw's (Y 0.174), so it absorbs more and keeps less
of that absolute difference (9.85% against 17.91%), even though less of it
hides by the standard ratio. On the same card the contrast ratio is
burnt 0.4642 against raw 0.4998. Every direct film on both substrates ranks
burnt lower (EVIDENCE.md). Because the restated check compares equal direct
films, it is independent of how lanes A and B change brush deposition.

## Tradeoffs

- **What it checks.** The restated (b) checks how the engine's optics treat
  the two catalog paints. It doesn't establish a historically calibrated
  pigment. A ratio test is not a full validation of historical paint
  (REVIEW_RESPONSE).
- **Lost coverage.** Brush-made film is no longer part of the ordering
  check. The diagnostic still runs, and its fixture assertions are still
  required: paint is laid, the films are equal, and both siennas are on the
  same card. It just doesn't rank them.
- **Channel disagreement.** The green channel ranks the other way from
  10 µm up: burnt's green ratio is higher. The check uses luminance and
  prints each channel without asserting on them, as the brief directs.
- **Unchanged catalog hiding.** The catalog keeps 0.45 for burnt and 0.40
  for raw, the reverse of the rendered order. The scalars are inputs
  calibrated through a grayscale surrogate, and their rendered one-coat
  luminance ratios are 0.3778 and 0.3884. A doc comment on `Tube::hiding`
  (`crates/paint/src/palette.rs`, committed on this branch) says so.
  Changing the scalars would change rendering and (a).
- **Gate effect.** Exit 3 is gone, so `scripts/test` can give `pass`
  instead of `known_failure` for the thinner steps. That makes the merge
  gate reachable for this check. The reviewer is approving that effect as
  well.

## Not part of this lane

The brush-made sienna card has to be regenerated from the exact final A+B
candidate, and the old card must not be reused as evidence. These commands
produce it:

- `~/src/a/claude-paint-tools/lockrun --timeout 300 --owner <you> -- scripts/test_thinner_acceptance --card`:
  check 1's 2400 px card (raw sienna thinned 0.5) and, with this patch,
  (b) and the sienna card's diagnostic, with their numbers in the log;
- `LOOK_OUT=<dir> cargo test --release -p paint --test look_sienna -- --ignored --nocapture`:
  the thinned two-sienna card and the numbers in notes/look/logs/sienna.txt.
