-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=500, aspect=1.25, linen={16,14}, seed=7, ground={{pile={{"lead white",6},{"yellow ochre",2},{"raw umber",0.4}}, um=150, apply="knife", texture=0.3}}}

--@ chunk 2
h = pencil("HB")
-- table edge (wall meets table)
h:rule({0,446},{1000,470},{pressure=0.2})
-- bottle
h:line({{238,606},{232,470},{238,398},{258,342},{276,304},{276,222},{296,220},{296,304},{314,344},{330,400},{340,470},{344,606}}, {pressure=0.2, smooth=true})
-- bowl rim (ellipse) and body
h:line({{498,542},{528,516},{578,504},{640,502},{702,508},{742,526},{762,546},{742,566},{692,578},{640,582},{578,576},{528,562},{500,550}}, {pressure=0.2, smooth=true})
h:line({{500,548},{508,592},{536,622},{588,634},{664,634},{712,618},{742,590},{760,548}}, {pressure=0.2, smooth=true})
-- pear (right foreground)
h:line({{820,660},{800,630},{806,590},{828,556},{852,540},{872,552},{878,590},{866,630},{852,660}}, {pressure=0.2, smooth=true})
h:line({{852,540},{856,512},{866,500}}, {pressure=0.2, smooth=true})
-- lemon (left foreground)
h:line({{92,662},{74,640},{80,614},{108,600},{146,600},{170,616},{168,644},{148,662}}, {pressure=0.2, smooth=true})

--@ chunk 3
h = pencil("B")
h:rule({0,446},{1000,470},{pressure=0.5})
h:line({{238,606},{232,470},{238,398},{258,342},{276,304},{276,222},{296,220},{296,304},{314,344},{330,400},{340,470},{344,606}}, {pressure=0.55, smooth=true})
h:line({{498,542},{528,516},{578,504},{640,502},{702,508},{742,526},{762,546},{742,566},{692,578},{640,582},{578,576},{528,562},{500,550}}, {pressure=0.55, smooth=true})
h:line({{500,548},{508,592},{536,622},{588,634},{664,634},{712,618},{742,590},{760,548}}, {pressure=0.55, smooth=true})
h:line({{820,660},{800,630},{806,590},{828,556},{852,540},{872,552},{878,590},{866,630},{852,660}}, {pressure=0.55, smooth=true})
h:line({{852,540},{856,512},{866,500}}, {pressure=0.55, smooth=true})
h:line({{92,662},{74,640},{80,614},{108,600},{146,600},{170,616},{168,644},{148,662}}, {pressure=0.55, smooth=true})

--@ chunk 4
tableline = function(x) return 446 + 0.024 * x end
wallmask = above(tableline)
tablemask = below(tableline)
print("wall", wallmask:area(), "table", tablemask:area())

--@ chunk 5
pw = pile{{"raw umber",2},{"smalt",1},{"green earth",1},{"lead white",4}}
work(wallmask, {hand="broad", pile=pw, clip=true, coverage=3, angle=0.15, fill=true, seed=11})
blend(wallmask, {seed=12})

--@ chunk 6
pdark = pile{{"raw umber",4},{"lead white",2},{"smalt",1},{"bone black",0.5}}
work(wallmask, {hand="broad", pile=pdark, clip=true, coverage=3, angle=0.1, fill=true, seed=21})
blend(wallmask, {seed=22})

--@ chunk 7
glow = mask(function(x,y)
  return clamp(1 - math.sqrt(((x-230)/430)^2 + ((y-360)/300)^2), 0, 1)
end):soften(80)
pglow = pile{{"lead white",3},{"yellow ochre",1},{"raw umber",1}}
work(glow * wallmask, {hand="broad", pile=pglow, clip=true, coverage=2, angle=0.05, load=0.7, seed=31})
blend((glow * wallmask):soften(60), {seed=32})

--@ chunk 8
work(wallmask, {hand="broad", pile=pdark, clip=true, coverage=3, angle=0.1, fill=true, seed=41})
blend(wallmask, {seed=42, coverage=2, pressure={0.3,0.3}})

--@ chunk 9
ptab = pile{{"yellow ochre",3},{"red earth",1.5},{"raw umber",1.5},{"lead white",2}}
work(tablemask, {hand="broad", pile=ptab, clip=true, coverage=3.5, angle=0.06, fill=true, seed=51})
blend(tablemask, {seed=52, coverage=2, pressure={0.3,0.3}})

--@ chunk 10
ptab = pile{{"raw umber",3},{"yellow ochre",2},{"lead white",2},{"smalt",0.5}}
work(tablemask, {hand="broad", pile=ptab, clip=true, coverage=3.5, angle=0.06, fill=true, seed=61})
blend(tablemask, {seed=62, coverage=2, pressure={0.3,0.3}})

--@ chunk 11
pdark2 = pile{{"raw umber",4},{"smalt",1.5},{"lead white",0.9},{"bone black",0.6}}
work(wallmask, {hand="broad", pile=pdark2, clip=true, coverage=3, angle=0.1, fill=true, seed=71})
blend(wallmask, {seed=72, coverage=2, pressure={0.3,0.3}})

--@ chunk 12
pgz = pile{{"raw umber",3},{"bone black",1},{"smalt",1}}
work(wallmask, {hand="glaze", pile=pgz, clip=true, coverage=1.8, seed=81})
blend(wallmask, {seed=82, coverage=1.5, pressure={0.25,0.25}})

--@ chunk 13
bottlemask = poly({{252,608},{252,520},{254,430},{264,380},{282,340},{292,300},{290,230},{288,218},{312,218},{310,230},{308,300},{318,340},{336,380},{346,430},{348,520},{348,608}}, true)
bowlmask = ellipse(640,540,130,36) + poly({{510,542},{516,580},{536,614},{566,630},{600,637},{680,637},{714,630},{744,614},{764,580},{770,542}}, true)
pearmask = poly({{851,536},{866,554},{876,584},{874,616},{858,644},{836,658},{814,652},{800,630},{798,600},{806,566},{828,542},{844,536}}, true) + ribbon({{850,540},{856,520},{864,510}}, {7,5,3})
lemonmask = poly({{82,632},{90,610},{112,600},{140,600},{160,610},{174,628},{172,648},{152,660},{124,662},{98,654},{84,644}}, true)
print("bot",bottlemask:area(),"bowl",bowlmask:area(),"pear",pearmask:area(),"lemon",lemonmask:area())

--@ chunk 14
shadows = (ellipse(430,636,185,40) + ellipse(200,660,95,28) + ellipse(900,662,75,24) + ellipse(755,656,130,34)):soften(22)
psh = pile{{"raw umber",2},{"smalt",1},{"bone black",0.6}, medium=0.25}
m = shadows * tablemask
work(m, {hand="glaze", pile=psh, clip=true, coverage=2.0, seed=91})
blend(m:soften(25), {seed=92, coverage=1.5, pressure={0.25,0.25}})

--@ chunk 15
work(tablemask, {hand="broad", pile=ptab, clip=true, coverage=3.5, angle=0.06, fill=true, seed=101})
blend(tablemask, {seed=102, coverage=2, pressure={0.3,0.3}})

--@ chunk 16
psh = pile{{"raw umber",1.5},{"smalt",1},{"bone black",0.25}, medium=0.4}
sb = brush{kind="filbert", width=44, stiffness=0.35, splay=0.25}
sb:load(psh, 0.35)
sb:stroke({{300,612},{420,628},{540,644},{640,656}}, {pressure={0.6,0.05}, ramps={0.12,0.6}})
sb:wipe(0.5); sb:load(psh, 0.25)
sb:stroke({{640,646},{760,660},{900,670}}, {pressure={0.5,0.05}, ramps={0.15,0.6}})
sb:stroke({{130,652},{230,664},{330,674}}, {pressure={0.5,0.08}, ramps={0.15,0.55}})
sb:stroke({{845,650},{920,660},{990,666}}, {pressure={0.45,0.08}, ramps={0.15,0.55}})
blend(shadows:soften(30), {seed=111, coverage=1.6, pressure={0.25,0.25}})

