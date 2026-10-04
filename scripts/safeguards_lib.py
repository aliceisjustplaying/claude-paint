"""Shared code for scripts/test_candidate, scripts/golden_approve and
scripts/merge_candidate (Python standard library only).

See notes/speed/SAFEGUARDS.md for how the three tools fit together.
"""

import base64
import datetime
import hashlib
import json
import math
import os
import re
import subprocess

RECEIPTS_REF = "refs/notes/test-receipts"
APPROVALS_REF = "refs/notes/golden-approvals"
GOLDEN_LIST = "notes/golden_paths.txt"
# the reviewed check set: the list scripts/test --all runs, and the runner itself
DEFAULT_LIST = "notes/speed/test_lists/all.tsv"
RUNNER = "scripts/test"
RECEIPT_FORMAT = "claude-paint test receipt v2"
APPROVAL_FORMAT = "claude-paint golden approval v1"
HEX_ID = re.compile(r"[0-9a-f]{7,64}")
HEX64 = re.compile(r"[0-9a-f]{64}")
REGULAR_MODES = ("100644", "100755")
STEP_KINDS = ("build", "cargo", "pytest", "script")
# what a summary step must repeat from its list line, exactly
STEP_KEYS = ("name", "kind", "command", "limit", "least", "group")
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


def regular_file_bytes(repo, commit, path):
    """Contents of PATH at COMMIT if it is a regular file there; Refuse otherwise."""
    e = entry_at(repo, commit, path)
    if e is None:
        raise Refuse("%s is not in %s" % (path, commit[:12]))
    if e[0] not in REGULAR_MODES:
        raise Refuse("%s at %s is not a regular file (mode %s)" % (path, commit[:12], e[0]))
    return blob_bytes(repo, commit, path)


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

def parse_test_list(data, where):
    """The steps of a scripts/test list file (bytes). Tab-separated lines: kind, name,
    limit (s), least (minimum tests), regex ('-' for none), command, and an optional
    group ('' or '-' for none). Blank lines and lines starting with '#' are skipped.
    Refuse on anything malformed: a list that can't be read can't be bound."""
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        raise Refuse("%s is not UTF-8" % where)
    steps = []
    for n, line in enumerate(text.replace("\r\n", "\n").replace("\r", "\n").split("\n"), 1):
        if not line.strip() or line.startswith("#"):
            continue
        f = line.split("\t")
        if len(f) not in (6, 7):
            raise Refuse("%s line %d: want 6 or 7 tab-separated fields, found %d" % (where, n, len(f)))
        kind, name, limit, least, regex, cmd = f[:6]
        group = f[6] if len(f) == 7 else ""
        if kind not in STEP_KINDS:
            raise Refuse("%s line %d: unknown kind %r" % (where, n, kind))
        if not name or not cmd.strip():
            raise Refuse("%s line %d: empty name or command" % (where, n))
        try:
            lim = float(limit)
        except ValueError:
            lim = float("nan")
        if not (math.isfinite(lim) and lim > 0):
            raise Refuse("%s line %d: bad time limit %r" % (where, n, limit))
        if not least.isdigit():
            raise Refuse("%s line %d: bad minimum test count %r" % (where, n, least))
        steps.append({"kind": kind, "name": name, "limit": lim, "least": int(least),
                      "regex": None if regex == "-" else regex, "command": cmd,
                      "group": None if group in ("", "-") else group})
    if not steps:
        raise Refuse("%s lists no steps" % where)
    names = [s["name"] for s in steps]
    dup = sorted({x for x in names if names.count(x) > 1})
    if dup:
        raise Refuse("%s names step(s) %s more than once" % (where, ", ".join(dup)))
    return steps


def manifest_problems(steps):
    """Why a parsed list can't pass even if every step does ([] if it can)."""
    p = ["step %s (kind %s) requires %d tests: every non-build step must require at least 1"
         % (s["name"], s["kind"], s["least"]) for s in steps if s["kind"] != "build" and s["least"] < 1]
    if not any(s["kind"] != "build" for s in steps):
        p.append("the list has only build steps: no tests would run")
    return p


