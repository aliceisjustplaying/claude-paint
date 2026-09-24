"""The Chapel Gable at Evening: a painting in the manner of C. D. Friedrich.

Painted pixel by pixel with numpy: a primed ground, a graphite underdrawing,
then brush strokes (bristles, load, dry-out, pooling in the ground texture),
stippled sky, glazes and an aged varnish. No reference images.

    uv run paint.py --width 1000 --out out_1000.png
"""

import argparse
import math
import time

import numpy as np
from PIL import Image

ASPECT = 0.72                 # height / width; all geometry is in width units
HOR = 0.58 * ASPECT           # the ruled horizon
AXIS = 0.5


# ---------------------------------------------------------------- noise ----

def _hash(ix, iy, seed):
    h = (ix.astype(np.int64) * 374761393 + iy.astype(np.int64) * 668265263
         + seed * 1013904223) & 0xFFFFFFFF
    h = ((h ^ (h >> 13)) * 1274126177) & 0xFFFFFFFF
    h = h ^ (h >> 16)
    return (h & 0xFFFFFF).astype(np.float32) / np.float32(0x1000000)


def vnoise(x, y, seed):
    ix = np.floor(x)
    iy = np.floor(y)
    fx = (x - ix).astype(np.float32)
    fy = (y - iy).astype(np.float32)
    fx = fx * fx * (3 - 2 * fx)
    fy = fy * fy * (3 - 2 * fy)
    ix = ix.astype(np.int64)
    iy = iy.astype(np.int64)
    a = _hash(ix, iy, seed)
    b = _hash(ix + 1, iy, seed)
    c = _hash(ix, iy + 1, seed)
    d = _hash(ix + 1, iy + 1, seed)
    return (a + (b - a) * fx) + ((c + (d - c) * fx) - (a + (b - a) * fx)) * fy


def fbm(x, y, seed, octaves=4, gain=0.5):
    tot = np.zeros(np.broadcast(x, y).shape, np.float32)
    amp, norm, f = 1.0, 0.0, 1.0
    for o in range(octaves):
        tot += amp * vnoise(x * f + o * 17.3, y * f - o * 9.1, seed + o * 101)
        norm += amp
        amp *= gain
        f *= 2.03
    return tot / norm


def smooth(e0, e1, x):
    t = np.clip((x - e0) / (e1 - e0), 0.0, 1.0)
    return t * t * (3 - 2 * t)


def box_blur(img, r):
    """Separable box blur via cumulative sums (r in pixels, int)."""
    if r < 1:
        return img
    out = img.astype(np.float32)
    for axis in (0, 1):
        pad = [(0, 0)] * out.ndim
        pad[axis] = (r + 1, r)
        p = np.pad(out, pad, mode="edge")
        c = np.cumsum(p, axis=axis, dtype=np.float64)
        n = out.shape[axis]
        hi = np.take(c, np.arange(2 * r + 1, 2 * r + 1 + n), axis=axis)
        lo = np.take(c, np.arange(0, n), axis=axis)
        out = ((hi - lo) / (2 * r + 1)).astype(np.float32)
    return out


def rgb(h):
    h = h.lstrip("#")
    return np.array([int(h[i:i + 2], 16) / 255 for i in (0, 2, 4)], np.float32)


def ramp(t, stops):
    """Piecewise-linear color ramp; stops = [(t, color), ...]."""
    t = np.asarray(t, np.float32)
    out = np.zeros(t.shape + (3,), np.float32)
    ts = [s[0] for s in stops]
    cs = [s[1] for s in stops]
    for i in range(3):
        out[..., i] = np.interp(t, ts, [c[i] for c in cs])
    return out


# ------------------------------------------------------------- geometry ----

def ridge_y(x):
    """The near ledge the gable stands on; beyond it the land falls away."""
    x = np.asarray(x, np.float32)
    base = 0.80 * ASPECT + 0.018 * (np.abs(x - AXIS) / 0.5) ** 1.6
    return base + 0.004 * (vnoise(x * 9, 0.0 * x, 5) - 0.5) + 0.002 * (vnoise(x * 40, 0.0 * x, 6) - 0.5)


# gable top outline, left to right (x, y); symmetric plan, broken in the parts
GABLE = [
    (0.215, 0.705 * ASPECT), (0.222, 0.655 * ASPECT), (0.236, 0.648 * ASPECT),
    (0.240, 0.600 * ASPECT), (0.258, 0.592 * ASPECT), (0.262, 0.520 * ASPECT),
    (0.284, 0.512 * ASPECT), (0.286, 0.372 * ASPECT), (0.300, 0.366 * ASPECT),
    (0.500, 0.128 * ASPECT),
    (0.556, 0.196 * ASPECT), (0.566, 0.214 * ASPECT), (0.578, 0.208 * ASPECT),
    (0.598, 0.248 * ASPECT), (0.612, 0.240 * ASPECT), (0.700, 0.366 * ASPECT),
    (0.714, 0.372 * ASPECT), (0.717, 0.430 * ASPECT), (0.728, 0.452 * ASPECT),
    (0.731, 0.520 * ASPECT), (0.742, 0.548 * ASPECT), (0.748, 0.600 * ASPECT),
    (0.762, 0.618 * ASPECT), (0.766, 0.676 * ASPECT), (0.781, 0.694 * ASPECT),
    (0.786, 0.740 * ASPECT),
]
ARCH_HALF = 0.098           # half span of the pointed arch
ARCH_SPRING = 0.545 * ASPECT
ARCH_RISE = 2 * ARCH_HALF * math.sin(math.pi / 3)
OCULUS = (AXIS, 0.228 * ASPECT, 0.024)


def gable_top(x):
    gx = np.array([p[0] for p in GABLE])
    gy = np.array([p[1] for p in GABLE])
    top = np.interp(x, gx, gy, left=9.0, right=9.0)
    # the ruled slopes stay true; the broken parts are ragged, stone by stone
    x = np.asarray(x, np.float32)
    ruled = ((x > 0.302) & (x < 0.553)) | ((x > 0.614) & (x < 0.698))
    rag = 0.006 * (vnoise(x * 70, 0 * x, 13) - 0.5) + 0.003 * (vnoise(x * 260, 0 * x, 14) - 0.5)
    return np.where(ruled, top, top + np.where(top < 5, rag, 0))


def arch_inside(x, y):
    """Pointed (equilateral) arch opening, from the ground up."""
    dx = x - AXIS
    below = (np.abs(dx) < ARCH_HALF) & (y >= ARCH_SPRING)
    r = 2 * ARCH_HALF
    c1 = (dx + ARCH_HALF) ** 2 + (y - ARCH_SPRING) ** 2 < r * r
    c2 = (dx - ARCH_HALF) ** 2 + (y - ARCH_SPRING) ** 2 < r * r
    above = c1 & c2 & (y < ARCH_SPRING)
    return below | above


def oculus_inside(x, y):
    cx, cy, r = OCULUS
    return (x - cx) ** 2 + (y - cy) ** 2 < r * r


def wall_inside(x, y):
    wob = (0.0010 * (vnoise(x * 420, y * 420, 11) - 0.5)
           + 0.0012 * (vnoise(x * 90, y * 90, 12) - 0.5))
    top = gable_top(x + wob)
    inside = (y + wob >= top) & (y <= ridge_y(x) + 0.012)
    return inside & ~arch_inside(x + wob, y) & ~oculus_inside(x + wob, y + wob)


