#!/usr/bin/env python3
"""Linux state limits stay narrow; the default comparison stays exact."""

import importlib.util
import json
import math
import os
from pathlib import Path
import struct
import subprocess
import sys
import tempfile
import unittest

sys.dont_write_bytecode = True
TOOL = Path(__file__).resolve().parents[2] / "notes/thinner/baseline/tools/state_compare.py"
spec = importlib.util.spec_from_file_location("state_compare", TOOL)
sc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(sc)


def numeric(dtype, *values):
    return dtype, (len(values),), struct.pack("<%d%s" % (len(values), sc.DT[dtype][0]), *values)


def text(value):
    raw = value.encode()
    return "utf8", (len(raw),), raw


def dump(path, fields):
    data = b""
    meta = []
    for name, (dtype, shape, raw) in fields.items():
        meta.append(dict(name=name, dtype=dtype, shape=shape, offset=len(data), nbytes=len(raw)))
        data += raw
    header = json.dumps(dict(fields=meta)).encode()
    path.write_bytes(b"CPSTATE1" + struct.pack("<Q", len(header)) + header + data)


class StateToleranceTests(unittest.TestCase):
    def compare(self, name, a, b, limits=sc.LINUX_LIMITS):
        problems, observed = [], {}
        sc.compare_numeric(name, a, b, problems, limits, observed)
        return problems, observed

    def test_every_limit_includes_boundary_and_rejects_next_float(self):
        for name, limit in sc.LINUX_LIMITS.items():
            with self.subTest(field=name):
                observed = {}
                self.assertTrue(sc.float_within_limit(name, 0.0, limit, sc.LINUX_LIMITS, observed))
                self.assertEqual(observed[name], limit)
                self.assertFalse(sc.float_within_limit(name, 0.0, math.nextafter(limit, math.inf), sc.LINUX_LIMITS, observed))
                self.assertGreater(observed[name], limit)

    def test_numeric_boundary_and_default_exact(self):
        limit = sc.LINUX_LIMITS["color"]
        a, b = numeric("f64", 0.0), numeric("f64", limit)
        self.assertFalse(self.compare("color", a, b)[0])
        self.assertTrue(self.compare("color", a, b, {})[0])
        self.assertTrue(self.compare("color", a, numeric("f64", math.nextafter(limit, math.inf)))[0])

    def test_material_rag_changes_are_rejected(self):
        self.assertTrue(self.compare("wet.vol", numeric("f32", 0.9863235950469971), numeric("f32", 3.949877977371216))[0])
        self.assertTrue(self.compare("wet.vol", numeric("f32", 1.9533520936965942), numeric("f32", 8.991713523864746))[0])

    def test_absolute_limits_do_not_grow_with_magnitude(self):
        self.assertTrue(self.compare("height", numeric("f64", 1e9), numeric("f64", 1e9 + 0.001))[0])

    def test_integers_remain_exact_even_under_a_bounded_name(self):
        for dtype, value in [("u32", 10), ("u64", 2**63)]:
            with self.subTest(dtype=dtype):
                self.assertTrue(self.compare("color", numeric(dtype, value), numeric(dtype, value + 1))[0])

    def test_unlisted_float_remains_exact(self):
        self.assertTrue(self.compare("studio.clock", numeric("f64", 0.0), numeric("f64", 1e-15))[0])

    def test_shape_and_dtype_remain_exact(self):
        self.assertTrue(self.compare("color", numeric("f32", 0.0), numeric("f64", 0.0))[0])
        self.assertTrue(self.compare("color", numeric("f32", 0.0), numeric("f32", 0.0, 0.0))[0])

    def test_signed_zero_and_nonfinite_changes_fail(self):
        for x, y in [(0.0, -0.0), (0.0, math.nan), (math.inf, -math.inf), (math.inf, 0.0)]:
            with self.subTest(x=x, y=y):
                self.assertTrue(self.compare("color", numeric("f64", x), numeric("f64", y))[0])
        self.assertFalse(self.compare("color", numeric("f64", math.inf), numeric("f64", math.inf))[0])
        self.assertFalse(self.compare("color", numeric("f64", math.nan), numeric("f64", math.nan))[0])
        a, b = ("f32", (1,), struct.pack("<I", 0x7fc00001)), ("f32", (1,), struct.pack("<I", 0x7fc00002))
        self.assertTrue(self.compare("color", a, b)[0])

    def compare_debug(self, a, b, limits=sc.LINUX_LIMITS, added_zero=True):
        problems, added, observed = [], [], {}
        sc.compare_text("brushes.debug", text(a), text(b), problems, added, added_zero, limits, observed)
        return problems, observed

    def test_debug_floats_and_integer_structure(self):
        a = "Held { bristles: [Bristle { vol: 0.0 }], seed: 18446744073709551614 }"
        b = a.replace("vol: 0.0", "vol: 0.0011")
        self.assertFalse(self.compare_debug(a, b)[0])
        self.assertTrue(self.compare_debug(a, b, {})[0])
        self.assertTrue(self.compare_debug(a, b.replace("0.0011", "0.0011000000000000003"))[0])
        self.assertTrue(self.compare_debug(a, a.replace("51614", "51615"))[0])
        self.assertTrue(self.compare_debug(a, a.replace("vol: 0.0", "vol: 1"))[0])
        self.assertTrue(self.compare_debug(a, a.replace("vol: 0.0", "vol: -0.0"))[0])
        self.assertTrue(self.compare_debug(a + "\nHeld { flag: false }", a + "\nHeld { flag: true }")[0])
        self.assertTrue(self.compare_debug(a, "Held { bristles: [] }")[0])
        self.assertTrue(self.compare_debug(a, a.replace("vol: 0.0", "vol: 0.0, solvent: 1e-15"))[0])

    def test_reports_all_errors_after_twenty_debug_failures(self):
        a = "Held { bristles: [" + ", ".join("Bristle { vol: 0.0 }" for _ in range(25)) + "] }"
        b = "Held { bristles: [" + ", ".join("Bristle { vol: %s }" % ("100.0" if i == 24 else "1.0") for i in range(25)) + "] }"
        problems, observed = self.compare_debug(a, b)
        self.assertTrue(problems)
        self.assertEqual(observed["brushes.debug:brushes[].bristles[].vol"], 100.0)

    def test_cli_reports_allowed_observed_and_preserves_added_zero(self):
        with tempfile.TemporaryDirectory() as tmp:
            a, b = Path(tmp) / "a.state", Path(tmp) / "b.state"
            dump(a, {"color": numeric("f64", 0.0)})
            dump(b, {"color": numeric("f64", 5e-6)})
            args = [sys.executable, str(TOOL), str(a), str(b), "--quiet"]
            exact = subprocess.run(args, capture_output=True, text=True)
            self.assertEqual(exact.returncode, 1, exact.stdout)
            within = subprocess.run(args + ["--linux-tolerance"], capture_output=True, text=True)
            self.assertEqual(within.returncode, 0, within.stdout)
            self.assertIn("maximum allowed |diff| 5e-06; maximum observed |diff| 5e-06", within.stdout)
            self.assertIn("WITHIN LINUX TOLERANCE", within.stdout)
            dump(b, {"color": numeric("f64", 0.0), "wet.solvent": numeric("f64", 1e-15)})
            added = subprocess.run(args + ["--linux-tolerance", "--added-zero"], capture_output=True, text=True)
            self.assertEqual(added.returncode, 1, added.stdout)
            self.assertIn("added field has 1 nonzero values", added.stdout)

    def test_baseline_script_only_opts_in_on_linux_x86_64(self):
        with tempfile.TemporaryDirectory() as tmp:
            bins = Path(tmp)
            for name, code in {"easel": "exit 0", "uv": "printf '%s\\n' \"$*\""}.items():
                path = bins / name
                path.write_text("#!/bin/sh\n" + code + "\n")
                path.chmod(0o755)
            env = dict(os.environ, PATH=str(bins) + os.pathsep + os.environ["PATH"])
            for system, arch, tolerant in [("Linux", "x86_64", True), ("Darwin", "arm64", False), ("Linux", "aarch64", False)]:
                with self.subTest(system=system, arch=arch):
                    uname = bins / "uname"
                    uname.write_text("#!/bin/sh\nif [ \"$1\" = -s ]; then echo %s; else echo %s; fi\n" % (system, arch))
                    uname.chmod(0o755)
                    script = TOOL.parents[4] / "scripts/tests/baseline_state.sh"
                    result = subprocess.run(["bash", str(script), str(bins / "easel")], env=env, capture_output=True, text=True)
                    self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                    self.assertEqual(result.stdout.count("--linux-tolerance"), 6 if tolerant else 0)
                    self.assertIn("state within tolerance on every baseline field" if tolerant else "state equal on every baseline field", result.stdout)


if __name__ == "__main__":
    unittest.main()
