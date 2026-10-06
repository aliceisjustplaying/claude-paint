"""Measurement only: RGB diff and PAINTC11 field diff, following checkpoint.rs."""
import argparse
import json
import struct
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw


def checkpoint(path):
    raw = Path(path).read_bytes()
    assert raw[:8] == b"PAINTC11", raw[:8]
    offset = 8

    def u64():
        nonlocal offset
        value = struct.unpack_from("<Q", raw, offset)[0]
        offset += 8
        return value

    header_size = u64()
    offset += header_size
    frame = [u64() for _ in range(10)]
    n = frame[0] * frame[1]
    offset += 8  # scale, mm_per_unit
    if u64():
        offset += 24  # four f32 linen properties, u64 seed
    offset += 16  # surf_gen, current stroke
    if u64():
        offset += 32  # dirty rect
    fields = {}

    def field(name, count, channels=1, integer=False):
        nonlocal offset
        fields[name] = np.frombuffer(raw, dtype="<u4" if integer else "<f4", count=count * channels, offset=offset).reshape(count, channels)
        offset += 4 * count * channels

    for name, channels in [("rgb", 3), ("height_um", 1), ("film", 1), ("wet_coats", 1), ("latent", 7), ("properties", 5)]:
        field(name, n, channels)
    field("stroke", n, integer=True)
    field("touched", n, integer=True)
    offset += 16  # clock.now, mark
    if u64():
        offset += 32  # tacky rect
    if u64():
        field("drying", n, 6)
    offset += 4  # ground_um
    field("cover", n)
    assert u64() == 0, "drawing not supported by this measurement parser"
    if u64():
        offset += 4  # hand_slice
    offset += 72  # nine tally words
    fields["engine"] = np.array([u64()], dtype=np.uint64)
    field("gloss", n)
    field("absorbency", n)
    field("solvent_um", n)
    assert offset == len(raw), (offset, len(raw))
    return raw, fields


def main():
    p = argparse.ArgumentParser()
    p.add_argument("before")
    p.add_argument("after")
    p.add_argument("out")
    args = p.parse_args()
    a = Image.open(args.before + ".png").convert("RGB")
    b = Image.open(args.after + ".png").convert("RGB")
    assert a.size == b.size
    delta = np.abs(np.asarray(a).astype(np.int16) - np.asarray(b).astype(np.int16))
    max_channel = delta.max(axis=2)
    changed = int(np.count_nonzero(max_channel))
    y, x = np.unravel_index(max_channel.argmax(), max_channel.shape)
    result = dict(size=a.size, changed_pixels=changed, total_pixels=a.width*a.height,
                  mean_absolute_rgb_levels=float(delta.mean()), maximum_rgb_levels=int(delta.max()),
                  pixels_at_least_2=int(np.count_nonzero(max_channel >= 2)),
                  cross_build_policy_pass=bool(delta.max() <= 1 and changed*10000 <= a.width*a.height))
    cw, ch = min(512, a.width), min(512, a.height)
    x0, y0 = int(np.clip(x-cw//2, 0, a.width-cw)), int(np.clip(y-ch//2, 0, a.height-ch))
    crop = (x0, y0, x0+cw, y0+ch)
    result["crop_px"] = crop
    sheet = Image.new("RGB", (2*cw+16, ch+32), "#eeeeee")
    sheet.paste(a.crop(crop), (0, 32))
    sheet.paste(b.crop(crop), (cw+16, 32))
    draw = ImageDraw.Draw(sheet)
    draw.text((8, 8), "Stock", fill="black")
    draw.text((cw+24, 8), "Fix", fill="black")
    sheet.save(args.out + "-crop.png")
    Image.fromarray(np.clip(delta*16, 0, 255).astype(np.uint8)).save(args.out + "-diff-x16.png")
    ar, af = checkpoint(args.before + ".ckpt")
    br, bf = checkpoint(args.after + ".ckpt")
    result["checkpoint_bytes"] = [len(ar), len(br)]
    if len(ar) == len(br):
        result["checkpoint_changed_bytes"] = int(np.count_nonzero(np.frombuffer(ar, np.uint8) != np.frombuffer(br, np.uint8)))
    result["fields"] = {}
    for name, av in af.items():
        bv = bf[name]
        bits_differ = av.view(np.uint32) != bv.view(np.uint32)
        difference = np.abs(av.astype(np.float64) - bv.astype(np.float64))
        result["fields"][name] = dict(changed_values=int(np.count_nonzero(bits_differ)),
                                      max_absolute=float(difference.max()),
                                      mean_absolute=float(difference.mean()))
    Path(args.out + ".json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
