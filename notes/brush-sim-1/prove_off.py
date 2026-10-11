#!/usr/bin/env python3
import hashlib
import json
import os
import re
import subprocess
import sys
from pathlib import Path

repo = Path(__file__).resolve().parents[2]
root = Path(sys.argv[1]).resolve()
binaries = dict(zip(["origin-main", "feature-off", "feature-on-engine5"], map(lambda p: str(Path(p).resolve()), sys.argv[2:5])))
assert len(binaries) == 3
root.mkdir(parents=True, exist_ok=True)
cases = []
old = repo / "crates/easel/tests/old_logs"
for row in (old / "cases.tsv").read_text().splitlines():
    if row and not row.startswith("#"):
        name, source, width, *_ = row.split("\t")
        cases.append((name, old / source, int(width)))
source = root / "engine5.lua"
source.write_text((repo / "crates/easel/tests/thinner/determinism.lua").read_text().replace("--@ engine 3", "--@ engine 5"))
cases.append(("engine5-work-brush-rag", source, 640))
sampler = root / "sampler.lua"
sampler.write_text("--@ box giverny\n--@ engine 5\n" + "".join(f"--@ chunk {i}\n" + (repo / "paintings/sampler" / name).read_text() + "\n" for i, name in enumerate(["p9_study.lua", "p9_study2.lua", "p9_study3.lua"], 1)))
cases.append(("engine5-layered-passage", sampler, 640))
receipt = {"baseline": subprocess.check_output(["git", "rev-parse", "origin/main"], cwd=repo, text=True).strip(), "binaries": {name: hashlib.sha256(Path(p).read_bytes()).hexdigest() for name, p in binaries.items()}, "cases": []}
for name, source, width in cases:
    got = {}
    for mode, binary in binaries.items():
        out = root / mode
        out.mkdir(exist_ok=True)
        env = {**os.environ, "EASEL_ROOT": str(out / "runtime"), "RAYON_NUM_THREADS": "2", "EASEL_BRUSH_SIM": "off"}
        env.pop("EASEL_BOX", None)
        png, dig = out / f"{name}.png", out / f"{name}.digests"
        result = subprocess.run(["nice", "-n", "15", binary, "run", str(source), "--width", str(width), "--gallery", "--out", str(png), "--state-digest", str(dig)], env=env, cwd=repo, capture_output=True, text=True)
        (out / f"{name}.stdout").write_text(result.stdout)
        (out / f"{name}.stderr").write_text(result.stderr)
        result.check_returncode()
        state = re.sub(r" secs=[^ \n]+", "", dig.read_text())
        got[mode] = dict(png_sha256=hashlib.sha256(png.read_bytes()).hexdigest(), digest_sha256=hashlib.sha256(state.encode()).hexdigest(), chunks=len(state.splitlines()), stdout_sha256=hashlib.sha256(result.stdout.encode()).hexdigest())
    assert len({json.dumps(v, sort_keys=True) for v in got.values()}) == 1, (name, got)
    receipt["cases"].append(dict(name=name, width=width, source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(), matching_outputs=got))
    print(name, "byte-identical PNG, stdout and chunk state")
receipt["result"] = "all byte-identical"
(root / "default-off-proof.json").write_text(json.dumps(receipt, indent=2) + "\n")
