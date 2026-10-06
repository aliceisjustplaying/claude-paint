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

--@ chunk 78
print(wait(0)); print("bottle", drying(300,500)); print("bowlOut", drying(640,600)); print("wall", drying(500,200)); print("table", drying(300,700)); print("pear", drying(852,600)); print("lemC", drying(126,644))

--@ chunk 79
-- repaint the bottle cleanly, killing the table-edge seam across it
pbot2 = pile{{"bone black",1},{"green earth",1.3},{"raw umber",0.3},{"Prussian blue",0.08}}
work(bottlemask, {hand="body", pile=pbot2, clip=true, coverage=3.8, angle=1.5, fill=true, dips={3,0.9}, seed=901})
-- broad pale window-reflection down the left of the glass
lref = grad(300,420,120,300, -1.0, -0.05, 0.30, 1.0) * bottlemask
plref = pile{{"lead white",1.8},{"pale smalt",0.9},{"green earth",0.6}}
work(lref, {hand="body", pile=plref, clip=true, coverage=1.5, angle=1.5, seed=902})
-- warm light picked up low on the right from the table
rref = grad(302,560,110,150, 1.0, 0.35, 0.35, 1.0) * bottlemask
wr = pile{{"lead white",1.1},{"yellow ochre",1.2},{"raw umber",1.0}, medium=0.3}
work(rref, {hand="glaze", pile=wr, clip=true, coverage=1.3, seed=903})
blend(bottlemask, {seed=904, coverage=1.1, pressure={0.26,0.26}})
-- the speculars
b = brush{kind="round", width=6, point=0.6, stiffness=0.55}
b:load(pile{{"lead white",4}}, 0.9)
b:stroke({{277,404},{273,452},{271,505},{272,555},{275,588}}, {pressure={0.42,0.26}, ramps={0.15,0.28}})
b:wipe(0.45); b:load(pile{{"lead white",4}}, 0.82)
b:stroke({{298,232},{295,262},{293,296}}, {pressure={0.30,0.32}})
b:touch(299,224,{pressure=0.4, drag={0.4,-0.2}})
b2 = brush{kind="round", width=3, point=0.7}
b2:load(pile{{"lead white",1.4},{"pale smalt",1.0},{"green earth",0.5}}, 0.8)
b2:stroke({{347,470},{345,520},{344,566}}, {pressure={0.24,0.24}})

--@ chunk 80
-- the reflection read too broad and grey: darken the glass again, keep a narrow window-light on the left
pbot3 = pile{{"bone black",1.2},{"green earth",1.0},{"Prussian blue",0.06},{"raw umber",0.25}}
work(bottlemask, {hand="body", pile=pbot3, clip=true, coverage=4.0, angle=1.5, fill=true, dips={3,0.95}, seed=911})
lref2 = grad(285,420,100,320, -1.0, 0.0, 0.45, 1.0) * bottlemask
work(lref2, {hand="glaze", pile=pile{{"lead white",1.2},{"pale smalt",0.9},{"green earth",0.5}, medium=0.35}, clip=true, coverage=1.3, seed=912})
rref2 = grad(300,555,110,150, 1.0, 0.4, 0.45, 1.0) * bottlemask
work(rref2, {hand="glaze", pile=pile{{"lead white",1.0},{"yellow ochre",1.1},{"raw umber",0.9}, medium=0.3}, clip=true, coverage=1.1, seed=913})
blend(bottlemask, {seed=914, coverage=0.9, pressure={0.25,0.25}})
b = brush{kind="round", width=6, point=0.6, stiffness=0.55}
b:load(pile{{"lead white",4}}, 0.9)
b:stroke({{278,404},{274,452},{272,505},{273,555},{276,588}}, {pressure={0.40,0.26}, ramps={0.15,0.28}})
b:wipe(0.45); b:load(pile{{"lead white",4}}, 0.82)
b:stroke({{298,232},{295,262},{293,296}}, {pressure={0.30,0.32}})
b:touch(299,224,{pressure=0.4, drag={0.4,-0.2}})
b2 = brush{kind="round", width=3, point=0.7}
b2:load(pile{{"lead white",1.3},{"pale smalt",0.9},{"green earth",0.5}}, 0.8)
b2:stroke({{347,470},{345,520},{344,566}}, {pressure={0.22,0.22}})

--@ chunk 81
-- the pear was transparent: give it a solid body again and model it
porear = pile{{"lead white",1.6},{"yellow ochre",1.6},{"green earth",1.1},{"chrome yellow",0.5}}
work(pear, {hand="body", pile=porear, clip=true, coverage=3.8, angle=0.25, fill=true, dips={3,0.9}, seed=921})
work(pear*f2:lit{parts={1}, soft=0.2}, {hand="body", pile=plightp, clip=true, coverage=2.1, seed=922})
work(pear*f2:shadow{parts={1}}, {hand="body", pile=pdarkp, clip=true, coverage=1.9, seed=923})
necklit = grad(852,548,32,44, -0.55, -0.9, 0.30, 1.0) * pear
work(necklit, {hand="body", pile=plightp, clip=true, coverage=1.3, seed=924})
blush2 = grad(852,600,48,60, -0.7, -0.45, 0.5, 1.0) * pear
work(blush2, {hand="body", pile=pile{{"red earth",1},{"yellow ochre",0.9},{"vermilion",0.25}}, clip=true, coverage=1.1, seed=925})
blend(pear, {seed=926, coverage=0.9, pressure={0.28,0.28}})
b = brush{kind="round", width=7, point=0.7}
b:load(pile{{"lead white",3},{"chrome yellow",0.4}}, 0.8)
b:touch(835,566,{pressure=0.45, drag={0.4,0.5}})

--@ chunk 82
-- the blend was thinning the pear and letting the table up: lay it in opaque and leave it
porear2 = pile{{"lead white",3},{"yellow ochre",1.4},{"chrome yellow",0.5},{"green earth",0.3}}
work(pear, {hand="body", pile=porear2, clip=true, coverage=4.6, angle=0.25, fill=true, dips={2,1.0}, seed=931})
work(pear*f2:lit{parts={1}, soft=0.25}, {hand="body", pile=plightp, clip=true, coverage=1.8, angle=-0.35, seed=932})
work(pear*f2:shadow{parts={1}}, {hand="body", pile=pdarkp, clip=true, coverage=1.7, angle=-0.35, seed=933})
necklit2 = grad(852,548,32,44, -0.55, -0.9, 0.25, 1.0) * pear
work(necklit2, {hand="body", pile=plightp, clip=true, coverage=1.3, seed=934})
blush3 = grad(852,600,46,58, -0.65, -0.5, 0.55, 1.0) * pear
work(blush3, {hand="body", pile=pile{{"red earth",1},{"yellow ochre",0.9},{"vermilion",0.22}}, clip=true, coverage=1.0, seed=935})
b = brush{kind="round", width=7, point=0.7}
b:load(pile{{"lead white",3},{"chrome yellow",0.4}}, 0.8)
b:touch(835,566,{pressure=0.45, drag={0.4,0.5}})
print("pear area", pear:area())

--@ chunk 83
print("pear at 852,600", pear:at(852,600)); print("pear at 852,540", pear:at(852,540)); print("pear at 820,600", pear:at(820,600))

--@ chunk 84
print(wait(5*1440)); print("bottle",drying(300,500),"pear",drying(852,600),"table",drying(500,700),"wall",drying(500,200),"bowl",drying(640,600),"lemC",drying(126,644))

--@ chunk 85
-- pear, now on dry ground: one clean body, a warm shadow, no olive
pPearBase = pile{{"lead white",3.2},{"yellow ochre",1.4},{"chrome yellow",0.4}}
work(pear, {hand="broad", pile=pPearBase, clip=true, coverage=3.0, angle=0.15, fill=true, dips={2,1.0}, seed=951})
shp2 = grad(852,600,52,64, 0.8, 0.5, 0.30, 1.0) * pear
work(shp2, {hand="broad", pile=pile{{"yellow ochre",2},{"lead white",0.9},{"raw umber",0.8},{"red earth",0.2}}, clip=true, coverage=2.2, angle=0.15, seed=952})
litp2 = grad(852,600,52,64, -0.7, -0.6, 0.20, 1.0) * pear
work(litp2, {hand="broad", pile=pile{{"lead white",3},{"chrome yellow",1},{"yellow ochre",0.4}}, clip=true, coverage=2.2, angle=0.15, seed=953})
necklit3 = grad(852,548,32,44, -0.55, -0.9, 0.15, 1.0) * pear
work(necklit3, {hand="broad", pile=plightp, clip=true, coverage=1.4, seed=954})
core = ellipse(852,650,46,20):soften(8) * pear
work(core, {hand="body", pile=pile{{"raw umber",1.6},{"yellow ochre",1.2},{"green earth",0.4}}, clip=true, coverage=1.6, seed=955})
blend(pear, {seed=956, coverage=0.8, pressure={0.28,0.28}})
blush4 = grad(852,600,46,58, -0.55, -0.6, 0.62, 1.0) * pear
work(blush4, {hand="body", pile=pile{{"red earth",1},{"yellow ochre",0.9},{"vermilion",0.2}}, clip=true, coverage=0.9, seed=957})
b = brush{kind="round", width=7, point=0.7}
b:load(pile{{"lead white",3},{"chrome yellow",0.4}}, 0.8)
b:touch(835,564,{pressure=0.45, drag={0.35,0.5}})

--@ chunk 86
-- table, on dry ground: lay it in clean, then the light pool, the dark corners and the cast shadows
tfree = tablemask - objall
ptabB = pile{{"yellow ochre",2.2},{"raw umber",1.8},{"lead white",1.3},{"red earth",0.3}}
work(tfree, {hand="broad", pile=ptabB, clip=true, coverage=3.4, angle=0.05, fill=true, dips={3,0.88}, seed=1001})
pool = ellipse(300,590,340,150):soften(100) * tfree
plight_t = pile{{"yellow ochre",3},{"lead white",2.5},{"raw umber",0.4}}
work(pool, {hand="broad", pile=plight_t, clip=true, coverage=2.2, angle=0.05, seed=1002})
rdark = mask(function(x,y) return clamp((x-690)/310,0,1) end):soften(75) * tfree
work(rdark, {hand="broad", pile=pfront, clip=true, coverage=1.9, angle=0.05, seed=1003})
bottom = mask(function(x,y) return clamp((y-700)/100,0,1) end):soften(55) * tfree
work(bottom, {hand="broad", pile=pfront, clip=true, coverage=2.2, angle=0.03, seed=1004})
pS4 = pile{{"raw umber",1.8},{"bone black",1.1},{"smalt",1.1},{"lead white",0.4}, medium=0.12}
sh_bot = castshadow(300,616,640,662,56,16)
sh_bowl = castshadow(640,644,900,680,76,26)
sh_lem = castshadow(140,666,340,688,36,12)
sh_pear2 = castshadow(848,660,1000,678,42,16)
for i,m in ipairs{sh_bot,sh_bowl,sh_lem,sh_pear2} do
  work((m:soften(10))*tfree, {hand="body", pile=pS4, clip=true, coverage=2.8, seed=1010+i})
end
contact = ellipse(302,616,84,20):soften(9) + ellipse(640,644,112,22):soften(10)
        + ellipse(128,678,60,17):soften(8) + ellipse(852,662,58,17):soften(8)
work(contact*tfree, {hand="body", pile=pS4, clip=true, coverage=2.8, seed=1021})
blend((sh_bot+sh_bowl+sh_lem+sh_pear2):soften(18)*tfree, {seed=1022, coverage=1.2, pressure={0.3,0.3}})

--@ chunk 87
-- the fresh shadows scalloped: fuse them and lose the stroke ends
tfree = tablemask - objall
sm = sh_bot + sh_bowl + sh_lem + sh_pear2
blend((sm):grow(7):soften(18) * tfree, {seed=1031, coverage=3.4, pressure={0.34,0.34}})
blend((sm):grow(14):soften(26) * tfree, {seed=1032, coverage=2.0, pressure={0.3,0.3}})
blend(contact:grow(6):soften(12) * tfree, {seed=1033, coverage=2.4, pressure={0.32,0.32}})

--@ chunk 88
-- wall, on dry ground: one even dim brown, a warm glow low behind the bottle, deeper above and to the right
wfree = wallmask - objall
pwallF = pile{{"raw umber",3.2},{"smalt",1.2},{"red earth",0.4},{"lead white",0.9}}
work(wfree, {hand="broad", pile=pwallF, clip=true, coverage=3.2, angle=0.08, fill=true, dips={3,0.88}, seed=1101})
glow2 = mask(function(x,y)
  local d = math.sqrt(((x-360)/430)^2 + ((y-430)/200)^2)
  return clamp(1-d,0,1)
end):soften(110) * wfree
work(glow2, {hand="broad", pile=pile{{"lead white",1.5},{"yellow ochre",1.3},{"raw umber",1.0}}, clip=true, coverage=1.4, angle=0.08, seed=1102})
wd = mask(function(x,y) return clamp(0.85*(1 - y/340) + 0.4*(x/1000), 0, 1) end):soften(120) * wfree
work(wd, {hand="broad", pile=pwall4, clip=true, coverage=1.8, angle=0.1, seed=1103})
blend(wfree, {seed=1104, coverage=2.4, pressure={0.32,0.32}})
blend(wfree, {seed=1105, coverage=1.2, pressure={0.28,0.28}})

--@ chunk 89
-- the wall banded along the broad strokes: cross the passes and even it out
wfree = wallmask - objall
pwallF = pile{{"raw umber",3.2},{"smalt",1.2},{"red earth",0.4},{"lead white",1.0}}
work(wfree, {hand="broad", pile=pwallF, clip=true, coverage=2.0, angle=0.06, fill=true, dips={3,0.8}, seed=1121})
work(wfree, {hand="broad", pile=pwallF, clip=true, coverage=2.0, angle=1.55, fill=true, dips={3,0.8}, seed=1122})
glow3 = mask(function(x,y) local d=math.sqrt(((x-340)/430)^2+((y-440)/210)^2); return clamp(1-d,0,1) end):soften(120)*wfree
work(glow3, {hand="broad", pile=pile{{"lead white",1.4},{"yellow ochre",1.2},{"raw umber",1.0}}, clip=true, coverage=1.2, angle=1.55, fill=true, dips={3,0.8}, seed=1123})
wd2 = mask(function(x,y) return clamp(0.9*(1-y/340) + 0.35*(x/1000),0,1) end):soften(120)*wfree
work(wd2, {hand="broad", pile=pwall4, clip=true, coverage=1.4, angle=0.06, fill=true, dips={3,0.8}, seed=1124})
blend(wfree, {seed=1125, coverage=2.6, pressure={0.33,0.33}})
blend(wfree, {seed=1126, coverage=1.4, pressure={0.28,0.28}})

--@ chunk 90
-- the broad strokes keep banding: work the wall in short back-and-forth scumbles and fuse hard
wfree = wallmask - objall
pwallF = pile{{"raw umber",3.2},{"smalt",1.2},{"red earth",0.4},{"lead white",1.0}}
work(wfree, {hand="scumble", pile=pwallF, clip=true, coverage=4.5, fill=true, seed=1161})
work(wfree, {hand="scumble", pile=pwallF, clip=true, coverage=3.0, fill=true, seed=1162})
blend(wfree, {seed=1163, coverage=3.0, pressure={0.34,0.34}})
blend(wfree, {seed=1164, coverage=1.6, pressure={0.30,0.30}})

--@ chunk 91
-- objall was built with an older pear mask; rebuild it from the current shapes
objall = bottlemask + bowlmask + lemA + lemB + lemC + pear
print("objall area", objall:area(), "pear", pear:area())
-- test: an opaque white ground on the pear, short strokes, no blend
work(pear, {hand="body", pile=pile{{"lead white",4}}, clip=true, coverage=6.0, angle=0.2, fill=true, dips={2,1.0}, seed=1211})

