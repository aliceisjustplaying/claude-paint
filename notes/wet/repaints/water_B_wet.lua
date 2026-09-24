-- easel session "lab2_waterB": a painting replayed chunk by chunk.
--   easel run notes/lab/water_B.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

-- Adapted for the r6-wet engine (notes/wet.md §8): chunk 5 blends the reflections level more (coverage 3.5 for 2.0).
--@ chunk 1 · clock 0
-- Lab study "water": a still water edge, a bank meeting calm water with its reflection.
-- Shared setup for water_A (the old way) and water_B (the new way): canvas, palette,
-- geometry (far shore, near bank, waterline, reflections) and color fields.
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=4/3, size=280, seed=73}
skypal = pal:only{"lead white","pale smalt","cobalt blue","yellow ochre","raw umber","red earth"}
HZ = 352                                   -- the far shore's waterline (eye level is just above)
fn = noise{seed=3, octaves=5, period=70}
fartop = function(x) return HZ - 10 - 9*fn:at01(x, 0) - 6*math.exp(-((x - 820)/60)^2) end
far = below(fartop) * above(function(x) return HZ end)
-- the near bank comes in from the left: a bush mass on a low earth bank, its tip at x ~ 640
bn = noise{seed=11, octaves=5, period=60}
bn2 = noise{seed=12, octaves=3, period=22}
WL = function(x) return 604 - 0.05*x + 2.5*bn(x, 400) end          -- where bank meets water
TIP = 650
banktop = function(x)
  local mass = 462 + 10*smoothstep(120, 440, x) + 12*bn:at01(x, 0) + 8*bn2(x, 0)
    - 36*math.exp(-((x - 150)/60)^2) - 14*math.exp(-((x - 318)/34)^2)
  local spit = lerp(WL(x) - 13 + 3*bn2(x, 9), WL(x) - 2, smoothstep(560, TIP, x)^1.5)   -- a low grassy spit
  return lerp(mass, spit, smoothstep(470, 545, x))                   -- the bushes end in a shoulder
end
bank = mask(function(x, y)
  if x > TIP + 6 then return 0 end
  return smoothstep(banktop(x) - 1, banktop(x) + 1, y) * (1 - smoothstep(WL(x) - 1, WL(x) + 1, y))
end)
earth = bank * mask(function(x, y) return smoothstep(WL(x) - 26 - 10*bn:at01(x, 7), WL(x) - 14, y) end)
bush = bank - earth
-- reflections: calm water mirrors each shore about its own waterline (shortened: we look down a little)
refl = mask(function(x, y)
  if x > TIP + 6 then return 0 end
  local d = WL(x) - banktop(x)
  if d <= 0 then return 0 end
  local yy = y - WL(x)
  return smoothstep(-1, 1, yy) * (1 - smoothstep(0.7*d - 1, 0.7*d + 1, yy))
end)
farrefl = mask(function(x, y)
  local d = HZ - fartop(x)
  local yy = y - HZ
  return smoothstep(-1, 1, yy) * (1 - smoothstep(0.85*d - 1, 0.85*d + 1, yy))
end)
water = below(function(x) return HZ end) - bank
skym = above(function(x) return fartop(x) + 6 end)
-- colors: a quiet late-afternoon sky; water is the mirrored sky, darkening toward you
skycol = function(x, y)
  return gradient({{0, "#7f8ea6"}, {0.55, "#aab3bd"}, {1, "#d8d4c4"}}, clamp(y / HZ, 0, 1))
end
watercol = function(x, y)
  local d = y - HZ
  local c = skycol(x, clamp(HZ - d*2.0 - 6, 0, HZ))
  return mix(c, "#4e5a66", smoothstep(0, 360, d) * 0.55)
end
farcol = function(x, y) return mix("#6e777c", "#5c6569", smoothstep(fartop(x), HZ, y)) end
bushcol = function(x, y)              -- lit from the upper left: lighter near the top-left of the mass
  local t = smoothstep(banktop(x) + 4, banktop(x) + 70, y)
  return mix(mix("#626240", "#383c29", t), "#282c22", smoothstep(0, 1, (x - 250)/500) * 0.5)
end
earthcol = function(x, y) return mix("#5b5140", "#3d372c", smoothstep(WL(x) - 22, WL(x), y)) end
-- what the reflection shows: the bank's color at the mirrored point, darker and cooler
reflsrc = function(x, y)
  local my = WL(x) - (y - WL(x))/0.7
  local c = (my > WL(x) - 22) and earthcol(x, my) or bushcol(x, my)
  return shift(mix(c, "#3c4550", 0.18), -0.05, 0, -0.01)
end
farreflcol = function(x, y) return shift(farcol(x, 2*HZ - y), -0.03, 0, -0.008) end
print(H, bank:area(), refl:area(), far:area(), farrefl:area())

--@ chunk 2 · clock 0

-- B1: masses first, one sitting. The bank, its earth and its reflection are ONE dark shape:
-- laid together, thin, the bush in a few big turning strokes, the reflection pulled DOWN in
-- vertical strokes (as a brush drags a reflection), the earth level. Far shore and its
-- reflection likewise as one band, in long level strokes.
darkm = bank + refl
local turn = noise{seed=7, period=30}
work(bush, {hand="body", tool="filbert 6", color=bushcol, angle=function(x, y) return 1.2 + 1.2*turn(x, y) end,
  length={10, 26}, coverage=2.6, medium=0.3, load=0.8})
