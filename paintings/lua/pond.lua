-- easel session "pond": a painting replayed chunk by chunk.
--   easel run paintings/lua/pond.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).
-- sittings enforced: a sitting ends at its length; the easel refuses marks until rest(hours) (notes/time.md)

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
dry()
local hn = noise{seed=51, period=60, octaves=5}
farhill = function(x) return 447 - 13 * math.exp(-((x - 330) / 80)^2) - 7 * math.exp(-((x - 470)/50)^2) - 3*math.exp(-((x-250)/40)^2) + 1.6 * hn(x, 0) end
hills = below(farhill) * above(function(x) return HZ + 2 end) * mask(function(x, y) return smoothstep(170, 215, x) end)
stipple(hills, {width=2.2, pal=skypal, color=function(x, y) return mix("#a6a2b4", "#bab2b8", smoothstep(432, 452, y)) end,
  coverage=3.2, pressure={0.5, 0.9}, dips={14, 0.4, 0.7}, medium=0.45, cluster={0.2, 4}, feather=0.5})

--@ chunk 8 · clock 29160.958984375
wood = fir_wood{skyline=outline{pts=WOODTOP, open=true, char="soft", lobe=10, seed=31}, foot={{500,461},{650,463},{800,462},{1005,466}},
  depth=3, count=15, horizon=HZ, recede=0.6, air=0.5, seed=32}
print(wood)
woodair = "#bdb3b3"
function woodrow(r)
  local h, s, nd = wood:haze(r), wood:scale(r), wood:needles(r)
  work(nd, {hand="hatch", tool=string.format("round %.1f", math.max(0.9, 1.6 * s)), length={2, 6 * s + 1}, coverage=2.4,
    clip=nd, angle=1.57, angle_jitter=0.5, pal=snowpal, color=mix("#2c3238", woodair, 0.25 + 0.6 * h)})
  local rb = brush("rigger", math.max(0.5, 1.2 * s))
  for n, f in ipairs(wood:trees(r)) do
    if n % 5 == 1 then rb:reload(mix("#262428", woodair, 0.2 + 0.6 * h), 0.85) end
    rb:stroke(f.leader.pts, {pressure={0.8, 0.15}, ramps={0.02, 0.3}})
  end
  if r == 1 then
    local hb = brush("round", 1.8 * s + 0.3)
    wood:paint(hb, r, {color=mix("#1f252a", woodair, 0.2), lit={0, 0.55}})
    wood:paint(hb, r, {color=mix("#4a4c52", woodair, 0.3), lit={0.55, 1}, every=6})
  end
end
woodrow(3); woodrow(2)
local fl = wood:floor():roughen(4, 16, 34, 2)
work(fl, {hand="body", coverage=3, clip=fl, angle=0, hug=false, pal=snowpal,
  color=function(x, y) return mix("#6e6d78", "#a9a7b2", smoothstep(448, 464, y)) end})
woodrow(1)

--@ chunk 9 · clock 29160.958984375
local sx = SPIRE[1]
tower = poly({{sx-3.2,447},{sx-3.2,414},{sx-3.8,412.5},{sx-0.25,386},{sx+0.25,386},{sx+3.8,412.5},{sx+3.2,414},{sx+3.2,447}})
nave = poly({{sx+3,447},{sx+3,432},{sx+9,426},{sx+25,426},{sx+29,431},{sx+29,447}})
local b = brush{kind="round", width=1.1, point=0.8}
work(tower + nave, {hand="detail", tool=b, pal=skypal, color="#9d97a4", angle=1.57, length={2, 6}, coverage=3, clip=true})
local lit = poly({{sx+0.2,387},{sx+3.6,412},{sx+1.2,412}}) + poly({{sx+9,426},{sx+25,426},{sx+26.5,428},{sx+10,428}})
work(lit, {hand="detail", tool=b, pal=skypal, color="#b9aca8", angle=1.2, length={2, 4}, coverage=2, clip=true})
local mistn = noise{seed=61, period=80, octaves=4, stretch={0, 4}}
stipple(above(function(x) return 462 end) * below(function(x) return 425 end), {width=2.2, pal=skypal, color="#d8ccc2",
  coverage=function(x, y) local u = (y - 436) / 26 + 0.4 * mistn(x, y); return 1.8 * smoothstep(0.15, 0.95, u) end,
  pressure={0.4, 0.8}, dips={16, 0.3, 0.7}, medium=0.7, cluster={0.2, 5}, feather=0.7, aim=false})