--@ chunk 17
band = rect(0,596,1000,120):soften(28) * tablemask
pwarm = pile{{"yellow ochre",1.5},{"raw umber",1},{"lead white",0.7}, medium=0.5}
work(band, {hand="glaze", pile=pwarm, clip=true, coverage=1.3, seed=121})
blend((shadows:soften(45) * tablemask), {seed=122, coverage=2.5, pressure={0.25,0.25}})
blend(band, {seed=123, coverage=1.5, pressure={0.3,0.3}})

--@ chunk 18
pbot = pile{{"bone black",1},{"green earth",1.2},{"raw umber",0.3},{"Prussian blue",0.08}}
work(bottlemask, {hand="body", pile=pbot, clip=true, coverage=3.5, angle=1.5, fill=true, seed=131})
blend(bottlemask, {seed=132, coverage=1.2, pressure={0.3,0.3}})

--@ chunk 19
pbowl = pile{{"lead white",5},{"pale smalt",0.8},{"yellow ochre",0.4}}
work(bowlmask, {hand="body", pile=pbowl, clip=true, coverage=3.5, fill=true, angle=0.25, seed=141})
blend(bowlmask, {seed=142, coverage=1.5, pressure={0.3,0.3}})

--@ chunk 20
oldbowl = bowlmask
bowlmask = ellipse(640,534,128,34) + ellipse(640,598,92,46)
gap = oldbowl - bowlmask
print("gap", gap:area())
work(gap, {hand="body", pile=ptab, clip=true, coverage=3, fill=true, seed=151})
blend(gap:soften(8), {seed=152, coverage=1.5, pressure={0.3,0.3}})
work(bowlmask, {hand="body", pile=pbowl, clip=true, coverage=3.5, fill=true, angle=0.25, seed=153})
blend(bowlmask, {seed=154, coverage=1.5, pressure={0.3,0.3}})

--@ chunk 21
clear = (ellipse(640,538,152,50) + ellipse(640,600,114,60)) - bottlemask
work(clear, {hand="body", pile=ptab, clip=true, coverage=3.2, fill=true, seed=161})
blend(clear:soften(6), {seed=162, coverage=1.5, pressure={0.3,0.3}})
bowlmask = ellipse(640,534,128,34) + poly({{512,534},{514,552},{520,572},{530,592},{544,608},{564,622},{586,632},{608,637},{672,637},{694,632},{716,622},{736,608},{750,592},{760,572},{766,552},{768,534}}, true)
work(bowlmask, {hand="body", pile=pbowl, clip=true, coverage=3.2, fill=true, angle=0.25, seed=163})
blend(bowlmask, {seed=164, coverage=1.5, pressure={0.3,0.3}})

--@ chunk 22
clear = ellipse(640,548,158,64)
work(clear, {hand="body", pile=ptab, clip=true, coverage=3.2, fill=true, seed=171})
blend(clear:soften(5), {seed=172, coverage=1.5, pressure={0.3,0.3}})
bowlmask = ellipse(640,548,130,46)
work(bowlmask, {hand="body", pile=pbowl, clip=true, coverage=3.2, fill=true, angle=0.25, seed=173})
blend(bowlmask, {seed=174, coverage=1.5, pressure={0.3,0.3}})

--@ chunk 23
ring = ellipse(640,548,205,98) - bowlmask
work(ring, {hand="body", pile=ptab, clip=true, coverage=3.0, fill=true, seed=181})
blend(ring:soften(8), {seed=182, coverage=1.0, pressure={0.3,0.3}})
work(ring, {hand="body", pile=ptab, clip=true, coverage=3.0, fill=true, seed=183})

--@ chunk 24
print(wait(2880))
print("bowl", drying(640,548), "halo", drying(790,560), "table", drying(300,700), "bottle", drying(300,500), "wall", drying(500,200))

--@ chunk 25
print(wait(5*1440))
print("bowl", drying(640,548), "halo", drying(790,560), "bottle", drying(300,500))

--@ chunk 26
bowlmask = mask(function(x,y)
  local hw
  local dy = y - 540
  if dy <= 0 then
    local t = -dy / 34
    if t >= 1 then return 0 end
    hw = 128 * math.sqrt(1 - t*t)
  else
    local t = dy / 96
    if t >= 1 then return 0 end
    hw = 30 + 98 * (1 - t*t)^0.7
  end
  if math.abs(x - 640) <= hw then return 1 else return 0 end
end)
oldring = ellipse(640,548,215,105) - bowlmask
print("ring", oldring:area())
work(oldring, {hand="body", pile=ptab, clip=true, coverage=3.5, fill=true, seed=191})
work(bowlmask, {hand="body", pile=pbowl, clip=true, coverage=3.5, fill=true, angle=0.25, seed=192})
blend(bowlmask, {seed=193, coverage=1.2, pressure={0.3,0.3}})

--@ chunk 27
ptab2 = pile{{"raw umber",4},{"yellow ochre",2},{"lead white",1.6},{"smalt",0.5}}
work(tablemask, {hand="broad", pile=ptab2, clip=true, coverage=3.2, angle=0.06, fill=true, seed=201})
bowlmask = mask(function(x,y)
  local hw
  local dy = y - 540
  if dy <= 0 then
    local t = -dy / 34
    if t >= 1 then return 0 end
    hw = 128 * math.sqrt(1 - t*t)
  else
    local t = dy / 96
    if t >= 1 then return 0 end
    hw = math.max(38, 128 * math.sqrt(1 - t*t*t*t))
  end
  if math.abs(x - 640) <= hw then return 1 else return 0 end
end)
work(bowlmask, {hand="body", pile=pbowl, clip=true, coverage=3.5, fill=true, angle=0.25, seed=202})
blend(bowlmask, {seed=203, coverage=1.2, pressure={0.3,0.3}})

--@ chunk 28
work(bottlemask, {hand="body", pile=pbot, clip=true, coverage=3.5, angle=1.5, fill=true, seed=211})
blend(bottlemask, {seed=212, coverage=1.0, pressure={0.3,0.3}})
bowlmask = mask(function(x,y)
  local hw
  local dy = y - 540
  if dy <= 0 then
    local t = -dy / 34
    if t >= 1 then return 0 end
    hw = 128 * math.sqrt(1 - t*t)
  else
    local t = dy / 96
    if t >= 1 then return 0 end
    hw = 128 * (1 - t*t)^0.45
  end
  if math.abs(x - 640) <= hw then return 1 else return 0 end
end)
work(bowlmask, {hand="body", pile=pbowl, clip=true, coverage=3.5, fill=true, angle=0.25, seed=213})
blend(bowlmask, {seed=214, coverage=1.2, pressure={0.3,0.3}})

--@ chunk 29
open = ellipse(640,538,126,32)
open_in = ellipse(640,538,113,25)
rimband = (open - open_in) * bowlmask
pin = pile{{"raw umber",1.6},{"lead white",2.5},{"smalt",1}}
prim = pile{{"lead white",5},{"yellow ochre",0.5},{"pale smalt",0.25}}
pdarkbowl = pile{{"raw umber",2.5},{"lead white",1.5},{"smalt",1}}
pshd = pile{{"raw umber",2},{"smalt",1.6},{"lead white",1.6},{"bone black",0.15}}
work(open_in, {hand="body", pile=pin, clip=true, coverage=2.6, angle=0.2, seed=221})
idark = open_in * mask(function(x,y) return clamp((y-520)/70,0,1) end)
work(idark, {hand="body", pile=pdarkbowl, clip=true, coverage=2.2, seed=222})
work(rimband, {hand="body", pile=prim, clip=true, coverage=2.6, angle=0.2, seed=223})
owall = bowlmask - open
gsh = mask(function(x,y) return clamp((x-620)/300, 0, 1) end):soften(20)
work(owall * gsh, {hand="body", pile=pshd, clip=true, coverage=2.2, angle=0.6, seed=224})
glit = mask(function(x,y) return clamp(1-(x-500)/160,0,1) end) * owall
work(glit, {hand="body", pile=prim, clip=true, coverage=1.8, angle=0.6, seed=225})

--@ chunk 30
function grad(cx, cy, rx, ry, kx, ky, lo, hi)
  return mask(function(x,y)
    local ux = (x-cx)/rx
    local uy = (y-cy)/ry
    local t = 0.5 + 0.5*(ux*kx + uy*ky)
    return clamp((t-lo)/(hi-lo), 0, 1)
  end)
