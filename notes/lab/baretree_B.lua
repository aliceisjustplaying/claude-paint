-- easel session "lab3_bt_B": a painting replayed chunk by chunk.
--   easel run paintings/lua/lab3_bt_B.lua [--width 3200]
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

-- B1. the sky in one thin, quiet layer, blended while open; a second soft pass laid into it wet
work(skym, {hand="broad", color=sky, angle=0, coverage=4.6, medium=0.3, pal=skypal})
blend(skym, {angle=0})
work(skym, {hand="broad", color=sky, angle=0, coverage=2.4, medium=0.35, load=0.6, pal=skypal})
blend(skym, {angle=0, length={80, 200}})
print(drying(500, 300))

--@ chunk 3 · clock 0

-- B2. the ground into the open sky, no dry(): the shade under the oak and its long cast shadow
-- are painted in the same pass as the ground (one dark family), not glazed on later
local fx, fy = 452, 660
SHADE = mask(function(x, y)
  local t = (x - fx) / 430
  if t < -0.08 or t > 1 then return 0 end
  local cy = fy + 6 + 18 * t
  local hw = 7 + 18 * t
  local d = math.abs(y - cy)
  return (1 - smoothstep(hw * 0.5, hw, d)) * (1 - smoothstep(0.1, 1, t)) * smoothstep(-0.08, 0.02, t)
end):blur(4)
local groundB = function(x, y)
  local c = ground(x, y)
  return mix(c, shift(c, -0.1, 0, -0.012), 0.85 * SHADE:at(x, y))
end
work(land, {hand="body", color=groundB, length={20, 60}, angle=0.02, coverage=3.4, medium=0.22, clip=land})
-- the crest: a clean blender along it while sky and ground are both open, a soft far edge
local crest = mask(function(x, y) local g = GROUND(x); return smoothstep(g - 7, g - 2, y) * (1 - smoothstep(g + 2, g + 7, y)) end)
blend(crest, {angle=0, length={40, 120}, clip=crest})
print(drying(500, 598), drying(500, 610))

--@ chunk 4 · clock 0

-- B2b. restate the band just under the crest, where the ground strokes plowed up the open sky
local band = mask(function(x, y) local g = GROUND(x); return smoothstep(g - 1, g + 2, y) * (1 - smoothstep(g + 16, g + 26, y)) end)
work(band, {hand="body", color=function(x, y) return ground(x, y) end, length={30, 80}, angle=0.01, coverage=2.6, load=0.9, medium=0.2, clip=land, hug=false, pressure={0.8, 0.6}})

--@ chunk 5 · clock 0
-- B3. the twig mass as one tone, dry-brushed straight into the open sky (no wait): where the fine twigs
-- are (their blurred density) short, lean, radial strokes pull the sky toward a warm brown-gray; they
-- taper out, so the crown's edge is a fringe, not a line. The 4,474 twigs themselves are not traced.
-- (Rejected on the way, crops in notes/lab/baretree.md: a filbert veil + blender and glaze-hand strokes
-- lifted the thin open sky to the red ground; a glaze() through the density read as a smoke cloud.)
print(drying(600, 200))
local fine = oak:wood(0, 1.2)
DEN = fine:blur(20):map(function(v) return smoothstep(0.03, 0.22, v) end)
FX, FY = 455, 380
RADIAL = function(x, y) return math.atan(y - FY, x - FX) end
work(DEN, {hand="body", tool="round 2", length={8, 20}, coverage=1.6, load=0.35, medium=0.25, hug=false,
  angle=RADIAL, angle_jitter=0.3, pressure={0.45, 0.05}, ramps={0.1, 0.8},
  color_over=function(x, y, under) return mix(under, "#5a5048", 0.4 * DEN:at(x, y)) end})

--@ chunk 6 · clock 0
-- B4. the next day (wood laid into the OPEN sky, or after 8 h, churned the pale sky into it: a mottled gray trunk,
-- crop in notes/lab/baretree.md), the sky tacky: the wood as one connected dark: trunk and stout limbs as body paint, then every limb down to
-- 0.7 wide pressed to its own width (a child is never wider than its parent, so nothing floats).
-- The 4,474 twigs are NOT traced: the tone says them.
wait(24*60)
print(drying(445, 440), drying(600, 200), drying(700, 250))
local dark = "#3a332c"
local thick = oak:wood(3.5)
work(thick, {hand="body", tool="round 2", length={4, 12}, coverage=3, angle=1.5, angle_jitter=0.6, clip=thick, color=dark, medium=0.15})
oak:paint_wood(brush("round", 2.4), {color=dark, min=1.2, max=3.5})
oak:paint_wood(brush("rigger", 0.9), {color="#453d35", min=0.7, max=1.2})
-- the light, laid straight into the wet dark so it melts at its inner edge: the low sun from the
-- left catches the trunk's left flank and the tops of the stout limbs
local stout = oak:wood(6)
local flank = stout * mask(function(x, y) return 1 - stout:at(x - 3.5, y + 1.5) end)
print(drying(440, 560), drying(600, 200), clock())

--@ chunk 7 · clock 1440

