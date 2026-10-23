# /// script
# requires-python = ">=3.10"
# ///

import ast
import json
import os
from pathlib import Path
import runpy
import tempfile
import unittest
from unittest.mock import patch
import subprocess
import sys


REPO = Path(__file__).resolve().parents[2]
PREPARE = runpy.run_path(str(REPO / "scripts/prepare_agy_studio"))
PI_READING = next(
    ast.literal_eval(node.value)
    for node in ast.parse((REPO / "scripts/prepare_friedrich_studio").read_text()).body
    if isinstance(node, ast.Assign)
    and any(isinstance(target, ast.Name) and target.id == "CRITIC_READING" for target in node.targets)
)
CRITIC = json.dumps({"args": ["pi-black", "--model", "claude-opus-5-5", "--thinking", "xhigh"], "dir": "/synthetic/critic"})


class CriticPrepareTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="critic-prepare-")
        self.addCleanup(self.temp.cleanup)
        self.studio = Path(self.temp.name) / "paint-studio-abcd"
        (self.studio / "bin").mkdir(parents=True)
        (self.studio / "notes").mkdir()
        (self.studio / "bin/easel").touch()
        self.originals = {
            "BRIEF.md": b"Synthetic approved brief.\n",
            "notes/easel_guide.md": b"Synthetic easel guide.\n",
            "notes/watercolor_easel_guide.md": b"Synthetic watercolor guide.\n",
            "notes/techniques.md": b"Synthetic techniques.\n",
        }
        for name, text in self.originals.items():
            (self.studio / name).write_bytes(text)

    def prepare(self, env):
        with patch.dict(os.environ, env, clear=True), patch.object(PREPARE["shutil"], "which", return_value="/synthetic/bin/uv"):
            config = PREPARE["prepare"](self.studio, Path(self.temp.name) / "sessions")
        self.assertEqual(config, json.loads((self.studio / ".agents/mcp_config.json").read_text()))
        self.assertEqual((self.studio / ".agents/agents/easel-painter.md").read_bytes(), (REPO / "harness/agy/painter.md").read_bytes())
        self.assertEqual((self.studio / "LAUNCH.md").read_bytes(), (REPO / "harness/agy/LAUNCH.md").read_bytes())
        self.assertEqual((self.studio / "BRIEF.md").read_bytes(), self.originals["BRIEF.md"])
        self.assertEqual((self.studio / "notes/techniques.md").read_bytes(), self.originals["notes/techniques.md"])
        return config["mcpServers"]["easel"]

    def test_disabled_keeps_text_and_exports_runtime_for_later_enablement(self):
        config = self.prepare({})
        self.assertEqual(config["env"], {"PAINTER_CRITIC_CLI": str(REPO / "harness/painter/visit-cli.ts")})
        self.assertTrue(Path(config["env"]["PAINTER_CRITIC_CLI"]).is_absolute())
        for name, text in self.originals.items():
            self.assertEqual((self.studio / name).read_bytes(), text)
        self.assertEqual(
            {path.name for path in (self.studio / "bin/mcp").iterdir()},
            {"server.py", "remote.py", "references.py", "tools.json", "sittings.py", "sitting-tools.json", "sitting-message.txt", "painting_chunks.py", "critic.py", "critic-tools.json"},
        )
        for path in (self.studio / "bin/mcp").iterdir():
            self.assertEqual(path.read_bytes(), (REPO / "harness/easel_mcp" / path.name).read_bytes())

    def test_empty_switch_is_forwarded_without_text_changes(self):
        config = self.prepare({"PAINTER_CRITIC": ""})
        self.assertEqual(config["env"]["PAINTER_CRITIC"], "")
        for name, text in self.originals.items():
            self.assertEqual((self.studio / name).read_bytes(), text)

    def test_enabled_forwards_config_and_appends_only_exact_pi_paragraph(self):
        config = self.prepare({"PAINTER_CRITIC": CRITIC, "PAINTER_CRITIC_CLI": "/ignored/override.ts"})
        self.assertEqual(config["env"], {
            "PAINTER_CRITIC_CLI": str(REPO / "harness/painter/visit-cli.ts"),
            "PAINTER_CRITIC": CRITIC,
        })
        self.assertEqual(PREPARE["CRITIC_READING"], PI_READING)
        for name in ("notes/easel_guide.md", "notes/watercolor_easel_guide.md"):
            self.assertEqual((self.studio / name).read_bytes(), self.originals[name] + PI_READING.encode())

    def test_existing_pi_paragraph_is_not_duplicated(self):
        for name in ("notes/easel_guide.md", "notes/watercolor_easel_guide.md"):
            self.originals[name] += PI_READING.encode() + b"\n"
            (self.studio / name).write_bytes(self.originals[name])
        self.prepare({"PAINTER_CRITIC": CRITIC})
        for name, text in self.originals.items():
            self.assertEqual((self.studio / name).read_bytes(), text)

    def test_oil_studio_does_not_gain_a_watercolor_guide(self):
        (self.studio / "notes/watercolor_easel_guide.md").unlink()
        self.prepare({"PAINTER_CRITIC": CRITIC})
        self.assertEqual((self.studio / "notes/easel_guide.md").read_bytes(), self.originals["notes/easel_guide.md"] + PI_READING.encode())
        self.assertFalse((self.studio / "notes/watercolor_easel_guide.md").exists())

    def test_watercolor_does_not_gain_critic_guide_text(self):
        (self.studio / "bin/easel").unlink()
        (self.studio / "bin/watercolor-studio").touch()
        self.prepare({"PAINTER_CRITIC": CRITIC})
        for name, text in self.originals.items():
            self.assertEqual((self.studio / name).read_bytes(), text)

    def test_recording_collision_leaves_no_configuration_and_allows_retry(self):
        collision = self.studio / ".agents/recording.json"
        collision.mkdir(parents=True)
        command = [sys.executable, str(REPO / "scripts/prepare_agy_studio"), str(self.studio),
                   "--sessions-root", str(Path(self.temp.name) / "sessions")]
        failed = subprocess.run(command, capture_output=True, text=True)
        self.assertEqual(failed.returncode, 2, failed.stderr)
        self.assertFalse((self.studio / ".agents/mcp_config.json").exists())
        self.assertFalse((self.studio / ".agents/agents/easel-painter.md").exists())
        self.assertTrue(collision.is_dir())
        collision.rmdir()
        retried = subprocess.run(command, capture_output=True, text=True)
        self.assertEqual(retried.returncode, 0, retried.stderr)
        self.assertTrue(collision.is_file())

    def test_late_publication_failure_rolls_back_and_allows_retry(self):
        runtime = self.studio / "bin/mcp/server.py"
        runtime.parent.mkdir()
        runtime.write_bytes(b"existing runtime\n")
        runtime.chmod(0o640)
        self.originals["bin/mcp/server.py"] = runtime.read_bytes()
        (self.studio / "LAUNCH.md").write_bytes(b"existing launch notes\n")
        self.originals["LAUNCH.md"] = b"existing launch notes\n"
        replace = os.replace
        link = os.link
        collision = self.studio / ".agents/recording.json"
        def fail_recording(source, destination):
            if Path(destination) == collision:
                raise OSError("recording write failed")
            return replace(source, destination)
        def fail_recording_link(source, destination):
            if Path(destination) == collision:
                raise OSError("recording write failed")
            return link(source, destination)
        with patch.dict(os.environ, {"PAINTER_CRITIC": CRITIC}, clear=True), \
                patch.object(PREPARE["shutil"], "which", return_value="/synthetic/bin/uv"), \
                patch.object(os, "replace", side_effect=fail_recording), \
                patch.object(os, "link", side_effect=fail_recording_link), self.assertRaises(OSError):
            PREPARE["prepare"](self.studio, Path(self.temp.name) / "sessions")
        self.assertFalse((self.studio / ".agents/mcp_config.json").exists())
        self.assertFalse((self.studio / ".agents/agents/easel-painter.md").exists())
        for name, text in self.originals.items():
            self.assertEqual((self.studio / name).read_bytes(), text)
        self.assertEqual(runtime.stat().st_mode & 0o777, 0o640)
        del self.originals["LAUNCH.md"]
        del self.originals["bin/mcp/server.py"]
        self.prepare({"PAINTER_CRITIC": CRITIC})


if __name__ == "__main__":
    unittest.main()
