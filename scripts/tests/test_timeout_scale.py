#!/usr/bin/env python3
"""Focused timeout policy and receipt validation tests; no publishing commands."""
import copy
import importlib.machinery
import importlib.util
import math
import os
from pathlib import Path
import sys
import unittest
from unittest.mock import patch

scripts = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(scripts))
import safeguards_lib as safeguards

spec = importlib.util.spec_from_loader("merge_candidate", importlib.machinery.SourceFileLoader("merge_candidate", str(scripts / "merge_candidate")))
merger = importlib.util.module_from_spec(spec)
spec.loader.exec_module(merger)


class TimeoutScaleTests(unittest.TestCase):
    def test_defaults(self):
        with patch.dict(os.environ, {}, clear=True), patch.object(sys, "platform", "linux"):
            for cpus, expected in ((1, 16), (2, 8), (4, 4), (8, 4), (16, 4), (32, 4)):
                with self.subTest(cpus=cpus), patch.object(os, "sched_getaffinity", return_value=set(range(cpus)), create=True):
                    self.assertEqual(safeguards.timeout_scale(), expected)
            with patch.object(os, "sched_getaffinity", side_effect=OSError, create=True), patch.object(os, "cpu_count", return_value=8):
                self.assertEqual(safeguards.timeout_scale(), 4)
            with patch.object(os, "sched_getaffinity", side_effect=AttributeError, create=True), patch.object(os, "cpu_count", return_value=None):
                self.assertEqual(safeguards.timeout_scale(), 16)
        with patch.dict(os.environ, {}, clear=True), patch.object(sys, "platform", "darwin"):
            self.assertEqual(safeguards.timeout_scale(), 1)

    def test_overrides_and_invalid_types(self):
        for value in ("1", "4", "1.5", "16"):
            with self.subTest(value=value), patch.dict(os.environ, {"SCRIPTS_TEST_TIMEOUT_SCALE": value}):
                self.assertEqual(safeguards.timeout_scale(), float(value))
        for value in ("", "nan", "inf", "-inf", "0", "0.5", "16.1", "true", "bogus"):
            with self.subTest(value=value), patch.dict(os.environ, {"SCRIPTS_TEST_TIMEOUT_SCALE": value}):
                with self.assertRaises(safeguards.Refuse):
                    safeguards.timeout_scale()
        for value in (True, False, None, "4", math.nan, math.inf, -1, 17, 10**500):
            with self.subTest(value=value), self.assertRaises(safeguards.Refuse):
                safeguards.validate_timeout_scale(value)


