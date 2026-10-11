#!/usr/bin/env -S uv run --script
# /// script
# dependencies = ["pillow>=11,<13"]
# ///
"""Exercise platform provenance and strict cross-platform image boundaries."""
import subprocess
import sys
import tempfile
from pathlib import Path

from PIL import Image
from PIL.PngImagePlugin import PngInfo

tool = Path(__file__).resolve().parents[1] / "compare_images.py"
count = 0
with tempfile.TemporaryDirectory() as directory:
    root = Path(directory)

    def check(name, expected, platforms=(), detail=""):
        global count
        result = subprocess.run([sys.executable, str(tool), str(root / "expected.png"), str(root / "actual.png"), *platforms], capture_output=True, text=True)
        assert result.returncode == expected and detail in result.stdout, (name, result.stdout, result.stderr)
        count += 1

    cross = ("--expected-platform", "macos-aarch64", "--actual-platform", "linux-x86_64")
    original = Image.new("RGBA", (100, 100), (100, 100, 100, 255))
    original.save(root / "expected.png")
    cases = [
        ("identical", [], 0),
        ("one pixel, two levels in all channels", [(0, (102, 98, 102, 255))], 0),
        ("nine pixels", [(i, (102, 100, 100, 255)) for i in range(9)], 0),
        ("exactly 0.1 percent", [(i, (101, 100, 100, 255)) for i in range(10)], 1),
        ("over 0.1 percent", [(i, (101, 100, 100, 255)) for i in range(11)], 1),
        ("three channel levels", [(0, (103, 100, 100, 255))], 1),
        ("alpha change", [(0, (100, 100, 100, 254))], 1),
    ]
    for name, changes, expected in cases:
        actual = original.copy()
        for index, color in changes:
            actual.putpixel((index, 0), color)
        actual.save(root / "actual.png")
        check(name, expected, cross)

    actual = original.copy()
    actual.putpixel((0, 0), (101, 100, 100, 255))
    actual.save(root / "actual.png")
    check("unknown origins", 1, detail="byte-exact")
    check("one unknown origin", 1, cross[:2], "byte-exact")
    for origin in ("macos-aarch64", "linux-x86_64", "current"):
        check("same platform", 1, ("--expected-platform", origin, "--actual-platform", origin), "byte-exact")
    check("different architecture", 0, ("--expected-platform", "macos-aarch64", "--actual-platform", "macos-x86_64"))

    metadata = PngInfo()
    metadata.add_text("test", "same pixels, different bytes")
    original.save(root / "actual.png", pnginfo=metadata)
    check("encoding differs with unknown origin", 1, detail="0/10000 pixels changed")
    check("encoding differs on same platform", 1, ("--expected-platform", "macos-aarch64", "--actual-platform", "macos-aarch64"))
    check("encoding differs across platforms", 0, cross)
    (root / "actual.png").write_bytes((root / "expected.png").read_bytes())
    check("unknown byte-identical images", 0)
    check("same-platform byte-identical images", 0, ("--expected-platform", "linux-x86_64", "--actual-platform", "linux-x86_64"))
    original.convert("RGB").save(root / "actual.png")
    check("implicit opaque alpha across platforms", 0, cross)
    Image.new("RGB", (99, 100)).save(root / "actual.png")
    check("dimensions", 1, cross, "dimensions differ")
    for size, expected in (((10, 100), 1), ((7, 143), 0)):
        small = Image.new("RGB", size, (100, 100, 100))
        small.save(root / "expected.png")
        small.putpixel((0, 0), (102, 100, 100))
        small.save(root / "actual.png")
        check(f"strict small pixel budget {size}", expected, cross)
    r40 = Image.new("RGB", (2400, 1500), (100, 100, 100))
    r40.save(root / "expected.png")
    for index in range(451):
        r40.putpixel((index, 0), (101, 100, 100))
    r40.save(root / "actual.png")
    check("r40-sized bounded drift cross-platform", 0, cross)
    check("r40-sized bounded drift same-platform", 1, ("--expected-platform", "macos-aarch64", "--actual-platform", "macos-aarch64"))
    Image.new("L", (100, 100)).save(root / "expected.png")
    check("unsupported mode", 2, cross)
    (root / "actual.png").write_text("not an image")
    check("invalid image", 2, cross)
print(f"image_tolerance: {count} cases passed")
