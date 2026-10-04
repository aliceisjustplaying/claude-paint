#!/usr/bin/env python3
"""Checks that state_compare.py finds what it must and passes what it may.

    test_state_compare.py

Starts from a real baseline dump (state/rag/chunk-008.state.xz: a wet canvas
after waits, one brush and two rags) and writes altered copies to a
temporary directory: a value changed in a canvas field, 0.0 turned to -0.0,
a field removed, fields and Debug values added (zero and nonzero, as a later
engine's solvent would be), a bristle's load changed, a rag's dampness
changed, a missing chunk, a file that isn't a dump. Exit 0 when every case
gives the expected verdict. Takes about a second.
"""

import json
import lzma
import os
import shutil
import struct
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
sys.dont_write_bytecode = True  # no __pycache__ in the package
sys.path.insert(0, HERE)
from state_compare import load  # noqa: E402

SRC = os.path.join(HERE, "..", "state", "rag", "chunk-008.state.xz")
TOOL = os.path.join(HERE, "state_compare.py")


def write(path, head, fields):
    """fields: ordered list of (name, dtype, shape, raw bytes)."""
    meta, off = [], 0
    for name, dt, shape, raw in fields:
        meta.append({"name": name, "dtype": dt, "shape": list(shape), "offset": off, "nbytes": len(raw)})
        off += len(raw)
    h = dict(head, fields=meta)
    hb = (json.dumps(h) + "\n").encode()
    with open(path, "wb") as f:
        f.write(b"CPSTATE1" + struct.pack("<Q", len(hb)) + hb + b"".join(r for *_, r in fields))


def base():
    head, fields = load(SRC)
    return head, [(n, dt, shape, raw) for n, (dt, shape, raw) in fields.items()]


def replace(fields, name, fn):
    return [(n, dt, s, fn(r) if n == name else r) for n, dt, s, r in fields]


def text_edit(fields, name, old, new, count=1):
    def fn(raw):
        t = raw.decode()
        assert old in t, (name, old)
        return t.replace(old, new, count).encode()
    out = []
    for n, dt, s, r in fields:
        if n == name:
            r = fn(r)
            s = (len(r),)
        out.append((n, dt, s, r))
    return out


def run(a, b, *flags):
    p = subprocess.run([sys.executable, TOOL, a, b, *flags], capture_output=True, text=True)
    return p.returncode, p.stdout + p.stderr


