#!/usr/bin/env python3
"""Self-test of scripts/thinner_dump_fields.py on tiny fake CPSTATE1 dumps
(the format in notes/thinner/baseline/README.md), no easel needed:

- all fields present, solvent nonzero  -> 0, and 0 with --nonzero
- all fields present, solvent all zero -> 0, but 1 with --nonzero
- no wet.solvent / a bristle without solvent / a rag without solvent_mm3 -> 1
- wet.solvent with the wrong shape     -> 1

    scripts/tests/thinner_dump_fields_test.py
"""
import json
import os
import struct
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
TOOL = os.path.join(HERE, "..", "thinner_dump_fields.py")


def dump(path, fields):
    data, head = b"", []
    for name, dtype, shape, raw in fields:
        head.append({"name": name, "dtype": dtype, "shape": shape, "offset": len(data), "nbytes": len(raw)})
        data += raw
    h = json.dumps({"format": "CPSTATE1", "chunk": 1, "digest": "", "fields": head}).encode()
    with open(path, "wb") as f:
        f.write(b"CPSTATE1" + struct.pack("<Q", len(h)) + h + data)


def f32s(v):
    return struct.pack("<%df" % len(v), *v)


def scene(d, solvent=0.5, bristle="solvent: 0.25", rag="solvent_mm3: 0.125", shape=(2, 2, 1), with_field=True):
    os.makedirs(d)
    fields = [("wet.vol", "f32", [2, 2, 1], f32s([1.0] * 4))]
    if with_field:
        n = shape[0] * shape[1] * shape[2]
        fields.append(("wet.solvent", "f32", list(shape), f32s([solvent] + [0.0] * (n - 1))))
    brush = "Held { tool: Tool { width: 1.0 }, bristles: [Bristle { vol: 1.0, %s }, Bristle { vol: 0.5, %s }] }" % (bristle, bristle)
    cloth = "Rag { width: 1.0, load: 0.5, damp: 0.0, %s }" % rag
    fields.append(("brushes.debug", "utf8", [len(brush)], brush.encode()))
    fields.append(("rags.debug", "utf8", [len(cloth)], cloth.encode()))
    dump(os.path.join(d, "chunk-001.state"), fields)
    return d


def code(*args):
    return subprocess.run([sys.executable, TOOL, *args], capture_output=True, text=True).returncode


fails = 0
with tempfile.TemporaryDirectory() as t:
    cases = [
        ("solvent nonzero", scene(os.path.join(t, "a")), 0, 0),
        ("solvent all zero", scene(os.path.join(t, "b"), solvent=0.0, bristle="solvent: 0.0", rag="solvent_mm3: 0.0"), 0, 1),
        ("no wet.solvent", scene(os.path.join(t, "c"), with_field=False), 1, 1),
        ("a bristle without solvent", scene(os.path.join(t, "d"), bristle="cure: 0.0"), 1, 1),
        ("a rag without solvent_mm3", scene(os.path.join(t, "e"), rag="fold: 0"), 1, 1),
        ("wet.solvent the wrong shape", scene(os.path.join(t, "f"), shape=(4, 1, 1)), 1, 1),
    ]
    for name, d, plain, nz in cases:
        got = (code(d), code("--nonzero", d))
        ok = got == (plain, nz)
        fails += not ok
        print("%s %-28s exit %d, --nonzero %d (want %d, %d)" % ("ok:  " if ok else "FAIL:", name, got[0], got[1], plain, nz))
print("thinner_dump_fields_test: %s" % ("all cases pass" if not fails else "%d cases FAILED" % fails))
sys.exit(1 if fails else 0)
