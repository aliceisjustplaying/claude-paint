# /// script
# requires-python = ">=3.10"
# dependencies = []
# ///
"""Follow agy conversations started in one studio and keep the viewer feed converted.

Reads agy's launch logs (<agy-dir>/log/cli-*.log) for launches whose workspace is exactly the studio,
joins those conversations' transcripts in launch order into one file under --work, and keeps one
convert.py --follow running on it. Writes only under --work and the converter's output. Never starts,
stops or signals agy or the easel, and never touches studio files.
"""
import argparse
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import time

CONVERSATION = re.compile(r"(?:Created|Streaming) conversation ([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})")


def log(text):
    line = time.strftime("%F %T ") + text
    print(line, flush=True)


def conversations(agy, studio):
    found = []
    marker = f"workspaceDirs=[{studio}]"
    for path in sorted((agy / "log").glob("cli-*.log")):
        try:
            text = path.read_text(errors="replace")
        except OSError:
            continue
        if marker not in text:
            continue
        for match in CONVERSATION.finditer(text):
            if match.group(1) not in found:
                found.append(match.group(1))
    return found


def joined(agy, ids):
    parts = []
    for conversation in ids:
        path = agy / "brain" / conversation / ".system_generated/logs/transcript_full.jsonl"
        try:
            data = path.read_bytes()
        except FileNotFoundError:
            continue
        parts.append(data[:data.rfind(b"\n") + 1])
    return b"".join(parts)


def write_if_changed(path, data):
    if path.exists() and path.read_bytes() == data:
        return False
    temporary = path.with_name(path.name + ".part")
    temporary.write_bytes(data)
    os.replace(temporary, path)
    return True


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--studio", type=Path, required=True)
    parser.add_argument("--agy-dir", type=Path, required=True)
    parser.add_argument("--convert", type=Path, required=True)
    parser.add_argument("--model", required=True)
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--uv", default="uv")
    parser.add_argument("--interval", type=float, default=2)
    args = parser.parse_args()
    studio = args.studio.resolve(strict=True)
    recording = json.loads((studio / ".agents/recording.json").read_text())
    args.work.mkdir(parents=True, exist_ok=True)
    transcript = args.work / "transcript.jsonl"
    command = [args.uv, "run", "--no-project", str(args.convert), "--recorder", recording["recorder"],
               "--transcript", str(transcript), "--output", recording["output"], "--model", args.model, "--follow"]
    converter, known, retry_at = None, [], 0.0

    def stop(*_):
        if converter and converter.poll() is None:
            converter.terminate()
            try:
                converter.wait(10)
            except subprocess.TimeoutExpired:
                converter.kill()
        log("watcher stopped")
        raise SystemExit(0)

    signal.signal(signal.SIGTERM, stop)
    signal.signal(signal.SIGINT, stop)
    log(f"watching agy launches in {studio}; feed -> {recording['output']}")
    while True:
        ids = conversations(args.agy_dir, studio)
        for conversation in ids:
            if conversation not in known:
                known.append(conversation)
                log(f"agy conversation {conversation} in this studio; following it")
        if ids:
            write_if_changed(transcript, joined(args.agy_dir, ids))
            if converter and converter.poll() is not None:
                log(f"converter exited with {converter.returncode}; restarting in 5 s")
                converter, retry_at = None, time.time() + 5
            if converter is None and time.time() >= retry_at:
                with open(args.work / "convert.log", "ab") as out:
                    converter = subprocess.Popen(command, stdout=out, stderr=subprocess.STDOUT, cwd=args.work)
                log(f"converter pid {converter.pid} started")
        time.sleep(args.interval)


if __name__ == "__main__":
    main()
