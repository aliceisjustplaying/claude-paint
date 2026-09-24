-- easel session "edges_old": a painting replayed chunk by chunk.
--   easel run paintings/lua/edges_old.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
-- A ridge and its reflection against an evening glow (edges study). Chunks 3 and 4 are the only difference
-- between the old way (stencil clips) and the new (edge=).
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.4, seed=71}
skypal = pal:only{"lead white","pale smalt","cobalt blue","yellow ochre","raw umber","red earth","chrome yellow","vermilion"}
G = function(x, c, s) return math.exp(-((x - c)/s)^2) end
HZ = 440
SUNX = 560
rn = noise{seed=5, octaves=5, period=170}
rn2 = noise{seed=6, octaves=3, period=60}
crest = function(x)
  return HZ - 12 - 14*rn:at01(x, 0) - 3*rn2(x, 0) - 150*G(x, 470, 150) - 60*G(x, 760, 90) - 30*G(x, 180, 80)
end
rangeM = mask(function(x, y) return (y > crest(x) and y < HZ + 1) and 1 or 0 end)
reflD = function(x) return 0.8*(HZ - crest(x)) end
reflM = mask(function(x, y) return (y >= HZ and y < HZ + reflD(x)) and 1 or 0 end)
skyM = mask(function(x, y) return y < HZ and 1 or 0 end) - rangeM
waterM = mask(function(x, y) return y >= HZ and 1 or 0 end) - reflM
skycol = function(x, y)
  local t = clamp(y / HZ, 0, 1)
  local c = gradient({{0,"#4a566e"},{0.35,"#6c748a"},{0.62,"#a0969b"},{0.85,"#c9b49c"},{1,"#d8c29a"}}, t)
  return mix(c, "#ecd49e", 0.55*G(x, SUNX, 300)*smoothstep(HZ - 330, HZ - 10, y))
end
rangecol = function(x, y)
  local c = mix("#4f5366", "#6f6d75", smoothstep(crest(x) + 4, HZ, y))
  return mix(c, "#7a716f", 0.25*G(x, SUNX, 200))
end
watercol = function(x, y)
  local d = y - HZ
  local c = skycol(x, math.max(HZ - 1.05*d - 3, 5))
  return mix(c, "#434856", 0.5*smoothstep(HZ + 20, 700, y))
end

--@ chunk 2 · clock 0
-- the sky and the water laid over everything (the range comes over them), then dry
local skyAll = mask(function(x, y) return y < HZ + 1 and 1 or 0 end)
local waterAll = mask(function(x, y) return y >= HZ and 1 or 0 end)
work(skyAll, {hand="broad", color=skycol, angle=0, angle_jitter=0.02, length={120, 320}, coverage=3.8, medium=0.3, load=0.75, pal=skypal, clip=true})
blend(skyAll, {angle=0, coverage=1.2, length={150, 400}, clip=true})
work(waterAll, {hand="broad", color=watercol, angle=0, angle_jitter=0.004, curve={0, 0}, length={120, 320}, coverage=3.8, medium=0.3, load=0.75, pal=skypal, clip=true})
-- the dead color of the range and its mirror: lean, a little inside their lines (the last pass decides the edges)
local dead = (rangeM + reflM):shrink(3)
work(dead, {hand="body", tool="filbert 8", color=function(x, y) return y < HZ and "#5a5b66" or "#62636c" end, angle=0, length={30, 90}, coverage=3, medium=0.4, load=0.6, pal=skypal, clip=true})
print(dry())

--@ chunk 3 · clock 0
-- the old way: range and reflection clipped to their masks
work(rangeM, {hand="body", tool="filbert 6", color=rangecol, angle=0.04, angle_jitter=0.12, length={24, 80}, coverage=3.0, medium=0.3, load=0.8, pal=skypal, clip=true})
work(reflM, {hand="body", tool="filbert 6", color=function(x, y) return mix(rangecol(x, HZ - (y - HZ)/0.8), watercol(x, y), 0.3) end,
  angle=0, angle_jitter=0.01, curve={0, 0}, length={30, 110}, coverage=2.8, medium=0.3, load=0.85, pal=skypal, clip=true})
