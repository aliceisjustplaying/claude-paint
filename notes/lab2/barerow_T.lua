-- easel session "barerow_T": a painting replayed chunk by chunk.
--   easel run notes/lab2/barerow_T.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
-- barerow setup (shared by every version): a row of three bare oaks going away from us over a low winter
-- field, near, middle and far, under a pale sky with a low sun from the left. Geometry, color and the
-- stroking helper only, no paint. The question: at a distance, do selected lines still beat a restrained tone?
canvas{style="friedrich", aspect=3/2, size=300, seed=62}
HZ = 452                                   -- the far edge of the field
WORLD = world{horizon=HZ, sun={azimuth=-120, elevation=14}}
sky = function(x, y)
  return gradient({{0,"#9aa6b3"},{0.45,"#bfc3c0"},{0.8,"#d9d3bf"},{1,"#e0d6ba"}}, y/HZ)
end
skypal = pal:only{"lead white","pale smalt","cobalt blue","yellow ochre","raw umber","red earth"}
local gn = noise{seed=4, octaves=4, period=300}
GROUND = function(x) return HZ + 4 * gn(x, 0) + 6 * math.exp(-((x - 180) / 260)^2) end
skym = above(function(x) return GROUND(x) + 8 end)
land = below(GROUND)
ground = function(x, y)
  local t = smoothstep(HZ - 6, H, y)
  return mix(mix("#8f8e7e", "#5f5c48", t), "#4a4636", smoothstep(0.55, 1, t) * 0.6 + 0.1 * gn(x * 4, y * 2))
end
AIR = "#b3b4aa"                           -- the air low over the field, a little darker than the low sky
-- three oaks in a row going away to the right, each drawn lopsided and different; sizes by distance
-- (foot below the horizon: near 170, middle 66, far 24)
NEARCROWN = {{34,440},{44,372},{80,300},{132,236},{190,178},{250,132},{318,104},{384,110},{440,136},{494,176},
             {536,226},{566,290,"c"},{572,352},{548,398},{506,428,"c"},{450,452},{390,466},{332,474},{270,470},{206,462},{150,470,"c"},{90,462}}
NEARTRUNK = {{300,622},{297,570},{292,500}}
MIDCROWN = {{552,446},{556,414},{570,382},{592,352},{618,330},{640,318},{662,320},{690,334},{718,352},{744,376},{764,402,"c"},
            {768,428},{748,446},{716,452,"c"},{680,458},{640,462},{600,460},{572,456}}
