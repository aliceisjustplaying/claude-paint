#!/usr/bin/env python3
"""Time every test of one Rust test binary, one process per test.

    time_tests.py <test binary> <crate dir> <out.tsv> [--skip-file F] [--timeout 60] [--only NAME]...

Each test runs alone (`<binary> --exact <name> --test-threads=1`, cwd the
crate dir, as cargo runs it) in its own process group, killed after
--timeout seconds (the whole group: integration tests start easel
processes). Writes `test<TAB>status<TAB>seconds` lines: status ok, FAIL or
TIMEOUT. Tests named in --skip-file (one `<name>` a line, `#` comments) are
not run and are written as SKIPPED. Ignored tests are not listed, so not run.
Standard library only.
"""

import argparse
import os
import signal
import subprocess
import sys
import time


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("binary")
    ap.add_argument("crate_dir")
    ap.add_argument("out")
    ap.add_argument("--skip-file")
    ap.add_argument("--timeout", type=float, default=60.0)
    ap.add_argument("--only", action="append", default=[])
    a = ap.parse_args()
    skip = set()
    if a.skip_file:
        for line in open(a.skip_file):
            line = line.split("#")[0].strip()
            if line:
                skip.add(line)
    listed = subprocess.run([a.binary, "--list"], capture_output=True, text=True, check=True).stdout
    ignored = set(l[:-len(": test")] for l in subprocess.run([a.binary, "--list", "--ignored"], capture_output=True, text=True, check=True).stdout.splitlines() if l.endswith(": test"))
    tests = [l[:-len(": test")] for l in listed.splitlines() if l.endswith(": test")]
    tests = [t for t in tests if t not in ignored]
    if a.only:
        tests = [t for t in tests if t in a.only]
    with open(a.out, "w") as out:
        for t in tests:
            if t in skip:
                out.write("%s\tSKIPPED\t\n" % t)
                out.flush()
                continue
            t0 = time.monotonic()
            p = subprocess.Popen([a.binary, "--exact", t, "--test-threads=1", "-q"], cwd=a.crate_dir,
                                 stdout=subprocess.PIPE, stderr=subprocess.STDOUT, start_new_session=True)
            try:
                p.communicate(timeout=a.timeout)
                status = "ok" if p.returncode == 0 else "FAIL"
            except subprocess.TimeoutExpired:
                os.killpg(p.pid, signal.SIGKILL)
                p.communicate()
                status = "TIMEOUT"
            dt = time.monotonic() - t0
            out.write("%s\t%s\t%.2f\n" % (t, status, dt))
            out.flush()
            print("%-70s %-7s %7.2f" % (t, status, dt), flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
