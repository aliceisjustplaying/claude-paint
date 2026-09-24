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
