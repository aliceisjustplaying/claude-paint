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


def test_next_sitting_stops_after_a_sitting_that_only_queried():
    from r18r_open import next_sitting
    s1 = dict(sitting=1, status="completed", chunks_before=0, chunks_after=40, painting_before=0, painting_after=31)
    s2 = dict(sitting=2, status="completed", chunks_before=40, chunks_after=43, painting_before=31, painting_after=31)
    assert next_sitting([s1]) == 2
    assert next_sitting([s1, s2]) is None                  # three new chunks, none of them painting
    old = dict(sitting=2, status="completed", chunks_before=40, chunks_after=43)   # an older runner's record
    assert next_sitting([s1, old]) == 3


def test_a_crashed_sitting_is_no_judgment_and_counts_only_if_it_painted():
    from r18r_open import next_sitting
    s1 = dict(sitting=1, status="crashed", exit=1, chunks_before=0, chunks_after=33, painting_before=0, painting_after=23)
    s2 = dict(sitting=2, status="crashed", exit=1, chunks_before=33, chunks_after=33, painting_before=23, painting_after=23)
    assert next_sitting([s1, s2]) == 3                     # no new painting, but a crash: another sitting
    done = [dict(sitting=i, status="completed", chunks_before=i, chunks_after=i + 1, painting_before=i, painting_after=i + 1)
            for i in (3, 4)]
    assert next_sitting([s1, s2] + done, max_sittings=4) == 5   # s1 painted: it counts (three of four); s2 doesn't
    assert next_sitting([s1, s2] + done, max_sittings=3) is None
    assert next_sitting([s2] + done, max_sittings=3) == 5       # a crash that painted nothing doesn't count
    crashes = [dict(s2, sitting=i) for i in range(1, 7)]
    assert next_sitting(crashes[:5], max_crashes=6) == 6
    assert next_sitting(crashes, max_crashes=6) is None      # the cap


def test_the_wait_after_a_crash_honors_the_providers_retry_hint():
    from r18r_open import crash_wait, retry_hint
    google = 'Quota exceeded ... \\nPlease retry in 53.812706879s.\\", ... \\"retryDelay\\": \\"53s\\"'
    assert retry_hint(google) == 53.812706879
    assert crash_wait(1, google) == 90                       # the backoff is longer
    assert crash_wait(1, "Please retry in 200s") == 215      # the hint is longer
    assert crash_wait(2, "") == 180 and crash_wait(99, "") == 1200
