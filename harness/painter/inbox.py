"""The run's inbox and delivery log, committed together by atomic replacement."""
import argparse
from contextlib import contextmanager
from datetime import datetime, timezone
import fcntl
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import uuid


FRAMING = "A message from the person who runs this studio:"
FILENAME = "painter-inbox.json"


def framed(messages):
    return "\n\n".join(FRAMING + "\n\n" + m["text"] + "\n\nEnd of message." for m in messages)


def timestamp():
    return datetime.now(timezone.utc).isoformat()


@contextmanager
def locked(run):
    path = run / FILENAME
    with (run / "painter-inbox.lock").open("a") as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        messages = json.loads(path.read_text()) if path.exists() else []
        before = json.dumps(messages, ensure_ascii=False, indent=2) + "\n"
        yield messages
        after = json.dumps(messages, ensure_ascii=False, indent=2) + "\n"
        if after != before or not path.exists():
            fd, name = tempfile.mkstemp(prefix=".painter-inbox-", dir=run)
            try:
                with os.fdopen(fd, "w") as f:
                    f.write(after)
                    f.flush()
                    os.fsync(f.fileno())
                os.replace(name, path)
                directory = os.open(run, os.O_RDONLY)
                try:
                    os.fsync(directory)
                finally:
                    os.close(directory)
            finally:
                if os.path.exists(name):
                    os.unlink(name)


def witnessed(message):
    delivery = message["delivery"]
    try:
        with Path(delivery["session"]).open("rb") as session:
            session.seek(delivery["offset"])
            lines = session.read().splitlines()
    except FileNotFoundError:
        return None
    for line in lines:
        try:
            m = json.loads(line).get("message", {})
        except ValueError:
            continue
        if delivery["tool_call"] is None:
            content = m.get("content", [])
            text = content if isinstance(content, str) else "\n".join(b.get("text", "") for b in content if b.get("type") == "text")
            if (m.get("role") == "user" and hashlib.sha256(text[:delivery["opening_chars"]].encode()).hexdigest()
                    == delivery["opening_hash"]):
                with Path(delivery["session"]).open("rb") as session:
                    os.fsync(session.fileno())
                return datetime.fromtimestamp(m["timestamp"] / 1000, timezone.utc).isoformat() if "timestamp" in m else delivery["attached_at"]
        elif (m.get("role") == "toolResult" and m.get("toolCallId") == delivery["tool_call"]
              and message["id"] in (m.get("details") or {}).get("painterInbox", [])):
            with Path(delivery["session"]).open("rb") as session:
                os.fsync(session.fileno())
            return datetime.fromtimestamp(m["timestamp"] / 1000, timezone.utc).isoformat() if "timestamp" in m else delivery["attached_at"]
    return None


def process_start(pid):
    return subprocess.run(["ps", "-o", "lstart=", "-p", str(pid)], env={**os.environ, "LC_ALL": "C", "TZ": "UTC"},
                          capture_output=True, text=True, check=False).stdout.strip()


def reconcile(messages):
    for message in messages:
        delivery = message.get("delivery")
        if not delivery or delivery["state"] == "delivered":
            continue
        alive = process_start(delivery["pid"]) == delivery["process_start"]
        delivered_at = witnessed(message)
        if delivered_at:
            delivery["state"] = "delivered"
            delivery["time"] = delivered_at
            delivery.pop("pid", None)
            delivery.pop("process_start", None)
            delivery.pop("offset", None)
            delivery.pop("opening_hash", None)
            delivery.pop("opening_chars", None)
            continue
        if not alive:
            message.pop("delivery")


def send(run, text):
    if not text.strip():
        raise ValueError("message is empty")
    message = {"id": str(uuid.uuid4()), "timestamp": timestamp(), "text": text}
    with locked(run) as messages:
        reconcile(messages)
        messages.append(message)
    return message


def receive(run, target=None):
    if not (run / FILENAME).exists():
        return []
    with locked(run) as messages:
        reconcile(messages)
        pending = [m for m in messages if "delivery" not in m] if target else []
        if pending:
            delivery = {**target, "attached_at": timestamp(), "state": "attaching"}
            session = Path(target["session"])
            delivery["offset"] = session.stat().st_size if session.exists() else 0
            delivery["process_start"] = process_start(target["pid"])
            if not delivery["process_start"]:
                raise ValueError("inbox receiver is no longer running")
            if target["tool_call"] is None:
                opening = framed(pending) + "\n\n"
                delivery["opening_hash"] = hashlib.sha256(opening.encode()).hexdigest()
                delivery["opening_chars"] = len(opening)
            for message in pending:
                message["delivery"] = dict(delivery)
        return pending


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("operation", choices=["send", "receive", "reconcile"])
    ap.add_argument("run", type=Path)
    ap.add_argument("value", nargs="?")
    args = ap.parse_args()
    run = args.run.resolve(strict=True)
    if args.operation == "send":
        print(json.dumps(send(run, args.value or ""), ensure_ascii=False))
    else:
        messages = receive(run, json.loads(args.value) if args.value else None)
        print(json.dumps([{"id": m["id"], "text": m["text"]} for m in messages], ensure_ascii=False))


if __name__ == "__main__":
    main()
