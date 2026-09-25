-- easel session "pond": a painting replayed chunk by chunk.
--   easel run paintings/lua/pond.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).
-- sittings enforced: a sitting ends at its length; the easel refuses marks until rest(hours) (notes/time.md)

-- dry rims fix: r8-arm3's paintings/lua/pond.lua, chunks 1 to 6, then a reconstructed
-- chunk 7 (its notes, friction 2): the far hills as body strokes over the tacky sky
-- (the log keeps the workaround: dry(), then a stipple).

--@ chunk 1 · clock 0
canvas{style="friedrich_early", size=640, aspect=1.4, seed=23}; print(W, H); print(pal)

--@ chunk 2 · clock 0
HZ = 452
-- the foreground bank: its crest from the left edge down to the right
BANK = {{-5,398},{60,404},{140,428},{230,466},{320,500},{420,530},{520,552},{640,566},{780,574},{900,580},{1005,584}}
-- the pond: far shore just below the horizon, near shore curving
POND_FAR = {{300,468},{420,462},{560,460},{700,461},{840,465},{960,470}}
POND_NEAR = {{330,500},{450,512},{600,520},{760,522},{900,512},{990,500}}
-- far fir wood skyline on the right, over the far shore
WOODTOP = {{520,452},{560,440},{600,432},{650,418},{700,414},{760,408},{820,404},{880,410},{940,400},{1005,396}}
SPIRE = {430, 372}
OAK_FOOT = {176, 424}
FIG = {588, 604}
h = pencil("2H")
h:rule({0, HZ}, {1000, HZ}, {pressure=0.25})
h:sketch(BANK, {pressure=0.3})
h:sketch(POND_FAR, {pressure=0.25})
h:sketch(POND_NEAR, {pressure=0.25})
h:sketch(WOODTOP, {pressure=0.25})
h:rule({SPIRE[1], SPIRE[2]}, {SPIRE[1], HZ-8}, {pressure=0.3})
h:sketch({{OAK_FOOT[1],OAK_FOOT[2]},{172,360},{160,300},{150,250}}, {pressure=0.3})
h:sketch({{FIG[1]-3,FIG[2]},{FIG[1]-2,FIG[2]-26},{FIG[1]+3,FIG[2]-30},{FIG[1]+4,FIG[2]}}, {pressure=0.3})
-- the trodden path to the pond
h:sketch({{700,714},{650,660},{600,612},{560,570},{540,540},{520,520}}, {pressure=0.3})

--@ chunk 3 · clock 0
skypal = pal:only{"lead white", "smalt", "pale smalt", "red earth", "yellow ochre", "vermilion", "raw umber"}
local glowx = 690
skycol = function(x, y)
  local t = y / HZ
  local g = math.exp(-((x - glowx) / 330)^2)
  local c = gradient({{0, "#5f6680"}, {0.3, "#7f8399"}, {0.55, "#a8a3ad"}, {0.78, "#cdb8b4"}, {0.92, "#dfcbb3"}, {1, "#e4d6b6"}}, t)
  return mix(c, "#ecdcb4", 0.35 * g * smoothstep(0.55, 1, t))
end
skym = above(function(x) return HZ + 14 end)
work(skym, {hand="broad", color=skycol, angle=function(x, y) return 0.03 * math.sin(x / 140 + y / 90) end,
  coverage=5, medium=0.3, load=0.8, pal=skypal, length={60, 180}})
-- no blend: the stipple will fuse it

--@ chunk 4 · clock 0
wait(40)
local vary = noise{seed=41, period=90, octaves=3}
stipple(above(function(x) return HZ + 4 end), {width=2.6, pal=skypal,
  color=function(x, y) return shift(skycol(x, y), 0.012 * vary(x, y), 0.004 * vary(y, x), 0) end,
  coverage=function(x, y) return 1.6 + 0.8 * vary:at01(x, y) end,
  pressure={0.45, 0.85}, dips={18, 0.35, 0.7}, medium=0.55, cluster={0.25, 6}, feather=0.6})

