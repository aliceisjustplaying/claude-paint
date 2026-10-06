-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1

canvas{size=700, aspect=1.25, linen={16, 14}, seed=7,
  ground={{pile={{"lead white", 1}, {"yellow ochre", 0.3}}, um=120, apply="knife", texture=0.35},
          {pile={{"yellow ochre", 1}, {"red earth", 0.2}, {"lead white", 0.5}, {"bone black", 0.08}}, um=45, apply="brush"}}}
print(table.concat(tubes(), ", "))
print("W,H =", W, H)

--@ chunk 2

table_line = function(x) return 506 + 4*math.sin(x/170) + (x-500)*0.012 end

pw  = pile{{"raw umber", 1}, {"bone black", 0.32}, {"red earth", 0.28}, medium=0.22}
pwl = pile{{"yellow ochre", 1}, {"lead white", 0.55}, {"red earth", 0.12}, medium=0.2}
pt  = pile{{"yellow ochre", 1}, {"red earth", 0.32}, {"lead white", 0.5}, {"bone black", 0.1}, medium=0.18}
ptd = pile{{"raw umber", 1}, {"red earth", 0.35}, {"bone black", 0.22}, medium=0.28}

local wall = above(table_line)
work(wall, {hand="broad", pile=pw, angle=function(x, y) return -0.12 + 0.1*math.sin(x/260) end,
            coverage=1.15, edge={found=0.25, soft=0.5, lost=0.25, period=70, seed=3}, seed=11})

local tbl = below(table_line)
work(tbl, {hand="broad", pile=pt, angle=function(x, y) return 0.06 - 0.14*(x-500)/500 end,
           coverage=1.25, edge={found=0.3, soft=0.5, lost=0.2, period=80, seed=5}, seed=12})
print("blocked in")

--@ chunk 3

local wall = above(table_line)
local tbl = below(table_line)
work(wall, {hand="broad", pile=pw, angle=function(x, y) return -0.12 + 0.12*math.sin(x/230) end,
            coverage=2.1, fill=true, edge={found=0.25, soft=0.5, lost=0.25, period=70, seed=8}, seed=21})
work(wall, {hand="body", pile=pw, angle=function(x, y) return 1.35 + 0.25*math.sin(x/180) end,
            coverage=1.1, fill=true, edge={found=0.2, soft=0.55, lost=0.25, period=60, seed=9}, seed=22})
work(tbl, {hand="broad", pile=pt, angle=function(x, y) return 0.06 - 0.16*(x-500)/500 end,
           coverage=2.0, fill=true, edge={found=0.3, soft=0.5, lost=0.2, period=80, seed=15}, seed=23})
work(tbl, {hand="body", pile=pt, angle=function(x, y) return 0.12 - 0.2*(x-500)/500 end,
           coverage=1.0, fill=true, edge={found=0.25, soft=0.55, lost=0.2, period=70, seed=16}, seed=24})
print("filled")

--@ chunk 4

print("now:", wait(0))
print("wall:", drying(500, 250), "table:", drying(500, 650), "table2:", drying(200, 700))

--@ chunk 5

ptl = pile{{"lead white", 1}, {"yellow ochre", 0.5}, {"red earth", 0.12}, medium=0.22}
local tbl = below(table_line)
local tdark = ellipse(990, 600, 470, 400) * tbl
work(tdark, {hand="broad", pile=ptd, angle=function(x, y) return 0.05 - 0.12*(x-500)/500 end,
             coverage=0.9, edge="lost", load=0.6, seed=41})
local band = below(table_line) - below(function(x) return table_line(x) + 42 end)
work(band, {hand="body", pile=ptd, angle=function(x, y) return 0.05 end, coverage=0.8, edge="lost", load=0.5, seed=43})
local tlight = ellipse(260, 730, 520, 250) * tbl
work(tlight, {hand="broad", pile=ptl, angle=function(x, y) return 0.02 - 0.1*(x-500)/500 end,
              coverage=1.1, edge="lost", load=0.7, seed=42})
blend(tbl, {angle=0.05, coverage=0.5})
print("table modelled")

--@ chunk 6

local tbl = below(table_line)
work(tbl, {hand="broad", pile=pt, angle=function(x, y) return 0.04 - 0.13*(x-500)/500 end,
           coverage=1.7, fill=true, edge={found=0.3, soft=0.5, lost=0.2, period=90, seed=27}, seed=51})
work(tbl, {hand="body", pile=pt, angle=function(x, y) return 0.1 - 0.18*(x-500)/500 end,
           coverage=0.9, fill=true, edge="lost", seed=52})
print("table unified", wait(0))

--@ chunk 7

psh = pile{{"raw umber", 1}, {"red earth", 0.3}, {"bone black", 0.28}, medium=0.3}
local s1 = ribbon({{570, 596}, {650, 612}, {740, 624}, {830, 634}, {886, 636}}, 62)
local s2 = ribbon({{322, 672}, {390, 684}, {452, 692}}, 46)
local s3 = ribbon({{210, 706}, {272, 716}, {322, 722}}, 38)
local contact = ellipse(585, 596, 112, 26) + ellipse(330, 672, 84, 20) + ellipse(215, 706, 66, 16)
local shadow = (s1 + s2 + s3 + contact):soften(9)
work(shadow, {hand="body", pile=psh, angle=function(x, y) return 0.15 end,
              coverage=1.2, fill=true, edge="lost", load=0.6, seed=61})
blend(shadow:grow(6), {angle=0.2, coverage=0.5})
print("shadows laid", wait(0))

--@ chunk 8

jug_pts = {{485, 530}, {489, 452}, {516, 388}, {548, 352}, {547, 318}, {566, 306}, {604, 306},
           {623, 318}, {622, 352}, {654, 388}, {681, 452}, {685, 530}, {664, 582}, {610, 604},
           {552, 592}, {512, 560}}
jugm = poly(jug_pts, true)
handlem = ribbon({{668, 425}, {716, 442}, {736, 478}, {722, 520}, {684, 542}}, 17)
jugall = jugm + handlem
pj  = pile{{"lead white", 1}, {"yellow ochre", 0.12}, {"raw umber", 0.05}, medium=0.15}
pjs = pile{{"lead white", 1}, {"smalt", 0.32}, {"raw umber", 0.2}, medium=0.2}

work(jugall, {hand="body", pile=pj, angle=function(x, y) return 1.45 end,
              coverage=1.7, fill=true, clip=true, edge="found", seed=71})
local dark_side = jugall * mask(function(x, y) return smoothstep(520, 682, x) end)
work(dark_side, {hand="body", pile=pjs, angle=function(x, y) return 1.55 + 0.2*math.sin(y/90) end,
                 coverage=1.1, clip=true, edge="soft", load=0.65, seed=72})
print("jug blocked", wait(0))

--@ chunk 9

local jugall = jugm + handlem
ph  = pile{{"lead white", 1}, {"yellow ochre", 0.08}, medium=0.12}
pjs = pile{{"lead white", 1}, {"smalt", 0.34}, {"raw umber", 0.22}, medium=0.2}

work(jugall, {hand="body", pile=pj, angle=function(x, y) return 1.5 + 0.22*math.sin(y/70) end,
              coverage=2.2, fill=true, clip=true, edge="found", dips={2, 0.9, 0.25}, seed=81})

local js = body.ellipsoid({585, 480, 0}, {108, 128, 108})
local f = form{{js, dist=0.3}, light={from={-1, -0.7}, front=0.5, ambient=0.25}}
work(f:lit{parts={1}, soft=0.16} * jugall, {hand="body", pile=ph, angle=f:field("fall"),
    coverage=1.2, clip=true, edge="soft", dips={3, 0.75, 0.3}, seed=82})
work(f:shadow{parts={1}} * jugall, {hand="body", pile=pjs, angle=f:field("fall"),
    coverage=1.3, clip=true, edge="soft", dips={3, 0.8, 0.3}, seed=83})
print("jug modelled", wait(0))

--@ chunk 10

pjm = pile{{"raw umber", 1}, {"bone black", 0.5}, {"lead white", 0.35}, medium=0.2}
local cut = rect(430, 280, 340, 130) - ellipse(585, 352, 46, 62) - ellipse(585, 310, 54, 24)
work(cut, {hand="detail", pile=pw, clip=true, fill=true, coverage=1.8, seed=91})
local mouth = ellipse(585, 308, 42, 13)
work(mouth, {hand="detail", pile=pjm, clip=true, fill=true, coverage=1.6, seed=92})

local jugall = jugm + handlem
local sm = jugall * mask(function(x, y) return smoothstep(535, 690, x) end)
local lm = jugall * mask(function(x, y) return 1 - smoothstep(505, 645, x) end)
work(sm, {hand="body", pile=pjs, angle=function(x, y) return 1.55 + 0.3*math.sin(y/80) end,
          coverage=1.3, clip=true, edge="soft", dips={3, 0.8, 0.3}, seed=93})
work(lm, {hand="body", pile=ph, angle=function(x, y) return 1.5 + 0.3*math.sin(y/80) end,
          coverage=1.3, clip=true, edge="soft", dips={3, 0.85, 0.3}, seed=94})
blend(jugall, {angle=1.5, coverage=0.35})
print("top cut, form stronger", wait(0), drying(585, 480))

--@ chunk 11

pl  = pile{{"chrome yellow", 1}, {"yellow ochre", 0.45}, {"lead white", 0.12}, medium=0.18}
plh = pile{{"chrome yellow", 1}, {"lead white", 0.5}, {"yellow ochre", 0.15}, medium=0.15}
pls = pile{{"yellow ochre", 1}, {"red earth", 0.5}, {"green earth", 0.25}, medium=0.22}

lm1 = ellipse(330, 632, 78, 52) + ellipse(392, 620, 20, 13) + ellipse(268, 644, 18, 12)
lm2 = ellipse(212, 668, 62, 44) + ellipse(154, 676, 18, 12)
lem = lm1 + lm2

local l1 = body.ellipsoid({330, 632, 0}, {78, 52, 55})
local l2 = body.ellipsoid({212, 668, 0}, {62, 44, 46})
local lf = form{{l1, dist=0.3}, {l2, dist=0.3}, light={from={-1, -0.7}, front=0.5, ambient=0.25}}

work(lem, {hand="body", pile=pl, angle=lf:field("fall"), coverage=1.8, fill=true, clip=true,
           edge="found", dips={3, 0.85, 0.3}, seed=101})
work(lf:shadow{parts={1}} * lm1 + lf:shadow{parts={2}} * lm2,
     {hand="body", pile=pls, angle=lf:field("fall"), coverage=1.2, clip=true, edge="soft",
      dips={3, 0.8, 0.3}, seed=102})
