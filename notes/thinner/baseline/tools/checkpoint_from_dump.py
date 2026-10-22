#!/usr/bin/env python3
"""Rebuild an engine-3 PAINTC19/base-9 checkpoint from a state dump.

    checkpoint_from_dump.py chunk-NNN.state[.xz] [--digest DIGEST_FILE]

Prints the FNV-1a-64 of the rebuilt bytes. That is the `canvas=` value
`easel run --state-digest` writes (main.rs `state_digest_line`: FNV-1a of
`Canvas::write_state(.., "")`), so when the two agree, the dump holds every
value the checkpoint holds, bit for bit, including the mixer identity and
solvent section. With --digest, checks the digest file's line for
the dump's chunk and exits 1 on a mismatch.

This supports the baseline's engine-3 scenes without drawings or carried
tube parts. Other layouts are refused rather than partially reconstructed.
"""

import argparse
import os
import re
import struct
import sys

sys.dont_write_bytecode = True  # no __pycache__ in the package
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from state_compare import FormatError, load  # noqa: E402


def fnv1a(b):
    h = 0xCBF29CE484222325
    for x in b:
        h ^= x
        h = (h * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return h


def rebuild(fields):
    def raw(name):
        return fields[name][2]

    def vals(name):
        dt, shape, r = fields[name]
        code, size = {"f32": ("f", 4), "f64": ("d", 8), "u32": ("I", 4), "u64": ("Q", 8)}[dt]
        return struct.unpack("<%d%s" % (len(r) // size, code), r)

    u64 = lambda v: struct.pack("<Q", v)
    f32 = lambda v: struct.pack("<f", v)
    if vals("engine") != (3,) or raw("drawing") or any(n.startswith("wet.parts.") for n in fields):
        raise ValueError("only engine-3 baseline scenes without drawings or tube parts are supported")
    if fields["wet.solvent"][:2] != ("f32", fields["wet.vol"][1]):
        raise ValueError("wet.solvent must be f32 with the wet.vol shape")
    if fields.get("mixer.identity", ())[:2] != ("u64", (1,)) or len(raw("mixer.identity")) != 8:
        raise ValueError("mixer.identity must be one u64 from the producing build")
    out = [b"PAINTC19", u64(0), raw("mixer.identity"), u64(9), u64(0)]
    out += [u64(v) for v in vals("frame")]
    out += [raw("scale"), raw("mm_per_unit")]
    linen = vals("linen")
    if linen[4] == 0.0:
        out.append(u64(0))
    else:
        out += [u64(1)] + [f32(v) for v in linen[:4]] + [raw("linen_seed")]
    out += [raw("surf_gen"), u64(vals("wet.current")[0])]
    for box in ("wet.dirty",):
        b = vals(box)
        out += [u64(0)] if b[0] == 0 else [u64(1)] + [u64(v) for v in b[1:]]
    out += [raw(n) for n in ("color", "height", "film", "wet.vol", "wet.lat", "wet.hide", "wet.stroke", "wet.touched")]
    out += [raw("clock.now"), u64(vals("clock.mark")[0])]
    b = vals("clock.tacky")
    out += [u64(0)] if b[0] == 0 else [u64(1)] + [u64(v) for v in b[1:]]
    parts = [vals("clock." + k) for k in ("cure", "lev", "seen", "sub", "srate", "th")]
    if len(parts[0]) == 0:
        out.append(u64(0))
    else:
        out.append(u64(1))
        out.append(b"".join(struct.pack("<6f", *p) for p in zip(*parts)))
    out += [raw("ground_um"), raw("wet.cover")]
    d = raw("drawing")
    out += [u64(0)] if len(d) == 0 else [u64(1), d]
    hs = vals("hand_slice")
    out += [u64(0)] if hs[0] == 0.0 else [u64(1), f32(hs[1])]
    out += [raw("tally"), u64(vals("engine")[0]), raw("wet.solvent")]
    return b"".join(out)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("dump")
    ap.add_argument("--digest")
    a = ap.parse_args()
    try:
        head, fields = load(a.dump)
        h = "%016x" % fnv1a(rebuild(fields))
    except (OSError, ValueError, KeyError, struct.error, FormatError) as error:
        print("%s: invalid baseline dump: %s" % (a.dump, error), file=sys.stderr)
        return 2
    print("chunk %d canvas=%s" % (head["chunk"], h))
    if a.digest:
        lines = open(a.digest).read().splitlines()
        want = re.search(r"canvas=([0-9a-f]{16})", lines[head["chunk"] - 1]).group(1)
        if want != h:
            print("MISMATCH: the digest file says canvas=%s" % want)
            return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