class ReceiptScaleTests(unittest.TestCase):
    def setUp(self):
        self.manifest = dict(list=safeguards.DEFAULT_LIST, list_sha256="a" * 64,
                             runner=safeguards.RUNNER, runner_sha256="b" * 64, steps=[])
        for kind, name, limit, least in (("build", "build", 600.0, 0), ("script", "check", 60.0, 1)):
            self.manifest["steps"].append(dict(kind=kind, name=name, command="true", limit=limit, least=least, group=None, known=None))

    def receipt(self, scale=1, explicit=True):
        phases = []
        for phase in safeguards.PHASES:
            summary = dict(mode="all", phase=phase, verdict="pass", list=self.manifest["list"],
                           list_sha256=self.manifest["list_sha256"], runner_sha256=self.manifest["runner_sha256"], steps=[])
            if explicit:
                summary["timeout_scale"] = scale
            for step in safeguards.phase_steps(self.manifest["steps"], phase):
                summary["steps"].append(dict(step, limit=step["limit"] * scale, exit=0, ok=True,
                                             known_failure=False, timed_out=False, failed=0, tests_run=step["least"]))
            phases.append(dict(phase=phase, limit=600 * scale, ran=True, exit=0, verdict="pass", summary=summary))
        receipt = dict(format=safeguards.RECEIPT_FORMAT, candidate=dict(commit="commit", tree="tree"), verdict="pass", exit=0,
                       cleanup_errors=[], lockrun=dict(sha256=safeguards.LOCKRUN_SHA256), manifest=copy.deepcopy(self.manifest),
                       phase_limit_s=600 * scale, checkout=dict(clean_before=True, **{key: dict(files_tree=1) for key in ("before", "between", "after")}), phases=phases)
        if explicit:
            receipt["timeout_scale"] = scale
        return receipt

    def problems(self, receipt):
        with patch.object(merger, "manifest_at", return_value=self.manifest), patch.object(merger, "git", return_value="file\0"), patch.object(merger, "checkout_problems", return_value=[]):
            return merger.receipt_problems("fixture", "commit", "tree", receipt)

    def test_scaled_and_legacy_receipts(self):
        for scale in (1, 1.5, 4, 16):
            with self.subTest(scale=scale):
                receipt = self.receipt(scale)
                original = copy.deepcopy(self.manifest)
                self.assertEqual(self.problems(receipt), [])
                self.assertEqual(self.manifest, original)
        self.assertEqual(self.problems(self.receipt(explicit=False)), [])
        legacy_child = self.receipt(4)
        for phase, legacy in zip(legacy_child["phases"], self.receipt(explicit=False)["phases"]):
            phase["summary"] = legacy["summary"]
        self.assertEqual(self.problems(legacy_child), [])

    def test_invalid_receipt_and_summary_scales(self):
        for value in (True, False, None, "4", math.nan, math.inf, 0, 17, 10**500):
            for location in ("receipt", "summary"):
                with self.subTest(value=value, location=location):
                    receipt = self.receipt(4)
                    target = receipt if location == "receipt" else receipt["phases"][1]["summary"]
                    target["timeout_scale"] = value
                    self.assertTrue(self.problems(receipt))

    def test_forged_mismatched_and_missing_scale(self):
        for location in ("receipt", "summary"):
            receipt = self.receipt(4)
            target = receipt if location == "receipt" else receipt["phases"][1]["summary"]
            target["timeout_scale"] = 8
            self.assertTrue(any("differs from the trusted scale" in problem for problem in self.problems(receipt)))
        receipt = self.receipt(4)
        del receipt["timeout_scale"]
        self.assertTrue(self.problems(receipt))
        receipt = self.receipt(4)
        del receipt["phases"][1]["summary"]["timeout_scale"]
        self.assertTrue(any("summary's limit" in problem for problem in self.problems(receipt)))

    def test_raw_explicit_phase_seconds_and_budget_bounds(self):
        receipt = self.receipt(4)
        receipt["phase_limit_s"] = 100
        for phase in receipt["phases"]:
            phase["limit"] = 100
        self.assertEqual(self.problems(receipt), [])
        for value in (True, math.nan, math.inf, 0, 2401):
            for location in ("receipt", "phase"):
                with self.subTest(value=value, location=location):
                    receipt = self.receipt(4)
                    if location == "receipt":
                        receipt["phase_limit_s"] = value
                    else:
                        receipt["phases"][1]["limit"] = value
                    self.assertTrue(self.problems(receipt))
        receipt = self.receipt(4)
        receipt["phases"][1]["limit"] = 2000
        self.assertTrue(any("not the recorded phase_limit_s" in problem for problem in self.problems(receipt)))
        receipt = self.receipt(4)
        del receipt["phase_limit_s"]
        self.assertTrue(self.problems(receipt))
        for value in (True, math.nan, math.inf, 241):
            receipt = self.receipt(4)
            receipt["phases"][1]["summary"]["steps"][0]["limit"] = value
            self.assertTrue(self.problems(receipt))
        self.manifest["steps"][1]["limit"] = 1e308
        receipt = self.receipt(16)
        self.assertTrue(self.problems(receipt))

    def test_known_failure_stays_unmergeable(self):
        self.manifest["steps"][1]["known"] = dict(exit=4, regex="known", text="pre-existing")
        receipt = self.receipt(4)
        receipt.update(verdict="known_failure", exit=4)
        phase = receipt["phases"][1]
        phase.update(verdict="known_failure", exit=4)
        phase["summary"]["verdict"] = "known_failure"
        phase["summary"]["steps"][0].update(exit=4, ok=False, known_failure=True)
        problems, known = safeguards.phase_problems(phase["summary"], self.manifest, "check", 4)
        self.assertEqual((problems, known), ([], ["check"]))
        self.assertTrue(any("not mergeable" in problem for problem in self.problems(receipt)))


if __name__ == "__main__":
    unittest.main()
