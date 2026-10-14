import hashlib
import json
from pathlib import Path

base = Path.home() / ".capy/work/wc-timeouts"
base.mkdir(parents=True, exist_ok=True)
manifest = []
for lane, lines in [(1, [524, 628, 792]), (2, [390, 751]), (3, [262, 452])]:
    directory = Path.home() / f".pi/agent/sessions/--Users-alice-src-a-stillwet-runs-rw{lane}-watercolor-paint-studio-8ee1cde{lane}--"
    session = next(directory.glob("*.jsonl"))
    records = [json.loads(line) for line in session.read_text().splitlines()]
    for line in lines:
        record = records[line - 1]
        call = next(c for c in record["message"]["content"] if c["type"] == "toolCall" and c["name"] == "paint")
        arguments = call["arguments"]
        source = arguments.get("code", arguments.get("lua", arguments.get("source")))
        if source is None:
            raise RuntimeError(arguments.keys())
        name = f"rw{lane}-L{line}"
        (base / f"{name}.lua").write_text(source)
        result = next(r for r in records if r.get("message", {}).get("toolCallId") == call["id"])
        item = dict(name=name, lane=lane, line=line, session=str(session), timestamp=record["timestamp"], arguments=arguments, result=result, sha256=hashlib.sha256(source.encode()).hexdigest())
        manifest.append(item)
        print(name, arguments.keys(), arguments.get("scratch"), arguments.get("new_scratch"))
    studio = Path(f"/Users/alice/src/a/stillwet-runs/rw{lane}-watercolor/paint-studio-8ee1cde{lane}")
    inventory = []
    for checkpoint in [studio / "checkpoint.json", *studio.glob("paintings/lua/*.json"), *studio.glob("out/watercolor/scratch/checkpoint.json")]:
        state = json.loads(checkpoint.read_text())
        sources = state.get("applied_sources", [])
        canvas = state.get("canvas") or {}
        inventory.append(dict(path=str(checkpoint), bytes=checkpoint.stat().st_size, sources=len(sources), clock=[canvas.get("tick"), canvas.get("residual_ns")], brushes=len(state.get("brushes", {}))))
        export = base / "states" / f"rw{lane}" / checkpoint.relative_to(studio).with_suffix(".sources.json")
        export.parent.mkdir(parents=True, exist_ok=True)
        export.write_text(json.dumps(sources, indent=2))
    (base / f"rw{lane}-states.json").write_text(json.dumps(inventory, indent=2))
    print(json.dumps(inventory, indent=2))
(base / "manifest.json").write_text(json.dumps(manifest, indent=2))
