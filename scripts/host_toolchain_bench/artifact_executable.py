import json
import os
from pathlib import Path
import sys

stream = Path(sys.argv[1])
diagnostics = Path(sys.argv[2])
executables = set()
finished = False
errors = []
with diagnostics.open("w") as output:
    for number, line in enumerate(stream.read_text().splitlines(), 1):
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            if line.lstrip().startswith("{"):
                raise SystemExit(f"malformed Cargo JSON at {stream}:{number}")
            output.write(line + "\n")
            continue
        if not isinstance(message, dict) or "reason" not in message:
            output.write(line + "\n")
            continue
        reason = message["reason"]
        if reason == "build-finished":
            finished = message.get("success") is True
            if not finished:
                errors.append("Cargo reported build failure")
        if reason == "compiler-message" and message.get("message", {}).get("level") == "error":
            errors.append(message["message"].get("message", "compiler error"))
        if reason == "compiler-artifact" and message.get("executable"):
            target = message.get("target", {})
            if target.get("name") == "easel" and "bin" in target.get("kind", []) and message.get("profile", {}).get("test") is False:
                executables.add(message["executable"])
if errors or not finished:
    raise SystemExit("; ".join(errors) or "missing successful Cargo build-finished message")
if len(executables) != 1:
    raise SystemExit(f"expected one easel binary, found {len(executables)}")
executable = executables.pop()
if not Path(executable).is_file() or not os.access(executable, os.X_OK):
    raise SystemExit(f"missing executable: {executable}")
print(executable)
