import json

from painting_chunks import count_painting_chunks
from r17_chains import etime_seconds, next_sitting, overdue, session_error


def painting_log(*chunks):
    return "-- easel session\n" + "".join(
        f"\n--@ chunk {number}\n{chunk}\n" for number, chunk in enumerate(chunks, 1)
    )


def test_only_mark_making_chunks_extend_a_painting():
    source = painting_log(
        'canvas{size=400}',
        'print(wait(60)); print(drying(20, 20))',
        'function dab(x, y) b:touch(x, y) end',
        'dab(20, 20)',
    )
    assert count_painting_chunks(source) == 3


def test_crashes_are_retried_and_do_not_count_as_sittings():
    crash = dict(sitting=1, status="crashed", chunks_before=10, chunks_after=10)
    assert next_sitting([crash]) == 2
    done = dict(
        sitting=2,
        status="completed",
        chunks_before=10,
        chunks_after=12,
        painting_before=8,
        painting_after=8,
    )
    assert next_sitting([crash, done]) is None
    assert next_sitting([dict(crash, sitting=n) for n in range(1, 7)]) is None


def test_provider_errors_are_not_mistaken_for_completed_sittings(tmp_path):
    session = tmp_path / "session.jsonl"
    rows = [
        {"message": {"role": "assistant", "stopReason": "toolUse"}},
        {"message": {"role": "assistant", "stopReason": "error", "errorMessage": "usage limit"}},
    ]
    session.write_text("".join(json.dumps(row) + "\n" for row in rows))
    assert session_error([session]) == "usage limit"


def test_macos_elapsed_times_drive_the_studio_watchdog():
    assert etime_seconds("01:02") == 62
    assert etime_seconds("2-03:04:05") == 183845
    studios = ["/work/paint-studio-one"]
    ps = [
        "101 31:00 python render.py",
        "102 31:00 pi --print",
        "103 29:59 python render.py",
        "104 31:00 python elsewhere.py",
    ]
    cwds = {"101": studios[0], "102": studios[0], "103": studios[0], "104": "/work/elsewhere"}
    assert overdue(ps, cwds, studios, 1800) == [
        ("101", 1860, studios[0], "python render.py")
    ]
