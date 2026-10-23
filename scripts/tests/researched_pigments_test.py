from contextlib import redirect_stdout
import io
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import build_pigment_tables as gen
import researched_pigments as researched


class ResearchedPigmentsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.matrix = gen.spectral_array("XYZ_RGB") @ gen.spectral_array("CMF")
        cls.reference = np.linspace(.86, .94, 38)

    def entry(self, name="synthetic", scatter=3.2):
        K = .08 + np.linspace(0, 2, 38) ** 2
        S = np.full(38, scatter)
        reflectance = gen.saunderson(gen.film(K, S, self.reference), False)
        return {
            "name": name,
            "source_nm": researched.WAVELENGTHS.tolist(),
            "source_reflectance": reflectance.tolist(),
            "hiding": researched.opacity(K, S, self.matrix),
            "provenance": {"source": "controlled synthetic spectrum, not vendor data"},
            "source_class": "synthetic",
            "source_rank": "test",
        }, K, S

    def test_recovers_controlled_finite_film_and_opacity(self):
        entry, expected_k, expected_s = self.entry()
        fitted, metadata = researched.build([entry], self.reference, self.matrix)
        K, S = fitted["synthetic"]
        np.testing.assert_allclose(K, expected_k, rtol=1e-8, atol=1e-10)
        np.testing.assert_allclose(S, expected_s, rtol=1e-8)
        row = metadata["entries"]["synthetic"]
        self.assertEqual(row["status"], "modeled")
        self.assertLess(row["source_fit_max_abs_reflectance"], 1e-10)
        self.assertLess(abs(row["hiding_residual"]), 1e-10)
        np.testing.assert_allclose(row["ks_ratio"], K / S)
        self.assertEqual(metadata["quantity_identified"], "reflectance only")
        self.assertIn("not experimental validation", metadata["opacity_constraint"]["evidence"])
        json.dumps(metadata, allow_nan=False)

    def test_single_spectrum_does_not_identify_scattering(self):
        entry, _, _ = self.entry()
        other = dict(entry, name="other", hiding=.9)
        fitted, metadata = researched.build([entry, other], self.reference, self.matrix)
        self.assertEqual(metadata["order"], ["synthetic", "other"])
        self.assertFalse(np.allclose(fitted["synthetic"][1], fitted["other"][1]))
        for name, (K, S) in fitted.items():
            prediction = gen.saunderson(gen.film(K, S, self.reference), False)
            np.testing.assert_allclose(prediction, entry["source_reflectance"], atol=1e-10)
            self.assertLess(metadata["entries"][name]["source_fit_rms_reflectance"], 1e-10)

    def test_source_fit_backing_is_supplied_and_labeled_independently(self):
        entry, _, _ = self.entry()
        reference = np.ones(38)
        fitted, metadata = researched.build(
            [entry], reference, self.matrix, reference_name="assumed ideal white",
            reference_assumption="source paper unknown; ideal white is a modeling assumption")
        self.assertEqual(metadata["reference_sample"], "assumed ideal white")
        self.assertIn("source paper unknown", metadata["reference_assumption"])
        self.assertEqual(metadata["reference_substrate"], reference.tolist())
        self.assertIn("runtime decoding substrate is independent", metadata["reference_scope"])
        K, S = fitted["synthetic"]
        np.testing.assert_allclose(gen.saunderson(gen.film(K, S, reference), False),
                                   entry["source_reflectance"], atol=1e-10)
        self.assertGreater(np.max(np.abs(gen.saunderson(gen.film(K, S, self.reference), False)
                                         - entry["source_reflectance"])), 1e-4)

    def test_diffuse_inversion_is_not_sci_inversion(self):
        inside = np.linspace(.01, .99, 38)
        diffuse = gen.saunderson(inside, False)
        np.testing.assert_allclose(researched.display_inside(diffuse), inside, atol=1e-14)
        self.assertFalse(np.allclose(gen.inside(diffuse), inside))

    def test_native_interpolation_records_endpoint_extension(self):
        output, info = researched.resample([400, 505, 700], [.1, .8, .2])
        self.assertEqual(output[0], .1)
        self.assertEqual(output[-1], .2)
        self.assertAlmostEqual(output[12], .1 + (.8 - .1) * 100 / 105)
        self.assertEqual(info["extended_wavelengths_nm"], [380, 390, 710, 720, 730, 740, 750])

    def test_invalid_spectra_fail_without_clipping_or_partial_coefficients(self):
        good, _, _ = self.entry()
        invalids = [
            dict(good, name="too bright", source_reflectance=[.98] * 38),
            dict(good, name="zero", source_reflectance=[0] * 38),
            dict(good, name="negative", source_reflectance=[-.01] * 38),
            dict(good, name="above one", source_reflectance=[1.01] * 38),
            dict(good, name="nan", source_reflectance=[float("nan")] * 38),
            dict(good, name="wavelengths", source_nm=[400] * 38),
            dict(good, name="shape", source_reflectance=[.2]),
            dict(good, name="hiding", hiding=1),
        ]
        fitted, metadata = researched.build([good, *invalids], self.reference, self.matrix)
        self.assertEqual(list(fitted), ["synthetic"])
        for entry in invalids:
            row = metadata["entries"][entry["name"]]
            self.assertEqual(row["status"], "failed")
            self.assertTrue(row["failure"])
        self.assertEqual(invalids[0]["source_reflectance"], [.98] * 38)
        json.dumps(metadata, allow_nan=False)

    def test_bright_target_with_insufficient_hiding_fails_explicitly(self):
        entry, _, _ = self.entry()
        reference = np.full(38, .4)
        entry.update(source_reflectance=[.8] * 38, hiding=.01)
        fitted, metadata = researched.build([entry], reference, self.matrix)
        self.assertEqual(fitted, {})
        self.assertIn("below attainable minimum", metadata["entries"]["synthetic"]["failure"])

    def test_rejects_invalid_shared_inputs_and_duplicate_names(self):
        entry, _, _ = self.entry()
        with self.assertRaisesRegex(ValueError, "reference"):
            researched.build([entry], np.zeros(38), self.matrix)
        with self.assertRaisesRegex(ValueError, "matrix"):
            researched.build([entry], self.reference, np.zeros((3, 37)))
        with self.assertRaisesRegex(ValueError, "unique"):
            researched.build([entry, entry], self.reference, self.matrix)

    def test_is_deterministic(self):
        entry, _, _ = self.entry()
        first, first_meta = researched.build([entry], self.reference, self.matrix)
        second, second_meta = researched.build([entry], self.reference, self.matrix)
        np.testing.assert_array_equal(first["synthetic"], second["synthetic"])
        self.assertEqual(first_meta, second_meta)

    def gum_ladder(self):
        qp = np.linspace(.3, 2, 38)
        qw = np.linspace(.004, .015, 38)
        scatter = np.linspace(.8, 2, 38)

        def source(r):
            return {"source_nm": researched.WAVELENGTHS.tolist(), "source_reflectance": r.tolist()}

        tints = []
        for name, f in [("light", .1), ("midpoint", .25), ("dark", .5)]:
            q = (f * scatter * qp + (1 - f) * qw) / (f * scatter + 1 - f)
            tints.append(dict(source(gen.km(q)), name=name, pigment_mass_fraction=f))
        return source(gen.km(qp)), source(gen.km(qw)), tints, qp, qw, scatter

    def test_source_tint_validation_is_relative_mass_not_oil_volume(self):
        pigment, white, tints, qp, qw, scatter = self.gum_ladder()
        report = researched.validate_gum_tints(pigment, white, tints, ["midpoint"])
        self.assertEqual(report["status"], "evaluated_source_domain")
        self.assertEqual(report["held_out_names"], ["midpoint"])
        self.assertFalse(report["oil_validation"])
        self.assertIn("not paint volume", report["fraction_basis"])
        self.assertLess(report["held_out_rms_reflectance"], 1e-8)
        coefficients = report["relative_coefficients"]
        np.testing.assert_allclose(coefficients["pigment_s"], scatter, rtol=1e-6)
        np.testing.assert_allclose(coefficients["pigment_k"], qp * scatter, rtol=1e-6)
        np.testing.assert_allclose(coefficients["white_k"], qw, rtol=1e-10)
        json.dumps(report, allow_nan=False)

    def test_unknown_gum_white_is_fitted_only_from_training_tints(self):
        pigment, _, tints, qp, qw, scatter = self.gum_ladder()
        first = researched.validate_gum_tints(pigment, None, tints, ["midpoint"])
        self.assertEqual(first["status"], "evaluated_source_domain")
        self.assertLess(first["held_out_rms_reflectance"], 1e-12)
        np.testing.assert_allclose(first["relative_coefficients"]["white_k"], qw, rtol=1e-10)
        np.testing.assert_allclose(first["relative_coefficients"]["pigment_s"], scatter, rtol=1e-10)
        tints[1]["source_reflectance"] = (np.array(tints[1]["source_reflectance"]) + .02).tolist()
        second = researched.validate_gum_tints(pigment, None, tints, ["midpoint"])
        self.assertEqual(first["relative_coefficients"], second["relative_coefficients"])
        self.assertAlmostEqual(second["held_out_rms_reflectance"], .02)
        self.assertEqual(second["training_names"], ["light", "dark"])
        self.assertIn("not an independent", second["white_evidence"])

    def test_colored_reference_partner_is_not_reported_as_white(self):
        pigment, partner, tints, _, qw, scatter = self.gum_ladder()
        report = researched.validate_gum_tints(pigment, partner, tints, ["midpoint"], partner_label="red lead")
        self.assertEqual(report["status"], "evaluated_source_domain")
        self.assertEqual(report["partner_label"], "red lead")
        self.assertIn("reference partner (red lead) S = 1", report["coefficient_gauge"])
        self.assertEqual(report["partner_evidence"], "supplied source-domain red lead measurement")
        self.assertNotIn("white", json.dumps(report))
        self.assertLess(report["held_out_rms_reflectance"], 1e-8)
        np.testing.assert_allclose(report["relative_coefficients"]["partner_k"], qw, rtol=1e-10)
        np.testing.assert_allclose(report["relative_coefficients"]["pigment_s"], scatter, rtol=1e-6)

    def test_rejects_unidentifiable_source_fit_and_bad_holdout(self):
        pigment, white, tints, _, _, _ = self.gum_ladder()
        report = researched.validate_gum_tints(pigment, pigment, tints, ["midpoint"])
        self.assertEqual(report["status"], "failed")
        self.assertIn("unidentifiable", report["failure"])
        for held_out in [[], ["not a tint"], ["light", "midpoint", "dark"]]:
            report = researched.validate_gum_tints(pigment, white, tints, held_out)
            self.assertEqual(report["status"], "failed")
            self.assertIn("held_out", report["failure"])

    def test_unknown_source_white_requires_two_positive_independent_tints(self):
        pigment, _, tints, _, _, _ = self.gum_ladder()
        report = researched.validate_gum_tints(pigment, None, tints, ["light", "midpoint"])
        self.assertEqual(report["status"], "failed")
        self.assertIn("exactly two", report["failure"])
        tints[2]["source_reflectance"] = [.95] * 38
        report = researched.validate_gum_tints(pigment, None, tints, ["midpoint"])
        self.assertEqual(report["status"], "failed")
        self.assertIn("no positive", report["failure"])