def manifest_at(repo, commit, list_path=DEFAULT_LIST):
    """The reviewed check set at COMMIT: its list file parsed, and the hashes of the
    list and of the runner (scripts/test) as committed. Refuse if either is missing,
    not a regular file or (the list) malformed."""
    data = regular_file_bytes(repo, commit, list_path)
    runner = regular_file_bytes(repo, commit, RUNNER)
    return {"list": list_path, "list_sha256": sha256_bytes(data),
            "runner": RUNNER, "runner_sha256": sha256_bytes(runner),
            "steps": parse_test_list(data, "%s at %s" % (list_path, commit[:12]))}


def _int(x):
    return type(x) is int


def _num(x):
    return type(x) in (int, float)


def _same(key, got, want):
    if key == "limit":
        return _num(got) and float(got) == want
    if key == "least":
        return _int(got) and got == want
    if key == "group":
        return (got is None and want is None) or (isinstance(got, str) and got == want)
    return isinstance(got, str) and got == want


def binding_problems(manifest_steps, steps):
    """Why the summary's steps are not exactly the list's steps ([] if they are):
    same steps, same order, same name, kind, command, limit, least and group."""
    p = []
    got = [s.get("name") if isinstance(s, dict) else None for s in steps]
    want = [m["name"] for m in manifest_steps]
    missing = [n for n in want if n not in got]
    extra = [n for n in got if n not in want]
    dup = sorted({str(n) for n in got if got.count(n) > 1})
    if missing:
        p.append("the summary omits listed step(s): %s" % ", ".join(missing))
    if extra:
        p.append("the summary has step(s) the list does not: %s" % ", ".join(map(str, extra)))
    if dup:
        p.append("the summary reports step(s) more than once: %s" % ", ".join(dup))
    if not missing and not extra and not dup and got != want:
        p.append("the summary's step order (%s) is not the list's (%s)" % (", ".join(got), ", ".join(want)))
    by_name = {}
    for s in steps:
        if isinstance(s, dict) and s.get("name") not in by_name:
            by_name[s.get("name")] = s
    for m in manifest_steps:
        s = by_name.get(m["name"])
        if s is None:
            continue
        for k in STEP_KEYS[1:]:
            if not _same(k, s.get(k), m[k]):
                p.append("step %s: the summary's %s is %r, the list's %r" % (m["name"], k, s.get(k), m[k]))
    return p


def summary_problems(summary, manifest=None):
    """Reasons a scripts/test summary is not a full pass ([] means pass). With
    MANIFEST (manifest_at), the summary must also be a run of exactly that check set."""
    if not isinstance(summary, dict):
        return ["the summary is missing or not a JSON object"]
    p = []
    if summary.get("mode") != "all":
        p.append("summary mode is %r, not 'all'" % summary.get("mode"))
    if summary.get("verdict") != "pass":
        p.append("summary verdict is %r, not 'pass'" % summary.get("verdict"))
    for key in ("list_sha256", "runner_sha256"):
        if not HEX64.fullmatch(str(summary.get(key, ""))):
            p.append("summary has no valid %s" % key)
    if manifest is not None:
        if summary.get("list") != manifest["list"]:
            p.append("the summary ran list %r, not %s" % (summary.get("list"), manifest["list"]))
        if summary.get("list_sha256") != manifest["list_sha256"]:
            p.append("the summary's list_sha256 is not the hash of %s in the commit" % manifest["list"])
        if summary.get("runner_sha256") != manifest["runner_sha256"]:
            p.append("the summary's runner_sha256 is not the hash of %s in the commit" % manifest["runner"])
    steps = summary.get("steps")
    if not isinstance(steps, list) or not steps:
        p.append("summary has no steps: an empty test selection never passes")
        return p
    if manifest is not None:
        p += binding_problems(manifest["steps"], steps)
    least_of = {m["name"]: m["least"] for m in (manifest or {}).get("steps", [])}
    for i, s in enumerate(steps):
        if not isinstance(s, dict):
            p.append("step %d is not an object" % i)
            continue
        name = s.get("name") or "step %d" % i
        if s.get("exit") != 0 or not _int(s.get("exit")):
            p.append("step %s exited %r" % (name, s.get("exit")))
        if s.get("timed_out") is not False:
            p.append("step %s timed out (timed_out=%r)" % (name, s.get("timed_out")))
        if s.get("failed") != 0 or not _int(s.get("failed")):
            p.append("step %s has failures (failed=%r)" % (name, s.get("failed")))
        if s.get("ok") is not True:
            p.append("step %s is not ok (ok=%r, why=%r)" % (name, s.get("ok"), s.get("why")))
        # a build step (scripts/test kind "build") has no tests; every other step must run some
        if s.get("kind") == "build":
            continue
        least = s.get("least")
        if not _int(least) or least < 1:
            p.append("step %s requires %r tests: a non-build step must require at least 1" % (name, least))
        if not _int(s.get("tests_run")) or s["tests_run"] < 1:
            p.append("step %s ran no tests (tests_run=%r)" % (name, s.get("tests_run")))
        elif s["tests_run"] < max(least_of.get(name, 1), least if _int(least) else 1):
            p.append("step %s ran %d tests, fewer than its minimum %r"
                     % (name, s["tests_run"], least_of.get(name, least)))
        if not _int(s.get("passed")) or s["passed"] < 1:
            p.append("step %s passed no tests (passed=%r)" % (name, s.get("passed")))
    if not any(isinstance(s, dict) and s.get("kind") != "build" for s in steps):
        p.append("summary has only build steps: no tests ran")
    return p


