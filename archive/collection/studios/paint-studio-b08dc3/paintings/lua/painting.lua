-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ engine 2

--@ chunk 1
canvas{size=560, aspect=1.4, linen={13,12}, seed=1774,
  ground={
    {pile={{"yellow ochre",3},{"red earth",1},{"lead white",4}}, um=70, apply="knife", texture=0.3},
    {pile={{"lead white",8},{"yellow ochre",0.5},{"raw umber",0.12}}, um=30, apply="roller"},
  }}
print(W, H)
print(table.concat(tubes(), ", "))

--@ chunk 2
T = {}
T.glow   = pile{{"lead white",6},{"chrome yellow",1},{"vermilion",0.25}}
T.gold   = pile{{"lead white",6},{"yellow ochre",1.2},{"chrome yellow",0.3}}
T.cobalt = pile{{"lead white",5},{"cobalt blue",1}}
T.smalt  = pile{{"lead white",4},{"smalt",3}}
T.psmalt = pile{{"pale smalt",4},{"lead white",1}}
T.green  = pile{{"lead white",5},{"cobalt blue",1},{"yellow ochre",1}}
T.violet = pile{{"lead white",5},{"raw umber",0.6},{"smalt",1.5},{"red earth",0.3}}
T.dark   = pile{{"bone black",2},{"raw umber",1}}
T.pdark  = pile{{"Prussian blue",0.4},{"raw umber",1},{"bone black",1}}
names = {"glow","gold","cobalt","smalt","psmalt","green","violet","dark","pdark"}
for i, n in ipairs(names) do
  local x = 20 + (i-1)*106
  work(rect(x, 585, 96, 50), {hand="body", pile=T[n], coverage=1.6, clip=true, angle=0})
end
print(drying(50, 600))

--@ chunk 3
-- (a) glaze with medium, (b) gradient bands + blend, (c) stipple, (d) thick body
local qa = pile{{"smalt",1},{"lead white",1}, medium=0.5}
work(rect(20,645,140,50), {hand="glaze", pile=qa})
-- (b)
local mb1 = rect(180,645,140,17)
local mb2 = rect(180,662,140,17)
local mb3 = rect(180,679,140,17)
work(mb1, {hand="body", pile=T.cobalt, coverage=2.5, clip=true, angle=0})
work(mb2, {hand="body", pile=T.green,  coverage=2.5, clip=true, angle=0})
work(mb3, {hand="body", pile=T.gold,   coverage=2.5, clip=true, angle=0})
blend(rect(180,645,140,51), {angle=0})
-- (c) stipple
stipple(rect(340,645,140,50), {pile=T.smalt, width=3, coverage=1.5})
-- (d) thick
work(rect(500,645,140,50), {hand="body", pile=T.glow, coverage=4, clip=true, angle=0})
-- (e) stipple gradient w/ feather
stipple(rect(660,645,140,50), {pile=T.dark, width=2.5, coverage=function(x,y) return (y-645)/50*2.5 end, feather=0.5})
print(wait(0))

--@ chunk 4
h = pencil("2H")
-- far shore / horizon, very light
h:line({{0,431},{250,430},{500,431},{750,430},{1000,431}}, {pressure=0.2})
-- dune contour
DUNE = {{0,478},{40,484},{90,494},{140,502},{190,506},{240,510},{290,516},{340,528},{390,545},{440,565},{490,585},{540,600},{590,604},{640,596},{690,574},{722,550},{745,536},{790,530},{850,530},{910,538},{960,548},{1000,558}}
h:line(DUNE, {pressure=0.35})
print(h:width())

--@ chunk 5
h2 = pencil("HB")
TREE = {
  trunk = {pts={{190,512},{188,470},{192,430},{198,392},{201,362}}, w={64,48,40,36,34}},
  limbs = {
    {pts={{190,376},{150,334},{96,298},{42,272},{-10,264}}, w={26,22,18,14,10}},
    {pts={{200,366},{212,322},{201,272},{215,216},{232,166},{226,120},{236,80}}, w={28,24,20,15,11,7,4}},
    {pts={{206,382},{256,352},{316,332},{372,324},{420,338}}, w={24,20,15,11,8}},
    {pts={{196,414},{250,402},{305,412},{350,432},{386,450}}, w={18,15,12,8,5}},
    {pts={{186,424},{140,410},{92,412},{46,428}}, w={15,11,8,5}},
  }
}
h2:line(TREE.trunk.pts, {pressure=0.6})
for _, L in ipairs(TREE.limbs) do h2:line(L.pts, {pressure=0.6}) end
print(h2:width())