--@ chunk 92
-- reshape the pear to a proper silhouette and paint it opaque, short strokes, no blend that thins it
pear = poly({{852,504},{862,510},{866,524},{867,542},{876,558},{884,578},{890,600},{890,622},{882,642},{866,656},{848,660},{830,656},{814,644},{806,626},{805,606},{810,586},{820,566},{830,546},{836,526},{838,510},{846,504}}, true) + ribbon({{852,512},{858,500},{862,490}}, {5,4,2})
objall = bottlemask + bowlmask + lemA + lemB + lemC + pear
tfree = tablemask - objall
pPearG = pile{{"lead white",3.0},{"chrome yellow",0.9},{"yellow ochre",1.1}}
work(pear, {hand="body", pile=pPearG, clip=true, coverage=5.5, angle=0.2, fill=true, dips={2,1.0}, seed=1221})
shp = grad(852,595,44,66, 0.75, 0.55, 0.30, 1.0) * pear
work(shp, {hand="scumble", pile=pile{{"lead white",1.3},{"yellow ochre",1.7},{"raw umber",0.6},{"red earth",0.25}}, clip=true, coverage=2.2, seed=1222})
litp = grad(852,590,44,66, -0.75, -0.55, 0.20, 1.0) * pear
work(litp, {hand="scumble", pile=pile{{"lead white",3.2},{"chrome yellow",0.9},{"yellow ochre",0.3}}, clip=true, coverage=2.2, seed=1223})
blush = grad(852,608,42,52, -0.7,-0.25, 0.68, 1.0) * pear
work(blush, {hand="body", pile=pile{{"red earth",1},{"yellow ochre",0.7},{"lead white",0.3}}, clip=true, coverage=0.9, seed=1224})
b = brush{kind="round", width=6, point=0.7}
b:load(pile{{"lead white",3.5},{"chrome yellow",0.5}}, 0.85)
b:touch(833,560,{pressure=0.45, drag={0.3,0.5}})
bs = brush{kind="round", width=2, point=0.6}
bs:load(pile{{"raw umber",1.4},{"yellow ochre",1},{"lead white",0.4}}, 0.8)
bs:stroke({{852,512},{858,501},{862,491}}, {pressure={0.35,0.3}})

--@ chunk 93
-- pear again: the lit/shadow split was a hard white/yellow seam; use wide smooth gradients and warmer lights
pPearG = pile{{"lead white",3.0},{"chrome yellow",0.9},{"yellow ochre",1.1}}
work(pear, {hand="body", pile=pPearG, clip=true, coverage=5.5, angle=0.2, fill=true, dips={2,1.0}, seed=1231})
shp = grad(852,600,50,70, 0.7, 0.5, 0.0, 1.0) * pear
work(shp, {hand="scumble", pile=pile{{"lead white",1.2},{"yellow ochre",1.8},{"raw umber",0.8},{"red earth",0.3}}, clip=true, coverage=2.0, seed=1232})
litp = grad(852,600,50,70, -0.6, -0.6, 0.0, 1.0) * pear
work(litp, {hand="scumble", pile=pile{{"lead white",2.2},{"chrome yellow",1.2},{"yellow ochre",0.5}}, clip=true, coverage=1.5, seed=1233})
blush = grad(852,606,42,54, -0.55,-0.4, 0.45, 1.0) * pear
work(blush, {hand="scumble", pile=pile{{"red earth",1},{"yellow ochre",0.8},{"lead white",0.4}}, clip=true, coverage=0.8, seed=1234})
b = brush{kind="round", width=6, point=0.7}
b:load(pile{{"lead white",3.0},{"chrome yellow",0.6}}, 0.85)
b:touch(834,562,{pressure=0.42, drag={0.3,0.5}})
bs = brush{kind="round", width=3, point=0.5}
bs:load(pile{{"raw umber",1.4},{"yellow ochre",1},{"lead white",0.4}}, 0.85)
bs:stroke({{851,516},{857,504},{862,492}}, {pressure={0.38,0.3}})

--@ chunk 94
-- pear, take three: a mid-yellow ground covering everything, then only soft glazes for form
pPearG2 = pile{{"lead white",1.7},{"chrome yellow",1.1},{"yellow ochre",1.4}}
work(pear, {hand="body", pile=pPearG2, clip=true, coverage=5.5, angle=0.2, fill=true, dips={2,1.0}, seed=1241})
shp = grad(852,600,50,70, 0.7, 0.5, 0.0, 1.0) * pear
work(shp, {hand="glaze", pile=pile{{"yellow ochre",1.5},{"raw umber",1.1},{"green earth",0.3},{"lead white",0.7}, medium=0.35}, clip=true, coverage=2.2, seed=1242})
litp = grad(852,600,50,70, -0.6, -0.6, 0.0, 1.0) * pear
work(litp, {hand="glaze", pile=pile{{"lead white",2.0},{"chrome yellow",1.0},{"yellow ochre",0.6}, medium=0.25}, clip=true, coverage=1.3, seed=1243})
blend(pear, {seed=1244, coverage=1.0, pressure={0.28,0.28}})
blush = grad(852,606,42,54, -0.55,-0.4, 0.5, 1.0) * pear
work(blush, {hand="glaze", pile=pile{{"red earth",1.0},{"yellow ochre",0.8},{"lead white",0.4}, medium=0.3}, clip=true, coverage=0.9, seed=1245})
b = brush{kind="round", width=6, point=0.7}
b:load(pile{{"lead white",3.0},{"chrome yellow",0.6}}, 0.85)
b:touch(834,562,{pressure=0.42, drag={0.3,0.5}})
bs = brush{kind="round", width=3.2, point=0.4}
bs:load(pile{{"raw umber",1.6},{"yellow ochre",0.8},{"bone black",0.2}}, 0.85)
bs:stroke({{853,520},{858,506},{863,492}}, {pressure={0.4,0.3}})

--@ chunk 95
-- cover the white halo left by the earlier test with a table ring, then repaint the pear clean on the bigger silhouette
pear = poly({{852,504},{862,510},{866,524},{867,542},{876,558},{884,578},{890,600},{890,622},{882,642},{866,656},{848,660},{830,656},{814,644},{806,626},{805,606},{810,586},{820,566},{830,546},{836,526},{838,510},{846,504}}, true):grow(9) + ribbon({{852,512},{858,500},{862,490}}, {5,4,2})
objall = bottlemask + bowlmask + lemA + lemB + lemC + pear
ring = (pear:grow(16) - pear):soften(9) * tablemask
work(ring, {hand="body", pile=pile{{"yellow ochre",2.0},{"raw umber",1.9},{"lead white",1.1},{"red earth",0.3}}, clip=true, coverage=2.6, angle=0.05, fill=true, seed=1252})
blend(ring:grow(6):soften(12), {seed=1253, coverage=1.6, pressure={0.30,0.30}})
pPearG2 = pile{{"lead white",1.7},{"chrome yellow",1.1},{"yellow ochre",1.4}}
work(pear, {hand="body", pile=pPearG2, clip=true, coverage=5.5, angle=0.2, fill=true, dips={2,1.0}, seed=1254})
shp = grad(852,600,50,70, 0.7, 0.5, 0.0, 1.0) * pear
work(shp, {hand="glaze", pile=pile{{"yellow ochre",1.5},{"raw umber",1.1},{"green earth",0.3},{"lead white",0.7}, medium=0.35}, clip=true, coverage=2.2, seed=1255})
litp = grad(852,600,50,70, -0.6, -0.6, 0.0, 1.0) * pear
work(litp, {hand="glaze", pile=pile{{"lead white",2.0},{"chrome yellow",1.0},{"yellow ochre",0.6}, medium=0.25}, clip=true, coverage=1.3, seed=1256})
blend(pear, {seed=1257, coverage=1.0, pressure={0.28,0.28}})
blush = grad(852,606,42,54, -0.55,-0.4, 0.5, 1.0) * pear
work(blush, {hand="glaze", pile=pile{{"red earth",1.0},{"yellow ochre",0.8},{"lead white",0.4}, medium=0.3}, clip=true, coverage=0.9, seed=1258})
b = brush{kind="round", width=6, point=0.7}
b:load(pile{{"lead white",3.0},{"chrome yellow",0.6}}, 0.85)
b:touch(834,562,{pressure=0.42, drag={0.3,0.5}})
bs = brush{kind="round", width=4, point=0.4}
bs:load(pile{{"raw umber",1.6},{"yellow ochre",0.8},{"bone black",0.2}}, 0.9)
bs:stroke({{853,520},{858,506},{863,492}}, {pressure={0.42,0.3}})
pconn2 = pile{{"raw umber",1.8},{"bone black",1.2},{"smalt",1.0},{"lead white",0.3}}
cs = ellipse(852,662,50,14):soften(9) * tablemask
work(cs, {hand="body", pile=pconn2, clip=true, coverage=2.6, seed=1259})
blend(cs:grow(8):soften(12), {seed=1260, coverage=1.4, pressure={0.3,0.3}})

--@ chunk 96
-- the ring left a bright halo: repaint the whole table around the correct object union
tfree = tablemask - objall
ptabB = pile{{"yellow ochre",2.2},{"raw umber",1.8},{"lead white",1.3},{"red earth",0.3}}
work(tfree, {hand="broad", pile=ptabB, clip=true, coverage=3.4, angle=0.05, fill=true, dips={3,0.88}, seed=1301})
pool = ellipse(300,590,340,150):soften(100) * tfree
work(pool, {hand="broad", pile=plight_t, clip=true, coverage=2.2, angle=0.05, seed=1302})
rdark = mask(function(x,y) return clamp((x-690)/310,0,1) end):soften(75) * tfree
work(rdark, {hand="broad", pile=pfront, clip=true, coverage=1.9, angle=0.05, seed=1303})
bottom = mask(function(x,y) return clamp((y-700)/100,0,1) end):soften(55) * tfree
work(bottom, {hand="broad", pile=pfront, clip=true, coverage=2.2, angle=0.03, seed=1304})
pS4 = pile{{"raw umber",1.8},{"bone black",1.1},{"smalt",1.1},{"lead white",0.4}, medium=0.12}
sh_bot = castshadow(300,616,640,662,56,16)
sh_bowl = castshadow(640,644,900,680,76,26)
sh_lem = castshadow(140,666,340,688,36,12)
sh_pear2 = castshadow(848,660,1000,678,42,16)
for i,m in ipairs{sh_bot,sh_bowl,sh_lem,sh_pear2} do
  work((m:soften(10))*tfree, {hand="body", pile=pS4, clip=true, coverage=2.8, seed=1310+i})
end
contact = ellipse(302,616,84,20):soften(9) + ellipse(640,644,112,22):soften(10)
        + ellipse(128,678,60,17):soften(8) + ellipse(852,662,58,17):soften(8)
work(contact*tfree, {hand="body", pile=pS4, clip=true, coverage=2.8, seed=1321})
sm = sh_bot+sh_bowl+sh_lem+sh_pear2
blend(sm:grow(7):soften(18)*tfree, {seed=1322, coverage=3.2, pressure={0.34,0.34}})
blend(sm:grow(14):soften(26)*tfree, {seed=1323, coverage=1.8, pressure={0.30,0.30}})
blend(contact:grow(6):soften(12)*tfree, {seed=1324, coverage=2.0, pressure={0.31,0.31}})

--@ chunk 97
-- pear form on the clean table: mid-yellow ground, smooth body gradients, no blend to thin it
pPearG2 = pile{{"lead white",1.7},{"chrome yellow",1.1},{"yellow ochre",1.4}}
work(pear, {hand="body", pile=pPearG2, clip=true, coverage=5.5, angle=0.2, fill=true, dips={2,1.0}, seed=1331})
shp = grad(852,600,52,72, 0.65, 0.5, 0.0, 1.0) * pear
work(shp, {hand="body", pile=pile{{"yellow ochre",1.6},{"raw umber",1.0},{"green earth",0.4},{"lead white",0.8}}, clip=true, coverage=2.6, angle=0.2, seed=1332})
litp = grad(852,600,52,72, -0.6, -0.6, 0.0, 1.0) * pear
work(litp, {hand="body", pile=pile{{"lead white",2.4},{"chrome yellow",1.0},{"yellow ochre",0.4}}, clip=true, coverage=2.0, angle=0.2, seed=1333})
blush = grad(852,606,40,52, -0.6,-0.35, 0.55, 1.0) * pear
work(blush, {hand="body", pile=pile{{"red earth",1.0},{"yellow ochre",0.8},{"lead white",0.3}}, clip=true, coverage=0.9, seed=1334})
b = brush{kind="round", width=6, point=0.7}
b:load(pile{{"lead white",3.2},{"chrome yellow",0.5}}, 0.85)
b:touch(834,562,{pressure=0.45, drag={0.3,0.5}})
bs = brush{kind="round", width=3.5, point=0.4}
bs:load(pile{{"raw umber",1.6},{"yellow ochre",0.8},{"bone black",0.2}}, 0.9)
bs:stroke({{853,520},{858,506},{863,492}}, {pressure={0.42,0.3}})

--@ chunk 98
-- bowl interior and the fruit: clean bright bodies, warm (not green) shadows
work(open_in, {hand="body", pile=pin3, clip=true, coverage=3.2, angle=0.2, fill=true, seed=1351})
far2 = grad(640,538,113,25, 0.0,-1.0, 0.05, 1.0) * open_in
work(far2, {hand="body", pile=pile{{"raw umber",2.2},{"smalt",1.2},{"lead white",1.0}}, clip=true, coverage=1.8, seed=1352})
near2 = grad(640,540,113,26, 0.0,1.0, 0.15, 1.0) * open_in
work(near2, {hand="body", pile=pile{{"lead white",2.2},{"raw umber",0.8},{"smalt",0.6}}, clip=true, coverage=1.4, seed=1353})
pLemonG = pile{{"chrome yellow",2.6},{"lead white",1.5},{"yellow ochre",0.4}}
pLemonS = pile{{"yellow ochre",1.8},{"raw umber",0.8},{"lead white",0.7}}
pLemonL = pile{{"chrome yellow",2.0},{"lead white",2.6}}
for i,m in ipairs{lemA, lemB} do
  local cx, cy, rx, ry = 600, 498, 60, 46
  if i == 2 then cx, cy, rx, ry = 700, 504, 54, 42 end
  work(m, {hand="body", pile=pLemonG, clip=true, coverage=4.5, angle=0.3, fill=true, dips={2,1.0}, seed=1360+i*3})
  local sh = grad(cx,cy,rx*1.05,ry*1.15, 0.55, 0.6, 0.0, 1.0) * m
  work(sh, {hand="body", pile=pLemonS, clip=true, coverage=2.2, angle=0.3, seed=1361+i*3})
  local li = grad(cx,cy,rx*1.05,ry*1.15, -0.6, -0.6, 0.0, 1.0) * m
  work(li, {hand="body", pile=pLemonL, clip=true, coverage=1.8, angle=0.3, seed=1362+i*3})
end
work(lemC, {hand="body", pile=pLemonG, clip=true, coverage=4.5, angle=0.25, fill=true, dips={2,1.0}, seed=1371})
shC = grad(126,644,58,42, 0.55, 0.6, 0.0, 1.0) * lemC
work(shC, {hand="body", pile=pLemonS, clip=true, coverage=2.2, angle=0.25, seed=1372})
liC = grad(126,644,58,42, -0.6,-0.6, 0.0, 1.0) * lemC
work(liC, {hand="body", pile=pLemonL, clip=true, coverage=1.8, angle=0.25, seed=1373})
crease = (ellipse(652,505,8,45):soften(6)) * (lemA+lemB)
work(crease, {hand="body", pile=pile{{"raw umber",1.4},{"yellow ochre",1.2},{"lead white",0.5}}, clip=true, coverage=2.2, seed=1380})
floor = ((ellipse(600,556,72,14) + ellipse(700,562,60,13)):soften(9)) * open_in
work(floor, {hand="body", pile=pile{{"raw umber",2.0},{"smalt",1.0},{"lead white",1.2}}, clip=true, coverage=2.0, seed=1381})
b = brush{kind="round", width=8, point=0.7}
b:load(pile{{"lead white",4}}, 0.9)
b:touch(578,474,{pressure=0.5, drag={0.5,0.5}})
b:wipe(0.35); b:load(pile{{"lead white",4}}, 0.85)
b:touch(680,480,{pressure=0.45, drag={0.45,0.45}})
b:wipe(0.35); b:load(pile{{"lead white",3.5}}, 0.85)
b:touch(108,624,{pressure=0.45, drag={0.5,0.5}})
b:wipe(0.4); b:load(pile{{"yellow ochre",1.6},{"raw umber",0.5}}, 0.5)
b:touch(72,640,{pressure=0.3})

