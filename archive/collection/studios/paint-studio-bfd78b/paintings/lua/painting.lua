-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=500, aspect=1.35, linen={16, 14}, seed=7,
  ground={
    {pile={{"lead white", 6}, {"yellow ochre", 1.5}, {"raw umber", 0.5}}, um=120, apply="knife", texture=0.25},
    {pile={{"yellow ochre", 2}, {"red earth", 1}, {"lead white", 4}}, um=30, apply="brush"}
  }}
print("W", W, "H", H)
print(table.concat(tubes(), ", "))

--@ chunk 2
h = pencil("2H")
-- horizon and far shore
h:line({{0,470},{200,468},{420,469},{610,468},{700,462},{770,454},{1000,449}}, {pressure=0.25})
-- tree clump skyline on the right headland
h:line({{770,454},{790,430},{812,410},{838,394},{862,400},{884,386},{910,402},{938,420},{966,438},{1000,448}}, {pressure=0.25})
-- windmill: tower and sail cross
h:line({{690,463},{696,420},{700,420},{705,463}}, {pressure=0.25, smooth=false})
h:line({{684,404},{712,436}}, {pressure=0.2})
h:line({{715,406},{682,432}}, {pressure=0.2})
-- boat: hull, mast, sail
h:line({{345,548},{360,556},{430,556},{448,544}}, {pressure=0.3})
h:line({{398,556},{398,436}}, {pressure=0.25})
h:line({{400,540},{400,452},{440,470},{446,540}}, {pressure=0.25, smooth=false})
-- sun and light path
h:line({{300,452},{330,446},{360,452}}, {pressure=0.2})
h:line({{270,480},{200,741}}, {pressure=0.15})
h:line({{390,480},{520,741}}, {pressure=0.15})
-- left foreground bank and post
h:line({{0,650},{80,690},{170,725},{230,741}}, {pressure=0.3})
h:line({{112,560},{118,700}}, {pressure=0.3})
-- cloud bank strata and big cloud at right
h:line({{60,415},{200,405},{330,412},{470,404},{560,418}}, {pressure=0.2})
h:line({{560,300},{640,290},{760,310},{880,296},{1000,290}}, {pressure=0.2})
h:line({{560,300},{540,200},{590,80},{680,20},{760,0}}, {pressure=0.2})
print("drawn")

--@ chunk 3
print(wait(0))
sky_top = pile{{"cobalt blue",2},{"smalt",3},{"lead white",3},{"raw umber",0.3}, medium=0.15}
sky_mid = pile{{"pale smalt",3},{"lead white",4},{"yellow ochre",0.3},{"cobalt blue",0.5}, medium=0.15}
sky_low = pile{{"lead white",6},{"yellow ochre",1},{"chrome yellow",0.3},{"vermilion",0.15}, medium=0.15}
sky_glow = pile{{"lead white",8},{"chrome yellow",0.6}, medium=0.1}
-- first lay-in band across the top of the sky
local band1 = rect(0, 0, W, 170)
work(band1, {hand="broad", pile=sky_top, angle=0.04, coverage=2.2, seed=11})
print(wait(0))

--@ chunk 4
print(drying(500, 80), drying(500, 300))
local band2 = rect(0, 120, W, 200)
work(band2, {hand="broad", tool={kind="flat", width=40}, pile=sky_mid, angle=0.03, coverage=3.5, fill=true, seed=12})
print(wait(0))
print(drying(500, 80))

--@ chunk 5
blend(rect(0, 60, W, 200), {angle=0})
print(wait(0))

--@ chunk 6
sky_dark = pile{{"cobalt blue",2},{"smalt",3},{"lead white",1.5},{"raw umber",0.5},{"Prussian blue",0.1}, medium=0.15}
sky_low_r = pile{{"lead white",5},{"pale smalt",2},{"red earth",0.3},{"yellow ochre",0.2}, medium=0.15}
-- close up the top band
work(rect(0, 0, W, 140), {hand="broad", tool={kind="flat", width=40}, pile=sky_top, angle=0.03, coverage=3.5, fill=true, seed=21})
work(rect(0, 0, W, 55), {hand="broad", tool={kind="flat", width=40}, pile=sky_dark, angle=0.03, coverage=3, fill=true, seed=22})
blend(rect(0, 0, W, 200), {angle=0})
-- lower sky: warm at left, cooler at right
work(rect(0, 290, 640, 200), {hand="broad", tool={kind="flat", width=40}, pile=sky_low, angle=0.02, coverage=3.5, fill=true, seed=23})
work(rect(560, 290, 440, 200), {hand="broad", tool={kind="flat", width=40}, pile=sky_low_r, angle=0.02, coverage=3.5, fill=true, seed=24})
blend(rect(0, 200, W, 290), {angle=0})
print(wait(0))

--@ chunk 7
local lumps = {
 {760,262,0, 150,45,60}, {905,255,20, 130,50,50}, {655,272,10, 80,32,40},
 {790,205,30, 115,60,60}, {885,185,40, 95,65,50}, {700,200,30, 70,48,45},
 {805,135,40, 85,58,55}, {735,120,30, 58,44,40}, {870,105,50, 62,50,45},
 {815,68,40, 52,40,35}, {965,200,20, 70,80,40}, {990,270,10, 60,42,30},
 {930,120,35, 55,45,40}, {690,140,20, 45,38,30}
}
local s
for i, l in ipairs(lumps) do
  local e = body.ellipsoid({l[1], l[2], l[3]}, {l[4], l[5], l[6]}):rough(10, 55, i)
  s = s and s:union(e) or e
end
cumA = form{ {s}, light={from={-1, 0.35}, front=0.35, ambient=0.25} }
print(cumA:value(760, 262), cumA:value(700, 200), cumA:value(880, 120), cumA:value(300, 300))
cl_mid = pile{{"lead white",5},{"pale smalt",1.5},{"yellow ochre",0.5},{"red earth",0.3}, medium=0.1}
cl_lit = pile{{"lead white",7},{"chrome yellow",0.5},{"vermilion",0.25},{"yellow ochre",0.3}, medium=0.1}
cl_shadow = pile{{"lead white",3},{"smalt",2.5},{"red earth",0.8},{"raw umber",0.5}, medium=0.1}
work(cumA:silhouette{}, {hand="body", pile=cl_mid, angle=0.1, coverage=2.5, fill=true, edge="soft", seed=31})
print(wait(0))

--@ chunk 8
work(cumA:shadow{}, {hand="body", pile=cl_shadow, angle=0.1, coverage=2, fill=true, edge="soft", seed=32})
print(wait(0))

--@ chunk 9
-- wipe the failed cloud back into the sky
work(rect(540, 0, 460, 150), {hand="broad", tool={kind="flat", width=40}, pile=sky_top, angle=0.03, coverage=4, fill=true, seed=41})
work(rect(540, 110, 460, 230), {hand="broad", tool={kind="flat", width=40}, pile=sky_mid, angle=0.03, coverage=4, fill=true, seed=42})
work(rect(540, 300, 460, 190), {hand="broad", tool={kind="flat", width=40}, pile=sky_low_r, angle=0.02, coverage=3, fill=true, seed=43})
blend(rect(500, 0, 500, 500), {angle=0})
blend(rect(0, 100, W, 380), {angle=0})
print(wait(0))

--@ chunk 10
blend(rect(380, 60, 620, 100), {angle=math.pi/2})
local o = outline{{100,150},{125,125},{160,112},{195,100},{225,115},{250,130},{270,150},{240,158},{170,160},{120,158}, closed=true, char="soft", lobe=16, seed=5}
smallcloud = o:mask()
work(smallcloud, {hand="body", pile=cl_mid, angle=0.1, coverage=2.5, fill=true, edge="soft", seed=51})
print(wait(0))

--@ chunk 11
local low = smallcloud * below({{90,140},{280,152}})
local up = smallcloud * above({{90,134},{280,142}})
work(low, {hand="body", pile=cl_shadow, length={30,70}, angle=0.05, coverage=2, fill=true, edge="soft", seed=52})
work(up, {hand="body", pile=cl_lit, length={30,70}, angle=-0.1, coverage=2, fill=true, edge="soft", seed=53})
blend(smallcloud:grow(3), {angle=0.15})
print(wait(0))

--@ chunk 12
local n = lose(smallcloud, {pile=sky_mid, where=0.8, load=0.3})
print(n)
print(wait(0))

--@ chunk 13
local rimr = smallcloud:rim(16, 8)
blend(rimr, {angle=0.25, clip=false})
print(wait(0))

--@ chunk 14
print(drying(500,30), drying(500,200), drying(500,380), drying(170,130), drying(800,200))

--@ chunk 15
cl_bank = pile{{"lead white",2.5},{"smalt",3},{"red earth",0.8},{"raw umber",0.9},{"cobalt blue",0.5}, medium=0.1}
cl_glow = pile{{"lead white",5},{"vermilion",0.5},{"chrome yellow",0.9},{"yellow ochre",0.3}, medium=0.1}
local ob = outline{{-20,412},{80,404},{160,409},{240,399},{320,405},{400,397},{480,403},{560,393},{640,399},{720,409},{820,414},{1020,414},
  {1020,436},{820,432},{700,426},{600,433},{500,427},{400,435},{300,431},{200,437},{100,431},{-20,436}, closed=true, char="soft", lobe=40, amount=0.6, seed=8}
bankm = ob:mask()
work(bankm, {hand="broad", tool={kind="flat", width=24}, pile=cl_bank, angle=0.02, coverage=3, fill=true, edge="soft", seed=61})
local under = bankm * below({{-20,424},{300,424},{600,426},{1020,428}})
work(under, {hand="broad", tool={kind="flat", width=14}, pile=cl_glow, angle=0.0, coverage=2.5, fill=true, edge="soft", seed=62})
blend(bankm, {angle=0})
print(wait(0))

--@ chunk 16
blend(rect(300, 30, 700, 300), {angle=0})
blend(rect(300, 30, 700, 300), {angle=0.03})
print(wait(0))

--@ chunk 17
blend(rect(240, 20, 120, 320), {angle=0, clip=false})
print(wait(0))

--@ chunk 18
cl_wedge = pile{{"lead white",2},{"smalt",3},{"red earth",0.8},{"raw umber",1.2},{"cobalt blue",0.6},{"Prussian blue",0.1}, medium=0.1}
cl_cool = pile{{"lead white",4},{"pale smalt",2},{"yellow ochre",0.3},{"red earth",0.1}, medium=0.1}
local ow = outline{{400,292},{470,272},{520,258},{548,238},{572,214},{612,204},{628,176},{660,160},{690,140},{700,112},{738,98},{760,72},{800,60},{830,40},{880,30},{930,22},{1020,0},
  {1020,330},{930,322},{830,330},{740,318},{650,324},{560,312},{480,304}, closed=true, char="soft", lobe=42, amount=0.8, seed=9}
wedge = ow:mask()
work(wedge, {hand="broad", tool={kind="flat", width=26}, pile=cl_wedge, angle=-0.15, coverage=3.2, fill=true, edge="soft", seed=71})
print(wait(0))

--@ chunk 19
local under = (wedge - wedge:shrink(30)) * below({{380,292},{500,286},{700,292},{1000,298}})
work(under, {hand="body", tool={kind="flat", width=12}, pile=cl_glow, angle=-0.03, coverage=2, fill=true, clip=wedge, edge="soft", seed=72})
local rim2 = (wedge - wedge:shrink(24)) * above({{380,330},{600,290},{800,140},{1000,80}})
work(rim2, {hand="body", tool={kind="round", width=8}, pile=cl_lit, angle=-0.45, coverage=2.2, fill=true, clip=wedge, seed=73})
print(wait(0))

--@ chunk 20
rim2 = (wedge - wedge:shrink(24)) * above({{380,330},{600,290},{800,140},{1000,80}})
under = (wedge - wedge:shrink(30)) * below({{380,292},{500,286},{700,292},{1000,298}})
-- soften the top rim back into the cloud: it is a backlit edge, not a bright cord
blend(rim2:grow(10), {angle=-0.45, clip=wedge})
blend(rim2:grow(12), {angle=1.2, clip=wedge})
blend(rim2:grow(12), {angle=-0.2, clip=wedge})
print(wait(0))

--@ chunk 21
blend(under:grow(14), {angle=0.0, clip=wedge})
blend(under:grow(14), {angle=1.5, clip=wedge})
print(wait(0))

--@ chunk 22
print(wait(0))
print("top", drying(500,30), "mid", drying(500,200), "low", drying(500,380), "cloud", drying(800,200), "smallcloud", drying(170,130), "ground", drying(500,600))

--@ chunk 23
sun_hi = pile{{"lead white",9},{"chrome yellow",0.5},{"vermilion",0.05}, medium=0.08}
glowm = ellipse(300, 448, 230, 62)
work(glowm, {hand="body", tool={kind="flat", width=22}, pile=sky_glow, angle=0.0, coverage=3, fill=true, edge="lost", seed=81})
work(ellipse(300, 452, 110, 34), {hand="body", tool={kind="flat", width=16}, pile=sun_hi, angle=0.0, coverage=3, fill=true, edge="lost", seed=82})
blend(ellipse(300, 448, 260, 80), {angle=0, clip=false})
print(wait(0))

--@ chunk 24
sky_deep = pile{{"smalt",3},{"cobalt blue",1},{"Prussian blue",0.15},{"lead white",1.2},{"raw umber",0.5}, medium=0.25}
-- darken and enrich the upper sky, heavier toward the corners
work(rect(0, 0, W, 90), {hand="glaze", pile=sky_deep, angle=0.02, coverage=2, seed=91})
work(rect(0, 0, 380, 60), {hand="glaze", pile=sky_deep, angle=0.02, coverage=2, seed=92})
blend(rect(0, 0, W, 170), {angle=0})
print(wait(0))

--@ chunk 25
w_far  = pile{{"lead white",6},{"yellow ochre",0.8},{"pale smalt",1.2},{"chrome yellow",0.15}, medium=0.15}
w_mid  = pile{{"lead white",4},{"pale smalt",2.2},{"raw umber",0.3},{"yellow ochre",0.25},{"cobalt blue",0.3}, medium=0.15}
w_near = pile{{"smalt",3},{"raw umber",1.4},{"Prussian blue",0.25},{"lead white",1.2},{"bone black",0.2}, medium=0.2}
-- far water band (bright, reflecting horizon sky)
work(rect(0, 484, W, 70), {hand="broad", tool={kind="flat", width=30}, pile=w_far, angle=0.0, coverage=3.5, fill=true, seed=101})
print(wait(0))
-- mid band
work(rect(0, 540, W, 110), {hand="broad", tool={kind="flat", width=30}, pile=w_mid, angle=0.0, coverage=3.5, fill=true, seed=102})
print(wait(0))
-- near band
work(rect(0, 630, W, 120), {hand="broad", tool={kind="flat", width=30}, pile=w_near, angle=0.0, coverage=3.5, fill=true, seed=103})
print(wait(0))

--@ chunk 26
blend(rect(0, 480, W, 270), {angle=0})
blend(rect(0, 480, W, 270), {angle=0.01})
print(wait(0))

--@ chunk 27
print(wait(2*24*60))
print("top", drying(500,30), "cloud", drying(800,200), "glow", drying(300,450), "water", drying(500,560), "near", drying(500,700))

--@ chunk 28
print(wait(4*24*60))
print("top", drying(500,30), "cloud", drying(800,200), "glow", drying(300,450), "water", drying(500,560), "near", drying(500,700))

--@ chunk 29
w_glow  = pile{{"lead white",6},{"chrome yellow",0.45},{"yellow ochre",0.5},{"pale smalt",0.4}, medium=0.12}
w_lilac = pile{{"lead white",4},{"pale smalt",2},{"red earth",0.35},{"raw umber",0.45}, medium=0.12}
w_mid   = pile{{"lead white",2.4},{"smalt",2.2},{"raw umber",0.8},{"green earth",0.6},{"Prussian blue",0.1}, medium=0.15}
w_dark  = pile{{"smalt",2},{"Prussian blue",0.4},{"raw umber",1.5},{"green earth",0.5},{"bone black",0.3}, medium=0.18}

local function hband(y0, y1, p, cov, sd, x0, x1, wd)
  work(rect(x0 or -10, y0, (x1 or 1010) - (x0 or -10), y1 - y0), {hand="broad", tool={kind="flat", width=wd or 16}, pile=p, angle=0.0,
       length={90, 260}, coverage=cov, fill=true, angle_jitter=0.02, seed=sd})
end
-- horizon water: warm under the glow, lilac at right
hband(496, 540, w_glow, 3.5, 201, -10, 560, 14)
hband(496, 540, w_lilac, 3.5, 202, 480, 1010, 14)
print(wait(0))
-- mid water
hband(535, 610, w_mid, 3.5, 203, -10, 1010, 16)
print(wait(0))
-- dark water
hband(600, 745, w_dark, 3.5, 204, -10, 1010, 18)
print(wait(0))

--@ chunk 30
blend(rect(0, 490, W, 260), {angle=0})
print(wait(0))
print(drying(500,700))

--@ chunk 31
local path_top = poly({{262,497},{338,497},{372,560},{228,560}}, true)
local path_mid = poly({{228,552},{372,552},{430,650},{176,650}}, true)
local path_low = poly({{176,640},{430,640},{520,745},{96,745}}, true)
w_path_lo = pile{{"lead white",4},{"pale smalt",1.5},{"chrome yellow",0.35},{"yellow ochre",0.4}, medium=0.1}
work(path_top, {hand="body", tool={kind="flat", width=8}, pile=sun_hi, length={25,80}, angle=0.0, angle_jitter=0.03, coverage=2.5, fill=true, edge="lost", seed=301})
work(path_mid, {hand="body", tool={kind="flat", width=8}, pile=w_glow, length={25,90}, angle=0.0, angle_jitter=0.03, coverage=2.2, fill=true, edge="lost", seed=302})
work(path_low, {hand="body", tool={kind="flat", width=7}, pile=w_path_lo, length={20,80}, angle=0.0, angle_jitter=0.03, coverage=1.6, fill=false, edge="lost", seed=303})
print(wait(0))
blend(rect(60, 490, 520, 260), {angle=0, clip=false})
print(wait(0))

--@ chunk 32
glaze_warm = pile{{"chrome yellow",1},{"vermilion",0.22},{"lead white",0.6}, medium=0.6}
glaze_hot  = pile{{"chrome yellow",1},{"vermilion",0.08},{"lead white",1.2}, medium=0.55}
-- broad warm halo, then a tighter, hotter one
work(ellipse(300, 462, 300, 92), {hand="glaze", pile=glaze_warm, angle=0.0, coverage=1.3, edge="lost", seed=401})
work(ellipse(300, 466, 190, 58), {hand="glaze", pile=glaze_hot, angle=0.0, coverage=1.4, edge="lost", seed=402})
print(wait(0))

--@ chunk 33
blend(ellipse(300, 480, 330, 130), {angle=0, clip=false})
blend(ellipse(300, 480, 330, 130), {angle=0.02, clip=false})
print(wait(0))

--@ chunk 34
print(wait(3*24*60))
print("glow", drying(300,460), "cloud", drying(800,200), "water", drying(500,700), "path", drying(300,650))

--@ chunk 35
g_outer = pile{{"lead white",6},{"yellow ochre",0.6},{"pale smalt",0.7},{"red earth",0.12}, medium=0.1}
g_mid   = pile{{"lead white",6},{"yellow ochre",0.45},{"chrome yellow",0.2},{"vermilion",0.1}, medium=0.1}
g_in    = pile{{"lead white",8},{"chrome yellow",0.3},{"vermilion",0.05}, medium=0.08}
g_core  = pile{{"lead white",10},{"chrome yellow",0.1}, medium=0.06}
local sky_only = rect(0, 0, W, 500)
local L1 = ellipse(300, 440, 380, 140) * sky_only
local L2 = ellipse(300, 455, 270, 92) * sky_only
local L3 = ellipse(300, 462, 160, 54) * sky_only
work(L1, {hand="body", tool={kind="flat", width=16}, pile=g_outer, length={40,140}, angle=0.0, angle_jitter=0.03, coverage=4, fill=true, edge="lost", seed=501})
print(wait(0))
blend(L1:grow(20), {angle=0, clip=false})
work(L2, {hand="body", tool={kind="flat", width=14}, pile=g_mid, length={40,120}, angle=0.0, angle_jitter=0.03, coverage=4, fill=true, edge="lost", seed=502})
print(wait(0))
blend(L2:grow(16), {angle=0, clip=false})
work(L3, {hand="body", tool={kind="flat", width=12}, pile=g_in, length={30,100}, angle=0.0, angle_jitter=0.03, coverage=3.5, fill=true, edge="lost", seed=503})
print(wait(0))
blend(L3:grow(12), {angle=0, clip=false})
print(wait(0))

--@ chunk 36
sky_mid2 = pile{{"lead white",4},{"pale smalt",2.4},{"yellow ochre",0.2},{"cobalt blue",0.35}, medium=0.15}
sky_peach = pile{{"lead white",5},{"pale smalt",0.8},{"vermilion",0.12},{"yellow ochre",0.5}, medium=0.15}
-- bring the pale blue-grey back down over the cream blob's upper part
work(rect(-10, 240, 700, 120), {hand="broad", tool={kind="flat", width=34}, pile=sky_mid2, length={100,260}, angle=0.02, coverage=3.2, fill=true, seed=601})
work(rect(-10, 340, 700, 70), {hand="broad", tool={kind="flat", width=30}, pile=sky_peach, length={100,260}, angle=0.02, coverage=2.2, fill=true, seed=602})
print(wait(0))
blend(rect(-10, 230, 720, 260), {angle=0})
blend(rect(-10, 230, 720, 260), {angle=0.02})
print(wait(0))

--@ chunk 37
local bands = {
 {150, 250, pile{{"pale smalt",3},{"lead white",4},{"yellow ochre",0.3},{"cobalt blue",0.5}, medium=0.15}},
 {235, 320, pile{{"lead white",5},{"pale smalt",2.2},{"yellow ochre",0.3}, medium=0.15}},
 {305, 385, pile{{"lead white",5.5},{"pale smalt",1.2},{"yellow ochre",0.55},{"red earth",0.1}, medium=0.15}},
 {370, 440, pile{{"lead white",5},{"yellow ochre",0.6},{"vermilion",0.1},{"chrome yellow",0.15}, medium=0.15}},
 {425, 505, pile{{"lead white",6},{"chrome yellow",0.35},{"vermilion",0.08}, medium=0.12}},
}
for i, b in ipairs(bands) do
  work(rect(-10, b[1], 1020, b[2]-b[1]), {hand="broad", tool={kind="flat", width=30}, pile=b[3], length={120,300}, angle=0.02, angle_jitter=0.02,
       coverage=3, fill=true, edge="soft", seed=700+i})
end
print(wait(0))
-- cooler, lilac right side toward the horizon
local rb = {
 {370, 440, pile{{"lead white",5},{"pale smalt",1.5},{"red earth",0.35},{"yellow ochre",0.2}, medium=0.15}},
 {425, 505, pile{{"lead white",5},{"pale smalt",2},{"red earth",0.3},{"yellow ochre",0.25}, medium=0.15}},
}
for i, b in ipairs(rb) do
  work(rect(520, b[1], 500, b[2]-b[1]), {hand="broad", tool={kind="flat", width=30}, pile=b[3], length={120,300}, angle=0.02, angle_jitter=0.02,
       coverage=3, fill=true, edge="soft", load_at=function(x,y) return clamp((x-540)/200, 0.05, 1) end, seed=720+i})
end
print(wait(0))
blend(rect(-10, 140, 1020, 380), {angle=math.pi/2})
print(wait(0))
blend(rect(-10, 140, 1020, 380), {angle=0})
print(wait(0))

--@ chunk 38
sky_top2 = pile{{"cobalt blue",1.6},{"smalt",2.2},{"lead white",2.6},{"raw umber",0.15}, medium=0.2}
work(rect(-10, 90, 1020, 200), {hand="broad", tool={kind="flat", width=34}, pile=sky_top2, length={120,300}, angle=0.02, angle_jitter=0.02,
     coverage=3, fill=true, edge="soft", load_at=function(x,y) return clamp(1.15-(y-110)/170, 0.03, 1) end, seed=801})
print(wait(0))
blend(rect(-10, 60, 1020, 300), {angle=math.pi/2})
blend(rect(-10, 60, 1020, 300), {angle=0.02})
print(wait(0))

--@ chunk 39
-- seam at y=360: blend vertically through it with a soft-edged mask
blend(rect(-10, 290, 1020, 150):blur(50), {angle=math.pi/2})
blend(rect(-10, 290, 1020, 150):blur(50), {angle=0.02})
print(wait(0))
-- seam at y=60: veil the dry top with the same blue, feathered out downward
work(rect(-10, 0, 1020, 120), {hand="broad", tool={kind="flat", width=34}, pile=sky_top2, length={120,300}, angle=0.02, angle_jitter=0.02,
     coverage=2.5, fill=true, edge="soft", load_at=function(x,y) return clamp((125-y)/90, 0.05, 1) end, seed=811})
print(wait(0))
blend(rect(-10, 0, 1020, 200):blur(50), {angle=math.pi/2})
blend(rect(-10, 0, 1020, 200):blur(50), {angle=0.03})
print(wait(0))

--@ chunk 40
sky_deep2 = pile{{"smalt",3},{"cobalt blue",1.2},{"lead white",1.6},{"Prussian blue",0.12},{"raw umber",0.3}, medium=0.2}
work(rect(-10, -10, 1020, 220), {hand="broad", tool={kind="flat", width=34}, pile=sky_deep2, length={120,300}, angle=0.02, angle_jitter=0.02,
     coverage=3, fill=true, edge="soft", load_at=function(x,y) return clamp(1.05-(y)/190, 0.03, 1) end, seed=821})
print(wait(0))
blend(rect(-30, -30, 1060, 300):blur(70), {angle=math.pi/2})
blend(rect(-30, -30, 1060, 300):blur(70), {angle=0.02})
print(wait(0))

--@ chunk 41
print(drying(500,40), drying(500,250), drying(500,450), drying(500,500), drying(500,600))
print(wait(3*24*60))
print(drying(500,40), drying(500,250), drying(500,450), drying(500,500), drying(500,600))

--@ chunk 42
-- cumulus tower on the right: lumps placed by hand {x, y, z, rx, ry, rz}
local L = {
 -- base tier
 {640,352,0, 88,40,60}, {775,358,10, 96,42,60}, {915,352,10, 100,44,60}, {1010,350,0, 70,44,50},
 -- second tier
 {700,296,30, 76,52,60}, {820,300,30, 84,54,62}, {945,292,20, 82,56,58},
 -- third tier
 {770,232,40, 74,56,60}, {885,236,45, 78,58,60}, {985,226,20, 60,60,50},
 -- fourth tier
 {835,168,50, 66,54,56}, {940,160,50, 64,58,54}, {760,178,30, 44,40,40},
 -- crown
 {880,104,50, 56,46,48}, {800,112,30, 36,34,34}, {965,96,40, 44,44,40},
}
local s
for i, l in ipairs(L) do
  local e = body.ellipsoid({l[1], l[2], l[3]}, {l[4], l[5], l[6]}):rough(9, 45, 10 + i)
  s = s and s:union(e) or e
end
cum = form{ {s}, light={from={-1, 0.3}, front=0.3, ambient=0.2} }
print("sil area", cum:silhouette{}:area())
print("lit area", cum:lit{soft=0.1}:area())
for _, p in ipairs({{640,352},{700,296},{775,240},{830,168},{880,104},{960,300},{900,60},{300,300},{620,330}}) do
  print(p[1], p[2], string.format("%.2f", cum:value(p[1], p[2])), string.format("%.2f", cum:silhouette{}:at(p[1], p[2])))
end

--@ chunk 43
cumsil = cum:silhouette{}
local vals = {}
for y = 40, 400, 6 do
  for x = 560, 1000, 6 do
    if cumsil:at(x, y) > 0.5 then vals[#vals+1] = cum:value(x, y) end
  end
end
table.sort(vals)
local n = #vals
print("n", n)
for _, q in ipairs({0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 0.97, 1}) do
  print(q, string.format("%.3f", vals[math.max(1, math.min(n, math.floor(q * n)))]))
end

--@ chunk 44
fall = cum:field("fall")
across = cum:field("across")
cs_shadow = pile{{"lead white",2},{"smalt",2.6},{"red earth",0.8},{"raw umber",0.9},{"cobalt blue",0.3},{"Prussian blue",0.08}, medium=0.1}
cs_mid    = pile{{"lead white",3.6},{"smalt",1.2},{"red earth",0.55},{"yellow ochre",0.35},{"raw umber",0.2}, medium=0.1}
cs_light  = pile{{"lead white",6},{"yellow ochre",0.45},{"chrome yellow",0.25},{"vermilion",0.2}, medium=0.08}
cs_high   = pile{{"lead white",9},{"chrome yellow",0.3},{"vermilion",0.06}, medium=0.05}
work(cumsil, {hand="body", tool={kind="filbert", width=16}, pile=cs_shadow, angle=across, length={30,80}, coverage=2.6, fill=true, edge="soft", seed=901})
print(wait(0))

--@ chunk 45
local function band(lo, hi, soft)
  return mask(function(x, y)
    if x < 560 or y > 400 or y < 30 then return 0 end
    if cumsil:at(x, y) < 0.5 then return 0 end
    local v = cum:value(x, y)
    local a = clamp((v - (lo - soft)) / (2 * soft), 0, 1)
    local b = 1 - clamp((v - (hi - soft)) / (2 * soft), 0, 1)
    return a * b
  end)
end
bd_mid   = band(0.27, 0.50, 0.05)
bd_light = band(0.45, 0.70, 0.05)
bd_high  = band(0.64, 1.2, 0.04)
print(bd_mid:area(), bd_light:area(), bd_high:area())
work(bd_mid, {hand="body", tool={kind="filbert", width=13}, pile=cs_mid, angle=across, length={20,60}, coverage=2.2, fill=true, edge="soft", threshold=0.4, seed=911})
print(wait(0))

--@ chunk 46
work(bd_light, {hand="body", tool={kind="filbert", width=11}, pile=cs_light, angle=across, length={16,46}, coverage=2.0, fill=true, edge="soft", threshold=0.4, seed=921})
print(wait(0))
work(bd_high, {hand="body", tool={kind="filbert", width=8}, pile=cs_high, angle=across, length={10,32}, coverage=1.8, fill=true, edge="soft", threshold=0.4, seed=931})
print(wait(0))

--@ chunk 47
print(drying(700,300), drying(880,120), drying(880,250), drying(650,380))
print(wait(0))

--@ chunk 48
blend(cumsil, {angle=across})
print(wait(0))

--@ chunk 49
function band(lo, hi, soft)
  return mask(function(x, y)
    if x < 560 or y > 400 or y < 30 then return 0 end
    if cumsil:at(x, y) < 0.5 then return 0 end
    local v = cum:value(x, y)
    local a = clamp((v - (lo - soft)) / (2 * soft), 0, 1)
    local b = 1 - clamp((v - (hi - soft)) / (2 * soft), 0, 1)
    return a * b
  end)
end
bd_deep = band(-0.1, 0.24, 0.04)
print(bd_deep:area())
cs_shadow2 = pile{{"lead white",1.6},{"smalt",2.6},{"red earth",0.9},{"raw umber",1.0},{"cobalt blue",0.3},{"Prussian blue",0.1}, medium=0.1}
work(bd_deep, {hand="body", tool={kind="filbert", width=14}, pile=cs_shadow2, angle=across, length={24,70}, coverage=1.8, fill=true, edge="soft", threshold=0.4, seed=941})
print(wait(0))
blend(bd_deep:grow(6), {angle=across})
print(wait(0))

--@ chunk 50
print(wait(6*24*60))
print(drying(700,300), drying(880,120), drying(880,250), drying(650,380))

--@ chunk 51
bd_mid2   = band(0.24, 0.50, 0.05)
bd_light2 = band(0.44, 0.70, 0.05)
bd_high2  = band(0.62, 1.2, 0.04)
-- mid-tones over the dry dark: fill the body of the lit side
work(bd_mid2, {hand="body", tool={kind="filbert", width=14}, pile=cs_mid, angle=across, length={20,60}, coverage=2.4, fill=true, edge="soft", threshold=0.4, seed=1011})
print(wait(0))
work(bd_light2, {hand="body", tool={kind="filbert", width=11}, pile=cs_light, angle=across, length={16,46}, coverage=2.2, fill=true, edge="soft", threshold=0.4, seed=1021})
print(wait(0))
work(bd_high2, {hand="body", tool={kind="filbert", width=8}, pile=cs_high, angle=across, length={10,32}, coverage=2.0, fill=true, edge="soft", threshold=0.4, seed=1031})
print(wait(0))

--@ chunk 52
ap_a = pile{{"lead white",5},{"chrome yellow",0.7},{"vermilion",0.3},{"yellow ochre",0.3}, medium=0.15}
ap_b = pile{{"lead white",4},{"vermilion",0.55},{"red earth",0.2},{"chrome yellow",0.35},{"pale smalt",0.3}, medium=0.15}
local function away_from_sun(x, y)
  local d = math.sqrt(((x-300)/1.7)^2 + (y-470)^2)
  return clamp((d - 70) / 200, 0.04, 1)
end
-- warm apricot skirt of the lower sky, held back from the sun itself
work(rect(-10, 330, 1020, 170), {hand="broad", tool={kind="flat", width=30}, pile=ap_a, length={120,300}, angle=0.02, angle_jitter=0.02,
     coverage=3, fill=true, edge="soft", load_at=function(x,y) return away_from_sun(x,y) * clamp((y-330)/70, 0.05, 1) end, seed=1101})
print(wait(0))
work(rect(-10, 400, 1020, 100), {hand="broad", tool={kind="flat", width=26}, pile=ap_b, length={120,300}, angle=0.02, angle_jitter=0.02,
     coverage=2.5, fill=true, edge="soft", load_at=function(x,y) return away_from_sun(x,y) * clamp((y-400)/60, 0.05, 0.9) end, seed=1102})
print(wait(0))
blend(rect(-10, 320, 1020, 190):blur(20), {angle=math.pi/2})
blend(rect(-10, 320, 1020, 190):blur(20), {angle=0.02})
print(wait(0))

--@ chunk 53
st_dark = pile{{"lead white",2},{"smalt",3},{"red earth",1.0},{"raw umber",1.0},{"cobalt blue",0.3}, medium=0.1}
st_mid  = pile{{"lead white",3.2},{"smalt",2},{"red earth",1.1},{"raw umber",0.5},{"vermilion",0.15}, medium=0.1}
local function streak(name, pts, seed, lobe, pile_, wd, cov)
  local o = outline{pts=pts, closed=true, char="soft", lobe=lobe or 28, amount=0.7, seed=seed}
  local m = o:mask()
  work(m, {hand="body", tool={kind="flat", width=wd or 9}, pile=pile_, length={50,150}, angle=0.0, angle_jitter=0.03, coverage=cov or 2.6, fill=true, edge="soft", seed=seed*3})
  return m
end
sL1 = streak("L1", {{-20,398},{60,393},{140,398},{220,407},{290,416},{345,424},{335,431},{270,429},{200,423},{120,417},{50,413},{-20,415}}, 21, 30, st_dark, 10, 2.8)
print(wait(0))
sL2 = streak("L2", {{-20,440},{50,437},{120,440},{200,446},{270,451},{350,458},{420,463},{410,469},{340,466},{260,461},{190,457},{110,453},{40,452},{-20,454}}, 22, 26, st_dark, 8, 2.6)
print(wait(0))
sL3 = streak("L3", {{-20,470},{70,468},{150,472},{230,476},{300,480},{290,486},{220,484},{140,480},{60,478},{-20,479}}, 23, 22, st_mid, 6, 2.6)
print(wait(0))
sR1 = streak("R1", {{430,466},{500,458},{580,450},{680,444},{780,440},{880,436},{1020,432},{1020,472},{900,474},{780,472},{680,474},{580,476},{500,476},{430,473}}, 24, 40, st_dark, 12, 2.8)
print(wait(0))

--@ chunk 54
local all_st = sL1 + sL2 + sL3 + sR1
blend(all_st:grow(8), {angle=0.0, clip=false})
print(wait(0))
blend(all_st:grow(5), {angle=0.03, clip=false})
print(wait(0))

--@ chunk 55
print(wait(3*24*60))
print(drying(200,470), drying(600,480), drying(850,250), drying(300,600))

--@ chunk 56
-- helper: horizontal flat-brush laydown over a mask
function hb(m, p, wd, sd, cov, len, extra)
  local o = {hand="broad", tool={kind="flat", width=wd}, pile=p, length=len or {80,240}, angle=0.0, angle_jitter=0.025,
             coverage=cov or 3, fill=true, edge="soft", seed=sd}
  if extra then for k, v in pairs(extra) do o[k] = v end end
  work(m, o)
end
-- opaque restoring coat over the smeared left glow (paint below is dry)
gc_upper = pile{{"lead white",6},{"yellow ochre",0.55},{"pale smalt",0.55},{"red earth",0.08}, medium=0.08}
gc_apri  = pile{{"lead white",5.5},{"vermilion",0.3},{"yellow ochre",0.45},{"chrome yellow",0.25}, medium=0.08}
gc_mid   = pile{{"lead white",6.5},{"chrome yellow",0.5},{"yellow ochre",0.25},{"vermilion",0.15}, medium=0.08}
gc_core  = pile{{"lead white",8.5},{"chrome yellow",0.55},{"vermilion",0.07}, medium=0.06}
local function fadetop(y0, y1) return function(x, y) return clamp((y - y0) / (y1 - y0), 0.05, 1) end end
local function fader(x0, x1) return function(x, y) return clamp((x1 - x) / (x1 - x0), 0.05, 1) end end
local reg = rect(-10, 330, 720, 200)
hb(reg, gc_upper, 26, 1201, 3, nil, {load_at=function(x, y) return clamp((y-330)/50, 0.05, 1) * clamp((730-x)/120, 0.05, 1) end})
print(wait(0))
hb(rect(-10, 400, 700, 130), gc_apri, 22, 1202, 3, nil, {load_at=function(x, y) return clamp((y-400)/50, 0.05, 1) * clamp((700-x)/150, 0.05, 1) end})
print(wait(0))
hb(rect(-10, 440, 620, 90), gc_mid, 20, 1203, 3, nil, {load_at=function(x, y) return clamp((y-440)/30, 0.05, 1) * clamp((640-x)/240, 0.05, 1) end})
print(wait(0))

--@ chunk 57
-- the slab is still wet: dissolve its top and right edges into the neighbouring sky
blend(rect(-10, 300, 730, 110):blur(30), {angle=math.pi/2})
print(wait(0))
blend(rect(560, 330, 330, 210):blur(30), {angle=0.0, clip=false})
print(wait(0))
blend(rect(-10, 300, 900, 250):blur(40), {angle=0.02})
print(wait(0))

--@ chunk 58
st_rose = pile{{"lead white",3},{"vermilion",0.55},{"red earth",0.5},{"smalt",0.7},{"yellow ochre",0.2}, medium=0.1}
st_dusk = pile{{"lead white",1.6},{"smalt",3},{"raw umber",1.1},{"red earth",0.8},{"cobalt blue",0.3},{"bone black",0.1}, medium=0.1}
-- Bar A: long low bar at left, thick at the left end and tapering toward the sun
local oa = outline{{-20,452},{40,446},{110,440},{180,444},{250,452},{330,458},{400,462},{470,467},{440,471},{370,472},{290,472},{220,474},{140,480},{70,484},{-20,486}, closed=true, char="soft", lobe=34, amount=0.6, seed=31}
barA = oa:mask()
hb(barA, st_dusk, 9, 1301, 2.6, {60,190})
print(wait(0))
local ob2 = outline{{-20,478},{60,475},{150,476},{240,474},{330,471},{420,470},{380,474},{300,477},{220,481},{140,485},{60,489},{-20,491}, closed=true, char="soft", lobe=26, amount=0.6, seed=32}
barA2 = ob2:mask()
hb(barA2, st_rose, 7, 1302, 2.4, {50,150})
print(wait(0))
blend(barA:grow(4) + barA2:grow(4), {angle=0.0, clip=false})
print(wait(0))

--@ chunk 59
print(wait(3*24*60))
print(drying(150,470), drying(300,500), drying(600,450), drying(850,450))

--@ chunk 60
st_dusk2 = pile{{"cobalt blue",1.6},{"raw umber",1.2},{"red earth",0.7},{"lead white",1.3},{"Prussian blue",0.12}, medium=0.06}
st_rose2 = pile{{"lead white",2.4},{"vermilion",0.7},{"red earth",0.5},{"cobalt blue",0.25},{"yellow ochre",0.3}, medium=0.06}
-- Bar A again on the dry slab: irregular lumpy top, flat warm underside
local oa = outline{{-20,458},{20,450},{60,441},{100,446},{140,436},{190,442},{230,449},{280,452},{330,457},{380,460},{430,464},{475,468},
  {430,473},{370,474},{300,475},{230,477},{160,481},{90,485},{20,487},{-20,488}, closed=true, char="soft", lobe=30, amount=0.9, seed=41}
barA = oa:mask()
work(barA, {hand="body", tool={kind="filbert", width=10}, pile=st_dusk2, length={40,120}, angle=0.0, angle_jitter=0.04, coverage=3, fill=true, edge="soft", seed=1401})
print(wait(0))
local ou = outline{{-20,482},{60,480},{150,478},{240,476},{330,473},{430,471},{380,477},{300,481},{220,484},{140,488},{60,491},{-20,493}, closed=true, char="soft", lobe=24, amount=0.7, seed=42}
barU = ou:mask()
work(barU, {hand="body", tool={kind="filbert", width=8}, pile=st_rose2, length={40,110}, angle=0.0, angle_jitter=0.04, coverage=2.6, fill=true, edge="soft", seed=1402})
print(wait(0))
blend((barA + barU):grow(3), {angle=0.0, clip=false})
print(wait(0))

--@ chunk 61
blend((barA + barU):grow(2), {angle=math.pi/2, clip=false})
print(wait(0))
blend((barA + barU):grow(4), {angle=0.0, clip=false})
print(wait(0))

--@ chunk 62
print(wait(2*24*60))
print(drying(50,470), drying(120,420), drying(300,520), drying(280,480))

--@ chunk 63
print(wait(2*24*60))
print(drying(50,470), drying(120,420), drying(300,520), drying(280,480))

--@ chunk 64
b1 = pile{{"lead white",5},{"pale smalt",1.6},{"yellow ochre",0.4},{"red earth",0.1}, medium=0.08}
b2 = pile{{"lead white",5.5},{"yellow ochre",0.45},{"vermilion",0.14},{"pale smalt",0.5}, medium=0.08}
b3 = pile{{"lead white",4.5},{"vermilion",0.5},{"chrome yellow",0.45},{"yellow ochre",0.3}, medium=0.08}
b4 = pile{{"lead white",4},{"chrome yellow",0.75},{"vermilion",0.5}, medium=0.08}
r3 = pile{{"lead white",4.5},{"vermilion",0.42},{"red earth",0.22},{"pale smalt",0.9}, medium=0.08}
r4 = pile{{"lead white",4.5},{"vermilion",0.6},{"chrome yellow",0.3},{"pale smalt",0.5}, medium=0.08}
local function rgt(x) return clamp((x - 480) / 220, 0, 1) end   -- 0 at left of the glow, 1 in the cool right
local bandmask = function(y0, y1) return rect(-10, y0, 1020, y1 - y0) end
-- left/centre laydown
hb(bandmask(340, 400), b1, 28, 1501, 2.6, {100,260}, {load_at=function(x,y) return (1 - 0.7*rgt(x)) * (x > 520 and y < 395 and 0.0 or 1) + 0.0 end})
hb(bandmask(385, 440), b2, 26, 1502, 2.6, {100,260})
print(wait(0))
hb(bandmask(425, 480), b3, 24, 1503, 2.6, {100,260}, {load_at=function(x,y) return 1 - 0.85*rgt(x) end})
hb(rect(470, 425, 550, 55), r3, 24, 1504, 2.6, {100,260}, {load_at=function(x,y) return rgt(x) end})
print(wait(0))
hb(bandmask(462, 500), b4, 20, 1505, 2.6, {100,260}, {load_at=function(x,y) return 1 - 0.8*rgt(x) end})
hb(rect(470, 462, 550, 38), r4, 20, 1506, 2.6, {100,260}, {load_at=function(x,y) return rgt(x) end})
print(wait(0))

--@ chunk 65
local bm = rect(-10, 330, 1020, 175):blur(24)
blend(bm, {angle=math.pi/2})
print(wait(0))
blend(bm, {angle=0.0})
print(wait(0))

--@ chunk 66
wb0 = pile{{"lead white",6},{"vermilion",0.22},{"yellow ochre",0.4},{"chrome yellow",0.2},{"pale smalt",0.3}, medium=0.08}
wb1 = pile{{"lead white",4.5},{"pale smalt",1.5},{"vermilion",0.3},{"red earth",0.25},{"yellow ochre",0.2}, medium=0.08}
wb2 = pile{{"lead white",3},{"smalt",2.2},{"raw umber",0.5},{"red earth",0.2},{"cobalt blue",0.5}, medium=0.1}
wb3 = pile{{"lead white",1.8},{"smalt",3},{"Prussian blue",0.25},{"raw umber",0.9},{"cobalt blue",0.5}, medium=0.1}
wb4 = pile{{"smalt",3},{"Prussian blue",0.4},{"raw umber",1.4},{"bone black",0.3},{"lead white",1}, medium=0.12}
local function wband(y0, y1) return rect(-10, y0, 1020, y1 - y0) end
hb(wband(494, 528), wb0, 14, 1601, 3.0, {90,240})
hb(wband(516, 575), wb1, 16, 1602, 3.0, {90,240})
print(wait(0))
hb(wband(560, 625), wb2, 18, 1603, 3.0, {90,240})
hb(wband(610, 685), wb3, 20, 1604, 3.0, {90,240})
print(wait(0))
hb(wband(668, 745), wb4, 22, 1605, 3.0, {90,240})
print(wait(0))
local wm = rect(-10, 490, 1020, 260):blur(16)
blend(wm, {angle=math.pi/2})
print(wait(0))
blend(wm, {angle=0.0})
print(wait(0))

--@ chunk 67
local wm2 = rect(-10, 494, 1020, 255):blur(10)
blend(wm2, {angle=0.0})
print(wait(0))
blend(wm2, {angle=0.015})
print(wait(0))
blend(wm2, {angle=-0.01})
print(wait(0))

--@ chunk 68
pth_hi = pile{{"lead white",9},{"chrome yellow",0.35},{"vermilion",0.05}, medium=0.08}
pth_lo = pile{{"lead white",6},{"yellow ochre",0.4},{"vermilion",0.2},{"pale smalt",0.5}, medium=0.08}
local op = outline{{285,494},{320,494},{338,540},{372,590},{420,650},{490,745},{100,745},{190,650},{240,590},{268,540}, closed=true, char="soft", lobe=30, amount=0.5, seed=77}
lpath = op:mask()
local function hw(y) return 34 + (y - 496) * 0.62 end
hb(lpath, pth_hi, 12, 1701, 3.0, {40,150}, {edge="lost", load_at=function(x, y)
      local d = math.abs(x - 300) / hw(y)
      return clamp(1.1 - d, 0.0, 1) * clamp(1.15 - (y - 496) / 210, 0.15, 1) end})
print(wait(0))
hb(rect(60, 570, 480, 175), pth_lo, 14, 1702, 3.0, {40,150}, {edge="lost", load_at=function(x, y)
      local d = math.abs(x - 300) / hw(y)
      return clamp(1.05 - d, 0.0, 1) * clamp((y - 560) / 90, 0.1, 1) * clamp(1.2 - (y - 570) / 250, 0.3, 1) end})
print(wait(0))
blend(lpath:grow(30):blur(20), {angle=0.0})
print(wait(0))

--@ chunk 69
print(wait(4*24*60))
print(drying(300,600), drying(700,650), drying(500,400))

--@ chunk 70
-- Sun halo (warm, broad), then bright inner glow, then the disc. Painted opaque-ish over the dry sky, then fused.
hal1 = pile{{"lead white",6},{"chrome yellow",0.7},{"vermilion",0.12},{"yellow ochre",0.2}, medium=0.12}
hal2 = pile{{"lead white",8},{"chrome yellow",0.55},{"vermilion",0.04}, medium=0.1}
hal3 = pile{{"lead white",10},{"chrome yellow",0.22}, medium=0.06}
local sky_only = rect(-10, 0, 1020, 496)
local H1 = ellipse(300, 462, 250, 100) * sky_only
local H2 = ellipse(300, 466, 150, 60) * sky_only
local H3 = ellipse(300, 468, 70, 30) * sky_only
hb(H1, hal1, 20, 1801, 3.2, {60,180}, {edge="lost", load_at=function(x,y) local d = math.sqrt(((x-300)/250)^2 + ((y-462)/100)^2); return clamp(1.15 - d, 0.05, 1) end})
print(wait(0))
blend(H1:grow(20), {angle=0.0, clip=false})
hb(H2, hal2, 16, 1802, 3.2, {50,150}, {edge="lost", load_at=function(x,y) local d = math.sqrt(((x-300)/150)^2 + ((y-466)/60)^2); return clamp(1.15 - d, 0.05, 1) end})
print(wait(0))
blend(H2:grow(14), {angle=0.0, clip=false})
hb(H3, hal3, 12, 1803, 3.2, {30,90}, {edge="lost", load_at=function(x,y) local d = math.sqrt(((x-300)/70)^2 + ((y-468)/30)^2); return clamp(1.2 - d, 0.05, 1) end})
print(wait(0))
blend(H3:grow(10), {angle=0.0, clip=false})
print(wait(0))

--@ chunk 71
bk_dark = pile{{"smalt",3},{"raw umber",1.3},{"red earth",1.0},{"lead white",0.9},{"cobalt blue",0.4},{"Prussian blue",0.1},{"bone black",0.15}, medium=0.08}
bk_mid  = pile{{"lead white",2.4},{"red earth",1.1},{"smalt",1.5},{"raw umber",0.5},{"vermilion",0.25}, medium=0.08}
bk_glow = pile{{"lead white",3},{"vermilion",0.7},{"chrome yellow",0.6},{"red earth",0.3}, medium=0.08}
bk_rim  = pile{{"lead white",6},{"chrome yellow",0.6},{"vermilion",0.3},{"yellow ochre",0.3}, medium=0.06}
local top = {{-30,414},{40,406},{110,394},{180,400},{240,384},{300,392},{370,380},{440,384},{510,368},{580,362},{650,352},{760,346},{880,350},{1030,346}}
local bot = {{1030,486},{900,478},{800,475},{700,471},{620,467},{540,463},{470,459},{400,455},{340,453},{280,453},{220,456},{150,460},{80,463},{-30,467}}
local pts = {}
for _, p in ipairs(top) do pts[#pts+1] = p end
for _, p in ipairs(bot) do pts[#pts+1] = p end
bank_o = outline{pts=pts, closed=true, char="soft", lobe=44, amount=0.7, seed=91}
bankM = bank_o:mask()
print(bankM:area())
work(bankM, {hand="body", tool={kind="filbert", width=14}, pile=bk_dark, length={40,120}, angle=0.0, angle_jitter=0.05, coverage=3.2, fill=true,
     edge={found=0.35, soft=0.45, lost=0.2, period=70, seed=4}, seed=1901})
print(wait(0))

--@ chunk 72
print(wait(5*24*60))
print(drying(300,420), drying(700,400), drying(300,470))

--@ chunk 73
local R = bankM:grow(10)
local Rl = R * rect(-10, 330, 540, 170)
local Rr = R * rect(500, 330, 520, 170)
-- left: sky glow colours, graded down toward the sun's halo
hb(Rl * rect(-10, 330, 560, 60), b1, 24, 2001, 3.0, {90,220})
hb(Rl * rect(-10, 375, 560, 55), gc_upper, 24, 2002, 3.0, {90,220})
hb(Rl * rect(-10, 415, 560, 60), hal1, 22, 2003, 3.0, {90,220}, {load_at=function(x,y) return 1 end})
hb(Rl * rect(-10, 445, 560, 55), hal2, 20, 2004, 3.0, {90,220})
print(wait(0))
-- right: peach and lilac under the cumulus
hb(Rr * rect(500, 330, 520, 60), b2, 24, 2005, 3.0, {90,220})
hb(Rr * rect(500, 375, 520, 60), b3, 22, 2006, 3.0, {90,220}, {load_at=function(x,y) return 1 - 0.5*clamp((x-520)/300, 0, 1) end})
hb(Rr * rect(500, 375, 520, 60), r3, 22, 2007, 3.0, {90,220}, {load_at=function(x,y) return clamp((x-520)/300, 0, 1) end})
hb(Rr * rect(500, 420, 520, 80), r4, 20, 2008, 3.0, {90,220})
print(wait(0))
local blm = R:grow(14):blur(12)
blend(blm, {angle=math.pi/2})
print(wait(0))
blend(blm, {angle=0.0})
print(wait(0))

--@ chunk 74
sh_far  = pile{{"lead white",2.2},{"smalt",2},{"red earth",0.8},{"raw umber",0.6},{"yellow ochre",0.2}, medium=0.06}
sh_head = pile{{"smalt",3},{"raw umber",1.5},{"green earth",1.0},{"Prussian blue",0.25},{"lead white",0.9},{"red earth",0.35}, medium=0.06}
-- far low shore at left: thin, hazy, bumpy
local fo = outline{{-20,489},{40,487},{95,488},{150,486},{210,487},{270,486},{340,487},{400,486},{470,487},{540,487},{610,485},
  {620,497},{540,498},{440,497},{340,497},{240,497},{140,498},{40,497},{-20,497}, closed=true, char="soft", lobe=26, amount=0.5, seed=101}
farM = fo:mask()
work(farM, {hand="body", tool={kind="filbert", width=6}, pile=sh_far, length={30,90}, angle=0.0, angle_jitter=0.03, coverage=3, fill=true, edge="soft", seed=2101})
print(wait(0))
-- right headland, rising to a tree crest
local ho = outline{{560,488},{620,483},{670,478},{705,477},{745,472},{775,462},{800,449},{826,441},{848,430},{872,418},{896,423},{920,431},{946,437},{975,445},{1030,451},
  {1030,505},{940,503},{860,501},{780,499},{700,498},{620,498},{560,497}, closed=true, char="searching", lobe=22, amount=0.55, seed=102}
headM = ho:mask()
work(headM, {hand="body", tool={kind="filbert", width=9}, pile=sh_head, length={20,60}, angle=0.0, angle_jitter=0.06, coverage=3.4, fill=true, edge="firm", seed=2102})
print(wait(0))

--@ chunk 75
blend(headM:shrink(2), {angle=0.0})
print(wait(0))
blend(farM:shrink(1), {angle=0.0})
print(wait(0))

--@ chunk 76
rf_dark = pile{{"smalt",3},{"raw umber",1.4},{"green earth",0.8},{"Prussian blue",0.3},{"lead white",1.6},{"red earth",0.3}, medium=0.15}
rf_far  = pile{{"lead white",2.4},{"smalt",2},{"red earth",0.8},{"raw umber",0.6},{"yellow ochre",0.2}, medium=0.15}
-- reflection of the headland: broken horizontal touches, deepest under the tree crest, ragged below
local ro = outline{{556,499},{640,499},{720,500},{790,500},{870,500},{950,500},{1030,500},
  {1030,540},{990,556},{950,572},{910,586},{880,594},{850,584},{820,570},{790,548},{750,536},{700,526},{650,520},{600,514},{556,510}, closed=true, char="searching", lobe=30, amount=0.6, seed=111}
refM = ro:mask()
work(refM, {hand="body", tool={kind="flat", width=5}, pile=rf_dark, length={30,120}, angle=0.0, angle_jitter=0.02, coverage=2.6, fill=false, edge="lost",
     load_at=function(x, y) return clamp(1.15 - (y - 500) / 85, 0.15, 1) end, seed=2111})
print(wait(0))
local rf2 = outline{{-20,499},{60,499},{200,499},{340,499},{480,499},{560,499},{560,508},{460,506},{340,506},{200,507},{60,507},{-20,507}, closed=true, char="soft", lobe=40, amount=0.5, seed=112}
work(rf2:mask(), {hand="body", tool={kind="flat", width=4}, pile=rf_far, length={30,120}, angle=0.0, angle_jitter=0.02, coverage=2, fill=false, edge="lost", seed=2112})
print(wait(0))
blend((refM + rf2:mask()):grow(4), {angle=0.0, clip=false})
print(wait(0))

--@ chunk 77
print(wait(4*24*60))
print(drying(300,490), drying(800,470), drying(800,540))

--@ chunk 78
-- repair: bring the sky glow down to a clean horizon over the smeared shore, and the warm water under it
local skyfix = rect(-10, 452, 700, 44)
local function fadeL(x) return clamp((640 - x) / 240, 0.0, 1) end   -- 1 on the left, 0 at x=640
hb(skyfix, hal1, 20, 2201, 3.4, {90,240}, {clip=true, load_at=function(x,y) return clamp((y-452)/24, 0.05, 1) * fadeL(x) end})
hb(skyfix, r4, 18, 2202, 3.2, {90,240}, {clip=true, load_at=function(x,y) return clamp((y-452)/24, 0.05, 1) * (1 - fadeL(x)) end})
hb(rect(120, 462, 360, 34), hal2, 14, 2203, 3.0, {60,200}, {clip=true, load_at=function(x,y) return clamp((y-462)/16, 0.05, 1) * clamp(1.2 - math.abs(x-300)/200, 0.0, 1) end})
print(wait(0))
blend(rect(-10, 440, 700, 58):blur(8), {angle=0.0})
print(wait(0))

--@ chunk 79
local wr = rect(-10, 495, 710, 85)
local function pathw(x, y) return clamp(1.15 - math.abs(x - 300) / (50 + (y - 496) * 0.5), 0.0, 1) end
hb(wr, wb0, 14, 2301, 3.4, {90,260}, {clip=true, load_at=function(x,y) return clamp((580 - y) / 40, 0.05, 1) end})
print(wait(0))
hb(rect(-10, 522, 710, 58), wb1, 16, 2302, 3.2, {90,260}, {load_at=function(x,y) return clamp((y-522)/22, 0.05, 1) * (1 - 0.85*pathw(x, y)) * clamp((590 - y) / 25, 0.1, 1) end})
print(wait(0))
hb(rect(200, 495, 200, 85), pth_hi, 10, 2303, 3.2, {40,140}, {clip=false, load_at=function(x,y) return pathw(x, y) * clamp((585 - y) / 30, 0.1, 1) end})
print(wait(0))
blend(rect(-10, 494, 710, 90):blur(10), {angle=0.0})
print(wait(0))

--@ chunk 80
print(wait(3*24*60))
print(drying(300,520), drying(100,480))

--@ chunk 81
-- cover the old headland and its reflection with sky and water colours, feathered at the seam
wr_r  = pile{{"lead white",4.5},{"vermilion",0.32},{"pale smalt",1.0},{"red earth",0.2},{"yellow ochre",0.2}, medium=0.08}
local zone_sky = rect(650, 400, 370, 100)
hb(zone_sky, r3, 22, 2401, 3.4, {90,240}, {load_at=function(x,y) return clamp((x-650)/70, 0.05, 1) * clamp((y-400)/25, 0.05, 1) * (y < 450 and 1 or 0) end})
hb(zone_sky, r4, 20, 2402, 3.4, {90,240}, {load_at=function(x,y) return clamp((x-650)/70, 0.05, 1) * clamp((y-435)/25, 0.05, 1) end})
print(wait(0))
hb(rect(650, 494, 370, 110), wr_r, 16, 2403, 3.4, {90,240}, {load_at=function(x,y) return clamp((x-650)/70, 0.05, 1) * clamp((606-y)/50, 0.05, 1) end})
print(wait(0))
blend(rect(640, 396, 380, 215):blur(12), {angle=0.0})
print(wait(0))

--@ chunk 82
-- Full-width-ish repaint of the horizon sky and the water, wet-in-wet, so the patchy pink zone dissolves into a continuous gradient
local function xr(x) return clamp((x - 480) / 220, 0, 1) end
local function lf(x) return clamp((x - 380) / 90, 0.0, 1) end       -- feather at the left end
local sx0, sw = 380, 640
-- sky, upper row
hb(rect(sx0, 388, sw, 62), gc_upper, 22, 2501, 3.4, {90,240}, {load_at=function(x,y) return (1 - xr(x)) * lf(x) * clamp((y-388)/26, 0.05, 1) end})
hb(rect(sx0, 388, sw, 62), r3, 22, 2502, 3.4, {90,240}, {load_at=function(x,y) return xr(x) * lf(x) * clamp((y-388)/26, 0.05, 1) end})
print(wait(0))
-- sky, lower row
hb(rect(sx0, 432, sw, 66), hal1, 20, 2503, 3.4, {90,240}, {load_at=function(x,y) return (1 - clamp((x-300)/340, 0, 1)) * lf(x) * clamp((y-432)/24, 0.05, 1) end})
hb(rect(sx0, 432, sw, 66), b3, 20, 2504, 3.4, {90,240}, {load_at=function(x,y) return clamp(1 - math.abs(x-540)/260, 0, 1) * lf(x) * clamp((y-432)/24, 0.05, 1) end})
hb(rect(sx0, 432, sw, 66), r4, 20, 2505, 3.4, {90,240}, {load_at=function(x,y) return clamp((x-480)/260, 0, 1) * lf(x) * clamp((y-432)/24, 0.05, 1) end})
print(wait(0))
blend(rect(sx0 - 20, 380, sw + 40, 122):blur(14), {angle=math.pi/2})
print(wait(0))
blend(rect(sx0 - 20, 380, sw + 40, 122):blur(14), {angle=0.0})
print(wait(0))

--@ chunk 83
function low_pile(t) return pile{{"lead white", lerp(6,4.5,t)}, {"chrome yellow", lerp(0.7,0.3,t)}, {"vermilion", lerp(0.12,0.55,t)}, {"yellow ochre", lerp(0.2,0.02,t)}, {"pale smalt", lerp(0.02,0.5,t)}, medium=0.08} end
function up_pile(t) return pile{{"lead white", lerp(6,4.6,t)}, {"yellow ochre", lerp(0.55,0.02,t)}, {"pale smalt", lerp(0.55,0.9,t)}, {"red earth", lerp(0.08,0.22,t)}, {"vermilion", lerp(0.02,0.4,t)}, medium=0.08} end
for i = 0, 7 do
  local cx = 300 + 90 * i
  local t = clamp((cx - 300) / 480, 0, 1)
  local x0 = (i == 0) and 270 or (cx - 52)
  local w = (i == 7) and 1030 - x0 or 106
  hb(rect(x0, 388, w, 62), up_pile(t), 22, 2600 + i, 3.0, {60,150}, {load_at=function(x,y) return clamp((y-388)/26, 0.05, 1) * clamp((456-y)/24, 0.1, 1) end})
  hb(rect(x0, 432, w, 66), low_pile(t), 20, 2620 + i, 3.0, {60,150}, {load_at=function(x,y) return clamp((y-432)/24, 0.05, 1) end})
end
print(wait(0))
local zz = rect(290, 378, 740, 124):blur(14)
blend(zz, {angle=0.0})
print(wait(0))
blend(zz, {angle=math.pi/2})
print(wait(0))
blend(zz, {angle=0.0})
print(wait(0))

--@ chunk 84
-- Repaint the horizon zone as one continuous coat: overlapping x-slices with triangular weights, so no seams.
function mixp(a, b, t, med)
  -- a, b: lists of {tube, parts}; linear interpolation of the parts by t
  local parts, names = {}, {}
  for _, e in ipairs(a) do parts[e[1]] = (parts[e[1]] or 0) + e[2] * (1 - t); names[#names+1] = e[1] end
  for _, e in ipairs(b) do
    if parts[e[1]] == nil then names[#names+1] = e[1] end
    parts[e[1]] = (parts[e[1]] or 0) + e[2] * t
  end
  local out = {}
  for _, n in ipairs(names) do
    if parts[n] > 0.01 then out[#out+1] = {n, parts[n]} end
  end
  out.medium = med or 0.08
  return pile(out)
end
function slices(y0, y1, A, B, wd, sd, ramp, n, tspan)
  n = n or 8
  for i = 0, n do
    local cx = 1000 * i / n
    local t = clamp(cx / (tspan or 1000), 0, 1)
    local hw = (1000 / n) * 1.15
    hb(rect(cx - hw, y0, 2 * hw, y1 - y0), mixp(A, B, t), wd, sd + i, 3.0, {60,150}, {load_at=function(x, y)
        local wx = clamp(1 - math.abs(x - cx) / hw, 0.03, 1)
        local wy = clamp((y - y0) / ramp, 0.05, 1) * clamp((y1 - y) / ramp, 0.05, 1)
        return wx * wy end})
  end
end
upA = {{"lead white",6},{"yellow ochre",0.5},{"pale smalt",0.6},{"chrome yellow",0.15}}
upB = {{"lead white",4.6},{"pale smalt",0.9},{"red earth",0.22},{"vermilion",0.4}}
miA = {{"lead white",5.5},{"chrome yellow",0.6},{"yellow ochre",0.3},{"vermilion",0.12}}
miB = {{"lead white",4.5},{"vermilion",0.5},{"chrome yellow",0.3},{"pale smalt",0.4}}
loA = {{"lead white",5},{"chrome yellow",0.85},{"vermilion",0.25},{"yellow ochre",0.2}}
loB = {{"lead white",4.2},{"vermilion",0.7},{"chrome yellow",0.35},{"red earth",0.15},{"pale smalt",0.3}}
slices(372, 446, upA, upB, 24, 3000, 30, 8, 800)
print(wait(0))
slices(420, 484, miA, miB, 22, 3020, 26, 8, 800)
print(wait(0))
slices(462, 502, loA, loB, 18, 3040, 16, 8, 800)
print(wait(0))
local zz = rect(-20, 362, 1040, 142):blur(12)
blend(zz, {angle=math.pi/2})
print(wait(0))
blend(zz, {angle=0.0})
print(wait(0))

--@ chunk 85
wTA = {{"lead white",5.5},{"chrome yellow",0.55},{"yellow ochre",0.3},{"pale smalt",0.5}}
wTB = {{"lead white",4.2},{"vermilion",0.4},{"pale smalt",1.0},{"red earth",0.2}}
wUA = {{"lead white",4.5},{"pale smalt",1.4},{"vermilion",0.25},{"yellow ochre",0.25}}
wUB = {{"lead white",3.6},{"pale smalt",1.8},{"vermilion",0.3},{"red earth",0.3},{"smalt",0.4}}
local water = rect(-10, 497, 1020, 260)
-- crisp horizon: every pass is clipped to the water mask
function slices_c(y0, y1, A, B, wd, sd, ramp_top, ramp_bot, n, tspan)
  n = n or 8
  for i = 0, n do
    local cx = 1000 * i / n
    local t = clamp(cx / (tspan or 1000), 0, 1)
    local hw = (1000 / n) * 1.15
    hb(rect(cx - hw, y0, 2 * hw, y1 - y0) * water, mixp(A, B, t), wd, sd + i, 3.2, {60,150}, {clip=water, edge="found", load_at=function(x, y)
        local wx = clamp(1 - math.abs(x - cx) / hw, 0.03, 1)
        local wy = clamp((y - y0) / ramp_top, 0.3, 1) * clamp((y1 - y) / ramp_bot, 0.05, 1)
        return wx * wy end})
  end
end
slices_c(497, 535, wTA, wTB, 16, 3100, 4, 14, 8, 800)
print(wait(0))
slices_c(522, 580, wUA, wUB, 18, 3120, 10, 20, 8, 800)
print(wait(0))
hb(rect(-10, 566, 1020, 66), wb2, 20, 3140, 3.2, {90,260}, {load_at=function(x,y) return clamp((y-566)/22,0.05,1) * clamp((632-y)/20,0.05,1) end})
print(wait(0))
hb(rect(-10, 616, 1020, 78), wb3, 22, 3141, 3.2, {90,260}, {load_at=function(x,y) return clamp((y-616)/22,0.05,1) * clamp((694-y)/20,0.05,1) end})
print(wait(0))
hb(rect(-10, 678, 1020, 70), wb4, 22, 3142, 3.2, {90,260}, {load_at=function(x,y) return clamp((y-678)/22,0.05,1) end})
print(wait(0))
blend(water, {angle=math.pi/2})
print(wait(0))
blend(water, {angle=0.0})
print(wait(0))

--@ chunk 86
water = rect(-10, 497, 1020, 260)
pth_hw = function(y) return 22 + (y - 497) * 0.56 end
local pathm = poly({{300-28,497},{300+28,497},{300+pth_hw(741)+20,745},{300-pth_hw(741)-20,745}})
pth_a = pile{{"lead white",9},{"chrome yellow",0.3},{"vermilion",0.04}, medium=0.06}
pth_b = pile{{"lead white",6},{"chrome yellow",0.45},{"yellow ochre",0.3},{"vermilion",0.12}, medium=0.08}
-- warm broad body of the path first
hb(pathm, pth_b, 14, 3200, 3.0, {30,120}, {clip=water, edge="lost", load_at=function(x, y)
    local d = math.abs(x - 300) / pth_hw(y)
    return clamp(1.25 - d, 0.0, 1) * clamp(1.15 - (y - 497) / 300, 0.2, 1) * clamp((y-497)/6, 0.3, 1) end})
print(wait(0))
-- then the bright core, shorter strokes, brightest at the horizon
hb(pathm, pth_a, 9, 3201, 3.0, {14,70}, {clip=water, edge="lost", load_at=function(x, y)
    local d = math.abs(x - 300) / (pth_hw(y) * 0.75)
    return clamp(1.2 - d, 0.0, 1) * clamp(1.2 - (y - 497) / 200, 0.1, 1) * clamp((y-497)/5, 0.3, 1) end})
print(wait(0))
blend(pathm:grow(20) * water, {angle=0.0})
print(wait(0))

--@ chunk 87
print(drying(300, 600), drying(100, 600))
local bm = rect(90, 497, 420, 250):blur(25)
blend(bm, {angle=0.0})
print(wait(0))
blend(bm, {angle=0.02})
print(wait(0))
blend(bm, {angle=-0.02})
print(wait(0))

--@ chunk 88
print(wait(5*24*60))
print(drying(300, 600), drying(100, 520), drying(500, 450), drying(900, 480))

--@ chunk 89
land_far  = pile{{"lead white",2.2},{"smalt",1.6},{"red earth",0.9},{"raw umber",0.9},{"yellow ochre",0.2}, medium=0.05}
land_dark = pile{{"raw umber",3},{"red earth",1},{"Prussian blue",0.5},{"smalt",1.2},{"lead white",1.4},{"bone black",0.25}, medium=0.04}
land_mid  = pile{{"raw umber",2},{"red earth",1.1},{"smalt",1},{"lead white",2.6},{"green earth",0.4}, medium=0.04}
land_rim  = pile{{"lead white",3},{"red earth",1},{"vermilion",0.5},{"chrome yellow",0.6},{"yellow ochre",0.5}, medium=0.04}
HZ = 497
-- far shore at the left: a hazy sliver along the horizon
local farO = outline{{-20,HZ},{-20,493},{20,492},{50,489},{75,491},{110,492},{150,489},{185,491},{240,492},{300,493},{360,492},{400,490},{409,481},{412,481},{414,490},{430,491},{480,492},{530,493},{575,494},{575,HZ}, closed=true, char="firm", amount=0.4, seed=201}
farM2 = farO:mask()
work(farM2, {hand="detail", tool={kind="round", width=3}, pile=land_far, coverage=3, clip=farM2, seed=3301})
print(wait(0))
-- right land with tree crest
local rightO = outline{{540,HZ},{540,495},{560,493.5},{600,491},{640,489},{672,487},{700,485},{730,483},{760,478},{785,470},{805,461},{822,453},{836,445},{846,437},
  {852,431},{860,433},{868,427},{876,432},{884,429},{894,433},{902,431},{912,435},{924,438},{934,442},{944,441},{956,447},{975,450},{1000,451},{1020,452},{1020,HZ}, closed=true, char="firm", amount=0.35, seed=202}
rightM = rightO:mask()
work(rightM, {hand="body", tool={kind="filbert", width=8}, pile=land_dark, length={16,50}, angle=0.0, angle_jitter=0.08, coverage=3.6, fill=true, clip=rightM, seed=3302})
print(wait(0))

--@ chunk 90
tA = pile{{"raw umber",3},{"bone black",1},{"Prussian blue",0.6},{"red earth",0.5}, medium=0.03}
tB = pile{{"raw umber",2},{"Prussian blue",1},{"bone black",0.6},{"green earth",0.6}, medium=0.03}
tC = pile{{"smalt",2},{"raw umber",2},{"bone black",0.8},{"red earth",0.6},{"lead white",0.5}, medium=0.03}
tD = pile{{"raw umber",2.5},{"Prussian blue",0.8},{"lead white",1.2},{"bone black",0.4},{"vermilion",0.2}, medium=0.03}
local ps = {tA, tB, tC, tD}
for i, p in ipairs(ps) do
  local m = rect(700 + (i-1)*75, 460, 75, 37) * rightM
  work(m, {hand="body", tool={kind="filbert", width=8}, pile=p, length={16,40}, angle=0.0, angle_jitter=0.08, coverage=3.6, fill=true, clip=m, seed=3400+i})
end
print(wait(0))

--@ chunk 91
-- opaque skyward coat over the ghost of the first headland (paint under is dry now)
gh_up = mixp(upA, upB, 1.0, 0.02)
gh_mi = mixp(miA, miB, 1.0, 0.02)
gh_lo = mixp(loA, loB, 1.0, 0.02)
local ghost = rect(640, 396, 380, 101)
local function fx(x) return clamp((x - 640) / 60, 0.04, 1) end
hb(rect(640, 396, 380, 60), gh_up, 20, 3501, 4.0, {60,140}, {clip=ghost, load_at=function(x,y) return fx(x) * clamp((y-396)/28, 0.04, 1) * clamp((458-y)/14, 0.1, 1) end})
print(wait(0))
hb(rect(640, 430, 380, 50), gh_mi, 18, 3502, 4.0, {60,140}, {clip=ghost, load_at=function(x,y) return fx(x) * clamp((y-430)/16, 0.04, 1) * clamp((482-y)/12, 0.1, 1) end})
print(wait(0))
hb(rect(640, 462, 380, 35), gh_lo, 16, 3503, 4.0, {60,140}, {clip=ghost, load_at=function(x,y) return fx(x) * clamp((y-462)/12, 0.04, 1) end})
print(wait(0))
blend(rect(630, 390, 400, 108):blur(10), {angle=0.0})
print(wait(0))

--@ chunk 92
print(wait(6*24*60))
print(drying(700,470), drying(850,450), drying(300,600), drying(300,300))

--@ chunk 93
gh_up = mixp(upA, upB, 1.0, 0.0)
gh_mi = mixp(miA, miB, 1.0, 0.0)
gh_lo = mixp(loA, loB, 1.0, 0.0)
local ghost = rect(640, 396, 380, 101)
local function fx(x) return clamp((x - 640) / 60, 0.04, 1) end
hb(rect(640, 396, 380, 64), gh_up, 20, 3601, 5.0, {60,140}, {clip=ghost, load_at=function(x,y) return fx(x) * clamp((y-396)/26, 0.04, 1) * clamp((464-y)/12, 0.1, 1) end})
print(wait(0))
hb(rect(640, 430, 380, 54), gh_mi, 18, 3602, 5.0, {60,140}, {clip=ghost, load_at=function(x,y) return fx(x) * clamp((y-430)/14, 0.04, 1) * clamp((486-y)/10, 0.1, 1) end})
print(wait(0))
hb(rect(640, 462, 380, 35), gh_lo, 16, 3603, 5.0, {60,140}, {clip=ghost, load_at=function(x,y) return fx(x) * clamp((y-462)/10, 0.04, 1) end})
print(wait(0))

--@ chunk 94
function slices2(y0, y1, A, B, wd, sd, ramp_top, ramp_bot, x0, x1, n, cov, span, med)
  for i = 0, n do
    local cx = x0 + (x1 - x0) * i / n
    local t = clamp(cx / (span or 800), 0, 1)
    local hw = ((x1 - x0) / n) * 1.2
    local rx0 = math.max(cx - hw, x0 - 30)
    hb(rect(rx0, y0, cx + hw - rx0, y1 - y0), mixp(A, B, t, med or 0.02), wd, sd + i, cov or 4.0, {60,150}, {load_at=function(x, y)
        local wx = clamp(1 - math.abs(x - cx) / hw, 0.02, 1)
        if i == 0 then wx = wx * clamp((x - x0 + 20) / 60, 0.0, 1) end
        local wy = clamp((y - y0) / ramp_top, 0.02, 1) * clamp((y1 - y) / ramp_bot, 0.05, 1)
        return wx * wy end})
  end
end
slices2(396, 462, upA, upB, 20, 3700, 34, 14, 560, 1010, 6, 4.0)
print(wait(0))
slices2(430, 486, miA, miB, 18, 3720, 18, 12, 560, 1010, 6, 4.0)
print(wait(0))
slices2(462, 497, loA, loB, 16, 3740, 12, 6, 560, 1010, 6, 4.0)
print(wait(0))

--@ chunk 95
slices2(398, 452, upA, upB, 22, 3800, 30, 14, 600, 1010, 5, 5.5)
print(wait(0))
slices2(428, 476, miA, miB, 20, 3820, 16, 12, 600, 1010, 5, 5.5)
print(wait(0))
slices2(458, 497, loA, loB, 18, 3840, 12, 6, 600, 1010, 5, 5.5)
print(wait(0))
local zz = rect(560, 392, 460, 106):blur(10)
blend(zz, {angle=0.0})
print(wait(0))

--@ chunk 96
print(wait(6*24*60))
print(drying(700,470), drying(850,450), drying(300,600), drying(650,410))

--@ chunk 97
-- repair the horizon at the right: water colour laid back over the sky's spill, clipped to the water so the line is crisp
water = rect(-10, 497, 1020, 260)
wfix_top = mixp(wTA, wTB, 0.9, 0.04)
wfix_mid = mixp(wUA, wUB, 0.9, 0.04)
local zone = rect(500, 497, 520, 30) * water
hb(zone, wfix_top, 12, 3901, 3.6, {60,160}, {clip=water, edge="found", load_at=function(x,y) return clamp((x-500)/40, 0.05, 1) * clamp((526-y)/14, 0.1, 1) end})
print(wait(0))
hb(rect(500, 512, 520, 24) * water, wfix_mid, 12, 3902, 3.2, {60,160}, {clip=water, edge="found", load_at=function(x,y) return clamp((x-500)/40, 0.05, 1) * clamp((y-512)/8, 0.1, 1) * clamp((536-y)/14, 0.1, 1) end})
print(wait(0))

--@ chunk 98
print(wait(4*24*60))
print(drying(700,505), drying(700,470))

--@ chunk 99
HZ = 497
land_lil = pile{{"raw umber",1.6},{"smalt",1.6},{"red earth",1.0},{"lead white",2.4},{"green earth",0.3}, medium=0.04}
land_blu = pile{{"raw umber",2},{"smalt",2},{"Prussian blue",0.25},{"red earth",0.6},{"lead white",1.6}, medium=0.04}
-- low spit and hill: one drawn contour, bumps on top are hand-placed
landO = outline{{528,HZ+2},{528,HZ-1},{548,HZ-3},{580,HZ-5},{615,HZ-7},{650,HZ-9},{690,HZ-11},{720,HZ-15},{745,HZ-21},{765,HZ-28},{790,HZ-32},{815,HZ-36},{840,HZ-42},
  {870,HZ-50},{905,HZ-55},{940,HZ-52},{975,HZ-47},{1020,HZ-44},{1020,HZ+2}, closed=true, char="firm", amount=0.5, lobe=18, seed=311}
landM = landO:mask()
work(landM, {hand="body", tool={kind="filbert", width=7}, pile=land_lil, length={14,40}, angle=0.0, angle_jitter=0.1, coverage=3.4, fill=true, clip=landM, mix_jitter=0.5, seed=4101})
print(wait(0))
work(landM * rect(700, 440, 330, 60), {hand="body", tool={kind="filbert", width=7}, pile=land_blu, length={12,34}, angle=0.05, angle_jitter=0.2, coverage=1.4, fill=false, clip=landM, mix_jitter=0.6, load_at=function(x,y) return clamp((x-700)/200, 0.1, 1) end, seed=4102})
print(wait(0))

--@ chunk 100
print(wait(4*24*60))
print(drying(900,470), drying(600,494))

--@ chunk 101
print(wait(3*24*60))
print(drying(900,470), drying(600,494), drying(700,490), drying(560,495))

--@ chunk 102
HZ = 497
farhill = pile{{"lead white",2.6},{"smalt",1.8},{"red earth",0.9},{"raw umber",0.5},{"yellow ochre",0.1}, medium=0.04}
farhill_lo = pile{{"lead white",3.4},{"smalt",1.2},{"red earth",1.0},{"raw umber",0.3},{"vermilion",0.15}, medium=0.04}
fhO = outline{{520,HZ+3},{520,HZ-1},{545,HZ-5},{580,HZ-9},{620,HZ-14},{665,HZ-22},{710,HZ-33},{750,HZ-45},{790,HZ-56},{830,HZ-64},{865,HZ-69},{900,HZ-72},{935,HZ-73},{975,HZ-70},{1020,HZ-66},{1020,HZ+3},
  closed=true, char="firm", amount=0.2, lobe=60, seed=321}
fhM = fhO:mask()
print(fhM:area())
work(fhM, {hand="broad", tool={kind="flat", width=12}, pile=farhill, length={50,150}, angle=0.0, angle_jitter=0.03, coverage=3.4, fill=true, clip=fhM, mix_jitter=0.1, seed=4201})
print(wait(0))
work(fhM * below({{500,HZ-30},{700,HZ-24},{1020,HZ-36}}), {hand="broad", tool={kind="flat", width=10}, pile=farhill_lo, length={50,140}, angle=0.0, angle_jitter=0.03, coverage=2.6, fill=true, clip=fhM, mix_jitter=0.1,
     load_at=function(x,y) return clamp((y - (HZ-36)) / 18, 0.05, 1) end, seed=4202})
print(wait(0))

--@ chunk 103
print(drying(700, 480))
local rimtop = fhM:rim(14, 8) * above({{500,HZ-2},{700,HZ-8},{1020,HZ-30}})
blend(rimtop, {angle=math.pi/2, clip=false})
print(wait(0))
blend(rimtop, {angle=0.03, clip=false})
print(wait(0))

--@ chunk 104
print(wait(5*24*60))
print(drying(700, 480), drying(725, 450), drying(737, 515))

--@ chunk 105
-- Repair 2: remove the smeared hill (sky above and water below), wet-in-wet, then blend. Land will be repainted on dry paint.
slices2(392, 452, upA, upB, 22, 5000, 30, 14, 600, 1010, 5, 5.5)
print(wait(0))
slices2(424, 480, miA, miB, 20, 5020, 16, 12, 600, 1010, 5, 5.5)
print(wait(0))
slices2(452, 497, loA, loB, 18, 5040, 12, 5, 600, 1010, 5, 5.5)
print(wait(0))
-- water below: crisp horizon, clipped
water = rect(-10, 497, 1020, 260)
function slices_c2(y0, y1, A, B, wd, sd, ramp_top, ramp_bot, x0, x1, n, cov, span, med)
  for i = 0, n do
    local cx = x0 + (x1 - x0) * i / n
    local t = clamp(cx / (span or 800), 0, 1)
    local hw = ((x1 - x0) / n) * 1.2
    local rx0 = math.max(cx - hw, x0 - 30)
    hb(rect(rx0, y0, cx + hw - rx0, y1 - y0) * water, mixp(A, B, t, med or 0.03), wd, sd + i, cov or 4.0, {60,150}, {clip=water, edge="found", load_at=function(x, y)
        local wx = clamp(1 - math.abs(x - cx) / hw, 0.02, 1)
        if i == 0 then wx = wx * clamp((x - x0 + 20) / 60, 0.0, 1) end
        local wy = clamp((y - y0) / ramp_top, 0.3, 1) * clamp((y1 - y) / ramp_bot, 0.05, 1)
        return wx * wy end})
  end
end
slices_c2(497, 540, wTA, wTB, 16, 5100, 4, 16, 560, 1010, 5, 4.5)
print(wait(0))
slices_c2(524, 590, wUA, wUB, 18, 5120, 10, 24, 560, 1010, 5, 4.5)
print(wait(0))
local zz = rect(560, 392, 460, 106):blur(10)
blend(zz, {angle=0.0})
print(wait(0))
blend(water * rect(540, 497, 480, 100):blur(10), {angle=0.0})
print(wait(0))

--@ chunk 106
print(drying(725,437), drying(500,480), drying(800,540), drying(300,650))
-- local sky patches over leftover spots and the wing of the old smear (all wet-in-wet)
local function patch(x0, y0, w, h, t, sd, y_lo_ramp)
  local pu = mixp(upA, upB, t, 0.03)
  local pm = mixp(miA, miB, t, 0.03)
  local pl = mixp(loA, loB, t, 0.03)
  local m = rect(x0, y0, w, h)
  hb(m, pm, 16, sd, 4.0, {30,90}, {load_at=function(x,y) return clamp(1 - math.abs(x-(x0+w/2))/(w/2), 0.05, 1) * clamp(1 - math.abs(y-(y0+h/2))/(h/2), 0.05, 1) end})
end
patch(696, 412, 60, 50, 0.9, 5201)
patch(925, 418, 60, 54, 1.0, 5202)
patch(430, 450, 190, 47, 0.6, 5203)
print(wait(0))
-- broad soft blends to dissolve the seams (wet paint): first the sky band, then the water
blend(rect(400, 380, 640, 117):blur(40), {angle=0.0})
print(wait(0))
blend(rect(400, 380, 640, 117):blur(40), {angle=math.pi/2})
print(wait(0))
blend(rect(380, 497, 660, 260):blur(70) * water, {angle=0.0})
print(wait(0))
blend(rect(380, 497, 660, 260):blur(70) * water, {angle=math.pi/2})
print(wait(0))
blend(rect(380, 497, 660, 260):blur(70) * water, {angle=0.0})
print(wait(0))

--@ chunk 107
-- cover the two dry ghost marks with an opaque pale coat, then soften its edge while it is wet
ghostA = pile{{"lead white",6},{"yellow ochre",0.45},{"vermilion",0.16},{"pale smalt",0.3}, medium=0.02}
ghostB = pile{{"lead white",6},{"yellow ochre",0.3},{"vermilion",0.22},{"pale smalt",0.45}, medium=0.02}
local mA = ellipse(725, 366, 34, 34)
local mB = ellipse(955, 372, 34, 34)
work(mA, {hand="body", tool={kind="filbert", width=12}, pile=ghostA, length={12,36}, angle=0.05, angle_jitter=0.2, coverage=5, fill=true, edge="soft", seed=6001})
work(mB, {hand="body", tool={kind="filbert", width=12}, pile=ghostB, length={12,36}, angle=0.05, angle_jitter=0.2, coverage=5, fill=true, edge="soft", seed=6002})
print(wait(0))
blend(mA:grow(22), {angle=0.0, clip=false})
blend(mB:grow(22), {angle=0.0, clip=false})
print(wait(0))

--@ chunk 108
zA = pile{{"lead white",6},{"yellow ochre",0.35},{"vermilion",0.16},{"pale smalt",0.35}, medium=0.04}
zB = pile{{"lead white",5},{"vermilion",0.42},{"chrome yellow",0.22},{"pale smalt",0.5},{"red earth",0.08}, medium=0.04}
zC = pile{{"lead white",4.6},{"chrome yellow",0.62},{"vermilion",0.55},{"yellow ochre",0.15}, medium=0.04}
local X0 = 470
local function lx(x) return clamp((x - X0) / 130, 0.03, 1) end
-- zone C (horizon apricot), then B (pink), then A (cream under the cloud)
hb(rect(X0-20, 452, 1050-X0, 46), zC, 26, 6101, 4.5, {200,420}, {load_at=function(x,y) return lx(x) * clamp((y-452)/14, 0.04, 1) end})
print(wait(0))
hb(rect(X0-20, 410, 1050-X0, 70), zB, 30, 6102, 4.5, {200,420}, {load_at=function(x,y) return lx(x) * clamp((y-410)/24, 0.04, 1) * clamp((482-y)/20, 0.05, 1) end})
print(wait(0))
hb(rect(X0-20, 366, 1050-X0, 70), zA, 30, 6103, 4.5, {200,420}, {load_at=function(x,y) return lx(x) * clamp((y-366)/14, 0.1, 1) * clamp((438-y)/28, 0.05, 1) end})
print(wait(0))
local bm = rect(X0-40, 360, 1080-X0, 140):blur(16)
blend(bm, {angle=0.0})
print(wait(0))

--@ chunk 109
print(wait(6*24*60))
print(drying(700,480), drying(700,400), drying(500,470), drying(300,650))

--@ chunk 110
HZ = 497
skyline = above({{-50,HZ},{1050,HZ}})
hill_a = pile{{"lead white",3.2},{"smalt",1.5},{"cobalt blue",0.25},{"red earth",0.65},{"raw umber",0.4}, medium=0.04}
hill_b = pile{{"lead white",2.6},{"smalt",1.6},{"red earth",0.7},{"raw umber",0.55},{"cobalt blue",0.2}, medium=0.04}
fhO = outline{{500,HZ+4},{500,HZ-1},{540,HZ-3},{580,HZ-6},{630,HZ-10},{690,HZ-16},{740,HZ-22},{790,HZ-29},{840,HZ-35},{890,HZ-40},{935,HZ-43},{975,HZ-42},{1020,HZ-39},{1020,HZ+4},
  closed=true, char="soft", amount=0.3, lobe=50, seed=331}
fhM = fhO:mask()
farClip = fhM:blur(5) * skyline
work(farClip, {hand="broad", tool={kind="flat", width=10}, pile=hill_a, length={60,160}, angle=0.0, angle_jitter=0.03, coverage=3.2, fill=true, clip=farClip, seed=7001})
print(wait(0))
-- slightly deeper toward the base, on the right
work(farClip * rect(560, HZ-24, 460, 30), {hand="broad", tool={kind="flat", width=8}, pile=hill_b, length={50,140}, angle=0.0, angle_jitter=0.03, coverage=2.4, fill=true, clip=farClip,
     load_at=function(x,y) return clamp((y-(HZ-24))/24, 0.05, 1) * clamp((x-560)/200, 0.1, 1) end, seed=7002})
print(wait(0))

--@ chunk 111
-- Preview a new, calmer cumulus model as an ASCII map (no painting)
local L = {
 -- {x, y, z, rx, ry, rz}
 {760,330,10, 100,46,70},   -- low left shoulder
 {905,338,10, 130,44,70},   -- low right shoulder
 {820,262,30, 95,62,72},    -- mid-left bulge
 {950,250,20, 95,80,70},    -- mid-right mass
 {875,178,45, 82,70,70},    -- main tower
 {800,168,30, 46,42,42},    -- left knuckle
 {950,140,40, 70,70,60},    -- upper right
 {885,95,45, 62,54,52},     -- crown
 {820,102,25, 34,30,30},    -- small crown lobe
 {985,60,30, 46,50,40},     -- top right corner mass
}
local s
for i, l in ipairs(L) do
  local e = body.ellipsoid({l[1], l[2], l[3]}, {l[4], l[5], l[6]}):rough(4, 90, 40 + i)
  s = s and s:union(e) or e
end
cum2 = form{ {s}, light={from={-1, -0.25}, front=0.35, ambient=0.2} }
cum2sil = cum2:silhouette{}
local chars = " .:-=+*#%@"
for y = 30, 390, 12 do
  local row = {}
  for x = 690, 1000, 6 do
    if cum2sil:at(x, y) > 0.5 then
      local v = clamp(cum2:value(x, y), 0, 0.999)
      local k = math.floor(v * 10) + 1
      row[#row+1] = chars:sub(k, k)
    else
      row[#row+1] = "'"
    end
  end
  print(string.format("%3d ", y) .. table.concat(row))
end

--@ chunk 112
-- where does the old cumulus stick out beyond the new model?
for y = 30, 390, 12 do
  local row = {}
  for x = 690, 1000, 6 do
    local o = cumsil:at(x, y) > 0.5
    local n = cum2sil:at(x, y) > 0.5
    row[#row+1] = (o and n) and "#" or (o and "O" or (n and "n" or "."))
  end
  print(string.format("%3d ", y) .. table.concat(row))
end

--@ chunk 113
for y = 30, 400, 10 do
  local row = {}
  for x = 540, 1000, 6 do
    local o = cumsil:at(x, y) > 0.5
    row[#row+1] = o and "#" or "."
  end
  print(string.format("%3d ", y) .. table.concat(row))
end

--@ chunk 114
-- same cloud lumps as before, a touch larger and smooth, so modelling reads in big planes
local L = {
 {640,352,0, 88,40,60}, {775,358,10, 96,42,60}, {915,352,10, 100,44,60}, {1010,350,0, 70,44,50},
 {700,296,30, 76,52,60}, {820,300,30, 84,54,62}, {945,292,20, 82,56,58},
 {770,232,40, 74,56,60}, {885,236,45, 78,58,60}, {985,226,20, 60,60,50},
 {835,168,50, 66,54,56}, {940,160,50, 64,58,54}, {760,178,30, 44,40,40},
 {880,104,50, 56,46,48}, {800,112,30, 36,34,34}, {965,96,40, 44,44,40},
}
local s
for i, l in ipairs(L) do
  local e = body.ellipsoid({l[1], l[2], l[3]}, {l[4]*1.1, l[5]*1.1, l[6]*1.1}):rough(3, 110, 60 + i)
  s = s and s:union(e) or e
end
cm = form{ {s}, light={from={-1, -0.3}, front=0.35, ambient=0.2} }
cmsil = cm:silhouette{}
cm_across = cm:field("across")
-- coverage check: old silhouette not covered by the new one
local miss = 0
for y = 40, 400, 4 do for x = 560, 1000, 4 do
  if cumsil:at(x, y) > 0.5 and cmsil:at(x, y) < 0.5 then miss = miss + 1 end
end end
print("uncovered samples", miss)
local vals = {}
for y = 40, 400, 5 do for x = 560, 1000, 5 do
  if cumsil:at(x, y) > 0.5 then vals[#vals+1] = cm:value(x, y) end
end end
table.sort(vals)
local n = #vals
for _, q in ipairs({0.02, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 0.97, 1}) do
  print(q, string.format("%.3f", vals[math.max(1, math.min(n, math.floor(q * n)))]))
end

--@ chunk 115
cShDeep = pile{{"lead white",2.2},{"smalt",2.0},{"cobalt blue",0.6},{"red earth",0.7},{"raw umber",0.55}, medium=0.08}
cSh     = pile{{"lead white",3.4},{"smalt",1.5},{"cobalt blue",0.45},{"red earth",0.6},{"raw umber",0.3}, medium=0.08}
cMid    = pile{{"lead white",4.8},{"pale smalt",0.9},{"yellow ochre",0.4},{"red earth",0.42}, medium=0.08}
cLit    = pile{{"lead white",6.5},{"yellow ochre",0.42},{"chrome yellow",0.3},{"vermilion",0.22}, medium=0.06}
cHigh   = pile{{"lead white",9},{"chrome yellow",0.32},{"vermilion",0.07}, medium=0.05}
cPeach  = pile{{"lead white",3.2},{"vermilion",0.5},{"yellow ochre",0.4},{"pale smalt",0.4}, medium=0.08}

function cband(lo, hi, s)
  return mask(function(x, y)
    if x < 560 or y > 400 or y < 30 then return 0 end
    if cumsil:at(x, y) < 0.5 then return 0 end
    local v = cm:value(x, y)
    local a = smoothstep(lo - s, lo + s, v)
    local b = 1 - smoothstep(hi - s, hi + s, v)
    return a * b
  end)
end
b_all   = cband(-1, 2, 0.05)
b_sh    = cband(0.19, 2, 0.05)
b_mid   = cband(0.30, 2, 0.06)
b_lit   = cband(0.44, 2, 0.06)
b_high  = cband(0.66, 2, 0.05)
-- base coat: the whole mass in the cool deep shadow tone
work(b_all, {hand="body", tool={kind="filbert", width=18}, pile=cShDeep, angle=cm_across, length={40,110}, coverage=3.2, fill=true, edge="soft", threshold=0.4, seed=8001})
print(wait(0))
work(b_sh, {hand="body", tool={kind="filbert", width=15}, pile=cSh, angle=cm_across, length={30,90}, coverage=2.6, fill=true, edge="soft", threshold=0.4, seed=8002})
print(wait(0))
work(b_mid, {hand="body", tool={kind="filbert", width=13}, pile=cMid, angle=cm_across, length={26,80}, coverage=2.4, fill=true, edge="soft", threshold=0.4, seed=8003})
print(wait(0))
work(b_lit, {hand="body", tool={kind="filbert", width=11}, pile=cLit, angle=cm_across, length={20,60}, coverage=2.2, fill=true, edge="soft", threshold=0.4, seed=8004})
print(wait(0))
work(b_high, {hand="body", tool={kind="filbert", width=9}, pile=cHigh, angle=cm_across, length={14,44}, coverage=2.0, fill=true, edge="soft", threshold=0.4, seed=8005})
print(wait(0))

--@ chunk 116
print(drying(800,200), drying(700,380))
-- soften the shadow planes only, along the contour direction; lit faces keep their strokes
b_shadowzone = cband(-1, 0.42, 0.06)
blend(b_shadowzone, {angle=cm_across})
print(wait(0))
-- feather the outline against the sky (cloud paint is still wet; sky is dry and will not move)
local ring = cmsil:rim(10, 5) * rect(560, 30, 460, 340)
blend(ring, {angle=cm_across, clip=false})
print(wait(0))

--@ chunk 117
print(wait(4*24*60))
print(drying(800,200), drying(700,110), drying(540,255))

--@ chunk 118
for y = 60, 300, 20 do
  local xe = nil
  for x = 560, 1000, 2 do
    if cmsil:at(x, y) > 0.5 then xe = x break end
  end
  print(y, xe)
end

--@ chunk 119
ears1 = rect(636, 72, 136, 92) - cmsil:shrink(3)
-- try the original top-sky mixture, soft edged, in wet-in-wet strokes following the sky's own horizontal drift
work(ears1, {hand="body", tool={kind="filbert", width=12}, pile=sky_top2, length={20,60}, angle=0.02, angle_jitter=0.05, coverage=4, fill=true, edge="soft", threshold=0.3, seed=9001})
print(wait(0))

--@ chunk 120
skyP1 = pile{{"smalt",2.6},{"cobalt blue",1.3},{"lead white",2.1},{"raw umber",0.28},{"Prussian blue",0.05}, medium=0.2}
skyP2 = pile{{"smalt",2.3},{"cobalt blue",1.0},{"lead white",2.6},{"raw umber",0.25}, medium=0.2}
-- work the wet patch: darker/greyer toward the top, so it matches the sky's gradient
work(rect(636, 72, 136, 46) - cmsil:shrink(3), {hand="body", tool={kind="filbert", width=12}, pile=skyP1, length={20,60}, angle=0.02, angle_jitter=0.05, coverage=3.5, fill=true, edge="soft", threshold=0.3, seed=9002})
work(rect(636, 100, 136, 64) - cmsil:shrink(3), {hand="body", tool={kind="filbert", width=12}, pile=skyP2, length={20,60}, angle=0.02, angle_jitter=0.05, coverage=2.5, fill=true, edge="soft", threshold=0.3, seed=9003,
     load_at=function(x,y) return clamp((y-100)/30, 0.05, 1) end})
print(wait(0))
blend(ears1, {angle=math.pi/2})
blend(ears1, {angle=0.02})
print(wait(0))

--@ chunk 121
print(drying(700,110))
skyP0 = pile{{"smalt",3},{"cobalt blue",1.2},{"lead white",1.3},{"raw umber",0.35},{"Prussian blue",0.06}, medium=0.15}
local core = rect(646, 78, 118, 76) - cmsil:shrink(3)
work(core, {hand="body", tool={kind="filbert", width=12}, pile=skyP0, length={20,60}, angle=0.02, angle_jitter=0.05, coverage=4.5, fill=true, edge="soft", threshold=0.3, seed=9004,
     load_at=function(x,y) return clamp(1.1 - (y-78)/90, 0.25, 1) end})
print(wait(0))
blend(ears1, {angle=0.02})
print(wait(0))

--@ chunk 122
skyPm = pile{{"smalt",2.75},{"cobalt blue",1.2},{"lead white",1.75},{"raw umber",0.3},{"Prussian blue",0.05}, medium=0.18}
local big = rect(626, 62, 156, 112) - cmsil:shrink(3)
work(big, {hand="body", tool={kind="filbert", width=14}, pile=skyPm, length={24,70}, angle=0.02, angle_jitter=0.05, coverage=4, fill=true, edge="soft", threshold=0.3, seed=9005,
     load_at=function(x,y) 
        local dx = clamp(1.25 - math.abs(x - 700)/70, 0.1, 1)
        local dy = clamp(1.25 - math.abs(y - 115)/60, 0.1, 1)
        return dx * dy end})
print(wait(0))
blend(big, {angle=0.02})
print(wait(0))
blend(big, {angle=math.pi/2})
print(wait(0))

--@ chunk 123
print(wait(5*24*60))
print(drying(700,110), drying(800,250))

--@ chunk 124
SK = {
 {-20, {{"smalt",3},{"cobalt blue",1.2},{"lead white",1.3},{"raw umber",0.35},{"Prussian blue",0.06}}},
 {110, {{"smalt",2.4},{"cobalt blue",1.2},{"lead white",2.2},{"raw umber",0.25}}},
 {200, {{"pale smalt",2.5},{"lead white",3.6},{"cobalt blue",0.5},{"yellow ochre",0.2}}},
 {270, {{"lead white",5},{"pale smalt",2},{"yellow ochre",0.35},{"cobalt blue",0.2}}},
 {345, {{"lead white",5.5},{"pale smalt",1.2},{"yellow ochre",0.5},{"red earth",0.08}}},
}
function skycol(y)
  for i = 1, #SK - 1 do
    local a, b = SK[i], SK[i+1]
    if y <= b[1] or i == #SK - 1 then
      local t = clamp((y - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[2], b[2], t, 0.15)
    end
  end
end
local nocloud = -cmsil:shrink(3)
local centers = {10, 60, 110, 160, 210, 260, 310, 350}
for i, yc in ipairs(centers) do
  local hh = 42
  local m = rect(-20, yc - hh, 1040, 2 * hh) * nocloud
  hb(m, skycol(yc), 26, 9100 + i, 3.2, {150,320}, {angle_jitter=0.015, load_at=function(x, y) return clamp(1.15 - math.abs(y - yc) / hh, 0.04, 1) end})
  print(yc, wait(0))
end

--@ chunk 125
local skym = rect(-20, -20, 1040, 400) * (-cmsil:shrink(2))
blend(skym, {angle=math.pi/2})
print(wait(0))
blend(skym, {angle=0.0})
print(wait(0))
blend(skym, {angle=0.02})
print(wait(0))

--@ chunk 126
print(wait(4*24*60))
print(drying(300,300), drying(300,100), drying(700,250))

--@ chunk 127
SK2 = {
 {-20, {{"smalt",3.2},{"cobalt blue",1.4},{"Prussian blue",0.12},{"lead white",0.9},{"raw umber",0.25}}},
 {90,  {{"smalt",2.8},{"cobalt blue",1.3},{"lead white",1.6},{"raw umber",0.22},{"Prussian blue",0.05}}},
 {180, {{"smalt",2.0},{"cobalt blue",0.9},{"lead white",3.0},{"yellow ochre",0.2},{"raw umber",0.12}}},
 {265, {{"pale smalt",2.5},{"lead white",4.3},{"cobalt blue",0.4},{"yellow ochre",0.35},{"red earth",0.05}}},
 {340, {{"lead white",5.5},{"pale smalt",1.2},{"yellow ochre",0.5},{"red earth",0.08}}},
}
function skycol2(y)
  for i = 1, #SK2 - 1 do
    local a, b = SK2[i], SK2[i+1]
    if y <= b[1] or i == #SK2 - 1 then
      local t = clamp((y - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[2], b[2], t, 0.15)
    end
  end
end
local nocloud = -cmsil:shrink(2)
local centers = {0, 50, 100, 150, 200, 250}
for i, yc in ipairs(centers) do
  local hh = 52
  local m = rect(-20, yc - hh, 1040, 2 * hh) * nocloud
  hb(m, skycol2(yc), 28, 9200 + i, 3.4, {160,340}, {angle_jitter=0.012, load_at=function(x, y) return clamp(1.15 - math.abs(y - yc) / hh, 0.04, 1) end})
  print(yc, wait(0))
end
local skym = rect(-20, -20, 1040, 330) * nocloud
blend(skym, {angle=math.pi/2})
print(wait(0))
blend(skym, {angle=0.0})
print(wait(0))

--@ chunk 128
local nocloud = -cmsil:shrink(2)
local tr = {
  {286, {{"pale smalt",2.0},{"lead white",4.6},{"cobalt blue",0.25},{"yellow ochre",0.45},{"red earth",0.06}}},
  {318, {{"lead white",5.2},{"pale smalt",1.5},{"yellow ochre",0.55},{"red earth",0.08}}},
  {352, {{"lead white",5.6},{"pale smalt",0.9},{"yellow ochre",0.65},{"red earth",0.1}}},
  {384, {{"lead white",5.6},{"pale smalt",0.5},{"yellow ochre",0.7},{"vermilion",0.1},{"chrome yellow",0.1}}},
}
for i, t in ipairs(tr) do
  local yc, hh = t[1], 30
  local m = rect(-20, yc - hh, 1040, 2 * hh) * nocloud
  hb(m, mixp(t[2], t[2], 0, 0.12), 28, 9300 + i, 3.4, {160,340}, {angle_jitter=0.01, load_at=function(x, y) return clamp(1.2 - math.abs(y - yc) / hh, 0.04, 1) end})
  print(yc, wait(0))
end
local seam = rect(-20, 270, 1040, 130):blur(12) * nocloud
blend(seam, {angle=math.pi/2})
print(wait(0))
blend(seam, {angle=0.0})
print(wait(0))

--@ chunk 129
print(drying(300,230), drying(300,260))
local nocloud = -cmsil:shrink(2)
local seam2 = rect(-20, 205, 1040, 100):blur(14) * nocloud
blend(seam2, {angle=math.pi/2})
print(wait(0))
blend(seam2, {angle=0.0})
print(wait(0))
blend(seam2, {angle=math.pi/2})
print(wait(0))

--@ chunk 130
local nocloud = -cmsil:shrink(2)
local seam2 = rect(-20, 190, 1040, 130):blur(14) * nocloud
blend(seam2, {angle=0.0})
print(wait(0))
blend(seam2, {angle=0.02})
print(wait(0))
blend(seam2, {angle=-0.02})
print(wait(0))

--@ chunk 131
dusk_a = pile{{"smalt",2.2},{"red earth",1.0},{"raw umber",0.8},{"lead white",1.9},{"cobalt blue",0.4}, medium=0.05}
dusk_b = pile{{"smalt",2.0},{"red earth",0.8},{"raw umber",0.6},{"lead white",2.8},{"cobalt blue",0.4}, medium=0.05}
glow_u = pile{{"lead white",3.0},{"vermilion",0.75},{"chrome yellow",0.45},{"red earth",0.25}, medium=0.05}
glow_v = pile{{"lead white",3.6},{"vermilion",0.5},{"chrome yellow",0.55},{"yellow ochre",0.2}, medium=0.05}

function bar(topPts, botPts, seed, lobe)
  local pts = {}
  for _, p in ipairs(topPts) do pts[#pts+1] = p end
  for _, p in ipairs(botPts) do pts[#pts+1] = p end
  return outline{pts=pts, closed=true, char="soft", lobe=lobe or 28, amount=0.55, seed=seed}:mask()
end
-- bar 1: long, low, left; heavier at the left
bar1 = bar({{-20,426},{40,421},{100,417},{160,420},{220,416},{290,421},{360,425},{430,430},{484,436}},
           {{474,441},{400,440},{330,437},{260,435},{190,437},{120,436},{50,439},{-20,441}}, 501, 34)
work(bar1, {hand="body", tool={kind="filbert", width=7}, pile=dusk_a, length={30,100}, angle=0.0, angle_jitter=0.03, coverage=3, fill=true, edge="soft", seed=9401})
print(wait(0))
-- warm underglow of bar 1, its own drawn shape
glow1 = bar({{-20,437},{60,435},{150,433},{250,431},{340,433},{420,437},{480,441}},
            {{455,445},{350,443},{250,441},{150,442},{60,444},{-20,446}}, 502, 26)
work(glow1, {hand="body", tool={kind="filbert", width=5}, pile=glow_u, length={30,90}, angle=0.0, angle_jitter=0.03, coverage=2.6, fill=true, edge="soft", seed=9402})
print(wait(0))

--@ chunk 132
HZ = 497
shoreD = pile{{"raw umber",3},{"Prussian blue",0.6},{"smalt",1.2},{"red earth",0.8},{"lead white",0.9},{"bone black",0.2}, medium=0.03}
shoreM = pile{{"raw umber",2.4},{"smalt",1.6},{"red earth",1.0},{"lead white",1.9},{"Prussian blue",0.25}, medium=0.03}
-- near shore at the right: low bank, a windmill knoll and a grove; every point is placed by hand
local pts = {
  {688,497},{700,494},{722,491},{746,488},{772,484},{794,479},{812,476},{826,470},
  {834,462},{841,456},{848,460},{854,454},{862,449},{870,452},{878,447},{886,451},{893,458},{900,461},
  {910,457},{917,453},{925,456},{932,462},{944,466},{958,468},{972,466},{988,470},{1004,473},{1020,474},
  {1020,497}
}
shoreO = outline{pts=pts, closed=true, char="firm", amount=0.35, lobe=8, seed=601}
shoreMask = shoreO:mask()
work(shoreMask, {hand="body", tool={kind="filbert", width=6}, pile=shoreD, length={10,30}, angle=0.0, angle_jitter=0.15, coverage=4, fill=true, clip=shoreMask, mix_jitter=0.3, seed=9501})
print(wait(0))
-- lighter, warmer lower bank (catches the horizon glow) at the water line
local lowbank = shoreMask * rect(680, 484, 350, 13)
work(lowbank, {hand="body", tool={kind="filbert", width=4}, pile=shoreM, length={10,30}, angle=0.0, angle_jitter=0.05, coverage=2.4, fill=true, clip=lowbank, seed=9502,
     load_at=function(x,y) return clamp((y-484)/9, 0.05, 0.9) end})
print(wait(0))

--@ chunk 133
treeD = pile{{"raw umber",3},{"green earth",0.7},{"Prussian blue",0.35},{"red earth",0.7},{"bone black",0.35}, medium=0.03}
treeL = pile{{"raw umber",2.2},{"yellow ochre",1.0},{"green earth",0.7},{"red earth",0.6},{"lead white",0.6}, medium=0.03}
-- 1. bank: dark again over the light strip
local bank = shoreMask * rect(680, 476, 350, 22)
work(bank, {hand="body", tool={kind="filbert", width=5}, pile=shoreD, length={10,30}, angle=0.0, angle_jitter=0.05, coverage=4, fill=true, clip=bank, seed=9601})
print(wait(0))
-- 2. crowns, each drawn with its own size
local cr = {
  {836,463, 9,10}, {849,456, 8,9}, {859,452, 10,12}, {872,447, 11,12}, {885,452, 9,10}, {897,458, 8,9},
  {910,459, 9,8}, {921,455, 10,10}, {933,461, 8,8}, {946,466, 9,7}, {962,468, 10,6}, {978,470, 9,6}, {996,473, 10,5},
}
grove = nil
for i, c in ipairs(cr) do
  local e = ellipse(c[1], c[2], c[3], c[4])
  grove = grove and (grove + e) or e
end
grove = grove:roughen(2.2, 9, 7)
grove = grove * rect(800, 400, 240, 100)
work(grove, {hand="body", tool={kind="filbert", width=5}, pile=treeD, length={8,22}, angle=0.0, angle_jitter=0.6, coverage=4.5, fill=true, clip=grove, mix_jitter=0.4, seed=9602})
print(wait(0))
-- sun-side lit rims on some crowns
local litz = grove * (ellipse(838,452,60,22) + ellipse(880,444,26,14) + ellipse(916,450,22,12))
local litrim = litz - litz:offset(3)
work(grove * litrim:grow(1), {hand="detail", tool={kind="round", width=2}, pile=treeL, coverage=1.6, clip=grove, seed=9603})
print(wait(0))

--@ chunk 134
treeD2 = pile{{"raw umber",2.6},{"smalt",1.4},{"red earth",0.9},{"bone black",0.3},{"Prussian blue",0.25}, medium=0.03}
treeG  = pile{{"raw umber",2.6},{"green earth",0.9},{"smalt",0.8},{"red earth",0.6},{"bone black",0.3}, medium=0.03}
local whole = (shoreMask + grove) * rect(680, 400, 350, 100)
work(whole, {hand="body", tool={kind="filbert", width=6}, pile=treeD2, length={8,26}, angle=0.0, angle_jitter=0.5, coverage=5, fill=true, clip=whole, mix_jitter=0.4, seed=9701})
print(wait(0))
-- green-brown variation in the crowns' upper parts
local tops = grove * above({{800,466},{1020,466}})
work(tops, {hand="body", tool={kind="filbert", width=4}, pile=treeG, length={5,14}, angle=0.0, angle_jitter=0.8, coverage=2.0, fill=false, clip=tops, mix_jitter=0.5, seed=9702,
     load_at=function(x,y) return clamp((470 - y) / 25, 0.1, 0.8) end})
print(wait(0))

--@ chunk 135
HZ = 497
skyClip = rect(-10, -10, 1020, HZ + 10) - farM2
glow_ring = pile{{"chrome yellow",1},{"vermilion",0.32},{"lead white",1.4}, medium=0.5}
glow_yel  = pile{{"chrome yellow",1},{"lead white",3.2},{"vermilion",0.04}, medium=0.3}
glow_core = pile{{"lead white",7},{"chrome yellow",0.18}, medium=0.1}
local function rad(x, y, cx, cy, rx, ry) return math.sqrt(((x - cx) / rx)^2 + ((y - cy) / ry)^2) end
local SX, SY = 300, 476
-- apricot ring
work(ellipse(SX, SY, 300, 96) * skyClip, {hand="glaze", pile=glow_ring, angle=0.0, coverage=1.8, clip=skyClip, edge="lost",
     load_at=function(x,y) return clamp(1.15 - rad(x,y,SX,SY,300,96), 0.0, 1) end, seed=9801})
print(wait(0))
blend(ellipse(SX, SY, 330, 110) * skyClip, {angle=0.0, clip=skyClip})
print(wait(0))
-- yellow halo
work(ellipse(SX, SY, 170, 56) * skyClip, {hand="glaze", pile=glow_yel, angle=0.0, coverage=2.2, clip=skyClip, edge="lost",
     load_at=function(x,y) return clamp(1.2 - rad(x,y,SX,SY,170,56), 0.0, 1) end, seed=9802})
print(wait(0))
blend(ellipse(SX, SY, 190, 66) * skyClip, {angle=0.0, clip=skyClip})
print(wait(0))

--@ chunk 136
print(wait(6*24*60))
print(drying(300,450), drying(600,485), drying(700,480), drying(583,425))

--@ chunk 137
HZ = 497
skyClip = rect(-10, -10, 1020, HZ + 10) - farM2
patchA = mixp(miA, miB, 0.45, 0.03)
patchB = mixp(loA, loB, 0.35, 0.03)
-- 1. the dark drag from the near shore, x 500-700, y 455-497: cover with peach then apricot
local z1 = rect(500, 452, 210, 45) * skyClip
hb(z1, patchA, 16, 10001, 4.0, {60,160}, {clip=skyClip, load_at=function(x,y)
     return clamp((x-500)/50, 0.05, 1) * clamp((y-452)/12, 0.05, 1) * clamp((476-y)/12, 0.05, 1) end})
print(wait(0))
hb(rect(500, 466, 210, 31) * skyClip, patchB, 14, 10002, 4.0, {60,160}, {clip=skyClip, load_at=function(x,y)
     return clamp((x-500)/50, 0.05, 1) * clamp((y-466)/10, 0.05, 1) end})
print(wait(0))
-- 2. the orphan yellow flash near (583,425)
hb(rect(520, 408, 150, 40) * skyClip, mixp(miA, miB, 0.35, 0.03), 14, 10003, 4.0, {40,100}, {clip=skyClip, load_at=function(x,y)
     return clamp(1.1 - math.abs(x-590)/60, 0.05, 1) * clamp(1.1 - math.abs(y-426)/20, 0.05, 1) end})
print(wait(0))
blend(rect(495, 400, 230, 97):blur(10) * skyClip, {angle=0.0, clip=skyClip})
print(wait(0))

--@ chunk 138
HZ = 497
skyClip = rect(-10, -10, 1020, HZ + 10) - farM2
SX, SY = 300, 484
base_apr = pile{{"lead white",5},{"yellow ochre",0.55},{"vermilion",0.25},{"pale smalt",0.3}, medium=0.04}
base_cr  = pile{{"lead white",6},{"yellow ochre",0.42},{"pale smalt",0.55}, medium=0.04}
base_ros = pile{{"lead white",4.4},{"vermilion",0.42},{"red earth",0.15},{"yellow ochre",0.35},{"pale smalt",0.3}, medium=0.04}
local function fadeR(x) return clamp((535 - x) / 110, 0.03, 1) end
local reg = rect(-12, 372, 560, 126) * skyClip
-- cream upper part, apricot lower part, rose at far left
hb(reg, base_cr, 22, 10101, 4.2, {90,220}, {clip=skyClip, load_at=function(x,y) return fadeR(x) * clamp((y-372)/22,0.05,1) * clamp((440-y)/40,0.04,1) end})
print(wait(0))
hb(reg, base_apr, 20, 10102, 4.2, {90,220}, {clip=skyClip, load_at=function(x,y) return fadeR(x) * clamp((y-410)/40,0.05,1) end})
print(wait(0))
hb(rect(-12, 420, 200, 77) * skyClip, base_ros, 18, 10103, 3.6, {80,200}, {clip=skyClip, load_at=function(x,y) return clamp((200-x)/160,0.05,1) * clamp((y-420)/30,0.05,1) end})
print(wait(0))
blend(rect(-12, 366, 570, 132) * skyClip, {angle=math.pi/2, clip=skyClip})
print(wait(0))
blend(rect(-12, 366, 570, 132) * skyClip, {angle=0.0, clip=skyClip})
print(wait(0))

--@ chunk 139
SX, SY = 300, 484
local function rad(x, y, rx, ry) return math.sqrt(((x - SX) / rx)^2 + ((y - SY) / ry)^2) end
hal1 = pile{{"lead white",6},{"chrome yellow",0.75},{"vermilion",0.12},{"yellow ochre",0.15}, medium=0.05}
hal2 = pile{{"lead white",8},{"chrome yellow",0.6},{"vermilion",0.04}, medium=0.05}
hal3 = pile{{"lead white",10},{"chrome yellow",0.28}, medium=0.04}
local H1 = ellipse(SX, SY, 270, 92) * skyClip
local H2 = ellipse(SX, SY, 160, 58) * skyClip
local H3 = ellipse(SX, SY, 80, 28) * skyClip
hb(H1, hal1, 20, 10201, 3.6, {60,170}, {clip=skyClip, edge="lost", load_at=function(x,y) return clamp(1.15 - rad(x,y,270,92), 0.04, 1) end})
print(wait(0))
blend(ellipse(SX, SY, 290, 100) * skyClip, {angle=0.0, clip=skyClip})
hb(H2, hal2, 16, 10202, 3.6, {50,140}, {clip=skyClip, edge="lost", load_at=function(x,y) return clamp(1.15 - rad(x,y,160,58), 0.04, 1) end})
print(wait(0))
blend(ellipse(SX, SY, 170, 64) * skyClip, {angle=0.0, clip=skyClip})
hb(H3, hal3, 12, 10203, 3.6, {30,90}, {clip=skyClip, edge="lost", load_at=function(x,y) return clamp(1.2 - rad(x,y,80,28), 0.04, 1) end})
print(wait(0))
blend(ellipse(SX, SY, 90, 34) * skyClip, {angle=0.0, clip=skyClip})
print(wait(0))

--@ chunk 140
HZ = 497
SX = 300
skyClip2 = rect(-12, -10, 1024, HZ + 10)      -- everything above the horizon line
local function radE(x, y, rx, ry) return math.sqrt(((x - SX) / rx)^2 + ((y - HZ) / ry)^2) end
rings = {
  {rx=350, ry=215, p=pile{{"lead white",5.6},{"yellow ochre",0.55},{"vermilion",0.12},{"pale smalt",0.5}, medium=0.05}, wd=24, cov=3.6},
  {rx=285, ry=160, p=pile{{"lead white",5.6},{"yellow ochre",0.4},{"chrome yellow",0.45},{"vermilion",0.15}, medium=0.05}, wd=22, cov=3.6},
  {rx=215, ry=112, p=pile{{"lead white",5.2},{"chrome yellow",0.9},{"vermilion",0.22}, medium=0.05}, wd=20, cov=3.6},
  {rx=150, ry=74,  p=pile{{"lead white",5.5},{"chrome yellow",1.0},{"vermilion",0.08}, medium=0.05}, wd=16, cov=3.6},
  {rx=96,  ry=46,  p=pile{{"lead white",7},{"chrome yellow",0.8}, medium=0.05}, wd=13, cov=3.6},
  {rx=56,  ry=27,  p=pile{{"lead white",9},{"chrome yellow",0.35}, medium=0.05}, wd=10, cov=3.6},
}
for i, r in ipairs(rings) do
  local m = ellipse(SX, HZ, r.rx, r.ry) * skyClip2
  hb(m, r.p, r.wd, 11000 + i, r.cov, {50, 150}, {clip=skyClip2, edge="lost", load_at=function(x, y) return clamp(1.25 - radE(x, y, r.rx, r.ry), 0.04, 1) end})
  blend(ellipse(SX, HZ, r.rx * 1.06, r.ry * 1.08) * skyClip2, {angle=0.0, clip=skyClip2})
  print(i, wait(0))
end
blend(ellipse(SX, HZ, 330, 200) * skyClip2, {angle=0.02, clip=skyClip2})
print(wait(0))

--@ chunk 141
water = rect(-10, 497, 1020, 260)
wd4 = pile{{"Prussian blue",0.55},{"raw umber",1.8},{"bone black",0.35},{"smalt",1.2},{"lead white",0.5},{"green earth",0.3}, medium=0.10}
wd3 = pile{{"Prussian blue",0.4},{"raw umber",1.3},{"smalt",1.8},{"lead white",1.0},{"cobalt blue",0.4},{"green earth",0.3}, medium=0.10}
-- test on the bottom-left corner: wet-in-wet bands, dark first at the very bottom
hb(rect(-10, 690, 520, 60), wd4, 22, 12001, 3.6, {90,260}, {clip=water, load_at=function(x,y) return clamp((y-690)/26, 0.05, 1) end})
print(wait(0))
hb(rect(-10, 640, 520, 70), wd3, 22, 12002, 3.6, {90,260}, {clip=water, load_at=function(x,y) return clamp((y-640)/26, 0.05, 1) * clamp((712-y)/24, 0.05, 1) end})
print(wait(0))
blend(rect(-10, 630, 520, 120):blur(10), {angle=0.0})
print(wait(0))

--@ chunk 142
water = rect(-10, 497, 1020, 260)
WS = {
 {497, {{"lead white",6},{"chrome yellow",0.55},{"yellow ochre",0.3},{"pale smalt",0.2}},
       {{"lead white",5},{"vermilion",0.35},{"pale smalt",0.9},{"yellow ochre",0.25}}},
 {525, {{"lead white",5.6},{"yellow ochre",0.5},{"chrome yellow",0.2},{"pale smalt",0.6},{"red earth",0.08}},
       {{"lead white",4.6},{"pale smalt",1.4},{"vermilion",0.3},{"red earth",0.15}}},
 {560, {{"lead white",4.4},{"pale smalt",1.8},{"yellow ochre",0.3},{"red earth",0.15},{"cobalt blue",0.2}},
       {{"lead white",3.8},{"pale smalt",2.0},{"red earth",0.3},{"cobalt blue",0.3},{"vermilion",0.1}}},
 {600, {{"lead white",2.6},{"smalt",2.0},{"cobalt blue",0.5},{"raw umber",0.5},{"green earth",0.3}},
       {{"lead white",2.6},{"smalt",2.0},{"cobalt blue",0.5},{"raw umber",0.5},{"green earth",0.3}}},
 {650, {{"lead white",1.4},{"smalt",2.2},{"cobalt blue",0.5},{"raw umber",1.0},{"Prussian blue",0.3},{"green earth",0.3}},
       {{"lead white",1.4},{"smalt",2.2},{"cobalt blue",0.5},{"raw umber",1.0},{"Prussian blue",0.3},{"green earth",0.3}}},
 {700, {{"Prussian blue",0.45},{"raw umber",1.6},{"smalt",1.4},{"lead white",0.7},{"bone black",0.25},{"green earth",0.3}},
       {{"Prussian blue",0.45},{"raw umber",1.6},{"smalt",1.4},{"lead white",0.7},{"bone black",0.25},{"green earth",0.3}}},
 {745, {{"Prussian blue",0.55},{"raw umber",1.9},{"bone black",0.4},{"smalt",1.0},{"lead white",0.4},{"green earth",0.3}},
       {{"Prussian blue",0.55},{"raw umber",1.9},{"bone black",0.4},{"smalt",1.0},{"lead white",0.4},{"green earth",0.3}}},
}
function wstop(y, side)
  for i = 1, #WS - 1 do
    local a, b = WS[i], WS[i+1]
    if y <= b[1] or i == #WS - 1 then
      local t = clamp((y - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[side], b[side], t, 0.10)
    end
  end
end
function wstrip(yc, hh, seed)
  local m = rect(-10, yc - hh, 1020, 2 * hh) * water
  local ramp = function(y) return clamp(1.2 - math.abs(y - yc) / hh, 0.04, 1) end
  local top = (yc - hh <= 497)
  local wd = 16 + (yc - 497) * 0.03
  hb(m, wstop(yc, 2), wd, seed, 3.6, {90,260}, {clip=water, load_at=function(x, y) return ramp(y) * (top and 1 or 1) * clamp((x - 380) / 320, 0.03, 1) end})
  hb(m, wstop(yc, 3), wd, seed + 1, 3.6, {90,260}, {clip=water, load_at=function(x, y) return ramp(y) * clamp((700 - x) / 320, 0.03, 1) end})
end
print(wstop(560, 2))
for i, yc in ipairs({504, 526, 552, 580, 610, 640, 672, 704, 732}) do
  wstrip(yc, 26, 12100 + i * 3)
  print(yc, wait(0))
end

--@ chunk 143
print(drying(300,520), drying(300,700))
local wm = rect(-10, 497, 1020, 260)
blend(wm, {angle=math.pi/2})
print(wait(0))
blend(wm, {angle=0.0})
print(wait(0))
blend(wm, {angle=math.pi/2})
print(wait(0))
blend(wm, {angle=0.0})
print(wait(0))

--@ chunk 144
print(wait(5*24*60))
print(drying(300,520), drying(300,700), drying(300,600))

--@ chunk 145
rip_l1 = pile{{"lead white",4},{"pale smalt",1.5},{"yellow ochre",0.3},{"red earth",0.1}, medium=0.05}
rip_l2 = pile{{"lead white",3.2},{"smalt",1.4},{"cobalt blue",0.5},{"raw umber",0.2}, medium=0.05}
rip_d  = pile{{"Prussian blue",0.5},{"raw umber",1.4},{"smalt",1.0},{"bone black",0.2},{"lead white",0.3}, medium=0.05}
water = rect(-10, 497, 1020, 260)
function ripples(y0, y1, w, L, cov, p, sd, xr)
  local m = rect(-10, y0, 1020, y1 - y0) * water
  work(m, {hand="detail", tool={kind="round", width=w}, pile=p, length=L, angle=0.0, angle_jitter=0.02, coverage=cov, fill=false, clip=false, order="scatter", edge="found",
           pressure={0.5,0.8}, seed=sd, load_at=xr})
end
ripples(500, 530, 1.4, {8,30}, 0.7, rip_l1, 13001)
ripples(500, 530, 1.2, {8,26}, 0.4, rip_d, 13002)
print(wait(0))
ripples(530, 570, 2.0, {12,44}, 0.7, rip_l1, 13003)
ripples(530, 570, 1.8, {12,40}, 0.5, rip_d, 13004)
print(wait(0))

--@ chunk 146
local rz = rect(-10, 497, 1020, 84) * water
blend(rz, {angle=0.0, clip=water})
print(wait(0))
blend(rz, {angle=0.0, clip=water})
print(wait(0))

--@ chunk 147
local nocloud = -cmsil:shrink(1)
local regn = rect(770, 20, 250, 120) * nocloud
-- match the surrounding sky: the same gradient colours as the strips painted earlier, applied in two strips
for i, yc in ipairs({50, 90, 125}) do
  local hh = 30
  local m = rect(770, yc - hh, 250, 2 * hh) * nocloud
  hb(m, skycol2(yc), 16, 14000 + i, 4.2, {40,140}, {clip=nocloud, angle_jitter=0.01, load_at=function(x, y) return clamp(1.2 - math.abs(y - yc) / hh, 0.04, 1) end})
  print(yc, wait(0))
end
blend(regn, {angle=0.0, clip=nocloud})
print(wait(0))
blend(regn, {angle=0.02, clip=nocloud})
print(wait(0))

--@ chunk 148
cn = noise{seed=21, octaves=3, period=38, persistence=0.55}
function cval(x, y)
  return cm:value(x, y) + 0.05 * cn(x, y)
end
function tri(v, c, h) return clamp(1 - math.abs(v - c) / h, 0, 1) end
cloud_tones = {
  {p=cShDeep, c=0.14, h=0.16},
  {p=cSh,     c=0.26, h=0.15},
  {p=cMid,    c=0.40, h=0.19},
  {p=cLit,    c=0.58, h=0.22},
  {p=cHigh,   c=0.84, h=0.30},
}
-- test region: the lower-left lobes
testm = cmsil * rect(700, 250, 170, 140)
for i, t in ipairs(cloud_tones) do
  work(testm, {hand="scumble", tool={kind="filbert", width=9}, pile=t.p, coverage=3.0, fill=false, edge="soft", threshold=0.3, seed=15000 + i,
       load_at=function(x, y) return clamp(tri(cval(x, y), t.c, t.h) * 1.4, 0.0, 1) end})
  print(i, wait(0))
end

--@ chunk 149
cn = noise{seed=21, octaves=3, period=34, persistence=0.6}
function cval(x, y) return cm:value(x, y) + 0.07 * cn(x, y) end
function vmask(t, s)
  return mask(function(x, y)
    if x < 560 or y > 400 or y < 30 then return 0 end
    if cmsil:at(x, y) < 0.5 then return 0 end
    return smoothstep(t - s, t + s, cval(x, y))
  end)
end
function vband(lo, hi, s)
  return mask(function(x, y)
    if x < 560 or y > 400 or y < 30 then return 0 end
    if cmsil:at(x, y) < 0.5 then return 0 end
    local v = cval(x, y)
    return smoothstep(lo - s, lo + s, v) * (1 - smoothstep(hi - s, hi + s, v))
  end)
end
m_all  = vmask(-9, 0.05)
m_deep = vband(-9, 0.19, 0.05)
m_mid  = vmask(0.30, 0.05)
m_lit  = vmask(0.44, 0.05)
m_high = vmask(0.66, 0.05)
print(m_all:area(), m_deep:area(), m_mid:area(), m_lit:area(), m_high:area())
-- A: cover everything (old cut lines, the scumble rectangle) with the shadow tone
work(m_all, {hand="body", tool={kind="filbert", width=18}, pile=cSh, angle=cm_across, length={40,110}, coverage=3.6, fill=true, edge="soft", threshold=0.4, seed=15101})
print(wait(0))
work(m_deep, {hand="body", tool={kind="filbert", width=12}, pile=cShDeep, angle=cm_across, length={20,60}, coverage=2.4, fill=true, edge="soft", threshold=0.4, seed=15102})
print(wait(0))

--@ chunk 150
work(m_mid, {hand="body", tool={kind="filbert", width=14}, pile=cMid, angle=cm_across, length={24,80}, coverage=2.8, fill=true, edge="soft", threshold=0.4, seed=15103})
print(wait(0))
work(m_lit, {hand="body", tool={kind="filbert", width=11}, pile=cLit, angle=cm_across, length={18,56}, coverage=2.6, fill=true, edge="soft", threshold=0.4, seed=15104})
print(wait(0))
work(m_high, {hand="body", tool={kind="filbert", width=8}, pile=cHigh, angle=cm_across, length={12,38}, coverage=2.4, fill=true, edge="soft", threshold=0.4, seed=15105})
print(wait(0))

--@ chunk 151
print(drying(800,200), drying(700,330))
blend(cmsil * rect(560, 30, 460, 370), {angle=cm_across, tool={kind="badger", width=16}})
print(wait(0))

--@ chunk 152
water = rect(-10, 497, 1020, 260)
PX = 300
function phw(y) return 46 + (y - 497) * 0.42 end
local function colf(x, y)   -- 0..1 across the path
  local d = math.abs(x - PX) / phw(y)
  return clamp(1.25 - d, 0, 1)
end
local function lenf(y) return clamp(1.12 - (y - 497) / 250, 0.12, 1) end
pbA = pile{{"lead white",8.5},{"chrome yellow",0.45},{"vermilion",0.04}, medium=0.05}
pbB = pile{{"lead white",6},{"yellow ochre",0.45},{"chrome yellow",0.35},{"vermilion",0.1}, medium=0.05}
-- bloom: warm body under the sun, then the bright core
local col = rect(PX - 260, 497, 520, 250) * water
hb(col, pbB, 12, 16001, 3.4, {60,180}, {clip=water, edge="lost", load_at=function(x,y) return colf(x,y) * lenf(y) * 0.9 end})
print(wait(0))
hb(col, pbA, 10, 16002, 3.4, {40,130}, {clip=water, edge="lost", load_at=function(x,y)
  local d = math.abs(x - PX) / (phw(y) * 0.7)
  return clamp(1.2 - d, 0, 1) * clamp(1.15 - (y - 497) / 190, 0.08, 1) end})
print(wait(0))
blend(col:blur(12), {angle=0.0, clip=water})
print(wait(0))

--@ chunk 153
PX = 300
function sidef(x, y) local d = math.abs(x - PX) / phw(y); return smoothstep(0.85, 1.7, d) end
sidem = mask(function(x, y) if y < 499 then return 0 end return sidef(x, y) end)
D = {
 {497, {{"lead white",4.6},{"pale smalt",1.4},{"yellow ochre",0.2},{"red earth",0.15},{"cobalt blue",0.2}}},
 {545, {{"lead white",3.4},{"pale smalt",1.5},{"cobalt blue",0.4},{"raw umber",0.4},{"red earth",0.15}}},
 {600, {{"lead white",2.0},{"smalt",2.2},{"cobalt blue",0.6},{"raw umber",0.8},{"green earth",0.3}}},
 {655, {{"lead white",1.0},{"smalt",2.2},{"cobalt blue",0.5},{"raw umber",1.1},{"Prussian blue",0.3},{"green earth",0.3}}},
 {705, {{"Prussian blue",0.4},{"raw umber",1.6},{"smalt",1.4},{"lead white",0.4},{"bone black",0.25},{"green earth",0.3}}},
 {745, {{"Prussian blue",0.5},{"raw umber",1.9},{"bone black",0.4},{"smalt",1.0},{"lead white",0.2},{"green earth",0.3}}},
}
function dcol(y)
  for i = 1, #D - 1 do
    local a, b = D[i], D[i+1]
    if y <= b[1] or i == #D - 1 then
      local t = clamp((y - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[2], b[2], t, 0.08)
    end
  end
end
-- strips, each fading in/out vertically; strokes horizontal, coverage high so they hide what's beneath
for i, yc in ipairs({560, 590, 620, 650, 680, 710, 738}) do
  local hh = 24
  local m = rect(-10, yc - hh, 1020, 2 * hh) * water
  hb(m, dcol(yc), 15 + (yc - 560) * 0.03, 17000 + i, 4.4, {80,240}, {clip=water, load_at=function(x, y)
        return clamp(1.25 - math.abs(y - yc) / hh, 0.04, 1) * sidef(x, y) end})
  print(yc, wait(0))
end

--@ chunk 154
print(drying(300,600), drying(300,700), drying(700,700), drying(700,540))
local wm = rect(-10, 500, 1020, 250)
blend(wm, {angle=0.0, clip=water})
print(wait(0))
blend(wm, {angle=0.01, clip=water})
print(wait(0))

--@ chunk 155
print(wait(6*24*60))
print(drying(300,600), drying(300,700), drying(700,700), drying(800,200), drying(300,300))

--@ chunk 156
HZ = 497
skyClip3 = rect(-12, -10, 1024, HZ + 10)
glz_or = pile{{"chrome yellow",1},{"vermilion",0.42},{"lead white",0.35}, medium=0.7}
glz_ap = pile{{"chrome yellow",1},{"vermilion",0.12},{"lead white",0.5}, medium=0.7}
local function core_dip(x) return 1 - 0.75 * math.exp(-((x - 300) / 90)^2) end   -- less orange right at the sun
-- 1. orange skirt hugging the horizon, deepest at the very bottom of the sky, fading upward
work(rect(-12, 400, 1024, 98) * skyClip3, {hand="glaze", pile=glz_or, angle=0.0, coverage=2.2, clip=skyClip3, edge="lost", seed=18001,
     load_at=function(x, y) return clamp((y - 415) / 80, 0.0, 1) ^ 1.4 * core_dip(x) * clamp((900 - x) / 500, 0.15, 1) end})
print(wait(0))
-- 2. broader apricot veil above it
work(rect(-12, 350, 700, 148) * skyClip3, {hand="glaze", pile=glz_ap, angle=0.0, coverage=1.8, clip=skyClip3, edge="lost", seed=18002,
     load_at=function(x, y) return clamp((y - 360) / 110, 0.0, 1) * clamp((680 - x) / 340, 0.0, 1) end})
print(wait(0))

--@ chunk 157
local reg = rect(-12, 340, 1024, 158) * skyClip3
blend(reg, {angle=0.0, clip=skyClip3})
print(wait(0))
blend(reg, {angle=math.pi/2, clip=skyClip3})
print(wait(0))
blend(reg, {angle=0.0, clip=skyClip3})
print(wait(0))

--@ chunk 158
print(wait(8*24*60))
print(drying(300,450), drying(100,470), drying(600,480), drying(1000,470), drying(800,300))

--@ chunk 159
HZ = 497
skyClip3 = rect(-12, -10, 1024, HZ + 10)
nocloud = -cmsil:shrink(2)
skyReg = skyClip3 * nocloud * (-shoreMask:grow(3))
function zoneSlices(y0, y1, A, B, wd, sd, rampTop, rampBot, cov, xt0, xt1)
  local n = 8
  for i = 0, n do
    local cx = 1000 * i / n
    local t = clamp((cx - (xt0 or 350)) / ((xt1 or 850) - (xt0 or 350)), 0, 1)
    local hw = (1000 / n) * 1.2
    local x0 = math.max(cx - hw, -12)
    local m = rect(x0, y0, cx + hw - x0, y1 - y0) * skyReg
    hb(m, mixp(A, B, t, 0.03), wd, sd + i, cov or 4.2, {70,170}, {clip=skyReg, load_at=function(x, y)
        local wx = clamp(1 - math.abs(x - cx) / hw, 0.02, 1)
        local wy = clamp((y - y0) / rampTop, 0.03, 1) * clamp((y1 - y) / rampBot, 0.05, 1)
        return wx * wy end})
  end
end
R4A = {{"lead white",5},{"chrome yellow",0.5},{"vermilion",0.2},{"yellow ochre",0.3}}
R4B = {{"lead white",4.4},{"vermilion",0.5},{"chrome yellow",0.3},{"pale smalt",0.4}}
R3A = {{"lead white",5.6},{"yellow ochre",0.5},{"chrome yellow",0.3},{"vermilion",0.1}}
R3B = {{"lead white",4.6},{"vermilion",0.4},{"pale smalt",0.7},{"yellow ochre",0.2}}
zoneSlices(440, 497, R4A, R4B, 18, 19000, 20, 4, 4.4)
print(wait(0))
zoneSlices(396, 470, R3A, R3B, 20, 19020, 26, 20, 4.4)
print(wait(0))

--@ chunk 160
HZ = 497
skyClip3 = rect(-12, -10, 1024, HZ + 10)
nocloud = -cmsil:shrink(2)
skyReg = skyClip3 * nocloud * (-shoreMask:grow(3))
-- colour stops by y: left (warm, near the sun) and right (pink, far from the sun)
STL = {
 {280, {{"lead white",4.3},{"pale smalt",2.5},{"cobalt blue",0.4},{"yellow ochre",0.35}}},
 {345, {{"lead white",5.4},{"pale smalt",1.3},{"yellow ochre",0.5}}},
 {405, {{"lead white",6},{"yellow ochre",0.5},{"chrome yellow",0.3},{"pale smalt",0.3}}},
 {455, {{"lead white",6},{"chrome yellow",0.6},{"yellow ochre",0.2},{"vermilion",0.1}}},
 {497, {{"lead white",5.5},{"chrome yellow",0.55},{"vermilion",0.25}}},
}
STR = {
 {280, {{"lead white",4.2},{"pale smalt",2.3},{"red earth",0.15},{"cobalt blue",0.3}}},
 {345, {{"lead white",5},{"pale smalt",1.5},{"vermilion",0.15},{"yellow ochre",0.3}}},
 {405, {{"lead white",5},{"vermilion",0.35},{"pale smalt",0.8},{"yellow ochre",0.3}}},
 {455, {{"lead white",4.8},{"vermilion",0.5},{"chrome yellow",0.35},{"pale smalt",0.4}}},
 {497, {{"lead white",4.5},{"vermilion",0.6},{"chrome yellow",0.35},{"red earth",0.1},{"pale smalt",0.3}}},
}
function stopcol(ST, y)
  for i = 1, #ST - 1 do
    local a, b = ST[i], ST[i+1]
    if y <= b[1] or i == #ST - 1 then
      local t = clamp((y - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[2], b[2], t, 0.03)
    end
  end
end
function tx(x) return smoothstep(330, 860, x) end
local centers = {296, 326, 356, 386, 416, 446, 476, 494}
for i, yc in ipairs(centers) do
  local hh = 24
  local m = rect(-12, yc - hh, 1024, 2 * hh) * skyReg
  local rt = function(y) return clamp(1.25 - math.abs(y - yc) / hh, 0.04, 1) end
  if yc >= 486 then rt = function(y) return clamp((y - 470) / 12, 0.04, 1) end end
  hb(m, stopcol(STL, yc), 24, 20000 + i * 3, 4.4, {90,240}, {clip=skyReg, load_at=function(x, y) return rt(y) * (1 - tx(x)) + 0.02 end})
  hb(m, stopcol(STR, yc), 24, 20001 + i * 3, 4.4, {90,240}, {clip=skyReg, load_at=function(x, y) return rt(y) * tx(x) + 0.02 end})
  print(yc, wait(0))
end

--@ chunk 161
local reg = rect(-12, 270, 1024, 228) * skyReg
blend(reg, {angle=math.pi/2, clip=skyReg})
print(wait(0))
blend(reg, {angle=0.0, clip=skyReg})
print(wait(0))
blend(reg, {angle=math.pi/2, clip=skyReg})
print(wait(0))
blend(reg, {angle=0.0, clip=skyReg})
print(wait(0))

--@ chunk 162
-- The vertical blend with clip=skyReg was not confined to its rect: it dragged wet cream up into the blue as flames.
-- Overpaint y 140..300 wet-in-wet with the sky gradient, then blend inside a mask that has a soft ramp.
skyMid = rect(-12, 130, 1024, 175) * skyReg
local ycs = {150, 182, 214, 246, 278, 300}
for i, yc in ipairs(ycs) do
  local hh = 24
  local m = rect(-12, yc - hh, 1024, 2 * hh) * skyReg
  local col
  if yc <= 265 then col = skycol2(yc) else col = mixp(STL[1][2], STR[1][2], 0.3, 0.05) end
  hb(m, col, 26, 21000 + i, 4.6, {150,320}, {clip=skyReg, angle_jitter=0.01, load_at=function(x, y) return clamp(1.25 - math.abs(y - yc) / hh, 0.04, 1) end})
  print(yc, wait(0))
end
blend(skyMid:blur(6), {angle=0.0})
print(wait(0))
blend(skyMid:blur(6), {angle=0.015})
print(wait(0))

--@ chunk 163
HZ = 497
-- sky region: excludes the cloud (feathered), the shore, and stops on the horizon line
cloudSoft = (-(cmsil:shrink(9))):blur(10)          -- 1 in the open sky, ~0.5 at the cloud outline shrunk by 9, fading inside
skyReg2 = rect(-12, -12, 1024, HZ + 12) * cloudSoft * (-shoreMask:grow(2))
SKL = {
 {-20, {{"smalt",3.2},{"cobalt blue",1.4},{"Prussian blue",0.12},{"lead white",0.9},{"raw umber",0.25}}},
 {90,  {{"smalt",2.8},{"cobalt blue",1.3},{"lead white",1.6},{"raw umber",0.22},{"Prussian blue",0.05}}},
 {180, {{"smalt",2.0},{"cobalt blue",0.9},{"lead white",3.0},{"yellow ochre",0.2},{"raw umber",0.12}}},
 {265, {{"pale smalt",2.5},{"lead white",4.3},{"cobalt blue",0.4},{"yellow ochre",0.35},{"red earth",0.05}}},
 {340, {{"lead white",5.4},{"pale smalt",1.3},{"yellow ochre",0.5}}},
 {400, {{"lead white",6},{"yellow ochre",0.5},{"chrome yellow",0.3},{"pale smalt",0.3}}},
 {450, {{"lead white",6},{"chrome yellow",0.6},{"yellow ochre",0.2},{"vermilion",0.1}}},
 {497, {{"lead white",5.5},{"chrome yellow",0.55},{"vermilion",0.25}}},
}
SKR = {
 {-20, SKL[1][2]}, {90, SKL[2][2]}, {180, SKL[3][2]},
 {265, {{"pale smalt",2.3},{"lead white",4.2},{"red earth",0.15},{"cobalt blue",0.3}}},
 {340, {{"lead white",5},{"pale smalt",1.5},{"vermilion",0.15},{"yellow ochre",0.3}}},
 {400, {{"lead white",5},{"vermilion",0.35},{"pale smalt",0.8},{"yellow ochre",0.3}}},
 {450, {{"lead white",4.8},{"vermilion",0.5},{"chrome yellow",0.35},{"pale smalt",0.4}}},
 {497, {{"lead white",4.5},{"vermilion",0.6},{"chrome yellow",0.35},{"red earth",0.1},{"pale smalt",0.3}}},
}
function skyc(ST, y, med)
  for i = 1, #ST - 1 do
    local a, b = ST[i], ST[i+1]
    if y <= b[1] or i == #ST - 1 then
      local t = clamp((y - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[2], b[2], t, med or 0.08)
    end
  end
end
-- upper sky, y = 0..250: one pile per strip (same left and right), strips overlapping generously
local ycs = {0, 36, 72, 108, 144, 180, 216, 252}
for i, yc in ipairs(ycs) do
  local hh = 34
  local m = rect(-12, yc - hh, 1024, 2 * hh) * skyReg2
  hb(m, skyc(SKL, yc, 0.14), 32, 22000 + i, 4.2, {160,340}, {clip=skyReg2, angle_jitter=0.012, load_at=function(x, y)
        local r = clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
        if yc == 0 then r = 1 end
        return r end})
  print(yc, wait(0))
end
local up = rect(-12, -12, 1024, 250) * skyReg2
blend(up, {angle=0.0})
print(wait(0))
blend(up, {angle=0.012})
print(wait(0))

--@ chunk 164
print(drying(300,450), drying(300,300), drying(300,100), drying(700,200), drying(900,250), drying(300,600))
print(wait(0))

--@ chunk 165
HZ = 497
cloudFree = -(cmsil:shrink(1))
skyReg3 = rect(-12, 170, 1024, HZ - 170 + 1) * cloudFree * (-shoreMask:grow(2))
SKL = {
 {170, {{"smalt",2.2},{"cobalt blue",1.0},{"lead white",2.8},{"raw umber",0.12}}},
 {235, {{"pale smalt",2.3},{"cobalt blue",0.5},{"lead white",4.2},{"vermilion",0.05}}},
 {295, {{"pale smalt",1.4},{"lead white",5.4},{"vermilion",0.1},{"yellow ochre",0.08}}},
 {350, {{"lead white",6},{"pale smalt",0.5},{"vermilion",0.16},{"yellow ochre",0.2}}},
 {405, {{"lead white",6},{"yellow ochre",0.35},{"chrome yellow",0.25},{"vermilion",0.12}}},
 {450, {{"lead white",5.8},{"chrome yellow",0.7},{"vermilion",0.22}}},
 {497, {{"lead white",5},{"chrome yellow",0.7},{"vermilion",0.5}}},
}
SKR = {
 {170, SKL[1][2]},
 {235, {{"pale smalt",2.2},{"cobalt blue",0.4},{"lead white",4.2},{"red earth",0.12}}},
 {295, {{"pale smalt",1.4},{"lead white",5},{"red earth",0.15},{"vermilion",0.12}}},
 {350, {{"lead white",5.4},{"vermilion",0.25},{"pale smalt",0.5},{"red earth",0.06}}},
 {405, {{"lead white",5},{"vermilion",0.4},{"pale smalt",0.5},{"yellow ochre",0.2}}},
 {450, {{"lead white",4.8},{"vermilion",0.55},{"chrome yellow",0.3},{"pale smalt",0.3}}},
 {497, {{"lead white",4.5},{"vermilion",0.7},{"chrome yellow",0.3},{"red earth",0.12}}},
}
function tx2(x) return smoothstep(330, 900, x) end
local ycs = {186, 216, 246, 276, 306, 336, 366, 396, 426, 456, 486}
for i, yc in ipairs(ycs) do
  local hh = 26
  local m = rect(-12, yc - hh, 1024, 2 * hh) * skyReg3
  local rt = function(y) return clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1) end
  if yc >= 486 then rt = function(y) return clamp((y - 468) / 14, 0.05, 1) end end
  hb(m, skyc(SKL, yc, 0.1), 26, 23000 + i * 3, 4.4, {120,300}, {clip=skyReg3, angle_jitter=0.012, load_at=function(x, y) return rt(y) * (1 - tx2(x)) + 0.02 end})
  hb(m, skyc(SKR, yc, 0.1), 26, 23001 + i * 3, 4.4, {120,300}, {clip=skyReg3, angle_jitter=0.012, load_at=function(x, y) return rt(y) * tx2(x) + 0.02 end})
  print(yc, wait(0))
end

--@ chunk 166
blend(skyReg3, {angle=math.pi/2})
print(wait(0))
blend(skyReg3, {angle=0.0})
print(wait(0))
local seamTop = rect(-12, 130, 1024, 90) * cloudFree
blend(seamTop, {angle=math.pi/2})
print(wait(0))
blend(seamTop, {angle=0.0})
print(wait(0))

--@ chunk 167
print(drying(300,130), drying(300,220), drying(300,300))
local softA = (rect(-12, 95, 1024, 165) * cloudFree):blur(28)
blend(softA, {angle=math.pi/2})
print(wait(0))
blend(softA, {angle=0.0})
print(wait(0))

--@ chunk 168
HZ = 497
SX = 300
skyAll = rect(-12, -12, 1024, HZ + 12) * cloudFree * (-shoreMask:grow(2))
local function radE(x, y, rx, ry) return math.sqrt(((x - SX) / rx)^2 + ((y - HZ) / ry)^2) end
rings = {
  {rx=380, ry=230, p=pile{{"lead white",5.6},{"yellow ochre",0.5},{"vermilion",0.14},{"pale smalt",0.3}, medium=0.06}, wd=24},
  {rx=300, ry=165, p=pile{{"lead white",5.6},{"yellow ochre",0.35},{"chrome yellow",0.5},{"vermilion",0.14}, medium=0.06}, wd=22},
  {rx=225, ry=115, p=pile{{"lead white",5.2},{"chrome yellow",0.95},{"vermilion",0.2}, medium=0.06}, wd=20},
  {rx=155, ry=76,  p=pile{{"lead white",5.6},{"chrome yellow",1.0},{"vermilion",0.06}, medium=0.06}, wd=16},
  {rx=98,  ry=47,  p=pile{{"lead white",7.5},{"chrome yellow",0.7}, medium=0.06}, wd=13},
  {rx=58,  ry=27,  p=pile{{"lead white",9.5},{"chrome yellow",0.3}, medium=0.05}, wd=10},
}
for i, r in ipairs(rings) do
  local m = ellipse(SX, HZ, r.rx, r.ry) * skyAll
  hb(m, r.p, r.wd, 24000 + i, 3.6, {50, 150}, {clip=skyAll, edge="lost", load_at=function(x, y) return clamp(1.3 - radE(x, y, r.rx, r.ry), 0.04, 1) end})
  local bm = (ellipse(SX, HZ, r.rx * 1.1, r.ry * 1.12) * skyAll):blur(10)
  blend(bm, {angle=0.0})
  print(i, wait(0))
end

--@ chunk 169
print(wait(6*24*60))
print(drying(300,450), drying(300,300), drying(700,470), drying(700,200))

--@ chunk 170
print(shoreMask:area(), grove:area())
silD  = pile{{"bone black",2},{"raw umber",2.2},{"smalt",1.0},{"red earth",0.6},{"lead white",0.5}, medium=0.02}
silD2 = pile{{"raw umber",2.4},{"bone black",1.2},{"Prussian blue",0.3},{"red earth",0.7},{"lead white",0.7}, medium=0.02}
-- a slightly enlarged mask so the old pale sliver at the waterline is covered too
shoreBig = shoreMask:grow(2) * rect(800, 380, 240, 118)
work(shoreBig, {hand="body", tool={kind="filbert", width=6}, pile=silD, length={10,30}, angle=0.0, angle_jitter=0.2, coverage=7, fill=true, clip=shoreBig, mix_jitter=0.2, seed=30001})
print(wait(0))
work(shoreBig, {hand="body", tool={kind="filbert", width=5}, pile=silD2, length={8,22}, angle=0.1, angle_jitter=0.6, coverage=3, fill=true, clip=shoreBig, mix_jitter=0.3, seed=30002})
print(wait(0))

--@ chunk 171
silK = pile{{"bone black",3},{"raw umber",1.2},{"Prussian blue",0.25},{"smalt",0.3}, medium=0.02}
silK2 = pile{{"bone black",2.4},{"raw umber",1.6},{"red earth",0.3},{"smalt",0.5}, medium=0.02}
shoreFull = shoreMask:grow(2) * rect(670, 380, 370, 118)
work(shoreFull, {hand="body", tool={kind="filbert", width=6}, pile=silK, length={10,30}, angle=0.0, angle_jitter=0.2, coverage=8, fill=true, clip=shoreFull, mix_jitter=0.15, seed=30011})
print(wait(0))
work(shoreFull * above({{670,470},{830,462},{1020,460}}), {hand="body", tool={kind="filbert", width=5}, pile=silK2, length={8,22}, angle=0.1, angle_jitter=0.5, coverage=2.4, fill=false, clip=shoreFull, mix_jitter=0.3, seed=30012,
     load_at=function(x,y) return clamp((470 - y) / 14, 0.1, 0.9) end})
print(wait(0))

--@ chunk 172
-- windmill on the low bank at left of the knoll; every part hand-placed
mill_tower = poly({{765.6,484.5},{774.6,484.5},{772.7,456.5},{767.5,456.5}})
mill_cap   = poly({{766.6,457.2},{773.6,457.2},{770.2,452.4}})
work(mill_tower, {hand="detail", tool={kind="round", width=2.2}, pile=silK, coverage=7, clip=mill_tower, seed=31001})
work(mill_cap,   {hand="detail", tool={kind="round", width=1.8}, pile=silK, coverage=7, clip=mill_cap, seed=31002})
-- sail blades: thin quads, one for each arm, each drawn with its own numbers; semi-transparent dark so the sky shows through the lattice
blade1 = poly({{773.5,452},{785.5,437.8},{788.6,440.6},{776.6,454.6}})
blade2 = poly({{775.0,460.9},{786.9,470.0},{784.2,473.4},{772.4,464.3}})
blade3 = poly({{764.8,460.8},{754.9,472.0},{751.9,469.4},{761.8,458.2}})
blade4 = poly({{765.2,450.8},{754.0,440.9},{756.6,437.9},{767.8,447.8}})
sailP = pile{{"bone black",2},{"raw umber",1.4},{"smalt",0.4}, medium=0.35}
for i, b in ipairs({blade1, blade2, blade3, blade4}) do
  work(b, {hand="detail", tool={kind="round", width=1.4}, pile=sailP, coverage=4, clip=b, seed=31010 + i})
end
-- the four stocks (arms) as firm lines from the hub; a pointed round brush
armB = brush{kind="round", width=1.5, point=0.6}
armB:load(silK, 1.0)
local pr = armB:pressure_for(1.3)
print("pressure for 1.3 units:", pr)
armB:stroke({{770.1,456.2},{785.2,436.6}}, {pressure={pr, pr}})
armB:load(silK, 1.0)
armB:stroke({{770.1,456.2},{788.4,471.2}}, {pressure={pr, pr}})
armB:load(silK, 1.0)
armB:stroke({{770.1,456.2},{753.6,474.4}}, {pressure={pr, pr}})
armB:load(silK, 1.0)
armB:stroke({{770.1,456.2},{752.6,440.2}}, {pressure={pr, pr}})
print(wait(0))

--@ chunk 173
HZ = 497
barDark = pile{{"smalt",2.2},{"red earth",0.9},{"raw umber",1.0},{"lead white",1.6},{"cobalt blue",0.4}, medium=0.05}
barDark2 = pile{{"smalt",2.6},{"raw umber",1.2},{"red earth",0.6},{"lead white",1.0},{"Prussian blue",0.15}, medium=0.05}
barRim = pile{{"lead white",3.2},{"vermilion",0.8},{"chrome yellow",0.7},{"red earth",0.2}, medium=0.05}
function mk(pts, seed, lobe, amt) return outline{pts=pts, closed=true, char="soft", lobe=lobe or 30, amount=amt or 0.5, seed=seed}:mask() end
barA = mk({{40,453},{90,448},{150,444},{215,442},{280,445},{340,450},{395,456},{440,462},{472,468},
           {452,471},{400,467},{340,462},{280,459},{215,458},{150,459},{95,462},{45,464}}, 601, 34, 0.45)
work(barA, {hand="body", tool={kind="filbert", width=6}, pile=barDark, length={30,90}, angle=0.0, angle_jitter=0.03, coverage=5, fill=true, edge="soft", seed=32001})
print(wait(0))
-- underside: warm rim, its own drawn strip along the lower edge
rimA = mk({{60,461},{120,459},{190,457},{260,458},{330,461},{400,466},{450,470},
           {420,473},{350,469},{280,465},{210,464},{140,465},{80,466}}, 602, 26, 0.4)
work(rimA, {hand="body", tool={kind="filbert", width=4}, pile=barRim, length={24,70}, angle=0.0, angle_jitter=0.03, coverage=3.2, fill=true, edge="soft", seed=32002})
print(wait(0))

--@ chunk 174
print(drying(250,455))
local bm = (barA + rimA):grow(5)
blend(bm, {angle=0.0, clip=false})
print(wait(0))
blend(bm, {angle=0.02, clip=false})
print(wait(0))

--@ chunk 175
print(wait(6*24*60))
print(drying(250,455), drying(300,300))

--@ chunk 176
barA2 = outline{pts={{8,450},{60,446},{120,442},{190,440},{255,442},{320,447},{380,452},{432,458},{444,461},
                   {415,462},{360,459},{300,456},{240,454},{180,454},{120,456},{60,458},{18,456}}, closed=true, char="firm", lobe=22, amount=0.4, seed=641}:mask()
work(barA2, {hand="body", tool={kind="filbert", width=5}, pile=barDark2, length={30,90}, angle=0.0, angle_jitter=0.02, coverage=6, fill=true, clip=barA2, edge="firm", seed=33001})
print(wait(0))
rimA2 = outline{pts={{50,456},{120,455.5},{200,455},{280,456},{350,459},{410,462.5},{412,464.5},{350,462},{280,459},{200,458},{120,458.5},{55,459}}, closed=true, char="firm", lobe=18, amount=0.3, seed=642}:mask()
work(rimA2, {hand="body", tool={kind="filbert", width=3}, pile=barRim, length={20,60}, angle=0.0, angle_jitter=0.02, coverage=5, fill=true, clip=rimA2, edge="firm", seed=33002})
print(wait(0))

--@ chunk 177
-- glaze test in the top-left corner of the sky
gl_dark = pile{{"Prussian blue",1},{"smalt",1.6},{"raw umber",0.35},{"bone black",0.1}, medium=0.7}
testg = rect(0, 0, 260, 110)
work(testg, {hand="glaze", pile=gl_dark, angle=0.02, coverage=1.5, seed=40001,
     load_at=function(x,y) return clamp(1.1 - y/110, 0.0, 1) * clamp(1.1 - x/260, 0.2, 1) end})
print(wait(0))
print(drying(100, 40))

--@ chunk 178
local bm = rect(-10, -10, 330, 150):blur(20)
blend(bm, {angle=0.0})
print(wait(0))
blend(bm, {angle=math.pi/2})
print(wait(0))
blend(bm, {angle=0.0})
print(wait(0))

--@ chunk 179
-- Sun region rebuild, step 1: opaque ground of glow colour over the smeared bars, feathered into the old sky.
HZ = 497
SX, SY = 305, 480
skyC = rect(-12, -12, 1024, HZ + 12) * cloudFree * (-shoreMask:grow(2))
region = rect(-12, 380, 590, HZ - 380 + 1) * skyC
function ringp(t) -- t: 0 at the sun .. 1 far
  return t
end
gA = pile{{"lead white",5.6},{"yellow ochre",0.45},{"pale smalt",0.5},{"vermilion",0.08}, medium=0.05}   -- cool cream, top
gB = pile{{"lead white",5.8},{"yellow ochre",0.4},{"chrome yellow",0.4},{"vermilion",0.1}, medium=0.05}   -- warm cream
gC = pile{{"lead white",5.4},{"chrome yellow",0.7},{"vermilion",0.22}, medium=0.05}                        -- apricot, at horizon
local rows = {
  {400, gA, 34},
  {430, gB, 30},
  {462, gC, 26},
  {489, gC, 20},
}
for i, r in ipairs(rows) do
  local yc, p, hh = r[1], r[2], r[3]
  local m = rect(-12, yc - hh, 590, 2 * hh) * region
  hb(m, p, 20, 41000 + i, 4.6, {80,200}, {clip=skyC, load_at=function(x, y)
      local ry = clamp(1.3 - math.abs(y - yc) / hh, 0.04, 1)
      if yc >= 489 then ry = clamp((y - 470) / 14, 0.04, 1) end
      local rx = clamp((575 - x) / 90, 0.03, 1)
      return ry * rx end})
  print(yc, wait(0))
end
local bm = region:blur(10)
blend(bm, {angle=0.0, clip=skyC})
print(wait(0))
blend(bm, {angle=0.01, clip=skyC})
print(wait(0))

--@ chunk 180
-- Top-left: repaint the glaze test out of the sky with the gradient colours, strips overlapping, then blend wet.
tl = rect(-14, -14, 560, 190)
local ycs = {0, 30, 60, 90, 120, 150}
for i, yc in ipairs(ycs) do
  local hh = 30
  local m = rect(-14, yc - hh, 560, 2 * hh)
  hb(m, skycol2(yc), 28, 42000 + i, 4.6, {120,300}, {angle_jitter=0.012, load_at=function(x, y)
      local r = clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
      if yc == 0 then r = 1 end
      return r * clamp((540 - x) / 120, 0.03, 1) end})
  print(yc, wait(0))
end
local bm = tl:blur(22)
blend(bm, {angle=0.0})
print(wait(0))
blend(bm, {angle=0.012})
print(wait(0))

--@ chunk 181
-- Full repaint of the upper sky (cloud included), wet, in one session. Horizontal blending only (vertical blends left curtains).
SKN = {
 {0,   {{"smalt",3.0},{"cobalt blue",1.6},{"Prussian blue",0.18},{"lead white",1.0},{"raw umber",0.25}}},
 {70,  {{"smalt",2.8},{"cobalt blue",1.6},{"lead white",1.6},{"Prussian blue",0.06},{"raw umber",0.2}}},
 {140, {{"smalt",2.1},{"cobalt blue",1.2},{"lead white",2.6},{"raw umber",0.12}}},
 {205, {{"pale smalt",2.2},{"cobalt blue",0.7},{"lead white",3.8},{"green earth",0.15}}},
 {268, {{"pale smalt",1.6},{"lead white",4.9},{"cobalt blue",0.25},{"green earth",0.3},{"yellow ochre",0.15}}},
 {326, {{"lead white",5.6},{"pale smalt",0.7},{"green earth",0.3},{"yellow ochre",0.35}}},
 {385, {{"lead white",6},{"yellow ochre",0.5},{"chrome yellow",0.2},{"pale smalt",0.15}}},
}
SKNR = {   -- right-hand variant below y=200: pinker toward the horizon
 {205, {{"pale smalt",2.1},{"cobalt blue",0.7},{"lead white",3.8},{"red earth",0.06}}},
 {268, {{"pale smalt",1.5},{"lead white",4.8},{"cobalt blue",0.2},{"red earth",0.12},{"vermilion",0.06}}},
 {326, {{"lead white",5.3},{"pale smalt",0.8},{"vermilion",0.2},{"yellow ochre",0.2},{"red earth",0.08}}},
 {385, {{"lead white",5.2},{"vermilion",0.36},{"pale smalt",0.6},{"yellow ochre",0.2}}},
}
function skyn(ST, y, med)
  for i = 1, #ST - 1 do
    local a, b = ST[i], ST[i+1]
    if y <= b[1] or i == #ST - 1 then
      local t = clamp((y - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[2], b[2], t, med or 0.1)
    end
  end
end
-- 1. opaque cover over the old cloud, one coat of pale blue-grey, so ghost lumps can't show through
cover = pile{{"lead white",3.2},{"smalt",1.2},{"cobalt blue",0.9},{"raw umber",0.15}, medium=0.05}
cloudArea = (cmsil:grow(6)) * rect(540, 20, 500, 400)
work(cloudArea, {hand="broad", tool={kind="flat", width=30}, pile=cover, length={100,260}, angle=0.02, angle_jitter=0.02, coverage=4.5, fill=true, edge="soft", seed=43001})
print(wait(0))
-- 2. strips over the whole upper sky
local ycs = {0, 34, 68, 102, 136, 170, 204, 238, 272, 306, 340, 374}
for i, yc in ipairs(ycs) do
  local hh = 36
  local m = rect(-14, yc - hh, 1028, 2 * hh)
  local rt = function(y)
    local r = clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
    if yc == 0 then r = 1 end
    return r
  end
  if yc <= 170 then
    hb(m, skyn(SKN, yc, 0.12), 28, 43100 + i * 2, 4.4, {140,320}, {angle_jitter=0.012, load_at=function(x, y) return rt(y) end})
  else
    local tx = function(x) return smoothstep(350, 850, x) end
    hb(m, skyn(SKN, yc, 0.12), 28, 43100 + i * 2, 4.4, {140,320}, {angle_jitter=0.012, load_at=function(x, y) return rt(y) * (1 - tx(x)) + 0.03 end})
    hb(m, skyn(SKNR, yc, 0.12), 28, 43101 + i * 2, 4.4, {140,320}, {angle_jitter=0.012, load_at=function(x, y) return rt(y) * tx(x) + 0.03 end})
  end
  print(yc, wait(0))
end

--@ chunk 182
-- fix the leftover cover-coat line at the old cloud base (y~405-430, x 560-1000) with the horizon peach/apricot, then blend the whole sky horizontally
HZ = 497
skyBig = rect(-14, -14, 1028, 452):blur(18)
peachR  = mixp(miA, miB, 0.85, 0.05)
apr     = mixp(loA, loB, 0.85, 0.05)
peachL  = mixp(miA, miB, 0.35, 0.05)
strip = rect(540, 396, 480, 50) - shoreMask:grow(3)
hb(strip, peachR, 20, 44001, 4.4, {80,200}, {load_at=function(x, y) return clamp((x-540)/60, 0.05, 1) * clamp(1.3 - math.abs(y - 420)/26, 0.05, 1) end})
print(wait(0))
hb(rect(540, 425, 480, 55) - shoreMask:grow(3), apr, 18, 44002, 4.4, {80,200}, {load_at=function(x, y) return clamp((x-540)/60, 0.05, 1) * clamp(1.3 - math.abs(y - 452)/26, 0.05, 1) end})
print(wait(0))
blend(skyBig, {angle=0.0})
print(wait(0))
blend(skyBig, {angle=0.012})
print(wait(0))
blend(skyBig, {angle=-0.012})
print(wait(0))

--@ chunk 183
print(wait(6*24*60))
print(drying(300,100), drying(300,300), drying(700,430), drying(300,480))

--@ chunk 184
HZ = 497
skyOnly = rect(-14, -14, 1028, HZ + 14)
peachR = mixp(miA, miB, 0.85, 0.05)
apr    = mixp(loA, loB, 0.85, 0.05)
zone = rect(660, 408, 370, HZ - 408) * skyOnly
hb(rect(660, 408, 370, 56) * skyOnly, peachR, 18, 45001, 5.0, {60,160}, {clip=skyOnly, load_at=function(x, y) return clamp((x-660)/50, 0.05, 1) * clamp((y-408)/16, 0.05, 1) * clamp((470-y)/16, 0.1, 1) end})
print(wait(0))
hb(rect(660, 440, 370, HZ - 440) * skyOnly, apr, 16, 45002, 5.0, {60,160}, {clip=skyOnly, load_at=function(x, y) return clamp((x-660)/50, 0.05, 1) * clamp((y-440)/14, 0.05, 1) end})
print(wait(0))
blend(zone:blur(6), {angle=0.0, clip=skyOnly})
print(wait(0))

--@ chunk 185
print(wait(7*24*60))
print(drying(700,430), drying(300,100), drying(300,300), drying(800,470))

--@ chunk 186
HZ = 497
skyOnly = rect(-14, -14, 1028, HZ + 14)
gA = pile{{"lead white",5.6},{"yellow ochre",0.45},{"pale smalt",0.5},{"vermilion",0.08}, medium=0.05}
gB = pile{{"lead white",5.8},{"yellow ochre",0.4},{"chrome yellow",0.4},{"vermilion",0.1}, medium=0.05}
gC = pile{{"lead white",5.4},{"chrome yellow",0.7},{"vermilion",0.22}, medium=0.05}
local rows = {{400, gA, 30}, {432, gA, 30}, {462, gB, 28}, {488, gC, 20}}
for i, r in ipairs(rows) do
  local yc, p, hh = r[1], r[2], r[3]
  local m = rect(590, yc - hh, 260, 2 * hh) * skyOnly
  hb(m, p, 20, 46000 + i, 4.4, {60,150}, {clip=skyOnly, load_at=function(x, y)
      local ry = clamp(1.3 - math.abs(y - yc) / hh, 0.04, 1)
      if yc >= 488 then ry = clamp((y - 470) / 14, 0.04, 1) end
      local rx = clamp((x - 590) / 40, 0.03, 1) * clamp((840 - x) / 170, 0.03, 1)
      return ry * rx end})
  print(yc, wait(0))
end
local bm = rect(570, 372, 300, HZ - 372):blur(14) * skyOnly
blend(bm, {angle=0.0, clip=skyOnly})
print(wait(0))
blend(bm, {angle=0.01, clip=skyOnly})
print(wait(0))

--@ chunk 187
print(wait(7*24*60))
print(drying(700,430), drying(800,470), drying(620,460))

--@ chunk 188
HZ = 497
silK  = pile{{"bone black",3},{"raw umber",1.2},{"Prussian blue",0.25},{"smalt",0.3}, medium=0.02}
silK2 = pile{{"bone black",2.4},{"raw umber",1.6},{"red earth",0.35},{"smalt",0.5}, medium=0.02}
silW  = pile{{"raw umber",2.4},{"red earth",1.2},{"bone black",1.0},{"lead white",0.5}, medium=0.02}
sky_line = {
  {668,HZ+2},{676,HZ},{692,HZ-3},{716,HZ-6},{742,HZ-9},{765,HZ-12},{790,HZ-15},{812,HZ-18},{824,HZ-22},
  {830,HZ-30},{836,HZ-38},{842,HZ-44},{846,HZ-40},{851,HZ-49},{857,HZ-56},{862,HZ-60},{867,HZ-65},{872,HZ-68},{877,HZ-64},{882,HZ-58},{887,HZ-54},{891,HZ-48},
  {896,HZ-50},{901,HZ-44},{906,HZ-41},{912,HZ-45},{918,HZ-49},{923,HZ-45},{929,HZ-40},{936,HZ-36},{944,HZ-33},{952,HZ-30},{960,HZ-29},{970,HZ-27},{985,HZ-24},{1000,HZ-22},{1024,HZ-20},
  {1024,HZ+2}
}
shoreO2 = outline{pts=sky_line, closed=true, char="firm", amount=0.5, lobe=6, seed=771}
shoreM2 = shoreO2:mask()
print(shoreM2:area())
work(shoreM2, {hand="body", tool={kind="filbert", width=6}, pile=silK, length={10,30}, angle=0.0, angle_jitter=0.25, coverage=7, fill=true, clip=shoreM2, mix_jitter=0.15, seed=50001})
print(wait(0))
work(shoreM2 * above({{660,HZ-6},{830,HZ-20},{1024,HZ-14}}), {hand="body", tool={kind="filbert", width=5}, pile=silK2, length={8,22}, angle=0.1, angle_jitter=0.6, coverage=2.2, fill=false, clip=shoreM2, mix_jitter=0.3, seed=50002,
     load_at=function(x,y) return clamp((470 - y) / 18, 0.1, 0.9) end})
print(wait(0))

--@ chunk 189
HZ = 497
skyOnly = rect(-14, -14, 1028, HZ + 14)
peachR = mixp(miA, miB, 0.85, 0.05)
apr    = mixp(loA, loB, 0.85, 0.05)
gB = pile{{"lead white",5.8},{"yellow ochre",0.4},{"chrome yellow",0.4},{"vermilion",0.1}, medium=0.05}
gC = pile{{"lead white",5.4},{"chrome yellow",0.7},{"vermilion",0.22}, medium=0.05}
-- cover the top of the hill (y < 476) with the local sky; the low bank below stays
cover_z = rect(650, 420, 380, 56) * skyOnly
local n = 5
for i = 0, n do
  local cx = 650 + (1030 - 650) * i / n
  local t = i / n
  local hw = (380 / n) * 1.2
  local x0 = math.max(cx - hw, 640)
  local pA = mixp({{"lead white",5.8},{"yellow ochre",0.4},{"chrome yellow",0.4},{"vermilion",0.1}}, {{"lead white",4.4},{"vermilion",0.5},{"chrome yellow",0.3},{"pale smalt",0.4}}, smoothstep(0.0, 0.8, t), 0.05)
  local pB = mixp({{"lead white",5.4},{"chrome yellow",0.7},{"vermilion",0.22}}, {{"lead white",4.2},{"vermilion",0.7},{"chrome yellow",0.35},{"red earth",0.15},{"pale smalt",0.3}}, smoothstep(0.0, 0.8, t), 0.05)
  hb(rect(x0, 420, cx + hw - x0, 56) * skyOnly, pA, 16, 51000 + i * 2, 6.0, {50,120}, {clip=skyOnly, load_at=function(x, y)
      local wx = clamp(1 - math.abs(x - cx) / hw, 0.03, 1)
      return wx * clamp((y - 420) / 16, 0.05, 1) * clamp((478 - y) / 10, 0.1, 1) end})
  hb(rect(x0, 445, cx + hw - x0, 33) * skyOnly, pB, 14, 51001 + i * 2, 6.0, {50,120}, {clip=skyOnly, load_at=function(x, y)
      local wx = clamp(1 - math.abs(x - cx) / hw, 0.03, 1)
      return wx * clamp((y - 445) / 12, 0.05, 1) * clamp((478 - y) / 8, 0.1, 1) end})
  print(i, wait(0))
end
blend(rect(640, 410, 390, 70) * skyOnly, {angle=0.0, clip=skyOnly})
print(wait(0))

--@ chunk 190
print(wait(10*24*60))
print(drying(800,480), drying(870,440), drying(700,490), drying(800,450))

--@ chunk 191
HZ = 497
skyOnly = rect(-14, -14, 1028, HZ + 14)
function pileT(t, A, B, med) return mixp(A, B, t, med or 0.05) end
cA_L = {{"lead white",5.6},{"yellow ochre",0.45},{"pale smalt",0.5},{"vermilion",0.08}}
cA_R = {{"lead white",5.2},{"vermilion",0.3},{"pale smalt",0.6},{"yellow ochre",0.3}}
cB_L = {{"lead white",5.8},{"yellow ochre",0.4},{"chrome yellow",0.4},{"vermilion",0.1}}
cB_R = {{"lead white",4.4},{"vermilion",0.5},{"chrome yellow",0.3},{"pale smalt",0.4}}
cC_L = {{"lead white",5.4},{"chrome yellow",0.7},{"vermilion",0.22}}
cC_R = {{"lead white",4.2},{"vermilion",0.7},{"chrome yellow",0.35},{"red earth",0.15},{"pale smalt",0.3}}
local x0s, x1s = 640, 1030
local n = 6
local rowsdef = {
  {yc=410, hh=30, L=cA_L, R=cA_R},
  {yc=440, hh=30, L=cA_L, R=cB_R},
  {yc=466, hh=26, L=cB_L, R=cB_R},
  {yc=490, hh=18, L=cC_L, R=cC_R},
}
for ri, r in ipairs(rowsdef) do
  for i = 0, n do
    local cx = x0s + (x1s - x0s) * i / n
    local t = smoothstep(0.0, 0.9, i / n)
    local hw = ((x1s - x0s) / n) * 1.2
    local xa = math.max(cx - hw, x0s - 10)
    local m = rect(xa, r.yc - r.hh, cx + hw - xa, 2 * r.hh) * skyOnly
    hb(m, pileT(t, r.L, r.R, 0.04), 16, 52000 + ri * 20 + i, 5.5, {50,120}, {clip=skyOnly, load_at=function(x, y)
        local wx = clamp(1 - math.abs(x - cx) / hw, 0.03, 1)
        local ry = clamp(1.3 - math.abs(y - r.yc) / r.hh, 0.05, 1)
        if r.yc >= 490 then ry = clamp((y - 476) / 10, 0.05, 1) end
        if i == 0 then wx = wx * clamp((x - x0s + 10) / 50, 0.02, 1) end
        return wx * ry end})
  end
  print(ri, wait(0))
end
blend(rect(630, 385, 400, 112):blur(12) * skyOnly, {angle=0.0, clip=skyOnly})
print(wait(0))
-- water strip at the horizon to bury the row of dark beads
water = rect(-10, 497, 1020, 260)
hb(rect(650, 495, 380, 14) * water, mixp(wTA, wTB, 0.9, 0.04), 8, 52999, 6.0, {60,160}, {clip=water, edge="found", load_at=function(x, y) return clamp((x-650)/40, 0.05, 1) end})
print(wait(0))

--@ chunk 192
water = rect(-10, 497, 1020, 260)
hzA = mixp(miB, wUB, 0.55, 0.05)      -- reflected peach, greyed by the water
hzB = mixp(wUB, wUA, 0.4, 0.05)       -- lilac-grey water just under it
print(hzA); print(hzB)
local x0s, x1s = 640, 1030
local n = 6
for i = 0, n do
  local cx = x0s + (x1s - x0s) * i / n
  local hw = ((x1s - x0s) / n) * 1.25
  local xa = math.max(cx - hw, x0s - 10)
  local m = rect(xa, 497, cx + hw - xa, 50) * water
  hb(m, hzA, 8, 53000 + i, 5.0, {60,160}, {clip=water, edge="found", load_at=function(x, y)
      local wx = clamp(1 - math.abs(x - cx) / hw, 0.03, 1)
      if i == 0 then wx = wx * clamp((x - x0s + 10) / 40, 0.02, 1) end
      return wx * clamp(1.15 - (y - 497) / 22, 0.05, 1) end})
  hb(m, hzB, 12, 53020 + i, 5.0, {60,160}, {clip=water, edge="found", load_at=function(x, y)
      local wx = clamp(1 - math.abs(x - cx) / hw, 0.03, 1)
      if i == 0 then wx = wx * clamp((x - x0s + 10) / 40, 0.02, 1) end
      return wx * clamp((y - 505) / 14, 0.05, 1) * clamp((548 - y) / 24, 0.05, 1) end})
end
print(wait(0))
blend(rect(620, 497, 410, 60):blur(8) * water, {angle=0.0, clip=water})
print(wait(0))
blend(rect(620, 497, 410, 60):blur(8) * water, {angle=0.008, clip=water})
print(wait(0))

--@ chunk 193
-- Cloud C1 (upper right): base mass, drawn outline, load fades toward the edge so it melts into the sky
C1o = outline{pts={{540,222},{600,212},{660,216},{730,224},{800,226},{870,224},{940,218},{1030,210},
                   {1030,66},{990,58},{940,66},{895,76},{850,62},{805,74},{765,92},{725,108},{690,128},{655,146},{620,165},{585,186},{555,205}},
              closed=true, char="soft", lobe=46, amount=0.7, seed=811}
C1 = C1o:mask()
C1soft = C1:blur(14)
print("C1 area", C1:area())
cl_base = pile{{"lead white",3.6},{"smalt",1.4},{"cobalt blue",0.4},{"red earth",0.6},{"raw umber",0.28}, medium=0.08}
work(C1, {hand="body", tool={kind="filbert", width=22}, pile=cl_base, angle=0.03, angle_jitter=0.06, length={60,170}, coverage=3.0, fill=true, clip=C1,
     load_at=function(x, y) return clamp(C1soft:at(x, y) * 1.25 - 0.1, 0.03, 1) end, seed=60001})
print(wait(0))

--@ chunk 194
print(drying(900,140))
-- shading while wet: dark base band along the underside, its own drawn strip
cl_under = pile{{"lead white",2.6},{"smalt",1.6},{"cobalt blue",0.4},{"red earth",0.8},{"raw umber",0.4}, medium=0.08}
underO = outline{pts={{560,208},{620,208},{690,214},{760,214},{840,216},{920,212},{1030,205},{1030,170},{960,182},{880,190},{800,186},{730,180},{670,180},{610,188},{575,196}},
                 closed=true, char="soft", lobe=40, amount=0.6, seed=821}
under1 = underO:mask() * C1
work(under1, {hand="body", tool={kind="filbert", width=16}, pile=cl_under, angle=0.02, angle_jitter=0.05, length={50,140}, coverage=2.6, fill=true, clip=C1,
     load_at=function(x, y) return clamp((y - 170) / 40, 0.05, 1) end, seed=60002})
-- lit tone along the sun-facing (left / upper-left) edge, thick short strokes
cl_lit1 = pile{{"lead white",6.2},{"yellow ochre",0.45},{"chrome yellow",0.25},{"vermilion",0.22}, medium=0.06}
litO = outline{pts={{560,206},{580,190},{610,172},{642,154},{676,136},{708,118},{742,100},{775,86},{810,80},{850,72},{884,80},{900,96},{870,90},{838,92},{806,100},{772,116},{738,134},{705,152},{672,170},{640,186},{610,200},{585,210}},
               closed=true, char="soft", lobe=30, amount=0.6, seed=822}
lit1 = litO:mask() * C1
work(lit1, {hand="body", tool={kind="filbert", width=10}, pile=cl_lit1, angle=-0.5, angle_jitter=0.3, length={14,40}, coverage=2.6, fill=true, clip=C1, seed=60003})
print(wait(0))

--@ chunk 195
local bm = C1:grow(8):blur(8)
blend(bm, {angle=0.03})
print(wait(0))
blend(bm, {angle=-0.02})
print(wait(0))

--@ chunk 196
print(wait(10*24*60))
print(drying(900,140), drying(700,530), drying(300,300))

--@ chunk 197
base_pts = {{535,224},{560,238},{600,247},{650,253},{700,255},{750,259},{800,269},{835,283},{870,278},{910,264},{960,256},{1030,251}}
skyUnder = pile{{"lead white",6},{"yellow ochre",0.4},{"pale smalt",0.9},{"vermilion",0.1}, medium=0.05}
underReg = below(base_pts) * rect(535, 235, 500, 70)
work(underReg, {hand="broad", tool={kind="flat", width=14}, pile=skyUnder, length={60,160}, angle=0.02, angle_jitter=0.03, coverage=3.6, fill=true, edge="soft", seed=61001,
     load_at=function(x, y) return clamp(1.1 - (y - 235) / 60, 0.1, 1) end})
print(wait(0))
-- dark base band, hand-drawn along the same line
cl_base2 = pile{{"lead white",2.4},{"smalt",1.5},{"cobalt blue",0.3},{"red earth",0.8},{"raw umber",0.4}, medium=0.06}
baseRib = ribbon(base_pts, 14) * rect(535, 200, 500, 100)
work(baseRib, {hand="body", tool={kind="filbert", width=10}, pile=cl_base2, length={40,110}, angle=0.02, angle_jitter=0.04, coverage=2.6, fill=true, edge="soft", seed=61002})
print(wait(0))

--@ chunk 198
-- Soft, glowing cloud base: overlapping horizontal strips graded from mauve to rose to sky peach. Horizontal strokes, wet together, then a horizontal blend.
BS = {
 {190, {{"lead white",2.6},{"smalt",1.5},{"cobalt blue",0.3},{"red earth",0.8},{"raw umber",0.4}}},
 {232, {{"lead white",3.2},{"smalt",1.0},{"red earth",0.9},{"raw umber",0.25},{"vermilion",0.15}}},
 {262, {{"lead white",4.2},{"vermilion",0.5},{"red earth",0.3},{"pale smalt",0.6},{"yellow ochre",0.15}}},
 {292, {{"lead white",5.6},{"yellow ochre",0.4},{"pale smalt",0.8},{"vermilion",0.14}}},
}
function bcol(y, med)
  for i = 1, #BS - 1 do
    local a, b = BS[i], BS[i+1]
    if y <= b[1] or i == #BS - 1 then
      local t = clamp((y - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[2], b[2], t, med or 0.06)
    end
  end
end
local x0s, x1s = 540, 1030
local baseMask = rect(x0s - 10, 190, x1s - x0s + 30, 120)
for i, yc in ipairs({206, 232, 258, 284}) do
  local hh = 24
  local m = rect(x0s - 10, yc - hh, x1s - x0s + 30, 2 * hh)
  hb(m, bcol(yc), 16, 62000 + i, 4.4, {80,200}, {angle_jitter=0.02, load_at=function(x, y)
      local wx = clamp((x - 575) / 110, 0.0, 1)
      local wy = clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
      return wx * wy end})
  print(yc, wait(0))
end
blend(baseMask:blur(12) * rect(590, 190, 450, 120), {angle=0.0})
print(wait(0))
blend(baseMask:blur(12) * rect(590, 190, 450, 120), {angle=0.01})
print(wait(0))

--@ chunk 199
print(wait(9*24*60))
print(drying(800,230), drying(700,530))

--@ chunk 200
-- Re-lay the sky over the pink slab (all four edges faded), on dry paint
function tx3(x) return smoothstep(350, 850, x) end
local x0, x1 = 500, 1040
local strips = {188, 214, 240, 266, 292, 314}
for i, yc in ipairs(strips) do
  local hh = 27
  local m = rect(x0, yc - hh, x1 - x0, 2 * hh)
  local ry = function(y)
    local r = clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
    return r
  end
  local rx = function(x) return clamp((x - 505) / 55, 0.03, 1) end
  hb(m, skyn(SKN, yc, 0.1), 26, 63000 + i * 2, 4.4, {120,300}, {angle_jitter=0.012, load_at=function(x, y) return ry(y) * rx(x) * (1 - tx3(x)) + 0.02 end})
  hb(m, skyn(SKNR, yc, 0.1), 26, 63001 + i * 2, 4.4, {120,300}, {angle_jitter=0.012, load_at=function(x, y) return ry(y) * rx(x) * tx3(x) + 0.02 end})
  print(yc, wait(0))
end
local bm = rect(490, 170, 550, 160):blur(20)
blend(bm, {angle=0.0})
print(wait(0))
blend(bm, {angle=0.008})
print(wait(0))

--@ chunk 201
C1v2o = outline{pts={{520,238},{548,222},{575,205},{600,190},{632,168},{668,146},{700,128},{730,104},{762,84},{798,70},{830,56},{866,46},{900,50},{930,42},{965,36},{1000,34},{1040,36},
   {1040,262},{1000,268},{965,273},{930,266},{895,279},{860,291},{825,284},{790,268},{755,262},{715,258},{675,252},{635,246},{595,242},{555,240}},
   closed=true, char="soft", lobe=44, amount=0.7, seed=911}
C1v2 = C1v2o:mask()
lowerC = C1v2 * rect(500, 160, 560, 150)
print(lowerC:area())
cl_mauve = pile{{"lead white",3.6},{"smalt",1.4},{"cobalt blue",0.4},{"red earth",0.6},{"raw umber",0.28}, medium=0.08}
cl_rose  = pile{{"lead white",3.4},{"red earth",0.7},{"vermilion",0.35},{"smalt",0.6},{"yellow ochre",0.15}, medium=0.08}
cl_peach = pile{{"lead white",5},{"vermilion",0.4},{"yellow ochre",0.4},{"pale smalt",0.5}, medium=0.08}
local function cfade(x) return clamp((x - 520) / 70, 0.04, 1) end
work(lowerC, {hand="body", tool={kind="filbert", width=20}, pile=cl_mauve, angle=0.03, angle_jitter=0.05, length={60,160}, coverage=3.2, fill=true, edge="soft", seed=64001,
     load_at=function(x, y) return clamp((y - 158) / 26, 0.03, 1) * clamp((262 - y) / 50, 0.2, 1) * cfade(x) end})
print(wait(0))
work(lowerC * rect(500, 205, 560, 100), {hand="body", tool={kind="filbert", width=16}, pile=cl_rose, angle=0.02, angle_jitter=0.05, length={50,140}, coverage=3.0, fill=true, edge="soft", seed=64002,
     load_at=function(x, y) return clamp((y - 205) / 34, 0.03, 1) * clamp((300 - y) / 40, 0.2, 1) * cfade(x) end})
print(wait(0))
work(lowerC * rect(500, 240, 560, 60), {hand="body", tool={kind="filbert", width=12}, pile=cl_peach, angle=0.02, angle_jitter=0.05, length={40,120}, coverage=2.6, fill=true, edge="soft", seed=64003,
     load_at=function(x, y) return clamp((y - 240) / 26, 0.03, 1) * cfade(x) end})
print(wait(0))
local bm = lowerC:grow(4):blur(8)
blend(bm, {angle=0.02})
print(wait(0))
blend(bm, {angle=-0.01})
print(wait(0))

--@ chunk 202
print(drying(700,165), drying(560,200), drying(700,120))
local seam = rect(470, 135, 570, 60):blur(10)
blend(seam, {angle=0.0})
print(wait(0))
blend(seam, {angle=0.02})
print(wait(0))

--@ chunk 203
water = rect(-10, 497, 1020, 260)
WS2 = {
 {497, {{"lead white",6},{"chrome yellow",0.55},{"yellow ochre",0.25},{"pale smalt",0.3}}},
 {512, {{"lead white",6},{"yellow ochre",0.5},{"pale smalt",0.7},{"chrome yellow",0.1}}},
 {535, {{"lead white",5.2},{"pale smalt",1.5},{"yellow ochre",0.25},{"red earth",0.08}}},
 {570, {{"lead white",3.8},{"pale smalt",2.0},{"cobalt blue",0.4},{"red earth",0.12}}},
 {610, {{"lead white",2.4},{"smalt",2.0},{"cobalt blue",0.6},{"raw umber",0.4},{"green earth",0.25}}},
 {655, {{"lead white",1.3},{"smalt",2.2},{"cobalt blue",0.5},{"raw umber",0.9},{"Prussian blue",0.25},{"green earth",0.3}}},
 {700, {{"lead white",0.6},{"smalt",1.6},{"Prussian blue",0.4},{"raw umber",1.5},{"green earth",0.3},{"bone black",0.25}}},
 {745, {{"Prussian blue",0.5},{"raw umber",1.9},{"bone black",0.5},{"smalt",1.0},{"green earth",0.3},{"lead white",0.1}}},
}
function wcol2(y, med)
  for i = 1, #WS2 - 1 do
    local a, b = WS2[i], WS2[i+1]
    if y <= b[1] or i == #WS2 - 1 then
      local t = clamp((y - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[2], b[2], t, med or 0.06)
    end
  end
end
local n = 0
for yc = 500, 748, 15 do
  n = n + 1
  local hh = 17
  local m = rect(-10, yc - hh, 1020, 2 * hh) * water
  local wd = 12 + (yc - 497) * 0.02
  hb(m, wcol2(yc), wd, 70000 + n * 3, 3.8, {90,260}, {clip=water, edge="found", angle_jitter=0.012, mix_jitter=0.3,
      load_at=function(x, y) return clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1) end})
  if n % 3 == 0 then print(yc, wait(0)) end
end
print(wait(0))

--@ chunk 204
print(drying(500,520), drying(500,700))
local wm = rect(-10, 497, 1020, 260)
blend(wm, {angle=0.0, clip=water})
print(wait(0))
blend(wm, {angle=0.006, clip=water})
print(wait(0))
blend(wm, {angle=-0.006, clip=water})
print(wait(0))

--@ chunk 205
print(wait(6*24*60))
print(drying(500,520), drying(500,700), drying(300,300))

--@ chunk 206
gl_top = pile{{"Prussian blue",1},{"smalt",2.2},{"raw umber",0.3},{"bone black",0.05}, medium=0.75}
gl_reg = rect(-14, -14, 1028, 330)
work(gl_reg, {hand="glaze", pile=gl_top, angle=0.015, coverage=2.0, seed=80001,
     load_at=function(x,y) return clamp(1.05 - y/300, 0.0, 1) ^ 1.3 end})
print(wait(0))
print(drying(300, 60))

--@ chunk 207
local bm = rect(-14, -14, 1028, 330):blur(30)
blend(bm, {angle=0.0})
print(wait(0))
blend(bm, {angle=0.01})
print(wait(0))
blend(bm, {angle=-0.01})
print(wait(0))

--@ chunk 208
SKD = {
 {0,   {{"smalt",3.0},{"cobalt blue",1.6},{"Prussian blue",0.3},{"lead white",0.6},{"raw umber",0.2}}},
 {70,  {{"smalt",2.8},{"cobalt blue",1.7},{"lead white",1.2},{"Prussian blue",0.12},{"raw umber",0.15}}},
 {140, {{"smalt",2.1},{"cobalt blue",1.3},{"lead white",2.2},{"raw umber",0.1}}},
 {205, {{"pale smalt",2.2},{"cobalt blue",0.8},{"lead white",3.4},{"green earth",0.15}}},
 {268, {{"pale smalt",1.6},{"lead white",4.6},{"cobalt blue",0.25},{"green earth",0.3},{"yellow ochre",0.15}}},
 {334, {{"lead white",5.6},{"pale smalt",0.7},{"green earth",0.3},{"yellow ochre",0.35}}},
}
SKDR = {
 {205, SKD[4][2]},
 {268, {{"pale smalt",1.5},{"lead white",4.5},{"cobalt blue",0.2},{"red earth",0.12},{"vermilion",0.06}}},
 {334, {{"lead white",5.3},{"pale smalt",0.8},{"vermilion",0.2},{"yellow ochre",0.2},{"red earth",0.08}}},
}
function skyd(ST, y, med)
  for i = 1, #ST - 1 do
    local a, b = ST[i], ST[i+1]
    if y <= b[1] or i == #ST - 1 then
      local t = clamp((y - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[2], b[2], t, med or 0.1)
    end
  end
end
local ycs = {0, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320}
for i, yc in ipairs(ycs) do
  local hh = 34
  local m = rect(-14, yc - hh, 1028, 2 * hh)
  local rt = function(y)
    local r = clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
    if yc == 0 then r = 1 end
    return r
  end
  if yc <= 190 then
    hb(m, skyd(SKD, yc, 0.12), 28, 81000 + i * 2, 4.6, {140,320}, {angle_jitter=0.012, load_at=function(x, y) return rt(y) end})
  else
    local tx = function(x) return smoothstep(350, 850, x) end
    hb(m, skyd(SKD, yc, 0.12), 28, 81000 + i * 2, 4.6, {140,320}, {angle_jitter=0.012, load_at=function(x, y) return rt(y) * (1 - tx(x)) + 0.03 end})
    hb(m, skyd(SKDR, yc, 0.12), 28, 81001 + i * 2, 4.6, {140,320}, {angle_jitter=0.012, load_at=function(x, y) return rt(y) * tx(x) + 0.03 end})
  end
  print(yc, wait(0))
end

--@ chunk 209
print(wait(8*24*60))
print(drying(300,60), drying(300,300), drying(800,300), drying(300,600))

--@ chunk 210
HZ = 497
skyOnly = rect(-14, -14, 1028, HZ + 14)
-- warm, deeper horizon sky. left = golden near the sun, right = rosy/apricot
SKG = {
 {330, {{"lead white",4.8},{"pale smalt",1.3},{"red earth",0.12},{"yellow ochre",0.35}}},
 {375, {{"lead white",4.8},{"yellow ochre",0.6},{"vermilion",0.22},{"pale smalt",0.5}}},
 {420, {{"lead white",4.3},{"vermilion",0.45},{"yellow ochre",0.5},{"chrome yellow",0.35}}},
 {460, {{"lead white",3.9},{"chrome yellow",0.9},{"vermilion",0.35},{"yellow ochre",0.1}}},
 {497, {{"lead white",3.7},{"chrome yellow",0.95},{"vermilion",0.5}}},
}
SKGR = {
 {330, {{"lead white",4.7},{"pale smalt",1.3},{"red earth",0.18},{"vermilion",0.14}}},
 {375, {{"lead white",4.6},{"vermilion",0.42},{"pale smalt",0.6},{"yellow ochre",0.3}}},
 {420, {{"lead white",4.0},{"vermilion",0.7},{"chrome yellow",0.35},{"pale smalt",0.3}}},
 {460, {{"lead white",3.6},{"vermilion",0.9},{"chrome yellow",0.5},{"red earth",0.15}}},
 {497, {{"lead white",3.4},{"vermilion",1.0},{"chrome yellow",0.55},{"red earth",0.2}}},
}
function skg(ST, y, med)
  for i = 1, #ST - 1 do
    local a, b = ST[i], ST[i+1]
    if y <= b[1] or i == #ST - 1 then
      local t = clamp((y - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[2], b[2], t, med or 0.05)
    end
  end
end
function tg(x) return smoothstep(250, 900, x) end
local ycs = {336, 362, 388, 414, 440, 466, 490}
for i, yc in ipairs(ycs) do
  local hh = 26
  local m = rect(-14, yc - hh, 1028, 2 * hh) * skyOnly
  local rt = function(y)
    if yc >= 490 then return clamp((y - 468) / 12, 0.05, 1) end
    return clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
  end
  hb(m, skg(SKG, yc), 24, 90000 + i * 2, 4.6, {140,320}, {clip=skyOnly, angle_jitter=0.012, load_at=function(x, y) return rt(y) * (1 - tg(x)) + 0.02 end})
  hb(m, skg(SKGR, yc), 24, 90001 + i * 2, 4.6, {140,320}, {clip=skyOnly, angle_jitter=0.012, load_at=function(x, y) return rt(y) * tg(x) + 0.02 end})
  print(yc, wait(0))
end
local bm = rect(-14, 316, 1028, HZ - 316):blur(10) * skyOnly
blend(bm, {angle=0.0, clip=skyOnly})
print(wait(0))
blend(bm, {angle=0.008, clip=skyOnly})
print(wait(0))

--@ chunk 211
print(wait(8*24*60))
print(drying(300,450), drying(800,480), drying(300,340))

--@ chunk 212
HZ = 497
skyOnly = rect(-14, -14, 1028, HZ + 14)
SX, SY = 300, 486
function radS(x, y, rx, ry) return math.sqrt(((x - SX) / rx)^2 + ((y - SY) / ry)^2) end
function fall(r) return clamp(1 - r, 0, 1) end
glowLayers = {
  {rx=330, ry=150, wd=22, cov=3.2, p=pile{{"lead white",4.6},{"chrome yellow",1.2},{"vermilion",0.14},{"yellow ochre",0.1}, medium=0.05}, pw=1.3},
  {rx=210, ry=92,  wd=18, cov=3.2, p=pile{{"lead white",5.6},{"chrome yellow",1.0},{"vermilion",0.04}, medium=0.05}, pw=1.2},
  {rx=120, ry=52,  wd=14, cov=3.2, p=pile{{"lead white",7.5},{"chrome yellow",0.7}, medium=0.05}, pw=1.1},
  {rx=62,  ry=27,  wd=10, cov=3.2, p=pile{{"lead white",10},{"chrome yellow",0.25}, medium=0.05}, pw=1.0},
}
for i, g in ipairs(glowLayers) do
  local m = ellipse(SX, SY, g.rx, g.ry) * skyOnly
  hb(m, g.p, g.wd, 91000 + i, g.cov, {40,140}, {clip=skyOnly, edge="lost", angle_jitter=0.02, load_at=function(x, y)
      return clamp(fall(radS(x, y, g.rx, g.ry)) ^ g.pw * 1.15, 0.02, 1) end})
  local bm = (ellipse(SX, SY, g.rx * 1.12, g.ry * 1.15) * skyOnly):blur(12)
  blend(bm, {angle=0.0, clip=skyOnly})
  print(i, wait(0))
end

--@ chunk 213
print(wait(7*24*60))
print(drying(300,470), drying(150,450))

--@ chunk 214
HZ = 497
skyOnly = rect(-14, -14, 1028, HZ + 14)
SX, SY = 300, 484
function radS2(x, y, rx, ry) return math.sqrt(((x - SX) / rx)^2 + ((y - SY) / ry)^2) end
-- brighter heart, opaque, feathered by load falloff, painted on dry paint and then only lightly fused
cores = {
  {rx=150, ry=64, wd=12, p=pile{{"lead white",8},{"chrome yellow",0.55},{"yellow ochre",0.05}, medium=0.03}, pw=1.4},
  {rx=88,  ry=38, wd=9,  p=pile{{"lead white",11},{"chrome yellow",0.28}, medium=0.03}, pw=1.3},
  {rx=44,  ry=19, wd=6,  p=pile{{"lead white",14},{"chrome yellow",0.10}, medium=0.02}, pw=1.2},
}
for i, g in ipairs(cores) do
  local m = ellipse(SX, SY, g.rx, g.ry) * skyOnly
  hb(m, g.p, g.wd, 92000 + i, 3.6, {20,90}, {clip=skyOnly, edge="lost", angle_jitter=0.02, load_at=function(x, y)
      return clamp((1 - radS2(x, y, g.rx, g.ry)) ^ g.pw * 1.2, 0.02, 1) end})
  local bm = (ellipse(SX, SY, g.rx * 1.05, g.ry * 1.08) * skyOnly):blur(6)
  blend(bm, {angle=0.0, clip=skyOnly})
  print(i, wait(0))
end

--@ chunk 215
print(wait(3*24*60))
bank_mauve = pile{{"smalt",2.4},{"red earth",0.9},{"raw umber",0.9},{"lead white",1.7},{"cobalt blue",0.4}, medium=0.05}
bank_deep  = pile{{"smalt",2.6},{"raw umber",1.3},{"red earth",0.6},{"lead white",0.9},{"Prussian blue",0.18}, medium=0.05}
bank_rim   = pile{{"lead white",3},{"vermilion",0.9},{"chrome yellow",0.8},{"red earth",0.2}, medium=0.05}
function outl(pts, seed, lobe, amt, char) return outline{pts=pts, closed=true, char=char or "soft", lobe=lobe or 30, amount=amt or 0.5, seed=seed}:mask() end
bankA = outl({{-15,420},{30,414},{80,409},{130,412},{175,405},{225,410},{275,416},{330,423},{390,431},{450,440},{500,449},
              {490,455},{430,452},{370,448},{310,446},{250,447},{190,452},{130,456},{70,458},{20,460},{-15,462}}, 4101, 34, 0.5)
bankB = outl({{-15,382},{60,376},{130,379},{190,385},{240,391},{225,397},{160,395},{90,392},{20,391},{-15,393}}, 4102, 26, 0.5)
bankC = outl({{365,404},{420,401},{470,406},{525,413},{500,419},{440,415},{390,412}}, 4103, 20, 0.4)
allbank = bankA + bankB + bankC
work(allbank, {hand="body", tool={kind="filbert", width=7}, pile=bank_mauve, length={40,110}, angle=0.0, angle_jitter=0.03, coverage=5, fill=true,
     edge={found=0.35, soft=0.45, lost=0.2, period=60, seed=3}, seed=95001})
print(wait(0))
-- deeper body in the thick part of bank A
deepA = bankA * rect(-15, 405, 320, 40)
work(deepA, {hand="body", tool={kind="filbert", width=6}, pile=bank_deep, length={30,90}, angle=0.0, angle_jitter=0.03, coverage=3, fill=true, edge="soft", seed=95002,
     load_at=function(x,y) return clamp(1.1 - x/320, 0.15, 1) * clamp((y-408)/20, 0.1, 1) end})
print(wait(0))

--@ chunk 216
HZ = 497
silK  = pile{{"bone black",3},{"raw umber",1.2},{"Prussian blue",0.25},{"smalt",0.3}, medium=0.02}
silK2 = pile{{"bone black",2.2},{"raw umber",1.7},{"red earth",0.45},{"smalt",0.6}, medium=0.02}
landPts = {{652,HZ+3},{672,HZ},{700,HZ-2},{732,HZ-5},{764,HZ-8},{796,HZ-11},{828,HZ-17},{850,HZ-25},{866,HZ-38},
           {874,HZ-49},{882,HZ-58},{889,HZ-61},{895,HZ-57},{903,HZ-63},{911,HZ-59},{918,HZ-52},{927,HZ-47},{940,HZ-44},{956,HZ-41},{976,HZ-39},{1000,HZ-37},{1030,HZ-35},
           {1030,HZ+3}}
landMask = outline{pts=landPts, closed=true, char="firm", amount=0.35, lobe=7, seed=5201}:mask()
print(landMask:area())
work(landMask, {hand="body", tool={kind="filbert", width=6}, pile=silK, length={10,32}, angle=0.0, angle_jitter=0.25, coverage=8, fill=true, clip=landMask, mix_jitter=0.15, seed=96001})
print(wait(0))
work(landMask * above({{640,HZ-4},{800,HZ-14},{1030,HZ-30}}), {hand="body", tool={kind="filbert", width=5}, pile=silK2, length={8,22}, angle=0.1, angle_jitter=0.6, coverage=2.0, fill=false, clip=landMask, mix_jitter=0.3, seed=96002,
     load_at=function(x,y) return clamp((HZ-24 - y) / 24, 0.1, 0.9) end})
print(wait(0))

--@ chunk 217
HZ = 497
treeD = pile{{"bone black",2.6},{"raw umber",1.8},{"green earth",0.5},{"Prussian blue",0.25}, medium=0.02}
treeW = pile{{"raw umber",2.2},{"red earth",1.1},{"yellow ochre",0.6},{"bone black",0.8}, medium=0.02}
crowns = {{868,452,11,12},{879,441,12,13},{892,436,13,13},{905,441,11,12},{917,449,10,10},{930,455,9,8},{946,457,9,7},{962,458,8,6},{980,459,8,5},{1000,461,8,5}}
grove2 = nil
for i, c in ipairs(crowns) do
  local e = ellipse(c[1], c[2], c[3], c[4])
  grove2 = grove2 and (grove2 + e) or e
end
grove2 = (grove2 * rect(840, 400, 200, 100)):roughen(2.0, 8, 11)
work(grove2, {hand="body", tool={kind="filbert", width=5}, pile=silK, length={8,20}, angle=0.0, angle_jitter=0.8, coverage=6, fill=true, clip=grove2, mix_jitter=0.2, seed=97001})
print(wait(0))
-- warm sunward lights on the left edges of the crowns
litz = grove2 * (ellipse(870,446,14,14) + ellipse(880,436,12,10) + ellipse(893,430,12,8) + ellipse(906,437,8,8))
work(litz, {hand="detail", tool={kind="round", width=2}, pile=treeW, length={3,8}, angle_jitter=1.0, coverage=1.6, clip=litz, seed=97002})
print(wait(0))
-- windmill
mill_tower = poly({{795,487},{805,487},{803,459},{797,459}})
mill_cap   = poly({{796.4,459.4},{803.6,459.4},{800,453.2}})
work(mill_tower, {hand="detail", tool={kind="round", width=2.2}, pile=silK, coverage=8, clip=mill_tower, seed=97011})
work(mill_cap, {hand="detail", tool={kind="round", width=1.8}, pile=silK, coverage=8, clip=mill_cap, seed=97012})
sailP = pile{{"bone black",2},{"raw umber",1.4},{"smalt",0.4}, medium=0.3}
blades = {
  poly({{805.49,453.38},{820.23,443.04},{822.53,446.32},{807.78,456.65}}),
  poly({{802.63,461.49},{812.96,476.23},{809.68,478.53},{799.35,463.78}}),
  poly({{794.51,458.63},{779.77,468.96},{777.47,465.68},{792.22,455.35}}),
  poly({{797.38,450.51},{787.04,435.77},{790.32,433.47},{800.65,448.22}}),
}
for i, b in ipairs(blades) do
  work(b, {hand="detail", tool={kind="round", width=1.4}, pile=sailP, coverage=4, clip=b, seed=97020 + i})
end
armB = brush{kind="round", width=1.5, point=0.6}
local pr = armB:pressure_for(1.2)
local tips = {{819.7,442.2},{813.8,475.7},{780.3,469.8},{786.2,436.3}}
for i, t in ipairs(tips) do
  armB:load(silK, 1.0)
  armB:stroke({{800,456},{t[1],t[2]}}, {pressure={pr, pr}})
end
print(wait(0))

--@ chunk 218
print(wait(2*24*60))
HZ = 497
silK  = pile{{"bone black",3},{"raw umber",1.2},{"Prussian blue",0.25},{"smalt",0.3}, medium=0.02}
-- retake the crowns in the dark, opaque; the brown cap is gone
cover = grove2:grow(1.5)
work(cover, {hand="body", tool={kind="filbert", width=5}, pile=silK, length={8,20}, angle=0.0, angle_jitter=0.8, coverage=9, fill=true, clip=cover, mix_jitter=0.1, seed=97101})
print(wait(0))
-- one tall slender tree and two small ones by the mill, each drawn with its own numbers
tall1 = outline{pts={{936,HZ-44},{935,HZ-58},{934,HZ-74},{936,HZ-90},{940,HZ-98},{944,HZ-90},{946,HZ-74},{945,HZ-58},{944,HZ-44}}, closed=true, char="firm", amount=0.25, lobe=5, seed=5301}:mask()
sm1 = ellipse(822, HZ-20, 6, 8) * above({{800,HZ-11},{850,HZ-11}})
sm2 = ellipse(832, HZ-22, 5, 7)
work(tall1 + sm1 + sm2, {hand="body", tool={kind="filbert", width=4}, pile=silK, length={6,16}, angle=1.5, angle_jitter=0.5, coverage=9, fill=true, clip=tall1 + sm1 + sm2, seed=97102})
print(wait(0))

--@ chunk 219
-- Upper-right cumulus: lobes placed by hand {cx, cy, rx, ry, z}
LB = {
  -- base tier
  {625,236,62,24,0}, {720,240,78,28,0}, {830,244,92,30,0}, {940,240,90,30,0}, {1015,236,50,28,0},
  -- second tier
  {668,198,56,32,20}, {760,196,72,40,20}, {865,198,78,42,25}, {965,190,72,44,20},
  -- third tier
  {735,152,50,34,35}, {815,146,60,44,40}, {902,140,64,50,40}, {985,128,58,50,35},
  -- crown
  {790,102,38,30,40}, {862,90,48,40,45}, {935,80,46,38,45}, {1002,72,42,34,35},
}
local s
for i, l in ipairs(LB) do
  local e = body.ellipsoid({l[1], l[2], l[5]}, {l[3], l[4], math.min(l[3], l[4]) * 1.05}):rough(3, 90, 200 + i)
  s = s and s:union(e) or e
end
cq = form{ {s}, light={from={-1, 0.75}, front=0.3, ambient=0.22} }
cqsil = cq:silhouette{}
cq_across = cq:field("across")
CQ = cqsil * rect(560, 30, 470, 260)
print("cloud area", CQ:area())
-- probe the value range
local vs = {}
for y = 60, 260, 5 do for x = 580, 1000, 5 do
  if CQ:at(x, y) > 0.5 then vs[#vs+1] = cq:value(x, y) end
end end
table.sort(vs)
local n = #vs
for _, q in ipairs({0.02, 0.1, 0.25, 0.5, 0.75, 0.9, 0.98}) do
  print(q, string.format("%.3f", vs[math.max(1, math.min(n, math.floor(q * n)))]))
end

--@ chunk 220
cq_shadow = pile{{"lead white",2.8},{"smalt",2.0},{"cobalt blue",0.5},{"red earth",0.6},{"raw umber",0.35}, medium=0.06}
cq_mauve  = pile{{"lead white",3.4},{"smalt",1.2},{"red earth",0.85},{"raw umber",0.3},{"vermilion",0.12}, medium=0.06}
cq_rose   = pile{{"lead white",4.0},{"vermilion",0.65},{"red earth",0.3},{"yellow ochre",0.3},{"pale smalt",0.25}, medium=0.06}
cq_gold   = pile{{"lead white",5.6},{"chrome yellow",0.55},{"vermilion",0.3},{"yellow ochre",0.3}, medium=0.05}
local vfun = function(x, y) return cq:value(x, y) end
-- 1. base coat across the whole silhouette: shadow tone, loaded fully at the interior, thin toward the rim
work(CQ, {hand="body", tool={kind="filbert", width=20}, pile=cq_shadow, angle=cq_across, angle_jitter=0.05, length={40,120}, coverage=3.4, fill=true, clip=CQ, seed=100001})
print(wait(0))
-- 2. mauve mid-tone, weighted by value (smooth ramp 0.2 .. 0.45)
work(CQ, {hand="body", tool={kind="filbert", width=16}, pile=cq_mauve, angle=cq_across, angle_jitter=0.05, length={30,90}, coverage=3.0, fill=true, clip=CQ, seed=100002,
     load_at=function(x, y) return smoothstep(0.20, 0.46, vfun(x, y)) end})
print(wait(0))
-- 3. rose light, ramp 0.38 .. 0.6
work(CQ, {hand="body", tool={kind="filbert", width=12}, pile=cq_rose, angle=cq_across, angle_jitter=0.05, length={24,70}, coverage=2.8, fill=true, clip=CQ, seed=100003,
     load_at=function(x, y) return smoothstep(0.38, 0.60, vfun(x, y)) end})
print(wait(0))
-- 4. gold, ramp 0.55 .. 0.72
work(CQ, {hand="body", tool={kind="filbert", width=9}, pile=cq_gold, angle=cq_across, angle_jitter=0.05, length={16,50}, coverage=2.4, fill=true, clip=CQ, seed=100004,
     load_at=function(x, y) return smoothstep(0.55, 0.72, vfun(x, y)) end})
print(wait(0))

--@ chunk 221
print(drying(800,150))
blend(CQ, {angle=cq_across})
print(wait(0))
blend(CQ, {angle=0.0})
print(wait(0))

--@ chunk 222
vfun = function(x, y) return cq:value(x, y) end
cq_deep = pile{{"lead white",2.0},{"smalt",2.2},{"cobalt blue",0.6},{"red earth",0.55},{"raw umber",0.4},{"Prussian blue",0.06}, medium=0.06}
work(CQ, {hand="body", tool={kind="filbert", width=16}, pile=cq_deep, angle=cq_across, angle_jitter=0.08, length={30,90}, coverage=3.2, fill=true, clip=CQ, seed=100011,
     load_at=function(x, y) return 1 - smoothstep(0.16, 0.44, vfun(x, y)) end})
print(wait(0))
blend(CQ, {angle=cq_across})
print(wait(0))
work(CQ, {hand="body", tool={kind="filbert", width=10}, pile=cq_gold, angle=cq_across, angle_jitter=0.1, length={14,44}, coverage=2.4, fill=true, clip=CQ, seed=100012,
     load_at=function(x, y) return smoothstep(0.56, 0.70, vfun(x, y)) end})
print(wait(0))
work(CQ, {hand="body", tool={kind="filbert", width=12}, pile=cq_rose, angle=cq_across, angle_jitter=0.1, length={20,60}, coverage=2.4, fill=true, clip=CQ, seed=100013,
     load_at=function(x, y) return smoothstep(0.44, 0.60, vfun(x, y)) * (1 - smoothstep(0.62, 0.72, vfun(x, y))) end})
print(wait(0))

--@ chunk 223
water = rect(-10, 497, 1020, 260)
-- colour stops for the water; y -> pile parts. L = under the sun, R = under the rose cloud.
WL = {
 {497, {{"lead white",5.8},{"chrome yellow",0.85},{"yellow ochre",0.25},{"vermilion",0.08}}},
 {518, {{"lead white",5.2},{"yellow ochre",0.65},{"pale smalt",0.7},{"vermilion",0.10},{"chrome yellow",0.2}}},
 {548, {{"lead white",3.6},{"pale smalt",1.5},{"red earth",0.3},{"cobalt blue",0.3},{"yellow ochre",0.25}}},
 {588, {{"lead white",2.0},{"smalt",1.8},{"cobalt blue",0.6},{"raw umber",0.5},{"green earth",0.2},{"red earth",0.1}}},
 {636, {{"lead white",0.9},{"smalt",2.2},{"cobalt blue",0.5},{"raw umber",0.9},{"Prussian blue",0.25},{"green earth",0.3}}},
 {690, {{"lead white",0.45},{"smalt",1.6},{"Prussian blue",0.4},{"raw umber",1.3},{"green earth",0.3},{"bone black",0.2}}},
 {745, {{"Prussian blue",0.5},{"raw umber",1.7},{"smalt",1.0},{"bone black",0.4},{"green earth",0.3}}},
}
WR = {
 {497, {{"lead white",4.3},{"vermilion",0.75},{"chrome yellow",0.35},{"red earth",0.15}}},
 {518, {{"lead white",4.4},{"vermilion",0.5},{"pale smalt",0.7},{"yellow ochre",0.2},{"red earth",0.1}}},
 {548, {{"lead white",3.4},{"pale smalt",1.4},{"vermilion",0.28},{"red earth",0.3},{"smalt",0.3}}},
 {588, {{"lead white",2.2},{"smalt",1.8},{"red earth",0.45},{"cobalt blue",0.4},{"raw umber",0.4}}},
 {636, WL[5][2]}, {690, WL[6][2]}, {745, WL[7][2]},
}
function wstopc(ST, y, med)
  for i = 1, #ST - 1 do
    local a, b = ST[i], ST[i+1]
    if y <= b[1] or i == #ST - 1 then
      local t = clamp((y - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[2], b[2], t, med or 0.05)
    end
  end
end
function xmix(x) return smoothstep(380, 800, x) end   -- 0 = L, 1 = R
n = 0
for yc = 500, 750, 13 do
  n = n + 1
  local hh = 15
  local m = rect(-10, yc - hh, 1020, 2 * hh) * water
  local wd = 7 + (yc - 497) * 0.04
  local rt = function(y)
    if yc < 505 then return clamp((y - 497) / 8, 0.05, 1) end
    return clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
  end
  hb(m, wstopc(WL, yc), wd, 110000 + n * 3, 3.8, {40,150}, {clip=water, edge="found", angle_jitter=0.015, mix_jitter=0.35,
      load_at=function(x, y) return rt(y) * (1 - xmix(x)) + 0.02 end})
  hb(m, wstopc(WR, yc), wd, 110001 + n * 3, 3.8, {40,150}, {clip=water, edge="found", angle_jitter=0.015, mix_jitter=0.35,
      load_at=function(x, y) return rt(y) * xmix(x) + 0.02 end})
  if n % 4 == 0 then print(yc, wait(0)) end
end
print(wait(0))

--@ chunk 224
print(drying(800,150), drying(300,300), drying(300,520), drying(300,700), drying(850,480))

--@ chunk 225
local wm = (rect(-10, 545, 1020, 205) * water):blur(14)
blend(wm, {angle=0.0, clip=water})
print(wait(0))
print(drying(300,600), drying(300,700))

--@ chunk 226
HZ = 497
skyOnly = rect(-14, -14, 1028, HZ + 14)
patchM = (bankA + bankB + bankC):grow(12):blur(7) * skyOnly * rect(-20, 300, 600, 197)
print("patch area", patchM:area())
-- 1. cover with the local horizon colours, strips overlapping, all clipped to the patch (feathered by the blur)
local ycs = {336, 362, 388, 414, 440, 466, 488}
for i, yc in ipairs(ycs) do
  local hh = 26
  local m = rect(-20, yc - hh, 600, 2 * hh) * patchM
  local rt = function(y)
    if yc >= 488 then return clamp((y - 468) / 12, 0.05, 1) end
    return clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
  end
  hb(m, skg(SKG, yc), 22, 120000 + i * 2, 5.0, {60,200}, {clip=patchM, angle_jitter=0.012, load_at=function(x, y) return rt(y) end})
  hb(m, skg(SKGR, yc), 22, 120001 + i * 2, 5.0, {60,200}, {clip=patchM, angle_jitter=0.012, load_at=function(x, y) return rt(y) * smoothstep(380, 600, x) end})
  print(yc, wait(0))
end
-- 2. re-lay the glow rings inside the patch
for i, g in ipairs(glowLayers) do
  local m = ellipse(SX, SY, g.rx, g.ry) * patchM
  hb(m, g.p, g.wd, 121000 + i, g.cov, {40,140}, {clip=patchM, edge="lost", angle_jitter=0.02, load_at=function(x, y)
      return clamp(fall(radS(x, y, g.rx, g.ry)) ^ g.pw * 1.15, 0.02, 1) end})
  print(i, wait(0))
end
for i, g in ipairs(cores) do
  local m = ellipse(SX, SY, g.rx, g.ry) * patchM
  hb(m, g.p, g.wd, 122000 + i, 3.6, {20,90}, {clip=patchM, edge="lost", angle_jitter=0.02, load_at=function(x, y)
      return clamp((1 - radS2(x, y, g.rx, g.ry)) ^ g.pw * 1.2, 0.02, 1) end})
  print(i, wait(0))
end
blend(patchM, {angle=0.0, clip=patchM})
print(wait(0))
blend(patchM, {angle=0.006, clip=patchM})
print(wait(0))

--@ chunk 227
print(drying(300,440))
patchM2 = (bankA + bankB + bankC):grow(6):blur(4) * skyOnly * rect(-20, 300, 600, 197)
local ycs = {350, 378, 406, 434, 462, 486}
for i, yc in ipairs(ycs) do
  local hh = 24
  local m = rect(-20, yc - hh, 600, 2 * hh) * patchM2
  local rt = function(y)
    if yc >= 486 then return clamp((y - 468) / 12, 0.05, 1) end
    return clamp(1.4 - math.abs(y - yc) / hh, 0.05, 1)
  end
  local pl = skg(SKG, yc, 0.01)
  local pr = skg(SKGR, yc, 0.01)
  hb(m, pl, 18, 123000 + i * 2, 8.0, {60,160}, {clip=patchM2, angle_jitter=0.012, load_at=function(x, y) return rt(y) * (1 - smoothstep(380, 600, x)) + 0.02 end})
  hb(m, pr, 18, 123001 + i * 2, 8.0, {60,160}, {clip=patchM2, angle_jitter=0.012, load_at=function(x, y) return rt(y) * smoothstep(380, 600, x) + 0.02 end})
  print(yc, wait(0))
end

--@ chunk 228
print(wait(10*24*60))
print(drying(300,440), drying(100,410), drying(300,600), drying(300,700), drying(850,480), drying(800,150))

--@ chunk 229
patchM3 = (bankA + bankB + bankC):grow(9):blur(5) * skyOnly * rect(-20, 300, 600, 197)
local ycs = {350, 378, 406, 434, 462, 486}
for i, yc in ipairs(ycs) do
  local hh = 24
  local m = rect(-20, yc - hh, 600, 2 * hh) * patchM3
  local rt = function(y)
    if yc >= 486 then return clamp((y - 468) / 12, 0.05, 1) end
    return clamp(1.4 - math.abs(y - yc) / hh, 0.05, 1)
  end
  local pl = skg(SKG, yc, 0.01)
  local pr = skg(SKGR, yc, 0.01)
  hb(m, pl, 18, 124000 + i * 2, 8.0, {60,160}, {clip=patchM3, angle_jitter=0.012, load_at=function(x, y) return rt(y) * (1 - smoothstep(380, 600, x)) + 0.02 end})
  hb(m, pr, 18, 124001 + i * 2, 8.0, {60,160}, {clip=patchM3, angle_jitter=0.012, load_at=function(x, y) return rt(y) * smoothstep(380, 600, x) + 0.02 end})
  print(yc, wait(0))
end

--@ chunk 230
water = rect(-10, 497, 1020, 260)
gl_a = pile{{"lead white",10},{"chrome yellow",0.35}, medium=0.02}
gl_b = pile{{"lead white",7},{"chrome yellow",0.9},{"yellow ochre",0.2}, medium=0.03}
gl_c = pile{{"lead white",6},{"yellow ochre",0.8},{"chrome yellow",0.6},{"vermilion",0.1}, medium=0.03}
PX = 300
function pw(y) return 16 + (y - 497) * 0.45 end
-- test patch: x 200..400, y 556..610
pathT = poly({{PX-pw(556),556},{PX+pw(556),556},{PX+pw(610),610},{PX-pw(610),610}})
work(pathT * water, {hand="detail", tool={kind="flat", width=2.2}, pile=gl_b, length={6,26}, angle=0.0, angle_jitter=0.03, coverage=1.6, fill=false, clip=false,
     order="scatter", pressure={0.6,0.9}, seed=130001,
     load_at=function(x, y) local d = math.abs(x - PX) / pw(y); return clamp(1.15 - d, 0, 1) end})
print(wait(0))

--@ chunk 231
patchW = pathT:grow(6):blur(4) * water
local ycs = {560, 574, 588, 602, 614}
for i, yc in ipairs(ycs) do
  local hh = 12
  local m = rect(150, yc - hh, 300, 2 * hh) * patchW
  hb(m, wstopc(WL, yc, 0.02), 8, 131000 + i, 8.0, {40,150}, {clip=patchW, angle_jitter=0.012, load_at=function(x, y) return clamp(1.5 - math.abs(y - yc) / hh, 0.05, 1) end})
end
print(wait(0))

--@ chunk 232
print(wait(0))
water = rect(-10, 497, 1020, 260)
function rowstrip(yc, hh, wd, seed, cov, twoSided)
  local m = rect(-10, yc - hh, 1020, 2 * hh) * water
  local rt = function(y)
    if yc < 505 then return clamp((y - 497) / 8, 0.05, 1) end
    return clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
  end
  if twoSided then
    hb(m, wstopc(WL, yc, 0.05), wd, seed, cov, {60,180}, {clip=water, edge="found", angle_jitter=0.012, mix_jitter=0.3, load_at=function(x, y) return rt(y) * (1 - xmix(x)) + 0.02 end})
    hb(m, wstopc(WR, yc, 0.05), wd, seed + 1, cov, {60,180}, {clip=water, edge="found", angle_jitter=0.012, mix_jitter=0.3, load_at=function(x, y) return rt(y) * xmix(x) + 0.02 end})
  else
    hb(m, wstopc(WL, yc, 0.05), wd, seed, cov, {60,180}, {clip=water, edge="found", angle_jitter=0.012, mix_jitter=0.3, load_at=function(x, y) return rt(y) end})
  end
end
-- bottom half first (dark paint stays open longest), then upward
local yc = 746
local k = 0
while yc >= 626 do
  k = k + 1
  rowstrip(yc, 14, 20, 140000 + k * 3, 3.2, false)
  yc = yc - 14
end
print(wait(0))

--@ chunk 233
local yc = 612
local k = 100
while yc >= 498 do
  k = k + 1
  rowstrip(yc, 14, 18, 140000 + k * 3, 3.2, true)
  yc = yc - 14
end
print(wait(0))
print(drying(300,745), drying(300,650), drying(300,560), drying(300,510))

--@ chunk 234
PX = 300
function phw(y) return 24 + (y - 497) * 0.5 end
function pint(y) return clamp(1.15 - (y - 497) / 290, 0.28, 1) end
function pshape(x, y, s) local d = math.abs(x - PX) / (phw(y) * s); return clamp(1.25 - d ^ 1.4, 0, 1) end
local pm = rect(100, 497, 400, 250) * water
hb(pm, gl_c, 10, 150001, 3.4, {30,100}, {clip=water, edge="found", angle_jitter=0.02, load_at=function(x, y) return pshape(x, y, 1.0) * pint(y) end})
print(wait(0))
hb(pm * rect(100, 497, 400, 170), gl_b, 8, 150002, 3.2, {24,80}, {clip=water, edge="found", angle_jitter=0.02, load_at=function(x, y) return pshape(x, y, 0.7) * pint(y) end})
print(wait(0))
hb(pm * rect(100, 497, 400, 110), gl_a, 6, 150003, 3.0, {16,60}, {clip=water, edge="found", angle_jitter=0.02, load_at=function(x, y) return pshape(x, y, 0.45) * pint(y) end})
print(wait(0))
print(drying(300, 560), drying(300, 700))

--@ chunk 235
print(drying(300,440), drying(100,410), drying(300,600), drying(300,700), drying(850,480), drying(800,150), drying(60,700), drying(700,560))
print(wait(0))

--@ chunk 236
print(wait(12*24*60))
print(drying(300,440), drying(100,410), drying(300,600), drying(300,700), drying(60,700), drying(700,560))

--@ chunk 237
water = rect(-10, 497, 1020, 260)
PX = 300
function phw(y) return 24 + (y - 497) * 0.5 end
sideMask = water * mask(function(x, y)
  local d = math.abs(x - PX) / phw(y)
  return smoothstep(0.95, 1.9, d)
end)
print("side area", sideMask:area())
function sidestrip(yc, hh, wd, seed, cov)
  local m = rect(-10, yc - hh, 1020, 2 * hh) * sideMask
  local rt = function(y)
    if yc < 505 then return clamp((y - 497) / 8, 0.05, 1) end
    return clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
  end
  hb(m, wstopc(WL, yc, 0.04), wd, seed, cov, {60,180}, {clip=sideMask, angle_jitter=0.012, mix_jitter=0.25, load_at=function(x, y) return rt(y) * (1 - xmix(x)) + 0.02 end})
  hb(m, wstopc(WR, yc, 0.04), wd, seed + 1, cov, {60,180}, {clip=sideMask, angle_jitter=0.012, mix_jitter=0.25, load_at=function(x, y) return rt(y) * xmix(x) + 0.02 end})
end
-- bottom first, upward; every strip is generously overlapped (hh 30, step 22)
local yc = 748
local k = 0
while yc >= 640 do
  k = k + 1
  sidestrip(yc, 30, 22, 160000 + k * 3, 4.2)
  yc = yc - 22
end
print(wait(0))

--@ chunk 238
local yc = 618
local k = 100
while yc >= 498 do
  k = k + 1
  sidestrip(yc, 30, 20, 160000 + k * 3, 4.2)
  yc = yc - 20
end
print(wait(0))
print(drying(60,700), drying(700,560))

--@ chunk 239
blend(sideMask, {angle=0.0})
print(wait(0))
blend(sideMask * rect(-10, 497, 1020, 130), {angle=0.006})
print(wait(0))

--@ chunk 240
print(drying(700,560), drying(700,700), drying(60,600))
local yc = 748
local k = 200
while yc >= 498 do
  k = k + 1
  sidestrip(yc, 30, 20, 170000 + k * 3, 4.2)
  yc = yc - 20
end
print(wait(0))

--@ chunk 241
SKD2 = {
 {0,   {{"smalt",3.0},{"cobalt blue",1.4},{"Prussian blue",0.55},{"lead white",0.35},{"raw umber",0.2}}},
 {70,  {{"smalt",2.8},{"cobalt blue",1.7},{"Prussian blue",0.28},{"lead white",0.9},{"raw umber",0.15}}},
 {140, {{"smalt",2.2},{"cobalt blue",1.5},{"lead white",1.9},{"Prussian blue",0.06},{"raw umber",0.1}}},
 {205, {{"pale smalt",2.2},{"cobalt blue",0.85},{"lead white",3.2},{"green earth",0.15}}},
 {268, {{"pale smalt",1.6},{"lead white",4.6},{"cobalt blue",0.25},{"green earth",0.3},{"yellow ochre",0.15}}},
 {334, {{"lead white",5.6},{"pale smalt",0.7},{"green earth",0.3},{"yellow ochre",0.35}}},
}
SKD2R = {
 {205, SKD2[4][2]},
 {268, {{"pale smalt",1.5},{"lead white",4.5},{"cobalt blue",0.2},{"red earth",0.12},{"vermilion",0.06}}},
 {334, {{"lead white",5.3},{"pale smalt",0.8},{"vermilion",0.2},{"yellow ochre",0.2},{"red earth",0.08}}},
}
local ycs = {0, 26, 52, 78, 104, 130, 156, 182, 208, 234, 260, 286, 310}
for i, yc in ipairs(ycs) do
  local hh = 34
  local m = rect(-14, yc - hh, 1028, 2 * hh)
  local rt = function(y)
    local r = clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
    if yc == 0 then r = 1 end
    return r
  end
  if yc <= 190 then
    hb(m, skyd(SKD2, yc, 0.12), 28, 180000 + i * 2, 4.8, {140,320}, {angle_jitter=0.012, load_at=function(x, y) return rt(y) end})
  else
    local tx = function(x) return smoothstep(350, 850, x) end
    hb(m, skyd(SKD2, yc, 0.12), 28, 180000 + i * 2, 4.8, {140,320}, {angle_jitter=0.012, load_at=function(x, y) return rt(y) * (1 - tx(x)) + 0.03 end})
    hb(m, skyd(SKD2R, yc, 0.12), 28, 180001 + i * 2, 4.8, {140,320}, {angle_jitter=0.012, load_at=function(x, y) return rt(y) * tx(x) + 0.03 end})
  end
  print(yc, wait(0))
end

--@ chunk 242
print(wait(12*24*60))
print(drying(300,100), drying(300,300), drying(700,480), drying(300,700), drying(700,560))

--@ chunk 243
cbar1 = pile{{"lead white",2.0},{"smalt",2.2},{"red earth",0.7},{"raw umber",0.6},{"cobalt blue",0.3}, medium=0.05}
cbar2 = pile{{"lead white",3.0},{"smalt",1.4},{"red earth",1.1},{"raw umber",0.4},{"vermilion",0.15}, medium=0.05}
cbar3 = pile{{"lead white",3.6},{"red earth",0.9},{"vermilion",0.7},{"yellow ochre",0.3},{"smalt",0.4}, medium=0.05}
cbar4 = pile{{"lead white",4.5},{"vermilion",0.8},{"chrome yellow",0.8},{"red earth",0.15}, medium=0.05}

function paint_bar(m, y0, y1, seed, wd, edge)
  local h = y1 - y0
  local stops = {
    {p=cbar1, c=y0 + 0.18 * h, hh=0.34 * h},
    {p=cbar2, c=y0 + 0.50 * h, hh=0.34 * h},
    {p=cbar3, c=y0 + 0.78 * h, hh=0.30 * h},
    {p=cbar4, c=y0 + 0.96 * h, hh=0.22 * h},
  }
  for i, s in ipairs(stops) do
    work(m, {hand="body", tool={kind="filbert", width=wd}, pile=s.p, length={40,120}, angle=0.0, angle_jitter=0.03, coverage=4.2, fill=true, edge=edge or "soft", seed=seed + i, mix_jitter=0.3,
      load_at=function(x, y) return clamp(1.35 - math.abs(y - s.c) / s.hh, 0.03, 1) end})
  end
end
B1 = outline{pts={{-20,356},{20,346},{70,338},{120,331},{175,335},{225,327},{280,334},{330,341},{380,350},{425,358},{455,365},
                  {420,372},{370,371},{310,368},{250,370},{190,372},{130,375},{70,377},{20,378},{-20,380}},
             closed=true, char="soft", lobe=34, amount=0.6, seed=2001}:mask()
paint_bar(B1, 328, 380, 250000, 8)
print(wait(0))
local bm = B1:grow(2)
blend(bm, {angle=0.0})
print(wait(0))

--@ chunk 244
print(wait(9*24*60))
print(drying(200,350))

--@ chunk 245
-- opaque, cooler and darker bar tones. Test on B1 and look at it in value.
cbar1 = pile{{"bone black",1.1},{"raw umber",1.0},{"smalt",1.4},{"lead white",1.4},{"red earth",0.5},{"cobalt blue",0.4}, medium=0.03}
cbar2 = pile{{"bone black",0.5},{"raw umber",0.9},{"smalt",1.0},{"lead white",2.2},{"red earth",1.0},{"vermilion",0.1}, medium=0.03}
cbar3 = pile{{"raw umber",0.5},{"red earth",1.0},{"lead white",3.0},{"vermilion",0.8},{"smalt",0.5},{"yellow ochre",0.3}, medium=0.03}
cbar4 = pile{{"lead white",4.2},{"vermilion",0.9},{"chrome yellow",1.0},{"red earth",0.15}, medium=0.03}
function paint_bar(m, y0, y1, seed, wd, edge, cov)
  local h = y1 - y0
  local stops = {
    {p=cbar1, c=y0 + 0.16 * h, hh=0.36 * h},
    {p=cbar2, c=y0 + 0.50 * h, hh=0.34 * h},
    {p=cbar3, c=y0 + 0.80 * h, hh=0.28 * h},
    {p=cbar4, c=y0 + 0.97 * h, hh=0.20 * h},
  }
  for i, s in ipairs(stops) do
    work(m, {hand="body", tool={kind="filbert", width=wd}, pile=s.p, length={40,120}, angle=0.0, angle_jitter=0.03, coverage=cov or 5.0, fill=true, edge=edge or "soft", seed=seed + i, mix_jitter=0.3,
      load_at=function(x, y) return clamp(1.35 - math.abs(y - s.c) / s.hh, 0.03, 1) end})
  end
end
paint_bar(B1, 328, 380, 251000, 8)
print(wait(0))
blend(B1:grow(1), {angle=0.0})
print(wait(0))

--@ chunk 246
sw = {
 {{"bone black",1},{"lead white",2},{"cobalt blue",1}},
 {{"bone black",1},{"lead white",2.5},{"cobalt blue",0.7},{"vermilion",0.3}},
 {{"bone black",1},{"lead white",3},{"smalt",1},{"red earth",0.3}},
 {{"Prussian blue",0.3},{"lead white",3},{"vermilion",0.3},{"bone black",0.5}},
 {{"smalt",2},{"bone black",0.8},{"lead white",2},{"vermilion",0.2}},
 {{"cobalt blue",1.5},{"red earth",0.5},{"lead white",2},{"bone black",0.5}},
 {{"lead white",4},{"vermilion",0.8},{"chrome yellow",0.8}},
 {{"lead white",3},{"vermilion",1},{"red earth",0.3}},
}
for i, t in ipairs(sw) do
  t.medium = 0.03
  local p = pile(t)
  local col = (i - 1) % 4
  local row = (i - 1) // 4
  local m = rect(560 + col * 100, 686 + row * 26, 96, 24)
  work(m, {hand="body", tool={kind="filbert", width=8}, pile=p, length={20,60}, angle=0.0, coverage=7, fill=true, clip=m, seed=270000 + i})
end
print(wait(0))

--@ chunk 247
sw2 = {
 {{"bone black",1},{"cobalt blue",1},{"lead white",0.5}},
 {{"bone black",1},{"cobalt blue",1},{"vermilion",0.3},{"lead white",0.8}},
 {{"bone black",1},{"smalt",1.5},{"red earth",0.4},{"lead white",0.6}},
 {{"Prussian blue",0.3},{"bone black",1},{"red earth",0.4},{"lead white",1}},
 {{"bone black",1},{"cobalt blue",0.6},{"red earth",0.6},{"lead white",1.2}},
 {{"cobalt blue",1.5},{"vermilion",0.3},{"bone black",0.8},{"lead white",1.5}},
 {{"bone black",0.8},{"raw umber",0.6},{"cobalt blue",1},{"lead white",1}},
 {{"Prussian blue",0.2},{"raw umber",1},{"vermilion",0.3},{"lead white",1.2}},
}
for i, t in ipairs(sw2) do
  t.medium = 0.03
  local p = pile(t)
  local col = (i - 1) % 4
  local row = (i - 1) // 4
  local m = rect(560 + col * 100, 632 + row * 26, 96, 24)
  work(m, {hand="body", tool={kind="filbert", width=8}, pile=p, length={20,60}, angle=0.0, coverage=7, fill=true, clip=m, seed=271000 + i})
end
print(wait(0))

--@ chunk 248
-- Upper-right cumulus R1: one drawn contour; hand-placed lobe centres guide the stroke direction (strokes follow the form)
R1o = outline{pts={{540,236},{562,220},{586,204},{606,190},{628,172},{650,152},{676,132},{704,118},{730,98},{756,84},{786,72},{808,56},{838,46},{866,38},{894,42},{918,28},{952,24},{986,20},{1040,20},
   {1040,264},{994,270},{952,264},{916,272},{880,266},{846,274},{810,268},{776,260},{742,264},{706,254},{670,252},{632,246},{592,242},{560,240}},
   closed=true, char="soft", lobe=40, amount=0.65, seed=3001}
R1 = R1o:mask()
print("R1", R1:area())
lobes = {{830,150,72},{935,118,62},{742,165,52},{692,205,42},{900,215,58},{800,226,46},{990,200,52},{985,68,50},{880,66,46},{782,102,36},{620,225,34}}
function lobe_angle(x, y)
  local best, bd = 1, 1e9
  for i, l in ipairs(lobes) do
    local d = math.sqrt((x - l[1])^2 + (y - l[2])^2) / l[3]
    if d < bd then bd = d; best = i end
  end
  local l = lobes[best]
  return math.atan(y - l[2], x - l[1]) + math.pi / 2
end
c_body = pile{{"bone black",0.55},{"cobalt blue",1.0},{"smalt",1.2},{"red earth",0.35},{"lead white",1.7}, medium=0.04}
work(R1, {hand="body", tool={kind="filbert", width=16}, pile=c_body, angle=lobe_angle, angle_jitter=0.12, length={30,90}, coverage=5, fill=true, clip=R1, seed=300001,
     mix_jitter=0.3})
print(wait(0))

--@ chunk 249
-- ellipsoid lobes for lighting (same centres/radii as the stroke-guide lobes, plus depth)
local s
for i, l in ipairs(lobes) do
  local e = body.ellipsoid({l[1], l[2], 0}, {l[3] * 1.15, l[3] * 0.95, l[3] * 1.0}):rough(2.5, 70, 300 + i)
  s = s and s:union(e) or e
end
r1f = form{ {s}, light={from={-1, 0.6}, front=0.3, ambient=0.2} }
local chars = " .:-=+*#%@"
for y = 30, 290, 12 do
  local row = {}
  for x = 540, 1020, 8 do
    if R1:at(x, y) > 0.5 then
      local v = clamp(r1f:value(x, y), 0, 0.999)
      local k = math.floor(v * 10) + 1
      row[#row+1] = chars:sub(k, k)
    else
      row[#row+1] = "'"
    end
  end
  print(string.format("%3d ", y) .. table.concat(row))
end

--@ chunk 250
r1v = function(x, y) return r1f:value(x, y) end
c_shade = pile{{"bone black",0.9},{"cobalt blue",1.0},{"smalt",1.2},{"red earth",0.3},{"lead white",1.0}, medium=0.04}
c_lit1  = pile{{"lead white",2.2},{"red earth",0.8},{"vermilion",0.25},{"cobalt blue",0.5},{"bone black",0.3}, medium=0.04}
c_lit2  = pile{{"lead white",3.5},{"vermilion",0.7},{"yellow ochre",0.5},{"red earth",0.3}, medium=0.04}
c_rim   = pile{{"lead white",5},{"chrome yellow",0.7},{"vermilion",0.35}, medium=0.04}
-- 1. shade: weighted to the low-value side
work(R1, {hand="body", tool={kind="filbert", width=14}, pile=c_shade, angle=lobe_angle, angle_jitter=0.12, length={24,70}, coverage=4.2, fill=true, clip=R1, seed=310001, mix_jitter=0.25,
     load_at=function(x, y) return 1 - smoothstep(0.12, 0.34, r1v(x, y)) end})
print(wait(0))
-- 2. warm grey-pink lit planes
work(R1, {hand="body", tool={kind="filbert", width=11}, pile=c_lit1, angle=lobe_angle, angle_jitter=0.12, length={18,54}, coverage=3.4, fill=true, clip=R1, seed=310002, mix_jitter=0.25,
     load_at=function(x, y) return smoothstep(0.34, 0.5, r1v(x, y)) end})
print(wait(0))
-- 3. rose-gold on the brighter parts
work(R1, {hand="body", tool={kind="filbert", width=8}, pile=c_lit2, angle=lobe_angle, angle_jitter=0.14, length={12,38}, coverage=2.6, fill=true, clip=R1, seed=310003, mix_jitter=0.25,
     load_at=function(x, y) return smoothstep(0.5, 0.64, r1v(x, y)) end})
print(wait(0))
-- 4. gold rim: only the brightest edges
work(R1, {hand="body", tool={kind="filbert", width=6}, pile=c_rim, angle=lobe_angle, angle_jitter=0.16, length={8,26}, coverage=2.0, fill=true, clip=R1, seed=310004, mix_jitter=0.25,
     load_at=function(x, y) return smoothstep(0.64, 0.78, r1v(x, y)) end})
print(wait(0))

--@ chunk 251
print(drying(800,150))
blend(R1, {angle=lobe_angle})
print(wait(0))

--@ chunk 252
HZ = 497
skyLow = rect(-14, -14, 1028, HZ + 14)
SXd, SYd = 300, 491
-- 1. deeper gold ring around the sun, painted opaque with lost edges so it fades into the glow
ringG = pile{{"lead white",4.6},{"chrome yellow",1.0},{"vermilion",0.09}, medium=0.04}
work(ellipse(SXd, SYd, 92, 46) * skyLow, {hand="body", tool={kind="flat", width=8}, pile=ringG, length={20,70}, angle=0.0, angle_jitter=0.02, coverage=3.0, fill=true, clip=skyLow, edge="lost", seed=310501,
     load_at=function(x, y)
        local r = math.sqrt(((x - SXd) / 92)^2 + ((y - SYd) / 46)^2)
        return clamp(1.25 - r, 0, 1) ^ 1.1 * 0.9 end})
print(wait(0))
blend((ellipse(SXd, SYd, 100, 52) * skyLow):blur(8), {angle=0.0, clip=skyLow})
print(wait(0))

--@ chunk 253
-- cover the sausage cloud B1 (dry) with the sky gradient, opaque, feathered
coverM = (B1:grow(10)):blur(6)
print("cover area", coverM:area())
local ycs = {326, 344, 362, 380, 396}
for i, yc in ipairs(ycs) do
  local hh = 20
  local m = rect(-20, yc - hh, 520, 2 * hh) * coverM
  local col = (yc <= 334) and skyd(SKD2, yc, 0.02) or skg(SKG, yc, 0.02)
  local colR = (yc <= 334) and skyd(SKD2R, yc, 0.02) or skg(SKGR, yc, 0.02)
  hb(m, col, 14, 320000 + i * 2, 8.0, {60,160}, {clip=coverM, angle_jitter=0.012, load_at=function(x, y)
      return clamp(1.5 - math.abs(y - yc) / hh, 0.05, 1) * (1 - smoothstep(300, 520, x)) + 0.02 end})
  hb(m, colR, 14, 320001 + i * 2, 8.0, {60,160}, {clip=coverM, angle_jitter=0.012, load_at=function(x, y)
      return clamp(1.5 - math.abs(y - yc) / hh, 0.05, 1) * smoothstep(300, 520, x) + 0.02 end})
end
print(wait(0))
blend(coverM, {angle=0.0, clip=coverM})
print(wait(0))

--@ chunk 254
HZ = 497
skyLow = rect(-14, -14, 1028, HZ + 14)
SLs = {
 {300, {{"lead white",5.4},{"pale smalt",0.9},{"green earth",0.25},{"yellow ochre",0.3}}},
 {340, {{"lead white",5.3},{"yellow ochre",0.45},{"vermilion",0.22},{"pale smalt",0.35}}},
 {385, {{"lead white",4.8},{"vermilion",0.42},{"yellow ochre",0.5},{"chrome yellow",0.3}}},
 {430, {{"lead white",4.4},{"chrome yellow",0.85},{"vermilion",0.34},{"yellow ochre",0.1}}},
 {470, {{"lead white",4.6},{"chrome yellow",1.0},{"vermilion",0.12}}},
 {497, {{"lead white",6},{"chrome yellow",0.8},{"vermilion",0.04}}},
}
function slcol(y, med)
  for i = 1, #SLs - 1 do
    local a, b = SLs[i], SLs[i+1]
    if y <= b[1] or i == #SLs - 1 then
      local t = clamp((y - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[2], b[2], t, med or 0.03)
    end
  end
end
function srcol(y, med)
  if y <= 334 then return skyd(SKD2R, y, med or 0.03) end
  return skg(SKGR, y, med or 0.03)
end
local ycs = {310, 334, 358, 382, 406, 430, 454, 478, 494}
for i, yc in ipairs(ycs) do
  local hh = 26
  local m = rect(-20, yc - hh, 730, 2 * hh) * skyLow
  local rt = function(y)
    if yc >= 494 then return clamp((y - 470) / 12, 0.05, 1) end
    return clamp(1.5 - math.abs(y - yc) / hh, 0.05, 1)
  end
  local edgeR = function(x) return clamp((700 - x) / 90, 0.0, 1) end
  hb(m, slcol(yc), 22, 330000 + i * 2, 8.0, {70,190}, {clip=skyLow, angle_jitter=0.012, mix_jitter=0.3, load_at=function(x, y)
      return rt(y) * (1 - smoothstep(380, 640, x)) * edgeR(x) + 0.02 end})
  hb(m, srcol(yc), 22, 330001 + i * 2, 8.0, {70,190}, {clip=skyLow, angle_jitter=0.012, mix_jitter=0.3, load_at=function(x, y)
      return rt(y) * smoothstep(380, 640, x) * edgeR(x) + 0.02 end})
  print(yc, wait(0))
end

--@ chunk 255
local bmask = (rect(-20, 296, 800, 201)):blur(12) * skyLow
blend(bmask, {angle=0.0, clip=skyLow})
print(wait(0))
blend(bmask, {angle=0.008, clip=skyLow})
print(wait(0))

--@ chunk 256
print(wait(10*24*60))
print(drying(800,150), drying(700,700), drying(300,400), drying(800,480), drying(300,600))

--@ chunk 257
water = rect(-10, 497, 1020, 260)
swReg = (rect(548, 612, 430, 134)):blur(10) * water
print("swReg", swReg:area())
local yc = 746
local k = 0
while yc >= 612 do
  k = k + 1
  local hh = 26
  local m = rect(540, yc - hh, 450, 2 * hh) * swReg
  local rt = function(y) return clamp(1.4 - math.abs(y - yc) / hh, 0.05, 1) end
  hb(m, wstopc(WR, yc, 0.04), 18, 400000 + k * 2, 6.0, {50,160}, {clip=swReg, angle_jitter=0.012, mix_jitter=0.25, load_at=function(x, y) return rt(y) end})
  yc = yc - 14
end
print(wait(0))
blend(swReg, {angle=0.0, clip=swReg})
print(wait(0))
blend(swReg, {angle=0.006, clip=swReg})
print(wait(0))

--@ chunk 258
water = rect(-10, 497, 1020, 260)
PX = 300
function phw(y) return 24 + (y - 497) * 0.5 end
sideMask = water * mask(function(x, y)
  local d = math.abs(x - PX) / phw(y)
  return smoothstep(0.95, 1.9, d)
end)
-- richer stops: darker in the foreground, pale mauve band reflecting the horizon glow
WL2 = {
 {497, {{"lead white",5.8},{"chrome yellow",0.85},{"yellow ochre",0.25},{"vermilion",0.08}}},
 {518, {{"lead white",5.2},{"yellow ochre",0.65},{"pale smalt",0.7},{"vermilion",0.10},{"chrome yellow",0.2}}},
 {548, {{"lead white",3.8},{"pale smalt",1.5},{"red earth",0.3},{"cobalt blue",0.3},{"yellow ochre",0.25}}},
 {590, {{"lead white",2.6},{"smalt",1.5},{"cobalt blue",0.5},{"raw umber",0.4},{"green earth",0.2},{"red earth",0.15}}},
 {640, {{"lead white",1.5},{"smalt",2.0},{"cobalt blue",0.5},{"raw umber",0.8},{"Prussian blue",0.25},{"green earth",0.3}}},
 {695, {{"lead white",0.8},{"smalt",1.6},{"Prussian blue",0.4},{"raw umber",1.2},{"green earth",0.3},{"bone black",0.2}}},
 {745, {{"lead white",0.3},{"Prussian blue",0.5},{"raw umber",1.6},{"smalt",1.0},{"bone black",0.4},{"green earth",0.3}}},
}
WR2 = {
 {497, {{"lead white",4.3},{"vermilion",0.75},{"chrome yellow",0.35},{"red earth",0.15}}},
 {518, {{"lead white",4.4},{"vermilion",0.5},{"pale smalt",0.7},{"yellow ochre",0.2},{"red earth",0.1}}},
 {548, {{"lead white",3.6},{"pale smalt",1.4},{"vermilion",0.28},{"red earth",0.3},{"smalt",0.3}}},
 {590, {{"lead white",2.7},{"smalt",1.6},{"red earth",0.4},{"cobalt blue",0.4},{"raw umber",0.35}}},
 {640, WL2[5][2]}, {695, WL2[6][2]}, {745, WL2[7][2]},
}
function wsc(ST, y, med)
  for i = 1, #ST - 1 do
    local a, b = ST[i], ST[i+1]
    if y <= b[1] or i == #ST - 1 then
      local t = clamp((y - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[2], b[2], t, med or 0.05)
    end
  end
end
function sidestrip2(yc, hh, wd, seed, cov)
  local m = rect(-10, yc - hh, 1020, 2 * hh) * sideMask
  local rt = function(y)
    if yc < 505 then return clamp((y - 497) / 8, 0.05, 1) end
    return clamp(1.4 - math.abs(y - yc) / hh, 0.05, 1)
  end
  hb(m, wsc(WL2, yc, 0.05), wd, seed, cov, {60,180}, {clip=sideMask, angle_jitter=0.012, mix_jitter=0.25, load_at=function(x, y) return rt(y) * (1 - xmix(x)) + 0.02 end})
  hb(m, wsc(WR2, yc, 0.05), wd, seed + 1, cov, {60,180}, {clip=sideMask, angle_jitter=0.012, mix_jitter=0.25, load_at=function(x, y) return rt(y) * xmix(x) + 0.02 end})
end
local yc = 748
local k = 0
while yc >= 498 do
  k = k + 1
  sidestrip2(yc, 28, 20, 410000 + k * 3, 5.0)
  yc = yc - 18
  if k % 5 == 0 then print(yc, wait(0)) end
end
print(wait(0))

--@ chunk 259
blend(sideMask, {angle=0.0, clip=sideMask})
print(wait(0))
blend(sideMask * rect(-10, 497, 1020, 150), {angle=0.006, clip=sideMask})
print(wait(0))

--@ chunk 260
print(wait(10*24*60))
print(drying(700,700), drying(300,600), drying(150,650), drying(800,480))

--@ chunk 261
-- cloud-blob plan: back-to-front groups; check that they cover the old cloud R1
BL = {
  -- group 1 (back, top/right)
  {g=1, cx=985, cy=70,  rx=100, ry=62},
  {g=1, cx=880, cy=62,  rx=78,  ry=48},
  {g=1, cx=1010,cy=165, rx=95,  ry=74},
  {g=1, cx=905, cy=145, rx=90,  ry=64},
  {g=1, cx=800, cy=112, rx=62,  ry=44},
  -- group 2 (middle)
  {g=2, cx=720, cy=165, rx=68,  ry=44},
  {g=2, cx=815, cy=182, rx=76,  ry=54},
  {g=2, cx=925, cy=212, rx=82,  ry=58},
  {g=2, cx=1012,cy=235, rx=60,  ry=48},
  -- group 3 (front, lit)
  {g=3, cx=622, cy=232, rx=50,  ry=32},
  {g=3, cx=705, cy=232, rx=56,  ry=34},
  {g=3, cx=795, cy=238, rx=64,  ry=36},
  {g=3, cx=890, cy=246, rx=66,  ry=36},
  {g=3, cx=968, cy=252, rx=58,  ry=32},
}
local U
for i, b in ipairs(BL) do
  b.m = ellipse(b.cx, b.cy, b.rx, b.ry):roughen(2.5, 30, 500 + i)
  U = U and (U + b.m) or b.m
end
blobU = U
left = R1 - U
print("R1 area", R1:area(), "blobs", U:area(), "leftover", left:area())
-- ascii map of leftover
for y = 10, 290, 10 do
  local row = {}
  for x = 520, 1040, 8 do
    local inR = R1:at(x, y) > 0.5
    local inU = U:at(x, y) > 0.5
    row[#row+1] = (inR and inU) and "#" or (inR and "L" or (inU and "+" or "."))
  end
  print(string.format("%3d ", y) .. table.concat(row))
end

--@ chunk 262
BL = {
  -- group 1 (back, top/right)
  {g=1, cx=985, cy=70,  rx=100, ry=62},
  {g=1, cx=880, cy=62,  rx=78,  ry=48},
  {g=1, cx=1010,cy=165, rx=95,  ry=74},
  {g=1, cx=905, cy=145, rx=90,  ry=64},
  {g=1, cx=800, cy=110, rx=62,  ry=44},
  -- group 2 (middle)
  {g=2, cx=742, cy=136, rx=52,  ry=40},
  {g=2, cx=720, cy=180, rx=64,  ry=44},
  {g=2, cx=815, cy=184, rx=76,  ry=54},
  {g=2, cx=925, cy=212, rx=82,  ry=54},
  {g=2, cx=1012,cy=232, rx=60,  ry=44},
  -- group 3 (front, lit)
  {g=3, cx=655, cy=205, rx=48,  ry=34},
  {g=3, cx=600, cy=228, rx=42,  ry=26},
  {g=3, cx=690, cy=228, rx=54,  ry=32},
  {g=3, cx=790, cy=236, rx=64,  ry=32},
  {g=3, cx=890, cy=242, rx=66,  ry=30},
  {g=3, cx=970, cy=246, rx=58,  ry=28},
}
local U
for i, b in ipairs(BL) do
  b.m = ellipse(b.cx, b.cy, b.rx, b.ry):roughen(2.5, 30, 500 + i)
  U = U and (U + b.m) or b.m
end
blobU = U
left = R1 - U
print("R1 area", R1:area(), "blobs", U:area(), "leftover", left:area())
for y = 10, 290, 10 do
  local row = {}
  for x = 520, 1040, 8 do
    local inR = R1:at(x, y) > 0.5
    local inU = U:at(x, y) > 0.5
    row[#row+1] = (inR and inU) and "#" or (inR and "L" or (inU and "+" or "."))
  end
  print(string.format("%3d ", y) .. table.concat(row))
end

--@ chunk 263
BL = {
  -- group 1 (back, top/right)
  {g=1, cx=985, cy=70,  rx=100, ry=62},
  {g=1, cx=880, cy=62,  rx=78,  ry=48},
  {g=1, cx=1010,cy=165, rx=95,  ry=74},
  {g=1, cx=905, cy=145, rx=90,  ry=64},
  {g=1, cx=800, cy=108, rx=62,  ry=44},
  {g=1, cx=748, cy=98,  rx=44,  ry=34},
  -- group 2 (middle)
  {g=2, cx=742, cy=140, rx=52,  ry=40},
  {g=2, cx=688, cy=152, rx=48,  ry=40},
  {g=2, cx=720, cy=186, rx=64,  ry=44},
  {g=2, cx=815, cy=186, rx=76,  ry=54},
  {g=2, cx=925, cy=214, rx=82,  ry=54},
  {g=2, cx=1012,cy=234, rx=60,  ry=44},
  -- group 3 (front, lit)
  {g=3, cx=642, cy=184, rx=44,  ry=34},
  {g=3, cx=655, cy=210, rx=48,  ry=32},
  {g=3, cx=600, cy=230, rx=44,  ry=26},
  {g=3, cx=690, cy=232, rx=54,  ry=32},
  {g=3, cx=790, cy=240, rx=64,  ry=32},
  {g=3, cx=890, cy=246, rx=66,  ry=30},
  {g=3, cx=970, cy=250, rx=58,  ry=28},
}
local U
for i, b in ipairs(BL) do
  b.m = ellipse(b.cx, b.cy, b.rx, b.ry):roughen(2.5, 30, 500 + i)
  U = U and (U + b.m) or b.m
end
blobU = U
left = R1 - U
print("R1 area", R1:area(), "blobs", U:area(), "leftover", left:area())
for y = 90, 280, 10 do
  local row = {}
  for x = 520, 1040, 8 do
    local inR = R1:at(x, y) > 0.5
    local inU = U:at(x, y) > 0.5
    row[#row+1] = (inR and inU) and "#" or (inR and "L" or (inU and "+" or "."))
  end
  print(string.format("%3d ", y) .. table.concat(row))
end

--@ chunk 264
-- two more small blobs to cover the tip and the bottom remnant
BL[#BL+1] = {g=3, cx=572, cy=236, rx=34, ry=16}
BL[#BL+1] = {g=3, cx=830, cy=258, rx=56, ry=16}
for i = #BL - 1, #BL do BL[i].m = ellipse(BL[i].cx, BL[i].cy, BL[i].rx, BL[i].ry):roughen(2.5, 30, 500 + i) end

LX, LY = -0.75, 0.66      -- direction toward the sun (from the cloud): left and down
cS = pile{{"lead white",2.2},{"smalt",1.8},{"red earth",0.7},{"raw umber",0.4},{"cobalt blue",0.3}, medium=0.04}
cM = pile{{"lead white",3.4},{"red earth",0.9},{"smalt",0.8},{"vermilion",0.2},{"yellow ochre",0.2}, medium=0.04}
cL = pile{{"lead white",4.8},{"vermilion",0.55},{"yellow ochre",0.5},{"chrome yellow",0.25}, medium=0.04}
cH = pile{{"lead white",6},{"chrome yellow",0.8},{"vermilion",0.3}, medium=0.04}
cD = pile{{"lead white",1.6},{"smalt",2.0},{"raw umber",0.6},{"red earth",0.6},{"cobalt blue",0.4},{"Prussian blue",0.1}, medium=0.04}
gshift = {0.35, 0.15, 0.0}

function paint_blob(b, seed)
  local cx, cy, rx, ry = b.cx, b.cy, b.rx, b.ry
  local r = math.sqrt(rx * ry)
  local sh = gshift[b.g]
  -- t: -1 on the lit (sun-facing) side .. +1 on the shadow side; shifted by group
  local function t(x, y) return clamp(((x - cx) * (-LX) + (y - cy) * (-LY)) / r + sh, -1.2, 1.4) end
  local m = b.m
  local ang = 0.85
  -- shadow tone
  work(m, {hand="body", tool={kind="filbert", width=14}, pile=cS, angle=ang, angle_jitter=0.15, length={20,60}, coverage=3.4, fill=true, clip=m, seed=seed,
       load_at=function(x, y) return smoothstep(-0.25, 0.7, t(x, y)) * 0.95 + 0.05 end,
       edge=function(x, y) return smoothstep(-0.2, 0.9, t(x, y)) * 0.7 end})
  -- mid tone
  work(m, {hand="body", tool={kind="filbert", width=12}, pile=cM, angle=ang, angle_jitter=0.15, length={16,50}, coverage=3.0, fill=true, clip=m, seed=seed + 1,
       load_at=function(x, y) local tt = t(x, y); return clamp(1.15 - math.abs(tt - 0.0) / 0.75, 0, 1) end})
  -- light tone
  work(m, {hand="body", tool={kind="filbert", width=10}, pile=cL, angle=ang, angle_jitter=0.15, length={14,40}, coverage=3.0, fill=true, clip=m, seed=seed + 2,
       load_at=function(x, y) return 1 - smoothstep(-0.9, 0.05, t(x, y)) end})
end
-- group 1: back
for i, b in ipairs(BL) do
  if b.g == 1 then paint_blob(b, 600000 + i * 10) end
end
print(wait(0))

--@ chunk 265
-- 1. base coat over the whole old cloud (opaque mid warm-grey), edges a little proud of the old rim
cBase = pile{{"lead white",3.6},{"red earth",0.8},{"smalt",1.0},{"vermilion",0.15},{"yellow ochre",0.2}, medium=0.04}
R1big = R1:grow(3)
work(R1big, {hand="body", tool={kind="filbert", width=20}, pile=cBase, angle=0.12, angle_jitter=0.1, length={50,140}, coverage=5, fill=true, clip=R1big, seed=700001, mix_jitter=0.3})
print(wait(0))
print(drying(800, 150))

--@ chunk 266
-- cover the leftover egg tops too, wet, same base
covM = (blobU + R1big) - R1big:shrink(2)
work(covM, {hand="body", tool={kind="filbert", width=16}, pile=cBase, angle=0.12, angle_jitter=0.1, length={40,120}, coverage=5, fill=true, seed=700002, mix_jitter=0.3, edge="soft",
     clip=(blobU + R1big)})
print(wait(0))
-- 2. modelling arcs, all on the wet base
function arc_pts(cx, cy, rx, ry, a0, a1, n, wob, seed)
  local pts = {}
  for i = 0, n do
    local a = lerp(a0, a1, i / n)
    local k = 1 + wob * math.sin(i * 1.9 + seed)
    pts[#pts+1] = {cx + rx * k * math.cos(a), cy + ry * k * math.sin(a)}
  end
  return pts
end
local bsh = brush("filbert", 13)
local blt = brush("filbert", 10)
local brm = brush{kind="round", width=3.2, point=0.5}
local phi_s = math.atan(-0.66, 0.75)      -- toward the shadow side (up and right)
local phi_l = phi_s + math.pi             -- toward the sun (down and left)
for i, b in ipairs(BL) do
  local sd = i * 7.3
  local k = ({0.8, 0.6, 0.45})[b.g]     -- how far the shadow reaches: back lobes are darker
  -- shadow arcs: three, from outside inward
  for j = 1, 3 do
    local f = ({0.88, 0.66, 0.44})[j]
    bsh:reload(j == 1 and cD or cS, 0.9)
    local span = (1.0 + 0.25 * k) * (1.1 - 0.2 * j)
    bsh:stroke(arc_pts(b.cx, b.cy, b.rx * f, b.ry * f, phi_s - span, phi_s + span * 0.8, 9, 0.04, sd + j), {pressure={0.75, 0.4}, ramps={0.1, 0.5}})
  end
  -- light arcs: two
  for j = 1, 2 do
    local f = ({0.86, 0.6})[j]
    blt:reload(cL, 0.9)
    blt:stroke(arc_pts(b.cx, b.cy, b.rx * f, b.ry * f, phi_l - 0.85, phi_l + 0.7, 9, 0.04, sd + 3 + j), {pressure={0.7, 0.4}, ramps={0.1, 0.5}})
  end
  -- gold rim, thin
  brm:reload(cH, 0.8)
  brm:stroke(arc_pts(b.cx, b.cy, b.rx * 0.97, b.ry * 0.97, phi_l - 0.6, phi_l + 0.45, 8, 0.02, sd + 6), {pressure={0.6, 0.6}, ramps={0.2, 0.4}})
end
print(wait(0))

--@ chunk 267
print(drying(800, 150), drying(700, 200))
blend(R1big, {angle=0.9})
print(wait(0))
blend(R1big, {angle=-0.5})
print(wait(0))

--@ chunk 268
print(wait(10*24*60))
print(drying(800, 150), drying(700, 200))

--@ chunk 269
print(type(shoreM2), type(landMask), type(grove2), type(tall1), type(shoreMask), type(mill_tower))
local U = shoreM2 + landMask + grove2 + tall1
IU = U
for y = 395, 500, 5 do
  local row = {}
  for x = 640, 1030, 5 do
    local a = shoreM2:at(x, y) > 0.5
    local b = landMask:at(x, y) > 0.5
    local c = (grove2:at(x, y) > 0.5) or (tall1:at(x, y) > 0.5)
    row[#row+1] = (a and b) and "#" or (a and "a" or (b and "b" or (c and "c" or ".")))
  end
  print(string.format("%3d ", y) .. table.concat(row))
end

--@ chunk 270
HZ = 497
isl_dark = pile{{"raw umber",2.2},{"bone black",1.1},{"smalt",1.2},{"red earth",0.6},{"lead white",0.5},{"Prussian blue",0.15}, medium=0.02}
isl_dark2 = pile{{"raw umber",2.0},{"bone black",0.8},{"smalt",1.5},{"red earth",0.9},{"lead white",0.9}, medium=0.02}
IUg = IU * rect(600, 380, 440, HZ - 380 + 2)
work(IUg, {hand="body", tool={kind="filbert", width=6}, pile=isl_dark, length={10,30}, angle=0.0, angle_jitter=0.3, coverage=9, fill=true, clip=IUg, mix_jitter=0.15, seed=800001})
print(wait(0))
-- mill and tower again, dark, crisp
mill_tower = poly({{795,487},{805,487},{803,459},{797,459}})
mill_cap   = poly({{796.4,459.4},{803.6,459.4},{800,453.2}})
work(mill_tower, {hand="detail", tool={kind="round", width=2.2}, pile=isl_dark, coverage=8, clip=mill_tower, seed=800011})
work(mill_cap,   {hand="detail", tool={kind="round", width=1.8}, pile=isl_dark, coverage=8, clip=mill_cap, seed=800012})
blades = {
  poly({{805.49,453.38},{820.23,443.04},{822.53,446.32},{807.78,456.65}}),
  poly({{802.63,461.49},{812.96,476.23},{809.68,478.53},{799.35,463.78}}),
  poly({{794.51,458.63},{779.77,468.96},{777.47,465.68},{792.22,455.35}}),
  poly({{797.38,450.51},{787.04,435.77},{790.32,433.47},{800.65,448.22}}),
}
sailP = pile{{"raw umber",2},{"bone black",1.2},{"smalt",0.5}, medium=0.25}
for i, b in ipairs(blades) do
  work(b, {hand="detail", tool={kind="round", width=1.4}, pile=sailP, coverage=5, clip=b, seed=800020 + i})
end
armB = brush{kind="round", width=1.5, point=0.6}
local pr = armB:pressure_for(1.2)
local tips = {{819.7,442.2},{813.8,475.7},{780.3,469.8},{786.2,436.3}}
for i, t in ipairs(tips) do
  armB:load(isl_dark, 1.0)
  armB:stroke({{800,456},{t[1],t[2]}}, {pressure={pr, pr}})
end
print(wait(0))

--@ chunk 271
isl_k1 = pile{{"raw umber",2.0},{"bone black",2.0},{"Prussian blue",0.35},{"red earth",0.35}, medium=0.02}
isl_k2 = pile{{"raw umber",2.2},{"bone black",1.4},{"smalt",0.7},{"red earth",0.6}, medium=0.02}
IUg2 = IU * rect(600, 380, 440, HZ - 380 + 2)
work(IUg2, {hand="body", tool={kind="filbert", width=6}, pile=isl_k1, length={10,30}, angle=0.0, angle_jitter=0.3, coverage=10, fill=true, clip=IUg2, mix_jitter=0.1, seed=810001})
print(wait(0))
-- mill and its sails once more
mill_tower = poly({{795,487},{805,487},{803,459},{797,459}})
mill_cap   = poly({{796.4,459.4},{803.6,459.4},{800,453.2}})
for i, t in ipairs({mill_tower, mill_cap}) do
  work(t, {hand="detail", tool={kind="round", width=2.0}, pile=isl_k1, coverage=8, clip=t, seed=810010 + i})
end
blades = {
  poly({{805.49,453.38},{820.23,443.04},{822.53,446.32},{807.78,456.65}}),
  poly({{802.63,461.49},{812.96,476.23},{809.68,478.53},{799.35,463.78}}),
  poly({{794.51,458.63},{779.77,468.96},{777.47,465.68},{792.22,455.35}}),
  poly({{797.38,450.51},{787.04,435.77},{790.32,433.47},{800.65,448.22}}),
}
for i, b in ipairs(blades) do
  work(b, {hand="detail", tool={kind="round", width=1.4}, pile=isl_k1, coverage=6, clip=b, seed=810020 + i})
end
armB = brush{kind="round", width=1.5, point=0.6}
local pr = armB:pressure_for(1.2)
local tips = {{819.7,442.2},{813.8,475.7},{780.3,469.8},{786.2,436.3}}
for i, t in ipairs(tips) do
  armB:load(isl_k1, 1.0)
  armB:stroke({{800,456},{t[1],t[2]}}, {pressure={pr, pr}})
end
print(wait(0))

--@ chunk 272
cap = outline{pts={{867,441},{871,433},{877,427},{884,422},{892,419.5},{900,419.5},{908,422},{915,426.5},{921,432},{926,438},{930,444},{930,456},{867,456}}, closed=true, char="firm", amount=0.15, lobe=6, seed=9101}:mask()
blob = ellipse(822, 472, 8, 7) * rect(800, 455, 40, 30)
work(cap + blob, {hand="body", tool={kind="filbert", width=5}, pile=isl_k1, length={6,16}, angle=0.0, angle_jitter=0.5, coverage=10, fill=true, clip=cap + blob, seed=811001})
print(wait(0))

--@ chunk 273
CL = R1big + blobU
print("cloud area", CL:area())
-- TEST at the cloud's left tip: wet base + stipple modelling
tip = CL * rect(540, 170, 130, 120)
cBaseM = pile{{"lead white",3.0},{"red earth",0.6},{"smalt",1.3},{"vermilion",0.1}, medium=0.05}
work(tip, {hand="body", tool={kind="filbert", width=14}, pile=cBaseM, angle=0.1, angle_jitter=0.1, length={30,80}, coverage=4, fill=true, clip=tip, seed=900001})
print(wait(0))
cWarmL = pile{{"lead white",4.6},{"vermilion",0.5},{"yellow ochre",0.5},{"chrome yellow",0.2}, medium=0.05}
cCoolS = pile{{"lead white",2.2},{"smalt",2.0},{"raw umber",0.4},{"red earth",0.5},{"cobalt blue",0.4}, medium=0.05}
lowleft = tip * below({{540,246},{600,250},{670,262}})
stipple(lowleft, {pile=cWarmL, width=7, coverage=1.6, feather=1, cluster=0.4, seed=900002})
upright = tip * above({{540,222},{600,205},{670,190}})
stipple(upright, {pile=cCoolS, width=7, coverage=1.2, feather=1, cluster=0.4, seed=900003})
print(wait(0))

--@ chunk 274
-- Cloud colour stops (T = 0 cool dark top .. 1 gold underside)
CST = {
 {0.00, {{"lead white",1.5},{"smalt",2.3},{"cobalt blue",0.6},{"raw umber",0.3},{"red earth",0.3}}},
 {0.22, {{"lead white",2.2},{"smalt",1.9},{"cobalt blue",0.3},{"red earth",0.6},{"raw umber",0.25}}},
 {0.45, {{"lead white",3.0},{"smalt",1.1},{"red earth",0.95},{"raw umber",0.15},{"vermilion",0.12}}},
 {0.68, {{"lead white",4.0},{"red earth",0.6},{"vermilion",0.55},{"yellow ochre",0.4},{"smalt",0.35}}},
 {0.86, {{"lead white",4.8},{"vermilion",0.6},{"yellow ochre",0.5},{"chrome yellow",0.35}}},
 {1.00, {{"lead white",5.4},{"chrome yellow",0.8},{"vermilion",0.5},{"yellow ochre",0.2}}},
}
function cst(T, med)
  T = clamp(T, 0, 1)
  for i = 1, #CST - 1 do
    local a, b = CST[i], CST[i+1]
    if T <= b[1] or i == #CST - 1 then
      local t = clamp((T - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[2], b[2], t, med or 0.05)
    end
  end
end
function Tcloud(x, y)
  local lit = 1 - smoothstep(560, 980, x)
  return clamp((y - 8 + 70 * lit) / 285, 0, 1)
end
CLsoft = CL:blur(5)
local nx, ny = 6, 11
local x0, x1, y0, y1 = 530, 1030, 5, 292
for j = 0, ny - 1 do
  local cy = lerp(y0, y1, j / (ny - 1))
  local hh = (y1 - y0) / (ny - 1) * 1.25
  for i = 0, nx - 1 do
    local cx = lerp(x0, x1, i / (nx - 1))
    local hw = (x1 - x0) / (nx - 1) * 1.25
    local m = rect(cx - hw, cy - hh, 2 * hw, 2 * hh) * CLsoft
    if m:area() > 50 then
      hb(m, cst(Tcloud(cx, cy), 0.05), 16, 910000 + j * 20 + i, 5.0, {40,110}, {clip=CLsoft, angle_jitter=0.02, mix_jitter=0.3, load_at=function(x, y)
        return clamp(1.4 - math.abs(x - cx) / hw, 0.04, 1) * clamp(1.4 - math.abs(y - cy) / hh, 0.04, 1) end})
    end
  end
  if j % 3 == 2 then print(j, wait(0)) end
end
print(wait(0))

--@ chunk 275
print(wait(12*24*60))
print(drying(800, 150), drying(600, 240), drying(800, 250))

--@ chunk 276
water = rect(-10, 497, 1020, 260)
swFix = (rect(528, 606, 490, 140)):blur(8) * water
print("swFix", swFix:area())
local yc = 748
local k = 0
while yc >= 606 do
  k = k + 1
  local hh = 22
  local m = rect(520, yc - hh, 500, 2 * hh) * swFix
  hb(m, wsc(WR2, yc, 0.03), 14, 500000 + k * 2, 9.0, {50,140}, {clip=swFix, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y)
      return clamp(1.5 - math.abs(y - yc) / hh, 0.05, 1) * clamp((x - 520) / 40, 0.05, 1) end})
  yc = yc - 11
end
print(wait(0))
blend(swFix, {angle=0.0, clip=swFix})
print(wait(0))

--@ chunk 277
water = rect(-10, 497, 1020, 260)
local yc = 748
local k = 0
while yc >= 640 do
  k = k + 1
  sidestrip2(yc, 26, 18, 520000 + k * 3, 8.5)
  yc = yc - 12
end
print(yc, wait(0))

--@ chunk 278
local yc = 628
local k = 100
while yc >= 546 do
  k = k + 1
  local cov = lerp(2.2, 8.0, smoothstep(552, 640, yc))
  sidestrip2(yc, 26, 18, 520000 + k * 3, cov)
  yc = yc - 12
end
print(yc, wait(0))
blend(sideMask * rect(-10, 540, 1020, 210), {angle=0.0, clip=sideMask})
print(wait(0))

--@ chunk 279
CST2 = {
 {0.00, {{"lead white",1.3},{"smalt",2.4},{"cobalt blue",0.7},{"raw umber",0.35},{"red earth",0.25},{"Prussian blue",0.05}}},
 {0.35, {{"lead white",1.9},{"smalt",1.9},{"cobalt blue",0.45},{"red earth",0.55},{"raw umber",0.3}}},
 {0.62, {{"lead white",2.6},{"smalt",1.4},{"red earth",0.9},{"raw umber",0.25},{"vermilion",0.1}}},
 {0.82, {{"lead white",3.4},{"red earth",0.9},{"smalt",0.9},{"vermilion",0.25},{"yellow ochre",0.2}}},
 {1.00, {{"lead white",4.2},{"vermilion",0.6},{"yellow ochre",0.5},{"red earth",0.3}}},
}
function cst2(T, med)
  T = clamp(T, 0, 1)
  for i = 1, #CST2 - 1 do
    local a, b = CST2[i], CST2[i+1]
    if T <= b[1] or i == #CST2 - 1 then
      local t = clamp((T - a[1]) / (b[1] - a[1]), 0, 1)
      return mixp(a[2], b[2], t, med or 0.04)
    end
  end
end
CLsoft = CL:blur(3)
local nx, ny = 6, 11
local x0, x1, y0, y1 = 530, 1030, 5, 292
for j = 0, ny - 1 do
  local cy = lerp(y0, y1, j / (ny - 1))
  local hh = (y1 - y0) / (ny - 1) * 1.3
  for i = 0, nx - 1 do
    local cx = lerp(x0, x1, i / (nx - 1))
    local hw = (x1 - x0) / (nx - 1) * 1.3
    local m = rect(cx - hw, cy - hh, 2 * hw, 2 * hh) * CLsoft
    if m:area() > 50 then
      hb(m, cst2(Tcloud(cx, cy), 0.04), 16, 930000 + j * 20 + i, 5.5, {40,110}, {clip=CLsoft, angle_jitter=0.02, mix_jitter=0.25, load_at=function(x, y)
        return clamp(1.5 - math.abs(x - cx) / hw, 0.04, 1) * clamp(1.5 - math.abs(y - cy) / hh, 0.04, 1) end})
    end
  end
  if j % 3 == 2 then print(j, wait(0)) end
end
print(wait(0))
local bm = CL:shrink(2)
blend(bm, {angle=0.0})
print(wait(0))

--@ chunk 280
SW = {
 {{"bone black",1},{"lead white",2},{"smalt",1},{"red earth",0.5}},
 {{"bone black",1},{"lead white",1.5},{"cobalt blue",1},{"red earth",0.4}},
 {{"raw umber",1},{"Prussian blue",0.15},{"lead white",2},{"red earth",0.8},{"smalt",1}},
 {{"bone black",1},{"Prussian blue",0.2},{"lead white",2.5},{"red earth",1}},
 {{"bone black",0.6},{"smalt",2},{"red earth",0.8},{"lead white",1.5},{"cobalt blue",0.5}},
 {{"raw umber",1.5},{"bone black",0.5},{"lead white",2},{"smalt",1.5},{"vermilion",0.2}},
 {{"lead white",3},{"red earth",1},{"smalt",1},{"bone black",0.3}},
 {{"lead white",3},{"vermilion",0.5},{"red earth",0.6},{"cobalt blue",0.5},{"bone black",0.25}},
 {{"lead white",3.5},{"red earth",1},{"cobalt blue",0.7},{"vermilion",0.2}},
 {{"lead white",4.5},{"vermilion",0.5},{"yellow ochre",0.5},{"chrome yellow",0.2}},
 {{"lead white",6},{"chrome yellow",0.7},{"vermilion",0.3}},
 {{"Prussian blue",0.3},{"bone black",1},{"lead white",1.5},{"red earth",0.6}},
}
for i, t in ipairs(SW) do
  t.medium = 0.03
  local p = pile(t)
  local col = (i - 1) % 6
  local row = (i - 1) // 6
  local m = rect(700 + col * 50, 60 + row * 50, 46, 46)
  work(m, {hand="body", tool={kind="filbert", width=9}, pile=p, length={20,44}, angle=0.0, coverage=6, fill=true, clip=m, seed=940000 + i})
end
print(wait(0))

--@ chunk 281
SW2 = {
 {{"bone black",1},{"Prussian blue",0.15},{"red earth",0.6},{"lead white",0.6},{"smalt",0.5}},
 {{"bone black",1},{"cobalt blue",0.8},{"red earth",0.5},{"lead white",0.8}},
 {{"raw umber",1},{"bone black",0.7},{"smalt",1},{"red earth",0.6},{"lead white",0.6},{"Prussian blue",0.1}},
 {{"bone black",1},{"red earth",0.8},{"smalt",1},{"lead white",1.0},{"vermilion",0.15}},
 {{"Prussian blue",0.35},{"red earth",1},{"bone black",0.5},{"lead white",1}},
 {{"Prussian blue",0.3},{"raw umber",1},{"red earth",0.6},{"lead white",1.2}},
 {{"bone black",1},{"Prussian blue",0.25},{"red earth",0.5},{"lead white",1.6},{"vermilion",0.2}},
 {{"bone black",0.8},{"cobalt blue",1.2},{"red earth",0.7},{"lead white",1.5}},
 {{"bone black",0.5},{"Prussian blue",0.2},{"red earth",1.0},{"lead white",2.2},{"vermilion",0.3}},
 {{"bone black",0.4},{"red earth",1.0},{"lead white",3},{"vermilion",0.5},{"cobalt blue",0.5}},
 {{"raw umber",0.5},{"red earth",1.0},{"lead white",3.5},{"vermilion",0.6},{"yellow ochre",0.3}},
 {{"lead white",5},{"red earth",0.6},{"vermilion",0.7},{"yellow ochre",0.6},{"chrome yellow",0.3}},
}
for i, t in ipairs(SW2) do
  t.medium = 0.03
  local p = pile(t)
  local col = (i - 1) % 6
  local row = (i - 1) // 6
  local m = rect(700 + col * 50, 170 + row * 50, 46, 46)
  work(m, {hand="body", tool={kind="filbert", width=9}, pile=p, length={20,44}, angle=0.0, coverage=6, fill=true, clip=m, seed=941000 + i})
end
print(wait(0))

--@ chunk 282
-- 1. cover the swatch corner that lies outside the cloud with the sky gradient (all dry underneath)
CLc = CL
Sreg = (rect(690, 50, 320, 230) - CLc:grow(1)):blur(2)
print("Sreg area", Sreg:area())
local ycs = {58, 74, 90, 106, 122}
for i, yc in ipairs(ycs) do
  local hh = 18
  local m = rect(690, yc - hh, 320, 2 * hh) * Sreg
  hb(m, skyd(SKD2, yc, 0.03), 12, 950000 + i, 9.0, {40,120}, {clip=Sreg, angle_jitter=0.012, load_at=function(x, y) return clamp(1.5 - math.abs(y - yc) / hh, 0.05, 1) end})
end
print(wait(0))
blend(Sreg, {angle=0.0, clip=Sreg})
print(wait(0))

--@ chunk 283
cp = {
 d1 = pile{{"Prussian blue",0.3},{"red earth",1},{"bone black",0.5},{"lead white",1.1}, medium=0.04},
 d2 = pile{{"bone black",0.8},{"cobalt blue",1.2},{"red earth",0.7},{"lead white",1.5}, medium=0.04},
 m  = pile{{"bone black",0.5},{"Prussian blue",0.2},{"red earth",1.0},{"lead white",2.2},{"vermilion",0.3}, medium=0.04},
 l  = pile{{"raw umber",0.5},{"red earth",1.0},{"lead white",3.5},{"vermilion",0.6},{"yellow ochre",0.3}, medium=0.04},
 h  = pile{{"lead white",5},{"red earth",0.6},{"vermilion",0.7},{"yellow ochre",0.6},{"chrome yellow",0.3}, medium=0.04},
 g  = pile{{"lead white",6},{"chrome yellow",1.0},{"vermilion",0.35}, medium=0.04},
}
CLm = CL:shrink(0.5)
-- base coat: whole cloud, mid mauve, opaque; covers the swatches
work(CL, {hand="body", tool={kind="filbert", width=18}, pile=cp.m, angle=0.85, angle_jitter=0.15, length={40,110}, coverage=6, fill=true, clip=CL, seed=960001, mix_jitter=0.25})
print(wait(0))
-- lobes: {cx, cy, r, layer}
LOBES = {
 {985, 48, 72, 1}, {880, 62, 58, 1}, {790, 92, 46, 1}, {975, 135, 74, 1}, {868, 138, 62, 1},
 {745, 132, 44, 2}, {700, 168, 42, 2}, {800, 185, 54, 2}, {915, 190, 62, 2}, {1010, 215, 58, 2},
 {640, 212, 42, 3}, {595, 232, 34, 3}, {720, 226, 50, 3}, {830, 236, 58, 3}, {935, 246, 56, 3}, {1005, 262, 40, 3},
}
for i, L in ipairs(LOBES) do
  L.m = ellipse(L[1], L[2], L[3] * 1.12, L[3] * 0.92):roughen(2.5, 24, 970 + i) * CL
end
LX0, LY0 = -0.751, 0.660
lobe_noise = noise{seed=33, octaves=2, period=40, persistence=0.5}
function tri(v, c, h) return clamp(1 - math.abs(v - c) / h, 0, 1) end
function lobe_u(L, x, y)
  local cx, cy, r = L[1], L[2], L[3]
  local u = ((x - cx) * (-LX0) * -1 + (y - cy) * LY0) / r    -- +1 toward the sun (lower-left)
  local shift = clamp((cy - 130) / 220, -0.15, 0.45) - 0.08
  return u + shift + 0.22 * lobe_noise(x, y)
end
print(#LOBES, LOBES[1].m:area(), LOBES[10].m:area())
-- check the sign of u: at the lower-left of lobe 8 (800,185,54) u should be high
print(lobe_u(LOBES[8], 770, 210), lobe_u(LOBES[8], 830, 160))

--@ chunk 284
function paint_lobe(L, seed)
  local m = L.m
  local r = L[3]
  local wd = (r >= 55) and 12 or ((r >= 42) and 10 or 8)
  local U = function(x, y) return lobe_u(L, x, y) end
  local tones = {
    {p=cp.d1, w=function(u) return 1 - smoothstep(-0.78, -0.30, u) end},
    {p=cp.d2, w=function(u) return tri(u, -0.30, 0.42) end},
    {p=cp.m,  w=function(u) return tri(u, 0.08, 0.42) end},
    {p=cp.l,  w=function(u) return tri(u, 0.48, 0.38) end},
    {p=cp.h,  w=function(u) return tri(u, 0.80, 0.30) end},
    {p=cp.g,  w=function(u) return smoothstep(0.90, 1.05, u) end},
  }
  for k, t in ipairs(tones) do
    work(m, {hand="body", tool={kind="filbert", width=wd}, pile=t.p, angle=0.85, angle_jitter=0.25, length={12,40}, coverage=3.4, fill=true, clip=m, seed=seed + k, mix_jitter=0.25,
      load_at=function(x, y) return clamp(t.w(U(x, y)) * 1.35, 0, 1) end})
  end
end
for i, L in ipairs(LOBES) do
  if L[4] == 1 then paint_lobe(L, 980000 + i * 10) end
end
print(wait(0))
for i, L in ipairs(LOBES) do
  if L[4] == 2 then paint_lobe(L, 980000 + i * 10) end
end
print(wait(0))
for i, L in ipairs(LOBES) do
  if L[4] == 3 then paint_lobe(L, 980000 + i * 10) end
end
print(wait(0))

--@ chunk 285
print(wait(12*24*60))
print(drying(800,150), drying(700,230), drying(900,60), drying(300,300))

--@ chunk 286
-- bottom boundary of the cloud, per x
ybot_t = {}
for x = 520, 1040, 4 do
  local yy = 262
  for y = 310, 20, -1 do if CL:at(x, y) > 0.5 then yy = y break end end
  ybot_t[#ybot_t + 1] = {x, yy}
end
function ybot(x)
  local i = clamp(math.floor((x - 520) / 4) + 1, 1, #ybot_t - 1)
  local a, b = ybot_t[i], ybot_t[i + 1]
  local t = clamp((x - a[1]) / (b[1] - a[1]), 0, 1)
  return lerp(a[2], b[2], t)
end
-- terminator: hand-placed, irregular scallops (no two alike)
TP = {{530,236},{560,226},{590,217},{612,203},{633,209},{655,196},{676,182},{699,190},{724,166},{748,176},{771,155},{790,168},{818,138},{846,158},{869,128},{893,151},{924,120},{951,146},{983,116},{1010,140},{1040,108}}
function Tterm(x)
  if x <= TP[1][1] then return TP[1][2] end
  for i = 1, #TP - 1 do
    local a, b = TP[i], TP[i+1]
    if x <= b[1] then
      local t = (x - a[1]) / (b[1] - a[1])
      t = 0.5 - 0.5 * math.cos(math.pi * t)
      return lerp(a[2], b[2], t)
    end
  end
  return TP[#TP][2]
end
print(ybot(560), ybot(700), ybot(830), ybot(1000), Tterm(700), Tterm(830), Tterm(1000))
-- 1. body: cool violet-grey over the whole cloud; deeper at top / right
work(CL, {hand="body", tool={kind="filbert", width=18}, pile=cp.d2, angle=0.15, angle_jitter=0.15, length={40,110}, coverage=6, fill=true, clip=CL, seed=990001, mix_jitter=0.25})
work(CL, {hand="body", tool={kind="filbert", width=16}, pile=cp.d1, angle=0.2, angle_jitter=0.2, length={30,90}, coverage=3.4, fill=true, clip=CL, seed=990002, mix_jitter=0.25,
     load_at=function(x, y) return clamp(1.25 - (y - 10) / 150, 0, 1) * clamp(0.4 + (x - 560) / 420, 0.3, 1) end})
print(wait(0))

--@ chunk 287
function fz(x, y)
  local T, B = Tterm(x), ybot(x)
  local f = (y - T) / math.max(B - T, 12)
  return f, y - T
end
lit_tones = {
  {p=cp.m, w=function(f) return tri(f, 0.06, 0.28) end},
  {p=cp.l, w=function(f) return tri(f, 0.34, 0.30) end},
  {p=cp.h, w=function(f) return tri(f, 0.64, 0.30) end},
  {p=cp.g, w=function(f) return smoothstep(0.80, 1.0, f) end},
}
for k, t in ipairs(lit_tones) do
  work(CL, {hand="body", tool={kind="filbert", width=12}, pile=t.p, angle=0.06, angle_jitter=0.15, length={20,60}, coverage=3.6, fill=true, clip=CL, seed=990010 + k, mix_jitter=0.25,
     load_at=function(x, y)
        local f, s = fz(x, y)
        return clamp(t.w(f) * 1.3, 0, 1) * smoothstep(-14, 6, s)
     end})
end
print(wait(0))

--@ chunk 288
print(drying(800, 100), drying(800, 250))
local inner = CL:shrink(3)
blend(inner, {angle=0.06})
print(wait(0))
blend(inner, {angle=-0.05})
print(wait(0))

--@ chunk 289
HZ = 497
skyOnly = rect(-14, -14, 1028, HZ + 14)
coverIsl = (rect(838, 384, 200, 92)):blur(3) * skyOnly
print("coverIsl", coverIsl:area())
-- sky colours at the right, by y (same stops as the surrounding sky there)
function rsky(y) return skg(SKGR, y, 0.02) end
local ycs = {392, 408, 424, 440, 456, 470}
for i, yc in ipairs(ycs) do
  local hh = 18
  local m = rect(830, yc - hh, 210, 2 * hh) * coverIsl
  hb(m, rsky(yc), 12, 1100000 + i, 9.0, {40,120}, {clip=coverIsl, angle_jitter=0.012, load_at=function(x, y)
      local ry = clamp(1.6 - math.abs(y - yc) / hh, 0.05, 1)
      if yc <= 392 then ry = clamp((yc + 6 - y) / 6, 0.05, 1) * 0 + clamp(1.5 - math.abs(y - yc) / hh, 0.05, 1) end
      return ry * clamp((x - 836) / 16, 0.04, 1) end})
end
print(wait(0))
blend(coverIsl:grow(2), {angle=0.0, clip=coverIsl:grow(2)})
print(wait(0))

--@ chunk 290
HZ = 497
-- 1. little cover patch for the old round blob at the mill's right
patchB = (ellipse(823, 472, 13, 12)):blur(2) * skyOnly
for i, yc in ipairs({462, 472}) do
  hb(rect(806, yc - 12, 34, 24) * patchB, rsky(yc), 8, 1110000 + i, 9.0, {14,40}, {clip=patchB, angle_jitter=0.02})
end
print(wait(0))
-- 2. new headland
gnd = outline{pts={{640,HZ},{662,495},{690,492},{725,490},{760,488},{792,487},{815,485},{836,481},{852,476},{872,472},{896,470},{920,469},{950,468},{985,466},{1030,464},{1030,HZ}}, closed=true, char="firm", amount=0.3, lobe=10, seed=1201}:mask()
clumpA = outline{pts={{846,478},{849,467},{854,458},{858,451},{864,448},{868,443},{875,441},{880,436},{887,438},{892,435},{898,439},{902,446},{905,455},{908,463},{911,474}}, closed=true, char="firm", amount=0.35, lobe=7, seed=1202}:mask()
pop1 = outline{pts={{929,471},{928,456},{929,443},{931.5,433},{934.5,426},{936,433},{937.5,443},{938.5,456},{937.5,471}}, closed=true, char="firm", amount=0.25, lobe=5, seed=1203}:mask()
pop2 = outline{pts={{942,471},{941,459},{942,449},{944,442},{946.5,436},{948,442},{950,449},{951,459},{950,471}}, closed=true, char="firm", amount=0.25, lobe=5, seed=1204}:mask()
pop3 = outline{pts={{917,471},{916.5,461},{917.5,454},{919.5,449},{921,446},{923,450},{925,454},{926,461},{925,471}}, closed=true, char="firm", amount=0.25, lobe=5, seed=1205}:mask()
pop4 = outline{pts={{957,471},{956,462},{957,456},{959,451},{960.5,448},{962,452},{963.5,456},{964.5,462},{963.5,471}}, closed=true, char="firm", amount=0.25, lobe=5, seed=1206}:mask()
scrubR = outline{pts={{968,470},{974,462},{982,458},{991,460},{1000,456},{1010,459},{1020,455},{1030,458},{1030,472}}, closed=true, char="firm", amount=0.35, lobe=6, seed=1207}:mask()
isl2 = gnd + clumpA + pop1 + pop2 + pop3 + pop4 + scrubR
isl_k = pile{{"raw umber",2.0},{"bone black",2.0},{"Prussian blue",0.35},{"red earth",0.35}, medium=0.02}
work(isl2, {hand="body", tool={kind="filbert", width=5}, pile=isl_k, length={8,22}, angle=0.0, angle_jitter=0.5, coverage=9, fill=true, clip=isl2, seed=1210001, mix_jitter=0.1})
print(wait(0))

--@ chunk 291
HZ = 497
fillNotch = outline{pts={{796,HZ},{800,487},{810,483},{822,479},{834,476},{846,471},{852,466},{856,HZ}}, closed=true, char="firm", amount=0.25, lobe=8, seed=1301}:mask()
work(fillNotch, {hand="body", tool={kind="filbert", width=5}, pile=isl_k, length={8,22}, angle=0.0, angle_jitter=0.5, coverage=10, fill=true, clip=fillNotch, seed=1300001, mix_jitter=0.1})
print(wait(0))

--@ chunk 292
water = rect(-10, 497, 1020, 260)
PX = 300
function pw2(y) return 20 + (y - 497) * 0.40 end          -- glitter half-width by depth
function pd(x, y, s) local d = (x - PX) / (pw2(y) * (s or 1)); return math.exp(-d * d) end
PST = {
 {497, {{"lead white",8},{"chrome yellow",0.55},{"vermilion",0.06}}},
 {535, {{"lead white",6.5},{"yellow ochre",0.55},{"vermilion",0.3},{"chrome yellow",0.2}}},
 {585, {{"lead white",4.8},{"vermilion",0.45},{"yellow ochre",0.4},{"red earth",0.15}}},
 {645, {{"lead white",3.0},{"vermilion",0.35},{"red earth",0.55},{"smalt",0.6},{"raw umber",0.15}}},
 {745, {{"lead white",1.6},{"smalt",1.2},{"red earth",0.6},{"raw umber",0.55},{"vermilion",0.1}}},
}
function pstc(y, med) return wsc(PST, y, med or 0.04) end
-- base: opaque, feathered laterally by a gaussian load; strips overlap so it is one continuous coat
local yc = 748
local k = 0
while yc >= 498 do
  k = k + 1
  local hh = 24
  local m = rect(40, yc - hh, 520, 2 * hh) * water
  local rt = function(y)
    if yc < 505 then return clamp((y - 497) / 8, 0.05, 1) end
    return clamp(1.5 - math.abs(y - yc) / hh, 0.05, 1)
  end
  hb(m, pstc(yc), 14, 1400000 + k * 2, 7.0, {50,150}, {clip=water, edge="found", angle_jitter=0.015, mix_jitter=0.2,
     load_at=function(x, y) return rt(y) * clamp(pd(x, y, 1.15) * 1.25, 0.02, 1) end})
  yc = yc - 12
  if k % 6 == 0 then print(yc, wait(0)) end
end
print(wait(0))

--@ chunk 293
print(wait(10*24*60))
print(drying(300,600), drying(800,150), drying(800,250), drying(700,700))

--@ chunk 294
water = rect(-14, 497, 1030, 260)
WL3 = {
 {497, {{"lead white",6},{"chrome yellow",0.7},{"vermilion",0.15},{"yellow ochre",0.2}}},
 {512, {{"lead white",5.4},{"yellow ochre",0.6},{"vermilion",0.25},{"pale smalt",0.5}}},
 {536, {{"lead white",4.0},{"pale smalt",1.4},{"vermilion",0.3},{"red earth",0.15},{"yellow ochre",0.2}}},
 {572, {{"lead white",2.6},{"cobalt blue",0.6},{"smalt",1.3},{"red earth",0.35},{"raw umber",0.15}}},
 {616, {{"lead white",1.5},{"cobalt blue",0.7},{"smalt",1.7},{"green earth",0.3},{"raw umber",0.4}}},
 {666, {{"lead white",0.8},{"Prussian blue",0.3},{"smalt",1.6},{"green earth",0.4},{"raw umber",0.7}}},
 {745, {{"Prussian blue",0.5},{"raw umber",1.2},{"smalt",1.0},{"green earth",0.4},{"bone black",0.3},{"lead white",0.25}}},
}
WR3 = {
 {497, {{"lead white",4.6},{"vermilion",0.7},{"chrome yellow",0.4},{"red earth",0.12}}},
 {512, {{"lead white",4.5},{"vermilion",0.5},{"pale smalt",0.7},{"yellow ochre",0.2}}},
 {536, {{"lead white",3.8},{"pale smalt",1.3},{"vermilion",0.35},{"red earth",0.3}}},
 {572, {{"lead white",2.7},{"smalt",1.3},{"red earth",0.6},{"cobalt blue",0.4},{"vermilion",0.15}}},
 {616, WL3[5][2]}, {666, WL3[6][2]}, {745, WL3[7][2]},
}
function xm3(x) return smoothstep(380, 780, x) end
function strip3(yc, hh, wd, seed, cov)
  local m = rect(-14, yc - hh, 1030, 2 * hh) * water
  local rt = function(y)
    if yc < 505 then return clamp((y - 497) / 8, 0.05, 1) end
    return clamp(1.5 - math.abs(y - yc) / hh, 0.05, 1)
  end
  hb(m, wsc(WL3, yc, 0.05), wd, seed, cov, {60,200}, {clip=water, edge="found", angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * (1 - xm3(x)) + 0.02 end})
  hb(m, wsc(WR3, yc, 0.05), wd, seed + 1, cov, {60,200}, {clip=water, edge="found", angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * xm3(x) + 0.02 end})
end
-- bottom to top; dark paint first so the light paint above stays clean
local yc = 748
local k = 0
while yc >= 640 do
  k = k + 1
  strip3(yc, 26, 20, 1500000 + k * 3, 6.5)
  yc = yc - 14
end
print(yc, wait(0))

--@ chunk 295
local yc = 634
local k = 100
while yc >= 498 do
  k = k + 1
  local cov = lerp(3.0, 6.5, smoothstep(500, 620, yc))
  strip3(yc, 26, 18, 1500000 + k * 3, cov)
  yc = yc - 13
end
print(yc, wait(0))

--@ chunk 296
print(drying(300,600), drying(300,700))
blend(water, {angle=0.0, clip=water})
print(wait(0))
blend(water, {angle=0.006, clip=water})
print(wait(0))
blend(water, {angle=-0.006, clip=water})
print(wait(0))

--@ chunk 297
print(wait(10*24*60))
print(drying(300,600), drying(300,700), drying(300,520))

--@ chunk 298
HZ = 497
skyOnly = rect(-14, -14, 1028, HZ + 14)
-- clean the specks above the island with sky colour: one coat, overlapping strips, feathered at the sides by a blurred mask
spk = (rect(806, 380, 224, 92)):blur(4) * skyOnly * (-isl2:shrink(1)) * (-(mill_tower + mill_cap))
print(spk:area())
local ycs = {390, 406, 422, 438, 454, 468}
for i, yc in ipairs(ycs) do
  local hh = 18
  local m = rect(800, yc - hh, 240, 2 * hh) * spk
  hb(m, rsky(yc), 10, 1600000 + i, 10.0, {30,90}, {clip=spk, angle_jitter=0.012, load_at=function(x, y) return clamp(1.6 - math.abs(y - yc) / hh, 0.05, 1) end})
end
print(wait(0))
blend(spk, {angle=0.0, clip=spk})
print(wait(0))

--@ chunk 299
local sm = rect(788, 384, 34, 90):blur(6) * skyOnly * (-(mill_tower + mill_cap))
blend(sm, {angle=0.0, clip=false})
print(wait(0))
-- Sun disc: a half-disc sitting on the horizon, painted on dry paint. Clip to the sky side so the horizon stays crisp.
sunD = ellipse(300, 497, 13, 13) * skyOnly
sun_c = pile{{"lead white",12},{"chrome yellow",0.12}, medium=0.02}
sun_e = pile{{"lead white",9},{"chrome yellow",0.5}, medium=0.02}
work(ellipse(300, 497, 20, 20) * skyOnly, {hand="body", tool={kind="filbert", width=5}, pile=sun_e, length={6,20}, angle=0.0, angle_jitter=0.05, coverage=4.0, fill=true, clip=skyOnly, edge="soft", seed=1700001,
     load_at=function(x, y) local r = math.sqrt((x-300)^2 + (y-497)^2); return clamp(1.3 - r/20, 0, 1) end})
work(sunD, {hand="body", tool={kind="filbert", width=4}, pile=sun_c, length={6,18}, angle=0.0, angle_jitter=0.05, coverage=6.0, fill=true, clip=skyOnly, edge="firm", seed=1700002})
print(wait(0))

--@ chunk 300
local hm = (ellipse(300, 497, 30, 27)):blur(3) * skyOnly
blend(hm, {angle=0.0, clip=skyOnly})
print(wait(0))
blend(hm, {angle=math.pi/2, clip=skyOnly})
print(wait(0))
blend(hm, {angle=0.0, clip=skyOnly})
print(wait(0))

--@ chunk 301
print(wait(10*24*60))
coverP = pile{{"lead white",3.2},{"smalt",1.2},{"cobalt blue",0.9},{"raw umber",0.15}, medium=0.05}
CLg = CL:grow(5)
work(CLg, {hand="body", tool={kind="filbert", width=22}, pile=coverP, angle=0.03, angle_jitter=0.05, length={60,150}, coverage=5, fill=true, clip=CLg, seed=1800001, mix_jitter=0.2})
print(wait(0))
local ycs = {0, 26, 52, 78, 104, 130, 156, 182, 208, 234, 260, 286, 312}
for i, yc in ipairs(ycs) do
  local hh = 34
  local m = rect(-14, yc - hh, 1028, 2 * hh)
  local rt = function(y)
    local r = clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
    if yc == 0 then r = 1 end
    return r
  end
  if yc <= 190 then
    hb(m, skyd(SKD2, yc, 0.12), 28, 1810000 + i * 2, 4.8, {140,320}, {angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) end})
  else
    local tx = function(x) return smoothstep(350, 850, x) end
    hb(m, skyd(SKD2, yc, 0.12), 28, 1810000 + i * 2, 4.8, {140,320}, {angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * (1 - tx(x)) + 0.03 end})
    hb(m, skyd(SKD2R, yc, 0.12), 28, 1810001 + i * 2, 4.8, {140,320}, {angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * tx(x) + 0.03 end})
  end
  if i % 4 == 0 then print(yc, wait(0)) end
end
print(wait(0))

--@ chunk 302
HZ = 497
skyOnly = rect(-14, -14, 1028, HZ + 14)
local k = 0
local yc = 324
while yc <= 432 do
  k = k + 1
  local hh = 28
  local m = rect(-14, yc - hh, 1028, 2 * hh) * skyOnly
  local rt = function(y) return clamp(1.4 - math.abs(y - yc) / hh, 0.05, 1) end
  hb(m, slcol(yc, 0.06), 26, 1820000 + k * 2, 5.0, {140,320}, {clip=skyOnly, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * (1 - smoothstep(380, 700, x)) + 0.03 end})
  hb(m, srcol(yc, 0.06), 26, 1820001 + k * 2, 5.0, {140,320}, {clip=skyOnly, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * smoothstep(380, 700, x) + 0.03 end})
  yc = yc + 12
end
print(wait(0))
local bm = (rect(-14, 296, 1028, 160)):blur(14)
blend(bm, {angle=0.0, clip=skyOnly})
print(wait(0))
blend(bm, {angle=0.008, clip=skyOnly})
print(wait(0))

--@ chunk 303
print(wait(10*24*60))
print(drying(300,100), drying(300,380), drying(800,400), drying(300,600), drying(300,480))

--@ chunk 304
cb_dark = pile{{"bone black",1},{"red earth",0.7},{"cobalt blue",0.7},{"lead white",1.4}, medium=0.04}
cb_mid  = pile{{"bone black",0.5},{"red earth",1.0},{"cobalt blue",0.4},{"lead white",2.2},{"vermilion",0.2}, medium=0.04}
cb_warm = pile{{"lead white",4.2},{"vermilion",0.9},{"chrome yellow",0.8},{"red earth",0.15}, medium=0.04}

function wob(y, amp, n, seed)
  -- returns n points' y offsets: a smooth wobble
  local o = {}
  local ph = seed * 1.7
  for i = 1, n do o[i] = amp * (math.sin(i * 1.3 + ph) * 0.6 + math.sin(i * 2.9 + ph * 2) * 0.4) end
  return o
end
function bar_stroke(bs, pile_, x0, x1, y, amp, seed, pr0, pr1, load)
  local n = 6
  local w = wob(y, amp, n, seed)
  local pts = {}
  for i = 1, n do
    local t = (i - 1) / (n - 1)
    pts[i] = {lerp(x0, x1, t), y + w[i] + 0.0}
  end
  bs:reload(pile_, load or 0.9)
  bs:stroke(pts, {pressure={pr0 or 0.8, pr1 or 0.2}, ramps={0.08, 0.55}})
end
-- test bar on the left glow
local b = brush{kind="flat", width=7}
bar_stroke(b, cb_dark, 30, 240, 424, 1.6, 1, 0.85, 0.25)
bar_stroke(b, cb_dark, 55, 270, 430, 1.6, 2, 0.85, 0.2)
bar_stroke(b, cb_mid, 40, 230, 436, 1.4, 3, 0.8, 0.2)
local bw = brush{kind="flat", width=3.2}
bar_stroke(bw, cb_warm, 60, 250, 442, 1.2, 4, 0.7, 0.15, 0.7)
print(wait(0))

--@ chunk 305
function bar_set(x0, x1, ya, yb, w, seed, opt)
  opt = opt or {}
  local dk = opt.dark or cb_dark
  local md = opt.mid or cb_mid
  local wm = opt.warm or cb_warm
  local b1 = brush{kind="flat", width=w}
  local b2 = brush{kind="flat", width=w * 0.75}
  local b3 = brush{kind="flat", width=math.max(2.2, w * 0.4)}
  local function ln(y0, y1, amp, sd, xa, xb)
    local n = 7
    local pts = {}
    local wv = wob(0, amp, n, sd)
    for i = 1, n do
      local t = (i - 1) / (n - 1)
      pts[i] = {lerp(xa, xb, t), lerp(y0, y1, t) + wv[i]}
    end
    return pts
  end
  local p0, p1 = opt.p0 or 0.85, opt.p1 or 0.18
  b1:reload(dk, 0.95)
  b1:stroke(ln(ya, yb, w * 0.2, seed, x0, x1), {pressure={p0, p1}, ramps={0.06, 0.6}})
  if opt.two then
    b1:reload(dk, 0.85)
    b1:stroke(ln(ya + w * 0.5, yb + w * 0.5, w * 0.2, seed + 1, x0 + (x1 - x0) * 0.08, x0 + (x1 - x0) * 0.92), {pressure={p0 * 0.95, p1}, ramps={0.06, 0.6}})
  end
  b2:reload(md, 0.9)
  b2:stroke(ln(ya + w * 1.0, yb + w * 1.0, w * 0.18, seed + 2, x0 + (x1 - x0) * 0.05, x0 + (x1 - x0) * 0.9), {pressure={p0 * 0.9, p1}, ramps={0.06, 0.6}})
  if not opt.nowarm then
    b3:reload(wm, 0.8)
    b3:stroke(ln(ya + w * 1.6, yb + w * 1.6, w * 0.15, seed + 3, x0 + (x1 - x0) * 0.1, x0 + (x1 - x0) * 0.85), {pressure={0.7, 0.12}, ramps={0.1, 0.5}})
  end
end
-- bars, upper ones paler; the sun is at x=300
bar_set(0, 200, 392, 398, 5, 11, {p0=0.7, p1=0.15, dark=cb_mid, mid=cb_warm, nowarm=true})
bar_set(-10, 260, 410, 416, 6.5, 12, {p0=0.9, p1=0.2, two=true})
bar_set(80, 380, 454, 458, 3.6, 13, {p0=0.8, p1=0.15})
bar_set(0, 150, 462, 466, 3.2, 14, {p0=0.75, p1=0.15})
bar_set(680, 430, 447, 454, 4.5, 15, {p0=0.8, p1=0.12, dark=cb_mid, mid=cb_warm})
bar_set(1010, 760, 402, 407, 6, 16, {p0=0.85, p1=0.15, two=true})
bar_set(1010, 840, 372, 376, 4, 17, {p0=0.6, p1=0.1, dark=cb_mid, mid=cb_warm, nowarm=true})
bar_set(20, 300, 340, 344, 3.4, 18, {p0=0.55, p1=0.1, dark=cb_mid, mid=cb_warm, nowarm=true})
print(wait(0))

--@ chunk 306
cb_cool = pile{{"bone black",0.5},{"cobalt blue",1.0},{"smalt",1.0},{"red earth",0.3},{"lead white",1.5}, medium=0.04}
cb_rosy = pile{{"lead white",3.2},{"vermilion",0.6},{"red earth",0.6},{"cobalt blue",0.3}, medium=0.04}
-- upper-right cloud deck: heavy ends at the right, tapering toward the sun
bar_set(1016, 650, 56, 64, 16, 21, {p0=0.9, p1=0.15, dark=cb_cool, mid=cb_dark, warm=cb_rosy, two=true})
bar_set(1016, 730, 98, 104, 14, 22, {p0=0.9, p1=0.15, dark=cb_cool, mid=cb_dark, warm=cb_rosy})
bar_set(1012, 600, 138, 148, 15, 23, {p0=0.9, p1=0.15, dark=cb_dark, mid=cb_mid, warm=cb_rosy, two=true})
bar_set(1016, 690, 178, 184, 13, 24, {p0=0.9, p1=0.15, dark=cb_dark, mid=cb_mid, warm=cb_warm})
bar_set(1012, 570, 214, 222, 13, 25, {p0=0.9, p1=0.15, dark=cb_dark, mid=cb_mid, warm=cb_warm, two=true})
bar_set(1016, 700, 252, 256, 11, 26, {p0=0.85, p1=0.15, dark=cb_dark, mid=cb_mid, warm=cb_warm})
bar_set(1012, 620, 290, 296, 9, 27, {p0=0.8, p1=0.12, dark=cb_dark, mid=cb_mid, warm=cb_warm})
bar_set(1012, 760, 332, 336, 6, 28, {p0=0.8, p1=0.12, dark=cb_mid, mid=cb_warm, warm=cb_warm})
-- upper-left high streaks
bar_set(-10, 260, 56, 62, 9, 31, {p0=0.6, p1=0.1, dark=cb_cool, mid=cb_dark, warm=cb_rosy, nowarm=true})
bar_set(-10, 190, 116, 120, 7, 32, {p0=0.55, p1=0.1, dark=cb_dark, mid=cb_mid, warm=cb_rosy, nowarm=true})
bar_set(20, 340, 190, 193, 5, 33, {p0=0.5, p1=0.1, dark=cb_mid, mid=cb_warm, nowarm=true})
print(wait(0))

--@ chunk 307
haze_m = pile{{"cobalt blue",1.0},{"smalt",1.2},{"red earth",0.5},{"bone black",0.25},{"lead white",1.0}, medium=0.55}
deckReg = rect(560, 0, 470, 360)
work(deckReg, {hand="glaze", pile=haze_m, angle=0.0, coverage=1.6, seed=2000001,
     load_at=function(x, y)
        local fx = smoothstep(560, 780, x)
        local fy = 1 - smoothstep(190, 350, y)
        return fx * fy * 0.9 end})
print(wait(0))
blend(deckReg:blur(20), {angle=0.0})
print(wait(0))

--@ chunk 308
print(wait(12*24*60))
print(drying(800,150), drying(700,300), drying(300,420), drying(300,600))

--@ chunk 309
HZ = 497
skyOnly = rect(-14, -14, 1028, HZ + 14)
isleZone = rect(630, 384, 400, HZ - 384):blur(3) * skyOnly
local function zone_strips(seed0, cov)
  local k = 0
  local yc = 390
  while yc <= 496 do
    k = k + 1
    local hh = 16
    local m = rect(620, yc - hh, 420, 2 * hh) * isleZone
    local rt = function(y) return clamp(1.6 - math.abs(y - yc) / hh, 0.05, 1) end
    hb(m, slcol(yc, 0.04), 12, seed0 + k * 2, cov, {40,120}, {clip=isleZone, angle_jitter=0.012, mix_jitter=0.15, load_at=function(x, y) return rt(y) * (1 - smoothstep(630, 720, x)) + 0.03 end})
    hb(m, srcol(yc, 0.04), 12, seed0 + k * 2 + 1, cov, {40,120}, {clip=isleZone, angle_jitter=0.012, mix_jitter=0.15, load_at=function(x, y) return rt(y) * smoothstep(630, 720, x) + 0.03 end})
    yc = yc + 9
  end
end
zone_strips(2100000, 9.0)
print(wait(0))
zone_strips(2110000, 9.0)
print(wait(0))
blend(isleZone, {angle=0.0, clip=isleZone})
print(wait(0))

--@ chunk 310
water = rect(-14, 497, 1030, 260)
tr_dk = pile{{"Prussian blue",0.45},{"raw umber",1.3},{"smalt",1.0},{"bone black",0.3},{"green earth",0.3}, medium=0.10}
tr_mid = pile{{"smalt",1.6},{"cobalt blue",0.4},{"raw umber",0.5},{"green earth",0.3},{"lead white",0.9}, medium=0.10}
tr_lt = pile{{"lead white",2.2},{"pale smalt",1.5},{"cobalt blue",0.3},{"green earth",0.2}, medium=0.10}
tz = rect(0, 640, 500, 101)
work(tz, {hand="body", tool={kind="flat", width=5}, pile=tr_dk, length={40,150}, angle=0.0, angle_jitter=0.025, coverage=1.3, fill=false, clip=false, mix_jitter=0.4, seed=2300001,
     pressure={0.35,0.7}, order="scatter"})
work(tz, {hand="body", tool={kind="flat", width=3}, pile=tr_mid, length={30,110}, angle=0.0, angle_jitter=0.025, coverage=0.9, fill=false, clip=false, mix_jitter=0.4, seed=2300002,
     pressure={0.3,0.6}, order="scatter"})
work(tz, {hand="body", tool={kind="flat", width=2}, pile=tr_lt, length={20,80}, angle=0.0, angle_jitter=0.025, coverage=0.5, fill=false, clip=false, mix_jitter=0.4, seed=2300003,
     pressure={0.3,0.55}, order="scatter"})
print(wait(0))

--@ chunk 311
print(wait(14*24*60))
print(drying(300,700), drying(700,450), drying(800,150))

--@ chunk 312
HZ = 497
skyOnly = rect(-14, -14, 1028, HZ + 14)
isl_k  = pile{{"raw umber",2.0},{"bone black",2.0},{"Prussian blue",0.35},{"red earth",0.35}, medium=0.02}
isl_k3 = pile{{"raw umber",1.6},{"bone black",1.4},{"smalt",0.9},{"red earth",0.5},{"lead white",0.35}, medium=0.02}
-- ground: low dyke with a mound for the mill, drawn point by point (y is the top edge)
G = outline{pts={{648,HZ+1},{664,496},{690,494.5},{716,493},{740,491},{756,489},{766,486},{776,483.5},{788,482.5},{800,483},{812,484.5},{826,486},{842,487},{858,486.5},{880,486},
                 {910,485},{940,484.5},{970,483.5},{1000,482},{1030,481},{1030,HZ+1}}, closed=true, char="firm", amount=0.3, lobe=9, seed=3101}:mask()
-- tree group A: overlapping crowns, each lobe drawn by hand
TA = outline{pts={{851,487},{850,479},{853,472},{852,466},{856,461},{862,459},{865,455},{871,452},{878,453},{883,450},{890,451},{896,455},{900,453},{906,457},{909,463},{908,469},{911,475},{910,487}}, closed=true, char="soft", amount=0.5, lobe=4, seed=3102}:mask()
-- two tall poplars and a smaller one
P1 = outline{pts={{939,486},{938,472},{938.5,460},{940,449},{942,441},{943.5,436},{945,441},{946.5,449},{948,460},{948.5,472},{948,486}}, closed=true, char="firm", amount=0.25, lobe=3, seed=3103}:mask()
P2 = outline{pts={{951,486},{950.5,474},{951,464},{952.5,455},{954,449},{955.5,445},{957,450},{958,457},{959,466},{959,474},{959,486}}, closed=true, char="firm", amount=0.25, lobe=3, seed=3104}:mask()
P3 = outline{pts={{927,486},{926.5,477},{927.5,469},{929,463},{931,460},{932.5,464},{933.5,470},{934,478},{934,486}}, closed=true, char="firm", amount=0.25, lobe=3, seed=3105}:mask()
-- scrub at the right
SC = outline{pts={{966,484},{968,477},{973,473},{979,472},{984,468},{991,467},{997,470},{1004,468},{1011,471},{1020,470},{1030,473},{1030,484}}, closed=true, char="soft", amount=0.45, lobe=4, seed=3106}:mask()
-- a small cottage to the left of the trees
CT = poly({{826,487},{826,479.5},{829.5,477},{833,474.5},{836.5,477},{840,479.5},{840,487}})
isl_all = G + TA + P1 + P2 + P3 + SC + CT
work(isl_all, {hand="body", tool={kind="filbert", width=4}, pile=isl_k, length={6,16}, angle=0.0, angle_jitter=0.6, coverage=10, fill=true, clip=isl_all, seed=3200001, mix_jitter=0.1})
print(wait(0))
-- mill: tower, cap, sails (thin quads) and the four stocks
mt = poly({{785,485},{795,485},{793.2,461},{786.8,461}})
mc = poly({{786.2,461.4},{793.8,461.4},{790,455.6}})
work(mt, {hand="detail", tool={kind="round", width=2}, pile=isl_k, coverage=9, clip=mt, seed=3200011})
work(mc, {hand="detail", tool={kind="round", width=1.6}, pile=isl_k, coverage=9, clip=mc, seed=3200012})
print(wait(0))

--@ chunk 313
sA = poly({{794.0,454.3},{806.0,437.0},{808.9,438.9},{796.9,456.3}})
sB = poly({{795.4,463.0},{811.2,473.2},{809.4,476.2},{793.4,466.0}})
sC = poly({{786.8,464.6},{777.0,478.2},{774.1,476.0},{784.0,462.4}})
sD = poly({{785.2,456.3},{769.8,445.0},{771.9,442.2},{787.2,453.5}})
for i, s in ipairs({sA, sB, sC, sD}) do
  work(s, {hand="detail", tool={kind="round", width=1.3}, pile=isl_k, coverage=9, clip=s, seed=3200020 + i})
end
armB = brush{kind="round", width=1.4, point=0.6}
local pr = armB:pressure_for(1.15)
for i, t in ipairs({{806.4,436.4},{811.6,473.8},{776.4,478.8},{769.2,444.4}}) do
  armB:load(isl_k, 1.0)
  armB:stroke({{790,459.5},{t[1],t[2]}}, {pressure={pr, pr}})
end
print(wait(0))

--@ chunk 314
water = rect(-14, 497, 1030, 260)
scr = (rect(-30, 622, 600, 140)):blur(14) * water
print("scr area", scr:area())
function strip_r(reg, yc, hh, wd, seed, cov)
  local m = rect(-30, yc - hh, 620, 2 * hh) * reg
  local rt = function(y) return clamp(1.5 - math.abs(y - yc) / hh, 0.05, 1) end
  hb(m, wsc(WL3, yc, 0.05), wd, seed, cov, {60,200}, {clip=reg, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) end})
end
local yc = 748
local k = 0
while yc >= 626 do
  k = k + 1
  strip_r(scr, yc, 26, 18, 2400000 + k * 2, 8.0)
  yc = yc - 12
end
print(wait(0))
blend(scr, {angle=0.0, clip=scr})
print(wait(0))

--@ chunk 315
water = rect(-14, 497, 1030, 260)
band = (rect(-14, 552, 1030, 120)):blur(10) * water
local yc = 664
local k = 0
while yc >= 566 do
  k = k + 1
  local m = rect(-14, yc - 26, 1030, 52) * band
  local rt = function(y) return clamp(1.5 - math.abs(y - yc) / 26, 0.05, 1) end
  hb(m, wsc(WL3, yc, 0.06), 18, 2410000 + k * 3, 5.5, {60,200}, {clip=band, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * (1 - xm3(x)) + 0.02 end})
  hb(m, wsc(WR3, yc, 0.06), 18, 2410001 + k * 3, 5.5, {60,200}, {clip=band, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * xm3(x) + 0.02 end})
  yc = yc - 12
end
print(wait(0))
blend(band, {angle=0.0, clip=band})
print(wait(0))
blend(band, {angle=0.006, clip=band})
print(wait(0))

--@ chunk 316
print(wait(12*24*60))
print(drying(300,700), drying(800,150), drying(300,600), drying(600,540))

--@ chunk 317
-- Cloud bank as one wet passage: smooth mass, soft irregular left edge.
CS = {
 {0,   {{"lead white",1.2},{"smalt",2.5},{"cobalt blue",0.8},{"raw umber",0.3},{"red earth",0.3},{"Prussian blue",0.1}}},
 {90,  {{"lead white",1.8},{"smalt",2.1},{"cobalt blue",0.6},{"red earth",0.5},{"raw umber",0.3}}},
 {170, {{"lead white",2.4},{"smalt",1.6},{"red earth",0.8},{"cobalt blue",0.3},{"raw umber",0.25}}},
 {240, {{"lead white",3.0},{"smalt",1.1},{"red earth",1.0},{"vermilion",0.2},{"raw umber",0.15}}},
 {300, {{"lead white",3.8},{"red earth",0.9},{"vermilion",0.5},{"smalt",0.6},{"yellow ochre",0.3}}},
 {350, {{"lead white",4.4},{"vermilion",0.7},{"chrome yellow",0.6},{"red earth",0.3}}},
 {400, {{"lead white",5.0},{"vermilion",0.7},{"chrome yellow",0.6},{"yellow ochre",0.2}}},
}
function ccol(y, med) return wsc(CS, y, med or 0.12) end
EDG = {{0,700},{40,650},{80,712},{120,640},{160,690},{200,618},{240,668},{280,606},{320,700},{360,770},{400,840},{440,900}}
function xedge(y)
  if y <= EDG[1][1] then return EDG[1][2] end
  for i = 1, #EDG - 1 do
    local a, b = EDG[i], EDG[i+1]
    if y <= b[1] then
      local t = (y - a[1]) / (b[1] - a[1])
      t = 0.5 - 0.5 * math.cos(math.pi * t)
      return lerp(a[2], b[2], t)
    end
  end
  return EDG[#EDG][2]
end
function cstrength(y) return 1 - smoothstep(330, 425, y) end
zoneC = rect(520, -14, 510, 460) - isl_all:grow(1)
local yc = 0
local k = 0
while yc <= 430 do
  k = k + 1
  local hh = 30
  local m = rect(520, yc - hh, 510, 2 * hh) * zoneC
  local rt = function(y)
    local r = clamp(1.4 - math.abs(y - yc) / hh, 0.05, 1)
    if yc == 0 then r = 1 end
    return r
  end
  hb(m, ccol(yc), 20, 2500000 + k * 2, 5.5, {80,220}, {angle_jitter=0.012, mix_jitter=0.2, clip=zoneC, load_at=function(x, y)
      local cl = smoothstep(xedge(y) - 30, xedge(y) + 45, x) * cstrength(y)
      return rt(y) * cl + 0.0 end})
  yc = yc + 20
  if k % 6 == 0 then print(yc, wait(0)) end
end
print(wait(0))
blend(rect(560, -14, 470, 450):blur(20) * zoneC, {angle=0.0, clip=zoneC})
print(wait(0))
blend(rect(560, -14, 470, 450):blur(20) * zoneC, {angle=0.008, clip=zoneC})
print(wait(0))

--@ chunk 318
function mixparts(a, b, t)
  local parts, names = {}, {}
  for _, e in ipairs(a) do if parts[e[1]] == nil then names[#names+1] = e[1] end; parts[e[1]] = (parts[e[1]] or 0) + e[2] * (1 - t) end
  for _, e in ipairs(b) do if parts[e[1]] == nil then names[#names+1] = e[1] end; parts[e[1]] = (parts[e[1]] or 0) + e[2] * t end
  local out = {}
  for _, n in ipairs(names) do if parts[n] > 0.01 then out[#out+1] = {n, parts[n]} end end
  return out
end
function stopparts(ST, y)
  for i = 1, #ST - 1 do
    local a, b = ST[i], ST[i+1]
    if y <= b[1] or i == #ST - 1 then
      local t = clamp((y - a[1]) / (b[1] - a[1]), 0, 1)
      return mixparts(a[2], b[2], t)
    end
  end
end
function mkpile(parts, med) local p = {}; for i, e in ipairs(parts) do p[i] = e end; p.medium = med or 0.05; return pile(p) end
function lparts(y)
  if y <= 312 then return stopparts(SKD2, y) end
  return stopparts(SLs, y)
end
function rparts(y) return stopparts(CS, y) end

print(drying(700, 40), drying(700, 200), drying(700, 400), drying(560, 300), drying(300, 300))
local X0, X1 = 340, 700
local nS = 7
local yc = 0
local k = 0
while yc <= 440 do
  k = k + 1
  local hh = 28
  for i = 0, nS do
    local cx = lerp(X0, X1, i / nS)
    local t = smoothstep(0, 1, i / nS)
    local hw = (X1 - X0) / nS * 1.3
    local m = rect(cx - hw, yc - hh, 2 * hw, 2 * hh)
    local rt = function(y)
      local r = clamp(1.4 - math.abs(y - yc) / hh, 0.05, 1)
      if yc == 0 then r = 1 end
      return r
    end
    hb(m, mkpile(mixparts(lparts(yc), rparts(yc), t), 0.08), 20, 2600000 + k * 100 + i, 5.0, {60,140}, {angle_jitter=0.012, mix_jitter=0.2, edge="lost",
        load_at=function(x, y) return rt(y) * clamp(1.5 - math.abs(x - cx) / hw, 0.05, 1) end})
  end
  yc = yc + 28
end
print(wait(0))

--@ chunk 319
skyR = rect(255, -14, 780, 462) - isl_all:grow(4)
print(drying(400, 100), drying(900, 100), drying(900, 400))
blend(skyR, {angle=0.0})
print(wait(0))
blend(skyR, {angle=0.01})
print(wait(0))

--@ chunk 320
print(wait(14*24*60))
print(drying(400, 100), drying(900, 100), drying(900, 400), drying(300, 300))

--@ chunk 321
function tx4(x) return smoothstep(260, 860, x) end
local k = 0
local yc = 0
while yc <= 340 do
  k = k + 1
  local hh = 34
  local m = rect(-14, yc - hh, 1044, 2 * hh)
  local rt = function(y)
    local r = clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
    if yc == 0 then r = 1 end
    return r
  end
  hb(m, mkpile(lparts(yc), 0.10), 28, 3000000 + k * 2, 4.6, {140,320}, {angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * (1 - tx4(x)) + 0.03 end})
  hb(m, mkpile(rparts(yc), 0.10), 28, 3000001 + k * 2, 4.6, {140,320}, {angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * tx4(x) + 0.03 end})
  yc = yc + 26
  if k % 4 == 0 then print(yc, wait(0)) end
end
print(wait(0))

--@ chunk 322
sk2 = rect(-14, -14, 1044, 360)
blend(sk2, {angle=0.0})
print(wait(0))
blend(sk2, {angle=0.008})
print(wait(0))

--@ chunk 323
HZ = 497
skyOnly = rect(-14, -14, 1044, HZ + 14)
SXg, SYg = 300, 497
glz_or2 = pile{{"chrome yellow",1},{"vermilion",0.5},{"lead white",0.3}, medium=0.75}
glz_go2 = pile{{"chrome yellow",1},{"vermilion",0.12},{"lead white",0.3}, medium=0.75}
local function rr(x, y, rx, ry) return math.sqrt(((x - SXg) / rx)^2 + ((y - SYg) / ry)^2) end
-- broad orange ring, strongest near the sun but leaving the core untouched
work(ellipse(SXg, SYg, 380, 190) * skyOnly, {hand="glaze", pile=glz_or2, angle=0.0, coverage=1.8, clip=skyOnly, seed=4000001,
     load_at=function(x, y) local r = rr(x, y, 380, 190); return clamp(1.1 - r, 0, 1) ^ 1.3 * smoothstep(0.03, 0.12, rr(x, y, 100, 50)) end})
print(wait(0))
work(ellipse(SXg, SYg, 200, 100) * skyOnly, {hand="glaze", pile=glz_go2, angle=0.0, coverage=1.6, clip=skyOnly, seed=4000002,
     load_at=function(x, y) local r = rr(x, y, 200, 100); return clamp(1.1 - r, 0, 1) ^ 1.2 * smoothstep(0.02, 0.10, rr(x, y, 70, 35)) end})
print(wait(0))
blend(ellipse(SXg, SYg, 400, 200) * skyOnly:blur(10), {angle=0.0, clip=skyOnly})
print(wait(0))

--@ chunk 324
print(wait(14*24*60))
print(drying(300,450), drying(600,300), drying(300,600))
SLn = {
 {268, {{"pale smalt",1.4},{"lead white",4.6},{"cobalt blue",0.2},{"yellow ochre",0.25},{"vermilion",0.08}}},
 {330, {{"lead white",5.0},{"yellow ochre",0.6},{"vermilion",0.2},{"pale smalt",0.4}}},
 {390, {{"lead white",4.4},{"yellow ochre",0.5},{"chrome yellow",0.6},{"vermilion",0.3}}},
 {440, {{"lead white",3.8},{"chrome yellow",1.0},{"vermilion",0.35}}},
 {475, {{"lead white",4.2},{"chrome yellow",1.1},{"vermilion",0.15}}},
 {497, {{"lead white",5.5},{"chrome yellow",0.9},{"vermilion",0.05}}},
}
SRn = {
 {268, {{"pale smalt",1.4},{"lead white",4.5},{"red earth",0.15},{"vermilion",0.15}}},
 {330, {{"lead white",4.8},{"vermilion",0.35},{"pale smalt",0.5},{"yellow ochre",0.2}}},
 {390, {{"lead white",4.2},{"vermilion",0.6},{"yellow ochre",0.35},{"pale smalt",0.3}}},
 {440, {{"lead white",3.8},{"vermilion",0.75},{"chrome yellow",0.55},{"red earth",0.1}}},
 {475, {{"lead white",3.8},{"chrome yellow",0.9},{"vermilion",0.55}}},
 {497, {{"lead white",4.4},{"chrome yellow",0.8},{"vermilion",0.4}}},
}
function nlp(y) if y <= 205 then return stopparts(SKD2, y) end; if y <= 268 then return mixparts(stopparts(SKD2, 205), SLn[1][2], (y - 205) / 63) end; return stopparts(SLn, y) end
function nrp(y) if y <= 205 then return stopparts(SKD2, y) end; if y <= 268 then return mixparts(stopparts(SKD2, 205), SRn[1][2], (y - 205) / 63) end; return stopparts(SRn, y) end
function tx5(x) return smoothstep(300, 880, x) end
HZ = 497
skyAllR = rect(-14, -14, 1044, HZ + 14)
local k = 0
local yc = 0
while yc <= 230 do
  k = k + 1
  local hh = 34
  local m = rect(-14, yc - hh, 1044, 2 * hh) * skyAllR
  local rt = function(y)
    local r = clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
    if yc == 0 then r = 1 end
    return r
  end
  hb(m, mkpile(nlp(yc), 0.10), 28, 4100000 + k * 2, 4.8, {140,320}, {clip=skyAllR, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * (1 - tx5(x)) + 0.03 end})
  hb(m, mkpile(nrp(yc), 0.10), 28, 4100001 + k * 2, 4.8, {140,320}, {clip=skyAllR, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * tx5(x) + 0.03 end})
  yc = yc + 26
end
print(yc, wait(0))

--@ chunk 325
local k = 100
local yc = 250
while yc <= 500 do
  k = k + 1
  local hh = 26
  local m = rect(-14, yc - hh, 1044, 2 * hh) * skyAllR
  local rt = function(y)
    if yc >= 490 then return clamp((y - 466) / 14, 0.05, 1) end
    return clamp(1.4 - math.abs(y - yc) / hh, 0.05, 1)
  end
  local cyc = math.min(yc, 497)
  hb(m, mkpile(nlp(cyc), 0.08), 24, 4100000 + k * 2, 5.4, {120,300}, {clip=skyAllR, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * (1 - tx5(x)) + 0.03 end})
  hb(m, mkpile(nrp(cyc), 0.08), 24, 4100001 + k * 2, 5.4, {120,300}, {clip=skyAllR, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * tx5(x) + 0.03 end})
  yc = yc + 13
  if k % 5 == 0 then print(yc, wait(0)) end
end
print(wait(0))

--@ chunk 326
print(drying(500, 250), drying(500, 400), drying(500, 100))
seam3 = (rect(-14, 205, 1044, 120)):blur(16) * skyAllR
blend(seam3, {angle=0.0, clip=skyAllR})
print(wait(0))
blend(seam3, {angle=0.01, clip=skyAllR})
print(wait(0))
blend(seam3, {angle=-0.01, clip=skyAllR})
print(wait(0))

--@ chunk 327
print(wait(0))
for _, p in ipairs({{200,40},{500,100},{800,180},{500,300},{200,400},{800,450},{300,480},{500,520},{500,700}}) do
  print(p[1], p[2], drying(p[1], p[2]))
end

--@ chunk 328
cw_body  = pile{{"lead white",1.7},{"smalt",1.6},{"cobalt blue",0.5},{"red earth",0.5},{"raw umber",0.25},{"bone black",0.12}, medium=0.06}
cw_mid   = pile{{"lead white",2.9},{"smalt",1.0},{"red earth",0.8},{"raw umber",0.15},{"vermilion",0.1}, medium=0.06}
cw_light = pile{{"lead white",4.2},{"vermilion",0.6},{"red earth",0.35},{"yellow ochre",0.4},{"smalt",0.3}, medium=0.06}
cw_gold  = pile{{"lead white",5.5},{"chrome yellow",0.7},{"vermilion",0.35}, medium=0.06}
CQsoft = CQ:blur(9)
cqv = function(x, y) return cq:value(x, y) end
-- test: body tone only, in the upper-right cloud, soft-edged by coverage falloff
stipple(CQ:grow(8), {pile=cw_body, width=9, coverage=function(x, y) return 2.6 * smoothstep(0.15, 0.85, CQsoft:at(x, y)) end,
                     cluster=0.25, feather=1, mix_jitter=0.3, seed=5000001})
print(wait(0))

--@ chunk 329
print(drying(850, 250), drying(900, 150), wait(0))
local E = function(x, y) return smoothstep(0.15, 0.85, CQsoft:at(x, y)) end
local T = function(v, c, h) return clamp(1 - math.abs(v - c) / h, 0, 1) end
-- 1. cool shadow body, larger dabs, darker toward the top / right (low value)
stipple(CQ:grow(6), {pile=cw_body, width=13, coverage=function(x, y) return 2.4 * E(x, y) * (1 - smoothstep(0.20, 0.52, cqv(x, y))) end,
                     cluster=0.3, feather=1, mix_jitter=0.3, seed=5000002})
print(wait(0))
-- 2. mauve-pink mid tones
stipple(CQ:grow(4), {pile=cw_mid, width=10, coverage=function(x, y) return 2.2 * E(x, y) * T(cqv(x, y), 0.42, 0.24) end,
                     cluster=0.3, feather=1, mix_jitter=0.3, seed=5000003})
print(wait(0))
-- 3. peach light
stipple(CQ:grow(2), {pile=cw_light, width=8, coverage=function(x, y) return 2.2 * E(x, y) * smoothstep(0.46, 0.64, cqv(x, y)) end,
                     cluster=0.3, feather=1, mix_jitter=0.3, seed=5000004})
print(wait(0))
-- 4. gold on the brightest faces
stipple(CQ, {pile=cw_gold, width=6, coverage=function(x, y) return 1.8 * E(x, y) * smoothstep(0.62, 0.76, cqv(x, y)) end,
                     cluster=0.3, feather=1, mix_jitter=0.3, seed=5000005})
print(wait(0))

--@ chunk 330
-- erase the stippled ghost cloud: fresh sky strips over the open sky, full width, then a soft horizontal blend
HZ = 497
skyAllR = rect(-14, -14, 1044, HZ + 14)
local k = 200
local yc = 0
while yc <= 320 do
  k = k + 1
  local hh = 34
  local m = rect(-14, yc - hh, 1044, 2 * hh) * skyAllR
  local rt = function(y)
    local r = clamp(1.3 - math.abs(y - yc) / hh, 0.05, 1)
    if yc == 0 then r = 1 end
    return r
  end
  local cyc = yc
  hb(m, mkpile(nlp(cyc), 0.10), 28, 4200000 + k * 2, 4.8, {140,320}, {clip=skyAllR, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * (1 - tx5(x)) + 0.03 end})
  hb(m, mkpile(nrp(cyc), 0.10), 28, 4200001 + k * 2, 4.8, {140,320}, {clip=skyAllR, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * tx5(x) + 0.03 end})
  yc = yc + 26
end
print(yc, wait(0))
local sb = (rect(-14, -14, 1044, 360)):blur(20) * skyAllR
blend(sb, {angle=0.0, clip=skyAllR})
print(wait(0))
blend(sb, {angle=0.01, clip=skyAllR})
print(wait(0))

--@ chunk 331
print(wait(14*24*60))
for _, p in ipairs({{200,40},{500,100},{800,180},{500,300},{200,400},{800,450},{300,480},{500,520},{500,700}}) do
  print(p[1], p[2], drying(p[1], p[2]))
end

--@ chunk 332
HZ = 497
skyAllR = rect(-14, -14, 1044, HZ + 14)
-- 1. base strips for the horizon zone, full width (same recipe as before)
local k = 300
local yc = 322
while yc <= 506 do
  k = k + 1
  local hh = 26
  local m = rect(-14, yc - hh, 1044, 2 * hh) * skyAllR
  local rt = function(y)
    if yc >= 490 then return clamp((y - 468) / 14, 0.05, 1) end
    return clamp(1.4 - math.abs(y - yc) / hh, 0.05, 1)
  end
  local cyc = math.min(yc, 497)
  hb(m, mkpile(nlp(cyc), 0.08), 24, 4300000 + k * 2, 5.4, {120,300}, {clip=skyAllR, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * (1 - tx5(x)) + 0.03 end})
  hb(m, mkpile(nrp(cyc), 0.08), 24, 4300001 + k * 2, 5.4, {120,300}, {clip=skyAllR, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * tx5(x) + 0.03 end})
  yc = yc + 13
end
print("strips done", wait(0))
-- 2. glow rings, outer to inner, each blended into the wet sky
SXg, SYg = 300, 497
rings2 = {
  {rx=420, ry=210, p=pile{{"lead white",5.4},{"yellow ochre",0.55},{"chrome yellow",0.25},{"vermilion",0.08}, medium=0.05}, wd=26},
  {rx=340, ry=165, p=pile{{"lead white",5.6},{"yellow ochre",0.4},{"chrome yellow",0.45},{"vermilion",0.08}, medium=0.05}, wd=24},
  {rx=270, ry=128, p=pile{{"lead white",5.8},{"chrome yellow",0.7},{"yellow ochre",0.15}, medium=0.05}, wd=22},
  {rx=205, ry=96,  p=pile{{"lead white",6.5},{"chrome yellow",0.7},{"vermilion",0.03}, medium=0.05}, wd=20},
  {rx=150, ry=70,  p=pile{{"lead white",7.5},{"chrome yellow",0.6}, medium=0.05}, wd=17},
  {rx=104, ry=48,  p=pile{{"lead white",9},{"chrome yellow",0.45}, medium=0.05}, wd=14},
  {rx=68,  ry=31,  p=pile{{"lead white",11},{"chrome yellow",0.3}, medium=0.05}, wd=11},
  {rx=40,  ry=18,  p=pile{{"lead white",14},{"chrome yellow",0.15}, medium=0.05}, wd=8},
}
for i, g in ipairs(rings2) do
  local m = ellipse(SXg, SYg, g.rx, g.ry) * skyAllR
  hb(m, g.p, g.wd, 4400000 + i, 3.6, {40,140}, {clip=skyAllR, edge="lost", angle_jitter=0.02, load_at=function(x, y)
      local r = math.sqrt(((x - SXg) / g.rx)^2 + ((y - SYg) / g.ry)^2)
      return clamp(1.35 - r, 0.03, 1) end})
  local bm = (ellipse(SXg, SYg, g.rx * 1.12, g.ry * 1.15) * skyAllR):blur(12)
  blend(bm, {angle=0.0, clip=skyAllR})
  print(i, wait(0))
end
local seam = (rect(-14, 300, 1044, 200)):blur(16) * skyAllR
blend(seam, {angle=0.0, clip=skyAllR})
print(wait(0))

--@ chunk 333
print(wait(14*24*60))
for _, p in ipairs({{200,40},{500,100},{800,180},{500,300},{200,400},{800,450},{300,480},{500,520},{500,700}}) do
  io = nil
  print(p[1], p[2], drying(p[1], p[2]))
end

--@ chunk 334
-- Upper-right cloud mass: broad soft body, cool violet-grey, strongest at top right. Glazed on, blended horizontally.
hazeA = pile{{"cobalt blue",1.0},{"smalt",1.2},{"red earth",0.55},{"bone black",0.3},{"lead white",1.2}, medium=0.35}
hazeB = pile{{"smalt",1.2},{"red earth",0.6},{"raw umber",0.3},{"cobalt blue",0.6},{"lead white",1.6}, medium=0.35}
massO = outline{pts={{1046,-14},{540,-14},{520,40},{560,100},{610,150},{650,210},{720,258},{810,290},{900,282},{980,300},{1046,296}}, closed=true, char="soft", lobe=60, amount=0.7, seed=6001}
mass1 = massO:mask()
work(mass1, {hand="glaze", pile=hazeA, angle=0.02, coverage=2.2, seed=6100001,
     load_at=function(x, y) return clamp(0.25 + (x - 560) / 520, 0, 1) * clamp(1.15 - y / 330, 0.15, 1) end})
print(wait(0))
work(mass1 * rect(560, 60, 500, 240), {hand="glaze", pile=hazeB, angle=0.02, coverage=1.6, seed=6100002,
     load_at=function(x, y) return clamp((x - 640) / 400, 0, 1) * clamp((y - 60) / 90, 0, 1) * clamp(1.2 - (y - 60) / 250, 0.1, 1) end})
print(wait(0))
blend(mass1:blur(24), {angle=0.0})
print(wait(0))
blend(mass1:blur(24), {angle=0.012})
print(wait(0))

--@ chunk 335
-- lower boundary of the cloud mass, per x (for lighting the underside)
mb = {}
for x = 500, 1040, 4 do
  local yy = 300
  for y = 340, 20, -1 do if mass1:at(x, y) > 0.5 then yy = y break end end
  mb[#mb + 1] = {x, yy}
end
function mbot(x)
  local i = clamp(math.floor((x - 500) / 4) + 1, 1, #mb - 1)
  local a, b = mb[i], mb[i + 1]
  return lerp(a[2], b[2], clamp((x - a[1]) / (b[1] - a[1]), 0, 1))
end
print(mbot(600), mbot(700), mbot(800), mbot(900), mbot(1000))
under_g = pile{{"lead white",3.6},{"vermilion",0.55},{"yellow ochre",0.55},{"chrome yellow",0.25}, medium=0.4}
under_r = pile{{"lead white",3.2},{"vermilion",0.7},{"red earth",0.35},{"cobalt blue",0.2}, medium=0.4}
zoneU = mass1 * rect(500, 100, 550, 230)
work(zoneU, {hand="glaze", pile=under_g, angle=0.02, coverage=1.6, clip=mass1, seed=6200001,
     load_at=function(x, y)
        local d = mbot(x) - y         -- distance above the lower edge
        local depth = clamp(1 - d / 90, 0, 1) ^ 1.3
        local sun = clamp(1.1 - (x - 540) / 420, 0.2, 1)
        return depth * sun end})
print(wait(0))
work(zoneU, {hand="glaze", pile=under_r, angle=0.02, coverage=1.4, clip=mass1, seed=6200002,
     load_at=function(x, y)
        local d = mbot(x) - y
        local depth = clamp(1 - d / 70, 0, 1) ^ 1.2
        local far = clamp((x - 640) / 380, 0, 1)
        return depth * far * 0.9 end})
print(wait(0))

--@ chunk 336
softZ = mass1 * (rect(500, 110, 550, 240)):blur(28)
print(drying(700, 200), drying(700, 250))
blend(softZ, {angle=1.25})
print(wait(0))

--@ chunk 337
print(drying(700, 200), drying(900, 250), drying(800, 100))
blend(softZ, {angle=0.0})
print(wait(0))
blend(softZ, {angle=0.015})
print(wait(0))

--@ chunk 338
print(wait(14*24*60))
for _, p in ipairs({{700,200},{900,250},{800,100},{300,300},{300,480},{500,520},{500,700}}) do
  print(p[1], p[2], drying(p[1], p[2]))
end

--@ chunk 339
water = rect(-14, 497, 1044, 260)
WL3 = {
 {497, {{"lead white",6},{"chrome yellow",0.9},{"yellow ochre",0.2},{"vermilion",0.1}}},
 {512, {{"lead white",5.4},{"yellow ochre",0.7},{"chrome yellow",0.3},{"pale smalt",0.4},{"vermilion",0.15}}},
 {540, {{"lead white",4.0},{"pale smalt",1.3},{"yellow ochre",0.4},{"vermilion",0.25},{"red earth",0.1}}},
 {580, {{"lead white",2.5},{"smalt",1.3},{"cobalt blue",0.5},{"red earth",0.35},{"raw umber",0.15}}},
 {625, {{"lead white",1.2},{"smalt",1.8},{"cobalt blue",0.6},{"Prussian blue",0.2},{"green earth",0.3},{"raw umber",0.5}}},
 {680, {{"lead white",0.5},{"Prussian blue",0.35},{"smalt",1.5},{"green earth",0.4},{"raw umber",0.9},{"bone black",0.15}}},
 {745, {{"Prussian blue",0.6},{"raw umber",1.4},{"smalt",0.9},{"green earth",0.4},{"bone black",0.5}}},
}
WR3 = {
 {497, {{"lead white",4.4},{"vermilion",0.75},{"chrome yellow",0.45},{"red earth",0.12}}},
 {512, {{"lead white",4.4},{"vermilion",0.55},{"pale smalt",0.7},{"yellow ochre",0.2}}},
 {540, {{"lead white",3.6},{"pale smalt",1.2},{"vermilion",0.4},{"red earth",0.35},{"smalt",0.2}}},
 {580, {{"lead white",2.5},{"smalt",1.3},{"red earth",0.7},{"cobalt blue",0.4},{"vermilion",0.15}}},
 {625, WL3[5][2]}, {680, WL3[6][2]}, {745, WL3[7][2]},
}
-- bottom half first (dark paint), overlapping strips, full width
local yc = 748
local k = 0
while yc >= 630 do
  k = k + 1
  strip3(yc, 26, 20, 5000000 + k * 3, 6.5)
  yc = yc - 13
end
print(yc, wait(0))

--@ chunk 340
local yc = 622
local k = 100
while yc >= 498 do
  k = k + 1
  local cov = lerp(3.4, 6.5, smoothstep(500, 620, yc))
  strip3(yc, 26, 18, 5000000 + k * 3, cov)
  yc = yc - 13
end
print(yc, wait(0))
print(drying(300,700), drying(300,600), drying(300,520))

--@ chunk 341
blend(water, {angle=0.0, clip=water})
print(wait(0))
blend(water, {angle=0.006, clip=water})
print(wait(0))
blend(water, {angle=-0.006, clip=water})
print(wait(0))

--@ chunk 342
print(wait(14*24*60))
for _, p in ipairs({{300,520},{300,600},{300,700},{800,600},{800,150}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 343
HZ = 497
isl_k3 = pile{{"raw umber",1.7},{"bone black",1.2},{"smalt",1.0},{"red earth",0.5},{"lead white",0.5}, medium=0.02}
isl_k4 = pile{{"raw umber",1.8},{"bone black",1.8},{"Prussian blue",0.25},{"red earth",0.4}, medium=0.02}
work(isl_all, {hand="body", tool={kind="filbert", width=4}, pile=isl_k3, length={6,16}, angle=0.0, angle_jitter=0.6, coverage=10, fill=true, clip=isl_all, seed=7000001, mix_jitter=0.1})
print(wait(0))
-- darker crowns
crowns = TA + P1 + P2 + P3 + SC
work(crowns, {hand="body", tool={kind="filbert", width=3.5}, pile=isl_k4, length={5,12}, angle=0.0, angle_jitter=0.8, coverage=4, fill=false, clip=crowns, seed=7000002, mix_jitter=0.3})
print(wait(0))
-- mill tower, cap, sails and stocks
work(mt, {hand="detail", tool={kind="round", width=2}, pile=isl_k4, coverage=9, clip=mt, seed=7000011})
work(mc, {hand="detail", tool={kind="round", width=1.6}, pile=isl_k4, coverage=9, clip=mc, seed=7000012})
for i, s in ipairs({sA, sB, sC, sD}) do
  work(s, {hand="detail", tool={kind="round", width=1.3}, pile=isl_k4, coverage=9, clip=s, seed=7000020 + i})
end
armB = brush{kind="round", width=1.4, point=0.6}
local pr = armB:pressure_for(1.15)
for i, t in ipairs({{806.4,436.4},{811.6,473.8},{776.4,478.8},{769.2,444.4}}) do
  armB:load(isl_k4, 1.0)
  armB:stroke({{790,459.5},{t[1],t[2]}}, {pressure={pr, pr}})
end
print(wait(0))

--@ chunk 344
HZ = 497
skyOnly = rect(-14, -14, 1044, HZ + 14)
SXg, SYg = 300, 497
glz_or2 = pile{{"chrome yellow",1},{"vermilion",0.5},{"lead white",0.3}, medium=0.75}
glz_go2 = pile{{"chrome yellow",1},{"vermilion",0.12},{"lead white",0.3}, medium=0.75}
local function rr(x, y, rx, ry) return math.sqrt(((x - SXg) / rx)^2 + ((y - SYg) / ry)^2) end
-- 1. wide low band of orange along the horizon, fading with height and with distance from the sun
work(rect(-14, 400, 1044, 100) * skyOnly, {hand="glaze", pile=glz_or2, angle=0.0, coverage=2.2, clip=skyOnly, seed=8000001,
     load_at=function(x, y)
        local up = clamp((y - 410) / 87, 0, 1) ^ 1.6
        local dx = clamp(1.05 - math.abs(x - 300) / 900, 0.15, 1)
        return up * dx * smoothstep(0.02, 0.10, rr(x, y, 100, 50)) end})
print(wait(0))
-- 2. ring of gold-orange around the sun
work(ellipse(SXg, SYg, 320, 170) * skyOnly, {hand="glaze", pile=glz_go2, angle=0.0, coverage=2.4, clip=skyOnly, seed=8000002,
     load_at=function(x, y) local r = rr(x, y, 320, 170); return clamp(1.1 - r, 0, 1) ^ 1.25 * smoothstep(0.02, 0.10, rr(x, y, 80, 40)) end})
print(wait(0))
blend(rect(-14, 300, 1044, 200):blur(14) * skyOnly, {angle=0.0, clip=skyOnly})
print(wait(0))
blend(rect(-14, 300, 1044, 200):blur(14) * skyOnly, {angle=0.008, clip=skyOnly})
print(wait(0))

--@ chunk 345
print(wait(14*24*60))
for _, p in ipairs({{300,450},{900,480},{500,300},{800,150},{300,520}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 346
HZ = 497
skyAllR = rect(-14, -14, 1044, HZ + 14)
SLn2 = {
 {268, {{"pale smalt",1.4},{"lead white",4.6},{"cobalt blue",0.2},{"yellow ochre",0.25},{"vermilion",0.08}}},
 {330, {{"lead white",5.0},{"yellow ochre",0.6},{"vermilion",0.25},{"pale smalt",0.3}}},
 {385, {{"lead white",4.6},{"yellow ochre",0.5},{"chrome yellow",0.5},{"vermilion",0.35}}},
 {435, {{"lead white",4.0},{"chrome yellow",0.9},{"vermilion",0.55},{"yellow ochre",0.15}}},
 {470, {{"lead white",3.6},{"chrome yellow",1.0},{"vermilion",0.75}}},
 {497, {{"lead white",4.5},{"chrome yellow",1.0},{"vermilion",0.45}}},
}
SRn2 = {
 {268, {{"pale smalt",1.4},{"lead white",4.5},{"red earth",0.15},{"vermilion",0.15}}},
 {330, {{"lead white",4.8},{"vermilion",0.4},{"pale smalt",0.5},{"yellow ochre",0.2}}},
 {385, {{"lead white",4.2},{"vermilion",0.7},{"yellow ochre",0.35},{"pale smalt",0.3}}},
 {435, {{"lead white",3.6},{"vermilion",0.95},{"chrome yellow",0.6},{"red earth",0.15}}},
 {470, {{"lead white",3.4},{"vermilion",1.0},{"chrome yellow",0.7},{"red earth",0.15}}},
 {497, {{"lead white",4.0},{"vermilion",0.9},{"chrome yellow",0.7}}},
}
function nlp2(y) if y <= 268 then return stopparts(SKD2, math.min(y, 205)) end; return stopparts(SLn2, y) end
function nrp2(y) if y <= 268 then return stopparts(SKD2, math.min(y, 205)) end; return stopparts(SRn2, y) end
local k = 400
local yc = 296
while yc <= 506 do
  k = k + 1
  local hh = 26
  local m = rect(-14, yc - hh, 1044, 2 * hh) * skyAllR
  local rt = function(y)
    if yc >= 490 then return clamp((y - 466) / 14, 0.05, 1) end
    return clamp(1.4 - math.abs(y - yc) / hh, 0.05, 1)
  end
  local cyc = math.min(yc, 497)
  hb(m, mkpile(nlp2(cyc), 0.08), 24, 9100000 + k * 2, 5.6, {120,300}, {clip=skyAllR, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * (1 - tx5(x)) + 0.03 end})
  hb(m, mkpile(nrp2(cyc), 0.08), 24, 9100001 + k * 2, 5.6, {120,300}, {clip=skyAllR, angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y) return rt(y) * tx5(x) + 0.03 end})
  yc = yc + 13
  if k % 6 == 0 then print(yc, wait(0)) end
end
print("strips done", wait(0))

--@ chunk 347
SXg, SYg = 300, 497
rings3 = {
  {rx=430, ry=215, p=pile{{"lead white",4.8},{"yellow ochre",0.5},{"chrome yellow",0.55},{"vermilion",0.3}, medium=0.05}, wd=26},
  {rx=350, ry=170, p=pile{{"lead white",5.0},{"chrome yellow",0.8},{"vermilion",0.3},{"yellow ochre",0.2}, medium=0.05}, wd=24},
  {rx=280, ry=130, p=pile{{"lead white",5.4},{"chrome yellow",0.9},{"vermilion",0.2}, medium=0.05}, wd=22},
  {rx=215, ry=98,  p=pile{{"lead white",6.2},{"chrome yellow",0.8},{"vermilion",0.1}, medium=0.05}, wd=20},
  {rx=155, ry=71,  p=pile{{"lead white",7.5},{"chrome yellow",0.65},{"vermilion",0.04}, medium=0.05}, wd=17},
  {rx=106, ry=49,  p=pile{{"lead white",9.5},{"chrome yellow",0.5}, medium=0.05}, wd=14},
  {rx=70,  ry=32,  p=pile{{"lead white",12},{"chrome yellow",0.35}, medium=0.05}, wd=11},
  {rx=42,  ry=19,  p=pile{{"lead white",16},{"chrome yellow",0.2}, medium=0.05}, wd=8},
}
for i, g in ipairs(rings3) do
  local m = ellipse(SXg, SYg, g.rx, g.ry) * skyAllR
  hb(m, g.p, g.wd, 9200000 + i, 3.6, {40,140}, {clip=skyAllR, edge="lost", angle_jitter=0.02, load_at=function(x, y)
      local r = math.sqrt(((x - SXg) / g.rx)^2 + ((y - SYg) / g.ry)^2)
      return clamp(1.35 - r, 0.03, 1) end})
  local bm = (ellipse(SXg, SYg, g.rx * 1.12, g.ry * 1.15) * skyAllR):blur(12)
  blend(bm, {angle=0.0, clip=skyAllR})
  print(i, wait(0))
end
local seam = (rect(-14, 240, 1044, 260)):blur(16) * skyAllR
blend(seam, {angle=0.0, clip=skyAllR})
print(wait(0))
blend(seam, {angle=0.008, clip=skyAllR})
print(wait(0))

--@ chunk 348
print(wait(14*24*60))
for _, p in ipairs({{300,450},{900,480},{500,300},{800,150},{300,520},{700,330}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 349
HZ = 497
isl_k3 = pile{{"raw umber",1.7},{"bone black",1.2},{"smalt",1.0},{"red earth",0.5},{"lead white",0.5}, medium=0.02}
isl_k4 = pile{{"raw umber",1.8},{"bone black",1.8},{"Prussian blue",0.25},{"red earth",0.4}, medium=0.02}
work(isl_all, {hand="body", tool={kind="filbert", width=4}, pile=isl_k3, length={6,16}, angle=0.0, angle_jitter=0.6, coverage=10, fill=true, clip=isl_all, seed=7100001, mix_jitter=0.1})
print(wait(0))
crowns = TA + P1 + P2 + P3 + SC
work(crowns, {hand="body", tool={kind="filbert", width=3.5}, pile=isl_k4, length={5,12}, angle=0.0, angle_jitter=0.8, coverage=4, fill=false, clip=crowns, seed=7100002, mix_jitter=0.3})
print(wait(0))
work(mt, {hand="detail", tool={kind="round", width=2}, pile=isl_k4, coverage=9, clip=mt, seed=7100011})
work(mc, {hand="detail", tool={kind="round", width=1.6}, pile=isl_k4, coverage=9, clip=mc, seed=7100012})
for i, s in ipairs({sA, sB, sC, sD}) do
  work(s, {hand="detail", tool={kind="round", width=1.3}, pile=isl_k4, coverage=9, clip=s, seed=7100020 + i})
end
armB = brush{kind="round", width=1.4, point=0.6}
local pr = armB:pressure_for(1.15)
for i, t in ipairs({{806.4,436.4},{811.6,473.8},{776.4,478.8},{769.2,444.4}}) do
  armB:load(isl_k4, 1.0)
  armB:stroke({{790,459.5},{t[1],t[2]}}, {pressure={pr, pr}})
end
print(wait(0))

--@ chunk 350
-- Rebuild the upper-right cloud as one wet passage: density field from a hand-drawn boundary, colour stops by height.
cloudM = outline{pts={{540,-30},{552,20},{578,58},{612,88},{640,118},{662,150},{688,176},{716,200},{744,222},{768,250},{796,272},{830,284},{866,278},{904,270},{942,286},{984,280},{1024,290},{1060,284},
                      {1060,-30}}, closed=true, char="soft", lobe=44, amount=0.7, seed=7001}:mask()
dens = cloudM:blur(24)
print("cloud area", cloudM:area(), dens:at(900, 100), dens:at(600, 200), dens:at(700, 300))
CPS = {
 {-14, {{"smalt",2.8},{"cobalt blue",1.0},{"red earth",0.45},{"raw umber",0.35},{"lead white",1.1},{"Prussian blue",0.1}}},
 {60,  {{"smalt",2.4},{"cobalt blue",0.8},{"red earth",0.6},{"raw umber",0.3},{"lead white",1.6}}},
 {120, {{"lead white",2.2},{"smalt",1.6},{"red earth",0.8},{"cobalt blue",0.3},{"raw umber",0.25}}},
 {180, {{"lead white",2.8},{"smalt",1.1},{"red earth",0.9},{"vermilion",0.1},{"raw umber",0.15}}},
 {230, {{"lead white",3.6},{"red earth",0.8},{"vermilion",0.45},{"smalt",0.55},{"yellow ochre",0.3}}},
 {270, {{"lead white",4.4},{"vermilion",0.7},{"yellow ochre",0.5},{"chrome yellow",0.4},{"red earth",0.2}}},
 {310, {{"lead white",4.8},{"vermilion",0.65},{"chrome yellow",0.6}}},
}
function skyat(y, x) return mixparts(nlp2(math.min(math.max(y, 0), 497)), nrp2(math.min(math.max(y, 0), 497)), tx5(x)) end
local X0, X1, nS = 500, 1050, 8
local k = 0
local yc = -14
while yc <= 330 do
  k = k + 1
  local hh = 26
  for i = 0, nS do
    local cx = lerp(X0, X1, i / nS)
    local hw = (X1 - X0) / nS * 1.35
    local xa = math.max(cx - hw, X0 - 20)
    local m = rect(xa, yc - hh, cx + hw - xa, 2 * hh)
    local dd = dens:at(clamp(cx, 0, 1000), clamp(yc, 0, 740))
    if yc < 0 then dd = dens:at(clamp(cx, 0, 1000), 1) end
    local p = mkpile(mixparts(skyat(yc, cx), stopparts(CPS, yc), dd), 0.10)
    hb(m, p, 22, 9300000 + k * 40 + i, 4.6, {70,180}, {angle_jitter=0.012, mix_jitter=0.2, load_at=function(x, y)
        local r = clamp(1.4 - math.abs(y - yc) / hh, 0.05, 1)
        if yc <= -10 then r = 1 end
        return r * clamp(1.5 - math.abs(x - cx) / hw, 0.05, 1) end})
  end
  yc = yc + 22
  if k % 5 == 0 then print(yc, wait(0)) end
end
print("cells done", wait(0))
