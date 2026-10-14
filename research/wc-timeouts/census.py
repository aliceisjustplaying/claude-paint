import json
from datetime import datetime
from pathlib import Path
import subprocess

base = Path.home() / ".capy/work/wc-timeouts"
rows = []
for lane in range(1, 4):
    directory = Path.home() / f".pi/agent/sessions/--Users-alice-src-a-stillwet-runs-rw{lane}-watercolor-paint-studio-8ee1cde{lane}--"
    session = next(directory.glob("*.jsonl"))
    records = [json.loads(line) for line in session.read_text().splitlines()]
    results = {r.get("message", {}).get("toolCallId"): r for r in records if r.get("message", {}).get("role") == "toolResult"}
    for line, record in enumerate(records, 1):
        message = record.get("message", {})
        content = message.get("content", [])
        if not isinstance(content, list):
            continue
        for call in content:
            if call.get("type") != "toolCall" or call.get("name") != "paint":
                continue
            result = results.get(call["id"])
            arguments = call["arguments"]
            source = arguments["lua"]
            name = f"rw{lane}-L{line}"
            path = base / "all-calls" / f"{name}.lua"
            path.parent.mkdir(exist_ok=True)
            path.write_text(source)
            trace = subprocess.run(["/etc/profiles/per-user/alice/bin/lua", str(base / "trace.lua"), str(path)], capture_output=True, text=True, timeout=10)
            row = dict(name=name, lane=lane, line=line, start=record["timestamp"], arguments=arguments, trace=trace.stdout, trace_exit=trace.returncode)
            if result:
                row.update(end=result["timestamp"], elapsed=(datetime.fromisoformat(result["timestamp"].replace("Z", "+00:00"))-datetime.fromisoformat(record["timestamp"].replace("Z", "+00:00"))).total_seconds(), result=result["message"])
            rows.append(row)
(base / "census.json").write_text(json.dumps(rows, indent=2))
for row in rows:
    if row["lane"] == 1 or row.get("result", {}).get("isError"):
        print(row["name"], row.get("elapsed"), row["arguments"].get("scratch", False), row["trace"].replace("\n", " "))
