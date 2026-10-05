"""Tests of the round-24 runner (r21_chains.py, round 21's runner with the changes of rounds 22 to 24).

    uv run --no-project --with pytest --with pillow pytest -q -p no:cacheprovider notes/round22/runner/
"""
import json
import subprocess

import pytest

import r21_chains as rc21

GO_LIMIT = '429: {"type":"GoUsageLimitError","message":"Go usage limit exceeded"}'      # round 19, KIMIF and MIMOF


@pytest.mark.parametrize("views, finished", [
    ([({}, False)], False),  # the first engine-3 painter's second sitting
    ([({}, False), ({"crop": "0,0,300,300"}, False)], True),
    ([({}, False), ({"crop": "0,0,600,600"}, True)], False),
    ([({"palette": True}, False), ({"crop": "0,0,300,300"}, False)], False),
    ([({"mode": "value"}, False), ({"crop": "0,0,300,300"}, False)], False),
])
def test_completion_requires_successful_whole_and_detail_review(tmp_path, views, finished):
    session = tmp_path / "session.jsonl"
    messages = []
    for i, (args, failed) in enumerate(views):
        messages += [
            {"role": "assistant", "content": [{"type": "toolCall", "id": str(i),
              "name": "look", "arguments": args}]},
            {"role": "toolResult", "toolCallId": str(i), "toolName": "look",
             "isError": failed, "content": [{"type": "text", "text": "crop too large"}]
             if failed else [{"type": "image", "data": "fixture", "mimeType": "image/png"}]},
        ]
    session.write_text("\n".join(json.dumps({"message": m}) for m in messages))
    sitting = dict(sitting=2, status="completed", painting_before=99, painting_after=99,
                   reviewed=rc21.session_reviewed([session]))
    step, reason = rc21.next_step([sitting])
    assert (step is None) is finished
    if not finished:
        assert (step, reason) == ("new", 3)


@pytest.mark.parametrize("painted_sittings", [1, 5])
def test_sittings_continue_until_reviewed_finish_despite_old_cap_file(tmp_path, monkeypatch, painted_sittings):
    monkeypatch.setattr(rc21, "RUN", tmp_path)
    monkeypatch.setattr(rc21, "LANES", {"T": rc21.lane("inness", rc21.OPUS)})
    messages = []
    monkeypatch.setattr(rc21, "log", lambda *a: None)
    monkeypatch.setattr(rc21, "stop_leftovers", lambda *a, **kw: None)
    monkeypatch.setattr(rc21, "open_easel", lambda *a: True)
    monkeypatch.setattr(rc21, "close_easel", lambda *a: None)
    monkeypatch.setattr(rc21, "session_dir", lambda *a: tmp_path)
    monkeypatch.setattr(rc21, "count_chunks", lambda *a: len(messages))
    monkeypatch.setattr(rc21, "count_painting", lambda *a: min(len(messages), painted_sittings))
    monkeypatch.setattr(rc21, "session_reviewed", lambda *a: True)
    (tmp_path / "max_sittings.txt").write_text("1")

    def painter(cmd, cwd, out, err, env=None):
        messages.append(cmd[-1])
        assert env.get("PAINTER_SITTING_RECOVERY") == ("1" if len(messages) > 1 else None)
        out.write_text("Unresolved: oak base. Lesson: the rectangular blend dragged dark paint into sky.")
        err.write_text("")
        return 0

    monkeypatch.setattr(rc21, "run", painter)
    rc21.paint("T", 1, tmp_path / "studio", tmp_path)
    records = json.loads((tmp_path / "p1_sittings.json").read_text())
    assert len(records) == painted_sittings + 1
    assert messages == [rc21.PAINTER_MSG] + [rc21.SITTING_MESSAGE +
        "\n\nYour previous sitting\x27s reply:\nUnresolved: oak base. Lesson: the rectangular blend dragged dark paint into sky."] * painted_sittings
    assert json.loads((tmp_path / "p1_outcome.json").read_text()) == {
        "status": "finished",
        "reason": f"the painter is done: sitting {painted_sittings + 1} reviewed whole and detail views and added no painting",
    }


def probe_waits(monkeypatch, tmp_path, *replies):
    """wait_out_limit against probe replies in turn (a CompletedProcess or an exception to raise):
    what it returned, and the log."""
    lines, replies = [], list(replies)

    def probe(cmd, **kw):
        r = replies.pop(0)
        if isinstance(r, Exception):
            raise r
        return subprocess.CompletedProcess(cmd, r[0], r[1], r[2])
    monkeypatch.setattr(rc21, "RUN", tmp_path)
    monkeypatch.setattr(rc21, "probe_cmd", lambda m: ["probe"])
    monkeypatch.setattr(rc21, "log", lines.append)
    clock = [1e9]                                                # sleeping moves the clock, nothing else
    monkeypatch.setattr(rc21.time, "time", lambda: clock[0])
    monkeypatch.setattr(rc21.time, "sleep", lambda s: clock.__setitem__(0, clock[0] + s))
    monkeypatch.setattr(rc21.subprocess, "run", probe)
    # the window holds exactly as many probes as there are replies (one every LIMIT_PROBE_S)
    monkeypatch.setattr(rc21, "LIMIT_GIVE_UP_H", (len(replies) + 0.5) * rc21.LIMIT_PROBE_S / 3600)
    return rc21.wait_out_limit({}, "T", GO_LIMIT, ended=clock[0]), lines


@pytest.mark.parametrize("reply, why", [
    (subprocess.TimeoutExpired("probe", 600), "timed out"),
    ((1, "", "Error: connection refused"), "exited 1"),
    # outages and transient errors, and anything unsure, are asked again
    ((1, "", "Error: 529 {\"type\":\"error\",\"error\":{\"type\":\"overloaded_error\",\"message\":\"Overloaded\"}}"),
     "exited 1"),
    ((1, "", "HTTP 503: Service Unavailable"), "exited 1"),
    ((1, "", "Error: fetch failed: ECONNRESET"), "exited 1"),
    ((1, "", '429: {"error":{"code":429,"message":"You exceeded your current quota, please check your plan and '
             'billing details.","status":"RESOURCE_EXHAUSTED"}}'), "exited 1"),
    ((1, "", "Error: something went wrong"), "exited 1"),
    # OpenCode Zen, real: a provider's server error is an outage, not a malformed request
    ((1, "", '400: {"type":"server_error","message":"Error from provider (Console): Upstream request failed: '
             'Model is unavailable."}'), "exited 1"),
    ((0, "", ""), "no reply"),
    ((0, "I can't help with that.\n", ""), "unexpected reply"),
])
def test_a_probe_that_fails_is_not_the_provider_answering(monkeypatch, tmp_path, reply, why):
    # the provider must answer the probe ("ok", exit 0); a timeout or an error is no recovery
    ok, lines = probe_waits(monkeypatch, tmp_path, reply, (0, "ok\n", ""))
    assert ok is True
    assert any(why in l and "attempt 1" in l for l in lines), lines
    assert any("answers again" in l and "attempt 2" in l for l in lines), lines
    # failing every time, the painter stops (never goes on) once the window is over
    ok, lines = probe_waits(monkeypatch, tmp_path, reply, reply)
    assert ok is False
    assert not any("answers again" in l for l in lines), lines


