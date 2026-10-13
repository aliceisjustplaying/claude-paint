# /// script
# requires-python = ">=3.10"
# dependencies = []
# ///

import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from convert import main, merge_records, project


PNG = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+/a9sAAAAASUVORK5CYII="


class ConvertTests(unittest.TestCase):
    def test_synthetic_projection_and_live_append(self):
        recorder_rows = [
            {"type": "session", "cwd": "/synthetic/paint-studio-aabbcc", "timestamp": "2026-10-13T12:00:00Z"},
            {"type": "model_change", "modelId": "Capy MCP"},
            {"type": "message", "timestamp": "2026-10-13T12:00:02Z", "message": {"role": "assistant", "content": [
                {"type": "toolCall", "id": "look-1", "name": "look", "arguments": {"call_id": "look-1"}},
            ]}},
            {"type": "message", "timestamp": "2026-10-13T12:00:03Z", "message": {"role": "toolResult", "toolCallId": "look-1", "content": [
                {"type": "text", "text": "out/easel/painting/example.png (1x1)"},
                {"type": "image", "mimeType": "image/png", "data": PNG},
            ]}},
        ]
        unknown = {"event": "future_event", "type": "message", "message": {"role": "assistant", "content": [{"type": "toolCall", "id": "unsafe"}]}}
        transcript_rows = [
            {"event": "init", "init": {"cwd": "/synthetic", "model": "documented-model"}},
            {"event": "step_update", "timestamp": "2026-10-13T12:00:01Z", "step_update": {
                "conversation_id": "synthetic", "step_index": 1, "state": "ACTIVE", "step_type": "agent_response",
                "thinking_delta": "Exposed synthetic thinking.", "text_delta": "I will ",
            }},
            {"event": "step_update", "timestamp": "2026-10-13T12:00:01Z", "step_update": {
                "conversation_id": "synthetic", "step_index": 1, "state": "DONE", "step_type": "agent_response", "text_delta": "look.",
            }},
            {"event": "step_update", "step_update": {"step_index": 2, "step_type": "tool", "state": "DONE", "tool_name": "look", "tool_info": {
                "name": "look", "parameters": {"call_id": "look-1"}, "output": "agy duplicate image result",
            }}},
            unknown,
            {"event": "result", "result": {"response": "I will look.", "usage": {"thinking_tokens": 10}}},
        ]
        with tempfile.TemporaryDirectory(prefix="agy-convert-") as root:
            root = Path(root)
            recorder = root / "studio/out/easel/mcp/session.jsonl"
            recorder.parent.mkdir(parents=True)
            transcript = root / "transcript.jsonl"
            output = root / "sessions/--mcp-src-a-paint-studio-aabbcc--/agy.jsonl"
            output.parent.mkdir(parents=True)
            recorder_sessions = root / "studio/out/easel/agy-recorder"
            recorder_sessions.mkdir(parents=True)
            raw_link = recorder_sessions / "raw-easel-mcp.jsonl"
            raw_link.symlink_to(recorder)
            viewer_link = output.parent / "existing-link.jsonl"
            viewer_link.symlink_to(recorder)
            other = output.parent / "other.jsonl"
            other.write_text("unrelated session\n")
            recorder.write_text("".join(json.dumps(row) + "\n" for row in recorder_rows))
            transcript.write_text("".join(json.dumps(row) + "\n" for row in transcript_rows))
            source_bytes = recorder.read_bytes(), transcript.read_bytes()
            project(recorder, transcript, output, "explicit-model")
            merged = [json.loads(line) for line in output.read_text().splitlines()]
            self.assertEqual(merged[0], recorder_rows[0])
            self.assertEqual(merged[1]["modelId"], "explicit-model")
            assistant = [row["message"]["content"] for row in merged if row.get("message", {}).get("role") == "assistant"]
            blocks = [block for content in assistant for block in content]
            self.assertEqual([block["text"] for block in blocks if block["type"] == "text"], ["I will look."])
            self.assertEqual([block["thinking"] for block in blocks if block["type"] == "thinking"], ["Exposed synthetic thinking."])
            self.assertEqual([block for block in blocks if block["type"] == "toolCall"], recorder_rows[2]["message"]["content"])
            results = [row for row in merged if row.get("message", {}).get("role") == "toolResult"]
            self.assertEqual(results, [recorder_rows[3]])
            self.assertEqual([row["data"] for row in merged if row["type"] == "custom"], [unknown])
            self.assertEqual(raw_link.resolve(), recorder.resolve())
            self.assertEqual(viewer_link.resolve(), recorder.resolve())
            self.assertEqual(other.read_text(), "unrelated session\n")
            self.assertEqual((recorder.read_bytes(), transcript.read_bytes()), source_bytes)
            self.assertEqual(merge_records(recorder_rows, transcript_rows)[1]["modelId"], "documented-model")
            self.assertEqual(merge_records(recorder_rows, [{"model": {"id": "statusline-model"}}])[1]["modelId"], "statusline-model")
            self.assertFalse(any(row["type"] == "model_change" for row in merge_records(recorder_rows, [])))
            with self.assertRaises(ValueError):
                merge_records(recorder_rows, transcript_rows, "Capy MCP")
            unfamiliar = {"event": "step_update", "step_update": {"step_type": "agent_response", "step_index": [], "text_delta": "unverified"}}
            self.assertEqual([row["data"] for row in merge_records(recorder_rows, [unfamiliar]) if row["type"] == "custom"], [unfamiliar])
            before = output.stat().st_ino
            project(recorder, transcript, output, "explicit-model")
            self.assertEqual(output.stat().st_ino, before)
            observed = {}
            tail = json.dumps({"event": "step_update", "step_update": {"step_index": 3, "step_type": "agent_response", "text_delta": "After looking."}}).encode()
            with transcript.open("ab") as file:
                file.write(tail[:len(tail) // 2])
            project(recorder, transcript, output, "explicit-model", follow=True, observed=observed)
            self.assertNotIn("After looking.", output.read_text())
            with transcript.open("ab") as file:
                file.write(tail[len(tail) // 2:] + b"\n")
            project(recorder, transcript, output, "explicit-model", follow=True, observed=observed)
            self.assertIn("After looking.", output.read_text())
            self.assertEqual(raw_link.resolve(), recorder.resolve())
            self.assertEqual(viewer_link.resolve(), recorder.resolve())
            before = output.read_bytes(), output.stat().st_ino
            project(recorder, transcript, output, "explicit-model", follow=True, observed=observed)
            self.assertEqual((output.read_bytes(), output.stat().st_ino), before)
            with self.assertRaises(ValueError):
                project(recorder, transcript, recorder, "explicit-model")
            self.assertEqual(recorder.read_bytes(), source_bytes[0])

    def test_follow_loop_waits_for_both_files_and_partial_result(self):
        with tempfile.TemporaryDirectory(prefix="agy-follow-") as root:
            root = Path(root)
            recorder, transcript, output = (root / name for name in ("recorder.jsonl", "transcript.jsonl", "merged.jsonl"))
            result = {"type": "message", "message": {"role": "toolResult", "toolCallId": "look-1", "content": [
                {"type": "image", "mimeType": "image/png", "data": PNG},
            ]}}
            encoded = json.dumps(result)
            ticks = 0

            def append_sources(_):
                nonlocal ticks
                ticks += 1
                if ticks == 1:
                    self.assertFalse(output.exists())
                    recorder.write_text(json.dumps({"type": "session", "cwd": "/synthetic"}) + "\n")
                    with recorder.open("a") as file:
                        file.write(json.dumps({"type": "message", "message": {"role": "assistant", "content": [
                            {"type": "toolCall", "id": "look-1", "name": "look", "arguments": {}},
                        ]}}) + "\n" + encoded[:20])
                    transcript.write_text(json.dumps({"event": "step_update", "step_update": {
                        "step_type": "agent_response", "step_index": 1, "text_delta": "Watching.",
                    }}) + "\n")
                elif ticks == 2:
                    self.assertIn("Watching.", output.read_text())
                    self.assertNotIn('"toolResult"', output.read_text())
                    with recorder.open("a") as file:
                        file.write(encoded[20:] + "\n")
                else:
                    rows = [json.loads(line) for line in output.read_text().splitlines()]
                    self.assertEqual([row for row in rows if row.get("message", {}).get("role") == "toolResult"], [result])
                    self.assertEqual(sum(block["type"] == "toolCall" for row in rows for block in row.get("message", {}).get("content", [])), 1)
                    raise KeyboardInterrupt

            with patch("sys.argv", ["convert.py", "--recorder", str(recorder), "--transcript", str(transcript),
                                    "--output", str(output), "--model", "synthetic-model", "--follow"]), patch("convert.time.sleep", append_sources):
                main()
            self.assertEqual(ticks, 3)


if __name__ == "__main__":
    unittest.main()
