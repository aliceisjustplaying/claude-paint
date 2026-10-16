# /// script
# requires-python = ">=3.10"
# dependencies = []
# ///

from pathlib import Path
import tempfile
import unittest

from watch import conversations, joined


CONVERSATION = "717ee779-5806-44c3-af29-6a3bf13fcd3d"


class WatchTests(unittest.TestCase):
    def test_joins_the_full_transcript_not_the_truncated_one(self):
        with tempfile.TemporaryDirectory(prefix="agy-watch-") as root:
            agy, studio = Path(root) / "agy", Path(root) / "paint-studio-aabbcc"
            (agy / "log").mkdir(parents=True)
            (agy / "log" / "cli-1.log").write_text(f"workspaceDirs=[{studio}]\nCreated conversation {CONVERSATION}\n")
            logs = agy / "brain" / CONVERSATION / ".system_generated/logs"
            logs.mkdir(parents=True)
            full = b'{"step_index":0,"type":"PLANNER_RESPONSE","content":"#### 2. Practical Lessons & Working Hypotheses"}\n'
            (logs / "transcript_full.jsonl").write_bytes(full)
            (logs / "transcript.jsonl").write_bytes(
                b'{"step_index":0,"type":"PLANNER_RESPONSE","content":"#### 2. P\\n<truncated 1172 bytes>\\nies",'
                b'"truncated_fields":["content"]}\n')
            ids = conversations(agy, studio)
            self.assertEqual(ids, [CONVERSATION])
            self.assertEqual(joined(agy, ids), full)


if __name__ == "__main__":
    unittest.main()
