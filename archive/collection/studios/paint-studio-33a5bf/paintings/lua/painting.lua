-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box inness
--@ engine 2

--@ chunk 1
canvas{size=1140, aspect=1.5, linen={16, 14}, seed=1889,
  ground={
    {pile={{"lead white", 10}, {"yellow ochre", 0.3}}, um=110, apply="knife", texture=0.35},
    {pile={{"raw sienna", 3}, {"raw umber", 1.2}, {"red earth", 0.4}}, um=7, apply="brush", texture=0.5},
  }}
print(W, H)
print(table.concat(tubes(), ", "))

--@ chunk 2
c = chalk()
-- horizon / distant hills
c:sketch({{0, 418}, {120, 410}, {260, 414}, {420, 405}, {560, 398}, {700, 392}, {820, 400}, {1000, 404}}, {pressure=0.25})
-- big tree group outline
c:sketch({{70, 420}, {60, 330}, {80, 240}, {120, 170}, {190, 120}, {270, 105}, {340, 130}, {400, 190}, {430, 280}, {440, 360}, {420, 420}}, {pressure=0.3})
-- slender trees right of center
c:line({{668, 440}, {664, 360}, {660, 280}, {655, 200}}, {pressure={0.4, 0.3, 0.2}})
c:line({{712, 445}, {716, 360}, {722, 260}, {728, 190}}, {pressure={0.4, 0.3, 0.2}})
-- pool
c:sketch({{600, 520}, {700, 505}, {840, 508}, {940, 525}, {860, 560}, {700, 560}, {610, 545}}, {pressure=0.25})
-- path through meadow
c:sketch({{420, 667}, {470, 590}, {530, 520}, {565, 470}}, {pressure=0.25})
-- figure
c:line({{565, 470}, {566, 448}}, {pressure=0.5})

--@ chunk 3
-- the main masses as masks, kept global
treeO = outline{{58, 425}, {52, 360}, {66, 290}, {92, 230}, {118, 175}, {160, 128}, {205, 92}, {258, 84}, {300, 108}, {332, 150}, {372, 168}, {410, 205}, {438, 262}, {452, 320}, {478, 352}, {496, 392}, {500, 428}, char="soft", closed=true, seed=11, amount=1.2, lobe=28}
treeM = treeO:mask()
print(treeM:area())
groundM = below({{0, 420}, {200, 418}, {420, 414}, {560, 406}, {700, 402}, {850, 406}, {1000, 408}})
poolM = poly({{598, 522}, {650, 510}, {740, 504}, {840, 507}, {930, 520}, {950, 530}, {900, 548}, {800, 560}, {690, 560}, {618, 548}}, true):roughen(4, 40, 3)
fgM = below({{0, 560}, {150, 575}, {330, 600}, {480, 620}, {650, 600}, {800, 590}, {1000, 575}}):roughen(10, 80, 5)

--@ chunk 4
umberT = pile{{"raw umber", 3}, {"bone black", 0.4}, {"Antwerp blue", 0.2}, medium=0.65}
work(treeM, {hand="scumble", pile=umberT, tool="filbert 16", coverage=1.4, length={20, 50}, angle=function(x, y) return -0.6 + 0.0015 * x end, scrub=2, edge="soft"})
work(fgM, {hand="broad", pile=umberT, coverage=1.2, angle=0.1, edge="loose"})
print(wait(0))

--@ chunk 5
blend(treeM:grow(12):soften(10), {angle=-0.7, clip=false})
blend(fgM:grow(10), {angle=0.05})
local d = drying(250, 250)
print(d, wait(0))

--@ chunk 6
local ring = treeM:grow(140) - treeM:grow(4):soften(4)
local rag = brush{kind="flat", width=16, stiffness=0.6}
local n = 0
for i = 1, 500 do
  local x, y = rand(0, 650), rand(0, 520)
  if ring:at(x, y) > 0.5 then
    local a = math.atan(y - 280, x - 260)
    local dx, dy = math.cos(a), math.sin(a)
    rag:wipe(1)
    rag:stroke({{x - 30 * dx, y - 30 * dy}, {x + 30 * dx, y + 30 * dy}}, {pressure={0.7, 0.6}, orient="across"})
    n = n + 1
  end
end
print(n, wait(0))

--@ chunk 7
oliveT = pile{{"raw umber", 2}, {"Antwerp blue", 0.4}, {"raw sienna", 1.5}, medium=0.85}
local mid = groundM - fgM:grow(10):soften(20) - poolM:grow(6)
work(mid, {hand="glaze", pile=oliveT, coverage=1.0, angle=0.03, clip=true})
-- shadow side under the trees on the meadow
local sh = poly({{40, 420}, {520, 420}, {600, 470}, {420, 500}, {150, 500}, {30, 480}}, true):soften(15)
work(sh, {hand="glaze", pile=umberT, coverage=0.8, angle=0.02})
print(wait(0))

--@ chunk 8
print(wait(22 * 60))
print(drying(250, 250), drying(500, 450), drying(300, 620))

--@ chunk 9
print(wait(10 * 60))
print(drying(250, 250), drying(500, 450), drying(300, 620))

--@ chunk 10
print(wait(15 * 60))
print(drying(250, 250), drying(150, 300), drying(400, 200))

--@ chunk 11
print(wait(24 * 60))
print(drying(250, 250), drying(150, 300), drying(400, 200))

--@ chunk 12
treeO2 = outline{{48, 430}, {40, 380}, {50, 320}, {70, 262}, {84, 214}, {112, 168}, {140, 128}, {178, 92}, {214, 72}, {252, 70}, {286, 88}, {312, 118}, {326, 156}, {340, 190}, {362, 178}, {392, 172}, {420, 188}, {440, 222}, {452, 262}, {470, 300}, {492, 330}, {512, 362}, {524, 400}, {528, 430}, char="soft", closed=true, seed=21, amount=1.3, lobe=22}
treeN = treeO2:mask()
horizon = {{0, 416}, {200, 414}, {420, 410}, {560, 404}, {700, 400}, {850, 404}, {1000, 406}}
skyM = above(horizon) - treeN:shrink(6):soften(6)
print(skyM:area(), treeN:area())

--@ chunk 13
skyTop = pile{{"lead white", 6}, {"cobalt blue", 1.0}, {"bone black", 0.12}, {"yellow ochre", 0.5}, medium=0.1}
skyMid = pile{{"lead white", 8}, {"yellow ochre", 0.7}, {"cobalt blue", 0.35}, {"red earth", 0.12}, medium=0.1}
skyGlow = pile{{"lead white", 10}, {"lemon chrome", 0.7}, {"yellow ochre", 0.4}, {"red earth", 0.05}, medium=0.1}
local function band(y0, y1, soft)
  return mask(function(x, y) return smoothstep(y0 - soft, y0 + soft, y) * (1 - smoothstep(y1 - soft, y1 + soft, y)) end)
end
local ang = function(x, y) return 0.05 * math.sin(x / 140) - 0.04 end
work(skyM * band(-60, 165, 25), {hand="broad", pile=skyTop, coverage=1.8, angle=ang, fill=true, clip=skyM})
work(skyM * band(150, 300, 25), {hand="broad", pile=skyMid, coverage=1.8, angle=ang, fill=true, clip=skyM})
work(skyM * band(285, 440, 25), {hand="broad", pile=skyGlow, coverage=1.8, angle=ang, fill=true, clip=skyM})
print(wait(0))

--@ chunk 14
blend(skyM:shrink(3), {angle=0.0})
print(wait(0))

--@ chunk 15
cloudP = pile{{"lead white", 5}, {"red earth", 0.35}, {"cobalt blue", 0.6}, {"bone black", 0.08}, {"yellow ochre", 0.3}, medium=0.15}
glowP = pile{{"lead white", 8}, {"lemon chrome", 1.3}, {"cadmium yellow", 0.25}, {"orange chrome", 0.12}, medium=0.1}
-- glow near the horizon, right of center
local glowM = (ellipse(700, 372, 260, 60):soften(30)) * skyM
work(glowM, {hand="scumble", pile=glowP, coverage=1.6, angle=0.0, fill=true, edge="lost", clip=skyM})
-- cloud bank
local c1 = poly({{430, 238}, {520, 220}, {640, 205}, {780, 196}, {900, 192}, {1000, 188}, {1000, 236}, {900, 250}, {760, 258}, {620, 262}, {500, 256}}, true):roughen(10, 60, 4):soften(6)
local c2 = poly({{620, 88}, {700, 72}, {820, 64}, {960, 70}, {1000, 78}, {1000, 104}, {900, 112}, {760, 114}, {660, 108}}, true):roughen(12, 70, 8):soften(6)
local c3 = poly({{760, 352}, {860, 344}, {1000, 340}, {1000, 356}, {880, 362}, {780, 362}}, true):roughen(4, 50, 9):soften(4)
work(c1 * skyM, {hand="scumble", pile=cloudP, coverage=1.3, angle=-0.03, edge="soft", clip=skyM})
work(c2 * skyM, {hand="scumble", pile=cloudP, coverage=1.0, angle=-0.05, edge="soft", clip=skyM})
work(c3 * skyM, {hand="scumble", pile=cloudP, coverage=0.9, angle=-0.02, edge="soft", clip=skyM})
print(wait(0))

--@ chunk 16
local g = ellipse(700, 365, 330, 100):soften(40) * skyM:shrink(4)
blend(g, {angle=0.0})
blend(g, {angle=0.25})
local cl = rect(420, 180, 600, 100) * skyM:shrink(4)
blend(cl, {angle=-0.03})
print(drying(700, 380), drying(700, 230), wait(0))

--@ chunk 17
hillP = pile{{"lead white", 5}, {"cobalt blue", 1.0}, {"red earth", 0.4}, {"bone black", 0.15}, {"yellow ochre", 0.3}, medium=0.1}
hillM = poly({{440, 420}, {440, 398}, {520, 392}, {600, 386}, {660, 378}, {720, 376}, {780, 382}, {840, 390}, {900, 386}, {960, 380}, {1000, 382}, {1000, 420}}, true):roughen(3, 30, 12)
work(hillM, {hand="body", pile=hillP, coverage=1.6, angle=0.0, edge="soft", fill=true, length={20, 50}})
print(wait(0))

--@ chunk 18
hill2P = pile{{"lead white", 3}, {"cobalt blue", 1.0}, {"red earth", 0.45}, {"bone black", 0.25}, {"raw umber", 0.2}, medium=0.1}
hillM2 = poly({{430, 422}, {440, 402}, {500, 398}, {560, 394}, {610, 390}, {650, 385}, {690, 384}, {730, 388}, {770, 393}, {810, 395}, {850, 392}, {890, 387}, {930, 383}, {970, 385}, {1000, 388}, {1000, 422}}, true):roughen(2, 25, 14)
work(hillM2, {hand="body", pile=hill2P, coverage=1.8, angle=0.0, edge="firm", fill=true, length={25, 60}, clip=true})
blend(hillM2:grow(4), {angle=0.0})
print(wait(0))

--@ chunk 19
mLight = pile{{"yellow ochre", 3}, {"lemon chrome", 1.0}, {"lead white", 2.5}, {"Antwerp blue", 0.12}, {"raw sienna", 0.5}, medium=0.1}
mMid = pile{{"yellow ochre", 2}, {"raw umber", 1.2}, {"Antwerp blue", 0.3}, {"lead white", 1.0}, {"raw sienna", 0.8}, medium=0.1}
mShadow = pile{{"raw umber", 2.0}, {"Antwerp blue", 0.35}, {"yellow ochre", 1.0}, {"bone black", 0.2}, {"lead white", 0.3}, medium=0.1}
fgP = pile{{"raw umber", 2.0}, {"raw sienna", 1.2}, {"bone black", 0.3}, {"red earth", 0.3}, {"Antwerp blue", 0.15}, medium=0.15}
meadow = groundM - poolM
-- middle meadow overall
local midA = meadow * mask(function(x, y) return smoothstep(425, 445, y) * (1 - smoothstep(590, 620, y)) end)
work(midA, {hand="body", pile=mMid, coverage=1.5, angle=function(x, y) return 0.02 + 0.03 * math.sin(x / 90) end, length={30, 70}, fill=true, edge="soft"})
-- distant light strip
local farA = meadow * mask(function(x, y) return (1 - smoothstep(432, 448, y)) * smoothstep(380, 520, x) end)
work(farA, {hand="body", pile=mLight, coverage=1.5, angle=0.0, length={25, 60}, fill=true, edge="soft"})
print(wait(0))

--@ chunk 20
local shadowA = meadow * poly({{0, 412}, {540, 412}, {600, 440}, {640, 470}, {560, 490}, {420, 500}, {250, 520}, {100, 530}, {0, 540}}, true):roughen(14, 90, 31):soften(14)
work(shadowA, {hand="body", pile=mShadow, coverage=1.4, angle=function(x, y) return 0.03 * math.sin(x / 70) end, length={30, 70}, fill=true, edge="soft"})
local fgA = mask(function(x, y) return smoothstep(555, 600, y + 25 * math.sin(x / 130) + 15 * math.sin(x / 47)) end)
work(fgA, {hand="broad", pile=fgP, coverage=1.6, angle=function(x, y) return -0.05 + 0.08 * math.sin(x / 200) end, fill=true})
print(wait(0))

--@ chunk 21
blend(meadow:shrink(2), {angle=0.0})
blend(meadow * mask(function(x, y) return smoothstep(470, 520, y) end), {angle=0.15})
print(wait(0))

--@ chunk 22
poolP = pile{{"lead white", 8}, {"lemon chrome", 0.8}, {"yellow ochre", 0.5}, {"cobalt blue", 0.15}, medium=0.1}
poolDk = pile{{"lead white", 3}, {"cobalt blue", 0.8}, {"red earth", 0.3}, {"bone black", 0.2}, {"yellow ochre", 0.4}, medium=0.1}
work(poolM, {hand="body", pile=poolP, coverage=1.8, angle=0.0, length={30, 80}, fill=true, clip=true})
local topband = poolM * mask(function(x, y) return 1 - smoothstep(512, 524, y) end)
work(topband, {hand="body", pile=poolDk, coverage=1.2, angle=0.0, length={30, 70}, clip=true})
blend(poolM, {angle=0.0})
print(wait(0))

--@ chunk 23
local clumps = {
 {212, 112, 62, 48}, {165, 175, 78, 60}, {268, 165, 70, 62}, {120, 255, 72, 70}, {235, 255, 95, 78},
 {95, 335, 62, 55}, {215, 345, 105, 50}, {320, 300, 60, 70},
 {392, 205, 52, 42}, {432, 255, 70, 58}, {362, 280, 62, 60}, {470, 325, 62, 50}, {400, 352, 90, 45}, {505, 380, 40, 35},
 {55, 345, 40, 75}, {70, 260, 35, 40}}
local m = treeN
for i, c in ipairs(clumps) do
  m = m + ellipse(c[1], c[2], c[3], c[4]):roughen(9, 26, 40 + i)
end
canopy = (m * above({{0, 400}, {120, 392}, {260, 396}, {380, 394}, {500, 404}, {540, 410}})):roughen(5, 18, 77)
print(canopy:area())

--@ chunk 24
-- see the canopy outline lightly with chalk? no: paint the dark base directly
tDark = pile{{"Antwerp blue", 0.8}, {"raw umber", 2.0}, {"yellow ochre", 0.8}, {"bone black", 0.35}, medium=0.12}
tMid = pile{{"yellow ochre", 1.6}, {"raw umber", 1.2}, {"Antwerp blue", 0.45}, {"raw sienna", 0.6}, {"lead white", 0.35}, medium=0.1}
local ang = function(x, y) return -0.5 + 0.9 * math.sin(x / 37 + y / 53) end
work(canopy, {hand="body", pile=tDark, coverage=1.6, angle=ang, angle_jitter=0.6, length={14, 36}, fill=true, edge={found=0.2, soft=0.5, lost=0.3, period=30}})
print(wait(0))

--@ chunk 25
tLight = pile{{"yellow ochre", 2.0}, {"raw sienna", 1.0}, {"Antwerp blue", 0.3}, {"lead white", 0.7}, {"raw umber", 0.4}, medium=0.1}
local lights = {
 {232, 100, 40, 26}, {195, 160, 48, 28}, {290, 150, 42, 32}, {140, 240, 40, 30}, {262, 238, 55, 34},
 {340, 280, 34, 34}, {410, 195, 32, 22}, {455, 245, 40, 30}, {490, 315, 34, 26}, {240, 330, 52, 22}, {420, 340, 40, 20}, {80, 320, 26, 30}}
local lm = rect(0, 0, 1, 1)
for i, c in ipairs(lights) do
  lm = lm + ellipse(c[1], c[2], c[3], c[4]):roughen(8, 20, 90 + i):soften(5)
end
lm = lm * canopy:shrink(6)
work(lm, {hand="scumble", pile=tMid, coverage=1.0, angle=function(x, y) return -0.4 + 0.8 * math.sin(x / 29 + y / 41) end, length={8, 20}, edge="lost"})
local lm2 = mask(function(x, y) return 1 end) * rect(0, 0, 1, 1)
for i, c in ipairs(lights) do
  lm2 = lm2 + ellipse(c[1] + 8, c[2] - 6, c[3] * 0.55, c[4] * 0.5):roughen(6, 14, 120 + i):soften(4)
end
lm2 = lm2 * canopy:shrink(8)
stipple(lm2, {pile=tLight, width=5, coverage=1.2, cluster=0.5, feather=0.4})
print(wait(0))

--@ chunk 26
blend(canopy:shrink(10), {angle=-0.6})
blend(canopy:shrink(14), {angle=0.7})
print(wait(0))

--@ chunk 27
clumps = {
 {212, 112, 62, 48}, {165, 175, 78, 60}, {268, 165, 70, 62}, {120, 255, 72, 70}, {235, 255, 95, 78},
 {95, 335, 62, 55}, {215, 345, 105, 50}, {320, 300, 60, 70},
 {392, 205, 52, 42}, {432, 255, 70, 58}, {362, 280, 62, 60}, {470, 325, 62, 50}, {400, 352, 90, 45}, {505, 380, 40, 35},
 {55, 345, 40, 75}, {70, 260, 35, 40}}
local sm = rect(0, 0, 1, 1)
for i, c in ipairs(clumps) do
  local e = ellipse(c[1], c[2], c[3], c[4])
  local lit = ellipse(c[1] + c[3] * 0.25, c[2] - c[4] * 0.35, c[3] * 0.85, c[4] * 0.75)
  sm = sm + (e - lit):roughen(7, 18, 200 + i)
end
-- the whole lower band of the canopy is in shade
sm = sm + mask(function(x, y) return smoothstep(330, 380, y) end)
sm = (sm * canopy):soften(3)
work(sm, {hand="body", pile=tDark, coverage=1.3, angle=function(x, y) return -0.3 + 0.9 * math.sin(x / 31 + y / 47) end, angle_jitter=0.5, length={10, 26}, fill=true, edge="soft", pressure={0.5, 0.85}})
print(wait(0))

--@ chunk 28
print(drying(700, 100), drying(700, 360), drying(250, 250), drying(500, 500), drying(750, 530))

--@ chunk 29
print(wait(20 * 60))
print(drying(700, 100), drying(700, 360), drying(250, 250), drying(500, 500), drying(750, 530), drying(300, 640))

--@ chunk 30
print(wait(2 * 24 * 60))
print(drying(700, 100), drying(700, 360), drying(250, 250), drying(500, 500), drying(750, 530), drying(300, 640))

--@ chunk 31
skyHi = pile{{"lead white", 4}, {"cobalt blue", 1.0}, {"raw umber", 0.35}, {"red earth", 0.12}, {"yellow ochre", 0.2}, medium=0.25}
local topM = skyM:shrink(5) * mask(function(x, y) return 1 - smoothstep(60, 200, y + 30 * math.sin(x / 160)) end)
work(topM, {hand="broad", pile=skyHi, coverage=1.5, angle=function(x, y) return -0.06 + 0.05 * math.sin(x / 120) end, fill=true, load_at=function(x, y) return 0.9 - y / 300 end, clip=skyM})
blend(topM:grow(30) * skyM:shrink(5), {angle=0.0})
print(wait(0))

--@ chunk 32
tDarkW = pile{{"raw umber", 2.0}, {"Antwerp blue", 0.5}, {"yellow ochre", 0.9}, {"bone black", 0.25}, {"raw sienna", 0.7}, medium=0.12}
crown2 = outline{{92, 150}, {104, 112}, {128, 84}, {150, 58}, {178, 40}, {206, 30}, {236, 34}, {262, 50}, {288, 70}, {312, 98}, {336, 132}, {350, 170}, {300, 190}, {200, 190}, {110, 185}, char="soft", closed=true, seed=301, amount=1.4, lobe=18}:mask()
work(crown2, {hand="body", pile=tDarkW, coverage=1.5, angle=function(x, y) return -0.8 + 0.7 * math.sin(x / 23 + y / 37) end, angle_jitter=0.6, length={10, 24}, fill=true, edge={found=0.15, soft=0.45, lost=0.4, period=24}, pressure={0.45, 0.8}})
print(wait(0))

--@ chunk 33
-- reshape the pool: bring the banks in over its ends and lower edge
local bankR = poly({{860, 500}, {960, 510}, {960, 570}, {800, 570}, {840, 548}, {870, 530}}, true):roughen(5, 30, 401)
local bankL = poly({{590, 515}, {640, 512}, {660, 530}, {640, 548}, {660, 560}, {590, 565}}, true):roughen(5, 30, 402)
local bankB = poly({{640, 548}, {720, 549}, {800, 546}, {850, 540}, {870, 560}, {640, 570}}, true):roughen(4, 25, 403)
local banks = (bankR + bankL + bankB) * poolM:grow(14)
work(banks, {hand="body", pile=mShadow, coverage=1.6, angle=0.0, length={15, 40}, fill=true, edge="soft"})
print(wait(0))

--@ chunk 34
local outer = (poolM:grow(14) - poolM:grow(3)):soften(4)
work(outer * mask(function(x, y) return smoothstep(-0.3, 0.6, math.sin(x / 23) + 0.5 * math.sin(x / 9 + 1)) end), {hand="scumble", pile=mMid, coverage=1.0, angle=0.0, length={8, 20}, edge="lost"})
blend(poolM:grow(22) - poolM:grow(1), {angle=0.0})
print(wait(0))

--@ chunk 35
mGold = pile{{"yellow ochre", 2}, {"lemon chrome", 0.9}, {"lead white", 1.6}, {"raw sienna", 0.5}, {"Antwerp blue", 0.08}, medium=0.12}
mWarm = pile{{"raw sienna", 1.5}, {"yellow ochre", 1.0}, {"raw umber", 0.9}, {"Antwerp blue", 0.2}, {"lead white", 0.5}, medium=0.12}
pool2 = poly({{640, 527}, {668, 518}, {700, 516}, {730, 519}, {760, 515}, {800, 516}, {840, 520}, {866, 527}, {850, 536}, {800, 541}, {740, 543}, {690, 541}, {652, 535}}, true):roughen(2, 20, 411)
local hz = function(x, y) return 0.012 * math.sin(x / 150) + 0.03 * math.sin(y / 17) end
local meadowA = groundM - pool2
local function yb(y0, y1, s) return mask(function(x, y) local yy = y + 8 * math.sin(x / 70) return smoothstep(y0 - s, y0 + s, yy) * (1 - smoothstep(y1 - s, y1 + s, yy)) end) end
-- lit distance, right of the trees
local litA = meadowA * yb(405, 462, 8) * mask(function(x, y) return smoothstep(470, 560, x) end)
work(litA, {hand="body", pile=mGold, coverage=1.5, angle=hz, length={30, 90}, fill=true})
-- middle meadow, right part
local midR = meadowA * yb(455, 530, 10) * mask(function(x, y) return smoothstep(420, 560, x) end)
work(midR, {hand="body", pile=mMid, coverage=1.4, angle=hz, length={30, 90}, fill=true})
-- warm lower meadow
local lowA = meadowA * yb(520, 600, 12)
work(lowA, {hand="body", pile=mWarm, coverage=1.3, angle=hz, length={30, 90}, fill=true})
-- shade of the trees
local shadeA = meadowA * yb(405, 525, 10) * mask(function(x, y) return 1 - smoothstep(470, 600, x + (y - 410) * 0.8) end)
work(shadeA, {hand="body", pile=mShadow, coverage=1.4, angle=hz, length={30, 90}, fill=true})
print(wait(0))

--@ chunk 36
local mA = groundM:shrink(3) - pool2:grow(3)
blend(mA * mask(function(x, y) return 1 - smoothstep(590, 620, y) end), {angle=0.0})
blend(mA * mask(function(x, y) return smoothstep(430, 470, y) * (1 - smoothstep(600, 630, y)) end), {angle=0.06})
print(wait(0))

--@ chunk 37
fgDark = pile{{"raw umber", 2.0}, {"raw sienna", 1.0}, {"Antwerp blue", 0.3}, {"bone black", 0.2}, {"yellow ochre", 0.6}, medium=0.12}
fgRust = pile{{"raw sienna", 1.4}, {"red earth", 0.5}, {"raw umber", 1.0}, {"yellow ochre", 0.6}, medium=0.12}
fgLine = {{0, 585}, {120, 592}, {260, 604}, {380, 612}, {480, 606}, {600, 598}, {720, 600}, {860, 590}, {1000, 584}}
fgM2 = below(fgLine):roughen(8, 50, 501)
work(fgM2, {hand="broad", pile=fgDark, coverage=1.6, angle=function(x, y) return -0.04 + 0.1 * math.sin(x / 170) end, fill=true, edge="soft"})
local rustM = fgM2:shrink(10) * mask(function(x, y) return smoothstep(0.1, 0.6, math.sin(x / 90 + 0.4) * 0.6 + math.sin(x / 37) * 0.4) end)
work(rustM, {hand="scumble", pile=fgRust, coverage=0.8, angle=-0.1, length={15, 40}, edge="lost"})
print(wait(0))

--@ chunk 38
blend(fgM2:grow(6), {angle=-0.05})
blend(fgM2:grow(6), {angle=0.6})
blend(fgM2:shrink(4), {angle=-0.4})
print(wait(0))

--@ chunk 39
print(wait(2 * 24 * 60))
print(drying(200, 120), drying(300, 300), drying(500, 470), drying(300, 630), drying(700, 300))

--@ chunk 40
print(wait(24 * 60))
print(drying(200, 120), drying(240, 160), drying(150, 100))

--@ chunk 41
-- open the space under the right-hand tree: sky, distant hill, far meadow seen beneath its canopy
under = poly({{392, 404}, {398, 372}, {418, 356}, {450, 350}, {482, 346}, {512, 352}, {540, 366}, {556, 392}, {560, 404}}, true):roughen(6, 22, 601)
local skyPart = under * above({{380, 396}, {460, 394}, {560, 392}})
work(skyPart, {hand="body", pile=skyGlow, coverage=1.6, angle=0.0, length={12, 30}, fill=true, clip=true})
local hillPart = under * below({{380, 396}, {460, 394}, {560, 392}}) * above({{380, 408}, {560, 406}})
work(hillPart, {hand="detail", pile=hill2P, coverage=1.6, angle=0.0, fill=true})
local farPart = under * below({{380, 408}, {560, 406}})
work(farPart, {hand="detail", pile=mGold, coverage=1.6, angle=0.0, fill=true})
-- a few sky holes high in the left crown and the gap between the two trees
holes = {}
local hp = {{150, 95, 9, 6}, {300, 110, 7, 9}, {335, 175, 12, 8}, {95, 210, 8, 7}, {260, 60, 6, 5}, {470, 270, 9, 7}, {505, 300, 7, 6}, {60, 300, 6, 9}, {352, 150, 14, 10}}
local hm = rect(0, 0, 1, 1)
for i, h in ipairs(hp) do hm = hm + ellipse(h[1], h[2], h[3], h[4]):roughen(3, 8, 610 + i) end
holeM = hm
work(hm * mask(function(x, y) return 1 - smoothstep(180, 260, y) end), {hand="detail", pile=skyMid, coverage=1.8, fill=true})
work(hm * mask(function(x, y) return smoothstep(180, 260, y) end), {hand="detail", pile=skyGlow, coverage=1.8, fill=true})
print(wait(0))

--@ chunk 42
treeAll = canopy + crown2
-- openings to cut into the tree mass
local notch = poly({{325, 120}, {352, 132}, {372, 168}, {362, 205}, {346, 236}, {332, 210}, {318, 170}}, true):roughen(6, 16, 701)
local leftOpen = poly({{0, 330}, {40, 322}, {70, 340}, {88, 372}, {82, 404}, {0, 410}}, true):roughen(8, 18, 702)
local topCut = poly({{60, 120}, {110, 70}, {140, 48}, {150, 80}, {120, 110}, {96, 150}, {60, 170}}, true):roughen(7, 16, 703)
local gap2 = poly({{232, 362}, {262, 352}, {300, 356}, {318, 380}, {310, 404}, {240, 406}, {226, 386}}, true):roughen(6, 14, 704)
openings = notch + leftOpen + topCut + gap2
skyR = above(horizon) - (treeAll:shrink(3) - openings)
print(skyR:area())

