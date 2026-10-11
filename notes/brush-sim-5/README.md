# Iteration-five offline tools

These Python tools operate on the shared cloud checkout at `/home/user/.capy/work/brush-sim-5/repo`. The Captain owns Rust, builds, painting replays, gates, commits and Drive publication. Outputs and checkpoints stay under `/home/user/.capy/work/brush-sim-5/`, outside the checkout. The tools reject Drive destinations.

## Commands

Run from the actual repository root. The existing scratch virtual environment supplies NumPy and Pillow. None of these preparation, analysis or publication commands launches a painter:

```sh
cd /home/user/.capy/work/brush-sim-5/repo
PY=../venv/bin/python
$PY notes/brush-sim-5/morning.py prepare ../calibration
OPENBLAS_NUM_THREADS=1 $PY notes/brush-sim-5/mesh.py \
  ../control-lean4/surface.csv ../calibration/mesh-analysis.json \
  --geometry-only ../control-lean4/geometry-only.csv
$PY notes/brush-sim-5/publish.py ../calibration ../publication
```

`prepare` is idempotent and verifies both retained controls. Analysis and publication refuse to overwrite existing output. Choose a fresh scratch output path to repeat them. Full publication requires all three matched variants. `--controls-only` is an explicitly labeled tool-verification preview with just baseline and lean4. `--extra-crop end-edge:620,100,820,380` adds a native crop and paired comparison from existing PNGs. Crop coordinates are half-open. No image filters, gamma conversion or resizing occur.

Only the following command launches replays. It is for the Captain to execute once the exact final binary and engine sources are ready:

```sh
$PY notes/brush-sim-5/morning.py calibrate /absolute/path/to/exact-binary ../calibration
```

The same command resumes safely. Each engine/factor candidate gets one invocation, persistent Lua, logs, a process-result marker, material CSV, paired native views, digests and final Canvas. Selection references that exact candidate directory and preserves its checkpoint. Accepted candidates are verified and skipped on subsequent runs. There is no matched-candidate rendering replay. Candidate factors start at 1.546, bracket up to 8 and bisect with at most 22 candidates per preset; `--max-candidates` can extend the search to 40. The target is fixed at 7,249,498.888835084 and tolerance at 0.03%. Matching uses whole final `material_total` minus the primed initial, not density-weighted mass. Each requested load receives the same factor and repeated finite dips at most one.

Failures or interrupted invocations are never automatically repainted. A successful process-result marker permits finishing interrupted receipt processing from existing outputs. A missing marker is explicitly uncertain, so the harness stops for inspection. The invocation count in that state is a reserved-or-started count, labeled uncertain; completed receipts confirm exactly one actual process invocation. Do not erase a candidate directory to force a rerun. Resume refuses changed binary, engine-source or Lua-template hashes; notes-only commits do not invalidate a run. An exclusive file lock prevents two calibration processes on the same root.

## Controls and provenance

Baseline is imported byte-for-byte from Captain Drive `renders/morning/brush/iteration-4-winddown-baseline-*` PNG/CSV/Lua/digests into scratch `calibration/controls/baseline`. The retained receipt supplies engine 5, factor 1, binary/Lua hashes and its original one invocation. The original invocation command was not retained and is recorded as unknown. Its checkpoint is absent and `final_canvas` is null. No baseline replay occurs.

Lean4 points to the existing `control-lean4` directory: engine 15, factor 1.546 and exactly one original invocation. Its receipt, exact binary, Lua, material report, paired PNGs, digests and scratch checkpoint are verified. Python float subtraction of the actual CSV yields whole deposited volume 7,249,518.806509474, error +19.917674391 coats times square canvas units or +0.000274746%. This differs from the supplied volume by about 1e-9, with no practical effect on tolerance. This control is never repainted or duplicated as a new candidate.

New engines are lean5 18, body5 19 and wild5 20. Candidate receipts contain the actual command, working directory, environment overrides, removed `EASEL_BOX`, binary/Lua hashes, exact tracked runtime source/build/Cargo hashes, load factor, measured initial/final volume, actual error, invocation count and artifact hashes. Runtime hashing includes the embedded easel Lua files and excludes sparse archived paintings, tests and examples. Baseline and lean4 retain separate control provenance and binary identities.

## Diagnostic mesh analysis

`mesh.py` reads every actual CSV pixel exactly once in native dark-wall ROI `(160,180)-(680,460)`. It summarizes dry support, wet paint, solvent, visible surface and linear Color/Gallery Y. The native pitch is 160/1200 mm; 16 threads/cm gives 0.2133333 cycles/pixel and alternating weave repeats give 0.1066667. A least-squares plane is removed from each numerical field, then Hann-window FFT energy in x/y frequency bands gives RMS with window-energy correction. Default band half-width is 0.012 cycles/pixel. Per-frequency and union-band values are reported alongside raw and plane-removed Pearson correlation with dry support. Constant-field correlation is null. This is diagnostic analysis, not image filtering or a causal texture score.

The separate narrow woven-lattice diagnostic uses rectangular spots at `|fx|=|fy|=0.1066667` plus full-thread spots at `|fx|=0.2133333, |fy|<0.009` and vice versa, including all conjugate signs. Every spot has half-width 0.009 cycles/pixel. It subtracts the mean, uses an unwindowed FFT with no plane removal and inverse-transforms the spot-mask projection of each numerical CSV field. It reports projected-field RMS and Pearson correlation against equally projected dry support. This does not edit or filter images. The actual lean4 control gives visible-surface spot RMS 6.292965 µm, projected support correlation 0.952124 and wet-paint spot RMS 1.932500 µm. That correlation describes woven-frequency fields; it is separate from raw ROI correlation and the broader axis-band metrics.

Geometry-only is an engine-18 relight of the SAME saved lean4 Canvas, not a brush replay. The analysis requires dry support, wet paint, solvent and Color Y to match exactly between CSVs, records the checkpoint/CSV hashes and reports changed visible-surface/Gallery diagnostics. These numerical checks complement the Captain's supplied provenance; they do not reconstruct a missing relight command.

## Native publication and checks

Publication adds only `iteration-5-*` files in a fresh scratch staging directory. It includes paired full studies, paired 1:1 dark-wall crops, Gallery/Color sheets, wild5 higher-entropy options, original CSV/Lua/digests, scalar material/volume summaries, receipts, diagnostic JSON when present, a README and SHA256SUMS. Every pasted crop is compared byte-for-byte before and after PNG encoding. No checkpoint is copied and every artifact must be at most 100 MB. The Captain can later copy the intended files beside existing morning/brush files using an exclusive no-overwrite copy after approval. The Python tools themselves do not write to Drive.

Focused verification uses explicitly labeled synthetic fixtures and mocked process launches only:

```sh
PYTHONPYCACHEPREFIX=/home/user/.capy/work/brush-sim-5/python-cache \
  $PY -m py_compile notes/brush-sim-5/*.py
OPENBLAS_NUM_THREADS=1 PYTHONDONTWRITEBYTECODE=1 \
  $PY notes/brush-sim-5/test_tools.py -v
cd ../publication && sha256sum -c iteration-5-SHA256SUMS
```

Synthetic PNGs, CSVs and non-Canvas mock checkpoint bytes stay in temporary directories labeled `synthetic-tools-*`, then are deleted. Tests never invoke easel and never fabricate real preset outcomes.
