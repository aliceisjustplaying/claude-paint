import json
import os
from pathlib import Path
import plistlib
import shutil
import subprocess
import sys
import tempfile
import unittest


REPO = Path(__file__).resolve().parents[2]


class LaunchdInstallTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.bin = self.root / "bin"
        self.bin.mkdir()
        self.home = self.root / "home"
        self.agents = self.home / "Library/LaunchAgents"
        self.agents.mkdir(parents=True)
        self.launchd = self.root / "launchd"
        self.launchd.mkdir()
        for path in (REPO / "notes/launchd").iterdir():
            if path.name == "install.sh" or path.name.endswith(".plist.in"):
                shutil.copy2(path, self.launchd / path.name)
        self.events = self.root / "events.jsonl"
        self.ip = ".".join(map(str, (100, 64, 0, 7)))
        self.env = dict(os.environ, HOME=str(self.home), PATH=f"{self.bin}:/usr/bin:/bin",
                        MOCK_EVENTS=str(self.events), MOCK_TAILNET_IP=self.ip,
                        MOCK_TAILSCALE_STATUS="0")
        self.mock("uv", "pass")
        self.mock("tailscale", 'print(os.environ["MOCK_TAILNET_IP"]); sys.exit(int(os.environ["MOCK_TAILSCALE_STATUS"]))')
        self.mock("plutil", 'plistlib.loads(Path(sys.argv[-1]).read_bytes())')
        self.mock("launchctl", "pass")

    def mock(self, name, action):
        path = self.bin / name
        path.write_text(f"#!{sys.executable}\nimport json, os, plistlib, sys\nfrom pathlib import Path\n"
                        f"with open(os.environ['MOCK_EVENTS'], 'a') as log:\n"
                        f"    log.write(json.dumps([{name!r}, *sys.argv[1:]]) + '\\n')\n{action}\n")
        path.chmod(0o755)

    def install(self):
        return subprocess.run(["bash", str(self.launchd / "install.sh")], env=self.env,
                              text=True, capture_output=True)

    def calls(self):
        return [json.loads(line) for line in self.events.read_text().splitlines()]

    def test_install_resolves_address_before_reloading_and_preserves_public_binding(self):
        result = self.install()
        self.assertEqual(result.returncode, 0, result.stderr)
        calls = self.calls()
        self.assertEqual(calls[0], ["tailscale", "ip", "-4"])
        for label, expected in (("tailnet", self.ip), ("public", "127.0.0.1")):
            path = self.agents / f"art.stillwet.studio.{label}.plist"
            config = plistlib.loads(path.read_bytes())
            args = config["ProgramArguments"]
            self.assertEqual(args[args.index("--host") + 1], expected)
            self.assertEqual(args[0], str(self.bin / "uv"))
            self.assertEqual(config["WorkingDirectory"], str(self.home / "src/a/claude-paint"))
            self.assertNotIn("@TAILNET_IP@", path.read_text())
            lint = next(i for i, call in enumerate(calls) if call == ["plutil", "-lint", str(path)])
            unload = next(i for i, call in enumerate(calls) if call[:2] == ["launchctl", "bootout"] and call[-1].endswith(f".{label}"))
            self.assertLess(lint, unload)
        self.assertEqual(sum(call[:2] == ["launchctl", "bootstrap"] for call in calls), 2)

    def test_invalid_or_unavailable_address_leaves_existing_services_untouched(self):
        addresses = ["", "not-an-address", "127.0.0.1", self.ip + "\n" + self.ip,
                     ".".join(map(str, (100, 63, 0, 7))),
                     ".".join(map(str, (100, 128, 0, 7))),
                     ".".join(map(str, (100, 64, 256, 7)))]
        for address, status in [(value, "0") for value in addresses] + [(self.ip, "1")]:
            with self.subTest(address=address, status=status):
                self.env.update(MOCK_TAILNET_IP=address, MOCK_TAILSCALE_STATUS=status)
                self.events.unlink(missing_ok=True)
                existing = self.agents / "art.stillwet.studio.tailnet.plist"
                existing.write_text("existing service")
                result = self.install()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("no viewer service was changed", result.stderr)
                self.assertEqual(self.calls(), [["tailscale", "ip", "-4"]])
                self.assertEqual(existing.read_text(), "existing service")
                self.assertEqual(list(self.agents.iterdir()), [existing])


if __name__ == "__main__":
    unittest.main()
