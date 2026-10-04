# Look experiments: siennas, the thinned stroke's rim, a less tidy rag

Branch `look-experiments`, from d6318d1. Engine 3 only; engines 1 and 2 are
unchanged (the old-log check replays all 11 old cases as before). Every
heavy command ran through `~/src/a/claude-paint-tools/lockrun`; nothing
timed out. Not pushed.

**Pictures**

- `compare.jpg`: today against each variant, all in one sheet:
  - top two rows: a thinned stroke and a thinned broad pass, today, (a),
    (b) and (B);
  - third row: the rag's dry and damp wipes, today against final;
  - bottom row: the damp wipe on thinned paint, today against final, and
    the two siennas.
- `thinned_strokes.jpg`: each rim variant, with the stroke just after and
  30 minutes after, and the broad pass with linen and on a plain ground.
- `rag_study_today.jpg` and `rag_study_final.jpg`: the whole rag study,
  before and after (the same 8 rows as `notes/thinner/rag_study.jpg`).
- `siennas.jpg`: the two siennas side by side.

| commit | what |
|---|---|
| 1efff68, 957d87e | 1: sienna measurement (test and log only) |
| b194901 | 2 (a): a thinned load ploughs less |
| bd5addc | 2 (b): the ceiling caps the film a pixel holds; (a) taken back out. **The rim fix now in place** |
| abb5f75, 0ce3dc4 | 2 (B): an absolute ceiling, and its revert (kept so you can see it) |
| d6bbd5e | 3 (1): folds that miss |
| 8add36d | 3 (2): streaks survive a damp cloth |
| 55b1ec0 | 3 (3): ragged pad edge, more hand wander |
| 41680fe | 3 (4): a loaded face smears a little back |
| (this commit) | this report and the pictures |

The branch ends with (b) and all four rag changes in place.

One slip to disclose: my first attempt at reverting (B) had a bad flag, so
the revert didn't happen. My next command (`git commit --amend`) then only
renamed the B commit (never pushed). I gave it back its proper message
(abb5f75) and reverted it properly (0ce3dc4). Nothing else was amended.

## 1. Siennas: which is more see-through

Measured by `crates/paint/tests/look_sienna.rs` (ignored; run with
`--ignored --nocapture`); the output is in `logs/sienna.txt`. It uses the
same card as the burnt-against-raw check (thinner check 13 (b)), which is
unchanged.