# figure outline in metres, (x right, y up) from the feet; seen from behind
FIG_M = 0.0905              # width units per metre
FIGURE = [
    (-0.09, 0.0), (-0.11, 0.06), (-0.10, 0.34), (-0.26, 0.40), (-0.27, 0.80),
    (-0.25, 1.10), (-0.235, 1.30), (-0.22, 1.40), (-0.16, 1.47), (-0.075, 1.52),
    (0.075, 1.52), (0.17, 1.47), (0.225, 1.40), (0.24, 1.30), (0.255, 1.10),
    (0.275, 0.80), (0.265, 0.40), (0.10, 0.34), (0.115, 0.06), (0.095, 0.0),
    (0.02, 0.0), (0.012, 0.30), (-0.012, 0.30), (-0.02, 0.0),
]


def figure_feet():
    return AXIS + 0.004, float(ridge_y(np.float32(AXIS + 0.004))) - 0.002


def poly_inside(px, py, poly):
    inside = np.zeros(np.broadcast(px, py).shape, bool)
    n = len(poly)
    for i in range(n):
        x1, y1 = poly[i]
        x2, y2 = poly[(i + 1) % n]
        cond = (y1 > py) != (y2 > py)
        with np.errstate(divide="ignore", invalid="ignore"):
            xin = (x2 - x1) * (py - y1) / (y2 - y1 + 1e-12) + x1
        inside ^= cond & (px < xin)
    return inside


def figure_parts(x, y):
    fx, fy = figure_feet()
    mx = (x - fx) / FIG_M
    my = (fy - y) / FIG_M
    wob = 0.012 * (vnoise(mx * 30, my * 30, 21) - 0.5)
    body = poly_inside(mx + wob, my, FIGURE)
    # head bowed a little, and a soft beret tipped to one side
    head = ((mx - 0.005) / 0.10) ** 2 + ((my - 1.615) / 0.12) ** 2 < 1
    beret = ((mx + 0.012) / 0.122) ** 2 + ((my - 1.70 - 0.08 * (mx + 0.012)) / 0.052) ** 2 < 1
    return body, head | beret


def figure_inside(x, y):
    body, head = figure_parts(x, y)
    return body | head


# ---------------------------------------------------------------- design ---

def sky_design(x, y):
    """The look of the sky: bands from a pale lemon horizon to dusk blue."""
    d = np.clip((HOR - y) / HOR, 0, 1)
    warp = 0.03 * (fbm(x * 2.2, y * 9, 31, 4) - 0.5)
    d2 = np.clip(d + warp, 0, 1)
    col = ramp(d2, [
        (0.00, rgb("#f1e3b4")), (0.035, rgb("#ecd6a6")), (0.09, rgb("#dfb996")),
        (0.17, rgb("#c49a8f")), (0.28, rgb("#9c8791")), (0.45, rgb("#6f6d82")),
        (0.70, rgb("#4a4f66")), (1.00, rgb("#2f3548")),
    ])
    # light gathers on the axis, low; no sun disk
    glow = np.exp(-((x - AXIS) / 0.34) ** 2) * np.exp(-d / 0.10)
    col = col + glow[..., None] * (rgb("#fff4cf") - col) * 0.55

    # cloud: one long dark band low, thin parallel strata, a pale wedge above
    n1 = fbm(x * 3.0, y * 22, 41, 5)
    band1 = smooth(0.02, 0.0, np.abs(y - (0.475 * ASPECT + 0.012 * (n1 - 0.5))) - 0.010 - 0.012 * n1)
    band1 *= smooth(0.02, 0.18, x) * smooth(0.99, 0.80, x) * (0.55 + 0.6 * fbm(x * 6, y * 30, 43, 3))
    n2 = fbm(x * 4.0, y * 40, 47, 5)
    strata = sum(smooth(0.006, 0.0, np.abs(y - (yy + 0.006 * (n2 - 0.5))) - 0.002 - 0.004 * n2)
                 * (0.3 + 0.7 * smooth(0.45, 0.75, fbm(x * 5 + i, y * 20, 50 + i, 3)))
                 for i, yy in enumerate([0.515 * ASPECT, 0.43 * ASPECT, 0.395 * ASPECT]))
    # the wedge: a pale bank cutting down from upper left
    wy = 0.10 * ASPECT + 0.34 * x * ASPECT
    wn = fbm(x * 2.5, y * 10, 61, 5)
    wedge = smooth(0.05, 0.0, np.abs(y - wy) - 0.035 - 0.035 * wn) * (0.35 + 0.65 * wn)
    wedge *= smooth(1.0, 0.55, x)
    dark = np.clip(band1 * 0.9 + strata * 0.6, 0, 1)
    dcol = ramp(d, [(0.0, rgb("#8f7a80")), (0.15, rgb("#6d6072")), (0.4, rgb("#4e4b5e")), (1, rgb("#2b2f40"))])
    col = col + dark[..., None] * (dcol - col) * 0.85
    # warm lit undersides of the dark band
    rim = smooth(0.012, 0.0, np.abs(y - (0.475 * ASPECT + 0.018 + 0.01 * (n1 - 0.5)))) * band1
    col = col + rim[..., None] * (rgb("#e0a488") - col) * 0.35
    wcol = ramp(d, [(0.3, rgb("#a79aa2")), (0.7, rgb("#6d6f84")), (1, rgb("#4d5268"))])
    col = col + np.clip(wedge, 0, 1)[..., None] * (wcol - col) * 0.55
    return col


def sea_design(x, y):
    ry = ridge_y(x)
    e = np.clip((y - HOR) / (ry - HOR), 0, 1)
    col = ramp(e, [
        (0.00, rgb("#d9c8a8")), (0.02, rgb("#b8a79c")), (0.08, rgb("#8e8590")),
        (0.30, rgb("#5f6070")), (0.65, rgb("#4f5263")), (1.0, rgb("#5e5e68")),
    ])
    # the glow echoed, fainter and broken, on the axis
    streak = fbm(x * 8, y * 160, 71, 3)
    refl = np.exp(-((x - AXIS) / (0.10 + 0.25 * e)) ** 2) * (1 - e) ** 1.5
    refl *= 0.35 + 0.65 * smooth(0.4, 0.7, streak)
    col = col + refl[..., None] * (rgb("#d8c4a2") - col) * 0.6
    # calm swell as long faint horizontal lines
    # calm water: long lighter lanes that echo the glow, fainter nearer
    lanes = smooth(0.55, 0.78, fbm(x * 2.2, y * 140, 73, 4)) * (1 - 0.6 * e)
    col = col + lanes[..., None] * (rgb("#b3a7a2") - col) * 0.45
    dark = smooth(0.6, 0.8, fbm(x * 1.5, y * 90, 74, 3)) * e
    col = col + dark[..., None] * (rgb("#3f4250") - col) * 0.35
    return col


def land_design(x, y):
    ry = ridge_y(x)
    k = np.clip((y - ry) / 0.06, 0, 1)
    col = ramp(k, [(0, rgb("#3f3a33")), (0.08, rgb("#2d2822")), (0.5, rgb("#231e19")), (1, rgb("#1b1714"))])
    n = fbm(x * 14, y * 20, 81, 4)
    col = col * (0.9 + 0.18 * n)[..., None]
    return col


