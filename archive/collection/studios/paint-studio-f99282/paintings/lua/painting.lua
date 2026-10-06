-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ engine 2

--@ chunk 1
canvas{size=720, aspect=1.28, linen={12, 11}, seed=2411,
  ground={
    {pile={{"yellow ochre",3},{"red earth",2},{"lead white",1}}, um=55, apply="knife", texture=0.55},
    {pile={{"lead white",6},{"raw umber",2},{"red earth",1}}, um=26, apply="knife", texture=0.25},
    {pile={{"lead white",8},{"yellow ochre",1},{"raw umber",1}}, um=16, apply="brush"},
  }}
print(W, H)

--@ chunk 2
-- underdrawing, first pass: faint hard pencil, a few light passes per line
p1 = pencil("2H")
hz = 445

r1 = {{-10,428},{60,420},{130,410},{200,400},{270,396},{330,402},{390,398},
     {450,388},{510,372},{570,352},{630,334},{690,320},{740,322},{790,332},
     {840,344},{890,356},{940,366},{1010,376}}

r2 = {{-10,442},{50,424},{110,406},{170,396},{225,390},{275,394},{325,404},
     {375,414},{425,424},{475,431},{525,436},{575,439},{700,441},{1010,443}}

r3 = {{770,440},{820,430},{870,424},{920,420},{960,418},{1010,418}}

sh = {{-10,706},{60,696},{140,688},{230,674},{320,662},{400,654},{470,651},
     {540,654},{610,660},{690,672},{780,684},{870,694},{1010,704}}

for _, pts in ipairs({r1, r2, r3, sh}) do
  p1:sketch(pts, {pressure=0.26, passes=2, wander=3})
end
p1:rule({-10, hz}, {1010, hz}, {pressure=0.2})
print("pass one done")

--@ chunk 3
-- the drawing's second pass: bolder pencil on the ridges, the trees, the figure
p2 = pencil("HB")
hz = 445

r1b = {{-10,427},{70,418},{145,408},{215,399},{280,394},{335,401},{392,397},
      {455,386},{515,370},{575,350},{635,332},{695,318},{745,321},{795,331},
      {845,343},{895,356},{945,367},{1010,378}}
r2b = {{-10,441},{55,422},{115,404},{175,395},{228,389},{278,393},{328,403},
      {378,413},{428,423},{478,430},{528,435},{578,438},{700,440},{1010,442}}
r3b = {{772,439},{822,429},{872,423},{922,419},{962,417},{1010,417}}
shb = {{-10,705},{65,695},{145,687},{235,673},{325,661},{402,653},{472,650},
      {542,653},{612,659},{692,671},{782,683},{872,693},{1010,703}}

p2:sketch(r1b, {pressure=0.42, passes=2, wander=3})
p2:sketch(r2b, {pressure=0.45, passes=2, wander=3})
p2:sketch(r3b, {pressure=0.42, passes=2, wander=2})
p2:sketch(shb, {pressure=0.40, passes=2, wander=4})

