"""Shape metrics of the wood as the brush draws it.
usage: shape.py LOG [--cr]  (--cr: strokes' own points are nodes, spline through them as the engine does)"""
import sys, math, numpy as np
log = sys.argv[1]
limbs, strokes, step = {}, [], 1.0
for line in open(log):
    if line.startswith("STEP"): step = float(line.split()[1])
    elif line.startswith("LIMB "):
        _, i, p, tw, pts = line.strip().split(" ", 4)
        limbs[int(i)] = (int(p), tw == "1", [tuple(map(float, q.split(","))) for q in pts.split(";")])
    elif line.startswith("STROKE "):
        _, li, fine, tip, pts = line.strip().split(" ", 4)
        strokes.append((int(li), fine == "1", tip == "1", [tuple(map(float, q.split(","))) for q in pts.split(";")]))

def densify(p):  # the engine's Catmull-Rom (path.rs), in units at 0.1 spacing
    n = len(p); out = []
    for i in range(n - 1):
        p0, p1, p2, p3 = p[max(i - 1, 0)], p[i], p[i + 1], p[min(i + 2, n - 1)]
        L = math.hypot(p2[0] - p1[0], p2[1] - p1[1]); k = max(1, math.ceil(L / 0.1))
        for j in range(k):
            t = j / k; t2, t3 = t * t, t * t * t
            cr = lambda a, b, c, d: 0.5 * (2 * b + (-a + c) * t + (2 * a - 5 * b + 4 * c - d) * t2 + (-a + 3 * b - 3 * c + d) * t3)
            out.append((cr(p0[0], p1[0], p2[0], p3[0]), cr(p0[1], p1[1], p2[1], p3[1])))
    out.append(p[-1][:2]); return out

def resample(p, h):
    a = np.array(p); seg = np.hypot(*np.diff(a, axis=0).T); s = np.concatenate([[0], np.cumsum(seg)])
    if s[-1] < h * 3: return None, None
    u = np.arange(0, s[-1], h)
    return np.stack([np.interp(u, s, a[:, 0]), np.interp(u, s, a[:, 1])], 1), s[-1]

res = {"limb": [], "twig": []}
for li, fine, tip, pts in strokes:
    parent, twig, lp = limbs[li]
    path = densify([q[:2] for q in pts])
    r, L = resample(path, 0.25)
    if r is None: continue
    d = np.diff(r, axis=0); th = np.unwrap(np.arctan2(d[:, 1], d[:, 0])); dth = np.abs(np.diff(th))
    mid = r[1:-1]
    nodes = np.array([q[:2] for q in lp])
    near = np.min(np.hypot(mid[:, None, 0] - nodes[None, :, 0], mid[:, None, 1] - nodes[None, :, 1]), axis=1) < 0.12 * step
    tot = dth.sum()
    # hooks: net (signed) turning over three model steps beyond 120 degrees
    sd = np.diff(th); w = max(1, int(3 * step / 0.25)); cs = np.concatenate([[0], np.cumsum(sd)])
    curl = np.abs(cs[w:] - cs[:-w]).max() if len(cs) > w else abs(cs[-1] - cs[0])
    chord = math.hypot(r[-1, 0] - r[0, 0], r[-1, 1] - r[0, 1])
    res["twig" if twig else "limb"].append((tot / L, tot and dth[near].sum() / tot, L / max(chord, 1e-6), curl > 2 * math.pi / 3, L))
for k, v in res.items():
    if not v: continue
    a = np.array(v, dtype=float)
    wts = a[:, 4]
    print(f"{k:5s} strokes {len(a):4d}: turning {np.average(a[:,0], weights=wts)*step:.2f} rad per model step, "
          f"at nodes {np.average(a[:,1], weights=wts)*100:.0f}%, sinuosity {np.average(a[:,2], weights=wts):.3f}, hooks {int(a[:,3].sum())}")
# taper at forks: the limb's width just after a fork over just before
drops = []
for i, (p, tw, lp) in limbs.items():
    if p == 0 or tw: continue
    pp = limbs[p][2]
    for k, q in enumerate(pp):
        if (q[0], q[1]) == lp[0][:2] and 0 < k < len(pp) - 1 and not limbs[p][1]:
            drops.append(pp[k + 1][2] / pp[k - 1][2])
drops = np.array(drops)
print(f"forks {len(drops)}: parent width after/before a fork: median {np.median(drops):.3f}, mean {drops.mean():.3f}, share with a drop > 10% {np.mean(drops < 0.9)*100:.0f}%")
# width along limbs between forks: runs of equal width
ev = []
for i, (p, tw, lp) in limbs.items():
    if tw or len(lp) < 4: continue
    w = [q[2] for q in lp[1:]]
    ev.append(sum(1 for a, b in zip(w, w[1:]) if abs(a - b) < 1e-4) / (len(w) - 1))
print(f"model limbs: share of internodes with no taper {np.mean(ev)*100:.0f}%")