def _courses():
    r = np.random.default_rng(7)
    b = [0.83 * ASPECT]
    while b[-1] > 0.08 * ASPECT:
        b.append(b[-1] - r.uniform(0.011, 0.024) - (0.005 if len(b) < 4 else 0.0))
    return np.array(b[::-1])


COURSES = _courses()


def arch_gap(x, y):
    """Distance outward from the arch opening's edge (negative inside)."""
    dx = x - AXIS
    r = 2 * ARCH_HALF
    d1 = np.sqrt((dx + ARCH_HALF) ** 2 + (y - ARCH_SPRING) ** 2) - r
    d2 = np.sqrt((dx - ARCH_HALF) ** 2 + (y - ARCH_SPRING) ** 2) - r
    above = np.maximum(d1, d2)
    below = np.abs(dx) - ARCH_HALF
    return np.where(y < ARCH_SPRING, above, below)


def masonry(x, y):
    """Stone tone (0..1), joint strength and a lit upper edge, per pixel."""
    ci = np.clip(np.searchsorted(COURSES, y), 1, len(COURSES) - 1)
    y0, y1 = COURSES[ci - 1], COURSES[ci]
    ciu = ci.astype(np.int64)
    sw = 0.022 + 0.026 * _hash(ciu, ciu * 0 + 3, 900)
    off = _hash(ciu, ciu * 0 + 5, 901)
    sx = x / sw + off * 7
    sx = sx + 0.6 * (vnoise(sx * 0.7, ciu * 3.1, 902) - 0.5)
    si = np.floor(sx)
    fx = sx - si
    tone = _hash(si.astype(np.int64), ciu, 903)
    jd = np.minimum(np.minimum(fx, 1 - fx) * sw, np.minimum(y - y0, y1 - y))
    # voussoirs round the arch, and a ring of stones round the oculus
    g = arch_gap(x, y)
    ring = (g > 0) & (g < 0.03) & (y < ARCH_SPRING + 0.002)
    side = np.where(x < AXIS, -1.0, 1.0)
    cx = AXIS - side * ARCH_HALF
    ang = np.arctan2(ARCH_SPRING - y, side * (x - cx))
    k = ang / (math.pi / 3) * 9.0
    kf = k - np.floor(k)
    rjd = np.minimum(np.minimum(kf, 1 - kf) * 0.03, 0.03 - g)
    vt = _hash(np.floor(k).astype(np.int64) + (side > 0) * 50, ciu * 0, 905)
    ocx, ocy, orr = OCULUS
    od = np.sqrt((x - ocx) ** 2 + (y - ocy) ** 2) - orr
    oring = (od > 0) & (od < 0.016)
    oa = (np.arctan2(y - ocy, x - ocx) / 6.2832 + 0.5) * 12
    of = oa - np.floor(oa)
    ojd = np.minimum(np.minimum(of, 1 - of) * 0.02, 0.016 - od)
    ot = _hash(np.floor(oa).astype(np.int64), ciu * 0, 906)
    tone = np.where(ring, vt, np.where(oring, ot, tone))
    jd = np.where(ring, rjd, np.where(oring, ojd, jd))
    jn = 0.0005 * (vnoise(x * 500, y * 500, 907) - 0.5)
    joint = smooth(0.0011, 0.0002, jd + jn)
    # weathered: long stretches of joint are lost, pointing crumbled
    joint = joint * smooth(0.3, 0.65, vnoise(x * 120, y * 120, 909)) * (0.5 + 0.5 * vnoise(x * 40, y * 40, 910))
    lit = smooth(0.0035, 0.001, y - y0) * (~ring & ~oring)
    return tone, joint, lit


def wall_design(x, y):
    n = fbm(x * 30, y * 30, 91, 4)
    top = (y - 0.12 * ASPECT) / (0.68 * ASPECT)
    col = ramp(np.clip(top, 0, 1), [(0, rgb("#3d3837")), (0.4, rgb("#2f2a28")), (1, rgb("#211c19"))])
    tone, joint, lit = masonry(x, y)
    warm = rgb("#3a2e25")
    col = col * (0.84 + 0.3 * tone)[..., None]
    col = col + (warm - col) * (0.35 * _hash((tone * 1e4).astype(np.int64), 0 * x.astype(np.int64), 908))[..., None]
    col = col * (0.9 + 0.2 * n)[..., None]
    # damp and lichen: broad patches, darker low down, a cold gray-green here and there
    damp = smooth(0.45, 0.8, fbm(x * 7, y * 5, 912, 4)) * smooth(0.45, 0.8, y / ASPECT)
    col = col * (1 - 0.25 * damp)[..., None]
    lich = smooth(0.6, 0.85, fbm(x * 12, y * 12, 913, 4))
    col = col + lich[..., None] * (rgb("#45463d") - col) * 0.35
    col = col * (1 + 0.07 * lit)[..., None]
    col = col * (1 - 0.3 * joint)[..., None]
    return col * 0.88


# ----------------------------------------------------------------- canvas --

