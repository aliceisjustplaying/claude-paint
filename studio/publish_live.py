# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow"]
# ///
import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import subprocess
import time

import export_static as E


def atomic(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    if path.exists() and path.read_bytes() == data:
        return False
    temp = path.with_name(path.name + ".tmp")
    temp.write_bytes(data)
    temp.chmod(0o644)
    os.replace(temp, path)
    return True


def snapshot(exported, staged):
    files = {}
    for rel in (exported / E.MARKER).read_text().splitlines():
        source = exported / rel
        if source.is_symlink() or not source.resolve().is_relative_to(exported.resolve()):
            raise ValueError("export contains an unsafe path")
        data = source.read_bytes()
        if rel == "index.html" or rel in E.S.VIEWER_ASSETS:
            atomic(staged / rel, data)
        else:
            obj = "objects/" + hashlib.sha256(data).hexdigest() + source.suffix
            atomic(staged / obj, data)
            files[rel] = obj
    data = json.dumps({"files": files}, sort_keys=True, separators=(",", ":")).encode()
    revision = hashlib.sha256(data).hexdigest()
    obj = f"objects/{revision}.json"
    atomic(staged / obj, data)
    manifest = json.dumps({"version": 1, "revision": revision, "snapshot": obj}, separators=(",", ":")).encode()
    return atomic(staged / "manifest.json", manifest)


def push(staged, destination, identity=None, local_root=None):
    if local_root is not None:
        for source in sorted(staged.rglob("*")):
            if source.is_file() and source.name != "manifest.json":
                atomic(local_root / source.relative_to(staged), source.read_bytes())
        atomic(local_root / "manifest.json", (staged / "manifest.json").read_bytes())
        return
    if not re.fullmatch(r"stillwet-live@[A-Za-z0-9.-]+:[A-Za-z0-9][A-Za-z0-9_-]*/", destination or ""):
        raise ValueError("destination must be stillwet-live@HOST:PAINTING/")
    ssh = "ssh -oBatchMode=yes -oStrictHostKeyChecking=yes -oConnectTimeout=10"
    if identity:
        ssh += " -oIdentitiesOnly=yes -i " + shlex.quote(str(identity))
    command = ["rsync", "-rc", "--chmod=Du=rwx,Dgo=rx,Fu=rw,Fgo=r", "--timeout=30", "-e", ssh]
    subprocess.run([*command, "--exclude=manifest.json", str(staged) + "/", destination], check=True, timeout=120)
    subprocess.run([*command, str(staged / "manifest.json"), destination], check=True, timeout=60)


def main():
    parser = argparse.ArgumentParser(description="Publish one public studio over outbound rsync, manifest last. No uploader runs by default.")
    parser.add_argument("--painter", required=True)
    parser.add_argument("--sessions-root", type=Path, required=True)
    parser.add_argument("--work-dir", type=Path, required=True)
    target = parser.add_mutually_exclusive_group(required=True)
    target.add_argument("--destination")
    target.add_argument("--local-root", type=Path, help="offline proof destination, no SSH")
    parser.add_argument("--identity", type=Path)
    parser.add_argument("--watch", action="store_true")
    parser.add_argument("--interval", type=float, default=2)
    args = parser.parse_args()
    if not re.fullmatch(r"paint-studio-[0-9a-f]+", args.painter) or not 0.2 <= args.interval <= 60:
        parser.error("use a paint-studio-<hex> painter and an interval between 0.2 and 60 seconds")
    work = args.work_dir.expanduser().resolve()
    sessions = args.sessions_root.expanduser().resolve(strict=True)
    if work == sessions or work.is_relative_to(sessions) or sessions.is_relative_to(work):
        parser.error("work-dir must be separate from sessions-root")
    work.mkdir(parents=True, exist_ok=True)
    E.S.SESSIONS = str(sessions)
    with (work / "publisher.lock").open("a") as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        previous = None
        while True:
            try:
                files = E.S.painter_files(args.painter)
                signature = [(f, os.stat(f).st_size, os.stat(f).st_mtime_ns) for f in files]
                if signature != previous:
                    E.export(str(work / "export"), only=args.painter, live=True)
                    snapshot(work / "export", work / "staged")
                    push(work / "staged", args.destination, args.identity, args.local_root)
                    previous = signature
                    print("published " + (work / "staged/manifest.json").read_text(), flush=True)
            except (OSError, ValueError, subprocess.SubprocessError) as error:
                if not args.watch:
                    raise
                print(f"publish failed: {error}", flush=True)
            if not args.watch:
                break
            time.sleep(args.interval)


if __name__ == "__main__":
    main()