--@ chunk 43
sA = pile{{"lead white", 5}, {"cobalt blue", 0.8}, {"red earth", 0.25}, {"raw umber", 0.25}, {"yellow ochre", 0.3}, medium=0.12}
sB = pile{{"lead white", 6}, {"red earth", 0.32}, {"cobalt blue", 0.45}, {"yellow ochre", 0.5}, {"raw umber", 0.12}, medium=0.12}
sC = pile{{"lead white", 10}, {"yellow ochre", 0.6}, {"lemon chrome", 0.35}, {"red earth", 0.05}, medium=0.1}
sD = pile{{"lead white", 10}, {"lemon chrome", 1.3}, {"cadmium yellow", 0.3}, {"orange chrome", 0.12}, medium=0.1}
sE = pile{{"lead white", 8}, {"yellow ochre", 0.8}, {"orange chrome", 0.15}, {"red earth", 0.12}, medium=0.1}
local nz = noise{seed=811, period=180, octaves=4, stretch={0.0, 3}}
local function zone(f) return mask(f) * skyR end
local ang = function(x, y) return -0.12 + 0.1 * math.sin(x / 210 + y / 90) end
-- upper sky
work(zone(function(x, y) local yy = y + 35 * nz(x, y) return 1 - smoothstep(110, 150, yy) end), {hand="broad", pile=sA, coverage=1.6, angle=ang, fill=true, clip=skyR})
-- cloud bank
work(zone(function(x, y) local yy = y + 35 * nz(x, y) return smoothstep(110, 150, yy) * (1 - smoothstep(270, 310, yy)) end), {hand="broad", pile=sB, coverage=1.6, angle=ang, fill=true, clip=skyR})
-- low sky: cream to the left, glow right of center
work(zone(function(x, y) local yy = y + 35 * nz(x, y) return smoothstep(270, 310, yy) * (1 - smoothstep(560, 640, x)) end), {hand="broad", pile=sC, coverage=1.6, angle=ang, fill=true, clip=skyR})
work(zone(function(x, y) local yy = y + 35 * nz(x, y) return smoothstep(270, 310, yy) * smoothstep(560, 640, x) end), {hand="broad", pile=sD, coverage=1.6, angle=ang, fill=true, clip=skyR})
print(wait(0))

--@ chunk 44
local sk = skyR:shrink(2)
blend(sk, {angle=-0.35})
blend(sk, {angle=0.25})
print(wait(0))

--@ chunk 45
farWoods = pile{{"lead white", 2.5}, {"cobalt blue", 0.8}, {"raw umber", 0.6}, {"bone black", 0.15}, {"yellow ochre", 0.4}, {"red earth", 0.15}, medium=0.1}
-- distant hill line on the right, and a far wood-line on the left behind the trees
local hl = poly({{420, 418}, {430, 402}, {480, 398}, {540, 395}, {600, 391}, {650, 386}, {690, 385}, {730, 389}, {770, 394}, {810, 396}, {850, 393}, {890, 388}, {930, 384}, {970, 386}, {1000, 389}, {1000, 418}}, true):roughen(2, 22, 821)
work(hl, {hand="body", pile=hill2P, coverage=1.6, angle=0.0, length={25, 60}, fill=true, edge="soft"})
local fw = poly({{0, 418}, {0, 392}, {30, 388}, {60, 391}, {90, 386}, {130, 389}, {170, 384}, {210, 389}, {250, 386}, {300, 390}, {340, 387}, {380, 392}, {420, 396}, {460, 400}, {470, 418}}, true):roughen(4, 10, 822)
work(fw, {hand="body", pile=farWoods, coverage=1.6, angle=0.0, length={12, 30}, fill=true, edge="soft"})
blend(hl:grow(3) + fw:grow(3), {angle=0.0})
print(wait(0))

--@ chunk 46
treeBody = treeAll - openings
treeOuter = (treeBody:grow(16):roughen(9, 20, 901) - openings:shrink(8)) * above({{0, 412}, {1000, 412}})
local rim = treeOuter - treeBody:shrink(16)
tEdge = pile{{"raw umber", 2}, {"Antwerp blue", 0.45}, {"yellow ochre", 1.0}, {"lead white", 0.9}, {"bone black", 0.15}, medium=0.12}
work(rim, {hand="body", tool="filbert 6", pile=tDarkW, coverage=1.4, angle=function(x, y) return -0.7 + 1.0 * math.sin(x / 19 + y / 27) end, angle_jitter=0.8, length={6, 16}, fill=true, edge={found=0.15, soft=0.45, lost=0.4, period=26}, pressure={0.4, 0.8}})
print(wait(0))

--@ chunk 47
tMass = pile{{"raw umber", 2}, {"Antwerp blue", 0.55}, {"yellow ochre", 0.7}, {"bone black", 0.3}, {"raw sienna", 0.5}, medium=0.15}
local core = treeOuter:shrink(5)
work(core, {hand="body", tool="filbert 12", pile=tMass, coverage=1.7, angle=function(x, y) return -0.6 + 0.8 * math.sin(x / 41 + y / 63) end, angle_jitter=0.5, length={18, 45}, fill=true, edge="soft", scrub=1})
print(wait(0))

--@ chunk 48
print(drying(700, 250), drying(700, 100), drying(200, 200), drying(700, 470), drying(300, 630))

--@ chunk 49
trunkP = pile{{"raw umber", 2}, {"bone black", 0.6}, {"Antwerp blue", 0.15}, {"lead white", 0.25}, medium=0.2}
rig = brush{kind="rigger", width=3.2, point=1}
rig:load(trunkP, 0.8)
-- tree A
rig:stroke({{702, 478}, {700, 430}, {697, 380}, {695, 330}, {692, 280}, {690, 240}, {688, 205}}, {pressure={0.95, 0.25}, ramps={0.02, 0.5}, shake=0.6})
rig:load(trunkP, 0.6)
rig:stroke({{696, 350}, {680, 320}, {668, 290}, {660, 262}}, {pressure={0.55, 0.05}, ramps={0.05, 0.6}, shake=0.6})
rig:stroke({{693, 300}, {708, 270}, {716, 240}, {722, 220}}, {pressure={0.5, 0.05}, ramps={0.05, 0.6}, shake=0.6})
rig:stroke({{691, 260}, {676, 236}, {670, 212}}, {pressure={0.4, 0.03}, ramps={0.05, 0.6}, shake=0.6})
-- tree B, leaning a little right
rig:load(trunkP, 0.8)
rig:stroke({{742, 482}, {744, 440}, {747, 395}, {751, 350}, {756, 300}, {760, 260}, {765, 225}}, {pressure={0.85, 0.2}, ramps={0.02, 0.5}, shake=0.6})
rig:load(trunkP, 0.6)
rig:stroke({{750, 360}, {770, 335}, {786, 312}, {798, 290}}, {pressure={0.45, 0.04}, ramps={0.05, 0.6}, shake=0.6})
rig:stroke({{756, 300}, {740, 275}, {733, 252}}, {pressure={0.4, 0.03}, ramps={0.05, 0.6}, shake=0.6})
rig:stroke({{760, 262}, {775, 240}, {784, 215}}, {pressure={0.35, 0.03}, ramps={0.05, 0.6}, shake=0.6})
-- a third, smaller, farther
rig:load(trunkP, 0.5)
rig:stroke({{812, 470}, {813, 440}, {815, 405}, {818, 372}, {821, 345}}, {pressure={0.6, 0.15}, ramps={0.02, 0.5}, shake=0.6})
print(wait(0))

--@ chunk 50
fol = pile{{"raw umber", 1.6}, {"lead white", 1.0}, {"Antwerp blue", 0.35}, {"yellow ochre", 0.9}, {"bone black", 0.12}, medium=0.15}
folD = pile{{"raw umber", 2}, {"Antwerp blue", 0.45}, {"yellow ochre", 0.6}, {"bone black", 0.3}, {"lead white", 0.3}, medium=0.15}
local fa = outline{{650, 300}, {656, 262}, {664, 228}, {680, 200}, {700, 190}, {722, 204}, {730, 236}, {728, 272}, {716, 300}, {720, 332}, {705, 352}, {684, 350}, {668, 332}, char="soft", closed=true, seed=1001, amount=1.4, lobe=12}:mask()
local fb = outline{{728, 290}, {732, 258}, {742, 228}, {760, 208}, {782, 210}, {796, 236}, {806, 270}, {812, 300}, {800, 330}, {780, 346}, {758, 350}, {740, 334}, char="soft", closed=true, seed=1002, amount=1.4, lobe=12}:mask()
local fc = outline{{798, 372}, {804, 348}, {818, 336}, {834, 344}, {842, 368}, {836, 392}, {818, 400}, {802, 392}, char="soft", closed=true, seed=1003, amount=1.4, lobe=8}:mask()
slimF = fa + fb + fc
local nzf = noise{seed=1010, period=26, octaves=3}
stipple(slimF, {pile=fol, width=6, coverage=function(x, y) return 0.9 + 0.8 * nzf(x, y) end, cluster={0.6, 10}, feather=0.5, pressure={0.3, 0.7}})
stipple(slimF:shrink(6) * mask(function(x, y) return smoothstep(-0.1, 0.5, nzf(x + 40, y)) end), {pile=folD, width=5, coverage=0.9, cluster={0.6, 8}, feather=0.5, pressure={0.3, 0.7}})
print(wait(0))

--@ chunk 51
blend(slimF:grow(4), {angle=-1.2, pressure={0.25, 0.45}})
blend(slimF:grow(4), {angle=0.4, pressure={0.2, 0.4}})
print(wait(0))

--@ chunk 52
print(wait(18 * 60))
print(drying(200, 200), drying(700, 260), drying(450, 380), drying(700, 100))

--@ chunk 53
folM = pile{{"raw umber", 2}, {"Antwerp blue", 0.4}, {"yellow ochre", 1.0}, {"bone black", 0.2}, {"lead white", 0.5}, {"raw sienna", 0.3}, medium=0.15}
local cl = {
 -- tree A (trunk ~ x 690-700)
 {672, 214, 16, 12}, {700, 200, 14, 11}, {718, 222, 13, 12}, {662, 255, 18, 13}, {690, 245, 15, 12}, {722, 262, 12, 14},
 {652, 292, 17, 12}, {680, 288, 14, 11}, {712, 300, 12, 12}, {668, 330, 16, 11}, {700, 338, 12, 10}, {640, 270, 9, 8},
 -- tree B (trunk ~ x 750-765)
 {768, 214, 14, 11}, {790, 230, 13, 12}, {746, 236, 12, 12}, {800, 268, 14, 12}, {770, 262, 13, 11}, {744, 290, 11, 12},
 {806, 300, 12, 13}, {782, 318, 14, 11}, {756, 336, 13, 10}, {816, 280, 8, 8},
 -- tree C
 {812, 350, 10, 9}, {832, 362, 9, 10}, {818, 384, 11, 8}}
local m = rect(0, 0, 1, 1)
for i, c in ipairs(cl) do m = m + ellipse(c[1], c[2], c[3], c[4]):roughen(5, 9, 1100 + i) end
folClumps = m
local ang = function(x, y) local cx = x < 740 and 695 or (x < 805 and 758 or 818) return math.atan(y - 380, x - cx) + 0.0 end
work(m, {hand="body", tool="filbert 4", pile=folM, coverage=1.2, angle=ang, angle_jitter=0.6, length={5, 12}, edge="lost", pressure={0.35, 0.75}, fill=false})
print(wait(0))

--@ chunk 54
local nz = noise{seed=811, period=180, octaves=4, stretch={0.0, 3}}
local region = poly({{570, 130}, {700, 115}, {880, 120}, {960, 150}, {970, 300}, {960, 392}, {570, 395}}, true):soften(20)
local keep = folClumps:shrink(1)
local R = (region - keep) * above(horizon)
local ang = function(x, y) return -0.12 + 0.1 * math.sin(x / 210 + y / 90) end
local function zone(f) return mask(f) * R end
work(zone(function(x, y) local yy = y + 35 * nz(x, y) return 1 - smoothstep(110, 150, yy) end), {hand="body", tool="filbert 10", pile=sA, coverage=1.5, angle=ang, length={20, 50}, fill=true, clip=R})
work(zone(function(x, y) local yy = y + 35 * nz(x, y) return smoothstep(110, 150, yy) * (1 - smoothstep(270, 310, yy)) end), {hand="body", tool="filbert 10", pile=sB, coverage=1.5, angle=ang, length={20, 50}, fill=true, clip=R})
work(zone(function(x, y) local yy = y + 35 * nz(x, y) return smoothstep(270, 310, yy) end), {hand="body", tool="filbert 10", pile=sD, coverage=1.5, angle=ang, length={20, 50}, fill=true, clip=R})
blend(R:shrink(2), {angle=-0.2})
print(wait(0))

--@ chunk 55
local crown = folClumps:grow(9):roughen(7, 14, 1201)
local holes = rect(0, 0, 1, 1)
local hp = {{690, 270, 7, 6}, {770, 290, 8, 6}, {705, 228, 5, 6}, {660, 310, 6, 5}, {795, 250, 6, 7}, {740, 318, 7, 5}, {680, 345, 6, 5}}
for i, h in ipairs(hp) do holes = holes + ellipse(h[1], h[2], h[3], h[4]):roughen(3, 6, 1210 + i) end
slimCrown = crown - holes
local nzf = noise{seed=1220, period=22, octaves=3}
local ang = function(x, y) local cx = x < 728 and 695 or (x < 805 and 758 or 818) return math.atan(y - 420, x - cx) + 0.25 * nzf(x, y) end
work(slimCrown * mask(function(x, y) return smoothstep(-0.4, 0.2, nzf(x, y)) end), {hand="body", tool="filbert 4", pile=folM, coverage=1.0, angle=ang, angle_jitter=0.5, length={6, 14}, edge="lost", pressure={0.3, 0.7}, dips={6, 0.5, 0.3}, load_at=function(x, y) return 0.45 + 0.25 * nzf(x + 30, y) end})
print(wait(0))

--@ chunk 56
blend(slimCrown:grow(3), {tool={kind="filbert", width=10, stiffness=0.2}, angle=-1.3, pressure={0.2, 0.4}, length={10, 25}})
print(wait(0))

--@ chunk 57
local nzf = noise{seed=1240, period=30, octaves=3}
local inner = slimCrown:shrink(7) * mask(function(x, y) return smoothstep(-0.2, 0.35, nzf(x, y)) end)
local ang = function(x, y) local cx = x < 728 and 695 or (x < 805 and 758 or 818) return math.atan(y - 420, x - cx) end
work(inner, {hand="body", tool="filbert 5", pile=folD, coverage=0.9, angle=ang, angle_jitter=0.6, length={5, 12}, edge="lost", pressure={0.3, 0.6}, load_at=function(x, y) return 0.4 + 0.2 * nzf(x + 50, y) end})
-- trunks and limbs into the wet foliage
rig:reload(trunkP, 0.8)
rig:stroke({{702, 478}, {700, 430}, {697, 395}, {695, 360}, {693, 330}, {691, 300}}, {pressure={0.95, 0.45}, ramps={0.02, 0.3}, shake=0.6})
rig:load(trunkP, 0.6)
rig:stroke({{695, 352}, {682, 326}, {672, 300}, {664, 276}}, {pressure={0.5, 0.05}, ramps={0.05, 0.6}, shake=0.7})
rig:stroke({{692, 318}, {705, 292}, {712, 268}}, {pressure={0.45, 0.04}, ramps={0.05, 0.6}, shake=0.7})
rig:reload(trunkP, 0.8)
rig:stroke({{742, 482}, {744, 440}, {746, 405}, {749, 372}, {752, 345}, {755, 318}}, {pressure={0.85, 0.4}, ramps={0.02, 0.3}, shake=0.6})
rig:load(trunkP, 0.6)
rig:stroke({{749, 368}, {766, 344}, {780, 320}}, {pressure={0.45, 0.04}, ramps={0.05, 0.6}, shake=0.7})
rig:stroke({{753, 335}, {740, 312}, {734, 292}}, {pressure={0.4, 0.04}, ramps={0.05, 0.6}, shake=0.7})
rig:load(trunkP, 0.5)
rig:stroke({{812, 470}, {813, 440}, {815, 410}, {817, 390}}, {pressure={0.6, 0.25}, ramps={0.02, 0.4}, shake=0.6})
print(wait(0))

--@ chunk 58
blend(slimCrown:shrink(2), {tool={kind="filbert", width=8, stiffness=0.2}, angle=1.2, pressure={0.15, 0.3}, length={8, 18}, coverage=0.7})
print(wait(0))

--@ chunk 59
wl = pile{{"raw umber", 1.6}, {"Antwerp blue", 0.45}, {"bone black", 0.2}, {"lead white", 0.9}, {"yellow ochre", 0.4}, {"red earth", 0.1}, medium=0.12}
-- a low far wood-line under and beyond the big trees, joining them to the land
local wlM = poly({{0, 432}, {0, 404}, {40, 400}, {90, 406}, {140, 402}, {200, 407}, {260, 403}, {330, 406}, {400, 404}, {470, 406}, {540, 408}, {580, 414}, {600, 424}, {560, 430}, {300, 432}}, true):roughen(3, 9, 1301)
work(wlM, {hand="body", tool="filbert 6", pile=wl, coverage=1.6, angle=-1.4, angle_jitter=0.4, length={6, 14}, fill=true, edge="soft"})
-- trunks under the canopy
tr = pile{{"raw umber", 2}, {"bone black", 0.5}, {"red earth", 0.15}, {"lead white", 0.2}, medium=0.15}
local fb = brush{kind="filbert", width=7}
local trunks = {
  {{150, 438}, {152, 420}, {150, 400}, {146, 384}}, {{198, 440}, {196, 418}, {199, 398}, {204, 380}},
  {{247, 434}, {246, 418}, {243, 398}, {240, 378}},
  {{392, 440}, {394, 420}, {392, 400}, {388, 384}}, {{448, 436}, {447, 420}, {451, 402}, {455, 388}}}
for i, t in ipairs(trunks) do
  fb:reload(tr, 0.7)
  fb:stroke(t, {pressure={0.85, 0.5}, ramps={0.05, 0.3}, shake=0.6})
end
print(wait(0))

--@ chunk 60
shadeBase = pile{{"raw umber", 2}, {"Antwerp blue", 0.5}, {"bone black", 0.35}, {"yellow ochre", 0.5}, {"raw sienna", 0.3}, medium=0.1}
local base = poly({{0, 440}, {0, 398}, {60, 394}, {140, 396}, {230, 398}, {330, 398}, {420, 398}, {500, 400}, {560, 404}, {590, 416}, {610, 430}, {560, 440}, {300, 446}}, true):roughen(4, 12, 1310)
local window = poly({{256, 404}, {262, 396}, {300, 394}, {318, 400}, {320, 418}, {300, 422}, {262, 422}}, true):roughen(3, 8, 1311)
work(base - window, {hand="body", tool="filbert 8", pile=shadeBase, coverage=2.2, angle=-0.05, angle_jitter=0.4, length={12, 30}, fill=true, edge="soft", pressure={0.6, 0.95}, dips={4, 0.9, 0.5}})
print(wait(0))

--@ chunk 61
print(wait(26 * 60))
print(drying(200, 415), drying(300, 425), drying(450, 360), drying(700, 260), drying(650, 150))

--@ chunk 62
print(wait(24 * 60))
print(drying(200, 415), drying(300, 425), drying(100, 410), drying(500, 420))

--@ chunk 63
print(wait(36 * 60))
print(drying(200, 415), drying(300, 425), drying(100, 410), drying(500, 420), drying(450, 360))

--@ chunk 64
print(wait(20 * 60))
print(drying(200, 415), drying(300, 425), drying(100, 410), drying(150, 405), drying(250, 300))

--@ chunk 65
print(wait(3 * 24 * 60))
print(drying(200, 415), drying(100, 410), drying(150, 405))

--@ chunk 66
print(wait(2 * 24 * 60))
print(drying(200, 415), drying(210, 420), drying(190, 410))

--@ chunk 67
local base = poly({{0, 438}, {0, 398}, {60, 394}, {140, 396}, {230, 398}, {330, 398}, {420, 398}, {500, 400}, {560, 404}, {590, 416}, {606, 428}, {560, 436}, {300, 440}}, true):roughen(4, 12, 1320)
local window = poly({{258, 404}, {264, 398}, {298, 396}, {314, 402}, {316, 416}, {298, 420}, {264, 420}}, true):roughen(3, 8, 1321)
underBase = base - window
work(underBase, {hand="body", tool="filbert 8", pile=shadeBase, coverage=2.0, angle=-0.05, angle_jitter=0.4, length={12, 30}, fill=true, edge="soft", pressure={0.6, 0.95}, dips={4, 0.9, 0.5}})
-- window: far light meadow and wood-line seen under the trees
work(window * above({{250, 408}, {320, 408}}), {hand="detail", pile=farWoods, coverage=1.5, fill=true})
work(window * below({{250, 408}, {320, 408}}), {hand="detail", pile=mGold, coverage=1.5, fill=true})
print(wait(0))

--@ chunk 68
trD = pile{{"raw umber", 1.5}, {"bone black", 0.8}, {"Antwerp blue", 0.1}, medium=0.15}
local fb = brush{kind="filbert", width=6}
local trunks = {
  {{148, 438}, {150, 418}, {148, 398}, {144, 378}, {140, 360}},
  {{206, 440}, {204, 418}, {207, 396}, {212, 376}, {218, 356}},
  {{257, 436}, {256, 418}, {253, 398}, {249, 380}, {246, 362}},
  {{392, 440}, {394, 420}, {392, 400}, {388, 382}, {384, 364}},
  {{452, 436}, {451, 420}, {455, 402}, {460, 386}}}
for i, t in ipairs(trunks) do
  fb:reload(trD, 0.8)
  fb:stroke(t, {pressure={0.9, 0.45}, ramps={0.05, 0.4}, shake=0.6})
end
-- foliage lobes hanging over the seam
local lobes = rect(0, 0, 1, 1)
local lp = {{40, 400, 30, 12}, {110, 402, 26, 10}, {180, 404, 22, 9}, {232, 398, 18, 9}, {330, 402, 30, 11}, {420, 400, 26, 10}, {500, 406, 28, 10}, {560, 410, 20, 9}}
for i, l in ipairs(lp) do lobes = lobes + ellipse(l[1], l[2], l[3], l[4]):roughen(4, 10, 1330 + i) end
work(lobes, {hand="body", tool="filbert 5", pile=tMass, coverage=1.4, angle=function(x, y) return -1.2 + 0.8 * math.sin(x / 13) end, length={6, 14}, fill=true, edge="lost"})
print(wait(0))

--@ chunk 69
local tops = rect(0, 0, 1, 1)
for i, p in ipairs({{140, 368}, {216, 364}, {247, 370}, {385, 372}, {459, 390}}) do
  tops = tops + ellipse(p[1], p[2], 14, 12):roughen(3, 8, 1340 + i)
end
work(tops, {hand="body", tool="filbert 5", pile=tMass, coverage=1.6, angle=function(x, y) return -1.0 + 0.9 * math.sin(x / 9 + y / 7) end, length={5, 12}, fill=true, edge="lost"})
-- warm light on the right flank of the trunk against the window and of the right-hand trunks
local lr = brush{kind="round", width=1.6, point=0.6}
lr:load(pile{{"yellow ochre", 1}, {"raw umber", 0.6}, {"lead white", 0.4}, medium=0.15}, 0.5)
lr:stroke({{259.5, 433}, {258.5, 418}, {256, 400}, {253, 388}}, {pressure={0.5, 0.15}, ramps={0.1, 0.4}, shake=0.5})
lr:stroke({{394.5, 436}, {396, 420}, {394.5, 404}}, {pressure={0.4, 0.1}, ramps={0.1, 0.5}, shake=0.5})
-- root flare
local fb2 = brush{kind="filbert", width=9}
for i, p in ipairs({{148, 437}, {206, 439}, {257, 435}, {392, 439}, {452, 435}}) do
  fb2:reload(trD, 0.5)
  fb2:stroke({{p[1] - 7, p[2] + 2}, {p[1], p[2] - 4}, {p[1] + 7, p[2] + 2}}, {pressure={0.5, 0.4}, orient="along"})
end
print(wait(0))

--@ chunk 70
print(drying(700, 60), drying(900, 200), drying(500, 300), drying(700, 470), drying(500, 630), drying(750, 530))

--@ chunk 71
cloudV = pile{{"lead white", 4}, {"cobalt blue", 0.7}, {"red earth", 0.35}, {"raw umber", 0.3}, {"yellow ochre", 0.2}, medium=0.2}
cloudRose = pile{{"lead white", 6}, {"red earth", 0.35}, {"yellow ochre", 0.5}, {"orange chrome", 0.08}, {"cobalt blue", 0.12}, medium=0.15}
local treeKeep = (treeAll - openings):grow(2) + slimCrown:grow(3)
-- a long drifting cloud bank, upper right, its belly lit
local cb = outline{{430, 70}, {520, 52}, {620, 40}, {720, 34}, {820, 30}, {920, 34}, {1000, 40}, {1000, 120}, {940, 128}, {880, 140}, {820, 146}, {760, 150}, {700, 146}, {640, 140}, {580, 128}, {520, 110}, {470, 92}, char="soft", closed=true, seed=1401, amount=1.6, lobe=40}:mask()
work(cb - treeKeep, {hand="broad", pile=cloudV, coverage=1.3, angle=function(x, y) return -0.08 + 0.06 * math.sin(x / 120) end, edge="lost", clip=-treeKeep})
local belly = outline{{560, 132}, {640, 146}, {720, 152}, {800, 150}, {880, 142}, {960, 128}, {1000, 124}, {1000, 150}, {930, 160}, {840, 168}, {740, 172}, {650, 164}, {590, 150}, char="soft", closed=true, seed=1402, amount=1.4, lobe=24}:mask()
work(belly - treeKeep, {hand="body", tool="filbert 12", pile=cloudRose, coverage=1.2, angle=-0.05, length={30, 70}, edge="lost", clip=-treeKeep})
-- a few small torn clouds near the glow
local tc = rect(0, 0, 1, 1)
for i, c in ipairs({{600, 226, 70, 8}, {880, 214, 110, 10}, {960, 262, 60, 6}, {560, 300, 40, 5}}) do tc = tc + ellipse(c[1], c[2], c[3], c[4]):roughen(5, 30, 1410 + i) end
work(tc - treeKeep, {hand="body", tool="filbert 8", pile=cloudRose, coverage=1.0, angle=-0.03, length={30, 70}, edge="lost", clip=-treeKeep})
print(wait(0))

--@ chunk 72
local treeKeep = (treeAll - openings):grow(2) + slimCrown:grow(3)
local reg = (poly({{380, 0}, {1000, 0}, {1000, 300}, {500, 300}, {380, 200}}, true) - treeKeep:grow(4)) * above(horizon)
blend(reg, {angle=0.5})
blend(reg, {angle=-0.4})
blend(reg, {angle=0.05})
print(wait(0))

--@ chunk 73
print(wait(30 * 60))
print(drying(440, 160), drying(500, 250), drying(700, 100), drying(355, 140))

--@ chunk 74
-- rebuild the crown's fringe: dark leafy lobes reaching out, with sky between
local fr = (treeOuter:grow(4) - treeAll:shrink(10)) * poly({{320, 100}, {560, 100}, {560, 330}, {320, 330}})
local nzf = noise{seed=1501, period=16, octaves=3}
local lacy = fr * mask(function(x, y) return smoothstep(-0.35, 0.15, nzf(x, y)) end)
work(lacy, {hand="body", tool="filbert 5", pile=tMass, coverage=1.6, angle=function(x, y) return math.atan(y - 330, x - 430) + 0.6 * nzf(x + 9, y) end, angle_jitter=0.5, length={5, 13}, fill=true, edge="lost", pressure={0.35, 0.75}})
-- left crown upper edge too: make it less of a cut silhouette
local fr2 = (treeOuter:grow(4) - treeAll:shrink(10)) * poly({{0, 0}, {330, 0}, {330, 130}, {120, 300}, {0, 300}})
local lacy2 = fr2 * mask(function(x, y) return smoothstep(-0.35, 0.15, nzf(x + 300, y)) end)
work(lacy2, {hand="body", tool="filbert 5", pile=tMass, coverage=1.4, angle=function(x, y) return math.atan(y - 260, x - 210) + 0.6 * nzf(x + 9, y) end, angle_jitter=0.5, length={5, 13}, fill=true, edge="lost", pressure={0.35, 0.75}})
print(wait(0))

--@ chunk 75
tWarm = pile{{"raw sienna", 1.2}, {"raw umber", 1.0}, {"yellow ochre", 0.8}, {"Antwerp blue", 0.25}, {"lead white", 0.5}, medium=0.2}
tOlive = pile{{"yellow ochre", 1.4}, {"raw umber", 0.8}, {"Antwerp blue", 0.3}, {"lead white", 0.9}, {"raw sienna", 0.4}, medium=0.2}
local nz = noise{seed=1601, period=60, octaves=4}
local body = treeAll:shrink(14) - openings
-- warm reddish-brown variation through the mass
work(body * mask(function(x, y) return smoothstep(0.05, 0.45, nz(x, y)) end), {hand="scumble", tool="filbert 10", pile=tWarm, coverage=0.7, angle=function(x, y) return -0.7 + 0.8 * math.sin(x / 33 + y / 51) end, length={10, 26}, edge="lost", pressure={0.25, 0.55}, load=0.35})
-- the side toward the glow: olive light along right/upper-right edges of each mass
local rightEdge = mask(function(x, y) return smoothstep(-0.1, 0.4, nz(x + 200, y + 100)) end) * (treeAll:shrink(4) - treeAll:shrink(34)) * mask(function(x, y) return smoothstep(250, 420, x) * (1 - smoothstep(330, 390, y)) end)
work(rightEdge - openings, {hand="scumble", tool="filbert 6", pile=tOlive, coverage=0.8, angle=function(x, y) return -0.9 + 0.7 * math.sin(x / 17 + y / 23) end, length={6, 15}, edge="lost", pressure={0.25, 0.5}, load=0.3})
print(wait(0))