class Canvas:
    def __init__(self, width):
        self.W = width
        self.H = int(round(width * ASPECT))
        self.s = float(width)
        ys, xs = np.mgrid[0:self.H, 0:self.W].astype(np.float32)
        self.X = (xs + 0.5) / self.s
        self.Y = (ys + 0.5) / self.s
        self.rgb = np.zeros((self.H, self.W, 3), np.float32)
        self.height = np.zeros((self.H, self.W), np.float32)
        self.tex = np.zeros((self.H, self.W), np.float32)
        self.design = np.zeros((self.H, self.W, 3), np.float32)
        self.bg = None          # design without the wall and figure
        self.masks = {}

    # ---- one brush stroke -------------------------------------------------
    def stroke(self, rng, x0, y0, x1, y1, w, col, alpha, *, bend=0.0, dry=0.2,
               soft=0.25, bristle=0.45, follow=0.5, mask=None, thick=1.0,
               pool=0.3, taper=0.0, jitter=0.025, wobble=0.12, src=None):
        s = self.s
        X0, Y0, X1, Y1 = x0 * s, y0 * s, x1 * s, y1 * s
        L = max(math.hypot(X1 - X0, Y1 - Y0), 1.0)
        dx, dy = (X1 - X0) / L, (Y1 - Y0) / L
        hw = max(w * s / 2, 0.45)
        bp = bend * L
        pad = hw * (1 + wobble) + abs(bp) + 2
        xa = int(max(math.floor(min(X0, X1) - pad), 0))
        xb = int(min(math.ceil(max(X0, X1) + pad), self.W))
        ya = int(max(math.floor(min(Y0, Y1) - pad), 0))
        yb = int(min(math.ceil(max(Y0, Y1) + pad), self.H))
        # draw per-stroke randomness before any early return: determinism
        K = int(np.clip(w / 0.0011, 5, 90))
        bv = rng.random(K).astype(np.float32)
        bend_k = (1 - dry * rng.random(K) ** 1.3).astype(np.float32)
        bshade = rng.normal(0, 0.035, K).astype(np.float32)
        ph = rng.random(3) * 6.283
        fr = rng.uniform(1.0, 3.0, 2)
        cj = rng.normal(0, jitter * 0.25, 3).astype(np.float32)
        lum_j = np.float32(rng.normal(0, jitter))
        thr_n = rng.random()
        if xa >= xb or ya >= yb:
            return
        ys = np.arange(ya, yb, dtype=np.float32)[:, None] + 0.5 - Y0
        xs = np.arange(xa, xb, dtype=np.float32)[None, :] + 0.5 - X0
        u = xs * dx + ys * dy
        v = -xs * dy + ys * dx
        t = u / L
        tc = np.clip(t, 0, 1)
        v = v - bp * 4 * tc * (1 - tc)
        wm = 1 + wobble * (0.6 * np.sin(6.283 * fr[0] * tc + ph[0]) + 0.4 * np.sin(6.283 * fr[1] * tc + ph[1]))
        tp = 1.0 - taper * tc ** 0.8
        hwt = hw * wm * tp * (0.8 + 0.2 * smooth(0.0, 0.12, t))
        vn = v / np.maximum(hwt, 1e-3)
        # round-ish caps
        cap = np.where(u < 0, -u / hw, np.where(u > L, (u - L) / hw, 0.0))
        r = np.sqrt(vn * vn + cap * cap)
        sn = max(soft, 1.1 / hw)
        across = np.clip((1 - r) / sn, 0, 1)
        if not across.any():
            return
        kf = np.clip((np.clip(vn, -1, 1) * 0.5 + 0.5) * (K - 1), 0, K - 1)
        k0 = kf.astype(np.int32)
        k1 = np.minimum(k0 + 1, K - 1)
        fk = kf - k0
        braw = bv[k0] * (1 - fk) + bv[k1] * fk
        bstr = 1 - bristle * braw ** 1.5
        endk = bend_k[k0] * (1 - fk) + bend_k[k1] * fk
        dryout = np.clip((endk - t) / 0.12 + 0.15, 0, 1)
        a = alpha * across * bstr * dryout
        tex = self.tex[ya:yb, xa:xb]
        if dry > 0:
            # dry paint catches on the peaks of the ground, more so as it runs out
            thr = (dry * (0.25 + 0.75 * tc)) * 0.9 - 0.25 + 0.2 * thr_n
            a = a * smooth(thr - 0.18, thr + 0.18, tex)
        if pool:
            a = a * np.clip(1 + pool * (0.5 - tex) * 2, 0, 2)
        if mask is not None:
            a = a * self.masks[mask][ya:yb, xa:xb]
        a = np.clip(a, 0, 1)
        if not (a > 0.002).any():
            return
        base = np.asarray(col, np.float32) + cj + lum_j
        if follow > 0:
            D = self.design if src is None else self.bg
            c = base * (1 - follow) + (D[ya:yb, xa:xb] + cj + lum_j) * follow
        else:
            c = np.broadcast_to(base, a.shape + (3,))
        bs = bshade[k0] * (1 - fk) + bshade[k1] * fk
        c = c + bs[..., None]
        dst = self.rgb[ya:yb, xa:xb]
        dst += (c - dst) * a[..., None]
        self.height[ya:yb, xa:xb] += a * thick * (0.55 + 0.45 * bstr)

    def polyline(self, rng, pts, w, col, alpha, **kw):
        for (x0, y0), (x1, y1) in zip(pts[:-1], pts[1:]):
            self.stroke(rng, x0, y0, x1, y1, w, col, alpha, **kw)

    def sample(self, x, y, bg=False):
        i = np.clip((np.asarray(y) * self.s).astype(int), 0, self.H - 1)
        j = np.clip((np.asarray(x) * self.s).astype(int), 0, self.W - 1)
        return (self.bg if bg else self.design)[i, j]


# ------------------------------------------------------------ the layers --

def lay_ground(cv):
    X, Y = cv.X, cv.Y
    # lower spatula layer: warm red ochre; top: a patchy lead-white priming
    red = rgb("#a86a4c")
    white = rgb("#d9cdb8")
    patch = fbm(X * 6, Y * 6, 1, 5)
    streak = fbm(X * 1.5, Y * 190, 2, 3)          # brushed top layer, horizontal
    cover = np.clip(0.72 + 0.35 * (patch - 0.5) + 0.25 * (streak - 0.5), 0, 1)
    cv.rgb[:] = red + (white - red) * cover[..., None]
    # surface texture: brush striations of the priming plus a fine linen weave
    tpx = cv.s / 1150.0                            # pixels per thread
    wx = np.sin(X * 1150 * math.pi * 2 + 0.6 * vnoise(X * 40, Y * 40, 3))
    wy = np.sin(Y * 1150 * math.pi * 2 + 0.6 * vnoise(X * 40, Y * 40, 4))
    weave = 0.5 + 0.25 * wx * wy
    wamp = float(np.clip((tpx - 1.6) / 1.5, 0, 1))
    cv.tex[:] = (0.55 * streak + 0.25 * fbm(X * 60, Y * 900, 7, 2)
                 + 0.20 * fbm(X * 300, Y * 300, 8, 2))
    cv.tex[:] = cv.tex * (1 - 0.35 * wamp) + weave * 0.35 * wamp
    t = cv.tex
    cv.tex[:] = (t - t.min()) / (t.max() - t.min() + 1e-6)
    cv.height[:] = cv.tex * 0.6


def build_design(cv):
    X, Y = cv.X, cv.Y
    t0 = time.time()
    sky = Y < HOR
    ry = ridge_y(X)
    sea = (Y >= HOR) & (Y < ry)
    land = Y >= ry
    d = np.zeros_like(cv.rgb)
    d[sky] = sky_design(X[sky], Y[sky])
    d[sea] = sea_design(X[sea], Y[sea])
    d[land] = land_design(X[land], Y[land])
    cv.bg = d.copy()
    wall = wall_inside(X, Y)
    d[wall] = wall_design(X[wall], Y[wall])
    body, head = figure_parts(X, Y)
    fig = body | head
    d[fig] = rgb("#16181a")
    cv.design[:] = d

    def aa(m):
        return np.clip(box_blur(m.astype(np.float32), max(1, int(cv.s / 2000))), 0, 1)
    hor_soft = smooth(-0.6 / cv.s, 0.6 / cv.s, HOR - Y)
    cv.masks["sky"] = hor_soft
    cv.masks["sea"] = (1 - hor_soft) * smooth(-0.8 / cv.s, 0.8 / cv.s, ry - Y)
    cv.masks["land"] = smooth(-1.2 / cv.s, 1.2 / cv.s, Y - ry)
    cv.masks["wall"] = aa(wall)
    cv.masks["fig"] = aa(fig)
    cv.masks["coat"] = aa(body & ~head)
    cv.masks["head"] = aa(head)
    cv.masks["notwall"] = 1 - cv.masks["wall"]
    cv.masks["skyonly"] = hor_soft * (1 - cv.masks["wall"])
    print(f"  design {time.time() - t0:.1f}s")