--@ chunk 99
-- bowl exterior came out grey and smudged: repaint it clean white with a soft right shadow and warm bounce
owym = bowlmask - open
pbowl2 = pile{{"lead white",5},{"pale smalt",0.7},{"yellow ochre",0.4}}
work(owym, {hand="body", pile=pbowl2, clip=true, coverage=3.8, angle=0.25, fill=true, dips={2,0.95}, seed=1401})
gr2 = grad(640,590,128,90, 0.85, 0.25, 0.15, 1.0) * owym
work(gr2, {hand="scumble", pile=pbd, clip=true, coverage=2.0, seed=1402})
gl2 = grad(640,590,128,90, -0.9, 0.3, 0.25, 1.0) * owym
work(gl2, {hand="scumble", pile=pile{{"lead white",4},{"pale smalt",0.3}}, clip=true, coverage=1.6, seed=1403})
wr2 = grad(640,616,120,70, 0.15, 0.95, 0.4, 1.0) * owym
work(wr2, {hand="scumble", pile=pbr, clip=true, coverage=1.3, seed=1404})
rimshadow = ((open:grow(10) - open):soften(7)) * owym
work(rimshadow, {hand="body", pile=pbd, clip=true, coverage=1.6, angle=0.3, seed=1405})
rimhl = ribbon({{522,543},{532,550},{549,556},{572,561},{599,565}}, {9,9,8,7,6}) * bowlmask
work(rimhl, {hand="body", pile=pile{{"lead white",4},{"yellow ochre",0.35}}, clip=true, coverage=2.2, seed=1406})
rimhr = ribbon({{681,565},{708,561},{731,556},{748,550},{758,543}}, {6,7,8,9,9}) * bowlmask
work(rimhr, {hand="body", pile=pile{{"lead white",3},{"pale smalt",0.4}}, clip=true, coverage=1.6, seed=1407})

--@ chunk 100
-- bottle: even the murky glass with one opaque dark lay-in, then its reflections and speculars
pbot4 = pile{{"bone black",1.1},{"green earth",1.0},{"raw umber",0.35}}
work(bottlemask, {hand="body", pile=pbot4, clip=true, coverage=4.0, angle=1.5, fill=true, dips={2,0.95}, seed=1501})
blend(bottlemask, {seed=1502, coverage=0.8, pressure={0.25,0.25}})
lref3 = grad(300,420,110,300, -1.0, -0.05, 0.35, 1.0) * bottlemask
work(lref3, {hand="glaze", pile=pile{{"lead white",1.2},{"pale smalt",0.9},{"green earth",0.5}, medium=0.4}, clip=true, coverage=1.3, seed=1503})
rref3 = grad(302,560,110,150, 1.0, 0.4, 0.4, 1.0) * bottlemask
work(rref3, {hand="glaze", pile=pile{{"lead white",1.0},{"yellow ochre",1.0},{"raw umber",0.8}, medium=0.35}, clip=true, coverage=1.1, seed=1504})
blend(bottlemask, {seed=1505, coverage=0.7, pressure={0.25,0.25}})
b = brush{kind="round", width=5, point=0.7, stiffness=0.6}
b:load(pile{{"lead white",4.5}}, 0.95)
b:stroke({{278,402},{274,450},{272,502},{273,552},{276,586}}, {pressure={0.40,0.26}, ramps={0.12,0.25}})
b:wipe(0.4); b:load(pile{{"lead white",4.5}}, 0.9)
b:stroke({{298,232},{295,262},{293,296}}, {pressure={0.30,0.32}})
b:touch(299,224,{pressure=0.42, drag={0.4,-0.2}})
b2 = brush{kind="round", width=2.5, point=0.7}
b2:load(pile{{"lead white",1.2},{"pale smalt",0.9},{"green earth",0.5}}, 0.8)
b2:stroke({{347,470},{345,520},{344,566}}, {pressure={0.22,0.22}})
b3 = brush{kind="round", width=2.5, point=0.7}
b3:load(pile{{"lead white",2.5},{"pale smalt",0.5}}, 0.7)
b3:stroke({{255,470},{255,520},{257,560},{263,590}}, {pressure={0.20,0.20}})

--@ chunk 101
-- the glaze passes banded the bottle horizontally: paint it all in vertical strokes and leave the glazes out
pbot5 = pile{{"bone black",1.1},{"green earth",1.0},{"raw umber",0.35},{"Prussian blue",0.05}}
work(bottlemask, {hand="body", pile=pbot5, clip=true, coverage=4.5, angle=1.5, fill=true, dips={2,0.95}, seed=1521})
lref4 = grad(288,420,90,300, -1.0, 0.0, 0.45, 1.0) * bottlemask
work(lref4, {hand="body", pile=pile{{"lead white",1.4},{"pale smalt",0.9},{"green earth",0.5}}, clip=true, coverage=1.8, angle=1.5, seed=1522})
rref4 = grad(300,555,100,150, 1.0, 0.4, 0.5, 1.0) * bottlemask
work(rref4, {hand="body", pile=pile{{"lead white",1.0},{"yellow ochre",1.0},{"raw umber",0.9}}, clip=true, coverage=1.3, angle=1.5, seed=1523})
blend(bottlemask, {seed=1524, coverage=1.2, pressure={0.27,0.27}})
b = brush{kind="round", width=5, point=0.7, stiffness=0.6}
b:load(pile{{"lead white",4.5}}, 0.95)
b:stroke({{278,402},{274,450},{272,502},{273,552},{276,586}}, {pressure={0.40,0.26}, ramps={0.12,0.25}})
b:wipe(0.4); b:load(pile{{"lead white",4.5}}, 0.9)
b:stroke({{298,232},{295,262},{293,296}}, {pressure={0.30,0.32}})
b:touch(299,224,{pressure=0.42, drag={0.4,-0.2}})
b2 = brush{kind="round", width=2.5, point=0.7}
b2:load(pile{{"lead white",1.2},{"pale smalt",0.9},{"green earth",0.5}}, 0.8)
b2:stroke({{347,470},{345,520},{344,566}}, {pressure={0.22,0.22}})
b3 = brush{kind="round", width=2.5, point=0.7}
b3:load(pile{{"lead white",2.5},{"pale smalt",0.5}}, 0.7)
b3:stroke({{255,470},{255,520},{257,560},{263,590}}, {pressure={0.20,0.20}})

--@ chunk 102
-- bowl exterior again: white ground, then soft glaze shadows (no scumble blotches)
owym = bowlmask - open
pbowl3 = pile{{"lead white",5},{"pale smalt",0.6},{"yellow ochre",0.4}}
work(owym, {hand="body", pile=pbowl3, clip=true, coverage=4.2, angle=0.25, fill=true, dips={2,0.95}, seed=1511})
gr3 = grad(640,585,126,95, 0.8, 0.25, 0.0, 1.0) * owym
work(gr3, {hand="glaze", pile=pile{{"raw umber",1.2},{"smalt",1.0},{"lead white",1.6}, medium=0.4}, clip=true, coverage=2.0, seed=1512})
wr3 = grad(640,616,120,70, -0.25, 0.95, 0.25, 1.0) * owym
work(wr3, {hand="glaze", pile=pile{{"yellow ochre",1.0},{"lead white",1.8},{"red earth",0.2}, medium=0.35}, clip=true, coverage=1.6, seed=1513})
blend(owym, {seed=1514, coverage=1.6, pressure={0.30,0.30}})
blend(owym, {seed=1515, coverage=1.0, pressure={0.27,0.27}})
rimhl = ribbon({{522,543},{532,550},{549,556},{572,561},{599,565}}, {9,9,8,7,6}) * bowlmask
work(rimhl, {hand="body", pile=pile{{"lead white",4},{"yellow ochre",0.35}}, clip=true, coverage=2.4, seed=1516})
rimhr = ribbon({{681,565},{708,561},{731,556},{748,550},{758,543}}, {6,7,8,9,9}) * bowlmask
work(rimhr, {hand="body", pile=pile{{"lead white",3},{"pale smalt",0.4}}, clip=true, coverage=1.6, seed=1517})

--@ chunk 103
-- final accents: a crisp table edge behind the bottle, and small speculars on the fruit
bj = brush{kind="round", width=4, point=0.6, stiffness=0.5}
pj = pile{{"raw umber",2},{"smalt",1.2},{"bone black",0.45}, medium=0.4}
bj:load(pj, 0.55)
bj:stroke({{0,446},{60,447},{130,448},{200,449},{248,450}}, {pressure={0.35,0.3}, ramps={0.05,0.1}})
bj:wipe(0.5); bj:load(pj, 0.55)
bj:stroke({{352,451},{430,453},{520,456},{620,459},{720,462},{820,465},{910,467},{1000,469}}, {pressure={0.35,0.3}, ramps={0.05,0.1}})
bj:wipe(0.6); bj:load(pile{{"lead white",1.4},{"yellow ochre",1.2},{"raw umber",1.0}}, 0.4)
bj:stroke({{0,449},{120,451},{248,453}}, {pressure={0.22,0.2}, ramps={0.05,0.1}})
bj:wipe(0.4); bj:load(pile{{"lead white",1.4},{"yellow ochre",1.2},{"raw umber",1.0}}, 0.4)
bj:stroke({{352,454},{500,458},{660,462},{820,466},{1000,471}}, {pressure={0.22,0.2}, ramps={0.05,0.1}})
b = brush{kind="round", width=4, point=0.8}
b:load(pile{{"lead white",4}}, 0.9)
b:touch(570,468,{pressure=0.4, drag={0.3,0.4}})
b:wipe(0.4); b:load(pile{{"lead white",4}}, 0.85)
b:touch(672,474,{pressure=0.38, drag={0.3,0.4}})
b:wipe(0.4); b:load(pile{{"lead white",3.5}}, 0.85)
b:touch(100,616,{pressure=0.38, drag={0.35,0.45}})

--@ chunk 104
-- give the bowl interior a little depth: deeper at the back, a warm bounce on the near inner wall
back3 = grad(640,535,112,25, 0.0, -1.0, 0.15, 1.0) * open_in
work(back3, {hand="glaze", pile=pile{{"raw umber",1.8},{"smalt",1.0},{"lead white",1.0}, medium=0.4}, clip=true, coverage=1.3, seed=1601})
near3 = grad(640,543,112,26, 0.0, 1.0, 0.2, 1.0) * open_in
work(near3, {hand="glaze", pile=pile{{"yellow ochre",1.0},{"lead white",1.6},{"red earth",0.15}, medium=0.4}, clip=true, coverage=1.3, seed=1602})

--@ chunk 105
print(wait(5*1440)); print("bottle",drying(300,500),"bowl",drying(640,600),"bowlIn",drying(640,545),"table",drying(500,700),"wall",drying(500,200),"lem",drying(600,490),"pear",drying(852,600),"lemC",drying(126,644))

--@ chunk 106
-- the near foreground had gone to dark blobs: lay it in evenly and fuse
fg = mask(function(x,y) return clamp((y-705)/95,0,1) end):soften(45) * (tablemask - objall)
pFG = pile{{"raw umber",2.6},{"bone black",0.7},{"smalt",1.0},{"lead white",0.45}}
work(fg, {hand="body", pile=pFG, clip=true, coverage=3.5, angle=0.04, fill=true, dips={3,0.85}, seed=1711})
work(fg, {hand="body", pile=pFG, clip=true, coverage=2.0, angle=1.55, fill=true, dips={3,0.85}, seed=1712})
blend(fg, {seed=1713, coverage=3.0, pressure={0.33,0.33}})
blend(fg, {seed=1714, coverage=1.6, pressure={0.29,0.29}})

--@ chunk 107
-- the foreground band left a step: relaid the whole table clean again with its shadows
tfree = tablemask - objall
ptabB = pile{{"yellow ochre",2.2},{"raw umber",1.8},{"lead white",1.3},{"red earth",0.3}}
work(tfree, {hand="broad", pile=ptabB, clip=true, coverage=3.4, angle=0.05, fill=true, dips={3,0.88}, seed=1731})
pool = ellipse(300,590,340,150):soften(100) * tfree
work(pool, {hand="broad", pile=plight_t, clip=true, coverage=2.2, angle=0.05, seed=1732})
rdark = mask(function(x,y) return clamp((x-690)/310,0,1) end):soften(75) * tfree
work(rdark, {hand="broad", pile=pfront, clip=true, coverage=1.9, angle=0.05, seed=1733})
bottom = mask(function(x,y) return clamp((y-700)/100,0,1) end):soften(55) * tfree
work(bottom, {hand="broad", pile=pfront, clip=true, coverage=2.2, angle=0.03, seed=1734})
pS4 = pile{{"raw umber",1.8},{"bone black",1.1},{"smalt",1.1},{"lead white",0.4}, medium=0.12}
sh_bot = castshadow(300,616,640,662,56,16)
sh_bowl = castshadow(640,644,900,680,76,26)
sh_lem = castshadow(140,666,340,688,36,12)
sh_pear2 = castshadow(848,660,1000,678,42,16)
for i,m in ipairs{sh_bot,sh_bowl,sh_lem,sh_pear2} do
  work((m:soften(10))*tfree, {hand="body", pile=pS4, clip=true, coverage=2.8, seed=1740+i})
end
contact = ellipse(302,616,84,20):soften(9) + ellipse(640,644,112,22):soften(10)
        + ellipse(128,678,60,17):soften(8) + ellipse(852,662,58,17):soften(8)
work(contact*tfree, {hand="body", pile=pS4, clip=true, coverage=2.8, seed=1751})
sm = sh_bot+sh_bowl+sh_lem+sh_pear2
blend(sm:grow(7):soften(18)*tfree, {seed=1752, coverage=3.2, pressure={0.34,0.34}})
blend(sm:grow(14):soften(26)*tfree, {seed=1753, coverage=1.8, pressure={0.30,0.30}})
blend(contact:grow(6):soften(12)*tfree, {seed=1754, coverage=2.0, pressure={0.31,0.31}})

--@ chunk 108
print(wait(7*1440)); print("bottle",drying(300,500),"bowl",drying(640,600),"table",drying(500,700),"wall",drying(500,200),"pear",drying(852,600),"lemC",drying(126,644),"lem",drying(600,490))

--@ chunk 109
print("bottle",drying(300,500),"bowlout",drying(640,610),"bowlin",drying(640,545),"table",drying(500,700),"wall",drying(500,200),"lem",drying(600,490),"pear",drying(852,600),"lemC",drying(126,644))

--@ chunk 110
pMidF = pile{{"chrome yellow",2.2},{"yellow ochre",0.8},{"lead white",2.4}}
pLitF = pile{{"lead white",3.2},{"chrome yellow",1.0}}
pShdF = pile{{"yellow ochre",1.6},{"raw umber",0.8},{"green earth",0.35},{"lead white",0.9}}
alllem = lemA+lemB+lemC
work(alllem, {hand="body", pile=pMidF, clip=true, coverage=4.2, angle=0.5, fill=true, dips={2,0.95}, seed=2001})
function sphereFruit(m,cx,cy,rx,ry,seed)
  local lit = grad(cx,cy,rx,ry,-0.7,-0.7, 0.42, 1.0) * m
  local sha = grad(cx,cy,rx,ry, 0.7,0.7, 0.35, 0.95) * m
  work(lit,{hand="body",pile=pLitF,clip=true,coverage=2.6,angle=0.6,seed=seed})
  work(sha,{hand="body",pile=pShdF,clip=true,coverage=2.8,angle=0.6,seed=seed+1})
