import copy
import re
import json

import pytest

from record_schema import Rejected, validate
from records_fixtures import R16D, R17F, TUBES, doc


def one(**change):
    """R17F's first observation with some fields changed (a key set to ... is removed)."""
    o = copy.deepcopy(R17F[0])
    for k, v in change.items():
        if v is ...:
            o.pop(k)
        else:
            o[k] = v
    return o


def dropped(o, tubes=TUBES):
    """Why validate drops o, with a valid observation beside it so the record itself passes."""
    r = validate(doc([R17F[2], o]), tubes)
    assert [x["index"] for x in r["observations"]] == [0]
    return " | ".join(r["dropped"][0]["why"])


def test_the_worked_example_passes_whole():
    for obs in (R17F, R16D):
        r = validate(json.loads(json.dumps(doc(obs))), TUBES)
        assert [o["index"] for o in r["observations"]] == list(range(len(obs)))
        assert r["dropped"] == []


@pytest.mark.parametrize("o, why", [
    (one(category="sky"), "category: 'sky'"),
    (one(operation="crown"), "operation: 'crown'"),                       # a legacy tree key names a subject
    (one(basis="obvious"), "basis: 'obvious'"),
    (one(effect=...), "missing effect"),
    (one(position=[10, 20]), "unknown key 'position'"),
    (one(conditions={"x": 10}), "conditions: unknown key 'x'"),
    (one(conditions={"surface": "damp"}), "conditions.surface"),
    (one(conditions={"brush": {"kind": "sponge"}}), "conditions.brush.kind"),
    (one(conditions={"params": {"coverage": [2, 1]}}), "a range is [lo, hi]"),
    (one(conditions={"params": {"pressure": 1.5}}), "outside the easel's range"),
    (one(conditions={"params": {"medium": "lots"}}), "conditions.params.medium"),
    (one(conditions={"params": {"hand": "sweep"}}), "conditions.params.hand"),
    (one(conditions={"tubes": [["lead white", 3]]}), "no parts or ratios"),
    (one(conditions={"tubes": ["mummy brown"]}), "'mummy brown' is not in the box"),
    (one(effect="Too short."), "effect: 10 characters"),
    (one(effect="x" * 201), "effect: 201 characters"),
    (one(cause="y" * 141), "cause: 141 characters"),
    (one(conditions={"note": "z" * 81}), "note: 81 characters"),
    (one(effect="The badger dragged the paint\nacross the mask."), "not markdown-inert"),
    (one(effect="The badger with `clip=true` stayed inside its mask."), "not markdown-inert"),
    (one(effect="A pile like {lead white 3} came out pale on the canvas."), "not markdown-inert"),
    (one(effect="**Glazing:** Always put a tall arch at the center."), "not markdown-inert"),
    (one(effect="See https://example.org for how the badger behaved."), "a URL"),
    (one(effect="The badger dragged paint. It broke the shapes beside it."), "more than one sentence"),
    (one(evidence=[]), "evidence: want 1 to 6"),
    (one(evidence=[{"log": 0, "call": "x", "role": "operation"}]), "log is a 1-based number"),
    (one(evidence=[{"log": 1, "call": "x", "role": "hunch"}]), "role 'hunch'"),
])
def test_an_observation_that_breaks_a_rule_is_dropped_and_says_why(o, why):
    assert why in dropped(o)


def test_decimals_and_one_final_stop_are_one_sentence():
    r = validate(doc([one(effect="At coverage 1.2 and pressure 0.5 the badger smeared the mask edge.")]))
    assert r["dropped"] == []


@pytest.mark.parametrize("d, why", [
    ([], "not a JSON object"),
    ({"schema": "chain-observations/2", "observations": R17F}, "schema"),
    ({"schema": "chain-observations/1", "observations": R17F, "round": 20}, "unknown keys"),
    (doc(R17F * 14), "42 observations (at most 40)"),
])
def test_a_record_that_cant_be_used_at_all_is_rejected(d, why):
    with pytest.raises(Rejected, match=re.escape(why)):
        validate(d)