@pytest.mark.parametrize("reply", ["ok\n", "Ok.\n", "OK\n", "Okay.\n", "okay\n", "OKAY!\n"])
def test_ok_or_okay_in_any_case_is_the_provider_answering(reply):
    from r21_chains import probe_outcome
    assert probe_outcome(subprocess.CompletedProcess(["probe"], 0, reply, ""))[0] == "available"
    assert probe_outcome(subprocess.CompletedProcess(["probe"], 0, "okapi\n", ""))[0] == "unavailable"


GOOGLE_402 = ('HTTP 402: {"error":{"message":"{\\n  \\"error\\": {\\n    \\"code\\": 402,\\n    \\"message\\": '
              '\\"Your prepayment credits are depleted. Please go to AI Studio at https://ai.studio/projects to manage '
              'your project and billing.\\",\\n    \\"status\\": \\"RESOURCE_EXHAUSTED\\"\\n  }\\n}\\n",'
              '"code":402,"status":"Payment Required"}}')


@pytest.mark.parametrize("reply, why", [
    ((1, "", '401 {"error":"invalid api key"}'), "API key"),
    ((1, "", 'Error: 401 {"type":"error","error":{"type":"authentication_error","message":"invalid x-api-key"}}'),
     "API key"),
    ((1, "", "No API key found for opencode-go."), "API key"),
    ((1, "", 'Error: 403 {"error":{"type":"permission_error","message":"not allowed"}}'), "API key"),
    ((1, "", GOOGLE_402), "no credit"),
    ((1, "", 'Error: 400 {"type":"error","error":{"type":"invalid_request_error","message":"Your credit balance '
             'is too low to access the Anthropic API."}}'), "no credit"),
    ((1, "", 'Model "gpt-9" not found. Use --list-models to see available models.'), "model"),
    ((1, "", 'Error: 404 {"error":{"type":"not_found_error","message":"model: claude-x"}}'), "model"),
    ((1, "", '400: {"type":"invalid_request_error","message":"Upstream request failed: [invalid_request_error] '
             'native reasoning control reasoning_effort is not allowed"}'), "malformed"),
    # real errors from pi session logs (review E, L2)
    ((1, "", 'opencode API error (403): {"type":"FreeTierError","message":"free tier not available"}'), "API key"),
    ((1, "", "Provided authentication token is expired."), "API key"),
    ((1, "", "Codex error: The 'gpt-5.4' model is not supported when using Codex with a ChatGPT account."), "model"),
    ((1, "", 'Error: 429 {"type":"error","error":{"type":"rate_limit_error","error_code":"credits_required",'
             '"message":"out_of_credits"}}'), "no credit"),
])
def test_a_probe_error_waiting_wont_fix_stops_the_painter_at_once(monkeypatch, tmp_path, reply, why):
    # a refused key, no credit, an unknown model or a malformed request: stop now, say why and to rerun
    ok, lines = probe_waits(monkeypatch, tmp_path, reply, (0, "ok\n", ""))
    assert ok is False
    assert any("painter stops now" in l and "attempt 1" in l and why in l and "rerun after fixing" in l
               for l in lines), lines
    assert not any("answers again" in l or "attempt 2" in l for l in lines), lines


def test_a_probe_that_answers_ends_the_wait_and_a_limit_keeps_it_going(monkeypatch, tmp_path):
    ok, lines = probe_waits(monkeypatch, tmp_path, (0, "Ok.\n", ""))
    assert ok is True and sum("answers again" in l for l in lines) == 1
    ok, lines = probe_waits(monkeypatch, tmp_path, (1, "", GO_LIMIT), (0, "ok\n", ""))
    assert ok is True and any("still limited" in l and "attempt 1" in l for l in lines), lines


def test_a_probe_that_answers_ok_is_available_whatever_its_stderr_says():
    # a warning on stderr that reads like a fatal error doesn't undo a reply of ok
    r = subprocess.CompletedProcess(["probe"], 0, "ok\n", "warning: set up billing to keep access after the trial\n")
    assert rc21.probe_outcome(r)[0] == "available"


def test_the_reader_is_launched_as_isolated_as_the_painter():
    cmd = rc21.reader_cmd("Read the brief.")
    assert [f for f in rc21.HARNESS if f.startswith("--no-")] == [f for f in cmd if f.startswith("--no-")]
    assert cmd[cmd.index("--tools") + 1] == "read,write"          # reader.ts checks both (READER_SCOPE)
    assert cmd[cmd.index("--system-prompt") + 1] == str(rc21.HERE / "reader_system_prompt.md")
    exts = [cmd[i + 1] for i, f in enumerate(cmd) if f == "-e"]
    assert exts == [str(rc21.HERE / "reader.ts")] + ([str(rc21.BLACK)] if rc21.READER_BLACK else [])


REPO_NAMES = rc21.HERE.parents[2] / "scripts/check_studio_names"
GOOD_RECORD = "## Blending\n- The badger only moves wet paint; `blend(m, {clip=true})` stayed inside.\n"


def chain_one_record(tmp_path, monkeypatch, record, rc, journal=True, lanes=None, no_logs=False):
    """Run lane T's chain (two painters) with painter 1 painted and a reader that writes record (unless
    None) and exits rc: the log lines and the READER_SCOPE the reader got."""
    lines, scopes = [], []
    monkeypatch.setattr(rc21, "RUN", tmp_path / "run")
    monkeypatch.setattr(rc21, "A", tmp_path)
    monkeypatch.setattr(rc21, "LANES", lanes or {"T": rc21.lane("sargent", rc21.OPUS, painters=2)})
    monkeypatch.setattr(rc21, "log", lines.append)
    monkeypatch.setattr(rc21, "check", lambda *a: None)
    monkeypatch.setattr(rc21, "finish", lambda *a: None)
    monkeypatch.setattr(rc21, "export_cmd", lambda profile, d: ["mkdir", "-p", str(d / "notes/research")])
    monkeypatch.setattr(rc21, "NAMES", REPO_NAMES)              # the round-21 checkout's copy at run time
    rd = tmp_path / "run/T"

    def reader(cmd, cwd, out, err, env=None):                  # the reader: writes the record, or doesn't
        scopes.append(json.loads(env["READER_SCOPE"]))
        if record is not None:
            (rd / "p1_record.md").write_text(record)
        return rc
    monkeypatch.setattr(rc21, "run", reader)
    rd.mkdir(parents=True, exist_ok=True)
    session = tmp_path / "s1.jsonl"
    if not no_logs:
        session.write_text("{}\n")
    (rd / "p1_sittings.json").write_text(json.dumps([{"sitting": 1, "sessions": [str(session)]}]))
    for marker in ("p1.exported", "p1.painted", "p2.painted"):   # painter 1 has painted; painter 2 won't paint
        (rd / marker).write_text("")
    (rc21.studio("T1") / "notes").mkdir(parents=True)
    if journal:
        (rc21.studio("T1") / "notes/journal.md").write_text("- day one\n")
    rc21.chain("T")
    return lines, scopes


