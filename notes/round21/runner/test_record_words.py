"""What a structured record's short fields may say: a clear prescription or placement drops the
observation, an ambiguous word only warns, another painter or a painter's name drops it. Measured
on every sentence of the released records and on studio_notes.md."""
import copy
import json
import re
from pathlib import Path

import pytest

from record_schema import studio_names, validate, word_hits
from records_fixtures import R16D, R17F, TUBES, doc

HERE = Path(__file__).resolve().parent
NAMES = HERE.parents[2] / "scripts/check_studio_names"
CORPUS = json.loads((HERE / "fixtures/r16_r17_record_sentences.json").read_text())["sentences"]


def sentences(text):
    """studio_notes.md as single sentences (items joined across wrapped lines, backticks removed)."""
    items, cur = [], None
    for line in text.splitlines():
        if re.match(r"^\s*(?:[-*]|\d+\.)\s", line) or line.startswith("#") or not line.strip():
            items += [cur] if cur else []
            cur = line if line.strip() and not line.startswith("#") else None
        else:
            cur = f"{cur} {line.strip()}" if cur else line
    items += [cur] if cur else []
    body = [re.sub(r"^\s*(?:[-*]|\d+\.)\s+", "", i).strip().replace("`", "") for i in items]
    return [s for b in body for s in re.split(r"(?<=[.!?])\s+(?=[A-Z(])", b) if s.strip()]


def test_the_released_records_drop_only_their_clear_prescriptions():
    dropped = [(s["at"], s["text"]) for s in CORPUS if word_hits(s["text"])[0]]
    clear = [(s["at"], s["text"]) for s in CORPUS if s["clear"]]
    assert len(CORPUS) == 716 and len(clear) == 13
    assert dropped == clear
    for text in ("Use clip=true on every blend near anything else.", "Lay the darks after the pale has set.",
                 "Pass it a number.", "In a large even passage, local corrections kept failing: repaint the whole passage instead."):
        assert (text in [t for _, t in clear]), text


@pytest.mark.parametrize("notes", sorted(HERE.parents[1].glob("round*/runner/studio_notes.md")),
                         ids=lambda p: p.parts[-3])
def test_no_studio_note_drops(notes):
    assert [s for s in sentences(notes.read_text()) if word_hits(s)[0]] == []


@pytest.mark.parametrize("text, drop", [
    # what to do
    ("Always put a tall arch at the center of the picture.", ["PRESCRIPTION", "PLACEMENT"]),
    ("Never glaze before the underlayer is dry.", ["PRESCRIPTION"]),
    ("Glazing: keep the glaze thin over dark passages.", ["PRESCRIPTION"]),
    ("To darken them, add more bone black and less white.", ["PRESCRIPTION"]),
    ("Streaks that show in a close crop can pass in the whole view, so judge both.", ["PRESCRIPTION"]),
    ("The badger drags paint unless clipped, so run it with clip on.", ["PRESCRIPTION"]),
    ("Mask sums are errors (use the map method).", ["PRESCRIPTION"]),
    ("A badger pass is safest when you should clip it to the mask.", ["PRESCRIPTION"]),
    ("It's best to lay the darks after the pale has set.", ["PRESCRIPTION"]),
    # an easel verb and its object (review C, finding 2)
    ("Clip every blend that sits near anything else in the painting.", ["PRESCRIPTION"]),
    ("Mix the darks with more medium so they spread over the ground evenly.", ["PRESCRIPTION"]),
    ("Glaze the shadows twice before the lights go in.", ["PRESCRIPTION"]),
    ("Wait a day before any glaze over lead white passages.", ["PRESCRIPTION"]),
    ("The badger smeared the edges; paint the sky before the arch.", ["PRESCRIPTION"]),
    ("The badger dragged wet paint... Stipple the hedge before the sky.", ["PRESCRIPTION"]),
    # where things sit in the picture
    ("The dark mass sits in the lower third of the picture.", ["PLACEMENT"]),
    ("A stipple in the foreground read as a hedge of dabs.", ["PLACEMENT"]),
    ("The brightest patch stayed on the left side of the painting.", ["PLACEMENT"]),
    ("A dark accent at the focal point stayed crisp.", ["PLACEMENT"]),
])
def test_clear_prescriptions_and_placements_drop(text, drop):
    assert [n for n, _ in word_hits(text)[0]] == drop