--@ chunk 6
local bo = body_of{spine={{900,705},{905,665},{900,625}}, widths={26,20,14},
   limbs={ {{903,655},{940,630},{975,615}, widths={10,7,4}}, {{900,635},{870,605},{850,585}, widths={9,6,3}} }, blend=0.8, char="soft"}
BO = bo
work(bo:mask(), {hand="body", pile=T.dark, coverage=2, clip=true})
print(bo:length(), bo:mask():area())

--@ chunk 7
SKYM = rect(0,0,1000,436)
S = {}
S.D1 = pile{{"smalt",3},{"cobalt blue",1},{"lead white",1.2},{"raw umber",0.2}}
S.B1 = pile{{"cobalt blue",2},{"smalt",1},{"lead white",3},{"raw umber",0.1}}
S.G1 = pile{{"lead white",5},{"cobalt blue",1},{"yellow ochre",0.7}}
S.Y1 = pile{{"lead white",6},{"yellow ochre",1},{"chrome yellow",0.3},{"cobalt blue",0.15}}
S.Y2 = pile{{"lead white",5},{"chrome yellow",1},{"yellow ochre",0.5}}
S.R1 = pile{{"lead white",4},{"chrome yellow",1},{"vermilion",0.4}}
local bands = {
  {0, 90, "D1"}, {70, 170, "B1"}, {150, 260, "G1"}, {240, 340, "Y1"}, {320, 400, "Y2"}, {380, 436, "R1"},
}
local t0 = wait(0)
for _, b in ipairs(bands) do
  work(rect(0, b[1], 1000, b[2]-b[1]), {hand="broad", pile=S[b[3]], coverage=2.2, angle=0, clip=SKYM})
end
print(t0, wait(0))

--@ chunk 8
local t0 = wait(0)
blend(SKYM, {angle=0})
print(t0, wait(0))

--@ chunk 9
S.D2 = pile{{"smalt",3},{"cobalt blue",0.6},{"lead white",1.3},{"raw umber",0.55},{"bone black",0.08}}
local t0 = wait(0)
work(rect(0,0,1000,150), {hand="broad", pile=S.D2, coverage=2.6, angle=0, clip=SKYM})
work(rect(0,120,1000,60), {hand="broad", pile=S.B1, coverage=1.5, angle=0, clip=SKYM})
blend(rect(0,0,1000,250), {angle=0})
print(t0, wait(0))

--@ chunk 10
S.W1 = pile{{"lead white",7},{"chrome yellow",0.7},{"vermilion",0.15}}
S.Rs = pile{{"lead white",3},{"red earth",0.45},{"chrome yellow",0.5},{"raw umber",0.1}}
local t0 = wait(0)
local glowm = ellipse(480, 425, 330, 120):blur(50)
work(glowm, {hand="body", pile=S.W1, coverage=2.2, clip=true, angle=0})
local sideL = ellipse(-40, 440, 300, 130):blur(60)
local sideR = ellipse(1040, 440, 300, 130):blur(60)
work(sideL + sideR, {hand="body", pile=S.Rs, coverage=1.6, clip=true, angle=0})
blend(rect(0,230,1000,206), {angle=0})
print(t0, wait(0))

--@ chunk 11
WATERM = rect(0,436,1000,180)
S.Wa = pile{{"lead white",5},{"chrome yellow",0.8},{"vermilion",0.2},{"yellow ochre",0.3}}
S.Wb = pile{{"lead white",6},{"yellow ochre",1},{"chrome yellow",0.3},{"cobalt blue",0.2}}
S.Wc = pile{{"lead white",5},{"cobalt blue",1},{"yellow ochre",0.8}}
S.Wd = pile{{"lead white",3},{"cobalt blue",1},{"smalt",1.2},{"raw umber",0.35}}
local wb = {
  {436, 40, "Wa"}, {465, 60, "Wb"}, {510, 70, "Wc"}, {560, 60, "Wd"},
}
local t0 = wait(0)
for _, b in ipairs(wb) do
  work(rect(0, b[1], 1000, b[2]), {hand="broad", pile=S[b[3]], coverage=2.2, angle=0, clip=WATERM})
end
blend(WATERM, {angle=0})
print(t0, wait(0))

--@ chunk 12
print(drying(500,100), drying(500,300), drying(300,480), drying(500,540))
print(wait(0))

--@ chunk 13
print(wait(24*60))
print(drying(500,100), drying(500,300), drying(300,480), drying(500,540))

--@ chunk 14
print(wait(24*60))
print(drying(500,100), drying(500,300), drying(300,480), drying(500,540), drying(500,420))

