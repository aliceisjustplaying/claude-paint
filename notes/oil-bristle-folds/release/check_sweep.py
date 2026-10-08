#!/usr/bin/env python3
"""Verify the small release study against saved evidence, then write its page."""
import csv
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import time

out = Path(__file__).resolve().parent
old = out.parent
root = out.parents[2]
assert os.environ.get("LOCKRUN_TOKEN"), "run through scripts/lockrun"
start = time.monotonic()


def rows(path):
    with path.open() as f:
        return [{k: v for k, v in r.items() if k != "seconds"} for r in csv.DictReader(f)]


metrics = rows(out / "metrics.csv")
assert len(metrics) == 32
files = sorted(out.glob("*-lit.png")) + sorted(out.glob("*-flat.png")) + sorted(out.glob("*-wet-height.png"))
assert len(files) == 96
# Verify every original evidence file against the approved report commit.
original_commit = "78b3dd5e"
evidence_paths = subprocess.check_output(["git", "ls-tree", "-r", "--name-only", original_commit, "notes/oil-bristle-folds"], cwd=root, text=True).splitlines()
manifest = []
for path in evidence_paths:
    saved = subprocess.check_output(["git", "show", f"{original_commit}:{path}"], cwd=root)
    assert (root / path).read_bytes() == saved, path
    manifest.append(f"{hashlib.sha256(saved).hexdigest()}  {path}\n")
(out / "original-evidence.sha256").write_text("".join(manifest))

originals = [f for f in files if "-candidate-" in f.name or "-baseline-" in f.name]
assert len(originals) == 48
for f in originals:
    assert f.read_bytes() == (old / f.name).read_bytes(), f.name
assert [r for r in metrics if r["mode"] in ["baseline", "candidate"]] == rows(old / "metrics.csv")
for threads in [1, 4]:
    with tempfile.TemporaryDirectory(prefix="fold-release-repeat-") as tmp:
        dest = Path(tmp)
        with (dest / "metrics.csv").open("w") as stdout:
            subprocess.run([str(root / "target/iter/examples/bristle_folds"), tmp, "--release-sweep"],
                           cwd=root, env=dict(os.environ, RAYON_NUM_THREADS=str(threads)),
                           stdout=stdout, check=True, timeout=60)
        assert rows(dest / "metrics.csv") == metrics
        assert (dest / "drying.csv").read_bytes() == (out / "drying.csv").read_bytes()
        for f in files:
            assert f.read_bytes() == (dest / f.name).read_bytes(), (threads, f.name)

lookup = {(r["scene"], r["width"], r["mode"]): r for r in metrics}
comparisons = []
for r in metrics:
    r = dict(r)
    for field in ["canvas", "brushes"]:
        for mode, label in [("candidate", "1.0"), ("baseline", "baseline")]:
            reference = float(lookup[(r["scene"], r["width"], mode)][field])
            r[f"{field}_pct_of_{label}"] = f"{100 * float(r[field]) / reference:.6f}"
    comparisons.append(r)
with (out / "comparison.csv").open("w") as f:
    writer = csv.DictWriter(f, fieldnames=list(comparisons[0]), lineterminator="\n")
    writer.writeheader()
    writer.writerows(comparisons)

proof = dict(original_evidence_commit=original_commit, original_evidence_files_exact=len(evidence_paths),
             original_candidate_pngs_exact=24, original_baseline_pngs_exact=24,
             original_budget_rows_exact=16, repeat_threads=[1, 4], repeat_pngs_exact_per_run=96,
             repeat_budget_rows_exact_per_run=32, repeat_drying_rows_exact_per_run=32,
             max_abs_relative_mass_error=max(abs((float(r["canvas"]) + float(r["brushes"])) / float(r["initial"]) - 1) for r in metrics),
             seconds=round(time.monotonic() - start, 3))
(out / "proof.json").write_text(json.dumps(proof, indent=2) + "\n")

