import json
import os

import pytest

from painting_chunks import classify, count_painting_chunks


def log(*chunks):
    return "-- easel session\n\n" + "".join(f"--@ chunk {i}\n{c}\n\n" for i, c in enumerate(chunks, 1))


def paints(*chunks):
    return [p for _, p in classify(log(*chunks))]


def test_queries_waits_and_mixing_are_not_painting():
    assert paints(
        'print(table.concat(tubes(), ", "))',
        "print(wait(60))",
        "print(drying(400, 300), drying(10, 10))",
        'p = pile{{"lead white", 3}, {"smalt", 1}}\nb = brush("filbert", 8)\nb:load(p, 0.8)',
        "m = rect(0, 0, 1000, 400):roughen(4, 40, 1)\nprint(m:area())",
        'o = outline{{1, 2}, {3, 4}, char="firm"}\nprint(o:length())',
    ) == [False] * 6


def test_mark_making_verbs_are_painting():
    assert paints(
        'canvas{size=400, aspect=1.25, ground={{pile={{"lead white", 1}}, um=100, apply="knife"}}}',
        "work(m, {hand=\"body\", pile=p})",
        "blend(rect(0, 0, 1000, 446), {angle=0})",
        "b:stroke({{100, 500}, {300, 520}}, {pressure={0.9, 0.3}})",
        "b:touch(400, 300, {pressure=0.6})",
        "o:paint(b, {pressure=0.8, dip={p, 0.6}})",
        'h = pencil("HB")\nh:rule({0, 440}, {1000, 440}, {pressure=0.25})',
        "print(lose(m, {pile=p}))",
        "erase(pts, {strength=0.9})",
        "print(wait(30))\nstipple(m, {pile=p})",
    ) == [True] * 10


def test_verbs_in_comments_strings_and_keys_do_not_count():
    assert paints(
        "-- next: work the sky, then blend it\nprint(wait(20))",
        '--[[ b:stroke(...) later ]]\nprint("work(m) after it dries")',
        "f = body_of{spine={{1, 2}, {3, 4}}, widths={3, 4}, blend=0.8}",
        'print(work_left, m.work)',
    ) == [False] * 4


def test_helpers_defined_in_a_painting_chunk_paint_later():
    assert paints(
        "function dab(x, y) b:touch(x, y, {pressure=0.5}) end",
        "dab(100, 200)",
        "local function band(y0, y1) return rect(0, y0, 1000, y1 - y0) end\nprint(band(1, 2):area())",
        "sweep = function(m) work(m, {hand=\"broad\", pile=p}) end",
        "for i = 1, 3 do sweep(rect(0, i * 10, 100, 10)) end",
        "go = sweep",
        "go(everywhere())",
    ) == [True, True, False, True, True, True, True]   # an alias mentions a painting name: counts (errs toward painting)


def test_the_stop_count_for_a_sitting_that_only_queried():
    before = log('canvas{size=400}', "work(m, {pile=p})")
    after = log('canvas{size=400}', "work(m, {pile=p})", "print(wait(120))", "print(drying(500, 300))")
    assert after.count("--@ chunk") - before.count("--@ chunk") == 2      # the old count: two more chunks
    assert count_painting_chunks(after) - count_painting_chunks(before) == 0


def done(k, before, after):
    return dict(sitting=k, status="completed", chunks_before=before, chunks_after=after, painting_before=before, painting_after=after)


def test_the_painter_is_done_after_a_sitting_it_ends_without_painting():
    from r19_chains import next_step
    s1 = dict(sitting=1, status="completed", chunks_before=0, chunks_after=40, painting_before=0, painting_after=31)
    s2 = dict(sitting=2, status="completed", chunks_before=40, chunks_after=43, painting_before=31, painting_after=31)
    assert next_step([s1]) == ("new", 2)
    assert next_step([s1, s2])[0] is None                  # three new chunks, none of them painting
    old = dict(sitting=2, status="completed", chunks_before=40, chunks_after=43)   # an older runner's record
    assert next_step([s1, old]) == ("new", 3)