--@ chunk 5 · clock 40
wait(24*60)
local function curve(pts)
  return function(x)
    if x <= pts[1][1] then return pts[1][2] end
    for i = 2, #pts do
      if x <= pts[i][1] then
        local a, b = pts[i-1], pts[i]
        local t = (x - a[1]) / (b[1] - a[1]); t = t*t*(3-2*t)
        return a[2] + (b[2] - a[2]) * t
      end
    end
    return pts[#pts][2]
  end
end
bankY = curve(BANK)
land = below(function(x) return HZ - 1 end)
bank = below(bankY):roughen(2.5, 30, 5, 1.5)
local pp = {}
for _, p in ipairs(POND_FAR) do pp[#pp+1] = p end
for i = #POND_NEAR, 1, -1 do pp[#pp+1] = POND_NEAR[i] end
pond = poly(pp, true):roughen(2, 26, 7, 0.8)
snowpal = pal:only{"lead white", "smalt", "pale smalt", "red earth", "yellow ochre", "raw umber", "bone black"}
local lie = noise{seed=17, period=140, octaves=4, stretch={0.05, 5}}
flatcol = function(x, y)
  local t = smoothstep(HZ, bankY(x), y)
  local c = mix("#d4cdc9", "#b4b3bf", t)
  return shift(c, 0.02 * lie(x, y), 0, 0.004 * lie(y, x))
end
work(land - bank - pond, {hand="body", color=flatcol, pal=snowpal, angle=function(x, y) return 0.02 * lie(x, y) end,
  length={30, 90}, coverage=3.4, medium=0.2, edge="soft"})

--@ chunk 6 · clock 1480
local streak = noise{seed=23, period=160, octaves=4, stretch={0.0, 9}}
icecol = function(x, y)
  local t = smoothstep(460, 522, y)
  local c = mix("#e0d4bb", "#c6c0c2", t)
  local g = math.exp(-((x - 690) / 150)^2)
  c = mix(c, "#ece0bf", 0.45 * g * (1 - t))
  local s = smoothstep(0.1, 0.5, streak(x, y))
  return mix(c, "#a3a7b3", 0.6 * s)
end
work(pond, {hand="broad", color=icecol, pal=snowpal, angle=0, length={40, 140}, coverage=4, medium=0.28, edge="found"})
blend(pond, {angle=0, coverage=1.2, clip=true})
local drift = noise{seed=29, period=90, octaves=4}
bankang = function(x, y)
  local s = (bankY(x + 20) - bankY(x - 20)) / 40
  local k = smoothstep(0, 160, y - bankY(x))
  return math.atan(s) * (1 - 0.7 * k) + 0.12 * drift(x, y)
end
work(bank, {hand="body", pal=snowpal, angle=bankang,
  length={25, 70}, coverage=3.8, medium=0.16, edge="soft",
  color=function(x, y)
    local d = y - bankY(x)
    local c = gradient({{0, "#d3cfd2"}, {0.08, "#c4c2cb"}, {0.5, "#b3b3c3"}, {1, "#a5a7ba"}}, smoothstep(0, 220, d))
    return shift(c, 0.025 * drift(x, y), 0.003 * drift(y, x), -0.004 * drift(x, y))
  end})

--@ chunk 7 · clock 1480
-- reconstruction of the passage in notes/round8/arm3 friction 2: the far
-- hills as body strokes over the sky as it was at this clock (no dry())
print("sky at the hills:", drying(330, 440), drying(470, 445))
local hn = noise{seed=51, period=60, octaves=5}
farhill = function(x) return 447 - 13 * math.exp(-((x - 330) / 80)^2) - 7 * math.exp(-((x - 470)/50)^2) - 3*math.exp(-((x-250)/40)^2) + 1.6 * hn(x, 0) end
hills = below(farhill) * above(function(x) return HZ + 2 end) * mask(function(x, y) return smoothstep(170, 215, x) end)
work(hills, {hand="body", pal=skypal, angle=0, length={20, 60}, medium=0.3,
  color=function(x, y) return mix("#a6a2b4", "#bab2b8", smoothstep(432, 452, y)) end})
