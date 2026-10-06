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
