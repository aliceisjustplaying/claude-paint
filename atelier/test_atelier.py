"""The atelier's start and finish in a scratch root, with the export, the replay check and the renders faked
(the real ones build the easel: the PR's end-to-end run does those)."""
import hashlib
import json
from pathlib import Path
import plistlib
import subprocess

from PIL import Image
import pytest

import atelier as A
import run as R

OPUS = "claude-opus-5-5"
REPLY = "**Harbor at Dusk**\n\nA quay at low tide, its bollards dark against a green sky.\n"


@pytest.fixture
def world(tmp_path, monkeypatch):
    home = tmp_path / "home"
    (home / "src/a").mkdir(parents=True)
    monkeypatch.setenv("HOME", str(home))
    monkeypatch.setattr(A.Path, "home", classmethod(lambda cls: home))
    data = tmp_path / "data"
    subprocess.run(["git", "init", "-q", str(data)], check=True)
    for key, value in (("user.name", "test"), ("user.email", "test@example.invalid")):
        subprocess.run(["git", "-C", str(data), "config", key, value], check=True)
    calls, real = [], A.run
    state = {"check": 0}

    def fake(command, **options):
        command = [str(c) for c in command]
        calls.append(command)
        name = Path(command[0]).name
        done = lambda out="": subprocess.CompletedProcess(command, 0, out, "")
        if name == "export_r16_studio":
            dest = Path(command[2])
            for sub in ("bin", "notes/research", "paintings/lua"):
                (dest / sub).mkdir(parents=True)
            (dest / "bin/easel").write_bytes(b"the painter's easel")
            (dest / "notes/easel_guide.md").write_text("guide\n")
            (dest / "notes/journal.md").write_text("")
            (dest / "notes/reference-library.json").write_text("[]")
            assert options["env"]["R16_BRANCH"] == real(["git", "-C", A.REPO, "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip()
            assert "STUDIO_WIDTH" not in options["env"]
            return done()
        if name == "check_painting":
            return subprocess.CompletedProcess(command, state["check"], "log: 3 chunks\ncheck: ok\n" if not state["check"] else "check: DIFFERS\n", "")
        if name == "finish_painting":
            Image.new("RGB", (240, 150), "teal").save(command[2])
            Path(command[2]).with_suffix(".lua").write_text("the finished log")
            save = Path(command[1]).parents[2] / "out/easel/painting/live.ckpt"
            if save.exists():
                save.rename(save.with_name("live.ckpt.zst"))
            else:
                assert save.with_name("live.ckpt.zst").exists()
            return done()
        if name == "replay_easel":
            return done("/replay/easel\n")
        if command[:2] == ["/replay/easel", "render-checkpoint"]:
            assert command[2].endswith("live.ckpt.zst") and command[-2:] == ["--log", str(Path(command[2]).parents[3] / "paintings/lua/painting.lua")]
            Image.new("RGB", (240, 150), "gray").save(command[3])
            return done()
        if name == "easel" and command[1] == "close":
            return done()
        assert name in ("git", "launchctl"), command
        return real(command, **options)

    monkeypatch.setattr(A, "run", fake)
    atelier = A.Atelier(home / "src/a/stillwet-runs/atelier", home / "src/a", data)
    atelier.calls, atelier.state = calls, state
    return atelier


def painted(atelier, sid, notebook="Glazes want a set underlayer.\n"):
    """What a painter leaves: a log, a journal, its notebook, the session's save and the runner's outcome."""
    work = atelier.on_easel(sid)[-1]
    folder = Path(json.loads((work / "manifest.json").read_text())["folder"])
    (folder / "paintings/lua/painting.lua").write_text("--@ chunk 1\ncanvas{}\n")
    (folder / "notes/journal.md").write_text("- day 1, 09:00: ground\n")
    (folder / "notebook.md").write_text((folder / "notebook.md").read_text() + notebook)
    (folder / "out/easel").mkdir(parents=True)
    (folder / "out/easel/notebook-revisions.jsonl").write_text('{"added": "x"}\n')
    (folder / "out/easel/painting").mkdir()
    (folder / "out/easel/painting/live.ckpt").write_bytes(b"state")
    lane = work / "run" / A.STUDIOS[sid]["lane"]
    (lane / "p1_sittings.json").write_text(json.dumps([{"sitting": 1, "status": "completed"}, {"sitting": 2, "status": "crashed"}, {"sitting": 2, "status": "completed"}]))
    (lane / "p1_final.txt").write_text(REPLY)
    (lane / "p1_outcome.json").write_text(json.dumps({"status": "finished", "reason": "the painter is done"}))
    return work, folder


def test_briefs_are_the_texts_as_section_5_renders_them(tmp_path, monkeypatch):
    monkeypatch.setattr(A.Path, "home", classmethod(lambda cls: tmp_path))
    folder = tmp_path / "src/a/paint-studio-a1b2c3"
    later = A.render_brief(folder, later=True)
    assert later == (Path(__file__).with_name("test_data") / "later_free_brief.md").read_text()
    first = A.render_brief(folder)
    assert first == later.replace(A.text("walls_line.md") + "\n", "").replace("notebook.md and walls/list.md; ", "")
    assert "walls/" not in first.split("## What to read")[1]
    themed = A.render_brief(folder, theme="a harbor at night")
    assert themed.split("\n\n## Your studio")[0] == "# Paint a picture\n\n" + A.text("opening_theme.md").replace("{THEME}", "a harbor at night")
    assert "on this theme: a harbor at night. Within the theme" in themed.replace("\n", " ")


def test_the_house_runner_is_r40s():
    source = (Path(__file__).with_name("house") / "r21_chains.py").read_bytes()
    assert hashlib.sha256(source).hexdigest() == "109d0fcd5d8dbad35a52ab88da72993ec456331af07a9ade80bdaab5546c826d"


def test_the_studios_come_from_studios_json():
    assert list(A.STUDIOS) == [OPUS]
    assert A.STUDIOS[OPUS] == {"id": OPUS, "name": "Claude Opus 5.5", "provider": "anthropic", "model": "claude-opus-5-5",
                               "thinking": "xhigh", "harness": "pi", "lane": "OPUS"}


def test_start_prepares_the_first_painting(world):
    work = world.start(OPUS, theme="a harbor at night", prepare=True)
    assert [c[0] for c in world.calls if "launchctl" in c[0]] == []
    manifest = json.loads((work / "manifest.json").read_text())
    folder = Path(manifest["folder"])
    assert work.name == f"{OPUS}-001" and folder.parent == world.studios and folder.name.startswith("paint-studio-")
    assert {k: manifest[k] for k in ("studio", "number", "width", "box", "harness", "theme")} == {
        "studio": OPUS, "number": 1, "width": 2400, "box": "every", "harness": "pi", "theme": "a harbor at night"}
    assert manifest["easel_sha256"] == hashlib.sha256(b"the painter's easel").hexdigest()
    assert (folder / "notebook.md").read_text() == "" and list((folder / "walls").iterdir()) == []
    assert not (folder / "notes/reference-library.json").exists()
    assert (folder / "BRIEF.md").read_text() == (work / "BRIEF.md").read_text() == A.render_brief(folder, "a harbor at night")
    assert (folder / "BRIEF.md").read_text().count("~/src/a/" + folder.name) == 1
    assert json.loads((work / "run/studios.json").read_text()) == {"OPUS1": folder.name}
    assert (world.home(OPUS) / "notebook/001-before.md").read_text() == ""
    studios = json.loads((world.root / "studios.json").read_text())
    assert studios[OPUS]["slug"] == OPUS and studios[OPUS]["model"] == "claude-opus-5-5"

    agent = plistlib.loads((work / "launch.plist").read_bytes())
    assert agent["Label"] == "art.stillwet.painter.atelier-opus"
    assert agent["ProcessType"] == "Interactive" and agent["EnvironmentVariables"]["RAYON_NUM_THREADS"] == "4"
    assert agent["ProgramArguments"][:2] == ["/usr/bin/caffeinate", "-i"] and agent["ProgramArguments"][-2:] == [OPUS, "1"]
    assert agent["ProgramArguments"][agent["ProgramArguments"].index("--root") + 1] == str(world.root)
    assert agent["StandardOutPath"] == str(work / "run/runner.OPUS.out")


def second_studio(monkeypatch):
    """A second studio in the list, as studios.json could hold one: starts count every studio's paintings."""
    other = {**A.STUDIOS[OPUS], "id": "second", "lane": "SECOND"}
    monkeypatch.setitem(A.STUDIOS, "second", other)
    return "second"


def test_one_painting_per_studio_and_the_disk_floor(world, monkeypatch):
    world.start(OPUS, prepare=True)
    with pytest.raises(A.Refused, match="painting 1 hasn't hung"):
        world.start(OPUS, prepare=True)
    other = second_studio(monkeypatch)
    monkeypatch.setattr(A.shutil, "disk_usage", lambda path: type("U", (), {"free": 20 * 10**9})())
    with pytest.raises(A.Refused, match="under the 21 GB floor for 2 painting"):
        world.start(other, prepare=True)
    assert len(world.works(other)) == 0


def test_finish_hangs_the_wall_carries_the_notebook_and_commits(world):
    world.start(OPUS, prepare=True)
    with pytest.raises(A.Refused, match="isn't done"):
        world.finish(OPUS)
    work, folder = painted(world, OPUS)
    world.finish(OPUS)
    home = world.home(OPUS)
    card = json.loads((home / "cards/001.json").read_text())
    assert card == {"number": 1, "title": "Harbor at Dusk", "words": REPLY, "folder": folder.name, "finished": card["finished"]}
    assert Image.open(home / "walls/1.png").size == (1000, 625)
    assert (home / "walls/list.md").read_text() == A.text("walls_list_header.md") + "\n\n## 1. Harbor at Dusk\n\nwalls/1.png\n\n" + REPLY
    assert (home / "notebook.md").read_text() == (home / "notebook/001-after.md").read_text() == "Glazes want a set underlayer.\n"
    assert (home / "notebook/001-revisions.jsonl").exists()
    assert sorted(p.name for p in (work / "final").iterdir()) == ["color.png", "gallery.png", "relief.png"]
    assert (work / "journal.md").read_text() == "- day 1, 09:00: ground\n" and (work / "painting.lua").exists()
    manifest = json.loads((work / "manifest.json").read_text())
    assert (manifest["sittings"], manifest["canvas"], manifest["check"]) == (2, [240, 150], "check: ok")
    committed = subprocess.run(["git", "-C", str(world.data), "show", "--name-only", "--format=%s", "HEAD"], capture_output=True, text=True, check=True).stdout.split()
    assert committed[:5] == ["atelier:", OPUS, "painting", "1", "hangs"]
    assert "stillwet-runs/atelier/studios/claude-opus-5-5/cards/001.json" in committed[5:]
    assert "stillwet-runs/atelier/works/claude-opus-5-5-001/manifest.json" in committed[5:]
    assert not [f for f in committed if f.endswith(".png")]
    record = (world.data / "stillwet-runs/atelier/works/claude-opus-5-5-001/manifest.json").read_text()
    assert json.loads(record)["folder"] == "~/src/a/" + folder.name
    assert "hung as wall 1 (ok replay check)" in (world.root / "history.md").read_text()

    second = world.start(OPUS, prepare=True)
    folder2 = Path(json.loads((second / "manifest.json").read_text())["folder"])
    assert (folder2 / "notebook.md").read_text() == "Glazes want a set underlayer.\n"
    assert (folder2 / "walls/list.md").read_bytes() == (home / "walls/list.md").read_bytes()
    assert (folder2 / "walls/1.png").read_bytes() == (home / "walls/1.png").read_bytes()
    assert (folder2 / "BRIEF.md").read_text() == A.render_brief(folder2, later=True)
    assert (home / "notebook/002-before.md").read_text() == "Glazes want a set underlayer.\n"


def test_a_failed_replay_check_hangs_nothing(world):
    world.start(OPUS, prepare=True)
    painted(world, OPUS)
    world.state["check"] = 1
    with pytest.raises(A.Refused, match="didn't pass the replay check"):
        world.finish(OPUS)
    assert not (world.home(OPUS) / "cards").exists()
    assert len(world.on_easel(OPUS)) == 1


@pytest.mark.parametrize("failure", ["walls", "history", "commit", "card"])
def test_interrupted_finish_can_be_retried(world, monkeypatch, failure):
    world.start(OPUS, prepare=True)
    work, folder = painted(world, OPUS)
    home = world.home(OPUS)
    real_write = A.write

    def fail(*args):
        raise OSError("interrupted finish")

    def write(path, data):
        if path == home / ("walls/list.md" if failure == "walls" else "cards/001.json"):
            fail()
        return real_write(path, data)

    with monkeypatch.context() as patch:
        if failure in ("walls", "card"):
            patch.setattr(A, "write", write)
        else:
            patch.setattr(world, failure, fail)
        with pytest.raises(OSError, match="interrupted finish"):
            world.finish(OPUS)

    assert not (home / "cards/001.json").exists()
    assert world.on_easel(OPUS) == [work]
    with pytest.raises(A.Refused, match="painting 1 hasn't hung"):
        world.start(OPUS, prepare=True)
    assert world.finish(OPUS) == work
    card = json.loads((home / "cards/001.json").read_text())
    assert card["number"] == 1
    assert (home / "walls/list.md").read_text() == A.render_walls([card])
    assert (work / "journal.md").read_bytes() == (folder / "notes/journal.md").read_bytes()
    assert (work / "painting.lua").read_bytes() == (folder / "paintings/lua/painting.lua").read_bytes()
    assert json.loads((work / "manifest.json").read_text())["finished"] == card["finished"]
    record = Path("stillwet-runs/atelier/studios") / OPUS / "cards/001.json"
    committed = subprocess.run(["git", "-C", str(world.data), "show", f"HEAD:{record}"], capture_output=True, text=True, check=True)
    assert json.loads(committed.stdout) == card
    assert world.on_easel(OPUS) == []
    with pytest.raises(A.Refused, match="no painting on the easel"):
        world.finish(OPUS)


def test_walls_list_without_a_title():
    cards = [{"number": 1, "title": "Boot Hill, Last Light", "words": "one\n"}, {"number": 2, "title": None, "words": "two"}]
    assert A.render_walls(cards) == ("# Your walls\n\nThe paintings you finished before this one, oldest first, each with the reply you gave when you finished it. They hang at stillwet.art/atelier too.\n\n"
                                     "## 1. Boot Hill, Last Light\n\nwalls/1.png\n\none\n\n## 2\n\nwalls/2.png\n\ntwo\n")


def test_pending_card_keeps_record_redaction_and_live_reply(world):
    world.start(OPUS, prepare=True)
    work, _ = painted(world, OPUS)
    reply = REPLY + f"\nNotebook: {Path.home()}/notebook.md\n"
    (work / "run" / A.STUDIOS[OPUS]["lane"] / "p1_final.txt").write_text(reply)
    world.finish(OPUS)
    card = json.loads((world.home(OPUS) / "cards/001.json").read_text())
    assert card["words"] == reply
    record = Path("stillwet-runs/atelier/studios") / OPUS / "cards/001.json"
    committed = subprocess.run(["git", "-C", str(world.data), "show", f"HEAD:{record}"], capture_output=True, text=True, check=True)
    assert str(Path.home()) not in committed.stdout
    assert json.loads(committed.stdout)["words"] == reply.replace(str(Path.home()), "~")


def test_run_configures_the_house_loop_with_the_atelier_texts(world, monkeypatch):
    work = world.start(OPUS, prepare=True)
    monkeypatch.setenv("PAINTER_COMPACTION_NOTES", "1")
    house = R.configure(OPUS, work)
    assert "PAINTER_COMPACTION_NOTES" not in R.os.environ
    assert (house.PAINTER_MSG, house.SITTING_MESSAGE, house.CONTINUE_MESSAGE) == tuple(A.text(f"{m}_message.md") for m in ("launch", "sitting", "continue"))
    assert house.HARNESS[house.HARNESS.index("--system-prompt") + 1] == str(A.TEXTS / "system_prompt.md")
    assert house.HARNESS[house.HARNESS.index("--tools") + 1] == "paint,look,note,status,log,read,notebook"
    model = house.LANES["OPUS"]["model"]
    assert model["args"] == ["--provider", "anthropic", "--model", "claude-opus-5-5", "--thinking", "xhigh"] and model["black"]
    assert house.painter_cmd(model, house.PAINTER_MSG)[-1] == A.text("launch_message.md")
    later = house.return_message([{"status": "completed", "final": "my reply"}])
    assert later == A.text("sitting_message.md") + "\n\nYour previous sitting's reply:\nmy reply"
    assert house.painter_env(model, later) == {"PAINTER_SITTING_RECOVERY": "1"}
    assert house.RUN == work / "run"


@pytest.mark.parametrize("enabled", [True, False])
def test_run_passes_the_run_directory_and_sitting_only_when_inbox_enabled(world, monkeypatch, enabled):
    work = world.start(OPUS, prepare=True)
    house = A.house()
    calls = []
    house.run = lambda *args, **kwargs: calls.append((args, kwargs)) or 0
    monkeypatch.setattr(A, "house", lambda: house)
    monkeypatch.setenv("PAINTER_INBOX_DIR", "inherited")
    configured = R.configure(OPUS, work, inbox=enabled)
    assert "PAINTER_INBOX_DIR" not in R.os.environ
    folder = Path(json.loads((work / "manifest.json").read_text())["folder"])
    rd = work / "run" / "OPUS"
    configured.run(["pi", "--print"], folder, rd / "p1_s2_part3_final.txt", rd / "err.txt", env={"KEEP": "yes"})
    env = calls[0][1]["env"]
    assert env == ({"KEEP": "yes", "PAINTER_INBOX_DIR": str(rd.resolve()), "PAINTER_INBOX_SITTING": "2"} if enabled else {"KEEP": "yes"})
    assert (folder / ".painter-run").exists() == enabled


def test_run_keeps_old_pointer_readable_until_atomic_publication(world, monkeypatch):
    work = world.start(OPUS, prepare=True)
    house = A.house()
    house.run = lambda *args, **kwargs: 0
    monkeypatch.setattr(A, "house", lambda: house)
    configured = R.configure(OPUS, work)
    folder = Path(json.loads((work / "manifest.json").read_text())["folder"])
    pointer = folder / ".painter-run"
    pointer.write_text(str(work) + "\n")
    rd = work / "run" / "OPUS"
    replaced = []
    replace = R.os.replace

    def publish(source, target):
        assert target == pointer
        assert Path(source).parent == folder
        assert pointer.read_text() == str(work) + "\n"
        assert Path(source).read_text() == str(rd) + "\n"
        replaced.append(target)
        return replace(source, target)

    monkeypatch.setattr(R.os, "replace", publish)
    configured.run(["pi"], folder, rd / "p1_s2_final.txt", rd / "err.txt")
    assert replaced == [pointer]
    assert pointer.read_text() == str(rd) + "\n"
    assert not list(folder.glob(".painter-run-*"))


def test_run_finishes_once_the_house_loop_is_done(world, monkeypatch):
    world.start(OPUS, prepare=True)
    seen = []

    def loop(self, lane, n, folder, rd):
        seen.append((lane, n, folder, rd))
        painted(world, OPUS)
        return [{}]

    monkeypatch.setattr(R, "configure", lambda sid, work, **options: type("H", (), {"paint": loop, "log": lambda self, m: None})())
    assert R.paint(world, OPUS, 1) == 0
    assert seen[0][0] == "OPUS" and seen[0][3] == world.root / "works/claude-opus-5-5-001/run/OPUS"
    assert (world.home(OPUS) / "cards/001.json").exists()


@pytest.mark.parametrize("finished", [False, True])
def test_house_loop_marks_only_finished_paintings(world, monkeypatch, finished):
    work = world.start(OPUS, prepare=True)
    folder = Path(json.loads((work / "manifest.json").read_text())["folder"])
    house = R.configure(OPUS, work)
    monkeypatch.setattr(house, "close_easel", lambda studio: None)
    lane = work / "run/OPUS"
    final = lane / "p1_s1_final.txt"
    final.write_text("synthetic final output\n")
    sittings = ([{"sitting": 1, "status": "completed", "reviewed": True,
                  "painting_before": 0, "painting_after": 0, "final_file": str(final)}]
                if finished else [{"sitting": k, "status": "crashed", "error": "synthetic crash",
                                   "final_file": str(final)} for k in range(1, 7)])
    (lane / "p1_sittings.json").write_text(json.dumps(sittings))
    result = house.paint("OPUS", 1, folder, lane)
    assert (result is not None) == finished
    assert json.loads((lane / "p1_outcome.json").read_text())["status"] == ("finished" if finished else "crash_limit_reached")
    assert (lane / "p1.painted").exists() == finished


def test_two_finishes_at_once_hang_the_painting_once(world, monkeypatch):
    import threading
    import time
    world.start(OPUS, prepare=True)
    painted(world, OPUS)
    fake = A.run

    def slow(command, **options):
        if Path(str(command[0])).name == "check_painting":
            time.sleep(0.3)
        return fake(command, **options)

    monkeypatch.setattr(A, "run", slow)
    outcomes = []

    def finish():
        try:
            outcomes.append(world.finish(OPUS).name)
        except A.Refused as why:
            outcomes.append(str(why))

    threads = [threading.Thread(target=finish) for _ in range(2)]
    for thread in threads:
        thread.start()
    for thread in threads:
        thread.join()
    assert sorted(outcomes) == [f"{OPUS}-001", f"{OPUS}: no painting on the easel"]
    assert [p.name for p in (world.home(OPUS) / "cards").iterdir()] == ["001.json"]
    assert sorted(p.name for p in (world.home(OPUS) / "walls").glob("*.png")) == ["1.png"]


def test_two_starts_at_once_share_the_disk_floor(world, monkeypatch):
    import threading
    import time
    other = second_studio(monkeypatch)
    monkeypatch.setattr(A.shutil, "disk_usage", lambda path: type("U", (), {"free": 20 * 10**9})())
    fake = A.run

    def slow(command, **options):
        if Path(str(command[0])).name == "export_r16_studio":
            time.sleep(0.3)
        return fake(command, **options)

    monkeypatch.setattr(A, "run", slow)
    outcomes = []

    def start(sid):
        try:
            outcomes.append(world.start(sid, prepare=True).name)
        except A.Refused as why:
            outcomes.append(str(why))

    threads = [threading.Thread(target=start, args=(sid,)) for sid in (OPUS, other)]
    for thread in threads:
        thread.start()
    for thread in threads:
        thread.join()
    assert len([o for o in outcomes if "under the 21 GB floor for 2 painting" in o]) == 1
    assert len(list(world.root.glob("works/*"))) == 1
