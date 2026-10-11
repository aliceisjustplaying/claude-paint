import importlib.machinery
import importlib.util
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch


SCRIPT = Path(__file__).resolve().parents[1] / "test-changed"
loader = importlib.machinery.SourceFileLoader("test_changed_runner", str(SCRIPT))
spec = importlib.util.spec_from_loader(loader.name, loader)
runner = importlib.util.module_from_spec(spec)
loader.exec_module(runner)
gate_loader = importlib.machinery.SourceFileLoader("gate_runner", str(SCRIPT.with_name("test")))
gate_spec = importlib.util.spec_from_loader(gate_loader.name, gate_loader)
gate = importlib.util.module_from_spec(gate_spec)
gate_loader.exec_module(gate)
sys.path.insert(0, str(SCRIPT.parent))
import safeguards_lib as safeguards


class ChangedTests(unittest.TestCase):
    def test_engine_includes_downstream_quick_checks(self):
        names, targets = runner.select(["crates/paint/src/rag.rs"])
        self.assertTrue(runner.ENGINE <= names)
        self.assertIn("build-test-binaries", names)
        self.assertNotIn("build-gate-easel", names)
        self.assertFalse(targets)
        self.assertTrue(names.isdisjoint(runner.STUDIO))

    def test_easel_changes_do_not_rerun_unmodified_paint_unit_tests(self):
        steps, full = runner.plan(["crates/easel/src/api/draw.rs"])
        self.assertFalse(full)
        self.assertIn("cargo-easel", {step[1] for step in steps})
        self.assertNotIn("cargo-test", {step[1] for step in steps})
        self.assertIn("--package easel", steps[-1][5])

    def test_rust_integration_selects_its_binary(self):
        names, targets = runner.select(["crates/easel/tests/determinism.rs"])
        self.assertEqual(names, set())
        self.assertEqual(targets, {("easel", "determinism")})

    def test_thinner_fixtures_use_the_approved_release_runner(self):
        names, targets = runner.select(["crates/paint/tests/thinner_support/mod.rs"])
        self.assertIn("thinner-quick", names)
        self.assertFalse(targets)
        names, _ = runner.select(["crates/paint/src/thinner.rs"])
        self.assertIn("thinner-quick", names)
        self.assertIn("build-test-binaries", names)

    def test_studio_selects_python_and_browsers_without_rust_builds(self):
        names, targets = runner.select(["studio/index.html", "studio/export_static.py"])
        self.assertEqual(names, runner.STUDIO)
        self.assertFalse(targets)

    def test_harness_and_runner_are_independent(self):
        self.assertEqual(runner.select(["harness/painter/pace.ts"])[0], {"node-harness", "painter-build-inness"})
        self.assertEqual(runner.select(["notes/round24/runner/runner.py"])[0], {"runner-round24", "reader-scope"})

    def test_feature_paths_include_feature_matrix(self):
        names, _ = runner.select(["crates/paint/build.rs"])
        self.assertTrue({"box-features", "box-tubes", "painter-build-inness"} <= names)

    def test_baseline_changes_are_not_treated_as_documentation(self):
        names, _ = runner.select(["notes/thinner/baseline/MANIFEST.txt"])
        self.assertIn("baseline-package", names)
        self.assertIn("state-compare", names)

    def test_known_scripts_select_their_regressions(self):
        expected = {
            "scripts/test": {"test-runner", "test-changed"},
            "scripts/test-changed": {"test-runner", "test-changed"},
            "scripts/lockrun": {"lockrun", "test-runner"},
            "scripts/strip_sources": {"script-python"},
            "scripts/replay_clip": {"replay-env", "check-live", "replay-clip"},
            "scripts/tests/cargo_tests.sh": {"cargo-test", "build-test-binaries"},
        }
        for path, names in expected.items():
            with self.subTest(path=path):
                self.assertEqual(runner.select([path])[0], names)

    def test_unknown_and_cross_cutting_paths_fall_back_to_full(self):
        for path in ["future/new.py", "scripts/new-check", "Cargo.lock", "Cargo.toml", "crates/easel/Cargo.toml", ".cargo/config.toml", "notes/speed/test_lists/all.tsv", ".github/workflows/test.yml"]:
            with self.subTest(path=path):
                self.assertIsNone(runner.select([path])[0])

    def test_docs_and_empty_changes_need_no_tests(self):
        self.assertEqual(runner.select(["README.md", "notes/testing.md", "AGENTS.md"]), (set(), set()))
        self.assertEqual(runner.plan([]), ([], False))

    def test_broad_fallback_keeps_every_iteration_check_and_reserves_slow_checks_for_final(self):
        steps, full = runner.plan(["unknown.py"])
        expected = [line.split("\t")[:6] for line in runner.FULL.read_text().splitlines() if line and not line.startswith("#") and line.split("\t")[1] not in {"slow-tests-by-name", "slow-thin-blend", "thinner-all"}]
        expected += [line.split("\t")[:6] for line in (runner.FULL.parent / "fast.tsv").read_text().splitlines() if line.startswith("script\tthinner-quick\t")]
        self.assertTrue(full)
        self.assertEqual(steps, expected)

    def test_targeted_plan_is_deduplicated_and_has_no_parallel_groups(self):
        steps, full = runner.plan(["crates/paint/src/rag.rs", "crates/easel/src/session.rs"])
        self.assertFalse(full)
        self.assertEqual(len(steps), len({step[1] for step in steps}))
        self.assertTrue(all(len(step) == 6 for step in steps))
        self.assertTrue(any(step[1] == "cargo-test" for step in steps))

    def test_integration_command_and_minimum_are_specific(self):
        steps, full = runner.plan(["crates/paint/tests/look_rim.rs"])
        self.assertFalse(full)
        step = steps[-1]
        self.assertEqual(step[5], "cargo test --profile gate -p paint --test look_rim")
        self.assertEqual(step[3], "1")
        self.assertEqual(steps[0][5], "cargo test --profile gate -p paint --test look_rim --no-run")

    def test_git_selection_includes_committed_dirty_and_untracked_paths(self):
        with patch.object(runner.subprocess, "check_output", side_effect=[b"studio/index.html\0space name.ts\0", b"studio/index.html\0scripts/test\0", b"new\nname.py\0"]) as call:
            self.assertEqual(runner.changed_paths("base"), ["new\nname.py", "scripts/test", "space name.ts", "studio/index.html"])
        self.assertEqual(call.call_args_list[0].args[0][-1], "base...HEAD")
        self.assertEqual(call.call_args_list[1].args[0][-1], "HEAD")
        self.assertEqual(call.call_args_list[2].args[0][-3:], ["--others", "--exclude-standard", "-z"])

    def test_missing_base_does_not_silently_skip_committed_changes(self):
        with patch.object(runner.subprocess, "check_output", side_effect=subprocess.CalledProcessError(128, "git")):
            with self.assertRaises(subprocess.CalledProcessError):
                runner.changed_paths("missing")

    def test_dry_run_does_not_start_tests(self):
        result = subprocess.run([str(SCRIPT), "--dry-run", "studio/index.html"], text=True, capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("studio-gallery", result.stdout)
        self.assertNotIn("build-gate-easel", result.stdout)

    def test_node_totals_are_real_test_counts(self):
        output = "# tests 9\n# pass 6\n# fail 1\n# cancelled 1\n# skipped 1\n"
        self.assertEqual(gate.count({"kind": "node"}, output), (8, 6, 2, 1))

    def test_node_missing_output_cannot_pass_a_positive_minimum(self):
        self.assertEqual(gate.count({"kind": "node"}, ""), (0, 0, 0, 0))

    def test_gate_accepts_the_updated_full_list(self):
        steps = gate.read_list(runner.FULL)
        self.assertEqual(len(steps), len({step["name"] for step in steps}))
        self.assertEqual(next(step["kind"] for step in steps if step["name"] == "node-harness"), "node")

    def test_candidate_receipts_accept_the_same_gate_schema(self):
        steps = safeguards.parse_test_list(runner.FULL.read_bytes(), str(runner.FULL))
        self.assertEqual(steps, gate.read_list(runner.FULL))
        self.assertFalse(safeguards.manifest_problems(steps))
        node = next(step for step in steps if step["kind"] == "node")
        self.assertTrue(safeguards.manifest_problems([{**node, "least": 0}]))

    def replay_builder(self, profile=None, no_build=False):
        import os
        import shutil

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            scripts = root / "scripts"
            scripts.mkdir()
            shutil.copyfile(SCRIPT.with_name("replay_easel"), scripts / "replay_easel")
            (scripts / "replay_easel").chmod(0o755)
            cargo = root / "cargo"
            cargo.write_text('#!/bin/sh\nprintf "%s\\n" "$*" >>"$TEST_CALLS"\n[ "$1" != metadata ] || printf \'{"target_directory":"/private/target"}\\n\'\n')
            cargo.chmod(0o755)
            env = dict(os.environ, PATH=str(root) + ":" + os.environ["PATH"], TEST_CALLS=str(root / "calls"))
            env.pop("CLAUDE_PAINT_TEST_PROFILE", None)
            if profile:
                env["CLAUDE_PAINT_TEST_PROFILE"] = profile
            result = subprocess.run([str(scripts / "replay_easel"), *(["--no-build"] if no_build else [])], env=env, text=True, capture_output=True)
            calls = (root / "calls").read_text() if (root / "calls").exists() else ""
            return result, calls

    def test_replay_builder_preserves_production_release_default(self):
        result, calls = self.replay_builder()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout.strip(), "/private/target/release/easel")
        self.assertIn("build --profile release -q -p easel", calls)

    def test_replay_builder_reuses_gate_artifacts(self):
        result, calls = self.replay_builder("gate")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout.strip(), "/private/target/gate/easel")
        self.assertIn("build --profile gate -q -p easel", calls)
        result, calls = self.replay_builder("gate", no_build=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertNotIn("build", calls)

    def test_replay_builder_rejects_unknown_profile(self):
        result, calls = self.replay_builder("unexpected")
        self.assertEqual(result.returncode, 2)
        self.assertFalse(calls)


if __name__ == "__main__":
    unittest.main()
