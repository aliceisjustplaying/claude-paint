#!/usr/bin/env python3
"""Capture the 11 existing tiny old-log cases with exact replay checks.

Each case runs twice. PNG bytes and all state-digest bytes must match; only
the non-state secs= field is removed from digests. The PNG must also match
the existing historical golden, which is a macOS arm64 reference. A golden
mismatch on another host/compiler is not evidence of a regression; use a
matched macOS arm64 build for this check. Golden files are never written.

With --compare DIRECTORY, compare against an earlier capture and write
old-log-comparison.csv with before/after hashes and exact byte equality.
"""

import argparse
import csv
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import subprocess
import sys


EXPECTED_CASES = 11
SECS_FIELD = re.compile(rb" secs=[^\s]+")
CASE_NAME = re.compile(r"[A-Za-z0-9_-]+")


class CaptureError(Exception):
    """A capture or comparison did not satisfy its exact checks."""


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def command_output(command):
    return subprocess.check_output(command, text=True).strip()


def read_cases(directory):
    rows = []
    with (directory / "cases.tsv").open(newline="") as source:
        for row in csv.reader(source, delimiter="\t"):
            if not row or row[0].startswith("#"):
                continue
            if len(row) < 3 or not CASE_NAME.fullmatch(row[0]):
                raise CaptureError("invalid old-log case row: %r" % row)
            name, log, width = row[:3]
            width = int(width)
            if width <= 0:
                raise CaptureError("%s: width must be positive" % name)
            rows.append((name, log, width))
    if len(rows) != EXPECTED_CASES or len({row[0] for row in rows}) != len(rows):
        raise CaptureError("expected exactly %d distinct old-log cases" % EXPECTED_CASES)
    return rows


def replay(binary, log, width, output, environment):
    png = output.parent / (output.name + ".png")
    digest = output.parent / (output.name + ".digest")
    result = subprocess.run(
        [str(binary), "run", str(log), "--width", str(width), "--out", str(png),
         "--state-digest", str(digest)],
        env=environment, capture_output=True, text=True,
    )
    if result.returncode:
        raise CaptureError("%s: replay exited %d\n%s" %
                           (output.name, result.returncode, result.stderr.strip()))
    normalized, fields = SECS_FIELD.subn(b"", digest.read_bytes())
    if not fields:
        raise CaptureError("%s: digest contains no secs= fields" % output.name)
    # Work in bytes: every other field, whitespace and line ending is retained.
    digest.write_bytes(normalized)
    return png.read_bytes(), normalized


def capture_case(binary, directory, destination, case, environment):
    name, log, width = case
    first = replay(binary, directory / log, width, destination / name, environment)
    repeat = replay(binary, directory / log, width,
                    destination / (name + ".repeat"), environment)
    if first != repeat:
        raise CaptureError("%s: repeated PNG or state digest differs" % name)
    if first[0] != (directory / "golden" / (name + ".png")).read_bytes():
        raise CaptureError("%s: PNG differs from historical macOS arm64 golden" % name)
    for suffix in (".repeat.png", ".repeat.digest"):
        (destination / (name + suffix)).unlink()
    print("%s: repeat PNG/digest and historical PNG identical" % name, flush=True)
    return {
        "case": name, "input": log, "width": width,
        "input_sha256": sha256((directory / log).read_bytes()),
        "png_sha256": sha256(first[0]),
        "state_digest_sha256": sha256(first[1]),
        "same_binary_repeat": "identical", "historical_png": "identical",
    }


def indexed_cases(manifest):
    rows = manifest["cases"]
    indexed = {row["case"]: row for row in rows}
    if len(indexed) != EXPECTED_CASES or len(indexed) != len(rows):
        raise CaptureError("manifest must contain exactly %d distinct cases" % EXPECTED_CASES)
    if any(not CASE_NAME.fullmatch(name) for name in indexed):
        raise CaptureError("manifest contains an invalid case name")
    return indexed