@pytest.mark.parametrize("journal", [True, False])
def test_the_reader_may_read_the_logs_journal_and_brief_and_write_only_its_record(tmp_path, monkeypatch, journal):
    lines, scopes = chain_one_record(tmp_path, monkeypatch, GOOD_RECORD, 0, journal=journal)
    rd = tmp_path / "run/T"
    j = [str(rc21.studio("T1") / "notes/journal.md")] if journal else []   # a journal never written isn't one
    assert scopes == [{"read": [str(rd / "p1_reader_logs/log1.jsonl"), *j, str(rd / "p1_reader_brief.md")],
                       "write": str(rd / "p1_record.md")}]
    assert GOOD_RECORD in (rc21.studio("T2") / "notes/studio_notes.md").read_text()
    assert any(l.endswith("record written") for l in lines), lines


def test_a_reader_that_fails_says_so_and_the_lane_stops(tmp_path, monkeypatch):
    # pi exits 1 when reader.ts refuses to load (no valid READER_SCOPE): the log names the exit
    lines, _ = chain_one_record(tmp_path, monkeypatch, None, 1)
    rd = tmp_path / "run/T"
    assert any("RECORD REJECTED: the reader exited 1; no record was written" in l for l in lines), lines
    assert not (rd / "p1.done").exists() and not (rc21.studio("T2") / "notes/studio_notes.md").exists()


# a record that fails round 19's hard checks isn't passed on: it's set aside and the lane stops (a
# rerun reads again); the pattern flags above are only warnings
@pytest.mark.parametrize("record, why", [
    ("", "the record is empty"),
    ("  \n\n", "the record is empty"),
    ("".join(f"- The paint did thing {i}.\n" for i in range(61)), "61 lines (at most 60)"),
    (GOOD_RECORD + "```lua\nb:stroke(p)\n```\n", "a code block"),
    (GOOD_RECORD + "local b = brush{size=20}\n", "code on line 3"),
    (GOOD_RECORD + '- The warm dark was pile{{"burnt sienna", 2}, {"bone black", 1}}.\n', "a color recipe on line 3"),
    (GOOD_RECORD + "- The earlier painter left the sky wet.\n", "another painter (painter)"),
    (GOOD_RECORD + "- DeepSeek laid the ground thin.\n", "another painter (deepseek)"),
    (GOOD_RECORD + "- The glaze behaved as in Friedrich's skies.\n", "painters' names"),
])
def test_a_record_that_fails_the_hard_checks_is_set_aside_and_the_lane_stops(tmp_path, monkeypatch, record, why):
    lines, _ = chain_one_record(tmp_path, monkeypatch, record, 0)
    rd = tmp_path / "run/T"
    assert any("RECORD REJECTED" in l and why in l for l in lines), lines
    assert (rd / "p1_record.rejected.md").read_text() == record
    assert not (rd / "p1_record.md").exists() and not (rd / "p1.done").exists()
    assert not (rc21.studio("T2") / "notes/studio_notes.md").exists()


def test_the_studio_s_own_artist_may_be_named(tmp_path, monkeypatch):
    record = GOOD_RECORD + "- A thin gray ground stayed visible, as in Sargent's lay-ins.\n"
    lines, _ = chain_one_record(tmp_path, monkeypatch, record, 0)
    assert not any("REJECTED" in l for l in lines), lines
    assert record in (rc21.studio("T2") / "notes/studio_notes.md").read_text()


def test_an_earlier_attempt_s_record_is_not_passed_on(tmp_path, monkeypatch):
    rd = tmp_path / "run/T"
    rd.mkdir(parents=True)
    (rd / "p1_record.md").write_text(GOOD_RECORD)               # a record from an earlier attempt
    lines, _ = chain_one_record(tmp_path, monkeypatch, None, 0)  # this reader writes nothing
    assert any("RECORD REJECTED: no record was written" in l for l in lines), lines
    assert not (rc21.studio("T2") / "notes/studio_notes.md").exists()


def test_a_painter_without_session_logs_stops_the_lane(tmp_path, monkeypatch):
    lines, scopes = chain_one_record(tmp_path, monkeypatch, GOOD_RECORD, 0, no_logs=True)
    assert scopes == []                                         # no reader ran
    assert any("RECORD MISSING: no session logs to read" in l for l in lines), lines
    assert not (tmp_path / "run/T/p1.done").exists()


# what to do and where things go in the picture are flagged, not rejected: the record goes on and
# the flagged lines are logged and listed in p1_record.flags.md
@pytest.mark.parametrize("record, flagged", [
    ("Always put a tall arch at the center of the picture. Keep the brightest patch on the left.\n",
     ["line 1 (what to do)", "line 1 (where things go in the picture)"]),
    (GOOD_RECORD + "- Glazing: never glaze before the underlayer is dry.\n", ["line 3 (what to do)"]),
    (GOOD_RECORD + "- Stippling: the dark mass sits in the lower third of the picture.\n",
     ["line 3 (where things go in the picture)"]),
    (GOOD_RECORD + "- Masks: the horizon line stayed crisp under the mask.\n",
     ["line 3 (where things go in the picture)"]),
])
def test_a_record_with_flagged_lines_still_reaches_the_next_studio(tmp_path, monkeypatch, record, flagged):
    lines, _ = chain_one_record(tmp_path, monkeypatch, record, 0)
    rd = tmp_path / "run/T"
    assert (rd / "p1.done").exists()
    assert record in (rc21.studio("T2") / "notes/studio_notes.md").read_text()
    flags = (rd / "p1_record.flags.md").read_text()
    for f in flagged:
        assert f in flags
        assert any("RECORD FLAGGED" in l and f in l for l in lines), lines
    assert any("record written" in l and "flagged lines went on with it" in l for l in lines), lines


def test_observations_of_what_the_paint_did_are_not_flagged(tmp_path, monkeypatch):
    record = (GOOD_RECORD + "- The knife left paint only on the top of the weave; a denser pass covered it.\n"
              "- A glaze over a dry passage stayed transparent; over a wet one it mixed in.\n")
    rd = tmp_path / "run/T"
    rd.mkdir(parents=True, exist_ok=True)
    (rd / "p1_record.flags.md").write_text("an earlier attempt's flags\n")
    lines, _ = chain_one_record(tmp_path, monkeypatch, record, 0)
    assert not (rd / "p1_record.flags.md").exists()
    assert not any("FLAGGED" in l for l in lines), lines
    assert record in (rc21.studio("T2") / "notes/studio_notes.md").read_text()


# structured records (record_kind="structured"): the reader writes observations, the runner checks
# them against the logs and renders them into the next studio's notes

