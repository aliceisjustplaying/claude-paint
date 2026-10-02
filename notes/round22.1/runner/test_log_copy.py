"""Tests of log_copy.py: the reader's copies of a painter's session logs (synthetic logs only)."""
import json

from log_copy import LINE_MAX, copy_logs
from records_fixtures import _call, _result, write_logs

BIG = "A" * 300_000                       # an image's base64, far over pi's 50 KB a line


def test_images_become_named_files_ids_stay_and_every_line_is_readable(tmp_path):
    studio = tmp_path / "studio"
    (studio / "out/easel/painting").mkdir(parents=True)
    (studio / "out/easel/painting/look-0001.png").write_bytes(b"png")
    look = _result("c2", "look", "out/easel/painting/look-0001.png (1000x714, 0.03s)", image=True)
    look["message"]["content"][1]["data"] = BIG
    ran = _call("c1", "paint", {"lua": "x = 1\n" + "-- long\n" * 5000})
    ran["message"]["diagnostics"] = {"blob": BIG}
    ran["message"]["content"].insert(0, {"type": "thinking", "thinking": "plan", "thinkingSignature": BIG})
    logs = write_logs(tmp_path, [[ran, _result("c1", "paint", "ok · chunk 1"), _call("c2", "look", {}), look]])
    copies, images = copy_logs(logs, studio, tmp_path / "copies")
    lines = open(copies[0]).read().splitlines()
    assert all(len(l.encode()) <= LINE_MAX for l in lines)
    assert images == [str(studio / "out/easel/painting/look-0001.png")]
    entries = [json.loads(l) for l in lines]
    ids = [c["id"] for e in entries for c in (e.get("message") or {}).get("content", []) if c.get("type") == "toolCall"]
    assert ids == ["c1", "c2"]
    shown = entries[-1]["message"]
    assert shown["toolCallId"] == "c2" and shown["content"][1] == {"type": "text", "text": f"[image: {images[0]}]"}
    assert "diagnostics" not in entries[1]["message"] and entries[1]["message"]["content"][0] == {"type": "thinking", "thinking": "plan"}
    assert "more characters cut in this copy" in entries[1]["message"]["content"][-1]["arguments"]["lua"]


def test_an_image_without_a_file_says_so(tmp_path):
    logs = write_logs(tmp_path, [[_call("c1", "look", {}), _result("c1", "look", "nothing here", image=True)]])
    copies, images = copy_logs(logs, tmp_path, tmp_path / "copies")
    assert images == [] and "[image: not saved as a file]" in open(copies[0]).read()
