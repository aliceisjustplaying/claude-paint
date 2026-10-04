#!/usr/bin/env python3
"""Thinner check 2, the field half: the new engine's state dumps declare
the solvent, so `--added-zero` has something to hold to zero.

    thinner_dump_fields.py <dump dir>...

Each dir holds `easel run --dump-state` output (chunk-NNN.state). Every
chunk must have the canvas field `wet.solvent`, dtype f32, the shape of
`wet.vol` ([h, w, 1]): the solvent per pixel, µm. In `brushes.debug` every
bristle that has a `vol` must have a `solvent`; in `rags.debug` every rag
must have a `solvent_mm3`. Across all the dirs at least one chunk must hold
a brush and one a rag, so neither half is checked vacuously. Exit 0 if all
holds, 1 if not, 2 on a usage error. Reads the dumps with the baseline's
own reader (notes/thinner/baseline/tools/state_compare.py, unchanged).
Standard library only.
"""

import glob
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "notes", "thinner", "baseline", "tools"))
import state_compare as sc  # noqa: E402

VOL = re.compile(r"^brushes\[(\d+)\]\.bristles\[(\d+)\]\.vol$")
RAG = re.compile(r"^rags\[(\d+)\]\.damp$")


def check(path, seen):
    problems = []
    _, fields = sc.load(path)
    if "wet.solvent" not in fields:
        problems.append("no field wet.solvent")
    else:
        dt, shape, _ = fields["wet.solvent"]
        want = fields["wet.vol"][1]
        if dt != "f32" or shape != want:
            problems.append("wet.solvent is %s %s; want f32 %s (the shape of wet.vol)" % (dt, list(shape), list(want)))
    for name, pat, key in (("brushes.debug", VOL, "solvent"), ("rags.debug", RAG, "solvent_mm3")):
        if name not in fields:
            problems.append("no field %s" % name)
            continue
        text = fields[name][2].decode("utf-8")
        vals = sc.debug_values(name, text)
        for k in vals:
            m = pat.match(k)
            if not m:
                continue
            seen[name] += 1
            other = k[: k.rindex(".") + 1] + key
            if other not in vals:
                problems.append("%s has no %s" % (k.rsplit(".", 1)[0], key))
    return problems


def main():
    if len(sys.argv) < 2:
        print(__doc__.strip().split("\n\n")[1], file=sys.stderr)
        return 2
    seen = {"brushes.debug": 0, "rags.debug": 0}
    bad = 0
    files = 0
    for d in sys.argv[1:]:
        for path in sorted(glob.glob(os.path.join(d, "chunk-*.state*"))):
            files += 1
            for p in check(path, seen):
                bad += 1
                print("%s: %s" % (path, p))
    if files == 0:
        print("thinner_dump_fields: no chunk-*.state files in %s" % " ".join(sys.argv[1:]))
        return 1
    if seen["brushes.debug"] == 0 or seen["rags.debug"] == 0:
        print("thinner_dump_fields: no chunk held a brush (%d bristles seen) or a rag (%d seen): nothing to check" % (seen["brushes.debug"], seen["rags.debug"]))
        return 1
    if bad:
        print("thinner_dump_fields: %d problems in %d chunks" % (bad, files))
        return 1
    print("thinner_dump_fields: %d chunks declare the solvent (%d bristle values, %d rag values)" % (files, seen["brushes.debug"], seen["rags.debug"]))
    return 0


if __name__ == "__main__":
    sys.exit(main())
