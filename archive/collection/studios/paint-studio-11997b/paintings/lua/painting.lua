-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
-- Dawn over a heath: setup. Fine linen, warm ground under a light lead-white top,
-- the way Friedrich's commercially primed canvases were built.
canvas{size=720, aspect=1.38, linen={14, 12}, seed=7,
  ground={
    {pile={{"red earth", 2}, {"yellow ochre", 1}, {"raw umber", 0.5}}, um=110, apply="knife", texture=0.35},
    {pile={{"lead white", 3}, {"yellow ochre", 1}, {"red earth", 0.4}}, um=55, apply="knife", texture=0.22},
    {pile={{"lead white", 4}, {"yellow ochre", 1}}, um=35, apply="brush"}
  }}

print("W " .. W .. "  H " .. H)

-- the palette: few pigments, thin paint, smalt for the blues
skyPale  = pile{{"lead white", 6}, {"pale smalt", 1}, {"yellow ochre", 1}, medium=0.25}
skyCool  = pile{{"lead white", 3}, {"smalt", 2}, {"pale smalt", 1}, medium=0.30}
skyDeep  = pile{{"smalt", 3}, {"lead white", 2}, {"bone black", 0.3}, medium=0.30}
glow     = pile{{"lead white", 5}, {"chrome yellow", 1}, {"yellow ochre", 1}, {"vermilion", 0.15}, medium=0.25}
cloud    = pile{{"lead white", 4}, {"yellow ochre", 1}, {"red earth", 0.35}, {"smalt", 0.3}, medium=0.30}
farRidge = pile{{"pale smalt", 3}, {"lead white", 2}, {"red earth", 0.6}, medium=0.40}
midRidge = pile{{"smalt", 2}, {"lead white", 1.5}, {"red earth", 1}, {"bone black", 0.5}, medium=0.35}
mist     = pile{{"lead white", 5}, {"pale smalt", 1}, {"yellow ochre", 0.6}, medium=0.40}
firDark  = pile{{"bone black", 2}, {"green earth", 1.5}, {"smalt", 0.8}, {"yellow ochre", 0.4}, medium=0.20}
deepDark = pile{{"bone black", 2}, {"raw umber", 1}, {"smalt", 0.5}, medium=0.15}
bark     = pile{{"raw umber", 2}, {"bone black", 1}, {"red earth", 0.8}, medium=0.15}
heath    = pile{{"yellow ochre", 2}, {"raw umber", 1.2}, {"green earth", 0.8}, {"red earth", 0.6}, medium=0.25}
heathDark= pile{{"raw umber", 2}, {"green earth", 1}, {"bone black", 1}, medium=0.20}
water    = pile{{"pale smalt", 3}, {"lead white", 2}, {"smalt", 0.8}, medium=0.45}

-- a precise underdrawing, as he drew: horizon, the ridge lines, the pool,
-- the oak's skeleton and the small figures to be placed on the heath.
h = pencil("2H")
h:line({0,402, 90,396, 180,404, 260,398, 340,390, 430,396, 520,388, 610,394, 700,384, 790,390, 880,382, 1000,388}, {pressure=0.22})
h:line({0,428, 110,422, 210,430, 300,424, 400,416, 500,422, 600,414, 700,420, 800,412, 900,418, 1000,412}, {pressure=0.22})
h:line({0,452, 80,447, 150,454, 230,448, 320,456, 420,450, 520,458, 620,452, 720,458, 820,450, 910,456, 1000,450}, {pressure=0.22})
h:sketch({380,538, 420,522, 470,514, 540,512, 600,516, 632,528, 640,538}, {pressure=0.2, wander=2})
-- the oak
h:sketch({295,712, 300,660, 292,610, 302,560, 296,515, 308,478, 318,452}, {pressure=0.3, wander=2})
h:sketch({305,520, 255,485, 215,452, 185,420, 168,392, 158,368}, {pressure=0.28, wander=3})
h:sketch({312,470, 285,420, 272,378, 276,342, 290,312}, {pressure=0.28, wander=3})
h:sketch({318,452, 330,405, 338,362, 330,325, 318,296}, {pressure=0.28, wander=3})
h:sketch({316,458, 365,432, 410,408, 452,388, 490,372, 516,358}, {pressure=0.28, wander=3})
h:sketch({330,430, 372,392, 406,352, 428,318, 438,290}, {pressure=0.28, wander=3})
-- two birches at the right, and a spruce group left
h:sketch({852,592, 856,540, 850,492, 858,452}, {pressure=0.2, wander=2})
h:sketch({884,586, 888,538, 882,500, 886,470}, {pressure=0.2, wander=2})
h:sketch({150,470, 152,430, 148,398}, {pressure=0.2, wander=2})
print(wait(0))

--@ chunk 2
-- SKY: thin, horizontal, graduated. Cool smalt above, warm pale light at the horizon.
skyAll = rect(0, 0, 1000, 466)
zTop = mask(function(x, y) return 1 - smoothstep(110, 300, y) end) * skyAll
zMid = mask(function(x, y) return smoothstep(110, 300, y) * (1 - smoothstep(300, 432, y)) end) * skyAll
zLow = mask(function(x, y) return smoothstep(300, 432, y) end) * skyAll

work(zTop, {hand="broad", pile=skyDeep, angle=0, coverage=1.3, fill=true})
work(zMid, {hand="broad", pile=skyCool, angle=0, coverage=1.3, fill=true})
work(zLow, {hand="broad", pile=skyPale, angle=0, coverage=1.4, fill=true})

-- the veiled sun, low over the far country
halo  = (ellipse(672, 402, 330, 150):blur(95)) * skyAll
core  = (ellipse(672, 398, 150, 88):blur(38)) * skyAll
work(halo, {hand="broad", pile=glow, angle=0, coverage=0.9, fill=true, edge="soft"})
work(core, {hand="broad", pile=glow, angle=0, coverage=1.3, fill=true, edge="soft"})

-- fuse the gradations while all of it is open
blend(skyAll, {angle=0})
print(wait(0))

--@ chunk 3
-- rework the sky while it is open: quieter light, real stratus
glowSoft = pile{{"lead white", 6}, {"chrome yellow", 0.6}, {"yellow ochre", 0.8}, {"vermilion", 0.1}, medium=0.3}
cloudCool= pile{{"lead white", 3}, {"pale smalt", 1}, {"smalt", 0.8}, {"red earth", 0.2}, medium=0.35}

-- knock the yellow back over its whole spread, keep it only at the core
work(halo, {hand="broad", pile=skyPale, angle=0, coverage=1.3, fill=true, edge="lost"})
work(core, {hand="broad", pile=glowSoft, angle=0, coverage=1.2, fill=true, edge="soft"})
sun = ellipse(672, 398, 88, 52):blur(22)
work(sun, {hand="broad", pile=glow, angle=0, coverage=1.0, fill=true, edge="lost"})

-- deepen the zenith a little
zHigh = mask(function(x, y) return 1 - smoothstep(60, 190, y) end) * rect(0, 0, 1000, 200)
work(zHigh, {hand="broad", pile=skyDeep, angle=0, coverage=0.8, fill=true})

-- stratus: long flat bands, each its own shape
c1 = ellipse(250, 302, 275, 15):blur(11)
c2 = ellipse(565, 266, 315, 12):blur(9)
c3 = ellipse(395, 350, 355, 17):blur(13)
c4 = ellipse(810, 322, 215, 11):blur(8)
c5 = ellipse(155, 372, 185, 10):blur(9)
c6 = ellipse(700, 236, 170, 9):blur(7)

work(c2 * zTop, {hand="broad", pile=cloudCool, angle=0, coverage=0.8, fill=false, edge="lost", hug=false})
work(c6 * zTop, {hand="broad", pile=cloudCool, angle=0, coverage=0.7, fill=false, edge="lost", hug=false})
work(c1, {hand="broad", pile=cloud, angle=0, coverage=0.8, fill=false, edge="lost", hug=false})
work(c4, {hand="broad", pile=cloudCool, angle=0, coverage=0.7, fill=false, edge="lost", hug=false})
work(c3, {hand="broad", pile=cloud, angle=0, coverage=0.9, fill=false, edge="lost", hug=false})
work(c5, {hand="broad", pile=cloud, angle=0, coverage=0.7, fill=false, edge="lost", hug=false})

-- fuse the light and the bands into the sky, in one pass over the middle sky
band = (rect(0, 180, 1000, 466)):blur(30)
blend(band, {angle=0})
print(wait(0))

--@ chunk 4
wait(24 * 60)

-- the farthest country, barely darker than the sky it stands against
farPts = {0,398, 70,392, 150,396, 230,388, 310,394, 400,386, 470,392, 560,382, 640,374, 720,380, 800,372, 880,378, 1000,370}
mFar = below(farPts) * mask(function(x, y) return 1 - smoothstep(432, 468, y) end)
mFar = mFar:roughen(1.1, 50, 4)
work(mFar, {hand="broad", pile=farRidge, angle=0, coverage=1.7, clip=true, fill=true})

