-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ engine 2

--@ chunk 1
canvas{size=900, aspect=1.4, linen={14, 13}, seed=11,
  ground={
    {pile={{"red earth", 2}, {"yellow ochre", 3}, {"lead white", 3}}, um=140, apply="knife", texture=0.35},
    {pile={{"lead white", 9}, {"yellow ochre", 0.5}, {"raw umber", 0.25}}, um=70, apply="brush", texture=0.3},
  }}
print(W, H)
print(table.concat(tubes(), ", "))

--@ chunk 2
-- swatch test strip at the very bottom (will be buried under the dark foreground later)
sw = {
  {"zenith",   pile{{"smalt",4},{"cobalt blue",1},{"Prussian blue",0.15},{"lead white",1}, medium=0.3}},
  {"upper",    pile{{"cobalt blue",2},{"smalt",2},{"lead white",3}, medium=0.3}},
  {"mid",      pile{{"lead white",4},{"cobalt blue",1},{"green earth",0.5},{"chrome yellow",0.2}, medium=0.3}},
  {"low",      pile{{"lead white",5},{"chrome yellow",1}, medium=0.3}},
  {"horizon",  pile{{"lead white",4},{"chrome yellow",1},{"vermilion",0.5},{"yellow ochre",0.3}, medium=0.3}},
  {"hills",    pile{{"smalt",2},{"lead white",3},{"red earth",0.4},{"cobalt blue",0.5}, medium=0.3}},
  {"umber",    pile{{"raw umber",3},{"bone black",1}, medium=0.1}},
  {"blackgrn", pile{{"Prussian blue",1},{"chrome yellow",0.6},{"bone black",1}, medium=0.1}},
  {"mist",     pile{{"lead white",5},{"smalt",1},{"red earth",0.3},{"yellow ochre",0.3}, medium=0.3}},
  {"warmdark", pile{{"raw umber",2},{"red earth",1},{"Prussian blue",0.5}, medium=0.1}},
}
for i, s in ipairs(sw) do
  local m = rect((i-1)*100, 654, 100, 60)
  work(m, {hand="body", pile=s[2], coverage=1.6, clip=true})
end
print("done")

--@ chunk 3
-- Sky, first lay-in: bands of graded color painted wet into wet, top to horizon
Zp = pile{{"smalt",4},{"cobalt blue",1},{"Prussian blue",0.35},{"lead white",0.8}, medium=0.3}
Up = pile{{"cobalt blue",2},{"smalt",2},{"lead white",3}, medium=0.3}
Mp = pile{{"lead white",4},{"cobalt blue",1},{"green earth",0.6},{"chrome yellow",0.25}, medium=0.3}
Lp = pile{{"lead white",5},{"chrome yellow",1},{"green earth",0.2}, medium=0.3}
Hp = pile{{"lead white",4},{"chrome yellow",1},{"vermilion",0.5},{"yellow ochre",0.3}, medium=0.3}

local function band(y0, y1, p, cov)
  local m = rect(-60, y0, 1120, y1 - y0)
  work(m, {hand="broad", tool={kind="flat", width=46}, pile=p, angle=0, coverage=cov or 1.5,
           length={250,520}, fill=true, angle_jitter=0.03, order="down"})
end
band(-30, 150, Zp, 1.7)
band(110, 235, Up, 1.6)
band(200, 310, Mp, 1.6)
band(270, 360, Lp, 1.6)
band(330, 430, Hp, 1.7)
print("sky laid")

--@ chunk 4
blend(rect(0, 0, 1000, 440), {angle=0})
print("blended once")

--@ chunk 5
print(drying(500,50), drying(500,200), drying(500,300), drying(500,400))

--@ chunk 6
PG = pile{{"lead white",6},{"chrome yellow",0.5},{"vermilion",0.1}, medium=0.35}
PL = pile{{"lead white",6},{"cobalt blue",0.5},{"green earth",0.5},{"chrome yellow",0.15}, medium=0.35}
-- cooler, greener, paler on the left of the lower sky
work(ellipse(150, 360, 420, 90), {hand="broad", tool={kind="flat", width=50}, pile=PL, angle=0,
     coverage=1.0, load_at=function(x,y) return 0.7 * (1 - smoothstep(100, 650, x)) + 0.1 end,
     length={250,520}, fill=false, angle_jitter=0.03, order="down"})
-- pale warm glow centered on the setting sun, right of center
work(ellipse(720, 380, 330, 110), {hand="broad", tool={kind="flat", width=50}, pile=PG, angle=0,
     coverage=1.2, load_at=function(x,y) return 0.8 * math.exp(-((x-720)/230)^2) * (0.35 + 0.65*smoothstep(270, 400, y)) + 0.1 end,
     length={200,420}, fill=false, angle_jitter=0.03, order="down"})
blend(rect(0, 150, 1000, 290), {angle=0})
print("ok")

--@ chunk 7
blend(rect(0, 0, 1000, 330), {angle=0})
print("ok")