end
lemA = poly({{540,500},{552,486},{558,468},{576,456},{600,452},{624,458},{642,472},{650,488},{660,498},{650,512},{642,528},{624,540},{600,546},{576,540},{558,528},{552,514}}, true)
lemB = poly({{648,506},{660,494},{666,478},{682,468},{702,464},{722,470},{736,482},{744,494},{752,506},{744,518},{736,532},{722,542},{702,546},{682,542},{666,530},{660,518}}, true)
lemC = poly({{66,652},{76,638},{88,624},{110,614},{136,612},{158,618},{172,630},{180,642},{178,656},{168,668},{150,676},{126,678},{102,674},{84,666},{72,660}}, true)
pear = poly({{850,520},{864,532},{876,552},{882,578},{880,608},{870,632},{852,650},{830,654},{810,644},{800,624},{798,598},{804,572},{816,548},{832,530},{844,520}}, true) + ribbon({{850,524},{858,506},{864,494}}, {6,4,2.5})
plemon = pile{{"chrome yellow",3},{"lead white",1.5},{"yellow ochre",0.6}}
ppear = pile{{"yellow ochre",2},{"green earth",1.5},{"lead white",2},{"chrome yellow",0.5}}
work(lemA + lemB, {hand="body", pile=plemon, clip=true, coverage=3.2, fill=true, seed=231})
work(lemC, {hand="body", pile=plemon, clip=true, coverage=3.2, fill=true, seed=232})
work(pear, {hand="body", pile=ppear, clip=true, coverage=3.2, fill=true, seed=233})
print("ok")

--@ chunk 31
plight = pile{{"lead white",3},{"chrome yellow",2}}
pdarkL = pile{{"yellow ochre",2},{"raw umber",1},{"green earth",0.8},{"smalt",0.3}}
-- model the two lemons in the bowl
local function model(m, cx, cy, rx, ry, seed)
  local lit = grad(cx,cy,rx,ry,-0.75,-0.66,0.55,1.0)
  local shad = grad(cx,cy,rx,ry,0.75,0.66,0.45,0.95)
  work(m * lit, {hand="body", pile=plight, clip=true, coverage=1.8, angle=0.9, seed=seed})
  work(m * shad, {hand="body", pile=pdarkL, clip=true, coverage=1.9, angle=0.9, seed=seed+1})
end
model(lemA, 600, 498, 60, 46, 251)
model(lemB, 700, 504, 54, 42, 253)
-- occlusion where the lemons meet and under them
pm = pile{{"raw umber",2},{"smalt",1.2},{"lead white",1.5}}
occl = ((ellipse(654,500,14,50) + ellipse(600,548,66,16) + ellipse(700,554,58,15)):soften(10)) * (lemA + lemB)
work(occl, {hand="body", pile=pm, clip=true, coverage=2.0, seed=255})
-- contact shadow on the bowl floor
floor = ((ellipse(600,556,70,15) + ellipse(700,562,58,14)):soften(8)) * open_in
work(floor, {hand="body", pile=pm, clip=true, coverage=1.8, seed=256})

--@ chunk 32
s1 = body.ellipsoid({600,498,0},{52,46,50})
s2 = body.ellipsoid({700,504,0},{48,42,46})
f = form{{s1,dist=0.3},{s2,dist=0.3}, light={from={-1,-0.75}, front=0.55, ambient=0.25}}
print("value at 590,480", f:value(590,480), " lit_at", f:lit_at(590,480,0.15))
litM = f:lit{parts={1,2}, soft=0.15}
shM  = f:shadow{parts={1,2}}
silM = f:silhouette{parts={1,2}}
print("lit area", litM:area(), "shadow area", shM:area(), "sil", silM:area())
work(litM, {hand="body", pile=plight, clip=true, coverage=1.6, seed=261})
work(shM, {hand="body", pile=pdarkL, clip=true, coverage=1.6, seed=262})

--@ chunk 33
function castshadow(bx,by,ex,ey,w0,w1)
  return mask(function(x,y)
    local dx, dy = ex-bx, ey-by
    local L2 = dx*dx+dy*dy
    local t = ((x-bx)*dx + (y-by)*dy)/L2
    local tc = clamp(t,0,1)
    local px, py = bx+dx*tc, by+dy*tc
    local d = math.sqrt((x-px)^2 + (y-py)^2)
    local w = w0 + (w1-w0)*tc
    local a = clamp(1 - (d/w)^2, 0, 1)
    local fade
    if t < 0 then fade = clamp(1 + t*5, 0, 1) else fade = clamp((1-t)*2.5, 0, 1) end
    return a*fade
  end)
end
objall = bottlemask + bowlmask + lemA + lemB + lemC + pear
table_free = tablemask - objall
ptabLight = pile{{"yellow ochre",2.5},{"lead white",2},{"raw umber",0.8}}
ptabDark = pile{{"raw umber",3},{"smalt",1.2},{"bone black",0.3},{"lead white",1}}
tlight = ellipse(390,545,430,150):soften(110) * table_free
work(tlight, {hand="broad", pile=ptabLight, clip=true, coverage=2.0, angle=0.06, seed=271})
tdark = mask(function(x,y)
  local a = clamp((x-780)/320,0,1)
  local b = clamp((y-680)/180,0,1)
  return clamp(a*0.85 + b*0.85, 0, 1)
end):soften(50) * table_free
work(tdark, {hand="broad", pile=ptabDark, clip=true, coverage=2.2, angle=0.06, seed=272})
print("ok")

--@ chunk 34
pshadow = pile{{"raw umber",2.5},{"smalt",1.5},{"bone black",0.5},{"lead white",0.9}, medium=0.25}
sh_bot = castshadow(310,614,650,662,54,18)
sh_bowl = castshadow(645,640,910,676,74,30)
sh_lem = castshadow(145,664,345,684,36,14)
sh_pear = castshadow(848,654,1000,672,42,18)
for i,m in ipairs{sh_bot, sh_bowl, sh_lem, sh_pear} do
  work((m:soften(7)) * table_free, {hand="glaze", pile=pshadow, clip=true, coverage=2.2, seed=280+i})
end

--@ chunk 35
pshadow2 = pile{{"raw umber",2},{"bone black",0.8},{"smalt",1.2},{"lead white",0.5}, medium=0.15}
sh_bot = castshadow(310,614,650,662,50,16)
sh_bowl = castshadow(645,640,910,676,70,26)
sh_lem = castshadow(145,664,345,684,34,12)
sh_pear = castshadow(848,654,1000,672,40,16)
contact = ellipse(302,616,84,20):soften(9) + ellipse(640,644,112,22):soften(10)
        + ellipse(128,678,60,17):soften(8) + ellipse(848,658,58,17):soften(8)
for i,m in ipairs{sh_bot, sh_bowl, sh_lem, sh_pear} do
  work((m:soften(9)) * table_free, {hand="glaze", pile=pshadow2, clip=true, coverage=2.8, seed=300+i})
end
work(contact * table_free, {hand="body", pile=pshadow2, clip=true, coverage=2.6, seed=311})

--@ chunk 36
owym = bowlmask - open
pbd = pile{{"raw umber",1.5},{"smalt",1.5},{"lead white",1.2}}
pbr = pile{{"yellow ochre",1.2},{"lead white",1.5},{"red earth",0.25}}
rimshadow = ((open:grow(11) - open):soften(7)) * owym
work(rimshadow, {hand="body", pile=pbd, clip=true, coverage=1.8, angle=0.3, seed=321})
gright = grad(640,600,128,90, 0.9, 0.25, 0.35, 1.0) * owym
work(gright, {hand="body", pile=pbd, clip=true, coverage=1.6, angle=1.1, seed=322})
grefl = grad(640,626,128,80, 0.15, 0.9, 0.4, 1.0) * owym
work(grefl, {hand="body", pile=pbr, clip=true, coverage=1.5, angle=0.1, seed=323})
rim_lit = grad(640,538,126,32, -0.9, -0.35, 0.25, 1.0) * rimband
work(rim_lit, {hand="body", pile=pile{{"lead white",4},{"yellow ochre",0.6}}, clip=true, coverage=1.6, angle=0.15, seed=324})

