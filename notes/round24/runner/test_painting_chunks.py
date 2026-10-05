"""Stopping regressions shared by the maintained round 24–26 runners."""
import importlib.util
import sys
from pathlib import Path

import pytest


@pytest.fixture(params=["round24", "round25", "round26"])
def runner(request, monkeypatch):
    directory = Path(__file__).resolve().parents[2] / request.param / "runner"
    monkeypatch.syspath_prepend(str(directory))
    for name in ("painting_chunks", "r21_chains"):
        spec = importlib.util.spec_from_file_location(name, directory / f"{name}.py")
        module = importlib.util.module_from_spec(spec)
        monkeypatch.setitem(sys.modules, name, module)
        spec.loader.exec_module(module)
    return module


def log(*chunks):
    return "".join(f"--@ chunk {i}\n{chunk}\n" for i, chunk in enumerate(chunks, 1))


@pytest.mark.parametrize("mark", [
    'bg:gesture({{316,963,0.3},{340,961,0.6},{372,962,0.2}}, {wobble=1})',
    'rg:wipe({{312,963},{376,961}}, {pressure=0.85})',
    'rg:wipe(rect(300,900,100,80), {pressure=0.6, passes=2})',
    'rg:blot(340,961, {pressure=0.8})',
    'b:spatter{at={300,400}, toward={1,0}, spread=0.5, force=0.8}',
    'k:lay({{100,200},{300,200}}, {pressure=0.5})',
    'k:scrape({{100,200},{300,200}}, {pressure=0.8})',
    'function soften() bg:gesture({{316,963,0.3},{372,962,0.2}}) end\nsoften()',
    'function lift() rg:wipe({{312,963},{376,961}}) end\nlift()',
])
def test_marks_require_another_sitting_before_no_paint_confirmation(runner, tmp_path, mark):
    painting = tmp_path / "paintings/lua/painting.lua"
    painting.parent.mkdir(parents=True)
    chunks = ['canvas{size=300}']
    painting.write_text(log(*chunks))
    before = runner.count_painting(tmp_path)
    chunks.append(mark)
    painting.write_text(log(*chunks))
    after = runner.count_painting(tmp_path)
    assert after == before + 1
    sitting = dict(sitting=3, status="completed", reviewed=True,
                   painting_before=before, painting_after=after)
    assert runner.next_step([sitting]) == ("new", 4)

    # A later call of a helper alias must also count as painting.
    chunks += ['sweep = function() ' + mark + ' end', 'again = sweep', 'again()']
    painting.write_text(log(*chunks))
    assert runner.count_painting(tmp_path) == after + 3

    before = runner.count_painting(tmp_path)
    chunks += ['print(wait(5*24*60))', 'print(drying(345,962))',
               'rg:refold(); rg:dip(0.6); bg:reload(p,0.3)',
               '-- bg:gesture(...) and rg:wipe(...) later\nprint("rg:wipe(...)")']
    painting.write_text(log(*chunks))
    after = runner.count_painting(tmp_path)
    assert after == before
    confirmation = dict(sitting=4, status="completed", reviewed=True,
                        painting_before=before, painting_after=after)
    assert runner.next_step([sitting, confirmation])[0] is None
