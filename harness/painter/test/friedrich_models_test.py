import ast
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


REPO = Path(__file__).resolve().parents[3]
HOUSE = REPO / "notes/round29/runner"
BLACK = Path.home() / ".pi/agent/git/github.com/aliceisjustplaying/pi-black/extensions/pi-black.ts"
OPUS = ["-e", str(BLACK), "--provider", "anthropic", "--model", "claude-opus-5-5", "--thinking", "xhigh"]
TOGETHER = "together/deepseek-ai/DeepSeek-V4.1-Flash-1017:high"
TOGETHER_ARGS = ["--provider", "together", "--model", "deepseek-ai/DeepSeek-V4.1-Flash-1017", "--thinking", "high"]


class FriedrichModelTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.studio, self.runner = (self.root / name for name in ("studio", "runner"))
        for name in ("bin", "notes/research", "paintings/lua"):
            (self.studio / name).mkdir(parents=True)
        (self.studio / "bin/box").write_text("friedrich\n")
        (self.studio / "notes/journal.md").write_text("")
        (self.studio / "notes/reference-library.json").write_text("{}")
        self.prepare = [sys.executable, str(REPO / "scripts/prepare_friedrich_studio"), str(self.studio),
                        "--house", str(HOUSE), "--runner-dir", str(self.runner),
                        "--studio-notes", str(HOUSE / "studio_notes.md")]

    def check(self, *args):
        subprocess.run(self.prepare + list(args), check=True, capture_output=True, text=True)
        config = json.loads((self.runner / "run-config.json").read_text())
        result = subprocess.run([sys.executable, str(self.runner / "run.py"), "--check"],
                                check=True, capture_output=True, text=True)
        return config, json.loads(result.stdout)

    def test_together_painter_and_anthropic_critic(self):
        config, check = self.check("--critic", "--painter", TOGETHER,
                                   "--critic-model", "anthropic/claude-opus-5-5:xhigh")
        self.assertEqual(config["painter"], TOGETHER)
        self.assertEqual(config["critic_model"], "anthropic/claude-opus-5-5:xhigh")
        self.assertFalse(check["model"]["black"])
        self.assertEqual(check["model"]["args"], TOGETHER_ARGS)
        self.assertEqual(check["first_sitting_cmd"][-7:-1], TOGETHER_ARGS)
        self.assertNotIn(str(BLACK), check["first_sitting_cmd"])
        for key in ("first_sitting_env", "later_sitting_env"):
            self.assertEqual(json.loads(check[key]["PAINTER_CRITIC"]),
                             {"args": OPUS, "dir": str(self.runner / "run/critic")})

    def test_defaults_keep_the_existing_command(self):
        config, check = self.check("--critic")
        self.assertNotIn("painter", config)
        self.assertNotIn("critic_model", config)
        message = next(ast.literal_eval(node.value) for node in ast.parse((HOUSE / "r21_chains.py").read_text()).body
                       if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "PAINTER_MSG" for t in node.targets))
        harness = REPO / "harness/painter"
        self.assertEqual(check["first_sitting_cmd"], ["pi", "--print", "--no-extensions",
            "-e", str(harness / "painter.ts"), "-e", str(harness / "compaction.ts"),
            "--system-prompt", str(harness / "system_prompt.md"), "--tools",
            "paint,look,probe,note,status,log,read,reference,reference_list,ask_critic",
            "--no-context-files", "--no-skills", "--no-prompt-templates", "--no-approve", *OPUS, message])
        self.assertTrue(check["model"]["black"])
        self.assertEqual(json.loads(check["first_sitting_env"]["PAINTER_CRITIC"])["args"], OPUS)

    def test_critic_model_requires_critic(self):
        result = subprocess.run(self.prepare + ["--critic-model", "anthropic/claude-opus-5-5:xhigh"],
                                capture_output=True, text=True)
        self.assertEqual(result.returncode, 2)
        self.assertIn("--critic-model requires --critic", result.stderr)
        self.assertFalse(self.runner.exists())
        self.assertFalse((self.studio / "BRIEF.md").exists())


if __name__ == "__main__":
    unittest.main()
