import json
from pathlib import Path
import shutil
import subprocess
import sys

import pytest

sys.path.insert(0, str(Path(__file__).parent))
import export_static as E
import publish_live as L


def exported(root, content=b"one"):
    files = {"index.html": b"live viewer", "stream.css": b"css", "data/sessions.json": b"[]",
             "data/paint-studio-abc/events.json": content, "data/paint-studio-abc/gallery/0.png": b"picture"}
    root.mkdir(exist_ok=True)
    for rel, data in files.items():
        path = root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
    (root / E.MARKER).write_text("\n".join(files))


def read_snapshot(root):
    manifest = json.loads((root / "manifest.json").read_text())
    files = json.loads((root / manifest["snapshot"]).read_text())["files"]
    return manifest, {rel: (root / obj).read_bytes() for rel, obj in files.items()}


def test_incremental_immutable_snapshot_and_manifest_last(tmp_path, monkeypatch):
    export, staged, hosted = (tmp_path / name for name in ("export", "staged", "hosted"))
    exported(export)
    assert L.snapshot(export, staged)
    L.push(staged, None, local_root=hosted)
    first, files = read_snapshot(hosted)
    assert files["data/paint-studio-abc/events.json"] == b"one"
    mtimes = {p: p.stat().st_mtime_ns for p in hosted.rglob("*") if p.is_file()}
    assert not L.snapshot(export, staged)
    L.push(staged, None, local_root=hosted)
    assert all(p.stat().st_mtime_ns == at for p, at in mtimes.items())
    exported(export, b"two")
    assert L.snapshot(export, staged)
    original = L.atomic
    calls = []

    def interrupted(path, data):
        calls.append(path)
        if path.name == "manifest.json":
            raise OSError("interrupted before commit")
        return original(path, data)

    monkeypatch.setattr(L, "atomic", interrupted)
    with pytest.raises(OSError):
        L.push(staged, None, local_root=hosted)
    assert calls[-1].name == "manifest.json"
    assert read_snapshot(hosted)[0] == first
    assert read_snapshot(hosted)[1] == files
    monkeypatch.setattr(L, "atomic", original)
    L.push(staged, None, local_root=hosted)
    assert read_snapshot(hosted)[1]["data/paint-studio-abc/events.json"] == b"two"
    for p, at in mtimes.items():
        if p.name != "manifest.json":
            assert p.stat().st_mtime_ns == at


def test_export_rejects_symlinks(tmp_path):
    export = tmp_path / "export"
    exported(export)
    (export / "data/sessions.json").unlink()
    (export / "data/sessions.json").symlink_to(tmp_path / "outside")
    with pytest.raises(ValueError, match="unsafe"):
        L.snapshot(export, tmp_path / "stage")


@pytest.mark.parametrize("destination", ["agent@vps:painting/", "stillwet-live@vps:../gallery/", "stillwet-live@vps:/var/lib/stillwet/", "stillwet-live@vps:paint;touch/", "--help"])
def test_restricted_destination(destination, tmp_path):
    with pytest.raises(ValueError):
        L.push(tmp_path, destination)


def test_transfer_failure_never_sends_manifest(tmp_path, monkeypatch):
    calls = []

    def failed(command, **kwargs):
        calls.append(command)
        raise subprocess.CalledProcessError(1, command)

    monkeypatch.setattr(L.subprocess, "run", failed)
    with pytest.raises(subprocess.CalledProcessError):
        L.push(tmp_path, "stillwet-live@fixture:painting/", identity=Path("key with spaces"))
    assert len(calls) == 1
    assert "--exclude=manifest.json" in calls[0]
    assert "-oStrictHostKeyChecking=yes" in calls[0][calls[0].index("-e") + 1]
    assert "-oIdentitiesOnly=yes -i 'key with spaces'" in calls[0][calls[0].index("-e") + 1]
    assert not any("--delete" in word or "--inplace" in word for word in calls[0])


@pytest.mark.skipif(not shutil.which("rrsync"), reason="rrsync is required for the restricted transport proof")
def test_real_rsync_through_forced_command(tmp_path, monkeypatch):
    import os
    export, staged, hosted = (tmp_path / name for name in ("export", "staged", "hosted"))
    hosted.mkdir()
    exported(export)
    L.snapshot(export, staged)
    shell = tmp_path / "rsh"
    shell.write_text(f"#!{sys.executable}\nimport os,sys\nos.environ['SSH_ORIGINAL_COMMAND']=' '.join(sys.argv[sys.argv.index('rsync'):])\nos.execv({shutil.which('rrsync')!r}, ['rrsync', '-wo', '-munge', '-no-del', {str(hosted)!r}])\n")
    shell.chmod(0o755)
    run = subprocess.run
    commands = []

    def transport(command, **kwargs):
        commands.append(command)
        command = list(command)
        command[command.index("-e") + 1] = str(shell)
        return run(command, **kwargs)

    monkeypatch.setattr(L.subprocess, "run", transport)
    L.push(staged, "stillwet-live@fixture:painting/")
    assert len(commands) == 2
    assert commands[-1][-2].endswith("/manifest.json")
    assert read_snapshot(hosted / "painting")[1]["data/paint-studio-abc/events.json"] == b"one"
    for command in ("sh -c id", "rsync --server --sender -r . painting/", "rsync --server -r --delete . painting/", "rsync --server -r . ../gallery/"):
        r = run([shutil.which("rrsync"), "-wo", "-munge", "-no-del", str(hosted)], env=dict(os.environ, SSH_ORIGINAL_COMMAND=command), capture_output=True)
        assert r.returncode != 0
    outside = tmp_path / "gallery"
    outside.mkdir()
    (outside / "keep").write_text("gallery")
    (staged / "escape").symlink_to(outside)
    run(["rsync", "-rl", "-e", str(shell), str(staged) + "/", "fixture:painting/"], check=True)
    assert os.readlink(hosted / "painting/escape").startswith("/rsyncd-munged/")
    assert (outside / "keep").read_text() == "gallery"


def test_live_export_is_relative_and_one_painter(tmp_path, monkeypatch):
    root = tmp_path / "sessions"
    for p in ("paint-studio-abc", "paint-studio-def"):
        folder = root / f"--mcp-src-a-{p}--"
        folder.mkdir(parents=True)
        (folder / "session.jsonl").write_text(json.dumps({"type": "session", "cwd": str(tmp_path / p)}) + "\n")
    monkeypatch.setattr(E.S, "SESSIONS", str(root))
    monkeypatch.setattr(E.S, "HOME", "/home/user")
    monkeypatch.setattr(E.S, "USER", "user")
    E.export(str(tmp_path / "export"), only="paint-studio-abc", live=True)
    page = (tmp_path / "export/index.html").read_text()
    assert "window.STUDIO_LIVE = true" in page
    assert '<base href="/studio/">' not in page
    assert "STUDIO_FEATURED=" not in page
    sessions = json.loads((tmp_path / "export/data/sessions.json").read_text())
    assert [s["p"] for s in sessions] == ["paint-studio-abc"]
    assert sessions[0]["active"] is True
    assert not (tmp_path / "export/data/paint-studio-def").exists()
    with pytest.raises(ValueError):
        E.export(str(tmp_path / "missing"), only="paint-studio-123", live=True)