work(lf:lit{parts={1}, soft=0.14} * lm1 + lf:lit{parts={2}, soft=0.14} * lm2,
     {hand="body", pile=plh, angle=lf:field("fall"), coverage=1.1, clip=true, edge="soft",
      dips={3, 0.85, 0.3}, seed=103})
blend(lem, {angle=0.6, coverage=0.3})
print("lemons in", wait(0))

--@ chunk 12

pshd = pile{{"raw umber", 1}, {"bone black", 0.5}, {"red earth", 0.25}, medium=0.22}
local s1 = ribbon({{566, 598}, {650, 612}, {742, 626}, {832, 636}, {888, 638}}, 68)
local s2 = ribbon({{318, 674}, {392, 686}, {456, 694}}, 52)
local s3 = ribbon({{206, 708}, {274, 718}, {326, 724}}, 44)
local contact = ellipse(585, 598, 120, 26) + ellipse(330, 674, 90, 21) + ellipse(215, 708, 72, 17)
local core = (ellipse(585, 596, 128, 34) + ellipse(330, 672, 94, 26) + ellipse(215, 706, 76, 21)):soften(10)
local shadow = (s1 + s2 + s3 + contact):soften(13)

work(shadow, {hand="scumble", pile=psh, angle=0.12, coverage=1.7, fill=true, edge="lost",
              load=0.8, seed=111})
work(core, {hand="scumble", pile=pshd, angle=0.1, coverage=1.3, fill=true, edge="lost",
            load=0.85, seed=112})
blend(shadow:grow(12), {angle=0.15, coverage=0.7})
print("shadows unified", wait(0))

--@ chunk 13

local wall = above(table_line)
local glow = (ellipse(150, 60, 640, 430) * wall):soften(60)
work(glow, {hand="scumble", pile=pwl, angle=function(x, y) return -0.2 + 0.2*math.sin(x/180) end,
            coverage=0.9, edge="lost", load=0.5, seed=121})
local shade = (ellipse(1010, 30, 580, 430) * wall):soften(60)
work(shade, {hand="scumble", pile=pw, angle=function(x, y) return -0.15 end,
             coverage=0.75, edge="lost", load=0.45, seed=122})
blend(wall, {angle=-0.1, coverage=0.3})

local tbl = below(table_line)
local pool = (ellipse(330, 690, 560, 250) * tbl):soften(50)
work(pool, {hand="scumble", pile=ptl, angle=function(x, y) return 0.02 - 0.1*(x-500)/500 end,
            coverage=1.0, edge="lost", load=0.6, seed=123})
blend(tbl, {angle=0.05, coverage=0.3})
print("light set", wait(0))

--@ chunk 14

pgl = pile{{"raw umber", 1}, {"red earth", 0.35}, {"bone black", 0.18}, medium=0.62}
local s1 = ribbon({{566, 598}, {650, 612}, {742, 626}, {832, 636}, {888, 638}}, 68)
local s2 = ribbon({{318, 674}, {392, 686}, {456, 694}}, 52)
local s3 = ribbon({{206, 708}, {274, 718}, {326, 724}}, 44)
local contact = ellipse(585, 598, 120, 26) + ellipse(330, 674, 90, 21) + ellipse(215, 708, 72, 17)
local shadow = (s1 + s2 + s3 + contact):soften(14)
work(shadow, {hand="glaze", pile=pgl, angle=0.08, coverage=1.2, edge="lost", load=0.7, seed=131})
work((contact):soften(8), {hand="glaze", pile=pgl, angle=0.1, coverage=1.4, edge="lost", load=0.8, seed=132})
blend(shadow:grow(10), {angle=0.1, coverage=0.45})
print("shadows glazed", wait(0), drying(700, 630))

--@ chunk 15

pjs = pile{{"lead white", 1}, {"smalt", 0.8}, {"raw umber", 0.5}, medium=0.2}
pjd = pile{{"smalt", 1}, {"raw umber", 0.9}, {"lead white", 0.4}, medium=0.25}
prf = pile{{"lead white", 1}, {"yellow ochre", 0.28}, {"smalt", 0.18}, medium=0.18}
local jugall = jugm + handlem
local js = body.ellipsoid({585, 480, 0}, {108, 128, 108})
local f = form{{js, dist=0.3}, light={from={-1, -0.7}, front=0.5, ambient=0.25}}
local sh = f:shadow{parts={1}}
local lit = f:lit{parts={1}, soft=0.12}
local edges = f:edges{turn=0.8}

work(sh * jugall, {hand="body", pile=pjs, angle=f:field("fall"), coverage=1.5, clip=true,
                   edge="found", dips={3, 0.85, 0.25}, seed=141})
work(sh * (-edges:grow(14)) * jugall, {hand="body", pile=pjd, angle=f:field("fall"),
                   coverage=1.1, clip=true, edge="soft", dips={3, 0.85, 0.25}, seed=142})
work(edges * sh * jugall, {hand="body", pile=prf, angle=f:field("fall"), coverage=1.0, clip=true,
                   edge="soft", dips={3, 0.8, 0.3}, seed=143})
work(lit * jugall, {hand="body", pile=ph, angle=f:field("fall"), coverage=1.4, clip=true,
                   edge="found", dips={3, 0.9, 0.25}, seed=144})
blend(jugall, {angle=1.5, coverage=0.3})
print("jug rendered", wait(0))

--@ chunk 16

pcal = pile{{"vermilion", 1}, {"chrome yellow", 0.2}, medium=0.2}
local m1 = rect(185, 100, 30, 200)
local m2 = rect(785, 100, 30, 200)
work(m1 + m2, {hand="detail", pile=pcal, clip=true, fill=true, coverage=2.5, seed=151})
print("calibration marks at x=200 and x=800, y=100..300")

--@ chunk 17

pwb = pile{{"raw umber", 1}, {"red earth", 0.35}, {"bone black", 0.35}, medium=0.2}
pwn = pile{{"raw umber", 1}, {"bone black", 0.7}, {"red earth", 0.2}, medium=0.3}
local wallwork = above(table_line) - (jugm + handlem):grow(4)
work(wallwork, {hand="broad", pile=pwb, angle=function(x, y) return -0.1 + 0.12*math.sin(x/230) end,
                coverage=2.2, fill=true, clip=true, edge="found", dips={3, 0.85, 0.3}, seed=161})
blend(wallwork, {angle=-0.1, coverage=0.45})
local glow = (ellipse(120, 40, 620, 420) * wallwork):soften(70)
work(glow, {hand="broad", pile=pwl, angle=function(x, y) return -0.22 + 0.15*math.sin(x/190) end,
            coverage=1.2, fill=true, edge="lost", load=0.65, seed=162})
blend(glow, {angle=-0.15, coverage=0.4})
local shade = (ellipse(1020, 20, 560, 420) * wallwork):soften(70)
work(shade, {hand="broad", pile=pwn, angle=-0.18, coverage=1.1, fill=true, edge="lost",
             load=0.6, seed=163})
blend(shade, {angle=-0.15, coverage=0.35})
print("wall rebuilt", wait(0))

--@ chunk 18

pwb2 = pile{{"raw umber", 1}, {"bone black", 0.6}, {"red earth", 0.3}, medium=0.08}
pwgl = pile{{"yellow ochre", 1}, {"lead white", 0.45}, {"red earth", 0.12}, medium=0.62}
pdgl = pile{{"raw umber", 1}, {"bone black", 0.5}, {"red earth", 0.2}, medium=0.62}
ptgl = pile{{"yellow ochre", 1}, {"lead white", 0.35}, {"red earth", 0.15}, medium=0.62}
pgsh = pile{{"raw umber", 1}, {"red earth", 0.3}, {"bone black", 0.35}, medium=0.62}

local ghost = rect(168, 88, 74, 224) + rect(768, 88, 74, 224)
work(ghost, {hand="body", pile=pwb2, clip=true, fill=true, coverage=3, seed=171})

local obj = (jugm + handlem + lm1 + lm2):grow(8)
local wallwork = above(table_line) - obj
local glow = (ellipse(120, 40, 620, 420) * wallwork):soften(70)
local shade = (ellipse(1020, 20, 560, 420) * wallwork):soften(70)
work(glow, {hand="glaze", pile=pwgl, angle=function(x, y) return -0.2 end, coverage=1.15,
            edge="lost", load=0.75, seed=172})
blend(glow:grow(20), {angle=-0.15, coverage=0.35})
work(shade, {hand="glaze", pile=pdgl, angle=-0.18, coverage=1.05, edge="lost", load=0.75, seed=173})
blend(shade:grow(20), {angle=-0.15, coverage=0.35})

local s1 = ribbon({{566, 598}, {650, 612}, {742, 626}, {832, 636}, {888, 638}}, 68)
local s2 = ribbon({{318, 674}, {392, 686}, {456, 694}}, 52)
local s3 = ribbon({{206, 708}, {274, 718}, {326, 724}}, 44)
local contact = ellipse(585, 598, 120, 26) + ellipse(330, 674, 90, 21) + ellipse(215, 708, 72, 17)
local tblfield = below(table_line) - obj
work(tblfield, {hand="glaze", pile=ptgl, angle=0.05, coverage=1.1, edge="lost", load=0.75, seed=174})
blend(tblfield:grow(16), {angle=0.05, coverage=0.35})
local shad = ((s1 + s2 + s3 + contact):soften(15)) - obj
work(shad, {hand="glaze", pile=pgsh, angle=0.1, coverage=1.2, edge="lost", load=0.8, seed=175})
blend(shad:grow(12), {angle=0.1, coverage=0.4})
print("glazed and unified", wait(0))

--@ chunk 19

pdg2 = pile{{"raw umber", 1}, {"bone black", 0.6}, {"red earth", 0.22}, medium=0.55}
local obj = (jugm + handlem + lm1 + lm2):grow(6)
local wallwork = above(table_line) - obj
work(wallwork, {hand="glaze", pile=pdg2, angle=function(x, y) return -0.18 + 0.1*math.sin(x/210) end,
                coverage=1.35, clip=true, edge="found", load=0.85, seed=181})
work((ellipse(120, 40, 560, 380) * wallwork):soften(60),
     {hand="glaze", pile=pwgl, angle=-0.22, coverage=0.8, edge="lost", load=0.7, seed=182})
local tbl = below(table_line) - obj
work((ellipse(1030, 560, 520, 380) * tbl):soften(60),
     {hand="glaze", pile=pgsh, angle=0.06, coverage=1.0, edge="lost", load=0.7, seed=183})
print("background deepened", wait(0))

--@ chunk 20

pjugsh = pile{{"smalt", 1}, {"raw umber", 0.55}, {"lead white", 0.5}, medium=0.55}
prfg = pile{{"lead white", 1}, {"yellow ochre", 0.3}, {"smalt", 0.15}, medium=0.5}
phh = pile{{"lead white", 1}, {"yellow ochre", 0.06}, medium=0.1}
local jugall = jugm + handlem
local js = body.ellipsoid({585, 480, 0}, {108, 128, 108})
local f = form{{js, dist=0.3}, light={from={-1, -0.7}, front=0.5, ambient=0.25}}
local sh = f:shadow{parts={1}}
local lit = f:lit{parts={1}, soft=0.12}
local edges = f:edges{turn=0.8}