end
sphereFruit(lemA,600,498,60,46,2010)
sphereFruit(lemB,700,504,54,42,2013)
sphereFruit(lemC,124,645,58,34,2016)
blend(alllem,{seed=2020,coverage=1.2,pressure={0.28,0.28}})
print("fruit keyed")

--@ chunk 111
-- table: unify the blotchy surface, then a light pool and receding shadow
tfree = tablemask - objall
pT = pile{{"yellow ochre",2.4},{"raw umber",1.5},{"lead white",1.5},{"red earth",0.3}}
work(tfree,{hand="broad",pile=pT,clip=true,coverage=5.0,angle=0.05,fill=true,dips={3,0.9},seed=2101})
pool = grad(320,600,400,200,-0.45,0.75,0.30,1.0):soften(40) * tfree
work(pool,{hand="broad",pile=pile{{"yellow ochre",2.2},{"lead white",2.2},{"chrome yellow",0.3}},clip=true,coverage=1.8,angle=0.05,seed=2102})
dark = grad(820,620,320,240,0.9,0.6,0.30,1.0):soften(50) * tfree
work(dark,{hand="broad",pile=pile{{"raw umber",2.4},{"smalt",1.1},{"bone black",0.3},{"lead white",0.55}},clip=true,coverage=1.6,angle=0.05,seed=2103})
backsh = (mask(function(x,y) return clamp((y-(470+0.02*x))/28,0,1) end):soften(8)) * tfree
work(backsh,{hand="broad",pile=pile{{"raw umber",2.0},{"smalt",1.0},{"lead white",1.0}},clip=true,coverage=1.4,angle=0.03,seed=2105})
blend(tfree:soften(6),{seed=2104,coverage=1.4,pressure={0.30,0.30}})
-- re-lay the cast shadows, warm and directional to the lower right
pSh2 = pile{{"raw umber",2.0},{"bone black",0.9},{"smalt",0.9},{"lead white",0.35}, medium=0.1}
sh_bot  = castshadow(300,616,660,668,58,17)
sh_bowl = castshadow(640,646,910,684,78,27)
sh_lem  = castshadow(142,668,350,692,38,13)
sh_pear = castshadow(848,662,1000,682,44,17)
for i,m in ipairs{sh_bot,sh_bowl,sh_lem,sh_pear} do
  work((m:soften(9))*tfree, {hand="body", pile=pSh2, clip=true, coverage=2.6, seed=2110+i})
end
contact = ellipse(302,618,84,20):soften(9) + ellipse(640,646,112,22):soften(10)
        + ellipse(128,678,60,17):soften(8) + ellipse(852,662,58,17):soften(8)
work(contact*tfree, {hand="body", pile=pSh2, clip=true, coverage=2.8, seed=2121})
sm = sh_bot+sh_bowl+sh_lem+sh_pear
blend(sm:grow(7):soften(16)*tfree, {seed=2122, coverage=3.0, pressure={0.33,0.33}})
blend(contact:grow(6):soften(12)*tfree, {seed=2123, coverage=2.0, pressure={0.31,0.31}})
print("table relaid")

--@ chunk 112
function turnFruit(m,cx,cy,rx,ry,seed)
  local pBase = pile{{"lead white",1.7},{"chrome yellow",1.1},{"yellow ochre",1.3}}
  local pSh  = pile{{"yellow ochre",1.5},{"raw umber",1.1},{"green earth",0.5}}
  local pCore= pile{{"yellow ochre",1.2},{"raw umber",1.3},{"green earth",0.55}}
  local pLit = pile{{"lead white",2.8},{"chrome yellow",1.0},{"yellow ochre",0.3}}
  local pRef = pile{{"lead white",1.4},{"yellow ochre",1.7},{"red earth",0.35}}
  work(m,{hand="body",pile=pBase,clip=true,coverage=4.6,angle=0.2,fill=true,dips={2,1.0},seed=seed})
  work(grad(cx,cy,rx,ry,0.8,0.5,0.12,0.95)*m,{hand="body",pile=pSh,clip=true,coverage=3.0,angle=0.2,seed=seed+1})
  work(grad(cx,cy,rx,ry,0.9,0.5,0.5,0.95)*m,{hand="body",pile=pCore,clip=true,coverage=2.0,angle=0.2,seed=seed+2})
  work((m:rim(5,4))*grad(cx,cy+ry*0.6,rx,ry*0.5,0.3,0.85,0.2,1.0),{hand="body",pile=pRef,clip=true,coverage=1.5,seed=seed+3})
  work(grad(cx,cy,rx,ry,-0.7,-0.6,0.5,1.0)*m,{hand="body",pile=pLit,clip=true,coverage=2.0,seed=seed+4})
  blend(m,{seed=seed+5,coverage=1.0,pressure={0.27,0.27}})
end
turnFruit(pear,850,588,50,76,2201)
turnFruit(lemA,600,498,60,46,2211)
turnFruit(lemB,700,504,54,42,2221)
turnFruit(lemC,124,645,58,34,2231)
print("fruit turned")

--@ chunk 113
pB = pile{{"chrome yellow",1.6},{"yellow ochre",0.9},{"lead white",2.6}}
pS = pile{{"yellow ochre",1.4},{"raw umber",0.9},{"lead white",0.9},{"bone black",0.12}}
pL = pile{{"lead white",2.6},{"chrome yellow",1.0}}
work(lemC,{hand="body",pile=pB,clip=true,coverage=7,angle=0.3,fill=true,dips={2,1.0},seed=2301})
work(grad(124,645,58,34,0.8,0.5,0.15,0.9)*lemC,{hand="body",pile=pS,clip=true,coverage=5,angle=0.3,fill=true,dips={2,1.0},seed=2302})
work(grad(124,645,58,34,-0.7,-0.55,0.5,1.0)*lemC,{hand="body",pile=pL,clip=true,coverage=3.5,angle=0.3,fill=true,dips={2,1.0},seed=2303})
blend(lemC,{seed=2304,coverage=1.6,pressure={0.28,0.28}})
print("lemC test")

--@ chunk 114
function opaqueFruit(m,cx,cy,rx,ry,seed)
  work(m,{hand="body",pile=pB,clip=true,coverage=7,angle=0.3,fill=true,dips={2,1.0},seed=seed})
  work(grad(cx,cy,rx,ry,0.8,0.5,0.10,0.90)*m,{hand="body",pile=pS,clip=true,coverage=5,angle=0.3,fill=true,dips={2,1.0},seed=seed+1})
  work(grad(cx,cy,rx,ry,-0.7,-0.55,0.48,1.0)*m,{hand="body",pile=pL,clip=true,coverage=3.5,angle=0.3,fill=true,dips={2,1.0},seed=seed+2})
  blend(m,{seed=seed+3,coverage=1.6,pressure={0.28,0.28}})
end
opaqueFruit(lemA,600,498,60,46,2311)
opaqueFruit(lemB,700,504,54,42,2321)
opaqueFruit(pear,850,588,50,76,2331)
print("fruit opaque")

--@ chunk 115
-- bowl: give the white body form and the interior depth
owym = bowlmask - open
pBowlSh  = pile{{"lead white",3.2},{"raw umber",0.8},{"pale smalt",1.0}}
pBowlLit = pile{{"lead white",5.0},{"yellow ochre",0.25}}
pBowlRef = pile{{"lead white",3.4},{"yellow ochre",0.9},{"red earth",0.25}}
work(grad(640,592,132,74,0.78,0.55,0.12,0.95)*owym,{hand="body",pile=pBowlSh,clip=true,coverage=3.6,angle=0.6,fill=true,dips={2,0.95},seed=2401})
work(grad(640,580,132,74,-0.7,-0.6,0.45,1.0)*owym,{hand="body",pile=pBowlLit,clip=true,coverage=2.6,angle=0.6,seed=2402})
work(owym:rim(6,4)*grad(640,642,132,60,0.15,0.9,0.2,1.0),{hand="body",pile=pBowlRef,clip=true,coverage=1.8,seed=2403})
blend(owym,{seed=2404,coverage=1.5,pressure={0.28,0.28}})
-- interior: deeper at the back, warm bounce on the near wall, occlusion under the fruit
pBowlIn = pile{{"lead white",1.2},{"raw umber",1.6},{"smalt",1.0}}
pBowlInW= pile{{"lead white",2.2},{"yellow ochre",1.0},{"red earth",0.2}}
work(grad(640,536,118,28,0.0,-1.0,0.05,0.9)*open_in,{hand="body",pile=pBowlIn,clip=true,coverage=2.6,angle=0.1,seed=2410})
work(grad(640,544,118,28,0.0,1.0,0.15,1.0)*open_in,{hand="body",pile=pBowlInW,clip=true,coverage=2.4,angle=0.1,seed=2411})
occl = ((ellipse(600,552,66,18) + ellipse(700,558,58,17) + ellipse(650,556,20,20)):soften(12)) * open_in
work(occl,{hand="body",pile=pile{{"raw umber",1.8},{"smalt",1.0},{"lead white",0.8}},clip=true,coverage=2.8,seed=2412})
blend(open_in,{seed=2413,coverage=1.2,pressure={0.27,0.27}})
print("bowl modelled")

--@ chunk 116
interior = open_in - (lemA + lemB)
pIn = pile{{"lead white",1.3},{"raw umber",1.5},{"smalt",1.1}}
work(interior,{hand="body",pile=pIn,clip=true,coverage=4.0,angle=0.1,fill=true,dips={2,0.95},seed=2501})
work(grad(640,536,116,28,0.0,-1.0,0.0,0.85)*interior,{hand="body",pile=pile{{"raw umber",1.8},{"smalt",1.1},{"lead white",0.8}},clip=true,coverage=2.6,seed=2502})
work(grad(640,546,116,28,0.0,1.0,0.2,1.0)*interior,{hand="body",pile=pile{{"lead white",2.0},{"yellow ochre",1.0},{"red earth",0.25}},clip=true,coverage=2.2,seed=2503})
occ = ((ellipse(600,544,74,22) + ellipse(700,550,66,21)):soften(10)) - (lemA+lemB)
work(occ*interior,{hand="body",pile=pile{{"raw umber",2.0},{"smalt",1.0},{"lead white",0.7}},clip=true,coverage=3.0,seed=2504})
blend(interior,{seed=2505,coverage=1.2,pressure={0.27,0.27}})
opaqueFruit(lemA,600,498,60,46,2510)
opaqueFruit(lemB,700,504,54,42,2520)
pBowlSh2 = pile{{"lead white",2.4},{"raw umber",1.2},{"pale smalt",1.4},{"bone black",0.12}}
work(grad(640,592,132,74,0.78,0.55,0.05,0.9)*owym,{hand="body",pile=pBowlSh2,clip=true,coverage=3.4,angle=0.6,fill=true,dips={2,0.95},seed=2530})
work(owym:rim(6,4)*grad(640,642,132,60,0.15,0.9,0.2,1.0),{hand="body",pile=pBowlRef,clip=true,coverage=1.8,seed=2531})
blend(owym,{seed=2532,coverage=1.3,pressure={0.27,0.27}})
print("bowl v2")

--@ chunk 117
pBotD = pile{{"bone black",1.0},{"green earth",0.9},{"raw umber",0.4},{"Prussian blue",0.05}}
pBotL = pile{{"smalt",1.0},{"lead white",1.0},{"green earth",0.4}}
pBotR = pile{{"green earth",1.0},{"lead white",0.8},{"raw umber",0.3}}
work(bottlemask,{hand="body",pile=pBotD,clip=true,coverage=4.2,angle=1.5,fill=true,dips={3,0.9},seed=2601})
work(grad(300,470,58,300,-1.0,0.0,0.35,1.0)*bottlemask,{hand="body",pile=pBotL,clip=true,coverage=2.2,angle=1.5,seed=2602})
work(grad(300,560,58,180,1.0,0.15,0.45,1.0)*bottlemask,{hand="body",pile=pBotR,clip=true,coverage=2.0,angle=1.5,seed=2603})
work((bottlemask:rim(6,4))*grad(300,600,60,40,0.1,0.9,0.2,1.0),{hand="body",pile=pile{{"yellow ochre",1.2},{"raw umber",1.0},{"lead white",0.5}},clip=true,coverage=1.5,seed=2605})
blend(bottlemask,{seed=2604,coverage=1.0,pressure={0.26,0.26}})
b = brush{kind="round",width=4,point=0.7,stiffness=0.6}
b:load(pile{{"lead white",3}},0.8)
b:stroke({{287,520},{286,470},{283,420}},{pressure={0.25,0.2},ramps={0.1,0.2}})
b:stroke({{291,350},{289,380}},{pressure={0.22,0.2},ramps={0.1,0.2}})
b:wipe(0.4); b:load(pile{{"lead white",2.5},{"pale smalt",0.5}},0.7)
b:touch(300,232,{pressure=0.28})
b:touch(292,300,{pressure=0.22})
print("bottle v2")

--@ chunk 118
pBotD2 = pile{{"bone black",1.0},{"raw umber",0.5},{"Prussian blue",0.05}}
work(bottlemask,{hand="body",pile=pBotD2,clip=true,coverage=7.0,angle=1.5,fill=true,dips={3,1.0},seed=2701})
work(bottlemask,{hand="body",pile=pBotD2,clip=true,coverage=3.0,angle=0.1,fill=true,dips={3,1.0},seed=2702})
blend(bottlemask,{seed=2703,coverage=1.6,pressure={0.26,0.26}})
b = brush{kind="round",width=11,point=0.35,stiffness=0.4}
b:load(pile{{"lead white",1.0},{"smalt",0.9},{"bone black",0.12}}, 0.45)
b:stroke({{284,390},{279,450},{276,510},{276,560},{280,592}},{pressure={0.45,0.3},ramps={0.25,0.3}})
b:wipe(0.55); b:load(pile{{"lead white",0.8},{"smalt",0.7},{"bone black",0.15}}, 0.4)
b:stroke({{324,430},{327,490},{327,540},{322,580}},{pressure={0.35,0.22},ramps={0.25,0.3}})
b:wipe(0.5); b:load(pile{{"lead white",1.2},{"smalt",0.5}}, 0.5)
b:stroke({{291,330},{288,360},{287,395}},{pressure={0.4,0.3},ramps={0.2,0.25}})
b:touch(300,232,{pressure=0.3})
lref = grad(300,470,55,300,-1.0,0.0,0.35,1.0)*bottlemask
blend(lref,{seed=2704,coverage=1.3,pressure={0.25,0.25}})
print("bottle v3")

--@ chunk 119
pWallD = pile{{"raw umber",3.0},{"smalt",1.2},{"lead white",1.0},{"bone black",0.25}}
work(wallmask,{hand="broad",pile=pWallD,clip=true,coverage=5.0,angle=0.1,fill=true,dips={3,0.9},seed=2801})
topd = grad(500,110,720,260,0.0,-0.6,0.2,1.0)*wallmask
work(topd,{hand="broad",pile=pile{{"raw umber",3.0},{"smalt",1.2},{"bone black",0.5},{"lead white",0.5}},clip=true,coverage=1.8,angle=0.1,seed=2802})
glow = ellipse(310,380,270,210):soften(120)*wallmask
work(glow,{hand="broad",pile=pile{{"raw umber",2.0},{"lead white",2.0},{"yellow ochre",0.8}},clip=true,coverage=2.4,angle=0.06,seed=2803})
rd = mask(function(x,y) return clamp((x-560)/440,0,1) end):soften(80)*wallmask
work(rd,{hand="broad",pile=pile{{"raw umber",3.0},{"smalt",1.3},{"bone black",0.4},{"lead white",0.4}},clip=true,coverage=1.6,angle=0.1,seed=2804})
blend(wallmask:soften(8),{seed=2805,coverage=1.6,pressure={0.3,0.3}})
print("wall re-keyed")