-- B5. restate the trunk, stout limbs and pressed limbs into their own wet dark: the first pass skipped on the tacky
-- sky and left pale flecks; wet into wet, the second closes them
local thick = oak:wood(3.5)
work(thick, {hand="body", tool="round 2", length={6, 16}, coverage=2.5, angle=1.5, angle_jitter=0.4, clip=thick, color="#3a332c", medium=0.12, load=0.9})
-- and the pressed limbs a second time: on the tacky sky their first strokes skipped into pale dashes
oak:paint_wood(brush("round", 2.4), {color="#3a332c", min=1.2, max=3.5})
oak:paint_wood(brush("rigger", 0.9), {color="#453d35", min=0.7, max=1.2})

--@ chunk 8 · clock 1440
-- B6. at once, into the wet wood: (1) the low sun from the left on the trunk's flank and the stout limbs' tops,
-- laid into the open dark so it melts at its inner edge
local thick = oak:wood(3.5)
local stout = oak:wood(6)
local flank = stout * mask(function(x, y) return 1 - stout:at(x - 4.5, y + 1.8) end)
work(flank, {hand="body", tool="round 1.6", length={4, 10}, coverage=3, angle=1.5, clip=flank:grow(1):soften(1), color="#aa9e8b", medium=0.1, load=0.85})
-- (2) a few twigs at the crown's edge, each painted as a whole chain down to the painted wood
local cd = oak:crown_mask():distance()
local painted = function(l) return l.w[1] >= 0.7 end
local chains, marks = 0, 0
local b = brush("rigger", 0.6)
b:load("#4a4139", 0.8)
local seen = {}
for i, l in ipairs(oak.limbs) do
  local e = l.pts[#l.pts]
  if not painted(l) and l.w[1] >= 0.35 and cd:at(e[1], e[2]) < 22 and (i * 7919) % 9 == 0 then
    chains = chains + 1
    local j = i
    while j and not painted(oak.limbs[j]) and not seen[j] do
      seen[j] = true
      local m = oak.limbs[j]
      if #m.pts >= 2 then
        b:stroke(m.pts, {pressure={b:pressure_for(math.max(0.3, m.w[1])), 0}, ramps={0.05, 0.7}})
        marks = marks + 1
        if marks % 6 == 0 then b:reload("#4a4139", 0.8) end
      end
      j = m.parent
    end
  end
end
print("chains", chains, "strokes", marks)

--@ chunk 9 · clock 1440
-- B7. the oak goes into the ground: the ground's own color (sampled beside the trunk) dragged over
-- the wet foot, so the trunk's dark is pulled a little into it. (Long shadow strokes laid into the open
-- ground came out as streaky blue-gray smears with pale rims: rejected, crop in notes/lab/baretree.md.)
local foot = ellipse(454, 680, 42, 11):roughen(5, 10, 5, 2):soften(3)
work(foot, {hand="body", tool="filbert 4", length={8, 22}, angle=0.02, angle_jitter=0.15, coverage=1.8, medium=0.2, load=0.6, hug=false,
  color=function(x, y) return shift(sample(math.abs(x - 454) < 24 and (x < 454 and 400 or 510) or x, y + 4, 3), -0.02, 0, 0) end})
print(drying(452, 670), drying(600, 680))

--@ chunk 10 · clock 1440
-- B8. the cast shadow as a soft glaze once the ground has set, starting inside the foot so the
-- trunk's dark, the contact and the shadow read as one shape
local fx, fy = 452, 668
local cast = mask(function(x, y)
  local t = (x - fx) / 430
  if t < -0.12 or t > 1 then return 0 end
  local cy = fy + 8 + 18 * t
  local hw = 9 + 16 * t
  local d = math.abs(y - cy)
  return (1 - smoothstep(hw * 0.5, hw, d)) * (1 - smoothstep(0.05, 1, t)) * smoothstep(-0.12, -0.02, t)
end):blur(3)
glaze(cast * land, {color="#3f3d33", coats=0.38, pigment="transparent"})

--@ chunk 11 · clock 40062.50390625
-- B9. the twig mass stated a second time, dry-brushed over the set first tone: the same lean radial
-- strokes, now weighted to the crown's outer part (where an oak's fine twigs crowd)
-- so the crown's edge is a warm fringe the limbs run out into; cut off the stout wood by its EXACT
-- mask (a grown cut left pale halos), the thin limbs may go lost in it
local cd = oak:crown_mask():distance()
OUTER = mask(function(x, y)
  local d = cd:at(x, y)
  return DEN:at(x, y) * (0.2 + 0.8 * (1 - smoothstep(25, 110, d)))
end)
work(OUTER - oak:wood(1.2), {hand="body", tool="round 2", length={8, 22}, coverage=1.6, load=0.35, medium=0.25, hug=false,
  angle=RADIAL, angle_jitter=0.3, pressure={0.45, 0.05}, ramps={0.1, 0.8},
  color_over=function(x, y, under) return mix(under, "#554a42", 0.45 * OUTER:at(x, y)) end})

--@ chunk 12 · clock 40062.50390625
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