# ---------------------------------------------------------------- checkout evidence

def checkout_state(wt, commit):
    """Evidence about the working copy WT of COMMIT: identity (HEAD, tree), whether
    the index is exactly the commit's tree, sparse checkout, skip-worktree or
    assume-unchanged flags, tracked changes and untracked files."""
    s = {"head": git(wt, "rev-parse", "HEAD"), "tree": git(wt, "rev-parse", "HEAD^{tree}")}
    s["sparse"] = git(wt, "config", "--bool", "core.sparseCheckout", check=False) == "true"
    flagged = [l for l in git(wt, "ls-files", "-v").splitlines() if l[:1] == "S" or l[:1].islower()]
    s["flagged"], s["flagged_count"] = flagged[:20], len(flagged)
    index = set()
    for rec in git(wt, "ls-files", "-s", "-z").split("\0"):
        if rec:
            meta, path = rec.split("\t", 1)
            mode, obj, stage = meta.split()
            index.add((mode, obj, stage, path))
    tree = set()
    for rec in git(wt, "ls-tree", "-r", "-z", "--full-tree", commit).split("\0"):
        if rec:
            meta, path = rec.split("\t", 1)
            mode, _type, obj = meta.split()
            tree.add((mode, obj, "0", path))
    s["files_index"], s["files_tree"] = len(index), len(tree)
    s["index_matches_commit"] = index == tree
    status = git(wt, "status", "--porcelain=v1", "--untracked-files=no", "--ignore-submodules=none")
    s["tracked_changes"] = status.splitlines()[:20]
    s["tracked_changed"] = bool(status)
    untracked = git(wt, "status", "--porcelain=v1", "--untracked-files=all").splitlines()
    s["untracked"] = [l for l in untracked if l.startswith("??")][:20]
    return s


def checkout_problems(state, commit, tree, when):
    """Why STATE (checkout_state) is not a full, unchanged checkout of COMMIT ([] if it is)."""
    if not isinstance(state, dict):
        return ["no checkout evidence %s" % when]
    p = []
    if state.get("head") != commit:
        p.append("HEAD %s was %s, not the candidate %s" % (when, state.get("head"), commit))
    if state.get("tree") != tree:
        p.append("the tree %s was %s, not the candidate's %s" % (when, state.get("tree"), tree))
    if state.get("sparse") is not False:
        p.append("the working copy was a sparse checkout %s (sparse=%r)" % (when, state.get("sparse")))
    if state.get("flagged_count") != 0:
        p.append("%r index entries had skip-worktree/assume-unchanged flags %s: %s"
                 % (state.get("flagged_count"), when, state.get("flagged")))
    if not _int(state.get("files_index")) or state.get("files_index") != state.get("files_tree"):
        p.append("the index listed %r files %s, the commit has %r"
                 % (state.get("files_index"), when, state.get("files_tree")))
    if state.get("index_matches_commit") is not True:
        p.append("the index was not the commit's tree %s" % when)
    if state.get("tracked_changed") is not False:
        p.append("tracked files or the index changed %s: %s" % (when, state.get("tracked_changes")))
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