--@ chunk 37
pwdark = pile{{"raw umber",3},{"smalt",1.4},{"bone black",0.6},{"lead white",0.5}}
pwlight = pile{{"lead white",2},{"yellow ochre",1},{"raw umber",1.2}}
wdark = mask(function(x,y) return clamp(0.2 + 0.8*(x/1000) + 0.55*(1 - y/455), 0, 1) end):soften(70)
wlight = mask(function(x,y) return clamp(1 - ((x-220)/430)^2 - ((y-370)/270)^2, 0, 1) end):soften(100)
work(wdark * wallmask, {hand="glaze", pile=pwdark, clip=true, coverage=2.4, seed=331})
work(wlight * wallmask, {hand="glaze", pile=pwlight, clip=true, coverage=1.7, seed=332})

--@ chunk 38
pwdark2 = pile{{"raw umber",3.5},{"smalt",1.2},{"lead white",1.6},{"bone black",0.3}}
work(bottlemask, {hand="body", pile=pbot, clip=true, coverage=3.5, angle=1.5, fill=true, seed=341})
blend(bottlemask, {seed=342, coverage=0.8, pressure={0.3,0.3}})
work(wallmask - objall, {hand="broad", pile=pwdark2, clip=true, coverage=3.0, angle=0.1, fill=true, seed=343})
blend((wallmask - objall):soften(10), {seed=344, coverage=1.2, pressure={0.3,0.3}})

--@ chunk 39
work(wallmask - objall, {hand="broad", pile=pdark2, clip=true, coverage=4.0, angle=0.1, fill=true, seed=351})
wlight2 = mask(function(x,y) return clamp(1 - ((x-200)/520)^2 - ((y-330)/330)^2, 0, 1) end):soften(140)
work(wlight2 * (wallmask - objall), {hand="glaze", pile=pwlight, clip=true, coverage=1.2, seed=352})

--@ chunk 40
pwall3 = pile{{"raw umber",3},{"bone black",1},{"smalt",1}}
work(wallmask - objall, {hand="broad", pile=pwall3, clip=true, coverage=4.0, angle=0.1, fill=true, seed=361})

--@ chunk 41
work(wallmask - objall, {hand="broad", pile=pwall3, clip=true, coverage=3.0, angle=0.1, fill=true, dips={3,0.9}, seed=371})

--@ chunk 42
test = rect(50,50,200,120)
work(test, {hand="body", pile=pile{{"bone black",1}}, clip=true, coverage=4.0, fill=true, dips={2,1.0}, load=1.0, seed=381})

--@ chunk 43
pwall4 = pile{{"bone black",1},{"raw umber",2},{"smalt",0.6}}
work(wallmask - objall, {hand="broad", pile=pwall4, clip=true, coverage=3.5, angle=0.1, fill=true, dips={3,0.85}, seed=391})

--@ chunk 44
oldpear = pear
pear = ellipse(852,600,46,54) + ellipse(852,538,27,32) + ribbon({{852,520},{860,500},{866,490}}, {6,4,2.5})
fix = (oldpear:grow(5) - pear) * tablemask
work(fix, {hand="body", pile=ptabLight, clip=true, coverage=3.4, fill=true, seed=401})
work(pear, {hand="body", pile=ppear, clip=true, coverage=3.5, fill=true, seed=402})
s_pear = body.ellipsoid({852,600,0},{48,56,48})
f2 = form{{s_pear,dist=0.3}, light={from={-1,-0.75}, front=0.55, ambient=0.25}}
litp = f2:lit{parts={1}, soft=0.2}
shp = f2:shadow{parts={1}}
plightp = pile{{"lead white",3},{"chrome yellow",1.5},{"yellow ochre",0.5}}
pdarkp = pile{{"yellow ochre",1.6},{"green earth",1.6},{"raw umber",1},{"smalt",0.4}}
work(pear*litp, {hand="body", pile=plightp, clip=true, coverage=1.9, seed=403})
work(pear*shp, {hand="body", pile=pdarkp, clip=true, coverage=1.8, seed=404})

--@ chunk 45
ptabBase = pile{{"yellow ochre",2.5},{"lead white",1.6},{"raw umber",1}}
ptabDark2 = pile{{"raw umber",2.5},{"smalt",1},{"lead white",0.8}}
ptabLight = pile{{"yellow ochre",2.5},{"lead white",2},{"raw umber",0.7}}
tfree = tablemask - objall
work(tfree, {hand="broad", pile=ptabBase, clip=true, coverage=3.4, angle=0.06, fill=true, dips={3,0.85}, seed=411})
tdark2 = mask(function(x,y)
  local a = clamp((x-700)/340,0,1)
  local b = clamp((y-620)/200,0,1)
  return clamp(a*0.9 + b*0.9, 0, 1)
end):soften(60) * tfree
work(tdark2, {hand="broad", pile=ptabDark2, clip=true, coverage=2.0, angle=0.06, seed=412})
tlight2 = ellipse(400,545,430,150):soften(120) * tfree
work(tlight2, {hand="broad", pile=ptabLight, clip=true, coverage=1.7, angle=0.06, seed=413})
sh_bot = castshadow(310,614,650,662,50,16)
sh_bowl = castshadow(645,640,910,676,70,26)
sh_lem = castshadow(145,664,345,684,34,12)
sh_pear2 = castshadow(848,656,1000,674,40,16)
for i,m in ipairs{sh_bot, sh_bowl, sh_lem, sh_pear2} do
  work((m:soften(9)) * tfree, {hand="glaze", pile=pshadow2, clip=true, coverage=2.6, seed=420+i})
end
contact = ellipse(302,616,84,20):soften(9) + ellipse(640,644,112,22):soften(10)
        + ellipse(128,678,60,17):soften(8) + ellipse(852,660,58,17):soften(8)
work(contact * tfree, {hand="body", pile=pshadow2, clip=true, coverage=2.6, seed=431})

--@ chunk 46
ptabMid = pile{{"yellow ochre",2.5},{"raw umber",2},{"lead white",1.2},{"red earth",0.4}}
tfree = tablemask - objall
work(tfree, {hand="broad", pile=ptabMid, clip=true, coverage=3.4, angle=0.06, fill=true, dips={3,0.85}, seed=441})
tlight3 = ellipse(420,560,380,140):soften(110) * tfree
work(tlight3, {hand="broad", pile=ptabLight, clip=true, coverage=1.8, angle=0.06, seed=442})
work(tdark2, {hand="broad", pile=ptabDark2, clip=true, coverage=2.2, angle=0.06, seed=443})
pshadow3 = pile{{"raw umber",2},{"bone black",1.1},{"smalt",1.2},{"lead white",0.4}, medium=0.1}
sh_bot = castshadow(300,616,620,660,52,18)
sh_bowl = castshadow(640,644,880,678,72,28)
sh_lem = castshadow(140,666,330,686,36,14)
sh_pear2 = castshadow(848,658,990,676,42,18)
for i,m in ipairs{sh_bot, sh_bowl, sh_lem, sh_pear2} do
  work((m:soften(8)) * tfree, {hand="glaze", pile=pshadow3, clip=true, coverage=3.0, seed=450+i})
end
contact = ellipse(302,616,84,20):soften(9) + ellipse(640,644,112,22):soften(10)
        + ellipse(128,678,60,17):soften(8) + ellipse(852,660,58,17):soften(8)
work(contact * tfree, {hand="body", pile=pshadow3, clip=true, coverage=2.8, seed=461})

--@ chunk 47
ptabD = pile{{"yellow ochre",2},{"raw umber",2.5},{"lead white",1.0},{"red earth",0.4}}
tfree = tablemask - objall
work(tfree, {hand="broad", pile=ptabD, clip=true, coverage=3.4, angle=0.06, fill=true, dips={3,0.85}, seed=471})
tlight4 = ellipse(420,560,380,140):soften(110) * tfree
work(tlight4, {hand="broad", pile=ptabLight, clip=true, coverage=1.8, angle=0.06, seed=472})
pS = pile{{"raw umber",2},{"bone black",1.2},{"smalt",1},{"lead white",0.3}, medium=0.1}
sh_bot = castshadow(300,616,620,660,52,18)
sh_bowl = castshadow(640,644,880,678,72,28)
sh_lem = castshadow(140,666,330,686,36,14)
sh_pear2 = castshadow(848,658,990,676,42,18)
for i,m in ipairs{sh_bot, sh_bowl, sh_lem, sh_pear2} do
  work((m:soften(8)) * tfree, {hand="body", pile=pS, clip=true, coverage=2.8, seed=480+i})