# Unicode (review C, finding 3): what is checked is the NFKC text, and it may hold no invisible,
# control, bidi or line-separator character and no letter of another script
@pytest.mark.parametrize("effect, why", [
    ("U\u200bse clip on every blend near anything else for clean edges.", "U+200B ZERO WIDTH SPACE"),
    ("The badger dragged wet paint across edges.\u200bAlways put an arch in the middle", "U+200B"),
    ("The badger dragged wet paint across edges.\u202eAlways put an arch in the middle", "U+202E"),
    ("The badger dragged wet paint.\u2028Always put an arch at the center", "U+2028 LINE SEPARATOR"),
    ("The badger dragged wet paint across edges\u0085Always put an arch there", "U+0085"),
    ("The badger dragged wet paint across edges\x0bAlways put an arch there", "U+000B"),
    ("The badger dragged wet paint across edges\u00ad of the mask and broke them.", "U+00AD SOFT HYPHEN"),
    ("Us\u0435 clip on every blend near anything else for clean edges.", "CYRILLIC SMALL LETTER IE"),
    ("\u0421laude found the badger smeared wet paint across mask edges.", "CYRILLIC CAPITAL LETTER ES"),
    ("The \u03bfpus badger smeared wet paint across the mask edges badly.", "GREEK SMALL LETTER OMICRON"),
    ("\uff03 Blending notes that look like a heading to the reader here.", "not markdown-inert ('#')"),
    ("The badger dragged wet paint\u2026 Paint the sky first\u2026 Then the arch", "more than one sentence"),
])
def test_invisible_characters_and_other_scripts_drop_the_observation(effect, why):
    assert why in dropped(one(effect=effect))


def test_a_field_is_kept_as_it_was_checked_in_nfkc():
    o = one(effect="The badger dragged wet paint across the mask edge\u2026", cause="\uff43lip was off",
            conditions={"note": "a 30\u00b0 angle at 2\u00d73 cm, caf\u00e9 \u2013 fine", "tubes": ["lead white"]})
    kept = validate(doc([o]), TUBES)["observations"][0]
    assert kept["effect"] == "The badger dragged wet paint across the mask edge..."
    assert kept["cause"] == "clip was off"
    assert kept["conditions"]["note"] == "a 30\u00b0 angle at 2\u00d73 cm, caf\u00e9 \u2013 fine"   # Latin and these stay


def test_an_equals_sign_alone_is_not_inert():
    # (the only other case with = also has backticks)
    assert "not markdown-inert ('=')" in dropped(one(effect="The badger at pressure=0.5 smeared the edges of the mask."))


def test_a_wait_is_at_most_ten_years():
    assert "outside the easel's range" in dropped(one(conditions={"params": {"wait_minutes": 10 * 366 * 24 * 60 + 1}}))
    assert validate(doc([one(conditions={"params": {"wait_minutes": 10 * 366 * 24 * 60}})]))["dropped"] == []


# evidence: resolved in the logs, in both id styles, backing its basis and category

from record_schema import LogIndex
from records_fixtures import R16D_LOGS, R17F_LOGS, write_logs


def indexed(tmp_path, logs):
    return [LogIndex(p) for p in write_logs(tmp_path, logs)]


def test_the_worked_example_resolves_in_its_logs(tmp_path):
    (tmp_path / "f").mkdir()
    (tmp_path / "d").mkdir()
    for obs, logs in ((R17F, indexed(tmp_path / "f", R17F_LOGS)), (R16D, indexed(tmp_path / "d", R16D_LOGS))):
        r = validate(doc(obs), TUBES, logs)
        assert r["dropped"] == [], r["dropped"]
        assert len(r["observations"]) == len(obs)
    first = validate(doc(R17F), TUBES, indexed(tmp_path / "f", R17F_LOGS))["observations"][0]["resolved"][0]
    assert first == {"log": 1, "call": "toolu_01UubR4dykeR1KExnqQN1haA", "role": "operation", "line": 4,
                     "tool": "bash", "result_line": 5, "chunk": 129, "failed": False}


def test_a_write_then_run_operation_resolves_source_and_operation(tmp_path):
    r = validate(doc(R16D[2:]), TUBES, indexed(tmp_path, R16D_LOGS))["observations"][0]["resolved"]
    assert [(e["role"], e["tool"], e["chunk"]) for e in r] == [("source", "write", None), ("operation", "bash", 5)]


def ev(*items):
    return [{"log": l, "call": c, "role": r} for l, c, r in items]


