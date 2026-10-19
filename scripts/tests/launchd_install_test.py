import configparser
import json
import os
from pathlib import Path
import plistlib
import shutil
import shlex
import subprocess
import sys
import tempfile
import unittest


REPO = Path(__file__).resolve().parents[2]


class InstallFixture:
    os_name = "Darwin"

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
            if path.name == "install.sh" or path.name.endswith(".in"):
                shutil.copy2(path, self.launchd / path.name)
        self.events = self.root / "events.jsonl"
        self.ip = ".".join(map(str, (100, 64, 0, 7)))
        self.env = dict(os.environ, HOME=str(self.home), PATH=f"{self.bin}:{os.environ['PATH']}",
                        MOCK_EVENTS=str(self.events), MOCK_TAILNET_IP=self.ip,
                        MOCK_TAILSCALE_STATUS="0", MOCK_OS=self.os_name)
        self.mock("uname", 'print(os.environ["MOCK_OS"])')
        self.mock("uv", "pass")
        self.mock("tailscale", 'print(os.environ["MOCK_TAILNET_IP"]); sys.exit(int(os.environ["MOCK_TAILSCALE_STATUS"]))')
        self.mock("plutil", 'plistlib.loads(Path(sys.argv[-1]).read_bytes())')
        self.mock("launchctl", "pass")
        self.mock("systemctl", "pass")

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

    def test_invalid_or_unavailable_address_leaves_existing_services_untouched(self):
        addresses = ["", "not-an-address", "127.0.0.1", self.ip + "\n" + self.ip,
                     ".".join(map(str, (100, 63, 0, 7))),
                     ".".join(map(str, (100, 128, 0, 7))),
                     ".".join(map(str, (100, 64, 256, 7))),
                     ".".join(map(str, (100, 64, 0, 256)))]
        service_dir = self.agents if self.os_name == "Darwin" else self.home / ".config/systemd/user"
        extension = "plist" if self.os_name == "Darwin" else "service"
        service_dir.mkdir(parents=True, exist_ok=True)
        existing = {service_dir / f"art.stillwet.studio.{label}.{extension}": f"existing {label} service"
                    for label in ("public", "tailnet")}
        for address, status in [(value, "0") for value in addresses] + [(self.ip, "1")]:
            with self.subTest(address=address, status=status):
                self.env.update(MOCK_TAILNET_IP=address, MOCK_TAILSCALE_STATUS=status)
                self.events.unlink(missing_ok=True)
                for path, content in existing.items():
                    path.write_text(content)
                result = self.install()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("no viewer service was changed", result.stderr)
                self.assertEqual(self.calls(), [["uname", "-s"], ["tailscale", "ip", "-4"]])
                self.assertEqual({path: path.read_text() for path in service_dir.iterdir()}, existing)
                self.assertFalse((self.home / "tmp/studio-viewer-logs").exists())


class LaunchdInstallTests(InstallFixture, unittest.TestCase):
    def test_install_resolves_address_before_reloading_and_preserves_public_binding(self):
        result = self.install()
        self.assertEqual(result.returncode, 0, result.stderr)
        calls = self.calls()
        self.assertEqual(calls[:2], [["uname", "-s"], ["tailscale", "ip", "-4"]])
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
        self.assertFalse((self.home / ".config").exists())
        self.assertFalse(any(call[0] == "systemctl" for call in calls))

    def test_install_with_tools_outside_system_bin(self):
        for name in ("bash", "dirname", "basename", "sed", "id"):
            (self.bin / name).symlink_to(shutil.which(name))
        self.env["PATH"] = str(self.bin)
        result = self.install()
        self.assertEqual(result.returncode, 0, result.stderr)


