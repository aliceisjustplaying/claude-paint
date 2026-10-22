"""Refresh mixer-dependent dump fields while retaining the frozen physics bytes."""

import argparse
import hashlib
import json
import lzma
from pathlib import Path
import re
import struct
import sys

sys.dont_write_bytecode = True
from checkpoint_from_dump import rebuild, fnv1a
from state_compare import load, state_files

ap = argparse.ArgumentParser(description=__doc__)
ap.add_argument("--reference", type=Path, required=True)
ap.add_argument("--capture", type=Path, required=True)
ap.add_argument("--package", type=Path, required=True)
args = ap.parse_args()
audit = []
for scene in sorted(args.reference.iterdir()):
    originals = state_files(str(scene))
    captured = state_files(str(args.capture / scene.name))
    assert originals.keys() == captured.keys()
    for chunk, source in originals.items():
        _, frozen = load(source)
        head, fresh = load(captured[chunk])
        fields = dict(frozen)
        changed = []
        for name in ("color", "wet.lat", "studio.piles", "mixer.identity"):
            fields[name] = fresh[name]
            changed.append(name)
        dt, shape, raw = frozen["wet.hide"]
        assert (dt, shape) == fresh["wet.hide"][:2] and shape[-1] == 3
        raw = bytearray(raw)
        for i in range(0, len(raw), 12):
            raw[i:i + 4] = fresh["wet.hide"][2][i:i + 4]
        fields["wet.hide"] = dt, shape, bytes(raw)
        assert fields["wet.hide"][2][4::12] == frozen["wet.hide"][2][4::12]
        changed.append("wet.hide:scattering")
        for name in ("brushes.debug", "knives.debug"):
            if name not in frozen:
                continue
            before = frozen[name][2]
            latents = re.findall(rb"\blat: \[[^\]]*\]", fresh[name][2])
            assert len(latents) == len(re.findall(rb"\blat: \[[^\]]*\]", before))
            iterator = iter(latents)
            raw = re.sub(rb"\blat: \[[^\]]*\]", lambda _: next(iterator), before)
            scattering = re.findall(rb"\bhide: \[([^,\]]+)", fresh[name][2])
            assert len(scattering) == len(re.findall(rb"\bhide: \[([^,\]]+)", raw))
            iterator = iter(scattering)
            raw = re.sub(rb"(\bhide: \[)[^,\]]+", lambda m: m[1] + next(iterator), raw)
            normalize = lambda s: re.sub(rb"(\bhide: \[)[^,\]]+", rb"\1SCATTER", re.sub(rb"\blat: \[[^\]]*\]", b"LATENT", s))
            assert normalize(raw) == normalize(before)
            fields[name] = frozen[name][0], (len(raw),), raw
            changed.append(name + ":latents/scattering")
        before = frozen["studio.debug"][2]
        piles = re.search(rb"piles=Piles \{[^}]*\}", fresh["studio.debug"][2]).group()
        raw = re.sub(rb"piles=Piles \{[^}]*\}", lambda _: piles, before)
        fields["studio.debug"] = frozen["studio.debug"][0], (len(raw),), raw
        changed.append("studio.debug:piles")
        allowed = {"color", "wet.lat", "studio.piles", "mixer.identity", "wet.hide", "brushes.debug", "knives.debug", "studio.debug"}
        for name in frozen.keys() - allowed:
            assert fields[name] == frozen[name], (scene.name, chunk, name)
        for offset in (4, 8):
            assert all(fields["wet.hide"][2][i + offset:i + offset + 4] == frozen["wet.hide"][2][i + offset:i + offset + 4] for i in range(0, len(frozen["wet.hide"][2]), 12))
        digest = "%016x" % fnv1a(rebuild(fields))
        head["digest"] = re.sub(r"canvas=[0-9a-f]{16}", "canvas=" + digest, head["digest"])
        metadata, data = [], bytearray()
        for name, (dtype, shape, raw) in fields.items():
            metadata.append(dict(name=name, dtype=dtype, shape=shape, offset=len(data), nbytes=len(raw)))
            data.extend(raw)
        head["fields"] = metadata
        header = json.dumps(head).encode()
        raw = b"CPSTATE1" + struct.pack("<Q", len(header)) + header + data
        (args.package / "state" / scene.name / (chunk + ".state.xz")).write_bytes(lzma.compress(raw, preset=9 | lzma.PRESET_EXTREME))
        path = args.package / "ref/digest" / (scene.name + ".txt")
        lines = path.read_text().splitlines()
        index = int(chunk.split("-")[1]) - 1
        lines[index] = re.sub(r"canvas=[0-9a-f]{16}", "canvas=" + digest, lines[index])
        path.write_text("\n".join(lines) + "\n")
        audit.append({"scene":scene.name,"chunk":chunk,"refreshed":changed,"all_other_fields_exact":True,"stiffness_and_drying_exact":True,"reference_sha256":hashlib.sha256(Path(source).read_bytes()).hexdigest(),"capture_sha256":hashlib.sha256(Path(captured[chunk]).read_bytes()).hexdigest()})
(args.package / "proof/refresh-field-audit.json").write_text(json.dumps(audit, indent=2) + "\n")
print("Refreshed", len(audit), "dumps; all non-mixer physics bytes retained")
