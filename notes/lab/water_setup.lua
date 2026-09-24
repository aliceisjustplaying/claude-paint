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
