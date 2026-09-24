-- easel session "lab2_lime_H": a painting replayed chunk by chunk.
--   easel run paintings/lua/lab2_lime_H.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
-- Round 6 lab 2, subject "lime": one tall summer lime (linden), a dense oval crown against a
-- pale afternoon sky, the whole tree in the frame with some ground. Shared setup for lime_A
-- and lime_H: canvas, palette, world and sun, sky colors, the crown and trunk drawn, the tree grown.
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=3/4, size=320, seed=71}
HZ = 1150
WORLD = world{horizon=HZ, sun={azimuth=-120, elevation=42}}
SUN = {-0.6, -0.6, 0.45}
-- a pale summer afternoon: a light gray-blue above, paler and warm toward the horizon
sky = function(x, y)
  return gradient({{0, "#8ea1b8"}, {0.4, "#aab7c3"}, {0.8, "#cdd1cb"}, {1, "#dbd7c6"}}, y / HZ)
end
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}
-- the ground: a far strip of land at the horizon, a meadow coming forward to the foot of the canvas
land = below(function(x) return HZ + 3 * math.sin(x / 70) end)
landcol = function(x, y) return mix(mix("#98997f", "#6d7249", smoothstep(HZ, HZ + 60, y)), "#4f5635", smoothstep(HZ + 60, H, y)) end
-- the crown: a tall egg, widest a little below the middle, a blunt dome at the top, lobed,
-- leaning a touch left; the lower edge hangs a little lower on the right
CROWN = {}
local N = 34
for i = 0, N - 1 do
  local t = 2 * math.pi * i / N
  local r = 1 + 0.045 * math.sin(5 * t + 1.3) + 0.03 * math.sin(9 * t + 0.4) + 0.02 * math.sin(13 * t + 2.2)
  local x = 500 - 12 * math.cos(t) + 285 * math.sin(t) * (1 - 0.2 * math.cos(t)) * r
  local y = 548 - 440 * math.cos(t) * r + 22 * math.max(0, math.sin(t)) * math.max(0, -math.cos(t))
  CROWN[#CROWN + 1] = (i % 6 == 3) and {math.floor(x), math.floor(y), "c"} or {math.floor(x), math.floor(y)}
end
TRUNK = {{502, 1215}, {499, 1140}, {495, 1060}, {492, 980}}
crown = outline{pts=CROWN, char="soft", seed=72}
-- denser than the species default (fewer voids, leaves deeper than the shell, larger clumps)
tree = tree_in{crown=crown, trunk=TRUNK, species="lime", season="summer", sun=WORLD, seed=73, girth=0.04, voids=0.06, shell=0.45, clump=0.03, leafiness=1.6}
print(tree)
-- leaf colors, deep shade to top light (a lime's leaf is a little yellower than an oak's)
LEAF = {deep="#1f291b", body="#34422a", sunny="#5f6f3a", shade="#253020", skyshade="#4a5548",
        mid="#48562f", lit="#728245", top="#a0a862"}
WOOD = {dark="#3a332b", light="#7d7566"}

--@ chunk 2 · clock 0
-- H chunk 2 (plan): hierarchical detail. Three scales, each deciding less than the one above.
-- 1. BIG: the silhouette, the crown as one egg lit from the upper left (which side is lit),
--    and ~10 big unequal lobes whose own turn and overhang split the shade half into several
--    separated darker masses with their own values (not one slab).
-- 2. MIDDLE: the tree's clumps gathered into ~70 irregular groups of unequal size (a weighted
--    Voronoi of the clumps, so each group's outline is the union of its clumps, not a dome).
-- 3. PARTICULARS come later, chosen from these fields.
print("bounds", tree.bounds[1], tree.bounds[2], tree.bounds[3], tree.bounds[4])
local lv = tree:leaves()
local edge = lv:blur(2.5):map(function(v) return smoothstep(0.3, 0.55, v) end)
local core = lv:blur(14):map(function(v) return smoothstep(0.22, 0.45, v) end):shrink(12)
-- a few sky holes, unequal, where limb masses part (upper and outer; a dense lime shows few)
HOLEPTS = {{612,352,20,12},{388,520,13,9},{700,610,9,7},{560,742,16,9},{318,300,7,5},{452,905,11,6}}
local hm = nil
for i, p in ipairs(HOLEPTS) do local e = ellipse(p[1], p[2], p[3], p[4]) hm = hm and (hm + e) or e end
HOLES = hm:roughen(5, 9, 5, 1.1)
-- the crown is dense: fill its inside (the tree's own gaps near the leader), keep only drawn holes
MASS = edge + core + tree:crown_mask():shrink(40):soften(6) - HOLES
-- the crown as one egg: which side is lit
local S = {SUN[1], SUN[2], SUN[3]}
local sl = math.sqrt(S[1]^2 + S[2]^2 + S[3]^2)
S = {S[1] / sl, S[2] / sl, S[3] / sl}
SUNV = S
CX, CY, CRX, CRY = 497, 560, 300, 460
function EGG(x, y)
  local dx, dy = (x - CX) / CRX, (y - CY) / CRY
  local d2 = math.min(0.98, dx * dx + dy * dy)
  local nz = math.sqrt(1 - d2)
  return clamp(0.15 + 0.85 * (dx * S[1] + dy * S[2] + nz * S[3]), 0, 1)
end
-- the big lobes, placed by hand: x, y, rx, ry, height (front), their own modeling amount
LOBES = {
  {470,190,170,120,0.0,1.0}, {335,340,120,140,0.1,0.9}, {585,330,170,150,0.05,0.7},
  {305,560,110,170,0.2,1.0}, {500,500,130,120,0.0,0.6}, {690,520,120,190,0.15,0.9},
  {410,760,170,130,0.35,0.8}, {640,790,140,150,0.3,1.0}, {300,860,90,110,0.45,0.7},
  {520,930,150,80,0.5,0.8}, {745,380,70,120,0.1,0.8},
}
-- how much each lobe's top turns to the sky, by hand (not every mass gets the same cap; wide
-- caps side by side in the lower crown read as horizontal bands)
LSK = {0.6, 0.6, 0.55, 0.6, 0.4, 0.95, 0.3, 0.8, 0.55, 0.15, 0.7}
GS = 4
NX, NY = math.ceil(W / GS) + 1, math.ceil(H / GS) + 1
local BL, BC, BI = {}, {}, {}
for j = 0, NY do
  for i = 0, NX do
    local x, y = i * GS, j * GS
    local best, bh, second, nearest, nd = nil, -1e9, -1e9, nil, 1e9
    for k, m in ipairs(LOBES) do
      local dx, dy = (x - m[1]) / m[3], (y - m[2]) / m[4]
      local d2 = dx * dx + dy * dy
      if d2 < nd then nd, nearest = d2, {k, dx, dy} end
      if d2 < 1 then
        local hh = m[5] + math.sqrt(1 - d2)
        if hh > bh then second = bh bh, best = hh, {k, dx, dy, d2} elseif hh > second then second = hh end
      end
    end
    if not best then local r = math.sqrt(nd) best = {nearest[1], nearest[2] / r, nearest[3] / r, 1} end
    local k, dx, dy, d2 = best[1], best[2], best[3], best[4]
    local nz = math.sqrt(math.max(0, 1 - d2))
    local sun = math.max(0, dx * S[1] + dy * S[2] + nz * S[3])
    local skyl = clamp(0.15 - 0.85 * dy, 0, 1) * LSK[k]
    local c = 0
    if second > -1e8 then c = smoothstep(0.3, 0.0, bh - second) * smoothstep(-0.1, 0.7, dy) end
    local id = j * (NX + 1) + i
    BL[id] = 0.5 + (sun - 0.5) * LOBES[k][6]
    BI[id] = skyl
    BC[id] = c
  end
end
function BIL(t, x, y)
  local fx, fy = clamp(x / GS, 0, NX - 1e-3), clamp(y / GS, 0, NY - 1e-3)
  local i, j = math.floor(fx), math.floor(fy)
  local u, w = fx - i, fy - j
  local a, b = t[j * (NX + 1) + i], t[j * (NX + 1) + i + 1]
  local c, d = t[(j + 1) * (NX + 1) + i], t[(j + 1) * (NX + 1) + i + 1]
  return (a * (1 - u) + b * u) * (1 - w) + (c * (1 - u) + d * u) * w
end
LOBE = function(x, y) return BIL(BL, x, y) end     -- the lobe's own sun, 0..1
LSKY = function(x, y) return BIL(BI, x, y) end     -- how much the lobe's top sees the sky
LCR  = function(x, y) return BIL(BC, x, y) end     -- under an overhanging lobe
-- the big value: the egg decides the families, the lobes model inside them
-- two families: the lit side (the egg turned to the sun) and the shade side. In the shade each
-- lobe keeps its own values: its top sees the sky (lighter, cool), its underside and the seam
-- under an overhanging lobe go deepest, so the shade is several masses, never one slab.
-- the terminator is pushed about by the lobes (a lobe turned to the sun catches it across the
-- line; egg light alone drew the terminator as one straight vertical)
FAM = function(x, y) return smoothstep(0.38, 0.56, EGG(x, y) + 0.3 * (LOBE(x, y) - 0.5) - 0.1 * LCR(x, y)) end
LITV = function(x, y) return clamp(0.45 + 0.35 * EGG(x, y) + 0.18 * LOBE(x, y) - 0.25 * LCR(x, y), 0, 1) end
SHV = function(x, y) return clamp(0.07 + 0.34 * LSKY(x, y) * (1 - LCR(x, y)) + 0.08 * LOBE(x, y) - 0.06 * LCR(x, y), 0, 1) end
BIGV = function(x, y) local f = FAM(x, y) return SHV(x, y) * (1 - f) + LITV(x, y) * f end

-- MIDDLE: weighted Voronoi of the clumps around ~70 seeds of unequal weight
local cl = tree.clumps
GROUPS = {}
local NG = 70
for g = 1, NG do
  local c = cl[1 + ((g * 2654435761 + 17) % #cl)]
  GROUPS[g] = {x=c.x, y=c.y, w=rand(0.55, 1.7), n=0, sx=0, sy=0, r2=0}
end
local owner = {}
for it = 1, 6 do
  for g = 1, NG do local G = GROUPS[g] G.n, G.sx, G.sy = 0, 0, 0 end
  for i, c in ipairs(cl) do
    local bg, bd = 1, 1e18
    for g = 1, NG do local G = GROUPS[g] local dx, dy = c.x - G.x, (c.y - G.y) * 1.2 local d = (dx * dx + dy * dy) / (G.w * G.w) if d < bd then bg, bd = g, d end end
    owner[i] = bg
    local G = GROUPS[bg] G.n, G.sx, G.sy = G.n + 1, G.sx + c.x, G.sy + c.y
  end
  for g = 1, NG do local G = GROUPS[g] if G.n > 0 then G.x, G.y = G.sx / G.n, G.sy / G.n end end
end
for i, c in ipairs(cl) do local G = GROUPS[owner[i]] G.r2 = G.r2 + (c.x - G.x)^2 + (c.y - G.y)^2 c.group = owner[i] end
for g = 1, NG do local G = GROUPS[g] G.r = math.sqrt(G.r2 / math.max(1, G.n)) * 1.5 + 12 G.k = rand(0.4, 1) end
-- on the grid: which clump is nearest (a spatial hash), so a group's shape is its clumps'
local CELL = 40
local hash = {}
for i, c in ipairs(cl) do
  local key = math.floor(c.x / CELL) * 1000 + math.floor(c.y / CELL)
  hash[key] = hash[key] or {}
  table.insert(hash[key], i)
end
local GL, GE = {}, {}
for j = 0, NY do
  for i = 0, NX do
    local x, y = i * GS, j * GS
    local bi, bd = nil, 1e18
    local cx, cy = math.floor(x / CELL), math.floor(y / CELL)
    for a = -1, 1 do for b = -1, 1 do
      local l = hash[(cx + a) * 1000 + cy + b]
      if l then for _, ci in ipairs(l) do local c = cl[ci] local d = ((x - c.x)^2 + (y - c.y)^2) / (c.r * c.r) if d < bd then bi, bd = ci, d end end end
    end end
    local v, e = 0.5, 0
    if bi then
      local c = cl[bi]
      local G = GROUPS[c.group]
      local dx, dy = (x - G.x) / G.r, (y - G.y) / G.r
      local d2 = math.min(1, dx * dx + dy * dy)
      local nz = math.sqrt(1 - d2)
      v = 0.5 + (math.max(0, dx * S[1] + dy * S[2] + nz * S[3]) - 0.5) * G.k
      -- the group's own edge: where the nearest clump of another group is close (a seam)
      e = 0
    end
    GL[j * (NX + 1) + i] = v
    GE[j * (NX + 1) + i] = bi and cl[bi].group or 0
  end
end
GROUPL = function(x, y) return BIL(GL, x, y) end
GROUPID = function(x, y)
  local i, j = math.floor(clamp(x / GS + 0.5, 0, NX)), math.floor(clamp(y / GS + 0.5, 0, NY))
  return GE[j * (NX + 1) + i]
end
-- a seam between groups: the group id differs a few units away (dark between groups)
SEAM = function(x, y)
  local g = GROUPID(x, y)
  local s = 0
  for _, o in ipairs({{5, 0}, {-5, 0}, {0, 5}, {0, -5}}) do if GROUPID(x + o[1], y + o[2]) ~= g then s = s + 0.25 end end
  return s
end
skym = above(function(x) return HZ + 6 + 3 * math.sin(x / 70) end)
TRUNKM = tree:wood(9) * mask(function(x, y) return smoothstep(990, 1030, y) end)
print("groups", #GROUPS, "lobes", #LOBES)
-- the sky alla prima AROUND the reserved crown and trunk (dark over wet sky plows into a pale
-- lace; lab round 1), reserved to the silhouette itself so the edges are decided later by hand
local sk = skym - MASS:soften(0.8) - TRUNKM:shrink(1.5)
work(sk, {hand="broad", color=sky, angle=0, coverage=5, medium=0.22, load=1, pal=skypal, clip=sk})
work(sk, {hand="broad", color=sky, angle=0.01, coverage=3.5, medium=0.25, load=0.9, pal=skypal, clip=sk})
blend(sk, {angle=0, coverage=1.5, clip=sk})
-- the ground as tone (as in A), and the tree's shadow on the meadow at its foot
local ld = land - TRUNKM:shrink(1.5)
work(ld, {hand="body", color=landcol, angle=0.02, length={20, 60}, coverage=3.4, medium=0.18, clip=ld})
SHADOW = ellipse(560, 1222, 120, 14):roughen(4, 10, 3, 1)
work(SHADOW, {hand="body", color="#3d4428", angle=0.02, length={10, 30}, coverage=2.6, medium=0.18, clip=SHADOW:grow(2)})

--@ chunk 3 · clock 0
-- H chunk 3: BIG SHAPES. 15 h later (the sky set, so the dark meets it edge to edge without a
-- lace), the whole crown laid in by its value families only: the lit side a body green that
-- warms and lightens where its lobes turn to the sun; the shade side as separate masses, each
-- deepest in the seam under the lobe above it and lifting to a cool gray-green on its top,
-- which sees the sky. Strokes wrap around their own lobe. No leaves yet.
wait(900)
function TOPLOBE(x, y)
  local bk, bh, nk, nd = nil, -1e9, nil, 1e9
  for k, m in ipairs(LOBES) do
    local dx, dy = (x - m[1]) / m[3], (y - m[2]) / m[4]
    local d2 = dx * dx + dy * dy
    if d2 < nd then nd, nk = d2, k end
    if d2 < 1 then local hh = m[5] + math.sqrt(1 - d2) if hh > bh then bh, bk = hh, k end end
  end
  return bk or nk
end
function WRAP(x, y)
  local m = LOBES[TOPLOBE(x, y)]
  return math.atan(y - m[2], x - m[1]) + math.pi / 2
end
SHADEC = function(v) return gradient({{0, "#141c12"}, {0.18, "#1c2618"}, {0.32, "#283323"}, {0.5, "#3a4739"}}, v / 0.5) end
LITC = function(v) return gradient({{0, "#2c3822"}, {0.3, "#3b4a29"}, {0.65, "#526338"}, {1, "#6a7a42"}}, (v - 0.35) / 0.65) end
function BIGCOL(x, y)
  local f = FAM(x, y)
  return mix(SHADEC(SHV(x, y)), LITC(LITV(x, y)), f)
end
work(MASS, {hand="body", tool="filbert 7", length={10, 24}, coverage=2.8, medium=0.3, load=0.6, clip=MASS,
  angle=WRAP, angle_jitter=0.45, curve={0.3, 0.1}, color=BIGCOL})
-- the trunk below the crown, darker toward the leaves
work(TRUNKM, {hand="body", tool="filbert 5", length={10, 30}, coverage=4.2, load=0.8, angle=1.55, angle_jitter=0.15, clip=TRUNKM, medium=0.18,
  color=function(x, y) return mix("#2a2621", "#3d3831", smoothstep(1020, 1180, y)) end})

--@ chunk 4 · clock 900
-- H chunk 4: the MIDDLE SCALE. The lay-in setting. The clump groups (unequal, irregular: the
-- union of their clumps) are what the eye reads as leaf masses inside the big lobes. In the
-- lit family each group gets a thin half-light on its sun side and a dark only a step below
-- the lay-in under it (near-black undersides broke the lit family into camouflage: tried and
-- dropped), and then the tree's own leaf touches, many of them, but only on the groups' lit
-- sides, so the groups' edges are made of leaves, not of a mask. In the shade family only the
-- groups on a lobe's sky-facing top get a cool lift; the rest stays in the lay-in.
-- (at 15 h the lay-in was still open here and the lights sank into it: wait until it sets)
wait(1500)
print("lay-in:", drying(300, 400), drying(700, 600))
local brk = noise{seed=31, period=11}
function GWRAP(x, y)
  local G = GROUPS[GROUPID(x, y)]
  if not G then return WRAP(x, y) end
  return math.atan(y - G.y, x - G.x) + math.pi / 2
end
-- the lit groups, clump by clump: each clump on a group's sun side gets a few short filbert
-- dabs on its own sun side, valued by the group's light and its own, so a group's light has
-- the ragged edge of its clumps and the lay-in between the groups is their shadow. (Dark dabs
-- on the groups' undersides read as polka dots: dropped.) (Mask-driven passes gave hard poster slabs and near-black
-- camouflage mottle in the open lay-in: tried twice and dropped.)
local hb = brush("filbert", 4)
local nh = 0
local wob = noise{seed=33, period=40}
for i, c in ipairs(tree.clumps) do
  if MASS:at(c.x, c.y) > 0.4 then
    local f = FAM(c.x, c.y)
    local g = GROUPL(c.x, c.y)
    local v = f * (0.6 * g + 0.4 * c.lit) * (1 - 0.8 * LCR(c.x, c.y)) + 0.12 * wob(c.x, c.y)
    local r = c.r
    if v > 0.47 then
      -- always a step above the lay-in under it (a fixed color came out darker than the lit lobes)
      local col = mix(BIGCOL(c.x, c.y), "#8d9a55", 0.22 + 0.3 * smoothstep(0.45, 0.8, v))
      local k = math.floor(3 + 9 * smoothstep(0.47, 0.72, v))
      for j = 1, k do
        local ox, oy = randn(-0.2, 0.3) * r, randn(-0.2, 0.3) * r
        local a = math.atan(oy, ox) + math.pi / 2 + randn(0, 0.5)
        local l = rand(3, 7)
        if nh % 5 == 0 or hb:fullness() < 0.3 then hb:reload(col, 0.65) end
        hb:stroke({{c.x + ox, c.y + oy}, {c.x + ox + math.cos(a) * l, c.y + oy + math.sin(a) * l}}, {pressure={0.7, 0.5}, ramps={0.3, 0.4}, clip=MASS})
        nh = nh + 1
      end
    end
  end
end
print("group lights", nh)
SHTOP = MASS:shrink(3) * mask(function(x, y)
  local f = 1 - FAM(x, y)
  return smoothstep(0.45, 0.75, f * smoothstep(0.1, 0.45, LSKY(x, y)) * (1 - LCR(x, y)) * (0.4 + GROUPL(x, y)) + 0.2 * brk(x, y))
end)
work(SHTOP, {hand="body", tool="filbert 4", length={4, 10}, coverage=1.4, medium=0.14, load=0.6, hug=false, clip=MASS,
  angle=GWRAP, angle_jitter=0.8, curve={0.3, 0.1},
  color=function(x, y) return mix("#2f3b30", "#445146", smoothstep(0.3, 0.8, LSKY(x, y) * GROUPL(x, y) * 1.4)) end})
-- the leaf touches: kept with a probability from the plan (family x the group's own light x
-- the touch's own light), graded in three values by the same
local all = tree:touches{}
local l1, l2, l3, cool = {}, {}, {}, {}
for i, t in ipairs(all) do
  local x, y = t.pts[1][1], t.pts[1][2]
  if MASS:at(x, y) > 0.3 then
    local f = FAM(x, y)
    local g = GROUPL(x, y)
    local p = f * smoothstep(0.38, 0.72, g) * (1 - LCR(x, y)) * smoothstep(0.15, 0.55, t.lit)
    if rand(0, 1) < p * 1.2 then
      local v = smoothstep(0.45, 0.8, g) * (0.5 + 0.5 * t.lit) * (0.6 + 0.4 * EGG(x, y))
      if v > 0.5 then l3[#l3 + 1] = t elseif v > 0.28 then l2[#l2 + 1] = t else l1[#l1 + 1] = t end
    elseif (1 - f) * smoothstep(0.3, 0.6, LSKY(x, y)) * smoothstep(0.45, 0.75, g) > rand(0, 1) * 2.5 then
      cool[#cool + 1] = t
    end
  end
end
print("touches: mid", #l1, "lit", #l2, "high", #l3, "cool", #cool)
local function lay(list, b, col, every)
  for i, t in ipairs(list) do
    if i % every == 1 or b:fullness() < 0.25 then b:reload(col, 0.75) end
    b:stroke(t.pts, {pressure={b:pressure_for(t.w), 0.05}, ramps={0.1, 0.7}})
  end
end
lay(cool, brush("round", tree.touch_w), "#46534a", 8)
lay(l1, brush("round", tree.touch_w), "#4f5e33", 8)
lay(l2, brush("round", tree.touch_w * 0.95), "#687842", 8)
lay(l3, brush("round", tree.touch_w * 0.9), "#83914f", 6)

--@ chunk 5 · clock 2400
-- H chunk 5: EDGES, once the sky is dry (dark leaves over tacky sky lie as gray ghosts: lab
-- round 1). The contour and the holes' edges decided in spans by a slow noise:
--  * FOUND AND BROKEN: leaf dabs pushed out past the contour, colored by the family they
--    come from (lit on the sun side, dark in shade), and a few small clusters just outside;
--  * LOST: a thin veil of the sky across the contour, feathered both ways (below);
--  * PLAIN: the underside against the bright horizon stays a clean found edge.
local t0 = clock()
while (drying(120, 500) ~= "dry" or drying(612, 352) ~= "dry") and clock() - t0 < 30*24*60 do wait(12*60) end
print("waited for the sky to dry:", (clock() - t0) / 60, "h")
local MB = MASS:blur(4)
local span = noise{seed=41, period=60}
local pts = {}
local tries = 0
while #pts < 3400 and tries < 900000 do
  tries = tries + 1
  local x, y = rand(120, 880), rand(40, 1030)
  local m = MASS:at(x, y)
  if m > 0.35 and m < 0.65 then pts[#pts + 1] = {x, y} end
end
local leafL, leafM, leafD = brush("filbert", 3.4), brush("filbert", 3.6), brush("filbert", 3.8)
local nf, nc = 0, 0
for i, p in ipairs(pts) do
  local x, y = p[1], p[2]
  local gx = MB:at(x + 2, y) - MB:at(x - 2, y)
  local gy = MB:at(x, y + 2) - MB:at(x, y - 2)
  local g = math.sqrt(gx * gx + gy * gy) + 1e-6
  local ox, oy = -gx / g, -gy / g
  local t = span(x, y)
  local ix, iy = x - ox * 6, y - oy * 6
  local f = FAM(ix, iy)
  local low = y > 880 and oy > 0.45
  local inner = BIGCOL(ix, iy)
  if low then
    -- plain
  elseif t > 0.05 and rand(0, 1) < 0.8 then
    local a = math.atan(oy, ox) + randn(0, 0.6)
    local out, inn = rand(2, 6), rand(1, 3)
    local x0, y0 = x - math.cos(a) * inn, y - math.sin(a) * inn
    local x2, y2 = x + math.cos(a) * out, y + math.sin(a) * out
    local b, c = leafD, mix(inner, "#172015", 0.3)
    if f > 0.6 then b, c = leafL, mix(inner, "#8d9a55", 0.35 * GROUPL(ix, iy))
    elseif f > 0.3 then b, c = leafM, inner end
    if nf % 5 == 0 or b:fullness() < 0.3 then b:reload(c, 0.7) end
    b:stroke({{x0, y0}, {x2, y2}}, {pressure={0.75, 0.55}, ramps={0.3, 0.35}})
    nf = nf + 1
    -- now and then a small cluster of leaves standing free just outside
    if t > 0.35 and rand(0, 1) < 0.06 then
      local d = rand(7, 14)
      local cx, cy = x + ox * d, y + oy * d
      for k = 1, math.random(3, 6) do
        local jx, jy = cx + randn(0, 2.5), cy + randn(0, 2.5)
        local a2 = rand(0, 6.28)
        b:stroke({{jx, jy}, {jx + math.cos(a2) * 3, jy + math.sin(a2) * 3}}, {pressure={0.7, 0.5}, ramps={0.3, 0.35}})
      end
      nc = nc + 1
    end
  end
end
print("found flicks", nf, "free clusters", nc)
-- LOST: where the span says so, leaf and sky interlock at close value: pale leaf dabs (the lit
-- green pulled toward the sky) reach out and dabs of the sky's own color bite in between them.
-- (A halfway tone straddling the contour made a frosty rim, and a thin sky veil laid white
-- frost patches: both tried and dropped.)
local skyb, paleb = brush("filbert", 4), brush("filbert", 3.4)
local nl = 0
for i, p in ipairs(pts) do
  local x, y = p[1], p[2]
  local t = span(x, y)
  if y < 880 and t < -0.28 and rand(0, 1) < 0.7 then
    local gx = MB:at(x + 2, y) - MB:at(x - 2, y)
    local gy = MB:at(x, y + 2) - MB:at(x, y - 2)
    local g = math.sqrt(gx * gx + gy * gy) + 1e-6
    local ox, oy = -gx / g, -gy / g
    local ix, iy = x - ox * 6, y - oy * 6
    local sx, sy = x + ox * 8, y + oy * 8
    local a = math.atan(oy, ox) + randn(0, 0.5)
    if rand(0, 1) < 0.5 then
      -- the sky biting in: from outside to just inside
      skyb:reload(sky(sx, sy), 0.7)
      skyb:stroke({{x + math.cos(a) * 5, y + math.sin(a) * 5}, {x - math.cos(a) * rand(1, 4), y - math.sin(a) * rand(1, 4)}},
        {pressure={0.7, 0.5}, ramps={0.3, 0.35}})
    else
      local c = mix(BIGCOL(ix, iy), sky(sx, sy), 0.25 + 0.2 * FAM(ix, iy))
      if nl % 4 == 0 or paleb:fullness() < 0.3 then paleb:reload(c, 0.65) end
      paleb:stroke({{x - math.cos(a) * 2, y - math.sin(a) * 2}, {x + math.cos(a) * rand(3, 7), y + math.sin(a) * rand(3, 7)}},
        {pressure={0.7, 0.5}, ramps={0.3, 0.35}})
    end
    nl = nl + 1
  end
end
print("lost-span dabs", nl)

--@ chunk 6 · clock 11040
-- H chunk 6: SELECTED PARTICULARS, everything dry. A few things placed where the eye goes:
-- the trunk carried up into the crown's shadow and lost there, with dark leaf clusters hanging
-- in front; one limb glimpsed in each of three holes; and a handful of sharp leaf accents
-- where the light hits hardest. (Tried and dropped: the trunk's top and two scaffold limbs
-- drawn into the crown read as a post with stick arms; three dark pockets with a lit branch
-- in the lit side read as black specks.)
local t1 = clock()
while (drying(490, 960) ~= "dry" or drying(300, 400) ~= "dry") and clock() - t1 < 30*24*60 do wait(12*60) end
print("waited:", (clock() - t1) / 60, "h")
local function rib(l, k0, k1, scale, ylo)
  local P, Wd = {}, {}
  for k = k0, k1 do local p = l.pts[k] if p and (not ylo or p[2] > ylo) then P[#P + 1] = {p[1], p[2]} Wd[#Wd + 1] = l.w[k] * scale end end
  return ribbon(P, Wd)
end
local L1 = tree.limbs[1]
local lo = {}
for k, p in ipairs(L1.pts) do if p[2] > 860 and p[2] < 1060 then lo[#lo + 1] = k end end
local wood = rib(L1, lo[1], lo[#lo], 0.9):roughen(1.5, 8, 3, 0.8)
-- the wood tapers into the crown's shadow and takes the leaves' dark color as it rises
work(wood * mask(function(x, y) return smoothstep(925, 985, y) end), {hand="body", tool="filbert 5", length={8, 20}, coverage=3, medium=0.2,
  load=0.7, hug=false, clip=wood, angle=1.55, angle_jitter=0.25,
  color=function(x, y) return mix("#182015", "#2e2a24", smoothstep(950, 1040, y)) end})
-- the trunk's lit flank (sun upper left) only below the crown's shadow
local fl = TRUNKM * mask(function(x, y) return smoothstep(1030, 1080, y) * (1 - TRUNKM:at(x + 5, y + 2)) end)
work(fl, {hand="body", tool="round 1.6", length={6, 14}, coverage=1.8, medium=0.15, angle=1.55, angle_jitter=0.2, clip=TRUNKM, color="#6a6356"})
-- dark leaf clusters hanging in front of the wood and breaking the underside's line
local db = brush("filbert", 3.8)
local nd = 0
local WB = wood:blur(8)
for i = 1, 1400 do
  local x, y = rand(360, 720), rand(900, 1010)
  local p = math.max(1.4 * WB:at(x, y) * smoothstep(1010, 930, y), 0.25 * MASS:blur(6):at(x, y) * smoothstep(0.2, 0.6, MASS:blur(6):at(x, y)) * (1 - MASS:at(x, y + 12)))
  if rand(0, 1) < p then
    if nd % 6 == 0 then db:reload(rand(0, 1) < 0.7 and "#182015" or "#26301e", 0.75) end
    local a = randn(1.5, 0.8)
    local l = rand(2, 5)
    db:stroke({{x, y}, {x + math.cos(a) * l, y + math.sin(a) * l}}, {pressure={0.75, 0.5}, ramps={0.3, 0.35}})
    nd = nd + 1
  end
end
print("hanging leaves", nd)
-- one limb in each of three holes: the stoutest crossing it, carried on into the leaves
-- either side
local bs = brush("round", 3)
local nlimb = 0
for _, h in ipairs({{612, 352, 26, 17}, {560, 742, 22, 14}, {388, 520, 18, 13}}) do
  local win = ellipse(h[1], h[2], h[3], h[4])
  local best, bw = nil, 0
  for li, l in ipairs(tree.limbs) do
    for k, p in ipairs(l.pts) do if win:at(p[1], p[2]) > 0.3 and (l.w[k] or 0) > bw then best, bw = l, l.w[k] end end
  end
  if best and bw >= 0.8 then
    local seg = {}
    local wide = win:grow(10)
    for k, p in ipairs(best.pts) do if wide:at(p[1], p[2]) > 0.3 then seg[#seg + 1] = p end end
    if #seg >= 2 then
      bs:reload("#2b2824", 0.85)
      local w0 = math.min(2.6, bw * 0.8 + 0.6)
      bs:stroke(seg, {pressure={bs:pressure_for(w0), bs:pressure_for(w0 * 0.6)}, ramps={0.15, 0.3}, clip=wide})
      nlimb = nlimb + 1
    end
  end
end
print("limbs in holes", nlimb)
-- the sharpest lights, a handful, where the sun hits hardest: tops of the most lit groups
local top = {}
for _, t in ipairs(tree:touches{lit={0.8, 1}}) do
  local x, y = t.pts[1][1], t.pts[1][2]
  if MASS:at(x, y) > 0.8 and EGG(x, y) > 0.82 and GROUPL(x, y) > 0.68 and rand(0, 1) < 0.35 then top[#top + 1] = t end
end
local b = brush("round", tree.touch_w * 0.9)
for i, t in ipairs(top) do
  if i % 8 == 1 then b:reload("#b3b673", 0.7) end
  b:stroke(t.pts, {pressure={b:pressure_for(t.w), 0.1}, ramps={0.1, 0.6}})
end
print("top lights", #top)

--@ chunk 7 · clock 18960
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