--@ chunk 120
wfree = wallmask - objall
pW = pile{{"raw umber",2.6},{"smalt",1.3},{"bone black",0.35},{"lead white",0.9}}
work(wfree,{hand="broad",pile=pW,clip=true,coverage=6.0,angle=0.1,fill=true,dips={3,0.9},seed=2901})
topd = grad(500,80,760,340,0.0,-0.55,0.25,1.0)*wfree
work(topd,{hand="broad",pile=pile{{"raw umber",2.8},{"smalt",1.3},{"bone black",0.55},{"lead white",0.6}},clip=true,coverage=1.8,angle=0.1,seed=2902})
glow = ellipse(300,400,300,220):soften(140)*wfree
work(glow,{hand="broad",pile=pile{{"raw umber",2.0},{"lead white",1.8},{"yellow ochre",0.7}},clip=true,coverage=2.2,angle=0.06,seed=2903})
blend(wfree:soften(8),{seed=2904,coverage=1.8,pressure={0.3,0.3}})
-- restore the bottle fully
pBotD2 = pile{{"bone black",1.0},{"raw umber",0.5},{"Prussian blue",0.05}}
work(bottlemask,{hand="body",pile=pBotD2,clip=true,coverage=7.0,angle=1.5,fill=true,dips={3,1.0},seed=2910})
work(bottlemask,{hand="body",pile=pBotD2,clip=true,coverage=3.0,angle=0.1,fill=true,dips={3,1.0},seed=2911})
blend(bottlemask,{seed=2912,coverage=1.5,pressure={0.26,0.26}})
b = brush{kind="round",width=11,point=0.35,stiffness=0.4}
b:load(pile{{"lead white",1.0},{"smalt",0.9},{"bone black",0.12}}, 0.45)
b:stroke({{284,390},{279,450},{276,510},{276,560},{280,592}},{pressure={0.45,0.3},ramps={0.25,0.3}})
b:wipe(0.55); b:load(pile{{"lead white",0.8},{"smalt",0.7},{"bone black",0.15}}, 0.4)
b:stroke({{324,430},{327,490},{327,540},{322,580}},{pressure={0.35,0.22},ramps={0.25,0.3}})
b:wipe(0.5); b:load(pile{{"lead white",1.2},{"smalt",0.5}}, 0.5)
b:stroke({{291,330},{288,360},{287,395}},{pressure={0.4,0.3},ramps={0.2,0.25}})
b:touch(300,232,{pressure=0.3})
print("wall fixed, bottle restored")

--@ chunk 121
pShF = pile{{"yellow ochre",1.3},{"raw umber",1.2},{"lead white",0.5},{"bone black",0.08}}
pCoreF = pile{{"yellow ochre",1.0},{"raw umber",1.4},{"green earth",0.4},{"lead white",0.4}}
function shadeFruit(m,cx,cy,rx,ry,seed)
  work(grad(cx,cy,rx,ry,0.8,0.5,0.05,0.9)*m,{hand="body",pile=pShF,clip=true,coverage=4.5,angle=0.25,fill=true,dips={2,1.0},seed=seed})
  work(grad(cx,cy,rx,ry,0.9,0.5,0.45,0.92)*m,{hand="body",pile=pCoreF,clip=true,coverage=3.0,angle=0.25,seed=seed+1})
  blend(m,{seed=seed+2,coverage=1.3,pressure={0.27,0.27}})
end
shadeFruit(lemA,600,498,60,46,3001)
shadeFruit(lemB,700,504,54,42,3011)
shadeFruit(lemC,124,645,58,34,3021)
shadeFruit(pear,850,588,50,76,3031)
b = brush{kind="round",width=5,point=0.8}
b:load(pile{{"lead white",4}},0.9)
b:touch(574,470,{pressure=0.4,drag={0.3,0.4}})
b:wipe(0.4); b:load(pile{{"lead white",4}},0.9)
b:touch(674,476,{pressure=0.38,drag={0.3,0.4}})
b:wipe(0.4); b:load(pile{{"lead white",4}},0.9)
b:touch(100,622,{pressure=0.4,drag={0.4,0.4}})
b:wipe(0.4); b:load(pile{{"lead white",4}},0.9)
b:touch(830,550,{pressure=0.4,drag={0.3,0.5}})
print("fruit shaded + speculars")

--@ chunk 122
wfree = wallmask - objall
pWd = pile{{"raw umber",2.8},{"smalt",1.5},{"bone black",0.5},{"lead white",0.6}}
work(wfree,{hand="broad",pile=pWd,clip=true,coverage=4.0,angle=0.08,fill=true,dips={3,0.9},seed=3101})
blend(wfree:soften(6),{seed=3102,coverage=2.4,pressure={0.32,0.32}})
glow = ellipse(300,400,320,230):soften(150)*wfree
work(glow,{hand="glaze",pile=pile{{"raw umber",1.6},{"lead white",2.0},{"yellow ochre",0.8},medium=0.3},clip=true,coverage=1.6,seed=3103})
print("wall deeper")

--@ chunk 123
wfree = wallmask - objall
pWeven = pile{{"raw umber",2.2},{"smalt",1.2},{"bone black",0.6},{"lead white",0.7}}
work(wfree,{hand="body",pile=pWeven,clip=true,coverage=5.0,angle=0.1,fill=true,dips={3,0.9},seed=3201})
blend(wfree:soften(5),{seed=3202,coverage=1.1,pressure={0.28,0.28}})
glow = ellipse(300,400,330,240):soften(150)*wfree
work(glow,{hand="glaze",pile=pile{{"raw umber",1.5},{"lead white",2.0},{"yellow ochre",0.8},medium=0.3},clip=true,coverage=1.4,seed=3203})
print("wall evened")

--@ chunk 124
tfree = tablemask - objall
pSh3 = pile{{"raw umber",2.2},{"bone black",1.0},{"smalt",0.8},{"lead white",0.3}, medium=0.08}
sh_bot  = castshadow(300,616,660,668,58,17)
sh_bowl = castshadow(640,646,910,684,78,27)
sh_lem  = castshadow(142,668,350,692,38,13)
sh_pear = castshadow(848,662,1000,682,44,17)
for i,m in ipairs{sh_bot,sh_bowl,sh_lem,sh_pear} do
  work((m:soften(8))*tfree, {hand="body", pile=pSh3, clip=true, coverage=3.2, seed=3301+i})
end
contact = ellipse(302,618,84,20):soften(9) + ellipse(640,646,112,22):soften(10)
        + ellipse(128,678,60,17):soften(8) + ellipse(852,662,58,17):soften(8)
work(contact*tfree, {hand="body", pile=pSh3, clip=true, coverage=3.4, seed=3311})
sm = sh_bot+sh_bowl+sh_lem+sh_pear
blend(sm:grow(6):soften(14)*tfree, {seed=3312, coverage=2.4, pressure={0.32,0.32}})
blend(contact:grow(5):soften(11)*tfree, {seed=3313, coverage=1.8, pressure={0.30,0.30}})
-- even out the foreground band
fg = mask(function(x,y) return clamp((y-700)/90,0,1) end):soften(45) * tfree
work(fg, {hand="body", pile=pile{{"raw umber",2.4},{"yellow ochre",1.0},{"bone black",0.5},{"lead white",0.6}}, clip=true, coverage=4.0, angle=0.04, fill=true, dips={3,0.9}, seed=3321})
blend(fg, {seed=3322, coverage=1.6, pressure={0.3,0.3}})
print("shadows + fg")

--@ chunk 125
tfree = tablemask - objall
pT = pile{{"yellow ochre",2.4},{"raw umber",1.5},{"lead white",1.5},{"red earth",0.3}}
work(tfree,{hand="broad",pile=pT,clip=true,coverage=5.0,angle=0.05,fill=true,dips={3,0.9},seed=3401})
pool = grad(320,600,400,200,-0.45,0.75,0.30,1.0):soften(50)*tfree
work(pool,{hand="broad",pile=pile{{"yellow ochre",2.2},{"lead white",2.2},{"chrome yellow",0.3}},clip=true,coverage=2.0,angle=0.05,seed=3402})
dark = mask(function(x,y)
  return clamp(0.15 + 0.45*clamp((y-560)/180,0,1) + 0.5*clamp((x-600)/400,0,1), 0, 1)
end):soften(70)*tfree
work(dark,{hand="broad",pile=pile{{"raw umber",2.4},{"smalt",1.1},{"bone black",0.3},{"lead white",0.55}},clip=true,coverage=1.8,angle=0.05,seed=3403})
blend(tfree:soften(6),{seed=3404,coverage=1.5,pressure={0.30,0.30}})
pSh4 = pile{{"raw umber",2.0},{"bone black",0.6},{"smalt",0.9},{"lead white",0.5}, medium=0.15}
sh_bot  = castshadow(300,616,560,650,50,14)
sh_bowl = castshadow(640,646,830,674,66,22)
sh_lem  = castshadow(142,668,300,686,34,11)
sh_pear = castshadow(848,662,966,678,40,14)
for i,m in ipairs{sh_bot,sh_bowl,sh_lem,sh_pear} do
  work((m:grow(3):soften(16))*tfree, {hand="body", pile=pSh4, clip=true, coverage=2.8, seed=3410+i})
end
contact = ellipse(302,618,84,20):soften(10) + ellipse(640,646,112,22):soften(11)
        + ellipse(128,678,60,17):soften(9) + ellipse(852,662,58,17):soften(9)
work(contact*tfree, {hand="body", pile=pSh4, clip=true, coverage=3.0, seed=3421})
sm = sh_bot+sh_bowl+sh_lem+sh_pear
blend(sm:grow(10):soften(26)*tfree, {seed=3422, coverage=3.0, pressure={0.32,0.32}})
blend(sm:grow(18):soften(40)*tfree, {seed=3423, coverage=1.6, pressure={0.29,0.29}})
blend(contact:grow(6):soften(14)*tfree, {seed=3424, coverage=1.8, pressure={0.30,0.30}})
print("table clean + soft shadows")

--@ chunk 126
pPearBase2 = pile{{"lead white",1.4},{"chrome yellow",1.4},{"yellow ochre",1.8}}
work(pear,{hand="body",pile=pPearBase2,clip=true,coverage=6.0,angle=0.1,fill=true,dips={2,1.0},seed=3501})
pPearSh2 = pile{{"yellow ochre",1.8},{"raw umber",1.2},{"green earth",0.4},{"lead white",0.6}}
work(grad(850,588,52,78,0.85,0.5,0.15,0.95)*pear,{hand="body",pile=pPearSh2,clip=true,coverage=5.0,angle=0.1,fill=true,dips={2,1.0},seed=3502})
pPearCore2 = pile{{"yellow ochre",1.2},{"raw umber",1.6},{"green earth",0.4}}
work(grad(850,588,52,78,0.95,0.5,0.45,0.95)*pear,{hand="body",pile=pPearCore2,clip=true,coverage=3.5,angle=0.1,seed=3503})
work(grad(850,588,52,78,-0.7,-0.6,0.5,1.0)*pear,{hand="body",pile=pile{{"lead white",3.0},{"chrome yellow",1.0}},clip=true,coverage=3.0,angle=0.1,seed=3504})
print("pear test")

--@ chunk 127
test = grad(850,588,52,78,0.85,0.5,0.15,0.95)*pear
print("test area", test:area(), "pear area", pear:area())
work(test,{hand="body",pile=pile{{"bone black",1}},clip=true,coverage=4.0,seed=3601})
print("painted test")

--@ chunk 128
pFB = pile{{"lead white",1.6},{"chrome yellow",1.5},{"yellow ochre",1.5}}
pFS = pile{{"yellow ochre",1.8},{"raw umber",1.0},{"lead white",0.7},{"bone black",0.05}}
pFC = pile{{"yellow ochre",1.2},{"raw umber",1.3},{"green earth",0.4},{"bone black",0.05}}
pFL = pile{{"lead white",3.0},{"chrome yellow",1.0}}
pFR = pile{{"lead white",1.3},{"yellow ochre",1.6},{"red earth",0.4}}
function modelFruit(m,cx,cy,rx,ry,seed)
  work(m,{hand="body",pile=pFB,clip=true,coverage=6.0,angle=0.15,fill=true,dips={2,1.0},seed=seed})
  work(grad(cx,cy,rx,ry,0.85,0.5,0.10,0.92)*m,{hand="body",pile=pFS,clip=true,coverage=5.0,angle=0.15,fill=true,dips={2,1.0},seed=seed+1})
  work(grad(cx,cy,rx,ry,0.95,0.5,0.45,0.95)*m,{hand="body",pile=pFC,clip=true,coverage=3.5,angle=0.15,seed=seed+2})
  work((m:rim(5,4))*grad(cx,cy+ry*0.55,rx,ry*0.55,0.25,0.9,0.25,1.0),{hand="body",pile=pFR,clip=true,coverage=1.6,seed=seed+3})
  work(grad(cx,cy,rx,ry,-0.7,-0.6,0.5,1.0)*m,{hand="body",pile=pFL,clip=true,coverage=3.0,angle=0.15,seed=seed+4})
end
modelFruit(pear,850,588,52,78,3701)
modelFruit(lemA,600,498,60,46,3711)
modelFruit(lemB,700,504,54,42,3721)
modelFruit(lemC,124,645,58,34,3731)
pW = pile{{"lead white",4}}
b = brush{kind="round",width=5,point=0.8}
b:load(pW,0.9); b:touch(824,548,{pressure=0.42,drag={0.3,0.5}})
b:wipe(0.4); b:load(pW,0.9); b:touch(574,470,{pressure=0.42,drag={0.3,0.4}})
b:wipe(0.4); b:load(pW,0.9); b:touch(674,476,{pressure=0.42,drag={0.3,0.4}})
b:wipe(0.4); b:load(pW,0.9); b:touch(100,622,{pressure=0.42,drag={0.4,0.4}})
bs = brush{kind="round",width=3.5,point=0.5}
bs:load(pile{{"raw umber",1.6},{"yellow ochre",0.8},{"bone black",0.3}},0.9)
bs:stroke({{851,516},{856,502},{861,490}},{pressure={0.4,0.3}})
print("fruit modelled v2")

--@ chunk 129
work(pear,{hand="body",pile=pFB,clip=true,coverage=6.0,angle=0.1,fill=true,dips={2,1.0},load=1.0,seed=3801})
pFSd = pile{{"yellow ochre",1.0},{"raw umber",1.2},{"bone black",0.30},{"lead white",0.4}}
work(grad(850,588,52,78,0.85,0.5,0.10,0.92)*pear,{hand="body",pile=pFSd,clip=true,coverage=6.0,angle=0.1,fill=true,dips={2,1.0},load=1.0,seed=3802})
pFCd = pile{{"yellow ochre",0.8},{"raw umber",1.4},{"bone black",0.35}}
work(grad(850,588,52,78,0.95,0.5,0.45,0.95)*pear,{hand="body",pile=pFCd,clip=true,coverage=4.0,angle=0.1,load=1.0,seed=3803})
work(grad(850,588,52,78,-0.7,-0.6,0.5,1.0)*pear,{hand="body",pile=pFL,clip=true,coverage=3.0,angle=0.1,load=1.0,seed=3804})
print("pear strong shadow")

--@ chunk 130
work(grad(850,588,52,78,0.85,0.5,0.10,0.92)*pear,{hand="body",pile=pile{{"vermilion",1}},clip=true,coverage=4.0,load=1.0,seed=3902})
print("red test2")

