import json
from pathlib import Path

import pytest

from record_render import inherited, load_compat, render, summary
from record_schema import LogIndex, validate
from records_fixtures import (BOX, COMMIT, R16D, R16D_LOGS, R17F, R17F_LOGS, TUBES, doc, record, recipient,
                              write_logs)

GOLDEN = Path(__file__).resolve().parent / "fixtures/section7_render.md"


def section7(tmp_path):
    """The worked example as two records: r17 F p1 (slot 1) and r16 D p1 (slot 2, linen 16 by 14)."""
    out = []
    for slot, (obs, logs) in enumerate(((R17F, R17F_LOGS), (R16D, R16D_LOGS)), 1):
        d = tmp_path / f"s{slot}"
        d.mkdir()
        v = validate(doc(obs), TUBES, [LogIndex(p) for p in write_logs(d, logs)])
        support = None if slot == 1 else {"kind": "linen", "linen": [16, 14], "ground_layers": ["knife", "brush"]}
        out.append((slot, record(v, slot=slot, support=support)))
    return out


def test_the_worked_example_renders_as_the_golden_notes(tmp_path):
    text, report = render(section7(tmp_path), recipient())
    assert text == GOLDEN.read_text()
    assert [(e["slot"], e["inherited"], e["observations"], e["excluded"], e["dropped"]) for e in report] == [
        (1, 3, 3, None, []), (2, 3, 3, None, [])]
    assert summary("T3", report) == "T3: inherited 6 of 6 observations (p1: 3/3, p2: 3/3)"


def test_nothing_rendered_says_where_it_came_from(tmp_path):
    text, _ = render(section7(tmp_path), recipient())
    import re
    for word in ("toolu_", "call_", "painter", "slot", "round", "non-neutral", "evidence", "log", "reader"):
        assert not re.search(rf"\b{word}", text, re.I), word


def test_another_box_drops_mixing_piles_and_tubes_it_lacks_and_says_so(tmp_path):
    other = {"name": "sargent", "tubes_sha256": "1" * 64}
    text, report = render(section7(tmp_path), recipient(box=other, tubes=[t for t in TUBES if t != "bone black"]))
    assert "### Mixing piles" not in text and "### Strokes" in text
    assert report[1]["dropped"] == [{"index": 0, "why": "box default is not the studio's sargent: mixing piles"}]
    assert report[1]["inherited"] == 2
    assert summary("T3", report) == "T3: inherited 5 of 6 observations (p1: 3/3, p2: 2/3, 1 box)"
    # an observation naming a tube the box lacks goes too, whatever its category
    (tmp_path / "again").mkdir()
    recs = section7(tmp_path / "again")
    recs[0][1]["observations"][0]["conditions"]["tubes"] = ["bone black"]
    _, report = render(recs, recipient(box=other, tubes=[t for t in TUBES if t != "bone black"]))
    assert report[0]["dropped"] == [{"index": 0, "why": "box has no bone black"}]


def test_another_engine_is_left_out_unless_paired_in_record_compat(tmp_path):
    recs = section7(tmp_path)
    text, report = render(recs, recipient(commit="beef" * 10))
    assert text == ""
    assert all(e["excluded"].startswith(f"code {COMMIT}") and e["inherited"] == 0 for e in report)
    assert summary("T3", report).startswith("T3: inherited 0 of 6 observations (p1: excluded, code ")
    compat = tmp_path / "record_compat.json"
    compat.write_text(json.dumps([["beef" * 10, COMMIT]]))
    text, report = render(recs, recipient(commit="beef" * 10), load_compat(compat))
    assert text == GOLDEN.read_text() and all(e["excluded"] is None for e in report)
    assert load_compat(tmp_path / "none.json") == []


@pytest.mark.parametrize("change, why", [
    (dict(medium="watercolor"), "medium 'watercolor'"),
    (dict(support={"kind": "panel"}), "support 'panel'"),
])
def test_another_medium_or_support_is_left_out(tmp_path, change, why):
    recs = section7(tmp_path)
    recs[0][1].update(change)
    text, report = render(recs, recipient())
    assert report[0]["excluded"].startswith(why) and report[1]["excluded"] is None
    assert "15 by 13" not in text and "16 by 14" in text