def underdrawing(cv, rng):
    g = rgb("#3b3836")
    kw = dict(dry=0.3, bristle=0.2, follow=0, pool=0, soft=0.5, thick=0.0, jitter=0.01)
    # the ruled horizon, drawn twice against a ruler
    for k in range(2):
        cv.stroke(rng, 0.0, HOR + k * 0.0006, 1.0, HOR + 0.0004 * k, 0.0011, g, 0.35, **kw)
    # the axis and the golden-section verticals (faint, construction)
    for xx in (AXIS, 0.382, 0.618):
        cv.stroke(rng, xx, 0.02, xx, ASPECT - 0.02, 0.0008, g, 0.12, **kw)
    # gable outline
    pts = [(x, y) for x, y in GABLE]
    cv.polyline(rng, pts, 0.0012, g, 0.3, **kw)
    # arch
    r = 2 * ARCH_HALF
    for side in (-1, 1):
        cx = AXIS - side * ARCH_HALF
        a0 = math.pi if side < 0 else 0.0
        pts = []
        for i in range(13):
            ang = (a0 + side * (-math.pi / 3) * i / 12) if side > 0 else (0 + (math.pi / 3) * i / 12)
            if side < 0:
                pts.append((cx + r * math.cos(-ang), ARCH_SPRING - r * math.sin(ang)))
            else:
                pts.append((cx - r * math.cos(ang), ARCH_SPRING + r * math.sin(ang)))
        cv.polyline(rng, pts, 0.0012, g, 0.3, **kw)
        xx = AXIS + side * ARCH_HALF
        cv.stroke(rng, xx, ARCH_SPRING, xx, float(ridge_y(np.float32(xx))), 0.0012, g, 0.3, **kw)
    cx, cy, rr = OCULUS
    pts = [(cx + rr * math.cos(a), cy + rr * math.sin(a)) for a in np.linspace(0, 6.3, 20)]
    cv.polyline(rng, pts, 0.0011, g, 0.3, **kw)
    xs = np.linspace(0, 1, 30)
    cv.polyline(rng, list(zip(xs, ridge_y(xs.astype(np.float32)))), 0.0012, g, 0.25, **kw)


def zone_at(cv, x, y):
    i = min(max(int(y * cv.s), 0), cv.H - 1)
    j = min(max(int(x * cv.s), 0), cv.W - 1)
    if cv.masks["wall"][i, j] > 0.5:
        return "wall"
    if y < HOR:
        return "sky"
    return "sea" if cv.masks["sea"][i, j] > 0.5 else "land"


def dead_color(cv, rng):
    """Thin umber-ish lay-in of the values, all over."""
    umber = rgb("#5a4535")
    for i in range(1400):
        x = rng.random() * 1.08 - 0.04
        y = rng.random() * (ASPECT + 0.04) - 0.02
        zone = zone_at(cv, x, y)
        c = cv.sample(x, y, zone != "wall")
        ang = rng.normal(0, 0.18) + (math.pi / 2 if (0.28 < x < 0.72 and y > 0.3 * ASPECT and rng.random() < 0.5) else 0)
        ln = rng.uniform(0.05, 0.14)
        w = rng.uniform(0.02, 0.045)
        col = c * 0.7 + umber * 0.3
        if zone != "sky":
            ang = rng.normal(0, 0.1) if zone != "wall" else ang
        cv.stroke(rng, x - math.cos(ang) * ln / 2, y - math.sin(ang) * ln / 2,
                  x + math.cos(ang) * ln / 2, y + math.sin(ang) * ln / 2, w, col, 0.45,
                  dry=0.45, bristle=0.6, follow=0.6, pool=0.4, thick=0.3, jitter=0.03,
                  mask=zone if zone != "sky" else "skyonly", src=None if zone == "wall" else "bg")


def paint_sky(cv, rng):
    # a thin, even first layer of the sky's color, the ground shimmering through
    veil = 0.72 + 0.12 * fbm(cv.X * 3, cv.Y * 12, 33, 3)
    m = (cv.masks["sky"] * veil)[..., None]
    cv.rgb += (cv.bg - cv.rgb) * m
    m = (cv.masks["sea"] * veil)[..., None]
    cv.rgb += (cv.bg - cv.rgb) * m
    # body color: broad horizontal strokes, then finer
    passes = [(260, 0.030, 0.060, 0.14, 0.34, 0.55, 0.35),
              (1400, 0.010, 0.022, 0.06, 0.18, 0.45, 0.4),
              (4200, 0.004, 0.009, 0.02, 0.07, 0.4, 0.3)]
    for n, w0, w1, l0, l1, alpha, dry in passes:
        for i in range(n):
            x = rng.random() * 1.1 - 0.05
            y = rng.random() ** 0.9 * HOR
            ln = rng.uniform(l0, l1)
            w = rng.uniform(w0, w1)
            ang = rng.normal(0, 0.035)
            # strokes near the horizon lie flatter and finer
            if HOR - y < 0.05:
                ang *= 0.3
            c = cv.sample(x, y, True)
            cv.stroke(rng, x - ln / 2, y - ang * ln / 2, x + ln / 2, y + ang * ln / 2, w, c, alpha,
                      bend=rng.normal(0, 0.012), dry=dry, bristle=0.35, follow=0.72,
                      mask="sky", pool=0.5, thick=0.35, jitter=0.018, soft=0.35, src="bg")