def test_a_painter_that_keeps_painting_stops_after_four_sittings():
    from r19_chains import next_step
    painting = [done(k, 10 * k, 10 * k + 10) for k in range(1, 4)]
    assert next_step(painting) == ("new", 4)
    assert next_step(painting + [done(4, 40, 50)])[0] is None


def test_a_crashed_sitting_is_no_judgment_and_counts_only_if_it_painted():
    from r19_chains import next_step
    s1 = dict(sitting=1, status="crashed", exit=1, chunks_before=0, chunks_after=33, painting_before=0, painting_after=23)
    s2 = dict(sitting=2, status="crashed", exit=1, chunks_before=33, chunks_after=33, painting_before=23, painting_after=23)
    assert next_step([s1, s2]) == ("new", 3)              # no new painting, but a crash: another sitting
    two = [done(k, 23 + k, 24 + k) for k in (3, 4)]
    assert next_step([s1, s2] + two, max_sittings=4) == ("new", 5)   # s1 painted: it counts (three of four); s2 doesn't
    assert next_step([s1, s2] + two, max_sittings=3)[0] is None
    crashes = [dict(s2, sitting=i) for i in range(1, 7)]
    assert next_step(crashes[:5], max_crashes=6) == ("new", 6)
    step, why = next_step(crashes, max_crashes=6)
    assert step is None and why.startswith("NOT FINISHED")


def test_the_wait_after_a_crash_honors_the_providers_retry_hint():
    from r19_chains import crash_wait, retry_hint
    google = 'Quota exceeded ... \\nPlease retry in 53.812706879s.\\", ... \\"retryDelay\\": \\"53s\\"'
    assert retry_hint(google) == 53.812706879
    assert crash_wait(1, google) == 90                       # the backoff is longer
    assert crash_wait(1, "Please retry in 200s") == 215      # the hint is longer
    assert crash_wait(2, "") == 180 and crash_wait(99, "") == 1200


def test_etime_reads_every_form_macos_ps_prints():
    from r19_chains import etime_seconds
    assert etime_seconds("00:45") == 45
    assert etime_seconds("31:02") == 31 * 60 + 2
    assert etime_seconds("01:20:59") == 3600 + 20 * 60 + 59
    assert etime_seconds("07-00:28:52") == 7 * 86400 + 28 * 60 + 52
    assert etime_seconds("COMMAND") is None


STUDIO = "/h/src/a/paint-studio-abc123"


def test_the_watchdog_stops_long_runners_in_its_studios_whatever_their_name():
    from r19_chains import overdue
    ps = [
        "101 45:00 python3 grid.py",                                  # a painter's script (rounds 17-18 spared python3)
        "102 45:00 node draw.js",
        "103 45:00 /bin/bash -c sleep 99999",
        "104 45:00 pi",                                               # the painter's pi: spared
        f"105 45:00 {STUDIO}/bin/easel serve painting",               # the easel server: spared
        f"106 45:00 /bin/bash /r/code/scripts/check_painting {STUDIO} /r/F/p1_check",
        "107 05:00 python3 young.py",                                 # not long enough
        "108 45:00 python3 other.py",                                 # another round's studio
        f"109 45:00 {STUDIO}/bin/easel do -",                         # its cwd elsewhere, the studio in its command
        "110 45:00 sleep 99999",                                      # the runner itself
        f"111 45:00 {STUDIO}/bin/easel open",                         # a replay: spared (it stops itself when stalled)
    ]
    cwds = {p: STUDIO for p in ("101", "102", "103", "104", "105", "107", "110", "111")}
    cwds.update({"106": "/r", "108": "/h/src/a/paint-studio-ffffff", "109": "/"})
    got = [pid for pid, *_ in overdue(ps, cwds, [STUDIO], 30 * 60, me="110")]
    assert got == ["101", "102", "103", "109"]


def test_the_watchdog_finds_a_real_process_in_a_studio(tmp_path):
    import subprocess
    from r19_chains import cwds_of_all, overdue
    studio = tmp_path / "paint-studio-test"
    studio.mkdir()
    p = subprocess.Popen(["sleep", "30"], cwd=studio)
    try:
        ps = subprocess.run(["/bin/ps", "-Ao", "pid=,etime=,command="], capture_output=True, text=True).stdout.splitlines()
        found = overdue(ps, cwds_of_all(), [str(studio.resolve())], 0)
        assert [pid for pid, *_ in found] == [str(p.pid)]
    finally:
        p.kill()


