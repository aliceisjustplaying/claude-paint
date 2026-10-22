#!/usr/bin/env python3
"""Current baseline dumps reconstruct the mixer-tagged checkpoint and its digest."""

import importlib.util
from pathlib import Path
import struct
import subprocess
import sys
import tempfile
import unittest

sys.dont_write_bytecode = True
from baseline_state_compare_test import dump, numeric

TOOL = Path(__file__).resolve().parents[2] / "notes/thinner/baseline/tools/checkpoint_from_dump.py"
sys.path.insert(0, str(TOOL.parent))
spec = importlib.util.spec_from_file_location("checkpoint_from_dump", TOOL)
checkpoint = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checkpoint)


def scene():
    fields = {
        "frame": numeric("u64", 1, 1, 0, 0, 1, 1, 0, 0, 1, 1),
        "scale": numeric("f32", 0.001),
        "mm_per_unit": numeric("f32", 0.2),
        "linen": numeric("f64", 0.0, 0.0, 0.0, 0.0, 0.0),
        "linen_seed": numeric("u64", 0),
        "surf_gen": numeric("u64", 0),
        "wet.current": numeric("u32", 1),
        "wet.dirty": numeric("u64", 1, 0, 0, 1, 1),
        "color": numeric("f32", 0.2, 0.4, 0.6),
        "height": numeric("f32", 1.0),
        "film": numeric("f32", 0.0),
        "wet.vol": numeric("f32", 1.0),
        "wet.lat": numeric("f32", *[0.1] * 7),
        "wet.hide": numeric("f32", 0.5, 0.6, 1.0),
        "wet.stroke": numeric("u32", 1),
        "wet.touched": numeric("u32", 1),
        "clock.now": numeric("f64", 0.0),
        "clock.mark": numeric("u32", 0),
        "clock.tacky": numeric("u64", 0, 0, 0, 0, 0),
        "ground_um": numeric("f32", 0.0),
        "wet.cover": numeric("f32", 1.0),
        "drawing": numeric("f32"),
        "hand_slice": numeric("f32", 0.0, 0.0),
        "tally": numeric("u64", *[0] * 9),
        "engine": numeric("u32", 3),
        "wet.solvent": numeric("f32", 0.0),
    }
    fields.update(("clock." + name, numeric("f32")) for name in ("cure", "lev", "seen", "sub", "srate", "th"))
    return fields


class BaselineCheckpointTests(unittest.TestCase):
    def test_current_mixer_envelope_and_solvent_section_are_reconstructed(self):
        fields = scene()
        rebuilt = checkpoint.rebuild(fields)
        self.assertEqual(rebuilt[:40], b"PAINTC19" + struct.pack("<4Q", 0, 1, 9, 0))
        self.assertEqual(rebuilt[-12:], struct.pack("<Qf", 3, 0.0))
        fields["wet.solvent"] = numeric("f32", 0.25)
        changed = checkpoint.rebuild(fields)
        self.assertEqual(changed[:-4], rebuilt[:-4])
        self.assertEqual(changed[-4:], struct.pack("<f", 0.25))

    def test_digest_verification_rejects_mutated_fields_or_digest(self):
        fields = scene()
        with tempfile.TemporaryDirectory() as tmp:
            source, digest = Path(tmp) / "chunk-001.state", Path(tmp) / "digest.txt"
            dump(source, fields)
            digest.write_text("chunk=1 canvas=%016x\n" % checkpoint.fnv1a(checkpoint.rebuild(fields)))
            args = [sys.executable, str(TOOL), str(source), "--digest", str(digest)]
            result = subprocess.run(args, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            for name, value in [("wet.lat", numeric("f32", *[0.2] * 7)), ("wet.solvent", numeric("f32", 1e-15))]:
                with self.subTest(field=name):
                    dump(source, dict(fields, **{name: value}))
                    result = subprocess.run(args, capture_output=True, text=True)
                    self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
                    self.assertIn("MISMATCH", result.stdout)
            dump(source, fields)
            digest.write_text("chunk=1 canvas=0000000000000000\n")
            self.assertEqual(subprocess.run(args, capture_output=True).returncode, 1)

    def test_unsupported_or_incomplete_layouts_are_refused(self):
        for change in [
            {"engine": numeric("u32", 4)},
            {"drawing": numeric("f32", 1.0)},
            {"wet.parts.offsets": numeric("u64", 0, 0)},
            {"wet.solvent": numeric("f64", 0.0)},
        ]:
            with self.subTest(change=change), self.assertRaises(ValueError):
                checkpoint.rebuild(dict(scene(), **change))
        fields = scene()
        del fields["wet.solvent"]
        with self.assertRaises(KeyError):
            checkpoint.rebuild(fields)


if __name__ == "__main__":
    unittest.main()