**The standard measure.** The contrast ratio is the paint's reflectance
over black divided by its reflectance over white, for the same film
(ASTM D2805 / ISO 6504-3, https://en.wikipedia.org/wiki/Hiding_power).
Lower means more see-through.

**By the standard ratio, burnt sienna is the more see-through of the two,
at every thickness, thinned or not.** By today's difference measure
(how much of the card's black-to-white difference still shows), raw comes
out ahead. The two measures disagree, and the check uses the difference.

Equal uniform films, the engine's own optics. The solvent has no color, so
thinned and unthinned paint look the same at an equal paint film. The
chart is black and 80% white:

| paint film | raw: ratio (luminance) | burnt: ratio (luminance) | raw: difference kept | burnt: difference kept |
|---|---|---|---|---|
| 3 µm | 0.047 | **0.035** | 78.8% | 66.5% |
| 10 µm | 0.169 | **0.154** | 49.7% | 30.6% |
| 25 µm (one coat) | 0.452 | **0.438** | 20.5% | 10.5% |
| 30 µm | 0.537 | **0.512** | 15.6% | 8.2% |

Per channel at 10 µm (red, green, blue), the two differ by hue:

- raw: 0.143, 0.177, 0.798;
- burnt: 0.104, 0.192, 0.661.

Burnt is lower in red and blue, higher in green.

On the card itself (twenty strokes at load 0.5, the check's own scene):

| | paint film | ratio (luminance) | difference kept |
|---|---|---|---|
| unthinned, raw | 37.9 µm | 0.500 | 17.9% |
| unthinned, burnt | 37.9 µm | **0.464** | 9.8% |
| thinned 0.5, raw | 1.64 µm | 0.040 | 87.3% |
| thinned 0.5, burnt | 1.64 µm | **0.033** | 79.7% |

Scattering per coat: raw 0.296, burnt 0.199. The tubes' own one-coat
hiding is raw 0.40, burnt 0.45. That figure is a ratio taken on the
masstone alone, not over a real black and white; over them, as above,
burnt's ratio is lower.

**Why they disagree.** Burnt sienna is much darker (masstone luminance
0.080 against 0.174), so it darkens the white band more. That shrinks the
black-to-white difference even though, relative to its own brightness,
it lets more of the black show.

`siennas.jpg` shows the same thin film of each (thinned 0.5) over light,
mid-grey and dark bands: raw reads as a yellow veil on the light band,
and burnt as a warmer, darker one.

**Recommendation.** Leave the tubes alone. If you agree, check 13 (b)
should be restated in terms of the contrast ratio: by that measure burnt
sienna already passes ("more transparent than raw", Field/Salter). I
haven't changed the test; that's your call.

## 2. The thinned stroke's rim

**The cause, verified.** The scene is the one of `single_stroke_edge.jpg`
(`crates/paint/tests/look_rim.rs`). With the plough and the pickup both
off (a diagnostic setting, not committed), a single thinned stroke is a
flat 3.0 µm from edge to edge, which is exactly its ceiling. With only the
plough off, it is 2.7 µm at the edges and 2.0 µm inside. With only the
pickup off, it is 2.4 and 1.4 µm. So the lead's hypothesis holds:

- the plough moves paint from the middle to the edges and accounts for
  most of the rim;
- the pickup adds a little more, since the edge hairs touch less and so
  lift less.

The ceiling counted only paint the stroke added, so neither the paint
taken nor the paint pushed was held back by it.

**What changed:**

- **(a):** the plough is scaled by the share of paint in the liquid
  (× 0.5 at thinner 0.5).
- **(b):** a thinned stroke leaves a pixel holding at most its ceiling
  more than it held when the stroke reached it. Paint lifted out makes
  room again, and paint ploughed in counts against the ceiling.
- **(B), absolute:** a pixel holds at most the larger of the ceiling and
  what it held before.

Unthinned paint never takes any of these paths: the baseline scenes
without a rag are equal on every field, and the unthinned stroke's
numbers are unchanged.

| variant | stroke edges / inside, just after (µm) | after 30 min (µm) | broad pass: paint, unevenness (spread ÷ mean) | card kept, load 0.3 / 0.6 | thinner acceptance (27 required) |
|---|---|---|---|---|---|
| today | 2.25, 2.13 / 0.96 | 2.03, 1.93 / 0.96 | 2.8 µm, 0.47 | 84.4% / 81.4% | all pass |
| (a) | 1.56, 1.39 / 1.35 | 1.51, 1.39 / 1.35 | 2.9 µm, 0.47 | 83.5% / 80.3% | 26 pass; **check 15 fails** |
| (a), plough × (1 − share)² | 2.06, 1.95 / 1.60 | 2.02, 1.87 / 1.61 | 2.9 µm, 0.47 | 82.9% / 79.7% | 25 pass; checks 15 and 16 fail |
| **(b), kept** | 3.00, 2.99 / 2.97 | 2.92, 2.98 / 2.97 | 7.7 µm, 0.49 | 73.5% / 71.7% | **all pass** |
| (a) + (b) | 2.82, 2.94 / 2.98 | 2.77, 2.87 / 2.98 | 7.4 µm, 0.49 | 74.1% / 72.3% | all pass |
| (B), reverted | 3.00, 2.99 / 2.97 | 2.92, 2.98 / 2.97 | 2.9 µm, **0.13** | 83.5% / 83.2% | 23 pass; **checks 7, 9, 15, 16 fail** |

In every row, check 13 (b) fails as before (the siennas, above). "All
pass" means every required test passes and only that known failure
remains. The (a)+(b) and the squared-plough rows were measured with
temporary switches (environment variables) in a single build and were not
committed. The committed (a) and (b) are the same code without the
switch.

**What the failing checks say:**

- **Check 15** (solvent doesn't slow the oil cure) needs at least 100
  pixels holding 12 µm or more after five thinned passes:
  - (a) leaves 79, and the squared plough 46;
  - (B) leaves none.

  With less plough, less paint heaps up into thick spots. Nothing about
  drying went wrong; the test no longer finds the thick film it measures.
- **Check 16** (spreading mixes the liquids at a boundary) needs at least
  20 boundary pixels to gain paint. It got 3 (squared plough) and 2 (B).
  Again, the film it relies on isn't there.
- **Check 7** (a second thinned pass adds at least 20% more paint than one)
  fails under (B): 20,379 against 23,187, only 14% more.
- **Check 9** (five passes are thicker than one) fails under (B): the
  median is 3.0 µm for both.

(B) contradicts the agreed rule that each thinned pass builds on the last.
That's why it is reverted.

**What you see** (`thinned_strokes.jpg`):

- **(a):** strokes are still outlined, a little more faintly.
- **(b):** a single stroke is now an even tone with no rim. The broad pass
  loses its outlined slivers, but now reads as blotchy. Where strokes
  overlap, each lays its own ceiling, so the pass holds 7.7 µm and is
  darker. Its unevenness is about the same as today's, now as overlaps
  rather than outlines.
- **(B):** the broad pass becomes one even veil, but only by breaking the
  build-up rule.

**About the card.** Under (b) it keeps 73.5% and 71.7%, down from 84.4%
and 81.4%, because the passes now stack. That is still above the half its
test name asks for.

**Recommendation:** keep (b). It removes the cause of the rim and passes
everything. Today's thin 2.8 µm broad pass existed partly because the
plough and pickup kept removing earlier strokes' paint. If the stacking
looks too dark or blotchy, a smaller `STROKE_FILM_UM` is the knob; that's
a separate, small sweep I didn't run. Don't take (a): with (b) in place it
changes nothing, and alone it fails check 15.

## 3. The rag, less tidy (`crates/paint/src/rag.rs`, engine 3 only)

New constants are marked [E] (estimates), as the file does.

1. **Folds that miss** (`FOLD_MISS` 0.25..0.5, `FOLD_PRESS` 0.2): the
   cloth's contact goes from 0 in the gaps between creases to 1.
   - About a tenth of the pad misses at pressure 0.5, and about 3% at 0.9.
   - The mean contact is 0.74, close to today's 0.75.
   - My first try, contact down to 0 over a much wider range of the crease
     noise, removed only a third of the paint. Its numbers are in the
     first run, not shown here.
2. **Streaks survive a damp cloth.**
   - The cloth's contact now multiplies the share taken after the pad's
     rate has saturated, so a damp cloth still streaks.
   - Spirits reach deeper (`DAMP_REACH` 1.5: the cloth sags further, and
     the fibers wick up to all of the paint below at a full dip).
   - The damp rate itself (`DAMP_LIFT`) is unchanged.
3. **Ragged pad edge, more wander.**
   - Each side of the pad frays by up to 35% of its half-width (`FRAY`,
     `FRAY_MM`).
   - Each end of a wipe starts and stops unevenly across the pad
     (`END_MM`).
   - The hand wanders more: width ±25% and line ±15% of the pad, at two
     scales (`WANDER_W`, `WANDER_OFF`), where today it is ±12% and ±6%.
4. **A loaded face smears a little back** (`SOAK` 0.35, `SMEAR` 0.2).
   - Paint just lifted stays at the cloth's surface for a few steps.
   - Some of it is laid back over the pad's light-pressed rim (the frayed
     sides and the trailing end), then subtracted from the rag's load.
   - The paint balance check (check 4) still passes, so paint is
     conserved. The effect is small in numbers (dry wipe: 79.2% removed
     without it, 79.1% with it).

**The rag study's numbers** (paint removed / tone left / elongation; the
`rag_study` test with THINNER_RAG_STUDY set):

| panel | today | final (1-4, with (b)) |
|---|---|---|
| dry cloth wipe, wet paint | 90.5% / 47.6% / 5.6 | 79.1% / 52.6% / 5.1 |
| blot, wet paint | 23.6% / 87.9% / 1.1 | 20.5% / 89.0% / 1.1 |
| damp cloth wipe, wet paint | 97.2% / 20.8% / 5.3 | 86.5% / 37.0% / 5.2 |
| dry cloth wipe, thinned paint | 68.1% / 44.3% / 5.7 | 73.2% / 42.0% / 5.2 |
| blot, thinned paint | 21.2% / 83.0% / 1.1 | 18.5% / 84.3% / 1.2 |
| damp cloth wipe, thinned paint | 68.3% / 44.1% / 5.4 | 78.8% / 35.6% / 5.3 |
| dry paint (control) | 0% / 100% | 0% / 100% |
| brush stroke over a wiped area | 3.3 → 8.5 µm | 23.9 → 28.5 µm |

Step by step, for the dry wipe on wet paint (removed / tone left):

| changes in place | dry wipe | damp wipe |
|---|---|---|
| today | 90.5% / 47.6% | 97.2% / 20.8% |
| 1 | 80.9% / 48.6% | 89.9% / 28.9% |
| 1, 2 | 77.3% / 53.4% | 84.5% / 40.1% |
| 1, 2, 3 | 79.2% / 52.4% | 86.6% / 36.8% |
| 1, 2, 3, 4 | 79.1% / 52.6% | 86.5% / 37.0% |

The step-by-step rows were measured on the rim fix's starting point, so
their thinned panels differ from the final column. The wet-paint panels
don't involve thinner, so they agree.

**What changed in what you see** (`compare.jpg`, third and fourth rows):

- **Damp wipe:** instead of a smooth, round-ended strip, it is a streaked
  clearing with ragged sides and uneven ends.
- **Dry wipe:** narrower, with more broken streaks.
- **On thinned paint:** a damp cloth now clearly lifts more than a dry one
  (78.8% against 73.2%; today 68.3% against 68.1%).
- **The cost:** the wipes are less clean. A damp wipe on wet paint leaves
  37% of the tone, where today it leaves 21%.
- **The stroke over the wipe** adds about as much as before, but the wipe
  left more paint (23.9 µm), so the first number is higher.
- **The thinned tone:** in the final study it is darker than today's,
  because of (b): the broad pass now stacks.

**Tests:**

- **The rag's own tests:** all pass at each step and in the final state.
- **"Spirits lift nearly to the ground"** (3 passes by hand):
  - dry cloth: 34% of the tone left, film off the tops 97% and the
    hollows 83%;
  - damp cloth: 12% of the tone left, 98% off both.
  - It asks for at least 95% off with spirits.
- **The rag study's own checks:** all pass.
- **Checks 4 and 16** (the rag carries solvent in the film's own ratio,
  and balances): pass.
- **The baseline's rag scene now differs, as expected.**
  `notes/thinner/baseline/tools/compare_build.sh --added-zero` shows chunks
  4 to 8 of the `rag` scene differ; chunk 4 is its first wipe. The fields:
  - `wet.vol`: 382 of 8192 values differ at chunk 4, 1248 by chunk 8;
  - `wet.lat`, `wet.hide`, `clock.cure`, `clock.lev`, `clock.seen` and
    `clock.th`;
  - `wet.floor` (chunk 8 only, the brush stroke after the wipes);
  - the rags' `load` and `soaked` (`rags.debug`), for example rag 1's load
    after its first wipe 0.785 → 0.596.

  The `rag` scene's PNG and printed output differ too. The other five
  scenes (body, overlap, pickup, stroke, wait) are equal on every field. I
  re-recorded nothing.

**Recommendation:** keep 1 to 3, which is where the visible change comes
from. Change 4 is nearly invisible at these settings. Keep it if you want
the behavior, or raise `SMEAR` and look again.

Before relying on the rag:

- check that you're happy with the damp wipe leaving more tone than
  before;
- run the reviewer's suggested second damp wipe, which I haven't done.

## Test runs on the final state (41680fe)

`scripts/test` (fast), once:

| step | result |
|---|---|
| build-release-easel, build-test-binaries, build-thinner-release-tests | pass (48 s) |
| cargo-test | pass: 273 run, 0 failed |
| old-logs | pass: all 11 old cases replay as before |
| baseline-package | **fail**, then pass. A stray Python cache file, `notes/thinner/baseline/tools/__pycache__/state_compare.cpython-313.pyc` (written at 10:22, during an acceptance run), made the file list disagree with the package's checksums. It is gitignored. Deleted, the check passes: "hashes ok". Running the acceptance checks seems to leave this file behind; worth a look by whoever owns the package check. |
| baseline-state | **fail, expected**: the `rag` scene differs (above), the other five are equal |
| thinner-quick | **fail**: check 2 fails because of the `rag` scene (above), plus check 13 (b)'s known failure. The other 20 required checks pass |

The full thinner acceptance (`--all`, with THINNER_RAG_STUDY_DIR set to a
temp folder): 26 of 27 required checks pass, including the card. Check 2
fails (the `rag` scene, above). Check 13 (b) fails as known.

## Questions for you

1. Siennas: restate check 13 (b) in terms of the contrast ratio, under
   which burnt is already more see-through?
2. Rim: (b) as committed, with thinned passes stacking (card 73.5%), or
   a smaller per-stroke ceiling to bring the broad pass back near today's
   tone?
3. Rag: accept the less clean damp wipe (37% of the tone left, against
   21%)? And re-record the baseline's `rag` scene once you've seen the
   pictures?
