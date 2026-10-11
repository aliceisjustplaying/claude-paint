#!/usr/bin/env python3
import hashlib
import json
import os
import re
import subprocess
import sys
from pathlib import Path

repo = Path(__file__).resolve().parents[2]
retained = json.loads(Path(sys.argv[1]).read_text())
root = Path(sys.argv[2]).resolve()
root.mkdir(parents=True, exist_ok=True)
binaries = dict(zip(["feature-off", "feature-on-engine5"], map(lambda p: str(Path(p).resolve()), sys.argv[3:5])))
sources = {}
old = repo / "crates/easel/tests/old_logs"
for line in (old / "cases.tsv").read_text().splitlines():
    if line and not line.startswith("#"):
        name, source, width, *_ = line.split("\t")
        sources[name] = (old / source, int(width))
for name, text in {
    "engine5-work-brush-rag": (repo / "crates/easel/tests/thinner/determinism.lua").read_text().replace("--@ engine 3", "--@ engine 5"),
    "engine5-layered-passage": "--@ engine 5\n--@ box impressionist\n--@ chunk 1\n" + (repo / "paintings/sampler/p9_study.lua").read_text(),
}.items():
    source = root / f"{name}.lua"
    source.write_text(text)
    sources[name] = (source, 640)
receipt = {"commit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=repo, text=True).strip(), "baseline": retained["baseline"], "retained_receipt_commit": retained["commit"], "profile": "test (opt-level 2, 256 codegen units)", "binaries": {name: hashlib.sha256(Path(path).read_bytes()).hexdigest() for name, path in binaries.items()}, "cases": []}
for case in retained["cases"]:
    mode, name = case["mode"], case["name"]
    source, width = sources[name]
    source_hash = hashlib.sha256(source.read_bytes()).hexdigest()
    assert source_hash == case["source_sha256"], (name, "source mismatch")
    assert width == case["width"]
    output = root / mode
    output.mkdir(exist_ok=True)
    env = {**os.environ, "EASEL_ROOT": str(output / "runtime"), "RAYON_NUM_THREADS": "2", "EASEL_BRUSH_SIM": "off"}
    env.pop("EASEL_BOX", None)
    png, digest = output / f"{name}.png", output / f"{name}.digests"
    result = subprocess.run([binaries[mode], "run", str(source), "--width", str(width), "--gallery", "--out", str(png), "--state-digest", str(digest)], env=env, cwd=repo, capture_output=True, text=True, check=True)
    (output / f"{name}.stdout").write_text(result.stdout)
    (output / f"{name}.stderr").write_text(result.stderr)
    state = re.sub(r" secs=[^ \n]+", "", digest.read_text())
    got = {"png_sha256": hashlib.sha256(png.read_bytes()).hexdigest(), "digest_sha256": hashlib.sha256(state.encode()).hexdigest(), "chunks": len(state.splitlines()), "stdout_sha256": hashlib.sha256(result.stdout.encode()).hexdigest()}
    assert got == case["matching_outputs"], (mode, name, got, case["matching_outputs"])
    receipt["cases"].append({"mode": mode, "name": name, "source_sha256": source_hash, "width": width, "matching_outputs": got})
    (root / "default-off-proof.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(mode, name, "byte-identical PNG, stdout and normalized chunk state", flush=True)
receipt["result"] = "all 26 comparisons byte-identical"
(root / "default-off-proof.json").write_text(json.dumps(receipt, indent=2) + "\n")
