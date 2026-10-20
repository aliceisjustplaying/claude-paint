import importlib.util
import json
import os
from pathlib import Path
import sys
import tempfile
import threading
import unittest
from unittest.mock import patch


REPO = Path(__file__).resolve().parents[2]
RUNNERS = (REPO / "atelier/house/r21_chains.py",
           REPO / "scripts/tests/fixtures/r41/r21_chains.py")


class HouseCleanupTests(unittest.TestCase):
    def setUp(self):
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
