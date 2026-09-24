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