@pytest.mark.parametrize("text, warn", [
    ("Paint laid up to the edge of the canvas thinned out over the weave.", "PLACEMENT"),
    ("A background wash at medium 0.6 stayed tacky for two days.", "PLACEMENT"),
    ("The pile's composition set how fast it dried.", "PLACEMENT"),
    ("The horizon line stayed crisp under the mask.", "PLACEMENT"),
    ("Thresholded noise showed only on one side of the canvas.", "PLACEMENT"),
    ("A hairline highlight swelled only slightly in the middle.", "PLACEMENT"),
    ("A curve that stopped at x 620 painted a rectangle beyond its end, out to x 700.", "PLACEMENT"),
    ("The glaze was laid at y 330 across the full width.", "PLACEMENT"),
    ("Patching the ring never matched.", "PRESCRIPTION"),
    ("A rigger with pressure falling to 0 tapers to a hair and suits fine lines.", "PRESCRIPTION"),
])
def test_ambiguous_words_only_warn(text, warn):
    drops, warns = word_hits(text)
    assert drops == [] and [n for n, _ in warns] == [warn]


@pytest.mark.parametrize("text", [
    "Thin paint lets a line show and body color hides it.",         # adjectives and the easel's names start facts
    "Run dry, the tip loses its point and splits.",
    "Use of a clipped badger kept the mask edge crisp.",
    "The knife left paint only on the top of the weave.",
    "Lay-in strokes over the wet ground picked up its color.",
    "Clip on a body pass kept a narrow band's edge crisp.",
    "A whole-field blend at 1000 x 520 units took 32 s.",
])
def test_material_facts_that_look_like_commands_pass(text):
    assert word_hits(text) == ([], [])


def test_no_worked_example_observation_warns():
    for obs in (R17F, R16D):
        r = validate(doc(obs), TUBES)
        assert r["warnings"] == [] and r["dropped"] == []


def one(**fields):
    o = copy.deepcopy(R17F[2])
    o.update(fields)
    return o


def test_a_prescription_in_a_record_drops_that_observation_and_the_record_goes_on():
    r = validate(doc([R17F[2], one(effect="Always put a tall arch at the center of the picture."),
                      one(cause="Use a longer wait for thick paint.")]), TUBES)
    assert [o["index"] for o in r["observations"]] == [0]
    assert [d["index"] for d in r["dropped"]] == [1, 2]
    assert "effect: PRESCRIPTION ('Always')" in r["dropped"][0]["why"]
    assert "effect: PLACEMENT ('center of the picture')" in r["dropped"][0]["why"]
    assert r["dropped"][1]["why"] == ["cause: PRESCRIPTION ('Use')"]


def test_an_ambiguous_word_warns_and_the_observation_goes_on():
    r = validate(doc([one(effect="A background wash at medium 0.6 stayed tacky for two days.")]), TUBES)
    assert r["dropped"] == [] and r["warnings"] == [
        {"index": 0, "field": "effect", "pattern": "PLACEMENT", "words": "background"}]


def test_review_as_bypasses_cant_be_written():
    # the N12 review's "**Glazing:** Always put ..." passed the free-text gate; here it's not inert
    r = validate(doc([R17F[2], one(effect="**Glazing:** Always put a tall arch at the center.")]), TUBES)
    assert "not markdown-inert" in " ".join(r["dropped"][0]["why"])


@pytest.mark.parametrize("field, value", [
    ("effect", "The earlier painter's badger dragged wet paint across the mask."),
    ("cause", "Opus said the pile was too lean."),
    ("note", "as the other painters did it"),
])
def test_another_painter_in_any_field_drops(field, value):
    o = one(**({"conditions": {"note": value}} if field == "note" else {field: value}))
    r = validate(doc([R17F[2], o]), TUBES)
    assert "another painter" in " ".join(r["dropped"][0]["why"])


def test_a_painters_name_in_a_note_drops_but_the_studios_own_artist_passes(tmp_path):
    names = studio_names(NAMES, ["Sargent"], tmp_path)
    turner = one(conditions={"params": {"wait_minutes": 1800}, "note": "a Turner scumble, thin"})
    own = one(conditions={"params": {"wait_minutes": 1800}, "note": "thin, as Sargent laid it"})
    r = validate(doc([R17F[2], turner, own]), TUBES, names=names)
    assert [o["index"] for o in r["observations"]] == [0, 2]
    assert r["dropped"] == [{"index": 1, "why": ["note: a painter's name"]}]
