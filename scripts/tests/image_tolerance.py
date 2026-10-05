#!/usr/bin/env -S uv run --script
# /// script
# dependencies = ["pillow>=11,<13"]
# ///
"""Exercise the cross-build image contract through its command-line exit code."""
import subprocess
import sys
import tempfile
from pathlib import Path

from PIL import Image

tool = Path(__file__).resolve().parents[1] / "compare_images.py"
with tempfile.TemporaryDirectory() as directory:
    root = Path(directory)
    original = Image.new("RGBA", (100, 100), (100, 100, 100, 255))
    original.save(root / "expected.png")
    cases = [
        ("identical", [], 0),
        ("one pixel, all channels", [(0, (101, 99, 101, 255))], 0),
        ("two pixels", [(0, (101, 100, 100, 255)), (1, (101, 100, 100, 255))], 1),
        ("two channel levels", [(0, (102, 100, 100, 255))], 1),
        ("alpha change", [(0, (100, 100, 100, 254))], 1),
    ]
    for name, changes, expected in cases:
        actual = original.copy()
        for index, color in changes:
            actual.putpixel((index, 0), color)
        actual.save(root / "actual.png")
        result = subprocess.run([sys.executable, str(tool), str(root / "expected.png"), str(root / "actual.png")], capture_output=True, text=True)
        assert result.returncode == expected, (name, result.stdout, result.stderr)
    Image.new("RGB", (99, 100)).save(root / "actual.png")
    result = subprocess.run([sys.executable, str(tool), str(root / "expected.png"), str(root / "actual.png")], capture_output=True, text=True)
    assert result.returncode == 1 and "dimensions differ" in result.stdout
    small = Image.new("RGB", (99, 100), (100, 100, 100))
    small.save(root / "expected.png")
    small.putpixel((0, 0), (101, 100, 100))
    small.save(root / "actual.png")
    result = subprocess.run([sys.executable, str(tool), str(root / "expected.png"), str(root / "actual.png")], capture_output=True, text=True)
    assert result.returncode == 1, "the pixel budget must not round up"
    (root / "actual.png").write_text("not an image")
    result = subprocess.run([sys.executable, str(tool), str(root / "expected.png"), str(root / "actual.png")], capture_output=True, text=True)
    assert result.returncode == 2
print("image_tolerance: 8 cases passed")
