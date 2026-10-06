-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box inness
--@ engine 2

--@ chunk 1
canvas{size=1140, aspect=1.5, linen={17, 15}, seed=1891,
  ground={
    {pile={{"lead white", 6}, {"yellow ochre", 0.15}, {"raw umber", 0.05}}, um=90, apply="knife", texture=0.35},
    {pile={{"raw umber", 1}, {"red earth", 0.25}, {"lead white", 1.2}}, um=8, apply="brush", texture=0.5},
  }}
print(W, H)
print(table.concat(tubes(), ", "))

--@ chunk 2
c = chalk()
-- distant treeline / horizon
c:sketch({{0,398},{80,388},{150,392},{230,380},{300,386},{360,396},{430,390},{520,384},{590,380}}, {pressure=0.3})
-- meadow edge / near middle ground
c:sketch({{0,470},{150,462},{300,468},{450,475},{560,478}}, {pressure=0.25})
-- oak mass outline
c:sketch({{600,440},{590,380},{605,300},{640,240},{660,180},{710,130},{780,110},{840,130},{880,170},{930,200},{960,260},{950,330},{970,400},{940,450}}, {pressure=0.35})
-- trunks
c:sketch({{730,480},{735,420},{728,360}}, {pressure=0.35})
c:sketch({{790,485},{785,420},{800,360}}, {pressure=0.35})
-- slender trees
c:sketch({{515,470},{518,350},{512,240},{520,170}}, {pressure=0.3})
c:sketch({{548,475},{545,360},{552,260}}, {pressure=0.3})
-- pond
c:sketch({{170,500},{250,488},{360,486},{470,494},{430,512},{300,516},{190,510},{170,500}}, {pressure=0.25})
-- path
c:sketch({{560,667},{590,600},{640,540},{690,500},{720,485}}, {pressure=0.25})
-- figure
c:sketch({{655,505},{657,485}}, {pressure=0.4})

--@ chunk 3
umb = pile{{"raw umber", 1}, medium=0.6}
dk = pile{{"raw umber", 1}, {"bone black", 0.5}, {"Antwerp blue", 0.25}, medium=0.5}
sien = pile{{"raw sienna", 1}, {"raw umber", 0.2}, medium=0.6}
olv = pile{{"raw sienna", 1}, {"Antwerp blue", 0.25}, {"raw umber", 0.4}, medium=0.55}
blu = pile{{"Antwerp blue", 0.4}, {"raw umber", 0.6}, {"bone black", 0.1}, medium=0.7}

--@ chunk 4
oak = (ellipse(705,205,75,65) + ellipse(785,160,85,58) + ellipse(865,215,80,70) + ellipse(660,300,70,85)
      + ellipse(765,285,115,95) + ellipse(905,315,65,85) + ellipse(700,395,105,60) + ellipse(855,400,105,58)
      + ellipse(620,410,40,45)):roughen(16, 55, 3)
trunks = ribbon({{730,490},{734,430},{728,370}}, 14) + ribbon({{792,492},{787,430},{798,370}}, 12)
far = below({{0,392},{40,383},{90,380},{130,386},{180,378},{230,370},{280,376},{330,384},{380,390},{430,384},{480,378},{540,372},{620,380},{1000,380}})
      * above({{0,412},{1000,412}})
far = far:roughen(5, 18, 7)
land = above({{0,1000}}) -- placeholder
print(oak:area(), far:area())

--@ chunk 5
land = below({{0,408},{200,404},{400,406},{600,410},{1000,412}})
fore = below({{0,560},{200,548},{400,556},{560,580},{700,540},{850,520},{1000,515}})
-- thin warm-olive scrub over the meadow
work(land - oak, {hand="broad", pile=olv, angle=0.02, coverage=1.2, pressure={0.4,0.7}, load=0.5, edge="soft", seed=11})
-- darker warm umber in the foreground
work(fore, {hand="broad", pile=umb, angle=-0.1, coverage=1.3, pressure={0.5,0.8}, load=0.6, edge="loose", seed=12})
-- distant treeline, thin cool
work(far, {hand="body", pile=blu, angle=0, coverage=1.5, pressure={0.4,0.7}, load=0.5, edge="soft", seed=13})
print(wait(0))

--@ chunk 6
-- the oaks: transparent dark scrubbed in, trunks too
work(oak + trunks, {hand="body", tool="filbert 14", pile=dk, angle=function(x,y) return -0.6 + 0.002*(x-700) end, coverage=1.6, pressure={0.5,0.9}, load=0.6, edge={found=0.2, soft=0.5, lost=0.3, period=50}, seed=14})
-- slender trees, thin lines
local b = brush{kind="round", width=5, point=0.6}
b:load(dk, 0.7)
b:stroke({{516,472},{518,400},{515,320},{513,250},{519,175}}, {pressure={0.8,0.2}, ramps={0.05,0.5}})
b:load(dk, 0.7)
b:stroke({{548,478},{546,400},{549,320},{553,255}}, {pressure={0.7,0.2}, ramps={0.05,0.5}})
print(wait(0))

--@ chunk 7
blend(land - oak, {angle=0.03, coverage=2, seed=21})
blend(oak:grow(10) + trunks, {angle=-0.7, coverage=2, seed=22})
print(wait(0))

--@ chunk 8
local rag = brush{kind="flat", width=16, stiffness=0.6}
-- wipe out light on the left flanks of the canopy clumps and some sky holes
local wipes = {
  {{640,215},{665,190},{700,170}}, {{720,140},{760,125},{800,118}},
  {{600,300},{615,265},{640,245}}, {{590,380},{600,345},{618,320}},
  {{690,250},{715,235},{745,232}}, {{830,180},{860,175},{890,190}},
  {{660,350},{690,335},{720,330}}, {{900,260},{925,270},{945,295}},
  {{600,420},{640,410},{670,405}},
}
for i, w in ipairs(wipes) do
  rag:wipe(1)
  rag:stroke(w, {pressure={0.7,0.4}, ramps={0.1,0.4}})
end
-- smaller holes where sky shows through
local hole = brush{kind="filbert", width=9}
for _, p in ipairs({{748,198},{835,250},{880,300},{690,300},{935,385},{812,350},{650,260},{905,210}}) do
  hole:wipe(1)
  hole:stroke({{p[1]-6,p[2]+3},{p[1]+6,p[2]-3}}, {pressure={0.8,0.6}})
end
print(wait(0))

--@ chunk 9
local r = brush{kind="filbert", width=12, stiffness=0.7}
-- test: repeated short lifts in one spot near the top right edge
for i = 1, 6 do
  r:wipe(1)
  r:stroke({{930,250},{955,262}}, {pressure={0.9,0.8}, ramps={0.05,0.1}})
end
print(drying(930,250))

--@ chunk 10
rag = brush{kind="filbert", width=12, stiffness=0.7}
function lift(pts, n, pr, br)
  br = br or rag
  for i = 1, n do
    br:wipe(1)
    br:stroke(pts, {pressure={pr or 0.9, (pr or 0.9)*0.8}, ramps={0.05,0.15}})
  end
end
-- notch in crown top
lift({{835,128},{832,150},{828,175}}, 6)
lift({{850,135},{842,160}}, 5)
-- right flank: bites
lift({{975,300},{955,320},{945,345}}, 6)
lift({{985,350},{960,365}}, 6)
lift({{950,200},{935,220}}, 5)
lift({{915,170},{900,185}}, 5)
-- left flank bites
lift({{588,330},{605,340}}, 5)
lift({{598,255},{618,262}}, 5)
lift({{640,200},{655,215}}, 5)
lift({{690,140},{700,160}}, 5)
-- sky holes
lift({{745,205},{758,198}}, 6, 0.8, brush{kind="filbert", width=8, stiffness=0.7})
lift({{838,262},{850,256}}, 6, 0.8, brush{kind="filbert", width=8, stiffness=0.7})
lift({{700,312},{712,306}}, 6, 0.8, brush{kind="filbert", width=7, stiffness=0.7})
lift({{885,335},{896,331}}, 6, 0.8, brush{kind="filbert", width=8, stiffness=0.7})
lift({{905,230},{915,226}}, 6, 0.8, brush{kind="filbert", width=7, stiffness=0.7})
print(wait(0))

--@ chunk 11
-- deepen the foreground with a thin dark umber, scrubbed
local fdk = pile{{"raw umber", 1}, {"bone black", 0.15}, {"raw sienna", 0.3}, medium=0.65}
local f2 = below({{0,600},{300,590},{500,610},{700,580},{1000,560}})
work(f2, {hand="scumble", tool="filbert 16", pile=fdk, angle=-0.05, coverage=1.0, pressure={0.4,0.7}, load=0.4, edge="lost", seed=31})
blend(f2:soften(20), {angle=0.05, coverage=1.5, seed=32})
print(wait(0))

