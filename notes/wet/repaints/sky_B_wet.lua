-- easel session "lab2_skyB": a painting replayed chunk by chunk.
--   easel run notes/lab/sky_B.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

-- Adapted for the r6-wet engine (notes/wet.md §8): chunk 3 blends the cloud banks' edges into the open sky.
--@ chunk 1 · clock 0
-- Lab study "sky": a quiet evening sky over a low dark horizon strip.
-- Shared setup for sky_A (the old way) and sky_B (the new way): canvas, palette,
-- geometry (the strip, two cloud banks, the glow) and color fields.
canvas{style="friedrich", aspect=1.5, size=300, seed=61}
skypal = pal:only{"lead white","pale smalt","cobalt blue","yellow ochre","raw umber","red earth","chrome yellow","vermilion"}
HZ = 566                      -- the strip's mean crest
SUNX = 650                    -- the sun has just set here
landn = noise{seed=5, octaves=4, period=150}
crest = function(x)
  return HZ - 4 - 9*landn:at01(x, 0) - 16*math.exp(-((x-230)/95)^2) - 8*math.exp(-((x-840)/55)^2)
end
land = below(crest)
skym = above(function(x) return crest(x) + 10 end)     -- the sky runs a little under the strip
-- the glow: a broad warm swell low over SUNX, uneven along the horizon
gn = noise{seed=17, octaves=3, period=260}
glow = function(x, y)
  local gx = math.exp(-((x - SUNX)/(290 + 60*gn(x, 0)))^2)
  return gx * smoothstep(HZ - 360, HZ - 10, y)
end
skycol = function(x, y)
  local t = clamp(y / HZ, 0, 1)
  local c = gradient({{0,"#4a5873"},{0.3,"#6c7890"},{0.6,"#9c9fa6"},{0.82,"#c2b49a"},{1,"#cfb994"}}, t)
  return mix(c, "#ead29a", 0.62*glow(x, y))
end
-- two cloud banks: a thin broken high band, and a long low bank above the glow that thins
-- and parts over it (the light breaks through there)
bn = noise{seed=9, octaves=5, period=240, stretch={0, 5}}
bn2 = noise{seed=23, octaves=4, period=120, stretch={0, 4}}
band = function(y, c, th) return 1 - smoothstep(0.55*th, th, math.abs(y - c)) end
bank1 = mask(function(x, y)
  local c = 196 + 22*bn(x*0.6, 0) + 0.035*(x - 500)
  local th = 22*smoothstep(-0.15, 0.35, bn2(x, 40))
  return band(y, c, th)
end):blur(1.5)
bank2 = mask(function(x, y)
  local c = 452 + 14*bn(x, 90) - 0.03*(x - 500)
  local th = (34 + 16*bn2(x, 300)) * (1 - 0.8*math.exp(-((x - SUNX - 30)/110)^2))
  return band(y, c, th) * smoothstep(-0.35, 0.05, bn2(x*1.3, 500))
end):blur(1.5)
clouds = bank1 + bank2
-- cloud color: gray-violet bodies, the undersides lit warm where they face the glow
cloudcol = function(x, y)
  local high = y < 320
  local body = high and "#6e6f80" or "#7b7480"
  local under = high and "#b59b8e" or "#e0b688"
  local c2 = high and (196 + 22*bn(x*0.6, 0) + 0.035*(x - 500)) or (452 + 14*bn(x, 90) - 0.03*(x - 500))
  local lit = smoothstep(c2 - 4, c2 + 18, y) * (0.35 + 0.65*math.exp(-((x - SUNX)/320)^2))
  return mix(body, under, lit)
end
stripcol = function(x, y) return mix("#2c2d2e", "#1f2021", smoothstep(HZ - 20, H, y)) end
print(H, land:area(), bank1:area(), bank2:area())

--@ chunk 2 · clock 0

-- B1: one sitting. The sky as a few unequal horizontal bands of thin paint (not an even ramp):
-- a slow stretched noise moves value and warmth band by band; thin (medium 0.4), light load.
sn = noise{seed=31, octaves=3, period=180, stretch={0, 7}}
skyB = function(x, y)
  local c = skycol(x, y)
  local v = sn(x, y)
  return shift(c, 0.022*v, 0.004*v, 0.008*v)
end
work(skym, {hand="broad", color=skyB, angle=0, angle_jitter=0.02, length={120, 320}, coverage=3.8, medium=0.4, load=0.7, pal=skypal})
blend(skym, {angle=0, coverage=1.2, length={150, 400}})
print(drying(300, 100), drying(650, 480))