def main():
    tmp = tempfile.mkdtemp(prefix="state-compare-test.")
    failures = []
    try:
        head, fields = base()
        ref = os.path.join(tmp, "ref.state")
        write(ref, head, fields)

        def case(label, fields2, want, *flags, expect_text=None):
            p = os.path.join(tmp, label.replace(" ", "_") + ".state")
            write(p, head, fields2)
            code, out = run(ref, p, *flags)
            ok = code == want and (expect_text is None or expect_text in out)
            print("%s %s (exit %d, wanted %d)" % ("ok  " if ok else "FAIL", label, code, want))
            if not ok:
                failures.append(label)
                print(out)

        # the dump is the same file compressed or not
        code, out = run(SRC, ref)
        print("%s compressed baseline vs its own bytes (exit %d)" % ("ok  " if code == 0 else "FAIL", code))
        if code:
            failures.append("self")

        vol = [r for n, _, _, r in fields if n == "wet.vol"][0]
        n = len(vol) // 4
        vals = list(struct.unpack("<%df" % n, vol))
        i = max(range(n), key=lambda k: vals[k])  # a wet pixel
        bumped = vals[:]
        bumped[i] = struct.unpack("<f", struct.pack("<I", struct.unpack("<I", struct.pack("<f", vals[i]))[0] + 1))[0]
        case("one wet volume one ulp off", replace(fields, "wet.vol", lambda r: struct.pack("<%df" % n, *bumped)), 1, expect_text="wet.vol: 1 of")
        z = vals.index(0.0)
        neg = vals[:]
        neg[z] = -0.0
        case("0.0 turned to -0.0", replace(fields, "wet.vol", lambda r: struct.pack("<%df" % n, *neg)), 1, expect_text="wet.vol")
        case("a field removed", [f for f in fields if f[0] != "clock.cure"], 1, expect_text="clock.cure: missing")
        npx = n
        zero = struct.pack("<%df" % npx, *([0.0] * npx))
        some = struct.pack("<%df" % npx, *([0.0] * (npx - 1) + [0.5]))
        shape = [s for nm, _, s, _ in fields if nm == "wet.vol"][0]
        case("an added zero field", fields + [("wet.solvent", "f32", shape, zero)], 0)
        case("an added zero field, --added-zero", fields + [("wet.solvent", "f32", shape, zero)], 0, "--added-zero")
        case("an added nonzero field", fields + [("wet.solvent", "f32", shape, some)], 0)
        case("an added nonzero field, --added-zero", fields + [("wet.solvent", "f32", shape, some)], 1, "--added-zero", expect_text="wet.solvent: added field has 1 nonzero")
        # Debug text: a later Bristle with a solvent field
        b0 = [r for nm, _, _, r in fields if nm == "brushes.debug"][0].decode()
        cure = b0[b0.index("cure: "):b0.index(" }", b0.index("cure: "))]
        with_solvent = text_edit(fields, "brushes.debug", cure + " }", cure + ", solvent: 0.0 }", count=-1)
        case("bristles gain solvent 0.0", with_solvent, 0)
        case("bristles gain solvent 0.0, --added-zero", with_solvent, 0, "--added-zero")
        case("a bristle gains solvent 0.25, --added-zero",
             text_edit(fields, "brushes.debug", cure + " }", cure + ", solvent: 0.25 }"), 1, "--added-zero", expect_text="solvent is not zero")
        vol0 = b0[b0.index("vol: "):b0.index(",", b0.index("vol: "))]
        case("a bristle's load changed", text_edit(fields, "brushes.debug", vol0 + ",", vol0 + "1,"), 1, expect_text="brushes[0].bristles[0].vol")
        r0 = [r for nm, _, _, r in fields if nm == "rags.debug"][0].decode()
        damp = r0[r0.index("damp: ", r0.index("Rag {", 1)):].split(",")[0]
        case("a rag's dampness changed", text_edit(fields, "rags.debug", damp, "damp: 0.25"), 1, expect_text="rags[1].damp")
        st = [r for nm, _, _, r in fields if nm == "studio.debug"][0].decode()
        clock = st.split()[1]
        case("the studio clock changed", text_edit(fields, "studio.debug", clock, "clock=1.5"), 1, expect_text="studio.debug: clock")

        # directories: a chunk missing from the new side
        da, db = os.path.join(tmp, "a"), os.path.join(tmp, "b")
        os.makedirs(da)
        os.makedirs(db)
        shutil.copy(ref, os.path.join(da, "chunk-001.state"))
        shutil.copy(ref, os.path.join(da, "chunk-002.state"))
        with open(os.path.join(db, "chunk-001.state.xz"), "wb") as f:
            f.write(lzma.compress(open(ref, "rb").read()))
        code, out = run(da, db)
        ok = code == 1 and "chunk-002: missing" in out
        print("%s directories: a missing chunk fails (exit %d)" % ("ok  " if ok else "FAIL", code))
        if not ok:
            failures.append("dir")
            print(out)
        junk = os.path.join(tmp, "junk.state")
        with open(junk, "wb") as f:
            f.write(b"PAINTCK8" + bytes(64))
        code, out = run(ref, junk)
        ok = code == 2
        print("%s not a dump: exit 2 (exit %d)" % ("ok  " if ok else "FAIL", code))
        if not ok:
            failures.append("junk")
    finally:
        shutil.rmtree(tmp)
    print("test_state_compare: %s" % ("all cases passed" if not failures else "FAILED: " + ", ".join(failures)))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