work(sh * jugall, {hand="glaze", pile=pjugsh, angle=1.5, coverage=1.15, clip=true,
                   edge="found", load=0.8, seed=191})
work(sh * (-edges:grow(12)) * jugall, {hand="glaze", pile=pjugsh, angle=1.5, coverage=1.35,
                   clip=true, edge="soft", load=0.85, seed=192})
work(edges * sh * jugall, {hand="glaze", pile=prfg, angle=1.5, coverage=1.0, clip=true,
                   edge="soft", load=0.8, seed=193})
work(lit * jugall, {hand="body", pile=phh, angle=f:field("fall"), coverage=1.5, clip=true,
                   edge="found", dips={3, 0.9, 0.25}, seed=194})
local hl = ellipse(534, 416, 26, 20) + ellipse(566, 336, 15, 10)
work(hl, {hand="detail", pile=phh, clip=true, fill=true, coverage=2.2, seed=195})
print("jug rendered v2", wait(0))

--@ chunk 21

pjugsh = pile{{"smalt", 1}, {"raw umber", 0.6}, {"lead white", 0.35}, medium=0.5}
pcore = pile{{"smalt", 1}, {"raw umber", 1}, {"lead white", 0.18}, medium=0.5}
prfg = pile{{"lead white", 1}, {"yellow ochre", 0.35}, medium=0.5}
phh = pile{{"lead white", 1}, {"yellow ochre", 0.06}, medium=0.1}
local jugall = jugm + handlem
local wm = function(x, y) return (x-585)*0.72 + (y-470)*0.69 end
local shadow = jugall * mask(function(x, y) return smoothstep(-20, 60, wm(x, y)) end)
local core = jugall * mask(function(x, y) return smoothstep(30, 100, wm(x, y)) * (1 - smoothstep(130, 200, wm(x, y))) end)
local refl = jugall * mask(function(x, y) return smoothstep(125, 205, wm(x, y)) end)
local lit = jugall * mask(function(x, y) return 1 - smoothstep(-60, 40, wm(x, y)) end)

work(shadow, {hand="glaze", pile=pjugsh, angle=1.5, coverage=1.25, clip=true, edge="found",
              load=0.85, seed=201})
work(core, {hand="glaze", pile=pcore, angle=1.45, coverage=1.45, clip=true, edge="soft",
            load=0.85, seed=202})
work(refl, {hand="glaze", pile=prfg, angle=1.4, coverage=1.1, clip=true, edge="soft",
            load=0.8, seed=203})
work(lit, {hand="body", pile=phh, angle=1.45, coverage=1.5, clip=true, edge="found",
           dips={3, 0.9, 0.25}, seed=204})
work(ribbon({{552, 322}, {586, 331}, {621, 323}}, 8),
     {hand="detail", pile=pcore, clip=true, fill=true, coverage=1.6, seed=205})
print("jug lit", wait(0))

--@ chunk 22

pdg3 = pile{{"raw umber", 1}, {"bone black", 0.55}, {"red earth", 0.28}, medium=0.58}
pwb3 = pile{{"raw umber", 1}, {"bone black", 0.75}, {"red earth", 0.18}, medium=0.58}
local obj = (jugm + handlem + lm1 + lm2):grow(6)
local wallwork = above(table_line) - obj
work(wallwork, {hand="glaze", pile=pdg3, angle=function(x, y) return -0.18 + 0.1*math.sin(x/210) end,
                coverage=1.15, clip=true, edge="found", load=0.85, seed=211})
work((ellipse(700, 60, 620, 420) * wallwork):soften(70),
     {hand="glaze", pile=pwb3, angle=-0.15, coverage=1.05, edge="lost", load=0.8, seed=212})
work((ellipse(60, 20, 330, 240) * wallwork):soften(60),
     {hand="glaze", pile=pwgl, angle=-0.25, coverage=0.55, edge="lost", load=0.6, seed=213})
print("wall deepened", wait(0))

--@ chunk 23

plh = pile{{"chrome yellow", 1}, {"lead white", 0.45}, medium=0.14}
plm = pile{{"chrome yellow", 1}, {"yellow ochre", 0.5}, {"lead white", 0.1}, medium=0.18}
pls = pile{{"yellow ochre", 1}, {"green earth", 0.45}, {"red earth", 0.25}, medium=0.4}
pld = pile{{"raw umber", 1}, {"green earth", 0.5}, {"yellow ochre", 0.3}, medium=0.4}
local lem = lm1 + lm2
local w1 = function(x, y) return (x-330)*0.72 + (y-632)*0.69 end
local w2 = function(x, y) return (x-212)*0.72 + (y-668)*0.69 end
local sh1 = lm1 * mask(function(x, y) return smoothstep(-15, 55, w1(x, y)) end)
local sh2 = lm2 * mask(function(x, y) return smoothstep(-15, 55, w2(x, y)) end)
local co1 = lm1 * mask(function(x, y) return smoothstep(25, 85, w1(x, y)) * (1 - smoothstep(115, 175, w1(x, y))) end)
local co2 = lm2 * mask(function(x, y) return smoothstep(25, 85, w2(x, y)) * (1 - smoothstep(115, 175, w2(x, y))) end)
local li1 = lm1 * mask(function(x, y) return 1 - smoothstep(-55, 35, w1(x, y)) end)
local li2 = lm2 * mask(function(x, y) return 1 - smoothstep(-55, 35, w2(x, y)) end)

work(sh1 + sh2, {hand="glaze", pile=pls, angle=0.9, coverage=1.3, clip=true, edge="found",
                 load=0.85, seed=221})
work(co1 + co2, {hand="glaze", pile=pld, angle=0.9, coverage=1.35, clip=true, edge="soft",
                 load=0.85, seed=222})
work(li1 + li2, {hand="body", pile=plh, angle=0.95, coverage=1.5, clip=true, edge="found",
                 dips={3, 0.9, 0.25}, seed=223})
work(ellipse(302, 606, 24, 15) + ellipse(190, 646, 18, 12),
     {hand="detail", pile=plh, clip=true, fill=true, coverage=2.0, seed=224})
work(ribbon({{272, 648}, {292, 660}, {312, 668}}, 9),
     {hand="detail", pile=pld, clip=true, fill=true, coverage=1.6, seed=225})
local leaf = poly({{392, 592}, {424, 570}, {462, 572}, {470, 588}, {438, 602}, {406, 602}}, true)
work(leaf, {hand="detail", pile=pld, clip=true, fill=true, coverage=1.8, seed=226})
print("lemons modelled", wait(0))

--@ chunk 24

plh2 = pile{{"chrome yellow", 1}, {"lead white", 0.6}, medium=0.12}
pld2 = pile{{"raw umber", 1}, {"green earth", 0.6}, {"yellow ochre", 0.25}, medium=0.35}
plsp = pile{{"chrome yellow", 1}, {"lead white", 1}, medium=0.1}
plg  = pile{{"green earth", 1}, {"yellow ochre", 0.55}, {"bone black", 0.12}, medium=0.3}
plgl = pile{{"green earth", 1}, {"yellow ochre", 1}, {"lead white", 0.25}, medium=0.25}

local w1 = function(x, y) return (x-330)*0.72 + (y-632)*0.69 end
local w2 = function(x, y) return (x-212)*0.72 + (y-668)*0.69 end
local li1 = lm1 * mask(function(x, y) return 1 - smoothstep(-55, 35, w1(x, y)) end)
local li2 = lm2 * mask(function(x, y) return 1 - smoothstep(-55, 35, w2(x, y)) end)
local co1 = lm1 * mask(function(x, y) return smoothstep(25, 85, w1(x, y)) * (1 - smoothstep(115, 175, w1(x, y))) end)
local co2 = lm2 * mask(function(x, y) return smoothstep(25, 85, w2(x, y)) * (1 - smoothstep(115, 175, w2(x, y))) end)

work(li1 + li2, {hand="body", pile=plh2, angle=0.95, coverage=1.35, clip=true, edge="found",
                 dips={2, 0.9, 0.2}, seed=231})
work(co1 + co2, {hand="glaze", pile=pld2, angle=0.9, coverage=1.3, clip=true, edge="soft",
                 load=0.85, seed=232})
work(ellipse(294, 598, 20, 13) + ellipse(182, 640, 14, 9),
     {hand="detail", pile=plsp, clip=true, fill=true, coverage=2.2, seed=233})
work(ribbon({{258, 652}, {284, 664}, {308, 672}}, 7),
     {hand="detail", pile=pld2, clip=true, fill=true, coverage=1.8, seed=234})
local leaf = poly({{392, 592}, {424, 570}, {462, 572}, {470, 588}, {438, 602}, {406, 602}}, true)
work(leaf, {hand="detail", pile=plg, clip=true, fill=true, coverage=2.0, seed=235})
work(leaf * mask(function(x, y) return 1 - smoothstep(578, 596, y) end),
     {hand="detail", pile=plgl, clip=true, fill=true, coverage=1.6, seed=236})
print("lemons brightened", wait(0))

--@ chunk 25

phh = pile{{"lead white", 1}, {"yellow ochre", 0.05}, medium=0.08}
pcore = pile{{"smalt", 1}, {"raw umber", 1}, {"lead white", 0.18}, medium=0.5}
local hlm = ellipse(524, 402, 22, 15) + ellipse(554, 314, 13, 8) + ellipse(706, 430, 9, 24)
work(hlm, {hand="detail", pile=phh, clip=true, fill=true, coverage=2.4, seed=241})
local handle_sh = ribbon({{694, 522}, {718, 502}, {732, 472}}, 13)
work(handle_sh, {hand="detail", pile=pcore, clip=true, fill=true, coverage=1.6, seed=242})
local base = ribbon({{508, 592}, {562, 606}, {618, 606}, {664, 588}}, 11)
work(base, {hand="detail", pile=pcore, clip=true, fill=true, coverage=1.5, seed=243})
local hl2 = ribbon({{700, 428}, {722, 448}, {730, 476}}, 7)
work(hl2, {hand="detail", pile=phh, clip=true, fill=true, coverage=1.8, seed=244})
print("jug accented", wait(0))

--@ chunk 26

local obj = (jugm + handlem + lm1 + lm2):grow(6)
local wallwork = above(table_line) - obj
work((ellipse(200, 120, 520, 380) * wallwork):soften(70),
     {hand="glaze", pile=pdg3, angle=-0.2, coverage=0.85, edge="lost", load=0.7, seed=251})
