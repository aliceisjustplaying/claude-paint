#!/usr/bin/env python3
# The orientation measurement behind notes/research/cassatt_pastel_strokes.md §2:
#   python3 -I notes/research/cassatt_stroke_orientation.py <image> <x> <y> <size px> <px per mm> [gray|rb]
# Needs ffmpeg. Standard library only.
# A crop's dominant stroke angle (degrees from the
# horizontal, counterclockwise, as seen), coherence (0 none .. 1 all parallel) and the
# period across the strokes (mm), from a structure tensor and an autocorrelation at 2 px/mm.
import subprocess, sys, math
f, x, y, size, pxmm = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), float(sys.argv[5])
ch = sys.argv[6] if len(sys.argv) > 6 else "gray"
N = int(size / pxmm * 2)
fmt = "gray" if ch == "gray" else "rgb24"
raw = subprocess.run(["ffmpeg","-loglevel","error","-i",f,"-vf",f"crop={size}:{size}:{x}:{y},scale={N}:{N}:flags=area,format={fmt}","-f","rawvideo","-"],capture_output=True).stdout
if fmt == "gray":
    a = [list(raw[j*N:(j+1)*N]) for j in range(N)]
else:
    # chroma contrast: R - B (ochre strokes on green, orange on orange shadow, blue on white)
    a = [[raw[(j*N+i)*3] - raw[(j*N+i)*3+2] for i in range(N)] for j in range(N)]
jxx = jyy = jxy = 0.0
for j in range(1, N-1):
    for i in range(1, N-1):
        gx = (a[j-1][i+1] + 2*a[j][i+1] + a[j+1][i+1]) - (a[j-1][i-1] + 2*a[j][i-1] + a[j+1][i-1])
        gy = (a[j+1][i-1] + 2*a[j+1][i] + a[j+1][i+1]) - (a[j-1][i-1] + 2*a[j-1][i] + a[j-1][i+1])
        jxx += gx*gx; jyy += gy*gy; jxy += gx*gy
# gradient direction (image y down); strokes run perpendicular to it
th = 0.5 * math.atan2(2*jxy, jxx - jyy)
coh = math.sqrt((jxx - jyy)**2 + 4*jxy*jxy) / (jxx + jyy)
stroke = th + math.pi/2
deg = (-math.degrees(stroke)) % 180  # counterclockwise as seen (y up)
# profile across the strokes: mean along them, binned at 0.5 mm
nx, ny = math.cos(th), math.sin(th)
bins = {}
for j in range(N):
    for i in range(N):
        b = int(round((i*nx + j*ny)))
        s, c = bins.get(b, (0.0, 0))
        bins[b] = (s + a[j][i], c + 1)
keys = sorted(k for k in bins if bins[k][1] > N // 3)
p = [bins[k][0] / bins[k][1] for k in keys]
m = sum(p) / len(p)
p = [v - m for v in p]
v0 = sum(v*v for v in p) or 1.0
ac = [sum(p[k]*p[k+L] for k in range(len(p)-L)) / v0 for L in range(len(p)//2)]
# first local maximum after the first zero crossing
period = None
z = next((L for L in range(1, len(ac)) if ac[L] < 0), None)
if z:
    L = max(range(z, min(len(ac), z + 60)), key=lambda L: ac[L])
    if ac[L] > 0.05:
        period = L / 2.0
print(f"angle {deg:5.1f} deg  coherence {coh:.2f}  period across {period} mm  (crop {size/pxmm:.0f} mm, {ch})")
