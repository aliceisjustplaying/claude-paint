# Test tiers

## Iteration

Run `scripts/test-changed` after a fix. It unions the committed diff against
`origin/main`, staged and unstaged changes and untracked files. Override the
base with `--base REF`, or pass repository-relative paths for just the current
round. `--dry-run` prints the checks and build commands without running them.
`--summary FILE` and `--logs DIR` retain the normal runner's evidence.

| Changed paths | Iteration checks |
| --- | --- |
| `crates/paint/src/` | Workspace Rust checks, including downstream easel tests |
| `crates/easel/src/` | Easel Rust binaries, without rerunning unchanged paint unit tests |
| Rust integration test files | That test binary and its build only |
| Thinner or solvent code and fixtures | Quick Rust checks plus the thinner's exact-name acceptance checks |
| Palette, box and build-script code | Rust checks plus the feature matrix, tube fixtures and painter build |
| `studio/` | Python server/export tests, Python Playwright startup tests and both Node Playwright suites |
| `harness/painter/` | The Node harness tests |
| `notes/round24/runner/` | Current runner Python tests and Node reader-scope tests |
| Known scripts and baseline fixtures | Their script regressions and required builds |
| Documentation only | No executable tests, except documents with dedicated regression checks |
| Build configuration, an unmapped code path or a gate list | Broad iteration checks, including all feature builds; ignored live-width checks and the full card remain final-only |

Selected checks run through `scripts/test`, with its lock, positive test
minimums, time limits and process cleanup. They run sequentially so a targeted
round does not start unrelated jobs. An iteration pass is not a full-gate
receipt. Existing ignored live-width checks stay in the final tier.

## Final head

Run `scripts/test --all` once after the final code changes, alongside independent
review when possible. Use targeted checks while addressing review findings.
Only a full-gate pass on the resulting head establishes readiness.

The full list is `notes/speed/test_lists/all.tsv`. It retains every earlier
required Rust test, exact-name slow test, thinner acceptance check, feature
configuration, shell regression and Python test. It also runs the Node harness,
current reader-scope tests, both Node Playwright suites, Python Playwright startup
tests, archive/source Python regressions and standalone thinner self-tests.
Node totals are counted as tests, including failures and cancellations. Browser
tests must run: missing browser dependencies fail their step or its positive
minimum. Historical copies and diagnostics remain listed in
`notes/speed/SKIPPED.md`; no tests were deleted.

The JSON summary records each step's wall time and user/system CPU seconds
when `/usr/bin/time` is available. CPU totals include reaped child processes;
detached test servers and runner bookkeeping are outside those totals.

## Shared builds and prerequisites

Both tiers use `target/gate`: optimization level 3, incremental code generation
with 256 units, no debug symbols, debug assertions and overflow checks enabled.
The CLI, workspace tests and thinner tests share the same engine build. The
thinner's gate run enables the workspace's `hand-hook` feature so Cargo can reuse
those test artifacts. Feature configurations that would replace the shared CLI
keep separate target directories. Production release and iteration profiles are
unchanged. Standalone thinner acceptance and replay/export scripts still default
to release; only the test runner sets `CLAUDE_PAINT_TEST_PROFILE=gate`.

The thinner's 20 monotonicity cards reuse one prepared 480 px substrate,
restoring the complete session snapshot before each unchanged load/thinner
case. Successful numeric card setup uses replay sessions to avoid rollback
copies; rollback tests still use live sessions. A new equivalence check compares
every measured field and complete save bytes against fresh live sessions.
The required 2400 px card, all seeds and all original assertions remain.
Python HTTP fixtures use a 20 ms shutdown poll and startup tests share one
browser process while closing every context after each test. Browser assertions
and server/export cases are unchanged. The replay suite writes its timing
report to step scratch storage instead of modifying a tracked report.

Keep `CARGO_TARGET_DIR` persistent across rounds. On macOS, a short target path
also keeps the integration tests' Unix socket paths under the system limit.
The runner defaults Cargo build jobs, Rayon threads and Rust test threads to 2;
explicit environment settings take precedence.

The gate needs Cargo, Bash 5, Python, uv, Node with native TypeScript support,
the pi CLI, ffmpeg, xz and Perl. Python dependencies are resolved with uv.
Install Python Playwright's Chromium with
`uv run --with playwright python -m playwright install chromium`. The Node
browser tests use installed Google Chrome and a resolvable `playwright` module;
on the live-stream Mac, set
`NODE_PATH=$HOME/.capy/work/redesign/tools/node_modules` and include
`$HOME/.local/bin` in PATH for pi. Run heavy checks with `nice -n 15` while the
live painter is streaming. No test needs a live studio or a running studio server.