local tbl = below(table_line) - obj
work((ellipse(280, 730, 520, 230) * tbl):soften(50),
     {hand="glaze", pile=ptgl, angle=0.03, coverage=0.85, edge="lost", load=0.7, seed=252})
work((ellipse(1010, 780, 420, 260) * tbl):soften(50),
     {hand="glaze", pile=pgsh, angle=0.05, coverage=0.8, edge="lost", load=0.7, seed=253})
print("field balanced", wait(0))

--@ chunk 27

local obj = jugm + handlem
local band = (obj:grow(26) - obj) 
work(band * above(table_line), {hand="detail", pile=pdg3, clip=true, fill=true, coverage=2.0,
                                edge="found", seed=261})
work(band * below(table_line), {hand="detail", pile=pgsh, clip=true, fill=true, coverage=1.8,
                                edge="found", seed=262})
local lband = ((lm1 + lm2):grow(22) - (lm1 + lm2))
work(lband, {hand="detail", pile=ptgl, clip=true, fill=true, coverage=1.8, edge="found", seed=263})
work(lband * mask(function(x, y) return smoothstep(640, 690, y) end),
     {hand="detail", pile=pgsh, clip=true, fill=true, coverage=1.6, edge="found", seed=264})
local neckcut = rect(430, 280, 340, 90) - ellipse(585, 356, 42, 60) - ellipse(585, 312, 50, 22)
work(neckcut, {hand="detail", pile=pdg3, clip=true, fill=true, coverage=2.0, seed=265})
print("edges cut in", wait(0))

--@ chunk 28

pterm = pile{{"lead white", 1}, {"smalt", 0.3}, {"raw umber", 0.25}, medium=0.42}
local term = ribbon({{588, 358}, {628, 420}, {656, 482}, {672, 542}}, 36) * (jugm + handlem)
work(term, {hand="scumble", pile=pterm, angle=1.5, coverage=0.75, clip=true, edge="soft",
            load=0.5, seed=271})
work(ellipse(585, 642, 155, 32):soften(22),
     {hand="glaze", pile=ptgl, angle=0.05, coverage=0.7, edge="lost", load=0.6, seed=272})
work(ellipse(300, 700, 150, 30):soften(22),
     {hand="glaze", pile=ptgl, angle=0.05, coverage=0.6, edge="lost", load=0.6, seed=273})
print("terminator softened", wait(0))

--@ chunk 29

local obj = (jugm + handlem + lm1 + lm2):grow(6)
local wallwork = above(table_line) - obj
work((ellipse(180, 120, 480, 360) * wallwork):soften(60),
     {hand="glaze", pile=pdg3, angle=-0.22, coverage=0.9, edge="lost", load=0.75, seed=281})
work((ellipse(0, 60, 300, 300) * wallwork):soften(50),
     {hand="glaze", pile=pwgl, angle=-0.28, coverage=0.45, edge="lost", load=0.6, seed=282})
print("wall settled", wait(0))

--@ chunk 30
print("now:", wait(0))
print("wall:", drying(300, 250), "| near halo:", drying(460, 420), "| table:", drying(500, 680), "| jug:", drying(560, 450), "| lemon:", drying(330, 632))

--@ chunk 31
print("now:", wait(2*24*60))
print("wall:", drying(300, 250), "| halo:", drying(460, 420), "| table:", drying(500, 680), "| jug:", drying(560, 450), "| lemon:", drying(330, 632))

--@ chunk 32
print("now:", wait(4*24*60))
print("wall:", drying(300, 250), "| halo:", drying(460, 420), "| table:", drying(500, 680), "| jug:", drying(560, 450), "| lemon:", drying(330, 632))

--@ chunk 33
print("now:", wait(12*24*60))
print("wall:", drying(300, 250), "| halo:", drying(460, 420), "| table:", drying(500, 680), "| jug:", drying(560, 450), "| lemon:", drying(330, 632), "| wall2:", drying(850, 300))

--@ chunk 34
pw_mid = pile{{"raw umber", 1}, {"red earth", 0.3}, {"bone black", 0.42}, {"lead white", 0.06}, medium=0.15}
wallf = above(table_line) - (jugm + handlem):grow(1.2)
work(wallf, {hand="broad", pile=pw_mid, angle=function(x, y) return -0.06 + 0.05*math.sin(x/300) end,
             coverage=1.7, fill=true, clip=true, edge="found", dips={3, 0.85, 0.25}, seed=301})
print("wall repainted", wait(0))

--@ chunk 35
pt_mid = pile{{"yellow ochre", 1}, {"red earth", 0.26}, {"lead white", 0.42}, {"bone black", 0.1}, medium=0.15}
tblf = below(table_line) - (jugm + handlem + lm1 + lm2):grow(0.8)
work(tblf, {hand="broad", pile=pt_mid, angle=function(x, y) return 0.04 - 0.09*(x-500)/500 end,
            coverage=1.7, fill=true, clip=true, edge="found", dips={3, 0.85, 0.25}, seed=311})
print("table repainted", wait(0))

--@ chunk 36
pt_light = pile{{"lead white", 1}, {"yellow ochre", 0.55}, {"red earth", 0.1}, medium=0.18}
pt_dark = pile{{"raw umber", 1}, {"red earth", 0.42}, {"bone black", 0.2}, {"lead white", 0.12}, medium=0.2}
local glow = (ellipse(300, 730, 520, 220) * tblf):soften(45)
work(glow, {hand="broad", pile=pt_light, angle=function(x, y) return 0.03 - 0.08*(x-500)/500 end,
            coverage=1.0, fill=true, clip=tblf, edge="lost", load=0.7, seed=312})
local back = (ellipse(520, 545, 620, 120) * tblf):soften(40)
work(back, {hand="broad", pile=pt_dark, angle=function(x, y) return 0.05 - 0.06*(x-500)/500 end,
            coverage=1.0, fill=true, clip=tblf, edge="lost", load=0.7, seed=313})
local right = (ellipse(1010, 640, 480, 330) * tblf):soften(50)
work(right, {hand="broad", pile=pt_dark, angle=function(x, y) return 0.06 - 0.07*(x-500)/500 end,
             coverage=1.15, fill=true, clip=tblf, edge="lost", load=0.7, seed=314})
blend(tblf, {angle=0.03, coverage=0.5})
print("table modelled", wait(0))

--@ chunk 37
pt_cover = pile{{"lead white", 1}, {"yellow ochre", 0.85}, {"red earth", 0.12}, medium=0.1}
local gj = (ellipse(618, 588, 250, 125) * tblf):soften(35)
work(gj, {hand="broad", pile=pt_cover, angle=function(x, y) return 0.05 - 0.08*(x-500)/500 end,
          coverage=1.5, fill=true, clip=tblf, edge="lost", load=0.8, seed=321})
local gl = (ellipse(300, 662, 290, 155) * tblf):soften(35)
work(gl, {hand="broad", pile=pt_cover, angle=function(x, y) return 0.06 - 0.08*(x-500)/500 end,
          coverage=1.5, fill=true, clip=tblf, edge="lost", load=0.8, seed=322})
blend((gj + gl), {angle=0.05, coverage=0.6})
print("halo ghosts knocked back", wait(0))

--@ chunk 38
pt_light = pile{{"lead white", 1}, {"yellow ochre", 0.55}, {"red earth", 0.1}, medium=0.16}
pt_dark = pile{{"raw umber", 1}, {"red earth", 0.42}, {"bone black", 0.22}, {"lead white", 0.14}, medium=0.18}
local back = (ellipse(520, 535, 640, 95) * tblf):soften(35)
work(back, {hand="broad", pile=pt_dark, angle=function(x, y) return 0.05 - 0.06*(x-500)/500 end,
            coverage=1.1, fill=true, clip=tblf, edge="lost", load=0.7, seed=331})
local right = (ellipse(1030, 630, 460, 320) * tblf):soften(55)
work(right, {hand="broad", pile=pt_dark, angle=function(x, y) return 0.06 - 0.07*(x-500)/500 end,
             coverage=1.2, fill=true, clip=tblf, edge="lost", load=0.7, seed=332})
local pool = (ellipse(290, 725, 500, 195) * tblf):soften(45)
work(pool, {hand="broad", pile=pt_light, angle=function(x, y) return 0.03 - 0.08*(x-500)/500 end,
            coverage=1.15, fill=true, clip=tblf, edge="lost", load=0.75, seed=333})
blend(tblf, {angle=0.03, coverage=0.45})
print("table light set", wait(0))

--@ chunk 39
psh_g = pile{{"raw umber", 1}, {"red earth", 0.35}, {"bone black", 0.25}, medium=0.5}
psh_c = pile{{"raw umber", 1}, {"bone black", 0.55}, {"red earth", 0.3}, medium=0.45}
local s1 = ribbon({{600, 601}, {680, 613}, {770, 623}, {860, 630}, {935, 633}}, 58)
local s2 = ribbon({{345, 677}, {405, 687}, {468, 693}}, 44)
local s3 = ribbon({{228, 709}, {285, 718}, {336, 723}}, 38)
local shadow = (s1 + s2 + s3):soften(13)
work(shadow, {hand="glaze", pile=psh_g, angle=0.12, coverage=1.15, clip=tblf, edge="lost",
              load=0.75, seed=341})
local contact = (ellipse(585, 597, 118, 22) + ellipse(330, 674, 86, 18) + ellipse(215, 706, 68, 15)):soften(9)
work(contact, {hand="glaze", pile=psh_c, angle=0.1, coverage=1.35, clip=tblf, edge="lost",
               load=0.85, seed=342})
blend((shadow + contact):grow(10), {angle=0.12, coverage=0.5})
print("cast shadows laid", wait(0))

--@ chunk 40
pgl_t = pile{{"yellow ochre", 1}, {"red earth", 0.3}, {"lead white", 0.22}, medium=0.6}
pgl_d = pile{{"raw umber", 1}, {"red earth", 0.4}, {"bone black", 0.28}, medium=0.62}
local right = (ellipse(1010, 640, 500, 340) * tblf):soften(60)
work(right, {hand="glaze", pile=pgl_d, angle=0.06, coverage=1.25, clip=tblf, edge="lost",
             load=0.8, seed=351})
local mid = (ellipse(420, 680, 620, 240) * tblf):soften(55)
work(mid, {hand="glaze", pile=pgl_t, angle=0.04, coverage=1.0, clip=tblf, edge="lost",
           load=0.8, seed=352})
blend(tblf, {angle=0.03, coverage=0.55})
print("table glazed", wait(0))

--@ chunk 41
pl_mid = pile{{"chrome yellow", 1}, {"yellow ochre", 0.55}, {"lead white", 0.12}, medium=0.15}
pl_lit = pile{{"chrome yellow", 1}, {"lead white", 0.5}, {"yellow ochre", 0.2}, medium=0.12}
pl_sh = pile{{"yellow ochre", 1}, {"green earth", 0.45}, {"red earth", 0.25}, medium=0.22}
pl_core = pile{{"raw umber", 1}, {"green earth", 0.5}, {"yellow ochre", 0.3}, medium=0.35}
local lem = lm1 + lm2
work(lem, {hand="body", pile=pl_mid, angle=1.0, coverage=1.6, fill=true, clip=true,
           edge="found", dips={3, 0.85, 0.25}, seed=361})
