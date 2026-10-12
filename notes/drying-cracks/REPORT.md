# Drying-cracks RFC: evidence and validation

Agent-written with OpenAI Codex. This change adds a proposal, a manual
diagnostic, observations and an example image. It changes no engine or
existing test/expected-answer files. Related:
[ai-painting issue 5](https://github.com/davidfwatson/ai-painting/issues/5).

## Provenance

- Upstream base: `04cc795b18cf4c63421d1fad29a6b4ade7bf1c9c`.
- Prototype: public fork `b9dfeb2ab185b2a50bed5ca18c89bc172e20e1f2`,
  which vendors upstream `059b3a2` and adds its own engine 6.
- Platform: macOS arm64; `rustc 1.97.1 (8bab26f4f 2026-07-14)`.
- The probe is injected into a disposable `git archive` snapshot of the
  pinned fork. Its physics code is unchanged. Synthetic film insertion
  bypasses the brush API; images are exploratory evidence, not goldens.
- Work is on a separate upstream branch. No whole painting was replayed,
  no engine number allocated, and no protected answer changed.

## Manual diagnostic

Command: the `scripts/lockrun --timeout 600 -- cargo test --locked ...`
invocation in [README.md](README.md), filtered to the single manual
`defects::issue5_probe::study` diagnostic. Build target and output live
under `$TMPDIR`.

The diagnostic completed: one selected test, no ignored or failed tests;
30.7 s to build and 0.61 s to run (31.6 s under the lock). The contact sheet
was visually inspected. The model's wrinkle severity at the chosen
undercoat parameters is zero.

The reported central fraction of pixels with linear luminance below 0.4
was 0%, 0.486% and 14.557% over the soft undercoat at one hour, five days
and sixty days. It remained 0% for the hardened-undercoat and engine-5
controls. This threshold is a descriptive measurement, not an assertion
of real crack area or a physical calibration.

`dry()` and `wait(0); dry()` advanced to the same clock and left equal
seen pixels, but the former recorded zero softness and the latter
10,354,687 pixel·µm summed softness. One three-year top-film wait left 0%
dark central pixels; splitting it into two days plus the remainder left
16.410%, with a maximum linear-channel difference of 0.7692 and height
difference of 25 µm. Both ended at the same clock. Retired active records
do not mean the permanent damage disappeared.

At the five-day checkpoint there were 33,642 active crack pixels, 17,474
with nonzero opening. Immediate restore and a further 55 days matched
complete checkpoint bytes, seen linear pixels and encoded RGB pixels
exactly. A one-worker versus four-worker run also matched all three.
These checks cover this scene and schedule, not arbitrary layers or waits.
Raw observations and comparisons are linked from the RFC.
The final probe was run again (2.8 s including its rebuild, 0.60 s test
time): all 26 generated PNG/CSV files were byte-identical to the first
run. The committed sheet's SHA-256 is
`4b3b8f3f20e10526a619ba255396b7fe893bf14c025ff80ac565098df1848c48`.

## Existing checks

Commands use disposable build/cache directories and upstream's
unmodified test runner. The required runner separates its build and
check phases, each under its own machine-wide lock.

The initial `scripts/test` run ended unfinished (exit 3, 138 s): the
system Bash 3.2 lacks `mapfile` and `EPOCHREALTIME`, causing `cargo-test`
to time out and the baseline guards, baseline state and thinner check to
fail. The eleven old-log image/repeat checks passed in that run. The
original receipt and logs were kept.

Bash 5.3.20 was extracted into a temporary directory from a verified
Homebrew bottle and its library paths relocated to the installed
dependencies; uv 0.13.0 was provided by a temporary virtual environment.
Bash and uv were exposed through the task's PATH. No test source, limits, expected answers
or known-failure policy changed.

With that prerequisite fixed, `scripts/test` completed (exit 4, 146.7 s):
12 steps, 442 runner-reported tests. All checks passed except the known
solvent-rag hollow-film lift assertion and historical rag baseline
mismatch (the latter appears in two steps). All eleven historical images
matched their macOS arm64 references byte for byte, and the repeated
replays matched images and state digests exactly. The baseline guard
tests passed, and all five non-rag baseline scenes matched their state
and images.

`scripts/test --all` completed in 595.5 s: 29 steps, 1,080 runner-reported
tests, exit 1. It retained the same known rag failures. Its other failures
were setup problems: `studio-viewer` could not read a tracked archived
source omitted by the sparse checkout, and `peek` lacked ImageMagick.
All remaining steps passed, including slow determinism, thin blend,
painter/box builds, safeguards and replay scripts.

The missing archived fixture was restored from the tested upstream commit.
ImageMagick 7.1.2-33 was extracted from a verified bottle into a temporary
directory and relocated to existing libraries. All runner fixture
directories were also included to cover the tests selected by their glob.
The focused recovery command was:

```sh
scripts/test --all --phase check --only studio-viewer --only peek --only runner-round24
```

It ran 367 tests in 29.1 s: all 72 viewer tests, the peek check and 294
runner tests passed. The runner count includes 15 fixture cases beyond
the original sparse run's 279. Each selected step exited 0; the coordinator
exited 3 because `--only` is deliberately classified as a partial run.
The original full-suite failure receipt remains intact. This is a full
run plus focused recovery, not a new all-green full-suite receipt; every
required step was exercised, and the only unresolved failures are the
two existing rag issues. Receipts and step logs were retained locally.

The contribution is one commit on `codex/drying-cracks-rfc`, based on the
upstream commit listed above. Independent read-only review found no
blocking issues in the probe, image, observations or proposal. Temporary
checkouts and build/dependency directories are removed after publication;
the report, diagnostic assets and validation archive are preserved.

## Scope and follow-up

This completes the proposal/evidence step of the linked issue. It does
not complete a production port of drying cracks, wrinkles or drips.
First resolve the lifecycle and material-state gaps described in the
RFC, then agree on the desired mechanism, engine number and enablement.
The first production slice would include only through-drying and
traction damage, upstream checkpoint/state integration, and verification
of old-engine compatibility and damage evolution. Wrinkles and gravity
remain separate proposals.