-- a few trees on the far bank, drawn small: upright axes with side branches
function twig(x, y, ang, len, depth, out)
  local pts, n = {{x, y}}, 4
  local px, py, a = x, y, ang
  for i = 1, n do
    a = a + randn(0, 0.16)
    px, py = px + math.cos(a) * len / n, py + math.sin(a) * len / n
    pts[#pts + 1] = {px, py}
    if depth >= 1 and i == 2 and rand() < 0.85 then
      twig(px, py, a + (rand() < 0.5 and -1 or 1) * rand(0.75, 1.2), len * rand(0.6, 0.8), depth - 1, out)
    end
  end
  if depth >= 1 then
    twig(px, py, a + randn(0.05, 0.22), len * rand(0.62, 0.78), depth - 1, out)
  end
  out[#out + 1] = pts
end

far = {}
for x, top in ipairs({305, 318, 352, 690, 706, 745, 840}) do
  local base = 441 + randn(0, 1.5)
  local tt = {}
  twig(x, base, -math.pi / 2 + randn(0, 0.08), base - top, 2, tt)
  for _, t in ipairs(tt) do far[#far + 1] = t end
end
for _, t in ipairs(far) do p2:line(t, {pressure=0.30, smooth=true}) end
print("ridges and far trees")

--@ chunk 4
-- the bare trees at the left: upright axes, side limbs strong near the tip of each shoot,
-- each tip forking.  Trunks run out of the picture at the bottom and the top.
pT, pL, pW = pencil("H"), pencil("HB"), pencil("2H")

function limb(x, y, ang, len, depth, sink)
  local n = (depth == 3) and 6 or 3
  local px, py, a = x, y, ang
  local pts = {{x, y}}
  for i = 1, n do
    a = a + randn(0, 0.11)
    px, py = px + math.cos(a) * len / n, py + math.sin(a) * len / n
    pts[#pts + 1] = {px, py}
    if depth > 0 and rand() < 0.8 then
      local s = (rand() < 0.5) and -1 or 1
      limb(px, py, a + s * rand(0.85, 1.4), len * rand(0.42, 0.68), depth - 1, sink)
    end
  end
  sink(pts, depth)
  if depth > 0 then limb(px, py, a + randn(0.06, 0.24), len * rand(0.6, 0.78), depth - 1, sink) end
end

function treedraw(px, py, ang, len, tr, br, tw)
  local function sink(pts, depth)
    local g, pr = tw, 0.34
    if depth == 3 then g, pr = tr, 0.5 elseif depth == 2 then g, pr = br, 0.42 end
    g:line(pts, {pressure=pr, smooth=true})
  end
  limb(px, py, ang, len, 3, sink)
end

treedraw(84, 830, -math.pi / 2 + 0.05, 900, pT, pL, pW)
treedraw(172, 812, -math.pi / 2 - 0.03, 720, pT, pL, pW)
print("trees")

--@ chunk 5
-- how dark can a pencil line get on this ground?
for i, g in ipairs({"4B", "B", "HB", "H", "2H"}) do
  local h = pencil(g)
  h:line({{60, 40 + i * 30}, {400, 30 + i * 30}}, {pressure=0.9})
end
print("test")

--@ chunk 6
erase(everywhere(), {strength=0.92})
print("cleaned")

--@ chunk 7
-- the drawing, second pass: the structure, in a bolder hand
hH, hB = pencil("H"), pencil("4B")
hz = 445

-- far range: highest where the light is, right of centre
r1 = {{-10,427},{70,418},{145,408},{215,399},{280,394},{335,401},{392,397},
     {455,386},{515,370},{575,350},{635,332},{695,318},{745,321},{795,331},
     {845,343},{895,356},{945,367},{1010,378}}
-- middle range: the left massif, closing in from the left
r2 = {{-10,441},{55,422},{115,404},{175,395},{228,389},{278,393},{328,403},
     {378,413},{428,423},{478,430},{528,435},{578,438},{700,440},{1010,442}}
-- near bank: low, wooded, right of centre
r3 = {{772,439},{822,429},{872,423},{922,419},{962,417},{1010,417}}
-- the water's edge in front
sh = {{-10,705},{65,695},{145,687},{235,673},{325,661},{402,653},{472,650},
     {542,653},{612,659},{692,671},{782,683},{872,693},{1010,703}}

hH:sketch(r1, {pressure=0.5, passes=2, wander=3})
hH:sketch(r2, {pressure=0.55, passes=2, wander=3})
hH:sketch(r3, {pressure=0.5, passes=2, wander=2})
hB:sketch(sh, {pressure=0.45, passes=2, wander=4})
hH:rule({-10, hz}, {1010, hz}, {pressure=0.28})

-- the mist lying on the water, its top and bottom
hB:sketch({{-10,436},{120,430},{260,437},{400,428},{540,431},{680,424},{820,429},{1010,422}},
          {pressure=0.3, passes=1, wander=6})
hB:sketch({{-10,486},{140,478},{300,484},{460,476},{620,482},{800,474},{1010,478}},
          {pressure=0.25, passes=1, wander=6})
print("structure")

--@ chunk 8
-- the trees.  A bare tree: an upright axis built of shoots, each shoot's tip forking,
-- side shoots leaving at 50-80 degrees near the tip and turning up after the first node.
gA, gB, gC = pencil("4B"), pencil("B"), pencil("H")

function shoot(x, y, ang, len, depth, sink, grav)
  local n = (depth >= 4) and 3 or 2
  local pts, px, py, a = {{x, y}}, x, y, ang
  for i = 1, n do
    a = a + randn(0, 0.07) + (grav or 0)      -- grav: turn back up
    px, py = px + math.cos(a) * len / n, py + math.sin(a) * len / n
    pts[#pts + 1] = {px, py}
  end
  sink(pts, depth)
  if depth <= 0 then return end
  local tip, k = {px, py}, 0
  while k < 3 do
    k = k + 1
    local s = (rand() < 0.5) and -1 or 1
    local d = (k == 1) and 0 or 0
    if rand() < 0.85 then
      shoot(px, py, a + s * rand(0.85, 1.35) + d, len * rand(0.55, 0.82), depth - 1, sink, 0.12)
    end
  end
  shoot(px, py, a + randn(0.05, 0.2), len * rand(0.72, 0.88), depth - 1, sink, grav)
end

function tree(px, py, ang, len, depth)
  local function sink(pts, d)
    local g, pr = gC, 0.4
    if d >= 5 then g, pr = gA, 0.75
    elseif d == 4 then g, pr = gB, 0.6
    elseif d == 3 then g, pr = gC, 0.5 end
    g:line(pts, {pressure=pr, smooth=true})
  end
  shoot(px, py, ang, len, depth, sink, 0.02)
end

tree(96, 840, -math.pi / 2 + 0.04, 210, 5)
tree(184, 820, -math.pi / 2 - 0.02, 165, 4)
print("trees")

--@ chunk 9
erase(rect(-5, -5, 430, 790), {strength=0.95})
erase(rect(-5, -5, 430, 790), {strength=0.9})
print("left field cleared")

--@ chunk 10
-- The trees.  Each is an upright axis; heavy limbs leave near the top of the shoots,
-- sweep out at 50-80 degrees, then turn up; secondaries leave those in turn.
-- Only three orders are drawn: the finest twigs are left to the paint.
gA, gB, gC = pencil("4B"), pencil("B"), pencil("H")

local function curve(x, y, ang, len, bend, n)
  local pts, a, px, py = {{x, y}}, ang, x, y
  for i = 1, n do
    a = a + bend / n + randn(0, 0.03)
    px, py = px + math.cos(a) * len / n, py + math.sin(a) * len / n
    pts[#pts + 1] = {px, py}
  end
  return pts, px, py, a
end

local function limb(x, y, ang, len, order, grav)
  local bend = -grav * rand(0.5, 1.0)          -- grav>0 turns the tip upward
  local pts, tx, ty, ta = curve(x, y, ang, len, bend, 4)
  local g, pr = gA, 0.8
  if order == 2 then g, pr = gB, 0.6 else g, pr = gC, 0.45 end
  g:line(pts, {pressure=pr, smooth=true})
  if order >= 3 then return end
  local n = (order == 0) and 3 or 2
  for i = 1, n do
    if rand() < 0.8 then
      local t = i / (n + 1)
      local j = 1 + math.floor(t * 3)
      local bx, by = pts[j][1], pts[j][2]
      local s = (i % 2 == 0) and -1 or 1
      limb(bx, by, ta + s * rand(0.6, 1.1), len * rand(0.38, 0.58), order + 1, grav + 0.25)
    end
  end
  return tx, ty, ta
end

-- the near tree, its base below the frame and its crown out of the top
trunkA = curve(96, 850, -math.pi / 2 + 0.05, 830, -0.06, 6)
gA:line(trunkA, {pressure=0.85, smooth=true})
for _, n in ipairs({{5, -0.95, 190}, {3, 1.05, 165}, {4, -1.25, 215}, {2, 1.15, 150},
                    {3, -0.8, 175}, {1, 1.0, 120}}) do
  local bx, by = trunkA[n[1]][1], trunkA[n[1]][2]
  limb(bx, by, -math.pi / 2 + n[2], n[3], 0, 0.3)
end
print("tree A")

--@ chunk 11
print(trunkA[1], trunkA[4], trunkA[7])
print(#trunkA, gA.worn)

--@ chunk 12
for i = 1, 7 do print(i, trunkA[i][1], trunkA[i][2]) end

--@ chunk 13
erase(rect(-5, -5, 430, 790), {strength=0.95})
erase(rect(-5, -5, 430, 790), {strength=0.9})

-- The trees, redrawn: each axis rises nearly straight; every limb leaves at 50-80 degrees
-- and turns up towards its tip, which is the direction its secondaries take over.
gA, gB, gC = pencil("4B"), pencil("B"), pencil("H")

local function draw(pts, order)
  local g, pr = gC, 0.42
  if order == 0 then g, pr = gA, 0.8 elseif order == 1 then g, pr = gB, 0.6 end
  g:line(pts, {pressure=pr, smooth=true})
end

local function limb(x, y, ang, tipang, len, order)
  local pts, n = {{x, y}}, 4
  local a, px, py = ang, x, y
  for i = 1, n do
    a = a + (tipang - a) * 0.32 + randn(0, 0.022)
    px, py = px + math.cos(a) * len / n, py + math.sin(a) * len / n
    pts[#pts + 1] = {px, py}
  end
  draw(pts, order)
  if order >= 2 then return end
  for i = 1, 2 do
    if rand() < 0.85 then
      local bx, by = pts[1 + i][1], pts[1 + i][2]
      local s = (i == 1) and 1 or -1
      local ba = a + s * rand(0.5, 0.95)
      limb(bx, by, ba, ba + s * rand(0.05, 0.45), len * rand(0.34, 0.55), order + 1)
    end
  end
end

local function axis(x, y, len, lean)
  local pts, a, px, py = {{x, y}}, -math.pi / 2 + lean, x, y
  for i = 1, 7 do
    a = a + randn(0, 0.012)
    px, py = px + math.cos(a) * len / 7, py + math.sin(a) * len / 7
    pts[#pts + 1] = {px, py}
  end
  return pts
end

trunkA = axis(96, 860, 850, 0.05)
draw(trunkA, 0)
-- height along the trunk (1..7), side (1 right, -1 left), angle off vertical, length
for _, b in ipairs({{6, 1, 0.95, 175}, {5, -1, 1.15, 200}, {4, 1, 1.3, 190},
                    {3, -1, 0.85, 160}, {3, 1, 0.7, 130}, {2, -1, 1.0, 120}}) do
  local bx, by = trunkA[b[1]][1], trunkA[b[1]][2]
  local a0 = -math.pi / 2 + b[2] * b[3]
  limb(bx, by, a0, a0 - b[2] * 0.45, b[4], 1)
end
print("tree A")

--@ chunk 14
draw = function(pts, order)
  local g, pr = gC, 0.42
  if order == 0 then g, pr = gA, 0.8 elseif order == 1 then g, pr = gB, 0.6 end
  g:line(pts, {pressure=pr, smooth=true})
end

limb = function(x, y, ang, tipang, len, order)
  local pts, n = {{x, y}}, 4
  local a, px, py = ang, x, y
  for i = 1, n do
    a = a + (tipang - a) * 0.32 + randn(0, 0.022)
    px, py = px + math.cos(a) * len / n, py + math.sin(a) * len / n
    pts[#pts + 1] = {px, py}
  end
  draw(pts, order)
  if order >= 2 then return end
  for i = 1, 2 do
    if rand() < 0.85 then
      local bx, by = pts[1 + i][1], pts[1 + i][2]
      local s = (i == 1) and 1 or -1
      local ba = a + s * rand(0.5, 0.95)
      limb(bx, by, ba, ba + s * rand(0.05, 0.45), len * rand(0.34, 0.55), order + 1)
    end
  end
end

axis = function(x, y, len, lean)
  local pts, a, px, py = {{x, y}}, -math.pi / 2 + lean, x, y
  for i = 1, 7 do
    a = a + randn(0, 0.012)
    px, py = px + math.cos(a) * len / 7, py + math.sin(a) * len / 7
    pts[#pts + 1] = {px, py}
  end
  return pts
end

trunkB = axis(252, 700, 800, 0.02)
draw(trunkB, 1)
for _, b in ipairs({{6, -1, 1.0, 150}, {5, 1, 1.2, 165}, {4, -1, 1.3, 140},
                    {3, 1, 0.9, 120}, {4, 1, 0.6, 95}}) do
  local bx, by = trunkB[b[1]][1], trunkB[b[1]][2]
  local a0 = -math.pi / 2 + b[2] * b[3]
  limb(bx, by, a0, a0 - b[2] * 0.4, b[4], 2)
end
print("tree B")

--@ chunk 15
erase(rect(395, 385, 580, 790), {strength=0.95})

-- the far trees: a thin stand on the left massif's foot and on the right bank,
-- and a small tower among the right-hand ones
hF = pencil("2H")
local function farstand(x0, x1, base, top, n)
  for i = 1, n do
    local x = lerp(x0, x1, (i - 0.5 + randn(0, 0.25)) / n)
    local t = top + randn(0, 9)
    local a = -math.pi / 2 + randn(0, 0.12)
    local len, pts = base - t, {{x, base}}
    for k = 1, 3 do
      a = a + randn(0, 0.1)
      x, t2 = x + math.cos(a) * len / 3, nil
      pts[#pts + 1] = {x, base - (base - t) * k / 3}
    end
    hF:line(pts, {pressure=0.5, smooth=true})
    for k = 1, 2 do
      local by = lerp(t, base, k / 3)
      local s = (k == 1) and 1 or -1
      hF:line({{x, by}, {x + s * rand(3, 9), by - rand(2, 8)}}, {pressure=0.45})
    end
  end
end
farstand(0, 120, 443, 420, 9)
farstand(800, 900, 428, 404, 7)
farstand(930, 1000, 420, 398, 6)
hF:line({{946, 418}, {946, 400}, {949, 399}, {950, 418}}, {pressure=0.6, smooth=false})
print("far stands")

--@ chunk 16
-- the figure: a woman at the water's edge, seen from behind, a bundle on her back.
-- 55 units tall, feet on the bank at y 655.
hF2 = pencil("2H")
fig = {}
-- the bundle of faggots, a wedge of sticks over her left shoulder
for i = 1, 7 do
  local a = -2.5 + i * 0.13 + randn(0, 0.03)
  fig[#fig + 1] = {{560 + 3, 604}, {560 + 3 + math.cos(a) * 26, 604 + math.sin(a) * 26}}
end
-- head and shoulders
hF2:line({{557, 600}, {558, 606}, {562, 610}, {566, 606}, {566, 599}, {561, 597}},
         {pressure=0.7, smooth=true})
-- the body: a shawl falling to the knee, seen from behind
hF2:line({{557, 604}, {552, 610}, {550, 622}, {552, 634}, {551, 640}},
         {pressure=0.7, smooth=true})
hF2:line({{566, 605}, {570, 612}, {571, 624}, {568, 636}, {569, 641}},
         {pressure=0.7, smooth=true})
-- the skirt to the ankles, and the feet
hF2:line({{551, 638}, {556, 650}, {555, 655}}, {pressure=0.7})
hF2:line({{569, 639}, {566, 650}, {568, 655}}, {pressure=0.7})
hF2:line({{550, 640}, {569, 641}}, {pressure=0.55})
-- her shadow on the bank
hF2:line({{555, 655}, {545, 659}}, {pressure=0.5})

-- the boat, drawn up on the bank to her right: 46 units long, bow toward us
hB2 = pencil("H")
hB2:line({{600, 676}, {612, 672}, {630, 670}, {646, 670}, {658, 674}},
         {pressure=0.6, smooth=true})
hB2:line({{600, 676}, {606, 680}, {620, 681}, {640, 680}, {656, 676}},
         {pressure=0.6, smooth=true})
hB2:line({{604, 680}, {605, 676}}, {pressure=0.6})
hB2:line({{652, 676}, {653, 680}}, {pressure=0.6})

-- reeds along the edge of the bank
for i = 1, 34 do
  local x = lerp(330, 720, i / 34) + randn(0, 8)
  local by = 663 + 6 * math.sin(x / 90)
  local hgt = rand(9, 26)
  hB2:line({{x, by}, {x + randn(0, 3), by - hgt}}, {pressure=rand(0.4, 0.7)})
end
-- three birds, high up
for _, b in ipairs({{252, 152, 6}, {288, 124, 5}, {332, 140, 4}}) do
  local x, y, w = b[1], b[2], b[3]
  hB2:line({{x - w, y}, {x, y - w * 0.5}, {x + w, y}}, {pressure=0.8})
end
print("figure, boat, reeds, birds")

--@ chunk 17
-- The sky: one thin veil, a little oil in it so it settles into the ground's texture.
sky = above(function(x) return 452 end)
skyHi = pile{{"pale smalt", 4}, {"lead white", 5}, {"raw umber", 1.1}, medium=0.35}
skyLo = pile{{"lead white", 7}, {"yellow ochre", 2.6}, {"pale smalt", 1.2},
             {"raw umber", 0.7}, medium=0.3}
skyGlow = pile{{"lead white", 6}, {"yellow ochre", 3}, {"vermilion", 0.8},
               {"raw umber", 0.5}, medium=0.25}

work(sky, {hand="glaze", pile=skyHi, coverage=1.5, angle=-0.03,
           length={140, 260}, pressure={0.35, 0.5}, curve={0, 10}})
print("sky veil")

--@ chunk 18
-- the veil wants knitting together: more of the same, laid light and long
work(sky, {hand="glaze", pile=skyHi, coverage=2.4, fill=true, angle=-0.03,
           length={180, 300}, pressure={0.4, 0.55}})
print("sky veil, second laying")

--@ chunk 19
print(wait(300))
print(drying(500, 200), drying(500, 420), drying(500, 600))

--@ chunk 20
print(wait(1080))
print(drying(500, 200), drying(500, 300), drying(500, 430))

--@ chunk 21
-- The zenith deepened by stippling: no blending, the gradation made of small touches.
skyDeep = pile{{"smalt", 3}, {"pale smalt", 2}, {"lead white", 2.4},
               {"raw umber", 1.2}, medium=0.25}
skyCov = function(x, y)
  local v = clamp((392 - y) / 300, 0, 1)
  v = v * v * (3 - 2 * v)
  return v * (0.78 + 0.22 * (1 - x / 1000)) * 1.15
end
stipple(sky, {pile=skyDeep, width=2.2, coverage=skyCov, pressure={0.3, 0.62},
              cluster=0.35, feather=0.5, dips={26, 0.55, 0.5}})
print("zenith stippled")

--@ chunk 22
-- the zenith stipple wants knitting into the film beneath it, and the lower sky
-- wants warming toward the horizon, brightest where the sun went down
palerTop = pile{{"smalt", 1.4}, {"pale smalt", 3}, {"lead white", 5},
               {"raw umber", 1}, medium=0.3}
warmLow = pile{{"lead white", 8}, {"yellow ochre", 2.2}, {"pale smalt", 1.4},
               {"raw umber", 0.9}, medium=0.25}

stipple(above(function(x) return 290 end), {pile=palerTop, width=2.6,
        coverage=function(x, y) return clamp((300 - y) / 200, 0, 1) * 0.85 end,
        pressure={0.25, 0.5}, cluster=0.3, feather=0.5, dips={26, 0.5, 0.5}})

stipple(sky, {pile=warmLow, width=2.2,
        coverage=function(x, y)
          local v = clamp((y - 180) / 250, 0, 1)
          v = v * v * (3 - 2 * v)
          return v * 1.1
        end,
        pressure={0.25, 0.55}, cluster=0.3, feather=0.5, dips={26, 0.5, 0.5}})
print("sky graded")

--@ chunk 23
-- ridgelines as functions of x, so each plane can be a mask between its crest and the water
ridgefn = function(pts)
  local n = #pts
  return function(x)
    if x <= pts[1][1] then return pts[1][2] end
    if x >= pts[n][1] then return pts[n][2] end
    for i = 1, n - 1 do
      if x >= pts[i][1] and x <= pts[i + 1][1] then
        local t = (x - pts[i][1]) / (pts[i + 1][1] - pts[i][1])
        return pts[i][2] + (pts[i + 1][2] - pts[i][2]) * (t * t * (3 - 2 * t))
      end
    end
  end
end

r1 = {{-10,427},{70,418},{145,408},{215,399},{280,394},{335,401},{392,397},
     {455,386},{515,370},{575,350},{635,332},{695,318},{745,321},{795,331},
     {845,343},{895,356},{945,367},{1010,378}}
r2 = {{-10,441},{55,422},{115,404},{175,395},{228,389},{278,393},{328,403},
     {378,413},{428,423},{478,430},{528,435},{578,438},{700,440},{1010,442}}
r3 = {{772,439},{822,429},{872,423},{922,419},{962,417},{1010,417}}

f1, f2, f3 = ridgefn(r1), ridgefn(r2), ridgefn(r3)

mFar  = above(f1) * below(function() return 462 end)
mMid  = above(f2) * below(function() return 456 end)
mBank = above(f3) * below(function() return 452 end)
mFar:roughen(3.5, 22, 7)
mMid:roughen(2.5, 18, 11)
mBank:roughen(4, 14, 3)

farP  = pile{{"lead white", 5}, {"pale smalt", 1.5}, {"raw umber", 1.3},
             {"yellow ochre", 0.6}, medium=0.45}
midP  = pile{{"lead white", 3.4}, {"pale smalt", 1.7}, {"raw umber", 2},
             {"bone black", 0.4}, medium=0.35}

work(mFar, {hand="glaze", pile=farP, coverage=1.3, angle=0, clip=true,
            length={100, 200}, pressure={0.4, 0.6}})
work(mMid, {hand="glaze", pile=midP, coverage=1.3, angle=0, clip=true,
            length={100, 200}, pressure={0.4, 0.6}})
print("far and middle ranges")

--@ chunk 24
-- the ranges need more body than an oil-rich veil gave them
farP = pile{{"lead white", 4}, {"pale smalt", 1.6}, {"raw umber", 2.2},
            {"yellow ochre", 0.6}, medium=0.2}
midP = pile{{"lead white", 2.6}, {"pale smalt", 1.8}, {"raw umber", 3},
            {"bone black", 1.2}, medium=0.15}
work(mFar, {hand="body", pile=farP, coverage=1.8, fill=true, angle=0, clip=true,
            length={30, 70}, pressure={0.4, 0.65}})
work(mMid, {hand="body", pile=midP, coverage=1.8, fill=true, angle=0, clip=true,
            length={30, 70}, pressure={0.4, 0.65}})
print("ranges, second laying")

--@ chunk 25
print("far", mFar:at(700,340), mFar:at(700,400), mFar:at(700,300), mFar:at(700,470))
print("mid", mMid:at(200,400), mMid:at(200,440), mMid:at(200,370))
print("bank", mBank:at(900,425), mBank:at(900,440), mBank:at(900,410))
print("areas", mFar:area(), mMid:area(), mBank:area())

--@ chunk 26
a1 = above(f1); b1 = below(function() return 462 end)
print("above(f1)", a1:at(700,300), a1:at(700,400), a1:area())
print("below(462)", b1:at(700,400), b1:at(700,300), b1:at(700,700), b1:area())
print("f1(700)", f1(700), "f1(200)", f1(200))

--@ chunk 27
-- I had the bands the wrong way up: a mountain's body lies below its crest.
mFar  = below(f1) * above(function() return 462 end)
mMid  = below(f2) * above(function() return 456 end)
mBank = below(f3) * above(function() return 452 end)
mFar:roughen(3.5, 22, 7)
mMid:roughen(2.5, 18, 11)
mBank:roughen(4, 14, 3)
print("areas", mFar:area(), mMid:area(), mBank:area())

work(mFar, {hand="body", pile=farP, coverage=1.6, angle=0, clip=true,
            length={30, 70}, pressure={0.4, 0.65}})
work(mMid, {hand="body", pile=midP, coverage=1.6, angle=0, clip=true,
            length={30, 70}, pressure={0.4, 0.65}})
print("ranges laid")

--@ chunk 28
-- The distant planes want an even film, not blobs, and a cooler colour: the warm ground
-- comes through a thin film and turns any earth into sand.
farP = pile{{"lead white", 3}, {"pale smalt", 2.2}, {"raw umber", 2.4},
            {"bone black", 0.35}, medium=0.15}
midP = pile{{"lead white", 2.2}, {"pale smalt", 2}, {"raw umber", 3},
            {"bone black", 1.2}, medium=0.12}
mFar  = below(f1) * above(function() return 472 end)
mMid  = below(f2) * above(function() return 466 end)
mFar:roughen(3.5, 22, 7)
mMid:roughen(2.5, 18, 11)

work(mFar, {hand="glaze", pile=farP, coverage=2.2, fill=true, angle=0, clip=true,
            length={120, 240}, pressure={0.4, 0.62}})
work(mMid, {hand="glaze", pile=midP, coverage=2.2, fill=true, angle=0, clip=true,
            length={120, 240}, pressure={0.4, 0.62}})
print("ranges, evened")

--@ chunk 29
print(wait(1440))
print(drying(700, 400), drying(200, 420))

--@ chunk 30
-- cooler films over the ranges, and the middle range's low right-hand spur taken back
-- into the mist so it stops reading as a bar
farP = pile{{"lead white", 2.4}, {"pale smalt", 2}, {"smalt", 1.6},
            {"raw umber", 1.6}, medium=0.15}
midP = pile{{"lead white", 1.6}, {"pale smalt", 1.8}, {"smalt", 1.8},
            {"raw umber", 2.4}, {"bone black", 0.8}, medium=0.12}
spurFade = mask(function(x, y) return clamp((690 - x) / 230, 0, 1) end)
mMid = mMid * spurFade

work(mFar, {hand="glaze", pile=farP, coverage=2.2, fill=true, angle=0, clip=true,
            length={120, 240}, pressure={0.4, 0.62}})
work(mMid, {hand="glaze", pile=midP, coverage=2.4, fill=true, angle=0, clip=true,
            length={120, 240}, pressure={0.4, 0.62}})
print("ranges, cooled")

--@ chunk 31
-- gullies down the flanks, and the warm rim along the crest where the sun went down
wedge = function(ax, ay, bx, by, wa, wb)
  local dx, dy, L = bx - ax, by - ay, math.sqrt((bx - ax)^2 + (by - ay)^2)
  local nx, ny = -dy / L, dx / L
  return poly({{ax + nx * wa / 2, ay + ny * wa / 2}, {ax - nx * wa / 2, ay - ny * wa / 2},
               {bx - nx * wb / 2, by - ny * wb / 2}, {bx + nx * wb / 2, by + ny * wb / 2}})
end

shadeP = pile{{"pale smalt", 1.4}, {"raw umber", 2.2}, {"bone black", 0.9}, medium=0.3}

local farFolds, midFolds = {}, {}
for _, f in ipairs({{565, 352, 522, 428, 6, 15}, {612, 334, 578, 414, 7, 17},
                    {652, 323, 628, 398, 6, 14}, {742, 322, 778, 402, 6, 15},
                    {792, 332, 828, 408, 7, 16}, {842, 344, 872, 414, 6, 14}}) do
  local m = wedge(f[1], f1(f[1]) + 2, f[3], f[4], f[5], f[6]):soften(7)
  farFolds[#farFolds + 1] = m * mFar
end
for _, f in ipairs({{96, 410, 74, 452, 6, 14}, {158, 399, 140, 452, 7, 16},
                    {212, 392, 202, 448, 6, 14}, {262, 395, 250, 448, 6, 13},
                    {322, 404, 312, 450, 5, 12}}) do
  local m = wedge(f[1], f2(f[1]) + 2, f[3], f[4], f[5], f[6]):soften(7)
  midFolds[#midFolds + 1] = m * mMid
end
mFarShade = farFolds[1]
for i = 2, #farFolds do mFarShade = mFarShade + farFolds[i] end
mMidShade = midFolds[1]
for i = 2, #midFolds do mMidShade = mMidShade + midFolds[i] end

work(mFarShade, {hand="glaze", pile=shadeP, coverage=1.5, clip=true,
                 angle=-1.0, length={60, 130}, pressure={0.3, 0.5}})
work(mMidShade, {hand="glaze", pile=shadeP, coverage=1.5, clip=true,
                 angle=1.0, length={60, 130}, pressure={0.3, 0.5}})
print("gullies")

--@ chunk 32
-- those gullies came down like cigar stains: lay the range colour back over them
work(mFar, {hand="glaze", pile=farP, coverage=1.6, angle=0, clip=true,
            length={120, 240}, pressure={0.4, 0.6}})
work(mMid, {hand="glaze", pile=midP, coverage=1.6, angle=0, clip=true,
            length={120, 240}, pressure={0.4, 0.6}})
print("covered")

--@ chunk 33
print(drying(700, 380), drying(700, 360), drying(200, 420))
work(mFar, {hand="body", pile=farP, coverage=3, fill=true, clip=true,
            length={25, 60}, pressure={0.4, 0.7}})
work(mMid, {hand="body", pile=midP, coverage=3, fill=true, clip=true,
            length={25, 60}, pressure={0.4, 0.7}})
print("covered again")

--@ chunk 34
-- the crest catching the last light, strongest where the sun set
rimDepth = function(x)
  local t = clamp(1 - math.abs(x - 720) / 330, 0, 1)
  return 4 + 30 * t
end
mRim = below(f1) * above(function(x) return f1(x) - rimDepth(x) end) * mFar
rimP = pile{{"lead white", 7}, {"yellow ochre", 2.2}, {"vermilion", 0.6},
            {"raw umber", 0.5}, medium=0.2}
work(mRim, {hand="body", pile=rimP, coverage=1.4, clip=true, angle=-0.15,
            length={20, 55}, pressure={0.35, 0.6}})
print("rim light")

--@ chunk 35
-- the zenith stipple reads as grit at 1:1; a veil of its own colour to settle it in
veil = pile{{"pale smalt", 2.4}, {"lead white", 3.2}, {"raw umber", 1.1}, medium=0.4}
work(above(function() return 340 end), {hand="glaze", pile=veil, coverage=1.3,
        angle=-0.05, length={150, 260}, pressure={0.3, 0.45}})
print("veil over the zenith")

--@ chunk 36
-- Long glazes banded the sky.  Stipple instead: no bands, and the contrast of the
-- early blue grit is swamped by a second film of its own colour.
stipple(above(function() return 310 end), {pile=veil, width=2.4,
        coverage=function(x, y) return 1.25 * clamp((320 - y) / 220, 0, 1) + 0.15 end,
        pressure={0.25, 0.5}, cluster=0.3, feather=0.5, dips={26, 0.5, 0.5}})
stipple(above(function() return 150 end), {pile=skyDeep, width=2.2,
        coverage=function(x, y) return 1.0 * clamp((170 - y) / 150, 0, 1) end,
        pressure={0.25, 0.5}, cluster=0.3, feather=0.5, dips={26, 0.5, 0.5}})
stipple(below(function() return 300 end) * sky, {pile=warmLow, width=2.2,
        coverage=1.0, pressure={0.25, 0.5}, cluster=0.3, feather=0.5,
        dips={26, 0.5, 0.5}})
print("sky stippled again")

--@ chunk 37
skyBase = pile{{"lead white", 5}, {"pale smalt", 1.8}, {"raw umber", 1}, medium=0.15}
work(sky, {hand="body", pile=skyBase, coverage=2.4, fill=true, clip=true,
           length={25, 65}, pressure={0.35, 0.6}, angle=-0.04})
print("base film")

--@ chunk 38
skyWarm = pile{{"lead white", 6}, {"yellow ochre", 2.6}, {"pale smalt", 0.8},
               {"raw umber", 0.8}, medium=0.2}
glowX = function(x)
  local d = math.abs(x - 700) / 420
  return 0.3 + 0.7 * clamp(1 - d * d, 0, 1)
end
mWarm = mask(function(x, y)
  local v = clamp((462 - y) / 235, 0, 1)
  return clamp(v * v * (3 - 2 * v) * glowX(x) * 1.15, 0, 1)
end) * sky
work(mWarm, {hand="body", pile=skyWarm, coverage=1.7, threshold=0.02, clip=true,
             angle=-0.04, length={30, 80}, pressure={0.3, 0.55}})
print("glow")

--@ chunk 39
silhouette = function(x) return math.min(f1(x), f2(x), f3(x)) - 1 end
skyM = above(silhouette)
skyBase = pile{{"lead white", 5}, {"pale smalt", 1.8}, {"raw umber", 1}, medium=0.15}
skyWarm = pile{{"lead white", 6}, {"yellow ochre", 2.6}, {"pale smalt", 0.8},
               {"raw umber", 0.8}, medium=0.2}
skyCool = pile{{"lead white", 3.4}, {"pale smalt", 2}, {"smalt", 1.4},
               {"raw umber", 1.4}, medium=0.18}
print("sky area", skyM:area())

work(skyM, {hand="body", pile=skyBase, coverage=2.4, fill=true, clip=true,
            length={25, 65}, pressure={0.35, 0.6}, angle=-0.04})

mWarm = mask(function(x, y)
  local v = clamp((y - 205) / 260, 0, 1)
  return clamp(v * v * (3 - 2 * v) * glowX(x), 0, 1)
end) * skyM
work(mWarm, {hand="body", pile=skyWarm, coverage=1.5, threshold=0.02, clip=true,
             angle=-0.04, length={30, 80}, pressure={0.3, 0.55}})

mCool = mask(function(x, y)
  local v = clamp((255 - y) / 255, 0, 1)
  return clamp(v * v * (3 - 2 * v), 0, 1)
end) * skyM
work(mCool, {hand="body", pile=skyCool, coverage=1.6, threshold=0.02, clip=true,
             angle=-0.04, length={30, 80}, pressure={0.3, 0.55}})
print("sky, three passes")

--@ chunk 40
-- the ranges again, then a deeper zenith and a real glow above the crest
bankP = pile{{"raw umber", 3}, {"bone black", 1.6}, {"green earth", 1.2},
             {"lead white", 1}, medium=0.15}
glowP = pile{{"lead white", 5}, {"yellow ochre", 3.4}, {"vermilion", 1},
             {"raw umber", 0.5}, medium=0.2}
deepP = pile{{"pale smalt", 1.8}, {"smalt", 2.6}, {"lead white", 2.4},
             {"raw umber", 1.2}, medium=0.15}

work(mFar, {hand="body", pile=farP, coverage=2, clip=true, angle=0,
            length={25, 65}, pressure={0.35, 0.6}})
work(mMid, {hand="body", pile=midP, coverage=2, clip=true, angle=0,
            length={25, 65}, pressure={0.35, 0.6}})
work(mBank, {hand="body", pile=bankP, coverage=1.8, clip=true, angle=0.1,
             length={20, 50}, pressure={0.35, 0.6}})

mGlow = mask(function(x, y)
  local d = clamp((y - (silhouette(x) - 130)) / 130, 0, 1)
  return clamp((1 - d) * (1 - d) * glowX(x) * 1.3, 0, 1)
end) * skyM
work(mGlow, {hand="body", pile=glowP, coverage=1.6, threshold=0.02, clip=true,
             angle=-0.04, length={30, 80}, pressure={0.3, 0.55}})

mZen = mask(function(x, y)
  local v = clamp((300 - y) / 300, 0, 1)
  return clamp(v * v * (3 - 2 * v), 0, 1)
end) * skyM
work(mZen, {hand="broad", pile=deepP, coverage=1.9, threshold=0.02,
            length={80, 220}, pressure={0.3, 0.45}})
print("ranges, glow, zenith")

--@ chunk 41
-- The ground is warm; any film that does not cover it lets orange grit through the gaps.
-- Long light glaze strokes level into a continuous film.
work(skyM, {hand="glaze", pile=skyBase, coverage=2.6, fill=true, clip=true,
            angle=-0.04, length={140, 280}, pressure={0.4, 0.6}})
work(mWarm, {hand="glaze", pile=skyWarm, coverage=1.9, fill=true, threshold=0.02,
             clip=true, angle=-0.04, length={130, 260}, pressure={0.35, 0.55}})
work(mZen, {hand="glaze", pile=deepP, coverage=2, fill=true, threshold=0.02,
            clip=true, angle=-0.04, length={130, 260}, pressure={0.35, 0.55}})
work(mGlow, {hand="glaze", pile=glowP, coverage=1.7, fill=true, threshold=0.02,
             clip=true, angle=-0.04, length={120, 240}, pressure={0.35, 0.55}})

work(mFar, {hand="body", pile=farP, coverage=2.6, fill=true, clip=true, angle=0,
            length={25, 65}, pressure={0.35, 0.6}})
work(mMid, {hand="body", pile=midP, coverage=2.6, fill=true, clip=true, angle=0,
            length={25, 65}, pressure={0.35, 0.6}})
work(mBank, {hand="body", pile=bankP, coverage=2.2, fill=true, clip=true, angle=0.1,
             length={20, 50}, pressure={0.35, 0.6}})
print("laid solid")

--@ chunk 42
skyClean = pile{{"lead white", 4}, {"pale smalt", 3}, {"smalt", 2.2},
                {"raw umber", 0.7}, medium=0.1}
work(skyM, {hand="body", pile=skyClean, coverage=3.2, fill=true, clip=true,
            angle=-0.04, length={20, 55}, pressure={0.4, 0.7}})
print("clean film")

--@ chunk 43
-- three candidates for a cool sky film, laid straight from the tubes, no medium
A = pile{{"lead white", 5}, {"pale smalt", 2.5}, {"smalt", 1}}
B = pile{{"pale smalt", 3}, {"smalt", 4}, {"raw umber", 0.5}}
C = pile{{"lead white", 4}, {"pale smalt", 3}, {"smalt", 2.2}}
for i, p in ipairs({A, B, C}) do
  work(rect(620 + (i - 1) * 80, 60, 70, 130), {hand="body", pile=p, coverage=3,
            fill=true, clip=true, length={20, 50}, pressure={0.4, 0.7}})
end
print("swatches")

--@ chunk 44
-- Straight from the tubes, with no medium, these read as colour; thinned they only
-- warm the ground.  The sky, begun over: pale cool blue, deeper towards the zenith.
skyPale = pile{{"lead white", 4}, {"pale smalt", 3}, {"smalt", 2.2}}
skyDeep2 = pile{{"pale smalt", 2.4}, {"smalt", 3.2}, {"raw umber", 0.5}}

work(skyM, {hand="body", pile=skyPale, coverage=3, fill=true, clip=true,
            angle=-0.04, length={20, 55}, pressure={0.4, 0.7}})
mZen2 = mask(function(x, y)
  local v = clamp((310 - y) / 310, 0, 1)
  return clamp(v * v * (3 - 2 * v), 0, 1)
end) * skyM
work(mZen2, {hand="body", pile=skyDeep2, coverage=2.2, threshold=0.02, clip=true,
             angle=-0.04, length={25, 60}, pressure={0.4, 0.7}})
print("sky laid cool")

--@ chunk 45
-- the glow, laid from the tubes: a wide warm band low down, a brighter one at the crest
glowBody = pile{{"lead white", 5}, {"yellow ochre", 3}, {"vermilion", 0.8},
                {"raw umber", 0.4}}
glowHot = pile{{"lead white", 4.5}, {"yellow ochre", 3.2}, {"vermilion", 1.4},
               {"raw umber", 0.3}}
work(mWarm, {hand="body", pile=glowBody, coverage=1.8, threshold=0.02, clip=true,
             angle=-0.04, length={30, 80}, pressure={0.35, 0.6}})
work(mGlow, {hand="body", pile=glowHot, coverage=1.5, threshold=0.02, clip=true,
             angle=-0.04, length={30, 80}, pressure={0.35, 0.6}})
print("glow laid")

--@ chunk 46
-- my glow mask was strongest at the top of its band, not at the crest: inverted
mGlow2 = mask(function(x, y)
  local v = clamp((y - (silhouette(x) - 135)) / 135, 0, 1)
  return clamp(v * v * (3 - 2 * v) * glowX(x) * 1.25, 0, 1)
end) * skyM
glowHot2 = pile{{"lead white", 3}, {"yellow ochre", 3.2}, {"vermilion", 1.5},
                {"raw umber", 0.3}}
glowBody2 = pile{{"lead white", 4}, {"yellow ochre", 2.6}, {"vermilion", 0.8},
                 {"raw umber", 0.5}}
work(mGlow2, {hand="body", pile=glowHot2, coverage=2, threshold=0.02, clip=true,
              angle=-0.04, length={30, 80}, pressure={0.35, 0.6}})
work(mWarm, {hand="body", pile=glowBody2, coverage=1.6, threshold=0.02, clip=true,
             angle=-0.04, length={30, 80}, pressure={0.35, 0.6}})
print("glow, the right way round")

--@ chunk 47
sh = {{-10,705},{65,695},{145,687},{235,673},{325,661},{402,653},{472,650},
     {542,653},{612,659},{692,671},{782,683},{872,693},{1010,703}}
shoreFn = ridgefn(sh)
mWater = below(function() return 444 end) * above(shoreFn)

waterPale = pile{{"lead white", 3}, {"pale smalt", 2.4}, {"raw umber", 0.5}}
waterDeep = pile{{"pale smalt", 1.8}, {"smalt", 2.6}, {"raw umber", 1.2}}

work(mWater, {hand="body", pile=waterPale, coverage=2.6, fill=true, clip=true,
              angle=0, length={30, 90}, pressure={0.4, 0.7}})
mDeep = mask(function(x, y)
  local v = clamp((y - 500) / 210, 0, 1)
  return clamp(v * v * (3 - 2 * v), 0, 1)
end) * mWater
work(mDeep, {hand="body", pile=waterDeep, coverage=2, threshold=0.02, clip=true,
             angle=0, length={30, 90}, pressure={0.4, 0.7}})
print("water")

--@ chunk 48
-- The mist on the water: a band that eats the feet of the ranges and lies on the lake.
mistMask = mask(function(x, y)
  local top = 408 + 16 * math.sin(x / 250)
  local bot = 500 + 30 * math.sin(x / 185 + 1.2)
  local v = clamp((y - top) / 30, 0, 1) * clamp((bot - y) / 65, 0, 1)
  local h = 0.5 + 0.5 * clamp(1 - math.abs(x - 520) / 640, 0, 1)
  return clamp(v * h * 1.4, 0, 1)
end)
mistP = pile{{"lead white", 7}, {"pale smalt", 1.2}, {"raw umber", 0.4}}
work(mistMask, {hand="glaze", pile=mistP, coverage=2.4, fill=true, threshold=0.02,
                clip=true, angle=-0.02, length={90, 220}, pressure={0.3, 0.5}})
print("mist")

--@ chunk 49
-- the mist wants body, not a glaze: it is the brightest thing on the lake
mistP = pile{{"lead white", 6}, {"pale smalt", 1.5}}
work(mistMask, {hand="body", pile=mistP, coverage=2.8, fill=true, threshold=0.02,
                clip=true, angle=-0.02, length={25, 70}, pressure={0.4, 0.7}})
print("mist, stronger")

--@ chunk 50
-- the ranges' feet were carrying a dark bar of paint gathered at the mask's edge;
-- the fog should eat it and dissolve their bases
mFeet = mask(function(x, y)
  local v = clamp((y - (silhouette(x) - 6)) / 26, 0, 1)
  local w = 1 - 0.55 * clamp((y - 468) / 42, 0, 1)
  return clamp(v * w * 1.5, 0, 1)
end)
work(mFeet, {hand="glaze", pile=mistP, coverage=2.2, fill=true, threshold=0.02,
             clip=true, angle=-0.02, length={90, 220}, pressure={0.35, 0.55}})
print("fog on the feet")

--@ chunk 51
-- the fog ate the hills.  Ranges back, then a veil only over their feet.
work(mFar, {hand="body", pile=farP, coverage=2.4, fill=true, clip=true, angle=0,
            length={25, 65}, pressure={0.4, 0.7}})
work(mMid, {hand="body", pile=midP, coverage=2.4, fill=true, clip=true, angle=0,
            length={25, 65}, pressure={0.4, 0.7}})
work(mBank, {hand="body", pile=bankP, coverage=2, fill=true, clip=true, angle=0.1,
             length={20, 50}, pressure={0.4, 0.7}})
mFeet2 = mask(function(x, y)
  local v = clamp((y - (silhouette(x) - 4)) / 20, 0, 1)
  local w = 1 - 0.6 * clamp((y - 462) / 40, 0, 1)
  return clamp(v * w * 0.72, 0, 1)
end)
work(mFeet2, {hand="glaze", pile=mistP, coverage=2, fill=true, threshold=0.02,
              clip=true, angle=-0.02, length={90, 220}, pressure={0.35, 0.55}})
print("ranges back, feet veiled")

--@ chunk 52
-- eat the dark bar at the far shore, then the lake's first reflections
mKill = mask(function(x, y)
  local t = clamp((y - 434) / 14, 0, 1) * clamp((480 - y) / 24, 0, 1)
  local h = 0.4 + 0.6 * clamp(1 - math.abs(x - 560) / 720, 0, 1)
  return clamp(t * h * 1.5, 0, 1)
end)
work(mKill, {hand="glaze", pile=mistP, coverage=1.7, fill=true, threshold=0.02,
             clip=true, angle=0, length={70, 180}, pressure={0.35, 0.55}})

reflP = pile{{"pale smalt", 1.6}, {"smalt", 1.8}, {"raw umber", 1.2}}
mRefl = mask(function(x, y)
  local v = clamp((y - 470) / 18, 0, 1) * clamp((558 - y) / 62, 0, 1)
  local h = 0.15 + 0.85 * clamp((400 - x) / 430, 0, 1)
  return clamp(v * h * 1.5, 0, 1)
end)
mRefl2 = mask(function(x, y)
  local v = clamp((y - 470) / 18, 0, 1) * clamp((540 - y) / 55, 0, 1)
  local h = 0.1 + 0.9 * clamp(1 - math.abs(x - 700) / 330, 0, 1)
  return clamp(v * h * 0.9, 0, 1)
end)
work(mRefl, {hand="glaze", pile=reflP, coverage=1.7, fill=true, threshold=0.02,
             clip=true, angle=0, length={70, 190}, pressure={0.35, 0.55}})
work(mRefl2, {hand="glaze", pile=reflP, coverage=1.4, fill=true, threshold=0.02,
              clip=true, angle=0, length={70, 190}, pressure={0.35, 0.55}})
print("far shore eaten, reflections in")

--@ chunk 53
-- the near bank: the darkest thing in the picture, and the frame for everything else
mShore = below(shoreFn)
earthDark = pile{{"raw umber", 4}, {"bone black", 2}, {"green earth", 1.5},
                 {"yellow ochre", 1}}
work(mShore, {hand="body", pile=earthDark, coverage=2.6, fill=true, clip=true,
              angle=-0.2, length={30, 80}, pressure={0.45, 0.75}})

-- the wet mud at the edge, darker and cooler, and the grass catching the last light
mMud = mask(function(x, y)
  local d = y - shoreFn(x)
  return clamp((d - 2) / 10, 0, 1) * clamp((40 - d) / 22, 0, 1) * 1.5
end) * mShore
mudP = pile{{"raw umber", 3}, {"bone black", 2.4}, {"pale smalt", 1}}
work(mMud, {hand="body", pile=mudP, coverage=2, threshold=0.02, clip=true,
            angle=0, length={25, 70}, pressure={0.45, 0.75}})

grassP = pile{{"yellow ochre", 2}, {"green earth", 2.5}, {"raw umber", 1.2}}
mGrass = mask(function(x, y)
  local d = y - shoreFn(x)
  return clamp((d - 14) / 26, 0, 1) * clamp((70 - d) / 40, 0, 1) * 1.3
end) * mShore
work(mGrass, {hand="body", pile=grassP, coverage=1.6, threshold=0.02, clip=true,
              angle=-0.35, length={20, 55}, pressure={0.35, 0.6}})
print("bank")

--@ chunk 54
erase(rect(-5, -5, 430, 790), {strength=0.95})
erase(rect(-5, -5, 430, 790), {strength=0.9})

-- The two near trees, built once and kept: trunks as polylines with a width at each end,
-- limbs leaving near the tip of the axis and turning up, secondaries in turn.
branches = {}
addLimb = function(x, y, ang, tipang, len, w0, w1, order)
  local n, pts, a, px, py = 4, {{x, y}}, ang, x, y
  for i = 1, n do
    a = a + (tipang - a) * 0.32 + randn(0, 0.02)
    px, py = px + math.cos(a) * len / n, py + math.sin(a) * len / n
    pts[#pts + 1] = {px, py}
  end
  branches[#branches + 1] = {pts = pts, w0 = w0, w1 = w1, order = order}
  if order >= 2 then return end
  for i = 1, 2 do
    if rand() < 0.85 then
      local s = (i == 1) and 1 or -1
      local bx, by = pts[1 + i][1], pts[1 + i][2]
      local ba = a + s * rand(0.55, 1.0)
      addLimb(bx, by, ba, ba + s * rand(0.05, 0.4), len * rand(0.32, 0.5),
              w1 * 0.85, w1 * 0.42, order + 1)
    end
  end
  return a
end

addAxis = function(x, y, len, w0, w1, lean)
  local pts, a, px, py = {{x, y}}, -math.pi / 2 + lean, x, y
  for i = 1, 7 do
    a = a + randn(0, 0.014)
    px, py = px + math.cos(a) * len / 7, py + math.sin(a) * len / 7
    pts[#pts + 1] = {px, py}
  end
  branches[#branches + 1] = {pts = pts, w0 = w0, w1 = w1, order = 0, axis = true}
  return pts
end

trA = addAxis(96, 880, 880, 30, 11, 0.05)
for _, b in ipairs({{6, 1, 0.95, 175, 0.62}, {5, -1, 1.15, 200, 0.72}, {4, 1, 1.3, 190, 0.7},
                    {3, -1, 0.85, 160, 0.62}, {3, 1, 0.7, 130, 0.5}, {2, -1, 1.0, 120, 0.5}}) do
  local bx, by = trA[b[1]][1], trA[b[1]][2]
  local a0 = -math.pi / 2 + b[2] * b[3]
  addLimb(bx, by, a0, a0 - b[2] * 0.45, b[4], 14 * b[5] + 3, 3, 1)
end
trB = addAxis(252, 720, 830, 20, 8, 0.02)
for _, b in ipairs({{6, -1, 1.0, 150, 0.8}, {5, 1, 1.2, 165, 0.85}, {4, -1, 1.3, 140, 0.85},
                    {3, 1, 0.9, 120, 0.7}, {4, 1, 0.6, 95, 0.6}}) do
  local bx, by = trB[b[1]][1], trB[b[1]][2]
  local a0 = -math.pi / 2 + b[2] * b[3]
  addLimb(bx, by, a0, a0 - b[2] * 0.4, b[4], 11 * b[5], 2.5, 1)
end
print("branches", #branches)

--@ chunk 55
-- the pencil for the same trees, over the erased field
gA, gB, gC = pencil("4B"), pencil("B"), pencil("H")
for _, b in ipairs(branches) do
  local g, pr = gC, 0.4
  if b.order == 0 then g, pr = gA, 0.8 elseif b.order == 1 then g, pr = gB, 0.6 end
  g:line(b.pts, {pressure=pr, smooth=true})
end
print("tree drawing")

--@ chunk 56
local b = branches[1]
local n, ws = #b.pts, {}
for i = 1, n do
  local t = (i - 1) / (n - 1)
  ws[i] = b.w0 + (b.w1 - b.w0) * t
end
print(b.w0, b.w1, n)
m1 = ribbon(b.pts, ws)
m1b = ribbon(b.pts, b.w0)
print("tapered area", m1:area(), "flat area", m1b:area())

--@ chunk 57
-- the trees, painted from the geometry I kept
mA, mB = nil, nil
for k, b in ipairs(branches) do
  local n, ws = #b.pts, {}
  for i = 1, n do
    local t = (i - 1) / (n - 1)
    ws[i] = b.w0 + (b.w1 - b.w0) * t
  end
  local m = ribbon(b.pts, ws)
  if k <= 16 then mA = (mA == nil) and m or (mA + m) else mB = (mB == nil) and m or (mB + m) end
end
treeDark = pile{{"bone black", 3}, {"raw umber", 2.4}, {"pale smalt", 1.2}}
treeFar  = pile{{"bone black", 2}, {"raw umber", 2.2}, {"pale smalt", 1.9}}

work(mA, {hand="body", pile=treeDark, coverage=2.6, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.75}})
work(mB, {hand="body", pile=treeFar, coverage=2.4, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.7}})
print("trees painted", mA:area(), mB:area())

--@ chunk 58
-- Repainting the left third: same films, same colours, so the new trees can go over it.
L = mask(function(x, y) return clamp((432 - x) / 22, 0, 1) end)
work(skyM * L, {hand="body", pile=skyPale, coverage=3, fill=true, clip=true,
                angle=-0.04, length={20, 55}, pressure={0.4, 0.7}})
work(mZen2 * L, {hand="body", pile=skyDeep2, coverage=2.2, threshold=0.02,
                 clip=true, angle=-0.04, length={25, 60}, pressure={0.4, 0.7}})
work(mWarm * L, {hand="body", pile=glowBody2, coverage=1.6, threshold=0.02,
                clip=true, angle=-0.04, length={30, 80}, pressure={0.35, 0.6}})
work(mGlow2 * L, {hand="body", pile=glowHot2, coverage=2, threshold=0.02,
                  clip=true, angle=-0.04, length={30, 80}, pressure={0.35, 0.6}})
work(mFar * L, {hand="body", pile=farP, coverage=2.4, fill=true, clip=true,
                angle=0, length={25, 65}, pressure={0.4, 0.7}})
work(mMid * L, {hand="body", pile=midP, coverage=2.4, fill=true, clip=true,
                angle=0, length={25, 65}, pressure={0.4, 0.7}})
print("left third: sky and ranges back")

--@ chunk 59
work(mWater * L, {hand="body", pile=waterPale, coverage=2.6, fill=true, clip=true,
                   angle=0, length={30, 90}, pressure={0.4, 0.7}})
work(mDeep * L, {hand="body", pile=waterDeep, coverage=2, threshold=0.02, clip=true,
                 angle=0, length={30, 90}, pressure={0.4, 0.7}})
work(mistMask * L, {hand="body", pile=mistP, coverage=2.8, fill=true, threshold=0.02,
                    clip=true, angle=-0.02, length={25, 70}, pressure={0.4, 0.7}})
work(mFeet2 * L, {hand="glaze", pile=mistP, coverage=2, fill=true, threshold=0.02,
                  clip=true, angle=-0.02, length={90, 220}, pressure={0.35, 0.55}})
work(mKill * L, {hand="glaze", pile=mistP, coverage=1.7, fill=true, threshold=0.02,
                 clip=true, angle=0, length={70, 180}, pressure={0.35, 0.55}})
work(mRefl * L, {hand="glaze", pile=reflP, coverage=1.7, fill=true, threshold=0.02,
                 clip=true, angle=0, length={70, 190}, pressure={0.35, 0.55}})
work(mShore * L, {hand="body", pile=earthDark, coverage=2.4, fill=true, clip=true,
                  angle=-0.2, length={30, 80}, pressure={0.45, 0.75}})
work(mMud * L, {hand="body", pile=mudP, coverage=1.8, threshold=0.02, clip=true,
                angle=0, length={25, 70}, pressure={0.45, 0.75}})
work(mGrass * L, {hand="body", pile=grassP, coverage=1.6, threshold=0.02, clip=true,
                  angle=-0.35, length={20, 55}, pressure={0.35, 0.6}})
print("left third: water, mist, bank back")

--@ chunk 60
-- The crowns.  Each main limb is long and sweeps out before turning up; it forks at
-- two points along its length, and its children fork again.  A few dead stubs on top.
crown = {}
limb2 = function(x, y, ang, tipang, len, w0, w1, depth)
  local n, pts, a, px, py = 4, {{x, y}}, ang, x, y
  for i = 1, n do
    a = a + (tipang - a) * 0.35 + randn(0, 0.018)
    px, py = px + math.cos(a) * len / n, py + math.sin(a) * len / n
    pts[#pts + 1] = {px, py}
  end
  crown[#crown + 1] = {pts = pts, w0 = w0, w1 = w1, d = depth}
  if depth <= 0 then return end
  for i = 1, 2 do
    if rand() < 0.9 then
      local s = (i == 1) and 1 or -1
      local bx, by = pts[2 + i][1], pts[2 + i][2]
      local ba = a + s * rand(0.45, 0.9)
      limb2(bx, by, ba, ba - s * rand(0.05, 0.35), len * rand(0.45, 0.68),
            w1 * 0.8, w1 * 0.42, depth - 1)
    end
  end
end

-- tree A: the big limb to the right reaches into the open sky
for _, b in ipairs({{3, 1, 1.18, 390, 13}, {4, -1, 1.0, 310, 11},
                    {5, 1, 0.95, 300, 10}, {6, -1, 1.05, 265, 9},
                    {7, 1, 1.05, 245, 8}}) do
  local bx, by = trA[b[1]][1], trA[b[1]][2]
  local a0 = -math.pi / 2 + b[2] * b[3]
  limb2(bx, by, a0, a0 - b[2] * 0.5, b[4], b[5], 2.6, 3)
end
for i = 1, 3 do            -- dead stubs standing above the crown
  local j = 6 + (i % 2)
  local bx, by = trA[j][1], trA[j][2]
  limb2(bx, by, -math.pi / 2 + (i % 2 == 0 and 0.5 or -0.55), -math.pi / 2,
        rand(50, 95), rand(4, 6), 1.8, 0)
end
nA = #crown
print("tree A crown", nA)

--@ chunk 61
-- tree B: smaller, further off, its limbs shorter
for _, b in ipairs({{5, -1, 1.1, 215, 8}, {6, 1, 1.0, 230, 8.5},
                    {4, 1, 0.85, 175, 6.5}, {3, -1, 0.9, 165, 6}}) do
  local bx, by = trB[b[1]][1], trB[b[1]][2]
  local a0 = -math.pi / 2 + b[2] * b[3]
  limb2(bx, by, a0, a0 - b[2] * 0.45, b[4], b[5], 2, 3)
end
for i = 1, 2 do
  local bx, by = trB[5 + (i % 2)][1], trB[5 + (i % 2)][2]
  limb2(bx, by, -math.pi / 2 + (i % 2 == 0 and 0.4 or -0.45), -math.pi / 2,
        rand(40, 70), 3.4, 1.4, 0)
end
nB = #crown
print("tree B crown", nB - nA)

--@ chunk 62
-- masks for both trees: the old sparse limbs and the new crown together, so the
-- pale ghosts of the first attempt get covered by the same dark pass
maskOf = function(list, first, last)
  local m
  for k = first, last do
    local b = list[k]
    local n, ws = #b.pts, {}
    for i = 1, n do
      local t = (i - 1) / (n - 1)
      ws[i] = b.w0 + (b.w1 - b.w0) * t
    end
    local r = ribbon(b.pts, ws)
    m = (m == nil) and r or (m + r)
  end
  return m
end
mA = maskOf(branches, 1, 16) + maskOf(crown, 1, nA)
mB = maskOf(branches, 17, 32) + maskOf(crown, nA + 1, nB)
mAt = maskOf(branches, 1, 1) + maskOf(crown, 1, 1)
mBt = maskOf(branches, 17, 17) + maskOf(crown, nA + 1, nA + 1)
work(mAt, {hand="body", pile=treeDark, coverage=2.4, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.5, 0.85}})
work(mA, {hand="body", pile=treeDark, coverage=2.6, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.75}})
work(mBt, {hand="body", pile=treeFar, coverage=2.4, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.45, 0.8}})
work(mB, {hand="body", pile=treeFar, coverage=2.5, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.7}})
print("trees repainted", mA:area(), mB:area())

--@ chunk 63
-- A woman at the water's edge, seen from behind, a bundle of faggots on her shoulder.
-- Everything about her is 55 units tall; she stands on the bank at y 655.
head = ellipse(561.5, 603, 4.6, 5.4)
shawl = poly({{551,609},{556,605.5},{567,605.5},{572,611},{573,622},{570,634},
              {571.5,639.5},{552,640},{548.5,634},{549.5,621}})
skirt = poly({{551,638},{571.5,638},{573.5,648},{570,655.5},{556,656},{549.5,652}})
arm = poly({{551.5,612},{548,624},{551,628},{555,624.5},{556.5,614}})
mFig = head + shawl + skirt + arm
for i = 1, 9 do
  local a = -2.45 + (i - 1) * 0.085 + randn(0, 0.02)
  local L = rand(17, 27)
  mFig = mFig + ribbon({{567, 607}, {567 + math.cos(a) * L, 607 + math.sin(a) * L}}, 1.3)
end
figP = pile{{"bone black", 3.5}, {"raw umber", 1.7}, {"pale smalt", 0.8}}
work(mFig, {hand="body", pile=figP, coverage=2.6, fill=true, clip=true,
            tool="filbert 3", length={6, 18}, pressure={0.5, 0.85}})
print("figure", mFig:area())

--@ chunk 64
-- the lake: pale where the fog lies on it, deepening towards the near shore
mWDeep = mask(function(x, y)
  local v = clamp((y - 478) / 210, 0, 1)
  return clamp(v * v * (3 - 2 * v) * 1.35, 0, 1)
end) * mWater
waterDarkP = pile{{"pale smalt", 1.5}, {"smalt", 1.9}, {"raw umber", 2},
                  {"bone black", 0.6}}
work(mWDeep, {hand="glaze", pile=waterDarkP, coverage=2.4, fill=true,
              threshold=0.02, clip=true, angle=0, length={90, 230},
              pressure={0.4, 0.6}})
print("water deepened")

--@ chunk 65
print(mWDeep:at(200, 560), mWDeep:at(600, 560), mWDeep:at(600, 620), mWDeep:at(600, 500))
print("area", mWDeep:area())
print("water", mWater:at(200, 560), mWater:at(600, 560))

--@ chunk 66
-- that was a flat slab; the lake wants a mid tone first, then a gentler gradient
waterMidP = pile{{"pale smalt", 2.2}, {"smalt", 1.6}, {"raw umber", 1.2}}
work(mWater, {hand="glaze", pile=waterMidP, coverage=2.1, fill=true, clip=true,
              angle=0, length={90, 230}, pressure={0.4, 0.6}})
mNear = mask(function(x, y)
  local v = clamp((y - 556) / 145, 0, 1)
  return clamp(v * v * (3 - 2 * v) * 1.1, 0, 1)
end) * mWater
work(mNear, {hand="glaze", pile=waterDarkP, coverage=1.7, fill=true, threshold=0.02,
             clip=true, angle=0, length={90, 230}, pressure={0.4, 0.6}})
print("water, mid tone and a gentler fall")

--@ chunk 67
-- the lake had been painted over the trees; lay the trees back
work(mAt, {hand="body", pile=treeDark, coverage=2.4, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.5, 0.85}})
work(mA, {hand="body", pile=treeDark, coverage=2.5, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.75}})
work(mBt, {hand="body", pile=treeFar, coverage=2.4, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.45, 0.8}})
work(mB, {hand="body", pile=treeFar, coverage=2.3, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.7}})

-- and even the left third of the lake with the same two passes
work(mWater * L, {hand="glaze", pile=waterMidP, coverage=2.2, fill=true, clip=true,
                  angle=0, length={90, 230}, pressure={0.45, 0.65}})
work(mNear * L, {hand="glaze", pile=waterDarkP, coverage=1.8, fill=true,
                 threshold=0.02, clip=true, angle=0, length={90, 230},
                 pressure={0.45, 0.65}})

-- the horizon line is too hard: fog across it
mHorizon = mask(function(x, y)
  return clamp((y - 414) / 16, 0, 1) * clamp((486 - y) / 22, 0, 1) * 0.85
end)
work(mHorizon, {hand="glaze", pile=mistP, coverage=1.9, fill=true, threshold=0.02,
                clip=true, angle=0, length={90, 230}, pressure={0.35, 0.55}})
print("trees back, lake evened, horizon softened")

--@ chunk 68
-- one tone over the whole lake to even it, then a light glaze to level the blotching
waterEven = pile{{"pale smalt", 1.6}, {"smalt", 2}, {"raw umber", 1.8}}
work(mWater, {hand="glaze", pile=waterEven, coverage=1.9, fill=true, clip=true,
              angle=0, length={100, 240}, pressure={0.45, 0.65}})
work(mWater, {hand="glaze", pile=waterEven, coverage=1.2, clip=true, angle=0,
              length={150, 280}, pressure={0.3, 0.45}})
print("lake evened")

--@ chunk 69
-- the trees and the figure again, this time over paint that has begun to set,
-- so their edges stay crisp
work(mAt, {hand="body", pile=treeDark, coverage=2.4, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.5, 0.85}})
work(mA, {hand="body", pile=treeDark, coverage=2.4, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.75}})
work(mBt, {hand="body", pile=treeFar, coverage=2.4, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.45, 0.8}})
work(mB, {hand="body", pile=treeFar, coverage=2.3, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.7}})
work(mFig, {hand="body", pile=figP, coverage=2.6, fill=true, clip=true,
            tool="filbert 3", length={6, 18}, pressure={0.5, 0.85}})
print("trees and figure")

--@ chunk 70
-- the lake needs light on it: long thin streaks, the glow broken by ripples
streakWarm = pile{{"lead white", 4}, {"yellow ochre", 1.6}, {"pale smalt", 1}}
streakCool = pile{{"lead white", 4.5}, {"pale smalt", 2}}
mStreakW = mask(function(x, y)
  local v = clamp((672 - y) / 230, 0, 1)
  local h = clamp(1 - math.abs(x - 660) / 420, 0, 1)
  return clamp(v * (0.35 + 0.65 * h) * 1.2, 0, 1)
end) * mWater
mStreakC = mask(function(x, y)
  local v = clamp((620 - y) / 200, 0, 1)
  return clamp(v * 0.8, 0, 1)
end) * mWater
work(mStreakW, {hand="glaze", pile=streakWarm, coverage=0.55, threshold=0.03,
                clip=true, angle=0, length={120, 300}, pressure={0.25, 0.4}})
work(mStreakC, {hand="glaze", pile=streakCool, coverage=0.4, threshold=0.03,
                clip=true, angle=0, length={150, 300}, pressure={0.22, 0.35}})

-- the woman: a thin light on her edge from the sky behind, and her reflection below
rim = mFig:offset(1.7) - mFig
rimP = pile{{"lead white", 3}, {"yellow ochre", 1}, {"pale smalt", 1.5}}
work(rim, {hand="body", pile=rimP, coverage=0.9, clip=true, tool="filbert 3",
           length={5, 14}, pressure={0.3, 0.55}})
refl = poly({{552, 654}, {573, 654}, {569, 704}, {556, 704}}):soften(4)
work(refl, {hand="glaze", pile=treeDark, coverage=0.8, clip=true, angle=0,
            length={30, 80}, pressure={0.3, 0.5}})
print("streaks, rim, reflection")

--@ chunk 71
-- the streaks merged into patches: knock them back, then lay finer ones
work(mWater, {hand="glaze", pile=waterEven, coverage=0.75, clip=true, angle=0,
              length={110, 260}, pressure={0.35, 0.5}})
work(mStreakW, {hand="body", pile=streakWarm, coverage=0.5, threshold=0.05,
                clip=true, angle=0, tool="filbert 3", length={40, 130},
                pressure={0.25, 0.45}})
work(mStreakC, {hand="body", pile=streakCool, coverage=0.45, threshold=0.05,
                clip=true, angle=0, tool="filbert 3", length={40, 140},
                pressure={0.25, 0.42}})

-- her edge had gone to a bright outline; bring it back to a hint
rimMid = pile{{"bone black", 1.6}, {"raw umber", 1.6}, {"pale smalt", 1}}
work(rim, {hand="body", pile=rimMid, coverage=0.85, clip=true, tool="filbert 3",
           length={5, 14}, pressure={0.3, 0.5}})
print("streaks finer, rim toned")

--@ chunk 72
-- the near bank: deeper in front, the wet line at the edge, grass overhanging into the water
nzEdge = noise{seed=5, period=42, octaves=3}
mFringe = mask(function(x, y)
  local d = y - shoreFn(x)
  local e = 5 + 17 * nzEdge:at01(x, 100)
  return clamp((d + 3) / 5, 0, 1) * clamp((e - d) / 7, 0, 1) * 1.4
end) * mShore
mFore = mask(function(x, y)
  local v = clamp((y - 690) / 110, 0, 1)
  return clamp(v * v * (3 - 2 * v) * 1.3, 0, 1)
end) * mShore

earthDeep = pile{{"raw umber", 4}, {"bone black", 2.4}, {"green earth", 1.2}}
work(mFore, {hand="glaze", pile=earthDeep, coverage=1.5, fill=true, threshold=0.02,
             clip=true, angle=-0.2, length={60, 150}, pressure={0.45, 0.65}})
work(mFringe, {hand="body", pile=grassP, coverage=1.7, threshold=0.02, clip=true,
               angle=-1.15, tool="filbert 4", length={12, 34}, pressure={0.35, 0.65}})
grassDark = pile{{"green earth", 2}, {"raw umber", 2.4}, {"bone black", 1.2}}
work(mFringe, {hand="body", pile=grassDark, coverage=1.3, threshold=0.02, clip=true,
               angle=-1.25, tool="rigger 1.6", length={10, 30}, pressure={0.4, 0.7}})

-- reeds standing at the edge, leaning downstream
reedP = pile{{"raw umber", 2}, {"yellow ochre", 2}, {"green earth", 1.6},
             {"bone black", 0.6}}
rr = brush{kind="round", width=1.8, point=0.9, stiffness=0.45}
for i = 1, 60 do
  local x = 340 + (i - 1) * 6.4 + randn(0, 4)
  local by = shoreFn(x) + 2 + rand(0, 6)
  local hgt = rand(9, 26)
  local lean = rand(-0.25, 0.25) + 0.12
  rr:load(reedP, rand(0.4, 0.85))
  rr:stroke({{x, by}, {x + lean * hgt * 0.5, by - hgt * 0.6}, {x + lean * hgt, by - hgt}},
            {pressure={0.7, 0.55, 0.12}, orient="across"})
end
print("bank modelled, reeds in")

--@ chunk 73
-- the edge had gone fluorescent; a muted green over it, and some dry grass left in
edgeP = pile{{"green earth", 2.5}, {"raw umber", 2.6}, {"bone black", 0.9}}
work(mFringe, {hand="body", pile=edgeP, coverage=1.7, threshold=0.02, clip=true,
               angle=-1.15, tool="filbert 4", length={12, 34}, pressure={0.35, 0.6}})
dryP = pile{{"yellow ochre", 2.4}, {"raw umber", 1.4}, {"lead white", 1}}
work(mFringe, {hand="body", pile=dryP, coverage=0.45, threshold=0.06, clip=true,
               angle=-1.2, tool="rigger 1.6", length={8, 26}, pressure={0.3, 0.5}})

-- a few stones along the wet line
stoneP = pile{{"raw umber", 2.6}, {"bone black", 1.2}, {"pale smalt", 1}}
for i = 1, 9 do
  local x = 380 + (i - 1) * 46 + randn(0, 14)
  local by = shoreFn(x) + rand(4, 16)
  work(ellipse(x, by, rand(2.5, 5), rand(1.6, 3)), {hand="body", pile=stoneP,
        coverage=1.6, fill=true, clip=true, tool="filbert 3", length={5, 14},
        pressure={0.4, 0.7}})
end
print("edge muted, stones")

--@ chunk 74
-- the bank, laid solid: dark olive earth, quiet
bankP = pile{{"raw umber", 4}, {"green earth", 2}, {"bone black", 1.6}}
work(mShore, {hand="body", pile=bankP, coverage=3, fill=true, clip=true,
              angle=-0.25, length={25, 60}, pressure={0.5, 0.8}})
-- darker as it comes forward, and a breath of light on the grass at the edge
work(mFore, {hand="glaze", pile=earthDeep, coverage=1.6, fill=true, threshold=0.02,
             clip=true, angle=-0.25, length={70, 170}, pressure={0.45, 0.65}})
mLit = mask(function(x, y)
  local d = y - shoreFn(x)
  return clamp((d - 10) / 16, 0, 1) * clamp((52 - d) / 26, 0, 1)
end) * mShore
litP = pile{{"yellow ochre", 1.6}, {"green earth", 1.6}, {"raw umber", 1.4},
            {"lead white", 1}}
work(mLit, {hand="body", pile=litP, coverage=0.8, threshold=0.04, clip=true,
            angle=-1.2, tool="filbert 4", length={10, 28}, pressure={0.3, 0.5}})
print("bank, solid")

--@ chunk 75
-- small bare trees along the right bank and the foot of the left massif, drawn as
-- fine dark marks: the same construction, far smaller
far = {}
function littleTree(x, y, hgt, seedang)
  local a = -math.pi / 2 + (seedang or 0) + randn(0, 0.1)
  local px, py = x, y
  far[#far + 1] = {pts = {{x, y}, {x + randn(0, 1.5), y - hgt * 0.55}, {x + randn(0, 3), y - hgt}},
                   w = 2.2}
  for i = 1, 3 do
    local t = 0.35 + i * 0.2
    local bx, by = x + randn(0, 2), y - hgt * t
    local s = (i % 2 == 0) and 1 or -1
    local L = hgt * rand(0.25, 0.45)
    far[#far + 1] = {pts = {{bx, by}, {bx + s * L * 0.8, by - L * 0.8}}, w = 1.3}
  end
end
for i = 1, 26 do
  local x = rand(788, 1010)
  littleTree(x, f3(x) + rand(2, 6), rand(14, 34))
end
for i = 1, 16 do
  local x = rand(-6, 150)
  littleTree(x, f2(x) + rand(2, 6), rand(12, 26))
end
mFarTrees = nil
for _, t in ipairs(far) do
  local r = ribbon(t.pts, t.w)
  mFarTrees = (mFarTrees == nil) and r or (mFarTrees + r)
end
farTreeP = pile{{"bone black", 2}, {"raw umber", 2}, {"pale smalt", 1.4}}
work(mFarTrees, {hand="body", pile=farTreeP, coverage=2, fill=true, clip=true,
                 tool="filbert 3", length={8, 22}, pressure={0.4, 0.7}})
print("far trees", mFarTrees:area())

--@ chunk 76
-- a boat drawn up on the bank, a little to her right: a dark hull, a pale gunwale
hull = poly({{597, 681}, {604, 673}, {618, 669.5}, {636, 669}, {652, 671.5},
             {659, 680}, {650, 684}, {628, 685}, {606, 684}})
inside = poly({{605, 679}, {612, 673}, {626, 671.5}, {642, 672}, {650, 678},
               {636, 681}, {618, 681}})
boatP = pile{{"raw umber", 2.6}, {"bone black", 2}, {"pale smalt", 0.8}}
work(hull, {hand="body", pile=boatP, coverage=2, fill=true, clip=true,
            tool="filbert 3", length={6, 18}, pressure={0.45, 0.75}})
gunwale = ribbon({{597, 681}, {604, 673}, {618, 669.5}, {636, 669}, {652, 671.5},
                  {659, 680}}, 1.1)
gunP = pile{{"yellow ochre", 2}, {"lead white", 1.6}, {"raw umber", 1}}
work(gunwale, {hand="body", pile=gunP, coverage=1.4, clip=true, tool="filbert 2.5",
               length={5, 14}, pressure={0.4, 0.7}})
-- an oar or a pole leaning out of her, and the boat's dark reflection in the wet mud
work(inside, {hand="body", pile=figP, coverage=1.4, fill=true, clip=true,
              tool="filbert 2.5", length={5, 12}, pressure={0.4, 0.7}})
pole = ribbon({{668, 682}, {681, 664}, {688, 655}}, 1.2)
work(pole, {hand="body", pile=boatP, coverage=1.6, clip=true, tool="filbert 2.5",
            length={6, 16}, pressure={0.45, 0.7}})

-- birds, high in the open sky
birdP = pile{{"raw umber", 2.4}, {"bone black", 1.6}}
br = brush{kind="round", width=1.5, point=1, stiffness=0.4}
for _, b in ipairs({{492, 104, 6.5}, {523, 88, 5}, {548, 112, 4}, {572, 96, 5.5},
                    {610, 122, 4.5}}) do
  local x, y, w = b[1], b[2], b[3]
  br:load(birdP, rand(0.5, 0.8))
  br:stroke({{x - w, y + w * 0.35}, {x - w * 0.3, y - w * 0.25}, {x, y}},
            {pressure={0.55, 0.5, 0.1}, orient="across"})
  br:stroke({{x, y}, {x + w * 0.35, y - w * 0.3}, {x + w, y + w * 0.4}},
            {pressure={0.1, 0.5, 0.55}, orient="across"})
end
print("boat, pole, birds")

--@ chunk 77
hazeP = pile{{"pale smalt", 3}, {"raw umber", 2}, {"bone black", 0.5},
            {"lead white", 1.4}}
work(mFarTrees, {hand="body", pile=hazeP, coverage=1.7, fill=true, clip=true,
                 tool="filbert 3", length={8, 22}, pressure={0.35, 0.6}})
-- and a breath of fog across the far shore, so their feet go into it
mShoreFog = mask(function(x, y)
  local v = clamp((y - 412) / 14, 0, 1) * clamp((466 - y) / 26, 0, 1)
  return clamp(v * 0.8, 0, 1)
end)
work(mShoreFog, {hand="glaze", pile=mistP, coverage=1.5, fill=true, threshold=0.03,
                 clip=true, angle=-0.02, length={90, 230}, pressure={0.3, 0.5}})
print(wait(480))
print(drying(300, 600), drying(300, 200), drying(700, 500))

--@ chunk 78
-- the ranges back, then only a narrow veil at their feet
work(mFar, {hand="body", pile=farP, coverage=2.4, fill=true, clip=true, angle=0,
            length={25, 65}, pressure={0.4, 0.7}})
work(mMid, {hand="body", pile=midP, coverage=2.4, fill=true, clip=true, angle=0,
            length={25, 65}, pressure={0.4, 0.7}})
mFeet3 = mask(function(x, y)
  local v = clamp((y - (silhouette(x) - 4)) / 16, 0, 1)
  local w = 1 - 0.7 * clamp((y - 444) / 26, 0, 1)
  return clamp(v * w * 0.85, 0, 1)
end)
work(mFeet3, {hand="glaze", pile=mistP, coverage=1.7, fill=true, threshold=0.03,
               clip=true, angle=-0.02, length={80, 200}, pressure={0.3, 0.5}})

-- modelling on the far range: its left flank in shadow, its crest and right in light
mLitPeak = mask(function(x, y)
  local v = clamp((y - f1(x) + 8) / 26, 0, 1)
  local h = clamp(1 - math.abs(x - 690) / 380, 0, 1)
  return clamp(v * (0.2 + 0.8 * h), 0, 1)
end) * mFar
peakLightP = pile{{"lead white", 3.4}, {"yellow ochre", 1.2}, {"pale smalt", 1.4}}
work(mLitPeak, {hand="glaze", pile=peakLightP, coverage=1.3, threshold=0.04,
                clip=true, angle=-0.25, length={50, 130}, pressure={0.3, 0.5}})
mShadowSide = mask(function(x, y)
  local v = clamp((y - f1(x) + 8) / 30, 0, 1)
  local h = clamp((x - 430) / 160, 0, 1)
  return clamp(v * h * 1.1, 0, 1)
end) * mFar
work(mShadowSide, {hand="glaze", pile=reflP, coverage=1.2, threshold=0.04,
                   clip=true, angle=-1.15, length={50, 130}, pressure={0.3, 0.5}})
print("ranges modelled")

--@ chunk 79
work(mFar, {hand="body", pile=farP, coverage=2.6, fill=true, clip=true, angle=0,
            length={25, 65}, pressure={0.45, 0.75}})
work(mMid, {hand="body", pile=midP, coverage=2.6, fill=true, clip=true, angle=0,
            length={25, 65}, pressure={0.45, 0.75}})
work(mWater, {hand="glaze", pile=waterEven, coverage=2, fill=true, clip=true,
              angle=0, length={110, 260}, pressure={0.45, 0.65}})
work(mNear, {hand="glaze", pile=waterDarkP, coverage=1.7, fill=true, threshold=0.02,
             clip=true, angle=0, length={100, 240}, pressure={0.45, 0.65}})
print("ranges and lake solid again")

--@ chunk 80
-- Solid films without fill: more strokes, no dabs, so no blobs.
work(mFar, {hand="body", pile=farP, coverage=4.2, clip=true, angle=0,
            length={22, 55}, pressure={0.45, 0.75}})
work(mMid, {hand="body", pile=midP, coverage=4.2, clip=true, angle=0,
            length={22, 55}, pressure={0.45, 0.75}})
work(mWater, {hand="glaze", pile=waterEven, coverage=3.2, clip=true, angle=0,
              length={110, 260}, pressure={0.45, 0.65}})
work(mNear, {hand="glaze", pile=waterDarkP, coverage=2.4, threshold=0.03, clip=true,
             angle=0, length={100, 240}, pressure={0.45, 0.65}})
work(mShore, {hand="body", pile=bankP, coverage=3.6, clip=true, angle=-0.25,
              length={22, 55}, pressure={0.5, 0.8}})
work(mFore, {hand="glaze", pile=earthDeep, coverage=2, threshold=0.03, clip=true,
             angle=-0.25, length={70, 170}, pressure={0.45, 0.65}})
print("solid, no fill")

--@ chunk 81
-- the trees, the woman, the boat, the far trees: all laid again
work(mAt, {hand="body", pile=treeDark, coverage=2.2, clip=true, tool="filbert 6",
           length={20, 50}, pressure={0.5, 0.85}})
work(mA, {hand="body", pile=treeDark, coverage=2.4, clip=true, tool="filbert 4",
          length={15, 45}, pressure={0.4, 0.75}})
work(mBt, {hand="body", pile=treeFar, coverage=2.2, clip=true, tool="filbert 6",
           length={20, 50}, pressure={0.45, 0.8}})
work(mB, {hand="body", pile=treeFar, coverage=2.2, clip=true, tool="filbert 4",
          length={15, 45}, pressure={0.4, 0.7}})
work(mFig, {hand="body", pile=figP, coverage=2.4, clip=true, tool="filbert 3",
            length={6, 18}, pressure={0.5, 0.85}})
work(hull, {hand="body", pile=boatP, coverage=2, clip=true, tool="filbert 3",
            length={6, 18}, pressure={0.45, 0.75}})
work(gunwale, {hand="body", pile=gunP, coverage=1.6, clip=true, tool="filbert 2.5",
               length={5, 14}, pressure={0.4, 0.7}})
work(inside, {hand="body", pile=figP, coverage=1.4, clip=true, tool="filbert 2.5",
              length={5, 12}, pressure={0.4, 0.7}})
work(pole, {hand="body", pile=boatP, coverage=1.6, clip=true, tool="filbert 2.5",
            length={6, 16}, pressure={0.45, 0.7}})
work(mFarTrees, {hand="body", pile=hazeP, coverage=1.8, clip=true, tool="filbert 3",
                 length={8, 22}, pressure={0.35, 0.6}})
print("incident back")

--@ chunk 82
-- and the light on the lake again, finer this time, and the reflections of the far bank
work(mStreakW, {hand="body", pile=streakWarm, coverage=0.5, threshold=0.06,
                clip=true, angle=0, tool="filbert 3", length={40, 130},
                pressure={0.25, 0.45}})
work(mStreakC, {hand="body", pile=streakCool, coverage=0.4, threshold=0.06,
                clip=true, angle=0, tool="filbert 3", length={40, 140},
                pressure={0.25, 0.42}})
work(mRefl, {hand="glaze", pile=reflP, coverage=1.1, threshold=0.05, clip=true,
             angle=0, length={80, 200}, pressure={0.3, 0.5}})
work(mRefl2, {hand="glaze", pile=reflP, coverage=0.9, threshold=0.05, clip=true,
              angle=0, length={80, 200}, pressure={0.3, 0.5}})
print("light on the lake")

--@ chunk 83
print(wait(1440))
print(drying(500, 100), drying(300, 550), drying(500, 700))

--@ chunk 84
-- the sky's dappled orange-and-blue is the weakest passage in the picture: one
-- smooth film over it, then the glow put back low down where it belongs
skySmooth = pile{{"lead white", 4.4}, {"pale smalt", 2.6}, {"smalt", 1.6},
                 {"raw umber", 0.8}}
work(skyM, {hand="glaze", pile=skySmooth, coverage=3.4, clip=true, angle=-0.03,
            length={160, 300}, pressure={0.45, 0.65}})
work(mZen2, {hand="glaze", pile=skyDeep2, coverage=2.2, threshold=0.04, clip=true,
             angle=-0.03, length={150, 280}, pressure={0.4, 0.6}})
work(mWarm, {hand="glaze", pile=glowBody2, coverage=1.8, threshold=0.04, clip=true,
             angle=-0.03, length={140, 270}, pressure={0.35, 0.55}})
work(mGlow2, {hand="glaze", pile=glowHot2, coverage=2, threshold=0.04, clip=true,
              angle=-0.03, length={140, 270}, pressure={0.35, 0.55}})
print("sky smoothed")

--@ chunk 85
-- the glaze dragged the still-tacky branches into blobs; lay the trees again, crisp
work(mAt, {hand="body", pile=treeDark, coverage=2.6, clip=true, tool="filbert 6",
           length={20, 50}, pressure={0.55, 0.9}})
work(mA, {hand="body", pile=treeDark, coverage=3, clip=true, tool="filbert 3",
          length={12, 40}, pressure={0.45, 0.85}})
work(mBt, {hand="body", pile=treeFar, coverage=2.4, clip=true, tool="filbert 6",
           length={20, 50}, pressure={0.5, 0.85}})
work(mB, {hand="body", pile=treeFar, coverage=2.6, clip=true, tool="filbert 3",
          length={12, 40}, pressure={0.45, 0.8}})
print("trees crisp again")

--@ chunk 86
-- The left massif is the nearest range: it must be the darkest thing above the fog.
midDark = pile{{"lead white", 1.2}, {"pale smalt", 2}, {"smalt", 1.6},
               {"raw umber", 3}, {"bone black", 1.4}}
work(mMid, {hand="body", pile=midDark, coverage=3.4, clip=true, angle=0,
            length={22, 55}, pressure={0.5, 0.8}})

-- the far range: pale where the light strikes it, cool and dark towards its base
mFarBase = mask(function(x, y)
  local v = clamp((y - f1(x) - 10) / 70, 0, 1)
  return clamp(v * v * (3 - 2 * v), 0, 1) * 1.2
end) * mFar
work(mFarBase, {hand="glaze", pile=reflP, coverage=1.5, threshold=0.05, clip=true,
                angle=-0.9, length={60, 150}, pressure={0.3, 0.5}})
mFarCrest = mask(function(x, y)
  local v = clamp(((f1(x) + 26) - y) / 26, 0, 1)
  local h = clamp(1 - math.abs(x - 680) / 400, 0, 1)
  return clamp(v * (0.25 + 0.75 * h), 0, 1) * 1.2
end) * mFar
work(mFarCrest, {hand="glaze", pile=peakLightP, coverage=1.3, threshold=0.05,
                 clip=true, angle=-0.2, length={50, 130}, pressure={0.3, 0.5}})
print("the two planes separated")

--@ chunk 87
-- that was too strong: the far range laid solid again, with only a whisper of form
work(mFar, {hand="body", pile=farP, coverage=3.6, clip=true, angle=0,
            length={22, 55}, pressure={0.45, 0.75}})
work(mFarBase, {hand="glaze", pile=reflP, coverage=0.45, threshold=0.08, clip=true,
                angle=-0.85, length={80, 190}, pressure={0.22, 0.36}})
work(mFarCrest, {hand="glaze", pile=peakLightP, coverage=0.5, threshold=0.08,
                 clip=true, angle=-0.2, length={70, 170}, pressure={0.22, 0.36}})
print("far range solid")

--@ chunk 88
-- the pale far-range film lay over the massif where the two ridges nearly meet:
-- the near plane goes on last
work(mMid, {hand="body", pile=midDark, coverage=3.4, clip=true, angle=0,
            length={22, 55}, pressure={0.5, 0.8}})
mFeet4 = mask(function(x, y)
  local v = clamp((y - (silhouette(x) - 4)) / 18, 0, 1)
  return clamp(v * 0.6 * (1 - 0.5 * clamp((y - 448) / 26, 0, 1)), 0, 1)
end) * mMid
work(mFeet4, {hand="glaze", pile=mistP, coverage=1.4, threshold=0.06, clip=true,
              angle=-0.02, length={80, 200}, pressure={0.3, 0.5}})
print("massif brought forward")

--@ chunk 89
-- the far trees read as television aerials; a low, dense, hazier treeline instead
far2 = {}
nzT = noise{seed=11, period=70, octaves=3}
function lowTree(x, y, hgt)
  local lean = randn(0, 1.5)
  local pts = {{x, y}, {x + lean * 0.4, y - hgt * 0.6}, {x + lean, y - hgt}}
  far2[#far2 + 1] = {pts = pts, w = 1.7}
  for i = 1, 4 do
    local t = 0.35 + i * 0.16
    local bx, by = x + lean * t, y - hgt * t
    local s = (i % 2 == 0) and 1 or -1
    local L = hgt * rand(0.2, 0.34)
    far2[#far2 + 1] = {pts = {{bx, by}, {bx + s * L, by - L * rand(0.7, 1.3)}}, w = 1.0}
  end
end
for i = 1, 150 do
  local x = 782 + rand(0, 230)
  local hgt = (7 + 12 * nzT:at01(x, 10)) * rand(0.7, 1.25)
  lowTree(x, f3(x) + rand(1, 5), hgt)
end
for i = 1, 80 do
  local x = -6 + rand(0, 165)
  local hgt = (6 + 9 * nzT:at01(x, 60)) * rand(0.7, 1.2)
  lowTree(x, f2(x) + rand(1, 5), hgt)
end
mFar2 = nil
for _, t in ipairs(far2) do
  local r = ribbon(t.pts, t.w)
  mFar2 = (mFar2 == nil) and r or (mFar2 + r)
end
work(mFarTrees, {hand="body", pile=hazeP, coverage=1.6, clip=true,
                 tool="filbert 3", length={8, 22}, pressure={0.35, 0.6}})
hazeTree = pile{{"pale smalt", 2.4}, {"raw umber", 2.2}, {"bone black", 0.7},
               {"lead white", 1}}
work(mFar2, {hand="body", pile=hazeTree, coverage=1.8, clip=true, tool="filbert 3",
             length={6, 16}, pressure={0.4, 0.7}})
print("treelines", mFar2:area())

--@ chunk 90
print("mMid", mMid:at(200, 420), mMid:at(200, 400), mMid:at(200, 450), mMid:at(200, 470))
print("feveil", mFeet4:at(200, 420), mFeet4:at(200, 445))
print("far", mFar:at(200, 420), mFar:at(200, 400))

--@ chunk 91
-- the veil was 0.6 over the whole massif and bleached it: massif dark, veil confined
-- to its feet and much weaker
work(mMid, {hand="body", pile=midDark, coverage=3.6, clip=true, angle=0,
            length={22, 55}, pressure={0.5, 0.8}})
mFeet5 = mask(function(x, y)
  local v = clamp((y - (silhouette(x) + 18)) / 26, 0, 1)
  return clamp(v * 0.45, 0, 1)
end) * mMid
work(mFeet5, {hand="glaze", pile=mistP, coverage=1.3, threshold=0.05, clip=true,
              angle=-0.02, length={80, 200}, pressure={0.3, 0.5}})
print("massif dark, feet only veiled")

--@ chunk 92
-- the massif wants a dark that has body: bone black and lead white carry the hiding,
-- the earths only stain it
midDark2 = pile{{"bone black", 3.4}, {"lead white", 1.6}, {"pale smalt", 1.4},
                {"raw umber", 1.6}}
work(mMid, {hand="body", pile=midDark2, coverage=3.6, clip=true, angle=0,
            length={22, 55}, pressure={0.5, 0.8}})
-- the little trees were sitting on the massif's crest; lay them back into the mass
mFar2L = mask(function(x, y) return clamp((200 - x) / 30, 0, 1) end)
work(mFar2 * mFar2L, {hand="body", pile=midDark2, coverage=1.8, clip=true,
                      tool="filbert 3", length={6, 16}, pressure={0.4, 0.7}})
work(mFeet5, {hand="glaze", pile=mistP, coverage=1.2, threshold=0.05, clip=true,
              angle=-0.02, length={80, 200}, pressure={0.3, 0.5}})
print("massif dark with body")

--@ chunk 93
DA = pile{{"bone black", 5}, {"lead white", 1}}
DB = pile{{"bone black", 3}, {"lead white", 2}, {"pale smalt", 1.5}}
DC = pile{{"bone black", 2}, {"lead white", 2.5}, {"raw umber", 1.5}, {"pale smalt", 1.5}}
for i, p in ipairs({DA, DB, DC}) do
  work(rect(100 + (i - 1) * 62, 405, 50, 36), {hand="body", pile=p, coverage=3.5,
            clip=true, length={18, 40}, pressure={0.5, 0.8}})
end
print("dark swatches")

--@ chunk 94
-- swatch A's weight is the one that darkens: bone black carries this pile
midDark3 = pile{{"bone black", 4}, {"lead white", 1.2}, {"pale smalt", 1},
                {"raw umber", 1}}
work(mMid, {hand="body", pile=midDark3, coverage=3.6, clip=true, angle=0,
            length={22, 55}, pressure={0.5, 0.8}})
work(mFeet5, {hand="glaze", pile=mistP, coverage=1.1, threshold=0.05, clip=true,
              angle=-0.02, length={80, 200}, pressure={0.3, 0.5}})
print("massif in shadow")

--@ chunk 95
work(mFeet5, {hand="glaze", pile=midDark3, coverage=1.5, threshold=0.05, clip=true,
              angle=-0.02, length={80, 200}, pressure={0.3, 0.5}})
work(mFar2 * mFar2L, {hand="body", pile=midDark3, coverage=2.2, clip=true,
                      tool="filbert 3", length={6, 16}, pressure={0.45, 0.75}})
mFoot6 = mask(function(x, y)
  local v = clamp((y - 438) / 12, 0, 1) * clamp((472 - y) / 18, 0, 1)
  return clamp(v * 0.85, 0, 1)
end) * (mMid + mFar)
work(mFoot6, {hand="glaze", pile=mistP, coverage=1.5, threshold=0.06, clip=true,
              angle=0, length={80, 200}, pressure={0.3, 0.5}})
print("massif's lower half back")

--@ chunk 96
-- the lake's light was a mesh of scratches: knock most of it back, keep a few
work(mWater, {hand="glaze", pile=waterEven, coverage=1.3, clip=true, angle=0,
              length={120, 280}, pressure={0.35, 0.5}})
work(mStreakW, {hand="body", pile=streakWarm, coverage=0.22, threshold=0.10,
                clip=true, angle=0, tool="filbert 2.5", length={70, 200},
                pressure={0.3, 0.5}})
work(mStreakC, {hand="body", pile=streakCool, coverage=0.16, threshold=0.10,
                clip=true, angle=0, tool="filbert 2.5", length={70, 200},
                pressure={0.28, 0.45}})
-- the far treeline on the right bank: darker, so it reads as trees and not frost
treeLineP = pile{{"bone black", 3}, {"raw umber", 1.6}, {"pale smalt", 1.4}}
work(mFar2, {hand="body", pile=treeLineP, coverage=1.6, clip=true, tool="filbert 3",
             length={6, 16}, pressure={0.45, 0.75}})
print("lake quieted, treeline darkened")

--@ chunk 97
print(wait(2880))
print(drying(300, 550), drying(560, 620), drying(100, 550), drying(700, 460))

--@ chunk 98
-- dry now, so the incident can be laid again without smearing
work(mAt, {hand="body", pile=treeDark, coverage=2.4, clip=true, tool="filbert 6",
           length={20, 50}, pressure={0.55, 0.9}})
work(mA, {hand="body", pile=treeDark, coverage=2.8, clip=true, tool="filbert 3",
          length={12, 40}, pressure={0.5, 0.9}})
work(mBt, {hand="body", pile=treeFar, coverage=2.4, clip=true, tool="filbert 6",
           length={20, 50}, pressure={0.5, 0.85}})
work(mB, {hand="body", pile=treeFar, coverage=2.6, clip=true, tool="filbert 3",
          length={12, 40}, pressure={0.5, 0.85}})
work(mFig, {hand="body", pile=figP, coverage=2.6, clip=true, tool="filbert 3",
            length={6, 18}, pressure={0.55, 0.9}})
work(hull, {hand="body", pile=boatP, coverage=2.2, clip=true, tool="filbert 3",
            length={6, 18}, pressure={0.5, 0.8}})
work(gunwale, {hand="body", pile=gunP, coverage=1.8, clip=true, tool="filbert 2.5",
               length={5, 14}, pressure={0.45, 0.75}})
work(inside, {hand="body", pile=figP, coverage=1.6, clip=true, tool="filbert 2.5",
              length={5, 12}, pressure={0.45, 0.75}})
work(pole, {hand="body", pile=boatP, coverage=1.8, clip=true, tool="filbert 2.5",
            length={6, 16}, pressure={0.5, 0.75}})
work(mFar2, {hand="body", pile=treeLineP, coverage=1.6, clip=true, tool="filbert 3",
             length={6, 16}, pressure={0.45, 0.75}})
print("incident, dry pass")

--@ chunk 99
-- the lake minus everything standing in it, so burying the squiggles can't bury the trees
standing = mA + mB + mFig + hull + inside + gunwale + pole
mW2 = mWater * (-standing)
work(mW2, {hand="glaze", pile=waterEven, coverage=2.6, clip=true, angle=0,
           length={120, 280}, pressure={0.45, 0.62}})
work(mW2, {hand="glaze", pile=waterEven, coverage=1.2, clip=true, angle=0,
           length={150, 300}, pressure={0.32, 0.48}})

-- and the light back, as a few long drawn streaks rather than a sprayed mask
sl = brush{kind = "filbert", width = 2.6, stiffness = 0.35}
for i = 1, 26 do
  local y = 480 + (i / 26) * 165
  local x0 = rand(-40, 520)
  local L = rand(90, 330) * (1 - 0.35 * ((y - 480) / 165))
  local warm = (x0 + L / 2) > 430 and rand() < 0.7 or rand() < 0.25
  sl:load(warm and streakWarm or streakCool, rand(0.35, 0.7))
  sl:stroke({{x0, y}, {x0 + L * 0.5, y + randn(0, 1.6)}, {x0 + L, y + randn(0, 2.4)}},
            {pressure = {rand(0.3, 0.5), 0.45, 0.05}, orient = "across"})
end
print("lake quieted, streaks drawn")

--@ chunk 100
-- cover her, then cut her again: a woman with a bundle carried on her left, the sticks
-- projecting up and away rather than standing on end
mCover = rect(520, 583, 86, 84):blur(15)
work(mCover, {hand="glaze", pile=waterDarkP, coverage=2.4, clip=true, angle=0,
              length={40, 110}, pressure={0.4, 0.6}})
work(rect(548, 652, 86, 56):blur(12), {hand="body", pile=mudP, coverage=2,
      clip=true, angle=0, length={25, 60}, pressure={0.4, 0.65}})

head2 = ellipse(561, 602, 4.4, 5)
shoulders = poly({{554, 607}, {559, 604.5}, {566, 604.5}, {571, 608.5},
                  {571.5, 615}, {568, 617}, {556, 617}, {552, 614}})
body2 = poly({{556, 615}, {568, 615}, {570, 630}, {571.5, 640}, {552, 641},
              {553, 630}})
skirt2 = poly({{551.5, 639}, {571.5, 639}, {573.5, 650}, {571, 656},
               {557, 656.5}, {549.5, 651}})
armR = poly({{568, 617}, {571, 618}, {572.5, 630}, {570, 632}, {567.5, 628}})
armL = poly({{553, 618}, {556, 617.5}, {555, 626}, {551.5, 627}, {551, 622}})
mFig2 = head2 + shoulders + body2 + skirt2 + armR + armL
for i = 1, 8 do
  local a = -2.95 + (i - 1) * 0.085 + randn(0, 0.02)
  local L = rand(19, 28)
  mFig2 = mFig2 + ribbon({{556, 610}, {556 + math.cos(a) * L, 610 + math.sin(a) * L}}, 1.25)
end
mFig2 = mFig2 + ribbon({{552, 619}, {541, 630}}, 2.4)     -- the arm carrying the bundle
figP2 = pile{{"bone black", 3.4}, {"raw umber", 1.4}, {"pale smalt", 0.9}}
work(mFig2, {hand="body", pile=figP2, coverage=2.6, clip=true, tool="filbert 3",
             length={6, 18}, pressure={0.55, 0.9}})
print("figure, cut again")

--@ chunk 101
-- knock the caterpillar treelines back into their banks
work(mFar2 * mFar2L, {hand="body", pile=midDark3, coverage=2.4, clip=true,
                      tool="filbert 3", length={6, 16}, pressure={0.45, 0.75}})
mFar2R = mFar2 * mask(function(x, y) return clamp((x - 760) / 60, 0, 1) end)
work(mFar2R, {hand="body", pile=bankP, coverage=2.4, clip=true, tool="filbert 3",
              length={6, 16}, pressure={0.45, 0.75}})

-- trees again, but scattered: varied heights, gaps, a few standing clear of the rest
far3 = {}
function smallTree(x, y, hgt)
  local lean = randn(0, 2.2)
  far3[#far3 + 1] = {pts = {{x, y}, {x + lean * 0.5, y - hgt * 0.6}, {x + lean, y - hgt}},
                     w = 1.6}
  local k = rand(3, 5)
  for i = 1, k do
    local t = 0.3 + 0.62 * (i - 1) / (k - 1) + randn(0, 0.05)
    local bx, by = x + lean * t, y - hgt * t
    local s = (i % 2 == 0) and 1 or -1
    local L = hgt * rand(0.18, 0.36)
    far3[#far3 + 1] = {pts = {{bx, by}, {bx + s * L, by - L * rand(0.6, 1.4)}}, w = 0.9}
  end
end
nzT2 = noise{seed=23, period=95, octaves=3}
for i = 1, 34 do
  local x = 776 + (i - 1) * 7 + randn(0, 3.4)
  local hgt = 6 + 20 * nzT2:at01(x, 10)
  smallTree(x, f3(x) + rand(1, 4), hgt * rand(0.6, 1.25))
end
for i = 1, 22 do
  local x = -6 + (i - 1) * 8 + randn(0, 3.6)
  local hgt = 5 + 13 * nzT2:at01(x, 60)
  smallTree(x, f2(x) + rand(1, 4), hgt * rand(0.6, 1.2))
end
mFar3 = nil
for _, t in ipairs(far3) do
  local r = ribbon(t.pts, t.w)
  mFar3 = (mFar3 == nil) and r or (mFar3 + r)
end
hazeTree2 = pile{{"bone black", 2.4}, {"raw umber", 1.8}, {"pale smalt", 1.8}}
work(mFar3, {hand="body", pile=hazeTree2, coverage=1.8, clip=true, tool="filbert 2.5",
             length={5, 14}, pressure={0.4, 0.7}})
print("treelines redrawn", mFar3:area())

--@ chunk 102
-- the old treelines are still there as pale scribbles; cover them with the colour
-- of whatever they stand against
fogP = pile{{"lead white", 5}, {"pale smalt", 1.5}, {"raw umber", 0.4}}
R = mask(function(x, y) return clamp((x - 700) / 80, 0, 1) end)
Lf = mask(function(x, y) return clamp((330 - x) / 80, 0, 1) end)
work((mFarTrees + mFar2) * R, {hand="body", pile=fogP, coverage=3, clip=true,
      tool="filbert 3", length={8, 22}, pressure={0.45, 0.75}})
work((mFarTrees + mFar2) * Lf, {hand="body", pile=midDark3, coverage=3, clip=true,
      tool="filbert 3", length={8, 22}, pressure={0.45, 0.75}})
print("old treelines covered")

--@ chunk 103
-- the cover took the new trees with it; lay them back, the right ones dark against the
-- light fog, the left ones a step greyer
treeDarkR = pile{{"bone black", 3}, {"raw umber", 1.5}, {"pale smalt", 1}}
mFar3R = mFar3 * R
mFar3L = mFar3 * Lf
work(mFar3R, {hand="body", pile=treeDarkR, coverage=2.4, clip=true, tool="filbert 2.5",
              length={5, 14}, pressure={0.45, 0.75}})
work(mFar3L, {hand="body", pile=hazeTree2, coverage=2, clip=true, tool="filbert 2.5",
              length={5, 14}, pressure={0.4, 0.7}})
-- fog over their feet, so they stand in it
mTreesFog = mask(function(x, y)
  local v = clamp((y - 428) / 10, 0, 1) * clamp((456 - y) / 14, 0, 1)
  return clamp(v * 0.8, 0, 1)
end) * (mFar3 + mFar2 + mFarTrees)
work(mTreesFog, {hand="glaze", pile=fogP, coverage=1.5, threshold=0.05, clip=true,
                 angle=0, length={80, 200}, pressure={0.3, 0.5}})
print("treelines again")

--@ chunk 104
-- clear both treeline zones through the mountain film and the lake film
work(mFar, {hand="body", pile=farP, coverage=3, clip=true, angle=0,
            length={22, 55}, pressure={0.45, 0.75}})
work(mW2, {hand="glaze", pile=waterEven, coverage=2.4, clip=true, angle=0,
           length={120, 280}, pressure={0.45, 0.62}})
work(mMid, {hand="body", pile=midDark3, coverage=3.2, clip=true, angle=0,
            length={22, 55}, pressure={0.5, 0.8}})

-- and the two treelines, once, as clean groups
far4 = {}
function stree(x, y, hgt)
  local lean = randn(0, 1.8)
  far4[#far4 + 1] = {pts = {{x, y}, {x + lean * 0.5, y - hgt * 0.6}, {x + lean, y - hgt}},
                     w = 1.7}
  local k = rand(3, 5)
  for i = 1, k do
    local t = 0.28 + 0.64 * (i - 1) / (k - 1) + randn(0, 0.05)
    local bx, by = x + lean * t, y - hgt * t
    local s = (i % 2 == 0) and 1 or -1
    local L = hgt * rand(0.2, 0.36)
    far4[#far4 + 1] = {pts = {{bx, by}, {bx + s * L, by - L * rand(0.6, 1.4)}}, w = 1.0}
  end
end
nzT3 = noise{seed=31, period=110, octaves=3}
for i = 1, 26 do
  local x = 792 + (i - 1) * 8.4 + randn(0, 3)
  stree(x, f3(x) + rand(1, 3), (7 + 19 * nzT3:at01(x, 10)) * rand(0.65, 1.2))
end
for i = 1, 16 do
  local x = -4 + (i - 1) * 10 + randn(0, 4)
  stree(x, f2(x) + rand(1, 3), (5 + 11 * nzT3:at01(x, 70)) * rand(0.6, 1.2))
end
mFar4 = nil
for _, t in ipairs(far4) do
  local r = ribbon(t.pts, t.w)
  mFar4 = (mFar4 == nil) and r or (mFar4 + r)
end
mFar4R = mFar4 * R
mFar4L = mFar4 * Lf
work(mFar4R, {hand="body", pile=treeDarkR, coverage=2.4, clip=true,
              tool="filbert 2.5", length={5, 14}, pressure={0.45, 0.8}})
work(mFar4L, {hand="body", pile=hazeTree2, coverage=2, clip=true,
              tool="filbert 2.5", length={5, 14}, pressure={0.4, 0.7}})
mFogFeet = mask(function(x, y)
  local v = clamp((y - 430) / 8, 0, 1) * clamp((452 - y) / 12, 0, 1)
  return clamp(v * 0.75, 0, 1)
end) * mFar4
work(mFogFeet, {hand="glaze", pile=fogP, coverage=1.4, threshold=0.05, clip=true,
                angle=0, length={80, 200}, pressure={0.3, 0.5}})
print("treelines, once more")

--@ chunk 105
-- the hand-drawn streaks crossed the trunks; the trees go on top again
work(mAt, {hand="body", pile=treeDark, coverage=2.4, clip=true, tool="filbert 6",
           length={20, 50}, pressure={0.55, 0.9}})
work(mA, {hand="body", pile=treeDark, coverage=2.8, clip=true, tool="filbert 3",
          length={12, 40}, pressure={0.5, 0.9}})
work(mBt, {hand="body", pile=treeFar, coverage=2.4, clip=true, tool="filbert 6",
           length={20, 50}, pressure={0.5, 0.85}})
work(mB, {hand="body", pile=treeFar, coverage=2.6, clip=true, tool="filbert 3",
          length={12, 40}, pressure={0.5, 0.85}})

-- the ranges' dabs went mechanical: a long light glaze to level them
work(mFar, {hand="glaze", pile=farP, coverage=1.3, clip=true, angle=-0.12,
            length={150, 290}, pressure={0.4, 0.6}})
work(mMid, {hand="glaze", pile=midDark3, coverage=1.3, clip=true, angle=-0.12,
            length={150, 290}, pressure={0.4, 0.6}})
print("trees back, ranges levelled")

--@ chunk 106
print(wait(2880))
print(drying(200, 430), drying(300, 550), draining or nil)
print(drying(700, 400), drying(500, 700), drying(500, 150))

--@ chunk 107
-- the ranges laid cleanly once more, then the treelines standing on them
work(mMid, {hand="body", pile=midDark3, coverage=3.6, clip=true, angle=-0.1,
            length={22, 55}, pressure={0.5, 0.8}})
work(mFar, {hand="body", pile=farP, coverage=3, clip=true, angle=-0.1,
            length={22, 55}, pressure={0.45, 0.75}})
-- the massif's wooded texture: soft dark clumps, not dabs
nzM = noise{seed=17, period=55, octaves=3}
mWood = mask(function(x, y)
  if y < f2(x) + 4 or y > 452 then return 0 end
  return clamp(0.35 + 0.65 * nzM:at01(x, y * 0.6), 0, 1)
end) * mMid
work(mWood, {hand="glaze", pile=midDark3, coverage=0.7, threshold=0.06, clip=true,
             angle=-0.9, length={40, 110}, pressure={0.3, 0.5}})
work(mFar4R, {hand="body", pile=treeDarkR, coverage=2.4, clip=true,
              tool="filbert 2.5", length={5, 14}, pressure={0.5, 0.85}})
work(mFar4L, {hand="body", pile=hazeTree2, coverage=2, clip=true,
              tool="filbert 2.5", length={5, 14}, pressure={0.45, 0.75}})
mFogFeet = mask(function(x, y)
  local v = clamp((y - 430) / 8, 0, 1) * clamp((452 - y) / 12, 0, 1)
  return clamp(v * 0.7, 0, 1)
end) * mFar4
work(mFogFeet, {hand="glaze", pile=fogP, coverage=1.3, threshold=0.06, clip=true,
                angle=0, length={80, 200}, pressure={0.3, 0.5}})
print("ranges and treelines")

--@ chunk 108
work(mMid, {hand="glaze", pile=midDark3, coverage=2.8, clip=true, angle=-0.08,
            length={180, 320}, pressure={0.45, 0.65}})
work(mFar, {hand="glaze", pile=farP, coverage=2.8, clip=true, angle=-0.08,
            length={180, 320}, pressure={0.45, 0.62}})
mCrestM = mask(function(x, y)
  local v = clamp(((f2(x) + 14) - y) / 14, 0, 1)
  return clamp(v * 1.2, 0, 1)
end) * mMid
crestP = pile{{"lead white", 3}, {"pale smalt", 2}, {"raw umber", 1}}
work(mCrestM, {hand="glaze", pile=crestP, coverage=0.8, threshold=0.06, clip=true,
               angle=-0.1, length={120, 260}, pressure={0.3, 0.5}})
print("ranges quiet")

--@ chunk 109
-- thin glazes lay too little to cover; the massif wants real body colour
work(mMid, {hand="body", pile=midDark3, coverage=4.5, clip=true, angle=-0.08,
            length={20, 50}, pressure={0.6, 0.95}})
work(mFar, {hand="body", pile=farP, coverage=4, clip=true, angle=-0.08,
            length={20, 50}, pressure={0.55, 0.9}})
print("ranges, body colour")

--@ chunk 110
print(midDark3)
print(midDark2)
print(figP)

--@ chunk 111
work(rect(150, 400, 100, 40), {hand="body", pile=midDark3, coverage=5,
      fill=true, pressure={0.6, 0.95}})
print("test rect")

--@ chunk 112
print("mMid", mMid:at(200, 430), mMid:at(200, 410), mMid:at(300, 440), mMid:at(100, 420))
print("area", mMid:area(), mFar:area())

--@ chunk 113
work(mMid, {hand="body", pile=midDark3, coverage=3, fill=true, clip=true,
            angle=-0.08, length={20, 50}, pressure={0.6, 0.95}})
print("massif with fill")

--@ chunk 114
work(mMid, {hand="glaze", pile=midDark3, coverage=2, fill=true, clip=true,
            angle=-0.08, length={150, 280}, pressure={0.5, 0.72}})
work(mFar, {hand="glaze", pile=farP, coverage=2, fill=true, clip=true, angle=-0.08,
            length={150, 280}, pressure={0.5, 0.7}})

-- the bank: its grass edge was a bright stripe; quieter, and darker in front
bankQuiet = pile{{"raw umber", 4}, {"green earth", 1.6}, {"bone black", 2},
                 {"lead white", 0.6}}
work(mLit, {hand="body", pile=bankQuiet, coverage=1.6, clip=true, angle=-1.2,
            tool="filbert 4", length={12, 32}, pressure={0.35, 0.6}})
work(mShore, {hand="glaze", pile=bankQuiet, coverage=1.4, fill=true, clip=true,
              angle=-0.25, length={90, 220}, pressure={0.45, 0.65}})
mFore2 = mask(function(x, y)
  local v = clamp((y - 672) / 95, 0, 1)
  return clamp(v * v * (3 - 2 * v) * 1.25, 0, 1)
end) * mShore
work(mFore2, {hand="glaze", pile=earthDeep, coverage=1.8, fill=true, threshold=0.03,
              clip=true, angle=-0.25, length={90, 220}, pressure={0.45, 0.68}})
print("massif evened, bank quieted")

--@ chunk 115
-- the far range lies over the massif where the ridges nearly meet; so the near plane
-- has to go on last, every time
work(mFar, {hand="body", pile=farP, coverage=2.6, fill=true, clip=true, angle=-0.08,
            length={20, 50}, pressure={0.55, 0.9}})
work(mMid, {hand="body", pile=midDark3, coverage=2.8, fill=true, clip=true, angle=-0.08,
            length={20, 50}, pressure={0.55, 0.9}})
work(mFar4R, {hand="body", pile=treeDarkR, coverage=2.4, clip=true,
              tool="filbert 2.5", length={5, 14}, pressure={0.5, 0.85}})
work(mFar4L, {hand="body", pile=hazeTree2, coverage=2, clip=true,
              tool="filbert 2.5", length={5, 14}, pressure={0.45, 0.75}})
work(mFogFeet, {hand="glaze", pile=fogP, coverage=1.2, threshold=0.06, clip=true,
                angle=0, length={80, 200}, pressure={0.3, 0.5}})

-- the bank again: its grass and reeds
rr2 = brush{kind = "round", width = 1.8, point = 0.9, stiffness = 0.45}
for i = 1, 90 do
  local x = 20 + (i - 1) * 10.8 + randn(0, 4.5)
  local by = shoreFn(x) + rand(2, 16)
  local hgt = rand(7, 24)
  rr2:load(rand() < 0.6 and reedP or grassDark, rand(0.35, 0.8))
  rr2:stroke({{x, by}, {x + randn(0, 2), by - hgt * 0.6}, {x + randn(0, 4) + 2, by - hgt}},
             {pressure = {rand(0.45, 0.7), 0.5, 0.08}, orient = "across"})
end
-- a few darker tufts standing up against the water
tuftP = pile{{"bone black", 2.6}, {"green earth", 1.4}, {"raw umber", 1.4}}
for i = 1, 26 do
  local x = 330 + (i - 1) * 25 + randn(0, 10)
  local by = shoreFn(x) + rand(3, 12)
  for k = 1, 5 do
    local a = -math.pi / 2 + randn(0, 0.55)
    local hgt = rand(9, 20)
    rr2:load(tuftP, rand(0.4, 0.8))
    rr2:stroke({{x + randn(0, 3), by}, {x + math.cos(a) * hgt * 0.6,
                by + math.sin(a) * hgt * 0.6}, {x + math.cos(a) * hgt,
                by + math.sin(a) * hgt}}, {pressure = {0.6, 0.45, 0.05},
                                         orient = "across"})
  end
end
print("ranges, treelines, bank grass")

--@ chunk 116
-- the massif lies under five layers of near-white fog; three passes of dark to beat
-- them down, then only a narrow band of fog at its foot
for i = 1, 3 do
  work(mMid, {hand="body", pile=midDark3, coverage=2.6, fill=true, clip=true,
              angle=-0.08 + i * 0.05, length={18, 45}, pressure={0.6, 0.95}})
end
mFootNarrow = mask(function(x, y)
  local v = clamp((y - 442) / 10, 0, 1) * clamp((474 - y) / 16, 0, 1)
  return clamp(v * 0.9, 0, 1)
end) * (mMid + mFar)
work(mFootNarrow, {hand="glaze", pile=fogP, coverage=1.6, fill=true, threshold=0.06,
                   clip=true, angle=0, length={90, 220}, pressure={0.35, 0.55}})
work(mFar4L, {hand="body", pile=hazeTree2, coverage=2, clip=true,
              tool="filbert 2.5", length={5, 14}, pressure={0.45, 0.75}})
work(mFar4R, {hand="body", pile=treeDarkR, coverage=2.2, clip=true,
              tool="filbert 2.5", length={5, 14}, pressure={0.5, 0.85}})
print("massif beaten down")

--@ chunk 117
print(drying(300, 430), drying(300, 550), drying(300, 700), drying(500, 200))
print(wait(1440))
print(drying(300, 430), drying(300, 550), drying(300, 700), drying(500, 200))

--@ chunk 118
-- the fog band evened out, and its lower edge softened
mFogBand = mask(function(x, y)
  local v = clamp((y - 424) / 12, 0, 1) * clamp((492 - y) / 34, 0, 1)
  local h = 0.35 + 0.65 * clamp(1 - math.abs(x - 520) / 700, 0, 1)
  return clamp(v * h * 1.15, 0, 1)
end)
work(mFogBand, {hand="glaze", pile=fogP, coverage=1.5, fill=true, threshold=0.04,
                clip=true, angle=-0.02, length={140, 280}, pressure={0.32, 0.5}})

-- the lake's light, a touch quieter: a veil of its own tone over the streaks
work(mW2, {hand="glaze", pile=waterEven, coverage=1, clip=true, angle=0,
           length={150, 300}, pressure={0.3, 0.45}})

-- one warm glaze over the whole picture, thin, to bind it
warmGlaze = pile{{"lead white", 3}, {"yellow ochre", 1.2}, {"pale smalt", 1},
                 {"raw umber", 0.4}, medium=0.5}
work(everywhere(), {hand="glaze", pile=warmGlaze, coverage=0.5, clip=true,
                    angle=0, length={200, 340}, pressure={0.22, 0.34}})
print("fog evened, glaze laid")

--@ chunk 119
-- the sky back: film, then the zenith, then the glow low down
work(skyM, {hand="body", pile=skySmooth, coverage=2.6, fill=true, clip=true,
            angle=-0.03, length={22, 55}, pressure={0.55, 0.9}})
work(mZen2, {hand="glaze", pile=skyDeep2, coverage=2, fill=true, threshold=0.04,
             clip=true, angle=-0.03, length={150, 290}, pressure={0.45, 0.65}})
work(mWarm, {hand="glaze", pile=glowBody2, coverage=2, fill=true, threshold=0.04,
             clip=true, angle=-0.03, length={150, 290}, pressure={0.42, 0.62}})
work(mGlow2, {hand="glaze", pile=glowHot2, coverage=2, fill=true, threshold=0.04,
              clip=true, angle=-0.03, length={140, 270}, pressure={0.42, 0.62}})
print("sky back")

--@ chunk 120
work(mFar, {hand="body", pile=farP, coverage=2.6, fill=true, clip=true, angle=-0.08,
            length={20, 50}, pressure={0.55, 0.9}})
work(mMid, {hand="body", pile=midDark3, coverage=2.8, fill=true, clip=true, angle=-0.08,
            length={20, 50}, pressure={0.55, 0.9}})
work(mW2, {hand="body", pile=waterEven, coverage=2.6, fill=true, clip=true, angle=0,
           length={25, 60}, pressure={0.5, 0.85}})
work(mNear * (-standing), {hand="glaze", pile=waterDarkP, coverage=2, fill=true,
     threshold=0.03, clip=true, angle=0, length={110, 250}, pressure={0.45, 0.65}})
print("ranges and lake back")

--@ chunk 121
work(mShore, {hand="body", pile=bankQuiet, coverage=2.8, fill=true, clip=true,
              angle=-0.25, length={22, 55}, pressure={0.5, 0.85}})
work(mFore2, {hand="glaze", pile=earthDeep, coverage=1.8, fill=true, threshold=0.03,
              clip=true, angle=-0.25, length={90, 220}, pressure={0.45, 0.68}})
work(mLit, {hand="body", pile=bankQuiet, coverage=1.2, clip=true, angle=-1.2,
            tool="filbert 4", length={12, 32}, pressure={0.35, 0.6}})

-- the fog band and the light on the lake, once more
work(mFogBand, {hand="glaze", pile=fogP, coverage=1.3, fill=true, threshold=0.04,
                clip=true, angle=-0.02, length={140, 280}, pressure={0.32, 0.5}})
work(mFootNarrow, {hand="glaze", pile=fogP, coverage=1.3, fill=true, threshold=0.06,
                   clip=true, angle=0, length={90, 220}, pressure={0.35, 0.55}})
for i = 1, 24 do
  local y = 486 + (i / 24) * 150
  local x0 = rand(-40, 540)
  local L = rand(80, 300) * (1 - 0.3 * ((y - 486) / 150))
  local warm = (x0 + L / 2) > 430 and rand() < 0.6 or rand() < 0.2
  sl:load(warm and streakWarm or streakCool, rand(0.3, 0.6))
  sl:stroke({{x0, y}, {x0 + L * 0.5, y + randn(0, 1.4)}, {x0 + L, y + randn(0, 2)}},
            {pressure = {rand(0.25, 0.42), 0.4, 0.04}, orient = "across"})
end
print("bank, fog, lake light")

--@ chunk 122
-- the incident, last again
work(mAt, {hand="body", pile=treeDark, coverage=2.4, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.6, 0.95}})
work(mA, {hand="body", pile=treeDark, coverage=2.8, fill=true, clip=true,
          tool="filbert 3", length={12, 40}, pressure={0.55, 0.95}})
work(mBt, {hand="body", pile=treeFar, coverage=2.4, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.55, 0.9}})
work(mB, {hand="body", pile=treeFar, coverage=2.6, fill=true, clip=true,
          tool="filbert 3", length={12, 40}, pressure={0.55, 0.9}})
work(mFig2, {hand="body", pile=figP2, coverage=2.6, fill=true, clip=true,
             tool="filbert 3", length={6, 18}, pressure={0.6, 0.95}})
work(hull, {hand="body", pile=boatP, coverage=2.2, fill=true, clip=true,
            tool="filbert 3", length={6, 18}, pressure={0.55, 0.85}})
work(gunwale, {hand="body", pile=gunP, coverage=1.8, clip=true, tool="filbert 2.5",
               length={5, 14}, pressure={0.45, 0.75}})
work(inside, {hand="body", pile=figP2, coverage=1.6, fill=true, clip=true,
              tool="filbert 2.5", length={5, 12}, pressure={0.45, 0.75}})
work(pole, {hand="body", pile=boatP, coverage=1.8, clip=true, tool="filbert 2.5",
            length={6, 16}, pressure={0.5, 0.8}})
work(mFar4R, {hand="body", pile=treeDarkR, coverage=2.2, fill=true, clip=true,
              tool="filbert 2.5", length={5, 14}, pressure={0.5, 0.85}})
work(mFar4L, {hand="body", pile=hazeTree2, coverage=2.2, fill=true, clip=true,
              tool="filbert 2.5", length={5, 14}, pressure={0.5, 0.8}})
print("incident, last")

--@ chunk 123
-- the branches need a heavier pass: they are one to three units wide and the fill dabs
-- were wider than the branch
work(mA, {hand="body", pile=treeDark, coverage=4, fill=true, clip=true,
          tool="filbert 2.5", length={10, 35}, pressure={0.6, 0.95}})
work(mB, {hand="body", pile=treeFar, coverage=3.6, fill=true, clip=true,
          tool="filbert 2.5", length={10, 35}, pressure={0.6, 0.95}})
work(mFig2, {hand="body", pile=figP2, coverage=3.4, fill=true, clip=true,
             tool="filbert 2.5", length={5, 16}, pressure={0.6, 0.95}})
work(gunwale, {hand="body", pile=pile{{"raw umber", 3}, {"yellow ochre", 1.4},
      {"bone black", 0.8}}, coverage=2, fill=true, clip=true, tool="filbert 2.5",
      length={5, 14}, pressure={0.45, 0.75}})

-- the bank again: grass and reeds along its edge
for i = 1, 110 do
  local x = 10 + (i - 1) * 9 + randn(0, 4)
  local by = shoreFn(x) + rand(1, 14)
  local hgt = rand(6, 22)
  rr2:load(rand() < 0.55 and reedP or grassDark, rand(0.35, 0.8))
  rr2:stroke({{x, by}, {x + randn(0, 2), by - hgt * 0.6}, {x + randn(0, 4) + 2, by - hgt}},
             {pressure = {rand(0.45, 0.7), 0.5, 0.08}, orient = "across"})
end
for i = 1, 30 do
  local x = 320 + (i - 1) * 23 + randn(0, 9)
  local by = shoreFn(x) + rand(3, 11)
  for k = 1, 5 do
    local a = -math.pi / 2 + randn(0, 0.5)
    local hgt = rand(8, 19)
    rr2:load(tuftP, rand(0.4, 0.8))
    rr2:stroke({{x + randn(0, 3), by}, {x + math.cos(a) * hgt * 0.6,
                by + math.sin(a) * hgt * 0.6}, {x + math.cos(a) * hgt,
                by + math.sin(a) * hgt}}, {pressure = {0.6, 0.45, 0.05},
                                         orient = "across"})
  end
end
print("branches, figure, boat, grass")

--@ chunk 124
work(ellipse(561, 602, 5.4, 6), {hand="body", pile=figP2, coverage=3.4, fill=true,
      clip=true, tool="filbert 2.5", length={5, 14}, pressure={0.6, 0.95}})
rimFine = pile{{"lead white", 2.6}, {"yellow ochre", 1.4}, {"pale smalt", 1.2}}
mRimF = mFig2:offset(1.3) - mFig2
work(mRimF, {hand="body", pile=rimFine, coverage=0.5, clip=true, tool="filbert 2.5",
             length={4, 12}, pressure={0.3, 0.5}})

bushP = pile{{"bone black", 2.4}, {"raw umber", 2}, {"green earth", 1.2}}
bb = brush{kind = "round", width = 3.2, point = 0.7, stiffness = 0.5}
for i = 1, 34 do
  local x = 20 + rand(0, 960)
  local by = shoreFn(x) + rand(0, 8)
  local r = rand(3, 8)
  for k = 1, 9 do
    local a = rand(0, 6.28)
    local d = r * rand(0.3, 1)
    bb:load(bushP, rand(0.4, 0.85))
    bb:touch(x + math.cos(a) * d, by - d * rand(0.2, 0.9),
             {pressure = rand(0.4, 0.75), twist = rand(-0.5, 0.5)})
  end
end
print("head, rim, bank edge")

--@ chunk 125
-- the rim pass beaded into a halo; take it back to the water's own tone
work(mRimF, {hand="body", pile=waterDarkP, coverage=2.6, fill=true, clip=true,
             tool="filbert 2.5", length={4, 14}, pressure={0.5, 0.8}})
work(ellipse(561, 591, 6, 4.5), {hand="body", pile=waterDarkP, coverage=2.6,
      fill=true, clip=true, tool="filbert 3", length={6, 16}, pressure={0.5, 0.8}})
work(gunwale, {hand="body", pile=pile{{"raw umber", 3.5}, {"yellow ochre", 1},
      {"bone black", 1.3}}, coverage=2.2, fill=true, clip=true, tool="filbert 2.5",
      length={5, 14}, pressure={0.5, 0.8}})
print("halo and gunwale")

--@ chunk 126
work(mFig2:offset(2.8), {hand="body", pile=waterDarkP, coverage=2.6, fill=true,
      clip=true, tool="filbert 3", length={5, 16}, pressure={0.5, 0.8}})
work(mFig2, {hand="body", pile=figP2, coverage=2.6, fill=true, clip=true,
             tool="filbert 2.5", length={5, 16}, pressure={0.6, 0.95}})
work(ellipse(561, 593, 9, 7), {hand="body", pile=waterDarkP, coverage=2.8,
      fill=true, clip=true, tool="filbert 3", length={6, 18}, pressure={0.5, 0.8}})
print("halo cleared")

--@ chunk 127
-- she had gone grey: laid her in properly, at enough coverage to cover what is under
work(mFig2, {hand="body", pile=figP2, coverage=5, fill=true, clip=true,
             tool="filbert 2.5", length={5, 16}, pressure={0.75, 1}})
work(ellipse(561, 602, 5.2, 5.6), {hand="body", pile=figP2, coverage=5, fill=true,
      clip=true, tool="filbert 2", length={4, 12}, pressure={0.75, 1}})
print("figure, opaque")

--@ chunk 128
-- an opaque blob of the old lozenges sits just over her head; it needs body colour,
-- not a thin one, to go
work(ellipse(561, 593, 10, 8), {hand="body", pile=waterDarkP, coverage=6, fill=true,
      clip=true, tool="filbert 3", length={8, 22}, pressure={0.8, 1}})
print("blob covered")

--@ chunk 129
-- it took an opaque pile to cover that cream: the earths and smalts are stains
waterOpaque = pile{{"lead white", 3}, {"pale smalt", 2}, {"smalt", 1.6},
                   {"raw umber", 1.3}}
work(ellipse(557, 594, 9, 7.5), {hand="body", pile=waterOpaque, coverage=4,
      fill=true, clip=true, tool="filbert 3", length={8, 22}, pressure={0.6, 0.95}})
print("covered with body colour")

--@ chunk 130
-- that disc is paint after all; cover a soft patch around it in the lake's own tone,
-- then the woman again on top
waterBody = pile{{"lead white", 1.2}, {"pale smalt", 2}, {"smalt", 1.8},
                 {"raw umber", 1.8}}
mFix = ellipse(557, 598, 30, 24):blur(12)
work(mFix, {hand="body", pile=waterBody, coverage=3, fill=true, clip=true,
            tool="filbert 4", length={10, 26}, pressure={0.55, 0.9}})
work(mFig2, {hand="body", pile=figP2, coverage=5, fill=true, clip=true,
             tool="filbert 2.5", length={5, 16}, pressure={0.75, 1}})
print("patch and figure")

--@ chunk 131
patchP = pile{{"lead white", 0.8}, {"pale smalt", 1.4}, {"smalt", 1.9},
             {"raw umber", 2}, {"bone black", 0.7}}
work(mFix, {hand="body", pile=patchP, coverage=3.5, fill=true, clip=true,
            tool="filbert 4", length={10, 26}, pressure={0.55, 0.9}})
work(mFig2, {hand="body", pile=figP2, coverage=5, fill=true, clip=true,
             tool="filbert 2.5", length={5, 16}, pressure={0.75, 1}})
print("patch toned down")

--@ chunk 132
-- the cream underneath is thick; only a pile with real hiding covers it
patchP2 = pile{{"bone black", 2.2}, {"lead white", 1.6}, {"pale smalt", 1.4},
               {"raw umber", 1}}
work(mFix, {hand="body", pile=patchP2, coverage=4, fill=true, clip=true,
            tool="filbert 4", length={10, 26}, pressure={0.6, 0.95}})
work(mFig2, {hand="body", pile=figP2, coverage=5, fill=true, clip=true,
             tool="filbert 2.5", length={5, 16}, pressure={0.75, 1}})
print("patch with hiding colour")

--@ chunk 133
work(ellipse(556, 594, 7, 6), {hand="body", pile=pile{{"vermilion", 3}},
      coverage=4, fill=true, clip=true, tool="filbert 3", length={8, 20},
      pressure={0.7, 1}})
print("vermilion test")

--@ chunk 134
print("before", drying(556, 594), drying(500, 300), drying(556, 620))
print(wait(3 * 24 * 60))
print("after 3 days", drying(556, 594), drying(500, 300))
print(wait(7 * 24 * 60))
print("after 10 days", drying(556, 594))

--@ chunk 135
print(wait(14 * 24 * 60))
print("after 24 days", drying(556, 594))
print(wait(10 * 24 * 60))
print("after 34 days", drying(556, 594))

--@ chunk 136
-- dry at last: an opaque blue-grey will now cover it
patchP3 = pile{{"bone black", 2.4}, {"lead white", 1.5}, {"pale smalt", 1.5},
               {"raw umber", 1.2}}
work(mFix, {hand="body", pile=patchP3, coverage=3.5, fill=true, clip=true,
            tool="filbert 4", length={10, 26}, pressure={0.55, 0.9}})
work(mFig2, {hand="body", pile=figP2, coverage=4.5, fill=true, clip=true,
             tool="filbert 2.5", length={5, 16}, pressure={0.7, 1}})
print("patched dry")

--@ chunk 137
for i = 1, 3 do
  work(mFix, {hand="body", pile=patchP3, coverage=3, fill=true, clip=true,
              tool="filbert 4", length={10, 26}, pressure={0.55, 0.9}})
end
work(mFig2, {hand="body", pile=figP2, coverage=4.5, fill=true, clip=true,
             tool="filbert 2.5", length={5, 16}, pressure={0.7, 1}})
print("halo covered")

--@ chunk 138
patchP4 = pile{{"bone black", 3.2}, {"lead white", 1}, {"pale smalt", 1.2},
               {"raw umber", 1.4}}
work(mFix, {hand="body", pile=patchP4, coverage=3, fill=true, clip=true,
            tool="filbert 4", length={10, 26}, pressure={0.55, 0.9}})
work(mFig2, {hand="body", pile=figP2, coverage=4.5, fill=true, clip=true,
             tool="filbert 2.5", length={5, 16}, pressure={0.7, 1}})
print("halo toned")

--@ chunk 139
-- the ranges repeated in the lake, broken by the ripples
mRefM = mask(function(x, y)
  local v = clamp((y - 466) / 16, 0, 1) * clamp((606 - y) / 95, 0, 1)
  local h = clamp((440 - x) / 400, 0, 1)
  return clamp(v * h * 1.35, 0, 1)
end) * mW2
mRefP = mask(function(x, y)
  local v = clamp((y - 464) / 14, 0, 1) * clamp((566 - y) / 74, 0, 1)
  local h = clamp(1 - math.abs(x - 700) / 330, 0, 1)
  return clamp(v * h * 0.95, 0, 1)
end) * mW2
work(mRefM, {hand="glaze", pile=midDark3, coverage=1.4, fill=true, threshold=0.05,
             clip=true, angle=0.02, length={100, 240}, pressure={0.4, 0.6}})
work(mRefP, {hand="glaze", pile=pile{{"lead white", 3}, {"yellow ochre", 1},
      {"pale smalt", 2}}, coverage=1.2, fill=true, threshold=0.05, clip=true,
      angle=0.02, length={100, 240}, pressure={0.35, 0.55}})

-- the halo behind her, a last time, a little wider and darker
mFix3 = ellipse(557, 598, 34, 27):blur(14)
work(mFix3, {hand="body", pile=patchP4, coverage=2.4, fill=true, clip=true,
             tool="filbert 4", length={10, 26}, pressure={0.5, 0.85}})
work(mFig2, {hand="body", pile=figP2, coverage=4.5, fill=true, clip=true,
             tool="filbert 2.5", length={5, 16}, pressure={0.7, 1}})
print("reflections, halo")

--@ chunk 140
-- the glow's reflection was reading as a second sun: knock it back to a sheen
work(mRefP, {hand="glaze", pile=waterEven, coverage=1.8, fill=true, threshold=0.04,
             clip=true, angle=0.02, length={110, 250}, pressure={0.35, 0.55}})
work(mRefP, {hand="glaze", pile=pile{{"lead white", 2.4}, {"yellow ochre", 1.2},
      {"pale smalt", 2.2}}, coverage=0.7, threshold=0.07, clip=true, angle=0.02,
      length={110, 250}, pressure={0.3, 0.45}})

-- the lake around her, laid again with the lake's own recipe
mFixBig = ellipse(560, 600, 130, 86):blur(22)
work(mFixBig, {hand="body", pile=waterEven, coverage=2.4, fill=true, clip=true,
               tool="filbert 4", length={10, 26}, pressure={0.5, 0.85}})
mFixBigDeep = mFixBig * mask(function(x, y)
  return clamp((y - 556) / 90, 0, 1)
end)
work(mFixBigDeep, {hand="glaze", pile=waterDarkP, coverage=1.8, fill=true,
                   threshold=0.04, clip=true, angle=0, length={110, 250},
                   pressure={0.4, 0.6}})
work(mFig2, {hand="body", pile=figP2, coverage=4.5, fill=true, clip=true,
             tool="filbert 2.5", length={5, 16}, pressure={0.7, 1}})
print("reflection toned, lake relaid")

--@ chunk 141
work(mShore, {hand="body", pile=bankQuiet, coverage=2.6, fill=true, clip=true,
              angle=-0.25, length={20, 50}, pressure={0.5, 0.85}})
work(mFore2, {hand="glaze", pile=earthDeep, coverage=1.6, fill=true, threshold=0.03,
              clip=true, angle=-0.25, length={90, 220}, pressure={0.45, 0.68}})
for i = 1, 46 do
  local x = 300 + (i - 1) * 9.5 + randn(0, 3)
  local by = shoreFn(x) + rand(1, 12)
  local hgt = rand(6, 21)
  rr2:load(rand() < 0.55 and reedP or grassDark, rand(0.35, 0.8))
  rr2:stroke({{x, by}, {x + randn(0, 2), by - hgt * 0.6}, {x + randn(0, 3) + 2, by - hgt}},
             {pressure = {rand(0.45, 0.7), 0.5, 0.08}, orient = "across"})
end
for i = 1, 12 do
  local x = 330 + (i - 1) * 28 + randn(0, 9)
  local by = shoreFn(x) + rand(3, 10)
  for k = 1, 5 do
    local a = -math.pi / 2 + randn(0, 0.5)
    local hgt = rand(8, 18)
    rr2:load(tuftP, rand(0.4, 0.8))
    rr2:stroke({{x + randn(0, 3), by}, {x + math.cos(a) * hgt * 0.6,
                by + math.sin(a) * hgt * 0.6}, {x + math.cos(a) * hgt,
                by + math.sin(a) * hgt}}, {pressure = {0.6, 0.45, 0.05},
                                         orient = "across"})
  end
end
work(mFig2, {hand="body", pile=figP2, coverage=4.5, fill=true, clip=true,
             tool="filbert 2.5", length={5, 16}, pressure={0.7, 1}})
work(hull, {hand="body", pile=boatP, coverage=2.2, fill=true, clip=true,
            tool="filbert 3", length={6, 18}, pressure={0.55, 0.85}})
print("bank, grass, figure")

--@ chunk 142
-- birds once more, and the last of the light on the grass at the water's edge
birdP = pile{{"raw umber", 2.4}, {"bone black", 1.8}}
br = brush{kind = "round", width = 1.4, point = 1, stiffness = 0.4}
for _, b in ipairs({{486, 108, 6}, {516, 92, 5}, {541, 116, 4.2}, {566, 99, 5.2},
                    {604, 126, 4.4}}) do
  local x, y, w = b[1], b[2], b[3]
  br:load(birdP, rand(0.5, 0.85))
  br:stroke({{x - w, y + w * 0.35}, {x - w * 0.3, y - w * 0.25}, {x, y}},
            {pressure = {0.6, 0.5, 0.1}, orient = "across"})
  br:stroke({{x, y}, {x + w * 0.35, y - w * 0.3}, {x + w, y + w * 0.4}},
            {pressure = {0.1, 0.5, 0.6}, orient = "across"})
end
gr = brush{kind = "round", width = 1.4, point = 1, stiffness = 0.4}
tipP = pile{{"yellow ochre", 2.2}, {"lead white", 1.4}}
for i = 1, 70 do
  local x = 20 + rand(0, 950)
  local by = shoreFn(x) + rand(2, 22)
  gr:load(tipP, rand(0.25, 0.5))
  gr:stroke({{x, by}, {x + randn(0, 1.5), by - rand(5, 15)}},
            {pressure = {rand(0.4, 0.65), 0.1}, orient = "across"})
end
print("birds and grass light")

--@ chunk 143
-- the grass tips came out like gold flecks: back to the bank's own colour, then a
-- quieter highlight
mEdgeBand = mask(function(x, y)
  local d = y - shoreFn(x)
  return clamp((d + 12) / 10, 0, 1) * clamp((46 - d) / 26, 0, 1) * 1.3
end) * mShore
work(mEdgeBand, {hand="body", pile=bankQuiet, coverage=3, fill=true, clip=true,
                 angle=-0.5, length={15, 40}, pressure={0.5, 0.85}})
for i = 1, 120 do
  local x = 10 + (i - 1) * 8.2 + randn(0, 4)
  local by = shoreFn(x) + rand(2, 30)
  local hgt = rand(6, 20)
  rr2:load(rand() < 0.5 and reedP or grassDark, rand(0.3, 0.65))
  rr2:stroke({{x, by}, {x + randn(0, 2), by - hgt * 0.6}, {x + randn(0, 3) + 2, by - hgt}},
             {pressure = {rand(0.4, 0.6), 0.45, 0.08}, orient = "across"})
end
tipP2 = pile{{"yellow ochre", 1.6}, {"lead white", 1.2}, {"raw umber", 0.6}}
for i = 1, 40 do
  local x = 40 + rand(0, 900)
  local by = shoreFn(x) + rand(3, 16)
  gr:load(tipP2, rand(0.2, 0.4))
  gr:stroke({{x, by}, {x + randn(0, 1.5), by - rand(4, 11)}},
            {pressure = {rand(0.3, 0.45), 0.08}, orient = "across"})
end
work(mFig2, {hand="body", pile=figP2, coverage=4.5, fill=true, clip=true,
             tool="filbert 2.5", length={5, 16}, pressure={0.7, 1}})
print("grass quieted")

--@ chunk 144
work(mEdgeBand, {hand="body", pile=bankQuiet, coverage=3.4, fill=true, clip=true,
                 angle=-0.5, length={15, 40}, pressure={0.5, 0.85}})
for i = 1, 150 do
  local x = 10 + (i - 1) * 6.6 + randn(0, 4)
  local by = shoreFn(x) + rand(2, 34)
  local hgt = rand(5, 18)
  rr2:load(rand() < 0.5 and reedP or grassDark, rand(0.3, 0.6))
  rr2:stroke({{x, by}, {x + randn(0, 2), by - hgt * 0.6}, {x + randn(0, 3) + 2, by - hgt}},
             {pressure = {rand(0.35, 0.55), 0.4, 0.08}, orient = "across"})
end
work(mFig2, {hand="body", pile=figP2, coverage=4.5, fill=true, clip=true,
             tool="filbert 2.5", length={5, 16}, pressure={0.7, 1}})
print("edge band, dark grass")

--@ chunk 145
print(drying(150, 200), drying(100, 500), drying(560, 620))
print(wait(720))
print(drying(150, 200), drying(100, 500), drying(560, 620))

--@ chunk 146
-- the last light catching the upper edges of the branches, in the sky where the paint
-- has set: a small warm touch beside each node, on the side the sun went down
rimB = brush{kind = "rigger", width = 1.3, point = 0.9, stiffness = 0.4}
edgeLight = pile{{"yellow ochre", 2.4}, {"lead white", 2}, {"vermilion", 0.5}}
local cnt = 0
for _, list in ipairs({branches, crown}) do
  for _, b in ipairs(list) do
    local n = #b.pts
    for i = 1, n do
      local x, y = b.pts[i][1], b.pts[i][2]
      if y < 392 and y > 2 and rand() < 0.75 then
        local t = (i - 1) / (n - 1)
        local w = b.w0 + (b.w1 - b.w0) * t
        rimB:load(edgeLight, rand(0.3, 0.65))
        rimB:touch(x + w * 0.42 + randn(0, 0.5), y - w * 0.42 + randn(0, 0.5),
                   {pressure = rand(0.25, 0.5), twist = rand(-0.4, 0.4)})
        cnt = cnt + 1
      end
    end
  end
end
print("edge lights", cnt)

--@ chunk 147
print("drying water 400,520: ", drying(400,520))
print("drying water 700,480: ", drying(700,480))
print("drying tree 120,300: ", drying(120,300))
print("drying sky 600,150: ", drying(600,150))
print("drying bank 500,700: ", drying(500,700))

--@ chunk 148
local L = {}
-- pull the tail of the painting log out of a global? not available; print instead
print("no")

--@ chunk 149
print("H", H, "W", W)
print("water", mWater:at(500,500), mWater:at(500,600), mWater:at(500,660), mWater:at(500,450))
print("w2", mW2:at(500,500), mW2:at(500,620), mW2:area())
print("shoreFn", shoreFn(100), shoreFn(500), shoreFn(900))
print("standing", standing:at(500,300))
print("water areas", mWater:area(), mW2:area(), mShore:area())
print("crown n", #crown, "branches n", #branches)
print(crown[1].pts[1][1], crown[1].pts[1][2], crown[1].w0, crown[1].w1)
print(branches[1].pts[1][1], branches[1].pts[1][2], branches[1].w0, branches[1].w1)

--@ chunk 150
mOpen = mWater * mask(function(x, y)
  return clamp((y - 450) / 9, 0, 1) * clamp((shoreFn(x) - y - 7) / 12, 0, 1)
end) * (-mFig2:grow(4))
print("open lake area", mOpen:area())
lakeTone = pile{{"lead white", 1.4}, {"pale smalt", 2.2}, {"smalt", 1.8},
                {"raw umber", 2}}
work(mOpen, {hand="body", pile=lakeTone, coverage=2.4, fill=true, clip=true,
             angle=0.02, tool="filbert 6", length={90, 210},
             pressure={0.5, 0.8}})
print("lake laid even")

--@ chunk 151
print("wet:", drying(400, 520), drying(800, 550))
print(wait(2 * 24 * 60))
print("after 2 days:", drying(400, 520), drying(800, 550), drying(300, 600))

--@ chunk 152
lakeDark = pile{{"lead white", 0.8}, {"pale smalt", 1.6}, {"smalt", 1.6},
                {"raw umber", 2.6}, {"bone black", 1.2}}
work(mOpen, {hand="glaze", pile=lakeDark, coverage=2.6, fill=true,
             threshold=0.02, clip=true, angle=0.02, length={90, 220},
             pressure={0.45, 0.65}})
print("lake darkened")

--@ chunk 153
-- the lake went muddy: take it back towards a cool blue-grey, cooler as it comes nearer
mCool = mOpen * mask(function(x, y) return clamp((y - 456) / 10, 0, 1) end)
lakeCool = pile{{"lead white", 1.1}, {"pale smalt", 2.8}, {"smalt", 2.2},
                {"raw umber", 1}, {"Prussian blue", 0.4}}
work(mCool, {hand="glaze", pile=lakeCool, coverage=2.3, fill=true,
             threshold=0.02, clip=true, angle=0.02, length={90, 220},
             pressure={0.45, 0.65}})
print("lake cooled")

--@ chunk 154
-- clean slate: one even body pass over the whole open lake, no hole for her
mOpen2 = mWater * mask(function(x, y)
  return clamp((y - 450) / 9, 0, 1) * clamp((shoreFn(x) - y - 7) / 12, 0, 1)
end)
lakeBase = pile{{"lead white", 1.2}, {"pale smalt", 2.6}, {"smalt", 2.2},
                {"raw umber", 1.8}, {"bone black", 0.6}}
work(mOpen2, {hand="body", pile=lakeBase, coverage=2.5, fill=true, clip=true,
              angle=0.02, tool="filbert 7", length={60, 170},
              pressure={0.5, 0.8}})
print("lake re-laid, area", mOpen2:area())

--@ chunk 155
lim = mask(function(x, y)
  return clamp((shoreFn(x) - y - 7) / 12, 0, 1) * clamp((y - 450) / 9, 0, 1)
end)
local function z(lo, hi, r)
  return mask(function(x, y)
    return clamp((y - lo) / r, 0, 1) * clamp((hi - y) / r, 0, 1)
  end)
end
zFar = lim * z(0, 514, 26) * mWater
zMid = lim * z(490, 592, 28) * mWater
zNear = lim * z(568, 900, 32) * mWater
pFar = pile{{"lead white", 2}, {"pale smalt", 1.8}, {"smalt", 1.4}, {"raw umber", 1.6}}
pMid = pile{{"lead white", 1.2}, {"pale smalt", 1.6}, {"smalt", 1.5}, {"raw umber", 2.4},
            {"bone black", 0.8}}
pNear = pile{{"lead white", 0.8}, {"pale smalt", 1.2}, {"smalt", 1.2}, {"raw umber", 2.8},
             {"bone black", 1.4}}
for _, p in ipairs({pFar, pMid, pNear}) do end
work(zFar, {hand="body", pile=pFar, coverage=2.2, fill=true, clip=true,
            angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.5, 0.8}})
work(zMid, {hand="body", pile=pMid, coverage=2.2, fill=true, clip=true,
            angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.5, 0.8}})
work(zNear, {hand="body", pile=pNear, coverage=2.2, fill=true, clip=true,
             angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.5, 0.8}})
print("lake in three bands")

--@ chunk 156
print(drying(500, 500), drying(500, 560), drying(300, 620), drying(800, 520))

--@ chunk 157
blend(mOpen2, {angle=0.03})
print("lake blended while open; drying:", drying(500, 560))

--@ chunk 158
print(wait(2 * 24 * 60))
-- the far shore wants its mist back: a band lying on the water, thickest at the left
nzMist = noise{seed=91, period=110, octaves=3}
mMist = rect(0, 432, 1000, 502) * mask(function(x, y)
  local d = y - 447
  local thick = 30 + 18 * nzMist:at01(x, 20) - 10 * x / 1000
  local e = thick + 9 * (nzMist:at01(x, 200) - 0.5)
  return clamp((d + 8) / 8, 0, 1) * clamp((e - d) / 13, 0, 1)
end)
print("mist band area", mMist:area())
mistP = pile{{"lead white", 3.4}, {"pale smalt", 2}, {"raw umber", 0.7}}
work(mMist, {hand="glaze", pile=mistP, coverage=2.2, fill=true, threshold=0.03,
             clip=true, angle=0.02, length={70, 180}, pressure={0.4, 0.6}})
print("mist back on the water")

--@ chunk 159
-- the mist is a white bar: warm it, drop its value, and let it veil the foot of the hill
mistWarm = pile{{"lead white", 1.5}, {"pale smalt", 1.2}, {"raw umber", 1.7}}
mMistUp = rect(0, 430, 1000, 500) * mask(function(x, y)
  return clamp((452 - y) / 12, 0, 1) * (0.55 + 0.45 * x / 1000)
end)
work(mMistUp, {hand="glaze", pile=mistWarm, coverage=1.5, threshold=0.03,
               clip=true, angle=0.02, length={60, 150}, pressure={0.4, 0.6}})
work(mMist, {hand="glaze", pile=mistP, coverage=0.55, threshold=0.06,
             clip=true, angle=0.02, length={60, 150}, pressure={0.3, 0.5}})
-- and a little mist climbing the dark hill at the left, so its foot is lost
mFootVeil = rect(0, 418, 420, 462) * mask(function(x, y)
  return clamp((462 - y) / 10, 0, 1) * clamp((y - 420) / 8, 0, 1) * (1 - 0.4 * x / 420)
end)
work(mFootVeil, {hand="glaze", pile=mistP, coverage=1.5, threshold=0.04,
                 clip=true, angle=0.05, length={50, 130}, pressure={0.35, 0.55}})
print("mist tempered")

--@ chunk 160
-- the band is still a bar: let the water show through it in three places
mBack = (ellipse(930, 461, 92, 20):blur(20) + ellipse(524, 458, 44, 15):blur(15)
         + ellipse(714, 470, 38, 12):blur(13)) * mMist * mWater
work(mBack, {hand="glaze", pile=pFar, coverage=1.9, threshold=0.03, clip=true,
             angle=0.02, length={60, 150}, pressure={0.4, 0.6}})
-- and mist over the foot of the far shore at the right too
mFootVeil2 = rect(560, 418, 440, 458) * mask(function(x, y)
  return clamp((458 - y) / 9, 0, 1) * (0.5 + 0.4 * x / 1000)
end)
work(mFootVeil2, {hand="glaze", pile=mistP, coverage=1.3, threshold=0.04,
                  clip=true, angle=0.05, length={50, 130}, pressure={0.35, 0.55}})
-- and a wisp or two lying further out on the water
for _, w in ipairs({{640, 486, 150, 5}, {812, 494, 110, 4}, {430, 482, 90, 4}}) do
  work(ellipse(w[1], w[2], w[3], w[4]):blur(9), {hand="glaze", pile=mistP,
        coverage=1.3, threshold=0.05, clip=true, angle=0.02,
        length={40, 110}, pressure={0.35, 0.55}})
end
print("mist broken")

--@ chunk 161
-- too much white: the band comes down to a luminous grey, a hint of warmth in it
mMistBand = rect(0, 426, 1000, 508) * mask(function(x, y)
  local d = y - 438
  local thick = 26 + 12 * nzMist:at01(x, 60) - 6 * x / 1000
  local e = thick + 8 * (nzMist:at01(x, 320) - 0.5)
  return clamp((d + 5) / 7, 0, 1) * clamp((e - d) / 14, 0, 1)
end)
mistDown = pile{{"lead white", 1.6}, {"pale smalt", 2}, {"raw umber", 1.6},
                {"vermilion", 0.15}}
work(mMistBand, {hand="body", pile=mistDown, coverage=2.6, fill=true,
                 threshold=0.03, clip=true, angle=0.02, tool="filbert 6",
                 length={60, 150}, pressure={0.5, 0.78}})
print("band tempered; area", mMistBand:area())

--@ chunk 162
-- those straight pale bars on the hills are the edges of my veil rectangles: re-lay the feet
mHillFootL = rect(0, 394, 432, 442):blur(16)
work(mHillFootL, {hand="body", pile=midDark, coverage=2.2, fill=true,
                  threshold=0.02, clip=true, angle=0.08, tool="filbert 6",
                  length={40, 110}, pressure={0.5, 0.8}})
mHillFootR = rect(566, 396, 434, 446):blur(18)
work(mHillFootR, {hand="body", pile=crestP, coverage=2.2, fill=true,
                  threshold=0.02, clip=true, angle=0.08, tool="filbert 6",
                  length={40, 110}, pressure={0.5, 0.8}})
print("hill feet re-laid")

--@ chunk 163
print("footL area", mHillFootL:area(), "expect ~", 432 * 48)
print("footR area", mHillFootR:area())
print("at 200,500", mHillFootL:at(200, 500), "at 200,420", mHillFootL:at(200, 420))
print("at 700,600", mHillFootR:at(700, 600), "at 700,420", mHillFootR:at(700, 420))
print("at 200,430", mHillFootL:at(200, 430), "at 700,430", mHillFootR:at(700, 430))
print("drying 200,500", drying(200, 500))

--@ chunk 164
t = rect(0, 394, 432, 48)
print("plain rect area", t:area(), t:at(200, 400), t:at(200, 450))
t2 = t:blur(16)
print("blurred area", t2:area(), t2:at(200, 400), t2:at(200, 450), t2:at(200, 500), t2:at(200, 780))
t3 = t:soften(4)
print("softened area", t3:area(), t3:at(200, 450), t3:at(200, 500))
t4 = rect(0, 394, 432, 442)
print("tall rect area", t4:area(), t4:at(200, 600))

--@ chunk 165
-- nothing to grep from here; repair the lake with masks built from functions instead
print("lim at 200,600", lim:at(200, 600), "at 200,460", lim:at(200, 460))
print("zFar", zFar:at(500, 470), "zMid", zMid:at(500, 540), "zNear", zNear:at(500, 630))
print("drying", drying(200, 600), drying(500, 500))

--@ chunk 166
print(wait(3 * 24 * 60))
print("dry?", drying(200, 600), drying(700, 600), drying(500, 500))
-- lay the lake again over the flood
work(zFar, {hand="body", pile=pFar, coverage=2.4, fill=true, clip=true,
            angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.5, 0.8}})
work(zMid, {hand="body", pile=pMid, coverage=2.4, fill=true, clip=true,
            angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.5, 0.8}})
work(zNear, {hand="body", pile=pNear, coverage=2.4, fill=true, clip=true,
             angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.5, 0.8}})
print("lake re-laid")

--@ chunk 167
blend(mOpen2, {angle=0.03})
print("blended")

--@ chunk 168
for _, m in ipairs({mShore, mWater, mFringe, mFore2, mFootNarrow, mEdgeBand, mLit,
                     mAt, mA, mB, mBt, mFig2, mCrestM, mWood}) do
  print(string.format("area %10.0f", m:area()))
end
print("mShore at 500,700", mShore:at(500,700), "at 500,640", mShore:at(500,640))
print("mEdgeBand at 500,670", mEdgeBand:at(500,670))
print("mFringe at 500,660", mFringe:at(500,660))
print("mWood at 560,660", mWood:at(560,660))
print("mCrestM at 700,400", mCrestM:at(700,400))
print("mAt at 200,300", mAt:at(200,300), " mA at 200,300", mA:at(200,300))
print("mB at 120,200", mB:at(120,200), " mBt at 120,200", mBt:at(120,200))

--@ chunk 169
print("bank", drying(300, 700), "trees", drying(150, 250), "fig", drying(560, 600))
print(wait(2 * 24 * 60))
print("now", drying(300, 700), drying(150, 250))
-- the flood took the trees, the bank and her: lay them again
work(mAt, {hand="body", pile=treeDark, coverage=2.6, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.5, 0.85}})
work(mA, {hand="body", pile=treeDark, coverage=2.6, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.75}})
work(mBt, {hand="body", pile=treeFar, coverage=2.6, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.45, 0.8}})
work(mB, {hand="body", pile=treeFar, coverage=2.5, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.7}})
work(mShore, {hand="body", pile=bankQuiet, coverage=2.8, fill=true, clip=true,
              angle=-0.25, length={22, 55}, pressure={0.5, 0.85}})
work(mFore2, {hand="glaze", pile=earthDeep, coverage=1.7, fill=true,
              threshold=0.03, clip=true, angle=-0.25, length={90, 220},
              pressure={0.45, 0.68}})
work(mFringe, {hand="body", pile=edgeP, coverage=1.8, threshold=0.03, clip=true,
               angle=-1.15, tool="filbert 4", length={12, 34}, pressure={0.35, 0.65}})
work(mFringe, {hand="body", pile=grassDark, coverage=1.4, threshold=0.03, clip=true,
               angle=-1.25, tool="rigger 1.6", length={10, 30}, pressure={0.4, 0.7}})
work(mFig2, {hand="body", pile=figP2, coverage=5, fill=true, clip=true,
             tool="filbert 2.5", length={5, 16}, pressure={0.75, 1}})
print("trees, bank, fringe, figure")

--@ chunk 170
print("lake dry?", drying(300, 550), drying(700, 600))
print(wait(2 * 24 * 60))
lim2 = mask(function(x, y)
  return clamp((shoreFn(x) - y - 1) / 6, 0, 1) * clamp((y - 450) / 9, 0, 1)
end)
mLake = lim2 * mWater
mDeepZ = mLake * mask(function(x, y) return clamp((y - 536) / 46, 0, 1) end)
lakeCoolP = pile{{"lead white", 1}, {"pale smalt", 2.2}, {"smalt", 2},
                 {"raw umber", 1.8}}
lakeDeepP = pile{{"lead white", 0.7}, {"pale smalt", 1.6}, {"smalt", 1.8},
                 {"raw umber", 2.4}, {"bone black", 1}}
work(mLake, {hand="body", pile=lakeCoolP, coverage=2.2, fill=true, clip=true,
             angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.5, 0.8}})
work(mDeepZ, {hand="body", pile=lakeDeepP, coverage=1.8, fill=true, clip=true,
              angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.5, 0.8}})
