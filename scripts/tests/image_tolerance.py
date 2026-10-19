#!/usr/bin/env -S uv run --script
# /// script
# dependencies = ["pillow>=11,<13"]
# ///
"""Exercise decoded RGBA tolerance independent of platform provenance."""
import subprocess
import sys
import tempfile
from pathlib import Path

from PIL import Image, ImageDraw
from PIL.PngImagePlugin import PngInfo

tool = Path(__file__).resolve().parents[1] / "compare_images.py"
count = 0
with tempfile.TemporaryDirectory() as directory:
    root = Path(directory)

    def check(name, expected, platforms=(), detail=""):
        global count
        result = subprocess.run([sys.executable, str(tool), str(root / "expected.png"), str(root / "actual.png"), *platforms], capture_output=True, text=True)
        assert result.returncode == expected and detail in result.stdout, (name, result.stdout, result.stderr)
        if expected == 2:
            assert "image comparison:" in result.stderr, (name, result.stderr)
        count += 1

    origins = [
        (),
        ("--expected-platform", "macos-aarch64"),
        ("--actual-platform", "linux-x86_64"),
        ("--expected-platform", "macos-aarch64", "--actual-platform", "linux-x86_64"),
        ("--expected-platform", "macos-aarch64", "--actual-platform", "macos-x86_64"),
    ]
    for origin in ("macos-aarch64", "linux-x86_64", "current"):
        origins.append(("--expected-platform", origin, "--actual-platform", origin))

    original = Image.new("RGBA", (100, 100), (100, 100, 100, 100))
    original.save(root / "expected.png", compress_level=0)
    metadata = PngInfo()
    metadata.add_text("test", "same pixels, different bytes")
    original.save(root / "actual.png", compress_level=9, pnginfo=metadata)
    assert (root / "expected.png").read_bytes() != (root / "actual.png").read_bytes()
    for platforms in origins:
        check("identical decoded pixels with different encoding", 0, platforms, "0/10000 pixels changed; max RGB delta 0/255; max alpha delta 0/255")

    for size in ((1, 1), (10, 10), (100, 100)):
        original = Image.new("RGBA", size, (100, 100, 100, 100))
        original.save(root / "expected.png")
        pixels = size[0] * size[1]
        for levels in (1, 2):
            Image.new("RGBA", size, (100 + levels, 100 - levels, 100 + levels, 100 - levels)).save(root / "actual.png")
            for platforms in origins:
                check(f"all {pixels} pixels drift by {levels}", 0, platforms, f"{pixels}/{pixels} pixels changed; max RGB delta {levels}/255; max alpha delta {levels}/255")

    original = Image.new("RGBA", (100, 100), (100, 100, 100, 100))
    original.save(root / "expected.png")
    for channel in range(4):
        for levels, expected in ((1, 0), (2, 0), (3, 1), (-3, 1)):
            actual = original.copy()
            color = [100, 100, 100, 100]
            color[channel] += levels
            actual.putpixel((99, 99), tuple(color))
            actual.save(root / "actual.png")
            for platforms in origins:
                check(f"single pixel channel {channel} drift {levels}", expected, platforms, "1/10000 pixels changed")
            check(f"channel {channel} diagnostic", expected, detail=f"max {'alpha' if channel == 3 else 'RGB'} delta {abs(levels)}/255")

    original = Image.new("RGBA", (100, 100), (100, 100, 100, 255))
    original.save(root / "expected.png")
    original.convert("RGB").save(root / "actual.png")
    for platforms in origins:
        check("implicit opaque alpha", 0, platforms)
    for size in ((99, 100), (100, 99), (50, 200)):
        Image.new("RGB", size).save(root / "actual.png")
        check(f"dimensions {size}", 1, detail="dimensions differ")

    stroke = Image.new("RGBA", (100, 100), (255, 255, 255, 255))
    ImageDraw.Draw(stroke).line((10, 50, 90, 50), fill=(180, 40, 20, 255), width=5)
    stroke.save(root / "expected.png")
    wrong_color = stroke.copy()
    ImageDraw.Draw(wrong_color).line((10, 50, 90, 50), fill=(20, 40, 180, 255), width=5)
    missing_stroke = Image.new("RGBA", stroke.size, (255, 255, 255, 255))
    black_rectangle = stroke.copy()
    ImageDraw.Draw(black_rectangle).rectangle((20, 20, 40, 40), fill=(0, 0, 0, 255))
    for name, actual in (("wrong color", wrong_color), ("missing stroke", missing_stroke), ("black rectangle", black_rectangle)):
        actual.save(root / "actual.png")
        for platforms in origins:
            check(name, 1, platforms)

    stroke.save(root / "actual.png")
    Image.new("L", (100, 100)).save(root / "expected.png")
    check("unsupported mode", 2)
    (root / "expected.png").write_text("not an image")
    check("invalid expected image", 2)
    stroke.save(root / "expected.png")
    (root / "actual.png").write_text("not an image")
    check("invalid actual image", 2)
    (root / "actual.png").unlink()
    check("missing actual image", 2)
    (root / "expected.png").unlink()
    check("missing expected image", 2)

    result = subprocess.run([sys.executable, str(tool), "--print-platform"], capture_output=True, text=True)
    assert result.returncode == 0 and result.stdout.strip() in ("unknown", "macos-aarch64", "macos-x86_64", "linux-aarch64", "linux-x86_64", "windows-aarch64", "windows-x86_64"), (result.stdout, result.stderr)
    count += 1
print(f"image_tolerance: {count} cases passed")