end
contact = ellipse(302,616,84,20):soften(9) + ellipse(640,644,112,22):soften(10)
        + ellipse(128,678,60,17):soften(8) + ellipse(852,660,58,17):soften(8)
work(contact * tfree, {hand="body", pile=pS, clip=true, coverage=2.8, seed=491})

--@ chunk 48
pS2 = pile{{"raw umber",2},{"bone black",1},{"smalt",1.2}, medium=0.35}
b = brush{kind="filbert", width=54, stiffness=0.3, splay=0.35}
b:load(pS2, 0.45)
b:stroke({{300,616},{420,632},{560,648}}, {pressure={0.55,0.12}, ramps={0.15,0.5}})
b:stroke({{636,646},{760,664},{880,676}}, {pressure={0.5,0.12}, ramps={0.15,0.5}})
b:stroke({{140,668},{240,680},{330,688}}, {pressure={0.45,0.15}, ramps={0.15,0.5}})
b:stroke({{848,658},{920,668},{990,674}}, {pressure={0.45,0.15}, ramps={0.15,0.5}})
tfree = tablemask - objall
smask = (sh_bot + sh_bowl + sh_lem + sh_pear2):soften(18) * tfree
blend(smask, {seed=501, coverage=1.6, pressure={0.3,0.3}})

--@ chunk 49
plb = pile{{"lead white",2},{"green earth",0.8},{"pale smalt",0.5},{"raw umber",0.5}}
pmb = pile{{"lead white",1},{"green earth",1.5},{"bone black",0.5},{"pale smalt",0.5}}
pdb = pile{{"bone black",1},{"green earth",1},{"smalt",0.5}}
b2 = brush{kind="filbert", width=16, stiffness=0.5}
b2:load(plb, 0.7)
b2:stroke({{278,392},{271,440},{267,500},{269,560},{273,600}}, {pressure={0.55,0.22}, ramps={0.2,0.35}})
b2:stroke({{294,326},{282,360},{275,392}}, {pressure={0.5,0.3}, ramps={0.2,0.3}})
b2:wipe(0.5); b2:load(plb, 0.55)
b2:stroke({{297,228},{294,262},{293,298}}, {pressure={0.45,0.4}})
b3 = brush{kind="round", width=7, point=0.5}
b3:load(pile{{"lead white",3},{"pale smalt",0.25}}, 0.8)
b3:stroke({{280,404},{281,444},{283,486}}, {pressure={0.5,0.28}})
b2:wipe(0.8); b2:load(pmb, 0.45)
b2:stroke({{345,460},{343,520},{341,566}}, {pressure={0.3,0.25}})
b2:load(pdb, 0.8)
b2:stroke({{340,600},{300,612},{262,600}}, {pressure={0.5,0.4}})
blend(bottlemask, {seed=511, coverage=0.7, pressure={0.25,0.25}})

--@ chunk 50
plLit = pile{{"lead white",3.5},{"chrome yellow",1.6}}
plDark = pile{{"yellow ochre",1.6},{"raw umber",1.3},{"green earth",0.8},{"smalt",0.4}}
sA = body.ellipsoid({600,498,0},{54,48,52})
sB = body.ellipsoid({700,504,0},{50,44,48})
fL = form{{sA,dist=0.3},{sB,dist=0.3}, light={from={-1,-0.75}, front=0.55, ambient=0.22}}
work(lemA * fL:shadow{parts={1}}, {hand="body", pile=plDark, clip=true, coverage=2.4, seed=521})
work(lemA * fL:lit{parts={1}, soft=0.15}, {hand="body", pile=plLit, clip=true, coverage=2.2, seed=522})
work(lemB * fL:shadow{parts={2}}, {hand="body", pile=plDark, clip=true, coverage=2.4, seed=523})
work(lemB * fL:lit{parts={2}, soft=0.15}, {hand="body", pile=plLit, clip=true, coverage=2.2, seed=524})
b4 = brush{kind="round", width=9, point=0.7}
b4:load(pile{{"lead white",3}}, 0.8)
b4:touch(578,478,{pressure=0.5, drag={0.6,0.5}})
b4:touch(682,486,{pressure=0.45, drag={0.5,0.4}})
rlA = grad(600,498,62,50, 0.7, 0.8, 0.5, 1.0)
work(lemA*rlA, {hand="body", pile=pile{{"yellow ochre",1.5},{"lead white",1.5},{"red earth",0.3}}, clip=true, coverage=1.4, seed=525})

--@ chunk 51
wfree = wallmask - objall
wdark_top = mask(function(x,y) return clamp(1 - (y/455)*1.5, 0, 1) end):soften(90)
work(wdark_top * wfree, {hand="broad", pile=pwall4, clip=true, coverage=2.0, angle=0.1, seed=531})
pwlight2 = pile{{"lead white",2},{"yellow ochre",1.2},{"raw umber",1}}
wlight_low = mask(function(x,y) return clamp(1 - (455-y)/150, 0, 1) end):soften(60)
work(wlight_low * wfree, {hand="broad", pile=pwlight2, clip=true, coverage=1.7, angle=0.1, seed=532})

--@ chunk 52
owym = bowlmask - open
pin2 = pile{{"raw umber",2},{"lead white",2},{"smalt",1.2}}
work(owym, {hand="body", pile=pbowl, clip=true, coverage=3.2, fill=true, angle=0.3, seed=541})
gright2 = grad(640,600,128,95, 0.95, 0.15, 0.28, 1.0) * owym
work(gright2, {hand="body", pile=pbd, clip=true, coverage=2.0, angle=1.1, seed=542})
blend(owym, {seed=543, coverage=1.3, pressure={0.3,0.3}})
grefl2 = grad(640,628,128,80, 0.1, 0.95, 0.45, 1.0) * owym
work(grefl2, {hand="body", pile=pbr, clip=true, coverage=1.5, angle=0.1, seed=544})
work(open_in, {hand="body", pile=pin2, clip=true, coverage=3.0, fill=true, angle=0.2, seed=545})
blend(open_in, {seed=546, coverage=1.2, pressure={0.3,0.3}})
sring = ((open_in - open_in:shrink(9)):soften(4)) * bowlmask
work(sring, {hand="body", pile=pin2, clip=true, coverage=1.6, seed=547})

--@ chunk 53
sC = body.ellipsoid({126,644,0},{54,36,42})
fC = form{{sC,dist=0.3}, light={from={-1,-0.75}, front=0.55, ambient=0.22}}
work(lemC * fC:shadow{parts={1}}, {hand="body", pile=plDark, clip=true, coverage=2.4, seed=551})
work(lemC * fC:lit{parts={1}, soft=0.15}, {hand="body", pile=plLit, clip=true, coverage=2.2, seed=552})
rlC = grad(126,644,56,40, 0.7, 0.85, 0.5, 1.0)
work(lemC*rlC, {hand="body", pile=pile{{"yellow ochre",1.5},{"lead white",1.4},{"red earth",0.3}}, clip=true, coverage=1.4, seed=553})
b5 = brush{kind="round", width=8, point=0.7}
b5:load(pile{{"lead white",3}}, 0.8)
b5:touch(108,626,{pressure=0.5, drag={0.6,0.5}})
-- pear: highlight and blush
b5:wipe(0.4); b5:load(pile{{"lead white",3},{"chrome yellow",0.4}}, 0.7)
b5:touch(828,576,{pressure=0.45, drag={0.5,0.7}})
blush = grad(852,600,48,60, -0.8, -0.4, 0.45, 1.0) * pear
work(blush, {hand="body", pile=pile{{"red earth",1},{"yellow ochre",0.8},{"vermilion",0.2}}, clip=true, coverage=1.2, seed=554})
-- bowl rim highlight on the left
b6 = brush{kind="filbert", width=9, stiffness=0.5}
b6:load(pile{{"lead white",4},{"yellow ochre",0.3}}, 0.75)
b6:stroke({{524,556},{540,568},{562,576}}, {pressure={0.5,0.4}, ramps={0.15,0.3}})

