-- easel session "lab_foliage_C": a painting replayed chunk by chunk.
--   easel run paintings/lua/lab_foliage_C.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
-- Round 6 lab, subject "foliage": one broadleaf crown in summer leaf against the sky,
-- trunk mostly hidden. Shared setup for foliage_A (old way) and foliage_B (new way):
-- canvas, palette, world and sun, sky colors, the crown and trunk drawn, the tree grown.
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=4/3, size=320, seed=61}
HZ = 688
WORLD = world{horizon=HZ, sun={azimuth=-120, elevation=42}}
SUN = {-0.6, -0.6, 0.45}
-- a summer afternoon sky: clear blue-gray above, pale and warm at the horizon
sky = function(x, y)
  return gradient({{0, "#6d86a8"}, {0.45, "#95a8bc"}, {0.85, "#c6cbc6"}, {1, "#d6d3c3"}}, y / HZ)
end
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}
-- a far strip of land under the tree, hazy
land = below(function(x) return HZ + 3 * math.sin(x / 70) end)
landcol = function(x, y) return mix("#8d9278", "#5e6545", smoothstep(HZ, H, y)) end
-- the crown: broad, lopsided (heavier and lower on the right), lobed, a notch top left
CROWN = {{166,452},{132,392},{146,318},{196,268},{236,236},{246,196},{296,160},{356,150},{400,118},
         {462,96},{516,112},{540,146,"c"},{582,116},{646,106},{714,136},{744,184},{798,210},{846,262},
         {858,326},{830,372,"c"},{870,418},{884,488},{850,540},{790,556},{742,588},{672,590},{620,566,"c"},
         {560,590},{470,600},{420,578},{360,598},{290,590},{240,552},{212,508,"c"}}
TRUNK = {{508,752},{503,700},{497,640},{490,590}}
crown = outline{pts=CROWN, char="soft", seed=62}
tree = tree_in{crown=crown, trunk=TRUNK, species="oak", season="summer", sun=WORLD, seed=63}
print(tree)
-- leaf colors, deep shade to top light
LEAF = {deep="#212a1c", body="#34402a", sunny="#5c6a3a", shade="#263020", skyshade="#4a5446",
        mid="#46522e", lit="#6f7c45", top="#98a060"}
WOOD = {dark="#3b342c", light="#7d7566"}

--@ chunk 2 · clock 0
-- C chunk 2: the plan. The silhouette as in B, but the crown modeled as ~20 overlapping
-- leaf MASSES (the clumps grouped by k-means), each a rounded form lit by the sun and the
-- sky, so the shade half gets internal structure (tops that catch the sky, dark undersides
-- where one mass overhangs the next) instead of one flat dark. The big light families (B's
-- LB) still decide which side of the crown is lit.
local lv = tree:leaves()
local edge = lv:blur(2.5):map(function(v) return smoothstep(0.3, 0.55, v) end)
local core = lv:blur(8):map(function(v) return smoothstep(0.3, 0.55, v) end):shrink(14)
-- sky holes: fewer and unequal, mostly high and on the outer lobes where masses part
HOLEPTS = {{425,318,22,12},{690,293,15,9},{760,445,20,10},{560,393,9,6},{480,150,10,7},{812,505,11,7},{300,470,9,6}}
local h = nil
for i, p in ipairs(HOLEPTS) do local e = ellipse(p[1], p[2], p[3], p[4]) h = h and (h + e) or e end
HOLES = h:roughen(6, 10, 5, 1.0)
-- the crown's underside at the trunk: two small peeks of sky between the hanging masses
PEEKS = (ellipse(452, 588, 16, 8) + ellipse(574, 584, 14, 7)):roughen(4, 8, 7, 0.8)
MASS = edge + core - HOLES - PEEKS
local Lb, Mb = tree:light():blur(10), lv:blur(10)
LB = mask(function(x, y) local m = Mb:at(x, y) if m < 0.05 then return 0 end return clamp(Lb:at(x, y) / m, 0, 1) end)