def imported_runner():
    """r21_chains.py as it was imported (fixtures/r21_chains_as_imported.py, verbatim), as a module."""
    import importlib.util
    spec = importlib.util.spec_from_file_location("r21_chains_as_imported", rc21.HERE / "fixtures/r21_chains_as_imported.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def free_text_notes(mod, tmp_path, monkeypatch, record, other_lane=None):
    """The notes painter 2 of a free-text lane T gets from runner module mod, painter 1's reader
    writing record (and another lane's record merged first, if given)."""
    monkeypatch.setattr(mod, "RUN", tmp_path / "run")
    monkeypatch.setattr(mod, "A", tmp_path)
    monkeypatch.setattr(mod, "HERE", rc21.HERE)
    monkeypatch.setattr(mod, "LANES", {"T": mod.lane("sargent", mod.OPUS, painters=2,
                                                     records=[other_lane] if other_lane else ())})
    monkeypatch.setattr(mod, "log", lambda *a: None)
    monkeypatch.setattr(mod, "check", lambda *a: None)
    monkeypatch.setattr(mod, "finish", lambda *a: None)
    monkeypatch.setattr(mod, "export_cmd", lambda profile, d: ["mkdir", "-p", str(d / "notes/research")])
    monkeypatch.setattr(mod, "NAMES", REPO_NAMES, raising=False)   # the imported runner has no gate
    rd = tmp_path / "run/T"

    def reader(cmd, cwd, out, err, env=None):
        (rd / "p1_record.md").write_text(record)
        return 0
    monkeypatch.setattr(mod, "run", reader)
    rd.mkdir(parents=True)
    (tmp_path / "s1.jsonl").write_text("{}\n")
    (rd / "p1_sittings.json").write_text(json.dumps([{"sitting": 1, "sessions": [str(tmp_path / "s1.jsonl")]}]))
    for marker in ("p1.exported", "p1.painted", "p2.painted"):
        (rd / marker).write_text("")
    (mod.studio("T1") / "notes").mkdir(parents=True)
    mod.chain("T")
    return (mod.studio("T2") / "notes/studio_notes.md").read_text(), sorted(f.name for f in rd.iterdir())


@pytest.mark.parametrize("other_lane", [False, True])
def test_a_free_text_lane_s_next_studio_gets_exactly_what_the_imported_runner_gave(tmp_path, monkeypatch, other_lane):
    old = imported_runner()
    for mod, d in ((old, tmp_path / "old"), (rc21, tmp_path / "new")):
        d.mkdir()
        other = None
        if other_lane:
            other = d / "X_p1_record.md"
            other.write_text("## Glazing\n- A glaze over dry paint stayed clear.\n")
        notes, files = free_text_notes(mod, d, monkeypatch, GOOD_RECORD, other)
        if mod is old:
            want, old_files = notes, files
    assert notes == want
    assert notes.endswith("\n## More notes from the studio\n\n" + GOOD_RECORD)
    # no record.json, condition.json or the like; only the reader's copies of the logs (round 21, H06)
    assert [f for f in files if f not in old_files] == ["p1_reader_logs"]


def code_repo(path):
    """A git repo at path with the round-21 tag (BASE, as the export reads it); its commit."""
    git = lambda *a: subprocess.run(["git", "-C", str(path), *a], check=True, capture_output=True, text=True).stdout
    path.mkdir()
    git("init", "-q")
    (path / "x").write_text("round 21's code\n")
    git("add", "x")
    git("-c", "user.name=t", "-c", "user.email=t@t", "commit", "-qm", "code")
    git("tag", rc21.BRANCH)
    return git("rev-parse", "HEAD").strip()


CANVAS = ('-- easel session "painting"\n\n--@ chunk 1\ncanvas{size=900, linen={15,13}, ground={{pile={{"lead white",1}},'
          'um=90,apply="knife"},{pile={{"lead white",5}},um=45,apply="roller"}}}\n\n--@ chunk 2\nwork(m, {pile=p})\n')


def chain_structured(tmp_path, monkeypatch, observations, rc=0, kind="structured", canvas=CANVAS, before=(), painters=2, read_last=False):
    """Run lane T (two painters, record_kind) with painter 1 painted in the r17 F fixture logs and a
    reader that writes observations (a dict as JSON, a str as it is, None nothing) and exits rc;
    files named in before are in the run folder from an earlier attempt."""
    from records_fixtures import R17F_LOGS, write_logs
    lines = []
    guide = rc21.HERE.parents[1] / "easel_guide.md"
    monkeypatch.setattr(rc21, "RUN", tmp_path / "run")
    monkeypatch.setattr(rc21, "A", tmp_path)
    monkeypatch.setattr(rc21, "BASE", tmp_path / "code")
    monkeypatch.setattr(rc21, "NAMES", rc21.HERE.parents[2] / "scripts/check_studio_names")
    monkeypatch.setattr(rc21, "LANES", {"T": rc21.lane("sargent", rc21.OPUS, painters=painters, record_kind=kind, read_last=read_last)})
    monkeypatch.setattr(rc21, "log", lines.append)
    monkeypatch.setattr(rc21, "check", lambda *a: None)
    monkeypatch.setattr(rc21, "finish", lambda *a: None)
    monkeypatch.setattr(rc21, "export_cmd", lambda profile, d: [
        "sh", "-c", f'mkdir -p "{d}/notes/research" && cp "{guide}" "{d}/notes/easel_guide.md"'])
    code_repo(tmp_path / "code")
    rd = tmp_path / "run/T"
    rd.mkdir(parents=True)
    for f in before:
        (rd / f).write_text("{}")
    (tmp_path / "logs").mkdir()
    sessions = [str(p) for p in write_logs(tmp_path / "logs", R17F_LOGS)]
    calls = []

    def reader(cmd, cwd, out, err, env=None):
        scope = json.loads(env["READER_SCOPE"])
        calls.append(scope)
        assert scope["write"] == str(rd / "p1_observations.json")
        copies = [str(rd / "p1_reader_logs" / f"log{i}.jsonl") for i in range(1, len(sessions) + 1)]
        assert scope["read"][:2] == copies                       # the reader's copies (log_copy.py)
        if observations is not None:
            (rd / "p1_observations.json").write_text(
                observations if isinstance(observations, str) else json.dumps(observations))
        return rc
    monkeypatch.setattr(rc21, "run", reader)
    (rd / "p1_sittings.json").write_text(json.dumps([{"sitting": 1, "sessions": sessions[:1]},
                                                     {"sitting": 2, "sessions": sessions[1:]}]))
    for marker in ("p1.exported", "p1.painted", "p2.painted"):
        (rd / marker).write_text("")
    d1 = rc21.studio("T1")
    for sub in ("notes", "paintings/lua"):
        (d1 / sub).mkdir(parents=True)
    (d1 / "notes/journal.md").write_text("- day one\n")
    (d1 / "notes/easel_guide.md").write_text(guide.read_text())
    (d1 / "paintings/lua/painting.lua").write_text(canvas)
    rc21.chain("T")
    return lines, calls


def test_a_structured_record_reaches_the_next_studio_as_the_rendered_notes(tmp_path, monkeypatch):
    import hashlib
    from records_fixtures import R17F, doc
    lines, calls = chain_structured(tmp_path, monkeypatch, doc(R17F))
    rd = tmp_path / "run/T"
    assert (rd / "p1.done").exists() and len(calls) == 1
    golden = (rc21.HERE / "fixtures/section7_render.md").read_text().split("\nObserved on linen 16")[0]
    notes = (rc21.studio("T2") / "notes/studio_notes.md").read_text()
    assert notes == (rc21.HERE / "studio_notes.md").read_text() + "\n" + golden
    rec = json.loads((rd / "p1_record.json").read_text())
    # stamped with the runner's own constants (BRANCH round-24, the commit it names in BASE), not an older round's
    assert (rec["schema"], rec["condition"], rec["round"], rec["lane"], rec["slot"], rec["medium"], rec["profile"]) == (
        "chain-record/1", "chain-inherited, non-neutral", 24, "T", 1, "oil", "sargent")
    assert rec["code"] == {"tag": rc21.BRANCH, "commit": rc21.code_commit()} and len(rec["code"]["commit"]) == 40
    assert rec["box"]["name"] == "default"
    assert rec["support"] == {"kind": "linen", "linen": [15, 13], "ground_layers": ["knife", "roller"]}
    assert rec["reader"]["model"] == "openai-codex/gpt-6.1-sol" and len(rec["logs"]) == 2
    assert [o["resolved"][0]["chunk"] for o in rec["observations"]] == [129, None, 162]
    inh = json.loads((rd / "p2_inherited.json").read_text())
    assert (inh["inherited"], inh["of"]) == (3, 3)
    assert inh["notes_sha256"] == hashlib.sha256(notes.encode()).hexdigest()
    assert "T2: inherited 3 of 3 observations (p1: 3/3)" in lines
    brief = (rd / "p1_reader_brief.md").read_text()
    # the brief lists the reader's copies; the record keeps the original logs its evidence is checked against
    copies = [rd / "p1_reader_logs" / f"log{i}.jsonl" for i in (1, 2)]
    assert f"- log 1: {copies[0]}\n- log 2: {copies[1]}" in brief and "{" + "OUT}" not in brief
    assert rec["logs"][0].endswith("logs/log1.jsonl") and all(c.exists() for c in copies)


@pytest.mark.parametrize("canvas, support, line", [
    # a comment's words never reach the notes (review C, finding 5)
    (CANVAS.replace("apply=\"roller\"}}}", "apply=\"roller\"}}} -- apply=\"Claude\""),
     {"kind": "linen", "linen": [15, 13], "ground_layers": ["knife", "roller"]},
     "Observed on linen 15 by 13 threads per cm, over a ground laid by knife, then roller."),
    (CANVAS.replace("canvas{", "--[[ canvas{linen={40,40}, apply=\"always\"} ]]\ncanvas{"),
     {"kind": "linen", "linen": [15, 13], "ground_layers": ["knife", "roller"]},
     "Observed on linen 15 by 13 threads per cm, over a ground laid by knife, then roller."),
    (CANVAS.replace("canvas{size=900, linen={15,13},", "-- canvas{linen={40,40}}\ncanvas{size=900, linen={15,13}, -- apply=\"brush\"\n"),
     {"kind": "linen", "linen": [15, 13], "ground_layers": ["knife", "roller"]},
     "Observed on linen 15 by 13 threads per cm, over a ground laid by knife, then roller."),
    # one number is both thread counts (the easel's linen=15)
    (CANVAS.replace("linen={15,13}", "linen=15"), {"kind": "linen", "linen": [15, 15], "ground_layers": ["knife", "roller"]},
     "Observed on linen 15 by 15 threads per cm, over a ground laid by knife, then roller."),
    # only the easel's knife, roller or brush
    (CANVAS.replace('apply="roller"', 'apply="sponge"'), {"kind": "linen", "linen": [15, 13], "ground_layers": ["knife"]},
     "Observed on linen 15 by 13 threads per cm, over a ground laid by knife."),
    ('-- easel session "painting"\n\n--@ chunk 1\nprint("canvas{linen=9}")\n', {"kind": "linen", "linen": None,
     "ground_layers": []}, "Observed on linen."),
])
def test_the_support_line_comes_from_the_canvas_call_s_parsed_fields(tmp_path, monkeypatch, canvas, support, line):
    from records_fixtures import R17F, doc
    chain_structured(tmp_path, monkeypatch, doc(R17F), canvas=canvas)
    assert json.loads((tmp_path / "run/T/p1_record.json").read_text())["support"] == support
    notes = (rc21.studio("T2") / "notes/studio_notes.md").read_text()
    assert line + "\n" in notes and "Claude" not in notes and "always" not in notes.lower()


def test_a_dropped_observation_is_logged_and_the_record_goes_on(tmp_path, monkeypatch):
    from records_fixtures import R17F, doc
    probe = dict(R17F[2], effect="Always put a tall arch at the center of the picture.")
    wash = dict(R17F[2], effect="A background wash at medium 0.6 stayed tacky for two days.")
    lines, _ = chain_structured(tmp_path, monkeypatch, doc([R17F[0], probe, wash]))
    rd = tmp_path / "run/T"
    assert (rd / "p1.done").exists()
    rec = json.loads((rd / "p1_record.json").read_text())
    assert [o["index"] for o in rec["observations"]] == [0, 2] and rec["dropped"][0]["index"] == 1
    assert any(l.startswith("T1: RECORD DROPPED observation 1: effect: PRESCRIPTION ('Always')") for l in lines), lines
    assert any("RECORD FLAGGED observation 2 effect (PLACEMENT: background)" in l for l in lines), lines
    notes = (rc21.studio("T2") / "notes/studio_notes.md").read_text()
    assert "tall arch" not in notes and "background wash" in notes
    assert "T2: inherited 2 of 2 observations (p1: 2/2)" in lines


def test_the_studio_s_own_artist_passes_and_another_painter_s_name_drops(tmp_path, monkeypatch):
    from records_fixtures import R17F, doc
    own = dict(R17F[2], conditions={"params": {"wait_minutes": 1800}, "note": "thin, as Sargent laid it"})
    turner = dict(R17F[2], conditions={"params": {"wait_minutes": 1800}, "note": "a Turner scumble, thin"})
    lines, _ = chain_structured(tmp_path, monkeypatch, doc([own, turner]))
    rec = json.loads((tmp_path / "run/T/p1_record.json").read_text())
    assert [o["index"] for o in rec["observations"]] == [0]
    assert rec["dropped"] == [{"index": 1, "why": ["note: a painter's name"]}]


@pytest.mark.parametrize("observations, rc, why", [
    (None, 0, "no record was written"),
    ("## Blending\n- The badger only moves wet paint.\n", 0, "not JSON"),
    ({"schema": "chain-observations/1", "observations": []}, 1, "the reader exited 1"),
])
def test_a_structured_record_that_cant_be_used_stops_the_lane(tmp_path, monkeypatch, observations, rc, why):
    # an earlier attempt's p1_record.json isn't this one's: it is gone even when this one fails
    lines, _ = chain_structured(tmp_path, monkeypatch, observations, rc, before=["p1_record.json"])
    rd = tmp_path / "run/T"
    assert not (rd / "p1.done").exists() and not (rd / "p2.exported").exists() and not (rd / "p1_record.json").exists()
    assert any("RECORD REJECTED" in l and why in l for l in lines), lines
    assert (rd / "p1_observations.rejected.json").exists() == (observations is not None)


@pytest.mark.parametrize("done_before", [False, True])
def test_read_last_reads_a_single_painter_once_and_starts_no_other(tmp_path, monkeypatch, done_before):
    # done_before: a runner without read_last finished the painter; the rerun reads it
    from records_fixtures import R17F, doc
    lines, calls = chain_structured(tmp_path, monkeypatch, doc(R17F), painters=1, read_last=True,
                                    before=("p1.done",) if done_before else ())
    rd = tmp_path / "run/T"
    assert len(calls) == 1 and (rd / "p1_record.json").exists() and (rd / "p1.done").exists()
    assert not (rd / "p2_brief.md").exists() and not rc21.studio("T2").exists()
    rc21.chain("T")                                  # a rerun: read already, nothing to do
    assert len(calls) == 1


def test_a_lane_without_records_runs_no_reader_and_inherits_nothing(tmp_path, monkeypatch):
    lines, calls = chain_structured(tmp_path, monkeypatch, None, kind="none")
    assert calls == [] and (tmp_path / "run/T/p1.done").exists()
    assert (rc21.studio("T2") / "notes/studio_notes.md").read_text() == (rc21.HERE / "studio_notes.md").read_text()


def test_a_lane_keeps_one_kind_of_record(tmp_path, monkeypatch):
    import sys
    monkeypatch.setattr(rc21, "RUN", tmp_path / "run")
    (tmp_path / "run/T").mkdir(parents=True)
    (tmp_path / "run/T/p1_record.md").write_text(GOOD_RECORD)
    assert rc21.mixed_records("T", "free-text") is None
    assert "p1_record.md" in rc21.mixed_records("T", "structured")
    monkeypatch.setattr(rc21, "LANES", {"T": rc21.lane("sargent", rc21.OPUS, painters=2, record_kind="structured")})
    monkeypatch.setattr(sys, "argv", ["r21_chains.py", "--only", "T", "--dry"])
    monkeypatch.setattr(rc21, "DRY", False)                     # main() sets it; restored after the test
    with pytest.raises(SystemExit, match="one kind of record per lane"):
        rc21.main()
    (tmp_path / "run/T/p1_record.md").unlink()
    (tmp_path / "run/T/p1_record.json").write_text("{}")
    assert "p1_record.json" in rc21.mixed_records("T", "free-text")
    with pytest.raises(ValueError):
        rc21.lane("sargent", rc21.OPUS, record_kind="markdown")
    with pytest.raises(ValueError, match="another lane's free-text records"):
        rc21.lane("sargent", rc21.OPUS, records=["x.md"], record_kind="structured")


def test_dry_shows_a_structured_lane_s_reader_and_merge(tmp_path, monkeypatch):
    lines = []
    monkeypatch.setattr(rc21, "RUN", tmp_path / "run")
    monkeypatch.setattr(rc21, "A", tmp_path)
    monkeypatch.setattr(rc21, "DRY", True)
    monkeypatch.setattr(rc21, "log", lines.append)
    monkeypatch.setattr(rc21, "LANES", {"T": rc21.lane("sargent", rc21.OPUS, painters=2, record_kind="structured")})
    rc21.chain("T")
    out = "\n".join(lines)
    assert "T1: reader (structured: writes p1_observations.json" in out
    assert "T2: export sargent studio (then studio_notes.md + the records rendered for this studio" in out
    assert not (tmp_path / "run").exists()


def test_the_round_and_commit_are_round_24_s(tmp_path, monkeypatch):
    assert rc21.round_number() == 24
    sha = code_repo(tmp_path / "code")
    monkeypatch.setattr(rc21, "BASE", tmp_path / "code")
    assert rc21.code_commit() == sha
    monkeypatch.setattr(rc21, "BRANCH", "round-99")
    with pytest.raises(RuntimeError, match="no commit for round-99"):
        rc21.code_commit()


def test_every_profile_has_its_own_artist_s_names():
    assert set(rc21.OPENING) <= set(rc21.OWN_NAMES)


def test_only_a_structured_chain_lane_is_labeled_non_neutral(tmp_path, monkeypatch):
    chain_structured(tmp_path, monkeypatch, None, rc=1)
    c = json.loads((tmp_path / "run/T/condition.json").read_text())
    assert c["condition"] == "chain-inherited, non-neutral" and c["record_kind"] == "structured"
    assert set(c["reader_briefs_sha256"]) == {"reader_brief.md", "reader_brief_structured.md"}
    assert set(c["renderer_sha256"]) == {"record_schema.py", "record_render.py"}
    # a free-text chain and a single painter get none
    (tmp_path / "ft").mkdir()
    chain_one_record(tmp_path / "ft", monkeypatch, GOOD_RECORD, 0)
    assert not (tmp_path / "ft/run/T/condition.json").exists()
    (tmp_path / "one").mkdir()
    monkeypatch.setattr(rc21, "RUN", tmp_path / "one/run")
    monkeypatch.setattr(rc21, "LANES", {"S": rc21.lane("sargent", rc21.OPUS, record_kind="structured")})
    (tmp_path / "one/run/S").mkdir(parents=True)
    (tmp_path / "one/run/S/p1.done").write_text("")
    monkeypatch.setattr(rc21, "studio", lambda key: tmp_path / "one" / key)
    rc21.chain("S")
    assert not (tmp_path / "one/run/S/condition.json").exists()


def test_a_chain_lane_can_t_start_without_the_reader_s_files(tmp_path, monkeypatch):
    # a runner copied without reader.ts & co. would fail every reader at launch: refuse at startup instead
    here = tmp_path / "runner"
    here.mkdir()
    black = tmp_path / "pi-black.ts"
    black.write_text("")
    monkeypatch.setattr(rc21, "HERE", here)
    monkeypatch.setattr(rc21, "BLACK", black)
    monkeypatch.setattr(rc21, "LANES", {"C": rc21.lane("sargent", rc21.OPUS, painters=2),
                                        "S": rc21.lane("sargent", rc21.OPUS)})
    assert rc21.preflight(["S"]) == []                              # a single painter needs no reader
    [why] = rc21.preflight(["C", "S"])
    assert "chain lanes C need reader.ts, reader-scope.ts, reader_system_prompt.md, reader_brief.md" in why
    for f in ("reader.ts", "reader-scope.ts", "reader_system_prompt.md", "reader_brief.md"):
        (here / f).write_text("")
    assert rc21.preflight(["C", "S"]) == []
    black.unlink()
    assert rc21.preflight(["C"]) == []                              # an OpenAI reader doesn't load pi-black
    monkeypatch.setattr(rc21, "READER_BLACK", True)
    assert ["the reader loads pi-black" in w for w in rc21.preflight(["C"])] == [True]


def test_the_runner_folder_holds_every_file_a_chain_lane_needs():
    # what gets copied to the run folder is this folder: it must have what preflight asks for
    lanes = {"C": rc21.lane("sargent", rc21.OPUS, painters=2, record_kind="structured")}
    missing = [f for f in ("reader.ts", "reader-scope.ts", "reader_system_prompt.md", "reader_brief.md",
                           "reader_brief_structured.md", "record_schema.py", "record_render.py")
               if not (rc21.HERE / f).exists()]
    assert missing == [] and lanes


def test_a_structured_lane_can_t_start_without_its_commit(tmp_path, monkeypatch):
    monkeypatch.setattr(rc21, "LANES", {"C": rc21.lane("sargent", rc21.OPUS, painters=2, record_kind="structured")})
    monkeypatch.setattr(rc21, "DRY", False)
    monkeypatch.setattr(rc21, "BASE", tmp_path / "no-checkout")
    assert any("no commit for" in w for w in rc21.preflight(["C"]))


def test_the_runner_starts_only_from_a_clean_checkout_of_its_tag(tmp_path, monkeypatch):
    # round 24: the export archives the tag, the harness, check and finishing run from BASE's files
    git = lambda *a: subprocess.run(["git", "-C", str(tmp_path / "code"), *a], check=True, capture_output=True)
    monkeypatch.setattr(rc21, "BASE", tmp_path / "no-checkout")
    assert ["no commit for" in w for w in rc21.checkout_problems()] == [True]
    monkeypatch.setattr(rc21, "BASE", tmp_path / "code")
    code_repo(tmp_path / "code")
    assert rc21.checkout_problems() == []
    (tmp_path / "code/x").write_text("edited\n")
    assert ["uncommitted changes" in w for w in rc21.checkout_problems()] == [True]
    git("-c", "user.name=t", "-c", "user.email=t@t", "commit", "-qam", "after the tag")
    assert [f"not {rc21.BRANCH}" in w for w in rc21.checkout_problems()] == [True]


def test_a_later_sitting_opens_with_the_recovery_sections_and_a_first_or_continued_one_doesn_t():
    m = rc21.lane("sargent", rc21.DSF)["model"]
    assert rc21.painter_env(m, rc21.SITTING_MESSAGE)["PAINTER_SITTING_RECOVERY"] == "1"
    assert "PAINTER_SITTING_RECOVERY" not in rc21.painter_env(m, rc21.PAINTER_MSG)
    assert "PAINTER_SITTING_RECOVERY" not in rc21.painter_env(m, rc21.CONTINUE_MESSAGE)
    gem = rc21.GEM2                                               # a lane's own variables stay
    assert rc21.painter_env(gem, rc21.SITTING_MESSAGE)["PAINTER_MAX_IMAGES"] == "8"


@pytest.mark.parametrize("observations", [[], [{"category": "sky"}]])
def test_an_empty_record_lets_the_chain_go_on_with_nothing_new_inherited(tmp_path, monkeypatch, observations):
    lines, _ = chain_structured(tmp_path, monkeypatch, {"schema": "chain-observations/1", "observations": observations})
    rd = tmp_path / "run/T"
    assert (rd / "p1.done").exists() and json.loads((rd / "p1_record.json").read_text())["observations"] == []
    assert any("RECORD EMPTY" in l for l in lines), lines
    assert (rc21.studio("T2") / "notes/studio_notes.md").read_text() == (rc21.HERE / "studio_notes.md").read_text()


# round 22: pictures of the artist's paintings in the studio's reference/

def pictures(folder, *names, size=(8, 6)):
    """Tiny pictures named names in folder (made with Pillow, in the format their suffix names)."""
    from PIL import Image
    folder.mkdir(parents=True, exist_ok=True)
    for i, n in enumerate(names):
        Image.new("RGB", size, (40 * i, 90, 160)).save(folder / n)
    return folder


def tonn_studio(tmp_path, monkeypatch, ref, reference=True):
    """Run lane TONN up to its painter, with REFERENCE at ref (reference=False: a lane without the
    pictures): the studio, the log and the painters started."""
    lines, painted = [], []
    monkeypatch.setattr(rc21, "RUN", tmp_path / "run")
    monkeypatch.setattr(rc21, "A", tmp_path)
    monkeypatch.setattr(rc21, "REFERENCE", ref)
    monkeypatch.setattr(rc21, "NAMES", REPO_NAMES)
    monkeypatch.setattr(rc21, "LANES", {"TONN": rc21.lane("tonn", rc21.OPUS, record_kind="none", reference=reference)})
    monkeypatch.setattr(rc21, "log", lines.append)
    monkeypatch.setattr(rc21, "export_cmd", lambda profile, d: ["mkdir", "-p", str(d / "notes/research")])
    monkeypatch.setattr(rc21, "paint", lambda name, n, d, rd: painted.append(d))     # returns None: the lane stops
    (tmp_path / "run/TONN").mkdir(parents=True)
    rc21.chain("TONN")
    return rc21.studio("TONN1"), lines, painted


@pytest.mark.parametrize("names, readme, opening", [
    (["a.jpg"], None, "A picture of one of his\npaintings is in reference/, there with his permission: study it for his\n"),
    (["b.png", "a.jpg", "c.webp"], None, "Pictures of his paintings are\nin reference/, there with his permission: study them for his manner."),
    (["a.jpeg", "b.JPG"], "# Two still lifes\n\n- a.jpeg: grapes, 2019\n- b.JPG: a jug\n", "Pictures of his paintings are\n"),
])
def test_a_tonn_studio_gets_his_pictures_and_a_brief_for_as_many(tmp_path, monkeypatch, names, readme, opening):
    ref = pictures(tmp_path / "ref", *names)
    if readme:
        (ref / "README.md").write_text(readme)
    (ref / ".DS_Store").write_bytes(b"\0")
    originals = {n: (ref / n).read_bytes() for n in names}
    d, lines, painted = tonn_studio(tmp_path, monkeypatch, ref)
    assert painted == [d]
    assert sorted(f.name for f in (d / "reference").iterdir()) == sorted(names + ["README.md"])
    assert all((d / "reference" / n).read_bytes() == b for n, b in originals.items())
    want = readme or "# reference/\n\nPictures of paintings by Kendric Tonn:\n\n" + "".join(f"- {n}\n" for n in sorted(names))
    assert (d / "reference/README.md").read_text() == want
    brief = (d / "BRIEF.md").read_text()
    assert opening in brief
    assert ("reference/ (his paintings; reference/README.md lists them) and\nnotes/research/oil_paint_physics.md as needed."
            in brief)
    assert brief == (tmp_path / "run/TONN/p1_brief.md").read_text()


def test_a_picture_pi_would_resize_or_show_turned_is_copied_to_fit_upright_and_hers_stays_as_it_is(tmp_path, monkeypatch):
    from PIL import Image
    ref = pictures(tmp_path / "ref", "wide.png", size=(2400, 120))
    exif = Image.Exif()
    exif[0x0112] = 6                                                 # EXIF: turn 90 degrees to show
    Image.new("RGB", (8, 6)).save(ref / "phone.jpg", exif=exif)
    before = {n: (ref / n).read_bytes() for n in ("wide.png", "phone.jpg")}
    d, lines, _ = tonn_studio(tmp_path, monkeypatch, ref)
    with Image.open(d / "reference/wide.png") as im:
        assert (im.format, im.size) == ("PNG", (2000, 100))
    with Image.open(d / "reference/phone.jpg") as im:
        assert (im.format, im.size, im.getexif().get(0x0112, 1)) == ("JPEG", (6, 8), 1)
    assert {n: (ref / n).read_bytes() for n in before} == before
    assert any("wide.png: 2400x120 copied as 2000x100" in l for l in lines), lines


@pytest.mark.parametrize("make, why", [
    (lambda ref: None, "there is no folder"),
    (lambda ref: ref.mkdir(), "there is no picture in"),
    (lambda ref: (ref.mkdir(), (ref / "README.md").write_text("# Tonn\n"), (ref / "notes.pdf").write_bytes(b"%PDF")),
     "there is no picture in"),
    (lambda ref: (ref.mkdir(), (ref / "a.jpg").write_bytes(b"\0\0\0\x18ftypheic")), "isn't a picture pi's read tool shows"),
    (lambda ref: (pictures(ref, "a.jpg"), (ref / "README.md").write_text("A still life after Rembrandt.\n")),
     "name other painters"),
    (lambda ref: pictures(ref, "after-Vermeer.jpg"), "name other painters"),
])
def test_without_his_pictures_nothing_starts(tmp_path, monkeypatch, make, why):
    import sys
    ref = tmp_path / "tonn-reference"
    make(ref)
    started = []
    monkeypatch.setattr(rc21, "RUN", tmp_path / "run")
    monkeypatch.setattr(rc21, "REFERENCE", ref)
    monkeypatch.setattr(rc21, "NAMES", REPO_NAMES)
    monkeypatch.setattr(rc21, "LANES", {"TONN": rc21.lane("tonn", rc21.OPUS, record_kind="none")})
    monkeypatch.setattr(rc21, "chain", started.append)               # a lane that starts would export and paint
    monkeypatch.setattr(rc21, "watchdog", lambda stop: None)
    monkeypatch.setattr(sys, "argv", ["r21_chains.py"])
    monkeypatch.setattr(rc21, "DRY", False)
    with pytest.raises(SystemExit, match="can't start") as e:
        rc21.main()
    assert why in str(e.value) and str(ref) in str(e.value)          # says what's wrong, and where
    assert started == [] and not (tmp_path / "run").exists()


# round 22's warmup: a studio of the same profile without the pictures (lane(..., reference=False))

def test_a_tonn_studio_without_the_pictures_has_no_reference_folder_and_the_plain_brief(tmp_path, monkeypatch):
    ref = pictures(tmp_path / "ref", "a.jpg", "b.jpg")               # there, and not for this studio
    d, lines, painted = tonn_studio(tmp_path, monkeypatch, ref, reference=False)
    assert painted == [d] and not (d / "reference").exists()
    brief = (d / "BRIEF.md").read_text()
    assert ("Compose and paint one original picture in the manner of Kendric Tonn, at\n"
            "the easel, a simulator of oil paint on linen. The subject and composition\n"
            "are yours to invent. Work from knowledge and the notes in your studio;\n"
            "don't use reference images, image models or pictures of his work.\n\n## Your studio") in brief
    assert ("## What to read\nnotes/easel_guide.md; notes/studio_notes.md;\n"
            "notes/research/tonn_materials.md (his materials and method) and\n"
            "notes/research/oil_paint_physics.md as needed.\n") in brief
    assert "reference/" not in brief and brief == (tmp_path / "run/TONN/p1_brief.md").read_text()


def test_only_a_lane_that_runs_and_gets_pictures_needs_them(tmp_path, monkeypatch):
    import sys
    started = []
    monkeypatch.setattr(rc21, "RUN", tmp_path / "run")
    monkeypatch.setattr(rc21, "REFERENCE", tmp_path / "tonn-reference")          # no such folder
    monkeypatch.setattr(rc21, "LANES", {"BUNT": rc21.lane("tonn", rc21.BUNNY, record_kind="none", reference=False),
                                        "TONN": rc21.lane("tonn", rc21.OPUS, record_kind="none")})
    monkeypatch.setattr(rc21, "chain", started.append)
    monkeypatch.setattr(rc21, "watchdog", lambda stop: None)
    monkeypatch.setattr(rc21.time, "sleep", lambda s: None)
    monkeypatch.setattr(rc21, "DRY", False)
    monkeypatch.setattr(rc21, "MY_LANES", None)                      # main() sets it; restored after the test
    monkeypatch.setattr(rc21, "checkout_problems", lambda: [])         # no tag checkout here
    for only in ("TONN", "BUNT,TONN"):
        monkeypatch.setattr(sys, "argv", ["r21_chains.py", "--only", only])
        with pytest.raises(SystemExit, match="can't start: there is no folder"):
            rc21.main()
    assert started == []
    monkeypatch.setattr(sys, "argv", ["r21_chains.py", "--only", "BUNT"])
    rc21.main()
    assert started == ["BUNT"]


def test_a_process_watches_only_its_own_lanes_paintings(tmp_path, monkeypatch):
    # two processes from one run folder (--only BUNT, later --only TONN) share run/studios.json
    monkeypatch.setattr(rc21, "RUN", tmp_path / "run")
    monkeypatch.setattr(rc21, "A", tmp_path)
    monkeypatch.setattr(rc21, "LANES", {"BUNT": rc21.lane("tonn", rc21.BUNNY, record_kind="none", reference=False),
                                        "TONN": rc21.lane("tonn", rc21.OPUS, record_kind="none")})
    (tmp_path / "run").mkdir()
    for key in ("BUNT1", "TONN1"):
        log = rc21.studio(key) / "paintings/lua/painting.lua"
        log.parent.mkdir(parents=True)
        log.write_text("--@ chunk 1\n")
    assert set(json.loads((tmp_path / "run/studios.json").read_text())) == {"BUNT1", "TONN1"}
    monkeypatch.setattr(rc21, "MY_LANES", ["TONN"])
    rc21.monitor_histories()
    assert sorted(f.name for f in (tmp_path / "run/monitor").iterdir()) == ["TONN1.last"]
    assert [k for k, _ in rc21.our_studios()] == ["TONN1"]
