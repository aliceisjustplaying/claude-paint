"""Shared code for scripts/test_candidate, scripts/golden_approve and
scripts/merge_candidate (Python standard library only).

See notes/speed/SAFEGUARDS.md for how the three tools fit together.
"""

import base64
import datetime
import hashlib
import json
import os
import re
import subprocess

RECEIPTS_REF = "refs/notes/test-receipts"
APPROVALS_REF = "refs/notes/golden-approvals"
GOLDEN_LIST = "notes/golden_paths.txt"
RECEIPT_FORMAT = "claude-paint test receipt v1"
APPROVAL_FORMAT = "claude-paint golden approval v1"
HEX_ID = re.compile(r"[0-9a-f]{7,64}")
HEX64 = re.compile(r"[0-9a-f]{64}")
# The privacy hooks' protected text, stored encoded like scripts/pre-commit-anonymity.
_PROTECTED = base64.b64decode("c2FyYWg=").decode()


class Refuse(Exception):
    """A request the tools will not carry out; the message says why."""


def now_iso():
    return datetime.datetime.now().astimezone().isoformat(timespec="seconds")


def run(cmd, cwd=None, check=True, text=True, input=None, env=None):
    p = subprocess.run(cmd, cwd=cwd, input=input, env=env, text=text,
                       stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if check and p.returncode != 0:
        err = p.stderr if text else p.stderr.decode(errors="replace")
        raise Refuse("command failed (%d): %s\n%s" % (p.returncode, " ".join(cmd), err.strip()))
    return p


def git(repo, *args, check=True, text=True, input=None):
    """stdout of git -C repo ARGS (stripped when text)."""
    p = run(["git", "-C", repo] + list(args), check=check, text=text, input=input)
    return p.stdout.strip() if text else p.stdout


def git_ok(repo, *args):
    return run(["git", "-C", repo] + list(args), check=False).returncode == 0


def toplevel(path="."):
    p = run(["git", "-C", path, "rev-parse", "--show-toplevel"], check=False)
    if p.returncode != 0:
        raise Refuse("%s is not inside a Git working copy" % path)
    return p.stdout.strip()


def resolve_exact_commit(repo, arg, what="commit"):
    """Full id of the one commit ARG names. ARG must be a commit id (hex), not
    a branch, tag or other name: names can move between testing and merging."""
    if not HEX_ID.fullmatch(arg or ""):
        raise Refuse(
            "%s %r: give an exact commit id (7 to 64 lowercase hex digits). A branch or other "
            "name can move; resolve it once with `git rev-parse NAME` and pass the id." % (what, arg))
    refs = git(repo, "for-each-ref", "--format=%(refname)").split()
    clash = [r for r in refs if r == arg or r.endswith("/" + arg)]
    if clash:
        raise Refuse("%s %r is also a ref name (%s); refusing the ambiguous name" % (what, arg, clash[0]))
    objs = git(repo, "rev-parse", "--disambiguate=" + arg, check=False).split()
    commits = [o for o in objs if git(repo, "cat-file", "-t", o, check=False) == "commit"]
    if not commits:
        raise Refuse("%s %r: no such commit in this repository" % (what, arg))
    if len(commits) > 1:
        raise Refuse("%s %r is ambiguous (%d commits start with it); give more digits"
                     % (what, arg, len(commits)))
    full = commits[0]
    if not full.startswith(arg):
        raise Refuse("%s %r resolved to %s, which is not what was asked" % (what, arg, full))
    return full


def tree_of(repo, commit):
    return git(repo, "rev-parse", commit + "^{tree}")


def is_ancestor(repo, a, b):
    return git_ok(repo, "merge-base", "--is-ancestor", a, b)


def sha256_bytes(b):
    return hashlib.sha256(b).hexdigest()


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def blob_bytes(repo, commit, path):
    """Contents of PATH at COMMIT, or None if absent."""
    p = run(["git", "-C", repo, "cat-file", "blob", "%s:%s" % (commit, path)], check=False, text=False)
    return p.stdout if p.returncode == 0 else None


def blob_id(repo, commit, path):
    p = run(["git", "-C", repo, "rev-parse", "--verify", "-q", "%s:%s" % (commit, path)], check=False)
    return p.stdout.strip() if p.returncode == 0 else None


def tools_sha256(*paths):
    """One hash over the given tool files (name and content), for receipts."""
    h = hashlib.sha256()
    for p in paths:
        h.update(os.path.basename(p).encode() + b"\0")
        with open(p, "rb") as f:
            h.update(f.read())
        h.update(b"\0")
    return h.hexdigest()


def home_redact(text):
    """Write the home directory as ~ (receipts and approvals are published as notes)."""
    for home in {os.path.expanduser("~"), os.path.realpath(os.path.expanduser("~"))}:
        if home and home != "/":
            text = text.replace(home, "~")
    return text


def assert_publishable(text, what):
    if _PROTECTED in text.lower():
        raise Refuse("%s contains text the privacy hooks protect; not recording it" % what)


# ---------------------------------------------------------------- summaries

def _int(x):
    return type(x) is int


def summary_problems(summary):
    """Reasons a scripts/test summary is not a full pass ([] means pass)."""
    if not isinstance(summary, dict):
        return ["the summary is missing or not a JSON object"]
    p = []
    if summary.get("mode") != "all":
        p.append("summary mode is %r, not 'all'" % summary.get("mode"))
    if summary.get("verdict") != "pass":
        p.append("summary verdict is %r, not 'pass'" % summary.get("verdict"))
    if not HEX64.fullmatch(str(summary.get("list_sha256", ""))):
        p.append("summary has no valid list_sha256")
    steps = summary.get("steps")
    if not isinstance(steps, list) or not steps:
        p.append("summary has no steps: an empty test selection never passes")
        return p
    for i, s in enumerate(steps):
        if not isinstance(s, dict):
            p.append("step %d is not an object" % i)
            continue
        name = s.get("name") or "step %d" % i
        if s.get("exit") != 0 or not _int(s.get("exit")):
            p.append("step %s exited %r" % (name, s.get("exit")))
        if s.get("timed_out") is not False:
            p.append("step %s timed out (timed_out=%r)" % (name, s.get("timed_out")))
        if not _int(s.get("tests_run")) or s["tests_run"] < 1:
            p.append("step %s ran no tests (tests_run=%r)" % (name, s.get("tests_run")))
        if not _int(s.get("passed")) or s["passed"] < 1:
            p.append("step %s passed no tests (passed=%r)" % (name, s.get("passed")))
        if s.get("failed") != 0 or not _int(s.get("failed")):
            p.append("step %s has failures (failed=%r)" % (name, s.get("failed")))
    return p


# ---------------------------------------------------------------- golden set

def read_patterns(repo, commit):
    """Protected entries listed in notes/golden_paths.txt at COMMIT ([] if absent)."""
    data = blob_bytes(repo, commit, GOLDEN_LIST)
    if data is None:
        return []
    out = []
    for line in data.decode().splitlines():
        line = line.strip()
        if line and not line.startswith("#"):
            out.append(line)
    return out


def protected(path, patterns):
    """PATH is protected if it equals an entry or lies under an entry ending in /.
    The list file itself is always protected."""
    if path == GOLDEN_LIST:
        return True
    for pat in patterns:
        if pat.endswith("/") and path.startswith(pat):
            return True
        if path == pat:
            return True
    return False


def protected_files(repo, commit, patterns):
    names = git(repo, "ls-tree", "-r", "-z", "--name-only", commit).split("\0")
    return [n for n in names if n and protected(n, patterns)]


def read_approvals(repo):
    """Every approval line in refs/notes/golden-approvals, with validity."""
    if not git_ok(repo, "rev-parse", "--verify", "-q", APPROVALS_REF):
        return []
    out = []
    for line in git(repo, "notes", "--ref=golden-approvals", "list").splitlines():
        note, obj = line.split()
        text = git(repo, "cat-file", "blob", note)
        for raw in text.splitlines():
            raw = raw.strip()
            if not raw:
                continue
            try:
                a = json.loads(raw)
            except ValueError:
                out.append({"attached_to": obj, "invalid": "not JSON"})
                continue
            a = dict(a, attached_to=obj) if isinstance(a, dict) else {"attached_to": obj}
            out.append(a)
    return out


def approved_blobs(repo):
    """{(path, blob): [approval, ...]} from approvals that are valid: the note
    sits on the commit it names, approver != builder (case-insensitive), and each
    listed blob is really that path's blob at the commit ("deleted" if absent)."""
    ok = {}
    for a in read_approvals(repo):
        if a.get("invalid") or a.get("format") != APPROVAL_FORMAT:
            continue
        approver, builder = str(a.get("approver", "")).strip(), str(a.get("builder", "")).strip()
        if not approver or not builder or approver.casefold() == builder.casefold():
            continue
        if a.get("commit") != a["attached_to"]:
            continue
        for e in a.get("paths") or []:
            path, blob = e.get("path"), e.get("blob")
            if not path or not blob:
                continue
            actual = blob_id(repo, a["commit"], path)
            if (blob == "deleted" and actual is None) or (blob != "deleted" and blob == actual):
                ok.setdefault((path, blob), []).append(a)
    return ok


def golden_changes(repo, base, cand):
    """Protected paths that differ between BASE and CAND: [(status, path, blob)].
    The protected set is the union of both commits' lists (so dropping a path
    from the list does not unprotect it). Renames show as a deletion plus an addition."""
    patterns = sorted(set(read_patterns(repo, base)) | set(read_patterns(repo, cand)))
    raw = git(repo, "diff-tree", "-r", "-z", "--no-renames", "--raw", "--no-abbrev", base, cand)
    toks = raw.split("\0")
    changes = []
    i = 0
    while i < len(toks) and toks[i]:
        header, path = toks[i], toks[i + 1]
        i += 2
        fields = header.lstrip(":").split()
        new_mode, new_blob, status = fields[1], fields[3], fields[4]
        if new_mode == "160000" or not protected(path, patterns):
            continue
        changes.append((status[0], path, "deleted" if status[0] == "D" else new_blob))
    return patterns, changes


def golden_check(repo, base, cand):
    """(changes, unapproved, approved_by) for the protected changes BASE..CAND."""
    patterns, changes = golden_changes(repo, base, cand)
    ok = approved_blobs(repo) if changes else {}
    unapproved, by = [], {}
    for status, path, blob in changes:
        hits = ok.get((path, blob))
        if hits:
            by[path] = hits
        else:
            unapproved.append((status, path, blob))
    return patterns, changes, unapproved, by
