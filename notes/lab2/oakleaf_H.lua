-- easel session "oakleaf_H": a painting replayed chunk by chunk.
--   easel run notes/lab2/oakleaf_H.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
-- Round 6 lab 2, subject "oakleaf": a broad summer oak in the middle distance (the crown about a
-- third of the canvas wide) standing in a meadow, a low line of distant trees behind, soft
-- daylight. Shared setup for oakleaf_A (round 1's winning detailed recipe) and oakleaf_H
-- (hierarchical detail): canvas, world and sun, sky, the far tree line, the meadow, the oak
-- drawn and grown, its cast shadow.
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=3/2, size=440, seed=71}
HZ = 432
WORLD = world{horizon=HZ, sun={azimuth=-125, elevation=40}}
SUN = {-0.6, -0.6, 0.45}
-- soft daylight: a pale blue-gray summer sky, milky and warm toward the horizon
sky = function(x, y)
  return gradient({{0, "#7a90ae"}, {0.4, "#9aabbd"}, {0.8, "#c3c8c3"}, {1, "#d3d1c0"}}, y / HZ)
end
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}
-- the far tree line: a low wood along the horizon with a few breaks (fields between groves)
local fn = noise{seed=5, period=60, octaves=3}
FARPTS = {}
for i = 0, 50 do
  local x = -10 + i * 20.4
  local h = 17 + 11 * fn(x, 0) + 9 * math.exp(-((x - 600) / 60)^2) + 6 * math.exp(-((x - 60) / 50)^2)
  if x > 165 and x < 225 then h = -6 end                  -- breaks: open field seen through
  if x > 706 and x < 736 then h = 3 end
  if x > 860 and x < 930 then h = h * 0.35 end
  FARPTS[#FARPTS + 1] = {x, HZ + 2 - h}
end
farline = outline{pts=FARPTS, open=true, char="soft", lobe=10, seed=9}
FAR = farline:below(HZ + 6) * above(function(x) return HZ + 5 + 1.5 * math.sin(x / 50) end)
-- the meadow: hazy and light at the horizon, deeper and warmer toward us
land = below(function(x) return HZ + 2 * math.sin(x / 80) end)
landcol = function(x, y)
  local t = smoothstep(HZ, H, y)
  return gradient({{0, "#8f9570"}, {0.25, "#7f8a55"}, {0.6, "#66743c"}, {1, "#4c5a2c"}}, t)
end
-- the oak: broad, lopsided (heavier and lower on the left), lobed, a notch in the top right
CROWN = {{292,352},{272,318},{280,282},{300,256},{306,226},{334,204},{366,196},{388,172},{426,160},
         {462,168},{486,186,"c"},{512,168},{552,172},{580,192},{604,220},{606,252},{626,280},
         {620,314},{600,338},{606,358,"c"},{580,376},{546,372},{520,386},{480,378},{452,390},
         {420,380},{384,386},{350,374},{318,378}}
TRUNK = {{452,474},{450,440},{447,405},{446,380}}
crown = outline{pts=CROWN, char="soft", seed=72}
tree = tree_in{crown=crown, trunk=TRUNK, species="oak", season="summer", sun=WORLD, seed=73}
print(tree, "touch_w", tree.touch_w, "grain", tree.grain)
-- its shadow on the meadow, thrown right and a little toward us
SHADOW = ellipse(520, 478, 118, 11):roughen(4, 30, 3, 3)
LEAF = {deep="#212a1c", body="#34402a", sunny="#5c6a3a", shade="#263020", skyshade="#4a5446",
        mid="#46522e", lit="#6f7c45", top="#98a060"}
WOOD = {dark="#3b342c", light="#7d7566"}

--@ chunk 2 · clock 0
-- H chunk 2: the plan, then the sky and the meadow.
-- Hierarchy for a tree at this distance: (1) a few BIG masses with an irregular silhouette and
-- value families (the sun side light, the shade side with internal steps); (2) a MIDDLE scale of
-- leaf groups only where the light is; (3) a few particulars: accents, one or two gaps.
local lv = tree:leaves()
-- the silhouette: the grown leaves' own lobes, one solid inside
local edge = lv:blur(2):map(function(v) return smoothstep(0.3, 0.55, v) end)
local core = lv:blur(14):map(function(v) return smoothstep(0.15, 0.35, v) end):shrink(14)   -- solid: no holes but the drawn ones
-- two gaps only: where the left scaffold limb parts the masses, and a small one on the right edge
GAPS = (ellipse(373, 313, 12, 8) + ellipse(592, 262, 7, 5)):roughen(4, 8, 5, 1.0)
MASS = edge + core - GAPS
-- five big masses drawn by hand, unequal, back to front (the lower ones hang in front)
MPTS = {
  {{498,190},{522,168},{562,170},{598,194},{614,230},{612,268},{590,296},{556,300},{520,284},{494,256},{482,222}},  -- top right, turning into shade
  {{292,212},{330,184},{388,162},{432,156},{474,166},{500,192},{494,230},{470,250},{430,256},{395,275},{350,292},{305,280},{280,250}}, -- the big sun mass
  {{560,300},{600,292},{628,310},{630,340},{614,362},{592,378},{548,382},{525,360},{535,325}},                   -- right low, deep shade
  {{262,310},{276,284},{318,284},{360,296},{392,316},{392,345},{360,368},{318,378},{280,360}},                   -- left low, hanging
  {{410,280},{440,252},{478,244},{516,262},{548,300},{550,340},{530,370},{492,390},{440,392},{404,372},{392,330}}} -- center, in front, rising high
MM = {}
for i, p in ipairs(MPTS) do MM[i] = poly(p, true):roughen(6, 22, 40 + i, 2):grow(6) end
-- each mass a dome over its own irregular shape: from its distance field, the normal faces us on
-- the mass's ridge and turns outward toward its edge (so the whole mass turns, not only a rim)
MB, MD, MR = {}, {}, {}
for i, m in ipairs(MM) do
  MB[i] = m:blur(6)
  MD[i] = m:distance()
  local r = 0
  for _, q in ipairs(MPTS[i]) do r = r + 0 end
  local sx, sy = 0, 0
  for _, q in ipairs(MPTS[i]) do sx, sy = sx + q[1], sy + q[2] end
  MR[i] = math.max(20, 1.1 * MD[i]:at(sx / #MPTS[i], sy / #MPTS[i]))
end
local S = {-0.6, -0.6, 0.45}
local sl = math.sqrt(S[1]^2 + S[2]^2 + S[3]^2)
S = {S[1] / sl, S[2] / sl, S[3] / sl}
local Lb, Mb = tree:light():blur(30), lv:blur(30)
BIG = mask(function(x, y) local m = Mb:at(x, y) if m < 0.05 then return 0 end return clamp(Lb:at(x, y) / m, 0, 1) end)
GS = 3
local x0, y0, x1, y1 = 240, 140, 660, 410
local nx, ny = math.ceil((x1 - x0) / GS), math.ceil((y1 - y0) / GS)
local FV, FF, FK, FU = {}, {}, {}, {}
for j = 0, ny do
  for i = 0, nx do
    local x, y = x0 + i * GS, y0 + j * GS
    -- the front mass here: the last one (in front) that covers it, else the nearest
    local k, best = nil, -1
    for q = #MM, 1, -1 do if MM[q]:at(x, y) > 0.5 then k = q break end end
    if not k then for q = 1, #MM do local v = MB[q]:at(x, y) if v > best then best, k = v, q end end end
    local D = MD[k]
    local d = D:at(x, y)
    local gx, gy = (D:at(x + 3, y) - D:at(x - 3, y)) / 6, (D:at(x, y + 3) - D:at(x, y - 3)) / 6
    local gl = math.sqrt(gx * gx + gy * gy) + 1e-6
    local u = clamp(1 - d / MR[k], 0, 0.98)
    local nxv, nyv, nz = -gx / gl * u, -gy / gl * u, math.sqrt(1 - u * u)
    local sun = math.max(0, nxv * S[1] + nyv * S[2] + nz * S[3])
    local skyl = clamp(0.5 - 0.7 * nyv, 0, 1)
    -- the recess above a mass that hangs in front: dark
    local c = 0
    for q = k + 1, #MM do c = math.max(c, MB[q]:at(x, y + 9) * (1 - MM[q]:at(x, y))) end
    local id = (y1 > 0) and (j * (nx + 1) + i) or 0
    FV[id], FF[id], FK[id], FU[id] = sun, skyl, smoothstep(0.15, 0.5, c), k
  end
end
local function bil(t, x, y)
  local fx, fy = clamp((x - x0) / GS, 0, nx - 1e-3), clamp((y - y0) / GS, 0, ny - 1e-3)
  local i, j = math.floor(fx), math.floor(fy)
  local u, w = fx - i, fy - j
  local a, b = t[j * (nx + 1) + i], t[j * (nx + 1) + i + 1]
  local c, d = t[(j + 1) * (nx + 1) + i], t[(j + 1) * (nx + 1) + i + 1]
  return (a * (1 - u) + b * u) * (1 - w) + (c * (1 - u) + d * u) * w
end
SUNF = function(x, y) return bil(FV, x, y) end
SKYF = function(x, y) return bil(FF, x, y) end
CREASE = function(x, y) return bil(FK, x, y) end
MASSID = function(x, y)
  local fx, fy = clamp(math.floor((x - x0) / GS + 0.5), 0, nx), clamp(math.floor((y - y0) / GS + 0.5), 0, ny)
  return FU[fy * (nx + 1) + fx]
end
-- each mass's own share of the sun (the crown-scale light at its center): masses, not one gradient
MSUN = {}
for i, p in ipairs(MPTS) do
  local sx, sy = 0, 0
  for _, q in ipairs(p) do sx, sy = sx + q[1], sy + q[2] end
  MSUN[i] = BIG:at(sx / #p, sy / #p)
end
-- a painter's decision on top of the measured light: how far each mass is on the sun side
-- (the far top right turns away, the front center mass catches the light on its top)
MSIDE = {0.5, 1.0, 0.08, 0.85, 0.5}
print("mass sun", table.concat((function() local o = {} for i, v in ipairs(MSUN) do o[i] = string.format("%.2f", v) end return o end)(), " "))
-- the value the crown should have (0..1), in two families: the shade side (recess 0.1, body 0.2,
-- tops that see the sky 0.35) and the sun side (recess 0.3, body 0.5, turned to the sun 0.85);
-- which family is the mass's side of the crown, so the masses read as masses, not one gradient
local brk = noise{seed=51, period=40}
VAL = function(x, y)
  local k = MASSID(x, y)
  local side = clamp(0.7 * MSIDE[k] + 0.3 * smoothstep(0.12, 0.5, BIG:at(x, y)), 0, 1)
  local f = 0.75 * SUNF(x, y) + 0.25 * SKYF(x, y)
  -- the recess comes and goes along its length (a continuous dark seam read as a mouth)
  local c = CREASE(x, y) * smoothstep(-0.5, 0.3, brk(x, y))
  -- undersides get a little light back from the sunlit meadow
  local bounce = 0.05 * smoothstep(0.35, 0.1, SKYF(x, y))
  local shadeV = 0.11 + 0.24 * smoothstep(0.3, 0.8, SKYF(x, y)) - 0.08 * c + bounce
  local litV = 0.28 + 0.6 * smoothstep(0.35, 0.9, f) - 0.16 * c
  return clamp(shadeV + (litV - shadeV) * side, 0, 1)
end
TRUNKM = tree:wood(7) * mask(function(x, y) return smoothstep(372, 392, y) end)
-- the sky alla prima AROUND the reserved crown, trunk and far trees (a dark laid over open
-- sky plows a pale lace), two thin broad layers wet into wet and a light blend, no stipple
skym = above(function(x) return HZ + 6 + 2 * math.sin(x / 80) end)
local sk = skym - MASS:soften(0.8) - TRUNKM:shrink(1) - FAR:shrink(2.5)
work(sk, {hand="broad", color=sky, angle=0, coverage=5, medium=0.22, load=1, pal=skypal, clip=sk})
work(sk, {hand="broad", color=sky, angle=0.01, coverage=3.5, medium=0.25, load=0.9, pal=skypal, clip=sk})
blend(sk, {angle=0, coverage=1.5, clip=sk})
-- the meadow massed: a few big light and dark patches (a slow noise stretched along the ground),
-- not a gradient; the foot of the far trees kept for the far pass
local pn = noise{seed=17, period=180, stretch={0, 4}, octaves=2}
MEADOWCOL = function(x, y)
  local c = landcol(x, y)
  local t = smoothstep(HZ, H, y)
  local p = pn(x, y)
  return shift(c, 0.035 * p * (0.4 + t), 0, 0.01 * p)
end
-- the crown laid in thinly at once, a middle dark, edge to edge with the wet sky (a thin pass left
-- on bare ground showed the orange ground in flecks through the lights)
work(MASS, {hand="body", tool="filbert 6", length={10, 24}, coverage=3.2, medium=0.32, load=0.75, angle=0.4, angle_jitter=1.2,
  clip=MASS:grow(0.6), color=function(x, y) return mix("#2a3322", "#46522f", smoothstep(0.2, 0.7, VAL(x, y))) end})
-- then at once, wet into the lay-in, the crown's big masses: values in steps (families, not a smooth
-- ramp), the shade side cool with sky-lit tops, the sun side warm, strokes wrapping around their own
-- mass. (First laid 15 h later over the setting lay-in; moved here wet into wet. The wormy grain
-- seen then at 3200 turned out to be the varnish, not this: see oakleaf.md.)
local function steps(v)
  local s = 0.12
  local i = math.floor(v / s)
  local fr = (v - i * s) / s
  return (i + smoothstep(0.7, 1, fr)) * s
end
CROWNRAMP = {{0.0, "#161d13"}, {0.14, "#1e2819"}, {0.24, "#29331f"}, {0.34, "#343f2b"},
             {0.46, "#414e2c"}, {0.6, "#556337"}, {0.74, "#6b793f"}, {0.9, "#838e4b"}}
function CROWNCOL(x, y)
  local v = steps(VAL(x, y))
  local c = gradient(CROWNRAMP, v)
  -- the tops of the shade masses turn a little cool with the sky
  local k = MASSID(x, y)
  local cool = (1 - MSIDE[k]) * smoothstep(0.55, 0.85, SKYF(x, y))
  return shift(c, 0.004 * cool, -0.003 * cool, -0.009 * cool)
end
function WRAP(x, y)
  local D = MD[MASSID(x, y)]
  local gx, gy = D:at(x + 3, y) - D:at(x - 3, y), D:at(x, y + 3) - D:at(x, y - 3)
  return math.atan(gy, gx) + math.pi / 2
end
work(MASS, {hand="body", tool="filbert 6", length={8, 18}, coverage=3.6, medium=0.26, load=0.8, clip=MASS:grow(0.6),
  angle=WRAP, angle_jitter=0.45, curve={0.3, 0.1}, color=CROWNCOL})
work(TRUNKM, {hand="body", tool="filbert 4", length={10, 24}, coverage=3, medium=0.3, load=0.75, angle=1.55, clip=TRUNKM, color="#3a342d"})
local fv = noise{seed=23, period=70, octaves=2}
-- a restricted palette: with the full one the gray-green mix jittered into rust flecks
farpal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "green earth"}
FARCOL = function(x, y)
  local t = smoothstep(HZ - 26, HZ + 4, y)
  return shift(mix("#7b857f", "#68736a", t), 0.012 * fv(x, y), 0, 0)
end
-- the far trees laid in thinly at once, edge to edge with the wet sky (the reserved band was bare ground)
work(FAR:shrink(1) - TRUNKM, {hand="body", tool="filbert 6", length={10, 30}, coverage=3.5, medium=0.3, load=0.8, angle=0.05,
  clip=FAR - TRUNKM:grow(0.5), color=FARCOL, pal=farpal})
local ld = land - TRUNKM:shrink(1) - FAR:shrink(2)
work(ld, {hand="body", color=MEADOWCOL, angle=0.02, length={20, 60}, coverage=3.4, medium=0.18, clip=ld})
-- the oak's shadow worked into the wet meadow at once, so it has no cut edge
work(SHADOW:blur(3) * ld, {hand="body", tool="filbert 6", color=function(x, y) return shift(MEADOWCOL(x, y), -0.075, -0.002, -0.008) end,
  angle=0.02, length={10, 30}, coverage=2.8, medium=0.2, load=0.8, hug=false, clip=SHADOW:grow(3)})

--@ chunk 3 · clock 0
-- H chunk 3: 15 h later: the far trees' body over their thin lay-in, as one soft mass, and the trunk.
wait(900)
print("sky at the crown edge:", drying(270, 300), drying(600, 330), "far foot:", drying(100, 430))
-- the far trees as ONE soft mass: close to the sky in value, a little bluer and lighter at the
-- top where they stand against it, a touch darker at the foot, the breaks left as they fall
local fv = noise{seed=23, period=70, octaves=2}
work(FAR - TRUNKM, {hand="body", tool="filbert 5", length={8, 26}, coverage=4.6, medium=0.18, load=0.95, angle=0.05, angle_jitter=0.5,
  clip=FAR:grow(1.5) - TRUNKM:grow(0.5), color=FARCOL, pal=farpal})
-- a second pass across the first, short, to close the weave (the ground showed through in the hollows)
work(FAR:shrink(1.5) - TRUNKM, {hand="body", tool="filbert 4", length={5, 12}, coverage=2.6, medium=0.2, load=0.9, angle=1.3, angle_jitter=0.8,
  clip=FAR:grow(0.5) - TRUNKM:grow(0.5), color=FARCOL, pal=farpal})
-- the trunk: lit on its left, dark under the crown, a darker right side
work(TRUNKM, {hand="body", tool="filbert 4", length={8, 24}, coverage=4, load=0.8, angle=1.55, angle_jitter=0.12, clip=TRUNKM, medium=0.18,
  color=function(x, y)
    local c = mix("#4b453c", "#2a2622", smoothstep(440, 450, x))
    return mix(c, "#211e1a", smoothstep(410, 385, y))
  end})

--@ chunk 4 · clock 900
-- H chunk 4: the masses setting to tacky. The MIDDLE scale: leaf groups, only where the light
-- is. Each chosen clump of the grown tree becomes one group: a crescent of 3-6 blunt filbert dabs on
-- its sun side, a step lighter than the mass under it, so a lit mass breaks into groups that
-- thin out into its own shade. In the shade half only a sparse few cool groups on the mass tops.
wait(1500)   -- 40 h after the masses: at 15 h they were open and the groups sank into them
print("mass:", drying(400, 200), drying(580, 340))
local cl = tree.clumps
local groups, cool = {}, {}
-- groups gather in patches (a slow noise), leaving quieter stretches of plain mass between them
local patch = noise{seed=61, period=34}
for i, c in ipairs(cl) do
  if MASS:at(c.x, c.y) > 0.6 then
    local v = VAL(c.x, c.y)
    local p = smoothstep(0.4, 0.72, v) * (1 - CREASE(c.x, c.y)) * (0.55 + 0.45 * c.lit)
    if rand(0, 1) < p * 0.8 * smoothstep(-0.15, 0.35, patch(c.x, c.y)) then groups[#groups + 1] = {c, v}
    else
      local k = MASSID(c.x, c.y)
      local q = (1 - MSIDE[k]) * smoothstep(0.62, 0.9, SKYF(c.x, c.y)) * (1 - CREASE(c.x, c.y))
      if rand(0, 1) < q * 0.18 then cool[#cool + 1] = c end
    end
  end
end
print("groups", #groups, "cool", #cool)
local sx, sy = -0.7, -0.7
local dab = {brush("filbert", 3.2), brush("filbert", 3.8), brush("filbert", 4.4)}
local nd = 0
local shadowdab = brush("filbert", 3.6)
local function group(c, col, n, size, under)
  local r = math.max(4, c.r * 1.25)
  -- a group has form: first a dab or two of its own shadow under it, then the lit crescent
  if under then
    if nd % 4 == 0 or shadowdab:fullness() < 0.3 then shadowdab:reload(under, 0.75) end
    for j = 1, (c.r > 6 and 2 or 1) do
      local x, y = c.x + randn(0, r * 0.3) + 0.3 * r, c.y + r * rand(0.35, 0.7)
      local t = randn(0, 0.5)
      shadowdab:stroke({{x - 2.5 * math.cos(t), y - 2.5 * math.sin(t)}, {x + 2.5 * math.cos(t), y + 2.5 * math.sin(t)}},
        {pressure={0.75, 0.6}, ramps={0.3, 0.35}, clip=MASS})
      nd = nd + 1
    end
  end
  local a0 = math.atan(sy, sx) + randn(0, 0.35)
  local b = dab[size]
  b:reload(col, 0.8)
  for j = 1, n do
    -- along a crescent on the sun side of the clump, each dab tilted like a leaf hanging out
    local a = a0 + (j - (n + 1) / 2) * rand(0.35, 0.6)
    local rr = r * rand(0.35, 0.8)
    local x, y = c.x + math.cos(a) * rr, c.y + math.sin(a) * rr * 0.8
    local t = a + math.pi / 2 + randn(0, 0.5)
    local l = rand(1.5, 3.2)
    b:stroke({{x - math.cos(t) * l, y - math.sin(t) * l}, {x + math.cos(t) * l, y + math.sin(t) * l}},
      {pressure={0.8, 0.6}, ramps={0.3, 0.35}, clip=MASS})
    nd = nd + 1
  end
end
-- far to near is back to front already; the groups a step above their mass
for _, g in ipairs(groups) do
  local c, v = g[1], g[2]
  local col = gradient(CROWNRAMP, clamp(v + 0.2 + 0.08 * c.lit, 0, 0.88))
  -- (a dab of the group's own shadow under it, tried: over the tacky mass a dark lies as a gray ghost)
  group(c, col, math.random(3, 6), (c.r > 7 and 3) or (c.r > 5 and 2) or 1, nil)
end
-- the cool groups greener than first tried (#434d3a read as gray patches at 3200)
for _, c in ipairs(cool) do group(c, "#404c34", math.random(2, 4), 2) end
print("dabs", nd)

--@ chunk 5 · clock 2400
-- H chunk 5: once the sky at the crown's edge is DRY (dark leaves over tacky sky lie as gray
-- ghosts). The few PARTICULARS: the sun-side edge broken in spans by leaf groups pushed out past
-- the contour (the shade side and the underside left as found masses), one limb seen in the big
-- gap, the leader running up into the crown's shadow, ~20 top lights and ~15 dark accents; the
-- trunk's lit side and foot; then only a few particulars in the meadow in front.
local t0 = clock()
while drying(300, 250) ~= "dry" and clock() - t0 < 30*24*60 do wait(12*60) end
print("waited for the sky to dry:", (clock() - t0) / 60, "h", drying(640, 300))
-- 1. edges: spans chosen by a slow noise, mostly on the sun side
local MB = MASS:blur(4)
local span = noise{seed=41, period=45}
local pts, tries = {}, 0
while #pts < 1500 and tries < 400000 do
  tries = tries + 1
  local x, y = rand(250, 650), rand(140, 400)
  local m = MASS:at(x, y)
  if m > 0.35 and m < 0.65 and GAPS:grow(3):at(x, y) < 0.1 then pts[#pts + 1] = {x, y} end
end
local dl, dd = brush("filbert", 3.4), brush("filbert", 3.8)
local ne = 0
for i, p in ipairs(pts) do
  local x, y = p[1], p[2]
  local gx, gy = MB:at(x + 2, y) - MB:at(x - 2, y), MB:at(x, y + 2) - MB:at(x, y - 2)
  local g = math.sqrt(gx * gx + gy * gy) + 1e-6
  local ox, oy = -gx / g, -gy / g
  local sunside = smoothstep(-0.1, 0.5, -(ox * 0.7 + oy * 0.7))      -- facing up-left
  local v = VAL(x - ox * 5, y - oy * 5)
  local want = (span(x, y) > 0.05) and (oy < 0.3) and rand(0, 1) < 0.35 + 0.5 * sunside
  if want then
    local a = math.atan(oy, ox) + randn(0, 0.6)
    local out = rand(2, 5)
    -- the dab carries its own mass's color out past the contour, a step up on the sun side
    -- (dark dabs on a lit top read as black knobs at 3200)
    local b, c = dd, CROWNCOL(x - ox * 4, y - oy * 4)
    if sunside > 0.5 and v > 0.4 then b, c = dl, gradient(CROWNRAMP, clamp(v + 0.1, 0, 0.86)) end
    b:reload(c, 0.75)
    b:stroke({{x - math.cos(a) * 2, y - math.sin(a) * 2}, {x + math.cos(a) * out, y + math.sin(a) * out}}, {pressure={0.75, 0.55}, ramps={0.3, 0.35}})
    ne = ne + 1
  end
end
print("edge dabs", ne)
-- 2. the leader into the crown's shadow, lost as it rises; one limb crossing the big gap
local tp = {}
for k, p in ipairs(tree.limbs[1].pts) do if p[2] < 395 and p[2] > 330 then tp[#tp + 1] = {p[1], p[2], tree.limbs[1].w[k]} end end
local function rib(pts, scale)
  local P, Wd = {}, {}
  for i, p in ipairs(pts) do P[i] = {p[1], p[2]} Wd[i] = p[3] * scale end
  return ribbon(P, Wd)
end
if #tp >= 2 then
  local wood = rib(tp, 0.7):roughen(1.5, 8, 3, 1.2) * mask(function(x, y) return smoothstep(352, 388, y) end)
  work(wood, {hand="body", tool="filbert 4", length={5, 12}, coverage=1.6, medium=0.25, load=0.45, hug=false, clip=wood, angle=1.5,
    color="#23231b"})
end
local bs = brush("round", 2.6)
local win = ellipse(373, 313, 16, 11)
local best, bw = nil, 0
for li, l in ipairs(tree.limbs) do
  for k, p in ipairs(l.pts) do if win:at(p[1], p[2]) > 0.3 and (l.w[k] or 0) > bw then best, bw = l, l.w[k] end end
end
if best then
  local seg, wide = {}, win:grow(8)
  for k, p in ipairs(best.pts) do if wide:at(p[1], p[2]) > 0.3 then seg[#seg + 1] = p end end
  if #seg >= 2 then
    bs:reload("#2b2824", 0.85)
    local w0 = math.min(2.4, bw * 0.8 + 0.5)
    bs:stroke(seg, {pressure={bs:pressure_for(w0), bs:pressure_for(w0 * 0.6)}, ramps={0.15, 0.3}, clip=wide})
    print("limb in the gap, width", bw)
  end
end
-- 3. accents: a few top lights on the most lit lobes, a few darkest darks in the recesses
local tops, darks = {}, {}
for i, c in ipairs(tree.clumps) do
  if MASS:at(c.x, c.y) > 0.8 then
    local v = VAL(c.x, c.y)
    if v > 0.72 and c.lit > 0.8 and rand(0, 1) < 0.25 then tops[#tops + 1] = c end
    if CREASE(c.x, c.y) > 0.4 and v < 0.3 and rand(0, 1) < 0.08 then darks[#darks + 1] = c end
  end
end
local function thin(list, n) local o = {} local step = math.max(1, #list / n) local i = 1 while i <= #list and #o < n do o[#o + 1] = list[math.floor(i)] i = i + step end return o end
-- dark accents tried (8-16 notches in the recesses): at this distance they read as black dots
-- against the half-lights, so the recesses carry the darks alone
tops, darks = thin(tops, 14), {}
local tb, db = brush("filbert", 3), brush("filbert", 3)
for i, c in ipairs(tops) do
  if i % 6 == 1 then tb:reload("#8f9656", 0.7) end
  local a = randn(-0.6, 0.5)
  tb:stroke({{c.x - 1.6 * math.cos(a) - 1.5, c.y - 1.6 * math.sin(a) - 1.5}, {c.x + 1.6 * math.cos(a) - 1.5, c.y + 1.6 * math.sin(a) - 1.5}}, {pressure={0.75, 0.55}, ramps={0.3, 0.35}, clip=MASS})
end
for i, c in ipairs(darks) do
  -- a dark accent is a notch a step below its surroundings, not a black dot
  if i % 4 == 1 then db:reload("#151c12", 0.6) end
  for j = 1, 2 do
    local a = randn(0.2, 0.8)
    local x, y = c.x + randn(0, 2), c.y + randn(0, 1.5)
    db:stroke({{x - 2 * math.cos(a), y - 2 * math.sin(a)}, {x + 2 * math.cos(a), y + 2 * math.sin(a)}}, {pressure={0.6, 0.45}, ramps={0.3, 0.35}, clip=MASS})
  end
end
print("top lights", #tops, "dark accents", #darks)
-- 4. the trunk: a lit strip on its left, broken; its foot seated in a dark contact and grass
local lit = TRUNKM * mask(function(x, y) return smoothstep(443, 439, x) * smoothstep(394, 412, y) end)
work(lit, {hand="body", tool="round 2", length={4, 10}, coverage=1.6, medium=0.2, load=0.7, hug=false, clip=TRUNKM, angle=1.55, color="#5f584c"})
local fb = brush("rigger", 0.8)
for i = 1, 45 do
  -- a few blades over the foot, more on the shadow side, not a ring
  local x, y = 452 + randn(4, 11), rand(476, 488)
  if i % 8 == 1 then fb:reload(i % 16 == 1 and "#6f7a3e" or "#4f5b2d", 0.7) end
  local h = rand(4, 10)
  fb:stroke({{x, y}, {x + randn(0, 1), y - h * 0.6}, {x + randn(0, 1.6), y - h}}, {pressure={0.45, 0}, ramps={0.05, 0.75}})
end

--@ chunk 6 · clock 12480
-- H chunk 6: the meadow stays massed (chunk 2). Tried first: two or three bands of taller grass
-- across the field (hedge stripes with specks, then flat bales): the plain massed meadow was better.
-- Only a FEW particulars in front, unequal: three plants of the meadow, each a dark clump of arching
-- leaves, grass blades through it and a few stalks with seed heads; flowers in the biggest.
wait(24*60)
local groups = {{140, 664, 1.5, true}, {846, 658, 1.15, false}, {604, 668, 0.75, false}}
local lb = brush("filbert", 3.4)
local rb = brush("rigger", 1.3)
local hb = brush("round", 2.8)
local nl, nb, nh, nf = 0, 0, 0, 0
for gi, g in ipairs(groups) do
  local gx, gy, sc, flowers = g[1], g[2], g[3], g[4]
  -- 1. the clump of leaves: lanceolate strokes arching up and out from the base, dark to mid
  for i = 1, math.floor(22 * sc) do
    local side = (i % 2 == 0) and 1 or -1
    local a = -math.pi / 2 + side * rand(0.25, 1.2)
    local L = (16 + 18 * sc) * rand(0.5, 1.1)
    local x0, y0 = gx + randn(0, 5 * sc), gy + rand(-2, 3)
    local x1, y1 = x0 + math.cos(a) * L * 0.55, y0 + math.sin(a) * L * 0.55
    local x2, y2 = x0 + math.cos(a + side * 0.5) * L, y0 + math.sin(a + side * 0.5) * L + L * 0.15
    if i % 4 == 1 then lb:reload(({"#27311a", "#34401f", "#46532a", "#5b6a33"})[1 + (i // 4) % 4], 0.8) end
    lb:stroke({{x0, y0}, {x1, y1}, {x2, y2}}, {pressure={0.55, 0.05}, ramps={0.2, 0.6}, swell={0.8, 1.3, 0.6}})
    nl = nl + 1
  end
  -- 2. blades through it, tall and leaning, dark stalks and lit ones
  for i = 1, math.floor(34 * sc) do
    local x, y = gx + randn(0, 12 * sc), gy + rand(-1, 4)
    local h = (26 + 30 * sc) * rand(0.5, 1.3)
    local a = -math.pi / 2 + (x - gx) / (50 * sc) * 0.6 + randn(0, 0.15)
    local c = randn(0, 0.3)
    if i % 5 == 1 then rb:reload(({"#34401f", "#56652f", "#7e8a45", "#a09c5c"})[1 + (i // 5) % 4], 0.8) end
    rb:stroke({{x, y}, {x + math.cos(a) * h * 0.55, y + math.sin(a) * h * 0.55}, {x + math.cos(a + c) * h, y + math.sin(a + c) * h}},
      {pressure={0.8, 0}, ramps={0.05, 0.7}})
    nb = nb + 1
  end
  -- 3. a few stalks with seed heads standing above the clump
  for i = 1, math.floor(4 * sc) do
    local x, y = gx + randn(0, 10 * sc), gy
    local h = (46 + 36 * sc) * rand(0.8, 1.2)
    local a = -math.pi / 2 + randn(0, 0.12)
    local tx, ty = x + math.cos(a) * h, y + math.sin(a) * h
    rb:reload("#6f7440", 0.8)
    rb:stroke({{x, y}, {x + randn(0, 2), y - h * 0.5}, {tx, ty}}, {pressure={0.7, 0.2}, ramps={0.05, 0.4}})
    -- the head: one spindle of pale seed laid down the top of the stalk (dots read as beads)
    hb:reload("#aea46a", 0.8)
    local hx, hy = tx - math.cos(a) * h * 0.2, ty - math.sin(a) * h * 0.2
    hb:stroke({{hx, hy}, {(hx + tx) / 2 + randn(0, 0.5), (hy + ty) / 2}, {tx, ty}}, {pressure={0.5, 0.15}, ramps={0.2, 0.5}, swell={0.8, 1.2, 0.5}})
    nh = nh + 1
  end
  if flowers then
    local fb = brush("round", 3.4)
    for i = 1, 6 do
      local x, y = gx + randn(4, 18), gy - rand(14, 44)
      fb:reload(i % 3 == 0 and "#d6c46a" or "#e4e0d0", 0.8)
      fb:touch(x, y, {pressure=0.8})
      fb:touch(x + 1.4, y + 0.5, {pressure=0.6})
      nf = nf + 1
    end
  end
end
print("leaves", nl, "blades", nb, "seed stalks", nh, "flowers", nf)

--@ chunk 7 · clock 13920
-- the finish, the same for both versions: varnish thinner than round 1 (coats 0.3 pooled into brown
-- worm lines around every dab edge at 3200 on the smooth crown; coats 0.12 does not)
wait(24*60); varnish{color="#e6d3a4", coats=0.12, vary=0.1}; relief()