--@ chunk 15
S.Cv = pile{{"lead white",3},{"smalt",1.3},{"raw umber",0.5},{"red earth",0.35}}
cb = brush("filbert", 14)
cb:load(S.Cv, 0.9)
cb:stroke({{4,412},{120,407},{240,409},{330,416}}, {pressure={0.35,0.6}, ramps={0.15,0.35}, orient="along"})
print(cb:mark_width(0.3), cb:mark_width(0.6), cb:mark_width(1.0), cb:fullness())

--@ chunk 16
local function cstroke(pts, w, load, pr)
  local b = brush("filbert", w)
  b:load(S.Cv, load or 0.9)
  b:stroke(pts, {pressure=pr or {0.5,0.7}, ramps={0.12,0.4}, orient="along"})
end
cstroke({{0,404},{80,401},{170,402},{260,407},{330,414}}, 12, 1.0, {0.5,0.7})
cstroke({{20,410},{110,408},{200,410},{300,418}}, 10, 0.9, {0.5,0.7})
cstroke({{0,418},{90,416},{180,418},{250,421}}, 8, 0.8, {0.5,0.7})
cstroke({{60,396},{150,395},{240,399}}, 7, 0.7, {0.4,0.6})
blend(rect(0,385,400,50), {angle=0})
print(wait(0))

--@ chunk 17
local mA = poly({{630,418},{700,409},{790,403},{880,400},{950,401},{1000,404},{1000,424},{930,422},{850,421},{760,424},{680,425}}, true)
work(mA, {hand="scumble", pile=S.Cv, coverage=1.6, angle=0, edge="soft", length={30,70}})
print(wait(0))

--@ chunk 18
blend(rect(600,380,400,60), {angle=0.03})
print(wait(0))

--@ chunk 19
S.W2 = pile{{"lead white",6},{"chrome yellow",0.8},{"yellow ochre",0.4},{"vermilion",0.12}}
fb = brush{kind="flat", width=16, stiffness=0.6, ragged=0.7}
fb:load(S.W2, 0.55)
fb:stroke({{10,428},{90,426},{180,427},{270,431},{320,433}}, {pressure={0.45,0.35}, ramps={0.1,0.4}, orient="along"})
fb:stroke({{0,420},{80,418},{170,421},{250,425}}, {pressure={0.4,0.3}, ramps={0.1,0.4}, orient="along"})
print(fb:fullness())

--@ chunk 20
S.Cd = pile{{"lead white",2.2},{"smalt",1.3},{"raw umber",0.55},{"red earth",0.3}}
local function dry(b, pile_, load, pts, pr)
  b:load(pile_, load)
  b:stroke(pts, {pressure=pr or {0.5,0.35}, ramps={0.1,0.4}, orient="along"})
end
-- left bar: more streaks
dry(fb, S.W2, 0.5, {{40,402},{110,401},{190,403},{260,408}}, {0.4,0.3})
dry(fb, S.W2, 0.45, {{0,411},{70,410},{140,412},{200,414}}, {0.4,0.3})
local fd = brush{kind="flat", width=10, stiffness=0.6, ragged=0.7}
dry(fd, S.Cd, 0.5, {{20,396},{100,394},{180,396},{250,400}}, {0.45,0.3})
dry(fd, S.Cd, 0.45, {{0,416},{60,415},{130,417},{190,419}}, {0.4,0.3})
-- right bar: bright streaks drawn through
dry(fb, S.W2, 0.55, {{640,424},{720,421},{810,421},{900,422},{990,424}}, {0.45,0.35})
dry(fb, S.W2, 0.5, {{700,413},{780,410},{860,410},{940,411},{1000,413}}, {0.4,0.3})
dry(fb, S.W2, 0.45, {{660,404},{740,402},{820,401},{900,402}}, {0.4,0.3})
dry(fd, S.Cd, 0.5, {{650,418},{740,415},{830,415},{920,416},{995,418}}, {0.45,0.3})
dry(fd, S.Cd, 0.45, {{740,398},{820,396},{900,397},{980,399}}, {0.4,0.3})
print(wait(0))

--@ chunk 21
local mL = rect(0,380,300,60):blur(14)
local mR = rect(610,372,390,72):blur(14)
blend(mL, {angle=0})
blend(mR, {angle=0})
print(wait(0))

--@ chunk 22
print(wait(26*60))
print(drying(100,410), drying(800,410), drying(500,300))

--@ chunk 23
print(wait(24*60))
print(drying(100,410), drying(800,410), drying(500,300))

