"""Measure floating twigs in a render of a bare tree on a plain ground.

usage: floating.py PNG LOG [--crop x0,y0] [--scale S] [--thr 0.3] [--out marked.png]

LOG is the easel stdout containing "LIMB i parent twig x,y,w;..." lines.
Reports: dark connected components not joined to the biggest one (the tree),
and for every limb the unpainted runs along its path (a gap anywhere but its
own tail cuts everything beyond it off).
"""
import sys, argparse
import numpy as np
from PIL import Image
from scipy import ndimage

ap = argparse.ArgumentParser()
ap.add_argument("png"); ap.add_argument("log")
ap.add_argument("--crop", default="0,0"); ap.add_argument("--scale", type=float, default=1.0)
ap.add_argument("--thr", type=float, default=0.3); ap.add_argument("--out")
a = ap.parse_args()
cx, cy = map(float, a.crop.split(","))
im = np.asarray(Image.open(a.png).convert("RGB")).astype(np.float32) / 255.0
lum = 0.2126 * im[..., 0] + 0.7152 * im[..., 1] + 0.0722 * im[..., 2]
bg = ndimage.median_filter(lum, size=max(15, int(12 * a.scale)) | 1) if False else np.percentile(lum, 75)
wood = np.percentile(lum, 1)
d = (bg - lum) / max(bg - wood, 1e-3)
dark = d > a.thr
lab, n = ndimage.label(dark, structure=np.ones((3, 3)))
sizes = ndimage.sum(dark, lab, index=np.arange(1, n + 1)) if n else np.array([])
main = int(np.argmax(sizes)) + 1 if n else 0
speck = max(3, int(round(3 * a.scale * a.scale)))
edge = set(np.unique(np.concatenate([lab[0], lab[-1], lab[:, 0], lab[:, -1]]))) if a.crop != "0,0" else set()
others = [(int(s), i + 1) for i, s in enumerate(sizes) if i + 1 != main and (i + 1) not in edge]
big = [s for s, _ in others if s > speck]
print(f"image {im.shape[1]}x{im.shape[0]} bg L {bg:.3f} wood L {wood:.3f} thr {a.thr}")
print(f"components: tree {int(sizes[main-1]) if n else 0} px; others {len(others)} "
      f"(> speck {speck}px: {len(big)}, their area {sum(big)} px, largest {max(big) if big else 0})")

# limbs
limbs = {}
for line in open(a.log):
    if not line.startswith("LIMB "):
        continue
    _, i, p, tw, pts = line.strip().split(" ", 4)
    P = [tuple(map(float, q.split(","))) for q in pts.split(";") if q]
    limbs[int(i)] = (int(p), tw == "1", P)
H, W = lum.shape


def sample(P):
    """d along the path, every 0.5 px: list of (d, t in limb length, width px, inside)."""
    out = []
    for k in range(1, len(P)):
        (x0, y0, w0), (x1, y1, w1) = P[k - 1], P[k]
        X0, Y0 = (x0 - cx) * a.scale, (y0 - cy) * a.scale
        X1, Y1 = (x1 - cx) * a.scale, (y1 - cy) * a.scale
        L = np.hypot(X1 - X0, Y1 - Y0)
        m = max(1, int(L / 0.5))
        for j in range(m):
            f = j / m
            X, Y = X0 + f * (X1 - X0), Y0 + f * (Y1 - Y0)
            ix, iy = int(X), int(Y)
            ins = 1 <= ix < W - 1 and 1 <= iy < H - 1
            v = d[iy - 1:iy + 2, ix - 1:ix + 2].max() if ins else np.nan
            out.append((v, k - 1 + f, (w0 + f * (w1 - w0)) * a.scale, ins))
    return out


kinds = {"joint": 0, "middle": 0, "tail": 0}
gap_px = {"joint": 0.0, "middle": 0.0, "tail": 0.0}
cut_twig = cut_limb = 0
checked = 0
for i, (p, tw, P) in limbs.items():
    s = sample(P)
    if not s or not all(q[3] for q in s):
        continue
    checked += 1
    ok = [q[0] > a.thr for q in s]
    n = len(ok)
    # runs of unpainted samples
    j = 0
    worst = None
    while j < n:
        if ok[j]:
            j += 1
            continue
        k = j
        while k < n and not ok[k]:
            k += 1
        run = (k - j) * 0.5
        if run >= 1.0:
            kind = "joint" if j == 0 else ("tail" if k == n else "middle")
            kinds[kind] += 1
            gap_px[kind] += run
            if kind != "tail":
                worst = kind
        j = k
    if worst:
        if tw:
            cut_twig += 1
        else:
            cut_limb += 1
print(f"limbs checked {checked}: gaps >= 1px at the joint {kinds['joint']} ({gap_px['joint']:.0f}px), "
      f"in the middle {kinds['middle']} ({gap_px['middle']:.0f}px), tail only {kinds['tail']} ({gap_px['tail']:.0f}px)")
print(f"limbs cut off from their parent by a gap: {cut_limb} model limbs, {cut_twig} fine twigs")
if a.out:
    rgb = (im * 255).astype(np.uint8).copy()
    fl = np.isin(lab, [i for s, i in others if s > speck])
    rgb[fl] = [0, 220, 255]
    Image.fromarray(rgb).save(a.out)
