import importlib.util
import json
import multiprocessing
import os
from pathlib import Path
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