work(earth, {hand="body", tool="filbert 5", color=earthcol, angle=0.03, length={20, 60}, coverage=2.6, medium=0.3, load=0.8, hug=false})
work(refl, {hand="body", tool="filbert 6", color=reflsrc, angle=math.pi/2, angle_jitter=0.05, length={12, 40}, coverage=2.8, medium=0.35, load=0.8})
farband = far + farrefl
work(farband, {hand="broad", tool="filbert 6", color=function(x, y) return y < HZ and farcol(x, y) or farreflcol(x, y) end,
  angle=0, angle_jitter=0.01, curve={0, 0}, length={80, 240}, coverage=2.8, medium=0.3, load=0.8})

--@ chunk 3 · clock 0

-- B2: same sitting: the far band again, fuller, so it closes (the first pass left ground between strokes)
work(farband, {hand="broad", color=function(x, y) return y < HZ and farcol(x, y) or farreflcol(x, y) end,
  angle=0, angle_jitter=0.01, curve={0, 0}, length={80, 240}, coverage=3.6, medium=0.3, load=0.9})

--@ chunk 4 · clock 0

-- B3: same sitting, the darks still open: the sky and the water laid AROUND them in long level
-- strokes of thin paint, a slow band noise so neither is an even ramp. Each is fenced by its own
-- region grown 3 units and softened: the light meets the wet dark over a few units (soft, lost in
-- places) instead of being dragged across it.
sn = noise{seed=31, octaves=3, period=200, stretch={0, 7}}
local band = function(c, x, y, k) local v = sn(x, y); return shift(c, k*v, 0, 0.3*k*v) end
local skyr = skym - bank - farband
work(skyr, {hand="broad", color=function(x, y) return band(skycol(x, y), x, y, 0.018) end, angle=0, angle_jitter=0.015,
  length={120, 320}, coverage=3.8, medium=0.4, load=0.7, pal=skypal, clip=skyr:grow(4):blur(2)})
openw = water - refl - farrefl
work(openw, {hand="broad", color=function(x, y) return band(watercol(x, y), x, y + 400, 0.012) end, angle=0, angle_jitter=0.004,
  curve={0, 0}, length={120, 320}, coverage=3.8, medium=0.4, load=0.7, pal=skypal, clip=openw:grow(3):blur(2)})

--@ chunk 5 · clock 0

-- B4: same sitting: the earth face again, fuller (it showed ground between strokes); then the
-- reflections dragged level with a clean blender while everything is open: the vertical pulls
-- fuse, the reflection outline goes soft into the water. Fenced to the water, so the bank stays.
work(earth, {hand="body", color=earthcol, angle=0.02, length={30, 90}, coverage=3.2, medium=0.3, load=0.9, clip=earth:grow(1)})
local wz = (refl:grow(8) + farrefl:grow(5)) * water
blend(wz, {angle=0, angle_jitter=0.004, length={40, 160}, coverage=3.5, clip=water})  -- (r6-wet) 2.0 -> 3.5: the vertical pulls' ends no longer sink by themselves

--@ chunk 6 · clock 0

-- B5: same sitting: a few unequal lights laid into the wet bush (not a layer): three or four
-- lit masses toward the upper left, gated by a coarse noise, a muted light olive, thin
local pn = noise{seed=5, octaves=2, period=55}
litm = (bush * mask(function(x, y)
  return (1 - smoothstep(banktop(x) + 6, banktop(x) + 40, y)) * (1 - smoothstep(180, 500, x)) * smoothstep(0.0, 0.35, pn(x, y))
end)):blur(3)
local turn = noise{seed=7, period=30}
work(litm, {hand="body", tool="filbert 4", color="#6c6b45", angle=function(x, y) return 1.2 + 1.2*turn(x, y) end,
  length={8, 18}, coverage=1.3, medium=0.3, load=0.7, hug=false})

--@ chunk 7 · clock 0

-- B6: the one accent: a broken sheen where the bank meets the water, three strokes of a round
-- brush along the waterline in the pale low sky the water mirrors, laid into the open paint;
-- then a short level blend over a thin band along it, so it sits in the water, not on it
local r = brush("round", 3)
for _, run in ipairs({{60, 230}, {290, 420}, {480, 636}}) do
  local pts = {}
  for x = run[1], run[2], 8 do pts[#pts + 1] = {x, WL(x) + 2.4 + randn(0, 0.4)} end
  r:reload(shift(watercol(run[1], HZ + 30), 0.0, 0, 0), 0.7)
  r:stroke(pts, {pressure={0.1, 0.5, 0.3, 0.55, 0.05}, ramps={0.25, 0.35}})
end
local sheen = mask(function(x, y) local d = y - WL(x) return smoothstep(0.5, 1.5, d) * (1 - smoothstep(4, 6, d)) end) * water
blend(sheen, {angle=0, coverage=1.5, length={20, 60}, clip=water})

--@ chunk 8 · clock 0
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