# A separate gallery: never rewrite the parent gallery or its evidence.
html = '''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Bristle folds: release comparison</title><style>
body{font:15px system-ui,sans-serif;background:#eee9e3;color:#242321;max-width:1360px;margin:24px auto;padding:0 16px}h1{font-size:24px}h2{font-size:18px}p{line-height:1.5;max-width:900px}select{font:inherit;margin:4px 16px 12px 0}section{margin:28px 0}.scroll{overflow-x:auto}.marks{display:grid;grid-template-columns:repeat(4,var(--w));gap:16px}figure{margin:0}img{display:block;width:var(--w);height:auto}figcaption{font-size:13px;padding:6px 0}table{border-collapse:collapse;font-size:13px;margin-top:8px;min-width:1000px}th,td{text-align:right;padding:7px 10px;border-bottom:1px solid #c6bfb6}th:first-child,td:first-child{text-align:left}a{color:#38546c}
</style><h1>Same bristle fields, less overall release</h1>
<p>Experiment only, not a default change. Three candidate release multipliers: <b>1.0×, 0.9× and 0.8×</b>, with ordinary engine 5 for reference. Geometry inputs, seeds, starting loads and lighting stay identical. Paint-dependent contact and pickup can change as the brush retains more paint.</p>
<p>All images display at native resolution. Current lighting: azimuth 315°, elevation 30°, strength 0.9. Each canvas is 100 × 50 mm. Wet-height view clips above 250 µm, so white peaks are not equal-height peaks. These are simulated amounts, not calibrated physical measurements.</p>
<label>Resolution <select id="resolution"><option>320</option><option>160</option></select></label>
<label>View <select id="view"><option value="lit">Current lighting</option><option value="flat">Flat color</option><option value="wet-height">Wet height (0–250 µm)</option></select></label>
<main id="scenes"></main>
<p>Amounts are net paint remaining on the canvas and total paint remaining in the brush(es), in coats × square canvas units. Crossing includes both brushes and the dirty continuation; canvas amount is not cumulative gross release before pickup. Percentages are actual amounts divided by the saved 1.0× candidate or original baseline, not the release coefficient.</p>
<p>Nearly spent means the same initial load wiped to 4%. No scene is larger than 320 px. <a href="REPORT.md">Short report</a> · <a href="comparison.csv">All measured amounts and percentages</a> · <a href="proof.json">Exact-image and repeat checks</a> · <a href="../index.html">Unchanged original gallery</a></p>
<script>const data=__DATA__;
const names={loaded:'Loaded',spent:'Nearly spent',curve:'Curved / varying pressure',crossing:'Wet crossing / dirty continuation'};
const modes=[['baseline','Original baseline'],['candidate','Candidate 1.0×'],['release-0.9','Candidate 0.9×'],['release-0.8','Candidate 0.8×']];
const resolution=document.querySelector('#resolution'),view=document.querySelector('#view');
const amount=v=>Number(v).toLocaleString('en-US',{minimumFractionDigits:3,maximumFractionDigits:3});
const pct=v=>Number(v).toFixed(2)+'%';
function render(){document.querySelector('#scenes').innerHTML=Object.entries(names).map(([key,label])=>{
const rows=modes.map(([mode,title])=>{const r=data.find(r=>r.scene===key&&r.width===resolution.value&&r.mode===mode);return `<tr><td>${title}</td><td>${amount(r.canvas)}</td><td>${pct(r['canvas_pct_of_1.0'])} / ${pct(r.canvas_pct_of_baseline)}</td><td>${amount(r.brushes)}</td><td>${pct(r['brushes_pct_of_1.0'])} / ${pct(r.brushes_pct_of_baseline)}</td><td>${amount(r.max_wet_um)}</td></tr>`}).join('');
return `<section><h2>${label}</h2><div class="scroll"><div class="marks" style="--w:${resolution.value}px">${modes.map(([mode,title])=>`<figure><figcaption>${title}</figcaption><img alt="${label}: ${title}" src="${key}-${resolution.value}-${mode}-${view.value}.png"></figure>`).join('')}</div><table><thead><tr><th>Mode</th><th>Net canvas paint</th><th>% of 1.0× / baseline</th><th>Brush paint left</th><th>% of 1.0× / baseline</th><th>Peak wet µm</th></tr></thead><tbody>${rows}</tbody></table></div></section>`}).join('')}
resolution.onchange=view.onchange=render;render();</script></html>'''
(out / "index.html").write_text(html.replace("__DATA__", json.dumps(comparisons)))
print(json.dumps(proof))