--@ chunk 54
pwsh = pile{{"raw umber",2},{"smalt",1},{"bone black",0.55}, medium=0.2}
wsh = mask(function(x,y)
  local cx = 402 + 0.14*(y-300)
  local d = math.abs(x - cx) / 98
  local v = clamp(1 - d*d, 0, 1)
  local h = clamp(1 - ((y-352)/155)^2, 0, 1)
  return v * h
end):soften(45)
work(wsh * (wallmask - objall), {hand="glaze", pile=pwsh, clip=true, coverage=1.8, seed=561})
wsh2 = mask(function(x,y)
  local cx = 500 + 0.10*(y-300)
  local d = math.abs(x - cx) / 70
  local v = clamp(1 - d*d, 0, 1)
  local h = clamp(1 - ((y-380)/120)^2, 0, 1)
  return v * h
end):soften(50)
work(wsh2 * (wallmask - objall), {hand="glaze", pile=pwsh, clip=true, coverage=1.2, seed=562})
blend((wsh + wsh2):soften(60) * (wallmask - objall), {seed=563, coverage=1.2, pressure={0.3,0.3}})

--@ chunk 55
pfront = pile{{"raw umber",2.5},{"bone black",1},{"smalt",1}}
tfree = tablemask - objall
bottom = mask(function(x,y) return clamp((y-690)/115, 0, 1) end):soften(55) * tfree
work(bottom, {hand="broad", pile=pfront, clip=true, coverage=2.6, angle=0.03, seed=571})
leftlow = mask(function(x,y) return clamp(1 - x/330, 0, 1) end):soften(70) * tfree
work(leftlow, {hand="broad", pile=ptabDark2, clip=true, coverage=1.6, angle=0.03, seed=572})
wfree = wallmask - objall
topv = mask(function(x,y) return clamp(1 - y/280, 0, 1) end):soften(90) * wfree
work(topv, {hand="broad", pile=pwall4, clip=true, coverage=1.6, angle=0.12, seed=573})

--@ chunk 56
wfree = wallmask - objall
tfree = tablemask - objall
pcool = pile{{"smalt",1},{"lead white",1.2},{"raw umber",0.6}, medium=0.6}
pwarmt = pile{{"yellow ochre",2},{"red earth",0.5},{"lead white",0.6}, medium=0.5}
work(wfree, {hand="glaze", pile=pcool, clip=true, coverage=1.3, seed=581})
work(tfree, {hand="glaze", pile=pwarmt, clip=true, coverage=1.3, seed=582})

--@ chunk 57
wfree = wallmask - objall
blend(wfree, {seed=591, coverage=3.0, pressure={0.32,0.32}})
blend(wfree, {seed=592, coverage=2.0, pressure={0.28,0.28}})

--@ chunk 58
pwm = pile{{"raw umber",3.5},{"smalt",1.3},{"red earth",0.5},{"lead white",1.0}}
wfree = wallmask - objall
work(wfree, {hand="broad", pile=pwm, clip=true, coverage=3.4, angle=0.08, fill=true, dips={3,0.85}, seed=601})
blend(wfree, {seed=602, coverage=2.5, pressure={0.32,0.32}})
blend(wfree, {seed=603, coverage=1.5, pressure={0.28,0.28}})

--@ chunk 59
plL = pile{{"chrome yellow",2},{"lead white",2}}
plD = pile{{"yellow ochre",1.5},{"raw umber",1.4},{"green earth",0.9},{"smalt",0.5}}
sA = body.ellipsoid({600,498,0},{54,48,52})
sB = body.ellipsoid({700,504,0},{50,44,48})
fL = form{{sA,dist=0.3},{sB,dist=0.3}, light={from={-1,-0.75}, front=0.55, ambient=0.22}}
work(lemA * fL:shadow{parts={1}}, {hand="body", pile=plD, clip=true, coverage=3.0, seed=611})
work(lemA * fL:lit{parts={1}, soft=0.12}, {hand="body", pile=plL, clip=true, coverage=2.4, seed=612})
work(lemB * fL:shadow{parts={2}}, {hand="body", pile=plD, clip=true, coverage=3.0, seed=613})
work(lemB * fL:lit{parts={2}, soft=0.12}, {hand="body", pile=plL, clip=true, coverage=2.4, seed=614})
rlA = grad(600,498,62,50, 0.5, 1.0, 0.55, 1.0)
work(lemA*rlA, {hand="body", pile=pile{{"yellow ochre",1.6},{"lead white",1.2},{"vermilion",0.15}}, clip=true, coverage=1.6, seed=615})
b7 = brush{kind="round", width=10, point=0.8}
b7:load(pile{{"lead white",4}}, 0.85)
b7:touch(576,474,{pressure=0.55, drag={0.7,0.6}})
b7:wipe(0.3); b7:load(pile{{"lead white",4}}, 0.7)
b7:touch(680,482,{pressure=0.5, drag={0.6,0.5}})

--@ chunk 60
tfree = tablemask - objall
plight_t = pile{{"yellow ochre",3},{"lead white",2.5},{"raw umber",0.4}}
pool = ellipse(300,600,330,155):soften(105) * tfree
work(pool, {hand="broad", pile=plight_t, clip=true, coverage=2.2, angle=0.05, seed=621})
-- far right darker
rdark = mask(function(x,y) return clamp((x-700)/300, 0, 1) end):soften(70) * tfree
work(rdark, {hand="broad", pile=pfront, clip=true, coverage=2.0, angle=0.05, seed=622})
-- clearer cast shadows
pS3 = pile{{"raw umber",1.8},{"bone black",1.2},{"smalt",1.1},{"lead white",0.35}, medium=0.1}
sh_bot = castshadow(300,616,640,662,58,16)
sh_bowl = castshadow(640,644,900,680,78,26)
sh_lem = castshadow(140,666,340,688,38,12)
sh_pear2 = castshadow(848,660,1000,678,44,16)
for i,m in ipairs{sh_bot, sh_bowl, sh_lem, sh_pear2} do
  work((m:soften(10)) * tfree, {hand="body", pile=pS3, clip=true, coverage=3.0, seed=630+i})
end
contact = ellipse(302,616,84,20):soften(9) + ellipse(640,644,112,22):soften(10)
        + ellipse(128,678,60,17):soften(8) + ellipse(852,660,58,17):soften(8)
work(contact * tfree, {hand="body", pile=pS3, clip=true, coverage=3.0, seed=641})
blend((sh_bot + sh_bowl + sh_lem + sh_pear2):soften(16) * tfree, {seed=642, coverage=1.4, pressure={0.3,0.3}})

--@ chunk 61
b8 = brush{kind="filbert", width=11, stiffness=0.55}
b8:load(pile{{"lead white",2.5},{"green earth",0.5},{"pale smalt",0.6}}, 0.8)
b8:stroke({{272,398},{268,450},{266,510},{268,560},{272,594}}, {pressure={0.4,0.3}, ramps={0.2,0.3}})
b9 = brush{kind="round", width=4, point=0.5}
b9:load(pile{{"lead white",4}}, 0.9)
b9:stroke({{276,420},{274,470},{274,520},{276,560}}, {pressure={0.3,0.25}})
b9:wipe(0.5); b9:load(pile{{"lead white",3.5},{"pale smalt",0.3}}, 0.8)
b9:stroke({{295,240},{293,270},{292,296}}, {pressure={0.3,0.3}})
b9:touch(296,222,{pressure=0.4})
-- bowl rim highlight, front-left
rimhl = ribbon({{522,543},{532,550},{549,556},{572,561},{599,565}}, {9,9,8,7,6}) * bowlmask
work(rimhl, {hand="body", pile=pile{{"lead white",4},{"yellow ochre",0.4}}, clip=true, coverage=2.4, seed=651})
-- left lemon highlight + nub
b10 = brush{kind="round", width=9, point=0.7}
b10:load(pile{{"lead white",3.2},{"chrome yellow",0.5}}, 0.85)
b10:touch(106,626,{pressure=0.5, drag={0.6,0.5}})
b10:load(pile{{"yellow ochre",2},{"raw umber",0.6}}, 0.6)
b10:touch(72,642,{pressure=0.35})