blend(mLake, {angle=0.03})
print("lake cooled and deepened")

--@ chunk 171
print(wait(2 * 24 * 60))
print("dry?", drying(500, 550))
-- the white line the flood left along the bank: water colour over it
mEdgeWhite = mLake * mask(function(x, y)
  local d = shoreFn(x) - y
  return clamp((d - 13) / 6, 0, 1) * clamp(d / 6, 0, 1)
end)
work(mEdgeWhite, {hand="body", pile=lakeDeepP, coverage=2, fill=true, clip=true,
                  angle=0.02, tool="filbert 3", length={20, 60},
                  pressure={0.5, 0.8}})
print("edge cleared")

--@ chunk 172
paleLight = pile{{"lead white", 4}, {"pale smalt", 1.6}, {"yellow ochre", 0.5}}
warmLight = pile{{"lead white", 3.4}, {"yellow ochre", 1.8}, {"vermilion", 0.35},
                 {"pale smalt", 1}}
darkRip = pile{{"lead white", 0.6}, {"pale smalt", 1.4}, {"smalt", 1.2},
               {"raw umber", 2.2}, {"bone black", 0.8}}
function rip(x0, x1, y0, y1, n, lmin, lmax, pile, pmin, pmax, wd)
  local b = brush{kind = "rigger", width = wd, point = 0.85, stiffness = 0.45}
  for i = 1, n do
    local x = rand(x0, x1)
    local y = rand(y0, y1)
    local L = rand(lmin, lmax)
    local p = rand(pmin, pmax)
    local w = randn(0, 0.5)
    b:load(pile, rand(0.3, 0.7))
    b:stroke({{x, y}, {x + L * 0.33, y + w}, {x + L * 0.66, y - w * 0.8},
              {x + L, y}},
             {pressure = {0.1, p, p * 0.85, 0.06}, orient = "across",
              clip = mLake})
  end