-- group the clumps into masses (k-means on position, a few seeds spread over the crown)
local cl = tree.clumps
K = 22
local cen = {}
for k = 1, K do local c = cl[1 + ((k * 2654435761) % #cl)] cen[k] = {c.x, c.y} end
local asg = {}
for it = 1, 12 do
  local sx, sy, n = {}, {}, {}
  for k = 1, K do sx[k], sy[k], n[k] = 0, 0, 0 end
  for i, c in ipairs(cl) do
    local bk, bd = 1, 1e18
    for k = 1, K do local dx, dy = c.x - cen[k][1], (c.y - cen[k][2]) * 1.25 local d = dx * dx + dy * dy if d < bd then bk, bd = k, d end end
    asg[i] = bk sx[bk] = sx[bk] + c.x sy[bk] = sy[bk] + c.y n[bk] = n[bk] + 1
  end
  for k = 1, K do if n[k] > 0 then cen[k] = {sx[k] / n[k], sy[k] / n[k]} end end
end
-- each mass: center, radii (from its clumps' spread), depth (front masses stand forward)
MASSES = {}
for k = 1, K do MASSES[k] = {x=cen[k][1], y=cen[k][2], sxx=0, syy=0, z=0, n=0} end
for i, c in ipairs(cl) do local m = MASSES[asg[i]] m.sxx = m.sxx + (c.x - m.x)^2 m.syy = m.syy + (c.y - m.y)^2 m.z = m.z + c.depth m.n = m.n + 1 end
for k, m in ipairs(MASSES) do
  local g = rand(0.8, 1.35)                            -- unequal masses
  m.rx = g * 1.9 * math.sqrt(m.sxx / math.max(1, m.n)) + 10
  m.ry = g * 1.7 * math.sqrt(m.syy / math.max(1, m.n)) + 8
  m.z = m.z / math.max(1, m.n)
  -- the lower masses hang in front of the upper ones (an oak's crown is seen from below)
  m.h = 0.35 * m.z + 0.25 * (m.y - 350) / 250
end
-- the modeled field on a 4-unit grid: which mass is in front, its own light and a crease
-- (dark) where it overhangs the mass behind
local S = {-0.6, -0.6, 0.45}
local sl = math.sqrt(S[1]^2 + S[2]^2 + S[3]^2)
S = {S[1] / sl, S[2] / sl, S[3] / sl}
GS = 4
FL, FC = {}, {}
local nx, ny = math.ceil(W / GS) + 1, math.ceil(H / GS) + 1
for j = 0, ny do
  for i = 0, nx do
    local x, y = i * GS, j * GS
    local best, bh, second = nil, -1e9, -1e9
    local near, nd = nil, 1e9
    for k, m in ipairs(MASSES) do
      local dx, dy = (x - m.x) / m.rx, (y - m.y) / m.ry
      local d2 = dx * dx + dy * dy
      if d2 < nd then nd, near = d2, {k, dx, dy} end
      if d2 < 1 then
        local hh = m.h + math.sqrt(1 - d2)
        if hh > bh then second = bh bh, best = hh, {k, dx, dy, d2} elseif hh > second then second = hh end
      end
    end
    local v, c = 0.35, 0
    -- outside every mass (the lobes of the silhouette): the rim of the nearest one
    if not best and near then local r = math.sqrt(nd) best = {near[1], near[2] / r, near[3] / r, 1} end
    if best then
      local dx, dy, d2 = best[2], best[3], best[4]
      local nz = math.sqrt(math.max(0, 1 - d2))
      local sun = math.max(0, dx * S[1] + dy * S[2] + nz * S[3])
      local skyl = 0.5 + 0.5 * (-dy * 0.8 + nz * 0.2)        -- the tops of every mass see the sky
      v = 0.7 * sun + 0.3 * clamp(skyl, 0, 1)
      if second > -1e8 then c = smoothstep(0.35, 0.0, bh - second) * smoothstep(0.2, 0.8, dy + 0.3) end
    end
    FL[j * (nx + 1) + i] = v
    FC[j * (nx + 1) + i] = c
  end
end
local function bil(t, x, y)
  local fx, fy = clamp(x / GS, 0, nx - 1e-3), clamp(y / GS, 0, ny - 1e-3)
  local i, j = math.floor(fx), math.floor(fy)
  local u, w = fx - i, fy - j
  local a, b = t[j * (nx + 1) + i], t[j * (nx + 1) + i + 1]
  local c, d = t[(j + 1) * (nx + 1) + i], t[(j + 1) * (nx + 1) + i + 1]
  return (a * (1 - u) + b * u) * (1 - w) + (c * (1 - u) + d * u) * w
end
FORM = function(x, y) return bil(FL, x, y) end     -- 0..1: the mass's own light
CREASE = function(x, y) return bil(FC, x, y) end   -- 0..1: under an overhanging mass
-- the value the crown should have: big families (LB) carry the sun side, masses model both halves
VAL = function(x, y) return clamp(0.55 * LB:at(x, y) + 0.45 * FORM(x, y) - 0.25 * CREASE(x, y), 0, 1) end
skym = above(function(x) return HZ + 6 + 3 * math.sin(x / 70) end)
TRUNKM = tree:wood(9) * mask(function(x, y) return smoothstep(540, 580, y) end)
print("masses", #MASSES)
-- the sky alla prima AROUND the reserved crown and trunk, as in B (dark laid over wet sky
-- plows into a pale lace), but reserved to the silhouette itself (no shrink, a crisp edge),
-- so the edge character comes from the painting of the crown, not a halo at every hole
local sk = skym - MASS:soften(0.8) - TRUNKM:shrink(1.5)
work(sk, {hand="broad", color=sky, angle=0, coverage=5, medium=0.22, load=1, pal=skypal, clip=sk})
work(sk, {hand="broad", color=sky, angle=0.01, coverage=3.5, medium=0.25, load=0.9, pal=skypal, clip=sk})
blend(sk, {angle=0, coverage=1.5, clip=sk})
local ld = land - TRUNKM:shrink(1.5)
work(ld, {hand="body", color=landcol, angle=0.02, length={20, 60}, coverage=3.4, medium=0.18, clip=ld})

--@ chunk 3 · clock 0
-- C chunk 3: the crown's dark in one pass, but MODELED: each leaf mass lighter where its top
-- turns to the sky (cool) or the sun (warm), darkest in the crease under an overhanging mass.
-- Strokes wrap around their own mass (tangent to it), turning with its form, short and thin,
-- so there are no grooves running across masses. The sky meanwhile set for 15 h, so the dark
-- meets it edge to edge without plowing a halo into it.
wait(900)
print("sky at the edge:", drying(160, 400), drying(820, 300), drying(425, 318))
-- the crown-scale light, smoother than LB (blur 10 still flickers mass to mass): the sun
-- side is the upper left of the crown, the lower right is in shade
local mb = MASS:blur(28)
local lbb = (LB * MASS):blur(28)
BIG = mask(function(x, y) local m = mb:at(x, y) if m < 0.05 then return 0 end return clamp(lbb:at(x, y) / m, 0, 1) end)
-- each mass models by its own amount (not every mass is a dome with a lit cap)
for k, m in ipairs(MASSES) do m.k = 0.35 + 0.65 * rand(0, 1) end
function TOPM(x, y)
  local bk, bh, nm, nd = nil, -1e9, nil, 1e9
  for k, m in ipairs(MASSES) do
    local dx, dy = (x - m.x) / m.rx, (y - m.y) / m.ry
    local d2 = dx * dx + dy * dy
    if d2 < nd then nd, nm = d2, m end
    if d2 < 1 then local hh = m.h + math.sqrt(1 - d2) if hh > bh then bh, bk = hh, m end end
  end
  return bk or nm
end
function MCOL(x, y)
  local s, f, c = smoothstep(0.22, 0.55, BIG:at(x, y)), FORM(x, y), CREASE(x, y)
  local m = TOPM(x, y)
  local k = m and m.k or 0.5
  f = 0.5 + (f - 0.5) * k
  -- shade half: a deep green, the tops of its masses turned a little cool by the sky
  local shade = mix("#1b2217", "#2f3a2d", smoothstep(0.4, 0.9, f))
  -- sun half: a body green, the masses' sunny sides warmer and lighter
  local lit = mix("#2a3423", "#4f5d35", smoothstep(0.35, 0.95, f))
  local col = mix(shade, lit, s)
  return mix(col, "#141a11", 0.55 * c)
end
function WRAP(x, y)
  local bk = TOPM(x, y)
  if not bk then return 0 end
  return math.atan(y - bk.y, x - bk.x) + math.pi / 2
end
work(MASS, {hand="body", tool="filbert 6", length={8, 18}, coverage=2.8, medium=0.3, load=0.6, clip=MASS,
  angle=WRAP, angle_jitter=0.45, curve={0.3, 0.1}, color=MCOL})
local tr = TRUNKM
work(tr, {hand="body", tool="filbert 5", length={10, 30}, coverage=4.2, load=0.8, angle=1.55, angle_jitter=0.15, clip=tr, medium=0.18,
  color=function(x, y) return mix("#2e2a25", "#3d3831", smoothstep(620, 740, y)) end})

--@ chunk 4 · clock 900
-- C chunk 4: 15 h later, the dark SETTING (B: lights go in soft and leafy then). The lights
-- as the tree's own hooked leaf touches (A's leafiness), but only a chosen few: each touch is
-- kept with a probability from the planned light (the crown-scale sun side times its mass's
-- own turn to the sun), so the lights come in groups that thin out at their edges into the
-- dark, not an even carpet. On the shade half a sparse few cool touches on the mass tops.
wait(900)
print("dark:", drying(300, 300), drying(700, 450), "hole sky:", drying(425, 318))
-- first the half-light of the lit masses, thin, a step above the dark, its edges broken by
-- noise and left to thin out (hug=false), so the touches have a body to sit in
local brk = noise{seed=31, period=14}
HALF = MASS:shrink(2) * mask(function(x, y)
  local s = smoothstep(0.25, 0.6, BIG:at(x, y))
  return smoothstep(0.42, 0.7, s * FORM(x, y) * (1 - CREASE(x, y)) + 0.12 * brk(x, y))
end)
work(HALF, {hand="body", tool="filbert 4", length={5, 12}, coverage=2.2, medium=0.14, load=0.7, hug=false, clip=MASS,
  angle=WRAP, angle_jitter=0.9, curve={0.3, 0.1},
  color=function(x, y) return mix("#3f4b2c", "#5a6839", smoothstep(0.55, 0.9, FORM(x, y))) end})
local all = tree:touches{}
local lit1, lit2, lit3, cool = {}, {}, {}, {}
for i, t in ipairs(all) do
  local x, y = t.pts[1][1], t.pts[1][2]
  if MASS:at(x, y) > 0.2 then
    local s = smoothstep(0.22, 0.6, BIG:at(x, y))
    local m = TOPM(x, y)
    local k = m and m.k or 0.5
    local f = 0.5 + (FORM(x, y) - 0.5) * k
    local c = CREASE(x, y)
    -- where the sun reaches: crown side x mass side, gated by the touch's own light
    local p = s * smoothstep(0.45, 0.85, f) * (1 - c) * smoothstep(0.25, 0.6, t.lit)
    local r = rand(0, 1)
    if r < p * 1.4 then
      local v = p * (0.6 + 0.4 * t.lit)
      if v > 0.62 and rand(0, 1) < 0.35 then lit3[#lit3 + 1] = t
      elseif v > 0.32 then lit2[#lit2 + 1] = t
      else lit1[#lit1 + 1] = t end
    elseif (1 - s) * smoothstep(0.55, 0.9, f) * (1 - c) > rand(0, 1) * 5 then
      cool[#cool + 1] = t
    end
  end
end
print("touches: mid", #lit1, "lit", #lit2, "top", #lit3, "cool", #cool)
local function lay(list, b, col, every)
  for i, t in ipairs(list) do
    if i % every == 1 or b:fullness() < 0.25 then b:reload(col, 0.75) end
    b:stroke(t.pts, {pressure={b:pressure_for(t.w) , 0.05}, ramps={0.1, 0.7}})
  end
end
lay(cool, brush("round", tree.touch_w), "#3a4636", 8)
lay(lit1, brush("round", tree.touch_w), "#46532e", 8)
lay(lit2, brush("round", tree.touch_w * 0.95), "#6d7b43", 8)
lay(lit3, brush("round", tree.touch_w * 0.85), "#9aa35e", 6)

--@ chunk 5 · clock 1800
-- C chunk 5: once the sky is DRY. (Laid the same day, over the 30 h old tacky sky, the dark
-- leaf dabs went on as translucent gray ghosts: wet/C1_edge_leaves_over_tacky_sky.jpg.)
local t0 = clock()
while drying(760, 445) ~= "dry" and clock() - t0 < 30*24*60 do wait(12*60) end
print("waited for the sky to dry:", (clock() - t0) / 60, "h")
-- Edges given a hierarchy instead of one treatment all round.
-- Along the contour (and the holes' edges), a slow noise picks spans:
--  * FOUND AND BROKEN: leaf flicks pulled out past the contour, lit on the sun side, dark
--    on the shade side (A's leafy edge, but only in stretches);
--  * SOFT: on the shade side, a few spans where a gray-green halfway between the dark and
--    the sky there is dragged across the contour with a light hand (leaves thinning to sky);
--  * left alone: the plain found edge of the mass (the lower edge against the bright horizon).
print("dark:", drying(300, 300), drying(700, 450), "sky:", drying(90, 330))
local MB = MASS:blur(4)
local span = noise{seed=41, period=55}
local pts = {}
local tries = 0
while #pts < 3000 and tries < 600000 do
  tries = tries + 1
  local x, y = rand(90, 950), rand(60, 610)
  local m = MASS:at(x, y)
  if m > 0.35 and m < 0.65 then pts[#pts + 1] = {x, y} end
end
-- leaf-sized filbert dabs, not flicks (a flick that ends in a hairline is a whisker at 3200)
local leafL, leafM, leafD = brush("filbert", 3.4), brush("filbert", 3.6), brush("filbert", 3.8)
local soft = brush{kind="round", width=4.5, point=0.4}
local nf, ns = 0, 0
for i, p in ipairs(pts) do
  local x, y = p[1], p[2]
  local gx = MB:at(x + 2, y) - MB:at(x - 2, y)
  local gy = MB:at(x, y + 2) - MB:at(x, y - 2)
  local g = math.sqrt(gx * gx + gy * gy) + 1e-6
  local ox, oy = -gx / g, -gy / g                   -- outward
  local t = span(x, y)
  local s = smoothstep(0.25, 0.6, BIG:at(x - ox * 6, y - oy * 6))
  local low = y > 520 and oy > 0.5                  -- the underside against the bright horizon: keep it found
  if t > 0.08 and not low and rand(0, 1) < 0.8 then
    local a = math.atan(oy, ox) + randn(0, 0.6)
    local out, inn = rand(2, 6), rand(1, 3)
    local x0, y0 = x - math.cos(a) * inn, y - math.sin(a) * inn
    local x2, y2 = x + math.cos(a) * out, y + math.sin(a) * out
    local f = FORM(x - ox * 4, y - oy * 4)
    local b, c = leafD, "#20291b"
    if s > 0.5 and f > 0.6 then b, c = leafL, (f > 0.8 and "#7f8c4c" or "#65733f")
    elseif s > 0.3 then b, c = leafM, "#3b4729" end
    if nf % 5 == 0 or b:fullness() < 0.3 then b:reload(c, 0.7) end
    -- a leaf is a pressed touch dragged a little outward, blunt at both ends
    b:stroke({{x0, y0}, {x2, y2}}, {pressure={0.75, 0.55}, ramps={0.3, 0.35}})
    if rand(0, 1) < 0.5 then
      local tx, ty = -oy, ox
      local d = randn(0, 3)
      local a2 = a + randn(0, 0.5)
      b:stroke({{x0 + tx * d, y0 + ty * d}, {x + tx * d + math.cos(a2) * out * 0.7, y + ty * d + math.sin(a2) * out * 0.7}},
        {pressure={0.7, 0.5}, ramps={0.3, 0.35}})
    end
    nf = nf + 1
  elseif t < -0.3 and s < 0.4 and not low and rand(0, 1) < 0.7 then
    -- soft: a half-tone between the dark and the sky just outside, laid across the contour
    local sx, sy = x + ox * 10, y + oy * 10
    local c = mix(mix("#1f281b", sky(sx, sy), 0.42), "#6e7a78", 0.15)
    local a = math.atan(oy, ox) + randn(0, 0.5)
    local x0, y0 = x - math.cos(a) * 3, y - math.sin(a) * 3
    local x2, y2 = x + math.cos(a) * rand(3, 6), y + math.sin(a) * rand(3, 6)
    if ns % 4 == 0 or soft:fullness() < 0.3 then soft:reload(c, 0.45) end
    soft:stroke({{x0, y0}, {x, y}, {x2, y2}}, {pressure={0.45, 0.05}, ramps={0.2, 0.6}})
    ns = ns + 1
  end
end
print("flicks", nf, "soft", ns)

--@ chunk 6 · clock 11160
-- C chunk 6: the same day, everything dry. The underside of the crown opened a little:
-- the trunk runs on up into the crown's shadow and forks (its leader bending right, the low
-- scaffold limb going left), both darkening and lost into the leaves as they rise, with a few
-- dark leaf clusters hanging in front. Then one limb glimpsed in each of two sky holes, a
-- stretch that enters and leaves the hole.
local tp = {}
for k, p in ipairs(tree.limbs[1].pts) do if p[2] < 610 and p[2] > 430 then tp[#tp + 1] = {p[1], p[2], tree.limbs[1].w[k]} end end
local function rib(pts, scale)
  local P, Wd = {}, {}
  for i, p in ipairs(pts) do P[i] = {p[1], p[2]} Wd[i] = p[3] * scale end
  return ribbon(P, Wd)
end
local lp = {}
local L614 = tree.limbs[614]
for k = 1, 10 do lp[#lp + 1] = {L614.pts[k][1], L614.pts[k][2], L614.w[k]} end
local wood = (rib(tp, 0.9) + rib(lp, 0.8)):roughen(1.5, 8, 3, 0.8)
-- light fades out as the wood rises into the crown (shadowed from above)
local fade = mask(function(x, y) return smoothstep(455, 560, y) end)
WOODIN = wood * fade
work(WOODIN, {hand="body", tool="filbert 5", length={8, 20}, coverage=3, medium=0.2, load=0.7, hug=false, clip=wood,
  angle=function(x, y) return y > 520 and 1.5 or 2.4 end, angle_jitter=0.25,
  color=function(x, y) return mix("#1e1d18", "#34302a", smoothstep(480, 600, y)) end})
-- the trunk just under the crown in its shadow, darker toward the leaves
work(TRUNKM * mask(function(x, y) return smoothstep(640, 590, y) end), {hand="body", tool="filbert 5", length={8, 20}, coverage=2,
  medium=0.2, load=0.5, hug=false, angle=1.55, clip=TRUNKM, color="#2a2621"})
-- dark leaf clusters hanging in front of the wood, thickest where it enters the leaves
local db = brush("filbert", 3.8)
local nd = 0
for i = 1, 600 do
  local x, y = rand(380, 600), rand(430, 575)
  local w = wood:blur(6):at(x, y)
  local p = w * smoothstep(575, 470, y) * 1.2
  if rand(0, 1) < p and MASS:at(x, y) > 0.3 then
    if nd % 6 == 0 then db:reload(rand(0, 1) < 0.7 and "#1c2418" or "#28331f", 0.75) end
    local a = randn(1.5, 0.8)
    local l = rand(2, 5)
    db:stroke({{x, y}, {x + math.cos(a) * l, y + math.sin(a) * l}}, {pressure={0.75, 0.5}, ramps={0.3, 0.35}})
    nd = nd + 1
  end
end
print("hanging leaves", nd)
-- one limb in each of two sky holes: the stoutest that crosses it (two crossing made an X of
-- floating sticks), its stretch carried on into the leaves either side
local bs = brush("round", 3)
local n = 0
for _, h in ipairs({{425, 318, 26, 16}, {760, 445, 24, 14}}) do
  local win = ellipse(h[1], h[2], h[3], h[4])
  local best, bw = nil, 0
  for li, l in ipairs(tree.limbs) do
    for k, p in ipairs(l.pts) do
      if win:at(p[1], p[2]) > 0.3 and (l.w[k] or 0) > bw then best, bw = l, l.w[k] end
    end
  end
  if best and bw >= 1.2 then
    local seg = {}
    local wide = win:grow(12)
    for k, p in ipairs(best.pts) do if wide:at(p[1], p[2]) > 0.3 then seg[#seg + 1] = p end end
    if #seg >= 2 then
      bs:reload("#2b2824", 0.85)
      local w0 = math.min(2.6, bw * 0.8 + 0.6)
      bs:stroke(seg, {pressure={bs:pressure_for(w0), bs:pressure_for(w0 * 0.6)}, ramps={0.15, 0.3}, clip=wide})
      n = n + 1
    end
  end
end
print("limbs in holes", n)
-- the brightest lights, a very few: single touches on the tops of the most lit lobes
local top, k = {}, 0
for _, t in ipairs(tree:touches{lit={0.86, 1}}) do
  k = k + 1
  local x, y = t.pts[1][1], t.pts[1][2]
  if k % 9 == 0 and MASS:at(x, y) > 0.8 and BIG:at(x, y) > 0.5 and FORM(x, y) > 0.7 then top[#top + 1] = t end
end
local b = brush("round", tree.touch_w * 0.9)
for i, t in ipairs(top) do
  if i % 8 == 1 then b:reload("#a9ad6e", 0.7) end
  b:stroke(t.pts, {pressure={b:pressure_for(t.w), 0.1}, ramps={0.1, 0.6}})
end
print("top lights", #top)

--@ chunk 7 · clock 11160
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
