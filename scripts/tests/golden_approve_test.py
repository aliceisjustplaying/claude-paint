import importlib.util
import json
from pathlib import Path
import unittest
from unittest.mock import patch


REPO = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("safeguards", REPO / "scripts/safeguards_lib.py")
SAFEGUARDS = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SAFEGUARDS)


class GoldenApprovalTests(unittest.TestCase):
    def test_unavailable_approval_cannot_approve_changes_or_block_valid_approvals(self):
        missing, commit, blob = "f" * 40, "a" * 40, "b" * 40
        entries = [{"path": "changed.txt", "mode": "100644", "blob": blob},
                   {"path": "deleted.txt", "blob": "deleted"}]
        valid = {"format": SAFEGUARDS.APPROVAL_FORMAT, "commit": commit,
                 "approver": "reviewer", "builder": "builder",
                 "paths": [{"path": "approved.txt", "mode": "100644", "blob": blob}]}
        notes = {"missing-note": valid | {"commit": missing, "paths": entries},
                 "blob-note": valid | {"commit": blob, "paths": entries}, "valid-note": valid}

        def git(repo, *args, input=None):
            if args == ("notes", "--ref=golden-approvals", "list"):
                return "\n".join(f"{note} {record['commit']}" for note, record in notes.items())
            if args == ("cat-file", "--batch-check=%(objecttype)"):
                self.assertEqual(input, f"{missing}\n{blob}\n{commit}\n")
                return f"{missing} missing\nblob\ncommit"
            self.assertEqual(args[:2], ("cat-file", "blob"))
            return json.dumps(notes[args[2]])

        def entry_at(repo, target, path):
            if target != commit:
                raise SAFEGUARDS.Refuse("fatal: not a tree object")
            return None if path == "deleted.txt" else ("100644", blob)

        changes = [("M", "approved.txt", blob, "100644"),
                   ("M", "changed.txt", blob, "100644"), ("D", "deleted.txt", "deleted", None)]
        with patch.object(SAFEGUARDS, "git_ok", return_value=True), \
             patch.object(SAFEGUARDS, "git", side_effect=git), \
             patch.object(SAFEGUARDS, "entry_at", side_effect=entry_at), \
             patch.object(SAFEGUARDS, "golden_changes", return_value=(["*.txt"], changes)):
            _, _, unapproved, by = SAFEGUARDS.golden_check("repo", "base", "candidate")
            self.assertEqual(unapproved, changes[1:])
            self.assertEqual(set(by), {"approved.txt"})
            self.assertEqual([a.get("invalid") for a in SAFEGUARDS.read_approvals("repo")],
                             ["not an available commit", "not an available commit", None])
            valid["paths"].extend(entries)
            _, _, unapproved, by = SAFEGUARDS.golden_check("repo", "base", "candidate")
            self.assertEqual(unapproved, [])
            self.assertEqual(set(by), {"approved.txt", "changed.txt", "deleted.txt"})


if __name__ == "__main__":
    unittest.main()
