import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


REPO = Path(__file__).resolve().parents[2]
TABLE = ("| tube | pigment | hiding | stiffness | tinting strength | drying |\n"
         "| --- | --- | --- | --- | --- | --- |\n"
         "| fixture-pigment-00 | pigment | opaque | medium | medium | medium |\n")
GIT = '''import os
from pathlib import Path
import subprocess
import sys

args = sys.argv[3:]
root = Path(os.environ["EXPORT_FIXTURES"])
if args[0] == "rev-parse":
    print("fixture-tree")
elif args[0] == "cat-file":
    ref, path = args[-1].split(":", 1)
    sys.exit(0 if (root / ref / path).is_file() else 1)
elif args[0] == "archive":
    ref = args[2]
    sys.exit(subprocess.run(["tar", "-cf", "-", "-C", str(root / ref), *args[4:]]).returncode)
elif args[0] == "show":
    ref, path = args[1].split(":", 1)
    sys.stdout.buffer.write((root / ref / path).read_bytes())
else:
    sys.exit("unexpected fixture git command")
'''
EASEL = '''from pathlib import Path
import sys

studio = Path(__file__).resolve().parents[1]
command = sys.argv[1]
if command == "tubes":
    print(TABLE, end="")
elif command == "save":
    mode, box = (studio / "bin/studio").read_text().split()
    assert mode == "painter"
    header = "" if box == "none" else f"--@ box {box}\\n"
    (studio / "paintings/lua/painting.lua").write_text(header + "--@ chunk\\n")
    image = studio / "out/easel/painting/painting.png"
    image.parent.mkdir(parents=True)
    image.write_bytes(b"fixture image")
elif command not in ("open", "do", "look", "close", "help"):
    sys.exit("unexpected fixture easel command")
'''


class ExportMaterialsTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="export-materials-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.repo = self.root / "repo"
        for name in ("export_r16_studio", "prepare_capy_studio", "strip_sources",
                     "check_studio_names", "check_box_tubes", "tests/export_profiles.sh",
                     "tests/export_concurrent.sh"):
            path = self.repo / "scripts" / name
            path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(REPO / "scripts" / name, path)
        for name in ("server.py", "references.py", "tools.json"):
            self.write(self.repo / "harness/easel_mcp" / name, "{}\n")
        fixtures = self.root / "revisions"
        for ref in ("HEAD", "historical"):
            revision = fixtures / ref
            self.write(revision / "notes/easel_guide.md", TABLE)
            self.write(revision / "notes/techniques.md", "Fixture techniques.\n")
            self.write(revision / "notes/research/oil_paint_physics.md", "Fixture physics.\n")
            self.write(revision / "THIRD_PARTY_NOTICES.md", "Fixture notices.\n")
            self.write(revision / "crates/paint/src/palette.rs",
                       "".join(f'    tube("fixture-pigment-{i:02d}")\n' for i in range(14)))
        self.write(fixtures / "historical/notes/research/tonn_materials.md",
                   "Historical fixture materials [TEST].\n\n## Sources\nFixture source.\n")
        self.executable(self.root / "bin/git", GIT)
        self.executable(self.root / "bin/cargo", 'raise SystemExit("builds forbidden in this test")\n')
        code = hashlib.sha1(b"fixture-tree\n" * 4).hexdigest()[:12]
        builds = self.root / "builds"
        self.executable(builds / f"code-{code}/built/universal/easel",
                        f"TABLE = {TABLE!r}\n" + EASEL)
        self.env = dict(os.environ, PATH=f"{self.root / 'bin'}:{os.environ['PATH']}",
                        EXPORT_FIXTURES=str(fixtures), STUDIO_BUILDS=str(builds),
                        TMPDIR=str(self.root), UV_PYTHON=sys.executable)
        for key in ("STUDIO_WIDTH", "STUDIO_BUILD_HOOK"):
            self.env.pop(key, None)

    def write(self, path, text):
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text)

    def executable(self, path, code):
        self.write(path, f"#!{sys.executable}\n" + code)
        path.chmod(0o755)

    def export(self, profile="tonn", ref="HEAD"):
        dest = self.root / "studio"
        result = subprocess.run(
            [str(self.repo / "scripts/export_r16_studio"), profile, str(dest)],
            env=dict(self.env, R16_BRANCH=ref), capture_output=True, text=True,
        )
        return dest, result

    def test_current_tonn_export_without_materials(self):
        dest, result = self.export()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertFalse((dest / "notes/research/tonn_materials.md").exists())
        self.assertEqual((dest / "bin/box").read_text(), "tonn\n")
        self.assertEqual((dest / "bin/studio").read_text(), "painter tonn\n")
        self.assertTrue((dest / "notes/research/oil_paint_physics.md").is_file())

    def test_success_removes_only_its_probe(self):
        unrelated = self.root / "studio-probe.unrelated"
        self.write(unrelated / "keep", "unrelated data\n")
        dest, result = self.export()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(list(self.root.glob("studio-probe.*")), [unrelated])
        self.assertEqual((unrelated / "keep").read_text(), "unrelated data\n")
        self.assertEqual(list((dest / "paintings/lua").iterdir()), [])
        self.assertFalse((dest / "out").exists())

    def test_external_builds_without_tmpdir_or_repo_target(self):
        self.env.pop("TMPDIR", None)
        self.assertFalse((self.repo / "target").exists())
        _, result = self.export()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(list((self.repo / "target").glob("studio-probe.*")), [])

    def test_missing_tmpdir_parent_is_created(self):
        parent = self.root / "new/tmp"
        self.env["TMPDIR"] = str(parent)
        _, result = self.export()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertTrue(parent.is_dir())
        self.assertEqual(list(parent.iterdir()), [])

    def test_failed_probe_is_removed(self):
        built = next((self.root / "builds").glob("code-*/built/universal/easel"))
        source = built.read_text()
        failures = {
            "session": source.replace('elif command == "save":',
                                      'elif command == "look":\n'
                                      '    sys.exit("fixture probe failure")\n'
                                      'elif command == "save":'),
            "validation": source.replace('header + "--@ chunk\\n"',
                                         '"--@ box wrong\\n--@ chunk\\n"'),
        }
        for stage, code in failures.items():
            with self.subTest(stage=stage):
                built.write_text(code)
                dest, result = self.export()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("fixture probe failure" if stage == "session" else
                              "the probe's log names box 'wrong'", result.stderr)
                self.assertEqual(list(self.root.glob("studio-probe.*")), [])
                self.assertTrue((dest / "bin/easel").is_file())
                shutil.rmtree(dest)

    def test_license_retry_is_only_used_on_macos(self):
        code = hashlib.sha1(b"fixture-tree\n" * 4).hexdigest()[:12]
        src = Path(self.env["STUDIO_BUILDS"]) / f"code-{code}"
        self.write(src / ".extracted", "")
        cached = src / "built/universal/easel"
        source = cached.read_text()
        for system in ("Linux", "Darwin"):
            with self.subTest(system=system):
                cached.unlink(missing_ok=True)
                calls = self.root / f"{system}.jsonl"
                self.executable(self.root / "bin/uname", f'print({system!r})\n')
                self.executable(self.root / "bin/cargo",
                                "import json, os, sys\nfrom pathlib import Path\n"
                                f"calls = Path({str(calls)!r})\n"
                                "retry = calls.exists()\n"
                                "with calls.open('a') as file:\n"
                                "    file.write(json.dumps(os.environ.get('DEVELOPER_DIR')) + '\\n')\n"
                                "if not retry:\n"
                                "    sys.exit('fixture license error')\n"
                                "target = Path(os.environ['CARGO_TARGET_DIR']) / 'release/easel'\n"
                                "target.parent.mkdir(parents=True, exist_ok=True)\n"
                                f"target.write_text({source!r})\n"
                                "target.chmod(0o755)\n")
                self.env.pop("DEVELOPER_DIR", None)
                _, result = self.export()
                observed = [json.loads(line) for line in calls.read_text().splitlines()]
                if system == "Darwin":
                    self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                    self.assertEqual(observed, [None, "/Library/Developer/CommandLineTools"])
                else:
                    self.assertNotEqual(result.returncode, 0)
                    self.assertIn("fixture license error", result.stderr)
                    self.assertEqual(observed, [None])

    def test_historical_tonn_export_includes_stripped_materials(self):
        dest, result = self.export(ref="historical")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual((dest / "notes/research/tonn_materials.md").read_text(),
                         "Historical fixture materials.\n")

    def test_default_ref_exports_head(self):
        env = self.env.copy()
        env.pop("R16_BRANCH", None)
        result = subprocess.run(
            [str(self.repo / "scripts/export_r16_studio"), "tonn", str(self.root / "studio")],
            env=env, capture_output=True, text=True,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("from HEAD", result.stdout)

    def test_concurrent_exports_build_once_with_default_features(self):
        built = next((self.root / "builds").glob("code-*/built/universal/easel"))
        source = self.root / "source-easel"
        shutil.move(built, source)
        source.write_text(source.read_text().replace(
            'print(TABLE, end="")',
            'box = (studio / "bin/studio").read_text().split()[1]\n'
            '    pigment = {"sargent": "01", "inness": "02"}.get(box, "00")\n'
            '    print(TABLE.replace("pigment-00", "pigment-" + pigment), end="")',
        ))
        revision = self.root / "revisions/HEAD"
        (revision / ".cargo").mkdir()
        for name in ("Cargo.toml", "Cargo.lock"):
            self.write(revision / name, "fixture\n")
        for profile in ("sargent", "inness"):
            self.write(revision / f"notes/research/{profile}_materials.md", "Fixture materials.\n")
        self.executable(self.root / "bin/cargo", '''import os
from pathlib import Path
import shutil
import sys

assert sys.argv[1:] == ["build", "--release", "-p", "easel"], sys.argv
root = Path(os.environ["EXPORT_FIXTURES"]).parent
with (root / "cargo-calls").open("a") as calls:
    calls.write("build\\n")
target = Path(os.environ["CARGO_TARGET_DIR"]) / "release/easel"
target.parent.mkdir(parents=True, exist_ok=True)
shutil.copy2(root / "source-easel", target)
''')
        result = subprocess.run(
            [str(self.repo / "scripts/tests/export_concurrent.sh")],
            env=dict(self.env, R16_BRANCH="HEAD"), capture_output=True, text=True,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual((self.root / "cargo-calls").read_text(), "build\n")

    def test_profile_checks_accept_current_and_historical_tonn(self):
        for ref in ("HEAD", "historical"):
            with self.subTest(ref=ref):
                result = subprocess.run(
                    [str(self.repo / "scripts/tests/export_profiles.sh"), "tonn"],
                    env=dict(self.env, R16_BRANCH=ref), capture_output=True, text=True,
                )
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_other_profiles_still_require_materials(self):
        _, result = self.export(profile="hopper")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("notes/research/hopper_materials.md", result.stderr)

    def test_tonn_still_requires_physics(self):
        (self.root / "revisions/HEAD/notes/research/oil_paint_physics.md").unlink()
        _, result = self.export()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("notes/research/oil_paint_physics.md", result.stderr)

    def test_prepare_current_tonn_preserves_approved_briefs(self):
        notes = self.root / "studio_notes.md"
        notes.write_text("Approved fixture notes.\n")
        dest = self.root / "prepared"
        brief = self.root / "approved-brief.md"
        original = ("- The folder /approved/studio: this brief, your notes and the easel.\n"
                    "notes/easel_guide.md; notes/studio_notes.md;\n"
                    "notes/research/oil_paint_physics.md as needed.\n")
        brief.write_text(original)
        result = subprocess.run(
            [sys.executable, str(self.repo / "scripts/prepare_capy_studio"),
             "tonn", str(dest), "--brief", str(brief), "--studio-notes", str(notes)],
            env=dict(self.env, R16_BRANCH="historical"), capture_output=True, text=True,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("from HEAD", result.stderr)
        self.assertFalse((dest / "notes/research/tonn_materials.md").exists())
        self.assertEqual((dest / "BRIEF.md").read_text(),
                         original.replace("/approved/studio", str(dest)))
        self.assertEqual(brief.read_text(), original)
        self.assertEqual((dest / "notes/studio_notes.md").read_bytes(), notes.read_bytes())
        self.assertEqual(json.loads(result.stdout)["cwd"], str(dest))


if __name__ == "__main__":
    unittest.main()