--@ chunk 131
pFB = pile{{"lead white",1.6},{"chrome yellow",1.5},{"yellow ochre",1.5}}
work(pear,{hand="body",pile=pFB,clip=true,coverage=9,angle=0.1,fill=true,dips={3,1.0},load=1.0,seed=4001})
work(pear,{hand="body",pile=pFB,clip=true,coverage=5,angle=0.1,fill=true,dips={3,1.0},load=1.0,seed=4002})
pFSd = pile{{"yellow ochre",1.0},{"raw umber",1.2},{"bone black",0.30},{"lead white",0.4}}
work(grad(850,588,52,78,0.85,0.5,0.25,0.95)*pear,{hand="body",pile=pFSd,clip=true,coverage=9,angle=0.1,fill=true,dips={3,1.0},load=1.0,seed=4003})
work(grad(850,588,52,78,0.85,0.5,0.25,0.95)*pear,{hand="body",pile=pFSd,clip=true,coverage=5,angle=0.1,fill=true,dips={3,1.0},load=1.0,seed=4004})
pFCd = pile{{"yellow ochre",0.8},{"raw umber",1.4},{"bone black",0.35}}
work(grad(850,588,52,78,0.95,0.5,0.5,0.95)*pear,{hand="body",pile=pFCd,clip=true,coverage=6,angle=0.1,fill=true,dips={3,1.0},load=1.0,seed=4005})
work(grad(850,588,52,78,-0.7,-0.6,0.55,1.0)*pear,{hand="body",pile=pFL,clip=true,coverage=5,angle=0.1,fill=true,dips={3,1.0},load=1.0,seed=4006})
print("pear built up")

--@ chunk 132
pW = pile{{"lead white",4}}
function buildFruit(m,cx,cy,rx,ry,seed)
  work(m,{hand="body",pile=pFB,clip=true,coverage=8,angle=0.15,fill=true,dips={3,1.0},load=1.0,seed=seed})
  work(m,{hand="body",pile=pFB,clip=true,coverage=4,angle=0.15,fill=true,dips={3,1.0},load=1.0,seed=seed+1})
  local sh = grad(cx,cy,rx,ry,0.85,0.5,0.25,0.95)*m
  work(sh,{hand="body",pile=pFSd,clip=true,coverage=8,angle=0.15,fill=true,dips={3,1.0},load=1.0,seed=seed+2})
  work(sh,{hand="body",pile=pFSd,clip=true,coverage=4,angle=0.15,fill=true,dips={3,1.0},load=1.0,seed=seed+3})
  work(grad(cx,cy,rx,ry,0.95,0.5,0.5,0.95)*m,{hand="body",pile=pFCd,clip=true,coverage=5,angle=0.15,fill=true,dips={3,1.0},load=1.0,seed=seed+4})
  work(grad(cx,cy,rx,ry,-0.7,-0.6,0.55,1.0)*m,{hand="body",pile=pFL,clip=true,coverage=4,angle=0.15,fill=true,dips={3,1.0},load=1.0,seed=seed+5})
end
buildFruit(lemA,600,498,60,46,4101)
buildFruit(lemB,700,504,54,42,4111)
buildFruit(lemC,124,645,58,34,4121)
b = brush{kind="round",width=5,point=0.8}
b:load(pW,0.9); b:touch(574,470,{pressure=0.45,drag={0.3,0.4}})
b:wipe(0.4); b:load(pW,0.9); b:touch(674,476,{pressure=0.45,drag={0.3,0.4}})
b:wipe(0.4); b:load(pW,0.9); b:touch(100,622,{pressure=0.45,drag={0.4,0.4}})
b:wipe(0.4); b:load(pW,0.9); b:touch(824,548,{pressure=0.45,drag={0.3,0.5}})
print("lemons built")

--@ chunk 133
tfree = tablemask - objall
wfree = wallmask - objall
grad2 = mask(function(x,y)
  local v = 0.20 + 0.55*clamp((y-540)/220,0,1) + 0.5*clamp((x-640)/380,0,1)
  return clamp(v,0,1)
end):soften(70)*tfree
work(grad2,{hand="broad",pile=pile{{"raw umber",2.6},{"smalt",1.1},{"bone black",0.35},{"lead white",0.5}},clip=true,coverage=2.6,angle=0.05,seed=4301})
blend(grad2,{seed=4302,coverage=1.6,pressure={0.30,0.30}})
pBowlSh3 = pile{{"lead white",2.0},{"raw umber",1.3},{"pale smalt",1.5},{"bone black",0.15}}
work(grad(640,592,132,74,0.78,0.55,0.02,0.9)*owym,{hand="body",pile=pBowlSh3,clip=true,coverage=4.5,angle=0.6,fill=true,dips={2,1.0},load=1.0,seed=4310})
work(grad(640,592,132,74,0.78,0.55,0.02,0.9)*owym,{hand="body",pile=pBowlSh3,clip=true,coverage=3.0,angle=0.6,fill=true,dips={2,1.0},load=1.0,seed=4311})
bsh = ellipse(362,370,62,175):soften(70)*wfree
work(bsh,{hand="glaze",pile=pile{{"raw umber",2.0},{"smalt",1.0},{"bone black",0.3},medium=0.2},clip=true,coverage=1.7,seed=4320})
blend(bsh,{seed=4321,coverage=1.0,pressure={0.27,0.27}})
print("final pass one")

--@ chunk 134
tfree = tablemask - objall
blend(grad2:grow(8):soften(24),{seed=4401,coverage=2.4,pressure={0.29,0.29}})
fg = mask(function(x,y) return clamp((y-700)/90,0,1) end):soften(50)*tfree
blend(fg,{seed=4402,coverage=1.8,pressure={0.29,0.29}})
blend(tfree,{seed=4403,coverage=1.0,pressure={0.26,0.26}})
print("table smoothed")

--@ chunk 135
pWh = pile{{"lead white",4}}
b = brush{kind="round",width=3.5,point=0.8}
b:load(pWh,0.9); b:touch(560,562,{pressure=0.4,drag={0.3,0.2}})
b:wipe(0.4); b:load(pWh,0.85); b:touch(726,556,{pressure=0.33,drag={0.3,0.2}})
b:wipe(0.4); b:load(pWh,0.85); b:touch(300,232,{pressure=0.4})
b:wipe(0.4); b:load(pWh,0.85); b:touch(286,394,{pressure=0.35,drag={0.2,0.5}})
-- warm bounce on the shadow sides
work((owym:rim(6,5))*grad(640,632,132,70,0.2,0.9,0.25,1.0),{hand="body",pile=pFR,clip=true,coverage=1.5,seed=4501})
work((lemA:rim(4,4))*grad(640,540,90,50,0.3,0.9,0.25,1.0),{hand="body",pile=pFR,clip=true,coverage=1.3,seed=4502})
work((pear:rim(4,4))*grad(855,650,60,40,0.3,0.9,0.25,1.0),{hand="body",pile=pFR,clip=true,coverage=1.3,seed=4503})
print("accents")

--@ chunk 136
print(wait(0))

--@ chunk 137
print(wait(0)); print("bottle",drying(300,500),"bowlout",drying(640,610),"bowlin",drying(640,545)); print("tableL",drying(300,650),"tableR",drying(850,650),"wall",drying(500,200),"lemA",drying(600,498),"pear",drying(850,588),"lemC",drying(124,645))

--@ chunk 138
print(type(bottlemask),type(bowlmask),type(open),type(open_in),type(lemA),type(lemB),type(lemC),type(pear),type(objall),type(tablemask),type(wallmask),type(owym),type(rimband)); print("piles",type(pFL),type(pFR),type(pFB),type(pFSd),type(pFCd),type(pSh4),type(plight_t),type(pfront)); print(wait(3*1440)); print("lemA",drying(600,498),"lemB",drying(700,504),"lemC",drying(124,645),"pear",drying(850,588)); print("bowlout",drying(640,610),"bowlin",drying(640,545),"bottle",drying(300,500),"table",drying(500,700),"wall",drying(500,200))

--@ chunk 139
print(wait(7*1440)); print("lemA",drying(600,498),"lemB",drying(700,504),"lemC",drying(124,645),"pear",drying(850,588)); print("bowlout",drying(640,610),"bottle",drying(300,500),"table",drying(500,700),"wall",drying(500,200))

--@ chunk 140

pMidP = pile{{"yellow ochre",1.3},{"lead white",1.6},{"chrome yellow",0.8},{"green earth",0.2}}
pShP  = pile{{"yellow ochre",1.5},{"raw umber",0.9},{"green earth",0.5},{"lead white",0.5}}
pCoreP= pile{{"raw umber",1.2},{"yellow ochre",1.0},{"green earth",0.55}}
pLitP = pile{{"lead white",2.8},{"chrome yellow",0.6},{"yellow ochre",0.3}}
pReflP= pile{{"lead white",1.2},{"yellow ochre",1.5},{"red earth",0.3}}
local m = pear
local cx,cy,rx,ry = 850,588,52,78
work(m,{hand="body",pile=pMidP,clip=true,coverage=6,angle=0.2,fill=true,dips={2,1.0},load=1.0,seed=5001})
work(grad(cx,cy,rx,ry,0.85,0.55,0.20,0.85)*m,{hand="body",pile=pShP,clip=true,coverage=6,angle=0.2,fill=true,dips={2,1.0},load=1.0,seed=5010})
work(grad(cx,cy,rx,ry,0.95,0.55,0.55,0.90)*m,{hand="body",pile=pCoreP,clip=true,coverage=4,angle=0.2,seed=5020})
work(grad(cx,cy,rx,ry,-0.75,-0.6,0.50,1.0)*m,{hand="body",pile=pLitP,clip=true,coverage=5,angle=0.2,fill=true,dips={2,1.0},load=1.0,seed=5030})
work((m:rim(4,4))*grad(cx,cy+ry*0.55,rx,ry*0.6,0.3,0.9,0.3,1.0),{hand="body",pile=pReflP,clip=true,coverage=2,seed=5040})
print("pear reworked")

--@ chunk 141

pMidP = pile{{"yellow ochre",1.7},{"lead white",0.9},{"chrome yellow",0.8},{"green earth",0.3}}
pShP  = pile{{"yellow ochre",1.2},{"raw umber",1.2},{"green earth",0.6},{"lead white",0.35}}
pCoreP= pile{{"raw umber",1.5},{"yellow ochre",0.8},{"green earth",0.6},{"bone black",0.12}}
pLitP = pile{{"lead white",2.2},{"chrome yellow",1.1},{"yellow ochre",0.4}}
pReflP= pile{{"lead white",1.1},{"yellow ochre",1.5},{"red earth",0.4}}
local m = pear
local cx,cy,rx,ry = 850,588,52,78
work(m,{hand="body",pile=pMidP,clip=true,coverage=6,angle=0.2,fill=true,dips={2,1.0},load=1.0,seed=5101})
work(grad(cx,cy,rx,ry,0.85,0.50,0.28,0.85)*m,{hand="body",pile=pShP,clip=true,coverage=8,angle=0.2,fill=true,dips={2,1.0},load=1.0,seed=5110})
work(grad(cx,cy,rx,ry,0.95,0.45,0.60,0.95)*m,{hand="body",pile=pCoreP,clip=true,coverage=6,angle=0.2,fill=true,dips={2,1.0},load=1.0,seed=5120})
work(grad(cx,cy,rx,ry,-0.75,-0.6,0.50,1.0)*m,{hand="body",pile=pLitP,clip=true,coverage=6,angle=0.2,fill=true,dips={2,1.0},load=1.0,seed=5130})
work((m:rim(4,4))*grad(cx,cy+ry*0.5,rx,ry*0.6,0.3,0.9,0.3,1.0),{hand="body",pile=pReflP,clip=true,coverage=2,seed=5140})
print("pear deepened")

--@ chunk 142

pMidL = pile{{"chrome yellow",1.3},{"yellow ochre",1.0},{"lead white",1.4}}
pShL  = pile{{"yellow ochre",1.3},{"raw umber",1.0},{"green earth",0.55},{"lead white",0.3}}
pCoreL= pile{{"raw umber",1.3},{"yellow ochre",0.9},{"green earth",0.6},{"bone black",0.15}}
pLitL = pile{{"lead white",2.4},{"chrome yellow",1.0},{"yellow ochre",0.2}}
pReflL= pile{{"lead white",1.1},{"yellow ochre",1.5},{"red earth",0.4}}
frontrim = rimband * mask(function(x,y) return (y > 539) and 1 or 0 end)
function modelLemon(m,cx,cy,rx,ry,seed)
  local mm = m - frontrim
  work(mm,{hand="body",pile=pMidL,clip=true,coverage=5,angle=0.35,fill=true,dips={2,1.0},load=1.0,seed=seed})
  work(grad(cx,cy,rx,ry,0.85,0.55,0.25,0.85)*mm,{hand="body",pile=pShL,clip=true,coverage=8,angle=0.35,fill=true,dips={2,1.0},load=1.0,seed=seed+1})
  work(grad(cx,cy,rx,ry,0.95,0.5,0.60,0.95)*mm,{hand="body",pile=pCoreL,clip=true,coverage=6,angle=0.35,fill=true,dips={2,1.0},load=1.0,seed=seed+2})
  work(grad(cx,cy,rx,ry,-0.75,-0.6,0.50,1.0)*mm,{hand="body",pile=pLitL,clip=true,coverage=5,angle=0.35,fill=true,dips={2,1.0},load=1.0,seed=seed+3})
  work((mm:rim(4,4))*grad(cx,cy+ry*0.5,rx,ry*0.6,0.3,0.9,0.3,1.0),{hand="body",pile=pReflL,clip=true,coverage=2,seed=seed+4})
end
modelLemon(lemA,600,498,60,46,5200)
modelLemon(lemB,700,504,54,42,5210)
modelLemon(lemC,124,645,58,34,5220)
print("lemons reworked")

--@ chunk 143

-- clean the bottle to a dark green-glass mass, then one broad window highlight + a crisp specular
pBotD = pile{{"bone black",1.0},{"green earth",0.5},{"raw umber",0.4},{"Prussian blue",0.05}}
work(bottlemask,{hand="body",pile=pBotD,clip=true,coverage=6,angle=1.5,fill=true,dips={3,1.0},load=1.0,seed=6001})
work(bottlemask,{hand="body",pile=pBotD,clip=true,coverage=3,angle=0.1,fill=true,dips={3,1.0},load=1.0,seed=6002})
-- broad soft highlight on the left of the body
hl = mask(function(x,y) return clamp(1-math.abs(x-(291-0.04*(y-420)))/18,0,1) end)
hl = hl * mask(function(x,y) return clamp((y-355)/70,0,1)*clamp((625-y)/45,0,1) end) * bottlemask
work(hl,{hand="body",pile=pile{{"lead white",1.0},{"smalt",0.7},{"green earth",0.4},{"bone black",0.1}},clip=true,coverage=1.5,angle=1.5,seed=6010})
-- brighter core
core = mask(function(x,y) return clamp(1-math.abs(x-(293-0.04*(y-420)))/7,0,1) end)
core = core * mask(function(x,y) return clamp((y-395)/45,0,1)*clamp((615-y)/40,0,1) end) * bottlemask
work(core,{hand="body",pile=pile{{"lead white",2.2},{"smalt",0.8}},clip=true,coverage=1.3,angle=1.5,seed=6020})
-- reflected edge along the right contour
rr = (bottlemask:rim(6,4)) * grad(300,470,55,210, 1.0, 0.0, 0.35, 1.0)
work(rr,{hand="glaze",pile=pile{{"lead white",1.0},{"yellow ochre",0.9},{"raw umber",0.6},medium=0.3},clip=true,coverage=1.2,seed=6030})
-- left reflected edge (cool)
ll = (bottlemask:rim(5,3)) * grad(300,470,55,210, -1.0, 0.0, 0.35, 1.0)
work(ll,{hand="glaze",pile=pile{{"lead white",0.9},{"smalt",0.7},{"green earth",0.4},medium=0.3},clip=true,coverage=1.1,seed=6040})
print("bottle glass")

