-- easel session "lab2_waterA": a painting replayed chunk by chunk.
--   easel run notes/lab/water_A.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

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

-- A1: the sky thin in long level strokes, a shade duller than the target, then blended
sky1 = function(x, y) return shift(skycol(x, y), -0.01, 0, 0) end
work(skym, {hand="broad", color=sky1, angle=0, coverage=4.2, medium=0.3, pal=skypal})
blend(skym, {angle=0})

--@ chunk 3 · clock 0

-- A2: dry, then the far shore as body color, level strokes, clipped
dry()
work(far, {hand="body", color=farcol, angle=0.02, length={25, 70}, coverage=3.2, medium=0.25, clip=far})

--@ chunk 4 · clock 8196.6884765625

-- A3: dry, then the water: the mirrored sky darkening toward you, whole level strokes (no flat),
-- clipped to the water (the bank and reflections go over it later), then blended
dry()
work(water, {hand="broad", tool="round 6", color=watercol, angle=0, length={80, 220}, coverage=4.2, medium=0.3, pal=skypal, clip=water})
blend(water, {angle=0, angle_jitter=0.003, length={80, 200}, clip=water})

--@ chunk 5 · clock 16186.16015625

-- A4: dry, then the bank inside its own masks: a body underlayer, the bushes in small strokes
-- turning every which way, hatched lit masses on the upper left, then the earth face
dry()
work(bank, {hand="body", tool="filbert 4", color=function(x, y) return earth:at(x, y) > 0.5 and earthcol(x, y) or bushcol(x, y) end,
  angle=0.1, length={8, 20}, coverage=3.6, medium=0.25, clip=bank})
local turn = noise{seed=7, period=12}
work(bush, {hand="body", tool="round 2.4", color=bushcol, angle=function(x, y) return 2.2*turn(x, y) end, angle_jitter=0.6,
  length={4, 10}, coverage=3.0, medium=0.2, clip=bush})
litb = bush * mask(function(x, y) return (1 - smoothstep(banktop(x) + 8, banktop(x) + 34, y)) * (1 - smoothstep(200, 520, x)) end)
work(litb, {hand="hatch", tool="round 1.6", color="#7a7a4c", angle=function(x, y) return 2.4*turn(x, y) end, angle_jitter=0.8,
  length={3, 6}, coverage=2.0, medium=0.15, clip=bush, hug=false})
work(earth, {hand="body", tool="round 2", color=earthcol, angle=0.03, length={8, 24}, coverage=3.0, medium=0.2, clip=earth})

--@ chunk 6 · clock 26576.09765625

-- A5: dry, then the reflections inside their masks: the bank mirrored, darker and cooler, in
-- whole level strokes, then blended level; the far shore likewise
dry()
work(refl, {hand="body", tool="round 3", color=reflsrc, angle=0, length={20, 80}, coverage=3.6, medium=0.3, clip=refl})
blend(refl, {angle=0, angle_jitter=0.003, length={40, 140}, clip=refl})
work(farrefl, {hand="body", tool="round 2", color=farreflcol, angle=0, length={30, 90}, coverage=3.2, medium=0.3, clip=farrefl})
blend(farrefl, {angle=0, length={40, 140}, clip=farrefl})

--@ chunk 7 · clock 55098.076171875

-- A6: after wait(240): a few level rigger lines on calm water, clipped to the water: pale ones
-- mixed toward the mirrored sky, darker ones in the troughs, closer toward the horizon, longer
-- toward you; pale breaks across the reflection
wait(240)
local pale, dark = brush("rigger", 0.9), brush("round", 1.6)
local y, n = HZ + 6, 0
while y < H - 6 do
  local d = y - HZ
  local len = 20 + d*0.35 + rand(0, 40)
  local x0 = rand(-20, 1000)
  local inrefl = refl:at(clamp(x0 + len/2, 0, 999), y) > 0.5
  if rand() < 0.55 then
    pale:reload(shift(watercol(x0, y), inrefl and 0.08 or 0.05, 0, 0), 0.7)
    pale:stroke({{x0, y}, {x0 + len/2, y + randn(0, 0.3)}, {x0 + len, y}}, {pressure={0.6, 0.25}, ramps={0.2, 0.5}, clip=water})
  else
    dark:reload(shift(watercol(x0, y), -0.05, 0, -0.01), 0.7)
    dark:stroke({{x0, y}, {x0 + len/2, y}, {x0 + len, y}}, {pressure={0.5, 0.2}, ramps={0.2, 0.5}, clip=water})
  end
  n = n + 1
  y = y + 4 + d*0.06 + rand(0, 6)
end
print("lines", n)

--@ chunk 8 · clock 55338.076171875
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