local w1 = function(x, y) return (x-330)*0.72 + (y-632)*0.69 end
local w2 = function(x, y) return (x-212)*0.72 + (y-668)*0.69 end
local sh1 = lm1 * mask(function(x, y) return smoothstep(-10, 60, w1(x, y)) end)
local sh2 = lm2 * mask(function(x, y) return smoothstep(-10, 60, w2(x, y)) end)
work(sh1 + sh2, {hand="body", pile=pl_sh, angle=1.0, coverage=1.25, clip=true, edge="soft",
                 dips={3, 0.8, 0.3}, seed=362})
local co1 = lm1 * mask(function(x, y) return smoothstep(35, 95, w1(x, y)) * (1 - smoothstep(125, 185, w1(x, y))) end)
local co2 = lm2 * mask(function(x, y) return smoothstep(35, 95, w2(x, y)) * (1 - smoothstep(125, 185, w2(x, y))) end)
work(co1 + co2, {hand="glaze", pile=pl_core, angle=1.0, coverage=1.2, clip=true, edge="soft",
                 load=0.8, seed=363})
local li1 = lm1 * mask(function(x, y) return 1 - smoothstep(-45, 45, w1(x, y)) end)
local li2 = lm2 * mask(function(x, y) return 1 - smoothstep(-45, 45, w2(x, y)) end)
work(li1 + li2, {hand="body", pile=pl_lit, angle=1.05, coverage=1.35, clip=true, edge="found",
                 dips={3, 0.9, 0.25}, seed=364})
print("lemons repainted", wait(0))

--@ chunk 42
pl_spec = pile{{"chrome yellow", 1}, {"lead white", 1}, medium=0.1}
pl_crease = pile{{"raw umber", 1}, {"green earth", 0.6}, {"yellow ochre", 0.3}, medium=0.4}
plg = pile{{"green earth", 1}, {"yellow ochre", 0.55}, {"bone black", 0.12}, medium=0.3}
plgl = pile{{"green earth", 1}, {"yellow ochre", 1}, {"lead white", 0.25}, medium=0.25}
work(ribbon({{272, 618}, {284, 640}, {292, 664}}, 10),
     {hand="detail", pile=pl_crease, clip=lm2, fill=true, coverage=1.7, seed=371})
work(ribbon({{318, 688}, {352, 696}, {386, 700}}, 12),
     {hand="detail", pile=pl_crease, clip=lm1, fill=true, coverage=1.4, seed=372})
work(ellipse(302, 604, 22, 13) + ellipse(188, 644, 15, 9),
     {hand="detail", pile=pl_spec, clip=true, fill=true, coverage=2.2, seed=373})
work(ellipse(272, 618, 9, 6),
     {hand="detail", pile=pl_spec, clip=true, fill=true, coverage=1.8, seed=374})
local leaf = poly({{386, 598}, {412, 570}, {450, 566}, {474, 578}, {456, 598}, {414, 606}}, true)
work(leaf, {hand="detail", pile=plg, clip=true, fill=true, coverage=1.9, seed=375})
work(leaf * mask(function(x, y) return 1 - smoothstep(576, 594, y) end),
     {hand="detail", pile=plgl, clip=true, fill=true, coverage=1.6, seed=376})
work(ribbon({{392, 596}, {428, 582}, {464, 576}}, 5),
     {hand="detail", pile=plgl, clip=true, fill=true, coverage=1.5, seed=377})
print("lemon accents and leaf", wait(0))

--@ chunk 43
pl_sh2 = pile{{"yellow ochre", 1}, {"chrome yellow", 0.35}, {"green earth", 0.2}, {"lead white", 0.15}, medium=0.12}
pl_core2 = pile{{"raw umber", 1}, {"green earth", 0.5}, {"yellow ochre", 0.35}, {"lead white", 0.1}, medium=0.25}
local w1 = function(x, y) return (x-330)*0.72 + (y-632)*0.69 end
local w2 = function(x, y) return (x-212)*0.72 + (y-668)*0.69 end
local sh1 = lm1 * mask(function(x, y) return smoothstep(0, 55, w1(x, y)) end)
local sh2 = lm2 * mask(function(x, y) return smoothstep(0, 55, w2(x, y)) end)
work(sh1 + sh2, {hand="body", pile=pl_sh2, angle=1.0, coverage=2.2, fill=true, clip=true,
                 edge="found", dips={3, 0.85, 0.2}, seed=381})
local co1 = lm1 * mask(function(x, y) return smoothstep(45, 95, w1(x, y)) * (1 - smoothstep(135, 185, w1(x, y))) end)
local co2 = lm2 * mask(function(x, y) return smoothstep(45, 95, w2(x, y)) * (1 - smoothstep(135, 185, w2(x, y))) end)
work(co1 + co2, {hand="body", pile=pl_core2, angle=1.0, coverage=1.5, fill=true, clip=true,
                 edge="soft", dips={3, 0.8, 0.25}, seed=382})
print("lemon shadows solid", wait(0))

--@ chunk 44
pt_op = pile{{"yellow ochre", 1}, {"lead white", 0.5}, {"red earth", 0.28}, {"bone black", 0.06}, medium=0.1}
work(tblf, {hand="broad", pile=pt_op, angle=function(x, y) return 0.03 - 0.07*(x-500)/500 end,
            coverage=2.0, fill=true, clip=true, edge="found", dips={3, 0.85, 0.2}, seed=391})
print("table laid opaque", wait(0))

--@ chunk 45
pt_lt2 = pile{{"lead white", 1}, {"yellow ochre", 0.5}, {"red earth", 0.08}, medium=0.12}
pt_dk2 = pile{{"raw umber", 1}, {"red earth", 0.5}, {"lead white", 0.22}, {"bone black", 0.12}, medium=0.14}
local back = (ellipse(520, 532, 640, 85) * tblf):soften(30)
work(back, {hand="broad", pile=pt_dk2, angle=function(x, y) return 0.05 - 0.06*(x-500)/500 end,
            coverage=1.2, fill=true, clip=tblf, edge="lost", load=0.75, seed=401})
local right = (ellipse(1040, 620, 430, 300) * tblf):soften(50)
work(right, {hand="broad", pile=pt_dk2, angle=function(x, y) return 0.06 - 0.07*(x-500)/500 end,
             coverage=1.35, fill=true, clip=tblf, edge="lost", load=0.75, seed=402})
local pool = (ellipse(300, 730, 480, 185) * tblf):soften(40)
work(pool, {hand="broad", pile=pt_lt2, angle=function(x, y) return 0.03 - 0.08*(x-500)/500 end,
            coverage=1.25, fill=true, clip=tblf, edge="lost", load=0.8, seed=403})
blend(tblf, {angle=0.03, coverage=0.5})
print("table modelled v2", wait(0))

--@ chunk 46
psh_g = pile{{"raw umber", 1}, {"red earth", 0.45}, {"bone black", 0.22}, medium=0.5}
psh_c = pile{{"raw umber", 1}, {"red earth", 0.5}, {"bone black", 0.35}, medium=0.42}
local s1 = ribbon({{602, 602}, {684, 614}, {772, 624}, {858, 631}, {930, 634}}, 56)
local s2 = ribbon({{348, 678}, {408, 688}, {466, 694}}, 42)
local s3 = ribbon({{230, 710}, {284, 719}, {332, 724}}, 36)
local shadow = (s1 + s2 + s3):soften(12)
work(shadow, {hand="glaze", pile=psh_g, angle=0.12, coverage=1.1, clip=tblf, edge="lost",
              load=0.75, seed=411})
local contact = (ellipse(585, 597, 116, 21) + ellipse(330, 674, 84, 17) + ellipse(215, 706, 66, 14)):soften(8)
work(contact, {hand="glaze", pile=psh_c, angle=0.1, coverage=1.3, clip=tblf, edge="lost",
               load=0.85, seed=412})
blend((shadow + contact) * tblf, {angle=0.12, coverage=0.5})
print("cast shadows v2", wait(0))

--@ chunk 47
pl_base = pile{{"chrome yellow", 1}, {"yellow ochre", 0.45}, {"lead white", 0.12}, medium=0.12}
pl_lit2 = pile{{"chrome yellow", 1}, {"lead white", 0.45}, {"yellow ochre", 0.15}, medium=0.1}
pl_warm = pile{{"yellow ochre", 1}, {"red earth", 0.35}, {"chrome yellow", 0.3}, medium=0.15}
pl_dark2 = pile{{"raw umber", 1}, {"red earth", 0.6}, {"yellow ochre", 0.4}, medium=0.22}
pl_refl2 = pile{{"yellow ochre", 1}, {"lead white", 0.3}, {"red earth", 0.15}, medium=0.16}
local lem = lm1 + lm2
local w1 = function(x, y) return (x-330)*0.72 + (y-632)*0.69 end
local w2 = function(x, y) return (x-212)*0.72 + (y-668)*0.69 end
work(lem, {hand="body", pile=pl_base, angle=1.0, coverage=1.8, fill=true, clip=true,
           edge="found", dips={3, 0.9, 0.2}, seed=421})
local sh = lm1 * mask(function(x, y) return smoothstep(5, 60, w1(x, y)) end) + lm2 * mask(function(x, y) return smoothstep(5, 60, w2(x, y)) end)
work(sh, {hand="body", pile=pl_warm, angle=1.0, coverage=1.6, fill=true, clip=true,
          edge="soft", dips={3, 0.85, 0.2}, seed=422})
local co = lm1 * mask(function(x, y) return smoothstep(50, 100, w1(x, y)) * (1 - smoothstep(135, 180, w1(x, y))) end) + lm2 * mask(function(x, y) return smoothstep(50, 100, w2(x, y)) * (1 - smoothstep(135, 180, w2(x, y))) end)
work(co, {hand="body", pile=pl_dark2, angle=1.0, coverage=1.4, fill=true, clip=true,
          edge="soft", dips={3, 0.8, 0.25}, seed=423})
local rf = lm1 * mask(function(x, y) return smoothstep(135, 185, w1(x, y)) end) + lm2 * mask(function(x, y) return smoothstep(135, 185, w2(x, y)) end)
work(rf, {hand="detail", pile=pl_refl2, clip=true, fill=true, coverage=1.6, seed=424})
local li = lm1 * mask(function(x, y) return 1 - smoothstep(-55, 40, w1(x, y)) end) + lm2 * mask(function(x, y) return 1 - smoothstep(-55, 40, w2(x, y)) end)
work(li, {hand="body", pile=pl_lit2, angle=1.05, coverage=1.5, fill=true, clip=true,
          edge="found", dips={3, 0.9, 0.2}, seed=425})
