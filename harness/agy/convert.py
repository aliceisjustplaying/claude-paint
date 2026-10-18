# /// script
# requires-python = ">=3.10"
# dependencies = []
# ///
"""Project recorder tools and agy stream-json prose into a separate pi session.

The interactive transcript schema is unverified. Only documented stream-json
fields and an optional agent_response.thinking_delta string are interpreted.
Other events become custom records ignored by the studio viewer. Untimestamped
events use observation time in follow mode; historical interleaving is unknown.
"""

import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import tempfile
import time


def read_records(path, follow=False):
    try:
        data = path.read_bytes()
    except FileNotFoundError:
        if follow:
            return []
        raise
    if follow and not data.endswith(b"\n"):
        data = data[:data.rfind(b"\n") + 1]
    records = []
    for number, line in enumerate(data.splitlines(), 1):
        if not line.strip():
            continue
        try:
            records.append(json.loads(line))
        except ValueError:
            raise ValueError(f"invalid JSON in {path}, line {number}") from None
    return records


def merge_records(recorder, transcript, model=None, observed=None):
    if model is not None:
        model = model.strip()
    if any(not isinstance(row, dict) for row in recorder):
        raise ValueError("recorder entries must be pi event objects")
    session = next((row for row in recorder if row.get("type") == "session"), None)
    if session is None:
        raise ValueError("recorder has no session header")
    if not model:
        for row in transcript:
            if not isinstance(row, dict):
                continue
            init, status_model = row.get("init"), row.get("model")
            candidate = init.get("model") if row.get("event") == "init" and isinstance(init, dict) else None
            if candidate is None and isinstance(status_model, dict):
                candidate = status_model.get("id")
            if isinstance(candidate, str) and candidate.strip() and candidate.strip() != "Capy MCP":
                model = candidate.strip()
                break
    if model == "Capy MCP":
        raise ValueError("--model must identify the agy model, not the recorder")
    header = [session]
    if model:
        header.append({"type": "model_change", "modelId": model, "timestamp": session.get("timestamp", "")})
    events = [row for row in recorder if row.get("type") not in ("session", "model_change")]
    steps = {}
    has_prose = False
    for number, row in enumerate(transcript):
        payload = row.get(row.get("event")) if isinstance(row, dict) and isinstance(row.get("event"), str) else None
        payload = payload if isinstance(payload, dict) else {}
        timestamp = (row.get("timestamp") if isinstance(row, dict) else None) or payload.get("timestamp")
        if not isinstance(timestamp, str) or not timestamp:
            timestamp = session.get("timestamp", "")
            if observed is not None:
                signature = json.dumps(row, sort_keys=True)
                previous = observed.get(number)
                if previous is None or previous[0] != signature:
                    observed[number] = (signature, datetime.now(timezone.utc).isoformat(timespec="milliseconds"))
                timestamp = observed[number][1]
        kind = row.get("event") if isinstance(row, dict) else None
        if kind == "init" and isinstance(row.get("init"), dict):
            steps = {}
            has_prose = False
            continue
        if kind == "step_update" and payload.get("step_type") == "tool":
            continue
        if kind == "result" and isinstance(row.get("result"), dict):
            response = payload.get("response")
            if not has_prose and isinstance(response, str) and response:
                events.append({"type": "message", "timestamp": timestamp, "message": {
                    "role": "assistant", "content": [{"type": "text", "text": response}],
                }})
                has_prose = True
            continue
        if (kind == "step_update" and payload.get("step_type") == "agent_response"
                and isinstance(payload.get("step_index"), int)
                and isinstance(payload.get("conversation_id", ""), str)):
            blocks = []
            thinking, text = payload.get("thinking_delta"), payload.get("text_delta")
            if isinstance(thinking, str) and thinking:
                blocks.append({"type": "thinking", "thinking": thinking})
            if isinstance(text, str) and text:
                blocks.append({"type": "text", "text": text})
                has_prose = True
            if blocks:
                key = (payload.get("conversation_id"), payload.get("step_index"))
                event = steps.get(key)
                if event is None:
                    event = {"type": "message", "timestamp": timestamp, "message": {"role": "assistant", "content": []}}
                    steps[key] = event
                    events.append(event)
                content = event["message"]["content"]
                for block in blocks:
                    field = "thinking" if block["type"] == "thinking" else "text"
                    if content and content[-1]["type"] == block["type"]:
                        content[-1][field] += block[field]
                    else:
                        content.append(block)
                continue
        events.append({"type": "custom", "customType": "agy", "timestamp": timestamp, "data": row})

    def clock(event):
        try:
            parsed = datetime.fromisoformat(event.get("timestamp", "").replace("Z", "+00:00"))
            return (parsed if parsed.tzinfo else parsed.replace(tzinfo=timezone.utc)).timestamp()
        except (ValueError, TypeError, AttributeError):
            return 0

    return header + sorted(events, key=clock)


def project(recorder, transcript, output, model=None, follow=False, observed=None):
    sources = (recorder.resolve(), transcript.resolve())
    if output.resolve() in sources:
        raise ValueError("output must be distinct from both source files")
    rows = read_records(recorder, follow)
    if not rows and follow:
        return
    merged = merge_records(rows, read_records(transcript, follow), model, observed)
    data = "".join(json.dumps(row, ensure_ascii=False) + "\n" for row in merged).encode()
    output.parent.mkdir(parents=True, exist_ok=True)
    if not output.exists() or output.read_bytes() != data:
        fd, temporary = tempfile.mkstemp(prefix=".agy-", dir=output.parent)
        try:
            with os.fdopen(fd, "wb") as file:
                file.write(data)
                file.flush()
                os.fsync(file.fileno())
            os.replace(temporary, output)
        finally:
            if os.path.exists(temporary):
                os.unlink(temporary)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--recorder", type=Path, required=True)
    parser.add_argument("--transcript", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True, help="merged JSONL, distinct from both source files")
    parser.add_argument("--model", help="agy model; otherwise use documented init.model or model.id, or leave unlabeled")
    parser.add_argument("--follow", action="store_true", help="watch both files; never launch agy")
    args = parser.parse_args()
    recorder, transcript, output = (Path(os.path.abspath(path.expanduser())) for path in (args.recorder, args.transcript, args.output))
    observed = {} if args.follow else None
    try:
        while True:
            project(recorder, transcript, output, args.model, args.follow, observed)
            if not args.follow:
                break
            time.sleep(0.5)
    except KeyboardInterrupt:
        pass
    except (OSError, ValueError) as error:
        parser.exit(1, f"{error}\n")


if __name__ == "__main__":
    main()