GO_LIMIT = '429: {"type":"GoUsageLimitError","message":"Go usage limit exceeded"}'      # round 19, KIMIF and MIMOF
GOOGLE_EMPTY = ('{"error":{"message":"{\\n  \\"error\\": {\\n    \\"code\\": 402,\\n    \\"message\\": \\"Your prepayment '
                'credits are depleted. Please go to AI Studio","status":"Payment Required"}}')   # round 19, GEMF


def test_a_usage_limit_is_waited_out_but_an_empty_balance_is_a_crash():
    from r19_chains import usage_limit
    assert usage_limit(GO_LIMIT)
    assert usage_limit("5-hour usage limit reached. Resets in 2hr 15min")
    assert not usage_limit(GOOGLE_EMPTY)
    assert not usage_limit("Please retry in 53s")                 # a per-minute quota: the crash backoff handles it


def test_the_reset_time_a_usage_limit_names():
    from r19_chains import limit_reset_s
    assert limit_reset_s("5-hour usage limit reached. Resets in 2hr 15min") == 2 * 3600 + 15 * 60
    assert limit_reset_s("Weekly usage limit reached. Resets in 2 days.") == 2 * 86400
    assert limit_reset_s(GO_LIMIT) is None


def test_a_sitting_cut_off_by_a_usage_limit_carries_on_in_its_session():
    from r19_chains import next_step
    # round 19's Kimi: sitting 7 painted 16 chunks in 32 min, then the Go limit
    s7 = dict(sitting=7, status="limited", worked=True, sessions=["s7.jsonl"],
              chunks_before=30, chunks_after=50, painting_before=26, painting_after=42)
    assert next_step([s7]) == ("continue", 7)
    # a limited sitting that never got a reply starts over, and none of them counts toward anything
    empty = [dict(s7, sitting=k, worked=False, chunks_before=50, painting_before=42) for k in range(8, 30)]
    assert next_step([s7] + empty, max_sittings=4, max_crashes=6) == ("new", 30)    # limited ones never count
    # a sitting the runner itself cut off carries on in its session too
    assert next_step([dict(s7, status="interrupted")]) == ("continue", 7)
    # the continued part ends: judged over the whole sitting (26 -> 42), so the painter goes on
    part2 = dict(s7, part=2, status="completed", chunks_after=50, painting_after=42)
    assert next_step([s7, part2]) == ("new", 8)


def test_the_first_probe_after_a_usage_limit_counts_from_when_the_sitting_ended():
    from r19_chains import first_probe_wait, LIMIT_PROBE_S
    assert first_probe_wait(GO_LIMIT, ended=1000, now=1000) == LIMIT_PROBE_S
    assert first_probe_wait(GO_LIMIT, ended=1000, now=1000 + 10 * 3600) == 0      # a resume hours later asks at once
    assert first_probe_wait("usage limit reached. Resets in 2hr", ended=0, now=3600) == 3600 + 60


def test_the_reader_is_launched_as_isolated_as_the_painter():
    from r19_chains import HARNESS, reader_cmd
    cmd = reader_cmd("Read the brief.")
    assert [f for f in HARNESS if f.startswith("--no-")] == [f for f in cmd if f.startswith("--no-")]
    for flag in ("--system-prompt", "--tools"):                 # no default system prompt or tools
        assert flag in cmd


GOOD_RECORD = "## Blending\n- The badger only moves wet paint; `blend(m, {clip=true})` stayed inside.\n"