--@ chunk 3 · clock 0

-- B2: same sitting, no wait: the two banks as masses laid into the open sky, unclipped,
-- few long strokes (coverage 2.2), soft attack/release so the ends do not bead
cm = clouds:blur(3)
work(cm, {hand="broad", color=cloudcol, hug=false, angle=0, angle_jitter=0.02, curve={0, 0}, length={90, 260},
  coverage=2.2, medium=0.35, load=0.9, pressure={0.45, 0.7}, ramps={0.25, 0.35}, pal=skypal})
-- (r6-wet) the long strokes' ends no longer sink into the open sky: lose the banks' edges
-- into it with a clean blender, level, while both are open
blend((cm:grow(6) - cm:shrink(6)):blur(3), {angle=0, coverage=1.8, length={60, 180}, hug=false})

--@ chunk 4 · clock 0

-- B3: still open: a few unequal lights laid into the wet bottom contour of the low bank,
-- stronger toward the glow, straddling the edge in long light strokes, broken in 3-4 runs by a
-- coarse noise; the high band keeps no lights. Then the bank tops lost into the sky (blend).
local c2 = function(x) return 452 + 14*bn(x, 90) - 0.03*(x - 500) end
local c1 = function(x) return 196 + 22*bn(x*0.6, 0) + 0.035*(x - 500) end
local pn = noise{seed=44, octaves=2, period=160, stretch={0, 3}}
local low = bank2:blur(3)
lightm = ((low:grow(4) - low:shrink(12)) * mask(function(x, y)
  return smoothstep(c2(x) - 2, c2(x) + 8, y) * math.exp(-((x - SUNX)/380)^2) * smoothstep(-0.2, 0.2, pn(x, 0))
end)):blur(2)
work(lightm, {hand="broad", color=function(x, y) return mix("#c9a58e", "#ecc48e", math.exp(-((x - SUNX)/220)^2)) end,
  hug=false, angle=0, length={70, 180}, coverage=1.5, medium=0.3, load=0.85, pressure={0.35, 0.5}, ramps={0.35, 0.4}, pal=skypal})
topsm = (cm:grow(6) * mask(function(x, y)
  local c = y > 320 and c2(x) or c1(x)
  return 1 - smoothstep(c - 2, c + 8, y) end)):blur(5)
blend(topsm, {angle=0, coverage=2.4, length={60, 200}, hug=false})
print(lightm:area(), topsm:area())

--@ chunk 5 · clock 0

-- B4: the second session, the same afternoon (8 h): the sky is setting (stiff, barely blends),
-- so a dark laid into it keeps its crest but does not drag the sky down into the land.
-- The strip as one dark mass in long level strokes of thin paint; clipped: the crest is found.
wait(480)
print(drying(500, crest(500)+4), drying(500, crest(500)-6))
stripB = function(x, y)
  local c = stripcol(x, y)
  local air = (1 - smoothstep(crest(x) + 2, crest(x) + 22, y)) * 0.18
  return mix(c, "#6e6a70", air)
end
work(land, {hand="body", color=stripB, angle=0.02, angle_jitter=0.05, length={60, 180}, coverage=3.0, medium=0.3, load=0.8, clip=land})
-- lose the crest where it runs away from the glow: a level blend over a band across it
crestband = mask(function(x, y)
  local d = math.abs(y - crest(x))
  return (1 - smoothstep(2, 6, d)) * (1 - math.exp(-((x - SUNX)/260)^2))
end):blur(1)
blend(crestband, {angle=0, coverage=1.6, length={40, 140}, hug=false})

--@ chunk 6 · clock 480

-- B5: same session: the crest band again, fuller and stiffer, to close the sky showing through
topband = land * mask(function(x, y) return 1 - smoothstep(crest(x) + 22, crest(x) + 34, y) end)
work(topband, {hand="body", color=stripB, angle=0.02, angle_jitter=0.04, length={50, 150}, coverage=2.6, medium=0.18, load=1, clip=land})

--@ chunk 7 · clock 480

-- B6: next morning: the crest band is set; knock the pale streaks down with a thin veil of the
-- strip color over what is there (70%), long level strokes, clipped to the land
wait(24*60)
print(drying(300, crest(300)+8), drying(300, crest(300)-8))
work(topband:blur(2), {hand="body", color_over=function(x, y, under) return mix(under, stripB(x, y), 0.7) end,
  angle=0.02, length={60, 160}, coverage=2.2, medium=0.35, load=0.7, clip=land, hug=false})

--@ chunk 8 · clock 1920
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
