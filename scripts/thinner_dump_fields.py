#!/usr/bin/env python3
"""Thinner check 2, the field half: the new engine's state dumps declare
the solvent, so `--added-zero` has something to hold to zero.

    thinner_dump_fields.py [--nonzero] <dump dir>...

Each dir holds `easel run --dump-state` output (chunk-NNN.state). Every
chunk must have the canvas field `wet.solvent`, dtype f32, the shape of
`wet.vol` ([h, w, 1]): the solvent per pixel, µm. In `brushes.debug` every
bristle that has a `vol` must have a `solvent`; in `rags.debug` every rag
must have a `solvent_mm3`. Across all the dirs at least one chunk must hold
a brush and one a rag, so neither half is checked vacuously. Exit 0 if all
holds, 1 if not, 2 on a usage error.

With --nonzero (the positive control, on a thinned scene): somewhere in
the dumps `wet.solvent` must hold a nonzero value, some bristle a nonzero
`solvent` and some rag a nonzero `solvent_mm3`, so a dumper that writes
the fields but always zero can't pass. Reads the dumps with the baseline's
own reader (notes/thinner/baseline/tools/state_compare.py, unchanged).
Standard library only.
"""

import glob
import os
import re
import struct
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "notes", "thinner", "baseline", "tools"))
import state_compare as sc  # noqa: E402

VOL = re.compile(r"^brushes\[(\d+)\]\.bristles\[(\d+)\]\.vol$")
RAG = re.compile(r"^rags\[(\d+)\]\.damp$")


def nonzero(node):
    return node[0] == "num" and sc.num(node[1]) != 0.0


def check(path, seen):
    problems = []
    _, fields = sc.load(path)
    if "wet.solvent" not in fields:
        problems.append("no field wet.solvent")
    else:
        dt, shape, raw = fields["wet.solvent"]
        want = fields["wet.vol"][1]
        if dt != "f32" or shape != want:
            problems.append("wet.solvent is %s %s; want f32 %s (the shape of wet.vol)" % (dt, list(shape), list(want)))
        elif any(v != 0.0 for v in struct.unpack("<%df" % (len(raw) // 4), raw)):
            seen["wet.solvent nonzero"] += 1
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
            elif nonzero(vals[other]):
                seen[name + " nonzero"] += 1
    return problems


def main():
    args = sys.argv[1:]
    want_nonzero = bool(args) and args[0] == "--nonzero"
    if want_nonzero:
        args = args[1:]
    if not args:
        print(__doc__.strip().split("\n\n")[1], file=sys.stderr)
        return 2
    seen = {"brushes.debug": 0, "rags.debug": 0, "wet.solvent nonzero": 0, "brushes.debug nonzero": 0, "rags.debug nonzero": 0}
    bad = 0
    files = 0
    for d in args:
        for path in sorted(glob.glob(os.path.join(d, "chunk-*.state*"))):
            files += 1
            for p in check(path, seen):
                bad += 1
                print("%s: %s" % (path, p))
    if files == 0:
        print("thinner_dump_fields: no chunk-*.state files in %s" % " ".join(args))
        return 1
    if seen["brushes.debug"] == 0 or seen["rags.debug"] == 0:
        print("thinner_dump_fields: no chunk held a brush (%d bristles seen) or a rag (%d seen): nothing to check" % (seen["brushes.debug"], seen["rags.debug"]))
        return 1
    if bad:
        print("thinner_dump_fields: %d problems in %d chunks" % (bad, files))
        return 1
    if want_nonzero:
        zero = [k for k in ("wet.solvent nonzero", "brushes.debug nonzero", "rags.debug nonzero") if seen[k] == 0]
        if zero:
            print("thinner_dump_fields: the thinned scene's dumps show no solvent in: %s" % ", ".join(z.replace(" nonzero", "") for z in zero))
            return 1
        print("thinner_dump_fields: solvent shown: %d chunks of wet.solvent, %d bristle values, %d rag values nonzero" % (seen["wet.solvent nonzero"], seen["brushes.debug nonzero"], seen["rags.debug nonzero"]))
    print("thinner_dump_fields: %d chunks declare the solvent (%d bristle values, %d rag values)" % (files, seen["brushes.debug"], seen["rags.debug"]))
    return 0


if __name__ == "__main__":
    sys.exit(main())