end
-- under the dark bank its reflection is dark; under the pale peak it is light
rip(0, 455, 456, 505, 30, 26, 95, darkRip, 0.3, 0.55, 1.3)
rip(470, 1000, 455, 512, 46, 26, 120, paleLight, 0.3, 0.6, 1.3)
rip(560, 900, 456, 495, 20, 40, 130, paleLight, 0.45, 0.7, 1.5)
-- the glow's path down the water, and the cool light at the left
rip(540, 1000, 500, 600, 34, 46, 165, warmLight, 0.28, 0.5, 1.5)
rip(0, 380, 478, 580, 28, 36, 130, paleLight, 0.24, 0.44, 1.5)
-- drags of dark between them, to give the surface its grain
rip(150, 900, 515, 610, 26, 60, 190, darkRip, 0.28, 0.48, 1.7)
-- near water: long and sparse
rip(60, 960, 595, 660, 26, 90, 240, warmLight, 0.26, 0.46, 2.1)
rip(40, 960, 615, 672, 12, 150, 330, darkRip, 0.3, 0.5, 2.3)
print("ripples drawn")

--@ chunk 173
print(wait(2 * 24 * 60))
print("dry?", drying(500, 550), drying(800, 500))
-- a veil of the lake's own colour to put the combed lines back into the surface
lakeCalm = pile{{"lead white", 1.1}, {"pale smalt", 2.1}, {"smalt", 1.9},
                {"raw umber", 2}}