--@ chunk 62
alllem = lemA + lemB + lemC
stipple(alllem, {pile=pile{{"yellow ochre",1},{"lead white",1.2}}, width=5, coverage=0.9,
  pressure={0.12,0.28}, feather=0.6, cluster={0.5,20}, seed=671})
stipple(alllem, {pile=pile{{"raw umber",0.7},{"yellow ochre",1.4}}, width=3.5, coverage=0.5,
  pressure={0.1,0.22}, feather=0.7, seed=672})

--@ chunk 63
wfree = wallmask - objall
wm_dark = mask(function(x,y)
  return clamp(0.15 + 0.85*(1 - y/455) + 0.45*(x/1000), 0, 1)
end):soften(85) * wfree
work(wm_dark, {hand="broad", pile=pwall4, clip=true, coverage=2.4, angle=0.1, seed=681})
blend((wm_dark + (wfree - wm_dark)):soften(20) * wfree, {seed=682, coverage=1.0, pressure={0.28,0.28}})

--@ chunk 64
work(bottlemask, {hand="body", pile=pbot, clip=true, coverage=3.6, angle=1.5, fill=true, dips={3,0.85}, seed=691})

--@ chunk 65
pgl = pile{{"lead white",2.2},{"green earth",0.6},{"pale smalt",0.7}}
b11 = brush{kind="round", width=9, point=0.45, stiffness=0.6}
b11:load(pgl, 0.85)
b11:stroke({{276,398},{270,450},{267,510},{269,560},{273,592}}, {pressure={0.5,0.35}, ramps={0.15,0.2}})
b11:stroke({{298,322},{285,352},{276,388}}, {pressure={0.5,0.42}, ramps={0.15,0.2}})
b11:wipe(0.4); b11:load(pgl, 0.7)
b11:stroke({{297,232},{294,266},{292,300}}, {pressure={0.4,0.4}})
b11:touch(299,223,{pressure=0.45})
b12 = brush{kind="round", width=3.5, point=0.6}
b12:load(pile{{"lead white",4}}, 0.9)
b12:stroke({{275,420},{271,480},{271,540},{274,580}}, {pressure={0.3,0.25}})
b12:wipe(0.5); b12:load(pile{{"lead white",2},{"pale smalt",0.6},{"green earth",0.4}}, 0.8)
b12:stroke({{347,472},{344,520},{343,566}}, {pressure={0.3,0.3}})
b13 = brush{kind="flat", width=20, stiffness=0.6}
b13:load(pile{{"bone black",1},{"green earth",0.5}}, 0.75)
b13:stroke({{262,596},{300,604},{340,596}}, {pressure={0.6,0.6}})

--@ chunk 66
print(wait(2*1440))
print("bottle", drying(300,500), "bowl", drying(640,560), "lemon", drying(600,490))

--@ chunk 67
print(wait(7*1440))
print("bottle", drying(300,500), "bowl", drying(640,560), "lemon", drying(600,490), "pear", drying(852,600))

--@ chunk 68
print(wait(14*1440))
print("bottle", drying(300,500), drying(300,300), drying(300,580))

--@ chunk 69
phl = pile{{"lead white",4},{"pale smalt",0.35},{"green earth",0.25}}
phl2 = pile{{"lead white",2},{"pale smalt",0.8},{"green earth",0.5}}
b = brush{kind="round", width=10, point=0.4, stiffness=0.7}
b:load(phl2, 0.9)
b:stroke({{277,398},{271,450},{268,505},{269,555},{273,590}}, {pressure={0.5,0.3}, ramps={0.12,0.18}})
b:stroke({{299,322},{288,350},{279,386}}, {pressure={0.5,0.4}, ramps={0.12,0.15}})
b:wipe(0.4); b:load(phl, 0.8)
b:stroke({{298,232},{295,264},{293,298}}, {pressure={0.35,0.4}})
b:touch(300,224,{pressure=0.4, drag={0.4,-0.2}})
b2 = brush{kind="round", width=4, point=0.7}
b2:load(pile{{"lead white",4}}, 0.95)
b2:stroke({{276,424},{272,470},{271,520},{273,560}}, {pressure={0.28,0.22}})
b2:wipe(0.4); b2:load(pile{{"lead white",1.6},{"pale smalt",1},{"green earth",0.5}}, 0.85)
b2:stroke({{347,470},{345,520},{344,566}}, {pressure={0.25,0.25}})
b3 = brush{kind="flat", width=22, stiffness=0.6}
b3:load(pile{{"bone black",1},{"green earth",0.4}}, 0.7)
b3:stroke({{264,598},{300,606},{338,598}}, {pressure={0.6,0.55}})

--@ chunk 70
rimhl = ribbon({{522,543},{532,550},{549,556},{572,561},{599,565}}, {9,9,8,7,6}) * bowlmask
work(rimhl, {hand="body", pile=pile{{"lead white",4},{"yellow ochre",0.35}}, clip=true, coverage=2.4, seed=701})
rimhr = ribbon({{681,565},{708,561},{731,556},{748,550},{758,543}}, {6,7,8,9,9}) * bowlmask
work(rimhr, {hand="body", pile=pbd, clip=true, coverage=1.8, seed=702})
b4 = brush{kind="round", width=8, point=0.75}
b4:load(pile{{"lead white",4}}, 0.9)
b4:touch(578,472,{pressure=0.5, drag={0.6,0.5}})
b4:wipe(0.4); b4:load(pile{{"lead white",4}}, 0.85)
b4:touch(680,480,{pressure=0.45, drag={0.5,0.4}})
b4:wipe(0.4); b4:load(pile{{"lead white",3.5}}, 0.85)
b4:touch(108,624,{pressure=0.45, drag={0.5,0.5}})
b4:wipe(0.4); b4:load(pile{{"lead white",3}}, 0.8)
b4:touch(828,570,{pressure=0.4, drag={0.4,0.6}})

--@ chunk 71
pjl = pile{{"raw umber",2},{"smalt",1.2},{"bone black",0.4}, medium=0.4}
pjh = pile{{"lead white",2},{"yellow ochre",1.2},{"raw umber",0.5}}
bj = brush{kind="round", width=7, point=0.3, stiffness=0.5}
bj:load(pjl, 0.5)
bj:stroke({{0,446},{120,448},{250,451},{380,453},{500,455},{620,458},{740,461},{860,464},{1000,468}}, {pressure={0.4,0.35}, ramps={0.05,0.1}})
bj:wipe(0.55); bj:load(pjh, 0.45)
bj:stroke({{0,451},{120,453},{250,456},{380,458},{500,460},{620,463},{740,466},{860,469},{1000,473}}, {pressure={0.35,0.3}, ramps={0.05,0.1}})
-- cool the shadows, warm the light pool
tfree = tablemask - objall
shm = (sh_bot + sh_bowl + sh_lem + sh_pear2):soften(16) * tfree
work(shm, {hand="glaze", pile=pile{{"smalt",1},{"red earth",0.25},{"lead white",0.5}, medium=0.5}, clip=true, coverage=1.2, seed=711})
pool = ellipse(300,600,330,155):soften(105) * tfree
work(pool, {hand="glaze", pile=pile{{"yellow ochre",2},{"vermilion",0.15},{"lead white",0.5}, medium=0.5}, clip=true, coverage=1.1, seed=712})

--@ chunk 72
tfree = tablemask - objall
ptabBase2 = pile{{"yellow ochre",2.5},{"raw umber",1.6},{"lead white",1.3},{"red earth",0.3}}
work(tfree, {hand="broad", pile=ptabBase2, clip=true, coverage=3.4, angle=0.05, fill=true, dips={3,0.85}, seed=721})
plight_t = pile{{"yellow ochre",3},{"lead white",2.5},{"raw umber",0.4}}
pool = ellipse(300,600,330,155):soften(105) * tfree
work(pool, {hand="broad", pile=plight_t, clip=true, coverage=2.2, angle=0.05, seed=722})
rdark = mask(function(x,y) return clamp((x-700)/300,0,1) end):soften(70) * tfree
work(rdark, {hand="broad", pile=pfront, clip=true, coverage=2.0, angle=0.05, seed=723})
bottom = mask(function(x,y) return clamp((y-690)/115,0,1) end):soften(55) * tfree
work(bottom, {hand="broad", pile=pfront, clip=true, coverage=2.4, angle=0.03, seed=724})
pS3 = pile{{"raw umber",1.8},{"bone black",1.2},{"smalt",1.1},{"lead white",0.35}, medium=0.1}
for i,m in ipairs{sh_bot,sh_bowl,sh_lem,sh_pear2} do
  work((m:soften(10))*tfree, {hand="body", pile=pS3, clip=true, coverage=3.0, seed=730+i})
