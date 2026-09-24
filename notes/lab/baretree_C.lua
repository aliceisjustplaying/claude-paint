-- easel session "lab_baretree_C": a painting replayed chunk by chunk.
--   easel run paintings/lua/lab_baretree_C.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
-- baretree setup (shared by A and B): a small study panel, 4:3, one bare winter oak
-- against a pale sky over a low ground. Everything here is geometry and color, no paint.
canvas{style="friedrich", aspect=4/3, size=300, seed=61}
HZ = 600                                   -- the far edge of the low ground
WORLD = world{horizon=HZ, sun={azimuth=-120, elevation=14}}   -- a low winter sun, from the left
SUN = {-0.8, -0.35, 0.45}
-- the sky: a pale, slightly warm winter sky, cool gray-blue overhead
sky = function(x, y)
  return gradient({{0,"#9aa6b3"},{0.45,"#bfc3c0"},{0.8,"#d9d3bf"},{1,"#e0d6ba"}}, y/HZ)
end
skypal = pal:only{"lead white","pale smalt","cobalt blue","yellow ochre","raw umber","red earth"}
-- the ground: a low, gently uneven line; darker and warmer toward us
local gn = noise{seed=4, octaves=4, period=300}
GROUND = function(x) return HZ + 6 * gn(x, 0) - 10 * math.exp(-((x - 470) / 160)^2) end
skym = above(function(x) return GROUND(x) + 8 end)
land = below(GROUND)
ground = function(x, y)
  local t = smoothstep(HZ - 10, H, y)
  return mix(mix("#8b8a78", "#5f5c48", t), "#4a4636", smoothstep(0.6, 1, t) * 0.6 + 0.1 * gn(x * 4, y * 2))
end
-- the oak: a broad, lopsided winter crown with a long low limb to the right; a trunk leaning a little
OAKCROWN = {{120,420},{128,350},{165,285},{210,230},{250,168},{318,118},{372,108},{420,70},{488,58},{548,84},
            {610,78},{690,112},{760,150},{830,210},{880,262},{912,330,"c"},{880,372},{842,392},{812,430,"c"},
            {740,452},{660,468},{580,474},{500,488},{420,480},{330,486},{250,478},{190,470,"c"},{140,452}}
OAKTRUNK = {{452,660},{448,590},{440,500}}
oak = tree_in{crown=outline{pts=OAKCROWN, char="soft", seed=5}, trunk=OAKTRUNK, species="oak",
              season="winter", sun=WORLD, seed=11, girth=0.06}
print(oak)

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

--@ chunk 5 · clock 9126.423828125

-- C4. (as A) cut the sky back over the crenellated crest (sketchbook 4), on dry paint
dry()
local cn = noise{seed=21, period=40}
local crest = function(x) return GROUND(x) - 1 + 1.5 * cn(x, 0) end
local cut = mask(function(x, y) return smoothstep(crest(x) - 14, crest(x) - 8, y) * (1 - smoothstep(crest(x) - 0.5, crest(x) + 0.5, y)) end)
work(cut, {hand="detail", tool="round 2", color=function(x, y) return sample(x, crest(x) - 26, 3) end, angle=0, angle_jitter=0.1, length={10, 30}, coverage=3, medium=0.3, clip=cut})
blend(cut, {angle=0, clip=cut})

