#!/usr/bin/env python3
"""Diagnose main's pre-existing rag/reference mismatch, never a baseline pass.

known_rag_baseline.py <rag dump directory> [--image-report FILE]
Exit 0: known diagnostic scalars and no unrelated state failures.
Exit 1: different failure; 2: invalid input. No known-failure marker is emitted.
"""

import argparse
import importlib.util
import math
from pathlib import Path
import platform
import re
import struct
import sys

sys.dont_write_bytecode = True
BASE = Path(__file__).resolve().parents[1] / "notes/thinner/baseline"
spec = importlib.util.spec_from_file_location("state_compare", BASE / "tools/state_compare.py")
sc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(sc)

SIGNATURE = [
    (4, "rags.debug", "rags[0].load", 0.5218438, 1e-7),
    (4, "rags.debug", "rags[0].mobile.mm3", 2.2489905, 1e-6),
    (4, "wet.vol", 1310, 3.9498796, 8e-6),
    (6, "rags.debug", "rags[1].load", 0.825874, 1e-6),
    (6, "wet.vol", 1884, 8.9917135, 1e-5),
    (8, "rags.debug", "rags[0].load", 0.050614506, 1e-7),
    (8, "brushes.debug", "brushes[0].bristles[92].vol", 2746.4558, 2e-3),
    (8, "wet.floor", 1310, 5.1260171, 1e-5),
]


def diagnose(directory, limits):
    files = sc.state_files(str(directory))
    expected = sc.state_files(str(BASE / "state/rag"))
    if set(files) != set(expected):
        return ["rag: expected exactly the eight baseline chunks"]
    problems = []
    for chunk, pa in expected.items():
        number = int(chunk.split("-")[1])
        _, reference = sc.load(pa)
        _, actual = sc.load(files[chunk])
        ignored_fields, ignored_values = set(), {}
        if number >= 4:
            ignored_fields = {"wet.vol", "wet.lat", "wet.hide", "clock.cure", "clock.lev", "clock.seen", "clock.th"}
            if number == 8:
                ignored_fields.add("wet.floor")
            for name in ignored_fields:
                if name not in actual or reference[name][:2] != actual[name][:2] or len(reference[name][2]) != len(actual[name][2]):
                    problems.append("%s: %s missing or dtype/shape changed" % (chunk, name))
                elif any(not math.isfinite(v[0]) for v in struct.iter_unpack("<" + sc.DT[actual[name][0]][0], actual[name][2])):
                    problems.append("%s: %s has nonfinite values" % (chunk, name))
            for name in ("brushes.debug", "rags.debug"):
                if name not in actual or actual[name][0] != "utf8":
                    continue
                values = sc.debug_values(name, actual[name][2].decode())
                skipped = set()
                for key, value in values.items():
                    allowed = bool(name == "rags.debug" and re.fullmatch(r"rags\[[01]\]\.(load|soaked)", key))
                    allowed |= bool(name == "rags.debug" and re.fullmatch(r"rags\[[01]\]\.mobile\.(mm3|lat\[[0-6]\]|hide\[[0-4]\]|cure|at_min|at_secs)", key))
                    allowed |= bool(number == 8 and name == "brushes.debug" and re.fullmatch(r"brushes\[0\]\.bristles\[\d+\]\.(vol|lat\[[0-6]\]|hide\[[0-4]\]|cure)", key))
                    if name == "rags.debug" and re.fullmatch(r"rags\[[01]\]\.mobile\.(lat|hide)\.len", key):
                        if value != ("num", "7" if key.endswith("lat.len") else "5"):
                            problems.append("%s: %s shape changed" % (chunk, key))
                    if allowed:
                        if value[0] != "num" or re.fullmatch(r"-?\d+", value[1]) or not math.isfinite(sc.num(value[1])):
                            problems.append("%s: %s has an unexpected type/value" % (chunk, key))
                        skipped.add(key)
                ignored_values[name] = skipped
        args = argparse.Namespace(ignore=ignored_fields, ignore_values=ignored_values,
                                  added_zero=True, limits=limits, observed={})
        _, unrelated, _ = sc.compare_files(pa, files[chunk], args)
        problems.extend("%s: %s" % (chunk, p) for p in unrelated)
        for at_chunk, field, index, value, tolerance in SIGNATURE:
            if at_chunk != number:
                continue
            if field not in actual:
                problems.append("%s: diagnostic field %s missing" % (chunk, field))
                continue
            if isinstance(index, str):
                node = sc.debug_values(field, actual[field][2].decode()).get(index)
                if node is None or node[0] != "num":
                    problems.append("%s: diagnostic value %s missing/not numeric" % (chunk, index))
                    continue
                got = sc.num(node[1])
            else:
                dtype, _, raw = actual[field]
                if dtype not in ("f32", "f64") or len(raw) < (index + 1) * sc.DT[dtype][1]:
                    problems.append("%s: invalid diagnostic field %s" % (chunk, field))
                    continue
                got = struct.unpack_from("<" + sc.DT[dtype][0], raw, index * sc.DT[dtype][1])[0]
            error = abs(got - value)
            print("%s %s[%s]: expected %.10g ± %.9g; observed %.10g; |error| %.9g" %
                  (chunk, field, index, value, tolerance, got, error))
            if not math.isfinite(got) or error > tolerance:
                problems.append("%s: diagnostic %s[%s] differs" % (chunk, field, index))
    return problems


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("directory", type=Path)
    ap.add_argument("--image-report", type=Path, help="also check compare_images.py's rag diagnostic")
    args = ap.parse_args()
    host = (sys.platform, platform.machine())
    if host not in (("linux", "x86_64"), ("darwin", "arm64")):
        print("known_rag_baseline: unmeasured host %s %s" % host)
        return 1
    try:
        problems = diagnose(args.directory, sc.LINUX_LIMITS if host[0] == "linux" else {})
        if args.image_report:
            report = args.image_report.read_text().strip()
            match = re.fullmatch(r"image comparison: FAIL: (?:(?:(?:tolerance|cross-platform) \([^()\n]+\)|byte-exact(?: comparison \([^()\n]+\))?); )?(\d+)/8192 pixels changed; max RGB delta (\d+)/255(?:; max alpha delta 0/255)?(?:; allowed \d+ pixels, max \d+/255)?", report)
            if not match or int(match[1]) != 707 or int(match[2]) != 109:
                problems.append("rag PNG does not match the known image diagnostic: " + report)
    except (OSError, ValueError, KeyError, TypeError, struct.error, sc.FormatError) as error:
        print("known_rag_baseline: invalid input: %s" % error, file=sys.stderr)
        return 2
    for problem in problems:
        print("known_rag_baseline: " + problem)
    print("known_rag_baseline: " + ("different/unrelated failure" if problems else "matches main's historical rag mismatch (not a pass)"))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
