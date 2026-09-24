-- easel session "lab3_bt_A": a painting replayed chunk by chunk.
--   easel run paintings/lua/lab3_bt_A.lua [--width 3200]
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

-- A1. the sky, first thin layer, a shade duller than the target (sketchbook 2)
work(skym, {hand="broad", color=function(x, y) return shift(sky(x, y), -0.02, 0, 0) end, angle=0, coverage=4.2, medium=0.3, pal=skypal})
blend(skym, {angle=0})

--@ chunk 3 · clock 0

-- A2. a day later: the second sky layer stippled over the set first one (sketchbook 2)
wait(24*60)
print(drying(500, 300))
stipple(skym, {width=2.6, color=sky, coverage=2.6, pressure={0.4, 0.8}, dips={18, 0.35, 0.7}, medium=0.55, pal=skypal})

--@ chunk 4 · clock 1440

-- A3. dry before the ground goes over the sky edge; the ground as tone (sketchbook 1)
dry()
work(land, {hand="body", color=ground, length={20, 60}, angle=0.02, coverage=3.4, medium=0.18})

--@ chunk 5 · clock 9126.423828125

-- A4. cut the sky back over the crenellated crest (sketchbook 4), on dry paint
dry()
local cn = noise{seed=21, period=40}
local crest = function(x) return GROUND(x) - 1 + 1.5 * cn(x, 0) end
local cut = mask(function(x, y) return smoothstep(crest(x) - 14, crest(x) - 8, y) * (1 - smoothstep(crest(x) - 0.5, crest(x) + 0.5, y)) end)
work(cut, {hand="detail", tool="round 2", color=function(x, y) return sample(x, crest(x) - 26, 3) end, angle=0, angle_jitter=0.1, length={10, 30}, coverage=3, medium=0.3, clip=cut})
blend(cut, {angle=0, clip=cut})

--@ chunk 6 · clock 21666.8955078125

-- A5. the oak, the recipe as written (sketchbook 5, tree_in; trees_in.lua paint_wood), on dry paint
dry()
local dark, light = "#3b342c", "#7d7566"
local thick = oak:wood(3.5)
work(thick, {hand="body", tool="round 2", length={4, 12}, coverage=3.5, angle=1.5, angle_jitter=0.6, clip=thick, color=dark, medium=0.15})
-- the lit flank only on the stout wood, on the sun side (left)
local stout = oak:wood(7)
local flank = stout * mask(function(x, y) return 1 - stout:at(x - 3, y + 1.2) end)
work(flank, {hand="body", tool="round 1.4", length={3, 8}, coverage=2.5, angle=1.5, clip=thick, color=light, medium=0.15})
oak:paint_wood(brush("round", 2.4), {color=dark, min=1.2, max=3.5})
-- the twigs a shade lighter than the limbs: the crown lace against the sky
oak:paint_wood(brush("rigger", 0.55), {color="#554e45", max=1.2, pressure=0.04})
-- the few dead leaves an oak keeps
oak:paint(brush("round", oak.touch_w * 0.7), {color="#6e5234", every=4, share=0.4})

--@ chunk 7 · clock 31733.5595703125

-- A6. the oak casts a long shadow to the right under the low sun (sketchbook 7/11: soft mask, blurred, low coats)
dry()
local fx, fy = 452, 660
local sh = mask(function(x, y)
  local t = (x - fx) / 430
  if t < -0.05 or t > 1 then return 0 end
  local cy = fy + 6 + 18 * t
  local hw = 5 + 16 * t
  local d = math.abs(y - cy)
  return (1 - smoothstep(hw * 0.6, hw, d)) * (1 - smoothstep(0.1, 1, t)) * smoothstep(-0.05, 0.02, t)
end):blur(3)
glaze(sh * land, {color="#4b4a40", coats=0.3, pigment="transparent"})

--@ chunk 8 · clock 47385.111328125

-- A7. winter grass: bury the foot and give the near ground particulars (sketchbook 5 roots, 8 blades)
local b = brush("rigger", 0.8)
local cols = {"#8a7d5c", "#3f3a2c", "#a39266", "#4d4634", "#6b6248", "#2f2c24"}
local lean = noise{seed=33, period=180}
local n = 0
local function blade(x, y, depth)
  local h = (6 + 26 * depth) * rand(0.5, 1.5)
  local a = -math.pi/2 + 0.5 * lean(x, y) + randn(0, 0.22)
  local c = randn(0, 0.25)
  local mx, my = x + math.cos(a) * h * 0.5, y + math.sin(a) * h * 0.5
  local tx, ty = x + math.cos(a + c) * h, y + math.sin(a + c) * h
  if n % 7 == 0 then b:reload(cols[(n // 7) % #cols + 1], 0.7) end
  b:stroke({{x, y}, {mx, my}, {tx, ty}}, {pressure={0.35 + 0.5 * depth, 0}, ramps={0.05, 0.75}})
  n = n + 1
end
-- a tuft round the foot
for i = 1, 320 do
  local x = 452 + randn(0, 26)
  local y = 674 + randn(0, 6)
  blade(x, y, 0.5)
end
-- sparse blades over the near ground, denser toward us
for i = 1, 700 do
  local y = HZ + 20 + (H - HZ - 20) * math.sqrt(rand())
  local x = rand(0, W)
  local d = smoothstep(HZ, H, y)
  if rand() < 0.3 + 0.7 * d then blade(x, y, d) end
end
print(n)

--@ chunk 9 · clock 47385.111328125
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
