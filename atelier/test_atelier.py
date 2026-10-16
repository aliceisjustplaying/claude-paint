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

OPUS, FLASH = "claude-opus-5-5", "gemini-3-8-flash"
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
            assert options["env"]["R16_BRANCH"] == real(["git", "-C", A.REPO, "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip()
            assert "STUDIO_WIDTH" not in options["env"]
            return done()
        if name == "check_painting":
            return subprocess.CompletedProcess(command, state["check"], "log: 3 chunks\ncheck: ok\n" if not state["check"] else "check: DIFFERS\n", "")
        if name == "finish_painting":
            Image.new("RGB", (240, 150), "teal").save(command[2])
            Path(command[2]).with_suffix(".lua").write_text("the finished log")
            save = Path(command[1]).parents[2] / "out/easel/painting/live.ckpt"
            save.rename(save.with_name("live.ckpt.zst"))
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
    atelier = A.Atelier(home / "src/a/stillwet-runs/atelier", home / "src/a", data, home / ".pi/agent/sessions")
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
    if sid == OPUS:
        (lane / "p1_sittings.json").write_text(json.dumps([{"sitting": 1, "status": "completed"}, {"sitting": 2, "status": "crashed"}, {"sitting": 2, "status": "completed"}]))
        (lane / "p1_final.txt").write_text(REPLY)
        (lane / "p1_outcome.json").write_text(json.dumps({"status": "finished", "reason": "the painter is done"}))
    else:
        (folder / "out/easel/sittings.json").write_text(json.dumps({"sitting": 3, "complete": True}))
        feed = Path(json.loads((lane / "p1_sittings.json").read_text())[0]["sessions"][0])
        feed.parent.mkdir(parents=True, exist_ok=True)
        feed.write_text(json.dumps({"type": "message", "message": {"role": "assistant", "stopReason": "stop", "content": [{"type": "text", "text": REPLY.strip()}]}}) + "\n")
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


def test_the_house_runner_is_r40s_as_stil_89_ran_it():
    source = (Path(__file__).with_name("house") / "r21_chains.py").read_bytes()
    assert hashlib.sha256(source).hexdigest() == "24e8f16af8a9214a410fa7673adfd0bac576d59df424571030f1de3ee98f08b9"


def test_start_prepares_both_first_paintings(world):
    opus = world.start(OPUS, prepare=True)
    flash = world.start(FLASH, theme="a harbor at night", prepare=True)
    assert [c[0] for c in world.calls if "launchctl" in c[0]] == []
    for work, sid, lane in ((opus, OPUS, "OPUS"), (flash, FLASH, "FLASH")):
        manifest = json.loads((work / "manifest.json").read_text())
        folder = Path(manifest["folder"])
        assert work.name == f"{sid}-001" and folder.parent == world.studios and folder.name.startswith("paint-studio-")
        assert {k: manifest[k] for k in ("studio", "number", "width", "box", "harness")} == {"studio": sid, "number": 1, "width": 2400, "box": "every", "harness": A.STUDIOS[sid]["harness"]}
        assert manifest["easel_sha256"] == hashlib.sha256(b"the painter's easel").hexdigest()
        assert (folder / "notebook.md").read_text() == "" and list((folder / "walls").iterdir()) == []
        assert (folder / "BRIEF.md").read_text() == (work / "BRIEF.md").read_text() == A.render_brief(folder, manifest["theme"])
        assert (folder / "BRIEF.md").read_text().count("~/src/a/" + folder.name) == 1
        assert json.loads((work / "run/studios.json").read_text()) == {f"{lane}1": folder.name}
        assert (world.home(sid) / "notebook/001-before.md").read_text() == ""
    assert json.loads((opus / "manifest.json").read_text())["theme"] is None
    studios = json.loads((world.root / "studios.json").read_text())
    assert studios[OPUS]["slug"] == OPUS and studios[FLASH]["model"] == "gemini-3.8-flash"

    agent = plistlib.loads((opus / "launch.plist").read_bytes())
    assert agent["ProcessType"] == "Interactive" and agent["EnvironmentVariables"]["RAYON_NUM_THREADS"] == "4"
    assert agent["ProgramArguments"][:2] == ["/usr/bin/caffeinate", "-i"] and agent["ProgramArguments"][-2:] == [OPUS, "1"]
    assert agent["ProgramArguments"][agent["ProgramArguments"].index("--root") + 1] == str(world.root)

    folder = Path(json.loads((flash / "manifest.json").read_text())["folder"])
    assert (folder / ".agents/agents/easel-painter.md").read_bytes() == (A.TEXTS / "easel-painter.md").read_bytes()
    assert (folder / "bin/mcp/sitting-message.txt").read_bytes() == (A.TEXTS / "sitting_message.md").read_bytes()
    assert (folder / "bin/mcp/completion-message.txt").read_bytes() == (A.TEXTS / "completion_message.md").read_bytes()
    assert not (folder / "LAUNCH.md").exists()
    easel = json.loads((folder / ".agents/mcp_config.json").read_text())["mcpServers"]["easel"]
    assert easel["env"] == {"RAYON_NUM_THREADS": "4"}
    assert easel["args"][-1] == "--notebook" and ["--break-seconds", "0"] == easel["args"][easel["args"].index("--break-seconds"):][:2]
    record = json.loads((flash / "run/FLASH/p1_sittings.json").read_text())
    assert record[0]["status"] == "running" and record[0]["message"] == A.text("launch_message.md") and "start" not in record[0]
    launch = (flash / "HAND-LAUNCH.md").read_text()
    assert "> " + A.text("hello.md") + "\n" in launch and "> " + A.text("launch_message.md") + "\n" in launch
    assert "@" not in launch.replace("mcp_easel", "")
    assert "finish gemini-3-8-flash" in launch and str(world.root) in launch


def test_one_painting_per_studio_and_the_disk_floor(world, monkeypatch):
    world.start(OPUS, prepare=True)
    with pytest.raises(A.Refused, match="painting 1 hasn't hung"):
        world.start(OPUS, prepare=True)
    monkeypatch.setattr(A.shutil, "disk_usage", lambda path: type("U", (), {"free": 20 * 10**9})())
    with pytest.raises(A.Refused, match="under the 21 GB floor for 2 painting"):
        world.start(FLASH, prepare=True)
    assert len(world.works(FLASH)) == 0


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


def test_flash_finishes_from_the_feed_and_records_the_outcome(world):
    world.start(FLASH, prepare=True)
    work, folder = painted(world, FLASH)
    world.finish(FLASH)
    lane = work / "run/FLASH"
    assert (lane / "p1_final.txt").read_text() == REPLY.strip()
    assert json.loads((lane / "p1_outcome.json").read_text())["status"] == "finished"
    assert json.loads((lane / "p1_sittings.json").read_text())[-1]["status"] == "completed"
    assert json.loads((work / "manifest.json").read_text())["sittings"] == 3
    assert json.loads((world.home(FLASH) / "cards/001.json").read_text())["title"] == "Harbor at Dusk"


def test_walls_list_without_a_title():
    cards = [{"number": 1, "title": "Boot Hill, Last Light", "words": "one\n"}, {"number": 2, "title": None, "words": "two"}]
    assert A.render_walls(cards) == ("# Your walls\n\nThe paintings you finished before this one, oldest first. They hang at stillwet.art/atelier too.\n\n"
                                     "## 1. Boot Hill, Last Light\n\nwalls/1.png\n\none\n\n## 2\n\nwalls/2.png\n\ntwo\n")


def test_run_configures_the_house_loop_with_the_atelier_texts(world, monkeypatch):
    work = world.start(OPUS, prepare=True)
    monkeypatch.setenv("PAINTER_COMPACTION_NOTES", "1")
    house = R.configure(work)
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


def test_run_finishes_once_the_house_loop_is_done(world, monkeypatch):
    world.start(OPUS, prepare=True)
    seen = []

    def loop(self, lane, n, folder, rd):
        seen.append((lane, n, folder, rd))
        painted(world, OPUS)
        return [{}]

    monkeypatch.setattr(R, "configure", lambda work, real=R.configure: type("H", (), {"paint": loop, "log": lambda self, m: None})())
    assert R.paint(world, 1) == 0
    assert seen[0][0] == "OPUS" and seen[0][3] == world.root / "works/claude-opus-5-5-001/run/OPUS"
    assert (world.home(OPUS) / "cards/001.json").exists()