MIDTRUNK = {{656,518},{655,494},{652,470}}
FARCROWN = {{828,454},{831,440},{840,426},{853,414},{866,406},{880,408},{892,418},{902,432,"c"},{906,446},{896,456},{876,458},{852,460}}
FARTRUNK = {{866,478},{866,470},{865,462}}
near = tree_in{crown=outline{pts=NEARCROWN, char="soft", seed=5}, trunk=NEARTRUNK, species="oak", season="winter", sun=WORLD, seed=11, girth=0.06}
mid = tree_in{crown=outline{pts=MIDCROWN, char="soft", seed=6}, trunk=MIDTRUNK, species="oak", season="winter", sun=WORLD, seed=12, girth=0.06}
far = tree_in{crown=outline{pts=FARCROWN, char="soft", seed=7}, trunk=FARTRUNK, species="oak", season="winter", sun=WORLD, seed=13, girth=0.06}
print(near); print(mid); print(far)
-- how far each tree stands in the air (0 near .. 1 lost), for its colors
HAZE = {near=0, mid=0.38, far=0.66}
DARK, WARM = "#302a24", "#3b3129"
-- what a distant limb is lost into: the air where it's against the sky, the field (a little darker) below
AIRAT = function(x, y) if y < GROUND(x) - 1 then return AIR end return mix(ground(x, y), DARK, 0.3) end
-- depth in a crown (bare tree C): 0 near .. 1 far, eased from the parent so no limb jumps in value at a fork
function depths(t)
  local L, RE = t.limbs, {}
  for i, l in ipairs(L) do
    local s = 0 for _, z in ipairs(l.z) do s = s + z end
    local r = smoothstep(0.13 * t.height, -0.58 * t.height, s / #l.z)     -- C's 40 .. -260 on a 450-tall crown
    local p = l.parent and RE[l.parent] or 0
    RE[i] = (l.w[1] >= 3.5) and 0 or p + clamp(r - p, -0.3, 0.3)
  end
  return RE
end
-- a limb's color: C's dark pulled toward the sky behind it by its depth, then the whole tree toward the air (AIRAT)
function woodcolor(t, RE, h, i, x, y)
  local l, r = t.limbs[i], RE[i]
  local thin = 1 - smoothstep(0.6, 3.0, l.w[1])
  local c = mix(mix(WARM, DARK, 0.5), sky(x, y), 0.06 + 0.5 * r + 0.2 * thin * (0.4 + 0.6 * r))
  if l.twig then c = mix(c, sky(x, y), 0.18) end
  return mix(c, AIRAT(x, y), h)
end
-- stroke one planned wood stroke (t:wood_strokes{}) by hand: pressed to its width (times kk) through swell
-- knots, lifting to a point (pressure tipp) only at a real tip; in pieces of <= 60 units, each on its own
-- load, the next starting 3 units back (bare_C2)
function lay(st, b, col, kk, tipp, load)
  local pts, w = st.pts, st.w
  local bw = b:mark_width(1)
  local arc = {0}
  for k = 2, #pts do arc[k] = arc[k-1] + math.sqrt((pts[k][1]-pts[k-1][1])^2 + (pts[k][2]-pts[k-1][2])^2) end
  local j, n = 1, 0
  while j < #pts do
    local e = j
    while e < #pts and arc[e] - arc[j] < 60 do e = e + 1 end
    local seg, sw = {}, {}
    for k = j, e do seg[#seg+1] = pts[k]; sw[#sw+1] = {arc[k] - arc[j], w[k]} end
    local total = math.max(1e-3, arc[e] - arc[j])
    local nk = clamp(math.ceil(total / math.max(0.5, 0.75 * bw)) + 1, 2, 24)
    local knots, q = {}, 1
    for m = 0, nk - 1 do
      local d = total * m / (nk - 1)
      while q + 1 < #sw and sw[q+1][1] < d do q = q + 1 end
      local a, c = sw[q], sw[math.min(q + 1, #sw)]
      local f = clamp((d - a[1]) / math.max(1e-6, c[1] - a[1]), 0, 1)
      knots[#knots+1] = clamp(b:pressure_for(math.min((a[2] + (c[2] - a[2]) * f) * kk, bw)), 0.05, 1)
    end
    if j > 1 then knots[1] = knots[1] * 0.85 end
    local last = (e == #pts) and st.tip
    if last then knots[#knots] = tipp end
    b:reload(col, load or 0.85, {medium=0.3})
    b:stroke(seg, {pressure={1, 1}, swell=knots, ramps={0, last and 0.12 or 0}, shake=0.3})
    n = n + 1
    j = (e < #pts) and math.max(j + 1, e - 6) or e
  end
  return n
end

--@ chunk 2 · clock 0

-- C1. (as A) the sky, first thin layer, a shade duller than the target (sketchbook 2)
work(skym, {hand="broad", color=function(x, y) return shift(sky(x, y), -0.02, 0, 0) end, angle=0, coverage=4.2, medium=0.3, pal=skypal})
blend(skym, {angle=0})

--@ chunk 3 · clock 0

-- C2. (as A) a day later: the second sky layer stippled over the set first one (sketchbook 2)
wait(24*60)
print(drying(500, 300))
stipple(skym, {width=2.6, color=sky, coverage=2.6, pressure={0.4, 0.8}, dips={18, 0.35, 0.7}, medium=0.55, pal=skypal})

--@ chunk 4 · clock 1440

-- C3. (as A) dry before the ground goes over the sky edge; the ground as tone (sketchbook 1)
dry()
work(land, {hand="body", color=ground, length={20, 60}, angle=0.02, coverage=3.4, medium=0.18})

--@ chunk 5 · clock 8391.345703125

-- C4. (as A) cut the sky back over the crenellated crest (sketchbook 4), on dry paint
dry()
local cn = noise{seed=21, period=40}
local crest = function(x) return GROUND(x) - 1 + 1.5 * cn(x, 0) end
local cut = mask(function(x, y) return smoothstep(crest(x) - 14, crest(x) - 8, y) * (1 - smoothstep(crest(x) - 0.5, crest(x) + 0.5, y)) end)
work(cut, {hand="detail", tool="round 2", color=function(x, y) return sample(x, crest(x) - 26, 3) end, angle=0, angle_jitter=0.1, length={10, 30}, coverage=3, medium=0.3, clip=cut})
blend(cut, {angle=0, clip=cut})

--@ chunk 6 · clock 19971.880859375
-- the near oak, by lines only (bare C2's recipe): the stout wood as body paint, then the planned strokes
-- thick to thin, each colored by its depth toward the sky behind it, the fine wood (detail 0.5) thinned
-- by a low clustered noise so some sprays crowd and some sky windows open; the limbs laid twice
dry()
NRE = depths(near)
local L = near.limbs
NDROP, NMAIN = {}, {}
function ndropped(i) while i do if NDROP[i] then return true end i = L[i].parent end return false end
local thick = near:wood(3.5)
work(thick, {hand="body", tool="round 2", length={4, 12}, coverage=3.5, angle=1.5, angle_jitter=0.6, clip=thick, color=WARM, medium=0.15})
local cl = noise{seed=71, octaves=2, period=140}
local bands = {{1.2, 3.5, brush("round", 2.4), 0.2}, {0.5, 1.2, brush("rigger", 0.9), 0.06}, {0, 0.5, brush("rigger", 0.55), 0.04}}
local n, kept = 0, 0
for _, bd in ipairs(bands) do
  for _, st in ipairs(near:wood_strokes{min=bd[1], max=bd[2], detail=0.5}) do
    local i = st.limb
    NMAIN[i] = true
    if st.fine and not NDROP[i] then
      local e = st.pts[#st.pts]
      if rand() >= 0.25 + 0.7 * smoothstep(-0.25, 0.35, cl(e[1], e[2])) then NDROP[i] = true end
    end
    if not ndropped(i) then
      local p = st.pts[math.max(1, #st.pts // 2)]
      local col, kk = woodcolor(near, NRE, 0, i, p[1], p[2]), 0.85 * (1 - 0.25 * NRE[i])
      n = n + lay(st, bd[3], col, kk, bd[4])
      if bd[1] >= 1.2 then n = n + lay(st, bd[3], col, kk, bd[4]) end
      if st.fine then kept = kept + 1 end
    end
  end
end
-- the faint second tier toward the rim (C's), 0.65 of its width, 40% toward the sky
local cl2 = noise{seed=81, octaves=2, period=110}
local cd = near:crown_mask():distance()
local rg, m = brush("rigger", 0.55), 0
for _, st in ipairs(near:wood_strokes{min=0, max=1.2, detail=0.8}) do
  local i = st.limb
  if not NMAIN[i] and not ndropped(i) then
    local a = st.pts[1]
    local share = clamp(0.08 + 0.4 * (1 - smoothstep(15, 100, cd:at(a[1], a[2]))) + 0.3 * smoothstep(-0.1, 0.5, cl2(a[1], a[2])), 0, 0.65)
    if rand() < share then
      local p = st.pts[math.max(1, #st.pts // 2)]
      n = n + lay(st, rg, mix(woodcolor(near, NRE, 0, i, p[1], p[2]), sky(p[1], p[2]), 0.4), 0.65 * (1 - 0.25 * NRE[i]), 0.04, 0.5)
      m = m + 1
    else NDROP[i] = true end
  end
end
print("near: pieces", n, "fine kept", kept, "faint", m)

--@ chunk 7 · clock 32216.640625
-- C7. the trunk and stout limbs turn (A's were one flat ink dark; a lit rim laid on the edge read as a
-- halo outline): the fill restated into its own wet dark with its color from where each point sits ACROSS
-- the limb, from the edge toward the low sun (left and a little above) to the other edge: a broad warm light
-- that rolls over into a core shadow, the far side a hair lighter again. No rim, no outline.
local M = near:wood(3.5)
local dx, dy = -0.87, -0.5
ACROSS = function(x, y)
  local a, b = 0, 0
  while a < 30 and M:at(x + dx * (a + 1), y + dy * (a + 1)) > 0.5 do a = a + 1 end
  while b < 30 and M:at(x - dx * (b + 1), y - dy * (b + 1)) > 0.5 do b = b + 1 end
  return a / math.max(1, a + b), a + b     -- 0 at the sun edge, 1 at the far edge; the width across
end
local function turn(x, y)
  local u, w = ACROSS(x, y)
  local lit = 1 - smoothstep(0.08, 0.42, u)
  local refl = smoothstep(0.85, 1, u) * 0.2
  local k = smoothstep(3, 14, w)         -- thin stout limbs turn less
  local c = mix("#29241f", "#857a68", 0.7 * lit * k)
  return mix(c, "#4d4944", refl * k)
end
work(M, {hand="body", tool="round 2", length={6, 16}, coverage=3, angle=1.5, angle_jitter=0.4, clip=M,
  color=turn, medium=0.12, load=0.9})

--@ chunk 8 · clock 32216.640625
-- T: the middle and far crowns' fine twig mass as a restrained, transparent dry-brushed haze. Through the
-- undrawn twigs' density (t:twig_mass(0.1), blurred over a thirtieth of the crown), lean stiff paint
-- (load 0.3, medium 0.05, hiding 0.5) in light filbert strokes that run the way the wood runs there
-- (a direction field splatted from the limbs, bent outward from the fork), each pulling what's under it
-- 50% of the way to a warm gray at the density: they catch the dry sky's tooth and read as a veil of fine
-- streaks, thickest toward the rim. Then (next chunk) a few lines on top.
function grainfield(t, cell)
  local b = t.bounds
  local x0, y0 = b[1] or b.x0, b[2] or b.y0
  local SX, SY = {}, {}
  local function key(i, j) return i * 100000 + j end
  for _, l in ipairs(t.limbs) do
    local p = l.pts
    for k = 2, #p do
      local dx, dy = p[k][1] - p[k-1][1], p[k][2] - p[k-1][2]
      local len = math.sqrt(dx * dx + dy * dy)
      if len > 1e-6 then
        local a = math.atan(dy, dx)
        local kk = key(math.floor(p[k][1] / cell), math.floor(p[k][2] / cell))
        SX[kk] = (SX[kk] or 0) + len * math.cos(2 * a); SY[kk] = (SY[kk] or 0) + len * math.sin(2 * a)
      end
    end
  end
  return function(x, y)
    local ci, cj = math.floor(x / cell), math.floor(y / cell)
    local sx, sy = 0, 0
    for di = -2, 2 do for dj = -2, 2 do
      local kk = key(ci + di, cj + dj)
      if SX[kk] then local wt = 1 / (1 + di * di + dj * dj); sx = sx + SX[kk] * wt; sy = sy + SY[kk] * wt end
    end end
    return 0.5 * math.atan(sy, sx)
  end
end
function haze(t, h, tool, len, seed)
  local tm = t:twig_mass(0.1):blur(t.height / 30)
  local dir = grainfield(t, t.height / 40)
  local FX, FY = t.fork[1], t.fork[2]
  local target = mix("#665d58", AIR, 0.35 + 0.5 * (h - HAZE.mid))
  local m = tm:map(function(v) return smoothstep(0.05, 0.5, v) end)
  work(m, {hand="body", tool=tool, length=len, coverage=2.0, load=0.3, medium=0.05, paint={0.5, 0.95}, seed=seed,
    pressure={0.45, 0.15}, ramps={0.1, 0.6}, hug=false, angle=function(x, y)
      local a = dir(x, y); local o = math.atan(y - FY, x - FX)
      if math.cos(a - o) < 0 then a = a + math.pi end
      return math.atan(math.sin(a) + 1.2 * math.sin(o), math.cos(a) + 1.2 * math.cos(o)) end, angle_jitter=0.15,
    color_over=function(x, y, u) return mix(u, target, 0.5 * m:at(x, y)) end})
end
haze(far, HAZE.far, "filbert 1.4", {4, 10}, 5)
haze(mid, HAZE.mid, "filbert 3", {10, 24}, 6)

--@ chunk 9 · clock 32216.640625
-- a few lines over the haze: the trunk and the limbs that carry the crown (every limb at least wmin wide at
-- its base), and of the thinner wood only a clustered share (never one whose parent isn't drawn)
function fewlines(t, h, wmin, share, seed)
  local RE, L, DROP = depths(t), t.limbs, {}
  local function dropped(i) while i do if DROP[i] then return true end i = L[i].parent end return false end
  local thick = t:wood(3.5)
  work(thick, {hand="body", tool="round 1.4", length={3, 8}, coverage=3.5, angle=1.5, angle_jitter=0.4, clip=thick, color=function(x, y) return mix(WARM, AIRAT(x, y), h) end, medium=0.15})
  local cl = noise{seed=seed, octaves=2, period=0.35 * t.height}
  local bands = {{1.2, 3.5, brush("rigger", 0.9), 0.1}, {0, 1.2, brush("rigger", 0.55), 0.04}}
  local n, kept = 0, 0
  for _, bd in ipairs(bands) do
    for _, st in ipairs(t:wood_strokes{min=bd[1], max=bd[2], detail=0}) do
      local i = st.limb
      if not DROP[i] and L[i].w[1] < wmin then
        local e = st.pts[#st.pts]
        if rand() >= share * (0.3 + 1.4 * smoothstep(-0.25, 0.35, cl(e[1], e[2]))) then DROP[i] = true end
      end
      if not dropped(i) then
        local p = st.pts[math.max(1, #st.pts // 2)]
        n = n + lay(st, bd[3], woodcolor(t, RE, h, i, p[1], p[2]), 1 - 0.25 * RE[i], bd[4])
        kept = kept + 1
      end
    end
  end
  return n, kept
end
print("far lines", fewlines(far, HAZE.far, 0.6, 0.2, 91))
print("middle lines", fewlines(mid, HAZE.mid, 1.4, 0.2, 92))

--@ chunk 10 · clock 32216.640625
-- the three oaks go into the field (bare C's foot patch, cast shadow and grass band, scaled to each trunk:
-- s = trunk width / C's 41.8): the ground's color from beside the foot dragged across the wet rounded foot,
-- then (next chunk) the long shadows to the right under the low sun, fainter with distance, and the band
FEET = {{near, 0.88, HAZE.near}, {mid, 0.34, HAZE.mid}, {far, 0.13, HAZE.far}}
for _, f in ipairs(FEET) do
  local t, s = f[1], f[2]
  local fx, fy = t.foot[1], t.foot[2] + 22 * s
  local cl, cr = sample(fx - 45 * s - 4, fy - 2 * s, 3), sample(fx + 45 * s + 4, fy - 2 * s, 3)
  local e = ellipse(fx + 1, fy, 27 * s + 2, 5 * s + 1):roughen(3 * s + 0.5, 8 * s + 2, 3, 2):soften(2 * s + 0.5)
  work(e, {hand="body", tool=(s > 0.5) and "filbert 4" or ((s > 0.2) and "filbert 2" or "round 1.2"), length={8 * s + 2, 20 * s + 3},
    angle=0.02, angle_jitter=0.15, coverage=1.8, medium=0.15, load=0.8, hug=false,
    color=function(x, y) return mix(cl, cr, smoothstep(fx - 13 * s, fx + 27 * s, x)) end})
end

--@ chunk 11 · clock 32216.640625
-- the long shadows (bare C's glaze, darkest at the contact, widening and fading along the run, blurred and
-- broken by the ground), each fainter with distance; then over each near and middle foot C's band of short
-- upward strokes in the shadow's own color, so the trunk stands in the grass (no blades)
local br = noise{seed=91, octaves=3, period=60}
local cast = mask(function(x, y) return 0 end)
for _, f in ipairs(FEET) do
  local t, s, h = f[1], f[2], f[3]
  local fx, fy = t.foot[1], t.foot[2] + 16 * s
  local run = 420 * s + 20
  local one = mask(function(x, y)
    local u = (x - fx) / run
    if u < -0.1 or u > 1.05 then return 0 end
    local cy = fy + 6 * s + 16 * s * u + 2 * s * br(x, 0)
    local hw = 7 * s + 1 + 14 * s * u
    local d = math.abs(y - cy)
    local body = (1 - smoothstep(hw * 0.35, hw, d)) * smoothstep(-0.1, -0.02, u)
    local fade = 1 - smoothstep(0.0, 1.0, u) ^ 0.8
    return body * fade * clamp(0.8 + 0.35 * br(x, y), 0, 1) * (1 - 0.6 * h)
  end)
  cast = cast + one:blur(6 * s + 1)
end
glaze(cast * land, {color="#403e34", coats=0.34, pigment="transparent"})
local tn = noise{seed=93, octaves=2, period=9}
for k = 1, 2 do
  local t, s = FEET[k][1], FEET[k][2]
  local fx, top0 = t.foot[1], t.foot[2] + 9 * s
  local sh = sample(fx + 8 * s, t.foot[2] + 22 * s, 2)
  local band = mask(function(x, y)
    local top = top0 + 4 * s * tn(x / s, 0) + 3 * s * smoothstep(12 * s, 30 * s, math.abs(x - fx))
    local inx = 1 - smoothstep(12 * s, 26 * s, math.abs(x - fx) + 3 * s * tn(x / s, y / s))
    return smoothstep(top - 1, top + 2, y) * (1 - smoothstep(top0 + 11 * s, top0 + 24 * s, y + 3 * s * tn(x * 2 / s, y / s))) * inx
  end)
  work(band, {hand="body", tool=(s > 0.5) and "round 2" or "round 1", length={3 * s + 1, 7 * s + 1}, angle=-1.57, angle_jitter=0.35, coverage=2.0,
    medium=0.15, load=0.7, hug=false, clip=band:soften(1), color=function(x, y) return mix(shift(sh, -0.01, 0, 0), sh, smoothstep(fx - 8 * s, fx - 28 * s, x)) end})
end

--@ chunk 12 · clock 54546.810546875
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