-- the next country, a shade nearer and warmer
midPts = {0,424, 90,418, 180,426, 270,418, 360,424, 450,412, 540,418, 630,408, 720,416, 810,406, 900,414, 1000,404}
mMid = below(midPts) * mask(function(x, y) return 1 - smoothstep(444, 476, y) end)
mMid = mMid:roughen(1.3, 40, 6)
work(mMid, {hand="broad", pile=midRidge, angle=0, coverage=1.5, clip=true, fill=true})

print(wait(0))

--@ chunk 5
-- the wooded ridge just beyond the valley: dark, its skyline broken by firs
wood = pile{{"smalt", 1.5}, {"green earth", 1}, {"red earth", 0.8}, {"bone black", 0.6}, {"lead white", 0.8}, medium=0.35}

function polyline_y(pts, x)
  if x <= pts[1] then return pts[2] end
  for i = 1, #pts - 3, 2 do
    if x >= pts[i] and x <= pts[i + 2] then
      local t = (x - pts[i]) / (pts[i + 2] - pts[i])
      return pts[i + 1] + t * (pts[i + 3] - pts[i + 1])
    end
  end
  return pts[#pts]
end

nearPts = {0,452, 80,447, 150,454, 230,448, 320,456, 420,450, 520,458, 620,452, 720,458, 820,450, 910,456, 1000,450}
mWood = below(nearPts) * mask(function(x, y) return 1 - smoothstep(462, 494, y) end)
mWood = mWood:roughen(1.5, 26, 8)
work(mWood, {hand="broad", pile=wood, angle=0, coverage=1.6, clip=true, fill=true})

-- individual trees standing on that skyline
rf = brush{kind="rigger", width=2.2, point=1, stiffness=0.5}
rf:load(wood, 0.55)
xs = uneven(120, 6, 994, 0.7, 0.5, 11)
for i = 1, #xs do
  local x = xs[i]
  local y0 = polyline_y(nearPts, x) + 1
  local h = rand(3, 11) * (0.7 + 0.6 * math.abs(math.sin(x * 0.021)))
  local lean = rand(-1.6, 1.6)
  rf:stroke({{x, y0}, {x + lean * 0.4, y0 - h * 0.55}, {x + lean, y0 - h}},
            {pressure={0.55, 0.12}, ramps={0.15, 0.5}})
end
print("trees " .. #xs .. "  " .. wait(0))

--@ chunk 6
-- MIST filling the valley, eating the feet of the hills
mistMain = mask(function(x, y) return smoothstep(436, 464, y) * (1 - smoothstep(478, 532, y)) end)
mistLit  = mask(function(x, y) return 0.5 + 0.5 * smoothstep(240, 860, x) end)
work(mistMain * mistLit, {hand="broad", pile=mist, angle=0, coverage=1.4, fill=true, edge="lost", hug=false})

-- wisps straying over the wooded ridge
w1 = ellipse(210, 462, 190, 13):blur(12)
w2 = ellipse(505, 458, 230, 11):blur(10)
w3 = ellipse(800, 456, 175, 12):blur(11)
w4 = ellipse(640, 478, 300, 16):blur(15)
for _, m in ipairs({w1, w2, w3, w4}) do
  work(m, {hand="broad", pile=mist, angle=0, coverage=0.9, fill=true, edge="lost", hug=false})
end

-- fuse into one soft bank
blend((mistMain + w1 + w2 + w3 + w4):grow(18):blur(16), {angle=0})

-- a cool shadow of mist still hanging at the left, away from the light
coolMist = pile{{"lead white", 3}, {"pale smalt", 1.2}, {"smalt", 0.5}, {"red earth", 0.15}, medium=0.4}
cm = ellipse(120, 486, 260, 20):blur(20)
work(cm, {hand="broad", pile=coolMist, angle=0, coverage=0.8, fill=true, edge="lost", hug=false})
print(wait(0))

--@ chunk 7
-- the wood back in front of the mist, firm skyline, feet to be hazed over
mWood2 = below(nearPts) * mask(function(x, y) return 1 - smoothstep(466, 473, y) end)
mWood2 = mWood2:roughen(1.4, 24, 9)
work(mWood2, {hand="broad", pile=wood, angle=0, coverage=1.7, clip=true, fill=true})

rf:load(wood, 0.5)
xs2 = uneven(95, 8, 992, 0.8, 0.6, 13)
for i = 1, #xs2 do
  local x = xs2[i]
  local y0 = polyline_y(nearPts, x) + 1.5
  local h = rand(2.5, 9) * (0.6 + 0.7 * math.abs(math.sin(x * 0.017 + 1.3)))
  local lean = rand(-1.4, 1.4)
  rf:stroke({{x, y0}, {x + lean * 0.5, y0 - h * 0.6}, {x + lean, y0 - h}},
            {pressure={0.6, 0.1}, ramps={0.15, 0.55}})
end

-- haze sinking the hills, brightest toward the sun
veil = ellipse(520, 434, 470, 30):blur(26)
work(veil, {hand="broad", pile=mist, angle=0, coverage=0.75, fill=true, edge="lost", hug=false})

-- the mist bank again over the wood's feet, feathered
mistFoot = mask(function(x, y) return smoothstep(450, 484, y) * (1 - smoothstep(502, 548, y)) end)
work(mistFoot, {hand="broad", pile=mist, angle=0, coverage=1.1, fill=true, edge="lost", hug=false})
blend((mistFoot + veil):grow(12):blur(14), {angle=0})
print(wait(0))

--@ chunk 8
-- LAND. Dark under the mist: the light is behind it.
midHeath = pile{{"green earth", 2}, {"raw umber", 1.2}, {"yellow ochre", 1}, {"smalt", 0.5}, {"bone black", 0.4}, medium=0.30}
foreHeath= pile{{"raw umber", 2}, {"green earth", 1.2}, {"bone black", 0.8}, {"red earth", 0.6}, medium=0.20}

mMidGround = mask(function(x, y) return smoothstep(492, 528, y) * (1 - smoothstep(572, 590, y)) end)
work(mMidGround, {hand="broad", pile=midHeath, angle=0.12, coverage=2.0, fill=true, edge="lost", hug=false})

mFore = mask(function(x, y) return smoothstep(566, 596, y) end) * rect(0, 0, 1000, 725)
work(mFore, {hand="broad", pile=foreHeath, angle=0.28, coverage=2.1, fill=true, edge="firm"})

-- break the flats up: patches of heather and dry grass, wet into wet
nz = noise{seed=3, octaves=3, period=140}
p1 = mask(function(x, y) return smoothstep(0.05, 0.45, nz(x, y)) end) * mMidGround
p2 = mask(function(x, y) return smoothstep(0.0, 0.4, -nz(x, y)) end) * mFore
work(p1, {hand="body", pile=heathDark, angle=0.15, coverage=1.2, fill=true, edge="soft"})
work(p2, {hand="body", pile=heath, angle=0.35, coverage=1.1, fill=true, edge="soft"})
print(wait(0))

--@ chunk 9
-- knock the land back: it must sit in shadow under the light
unify   = pile{{"raw umber", 2}, {"green earth", 1.5}, {"bone black", 1}, {"red earth", 0.3}, medium=0.35}
coolVeil= pile{{"smalt", 1.2}, {"green earth", 1}, {"bone black", 0.7}, {"lead white", 0.4}, medium=0.40}

work(mFore, {hand="body", pile=unify, angle=0.28, coverage=1.3, fill=true, edge="firm"})
work(p2, {hand="body", pile=unify, angle=0.35, coverage=0.9, fill=true, edge="soft"})
work(mMidGround * mask(function(x, y) return smoothstep(516, 540, y) end),
     {hand="body", pile=coolVeil, angle=0.12, coverage=1.2, fill=true, edge="soft"})

-- a bog pool holding the sky
pool = ellipse(516, 561, 118, 19):roughen(3.5, 22, 2)
work(pool, {hand="body", pile=water, angle=0, coverage=1.6, fill=true, clip=true})
poolLit = ellipse(516, 555, 104, 12):roughen(2.5, 18, 3)
work(poolLit, {hand="body", pile=glowSoft, angle=0, coverage=1.1, fill=true, clip=true})
-- banks: dark peat at the near edge, mist still at the far one
nearBank = (ellipse(516, 578, 132, 14):roughen(3, 20, 5)) * mFore
work(nearBank, {hand="body", pile=heathDark, angle=0.2, coverage=1.3, fill=true, clip=true})
print(wait(0))

--@ chunk 10
-- rebuild the middle distance as dark heath, and the pool slim and dim
water2 = pile{{"pale smalt", 2}, {"lead white", 1.2}, {"smalt", 1}, {"raw umber", 0.3}, medium=0.5}
band = mask(function(x, y) return smoothstep(508, 534, y) * (1 - smoothstep(588, 606, y)) end)
work(band, {hand="body", pile=midHeath, angle=0.15, coverage=1.5, fill=true, edge="soft"})

nz2 = noise{seed=9, octaves=3, period=95}
work(mask(function(x, y) return smoothstep(0.05, 0.5, nz2(x, y)) end) * band,
     {hand="body", pile=heathDark, angle=0.2, coverage=1.1, fill=true, edge="soft"})

-- the pool: a flat lens of water, not a plate
pool2 = ellipse(518, 566, 124, 11):roughen(3, 26, 7)
work(pool2, {hand="body", pile=water2, angle=0, coverage=1.1, fill=true, clip=true})
refl = ellipse(512, 562, 108, 5.5):roughen(1.8, 20, 8)
work(refl, {hand="body", pile=cloudCool, angle=0, coverage=0.85, fill=true, clip=true})
glint = ellipse(560, 563, 42, 3):roughen(1.2, 16, 9)
work(glint, {hand="body", pile=glowSoft, angle=0, coverage=0.8, fill=true, clip=true})

-- reeds and peat breaking its rim
stipple(pool2:rim(9, 4), {pile=heathDark, width=2.2, coverage=1.1, pressure={0.25, 0.5}, drag={2.5, -1.4}, cluster={0.5, 26}})
stipple((pool2:offset(7):shrink(4) - pool2:shrink(2)), {pile=heathDark, width=2.2, coverage=0.8, pressure={0.2, 0.42}, drag={3, -1.5}})
print(wait(0))

--@ chunk 11
-- dim the water: it mirrors a pale sky but sits in dark ground
dim = pile{{"smalt", 1.2}, {"pale smalt", 1.2}, {"raw umber", 0.5}, {"green earth", 0.4}, medium=0.5}
work(pool2:grow(2), {hand="body", pile=dim, angle=0, coverage=1.25, fill=true, clip=true})
refl2 = ellipse(506, 563, 94, 3.6):roughen(1.4, 18, 12)
work(refl2, {hand="body", pile=cloudCool, angle=0, coverage=0.7, fill=true, clip=true})
glint2 = ellipse(556, 564, 30, 2.2):roughen(1, 14, 13)
work(glint2, {hand="body", pile=glowSoft, angle=0, coverage=0.7, fill=true, clip=true})
stipple(pool2:rim(10, 5), {pile=deepDark, width=2.4, coverage=1.2, pressure={0.25, 0.55}, drag={3, -1.6}, cluster={0.55, 30}})

-- break the seam where the middle heath meets the foreground peat
nz3 = noise{seed=21, octaves=3, period=70}
tr = mask(function(x, y) return smoothstep(556, 584, y) * (1 - smoothstep(600, 628, y)) end)
tr = tr * mask(function(x, y) return 0.35 + 0.65 * smoothstep(-0.25, 0.35, nz3(x, y)) end)
work(tr, {hand="body", pile=heathDark, angle=0.3, coverage=1.2, fill=true, edge="soft"})
work(tr * mask(function(x, y) return smoothstep(0.1, 0.6, -nz3(x, y)) end),
     {hand="body", pile=heath, angle=0.35, coverage=1.0, fill=true, edge="soft"})
print(wait(0))

--@ chunk 12
-- let the mist and the land set before dark paint goes over them
print(wait(24 * 60))

-- SPRUCE GROUP at the left, standing in the bright mist on the ridge
sb = brush{kind="rigger", width=2.6, point=1, stiffness=0.55}
sb:load(firDark, 0.6)

function spruce(x, base, top, halfW)
  local h = base - top
  -- the trunk, seen as the dark core between the whorls
  sb:stroke({{x, base}, {x + rand(-1, 1), base - h * 0.5}, {x + rand(-1, 1), top + h * 0.12}},
            {pressure={0.7, 0.12}, ramps={0.12, 0.5}})
  -- whorls of branches, ascending at the top, drooping lower down
  for i = 1, 15 do
    local t = i / 15
    local y = top + t * h * 0.95
    local len = halfW * (0.12 + 0.88 * t ^ 1.35)
    local droop = 0.32 + 0.38 * t
    for side = -1, 1, 2 do
      sb:stroke({{x, y}, {x + side * len * 0.55, y + len * droop * 0.5}, {x + side * len, y + len * droop}},
                {pressure={0.55, 0.08}, ramps={0.2, 0.55}})
      if rand(0, 1) < 0.5 then
        sb:stroke({{x, y + 2}, {x + side * len * 0.7, y + 2.5 + len * droop * 0.72}},
                  {pressure={0.42, 0.06}, ramps={0.25, 0.6}})
      end
    end
  end
end

spruce(128, 472, 406, 13)
spruce(152, 470, 396, 15)
spruce(178, 468, 424, 10)

-- the foliage mass inside each spire, as touches of the tip
for _, tr in ipairs({{128, 439, 406, 13}, {152, 433, 396, 15}, {178, 446, 424, 10}}) do
  local cx, mid, top, hw = tr[1], tr[2], tr[3], tr[4]
  local cone = poly({{cx, top - 4}, {cx - hw, mid + 34}, {cx + hw * 0.9, mid + 36}}, true)
  cone = cone:roughen(3, 12, math.floor(cx))
  stipple(cone, {pile=firDark, width=2.6, coverage=1.5, pressure={0.25, 0.5}, drag={2, 0.6}, cluster={0.55, 22}, feather=0.4})
end

-- the mist still curling round their feet
wL = ellipse(150, 474, 120, 16):blur(14)
work(wL, {hand="broad", pile=mist, angle=0, coverage=0.7, fill=true, edge="lost", hug=false})
print("spruces done  " .. wait(0))

--@ chunk 13
-- TWO BIRCHES at the right, standing on the middle heath
birchPale = pile{{"lead white", 5}, {"pale smalt", 1}, {"yellow ochre", 0.4}, {"raw umber", 0.35}, medium=0.3}
birchDark = pile{{"raw umber", 1.4}, {"bone black", 1}, {"red earth", 0.7}, medium=0.25}
birchTwig = pile{{"red earth", 1.2}, {"raw umber", 1}, {"bone black", 0.5}, medium=0.3}

tb = brush{kind="round", width=3.2, point=0.55, stiffness=0.5}
tb:load(birchPale, 0.75)
tb:stroke({{852, 593}, {853, 560}, {851, 522}, {855, 486}, {858, 455}}, {pressure={0.85, 0.3}, ramps={0.08, 0.35}})
tb:reload(birchPale, 0.75)
tb:stroke({{884, 587}, {886, 556}, {883, 528}, {887, 502}, {889, 476}}, {pressure={0.8, 0.28}, ramps={0.08, 0.35}})

-- dark bark at the feet, lenticels and scars along the pale trunks
td = brush{kind="rigger", width=1.6, point=1, stiffness=0.5}
td:load(birchDark, 0.55)
td:stroke({{850, 593}, {851, 578}, {852, 566}}, {pressure={0.75, 0.25}, ramps={0.2, 0.5}})
td:stroke({{883, 587}, {884, 573}, {885, 562}}, {pressure={0.7, 0.22}, ramps={0.2, 0.5}})
for _, y in ipairs({556, 543, 530, 517, 504, 492, 481, 470}) do
  td:stroke({{849 + rand(-1, 1), y}, {856 + rand(-1, 1), y + rand(-0.8, 0.8)}}, {pressure={0.5, 0.18}, ramps={0.25, 0.5}})
end
for _, y in ipairs({552, 538, 524, 511, 498, 486, 476}) do
  td:stroke({{881 + rand(-1, 1), y}, {888 + rand(-1, 1), y + rand(-0.8, 0.8)}}, {pressure={0.48, 0.16}, ramps={0.25, 0.5}})
end

-- the crowns: bare, open, the fine branches distinctly drooping
tt = brush{kind="rigger", width=1.8, point=1, stiffness=0.45}
tt:load(birchTwig, 0.5)
-- first birch: its own boughs
tt:stroke({{857, 488}, {842, 473}, {831, 478}}, {pressure={0.55, 0.08}, ramps={0.2, 0.6}})
tt:stroke({{856, 472}, {871, 459}, {881, 465}}, {pressure={0.55, 0.08}, ramps={0.2, 0.6}})
tt:stroke({{857, 461}, {845, 448}, {836, 453}}, {pressure={0.5, 0.07}, ramps={0.2, 0.6}})
tt:stroke({{858, 456}, {872, 443}, {883, 449}}, {pressure={0.5, 0.07}, ramps={0.2, 0.6}})
tt:stroke({{856, 479}, {867, 469}, {875, 474}}, {pressure={0.45, 0.07}, ramps={0.25, 0.6}})
tt:stroke({{854, 498}, {840, 488}, {830, 493}}, {pressure={0.45, 0.07}, ramps={0.25, 0.6}})
-- second birch
tt:stroke({{886, 505}, {873, 493}, {864, 498}}, {pressure={0.5, 0.07}, ramps={0.2, 0.6}})
tt:stroke({{888, 490}, {901, 479}, {909, 485}}, {pressure={0.5, 0.07}, ramps={0.2, 0.6}})
tt:stroke({{888, 479}, {877, 467}, {869, 472}}, {pressure={0.48, 0.07}, ramps={0.2, 0.6}})
tt:stroke({{889, 476}, {900, 465}, {908, 470}}, {pressure={0.45, 0.07}, ramps={0.25, 0.6}})
tt:stroke({{888, 470}, {882, 458}, {878, 461}}, {pressure={0.4, 0.06}, ramps={0.25, 0.65}})

-- twig haze hanging off both crowns, each stroke its own
for i = 1, 26 do
  local bx = rand(828, 886)
  local by = rand(442, 502)
  local l = rand(5, 13)
  tt:stroke({{bx, by}, {bx + rand(-3, 3), by + l * 0.5}, {bx + rand(-5, 5), by + l}},
            {pressure={0.32, 0.04}, ramps={0.3, 0.6}})
end
for i = 1, 20 do
  local bx = rand(866, 912)
  local by = rand(452, 508)
  local l = rand(4, 11)
  tt:stroke({{bx, by}, {bx + rand(-3, 3), by + l * 0.5}, {bx + rand(-4, 4), by + l}},
            {pressure={0.3, 0.04}, ramps={0.3, 0.6}})
end
print("birches done  " .. wait(0))

--@ chunk 14
-- the birches read as pale poles with red scribbles: paint the sky and hill
-- back over their upper halves and rebuild them lower and dimmer
hillBack = pile{{"lead white", 2.2}, {"green earth", 1}, {"yellow ochre", 0.8}, {"pale smalt", 0.8}, {"raw umber", 0.5}, medium=0.35}
mistBack = pile{{"lead white", 3}, {"pale smalt", 1}, {"yellow ochre", 0.5}, {"green earth", 0.3}, medium=0.4}

cover = rect(802, 428, 132, 90)
work(cover, {hand="broad", pile=hillBack, angle=0, coverage=1.6, fill=true, edge="lost"})
work(rect(802, 428, 132, 44) * mask(function(x, y) return 1 - smoothstep(450, 474, y) end),
     {hand="broad", pile=mistBack, angle=0, coverage=1.2, fill=true, edge="lost"})
blend(cover:grow(16):blur(20), {angle=0})

-- dim the trunks where they still stand: silver-grey, not white
dimB = pile{{"lead white", 2}, {"raw umber", 0.8}, {"green earth", 0.6}, {"smalt", 0.4}, medium=0.35}
trA = ribbon({{852, 594}, {853, 562}, {851, 530}, {854, 512}}, 6)
trB = ribbon({{884, 588}, {886, 558}, {883, 532}, {886, 515}}, 5.5)
work(trA + trB, {hand="detail", pile=dimB, coverage=1.3, fill=true, clip=true})

-- the crowns rebuilt: dull, bare, distinctly drooping
twigDull = pile{{"raw umber", 1.6}, {"bone black", 1}, {"red earth", 0.5}, medium=0.3}
tt2 = brush{kind="rigger", width=1.6, point=1, stiffness=0.45}
tt2:load(twigDull, 0.55)

-- first birch
tt2:stroke({{853, 517}, {841, 507}, {832, 512}}, {pressure={0.5, 0.07}, ramps={0.2, 0.6}})
tt2:stroke({{854, 511}, {867, 501}, {877, 506}}, {pressure={0.5, 0.07}, ramps={0.2, 0.6}})
tt2:stroke({{853, 505}, {844, 495}, {836, 500}}, {pressure={0.45, 0.06}, ramps={0.2, 0.6}})
tt2:stroke({{855, 501}, {866, 492}, {875, 496}}, {pressure={0.45, 0.06}, ramps={0.2, 0.6}})
tt2:stroke({{854, 526}, {843, 519}, {835, 523}}, {pressure={0.42, 0.06}, ramps={0.25, 0.6}})
tt2:stroke({{855, 508}, {861, 498}, {864, 491}}, {pressure={0.4, 0.05}, ramps={0.25, 0.65}})
-- second birch
tt2:stroke({{885, 521}, {874, 512}, {866, 516}}, {pressure={0.48, 0.06}, ramps={0.2, 0.6}})
tt2:stroke({{886, 515}, {897, 506}, {905, 510}}, {pressure={0.48, 0.06}, ramps={0.2, 0.6}})
tt2:stroke({{886, 509}, {877, 500}, {870, 504}}, {pressure={0.42, 0.06}, ramps={0.2, 0.6}})
tt2:stroke({{887, 505}, {896, 497}, {903, 501}}, {pressure={0.42, 0.06}, ramps={0.25, 0.6}})
tt2:stroke({{886, 512}, {881, 503}, {878, 498}}, {pressure={0.36, 0.05}, ramps={0.25, 0.65}})

-- twiglets hanging from those boughs
for i = 1, 22 do
  local bx = rand(833, 878)
  local by = rand(490, 528)
  local l = rand(4, 9)
  tt2:stroke({{bx, by}, {bx + rand(-1.2, 1.2), by + l * 0.55}, {bx + rand(-2, 2), by + l}},
             {pressure={0.3, 0.04}, ramps={0.3, 0.6}})
end
for i = 1, 16 do
  local bx = rand(866, 906)
  local by = rand(497, 530)
  local l = rand(3.5, 8)
  tt2:stroke({{bx, by}, {bx + rand(-1.2, 1.2), by + l * 0.55}, {bx + rand(-2, 2), by + l}},
             {pressure={0.28, 0.04}, ramps={0.3, 0.6}})
end

-- plant their feet in the dark heath
tb:load(birchDark, 0.5)
tb:touch(852, 592, {pressure=0.5, drag={2, -0.6}, twist=0.2})
tb:touch(884, 587, {pressure=0.45, drag={2, -0.6}, twist=0.2})
print("birches rebuilt  " .. wait(0))

--@ chunk 15
-- clear the right-hand birches away and stand the country back up behind them
clumpBack = pile{{"raw umber", 1.6}, {"green earth", 1}, {"bone black", 0.8}, {"yellow ochre", 0.5}, medium=0.25}
coverAll = rect(794, 422, 152, 186)

work(coverAll * mask(function(x, y) return 1 - smoothstep(468, 502, y) end),
     {hand="broad", pile=mistBack, angle=0, coverage=1.4, fill=true, edge="lost"})
work(coverAll * mask(function(x, y) return smoothstep(468, 502, y) * (1 - smoothstep(532, 562, y)) end),
     {hand="broad", pile=hillBack, angle=0, coverage=1.5, fill=true, edge="lost"})
work(coverAll * mask(function(x, y) return smoothstep(532, 562, y) end),
     {hand="body", pile=clumpBack, angle=0.2, coverage=1.6, fill=true, edge="lost"})
nzC = noise{seed=31, octaves=3, period=55}
work(coverAll * mask(function(x, y) return smoothstep(528, 558, y) * smoothstep(0.0, 0.45, nzC(x, y)) end),
     {hand="body", pile=heathDark, angle=0.25, coverage=1.1, fill=true, edge="soft"})
work(coverAll * mask(function(x, y) return smoothstep(520, 548, y) * smoothstep(0.05, 0.5, -nzC(x, y)) end),
     {hand="body", pile=wood, angle=0.1, coverage=1.0, fill=true, edge="soft"})
blend((coverAll * mask(function(x, y) return 1 - smoothstep(530, 556, y) end)):grow(18):blur(22), {angle=0})

-- two birches, small, rooted in the dark scrub
birchPale2 = pile{{"lead white", 3.5}, {"pale smalt", 1}, {"raw umber", 0.6}, {"green earth", 0.5}, medium=0.3}
tb2 = brush{kind="round", width=2.6, point=0.5, stiffness=0.5}
tb2:load(birchPale2, 0.7)
tb2:stroke({{846, 580}, {847, 559}, {846, 539}, {849, 517}}, {pressure={0.85, 0.3}, ramps={0.08, 0.35}})
tb2:reload(birchPale2, 0.7)
tb2:stroke({{881, 574}, {883, 555}, {881, 539}, {883, 523}}, {pressure={0.8, 0.28}, ramps={0.08, 0.35}})

-- dark feet and lenticels
td2 = brush{kind="rigger", width=1.4, point=1, stiffness=0.5}
td2:load(birchDark, 0.55)
td2:stroke({{845, 580}, {846, 569}, {847, 561}}, {pressure={0.7, 0.2}, ramps={0.2, 0.5}})
td2:stroke({{880, 574}, {881, 565}, {882, 557}}, {pressure={0.65, 0.2}, ramps={0.2, 0.5}})
for _, y in ipairs({553, 543, 533, 524}) do
  td2:stroke({{844 + rand(-0.8, 0.8), y}, {850 + rand(-0.8, 0.8), y + 0.5}}, {pressure={0.45, 0.15}, ramps={0.3, 0.5}})
end
for _, y in ipairs({551, 541, 532}) do
  td2:stroke({{879 + rand(-0.8, 0.8), y}, {885 + rand(-0.8, 0.8), y + 0.5}}, {pressure={0.42, 0.15}, ramps={0.3, 0.5}})
end

-- crowns as masses of fine twigs
crownA = ellipse(849, 521, 26, 19):roughen(4, 15, 5)
stipple(crownA, {pile=twigDull, width=2, coverage=1.6, pressure={0.28, 0.5}, drag={2, 1.2}, cluster={0.6, 20}, feather=0.5})
crownB = ellipse(882, 531, 21, 16):roughen(3.5, 13, 6)
stipple(crownB, {pile=twigDull, width=2, coverage=1.5, pressure={0.26, 0.48}, drag={2, 1.2}, cluster={0.6, 18}, feather=0.5})

-- boughs drawn through the masses, tips drooping
tt3 = brush{kind="rigger", width=1.5, point=1, stiffness=0.45}
tt3:load(twigDull, 0.5)
tt3:stroke({{848, 528}, {837, 521}, {830, 525}}, {pressure={0.5, 0.06}, ramps={0.2, 0.6}})
tt3:stroke({{850, 524}, {861, 516}, {869, 520}}, {pressure={0.5, 0.06}, ramps={0.2, 0.6}})
tt3:stroke({{849, 517}, {841, 508}, {835, 511}}, {pressure={0.42, 0.05}, ramps={0.25, 0.6}})
tt3:stroke({{850, 513}, {858, 505}, {864, 508}}, {pressure={0.42, 0.05}, ramps={0.25, 0.6}})
tt3:stroke({{881, 537}, {872, 531}, {866, 535}}, {pressure={0.45, 0.06}, ramps={0.2, 0.6}})
tt3:stroke({{882, 533}, {891, 526}, {897, 530}}, {pressure={0.45, 0.06}, ramps={0.2, 0.6}})
tt3:stroke({{882, 527}, {876, 519}, {872, 522}}, {pressure={0.38, 0.05}, ramps={0.25, 0.65}})
for i = 1, 18 do
  local bx = rand(828, 872)
  local by = rand(508, 538)
  local l = rand(3, 7)
  tt3:stroke({{bx, by}, {bx + rand(-1, 1), by + l * 0.55}, {bx + rand(-1.8, 1.8), by + l}},
             {pressure={0.28, 0.04}, ramps={0.3, 0.6}})
end
for i = 1, 13 do
  local bx = rand(865, 900)
  local by = rand(518, 545)
  local l = rand(2.5, 6)
  tt3:stroke({{bx, by}, {bx + rand(-1, 1), by + l * 0.55}, {bx + rand(-1.8, 1.8), by + l}},
             {pressure={0.26, 0.04}, ramps={0.3, 0.6}})
end
print("birches third pass  " .. wait(0))

--@ chunk 16
-- the scrub band under the birches: dark, its top edge broken, the pale blotches under the trunks gone
scrubM = rect(738, 524, 262, 98) * mask(function(x, y) return smoothstep(524, 548, y) * (0.62 + 0.38 * nzC(x, y)) end)
work(scrubM, {hand="body", pile=clumpBack, angle=0.22, coverage=1.8, fill=true, edge="soft"})
work(rect(738, 524, 262, 98) * mask(function(x, y) return smoothstep(530, 554, y) * smoothstep(0.05, 0.5, nzC(x, y)) end),
     {hand="body", pile=heathDark, angle=0.25, coverage=1.3, fill=true, edge="soft"})
work(rect(738, 524, 262, 98) * mask(function(x, y) return smoothstep(522, 546, y) * smoothstep(0.1, 0.55, -nzC(x, y)) end),
     {hand="body", pile=wood, angle=0.12, coverage=1.1, fill=true, edge="soft"})
blend((rect(738, 528, 262, 44) * mask(function(x, y) return smoothstep(528, 552, y) end)):blur(10), {angle=0})

-- the trunks again, grey this time
trA2 = ribbon({{846, 522}, {847, 546}, {845, 562}, {843, 580}}, 6)
trB2 = ribbon({{881, 528}, {882, 548}, {880, 562}, {878, 577}}, 5)
work(trA2 + trB2, {hand="detail", pile=dimB, coverage=1.6, fill=true, clip=true})

-- a pale sliver on the side toward the sun, the other side in shadow
r2 = brush{kind="rigger", width=1.8, point=1, stiffness=0.5}
r2:load(birchPale2, 0.5)
r2:stroke({{848, 522}, {849, 546}, {847, 562}, {846, 578}}, {pressure={0.5, 0.18}, ramps={0.15, 0.45}})
r2:stroke({{883, 528}, {884, 548}, {882, 562}, {881, 575}}, {pressure={0.45, 0.16}, ramps={0.15, 0.45}})
r2:load(birchDark, 0.5)
r2:stroke({{844, 524}, {845, 548}, {843, 564}, {842, 578}}, {pressure={0.5, 0.15}, ramps={0.2, 0.5}})
r2:stroke({{879, 530}, {880, 550}, {878, 564}, {877, 575}}, {pressure={0.45, 0.15}, ramps={0.2, 0.5}})

-- their feet, in the scrub
r2:stroke({{838, 582}, {846, 585}, {853, 583}}, {pressure={0.55, 0.12}, ramps={0.3, 0.5}})
r2:stroke({{873, 579}, {880, 581}, {887, 579}}, {pressure={0.5, 0.12}, ramps={0.3, 0.5}})
print("scrub and trunks  " .. wait(0))

--@ chunk 17
-- the right-hand corner is worked to mud: paint it out clean and plain
mistWide = pile{{"lead white", 3}, {"pale smalt", 1}, {"yellow ochre", 0.55}, {"green earth", 0.3}, medium=0.3}
hillWide = pile{{"green earth", 1.2}, {"lead white", 1.2}, {"yellow ochre", 0.9}, {"raw umber", 0.5}, {"pale smalt", 0.4}, medium=0.25}
darkWide = pile{{"raw umber", 1.8}, {"green earth", 1}, {"bone black", 0.9}, {"yellow ochre", 0.4}, medium=0.2}

region = rect(716, 410, 284, 222)
work(region * mask(function(x, y) return 1 - smoothstep(476, 504, y) end),
     {hand="broad", pile=mistWide, angle=0, coverage=2.3, fill=true, edge="lost", length={120, 260}})
work(region * mask(function(x, y) return smoothstep(476, 504, y) * (1 - smoothstep(526, 552, y)) end),
     {hand="broad", pile=hillWide, angle=0, coverage=2.3, fill=true, edge="lost", length={100, 220}})
work(region * mask(function(x, y) return smoothstep(526, 552, y) end),
     {hand="broad", pile=darkWide, angle=0.18, coverage=2.3, fill=true, edge="lost", length={90, 200}})

-- a low swell of darker scrub rising at the far right, as the wood had it
swell = (ellipse(980, 545, 90, 30):roughen(6, 40, 3)) * region
work(swell, {hand="body", pile=wood, angle=0.1, coverage=1.4, fill=true, edge="soft"})

-- fuse the new ground with the old at the seam
blend(rect(676, 410, 130, 222), {angle=0})
print("right corner repainted  " .. wait(0))

--@ chunk 18
-- the right end rebuilt in layers, each left to set before the next
cornerR = rect(726, 402, 274, 238) * mask(function(x, y) return smoothstep(726, 792, x) end)
work(cornerR, {hand="broad", pile=hillWide, angle=0, coverage=2.2, fill=true, edge="soft", hug=false, length={110, 240}})
print(wait(360))

--@ chunk 19
-- the dark scrub at the right end and the mist above it; the hill between stays as laid
scrubR = rect(726, 505, 274, 135) * mask(function(x, y) return smoothstep(726, 792, x) * smoothstep(502, 536, y) end)
work(scrubR, {hand="body", pile=darkWide, angle=0.2, coverage=2.1, fill=true, edge="soft", hug=false})

-- clumps of heather and gorse breaking the top of the scrub
nzR = noise{seed=37, octaves=3, period=75}
work(scrubR * mask(function(x, y) return smoothstep(0.02, 0.42, nzR(x, y)) end),
     {hand="body", pile=heathDark, angle=0.22, coverage=1.2, fill=true, edge="soft", hug=false})
work(scrubR * mask(function(x, y) return smoothstep(0.08, 0.5, -nzR(x, y)) end),
     {hand="body", pile=heath, angle=0.28, coverage=1.0, fill=true, edge="soft", hug=false})

-- the mist bank running on to the right edge of the picture
mistR = rect(726, 402, 274, 110) * mask(function(x, y) return smoothstep(726, 792, x) * (1 - smoothstep(452, 492, y)) end)
work(mistR, {hand="broad", pile=mistWide, angle=0, coverage=1.9, fill=true, edge="soft", hug=false, length={120, 250}})
print(wait(300))

--@ chunk 20
-- retexture the smeared column left of the corner with touches, which do not smear
featherX = mask(function(x, y) return smoothstep(638, 702, x) * (1 - smoothstep(812, 872, x)) end)

scrubZone = rect(632, 500, 250, 140) * featherX * mask(function(x, y) return smoothstep(512, 538, y) end)
stipple(scrubZone, {pile=darkWide, width=2.8, coverage=1.8, pressure={0.3, 0.55}, drag={2.6, 0.3}, cluster={0.55, 26}, feather=0.35})
nzR2 = noise{seed=41, octaves=3, period=70}
stipple(scrubZone * mask(function(x, y) return smoothstep(0.05, 0.45, nzR2(x, y)) end),
        {pile=heathDark, width=2.6, coverage=1.2, pressure={0.28, 0.5}, drag={2.4, -0.3}, cluster={0.6, 22}, feather=0.45})
stipple(scrubZone * mask(function(x, y) return smoothstep(0.1, 0.5, -nzR2(x, y)) end),
        {pile=heath, width=2.4, coverage=1.0, pressure={0.25, 0.45}, drag={2.2, 0.4}, cluster={0.55, 20}, feather=0.5})

hillZone = rect(632, 452, 250, 92) * featherX * mask(function(x, y) return smoothstep(455, 478, y) * (1 - smoothstep(512, 538, y)) end)
stipple(hillZone, {pile=hillWide, width=2.8, coverage=1.6, pressure={0.28, 0.5}, drag={2.6, 0.1}, cluster={0.5, 24}, feather=0.45})
stipple(hillZone * mask(function(x, y) return smoothstep(0.12, 0.55, nzR2(x, y)) end),
        {pile=wood, width=2.4, coverage=0.9, pressure={0.22, 0.42}, drag={2.2, -0.2}, cluster={0.6, 18}, feather=0.55})

mistZone = rect(632, 400, 250, 82) * featherX * mask(function(x, y) return 1 - smoothstep(452, 478, y) end)
work(mistZone, {hand="broad", pile=mistWide, angle=0, coverage=1.7, fill=true, edge="soft", hug=false, length={110, 230}})
print(wait(240))

--@ chunk 21
-- the right end once more: one broad field of hill, soft at its boundaries
field = rect(648, 448, 352, 154)
work(field, {hand="broad", pile=hillWide, angle=0.05, coverage=2.5, fill=true,
             edge={found=0.1, soft=0.5, lost=0.4, period=75, seed=6}, length={120, 260}, load=0.85})
mistTop = rect(648, 392, 352, 84) * mask(function(x, y) return 1 - smoothstep(438, 468, y) end)
work(mistTop, {hand="broad", pile=mistWide, angle=0, coverage=2.0, fill=true,
               edge={found=0, soft=0.45, lost=0.55, period=80, seed=7}, length={130, 270}, load=0.85})
print(wait(300))

--@ chunk 22
-- the dark scrub at the right end, as soft masses along the hill
scrubPts = {648, 532, 700, 526, 760, 530, 820, 522, 880, 528, 940, 518, 1000, 524}
mScrub = (below(scrubPts) * rect(648, 495, 352, 162)):roughen(9, 70, 8)
work(mScrub, {hand="broad", pile=darkWide, angle=0.14, coverage=2.2, fill=true,
              edge={found=0.15, soft=0.55, lost=0.3, period=60, seed=9}, length={90, 200}, load=0.85})

-- masses of gorse and heather standing on the band's top edge
c1 = (ellipse(720, 524, 55, 18):roughen(6, 30, 11)) * mScrub:grow(8)
work(c1, {hand="broad", pile=heathDark, angle=0.1, coverage=1.3, fill=true, edge="soft"})
c2 = (ellipse(852, 516, 70, 22):roughen(7, 34, 12)) * mScrub:grow(8)
work(c2, {hand="broad", pile=heathDark, angle=0.1, coverage=1.3, fill=true, edge="soft"})
c3 = (ellipse(958, 508, 62, 20):roughen(6, 28, 13)) * mScrub:grow(8)
work(c3, {hand="broad", pile=heathDark, angle=0.1, coverage=1.2, fill=true, edge="soft"})

-- dry heath catching the low light along the crests
crest = (ellipse(862, 512, 84, 11):blur(9)) * mScrub
work(crest, {hand="scumble", pile=heath, angle=0, coverage=0.7, edge="lost", hug=false})
print(wait(240))

--@ chunk 23
-- the last of the dot-matrix marks at the left edge of the repainted block
work(rect(604, 438, 74, 54) * mask(function(x, y) return 1 - smoothstep(466, 492, y) end),
     {hand="broad", pile=mistWide, angle=0, coverage=1.5, fill=true, edge="soft", hug=false})
work(rect(606, 484, 72, 52) * mask(function(x, y) return smoothstep(484, 500, y) * (1 - smoothstep(518, 538, y)) end),
     {hand="broad", pile=hillWide, angle=0.04, coverage=1.5, fill=true, edge="soft", hug=false})
work(rect(610, 516, 70, 46) * mask(function(x, y) return smoothstep(516, 534, y) * (1 - smoothstep(548, 562, y)) end),
     {hand="broad", pile=darkWide, angle=0.1, coverage=1.5, fill=true, edge="soft", hug=false})

-- dissolve the seam where the repainted hill meets the mist band
work(rect(612, 422, 388, 54), {hand="broad", pile=mistWide, angle=0, coverage=1.1, fill=true, edge="lost", hug=false, length={120, 260}})

-- the bright patch under the scrub crest, sunk
work(ellipse(828, 510, 28, 13):blur(7), {hand="body", pile=darkWide, angle=0.1, coverage=1.2, fill=true, edge="lost"})
print(wait(180))

--@ chunk 24
-- THE OAK: the picture's anchor, skeleton after the underdrawing
oakDeep = pile{{"bone black", 2}, {"raw umber", 1.2}, {"smalt", 0.5}, {"green earth", 0.3}, medium=0.18}
oak = body_of{
  spine={296, 718, 300, 662, 292, 612, 302, 560, 296, 515, 306, 480, 316, 455, 322, 432},
  widths={31, 26, 22, 18, 15, 12, 10, 8},
  limbs={
    {pts={302, 508, 258, 486, 218, 454, 186, 422, 168, 392, 156, 366}, widths={11, 8.5, 6.5, 4.5, 2.6, 1.2}},
    {pts={310, 472, 286, 422, 273, 378, 277, 342, 291, 310}, widths={9.5, 7, 5, 3, 1.4}},
    {pts={318, 454, 331, 406, 339, 362, 331, 325, 318, 294}, widths={8.5, 6, 4, 2.4, 1.1}},
    {pts={316, 458, 366, 433, 412, 409, 454, 389, 492, 372, 518, 356}, widths={9.5, 7.5, 5.5, 3.8, 2.2, 1}},
    {pts={330, 434, 373, 393, 407, 353, 429, 318, 439, 288}, widths={7, 5.5, 4, 2.6, 1.2}}
  },
  blend=0.85, char="firm"
}
work(oak:mask(), {hand="body", pile=oakDeep, angle=1.35, coverage=2.4, clip=true, fill=true, load=0.9})
print("oak skeleton laid  " .. wait(0))

--@ chunk 25
-- the oak's real mass: a heavy trunk and limbs thick at every fork
oak2 = body_of{
  spine={292, 722, 298, 665, 288, 612, 300, 562, 294, 518, 304, 482, 314, 456, 320, 436},
  widths={58, 47, 41, 35, 29, 23, 18, 14},
  limbs={
    {pts={300, 512, 256, 488, 216, 456, 184, 424, 166, 392, 154, 364}, widths={17, 13, 10, 7, 4, 1.8}},
    {pts={308, 476, 284, 424, 272, 378, 276, 342, 290, 308}, widths={15, 11, 8, 5, 2}},
    {pts={318, 458, 332, 408, 340, 362, 332, 326, 319, 292}, widths={13, 10, 7, 4.5, 1.8}},
    {pts={314, 462, 364, 436, 410, 412, 452, 392, 490, 374, 516, 358}, widths={14, 11, 8, 5.5, 3, 1.4}},
    {pts={328, 438, 370, 396, 404, 356, 426, 320, 437, 288}, widths={11, 8.5, 6, 3.5, 1.6}}
  },
  blend=0.85, char="firm"
}
work(oak2:mask(), {hand="body", pile=oakDeep, angle=1.35, coverage=2.2, clip=true, fill=true, load=0.9})

-- roots gripping the peat
sb2 = brush{kind="round", width=5, point=0.65, stiffness=0.55}
sb2:load(oakDeep, 0.85)
sb2:stroke({282, 712, 268, 704, 252, 700}, {pressure={0.85, 0.1}, ramps={0.15, 0.5}})
sb2:stroke({300, 716, 316, 706, 332, 702}, {pressure={0.85, 0.1}, ramps={0.15, 0.5}})
sb2:stroke({286, 700, 272, 692, 258, 690}, {pressure={0.7, 0.08}, ramps={0.2, 0.55}})

-- crooked secondary limbs: each its own path, thick where it leaves its parent
branches = {
  {{256, 488, 238, 462, 226, 438, 212, 414}, {0.85, 0.06}},
  {{216, 456, 198, 438, 180, 430, 160, 426}, {0.8, 0.06}},
  {{184, 424, 168, 408, 150, 402, 134, 398}, {0.7, 0.05}},
  {{284, 424, 300, 400, 310, 378, 312, 356}, {0.85, 0.05}},
  {{272, 378, 252, 362, 238, 344, 230, 324}, {0.75, 0.05}},
  {{276, 342, 258, 322, 246, 302, 242, 284}, {0.65, 0.05}},
  {{290, 308, 282, 288, 280, 268, 284, 252}, {0.5, 0.04}},
  {{332, 408, 348, 384, 358, 360, 356, 338}, {0.8, 0.05}},
  {{340, 362, 358, 346, 372, 336, 388, 332}, {0.7, 0.05}},
  {{332, 326, 348, 308, 358, 290, 356, 274}, {0.55, 0.04}},
  {{364, 436, 382, 452, 402, 464, 424, 470}, {0.8, 0.05}},
  {{412, 412, 432, 392, 456, 382, 478, 378}, {0.7, 0.05}},
  {{452, 392, 474, 374, 498, 364, 520, 358}, {0.55, 0.04}},
  {{370, 396, 388, 372, 400, 350, 402, 328}, {0.75, 0.05}},
  {{404, 356, 422, 338, 442, 328, 462, 324}, {0.6, 0.045}},
  {{426, 320, 442, 302, 458, 290, 472, 282}, {0.5, 0.04}},
  {{318, 452, 296, 462, 274, 476, 254, 488}, {0.75, 0.05}},
  {{330, 438, 352, 448, 376, 452, 398, 452}, {0.65, 0.05}}
}
for i = 1, #branches do
  local p = branches[i][1]
  local w = branches[i][2]
  if i % 3 == 1 then sb2:reload(oakDeep, 0.8) end
  sb2:stroke({{p[1], p[2]}, {p[3], p[4]}, {p[5], p[6]}, {p[7], p[8]}}, {pressure=w, ramps={0.12, 0.5}})
end
print("oak limbs  " .. wait(0))

--@ chunk 26
-- the crown's net of fine twigs, drawn stroke by stroke
tw = brush{kind="rigger", width=2.2, point=1, stiffness=0.5}
tw:load(oakDeep, 0.5)

tips = {{212, 414}, {160, 426}, {134, 398}, {312, 356}, {230, 324}, {242, 284}, {284, 252},
        {356, 338}, {388, 332}, {356, 274}, {424, 470}, {478, 378}, {520, 358}, {402, 328},
        {462, 324}, {472, 282}, {254, 488}, {398, 452}, {154, 364}, {290, 308}, {319, 292},
        {516, 358}, {437, 288}, {352, 448}, {300, 400}, {226, 438}, {376, 452}, {456, 382}}

for i = 1, #tips do
  local cx, cy = tips[i][1], tips[i][2]
  local base = math.atan(cy - 372, cx - 332)
  local n = rand(4, 7)
  for j = 1, n do
    local a = base + rand(-0.95, 0.95)
    local l = rand(8, 24)
    local x1 = cx + math.cos(a) * l * 0.5 + rand(-2, 2)
    local y1 = cy + math.sin(a) * l * 0.5 + rand(-2, 2) - l * 0.06
    local x2 = cx + math.cos(a) * l + rand(-3, 3)
    local y2 = cy + math.sin(a) * l + rand(-3, 3) - l * 0.1
    tw:stroke({{cx, cy}, {x1, y1}, {x2, y2}}, {pressure={0.48, 0.05}, ramps={0.25, 0.55}})
    if j % 3 == 0 then tw:reload(oakDeep, 0.45) end
  end
end

-- side twigs off the middle of each secondary limb
for i = 1, #branches do
  local p = branches[i][1]
  for k = 0, 1 do
    local bx = p[3 + k * 2]
    local by = p[4 + k * 2]
    local a = math.atan(by - 372, bx - 332) + rand(-1.2, 1.2)
    local l = rand(7, 16)
    tw:stroke({{bx, by}, {bx + math.cos(a) * l * 0.5, by + math.sin(a) * l * 0.5}, {bx + math.cos(a) * l, by + math.sin(a) * l}},
              {pressure={0.42, 0.04}, ramps={0.3, 0.6}})
    if (i + k) % 4 == 0 then tw:reload(oakDeep, 0.42) end
  end
end

-- a few dead stubs standing in the crown, snapped off blunt
tw:reload(oakDeep, 0.7)
tw:stroke({314, 342, 322, 318, 326, 300}, {pressure={0.75, 0.5}, ramps={0.2, 0.4}})
tw:stroke({262, 348, 252, 330, 248, 316}, {pressure={0.7, 0.45}, ramps={0.2, 0.4}})
tw:stroke({420, 366, 434, 350, 442, 338}, {pressure={0.65, 0.4}, ramps={0.2, 0.4}})
print("crown twigs  " .. wait(0))

--@ chunk 27
-- light from the veiled sun at the right: a warm rim along the oak's sun side
oakRim = pile{{"yellow ochre", 1.2}, {"red earth", 0.7}, {"lead white", 0.7}, {"raw umber", 0.5}, medium=0.3}
barkLit = pile{{"raw umber", 2}, {"red earth", 1}, {"yellow ochre", 0.6}, medium=0.25}
rr = brush{kind="rigger", width=2.4, point=1, stiffness=0.5}

-- the trunk's right edge, following its course
rr:load(oakRim, 0.55)
rr:stroke({{322, 716}, {321, 665}, {309, 612}, {318, 562}, {309, 518}, {316, 482}, {323, 456}}, {pressure={0.5, 0.3}, ramps={0.1, 0.35}})
-- the tops of the limbs, where they turn toward the light
rr:reload(oakRim, 0.5)
rr:stroke({{302, 508}, {258, 485}, {218, 453}, {186, 421}, {168, 390}, {156, 364}}, {pressure={0.45, 0.15}, ramps={0.15, 0.4}})
rr:reload(oakRim, 0.5)
rr:stroke({{310, 473}, {286, 421}, {274, 376}, {278, 340}, {292, 308}}, {pressure={0.45, 0.15}, ramps={0.15, 0.4}})
rr:reload(oakRim, 0.5)
rr:stroke({{320, 455}, {334, 405}, {342, 359}, {334, 323}, {321, 293}}, {pressure={0.45, 0.15}, ramps={0.15, 0.4}})
rr:reload(oakRim, 0.5)
rr:stroke({{316, 459}, {366, 433}, {412, 409}, {454, 389}, {492, 371}, {518, 356}}, {pressure={0.5, 0.15}, ramps={0.15, 0.4}})
rr:reload(oakRim, 0.5)
rr:stroke({{330, 435}, {372, 393}, {406, 353}, {428, 317}, {439, 287}}, {pressure={0.45, 0.12}, ramps={0.15, 0.45}})

-- bark grain down the trunk and its big limbs
rr:reload(barkLit, 0.5)
for _, x in ipairs({282, 292, 300, 308}) do
  rr:stroke({{x + rand(-1, 1), 714}, {x + rand(-2, 2), 640}, {x + rand(-2, 2), 560}, {x + rand(-2, 2), 490}},
            {pressure={0.35, 0.12}, ramps={0.15, 0.4}})
end
rr:reload(barkLit, 0.5)
rr:stroke({{270, 520}, {258, 486}, {242, 462}, {228, 442}}, {pressure={0.3, 0.1}, ramps={0.2, 0.45}})
rr:stroke({{322, 470}, {342, 452}, {362, 438}, {386, 424}}, {pressure={0.3, 0.1}, ramps={0.2, 0.45}})
-- the blocky scars of old bark
rr:reload(barkLit, 0.55)
for _, s in ipairs({{288, 690}, {302, 648}, {286, 606}, {304, 566}, {290, 526}, {302, 486}}) do
  rr:stroke({{s[1] - 7, s[2]}, {s[1] + 7, s[2] + rand(-1.5, 1.5)}}, {pressure={0.42, 0.15}, ramps={0.3, 0.5}})
end

-- roots gripping the peat
rr:reload(oakDeep, 0.8)
rr:stroke({286, 712, 270, 722}, {pressure={0.8, 0.3}, ramps={0.2, 0.4}})
rr:stroke({306, 712, 324, 722}, {pressure={0.8, 0.3}, ramps={0.2, 0.4}})
rr:stroke({296, 706, 288, 722}, {pressure={0.7, 0.25}, ramps={0.2, 0.4}})
rr:stroke({314, 704, 332, 716}, {pressure={0.6, 0.2}, ramps={0.25, 0.45}})
print("oak lit  " .. wait(0))

--@ chunk 28
-- two small figures on the heath, walking toward the light
fig = pile{{"bone black", 2}, {"raw umber", 1}, {"red earth", 0.4}, medium=0.15}
fb = brush{kind="round", width=2.2, point=0.6, stiffness=0.55}
fb:load(fig, 0.7)
-- the taller, standing
fb:stroke({{599, 613}, {599, 604}}, {pressure={0.6, 0.35}, ramps={0.2, 0.3}})
fb:stroke({{605, 613}, {604, 605}}, {pressure={0.55, 0.3}, ramps={0.2, 0.3}})
fb:stroke({{602, 606}, {602, 594}}, {pressure={0.85, 0.6}, ramps={0.15, 0.3}})
fb:touch(602, 591, {pressure=0.55, drag={0.5, -0.3}})
-- the shorter, walking
fb:reload(fig, 0.7)
fb:stroke({{616, 615}, {618, 606}}, {pressure={0.55, 0.3}, ramps={0.25, 0.35}})
fb:stroke({{623, 614}, {621, 607}}, {pressure={0.5, 0.28}, ramps={0.25, 0.35}})
fb:stroke({{619, 608}, {619, 598}}, {pressure={0.8, 0.55}, ramps={0.15, 0.3}})
fb:touch(619, 595, {pressure=0.5, drag={0.4, -0.3}})

-- their shadows thrown far to the left in the low light
sh = pile{{"raw umber", 1.5}, {"green earth", 1}, {"bone black", 0.8}, medium=0.35}
work(ellipse(572, 616, 34, 3.2):blur(3.5), {hand="body", pile=sh, angle=0, coverage=0.9, fill=true, edge="lost", hug=false})
work(ellipse(594, 619, 30, 2.8):blur(3), {hand="body", pile=sh, angle=0, coverage=0.85, fill=true, edge="lost", hug=false})

-- the middle distance quieted: pale specks and stray light marks sunk
work(rect(190, 482, 78, 42), {hand="body", pile=hillWide, angle=0.05, coverage=1.3, fill=true, edge="soft", hug=false})
work(rect(330, 476, 110, 34), {hand="body", pile=hillWide, angle=0.05, coverage=1.2, fill=true, edge="soft", hug=false})
work(rect(380, 490, 48, 30), {hand="body", pile=hillWide, angle=0.05, coverage=1.2, fill=true, edge="soft", hug=false})
print("figures  " .. wait(0))

--@ chunk 29
-- the figures again, solid little shapes rather than spider legs
fb2 = brush{kind="round", width=3.4, point=0.45, stiffness=0.55}
fb2:load(fig, 0.8)
fb2:stroke({{602, 607}, {602, 596}}, {pressure={0.9, 0.8}, ramps={0.15, 0.25}})
fb2:stroke({{601, 613}, {601, 606}}, {pressure={0.55, 0.4}, ramps={0.2, 0.3}})
fb2:stroke({{604, 613}, {604, 606}}, {pressure={0.5, 0.38}, ramps={0.2, 0.3}})
fb2:touch(602, 593, {pressure=0.6, drag={1.5, -1.57}})
fb2:reload(fig, 0.8)
fb2:stroke({{619, 609}, {619, 599}}, {pressure={0.85, 0.75}, ramps={0.15, 0.25}})
fb2:stroke({{617, 615}, {618, 608}}, {pressure={0.5, 0.36}, ramps={0.25, 0.3}})
fb2:stroke({{622, 614}, {621, 608}}, {pressure={0.48, 0.34}, ramps={0.25, 0.3}})
fb2:touch(619, 596, {pressure=0.55, drag={1.5, -1.57}})

-- the weave showing as a dot matrix right of the pond: fill it in
work(rect(558, 512, 134, 40), {hand="body", pile=midHeath, angle=0.1, coverage=1.5, fill=true, edge="soft", hug=false})
work(rect(558, 512, 134, 40) * mask(function(x, y) return smoothstep(0.1, 0.55, nz3(x, y)) end),
     {hand="body", pile=heathDark, angle=0.15, coverage=0.9, fill=true, edge="soft", hug=false})

-- the pond's bright core softened
work(ellipse(552, 562, 62, 7):roughen(2, 16, 4), {hand="scumble", pile=water2, coverage=0.55, pressure={0.18, 0.32}, edge="lost", hug=false})
print("figures and pond  " .. wait(0))

--@ chunk 30
-- the pond's reflection has come out like white stones: sink it all
work(ellipse(518, 564, 132, 17):roughen(3, 26, 7), {hand="body", pile=dim, angle=0, coverage=2.2, fill=true, clip=true})
print(wait(150))

--@ chunk 31
-- the water laid again: one calm lens, a soft reflection, one small glint
reflP = pile{{"lead white", 1.6}, {"pale smalt", 1}, {"yellow ochre", 0.5}, medium=0.45}
work(ellipse(518, 565, 124, 13):roughen(2.5, 24, 8), {hand="body", pile=water2, angle=0, coverage=1.3, fill=true, clip=true})
work(ellipse(538, 561, 88, 5):roughen(1.6, 18, 12):blur(1.2), {hand="body", pile=reflP, angle=0, coverage=0.7, fill=true, clip=true, edge="lost"})
work(ellipse(556, 562, 22, 2):blur(2), {hand="body", pile=glowSoft, angle=0, coverage=0.5, fill=true, clip=true, edge="lost"})

-- the peat bank along its near edge, and reeds breaking the rim
work(ellipse(516, 581, 132, 13):roughen(3, 20, 5), {hand="body", pile=heathDark, angle=0.2, coverage=1.2, fill=true, clip=true})
stipple((ellipse(518, 565, 124, 13):roughen(2.5, 24, 8)):rim(8, 4),
        {pile=heathDark, width=2.2, coverage=0.9, pressure={0.22, 0.45}, drag={2.5, -1.4}, cluster={0.5, 26}})
print(wait(120))

--@ chunk 32
-- clear the heath where the old figures and their shadows stood
work(rect(526, 586, 146, 56), {hand="body", pile=unify, angle=0.25, coverage=1.6, fill=true, edge="soft", hug=false})
work(rect(526, 586, 146, 56) * mask(function(x, y) return smoothstep(0.1, 0.55, nz3(x, y)) end),
     {hand="body", pile=heathDark, angle=0.3, coverage=1.0, fill=true, edge="soft", hug=false})
print(wait(150))

--@ chunk 33
-- two small figures standing at the water's edge, dark against the pale reflection
fb2:reload(fig, 0.85)
fb2:stroke({{553, 575}, {553, 563}}, {pressure={0.95, 0.85}, ramps={0.12, 0.2}})
fb2:stroke({{551, 581}, {551, 574}}, {pressure={0.6, 0.45}, ramps={0.2, 0.28}})
fb2:stroke({{555, 581}, {555, 574}}, {pressure={0.55, 0.42}, ramps={0.2, 0.28}})
fb2:touch(553, 560, {pressure=0.62, drag={1.8, -1.57}})
fb2:reload(fig, 0.85)
fb2:stroke({{574, 577}, {574, 566}}, {pressure={0.9, 0.8}, ramps={0.12, 0.2}})
fb2:stroke({{572, 583}, {573, 576}}, {pressure={0.55, 0.4}, ramps={0.25, 0.3}})
fb2:stroke({{577, 583}, {576, 576}}, {pressure={0.52, 0.38}, ramps={0.25, 0.3}})
fb2:touch(574, 563, {pressure=0.58, drag={1.8, -1.57}})

-- their shadows on the bank, thrown left by the low light
work(ellipse(534, 586, 22, 2.6):blur(3), {hand="body", pile=sh, angle=0, coverage=0.8, fill=true, edge="lost", hug=false})

-- the white scribbles on the hill right of the pond, sunk into the ground
work(rect(636, 470, 62, 78), {hand="body", pile=midHeath, angle=0.12, coverage=1.5, fill=true, edge="soft", hug=false})
work(rect(636, 470, 62, 78) * mask(function(x, y) return smoothstep(0.08, 0.5, -nz3(x, y)) end),
     {hand="body", pile=heathDark, angle=0.18, coverage=0.9, fill=true, edge="soft", hug=false})
print("figures by the water  " .. wait(0))

--@ chunk 34
-- a clump of old dark paint standing in the pond's right end: sink it into the water
work(rect(578, 544, 34, 32) * mask(function(x, y) return 1 - smoothstep(566, 576, y) end),
     {hand="body", pile=water2, angle=0, coverage=1.6, fill=true, clip=true, edge="soft"})
work(ellipse(568, 562, 34, 4):blur(3), {hand="body", pile=reflP, angle=0, coverage=0.6, fill=true, clip=true, edge="lost"})

-- stray white specks above and beside the pond, sunk into the heath
work(rect(518, 526, 32, 24), {hand="body", pile=midHeath, angle=0.1, coverage=1.4, fill=true, edge="soft", hug=false})
work(rect(640, 532, 36, 26), {hand="body", pile=midHeath, angle=0.1, coverage=1.4, fill=true, edge="soft", hug=false})
print(wait(90))

--@ chunk 35
hillOlive = pile{{"green earth", 1.2}, {"yellow ochre", 1}, {"raw umber", 0.8}, {"lead white", 0.5}, {"pale smalt", 0.3}, medium=0.3}

-- the pale slabs standing on the hillside: sink them into the hill
work(ellipse(162, 494, 54, 27):roughen(8, 30, 2), {hand="broad", pile=hillOlive, angle=0.08, coverage=1.7, fill=true, edge="lost", hug=false})
work(ellipse(385, 494, 82, 31):roughen(10, 36, 3), {hand="broad", pile=hillOlive, angle=0.08, coverage=1.7, fill=true, edge="lost", hug=false})
work(ellipse(424, 504, 48, 21):roughen(7, 28, 4), {hand="broad", pile=hillOlive, angle=0.1, coverage=1.5, fill=true, edge="lost", hug=false})
-- and the dark box at their right
work(ellipse(664, 502, 54, 48):roughen(10, 34, 5), {hand="broad", pile=hillOlive, angle=0.1, coverage=1.6, fill=true, edge="lost", hug=false})
work(ellipse(664, 528, 58, 26):roughen(8, 30, 6), {hand="broad", pile=darkWide, angle=0.14, coverage=1.4, fill=true, edge="lost", hug=false})

-- the pond's brightness taken down a step
work(ellipse(518, 565, 126, 14):roughen(2.5, 24, 8), {hand="scumble", pile=dim, angle=0, coverage=0.55, pressure={0.15, 0.3}, edge="soft", clip=true})
print(wait(120))

--@ chunk 36
-- the middle distance rebuilt as one band: pale olive under the mist
hillPale = pile{{"green earth", 1}, {"yellow ochre", 0.55}, {"raw umber", 0.55}, {"lead white", 1.1}, {"pale smalt", 0.5}, medium=0.3}
trunkMask = oak2:mask():grow(2)
bandFade = mask(function(x, y) return smoothstep(448, 468, y) * (1 - smoothstep(520, 540, y)) end)
hillBand = rect(0, 438, 1000, 112) * bandFade - trunkMask - rect(96, 446, 114, 38)
work(hillBand, {hand="broad", pile=hillPale, angle=0.06, coverage=2.2, fill=true, clip=true, length={110, 250}, load=0.9})
print(wait(240))
