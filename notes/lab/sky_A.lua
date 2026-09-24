-- easel session "lab2_skyA": a painting replayed chunk by chunk.
--   easel run notes/lab/sky_A.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

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

-- A1: the first sky layer, thin, a shade duller than the target; long level strokes, then blend
sky1 = function(x, y) return shift(mix(skycol(x, y), "#8f8f94", 0.12), -0.012, 0, 0) end
work(skym, {hand="broad", color=sky1, angle=0, coverage=4.2, medium=0.3, pal=skypal})
blend(skym, {angle=0})

--@ chunk 3 · clock 0

-- A2: next day, dry before going over; the second sky layer stippled, lifted a little (L +0.015)
wait(24*60)
print(drying(300, 100), drying(650, 480))
dry()
sky2 = function(x, y) return shift(skycol(x, y), 0.015, 0, 0) end
stipple(skym, {width=2.6, color=sky2, coverage=2.6, pressure={0.4, 0.8}, dips={18, 0.35, 0.7}, medium=0.55, pal=skypal})

--@ chunk 4 · clock 8155.44140625

-- A3: dry, then the cloud banks inside their masks, broad, then blended
dry()
work(clouds, {hand="broad", color=cloudcol, angle=0, coverage=3.5, medium=0.35, aim=1.6, pal=skypal, clip=clouds})
blend(clouds, {angle=0, clip=clouds})

--@ chunk 5 · clock 9759.444946289063

-- A4: next day, the lit undersides scumbled warm (thin), the tops darkened with a glaze veil
dry()
local function c2(x, y) if y < 320 then return 196 + 22*bn(x*0.6, 0) + 0.035*(x - 500) end return 452 + 14*bn(x, 90) - 0.03*(x - 500) end
litm = (clouds * mask(function(x, y) return 0.8*smoothstep(c2(x, y) + 6, c2(x, y) + 16, y) * math.exp(-((x - SUNX)/300)^2) end)):blur(3)
topm = (clouds * mask(function(x, y) return 1 - smoothstep(c2(x, y) - 14, c2(x, y) + 2, y) end)):blur(4)
work(litm, {hand="scumble", color=function(x, y) return y < 320 and "#a98f8c" or "#d7ae86" end, coverage=1.4, medium=0.35, hug=false, pal=skypal})
work(topm, {hand="glaze", medium=0.8, color_over={shift={-0.04, 0, -0.012}}})
blend(clouds, {angle=0, clip=clouds})

--@ chunk 6 · clock 15327.818481445313

-- A5: dry, then the strip as body color in near-level strokes, clipped to it
dry()
work(land, {hand="body", color=stripcol, angle=0.03, length={25, 70}, coverage=3.2, medium=0.2, clip=land})

--@ chunk 7 · clock 21433.288696289063
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