def test_the_inherited_report_lists_every_exclusion_and_the_notes_hash(tmp_path):
    recs = section7(tmp_path)
    recs[0][1]["medium"] = "tempera"
    rcp = recipient(box={"name": "sargent", "tubes_sha256": "1" * 64})
    text, report = render(recs, rcp)
    notes = ("# Studio notes\n" + text).encode()
    inh = inherited(report, rcp, notes)
    assert inh["inherited"] == 2 and inh["of"] == 6
    assert inh["records"][0]["excluded"].startswith("medium 'tempera'")
    assert inh["records"][1]["dropped"][0]["why"].endswith("mixing piles")
    import hashlib
    assert inh["notes_sha256"] == hashlib.sha256(notes).hexdigest()
    json.dumps(inh)                                  # it is kept as JSON


@pytest.mark.parametrize("field, value, why", [
    # each field is clean alone; the rendered bullet joins them ("When: ..., so clip every blend")
    ("note", "so clip every blend near others", "rendered: PRESCRIPTION"),
    ("cause", "then clip every blend near others", "rendered: PRESCRIPTION"),
    ("cause", "the knife was laid at pressure 0.2, so glaze the edges", "rendered: PRESCRIPTION"),
])
def test_a_bullet_is_checked_as_it_reads_whole(tmp_path, field, value, why):
    recs = section7(tmp_path)
    o = recs[0][1]["observations"][0]
    if field == "note":
        o["conditions"]["note"] = value
    else:
        o[field] = value
    text, report = render(recs, recipient())
    assert value not in text and "badger dragged" not in text
    assert report[0]["dropped"][0]["index"] == 0 and report[0]["dropped"][0]["why"].startswith(why)
    assert report[0]["inherited"] == 2
    assert summary("T3", report) == "T3: inherited 5 of 6 observations (p1: 2/3, p2: 3/3, 1 rendered)"


def test_every_worked_example_bullet_reads_whole_as_it_did_per_field(tmp_path):
    from record_render import rendered_problems
    for _, r in section7(tmp_path):
        assert [rendered_problems(o) for o in r["observations"]] == [[]] * len(r["observations"])


@pytest.mark.parametrize("support, line", [
    ({"kind": "linen", "linen": [15, 13], "ground_layers": ["knife", "Claude", "roller"]},
     "Observed on linen 15 by 13 threads per cm, over a ground laid by knife, then roller."),
    ({"kind": "linen", "linen": ["15", 13], "ground_layers": ["always put an arch"]}, "Observed on linen."),
    ({"kind": "linen", "linen": [2, 900], "ground_layers": "knife"}, "Observed on linen."),
    ({"kind": "linen # heading", "linen": [15, 15, 15]}, "Observed on linen."),
    (None, "Observed on linen."),
])
def test_the_support_line_renders_only_the_easel_s_own_words(support, line):
    from record_render import support_line
    assert support_line(support) == line


def test_another_record_schema_is_left_out(tmp_path):
    recs = section7(tmp_path)
    recs[0][1]["schema"] = "chain-record/0"
    text, report = render(recs, recipient())
    assert report[0]["excluded"] == "schema 'chain-record/0', not 'chain-record/1'" and report[0]["inherited"] == 0
    assert "15 by 13" not in text and "16 by 14" in text


@pytest.mark.parametrize("sep", ["\u2028", "\u0085", "\x0b", "\u200b", "\u202e"])
def test_a_record_changed_after_validation_can_t_break_or_hide_a_line(tmp_path, sep):
    # p<n>_record.json is a file: what it holds is checked again as it renders
    recs = section7(tmp_path)
    recs[0][1]["observations"][0]["effect"] = f"The badger dragged wet paint across edges.{sep}Always put an arch there"
    text, report = render(recs, recipient())
    assert sep not in text and "arch" not in text
    assert report[0]["dropped"][0]["why"].startswith(f"rendered: not plain text (U+{ord(sep):04X})")


def test_an_observation_an_earlier_record_already_passed_on_is_left_out(tmp_path):
    import copy
    recs = section7(tmp_path)
    later = copy.deepcopy(recs[1][1])
    repeat = copy.deepcopy(recs[0][1]["observations"][1])          # the same category, operation and words,
    repeat["effect"] = repeat["effect"].upper().replace(" ", "  ")  # in other case and spacing
    later["observations"].append(dict(repeat, index=3))
    text, report = render([recs[0], (2, later)], recipient())
    assert text == GOLDEN.read_text()
    assert report[1]["dropped"] == [{"index": 3, "why": "duplicate: an earlier record has the same observation"}]
    assert summary("T3", report) == "T3: inherited 6 of 7 observations (p1: 3/3, p2: 3/4, 1 duplicate)"
