-- easel session "winter_willows": a painting replayed chunk by chunk.
--   easel run paintings/lua/winter_willows.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).
-- sittings enforced: a sitting ends at its length; the easel refuses marks until rest(hours) (notes/time.md)

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.4, seed=29}; print(W, H); print(pal); print(type(world), type(tree))

--@ chunk 2 · clock 0
HZ = 478
-- the far land line: flat, with a low wood band and the village
farn = noise{seed=41, octaves=4, period=90}
function farline(x)
  local wood = 10 * math.exp(-((x - 880) / 90)^2) + 6 * math.exp(-((x - 120) / 110)^2)
  return HZ - 3 - wood - 2.5 * farn(x, 0)
end
-- the stream: center line from near foreground to vanishing point
STREAM = {{150,714},{205,672},{300,626},{360,598},{388,574},{430,552},{478,530},{520,512},{548,500},{566,491},{578,484}}
-- widths shrink with depth
function stream_w(y) return 4 + 70 * ((y - HZ) / (714 - HZ))^1.6 end
-- willow feet on the right bank (x, y, height of trunk, head width)
WILLOWS = {{352,606,86,26},{462,546,46,15},{528,514,27,9},{562,496,15,5}}
FIG = {640, 552}   -- the walker's feet
MOON = {238, 176}
h = pencil("2H")
h:rule({0, HZ}, {1000, HZ}, {pressure=0.22})
local fl = {} for x = 0, 1000, 20 do fl[#fl+1] = {x, farline(x)} end
h:sketch(fl, {pressure=0.25})
-- stream banks
local L, R = {}, {}
for i, p in ipairs(STREAM) do local w = stream_w(p[2]) / 2; L[i] = {p[1] - w, p[2]}; R[i] = {p[1] + w, p[2]} end
h:sketch(L, {pressure=0.3}); h:sketch(R, {pressure=0.3})
-- willows: trunks and heads
for _, t in ipairs(WILLOWS) do
  local x, y, th, hw = t[1], t[2], t[3], t[4]
  h:sketch({{x - hw*0.35, y}, {x - hw*0.3, y - th*0.5}, {x - hw*0.5, y - th}}, {pressure=0.3})
  h:sketch({{x + hw*0.35, y}, {x + hw*0.28, y - th*0.5}, {x + hw*0.5, y - th}}, {pressure=0.3})
end
-- village and church
h:sketch({{700,HZ-4},{700,HZ-12},{712,HZ-18},{724,HZ-12},{724,HZ-4}}, {pressure=0.25})
h:sketch({{738,HZ-4},{738,HZ-30},{741,HZ-44},{744,HZ-30},{744,HZ-4}}, {pressure=0.25})
-- figure
h:sketch({{FIG[1]-4,FIG[2]},{FIG[1]-3,FIG[2]-22},{FIG[1],FIG[2]-30},{FIG[1]+3,FIG[2]-22},{FIG[1]+5,FIG[2]}}, {pressure=0.3})
fix()

--@ chunk 3 · clock 0
skyn = noise{seed=17, octaves=3, period=380, stretch={0.02, 5}}
GLOW = 250
function skycol(x, y)
  local t = clamp(y / HZ, 0, 1)
  local g = math.exp(-((x - GLOW) / 330)^2)           -- where the sun went down
  local band = skyn(x, y) * 0.04
  local c = gradient({{0, "#5d6c86"}, {0.35, "#8190a6"}, {0.62, "#b1b3bb"}, {0.8, "#cdbfb6"}, {1, "#dcc8ae"}}, t + band)
  local warm = gradient({{0, "#6c7890"}, {0.45, "#a4a8b4"}, {0.7, "#d9c7b0"}, {0.88, "#ecd7a8"}, {1, "#f0dca6"}}, t + band)
  return mix(c, warm, g * smoothstep(0.2, 0.9, t) * 0.9)
end
skym = above(function(x) return farline(x) + 10 end)
work(skym, {hand="broad", color=skycol, angle=function(x, y) return 0.02 * skyn(x * 2, y) end, coverage=4.5, medium=0.28, length={80, 240}})
blend(skym, {angle=0.01})

--@ chunk 4 · clock 0

work(skym, {hand="broad", color=skycol, angle=function(x, y) return 0.015 * skyn(x * 1.5, y + 90) end, angle_jitter=0.03, coverage=3.2, medium=0.3, length={160, 380}, pressure={0.6, 0.45}})
blend(skym, {angle=0.0, coverage=2.5})
blend(skym, {angle=0.03, coverage=1.5})

--@ chunk 5 · clock 0
wait(24*60)
local bandn = noise{seed=5, octaves=5, period=26}
BAND = (below(function(x) return farline(x) + 1.5 * bandn(x, 0) end) * above(function(x) return HZ + 4 end)):roughen(0.6, 9, 3, 0.5)
work(BAND, {hand="body", tool="filbert 2.5", length={5, 16}, coverage=4.5, medium=0.2, angle=0.0, angle_jitter=0.08, pressure={0.7, 0.6},
  edge={found=0.15, soft=0.6, lost=0.25, period=30, seed=4},
  color=function(x, y)
    local g = math.exp(-((x - GLOW) / 300)^2)
    return mix(mix("#7b7580", "#978c8b", g), "#a7a3aa", smoothstep(HZ - 14, HZ + 4, y) * 0.55)
  end})
blend(BAND, {angle=0, coverage=1.2})

--@ chunk 6 · clock 1440
dry()
local roofs = poly({{695,HZ+1},{695,HZ-7},{702,HZ-14.5},{709,HZ-7},{710,HZ-5},{712,HZ-10.5},{716,HZ-5},{719,HZ-6},{725,HZ-12.5},{731,HZ-6},{731,HZ+1}})
local tower = poly({{736.6,HZ+1},{736.8,HZ-21},{742.4,HZ-21},{742.6,HZ+1}})
work(roofs + tower, {hand="detail", tool="round 1", length={2, 5}, coverage=4, medium=0.18, angle=1.57, color="#5f5b66", edge={found=0.8, soft=0.2, period=10, seed=2}, aim="masstone"})
local sp = brush{kind="round", width=1.8, point=1}
sp:load("#5d5963", 0.9)
sp:stroke({{739.6, HZ - 19}, {739.5, HZ - 30}, {739.4, HZ - 41}}, {pressure={0.95, 0.0}, ramps={0.0, 0.9}})
sp:stroke({{738.2, HZ - 21}, {739.4, HZ - 33}}, {pressure={0.8, 0.0}})
sp:stroke({{741.0, HZ - 21}, {739.6, HZ - 33}}, {pressure={0.8, 0.0}})

--@ chunk 7 · clock 32842.828125
STREAM = {{112,714},{165,690},{245,664},{322,643},{372,626},{392,611},{384,598},{360,588},{356,577},{380,566},{428,553},{470,540},{496,528},{498,517},{516,506},{546,496},{568,489},{580,484}}
function stream_w(y) return 3 + 52 * ((y - HZ) / (714 - HZ))^1.5 end
-- the stream as a band
local ws = {} for i, p in ipairs(STREAM) do ws[i] = stream_w(p[2]) end
STREAMM = ribbon(STREAM, ws):roughen(1.2, 14, 8, 0.6)
drift = noise{seed=23, octaves=4, period=160, stretch={0.05, 3.5}}
local fine = noise{seed=24, octaves=3, period=40, stretch={0.05, 3}}
function snowcol(x, y)
  local d = clamp((y - HZ) / (H - HZ), 0, 1)
  local g = math.exp(-((x - GLOW) / 380)^2)
  local c = gradient({{0, "#a9a6af"}, {0.12, "#b6b3ba"}, {0.45, "#c6c3c6"}, {1, "#d4d0cc"}}, d)
  c = mix(c, "#d8cfc0", 0.35 * g * (1 - d * 0.5))                   -- the glow's warmth on the far snow
  local sh = drift(x, y) * 0.5 + 0.5
  c = mix(c, shift(c, -0.05, -0.002, -0.022), smoothstep(0.55, 0.85, sh) * (0.3 + 0.7 * d))  -- lee of the drifts
  return shift(c, 0.012 * fine(x, y), 0, 0)
end
SNOW = below(function(x) return HZ + 1.2 + 0.8 * farn(x * 3, 5) end)
-- a thin underpainting over all the land, stream included
work(SNOW, {hand="broad", tool="flat 10", color=function(x, y) return shift(snowcol(x, y), -0.08, 0.0, -0.01) end, medium=0.35, coverage=4.5, length={60, 200}, angle=0.03, angle_jitter=0.04})
blend(SNOW, {angle=0.02, coverage=1.5})

--@ chunk 8 · clock 32842.828125
wait(24*60)
local snow = SNOW - STREAMM:shrink(1.5)
work(snow, {hand="body", tool="filbert 6", color=snowcol, medium=0.14, coverage=5.5, load=0.95, length={24, 80},
  angle=function(x, y) local d = clamp((y - HZ) / (H - HZ), 0, 1); return 0.02 + d * 0.22 * drift(x * 1.3, y + 40) end,
  angle_jitter=0.06, pressure={0.8, 0.6}})
blend(snow * below(function(x) return HZ + 40 end), {angle=0.02, coverage=1.2})

--@ chunk 9 · clock 34282.828125
local icen = noise{seed=31, octaves=4, period=30, stretch={0.0, 4}}
function icecol(x, y)
  local d = clamp((y - HZ) / (H - HZ), 0, 1)
  local c = gradient({{0, "#d9ccb4"}, {0.15, "#c3bdb6"}, {0.45, "#9ea2ae"}, {1, "#7f8798"}}, d)
  c = mix(c, "#cfd0d4", 0.35 * smoothstep(0.2, 0.7, icen(x, y)))   -- snow dusted on the ice
  return c
end
work(STREAMM, {hand="body", tool="filbert 3", color=icecol, medium=0.2, coverage=4.5, length={8, 30}, angle=0.0, angle_jitter=0.1,
  edge={found=0.3, soft=0.5, lost=0.2, period=35, seed=9}})
-- open water in a hole in the ice by the near willow
HOLE = poly({{232,670},{252,662},{278,654},{300,647},{312,648},{300,655},{276,662},{250,670},{232,675}}, true)
work(HOLE * STREAMM, {hand="body", tool="filbert 2.5", color=function(x, y) return mix("#2f3440", "#4a4f5c", smoothstep(625, 660, y)) end,
  medium=0.18, coverage=4, length={6, 20}, angle=-0.35, edge={found=0.6, soft=0.4, period=20, seed=3}})

--@ chunk 10 · clock 34282.828125
wait(8*60)
local function k(y) return 1 + 7 * clamp((y - HZ) / (H - HZ), 0, 1) end
FARBANK = mask(function(x, y) return STREAMM:at(x, y) * (1 - STREAMM:at(x, y - k(y))) end):roughen(0.6, 8, 12, 0.4)
NEARLIP = mask(function(x, y) return STREAMM:at(x, y) * (1 - STREAMM:at(x, y + 0.6 * k(y))) end):roughen(0.8, 10, 13, 0.4)
work(FARBANK, {hand="detail", tool="round 1.5", length={4, 14}, coverage=3.5, medium=0.18, angle=0, angle_jitter=0.2,
  color=function(x, y) return mix("#8c8c9c", "#6f7385", smoothstep(HZ, H, y)) end, edge={found=0.2, soft=0.5, lost=0.3, period=25, seed=5}})
work(NEARLIP, {hand="detail", tool="round 1.8", length={5, 16}, coverage=3.5, medium=0.12, angle=0, angle_jitter=0.25,
  color=function(x, y) return shift(snowcol(x, y), 0.02, 0, 0.004) end, edge={found=0.4, soft=0.4, lost=0.2, period=22, seed=6}})

--@ chunk 11 · clock 34762.828125
dry()
AIR = "#a9a5ad"
function per_m(y) return (y - HZ) / 1.7 end
-- one pollard willow, by hand: trunk outline, dark body, rim of glow on the left, rods from the knuckle
function willow(x, y, lean, nrods, seed, haze)
  local s = per_m(y)
  local th = 1.9 * s * (0.9 + 0.2 * rand())
  local fw = 0.34 * s
  local function P(dx, h, c) return {x + dx + lean * h, y - h, c} end
  local pts = {
    {x - fw * 1.35, y + 0.06 * s},
    P(-fw * 1.0, 0.25 * th), P(-fw * 0.85, 0.55 * th), P(-fw * 1.05, 0.78 * th),
    P(-fw * 1.75, 0.9 * th, "c"), P(-fw * 1.25, 1.04 * th), P(-fw * 0.5, 1.09 * th, "c"), P(fw * 0.2, 1.05 * th),
    P(fw * 0.8, 1.1 * th, "c"), P(fw * 1.7, 0.95 * th, "c"), P(fw * 1.05, 0.8 * th), P(fw * 0.9, 0.5 * th), P(fw * 1.05, 0.22 * th),
    {x + fw * 1.45, y + 0.06 * s}}
  local o = outline{pts=pts, char="broken", seed=seed, amount=0.8}
  local m = o:mask()
  local dark = mix("#35302c", AIR, haze)
  work(m, {hand="body", tool=string.format("filbert %.1f", math.max(1.2, fw * 0.5)), length={0.1 * th, 0.35 * th}, coverage=4, medium=0.15,
    angle=1.57 + lean, angle_jitter=0.15, color=function(xx, yy) return mix(dark, mix("#4d433b", AIR, haze), 0.5 + 0.5 * math.sin(xx * 1.7 + yy * 0.3)) end,
    edge={found=0.8, soft=0.2, period=15, seed=seed}})
  -- rim of light from the glow on the left flank
  local rim = m * mask(function(xx, yy) return 1 - m:at(xx - math.max(0.8, fw * 0.18), yy) end)
  work(rim, {hand="detail", tool="round 0.8", length={2, 7}, coverage=1.2, medium=0.2, angle=1.57 + lean, broken=0.6,
    color=mix("#6d6158", AIR, haze * 0.6)})
  -- rods: from a few knobs on the head, outer ones lean out and curve back up, lengths very uneven
  local hx, hy = x + lean * th, y - 1.06 * th
  local knobs = {}
  for k = 1, 5 do knobs[k] = {hx + (k - 3) * fw * 0.6 + randn(0, fw * 0.12), hy + math.abs(k - 3) * 0.05 * th + randn(0, 0.02 * th)} end
  local rodc = mix("#2c2724", AIR, haze)
  for pass = 1, 2 do
    local rb = brush{kind="rigger", width=math.max(0.5, (pass == 1 and 0.06 or 0.035) * s), point=1}
    local n = pass == 1 and nrods or math.floor(nrods * 0.8)
    for i = 1, n do
      if i % 5 == 1 then rb:reload(mix(rodc, "#4a3d33", rand() * 0.35), 0.95) end
      local kn = knobs[math.random(1, 5)]
      local u = clamp((kn[1] - hx) / (fw * 1.4) + randn(0, 0.35), -1.2, 1.2)
      local a0 = -1.5708 + u * 0.85 + randn(0, 0.1)
      local bend = -u * 0.55 + randn(0, 0.08)             -- curving back toward the vertical
      local len = s * 4.6 * rand(0.3, 1.0)^0.7 * (pass == 1 and 1 or 0.6) * (1 - 0.25 * math.abs(u))
      local p = {{kn[1] + randn(0, fw * 0.1), kn[2]}}
      local px, py = p[1][1], p[1][2]
      for k = 1, 5 do
        local aa = a0 + bend * (k / 5)^1.3
        px = px + math.cos(aa) * len / 5; py = py + math.sin(aa) * len / 5
        p[#p + 1] = {px + randn(0, 0.15), py}
      end
      rb:stroke(p, {pressure={0.9, 0.0}, ramps={0.02, 0.75}, shake=0.5})
    end
  end
  return m, hx, hy, th, fw
end
W1 = {willow(455, 572, 0.05, 60, 101, 0.0)}
W2 = {willow(532, 521, -0.04, 50, 102, 0.22)}
W3 = {willow(577, 500, 0.06, 36, 103, 0.4)}
W4 = {willow(603, 491.5, -0.02, 26, 104, 0.55)}
W5 = {willow(621, 486.5, 0.03, 18, 105, 0.66)}

--@ chunk 12 · clock 112010.5
W0 = {willow(852, 616, -0.07, 95, 100, 0.0)}
-- a split in the old trunk: a dark cleft with a pale torn edge
local cx = 852 - 0.07 * 60
local cleft = outline{pts={{cx - 4, 606}, {cx - 7, 580}, {cx - 3, 548}, {cx - 9, 520, "c"}, {cx - 2, 512}, {cx + 2, 546}, {cx + 1, 580}, {cx + 2, 606}}, char="broken", seed=77}
work(cleft:mask(), {hand="detail", tool="round 1.4", length={4, 12}, coverage=4, medium=0.15, angle=1.6, color="#1f1b1a", edge="firm"})
local tb = brush{kind="round", width=1.2, point=1}
tb:load("#77695c", 0.8)
local pp = cleft:path(1)
tb:stroke({pp[1], pp[2], pp[3], pp[4]}, {pressure={0.5, 0.1}, shake=0.6})

--@ chunk 13 · clock 112010.5
FEET = {{852,616,-0.07,0.0,W0}, {455,572,0.05,0.0,W1}, {532,521,-0.04,0.22,W2}, {577,500,0.06,0.4,W3}, {603,491.5,-0.02,0.55,W4}, {621,486.5,0.03,0.66,W5}}
function whips(Wt, n, haze, lean)
  local m, hx, hy, th, fw = Wt[1], Wt[2], Wt[3], Wt[4], Wt[5]
  local s = th / 1.9
  local rb = brush{kind="rigger", width=math.max(0.45, 0.025 * s), point=1}
  for i = 1, n do
    if i % 7 == 1 then rb:reload(mix(mix("#302a27", "#54463b", rand() * 0.5), AIR, haze), 0.8) end
    local u = clamp(randn(0, 0.5), -1.05, 1.05)
    local sx, sy = hx + u * fw * 1.1, hy + math.abs(u) * 0.06 * th + rand(-0.03, 0.05) * th
    local a0 = -1.5708 + u * 0.8 + randn(0, 0.12)
    local bend = -u * 0.5 + randn(0, 0.1)
    local len = s * 4.4 * rand(0.35, 1.0)^0.6
    local p = {{sx, sy}}
    local px, py = sx, sy
    for k = 1, 6 do
      local aa = a0 + bend * (k / 6)^1.3
      px = px + math.cos(aa) * len / 6; py = py + math.sin(aa) * len / 6
      p[#p + 1] = {px + randn(0, 0.12), py}
    end
    rb:stroke(p, {pressure={0.7, 0.0}, ramps={0.02, 0.85}, shake=0.6})
  end
end
function bark(Wt, haze, lean, seed)
  local m, hx, hy, th, fw = Wt[1], Wt[2], Wt[3], Wt[4], Wt[5]
  local s = th / 1.9
  local x0, y0 = hx - lean * th, hy + 1.06 * th
  -- darken the body again in its own paint, aimed as masstone so it is truly dark
  work(m:grow(0.5), {hand="body", tool=string.format("round %.1f", math.max(1, fw * 0.25)), length={0.1 * th, 0.3 * th}, coverage=2.5, medium=0.12, aim="masstone",
    angle=1.57 + lean, angle_jitter=0.2, edge={found=0.6, soft=0.4, period=12, seed=seed}, color=mix("#2a2522", AIR, haze), load=0.9})
  if haze > 0.3 then return end
  -- fissures and ridges, wandering down the trunk
  local fb = brush{kind="round", width=math.max(0.6, fw * 0.09), point=0.7}
  local lb = brush{kind="round", width=math.max(0.6, fw * 0.1), point=0.7}
  local nf = math.floor(fw / 1.3)
  for i = 1, nf do
    local u = rand(-0.9, 0.9)
    local ytop = y0 - th * rand(0.6, 1.0); local ybot = y0 - th * rand(0.0, 0.25)
    local p = {}
    for k = 0, 6 do local f = k / 6; local yy = ytop + (ybot - ytop) * f; p[#p + 1] = {x0 + lean * (y0 - yy) + u * fw * (0.9 + 0.2 * math.sin(f * 3 + i)) + randn(0, 0.4), yy} end
    if false then
      lb:reload(mix(mix("#6f6459", "#8a7b6c", rand()), AIR, haze), 0.6); lb:stroke(p, {pressure={0.6, 0.15}, shake=0.9, clip=m})
    else
      fb:reload(mix("#15120f", AIR, haze), 0.8); fb:stroke(p, {pressure={0.7, 0.3}, shake=0.8, clip=m})
    end
  end
  -- knobs on the knuckle: clusters of dark dabs, a lit dab or two on the upper left of each
  for i = 1, 5 do
    local bx, by = hx + rand(-1.3, 1.3) * fw, hy + rand(0.0, 0.14) * th
    local r = fw * rand(0.1, 0.22)
    fb:reload("#1a1614", 0.8)
    for k = 1, 4 do fb:touch(bx + randn(0, r * 0.5), by + randn(0, r * 0.35), {pressure=rand(0.5, 0.9), clip=m}) end
  end
end
for i, f in ipairs(FEET) do bark(f[5], f[4], f[3], 200 + i) end
whips(W0, 160, 0.0); whips(W1, 110, 0.0); whips(W2, 70, 0.22); whips(W3, 40, 0.4); whips(W4, 24, 0.55); whips(W5, 16, 0.66)

--@ chunk 14 · clock 112010.5
dry()
-- a second coat on the trunks: the first dried thin on the weave's crests
for i, f in ipairs(FEET) do
  local m, fw, th = f[5][1], f[5][5], f[5][4]
  work(m, {hand="body", tool=string.format("filbert %.1f", math.max(1, fw * 0.3)), length={0.1 * th, 0.3 * th}, coverage=3.5, medium=0.1, aim="masstone", load=1,
    angle=1.57 + f[3], angle_jitter=0.15, edge={found=0.8, soft=0.2, period=12, seed=i}, color=mix(i % 2 == 0 and "#2b2623" or "#302925", AIR, f[4])})
end
dry()
local capn = noise{seed=61, octaves=3, period=5}
function ridges(Wt, haze, lean)
  local m, hx, hy, th, fw = Wt[1], Wt[2], Wt[3], Wt[4], Wt[5]
  local x0, y0 = hx - lean * th, hy + 1.06 * th
  local lb = brush{kind="round", width=math.max(0.6, fw * 0.07), point=0.5}
  local n = math.floor(fw / 1.6)
  for i = 1, n do
    local u = clamp(randn(-0.4, 0.45), -1, 1)
    local ytop = y0 - th * rand(0.4, 1.0); local ybot = ytop + th * rand(0.15, 0.4)
    local p = {}
    for k = 0, 5 do local f = k / 5; local yy = ytop + (ybot - ytop) * f
      p[#p + 1] = {x0 + lean * (y0 - yy) + u * fw * (0.95 + 0.1 * math.sin(f * 4 + i)) + randn(0, 0.25), yy} end
    lb:reload(mix(mix("#4d443d", "#62574d", rand()), AIR, haze), rand(0.08, 0.18))
    lb:stroke(p, {pressure={0.3, 0.1}, ramps={0.2, 0.5}, shake=1.0, clip=m})
  end
  -- snow lying in patches on the knuckle's upper faces
  local cap = m * mask(function(xx, yy) return (1 - m:at(xx, yy - math.max(1, fw * 0.12))) * smoothstep(0.05, 0.35, capn(xx * 3 / math.max(1, fw * 0.3), yy)) end) * rect(hx - fw * 2, hy - th * 0.2, fw * 4, th * 0.3)
  work(cap, {hand="detail", tool=string.format("round %.1f", math.max(0.6, fw * 0.1)), length={1, fw * 0.4}, coverage=2.4, medium=0.12, angle=0.1, angle_jitter=0.5,
    color=mix("#cfcacb", AIR, haze * 0.5), edge={found=0.2, soft=0.5, lost=0.3, period=5, seed=3}})
end
for i, f in ipairs(FEET) do if f[4] < 0.5 then ridges(f[5], f[4], f[3]) end end
-- snow drifted against every foot, in the field's own color, climbing the trunk unevenly
for i, f in ipairs(FEET) do
  local x, y, fw = f[1], f[2], f[5][5]
  local s = per_m(y)
  local top = function(xx) local u = (xx - x) / (fw * 2.2); return y - 0.09 * s * math.max(0, 1 - u * u) + 0.03 * s * capn(xx * 0.4 + 30 * i, i) end
  local zone = (below(top) * rect(x - fw * 2.6, y - 0.2 * s, fw * 5.2, 0.5 * s)):roughen(0.03 * s, 0.25 * s, 50 + i, 0.3)
  work(zone, {hand="body", tool=string.format("filbert %.1f", math.max(1.0, 0.05 * s)), length={0.15 * s, 0.5 * s}, coverage=3.2, medium=0.12,
    angle=function(xx, yy) return 0.25 * (xx - x) / fw * 0.3 end, angle_jitter=0.2, color=function(xx, yy) return shift(snowcol(xx, yy + 0.3 * s), -0.015, 0, -0.004) end,
    edge={found=0.3, soft=0.4, lost=0.3, period=0.3 * s, seed=i}})
end

--@ chunk 15 · clock 163287.080078125
FX, FY = 674, 522
TRACK = {{770,714},{742,664},{712,610},{692,568},{680,540},{674,522},{671,508},{676,497},{694,488},{716,481.5}}
KF = 1.7 * 500 / math.tan(math.rad(25))     -- ground: y - HZ = KF / Z (fov 50 over the width)
-- footprints in two broken lines, each step 0.36 m on the ground: distinct near, a trodden furrow far off
local side = 1
for i = 1, #TRACK - 1 do
  local a, b = TRACK[i], TRACK[i + 1]
  local dx, dy = b[1] - a[1], b[2] - a[2]
  local L = math.sqrt(dx * dx + dy * dy)
  local t = rand(0, 1)
  while t < L do
    local x, y = a[1] + dx * t / L, a[2] + dy * t / L
    local s = per_m(y)
    local hidden = (i == 5 or i == 6) and y < FY + 1 and y > FY - 14    -- under the walker
    if s > 3 and not hidden then
      local cx, cy = x + side * 0.1 * s + randn(0, 0.03 * s), y + randn(0, 0.01 * s)
      local w = 0.12 * s * rand(0.75, 1.2)
      local fb = brush{kind="filbert", width=math.max(0.6, 0.04 * s), stiffness=0.4}
      fb:load(shift(snowcol(cx, cy), -0.06 - 0.03 * rand(), -0.002, -0.022), 0.5)
      fb:stroke({{cx - w / 2, cy - 0.006 * s}, {cx + randn(0, 0.02 * s), cy + 0.012 * s}, {cx + w / 2, cy - 0.004 * s}}, {pressure={0.5, 0.35}, shake=0.6})
      if s > 25 and rand() < 0.7 then
        local lb = brush{kind="round", width=math.max(0.5, 0.02 * s), point=0.4}
        lb:load(shift(snowcol(cx, cy), 0.03, 0, 0.006), 0.5)
        lb:stroke({{cx - w * 0.4, cy - 0.03 * s}, {cx + w * 0.45, cy - 0.034 * s}}, {pressure={0.45, 0.2}, shake=0.6})
      end
    end
    side = -side
    local dz = 0.36 * rand(0.85, 1.15)
    local ystep = dz * (y - HZ)^2 / KF                   -- canvas y for that step on the ground
    local slope = math.abs(dy) / L
    t = t + math.max(0.35, ystep / math.max(0.3, slope))
  end
end

--@ chunk 16 · clock 163287.080078125
-- the walker, seen from behind, going toward the village: greatcoat, tall hat, stick
local k = per_m(FY) / 26          -- the drawing below is in units at 26 per meter
local function R(pts) local o = {} for i, p in ipairs(pts) do o[i] = {FX + p[1] * k, FY + p[2] * k} end return o end
COAT = R{{-4.8,-38.2},{-6.2,-35},{-6.5,-27},{-7.0,-18},{-7.9,-9.0},{-4,-8.2},{0.2,-9.0},{3.6,-8.9},{7.2,-10.2},{6.6,-18},{6.0,-27},{5.9,-35},{4.7,-38.2},{2.2,-39.6},{-2.0,-39.6}}
HAT = R{{-4.0,-43.2},{4.1,-43.4},{3.9,-44.3},{2.7,-44.6},{2.5,-48.6},{0,-49},{-2.4,-48.7},{-2.7,-44.5},{-3.9,-44.2}}
LEGL = R{{-3.4,-9},{-1.1,-9},{-1.6,-0.3},{-3.7,0.1}}
LEGR = R{{1.2,-9},{3.4,-9.2},{4.6,-3.4},{4.9,-2.2},{3.0,-1.9},{2.3,-3.2}}
local coat, hat, head = poly(COAT, true), poly(HAT), ellipse(FX + 0.3 * k, FY - 41.2 * k, 2.4 * k, 3.0 * k)
local legs = poly(LEGL) + poly(LEGR)
FIGM = coat + hat + head + legs
work(legs, {hand="detail", tool="round 1", length={2, 5}, coverage=4, medium=0.12, angle=1.57, aim="masstone", color="#1f1d20", edge={found=0.8, soft=0.2, period=8, seed=1}})
work(coat, {hand="detail", tool="round 1.4", length={3, 9}, coverage=4.5, medium=0.12, angle=1.62, angle_jitter=0.15, aim="masstone",
  color=function(x, y) return mix("#262a35", "#1c1e25", smoothstep(FY - 38 * k, FY - 9 * k, y)) end, edge={found=0.7, soft=0.3, period=10, seed=2}})
work(head, {hand="detail", tool="round 1", length={1, 3}, coverage=4, medium=0.12, aim="masstone", color="#2c2526", edge="firm"})
work(hat, {hand="detail", tool="round 1", length={1.5, 4}, coverage=4, medium=0.12, angle=0, aim="masstone", color="#18171a", edge={found=0.9, soft=0.1, period=6, seed=3}})
-- the arm and the stick, the collar's fold, a lit edge of glow on his left shoulder
local ab = brush{kind="round", width=1.8 * k, point=0.5}
ab:load("#22242c", 0.9)
ab:stroke(R{{6.2,-36}, {7.8,-30}, {8.6,-24.5}}, {pressure={0.8, 0.7}})
local sb = brush{kind="rigger", width=0.7 * k, point=1}
sb:load("#2b2420", 0.9)
sb:stroke(R{{8.8,-25}, {10.6,-12}, {12.2,-0.2}}, {pressure={0.8, 0.55}, shake=0.3})
local rb = brush{kind="round", width=0.7 * k, point=1}
rb:load("#6a6268", 0.6)
rb:stroke(R{{-5.4,-37.6}, {-6.8,-34}, {-7.2,-28}}, {pressure={0.45, 0.05}})
rb:load("#595158", 0.5)
rb:stroke(R{{-3.6,-43.4}, {-2.2,-43.5}}, {pressure={0.35, 0.1}})
-- his shadow: at dusk only a faint cool trace, and snow over his boot soles
local sh = brush{kind="filbert", width=2.2 * k, stiffness=0.4}
sh:load(shift(snowcol(FX, FY), -0.06, -0.002, -0.02), 0.4)
sh:stroke(R{{-5,0.4}, {0,0.9}, {6,0.5}}, {pressure={0.5, 0.3}})
local sn = brush{kind="filbert", width=1.2 * k, stiffness=0.4}
sn:load(snowcol(FX, FY + 2), 0.6)
sn:stroke(R{{-4.6,0.1}, {-2.5,-0.5}, {-0.6,0.2}}, {pressure={0.6, 0.4}})
sn:stroke(R{{1.0,-1.4}, {2.6,-1.9}, {4.0,-1.2}}, {pressure={0.4, 0.2}})