--@ chunk 144
print(wait(7*1440)); print("bottle",drying(300,500),"lemA",drying(600,498),"lemB",drying(700,504),"lemC",drying(124,645),"pear",drying(850,588),"table",drying(500,700),"wall",drying(500,200))

--@ chunk 145
print(wait(12*1440)); print("bottle",drying(300,500),"pear",drying(850,588),"table",drying(500,700),"wall",drying(500,200))

--@ chunk 146

pMidP = pile{{"yellow ochre",1.6},{"lead white",1.2},{"chrome yellow",0.8},{"green earth",0.25}}
pShP  = pile{{"yellow ochre",1.2},{"raw umber",1.2},{"green earth",0.6},{"lead white",0.3}}
pCoreP= pile{{"raw umber",1.4},{"yellow ochre",0.8},{"green earth",0.6},{"bone black",0.12}}
pLitP = pile{{"lead white",2.4},{"chrome yellow",1.0},{"yellow ochre",0.35}}
pReflP= pile{{"lead white",1.0},{"yellow ochre",1.5},{"red earth",0.4}}
local m = pear
local cx,cy,rx,ry = 850,590,52,78
work(m,{hand="body",pile=pMidP,clip=true,coverage=7,angle=0.2,fill=true,dips={3,1.0},load=1.0,seed=7001})
work(grad(cx,cy,rx,ry,0.75,0.65,0.28,0.82)*m,{hand="body",pile=pShP,clip=true,coverage=8,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=7010})
work(grad(cx,cy,rx,ry,0.85,0.55,0.62,0.95)*m,{hand="body",pile=pCoreP,clip=true,coverage=7,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=7020})
work(grad(cx,cy,rx,ry,-0.75,-0.6,0.52,1.0)*m,{hand="body",pile=pLitP,clip=true,coverage=7,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=7030})
work((m:rim(5,4))*grad(cx,cy+ry*0.5,rx,ry*0.62,0.3,0.9,0.35,1.0),{hand="body",pile=pReflP,clip=true,coverage=3,seed=7040})
print("pear solid")

--@ chunk 147

pShP2 = pile{{"raw umber",1.2},{"bone black",0.6},{"green earth",0.5},{"yellow ochre",0.9},{"lead white",0.2}}
pCoreP2 = pile{{"raw umber",1.0},{"bone black",0.8},{"green earth",0.5},{"yellow ochre",0.4}}
pLitP2 = pile{{"lead white",2.6},{"chrome yellow",1.0},{"yellow ochre",0.3}}
local m = pear
local cx,cy,rx,ry = 850,590,52,78
work(grad(cx,cy,rx,ry,0.80,0.60,0.30,0.85)*m,{hand="body",pile=pShP2,clip=true,coverage=10,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=7100})
work(grad(cx,cy,rx,ry,0.90,0.50,0.60,0.95)*m,{hand="body",pile=pCoreP2,clip=true,coverage=8,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=7110})
work(grad(cx,cy,rx,ry,-0.75,-0.6,0.50,1.0)*m,{hand="body",pile=pLitP2,clip=true,coverage=8,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=7120})
work((m:rim(5,4))*grad(cx,cy+ry*0.5,rx,ry*0.62,0.3,0.9,0.35,1.0),{hand="body",pile=pReflP,clip=true,coverage=3,seed=7130})
print("pear strong shadow")

--@ chunk 148

pBotD = pile{{"bone black",1.0},{"green earth",0.55},{"raw umber",0.4},{"Prussian blue",0.05}}
work(bottlemask,{hand="body",pile=pBotD,clip=true,coverage=6,angle=1.5,fill=true,dips={3,1.0},load=1.0,seed=8001})
work(bottlemask,{hand="body",pile=pBotD,clip=true,coverage=3,angle=0.1,fill=true,dips={3,1.0},load=1.0,seed=8002})
-- broad soft window highlight, laid wet into the fresh dark then fused
bh = brush{kind="filbert", width=15, point=0, stiffness=0.3}
bh:load(pile{{"lead white",1.6},{"smalt",0.9},{"green earth",0.4}},0.7)
bh:stroke({{287,398},{282,455},{279,510},{278,555},{281,598}},{pressure={0.34,0.30},ramps={0.15,0.2}})
bh:reload(pile{{"lead white",1.6},{"smalt",0.9},{"green earth",0.4}},0.7)
bh:stroke({{297,400},{292,458},{289,512},{288,556},{291,596}},{pressure={0.26,0.24},ramps={0.15,0.2}})
-- brighter core within it
bc = brush{kind="filbert", width=5, point=0.2, stiffness=0.35}
bc:load(pile{{"lead white",3.0},{"smalt",0.6}},0.8)
bc:stroke({{285,410},{281,460},{279,510},{279,552},{282,592}},{pressure={0.30,0.26},ramps={0.15,0.2}})
-- reflected warm edge just inside the right contour
be = brush{kind="round", width=4, point=0.7}
be:load(pile{{"lead white",1.1},{"yellow ochre",0.9},{"raw umber",0.6}},0.7)
be:stroke({{341,412},{344,462},{346,512},{346,556},{345,596}},{pressure={0.24,0.18},ramps={0.2,0.25}})
-- cool reflected edge on the far left
bl = brush{kind="round", width=3.5, point=0.7}
bl:load(pile{{"lead white",1.0},{"smalt",0.8},{"green earth",0.4}},0.6)
bl:stroke({{260,420},{258,470},{257,520},{258,565},{261,600}},{pressure={0.20,0.16},ramps={0.2,0.25}})
-- neck highlight, and the lit lip
bn = brush{kind="round", width=3.5, point=0.6}
bn:load(pile{{"lead white",2.4},{"smalt",0.5}},0.7)
bn:stroke({{294,242},{292,270},{291,300}},{pressure={0.24,0.22},ramps={0.15,0.2}})
bn:wipe(0.4); bn:load(pile{{"lead white",1.4},{"pale smalt",0.6}},0.6)
bn:stroke({{286,224},{296,223}},{pressure={0.3,0.3},ramps={0.2,0.2}})
-- crisp specular where the window is strongest
bsp = brush{kind="round", width=3.5, point=0.8}
bsp:load(pile{{"lead white",5}},0.95)
bsp:touch(280,468,{pressure=0.4, drag={0.0,0.5}})
bsp:wipe(0.5); bsp:load(pile{{"lead white",5}},0.9)
bsp:touch(297,236,{pressure=0.32})
-- fuse the highlight gently into the dark
blend(mask(function(x,y) return clamp(1-math.abs(x-292)/34,0,1) end) * bottlemask, {seed=8090, coverage=0.6, pressure={0.18,0.18}})
print("bottle cleaned + glass")

--@ chunk 149

wfree = wallmask - objall
pW = pile{{"raw umber",2.4},{"smalt",1.3},{"lead white",1.2},{"bone black",0.18}}
work(wfree,{hand="broad",pile=pW,clip=true,coverage=4.5,angle=0.08,fill=true,dips={3,0.9},seed=9001})
-- warm glow behind the bottle
glow = ellipse(300,390,300,230):soften(150)*wfree
work(glow,{hand="broad",pile=pile{{"raw umber",1.8},{"lead white",2.0},{"yellow ochre",0.8}},clip=true,coverage=2.0,angle=0.05,seed=9002})
-- top deepening
topd = mask(function(x,y) return clamp((250-y)/230,0,1) end)*wfree
work(topd,{hand="broad",pile=pile{{"raw umber",2.8},{"smalt",1.4},{"bone black",0.45},{"lead white",0.5}},clip=true,coverage=1.8,angle=0.1,seed=9003})
-- right deepening
rd = mask(function(x,y) return clamp((x-630)/370,0,1) end)*wfree
work(rd,{hand="broad",pile=pile{{"raw umber",2.6},{"smalt",1.3},{"bone black",0.35},{"lead white",0.5}},clip=true,coverage=1.5,angle=0.1,seed=9004})
blend(wfree,{seed=9005,coverage=1.2,pressure={0.28,0.28}})
print("wall evened")

--@ chunk 150

wfree = wallmask - objall
pW = pile{{"raw umber",2.4},{"smalt",1.3},{"lead white",1.2},{"bone black",0.18}}
work(wfree,{hand="broad",pile=pW,clip=true,coverage=5,angle=0.08,fill=true,dips={3,0.9},seed=9101})
work(wfree,{hand="broad",pile=pW,clip=true,coverage=3,angle=1.60,fill=true,dips={3,0.9},seed=9102})
glow = ellipse(300,390,290,225):soften(150)*wfree
work(glow,{hand="broad",pile=pile{{"raw umber",1.9},{"lead white",2.0},{"yellow ochre",0.8}},clip=true,coverage=1.8,angle=0.05,seed=9110})
topd = mask(function(x,y) return clamp((250-y)/230,0,1) end)*wfree
work(topd,{hand="broad",pile=pile{{"raw umber",2.8},{"smalt",1.4},{"bone black",0.45},{"lead white",0.5}},clip=true,coverage=1.5,angle=0.1,seed=9111})
rd = mask(function(x,y) return clamp((x-640)/360,0,1) end)*wfree
work(rd,{hand="broad",pile=pile{{"raw umber",2.6},{"smalt",1.3},{"bone black",0.3},{"lead white",0.5}},clip=true,coverage=1.4,angle=0.1,seed=9112})
print("wall repainted even")

--@ chunk 151
print(wait(0)); print("pear",drying(850,588),"lemA",drying(600,498),"lemB",drying(700,504),"lemC",drying(124,645)); print("bottle",drying(300,500),"wall",drying(500,200),"wallR",drying(800,250),"table",drying(400,700),"bowlout",drying(640,600),"bowlin",drying(640,545))

--@ chunk 152
print(wait(5*1440)); print("pear",drying(850,588),"bottle",drying(300,500),"wall",drying(500,200),"lemA",drying(600,498))

--@ chunk 153
print(wait(8*1440)); print("pear",drying(850,588),"bottle",drying(300,500),"wall",drying(500,200))

--@ chunk 154

pMidP = pile{{"yellow ochre",1.6},{"chrome yellow",0.7},{"lead white",0.9},{"green earth",0.2}}
pShP  = pile{{"yellow ochre",1.2},{"raw umber",1.1},{"green earth",0.5},{"lead white",0.3}}
pCoreP= pile{{"raw umber",1.2},{"yellow ochre",0.9},{"green earth",0.55},{"bone black",0.1}}
pMidLitP = pile{{"yellow ochre",1.3},{"chrome yellow",1.0},{"lead white",1.3}}
pLitP = pile{{"lead white",1.8},{"chrome yellow",1.3},{"yellow ochre",0.4}}
pHotP = pile{{"lead white",2.4},{"chrome yellow",1.2}}
pReflP= pile{{"lead white",1.1},{"yellow ochre",1.4},{"red earth",0.4}}
local m = pear
local cx,cy,rx,ry = 850,590,52,78
work(m,{hand="body",pile=pMidP,clip=true,coverage=6,angle=0.2,fill=true,dips={3,1.0},load=1.0,seed=9501})
work(grad(cx,cy,rx,ry,0.75,0.65,0.30,0.80)*m,{hand="body",pile=pShP,clip=true,coverage=8,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=9510})
work(grad(cx,cy,rx,ry,0.85,0.55,0.62,0.95)*m,{hand="body",pile=pCoreP,clip=true,coverage=7,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=9520})
work(grad(cx,cy,rx,ry,-0.75,-0.6,0.38,0.72)*m,{hand="body",pile=pMidLitP,clip=true,coverage=6,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=9530})
work(grad(cx,cy,rx,ry,-0.75,-0.6,0.62,1.0)*m,{hand="body",pile=pLitP,clip=true,coverage=6,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=9540})
work(grad(cx,cy,rx,ry,-0.8,-0.6,0.82,1.0)*m,{hand="body",pile=pHotP,clip=true,coverage=4,angle=0.6,seed=9550})
work((m:rim(5,4))*grad(cx,cy+ry*0.5,rx,ry*0.62,0.3,0.9,0.35,1.0),{hand="body",pile=pReflP,clip=true,coverage=3,seed=9560})
print("pear rebuilt")

--@ chunk 155

work(grad(850,590,52,78,0.85,0.55,0.55,0.62)*pear,{hand="body",pile=pile{{"raw umber",1.0},{"bone black",0.8},{"green earth",0.4},{"yellow ochre",0.4}},clip=true,coverage=10,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=9600})
print("dark test")

--@ chunk 156

pBodyF = pile{{"yellow ochre",1.4},{"raw umber",0.9},{"green earth",0.5},{"lead white",0.5}}
pCoreF = pile{{"yellow ochre",1.1},{"raw umber",1.1},{"green earth",0.5},{"bone black",0.12}}
pLitF  = pile{{"lead white",2.0},{"chrome yellow",1.3},{"yellow ochre",0.3}}
pReflF = pile{{"lead white",1.1},{"yellow ochre",1.4},{"red earth",0.4}}
pHotF  = pile{{"lead white",2.6},{"chrome yellow",1.1}}
function roundFruit(m,cx,cy,rx,ry,seed)
  local litE = ellipse(cx-0.62*rx, cy-0.08*ry, 0.95*rx, 1.02*ry)
  local lit = litE:soften(5)
  local shadow = m - lit
  work(m,{hand="body",pile=pBodyF,clip=true,coverage=8,angle=0.3,fill=true,dips={3,1.0},load=1.0,seed=seed})
  work(shadow * litE:grow(9),{hand="body",pile=pCoreF,clip=true,coverage=6,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=seed+1})
  work(lit*m,{hand="body",pile=pLitF,clip=true,coverage=8,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=seed+2})
  work((m:rim(4,4))*grad(cx,cy+ry*0.45,rx,ry*0.62,0.25,0.9,0.35,1.0),{hand="body",pile=pReflF,clip=true,coverage=3,seed=seed+3})
  work(grad(cx,cy,rx,ry,-0.8,-0.6,0.85,1.0)*m,{hand="body",pile=pHotF,clip=true,coverage=3,seed=seed+4})
end
frontrim = rimband * mask(function(x,y) return (y > 539) and 1 or 0 end)
roundFruit(lemA - frontrim,600,498,60,46,9700)
roundFruit(lemB - frontrim,700,504,54,42,9720)
roundFruit(lemC,124,645,58,34,9740)
print("lemons rounded")

--@ chunk 157

pBodyF = pile{{"yellow ochre",1.4},{"raw umber",0.9},{"green earth",0.5},{"lead white",0.5}}
pCoreF = pile{{"yellow ochre",1.1},{"raw umber",1.1},{"green earth",0.5},{"bone black",0.12}}
pLitF  = pile{{"lead white",2.0},{"chrome yellow",1.3},{"yellow ochre",0.3}}
pReflF = pile{{"lead white",1.1},{"yellow ochre",1.4},{"red earth",0.4}}
pHotF  = pile{{"lead white",2.6},{"chrome yellow",1.1}}
function roundFruit(m,cx,cy,rx,ry,seed)
  local litE = ellipse(cx-1.0*rx, cy-0.4*ry, 1.3*rx, 1.8*ry)
  local lit = litE:soften(5)
  local shadow = m - lit
  work(m,{hand="body",pile=pBodyF,clip=true,coverage=8,angle=0.3,fill=true,dips={3,1.0},load=1.0,seed=seed})
  work(shadow * litE:grow(9),{hand="body",pile=pCoreF,clip=true,coverage=6,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=seed+1})
  work(lit*m,{hand="body",pile=pLitF,clip=true,coverage=8,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=seed+2})
  work((m:rim(4,4))*grad(cx,cy+ry*0.45,rx,ry*0.62,0.25,0.9,0.35,1.0),{hand="body",pile=pReflF,clip=true,coverage=3,seed=seed+3})
  work(grad(cx,cy,rx,ry,-0.8,-0.6,0.85,1.0)*m,{hand="body",pile=pHotF,clip=true,coverage=3,seed=seed+4})