blend(lem, {angle=1.0, coverage=0.35})
print("lemons rebuilt", wait(0))

--@ chunk 48
pl_spec = pile{{"chrome yellow", 1}, {"lead white", 1}, medium=0.1}
pl_crease = pile{{"raw umber", 1}, {"red earth", 0.5}, {"yellow ochre", 0.3}, medium=0.35}
plg2 = pile{{"green earth", 1}, {"yellow ochre", 0.6}, {"red earth", 0.1}, medium=0.28}
plgl2 = pile{{"green earth", 1}, {"yellow ochre", 1}, {"lead white", 0.3}, medium=0.22}
work(ribbon({{274, 616}, {286, 640}, {294, 666}}, 9),
     {hand="detail", pile=pl_crease, clip=lm2, fill=true, coverage=1.6, seed=431})
work(ribbon({{322, 690}, {354, 697}, {384, 700}}, 10),
     {hand="detail", pile=pl_crease, clip=lm1, fill=true, coverage=1.3, seed=432})
work(ellipse(304, 606, 20, 12) + ellipse(190, 646, 13, 8),
     {hand="detail", pile=pl_spec, clip=true, fill=true, coverage=2.2, seed=433})
work(ellipse(270, 616, 7, 5),
     {hand="detail", pile=pl_spec, clip=true, fill=true, coverage=1.8, seed=434})
local leaf = poly({{386, 598}, {412, 570}, {450, 566}, {474, 578}, {456, 598}, {414, 606}}, true)
work(leaf, {hand="detail", pile=plg2, clip=true, fill=true, coverage=1.9, seed=435})
work(leaf * mask(function(x, y) return 1 - smoothstep(576, 594, y) end),
     {hand="detail", pile=plgl2, clip=true, fill=true, coverage=1.6, seed=436})
work(ribbon({{392, 596}, {428, 582}, {464, 576}}, 4),
     {hand="detail", pile=plgl2, clip=true, fill=true, coverage=1.5, seed=437})
print("lemon accents v2", wait(0))

--@ chunk 49
pj_base = pile{{"lead white", 1}, {"yellow ochre", 0.1}, medium=0.12}
pj_lit = pile{{"lead white", 1}, {"yellow ochre", 0.08}, medium=0.1}
pj_sh = pile{{"lead white", 1}, {"smalt", 0.55}, {"raw umber", 0.3}, medium=0.18}
pj_core = pile{{"smalt", 1}, {"raw umber", 0.9}, {"lead white", 0.4}, medium=0.28}
pj_refl = pile{{"lead white", 1}, {"yellow ochre", 0.35}, {"smalt", 0.12}, medium=0.18}
local jugall = jugm + handlem
local wm = function(x, y) return (x-585)*0.72 + (y-470)*0.69 end
work(jugall, {hand="body", pile=pj_base, angle=1.5, coverage=1.9, fill=true, clip=true,
              edge="found", dips={3, 0.9, 0.2}, seed=441})
local sh = jugall * mask(function(x, y) return smoothstep(-15, 55, wm(x, y)) end)
work(sh, {hand="body", pile=pj_sh, angle=1.5, coverage=1.5, fill=true, clip=true,
          edge="soft", dips={3, 0.85, 0.2}, seed=442})
local co = jugall * mask(function(x, y) return smoothstep(40, 95, wm(x, y)) * (1 - smoothstep(135, 195, wm(x, y))) end)
work(co, {hand="body", pile=pj_core, angle=1.5, coverage=1.4, fill=true, clip=true,
          edge="soft", dips={3, 0.8, 0.25}, seed=443})
local rf = jugall * mask(function(x, y) return smoothstep(140, 200, wm(x, y)) end)
work(rf, {hand="detail", pile=pj_refl, clip=true, fill=true, coverage=1.7, seed=444})
local li = jugall * mask(function(x, y) return 1 - smoothstep(-55, 45, wm(x, y)) end)
work(li, {hand="body", pile=pj_lit, angle=1.5, coverage=1.5, fill=true, clip=true,
          edge="found", dips={3, 0.9, 0.2}, seed=445})
blend(jugall, {angle=1.5, coverage=0.3})
print("jug body rebuilt", wait(0))

--@ chunk 50
pdark_m = pile{{"raw umber", 1}, {"bone black", 0.6}, {"red earth", 0.2}, medium=0.3}
work(ellipse(585, 311, 34, 9), {hand="detail", pile=pdark_m, clip=true, fill=true, coverage=2.0, seed=451})
local ring = ellipse(585, 307, 45, 15) - ellipse(585, 311, 35, 10)
local lip_up = ring * mask(function(x, y) return 1 - smoothstep(302, 313, y) end)
work(lip_up, {hand="detail", pile=pj_lit, clip=true, fill=true, coverage=1.8, seed=452})
local lip_dn = ring * mask(function(x, y) return smoothstep(308, 317, y) end)
work(lip_dn, {hand="detail", pile=pj_sh, clip=true, fill=true, coverage=1.6, seed=453})
local neck_sh = jugm * mask(function(x, y) return smoothstep(592, 620, x) * (1 - smoothstep(346, 362, y)) end)
work(neck_sh, {hand="detail", pile=pj_sh, clip=true, fill=true, coverage=1.5, seed=454})
local under_lip = (rect(552, 318, 68, 13)):soften(6) * jugm
work(under_lip, {hand="detail", pile=pj_sh, clip=true, fill=true, coverage=1.1, seed=455})
work(ellipse(553, 318, 7, 11) + ellipse(538, 412, 20, 26),
     {hand="detail", pile=pj_lit, clip=true, fill=true, coverage=1.8, seed=456})
print("rim and neck", wait(0))

--@ chunk 51
newh = ribbon({{666, 424}, {714, 440}, {736, 476}, {724, 518}, {686, 542}}, 30)
work(newh, {hand="body", pile=pj_base, angle=1.2, coverage=2.1, fill=true, clip=true,
            edge="found", dips={3, 0.9, 0.2}, seed=461})
local wm = function(x, y) return (x-585)*0.72 + (y-470)*0.69 end
work(newh * mask(function(x, y) return smoothstep(5, 45, wm(x, y)) end),
     {hand="body", pile=pj_sh, angle=1.2, coverage=1.5, fill=true, clip=true, edge="soft",
      dips={3, 0.85, 0.2}, seed=462})
work(newh * mask(function(x, y) return 1 - smoothstep(-25, 20, wm(x, y)) end),
     {hand="body", pile=pj_lit, angle=1.2, coverage=1.3, fill=true, clip=true, edge="soft",
      dips={3, 0.9, 0.2}, seed=463})
work(ribbon({{672, 432}, {704, 452}, {718, 478}, {708, 506}, {682, 534}}, 5) * newh,
     {hand="detail", pile=pj_core, clip=true, fill=true, coverage=1.3, seed=464})
work(ribbon({{676, 424}, {712, 436}, {732, 468}}, 6) * newh,
     {hand="detail", pile=pj_lit, clip=true, fill=true, coverage=1.5, seed=465})
print("handle rebuilt", wait(0))

--@ chunk 52
pgl_uni = pile{{"yellow ochre", 1}, {"red earth", 0.25}, {"lead white", 0.18}, medium=0.55}
pgl_fr = pile{{"raw umber", 1}, {"red earth", 0.45}, {"bone black", 0.18}, medium=0.6}
work(tblf, {hand="glaze", pile=pgl_uni, angle=0.04, coverage=0.9, clip=true, edge="found",
            load=0.8, seed=471})
local front = (rect(0, 726, 1000, 74):soften(28) * tblf)
work(front, {hand="glaze", pile=pgl_fr, angle=0.02, coverage=1.0, clip=tblf, edge="lost",
             load=0.75, seed=472})
local right = (ellipse(1020, 620, 420, 300) * tblf):soften(55)
work(right, {hand="glaze", pile=pgl_fr, angle=0.05, coverage=0.9, clip=tblf, edge="lost",
             load=0.75, seed=473})
blend(tblf, {angle=0.03, coverage=0.45})
print("table settled", wait(0))

--@ chunk 53
psh_g = pile{{"raw umber", 1}, {"red earth", 0.45}, {"bone black", 0.22}, medium=0.5}
psh_c = pile{{"raw umber", 1}, {"red earth", 0.5}, {"bone black", 0.35}, medium=0.42}
local s1 = ribbon({{602, 602}, {684, 614}, {772, 624}, {858, 631}, {930, 634}}, 54)
local s2 = ribbon({{348, 678}, {408, 688}, {466, 694}}, 40)
local s3 = ribbon({{230, 710}, {284, 719}, {332, 724}}, 34)
local shadow = (s1 + s2 + s3):soften(11)
work(shadow, {hand="glaze", pile=psh_g, angle=0.12, coverage=1.15, clip=tblf, edge="lost",
              load=0.75, seed=481})
local contact = (ellipse(585, 597, 114, 20) + ellipse(330, 674, 82, 16) + ellipse(215, 706, 64, 13)):soften(7)
work(contact, {hand="glaze", pile=psh_c, angle=0.1, coverage=1.3, clip=tblf, edge="lost",
               load=0.85, seed=482})
blend((shadow + contact) * tblf, {angle=0.12, coverage=0.45})
print("shadows restated", wait(0))

--@ chunk 54
pj_sh2 = pile{{"lead white", 1}, {"smalt", 0.45}, {"raw umber", 0.22}, medium=0.16}
pj_core2 = pile{{"smalt", 1}, {"raw umber", 0.8}, {"lead white", 0.55}, medium=0.22}
pj_refl2 = pile{{"lead white", 1}, {"yellow ochre", 0.4}, {"smalt", 0.1}, medium=0.16}
local jugall = jugm + handlem
local wm = function(x, y) return (x-585)*0.72 + (y-470)*0.69 end
local sh = jugall * mask(function(x, y) return smoothstep(-10, 60, wm(x, y)) end)
work(sh, {hand="broad", pile=pj_sh2, angle=1.35, coverage=1.4, fill=true, clip=true,
          edge="soft", load=0.75, seed=491})
local co = jugall * mask(function(x, y) return smoothstep(85, 120, wm(x, y)) * (1 - smoothstep(155, 190, wm(x, y))) end)
work(co, {hand="body", pile=pj_core2, angle=1.3, coverage=1.2, fill=true, clip=true,
          edge="soft", dips={3, 0.8, 0.25}, seed=492})
local rf = jugall * mask(function(x, y) return smoothstep(155, 195, wm(x, y)) end)
work(rf, {hand="detail", pile=pj_refl2, clip=true, fill=true, coverage=1.6, seed=493})
blend(jugall * mask(function(x, y) return smoothstep(-30, 20, wm(x, y)) end),
      {angle=1.4, coverage=0.5})
