#!/usr/bin/env python3
"""State float limits stay narrow and apply on every platform by default."""

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


def dump(path, fields, chunk=1):
    data = b""
    meta = []
    for name, (dtype, shape, raw) in fields.items():
        meta.append(dict(name=name, dtype=dtype, shape=shape, offset=len(data), nbytes=len(raw)))
        data += raw
    header = json.dumps(dict(format="CPSTATE1", chunk=chunk, fields=meta)).encode()
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

    def test_numeric_boundary_applies_with_default_or_empty_limits(self):
        limit = sc.LINUX_LIMITS["color"]
        a, b = numeric("f64", 0.0), numeric("f64", limit)
        self.assertFalse(self.compare("color", a, b)[0])
        self.assertFalse(self.compare("color", a, b, {})[0])
        problems = []
        sc.compare_numeric("color", a, b, problems)
        self.assertFalse(problems)
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

    def test_unlisted_float_uses_default_absolute_limit(self):
        limit = sc.DEFAULT_ABS_TOLERANCE
        self.assertEqual(sc.MAX_REL_TOLERANCE, 0.0)
        for dtype in ("f32", "f64"):
            with self.subTest(dtype=dtype):
                self.assertFalse(self.compare("studio.clock", numeric(dtype, 0.0), numeric(dtype, limit / 2))[0])
                self.assertTrue(self.compare("studio.clock", numeric(dtype, 0.0), numeric(dtype, limit * 2))[0])
        self.assertFalse(self.compare("studio.clock", numeric("f64", 0.0), numeric("f64", limit))[0])
        self.assertTrue(self.compare("studio.clock", numeric("f64", 0.0), numeric("f64", math.nextafter(limit, math.inf)))[0])
        self.assertTrue(self.compare("studio.clock", numeric("f64", 1e9), numeric("f64", 1e9 + 0.001))[0])

    def test_shape_and_dtype_remain_exact(self):
        self.assertTrue(self.compare("color", numeric("f32", 0.0), numeric("f64", 0.0))[0])
        self.assertTrue(self.compare("color", numeric("f32", 0.0), numeric("f32", 0.0, 0.0))[0])

    def test_signed_zero_matches_and_all_nonfinite_values_fail(self):
        self.assertFalse(self.compare("color", numeric("f64", 0.0), numeric("f64", -0.0))[0])
        for x, y in [(0.0, math.nan), (math.inf, -math.inf), (math.inf, 0.0), (math.inf, math.inf), (math.nan, math.nan)]:
            with self.subTest(x=x, y=y):
                self.assertTrue(self.compare("color", numeric("f64", x), numeric("f64", y))[0])
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
        self.assertFalse(self.compare_debug(a, b, {})[0])
        self.assertTrue(self.compare_debug(a, b.replace("0.0011", "0.0011000000000000003"))[0])
        self.assertTrue(self.compare_debug(a, a.replace("51614", "51615"))[0])
        self.assertTrue(self.compare_debug(a, a.replace("vol: 0.0", "vol: 1"))[0])
        self.assertFalse(self.compare_debug(a, a.replace("vol: 0.0", "vol: -0.0"))[0])
        self.assertTrue(self.compare_debug(a + "\nHeld { flag: false }", a + "\nHeld { flag: true }")[0])
        self.assertTrue(self.compare_debug(a, "Held { bristles: [] }")[0])
        self.assertTrue(self.compare_debug(a, a.replace("vol: 0.0", "vol: 0.0, solvent: 1e-15"))[0])

    def test_debug_unlisted_floats_and_nonfinite_values(self):
        a = "Held { pickup: 0.0 }"
        self.assertFalse(self.compare_debug(a, "Held { pickup: 0.00000005 }")[0])
        self.assertTrue(self.compare_debug(a, "Held { pickup: 0.0000002 }")[0])
        for value in ("NaN", "inf", "-inf"):
            with self.subTest(value=value):
                invalid = "Held { pickup: %s }" % value
                self.assertTrue(self.compare_debug(a, invalid)[0])
                self.assertTrue(self.compare_debug(invalid, invalid)[0])

    def test_recorded_zero_solvent_rejects_any_nonzero_value(self):
        self.assertFalse(self.compare("wet.solvent", numeric("f32", 0.0), numeric("f32", -0.0))[0])
        self.assertTrue(self.compare("wet.solvent", numeric("f32", 0.0), numeric("f32", 1e-15))[0])
        for name, original in [
            ("brushes.debug", "Held { bristles: [Bristle { solvent: 0.0 }] }"),
            ("rags.debug", "Rag { solvent_mm3: 0.0 }"),
            ("rags.debug", "Rag { mobile: Mobile { solv_mm3: 0.0 } }"),
        ]:
            with self.subTest(field=original):
                problems = []
                sc.compare_text(name, text(original), text(original.replace("0.0", "1e-15")), problems, [], True)
                self.assertTrue(problems)

    def test_cli_checks_solvent_already_present_in_the_reference(self):
        with tempfile.TemporaryDirectory() as tmp:
            a, b = Path(tmp) / "a.state", Path(tmp) / "b.state"
            dump(a, {"wet.solvent": numeric("f32", 0.0)})
            dump(b, {"wet.solvent": numeric("f32", 1e-15)})
            result = subprocess.run([sys.executable, str(TOOL), str(a), str(b), "--added-zero"], capture_output=True, text=True)
            self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
            self.assertIn("wet.solvent: maximum allowed |diff| 0", result.stdout)

    def test_debug_parser_remains_lossless(self):
        for invalid in ("Held { pickup: 0.0 } trailing", "Held { pickup: ??? }", "Held { pickup: 0.0"):
            with self.subTest(invalid=invalid), self.assertRaises(sc.FormatError):
                self.compare_debug(invalid, invalid)

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
            default = subprocess.run(args, capture_output=True, text=True)
            self.assertEqual(default.returncode, 0, default.stdout)
            within = subprocess.run(args + ["--linux-tolerance"], capture_output=True, text=True)
            self.assertEqual(within.returncode, 0, within.stdout)
            self.assertIn("maximum allowed |diff| 5e-06; maximum observed |diff| 5e-06", within.stdout)
            self.assertEqual(default.stdout, within.stdout)
            self.assertIn("relative tolerance 0", within.stdout)
            self.assertIn("WITHIN NUMERIC TOLERANCE", within.stdout)
            dump(b, {"color": numeric("f64", 0.0), "wet.solvent": numeric("f64", 1e-15)})
            added = subprocess.run(args + ["--linux-tolerance", "--added-zero"], capture_output=True, text=True)
            self.assertEqual(added.returncode, 1, added.stdout)
            self.assertIn("added field has 1 nonzero values", added.stdout)

    def test_baseline_script_uses_the_same_default_on_every_platform(self):
        with tempfile.TemporaryDirectory() as tmp:
            bins = Path(tmp)
            for name, code in {"easel": "exit 0", "uv": "printf '%s\\n' \"$*\""}.items():
                path = bins / name
                path.write_text("#!/bin/sh\n" + code + "\n")
                path.chmod(0o755)
            env = dict(os.environ, PATH=str(bins) + os.pathsep + os.environ["PATH"])
            for system, arch in [("Linux", "x86_64"), ("Darwin", "arm64"), ("Linux", "aarch64")]:
                with self.subTest(system=system, arch=arch):
                    uname = bins / "uname"
                    uname.write_text("#!/bin/sh\nif [ \"$1\" = -s ]; then echo %s; else echo %s; fi\n" % (system, arch))
                    uname.chmod(0o755)
                    script = TOOL.parents[4] / "scripts/tests/baseline_state.sh"
                    result = subprocess.run(["bash", str(script), str(bins / "easel")], env=env, capture_output=True, text=True)
                    self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                    self.assertNotIn("--linux-tolerance", result.stdout)
                    self.assertEqual(result.stdout.count("state within tolerance on every baseline field"), 6)


if __name__ == "__main__":
    unittest.main()
