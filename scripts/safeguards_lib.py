"""scripts/golden_approve's code (Python standard library only)."""

import base64
import datetime
import hashlib
import json
import os
import re
import subprocess

APPROVALS_REF = "refs/notes/golden-approvals"
GOLDEN_LIST = "notes/golden_paths.txt"
APPROVAL_FORMAT = "claude-paint golden approval v1"
HEX_ID = re.compile(r"[0-9a-f]{7,64}")
REGULAR_MODES = ("100644", "100755")
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


def sha256_bytes(b):
    return hashlib.sha256(b).hexdigest()


def blob_bytes(repo, commit, path):
    """Contents of PATH at COMMIT, or None if absent."""
    p = run(["git", "-C", repo, "cat-file", "blob", "%s:%s" % (commit, path)], check=False, text=False)
    return p.stdout if p.returncode == 0 else None


def entry_at(repo, commit, path):
    """(mode, object id) of PATH's tree entry at COMMIT, or None if absent."""
    out = git(repo, "ls-tree", "-z", "--full-tree", commit, "--", path)
    for rec in out.split("\0"):
        if not rec:
            continue
        meta, name = rec.split("\t", 1)
        if name == path:
            mode, _type, obj = meta.split()
            return mode, obj
    return None


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


# ---------------------------------------------------------------- the reviewed check set


# ---------------------------------------------------------------- checkout evidence


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
    """PATH is protected if it equals an entry, lies under an entry ending in /, or
    lies under an entry (a file that became a directory). The list file itself is
    always protected. Entries need not exist."""
    if path == GOLDEN_LIST:
        return True
    for pat in patterns:
        if pat.endswith("/") and path.startswith(pat):
            return True
        if path == pat or path.startswith(pat + "/"):
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
    notes = [line.split() for line in git(repo, "notes", "--ref=golden-approvals", "list").splitlines()]
    types = git(repo, "cat-file", "--batch-check=%(objecttype)",
                input="".join(obj + "\n" for _, obj in notes)).splitlines()
    for (note, obj), kind in zip(notes, types):
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
            if kind != "commit":
                a["invalid"] = "not an available commit"
            out.append(a)
    return out


def approved_entries(repo):
    """{(path, mode, object): [approval, ...]} from approvals that are valid: the
    note sits on the commit it names, approver != builder (case-insensitive), and
    each listed entry really is that path's entry at the commit. An entry is
    {"path", "blob", "mode"}: blob is the object id ("deleted" if absent). An entry
    without mode (older approvals) means the regular file's mode at the commit; a
    symlink, submodule (gitlink) or other type is approved only by an entry naming
    its exact mode."""
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
            if not isinstance(e, dict):
                continue
            path, obj, mode = e.get("path"), e.get("blob"), e.get("mode")
            if not path or not obj:
                continue
            actual = entry_at(repo, a["commit"], path)
            if obj == "deleted":
                if actual is None:
                    ok.setdefault((path, None, "deleted"), []).append(a)
                continue
            if actual is None or actual[1] != obj:
                continue
            if mode is None and actual[0] in REGULAR_MODES:
                mode = actual[0]
            if mode == actual[0]:
                ok.setdefault((path, mode, obj), []).append(a)
    return ok


def golden_changes(repo, base, cand):
    """Protected entries that differ between BASE and CAND: [(status, path, object, mode)].
    object is "deleted" (mode None) for a deletion. Every entry type counts, also
    submodule (gitlink 160000) and symlink (120000) entries. The protected set is the
    union of both commits' lists (so dropping a path from the list does not unprotect
    it); listed paths need not exist. Renames show as a deletion plus an addition."""
    patterns = sorted(set(read_patterns(repo, base)) | set(read_patterns(repo, cand)))
    raw = git(repo, "diff-tree", "-r", "-z", "--no-renames", "--raw", "--no-abbrev", base, cand)
    toks = raw.split("\0")
    changes = []
    i = 0
    while i < len(toks) and toks[i]:
        header, path = toks[i], toks[i + 1]
        i += 2
        fields = header.lstrip(":").split()
        new_mode, new_obj, status = fields[1], fields[3], fields[4]
        if not protected(path, patterns):
            continue
        if status[0] == "D":
            changes.append(("D", path, "deleted", None))
        else:
            changes.append((status[0], path, new_obj, new_mode))
    return patterns, changes


def unsupported(mode):
    return mode is not None and mode not in REGULAR_MODES


def golden_check(repo, base, cand):
    """(patterns, changes, unapproved, approved_by) for the protected changes BASE..CAND."""
    patterns, changes = golden_changes(repo, base, cand)
    ok = approved_entries(repo) if changes else {}
    unapproved, by = [], {}
    for status, path, obj, mode in changes:
        hits = ok.get((path, mode, obj))
        if hits:
            by[path] = hits
        else:
            unapproved.append((status, path, obj, mode))
    return patterns, changes, unapproved, by


def describe_change(status, path, obj, mode):
    if obj == "deleted":
        return "%s %s (deleted)" % (status, path)
    s = "%s %s (mode %s, object %s)" % (status, path, mode, obj[:12])
    if unsupported(mode):
        s += " [unsupported type %s: refused unless an approval names mode %s and object %s]" % (
            {"160000": "submodule", "120000": "symlink"}.get(mode, "mode " + mode), mode, obj)
    return s
