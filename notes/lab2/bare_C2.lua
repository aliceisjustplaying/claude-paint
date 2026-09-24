-- easel session "bare_C2": a painting replayed chunk by chunk.
--   easel run notes/lab2/bare_C2.lua [--width 3200]
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
-- C2-5. C's wood recipe on the new tool. The oak now grows angular and connected, and
-- oak:wood_strokes{} plans every stroke (straight runs subdivided so the brush keeps the elbows, each
-- stroke starting inside the wood it leaves from, a leading twig drawn on in the same stroke). So I stroke
-- the planned strokes by hand, keeping C's recipe: each colored by its depth toward the sky behind it
-- (eased from its parent), far wood 25% thinner, pressed to its width along the way through swell knots,
-- lifting to a point only at a real tip; long ones in pieces of <= 60 units, each on its own load,
-- overlapping 3 units; the round-brush limbs laid twice, the second into the first's wet paint.
-- Selection as C: the fine wood (detail 0.5) thinned again by a low clustered noise, so some sprays
-- crowd and some sky windows open; a dropped limb takes everything beyond it along (nothing floats).
dry()
local L = oak.limbs
local function meanz(l) local s = 0 for _, z in ipairs(l.z) do s = s + z end return s / #l.z end
DARK, WARM = "#302a24", "#3b3129"
REC = function(l) return smoothstep(40, -260, meanz(l)) end
RE = {}
for i, l in ipairs(L) do
  local p = l.parent and RE[l.parent] or 0
  RE[i] = (l.w[1] >= 3.5) and 0 or p + clamp(REC(l) - p, -0.3, 0.3)
end
COLOR = function(i, x, y)
  local l, r = L[i], RE[i]
  local thin = 1 - smoothstep(0.6, 3.0, l.w[1])
  local c = mix(mix(WARM, DARK, 0.5), sky(x, y), 0.06 + 0.5 * r + 0.2 * thin * (0.4 + 0.6 * r))
  if l.twig then c = mix(c, sky(x, y), 0.18) end        -- twigs a little lighter than their limb
  return c
end
DROP = {}
function dropped(i) while i do if DROP[i] then return true end i = L[i].parent end return false end
-- stroke one planned wood stroke: pressure from its width (times kk), in pieces of <= 60 units
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
      local wd = (a[2] + (c[2] - a[2]) * f) * kk
      knots[#knots+1] = clamp(b:pressure_for(math.min(wd, bw)), 0.05, 1)
    end
    if j > 1 then knots[1] = knots[1] * 0.85 end          -- a joining piece: no bead at the join
    local last = (e == #pts) and st.tip
    if last then knots[#knots] = tipp end
    b:reload(col, load or 0.85, {medium=0.3})
    b:stroke(seg, {pressure={1, 1}, swell=knots, ramps={0, last and 0.12 or 0}, shake=0.3})
    n = n + 1
    j = (e < #pts) and math.max(j + 1, e - 6) or e   -- the next piece starts 3 units back: no nick at the join
  end
  return n
end
local function mid(st) local p = st.pts[math.max(1, #st.pts // 2)] return p[1], p[2] end
-- 1. the trunk and stout wood as filled body paint (as C)
local thick = oak:wood(3.5)
work(thick, {hand="body", tool="round 2", length={4, 12}, coverage=3.5, angle=1.5, angle_jitter=0.6, clip=thick, color=WARM, medium=0.15})
-- 2. the planned strokes thick to thin, the fine ones chosen by the clustered noise
local cl = noise{seed=71, octaves=2, period=140}
MAIN = {}
local bands = {{1.2, 3.5, brush("round", 2.4), 0.2}, {0.5, 1.2, brush("rigger", 0.9), 0.06}, {0, 0.5, brush("rigger", 0.55), 0.04}}
local n, kept, all = 0, 0, 0
for _, bd in ipairs(bands) do
  for _, st in ipairs(oak:wood_strokes{min=bd[1], max=bd[2], detail=0.5}) do
    local i = st.limb
    MAIN[i] = true
    if st.fine and not DROP[i] then
      all = all + 1
      local e = st.pts[#st.pts]
      if rand() >= 0.25 + 0.7 * smoothstep(-0.25, 0.35, cl(e[1], e[2])) then DROP[i] = true end
    end
    if not dropped(i) then
      local x, y = mid(st)
      n = n + lay(st, bd[3], COLOR(i, x, y), 0.85 * (1 - 0.25 * RE[i]), bd[4])
      -- the limbs (round 2.4) skipped on the dry sky's tooth: restated at once into their own wet paint
      if bd[1] >= 1.2 then n = n + lay(st, bd[3], COLOR(i, x, y), 0.85 * (1 - 0.25 * RE[i]), bd[4]) end
      if st.fine then kept = kept + 1 end
    end
  end
end
print("wood pieces", n, "fine strokes kept", kept, "of", all)

--@ chunk 7 · clock 31733.5595703125
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

--@ chunk 8 · clock 31733.5595703125
-- C2-7. the crown's mass as C did it: a second tier of real wood, not a tone. The planned fine strokes
-- at detail 0.8 that aren't in the main set, a share weighted to the rim and clustered (C's numbers),
-- drawn at 0.65 of their width and pulled 40% of the way to the sky behind; each springs from drawn
-- wood (a skipped one takes what grows from it along).
local cl = noise{seed=81, octaves=2, period=110}
local cd = oak:crown_mask():distance()
local rg = brush("rigger", 0.55)
local n, m = 0, 0
for _, st in ipairs(oak:wood_strokes{min=0, max=1.2, detail=0.8}) do
  local i = st.limb
  if not MAIN[i] and not dropped(i) then
    local a = st.pts[1]
    local outer = 1 - smoothstep(15, 100, cd:at(a[1], a[2]))
    local share = clamp(0.08 + 0.4 * outer + 0.3 * smoothstep(-0.1, 0.5, cl(a[1], a[2])), 0, 0.65)
    if rand() < share then
      local p = st.pts[math.max(1, #st.pts // 2)]
      n = n + lay(st, rg, mix(COLOR(i, p[1], p[2]), sky(p[1], p[2]), 0.4), 0.65 * (1 - 0.25 * RE[i]), 0.04, 0.5)
      m = m + 1
    else
      DROP[i] = true
    end
  end
end
print("faint strokes", m, "pieces", n)

--@ chunk 9 · clock 31733.5595703125
-- C9. the oak goes into the ground (A buried the round foot in a straw tuft that read as a broom): the ground's
-- color in the shade at the foot, matched to the ground beside it (probed #6c6b58-#74715d; the cast shadow ties it in), dragged
-- across the wet foot in a low rough ellipse, so the trunk's dark is pulled a little into it and its base
-- reads as a line in the grass, not a rounded end
local foot = ellipse(453, 682, 27, 5):roughen(3, 8, 3, 2):soften(2)
work(foot, {hand="body", tool="filbert 4", length={8, 20}, angle=0.02, angle_jitter=0.15, coverage=1.8, medium=0.15, load=0.8, hug=false,
  color=function(x, y) return mix("#6d6b58", "#686754", smoothstep(440, 480, x)) end})

--@ chunk 10 · clock 31733.5595703125
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

--@ chunk 11 · clock 54848.2607421875
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

--@ chunk 12 · clock 54848.2607421875
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