--@ chunk 76
local body = (treeAll:shrink(4) - openings)
for i, a in ipairs({-0.6, 0.8, 0.1, -1.2}) do
  blend(body, {angle=a})
end
print(wait(0))

--@ chunk 77
local body = (treeAll:shrink(2) - openings)
local rag = brush{kind="flat", width=18, stiffness=0.7}
local n = 0
for i = 1, 900 do
  local x, y = rand(0, 560), rand(20, 420)
  if body:at(x, y) > 0.9 then
    local a = rand(-math.pi, math.pi)
    local dx, dy = math.cos(a), math.sin(a)
    rag:wipe(1)
    rag:stroke({{x - 22 * dx, y - 22 * dy}, {x + 22 * dx, y + 22 * dy}}, {pressure={0.85, 0.75}, orient="across", clip=body})
    n = n + 1
  end
end
print(n, wait(0))

--@ chunk 78
print(wait(24 * 60))
print(drying(200, 200), drying(300, 300), drying(450, 250), drying(120, 380), drying(250, 60))

--@ chunk 79
local body = (treeAll:shrink(1) - openings)
local nz = noise{seed=1701, period=45, octaves=4}
-- keep small olive glimpses toward the glow side and in the tops
local keepLight = mask(function(x, y)
  local side = smoothstep(150, 480, x) * (1 - smoothstep(260, 360, y))
  return smoothstep(0.25, 0.45, nz(x, y) + 0.25 * side)
end)
local paintM = body - keepLight
work(paintM, {hand="body", tool="filbert 9", pile=tMass, coverage=1.7, angle=function(x, y) return -0.8 + 0.9 * math.sin(x / 27 + y / 41) end, angle_jitter=0.6, length={10, 26}, fill=true, edge="soft", pressure={0.5, 0.9}, dips={5, 0.8, 0.4}})
print(wait(0))

--@ chunk 80
local body = (treeAll:shrink(6) - openings)
blend(body, {angle=-0.7, pressure={0.2, 0.4}})
print(wait(0))

--@ chunk 81
print(drying(690, 260), drying(770, 260), drying(700, 420), drying(750, 530), drying(500, 400))

--@ chunk 82
glzTree = pile{{"raw umber", 1.5}, {"Antwerp blue", 0.35}, {"yellow ochre", 0.6}, {"bone black", 0.15}, medium=0.7}
local cr = slimCrown:grow(2)
work(cr, {hand="glaze", tool="filbert 10", pile=glzTree, coverage=1.2, angle=-1.3, clip=cr, length={20, 40}})
print(wait(0))

--@ chunk 83
local oA = outline{{644, 300}, {640, 268}, {652, 236}, {664, 210}, {684, 192}, {706, 186}, {724, 198}, {734, 222}, {732, 252}, {726, 282}, {730, 314}, {716, 342}, {694, 356}, {668, 350}, {650, 330}, char="soft", closed=true, seed=1801, amount=1.8, lobe=10}:mask()
local oB = outline{{730, 300}, {734, 262}, {744, 230}, {762, 206}, {784, 204}, {800, 222}, {812, 252}, {818, 288}, {812, 318}, {796, 340}, {772, 352}, {748, 348}, {734, 330}, char="soft", closed=true, seed=1802, amount=1.8, lobe=10}:mask()
local oC = outline{{796, 380}, {800, 352}, {814, 336}, {832, 338}, {846, 356}, {846, 384}, {834, 402}, {812, 404}, char="soft", closed=true, seed=1803, amount=1.6, lobe=7}:mask()
local m = oA + oB + oC
local nz = noise{seed=1810, period=14, octaves=3}
local ez = m - m:shrink(14)
local gaps = ez * mask(function(x, y) return smoothstep(0.15, 0.3, nz(x, y)) end)
local inner = m:shrink(14) * mask(function(x, y) return smoothstep(0.42, 0.5, nz(x + 70, y)) end)
slim2 = m - gaps - inner
folO = pile{{"raw umber", 1.6}, {"Antwerp blue", 0.35}, {"yellow ochre", 1.0}, {"lead white", 0.9}, {"bone black", 0.12}, {"raw sienna", 0.3}, medium=0.15}
local ang = function(x, y) local cx = x < 728 and 695 or (x < 805 and 758 or 818) return math.atan(y - 430, x - cx) + 0.3 * nz(x, y) end
work(slim2, {hand="body", tool="filbert 4", pile=folO, coverage=1.8, angle=ang, angle_jitter=0.5, length={5, 12}, fill=true, edge="soft", pressure={0.5, 0.85}, dips={8, 0.7, 0.3}})
print(wait(0))

--@ chunk 84
local nz = noise{seed=1820, period=20, octaves=3}
local m = slimCrown:grow(10) + slim2
local cx = function(x) return x < 728 and 690 or (x < 805 and 772 or 821) end
local dk = m * mask(function(x, y)
  local c = cx(x)
  local d = (x - c) * 0.6 + (y - 260) * -0.4
  return 1 - smoothstep(0, 40, d + 25 * nz(x, y))
end)
local ang = function(x, y) return math.atan(y - 430, x - cx(x)) + 0.3 * nz(x, y) end
work(dk, {hand="body", tool="filbert 4", pile=folD, coverage=1.5, angle=ang, angle_jitter=0.5, length={5, 12}, fill=true, edge="lost", pressure={0.4, 0.8}, dips={8, 0.6, 0.3}})
print(wait(0))

--@ chunk 85
local tA = outline{{660, 300}, {656, 270}, {662, 240}, {672, 214}, {688, 196}, {706, 192}, {720, 204}, {726, 230}, {724, 262}, {728, 296}, {722, 326}, {706, 344}, {684, 348}, {666, 332}, char="soft", closed=true, seed=1901, amount=1.6, lobe=9}:mask()
local tB = outline{{738, 300}, {740, 268}, {748, 238}, {760, 214}, {776, 206}, {792, 218}, {802, 246}, {806, 280}, {800, 312}, {786, 336}, {764, 346}, {744, 338}, char="soft", closed=true, seed=1902, amount=1.6, lobe=9}:mask()
local tC = outline{{800, 380}, {804, 356}, {816, 342}, {830, 344}, {840, 362}, {840, 386}, {828, 400}, {810, 400}, char="soft", closed=true, seed=1903, amount=1.4, lobe=7}:mask()
crowns = tA + tB + tC
local region = rect(610, 150, 260, 255) * above(horizon)
local skyBack = region - crowns
local up = skyBack * mask(function(x, y) return 1 - smoothstep(250, 300, y) end)
local lo = skyBack * mask(function(x, y) return smoothstep(250, 300, y) end)
work(up, {hand="body", tool="filbert 7", pile=sC, coverage=1.6, angle=function(x, y) return -0.1 + 0.4 * math.sin(y / 30) end, length={10, 25}, fill=true, edge="firm", clip=region})
work(lo, {hand="body", tool="filbert 7", pile=sD, coverage=1.6, angle=function(x, y) return -0.1 + 0.4 * math.sin(y / 30) end, length={10, 25}, fill=true, edge="firm", clip=region})
print(wait(0))

--@ chunk 86
-- lift the wet halo around the slim crowns with a clean brush, wiped every stroke
local halo = (slimCrown:grow(16) + slim2:grow(8)) - crowns:shrink(1)
local rag = brush{kind="flat", width=10, stiffness=0.7}
local n = 0
for i = 1, 1400 do
  local x, y = rand(600, 880), rand(160, 420)
  if halo:at(x, y) > 0.5 then
    local a = rand(-math.pi, math.pi)
    local dx, dy = math.cos(a), math.sin(a)
    rag:wipe(1)
    rag:stroke({{x - 10 * dx, y - 10 * dy}, {x + 10 * dx, y + 10 * dy}}, {pressure={0.9, 0.8}, orient="across", clip=halo})
    n = n + 1
  end
end
print(n, wait(0))

--@ chunk 87
skyLine = {{0, 388}, {60, 386}, {130, 385}, {210, 385}, {300, 386}, {380, 389}, {430, 398}, {480, 395}, {540, 392}, {600, 388}, {650, 383}, {690, 382}, {730, 386}, {770, 391}, {810, 393}, {850, 390}, {890, 385}, {930, 381}, {970, 383}, {1000, 386}}
treeKeep = (treeAll - openings):grow(1) + crowns:shrink(2)
skyAll = (above(skyLine) - treeKeep):soften(1)
s1 = pile{{"lead white", 5}, {"cobalt blue", 0.85}, {"red earth", 0.22}, {"raw umber", 0.28}, {"yellow ochre", 0.25}, medium=0.12}
s2 = pile{{"lead white", 7}, {"red earth", 0.3}, {"cobalt blue", 0.35}, {"yellow ochre", 0.6}, {"raw umber", 0.08}, medium=0.12}
s3 = pile{{"lead white", 10}, {"yellow ochre", 0.7}, {"lemon chrome", 0.4}, {"red earth", 0.08}, {"cobalt blue", 0.05}, medium=0.1}
s4 = pile{{"lead white", 10}, {"lemon chrome", 1.4}, {"cadmium yellow", 0.3}, {"orange chrome", 0.15}, medium=0.1}
local nz = noise{seed=2001, period=200, octaves=4, stretch={0.0, 3.5}}
local function yy(x, y) return y + 40 * nz(x, y) + 30 * math.exp(-((x - 740) / 220) ^ 2) end
local function band(a, b) return mask(function(x, y) local v = yy(x, y) return smoothstep(a - 15, a + 15, v) * (1 - smoothstep(b - 15, b + 15, v)) end) * skyAll end
local ang = function(x, y) return -0.07 + 0.06 * math.sin(x / 150 + y / 80) end
local opt = function(p) return {hand="broad", pile=p, coverage=1.7, angle=ang, fill=true, clip=skyAll} end
work(band(-100, 120), opt(s1))
work(band(120, 235), opt(s2))
work(band(235, 315), opt(s3))
work(band(315, 500), opt(s4))
print(wait(0))

--@ chunk 88
local sk = skyAll:shrink(2)
blend(sk, {angle=-0.3})
blend(sk, {angle=0.3})
blend(sk * rect(590, 140, 300, 270), {angle=0.0})
print(wait(0))

--@ chunk 89
local core = treeAll - openings
local nz = noise{seed=2101, period=18, octaves=3}
local band = (core:grow(14):roughen(8, 16, 2102) - core:shrink(8)) * above({{0, 395}, {1000, 395}})
local lacy = band * mask(function(x, y) return smoothstep(-0.3, 0.2, nz(x, y)) end)
local ang = function(x, y) return math.atan(y - 330, x - (x < 330 and 200 or 430)) + 0.5 * nz(x + 11, y) end
work(lacy, {hand="body", tool="filbert 5", pile=tMass, coverage=1.3, angle=ang, angle_jitter=0.5, length={6, 15}, edge="lost", pressure={0.3, 0.7}, dips={6, 0.6, 0.3}})
-- slim crowns: feather their edges
local sb = (crowns:grow(8):roughen(5, 10, 2103) - crowns:shrink(4)) * mask(function(x, y) return smoothstep(-0.2, 0.3, nz(x + 400, y)) end)
local ang2 = function(x, y) local cx = x < 728 and 690 or (x < 805 and 772 or 821) return math.atan(y - 420, x - cx) + 0.4 * nz(x, y) end
work(sb, {hand="body", tool="filbert 3", pile=folM, coverage=1.2, angle=ang2, angle_jitter=0.5, length={4, 10}, edge="lost", pressure={0.3, 0.6}, dips={8, 0.5, 0.3}})
print(wait(0))

--@ chunk 90
local core = treeAll - openings
local band = (core:grow(16) - core:shrink(6)) * above({{0, 392}, {1000, 392}})
blend(band, {tool={kind="filbert", width=12, stiffness=0.15}, angle=-0.9, pressure={0.12, 0.25}, coverage=0.6})
local sb = crowns:grow(10) - crowns:shrink(3)
blend(sb, {tool={kind="filbert", width=8, stiffness=0.15}, angle=-1.2, pressure={0.12, 0.25}, coverage=0.6})
-- trunks up into the crowns
rig:reload(trunkP, 0.8)
rig:stroke({{701, 470}, {699, 430}, {697, 395}, {695, 360}, {693, 330}, {691, 300}, {689, 270}}, {pressure={0.9, 0.3}, ramps={0.02, 0.4}, shake=0.6})
rig:load(trunkP, 0.6)
rig:stroke({{695, 345}, {682, 322}, {674, 300}}, {pressure={0.45, 0.05}, ramps={0.05, 0.6}, shake=0.7})
rig:stroke({{692, 315}, {704, 292}, {711, 270}}, {pressure={0.4, 0.04}, ramps={0.05, 0.6}, shake=0.7})
rig:reload(trunkP, 0.8)
rig:stroke({{743, 474}, {745, 440}, {747, 405}, {750, 372}, {753, 345}, {757, 315}, {761, 285}}, {pressure={0.85, 0.3}, ramps={0.02, 0.4}, shake=0.6})
rig:load(trunkP, 0.6)
rig:stroke({{750, 368}, {765, 345}, {778, 322}}, {pressure={0.4, 0.04}, ramps={0.05, 0.6}, shake=0.7})
rig:stroke({{755, 330}, {742, 308}, {737, 290}}, {pressure={0.35, 0.04}, ramps={0.05, 0.6}, shake=0.7})
rig:load(trunkP, 0.5)
rig:stroke({{812, 465}, {813, 440}, {815, 410}, {817, 388}, {819, 370}}, {pressure={0.6, 0.2}, ramps={0.02, 0.4}, shake=0.6})
print(wait(0))

--@ chunk 91
print(wait(20 * 60))
print(drying(700, 380), drying(500, 100), drying(300, 300), drying(820, 300), drying(700, 470))

--@ chunk 92
print(wait(30 * 60))
print(drying(700, 380), drying(300, 300), drying(820, 300), drying(150, 300))

--@ chunk 93
-- distant hills: break the flat lilac stripe into a low range with a nearer wooded ridge
hFar = pile{{"lead white", 5}, {"cobalt blue", 0.8}, {"red earth", 0.3}, {"yellow ochre", 0.35}, {"bone black", 0.08}, medium=0.12}
hNear = pile{{"lead white", 2.2}, {"cobalt blue", 0.7}, {"raw umber", 0.6}, {"red earth", 0.2}, {"yellow ochre", 0.4}, {"bone black", 0.12}, medium=0.12}
local far = poly({{560, 412}, {566, 396}, {620, 391}, {680, 386}, {720, 380}, {760, 378}, {800, 383}, {850, 389}, {900, 384}, {950, 377}, {1000, 380}, {1000, 412}}, true):roughen(2, 30, 2201)
work(far, {hand="body", tool="filbert 8", pile=hFar, coverage=1.7, angle=0.0, length={20, 50}, fill=true, edge="soft"})
local near = poly({{560, 424}, {560, 408}, {600, 404}, {640, 400}, {680, 402}, {720, 404}, {770, 400}, {820, 403}, {860, 398}, {900, 396}, {940, 399}, {1000, 397}, {1000, 424}}, true):roughen(4, 10, 2202)
work(near, {hand="body", tool="filbert 5", pile=hNear, coverage=1.6, angle=-1.4, angle_jitter=0.4, length={5, 12}, fill=true, edge="soft"})
blend((far + near):grow(2), {angle=0.0, pressure={0.15, 0.3}})
print(wait(0))

--@ chunk 94
hFar2 = pile{{"lead white", 2.5}, {"cobalt blue", 1.0}, {"red earth", 0.35}, {"bone black", 0.1}, {"yellow ochre", 0.15}, medium=0.1}
hNear2 = pile{{"lead white", 0.8}, {"cobalt blue", 0.8}, {"raw umber", 0.9}, {"bone black", 0.2}, {"yellow ochre", 0.35}, medium=0.1}
local far = poly({{590, 404}, {600, 397}, {640, 393}, {690, 388}, {730, 383}, {765, 381}, {800, 385}, {850, 391}, {900, 386}, {950, 380}, {1000, 382}, {1000, 404}}, true):roughen(2, 30, 2211)
work(far, {hand="body", tool="filbert 6", pile=hFar2, coverage=1.8, angle=0.0, length={15, 40}, fill=true, clip=true, pressure={0.6, 0.9}})
local near = poly({{590, 426}, {596, 410}, {630, 406}, {670, 403}, {710, 405}, {760, 402}, {810, 404}, {850, 400}, {900, 398}, {950, 400}, {1000, 399}, {1000, 426}}, true):roughen(3, 8, 2212)
work(near, {hand="body", tool="filbert 5", pile=hNear2, coverage=2.0, angle=-1.45, angle_jitter=0.3, length={5, 12}, fill=true, clip=true, pressure={0.6, 0.9}})
-- the spill over the tree base
local spill = poly({{500, 400}, {600, 400}, {600, 440}, {500, 440}}) * (underBase:grow(4) + poly({{520, 395}, {600, 395}, {610, 432}, {520, 445}}, true))
work(spill * above({{0, 445}, {1000, 445}}) - near, {hand="body", tool="filbert 6", pile=shadeBase, coverage=2.0, angle=-0.05, length={8, 20}, fill=true, clip=true, pressure={0.6, 0.95}})
print(wait(0))

--@ chunk 95
print(wait(24 * 60))
print(drying(700, 395), drying(700, 415), drying(560, 420), drying(150, 300))

--@ chunk 96
print(wait(24 * 60))
print(drying(700, 395), drying(700, 415), drying(560, 420))

--@ chunk 97
print(wait(3 * 24 * 60))
print(drying(700, 395), drying(700, 415), drying(560, 420))

--@ chunk 98
glzHill = pile{{"cobalt blue", 1.0}, {"red earth", 0.3}, {"raw umber", 0.25}, medium=0.85}
local far = poly({{560, 404}, {600, 397}, {640, 392}, {690, 387}, {730, 382}, {765, 380}, {800, 384}, {850, 390}, {900, 385}, {950, 379}, {1000, 381}, {1000, 406}, {560, 406}}, true):roughen(2, 30, 2221)
work(far, {hand="glaze", pile=glzHill, coverage=1.0, angle=0.0, clip=true, length={60, 150}})
hNear3 = pile{{"lead white", 0.35}, {"cobalt blue", 0.6}, {"raw umber", 1.0}, {"bone black", 0.25}, {"yellow ochre", 0.45}, {"Antwerp blue", 0.12}, medium=0.1}
local near = outline{{560, 428}, {566, 412}, {600, 406}, {640, 404}, {680, 401}, {700, 404}, {740, 403}, {780, 398}, {820, 402}, {860, 399}, {900, 396}, {940, 400}, {980, 397}, {1000, 398}, {1000, 428}, char="soft", closed=true, seed=2222, amount=0.8, lobe=6}:mask()
work(near, {hand="body", tool="filbert 4", pile=hNear3, coverage=2.0, angle=-1.45, angle_jitter=0.35, length={4, 10}, fill=true, edge="soft", pressure={0.55, 0.9}})
print(wait(0))

--@ chunk 99
local far = rect(560, 372, 440, 34)
blend(far, {angle=0.0})
blend(far, {angle=0.0, pressure={0.2, 0.4}})
print(wait(0))

--@ chunk 100
print(wait(4 * 24 * 60))
print(drying(700, 385), drying(700, 415), drying(900, 380), drying(560, 420))

--@ chunk 101
hA = pile{{"lead white", 1.2}, {"cobalt blue", 1.0}, {"red earth", 0.35}, {"raw umber", 0.4}, {"yellow ochre", 0.2}, medium=0.1}
hB = pile{{"lead white", 1.8}, {"cobalt blue", 1.0}, {"red earth", 0.4}, {"raw umber", 0.25}, {"yellow ochre", 0.3}, medium=0.1}
local b = brush("filbert", 6)
b:load(hA, 0.8)
b:stroke({{960, 386}, {995, 386}}, {pressure={0.8, 0.8}})
b:reload(hB, 0.8)
b:stroke({{960, 396}, {995, 396}}, {pressure={0.8, 0.8}})
print(wait(0))

--@ chunk 102
local keep = treeKeep:grow(2) + crowns:grow(1)
stripSky = (mask(function(x, y) return smoothstep(345, 362, y) * (1 - smoothstep(402, 406, y)) * smoothstep(548, 560, x) end) - keep)
work(stripSky, {hand="body", tool="filbert 10", pile=s4, coverage=2.0, angle=0.0, angle_jitter=0.1, length={30, 70}, fill=true, edge="lost", clip=-keep, pressure={0.6, 0.9}})
print(wait(0))

--@ chunk 103
hA2 = pile{{"lead white", 1.2}, {"cobalt blue", 1.15}, {"red earth", 0.4}, {"raw umber", 0.35}, {"yellow ochre", 0.15}, medium=0.1}
local keep = treeKeep:grow(2) + crowns:grow(1)
farHills = (outline{{556, 410}, {560, 400}, {600, 396}, {640, 393}, {680, 389}, {715, 384}, {745, 382}, {775, 385}, {810, 390}, {850, 393}, {885, 389}, {920, 384}, {955, 381}, {985, 383}, {1000, 385}, {1000, 410}, char="soft", closed=true, seed=2301, amount=0.6, lobe=30}:mask()) - keep
work(farHills, {hand="body", tool="filbert 7", pile=hA2, coverage=1.8, angle=0.0, angle_jitter=0.1, length={25, 60}, fill=true, edge="soft", pressure={0.55, 0.85}, clip=-keep})
print(wait(0))

--@ chunk 104
local keep = treeKeep:grow(3) + crowns:grow(2)
local reg = rect(556, 350, 444, 62) - keep
blend(reg, {angle=0.0, pressure={0.3, 0.5}})
blend(reg, {angle=0.05, pressure={0.2, 0.4}})
print(wait(0))

--@ chunk 105
local keep = treeKeep:grow(2) + crowns:grow(1)
local nz = noise{seed=2401, period=120, octaves=3}
local skyFix = mask(function(x, y) return smoothstep(318, 362, y + 10 * nz(x, y)) * (1 - smoothstep(380, 392, y + 8 * nz(x + 50, y))) * smoothstep(535, 580, x + 15 * nz(x, y + 40)) end) - keep
work(skyFix, {hand="body", tool="filbert 10", pile=s4, coverage=2.0, angle=0.0, angle_jitter=0.1, length={30, 70}, fill=true, edge="lost", clip=-keep, pressure={0.6, 0.95}, dips={4, 0.9, 0.5}})
print(wait(0))

--@ chunk 106
local keep = treeKeep:grow(2) + crowns:grow(1)
hillShape = outline{{556, 418}, {560, 401}, {590, 398}, {630, 395}, {670, 392}, {705, 388}, {735, 385}, {760, 386}, {790, 390}, {825, 394}, {860, 392}, {895, 388}, {925, 384}, {960, 382}, {1000, 385}, {1000, 418}, char="soft", closed=true, seed=2411, amount=0.5, lobe=40}:mask()
work(hillShape - keep, {hand="body", tool="filbert 6", pile=hA2, coverage=2.2, angle=0.0, angle_jitter=0.08, length={25, 60}, fill=true, clip=hillShape - keep, pressure={0.6, 0.9}, dips={5, 0.85, 0.5}})
print(wait(0))

--@ chunk 107
local keep = treeKeep:grow(2) + crowns:grow(1)
hillCurve = {{550, 402}, {590, 399}, {630, 396}, {670, 393}, {700, 390}, {730, 387}, {755, 386}, {780, 388}, {810, 392}, {840, 395}, {870, 394}, {900, 390}, {930, 386}, {960, 384}, {1000, 386}}
local cut = (above(hillCurve) * mask(function(x, y) return smoothstep(352, 365, y) * smoothstep(545, 560, x) end)) - keep
work(cut, {hand="body", tool="filbert 6", pile=s4, coverage=2.2, angle=0.0, angle_jitter=0.05, length={20, 50}, fill=true, clip=cut, pressure={0.65, 0.95}, dips={4, 0.9, 0.5}})
print(wait(0))