class SystemdInstallTests(InstallFixture, unittest.TestCase):
    os_name = "Linux"

    def setUp(self):
        super().setUp()
        self.env.pop("XDG_CONFIG_HOME", None)
        self.units = self.home / ".config/systemd/user"
        self.unit_names = [f"art.stillwet.studio.{label}.service" for label in ("public", "tailnet")]

    def test_install_renders_both_units_before_enabling_and_restarting(self):
        self.env["MOCK_UNIT_DIR"] = str(self.units)
        self.mock("systemctl", 'assert all((Path(os.environ["MOCK_UNIT_DIR"]) / name).is_file() '
                  'for name in ("art.stillwet.studio.public.service", "art.stillwet.studio.tailnet.service"))\n'
                  'assert (Path(os.environ["HOME"]) / "tmp/studio-viewer-logs").is_dir()')
        result = self.install()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.calls(), [
            ["uname", "-s"], ["tailscale", "ip", "-4"],
            ["systemctl", "--user", "daemon-reload"],
            ["systemctl", "--user", "enable", *self.unit_names],
            ["systemctl", "--user", "restart", *self.unit_names],
        ])
        self.assertEqual(sorted(path.name for path in self.units.iterdir()), self.unit_names)
        for label, host, port in (("public", "127.0.0.1", "8766"), ("tailnet", self.ip, "8765")):
            path = self.units / f"art.stillwet.studio.{label}.service"
            config = configparser.ConfigParser(interpolation=None)
            config.read(path)
            service = config["Service"]
            expected = [str(self.bin / "uv"), "run", "studio/studio.py"]
            if label == "public":
                expected.append("--public")
            expected.extend(["--host", host, "--port", port])
            self.assertEqual(shlex.split(service["ExecStart"]), expected)
            self.assertEqual(service["WorkingDirectory"], "%h/src/a/claude-paint")
            self.assertEqual(shlex.split(service["Environment"]),
                             [f"PATH={self.bin}:/usr/local/bin:/usr/bin:/bin", "HOME=%h"])
            self.assertEqual(service["Restart"], "always")
            self.assertEqual(service["RestartSec"], "15s")
            log = f"append:%h/tmp/studio-viewer-logs/art.stillwet.studio.{label}.log"
            self.assertEqual(service["StandardOutput"], log)
            self.assertEqual(service["StandardError"], log)
            self.assertEqual(config["Install"]["WantedBy"], "default.target")
            self.assertNotIn("@", path.read_text())
        self.assertEqual(list(self.agents.iterdir()), [])

    @unittest.skipUnless(shutil.which("systemd-analyze"), "systemd-analyze is not installed")
    def test_rendered_units_pass_systemd_verification(self):
        original_bin = self.bin
        self.bin = self.root / "tools & % $"
        original_bin.rename(self.bin)
        self.env["PATH"] = f"{self.bin}:{os.environ['PATH']}"
        result = self.install()
        self.assertEqual(result.returncode, 0, result.stderr)
        result = subprocess.run(["systemd-analyze", "verify", *[str(self.units / name) for name in self.unit_names]],
                                text=True, capture_output=True)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_install_with_tools_outside_system_bin(self):
        for name in ("bash", "dirname", "basename", "mkdir"):
            (self.bin / name).symlink_to(shutil.which(name))
        self.env["PATH"] = str(self.bin)
        result = self.install()
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_reinstall_replaces_units_and_restarts_enabled_services(self):
        self.units.mkdir(parents=True)
        for name in self.unit_names:
            (self.units / name).write_text("old service")
        result = self.install()
        self.assertEqual(result.returncode, 0, result.stderr)
        for name in self.unit_names:
            self.assertIn("[Service]", (self.units / name).read_text())
        self.assertEqual(self.calls()[-1], ["systemctl", "--user", "restart", *self.unit_names])

    def test_install_honors_xdg_config_home(self):
        config_home = self.home / "custom config"
        self.env["XDG_CONFIG_HOME"] = str(config_home)
        result = self.install()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(sorted(path.name for path in (config_home / "systemd/user").iterdir()), self.unit_names)
        self.assertFalse(self.units.exists())

    def test_invalid_address_does_not_create_directories(self):
        self.env["MOCK_TAILNET_IP"] = "invalid"
        result = self.install()
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse((self.home / ".config").exists())
        self.assertFalse((self.home / "tmp").exists())

    def test_systemctl_failure_is_reported_without_starting_units(self):
        self.mock("systemctl", "sys.exit(1)")
        result = self.install()
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(self.calls()[-1], ["systemctl", "--user", "daemon-reload"])
        self.assertNotIn("loaded", result.stdout)

    def test_unsupported_operating_system_does_not_touch_services(self):
        self.env["MOCK_OS"] = "FreeBSD"
        result = self.install()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("unsupported operating system", result.stderr)
        self.assertEqual(self.calls(), [["uname", "-s"]])
        self.assertFalse((self.home / ".config").exists())


if __name__ == "__main__":
    unittest.main()