def stipple(cv, rng, region, n, r_units, alpha, jitter=0.03, lift=0.0):
    """Many tiny dabs, splatted and softened (his stippled skies and mists)."""
    m = cv.masks[region]
    pts_x = rng.random(n)
    pts_y = rng.random(n) * ASPECT
    col_j = rng.normal(0, jitter, (n, 3)).astype(np.float32) + rng.normal(0, jitter, (n, 1)).astype(np.float32) + lift
    wgt = rng.uniform(0.3, 1.0, n).astype(np.float32)
    i = np.clip((pts_y * cv.s).astype(int), 0, cv.H - 1)
    j = np.clip((pts_x * cv.s).astype(int), 0, cv.W - 1)
    acc = np.zeros((cv.H, cv.W, 3), np.float32)
    wacc = np.zeros((cv.H, cv.W), np.float32)
    np.add.at(wacc, (i, j), wgt)
    np.add.at(acc, (i, j), col_j * wgt[:, None])
    r = max(1, int(round(r_units * cv.s)))
    wb = box_blur(wacc, r)
    wb = box_blur(wb, max(1, r // 2))
    ab = box_blur(box_blur(acc, r), max(1, r // 2))
    shift = ab / np.maximum(wb, 1e-6)[..., None]
    cov = np.clip(wb * (2 * r + 1) ** 2 * 0.6, 0, 1)
    cov = cov * (0.6 + 0.8 * (1 - cv.tex))           # settles into the ground's valleys
    a = np.clip(cov * alpha * m, 0, 1)[..., None]
    target = cv.bg + shift
    cv.rgb += (target - cv.rgb) * a
    cv.height += a[..., 0] * 0.15


def paint_clouds(cv, rng):
    """Strokes that follow the cloud strata, flat and long, finer at the rims."""
    for i in range(2600):
        x = rng.random()
        y = rng.random() * HOR
        c = cv.sample(x, y, True)
        ln = rng.uniform(0.015, 0.07)
        w = rng.uniform(0.003, 0.009)
        ang = rng.normal(0, 0.02)
        in_wedge = abs(y - (0.10 * ASPECT + 0.34 * x * ASPECT)) < 0.06
        if in_wedge:
            ang = math.atan(0.34 * ASPECT) + rng.normal(0, 0.05)   # the wedge
            w *= 1.6
        cv.stroke(rng, x - math.cos(ang) * ln / 2, y - math.sin(ang) * ln / 2,
                  x + math.cos(ang) * ln / 2, y + math.sin(ang) * ln / 2, w, c, 0.22 if in_wedge else 0.35,
                  dry=0.15 if in_wedge else 0.35, bristle=0.3, follow=0.7 if in_wedge else 0.45, mask="sky", pool=0.3, thick=0.4, src="bg", soft=0.5,
                  jitter=0.02, taper=0.3)


def paint_moon(cv, rng):
    mx, my, mr = 0.805, 0.275 * ASPECT, 0.0105
    pale = rgb("#f0e6c8")
    x0, x1 = int((mx - 2 * mr) * cv.s), int((mx + 2 * mr) * cv.s) + 1
    y0, y1 = int((my - 2 * mr) * cv.s), int((my + 2 * mr) * cv.s) + 1
    X, Y = cv.X[y0:y1, x0:x1], cv.Y[y0:y1, x0:x1]
    edge = 0.9 / cv.s + mr * 0.04
    d1 = np.sqrt((X - mx) ** 2 + (Y - my) ** 2)
    d2 = np.sqrt((X - mx - mr * 0.45) ** 2 + (Y - my + mr * 0.25) ** 2)
    cres = smooth(edge, -edge, d1 - mr) * smooth(-edge, edge, d2 - mr * 0.93)
    # thick paint: a little uneven, heavier on the lit limb
    lump = 0.8 + 0.2 * vnoise(X * 900, Y * 900, 77) + 0.1 * cv.tex[y0:y1, x0:x1]
    a = np.clip(cres * lump, 0, 1)[..., None] * 0.92
    cv.rgb[y0:y1, x0:x1] += (pale - cv.rgb[y0:y1, x0:x1]) * a
    cv.height[y0:y1, x0:x1] += a[..., 0] * 1.2
    # the veil: a thin strip of cloud drawn across its lower half
    for i in range(22):
        y = my + mr * rng.uniform(0.0, 0.9)
        x = mx + rng.normal(0, 0.8) * mr
        c = cv.sample(x, my + mr * 2.5, True) * 0.9
        cv.stroke(rng, x - 0.035, y, x + 0.035, y + rng.normal(0, 0.0015), rng.uniform(0.002, 0.005), c, 0.4,
                  dry=0.3, bristle=0.4, follow=0.0, pool=0.2, thick=0.3, jitter=0.01, taper=0.4, soft=0.6)


def paint_sea(cv, rng):
    for n, w0, w1, l0, l1, alpha in [(900, 0.006, 0.014, 0.05, 0.18, 0.5),
                                     (3000, 0.0015, 0.004, 0.02, 0.09, 0.35)]:
        for i in range(n):
            x = rng.random() * 1.1 - 0.05
            y = HOR + rng.random() ** 1.2 * (0.80 * ASPECT + 0.03 - HOR)
            c = cv.sample(x, y, True)
            ln = rng.uniform(l0, l1)
            w = rng.uniform(w0, w1)
            ang = rng.normal(0, 0.006)
            cv.stroke(rng, x - ln / 2, y - ang * ln, x + ln / 2, y + ang * ln, w, c, alpha,
                      dry=0.45, bristle=0.4, follow=0.6, mask="sea", pool=0.35, thick=0.4, src="bg",
                      jitter=0.015, taper=0.2)
    # a few long pale glints of the horizon light, fewer and shorter nearer
    for i in range(420):
        e = rng.random() ** 2
        y = HOR + 0.002 + e * 0.12
        x = AXIS + rng.normal(0, 0.10 + 0.3 * e) if rng.random() < 0.5 else rng.random()
        ln = rng.uniform(0.01, 0.06) * (1 - 0.6 * e)
        c = rgb("#cdb99c") * (1 - 0.3 * e) + rgb("#6b6a76") * 0.3 * e
        cv.stroke(rng, x - ln / 2, y, x + ln / 2, y, rng.uniform(0.0008, 0.0022), c, 0.5 * (1 - e),
                  dry=0.6, bristle=0.5, follow=0.0, mask="sea", pool=0.0, thick=0.5, jitter=0.02, taper=0.5)
    # the ruled horizon, crisp at the center, softened by mist to the sides
    for i in range(40):
        x = rng.random()
        off = abs(x - AXIS)
        c = cv.sample(x, HOR - 0.004, True) * 0.6 + cv.sample(x, HOR + 0.004, True) * 0.4
        cv.stroke(rng, x - 0.04, HOR + 0.0005, x + 0.04, HOR + 0.0005, 0.003 + 0.01 * off, c, 0.3 * off,
                  dry=0.2, bristle=0.3, follow=0.0, pool=0.3, thick=0.1, jitter=0.01, soft=0.9)


def paint_land(cv, rng):
    m = cv.masks["land"][..., None]
    cv.rgb += (cv.bg - cv.rgb) * m * 0.9
    for n, w0, w1, alpha in [(700, 0.012, 0.03, 0.7), (1800, 0.003, 0.008, 0.5)]:
        for i in range(n):
            x = rng.random() * 1.1 - 0.05
            y = 0.79 * ASPECT + rng.random() * (0.22 * ASPECT)
            c = cv.sample(x, max(y, float(ridge_y(np.float32(x))) + 0.004), True)
            ln = rng.uniform(0.02, 0.08)
            ang = rng.normal(0, 0.25) + (0.3 if x < AXIS else -0.3) * rng.random()
            cv.stroke(rng, x - math.cos(ang) * ln / 2, y - math.sin(ang) * ln / 2,
                      x + math.cos(ang) * ln / 2, y + math.sin(ang) * ln / 2, rng.uniform(w0, w1), c, alpha,
                      dry=0.3, bristle=0.4, follow=0.6, mask="land", pool=0.2, thick=0.8, jitter=0.012, src="bg")
    # the crest catches a little of the sky
    xs = np.linspace(-0.02, 1.02, 90)
    for i in range(len(xs) - 1):
        x0, x1 = xs[i], xs[i + 1]
        y0 = float(ridge_y(np.float32(x0))) + 0.0025
        y1 = float(ridge_y(np.float32(x1))) + 0.0025
        cv.stroke(rng, x0, y0, x1 + 0.004, y1, 0.0035, rgb("#57504a"), 0.45,
                  dry=0.6, bristle=0.6, follow=0.0, mask="land", pool=0.0, thick=0.6, jitter=0.03)


def stone_courses():
    """Coursed rubble: rows of stones with jittered joints (unit coords)."""
    rng = np.random.default_rng(7)
    stones = []
    y = 0.80 * ASPECT + 0.01
    while y > 0.09 * ASPECT:
        hgt = rng.uniform(0.013, 0.021)
        x = 0.20 + rng.uniform(0, 0.03)
        while x < 0.80:
            wd = rng.uniform(0.022, 0.05)
            stones.append((x, y - hgt, x + wd, y, rng.normal(0, 1)))
            x += wd + rng.uniform(0.0006, 0.0018)
        y -= hgt + rng.uniform(0.0006, 0.0016)
    return stones


def paint_wall(cv, rng):
    # a thin flat dead color first, so nothing of the pale ground grins through
    m = cv.masks["wall"][..., None]
    cv.rgb += (cv.design * 0.9 + rgb("#2a211b") * 0.1 - cv.rgb) * m * 0.9
    # body color laid in: a grid of overlapping strokes, mostly vertical
    for gy in np.arange(0.09 * ASPECT, 0.83 * ASPECT, 0.008):
        for gx in np.arange(0.2, 0.8, 0.025):
            x = gx + rng.uniform(-0.01, 0.01)
            y = gy + rng.uniform(-0.009, 0.009)
            if zone_at(cv, x, y) != "wall":
                continue
            c = cv.sample(x, y)
            ang = rng.normal(0, 0.05)
            ln = rng.uniform(0.03, 0.05)
            cv.stroke(rng, x - math.cos(ang) * ln / 2, y - math.sin(ang) * ln / 2,
                      x + math.cos(ang) * ln / 2, y + math.sin(ang) * ln / 2, rng.uniform(0.008, 0.016), c, 0.75,
                      dry=0.15, bristle=0.2, follow=0.9, mask="wall", pool=0.1, thick=0.9, jitter=0.01,
                      wobble=0.04)
    # stone faces dabbed on with a stiff brush: granular, not streaked
    for i in range(2600):
        x = rng.uniform(0.2, 0.8)
        y = rng.uniform(0.09, 0.83) * ASPECT
        if zone_at(cv, x, y) != "wall":
            continue
        ln = rng.uniform(0.006, 0.016)
        ang = rng.normal(0, 0.3)
        cv.stroke(rng, x - math.cos(ang) * ln / 2, y - math.sin(ang) * ln / 2,
                  x + math.cos(ang) * ln / 2, y + math.sin(ang) * ln / 2, rng.uniform(0.005, 0.011),
                  cv.sample(x, y), 0.4, dry=0.65, bristle=0.45, follow=0.75, mask="wall",
                  pool=0.0, thick=1.0, jitter=0.02, wobble=0.05)
    # weather: long faint rain streaks down from the broken top
    for i in range(90):
        x = rng.uniform(0.21, 0.79)
        y0 = float(gable_top(np.float32(x))) + 0.004
        ln = rng.uniform(0.03, 0.18)
        c = rgb("#231f1c")
        cv.stroke(rng, x, y0, x + rng.normal(0, 0.002), y0 + ln, rng.uniform(0.001, 0.004), c, 0.18,
                  dry=0.6, bristle=0.6, follow=0.0, mask="wall", pool=0.2, thick=0.2, jitter=0.02, taper=0.8)
    # the reveal of the arch catches the glow: a warm band just inside the edge
    X, Y = cv.X, cv.Y
    g = arch_gap(X, Y)
    band = smooth(0.0045, 0.0012, g) * (g > -0.002)
    fade = np.clip(1.0 - (Y - HOR) / 0.05, 0.0, 1.0) * (0.55 + 0.45 * np.exp(-((Y - HOR) / 0.07) ** 2))
    brk = smooth(0.3, 0.6, fbm(X * 300, Y * 60, 911, 3)) * (0.5 + 0.5 * cv.tex)
    a = np.clip(band * fade * brk * 0.55, 0, 1) * cv.masks["wall"]
    cv.rgb += (rgb("#9c7a60") - cv.rgb) * a[..., None]
    cv.height += a * 0.6
    # and the top edge of the broken gable, light against the dusk, exactly drawn
    pts = GABLE
    for (x0, y0), (x1, y1) in zip(pts[:-1], pts[1:]):
        cv.stroke(rng, x0, y0 + 0.0022, x1, y1 + 0.0022, 0.0022, rgb("#4d4642"), 0.25, dry=0.6,
                  bristle=0.5, follow=0.0, mask="wall", pool=0.0, thick=0.5, jitter=0.02)


def paint_figure(cv, rng):
    fx, fy = figure_feet()
    coat = rgb("#1c201f")         # a dark blue-green coat, the only other color
    legs = rgb("#131210")
    hair = rgb("#1f1813")
    # flat lay-in, like a silhouette cut exactly against the light
    Y = cv.Y
    lay = np.where((Y > fy - 0.36 * FIG_M)[..., None], legs, coat)
    m = cv.masks["coat"][..., None]
    cv.rgb += (lay - cv.rgb) * m * 0.95
    m = cv.masks["head"][..., None]
    cv.rgb += (hair - cv.rgb) * m * 0.95
    # the coat worked in long, slightly curved verticals: its folds
    for i in range(260):
        x = fx + rng.uniform(-0.28, 0.28) * FIG_M
        y0 = fy - rng.uniform(0.4, 1.45) * FIG_M
        ln = rng.uniform(0.2, 0.6) * FIG_M
        c = coat * rng.uniform(0.8, 1.25)
        cv.stroke(rng, x, y0, x + (x - fx) * 0.08, y0 + ln, rng.uniform(0.0015, 0.004), c, 0.45,
                  dry=0.35, bristle=0.4, follow=0.0, mask="coat", pool=0.0, thick=0.8, jitter=0.006,
                  taper=0.4, bend=rng.normal(0, 0.03))
    for i in range(60):
        x = fx + rng.uniform(-0.12, 0.12) * FIG_M
        y0 = fy - rng.uniform(0.0, 0.4) * FIG_M
        cv.stroke(rng, x, y0, x, y0 + 0.1 * FIG_M, 0.003, legs * rng.uniform(0.8, 1.4), 0.5,
                  dry=0.2, bristle=0.3, follow=0.0, mask="coat", pool=0.0, thick=0.6, jitter=0.006)
    for i in range(40):
        a = rng.uniform(0, 6.283)
        x = fx + 0.005 * FIG_M + math.cos(a) * 0.06 * FIG_M
        y = fy - 1.64 * FIG_M + math.sin(a) * 0.07 * FIG_M
        cv.stroke(rng, x, y, x + 0.03 * FIG_M, y + 0.06 * FIG_M, 0.0016, hair * rng.uniform(0.9, 1.6), 0.5,
                  dry=0.3, bristle=0.3, follow=0.0, mask="head", pool=0.0, thick=0.5, jitter=0.006)
    # faint warm rim along the shoulders, collar and head, from the glow behind
    warm = rgb("#7a6150")
    for side in (-1, 1):
        pts = [(fx + side * 0.235 * FIG_M, fy - 1.30 * FIG_M), (fx + side * 0.215 * FIG_M, fy - 1.40 * FIG_M),
               (fx + side * 0.16 * FIG_M, fy - 1.465 * FIG_M), (fx + side * 0.08 * FIG_M, fy - 1.515 * FIG_M)]
        cv.polyline(rng, pts, 0.0011, warm, 0.4, dry=0.5, bristle=0.4, follow=0.0,
                    mask="coat", pool=0.0, thick=0.3, jitter=0.005)
        pts = [(fx + 0.005 * FIG_M + side * 0.098 * math.sin(a) * FIG_M,
                fy - (1.615 + 0.118 * math.cos(a)) * FIG_M) for a in np.linspace(0.9, 1.9, 5)]
        cv.polyline(rng, pts, 0.001, warm, 0.35, dry=0.5, bristle=0.4, follow=0.0,
                    mask="head", pool=0.0, thick=0.3, jitter=0.005)


def grass(cv, rng, x, y, n, h, lean=0.0, col=None):
    col = rgb("#2c2a20") if col is None else col
    for i in range(n):
        bx = x + rng.normal(0, h * 0.35)
        ang = -math.pi / 2 + lean + rng.normal(0, 0.28)
        ln = h * rng.uniform(0.4, 1.0)
        cv.stroke(rng, bx, y, bx + math.cos(ang) * ln, y + math.sin(ang) * ln,
                  rng.uniform(0.0008, 0.0016), col * rng.uniform(0.8, 1.2), 0.8,
                  dry=0.2, bristle=0.2, follow=0.0, pool=0.0, thick=0.9, taper=0.9,
                  bend=rng.normal(0, 0.06), jitter=0.02, soft=0.4, wobble=0.0)


def branch(cv, rng, x, y, ang, ln, w, depth, col):
    x1 = x + math.cos(ang) * ln
    y1 = y + math.sin(ang) * ln
    cv.stroke(rng, x, y, x1, y1, w, col, 0.9, dry=0.1, bristle=0.2, follow=0.0, pool=0.0,
              thick=0.8, taper=0.35, bend=rng.normal(0, 0.08), jitter=0.01, soft=0.35, wobble=0.05)
    if depth == 0 or w < 0.0005:
        return
    nb = int(rng.integers(2, 4))
    for k in range(nb):
        # twitching, breaking angles: gnarled inside the calm whole
        a2 = ang + rng.normal(0, 0.55) + (0.35 if k % 2 else -0.35)
        a2 = a2 * 0.8 + (-math.pi / 2) * 0.2
        branch(cv, rng, x1, y1, a2, ln * rng.uniform(0.55, 0.8), w * 0.62, depth - 1, col)


def vegetation(cv, rng):
    # grass along the near crest, flicked up last; thicker by the wall
    for i in range(70):
        x = rng.uniform(0.0, 1.0)
        near = math.exp(-((abs(x - AXIS) - 0.26) / 0.06) ** 2)
        if rng.random() > 0.25 + 0.75 * near:
            continue
        if abs(x - AXIS) < ARCH_HALF + 0.01:
            continue
        y = float(ridge_y(np.float32(x))) + 0.004
        grass(cv, rng, x, y, int(rng.integers(8, 22)), rng.uniform(0.008, 0.02), lean=rng.normal(0, 0.1))
    # tufts on the broken steps of the gable
    for (gx, gy) in [(0.227, 0.652 * ASPECT), (0.25, 0.596 * ASPECT), (0.272, 0.516 * ASPECT),
                     (0.568, 0.212 * ASPECT), (0.605, 0.246 * ASPECT), (0.745, 0.563 * ASPECT),
                     (0.725, 0.475 * ASPECT)]:
        grass(cv, rng, gx, gy + 0.002, int(rng.integers(6, 14)), rng.uniform(0.006, 0.012),
              col=rgb("#2a2820"))
    # a bare young birch that has seeded itself in the broken right shoulder


def finishing(cv, rng):
    X, Y = cv.X, cv.Y
    # Carus's advice: a dark glaze toward the edges, the light kept free
    dx = (X - AXIS) / 0.5
    dy = (Y - HOR) / (ASPECT * 0.7)
    e = np.clip(np.sqrt(dx * dx * 0.8 + dy * dy) - 0.55, 0, 1)
    glaze = rgb("#3a2c22")
    k = (0.35 * e ** 1.5)[..., None]
    cv.rgb[:] = cv.rgb * (1 - k) + cv.rgb * glaze * 1.6 * k
    # varnish: warm, a little milky
    cv.rgb[:] = cv.rgb * rgb("#fbf1dc") + 0.012
    # surface relief lit from the upper left
    h = box_blur(cv.height, max(1, int(cv.s / 3000)))
    gy, gx = np.gradient(h)
    amp = 0.9 * cv.s / 3200
    shade = 1 + np.clip((-gx * 0.6 - gy * 0.8) * 0.06 / max(amp, 0.3), -0.12, 0.12)
    cv.rgb *= shade[..., None]
    craquelure(cv, rng)


def crack_net(X, Y, cell, seed):
    """Distance to the nearest cell border of a jittered Voronoi net (cell units)."""
    gx, gy = X / cell, Y / cell
    ix, iy = np.floor(gx).astype(np.int64), np.floor(gy).astype(np.int64)
    d1 = np.full(X.shape, 9.0, np.float32)
    d2 = np.full(X.shape, 9.0, np.float32)
    for oy in (-1, 0, 1):
        for ox in (-1, 0, 1):
            cx, cy = ix + ox, iy + oy
            px = cx + 0.1 + 0.8 * _hash(cx, cy, seed)
            py = cy + 0.1 + 0.8 * _hash(cx, cy, seed + 1)
            # cells stretched a little along the weave, like real grain
            d = np.sqrt(((gx - px) * 1.15) ** 2 + (gy - py) ** 2)
            d2 = np.where(d < d1, d1, np.minimum(d2, d))
            d1 = np.minimum(d1, d)
    return d2 - d1


def craquelure(cv, rng):
    """Fine, clustered, uneven cracks in the paint and the old varnish."""
    X, Y = cv.X, cv.Y
    out = np.zeros(X.shape, np.float32)
    for cell, strength, seed in ((0.0062, 0.11, 300), (0.0026, 0.06, 310)):
        cpx = cell * cv.s
        fade = float(np.clip((cpx - 5) / 6, 0, 1))
        if fade <= 0:
            continue
        gap = crack_net(X, Y, cell, seed)
        width = 0.45 / cpx
        line = smooth(width * 1.6, width * 0.3, gap)
        # strength varies per stretch of crack and in clusters over the picture
        var = smooth(0.35, 0.75, vnoise(X / cell * 0.7, Y / cell * 0.7, seed + 5))
        clus = smooth(0.4, 0.7, fbm(X * 5, Y * 5, seed + 7, 3))
        out = np.maximum(out, line * var * clus * strength * fade)
    cv.rgb *= (1 - out)[..., None]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--width", type=int, default=1000)
    ap.add_argument("--out", default="out.png")
    ap.add_argument("--preview", default=None, help="also write a JPEG preview (max 1000px)")
    args = ap.parse_args()
    t0 = time.time()
    cv = Canvas(args.width)
    lay_ground(cv)
    build_design(cv)
    rng = np.random.default_rng(1774)
    steps = [("underdrawing", underdrawing), ("dead color", dead_color), ("sky", paint_sky),
             ("stipple", None), ("clouds", paint_clouds), ("moon", paint_moon), ("sea", paint_sea),
             ("land", paint_land), ("wall", paint_wall), ("figure", paint_figure),
             ("vegetation", vegetation), ("finishing", finishing)]
    for name, fn in steps:
        t1 = time.time()
        if fn is None:
            srng = np.random.default_rng(1840)
            stipple(cv, srng, "sky", 260000, 0.0010, 0.35, jitter=0.02)
            stipple(cv, srng, "sky", 90000, 0.0025, 0.25, jitter=0.025)
            stipple(cv, srng, "sea", 50000, 0.0012, 0.2, jitter=0.02)
        else:
            fn(cv, rng)
        print(f"  {name} {time.time() - t1:.1f}s")
    out = np.clip(cv.rgb, 0, 1)
    img = Image.fromarray((out * 255 + 0.5).astype(np.uint8))
    img.save(args.out)
    if args.preview:
        p = img
        if p.width > 1000:
            p = p.resize((1000, int(1000 * p.height / p.width)), Image.LANCZOS)
        p.save(args.preview, quality=95, subsampling=0)
    print(f"done {time.time() - t0:.1f}s -> {args.out}")


if __name__ == "__main__":
    main()