class ResearchedGenerationTests(unittest.TestCase):
    def test_real_sources_regenerate_deterministically_without_legacy_writes(self):
        committed = gen.ROOT / "crates/paint/src/mixer"
        generated_names = ["researched.bin", "researched.json", "researched.rs"]
        legacy_names = ["inverse.bin", "measured.bin", "measured.json", "primaries.json", "reference.bin"]
        legacy = {name: (committed / name).read_bytes() for name in legacy_names}
        expected = {name: (committed / name).read_bytes() for name in generated_names}
        with tempfile.TemporaryDirectory(prefix="researched-pigments-") as directory:
            output = Path(directory)
            for name, contents in legacy.items():
                (output / name).write_bytes(contents)
            with patch.object(gen, "OUT", output), redirect_stdout(io.StringIO()):
                gen.build_researched()
                first = {name: (output / name).read_bytes() for name in generated_names}
                gen.build_researched()
            second = {name: (output / name).read_bytes() for name in generated_names}
            self.assertEqual(first, second)
            self.assertEqual({path.name for path in output.iterdir()}, set(generated_names + legacy_names))
            for name, contents in legacy.items():
                self.assertEqual((output / name).read_bytes(), contents, name)
        for name, contents in legacy.items():
            self.assertEqual((committed / name).read_bytes(), contents, name)

        metadata = json.loads(first["researched.json"])
        coverage = metadata["tube_coverage"]
        order = metadata["order"]
        self.assertEqual(len(coverage), 39)
        self.assertEqual(len({row["name"] for row in coverage}), 39)
        self.assertEqual(len(order), 27)
        self.assertEqual(len(set(order)), 27)
        self.assertEqual(sum(row["status"] == "modeled" for row in metadata["entries"].values()), 27)
        fallback = [row for row in coverage if row["name"] not in order]
        self.assertEqual(len(fallback), 12)
        for row in fallback:
            self.assertEqual(row["runtime_provenance"], "catalog RGB and estimated opacity; no measured spectrum wired")
        coefficients = np.frombuffer(first["researched.bin"], dtype="<f4").reshape(27, 2, 38)
        self.assertTrue(np.all(np.isfinite(coefficients)))
        self.assertTrue(np.all(coefficients > 0))
        np.testing.assert_array_equal(coefficients[:, 1], np.repeat(coefficients[:, 1, :1], 38, axis=1))

        validation = metadata["bath_tint_validation"]
        self.assertEqual(validation["status"], "failed")
        self.assertIn("no positive exact opaque-KM solution", validation["failure"])
        self.assertIn(str(researched.WAVELENGTHS.tolist()), validation["failure"])
        self.assertFalse(validation["oil_validation"])
        self.assertEqual(validation["held_out_names"], ["LW-LTY_50.0-50.0"])
        self.assertEqual(set(validation["training_names"]), {"LW-LTY_33.3-66.7", "LW-LTY_66.7-33.3"})
        self.assertIn("not independent prepared films", validation["repeat_scope"])
        tints = metadata["bath_measured_tints"]
        self.assertEqual({tint["pigment_mass_fraction"] for tint in tints}, {.333, .5, .667})
        self.assertEqual({tint["name"] for tint in tints}, set(validation["training_names"] + validation["held_out_names"]))
        for tint in tints:
            self.assertEqual(len(set(tint["source_files"])), 3)
            self.assertGreater(tint["spot_range_rms"], 0)
            mean = np.array(tint["source_reflectance"])
            self.assertTrue(np.all(np.array(tint["spot_min"]) <= mean))
            self.assertTrue(np.all(mean <= np.array(tint["spot_max"])))
        for name in generated_names:
            self.assertEqual(first[name], expected[name], name)


if __name__ == "__main__":
    unittest.main()
