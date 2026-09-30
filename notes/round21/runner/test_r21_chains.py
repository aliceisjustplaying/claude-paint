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