print("jug shadow side smoothed", wait(0))

--@ chunk 55
pj_lit3 = pile{{"lead white", 1}, {"yellow ochre", 0.09}, medium=0.1}
pj_sh3 = pile{{"lead white", 1}, {"smalt", 0.32}, {"raw umber", 0.15}, medium=0.15}
pj_core3 = pile{{"lead white", 1}, {"smalt", 0.7}, {"raw umber", 0.35}, medium=0.2}
pj_refl3 = pile{{"lead white", 1}, {"yellow ochre", 0.42}, {"smalt", 0.1}, medium=0.15}
local jugall = jugm + handlem
local wm = function(x, y) return (x-585)*0.72 + (y-470)*0.69 end
local mid = jugall * mask(function(x, y) return 1 - smoothstep(35, 85, wm(x, y)) end)
work(mid, {hand="broad", pile=pj_lit3, angle=1.35, coverage=1.25, fill=true, clip=true,
           edge="soft", load=0.8, seed=501})
local sh = jugall * mask(function(x, y) return smoothstep(60, 105, wm(x, y)) end)
work(sh, {hand="broad", pile=pj_sh3, angle=1.35, coverage=1.35, fill=true, clip=true,
          edge="soft", load=0.8, seed=502})
local co = jugall * mask(function(x, y) return smoothstep(105, 135, wm(x, y)) * (1 - smoothstep(150, 180, wm(x, y))) end)
work(co, {hand="body", pile=pj_core3, angle=1.3, coverage=1.15, fill=true, clip=true,
          edge="soft", dips={3, 0.8, 0.25}, seed=503})
local rf = jugall * mask(function(x, y) return smoothstep(150, 185, wm(x, y)) end)
work(rf, {hand="detail", pile=pj_refl3, clip=true, fill=true, coverage=1.5, seed=504})
blend(jugall * mask(function(x, y) return smoothstep(-20, 30, wm(x, y)) end),
      {angle=1.4, coverage=0.45})
print("jug shadow narrowed", wait(0))

--@ chunk 56
work(((ellipse(585, 306, 50, 18) - ellipse(585, 307, 40, 12)) * above(table_line)) - jugm,
     {hand="detail", pile=pw_mid, clip=true, fill=true, coverage=2.2, seed=511})
local lip = (ellipse(585, 307, 40, 12) - ellipse(585, 311, 31, 8)) * jugm
work(lip * mask(function(x, y) return 1 - smoothstep(304, 312, y) end),
     {hand="detail", pile=pj_lit, clip=true, fill=true, coverage=1.8, seed=512})
work(lip * mask(function(x, y) return smoothstep(308, 315, y) end),
     {hand="detail", pile=pj_sh2, clip=true, fill=true, coverage=1.5, seed=513})
work(ellipse(585, 313, 30, 7), {hand="detail", pile=pdark_m, clip=true, fill=true, coverage=2.0, seed=514})
local fr1 = ((newh:grow(12) - newh:shrink(1)) * above(table_line)) - jugm:grow(1)
work(fr1, {hand="detail", pile=pw_mid, clip=true, fill=true, coverage=2.1, seed=515})
local fr2 = ((newh:grow(12) - newh:shrink(1)) * below(table_line)) - jugm:grow(1)
work(fr2, {hand="detail", pile=pt_op, clip=true, fill=true, coverage=2.1, seed=516})
print("rim and handle contour cleaned", wait(0))

--@ chunk 57
local hregion = newh:grow(14) - jugm:grow(1)
work(hregion * above(table_line), {hand="detail", pile=pw_mid, clip=true, fill=true, coverage=2.4, seed=521})
work(hregion * below(table_line), {hand="detail", pile=pt_op, clip=true, fill=true, coverage=2.4, seed=522})
blend(hregion * below(table_line), {angle=0.1, coverage=0.6})
print("handle erased to background", wait(0))

--@ chunk 58
newh2 = ribbon({{668, 422}, {712, 434}, {734, 470}, {727, 512}, {688, 544}}, 22)
work(newh2, {hand="body", pile=pj_base, angle=1.1, coverage=2.2, fill=true, clip=true,
             edge="found", dips={3, 0.9, 0.2}, seed=531})
local wm = function(x, y) return (x-585)*0.72 + (y-470)*0.69 end
work(newh2 * mask(function(x, y) return smoothstep(15, 60, wm(x, y)) end),
     {hand="body", pile=pj_sh2, angle=1.1, coverage=1.5, fill=true, clip=true, edge="soft",
      dips={3, 0.85, 0.2}, seed=532})
work(newh2 * mask(function(x, y) return 1 - smoothstep(-20, 25, wm(x, y)) end),
     {hand="body", pile=pj_lit, angle=1.1, coverage=1.4, fill=true, clip=true, edge="soft",
      dips={3, 0.9, 0.2}, seed=533})
work(ribbon({{674, 428}, {706, 443}, {723, 470}, {717, 506}, {686, 537}}, 4) * newh2,
     {hand="detail", pile=pj_core2, clip=true, fill=true, coverage=1.5, seed=534})
work(ribbon({{672, 418}, {712, 428}, {731, 461}}, 4) * newh2,
     {hand="detail", pile=pj_lit, clip=true, fill=true, coverage=1.6, seed=535})
work(ellipse(686, 546, 16, 8) * jugm, {hand="detail", pile=pj_core2, clip=true, fill=true,
     coverage=1.4, seed=536})
print("handle drawn v2", wait(0))

--@ chunk 59
pj_sh4 = pile{{"lead white", 1}, {"smalt", 0.6}, {"raw umber", 0.28}, medium=0.16}
pj_core4 = pile{{"smalt", 1}, {"raw umber", 0.7}, {"lead white", 0.6}, medium=0.22}
pj_refl4 = pile{{"lead white", 1}, {"yellow ochre", 0.45}, {"smalt", 0.12}, medium=0.15}
local jugall = jugm + handlem
local wm = function(x, y) return (x-585)*0.72 + (y-470)*0.69 end
local sh = jugall * mask(function(x, y) return smoothstep(25, 75, wm(x, y)) end)
work(sh, {hand="broad", pile=pj_sh4, angle=1.35, coverage=1.35, fill=true, clip=true,
          edge="soft", load=0.8, seed=541})
local co = jugall * mask(function(x, y) return smoothstep(95, 125, wm(x, y)) * (1 - smoothstep(150, 178, wm(x, y))) end)
work(co, {hand="body", pile=pj_core4, angle=1.3, coverage=1.2, fill=true, clip=true,
          edge="soft", dips={3, 0.8, 0.25}, seed=542})
local rf = jugall * mask(function(x, y) return smoothstep(150, 182, wm(x, y)) end)
work(rf, {hand="detail", pile=pj_refl4, clip=true, fill=true, coverage=1.5, seed=543})
blend(sh * mask(function(x, y) return 1 - smoothstep(120, 160, wm(x, y)) end),
      {angle=1.4, coverage=0.5})
work(ellipse(534, 410, 18, 24) + ellipse(556, 330, 6, 12) + ellipse(566, 303, 10, 4),
     {hand="detail", pile=pj_lit, clip=true, fill=true, coverage=1.8, seed=544})
print("jug modelling v3", wait(0))

--@ chunk 60
pj_mid5 = pile{{"lead white", 1}, {"smalt", 0.22}, {"yellow ochre", 0.12}, medium=0.13}
local jugall = jugm + handlem
local wm = function(x, y) return (x-585)*0.72 + (y-470)*0.69 end
local band = jugall * mask(function(x, y) return smoothstep(15, 70, wm(x, y)) * (1 - smoothstep(80, 135, wm(x, y))) end)
work(band, {hand="body", pile=pj_mid5, angle=1.4, coverage=1.3, fill=true, clip=true,
            edge="soft", dips={3, 0.85, 0.2}, seed=551})
blend(band:grow(5) * jugall, {angle=1.45, coverage=0.7})
print("terminator softened", wait(0))

--@ chunk 61
local jugall = jugm + handlem
local keep = ellipse(585, 302, 52, 26) + ellipse(585, 580, 150, 55)
local ring = (((jugall:grow(10) - jugall) * above(table_line)) - keep) * mask(function(x, y) return 1 - smoothstep(520, 560, y) end)
work(ring, {hand="detail", pile=pw_mid, clip=true, fill=true, coverage=2.3, seed=561})
local ring2 = (((jugall:grow(8) - jugall) * below(table_line)) - keep) * mask(function(x, y) return 1 - smoothstep(520, 560, y) end)
work(ring2, {hand="detail", pile=pt_op, clip=true, fill=true, coverage=2.3, seed=562})
print("jug contour cleaned", wait(0))

--@ chunk 62
work(ellipse(585, 308, 42, 20) * jugm,
     {hand="detail", pile=pj_base, clip=true, fill=true, coverage=1.8, seed=571})
work(ellipse(585, 312, 27, 7), {hand="detail", pile=pdark_m, clip=true, fill=true, coverage=2.1, seed=572})
local lipring = (ellipse(585, 306, 35, 11) - ellipse(585, 311, 27, 7))
work(lipring * mask(function(x, y) return 1 - smoothstep(303, 309, y) end),
     {hand="detail", pile=pj_lit, clip=true, fill=true, coverage=1.7, seed=573})
work(lipring * mask(function(x, y) return smoothstep(307, 313, y) end),
     {hand="detail", pile=pj_sh2, clip=true, fill=true, coverage=1.3, seed=574})
work(ellipse(676, 426, 22, 15) + ellipse(684, 546, 24, 15),
     {hand="detail", pile=pj_base, clip=true, fill=true, coverage=1.8, seed=575})
work(ellipse(678, 436, 11, 6) + ellipse(686, 554, 13, 6),
     {hand="detail", pile=pj_core2, clip=true, fill=true, coverage=1.5, seed=576})
print("rim and handle joints", wait(0))

--@ chunk 63
local clean = (ellipse(585, 318, 95, 58) - jugm:grow(3))
work(clean, {hand="detail", pile=pw_mid, clip=true, fill=true, coverage=2.3, seed=581})
work(ellipse(585, 316, 46, 30) * jugm, {hand="detail", pile=pj_base, clip=true, fill=true,
     coverage=1.8, seed=582})
work(ellipse(585, 313, 27, 9), {hand="detail", pile=pdark_m, clip=true, fill=true,
     coverage=2.1, seed=583})
local lipring = (ellipse(585, 309, 32, 12) - ellipse(585, 313, 27, 9))
work(lipring * mask(function(x, y) return 1 - smoothstep(306, 311, y) end),
     {hand="detail", pile=pj_lit, clip=true, fill=true, coverage=1.7, seed=584})
work(lipring * mask(function(x, y) return smoothstep(312, 317, y) end),
     {hand="detail", pile=pj_sh2, clip=true, fill=true, coverage=1.4, seed=585})
