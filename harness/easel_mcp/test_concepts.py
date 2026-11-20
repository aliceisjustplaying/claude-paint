# /// script
# requires-python = ">=3.11"
# dependencies = ["mcp==1.30.0", "Pillow==12.3.0"]
# ///
import ast
import json
import os
from pathlib import Path
import runpy
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

from concepts import ConceptsEasel, TOOL


REPO = Path(__file__).resolve().parents[2]
COMMAND = REPO / "scripts/agy_die_roll"
ROLL = runpy.run_path(str(COMMAND))
DIE = ROLL["DIE"]
PREPARE = runpy.run_path(str(REPO / "scripts/prepare_friedrich_studio"))
HOUSE = REPO / "notes/round29/runner"
GOOD = "\n\n".join(f"{i}. Picture {i}.\nEstimate: {i}/100" for i in range(1, 17))
LAUNCH = next(ast.literal_eval(node.value) for node in ast.parse((HOUSE / "r21_chains.py").read_text()).body
              if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "PAINTER_MSG" for target in node.targets))


class ConceptsTests(unittest.IsolatedAsyncioTestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.studio = self.root / "paint-studio-abcd"
        (self.studio / "notes").mkdir(parents=True)
        (self.studio / "BRIEF.md").write_text("Approved brief.\n")

    async def test_protocol_has_only_read_and_write_concepts_even_with_critic(self):
        (self.studio / "bin").mkdir()
        (self.studio / "bin/easel").touch()
        params = StdioServerParameters(command=sys.executable, args=[str(REPO / "harness/easel_mcp/server.py"),
            "--studio", str(self.studio), "--concepts-only", "--agy", "--sessions-root", str(self.root / "sessions")],
            env={**os.environ, "PAINTER_CRITIC": "configured", "PAINTER_CRITIC_CLI": "/never-run"})
        async with stdio_client(params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                tools = {tool.name: tool for tool in (await session.list_tools()).tools}
                self.assertEqual(set(tools), {"read", "write_concepts"})
                self.assertTrue(all(tool.meta == {"antigravity/alwaysLoad": True} for tool in tools.values()))
                self.assertEqual(tools["write_concepts"].description, TOOL["description"])
                self.assertIn('description: ' + json.dumps(TOOL["description"]), (REPO / "harness/painter/concepts.ts").read_text())
                brief = await session.call_tool("read", {"path": "BRIEF.md"})
                self.assertEqual(brief.content[0].text, "Approved brief.\n")
                for text in (GOOD, "New pictures.\n"):
                    saved = await session.call_tool("write_concepts", {"text": text})
                    self.assertFalse(saved.isError)
                    self.assertEqual(saved.content[0].text, "Saved notes/concepts.md.")
                    self.assertEqual((self.studio / "notes/concepts.md").read_text(), text)
                for tool in ("paint", "look", "note", "status", "log", "reference", "reference_list", "end_sitting", "ask_critic"):
                    self.assertTrue((await session.call_tool(tool, {})).isError)
        self.assertFalse((self.studio / "out/easel/sittings.json").exists())
        self.assertFalse((self.studio / "paintings").exists())

    async def test_save_is_atomic_and_cleans_up_on_failure(self):
        path = self.studio / "notes/concepts.md"
        path.write_text("original")
        easel = ConceptsEasel(self.studio)
        replace = os.replace

        def publish(source, destination):
            self.assertEqual(path.read_text(), "original")
            self.assertEqual(Path(source).read_text(), GOOD)
            self.assertEqual(Path(source).parent, path.parent)
            return replace(source, destination)

        with patch("concepts.os.replace", side_effect=publish):
            await easel.call("write_concepts", {"text": GOOD})
        self.assertEqual(path.read_text(), GOOD)
        with patch("concepts.os.replace", side_effect=OSError("cannot publish")), self.assertRaises(OSError):
            await easel.call("write_concepts", {"text": "replacement"})
        self.assertEqual(path.read_text(), GOOD)
        self.assertEqual(list(path.parent.iterdir()), [path])

    async def test_exported_server_keeps_critic_for_painting_only(self):
        (self.studio / "bin").mkdir()
        (self.studio / "bin/easel").touch()
        prepare = runpy.run_path(str(REPO / "scripts/prepare_agy_studio"))["prepare"]
        config = prepare(self.studio, self.root / "sessions", concepts_only=True,
                         critic_config=json.dumps({"args": ["--model", "unused"], "dir": str(self.root / "critic")}))
        easel = config["mcpServers"]["easel"]
        for mode in ("--concepts-only", "--sittings"):
            params = StdioServerParameters(command=easel["command"], args=easel["args"], cwd=easel["cwd"], env=easel["env"])
            async with stdio_client(params) as (read, write):
                async with ClientSession(read, write) as session:
                    await session.initialize()
                    tools = {tool.name for tool in (await session.list_tools()).tools}
                    if mode == "--concepts-only":
                        self.assertEqual(tools, {"read", "write_concepts"})
                        self.assertFalse((self.studio / "out/easel/sittings.json").exists())
                    else:
                        self.assertIn("ask_critic", tools)
                        self.assertIn("end_sitting", tools)
                        self.assertIn("paint", tools)
                        self.assertNotIn("write_concepts", tools)
            if mode == "--concepts-only":
                easel["args"][easel["args"].index("--concepts-only")] = "--sittings"


class RollTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.studio = self.root / "paint-studio-abcd"
        self.runner = self.root / "runner"
        self.run = self.runner / "run"
        for name in ("bin", "notes/research", "paintings/lua"):
            (self.studio / name).mkdir(parents=True)
        (self.studio / "bin/easel").touch()
        (self.studio / "bin/box").write_text("friedrich\n")
        (self.studio / "notes/journal.md").write_text("")
        (self.studio / "notes/reference-library.json").write_text("{}")
        (self.studio / "notes/easel_guide.md").write_text("| tool | what it does |\n|---|---|\n| `read` | Read. |\n\n")
        self.prepare = [sys.executable, str(REPO / "scripts/prepare_friedrich_studio"), str(self.studio),
                        "--house", str(HOUSE), "--runner-dir", str(self.runner),
                        "--studio-notes", str(HOUSE / "studio_notes.md"), "--die-roll"]
        self.prepared = subprocess.run(self.prepare + ["--agy", "--sessions-root", str(self.root / "sessions")], capture_output=True, text=True)
        self.assertEqual(self.prepared.returncode, 0, self.prepared.stderr)

    def command(self):
        return subprocess.run([sys.executable, str(COMMAND), "roll", str(self.studio), "--run-dir", str(self.run)], capture_output=True, text=True)

    def test_preparation_reuses_exact_friedrich_text_and_exports_concepts_server(self):
        brief = (self.studio / "BRIEF.md").read_bytes()
        notes = (self.studio / "notes/studio_notes.md").read_bytes()
        config = json.loads((self.studio / ".agents/mcp_config.json").read_text())
        self.assertIn("--concepts-only", config["mcpServers"]["easel"]["args"])
        self.assertNotIn("--sittings", config["mcpServers"]["easel"]["args"])
        self.assertEqual(config["mcpServers"]["easel"]["env"]["PAINTER_CRITIC"], "")
        self.assertEqual((self.studio / "bin/mcp/concepts.py").read_bytes(), (REPO / "harness/easel_mcp/concepts.py").read_bytes())
        self.assertEqual((self.studio / "LAUNCH.md").read_bytes(), (REPO / "harness/agy/LAUNCH.md").read_bytes())
        (self.studio / "BRIEF.md").unlink()
        PREPARE["shutil"].rmtree(self.runner)
        PREPARE["prepare"](self.studio, HOUSE, self.runner, HOUSE / "studio_notes.md", die_roll=True)
        self.assertEqual((self.studio / "BRIEF.md").read_bytes(), brief)
        self.assertEqual((self.studio / "notes/studio_notes.md").read_bytes(), notes)

    def test_message_is_unchanged(self):
        result = subprocess.run([sys.executable, str(COMMAND), "message"], capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout, DIE["SITTING_ZERO"] + "\n")

    def test_critic_config_uses_selected_model_run_folder_and_pi_black(self):
        original_guide = (self.studio / "notes/easel_guide.md").read_bytes()
        (self.studio / "BRIEF.md").unlink()
        PREPARE["shutil"].rmtree(self.runner)
        PREPARE["shutil"].rmtree(self.studio / ".agents")
        black = self.root / "pi-black/extensions/pi-black.ts"
        run = self.root / "selected-run"
        result = subprocess.run(self.prepare + ["--agy", "--sessions-root", str(self.root / "sessions"),
            "--critic", "--critic-model", "anthropic/claude-fable-5-2:xhigh", "--pi-black", str(black),
            "--run-dir", str(run)], capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        config_path = self.studio / ".agents/mcp_config.json"
        env = json.loads(config_path.read_text())["mcpServers"]["easel"]["env"]
        self.assertEqual(json.loads(env["PAINTER_CRITIC"]), {
            "args": ["-e", str(black), "--provider", "anthropic", "--model", "claude-fable-5-2", "--thinking", "xhigh"],
            "dir": str(run / "critic"),
        })
        self.assertEqual(env["PAINTER_CRITIC_CLI"], str(REPO / "harness/painter/visit-cli.ts"))
        brief = (self.studio / "BRIEF.md").read_bytes()
        guide = (self.studio / "notes/easel_guide.md").read_bytes()
        self.assertIn(PREPARE["CRITIC_READING"].encode(), brief)
        description = json.loads((REPO / "harness/easel_mcp/critic-tools.json").read_text())[0]["description"]
        self.assertIn(f"| `ask_critic` | {description} |\n".encode(), guide)
        self.assertNotIn(PREPARE["CRITIC_READING"].encode(), guide)
        (self.studio / "notes/concepts.md").write_text(GOOD)
        ROLL["roll_studio"](self.studio, run)
        self.assertEqual(json.loads(config_path.read_text())["mcpServers"]["easel"]["env"], env)
        (self.studio / "BRIEF.md").unlink()
        PREPARE["shutil"].rmtree(self.runner)
        (self.studio / "notes/easel_guide.md").write_bytes(original_guide)
        PREPARE["prepare"](self.studio, HOUSE, self.runner, HOUSE / "studio_notes.md", run_dir=run,
                           critic=True, die_roll=True, critic_model="anthropic/claude-fable-5-2:xhigh")
        self.assertEqual((self.studio / "BRIEF.md").read_bytes(), brief)
        self.assertEqual((self.studio / "notes/easel_guide.md").read_bytes(), guide)

    def test_roll_records_once_and_prints_exact_painting_message(self):
        (self.studio / "notes/concepts.md").write_text(GOOD)
        result = self.command()
        self.assertEqual(result.returncode, 0, result.stderr)
        record = self.run / "die_roll.json"
        original = record.read_bytes()
        chosen = json.loads(original)
        self.assertEqual(chosen["concepts"], DIE["parse_concepts"](GOOD))
        self.assertEqual(chosen["dropped"], list(range(16, 6, -1)))
        self.assertIn(chosen["pick"], chosen["concepts"][:6])
        self.assertEqual(result.stdout, DIE["opening"](chosen) + "\n\n" + LAUNCH + "\n")
        config = json.loads((self.studio / ".agents/mcp_config.json").read_text())
        self.assertIn("--sittings", config["mcpServers"]["easel"]["args"])
        self.assertNotIn("--concepts-only", config["mcpServers"]["easel"]["args"])
        again = self.command()
        self.assertNotEqual(again.returncode, 0)
        self.assertIn("already exists; no die was rolled again", again.stderr)
        self.assertEqual(again.stdout, "")
        self.assertEqual(record.read_bytes(), original)

    def test_parse_failure_prints_one_retry_and_never_rolls(self):
        config = (self.studio / ".agents/mcp_config.json").read_bytes()
        for attempt in range(3):
            result = self.command()
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(result.stdout, DIE["RETRY"] + "\n" if attempt == 0 else "")
            self.assertFalse((self.run / "die_roll.json").exists())
            self.assertEqual((self.studio / ".agents/mcp_config.json").read_bytes(), config)
            if attempt == 0:
                (self.studio / "notes/concepts.md").write_text("bad concepts")

    def test_retry_can_succeed(self):
        self.assertNotEqual(self.command().returncode, 0)
        (self.studio / "notes/concepts.md").write_text(GOOD)
        self.assertEqual(self.command().returncode, 0)
        self.assertTrue((self.run / "die_roll.json").exists())

    def test_any_chunk_in_either_log_refuses_before_rolling(self):
        (self.studio / "notes/concepts.md").write_text(GOOD)
        for name in ("paintings/lua/painting.lua", "out/easel/painting/committed.lua"):
            log = self.studio / name
            log.parent.mkdir(parents=True, exist_ok=True)
            log.write_text("--@ chunk 1\nprint(wait(0))\n")
            result = self.command()
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("requires a pristine painting", result.stderr)
            self.assertFalse((self.run / "die_roll.json").exists())
            log.unlink()

    def test_existing_record_is_never_replaced_even_if_invalid(self):
        self.run.mkdir()
        record = self.run / "die_roll.json"
        record.write_text("not json")
        with patch.object(DIE["secrets"], "choice", side_effect=AssertionError("rerolled")):
            with self.assertRaisesRegex(ValueError, "already exists"):
                ROLL["roll_studio"](self.studio, self.run)
        self.assertEqual(record.read_text(), "not json")

    def test_agy_preflight_failure_leaves_the_empty_studio_untouched(self):
        studio = self.root / "paint-studio-1234"
        for name in ("bin", "notes/research", "paintings/lua"):
            (studio / name).mkdir(parents=True)
        (studio / "bin/box").write_text("friedrich\n")
        (studio / "notes/journal.md").touch()
        (studio / "notes/reference-library.json").write_text("{}")
        command = self.prepare.copy()
        command[2] = str(studio)
        command[command.index("--runner-dir") + 1] = str(self.root / "new-runner")
        result = subprocess.run(command + ["--agy", "--sessions-root", str(self.root / "sessions")], capture_output=True, text=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("missing bin/easel", result.stderr)
        self.assertFalse((studio / "BRIEF.md").exists())
        self.assertFalse((self.root / "new-runner").exists())


if __name__ == "__main__":
    unittest.main()