end
work(contact*tfree, {hand="body", pile=pS3, clip=true, coverage=3.0, seed=741})
blend((sh_bot+sh_bowl+sh_lem+sh_pear2):soften(16)*tfree, {seed=742, coverage=1.4, pressure={0.3,0.3}})
pjl = pile{{"raw umber",2},{"smalt",1.2},{"bone black",0.4}, medium=0.4}
pjh = pile{{"lead white",2},{"yellow ochre",1.2},{"raw umber",0.5}}
bj = brush{kind="round", width=7, point=0.3, stiffness=0.5}
bj:load(pjl, 0.5)
bj:stroke({{0,446},{120,448},{250,451},{380,453},{500,455},{620,458},{740,461},{860,464},{1000,468}}, {pressure={0.4,0.35}, ramps={0.05,0.1}})
bj:wipe(0.55); bj:load(pjh, 0.45)
bj:stroke({{0,451},{120,453},{250,456},{380,458},{500,460},{620,463},{740,466},{860,469},{1000,473}}, {pressure={0.35,0.3}, ramps={0.05,0.1}})

--@ chunk 73
plemon = pile{{"chrome yellow",3},{"lead white",1.5},{"yellow ochre",0.6}}
plL = pile{{"chrome yellow",2},{"lead white",2}}
plD = pile{{"yellow ochre",1.5},{"raw umber",1.4},{"green earth",0.9},{"smalt",0.5}}
work(lemA + lemB, {hand="body", pile=plemon, clip=true, coverage=3.2, fill=true, angle=0.3, seed=751})
sA = body.ellipsoid({600,498,0},{54,48,52})
sB = body.ellipsoid({700,504,0},{50,44,48})
fL = form{{sA,dist=0.3},{sB,dist=0.3}, light={from={-1,-0.75}, front=0.55, ambient=0.22}}
work(lemA*fL:shadow{parts={1}}, {hand="body", pile=plD, clip=true, coverage=2.6, seed=752})
work(lemA*fL:lit{parts={1}, soft=0.15}, {hand="body", pile=plL, clip=true, coverage=2.2, seed=753})
work(lemB*fL:shadow{parts={2}}, {hand="body", pile=plD, clip=true, coverage=2.6, seed=754})
work(lemB*fL:lit{parts={2}, soft=0.15}, {hand="body", pile=plL, clip=true, coverage=2.2, seed=755})
pin3 = pile{{"raw umber",2.2},{"lead white",1.6},{"smalt",1}}
pin_dark = pile{{"raw umber",2.5},{"smalt",1.2},{"lead white",0.8}}
work(open_in, {hand="body", pile=pin3, clip=true, coverage=3.2, fill=true, angle=0.2, seed=756})
blend(open_in, {seed=757, coverage=1.5, pressure={0.3,0.3}})
floor = ((ellipse(600,556,72,16) + ellipse(700,562,60,15)):soften(10)) * open_in
work(floor, {hand="body", pile=pin_dark, clip=true, coverage=2.2, seed=758})
b4 = brush{kind="round", width=8, point=0.75}
b4:load(pile{{"lead white",4}}, 0.9)
b4:touch(578,472,{pressure=0.5, drag={0.6,0.5}})
b4:wipe(0.4); b4:load(pile{{"lead white",4}}, 0.85)
b4:touch(680,480,{pressure=0.45, drag={0.5,0.4}})

--@ chunk 74
work(lemA + lemB, {hand="body", pile=plemon, clip=true, coverage=3.2, fill=true, angle=0.3, seed=761})
work(lemA*fL:shadow{parts={1}}, {hand="body", pile=plD, clip=true, coverage=2.6, seed=762})
work(lemA*fL:lit{parts={1}, soft=0.15}, {hand="body", pile=plL, clip=true, coverage=2.2, seed=763})
work(lemB*fL:shadow{parts={2}}, {hand="body", pile=plD, clip=true, coverage=2.6, seed=764})
work(lemB*fL:lit{parts={2}, soft=0.15}, {hand="body", pile=plL, clip=true, coverage=2.2, seed=765})
floor2 = ((ellipse(600,556,74,16) + ellipse(700,562,62,15)):soften(9)) * open_in * (-(lemA + lemB))
work(floor2, {hand="body", pile=pin_dark, clip=true, coverage=2.4, seed=766})
b4 = brush{kind="round", width=8, point=0.75}
b4:load(pile{{"lead white",4}}, 0.9)
b4:touch(578,472,{pressure=0.5, drag={0.6,0.5}})
b4:wipe(0.4); b4:load(pile{{"lead white",4}}, 0.85)
b4:touch(680,480,{pressure=0.45, drag={0.5,0.4}})

--@ chunk 75
blend(lemA+lemB, {seed=771, coverage=1.4, pressure={0.31,0.31}})
work(lemC, {hand="body", pile=plemon, clip=true, coverage=3.2, fill=true, angle=0.25, seed=781})
sC = body.ellipsoid({126,644,0},{54,36,42})
fC = form{{sC,dist=0.3}, light={from={-1,-0.75}, front=0.55, ambient=0.22}}
work(lemC*fC:shadow{parts={1}}, {hand="body", pile=plD, clip=true, coverage=2.6, seed=782})
work(lemC*fC:lit{parts={1}, soft=0.15}, {hand="body", pile=plL, clip=true, coverage=2.2, seed=783})
rlC = grad(126,644,58,40, 0.55, 0.9, 0.55, 1.0)
work(lemC*rlC, {hand="body", pile=pile{{"yellow ochre",1.6},{"lead white",1.2},{"vermilion",0.15}}, clip=true, coverage=1.5, seed=784})
b4 = brush{kind="round", width=8, point=0.75}
b4:load(pile{{"lead white",4}}, 0.9)
b4:touch(578,472,{pressure=0.5, drag={0.6,0.5}})
b4:wipe(0.4); b4:load(pile{{"lead white",4}}, 0.85)
b4:touch(680,480,{pressure=0.45, drag={0.5,0.4}})
b4:wipe(0.4); b4:load(pile{{"lead white",3.5}}, 0.85)
b4:touch(106,622,{pressure=0.45, drag={0.5,0.5}})

--@ chunk 76
tfree = tablemask - objall
wfree = wallmask - objall
pconn = pile{{"raw umber",1.6},{"bone black",1.4},{"smalt",1},{"lead white",0.2}}
contact2 = ellipse(302,616,80,17):soften(10) + ellipse(640,646,116,20):soften(11)
        + ellipse(128,680,58,15):soften(9) + ellipse(852,662,56,15):soften(9)
work(contact2 * tfree, {hand="body", pile=pconn, clip=true, coverage=2.6, seed=801})
-- fruit core shadows at the base
coreL = (grad(600,498,54,48, 0.25, 1.0, 0.62, 1.0) * lemA) + (grad(700,504,50,44, 0.25, 1.0, 0.62, 1.0) * lemB)
work(coreL, {hand="body", pile=pile{{"raw umber",1.4},{"yellow ochre",1.2},{"green earth",0.6}}, clip=true, coverage=1.6, seed=802})
-- warm glow on the wall behind the objects
pgw = pile{{"lead white",1.2},{"yellow ochre",1.2},{"raw umber",1}}
gw = ellipse(480,430,430,225):soften(140) * wfree
work(gw, {hand="broad", pile=pgw, clip=true, coverage=1.2, angle=0.1, seed=803})

--@ chunk 77
wfree = wallmask - objall
pwmid = pile{{"raw umber",3.2},{"smalt",1.2},{"red earth",0.4},{"lead white",1.2}}
work(wfree, {hand="broad", pile=pwmid, clip=true, coverage=3.6, angle=0.1, fill=true, dips={3,0.9}, seed=811})
blend(wfree, {seed=812, coverage=3.0, pressure={0.32,0.32}})
blend(wfree, {seed=813, coverage=1.5, pressure={0.28,0.28}})

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