work(mLake, {hand="body", pile=lakeCalm, coverage=1.3, fill=true, clip=true,
             angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.45, 0.72}})
print("calmed")

--@ chunk 174
mistLay = pile{{"lead white", 2.2}, {"pale smalt", 1.6}, {"raw umber", 1.1},
                {"vermilion", 0.12}}
-- first a thin veil over the foot of the far shore, thickest at the left
mFootV = mask(function(x, y)
  local d = y - 424
  return clamp((d + 7) / 9, 0, 1) * clamp((453 - y) / 11, 0, 1)
         * (0.42 + 0.52 * (1 - x / 1000))
end)
work(mFootV, {hand="body", pile=mistLay, coverage=1.2, threshold=0.04, clip=true,
              angle=0.05, tool="filbert 5", length={50, 130},
              pressure={0.4, 0.7}})
-- then the band lying on the water
mMist2 = mask(function(x, y)
  local d = y - 446
  local thick = 21 + 14 * nzMist:at01(x, 20) - 8 * x / 1000
  local e = thick + 7 * (nzMist:at01(x, 260) - 0.5)
  return clamp((d + 6) / 7, 0, 1) * clamp((e - d) / 12, 0, 1)
end)
print("band area", mMist2:area())
work(mMist2, {hand="body", pile=mistLay, coverage=2.2, fill=true, threshold=0.04,
              clip=true, angle=0.02, tool="filbert 6", length={50, 130},
              pressure={0.45, 0.75}})
blend(mMist2, {angle=0.03})
print("mist laid")

--@ chunk 175
print(wait(2 * 24 * 60))
print("dry?", drying(600, 460), drying(200, 455))
mistCool = pile{{"lead white", 1.2}, {"pale smalt", 2}, {"smalt", 0.8},
                {"raw umber", 1.6}, {"vermilion", 0.05}}
work(mMist2, {hand="body", pile=mistCool, coverage=1.5, threshold=0.04, clip=true,
              angle=0.02, tool="filbert 6", length={50, 130}, pressure={0.4, 0.7}})
work(mFootV, {hand="body", pile=mistCool, coverage=1, threshold=0.05, clip=true,
              angle=0.05, tool="filbert 5", length={40, 110}, pressure={0.35, 0.6}})
blend(mMist2, {angle=0.03})
print("mist tempered")

--@ chunk 176
print(wait(2 * 24 * 60))
lakeD2 = pile{{"lead white", 0.7}, {"pale smalt", 1.6}, {"smalt", 1.8},
              {"raw umber", 2.4}, {"bone black", 1}}
lakeD3 = pile{{"lead white", 0.5}, {"pale smalt", 1.2}, {"smalt", 1.4},
              {"raw umber", 2.6}, {"bone black", 1.6}}
mLow = mLake * mask(function(x, y) return clamp((y - 520) / 60, 0, 1) end)
work(mLake, {hand="body", pile=lakeD2, coverage=1.35, fill=true, clip=true,
             angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.45, 0.72}})
work(mLow, {hand="body", pile=lakeD3, coverage=1.25, fill=true, clip=true,
            angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.45, 0.72}})
print("lake down a step")

--@ chunk 177
print(wait(2 * 24 * 60))
print("dry?", drying(500, 550), drying(700, 500))
paleLight = pile{{"lead white", 4}, {"pale smalt", 1.6}, {"yellow ochre", 0.5}}
warmLight = pile{{"lead white", 3.4}, {"yellow ochre", 1.8}, {"vermilion", 0.35},
                 {"pale smalt", 1}}
darkRip = pile{{"lead white", 0.5}, {"pale smalt", 1.2}, {"smalt", 1.3},
               {"raw umber", 2.4}, {"bone black", 1.2}}
-- the trees repeated in the water, and the ripples breaking them
vb = brush{kind = "rigger", width = 2.4, point = 0.6, stiffness = 0.4}
for _, tx in ipairs({104, 118, 246, 262}) do
  for i = 1, 22 do
    local y0 = 456 + i * 7.4 + randn(0, 3)
    local len = rand(9, 26) + i * 0.7
    local x = tx + randn(0, 5 + i * 0.5)
    vb:load(darkRip, rand(0.25, 0.55) * (1 - i / 30))
    vb:stroke({{x, y0}, {x + randn(0, 2.5), y0 + len * 0.55}, {x + randn(0, 4), y0 + len}},
              {pressure = {0.5, 0.35, 0.03}, orient = "across", clip = mLake})
  end
end
-- then the light: gathered in clusters, with quiet water between
function cluster(cx, cy, sx, sy, n, lmin, lmax, pile, pmin, pmax, wd)
  local b = brush{kind = "rigger", width = wd, point = 0.85, stiffness = 0.45}
  for i = 1, n do
    local x = cx + randn(0, sx)
    local y = cy + randn(0, sy)
    local L = rand(lmin, lmax)
    local p = rand(pmin, pmax)
    local w = randn(0, 0.5)
    b:load(pile, rand(0.3, 0.7))
    b:stroke({{x, y}, {x + L * 0.33, y + w}, {x + L * 0.66, y - w * 0.8}, {x + L, y}},
             {pressure = {0.1, p, p * 0.85, 0.06}, orient = "across", clip = mLake})
  end
end
cluster(560, 462, 180, 5, 11, 30, 120, paleLight, 0.5, 0.75, 1.4)
cluster(860, 463, 140, 5, 9, 30, 110, paleLight, 0.45, 0.7, 1.4)
cluster(150, 467, 130, 6, 9, 25, 100, paleLight, 0.4, 0.6, 1.4)
cluster(300, 476, 90, 5, 6, 25, 90, paleLight, 0.35, 0.55, 1.3)
cluster(140, 502, 110, 12, 7, 30, 120, paleLight, 0.28, 0.45, 1.5)
cluster(255, 532, 90, 10, 5, 40, 140, paleLight, 0.22, 0.36, 1.6)
cluster(620, 500, 70, 8, 6, 25, 90, warmLight, 0.3, 0.5, 1.4)
cluster(700, 494, 130, 9, 9, 30, 110, warmLight, 0.4, 0.62, 1.5)
cluster(760, 522, 120, 8, 8, 40, 130, warmLight, 0.35, 0.55, 1.6)
cluster(690, 550, 140, 9, 7, 50, 160, warmLight, 0.3, 0.48, 1.7)
cluster(830, 574, 110, 8, 6, 60, 190, warmLight, 0.26, 0.42, 1.9)
-- quiet water: a few long drags and a breath of light near the bank
cluster(450, 540, 200, 14, 7, 90, 260, darkRip, 0.22, 0.36, 2.2)
cluster(760, 506, 150, 8, 5, 60, 200, darkRip, 0.2, 0.34, 1.8)
cluster(600, 632, 300, 10, 5, 120, 300, paleLight, 0.18, 0.3, 2.4)
cluster(210, 622, 190, 10, 4, 100, 250, warmLight, 0.16, 0.26, 2.2)
print("light laid")

--@ chunk 178
print(wait(2 * 24 * 60))
print("dry?", drying(500, 550))
work(mLake, {hand="body", pile=lakeD2, coverage=0.85, fill=true, clip=true,
             angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.4, 0.66}})
print("light knocked back")

--@ chunk 179
blend(mLake, {angle=0.03})
print("blended; drying:", drying(600, 550))

--@ chunk 180
print(wait(2 * 24 * 60))
print("dry?", drying(600, 550))
shoreP = pile{{"lead white", 4}, {"pale smalt", 2}, {"yellow ochre", 0.5}}
glowP = pile{{"lead white", 3}, {"pale smalt", 2.4}, {"yellow ochre", 0.9}}
leftP = pile{{"lead white", 3}, {"pale smalt", 2.8}, {"yellow ochre", 0.3}}
mShoreLight = mLake * mask(function(x, y)
  local d = y - 452
  return clamp((d - 4) / 6, 0, 1) * clamp((31 - d) / 11, 0, 1)
end)
mGlow = mLake * mask(function(x, y)
  local h = clamp(1 - math.abs(x - 770) / 310, 0, 1)
  local v = clamp((y - 464) / 26, 0, 1) * clamp((606 - y) / 95, 0, 1)
  return v * h
end)
mLeftLight = mLake * mask(function(x, y)
  local h = clamp(1 - x / 430, 0, 1)
  local v = clamp((y - 460) / 22, 0, 1) * clamp((566 - y) / 84, 0, 1)
  return v * h * h
end)
print(shoreP, glowP, leftP)
print(mShoreLight:area(), mGlow:area(), mLeftLight:area())
work(mShoreLight, {hand="body", pile=shoreP, coverage=1.7, fill=true,
                   threshold=0.03, clip=true, angle=0.02, tool="filbert 6",
                   length={40, 110}, pressure={0.3, 0.55}})
work(mGlow, {hand="body", pile=glowP, coverage=1.6, fill=true, threshold=0.03,
             clip=true, angle=0.02, tool="filbert 6", length={40, 120},
             pressure={0.28, 0.5}})
work(mLeftLight, {hand="body", pile=leftP, coverage=1.5, fill=true,
                  threshold=0.03, clip=true, angle=0.02, tool="filbert 6",
                  length={40, 120}, pressure={0.28, 0.5}})
blend(mLake, {angle=0.03})
print("light laid as films")

--@ chunk 181
print(wait(2 * 24 * 60))
print("dry?", drying(600, 550))
work(mLake, {hand="body", pile=lakeD2, coverage=1.45, fill=true, clip=true,
             angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.45, 0.72}})
blend(mLake, {angle=0.03})
print("back to calm")

--@ chunk 182
print(wait(2 * 24 * 60))
print("dry?", drying(600, 550))
-- bury the orange glitter under one more cover
work(mLake, {hand="body", pile=lakeD2, coverage=1.25, fill=true, clip=true,
             angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.45, 0.72}})
blend(mLake, {angle=0.03})
print("covered")

--@ chunk 183
print(wait(2 * 24 * 60))
print("dry?", drying(600, 550), drying(200, 520))
-- the bright edge where the water takes the mist
mEdgeLight = mLake * mask(function(x, y)
  local d = y - 452
  return clamp((d - 2) / 5, 0, 1) * clamp((23 - d) / 12, 0, 1)
end)
work(mEdgeLight, {hand="body", pile=shoreP, coverage=1.05, fill=true,
                  threshold=0.04, clip=true, angle=0.02, tool="filbert 4",
                  length={30, 90}, pressure={0.25, 0.5}})
print("edge of light")

--@ chunk 184
-- soft dashes, a step lighter than the water, gathered in the glow's path
softP = pile{{"lead white", 1.8}, {"pale smalt", 2.8}, {"yellow ochre", 1.1}}
softC = pile{{"lead white", 1.8}, {"pale smalt", 3.2}, {"yellow ochre", 0.4}}
darkRip = pile{{"lead white", 0.5}, {"pale smalt", 1.2}, {"smalt", 1.3},
               {"raw umber", 2.4}, {"bone black", 1.2}}
function softdash(cx, cy, sx, sy, n, lmin, lmax, pile, pmin, pmax, wd, ld)
  local b = brush{kind = "filbert", width = wd, point = 0.3, stiffness = 0.5}
  for i = 1, n do
    local x = cx + randn(0, sx)
    local y = cy + randn(0, sy)
    local L = rand(lmin, lmax)
    local p = rand(pmin, pmax)
    local w = randn(0, 0.9)
    b:load(pile, rand(ld, ld + 0.25))
    b:stroke({{x, y}, {x + L * 0.4, y + w}, {x + L * 0.75, y - w * 0.6}, {x + L, y}},
             {pressure = {0.04, p, p * 0.8, 0.03}, orient = "across", clip = mLake})
  end
end
softdash(700, 480, 120, 8, 8, 40, 130, softP, 0.14, 0.26, 5, 0.25)
softdash(760, 508, 130, 9, 8, 55, 160, softP, 0.12, 0.24, 5.5, 0.22)
softdash(700, 538, 140, 10, 7, 70, 190, softP, 0.1, 0.2, 6, 0.2)
softdash(830, 566, 100, 9, 5, 80, 220, softP, 0.09, 0.18, 6.5, 0.18)
softdash(630, 496, 60, 7, 5, 30, 90, softP, 0.12, 0.24, 4.5, 0.22)
-- cool light at the left under the bright sky
softdash(140, 476, 110, 8, 6, 30, 110, softC, 0.12, 0.24, 5, 0.2)
softdash(230, 505, 100, 10, 6, 40, 140, softC, 0.1, 0.2, 5.5, 0.18)
softdash(90, 540, 80, 9, 4, 50, 170, softC, 0.08, 0.16, 6, 0.16)
-- and dark drags to give the surface its grain
softdash(450, 520, 190, 16, 5, 100, 280, darkRip, 0.1, 0.2, 7, 0.22)
softdash(620, 588, 260, 12, 4, 130, 330, darkRip, 0.09, 0.18, 8, 0.2)
print("soft light")

--@ chunk 185
print(wait(2 * 24 * 60))
print("dry?", drying(600, 550), drying(300, 460))
work(mLake, {hand="body", pile=lakeD2, coverage=1.3, fill=true, clip=true,
             angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.45, 0.72}})
blend(mLake, {angle=0.03})
print("cleared")

--@ chunk 186
-- the mountain upside down in the water: a soft wedge narrowing as it comes forward
mPeakRefl = mLake * mask(function(x, y)
  local d = y - 455
  local w = 265 * clamp(1 - d / 128, 0, 1) ^ 0.55
  local h = clamp(1 - math.abs(x - 745) / w, 0, 1)
  return h * h * (3 - 2 * h) * clamp(d / 7, 0, 1) * clamp((124 - d) / 44, 0, 1)
end)
print("wedge area", mPeakRefl:area())
reflP = pile{{"lead white", 1.6}, {"pale smalt", 2.6}, {"raw umber", 0.8},
             {"yellow ochre", 0.35}}
work(mPeakRefl, {hand="body", pile=reflP, coverage=1.5, fill=true,
                 threshold=0.04, clip=true, angle=0.02, tool="filbert 6",
                 length={40, 120}, pressure={0.3, 0.55}})
blend(mLake, {angle=0.03})
print("the mountain in the water")

--@ chunk 187
print(wait(2 * 24 * 60))
print("dry?", drying(700, 500))
work(mLake, {hand="body", pile=lakeD2, coverage=1.25, fill=true, clip=true,
             angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.45, 0.72}})
blend(mLake, {angle=0.03})
print("cleared")

--@ chunk 188
function sampleIn(m, n, thresh, y0, y1)
  local out, k, t = {}, 0, 0
  while k < n and t < n * 12 do
    local x = rand(0, W)
    local y = rand(y0, y1)
    local v = m:at(x, y)
    if v > thresh then
      out[k + 1] = {x, y, v}
      k = k + 1
    end
    t = t + 1
  end
  return out
end
reflP = pile{{"lead white", 1.4}, {"pale smalt", 3}, {"raw umber", 0.7},
             {"yellow ochre", 0.3}}
bN = brush{kind = "filbert", width = 2.2, point = 0.25, stiffness = 0.5}
bM = brush{kind = "filbert", width = 3.6, point = 0.25, stiffness = 0.5}
bW = brush{kind = "filbert", width = 5.4, point = 0.25, stiffness = 0.5}
local pts = sampleIn(mPeakRefl, 150, 0.25, 455, 590)
for i = 1, #pts do
  local x, y, v = pts[i][1], pts[i][2], pts[i][3]
  local dep = clamp((y - 455) / 130, 0, 1)
  local L = rand(14, 34) + dep * rand(20, 90)
  local b = dep < 0.33 and bN or (dep < 0.66 and bM or bW)
  local p = (0.22 + 0.26 * v) * (1 - dep * 0.35)
  b:load(reflP, rand(0.25, 0.5))
  b:stroke({{x, y}, {x + L * 0.4, y + randn(0, 0.7)}, {x + L * 0.75, y - randn(0, 0.5)},
             {x + L, y}},
            {pressure = {0.05, p, p * 0.8, 0.04}, orient = "across", clip = mLake})
end
print("marks", #pts)

--@ chunk 189
print(wait(2 * 24 * 60))
print("dry?", drying(700, 500))
work(mLake, {hand="body", pile=lakeD2, coverage=1.3, fill=true, clip=true,
             angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.45, 0.72}})
blend(mLake, {angle=0.03})
print(wait(2 * 24 * 60))
print("dry?", drying(700, 500))
-- the mountain's reflection: a tonal lift, low contrast, tapering as it comes forward
reflP2 = pile{{"lead white", 2}, {"pale smalt", 2}, {"smalt", 1.6},
              {"raw umber", 2}, {"bone black", 0.5}}
work(mPeakRefl, {hand="body", pile=reflP2, coverage=0.85, fill=true,
                 threshold=0.05, clip=true, angle=0.02, tool="filbert 6",
                 length={40, 120}, pressure={0.3, 0.55}})
blend(mLake, {angle=0.03})
print("tonal reflection")

--@ chunk 190
print(wait(2 * 24 * 60))
print("dry?", drying(300, 500), drying(700, 500))
-- even out that brighter patch at the left of the reflection
work(ellipse(556, 502, 88, 46):blur(30), {hand="body", pile=lakeD2,
      coverage=1, threshold=0.03, clip=true, angle=0.02, tool="filbert 6",
      length={40, 120}, pressure={0.35, 0.6}})
-- cool light under the bright sky at the left
mLeftSky = mLake * mask(function(x, y)
  local h = clamp(1 - x / 400, 0, 1)
  local v = clamp((y - 454) / 18, 0, 1) * clamp((548 - y) / 74, 0, 1)
  return v * h * h
end)
work(mLeftSky, {hand="body", pile=reflP2, coverage=0.6, threshold=0.04,
                clip=true, angle=0.02, tool="filbert 6", length={40, 120},
                pressure={0.3, 0.55}})
-- and the dark bank's reflection lying under it
mBankRefl = mLake * mask(function(x, y)
  local d = y - 453
  local h = clamp(1 - x / 470, 0, 1)
  return clamp(d / 6, 0, 1) * clamp((92 - d) / 40, 0, 1) * h
end)
work(mBankRefl, {hand="body", pile=darkRip, coverage=0.9, threshold=0.04,
                 clip=true, angle=0.02, tool="filbert 6", length={40, 120},
                 pressure={0.3, 0.55}})
print("left of the lake")

--@ chunk 191
-- the two trunks again in the water, and the ripples that break them
vb = brush{kind = "filbert", width = 5, point = 0.3, stiffness = 0.45}
for _, tx in ipairs({106, 118, 250, 264}) do
  for i = 1, 16 do
    local y0 = 456 + i * 8.6 + randn(0, 3)
    local len = rand(10, 24) + i * 0.9
    local x = tx + randn(0, 4 + i * 0.6)
    vb:load(darkRip, rand(0.2, 0.42) * (1 - i / 26))
    vb:stroke({{x, y0}, {x + randn(0, 2), y0 + len * 0.55}, {x + randn(0, 3.5), y0 + len}},
              {pressure = {0.45, 0.3, 0.03}, orient = "across", clip = mLake})
  end
end
-- ripples over them, and long quiet drags in the near water
bN = brush{kind = "filbert", width = 2.4, point = 0.25, stiffness = 0.5}
bM = brush{kind = "filbert", width = 4, point = 0.25, stiffness = 0.5}
bW = brush{kind = "filbert", width = 6.5, point = 0.25, stiffness = 0.5}
local function drags(y0, y1, n, lmin, lmax, pile, pmin, pmax)
  for i = 1, n do
    local x = rand(-20, W - 40)
    local y = rand(y0, y1)
    local dep = clamp((y - 455) / 200, 0, 1)
    local L = rand(lmin, lmax)
    local b = dep < 0.33 and bN or (dep < 0.66 and bM or bW)
    local p = rand(pmin, pmax) * (1 - dep * 0.3)
    b:load(pile, rand(0.2, 0.42))
    b:stroke({{x, y}, {x + L * 0.4, y + randn(0, 0.9)}, {x + L * 0.75, y - randn(0, 0.7)},
              {x + L, y}},
             {pressure = {0.04, p, p * 0.8, 0.03}, orient = "across", clip = mLake})
  end
end
drags(456, 540, 34, 26, 90, reflP2, 0.14, 0.28)
drags(540, 665, 26, 90, 260, reflP2, 0.1, 0.2)
drags(560, 668, 12, 110, 300, darkRip, 0.1, 0.18)
drags(456, 600, 14, 30, 120, darkRip, 0.1, 0.2)
print("reflections and ripples")

--@ chunk 192
print(wait(2 * 24 * 60))
print("dry?", drying(300, 500), drying(700, 520))
work(mLake, {hand="body", pile=lakeD2, coverage=1.45, fill=true, clip=true,
             angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.45, 0.72}})
blend(mLake, {angle=0.03})
print(wait(2 * 24 * 60))
print("dry?", drying(300, 500), drying(700, 520))
work(mPeakRefl, {hand="body", pile=reflP2, coverage=0.85, fill=true,
                 threshold=0.05, clip=true, angle=0.02, tool="filbert 6",
                 length={40, 120}, pressure={0.3, 0.55}})
work(mBankRefl, {hand="body", pile=darkRip, coverage=0.5, threshold=0.05,
                 clip=true, angle=0.02, tool="filbert 6", length={40, 120},
                 pressure={0.3, 0.55}})
work(mLeftSky, {hand="body", pile=reflP2, coverage=0.5, threshold=0.05,
                clip=true, angle=0.02, tool="filbert 6", length={40, 120},
                pressure={0.3, 0.55}})
blend(mLake, {angle=0.03})
print("quiet water")

--@ chunk 193
print(wait(2 * 24 * 60))
bW = brush{kind = "filbert", width = 6.5, point = 0.25, stiffness = 0.5}
for i = 1, 15 do
  local x = rand(-30, W - 200)
  local y = rand(505, 655)
  local L = rand(120, 300)
  local p = rand(0.05, 0.11)
  bW:load(reflP2, rand(0.15, 0.3))
  bW:stroke({{x, y}, {x + L * 0.4, y + randn(0, 1.2)}, {x + L * 0.75, y - randn(0, 0.9)},
             {x + L, y}},
            {pressure = {0.02, p, p * 0.8, 0.02}, orient = "across", clip = mLake})
end
print("drags")

--@ chunk 194
twigP = pile{{"bone black", 1.6}, {"raw umber", 2.4}, {"pale smalt", 1.8}}
tb = brush{kind = "rigger", width = 1.3, point = 1, stiffness = 0.4}
local cnt = 0
for _, list in ipairs({branches, crown}) do
  for _, b in ipairs(list) do
    local n = #b.pts
    for i = math.max(2, math.floor(n * 0.5)), n do
      local x, y = b.pts[i][1], b.pts[i][2]
      if y < 655 and y > -20 then
        local dx, dy
        if b.pts[i + 1] then
          dx, dy = b.pts[i + 1][1] - x, b.pts[i + 1][2] - y
        else
          dx, dy = x - b.pts[i - 1][1], y - b.pts[i - 1][2]
        end
        local ang = math.atan(dy, dx)
        local outer = i / n
        local nt = rand() < 0.8 and rand(1, 3) or 1
        for k = 1, nt do
          local a = ang + rand(-0.85, 0.85) - 0.4
          local L = (7 + 20 * outer) * rand(0.6, 1.4)
          local p0 = rand(0.25, 0.6)
          local mx = x + math.cos(a) * L * 0.5
          local my = y + math.sin(a) * L * 0.5 - L * 0.14
          tb:load(twigP, p0)
          tb:stroke({{x, y}, {mx, my},
                     {x + math.cos(a + rand(-0.3, 0.3)) * L,
                      y + math.sin(a + rand(-0.3, 0.3)) * L - L * 0.22}},
                    {pressure = {0.55, 0.34, 0.03}, orient = "across"})
          cnt = cnt + 1
          if rand() < 0.55 then
            local a2 = a - rand(0.15, 0.75)
            local L2 = L * rand(0.4, 0.75)
            tb:load(twigP, p0 * 0.6)
            tb:stroke({{mx, my}, {mx + math.cos(a2) * L2 * 0.6,
                                  my + math.sin(a2) * L2 * 0.6 - L2 * 0.15},
                       {mx + math.cos(a2 + rand(-0.25, 0.25)) * L2,
                        my + math.sin(a2 + rand(-0.25, 0.25)) * L2 - L2 * 0.2}},
                      {pressure = {0.3, 0.18, 0.02}, orient = "across"})
            cnt = cnt + 1
          end
        end
      end
    end
  end
end
print("twigs", cnt)

