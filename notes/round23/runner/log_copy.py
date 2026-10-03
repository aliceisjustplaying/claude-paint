"""The reader's copies of a painter's session logs (round 21 triage, H06).

pi's read tool can't read a line over 50 KB, and a session log puts each message on one line: every
`look` result carries its image as base64, so the reader never saw those lines at all (neither the
picture nor the text beside it). The copy keeps every entry and every tool call id (the reader cites
them; record_schema checks the evidence against the original logs), with each image replaced by a
line naming its file, and any text long enough to push a line over the limit cut with a note. The
images themselves are listed for the reader to open as images.
"""
import json
import re
from pathlib import Path

LINE_MAX = 40_000            # under pi's 50 KB per line, with room for the JSON around the text
TEXT_MAX = 12_000            # a text or argument string longer than this is cut
# what the reader needs of a message; the rest (diagnostics, usage, signatures) only makes lines long
KEEP = {"role", "content", "toolCallId", "toolName", "isError", "stopReason", "errorMessage", "timestamp"}
PNG_IN_TEXT = re.compile(r"(\S+\.png)\b")


def _cut(s, limit=TEXT_MAX):
    return s if len(s) <= limit else s[:limit] + f"\n[... {len(s) - limit} more characters cut in this copy]"


def _image_file(text, studio):
    """The .png a look result names (its first such path), resolved in the studio, if it exists."""
    m = PNG_IN_TEXT.search(text or "")
    if not m:
        return None
    p = Path(m.group(1)).expanduser()
    p = p if p.is_absolute() else Path(studio) / p
    return p if p.is_file() else None


def _slim(value):
    """Strings in a tool call's arguments cut to TEXT_MAX."""
    if isinstance(value, str):
        return _cut(value)
    if isinstance(value, list):
        return [_slim(v) for v in value]
    if isinstance(value, dict):
        return {k: _slim(v) for k, v in value.items()}
    return value


def copy_log(src, dst, studio):
    """Write the reader's copy of session log src to dst; returns the image files its results showed."""
    images, out = [], []
    read_paths = {}
    for line in Path(src).read_text(errors="replace").splitlines():
        try:
            e = json.loads(line)
        except ValueError:
            continue
        m = e.get("message") if isinstance(e, dict) else None
        if isinstance(m, dict) and isinstance(m.get("content"), list):
            if m.get("role") == "assistant":
                for c in m["content"]:
                    if isinstance(c, dict) and c.get("type") == "toolCall":
                        if c.get("name") == "read" and isinstance((c.get("arguments") or {}).get("path"), str):
                            read_paths[c.get("id")] = c["arguments"]["path"]
                        c["arguments"] = _slim(c.get("arguments"))
            text = " ".join(c.get("text") or "" for c in m["content"] if isinstance(c, dict) and c.get("type") == "text")
            content = []
            for c in m["content"]:
                if not isinstance(c, dict):
                    continue
                if c.get("type") == "image":
                    f = _image_file(read_paths.get(m.get("toolCallId")) or text, studio)
                    if f and str(f) not in images:
                        images.append(str(f))
                    content.append({"type": "text", "text": f"[image: {f}]" if f else "[image: not saved as a file]"})
                elif c.get("type") in ("text", "thinking"):
                    key = c["type"]
                    content.append({"type": key, key: _cut(c.get(key) or "")})   # no signatures
                else:
                    content.append(c)
            m["content"] = content
            for k in [k for k in m if k not in KEEP]:   # pi's diagnostics, usage and the like
                del m[k]
        s = json.dumps(e, ensure_ascii=False)
        if len(s.encode()) > LINE_MAX:     # still too long (many blocks): keep the entry's shape, say so
            s = json.dumps({"type": e.get("type"), "id": e.get("id"),
                            "note": f"entry cut in this copy: {len(s)} characters"}, ensure_ascii=False)
        out.append(s)
    Path(dst).write_text("\n".join(out) + "\n")
    return images


def copy_logs(logs, studio, dst_dir):
    """Copies of every log in dst_dir (log1.jsonl, ...): (copies, image files)."""
    Path(dst_dir).mkdir(parents=True, exist_ok=True)
    copies, images = [], []
    for i, src in enumerate(logs, 1):
        dst = Path(dst_dir) / f"log{i}.jsonl"
        for f in copy_log(src, dst, studio):
            if f not in images:
                images.append(f)
        copies.append(str(dst))
    return copies, images