--@ chunk 6 · clock 21666.8955078125
-- C5. the oak's wood, dry sky under it (A's connected network), but chosen and weighted by hand:
-- each limb stroked on its own, its color by its depth (far limbs pulled toward the sky behind them,
-- lighter and cooler; near limbs dark and warm), its weight by depth too, the thin wood smoothed a
-- little (the scaffold's zigzag), and every limb that ends tapered to nothing.
dry()
local L = oak.limbs
KIDS = {}
for i, l in ipairs(L) do if l.parent then KIDS[l.parent] = KIDS[l.parent] or {}; table.insert(KIDS[l.parent], i) end end
local function meanz(l) local s = 0 for _, z in ipairs(l.z) do s = s + z end return s / #l.z end
DARK, WARM = "#302a24", "#3b3129"
-- recession: 0 near, 1 far (z runs about -300 at the back to +300 in front)
REC = function(l) return smoothstep(40, -260, meanz(l)) end
-- which limbs: everything non-twig down to 0.7; below that a clustered choice (a low noise opens
-- windows of sky and lets some sprays crowd), and never a limb whose parent isn't painted
local cl = noise{seed=71, octaves=2, period=140}
KEEP = {}
for i, l in ipairs(L) do
  local p = l.parent
  local ok = (not p) or KEEP[p]
  if ok and not l.twig then
    if l.w[1] < 0.7 then
      local e = l.pts[#l.pts]
      ok = rand() < 0.25 + 0.7 * smoothstep(-0.25, 0.35, cl(e[1], e[2]))
    end
    if ok and l.w[1] < 2.2 and #l.pts >= 3 then   -- a painter drops the curliest thin and middling limbs
      local len = 0
      for k = 2, #l.pts do len = len + math.sqrt((l.pts[k][1]-l.pts[k-1][1])^2 + (l.pts[k][2]-l.pts[k-1][2])^2) end
      local a, e = l.pts[1], l.pts[#l.pts]
      local chord = math.sqrt((e[1]-a[1])^2 + (e[2]-a[2])^2)
      ok = len < (l.w[1] < 1.2 and 1.12 or 1.2) * chord + 2
    end
    KEEP[i] = ok or nil
  end
end
-- recession per limb, eased from its parent's so no limb jumps in value where it forks
RE = {}
for i, l in ipairs(L) do
  local p = l.parent and RE[l.parent] or 0
  RE[i] = (l.w[1] >= 3.5) and 0 or p + clamp(REC(l) - p, -0.3, 0.3)
end
local function smooth1(pts)
  if #pts < 3 then return pts end
  local o = {pts[1]}
  for k = 2, #pts - 1 do
    local a, b, c = pts[k-1], pts[k], pts[k+1]
    o[k] = {(a[1] + 2*b[1] + c[1]) / 4, (a[2] + 2*b[2] + c[2]) / 4}
  end
  o[#pts] = pts[#pts]
  return o
end
local function smooth(pts) return smooth1(smooth1(pts)) end
local function haskept(i) for _, j in ipairs(KIDS[i] or {}) do if KEEP[j] then return true end end return false end
COLOR = function(l)
  local e = l.pts[math.max(1, #l.pts // 2)]
  local r = RE[l.i]
  local thin = 1 - smoothstep(0.6, 3.0, l.w[1])
  return mix(mix(WARM, DARK, 0.5), sky(e[1], e[2]), 0.06 + 0.5 * r + 0.2 * thin * (0.4 + 0.6 * r))
end
-- 1. the trunk and stout wood as filled body paint
local thick = oak:wood(3.5)
work(thick, {hand="body", tool="round 2", length={4, 12}, coverage=3.5, angle=1.5, angle_jitter=0.6, clip=thick, color=WARM, medium=0.15})
-- 2. each kept limb under 3.5 wide, and the thin outer part of the stout limbs and the leader, which the
-- fill stops short of (A's blunt stumps): pressed from its width to its tip; limbs that end taper to nothing
local b24, rg9, rg5 = brush("round", 2.4), brush("rigger", 0.9), brush("rigger", 0.5)
local n = 0
for i, l in ipairs(L) do
  if KEEP[i] and #l.pts >= 2 then
    local pts, ws = l.pts, l.w
    if ws[1] >= 3.5 then       -- the stout limb's tail, from a little inside the fill
      local k = 1
      while k < #ws and ws[k] >= 4.0 do k = k + 1 end
      k = math.max(1, k - 1)
      pts, ws = {}, {}
      for j = k, #l.pts do pts[#pts+1] = l.pts[j]; ws[#ws+1] = l.w[j] end
    end
    if #pts >= 2 then
      local kk = 0.85 * (1 - 0.25 * RE[i])
      if ws[1] < 1.2 then pts = smooth(pts) end
      local col, ends = COLOR(l), not haskept(i)
      -- long limbs in pieces of about 60 units (one load each, overlapping a point), each with the
      -- brush for its own width: a long stroke ran dry and a lightly pressed big brush skipped
      local j = 1
      while j < #pts do
        local seg, len, e = {pts[j]}, 0, j
        while e < #pts and len < 60 do
          e = e + 1; seg[#seg+1] = pts[e]
          len = len + math.sqrt((pts[e][1]-pts[e-1][1])^2 + (pts[e][2]-pts[e-1][2])^2)
        end
        local w0, w1 = math.min(ws[j], 4.2) * kk, ws[e] * kk
        local p0 = (j > 1) and 0.85 or 1   -- a joining piece starts a touch lighter: no bead
        local b = w0 >= 1.2 and b24 or (w0 >= 0.6 and rg9 or rg5)
        b:reload(col, 0.6, {medium=0.3})
        if ends and e == #pts then
          b:stroke(seg, {pressure={p0 * b:pressure_for(w0), 0}, ramps={0.08, 0.55}})
        else
          b:stroke(seg, {pressure={p0 * b:pressure_for(w0), b:pressure_for(math.max(0.25, w1))}, ramps={0.08, 0.3}})
        end
        n = n + 1
        j = e
      end
    end
  end
end
print("limb strokes", n)

--@ chunk 7 · clock 31733.5595703125
-- C6. a chosen few of the fine twigs (A traced all 4,474 as fishbone sprays that read as stars). Only
-- twigs whose limb is painted; a clustered share (more where the crown crowds toward its rim, few in
-- the open windows); spaced along each limb; at most one near a limb's tip, so no starbursts; each a
-- little lighter than its limb, tapered to nothing.
local L = oak.limbs
local cl = noise{seed=71, octaves=2, period=140}
local fine = noise{seed=72, octaves=2, period=45}
local cd = oak:crown_mask():distance()
local rg = brush("rigger", 0.5)
local kept, byparent = 0, {}
TWIGS = {}
for i, l in ipairs(L) do
  local p = l.parent
  if l.twig and p and KEEP[p] and #l.pts >= 2 then
    local a = l.pts[1]
    local pl = L[p].pts; local tip = pl[#pl]
    local neartip = math.abs(a[1] - tip[1]) + math.abs(a[2] - tip[2]) < 10
    local outer = 1 - smoothstep(20, 120, cd:at(a[1], a[2]))
    local share = clamp(0.06 + 0.22 * smoothstep(-0.2, 0.4, cl(a[1], a[2])) + 0.18 * outer + 0.12 * fine(a[1], a[2]), 0, 0.5)
    if rand() < share then
      local bp = byparent[p] or {}
      local ok = true
      for _, q in ipairs(bp) do
        if math.abs(q[1] - a[1]) + math.abs(q[2] - a[2]) < 14 or (neartip and q.tip) then ok = false end
      end
      if ok then
        bp[#bp+1] = {a[1], a[2], tip=neartip}; byparent[p] = bp
        TWIGS[#TWIGS+1] = i
      end
    end
  end
end
for _, i in ipairs(TWIGS) do
  local l = L[i]
  local col = mix(COLOR(L[l.parent]), sky(l.pts[1][1], l.pts[1][2]), 0.18)
  -- some twigs a little shorter than grown (a painter's flick stops where it likes)
  local pts = l.pts
  if #pts >= 4 and rand() < 0.4 then pts = {table.unpack(pts, 1, #pts - 1)} end
  rg:reload(col, 0.6, {medium=0.3})
  rg:stroke(pts, {pressure={rg:pressure_for(0.8 * l.w[1] * (1 - 0.2 * RE[l.parent])), 0}, ramps={0.05, 0.5}})
  kept = kept + 1
end
print("twigs", kept)

--@ chunk 8 · clock 31733.5595703125
-- C7. the trunk and stout limbs turn (A's were one flat ink dark; a lit rim laid on the edge read as a
-- halo outline): the fill restated into its own wet dark with its color from where each point sits ACROSS
-- the limb, from the edge toward the low sun (left and a little above) to the other edge: a broad warm light
-- that rolls over into a core shadow, the far side a hair lighter again. No rim, no outline.
local M = oak:wood(3.5)
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

--@ chunk 9 · clock 31733.5595703125
-- C8. the crown's mass: not a scumbled tone (B's fur) but a second tier of real twigs, far fewer than grown,
-- drawn very thin and pulled most of the way to the sky behind them, so at arm's length they read as a
-- restrained transparent tone thickest toward the rim, and close up as twigs that each spring from painted wood
local L = oak.limbs
local cl = noise{seed=81, octaves=2, period=110}
local cd = oak:crown_mask():distance()
local chosen = {}
for _, i in ipairs(TWIGS) do chosen[i] = true end
local rg = brush("rigger", 0.5)
local byparent, n = {}, 0
for i, l in ipairs(L) do
  local p = l.parent
  if l.twig and not chosen[i] and p and KEEP[p] and #l.pts >= 2 then
    local a = l.pts[1]
    local outer = 1 - smoothstep(15, 100, cd:at(a[1], a[2]))
    local share = clamp(0.08 + 0.4 * outer + 0.3 * smoothstep(-0.1, 0.5, cl(a[1], a[2])), 0, 0.65)
    if rand() < share then
      local bp = byparent[p] or {}
      local ok = true
      for _, q in ipairs(bp) do if math.abs(q[1] - a[1]) + math.abs(q[2] - a[2]) < 7 then ok = false end end
      if ok then
        bp[#bp+1] = {a[1], a[2]}; byparent[p] = bp
        local col = mix(COLOR(L[p]), sky(a[1], a[2]), 0.4)
        rg:reload(col, 0.5, {medium=0.35})
        rg:stroke(l.pts, {pressure={rg:pressure_for(0.65 * l.w[1]), 0}, ramps={0.05, 0.45}})
        n = n + 1
      end
    end
  end
end
print("faint twigs", n)

--@ chunk 10 · clock 31733.5595703125
-- C9. the oak goes into the ground (A buried the round foot in a straw tuft that read as a broom): the ground's
-- color in the shade at the foot, matched to the ground beside it (probed #6c6b58-#74715d; the cast shadow ties it in), dragged
-- across the wet foot in a low rough ellipse, so the trunk's dark is pulled a little into it and its base
-- reads as a line in the grass, not a rounded end
local foot = ellipse(453, 682, 27, 5):roughen(3, 8, 3, 2):soften(2)
work(foot, {hand="body", tool="filbert 4", length={8, 20}, angle=0.02, angle_jitter=0.15, coverage=1.8, medium=0.15, load=0.8, hug=false,
  color=function(x, y) return mix("#6d6b58", "#686754", smoothstep(440, 480, x)) end})

--@ chunk 11 · clock 31733.5595703125
-- C10. the long shadow under the low sun (A's was one flat, hard-edged band): a glaze that starts inside the
-- foot so trunk, contact and shadow read as one dark shape, darkest at the contact, fading and widening as
-- it goes, its edges well blurred and broken a little by the ground's unevenness
local fx, fy = 452, 676
local br = noise{seed=91, octaves=3, period=60}
local cast = mask(function(x, y)
  local t = (x - fx) / 420
  if t < -0.1 or t > 1.05 then return 0 end
  local cy = fy + 6 + 16 * t + 2 * br(x, 0)
  local hw = 7 + 14 * t
  local d = math.abs(y - cy)
  local body = (1 - smoothstep(hw * 0.35, hw, d)) * smoothstep(-0.1, -0.02, t)
  local fade = 1 - smoothstep(0.0, 1.0, t) ^ 0.8
  return body * fade * clamp(0.8 + 0.35 * br(x, y), 0, 1)
end):blur(6)
glaze(cast * land, {color="#403e34", coats=0.34, pigment="transparent"})

--@ chunk 12 · clock 54728.8837890625
-- C11. the foot, finished as a mass, not blades: the shadow's color at the foot flicked up in short
-- round-brush strokes over the trunk's rounded base, to a rough, low top edge, so the trunk stands IN the grass
local tn = noise{seed=93, octaves=2, period=9}
local band = mask(function(x, y)
  local top = 669 + 4 * tn(x, 0) + 3 * smoothstep(12, 30, math.abs(x - 453))
  local inx = 1 - smoothstep(12, 26, math.abs(x - 453) + 3 * tn(x, y))
  return smoothstep(top - 1, top + 2, y) * (1 - smoothstep(680, 693, y + 3 * tn(x * 2, y))) * inx
end)
work(band, {hand="body", tool="round 2", length={3, 7}, angle=-1.57, angle_jitter=0.35, coverage=2.0, medium=0.15, load=0.7, hug=false,
  clip=band:soften(1),
  color=function(x, y) return mix("#4a473a", "#58564a", smoothstep(445, 425, x)) end})   -- the shadow at the foot, probed #4b4b3b

--@ chunk 13 · clock 54728.8837890625
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
