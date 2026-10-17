import json
import re
import secrets


SITTING_ZERO = (
    'Your brief is in BRIEF.md. This sitting is only for writing notes/concepts.md as described under Choosing the picture. '
    'Write each estimate on its own line after the picture as Estimate: N/100, where N is a whole number from 0 to 100. '
    'Use write_concepts to save the whole file at once. Each save replaces it.'
)
RETRY = (
    "notes/concepts.md didn't parse. Replace it with eight entries numbered 1 to 8, each with a picture in a sentence or two "
    'followed by its own line Estimate: N/100, where N is a whole number from 0 to 100. '
    'Use write_concepts to save the whole file at once. Each save replaces it.'
)


def parse_concepts(text):
    entries = list(re.finditer(r"(?m)^[ \t]*([1-8])[.)][ \t]+", text))
    if len(entries) != 8 or [int(m[1]) for m in entries] != list(range(1, 9)) or text[:entries[0].start()].strip():
        raise ValueError("expected exactly eight entries numbered 1 to 8")
    concepts = []
    for i, match in enumerate(entries):
        body = text[match.end():entries[i + 1].start() if i < 7 else len(text)].strip()
        estimate = re.search(r"(?:^|\n)[ \t]*Estimate: ([0-9]{1,3})/100[ \t]*$", body)
        if not estimate or int(estimate[1]) > 100:
            raise ValueError(f"concept {i + 1}: expected a final Estimate: N/100 line, from 0 to 100")
        picture = body[:estimate.start()].strip()
        if not picture or re.search(r"(?mi)^\s*(?:Estimate:|\d+[.)]\s)", picture):
            raise ValueError(f"concept {i + 1}: expected one picture and one estimate")
        concepts.append({"number": i + 1, "text": picture, "estimate": int(estimate[1])})
    return concepts


def roll(concepts):
    dropped = sorted(concepts, key=lambda c: (-c["estimate"], c["number"]))[:2]
    remaining = [c for c in concepts if c not in dropped]
    return {"concepts": concepts, "dropped": [c["number"] for c in dropped], "pick": secrets.choice(remaining)}


def opening(result):
    return f'The die picked concept {result["pick"]["number"]}: {result["pick"]["text"]}'


def choose(runner, model, studio, run_dir):
    record = run_dir / "die_roll.json"
    if record.exists():
        return opening(json.loads(record.read_text()))
    logs = [studio / "paintings/lua/painting.lua", studio / "out/easel/painting/committed.lua"]
    before = [p.read_bytes() if p.exists() else None for p in logs]
    if runner.count_chunks(studio):
        raise RuntimeError("sitting 0 requires a pristine painting")
    state = run_dir / "sitting_0.json"
    attempts = json.loads(state.read_text()) if state.exists() else []
    if attempts and attempts[-1]["status"] != "invalid":
        raise RuntimeError("sitting 0 was interrupted or failed; refusing to guess or roll again")
    for attempt in range(len(attempts), 2):
        message = SITTING_ZERO if attempt == 0 else RETRY
        attempts.append({"sitting": 0, "attempt": attempt + 1, "message": message, "status": "running"})
        state.write_text(json.dumps(attempts, indent=2) + "\n")
        cmd = runner.painter_cmd(model, message)
        cmd[cmd.index("--tools") + 1] = "read,write_concepts"
        cmd[cmd.index(str(runner.H / "painter.ts"))] = str(runner.H / "concepts.ts")
        env = {**(model["env"] or {}), "PAINTER_CRITIC": ""}
        rc = runner.run(cmd, studio, run_dir / f"sitting_0_{attempt + 1}.out", run_dir / f"sitting_0_{attempt + 1}.err", env=env)
        if before != [p.read_bytes() if p.exists() else None for p in logs]:
            attempts[-1]["status"] = "marked"
            state.write_text(json.dumps(attempts, indent=2) + "\n")
            raise RuntimeError("SITTING 0 CHANGED THE PAINTING; nothing was cleaned up")
        if rc:
            attempts[-1]["status"] = "failed"
            state.write_text(json.dumps(attempts, indent=2) + "\n")
            raise RuntimeError(f"sitting 0 exited {rc}; see its stderr")
        try:
            concepts = parse_concepts((studio / "notes/concepts.md").read_text())
        except (ValueError, FileNotFoundError) as error:
            attempts[-1].update(status="invalid", error=str(error))
            state.write_text(json.dumps(attempts, indent=2) + "\n")
            continue
        result = roll(concepts)
        temporary = record.with_suffix(".json.tmp")
        temporary.write_text(json.dumps(result, indent=2) + "\n")
        temporary.replace(record)
        attempts[-1]["status"] = "completed"
        state.write_text(json.dumps(attempts, indent=2) + "\n")
        return opening(result)
    raise RuntimeError("notes/concepts.md did not parse after two sitting-0 attempts; no die was rolled")