--@ chunk 8
-- thin strata of blue-grey cloud lying along the lower sky, mostly to the right
Cg = pile{{"smalt",2},{"lead white",3},{"red earth",0.35},{"cobalt blue",0.3}, medium=0.35}
local cb = brush{kind="flat", width=9, stiffness=0.4}
local rows = {
  {y=252, x0=560, x1=930, w=7, p=0.30}, {y=266, x0=640, x1=990, w=6, p=0.34},
  {y=281, x0=520, x1=860, w=6, p=0.36}, {y=294, x0=700, x1=1000, w=5, p=0.34},
  {y=306, x0=560, x1=800, w=4, p=0.30}, {y=317, x0=660, x1=950, w=4, p=0.32},
  {y=328, x0=590, x1=880, w=3, p=0.30}, {y=337, x0=730, x1=1000, w=3, p=0.28},
  {y=224, x0=700, x1=980, w=8, p=0.22}, {y=205, x0=620, x1=900, w=7, p=0.18},
  {y=270, x0=60, x1=330, w=5, p=0.22}, {y=300, x0=120, x1=420, w=4, p=0.22},
}
for i, r in ipairs(rows) do
  cb:load(Cg, 0.55)
  local pts, x = {}, r.x0
  while x < r.x1 do
    pts[#pts+1] = {x, r.y + rand(-1.2, 1.2) + 0.012*(x - r.x0)*rand(-1,1)}
    x = x + rand(30, 60)
  end
  pts[#pts+1] = {r.x1, r.y + rand(-1, 1)}
  cb:stroke(pts, {pressure={r.p, r.p*0.6}, ramps={0.25, 0.5}, shake=0.5})
end
print("strata laid")

--@ chunk 9
blend(rect(480, 195, 540, 160), {angle=0})
blend(rect(40, 255, 400, 60), {angle=0})
print("ok")

--@ chunk 10
blend(rect(0, 0, 1000, 420), {angle=0})
print("ok")

--@ chunk 11
print(wait(1))
print(drying(500,50), drying(500,200), drying(500,300), drying(500,400))

--@ chunk 12
print(wait(22*60))
print(drying(500,50), drying(500,200), drying(500,300), drying(500,400))

--@ chunk 13
print(wait(30*60))
print(drying(500,50), drying(500,200), drying(500,300), drying(500,400))

--@ chunk 14
-- Far hills: blue-lilac ridge on the horizon, with a basalt cone at the right where the sun went down
local ridge = outline{
  {-20,394},{60,389},{125,381},{175,372},{222,377},{268,386},{330,392},{385,389},{440,383},{500,388},
  {560,395},{625,399},{675,393},{708,384},{742,371},{770,352},{793,338},{812,331},{832,337},{852,352},
  {876,369},{905,383},{950,392},{1020,396},
  char="soft", amount=0.35, seed=5, open=true}
H1m = ridge:below(455)
H1p = pile{{"smalt",2},{"lead white",4},{"red earth",0.35},{"cobalt blue",0.3}, medium=0.3}
work(H1m, {hand="body", pile=H1p, coverage=2.0, fill=true, clip=true, angle=0})
print(H1m:area())

--@ chunk 15
-- Redo the far hills: smoother, firmer ridge drawn a little above the first attempt so it buries it
local ridge = outline{
  {-20,386},{70,382},{130,374},{178,365},{225,369},{272,379},{330,385},{390,382},{445,376},{505,381},
  {565,388},{620,391},{665,385},{700,372},{735,352},{765,333},{790,319},{806,313},{823,321},{845,340},
  {870,358},{900,373},{945,383},{1020,388},
  char="firm", amount=0.12, seed=9, open=true}
H1m = ridge:below(455)
H1p = pile{{"smalt",3},{"lead white",3},{"red earth",0.45},{"cobalt blue",0.35}, medium=0.3}
work(H1m, {hand="body", pile=H1p, coverage=2.2, fill=true, clip=true, angle=0})
print(H1m:area())

--@ chunk 16
-- Nearer wooded hills: darker blue-green, low on the horizon
local r2 = outline{
  {-20,410},{40,405},{100,399},{160,396},{215,400},{270,406},{325,412},{385,409},{445,403},{500,400},
  {550,403},{600,409},{650,414},{705,415},{760,411},{815,405},{865,399},{910,395},{955,398},{1020,404},
  char="firm", amount=0.3, seed=21, open=true}
H2m = r2:below(480)
H2p = pile{{"smalt",3},{"lead white",2},{"green earth",1.2},{"raw umber",0.35},{"red earth",0.2}, medium=0.3}
work(H2m, {hand="body", pile=H2p, coverage=2.2, fill=true, clip=true, angle=0})
print(H2m:area())

--@ chunk 17
-- Plain: horizontal bands of underpainting, mist-lit far away, darker and olive near
P1 = pile{{"lead white",5},{"yellow ochre",0.5},{"smalt",0.3},{"vermilion",0.08}, medium=0.3}
P2 = pile{{"lead white",2},{"green earth",1},{"yellow ochre",0.8},{"smalt",0.4},{"raw umber",0.3}, medium=0.3}
P3 = pile{{"raw umber",2},{"green earth",1},{"yellow ochre",0.8},{"Prussian blue",0.2},{"lead white",0.6}, medium=0.25}
P4 = pile{{"raw umber",2},{"Prussian blue",0.5},{"bone black",0.8},{"green earth",0.5}, medium=0.2}
local function pband(y0, y1, p, cov)
  work(rect(-60, y0, 1120, y1 - y0), {hand="broad", tool={kind="flat", width=46}, pile=p, angle=0,
       coverage=cov or 1.5, length={250,520}, fill=true, angle_jitter=0.03, order="down"})
end
pband(432, 490, P1, 1.8)
pband(478, 545, P2, 1.7)
pband(530, 625, P3, 1.7)
pband(610, 735, P4, 1.7)
blend(rect(0, 425, 1000, 300), {angle=0})
print("ok")