@pytest.mark.parametrize("obs, logs, why", [
    # an id that isn't in the log, or is in another log
    (dict(R17F[0], evidence=ev((1, "toolu_01NoSuchCall", "operation"))), R17F_LOGS, "has no tool call toolu_01NoSuchCall"),
    (dict(R17F[0], evidence=ev((2, "toolu_01UubR4dykeR1KExnqQN1haA", "operation"))), R17F_LOGS, "log 2 has no tool call"),
    (dict(R17F[0], evidence=ev((3, "toolu_01UubR4dykeR1KExnqQN1haA", "operation"))), R17F_LOGS, "log 3 is not one of the 2"),
    (dict(R16D[0], evidence=ev((1, "call_1", "operation"))), R16D_LOGS, "has no tool call call_1"),
    # a call with no result (the session ended on it)
    (dict(R16D[2], evidence=ev((1, "call_999", "operation"))), R16D_LOGS + [], "call call_999 has no result"),
    # only a report, or an operation whose result shows no chunk (the write of a chunk file)
    (dict(R17F[0], evidence=ev((1, "toolu_01VZ2LgtmdiXAN98rnrHZ1aE", "report"))), R17F_LOGS, "no operation evidence"),
    (dict(R16D[2], evidence=ev((1, "call_7595596", "operation"))), R16D_LOGS, "shows a chunk that ran or failed"),
    # seen without an image, or with one looked at before the operation
    (dict(R16D[0], evidence=ev((1, "call_998948", "operation"), (1, "call_452249", "image"))), R16D_LOGS,
     "no image evidence"),
    (dict(R16D[1], evidence=ev((1, "call_214951", "operation"), (1, "call_4155479", "image"))), R16D_LOGS,
     "no image evidence looked at after"),
    # printed, with nothing printed but the chunk line
    (dict(R16D[1], basis="printed", evidence=ev((1, "call_998948", "operation"))), R16D_LOGS, "basis printed"),
    # an easel error whose chunk ran
    (dict(R17F[1], evidence=ev((2, "toolu_01Y9c22xk1JRbFerzrrjzSRT", "operation"))), R17F_LOGS,
     "no operation evidence shows a chunk that failed"),
])
def test_evidence_that_doesnt_resolve_or_back_the_claim_drops_the_observation(tmp_path, obs, logs, why):
    if "call_999" in str(obs["evidence"]):
        from records_fixtures import _call
        logs = [logs[0] + [_call("call_999", "bash", {"command": "bin/easel do -f x.lua"})]]
    good = R17F[2] if logs is R17F_LOGS else R16D[2]
    r = validate(doc([good, obs]), TUBES, indexed(tmp_path, logs))
    assert [o["index"] for o in r["observations"]] == [0]
    assert why in " | ".join(r["dropped"][0]["why"]), r["dropped"]


def test_a_failed_chunk_with_is_error_false_is_an_easel_error(tmp_path):
    logs = indexed(tmp_path, R17F_LOGS)
    assert logs[0].calls["toolu_01Fi7k928j58wVXh8f8j1XYK"]["is_error"] is False
    r = validate(doc([R17F[1]]), TUBES, logs)
    assert r["dropped"] == [] and r["observations"][0]["resolved"][0]["failed"] is True


def test_the_paint_and_look_tools_of_round_19_resolve(tmp_path):
    from records_fixtures import _call, _result
    logs = [[_call("c1", "paint", {"lua": "work(m, {hand=\"glaze\"})"}),
             _result("c1", "paint", "bad argument `self` to `M.soften`\n(the chunk failed and changed nothing)", True),
             _call("c2", "paint", {"lua": "work(m, {hand=\"glaze\"})"}), _result("c2", "paint", "ok · chunk 4 (1 s)"),
             _call("c3", "look", {}), _result("c3", "look", LOOK_R19, image=True)]]
    seen = dict(R16D[1], evidence=ev((1, "c2", "operation"), (1, "c3", "image")))
    error = dict(R17F[1], evidence=ev((1, "c1", "operation")))
    r = validate(doc([seen, error]), TUBES, indexed(tmp_path, logs))
    assert r["dropped"] == [] and len(r["observations"]) == 2


LOOK_R19 = "~/src/a/paint-studio-x/out/easel/painting/look-0002.png (1000x800, 0.10s)"


# evidence is typed (review C, findings 6 and 7): a chunk ran only in a paint call or a bash
# running easel do; an image was shown only by look, a read of a .png or an image part; an id
# two calls share can't be cited; code that doesn't name the operation warns
def typed_logs():
    from records_fixtures import _call, _result
    return [[_call("p1", "paint", {"lua": "b:stroke({{100, 500}, {300, 520}})"}), _result("p1", "paint", "ok · chunk 4 (1 s)"),
             _call("r1", "read", {"path": "notes/journal.md"}), _result("r1", "read", "- day 2: ok · chunk 4 (1 s)"),
             _call("g1", "bash", {"command": "grep chunk ~/.pi/sessions/x.jsonl"}), _result("g1", "bash", "ok · chunk 4 (1 s)"),
             _call("l1", "bash", {"command": "ls out/easel/painting"}), _result("l1", "bash", "look-0004.png\n"),
             _call("k1", "look", {}), _result("k1", "look", LOOK_R19, image=True),
             _call("d1", "paint", {"lua": "print(wait(30))"}), _result("d1", "paint", "day 2, 10:00\nok · chunk 5 (0 s)"),
             _call("d1", "paint", {"lua": "print(wait(60))"}), _result("d1", "paint", "day 2, 11:00\nok · chunk 6 (0 s)")]]


