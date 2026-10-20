#!/usr/bin/env python3
"""The known-rag diagnostic never hides another baseline failure."""

import contextlib
import importlib.util
import io
import math
import os
from pathlib import Path
import struct
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.dont_write_bytecode = True
from baseline_state_compare_test import dump, sc

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("known_rag_baseline", ROOT / "scripts/known_rag_baseline.py")
guard = importlib.util.module_from_spec(spec)
spec.loader.exec_module(guard)
IMAGE = "image comparison: FAIL: 707/8192 pixels changed; max RGB delta 109/255; allowed 8 pixels, max 2/255"


class KnownRagTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.scenes = self.root / "scenes"
        self.prepare_scene(guard.BASE / "state" / "rag")
        self.rag = self.scenes / "rag"
        self.report = self.root / "image-report.txt"
        self.report.write_text(IMAGE + "\n")

    def prepare_scene(self, scene):
        destination = self.scenes / scene.name
        destination.mkdir(parents=True)
        for chunk, source in sc.state_files(str(scene)).items():
            _, fields = sc.load(source)
            if scene.name == "rag":
                for number, field, index, value, _ in guard.SIGNATURE:
                    if chunk != "chunk-%03d" % number:
                        continue
                    dtype, shape, raw = fields[field]
                    if isinstance(index, int):
                        raw = bytearray(raw)
                        struct.pack_into("<" + sc.DT[dtype][0], raw, index * sc.DT[dtype][1], value)
                        raw = bytes(raw)
                    else:
                        values = sc.debug_values(field, raw.decode())
                        if index.endswith("mobile.mm3"):
                            mobile = "Mobile { mm3: %s, lat: [%s], hide: [%s], cure: 0.0, solv_mm3: 0.0, at_min: 0.0, at_secs: 0.0 }" % (value, ", ".join(["0.0"] * 7), ", ".join(["0.0"] * 5))
                            raw = raw.replace(b" }", (", mobile: " + mobile + " }").encode(), 1)
                        else:
                            old = index.split(".")[-1] + ": " + values[index][1]
                            self.assertEqual(raw.count(old.encode()), 1)
                            raw = raw.replace(old.encode(), (index.split(".")[-1] + ": " + str(value)).encode(), 1)
                        shape = (len(raw),)
                    fields[field] = dtype, shape, raw
            dump(destination / (chunk + ".state"), fields)

    def edit(self, chunk, field, transform, scene="rag"):
        path = self.scenes / scene / ("chunk-%03d.state" % chunk)
        _, fields = sc.load(str(path))
        fields[field] = transform(fields[field])
        dump(path, fields)

    def diagnose(self, limits=sc.LINUX_LIMITS):
        with contextlib.redirect_stdout(io.StringIO()):
            return guard.diagnose(self.rag, limits)

    def test_known_scalars_with_exact_or_linux_unaffected_fields(self):
        self.assertFalse(self.diagnose())
        self.assertFalse(self.diagnose({}))

    def test_each_signature_scalar_outside_its_bound_fails(self):
        for number, field, index, _, tolerance in guard.SIGNATURE:
            with self.subTest(chunk=number, field=field, index=index):
                path = self.rag / ("chunk-%03d.state" % number)
                original = path.read_bytes()
                def change(value):
                    dtype, shape, raw = value
                    if isinstance(index, int):
                        raw = bytearray(raw)
                        got = struct.unpack_from("<" + sc.DT[dtype][0], raw, index * sc.DT[dtype][1])[0]
                        struct.pack_into("<" + sc.DT[dtype][0], raw, index * sc.DT[dtype][1], got + 10 * tolerance)
                        raw = bytes(raw)
                    else:
                        got = sc.debug_values(field, raw.decode())[index][1]
                        old = index.split(".")[-1] + ": " + got
                        raw = raw.replace(old.encode(), (index.split(".")[-1] + ": " + str(float(got) + 10 * tolerance)).encode(), 1)
                        shape = (len(raw),)
                    return dtype, shape, raw
                self.edit(number, field, change)
                self.assertTrue(self.diagnose())
                path.write_bytes(original)

    def test_unrelated_integer_float_and_pre_wipe_changes_fail(self):
        for chunk, field, dtype, value in [(4, "studio.seed", "Q", 99), (8, "color", "f", 0.1), (3, "wet.vol", "f", 0.1)]:
            with self.subTest(chunk=chunk, field=field):
                path = self.rag / ("chunk-%03d.state" % chunk)
                original = path.read_bytes()
                self.edit(chunk, field, lambda f: (f[0], f[1], struct.pack("<" + dtype, value) + f[2][sc.DT[f[0]][1]:]))
                self.assertTrue(self.diagnose())
                path.write_bytes(original)

    def test_known_field_shape_and_nonfinite_changes_fail(self):
        for transform in [lambda f: (f[0], (1,), f[2]), lambda f: (f[0], f[1], struct.pack("<f", math.nan) + f[2][4:])]:
            path = self.rag / "chunk-004.state"
            original = path.read_bytes()
            self.edit(4, "wet.lat", transform)
            self.assertTrue(self.diagnose())
            path.write_bytes(original)

    def test_unrelated_debug_change_is_not_hidden_after_known_differences(self):
        self.edit(8, "brushes.debug", lambda f: (f[0], f[1], f[2].replace(b"pickup: 0.18", b"pickup: 0.19", 1)))
        self.assertTrue(self.diagnose())

    def test_added_nonzero_solvent_is_not_known(self):
        path = self.rag / "chunk-004.state"
        _, fields = sc.load(str(path))
        fields["wet.solvent"] = "f32", (1,), struct.pack("<f", 1e-15)
        dump(path, fields)
        self.assertTrue(self.diagnose())

    def test_missing_or_extra_chunks_fail(self):
        path = self.rag / "chunk-008.state"
        path.rename(self.rag / "chunk-009.state")
        self.assertTrue(self.diagnose())

    def test_malformed_dump_returns_exit_two(self):
        (self.rag / "chunk-004.state").write_bytes(b"not a state dump")
        result = subprocess.run([sys.executable, str(ROOT / "scripts/known_rag_baseline.py"), str(self.rag)], capture_output=True, text=True)
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertIn("invalid input", result.stderr)

    def test_image_diagnostic_includes_both_measured_platform_formats(self):
        for host, machine, report, expected in [
            ("linux", "x86_64", IMAGE, 0),
            ("linux", "x86_64", IMAGE.replace("707/", "708/"), 1),
            ("linux", "x86_64", IMAGE.replace("109/", "110/"), 1),
            ("linux", "x86_64", "image comparison: FAIL: alpha differs", 1),
            ("linux", "x86_64", IMAGE.replace("FAIL: ", "FAIL: cross-platform (macos-aarch64 -> linux-x86_64); "), 0),
            ("linux", "x86_64", "image comparison: FAIL: tolerance (macos-aarch64 -> linux-x86_64); 707/8192 pixels changed; max RGB delta 109/255; max alpha delta 0/255", 0),
            ("darwin", "arm64", "image comparison: FAIL: tolerance (macos-aarch64 -> macos-aarch64); 707/8192 pixels changed; max RGB delta 109/255; max alpha delta 0/255", 0),
            ("linux", "x86_64", "image comparison: FAIL: tolerance (unknown -> unknown); 707/8192 pixels changed; max RGB delta 109/255; max alpha delta 3/255", 1),
            ("darwin", "arm64", "image comparison: FAIL: byte-exact; 707/8192 pixels changed; max RGB delta 109/255", 0),
            ("darwin", "arm64", "image comparison: FAIL: byte-exact comparison (macos-aarch64 -> macos-aarch64); 707/8192 pixels changed; max RGB delta 109/255", 0),
            ("darwin", "arm64", IMAGE.replace("707/", "708/"), 1),
        ]:
            with self.subTest(host=host, report=report):
                self.report.write_text(report + "\n")
                with patch.object(sys, "platform", host), patch.object(guard.platform, "machine", return_value=machine), patch.object(sys, "argv", ["known_rag_baseline.py", str(self.rag), "--image-report", str(self.report)]), contextlib.redirect_stdout(io.StringIO()):
                    self.assertEqual(guard.main(), expected)

    def baseline(self, extra_env=None):
        for scene in (guard.BASE / "state").iterdir():
            if not (self.scenes / scene.name).exists():
                self.prepare_scene(scene)
        bins = self.root / "bin"
        bins.mkdir(exist_ok=True)
        programs = {
            "uname": "#!/bin/sh\nif [ \"$1\" = -s ]; then echo Linux; else echo x86_64; fi\n",
            "easel": "#!%s\nimport pathlib, shutil, sys\nname=pathlib.Path(sys.argv[2]).stem\nout=sys.argv[sys.argv.index('--dump-state')+1]\nshutil.copytree(pathlib.Path(%r)/name,out)\n" % (sys.executable, str(self.scenes)),
            "uv": "#!%s\nimport os, pathlib, subprocess, sys\nargs=sys.argv[2:]\nif args[0]=='--script':\n name=pathlib.Path(args[2]).stem\n if name=='rag':\n  print(os.environ.get('RAG_IMAGE_REPORT',%r));sys.exit(1)\n if name==os.environ.get('FAIL_IMAGE'):\n  print('image comparison: FAIL: 100/8192 pixels changed; max RGB delta 50/255; allowed 8 pixels, max 2/255');sys.exit(1)\n print('image comparison: PASS: 0/8192 pixels changed; max RGB delta 0/255; allowed 8 pixels, max 2/255');sys.exit(0)\nsys.exit(subprocess.run([sys.executable,*args[2:]]).returncode)\n" % (sys.executable, IMAGE),
        }
        for name, code in programs.items():
            path = bins / name
            path.write_text(code)
            path.chmod(0o755)
        env = dict(os.environ, PATH=str(bins) + os.pathsep + os.environ["PATH"], **(extra_env or {}))
        return subprocess.run(["bash", str(ROOT / "scripts/tests/baseline_state.sh"), str(bins / "easel")], env=env, capture_output=True, text=True)

    def test_baseline_checks_all_scenes_and_emits_one_exit_four_marker(self):
        result = self.baseline()
        self.assertEqual(result.returncode, 4, result.stdout + result.stderr)
        self.assertEqual(result.stdout.count("KNOWN_FAILURE: rag-historical-baseline-main"), 1)
        self.assertIn("stroke: state within tolerance", result.stdout)
        self.assertIn("wait: state within tolerance", result.stdout)
        self.assertIn("chunk-004: DIFFERENT", result.stdout)

    def test_baseline_never_marks_unrelated_state_or_image_failure_known(self):
        for env in [{"FAIL_IMAGE": "body"}, {"RAG_IMAGE_REPORT": "image comparison: FAIL: dimensions differ"}]:
            result = self.baseline(env)
            self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
            self.assertNotIn("KNOWN_FAILURE:", result.stdout)
            self.assertIn("wait: state within tolerance", result.stdout)
        self.edit(1, "studio.seed", lambda f: (f[0], f[1], struct.pack("<Q", 99)), scene="body")
        result = self.baseline()
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertNotIn("KNOWN_FAILURE:", result.stdout)


if __name__ == "__main__":
    unittest.main()