def compare_captures(before_directory, after_directory, after_manifest):
    before_bytes = (before_directory / "manifest.json").read_bytes()
    before_manifest = json.loads(before_bytes)
    before = indexed_cases(before_manifest)
    after = indexed_cases(after_manifest)
    if before.keys() != after.keys():
        raise CaptureError("capture manifests contain different case names")
    receipts = []
    for name, current in after.items():
        previous = before[name]
        if any(previous[key] != current[key] for key in ("input", "width")):
            raise CaptureError("%s: capture input or width differs" % name)
        if ("input_sha256" in previous and
                previous["input_sha256"] != current["input_sha256"]):
            raise CaptureError("%s: input log contents differ" % name)
        receipt = {"case": name, "input": current["input"], "width": current["width"]}
        for suffix, key, label in ((".png", "png_sha256", "png"),
                                   (".digest", "state_digest_sha256", "state_digest")):
            old_bytes = (before_directory / (name + suffix)).read_bytes()
            new_bytes = (after_directory / (name + suffix)).read_bytes()
            old_hash, new_hash = sha256(old_bytes), sha256(new_bytes)
            if old_hash != previous[key] or new_hash != current[key]:
                raise CaptureError("%s: %s bytes do not match their manifest SHA256" %
                                   (name, label))
            receipt["before_" + key] = old_hash
            receipt["after_" + key] = new_hash
            receipt[label + "_identical"] = old_bytes == new_bytes
        receipt["identical"] = receipt["png_identical"] and receipt["state_digest_identical"]
        receipts.append(receipt)
    with (after_directory / "old-log-comparison.csv").open("w", newline="") as target:
        writer = csv.DictWriter(target, fieldnames=list(receipts[0]), lineterminator="\n")
        writer.writeheader()
        writer.writerows(receipts)
    identical = all(row["identical"] for row in receipts)
    after_manifest["comparison"] = {
        "before_manifest_sha256": sha256(before_bytes),
        "before_commit": before_manifest["commit"],
        "before_binary_sha256": before_manifest["binary_sha256"],
        "before_rustc": before_manifest["rustc"],
        "identical": identical,
    }
    return identical


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("checkout", type=Path, help="checkout containing the old-log fixtures")
    parser.add_argument("release_easel", type=Path, help="release easel binary to replay")
    parser.add_argument("output_directory", type=Path, help="destination outside the fixture tree")
    parser.add_argument("--compare", type=Path, metavar="DIRECTORY",
                        help="compare PNG/digest bytes and manifest cases with an earlier capture")
    args = parser.parse_args()
    try:
        checkout = args.checkout.resolve()
        binary = args.release_easel.resolve(strict=True)
        destination = args.output_directory.resolve()
        directory = checkout / "crates/easel/tests/old_logs"
        if destination == directory or directory in destination.parents:
            raise CaptureError("output directory must be outside the historical fixture tree")
        if args.compare and args.compare.resolve() == destination:
            raise CaptureError("comparison directory must differ from the capture destination")
        cases = read_cases(directory)
        outputs = ["manifest.json", "old-log-comparison.csv"]
        outputs.extend(name + suffix for name, _, _ in cases
                       for suffix in (".png", ".digest", ".repeat.png", ".repeat.digest"))
        for name in outputs:
            path = (destination / name).resolve()
            if path == directory or directory in path.parents:
                raise CaptureError("%s: output resolves into the historical fixture tree" % name)
        destination.mkdir(parents=True, exist_ok=True)
        environment = dict(os.environ)
        environment.pop("EASEL_BOX", None)
        manifest = {
            "commit": command_output(["git", "-C", str(checkout), "rev-parse", "HEAD"]),
            "working_tree_dirty": bool(command_output(
                ["git", "-C", str(checkout), "status", "--porcelain"])),
            "binary_sha256": sha256(binary.read_bytes()),
            "rustc": command_output(["rustc", "-V"]),
            "rustc_verbose": command_output(["rustc", "-vV"]),
            "host": {"system": platform.system(), "machine": platform.machine()},
            "cases_tsv_sha256": sha256((directory / "cases.tsv").read_bytes()),
            "digest_normalization": "remove only secs= fields; retain all other bytes",
            "cases": [capture_case(binary, directory, destination, case, environment)
                      for case in cases],
        }
        identical = not args.compare or compare_captures(args.compare, destination, manifest)
        (destination / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
        if not identical:
            raise CaptureError("PNG or state digest differs; see old-log-comparison.csv")
        print("%d existing old-log cases passed exact replay checks" % len(cases))
        return 0
    except (CaptureError, OSError, ValueError, KeyError, TypeError, subprocess.CalledProcessError) as error:
        print("capture_old_logs: %s" % error, file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
