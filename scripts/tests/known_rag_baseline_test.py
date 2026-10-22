#!/usr/bin/env python3
"""Every current baseline scene, including rag, must pass without exemptions."""

import os
from pathlib import Path
import re
import shutil
import struct
import subprocess
import sys
import tempfile
import unittest

sys.dont_write_bytecode = True
from baseline_state_compare_test import dump, numeric, sc

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / "notes/thinner/baseline"
GATES = (ROOT / "scripts/tests/baseline_state.sh", BASE / "tools/compare_build.sh")


class StrictRagBaselineTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.scenes = self.root / "state"
        shutil.copytree(BASE / "state", self.scenes)
        self.bins = self.root / "bin"
        self.bins.mkdir()
        programs = {
            "easel": """import pathlib, shutil, sys
name = pathlib.Path(sys.argv[2]).stem
out = sys.argv[sys.argv.index('--dump-state') + 1]
shutil.copytree(pathlib.Path(%r) / name, out)
if '--state-digest' in sys.argv:
    pathlib.Path(sys.argv[sys.argv.index('--state-digest') + 1]).write_text((pathlib.Path(%r) / 'digest' / (name + '.txt')).read_text())
sys.stdout.write((pathlib.Path(%r) / 'out' / (name + '.txt')).read_text())
""" % (str(self.scenes), str(BASE / "ref"), str(BASE / "ref")),
            "uv": """import os, pathlib, subprocess, sys
args = sys.argv[2:]
if args[0] == '--script':
    name = pathlib.Path(args[3]).stem
    failed = name == os.environ.get('FAIL_IMAGE')
    if failed:
        print('image comparison: FAIL: 707/8192 pixels changed; max RGB delta 109/255')
    else:
        print('image comparison: PASS: tolerance (current -> current); 0/8192 pixels changed; max RGB delta 0/255; max alpha delta 0/255')
    sys.exit(1 if failed else 0)
sys.exit(subprocess.run([sys.executable, *args[2:]]).returncode)
""",
        }
        for name, code in programs.items():
            path = self.bins / name
            path.write_text("#!" + sys.executable + "\n" + code)
            path.chmod(0o755)

    def edit(self, chunk, field, transform, scene="rag"):
        key = "chunk-%03d" % chunk
        _, fields = sc.load(sc.state_files(str(BASE / "state" / scene))[key])
        fields[field] = transform(fields[field])
        self.write_chunk(scene, key, fields)

    def write_chunk(self, scene, key, fields):
        original = sc.state_files(str(self.scenes / scene)).get(key)
        if original:
            Path(original).unlink()
        dump(self.scenes / scene / (key + ".state"), fields)

    def run_gate(self, script=GATES[0], extra_env=None):
        env = dict(os.environ, PATH=str(self.bins) + os.pathsep + os.environ["PATH"], **(extra_env or {}))
        return subprocess.run(["bash", str(script), str(self.bins / "easel")], env=env, capture_output=True, text=True)

    def assert_strict_failure(self, result):
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertNotIn("KNOWN_FAILURE:", result.stdout + result.stderr)
        self.assertIn("wait: state within tolerance", result.stdout)

    def test_current_references_pass_all_six_scenes(self):
        for script in GATES:
            with self.subTest(script=script):
                result = self.run_gate(script)
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                self.assertEqual(result.stdout.count("state within tolerance on every baseline field"), 6)
                self.assertNotIn("KNOWN_FAILURE:", result.stdout + result.stderr)

    def test_rag_volume_latent_and_debug_mutations_are_not_forgiven(self):
        for field, transform in [
            ("wet.vol", lambda f: self.changed_float(f, 1310, 0.1)),
            ("wet.lat", lambda f: self.changed_float(f, 0, 0.1)),
            ("rags.debug", lambda f: self.changed_rag_load(f)),
        ]:
            with self.subTest(field=field):
                self.edit(4, field, transform)
                self.assert_strict_failure(self.run_gate())

    @staticmethod
    def changed_float(field, index, delta):
        dtype, shape, raw = field
        raw = bytearray(raw)
        value = struct.unpack_from("<f", raw, index * 4)[0]
        struct.pack_into("<f", raw, index * 4, value + delta)
        return dtype, shape, bytes(raw)

    @staticmethod
    def changed_rag_load(field):
        dtype, _, raw = field
        text = re.sub(r"load: ([^ ,}]+)", lambda m: "load: " + str(float(m[1]) + 0.1), raw.decode(), count=1)
        assert text.encode() != raw
        return dtype, (len(text.encode()),), text.encode()

    def test_historical_rag_image_signature_is_now_a_failure(self):
        for script in GATES:
            with self.subTest(script=script):
                self.assert_strict_failure(self.run_gate(script, {"FAIL_IMAGE": "rag"}))

    def test_non_rag_image_and_state_failures_remain_failures(self):
        self.assert_strict_failure(self.run_gate(extra_env={"FAIL_IMAGE": "body"}))
        self.edit(1, "studio.seed", lambda _: numeric("u64", 99), scene="body")
        self.assert_strict_failure(self.run_gate())

    def test_missing_rag_chunk_is_a_failure(self):
        Path(sc.state_files(str(self.scenes / "rag"))["chunk-004"]).unlink()
        self.assert_strict_failure(self.run_gate())


if __name__ == "__main__":
    unittest.main()