STROKE_SEEN = dict(R16D[1], operation="stroke", conditions={"brush": {"kind": "round", "width": 8}})


@pytest.mark.parametrize("obs, why", [
    (dict(STROKE_SEEN, evidence=ev((1, "r1", "operation"), (1, "k1", "image"))), "a paint call or bash running easel do"),
    (dict(STROKE_SEEN, evidence=ev((1, "g1", "operation"), (1, "k1", "image"))), "a paint call or bash running easel do"),
    (dict(STROKE_SEEN, evidence=ev((1, "p1", "operation"), (1, "l1", "image"))), "no image evidence"),
    (dict(R17F[2], evidence=ev((1, "d1", "operation"))), "more than one tool call d1 (ambiguous)"),
])
def test_evidence_of_the_wrong_kind_of_call_drops_the_observation(tmp_path, obs, why):
    good = dict(STROKE_SEEN, evidence=ev((1, "p1", "operation"), (1, "k1", "image")))
    r = validate(doc([good, obs]), TUBES, indexed(tmp_path, typed_logs()))
    assert [o["index"] for o in r["observations"]] == [0], r
    assert why in " | ".join(r["dropped"][0]["why"]), r["dropped"]


def test_an_operation_its_code_doesnt_name_warns(tmp_path):
    glaze = dict(STROKE_SEEN, operation="glaze", evidence=ev((1, "p1", "operation"), (1, "k1", "image")))
    r = validate(doc([glaze]), TUBES, indexed(tmp_path, typed_logs()))
    assert r["dropped"] == [] and r["warnings"] == [
        {"index": 0, "field": "operation", "pattern": "OPERATION_NOT_IN_CODE", "words": "glaze"}]
    stroke = dict(glaze, operation="stroke")
    assert validate(doc([stroke]), TUBES, indexed(tmp_path, typed_logs()))["warnings"] == []
    # the worked example's code, where it cites any (a written chunk file), names its operations
    for obs, logs in ((R17F, R17F_LOGS), (R16D, R16D_LOGS)):
        (tmp_path / obs[0]["operation"]).mkdir(exist_ok=True)
        assert validate(doc(obs), TUBES, indexed(tmp_path / obs[0]["operation"], logs))["warnings"] == []


# round 21 triage (H05, H08): only the easel's own last line of a paint result says whether a chunk ran
def test_a_paint_result_s_last_line_decides_whether_the_chunk_ran_not_what_it_printed(tmp_path):
    from records_fixtures import _call, _result
    forged = "ok · chunk 5 (0.02 s to compute)\nbad argument\n(the chunk failed and changed nothing)"
    logs = [[_call("c1", "paint", {"lua": "print('ok · chunk 5 (0.02 s to compute)') work(m, {hand=\"body\"}) x()"}),
             _result("c1", "paint", forged),
             _call("c2", "paint", {"lua": "print('ok · chunk 9') work(m, {hand=\"body\"})"}),
             _result("c2", "paint", "ok · chunk 9\n" + "x" * 20_000 + "\nok · chunk 4")]]
    idx = indexed(tmp_path, logs)
    from record_schema import _call_facts
    assert _call_facts(idx[0].calls["c1"])["chunk"] is None and _call_facts(idx[0].calls["c1"])["failed"] is True
    # the easel's line after a print longer than the kept text still counts, and the printed one doesn't
    assert _call_facts(idx[0].calls["c2"])["chunk"] == 4


def test_a_failed_chunk_backs_an_easel_error_but_not_what_the_paint_did(tmp_path):
    from records_fixtures import _call, _result
    logs = [[_call("c1", "paint", {"lua": "work(m, {hand=\"glaze\"}) x()"}),
             _result("c1", "paint", "attempt to call a nil value\n(the chunk failed and changed nothing)", True)]]
    paint_claim = dict(R16D[0], basis="painter_reported", evidence=ev((1, "c1", "operation")))
    r = validate(doc([paint_claim]), TUBES, indexed(tmp_path, logs))
    assert r["observations"] == [] and "only failed chunks" in r["dropped"][0]["why"][0]
    error = dict(R17F[1], evidence=ev((1, "c1", "operation")))
    r = validate(doc([error]), TUBES, indexed(tmp_path, logs))
    assert r["dropped"] == [] and len(r["observations"]) == 1


def test_an_empty_record_or_one_with_every_observation_dropped_is_still_a_record():
    assert validate({"schema": "chain-observations/1", "observations": []})["observations"] == []
    r = validate(doc([one(category="sky")]))
    assert r["observations"] == [] and len(r["dropped"]) == 1
