#!/usr/bin/env python3
"""Why check 17's exact cases catch a clock that restarts at each wait.

A tiny fake: one pixel's solvent evaporating in f32 (s *= exp(-dt / tau)),
nothing else (no spreading, no cure, which only add differences). On the
shared grid, `wait(0.25); wait(0.75); wait(14)` and `wait(0.25);
wait(14.75)` take the same steps (0.25, 0.75, 14 x 1) and agree exactly. A
clock restarting at each wait steps the second as 0.25, 14 x 1, 0.75; one
closed-form step per wait steps 0.25, 0.75, 14 against 0.25, 14.75. Prints
how many random (solvent, tau) pairs end bit-different; a canvas has
thousands of pixels, so its save differs. Standard library only.

    scripts/tests/thinner_clock_restart_fake.py
"""
import math
import random
import struct


def f32(x):
    return struct.unpack("<f", struct.pack("<f", x))[0]


def run(steps, s, tau):
    for dt in steps:
        s = f32(s * f32(math.exp(-dt / tau)))
    return s


random.seed(1)
n = 10000
cases = {
    "shared grid (both)": ([0.25, 0.75] + [1.0] * 14, [0.25, 0.75] + [1.0] * 14),
    "restart at each wait": ([0.25, 0.75] + [1.0] * 14, [0.25] + [1.0] * 14 + [0.75]),
    "one closed-form step per wait": ([0.25, 0.75, 14.0], [0.25, 14.75]),
}
for name, (a, b) in cases.items():
    diff = 0
    for _ in range(n):
        s, tau = f32(random.uniform(0.01, 20.0)), random.uniform(1.0, 30.0)
        diff += run(a, s, tau) != run(b, s, tau)
    print("%-30s %5d of %d pixels bit-different" % (name, diff, n))