--@ chunk 195
print(wait(2 * 24 * 60))
print("dry?", drying(150, 250))
-- fine side-branchlets along the outer half of every limb: this is what makes
-- a stick read as a branch
fb = brush{kind = "rigger", width = 0.9, point = 1, stiffness = 0.35}
local cnt = 0
for _, list in ipairs({branches, crown}) do
  for _, b in ipairs(list) do
    local n = #b.pts
    for i = math.max(2, math.floor(n * 0.35)), n do
      local x, y = b.pts[i][1], b.pts[i][2]
      if y < 600 and y > -20 then
        local dx, dy
        if b.pts[i + 1] then
          dx, dy = b.pts[i + 1][1] - x, b.pts[i + 1][2] - y
        else
          dx, dy = x - b.pts[i - 1][1], y - b.pts[i - 1][2]
        end
        local ang = math.atan(dy, dx)
        local outer = i / n
        local k = (i >= n) and 3 or rand(1, 3)
        for s = 1, k do
          for _, side in ipairs({-1, 1}) do
            if rand() < 0.75 then
              local a = ang + side * rand(0.45, 1.15) - 0.35
              local L = (3.5 + 9 * outer) * rand(0.5, 1.5)
              fb:load(twigP, rand(0.2, 0.5))
              fb:stroke({{x, y},
                         {x + math.cos(a) * L * 0.6, y + math.sin(a) * L * 0.6 - L * 0.1},
                         {x + math.cos(a + rand(-0.35, 0.35)) * L,
                          y + math.sin(a + rand(-0.35, 0.35)) * L - L * 0.25}},
                        {pressure = {0.5, 0.3, 0.02}, orient = "across"})
              cnt = cnt + 1
            end
          end
        end
      end
    end
  end
end
print("side-branchlets", cnt)

--@ chunk 196
print("mFig2 area", mFig2:area(), "hull", hull:area(), "gunwale", gunwale:area())
local cx, cy, n = 0, 0, 0
for x = 400, 760, 4 do
  for y = 540, 680, 4 do
    if mFig2:at(x, y) > 0.5 then cx = cx + x; cy = cy + y; n = n + 1 end
  end
end
print("figure centre", cx / n, cy / n, n)
cx, cy, n = 0, 0, 0
for x = 400, 800, 4 do
  for y = 560, 700, 4 do
    if hull:at(x, y) > 0.5 then cx = cx + x; cy = cy + y; n = n + 1 end
  end
end
print("hull centre", cx / n, cy / n, n)
print("head2", head2:area(), "skirt2", skirt2:area(), "shoulders", shoulders:area())
print("body2", body2:area(), "armL", armL:area(), "armR", armR:area())
for x = 400, 800, 8 do
  for y = 560, 700, 8 do
    if mFig2:at(x, y) > 0.5 then print("fig pt", x, y) break end
  end
end

--@ chunk 197
local x0, x1, y0, y1 = 1e9, -1e9, 1e9, -1e9
for x = 500, 620, 2 do
  for y = 560, 680, 2 do
    if mFig2:at(x, y) > 0.4 then
      x0 = math.min(x0, x); x1 = math.max(x1, x)
      y0 = math.min(y0, y); y1 = math.max(y1, y)
    end
  end
end
print("mFig2 bbox", x0, y0, x1, y1)
local hx0, hx1, hy0, hy1 = 1e9, -1e9, 1e9, -1e9
for x = 560, 700, 2 do
  for y = 620, 700, 2 do
    if hull:at(x, y) > 0.4 then
      hx0 = math.min(hx0, x); hx1 = math.max(hx1, x)
      hy0 = math.min(hy0, y); hy1 = math.max(hy1, y)
    end
  end
end
print("hull bbox", hx0, hy0, hx1, hy1)
print("drying lake", drying(620, 660), drying(300, 600))

--@ chunk 198
-- her again
work(mFig2, {hand="body", pile=figP2, coverage=5, fill=true, clip=true,
             tool="filbert 2.2", length={5, 16}, pressure={0.75, 1}})
rimF = mFig2:offset(1.5) - mFig2
work(rimF, {hand="body", pile=pile{{"lead white", 2}, {"yellow ochre", 1.2},
      {"pale smalt", 1.2}}, coverage=0.7, clip=true, tool="filbert 2",
      length={4, 12}, pressure={0.3, 0.55}})
-- and her boat, which I had put below the waterline: float it, moored at the edge
hull2 = poly({{592, 651}, {638, 647}, {680, 656}, {664, 666}, {612, 663}}, true)
work(hull2, {hand="body", pile=boatP, coverage=2.4, fill=true, clip=true,
             tool="filbert 2.6", length={5, 16}, pressure={0.55, 0.85}})
well2 = hull2:shrink(3.4)
gunw2 = hull2 - well2:offset(0, 2)
work(gunw2, {hand="body", pile=pile{{"lead white", 1.5}, {"yellow ochre", 1.3},
      {"raw umber", 1.6}}, coverage=1.6, clip=true, tool="filbert 2",
      length={4, 14}, pressure={0.4, 0.7}})
work(ellipse(637, 671, 40, 7):blur(7), {hand="body", pile=lakeD2,
      coverage=1.2, threshold=0.03, clip=true, tool="filbert 4",
      length={20, 60}, pressure={0.35, 0.6}})
print("figure and boat")

--@ chunk 199
print(wait(2 * 24 * 60))
print("dry?", drying(557, 620), drying(637, 656), drying(300, 672))
-- her: no glowing outline, just the shape, with one thin edge of light
work(mFig2, {hand="body", pile=figP2, coverage=4.5, fill=true, clip=true,
             tool="filbert 2.2", length={5, 16}, pressure={0.75, 1}})
mRimR = (mFig2:offset(1.5) - mFig2) * mask(function(x, y)
  return clamp((572 - x) / 5, 0, 1) * clamp((636 - y) / 30, 0, 1)
end)
work(mRimR, {hand="body", pile=pile{{"lead white", 1.6}, {"pale smalt", 1.8}},
      coverage=0.5, clip=true, tool="filbert 1.6", length={3, 9},
      pressure={0.2, 0.4}})
-- the boat, dark, with a thin gunwale
hull2 = poly({{590, 654}, {606, 648}, {640, 646}, {676, 652}, {684, 658},
              {668, 666}, {612, 664}}, true)
work(hull2, {hand="body", pile=boatP, coverage=3, fill=true, clip=true,
             tool="filbert 2.6", length={5, 16}, pressure={0.6, 0.9}})
work(ribbon({{591, 653}, {608, 648}, {640, 646}, {676, 652}, {683, 658}}, 1.5),
      {hand="body", pile=pile{{"lead white", 1.2}, {"yellow ochre", 1.1},
       {"raw umber", 2}}, coverage=1.4, clip=true, tool="filbert 1.8",
       length={4, 12}, pressure={0.35, 0.65}})
print("figure and boat, quieter")

--@ chunk 200
print(wait(2 * 24 * 60))
print("dry?", drying(557, 615), drying(300, 668))
-- the glowing ring round her: cover it with the water it sits in, then her again
mRing = (mFig2:grow(3.4) - mFig2) * mLake * mask(function(x, y)
  return clamp((652 - y) / 8, 0, 1)
end)
print("ring", mRing:area())
work(mRing, {hand="body", pile=lakeD2, coverage=2.2, fill=true, threshold=0.03,
             clip=true, angle=0.02, tool="filbert 2.4", length={6, 20},
             pressure={0.5, 0.8}})
work(mFig2, {hand="body", pile=figP2, coverage=4.5, fill=true, clip=true,
             tool="filbert 2.2", length={5, 16}, pressure={0.75, 1}})
print("ring cleared")

--@ chunk 201
print(wait(2 * 24 * 60))
print("dry?", drying(300, 668), drying(200, 540), drying(800, 430))
-- the bank: even it, then a grass line again
work(mShore, {hand="body", pile=bankQuiet, coverage=1.5, fill=true, clip=true,
              angle=-0.25, length={22, 55}, pressure={0.5, 0.8}})
-- the left of the lake: bury the scribble of old marks
mBury = mLake * mask(function(x, y)
  return clamp((450 - x) / 80, 0, 1) * clamp((y - 452) / 16, 0, 1)
              * clamp((662 - y) / 60, 0, 1)
end)
work(mBury, {hand="body", pile=lakeD2, coverage=1.4, fill=true, clip=true,
             angle=0.02, tool="filbert 7", length={70, 180}, pressure={0.45, 0.72}})
blend(mLake, {angle=0.03})
print("bank and left water")

--@ chunk 202
print(wait(2 * 24 * 60))
print("dry?", drying(800, 430), drying(700, 420))
-- the far shore: mist over it, so the little trees stop being flies
mFarV = mask(function(x, y)
  local d = y - 403
  return clamp((d + 9) / 11, 0, 1) * clamp((459 - y) / 17, 0, 1)
end)
work(mFarV, {hand="body", pile=mistLay, coverage=2.4, fill=true,
             threshold=0.03, clip=true, angle=0.03, tool="filbert 5",
             length={40, 120}, pressure={0.45, 0.75}})
mFarV2 = mFarV * mask(function(x, y) return clamp((x - 520) / 70, 0, 1) end)
work(mFarV2, {hand="body", pile=mistLay, coverage=1.8, fill=true,
              threshold=0.03, clip=true, angle=0.03, tool="filbert 5",
              length={40, 120}, pressure={0.45, 0.75}})
blend(mFarV, {angle=0.03})
print("far shore in mist")

--@ chunk 203
print(wait(2 * 24 * 60))
print("dry?", drying(300, 700), drying(557, 620), drying(800, 430))
mBand = mask(function(x, y)
  local d = y - 398
  return clamp((d + 8) / 10, 0, 1) * clamp((466 - y) / 18, 0, 1)
end)
-- first take the pink out of the band
work(mBand, {hand="body", pile=mistCool, coverage=1.7, fill=true,
             threshold=0.03, clip=true, angle=0.03, tool="filbert 5",
             length={40, 120}, pressure={0.45, 0.75}})
-- the dark bank under the trees, its foot lost in the mist
ridge = {{-10, 444}, {60, 422}, {120, 405}, {180, 396}, {230, 390}, {280, 394},
         {330, 404}, {380, 414}, {430, 424}, {480, 431}, {530, 436}, {575, 439},
         {575, 470}, {300, 470}, {-10, 466}}
mBankDark = poly(ridge, true) * mask(function(x, y)
  return clamp((470 - y) / 16, 0, 1)
end)
work(mBankDark, {hand="body", pile=midDark, coverage=2.2, fill=true,
                 threshold=0.03, clip=true, angle=0.05, tool="filbert 6",
                 length={40, 110}, pressure={0.45, 0.75}})
-- and the pale foot of the peak at the right
mPeakFoot = mBand * mask(function(x, y) return clamp((x - 440) / 80, 0, 1) end)
work(mPeakFoot, {hand="body", pile=crestP, coverage=1.8, fill=true,
                 threshold=0.03, clip=true, angle=0.03, tool="filbert 6",
                 length={40, 120}, pressure={0.45, 0.75}})
blend(mBand, {angle=0.03})
print("far shore back")

--@ chunk 204
print(wait(2 * 24 * 60))
print("dry?", drying(150, 300), drying(557, 620))
-- the trunks again over the bank
work(mAt, {hand="body", pile=treeDark, coverage=2.6, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.5, 0.85}})
work(mA, {hand="body", pile=treeDark, coverage=2.6, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.75}})
work(mBt, {hand="body", pile=treeFar, coverage=2.6, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.45, 0.8}})
work(mB, {hand="body", pile=treeFar, coverage=2.5, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.7}})
-- and her, and the boat, which the bank reached
work(mFig2, {hand="body", pile=figP2, coverage=4.5, fill=true, clip=true,
             tool="filbert 2.2", length={5, 16}, pressure={0.75, 1}})
work(hull2, {hand="body", pile=boatP, coverage=3, fill=true, clip=true,
             tool="filbert 2.6", length={5, 16}, pressure={0.6, 0.9}})
work(ribbon({{591, 653}, {608, 648}, {640, 646}, {676, 652}, {683, 658}}, 1.5),
      {hand="body", pile=pile{{"lead white", 1.2}, {"yellow ochre", 1.1},
       {"raw umber", 2}}, coverage=1.4, clip=true, tool="filbert 1.8",
       length={4, 12}, pressure={0.35, 0.65}})
print("trees, figure, boat")

--@ chunk 205
print(wait(2 * 24 * 60))
print("dry?", drying(300, 420), drying(800, 420), drying(150, 300))
farLand = poly({{-10, 446}, {60, 424}, {120, 407}, {180, 398}, {230, 392},
                {280, 396}, {330, 406}, {380, 416}, {430, 426}, {480, 433},
                {530, 438}, {575, 441}, {620, 414}, {700, 401}, {800, 398},
                {900, 402}, {1010, 408}, {1010, 456}, {575, 456}, {-10, 454}},
               true)
farLand = farLand * mask(function(x, y) return clamp((456 - y) / 12, 0, 1) end)
mLandL = farLand * mask(function(x, y) return clamp((560 - x) / 70, 0, 1) end)
mLandR = farLand * mask(function(x, y) return clamp((x - 520) / 70, 0, 1) end)
work(mLandL, {hand="body", pile=midDark, coverage=2.4, fill=true,
              threshold=0.03, clip=true, angle=0.05, tool="filbert 6",
              length={40, 110}, pressure={0.5, 0.8}})
work(mLandR, {hand="body", pile=crestP, coverage=2, fill=true,
              threshold=0.03, clip=true, angle=0.05, tool="filbert 6",
              length={40, 110}, pressure={0.5, 0.8}})
-- and only a narrow mist where the land meets the water
mThin = mask(function(x, y)
  local d = y - 436
  local thick = 14 + 9 * nzMist:at01(x, 90) - 4 * x / 1000
  return clamp((d + 7) / 8, 0, 1) * clamp((thick - d) / 11, 0, 1)
end)
work(mThin, {hand="body", pile=mistLay, coverage=1.6, fill=true,
             threshold=0.04, clip=true, angle=0.02, tool="filbert 5",
             length={40, 120}, pressure={0.4, 0.7}})
blend(mThin, {angle=0.03})
print("far land back")

--@ chunk 206
print(wait(2 * 24 * 60))
-- the trunks again where the land went over them
work(mAt, {hand="body", pile=treeDark, coverage=2.6, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.5, 0.85}})
work(mA, {hand="body", pile=treeDark, coverage=2.6, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.75}})
work(mBt, {hand="body", pile=treeFar, coverage=2.6, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.45, 0.8}})
work(mB, {hand="body", pile=treeFar, coverage=2.5, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.7}})
-- and the bank: kill the bright line along its edge, then grass again
mTopLine = mask(function(x, y)
  local d = y - shoreFn(x)
  return clamp((d + 9) / 6, 0, 1) * clamp((15 - d) / 11, 0, 1)
end)
work(mTopLine, {hand="body", pile=bankQuiet, coverage=2, fill=true, threshold=0.03,
                clip=true, angle=-0.3, tool="filbert 4", length={15, 45},
                pressure={0.45, 0.75}})
print("trunks, bank edge")

--@ chunk 207
print(wait(2 * 24 * 60))
print("dry?", drying(380, 415), drying(500, 465))
-- a pale gash left on the dark hill: back to the hill's colour
work(ellipse(378, 416, 58, 32):blur(22), {hand="body", pile=midDark,
      coverage=2.2, threshold=0.03, clip=true, angle=0.1, tool="filbert 5",
      length={30, 80}, pressure={0.45, 0.75}})
-- and the mist's lower edge, dissolved into the water
mThin2 = mask(function(x, y)
  local d = y - 455
  return clamp((d + 7) / 9, 0, 1) * clamp((27 - d) / 15, 0, 1)
end)
work(mThin2, {hand="body", pile=mistLay, coverage=1.0, fill=true,
              threshold=0.05, clip=true, angle=0.02, tool="filbert 5",
              length={40, 120}, pressure={0.35, 0.6}})
print("hill and mist edge")

--@ chunk 208
-- the lower limbs are the heaviest marks in the picture: give them their fine
-- branchlets too, so they taper instead of ending as bars
fb = brush{kind = "rigger", width = 0.9, point = 1, stiffness = 0.35}
local cnt = 0
for _, list in ipairs({branches, crown}) do
  for _, b in ipairs(list) do
    local n = #b.pts
    for i = math.max(2, math.floor(n * 0.3)), n do
      local x, y = b.pts[i][1], b.pts[i][2]
      if y >= 440 and y < 790 then
        local dx, dy
        if b.pts[i + 1] then
          dx, dy = b.pts[i + 1][1] - x, b.pts[i + 1][2] - y
        else
          dx, dy = x - b.pts[i - 1][1], y - b.pts[i - 1][2]
        end
        local ang = math.atan(dy, dx)
        local outer = i / n
        for s = 1, (i >= n) and 3 or 2 do
          for _, side in ipairs({-1, 1}) do
            if rand() < 0.8 then
              local a = ang + side * rand(0.5, 1.25) - 0.4
              local L = (5 + 13 * outer) * rand(0.5, 1.5)
              fb:load(twigP, rand(0.2, 0.48))
              fb:stroke({{x, y},
                         {x + math.cos(a) * L * 0.6, y + math.sin(a) * L * 0.6 - L * 0.1},
                         {x + math.cos(a + rand(-0.35, 0.35)) * L,
                          y + math.sin(a + rand(-0.35, 0.35)) * L - L * 0.3}},
                        {pressure = {0.5, 0.3, 0.02}, orient = "across"})
              cnt = cnt + 1
              if rand() < 0.4 then
                local a2 = a - rand(0.2, 0.8)
                local L2 = L * rand(0.35, 0.6)
                fb:load(twigP, rand(0.12, 0.3))
                fb:stroke({{x + math.cos(a) * L * 0.55,
                            y + math.sin(a) * L * 0.55 - L * 0.1},
                           {x + math.cos(a2) * L2, y + math.sin(a2) * L2 - L2 * 0.2}},
                          {pressure = {0.28, 0.02}, orient = "across"})
                cnt = cnt + 1
              end
            end
          end
        end
      end
    end
  end
end
print("lower branchlets", cnt)

--@ chunk 209
for i = 1, 6 do
  local b = branches[i]
  local s = ""
  for j = 1, #b.pts do s = s .. string.format("(%.0f,%.0f) ", b.pts[j][1], b.pts[j][2]) end
  print(i, "w", b.w0, b.w1, s)
end
print("---")
for i = 1, 3 do
  local b = crown[i]
  local s = ""
  for j = 1, #b.pts do s = s .. string.format("(%.0f,%.0f) ", b.pts[j][1], b.pts[j][2]) end
  print(i, "w", b.w0, b.w1, s)
end
print("mAt 250,700", mAt:at(250, 700), "mA 250,700", mA:at(250, 700))
print("mAt 250,600", mAt:at(250, 600), "mA 250,600", mA:at(250, 600))
print("mAt 100,600", mAt:at(100, 600), "mA 100,600", mA:at(100, 600))

--@ chunk 210
for i, b in ipairs(branches) do
  if b.w0 >= 7 then
    local p1, pn = b.pts[1], b.pts[#b.pts]
    print(string.format("b%d w=%.1f->%.1f base=(%.0f,%.0f) tip=(%.0f,%.0f)",
          i, b.w0, b.w1, p1[1], p1[2], pn[1], pn[2]))
  end
end
print("--- crown")
for i, b in ipairs(crown) do
  if b.w0 >= 7 then
    local p1, pn = b.pts[1], b.pts[#b.pts]
    print(string.format("c%d w=%.1f->%.1f base=(%.0f,%.0f) tip=(%.0f,%.0f)",
          i, b.w0, b.w1, p1[1], p1[2], pn[1], pn[2]))
  end
end

--@ chunk 211
print(wait(2 * 24 * 60))
print("dry?", drying(252, 700), drying(300, 300))
-- the second trunk ended in a rounded cap above the bank: carry it into the ground
tb2 = brush{kind = "filbert", width = 23, point = 0, stiffness = 0.55}
tb2:load(treeDark, 0.85)
tb2:stroke({{250, 690}, {245, 745}, {241, 800}},
           {pressure = {0.55, 0.8, 0.9}, orient = "across"})
tb3 = brush{kind = "filbert", width = 26, point = 0, stiffness = 0.55}
tb3:load(treeDark, 0.7)
tb3:stroke({{100, 700}, {97, 760}, {95, 810}},
           {pressure = {0.5, 0.85, 0.9}, orient = "across"})
print("trunks into the ground")

--@ chunk 212
-- every blunt tip carried on into a point
local cnt = 0
for _, list in ipairs({branches, crown}) do
  for _, b in ipairs(list) do
    local n = #b.pts
    local px, py = b.pts[n][1], b.pts[n][2]
    local qx, qy = b.pts[n - 1][1], b.pts[n - 1][2]
    local dx, dy = px - qx, py - qy
    local len = math.sqrt(dx * dx + dy * dy)
    if len > 0.5 then
      local eb = brush{kind = "filbert", width = math.max(1.2, b.w1 * 1.15),
                      point = 0.5, stiffness = 0.5}
      local ang = math.atan(dy, dx)
      local L = 5 + b.w1 * rand(2.5, 5)
      local a = ang + randn(0, 0.12)
      eb:load(treeDark, rand(0.45, 0.8))
      eb:stroke({{px, py},
                 {px + math.cos(a) * L * 0.55 + randn(0, 1),
                  py + math.sin(a) * L * 0.55 + randn(0, 1)},
                 {px + math.cos(a + randn(0, 0.1)) * L,
                  py + math.sin(a + randn(0, 0.1)) * L}},
                {pressure = {0.75, 0.45, 0.02}, orient = "across"})
      cnt = cnt + 1
    end
  end
end
print("tips carried out", cnt)

--@ chunk 213
print(wait(2 * 24 * 60))
print("dry?", drying(300, 720), drying(252, 700))
-- firm the bank, then the trunks into it again
work(mShore, {hand="body", pile=bankQuiet, coverage=2, fill=true, clip=true,
              angle=-0.25, length={22, 55}, pressure={0.5, 0.8}})
tA = brush{kind = "filbert", width = 21, point = 0, stiffness = 0.55}
tA:load(treeDark, 0.9)
tA:stroke({{256, 620}, {248, 700}, {240, 800}}, {pressure = {0.7, 0.8, 0.9},
           orient = "across"})
tB = brush{kind = "filbert", width = 26, point = 0, stiffness = 0.55}
tB:load(treeDark, 0.9)
tB:stroke({{116, 620}, {104, 700}, {96, 800}}, {pressure = {0.7, 0.85, 0.9},
           orient = "across"})
print("bank, trunks")

--@ chunk 214
-- grass along the edge of the water, and tufts further back
gr = brush{kind = "rigger", width = 1.5, point = 1, stiffness = 0.4}
for i = 1, 320 do
  local x = rand(-10, 1010)
  local by = shoreFn(x) + rand(-3, 14)
  local hgt = rand(4, 17)
  local lean = rand(-0.35, 0.35) - 0.1
  gr:load(rand() < 0.55 and grassDark or reedP, rand(0.25, 0.6))
  gr:stroke({{x, by}, {x + lean * hgt * 0.6, by - hgt * 0.65}, {x + lean * hgt, by - hgt}},
            {pressure = {rand(0.4, 0.6), 0.35, 0.05}, orient = "across"})
end
for _, spot in ipairs({{60, 726}, {180, 706}, {300, 700}, {430, 716}, {520, 690},
                       {640, 704}, {760, 712}, {880, 726}, {960, 744}, {120, 752},
                       {350, 758}, {700, 754}}) do
  local x, y = spot[1], spot[2]
  for k = 1, 7 do
    local a = -math.pi / 2 + randn(0, 0.55)
    local hgt = rand(7, 17)
    gr:load(rand() < 0.5 and grassDark or reedP, rand(0.3, 0.6))
    gr:stroke({{x + randn(0, 5), y}, {x + math.cos(a) * hgt * 0.6 + randn(0, 2),
               y + math.sin(a) * hgt * 0.6}, {x + math.cos(a) * hgt + randn(0, 3),
               y + math.sin(a) * hgt}},
              {pressure = {0.55, 0.4, 0.03}, orient = "across"})
  end
end
print("grass")

--@ chunk 215
print(wait(2 * 24 * 60))
print("dry?", drying(300, 420), drying(200, 520))
-- the far bank: darker, so the left of the picture has an anchor
mBank2 = mLandL * mask(function(x, y) return clamp((y - 384) / 16, 0, 1) end)
bankD = pile{{"lead white", 0.8}, {"pale smalt", 1.8}, {"smalt", 1.6},
             {"raw umber", 3}, {"bone black", 1.8}}
work(mBank2, {hand="body", pile=bankD, coverage=1.6, fill=true,
              threshold=0.03, clip=true, angle=0.08, tool="filbert 6",
              length={40, 110}, pressure={0.45, 0.75}})
print("far bank darker")

--@ chunk 216
-- the trees again in the water at the left, very softly
mReflT = mLake * mask(function(x, y)
  local d = y - 455
  return clamp(d / 8, 0, 1) * clamp((190 - d) / 60, 0, 1)
         * clamp((330 - x) / 70, 0, 1)
end)
work(mReflT, {hand="body", pile=darkRip, coverage=0.45, threshold=0.05,
              clip=true, angle=1.35, tool="filbert 9", length={30, 80},
              pressure={0.25, 0.5}})
vb = brush{kind = "filbert", width = 7, point = 0.3, stiffness = 0.45}
for _, tx in ipairs({112, 126, 246, 258}) do
  for i = 1, 9 do
    local y0 = 458 + i * 9 + randn(0, 3)
    local len = rand(12, 26)
    local x = tx + randn(0, 3 + i * 0.5)
    vb:load(darkRip, rand(0.12, 0.26))
    vb:stroke({{x, y0}, {x + randn(0, 2), y0 + len * 0.6}, {x + randn(0, 3), y0 + len}},
              {pressure = {0.4, 0.25, 0.03}, orient = "across", clip = mLake})
  end
end
print("trees in the water")

--@ chunk 217
print(wait(2 * 24 * 60))
print("dry?", drying(200, 520))
work(mReflT, {hand="body", pile=lakeD2, coverage=2.3, fill=true, threshold=0.03,
              clip=true, angle=0.02, tool="filbert 7", length={70, 180},
              pressure={0.45, 0.72}})
blend(mLake, {angle=0.03})
print("fence covered")

--@ chunk 218
-- the far bank: a little incident, and light along its ridge
bankDarkP = pile{{"lead white", 0.6}, {"pale smalt", 1.3}, {"raw umber", 2.6},
                 {"bone black", 1.8}}
bankLightP = pile{{"lead white", 2.2}, {"pale smalt", 2.2}, {"yellow ochre", 1}}
for _, c in ipairs({{90, 425, 34, 12}, {210, 415, 28, 10}, {330, 432, 30, 9},
                    {430, 440, 24, 8}, {150, 440, 26, 8}}) do
  work(ellipse(c[1], c[2], c[3], c[4]):blur(14), {hand="body", pile=bankDarkP,
        coverage=1.1, threshold=0.03, clip=true, angle=0.1, tool="filbert 5",
        length={25, 70}, pressure={0.35, 0.6}})
end
for _, c in ipairs({{140, 412, 30, 7}, {270, 424, 26, 6}, {390, 436, 22, 5},
                    {480, 441, 18, 4}}) do
  work(ellipse(c[1], c[2], c[3], c[4]):blur(11), {hand="body", pile=bankLightP,
        coverage=1, threshold=0.03, clip=true, angle=0.05, tool="filbert 4",
        length={20, 55}, pressure={0.3, 0.55}})
end
print("bank incident")

--@ chunk 219
print(wait(2 * 24 * 60))
print("dry?", drying(150, 425))
work(mBank2, {hand="body", pile=bankD, coverage=1.5, fill=true, threshold=0.03,
              clip=true, angle=0.08, tool="filbert 6", length={40, 110},
              pressure={0.45, 0.75}})
-- a ridge light, dull this time
ridgeL = pile{{"lead white", 1.1}, {"pale smalt", 2}, {"raw umber", 2.2},
              {"yellow ochre", 0.5}}
for _, c in ipairs({{140, 412, 34, 6}, {270, 424, 30, 5}, {390, 436, 24, 4}}) do
  work(ellipse(c[1], c[2], c[3], c[4]):blur(13), {hand="body", pile=ridgeL,
        coverage=0.7, threshold=0.04, clip=true, angle=0.05, tool="filbert 4",
        length={20, 55}, pressure={0.25, 0.45}})
end
blend(mBank2, {angle=0.1})
print("bank evened")

--@ chunk 220
ridgePts = {{-10, 447}, {60, 425}, {120, 408}, {180, 399}, {230, 393},
             {280, 397}, {330, 407}, {380, 417}, {430, 427}, {480, 434},
             {530, 439}, {575, 442}}
function ridgeAt(x)
  for i = 1, #ridgePts - 1 do
    local a, b = ridgePts[i], ridgePts[i + 1]
    if x >= a[1] and x <= b[1] then
      return a[2] + (b[2] - a[2]) * (x - a[1]) / (b[1] - a[1])
    end
  end
  return ridgePts[#ridgePts][2]
end
print("ridge at 0,230,575:", ridgeAt(0), ridgeAt(230), ridgeAt(575))
print(wait(2 * 24 * 60))
print("dry?", drying(230, 410))
-- mist along the ridge, so the arc is not a drawn line
mRidgeV = mask(function(x, y)
  local d = math.abs(y - ridgeAt(x))
  return clamp((d - 2) / 7, 0, 1) * clamp((34 - d) / 14, 0, 1)
end)
work(mRidgeV, {hand="body", pile=mistLay, coverage=1.3, threshold=0.04,
               clip=true, angle=0.03, tool="filbert 5", length={40, 120},
               pressure={0.4, 0.65}})
-- mist climbing the hill in three places
for _, c in ipairs({{104, 404, 30, 26}, {292, 412, 26, 22}, {438, 428, 24, 16}}) do
  work(ellipse(c[1], c[2], c[3], c[4]):blur(15), {hand="body", pile=mistLay,
        coverage=1.5, threshold=0.04, clip=true, angle=0.04, tool="filbert 5",
        length={30, 90}, pressure={0.4, 0.65}})
end
print("ridge broken")

--@ chunk 221
work(mBank2, {hand="body", pile=bankD, coverage=2.6, fill=true,
              threshold=0.03, clip=true, angle=0.08, tool="filbert 6",
              length={40, 110}, pressure={0.5, 0.8}})
blend(mBank2, {angle=0.1})
print("bank re-laid")

--@ chunk 222
print(wait(2 * 24 * 60))
print("dry?", drying(230, 415))
-- a wood along the crest of the far bank: little trees, in clumps and gaps
nzWood = noise{seed=77, period=70, octaves=3}
hazeP2 = pile{{"bone black", 1.8}, {"raw umber", 2}, {"pale smalt", 2}}
tbw = brush{kind = "rigger", width = 1.6, point = 0.9, stiffness = 0.4}
local cnt = 0
for i = 1, 230 do
  local x = rand(-8, 578)
  local dens = nzWood:at01(x, 40)
  if rand() < 0.25 + 0.75 * dens then
    local gy = ridgeAt(x) + rand(-1, 9)
    local hgt = rand(4, 13) * (0.6 + 0.6 * dens)
    local lean = randn(0, 0.06)
    tbw:load(hazeP2, rand(0.3, 0.65))
    tbw:stroke({{x, gy}, {x + lean * hgt * 0.5, gy - hgt * 0.6},
                {x + lean * hgt, gy - hgt}},
               {pressure = {0.55, 0.4, 0.05}, orient = "across"})
    cnt = cnt + 1
    for s = 1, 2 do
      local a = -math.pi / 2 + rand(-1.1, 1.1) - 0.25
      local L = hgt * rand(0.3, 0.55)
      local by = gy - hgt * rand(0.35, 0.8)
      tbw:load(hazeP2, rand(0.2, 0.45))
      tbw:stroke({{x, by}, {x + math.cos(a) * L, by + math.sin(a) * L}},
                 {pressure = {0.4, 0.02}, orient = "across"})
    end
  end
end
print("trees on the crest", cnt)

--@ chunk 223
print(wait(2 * 24 * 60))
print("dry?", drying(300, 370))
-- the haze above the hills, evened: the white lozenges went with it
hazeLay = pile{{"lead white", 1.8}, {"pale smalt", 2.6}, {"smalt", 0.8},
               {"raw umber", 1.2}, {"yellow ochre", 0.3}}
mHaze = mask(function(x, y)
  return clamp((y - 322) / 20, 0, 1) * clamp((406 - y) / 20, 0, 1)
         * clamp((650 - x) / 90, 0, 1)
end)
print("haze area", mHaze:area())
work(mHaze, {hand="body", pile=hazeLay, coverage=2.2, fill=true,
             threshold=0.03, clip=true, angle=0.05, tool="filbert 7",
             length={60, 160}, pressure={0.45, 0.75}})
blend(mHaze, {angle=0.05})
print("haze")

--@ chunk 224
-- the trunks again, and the crest trees in a softer grey
work(mAt, {hand="body", pile=treeDark, coverage=2.6, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.5, 0.85}})
work(mA, {hand="body", pile=treeDark, coverage=2.6, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.75}})
work(mBt, {hand="body", pile=treeFar, coverage=2.6, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.45, 0.8}})
work(mB, {hand="body", pile=treeFar, coverage=2.5, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.7}})
crestP = pile{{"bone black", 1}, {"raw umber", 1.6}, {"pale smalt", 2.4}}
tbw = brush{kind = "rigger", width = 1.6, point = 0.9, stiffness = 0.4}
for i = 1, 210 do
  local x = rand(-8, 578)
  local dens = nzWood:at01(x, 40)
  if rand() < 0.25 + 0.75 * dens then
    local gy = ridgeAt(x) + rand(-1, 9)
    local hgt = rand(4, 13) * (0.6 + 0.6 * dens)
    local lean = randn(0, 0.06)
    tbw:load(crestP, rand(0.25, 0.55))
    tbw:stroke({{x, gy}, {x + lean * hgt * 0.5, gy - hgt * 0.6},
                {x + lean * hgt, gy - hgt}},
               {pressure = {0.5, 0.35, 0.05}, orient = "across"})
    for s = 1, 2 do
      local a = -math.pi / 2 + rand(-1.1, 1.1) - 0.25
      local L = hgt * rand(0.3, 0.55)
      local by = gy - hgt * rand(0.35, 0.8)
      tbw:load(crestP, rand(0.15, 0.38))
      tbw:stroke({{x, by}, {x + math.cos(a) * L, by + math.sin(a) * L}},
                 {pressure = {0.35, 0.02}, orient = "across"})
    end
  end
end
print("crest re-laid")

--@ chunk 225
print(wait(2 * 24 * 60))
print("dry?", drying(230, 420))
work(mBank2, {hand="body", pile=bankD, coverage=1.7, fill=true, threshold=0.03,
              clip=true, angle=0.08, tool="filbert 6", length={40, 110},
              pressure={0.5, 0.8}})
-- a thin veil of mist over the crest, so the little trees sit back in the air
mCrestV = mask(function(x, y)
  local d = math.abs(y - ridgeAt(x))
  return clamp((d - 4) / 10, 0, 1) * clamp((46 - d) / 20, 0, 1)
end)
work(mCrestV, {hand="glaze", pile=mistLay, coverage=0.9, threshold=0.05,
               clip=true, angle=0.03, length={80, 200}, pressure={0.3, 0.45}})
print("crest veiled")

--@ chunk 226
print(wait(2 * 24 * 60))
print("dry?", drying(230, 405))
-- the top of the far hill lost in mist: only its lower half stays as land
mSink = mask(function(x, y)
  local d = y - 384
  return clamp(d / 9, 0, 1) * clamp((40 - d) / 14, 0, 1)
         * clamp((600 - x) / 90, 0, 1)
end)
work(mSink, {hand="body", pile=hazeLay, coverage=2.2, fill=true,
             threshold=0.03, clip=true, angle=0.05, tool="filbert 7",
             length={60, 160}, pressure={0.45, 0.75}})
blend(mSink, {angle=0.05})
-- and its foot lost in the mist on the water
mFoot = mask(function(x, y)
  local d = y - 440
  return clamp(d / 8, 0, 1) * clamp((22 - d) / 14, 0, 1)
end)
work(mFoot, {hand="glaze", pile=mistLay, coverage=1.3, threshold=0.05,
             clip=true, angle=0.02, length={80, 200}, pressure={0.3, 0.5}})
print("hill sunk")

--@ chunk 227
-- the trunks again, and a low line of trees along the shore's edge
work(mAt, {hand="body", pile=treeDark, coverage=2.6, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.5, 0.85}})
work(mA, {hand="body", pile=treeDark, coverage=2.6, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.75}})
work(mBt, {hand="body", pile=treeFar, coverage=2.6, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.45, 0.8}})
work(mB, {hand="body", pile=treeFar, coverage=2.5, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.7}})
shoreP2 = pile{{"bone black", 1.4}, {"raw umber", 1.8}, {"pale smalt", 2.2}}
tbw = brush{kind = "rigger", width = 1.5, point = 0.9, stiffness = 0.4}
local cnt = 0
for i = 1, 260 do
  local x = rand(-8, 700)
  local dens = nzWood:at01(x * 1.4, 90)
  if rand() < 0.2 + 0.8 * dens then
    local gy = 428 + rand(-6, 9) + 6 * (x / 700)
    local hgt = rand(3.5, 10) * (0.6 + 0.6 * dens)
    local lean = randn(0, 0.05)
    tbw:load(shoreP2, rand(0.25, 0.55))
    tbw:stroke({{x, gy}, {x + lean * hgt * 0.5, gy - hgt * 0.6},
                {x + lean * hgt, gy - hgt}},
               {pressure = {0.5, 0.35, 0.05}, orient = "across"})
    cnt = cnt + 1
    for s = 1, 2 do
      local a = -math.pi / 2 + rand(-1.1, 1.1) - 0.25
      local L = hgt * rand(0.3, 0.55)
      local by = gy - hgt * rand(0.35, 0.8)
      tbw:load(shoreP2, rand(0.15, 0.38))
      tbw:stroke({{x, by}, {x + math.cos(a) * L, by + math.sin(a) * L}},
                 {pressure = {0.35, 0.02}, orient = "across"})
    end
  end
end
print("shore trees", cnt)

--@ chunk 228
print(wait(2 * 24 * 60))
print("dry?", drying(637, 655))
-- cover what was there
mCBank = hull2:grow(3.5) * mask(function(x, y)
  return clamp((shoreFn(x) - y + 3) / 7, 0, 1)
end)
mCWater = hull2:grow(3.5) * mask(function(x, y)
  return clamp((y - shoreFn(x) + 3) / 7, 0, 1)
end)
work(mCBank, {hand="body", pile=bankQuiet, coverage=2.4, fill=true,
              threshold=0.03, clip=true, angle=-0.2, tool="filbert 3",
              length={10, 30}, pressure={0.5, 0.8}})
work(mCWater, {hand="body", pile=lakeD2, coverage=2.2, fill=true,
               threshold=0.03, clip=true, angle=0.02, tool="filbert 3",
               length={10, 30}, pressure={0.5, 0.8}})