--@ chunk 10 · clock 29160.958984375
OAKC = {{52,236},{70,178},{112,140},{150,96},{196,84},{232,110},{262,104},{300,140},{322,190},{336,246,"c"},{318,292},
        {296,330},{250,352},{206,358},{160,362},{112,350},{76,326,"c"},{48,290}}
oak = tree_in{crown=outline{pts=OAKC, char="soft", seed=9}, trunk={{176,448},{174,410},{168,372},{170,340}},
  species="oak", season="winter", sun={0.55, -0.15, -0.6}, detail=0.3, seed=17}
barkpal = pal:only{"lead white", "smalt", "raw umber", "bone black", "red earth", "yellow ochre"}
local bark = "#37322e"
local thick = oak:wood(3.5)
local bn = noise{seed=71, period=6, octaves=3, stretch={1.57, 3}}
work(thick, {hand="body", tool="round 2", length={4, 12}, coverage=3.5, clip=thick, pal=barkpal,
  angle=function(x, y) return 1.5 + 0.3 * bn(x, y) end,
  color=function(x, y) return shift(bark, 0.03 * bn(x, y), 0.004, 0.006 * bn(y, x)) end})
-- the flank toward the dawn glow (right)
local stout = oak:wood(6)
work(stout * mask(function(x, y) return 1 - stout:at(x + 2.2, y + 0.5) end),
  {hand="body", tool="round 1.2", length={3, 9}, coverage=2.4, angle=1.5, clip=thick, pal=barkpal, color="#6f655d"})
-- the shaded left side darker, bark furrows
work(stout * mask(function(x, y) return 1 - stout:at(x - 4, y) end),
  {hand="hatch", tool="round 1", length={4, 10}, coverage=2, angle=1.55, clip=thick, pal=barkpal, color="#221f1e"})