--@ chunk 108
local pts = {}
local x = 548
local i = 0
while x <= 1004 do
  i = i + 1
  local base = 407 + 4 * math.sin(x / 90)
  local bump = (i % 3 == 0) and rand(5, 9) or rand(1, 4)
  pts[#pts + 1] = {x, base - bump}
  x = x + rand(7, 16)
end
ridgeCurve = pts
ridgeM = (below(ridgeCurve) * above({{540, 436}, {700, 433}, {850, 431}, {1000, 430}})):roughen(1.5, 5, 2501)
hNear4 = pile{{"lead white", 0.5}, {"cobalt blue", 0.7}, {"raw umber", 1.0}, {"bone black", 0.2}, {"yellow ochre", 0.4}, {"red earth", 0.1}, medium=0.1}
work(ridgeM, {hand="body", tool="filbert 4", pile=hNear4, coverage=2.4, angle=-1.5, angle_jitter=0.25, length={4, 10}, fill=true, clip=ridgeM, pressure={0.6, 0.95}, dips={6, 0.85, 0.5}})
print(wait(0))

--@ chunk 109
print(wait(4 * 24 * 60))
print(drying(700, 380), drying(700, 415), drying(900, 395), drying(560, 420), drying(780, 390))

--@ chunk 110
print(wait(2 * 24 * 60))
print(drying(700, 380), drying(700, 415), drying(900, 395), drying(560, 420), drying(780, 390))

--@ chunk 111
glzRidge = pile{{"raw umber", 1.4}, {"Antwerp blue", 0.4}, {"bone black", 0.3}, {"yellow ochre", 0.3}, medium=0.75}
local m = ridgeM:grow(1)
work(m, {hand="glaze", tool="filbert 10", pile=glzRidge, coverage=1.3, angle=0.0, clip=m, length={30, 80}})
print(wait(0))

--@ chunk 112
local m = ridgeM:grow(1)
blend(m, {angle=0.0})
blend(m, {angle=-1.5, tool={kind="filbert", width=8, stiffness=0.2}, pressure={0.3, 0.5}})
blend(m, {angle=0.0})
print(wait(0))

--@ chunk 113
print(wait(5 * 24 * 60))
print(drying(700, 415), drying(900, 420), drying(780, 390), drying(200, 200), drying(600, 600))

--@ chunk 114
treeKeep2 = (treeAll - openings):grow(1) + crowns:shrink(2)
skyLine2 = {{0, 404}, {200, 404}, {400, 404}, {550, 406}, {700, 406}, {850, 406}, {1000, 404}}
skyAll2 = (above(skyLine2) - treeKeep2)
local nz = noise{seed=2601, period=210, octaves=4, stretch={0.0, 3.5}}
local function yy(x, y) return y + 38 * nz(x, y) + 34 * math.exp(-((x - 740) / 230) ^ 2) end
local function band(a, b) return mask(function(x, y) local v = yy(x, y) return smoothstep(a - 15, a + 15, v) * (1 - smoothstep(b - 15, b + 15, v)) end) * skyAll2 end
local ang = function(x, y) return -0.07 + 0.06 * math.sin(x / 150 + y / 80) end
local opt = function(p) return {hand="broad", pile=p, coverage=1.8, angle=ang, fill=true, clip=skyAll2} end
work(band(-100, 120), opt(s1))
work(band(120, 235), opt(s2))
work(band(235, 318), opt(s3))
work(band(318, 520), opt(s4))
local sk = skyAll2:shrink(2)
blend(sk, {angle=-0.3})
blend(sk, {angle=0.3})
print(wait(0))

--@ chunk 115
local keep = treeKeep2
hillsF = outline{{545, 420}, {548, 402}, {580, 399}, {620, 396}, {660, 393}, {695, 390}, {725, 387}, {752, 385}, {780, 387}, {812, 391}, {845, 394}, {875, 392}, {905, 388}, {935, 384}, {965, 382}, {1000, 384}, {1000, 420}, char="soft", closed=true, seed=2611, amount=0.35, lobe=60}:mask()
work(hillsF - keep, {hand="body", tool="filbert 7", pile=hA2, coverage=2.0, angle=0.0, angle_jitter=0.06, length={30, 70}, fill=true, edge="soft", pressure={0.55, 0.85}, dips={5, 0.8, 0.5}, clip=-keep})
-- left: a low far wood/hill line seen beyond and between the trees
local leftF = outline{{0, 420}, {0, 398}, {40, 396}, {90, 397}, {140, 395}, {200, 396}, {260, 394}, {320, 396}, {380, 398}, {440, 399}, {500, 400}, {550, 402}, {550, 420}, char="soft", closed=true, seed=2612, amount=0.5, lobe=20}:mask()
work(leftF - keep, {hand="body", tool="filbert 6", pile=hA2, coverage=2.0, angle=0.0, length={20, 50}, fill=true, edge="soft", pressure={0.55, 0.85}, clip=-keep})
blend(rect(0, 376, 1000, 30) - keep:grow(3), {angle=0.0, pressure={0.15, 0.3}})
print(wait(0))

--@ chunk 116
local keep = treeKeep2:grow(1)
local hc = {{0, 398}, {40, 396}, {90, 397}, {140, 395}, {200, 396}, {260, 394}, {320, 396}, {380, 398}, {440, 399}, {500, 400}, {548, 402}, {580, 399}, {620, 396}, {660, 393}, {695, 390}, {725, 387}, {752, 385}, {780, 387}, {812, 391}, {845, 394}, {875, 392}, {905, 388}, {935, 384}, {965, 382}, {1000, 384}}
local cut = (above(hc) * mask(function(x, y) return smoothstep(350, 366, y) end)) - keep
work(cut, {hand="body", tool="filbert 6", pile=s4, coverage=2.4, angle=0.0, angle_jitter=0.05, length={20, 50}, fill=true, clip=cut, pressure={0.65, 0.95}, dips={4, 0.9, 0.5}})
print(wait(0))

--@ chunk 117
local core = treeAll - openings
local nz = noise{seed=2701, period=16, octaves=3}
local band = (core:grow(15):roughen(8, 16, 2702) - core:shrink(8)) * above({{0, 398}, {1000, 398}})
local lacy = band * mask(function(x, y) return smoothstep(-0.3, 0.2, nz(x, y)) end)
local ang = function(x, y) return math.atan(y - 330, x - (x < 330 and 200 or 430)) + 0.5 * nz(x + 11, y) end
work(lacy, {hand="body", tool="filbert 5", pile=tMass, coverage=1.4, angle=ang, angle_jitter=0.5, length={6, 15}, edge="lost", pressure={0.3, 0.7}, dips={6, 0.6, 0.3}})
local sb = (crowns:grow(9):roughen(5, 10, 2703) - crowns:shrink(4)) * mask(function(x, y) return smoothstep(-0.25, 0.25, nz(x + 400, y)) end)
local ang2 = function(x, y) local cx = x < 728 and 690 or (x < 805 and 772 or 821) return math.atan(y - 420, x - cx) + 0.4 * nz(x, y) end
work(sb, {hand="body", tool="filbert 3", pile=folM, coverage=1.3, angle=ang2, angle_jitter=0.5, length={4, 10}, edge="lost", pressure={0.3, 0.6}, dips={8, 0.5, 0.3}})
print(wait(0))

--@ chunk 118
groundLine = {{0, 416}, {150, 417}, {300, 419}, {450, 421}, {560, 424}, {700, 427}, {850, 426}, {1000, 425}}
G = below(groundLine)
mFar = pile{{"yellow ochre", 2}, {"raw sienna", 0.8}, {"lemon chrome", 0.35}, {"lead white", 0.9}, {"Antwerp blue", 0.06}, medium=0.1}
mOl = pile{{"yellow ochre", 1.6}, {"raw umber", 1.0}, {"raw sienna", 0.7}, {"Antwerp blue", 0.22}, {"lead white", 0.35}, medium=0.1}
mSh = pile{{"raw umber", 2.0}, {"Antwerp blue", 0.35}, {"yellow ochre", 0.8}, {"bone black", 0.2}, {"raw sienna", 0.3}, medium=0.1}
fgD = pile{{"raw umber", 2.0}, {"raw sienna", 0.9}, {"bone black", 0.3}, {"yellow ochre", 0.5}, {"Antwerp blue", 0.15}, {"red earth", 0.12}, medium=0.12}
local nz = noise{seed=2801, period=140, octaves=4, stretch={0.0, 3}}
local function yy(x, y) return y + 14 * nz(x, y) end
local hz = function(x, y) return 0.02 * math.sin(x / 160) + 0.03 * nz(x * 2, y * 2) end
local opt = function(p, cov) return {hand="body", tool="filbert 12", pile=p, coverage=cov or 1.7, angle=hz, length={40, 110}, fill=true, clip=G, pressure={0.55, 0.9}, dips={4, 0.8, 0.5}} end
-- far strip, lit
work(G * mask(function(x, y) return (1 - smoothstep(448, 462, yy(x, y))) * smoothstep(500, 600, x) end), opt(mFar))
-- under the trees: shadow
work(G * mask(function(x, y) return (1 - smoothstep(470, 490, yy(x, y) + (x - 300) * 0.12)) * (1 - smoothstep(500, 600, x)) end), opt(mSh))
-- mid meadow
work(G * mask(function(x, y) local v = yy(x, y) return smoothstep(448, 462, v) * (1 - smoothstep(560, 585, v)) * (1 - (1 - smoothstep(470, 490, v + (x - 300) * 0.12)) * (1 - smoothstep(500, 600, x))) end), opt(mOl))
-- foreground
work(G * mask(function(x, y) return smoothstep(560, 585, yy(x, y)) end), opt(fgD, 1.9))
print(wait(0))

--@ chunk 119
-- shadows of the slim trees, running toward us across the wet meadow
shStreak = pile{{"raw umber", 2.0}, {"Antwerp blue", 0.4}, {"bone black", 0.25}, {"yellow ochre", 0.5}, medium=0.15}
local fb = brush{kind="filbert", width=7}
local bases = {{700, 466, -0.18, 1.0}, {744, 470, -0.05, 1.0}, {813, 457, 0.12, 0.7}}
for i, b in ipairs(bases) do
  local x0, y0, s, k = b[1], b[2], b[3], b[4]
  fb:reload(shStreak, 0.7)
  local L = 95 * k
  fb:stroke({{x0, y0 + 1}, {x0 + s * L * 0.33, y0 + L * 0.33}, {x0 + s * L * 0.66, y0 + L * 0.66}, {x0 + s * L, y0 + L}}, {pressure={0.35, 0.9}, ramps={0.05, 0.4}, swell={0.7, 1.0, 1.15}, shake=0.6, orient="across"})
end
-- the pool: a narrow gleam with the glow in it
poolN = outline{{700, 548}, {730, 541}, {780, 538}, {840, 539}, {900, 542}, {935, 547}, {915, 553}, {860, 556}, {790, 556}, {730, 554}, char="soft", closed=true, seed=2811, amount=0.5, lobe=20}:mask()
work(poolN, {hand="body", tool="filbert 6", pile=s4, coverage=2.2, angle=0.0, length={20, 60}, fill=true, clip=poolN, pressure={0.6, 0.95}})
local lowEdge = poolN * mask(function(x, y) return smoothstep(549, 556, y) end)
work(lowEdge, {hand="detail", pile=hA2, coverage=1.2, angle=0.0, fill=true})
print(wait(0))

--@ chunk 120
local Gm = G - poolN:grow(3)
blend(Gm, {angle=0.0})
blend(Gm * mask(function(x, y) return smoothstep(440, 470, y) end), {angle=0.04, pressure={0.3, 0.5}})
blend(poolN, {angle=0.0, tool={kind="filbert", width=8, stiffness=0.2}})
print(wait(0))

--@ chunk 121
print(wait(3 * 60))
print(drying(300, 450), drying(700, 500), drying(500, 620), drying(800, 547))

--@ chunk 122
-- cast shadow of the big trees, lying across the meadow toward us
local nz = noise{seed=2901, period=60, octaves=3}
treeShadow = outline{{0, 418}, {120, 418}, {260, 420}, {400, 421}, {520, 423}, {575, 428}, {590, 440}, {560, 452}, {500, 462}, {430, 474}, {360, 486}, {300, 500}, {230, 510}, {150, 516}, {70, 520}, {0, 522}, char="soft", closed=true, seed=2902, amount=1.0, lobe=40}:mask()
work(treeShadow, {hand="body", tool="filbert 10", pile=mSh, coverage=1.6, angle=function(x, y) return 0.02 + 0.04 * nz(x, y) end, length={30, 80}, fill=true, edge={found=0.2, soft=0.5, lost=0.3, period=60}, pressure={0.5, 0.85}, dips={4, 0.8, 0.5}})
print(wait(0))

--@ chunk 123
blend(treeShadow:grow(10) * G, {angle=0.0, pressure={0.4, 0.6}})
blend(treeShadow:grow(10) * G, {angle=0.03, pressure={0.3, 0.5}})
print(wait(0))

--@ chunk 124
print(wait(3 * 24 * 60))
print(drying(300, 450), drying(700, 500), drying(500, 620), drying(800, 547), drying(200, 200), drying(700, 300))

--@ chunk 125
local top = {}
local x = 0
while x <= 610 do
  top[#top + 1] = {x, 396 + rand(-4, 3) + (x > 560 and (x - 560) * 0.35 or 0)}
  x = x + rand(10, 22)
end
local pts = {{0, 430}}
for i, p in ipairs(top) do pts[#pts + 1] = p end
pts[#pts + 1] = {612, 432}
pts[#pts + 1] = {300, 434}
baseM2 = poly(pts, true):roughen(3, 8, 3001)
win1 = poly({{238, 398}, {250, 395}, {290, 394}, {306, 398}, {304, 414}, {240, 414}}, true):roughen(2, 8, 3002)
win2 = poly({{30, 400}, {44, 398}, {70, 399}, {78, 404}, {74, 414}, {32, 414}}, true):roughen(2, 8, 3003)
local m = baseM2 - win1 - win2
work(m, {hand="body", tool="filbert 6", pile=shadeBase, coverage=2.2, angle=-0.05, angle_jitter=0.5, length={8, 22}, fill=true, edge="soft", pressure={0.6, 0.95}, dips={5, 0.9, 0.5}})
local wins = win1 + win2
work(wins * above({{0, 405}, {600, 405}}), {hand="detail", pile=hA2, coverage=1.8, fill=true})
work(wins * below({{0, 405}, {600, 405}}), {hand="detail", pile=mFar, coverage=1.8, fill=true})
print(wait(0))

--@ chunk 126
local w1 = poly({{244, 386}, {300, 386}, {306, 400}, {303, 414}, {241, 414}, {238, 400}}, true):roughen(2, 6, 3011)
local w2 = poly({{0, 392}, {70, 392}, {80, 402}, {76, 414}, {30, 415}, {0, 414}}, true):roughen(2, 6, 3012)
local w = w1 + w2
work(w * above({{0, 398}, {600, 398}}), {hand="detail", pile=s4, coverage=2.0, fill=true})
work(w * below({{0, 398}, {600, 398}}) * above({{0, 406}, {600, 406}}), {hand="detail", pile=hA2, coverage=2.0, fill=true})
work(w * below({{0, 406}, {600, 406}}), {hand="detail", pile=mFar, coverage=2.0, fill=true})
-- trunks: dark, tapering into the canopy, through the windows
local fb = brush{kind="filbert", width=6}
local trunks = {
  {{262, 424}, {261, 408}, {259, 392}, {255, 376}, {252, 360}},
  {{286, 424}, {287, 410}, {290, 396}, {294, 382}, {297, 368}},
  {{52, 424}, {53, 410}, {55, 398}, {58, 386}},
  {{146, 426}, {148, 410}, {146, 396}, {142, 380}},
  {{206, 428}, {205, 412}, {207, 396}, {212, 380}},
  {{392, 428}, {393, 412}, {391, 396}, {388, 380}},
  {{452, 426}, {451, 412}, {455, 398}, {460, 384}}}
for i, t in ipairs(trunks) do
  fb:reload(trD, 0.8)
  fb:stroke(t, {pressure={0.9, 0.4}, ramps={0.05, 0.4}, shake=0.6})
end
print(wait(0))

--@ chunk 127
trunkS = pile{{"raw umber", 1.6}, {"bone black", 0.6}, {"Antwerp blue", 0.15}, {"lead white", 0.35}, {"red earth", 0.1}, medium=0.15}
local rg = brush{kind="round", width=4, point=0.7}
rg:reload(trunkS, 0.9)
rg:stroke({{701, 470}, {700, 440}, {698, 410}, {696, 380}, {694, 355}, {692, 330}}, {pressure={0.85, 0.55}, ramps={0.02, 0.2}, shake=0.5})
rg:reload(trunkS, 0.9)
rg:stroke({{744, 474}, {746, 445}, {748, 415}, {750, 385}, {753, 360}, {756, 335}}, {pressure={0.8, 0.5}, ramps={0.02, 0.2}, shake=0.5})
rg:reload(trunkS, 0.7)
rg:stroke({{813, 462}, {814, 440}, {815, 418}, {817, 398}, {819, 382}}, {pressure={0.6, 0.35}, ramps={0.02, 0.3}, shake=0.5})
print(wait(0))

--@ chunk 128
-- cover the blue skirt of the small tree with foliage
local skirt = ellipse(820, 382, 22, 12):roughen(4, 8, 3101)
local nz = noise{seed=3102, period=10, octaves=2}
work(skirt, {hand="body", tool="filbert 3", pile=folM, coverage=2.0, angle=function(x, y) return math.atan(y - 430, x - 818) + 0.5 * nz(x, y) end, angle_jitter=0.5, length={4, 9}, fill=true, edge="lost", pressure={0.4, 0.8}})
-- a far line of trees on the horizon to the right, low and soft
distT = pile{{"lead white", 0.9}, {"cobalt blue", 0.8}, {"raw umber", 0.8}, {"red earth", 0.25}, {"yellow ochre", 0.3}, {"bone black", 0.1}, medium=0.12}
local pts = {}
local x = 560
while x <= 1004 do
  local h = rand(2, 6)
  if math.random() < 0.25 then h = rand(8, 14) end
  if x > 860 and x < 905 then h = rand(0, 2) end
  pts[#pts + 1] = {x, 424 - h}
  x = x + rand(6, 14)
end
local dl = (below(pts) * above({{560, 430}, {1000, 429}})):roughen(1.5, 5, 3103)
work(dl, {hand="body", tool="filbert 3", pile=distT, coverage=2.2, angle=-1.5, angle_jitter=0.3, length={3, 8}, fill=true, clip=dl, pressure={0.5, 0.9}})
print(wait(0))

--@ chunk 129
local band = rect(548, 405, 456, 27)
local rag = brush{kind="flat", width=8, stiffness=0.7}
local n = 0
for pass = 1, 3 do
  for x = 548, 1000, 5 do
    rag:wipe(1)
    rag:stroke({{x, 432}, {x + rand(-2, 2), 404}}, {pressure={0.95, 0.9}, orient="across", clip=band})
    n = n + 1
  end
end
print(n, wait(0))

--@ chunk 130
print(wait(2 * 24 * 60))
print(drying(700, 420), drying(900, 415), drying(580, 420), drying(260, 410))

--@ chunk 131
-- value tests, small, at the far right edge in the band to be repainted
dT1 = pile{{"cobalt blue", 1.0}, {"raw umber", 0.7}, {"red earth", 0.2}, {"lead white", 0.25}, {"yellow ochre", 0.15}, medium=0.1}
dT2 = pile{{"cobalt blue", 1.0}, {"raw umber", 0.5}, {"red earth", 0.25}, {"lead white", 0.6}, {"yellow ochre", 0.2}, medium=0.1}
local b = brush("filbert", 4)
b:load(dT1, 0.8)
b:stroke({{975, 412}, {995, 412}}, {pressure={0.8, 0.8}})
b:reload(dT2, 0.8)
b:stroke({{975, 420}, {995, 420}}, {pressure={0.8, 0.8}})
b:reload(hA2, 0.8)
b:stroke({{975, 428}, {995, 428}}, {pressure={0.8, 0.8}})
print(wait(0))

--@ chunk 132
local keep = treeKeep2 + crowns:grow(2)
local bandHill = (rect(548, 392, 452, 34) * below({{548, 398}, {600, 397}, {660, 393}, {700, 390}, {752, 385}, {812, 391}, {845, 394}, {905, 388}, {965, 382}, {1000, 384}})) - keep
work(bandHill, {hand="body", tool="filbert 6", pile=hA2, coverage=2.4, angle=0.0, angle_jitter=0.05, length={20, 50}, fill=true, clip=bandHill, pressure={0.65, 0.95}, dips={5, 0.9, 0.5}})
-- far meadow strip top
fieldFar = pile{{"yellow ochre", 2}, {"raw sienna", 0.6}, {"lead white", 1.0}, {"raw umber", 0.3}, {"red earth", 0.1}, medium=0.1}
local strip = rect(548, 424, 452, 12) * below({{548, 426}, {1000, 426}})
work(strip, {hand="body", tool="filbert 6", pile=fieldFar, coverage=2.2, angle=0.0, length={20, 50}, fill=true, clip=strip, pressure={0.65, 0.95}})
print(wait(0))

--@ chunk 133
local pts = {}
local x = 546
while x <= 1004 do
  local base = 414 + 2 * math.sin(x / 70)
  local h = rand(1, 4)
  local r = math.random()
  if r < 0.22 then h = rand(6, 11) end
  pts[#pts + 1] = {x, base - h}
  x = x + rand(5, 12)
end
local keep = crowns:grow(2) + treeKeep2
ridge2 = ((below(pts) * above({{546, 426}, {1000, 426}})):roughen(1.2, 4, 3201)) - keep
work(ridge2, {hand="body", tool="filbert 3", pile=dT1, coverage=2.4, angle=-1.5, angle_jitter=0.3, length={3, 8}, fill=true, clip=ridge2, pressure={0.6, 0.95}, dips={8, 0.9, 0.5}})
print(wait(0))

--@ chunk 134
print(wait(3 * 24 * 60))
print(drying(700, 415), drying(900, 400), drying(700, 430), drying(260, 410))

--@ chunk 135
print(wait(2 * 24 * 60))
print(drying(700, 415), drying(600, 418), drying(700, 430))

--@ chunk 136
print(wait(3 * 24 * 60))
print(drying(700, 415), drying(600, 418), drying(900, 418))

--@ chunk 137
-- deepen the wooded ridge with a glaze, and fix the hill's corner by the big trees
glzR = pile{{"raw umber", 1.0}, {"cobalt blue", 0.8}, {"bone black", 0.15}, {"red earth", 0.15}, medium=0.8}
local r = ridge2:grow(1)
work(r, {hand="glaze", tool="filbert 8", pile=glzR, coverage=1.2, angle=0.0, clip=r, length={30, 80}})
local corner = poly({{540, 384}, {600, 384}, {600, 400}, {560, 400}, {540, 404}}) - (treeKeep2:grow(1))
local hc = {{530, 403}, {560, 402}, {600, 399}, {640, 396}}
work(corner * above(hc), {hand="detail", pile=s4, coverage=2.0, fill=true})
work(corner * below(hc), {hand="detail", pile=hA2, coverage=2.0, fill=true})
print(wait(0))

--@ chunk 138
local r = ridge2:grow(2)
blend(r, {angle=0.0, tool={kind="badger", width=20}})
blend(r, {angle=1.5, tool={kind="filbert", width=6, stiffness=0.2}, pressure={0.3, 0.5}})
blend(r, {angle=0.0, tool={kind="badger", width=20}})
local corner = poly({{540, 380}, {604, 380}, {604, 404}, {540, 406}}) - treeKeep2:grow(2)
blend(corner, {angle=0.0, tool={kind="badger", width=14}})
print(wait(0))

--@ chunk 139
local rg = brush{kind="round", width=4, point=0.7}
rg:reload(trunkS, 0.9)
rg:stroke({{701, 472}, {700, 445}, {699, 420}, {697, 395}, {695, 372}, {693, 350}}, {pressure={0.85, 0.6}, ramps={0.02, 0.2}, shake=0.5})
rg:reload(trunkS, 0.9)
rg:stroke({{744, 476}, {745, 450}, {747, 424}, {749, 400}, {751, 378}, {753, 355}}, {pressure={0.8, 0.55}, ramps={0.02, 0.2}, shake=0.5})
rg:reload(trunkS, 0.7)
rg:stroke({{813, 462}, {814, 445}, {815, 425}, {816, 405}, {817, 390}}, {pressure={0.6, 0.4}, ramps={0.02, 0.3}, shake=0.5})
print(wait(0))

--@ chunk 140
print(wait(3 * 24 * 60))
print(drying(700, 415), drying(300, 420), drying(300, 480), drying(750, 450), drying(700, 380))

--@ chunk 141
mLit = pile{{"yellow ochre", 1.5}, {"Antwerp blue", 0.22}, {"raw umber", 0.5}, {"lead white", 0.6}, {"lemon chrome", 0.25}, medium=0.1}
mLit2 = pile{{"yellow ochre", 1.5}, {"raw sienna", 0.5}, {"lead white", 1.0}, {"lemon chrome", 0.3}, {"Antwerp blue", 0.08}, medium=0.1}
mHalf = pile{{"raw umber", 1.2}, {"yellow ochre", 1.2}, {"Antwerp blue", 0.3}, {"lead white", 0.2}, {"bone black", 0.08}, medium=0.1}
mDeep = pile{{"raw umber", 2}, {"Antwerp blue", 0.4}, {"bone black", 0.3}, {"yellow ochre", 0.5}, {"red earth", 0.1}, medium=0.1}
local b = brush("filbert", 6)
local y = 600
for i, p in ipairs({mLit, mLit2, mHalf, mDeep}) do
  b:reload(p, 0.8)
  b:stroke({{900, y + i * 10}, {990, y + i * 10}}, {pressure={0.8, 0.8}})
end
print(wait(0))

--@ chunk 142
local topLine = {{0, 428}, {200, 430}, {400, 431}, {540, 432}, {600, 433}, {700, 434}, {850, 434}, {1000, 434}}
M = below(topLine)
local shadowPoly = poly({{-20, 420}, {618, 420}, {612, 428}, {592, 440}, {552, 456}, {502, 474}, {442, 494}, {382, 512}, {322, 530}, {262, 548}, {200, 562}, {120, 578}, {40, 590}, {-20, 596}}, true):roughen(10, 70, 3301):soften(10)
shadowZ = shadowPoly
local nz = noise{seed=3302, period=150, octaves=3, stretch={0.0, 4}}
local deepZ = mask(function(x, y)
  local d = (1 - smoothstep(440, 470, y + 12 * nz(x, y))) + smoothstep(560, 600, y + 0.25 * x + 20 * nz(x + 77, y))
  return math.min(1, d)
end)
local hz = function(x, y) return 0.015 * math.sin(x / 170) + 0.04 * nz(x * 1.5, y * 3) end
local function opt(p, cov) return {hand="body", tool="filbert 12", pile=p, coverage=cov or 1.8, angle=hz, length={50, 130}, fill=true, clip=M, pressure={0.55, 0.9}, dips={4, 0.85, 0.5}, edge="soft"} end
-- lit meadow
local litZ = M - shadowZ
local farGlow = mask(function(x, y) return (1 - smoothstep(448, 470, y + 10 * nz(x, y))) end)
work(litZ * farGlow, opt(mLit2))
work(litZ - farGlow, opt(mLit))
-- shadow
work(shadowZ * M * deepZ, opt(mDeep, 2.0))
work(shadowZ * M - deepZ, opt(mHalf))
-- foreground across the whole width
local fgZ = M * mask(function(x, y) return smoothstep(585, 610, y + 15 * nz(x + 300, y)) end)
work(fgZ, opt(mDeep, 2.0))
print(wait(0))

--@ chunk 143
local nz = noise{seed=3302, period=150, octaves=3, stretch={0.0, 4}}
local hz = function(x, y) return 0.015 * math.sin(x / 170) + 0.04 * nz(x * 1.5, y * 3) end
local function opt(p, cov) return {hand="body", tool="filbert 12", pile=p, coverage=cov or 1.8, angle=hz, length={50, 130}, fill=true, clip=M, pressure={0.6, 0.95}, dips={3, 0.9, 0.5}, edge="soft"} end
work(shadowZ:shrink(6) * M, opt(mDeep, 1.5))
local fgZ = M * mask(function(x, y) return smoothstep(580, 605, y + 15 * nz(x + 300, y)) end)
work(fgZ, opt(fgD, 2.2))
print(wait(0))

--@ chunk 144
blend(M:shrink(2), {angle=0.0, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
blend(M:shrink(2) * mask(function(x, y) return smoothstep(440, 460, y) end), {angle=0.08, tool={kind="badger", width=30}, pressure={0.3, 0.5}})
print(wait(0))

--@ chunk 145
local fb = brush{kind="filbert", width=8}
local bases = {{700, 434, -0.22, 1.0}, {745, 435, -0.08, 1.0}, {814, 433, 0.08, 0.8}}
for i, b in ipairs(bases) do
  local x0, y0, s, k = b[1], b[2], b[3], b[4]
  fb:reload(mDeep, 0.5)
  local L = 150 * k
  fb:stroke({{x0, y0 + 1}, {x0 + s * L * 0.3, y0 + L * 0.3}, {x0 + s * L * 0.65, y0 + L * 0.65}, {x0 + s * L, y0 + L}}, {pressure={0.25, 0.6}, ramps={0.05, 0.5}, swell={0.6, 1.0, 1.3, 0.8}, shake=0.8, orient="across"})
end
print(wait(0))

--@ chunk 146
local sh = poly({{690, 436}, {760, 436}, {830, 436}, {840, 600}, {660, 600}}, true)
blend(sh * M, {angle=1.4, tool={kind="filbert", width=14, stiffness=0.2}, pressure={0.3, 0.5}})
blend(sh * M, {angle=0.0, tool={kind="badger", width=24}, pressure={0.3, 0.5}})
print(wait(0))

--@ chunk 147
print(drying(750, 500), drying(600, 500), drying(300, 500), drying(750, 620))

--@ chunk 148
local nz = noise{seed=3401, period=150, octaves=3, stretch={0.0, 4}}
local hz = function(x, y) return 0.015 * math.sin(x / 170) + 0.04 * nz(x * 1.5, y * 3) end
local patch = poly({{640, 436}, {870, 436}, {880, 630}, {630, 630}}, true):soften(25)
local upper = patch * mask(function(x, y) return 1 - smoothstep(570, 600, y) end)
local lower = patch * mask(function(x, y) return smoothstep(570, 600, y) end)
work(upper * M, {hand="body", tool="filbert 12", pile=mLit, coverage=1.6, angle=hz, length={50, 120}, fill=true, clip=M, pressure={0.55, 0.9}, dips={3, 0.85, 0.5}, edge="lost"})
work(lower * M, {hand="body", tool="filbert 12", pile=mDeep, coverage=1.6, angle=hz, length={50, 120}, fill=true, clip=M, pressure={0.55, 0.9}, dips={3, 0.85, 0.5}, edge="lost"})
blend(rect(560, 436, 440, 231) * M:shrink(2), {angle=0.0, tool={kind="badger", width=40}, pressure={0.35, 0.55}})
print(wait(0))

--@ chunk 149
local seam = rect(470, 436, 180, 231):soften(40) * M:shrink(2)
blend(seam, {angle=0.0, tool={kind="badger", width=40}, pressure={0.35, 0.55}})
blend(seam, {angle=0.05, tool={kind="badger", width=30}, pressure={0.3, 0.5}})
print(wait(0))

--@ chunk 150
print(wait(3 * 24 * 60))
print(drying(750, 500), drying(600, 500), drying(300, 500), drying(750, 620), drying(500, 420))

--@ chunk 151
print(wait(3 * 24 * 60))
print(drying(750, 500), drying(600, 500), drying(300, 500), drying(750, 620), drying(100, 600))

--@ chunk 152
bushR = outline{{520, 436}, {522, 400}, {534, 372}, {552, 360}, {566, 372}, {580, 386}, {596, 398}, {612, 408}, {626, 420}, {636, 432}, {630, 440}, {560, 442}, char="soft", closed=true, seed=3501, amount=1.3, lobe=9}:mask()
local nz = noise{seed=3502, period=14, octaves=3}
work(bushR, {hand="body", tool="filbert 5", pile=tMass, coverage=2.2, angle=function(x, y) return -1.0 + 0.8 * nz(x, y) end, angle_jitter=0.5, length={5, 12}, fill=true, edge="lost", pressure={0.5, 0.9}, dips={6, 0.85, 0.4}})
print(wait(0))

--@ chunk 153
glzCrown = pile{{"raw umber", 1.5}, {"Antwerp blue", 0.4}, {"yellow ochre", 0.5}, {"bone black", 0.2}, medium=0.7}
local nz = noise{seed=3601, period=30, octaves=3}
local cm = (crowns:grow(6) * mask(function(x, y)
  local cx = x < 728 and 690 or (x < 805 and 772 or 821)
  return smoothstep(-25, 10, (cx - x) * 0.5 + (y - 250) * 0.6 + 20 * nz(x, y))
end)):soften(4)
work(cm, {hand="glaze", tool="filbert 10", pile=glzCrown, coverage=1.3, angle=-1.3, clip=cm, length={15, 40}})
print(wait(0))

--@ chunk 154
local cm = crowns:shrink(3)
blend(cm, {angle=-1.2, tool={kind="badger", width=12}, pressure={0.25, 0.4}})
blend(cm, {angle=0.4, tool={kind="badger", width=12}, pressure={0.2, 0.35}})
print(wait(0))

--@ chunk 155
haze = pile{{"lead white", 3}, {"yellow ochre", 0.8}, {"cobalt blue", 0.15}, {"red earth", 0.05}, medium=0.35}
local b = brush("filbert", 5)
b:load(haze, 0.6)
b:stroke({{960, 650}, {995, 650}}, {pressure={0.4, 0.4}})
print(wait(0))

--@ chunk 156
local rag = brush{kind="filbert", width=8}
for i = 1, 4 do
  rag:wipe(1)
  rag:stroke({{955, 650}, {1000, 650}}, {pressure={0.9, 0.9}})
end
local strip = mask(function(x, y) return smoothstep(420, 425, y) * (1 - smoothstep(434, 444, y)) * smoothstep(625, 650, x) end)
work(strip, {hand="scumble", tool="filbert 6", pile=haze, coverage=1.0, angle=0.0, length={20, 50}, pressure={0.2, 0.4}, load=0.3, clip=strip})
print(wait(0))

--@ chunk 157
local band = mask(function(x, y) return smoothstep(418, 422, y) * (1 - smoothstep(442, 446, y)) * smoothstep(615, 625, x) end)
local rag = brush{kind="flat", width=10, stiffness=0.7}
local n = 0
for pass = 1, 4 do
  for y = 420, 446, 5 do
    local x = 620
    while x < 1000 do
      rag:wipe(1)
      rag:stroke({{x, y + rand(-1, 1)}, {x + 40, y + rand(-1, 1)}}, {pressure={0.95, 0.9}, orient="across", clip=band})
      x = x + 35
      n = n + 1
    end
  end
end
print(n, wait(0))

--@ chunk 158
print(wait(3 * 24 * 60))
print(drying(700, 430), drying(900, 440), drying(560, 420), drying(700, 300))

--@ chunk 159
print(wait(2 * 24 * 60))
print(drying(700, 430), drying(900, 440), drying(560, 420), drying(580, 400), drying(620, 430))

--@ chunk 160
local keepTr = ribbon({{701, 472}, {700, 445}, {699, 420}, {697, 395}}, 6) + ribbon({{744, 476}, {745, 450}, {747, 424}, {749, 400}}, 6) + ribbon({{813, 462}, {814, 445}, {815, 425}, {816, 405}}, 5) + bushR:grow(2)
-- base of the wood-line
local pts = {}
local x = 618
while x <= 1004 do
  pts[#pts + 1] = {x, 421 + rand(-2, 1)}
  x = x + rand(6, 12)
end
local woodBase = (above({{618, 428}, {1000, 428}}) * below(pts) * mask(function(x, y) return smoothstep(612, 622, x) end)) - keepTr
work(woodBase, {hand="body", tool="filbert 4", pile=dT1, coverage=2.2, angle=-1.5, angle_jitter=0.2, length={3, 8}, fill=true, clip=woodBase, pressure={0.6, 0.9}})
-- far field strip, warm gold, firm on top, lost into meadow below
local farStrip = (below({{618, 427}, {1000, 427}}) * mask(function(x, y) return (1 - smoothstep(438, 452, y)) * smoothstep(612, 625, x) end)) - keepTr
work(farStrip, {hand="body", tool="filbert 8", pile=mLit2, coverage=2.0, angle=0.0, angle_jitter=0.03, length={30, 70}, fill=true, clip=farStrip, pressure={0.6, 0.9}, dips={4, 0.8, 0.5}})
print(wait(0))

--@ chunk 161
local soft = mask(function(x, y) return smoothstep(432, 442, y) * (1 - smoothstep(452, 466, y)) * smoothstep(625, 660, x) end)
blend(soft, {angle=0.0, tool={kind="badger", width=24}, pressure={0.35, 0.55}})
blend(soft, {angle=1.3, tool={kind="badger", width=16}, pressure={0.25, 0.4}})
local band2 = mask(function(x, y) return smoothstep(418, 421, y) * (1 - smoothstep(428, 432, y)) * smoothstep(625, 640, x) end)
blend(band2, {angle=0.0, tool={kind="badger", width=12}, pressure={0.3, 0.5}})
-- the hole in the bush
local hole = ellipse(628, 436, 14, 8)
work(hole, {hand="body", tool="filbert 4", pile=tMass, coverage=2.2, angle=-1.0, length={4, 9}, fill=true, edge="lost", pressure={0.6, 0.9}})
-- trunks down to the ground
local rg = brush{kind="round", width=4, point=0.7}
for i, t in ipairs({{{699, 410}, {700, 430}, {700, 452}, {701, 472}}, {{748, 410}, {747, 432}, {746, 454}, {744, 476}}, {{816, 405}, {815, 425}, {814, 445}, {813, 462}}}) do
  rg:reload(trunkS, 0.9)
  rg:stroke(t, {pressure={0.6, 0.85}, ramps={0.2, 0.05}, shake=0.4})
end
print(wait(0))

--@ chunk 162
print(wait(4 * 24 * 60))
print(drying(700, 440), drying(900, 440), drying(620, 436), drying(200, 200), drying(980, 650))

--@ chunk 163
glzWarm = pile{{"raw sienna", 1.0}, {"red earth", 0.25}, {"raw umber", 0.3}, medium=0.85}
local nz = noise{seed=3801, period=70, octaves=3}
local inside = (treeAll - openings):shrink(6)
local patches = inside * mask(function(x, y) return smoothstep(-0.05, 0.3, nz(x, y)) end):soften(8)
work(patches, {hand="glaze", tool="filbert 14", pile=glzWarm, coverage=1.0, angle=-0.6, clip=patches, length={30, 80}})
print(wait(0))

--@ chunk 164
local inside = (treeAll - openings):shrink(4)
for i, a in ipairs({-0.6, 0.9, 0.1}) do
  blend(inside, {angle=a, tool={kind="badger", width=30}, pressure={0.4, 0.6}})
end
print(wait(0))

--@ chunk 165
print(drying(700, 80), drying(900, 350), drying(500, 200), drying(300, 300))
-- test strokes for cloud colors at the top right corner, small
cT1 = pile{{"lead white", 4}, {"cobalt blue", 0.5}, {"red earth", 0.3}, {"raw umber", 0.25}, {"yellow ochre", 0.2}, medium=0.3}
cT2 = pile{{"lead white", 5}, {"red earth", 0.35}, {"orange chrome", 0.15}, {"yellow ochre", 0.4}, medium=0.3}
local b = brush("filbert", 4)
b:load(cT1, 0.6)
b:stroke({{965, 10}, {995, 10}}, {pressure={0.6, 0.6}})
b:reload(cT2, 0.6)
b:stroke({{965, 20}, {995, 20}}, {pressure={0.6, 0.6}})
print(wait(0))

--@ chunk 166
local rag = brush{kind="filbert", width=6}
for i = 1, 4 do
  rag:wipe(1)
  rag:stroke({{960, 20}, {1000, 20}}, {pressure={0.9, 0.9}})
  rag:wipe(1)
  rag:stroke({{960, 10}, {1000, 10}}, {pressure={0.9, 0.9}})
end
cV = pile{{"lead white", 3}, {"cobalt blue", 0.6}, {"red earth", 0.35}, {"raw umber", 0.4}, {"yellow ochre", 0.15}, medium=0.3}
local b = brush("filbert", 4)
b:load(cV, 0.5)
b:stroke({{600, 60}, {640, 58}}, {pressure={0.5, 0.5}})
print(wait(0))

--@ chunk 167
local keep = treeKeep2:grow(3) + crowns:grow(6)
local nz = noise{seed=3901, period=90, octaves=4, stretch={0.0, 2.5}}
cloudBank = mask(function(x, y)
  local bottom = 95 + 25 * nz(x, 0) + 30 * smoothstep(400, 1000, x)
  local left = smoothstep(380, 520, x + 40 * nz(x, y))
  return left * (1 - smoothstep(bottom - 12, bottom + 12, y + 18 * nz(x * 1.7, y * 1.7)))
end) - keep
work(cloudBank, {hand="scumble", tool="filbert 12", pile=cV, coverage=1.2, angle=function(x, y) return -0.06 + 0.08 * nz(x, y) end, length={20, 50}, edge="lost", pressure={0.35, 0.6}, load=0.5, clip=-keep})
local bankSoft = cloudBank:grow(30):soften(25) - keep
blend(bankSoft, {angle=-0.05, tool={kind="badger", width=40}, pressure={0.35, 0.55}})
print(wait(0))

--@ chunk 168
local keep = treeKeep2:grow(3) + crowns:grow(6)
local nz = noise{seed=3911, period=60, octaves=3}
-- lit underside, worked into the wet bank's lower edge
local belly = mask(function(x, y)
  local bottom = 95 + 25 * 0 + 30 * smoothstep(400, 1000, x)
  return smoothstep(380, 560, x) * smoothstep(bottom - 40, bottom - 10, y + 15 * nz(x, y)) * (1 - smoothstep(bottom + 5, bottom + 30, y + 15 * nz(x, y)))
end) - keep
work(belly, {hand="scumble", tool="filbert 8", pile=cT2, coverage=0.9, angle=function(x, y) return -0.1 + 0.15 * nz(x, y) end, length={15, 40}, edge="lost", pressure={0.3, 0.55}, load=0.4, clip=-keep})
local reg = (rect(360, 0, 640, 220):soften(30)) - keep
blend(reg, {angle=-0.5, tool={kind="badger", width=40}, pressure={0.35, 0.55}})
blend(reg, {angle=0.35, tool={kind="badger", width=40}, pressure={0.3, 0.5}})
blend(reg, {angle=-0.05, tool={kind="badger", width=40}, pressure={0.3, 0.5}})
print(wait(0))

--@ chunk 169
local keep = treeKeep2:grow(4) + crowns:grow(6)
local seam = mask(function(x, y) return smoothstep(230, 320, x) * (1 - smoothstep(380, 470, x)) * (1 - smoothstep(170, 220, y)) end) - keep
blend(seam, {angle=0.0, tool={kind="badger", width=30}, pressure={0.35, 0.55}})
blend(seam, {angle=-0.6, tool={kind="badger", width=30}, pressure={0.3, 0.5}})
blend(seam, {angle=0.5, tool={kind="badger", width=30}, pressure={0.3, 0.5}})
print(wait(0))

--@ chunk 170
print(wait(3 * 24 * 60))
print(drying(450, 160), drying(700, 100), drying(300, 80), drying(150, 300))

--@ chunk 171
print(wait(2 * 24 * 60))
print(drying(450, 160), drying(700, 100), drying(900, 60), drying(600, 150))

--@ chunk 172
local core = treeAll - openings
local nz = noise{seed=4001, period=15, octaves=3}
local zone = poly({{100, 0}, {560, 0}, {560, 330}, {320, 330}, {100, 120}})
local band = (core:grow(13):roughen(7, 14, 4002) - core:shrink(6)) * zone * above({{0, 395}, {1000, 395}})
local lacy = band * mask(function(x, y) return smoothstep(-0.3, 0.2, nz(x, y)) end)
local ang = function(x, y) return math.atan(y - 330, x - (x < 330 and 200 or 430)) + 0.5 * nz(x + 11, y) end
work(lacy, {hand="body", tool="filbert 4", pile=tMass, coverage=1.5, angle=ang, angle_jitter=0.5, length={5, 12}, edge="lost", pressure={0.35, 0.75}, dips={6, 0.6, 0.3}})
print(wait(0))

--@ chunk 173
print(drying(150, 410), drying(270, 405), drying(400, 380), drying(50, 405))

--@ chunk 174
-- one window under the trees, everything else in deep shade joining canopy and ground
winA = outline{{236, 420}, {234, 404}, {240, 392}, {252, 386}, {270, 384}, {290, 386}, {304, 393}, {310, 405}, {308, 420}, char="soft", closed=true, seed=4101, amount=0.8, lobe=8}:mask()
local shadeZone = (poly({{-10, 446}, {-10, 372}, {60, 368}, {140, 366}, {240, 364}, {330, 366}, {420, 368}, {520, 372}, {560, 380}, {580, 400}, {590, 440}, {300, 450}}, true):roughen(6, 14, 4102)) - winA
underDk = pile{{"raw umber", 2}, {"Antwerp blue", 0.45}, {"bone black", 0.35}, {"yellow ochre", 0.45}, {"raw sienna", 0.3}, medium=0.12}
work(shadeZone * mask(function(x, y) return smoothstep(360, 395, y) end), {hand="body", tool="filbert 8", pile=underDk, coverage=2.2, angle=function(x, y) return -1.2 + 0.5 * math.sin(x / 31) end, angle_jitter=0.5, length={8, 22}, fill=true, edge="soft", pressure={0.6, 0.95}, dips={5, 0.9, 0.5}})
-- inside the window: sky, distant line, far lit field
work(winA * above({{220, 402}, {320, 402}}), {hand="detail", pile=s4, coverage=2.2, fill=true})
work(winA * below({{220, 402}, {320, 402}}) * above({{220, 409}, {320, 409}}), {hand="detail", pile=hA2, coverage=2.2, fill=true})
work(winA * below({{220, 409}, {320, 409}}), {hand="detail", pile=mLit2, coverage=2.2, fill=true})
print(wait(0))

--@ chunk 175
local nz = noise{seed=4201, period=18, octaves=3}
-- cover the old light ring above the window and break the band's straight top edge with foliage lobes
local seam = mask(function(x, y) return smoothstep(340, 356, y + 10 * nz(x, y)) * (1 - smoothstep(378, 392, y + 10 * nz(x + 50, y))) end) * poly({{0, 300}, {600, 300}, {600, 420}, {0, 420}}) - winA:grow(3)
local oldRing = ellipse(270, 355, 60, 28):roughen(5, 10, 4202)
work(oldRing - winA:grow(2), {hand="body", tool="filbert 6", pile=tMass, coverage=2.0, angle=function(x, y) return -1.0 + 0.8 * nz(x, y) end, angle_jitter=0.5, length={6, 14}, fill=true, edge="lost", pressure={0.5, 0.9}})
work(seam * mask(function(x, y) return x > 80 and 1 or 0 end), {hand="body", tool="filbert 5", pile=tMass, coverage=1.4, angle=function(x, y) return -1.3 + 0.9 * nz(x, y) end, angle_jitter=0.6, length={5, 12}, edge="lost", pressure={0.45, 0.85}})
print(wait(0))

--@ chunk 176
local nz = noise{seed=4301, period=8, octaves=2}
-- foliage hanging into the window top and sides
local hang = (winA:grow(3) - winA:shrink(5)) * mask(function(x, y) return smoothstep(-0.2, 0.3, nz(x, y)) end) * above({{220, 404}, {320, 404}})
work(hang, {hand="body", tool="filbert 3", pile=tMass, coverage=1.6, angle=function(x, y) return 1.3 + 0.6 * nz(x, y) end, angle_jitter=0.6, length={4, 9}, edge="lost", pressure={0.4, 0.8}})
-- trunks crossing the window
local fb = brush{kind="filbert", width=6}
fb:reload(trD, 0.9)
fb:stroke({{262, 440}, {261, 420}, {259, 400}, {256, 380}, {252, 360}, {248, 340}}, {pressure={0.95, 0.55}, ramps={0.03, 0.3}, shake=0.5})
fb:reload(trD, 0.7)
local fb2 = brush{kind="filbert", width=3.5}
fb2:reload(trD, 0.8)
fb2:stroke({{291, 438}, {292, 420}, {294, 402}, {297, 386}, {300, 370}}, {pressure={0.9, 0.5}, ramps={0.03, 0.3}, shake=0.5})
-- faint trunks in the shade, a shade warmer than the dark
trFaint = pile{{"raw umber", 2}, {"bone black", 0.3}, {"red earth", 0.2}, {"lead white", 0.35}, {"yellow ochre", 0.2}, medium=0.15}
local fb3 = brush{kind="filbert", width=7}
for i, t in ipairs({{{150, 440}, {151, 420}, {149, 398}, {146, 378}}, {{400, 442}, {401, 420}, {399, 398}, {395, 378}}, {{462, 440}, {461, 422}, {464, 404}}}) do
  fb3:reload(trFaint, 0.5)
  fb3:stroke(t, {pressure={0.5, 0.25}, ramps={0.05, 0.6}, shake=0.5})
end
print(wait(0))

--@ chunk 177
local fb3 = brush{kind="filbert", width=9}
for i, t in ipairs({{{150, 442}, {151, 420}, {149, 398}, {146, 376}}, {{400, 444}, {401, 420}, {399, 398}, {395, 376}}, {{462, 442}, {461, 422}, {464, 402}}}) do
  fb3:reload(underDk, 0.9)
  fb3:stroke(t, {pressure={0.8, 0.8}, ramps={0.05, 0.1}, shake=0.4})
end
-- fainter, darker trunk hints
trHint = pile{{"raw umber", 1.5}, {"bone black", 0.6}, {"Antwerp blue", 0.1}, medium=0.15}
local fb4 = brush{kind="filbert", width=6}
for i, t in ipairs({{{152, 440}, {151, 420}, {148, 398}, {144, 378}}, {{402, 442}, {401, 420}, {398, 398}, {394, 378}}}) do
  fb4:reload(trHint, 0.6)
  fb4:stroke(t, {pressure={0.7, 0.3}, ramps={0.05, 0.6}, shake=0.5})
end
-- break the left opening's straight lower edge
local nz = noise{seed=4401, period=10, octaves=2}
local leftEdge = rect(0, 355, 90, 25) * mask(function(x, y) return smoothstep(-0.2, 0.3, nz(x, y)) end)
work(leftEdge, {hand="body", tool="filbert 4", pile=tMass, coverage=1.6, angle=function(x, y) return -1.3 + 0.6 * nz(x, y) end, angle_jitter=0.5, length={4, 10}, edge="lost", pressure={0.4, 0.8}})
print(wait(0))

--@ chunk 178
print(wait(3 * 24 * 60))
print(drying(300, 420), drying(600, 440), drying(300, 500), drying(800, 600), drying(60, 370))

--@ chunk 179
print(wait(3 * 24 * 60))
print(drying(300, 420), drying(150, 430), drying(400, 400), drying(500, 440))

--@ chunk 180
-- grass and low growth at the foot of the grove, breaking the straight base
grassSh = pile{{"raw umber", 1.6}, {"Antwerp blue", 0.3}, {"yellow ochre", 0.9}, {"bone black", 0.15}, {"raw sienna", 0.3}, medium=0.15}
local nz = noise{seed=4501, period=40, octaves=3}
local foot = mask(function(x, y)
  local base = 452 + 6 * nz(x, 0) - (x > 560 and (x - 560) * 0.1 or 0)
  return smoothstep(base - 12, base - 4, y) * (1 - smoothstep(base + 4, base + 18, y + 8 * nz(x * 3, y)))
end) * rect(0, 400, 640, 100)
work(foot, {hand="body", tool="filbert 4", pile=grassSh, coverage=1.6, angle=function(x, y) return -1.45 + 0.35 * nz(x * 2, y * 2) end, angle_jitter=0.4, length={5, 14}, edge="lost", pressure={0.4, 0.8}, dips={6, 0.7, 0.4}})
print(wait(0))

--@ chunk 181
local nz = noise{seed=4501, period=40, octaves=3}
local foot = mask(function(x, y)
  local base = 452 + 6 * nz(x, 0) - (x > 560 and (x - 560) * 0.1 or 0)
  return smoothstep(base - 16, base - 6, y) * (1 - smoothstep(base + 6, base + 20, y + 8 * nz(x * 3, y)))
end) * rect(0, 400, 640, 100)
work(foot, {hand="body", tool="filbert 5", pile=underDk, coverage=1.8, angle=function(x, y) return -1.45 + 0.35 * nz(x * 2, y * 2) end, angle_jitter=0.4, length={6, 16}, fill=true, edge="lost", pressure={0.5, 0.9}, dips={5, 0.85, 0.4}})
print(wait(0))

--@ chunk 182
local nz = noise{seed=4601, period=110, octaves=4, stretch={0.0, 2.5}}
local fgZ = mask(function(x, y) return smoothstep(575, 615, y + 18 * nz(x, y)) end)
local ang = function(x, y) return -0.06 + 0.12 * nz(x * 2, y) end
work(fgZ, {hand="broad", pile=fgD, coverage=1.6, angle=ang, fill=true, edge="lost", pressure={0.5, 0.85}})
local rust = fgZ * mask(function(x, y) return smoothstep(0.05, 0.4, nz(x + 500, y * 1.5)) end)
work(rust, {hand="scumble", tool="filbert 9", pile=fgRust, coverage=0.9, angle=ang, length={15, 40}, edge="lost", pressure={0.4, 0.7}, load=0.5})
local deep = fgZ * mask(function(x, y) return smoothstep(0.1, 0.45, nz(x + 900, y * 1.2)) * smoothstep(610, 650, y) end)
work(deep, {hand="scumble", tool="filbert 9", pile=underDk, coverage=1.0, angle=ang, length={15, 40}, edge="lost", pressure={0.4, 0.7}, load=0.6})
print(wait(0))

--@ chunk 183
local nz = noise{seed=4601, period=110, octaves=4, stretch={0.0, 2.5}}
local fgB = mask(function(x, y) return smoothstep(555, 600, y + 18 * nz(x, y)) end)
blend(fgB, {angle=0.0, tool={kind="badger", width=40}, pressure={0.5, 0.7}})
blend(fgB, {angle=0.5, tool={kind="badger", width=40}, pressure={0.45, 0.65}})
blend(fgB, {angle=-0.4, tool={kind="badger", width=40}, pressure={0.45, 0.65}})
blend(fgB, {angle=0.02, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
print(wait(0))

--@ chunk 184
shGlz = pile{{"raw umber", 1.5}, {"Antwerp blue", 0.3}, {"bone black", 0.2}, {"yellow ochre", 0.3}, medium=0.8}
local fb = brush{kind="filbert", width=6, stiffness=0.4}
local bases = {{701, 472, -0.25, 1.0}, {745, 476, -0.1, 1.0}, {814, 463, 0.05, 0.8}}
for i, b in ipairs(bases) do
  local x0, y0, s, k = b[1], b[2], b[3], b[4]
  local L = 85 * k
  for j = 1, 2 do
    fb:reload(shGlz, 0.6)
    fb:stroke({{x0, y0 + 1}, {x0 + s * L * 0.35, y0 + L * 0.35}, {x0 + s * L * 0.7, y0 + L * 0.7}, {x0 + s * L, y0 + L}}, {pressure={0.4, 0.75}, ramps={0.05, 0.5}, swell={0.7, 1.0, 1.3, 0.9}, shake=0.7, orient="across"})
  end
end
print(wait(0))

--@ chunk 185
local m = (ribbon({{701, 472}, {680, 557}}, 30) + ribbon({{745, 476}, {736, 561}}, 30) + ribbon({{814, 463}, {817, 531}}, 26)):soften(10)
blend(m, {angle=0.0, tool={kind="badger", width=20}, pressure={0.5, 0.7}})
blend(m, {angle=1.4, tool={kind="badger", width=20}, pressure={0.4, 0.6}})
blend(m, {angle=0.0, tool={kind="badger", width=20}, pressure={0.4, 0.6}})
print(wait(0))

--@ chunk 186
print(wait(4 * 24 * 60))
print(drying(700, 520), drying(500, 620), drying(300, 460), drying(800, 640))

--@ chunk 187
local nz = noise{seed=4701, period=120, octaves=4, stretch={0.0, 3}}
-- right half of the meadow, lit, from far gold strip down to the foreground
local Rm = mask(function(x, y)
  return smoothstep(430, 445, y) * (1 - smoothstep(560, 590, y + 15 * nz(x, y))) * smoothstep(560, 640, x + 30 * nz(x, y + 200))
end)
local hz = function(x, y) return 0.012 * math.sin(x / 170) + 0.035 * nz(x * 1.5, y * 3) end
local top = Rm * mask(function(x, y) return 1 - smoothstep(452, 475, y + 8 * nz(x, y)) end)
local rest = Rm - top
work(top, {hand="body", tool="filbert 10", pile=mLit2, coverage=1.8, angle=hz, length={40, 100}, fill=true, edge="lost", pressure={0.55, 0.9}, dips={4, 0.85, 0.5}})
work(rest, {hand="body", tool="filbert 12", pile=mLit, coverage=1.8, angle=hz, length={50, 120}, fill=true, edge="lost", pressure={0.55, 0.9}, dips={4, 0.85, 0.5}})
print(wait(0))

--@ chunk 188
-- shadows of the slim trees, wet into wet, as soft dark horizontal-ish drags
local fb = brush{kind="filbert", width=9}
local bases = {{701, 470, -0.3}, {745, 474, -0.12}, {814, 462, 0.04}}
for i, b in ipairs(bases) do
  local x0, y0, s = b[1], b[2], b[3]
  local L = (i == 3) and 60 or 85
  fb:reload(mDeep, 0.45)
  fb:stroke({{x0, y0}, {x0 + s * L * 0.4, y0 + L * 0.4}, {x0 + s * L * 0.75, y0 + L * 0.75}, {x0 + s * L, y0 + L}}, {pressure={0.3, 0.55}, ramps={0.05, 0.6}, swell={0.6, 1.0, 1.2, 0.8}, shake=0.9, orient="across"})
end
-- bottom transition into the foreground: dark warm drags over the wet lower edge
local nz = noise{seed=4702, period=90, octaves=3}
local tr = mask(function(x, y) return smoothstep(540, 560, y + 12 * nz(x, y)) * (1 - smoothstep(590, 610, y + 12 * nz(x, y))) * smoothstep(540, 600, x) end)
work(tr, {hand="body", tool="filbert 12", pile=fgD, coverage=1.3, angle=function(x, y) return 0.03 * nz(x * 2, y) end, length={40, 100}, edge="lost", pressure={0.45, 0.8}, load=0.6})
print(wait(0))

--@ chunk 189
local nz = noise{seed=4703, period=90, octaves=3}
local Bm = mask(function(x, y) return smoothstep(436, 450, y) * (1 - smoothstep(600, 625, y)) * smoothstep(520, 620, x + 20 * nz(x, y)) end)
blend(Bm, {angle=0.0, tool={kind="badger", width=40}, pressure={0.45, 0.65}})
blend(Bm, {angle=0.12, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
blend(Bm * mask(function(x, y) return smoothstep(530, 560, y) end), {angle=-0.1, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
print(wait(0))

--@ chunk 190
print(wait(5 * 24 * 60))
print(drying(700, 520), drying(500, 620), drying(300, 460), drying(800, 640), drying(560, 450))

--@ chunk 191
local nz = noise{seed=4801, period=14, octaves=3}
footR = outline{{500, 470}, {505, 440}, {540, 434}, {580, 432}, {610, 428}, {632, 426}, {650, 432}, {660, 444}, {652, 456}, {620, 462}, {580, 468}, {540, 474}, char="soft", closed=true, seed=4802, amount=1.2, lobe=8}:mask()
work(footR, {hand="body", tool="filbert 5", pile=underDk, coverage=2.2, angle=function(x, y) return -1.4 + 0.5 * nz(x, y) end, angle_jitter=0.5, length={5, 13}, fill=true, edge="lost", pressure={0.55, 0.9}, dips={5, 0.85, 0.4}})
print(wait(0))

--@ chunk 192
mLitB = pile{{"yellow ochre", 1.5}, {"raw umber", 0.55}, {"Antwerp blue", 0.16}, {"lead white", 0.7}, {"red earth", 0.06}, medium=0.1}
mMidB = pile{{"yellow ochre", 1.2}, {"raw umber", 1.0}, {"Antwerp blue", 0.25}, {"lead white", 0.35}, {"bone black", 0.05}, medium=0.1}
local b = brush("filbert", 6)
b:load(mLitB, 0.8); b:stroke({{820, 500}, {880, 500}}, {pressure={0.8, 0.8}})
b:reload(mMidB, 0.8); b:stroke({{820, 512}, {880, 512}}, {pressure={0.8, 0.8}})
b:reload(mHalf, 0.8); b:stroke({{820, 524}, {880, 524}}, {pressure={0.8, 0.8}})
b:reload(mDeep, 0.8); b:stroke({{820, 536}, {880, 536}}, {pressure={0.8, 0.8}})
print(wait(0))

--@ chunk 193
mLitG = pile{{"yellow ochre", 1.5}, {"raw umber", 0.4}, {"Antwerp blue", 0.18}, {"lead white", 0.7}, {"lemon chrome", 0.12}, medium=0.1}
local nz = noise{seed=4901, period=130, octaves=4, stretch={0.0, 3}}
local topLine = {{0, 474}, {150, 476}, {300, 478}, {450, 478}, {520, 474}, {580, 468}, {630, 462}, {655, 452}, {672, 446}, {800, 446}, {1000, 446}}
meadowB = below(topLine):soften(3)
local function shadowSide(x, y)
  local c = (160 * (x - 650) + 200 * (y - 460)) / 256 + 25 * nz(x + 300, y)
  return 1 - smoothstep(-30, 30, c)
end
local function zy(x, y) return y + 10 * nz(x, y) end
local hz = function(x, y) return 0.012 * math.sin(x / 170) + 0.035 * nz(x * 1.5, y * 3) end
local function go(m, p, cov) work(m * meadowB, {hand="body", tool="filbert 12", pile=p, coverage=cov or 1.7, angle=hz, length={50, 120}, fill=true, clip=meadowB, pressure={0.55, 0.9}, dips={4, 0.85, 0.5}}) end
-- lit side
go(mask(function(x, y) local v = zy(x, y) return (1 - shadowSide(x, y)) * (1 - smoothstep(510, 530, v)) end), mLitG)
go(mask(function(x, y) local v = zy(x, y) return (1 - shadowSide(x, y)) * smoothstep(510, 530, v) * (1 - smoothstep(555, 575, v)) end), mMidB)
go(mask(function(x, y) local v = zy(x, y) return (1 - shadowSide(x, y)) * smoothstep(555, 575, v) * (1 - smoothstep(588, 605, v)) end), mHalf)
-- shadow side
go(mask(function(x, y) local v = zy(x, y) return shadowSide(x, y) * (1 - smoothstep(588, 605, v)) end), mDeep)
-- foreground
go(mask(function(x, y) local v = zy(x, y) return smoothstep(588, 605, v) end), fgD, 1.9)
print(wait(0))

--@ chunk 194
local m = meadowB * below({{0, 480}, {150, 482}, {300, 484}, {450, 484}, {520, 480}, {580, 474}, {630, 468}, {655, 458}, {672, 452}, {800, 452}, {1000, 452}}):soften(6)
blend(m, {angle=0.0, tool={kind="badger", width=40}, pressure={0.45, 0.65}})
blend(m, {angle=-0.35, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
blend(m, {angle=0.03, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
print(wait(0))

--@ chunk 195
local fb = brush{kind="filbert", width=8}
local bases = {{701, 452, -0.35}, {746, 452, -0.18}, {815, 448, -0.02}}
for i, b in ipairs(bases) do
  local x0, y0, s = b[1], b[2], b[3]
  local L = (i == 3) and 55 or 80
  fb:reload(mDeep, 0.4)
  fb:stroke({{x0, y0}, {x0 + s * L * 0.4, y0 + L * 0.4}, {x0 + s * L * 0.75, y0 + L * 0.75}, {x0 + s * L, y0 + L}}, {pressure={0.25, 0.5}, ramps={0.05, 0.6}, swell={0.6, 1.0, 1.25, 0.8}, shake=0.9, orient="across"})
end
print(wait(0))

--@ chunk 196
local m = (ribbon({{701, 452}, {673, 532}}, 26) + ribbon({{746, 452}, {732, 532}}, 26) + ribbon({{815, 448}, {814, 503}}, 22)):soften(10)
blend(m, {angle=0.0, tool={kind="badger", width=18}, pressure={0.35, 0.5}})
blend(m, {angle=0.25, tool={kind="badger", width=18}, pressure={0.3, 0.45}})
-- trunks down to their feet in the meadow
local rg = brush{kind="round", width=4, point=0.7}
for i, t in ipairs({{{699, 420}, {700, 435}, {700, 446}, {701, 455}}, {{748, 420}, {747, 436}, {746, 446}, {746, 455}}, {{815, 418}, {815, 432}, {815, 442}, {815, 450}}}) do
  rg:reload(trunkS, 0.9)
  rg:stroke(t, {pressure={0.7, 0.85}, ramps={0.1, 0.05}, shake=0.4})
end
print(wait(0))

--@ chunk 197
print(wait(4 * 24 * 60))
print(drying(700, 520), drying(500, 620), drying(300, 500), drying(800, 640), drying(600, 480))

--@ chunk 198
local nz = noise{seed=5001, period=60, octaves=3}
baseBand = mask(function(x, y)
  local top = 436 + 4 * nz(x, 0)
  local bot = 488 + 10 * nz(x * 2, 5) - (x > 520 and (x - 520) * 0.15 or 0)
  return smoothstep(top - 6, top + 4, y) * (1 - smoothstep(bot - 14, bot + 6, y)) * (1 - smoothstep(600, 660, x + 20 * nz(x, y)))
end)
work(baseBand, {hand="body", tool="filbert 10", pile=mDeep, coverage=1.8, angle=function(x, y) return 0.03 * nz(x * 2, y) end, length={30, 80}, fill=true, edge="lost", pressure={0.55, 0.9}, dips={4, 0.85, 0.5}})
print(wait(0))

--@ chunk 199
redP = pile{{"red earth", 1.0}, {"Indian red", 0.4}, {"orange chrome", 0.35}, {"lead white", 0.15}, medium=0.1}
skirtP = pile{{"raw umber", 1.5}, {"bone black", 0.5}, {"Antwerp blue", 0.2}, {"lead white", 0.2}, medium=0.1}
headP = pile{{"raw umber", 1.0}, {"red earth", 0.3}, {"lead white", 0.6}, {"yellow ochre", 0.3}, medium=0.1}
local fx, fy = 586, 512
skirtM = poly({{fx - 3, fy - 17}, {fx + 3, fy - 17}, {fx + 5.5, fy}, {fx - 5.5, fy}}, false):soften(0.6)
shawlM = poly({{fx - 3.6, fy - 26}, {fx + 3.2, fy - 26.5}, {fx + 4.5, fy - 17}, {fx + 1, fy - 14}, {fx - 4.8, fy - 16}}, true):soften(0.5)
headM = ellipse(fx - 0.3, fy - 29.5, 2.3, 2.7):soften(0.5)
work(skirtM, {hand="detail", pile=skirtP, coverage=3, fill=true, tool={kind="round", width=2, point=0.3}})
work(shawlM, {hand="detail", pile=redP, coverage=3, fill=true, tool={kind="round", width=2, point=0.3}})
work(headM, {hand="detail", pile=headP, coverage=3, fill=true, tool={kind="round", width=1.6, point=0.3}})
print(wait(0))

--@ chunk 200
local fx, fy = 586, 512
local t = brush{kind="round", width=2.2, point=0.5}
t:load(skirtP, 0.5)
t:touch(fx - 0.5, fy - 30.5, {pressure=0.5})
-- cast shadow toward us, lower left
local s = brush{kind="filbert", width=4}
s:load(mDeep, 0.5)
s:stroke({{fx, fy + 0.5}, {fx - 8, fy + 6}, {fx - 18, fy + 11}}, {pressure={0.6, 0.15}, ramps={0.05, 0.6}, shake=0.4})
-- soft the outline a touch: a small badger kiss
blend((skirtM + shawlM):grow(1.5), {tool={kind="badger", width=6}, pressure={0.15, 0.25}, angle=1.5, coverage=0.5})
print(wait(0))

--@ chunk 201
sunP = pile{{"lead white", 10}, {"lemon chrome", 0.9}, {"cadmium yellow", 0.15}, {"orange chrome", 0.05}, medium=0.15}
local b = brush("filbert", 4)
b:load(sunP, 0.6)
b:stroke({{960, 360}, {990, 360}}, {pressure={0.5, 0.5}})
print(wait(0))

--@ chunk 202
local rag = brush{kind="filbert", width=6}
for i = 1, 4 do rag:wipe(1); rag:stroke({{955, 360}, {995, 360}}, {pressure={0.9, 0.9}}) end
fgGlz = pile{{"raw sienna", 1.0}, {"raw umber", 0.8}, {"red earth", 0.15}, medium=0.85}
local nz = noise{seed=5101, period=120, octaves=3}
local g = mask(function(x, y) return smoothstep(600, 650, y + 15 * nz(x, y)) end)
work(g, {hand="glaze", pile=fgGlz, coverage=1.0, angle=function(x, y) return 0.04 * nz(x, y) end, clip=g})
print(wait(0))

--@ chunk 203
local nz = noise{seed=5101, period=120, octaves=3}
local g = mask(function(x, y) return smoothstep(600, 650, y + 15 * nz(x, y)) end)
blend(g, {angle=0.0, tool={kind="badger", width=40}, pressure={0.3, 0.5}})
print(wait(0))

--@ chunk 204
print(wait(4 * 24 * 60))
print(drying(586, 500), drying(300, 470), drying(500, 640), drying(980, 360))

--@ chunk 205
local nz = noise{seed=5201, period=50, octaves=3}
local zone = poly({{630, 452}, {900, 452}, {900, 545}, {630, 545}}, true):soften(25) * mask(function(x, y) return smoothstep(-0.2, 0.3, nz(x, y)) end)
work(zone, {hand="scumble", tool="filbert 8", pile=mLitG, coverage=0.7, angle=function(x, y) return 0.03 * nz(x * 2, y) end, length={20, 50}, edge="lost", pressure={0.3, 0.5}, load=0.3, dips={4, 0.3, 0.5}})
print(wait(0))

--@ chunk 206
local nz = noise{seed=5202, period=120, octaves=3, stretch={0.0, 3}}
local zone = poly({{600, 448}, {910, 448}, {910, 560}, {600, 560}}, true):soften(20)
local hz = function(x, y) return 0.012 * math.sin(x / 170) + 0.035 * nz(x * 1.5, y * 3) end
work(zone, {hand="body", tool="filbert 12", pile=mMidB, coverage=1.8, angle=hz, length={50, 120}, fill=true, edge="lost", pressure={0.55, 0.9}, dips={4, 0.85, 0.5}})
print(wait(0))

--@ chunk 207
local nz = noise{seed=4901, period=130, octaves=4, stretch={0.0, 3}}
local nz2 = noise{seed=5301, period=40, octaves=3}
local function shadowSide(x, y)
  local c = (160 * (x - 650) + 200 * (y - 460)) / 256 + 25 * nz(x + 300, y)
  return 1 - smoothstep(-30, 30, c)
end
local function zy(x, y) return y + 10 * nz(x, y) end
local keep = footR:grow(2) + bushR:grow(2)
local R = (mask(function(x, y) return smoothstep(440, 446, y) * (1 - smoothstep(585, 600, y)) * smoothstep(470, 500, x) end) - keep)
local hz = function(x, y) return 0.012 * math.sin(x / 170) + 0.035 * nz(x * 1.5, y * 3) end
local function go(m, p) local mm = (m * R) work(mm, {hand="body", tool="filbert 12", pile=p, coverage=1.8, angle=hz, length={50, 120}, fill=true, clip=R, pressure={0.55, 0.9}, dips={4, 0.85, 0.5}}) end
go(mask(function(x, y) local v = zy(x, y) return (1 - shadowSide(x, y)) * (1 - smoothstep(505, 525, v)) end), mLitG)
go(mask(function(x, y) local v = zy(x, y) return (1 - shadowSide(x, y)) * smoothstep(505, 525, v) * (1 - smoothstep(555, 575, v)) end), mMidB)
go(mask(function(x, y) local v = zy(x, y) return (1 - shadowSide(x, y)) * smoothstep(555, 575, v) end), mHalf)
go(mask(function(x, y) return shadowSide(x, y) end), mDeep)
-- far gold strip, clipped
local strip = (mask(function(x, y) return smoothstep(426, 430, y) * (1 - smoothstep(444, 452, y + 4 * nz2(x, y))) * smoothstep(600, 640, x) end) - keep)
work(strip, {hand="body", tool="filbert 8", pile=mLit2, coverage=2.0, angle=0.0, length={30, 70}, fill=true, clip=strip, pressure={0.6, 0.9}})
print(wait(0))

--@ chunk 208
local keep = footR:grow(6) + bushR:grow(4)
local B = (mask(function(x, y) return smoothstep(446, 456, y) * (1 - smoothstep(590, 602, y)) * smoothstep(480, 520, x) end) - keep)
blend(B, {angle=0.0, tool={kind="badger", width=40}, pressure={0.45, 0.65}})
blend(B, {angle=-0.3, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
blend(B, {angle=0.04, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
local S = (mask(function(x, y) return smoothstep(430, 434, y) * (1 - smoothstep(452, 460, y)) * smoothstep(600, 640, x) end) - keep)
blend(S, {angle=0.0, tool={kind="badger", width=20}, pressure={0.3, 0.5}})
print(wait(0))

--@ chunk 209
local nz = noise{seed=5401, period=90, octaves=3}
local function c(x, y) return (160 * (x - 650) + 200 * (y - 460)) / 256 + 12 * nz(x, y) end
local keep = footR:grow(2)
local SH = (mask(function(x, y) return (1 - smoothstep(-8, 8, c(x, y))) * smoothstep(440, 448, y) * (1 - smoothstep(600, 612, y)) * smoothstep(440, 470, x) end) - keep)
local hz = function(x, y) return -0.05 + 0.03 * nz(x * 2, y) end
work(SH, {hand="body", tool="filbert 10", pile=mDeep, coverage=2.2, angle=hz, length={30, 80}, fill=true, clip=SH, pressure={0.6, 0.95}, dips={3, 0.9, 0.5}})
print(wait(0))

--@ chunk 210
local nz = noise{seed=5401, period=90, octaves=3}
local nzb = noise{seed=5402, period=110, octaves=3, stretch={0.0, 3}}
local function c(x, y) return (160 * (x - 650) + 200 * (y - 460)) / 256 + 12 * nz(x, y) end
local keep = footR:grow(4)
-- transition band to the foreground, over the wet bottom edge and onto the dry foreground
local TB = mask(function(x, y) local v = y + 14 * nzb(x, y) return smoothstep(560, 585, v) * (1 - smoothstep(615, 635, v)) * smoothstep(420, 470, x) end)
work(TB, {hand="body", tool="filbert 12", pile=mHalf, coverage=1.5, angle=function(x, y) return 0.03 * nzb(x * 2, y) end, length={50, 120}, fill=true, clip=TB, pressure={0.5, 0.85}, dips={4, 0.8, 0.5}})
local TB2 = mask(function(x, y) local v = y + 14 * nzb(x + 100, y) return smoothstep(590, 610, v) * (1 - smoothstep(630, 650, v)) * smoothstep(420, 470, x) end)
work(TB2, {hand="body", tool="filbert 12", pile=fgD, coverage=1.4, angle=function(x, y) return 0.03 * nzb(x * 2, y) end, length={50, 120}, fill=true, clip=TB2, pressure={0.5, 0.85}, dips={4, 0.8, 0.5}})
-- soften: the shadow's edge, then the band
local E = mask(function(x, y) local d = math.abs(c(x, y)) return (1 - smoothstep(10, 30, d)) * smoothstep(446, 456, y) * (1 - smoothstep(600, 615, y)) end) - keep
blend(E, {angle=-0.67, tool={kind="badger", width=24}, pressure={0.35, 0.55}})
local EB = mask(function(x, y) local v = y + 14 * nzb(x, y) return smoothstep(545, 575, v) * (1 - smoothstep(630, 655, v)) * smoothstep(420, 480, x) end)
blend(EB, {angle=0.0, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
blend(EB, {angle=0.05, tool={kind="badger", width=40}, pressure={0.35, 0.55}})
local EL = mask(function(x, y) return smoothstep(410, 440, x) * (1 - smoothstep(500, 530, x)) * smoothstep(470, 490, y) end)
blend(EL, {angle=0.1, tool={kind="badger", width=30}, pressure={0.35, 0.55}})
print(wait(0))

--@ chunk 211
print(wait(5 * 24 * 60))
print(drying(700, 520), drying(450, 600), drying(300, 500), drying(800, 640), drying(600, 450))

--@ chunk 212
local nz = noise{seed=4901, period=130, octaves=4, stretch={0.0, 3}}
local topLine = {{0, 480}, {150, 482}, {300, 484}, {420, 482}, {500, 474}, {550, 464}, {590, 454}, {625, 447}, {655, 445}, {700, 446}, {850, 446}, {1000, 446}}
meadowC = below(topLine):soften(3)
local function shadowSide(x, y)
  local c = (160 * (x - 650) + 200 * (y - 460)) / 256 + 25 * nz(x + 300, y)
  return 1 - smoothstep(-30, 30, c)
end
local function zy(x, y) return y + 10 * nz(x, y) end
local hz = function(x, y) return 0.012 * math.sin(x / 170) + 0.035 * nz(x * 1.5, y * 3) end
local function go(m, p, cov) work(m * meadowC, {hand="body", tool="filbert 12", pile=p, coverage=cov or 1.8, angle=hz, length={50, 120}, fill=true, clip=meadowC, pressure={0.55, 0.9}, dips={4, 0.85, 0.5}}) end
go(mask(function(x, y) local v = zy(x, y) return (1 - shadowSide(x, y)) * (1 - smoothstep(510, 530, v)) end), mLitG)
go(mask(function(x, y) local v = zy(x, y) return (1 - shadowSide(x, y)) * smoothstep(510, 530, v) * (1 - smoothstep(555, 575, v)) end), mMidB)
go(mask(function(x, y) local v = zy(x, y) return (1 - shadowSide(x, y)) * smoothstep(555, 575, v) * (1 - smoothstep(588, 605, v)) end), mHalf)
go(mask(function(x, y) local v = zy(x, y) return shadowSide(x, y) * (1 - smoothstep(588, 605, v)) end), mDeep)
go(mask(function(x, y) local v = zy(x, y) return smoothstep(588, 605, v) end), fgD, 2.0)
local m = meadowC * below({{0, 486}, {150, 488}, {300, 490}, {420, 488}, {500, 480}, {550, 470}, {590, 460}, {625, 453}, {655, 451}, {700, 452}, {850, 452}, {1000, 452}}):soften(6)
blend(m, {angle=0.0, tool={kind="badger", width=40}, pressure={0.45, 0.65}})
blend(m, {angle=-0.35, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
blend(m, {angle=0.03, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
print(wait(0))

--@ chunk 213
local nz = noise{seed=5501, period=14, octaves=3}
groveFoot = outline{{470, 484}, {468, 432}, {500, 422}, {540, 418}, {580, 417}, {612, 420}, {636, 427}, {650, 436}, {644, 446}, {622, 452}, {592, 458}, {560, 465}, {528, 473}, {498, 480}, char="soft", closed=true, seed=5502, amount=1.2, lobe=8}:mask()
work(groveFoot, {hand="body", tool="filbert 5", pile=underDk, coverage=2.3, angle=function(x, y) return -1.4 + 0.5 * nz(x, y) end, angle_jitter=0.5, length={5, 13}, fill=true, edge="soft", pressure={0.55, 0.9}, dips={5, 0.85, 0.4}})
-- far strip continuation, clipped
local strip = (mask(function(x, y) return smoothstep(430, 434, y) * (1 - smoothstep(448, 454, y)) * smoothstep(640, 660, x) * (1 - smoothstep(740, 760, x)) end) - groveFoot:grow(2))
work(strip, {hand="body", tool="filbert 6", pile=mLit2, coverage=2.0, angle=0.0, length={20, 50}, fill=true, clip=strip, pressure={0.6, 0.9}})
blend(strip:grow(4) - groveFoot:grow(3), {angle=0.0, tool={kind="badger", width=16}, pressure={0.3, 0.5}})
print(wait(0))

--@ chunk 214
local keepTr = ribbon({{699, 420}, {700, 446}, {701, 456}}, 5) + ribbon({{748, 420}, {747, 446}, {746, 456}}, 5) + ribbon({{815, 418}, {815, 442}, {815, 450}}, 5)
local nz = noise{seed=5601, period=60, octaves=3}
local strip = (mask(function(x, y) return smoothstep(429, 433, y) * (1 - smoothstep(446, 456, y + 4 * nz(x, y))) * smoothstep(640, 660, x) end) - groveFoot:grow(2) - keepTr)
work(strip, {hand="body", tool="filbert 6", pile=mLit2, coverage=2.0, angle=0.0, length={30, 70}, fill=true, clip=strip, pressure={0.6, 0.9}, dips={4, 0.85, 0.5}})
blend(strip:grow(5) - groveFoot:grow(3) - keepTr, {angle=0.0, tool={kind="badger", width=20}, pressure={0.3, 0.5}})
print(wait(0))

--@ chunk 215
local rg = brush{kind="round", width=4, point=0.7}
for i, t in ipairs({{{699, 412}, {700, 430}, {700, 446}, {701, 457}}, {{748, 412}, {747, 430}, {746, 446}, {746, 459}}, {{816, 405}, {815, 425}, {815, 440}, {815, 452}}}) do
  rg:reload(trunkS, 0.9)
  rg:stroke(t, {pressure={0.75, 0.9}, ramps={0.15, 0.05}, shake=0.4})
end
-- short soft shadows from their feet toward the viewer
local fb = brush{kind="filbert", width=6}
for i, b in ipairs({{701, 458, -0.35, 55}, {746, 460, -0.18, 55}, {815, 453, -0.02, 40}}) do
  fb:reload(mDeep, 0.35)
  local x0, y0, s, L = b[1], b[2], b[3], b[4]
  fb:stroke({{x0, y0}, {x0 + s * L * 0.5, y0 + L * 0.5}, {x0 + s * L, y0 + L}}, {pressure={0.3, 0.1}, ramps={0.05, 0.7}, shake=0.8, orient="across"})
end
print(wait(0))

--@ chunk 216
local m = (ribbon({{701, 458}, {682, 513}}, 22) + ribbon({{746, 460}, {736, 515}}, 22) + ribbon({{815, 453}, {814, 493}}, 18)):soften(8)
blend(m, {angle=0.0, tool={kind="badger", width=16}, pressure={0.35, 0.5}})
blend(m, {angle=0.3, tool={kind="badger", width=16}, pressure={0.3, 0.45}})
print(wait(0))

--@ chunk 217
print(wait(4 * 24 * 60))
print(drying(700, 440), drying(690, 490), drying(560, 450), drying(300, 500), drying(600, 640))

--@ chunk 218
mMix = pile{{"yellow ochre", 1.35}, {"raw umber", 0.72}, {"Antwerp blue", 0.21}, {"lead white", 0.5}, {"lemon chrome", 0.06}, medium=0.1}
local b = brush("filbert", 5)
b:load(mMix, 0.7)
b:stroke({{900, 500}, {930, 500}}, {pressure={0.7, 0.7}})
print(wait(0))

--@ chunk 219
mMix2 = pile{{"yellow ochre", 1.5}, {"raw umber", 0.62}, {"Antwerp blue", 0.18}, {"lead white", 0.55}, {"lemon chrome", 0.12}, medium=0.1}
local b = brush("filbert", 5)
b:load(mMix2, 0.7)
b:stroke({{900, 510}, {930, 510}}, {pressure={0.7, 0.7}})
print(wait(0))

--@ chunk 220
mMix3 = pile{{"yellow ochre", 1.5}, {"raw umber", 0.5}, {"Antwerp blue", 0.17}, {"lead white", 0.65}, {"lemon chrome", 0.14}, medium=0.1}
local b = brush("filbert", 5)
b:load(mMix2, 0.7)
b:stroke({{900, 470}, {930, 470}}, {pressure={0.7, 0.7}})
b:reload(mMix3, 0.7)
b:stroke({{900, 480}, {930, 480}}, {pressure={0.7, 0.7}})
print(wait(0))

--@ chunk 221
local nz = noise{seed=4901, period=130, octaves=4, stretch={0.0, 3}}
local function shadowSide(x, y)
  local c = (160 * (x - 650) + 200 * (y - 460)) / 256 + 25 * nz(x + 300, y)
  return 1 - smoothstep(-30, 30, c)
end
local function zy(x, y) return y + 10 * nz(x, y) end
local keepTr = ribbon({{700, 425}, {700, 446}, {701, 457}}, 5) + ribbon({{747, 425}, {746, 446}, {746, 459}}, 5) + ribbon({{815, 425}, {815, 442}, {815, 452}}, 5)
LR = (mask(function(x, y) return smoothstep(446, 452, y) * (1 - smoothstep(560, 580, zy(x, y))) * (1 - shadowSide(x, y)) end) - groveFoot:grow(2) - keepTr)
local hz = function(x, y) return 0.012 * math.sin(x / 170) + 0.035 * nz(x * 1.5, y * 3) end
local function go(m, p) local mm = m * LR work(mm, {hand="body", tool="filbert 12", pile=p, coverage=1.8, angle=hz, length={50, 120}, fill=true, clip=LR, pressure={0.55, 0.9}, dips={4, 0.85, 0.5}}) end
go(mask(function(x, y) return 1 - smoothstep(510, 530, zy(x, y)) end), mLitG)
go(mask(function(x, y) return smoothstep(510, 530, zy(x, y)) end), mMidB)
-- the slim trees' shadows as darker strips inside the wet field (no local blending)
local shz = (ribbon({{701, 458}, {690, 480}, {680, 505}}, 9) + ribbon({{746, 460}, {741, 484}, {735, 508}}, 9) + ribbon({{815, 453}, {814, 470}, {813, 488}}, 7)):roughen(3, 10, 5701) * LR
work(shz, {hand="body", tool="filbert 6", pile=mMidB, coverage=1.5, angle=function(x, y) return 1.3 end, length={10, 25}, fill=true, clip=shz, pressure={0.4, 0.7}})
local whole = LR:grow(6) - groveFoot:grow(4) - keepTr
blend(whole, {angle=0.0, tool={kind="badger", width=40}, pressure={0.45, 0.65}})
blend(whole, {angle=-0.3, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
blend(whole, {angle=0.03, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
print(wait(0))

--@ chunk 222
local nz = noise{seed=4901, period=130, octaves=4, stretch={0.0, 3}}
local function cc(x, y) return (160 * (x - 650) + 200 * (y - 460)) / 256 + 25 * nz(x + 300, y) end
local function zy(x, y) return y + 10 * nz(x, y) end
local keepTr = ribbon({{700, 425}, {700, 446}, {701, 457}}, 5) + ribbon({{747, 425}, {746, 446}, {746, 459}}, 5) + ribbon({{815, 425}, {815, 442}, {815, 452}}, 5)
-- grade the lower lit field down, wet into wet
local low = mask(function(x, y) local v = zy(x, y) return smoothstep(530, 555, v) * (1 - smoothstep(585, 600, v)) * smoothstep(-40, 0, cc(x, y)) end)
work(low, {hand="body", tool="filbert 12", pile=mHalf, coverage=1.4, angle=function(x, y) return 0.03 * nz(x * 2, y) end, length={50, 120}, fill=true, clip=low, pressure={0.45, 0.8}, dips={4, 0.7, 0.5}})
-- soften the shadow boundary and the foreground boundary
local edgeS = mask(function(x, y) return (1 - smoothstep(12, 40, math.abs(cc(x, y)))) * smoothstep(455, 470, y) * (1 - smoothstep(600, 620, y)) end) - groveFoot:grow(3)
blend(edgeS, {angle=-0.67, tool={kind="badger", width=30}, pressure={0.45, 0.65}})
blend(edgeS, {angle=0.2, tool={kind="badger", width=30}, pressure={0.4, 0.6}})
local edgeB = mask(function(x, y) local v = zy(x, y) return smoothstep(520, 550, v) * (1 - smoothstep(600, 625, v)) * smoothstep(-60, -20, cc(x, y)) end)
blend(edgeB, {angle=0.0, tool={kind="badger", width=40}, pressure={0.45, 0.65}})
blend(edgeB, {angle=0.05, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
-- far strip's lower edge
local edgeT = mask(function(x, y) return smoothstep(440, 448, y) * (1 - smoothstep(458, 470, y)) * smoothstep(640, 670, x) end) - keepTr
blend(edgeT, {angle=0.0, tool={kind="badger", width=20}, pressure={0.35, 0.55}})
print(wait(0))

--@ chunk 223
local nz = noise{seed=5801, period=110, octaves=3, stretch={0.0, 3}}
local band = mask(function(x, y) local v = y + 12 * nz(x, y) return smoothstep(570, 600, v) * (1 - smoothstep(632, 645, v)) * smoothstep(330, 380, x) end)
work(band, {hand="body", tool="filbert 12", pile=fgD, coverage=1.6, angle=function(x, y) return 0.03 * nz(x * 2, y) end, length={50, 120}, fill=true, clip=band, pressure={0.5, 0.85}, dips={4, 0.8, 0.5}})
local band2 = mask(function(x, y) local v = y + 12 * nz(x + 40, y) return smoothstep(548, 572, v) * (1 - smoothstep(600, 615, v)) * smoothstep(330, 380, x) end)
work(band2, {hand="body", tool="filbert 12", pile=mHalf, coverage=1.3, angle=function(x, y) return 0.03 * nz(x * 2, y) end, length={50, 120}, fill=true, clip=band2, pressure={0.45, 0.8}, dips={4, 0.7, 0.5}})
local B = mask(function(x, y) local v = y + 12 * nz(x, y) return smoothstep(535, 565, v) * (1 - smoothstep(640, 660, v)) * smoothstep(300, 380, x) end)
blend(B, {angle=0.0, tool={kind="badger", width=40}, pressure={0.45, 0.65}})
blend(B, {angle=0.06, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
print(wait(0))

--@ chunk 224
print(wait(5 * 24 * 60))
print(drying(700, 520), drying(450, 600), drying(300, 600), drying(800, 640), drying(600, 450))

--@ chunk 225
local nz = noise{seed=5901, period=150, octaves=4, stretch={0.0, 2.5}}
local fgZ = mask(function(x, y) return smoothstep(565, 610, y + 22 * nz(x, y)) end)
local ang = function(x, y) return -0.04 + 0.1 * nz(x * 2, y) end
work(fgZ, {hand="broad", pile=fgD, coverage=1.7, angle=ang, fill=true, clip=fgZ, pressure={0.5, 0.85}})
local rust = fgZ * mask(function(x, y) return smoothstep(0.05, 0.4, nz(x + 500, y * 1.5)) end)
work(rust, {hand="scumble", tool="filbert 9", pile=fgRust, coverage=0.8, angle=ang, length={15, 40}, clip=fgZ, pressure={0.4, 0.7}, load=0.5})
local deep = fgZ * mask(function(x, y) return smoothstep(0.1, 0.45, nz(x + 900, y * 1.2)) * smoothstep(610, 650, y) end)
work(deep, {hand="scumble", tool="filbert 9", pile=underDk, coverage=0.9, angle=ang, length={15, 40}, clip=fgZ, pressure={0.4, 0.7}, load=0.6})
local fgB = mask(function(x, y) return smoothstep(550, 595, y + 22 * nz(x, y)) end)
blend(fgB, {angle=0.0, tool={kind="badger", width=40}, pressure={0.5, 0.7}})
blend(fgB, {angle=0.5, tool={kind="badger", width=40}, pressure={0.45, 0.65}})
blend(fgB, {angle=-0.4, tool={kind="badger", width=40}, pressure={0.45, 0.65}})
blend(fgB, {angle=0.02, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
print(wait(0))

--@ chunk 226
local fx, fy = 606, 506
local sk = poly({{fx - 2.8, fy - 16}, {fx + 2.6, fy - 16}, {fx + 5, fy}, {fx - 5.2, fy}}, false):soften(0.6)
local sh = poly({{fx - 3.4, fy - 25}, {fx + 3, fy - 25.5}, {fx + 4.2, fy - 16.5}, {fx + 0.5, fy - 13.5}, {fx - 4.6, fy - 15.5}}, true):soften(0.5)
local hd = ellipse(fx - 0.2, fy - 28.2, 2.2, 2.6):soften(0.5)
work(sk, {hand="detail", pile=skirtP, coverage=3, fill=true, tool={kind="round", width=2, point=0.3}})
work(sh, {hand="detail", pile=redP, coverage=3, fill=true, tool={kind="round", width=2, point=0.3}})
work(hd, {hand="detail", pile=headP, coverage=3, fill=true, tool={kind="round", width=1.6, point=0.3}})
local t = brush{kind="round", width=2.2, point=0.5}
t:load(skirtP, 0.5)
t:touch(fx - 0.4, fy - 29.3, {pressure=0.5})
local s = brush{kind="filbert", width=4}
s:load(mDeep, 0.5)
s:stroke({{fx, fy + 0.5}, {fx - 8, fy + 5}, {fx - 18, fy + 9}}, {pressure={0.6, 0.15}, ramps={0.05, 0.6}, shake=0.4})
print(wait(0))

--@ chunk 227
print(drying(320, 555), drying(280, 545), drying(320, 545), drying(400, 540))

--@ chunk 228
print(wait(4 * 24 * 60))
print(drying(320, 555), drying(500, 620), drying(606, 495), drying(800, 640), drying(100, 600))

--@ chunk 229
local nz = noise{seed=6001, period=80, octaves=3}
-- carry the grove's shadow down into the foreground on the left so there is no corner
local W = mask(function(x, y)
  local v = y + 10 * nz(x, y)
  local edge = 300 + (v - 540) * 2.2 + 25 * nz(x + 77, y)
  return smoothstep(528, 545, v) * (1 - smoothstep(585, 605, v)) * (1 - smoothstep(edge - 20, edge + 20, x))
end)
work(W, {hand="body", tool="filbert 12", pile=mDeep, coverage=1.7, angle=function(x, y) return 0.03 * nz(x * 2, y) end, length={40, 100}, fill=true, clip=W, pressure={0.5, 0.85}, dips={4, 0.8, 0.5}})
local WB = W:grow(25):soften(20)
blend(WB, {angle=0.0, tool={kind="badger", width=40}, pressure={0.45, 0.65}})
blend(WB, {angle=0.25, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
print(wait(0))

--@ chunk 230
local nz = noise{seed=6001, period=80, octaves=3}
local E = mask(function(x, y)
  local v = y + 10 * nz(x, y)
  local edge = 300 + (v - 540) * 2.2 + 25 * nz(x + 77, y)
  return smoothstep(525, 540, v) * (1 - smoothstep(600, 615, v)) * (1 - smoothstep(25, 60, math.abs(x - edge)))
end)
blend(E, {angle=0.0, tool={kind="badger", width=30}, pressure={0.45, 0.65}})
blend(E, {angle=0.6, tool={kind="badger", width=30}, pressure={0.4, 0.6}})
local Lo = mask(function(x, y) return smoothstep(585, 600, y + 10 * nz(x, y)) * (1 - smoothstep(615, 630, y + 10 * nz(x, y))) * (1 - smoothstep(420, 480, x)) end)
blend(Lo, {angle=0.0, tool={kind="badger", width=30}, pressure={0.45, 0.65}})
print(wait(0))

--@ chunk 231
print(wait(4 * 24 * 60))
print(drying(320, 555), drying(500, 620), drying(606, 495), drying(200, 480), drying(100, 600))

--@ chunk 232
wSh = pile{{"raw sienna", 1.0}, {"raw umber", 1.0}, {"yellow ochre", 0.5}, {"Antwerp blue", 0.08}, {"red earth", 0.08}, medium=0.3}
goldTouch = pile{{"yellow ochre", 1.5}, {"raw sienna", 0.6}, {"lead white", 0.5}, {"lemon chrome", 0.2}, {"raw umber", 0.2}, medium=0.15}
local b = brush("filbert", 5)
b:load(wSh, 0.5)
b:stroke({{60, 520}, {100, 520}}, {pressure={0.5, 0.5}})
b:reload(goldTouch, 0.5)
b:stroke({{60, 532}, {100, 532}}, {pressure={0.5, 0.5}})
print(wait(0))

--@ chunk 233
local nz = noise{seed=6101, period=90, octaves=3, stretch={0.0, 3}}
local shadowField = mask(function(x, y)
  local c = (160 * (x - 650) + 200 * (y - 460)) / 256 + 25 * nz(x + 300, y)
  return (1 - smoothstep(-40, 0, c)) * smoothstep(470, 490, y) * (1 - smoothstep(590, 610, y))
end) - groveFoot:grow(3)
local patches = shadowField * mask(function(x, y) return smoothstep(0.0, 0.35, nz(x, y)) end)
work(patches, {hand="scumble", tool="filbert 12", pile=wSh, coverage=0.9, angle=function(x, y) return 0.03 * nz(x * 2, y) end, length={30, 70}, clip=shadowField, pressure={0.35, 0.6}, load=0.45})
-- the test strokes
local tst = rect(50, 510, 60, 30):soften(6)
blend(tst, {angle=0.0, tool={kind="badger", width=20}, pressure={0.5, 0.7}})
blend(shadowField, {angle=0.0, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
blend(shadowField, {angle=0.08, tool={kind="badger", width=40}, pressure={0.35, 0.55}})
print(wait(0))

--@ chunk 234
local nz = noise{seed=6101, period=90, octaves=3, stretch={0.0, 3}}
local SF = mask(function(x, y)
  local c = (160 * (x - 650) + 200 * (y - 460)) / 256 + 25 * nz(x + 300, y)
  return (1 - smoothstep(-40, 0, c)) * smoothstep(458, 475, y) * (1 - smoothstep(595, 612, y))
end) - groveFoot:grow(3)
local keepWarm = mask(function(x, y) return smoothstep(0.25, 0.5, nz(x + 40, y)) end)
work(SF - keepWarm, {hand="body", tool="filbert 12", pile=mDeep, coverage=1.6, angle=function(x, y) return 0.03 * nz(x * 2, y) end, length={40, 100}, fill=true, clip=SF, pressure={0.5, 0.85}, dips={4, 0.85, 0.5}})
blend(SF, {angle=0.0, tool={kind="badger", width=40}, pressure={0.45, 0.65}})
blend(SF, {angle=0.1, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
print(wait(0))

--@ chunk 235
print(wait(4 * 24 * 60))
print(drying(606, 495), drying(400, 520), drying(300, 600))

--@ chunk 236
local fx, fy = 607, 506
local sk = poly({{fx - 2.8, fy - 16}, {fx + 2.6, fy - 16}, {fx + 5, fy}, {fx - 5.2, fy}}, false):soften(0.6)
local sh = poly({{fx - 3.4, fy - 25}, {fx + 3, fy - 25.5}, {fx + 4.2, fy - 16.5}, {fx + 0.5, fy - 13.5}, {fx - 4.6, fy - 15.5}}, true):soften(0.5)
local hd = ellipse(fx - 0.2, fy - 28.2, 2.2, 2.6):soften(0.5)
work(sk, {hand="detail", pile=skirtP, coverage=3, fill=true, tool={kind="round", width=2, point=0.3}})
work(sh, {hand="detail", pile=redP, coverage=3.5, fill=true, tool={kind="round", width=2, point=0.3}})
work(hd, {hand="detail", pile=headP, coverage=3, fill=true, tool={kind="round", width=1.6, point=0.3}})
local t = brush{kind="round", width=2.2, point=0.5}
t:load(skirtP, 0.5)
t:touch(fx - 0.4, fy - 29.3, {pressure=0.5})
-- a light catching the shawl's sunward edge
local lt = brush{kind="round", width=1.2, point=0.6}
lt:load(pile{{"orange chrome", 1}, {"lead white", 0.6}, {"red earth", 0.2}, medium=0.1}, 0.5)
lt:stroke({{fx + 2.6, fy - 24}, {fx + 3.6, fy - 18}}, {pressure={0.4, 0.2}})
print(wait(0))

--@ chunk 237
local nz = noise{seed=6201, period=20, octaves=3}
local core = treeAll - openings
local rimR = (core - core:offset(-1):shrink(10)) * mask(function(x, y) return smoothstep(380, 520, x) * smoothstep(140, 200, y) * (1 - smoothstep(370, 400, y)) end)
local rimSel = rimR * mask(function(x, y) return smoothstep(0.1, 0.4, nz(x, y)) end)
work(rimSel, {hand="body", tool="filbert 3", pile=goldTouch, coverage=0.6, angle=function(x, y) return -1.0 + 0.8 * nz(x + 5, y) end, length={3, 7}, clip=core:grow(2), pressure={0.3, 0.55}, load=0.35})
-- slim crowns: right flanks
local rc = (crowns - crowns:shrink(8)) * mask(function(x, y)
  local cx = x < 728 and 690 or (x < 805 and 772 or 821)
  return smoothstep(cx + 4, cx + 18, x)
end) * mask(function(x, y) return smoothstep(0.0, 0.35, nz(x + 200, y)) end)
work(rc, {hand="body", tool="filbert 3", pile=goldTouch, coverage=0.7, angle=function(x, y) return -1.2 + 0.8 * nz(x, y) end, length={3, 7}, clip=crowns:grow(2), pressure={0.3, 0.55}, load=0.35})
print(wait(0))

--@ chunk 238
local nz = noise{seed=6201, period=20, octaves=3}
local core = treeAll - openings
local rimR = (core - core:offset(-1):shrink(10)) * mask(function(x, y) return smoothstep(380, 520, x) * smoothstep(140, 200, y) * (1 - smoothstep(370, 400, y)) end)
local rc = (crowns - crowns:shrink(8)) * mask(function(x, y)
  local cx = x < 728 and 690 or (x < 805 and 772 or 821)
  return smoothstep(cx + 4, cx + 18, x)
end)
local zone = (rimR + rc):grow(4)
local rag = brush{kind="filbert", width=5, stiffness=0.7}
local n = 0
for i = 1, 5000 do
  local x, y = rand(380, 860), rand(140, 420)
  if zone:at(x, y) > 0.5 then
    rag:wipe(1)
    local a = rand(-math.pi, math.pi)
    rag:stroke({{x - 3 * math.cos(a), y - 3 * math.sin(a)}, {x + 3 * math.cos(a), y + 3 * math.sin(a)}}, {pressure={0.8, 0.8}, clip=zone})
    n = n + 1
  end
end
print(n, wait(0))

--@ chunk 239
print(wait(3 * 24 * 60))
print(drying(520, 230), drying(720, 230), drying(830, 360))

--@ chunk 240
local core = treeAll - openings
local rimR = (core:grow(3) - core:shrink(14)) * mask(function(x, y) return smoothstep(370, 400, x) * smoothstep(130, 160, y) * (1 - smoothstep(380, 400, y)) end) * core:grow(3)
local rc = crowns:grow(3) * mask(function(x, y)
  local cx = x < 728 and 690 or (x < 805 and 772 or 821)
  return smoothstep(cx - 2, cx + 10, x)
end)
glzOl = pile{{"raw umber", 1.2}, {"Antwerp blue", 0.4}, {"yellow ochre", 0.3}, {"bone black", 0.25}, medium=0.75}
work(rimR, {hand="glaze", tool="filbert 8", pile=glzOl, coverage=1.3, angle=-1.0, clip=rimR, length={15, 40}})
work(rc, {hand="glaze", tool="filbert 8", pile=glzOl, coverage=1.3, angle=-1.3, clip=rc, length={15, 40}})
print(wait(0))

--@ chunk 241
local core = treeAll - openings
local rimR = (core:grow(3) - core:shrink(18)) * mask(function(x, y) return smoothstep(370, 400, x) * smoothstep(130, 160, y) * (1 - smoothstep(380, 400, y)) end) * core:grow(2)
local rc = crowns:grow(2)
for i, a in ipairs({-1.2, 0.3, -0.4}) do
  blend(rc, {angle=a, tool={kind="badger", width=14}, pressure={0.35, 0.55}})
  blend(rimR, {angle=a, tool={kind="badger", width=18}, pressure={0.35, 0.55}})
end
print(wait(0))

--@ chunk 242
print(wait(2 * 24 * 60))
print(drying(520, 230), drying(720, 230), drying(830, 360))

--@ chunk 243
local nz = noise{seed=6301, period=12, octaves=3}
local core = treeAll - openings
local rimR = (core:grow(1) - core:shrink(14)) * mask(function(x, y) return smoothstep(370, 400, x) * smoothstep(130, 160, y) * (1 - smoothstep(380, 400, y)) end)
local rc = (crowns - crowns:shrink(10)) * mask(function(x, y)
  local cx = x < 728 and 690 or (x < 805 and 772 or 821)
  return smoothstep(cx, cx + 12, x)
end)
local keep = mask(function(x, y) return smoothstep(0.55, 0.6, nz(x, y)) end)
work(rimR - keep, {hand="body", tool="filbert 4", pile=tMass, coverage=1.8, angle=function(x, y) return -1.0 + 0.8 * nz(x + 5, y) end, angle_jitter=0.5, length={4, 9}, fill=true, clip=core:grow(1), pressure={0.45, 0.8}})
work(rc - keep, {hand="body", tool="filbert 3", pile=folO, coverage=1.8, angle=function(x, y) return -1.2 + 0.8 * nz(x, y) end, angle_jitter=0.5, length={3, 8}, fill=true, clip=crowns:grow(1), pressure={0.45, 0.8}})
print(wait(0))

--@ chunk 244
local rc = (crowns:grow(1) - crowns:shrink(12)) * mask(function(x, y)
  local cx = x < 728 and 690 or (x < 805 and 772 or 821)
  return smoothstep(cx - 4, cx + 8, x)
end)
local rag = brush{kind="filbert", width=6, stiffness=0.7}
local n = 0
for pass = 1, 3 do
  for i = 1, 4000 do
    local x, y = rand(680, 860), rand(170, 420)
    if rc:at(x, y) > 0.3 then
      rag:wipe(1)
      local a = rand(-math.pi, math.pi)
      rag:stroke({{x - 4 * math.cos(a), y - 4 * math.sin(a)}, {x + 4 * math.cos(a), y + 4 * math.sin(a)}}, {pressure={0.9, 0.9}, clip=rc})
      n = n + 1
    end
  end
end
print(n, wait(0))

--@ chunk 245
print(wait(3 * 24 * 60))
print(drying(720, 230), drying(800, 300), drying(830, 360))

--@ chunk 246
cMid = pile{{"raw umber", 1.6}, {"Antwerp blue", 0.35}, {"yellow ochre", 1.1}, {"bone black", 0.1}, {"lead white", 0.25}, {"raw sienna", 0.3}, medium=0.15}
cDk = pile{{"raw umber", 2}, {"Antwerp blue", 0.5}, {"yellow ochre", 0.6}, {"bone black", 0.3}, medium=0.15}
local b = brush("filbert", 3)
b:load(cMid, 0.6)
b:stroke({{720, 260}, {735, 262}}, {pressure={0.6, 0.6}})
b:reload(cDk, 0.6)
b:stroke({{720, 270}, {735, 272}}, {pressure={0.6, 0.6}})
b:reload(folM, 0.6)
b:stroke({{720, 280}, {735, 282}}, {pressure={0.6, 0.6}})
print(wait(0))

--@ chunk 247
local nz = noise{seed=6401, period=16, octaves=3}
local function cx(x) return x < 728 and 690 or (x < 805 and 772 or 821) end
local C = crowns:grow(2)
local ang = function(x, y) return math.atan(y - 430, x - cx(x)) + 0.45 * nz(x, y) end
local side = function(x, y) return smoothstep(-6, 14, x - cx(x) + 10 * nz(x + 50, y)) end
local right = C * mask(side)
local left = C - right
work(left, {hand="body", tool="filbert 3", pile=cDk, coverage=1.7, angle=ang, angle_jitter=0.5, length={4, 9}, fill=true, clip=C, pressure={0.45, 0.8}, dips={8, 0.7, 0.4}})
work(right, {hand="body", tool="filbert 3", pile=cMid, coverage=1.7, angle=ang, angle_jitter=0.5, length={4, 9}, fill=true, clip=C, pressure={0.45, 0.8}, dips={8, 0.7, 0.4}})
print(wait(0))

--@ chunk 248
local nz = noise{seed=6501, period=10, octaves=3}
local function cx(x) return x < 728 and 690 or (x < 805 and 772 or 821) end
local C = crowns:grow(2)
local ang = function(x, y) return math.atan(y - 430, x - cx(x)) + 0.45 * nz(x, y) end
-- dark leaf clusters scattered through the whole crown, more on the left
local dots = C * mask(function(x, y) return smoothstep(0.0, 0.25, nz(x, y) - 0.25 * smoothstep(-10, 25, x - cx(x))) end)
work(dots, {hand="body", tool="filbert 3", pile=cDk, coverage=1.6, angle=ang, angle_jitter=0.5, length={4, 9}, clip=C, pressure={0.45, 0.8}, dips={8, 0.7, 0.4}})
for i, a in ipairs({-1.3, 0.3}) do
  blend(C:shrink(3), {angle=a, tool={kind="badger", width=10}, pressure={0.25, 0.4}, coverage=0.6})
end
print(wait(0))

--@ chunk 249
local nz = noise{seed=6601, period=9, octaves=3}
local function cx(x) return x < 728 and 690 or (x < 805 and 772 or 821) end
local C = crowns:grow(2)
local ang = function(x, y) return math.atan(y - 430, x - cx(x)) + 0.5 * nz(x, y) end
local a = C * mask(function(x, y) return smoothstep(-0.1, 0.2, nz(x, y)) end)
local b = C * mask(function(x, y) return smoothstep(-0.1, 0.2, -nz(x, y)) * smoothstep(-5, 15, x - cx(x)) end)
work(a, {hand="body", tool="filbert 2.5", pile=cDk, coverage=1.0, angle=ang, angle_jitter=0.6, length={3, 7}, clip=C, pressure={0.4, 0.75}, dips={10, 0.6, 0.4}})
work(b, {hand="body", tool="filbert 2.5", pile=cMid, coverage=0.7, angle=ang, angle_jitter=0.6, length={3, 7}, clip=C, pressure={0.35, 0.6}, dips={10, 0.5, 0.4}})
print(wait(0))

--@ chunk 250
print(drying(200, 520), drying(300, 600), drying(150, 250))
local b = brush{kind="filbert", width=12}
b:load(mHalf, 0.2)
b:wipe(0.3)
b:stroke({{40, 540}, {90, 538}, {140, 541}}, {pressure={0.35, 0.25}, shake=0.6})
print(wait(0))

--@ chunk 251
blend(rect(30, 525, 130, 30):soften(8), {angle=0.0, tool={kind="badger", width=20}, pressure={0.5, 0.7}})
sh2 = pile{{"raw umber", 1.6}, {"Antwerp blue", 0.3}, {"yellow ochre", 1.0}, {"bone black", 0.12}, {"raw sienna", 0.35}, {"lead white", 0.12}, medium=0.15}
local b = brush{kind="filbert", width=12}
b:load(sh2, 0.2)
b:wipe(0.3)
b:stroke({{40, 580}, {90, 578}, {140, 581}}, {pressure={0.35, 0.25}, shake=0.6})
print(wait(0))

--@ chunk 252
blend(rect(30, 565, 130, 30):soften(8), {angle=0.0, tool={kind="badger", width=20}, pressure={0.5, 0.7}})
local b = brush{kind="filbert", width=14, stiffness=0.3}
b:load(fgGlz, 0.5)
b:stroke({{200, 560}, {260, 556}, {320, 560}}, {pressure={0.4, 0.4}})
print(wait(0))

--@ chunk 253
local z = ellipse(180, 560, 190, 45):soften(30)
blend(z, {angle=0.0, tool={kind="badger", width=40}, pressure={0.5, 0.7}})
blend(z, {angle=0.2, tool={kind="badger", width=40}, pressure={0.45, 0.65}})
blend(z, {angle=-0.15, tool={kind="badger", width=40}, pressure={0.45, 0.65}})
print(wait(0))

--@ chunk 254
local W = winA:shrink(1)
-- lower part of the window: near grass in shade
work(W * below({{220, 414}, {320, 414}}), {hand="detail", pile=underDk, coverage=2.2, fill=true})
-- a thinner, darker distant wood-line
work(W * below({{220, 401}, {320, 401}}) * above({{220, 406}, {320, 406}}), {hand="detail", pile=dT1, coverage=2.2, fill=true})
-- far field, narrower
work(W * below({{220, 406}, {320, 406}}) * above({{220, 414}, {320, 414}}), {hand="detail", pile=mLitG, coverage=2.2, fill=true})
blend(W, {angle=0.0, tool={kind="badger", width=8}, pressure={0.3, 0.45}})
print(wait(0))

--@ chunk 255
print(wait(3 * 24 * 60))
print(drying(260, 405), drying(720, 250), drying(200, 560))

--@ chunk 256
-- re-shape the window smaller, then refill its bands, then trunks
local nz = noise{seed=6701, period=10, octaves=2}
local winB = outline{{240, 418}, {238, 406}, {244, 396}, {256, 391}, {272, 390}, {290, 392}, {300, 399}, {304, 410}, {302, 418}, char="soft", closed=true, seed=6702, amount=0.7, lobe=6}:mask()
local ring = (winA:grow(5) - winB)
work(ring, {hand="body", tool="filbert 3", pile=underDk, coverage=2.4, angle=function(x, y) return -1.2 + 0.6 * nz(x, y) end, angle_jitter=0.5, length={3, 8}, fill=true, clip=ring, pressure={0.6, 0.95}})
work(winB * above({{230, 402}, {310, 402}}), {hand="detail", pile=s4, coverage=2.4, fill=true})
work(winB * below({{230, 402}, {310, 402}}) * above({{230, 406}, {310, 406}}), {hand="detail", pile=dT1, coverage=2.4, fill=true})
work(winB * below({{230, 406}, {310, 406}}) * above({{230, 412}, {310, 412}}), {hand="detail", pile=mLitG, coverage=2.4, fill=true})
work(winB * below({{230, 412}, {310, 412}}), {hand="detail", pile=underDk, coverage=2.4, fill=true})
print(wait(0))

--@ chunk 257
local nz = noise{seed=6801, period=7, octaves=2}
-- foliage hanging into the window top
local top = rect(236, 386, 72, 14) * mask(function(x, y) return smoothstep(-0.15, 0.25, nz(x, y)) * (1 - smoothstep(392, 400, y + 6 * nz(x + 30, y))) end)
work(top, {hand="body", tool="filbert 2.5", pile=underDk, coverage=1.8, angle=function(x, y) return 1.4 + 0.6 * nz(x, y) end, angle_jitter=0.5, length={3, 7}, fill=true, clip=rect(230, 380, 85, 22), pressure={0.5, 0.85}})
-- trunks across the window
local fb = brush{kind="filbert", width=5}
fb:reload(trD, 0.9)
fb:stroke({{262, 422}, {261, 410}, {259, 398}, {257, 386}, {254, 372}}, {pressure={0.95, 0.6}, ramps={0.03, 0.3}, shake=0.4})
local fb2 = brush{kind="filbert", width=3}
fb2:reload(trD, 0.9)
fb2:stroke({{287, 420}, {288, 408}, {290, 396}, {292, 384}}, {pressure={0.9, 0.55}, ramps={0.03, 0.3}, shake=0.4})
-- specks of bare ground in the canopy: dab them dark
local specks = rect(200, 340, 140, 50)
stipple(specks, {pile=underDk, width=4, coverage=1.2, pressure={0.5, 0.8}})
print(wait(0))

--@ chunk 258
print(drying(100, 640), drying(500, 650), drying(850, 640))
local b = brush{kind="filbert", width=10, stiffness=0.8}
b:load(fgRust, 0.15)
b:stroke({{60, 645}, {110, 640}, {150, 646}}, {pressure={0.5, 0.4}, shake=0.8})
print(wait(0))

--@ chunk 259
local nz = noise{seed=6901, period=100, octaves=3}
local fz = mask(function(x, y) return smoothstep(600, 625, y + 15 * nz(x, y)) end)
local ang = function(x, y) return -0.08 + 0.18 * nz(x * 2, y * 2) end
work(fz * mask(function(x, y) return smoothstep(0.0, 0.3, nz(x + 300, y)) end), {hand="body", tool={kind="filbert", width=10, stiffness=0.8}, pile=fgRust, coverage=0.6, angle=ang, length={25, 60}, pressure={0.4, 0.6}, load=0.18, dips={3, 0.18, 0.2}, clip=fz})
work(fz * mask(function(x, y) return smoothstep(0.0, 0.3, nz(x + 800, y)) end), {hand="body", tool={kind="filbert", width=9, stiffness=0.8}, pile=underDk, coverage=0.6, angle=ang, length={20, 50}, pressure={0.4, 0.6}, load=0.2, dips={3, 0.2, 0.2}, clip=fz})
print(wait(0))

--@ chunk 260
local nz = noise{seed=6901, period=100, octaves=3}
local fz = mask(function(x, y) return smoothstep(590, 615, y + 15 * nz(x, y)) end)
for i, a in ipairs({0.0, 0.5, -0.4, 0.05, 0.3}) do
  blend(fz, {angle=a, tool={kind="badger", width=40}, pressure={0.55, 0.75}})
end
print(wait(0))

--@ chunk 261
print(wait(4 * 24 * 60))
print(drying(260, 400), drying(300, 350), drying(500, 640), drying(720, 250))

--@ chunk 262
glzDeep = pile{{"raw umber", 1.2}, {"Antwerp blue", 0.35}, {"bone black", 0.35}, {"yellow ochre", 0.2}, medium=0.75}
local nz = noise{seed=7001, period=40, octaves=3}
local core = treeAll - openings
local band = mask(function(x, y) return smoothstep(310, 345, y + 12 * nz(x, y)) * (1 - smoothstep(440, 470, y)) * (1 - smoothstep(600, 650, x)) end) - winA:shrink(2)
work(band, {hand="glaze", tool="filbert 20", pile=glzDeep, coverage=1.2, angle=0.0, clip=band, length={60, 150}})
blend(band, {angle=0.0, tool={kind="badger", width=40}, pressure={0.35, 0.55}})
print(wait(0))

--@ chunk 263
local nz = noise{seed=7001, period=40, octaves=3}
local band = mask(function(x, y) return smoothstep(300, 335, y + 12 * nz(x, y)) * (1 - smoothstep(450, 480, y)) * (1 - smoothstep(610, 660, x)) end) - winA:shrink(2)
for i, a in ipairs({0.5, -0.5, 0.0, 1.2}) do
  blend(band, {angle=a, tool={kind="badger", width=40}, pressure={0.4, 0.6}})
end
print(wait(0))

--@ chunk 264
local z = (rect(560, 290, 110, 115) * above({{540, 392}, {700, 390}})) - treeKeep2:grow(1) - crowns:grow(1)
local rag = brush{kind="filbert", width=8, stiffness=0.6}
local n = 0
for pass = 1, 3 do
  for i = 1, 1500 do
    local x, y = rand(560, 670), rand(290, 405)
    if z:at(x, y) > 0.5 then
      rag:wipe(1)
      local a = rand(-0.3, 0.3)
      rag:stroke({{x - 6 * math.cos(a), y - 6 * math.sin(a)}, {x + 6 * math.cos(a), y + 6 * math.sin(a)}}, {pressure={0.8, 0.8}, clip=z})
      n = n + 1
    end
  end
end
print(n, wait(0))

--@ chunk 265
local b = brush("filbert", 3)
b:load(s3, 0.6)
b:stroke({{120, 300}, {130, 300}}, {pressure={0.6, 0.6}})
print(wait(0))

--@ chunk 266
local rag = brush{kind="filbert", width=5}
for i = 1, 5 do rag:wipe(1); rag:stroke({{116, 300}, {134, 300}}, {pressure={0.9, 0.9}}) end
for i = 1, 3 do rag:wipe(1); rag:stroke({{125, 296}, {125, 304}}, {pressure={0.9, 0.9}}) end
print(wait(0))

--@ chunk 267
local m = ellipse(125, 300, 14, 9)
work(m, {hand="body", tool="filbert 4", pile=tMass, coverage=2.5, angle=-0.8, length={4, 9}, fill=true, clip=m:grow(2), pressure={0.6, 0.9}})
print(wait(0))

--@ chunk 268
blend(ellipse(125, 300, 22, 15):soften(6), {angle=-0.6, tool={kind="badger", width=14}, pressure={0.4, 0.6}})
print(wait(0))

--@ chunk 269
print(wait(3 * 24 * 60))
print(drying(270, 400), drying(800, 520), drying(125, 300), drying(620, 350))

--@ chunk 270
winGlz = pile{{"raw umber", 1.0}, {"raw sienna", 0.6}, {"Antwerp blue", 0.1}, medium=0.85}
local w = winA:grow(3)
work(w, {hand="glaze", tool="filbert 10", pile=winGlz, coverage=1.0, angle=0.0, clip=w, length={20, 50}})
blend(w, {angle=0.0, tool={kind="badger", width=12}, pressure={0.3, 0.45}})
print(wait(0))

--@ chunk 271
local ring = (winA:grow(12) - winA:shrink(1)):soften(4)
local rag = brush{kind="filbert", width=6, stiffness=0.6}
local n = 0
for pass = 1, 3 do
  for i = 1, 2000 do
    local x, y = rand(220, 330), rand(370, 440)
    if ring:at(x, y) > 0.4 then
      rag:wipe(1)
      local a = rand(-math.pi, math.pi)
      rag:stroke({{x - 4 * math.cos(a), y - 4 * math.sin(a)}, {x + 4 * math.cos(a), y + 4 * math.sin(a)}}, {pressure={0.85, 0.85}, clip=ring})
      n = n + 1
    end
  end
end
print(n, wait(0))

--@ chunk 272
print(drying(690, 260), drying(770, 260), drying(700, 200), drying(820, 360))
slimDk = pile{{"raw umber", 1.8}, {"Antwerp blue", 0.4}, {"yellow ochre", 0.9}, {"bone black", 0.15}, {"raw sienna", 0.3}, {"lead white", 0.15}, medium=0.15}
slimMd = pile{{"raw umber", 1.6}, {"Antwerp blue", 0.3}, {"yellow ochre", 1.1}, {"bone black", 0.1}, {"raw sienna", 0.4}, {"lead white", 0.3}, medium=0.15}
slimEdge = pile{{"raw umber", 1.6}, {"Antwerp blue", 0.35}, {"yellow ochre", 0.8}, {"bone black", 0.12}, {"raw sienna", 0.3}, medium=0.55}
local b = brush("filbert", 3)
b:load(slimDk, 0.6)
b:stroke({{680, 250}, {700, 252}}, {pressure={0.6, 0.6}})
b:reload(slimMd, 0.6)
b:stroke({{680, 262}, {700, 264}}, {pressure={0.6, 0.6}})
b:reload(slimEdge, 0.3)
b:stroke({{850, 300}, {862, 298}}, {pressure={0.35, 0.3}})
print(wait(0))

--@ chunk 273
slimA = pile{{"raw umber", 1.8}, {"Antwerp blue", 0.4}, {"yellow ochre", 1.0}, {"bone black", 0.15}, {"raw sienna", 0.4}, medium=0.15}
slimB = pile{{"raw umber", 2}, {"Antwerp blue", 0.45}, {"yellow ochre", 0.7}, {"bone black", 0.25}, {"raw sienna", 0.3}, medium=0.15}
local b = brush("filbert", 3)
b:load(slimA, 0.6)
b:stroke({{680, 275}, {700, 277}}, {pressure={0.6, 0.6}})
b:reload(slimB, 0.6)
b:stroke({{680, 288}, {700, 290}}, {pressure={0.6, 0.6}})
print(wait(0))

--@ chunk 274
local function scan(m, y)
  local s = {}
  local inside = false
  for x = 600, 880 do
    local v = m:at(x, y) > 0.5
    if v ~= inside then s[#s+1] = x; inside = v end
  end
  return table.concat(s, ",")
end
for _, y in ipairs({180, 190, 200, 220, 250, 280, 300, 320, 340, 350, 360, 380, 400, 410}) do
  print(y, scan(crowns, y))
end

--@ chunk 275
c1o = outline{{700, 356}, {684, 352}, {668, 357}, {652, 347}, {644, 332}, {637, 316}, {630, 300}, {634, 284}, {640, 268}, {638, 252}, {645, 234}, {652, 218}, {662, 202}, {670, 193}, {682, 185}, {694, 176}, {706, 178}, {716, 188}, {724, 202}, {729, 220}, {731, 240}, {733, 262}, {731, 284}, {730, 306}, {732, 326}, {726, 344}, {714, 354}, char="soft", closed=true, seed=7101, amount=0.8, lobe=9}
c2o = outline{{752, 362}, {738, 356}, {731, 340}, {729, 318}, {732, 296}, {733, 272}, {735, 248}, {738, 226}, {744, 210}, {752, 198}, {764, 190}, {778, 192}, {792, 200}, {802, 214}, {810, 232}, {818, 252}, {824, 272}, {818, 290}, {808, 304}, {804, 322}, {800, 340}, {790, 354}, {776, 361}, {764, 357}, char="soft", closed=true, seed=7102, amount=0.8, lobe=9}
c3o = outline{{812, 410}, {800, 402}, {794, 388}, {793, 370}, {796, 352}, {803, 338}, {814, 328}, {828, 326}, {840, 334}, {848, 348}, {852, 364}, {849, 382}, {842, 398}, {828, 408}, char="soft", closed=true, seed=7103, amount=0.8, lobe=8}
C1, C2, C3 = c1o:mask(), c2o:mask(), c3o:mask()
CN = C1 + C2 + C3
print(C1:area(), C2:area(), C3:area(), crowns:area())
local function scan(m, y)
  local s = {}
  local inside = false
  for x = 600, 880 do
    local v = m:at(x, y) > 0.5
    if v ~= inside then s[#s+1] = x; inside = v end
  end
  return table.concat(s, ",")
end
for _, y in ipairs({180, 200, 220, 250, 280, 300, 320, 340, 350, 360, 380, 400}) do
  print(y, scan(CN, y))
end

--@ chunk 276
slit = ribbon({{731, 205}, {731, 235}, {730, 262}, {730, 290}}, {3.5, 4, 3, 1.5}):roughen(1.2, 6, 7111)
CNp = CN - slit
local nz = noise{seed=7121, period=26, octaves=3}
local function cx(x) return x < 731 and 688 or (x < 790 and 770 or 822) end
local function by(x) return x < 731 and 440 or (x < 790 and 450 or 440) end
local ang = function(x, y) return math.atan(y - by(x), x - cx(x)) + 0.5 * nz(x + 31, y) end
-- warmer olive patches, more of them toward the sun side (right)
local warmSide = mask(function(x, y) return smoothstep(-0.05, 0.25, nz(x, y) + 0.35 * smoothstep(-15, 25, x - cx(x)) - 0.15) end)
local A = CNp * warmSide
local B = CNp - A
work(B, {hand="body", tool="filbert 5", pile=slimB, coverage=2.0, angle=ang, angle_jitter=0.45, length={6, 14}, fill=true, clip=CNp, pressure={0.55, 0.9}, dips={6, 0.85, 0.4}})
work(A, {hand="body", tool="filbert 5", pile=slimA, coverage=2.0, angle=ang, angle_jitter=0.45, length={6, 14}, fill=true, clip=CNp, pressure={0.55, 0.9}, dips={6, 0.85, 0.4}})
print(wait(0))

--@ chunk 277
local nz = noise{seed=7131, period=22, octaves=3}
local function cx(x) return x < 731 and 688 or (x < 790 and 770 or 822) end
local function by(x) return x < 731 and 440 or (x < 790 and 450 or 440) end
local ang = function(x, y) return math.atan(y - by(x), x - cx(x)) + 0.5 * nz(x + 31, y) end
-- the shade inside the crowns: left/lower parts and scattered clusters
local shade = CNp * mask(function(x, y)
  local s = smoothstep(10, -20, x - cx(x)) * 0.6 + smoothstep(0.0, 0.3, nz(x, y)) * 0.7
  return smoothstep(0.35, 0.6, s)
end)
work(shade, {hand="body", tool="filbert 4", pile=cDk, coverage=1.6, angle=ang, angle_jitter=0.5, length={5, 11}, clip=CNp, pressure={0.5, 0.85}, dips={6, 0.75, 0.4}})
print(wait(0))

--@ chunk 278
local nz = noise{seed=7141, period=9, octaves=3}
local nz2 = noise{seed=7142, period=30, octaves=2}
local function ctr(x, y)
  if x < 731 then return 684, 268 elseif x < 792 or y < 320 then return 776, 278 else return 822, 368 end
end
local ang = function(x, y) local a, b = ctr(x, y) return math.atan(y - b, x - a) + 0.6 * nz(x + 17, y) end
local ring = (CNp:grow(9):roughen(4, 9, 7143) - CNp:shrink(3)) - slit:grow(1)
local fringe = ring * mask(function(x, y) return smoothstep(-0.15, 0.25, nz(x, y) + 0.2 * nz2(x, y)) end)
slimEdgeB = pile{{"raw umber", 2}, {"Antwerp blue", 0.45}, {"yellow ochre", 0.7}, {"bone black", 0.2}, {"raw sienna", 0.3}, medium=0.4}
work(fringe, {hand="body", tool="filbert 3", pile=slimEdgeB, coverage=1.3, angle=ang, angle_jitter=0.5, length={4, 9}, clip=CNp:grow(10), pressure={0.35, 0.75}, dips={8, 0.6, 0.4}})
print(wait(0))

--@ chunk 279
local band = (CNp:grow(12) - CNp:shrink(6)):soften(4) - slit
blend(band, {angle=-1.2, tool={kind="badger", width=12}, pressure={0.2, 0.32}, coverage=0.7})
blend(band, {angle=0.4, tool={kind="badger", width=12}, pressure={0.18, 0.3}, coverage=0.7})
print(wait(0))

--@ chunk 280
local rag = brush{kind="filbert", width=5, stiffness=0.6}
for i = 1, 5 do rag:wipe(1); rag:stroke({{846, 300}, {866, 298}}, {pressure={0.9, 0.9}}) end
for i = 1, 3 do rag:wipe(1); rag:stroke({{856, 294}, {856, 304}}, {pressure={0.9, 0.9}}) end
print(wait(0))

--@ chunk 281
local z = rect(842, 291, 26, 16)
local rag = brush{kind="filbert", width=4, stiffness=0.6}
local n = 0
for pass = 1, 3 do
  for i = 1, 400 do
    local x, y = rand(842, 868), rand(291, 307)
    rag:wipe(1)
    local a = rand(-math.pi, math.pi)
    rag:stroke({{x - 3 * math.cos(a), y - 3 * math.sin(a)}, {x + 3 * math.cos(a), y + 3 * math.sin(a)}}, {pressure={0.85, 0.85}, clip=z})
    n = n + 1
  end
end
print(n, wait(0))

--@ chunk 282
print(drying(690, 260), drying(820, 250), drying(640, 300), drying(830, 380))
local b = brush("filbert", 4)
b:load(s3, 0.5)
b:stroke({{836, 252}, {846, 251}}, {pressure={0.5, 0.5}})
b:reload(s4, 0.5)
b:stroke({{850, 330}, {860, 329}}, {pressure={0.5, 0.5}})
b:reload(s2, 0.5)
b:stroke({{820, 190}, {830, 189}}, {pressure={0.5, 0.5}})
print(wait(0))

--@ chunk 283
local rag = brush{kind="filbert", width=4, stiffness=0.6}
local n = 0
for _, t in ipairs({{816, 186, 834, 194}, {834, 248, 850, 256}, {846, 326, 864, 334}}) do
  local z = rect(t[1], t[2], t[3] - t[1], t[4] - t[2])
  for i = 1, 250 do
    local x, y = rand(t[1], t[3]), rand(t[2], t[4])
    rag:wipe(1)
    local a = rand(-math.pi, math.pi)
    rag:stroke({{x - 3 * math.cos(a), y - 3 * math.sin(a)}, {x + 3 * math.cos(a), y + 3 * math.sin(a)}}, {pressure={0.85, 0.85}, clip=z})
    n = n + 1
  end
end
print(n, wait(0))

--@ chunk 284
local nz = noise{seed=7151, period=8, octaves=2}
-- repair the lifted notch at crown 3's right edge with leaves
local notch = ellipse(851, 330, 9, 7):roughen(2, 5, 7152)
work(notch, {hand="body", tool="filbert 3", pile=slimEdgeB, coverage=1.8, angle=function(x, y) return math.atan(y - 368, x - 822) + 0.6 * nz(x, y) end, angle_jitter=0.5, length={4, 8}, clip=notch:grow(2), pressure={0.4, 0.8}, dips={6, 0.6, 0.4}})
-- sky pile slightly toned to the aged sky
skC = pile{{"lead white", 10}, {"yellow ochre", 0.75}, {"lemon chrome", 0.35}, {"red earth", 0.1}, {"cobalt blue", 0.06}, {"raw umber", 0.08}, medium=0.1}
skW = pile{{"lead white", 10}, {"lemon chrome", 1.1}, {"cadmium yellow", 0.2}, {"orange chrome", 0.1}, {"yellow ochre", 0.3}, {"raw umber", 0.06}, medium=0.1}
-- the gap of sky between the middle tree and the small one
local gap = ribbon({{836, 306}, {824, 316}, {813, 326}, {804, 337}, {797, 349}, {792, 360}}, {7, 6, 5, 4, 3, 1.5}):roughen(1.5, 6, 7153)
local fb = brush{kind="filbert", width=3.5}
for i = 1, 3 do
  fb:reload(skC, 0.8)
  fb:stroke({{838, 305}, {825, 315}, {814, 325}, {805, 336}}, {pressure={0.7, 0.5}, clip=gap})
  fb:reload(skC, 0.8)
  fb:stroke({{812, 328}, {803, 338}, {797, 349}, {792, 360}}, {pressure={0.6, 0.3}, clip=gap})
end
print(wait(0))

--@ chunk 285
local nz = noise{seed=7161, period=6, octaves=2}
local gapZ = ribbon({{840, 304}, {826, 315}, {814, 326}, {805, 337}, {797, 349}, {791, 362}}, {12, 11, 10, 9, 7, 5})
-- leaves breaking across the gap from both trees
local br = gapZ * mask(function(x, y) return smoothstep(-0.05, 0.3, nz(x, y)) end)
work(br, {hand="body", tool="filbert 2.5", pile=slimB, coverage=1.6, angle=function(x, y) return -0.7 + 0.8 * nz(x + 9, y) end, angle_jitter=0.6, length={3, 7}, clip=gapZ, pressure={0.45, 0.8}, dips={8, 0.6, 0.4}})
print(wait(0))

--@ chunk 286
local nz = noise{seed=7171, period=7, octaves=2}
local gapZ = ribbon({{842, 303}, {826, 315}, {814, 326}, {805, 337}, {797, 349}, {791, 362}}, {11, 10, 9, 8, 7, 5})
work(gapZ, {hand="body", tool="filbert 3", pile=slimB, coverage=2.4, angle=function(x, y) return -0.7 + 0.8 * nz(x + 9, y) end, angle_jitter=0.6, length={4, 8}, fill=true, clip=gapZ:grow(1), pressure={0.6, 0.95}, dips={6, 0.85, 0.4}})
print(wait(0))

--@ chunk 287
-- lift the club-shaped overrun at the top of the gap, back to the dry sky beneath
local club = (ellipse(840, 302, 10, 8) - C2:shrink(1) - C3:shrink(1))
local rag = brush{kind="filbert", width=4, stiffness=0.6}
local n = 0
for pass = 1, 3 do
  for i = 1, 300 do
    local x, y = rand(828, 852), rand(292, 312)
    if club:at(x, y) > 0.4 then
      rag:wipe(1)
      local a = rand(-math.pi, math.pi)
      rag:stroke({{x - 3 * math.cos(a), y - 3 * math.sin(a)}, {x + 3 * math.cos(a), y + 3 * math.sin(a)}}, {pressure={0.85, 0.85}, clip=club})
      n = n + 1
    end
  end
end
-- lift the pale streak inside, then restate it with dark leaves, a fresh load for each few
local streak = ribbon({{836, 306}, {824, 316}, {814, 326}, {805, 337}, {797, 349}, {791, 362}}, {7, 7, 6, 6, 5, 4}) * (C2 + C3):shrink(1)
for pass = 1, 2 do
  for i = 1, 400 do
    local x, y = rand(786, 842), rand(300, 366)
    if streak:at(x, y) > 0.4 then
      rag:wipe(1)
      local a = rand(-math.pi, math.pi)
      rag:stroke({{x - 3 * math.cos(a), y - 3 * math.sin(a)}, {x + 3 * math.cos(a), y + 3 * math.sin(a)}}, {pressure={0.85, 0.85}, clip=streak})
      n = n + 1
    end
  end
end
print(n, wait(0))

--@ chunk 288
local nz = noise{seed=7181, period=7, octaves=2}
local streak = ribbon({{838, 304}, {824, 316}, {814, 326}, {805, 337}, {797, 349}, {791, 362}}, {9, 9, 8, 8, 7, 5}) * (C2 + C3):shrink(1)
local fb = brush{kind="filbert", width=3}
local n = 0
for i = 1, 260 do
  local x, y = rand(784, 846), rand(298, 368)
  if streak:at(x, y) > 0.3 then
    fb:reload(slimB, 0.7)
    local a = -0.8 + 0.9 * nz(x, y)
    fb:stroke({{x - 3 * math.cos(a), y - 3 * math.sin(a)}, {x + 3 * math.cos(a), y + 3 * math.sin(a)}}, {pressure={0.75, 0.6}, clip=streak:grow(2)})
    n = n + 1
  end
end
print(n, wait(0))

--@ chunk 289
local nz = noise{seed=7191, period=7, octaves=2}
local streak = ribbon({{838, 305}, {824, 316}, {814, 326}, {805, 337}, {797, 349}, {791, 362}}, {8, 8, 8, 7, 6, 5})
local fb = brush{kind="filbert", width=3}
local n = 0
for i = 1, 600 do
  local x, y = rand(784, 846), rand(298, 368)
  if streak:at(x, y) > 0.5 then
    if n % 3 == 0 then fb:reload(slimB, 0.7) end
    local a = -0.8 + 0.9 * nz(x, y)
    fb:stroke({{x - 2.5 * math.cos(a), y - 2.5 * math.sin(a)}, {x + 2.5 * math.cos(a), y + 2.5 * math.sin(a)}}, {pressure={0.75, 0.6}, clip=streak})
    n = n + 1
  end
end
print(n, wait(0))

--@ chunk 290
print(drying(40, 250), drying(100, 150), drying(20, 330))
local b = brush("filbert", 3)
b:load(s2, 0.5)
b:stroke({{30, 150}, {38, 150}}, {pressure={0.5, 0.5}})
b:reload(s3, 0.5)
b:stroke({{12, 250}, {20, 250}}, {pressure={0.5, 0.5}})
b:reload(s1, 0.5)
b:stroke({{30, 120}, {38, 120}}, {pressure={0.5, 0.5}})
print(wait(0))

--@ chunk 291
local rag = brush{kind="filbert", width=4, stiffness=0.6}
for _, s in ipairs({{26, 150, 42, 150}, {8, 250, 24, 250}, {26, 120, 42, 120}}) do
  for i = 1, 4 do rag:wipe(1); rag:stroke({{s[1], s[2]}, {s[3], s[4]}}, {pressure={0.9, 0.9}}) end
end
local core = treeAll - openings
local function scan(m, y)
  local s = {}
  local inside = false
  for x = 0, 400 do
    local v = m:at(x, y) > 0.5
    if v ~= inside then s[#s+1] = x; inside = v end
  end
  return table.concat(s, ",")
end
for _, y in ipairs({10, 30, 60, 100, 150, 200, 250, 300, 340, 370}) do
  print(y, scan(core, y), "|", scan(core:grow(15), y))
end
print(wait(0))

--@ chunk 292
local core = treeAll - openings
local nz = noise{seed=7201, period=9, octaves=2}
local test = (core:grow(18) - core:shrink(4)) * rect(0, 230, 80, 100)
local broken = test * mask(function(x, y) return smoothstep(-0.1, 0.3, nz(x, y)) end)
work(broken, {hand="body", tool="filbert 4", pile=s3, coverage=1.0, angle=function(x, y) return 0.8 * nz(x + 7, y) end, angle_jitter=0.8, length={4, 9}, clip=test, pressure={0.45, 0.75}, dips={6, 0.6, 0.4}})
print(wait(0))

--@ chunk 293
local z = rect(0, 226, 84, 108)
local rag = brush{kind="filbert", width=5, stiffness=0.6}
local n = 0
for pass = 1, 4 do
  for i = 1, 700 do
    local x, y = rand(0, 84), rand(226, 334)
    rag:wipe(1)
    local a = rand(-math.pi, math.pi)
    rag:stroke({{x - 3 * math.cos(a), y - 3 * math.sin(a)}, {x + 3 * math.cos(a), y + 3 * math.sin(a)}}, {pressure={0.85, 0.85}, clip=z})
    n = n + 1
  end
end
print(n, wait(0))

--@ chunk 294
tHalf = pile{{"raw umber", 1.5}, {"yellow ochre", 0.9}, {"Antwerp blue", 0.3}, {"lead white", 0.35}, {"bone black", 0.1}, {"raw sienna", 0.2}, medium=0.25}
local b = brush("filbert", 3)
b:load(tHalf, 0.5)
b:stroke({{6, 180}, {14, 178}}, {pressure={0.5, 0.5}})
b:reload(slimEdgeB, 0.4)
b:stroke({{6, 192}, {14, 190}}, {pressure={0.4, 0.4}})
print(wait(0))

--@ chunk 295
local rag = brush{kind="filbert", width=4, stiffness=0.6}
for _, s in ipairs({{2, 179, 18, 178}, {2, 191, 18, 190}}) do
  for i = 1, 4 do rag:wipe(1); rag:stroke({{s[1], s[2]}, {s[3], s[4]}}, {pressure={0.9, 0.9}}) end
end
local core = treeAll - openings
local nz = noise{seed=7211, period=8, octaves=2}
local zone = (core:grow(17) - core:shrink(4)) * rect(0, 226, 84, 108)
local ang = function(x, y) return math.atan(y - 330, x - 200) + 0.6 * nz(x + 3, y) end
-- inner part: denser, darker
local inner = zone * (core:grow(6))
work(inner, {hand="body", tool="filbert 3", pile=slimEdgeB, coverage=1.4, angle=ang, angle_jitter=0.5, length={4, 8}, clip=zone, pressure={0.45, 0.8}, dips={8, 0.6, 0.4}})
-- outer: halftone leaves, broken
local outer = (zone - core:grow(6)) * mask(function(x, y) return smoothstep(-0.15, 0.25, nz(x, y)) end)
work(outer, {hand="body", tool="filbert 3", pile=tHalf, coverage=1.1, angle=ang, angle_jitter=0.6, length={3, 7}, clip=zone, pressure={0.35, 0.7}, dips={8, 0.5, 0.4}})
print(wait(0))

--@ chunk 296
local core = treeAll - openings
local nz = noise{seed=7221, period=6, octaves=2}
local zone = (core:grow(9) - core:shrink(3)) * rect(0, 214, 96, 130)
local ang = function(x, y) return math.atan(y - 330, x - 200) + 0.7 * nz(x + 3, y) end
local fill = zone * mask(function(x, y) return smoothstep(-0.35, 0.05, nz(x, y)) end)
work(fill, {hand="body", tool="filbert 3", pile=slimEdgeB, coverage=1.8, angle=ang, angle_jitter=0.6, length={3, 7}, clip=zone, pressure={0.5, 0.85}, dips={8, 0.7, 0.4}})
print(wait(0))

--@ chunk 297
local core = treeAll - openings
local nz = noise{seed=7231, period=6, octaves=2}
local rim = (core:grow(4) - core:shrink(8)) * rect(0, 214, 100, 140)
local ang = function(x, y) return math.atan(y - 330, x - 200) + 0.7 * nz(x + 3, y) end
work(rim, {hand="body", tool="filbert 3", pile=slimEdgeB, coverage=2.2, angle=ang, angle_jitter=0.6, length={3, 7}, fill=true, clip=rim, pressure={0.55, 0.9}, dips={8, 0.75, 0.4}})
-- the old test marks in the sky at top left: tiny dabs of a sky tone
print(wait(0))

--@ chunk 298
print(wait(3 * 24 * 60))
print(drying(690, 260), drying(820, 360), drying(30, 280), drying(10, 185), drying(845, 300))

--@ chunk 299
print(wait(2 * 24 * 60))
print(drying(690, 260), drying(820, 360), drying(30, 280), drying(60, 300), drying(845, 300))

--@ chunk 300
print(drying(700, 450), drying(747, 455), drying(815, 445), drying(800, 435), drying(900, 438))
local rg = brush{kind="round", width=3.5, point=0.7}
for i, t in ipairs({{{699, 380}, {699, 410}, {700, 435}, {701, 456}}, {{750, 372}, {748, 405}, {747, 432}, {746, 458}}}) do
  rg:reload(trD, 0.9)
  rg:stroke(t, {pressure={0.6, 0.8}, ramps={0.2, 0.05}, shake=0.35})
end
print(wait(0))

--@ chunk 301
local fb = brush{kind="filbert", width=5.5}
fb:reload(trD, 0.9)
fb:stroke({{700, 459}, {699, 435}, {698, 410}, {696, 385}, {694, 362}, {692, 350}}, {pressure={0.95, 0.6}, ramps={0.03, 0.35}, shake=0.3})
local fb2 = brush{kind="filbert", width=4.5}
fb2:reload(trD, 0.9)
fb2:stroke({{746, 461}, {747, 435}, {748, 410}, {749, 385}}, {pressure={0.9, 0.6}, ramps={0.03, 0.4}, shake=0.3})
fb2:reload(trD, 0.7)
fb2:stroke({{815, 455}, {815, 440}, {815, 425}}, {pressure={0.85, 0.6}, ramps={0.03, 0.4}, shake=0.3})
print(wait(0))

--@ chunk 302
local g = brush{kind="round", width=1.6, point=0.8}
local n = 0
for _, f in ipairs({{700, 459}, {746, 461}, {815, 455}}) do
  for i = 1, 9 do
    local x = f[1] + rand(-6, 6)
    local y = f[2] + rand(0, 4)
    g:reload((i % 3 == 0) and mMidB or mLitG, 0.6)
    g:stroke({{x, y}, {x + rand(-1.5, 1.5), y - rand(5, 10)}}, {pressure={0.7, 0.0}, ramps={0.02, 0.8}})
    n = n + 1
  end
end
print(n, wait(0))

--@ chunk 303
print(drying(980, 438), drying(900, 435))
glzStrip = pile{{"raw umber", 1.0}, {"Antwerp blue", 0.25}, {"cobalt blue", 0.3}, {"red earth", 0.1}, medium=0.8}
local b = brush{kind="filbert", width=8, stiffness=0.3}
b:load(glzStrip, 0.4)
b:stroke({{965, 437}, {998, 437}}, {pressure={0.4, 0.4}})
print(wait(0))

--@ chunk 304
local rag = brush{kind="filbert", width=8, stiffness=0.5}
for i = 1, 6 do rag:wipe(1); rag:stroke({{960, 437}, {1000, 437}}, {pressure={0.9, 0.9}}) end
for i = 1, 3 do rag:wipe(1); rag:stroke({{960, 434}, {1000, 434}}, {pressure={0.8, 0.8}}); rag:wipe(1); rag:stroke({{960, 440}, {1000, 440}}, {pressure={0.8, 0.8}}) end
local b = brush{kind="filbert", width=14, stiffness=0.25}
b:load(glzStrip, 0.15)
b:wipe(0.5)
b:stroke({{920, 437}, {945, 436}, {970, 437}}, {pressure={0.3, 0.3}})
print(wait(0))

--@ chunk 305
local rag = brush{kind="filbert", width=10, stiffness=0.5}
for i = 1, 6 do rag:wipe(1); rag:stroke({{912, 437}, {976, 437}}, {pressure={0.9, 0.9}}) end
for i = 1, 3 do rag:wipe(1); rag:stroke({{912, 433}, {976, 433}}, {pressure={0.8, 0.8}}); rag:wipe(1); rag:stroke({{912, 441}, {976, 441}}, {pressure={0.8, 0.8}}) end
print(wait(0))

--@ chunk 306
local rag = brush{kind="filbert", width=4, stiffness=0.6}
local z = rect(960, 428, 40, 18)
for pass = 1, 3 do
  for i = 1, 250 do
    local x, y = rand(960, 1000), rand(428, 446)
    rag:wipe(1)
    local a = rand(-math.pi, math.pi)
    rag:stroke({{x - 3 * math.cos(a), y - 3 * math.sin(a)}, {x + 3 * math.cos(a), y + 3 * math.sin(a)}}, {pressure={0.85, 0.85}, clip=z})
  end
end
print(wait(0))

--@ chunk 307
print(wait(5 * 24 * 60))
print(drying(690, 260), drying(30, 280), drying(700, 440), drying(980, 437))

--@ chunk 308
local b = brush{kind="filbert", width=4}
b:load(s3, 0.5)
b:stroke({{0, 182}, {10, 181}, {20, 182}}, {pressure={0.55, 0.5}})
b:reload(s3, 0.5)
b:stroke({{0, 189}, {10, 188}, {20, 189}}, {pressure={0.55, 0.5}})
print(wait(0))

--@ chunk 309
local z = rect(0, 174, 26, 22):soften(4)
blend(z, {angle=0.0, tool={kind="badger", width=10}, pressure={0.35, 0.5}})
blend(z, {angle=1.4, tool={kind="badger", width=10}, pressure={0.3, 0.45}})
print(wait(0))

--@ chunk 310
local b = brush{kind="filbert", width=5}
for _, y in ipairs({181, 189}) do
  b:reload(s3, 0.7)
  b:stroke({{2, y}, {11, y}, {20, y}}, {pressure={0.75, 0.7}})
end
blend(rect(0, 174, 26, 22):soften(4), {angle=0.0, tool={kind="badger", width=10}, pressure={0.3, 0.45}})
print(wait(0))

--@ chunk 311
print(wait(5 * 24 * 60))
print(drying(10, 185), drying(700, 440))