print("old boat covered")

--@ chunk 229
-- a boat drawn up on the shore, its well showing, an oar across it
boat3 = poly({{588, 648}, {606, 643}, {638, 642}, {668, 646}, {680, 652},
              {668, 660}, {634, 663}, {600, 659}}, true)
work(boat3, {hand="body", pile=boatP, coverage=3, fill=true, clip=true,
             tool="filbert 2.4", length={5, 16}, pressure={0.6, 0.9}})
well3 = boat3:shrink(3.4)
work(well3, {hand="body", pile=pile{{"raw umber", 2.2}, {"bone black", 1.1},
      {"yellow ochre", 0.7}}, coverage=2.4, fill=true, clip=true,
      tool="filbert 2", length={4, 14}, pressure={0.5, 0.8}})
work(ribbon({{589, 647}, {607, 642}, {638, 641}, {668, 645}, {679, 651}}, 1.3),
      {hand="body", pile=pile{{"lead white", 1.3}, {"raw umber", 2.2},
       {"pale smalt", 1.2}}, coverage=1.3, clip=true, tool="filbert 1.6",
       length={4, 12}, pressure={0.3, 0.55}})
ob = brush{kind = "rigger", width = 2.2, point = 0.4, stiffness = 0.45}
ob:load(pile{{"raw umber", 2.6}, {"yellow ochre", 1}, {"bone black", 0.8}}, 0.7)
ob:stroke({{676, 638}, {640, 652}, {602, 666}},
          {pressure = {0.3, 0.7, 0.2}, orient = "across"})
print("boat")

--@ chunk 230
print(wait(2 * 24 * 60))
work(well3, {hand="body", pile=pile{{"raw umber", 2.6}, {"bone black", 2.2},
      {"pale smalt", 0.5}}, coverage=2.4, fill=true, clip=true,
      tool="filbert 2", length={4, 14}, pressure={0.5, 0.8}})
work(ribbon({{589, 647}, {607, 642}, {638, 641}, {668, 645}, {679, 651}}, 1.3),
      {hand="body", pile=pile{{"lead white", 0.7}, {"raw umber", 2.7},
       {"pale smalt", 1.6}}, coverage=1.4, clip=true, tool="filbert 1.6",
       length={4, 12}, pressure={0.25, 0.45}})
print("well and gunwale")

--@ chunk 231
-- the mist's lower edge: broken, and a touch less pink
nzEdge2 = noise{seed=143, period=85, octaves=3}
mMistLo = mask(function(x, y)
  local d = y - 466
  local e = 16 + 12 * nzEdge2:at01(x, 30)
  return clamp(d / 7, 0, 1) * clamp((e - d) / 13, 0, 1)
end)
work(mMistLo, {hand="glaze", pile=mistCool, coverage=1.1, threshold=0.05,
               clip=true, angle=0.02, length={80, 200}, pressure={0.3, 0.5}})
work(mMist2, {hand="glaze", pile=mistCool, coverage=0.7, threshold=0.06,
              clip=true, angle=0.02, length={80, 200}, pressure={0.28, 0.45}})
print("mist edge")

--@ chunk 232
-- the bank edge: a thicker, more varied fringe of grass
gr = brush{kind = "rigger", width = 1.5, point = 1, stiffness = 0.4}
nzGrass = noise{seed=208, period=120, octaves=3}
for i = 1, 420 do
  local x = rand(-10, 1010)
  local dens = nzGrass:at01(x, 60)
  if rand() < 0.35 + 0.65 * dens then
    local by = shoreFn(x) + rand(-4, 16)
    local hgt = rand(4, 20) * (0.55 + 0.75 * dens)
    local lean = rand(-0.35, 0.35) - 0.1
    gr:load(rand() < 0.55 and grassDark or reedP, rand(0.25, 0.6))
    gr:stroke({{x, by}, {x + lean * hgt * 0.6, by - hgt * 0.65},
               {x + lean * hgt, by - hgt}},
              {pressure = {rand(0.4, 0.6), 0.35, 0.05}, orient = "across"})
  end
end
for _, spot in ipairs({{760, 668}, {860, 676}, {950, 686}, {700, 700},
                       {820, 716}, {900, 730}, {640, 736}, {740, 748}}) do
  for k = 1, 8 do
    local a = -math.pi / 2 + randn(0, 0.6)
    local hgt = rand(8, 20)
    gr:load(rand() < 0.5 and grassDark or reedP, rand(0.3, 0.6))
    gr:stroke({{spot[1] + randn(0, 6), spot[2]},
               {spot[1] + math.cos(a) * hgt * 0.6 + randn(0, 2),
                spot[2] + math.sin(a) * hgt * 0.6},
               {spot[1] + math.cos(a) * hgt + randn(0, 3),
                spot[2] + math.sin(a) * hgt}},
              {pressure = {0.55, 0.4, 0.03}, orient = "across"})
  end
end
print("grass edge")

--@ chunk 233
print(wait(2 * 24 * 60))
print("dry?", drying(300, 460))
mBandFix = mask(function(x, y)
  local d = y - 444
  local thick = 25 + 12 * nzMist:at01(x, 70) - 8 * x / 1000
  return clamp((d + 15) / 17, 0, 1) * clamp((thick - d) / 19, 0, 1)
end):blur(9)
mistFix = pile{{"lead white", 1.6}, {"pale smalt", 2.6}, {"raw umber", 1.5},
               {"vermilion", 0.06}}
work(mBandFix, {hand="body", pile=mistFix, coverage=2, fill=true,
                threshold=0.03, clip=true, angle=0.02, tool="filbert 6",
                length={60, 150}, pressure={0.45, 0.72}})
blend(mBandFix, {angle=0.03})
print("band smoothed")

--@ chunk 234
-- the join where the mist meets the water, dissolved
mJoin = mask(function(x, y)
  local d = y - 460
  return clamp(d / 11, 0, 1) * clamp((48 - d) / 24, 0, 1)
end):blur(8)
work(mJoin, {hand="body", pile=mistFix, coverage=1.4, fill=true,
             threshold=0.03, clip=true, angle=0.02, tool="filbert 6",
             length={60, 150}, pressure={0.45, 0.72}})
blend(mJoin, {angle=0.03})
print("join softened")

--@ chunk 235
-- the treed shore again, sitting above the mist this time
shoreP2 = pile{{"bone black", 1.4}, {"raw umber", 1.8}, {"pale smalt", 2.2}}
tbw = brush{kind = "rigger", width = 1.5, point = 0.9, stiffness = 0.4}
local cnt = 0
for i = 1, 300 do
  local x = rand(-8, 760)
  local dens = nzWood:at01(x * 1.4, 90)
  if rand() < 0.18 + 0.8 * dens then
    local gy = 424 + rand(-5, 8) + 4 * (x / 760)
    local hgt = rand(3.5, 11) * (0.6 + 0.6 * dens)
    local lean = randn(0, 0.05)
    tbw:load(shoreP2, rand(0.25, 0.55))
    tbw:stroke({{x, gy}, {x + lean * hgt * 0.5, gy - hgt * 0.6},
                {x + lean * hgt, gy - hgt}},
               {pressure = {0.5, 0.35, 0.05}, orient = "across"})
    cnt = cnt + 1
    for s = 1, 2 do
      local a = -math.pi / 2 + rand(-1.1, 1.1) - 0.25
      local L = hgt * rand(0.3, 0.55)
      local by = gy - hgt * rand(0.35, 0.8)
      tbw:load(shoreP2, rand(0.15, 0.38))
      tbw:stroke({{x, by}, {x + math.cos(a) * L, by + math.sin(a) * L}},
                 {pressure = {0.35, 0.02}, orient = "across"})
    end
  end
end
print("shore trees", cnt)

--@ chunk 236
print(wait(2 * 24 * 60))
print("dry?", drying(400, 485))
mFade = mask(function(x, y)
  local d = y - 472
  return clamp(d / 7, 0, 1) * clamp((36 - d) / 20, 0, 1)
end):blur(7)
work(mFade, {hand="body", pile=lakeD2, coverage=2.2, fill=true,
             threshold=0.03, clip=true, angle=0.02, tool="filbert 7",
             length={70, 180}, pressure={0.45, 0.72}})
blend(mFade, {angle=0.03})
print("bottom edge faded")

--@ chunk 237
print(wait(2 * 24 * 60))
print("dry?", drying(400, 490))
mFadeZone = mLake * mask(function(x, y)
  return clamp((y - 462) / 8, 0, 1) * clamp((530 - y) / 24, 0, 1)
end)
stipple(mFadeZone, {pile = lakeD2, tool = "filbert 3",
                    coverage = function(x, y)
                      return 2.6 * clamp((y - 466) / 42, 0, 1)
                    end,
                    pressure = {0.35, 0.6}, clip = mFadeZone,
                    cluster = {0.4, 30}, mix_jitter = 0.5})
print("graded fade")

--@ chunk 238
print(wait(2 * 24 * 60))
print("dry?", drying(400, 478))
mFade2 = mLake * mask(function(x, y)
  return clamp((y - 452) / 8, 0, 1) * clamp((500 - y) / 12, 0, 1)
end)
stipple(mFade2, {pile = mistFix, tool = "filbert 3",
                 coverage = function(x, y)
                   return 2 * clamp((494 - y) / 32, 0, 1)
                 end,
                 pressure = {0.3, 0.55}, clip = mFade2,
                 cluster = {0.4, 30}, mix_jitter = 0.5})
print("mist graded down")

--@ chunk 239
print(wait(2 * 24 * 60))
print("dry?", drying(780, 530), drying(300, 700))
-- the reflection's far end fades instead of ending in a pale raft
stipple(mPeakRefl, {pile = lakeD2, tool = "filbert 3",
                    coverage = function(x, y)
                      return 1.5 * clamp((y - 480) / 60, 0, 1)
                    end,
                    pressure = {0.3, 0.55}, clip = mPeakRefl,
                    cluster = {0.4, 26}, mix_jitter = 0.5})
-- and the bank's slope takes a little of the sky
mSlope = mask(function(x, y)
  local d = y - shoreFn(x)
  return clamp((d - 4) / 8, 0, 1) * clamp((62 - d) / 28, 0, 1)
end)
stipple(mShore, {pile = pile{{"raw umber", 3}, {"green earth", 1.6},
                             {"bone black", 1}, {"lead white", 0.9}},
                 tool = "filbert 4",
                 coverage = function(x, y)
                   local d = y - shoreFn(x)
                   return 1.5 * clamp((d - 6) / 10, 0, 1) * clamp((58 - d) / 26, 0, 1)
                 end,
                 pressure = {0.3, 0.55}, clip = mSlope,
                 cluster = {0.4, 34}, mix_jitter = 0.5})
print("reflection and bank slope")

--@ chunk 240
print(wait(2 * 24 * 60))
print("dry?", drying(300, 490), drying(400, 690))
-- the water band: smooth again
mSmooth = mLake * mask(function(x, y)
  return clamp((y - 450) / 10, 0, 1) * clamp((516 - y) / 14, 0, 1)
end):blur(6)
work(mSmooth, {hand="body", pile=mistFix, coverage=1.3, fill=true,
               threshold=0.03, clip=true, angle=0.02, tool="filbert 6",
               length={60, 150}, pressure={0.45, 0.72}})
blend(mSmooth, {angle=0.03})
-- and the bank's top, back to earth
mTop2 = mask(function(x, y)
  local d = y - shoreFn(x)
  return clamp((d - 2) / 8, 0, 1) * clamp((64 - d) / 24, 0, 1)
end) * mShore
work(mTop2, {hand="body", pile=bankQuiet, coverage=2.2, fill=true,
             threshold=0.03, clip=true, angle=-0.25, tool="filbert 5",
             length={20, 50}, pressure={0.5, 0.8}})
print("smoothed")

--@ chunk 241
-- the trunks and limbs again where the bank went over them
work(mAt, {hand="body", pile=treeDark, coverage=2.6, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.5, 0.85}})
work(mA, {hand="body", pile=treeDark, coverage=2.6, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.75}})
work(mBt, {hand="body", pile=treeFar, coverage=2.6, fill=true, clip=true,
           tool="filbert 6", length={20, 50}, pressure={0.45, 0.8}})
work(mB, {hand="body", pile=treeFar, coverage=2.5, fill=true, clip=true,
          tool="filbert 4", length={15, 45}, pressure={0.4, 0.7}})
-- and the grass along the edge of the water
gr = brush{kind = "rigger", width = 1.5, point = 1, stiffness = 0.4}
nzGrass = noise{seed=208, period=120, octaves=3}
for i = 1, 460 do
  local x = rand(-10, 1010)
  local dens = nzGrass:at01(x, 60)
  if rand() < 0.35 + 0.65 * dens then
    local by = shoreFn(x) + rand(-4, 16)
    local hgt = rand(4, 20) * (0.55 + 0.75 * dens)
    local lean = rand(-0.35, 0.35) - 0.1
    gr:load(rand() < 0.55 and grassDark or reedP, rand(0.3, 0.65))
    gr:stroke({{x, by}, {x + lean * hgt * 0.6, by - hgt * 0.65},
               {x + lean * hgt, by - hgt}},
              {pressure = {rand(0.45, 0.65), 0.4, 0.05}, orient = "across"})
  end
end
print("grass")

--@ chunk 242
print(wait(2 * 24 * 60))
-- a few stones on the bank, each with a lit top and a dark side
stoneP = pile{{"raw umber", 2.6}, {"bone black", 1.2}, {"pale smalt", 1}}
stoneL = pile{{"lead white", 1.2}, {"pale smalt", 1.6}, {"raw umber", 1.6}}
for _, s in ipairs({{168, 706, 9, 5}, {352, 742, 12, 6}, {690, 690, 7, 4},
                    {806, 722, 10, 5}, {930, 752, 8, 4}, {508, 700, 6, 3.5},
                    {258, 766, 11, 5}}) do
  local e = ellipse(s[1], s[2], s[3], s[4])
  work(e, {hand="body", pile=stoneP, coverage=2, fill=true, clip=true,
           tool="filbert 2.6", length={5, 14}, pressure={0.5, 0.8}})
  work(e:offset(0, -s[4] * 0.5) - e:offset(0, -s[4]), {hand="body", pile=stoneL,
        coverage=1, clip=true, tool="filbert 2", length={3, 10},
        pressure={0.3, 0.55}})
  work(e:offset(0, s[4] * 0.55) - e:offset(0, 0), {hand="body",
        pile=pile{{"raw umber", 2}, {"bone black", 2}, {"pale smalt", 0.6}},
        coverage=1, clip=true, tool="filbert 2", length={3, 10},
        pressure={0.3, 0.55}})
end
print("stones")

--@ chunk 243
darkGrass = pile{{"bone black", 2.4}, {"raw umber", 2}, {"pale smalt", 0.8}}
gr = brush{kind = "rigger", width = 1.6, point = 1, stiffness = 0.4}
nzGrass = noise{seed=208, period=120, octaves=3}
for i = 1, 520 do
  local x = rand(-10, 1010)
  local dens = nzGrass:at01(x, 60)
  if rand() < 0.35 + 0.65 * dens then
    local by = shoreFn(x) + rand(-5, 18)
    local hgt = rand(5, 22) * (0.55 + 0.75 * dens)
    local lean = rand(-0.35, 0.35) - 0.1
    gr:load(rand() < 0.6 and darkGrass or grassDark, rand(0.35, 0.7))
    gr:stroke({{x, by}, {x + lean * hgt * 0.6, by - hgt * 0.65},
               {x + lean * hgt, by - hgt}},
              {pressure = {rand(0.5, 0.7), 0.45, 0.06}, orient = "across"})
  end
end
for _, spot in ipairs({{760, 668}, {860, 676}, {950, 686}, {700, 700},
                       {820, 716}, {900, 730}, {640, 736}, {740, 748},
                       {120, 762}, {420, 768}, {600, 770}}) do
  for k = 1, 9 do
    local a = -math.pi / 2 + randn(0, 0.6)
    local hgt = rand(9, 22)
    gr:load(rand() < 0.5 and darkGrass or reedP, rand(0.35, 0.65))
    gr:stroke({{spot[1] + randn(0, 7), spot[2]},
               {spot[1] + math.cos(a) * hgt * 0.6 + randn(0, 2),
                spot[2] + math.sin(a) * hgt * 0.6},
               {spot[1] + math.cos(a) * hgt + randn(0, 3),
                spot[2] + math.sin(a) * hgt}},
              {pressure = {0.6, 0.45, 0.03}, orient = "across"})
  end
end
print("grass, darker")

--@ chunk 244
print(wait(2 * 24 * 60))
print("dry?", drying(500, 740))
-- the near ground falls into shadow: three graded passes
fg1 = mShore * mask(function(x, y) return clamp((y - 700) / 26, 0, 1) end)
fg2 = mShore * mask(function(x, y) return clamp((y - 736) / 26, 0, 1) end)
fg3 = mShore * mask(function(x, y) return clamp((y - 764) / 20, 0, 1) end)
work(fg1, {hand="body", pile=bankQuiet, coverage=1.3, threshold=0.03, clip=true,
           angle=-0.2, tool="filbert 6", length={40, 100}, pressure={0.4, 0.68}})
work(fg2, {hand="body", pile=earthDeep, coverage=1.3, threshold=0.03, clip=true,
           angle=-0.2, tool="filbert 6", length={40, 100}, pressure={0.4, 0.68}})
work(fg3, {hand="body", pile=pile{{"raw umber", 3.4}, {"bone black", 3},
       {"green earth", 0.8}}, coverage=1.4, threshold=0.03, clip=true,
       angle=-0.2, tool="filbert 6", length={40, 100}, pressure={0.4, 0.68}})
print("foreground shadow")

--@ chunk 245
-- the trunk's pale core: lay it back dark
work(mAt, {hand="body", pile=treeDark, coverage=3.6, fill=true, clip=true,
           tool="filbert 4", length={15, 40}, pressure={0.55, 0.9}})
work(mBt, {hand="body", pile=treeFar, coverage=3.4, fill=true, clip=true,
           tool="filbert 4", length={15, 40}, pressure={0.5, 0.85}})
print("trunks back to dark")

--@ chunk 246
print(wait(2 * 24 * 60))
-- a dark bed under the grass, so the fringe is vegetation and not nails
mBed = mask(function(x, y)
  local d = y - shoreFn(x)
  return clamp((d + 6) / 7, 0, 1) * clamp((26 - d) / 16, 0, 1)
end) * mShore
work(mBed, {hand="body", pile=pile{{"bone black", 2.2}, {"raw umber", 2.4},
      {"green earth", 0.8}}, coverage=1.5, threshold=0.03, clip=true,
      angle=-0.3, tool="filbert 4", length={20, 50}, pressure={0.45, 0.72}})
-- and finer grass over it, in clumps
gr = brush{kind = "rigger", width = 1.0, point = 1, stiffness = 0.35}
nzGrass = noise{seed=331, period=58, octaves=4}
for i = 1, 900 do
  local x = rand(-10, 1010)
  local dens = nzGrass:at01(x, 60)
  if rand() < 0.3 + 0.7 * dens then
    local by = shoreFn(x) + rand(-4, 20)
    local hgt = rand(5, 20) * (0.5 + 0.8 * dens)
    local lean = rand(-0.5, 0.5) - 0.12
    gr:load(rand() < 0.5 and darkGrass or grassDark, rand(0.3, 0.65))
    gr:stroke({{x, by}, {x + lean * hgt * 0.6, by - hgt * 0.65},
               {x + lean * hgt, by - hgt}},
              {pressure = {rand(0.45, 0.62), 0.34, 0.02}, orient = "across"})
  end
end
print("grass bed and blades")

--@ chunk 247
print(wait(2 * 24 * 60))
print("dry?", drying(200, 540))
-- a little life in the left of the water: long, low, quiet
bW = brush{kind = "filbert", width = 7, point = 0.2, stiffness = 0.5}
for i = 1, 9 do
  local x = rand(-20, 380)
  local y = rand(478, 560)
  local L = rand(150, 330)
  bW:load(reflP2, rand(0.12, 0.26))
  bW:stroke({{x, y}, {x + L * 0.4, y + randn(0, 1.4)}, {x + L * 0.75, y - randn(0, 1)},
             {x + L, y}},
            {pressure = {0.02, 0.1, 0.08, 0.02}, orient = "across", clip = mLake})
end
for i = 1, 7 do
  local x = rand(-20, 400)
  local y = rand(500, 625)
  local L = rand(120, 290)
  bW:load(darkRip, rand(0.12, 0.24))
  bW:stroke({{x, y}, {x + L * 0.4, y + randn(0, 1.4)}, {x + L * 0.75, y - randn(0, 1)},
             {x + L, y}},
            {pressure = {0.02, 0.1, 0.08, 0.02}, orient = "across", clip = mLake})
end
print("left water")

--@ chunk 248
print(wait(2 * 24 * 60))
print("dry?", draining and 0 or 0)
print("dry?", drying(300, 480), drying(130, 500))
bandA = mLake * mask(function(x, y)
  local d = y - 462
  return clamp(d / 7, 0, 1) * clamp((28 - d) / 8, 0, 1)
end)
bandB = mLake * mask(function(x, y)
  local d = y - 476
  return clamp(d / 9, 0, 1) * clamp((32 - d) / 10, 0, 1)
end)
bandC = mLake * mask(function(x, y)
  local d = y - 494
  return clamp(d / 10, 0, 1) * clamp((36 - d) / 14, 0, 1)
end)
work(bandA, {hand="body", pile=mistFix, coverage=1.9, fill=true, threshold=0.03,
             clip=true, angle=0.02, tool="filbert 6", length={60, 150},
             pressure={0.45, 0.72}})
work(bandB, {hand="body", pile=mistFix, coverage=1.5, fill=true, threshold=0.03,
             clip=true, angle=0.02, tool="filbert 6", length={60, 150},
             pressure={0.45, 0.72}})
work(bandC, {hand="body", pile=lakeD2, coverage=1.5, fill=true, threshold=0.03,
             clip=true, angle=0.02, tool="filbert 6", length={60, 150},
             pressure={0.45, 0.72}})
blend(mLake, {angle=0.03})
print("band graded")

--@ chunk 249
print(wait(2 * 24 * 60))
-- the trunks and limbs again over the band
work(mAt, {hand="body", pile=treeDark, coverage=3.2, fill=true, clip=true,
           tool="filbert 4", length={15, 40}, pressure={0.55, 0.9}})
work(mA, {hand="body", pile=treeDark, coverage=3, fill=true, clip=true,
          tool="filbert 3.5", length={12, 35}, pressure={0.5, 0.85}})
work(mBt, {hand="body", pile=treeFar, coverage=3.2, fill=true, clip=true,
           tool="filbert 4", length={15, 40}, pressure={0.5, 0.85}})
work(mB, {hand="body", pile=treeFar, coverage=3, fill=true, clip=true,
          tool="filbert 3.5", length={12, 35}, pressure={0.5, 0.85}})
print("trees")

--@ chunk 250
for _, nm in ipairs({"mAt", "mA", "mB", "mBt", "standing", "m1", "m1b", "mWood", "L", "R"}) do
  local m = _G[nm]
  if m then print(nm, string.format("%.2f %.2f %.2f", m:at(255, 500), m:at(255, 600), m:at(122, 500))) end
end

--@ chunk 251
print(wait(2 * 24 * 60))
print("dry?", drying(255, 500), drying(122, 500))
-- the right trunk's core has gone pale: lay it dark and opaque
veryDark = pile{{"bone black", 5}, {"raw umber", 2}, {"pale smalt", 0.5}}
work(mB, {hand="body", pile=veryDark, coverage=4, fill=true, clip=true,
          tool="filbert 3", length={12, 30}, pressure={0.6, 0.95}})
work(mAt, {hand="body", pile=veryDark, coverage=3, fill=true, clip=true,
           tool="filbert 4", length={12, 30}, pressure={0.55, 0.9}})
print("trunks laid")

--@ chunk 252
print(mB:at(252, 730), mB:at(250, 700), mB:at(246, 760), mAt:at(120, 760))
print(wait(2 * 24 * 60))
tA = brush{kind = "filbert", width = 19, point = 0, stiffness = 0.55}
tA:load(veryDark, 0.9)
tA:stroke({{253, 640}, {248, 710}, {243, 800}},
          {pressure = {0.7, 0.85, 0.95}, orient = "across"})
tB = brush{kind = "filbert", width = 24, point = 0, stiffness = 0.55}
tB:load(veryDark, 0.9)
tB:stroke({{114, 660}, {106, 730}, {100, 810}},
          {pressure = {0.7, 0.9, 0.95}, orient = "across"})
print("trunks to the bottom")

--@ chunk 253
print(wait(2 * 24 * 60))
print("dry?", drying(200, 300), drying(300, 500))
rimP = pile{{"lead white", 1.5}, {"yellow ochre", 1.3}, {"raw umber", 1.6},
            {"pale smalt", 0.6}}
rb = brush{kind = "rigger", width = 0.9, point = 1, stiffness = 0.4}
local cnt = 0
for _, list in ipairs({branches, crown}) do
  for _, b in ipairs(list) do
    local n = #b.pts
    for i = math.max(2, math.floor(n * 0.4)), n - 1 do
      local x, y = b.pts[i][1], b.pts[i][2]
      local nx, ny = b.pts[i + 1][1], b.pts[i + 1][2]
      local dx, dy = nx - x, ny - y
      local len = math.sqrt(dx * dx + dy * dy)
      if len > 1 then
        local t = i / n
        local w = b.w0 + (b.w1 - b.w0) * t
        if w > 2 and y < 700 then
          -- the right-hand side of the limb, where the last light is
          local ox, oy = dy / len * w * 0.42, -dx / len * w * 0.42
          rb:load(rimP, rand(0.2, 0.45))
          rb:stroke({{x + ox, y + oy},
                     {(x + nx) * 0.5 + ox, (y + ny) * 0.5 + oy},
                     {nx + ox, ny + oy}},
                    {pressure = {0.05, rand(0.25, 0.45), 0.03},
                     orient = "across"})
          cnt = cnt + 1
        end
      end
    end
  end
end
print("rims", cnt)

--@ chunk 254
print(wait(2 * 24 * 60))
print("dry?", drying(200, 300))
work(mAt, {hand="body", pile=veryDark, coverage=2.6, fill=true, clip=true,
           tool="filbert 4", length={15, 40}, pressure={0.55, 0.9}})
work(mA, {hand="body", pile=treeDark, coverage=2.6, fill=true, clip=true,
          tool="filbert 3.5", length={12, 35}, pressure={0.5, 0.85}})
work(mBt, {hand="body", pile=veryDark, coverage=2.6, fill=true, clip=true,
           tool="filbert 4", length={15, 40}, pressure={0.5, 0.85}})
work(mB, {hand="body", pile=veryDark, coverage=2.6, fill=true, clip=true,
          tool="filbert 3.5", length={12, 35}, pressure={0.5, 0.85}})
print("limbs")

--@ chunk 255
rimP2 = pile{{"lead white", 1.2}, {"pale smalt", 2}, {"raw umber", 2},
             {"yellow ochre", 0.25}}
rb = brush{kind = "rigger", width = 0.8, point = 1, stiffness = 0.4}
local cnt = 0
for _, list in ipairs({branches, crown}) do
  for _, b in ipairs(list) do
    local n = #b.pts
    for i = math.max(2, math.floor(n * 0.35)), n - 2 do
      local x, y = b.pts[i][1], b.pts[i][2]
      local w = b.w0 + (b.w1 - b.w0) * (i / n)
      if w > 5 and y < 700 then
        local nx, ny = b.pts[i + 1][1], b.pts[i + 1][2]
        local dx, dy = nx - x, ny - y
        local len = math.sqrt(dx * dx + dy * dy)
        if len > 3 and rand() < 0.55 then
          local ox, oy = dy / len * w * 0.4, -dx / len * w * 0.4
          rb:load(rimP2, rand(0.12, 0.28))
          rb:stroke({{x + ox, y + oy}, {nx + ox, ny + oy}},
                    {pressure = {rand(0.1, 0.3), 0.03}, orient = "across"})
          cnt = cnt + 1
        end
      end
    end
  end
end
print("rims", cnt)

--@ chunk 256
for _, n in ipairs{"t","t2","t3","t4"} do
  local m = _G[n]
  print(n, m:area(), m:at(120, 500), m:at(600, 200), m:at(500, 700))
end
print("trunk wet?", drying(120, 500), drying(250, 300), drying(500, 700), drying(60, 620))

--@ chunk 257
local pts = {{120,100},{120,300},{122,450},{125,600},{128,750},
            {255,150},{258,300},{260,450},{260,600},{262,700},
            {210,250},{300,120},{60,300},{80,500},{200,560}}
for _, p in ipairs(pts) do
  print(p[1], p[2], drying(p[1], p[2]))
end

--@ chunk 258
local ys = {100, 300, 500, 700}
for _, n in ipairs{"t","t2","t3"} do
  local m = _G[n]
  local s = {}
  for _, y in ipairs(ys) do
    s[#s+1] = string.format("x120:%0.2f x135:%0.2f x145:%0.2f x265:%0.2f x272:%0.2f",
      m:at(120,y), m:at(135,y), m:at(145,y), m:at(265,y), m:at(272,y))
  end
  print(n, m:area())
  for _, l in ipairs(s) do print("   y", l) end
end

--@ chunk 259
print(wait(3 * 24 * 60))
print("trunk A", drying(120, 600), drying(125, 600), drying(135, 300))
print("trunk B", drying(260, 450), drying(265, 700))
-- a dim warm grey for the lit flank of a trunk: dusk light, not daylight
litFlank = pile{ {"lead white",1.0}, {"raw umber",1.3}, {"pale smalt",0.9}, {"yellow ochre",0.35} }
-- a cooler, deeper grey for the shadow side
coreDark = pile{ {"bone black",1.6}, {"raw umber",2.0}, {"pale smalt",1.2} }
print(litFlank, coreDark)

--@ chunk 260
-- modelling the two near trunks: a dim warm light down the right flank
flankLit = pile{ {"lead white",0.8}, {"raw umber",1.5}, {"pale smalt",1.0}, {"yellow ochre",0.3} }
fb5 = brush("filbert", 5)

local segA = {
  {{167, -6}, {164, 40}, {161, 86}, {158, 132}},
  {{158, 132}, {155, 180}, {152, 228}, {150, 272}},
  {{150, 272}, {148, 318}, {145, 366}, {143, 412}},
  {{143, 412}, {141, 458}, {139, 504}, {136, 548}},
  {{136, 548}, {134, 592}, {132, 638}, {129, 682}},
  {{129, 682}, {127, 722}, {126, 752}, {125, 788}},
}
local segB = {
  {{270, -6}, {271, 40}, {272, 88}, {273, 134}},
  {{273, 134}, {274, 180}, {275, 228}, {276, 274}},
  {{276, 274}, {277, 320}, {277, 366}, {277, 412}},
  {{277, 412}, {277, 458}, {276, 504}, {275, 548}},
  {{275, 548}, {274, 592}, {273, 638}, {272, 682}},
  {{272, 682}, {271, 722}, {270, 752}, {269, 788}},
}
for _, seg in ipairs(segA) do
  fb5:load(flankLit, 0.5)
  fb5:stroke(seg, {pressure={0.12, 0.30, 0.14}, ramps={0.22, 0.3}})
end
for _, seg in ipairs(segB) do
  fb5:load(flankLit, 0.5)
  fb5:stroke(seg, {pressure={0.12, 0.30, 0.14}, ramps={0.22, 0.3}})
end
fb5:wipe(0.9)
print("flank light on both trunks")

--@ chunk 261
-- that stripe reads as a zipper: too light, too even. Knock it back to trunk colour.
trunkCol = pile{ {"bone black",3.0}, {"raw umber",2.4}, {"pale smalt",1.2} }
print(trunkCol)
bc = brush("filbert", 9)
local pathA = {{167,-6},{164,40},{161,86},{158,132},{155,180},{152,228},{150,272},
               {148,318},{145,366},{143,412},{141,458},{139,504},{136,548},
               {134,592},{132,638},{129,682},{127,722},{126,752},{125,788}}
local pathB = {{270,-6},{271,40},{272,88},{273,134},{274,180},{275,228},{276,274},
               {277,320},{277,366},{277,412},{277,458},{276,504},{275,548},
               {274,592},{273,638},{272,682},{271,722},{270,752},{269,788}}
for _, path in ipairs{pathA, pathB} do
  for i = 1, #path - 1 do
    bc:load(trunkCol, 0.6)
    bc:stroke({path[i], path[i+1]}, {pressure={0.42, 0.34}, ramps={0.15, 0.2}})
  end
end
bc:wipe(0.9)
print("covered")

