import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


REPO = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("cargo_test_executables", REPO / "scripts/cargo_test_executables.py")
PARSER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PARSER)


def artifact(name="easel", kind="bin", test=False, executable="/bin/true"):
    return {"reason": "compiler-artifact", "target": {"name": name, "kind": [kind]},
            "profile": {"test": test}, "executable": executable}


class CargoTestExecutablesTests(unittest.TestCase):
    def test_mixed_diagnostics_and_artifacts(self):
        lines = ["warning: compiler wrapper could not read cache; compiling locally\n", "\n",
                 "[libc 0.2.189] cargo:rerun-if-changed=build.rs\n",
                 "[libc 0.2.189] cargo:rerun-if-env-changed=LIBC_BUILD_VERBOSE\n",
                 "[libc 0.2.189] cargo:rustc-cfg=freebsd12\n",
                 "[libc 0.2.189] cargo:rustc-check-cfg=cfg(emscripten_old_stat_abi)\n",
                 "{non-JSON diagnostic}\n", json.dumps({"reason": "compiler-message"}),
                 json.dumps({"reason": "build-script-executed"}),
                 json.dumps(artifact(executable=None)), json.dumps(artifact()),
                 json.dumps(artifact("paint", "lib", True, "/tmp/paint test")),
                 json.dumps({"reason": "build-finished", "success": True}), "null", "[]"]
        self.assertEqual(list(PARSER.executables(lines)),
                         ["bin easel\t/bin/true", "lib paint test\t/tmp/paint test"])

    def test_invalid_artifacts_are_not_executables(self):
        changes = [{"executable": ""}, {"executable": ["/bin/true"]},
                   {"executable": "/bin/true\n/bin/false"}, {"target": None},
                   {"target": {"kind": [], "name": "easel"}},
                   {"target": {"kind": "bin", "name": "easel"}},
                   {"target": {"kind": ["bin"], "name": "easel test"}},
                   {"profile": {"test": "false"}}, {"profile": {}}]
        for change in changes:
            with self.subTest(change=change), self.assertRaises(ValueError):
                list(PARSER.executables([json.dumps(artifact() | change)]))

    def run_gate(self, stream, returncode=0):
        with tempfile.TemporaryDirectory(prefix="cargo-gate-") as temp:
            root = Path(temp)
            (root / "scripts").mkdir()
            (root / "bin").mkdir()
            for name in ("test", "cargo_test_executables.py"):
                shutil.copyfile(REPO / "scripts" / name, root / "scripts" / name)
            cargo = root / "bin/cargo"
            cargo.write_text(f"#!{sys.executable}\nimport sys\nprint({stream!r})\nsys.exit({returncode})\n")
            cargo.chmod(0o755)
            env = os.environ | {"PATH": f'{root / "bin"}:{os.environ["PATH"]}', "TMPDIR": str(root)}
            result = subprocess.run(["bash", str(root / "scripts/test")], env=env,
                                    capture_output=True, text=True, timeout=30)
            self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
            return result.stdout + result.stderr

    def test_missing_artifacts_fail_the_gate(self):
        output = self.run_gate('warning: compiler wrapper diagnostic\n{"reason":"build-finished","success":true}')
        self.assertIn("gate: the build made no easel executable", output)

    def test_missing_executable_fails_the_gate(self):
        self.assertIn("gate: the build made no easel executable",
                      self.run_gate(json.dumps(artifact(executable="/no/such/easel"))))

    def test_parser_failure_cannot_use_partial_artifacts(self):
        stream = json.dumps(artifact()) + "\n" + json.dumps(artifact() | {"profile": {}})
        self.assertIn("gate: the build artifact stream FAILED", self.run_gate(stream))

    def test_nonzero_build_exit_fails_even_with_artifacts(self):
        output = self.run_gate(json.dumps(artifact()), returncode=101)
        self.assertIn("gate: the build FAILED", output)
        self.assertNotIn("gate: built in", output)


if __name__ == "__main__":
    unittest.main()