--@ chunk 24
S.Vd = pile{{"smalt",2},{"cobalt blue",0.5},{"raw umber",0.7},{"bone black",0.12}, medium=0.55}
local topm = rect(0,0,1000,250)
local t0 = wait(0)
work(topm, {hand="glaze", pile=S.Vd, coverage=1.0, angle=0.0, clip=topm,
  load_at=function(x,y) return clamp(1.1 - y/230, 0.05, 1.0) end})
print(t0, wait(0))

--@ chunk 25
local topm = rect(0,0,1000,250)
local t0 = wait(0)
blend(topm, {angle=0})
blend(topm, {angle=0.12})
print(t0, wait(0))

--@ chunk 26
local function trial(x, ch, amt, lobe)
  local o = body_of{spine={{x,712},{x+3,690},{x-2,668},{x+2,644}}, widths={26,20,16,12},
     limbs={ {{x+1,664},{x+20,650},{x+36,640}, widths={8,5,3}} }, blend=0.8, char=ch, amount=amt, lobe=lobe}
  work(o:mask(), {hand="body", pile=T.dark, coverage=4, clip=true, fill=true, angle=math.pi/2})
end
trial(560, "firm", 0.3, 30)
trial(640, "firm", 1.0, 20)
trial(720, "soft", 0.4, 40)
trial(800, "broken", 0.6, 25)
print(wait(0))

--@ chunk 27
S.Gs = pile{{"lead white",5},{"cobalt blue",1},{"yellow ochre",0.8}, medium=0.45}
local seam = rect(0,205,1000,90)
local t0 = wait(0)
work(seam, {hand="glaze", pile=S.Gs, coverage=1.0, angle=0.0, clip=seam,
  load_at=function(x,y) return 0.55 * (1 - math.abs(y-250)/48) end, edge="lost"})
blend(seam, {angle=0})
print(t0, wait(0))

--@ chunk 28
print(drying(500,295), drying(500,260), drying(500,100), drying(300,400))
print(wait(0))

--@ chunk 29
S.Y1m = pile{{"lead white",6},{"yellow ochre",1},{"chrome yellow",0.3},{"cobalt blue",0.15}, medium=0.5}
local t0 = wait(0)
local band = rect(0,262,1000,90)
work(band, {hand="glaze", pile=S.Y1m, coverage=1.2, angle=0.0, edge=0.5,
  load_at=function(x,y) return clamp((y-262)/45, 0, 1) * 0.6 end})
print(t0, wait(0))

--@ chunk 30
local seam = rect(0,205,1000,90)
local n = lose(seam, {pile=S.Y1m, where=function(x,y) return (y>270) and 1 or 0 end, load=0.3, reach={30,30}, tool="filbert 10", every=1.0})
print(n, wait(0))

--@ chunk 31
S.C1 = S.Y1
S.C2 = S.Y2
S.C3 = pile{{"lead white",7},{"chrome yellow",0.7},{"vermilion",0.15}}
S.C4 = pile{{"lead white",4},{"chrome yellow",1},{"vermilion",0.45}}
S.C5 = S.Wa
LOW = rect(0, 304, 1000, 166):blur(14)
local bands = {{300,345,"C1"},{335,385,"C2"},{375,420,"C3"},{410,448,"C4"},{440,472,"C5"}}
local t0 = wait(0)
for _, b in ipairs(bands) do
  work(rect(0, b[1], 1000, b[2]-b[1]), {hand="broad", pile=S[b[3]], coverage=2.2, angle=0, clip=LOW})
end
print(t0, wait(0))

--@ chunk 32
S.Sd = pile{{"lead white",3.2},{"smalt",0.6},{"red earth",0.6},{"yellow ochre",0.5},{"raw umber",0.25}}
S.W3 = pile{{"lead white",8},{"chrome yellow",0.5},{"vermilion",0.08}}
local t0 = wait(0)
local sides = (ellipse(-80, 415, 390, 115) + ellipse(1080, 415, 390, 115)):blur(55)
work(sides, {hand="body", pile=S.Sd, coverage=1.8, clip=sides, angle=0})
local core = ellipse(480, 418, 250, 95):blur(40)
work(core, {hand="body", pile=S.W3, coverage=2.2, clip=core, angle=0})
blend(rect(0,290,1000,190), {angle=0})
print(t0, wait(0))

--@ chunk 33
print(drying(100,380), drying(100,500), drying(480,380), drying(480,500), drying(480,320), drying(480,550))

--@ chunk 34
print(wait(36*60))
print(drying(100,380), drying(100,500), drying(480,380), drying(480,500), drying(480,320))

--@ chunk 35
print(wait(14*60))
print(drying(100,380), drying(100,500), drying(480,380), drying(480,500), drying(480,320), drying(900,380))
