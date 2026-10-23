import hashlib
import importlib.util
import json
import os
from pathlib import Path
import runpy
import shutil
import subprocess
import sys
import tempfile
import threading
import unittest
from unittest.mock import patch


REPO = Path(__file__).resolve().parents[2]
RUNNERS = (REPO / "atelier/house/r21_chains.py",
           REPO / "scripts/tests/fixtures/r41/r21_chains.py",
           REPO / "notes/round29/runner/r21_chains.py")


class HouseCleanupTests(unittest.TestCase):
    def setUp(self):
        self.ps = shutil.which("ps")
        self.sleep = shutil.which("sleep")
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        self.root = Path(temp.name)
        prepare = runpy.run_path(str(REPO / "scripts/prepare_friedrich_studio"))
        self.runners = list(RUNNERS)
        for i, path in enumerate(RUNNERS[:-1]):
            generated = self.root / f"generated-{i}.py"
            generated.write_text(prepare["corrected_runner"](path.read_text()))
            self.runners.append(generated)
        self.studio = self.root / "studio"
        self.events = self.root / "events.jsonl"
        self.bin = self.root / "bin"
        self.bin.mkdir()
        self.tool("ps", f"print('424242 ' + ('31:00 ' if 'etime=' in sys.argv[-1] else '') + {str(self.studio / 'bin/easel')!r} + ' paint')")
        self.tool("kill", "pass")
        self.tool("lsof", "pass")
        self.env = patch.dict(os.environ, PATH=str(self.bin))
        self.env.start()
        self.addCleanup(self.env.stop)

    def tool(self, name, action):
        path = self.bin / name
        path.write_text(f"#!{sys.executable}\nimport json, sys\n"
                        f"with open({str(self.events)!r}, 'a') as events:\n"
                        f"    events.write(json.dumps([{name!r}, *sys.argv[1:]]) + '\\n')\n"
                        f"{action}\n")
        path.chmod(0o755)

    def runner(self, path):
        spec = importlib.util.spec_from_file_location("house_cleanup_runner", path)
        runner = importlib.util.module_from_spec(spec)
        with patch.dict(sys.modules), \
             patch.object(sys, "path", [str(path.parent), str(REPO / "notes/round29/runner"), *sys.path]):
            sys.modules.pop("painting_chunks", None)
            spec.loader.exec_module(runner)
        return runner

    def test_stop_leftovers_uses_ps_from_path(self):
        for path in self.runners:
            with self.subTest(runner=str(path)):
                self.events.write_text("")
                runner = self.runner(path)
                with patch.object(runner, "log"):
                    runner.stop_leftovers(self.studio)
                self.assertEqual([json.loads(line) for line in self.events.read_text().splitlines()],
                                 [["ps", "-Ao", "pid=,command="], ["kill", "424242"]])

    def test_round29_close_failure_keeps_server_and_retries_only_finalization(self):
        for i, path in enumerate(self.runners):
            with self.subTest(runner=str(path)):
                runner = self.runner(path)
                rd = self.root / f"run-{i}/OPUS"
                rd.mkdir(parents=True)
                final = rd / "sitting-final.txt"
                final.write_text("A finished painting")
                sittings = [dict(sitting=1, status="completed", exit=0, reviewed=True,
                                 painting_before=1, painting_after=1, final_file=str(final))]
                runner.save_sittings(rd, 1, sittings)
                self.studio.joinpath("bin").mkdir(parents=True, exist_ok=True)
                self.studio.joinpath("bin/easel").touch()
                reason = runner.next_step(sittings)[1]
                outcome_path = rd / "p1_outcome.json"
                with patch.object(runner, "log"), \
                     patch.object(runner, "LANES", {"OPUS": {"model": {}}}), \
                     patch.object(runner, "open_easel") as opened, \
                     patch.object(runner, "run") as painter, \
                     patch.object(runner, "stop_leftovers") as cleanup, \
                     patch.object(runner.subprocess, "run", return_value=subprocess.CompletedProcess([], 1, "", "checkpoint disk full")):
                    self.assertIsNone(runner.paint("OPUS", 1, self.studio, rd))
                    cleanup.assert_not_called()
                    outcome = json.loads(outcome_path.read_text())
                    self.assertEqual(outcome["status"], "finalization_failed")
                    self.assertEqual(outcome["reason"], reason)
                    self.assertIn("checkpoint disk full", outcome["error"])
                    self.assertIsInstance(outcome["finished_at"], (int, float))
                    self.assertFalse((rd / "p1.painted").exists())
                    outcome["finished_at"] = 1792656000.0
                    outcome_path.write_text(json.dumps(outcome))
                    replace = os.replace
                    published = []

                    def publish(source, target):
                        if Path(target) == outcome_path:
                            self.assertEqual(json.loads(Path(target).read_text()), published[-1] if published else outcome)
                            value = json.loads(Path(source).read_text())
                            self.assertEqual(value["finished_at"], outcome["finished_at"])
                            published.append(value)
                        replace(source, target)

                    with patch.object(runner.subprocess, "run", return_value=subprocess.CompletedProcess([], 0, "closed", "")), \
                         patch.object(runner.os, "replace", side_effect=publish) as replaced, \
                         patch.object(runner.os, "fsync", wraps=os.fsync) as synced:
                        self.assertEqual(runner.paint("OPUS", 1, self.studio, rd), sittings)
                    self.assertEqual(sum(Path(call.args[1]) == outcome_path for call in replaced.call_args_list), 2)
                    self.assertEqual([value["status"] for value in published], ["finalization_failed", "finished"])
                    self.assertEqual(published[0]["error"], "final save pending")
                    self.assertGreaterEqual(synced.call_count, 4)
                    self.assertFalse(list(rd.glob(".p1_outcome.*.tmp")))
                    cleanup.assert_called_once_with(self.studio)
                    opened.assert_not_called()
                    painter.assert_not_called()
                recovered = json.loads(outcome_path.read_text())
                self.assertEqual(recovered["status"], "finished")
                self.assertEqual(recovered["reason"], reason)
                self.assertIsNone(recovered["error"])
                self.assertEqual(recovered["finished_at"], outcome["finished_at"])
                self.assertTrue((rd / "p1.painted").exists())

    def test_round29_finished_rerun_preserves_verified_outcome_without_close(self):
        for i, path in enumerate(self.runners):
            with self.subTest(runner=str(path)):
                runner = self.runner(path)
                rd = self.root / f"run-{i}/OPUS"
                rd.mkdir(parents=True)
                final = rd / "sitting-final.txt"
                final.write_text("A finished painting")
                sittings = [dict(sitting=1, status="completed", exit=0, reviewed=True,
                                 painting_before=1, painting_after=1, final_file=str(final))]
                runner.save_sittings(rd, 1, sittings)
                outcome_path = rd / "p1_outcome.json"
                outcome = dict(status="finished", reason=runner.next_step(sittings)[1],
                               error=None, finished_at=1792656000.0)
                runner.write_outcome(rd, 1, outcome)
                (rd / "p1.painted").write_text("verified finish\n")
                with patch.object(runner, "LANES", {"OPUS": {"model": {}}}), \
                     patch.object(runner, "log"), \
                     patch.object(runner, "close_easel", return_value="no painting open") as close, \
                     patch.object(runner, "open_easel") as opened, \
                     patch.object(runner, "run") as painter:
                    self.assertEqual(runner.paint("OPUS", 1, self.studio, rd), sittings)
                close.assert_not_called()
                opened.assert_not_called()
                painter.assert_not_called()
                self.assertEqual(json.loads(outcome_path.read_text()), outcome)
                self.assertEqual((rd / "p1.painted").read_text(), "verified finish\n")

    def test_close_exit_42_does_not_certify_a_saved_painting(self):
        self.tool("timeout", "import os\nos.execv(sys.argv[2], sys.argv[2:])")
        for i, path in enumerate(self.runners):
            with self.subTest(runner=str(path)):
                runner = self.runner(path)
                rd = self.root / f"run-{i}/OPUS"
                rd.mkdir(parents=True)
                final = rd / "final.txt"
                final.write_text("A finished painting")
                runner.save_sittings(rd, 1, [dict(sitting=1, status="completed", reviewed=True,
                                                chunks_before=1, chunks_after=1, final_file=str(final))])
                easel = self.studio / "bin/easel"
                easel.parent.mkdir(parents=True, exist_ok=True)
                easel.write_text(f"#!{sys.executable}\nimport sys\nsys.stderr.write('checkpoint disk full')\nsys.exit(42)\n")
                easel.chmod(0o755)
                with patch.object(runner, "log"), \
                     patch.object(runner, "LANES", {"OPUS": {"model": {}}}), \
                     patch.object(runner, "stop_leftovers") as cleanup, \
                     patch.object(runner, "open_easel") as opened, \
                     patch.object(runner, "run") as painter:
                    self.assertIsNone(runner.paint("OPUS", 1, self.studio, rd))
                outcome = json.loads((rd / "p1_outcome.json").read_text())
                self.assertEqual(outcome["status"], "finalization_failed")
                self.assertIn("42", outcome["error"])
                self.assertIn("checkpoint disk full", outcome["error"])
                self.assertFalse((rd / "p1.painted").exists())
                cleanup.assert_not_called()
                opened.assert_not_called()
                painter.assert_not_called()

    def test_round29_interrupted_close_leaves_a_durable_finalization_trigger(self):
        for i, path in enumerate(self.runners):
            with self.subTest(runner=str(path)):
                runner = self.runner(path)
                rd = self.root / f"run-{i}/OPUS"
                rd.mkdir(parents=True)
                sittings = [dict(sitting=1, status="completed", exit=0, reviewed=True,
                                 painting_before=1, painting_after=1)]
                runner.save_sittings(rd, 1, sittings)
                with patch.object(runner, "log"), \
                     patch.object(runner, "LANES", {"OPUS": {"model": {}}}), \
                     patch.object(runner.time, "time", return_value=1792656000.0), \
                     patch.object(runner, "close_easel", side_effect=KeyboardInterrupt), \
                     patch.object(runner, "open_easel") as opened, \
                     patch.object(runner, "run") as painter, \
                     patch.object(runner, "stop_leftovers") as cleanup, \
                     self.assertRaises(KeyboardInterrupt):
                    runner.paint("OPUS", 1, self.studio, rd)
                self.assertEqual(json.loads((rd / "p1_outcome.json").read_text()),
                                 dict(status="finalization_failed", reason=runner.next_step(sittings)[1],
                                      error="final save pending", finished_at=1792656000.0))
                self.assertFalse((rd / "p1.painted").exists())
                opened.assert_not_called()
                painter.assert_not_called()
                cleanup.assert_not_called()

    def test_round29_close_missing_binary_and_timeout_do_not_kill_server(self):
        for path in self.runners:
            with self.subTest(runner=str(path)):
                runner = self.runner(path)
                for failure in [FileNotFoundError("missing easel"), subprocess.TimeoutExpired("easel close", 60)]:
                    with self.subTest(failure=str(failure)), \
                         patch.object(runner.subprocess, "run", side_effect=failure), \
                         patch.object(runner, "stop_leftovers") as cleanup:
                        self.assertIsInstance(runner.close_easel(self.studio), str)
                        cleanup.assert_not_called()

    def finalization_case(self, path, name):
        runner = self.runner(path)
        rd = self.root / name
        rd.mkdir(parents=True)
        final = rd / "sitting-final.txt"
        final.write_text("A finished painting")
        sittings = [dict(sitting=1, status="completed", exit=0, reviewed=True,
                         painting_before=1, painting_after=1, final_file=str(final))]
        runner.save_sittings(rd, 1, sittings)
        return runner, rd, sittings

    def test_finalization_recovers_after_close_succeeded_without_publishing_success(self):
        for i, path in enumerate(p for p in self.runners if p != RUNNERS[-1]):
            for failure in ("interrupted", "timeout"):
                with self.subTest(runner=str(path), failure=failure):
                    runner, rd, sittings = self.finalization_case(path, f"recovery-{i}-{failure}")
                    state = rd / "server"
                    state.write_text("open")
                    self.studio.joinpath("bin").mkdir(parents=True, exist_ok=True)
                    easel = self.studio / "bin/easel"
                    easel.write_text(f"#!{sys.executable}\nfrom pathlib import Path\nimport sys\n"
                                     f"state = Path({str(state)!r})\n"
                                     f"with open({str(self.events)!r}, 'a') as events:\n"
                                     "    events.write(sys.argv[1] + '\\n')\n"
                                     "if sys.argv[1] == 'open':\n    state.write_text('open')\n"
                                     "elif state.read_text() == 'open':\n    state.write_text('closed')\n"
                                     "else:\n    sys.stderr.write('no painting open: easel open')\n    sys.exit(1)\n")
                    easel.chmod(0o755)
                    self.events.write_text("")
                    write_outcome = runner.write_outcome
                    subprocess_run = subprocess.run

                    def interrupted_publish(rd, n, outcome):
                        if outcome["status"] == "finished":
                            raise KeyboardInterrupt
                        write_outcome(rd, n, outcome)

                    def timed_out_close(command, **options):
                        result = subprocess_run(command, **options)
                        if command[-1] == "close" and result.returncode == 0:
                            raise subprocess.TimeoutExpired(command, 60)
                        return result

                    with patch.object(runner, "log"), \
                         patch.object(runner, "LANES", {"OPUS": {"model": {}}}), \
                         patch.object(runner, "run") as painter, \
                         patch.object(runner, "stop_leftovers"):
                        if failure == "interrupted":
                            with patch.object(runner, "write_outcome", side_effect=interrupted_publish), \
                                 self.assertRaises(KeyboardInterrupt):
                                runner.paint("OPUS", 1, self.studio, rd)
                        else:
                            with patch.object(runner.subprocess, "run", side_effect=timed_out_close):
                                self.assertIsNone(runner.paint("OPUS", 1, self.studio, rd))
                        self.assertEqual(state.read_text(), "closed")
                        pending = json.loads((rd / "p1_outcome.json").read_text())
                        self.assertEqual(pending["status"], "finalization_failed")
                        self.assertEqual(runner.paint("OPUS", 1, self.studio, rd), sittings)
                        painter.assert_not_called()
                    self.assertEqual(self.events.read_text().splitlines(), ["close", "close", "open", "close"])
                    self.assertEqual(json.loads((rd / "p1_outcome.json").read_text()),
                                     dict(pending, status="finished", error=None))
                    self.assertEqual((rd / "p1_final.txt").read_text(), "A finished painting")

    def test_final_reply_failure_cannot_publish_success_and_retry_repairs_it(self):
        for i, path in enumerate(p for p in self.runners if p != RUNNERS[-1]):
            for failure in (OSError, KeyboardInterrupt):
                with self.subTest(runner=str(path), failure=failure.__name__):
                    runner, rd, sittings = self.finalization_case(path, f"reply-{i}-{failure.__name__}")
                    write_outcome = runner.write_outcome

                    def failed_copy(source, target):
                        Path(target).write_text("A partial reply")
                        raise failure("final reply publication failed")

                    def publish(rd, n, outcome):
                        if outcome["status"] == "finished":
                            self.assertEqual((rd / "p1_final.txt").read_text(), "A finished painting")
                            self.assertFalse((rd / "p1.painted").exists())
                        write_outcome(rd, n, outcome)

                    with patch.object(runner, "log"), \
                         patch.object(runner, "LANES", {"OPUS": {"model": {}}}), \
                         patch.object(runner, "close_easel", side_effect=[None, "no painting open", None]) as close, \
                         patch.object(runner, "open_easel", return_value=True) as opened, \
                         patch.object(runner, "run") as painter, \
                         patch.object(runner, "write_outcome", side_effect=publish):
                        with patch.object(runner.shutil, "copy", side_effect=failed_copy), \
                             self.assertRaises(failure):
                            runner.paint("OPUS", 1, self.studio, rd)
                        pending = json.loads((rd / "p1_outcome.json").read_text())
                        self.assertEqual(pending["status"], "finalization_failed")
                        self.assertFalse((rd / "p1.painted").exists())
                        self.assertEqual(runner.paint("OPUS", 1, self.studio, rd), sittings)
                        self.assertEqual(close.call_count, 3)
                        opened.assert_called_once_with(self.studio, "OPUS1")
                        painter.assert_not_called()
                    self.assertEqual(json.loads((rd / "p1_outcome.json").read_text()),
                                     dict(pending, status="finished", error=None))

    def test_finished_rerun_repairs_a_missing_or_partial_final_reply(self):
        for i, path in enumerate(p for p in self.runners if p != RUNNERS[-1]):
            runner, rd, sittings = self.finalization_case(path, f"finished-{i}")
            outcome = dict(status="finished", reason=runner.next_step(sittings)[1],
                           error=None, finished_at=1792656000.0)
            runner.write_outcome(rd, 1, outcome)
            for partial in (False, True):
                with self.subTest(runner=str(path), partial=partial):
                    if partial:
                        (rd / "p1_final.txt").write_text("A partial reply")
                    with patch.object(runner, "LANES", {"OPUS": {"model": {}}}), \
                         patch.object(runner, "close_easel") as close, \
                         patch.object(runner, "open_easel") as opened, \
                         patch.object(runner, "run") as painter:
                        self.assertEqual(runner.paint("OPUS", 1, self.studio, rd), sittings)
                    self.assertEqual((rd / "p1_final.txt").read_text(), "A finished painting")
                    self.assertEqual(json.loads((rd / "p1_outcome.json").read_text()), outcome)
                    self.assertEqual((rd / "p1.painted").read_text(),
                                     f"{outcome['finished_at']} {len(sittings)} record(s): {outcome['reason']}\n")
                    close.assert_not_called()
                    opened.assert_not_called()
                    painter.assert_not_called()

    def test_finished_rerun_leaves_an_already_correct_reply_untouched(self):
        for i, path in enumerate(p for p in self.runners if p != RUNNERS[-1]):
            with self.subTest(runner=str(path)):
                runner, rd, sittings = self.finalization_case(path, f"untouched-{i}")
                outcome = dict(status="finished", reason=runner.next_step(sittings)[1], finished_at=1792656000.0)
                runner.write_outcome(rd, 1, outcome)
                reply = rd / "p1_final.txt"
                reply.write_bytes(Path(sittings[-1]["final_file"]).read_bytes())
                before = reply.stat()
                with patch.object(runner, "LANES", {"OPUS": {"model": {}}}), \
                     patch.object(runner.shutil, "copy", side_effect=AssertionError("published reply overwritten")), \
                     patch.object(runner.os, "replace") as replaced:
                    self.assertEqual(runner.paint("OPUS", 1, self.studio, rd), sittings)
                replaced.assert_not_called()
                self.assertEqual((reply.stat().st_ino, reply.stat().st_mtime_ns), (before.st_ino, before.st_mtime_ns))
                self.assertEqual(reply.read_text(), "A finished painting")
                self.assertEqual(json.loads((rd / "p1_outcome.json").read_text()), outcome)

    def test_finished_reply_repairs_are_atomic_even_when_interrupted(self):
        for i, path in enumerate(p for p in self.runners if p != RUNNERS[-1]):
            for failure in (OSError, KeyboardInterrupt):
                for stage in ("fsync", "replace"):
                    with self.subTest(runner=str(path), failure=failure.__name__, stage=stage):
                        runner, rd, sittings = self.finalization_case(path, f"atomic-{i}-{failure.__name__}-{stage}")
                        outcome = dict(status="finished", reason=runner.next_step(sittings)[1], finished_at=1792656000.0)
                        runner.write_outcome(rd, 1, outcome)
                        reply = rd / "p1_final.txt"
                        reply.write_text("An old partial reply")
                        replace = os.replace

                        def publish(source, target):
                            self.assertEqual(Path(target), reply)
                            self.assertEqual(reply.read_text(), "An old partial reply")
                            self.assertEqual(json.loads((rd / "p1_outcome.json").read_text()), outcome)
                            self.assertEqual(Path(source).read_text(), "A finished painting")
                            replace(source, target)

                        with patch.object(runner, "LANES", {"OPUS": {"model": {}}}), \
                             patch.object(runner, "close_easel") as close, \
                             patch.object(runner, "open_easel") as opened, \
                             patch.object(runner, "run") as painter:
                            with patch.object(runner.os, stage, side_effect=failure("publication interrupted")), \
                                 self.assertRaises(failure):
                                runner.paint("OPUS", 1, self.studio, rd)
                            self.assertEqual(reply.read_text(), "An old partial reply")
                            self.assertEqual(json.loads((rd / "p1_outcome.json").read_text()), outcome)
                            self.assertFalse((rd / "p1.painted").exists())
                            self.assertFalse(list(rd.glob(".p1_final.*.tmp")))
                            with patch.object(runner.os, "replace", side_effect=publish) as replaced, \
                                 patch.object(runner.os, "fsync", wraps=os.fsync) as synced:
                                self.assertEqual(runner.paint("OPUS", 1, self.studio, rd), sittings)
                            replaced.assert_called_once()
                            self.assertEqual(synced.call_count, 2)
                            close.assert_not_called()
                            opened.assert_not_called()
                            painter.assert_not_called()
                        self.assertEqual(reply.read_text(), "A finished painting")
                        self.assertFalse(list(rd.glob(".p1_final.*.tmp")))

    def test_legacy_finished_outcome_repairs_marker_using_outcome_mtime(self):
        for i, path in enumerate(p for p in self.runners if p != RUNNERS[-1]):
            with self.subTest(runner=str(path)):
                runner, rd, sittings = self.finalization_case(path, f"legacy-{i}")
                outcome = dict(status="finished", reason=runner.next_step(sittings)[1])
                outcome_path = rd / "p1_outcome.json"
                runner.write_outcome(rd, 1, outcome)
                os.utime(outcome_path, (1792656000.0, 1792656000.0))
                with patch.object(runner, "LANES", {"OPUS": {"model": {}}}), \
                     patch.object(runner, "close_easel") as close, \
                     patch.object(runner, "open_easel") as opened, \
                     patch.object(runner, "run") as painter:
                    self.assertEqual(runner.paint("OPUS", 1, self.studio, rd), sittings)
                    self.assertEqual(runner.paint("OPUS", 1, self.studio, rd), sittings)
                close.assert_not_called()
                opened.assert_not_called()
                painter.assert_not_called()
                self.assertEqual((rd / "p1_final.txt").read_text(), "A finished painting")
                self.assertEqual((rd / "p1.painted").read_text(),
                                 f"1792656000.0 {len(sittings)} record(s): {outcome['reason']}\n")
                self.assertEqual(json.loads(outcome_path.read_text()), outcome)
                self.assertEqual(outcome_path.stat().st_mtime, 1792656000.0)

    def test_executed_effects_not_source_tokens_decide_whether_to_continue(self):
        for path in self.runners:
            with self.subTest(runner=str(path)):
                runner = self.runner(path)
                chunks = ['canvas{}', 'b=brush("round",4)']
                before = self.effects(runner, chunks, [1, 0])
                chunks.append('function possible_mark() b:touch(850,100) end')
                after = self.effects(runner, chunks, [1, 0, 0])
                self.assertEqual(before, after)
                sitting = dict(sitting=1, status="completed", reviewed=True,
                               chunks_before=2, chunks_after=3, effects_before=before, effects_after=after)
                self.assertIsNone(runner.next_step([sitting])[0])
                chunks.append('b["touch"](b,850,100)')
                marked = self.effects(runner, chunks, [1, 0, 0, 1])
                self.assertEqual(marked, after + 1)
                self.assertEqual(runner.next_step([dict(sitting, sitting=2, chunks_before=3, chunks_after=4,
                                                        effects_before=after, effects_after=marked)]), ("new", 3))

    def effects(self, runner, chunks, counts):
        source = "".join(f"--@ chunk {i}\n{chunk}\n" for i, chunk in enumerate(chunks, 1))
        lua = self.studio / "paintings/lua/painting.lua"
        lua.parent.mkdir(parents=True, exist_ok=True)
        lua.write_text(source)
        evidence = self.studio / "out/easel/painting/painting-effects.json"
        evidence.parent.mkdir(parents=True, exist_ok=True)
        evidence.write_text(json.dumps(dict(source_sha256=hashlib.sha256(source.encode()).hexdigest(),
                                           mark_effects=counts)))
        return runner.count_painting(self.studio)

    def review_session(self, actions):
        session = self.root / "review.jsonl"
        messages = []
        for i, (name, args) in enumerate(actions):
            messages.extend([
                dict(role="assistant", content=[dict(type="toolCall", id=str(i), name=name, arguments=args)]),
                dict(role="toolResult", toolCallId=str(i), toolName=name, isError=False,
                     content=[dict(type="image", data="fixture", mimeType="image/png")] if name == "look"
                     else [dict(type="text", text="0")]),
            ])
        session.write_text("\n".join(json.dumps(dict(message=m)) for m in messages))
        return session

    def test_scratch_views_do_not_establish_painting_review(self):
        for path in self.runners:
            with self.subTest(runner=str(path)):
                runner = self.runner(path)
                session = self.review_session([
                    ("look", {"scratch": True}),
                    ("look", {"scratch": True, "crop": "0,0,100,100"}),
                ])
                sitting = dict(sitting=2, status="completed", effects_before=1, effects_after=1,
                               chunks_before=2, chunks_after=2, reviewed=runner.session_reviewed([session]))
                self.assertFalse(sitting["reviewed"])
                self.assertEqual(runner.next_step([sitting]), ("new", 3))

    def test_query_after_review_finishes_without_another_sitting(self):
        for i, path in enumerate(self.runners):
            with self.subTest(runner=str(path)):
                runner = self.runner(path)
                rd = self.root / f"query-{i}"
                rd.mkdir()
                chunks = ['canvas{}', 'b=brush("round",4)']
                self.effects(runner, chunks, [1, 0])
                session = self.root / "review.jsonl"
                session.unlink(missing_ok=True)

                def painter(cmd, cwd, out, err, env=None):
                    self.assertFalse(session.exists(), "query-only review launched another sitting")
                    self.review_session([
                        ("look", {}), ("look", {"crop": "0,0,100,100"}),
                        ("paint", {"code": "print(clock())"}),
                    ])
                    self.effects(runner, chunks + ['print(clock())'], [1, 0, 0])
                    out.write_text("Finished")
                    err.write_text("")
                    return 0

                with patch.object(runner, "log"), \
                     patch.object(runner, "LANES", {"OPUS": {"model": runner.OPUS}}), \
                     patch.object(runner, "open_easel", return_value=True), \
                     patch.object(runner, "close_easel", return_value=None), \
                     patch.object(runner, "stop_leftovers"), \
                     patch.object(runner, "session_dir", return_value=self.root), \
                     patch.object(runner, "run", side_effect=painter):
                    sittings = runner.paint("OPUS", 1, self.studio, rd)
                self.assertEqual(len(sittings), 1)
                self.assertTrue(sittings[0]["reviewed"])
                self.assertFalse(runner.added_painting(sittings[0]))
                self.assertEqual(json.loads((rd / "p1_outcome.json").read_text())["status"], "finished")

    def test_unavailable_effects_fall_back_to_chunks_not_classified_painting(self):
        for path in self.runners:
            for invalid in ("missing", "stale"):
                with self.subTest(runner=str(path), evidence=invalid):
                    runner = self.runner(path)
                    self.effects(runner, ['canvas{}', 'b["touch"](b,850,100)'], [1, 1])
                    evidence = self.studio / "out/easel/painting/painting-effects.json"
                    if invalid == "missing":
                        evidence.unlink()
                    else:
                        with (self.studio / "paintings/lua/painting.lua").open("a") as source:
                            source.write("-- stale evidence\n")
                    self.assertIsNone(runner.count_painting(self.studio))
                    for before, after in ((1, None), (None, 2), (None, None)):
                        sitting = dict(sitting=1, status="completed", reviewed=True,
                                       chunks_before=1, chunks_after=2, painting_before=1, painting_after=1,
                                       effects_before=before, effects_after=after)
                        self.assertEqual(runner.next_step([sitting]), ("new", 2))

    def test_stop_leftovers_matches_only_the_studio_executable(self):
        for path in self.runners:
            for keep_server in (False, True):
                for name in ("paint-studio-ab12", "paint studio-ab12"):
                    with self.subTest(runner=str(path), keep_server=keep_server, studio=name):
                        runner = self.runner(path)
                        studio = self.root / name
                        easel = studio / "bin/easel"
                        commands = [
                            f"101 {easel} paint",
                            f"102 {easel} serve painting",
                            f"103 {studio}3/bin/easel paint",
                            f"104 /usr/bin/printf {easel} paint",
                            f"105 /usr/bin/editor {studio}/notes/easel_guide.md",
                            f"106 /usr/bin/python -c print('easel') {studio}",
                            f"107 {easel}-other paint",
                            f"108 {easel}/other paint",
                            f"109 /elsewhere{easel} paint",
                            f"110 /bin/sh -c {easel} paint",
                            f"111 {easel} paint finish_painting",
                            f"112 {easel} paint check_painting",
                            f"113 /repo/scripts/finish_painting {studio}/paintings/lua/painting.lua",
                            f"114 /repo/scripts/check_painting {studio} {studio}/out/easel",
                            f"115\t{easel}\tstatus",
                            f"116 {easel}",
                            f"{os.getpid()} {easel} paint",
                            f"0 {easel} paint",
                            f"1 {easel} paint",
                            f"-1 {easel} paint",
                            f"bad {easel} paint",
                            "117",
                            "",
                        ]
                        with patch.object(runner.subprocess, "run", return_value=subprocess.CompletedProcess([], 0, "\n".join(commands))) as run, \
                             patch.object(runner, "log"):
                            runner.stop_leftovers(studio, keep_server=keep_server)
                        killed = [call.args[0] for call in run.call_args_list[1:]]
                        expected = ["101", "115", "116"] if keep_server else ["101", "102", "115", "116"]
                        self.assertEqual(killed, [["kill", pid] for pid in expected])

    @unittest.skipUnless(sys.platform.startswith("linux"), "uses Linux argv[0] process display")
    def test_stop_leftovers_terminates_only_the_matching_child(self):
        self.assertIsNotNone(self.ps)
        self.assertIsNotNone(self.sleep)
        sleep_args = ["--coreutils-prog=sleep", "60"] if Path(self.sleep).resolve().name == "coreutils" else ["60"]
        for path in self.runners:
            with self.subTest(runner=str(path)):
                studio = self.root / "paint-studio-ab12"
                commands = ([str(studio / "bin/easel"), *sleep_args],
                            [str(self.root / "paint-studio-ab123/bin/easel"), *sleep_args],
                            [f"/usr/bin/printf {studio}/bin/easel", *sleep_args])
                children = []
                try:
                    for command in commands:
                        children.append(subprocess.Popen(command, executable=self.sleep))
                    listing = subprocess.check_output([self.ps, "-p", ",".join(str(p.pid) for p in children),
                                                       "-o", "pid=,command="], text=True)
                    for command in commands:
                        self.assertIn(command[0], listing)
                    runner = self.runner(path)
                    by_pid = {str(p.pid): p for p in children}

                    def run(command, **options):
                        if command[0] == "ps":
                            return subprocess.CompletedProcess(command, 0, listing)
                        self.assertEqual(command[0], "kill")
                        self.assertIn(command[1], by_pid)
                        by_pid[command[1]].terminate()
                        return subprocess.CompletedProcess(command, 0)

                    with patch.object(runner.subprocess, "run", side_effect=run), patch.object(runner, "log"):
                        runner.stop_leftovers(studio)
                    self.assertEqual(children[0].wait(timeout=5), -15)
                    self.assertTrue(all(p.poll() is None for p in children[1:]))
                finally:
                    for child in children:
                        if child.poll() is None:
                            child.terminate()
                        child.wait(timeout=5)

    def test_watchdog_uses_ps_from_path(self):
        for path in self.runners:
            with self.subTest(runner=str(path)):
                self.events.write_text("")
                runner = self.runner(path)
                stop = threading.Event()
                with patch.object(runner, "A", self.root), \
                     patch.object(runner, "our_studios", return_value=[("lane1", "studio")]), \
                     patch.object(runner, "monitor_histories"), \
                     patch.object(runner, "log") as log, \
                     patch.object(stop, "wait", side_effect=lambda _: stop.set()):
                    runner.watchdog(stop)
                self.assertEqual([json.loads(line) for line in self.events.read_text().splitlines()],
                                 [["ps", "-Ao", "pid=,etime=,command="],
                                  ["lsof", "-a", "-d", "cwd", "-Fpn"], ["kill", "424242"]])
                self.assertEqual(log.call_count, 1)
                self.assertIn("WATCHDOG stopped 424242", log.call_args.args[0])


if __name__ == "__main__":
    unittest.main()