--@ chunk 262
spineA = {{158,-20},{155,60},{151,150},{147,240},{144,330},{141,420},{138,510},{135,600},{132,690},{129,800}}
spineB = {{270,-20},{271,60},{272,150},{273,240},{274,330},{275,420},{276,510},{275,600},{273,690},{271,800}}
trA = body_of{spine=spineA, widths=15}:mask()
trB = body_of{spine=spineB, widths=14}:mask()
local function chk(n, m, ins, outs)
  local a = {}
  for _, p in ipairs(ins) do a[#a+1] = string.format("%0.2f", m:at(p[1],p[2])) end
  print(n, "inside ", table.concat(a, " "))
  local b = {}
  for _, p in ipairs(outs) do b[#b+1] = string.format("%0.2f", m:at(p[1],p[2])) end
  print(n, "outside", table.concat(b, " "))
end
chk("A", trA, {{137,500},{161,100},{126,700},{143,300}}, {{115,500},{178,100},{105,700},{150,780}})
chk("B", trB, {{267,500},{262,100},{261,700},{274,300}}, {{248,500},{283,100},{240,700},{280,780}})
print(trA:area(), trB:area())

--@ chunk 263
spineA = {{158,-20},{155,60},{151,150},{147,240},{144,330},{141,420},{138,510},{135,600},{132,690},{129,800}}
spineB = {{261,-20},{262,60},{263,150},{264,240},{265,330},{266,420},{267,510},{267,600},{266,690},{265,800}}
trA = body_of{spine=spineA, widths=14}:mask()
trB = body_of{spine=spineB, widths=13}:mask()
local function chk(n, m, ins, outs)
  local a = {}
  for _, p in ipairs(ins) do a[#a+1] = string.format("%0.2f", m:at(p[1],p[2])) end
  print(n, "inside ", table.concat(a, " "))
  local b = {}
  for _, p in ipairs(outs) do b[#b+1] = string.format("%0.2f", m:at(p[1],p[2])) end
  print(n, "outside", table.concat(b, " "))
end
chk("A", trA, {{137,500},{161,100},{126,700},{143,300}}, {{115,500},{178,100},{105,700},{150,780}})
chk("B", trB, {{266,500},{262,100},{266,700},{265,300}}, {{250,500},{280,100},{242,700},{280,780}})

--@ chunk 264
spineA = {{160,-30},{159,100},{156,300},{152,450},{148,600},{143,750},{141,820}}
trA = body_of{spine=spineA, widths=9.5}:mask()
local a, b = {}, {}
for _, p in ipairs{{159,100},{155,300},{151,450},{147,600},{143,750},{150,50}} do a[#a+1]=string.format("%0.2f", trA:at(p[1],p[2])) end
for _, p in ipairs{{168,100},{164,300},{160,450},{156,600},{152,750},{144,300}} do b[#b+1]=string.format("%0.2f", trA:at(p[1],p[2])) end
print("centre", table.concat(a," "))
print("left  ", table.concat(b," "))
print(trA:area())

--@ chunk 265
trunkCol = pile{ {"bone black",3.0}, {"raw umber",2.4}, {"pale smalt",1.2} }
trunks = (trA + trB):grow(1)
work(trunks, {hand="body", tool="filbert 7", pile=trunkCol, coverage=2,
              clip=true, load=0.45, angle=1.5708, pressure={0.3,0.45},
              edge="firm"})
print("even pass on the trunks")

--@ chunk 266
trunkCol2 = pile{ {"bone black",5.0}, {"raw umber",2.2}, {"pale smalt",0.8} }
print(trunkCol2)
work(trunks, {hand="body", tool="filbert 9", pile=trunkCol2, coverage=3, fill=true,
              clip=true, load=0.5, angle=1.5708, pressure={0.35,0.55}, edge="firm"})
print("even pass, darker")

--@ chunk 267
-- the sun is down behind the right-hand hill: the two trees throw long soft
-- shadows across the bank to the left, and darken the ground at their feet
shadowP = pile{ {"bone black",2.2}, {"raw umber",2.6}, {"pale smalt",1.1} }
shA = ribbon({{152, 688}, {126, 698}, { 92, 712}, { 52, 726}, { 14, 740}}, {9, 15, 20, 24, 26})
shB = ribbon({{266, 686}, {236, 700}, {198, 714}, {156, 728}, {116, 740}}, {9, 15, 20, 24, 26})
shC = ribbon({{268, 700}, {246, 716}, {214, 732}, {176, 748}}, {8, 14, 18, 20})
work(shA, {hand="body", tool="filbert 12", pile=shadowP, coverage=2, load=0.4,
           pressure={0.18, 0.30}, edge="lost", clip=true, angle=0.1})
work(shB, {hand="body", tool="filbert 12", pile=shadowP, coverage=2, load=0.4,
           pressure={0.18, 0.30}, edge="lost", clip=true, angle=0.1})
print("shadows laid in")

--@ chunk 268
for _, p in ipairs{{100,700},{200,690},{300,700},{60,740},{350,660},{500,700}} do
  print(p[1], p[2], drying(p[1],p[2]))
end

--@ chunk 269
-- those pale scars: the shadow strokes lifted paint that was still open.
-- Start again on the bank with an even ground of its own colour, trees kept out.
nzBank = noise{seed=17, octaves=3, period=170, persistence=0.5}
bankCol = pile{ {"raw umber",3.0}, {"yellow ochre",1.2}, {"pale smalt",1.2}, {"bone black",0.8} }
bankM = below(function(x) return 676 - 0.02 * x + 9 * nzBank(x, 4) end)
bankM = bankM - (trA + trB):grow(4)
work(bankM, {hand="body", tool="filbert 11", pile=bankCol, coverage=2, clip=true,
             load=0.5, pressure={0.3, 0.5}, edge="soft"})
print(bankM:area())

--@ chunk 270
print(wait(2 * 24 * 60))
print(drying(200, 700), drying(400, 720), drying(100, 690))

--@ chunk 271
bankDark2 = pile{ {"raw umber",3.2}, {"yellow ochre",0.8}, {"pale smalt",1.4}, {"bone black",2.2} }
print(bankDark2)
work(bankM, {hand="body", tool="filbert 11", pile=bankDark2, coverage=2, fill=true,
             clip=true, edge="soft",
             load_at=function(x, y) return 0.34 + 0.34 * (0.5 + 0.5 * nzBank(x * 0.6, y * 1.4)) end,
             pressure={0.35, 0.55}})
print("bank darkened, mottled")

--@ chunk 272
bankDark3 = pile{ {"raw umber",3.4}, {"yellow ochre",0.4}, {"pale smalt",1.8}, {"bone black",3.0} }
bankM2 = below(function(x) return 674 - 0.015 * x + 5 * nzBank(x, 7) end)
bankM2 = bankM2 - (trA + trB):grow(4)
work(bankM2, {hand="body", tool="filbert 12", pile=bankDark3, coverage=2, fill=true,
              clip=true, edge="lost",
              load_at=function(x, y) return 0.40 + 0.30 * (0.5 + 0.5 * nzBank(x * 0.5, y * 1.2)) end,
              pressure={0.4, 0.6}})
print("bank, darker and cooler")

--@ chunk 273
print(wait(2 * 24 * 60))
print(drying(300, 700), drying(400, 750))
-- the bank wants a lit ridge along the top and a deep foreground
fgM = below(function(x) return 744 + 7 * nzBank(x, 11) end) - (trA + trB):grow(4)
fgP  = pile{ {"bone black",2.6}, {"raw umber",2.6}, {"pale smalt",1.2} }
work(fgM, {hand="body", tool="filbert 12", pile=fgP, coverage=2, fill=true,
           clip=true, edge="lost", pressure={0.35, 0.5},
           load_at=function(x, y) return 0.35 + 0.25 * (0.5 + 0.5 * nzBank(x * 0.5, y)) end})

ridgeM = (below(function(x) return 666 + 4 * nzBank(x, 3) end)
        - below(function(x) return 694 + 7 * nzBank(x, 13) end)) * bankM2
ridgeP = pile{ {"raw umber",2.2}, {"yellow ochre",1.0}, {"pale smalt",1.2}, {"lead white",0.5} }
work(ridgeM, {hand="body", tool="filbert 8", pile=ridgeP, coverage=2, fill=true,
              clip=true, edge="soft", pressure={0.2, 0.35},
              load_at=function(x, y) return 0.3 + 0.25 * (0.5 + 0.5 * nzBank(x, y * 2)) end})
print("ridge and foreground")

--@ chunk 274
print(wait(2 * 24 * 60))
ridgeP2 = pile{ {"raw umber",2.6}, {"yellow ochre",0.9}, {"pale smalt",1.0}, {"bone black",1.6} }
work(ridgeM, {hand="body", tool="filbert 9", pile=ridgeP2, coverage=2, fill=true,
              clip=true, edge="soft", pressure={0.3, 0.45},
              load_at=function(x, y)
                local v = nzBank(x * 1.1, y * 2.2)
                return 0.26 + 0.34 * clamp(0.5 + 0.9 * v, 0, 1)
              end})
print("ridge, darker")

--@ chunk 275
-- incident on the bank: stones with a shadow under each, dry stalks, a bleached stick
stoneP  = pile{ {"raw umber",2.4}, {"pale smalt",1.6}, {"lead white",0.3}, {"bone black",0.5} }
stalkP  = pile{ {"raw umber",2.0}, {"yellow ochre",1.4}, {"pale smalt",0.8} }
stickP  = pile{ {"lead white",0.8}, {"raw umber",1.7}, {"pale smalt",1.0}, {"yellow ochre",0.3} }
bs = brush{kind="round", width=3.4, point=0.6}
bk = brush{kind="rigger", width=1.6, point=0.9}

local stones = {{352,712,2.6},{434,736,3.2},{520,700,2.2},{612,746,3.6},
                {702,719,2.8},{792,761,4.0},{880,701,2.4},{958,741,3.0},{332,762,2.4}}
for _, s in ipairs(stones) do
  bs:load(stoneP, 0.6); bs:touch(s[1], s[2], {pressure=0.55})
  bs:wipe(0.5)
end
bs:wipe(0.9)
for _, s in ipairs(stones) do
  bs:load(fgP, 0.5)
  bs:stroke({{s[1]-s[3]*0.9, s[2]+s[3]*0.7}, {s[1]+s[3]*0.2, s[2]+s[3]*1.0}, {s[1]+s[3]*1.1, s[2]+s[3]*0.6}},
            {pressure={0.4, 0.3}, ramps={0.2, 0.3}})
  bs:wipe(0.5)
end
bs:wipe(0.9)

local stalks = {{{380,726},{390,708},{394,698}}, {{470,702},{462,686}},
               {{560,762},{574,744},{580,736}}, {{642,716},{652,698},{656,690}},
               {{762,692},{752,674},{748,666}}, {{842,746},{858,730},{862,722}},
               {{930,706},{944,690},{948,682}}, {{302,702},{292,688}},
               {{580,700},{596,690}}, {{866,700},{880,690}}}
for _, s in ipairs(stalks) do
  bk:load(stalkP, 0.55); bk:stroke(s, {pressure={0.5, 0.12}, ramps={0.1, 0.2}})
  bk:wipe(0.4)
end
bk:wipe(0.9)
print("stones and stalks")

--@ chunk 276
print(wait(3 * 24 * 60))
-- the bank is the same value as the water, so it does not read as foreground.
-- Make it the darkest mass in the picture but for the trees.
bankDk = pile{ {"raw umber",3.2}, {"yellow ochre",0.5}, {"pale smalt",1.4}, {"bone black",3.6} }
print(bankDk)
work(bankM2, {hand="body", tool="filbert 12", pile=bankDk, coverage=2, fill=true,
              clip=true, edge="lost", pressure={0.4, 0.6},
              load_at=function(x, y)
                return 0.42 + 0.34 * (0.5 + 0.5 * nzBank(x * 0.45, y * 0.9))
              end})
print("bank, deep")

--@ chunk 277
print(wait(3 * 24 * 60))
-- light accidents on the dark bank: tussock tops, a few stones, one bleached stick
tussP  = pile{ {"raw umber",1.6}, {"yellow ochre",1.2}, {"pale smalt",1.0} }
stoneP = pile{ {"raw umber",2.2}, {"pale smalt",1.8}, {"lead white",0.35}, {"bone black",0.8} }
stickP = pile{ {"lead white",0.7}, {"raw umber",1.9}, {"pale smalt",1.2}, {"yellow ochre",0.25} }
bg = brush{kind="round", width=3.0, point=0.7}
bl = brush{kind="rigger", width=2.0, point=0.9}

-- tussocks: short blades leaning, in little clumps
local tuss = {
  {{348,704},{352,694},{350,686},{356,697}},
  {{352,700},{344,692},{342,684}},
  {{558,718},{564,708},{566,698},{558,708}},
  {{564,712},{556,704}},
  {{758,692},{764,682},{768,676}},
  {{928,722},{936,712},{938,704},{930,714}},
  {{936,716},{944,708}},
}
for _, t in ipairs(tuss) do
  for _, s in ipairs(t) do
    bl:load(tussP, 0.5); bl:stroke({s, {s[1] + rand(-2,2), s[2] - rand(5,9)}},
      {pressure={0.45, 0.08}, ramps={0.1, 0.25}})
    bl:wipe(0.35)
  end
end
-- stones: a dab, then a dark crescent under it
for _, s in ipairs{{432,731,4.2},{688,757,5.0},{872,703,3.4},{500,676,3.0},{808,742,4.4}} do
  bg:load(stoneP, 0.55); bg:touch(s[1], s[2], {pressure=0.5}); bg:wipe(0.5)
  bg:load(fgP, 0.45)
  bg:stroke({{s[1]-s[3], s[2]+s[3]*0.55}, {s[1], s[2]+s[3]*0.85}, {s[1]+s[3]*0.9, s[2]+s[3]*0.4}},
            {pressure={0.35, 0.25}, ramps={0.2, 0.3}})
  bg:wipe(0.5)
end
-- one stick, lying where the light reaches it
bl:load(stickP, 0.5)
bl:stroke({{812,734},{836,729},{862,728},{884,731}}, {pressure={0.5,0.35,0.4,0.1}, ramps={0.1,0.2}})
bl:wipe(0.9)
print("bank incident")

--@ chunk 278
print(wait(3 * 24 * 60))
-- the yellow tussocks and the stone crescents read as sparks and eyes.
-- Knock them back into the bank, then put quieter marks in their place.
nzBank2 = noise{seed=29, octaves=3, period=210, persistence=0.55}
scrub = {}
for _, e in ipairs{{350,694,16},{561,708,15},{763,684,13},{936,712,14},
                   {432,732,9},{688,758,10},{872,704,8},{500,677,8},{808,743,9}} do
  scrub[#scrub+1] = ellipse(e[1], e[2], e[3], e[3]*0.8)
end
scrubM = scrub[1]
for i = 2, #scrub do scrubM = scrubM + scrub[i] end
scrubM = (scrubM:grow(3)):blur(1)
work(scrubM, {hand="body", tool="filbert 7", pile=bankDk, coverage=3, fill=true,
              clip=true, edge="soft", pressure={0.35, 0.55},
              load_at=function(x, y) return 0.42 + 0.34 * (0.5 + 0.5 * nzBank2(x * 0.45, y * 0.9)) end})
print("scrubbed back into the bank")

--@ chunk 279
print(wait(3 * 24 * 60))
tussP2 = pile{ {"raw umber",2.4}, {"pale smalt",1.5}, {"lead white",0.35}, {"bone black",1.2}, {"yellow ochre",0.3} }
rimP3  = pile{ {"raw umber",2.6}, {"pale smalt",1.8}, {"lead white",0.5}, {"bone black",0.8} }
bl = brush{kind="rigger", width=2.0, point=0.9}
br = brush{kind="rigger", width=2.6, point=0.85}

local clumps = {{95,700},{250,712},{390,700},{600,706},{770,724},{930,700}}
for _, c in ipairs(clumps) do
  local x0, y0 = c[1], c[2]
  for i = 1, 7 do
    local x = x0 + rand(-7, 7)
    local y = y0 + rand(-3, 4)
    local dx, dy = rand(-4, 4), -rand(6, 12)
    bl:load(tussP2, 0.5)
    bl:stroke({{x, y}, {x + dx * 0.5, y + dy * 0.6}, {x + dx, y + dy}},
              {pressure={0.4, 0.06}, ramps={0.12, 0.3}})
    bl:wipe(0.35)
  end
end
bl:wipe(0.9)
br:load(rimP3, 0.6)
br:stroke({{590,644},{610,641},{632,639},{654,641},{674,644},{690,646}},
          {pressure={0.5,0.35,0.5,0.3,0.45,0.15}, ramps={0.12, 0.25}})
br:wipe(0.9)
print("tussocks and the boat's rim")

--@ chunk 280
scrub2 = {}
for _, e in ipairs{{95,700,16},{250,712,16},{390,700,16},{600,706,16},{770,724,16},{930,700,16}} do
  scrub2[#scrub2+1] = ellipse(e[1], e[2], e[3], e[3]*0.75)
end
scrubM2 = scrub2[1]
for i = 2, #scrub2 do scrubM2 = scrubM2 + scrub2[i] end
scrubM2 = (scrubM2:grow(3)):blur(1)
work(scrubM2, {hand="body", tool="filbert 7", pile=bankDk, coverage=3, fill=true,
               clip=true, edge="soft", pressure={0.35, 0.55},
               load_at=function(x, y) return 0.42 + 0.34 * (0.5 + 0.5 * nzBank2(x * 0.45, y * 0.9)) end})
print("scratch marks scrubbed back")

--@ chunk 281
print(wait(3 * 24 * 60))
tussP3 = pile{ {"raw umber",3.2}, {"yellow ochre",0.6}, {"pale smalt",1.4}, {"bone black",2.2}, {"lead white",0.5} }
for _, c in ipairs{{95,702},{250,714},{390,702},{600,708},{770,726},{930,702}} do
  local m = ellipse(c[1], c[2], 15, 11)
  stipple(m, {pile=tussP3, width=2.6, tool={kind="stippler", width=2.6},
              coverage=function(x, y)
                local d = ((x - c[1]) / 15) ^ 2 + ((y - c[2]) / 11) ^ 2
                return 0.9 * clamp(1.25 - d, 0, 1)
              end,
              pressure={0.45, 0.7}, drag={2.4, -1.1}, twist=0.3,
              cluster=0.55, dips={40, 0.5, 0.3}})
end
print("scumbled grass on the bank")

--@ chunk 282
scrub3 = {}
for _, e in ipairs{{95,702,18},{250,714,18},{390,702,18},{600,708,18},{770,726,18},{930,702,18}} do
  scrub3[#scrub3+1] = ellipse(e[1], e[2], e[3], e[3]*0.8)
end
scrubM3 = scrub3[1]
for i = 2, #scrub3 do scrubM3 = scrubM3 + scrub3[i] end
scrubM3 = (scrubM3:grow(4)):blur(2)
work(scrubM3, {hand="body", tool="filbert 7", pile=bankDk, coverage=3, fill=true,
               clip=true, edge="soft", pressure={0.35, 0.55},
               load_at=function(x, y) return 0.42 + 0.34 * (0.5 + 0.5 * nzBank2(x * 0.45, y * 0.9)) end})
print("stipple scrubbed back")

--@ chunk 283
print(drying(150, 300), drying(270, 500), drying(120, 650))
-- bark: a few dark knots and checks across each trunk, then two or three
-- short touches of the last light on the right flank of each
knotP = pile{ {"bone black",4.0}, {"raw umber",2.0}, {"pale smalt",0.6} }
bknot = brush{kind="rigger", width=2.4, point=0.6}
local knots = {
  {{130,120},{146,124}}, {{128,205},{143,202}}, {{135,268},{150,272}},
  {{131,340},{146,336}}, {{137,415},{150,420}}, {{130,486},{144,483}},
  {{127,556},{141,560}}, {{132,630},{146,627}}, {{126,706},{140,710}},
  {{258,150},{272,146}}, {{255,232},{270,236}}, {{259,310},{273,306}},
  {{254,388},{268,392}}, {{258,466},{272,462}}, {{253,548},{267,552}},
  {{256,628},{270,624}}, {{251,710},{265,714}},
}
for _, k in ipairs(knots) do
  bknot:load(knotP, 0.55)
  bknot:stroke(k, {pressure={0.35, 0.2}, ramps={0.2, 0.3}})
  bknot:wipe(0.4)
end
bknot:wipe(0.9)
print("bark knots")

--@ chunk 284
rimP4 = pile{ {"lead white",0.5}, {"raw umber",2.6}, {"pale smalt",1.6}, {"bone black",1.2}, {"yellow ochre",0.2} }
brim = brush{kind="rigger", width=2.2, point=0.95}
local rim = {
  {{165,120},{163,180}}, {{160,255},{157,320}}, {{153,415},{150,470}}, {{144,585},{141,640}},
  {{275,140},{275,200}}, {{275,280},{276,340}}, {{276,430},{277,490}}, {{277,590},{276,650}},
}
for _, r in ipairs(rim) do
  brim:load(rimP4, 0.45)
  brim:stroke(r, {pressure={0.3, 0.18}, ramps={0.2, 0.3}})
  brim:wipe(0.4)
end
brim:wipe(0.9)
print("rim light on both trunks")

--@ chunk 285
-- the rim lights read as strips of tape: too light, and set in from the edge
-- rather than on it. Three attempts at modelling these trunks; the dark
-- silhouette is what the picture wants. Knock them back and leave them be.
work(trA + trB, {hand="body", tool="filbert 9", pile=trunkCol2, coverage=2, fill=true,
                clip=true, load=0.5, angle=1.5708, pressure={0.35, 0.5}, edge="firm"})
print("trunks back to silhouette")

--@ chunk 286
print(drying(400, 700), drying(900, 690), drying(200, 750))
-- rather than lifting the dead bank again, sink it: a thin warm glaze
-- deepens and unifies it without changing its value much
warmGlz = pile{ {"raw umber",2.2}, {"bone black",1.0}, {"yellow ochre",0.3}, {"pale smalt",0.6}, medium=0.35 }
bankOnly = bankM2 - (trA + trB):grow(3)
work(bankOnly, {hand="glaze", pile=warmGlz, coverage=1, clip=true, edge="lost",
                pressure={0.12, 0.2}, angle=0.05})
print("bank glazed")

--@ chunk 287
print(wait(4 * 24 * 60))
print(drying(400, 700), drying(700, 730), drying(900, 700))
work(bankOnly, {hand="body", tool="filbert 12", pile=bankDk, coverage=3, fill=true,
                clip=true, edge="lost", pressure={0.45, 0.65},
                load_at=function(x, y) return 0.44 + 0.32 * (0.5 + 0.5 * nzBank2(x * 0.45, y * 0.9)) end})
print("bank evened again")

--@ chunk 288
print(wait(3 * 24 * 60))
-- give the bank a deliberate surface: short broken drags in its own colour,
-- turned every which way, so the texture reads as handled rather than blotted
work(bankOnly, {hand="scumble", tool="filbert 9", pile=bankDk, coverage=1.4,
                clip=true, edge="lost", pressure={0.2, 0.4}, length={8, 26},
                angle=function(x, y) return -0.9 + 2.4 * (0.5 + 0.5 * nzBank2(x * 0.8, y * 1.6)) end,
                broken=0.6, clump=0.4, load=0.32})
print("bank scumbled")

--@ chunk 289
print(wait(3 * 24 * 60))
work(bankOnly, {hand="body", tool="filbert 12", pile=bankDk, coverage=3, fill=true,
                clip=true, edge="lost", pressure={0.45, 0.65},
                load_at=function(x, y) return 0.40 + 0.40 * (0.5 + 0.5 * nzBank2(x * 0.45, y * 0.9)) end})
print("bank back to its dark mottle")

--@ chunk 290
print(drying(500, 425), drying(600, 420), drying(352, 420), drying(900, 430), drying(505, 470))

--@ chunk 291
shA = {
  {352,431},{353,420},{351,411},{352,402},
}
ph = pencil("H")
ph:line(shA, {pressure={0.35,0.3,0.15}})
ph:line({{352,420},{346,415},{341,411}}, {pressure={0.3,0.22,0.1}})
ph:line({{352,412},{358,407},{362,404}}, {pressure={0.28,0.2,0.1}})
ph:line({{351,425},{346,421}}, {pressure={0.26,0.1}})

shB = {{505,431},{507,419},{505,408},{506,398}}
ph:line(shB, {pressure={0.35,0.3,0.15}})
ph:line({{507,419},{513,413},{517,409}}, {pressure={0.3,0.22,0.1}})
ph:line({{505,408},{499,403},{496,399}}, {pressure={0.28,0.2,0.1}})
ph:line({{506,406},{512,401}}, {pressure={0.26,0.1}})

shC = {{648,430},{646,420},{648,412},{647,404}}
ph:line(shC, {pressure={0.32,0.28,0.14}})
ph:line({{646,420},{640,416},{638,414}}, {pressure={0.28,0.2,0.1}})
ph:line({{648,412},{654,407}}, {pressure={0.26,0.1}})

shD = {{712,430},{714,418},{712,407},{713,395}}
ph:line(shD, {pressure={0.36,0.3,0.16}})
ph:line({{714,418},{720,412},{723,408}}, {pressure={0.3,0.22,0.1}})
ph:line({{712,407},{706,401},{703,398}}, {pressure={0.28,0.2,0.1}})
ph:line({{713,402},{719,398}}, {pressure={0.26,0.1}})

shE = {{452,430},{454,421},{452,413}}
ph:line(shE, {pressure={0.3,0.25,0.12}})
ph:line({{454,422},{459,417}}, {pressure={0.26,0.1}})
ph:line({{452,417},{447,414}}, {pressure={0.24,0.1}})
print("laid")

--@ chunk 292
farTree = pile{{"bone black",2.2},{"raw umber",1.6},{"pale smalt",2.4}}
rb2 = brush{kind="rigger", width=1.5, point=0.85, stiffness=0.55}
rb2:load(farTree, 0.85)

rb2:stroke({{352,432},{353,420},{351,411},{352,401}}, {pressure={0.75,0.28}, ramps={0.03,0.35}})
rb2:stroke({{352,420},{346,415},{340,410}}, {pressure={0.5,0.16}, ramps={0.03,0.4}})
rb2:stroke({{352,412},{358,407},{363,403}}, {pressure={0.45,0.15}, ramps={0.03,0.4}})
rb2:stroke({{351,425},{345,420}}, {pressure={0.4,0.12}})
rb2:reload(farTree, 0.85)

rb2:stroke({{505,432},{507,419},{505,408},{506,397}}, {pressure={0.8,0.3}, ramps={0.03,0.35}})
rb2:stroke({{507,419},{513,413},{518,408}}, {pressure={0.52,0.16}, ramps={0.03,0.4}})
rb2:stroke({{505,408},{499,403},{495,398}}, {pressure={0.46,0.15}, ramps={0.03,0.4}})
rb2:stroke({{506,406},{513,400}}, {pressure={0.4,0.12}})
rb2:reload(farTree, 0.85)

rb2:stroke({{648,431},{646,420},{648,412},{647,403}}, {pressure={0.7,0.26}, ramps={0.03,0.35}})
rb2:stroke({{646,420},{640,416},{637,413}}, {pressure={0.46,0.14}, ramps={0.03,0.4}})
rb2:stroke({{648,412},{655,406}}, {pressure={0.38,0.12}})
rb2:reload(farTree, 0.85)

rb2:stroke({{712,431},{714,418},{712,407},{713,394}}, {pressure={0.8,0.3}, ramps={0.03,0.35}})
rb2:stroke({{714,418},{720,412},{724,407}}, {pressure={0.5,0.16}, ramps={0.03,0.4}})
rb2:stroke({{712,407},{706,401},{702,397}}, {pressure={0.46,0.15}, ramps={0.03,0.4}})
rb2:stroke({{713,402},{720,397}}, {pressure={0.4,0.12}})
rb2:reload(farTree, 0.8)

rb2:stroke({{452,431},{454,421},{452,412}}, {pressure={0.6,0.22}, ramps={0.03,0.35}})
rb2:stroke({{454,422},{460,416}}, {pressure={0.42,0.13}})
rb2:stroke({{452,417},{446,413}}, {pressure={0.38,0.12}})
print("painted far trees")

--@ chunk 293
rb2:reload(farTree, 0.7)
-- finer tips on the first pair
rb2:stroke({{352,404},{349,399},{347,396}}, {pressure={0.3,0.08}, ramps={0.03,0.4}})
rb2:stroke({{358,407},{361,403}}, {pressure={0.28,0.07}})
rb2:stroke({{340,410},{336,407}}, {pressure={0.26,0.07}})
rb2:stroke({{353,414},{357,411}}, {pressure={0.26,0.07}})
rb2:stroke({{506,401},{503,396}}, {pressure={0.3,0.08}})
rb2:stroke({{513,401},{517,397}}, {pressure={0.28,0.07}})
rb2:stroke({{499,403},{495,400}}, {pressure={0.26,0.07}})
rb2:stroke({{508,414},{512,411}}, {pressure={0.26,0.07}})
rb2:reload(farTree, 0.7)
rb2:stroke({{647,407},{643,402}}, {pressure={0.3,0.08}})
rb2:stroke({{650,415},{655,411}}, {pressure={0.28,0.07}})
rb2:stroke({{637,413},{633,411}}, {pressure={0.26,0.07}})
rb2:stroke({{713,398},{716,393}}, {pressure={0.3,0.08}})
rb2:stroke({{706,401},{701,398}}, {pressure={0.28,0.07}})
rb2:stroke({{720,411},{725,408}}, {pressure={0.26,0.07}})
rb2:stroke({{714,422},{718,418}}, {pressure={0.26,0.07}})
rb2:reload(farTree, 0.72)
-- two more, unevenly placed and of uneven height
rb2:stroke({{575,430},{574,421},{576,414},{575,406}}, {pressure={0.62,0.2}, ramps={0.03,0.35}})
rb2:stroke({{574,421},{569,417}}, {pressure={0.4,0.12}})
rb2:stroke({{576,417},{581,413}}, {pressure={0.38,0.12}})
rb2:stroke({{575,409},{572,405}}, {pressure={0.26,0.07}})
rb2:reload(farTree, 0.72)
rb2:stroke({{838,430},{840,424},{839,417}}, {pressure={0.5,0.18}, ramps={0.03,0.35}})
rb2:stroke({{840,424},{845,421}}, {pressure={0.34,0.1}})
rb2:stroke({{839,420},{835,417}}, {pressure={0.32,0.1}})
print("twigs")

--@ chunk 294
woodP = pile{{"bone black",1.7},{"raw umber",1.5},{"pale smalt",2.6}}
woodBase = ellipse(345,425,44,6) + ellipse(497,424,52,6.5) + ellipse(612,426,36,5.5) + ellipse(702,425,46,6.5)
woodBase = woodBase:blur(2.2)
woodBase = woodBase * mask(function(x,y) return (y>414 and y<433) and 1 or 0 end)
print(woodBase:area())
work(woodBase, {hand="scumble", tool="filbert 8", pile=woodP, coverage=1.5,
                pressure={0.35,0.22}, clip=true, edge=0.6, seed=11})

--@ chunk 295
lift1 = brush{kind="filbert", width=13, stiffness=0.85, hair=0.7}
lift1:wipe(1)
for i=1,3 do
  lift1:stroke({{305,424},{388,424}}, {pressure={0.9,0.9}, ramps={0.02,0.05}})
  lift1:stroke({{388,422},{305,422}}, {pressure={0.9,0.9}, ramps={0.02,0.05}})
end
print(drying(345,425))

--@ chunk 296
print(wait(1440)) print(drying(345,425), drying(497,424), drying(702,425))

--@ chunk 297
print(wait(720)) print(drying(345,425), drying(497,424), drying(702,425))

--@ chunk 298
print(wait(1440)) print(drying(345,425), drying(497,424), drying(702,425), drying(345,405))

--@ chunk 299
print(wait(2880)) print(drying(345,425), drying(497,424), drying(702,425))

--@ chunk 300
print(wait(2880)) print(drying(345,425), drying(497,424), drying(702,425), drying(575,426))

--@ chunk 301
print(wait(1440)) print(drying(345,425), drying(497,424), drying(702,425), drying(575,426))

--@ chunk 302
mTest = mask(function(x,y) return (x>398 and x<442 and y>410 and y<436) and 1 or 0 end):blur(2)
work(mTest, {hand="body", tool="filbert 12", pile=mistFix, coverage=3, fill=true,
             pressure={0.5,0.42}, clip=true, edge=0.5, seed=23})

--@ chunk 303
mRep = ellipse(318,427,62,13) + ellipse(420,426,72,12) + ellipse(545,427,72,13) + ellipse(662,426,62,12) + ellipse(736,427,48,12)
mRep = mRep:blur(2)
print(mRep:area())
work(mRep, {hand="body", tool="filbert 12", pile=mistFix, coverage=3, fill=true,
             pressure={0.5,0.42}, clip=true, edge=0.5, seed=31})

--@ chunk 304
mShoreBand = mask(function(x,y) return (y>402 and y<443) and 1 or 0 end)
clumps = {{300,70,9,17,14},{432,44,5,11,7},{505,70,8,16,12},{580,42,4,9,6},{650,62,9,18,13},{722,62,6,14,9}}
fringeP = pile{{"bone black",2.6},{"raw umber",1.8},{"pale smalt",2.0}}
rb3 = brush{kind="rigger", width=1.5, point=0.8, stiffness=0.6}
for ci, c in ipairs(clumps) do
  cx, hw, lo, hi, n = c[1], c[2], c[3], c[4], c[5]
  rb3:reload(fringeP, 0.8)
  for i = 1, n do
    local x = cx + rand(-hw, hw)
    local h = rand(lo, hi)
    local lean = rand(-2.5, 2.5)
    local by = 433 + rand(-2, 1)
    local pr = rand(0.32, 0.78)
    rb3:stroke({{x, by}, {x + lean*0.4, by - h*0.6}, {x + lean, by - h}},
      {pressure={pr, pr*0.28}, ramps={0.03, 0.4}, clip=mShoreBand})
  end
end
print("fringe")

--@ chunk 305
shoreLo = ellipse(300,432,55,5.5)+ellipse(382,433,62,5)+ellipse(472,431,52,6)+
          ellipse(562,433,56,5.5)+ellipse(652,432,60,6)+ellipse(732,433,46,5)
shoreLo = shoreLo:blur(1.6)
woodDark = pile{{"bone black",3.2},{"raw umber",2.0},{"pale smalt",1.8}}
work(shoreLo, {hand="detail", tool={kind="rigger",width=1.8,point=0.9,stiffness=0.6},
               pile=woodDark, coverage=2.2, fill=false, length={4,13},
               angle=function(x,y) return math.pi/2 + randn(0,0.45) end,
               pressure={0.55,0.75}, clip=true, edge=0.65, seed=41})

--@ chunk 306
rb3:reload(woodDark, 0.85)
-- clumps of wood standing above the shore line, at uneven intervals
rb3:stroke({{300,431},{299,423},{301,418}}, {pressure={0.7,0.3}, ramps={0.03,0.4}, clip=mShoreBand})
rb3:stroke({{305,432},{306,425}}, {pressure={0.6,0.25}, ramps={0.03,0.4}, clip=mShoreBand})
rb3:stroke({{357,432},{358,424},{356,417}}, {pressure={0.72,0.3}, ramps={0.03,0.4}, clip=mShoreBand})
rb3:stroke({{352,432},{351,426}}, {pressure={0.58,0.24}, ramps={0.03,0.4}, clip=mShoreBand})
rb3:stroke({{362,432},{364,425},{366,421}}, {pressure={0.6,0.26}, ramps={0.03,0.4}, clip=mShoreBand})
rb3:reload(woodDark, 0.85)
rb3:stroke({{468,431},{466,422},{468,415}}, {pressure={0.75,0.3}, ramps={0.03,0.4}, clip=mShoreBand})
rb3:stroke({{473,432},{475,424}}, {pressure={0.6,0.24}, ramps={0.03,0.4}, clip=mShoreBand})
rb3:stroke({{462,432},{460,427}}, {pressure={0.56,0.22}, ramps={0.03,0.4}, clip=mShoreBand})
rb3:reload(woodDark, 0.85)
rb3:stroke({{520,432},{522,425},{521,419}}, {pressure={0.68,0.28}, ramps={0.03,0.4}, clip=mShoreBand})
rb3:stroke({{526,432},{528,427}}, {pressure={0.55,0.22}, ramps={0.03,0.4}, clip=mShoreBand})
rb3:reload(woodDark, 0.85)
rb3:stroke({{639,432},{637,423},{639,416}}, {pressure={0.76,0.3}, ramps={0.03,0.4}, clip=mShoreBand})
rb3:stroke({{645,432},{646,425}}, {pressure={0.6,0.24}, ramps={0.03,0.4}, clip=mShoreBand})
rb3:stroke({{632,432},{631,428}}, {pressure={0.55,0.22}, ramps={0.03,0.4}, clip=mShoreBand})
rb3:reload(woodDark, 0.85)
rb3:stroke({{690,432},{692,426},{691,421}}, {pressure={0.66,0.27}, ramps={0.03,0.4}, clip=mShoreBand})
rb3:stroke({{748,432},{747,425},{749,420}}, {pressure={0.62,0.25}, ramps={0.03,0.4}, clip=mShoreBand})
rb3:stroke({{756,432},{757,427}}, {pressure={0.52,0.2}, ramps={0.03,0.4}, clip=mShoreBand})
print("clumps")

--@ chunk 307
rb3:reload(woodDark, 0.85)
rb3:stroke({{352,429},{353,421}}, {pressure={0.6,0.42}, ramps={0.03,0.3}, clip=mShoreBand})
rb3:stroke({{505,429},{507,420}}, {pressure={0.62,0.42}, ramps={0.03,0.3}, clip=mShoreBand})
rb3:stroke({{648,429},{646,421}}, {pressure={0.56,0.4}, ramps={0.03,0.3}, clip=mShoreBand})
rb3:stroke({{712,429},{714,419}}, {pressure={0.62,0.42}, ramps={0.03,0.3}, clip=mShoreBand})
rb3:stroke({{452,429},{454,422}}, {pressure={0.5,0.36}, ramps={0.03,0.3}, clip=mShoreBand})
rb3:stroke({{575,429},{574,422}}, {pressure={0.5,0.36}, ramps={0.03,0.3}, clip=mShoreBand})
rb3:stroke({{838,429},{840,425}}, {pressure={0.46,0.32}, ramps={0.03,0.3}, clip=mShoreBand})
print("feet")

--@ chunk 308
print(drying(320,447), drying(495,446), drying(655,447), drying(500,455))

--@ chunk 309
reflShore = ellipse(318,447,46,5)+ellipse(372,448,26,4)+ellipse(496,446,56,5)+
            ellipse(578,448,30,4)+ellipse(652,447,50,5)+ellipse(734,447,34,4.5)
reflShore = reflShore:blur(4.5)
reflShore = reflShore * mask(function(x,y) return (y>436 and y<466) and 1 or 0 end)
reflPile = pile{{"lead white",0.55},{"raw umber",1.3},{"pale smalt",2.1},{"bone black",0.7}}
work(reflShore, {hand="body", tool="filbert 16", pile=reflPile, coverage=1.4,
                 pressure={0.32,0.24}, fill=false, clip=true, edge=0.75, seed=57})

--@ chunk 310
smear = brush{kind="filbert", width=22, stiffness=0.3, hair=0.6}
smear:wipe(1)
for i=1,4 do
  smear:stroke({{288,449},{360,447},{400,448}}, {pressure={0.45,0.3}, ramps={0.08,0.25}})
  smear:stroke({{400,447},{360,450},{288,449}}, {pressure={0.4,0.3}, ramps={0.08,0.25}})
end
print("smear test")

--@ chunk 311
for i=1,3 do
  smear:stroke({{285,450},{400,447},{520,449},{640,446},{775,449}}, {pressure={0.45,0.32}, ramps={0.08,0.25}})
  smear:stroke({{775,449},{640,452},{520,450},{400,452},{285,450}}, {pressure={0.4,0.3}, ramps={0.08,0.25}})
end
smear:wipe(0.5)
for i=1,3 do
  smear:stroke({{290,455},{430,453},{560,456},{700,453},{778,455}}, {pressure={0.3,0.22}, ramps={0.1,0.3}})
  smear:stroke({{778,455},{700,457},{560,454},{430,457},{290,455}}, {pressure={0.28,0.2}, ramps={0.1,0.3}})
end
print("smeared all")

--@ chunk 312
smear:reload(mistFix, 0.35)
for i=1,2 do
  smear:stroke({{372,449},{415,448}}, {pressure={0.35,0.3}, ramps={0.1,0.3}})
  smear:stroke({{536,452},{572,451}}, {pressure={0.32,0.28}, ramps={0.1,0.3}})
  smear:stroke({{686,450},{722,449}}, {pressure={0.3,0.26}, ramps={0.1,0.3}})
end
smear:wipe(0.6)
smear:reload(mistFix, 0.3)
for i=1,2 do
  smear:stroke({{420,460},{462,459}}, {pressure={0.3,0.26}, ramps={0.12,0.35}})
  smear:stroke({{600,458},{640,457}}, {pressure={0.28,0.24}, ramps={0.12,0.35}})
end
print("breaks")

--@ chunk 313
smear:wipe(1); rb3:wipe(1); rb2:wipe(1)
print(wait(2880)) print(drying(500,450), drying(400,448), drying(700,452), drying(330,460))

--@ chunk 314
print(wait(2880)) print(drying(500,450), drying(400,448), drying(700,452), drying(330,460), drying(600,458))

--@ chunk 315
mMist = ellipse(340,452,84,20)+ellipse(485,451,92,21)+ellipse(645,452,88,20)+ellipse(748,452,62,18)
mMist = mMist:blur(4)
mMist = mMist * mask(function(x,y) return (y>430 and y<480) and 1 or 0 end)
work(mMist, {hand="body", tool="filbert 12", pile=mistFix, coverage=3, fill=true,
             pressure={0.5,0.42}, clip=true, edge=0.6, seed=63})

--@ chunk 316
print(drying(500,455), drying(350,460), drying(650,455))

--@ chunk 317
print(wait(2880)) print(drying(500,455), drying(350,460), drying(650,455))

--@ chunk 318
print(wait(2880)) print(drying(500,455), drying(350,460), draining)

--@ chunk 319
print(drying(500,455), drying(350,460), drying(650,455), drying(760,450))

--@ chunk 320
mRefl1 = ellipse(316,446,34,3.2)+ellipse(362,447,20,2.8)+ellipse(432,448,17,2.6)+ellipse(498,445,38,3.2)
mRefl1 = mRefl1:blur(1.6)
mRefl1 = mRefl1 * mask(function(x,y) return (y>437 and y<461) and 1 or 0 end)
reflPile2 = pile{{"lead white",1.1},{"pale smalt",2.2},{"raw umber",1.0},{"bone black",0.35}}
work(mRefl1, {hand="detail", tool={kind="rigger",width=1.9,point=0.9,stiffness=0.6},
              pile=reflPile2, coverage=2.2, fill=false, length={5,14},
              angle=0, pressure={0.42,0.62}, clip=true, edge=0.6, seed=71})

--@ chunk 321
mRefl2 = ellipse(560,447,22,2.8)+ellipse(628,446,32,3.0)+ellipse(700,448,20,2.6)+ellipse(744,447,23,2.8)
mRefl2 = mRefl2:blur(1.6)
mRefl2 = mRefl2 * mask(function(x,y) return (y>437 and y<461) and 1 or 0 end)
work(mRefl2, {hand="detail", tool={kind="rigger",width=1.9,point=0.9,stiffness=0.6},
              pile=reflPile2, coverage=2.2, fill=false, length={5,14},
              angle=0, pressure={0.42,0.62}, clip=true, edge=0.6, seed=73})

--@ chunk 322
smear:wipe(1)
for i=1,3 do
  smear:stroke({{292,448},{380,446},{470,449},{560,446},{660,448},{772,447}}, {pressure={0.5,0.36}, ramps={0.1,0.25}})
  smear:stroke({{772,447},{660,450},{560,447},{470,450},{380,447},{292,449}}, {pressure={0.44,0.34}, ramps={0.1,0.25}})
end
smear:wipe(0.55)
for i=1,3 do
  smear:stroke({{296,453},{420,452},{560,454},{700,452},{770,453}}, {pressure={0.3,0.24}, ramps={0.12,0.3}})
  smear:stroke({{770,453},{700,455},{560,452},{420,455},{296,453}}, {pressure={0.26,0.2}, ramps={0.12,0.3}})
end
print("merged")

--@ chunk 323
print(drying(700,610), drying(850,600), drying(600,622), drying(950,615), drying(760,590))
