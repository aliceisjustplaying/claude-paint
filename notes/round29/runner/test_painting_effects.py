import hashlib
import json

import pytest

import painting_chunks
import r21_chains as runner


def evidence(studio, chunks, effects):
    source = "".join(f"--@ chunk {i}\n{chunk}\n" for i, chunk in enumerate(chunks, 1))
    path = studio / "paintings/lua/painting.lua"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(source)
    path = studio / "out/easel/painting/painting-effects.json"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(dict(source_sha256=hashlib.sha256(source.encode()).hexdigest(),
                                   mark_effects=effects)))
    return path


@pytest.mark.parametrize("mark", [
    'b["touch"](b,400,300,{pressure=0.6})',
    'jobs[1]()',
    'p:side({{100,400},{900,400}})',
    'smudge({{100,400},{900,400}})',
])
def test_executed_marks_require_another_sitting(tmp_path, mark):
    chunks = ['canvas{size=200,aspect=1.25}',
              'b=brush("round",4); jobs={function() b:touch(400,300) end}']
    evidence(tmp_path, chunks, [1, 0])
    before = runner.count_painting(tmp_path)
    evidence(tmp_path, chunks + [mark], [1, 0, 1])
    after = runner.count_painting(tmp_path)
    assert after == before + 1
    sitting = dict(sitting=3, status="completed", reviewed=True,
                   effects_before=before, effects_after=after, chunks_before=2, chunks_after=3)
    assert runner.next_step([sitting]) == ("new", 4)


def test_preparation_queries_and_uncalled_helpers_are_not_painting(tmp_path):
    chunks = ['canvas{size=200,aspect=1.25}']
    evidence(tmp_path, chunks, [1])
    before = runner.count_painting(tmp_path)
    chunks += ['jobs={function() b:touch(400,300) end}',
               'p=pile{{"lead white",1}}; b=brush("round",4); b:load(p)',
               'print(wait(60)); print(drying(400,300))']
    evidence(tmp_path, chunks, [1, 0, 0, 0])
    after = runner.count_painting(tmp_path)
    assert after == before
    sitting = dict(sitting=4, status="completed", reviewed=True,
                   effects_before=before, effects_after=after, chunks_before=1, chunks_after=4)
    assert runner.next_step([sitting])[0] is None


@pytest.mark.parametrize("invalid", ["missing", "stale", "broken", "negative", "wrong-type", "short"])
def test_unavailable_evidence_does_not_hide_accepted_chunks(tmp_path, invalid):
    path = evidence(tmp_path, ['canvas{}', 'jobs[1]()'], [1, 1])
    data = json.loads(path.read_text())
    if invalid == "missing":
        path.unlink()
    elif invalid == "stale":
        data["source_sha256"] = "0" * 64
    elif invalid == "broken":
        path.write_text("{")
    elif invalid == "negative":
        data["mark_effects"] = [1, -1]
    elif invalid == "wrong-type":
        data["mark_effects"] = [1, True]
    else:
        data["mark_effects"] = []
    if invalid not in ("missing", "broken"):
        path.write_text(json.dumps(data))
    assert runner.count_painting(tmp_path) is None
    for before, after in [(1, None), (None, 2), (None, None)]:
        sitting = dict(sitting=2, status="completed", reviewed=True,
                       effects_before=before, effects_after=after, chunks_before=1, chunks_after=2)
        assert runner.next_step([sitting]) == ("new", 3)


def test_legacy_sittings_keep_their_original_evidence():
    assert runner.added_painting(dict(painting_before=1, painting_after=2))
    assert runner.added_painting(dict(chunks_before=1, chunks_after=2))
    assert runner.added_painting(dict(effects_before=None, effects_after=1,
                                    painting_before=3, painting_after=3,
                                    chunks_before=3, chunks_after=4))


def test_completed_legacy_sitting_uses_chunk_bounds_before_old_painting_counts():
    sitting = dict(sitting=2, status="completed", reviewed=True,
                   painting_before=2, painting_after=2, chunks_before=2, chunks_after=3)
    assert runner.next_step([sitting]) == ("new", 3)
    assert runner.next_step([dict(sitting, effects_before=None, effects_after=None)]) == ("new", 3)
    assert runner.next_step([dict(sitting, chunks_after=2)])[0] is None


@pytest.mark.parametrize("mark", ["smudge({{1,2},{3,4}})", "blow(500, 400)", "tap()",
                                  "b:dust({{1,2},{3,4}})", "p:side({{1,2},{3,4}})"])
def test_the_source_classifier_counts_pastel_marks(mark):
    assert painting_chunks.classify(f"--@ chunk 1\nb = brush('round', 4)\n--@ chunk 2\n{mark}\n") == [(1, False), (2, True)]