@pytest.mark.parametrize("record, rc, why", [
    (GOOD_RECORD, 0, None),
    (None, 0, "no record was written"),
    ("\n", 0, "the record is empty"),
    (GOOD_RECORD, 1, "the reader exited 1"),
    (GOOD_RECORD + "- Turner's scumbles were thinner.\n", 0, "painters' names"),
    (GOOD_RECORD + "- The earlier painter glazed twice.\n", 0, "another painter"),
    (GOOD_RECORD + "local p = pile{{\"smalt\", 1}}\n", 0, "code on line 3"),
    (GOOD_RECORD + "- Sky: pile{{\"lead white\", 3}, {\"smalt\", 1}}.\n", 0, "a color recipe"),
])
def test_a_reader_record_reaches_the_next_studio_only_if_it_passes(tmp_path, monkeypatch, record, rc, why):
    import r19_chains as rc19
    lines = []
    monkeypatch.setattr(rc19, "RUN", tmp_path / "run")
    monkeypatch.setattr(rc19, "A", tmp_path)
    monkeypatch.setattr(rc19, "LANES", {"T": rc19.lane("friedrich", rc19.OPUS, painters=2)})
    monkeypatch.setattr(rc19, "log", lines.append)
    monkeypatch.setattr(rc19, "check", lambda *a: None)
    monkeypatch.setattr(rc19, "finish", lambda *a: None)
    monkeypatch.setattr(rc19, "export_cmd", lambda profile, d: ["mkdir", "-p", str(d / "notes/research")])

    def reader(cmd, cwd, out, err, env=None):                  # the reader: writes the record, or doesn't
        if record is not None:
            (rd / "p1_record.md").write_text(record)
        return rc
    monkeypatch.setattr(rc19, "run", reader)
    rd = tmp_path / "run/T"
    rd.mkdir(parents=True)
    session = tmp_path / "s1.jsonl"
    session.write_text("{}\n")
    (rd / "p1_sittings.json").write_text(json.dumps([{"sitting": 1, "sessions": [str(session)]}]))
    for marker in ("p1.exported", "p1.painted", "p2.painted"):   # painter 1 has painted; painter 2 won't paint
        (rd / marker).write_text("")
    rc19.studio("T1").mkdir()

    rc19.chain("T")
    if why is None:
        assert (rd / "p1.done").exists()
        assert GOOD_RECORD in (rc19.studio("T2") / "notes/studio_notes.md").read_text()
    else:
        assert not (rd / "p1.done").exists() and not (rd / "p2.exported").exists()
        assert not (rd / "p1_record.md").exists()
        assert any("RECORD REJECTED" in l and why in l for l in lines), lines


def test_the_code_is_the_tag_s_even_with_its_branch_gone(tmp_path, monkeypatch):
    # round 19's scripts and harness come from the round-19 tag, not a branch or a worktree; the
    # export script, in the extracted tree (no .git), reads the tag from the repo's git
    import subprocess
    import r19_chains as rc19
    repo = tmp_path / "claude-paint"
    git = lambda *a: subprocess.run(["git", "-C", str(repo), *a], check=True, capture_output=True, text=True).stdout
    (repo / "scripts").mkdir(parents=True)
    git("init", "-q", "-b", "r19-base")
    git("config", "user.email", "t@t"); git("config", "user.name", "t")
    (repo / "scripts/export_r16_studio").write_text("round 19's export\n")
    git("add", "-A"); git("commit", "-qm", "round 19")
    git("tag", "-a", "round-19", "-m", "round 19")
    (repo / "scripts/export_r16_studio").write_text("later\n")
    git("commit", "-qam", "later")
    git("checkout", "-q", "--detach"); git("branch", "-D", "r19-base")
    monkeypatch.setattr(rc19, "REPO", repo)
    monkeypatch.setattr(rc19, "BASE", tmp_path / "run/code")
    monkeypatch.setattr(rc19, "log", lambda msg: None)

    rc19.checkout()
    assert (tmp_path / "run/code/scripts/export_r16_studio").read_text() == "round 19's export\n"
    assert not (tmp_path / "run/code/.git").exists()
    env = rc19.export_env()
    shown = subprocess.run(["git", "-C", str(tmp_path / "run/code"), "show", f"{env['R16_BRANCH']}:scripts/export_r16_studio"],
                           env={**os.environ, **env}, capture_output=True, text=True)
    assert shown.stdout == "round 19's export\n", shown.stderr
