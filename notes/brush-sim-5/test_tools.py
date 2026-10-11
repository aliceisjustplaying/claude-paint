#!/usr/bin/env python3
import csv
import json
import math
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

import numpy as np
from PIL import Image

import common
import mesh
import morning
import publish


def synthetic_artifacts(directory, engine, volume=common.TARGET):
    directory.mkdir(parents=True, exist_ok=True)
    for view in ("gallery", "color"):
        Image.new("RGB", common.NATIVE, (17, 31, 47)).save(directory / f"{view}.png")
    with (directory / "materials.csv").open("w", newline="") as stream:
        writer = csv.writer(stream)
        writer.writerow(["chunk_index", "native_width_px", "native_height_px", "recorded_engine", common.MATERIAL, common.SOLVENT])
        for chunk in range(1, 5):
            writer.writerow([chunk, 1200, 800, engine, 1 + volume * (chunk - 1) / 3, 0])
    (directory / "study.lua").write_text("-- SYNTHETIC FIXTURE, never a real outcome\n")
    (directory / "digests.txt").write_text("SYNTHETIC FIXTURE\n")
    (directory / "final.ckpt").write_bytes(b"SYNTHETIC NON-CANVAS FIXTURE")


class SyntheticTests(unittest.TestCase):
    def setUp(self):
        common.SCRATCH.mkdir(parents=True, exist_ok=True)
        self.temporary = tempfile.TemporaryDirectory(prefix="synthetic-tools-", dir=common.SCRATCH)
        self.root = Path(self.temporary.name)

    def tearDown(self):
        self.temporary.cleanup()

    def test_finite_load_transform_preserves_gestures(self):
        source = (common.REPO / "notes/brush-sim-3/morning.lua").read_text()
        text = morning.study(source, 20, 1.546)
        self.assertIn("--@ engine 20", text)
        self.assertIn("local dip=math.min(n,1)", text)
        self.assertEqual(text.count(":stroke("), 18)
        self.assertEqual(text.count("finite_load("), source.count(":load(") + 1)
        for left, right in zip(source.splitlines(), [line for line in text.splitlines() if not line.startswith(("function finite_load", "  ", "end"))]):
            if ":load(" not in left and "--@ engine" not in left:
                self.assertEqual(left, right)
        for factor in (0, -1, 9, math.inf, math.nan):
            with self.assertRaises(ValueError):
                morning.study(source, 18, factor)

    def test_volume_scope_and_validation(self):
        synthetic_artifacts(self.root, 18)
        self.assertEqual(common.materials(self.root / "materials.csv", 18)["deposited_volume"], common.TARGET)
        with self.assertRaises(ValueError):
            common.materials(self.root / "materials.csv", 19)
        Image.new("RGB", (12, 8)).save(self.root / "bad.png")
        with self.assertRaises(ValueError):
            common.native(self.root / "bad.png")
        with self.assertRaises(ValueError):
            common.scratch(common.REPO / "notes/brush-sim-5")
        with self.assertRaises(ValueError):
            common.scratch("/home/user/.capy/drive/captain-stillwet/renders")

    def test_real_source_identity_uses_runtime_inputs_not_sparse_archive(self):
        identity = morning.source_identity()
        self.assertIn("crates/paint/src/bristle.rs", identity["engine_source_files_sha256"])
        self.assertIn("crates/easel/src/prelude.lua", identity["engine_source_files_sha256"])
        self.assertFalse(any(name.startswith("archive/") for name in identity["engine_source_files_sha256"]))

    def candidate_setup(self):
        binary = self.root / "SYNTHETIC-not-executable.txt"
        binary.write_text("SYNTHETIC mock process only\n")
        saved = {"runs": [], "selected": {"baseline": {"initial_volume": 1}},
                 "calibration_identity": {"binary_sha256": common.sha256(binary), "engine_source_sha256": "synthetic-source"}}
        source = (common.REPO / "notes/brush-sim-3/morning.lua").read_text()
        return binary, saved, source

    def test_candidate_cache_and_completed_process_recovery(self):
        binary, saved, source = self.candidate_setup()

        def fake_process(command, **kwargs):
            directory = Path(command[2]).parent
            lua = (directory / "study.lua").read_bytes()
            synthetic_artifacts(directory, 18)
            (directory / "study.lua").write_bytes(lua)
            return SimpleNamespace(returncode=0)

        with patch.object(morning, "source_identity", return_value={"engine_source_sha256": "synthetic-source"}), patch.object(morning.subprocess, "run", side_effect=fake_process) as process:
            first = morning.run_candidate(self.root, saved, binary, source, "lean5", 1.546)
            second = morning.run_candidate(self.root, saved, binary, source, "lean5", 1.546)
            self.assertIs(first, second)
            self.assertEqual(process.call_count, 1)
            first["status"] = "launching"
            recovered = morning.run_candidate(self.root, saved, binary, source, "lean5", 1.546)
            self.assertEqual(recovered["status"], "completed")
            self.assertEqual(process.call_count, 1)
            self.assertTrue(Path(recovered["final_canvas"]).exists())
            Image.new("RGB", common.NATIVE, (17, 31, 48)).save(Path(recovered["directory"]) / "color.png")
            with self.assertRaises(ValueError):
                morning.run_candidate(self.root, saved, binary, source, "lean5", 1.546)
            self.assertEqual(process.call_count, 1)

    def test_interrupted_candidate_is_never_automatically_replayed(self):
        binary, saved, source = self.candidate_setup()
        with patch.object(morning, "source_identity", return_value={"engine_source_sha256": "synthetic-source"}), patch.object(morning.subprocess, "run", side_effect=KeyboardInterrupt) as process:
            with self.assertRaises(KeyboardInterrupt):
                morning.run_candidate(self.root, saved, binary, source, "lean5", 1.546)
            with self.assertRaisesRegex(RuntimeError, "refusing a second replay"):
                morning.run_candidate(self.root, saved, binary, source, "lean5", 1.546)
            self.assertEqual(process.call_count, 1)

    def test_source_drift_blocks_before_candidate_invocation(self):
        binary, saved, source = self.candidate_setup()
        with patch.object(morning, "source_identity", return_value={"engine_source_sha256": "changed"}), patch.object(morning.subprocess, "run") as process:
            with self.assertRaisesRegex(RuntimeError, "no replay launched"):
                morning.run_candidate(self.root, saved, binary, source, "lean5", 1.546)
            process.assert_not_called()
            self.assertEqual(saved["runs"], [])

    def test_failed_candidate_is_never_automatically_replayed(self):
        binary, saved, source = self.candidate_setup()
        with patch.object(morning, "source_identity", return_value={"engine_source_sha256": "synthetic-source"}), patch.object(morning.subprocess, "run", return_value=SimpleNamespace(returncode=7)) as process:
            with self.assertRaisesRegex(RuntimeError, "invocation failed"):
                morning.run_candidate(self.root, saved, binary, source, "lean5", 1.546)
            with self.assertRaisesRegex(RuntimeError, "previous invocation failed"):
                morning.run_candidate(self.root, saved, binary, source, "lean5", 1.546)
            self.assertEqual(process.call_count, 1)
            self.assertEqual(saved["runs"][0]["invocations"], 1)

    def test_accepted_candidates_and_controls_are_never_replayed(self):
        binary, saved, _ = self.candidate_setup()
        for name, engine in {"baseline": 5, "lean4": 15, **common.VARIANTS}.items():
            directory = self.root / name
            synthetic_artifacts(directory, engine)
            saved["selected"][name] = {"variant": name, "engine": engine, "factor": 1,
                                       "directory": str(directory), "invocations": 1, "checkpoint_available": name != "baseline",
                                       "artifact_sha256": common.artifacts(directory), **common.materials(directory / "materials.csv", engine)}
        del saved["calibration_identity"]
        common.write_json(self.root / "receipt.json", saved)
        with patch.object(morning, "source_identity", return_value={"git_commit": "synthetic-commit", "engine_source_sha256": "synthetic-source"}), patch.object(morning.subprocess, "run") as process:
            morning.calibrate(self.root, binary, 22)
            morning.calibrate(self.root, binary, 22)
            process.assert_not_called()
            completed = json.loads((self.root / "receipt.json").read_text())
            self.assertEqual(completed["status"], "matched")
            self.assertEqual(completed["candidate_invocation_count"], 0)

    def test_grid_band_rms_and_support_correlation(self):
        y, x = np.indices((280, 520))
        signal = 3 * np.sin(2 * np.pi * mesh.FREQUENCIES[0] * x)
        band = mesh.band_rms(signal + 12 + 0.01 * x + 0.03 * y, 0.012)
        self.assertAlmostEqual(band["combined"], 3 / math.sqrt(2), delta=0.03)
        self.assertLess(band["0.2133333"], 0.001)
        self.assertAlmostEqual(mesh.correlation(signal, signal * 2 + 7), 1)
        self.assertIsNone(mesh.correlation(signal, np.zeros_like(signal)))

    def test_narrow_lattice_spots_and_projected_support_correlation(self):
        y, x = np.indices((280, 520))
        checker = 3 * np.cos(2 * np.pi * 55 * x / 520) * np.cos(2 * np.pi * 30 * y / 280)
        thread = 4 * np.sin(2 * np.pi * 111 * x / 520)
        background = 300 * np.sin(2 * np.pi * 3 * x / 520)
        support = checker + background
        visible = 2 * checker + 0.7 * thread
        projected = mesh.lattice_projection(visible + 500)
        self.assertAlmostEqual(float(np.sqrt(np.mean(projected ** 2))), math.sqrt(9 + 0.49 * 8), places=10)
        projected_correlation = mesh.correlation(mesh.lattice_projection(support), projected)
        self.assertAlmostEqual(projected_correlation, math.sqrt(9 / (9 + 0.49 * 8)), places=10)
        self.assertLess(abs(mesh.correlation(support, visible)), 0.02)
        off_spot = np.sin(2 * np.pi * 111 * x / 520) * np.cos(2 * np.pi * 42 * y / 280)
        self.assertLess(float(np.sqrt(np.mean(mesh.lattice_projection(off_spot) ** 2))), 1e-12)
        arrays = {name: np.zeros_like(checker) for name in mesh.FIELDS}
        arrays.update(dry_height_um=support, visible_surface_um=visible)
        summary = mesh.summarize(arrays, 0.012)
        self.assertIn("grid_band_rms", summary["visible_surface_um"])
        self.assertAlmostEqual(summary["visible_surface_um"]["woven_lattice_spot_dry_support_pearson"], projected_correlation)
        self.assertIsNone(summary["solvent_um"]["woven_lattice_spot_dry_support_pearson"])

    def test_surface_csv_coordinates_must_cover_roi_once(self):
        path = self.root / "SYNTHETIC-surface.csv"
        with path.open("w", newline="") as stream:
            writer = csv.DictWriter(stream, fieldnames=["x_px", "y_px", *mesh.FIELDS])
            writer.writeheader()
            for y in range(3):
                for x in range(4):
                    writer.writerow({"x_px": x, "y_px": y, **{name: x + y for name in mesh.FIELDS}})
        arrays = mesh.read_grid(path, (0, 0, 4, 3))
        self.assertEqual(arrays["gallery_y"].shape, (3, 4))
        with path.open("a") as stream:
            stream.write("0,0,0,0,0,0,0,0\n")
        with self.assertRaisesRegex(ValueError, "Duplicate"):
            mesh.read_grid(path, (0, 0, 4, 3))

    def test_full_publication_pixels_manifest_and_no_overwrite(self):
        saved = {"fixture_kind": "SYNTHETIC TEST ONLY; no real brush outcomes", "selected": {}}
        for name, engine in {"baseline": 5, "lean4": 15, **common.VARIANTS}.items():
            directory = self.root / name
            synthetic_artifacts(directory, engine)
            saved["selected"][name] = {"variant": name, "engine": engine, "factor": 1,
                                       "directory": str(directory), "invocations": 1, "checkpoint_available": name != "baseline",
                                       "artifact_sha256": common.artifacts(directory), **common.materials(directory / "materials.csv", engine)}
        common.write_json(self.root / "receipt.json", saved)
        destination = self.root / "SYNTHETIC-publication"
        count = publish.build(self.root, destination, extra_crops=[("end-edge", (620, 100, 820, 380))])
        self.assertGreater(count, 40)
        self.assertFalse(list(destination.glob("*.ckpt")))
        for line in (destination / "iteration-5-SHA256SUMS").read_text().splitlines():
            digest, name = line.split("  ", 1)
            self.assertEqual(common.sha256(destination / name), digest)
        with Image.open(destination / "iteration-5-dark-wall-comparison.png") as sheet:
            with Image.open(destination / "iteration-5-baseline-dark-wall-gallery.png") as crop:
                self.assertEqual(sheet.crop((0, 64, 520, 344)).tobytes(), crop.tobytes())
        with self.assertRaisesRegex(ValueError, "never overwrites"):
            publish.build(self.root, destination)


if __name__ == "__main__":
    print("Focused tests use explicitly labeled synthetic fixtures and mocked process invocations only", flush=True)
    unittest.main()