work(ribbon({{560, 322}, {585, 327}, {610, 322}}, 4),
     {hand="detail", pile=pj_core2, clip=true, fill=true, coverage=1.3, seed=586})
print("neck top rebuilt", wait(0))

--@ chunk 64
objf = jugm + newh2
local s1 = ribbon({{602, 602}, {684, 614}, {772, 624}, {858, 631}, {930, 634}}, 54):soften(11)
local contact = ellipse(585, 597, 114, 20):soften(7)
local ringA = (objf:grow(45) - objf:grow(1)) * above(table_line)
work(ringA, {hand="detail", pile=pw_mid, clip=true, fill=true, coverage=2.3, seed=591})
local ringB = ((objf:grow(45) - objf:grow(1)) * below(table_line)) - s1 - contact
work(ringB, {hand="detail", pile=pt_op, clip=true, fill=true, coverage=2.3, seed=592})
print("halo cleared around jug", wait(0))

--@ chunk 65
print("now:", wait(2*24*60))
print("fringe:", drying(470, 450), "| jug:", drying(560, 450), "| wall:", drying(300, 300), "| table:", drying(500, 700), "| neck:", drying(530, 310))

--@ chunk 66
print("now:", wait(4*24*60))
print("fringe:", drying(470, 450), "| jug:", drying(560, 450), "| table:", drying(500, 700), "| neck:", drying(530, 310), "| lemon:", drying(330, 632))

--@ chunk 67
print("now:", wait(3*24*60))
print("fringe:", drying(470, 450), "| jug:", drying(560, 450), "| table:", drying(500, 700), "| neck:", drying(530, 310), "| wallr:", drying(700, 300))

--@ chunk 68
local s1 = ribbon({{602, 602}, {684, 614}, {772, 624}, {858, 631}, {930, 634}}, 54):soften(11)
local contact = ellipse(585, 597, 114, 20):soften(7)
local ringA = (objf:grow(45) - objf:grow(1)) * above(table_line)
work(ringA, {hand="detail", pile=pw_mid, clip=true, fill=true, coverage=2.4, seed=601})
local ringB = ((objf:grow(45) - objf:grow(1)) * below(table_line)) - s1 - contact
work(ringB, {hand="detail", pile=pt_op, clip=true, fill=true, coverage=2.4, seed=602})
print("halo cleared v2", wait(0))

--@ chunk 69
local base_zone = (ellipse(600, 602, 250, 85) * tblf) - newh2:grow(1)
work(base_zone, {hand="detail", pile=pt_op, clip=true, fill=true, coverage=2.2, seed=611})
blend(base_zone:soften(18), {angle=0.08, coverage=0.55})
local s1 = ribbon({{604, 603}, {686, 615}, {774, 625}, {858, 632}, {930, 635}}, 52):soften(11)
local contact = ellipse(588, 598, 118, 21):soften(8)
work(contact, {hand="glaze", pile=psh_c, angle=0.1, coverage=1.35, clip=tblf, edge="lost",
               load=0.85, seed=612})
work(s1, {hand="glaze", pile=psh_g, angle=0.12, coverage=1.1, clip=tblf, edge="lost",
          load=0.75, seed=613})
blend((s1 + contact) * tblf, {angle=0.12, coverage=0.45})
print("jug base settled", wait(0))

--@ chunk 70
work(ellipse(585, 316, 44, 28) * jugm, {hand="detail", pile=pj_base, clip=true, fill=true,
     coverage=1.9, seed=621})
work(ellipse(585, 313, 26, 8), {hand="detail", pile=pdark_m, clip=true, fill=true,
     coverage=2.2, seed=622})
local lipring = (ellipse(585, 309, 31, 11) - ellipse(585, 313, 26, 8))
work(lipring * mask(function(x, y) return 1 - smoothstep(305, 311, y) end),
     {hand="detail", pile=pj_lit, clip=true, fill=true, coverage=1.8, seed=623})
work(lipring * mask(function(x, y) return smoothstep(312, 317, y) end),
     {hand="detail", pile=pj_sh2, clip=true, fill=true, coverage=1.4, seed=624})
work(ellipse(556, 330, 5, 11) + ellipse(540, 412, 14, 20) + ellipse(703, 431, 7, 6),
     {hand="detail", pile=pj_lit, clip=true, fill=true, coverage=1.9, seed=625})
print("rim and highlights", wait(0))

--@ chunk 71
pw_glow2 = pile{{"yellow ochre", 1}, {"raw umber", 0.5}, {"lead white", 0.28}, {"red earth", 0.12}, medium=0.5}
pw_dark2 = pile{{"raw umber", 1}, {"bone black", 0.7}, {"red earth", 0.2}, medium=0.55}
local wallf = above(table_line) - objf:grow(1.5)
work((ellipse(80, 40, 420, 300) * wallf):soften(60),
     {hand="glaze", pile=pw_glow2, angle=-0.2, coverage=0.8, clip=wallf, edge="lost",
      load=0.7, seed=631})
work((ellipse(1030, 260, 380, 380) * wallf):soften(60),
     {hand="glaze", pile=pw_dark2, angle=-0.15, coverage=0.7, clip=wallf, edge="lost",
      load=0.7, seed=632})
blend(wallf, {angle=-0.1, coverage=0.3})
print("wall atmosphere", wait(0))

--@ chunk 72
pj_core5 = pile{{"smalt", 1}, {"raw umber", 0.75}, {"lead white", 0.5}, medium=0.2}
pj_refl5 = pile{{"lead white", 1}, {"yellow ochre", 0.42}, {"smalt", 0.12}, medium=0.15}
local wm = function(x, y) return (x-585)*0.72 + (y-470)*0.69 end
local co = objf * mask(function(x, y) return smoothstep(105, 135, wm(x, y)) * (1 - smoothstep(152, 178, wm(x, y))) end)
work(co, {hand="body", pile=pj_core5, angle=1.3, coverage=1.3, fill=true, clip=true,
          edge="soft", dips={3, 0.8, 0.2}, seed=641})
local rf = objf * mask(function(x, y) return smoothstep(152, 182, wm(x, y)) end)
work(rf, {hand="detail", pile=pj_refl5, clip=true, fill=true, coverage=1.5, seed=642})
blend(co, {angle=1.4, coverage=0.5})
print("core shadow strengthened", wait(0))

--@ chunk 73
local wh = function(x, y) return (x-702)*0.72 + (y-482)*0.69 end
work(newh2 * mask(function(x, y) return 1 - smoothstep(-5, 25, wh(x, y)) end),
     {hand="body", pile=pj_lit, angle=1.1, coverage=1.4, fill=true, clip=true, edge="soft",
      dips={3, 0.9, 0.2}, seed=651})
work(newh2 * mask(function(x, y) return smoothstep(20, 55, wh(x, y)) end),
     {hand="body", pile=pj_sh2, angle=1.1, coverage=1.2, fill=true, clip=true, edge="soft",
      dips={3, 0.85, 0.2}, seed=652})
work(newh2 * mask(function(x, y) return smoothstep(45, 68, wh(x, y)) end),
     {hand="detail", pile=pj_core5, clip=true, fill=true, coverage=1.3, seed=653})
blend(newh2, {angle=1.1, coverage=0.4})
print("handle lit properly", wait(0))

--@ chunk 74
local wallf = above(table_line) - objf:grow(1.5)
work((ellipse(150, 110, 500, 350) * wallf):soften(65),
     {hand="broad", pile=pw_mid, angle=-0.18, coverage=1.35, fill=true, clip=wallf,
      edge="lost", load=0.8, seed=661})
blend((ellipse(150, 110, 520, 360) * wallf):soften(65), {angle=-0.15, coverage=0.5})
pgl_glow = pile{{"yellow ochre", 1}, {"raw umber", 0.6}, medium=0.65}
work((ellipse(60, 30, 380, 260) * wallf):soften(70),
     {hand="glaze", pile=pgl_glow, angle=-0.2, coverage=0.55, clip=wallf, edge="lost",
      load=0.7, seed=662})
print("wall glow settled", wait(0))

--@ chunk 75
pj_core6 = pile{{"smalt", 1}, {"raw umber", 0.8}, {"lead white", 0.55}, medium=0.18}
pj_refl6 = pile{{"lead white", 1}, {"yellow ochre", 0.4}, {"smalt", 0.1}, medium=0.14}
local wm = function(x, y) return (x-585)*0.72 + (y-470)*0.69 end
local co = objf * mask(function(x, y) return smoothstep(112, 140, wm(x, y)) * (1 - smoothstep(152, 176, wm(x, y))) end)
work(co, {hand="body", pile=pj_core6, angle=1.3, coverage=1.35, fill=true, clip=true,
          edge="soft", dips={3, 0.8, 0.2}, seed=671})
local rf = objf * mask(function(x, y) return smoothstep(152, 180, wm(x, y)) end)
work(rf, {hand="detail", pile=pj_refl6, clip=true, fill=true, coverage=1.6, seed=672})
blend(co, {angle=1.4, coverage=0.55})
print("jug final modelling", wait(0))

--@ chunk 76
local wallf = above(table_line) - objf:grow(1.5)
work((ellipse(1030, 90, 430, 340) * wallf):soften(65),
     {hand="broad", pile=pw_mid, angle=-0.15, coverage=1.3, fill=true, clip=wallf,
      edge="lost", load=0.8, seed=681})
work((ellipse(150, 90, 430, 340) * wallf):soften(65),
     {hand="broad", pile=pw_mid, angle=-0.18, coverage=1.15, fill=true, clip=wallf,
      edge="lost", load=0.8, seed=682})
blend(wallf, {angle=-0.12, coverage=0.45})
print("wall corners quieted", wait(0))

--@ chunk 77
pgl_veil = pile{{"yellow ochre", 1}, {"red earth", 0.3}, {"lead white", 0.2}, medium=0.6}
work(tblf, {hand="glaze", pile=pgl_veil, angle=0.04, coverage=0.6, clip=true, edge="found",
            load=0.8, seed=691})
blend(tblf, {angle=0.03, coverage=0.4})
print("table veiled", wait(0))

--@ chunk 78
print("left to dry until", wait(3*24*60))
print("jug:", drying(560, 450), "| table:", drying(500, 700), "| wall:", drying(300, 300), "| lemon:", drying(330, 632))

--@ chunk (finishing)
-- finishing, applied after the session by scripts/finish_painting
local function when_dry(f)
  for _ = 1, 120 do
    local ok, e = pcall(f)
    if ok then return end
    if not tostring(e):find('not all dry', 1, true) then error(e, 0) end
    wait(30 * 24 * 60)
  end
  error('still not dry after ten years', 0)
end
when_dry(function() varnish{coats=0.4} end)
when_dry(function() cracks{} end)