--@ chunk 12
print(wait(24*60))
for _, p in ipairs({{760,300},{700,620},{300,450},{200,395},{518,300}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 13
print(wait(2*24*60))
for _, p in ipairs({{760,300},{700,620},{300,450},{200,395},{800,420}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 14
sk_top = pile{{"lead white", 5}, {"cobalt blue", 0.6}, {"raw umber", 0.5}, {"Indian red", 0.25}, medium=0.1}
sk_mid = pile{{"lead white", 6}, {"cobalt blue", 0.3}, {"raw umber", 0.3}, {"Indian red", 0.25}, {"yellow ochre", 0.3}, medium=0.1}
sk_low = pile{{"lead white", 6}, {"yellow ochre", 0.7}, {"red earth", 0.2}, {"raw umber", 0.05}, medium=0.1}
sk_glow = pile{{"lead white", 5}, {"lemon chrome", 1.0}, {"cadmium yellow", 0.3}, {"orange chrome", 0.15}, medium=0.1}
sk_rose = pile{{"lead white", 5}, {"orange chrome", 0.35}, {"Indian red", 0.2}, {"yellow ochre", 0.3}, medium=0.1}

--@ chunk 15
sk_top = pile{{"lead white", 4}, {"cobalt blue", 1.0}, {"raw umber", 0.6}, {"Indian red", 0.35}, {"bone black", 0.05}, medium=0.1}
sk_mid = pile{{"lead white", 5}, {"cobalt blue", 0.5}, {"raw umber", 0.3}, {"Indian red", 0.3}, {"yellow ochre", 0.2}, medium=0.1}
sk_rose = pile{{"lead white", 5}, {"Indian red", 0.5}, {"orange chrome", 0.3}, {"yellow ochre", 0.2}, medium=0.1}
sk_glow = pile{{"lead white", 5}, {"lemon chrome", 1.2}, {"cadmium yellow", 0.5}, {"orange chrome", 0.2}, medium=0.1}

--@ chunk 16
oak2 = outline{{612,446},{588,410},{594,372},{579,338},{597,304},{606,268},{630,246},{638,212},{664,186},{688,158},{716,146},{742,120},{772,110},{800,118},{818,136},{828,160},{846,166},{862,152},{884,160},{900,186},{926,206},{938,240},{926,268},{950,290},{972,318},{980,352},{964,384},{950,412},{922,436},{880,446},{830,452},{780,450},{720,452},{660,450},
  closed=true, char="soft", seed=41, amount=1.0, lobe=18}
oakm = oak2:mask()
skym = above({{0,402},{300,398},{600,396},{1000,400}})
print(oakm:area(), skym:area())

--@ chunk 17
-- debug view: lightly sketch the new silhouette with a 2H pencil so I can see it
local h = pencil("2H")
for _, path in ipairs(oak2:paths()) do h:line(path, {pressure=0.25}) end

--@ chunk 18
local o1 = outline{{625,448},{600,420},{604,392},{586,365},{598,335},{590,308},{612,282},{608,255},{632,236},{650,214},{672,206},{684,178},{708,160},{722,134},{752,114},{782,110},{808,124},{826,150},{846,160},{860,186},{870,214},{862,244},{880,270},{875,300},{888,330},{880,366},{895,392},{880,420},{860,446},{800,452},{740,452},{680,452},
  closed=true, char="soft", seed=42, amount=1.0, lobe=16}
local o2 = outline{{880,446},{870,410},{880,380},{875,345},{892,315},{900,282},{918,258},{944,246},{966,256},{980,282},{978,312},{992,340},{988,380},{975,412},{960,440},{920,450},
  closed=true, char="soft", seed=43, amount=1.0, lobe=14}
oakm = o1:mask() + o2:mask()
print(oakm:area())

--@ chunk 19
local skyonly = skym - oakm
local n1 = noise{seed=5, octaves=3, period=260, stretch={0, 3}}
local function band(y0, y1)
  return mask(function(x, y)
    local d = n1(x, y) * 30
    return (y > y0 + d and y < y1 + d) and 1 or 0
  end)
end
top_b = band(-50, 150) * skyonly
mid_b = band(130, 270) * skyonly
low_b = band(250, 345) * skyonly
hor_b = band(330, 420) * skyonly
local opts = function(p, s) return {hand="body", tool="filbert 14", pile=p, angle=function(x,y) return 0.04*math.sin(x/90) end,
  angle_jitter=0.15, coverage=1.6, pressure={0.6,0.9}, load=0.75, fill=true, clip=skyonly, length={40,90}, seed=s} end
work(top_b, opts(sk_top, 51))
work(mid_b, opts(sk_mid, 52))
work(low_b, opts(sk_rose, 53))
work(hor_b, opts(sk_low, 54))
print(wait(0))

--@ chunk 20
local skyonly = skym - oakm
local g = (ellipse(320, 372, 240, 48):roughen(12, 60, 61) * skyonly)
work(g, {hand="body", tool="filbert 12", pile=sk_glow, angle=0, angle_jitter=0.1, coverage=1.6, pressure={0.6,0.9}, load=0.8, fill=true, clip=skyonly, length={40,90}, seed=62})
local g2 = (ellipse(330, 390, 140, 18):roughen(6, 40, 63) * skyonly)
local glow2 = pile{{"lead white", 4}, {"lemon chrome", 1.0}, {"cadmium yellow", 0.6}, medium=0.05}
work(g2, {hand="body", tool="filbert 10", pile=glow2, angle=0, coverage=1.8, pressure={0.7,0.9}, load=0.85, fill=true, clip=skyonly, length={30,70}, seed=64})
print(wait(0))

--@ chunk 21
blend(skym - oakm, {angle=0.0, coverage=2.5, seed=71})
print(wait(0))

--@ chunk 22
cl_dk = pile{{"lead white", 3}, {"cobalt blue", 0.6}, {"raw umber", 0.55}, {"Indian red", 0.4}, medium=0.15}
cl_rose = pile{{"lead white", 4}, {"Indian red", 0.55}, {"orange chrome", 0.45}, {"yellow ochre", 0.2}, medium=0.15}
local b = brush{kind="filbert", width=9, stiffness=0.5}
-- low cloud bars across the glow
local bars = {
  {{40,305},{120,300},{200,303},{280,298},{340,302}},
  {{150,322},{230,318},{300,321},{380,316},{440,320}},
  {{380,292},{450,289},{520,294},{575,290}},
  {{0,335},{50,333},{110,336}},
}
for i, p in ipairs(bars) do
  b:load(cl_dk, 0.35)
  b:stroke(p, {pressure={0.45,0.25}, ramps={0.2,0.4}, swell={0.8,1.2,0.7}, shake=0.5})
end
-- rose-lit cloud masses higher up, left half
local sk = skym - oakm
local cm = (ellipse(200, 205, 150, 22):roughen(14, 50, 81) + ellipse(420, 175, 120, 16):roughen(10, 45, 82) + ellipse(80, 240, 90, 14):roughen(10, 40, 83)) * sk
work(cm, {hand="scumble", tool="filbert 10", pile=cl_rose, angle=0.02, coverage=1.0, pressure={0.35,0.6}, load=0.4, seed=84})
print(wait(0))

--@ chunk 23
local sk = skym - oakm
-- soften the rose cloud into the sky
local cm = (ellipse(200, 205, 175, 40) + ellipse(420, 175, 145, 32) + ellipse(80, 240, 110, 30)) * sk
blend(cm, {angle=0.05, coverage=2.0, seed=91})
-- broader cloud bars
local b = brush{kind="filbert", width=14, stiffness=0.45}
local bars = {
  {{30,306},{110,300},{200,304},{280,299},{345,303}},
  {{160,324},{235,319},{305,322},{385,317},{445,321}},
  {{385,292},{455,288},{520,293},{580,290}},
}
for i, p in ipairs(bars) do
  b:load(cl_dk, 0.5)
  b:stroke(p, {pressure={0.55,0.35}, ramps={0.25,0.4}, swell={0.7,1.2,0.6}, shake=0.6})
end
local barm = ribbon({{20,304},{350,302}}, 26) + ribbon({{150,322},{450,320}}, 26) + ribbon({{380,291},{590,291}}, 24)
blend(barm * sk, {angle=0, coverage=1.5, seed=92})
print(wait(0))

--@ chunk 24
local tn = noise{seed=101, octaves=4, period=60, persistence=0.55}
local tn2 = noise{seed=102, octaves=2, period=220}
treeline = mask(function(x, y)
  local base = 385 + tn2(x, 0) * 12
  -- lower and thinner behind the glow, higher toward the right
  if x > 80 and x < 520 then base = base + 8 * math.sin((x - 80) / 440 * math.pi) end
  if x > 520 then base = base - (x - 520) * 0.03 end
  local top = base + tn(x, 0) * 10
  return (y > top and y < 418) and 1 or 0
end) - oakm
far_c = pile{{"lead white", 3}, {"cobalt blue", 0.7}, {"Indian red", 0.35}, {"raw umber", 0.5}, {"yellow ochre", 0.1}, medium=0.15}
far_w = pile{{"lead white", 4}, {"cobalt blue", 0.35}, {"Indian red", 0.35}, {"yellow ochre", 0.4}, {"raw umber", 0.25}, medium=0.15}

--@ chunk 25
far_c = pile{{"lead white", 1.4}, {"cobalt blue", 0.8}, {"Indian red", 0.4}, {"raw umber", 0.7}, medium=0.15}
far_w = pile{{"lead white", 2.2}, {"cobalt blue", 0.45}, {"Indian red", 0.45}, {"yellow ochre", 0.3}, {"raw umber", 0.45}, medium=0.15}

--@ chunk 26
local glowzone = ellipse(310, 400, 230, 60):blur(40)
local tl_w = treeline * glowzone:map(function(v) return v > 0.5 and 1 or 0 end)
local tl_c = treeline - tl_w
work(tl_c, {hand="body", tool="filbert 8", pile=far_c, angle=function(x,y) return -1.4 + 0.3*math.sin(x/23) end, angle_jitter=0.4, length={10,24}, coverage=2.0, pressure={0.5,0.85}, load=0.7, fill=true, edge={found=0.2, soft=0.6, lost=0.2, period=30}, seed=111})
work(tl_w, {hand="body", tool="filbert 8", pile=far_w, angle=function(x,y) return -1.4 + 0.3*math.sin(x/23) end, angle_jitter=0.4, length={10,24}, coverage=2.0, pressure={0.5,0.85}, load=0.7, fill=true, edge={found=0.1, soft=0.6, lost=0.3, period=30}, seed=112})
print(wait(0))

--@ chunk 27
tr_dk = pile{{"raw umber", 1}, {"Antwerp blue", 0.3}, {"bone black", 0.25}, {"raw sienna", 0.5}, {"lead white", 0.12}, medium=0.2}
tr_mid = pile{{"raw sienna", 1}, {"Antwerp blue", 0.3}, {"raw umber", 0.7}, {"yellow ochre", 0.35}, {"lead white", 0.3}, medium=0.2}
tr_lit = pile{{"yellow ochre", 1}, {"raw sienna", 0.4}, {"Antwerp blue", 0.12}, {"lead white", 0.6}, {"orange chrome", 0.1}, medium=0.15}
tr_rus = pile{{"Indian red", 0.6}, {"raw sienna", 0.8}, {"raw umber", 0.6}, {"orange chrome", 0.2}, {"lead white", 0.2}, medium=0.2}

--@ chunk 28
tr_lit = pile{{"yellow ochre", 1}, {"raw sienna", 0.5}, {"raw umber", 0.4}, {"Antwerp blue", 0.15}, {"lead white", 0.35}, medium=0.15}
-- body of the oaks, dark olive, short strokes curling with the clumps
work(oakm, {hand="body", tool="filbert 10", pile=tr_dk, angle=function(x,y) return 0.8*math.sin(x/37 + y/29) end, angle_jitter=0.5, length={14,32}, coverage=2.0, pressure={0.55,0.9}, load=0.7, fill=true, edge={found=0.25, soft=0.5, lost=0.25, period=35}, seed=121})
print(wait(0))

--@ chunk 29
clumps = {
  {705,178,42},{770,138,42},{815,170,36},{652,238,42},{722,238,50},{798,232,50},{846,222,32},
  {628,318,40},{702,322,52},{786,322,52},{852,312,32},{640,400,42},{722,402,52},{818,404,52},
  {940,282,38},{918,342,38},{962,360,32},{930,412,40},{880,420,30}
}
function clump_parts(lx, ly, inner, outer)
  local lit, sh = nil, nil
  for i, c in ipairs(clumps) do
    local cx, cy, r = c[1], c[2], c[3]
    local e = ellipse(cx, cy, r, r*0.85):roughen(r*0.25, r*0.6, 200+i)
    local l = e * ellipse(cx + lx*r*0.55, cy + ly*r*0.55, r*inner, r*inner*0.8):roughen(r*0.2, r*0.5, 300+i)
    local s = e * ellipse(cx - lx*r*0.6, cy - ly*r*0.6, r*outer, r*outer*0.85):roughen(r*0.2, r*0.5, 400+i)
    lit = lit and (lit + l) or l
    sh = sh and (sh + s) or s
  end
  return lit * oakm, sh * oakm
end
oak_lit, oak_sh = clump_parts(-0.8, -0.6, 0.65, 0.75)
print(oak_lit:area(), oak_sh:area())

--@ chunk 30
tr_deep = pile{{"raw umber", 1}, {"Antwerp blue", 0.4}, {"bone black", 0.45}, {"raw sienna", 0.3}, medium=0.3}
-- shadow sides: deep transparent-ish dark, into the wet olive
work(oak_sh, {hand="body", tool="filbert 8", pile=tr_deep, angle=function(x,y) return 0.8*math.sin(x/31 + y/23) end, angle_jitter=0.5, length={10,24}, coverage=1.4, pressure={0.5,0.85}, load=0.55, edge="soft", seed=131})
-- lower canopy and underside darker overall
local under = oakm * below({{560,370},{700,385},{850,375},{1000,370}})
work(under, {hand="body", tool="filbert 10", pile=tr_deep, angle=0.1, angle_jitter=0.6, length={14,30}, coverage=1.2, pressure={0.5,0.8}, load=0.5, edge="soft", seed=132})
-- lit sides: mid olive, then a little russet
work(oak_lit, {hand="scumble", tool="filbert 7", pile=tr_mid, angle=function(x,y) return 0.8*math.sin(x/31 + y/23) end, coverage=1.3, pressure={0.4,0.75}, load=0.55, edge="soft", seed=133})
print(wait(0))

--@ chunk 31
blend(oakm:shrink(4), {angle=0.6, coverage=2.0, seed=141})
blend(oakm:shrink(4), {angle=-0.7, coverage=1.5, seed=142})
print(wait(0))

--@ chunk 32
md_far = pile{{"lead white", 2}, {"yellow ochre", 1}, {"raw umber", 0.3}, {"cobalt blue", 0.15}, {"Indian red", 0.15}, medium=0.15}
md_gold = pile{{"yellow ochre", 1}, {"raw sienna", 0.6}, {"lead white", 0.5}, {"raw umber", 0.3}, {"Antwerp blue", 0.08}, medium=0.15}
md_olv = pile{{"raw sienna", 1}, {"raw umber", 0.7}, {"Antwerp blue", 0.2}, {"yellow ochre", 0.5}, {"lead white", 0.2}, medium=0.15}
md_dk = pile{{"raw umber", 1}, {"raw sienna", 0.5}, {"Antwerp blue", 0.15}, {"bone black", 0.15}, medium=0.2}
pond_c = pile{{"lead white", 5}, {"lemon chrome", 0.8}, {"cadmium yellow", 0.3}, {"orange chrome", 0.1}, medium=0.1}

--@ chunk 33
pondo = outline{{165,503,"c"},{205,492},{260,487},{330,485},{400,488},{455,492,"c"},{430,500},{380,507},{310,513},{240,514},{190,511},
  closed=true, char="soft", seed=151, amount=0.6, lobe=10}
pondm = pondo:mask()
meadow = (below({{0,410},{1000,412}}) * above({{0,575},{200,560},{400,566},{560,585},{700,548},{850,530},{1000,525}})) - oakm - pondm
local far_strip = meadow * above({{0,442},{300,446},{600,450},{1000,452}})
local mid_l = (meadow - far_strip) * mask(function(x,y) return x < 560 and 1 or 0 end)
local mid_r = (meadow - far_strip) - mid_l
work(far_strip, {hand="body", tool="filbert 10", pile=md_far, angle=0, angle_jitter=0.08, length={30,70}, coverage=1.8, pressure={0.5,0.85}, load=0.7, fill=true, edge="soft", seed=152})
work(mid_l, {hand="body", tool="filbert 12", pile=md_gold, angle=0.02, angle_jitter=0.12, length={30,80}, coverage=1.8, pressure={0.5,0.85}, load=0.7, fill=true, edge="soft", seed=153})
work(mid_r, {hand="body", tool="filbert 12", pile=md_olv, angle=-0.05, angle_jitter=0.12, length={30,80}, coverage=1.8, pressure={0.5,0.85}, load=0.7, fill=true, edge="soft", seed=154})
print(wait(0))

--@ chunk 34
blend(meadow * mask(function(x,y) return (x > 470 and x < 660) and 1 or 0 end):blur(30), {angle=0.0, coverage=2.5, seed=161})
-- modulate: darker olive washes across the lower meadow and toward the right, wet into wet
local n = noise{seed=162, octaves=3, period=140, stretch={0, 4}}
local dkpatch = meadow * mask(function(x,y) return (n(x,y) + (y-470)/150 + (x-500)/900 > 0.35) and 1 or 0 end):blur(6)
work(dkpatch, {hand="scumble", tool="filbert 12", pile=md_dk, angle=0.0, angle_jitter=0.1, length={30,70}, coverage=1.0, pressure={0.35,0.6}, load=0.4, edge="lost", seed=163})
print(wait(0))

--@ chunk 35
blend(meadow, {angle=0.02, coverage=3, seed=171})
print(wait(0))

--@ chunk 36
pondo = outline{{150,503,"c"},{190,490},{250,484},{330,482},{410,485},{478,492,"c"},{450,501},{395,509},{320,516},{240,518},{180,513},
  closed=true, char="soft", seed=181, amount=0.5, lobe=10}
pondm = pondo:mask()
work(pondm, {hand="body", tool="filbert 8", pile=pond_c, angle=0, angle_jitter=0.04, length={30,70}, coverage=2.0, pressure={0.6,0.9}, load=0.8, fill=true, edge={found=0.5, soft=0.5, period=30}, seed=182})
-- cooler, rosier toward the near edge (reflecting higher sky)
local near = pondm * below({{0,506},{1000,506}})
local pond_r = pile{{"lead white", 5}, {"Indian red", 0.25}, {"cobalt blue", 0.2}, {"yellow ochre", 0.3}, medium=0.1}
work(near, {hand="body", tool="filbert 6", pile=pond_r, angle=0, length={20,50}, coverage=1.2, pressure={0.5,0.8}, load=0.6, clip=pondm, seed=183})
blend(pondm, {angle=0, coverage=2, seed=184})
print(wait(0))

--@ chunk 37
fg = below({{0,548},{90,552},{180,560},{280,566},{380,572},{470,580},{540,586},{600,578},{680,556},{760,540},{860,532},{1000,528}}):roughen(6, 30, 191)
fg_dk = pile{{"raw umber", 1}, {"bone black", 0.2}, {"raw sienna", 0.4}, {"Antwerp blue", 0.1}, {"lead white", 0.05}, medium=0.2}
fg_rus = pile{{"raw umber", 0.8}, {"Indian red", 0.4}, {"raw sienna", 0.6}, {"yellow ochre", 0.3}, {"lead white", 0.1}, medium=0.2}
work(fg, {hand="body", tool="filbert 14", pile=fg_dk, angle=function(x,y) return -0.15 + 0.2*math.sin(x/70) end, angle_jitter=0.25, length={30,80}, coverage=1.8, pressure={0.55,0.9}, load=0.7, fill=true, edge={found=0.2, soft=0.5, lost=0.3, period=40}, seed=192})
local n = noise{seed=193, octaves=3, period=110}
local rusm = fg * mask(function(x,y) return (n(x,y) > 0.1 and y < 640) and 1 or 0 end):blur(5)
work(rusm, {hand="scumble", tool="filbert 10", pile=fg_rus, angle=-0.2, length={20,50}, coverage=1.0, pressure={0.4,0.7}, load=0.45, edge="lost", seed=194})
print(wait(0))

--@ chunk 38
blend(fg, {angle=-0.1, coverage=2.5, seed=201})
blend(fg * above({{0,600},{1000,590}}):blur(10), {angle=0.3, coverage=1.5, seed=202})
print(wait(0))

--@ chunk 39
print(wait(3*24*60))
for _, p in ipairs({{760,300},{700,620},{300,450},{300,500},{200,395},{300,360},{500,100},{900,600}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 40
print(wait(3*24*60))
for _, p in ipairs({{760,300},{650,400},{900,350},{200,395},{50,395}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 41
print(wait(5*24*60))
for _, p in ipairs({{760,300},{650,400},{720,200},{800,420}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 42
gl_land = pile{{"raw umber", 1}, {"raw sienna", 0.6}, {"bone black", 0.15}, {"Antwerp blue", 0.08}, medium=0.8}
gl_tree = pile{{"raw umber", 1}, {"bone black", 0.35}, {"Antwerp blue", 0.3}, {"raw sienna", 0.3}, medium=0.75}

--@ chunk 43
landm = below({{0,412},{1000,414}}) - pondm
local landg = landm * below({{0,425},{1000,425}})
work(landg, {hand="glaze", pile=gl_land, angle=0.0, angle_jitter=0.05, coverage=1.2, clip=landm,
  load_at=function(x,y) return clamp(0.15 + (y-430)/240*0.6 + (x-300)/1400, 0.12, 0.8) end, seed=211})
print(wait(0))

--@ chunk 44
gl_land2 = pile{{"raw umber", 1}, {"raw sienna", 0.4}, {"bone black", 0.3}, {"Antwerp blue", 0.12}, medium=0.6}
local nearg = landm * below({{0,470},{500,480},{1000,460}})
work(nearg, {hand="glaze", pile=gl_land2, angle=0.0, angle_jitter=0.05, coverage=2.0, clip=landm, pressure={0.5,0.8},
  load_at=function(x,y) return clamp(0.3 + (y-470)/180*0.7, 0.3, 1.0) end, seed=221})
print(wait(0))

--@ chunk 45
blend(landm * below({{0,440},{1000,440}}), {angle=0.0, coverage=2.5, seed=231})
print(wait(0))

--@ chunk 46
work(oakm, {hand="glaze", tool={kind="filbert", width=18, stiffness=0.3}, pile=gl_tree, angle=function(x,y) return 0.6*math.sin(x/50+y/40) end, coverage=2.2, clip=oakm:grow(2), pressure={0.5,0.85}, load=0.9, length={40,120}, seed=241})
print(wait(0))

--@ chunk 47
blend(oakm:shrink(3), {angle=-0.5, coverage=2.0, seed=251})
blend(oakm:shrink(3), {angle=0.8, coverage=1.5, seed=252})
print(wait(0))

--@ chunk 48
print(wait(4*24*60))
for _, p in ipairs({{760,300},{650,400},{720,200},{800,420},{300,600},{900,500}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 49
print(wait(4*24*60))
for _, p in ipairs({{760,300},{800,420},{700,350},{850,300}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 50
h_hi = pile{{"lead white", 5}, {"cobalt blue", 0.45}, {"raw umber", 0.35}, {"Indian red", 0.3}, {"yellow ochre", 0.2}, medium=0.1}
h_mid = pile{{"lead white", 5}, {"Indian red", 0.35}, {"cobalt blue", 0.25}, {"orange chrome", 0.15}, {"yellow ochre", 0.25}, {"raw umber", 0.15}, medium=0.1}
h_lo = pile{{"lead white", 6}, {"yellow ochre", 0.6}, {"orange chrome", 0.12}, {"Indian red", 0.12}, {"raw umber", 0.05}, medium=0.1}
-- channel between the two trees
chan = outline{{846,212},{868,206},{884,232},{878,262},{892,296},{884,330},{878,362},{866,388},{858,360},{852,322},{856,288},{846,252},
  closed=true, char="soft", seed=261, amount=1.0, lobe=8}
local cm = chan:mask()
local upper = cm * above({{0,300},{1000,300}})
local lower = cm - upper
work(upper, {hand="detail", tool="filbert 6", pile=h_mid, angle=-1.4, length={8,20}, coverage=2.5, pressure={0.6,0.9}, load=0.8, fill=true, seed=262})
work(lower, {hand="detail", tool="filbert 6", pile=h_lo, angle=-1.4, length={8,20}, coverage=2.5, pressure={0.6,0.9}, load=0.8, fill=true, seed=263})
print(wait(0))

--@ chunk 51
local cm = chan:mask()
local upper = cm * above({{0,285},{1000,285}})
local lower = cm - upper
work(upper, {hand="detail", tool="filbert 6", pile=sk_mid, angle=-1.4, length={8,20}, coverage=2.0, pressure={0.6,0.9}, load=0.8, fill=true, seed=271})
work(lower, {hand="detail", tool="filbert 6", pile=sk_rose, angle=-1.4, length={8,20}, coverage=2.0, pressure={0.6,0.9}, load=0.8, fill=true, seed=272})
blend(cm, {angle=-1.4, coverage=2, seed=273})
print(wait(0))

--@ chunk 52
sk_a = pile{{"lead white", 5}, {"cobalt blue", 0.55}, {"raw umber", 0.35}, {"Indian red", 0.32}, {"yellow ochre", 0.15}, medium=0.12}
sk_b = pile{{"lead white", 5}, {"cobalt blue", 0.3}, {"raw umber", 0.25}, {"Indian red", 0.38}, {"yellow ochre", 0.25}, {"orange chrome", 0.1}, medium=0.12}
sk_c = pile{{"lead white", 5}, {"Indian red", 0.3}, {"orange chrome", 0.25}, {"yellow ochre", 0.35}, {"cobalt blue", 0.08}, medium=0.12}
sk_d = pile{{"lead white", 5}, {"yellow ochre", 0.6}, {"lemon chrome", 0.3}, {"orange chrome", 0.1}, {"Indian red", 0.06}, medium=0.12}
local cnt = 0
local bands = {{0,200,sk_a},{200,275,sk_b},{275,350,sk_c},{350,420,sk_d}}
for i, bd in ipairs(bands) do
  local y0, y1, p = bd[1], bd[2], bd[3]
  cnt = cnt + lose(oakm, {pile=p, tool="filbert 5", reach={6, 14}, load=0.35, pressure={0.5, 0.05},
    where=function(x,y) return (y >= y0 and y < y1 and y < 430) and 1 or 0 end, seed=280+i})
end
print(cnt)
print(wait(0))

--@ chunk 53
crown = outline{{612,405},{598,385},{590,355},{600,330},{594,300},{612,275},{618,248},{640,228},{655,200},{680,180},{700,160},{722,140},{745,122},{772,114},{800,121},{818,140},{835,150},{852,172},{858,200},{870,225},{866,255},{876,285},{872,320},{880,350},{872,380},{860,402},{830,410},{800,402},{770,410},{745,404},{715,412},{685,404},{655,410},{630,404},
  closed=true, char="soft", seed=301, amount=1.1, lobe=14}
crown2 = outline{{890,408},{884,378},{892,348},{886,318},{898,292},{912,270},{930,254},{952,250},{970,262},{982,285},{990,315},{998,345},{996,380},{986,404},{960,412},{930,406},
  closed=true, char="soft", seed=302, amount=1.1, lobe=12}
crownm = crown:mask() + crown2:mask()
local tn = noise{seed=101, octaves=4, period=60, persistence=0.55}
local tn2 = noise{seed=102, octaves=2, period=220}
tl_top = function(x)
  local base = 385 + tn2(x, 0) * 12
  if x > 80 and x < 520 then base = base + 8 * math.sin((x - 80) / 440 * math.pi) end
  if x > 520 then base = base - (x - 520) * 0.03 end
  return base + tn(x, 0) * 10
end
treeline_all = mask(function(x, y) local t = tl_top(x); return (y > t and y < 418) and 1 or 0 end)
sky_all = mask(function(x, y) return y <= tl_top(x) and 1 or 0 end)
print(crownm:area(), sky_all:area())
-- show new silhouette faintly
local h = pencil("H")
for _, path in ipairs(crown:paths()) do h:line(path, {pressure=0.3}) end
for _, path in ipairs(crown2:paths()) do h:line(path, {pressure=0.3}) end

--@ chunk 54
print(wait(2*24*60))
for _, p in ipairs({{870,250},{600,330},{760,118}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 55
ns1 = pile{{"lead white", 3.2}, {"cobalt blue", 0.7}, {"Indian red", 0.42}, {"raw umber", 0.5}, medium=0.12}
ns2 = pile{{"lead white", 4.5}, {"cobalt blue", 0.4}, {"Indian red", 0.38}, {"raw umber", 0.3}, {"yellow ochre", 0.15}, medium=0.12}
ns3 = pile{{"lead white", 5}, {"Indian red", 0.32}, {"orange chrome", 0.25}, {"yellow ochre", 0.35}, {"cobalt blue", 0.08}, medium=0.12}
ns4 = pile{{"lead white", 5}, {"yellow ochre", 0.55}, {"lemon chrome", 0.45}, {"orange chrome", 0.1}, medium=0.12}
ns5 = pile{{"lead white", 5}, {"lemon chrome", 1.1}, {"cadmium yellow", 0.4}, {"orange chrome", 0.06}, medium=0.1}

--@ chunk 56
local core = crownm:shrink(14)
local skyp = sky_all - core
local nz = noise{seed=311, octaves=4, period=180, persistence=0.5, stretch={0.05, 3}}
local function dist(x, y) return math.sqrt((x-330)^2 + ((y-380)*2.6)^2) + nz(x,y)*70 end
local function bandm(d0, d1) return skyp * mask(function(x,y) local d = dist(x,y); return (d >= d0 and d < d1) and 1 or 0 end) end
local ang = function(x,y) return 0.03*math.sin(x/110) - 0.02 end
local function lay(m, p, s, tool)
  work(m, {hand="body", tool=tool or "filbert 16", pile=p, angle=ang, angle_jitter=0.12, length={50,110}, coverage=1.7, pressure={0.6,0.9}, load=0.8, fill=true, clip=skyp, seed=s})
end
lay(bandm(760, 99999), ns1, 321)
lay(bandm(540, 760), ns2, 322)
lay(bandm(330, 540), ns3, 323)
lay(bandm(160, 330), ns4, 324, "filbert 12")
lay(bandm(-999, 160), ns5, 325, "filbert 12")
print(wait(0))

--@ chunk 57
local skyp = sky_all - crownm:shrink(14)
blend(skyp, {angle=0.0, coverage=2.5, seed=331})
blend(skyp, {angle=0.25, coverage=1.5, seed=332})
print(wait(0))

--@ chunk 58
cr_dk = pile{{"raw umber", 1}, {"Antwerp blue", 0.35}, {"bone black", 0.3}, {"raw sienna", 0.4}, {"yellow ochre", 0.15}, {"lead white", 0.15}, medium=0.15}
cr_warm = pile{{"raw sienna", 1}, {"raw umber", 0.6}, {"Antwerp blue", 0.15}, {"yellow ochre", 0.35}, {"Indian red", 0.12}, {"lead white", 0.25}, medium=0.15}
local ang = function(x,y) return 0.9*math.sin(x/33 + y/27) end
work(crownm, {hand="body", tool="filbert 9", pile=cr_dk, angle=ang, angle_jitter=0.6, length={10,26}, coverage=1.8, pressure={0.55,0.9}, load=0.7, fill=true,
  edge={found=0.15, soft=0.5, lost=0.35, period=28, reach=1.2}, seed=341})
print(wait(0))

--@ chunk 59
print(wait(3*24*60))
for _, p in ipairs({{760,300},{620,300},{870,250},{300,300},{950,300}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 60
print(wait(3*24*60))
for _, p in ipairs({{760,300},{620,300},{870,250},{950,300}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 61
function fol(cx, cy, rx, ry, seed, amt)
  local pts = {}
  local n = 11
  for i = 0, n-1 do
    local a = i / n * 2 * math.pi + rand(-0.15, 0.15)
    local r = 1 + rand(-0.18, 0.12)
    -- flatter bottoms
    local yy = math.sin(a)
    if yy > 0 then yy = yy * 0.75 end
    pts[#pts+1] = {cx + math.cos(a) * rx * r, cy + yy * ry * r}
  end
  pts.closed = true; pts.char = "soft"; pts.seed = seed; pts.amount = amt or 1.0; pts.lobe = math.max(6, rx / 5)
  return outline(pts)
end
masses = {
  {760,160,66,42}, {690,215,52,34}, {832,206,40,40}, {655,292,56,32}, {752,270,76,50},
  {842,292,38,42}, {628,372,44,26}, {748,362,84,38}, {846,378,38,28},
  {940,282,36,32}, {958,338,42,38}, {934,394,50,24},
}
folm = nil
for i, m in ipairs(masses) do
  local o = fol(m[1], m[2], m[3], m[4], 400 + i)
  folm = folm and (folm + o:mask()) or o:mask()
end
print(folm:area(), crownm:area())
local h = pencil("2H")
for i, m in ipairs(masses) do
  local o = fol(m[1], m[2], m[3], m[4], 400 + i)
  for _, path in ipairs(o:paths()) do h:line(path, {pressure=0.35}) end
end

--@ chunk 62
oldtree = (oakm + crownm):grow(5)
trunkm = ribbon({{736,470},{738,440},{733,410},{728,385}}, {13,12,10,8}) + ribbon({{948,472},{946,440},{950,410}}, {10,9,7})
local skyR = oldtree * sky_all - folm
local nz = noise{seed=311, octaves=4, period=180, persistence=0.5, stretch={0.05, 3}}
local function dist(x, y) return math.sqrt((x-330)^2 + ((y-380)*2.6)^2) + nz(x,y)*70 end
local function bandm(d0, d1) return skyR * mask(function(x,y) local d = dist(x,y); return (d >= d0 and d < d1) and 1 or 0 end) end
local function lay(m, p, s)
  work(m, {hand="body", tool="filbert 10", pile=p, angle=0.0, angle_jitter=0.15, length={20,50}, coverage=2.6, pressure={0.7,0.95}, load=0.9, fill=true, clip=skyR, seed=s})
end
lay(bandm(760, 99999), ns1, 351)
lay(bandm(540, 760), ns2, 352)
lay(bandm(330, 540), ns3, 353)
lay(bandm(160, 330), ns4, 354)
print(wait(0))

--@ chunk 63
local f1 = poly({{622,300},{660,296},{700,303},{740,300},{790,306},{830,312},{858,322},{862,345},{840,352},{800,346},{760,342},{720,345},{680,348},{640,346},{620,330}}, true):roughen(6, 22, 501)
local f2 = ellipse(742,200,32,16):roughen(5, 18, 502)
local f3 = ellipse(606,248,16,10):roughen(4,12,503) + ellipse(594,326,14,9):roughen(4,12,504) + ellipse(884,262,12,16):roughen(4,12,505)
local holes = ellipse(668,323,9,6) + ellipse(792,322,11,7) + ellipse(775,203,10,7)
fill2 = (f1 + f2 + f3) - holes
work(fill2, {hand="body", tool="filbert 7", pile=cr_dk, angle=function(x,y) return 0.9*math.sin(x/33 + y/27) end, angle_jitter=0.6, length={8,20}, coverage=2.2, pressure={0.6,0.9}, load=0.85, fill=true,
  edge={found=0.3, soft=0.5, lost=0.2, period=25}, seed=506})
folm = folm + fill2
print(wait(0))

--@ chunk 64
local und = mask(function(x,y) return (x > 598 and x < 1000 and y > 380 and y < 462) and 1 or 0 end) - folm:grow(1) - trunkm
local tn = noise{seed=101, octaves=4, period=60, persistence=0.55}
local farpart = und * mask(function(x,y) return (y < 419 + tn(x, 7)*2) and 1 or 0 end)
local meadpart = und - farpart
local md_far2 = pile{{"lead white", 2}, {"yellow ochre", 1}, {"raw umber", 0.6}, {"cobalt blue", 0.2}, {"Indian red", 0.15}, {"raw sienna", 0.3}, medium=0.15}
work(farpart, {hand="detail", tool="filbert 5", pile=far_c, angle=-1.5, angle_jitter=0.3, length={6,14}, coverage=2.5, pressure={0.6,0.9}, load=0.85, fill=true, seed=511})
work(meadpart, {hand="detail", tool="filbert 6", pile=md_far2, angle=0, angle_jitter=0.1, length={15,40}, coverage=2.5, pressure={0.6,0.9}, load=0.85, fill=true, seed=512})
print(wait(0))

--@ chunk 65
local reg = mask(function(x,y) return (x > 578 and x < 1000 and y > 340 and y < 466) and 1 or 0 end) - folm:grow(1) - trunkm
local tn = noise{seed=101, octaves=4, period=60, persistence=0.55}
local skypart = reg * mask(function(x,y) return (y < tl_top(x)) and 1 or 0 end)
local farp = reg * mask(function(x,y) local t = tl_top(x); return (y >= t and y < 409 + tn(x,3)*2) and 1 or 0 end)
local meadp = reg - skypart - farp
mg1 = pile{{"lead white", 2}, {"yellow ochre", 1}, {"raw sienna", 0.4}, {"raw umber", 0.45}, {"cobalt blue", 0.08}, medium=0.15}
mg2 = pile{{"lead white", 0.8}, {"yellow ochre", 1}, {"raw sienna", 0.6}, {"raw umber", 0.7}, {"Antwerp blue", 0.08}, medium=0.15}
local nz = noise{seed=311, octaves=4, period=180, persistence=0.5, stretch={0.05, 3}}
local function dist(x, y) return math.sqrt((x-330)^2 + ((y-380)*2.6)^2) + nz(x,y)*70 end
work(skypart * mask(function(x,y) return dist(x,y) < 540 and 1 or 0 end), {hand="detail", tool="filbert 5", pile=ns3, angle=0, length={8,20}, coverage=2.5, pressure={0.7,0.95}, load=0.9, fill=true, seed=521})
work(skypart * mask(function(x,y) return dist(x,y) >= 540 and 1 or 0 end), {hand="detail", tool="filbert 5", pile=ns2, angle=0, length={8,20}, coverage=2.5, pressure={0.7,0.95}, load=0.9, fill=true, seed=522})
work(farp, {hand="detail", tool="filbert 4", pile=far_c, angle=-1.5, angle_jitter=0.3, length={5,12}, coverage=2.5, pressure={0.6,0.9}, load=0.85, fill=true, seed=523})
local m_top = meadp * mask(function(x,y) return y < 432 and 1 or 0 end)
work(m_top, {hand="detail", tool="filbert 6", pile=mg1, angle=0, angle_jitter=0.08, length={15,40}, coverage=2.5, pressure={0.6,0.9}, load=0.85, fill=true, seed=524})
work(meadp - m_top, {hand="detail", tool="filbert 6", pile=mg2, angle=0, angle_jitter=0.08, length={15,40}, coverage=2.5, pressure={0.6,0.9}, load=0.85, fill=true, seed=525})
blend(meadp, {angle=0, coverage=2, seed=526})
print(wait(0))

--@ chunk 66
local reg = mask(function(x,y) return (x > 570 and x < 700 and y > 340 and y < 420) and 1 or 0 end) - folm:grow(1)
local nz = noise{seed=311, octaves=4, period=180, persistence=0.5, stretch={0.05, 3}}
local function dist(x, y) return math.sqrt((x-330)^2 + ((y-380)*2.6)^2) + nz(x,y)*70 end
local sp = reg * mask(function(x,y) return (y < tl_top(x) and dist(x,y) < 330) and 1 or 0 end)
work(sp, {hand="detail", tool="filbert 5", pile=ns4, angle=0, length={8,20}, coverage=2.5, pressure={0.7,0.95}, load=0.9, fill=true, seed=531})
blend(reg * mask(function(x,y) return (y < tl_top(x)) and 1 or 0 end), {angle=0, coverage=2, seed=532})
print(wait(0))

--@ chunk 67
print(wait(5*24*60))
for _, p in ipairs({{800,440},{620,380},{940,300},{700,250},{590,400}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 68
mb1 = pile{{"yellow ochre", 1}, {"raw sienna", 0.5}, {"raw umber", 0.6}, {"lead white", 0.6}, {"Antwerp blue", 0.06}, {"Indian red", 0.08}, medium=0.15}
mb2 = pile{{"raw sienna", 1}, {"raw umber", 0.9}, {"yellow ochre", 0.5}, {"Antwerp blue", 0.12}, {"lead white", 0.25}, medium=0.15}
local band = mask(function(x,y) return (x > 540 and y > 408 and y < 478) and 1 or 0 end) - trunkm:grow(1) - folm:grow(1)
local nz = noise{seed=541, octaves=3, period=120, stretch={0, 5}}
local lightm = band * mask(function(x,y) return (nz(x,y) * 0.6 + (440 - y)/40 - (x-560)/500 > 0) and 1 or 0 end)
work(band - lightm, {hand="body", tool="filbert 10", pile=mb2, angle=0, angle_jitter=0.06, length={30,80}, coverage=2.2, pressure={0.6,0.9}, load=0.8, fill=true, edge={found=0.1, soft=0.4, lost=0.5, period=40}, seed=542})
work(lightm, {hand="body", tool="filbert 8", pile=mb1, angle=0, angle_jitter=0.06, length={30,70}, coverage=2.2, pressure={0.6,0.9}, load=0.8, fill=true, edge="soft", seed=543})
blend(band:grow(6), {angle=0, coverage=2.5, seed=544})
print(wait(0))

--@ chunk 69
mh = pile{{"lead white", 2.5}, {"yellow ochre", 1}, {"raw umber", 0.5}, {"cobalt blue", 0.2}, {"Indian red", 0.15}, medium=0.15}
mo = pile{{"yellow ochre", 0.8}, {"raw sienna", 0.6}, {"raw umber", 1.0}, {"Antwerp blue", 0.15}, {"lead white", 0.5}, {"bone black", 0.05}, medium=0.15}
md = pile{{"raw umber", 1}, {"raw sienna", 0.5}, {"bone black", 0.2}, {"Antwerp blue", 0.12}, {"yellow ochre", 0.2}, medium=0.2}
local n1 = noise{seed=551, octaves=3, period=90, stretch={0, 5}}
local top = mask(function(x,y) return (x > 535 and y > 406 and y < 427 + n1(x,y)*5) and 1 or 0 end) - folm:grow(1)
local bot = mask(function(x,y) return (x > 520 and y > 455 + n1(x,y)*6 and y < 490) and 1 or 0 end)
local mid = mask(function(x,y) return (x > 535 and y > 420 and y < 465) and 1 or 0 end) - top - bot
work(mid, {hand="body", tool="filbert 10", pile=mo, angle=0, angle_jitter=0.05, length={30,80}, coverage=1.6, pressure={0.6,0.9}, load=0.6, edge="lost", seed=552})
work(top, {hand="body", tool="filbert 8", pile=mh, angle=0, angle_jitter=0.05, length={30,70}, coverage=1.8, pressure={0.6,0.9}, load=0.7, clip=top:grow(4), seed=553})
work(bot, {hand="body", tool="filbert 10", pile=md, angle=0, angle_jitter=0.08, length={30,80}, coverage=1.6, pressure={0.5,0.85}, load=0.6, edge="lost", seed=554})
print(wait(0))

--@ chunk 70
local z = (mask(function(x,y) return (x > 520 and y > 404 and y < 495) and 1 or 0 end) - folm:grow(2)):blur(8)
blend(z, {angle=0, coverage=3, seed=561})
blend(z, {angle=0.08, coverage=2, seed=562})
print(wait(0))

--@ chunk 71
mh_w = pile{{"lead white", 3}, {"yellow ochre", 1}, {"raw umber", 0.3}, {"lemon chrome", 0.2}, {"Indian red", 0.1}, {"cobalt blue", 0.08}, medium=0.15}
mo_w = pile{{"yellow ochre", 1}, {"raw sienna", 0.5}, {"raw umber", 0.5}, {"lead white", 0.8}, {"Antwerp blue", 0.06}, medium=0.15}
local n1 = noise{seed=571, octaves=3, period=110, stretch={0, 5}}
local mb = (mask(function(x,y) local t = 405; local b = 520 + n1(x, 3)*10 + (x > 520 and -(x-520)*0.03 or 0); return (y > t and y < b) and 1 or 0 end) - folm:grow(2) - pondm)
-- zones by a "distance from the glow" in the land, with a wobble
local function g(x, y) return (y - 405) / 110 + math.max(0, x - 300) / 900 + math.max(0, 200 - x) / 700 + n1(x, y) * 0.18 end
local z1 = mb * mask(function(x,y) return g(x,y) < 0.28 and 1 or 0 end)
local z2 = mb * mask(function(x,y) local v = g(x,y); return (v >= 0.28 and v < 0.6) and 1 or 0 end)
local z3 = mb * mask(function(x,y) local v = g(x,y); return (v >= 0.6 and v < 0.9) and 1 or 0 end)
local z4 = mb * mask(function(x,y) return g(x,y) >= 0.9 and 1 or 0 end)
local o = function(p, s) return {hand="body", tool="filbert 12", pile=p, angle=0, angle_jitter=0.06, length={40,100}, coverage=1.8, pressure={0.6,0.9}, load=0.75, fill=true, edge="soft", seed=s} end
work(z1, o(mh_w, 572))
work(z2, o(mo_w, 573))
work(z3, o(mo, 574))
work(z4, o(md, 575))
print(wait(0))

--@ chunk 72
local mb = mask(function(x,y) return (y > 404 and y < 535) and 1 or 0 end) - folm:grow(3) - pondm:shrink(2)
blend(mb, {angle=0, coverage=2.5, seed=581})
print(wait(0))

--@ chunk 73
print(wait(5*24*60))
for _, p in ipairs({{800,440},{300,450},{100,500},{740,430}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 74
trk = pile{{"raw umber", 1}, {"bone black", 0.35}, {"Indian red", 0.1}, {"lead white", 0.12}, medium=0.15}
local b = brush{kind="filbert", width=13, stiffness=0.6}
b:load(trk, 0.9)
b:stroke({{736,472},{738,445},{734,418},{730,395},{728,378}}, {pressure={0.95,0.7}, ramps={0.05,0.3}, shake=0.4})
b:load(trk, 0.9)
b:stroke({{742,471},{743,445},{740,420},{741,398}}, {pressure={0.9,0.6}, ramps={0.05,0.3}, shake=0.4})
local r = brush{kind="round", width=7, point=0.7, stiffness=0.6}
local limbs = {
  {{731,392},{715,372},{690,352},{668,338},{648,330}},
  {{736,388},{748,362},{760,338},{772,318}},
  {{740,396},{770,380},{805,368},{840,362}},
  {{730,385},{722,350},{716,318}},
  {{760,345},{790,330},{812,318}},
}
for i, l in ipairs(limbs) do
  r:load(trk, 0.8)
  r:stroke(l, {pressure={0.85,0.15}, ramps={0.05,0.6}, shake=0.6})
end
-- second tree trunk
local b2 = brush{kind="filbert", width=8, stiffness=0.6}
b2:load(trk, 0.9)
b2:stroke({{946,464},{947,440},{944,418},{946,400}}, {pressure={0.9,0.6}, ramps={0.05,0.3}, shake=0.4})
r:load(trk, 0.7)
r:stroke({{945,404},{938,380},{934,360}}, {pressure={0.7,0.15}, ramps={0.05,0.6}})
r:load(trk, 0.7)
r:stroke({{947,402},{958,380},{965,362}}, {pressure={0.7,0.15}, ramps={0.05,0.6}})
print(wait(0))

--@ chunk 75
print(wait(4*24*60))
for _, p in ipairs({{738,440},{715,372},{946,430}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 76
trk2 = pile{{"raw umber", 1}, {"bone black", 0.6}, {"Antwerp blue", 0.1}, {"lead white", 0.08}, medium=0.15}
local limbsR = ribbon({{731,392},{715,372},{690,352},{668,338},{648,330}}, 10) + ribbon({{736,388},{748,362},{760,338},{772,318}}, 10)
  + ribbon({{740,396},{770,380},{805,368},{840,362}}, 10) + ribbon({{730,385},{722,350},{716,318}}, 10) + ribbon({{760,345},{790,330},{812,318}}, 10)
  + ribbon({{945,404},{938,380},{934,360}}, 8) + ribbon({{947,402},{958,380},{965,362}}, 8)
local inside = limbsR * folm:shrink(1)
work(inside, {hand="detail", tool="filbert 5", pile=cr_dk, angle=function(x,y) return 0.9*math.sin(x/33 + y/27) end, angle_jitter=0.6, length={5,12}, coverage=3, pressure={0.7,0.95}, load=0.9, fill=true, seed=601})
-- trunks repainted darker, a touch narrower
local b = brush{kind="filbert", width=11, stiffness=0.6}
b:load(trk2, 0.95)
b:stroke({{737,474},{739,448},{735,420},{732,398}}, {pressure={0.95,0.8}, ramps={0.03,0.2}, shake=0.3})
b:load(trk2, 0.95)
b:stroke({{742,473},{743,448},{740,422},{739,400}}, {pressure={0.9,0.75}, ramps={0.03,0.2}, shake=0.3})
local b2 = brush{kind="filbert", width=7, stiffness=0.6}
b2:load(trk2, 0.95)
b2:stroke({{946,466},{947,440},{944,418},{946,402}}, {pressure={0.9,0.7}, ramps={0.03,0.2}, shake=0.3})
print(wait(0))

--@ chunk 77
f_dk = pile{{"raw umber", 1}, {"Antwerp blue", 0.3}, {"bone black", 0.25}, {"raw sienna", 0.6}, {"lead white", 0.06}, medium=0.15}
f_mid = pile{{"raw sienna", 1}, {"raw umber", 0.7}, {"Antwerp blue", 0.18}, {"yellow ochre", 0.4}, {"Indian red", 0.15}, {"lead white", 0.2}, medium=0.15}
f_lit = pile{{"yellow ochre", 1}, {"raw sienna", 0.5}, {"orange chrome", 0.25}, {"lead white", 0.45}, {"raw umber", 0.15}, {"Antwerp blue", 0.04}, medium=0.12}
f_cool = pile{{"lead white", 1}, {"raw umber", 0.6}, {"Antwerp blue", 0.12}, {"yellow ochre", 0.4}, {"Indian red", 0.08}, medium=0.12}
f_edge = pile{{"lead white", 2}, {"raw umber", 0.7}, {"cobalt blue", 0.3}, {"Indian red", 0.25}, {"yellow ochre", 0.3}, medium=0.12}

--@ chunk 78
local litm, shm, midm
for i, m in ipairs(masses) do
  local cx, cy, rx, ry = m[1], m[2], m[3], m[4]
  local e = ellipse(cx, cy, rx*1.05, ry*1.05)
  local l = e * ellipse(cx - rx*0.45, cy - ry*0.5, rx*0.75, ry*0.7):roughen(rx*0.15, rx*0.4, 610+i)
  local s = e * ellipse(cx + rx*0.2, cy + ry*0.65, rx*0.95, ry*0.6):roughen(rx*0.15, rx*0.4, 630+i)
  litm = litm and (litm + l) or l
  shm = shm and (shm + s) or s
end
litm = litm * folm
shm = (shm * folm) - litm
oak_l2, oak_s2 = litm, shm
local ang = function(x,y) return 0.9*math.sin(x/33 + y/27) end
work(shm, {hand="scumble", tool="filbert 8", pile=f_dk, angle=ang, angle_jitter=0.6, length={8,18}, coverage=1.4, pressure={0.5,0.85}, load=0.6, edge="soft", seed=641})
work(litm, {hand="scumble", tool="filbert 7", pile=f_mid, angle=ang, angle_jitter=0.6, length={8,18}, coverage=1.5, pressure={0.5,0.85}, load=0.6, edge="soft", seed=642})
print(wait(0))

--@ chunk 79
blend(folm:shrink(2), {angle=0.7, coverage=2.5, seed=651})
blend(folm:shrink(2), {angle=-0.6, coverage=2.0, seed=652})
print(wait(0))

--@ chunk 80
print(wait(5*24*60))
for _, p in ipairs({{700,250},{760,160},{940,300},{650,370}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 81
print(wait(5*24*60))
for _, p in ipairs({{700,250},{760,160},{940,300},{650,370}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 82
gl_t2 = pile{{"raw umber", 1}, {"bone black", 0.3}, {"Antwerp blue", 0.25}, {"raw sienna", 0.4}, medium=0.8}
local lit = oak_l2:blur(10)
work(folm:grow(1), {hand="glaze", tool={kind="filbert", width=14, stiffness=0.3}, pile=gl_t2, angle=function(x,y) return 0.6*math.sin(x/50+y/40) end,
  length={30,80}, coverage=1.6, clip=folm:grow(2), pressure={0.4,0.75},
  load_at=function(x,y) return 0.55 - 0.4*lit:at(x,y) end, seed=661})
print(wait(0))

--@ chunk 83
blend(folm:grow(1), {angle=0.5, coverage=2.5, seed=671})
blend(folm:grow(1), {angle=-0.8, coverage=2.0, seed=672})
print(wait(0))

--@ chunk 84
print(wait(6*24*60))
for _, p in ipairs({{700,250},{760,160},{940,300},{650,370}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 85
cd1 = pile{{"raw umber", 1}, {"Antwerp blue", 0.3}, {"bone black", 0.35}, {"raw sienna", 0.35}, {"lead white", 0.05}, medium=0.2}
cd2 = pile{{"raw umber", 1}, {"raw sienna", 0.7}, {"Antwerp blue", 0.2}, {"bone black", 0.15}, {"yellow ochre", 0.2}, {"lead white", 0.08}, medium=0.2}
-- light-side weight: upper-left of each mass, and whole tree's left flank facing the glow
local lit = oak_l2:blur(12)
local w = function(x,y) return clamp(lit:at(x,y)*0.8 + math.max(0, 680 - x)/250 * 0.4, 0, 1) end
local darkzone = folm * mask(function(x,y) return w(x,y) < 0.45 and 1 or 0 end)
local warmzone = folm - darkzone
local ang = function(x,y) return 0.9*math.sin(x/33 + y/27) end
work(darkzone, {hand="body", tool="filbert 8", pile=cd1, angle=ang, angle_jitter=0.6, length={8,20}, coverage=1.8, pressure={0.55,0.9}, load=0.7, fill=true, edge={found=0.2, soft=0.5, lost=0.3, period=25}, seed=681})
work(warmzone, {hand="body", tool="filbert 7", pile=cd2, angle=ang, angle_jitter=0.6, length={8,18}, coverage=1.5, pressure={0.5,0.85}, load=0.6, fill=false, edge="soft", seed=682})
print(wait(0))

--@ chunk 86
blend(folm, {angle=0.6, coverage=1.5, seed=691})
print(wait(0))

--@ chunk 87
-- distant clumps on the left horizon
dist_t = pile{{"lead white", 1.2}, {"cobalt blue", 0.7}, {"Indian red", 0.4}, {"raw umber", 0.8}, {"yellow ochre", 0.1}, medium=0.15}
local cl = {
  {60,385,34,16,701},{108,380,26,20,702},{150,388,30,12,703},{20,390,22,12,704},{470,382,20,12,705},{505,378,26,16,706}
}
local dm
for i, c in ipairs(cl) do
  local o = fol(c[1], c[2], c[3], c[4], c[5], 1.2)
  dm = dm and (dm + o:mask()) or o:mask()
end
dm = dm * above({{0,410},{1000,410}})
work(dm, {hand="detail", tool="filbert 5", pile=dist_t, angle=-1.3, angle_jitter=0.5, length={5,12}, coverage=2.2, pressure={0.55,0.85}, load=0.75, fill=true, edge={found=0.1, soft=0.5, lost=0.4, period=20}, seed=707})
print(wait(0))

--@ chunk 88
local cl = {
  {60,385,34,16,701},{108,380,26,20,702},{150,388,30,12,703},{20,390,22,12,704},{470,382,20,12,705},{505,378,26,16,706}
}
distm = nil
for i, c in ipairs(cl) do
  local o = fol(c[1], c[2], c[3], c[4], c[5], 1.2)
  distm = distm and (distm + o:mask()) or o:mask()
end
distm = distm * above({{0,410},{1000,410}})
blend(distm:grow(3), {angle=-1.3, coverage=1.5, seed=711})
-- warm haze scumbled lightly into the wet distant trees, more toward the glow
local hz = pile{{"lead white", 3}, {"yellow ochre", 0.5}, {"Indian red", 0.2}, {"cobalt blue", 0.15}, {"raw umber", 0.15}, medium=0.2}
work(distm, {hand="scumble", tool="filbert 6", pile=hz, angle=-0.2, length={8,16}, coverage=0.9, pressure={0.3,0.55}, load=0.3, clip=true, seed=712})
print(wait(0))

--@ chunk 89
blend(distm:grow(2), {angle=-0.3, coverage=3, seed=721})
blend(distm:grow(2), {angle=-1.4, coverage=2, seed=722})
print(wait(0))

--@ chunk 90
far_m = pile{{"lead white", 1.8}, {"cobalt blue", 0.5}, {"Indian red", 0.45}, {"raw umber", 0.55}, {"yellow ochre", 0.25}, medium=0.15}
work(distm, {hand="detail", tool="filbert 5", pile=far_m, angle=-1.3, angle_jitter=0.5, length={5,12}, coverage=1.6, pressure={0.55,0.85}, load=0.7, fill=true, seed=731})
blend(distm:grow(2), {angle=-1.2, coverage=2, seed=732})
print(wait(0))

--@ chunk 91
local nz = noise{seed=311, octaves=4, period=180, persistence=0.5, stretch={0.05, 3}}
local function dist(x, y) return math.sqrt((x-330)^2 + ((y-380)*2.6)^2) + nz(x,y)*70 end
local cnt = 0
local bands = {{760,99999,ns1},{540,760,ns2},{330,540,ns3},{0,330,ns4}}
local e = noise{seed=741, octaves=2, period=60}
for i, bd in ipairs(bands) do
  cnt = cnt + lose(folm, {pile=bd[3], tool="filbert 4", reach={5, 10}, load=0.25, pressure={0.4, 0.03},
    where=function(x,y) local d = dist(x,y); return (d >= bd[1] and d < bd[2] and y < 395 and e(x,y) > -0.2) and 1 or 0 end, seed=740+i})
end
print(cnt, wait(0))

--@ chunk 92
local zone = folm:grow(8)
blend(zone, {angle=0.7, coverage=3, seed=751})
blend(zone, {angle=-0.6, coverage=2.5, seed=752})
print(wait(0))

--@ chunk 93
sl_t = pile{{"raw umber", 1}, {"bone black", 0.3}, {"lead white", 0.2}, {"Indian red", 0.1}, medium=0.2}
sl_f = pile{{"raw umber", 1}, {"raw sienna", 0.5}, {"Antwerp blue", 0.2}, {"bone black", 0.15}, {"lead white", 0.35}, {"yellow ochre", 0.2}, medium=0.25}
local r = brush{kind="round", width=6, point=0.7, stiffness=0.55}
r:load(sl_t, 0.9)
r:stroke({{158,482},{160,440},{156,400},{160,350},{165,300},{162,262}}, {pressure={0.9,0.25}, ramps={0.03,0.5}, shake=0.5})
r:load(sl_t, 0.9)
r:stroke({{196,480},{193,440},{198,395},{205,345},{203,310}}, {pressure={0.75,0.2}, ramps={0.03,0.5}, shake=0.5})
local rg = brush{kind="rigger", width=2.5, point=1}
local tw = {
  {{160,360},{145,340},{130,330}}, {{159,330},{175,312},{188,300}}, {{162,300},{150,282},{140,270}},
  {{163,285},{172,268}}, {{199,370},{214,352},{226,345}}, {{202,340},{190,322},{182,312}}, {{204,320},{216,300}},
}
for i, t in ipairs(tw) do
  rg:load(sl_t, 0.6)
  rg:stroke(t, {pressure={0.7,0.05}, ramps={0.05,0.7}, shake=0.6})
end
print(wait(0))

--@ chunk 94
local fm = nil
local cl = {
  {150,330,26,22},{176,300,24,26},{142,280,18,16},{168,262,16,14},{160,365,20,12},
  {212,350,22,18},{196,318,18,18},{214,300,12,12},{188,345,12,10}
}
for i, c in ipairs(cl) do
  local o = fol(c[1], c[2], c[3], c[4], 800+i, 1.4)
  fm = fm and (fm + o:mask()) or o:mask()
end
slfol = fm:soften(6)
stipple(slfol, {pile=sl_f, width=5, coverage=function(x,y) return 1.4 * slfol:at(x,y) end, pressure={0.3,0.7}, dips={14, 0.5, 0.3}, cluster={0.6, 8}, feather=0.7, drag={3, -0.5}, seed=811})
print(wait(0))

--@ chunk 95
sl_d = pile{{"raw umber", 1}, {"bone black", 0.25}, {"Antwerp blue", 0.2}, {"raw sienna", 0.3}, {"lead white", 0.1}, medium=0.25}
local inner = slfol:shrink(5):blur(5)
stipple(inner, {pile=sl_d, width=4, coverage=function(x,y) return 0.9 * inner:at(x,y) end, pressure={0.3,0.65}, dips={12, 0.5, 0.3}, cluster={0.6, 6}, feather=0.7, drag={2, -0.5}, seed=821})
-- restate the trunk where it crosses the hazy clump
local r = brush{kind="round", width=5, point=0.6, stiffness=0.55}
r:load(sl_t, 0.9)
r:stroke({{159,430},{157,400},{158,375},{160,350}}, {pressure={0.75,0.6}, ramps={0.1,0.3}, shake=0.4})
print(wait(0))

--@ chunk 96
pathm = ribbon({{560,670},{585,630},{620,590},{655,555},{685,525},{705,503},{722,488}}, {70,58,44,32,22,14,8}):roughen(4, 20, 831)
path_c = pile{{"yellow ochre", 1}, {"lead white", 0.9}, {"raw umber", 0.5}, {"Indian red", 0.15}, {"cobalt blue", 0.05}, medium=0.2}
work(pathm, {hand="scumble", tool="filbert 8", pile=path_c, angle=function(x,y) return -0.9 + (y-500)/400 end, length={10,25}, coverage=1.1, pressure={0.35,0.65}, load=0.45, edge="loose", seed=832})
print(wait(0))

--@ chunk 97
local r = brush{kind="flat", width=14, stiffness=0.7}
local n = 0
for y = 495, 667, 6 do
  -- path centre at this y (approx from the ribbon)
  local t = (y - 488) / (670 - 488)
  local cx = 722 + (560 - 722) * t
  local hw = 8 + 34 * t
  for x = cx - hw - 6, cx + hw + 6, 12 do
    r:wipe(1)
    r:stroke({{x - 8, y + 3}, {x + 8, y - 3}}, {pressure={0.9, 0.8}, ramps={0.05, 0.1}})
    n = n + 1
  end
end
print(n, wait(0))

--@ chunk 98
print(wait(4*24*60))
for _, p in ipairs({{640,600},{700,250},{160,320},{100,385},{940,300}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 99
local n1 = noise{seed=851, octaves=3, period=130, stretch={0, 4}}
fgtop = function(x)
  if x < 470 then return 524 + (x < 150 and (150 - x) * 0.05 or 0) end
  if x < 760 then return 524 - (x - 470) / 290 * 42 end
  return 482 + (x - 760) * 0.02
end
fgz = mask(function(x,y) return y > fgtop(x) + n1(x, 0) * 4 and 1 or 0 end) - pondm:grow(2) - trunkm
local function g(x,y) return (y - fgtop(x)) / (667 - fgtop(x)) + n1(x,y) * 0.15 end
f1 = pile{{"raw umber", 1}, {"raw sienna", 0.6}, {"yellow ochre", 0.4}, {"Antwerp blue", 0.12}, {"lead white", 0.2}, medium=0.15}
f2 = pile{{"raw umber", 1}, {"raw sienna", 0.5}, {"Indian red", 0.2}, {"bone black", 0.1}, {"lead white", 0.08}, medium=0.15}
f3 = pile{{"raw umber", 1}, {"bone black", 0.25}, {"Indian red", 0.15}, {"raw sienna", 0.3}, medium=0.15}
local zA = fgz * mask(function(x,y) return g(x,y) < 0.25 and 1 or 0 end)
local zB = fgz * mask(function(x,y) local v = g(x,y); return (v >= 0.25 and v < 0.6) and 1 or 0 end)
local zC = fgz * mask(function(x,y) return g(x,y) >= 0.6 and 1 or 0 end)
local ang = function(x,y) return -0.06 + 0.12 * math.sin(x / 140 + y / 60) end
local o = function(p, s) return {hand="body", tool="filbert 14", pile=p, angle=ang, angle_jitter=0.15, length={40,100}, coverage=2.2, pressure={0.65,0.95}, load=0.85, fill=true, edge="soft", seed=s} end
work(zA, o(f1, 852))
work(zB, o(f2, 853))
work(zC, o(f3, 854))
print(wait(0))

--@ chunk 100
blend(fgz, {angle=-0.5, coverage=3, seed=861})
blend(fgz, {angle=0.6, coverage=2.5, seed=862})
blend(fgz, {angle=-1.2, coverage=2, seed=863})
print(wait(0))

--@ chunk 101
for _, p in ipairs({{700,250},{640,330},{860,200},{940,300},{160,300},{200,340}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 102
-- ragged foliage reaching out into the sky: small dark touches in a band outside the edge
local outer = (folm:grow(14) - folm:shrink(2)) * above({{0,400},{1000,400}})
local en = noise{seed=871, octaves=3, period=45}
local cov = function(x,y) local d = outer:at(x,y); return d * clamp(0.5 + en(x,y) * 0.9, 0, 1.2) * 0.9 end
fe1 = pile{{"raw umber", 1}, {"Antwerp blue", 0.25}, {"bone black", 0.25}, {"raw sienna", 0.4}, {"lead white", 0.35}, {"yellow ochre", 0.15}, medium=0.2}
stipple(outer, {pile=fe1, width=4.5, coverage=cov, pressure={0.25,0.6}, dips={10, 0.45, 0.4}, cluster={0.7, 10}, feather=0.8, drag={3, -0.6}, twist=0.5, seed=872})
print(wait(0))

--@ chunk 103
fe2 = pile{{"raw umber", 1}, {"Antwerp blue", 0.25}, {"bone black", 0.2}, {"raw sienna", 0.45}, {"lead white", 0.22}, {"yellow ochre", 0.2}, medium=0.2}
local b = brush{kind="filbert", width=4, stiffness=0.55}
local count = 0
for i, m in ipairs(masses) do
  local cx, cy, rx, ry = m[1], m[2], m[3], m[4]
  local nstr = math.floor((rx + ry) * 0.55)
  for k = 1, nstr do
    local a = rand(0, 2*math.pi)
    local ex, ey = cx + math.cos(a) * rx, cy + math.sin(a) * ry * (math.sin(a) > 0 and 0.75 or 1)
    -- only where this point is on the outer edge of the whole tree, and above the meadow
    local ox, oy = ex + math.cos(a) * 7, ey + math.sin(a) * 7
    if folm:at(ox, oy) < 0.3 and ey < 395 and math.sin(a) < 0.5 then
      local aj = a + rand(-0.6, 0.6)
      local L = rand(8, 20)
      if count % 6 == 0 then b:load(fe2, 0.6) end
      b:stroke({{ex - math.cos(aj) * 5, ey - math.sin(aj) * 5}, {ex + math.cos(aj) * L * 0.5, ey + math.sin(aj) * L * 0.5 + rand(-2,2)}, {ex + math.cos(aj) * L, ey + math.sin(aj) * L}},
        {pressure={0.7, 0.05}, ramps={0.05, 0.7}, shake=0.6})
      count = count + 1
    end
  end
end
print(count, wait(0))

--@ chunk 104
print(wait(5*24*60))
for _, p in ipairs({{700,140},{860,200},{640,600},{300,600},{900,550}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 105
local extra = nil
local sprays = {
  {700,112,22,12},{800,104,18,10},{612,232,16,12},{580,318,14,10},{582,370,16,10},{665,180,16,10},
  {858,170,14,12},{902,262,14,10},{985,318,10,14},{920,250,16,10}
}
for i, s in ipairs(sprays) do
  local o = fol(s[1], s[2], s[3], s[4], 910+i, 1.5)
  extra = extra and (extra + o:mask()) or o:mask()
end
canopy = (folm + extra):roughen(8, 22, 921)
canopy = canopy * above({{0,412},{1000,412}}) + folm * below({{0,395},{1000,395}})
holesm = nil
local hl = {{722,232,8,6},{806,252,10,7},{668,262,7,5},{846,330,6,8},{958,300,7,6},{612,352,6,5},{780,300,5,4},{700,172,6,5},{905,350,5,6}}
for i, h in ipairs(hl) do
  local e = ellipse(h[1], h[2], h[3], h[4]):roughen(2.5, 6, 930+i)
  holesm = holesm and (holesm + e) or e
end
print(canopy:area(), folm:area(), holesm:area())

--@ chunk 106
-- sky holes first, on dry paint
local nz = noise{seed=311, octaves=4, period=180, persistence=0.5, stretch={0.05, 3}}
local function dist(x, y) return math.sqrt((x-330)^2 + ((y-380)*2.6)^2) + nz(x,y)*70 end
local hA = holesm * mask(function(x,y) return dist(x,y) < 540 and 1 or 0 end)
local hB = holesm - hA
work(hA, {hand="detail", tool="round 3", pile=ns3, angle=0, length={3,8}, coverage=3, pressure={0.7,0.95}, load=0.9, fill=true, seed=941})
work(hB, {hand="detail", tool="round 3", pile=ns2, angle=0, length={3,8}, coverage=3, pressure={0.7,0.95}, load=0.9, fill=true, seed=942})
-- canopy: dark core, warmer lit side, wet into wet, no blending
c_dk = pile{{"raw umber", 1}, {"Antwerp blue", 0.3}, {"bone black", 0.3}, {"raw sienna", 0.4}, {"lead white", 0.1}, medium=0.15}
c_md = pile{{"raw sienna", 1}, {"raw umber", 0.8}, {"Antwerp blue", 0.2}, {"yellow ochre", 0.3}, {"Indian red", 0.1}, {"lead white", 0.18}, medium=0.15}
local lit = oak_l2:blur(12)
local w = function(x,y) return clamp(lit:at(x,y)*0.8 + math.max(0, 680 - x)/250 * 0.4, 0, 1) end
local dz = canopy * mask(function(x,y) return w(x,y) < 0.5 and 1 or 0 end)
local mz = canopy - dz
local ang = function(x,y) return 0.9*math.sin(x/33 + y/27) end
local cl = canopy:grow(12) - holesm:grow(1)
work(dz, {hand="body", tool="filbert 7", pile=c_dk, angle=ang, angle_jitter=0.7, length={8,20}, coverage=2.0, pressure={0.55,0.9}, load=0.75, fill=true, clip=cl,
  edge={found=0.15, soft=0.45, lost=0.4, period=24, reach=1.2}, seed=943})
work(mz, {hand="body", tool="filbert 6", pile=c_md, angle=ang, angle_jitter=0.7, length={8,18}, coverage=1.8, pressure={0.5,0.85}, load=0.7, fill=true, clip=cl,
  edge={found=0.1, soft=0.5, lost=0.4, period=24, reach=1.2}, seed=944})
print(wait(0))

--@ chunk 107
local inner = canopy:shrink(9) - holesm:grow(4)
blend(inner, {angle=0.7, coverage=1.5, seed=951})
blend(inner, {angle=-0.6, coverage=1.0, seed=952})
print(wait(0))

--@ chunk 108
print(wait(6*24*60))
for _, p in ipairs({{700,140},{860,200},{640,300},{740,330},{940,300}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 109
gl_c = pile{{"raw umber", 1}, {"bone black", 0.25}, {"Antwerp blue", 0.2}, {"raw sienna", 0.3}, medium=0.85}
local cm = canopy:grow(4)
work(cm, {hand="glaze", tool={kind="filbert", width=12, stiffness=0.3}, pile=gl_c, angle=function(x,y) return 0.6*math.sin(x/50+y/40) end,
  length={30,70}, coverage=1.3, clip=cm, pressure={0.4,0.7},
  load_at=function(x,y) return clamp(0.35 + (x - 620) / 400 * 0.3 + (y - 200) / 400 * 0.2, 0.15, 0.6) end, seed=961})
print(wait(0))

--@ chunk 110
blend(canopy:grow(3), {angle=0.5, coverage=1.2, seed=971})
print(wait(0))

--@ chunk 111
print(wait(6*24*60))
for _, p in ipairs({{700,140},{860,200},{640,300},{300,600},{100,520}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 112
local n1 = noise{seed=991, octaves=3, period=90}
local n2 = noise{seed=992, octaves=3, period=60}
local fgr = mask(function(x,y) return y > fgtop(x) - 12 + n1(x, 0) * 5 and 1 or 0 end) - pondm:grow(3) - trunkm
g1 = pile{{"raw umber", 1}, {"raw sienna", 0.7}, {"yellow ochre", 0.5}, {"Antwerp blue", 0.12}, {"lead white", 0.15}, medium=0.15}
g2 = pile{{"raw umber", 1}, {"Indian red", 0.3}, {"raw sienna", 0.5}, {"bone black", 0.12}, {"lead white", 0.08}, medium=0.15}
g3 = pile{{"raw umber", 1}, {"bone black", 0.3}, {"Antwerp blue", 0.12}, {"raw sienna", 0.2}, medium=0.15}
local function depth(x,y) return (y - fgtop(x)) / (667 - fgtop(x)) end
local zA = fgr * mask(function(x,y) return (depth(x,y) + n1(x,y)*0.25 < 0.35) and 1 or 0 end)
local zC = fgr * mask(function(x,y) return (depth(x,y) + n1(x,y)*0.25 > 0.72) and 1 or 0 end)
local zB = fgr - zA - zC
local ang = function(x,y) return -1.25 + 0.35 * n2(x,y) end
local o = function(p, s, cov) return {hand="body", tool="filbert 8", pile=p, angle=ang, angle_jitter=0.3, length={14,34}, coverage=cov, pressure={0.55,0.9}, load=0.75, fill=true, edge="loose", seed=s} end
work(zA, o(g1, 993, 1.8))
work(zB, o(g2, 994, 1.8))
work(zC, o(g3, 995, 1.8))
print(wait(0))

--@ chunk 113
local fgr = mask(function(x,y) return y > fgtop(x) - 14 and 1 or 0 end) - pondm:grow(3) - trunkm
blend(fgr, {angle=-1.2, coverage=3, seed=1001})
blend(fgr, {angle=-0.3, coverage=2.5, seed=1002})
blend(fgr, {angle=0.9, coverage=2, seed=1003})
print(wait(0))

--@ chunk 114
print(wait(5*24*60))
for _, p in ipairs({{300,600},{100,560},{800,520},{700,640}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 115
gl_fg = pile{{"raw umber", 1}, {"bone black", 0.2}, {"raw sienna", 0.4}, {"Indian red", 0.1}, medium=0.8}
local fgr = mask(function(x,y) return y > fgtop(x) + 8 and 1 or 0 end) - pondm:grow(3) - trunkm
work(fgr, {hand="glaze", pile=gl_fg, angle=-0.05, angle_jitter=0.1, coverage=1.4, clip=fgr, pressure={0.4,0.75},
  load_at=function(x,y) local d = (y - fgtop(x)) / (667 - fgtop(x)); local corner = math.max(0, (x - 700) / 300, (200 - x) / 200) * 0.25; return clamp(0.12 + d * 0.6 + corner, 0.1, 0.85) end, seed=1011})
print(wait(0))

--@ chunk 116
gl_fg2 = pile{{"raw umber", 1}, {"bone black", 0.35}, {"raw sienna", 0.3}, {"Antwerp blue", 0.08}, medium=0.55}
local fgr = mask(function(x,y) return y > fgtop(x) + 20 and 1 or 0 end) - pondm:grow(3) - trunkm
work(fgr, {hand="glaze", pile=gl_fg2, angle=-0.05, angle_jitter=0.15, coverage=1.6, clip=fgr, pressure={0.5,0.8},
  load_at=function(x,y) local d = (y - fgtop(x)) / (667 - fgtop(x)); local corner = math.max(0, (x - 650) / 350, (250 - x) / 250) * 0.3; return clamp(d * 0.9 + corner - 0.05, 0.05, 0.95) end, seed=1021})
blend(mask(function(x,y) return y > fgtop(x) + 5 and 1 or 0 end) - pondm:grow(3) - trunkm, {angle=-0.1, coverage=2, seed=1022})
print(wait(0))

--@ chunk 117
-- open the underside of the big oak: a lobed opening between hanging masses
local cut = outline{{632,413},{640,392},{652,382},{668,378},{684,384},{700,376},{716,370},{734,374},{752,368},{772,372},{790,366},{806,374},{822,384},{836,380},{850,392},{858,413},
  closed=true, char="soft", seed=1031, amount=0.8, lobe=9}
local cut2 = outline{{898,422},{904,404},{916,396},{934,400},{950,394},{966,402},{978,412},{984,424},
  closed=true, char="soft", seed=1032, amount=0.8, lobe=7}
underm = (cut:mask() + cut2:mask())
local skp = underm * mask(function(x,y) return y < tl_top(x) and 1 or 0 end)
local farp = underm * mask(function(x,y) return (y >= tl_top(x) and y < 409) and 1 or 0 end)
local mdp = underm - skp - farp
work(skp, {hand="detail", tool="filbert 4", pile=ns2, angle=0, length={6,14}, coverage=3, pressure={0.7,0.95}, load=0.9, fill=true, seed=1033})
work(farp, {hand="detail", tool="filbert 4", pile=far_c, angle=-1.5, angle_jitter=0.3, length={4,10}, coverage=3, pressure={0.6,0.9}, load=0.85, fill=true, seed=1034})
work(mdp, {hand="detail", tool="filbert 5", pile=mh, angle=0, length={10,25}, coverage=3, pressure={0.6,0.9}, load=0.85, fill=true, seed=1035})
print(wait(0))

--@ chunk 118
local strip = mask(function(x,y) return (y > 402 and y < 440) and 1 or 0 end) * (underm:grow(6)) - trunkm
work(strip * mask(function(x,y) return y > 406 and 1 or 0 end), {hand="detail", tool="filbert 5", pile=mo_w, angle=0, length={10,25}, coverage=2.5, pressure={0.6,0.9}, load=0.8, fill=true, seed=1041})
blend(strip:grow(4), {angle=0, coverage=2.5, seed=1042})
-- lose the pale rim at the top of the opening into the treeline/sky
blend((underm:rim(5, 2)) * above({{0,402},{1000,402}}), {angle=-1.3, coverage=2, seed=1043})
print(wait(0))

--@ chunk 119
local strip = mask(function(x,y) return (y > 405 and y < 445) and 1 or 0 end) * (underm:grow(14)) - trunkm
work(strip, {hand="detail", tool="filbert 6", pile=mo, angle=0, length={10,25}, coverage=2.5, pressure={0.6,0.9}, load=0.85, fill=true, seed=1051})
blend(strip:grow(3):soften(4), {angle=0, coverage=2.5, seed=1052})
print(wait(0))

--@ chunk 120
print(wait(4*24*60))
for _, p in ipairs({{700,420},{930,420},{760,390},{900,410}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 121
print(wait(3*24*60))
for _, p in ipairs({{700,420},{930,420},{760,390},{900,410}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 122
-- cast shadow of the trees: a soft band across the meadow under and to the right of the oaks
shadowm = poly({{600,404},{660,402},{760,404},{860,408},{1000,412},{1000,478},{900,482},{820,480},{740,478},{680,472},{620,452},{590,430}}, true):roughen(6, 30, 1061)
gl_sh = pile{{"raw umber", 1}, {"raw sienna", 0.4}, {"bone black", 0.2}, {"Antwerp blue", 0.12}, medium=0.75}
work(shadowm, {hand="glaze", tool={kind="filbert", width=16, stiffness=0.3}, pile=gl_sh, angle=0.02, angle_jitter=0.05, length={40,120}, coverage=1.8, clip=shadowm:soften(6), pressure={0.5,0.8},
  load_at=function(x,y) local s = shadowm:at(x,y); return 0.15 + 0.45 * s end, seed=1062})
blend(shadowm:grow(6):soften(8), {angle=0, coverage=2, seed=1063})
print(wait(0))

--@ chunk 123
print(wait(3*24*60))
for _, p in ipairs({{700,440},{930,440},{760,420}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 124
trk3 = pile{{"raw umber", 1}, {"bone black", 0.5}, {"Antwerp blue", 0.08}, {"Indian red", 0.08}, {"lead white", 0.06}, medium=0.12}
trk_lit = pile{{"raw umber", 1}, {"Indian red", 0.3}, {"yellow ochre", 0.4}, {"lead white", 0.4}, medium=0.12}
-- main trunk: wider at base, splitting into two limbs that rise into the canopy
local b = brush{kind="filbert", width=12, stiffness=0.65}
b:load(trk3, 1.0)
b:stroke({{740,478},{741,455},{738,432},{734,410},{730,392},{722,372}}, {pressure={1.0,0.6}, ramps={0.02,0.3}, shake=0.3})
b:load(trk3, 1.0)
b:stroke({{746,477},{745,452},{744,430},{748,408},{756,388},{766,370}}, {pressure={1.0,0.55}, ramps={0.02,0.3}, shake=0.3})
local r = brush{kind="round", width=6, point=0.7, stiffness=0.6}
for _, l in ipairs({
  {{724,376},{710,360},{692,346},{676,340}},
  {{763,374},{780,360},{802,352},{822,350}},
  {{730,392},{726,370},{724,350}},
  {{756,390},{762,365},{768,350}},
}) do
  r:load(trk3, 0.85)
  r:stroke(l, {pressure={0.9,0.2}, ramps={0.03,0.6}, shake=0.5})
end
-- root flare
r:load(trk3, 0.9)
r:stroke({{728,481},{736,474},{742,466}}, {pressure={0.6,0.9}, ramps={0.2,0.1}})
r:load(trk3, 0.9)
r:stroke({{756,481},{748,474},{745,466}}, {pressure={0.6,0.9}, ramps={0.2,0.1}})
-- second tree
local b2 = brush{kind="filbert", width=7, stiffness=0.65}
b2:load(trk3, 1.0)
b2:stroke({{947,470},{948,448},{945,428},{947,410},{944,396}}, {pressure={1.0,0.6}, ramps={0.02,0.3}, shake=0.3})
r:load(trk3, 0.8)
r:stroke({{946,412},{935,398},{925,388}}, {pressure={0.8,0.2}, ramps={0.03,0.6}})
r:load(trk3, 0.8)
r:stroke({{948,410},{960,396},{972,388}}, {pressure={0.8,0.2}, ramps={0.03,0.6}})
-- a thin warm light along the left (glow-facing) edge of the main trunk
local rl = brush{kind="round", width=2.5, point=0.8}
rl:load(trk_lit, 0.5)
rl:stroke({{735,472},{735,450},{732,428},{729,408}}, {pressure={0.55,0.2}, ramps={0.1,0.5}, shake=0.4})
print(wait(0))

--@ chunk 125
local hang = nil
local hc = {{688,350,30,15},{814,354,34,16},{752,366,16,8},{640,392,18,10},{858,392,16,10},{776,376,12,7},{712,372,12,7},{930,392,18,9},{968,394,14,8}}
for i, h in ipairs(hc) do
  local o = fol(h[1], h[2], h[3], h[4], 1070+i, 1.4)
  hang = hang and (hang + o:mask()) or o:mask()
end
hangm = hang
local ang = function(x,y) return 0.9*math.sin(x/33 + y/27) end
work(hang, {hand="body", tool="filbert 6", pile=c_dk, angle=ang, angle_jitter=0.7, length={6,16}, coverage=2.2, pressure={0.55,0.9}, load=0.8, fill=true,
  edge={found=0.2, soft=0.45, lost=0.35, period=20}, seed=1081})
print(wait(0))

--@ chunk 126
sl_t2 = pile{{"raw umber", 1}, {"bone black", 0.35}, {"Indian red", 0.12}, {"lead white", 0.15}, medium=0.15}
local r = brush{kind="round", width=5, point=0.6, stiffness=0.6}
r:load(sl_t2, 0.95)
r:stroke({{161,486},{159,460},{157,430},{157,400},{158,375},{160,350}}, {pressure={0.95,0.55}, ramps={0.02,0.4}, shake=0.35})
r:load(sl_t2, 0.95)
r:stroke({{194,484},{193,460},{195,430},{197,405},{200,380}}, {pressure={0.85,0.5}, ramps={0.02,0.4}, shake=0.35})
-- bases: small dark grass tufts
local t = brush{kind="filbert", width=4, stiffness=0.6}
for _, p in ipairs({{150,488},{158,489},{166,488},{187,487},{194,488},{201,487}}) do
  t:load(md, 0.6)
  t:stroke({{p[1]-5, p[2]+1}, {p[1]+5, p[2]-1}}, {pressure={0.7,0.4}})
end
-- darker backlit foliage touches over the bright speckle
sl_f2 = pile{{"raw umber", 1}, {"raw sienna", 0.5}, {"Antwerp blue", 0.15}, {"bone black", 0.15}, {"lead white", 0.2}, {"yellow ochre", 0.15}, medium=0.2}
local inner = slfol:shrink(3)
stipple(inner, {pile=sl_f2, width=4, coverage=function(x,y) return 1.1 * inner:at(x,y) end, pressure={0.3,0.65}, dips={10, 0.5, 0.3}, cluster={0.6, 7}, feather=0.7, drag={2, -0.5}, seed=1091})
print(wait(0))

--@ chunk 127
cloud_g = pile{{"lead white", 3.6}, {"cobalt blue", 0.6}, {"Indian red", 0.42}, {"raw umber", 0.45}, medium=0.15}
cloud_r = pile{{"lead white", 5}, {"Indian red", 0.35}, {"orange chrome", 0.3}, {"yellow ochre", 0.4}, {"cobalt blue", 0.06}, medium=0.15}
local c1 = poly({{-20,40},{80,30},{200,48},{320,60},{430,70},{520,92},{560,112},{520,128},{440,132},{360,140},{280,150},{200,148},{120,160},{40,170},{-20,175}}, true):roughen(14, 60, 1101)
local c2 = poly({{360,190},{430,182},{520,188},{570,200},{540,212},{460,216},{390,212}}, true):roughen(8, 40, 1102)
local skymask = sky_all - canopy:grow(10)
cloudm = (c1 + c2) * skymask
work(cloudm, {hand="scumble", tool="filbert 14", pile=cloud_g, angle=function(x,y) return -0.08 + 0.05*math.sin(x/120) end, length={30,70}, coverage=1.3, pressure={0.4,0.7}, load=0.5, edge="lost", seed=1103})
-- lit undersides
local under1 = (c1 - c1:offset(-1):shrink(10)):blur(2)
local rim = (c1:grow(4) - c1:shrink(12)) * below({{0,90},{200,110},{400,100},{560,95}}) + (c2:grow(3) - c2:shrink(6)) * below({{0,200},{1000,200}})
work(rim * skymask, {hand="scumble", tool="filbert 8", pile=cloud_r, angle=-0.05, length={20,50}, coverage=1.2, pressure={0.4,0.7}, load=0.5, edge="lost", seed=1104})
print(wait(0))

--@ chunk 128
local skymask = sky_all - canopy:grow(10)
local z = mask(function(x,y) return y < 260 and 1 or 0 end) * skymask
blend(z, {angle=-0.05, coverage=3, seed=1111})
blend(z, {angle=0.15, coverage=2.5, seed=1112})
blend(z, {angle=-0.3, coverage=2, seed=1113})
print(wait(0))

--@ chunk 129
local skymask = sky_all - canopy:grow(10) - slfol:grow(6)
local seam = mask(function(x,y) return (y > 215 and y < 300) and 1 or 0 end):soften(10) * skymask
blend(seam, {angle=-1.45, coverage=3, seed=1121})
blend(seam, {angle=0.05, coverage=2, seed=1122})
print(wait(0))

--@ chunk 130
print(wait(5*24*60))
for _, p in ipairs({{300,100},{740,440},{688,350},{160,450},{150,300}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 131
local cover = {{700,172,9,7},{777,203,9,8},{722,232,10,8},{668,262,9,7},{780,300,7,6},{612,352,8,7},{958,300,9,7}}
local cm = nil
for i, h in ipairs(cover) do
  local e = ellipse(h[1], h[2], h[3]+3, h[4]+3):roughen(2, 6, 1130+i)
  cm = cm and (cm + e) or e
end
c_mx = pile{{"raw umber", 1}, {"Antwerp blue", 0.28}, {"bone black", 0.25}, {"raw sienna", 0.5}, {"yellow ochre", 0.12}, {"lead white", 0.12}, medium=0.15}
work(cm, {hand="detail", tool="filbert 4", pile=c_mx, angle=function(x,y) return 0.9*math.sin(x/33 + y/27) end, angle_jitter=0.7, length={4,10}, coverage=3, pressure={0.6,0.9}, load=0.85, fill=true, edge="soft", seed=1138})
-- remaining holes: tone them darker, toward the violet-grey sky behind
hole_t = pile{{"lead white", 2.5}, {"cobalt blue", 0.5}, {"Indian red", 0.35}, {"raw umber", 0.45}, {"yellow ochre", 0.1}, medium=0.2}
local keep = ellipse(806,252,11,8) + ellipse(846,330,7,8)
work(keep, {hand="detail", tool="round 3", pile=hole_t, angle=0, length={3,8}, coverage=2.5, pressure={0.6,0.9}, load=0.8, fill=true, seed=1139})
print(wait(0))

--@ chunk 132
print(wait(6*24*60))
for _, p in ipairs({{700,172},{806,252},{688,350},{777,203}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 133
local cover = {{700,172,9,7},{777,203,9,8},{722,232,10,8},{668,262,9,7},{780,300,7,6},{612,352,8,7},{958,300,9,7}}
local cm = nil
for i, h in ipairs(cover) do
  local e = ellipse(h[1], h[2], h[3]+8, h[4]+7):roughen(3, 8, 1140+i)
  cm = cm and (cm + e) or e
end
cm = cm:soften(4)
gl_c2 = pile{{"raw umber", 1}, {"bone black", 0.3}, {"Antwerp blue", 0.25}, {"raw sienna", 0.25}, medium=0.7}
for i = 1, 2 do
  work(cm, {hand="detail", tool="filbert 6", pile=gl_c2, angle=function(x,y) return 0.9*math.sin(x/33 + y/27) end, angle_jitter=0.8, length={6,14}, coverage=1.6, pressure={0.5,0.8}, load=0.6, seed=1150+i})
end
-- break the violet holes' outlines with a few dark leafy touches
local b = brush{kind="filbert", width=4, stiffness=0.5}
for _, h in ipairs({{806,252,11,8},{846,330,7,8}}) do
  for k = 1, 9 do
    local a = (k / 9) * 2 * math.pi + rand(-0.3, 0.3)
    local x0, y0 = h[1] + math.cos(a) * (h[3] + 3), h[2] + math.sin(a) * (h[4] + 3)
    local x1, y1 = h[1] + math.cos(a) * (h[3] * rand(0.2, 0.6)), h[2] + math.sin(a) * (h[4] * rand(0.2, 0.6))
    if k % 3 == 1 then b:load(c_dk, 0.6) end
    b:stroke({{x0, y0}, {x1, y1}}, {pressure={0.7, 0.1}, ramps={0.05, 0.6}})
  end
end
print(wait(0))

--@ chunk 134
local cover = {{700,172,9,7},{777,203,9,8},{722,232,10,8},{668,262,9,7},{780,300,7,6},{612,352,8,7},{958,300,9,7},{806,252,11,8},{846,330,7,8}}
local cm = nil
for i, h in ipairs(cover) do
  local e = ellipse(h[1], h[2], h[3]+22, h[4]+20)
  cm = cm and (cm + e) or e
end
cm = cm:soften(8)
blend(cm, {angle=0.6, coverage=3, seed=1161})
blend(cm, {angle=-0.7, coverage=3, seed=1162})
blend(cm, {angle=1.5, coverage=2, seed=1163})
print(wait(0))

--@ chunk 135
local hm = ellipse(806,252,13,10) + ellipse(846,330,9,10)
work(hm, {hand="detail", tool="filbert 5", pile=c_dk, angle=0.5, angle_jitter=0.8, length={5,10}, coverage=2.5, pressure={0.6,0.9}, load=0.7, fill=true, seed=1171})
local bm = (ellipse(806,252,26,22) + ellipse(846,330,22,22)):soften(6)
blend(bm, {angle=0.6, coverage=3, seed=1172})
blend(bm, {angle=-0.8, coverage=2, seed=1173})
print(wait(0))

--@ chunk 136
for _, p in ipairs({{60,385},{490,380},{300,500},{160,470}}) do print(p[1], p[2], drying(p[1], p[2])) end
far_d = pile{{"lead white", 1.0}, {"cobalt blue", 0.7}, {"Indian red", 0.45}, {"raw umber", 0.75}, {"yellow ochre", 0.15}, medium=0.2}

--@ chunk 137
local dm2 = (distm:grow(2)):roughen(5, 12, 1181) * above({{0,408},{1000,408}})
work(dm2, {hand="detail", tool="filbert 4", pile=far_d, angle=-1.35, angle_jitter=0.6, length={4,10}, coverage=2.2, pressure={0.55,0.85}, load=0.75, fill=true,
  edge={found=0.1, soft=0.5, lost=0.4, period=16}, seed=1182})
-- feathery tops: a few short upward touches along the crowns
local b = brush{kind="round", width=2.5, point=0.8}
local n = 0
for x = 2, 540, 5 do
  local y = nil
  for yy = 340, 405 do if distm:at(x, yy) > 0.5 then y = yy; break end end
  if y and math.random() < 0.6 then
    if n % 8 == 0 then b:load(far_d, 0.5) end
    b:stroke({{x, y + 4}, {x + rand(-2, 2), y - rand(3, 8)}}, {pressure={0.6, 0.05}, ramps={0.05, 0.7}})
    n = n + 1
  end
end
print(n, wait(0))

--@ chunk 138
gr_d = pile{{"raw umber", 1}, {"raw sienna", 0.5}, {"Antwerp blue", 0.12}, {"bone black", 0.12}, {"yellow ochre", 0.2}, {"lead white", 0.1}, medium=0.15}
gr_m = pile{{"raw sienna", 1}, {"raw umber", 0.6}, {"yellow ochre", 0.6}, {"Antwerp blue", 0.08}, {"lead white", 0.3}, medium=0.15}
gr_l = pile{{"yellow ochre", 1}, {"raw sienna", 0.3}, {"lead white", 0.7}, {"raw umber", 0.2}, {"orange chrome", 0.08}, medium=0.15}
function tufts(pts, pile_, opts)
  local b = brush{kind="round", width=opts.w or 2.5, point=0.8, stiffness=0.6}
  local n = 0
  for _, p in ipairs(pts) do
    if n % (opts.per or 6) == 0 then b:load(pile_, opts.load or 0.6) end
    local h = rand(opts.hmin or 6, opts.hmax or 14)
    local lean = rand(-0.35, 0.35) + (opts.lean or 0)
    b:stroke({{p[1], p[2]}, {p[1] + math.sin(lean) * h * 0.5, p[2] - h * 0.5}, {p[1] + math.sin(lean) * h * 1.1, p[2] - h}}, {pressure={opts.pr or 0.7, 0.03}, ramps={0.05, 0.75}, shake=0.4})
    n = n + 1
  end
  return n
end
-- along the near bank of the pond and around the trunk bases
local pts = {}
for i = 1, 150 do
  local x = rand(140, 500)
  local y = 512 + (x - 150) * 0.02 + rand(-4, 10)
  pts[#pts+1] = {x, y}
end
for i = 1, 40 do pts[#pts+1] = {rand(138, 215), rand(484, 496)} end
print(tufts(pts, gr_d, {w=3, hmin=6, hmax=16, per=6, load=0.65}))
local pts2 = {}
for i = 1, 60 do local x = rand(140, 500); pts2[#pts2+1] = {x, 510 + (x - 150) * 0.02 + rand(-2, 8)} end
print(tufts(pts2, gr_m, {w=2.2, hmin=5, hmax=11, per=6, load=0.55, pr=0.6}))
print(wait(0))

--@ chunk 139
local strip = mask(function(x,y) return (x > 130 and x < 510 and y > 478 and y < 530) and 1 or 0 end):soften(5) - pondm:grow(1)
blend(strip, {angle=-1.2, coverage=2, seed=1191})
blend(strip, {angle=0.1, coverage=1.5, seed=1192})
print(wait(0))

--@ chunk 140
fig_red = pile{{"red earth", 1}, {"orange chrome", 0.45}, {"Indian red", 0.3}, {"lead white", 0.08}, medium=0.08}
fig_dk = pile{{"raw umber", 1}, {"bone black", 0.4}, {"Indian red", 0.15}, medium=0.1}
fig_sk = pile{{"yellow ochre", 1}, {"Indian red", 0.3}, {"lead white", 0.8}, {"raw umber", 0.2}, medium=0.1}
local fx, fy = 612, 482
-- skirt
local b = brush{kind="filbert", width=3.2, stiffness=0.6}
b:load(fig_dk, 0.8)
b:stroke({{fx - 1.5, fy}, {fx - 0.8, fy - 10}}, {pressure={0.9, 0.7}})
b:load(fig_dk, 0.8)
b:stroke({{fx + 1.8, fy}, {fx + 0.8, fy - 10}}, {pressure={0.9, 0.7}})
-- shawl/torso
local r = brush{kind="filbert", width=3.4, stiffness=0.6}
r:load(fig_red, 0.9)
r:stroke({{fx - 0.5, fy - 9}, {fx + 0.3, fy - 15}}, {pressure={0.95, 0.8}})
r:load(fig_red, 0.9)
r:stroke({{fx - 2.2, fy - 11}, {fx + 2.4, fy - 13}}, {pressure={0.8, 0.6}})
-- head
local h = brush{kind="round", width=2.6, point=0.3}
h:load(fig_sk, 0.8)
h:touch(fx + 0.4, fy - 17.5, {pressure=0.75})
print(wait(0))

--@ chunk 141
local fx, fy = 612, 482
local h = brush{kind="round", width=2.4, point=0.4}
h:load(fig_dk, 0.8)
h:touch(fx + 0.4, fy - 18, {pressure=0.7})
-- narrow the shoulders: dark touches at the side of the red, then a thin shadow down the right side
local r = brush{kind="round", width=1.6, point=0.6}
r:load(fig_dk, 0.7)
r:stroke({{fx + 2.6, fy - 14}, {fx + 2.2, fy - 8}}, {pressure={0.6, 0.4}})
print(wait(0))

--@ chunk 142
print(wait(6*24*60))
for _, p in ipairs({{612,470},{60,385},{300,515},{300,300},{740,250}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 143
gl_warm = pile{{"orange chrome", 0.5}, {"red earth", 0.35}, {"raw sienna", 0.5}, medium=0.9}
local nz = noise{seed=311, octaves=4, period=180, persistence=0.5, stretch={0.05, 3}}
local function dist(x, y) return math.sqrt((x-330)^2 + ((y-380)*2.6)^2) + nz(x,y)*70 end
local skyz = sky_all - canopy:grow(6) - slfol:grow(4) - distm:grow(2)
local ring = skyz * mask(function(x,y) local d = dist(x,y); return (d > 130 and d < 620) and 1 or 0 end)
work(ring, {hand="glaze", pile=gl_warm, angle=0.0, angle_jitter=0.05, coverage=1.2, clip=skyz, pressure={0.35,0.6},
  load_at=function(x,y) local d = dist(x,y); local t = 1 - math.abs(d - 360) / 260; return clamp(0.05 + 0.3 * t, 0.03, 0.35) end, seed=1201})
blend(ring:grow(10) * skyz, {angle=0, coverage=2, seed=1202})
print(wait(0))

--@ chunk 144
local skyz = sky_all - canopy:grow(6) - slfol:grow(4) - distm:grow(2)
local z = skyz * mask(function(x,y) return (y > 100 and y < 395) and 1 or 0 end)
blend(z, {angle=0.03, coverage=3, seed=1211})
blend(z, {angle=-0.25, coverage=2, seed=1212})
print(drying(300,200), wait(0))

--@ chunk 145
gl_o = pile{{"raw umber", 1}, {"bone black", 0.2}, {"Antwerp blue", 0.3}, {"raw sienna", 0.2}, medium=0.85}
local cm = canopy:grow(3) + hangm
work(cm, {hand="glaze", tool={kind="filbert", width=12, stiffness=0.3}, pile=gl_o, angle=function(x,y) return 0.6*math.sin(x/50+y/40) end,
  length={30,70}, coverage=1.4, clip=cm, pressure={0.4,0.7},
  load_at=function(x,y) return clamp(0.25 + 0.35 * oak_l2:at(x,y) + (x > 680 and 0.1 or 0), 0.2, 0.6) end, seed=1221})
blend(canopy:shrink(6), {angle=0.5, coverage=1.2, seed=1222})
-- slender trees: thin dark glaze to backlight them
work(slfol:grow(2), {hand="glaze", tool={kind="filbert", width=8, stiffness=0.3}, pile=gl_o, angle=0.3, length={15,40}, coverage=1.2, clip=slfol:grow(3), pressure={0.4,0.7}, load=0.3, seed=1223})
print(wait(0))

--@ chunk 146
fr1 = pile{{"raw sienna", 1}, {"Indian red", 0.35}, {"orange chrome", 0.2}, {"raw umber", 0.3}, {"lead white", 0.15}, medium=0.12}
fr2 = pile{{"yellow ochre", 1}, {"raw sienna", 0.5}, {"lead white", 0.35}, {"raw umber", 0.3}, {"Indian red", 0.08}, medium=0.12}
-- dry-brush autumn grasses: broken low-load strokes over the dry foreground, mostly in the middle band
local n1 = noise{seed=1231, octaves=3, period=120}
local band = mask(function(x,y) local t = fgtop(x); return (y > t + 15 and y < 640) and 1 or 0 end) - pondm:grow(4)
local warmz = band * mask(function(x,y) return (n1(x,y) > 0.05) and 1 or 0 end)
work(warmz, {hand="scumble", tool={kind="filbert", width=10, stiffness=0.8}, pile=fr1, angle=function(x,y) return -0.08 + 0.15*n1(x*2,y*2) end, length={25,60}, coverage=0.7,
  pressure={0.25,0.45}, load=0.18, edge="lost", seed=1232})
local lz = band * mask(function(x,y) local t = fgtop(x); return (n1(x+300,y) > 0.2 and y < t + 80) and 1 or 0 end)
work(lz, {hand="scumble", tool={kind="filbert", width=8, stiffness=0.8}, pile=fr2, angle=-0.05, length={25,50}, coverage=0.6,
  pressure={0.22,0.4}, load=0.16, edge="lost", seed=1233})
print(wait(0))

--@ chunk 147
local band = mask(function(x,y) local t = fgtop(x); return (y > t + 5) and 1 or 0 end) - pondm:grow(4)
for i = 1, 4 do
  blend(band, {angle=(i % 2 == 0) and -0.1 or 0.6, coverage=3, seed=1240+i})
end
print(wait(0))

--@ chunk 148
print(wait(4*24*60))
for _, p in ipairs({{300,600},{800,560},{700,250},{300,250}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 149
gl_f3 = pile{{"raw umber", 1}, {"raw sienna", 0.5}, {"bone black", 0.15}, {"Antwerp blue", 0.06}, medium=0.75}
local n1 = noise{seed=1251, octaves=3, period=160}
local fz = mask(function(x,y) return (y > fgtop(x) - 25) and 1 or 0 end) - pondm:grow(3) - trunkm:grow(2)
work(fz, {hand="glaze", pile=gl_f3, angle=-0.04, angle_jitter=0.08, coverage=1.6, clip=fz, pressure={0.45,0.75},
  load_at=function(x,y)
    local d = clamp((y - fgtop(x) + 25) / (667 - fgtop(x) + 25), 0, 1)
    local corner = math.max(0, (x - 620) / 380, (260 - x) / 260) * 0.25
    return clamp(0.15 + d * 0.55 + corner + n1(x,y) * 0.12, 0.08, 0.9) end, seed=1252})
blend(fz, {angle=-0.05, coverage=2.5, seed=1253})
blend(fz, {angle=0.5, coverage=1.5, seed=1254})
print(wait(0))

--@ chunk 150
print(wait(3*24*60))
for _, p in ipairs({{300,600},{800,560}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 151
local n1 = noise{seed=1261, octaves=3, period=150, stretch={0, 3}}
local fz = mask(function(x,y) return (y > fgtop(x) + 30) and 1 or 0 end) - pondm:grow(3)
work(fz, {hand="glaze", pile=gl_fg2, angle=-0.04, angle_jitter=0.1, coverage=1.8, clip=fz:soften(15), pressure={0.5,0.8},
  load_at=function(x,y)
    local d = clamp((y - fgtop(x) - 30) / (667 - fgtop(x) - 30), 0, 1)
    local corner = math.max(0, (x - 650) / 350, (220 - x) / 220) * 0.3
    return clamp(0.05 + d * 0.85 + corner + n1(x,y) * 0.15, 0.03, 1.0) end, seed=1262})
print(wait(0))

--@ chunk 152
local fz = mask(function(x,y) return (y > fgtop(x) + 20) and 1 or 0 end) - pondm:grow(3)
blend(fz, {angle=-0.05, coverage=2.5, seed=1271})
blend(fz, {angle=0.35, coverage=2, seed=1272})
print(wait(0))

--@ chunk 153
local cb = (canopy + hangm):blur(6)
local edge = ((canopy + hangm):grow(2) - (canopy + hangm):shrink(3)) * above({{0,385},{1000,392}})
local b = brush{kind="round", width=3.2, point=0.8, stiffness=0.55}
local n, tries = 0, 0
local en = noise{seed=1281, octaves=2, period=50}
while n < 260 and tries < 20000 do
  tries = tries + 1
  local x, y = rand(550, 1000), rand(90, 395)
  if edge:at(x, y) > 0.5 and en(x, y) > -0.3 then
    local gx = cb:at(x + 3, y) - cb:at(x - 3, y)
    local gy = cb:at(x, y + 3) - cb:at(x, y - 3)
    local gl = math.sqrt(gx*gx + gy*gy)
    if gl > 0.05 then
      local ox, oy = -gx / gl, -gy / gl
      local a = math.atan(oy, ox) + rand(-0.7, 0.7)
      local L = rand(6, 16)
      if n % 7 == 0 then b:load(c_dk, 0.7) end
      b:stroke({{x - math.cos(a) * 3, y - math.sin(a) * 3}, {x + math.cos(a) * L * 0.5 + rand(-1.5,1.5), y + math.sin(a) * L * 0.5 + rand(-1.5,1.5)}, {x + math.cos(a) * L, y + math.sin(a) * L}},
        {pressure={0.75, 0.02}, ramps={0.05, 0.75}, shake=0.7})
      n = n + 1
    end
  end
end
print(n, tries, wait(0))

--@ chunk 154
local band = ((canopy + hangm):grow(14) - (canopy + hangm):shrink(6)) * above({{0,388},{1000,394}})
blend(band, {angle=0.4, coverage=2, seed=1291})
blend(band, {angle=-0.9, coverage=1.5, seed=1292})
print(wait(0))

--@ chunk 155
print(wait(5*24*60))
for _, p in ipairs({{300,600},{800,560},{700,100},{560,350},{160,500}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 156
refl = pile{{"raw umber", 1}, {"raw sienna", 0.4}, {"lead white", 0.5}, {"yellow ochre", 0.3}, {"bone black", 0.1}, medium=0.2}
local r = brush{kind="round", width=3.2, point=0.5, stiffness=0.5}
r:load(refl, 0.6)
r:stroke({{193,488},{194,494},{192,499},{194,505}}, {pressure={0.8,0.2}, ramps={0.05,0.6}, shake=0.6})
-- far-bank reflection: a soft darker band just under the top edge of the water, broken
local rb = brush{kind="filbert", width=4, stiffness=0.5}
for _, s in ipairs({{{180,492},{215,491}}, {{240,490},{290,489}}, {{330,488},{372,489}}, {{395,490},{430,492}}}) do
  rb:load(refl, 0.35)
  rb:stroke(s, {pressure={0.4,0.2}, ramps={0.2,0.5}, shake=0.5})
end
blend(pondm:shrink(1), {angle=0, coverage=1.5, seed=1301})
print(wait(0))

--@ chunk 157
local bush = outline{{-10,540},{20,528},{48,522},{70,530},{92,526},{112,540},{130,556},{150,575},{175,600},{205,630},{230,667},{-10,667},
  closed=true, char="soft", seed=1311, amount=1.2, lobe=10}
bushm = bush:mask()
bsh1 = pile{{"raw umber", 1}, {"bone black", 0.25}, {"Antwerp blue", 0.12}, {"raw sienna", 0.4}, {"lead white", 0.05}, medium=0.15}
bsh2 = pile{{"raw sienna", 1}, {"Indian red", 0.4}, {"raw umber", 0.6}, {"orange chrome", 0.15}, {"lead white", 0.15}, medium=0.15}
work(bushm, {hand="body", tool="filbert 9", pile=bsh1, angle=function(x,y) return -1.0 + 0.6*math.sin(x/40 + y/30) end, angle_jitter=0.6, length={10,26}, coverage=2.0, pressure={0.6,0.9}, load=0.8, fill=true,
  edge={found=0.15, soft=0.45, lost=0.4, period=20}, seed=1312})
-- russet lit leaves on the upper edge facing the glow
local top = bushm * above({{0,560},{100,560},{160,600},{220,650}}) 
work(top, {hand="scumble", tool="filbert 5", pile=bsh2, angle=-0.8, length={6,14}, coverage=0.9, pressure={0.4,0.7}, load=0.5, seed=1313})
print(wait(0))

--@ chunk 158
local z = bushm:grow(10)
blend(z, {angle=-0.9, coverage=3, seed=1321})
blend(z, {angle=0.4, coverage=2.5, seed=1322})
print(wait(0))

--@ chunk 159
print(wait(4*24*60))
for _, p in ipairs({{60,600},{100,540},{180,600}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 160
print(wait(4*24*60))
for _, p in ipairs({{60,600},{100,540},{180,600}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 161
local pts, lit = {}, {}
local edgeline = {{-10,540},{20,528},{48,522},{70,530},{92,526},{112,540},{130,556},{150,575},{175,600},{205,630},{230,667}}
for i = 1, #edgeline - 1 do
  local a, b = edgeline[i], edgeline[i+1]
  local seg = math.sqrt((b[1]-a[1])^2 + (b[2]-a[2])^2)
  local n = math.floor(seg / 2.2)
  for k = 1, n do
    local t = rand(0, 1)
    local x, y = lerp(a[1], b[1], t), lerp(a[2], b[2], t) + rand(-2, 10)
    pts[#pts+1] = {x, y}
    if math.random() < 0.25 then lit[#lit+1] = {x + rand(-3,3), y + rand(-2, 4)} end
  end
end
print(tufts(pts, bsh1, {w=3.2, hmin=8, hmax=22, per=5, load=0.7, lean=0.15}))
print(tufts(lit, fr1, {w=2.2, hmin=6, hmax=14, per=6, load=0.45, pr=0.55, lean=0.2}))
print(wait(0))

--@ chunk 162
local edgeband = ribbon({{-10,535},{20,523},{48,517},{70,525},{92,521},{112,535},{130,551},{150,570},{175,595},{205,625},{230,662}}, 34)
blend(edgeband, {angle=-1.35, coverage=2.5, seed=1331})
blend(edgeband, {angle=-0.6, coverage=1.5, seed=1332})
print(wait(0))

--@ chunk 163
local soft = ribbon({{-10,535},{20,523},{48,517},{70,525},{92,521},{112,535},{130,551},{150,570},{175,595},{205,625},{230,662}}, 50):blur(14)
blend(soft, {angle=-0.7, coverage=3, seed=1341})
blend(soft, {angle=0.5, coverage=2, seed=1342})
print(wait(0))

--@ chunk 164
for _, p in ipairs({{660,120},{880,180},{570,330},{990,260},{700,95}}) do print(p[1], p[2], drying(p[1], p[2])) end
hs2 = pile{{"lead white", 4.5}, {"cobalt blue", 0.38}, {"Indian red", 0.38}, {"raw umber", 0.3}, {"yellow ochre", 0.18}, {"orange chrome", 0.04}, medium=0.12}
hs3 = pile{{"lead white", 5}, {"Indian red", 0.3}, {"orange chrome", 0.3}, {"yellow ochre", 0.4}, {"cobalt blue", 0.08}, medium=0.12}
hs4 = pile{{"lead white", 5}, {"yellow ochre", 0.6}, {"lemon chrome", 0.4}, {"orange chrome", 0.15}, medium=0.12}
hs1 = pile{{"lead white", 3.6}, {"cobalt blue", 0.62}, {"Indian red", 0.42}, {"raw umber", 0.45}, {"yellow ochre", 0.05}, medium=0.12}

--@ chunk 165
local tree = canopy + hangm
local H = (tree:grow(20) - tree:grow(1)) * mask(function(x,y) return y < tl_top(x) - 2 and 1 or 0 end)
local nz = noise{seed=311, octaves=4, period=180, persistence=0.5, stretch={0.05, 3}}
local function dist(x, y) return math.sqrt((x-330)^2 + ((y-380)*2.6)^2) + nz(x,y)*70 end
local function bandm(d0, d1) return H * mask(function(x,y) local d = dist(x,y); return (d >= d0 and d < d1) and 1 or 0 end) end
local function lay(m, p, s)
  work(m, {hand="detail", tool="filbert 6", pile=p, angle=function(x,y) return 0.1*math.sin(x/40) end, angle_jitter=0.3, length={8,20}, coverage=2.6, pressure={0.7,0.95}, load=0.9, fill=true, clip=H, seed=s})
end
lay(bandm(760, 99999), hs1, 1351)
lay(bandm(540, 760), hs2, 1352)
lay(bandm(330, 540), hs3, 1353)
lay(bandm(0, 330), hs4, 1354)
print(wait(0))

--@ chunk 166
local tree = canopy + hangm
local outer = ((tree:grow(30) - tree:grow(10)):blur(6)) * mask(function(x,y) return y < tl_top(x) - 2 and 1 or 0 end)
blend(outer, {angle=0.05, coverage=3, seed=1361})
blend(outer, {angle=0.8, coverage=2, seed=1362})
print(wait(0))

--@ chunk 167
print(wait(4*24*60))
for _, p in ipairs({{600,250},{900,260},{60,385},{490,380}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 168
haze = pile{{"lead white", 4}, {"yellow ochre", 0.6}, {"Indian red", 0.22}, {"orange chrome", 0.08}, {"cobalt blue", 0.12}, {"raw umber", 0.12}, medium=0.3}
local dz = distm:grow(6) * above({{0,410},{1000,410}})
work(dz, {hand="scumble", tool={kind="filbert", width=8, stiffness=0.8}, pile=haze, angle=-0.1, length={10,24}, coverage=1.0, pressure={0.25,0.45}, load=0.22, clip=dz:soften(3),
  edge="lost", seed=1371})
-- also veil the whole far treeline band slightly near the glow
local glowband = mask(function(x,y) return (y > 370 and y < 412 and x > 180 and x < 470) and 1 or 0 end):soften(15)
work(glowband, {hand="scumble", tool={kind="filbert", width=8, stiffness=0.8}, pile=hs4, angle=0, length={14,30}, coverage=0.6, pressure={0.2,0.4}, load=0.18, edge="lost", seed=1372})
print(wait(0))

--@ chunk 169
print(drying(100,380), drying(300,395))
-- restate the distant groves into the wet veil, then fuse
work(distm:shrink(2), {hand="detail", tool="filbert 5", pile=far_m, angle=-1.3, angle_jitter=0.6, length={5,12}, coverage=2.2, pressure={0.6,0.9}, load=0.85, fill=true, seed=1381})
blend(distm:grow(4), {angle=-0.2, coverage=2, seed=1382})
local glowband = mask(function(x,y) return (y > 368 and y < 414 and x > 170 and x < 480) and 1 or 0 end):soften(10) - distm:grow(3) - slfol
blend(glowband, {angle=0, coverage=3, seed=1383})
print(wait(0))

--@ chunk 170
print(wait(5*24*60))
for _, p in ipairs({{100,380},{300,395},{490,375}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 171
grL = outline{{-5,408,"c"},{-5,372},{12,366},{26,358},{38,362},{50,350},{66,346},{80,352},{92,344},{106,350},{118,358},{132,354},{146,362},{160,370},{176,378},{196,386},{214,392},{232,398},{240,408,"c"},
  closed=true, char="soft", seed=1391, amount=1.3, lobe=7}
grR = outline{{432,408,"c"},{436,388},{448,378},{460,370},{474,372},{486,362},{500,360},{514,366},{526,372},{540,382},{556,390},{572,396},{580,408,"c"},
  closed=true, char="soft", seed=1392, amount=1.3, lobe=6}
grLm, grRm = grL:mask(), grR:mask()
grove_c = pile{{"lead white", 1.5}, {"cobalt blue", 0.6}, {"Indian red", 0.45}, {"raw umber", 0.6}, {"yellow ochre", 0.15}, medium=0.15}
grove_w = pile{{"lead white", 2.2}, {"cobalt blue", 0.45}, {"Indian red", 0.45}, {"raw umber", 0.45}, {"yellow ochre", 0.3}, medium=0.15}
local ang = function(x,y) return -1.5 + 0.4*math.sin(x/9) end
local cl = (grLm + grRm):grow(5) - slfol
work(grLm, {hand="detail", tool="filbert 4", pile=grove_c, angle=ang, angle_jitter=0.5, length={5,12}, coverage=2.4, pressure={0.6,0.9}, load=0.8, fill=true, clip=cl,
  edge={found=0.2, soft=0.5, lost=0.3, period=14}, seed=1393})
work(grRm, {hand="detail", tool="filbert 4", pile=grove_w, angle=ang, angle_jitter=0.5, length={5,12}, coverage=2.4, pressure={0.6,0.9}, load=0.8, fill=true, clip=cl,
  edge={found=0.15, soft=0.5, lost=0.35, period=14}, seed=1394})
print(wait(0))

--@ chunk 172
local both = grLm + grRm
local tops = both - both:offset(0):shrink(9)
local base = both * below({{0,396},{1000,396}})
local n = noise{seed=1401, octaves=2, period=40}
work(base * mask(function(x,y) return n(x,y) > -0.3 and 1 or 0 end), {hand="detail", tool="filbert 4", pile=far_d, angle=-1.5, angle_jitter=0.5, length={4,10}, coverage=1.2, pressure={0.5,0.8}, load=0.6, clip=both, seed=1402})
work(tops * mask(function(x,y) return n(x+99,y) > -0.2 and 1 or 0 end), {hand="detail", tool="filbert 3", pile=far_w, angle=-1.5, angle_jitter=0.6, length={3,8}, coverage=1.0, pressure={0.4,0.7}, load=0.5, clip=both, seed=1403})
blend(both:shrink(3), {angle=-1.4, coverage=1.5, seed=1404})
print(wait(0))

--@ chunk 173
print(wait(5*24*60))
for _, p in ipairs({{100,380},{500,380}}) do print(p[1], p[2], drying(p[1], p[2])) end
gl_grove = pile{{"raw umber", 1}, {"Antwerp blue", 0.25}, {"yellow ochre", 0.3}, {"bone black", 0.08}, medium=0.85}

--@ chunk 174
local both = (grLm + grRm)
work(both, {hand="glaze", tool={kind="filbert", width=10, stiffness=0.3}, pile=gl_grove, angle=-0.05, length={30,80}, coverage=1.4, clip=both:soften(2), pressure={0.4,0.7},
  load_at=function(x,y) return clamp(0.3 + (y - 360) / 50 * 0.25 - math.max(0, x - 400) / 400 * 0.1, 0.15, 0.55) end, seed=1411})
print(wait(0))

--@ chunk 175
local both = (grLm + grRm):grow(3)
blend(both, {angle=-1.4, coverage=3, seed=1421})
blend(both, {angle=-0.2, coverage=2.5, seed=1422})
blend(both, {angle=0.9, coverage=2, seed=1423})
print(wait(0))

--@ chunk 176
print(wait(5*24*60))
for _, p in ipairs({{100,380},{500,380},{160,300},{300,385}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 177
gl_g2 = pile{{"raw umber", 1}, {"yellow ochre", 0.35}, {"Antwerp blue", 0.15}, {"Indian red", 0.1}, medium=0.7}
local both = grLm + grRm
-- darker toward the base, lighter at the crowns near the glow
work(both, {hand="glaze", tool={kind="filbert", width=8, stiffness=0.3}, pile=gl_g2, angle=-1.4, angle_jitter=0.4, length={10,30}, coverage=1.6, clip=both:soften(1.5), pressure={0.4,0.7},
  load_at=function(x,y) return clamp(0.15 + (y - 350) / 55 * 0.45 - math.max(0, 400 - math.abs(x - 330)) / 400 * 0.1, 0.1, 0.6) end, seed=1431})
-- slender trees' foliage: darken and unify
work(slfol:grow(1), {hand="glaze", tool={kind="filbert", width=7, stiffness=0.3}, pile=gl_t2, angle=0.4, angle_jitter=0.6, length={10,25}, coverage=1.5, clip=slfol:grow(2), pressure={0.4,0.7}, load=0.45, seed=1432})
print(wait(0))

--@ chunk 178
local both = (grLm + grRm):grow(2)
blend(both, {angle=-1.4, coverage=3, seed=1441})
blend(both, {angle=0.1, coverage=3, seed=1442})
blend(both, {angle=-0.7, coverage=2, seed=1443})
blend(slfol:grow(2), {angle=0.5, coverage=2, seed=1444})
print(wait(0))

--@ chunk 179
print(wait(5*24*60))
for _, p in ipairs({{100,380},{500,380},{160,300},{200,330}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 180
-- restate slender trunks through the grove
local r = brush{kind="round", width=4.5, point=0.6, stiffness=0.6}
r:load(sl_t2, 0.9)
r:stroke({{158,432},{157,410},{157,390},{158,372},{160,355}}, {pressure={0.8,0.55}, ramps={0.05,0.3}, shake=0.3})
r:load(sl_t2, 0.9)
r:stroke({{194,432},{195,410},{197,390},{199,372},{201,356}}, {pressure={0.75,0.5}, ramps={0.05,0.3}, shake=0.3})
-- grove rims: a thin warm-grey scumble over the chalky edges, low load
local rimz = ((grLm + grRm):grow(4) - (grLm + grRm):shrink(5))
grove_r = pile{{"lead white", 1.6}, {"raw umber", 0.6}, {"cobalt blue", 0.3}, {"Indian red", 0.35}, {"yellow ochre", 0.35}, medium=0.3}
work(rimz, {hand="scumble", tool="filbert 4", pile=grove_r, angle=-1.4, angle_jitter=0.5, length={5,10}, coverage=1.2, pressure={0.35,0.6}, load=0.35, clip=rimz, seed=1451})
print(wait(0))

--@ chunk 181
grove_m = pile{{"lead white", 1.3}, {"cobalt blue", 0.45}, {"Indian red", 0.38}, {"raw umber", 0.75}, {"yellow ochre", 0.35}, medium=0.15}
local both = grLm + grRm
local ang = function(x,y) return -1.5 + 0.4*math.sin(x/9) end
work(both, {hand="detail", tool="filbert 5", pile=grove_m, angle=ang, angle_jitter=0.5, length={5,12}, coverage=2.6, pressure={0.65,0.95}, load=0.85, fill=true, clip=both:grow(1) - slfol,
  seed=1461})
print(wait(0))

--@ chunk 182
grove_dk = pile{{"lead white", 0.7}, {"cobalt blue", 0.5}, {"Indian red", 0.4}, {"raw umber", 1.0}, {"yellow ochre", 0.3}, medium=0.15}
local both = grLm + grRm
local n = noise{seed=1471, octaves=3, period=35}
local low = both * mask(function(x,y) return (y > 384 + n(x,0)*8) and 1 or 0 end)
local inner = both:shrink(6) * mask(function(x,y) return n(x,y) > 0.25 and 1 or 0 end)
work(low + inner, {hand="detail", tool="filbert 4", pile=grove_dk, angle=-1.45, angle_jitter=0.5, length={4,10}, coverage=1.6, pressure={0.55,0.85}, load=0.7, clip=both:grow(1), seed=1472})
blend(both:grow(2), {angle=-1.4, coverage=2, seed=1473})
print(wait(0))

--@ chunk 183
print(wait(5*24*60))
for _, p in ipairs({{100,380},{500,380}}) do print(p[1], p[2], drying(p[1], p[2])) end
gv1 = pile{{"lead white", 1.0}, {"cobalt blue", 0.45}, {"raw umber", 0.9}, {"yellow ochre", 0.45}, {"Indian red", 0.2}, {"Antwerp blue", 0.05}, medium=0.15}
gv2 = pile{{"lead white", 0.6}, {"cobalt blue", 0.4}, {"raw umber", 1.0}, {"yellow ochre", 0.35}, {"Indian red", 0.2}, {"Antwerp blue", 0.08}, medium=0.15}
gv3 = pile{{"lead white", 1.6}, {"cobalt blue", 0.35}, {"raw umber", 0.6}, {"yellow ochre", 0.6}, {"Indian red", 0.25}, medium=0.15}

--@ chunk 184
local both = grLm + grRm
local n = noise{seed=1481, octaves=3, period=30}
local ang = function(x,y) return -1.5 + 0.4*math.sin(x/9) end
local cl = both:grow(1) - slfol
local top = both * mask(function(x,y) return (y < 372 + n(x,0)*8) and 1 or 0 end)
local base = both * mask(function(x,y) return (y > 392 + n(x,0)*6) and 1 or 0 end)
local mid = both - top - base
work(mid, {hand="detail", tool="filbert 5", pile=gv1, angle=ang, angle_jitter=0.5, length={5,12}, coverage=2.4, pressure={0.65,0.95}, load=0.85, fill=true, clip=cl, seed=1482})
work(top, {hand="detail", tool="filbert 4", pile=gv3, angle=ang, angle_jitter=0.5, length={4,10}, coverage=2.2, pressure={0.6,0.9}, load=0.8, fill=true, clip=cl, seed=1483})
work(base, {hand="detail", tool="filbert 5", pile=gv2, angle=ang, angle_jitter=0.5, length={5,12}, coverage=2.4, pressure={0.65,0.95}, load=0.85, fill=true, clip=cl, seed=1484})
print(wait(0))

--@ chunk 185
local both = grLm + grRm
blend(both:grow(1), {angle=-1.4, coverage=3, seed=1491})
blend(both:grow(1), {angle=-0.3, coverage=2, seed=1492})
-- cover the old mauve rim with the top tone, letting it run lost
local rim = (both:grow(5) - both:shrink(2)) * above({{0,380},{1000,380}}) - slfol
work(rim, {hand="detail", tool="filbert 4", pile=gv3, angle=-1.5, angle_jitter=0.6, length={4,9}, coverage=2.2, pressure={0.6,0.9}, load=0.75, fill=true, clip=rim, seed=1493})
local rimlow = (both:grow(5) - both:shrink(2)) * below({{0,380},{1000,380}}) * above({{0,409},{1000,409}})
work(rimlow, {hand="detail", tool="filbert 4", pile=gv1, angle=-1.5, angle_jitter=0.6, length={4,9}, coverage=2.2, pressure={0.6,0.9}, load=0.75, fill=true, clip=rimlow, seed=1494})
print(wait(0))

--@ chunk 186
local both = grLm + grRm
local seam = mask(function(x,y) return (y > 400 and y < 416) and 1 or 0 end) * both:grow(6)
work(seam, {hand="detail", tool="filbert 4", pile=gv2, angle=0, angle_jitter=0.2, length={6,14}, coverage=2.0, pressure={0.6,0.9}, load=0.75, fill=true, clip=seam * above({{0,409},{1000,409}}), seed=1501})
blend(both:grow(3) * below({{0,385},{1000,385}}), {angle=-1.45, coverage=2, seed=1502})
print(wait(0))

--@ chunk 187
for _, p in ipairs({{150,485},{250,515},{400,515}}) do print(p[1], p[2], drying(p[1], p[2])) end
bank1 = pile{{"raw umber", 1}, {"raw sienna", 0.6}, {"yellow ochre", 0.5}, {"Antwerp blue", 0.1}, {"lead white", 0.25}, medium=0.15}
bank2 = pile{{"raw umber", 1}, {"raw sienna", 0.5}, {"yellow ochre", 0.3}, {"Antwerp blue", 0.12}, {"bone black", 0.08}, {"lead white", 0.12}, medium=0.15}
local blk = poly({{118,470},{215,470},{215,486},{180,490},{160,494},{140,500},{118,500}}, true):roughen(4, 14, 1511)
work(blk - trunkm, {hand="detail", tool="filbert 6", pile=bank1, angle=0, angle_jitter=0.15, length={10,24}, coverage=2.4, pressure={0.6,0.9}, load=0.8, fill=true, clip=blk:grow(3), seed=1512})
-- near bank strip: repaint as one soft dark band, then fuse
local nb = mask(function(x,y) local top = 506 + (x - 150) * 0.018; return (x > 120 and x < 520 and y > top and y < top + 16) and 1 or 0 end) - pondm
work(nb, {hand="detail", tool="filbert 6", pile=bank2, angle=0, angle_jitter=0.1, length={14,30}, coverage=2.4, pressure={0.6,0.9}, load=0.8, fill=true, clip=nb:grow(2), seed=1513})
blend((blk + nb):grow(4) - pondm:shrink(1), {angle=0, coverage=2.5, seed=1514})
print(wait(0))

--@ chunk 188
local z = poly({{100,460},{235,460},{235,495},{120,512},{100,512}}, true):blur(10)
blend(z - trunkm, {angle=0.1, coverage=3, seed=1521})
print(wait(5*24*60))
for _, p in ipairs({{150,485},{250,515}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 189
gl_bank = pile{{"raw umber", 1}, {"raw sienna", 0.4}, {"Antwerp blue", 0.12}, {"bone black", 0.12}, medium=0.75}
local reg = poly({{95,452},{240,455},{250,490},{520,500},{525,535},{110,535},{95,520}}, true):soften(8) - pondm:shrink(1)
work(reg, {hand="glaze", tool={kind="filbert", width=12, stiffness=0.3}, pile=gl_bank, angle=0.02, length={30,80}, coverage=1.8, clip=reg, pressure={0.45,0.75},
  load_at=function(x,y) return clamp(0.25 + (y - 460) / 80 * 0.3, 0.2, 0.55) end, seed=1531})
blend(reg, {angle=0.0, coverage=2.5, seed=1532})
blend(reg, {angle=-1.3, coverage=1.5, seed=1533})
print(wait(0))

--@ chunk 190
print(wait(5*24*60))
for _, p in ipairs({{150,485},{250,515},{400,470}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 191
pondo2 = outline{{168,500,"c"},{205,491},{260,487},{330,486},{400,488},{458,494,"c"},{430,501},{380,507},{310,511},{245,512},{195,509},
  closed=true, char="soft", seed=1541, amount=0.5, lobe=10}
pond2 = pondo2:mask()
local n1 = noise{seed=1542, octaves=3, period=120, stretch={0, 4}}
lm = mask(function(x,y) return (y > 410 and y < 548 + n1(x,0)*6 and x < 640 - (y-410)*0.3) and 1 or 0 end) - pond2 - distm - grLm - grRm
local function g(x,y) return (y - 410) / 130 + math.abs(x - 320) / 1400 + n1(x,y) * 0.12 end
local z1 = lm * mask(function(x,y) return g(x,y) < 0.18 and 1 or 0 end)
local z2 = lm * mask(function(x,y) local v = g(x,y); return (v >= 0.18 and v < 0.45) and 1 or 0 end)
local z3 = lm * mask(function(x,y) local v = g(x,y); return (v >= 0.45 and v < 0.75) and 1 or 0 end)
local z4 = lm * mask(function(x,y) return g(x,y) >= 0.75 and 1 or 0 end)
lm1 = pile{{"lead white", 2.4}, {"yellow ochre", 1}, {"raw umber", 0.35}, {"lemon chrome", 0.15}, {"Indian red", 0.1}, {"cobalt blue", 0.08}, medium=0.15}
lm2 = pile{{"yellow ochre", 1}, {"raw sienna", 0.5}, {"raw umber", 0.5}, {"lead white", 0.8}, {"Antwerp blue", 0.08}, medium=0.15}
lm3 = pile{{"raw sienna", 1}, {"raw umber", 0.9}, {"yellow ochre", 0.5}, {"Antwerp blue", 0.14}, {"lead white", 0.3}, medium=0.15}
lm4 = pile{{"raw umber", 1}, {"raw sienna", 0.5}, {"yellow ochre", 0.3}, {"Antwerp blue", 0.12}, {"bone black", 0.08}, {"lead white", 0.12}, medium=0.15}
local o = function(p, s, tool) return {hand="body", tool=tool or "filbert 12", pile=p, angle=0, angle_jitter=0.06, length={40,100}, coverage=2.0, pressure={0.6,0.9}, load=0.8, fill=true, edge="soft", seed=s} end
work(z1, o(lm1, 1543, "filbert 9"))
work(z2, o(lm2, 1544))
work(z3, o(lm3, 1545))
work(z4, o(lm4, 1546))
print(wait(0))

--@ chunk 192
local z = lm:grow(6) - pond2:shrink(1)
blend(z, {angle=0.0, coverage=3, seed=1551})
blend(z, {angle=0.12, coverage=2.5, seed=1552})
local edges = (lm:grow(25) - lm:shrink(25)) - pond2
blend(edges, {angle=-0.6, coverage=3, clip=false, seed=1553})
blend(edges, {angle=0.6, coverage=2, clip=false, seed=1554})
print(wait(0))

--@ chunk 193
print(wait(6*24*60))
for _, p in ipairs({{100,320},{300,330},{450,350},{300,450},{150,600},{500,600},{740,450}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 194
local tree = (canopy + hangm):grow(1)
local nz = noise{seed=311, octaves=4, period=180, persistence=0.5, stretch={0.05, 3}}
local function dist(x, y) return math.sqrt((x-330)^2 + ((y-380)*2.6)^2) + nz(x,y)*70 end
local reg = mask(function(x,y)
  if x > 690 then return 0 end
  if y > tl_top(x) + 3 then return 0 end
  return clamp((y - 200) / 40, 0, 1)
end) - tree
repsky = reg
local function bandm(d0, d1) return reg * mask(function(x,y) local d = dist(x,y); return (d >= d0 and d < d1) and 1 or 0 end) end
local function lay(m, p, s, tool)
  work(m, {hand="body", tool=tool or "filbert 12", pile=p, angle=function(x,y) return 0.03*math.sin(x/110) - 0.02 end, angle_jitter=0.12, length={40,90}, coverage=2.0, pressure={0.65,0.95}, load=0.85, fill=true, clip=reg, seed=s})
end
lay(bandm(540, 99999), hs2, 1561)
lay(bandm(330, 540), hs3, 1562)
lay(bandm(160, 330), hs4, 1563)
lay(bandm(-999, 160), ns5, 1564, "filbert 10")
print(wait(0))

--@ chunk 195
blend(repsky, {angle=0.0, coverage=3, seed=1571})
blend(repsky, {angle=0.2, coverage=2, seed=1572})
print(wait(0))

--@ chunk 196
local tree = (canopy + hangm):grow(1)
local xf = function(x) return clamp((705 - x) / 50, 0, 1) end
local tlm = mask(function(x,y) local t = tl_top(x); return (y > t and y < 411) and xf(x) or 0 end) - tree
local glowzone = mask(function(x,y) return (math.abs(x - 320) < 190) and 1 or 0 end)
local tw = tlm * glowzone
local tc = tlm - tw
local ang = function(x,y) return -1.4 + 0.3*math.sin(x/23) end
work(tc, {hand="detail", tool="filbert 6", pile=far_c, angle=ang, angle_jitter=0.4, length={8,20}, coverage=2.4, pressure={0.55,0.9}, load=0.8, fill=true, clip=tlm, seed=1581})
work(tw, {hand="detail", tool="filbert 6", pile=far_w, angle=ang, angle_jitter=0.4, length={8,20}, coverage=2.4, pressure={0.55,0.9}, load=0.8, fill=true, clip=tlm, seed=1582})
newtl = tlm
print(wait(0))

--@ chunk 197
local tree = (canopy + hangm):grow(1)
local n1 = noise{seed=1591, octaves=3, period=120, stretch={0, 4}}
local xf = function(x, y) return clamp((720 - x - (y - 410) * 0.15) / 60, 0, 1) end
newland = mask(function(x,y) if y < 409 then return 0 end; return xf(x, y) end) - tree - trunkm:grow(2)
local function g(x,y) return (y - 409) / 150 + math.abs(x - 320) / 1600 + n1(x,y) * 0.1 end
local function z(a, b) return newland * mask(function(x,y) local v = g(x,y); return (v >= a and v < b) and 1 or 0 end) end
local o = function(p, s, tool) return {hand="body", tool=tool or "filbert 14", pile=p, angle=function(x,y) return -0.02 + 0.06*math.sin(x/150 + y/80) end, angle_jitter=0.08, length={40,100}, coverage=2.0, pressure={0.6,0.9}, load=0.8, fill=true, clip=newland, seed=s} end
work(z(-9, 0.12), o(lm1, 1592, "filbert 9"))
work(z(0.12, 0.35), o(lm2, 1593))
work(z(0.35, 0.62), o(lm3, 1594))
work(z(0.62, 0.95), o(lm4, 1595))
work(z(0.95, 1.4), o(f2, 1596))
work(z(1.4, 99), o(f3, 1597))
print(wait(0))

--@ chunk 198
blend(newland, {angle=0.0, coverage=3, seed=1601})
blend(newland, {angle=0.15, coverage=2.5, seed=1602})
blend(newland, {angle=-0.12, coverage=2, seed=1603})
blend(newtl, {angle=-1.4, coverage=1.5, seed=1604})
print(wait(0))

--@ chunk 199
local tree = (canopy + hangm):grow(2)
local seam = mask(function(x,y) local c = 705 - (y - 410) * 0.15; return (y > 409 and math.abs(x - c) < 45) and 1 or 0 end):soften(10) - tree - trunkm:grow(4)
blend(seam, {angle=0.0, coverage=3, seed=1611})
blend(seam, {angle=0.3, coverage=2, seed=1612})
print(wait(0))

--@ chunk 200
local tree = (canopy + hangm):grow(1)
local n1 = noise{seed=1621, octaves=3, period=140, stretch={0, 4}}
local n2 = noise{seed=1622, octaves=2, period=300}
fullland = mask(function(x,y) return y >= 409 and 1 or 0 end) - tree - trunkm:grow(1)
local function g(x,y) return (y - 409) / 150 + math.abs(x - 320) / 1600 + n1(x,y) * 0.1 + n2(x,y) * 0.06 end
local function z(a, b) return fullland * mask(function(x,y) local v = g(x,y); return (v >= a and v < b) and 1 or 0 end) end
local o = function(p, s, tool) return {hand="body", tool=tool or "filbert 14", pile=p, angle=function(x,y) return -0.02 + 0.06*math.sin(x/150 + y/80) end, angle_jitter=0.08, length={40,100}, coverage=1.8, pressure={0.6,0.9}, load=0.75, fill=true, clip=fullland, seed=s} end
work(z(-9, 0.12), o(lm1, 1623, "filbert 9"))
work(z(0.12, 0.35), o(lm2, 1624))
work(z(0.35, 0.62), o(lm3, 1625))
work(z(0.62, 0.95), o(lm4, 1626))
work(z(0.95, 1.4), o(f2, 1627))
work(z(1.4, 99), o(f3, 1628))
blend(fullland, {angle=0.0, coverage=3, seed=1629})
blend(fullland, {angle=0.15, coverage=2.5, seed=1630})
blend(fullland, {angle=-0.12, coverage=2, seed=1631})
print(wait(0))

--@ chunk 201
print(wait(6*24*60))
for _, p in ipairs({{100,320},{300,380},{300,450},{150,600},{800,600},{900,440}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 202
local tn = noise{seed=1641, octaves=4, period=26, persistence=0.6}
local function grove(x0, x1, h, base)
  return mask(function(x,y)
    if x < x0 - 10 or x > x1 + 10 then return 0 end
    local t = (x - x0) / (x1 - x0)
    local env = math.sin(clamp(t, 0, 1) * math.pi) ^ 0.6
    local top = base - h * env - tn(x, 0) * 7 * env
    return (y > top and y < base + 3) and 1 or 0
  end)
end
grA = (grove(-30, 175, 42, 408) + grove(110, 240, 18, 408)):blur(2)
grB = (grove(420, 600, 30, 408) + grove(520, 640, 20, 408)):blur(2)
local both = (grA + grB) - (canopy + hangm):grow(2)
stipple(both, {pile=gv1, width=5, coverage=function(x,y) return 2.2 * both:at(x,y) end, pressure={0.4,0.8}, dips={12, 0.6, 0.3}, cluster={0.5, 6}, feather=0.6, drag={4, -1.5}, seed=1642})
local lower = both * below({{0,395},{1000,395}})
stipple(lower, {pile=gv2, width=5, coverage=function(x,y) return 1.6 * lower:at(x,y) end, pressure={0.4,0.8}, dips={12, 0.6, 0.3}, cluster={0.5, 6}, feather=0.6, drag={3, 0}, seed=1643})
print(wait(0))

--@ chunk 203
local tn = noise{seed=1651, octaves=4, period=18, persistence=0.6}
local midtl = mask(function(x,y)
  if x < 140 or x > 470 then return 0 end
  local top = tl_top(x) - 3 - tn(x,0) * 4
  return (y > top and y < 409) and 1 or 0
end):blur(1.5)
local topband = midtl * mask(function(x,y) return y < tl_top(x) + 8 and 1 or 0 end)
stipple(topband:grow(3), {pile=far_w, width=3.5, coverage=function(x,y) return 1.5 * topband:grow(3):at(x,y) end, pressure={0.3,0.65}, dips={14, 0.5, 0.3}, cluster={0.5, 5}, feather=0.7, drag={2, -1.5}, seed=1652})
-- haze veil over the distant middle treeline
local veil = mask(function(x,y) return (x > 130 and x < 480 and y > 375 and y < 412) and 1 or 0 end):soften(12) - grA - grB
work(veil, {hand="scumble", tool={kind="filbert", width=10, stiffness=0.8}, pile=hs4, angle=0, length={20,40}, coverage=0.7, pressure={0.2,0.4}, load=0.18, edge="lost", clip=veil, seed=1653})
print(wait(0))

--@ chunk 204
local veil = mask(function(x,y) return (x > 130 and x < 480 and y > 370 and y < 412) and 1 or 0 end):soften(12)
blend(veil, {angle=0, coverage=3, seed=1661})
blend(veil, {angle=-0.3, coverage=2, seed=1662})
print(wait(0))

--@ chunk 205
pondN = outline{{150,462,"c"},{185,454},{235,450},{290,449},{345,451},{392,455},{425,460,"c"},{400,465},{360,468},{320,471},{300,476},{262,478},{220,476},{185,471},
  closed=true, char="soft", seed=1671, amount=0.5, lobe=9}
pondNm = pondN:mask()
pw1 = pile{{"lead white", 5}, {"lemon chrome", 0.9}, {"cadmium yellow", 0.25}, {"orange chrome", 0.06}, medium=0.1}
pw2 = pile{{"lead white", 5}, {"yellow ochre", 0.5}, {"Indian red", 0.15}, {"cobalt blue", 0.1}, medium=0.1}
local near = pondNm * mask(function(x,y) return y > 464 and 1 or 0 end)
work(pondNm - near, {hand="detail", tool="filbert 6", pile=pw1, angle=0, angle_jitter=0.04, length={20,50}, coverage=2.6, pressure={0.65,0.95}, load=0.9, fill=true, clip=pondNm, seed=1672})
work(near, {hand="detail", tool="filbert 5", pile=pw2, angle=0, angle_jitter=0.04, length={20,50}, coverage=2.4, pressure={0.65,0.95}, load=0.85, fill=true, clip=pondNm, seed=1673})
blend(pondNm, {angle=0, coverage=2, seed=1674})
print(wait(0))

--@ chunk 206
bankF = pile{{"raw umber", 1}, {"raw sienna", 0.4}, {"Antwerp blue", 0.12}, {"yellow ochre", 0.3}, {"lead white", 0.25}, medium=0.15}
local b = brush{kind="filbert", width=5, stiffness=0.6}
local segs = {
  {{140,452},{190,451},{240,449.5}}, {{230,450},{290,448.5},{345,449.5}}, {{335,450},{390,452},{432,455}},
}
for _, s in ipairs(segs) do
  b:load(bankF, 0.8)
  b:stroke(s, {pressure={0.8,0.7}, ramps={0.1,0.2}, shake=0.2})
end
-- near bank: firm lower edge, slightly curving
local nb = brush{kind="filbert", width=6, stiffness=0.6}
for _, s in ipairs({{{150,466},{190,472},{240,476}}, {{235,477},{280,477},{320,472}}, {{310,473},{360,469},{410,465},{432,461}}}) do
  nb:load(lm3, 0.8)
  nb:stroke(s, {pressure={0.8,0.7}, ramps={0.1,0.2}, shake=0.2})
end
print(wait(0))

--@ chunk 207
local nbz = (pondNm:grow(8) - pondNm:shrink(1)) * mask(function(x,y) return y > 462 and 1 or 0 end)
blend(nbz, {angle=0, coverage=2, seed=1681})
local fbz = (pondNm:grow(5) - pondNm:shrink(3)) * mask(function(x,y) return y < 458 and 1 or 0 end)
blend(fbz, {angle=0, coverage=1.5, seed=1682})
print(wait(0))

--@ chunk 208
print(wait(5*24*60))
for _, p in ipairs({{300,460},{100,390},{500,395},{300,400}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 209
-- re-establish the pond with a crisp straight-edged shape: paint the meadow around it back over the smears
pondS = poly({{158,458},{200,454.5},{260,452.5},{320,452.5},{380,454.5},{425,458.5},{395,462},{340,465.5},{290,467},{240,466.5},{195,463.5}}, true)
local ring = (pondS:grow(16) - pondS) * mask(function(x,y) return (x > 120 and x < 470) and 1 or 0 end)
local up = ring * mask(function(x,y) return y < 460 and 1 or 0 end)
local dn = ring - up
mU = pile{{"yellow ochre", 1}, {"raw sienna", 0.45}, {"raw umber", 0.45}, {"lead white", 0.85}, {"Antwerp blue", 0.08}, medium=0.15}
mD = pile{{"yellow ochre", 0.9}, {"raw sienna", 0.5}, {"raw umber", 0.6}, {"lead white", 0.55}, {"Antwerp blue", 0.1}, medium=0.15}
work(up, {hand="detail", tool="filbert 6", pile=mU, angle=0, angle_jitter=0.05, length={15,40}, coverage=2.6, pressure={0.65,0.95}, load=0.85, fill=true, clip=up, seed=1691})
work(dn, {hand="detail", tool="filbert 6", pile=mD, angle=0, angle_jitter=0.05, length={15,40}, coverage=2.6, pressure={0.65,0.95}, load=0.85, fill=true, clip=dn, seed=1692})
print(wait(0))

--@ chunk 210
local z = ((pondS:grow(40) - pondS):blur(8)) * mask(function(x,y) return y > 412 and 1 or 0 end)
blend(z, {angle=0, coverage=3, seed=1701})
blend(z, {angle=0.12, coverage=3, seed=1702})
blend(z, {angle=-0.1, coverage=2, seed=1703})
print(wait(0))

--@ chunk 211
print(wait(5*24*60))
for _, p in ipairs({{300,460},{150,430},{450,500}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 212
gl_m = pile{{"raw umber", 1}, {"raw sienna", 0.6}, {"Antwerp blue", 0.06}, medium=0.85}
local n1 = noise{seed=1711, octaves=3, period=150, stretch={0, 4}}
local mz = mask(function(x,y) return (y > 412 and y < 600) and 1 or 0 end):soften(6) - (canopy + hangm):grow(2) - trunkm:grow(2)
work(mz, {hand="glaze", pile=gl_m, angle=0.0, angle_jitter=0.05, coverage=1.4, clip=mz, pressure={0.4,0.7},
  load_at=function(x,y) 
    local ov = ((x - 290) / 200)^2 + ((y - 463) / 52)^2
    local inside = ov < 1.1 and 0.22 or 0.08
    return clamp(inside + (y - 412) / 300 * 0.2 + n1(x,y) * 0.05, 0.04, 0.5) end, seed=1712})
print(wait(0))

--@ chunk 213
print(wait(4*24*60))
for _, p in ipairs({{300,460},{150,430},{450,500}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 214
pondW = outline{{132,452,"c"},{170,448},{230,446},{290,445},{350,446},{410,448},{452,452,"c"},{438,460},{410,466},{380,472},{350,480},{318,488},{285,491},{250,489},{215,482},{180,472},{150,462},
  closed=true, char="soft", seed=1721, amount=0.35, lobe=12}
pondWm = pondW:mask()
local zf = pondWm * mask(function(x,y) return y < 458 and 1 or 0 end)
local zm = pondWm * mask(function(x,y) return (y >= 458 and y < 474) and 1 or 0 end)
local zn = pondWm * mask(function(x,y) return y >= 474 and 1 or 0 end)
pw3 = pile{{"lead white", 4}, {"cobalt blue", 0.35}, {"Indian red", 0.3}, {"yellow ochre", 0.3}, {"raw umber", 0.2}, medium=0.1}
local o = function(p, s) return {hand="detail", tool="filbert 6", pile=p, angle=0, angle_jitter=0.03, length={20,50}, coverage=2.8, pressure={0.7,0.95}, load=0.9, fill=true, clip=pondWm, seed=s} end
work(zf, o(pw1, 1722))
work(zm, o(pw2, 1723))
work(zn, o(pw3, 1724))
blend(pondWm:shrink(3), {angle=0, coverage=2, seed=1725})
print(wait(0))

--@ chunk 215
print(wait(4*24*60))
for _, p in ipairs({{300,460},{200,465},{400,455}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 216
-- far bank: a straight strip, covering the lumps
farbank = poly({{120,441},{460,441},{462,452.5},{430,453.5},{360,452.5},{290,452},{220,452.5},{160,453.5},{120,454}}, false)
work(farbank, {hand="detail", tool="filbert 4", pile=bankF, angle=0, angle_jitter=0.02, length={15,40}, coverage=2.8, pressure={0.7,0.95}, load=0.85, fill=true, clip=farbank, seed=1731})
-- near and side banks: darker olive band hugging the pond, irregular outer edge, covering the light oval rim
local outerB = outline{{95,470},{110,446},{135,438},{180,440},{240,438},{320,438},{400,439},{460,440},{485,452},{490,480},{470,505},{420,512},{350,515},{280,520},{200,516},{140,508},{100,494},
  closed=true, char="soft", seed=1732, amount=0.8, lobe=14}
banks = outerB:mask() - pondWm - farbank
bnk = pile{{"raw umber", 1}, {"raw sienna", 0.55}, {"yellow ochre", 0.4}, {"Antwerp blue", 0.12}, {"lead white", 0.25}, medium=0.15}
work(banks, {hand="detail", tool="filbert 6", pile=bnk, angle=0, angle_jitter=0.08, length={15,40}, coverage=2.6, pressure={0.65,0.95}, load=0.85, fill=true, clip=banks:grow(1) - pondWm, seed=1733})
print(wait(0))

--@ chunk 217
local ob = banks + pondWm + farbank
local edgeband = (ob:grow(10) - ob:shrink(10)) - pondWm:grow(8)
blend(edgeband, {angle=0, coverage=2.5, seed=1741})
blend(edgeband, {angle=0.4, coverage=1.5, seed=1742})
print(wait(0))

--@ chunk 218
print(wait(5*24*60))
for _, p in ipairs({{300,460},{120,470},{450,500}}) do print(p[1], p[2], drying(p[1], p[2])) end
function hstrokes(o)
  local b = brush{kind=o.kind or "filbert", width=o.width, stiffness=o.stiff or 0.55}
  local n = 0
  local y = o.y0
  while y <= o.y1 do
    local x = o.x0 - rand(0, 60)
    while x < o.x1 do
      local L = rand(o.lmin or 60, o.lmax or 160)
      local p = o.pile
      if type(p) == "function" then p = p(x, y) end
      b:reload(p, o.load or 0.7)
      local yy = y + rand(-o.width * 0.25, o.width * 0.25)
      local tilt = rand(-1, 1) * (o.tilt or 2)
      local mx = x + L * 0.5
      b:stroke({{x, yy}, {mx, yy + tilt * 0.5 + rand(-1.5, 1.5)}, {x + L, yy + tilt}},
        {pressure={o.pmax or 0.8, o.pmin or 0.5}, ramps={0.08, 0.3}, shake=0.4, swell={1, 1.05, 0.85}})
      n = n + 1
      x = x + L * rand(0.55, 0.85)
    end
    y = y + o.width * (o.step or 0.55)
  end
  return n
end

--@ chunk 219
function hstrokes(o)
  local b = brush{kind=o.kind or "filbert", width=o.width, stiffness=o.stiff or 0.55}
  local n = 0
  local y = o.y0
  while y <= o.y1 do
    local x = o.x0 - rand(0, 60)
    while x < o.x1 do
      local L = rand(o.lmin or 60, o.lmax or 160)
      if o.xmax and x + L > o.xmax then L = o.xmax - x end
      if L > 15 then
        local p = o.pile
        if type(p) == "function" then p = p(x + L/2, y) end
        b:reload(p, o.load or 0.7)
        local yy = y + rand(-o.width * 0.25, o.width * 0.25)
        local tilt = rand(-1, 1) * (o.tilt or 2)
        b:stroke({{x, yy}, {x + L * 0.5, yy + tilt * 0.5 + rand(-1.5, 1.5)}, {x + L, yy + tilt}},
          {pressure={o.pmax or 0.8, o.pmin or 0.5}, ramps={0.08, 0.3}, shake=0.4, swell={1, 1.05, 0.85}})
        n = n + 1
      end
      x = x + math.max(L, 20) * rand(0.55, 0.85)
    end
    y = y + o.width * (o.step or 0.55)
  end
  return n
end
local function mp(x, y)
  local g = (y - 412) / 130 + math.abs(x - 320) / 1500 + rand(-0.06, 0.06)
  if g < 0.12 then return lm1 elseif g < 0.32 then return lm2 elseif g < 0.6 then return lm3 elseif g < 0.85 then return lm4 else return f2 end
end
print(hstrokes{x0=-10, x1=600, xmax=690, y0=418, y1=548, width=14, pile=mp, load=0.75, lmin=70, lmax=170, step=0.5})
print(wait(0))

--@ chunk 220
local z = rect(-10, 413, 700, 140) - (canopy + hangm):grow(4) - trunkm:grow(6)
blend(z, {angle=0, coverage=2.5, seed=1751})
blend(z, {angle=0.06, coverage=2, seed=1752})
print(wait(0))

--@ chunk 221
local b = brush{kind="flat", width=7, stiffness=0.6}
local rows = {
  {455, 175, 420, pw1}, {458, 160, 440, pw1}, {461, 170, 430, pw1}, {464, 190, 410, pw2}, {467, 215, 385, pw2}, {470, 245, 350, pw3},
}
for i, r in ipairs(rows) do
  local y, x0, x1, p = r[1], r[2], r[3], r[4]
  local x = x0
  while x < x1 do
    local L = math.min(rand(60, 110), x1 - x)
    b:reload(p, 0.9)
    b:stroke({{x, y + rand(-0.6, 0.6)}, {x + L * 0.5, y + rand(-0.6, 0.6)}, {x + L, y + rand(-0.6, 0.6)}}, {pressure={0.95, 0.85}, ramps={0.05, 0.15}, shake=0.15, orient="across"})
    x = x + L * 0.8
  end
end
print(wait(0))

--@ chunk 222
print(wait(5*24*60))
for _, p in ipairs({{300,460},{120,470},{450,500},{300,430}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 223
-- pond over dry paint: a long thin lens, light stroke by stroke, top edge straight (far bank), bottom edge slightly curved
local b = brush{kind="flat", width=6, stiffness=0.6}
local rows = {
  {456.0, 168, 436, pw1}, {459.5, 172, 432, pw1}, {463.0, 186, 418, pw2}, {466.5, 212, 392, pw2}, {469.5, 250, 352, pw3},
}
for i, r in ipairs(rows) do
  local y, x0, x1, p = r[1], r[2], r[3], r[4]
  local x = x0
  while x < x1 - 4 do
    local L = math.min(rand(70, 120), x1 - x)
    b:reload(p, 0.95)
    b:stroke({{x, y}, {x + L * 0.5, y + rand(-0.3, 0.3)}, {x + L, y}}, {pressure={0.95, 0.9}, ramps={0.03, 0.08}, shake=0.05})
    x = x + L * 0.85
  end
end
print(wait(0))

--@ chunk 224
local r = brush{kind="filbert", width=3.2, stiffness=0.6}
local x = 165
while x < 440 do
  local L = math.min(rand(50, 90), 442 - x)
  r:reload(bankF, 0.8)
  r:stroke({{x, 453.2 + rand(-0.4, 0.4)}, {x + L * 0.5, 453.0 + rand(-0.6, 0.4)}, {x + L, 453.2 + rand(-0.4, 0.4)}}, {pressure={0.75, 0.65}, ramps={0.1, 0.2}, shake=0.2})
  x = x + L * 0.85
end
-- pointed ends of the water: meadow paint pulled in from each side
local e = brush{kind="filbert", width=6, stiffness=0.6}
e:reload(lm2, 0.8)
e:stroke({{140,458},{165,458},{182,461}}, {pressure={0.8,0.2}, ramps={0.05,0.6}})
e:reload(lm2, 0.8)
e:stroke({{150,463},{180,464},{205,467}}, {pressure={0.8,0.2}, ramps={0.05,0.6}})
e:reload(lm2, 0.8)
e:stroke({{460,458},{438,458},{418,462}}, {pressure={0.8,0.2}, ramps={0.05,0.6}})
e:reload(lm2, 0.8)
e:stroke({{450,464},{420,465},{392,468}}, {pressure={0.8,0.2}, ramps={0.05,0.6}})
-- reflection of the far bank: a faint warm shadow line just under the bank
local rr = brush{kind="flat", width=2.2, stiffness=0.5}
rr:reload(pile{{"lead white", 2}, {"yellow ochre", 0.6}, {"raw umber", 0.4}, {"cobalt blue", 0.1}, medium=0.2}, 0.5)
rr:stroke({{190,455.4},{300,455.2},{420,455.4}}, {pressure={0.4,0.3}, ramps={0.2,0.3}, shake=0.3})
print(wait(0))

--@ chunk 225
print(wait(4*24*60))
local function mp(x, y)
  local g = (y - 412) / 150 + math.abs(x - 320) / 1600 + rand(-0.05, 0.05)
  if g < 0.12 then return lm1 elseif g < 0.32 then return lm2 elseif g < 0.6 then return lm3 elseif g < 0.85 then return lm4 elseif g < 1.25 then return f2 else return f3 end
end
-- right part of the land, around and under the oak (strokes go over the trunk bases; they'll be restated)
print(hstrokes{x0=560, x1=1010, y0=416, y1=480, width=12, pile=mp, load=0.75, lmin=60, lmax=140, step=0.5})
-- lower land across the whole width
print(hstrokes{x0=-10, x1=1010, y0=482, y1=672, width=18, pile=mp, load=0.75, lmin=80, lmax=200, step=0.5, tilt=4})
print(wait(0))

--@ chunk 226
local tree = (canopy + hangm):grow(4)
local z1 = rect(-10, 478, 1020, 200)
local z2 = rect(540, 414, 470, 70) - tree
blend(z1 + z2, {angle=0.0, coverage=2.5, seed=1761})
blend(z1 + z2, {angle=0.1, coverage=2, seed=1762})
blend(z1, {angle=-0.15, coverage=1.5, seed=1763})
print(wait(0))

--@ chunk 227
local seam = rect(490, 412, 110, 72):soften(12)
blend(seam, {angle=0.0, coverage=3, seed=1771})
blend(seam, {angle=0.25, coverage=2, seed=1772})
local seam2 = rect(440, 448, 120, 32):soften(10)
blend(seam2, {angle=0.0, coverage=2, seed=1773})
print(wait(0))

--@ chunk 228
print(wait(5*24*60))
for _, p in ipairs({{300,500},{700,450},{150,600},{900,600}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 229
local function mp(x, y)
  local g = (y - 412) / 150 + math.abs(x - 320) / 1600 + rand(-0.05, 0.05)
  if g < 0.12 then return lm1 elseif g < 0.32 then return lm2 elseif g < 0.6 then return lm3 elseif g < 0.85 then return lm4 elseif g < 1.25 then return f2 else return f3 end
end
-- the band below the pond, across the hard edge at y~478
print(hstrokes{x0=-10, x1=600, y0=472, y1=500, width=14, pile=mp, load=0.7, lmin=80, lmax=170, step=0.5, tilt=2})
-- banks either side of the pond: short strokes of meadow that taper the water's ends
print(hstrokes{x0=-10, x1=150, xmax=168, y0=440, y1=470, width=10, pile=mp, load=0.7, lmin=50, lmax=110, step=0.55})
print(hstrokes{x0=440, x1=600, y0=440, y1=470, width=10, pile=mp, load=0.7, lmin=50, lmax=110, step=0.55})
-- the strip between the far bank and the distant trees, over the old figure smudge
print(hstrokes{x0=-10, x1=600, y0=416, y1=446, width=10, pile=mp, load=0.7, lmin=70, lmax=150, step=0.55})
print(wait(0))

--@ chunk 230
pondSl = rect(170, 454, 260, 9)
local z = rect(-10, 414, 620, 90) - pondSl:grow(3)
blend(z, {angle=0.0, coverage=3, seed=1781})
blend(z, {angle=0.08, coverage=2, seed=1782})
print(wait(0))

--@ chunk 231
local z = rect(-10, 488, 620, 26):soften(8)
blend(z, {angle=-0.2, coverage=3, seed=1791})
blend(z, {angle=0.2, coverage=2, seed=1792})
print(wait(0))

--@ chunk 232
print(wait(5*24*60))
for _, p in ipairs({{300,500},{700,450},{150,600},{900,600},{300,430}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 233
local function mp(x, y)
  local g = (y - 412) / 155 + math.abs(x - 320) / 1700 + rand(-0.04, 0.04)
  if g < 0.1 then return lm1 elseif g < 0.3 then return lm2 elseif g < 0.56 then return lm3 elseif g < 0.82 then return lm4 elseif g < 1.2 then return f2 else return f3 end
end
print(hstrokes{x0=-10, x1=1010, y0=415, y1=480, width=12, pile=mp, load=0.75, lmin=80, lmax=180, step=0.5, tilt=1.5})
print(hstrokes{x0=-10, x1=1010, y0=482, y1=674, width=18, pile=mp, load=0.75, lmin=90, lmax=220, step=0.5, tilt=4})
local tree = (canopy + hangm):grow(3)
local land = rect(-10, 412, 1020, 262) - tree
blend(land, {angle=0.0, coverage=3, seed=1801})
blend(land, {angle=0.08, coverage=2.5, seed=1802})
blend(land, {angle=-0.08, coverage=2, seed=1803})
print(wait(0))

--@ chunk 234
print(wait(5*24*60))
for _, p in ipairs({{300,460},{700,450},{150,600},{900,600},{300,420}}) do print(p[1], p[2], drying(p[1], p[2])) end

--@ chunk 235
local b = brush{kind="flat", width=4, stiffness=0.6}
local rows = {
  {456.5, 195, 430, pw1}, {459.5, 190, 420, pw1}, {462.5, 205, 402, pw2}, {465.5, 230, 372, pw2}, {468.0, 262, 335, pw3},
}
for _, r in ipairs(rows) do
  local y, x0, x1, p = r[1], r[2], r[3], r[4]
  local mid = (x0 + x1) / 2
  b:reload(p, 0.95)
  b:stroke({{x0, y}, {mid, y + rand(-0.2, 0.2)}, {mid + 4, y}}, {pressure={0.55, 0.95}, ramps={0.35, 0.02}, shake=0.05})
  b:reload(p, 0.95)
  b:stroke({{mid - 4, y}, {mid, y}, {x1, y + rand(-0.2, 0.2)}}, {pressure={0.95, 0.5}, ramps={0.02, 0.35}, shake=0.05})
end
print(wait(0))

--@ chunk 236
local b = brush{kind="flat", width=4, stiffness=0.6}
local rows = {
  {456.5, 300, 428, pw1}, {459.5, 300, 418, pw1}, {462.5, 300, 400, pw2},
}
for _, r in ipairs(rows) do
  local y, x0, x1, p = r[1], r[2], r[3], r[4]
  b:reload(p, 1.0)
  b:stroke({{x1, y}, {(x0 + x1) / 2, y}, {x0, y}}, {pressure={0.75, 0.95}, ramps={0.15, 0.05}, shake=0.05})
end
-- thin dark far-bank edge, straight
local r = brush{kind="flat", width=2, stiffness=0.6}
r:reload(bankF, 0.8)
r:stroke({{185,454.3},{300,454.1},{435,454.4}}, {pressure={0.6,0.5}, ramps={0.25,0.25}, shake=0.1})
print(wait(0))

--@ chunk 237
blend(ellipse(312, 461, 110, 5), {angle=0, coverage=1.2, seed=1811})
print(wait(0))
