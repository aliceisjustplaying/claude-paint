"""Tests of the round-21 runner (r21_chains.py).

    uv run --no-project --with pytest pytest -q -p no:cacheprovider notes/round21/runner/
"""
import json
import subprocess

import pytest

import r21_chains as rc21

GO_LIMIT = '429: {"type":"GoUsageLimitError","message":"Go usage limit exceeded"}'      # round 19, KIMIF and MIMOF


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
    assert exts == [str(rc21.HERE / "reader.ts"), str(rc21.BLACK)]  # pi-black for its Anthropic model


GOOD_RECORD = "## Blending\n- The badger only moves wet paint; `blend(m, {clip=true})` stayed inside.\n"


def chain_one_record(tmp_path, monkeypatch, record, rc, journal=True, lanes=None):
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
    rd = tmp_path / "run/T"

    def reader(cmd, cwd, out, err, env=None):                  # the reader: writes the record, or doesn't
        scopes.append(json.loads(env["READER_SCOPE"]))
        if record is not None:
            (rd / "p1_record.md").write_text(record)
        return rc
    monkeypatch.setattr(rc21, "run", reader)
    rd.mkdir(parents=True, exist_ok=True)
    session = tmp_path / "s1.jsonl"
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
    assert scopes == [{"read": [str(tmp_path / "s1.jsonl"), *j, str(rd / "p1_reader_brief.md")],
                       "write": str(rd / "p1_record.md")}]
    assert GOOD_RECORD in (rc21.studio("T2") / "notes/studio_notes.md").read_text()
    assert any(l.endswith("record written") for l in lines), lines


def test_a_reader_that_fails_says_so(tmp_path, monkeypatch):
    # pi exits 1 when reader.ts refuses to load (no valid READER_SCOPE): the log names the exit
    lines, _ = chain_one_record(tmp_path, monkeypatch, None, 1)
    assert any("record MISSING (the reader exited 1" in l for l in lines), lines


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
