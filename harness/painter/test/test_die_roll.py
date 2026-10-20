import importlib.util
import hashlib
import json
from pathlib import Path
import runpy
import tempfile
import types
import unittest
from unittest.mock import patch


REPO = Path(__file__).resolve().parents[3]
SPEC = importlib.util.spec_from_file_location("die_roll", REPO / "harness/painter/die_roll.py")
D = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(D)
PREPARE = runpy.run_path(str(REPO / "scripts/prepare_friedrich_studio"))
GOOD = "\n\n".join(f"{i}. Picture {i}.\nEstimate: {i}/100" for i in range(1, 17))
HOUSE = '''TOOLS = "old"
READING = {}
READING['friedrich'] = "old"
OPENING = {'friedrich': "old"}
if t['profile'] == 'friedrich':
    copy("friedrich_painter.md", "trees.md")
def paint(k, before, cont):
    msg = PAINTER_MSG if before == 0 else "later"
    stem = f"p{n}_s{k}"
    return msg
if __name__ == '__main__':
    launch()
'''


class ParserTests(unittest.TestCase):
    def test_good_preserves_picture_text(self):
        text = GOOD.replace("Picture 1.", "A cowboy.\nA zombie.")
        concepts = D.parse_concepts(text)
        self.assertEqual(len(concepts), 16)
        self.assertEqual(concepts[0], {"number": 1, "text": "A cowboy.\nA zombie.", "estimate": 1})
        self.assertEqual(D.opening({"pick": concepts[0]}), "The die picked concept 1: A cowboy.\nA zombie.")

    def test_missing_estimate(self):
        with self.assertRaises(ValueError):
            D.parse_concepts(GOOD.replace("Estimate: 4/100", ""))

    def test_fifteen_entries(self):
        with self.assertRaises(ValueError):
            D.parse_concepts(GOOD[:GOOD.index("16.")])

    def test_malformed_estimates_and_numbering(self):
        for text in (GOOD.replace("4/100", "101/100"), GOOD.replace("4/100", "4.5/100"),
                     GOOD.replace("4.", "3."), GOOD + "\n17. Extra.\nEstimate: 1/100",
                     GOOD.replace("Picture 4.", "Picture 4.\nEstimate: 3/100")):
            with self.subTest(text=text), self.assertRaises(ValueError):
                D.parse_concepts(text)

    def test_drop_ten_highest_and_roll_over_six(self):
        concepts = D.parse_concepts(GOOD)
        with patch.object(D.secrets, "choice", side_effect=lambda options: options[0]) as choice:
            result = D.roll(concepts)
        self.assertEqual(result["dropped"], [16, 15, 14, 13, 12, 11, 10, 9, 8, 7])
        self.assertEqual(choice.call_args.args[0], concepts[:6])
        self.assertEqual(result["pick"], concepts[0])

    def test_ties_drop_lower_numbers_first(self):
        concepts = D.parse_concepts(GOOD)
        for concept in concepts:
            concept["estimate"] = 10
        with patch.object(D.secrets, "choice", side_effect=lambda options: options[-1]) as choice:
            result = D.roll(concepts)
        self.assertEqual(result["dropped"], list(range(1, 11)))
        self.assertEqual(choice.call_args.args[0], concepts[10:])


class SittingTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.studio = self.root / "studio"
        self.run = self.root / "run"
        (self.studio / "notes").mkdir(parents=True)
        (self.studio / "paintings/lua").mkdir(parents=True)
        self.run.mkdir()
        self.log = self.studio / "paintings/lua/painting.lua"
        self.log.write_text("pristine\n")
        self.messages = []
        self.model = {"env": {"PAINTER_CRITIC": "configured"}}
        self.runner = types.SimpleNamespace(H=self.root, count_chunks=lambda studio: 0,
                                            painter_cmd=lambda model, message: ["pi", "-e", str(self.root / "painter.ts"), "--tools", "paint,note", message], run=self.invoke)
        self.replies = [GOOD]

    def invoke(self, cmd, studio, out, err, env):
        self.assertEqual(cmd[cmd.index("--tools") + 1], "read,write_concepts")
        self.assertIn(str(self.root / "concepts.ts"), cmd)
        self.assertEqual(env["PAINTER_CRITIC"], "")
        self.messages.append(cmd[-1])
        (studio / "notes/concepts.md").write_text(self.replies.pop(0))
        return 0

    def test_records_roll_and_resumes_without_reroll(self):
        result = D.choose(self.runner, self.model, self.studio, self.run)
        record = json.loads((self.run / "die_roll.json").read_text())
        self.assertEqual(record["concepts"], D.parse_concepts(GOOD))
        self.assertEqual(record["dropped"], [16, 15, 14, 13, 12, 11, 10, 9, 8, 7])
        self.assertEqual(result, D.opening(record))
        self.assertEqual(D.choose(self.runner, self.model, self.studio, self.run), result)
        self.assertEqual(self.messages, [D.SITTING_ZERO])

    def test_retries_once(self):
        self.replies = ["bad", GOOD]
        D.choose(self.runner, self.model, self.studio, self.run)
        self.assertEqual(self.messages, [D.SITTING_ZERO, D.RETRY])

    def test_second_parse_failure_stops_and_cannot_retry_again(self):
        self.replies = ["bad", "still bad"]
        for _ in range(2):
            with self.assertRaisesRegex(RuntimeError, "two sitting-0 attempts"):
                D.choose(self.runner, self.model, self.studio, self.run)
        self.assertEqual(len(self.messages), 2)
        self.assertFalse((self.run / "die_roll.json").exists())

    def test_marks_fail_loudly_and_stay_in_log(self):
        original = self.runner.run
        def marked(*args, **kwargs):
            self.log.write_text("--@ chunk 1\nmark\n")
            return original(*args, **kwargs)
        self.runner.run = marked
        with self.assertRaisesRegex(RuntimeError, "SITTING 0 CHANGED THE PAINTING"):
            D.choose(self.runner, self.model, self.studio, self.run)
        self.assertEqual(self.log.read_text(), "--@ chunk 1\nmark\n")
        self.assertFalse((self.run / "die_roll.json").exists())
        with self.assertRaisesRegex(RuntimeError, "interrupted or failed"):
            D.choose(self.runner, self.model, self.studio, self.run)

    def test_provider_failure_does_not_roll(self):
        self.runner.run = lambda *args, **kwargs: 7
        with self.assertRaisesRegex(RuntimeError, "exited 7"):
            D.choose(self.runner, self.model, self.studio, self.run)
        self.assertFalse((self.run / "die_roll.json").exists())


class PreparationTests(unittest.TestCase):
    def test_opt_out_is_byte_identical_and_opt_in_adds_only_section_and_runtime(self):
        baseline = json.loads((Path(__file__).parent / "fixtures/prepare-baseline.json").read_text())
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            studio, house, runner = (root / name for name in ("studio", "house", "runner"))
            (studio / "bin").mkdir(parents=True)
            (studio / "notes/research").mkdir(parents=True)
            (studio / "paintings/lua").mkdir(parents=True)
            (studio / "bin/box").write_text("friedrich\n")
            (studio / "notes/journal.md").write_text("")
            (studio / "notes/reference-library.json").write_text("{}")
            house.mkdir()
            (house / "r21_chains.py").write_text(HOUSE)
            template = "{OPENING}\n{STUDIO}\n{READING}\n\n## Working\nWork.\n"
            (house / "brief_template.md").write_text(template)
            for name in ("painting_chunks.py", "defaults_used.py", "log_copy.py", "copied_shapes.py", "studio_notes.md"):
                (house / name).write_text("plain\n")
            for critic in (False, True):
                snapshots = []
                for flag in (False, True):
                    PREPARE["prepare"](studio, house, runner, house / "studio_notes.md", critic=critic, die_roll=flag)
                    snapshots.append({str(p.relative_to(root)): p.read_bytes() for directory in (studio, runner) for p in directory.rglob("*") if p.is_file()})
                    (studio / "BRIEF.md").unlink()
                    PREPARE["shutil"].rmtree(runner)
                off, on = snapshots
                self.assertEqual(off["runner/run.py"], (REPO / "harness/painter/run_friedrich.py").read_bytes())
                revision = json.loads(off["runner/run-config.json"])["source_revision"]
                normalized = {name: hashlib.sha256(data.replace(str(root).encode(), b"/fixture").replace(str(REPO).encode(), b"/repo").replace(revision.encode(), b"<revision>")).hexdigest() for name, data in off.items() if name != "runner/run.py"}
                self.assertEqual(normalized, baseline["with_critic" if critic else "without_critic"])
                self.assertEqual(on["studio/BRIEF.md"], off["studio/BRIEF.md"].replace(b"## Working\n", PREPARE["DIE_ROLL_SECTION"].encode() + b"## Working\n"))
                self.assertEqual(on["runner/brief_template.md"], template.replace("## Working\n", PREPARE["DIE_ROLL_SECTION"] + "## Working\n").encode())
                self.assertTrue(json.loads(on["runner/run-config.json"])["die_roll"])
                hook = b'    if R.paint("OPUS", 1, STUDIO, RUN / "OPUS") is None:\n'
                self.assertEqual(off["runner/run.py"].count(hook), 1)
                self.assertEqual(on["runner/run.py"], off["runner/run.py"].replace(hook, b"    from die_roll import choose\n    R.DIE_ROLL_OPENING = choose(R, MODEL, STUDIO, RUN)\n" + hook))
                namespace = {"__name__": "generated", "t": {"profile": "other"}, "PAINTER_MSG": "start", "DIE_ROLL_OPENING": "The die picked concept 3: Picture 3.", "n": 1}
                exec(on["runner/r21_chains.py"], namespace)
                self.assertEqual(namespace["paint"](1, 0, False), "The die picked concept 3: Picture 3.\n\nstart")
                self.assertEqual(namespace["paint"](2, 0, False), "The die picked concept 3: Picture 3.\n\nstart")
                self.assertEqual(namespace["paint"](2, 0, True), "start")
                self.assertEqual(namespace["paint"](2, 1, False), "later")


class PreparationInputTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.studio, self.house, self.runner = (self.root / name for name in ("studio", "house", "prepared/runner"))
        (self.studio / "bin").mkdir(parents=True)
        (self.studio / "notes/research").mkdir(parents=True)
        (self.studio / "paintings/lua").mkdir(parents=True)
        (self.studio / "bin/box").write_text("friedrich\n")
        (self.studio / "notes/journal.md").write_text("")
        (self.studio / "notes/reference-library.json").write_text("{}")
        self.house.mkdir()
        (self.house / "r21_chains.py").write_text(HOUSE)
        (self.house / "brief_template.md").write_text("{OPENING}\n{STUDIO}\n{READING}\n\n## Working\nWork.\n")
        for name in ("painting_chunks.py", "defaults_used.py", "log_copy.py", "copied_shapes.py", "studio_notes.md"):
            (self.house / name).write_text("plain\n")
        self.before = {p.relative_to(self.studio): p.read_bytes() for p in self.studio.rglob("*") if p.is_file()}

    def test_missing_house_helper_leaves_no_residue_and_allows_retry(self):
        for name in ("painting_chunks.py", "defaults_used.py", "log_copy.py", "copied_shapes.py"):
            with self.subTest(name=name):
                helper = self.house / name
                data = helper.read_bytes()
                helper.unlink()
                with self.assertRaises(FileNotFoundError):
                    PREPARE["prepare"](self.studio, self.house, self.runner, self.house / "studio_notes.md")
                self.assertFalse(self.runner.parent.exists())
                self.assertEqual({p.relative_to(self.studio): p.read_bytes() for p in self.studio.rglob("*") if p.is_file()}, self.before)
                helper.write_bytes(data)
                PREPARE["prepare"](self.studio, self.house, self.runner, self.house / "studio_notes.md")
                self.assertEqual((self.runner / name).read_bytes(), data)
                self.assertTrue((self.runner / "run-config.json").is_file())
                self.assertEqual((self.runner / "run.py").stat().st_mode, (REPO / "harness/painter/run_friedrich.py").stat().st_mode)
                PREPARE["shutil"].rmtree(self.runner.parent)
                (self.studio / "BRIEF.md").unlink()
                (self.studio / "notes/studio_notes.md").unlink()

    def test_runtime_inputs_are_checked_before_destinations_and_allow_retry(self):
        read_bytes = Path.read_bytes
        for die_roll, name, invalid in ((False, "run_friedrich.py", "missing"),
                                        (True, "run_friedrich.py", "missing"),
                                        (True, "die_roll.py", "missing"),
                                        (True, "run_friedrich.py", "no hook"),
                                        (True, "run_friedrich.py", "duplicate hook")):
            with self.subTest(die_roll=die_roll, name=name, invalid=invalid):
                path = REPO / "harness/painter" / name
                def read_input(candidate):
                    if candidate != path:
                        return read_bytes(candidate)
                    if invalid == "missing":
                        raise FileNotFoundError(str(candidate))
                    if invalid == "no hook":
                        return b"pass\n"
                    return read_bytes(candidate) * 2
                with patch.object(Path, "read_bytes", read_input), self.assertRaises((FileNotFoundError, ValueError)):
                    PREPARE["prepare"](self.studio, self.house, self.runner, self.house / "studio_notes.md", die_roll=die_roll)
                self.assertFalse(self.runner.parent.exists())
                self.assertEqual({p.relative_to(self.studio): p.read_bytes() for p in self.studio.rglob("*") if p.is_file()}, self.before)
                PREPARE["prepare"](self.studio, self.house, self.runner, self.house / "studio_notes.md", die_roll=die_roll)
                self.assertTrue((self.runner / "run-config.json").is_file())
                self.assertEqual((self.runner / "die_roll.py").exists(), die_roll)
                PREPARE["shutil"].rmtree(self.runner.parent)
                (self.studio / "BRIEF.md").unlink()
                (self.studio / "notes/studio_notes.md").unlink()


if __name__ == "__main__":
    unittest.main()
