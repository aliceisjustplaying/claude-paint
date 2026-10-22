import importlib.util
import json
import os
from pathlib import Path
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
        with patch.object(sys, "path", [str(REPO / "notes/round29/runner"), *sys.path]):
            spec.loader.exec_module(runner)
        return runner

    def test_stop_leftovers_uses_ps_from_path(self):
        for path in RUNNERS:
            with self.subTest(runner=str(path.relative_to(REPO))):
                self.events.write_text("")
                runner = self.runner(path)
                with patch.object(runner, "log"):
                    runner.stop_leftovers(self.studio)
                self.assertEqual([json.loads(line) for line in self.events.read_text().splitlines()],
                                 [["ps", "-Ao", "pid=,command="], ["kill", "424242"]])

    def test_round29_close_failure_keeps_server_and_retries_only_finalization(self):
        runner = self.runner(RUNNERS[-1])
        rd = self.root / "run/OPUS"
        rd.mkdir(parents=True)
        final = rd / "sitting-final.txt"
        final.write_text("A finished painting")
        sittings = [dict(sitting=1, status="completed", exit=0, reviewed=True,
                         painting_before=1, painting_after=1, final_file=str(final))]
        runner.save_sittings(rd, 1, sittings)
        self.studio.joinpath("bin").mkdir(parents=True)
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

    def test_round29_interrupted_close_leaves_a_durable_finalization_trigger(self):
        runner = self.runner(RUNNERS[-1])
        rd = self.root / "run/OPUS"
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
        runner = self.runner(RUNNERS[-1])
        for failure in [FileNotFoundError("missing easel"), subprocess.TimeoutExpired("easel close", 60)]:
            with self.subTest(failure=str(failure)), \
                 patch.object(runner.subprocess, "run", side_effect=failure), \
                 patch.object(runner, "stop_leftovers") as cleanup:
                self.assertIsInstance(runner.close_easel(self.studio), str)
                cleanup.assert_not_called()

    def test_stop_leftovers_matches_only_the_studio_executable(self):
        for path in RUNNERS:
            for keep_server in (False, True):
                for name in ("paint-studio-ab12", "paint studio-ab12"):
                    with self.subTest(runner=str(path.relative_to(REPO)), keep_server=keep_server, studio=name):
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
        for path in RUNNERS:
            with self.subTest(runner=str(path.relative_to(REPO))):
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
        for path in RUNNERS:
            with self.subTest(runner=str(path.relative_to(REPO))):
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