end
frontrim = rimband * mask(function(x,y) return (y > 539) and 1 or 0 end)
roundFruit(lemA - frontrim,600,498,60,46,9800)
roundFruit(lemB - frontrim,700,504,54,42,9820)
roundFruit(lemC,124,645,58,34,9840)
print("lemons rounded v2")

--@ chunk 158

pRefW = pile{{"lead white",1.4},{"yellow ochre",1.5},{"red earth",0.35}}
work((lemA:rim(5,4))*grad(600,498+23,60,29,0.25,0.9,0.3,1.0),{hand="body",pile=pRefW,clip=true,coverage=3,seed=9900})
work((lemB:rim(5,4))*grad(700,504+21,54,26,0.25,0.9,0.3,1.0),{hand="body",pile=pRefW,clip=true,coverage=3,seed=9910})
work((lemC:rim(5,4))*grad(124,645+17,58,21,0.25,0.9,0.3,1.0),{hand="body",pile=pRefW,clip=true,coverage=3,seed=9920})
work((pear:rim(5,4))*grad(850,590+39,52,48,0.3,0.9,0.35,1.0),{hand="body",pile=pRefW,clip=true,coverage=3,seed=9930})
bs=brush{kind="round",width=4,point=0.8}
bs:load(pile{{"lead white",5}},0.95); bs:touch(573,472,{pressure=0.42,drag={0.35,0.3}})
bs:wipe(0.35); bs:load(pile{{"lead white",5}},0.9); bs:touch(673,478,{pressure=0.40,drag={0.35,0.3}})
bs:wipe(0.35); bs:load(pile{{"lead white",5}},0.9); bs:touch(99,626,{pressure=0.40,drag={0.4,0.3}})
bs:wipe(0.35); bs:load(pile{{"lead white",5}},0.9); bs:touch(834,540,{pressure=0.42,drag={0.3,0.45}})
print("fruit rims + speculars")

--@ chunk 159

owym = bowlmask - open
pBowlBody = pile{{"lead white",3.4},{"pale smalt",0.6},{"yellow ochre",0.35}}
pBowlSh   = pile{{"lead white",1.5},{"raw umber",1.2},{"pale smalt",1.2},{"bone black",0.08}}
pBowlCore = pile{{"lead white",1.0},{"raw umber",1.5},{"pale smalt",1.2},{"bone black",0.15}}
pBowlLit  = pile{{"lead white",5.0},{"yellow ochre",0.3}}
pBowlRef  = pile{{"lead white",3.0},{"yellow ochre",1.0},{"red earth",0.25}}
work(owym,{hand="body",pile=pBowlBody,clip=true,coverage=6,angle=0.5,fill=true,dips={3,1.0},load=1.0,seed=10001})
work(grad(640,600,132,74,0.85,0.50,0.30,0.85)*owym,{hand="body",pile=pBowlSh,clip=true,coverage=6,angle=0.5,fill=true,dips={3,1.0},load=1.0,seed=10010})
work(grad(640,600,132,74,0.90,0.45,0.60,0.90)*owym,{hand="body",pile=pBowlCore,clip=true,coverage=5,angle=0.5,fill=true,dips={3,1.0},load=1.0,seed=10020})
work(grad(640,600,132,74,-0.85,-0.40,0.40,1.0)*owym,{hand="body",pile=pBowlLit,clip=true,coverage=5,angle=0.5,fill=true,dips={3,1.0},load=1.0,seed=10030})
work((owym:rim(6,4))*grad(640,650,132,60,0.1,0.9,0.30,1.0),{hand="body",pile=pBowlRef,clip=true,coverage=2,seed=10040})
-- rim: far/lit side bright, near/right side shaded
work(rimband*grad(640,538,126,36,-0.85,-0.5,0.35,1.0),{hand="body",pile=pBowlLit,clip=true,coverage=3,seed=10050})
work(rimband*grad(640,538,126,36,0.80,0.5,0.35,1.0),{hand="body",pile=pBowlSh,clip=true,coverage=3,seed=10060})
-- interior
interior = open_in - (lemA+lemB)
work(interior,{hand="body",pile=pile{{"lead white",1.5},{"raw umber",1.2},{"smalt",1.2}},clip=true,coverage=5,angle=0.2,fill=true,dips={3,0.95},seed=10070})
work(grad(640,538,113,25,0.0,-1.0,0.30,1.0)*interior,{hand="body",pile=pile{{"lead white",2.4},{"yellow ochre",0.8}},clip=true,coverage=4,seed=10080})
work(grad(640,538,113,25,0.0,1.0,0.30,1.0)*interior,{hand="body",pile=pile{{"lead white",2.0},{"yellow ochre",1.0},{"red earth",0.25}},clip=true,coverage=3,seed=10090})
work(((ellipse(600,544,72,20)+ellipse(700,550,64,19)):soften(10) * interior),{hand="body",pile=pile{{"raw umber",1.8},{"smalt",1.0},{"lead white",0.8}},clip=true,coverage=4,seed=10100})
print("bowl done")

--@ chunk 160

pMidSoft = pile{{"yellow ochre",1.3},{"lead white",1.2},{"chrome yellow",0.6}}
function softRing(m,cx,cy,rx,ry,seed)
  local litE = ellipse(cx-1.0*rx, cy-0.4*ry, 1.3*rx, 1.8*ry)
  local ring = (litE:grow(5) - litE:shrink(5)) * m
  work(ring,{hand="body",pile=pMidSoft,clip=true,coverage=3,angle=0.6,seed=seed})
end
frontrim = rimband * mask(function(x,y) return (y > 539) and 1 or 0 end)
softRing(lemA-frontrim,600,498,60,46,11001)
softRing(lemB-frontrim,700,504,54,42,11002)
softRing(lemC,124,645,58,34,11003)
work(grad(850,590,52,78,0.85,0.55,0.46,0.70)*pear,{hand="body",pile=pile{{"yellow ochre",1.4},{"lead white",1.0},{"chrome yellow",0.4},{"green earth",0.2}},clip=true,coverage=3,angle=0.6,seed=11004})
print("terminators softened")

--@ chunk 161
print(wait(3*1440)); print("lemA",drying(600,470),"lemB",drying(700,478),"lemC",drying(120,626),"pear",drying(830,560),"bowl",drying(640,600),"bottle",drying(300,500),"table",drying(400,700),"wall",drying(500,200))

--@ chunk 162
print(wait(6*1440)); print("lemA",drying(600,470),"lemB",drying(700,478),"lemC",drying(120,626),"pear",drying(830,560),"bowl",drying(640,600))

--@ chunk 163

A0 = pile{{"lead white",2.8},{"chrome yellow",1.0}}
A1 = pile{{"lead white",2.3},{"chrome yellow",1.2},{"yellow ochre",0.2}}
A2 = pile{{"lead white",1.8},{"chrome yellow",1.2},{"yellow ochre",0.5}}
A3 = pile{{"yellow ochre",1.4},{"lead white",0.9},{"chrome yellow",0.8},{"green earth",0.15}}
A4 = pile{{"yellow ochre",1.5},{"lead white",0.6},{"raw umber",0.4},{"green earth",0.3}}
Acore = pile{{"yellow ochre",1.1},{"raw umber",1.1},{"green earth",0.5},{"bone black",0.12}}
Arefl = pile{{"lead white",1.3},{"yellow ochre",1.5},{"red earth",0.35}}
function fruitRamp(m,cx,cy,rx,ry,seed)
  local px,py = cx-0.9*rx, cy-0.5*ry
  local function disc(k) return ellipse(px,py, k*rx, k*ry) end
  local cres = m - disc(1.55)
  work(m,{hand="body",pile=A4,clip=true,coverage=7,angle=0.3,fill=true,dips={3,1.0},load=1.0,seed=seed})
  work(disc(1.55)*m,{hand="body",pile=A3,clip=true,coverage=7,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=seed+1})
  work(disc(1.40)*m,{hand="body",pile=A2,clip=true,coverage=6,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=seed+2})
  work(disc(1.25)*m,{hand="body",pile=A1,clip=true,coverage=6,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=seed+3})
  work(disc(1.10)*m,{hand="body",pile=A0,clip=true,coverage=5,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=seed+4})
  work(cres*disc(1.55):grow(11),{hand="body",pile=Acore,clip=true,coverage=6,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=seed+5})
  work(cres*(m:rim(5,3))*grad(cx,cy+ry*0.4,rx,ry*0.6,0.25,0.9,0.3,1.0),{hand="body",pile=Arefl,clip=true,coverage=3,seed=seed+6})
end
frontrim = rimband * mask(function(x,y) return (y > 539) and 1 or 0 end)
fruitRamp(lemA-frontrim,600,498,60,46,12001)
fruitRamp(lemB-frontrim,700,504,54,42,12020)
fruitRamp(lemC,124,645,58,34,12040)
fruitRamp(pear,850,590,52,78,12060)
print("fruit ramp")

--@ chunk 164

frontrim = rimband * mask(function(x,y) return (y > 539) and 1 or 0 end)
blend((lemA-frontrim),{seed=13001,coverage=1.3,pressure={0.26,0.26}})
blend((lemB-frontrim),{seed=13002,coverage=1.3,pressure={0.26,0.26}})
blend(lemC,{seed=13003,coverage=1.3,pressure={0.26,0.26}})
blend(pear,{seed=13004,coverage=1.3,pressure={0.26,0.26}})
print("fruit blended")

--@ chunk 165
print(wait(4*1440)); print("lemA",drying(600,470),"lemB",drying(700,478),"lemC",drying(120,626),"pear",drying(830,560))

--@ chunk 166
print(wait(6*1440)); print("lemA",drying(600,470),"lemC",drying(120,626),"pear",drying(830,560))

--@ chunk 167

local cx,cy,rx,ry = 124,645,58,34
local m = lemC
local pBase = pile{{"yellow ochre",1.3},{"raw umber",0.9},{"green earth",0.45},{"lead white",0.5}}
local pLight = pile{{"lead white",2.4},{"chrome yellow",1.1},{"yellow ochre",0.3}}
work(m,{hand="body",pile=pBase,clip=true,coverage=7,angle=0.3,fill=true,dips={3,1.0},load=1.0,seed=14001})
local tt = function(x,y) return clamp(0.5+0.5*(((x-cx)/rx)*0.85 + ((y-cy)/ry)*0.6),0,1) end
work(m,{hand="body",pile=pLight,clip=true,coverage=6,angle=0.6,fill=true,dips={3,1.0},load_at=function(x,y) return 0.10+0.95*tt(x,y) end,seed=14002})
work(grad(cx,cy,rx,ry,-0.8,-0.6,0.80,1.0)*m,{hand="body",pile=pile{{"lead white",2.8},{"chrome yellow",1.0}},clip=true,coverage=3,seed=14003})
work((m:rim(4,3))*grad(cx,cy+ry*0.4,rx,ry*0.6,0.25,0.9,0.3,1.0),{hand="body",pile=pile{{"lead white",1.3},{"yellow ochre",1.5},{"red earth",0.35}},clip=true,coverage=2,seed=14004})
print("lemC smooth test")

--@ chunk 168

local cx,cy,rx,ry = 600,498,60,46
local m = lemA - frontrim
local litE = ellipse(cx-0.95*rx, cy-0.45*ry, 1.35*rx, 1.75*ry):soften(14)
local pBase = pile{{"yellow ochre",1.35},{"raw umber",0.85},{"green earth",0.45},{"lead white",0.5}}
local pLit  = pile{{"lead white",2.2},{"chrome yellow",1.2},{"yellow ochre",0.3}}
local pHot  = pile{{"lead white",2.8},{"chrome yellow",1.0}}
local pCore = pile{{"yellow ochre",1.0},{"raw umber",1.2},{"green earth",0.5},{"bone black",0.12}}
local pRefl = pile{{"lead white",1.3},{"yellow ochre",1.5},{"red earth",0.35}}
work(m,{hand="body",pile=pBase,clip=true,coverage=7,angle=0.3,fill=true,dips={3,1.0},load=1.0,seed=15001})
work(litE*m,{hand="body",pile=pLit,clip=true,coverage=10,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=15002})
work(grad(cx,cy,rx,ry,-0.8,-0.6,0.85,1.0)*m,{hand="body",pile=pHot,clip=true,coverage=3,seed=15003})
work(((litE:grow(14)-litE):soften(8))*m,{hand="body",pile=pCore,clip=true,coverage=4,angle=0.6,seed=15004})
work((m:rim(4,3))*grad(cx,cy+ry*0.4,rx,ry*0.6,0.25,0.9,0.3,1.0),{hand="body",pile=pRefl,clip=true,coverage=2,seed=15005})
print("lemA soft test")

--@ chunk 169

local pBase = pile{{"yellow ochre",1.35},{"raw umber",0.85},{"green earth",0.45},{"lead white",0.5}}
local pLit  = pile{{"lead white",2.2},{"chrome yellow",1.2},{"yellow ochre",0.3}}
local pHot  = pile{{"lead white",2.8},{"chrome yellow",1.0}}
local pCore = pile{{"yellow ochre",1.0},{"raw umber",1.2},{"green earth",0.5},{"bone black",0.12}}
local pRefl = pile{{"lead white",1.3},{"yellow ochre",1.5},{"red earth",0.35}}
function softFruit(m,cx,cy,rx,ry,seed)
  local litE = ellipse(cx-0.95*rx, cy-0.45*ry, 1.35*rx, 1.75*ry):soften(14)
  work(m,{hand="body",pile=pBase,clip=true,coverage=7,angle=0.3,fill=true,dips={3,1.0},load=1.0,seed=seed})
  work(litE*m,{hand="body",pile=pLit,clip=true,coverage=10,angle=0.6,fill=true,dips={3,1.0},load=1.0,seed=seed+1})
  work(grad(cx,cy,rx,ry,-0.8,-0.6,0.85,1.0)*m,{hand="body",pile=pHot,clip=true,coverage=3,seed=seed+2})
  work(((litE:grow(14)-litE):soften(8))*m,{hand="body",pile=pCore,clip=true,coverage=4,angle=0.6,seed=seed+3})
  work((m:rim(4,3))*grad(cx,cy+ry*0.4,rx,ry*0.6,0.25,0.9,0.3,1.0),{hand="body",pile=pRefl,clip=true,coverage=2,seed=seed+4})
end
softFruit(lemB-frontrim,700,504,54,42,15100)
softFruit(lemC,124,645,58,34,15120)
softFruit(pear,850,590,52,78,15140)
print("soft fruit done")

--@ chunk 170
print(wait(4*1440)); print("lemA",drying(600,470),"lemB",drying(700,478),"lemC",drying(120,626),"pear",drying(830,560))

--@ chunk 171
print(wait(6*1440)); print("lemA",drying(600,470),"lemC",drying(120,626),"pear",drying(830,560),"bowl",drying(640,600),"bottle",drying(300,500))

--@ chunk 172

local pW = pile{{"lead white",5}}
local b = brush{kind="round", width=3.5, point=0.85}
b:load(pW,0.95); b:touch(570,468,{pressure=0.42, drag={0.35,0.3}})
b:wipe(0.4); b:load(pW,0.9); b:touch(670,474,{pressure=0.40, drag={0.35,0.3}})
b:wipe(0.4); b:load(pW,0.9); b:touch(95,624,{pressure=0.40, drag={0.4,0.3}})
b:wipe(0.4); b:load(pW,0.9); b:touch(826,536,{pressure=0.42, drag={0.3,0.45}})
-- small crisp lights on the bowl rim
local b2 = brush{kind="round", width=3, point=0.8}
b2:load(pW,0.85); b2:touch(524,540,{pressure=0.32, drag={0.3,0.15}})
b2:wipe(0.4); b2:load(pW,0.8); b2:touch(548,562,{pressure=0.28, drag={0.3,0.1}})
print("final speculars")
