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


if __name__ == "__main__":
    unittest.main()
