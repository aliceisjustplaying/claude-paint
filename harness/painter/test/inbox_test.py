import importlib.util
import json
import multiprocessing
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
from unittest.mock import patch


spec = importlib.util.spec_from_file_location("inbox", Path(__file__).resolve().parents[1] / "inbox.py")
inbox = importlib.util.module_from_spec(spec)
spec.loader.exec_module(inbox)


def take(run, session):
    inbox.receive(run, {"session": str(session), "sitting": "1", "tool_call": "call-1", "tool": "status", "pid": os.getpid()})


class InboxTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.run = Path(self.tmp.name)
        self.session = self.run / "session.jsonl"

    def target(self, call="call-1", pid=None):
        return {"session": str(self.session), "sitting": "1", "tool_call": call, "tool": "status" if call else None,
                "pid": pid or os.getpid()}

    def records(self):
        return json.loads((self.run / inbox.FILENAME).read_text())

    def persist(self, messages):
        self.session.write_text(json.dumps({"type": "message", "message": {"role": "toolResult", "toolCallId": "call-1",
                                "details": {"painterInbox": [m["id"] for m in messages]}}}) + "\n")

    def test_empty_inbox_changes_nothing(self):
        self.assertEqual(inbox.receive(self.run, self.target()), [])
        self.assertEqual(list(self.run.iterdir()), [])

    def test_receiver_identity_is_independent_of_sender_timezone(self):
        with patch.dict(os.environ, {"TZ": "UTC"}):
            first = inbox.process_start(os.getpid())
        with patch.dict(os.environ, {"TZ": "America/New_York"}):
            self.assertEqual(inbox.process_start(os.getpid()), first)
        self.assertTrue(first)

    def test_exactly_once_after_restart_between_transcript_and_commit(self):
        inbox.send(self.run, "Keep the blue.")
        process = multiprocessing.get_context("fork").Process(target=take, args=(self.run, self.session))
        process.start()
        process.join()
        self.assertEqual(process.exitcode, 0)
        messages = self.records()
        self.persist(messages)
        self.assertEqual(inbox.receive(self.run, self.target()), [])
        self.assertEqual(self.records()[0]["delivery"]["state"], "delivered")
        self.assertEqual(inbox.receive(self.run, self.target("call-2")), [])

    def test_restart_before_transcript_returns_message_to_inbox(self):
        inbox.send(self.run, "Keep the red.")
        process = multiprocessing.get_context("fork").Process(target=take, args=(self.run, self.session))
        process.start()
        process.join()
        messages = inbox.receive(self.run, self.target())
        self.assertEqual([m["text"] for m in messages], ["Keep the red."])
        self.persist(messages)
        inbox.receive(self.run)
        self.assertEqual(self.records()[0]["delivery"]["state"], "delivered")

    def test_receipt_written_just_before_receiver_death_is_not_lost(self):
        inbox.send(self.run, "Race.")
        inbox.receive(self.run, self.target())
        messages = self.records()

        def died(pid):
            self.persist(messages)
            return ""

        with patch.object(inbox, "process_start", died):
            self.assertEqual(inbox.receive(self.run, self.target("call-2")), [])
        self.assertEqual(self.records()[0]["delivery"]["state"], "delivered")

    def test_large_backlog_does_not_repeat_the_opening_in_each_record(self):
        with inbox.locked(self.run) as messages:
            messages.extend({"id": str(i), "timestamp": inbox.timestamp(), "text": "x" * 3000} for i in range(200))
        pending = inbox.receive(self.run, self.target(None))
        self.assertEqual(len(pending), 200)
        self.assertNotIn("opening", pending[0]["delivery"])
        self.assertLess(len(json.dumps(pending)), 800000)

    def test_concurrent_receivers_do_not_steal_live_attachment(self):
        inbox.send(self.run, "One message.")
        self.assertEqual(len(inbox.receive(self.run, self.target())), 1)
        self.assertEqual(inbox.receive(self.run, self.target("call-2")), [])

    def test_settle_releases_only_own_unpersisted_attachments_without_a_lease(self):
        inbox.send(self.run, "Retry.")
        inbox.receive(self.run, self.target())
        with inbox.locked(self.run) as messages:
            messages[0]["delivery"]["attached_at"] = "2000-01-01T00:00:00+00:00"
        self.assertEqual(inbox.receive(self.run, self.target("call-2")), [])
        self.assertEqual(inbox.settle(self.run, {"pid": os.getppid()}), [])
        self.assertEqual(self.records()[0]["delivery"]["state"], "attaching")
        self.assertEqual([m["text"] for m in inbox.settle(self.run, {"pid": os.getpid()})], ["Retry."])
        self.assertEqual(len(inbox.receive(self.run, self.target("call-2"))), 1)

    def test_settle_commits_saved_receipt_instead_of_releasing_it(self):
        inbox.send(self.run, "Saved.")
        self.persist(inbox.receive(self.run, self.target()))
        self.assertEqual(inbox.settle(self.run, {"pid": os.getpid()}), [])
        self.assertEqual(self.records()[0]["delivery"]["state"], "delivered")

    def test_house_runner_enables_inbox_only_for_painter_sittings(self):
        repo = Path(__file__).resolve().parents[3]
        runner = self.run / "runner"
        runner.mkdir()
        shutil.copy2(repo / "harness/painter/run_friedrich.py", runner / "run.py")
        studio = self.run / "studio"
        (studio / "bin").mkdir(parents=True)
        (studio / "bin/box").write_text("friedrich\n")
        (studio / "BRIEF.md").write_text("Brief.")
        (studio / ".painter-run").write_text(str(self.run) + "\n")
        (runner / "run-config.json").write_text(json.dumps({"studio": str(studio), "source": str(repo),
            "source_revision": subprocess.check_output(["git", "-C", str(repo), "rev-parse", "HEAD"], text=True).strip(),
            "run": str(self.run / "records")}))
        (runner / "r21_chains.py").write_text('''import json
import os
from pathlib import Path
from unittest.mock import patch
TOOLS = "read"
def model(*args, **kwargs): return {}
def lane(*args, **kwargs): return {}
def brief(*args, **kwargs): return "Brief."
def run(cmd, cwd, out, err, env=None):
    with (RUN / "calls.jsonl").open("a") as f:
        f.write(json.dumps(env) + "\\n")
    return 0
def paint(name, n, studio, rd):
    run(["pi"], studio, rd / "probe.txt", rd / "err", env={"KEEP": "yes"})
    replaced = []
    replace = os.replace
    def publish(source, target):
        assert target == studio / ".painter-run"
        assert Path(source).parent == studio
        assert target.read_text() == str(studio.parent) + "\\n"
        assert Path(source).read_text() == str(rd) + "\\n"
        replaced.append(target)
        return replace(source, target)
    with patch.object(os, "replace", publish):
        run(["pi"], studio, rd / "p1_s2_part3_final.txt", rd / "err", env={"KEEP": "yes"})
    assert replaced == [studio / ".painter-run"]
    assert not list(studio.glob(".painter-run-*"))
    return True
''')
        subprocess.run(["uv", "run", "--no-project", "python", str(runner / "run.py")], check=True,
                       env={**os.environ, "PAINTER_INBOX_DIR": "inherited", "PAINTER_INBOX_SITTING": "99"})
        calls = [json.loads(line) for line in (self.run / "records/calls.jsonl").read_text().splitlines()]
        inbox_dir = self.run / "records/OPUS"
        self.assertEqual(calls, [{"KEEP": "yes"}, {"KEEP": "yes", "PAINTER_INBOX_DIR": str(inbox_dir),
                                                  "PAINTER_INBOX_SITTING": "2"}])
        self.assertEqual((studio / ".painter-run").read_text(), str(inbox_dir) + "\n")
        subprocess.run([str(repo / "scripts/tell-painter"), str(studio), "Hello."], check=True, capture_output=True)
        self.assertEqual(json.loads((inbox_dir / inbox.FILENAME).read_text())[0]["text"], "Hello.")

    def test_tell_painter_rejects_empty_pointer_without_writing_to_sender_cwd(self):
        repo = Path(__file__).resolve().parents[3]
        studio = self.run / "studio"
        studio.mkdir()
        sender = self.run / "sender"
        sender.mkdir()
        for text in ("", " \n\t"):
            with self.subTest(pointer=text):
                (studio / ".painter-run").write_text(text)
                result = subprocess.run([str(repo / "scripts/tell-painter"), str(studio), "Hello."],
                                        cwd=sender, capture_output=True, text=True)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("FileNotFoundError", result.stderr)
                self.assertNotIn("queued", result.stdout)
                self.assertEqual(list(sender.iterdir()), [])
                self.assertEqual(list(studio.iterdir()), [studio / ".painter-run"])

    def test_an_older_identical_opening_is_not_a_delivery_receipt(self):
        text = "Again."
        self.session.write_text(json.dumps({"message": {"role": "user", "content": inbox.framed([{"text": text}]) + "\n\nOpening."}}) + "\n")
        inbox.send(self.run, text)
        inbox.receive(self.run, self.target(None))
        inbox.receive(self.run)
        self.assertEqual(self.records()[0]["delivery"]["state"], "attaching")

    def test_opening_delivery_and_send_after_delivery(self):
        inbox.send(self.run, "First.\nSecond line.")
        messages = inbox.receive(self.run, self.target(None))
        self.session.write_text(json.dumps({"message": {"role": "user", "content": inbox.framed(messages) + "\n\nOpening."}}) + "\n")
        inbox.send(self.run, "Next sitting.")
        records = self.records()
        self.assertEqual(records[0]["delivery"]["state"], "delivered")
        self.assertIsNone(records[0]["delivery"]["tool_call"])
        self.assertIn("time", records[0]["delivery"])
        self.assertNotIn("delivery", records[1])

    def test_concurrent_senders_preserve_all_messages(self):
        processes = [multiprocessing.get_context("fork").Process(target=inbox.send, args=(self.run, str(i))) for i in range(8)]
        for process in processes:
            process.start()
        for process in processes:
            process.join()
            self.assertEqual(process.exitcode, 0)
        self.assertEqual({m["text"] for m in self.records()}, {str(i) for i in range(8)})


if __name__ == "__main__":
    unittest.main()