oak:paint_wood(brush("round", 2.4), {color=bark, min=1.2, max=3.5, every=2, load=1})
-- the fine wood by hand: a pointed round, pressed to each run's width at its root and lifting to a point
local fb = brush{kind="round", width=1.3, point=1}
local ff = brush{kind="rigger", width=0.8, point=1}
local n = 0
for _, s in ipairs(oak:wood_strokes{max=1.2}) do
  local b = (s.w[1] > 0.6) and fb or ff
  n = n + 1
  if n % 7 == 1 or b:fullness() < 0.3 then b:reload(mix("#3a3431", "#57504d", rand()), 0.8) end
  local p0 = clamp(b:pressure_for(s.w[1]), 0.05, 1)
  local p1 = s.tip and 0 or clamp(b:pressure_for(s.w[#s.w]), 0.02, 1)
  b:stroke(s.pts, {pressure={p0, p1}, ramps={0.02, s.tip and 0.6 or 0.1}, shake=0.3})
end
print(n, "fine strokes")

--@ chunk 11 · clock 29160.958984375
firs = {}
local specs = {{268, 150, 44, "spire", 41}, {318, 96, 32, "young", 42}, {236, 70, 24, "young", 43}, {352, 58, 20, "old", 44}}
for i, s in ipairs(specs) do
  local x, h, wd, habit, sd = s[1], s[2], s[3], s[4], s[5]
  local fy = bankY(x) + 5
  firs[i] = fir{x=x, y=fy, height=h, width=wd, habit=habit, seed=sd, sun={0.55, -0.15, -0.6}, gap=0.22, pad=0.85, droop=1.2}
  print(firs[i])
end
firpal = pal:only{"lead white", "smalt", "yellow ochre", "raw umber", "bone black"}
for i = #firs, 1, -1 do
  local f = firs[i]
  local nd = f:needles()
  work(nd, {hand="hatch", tool="round 0.9", length={2, 5}, coverage=1.6, clip=nd, pal=firpal,
    angle=function(x, y) return 1.57 + 0.5 * clamp((f.foot[1] - x) / 10, -1, 1) end, color="#1e2524"})
  local st = brush{kind="round", width=math.max(1.2, f.hatch * 1.6), point=0.6}
  st:load("#211f1d", 0.9)
  st:stroke(f.leader.pts, {pressure={0.9, 0.05}, ramps={0.01, 0.5}, shake=0.4})
  local rb = brush("rigger", 0.9)
  for k, b in ipairs(f.boughs) do
    if k % 6 == 1 or b.dead then rb:reload(b.dead and "#5d5853" or "#1c1f1e", 0.8) end
    rb:stroke(b.pts, {pressure={0.6, 0.05}, ramps={0.05, 0.6}})
  end
  local hb = brush{kind="round", width=math.max(0.6, f.hatch), point=0.7}
  f:paint(hb, {color="#171d1c", lit={0, 0.5}})
  f:paint(hb, {color="#2e3632", lit={0.5, 0.75}})
  f:paint(hb, {color="#5a5f5c", lit={0.75, 1}, every=6})
  -- snow lying along the upper faces of some boughs
  local sb = brush{kind="round", width=math.max(0.7, f.hatch * 1.1), point=0.5}
  local k = 0
  for _, s in ipairs(f:strokes{kind="top"}) do
    k = k + 1
    if k % 3 ~= 0 then
      if k % 9 == 1 then sb:reload(mix("#c3c2cf", "#dcd5d3", rand()), 0.7) end
      sb:stroke(s.pts, {pressure={0.55, 0.1}, ramps={0.1, 0.5}})
    end
  end
end

--@ chunk 12 · clock 29160.958984375
dry()
-- the foot of each tree: the trunk carried down into the snow, then a few short strokes of snow across it
local tb = brush{kind="round", width=1.4, point=0.4}
for _, f in ipairs(firs) do
  tb:reload("#26221f", 0.8)
  tb:stroke({{f.foot[1] + rand(-0.3, 0.3), f.crown_base - 4}, {f.foot[1], f.foot[2] + 2}}, {pressure={0.7, 0.8}})
end
local sb = brush{kind="filbert", width=3}
local feet = {{176, 444, 30}}
for _, f in ipairs(firs) do feet[#feet+1] = {f.foot[1], f.foot[2], 12} end
for i, ft in ipairs(feet) do
  local x, y, r = ft[1], ft[2], ft[3]
  for k = 1, (i == 1 and 14 or 3) do
    local yy = y - 4 + k * 1.6 + rand(-0.8, 0.8)
    local x0 = x - r * rand(0.5, 0.9); local x1 = x + r * rand(0.5, 0.9)
    local c = sample(x0 - 6, yy + 2, 2):mix(sample(x1 + 6, yy + 2, 2), 0.5)
    sb:reload(c, 0.75)
    local sl = (bankY(x1) - bankY(x0)) / (x1 - x0)
    local ym = yy + sl * (0.5 * (x0 + x1) - x)
    sb:stroke({{x0, yy + sl * (x0 - x)}, {0.5 * (x0 + x1), ym - rand(0.5, 1.5)}, {x1, yy + sl * (x1 - x)}},
      {pressure={0.4, 0.7}, ramps={0.25, 0.35}, shake=0.4})
  end
end
for k = 1, 6 do
  local yy = 452 + k * 2.2 + rand(-0.6, 0.6)
  local x0, x1 = 146 + rand(-3, 3), 196 + rand(-3, 3)
  sb:reload(sample(x0 - 5, yy, 2):mix(sample(x1 + 5, yy, 2), 0.5), 0.8)
  sb:stroke({{x0, yy - 3}, {0.5 * (x0 + x1), yy + 0.3}, {x1, yy + 4}}, {pressure={0.5, 0.7}, ramps={0.25, 0.35}, shake=0.4})
end

--@ chunk 13 · clock 46798.708984375
-- the trodden path: from the bottom edge up the slope to the figure, then over the flat to the ice
PATH = {{668,716},{672,694},{662,668},{640,644},{612,622},{594,608},{580,592},{560,574},{540,556},{530,540},{526,528},{524,520}}
local wds = {}
for i, p in ipairs(PATH) do wds[i] = 1.5 + 11 * smoothstep(520, 716, p[2]) end
pathm = ribbon(PATH, wds):roughen(2, 10, 91, 1.5)
glaze(pathm:blur(1.5), {color="#7a7f9c", coats=0.16})
stipple(pathm, {width=1.8, pal=snowpal, color="#c9c8d0", coverage=0.35, pressure={0.4, 0.8}, dips={16, 0.4, 0.7}, medium=0.4, feather=0.6})
-- footprints: small darker hollows, pairs staggered, shrinking with distance
local fb = brush{kind="round", width=2.2, point=0.3}
for t = 0.02, 0.98, 0.034 do
  local n = #PATH - 1
  local u = t * n; local i = math.floor(u) + 1; local f = u - (i - 1)
  local a, b = PATH[i], PATH[i + 1]
  local x, y = a[1] + (b[1] - a[1]) * f, a[2] + (b[2] - a[2]) * f
  if y > 610 or y < 590 then
    local s = 0.25 + 0.9 * smoothstep(520, 716, y)
    local side = (math.floor(t / 0.034) % 2 == 0) and -1 or 1
    fb:reload(mix("#707592", "#80849e", rand()), 0.9)
    fb:reload(mix("#7f839c", "#8d90a6", rand()), 0.5)
    fb:stroke({{x + side * 1.6 * s, y - 1.2 * s}, {x + side * 1.6 * s + 0.3, y + 1.4 * s}}, {pressure={0.15 + 0.35 * s, 0.1}, ramps={0.2, 0.5}})
  end
end

--@ chunk 14 · clock 58599.51953125
-- the walker, from behind: greatcoat to the calves, a low hat, a stick
local fx, fy = 588, 603
coat = poly({{fx-4.4,fy-7.6},{fx-3.5,fy-15},{fx-3.3,fy-21.5},{fx-1.5,fy-23.2},{fx+1.8,fy-23.2},{fx+3.4,fy-21.4},{fx+3.7,fy-15},{fx+4.8,fy-7.9},{fx+2,fy-7.2},{fx-1,fy-7.1}}, true)
head = ellipse(fx + 0.2, fy - 25, 1.5, 1.9)
hat = poly({{fx-2.8,fy-26.4},{fx-1.7,fy-26.8},{fx-1.6,fy-29.6},{fx+1.9,fy-29.8},{fx+2,fy-26.8},{fx+3,fy-26.3},{fx+2.8,fy-25.8},{fx-2.6,fy-25.9}})
figm = coat + head + hat
local fb = brush{kind="round", width=1.2, point=0.5}
work(figm, {hand="detail", tool=fb, pal=barkpal, color="#29272d", angle=1.57, length={2, 6}, coverage=3.2, edge={found=0.6, soft=0.4, period=10}})
-- legs and boots below the hem
local lb = brush{kind="round", width=1.3, point=0.3}
lb:load("#24222a", 0.9)
lb:stroke({{fx-1.5, fy-7.6}, {fx-1.8, fy-3.6}, {fx-2.1, fy+0.2}}, {pressure={0.6, 0.7}})
lb:stroke({{fx+1.4, fy-7.6}, {fx+1.9, fy-4.4}, {fx+2.6, fy-1.6}}, {pressure={0.6, 0.7}})
-- the stick
local sb = brush{kind="rigger", width=0.7, point=1}
sb:load("#3a3330", 0.8)
sb:stroke({{fx+4, fy-12.5}, {fx+5.6, fy-5.5}, {fx+6.9, fy+0.6}}, {pressure={0.7, 0.45}})
-- the dawn catching the right shoulder and the hat brim
local rb = brush{kind="round", width=0.8, point=1}
rb:load("#6e6464", 0.7)
rb:stroke({{fx+2.1, fy-22.9}, {fx+3.3, fy-21.2}, {fx+3.6, fy-16.5}}, {pressure={0.7, 0.15}})
rb:stroke({{fx+0.8, fy-29.6}, {fx+1.9, fy-29.3}}, {pressure={0.5, 0.1}})

--@ chunk 15 · clock 58599.51953125
local drift = below(function(x) return 439.5 + (x - 158) * 0.5 end)
local patch = (ellipse(184, 456, 32, 13) * drift):soften(2.5)
stipple(patch, {width=1.8, pal=snowpal, color=function(x, y) return mix("#cdcecf", "#c6c8cc", smoothstep(440, 475, y)) end, coverage=2.2,
  pressure={0.45, 0.85}, dips={14, 0.4, 0.7}, medium=0.3, cluster={0.2, 4}, feather=0.7})

--@ chunk 16 · clock 58599.51953125
-- a few thin strata, drawn by hand: long, slightly wavering strokes of a half-dry filbert
local notree = -(oak:mask():grow(1.5) + tower:grow(1))
local cb = brush{kind="filbert", width=5}
local lines = {
  {y=318, x0=250, x1=560, c="#d0b5b3"}, {y=328, x0=600, x1=960, c="#d6bab2"}, {y=300, x0=420, x1=780, c="#c8b0b2"},
  {y=276, x0=140, x1=430, c="#bca9b1"}, {y=262, x0=560, x1=1000, c="#b9a8b2"}, {y=236, x0=300, x1=640, c="#aea2ae"},
  {y=345, x0=700, x1=1000, c="#dbc0b0"}, {y=352, x0=360, x1=560, c="#d8c0b2"}, {y=212, x0=720, x1=990, c="#a59cab"},
}
for i, l in ipairs(lines) do
  local pts = {}
  local n = 7
  for k = 0, n do
    local x = l.x0 + (l.x1 - l.x0) * k / n
    pts[#pts + 1] = {x, l.y + 2.5 * math.sin(x / 70 + i) + rand(-1, 1)}
  end
  for pass = 1, 2 do
    cb:reload(mix(l.c, sample(0.5 * (l.x0 + l.x1), l.y), 0.05 + 0.12 * pass), 0.55)
    local off = {}
    for k, p in ipairs(pts) do off[k] = {p[1] + rand(-8, 8), p[2] + (pass - 1.5) * 3 + rand(-0.6, 0.6)} end
    cb:stroke(off, {pressure={0.45, 0.2}, ramps={0.3, 0.4}, shake=0.8, swell={0.8, 1.2, 0.7}, clip=notree})
  end
end

--@ chunk 17 · clock 58599.51953125
-- bare ice swept clear of snow: long thin darker streaks, reflecting the gray upper sky
local sw = noise{seed=113, period=110, octaves=4, stretch={0.0, 10}, warp={60, 6}}
local clear = (pond:shrink(2) * mask(function(x, y) return smoothstep(0.12, 0.45, sw(x, y)) * smoothstep(466, 490, y) end)):blur(1.2)
glaze(clear, {color="#8d90a8", coats=0.22})
-- reeds and rushes along the near shore and at the pond's two ends, dry and pale where lit, dark against the ice
reedpal = pal:only{"lead white", "raw umber", "yellow ochre", "bone black", "red earth"}
local rb = brush{kind="rigger", width=0.7, point=1}
local clumps = {{338, 500, 22}, {372, 507, 14}, {455, 514, 18}, {690, 522, 10}, {845, 518, 20}, {935, 509, 16}, {985, 500, 12}, {312, 474, 10}, {952, 472, 14}}
for ci, cl in ipairs(clumps) do
  local cx, cy, n = cl[1], cl[2], cl[3]
  local scale = 0.6 + 0.6 * smoothstep(465, 525, cy)
  for k = 1, n do
    local x = cx + randn(0, 7 * scale); local y = cy + rand(-1.5, 2.5)
    local h = rand(6, 16) * scale
    local lean = randn(0.1, 0.25)
    if k % 4 == 1 then rb:reload(mix("#4a4036", "#8a7a60", rand() ^ 1.5), 0.7) end
    local tip = {x + lean * h, y - h}
    local bend = (rand() < 0.15)
    if bend then tip = {x + lean * h + h * 0.5, y - h * 0.6} end
    rb:stroke({{x, y}, {x + lean * h * 0.35, y - h * 0.55}, tip}, {pressure={clamp(0.35 + 0.4 * scale, 0.3, 0.8), 0}, ramps={0.05, 0.7}})
  end
end
