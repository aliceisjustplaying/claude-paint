-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box friedrich
--@ engine 5

--@ chunk 1
canvas{size=700, aspect=1.4, linen={14,13}, seed=1818,
 ground={
  {pile={{"red earth",2},{"yellow ochre",2},{"lead white",1}}, um=120, apply="knife", texture=0.4},
  {pile={{"lead white",6},{"yellow ochre",0.4},{"raw umber",0.15}}, um=60, apply="brush"}
 }}
print(W,H)
print(table.concat(tubes(), ", "))

--@ chunk 2
HZ = 478
h = pencil("2H")
h:rule({0, HZ}, {1000, HZ}, {pressure=0.25})
-- snow edge (near rise)
SNOW = {{0,566},{90,556},{200,540},{320,522},{430,509},{500,506},{560,507},{660,516},{780,530},{900,545},{1000,553}}
h:sketch(SNOW, {pressure=0.25})
-- far land profile
FAR = {{0,470},{60,466},{140,469},{230,463},{300,467},{380,471},{470,468},{560,470},{640,465},{720,461},{800,466},{880,470},{1000,467}}
h:sketch(FAR, {pressure=0.18})
-- oak skeleton
TRUNK = {{500,512},{497,470},{499,420},{503,370},{503,342}}
BA = {{502,345},{478,300},{438,262},{392,228},{342,205},{292,195},{252,198}}
BB = {{505,342},{514,290},{503,232},{512,172},{503,120},{492,84}}
BC = {{506,348},{548,308},{598,280},{650,246},{702,232},{752,208}}
LR = {{502,402},{545,392},{600,386},{652,372},{712,374}}
LL = {{498,388},{458,374},{410,362},{362,366},{318,350}}
for _,L in ipairs({TRUNK,BA,BB,BC,LR,LL}) do h:sketch(L, {pressure=0.3}) end

--@ chunk 3
s_glow = pile{{"lead white",6},{"Naples yellow",1.6},{"chrome yellow",0.08},{"rose madder",0.06}, name="glow"}
s_pale = pile{{"lead white",6},{"Naples yellow",0.5},{"cobalt blue",0.18},{"green earth",0.3}, name="pale"}
s_mid  = pile{{"lead white",5},{"smalt",1.2},{"cobalt blue",0.35},{"raw umber",0.05}, name="mid"}
s_top  = pile{{"lead white",3},{"smalt",1.6},{"cobalt blue",0.7},{"bone black",0.1},{"rose madder",0.12}, name="top"}

--@ chunk 4
s_top:add{{"smalt",1.0},{"cobalt blue",0.6},{"bone black",0.08}}
s_mid:add{{"smalt",0.5},{"cobalt blue",0.2}}
s_glow:add{{"rose madder",0.04},{"chrome yellow",0.06}}

--@ chunk 5
local function bump(y, c, r) local t = (y - c) / r; return math.max(0, 1 - t*t) end
sky = rect(0, 0, 1000, HZ + 14)
work(sky, {hand="broad", angle=0, coverage=2.2, angle_jitter=0.05,
  piles={
    {s_glow, function(x,y) return bump(y, 490, 105) end},
    {s_pale, function(x,y) return bump(y, 375, 110) end},
    {s_mid,  function(x,y) return bump(y, 250, 120) end},
    {s_top,  function(x,y) return y < 150 and 1 or bump(y, 150, 130) end}},
  fill=true})

--@ chunk 6
blend(rect(0,0,1000,HZ+10), {angle=0})
blend(rect(0,0,1000,HZ+10), {angle=0.03})

--@ chunk 7
s_zen = pile{{"lead white",2},{"smalt",2.2},{"cobalt blue",1.0},{"bone black",0.12},{"rose madder",0.15}, name="zenith"}
local function bump(y, c, r) local t = (y - c) / r; return math.max(0, 1 - t*t) end
work(rect(0,0,1000,330), {hand="broad", angle=0, coverage=1.6, angle_jitter=0.02, curve={0.02,0.0},
  piles={
    {s_pale, function(x,y) return bump(y, 340, 80) end},
    {s_mid,  function(x,y) return bump(y, 230, 110) end},
    {s_top,  function(x,y) return bump(y, 110, 110) end},
    {s_zen,  function(x,y) return y < 40 and 1 or bump(y, 40, 110) end}},
  fill=true, load=0.9})

--@ chunk 8
blend(rect(0,0,1000,380), {angle=0})

--@ chunk 9
sn_lt = pile{{"lead white",6},{"smalt",0.55},{"Naples yellow",0.3},{"rose madder",0.06}, name="snow lt"}
sn_md = pile{{"lead white",4},{"smalt",0.9},{"cobalt blue",0.15},{"raw umber",0.12},{"rose madder",0.08}, name="snow md"}
sn_dk = pile{{"lead white",3},{"smalt",1.1},{"cobalt blue",0.3},{"raw umber",0.3},{"rose madder",0.1}, name="snow dk"}
far_p = pile{{"lead white",3.5},{"smalt",0.9},{"rose madder",0.12},{"raw umber",0.18}, name="far"}

--@ chunk 10
sn_lt:add{{"smalt",0.4},{"cobalt blue",0.12},{"rose madder",0.04}}
sn_md:add{{"cobalt blue",0.3},{"bone black",0.08},{"smalt",0.5}}
sn_dk:add{{"cobalt blue",0.45},{"bone black",0.18},{"smalt",0.6},{"rose madder",0.06}}
far_p:add{{"smalt",0.8},{"cobalt blue",0.25},{"raw umber",0.15},{"rose madder",0.08}}

--@ chunk 11
snowm = below(SNOW)
farm = below(FAR) - snowm
work(farm, {hand="body", pile=far_p, angle=0, coverage=2, clip=true, fill=true, angle_jitter=0.03})
work(snowm, {hand="broad", angle=function(x,y) return (x<500) and -0.06 or 0.06 end, coverage=2.2, fill=true,
  piles={{sn_lt, function(x,y) return math.max(0, 1 - (y-505)/90) end},
         {sn_md, function(x,y) local t=(y-590)/80; return math.max(0,1-t*t) end},
         {sn_dk, function(x,y) return clamp((y-600)/90,0,1) end}}, clip=true})

--@ chunk 12
blend(snowm:shrink(3), {angle=0})
blend(farm:shrink(2), {angle=0})

--@ chunk 13
cl = pile{{"lead white",3},{"smalt",1.0},{"rose madder",0.18},{"raw umber",0.18},{"cobalt blue",0.1}, name="cloud"}
cb = brush{kind="filbert", width=14, stiffness=0.4}
cb:load(cl, 0.6)
-- long thin stratum, lower sky, left of centre, thinning to right
cb:gesture({{-20,352,0.5},{120,348,0.6},{300,343,0.55},{480,346,0.4},{640,342,0.25},{760,345,0.05}}, {wobble=3})
cb:load(cl, 0.4)
cb:gesture({{60,362,0.2},{200,358,0.35},{340,357,0.25},{430,360,0.05}}, {wobble=2})
-- a shorter, slimmer one low on the right
cb:load(cl, 0.45)
cb:gesture({{560,412,0.05},{700,407,0.3},{860,404,0.4},{1020,406,0.45}}, {wobble=2})
-- broad soft band high
local bb = brush{kind="filbert", width=30, stiffness=0.3}
bb:load(s_zen, 0.5)
bb:gesture({{-30,165,0.3},{250,158,0.45},{520,163,0.3},{800,150,0.4},{1030,156,0.35}}, {wobble=6})

--@ chunk 14
blend(rect(0,330,800,45):soften(8), {angle=0})
blend(rect(540,394,460,26):soften(6), {angle=0})
blend(rect(0,135,1000,50):soften(10), {angle=0})

--@ chunk 15
print(wait(2*24*60))
print(drying(500,100), drying(500,450), drying(500,600), drying(200,500))

--@ chunk 16
print(wait(24*60))
print(drying(500,100), drying(500,30), drying(500,250), drying(200,500))

--@ chunk 17
mtn = pile{{"lead white",4},{"smalt",1.1},{"rose madder",0.14},{"cobalt blue",0.2},{"Naples yellow",0.25}, name="mtn"}
forest = pile{{"lead white",1.2},{"smalt",1.3},{"cobalt blue",0.45},{"raw umber",0.5},{"bone black",0.2},{"rose madder",0.1}, name="forest"}
mist = pile{{"lead white",5},{"Naples yellow",0.45},{"smalt",0.45},{"rose madder",0.05}, name="mist"}
MTN = {{0,462},{70,458},{150,461},{210,455},{260,457},{330,463},{420,466},{520,465},{600,459},{660,450},{720,441},{775,436},{820,439},{880,449},{940,456},{1000,458}}
mtnm = below(MTN) * above(function(x) return 500 end)
work(mtnm, {hand="body", pile=mtn, angle=0, coverage=2, fill=true, edge={soft=0.5, found=0.3, lost=0.2, period=60}, angle_jitter=0.03})

--@ chunk 18
local edgezone = ribbon(MTN, 14):soften(4)
blend(edgezone, {angle=0})

--@ chunk 19
FOR = {{0,484},{80,481},{160,485},{240,480},{310,486},{390,489},{460,487},{540,488},{610,484},{680,480},{740,483},{820,478},{900,483},{1000,480}}
form_m = (below(FOR):roughen(2.5, 9, 7) - snowm):soften(0.8)
work(form_m, {hand="body", pile=forest, angle=0, coverage=2.2, fill=true, clip=true, tool="filbert 6"})

--@ chunk 20
function snowy(x)
  for i=1,#SNOW-1 do local a,b=SNOW[i],SNOW[i+1]
    if x>=a[1] and x<=b[1] then return lerp(a[2],b[2],(x-a[1])/(b[1]-a[1])) end end
  return SNOW[#SNOW][2]
end
-- first unify the forest band itself
blend(form_m:shrink(2), {angle=0})
local mistm = form_m * below(function(x) return snowy(x) - 0.55*(snowy(x)-484) end)
work(mistm, {hand="body", pile=mist, angle=0, coverage=1.6, clip=form_m, load=0.5, pressure={0.3,0.5}, tool="filbert 8"})
blend(form_m:shrink(1), {angle=0})

--@ chunk 21
local function t(x,y) local top=484; local bot=snowy(x); return clamp((y-top)/(bot-top),0,1) end
haze = mix{{forest,0.5},{mist,0.5}, name="haze"}
work(form_m, {hand="body", tool="filbert 7", angle=0, ruler=true, coverage=2, clip=true, fill=true, pressure={0.4,0.6},
  piles={{forest, function(x,y) return math.max(0,1-2*t(x,y)) end},
         {haze, function(x,y) local u=t(x,y); return math.max(0,1-math.abs(u-0.55)*2.2) end},
         {mist, function(x,y) return clamp((t(x,y)-0.6)*2.5,0,1) end}}})
work(form_m:shrink(1), {hand="blend", angle=0, ruler=true, angle_jitter=0, coverage=1})

--@ chunk 22
sn2_crest = pile{{"lead white",5},{"smalt",1.0},{"cobalt blue",0.2},{"rose madder",0.1},{"Naples yellow",0.2}, name="sn crest"}
sn2_mid = pile{{"lead white",3},{"smalt",1.2},{"cobalt blue",0.4},{"raw umber",0.25},{"rose madder",0.12},{"bone black",0.06}, name="sn mid"}
sn2_low = pile{{"lead white",2},{"smalt",1.3},{"cobalt blue",0.5},{"raw umber",0.4},{"rose madder",0.12},{"bone black",0.15}, name="sn low"}

--@ chunk 23
sn2_crest:add{{"smalt",0.5},{"cobalt blue",0.15},{"raw umber",0.12},{"rose madder",0.05}}

--@ chunk 24
local function d(x,y) local s=snowy(x); return clamp((y-s)/(714-s),0,1) end
local function corner(x) return math.abs(x-500)/500 end
work(snowm, {hand="broad", coverage=2.2, fill=true, clip=true,
  angle=function(x,y) return (x<500 and -1 or 1) * 0.12 * (1 - d(x,y)*0.5) end, angle_jitter=0.04,
  piles={{sn2_crest, function(x,y) local u=d(x,y)+0.25*corner(x); return math.max(0,1-u*2.8) end},
         {sn2_mid,   function(x,y) local u=d(x,y)+0.25*corner(x); return math.max(0,1-math.abs(u-0.42)*2.8) end},
         {sn2_low,   function(x,y) local u=d(x,y)+0.25*corner(x); return clamp((u-0.45)*2.5,0,1) end}}})
blend(snowm:shrink(2), {angle=0})

--@ chunk 25
print(wait(30*60))
print(drying(500,600), drying(200,690), drying(500,500), drying(500,300))

--@ chunk 26
bark = pile{{"bone black",1.2},{"raw umber",1.4},{"smalt",0.5},{"lead white",0.35},{"rose madder",0.1}, name="bark"}
bark_far = pile{{"bone black",0.8},{"raw umber",1.0},{"smalt",0.7},{"lead white",1.0},{"rose madder",0.1}, medium=0.15, name="bark far"}
twig = pile{{"bone black",1},{"raw umber",1.2},{"smalt",0.6},{"lead white",0.6}, medium=0.35, name="twig"}
bark_lt = pile{{"lead white",2},{"raw umber",1},{"bone black",0.5},{"smalt",0.6},{"rose madder",0.1}, name="bark lt"}

--@ chunk 27
bark:add{{"bone black",0.5},{"smalt",0.2}}
-- trunk: drawn as its own silhouette, flaring into the snow
TR_L = {{462,516},{474,508},{481,496},{484,478},{485,455},{487,430},{486,405},{488,382},{490,362},{489,346}}
TR_R = {{518,343},{517,360},{516,380},{518,402},{516,428},{514,452},{516,474},{521,494},{530,507},{545,515}}
local pts = {}
for _,p in ipairs(TR_L) do pts[#pts+1]=p end
for _,p in ipairs(TR_R) do pts[#pts+1]=p end
trunk_m = poly(pts, true):roughen(1.2, 8, 11)
-- main limbs: spines with widths (taper outward only)
L_A = {pts={{492,352},{474,334},{450,318},{424,306},{398,288},{372,266},{344,250},{312,240},{280,236},{250,228}}, w={20,17,15,14,12,10,9,7.5,6,4.5}}
L_B = {pts={{503,345},{509,318},{514,290},{508,262},{503,238},{510,210},{514,180},{507,150},{500,124},{496,100}}, w={21,18,16,14,12,10.5,9,7,5.5,4}}
L_C = {pts={{512,352},{534,336},{558,322},{584,312},{612,296},{640,276},{668,262},{700,254},{730,244},{760,230}}, w={19,16,14.5,13,11.5,10,8.5,7,5.5,4.2}}
L_D = {pts={{489,408},{466,402},{440,396},{414,394},{388,396},{360,390},{332,384},{306,374}}, w={12,10,9,8,7,6,5,3.6}}
L_E = {pts={{515,420},{540,413},{566,410},{592,404},{620,396},{648,392},{676,392},{704,384}}, w={11,9.5,8.5,7.5,6.5,5.5,4.5,3.4}}
limbs_m = ribbon(L_A.pts, L_A.w) + ribbon(L_B.pts, L_B.w) + ribbon(L_C.pts, L_C.w) + ribbon(L_D.pts, L_D.w) + ribbon(L_E.pts, L_E.w)
tree_core = (trunk_m + limbs_m:roughen(0.8, 6, 12))
print(tree_core:area())

--@ chunk 28
work(trunk_m, {hand="body", tool="filbert 5", pile=bark, angle=math.pi/2, angle_jitter=0.1, coverage=2.5, clip=true, fill=true})
local function seg_angle(L) local a,b=L.pts[1],L.pts[#L.pts]; return math.atan(b[2]-a[2], b[1]-a[1]) end
for _,L in ipairs({L_A,L_B,L_C,L_D,L_E}) do
  local m = ribbon(L.pts, L.w):roughen(0.8, 6, 12)
  work(m, {hand="body", tool="filbert 4", pile=bark, angle=seg_angle(L), angle_jitter=0.25, coverage=2.5, clip=true, fill=true})
end

--@ chunk 29
rb8 = brush{kind="round", width=8, point=1, stiffness=0.5}
rb4 = brush{kind="round", width=4, point=1, stiffness=0.5}
rg = brush{kind="rigger", width=1.6, point=1}
for _,b in ipairs({rb8,rb4,rg}) do
  local s=""
  for _,p in ipairs({0.05,0.2,0.4,0.6,0.8,1}) do s=s..string.format("%.2f->%.2f  ",p,b:mark_width(p)) end
  print(s)
end

--@ chunk 30
function pick_brush(w) if w > 4.2 then return rb8 elseif w > 1.7 then return rb4 else return rg end end
function draw_branch(pts, ws, pile, load)
  local b = pick_brush(ws[1])
  b:load(pile, load or 0.75)
  local g = {}
  for i,p in ipairs(pts) do
    local w = ws[i]
    local pr = (w < 0.6) and 0.2 or b:pressure_for(w)
    g[#g+1] = {p[1], p[2], clamp(pr,0.05,1)}
  end
  b:gesture(g, {wobble=0.6})
end
-- grow a crooked branch; returns pts, widths
function grow(x, y, ang, len, w0, w1, nseg, bend, up)
  local pts, ws = {{x,y}}, {w0}
  local a = ang
  local sl = len / nseg
  for i=1,nseg do
    a = a + rand(-bend, bend)
    -- pull upward (toward -pi/2) gently
    local target = -math.pi/2
    local diff = math.atan(math.sin(target-a), math.cos(target-a))
    a = a + diff*up
    x = x + math.cos(a)*sl; y = y + math.sin(a)*sl
    pts[#pts+1] = {x,y}
    ws[#ws+1] = lerp(w0, w1, i/nseg)
  end
  return pts, ws, a
end
function envelope(x,y) -- crown shape: 1 inside, 0 outside
  local dx=(x-500)/345; local dy=(y-262)/(y<262 and 190 or 160)
  return dx*dx+dy*dy
end
-- recursive branching; children along path
function tree_rec(x,y,ang,len,w,depth,pile)
  if w < 0.45 or depth > 5 or len < 8 then return end
  local nseg = math.max(2, math.floor(len/14))
  local pts, ws, aend = grow(x,y,ang,len,w,math.max(0.4,w*0.55),nseg, 0.32, 0.06)
  -- clip to envelope: cut path where outside
  local cut = #pts
  for i=2,#pts do if envelope(pts[i][1],pts[i][2]) > 1.0 + rand(-0.05,0.12) then cut=i break end end
  local P,Wd = {},{}
  for i=1,cut do P[i]=pts[i]; Wd[i]=ws[i] end
  if #P < 2 then return end
  Wd[#Wd] = math.min(Wd[#Wd], 0.5) 
  draw_branch(P, Wd, pile)
  -- children
  local nch = math.max(1, math.floor(#P*0.8))
  for k=1,nch do
    local i = math.random(2, #P)
    local px,py = P[i][1],P[i][2]
    local pa
    if i < #P then pa = math.atan(P[i+1][2]-py, P[i+1][1]-px) else pa = aend end
    local side = (math.random() < 0.5) and -1 or 1
    local ca = pa + side*rand(0.45, 1.0)
    local cw = Wd[i] * rand(0.5, 0.75)
    tree_rec(px,py,ca, len*rand(0.45,0.7), cw, depth+1, pile)
  end
end
print("ok helpers")

--@ chunk 31
local function tip(L) local n=#L.pts; local a=L.pts[n-1]; local b=L.pts[n]; return b[1],b[2],math.atan(b[2]-a[2],b[1]-a[1]), L.w[n] end
for _,L in ipairs({L_A,L_B,L_C,L_D,L_E}) do
  local x,y,a,w = tip(L)
  tree_rec(x,y,a, 85, w, 2, bark)
end

--@ chunk 32
function envelope(x,y)
  local dx=(x-500)/365; local dy=(y-258)/(y<258 and 205 or 165)
  return dx*dx+dy*dy
end
SEC = {
 {424,306,-1.75,120,7},{372,266,-2.0,110,6},{312,240,-1.9,90,5},{398,288,2.9,70,4},
 {508,262,-2.4,110,7},{510,210,-0.8,110,6},{507,150,-2.3,80,4.5},{514,290,-0.6,95,6},
 {584,312,-1.4,120,7},{640,276,-1.2,110,6},{700,254,-1.3,80,5},{612,296,0.15,70,4},
 {440,396,-1.9,80,5},{360,390,-2.1,75,4.5},{388,396,2.7,50,3.5},
 {566,410,-1.3,85,5},{648,392,-1.1,75,4.5},{676,392,0.4,45,3}}
for _,s in ipairs(SEC) do tree_rec(s[1],s[2],s[3],s[4],s[5],1,bark) end

--@ chunk 33
local L = {{446,518},{458,512},{468,503},{474,490},{476,472},{477,450},{476,428},{479,406},{481,386},{478,366},{472,350},{462,338}}
local R = {{528,334},{530,348},{525,366},{524,388},{526,410},{524,432},{523,452},{525,472},{531,492},{542,506},{560,516}}
local pts={}
for _,p in ipairs(L) do pts[#pts+1]=p end
pts[#pts+1]={480,334}; pts[#pts+1]={500,330}; pts[#pts+1]={515,330}
for _,p in ipairs(R) do pts[#pts+1]=p end
trunk2 = poly(pts, true):roughen(1.5, 10, 21)
work(trunk2, {hand="body", tool="filbert 6", pile=bark, angle=math.pi/2, angle_jitter=0.15, coverage=2.5, clip=true, fill=true})
-- thicken limb bases (collars), tapering out
local function collar(L, n, extra)
  local P,Wd={},{}
  for i=1,n do P[i]=L.pts[i]; Wd[i]=L.w[i] + extra*(1-(i-1)/(n-1)) end
  return ribbon(P,Wd)
end
local cm = collar(L_A,5,10)+collar(L_B,5,8)+collar(L_C,5,10)+collar(L_D,4,6)+collar(L_E,4,6)
work(cm:roughen(1,8,22), {hand="body", tool="filbert 5", pile=bark, angle_jitter=0.6, coverage=2.5, clip=true, fill=true})

--@ chunk 34
-- broken stub, right side of trunk
local stub = ribbon({{524,446},{540,438},{552,428},{557,420}}, {13,11,9.5,9}):roughen(1.2,6,31)
work(stub, {hand="body", tool="filbert 4", pile=bark, coverage=2.5, clip=true, fill=true, angle=-0.6})
-- left stub lower, short and ragged
local stub2 = ribbon({{478,470},{466,466},{458,460}}, {10,8,7}):roughen(1.2,5,32)
work(stub2, {hand="body", tool="filbert 4", pile=bark, coverage=2.5, clip=true, fill=true, angle=3.4})
-- crooked elbowed boughs, drawn by hand
local E = {
 {pts={{398,288},{388,262},{392,240},{378,214},{352,196},{344,170}}, w={9,8,7,6,5,3.5}},
 {pts={{612,296},{622,268},{612,244},{628,218},{652,204},{660,178}}, w={9,8,7,6,4.8,3.5}},
 {pts={{503,238},{478,214},{470,186},{452,166},{456,140},{442,118}}, w={8,7,6,5,4,3}},
 {pts={{514,180},{534,160},{556,154},{566,128},{586,110}}, w={7,6,5,4,3}},
 {pts={{344,250},{326,262},{300,262},{276,276},{252,276}}, w={6.5,5.5,4.5,3.5,2.6}},
 {pts={{668,262},{690,276},{716,276},{742,290},{766,290}}, w={6.5,5.5,4.5,3.5,2.6}},
}
for _,L in ipairs(E) do draw_branch(L.pts, L.w, bark, 0.9) end
-- stag-head: dead spikes above crown, no twigs, blunt broken ends
draw_branch({{496,100},{493,82},{497,62},{494,46}}, {4,3.6,3.2,3}, bark, 0.8)
draw_branch({{586,110},{596,92},{594,74}}, {3,2.6,2.4}, bark, 0.8)

--@ chunk 35
r = rag{width=14}
local stubzone = (ribbon({{528,444},{540,438},{552,428},{558,420}}, 18) - trunk2:grow(1))
r:wipe(stubzone, {pressure=0.7, angle=-0.6, passes=2, refold=0.3})
print(drying(545,432))

--@ chunk 36
r:refold(); r:dip(0.6)
local stubzone = (ribbon({{530,446},{540,438},{552,428},{560,418}}, 20) - trunk2:grow(1.5))
r:wipe(stubzone, {pressure=0.8, angle=-0.6, passes=2, refold=0.2})

--@ chunk 37
patch = mix{{s_glow,0.55},{s_pale,0.45}, name="patch"}

--@ chunk 38
local limbE = ribbon(L_E.pts, L_E.w):grow(1.5)
local zone = (ribbon({{528,448},{540,438},{552,428},{566,414}}, 26):soften(3) - trunk2:grow(1) - limbE):soften(0.5)
work(zone, {hand="body", tool="filbert 4", pile=patch, coverage=3, clip=true, fill=true, angle=0, load=0.9})
blend(zone:shrink(1), {angle=0, tool="badger 10"})

--@ chunk 39
r2 = rag{width=9}
r2:dip(0.5)
local paths = {
 {{392,240},{378,214},{352,196},{344,170}},
 {{470,186},{452,166},{456,140},{442,118}},
 {{612,244},{628,218},{652,204},{660,178}},
 {{534,160},{556,154},{566,128},{586,110},{596,92},{594,74}},
}
for _,p in ipairs(paths) do
  for k=1,3 do r2:wipe(p, {pressure=0.75}); r2:refold() end
  r2:dip(0.4)
end

--@ chunk 40
r3 = rag{width=12}
r3:dip(0.8)
local paths = {
 {{398,280},{388,262},{392,240},{378,214},{352,196},{344,170}},
 {{503,238},{478,214},{470,186},{452,166},{456,140},{442,118}},
 {{612,290},{622,268},{612,244},{628,218},{652,204},{660,178}},
 {{514,180},{534,160},{556,154},{566,128},{586,110},{596,92},{594,74}},
}
for _,p in ipairs(paths) do
  for k=1,4 do r3:wipe(p, {pressure=0.9}); r3:refold() end
  r3:dip(0.6)
end
print(drying(380,220), drying(300,120))

--@ chunk 41
local paths = {
 {{398,280},{388,262},{392,240},{378,214},{352,196},{344,170}},
 {{503,238},{478,214},{470,186},{452,166},{456,140},{442,118}},
 {{612,290},{622,268},{612,244},{628,218},{652,204},{660,178}},
 {{514,180},{534,160},{556,154},{566,128},{586,110},{596,92},{594,74}},
}
wiped = nil
for _,p in ipairs(paths) do local m = ribbon(p, 16); wiped = wiped and (wiped + m) or m end
local function bump(y, c, r) local t = (y - c) / r; return math.max(0, 1 - t*t) end
work(wiped, {hand="body", tool="filbert 5", coverage=2.5, clip=true, fill=true, angle=0, load=0.7,
  piles={{s_pale, function(x,y) return bump(y, 330, 90) end},
         {s_mid,  function(x,y) return bump(y, 230, 100) end},
         {s_top,  function(x,y) return bump(y, 110, 100) end},
         {s_zen,  function(x,y) return bump(y, 40, 90) end}}})

--@ chunk 42
local function bump(y, c, r) local t = (y - c) / r; return math.max(0, 1 - t*t) end
local tb = brush{kind="filbert", width=6, lay=1.6}
work(wiped, {hand="body", tool=tb, coverage=3.5, clip=true, fill=true, angle=0, load=1.0, pressure={0.6,0.8},
  piles={{s_pale, function(x,y) return bump(y, 340, 70) end},
         {s_mid,  function(x,y) return bump(y, 270, 70) end},
         {s_top,  function(x,y) return bump(y, 170, 90) end},
         {s_zen,  function(x,y) return bump(y, 70, 80) end}}})

--@ chunk 43
sky_a = mix{{s_top,0.55},{s_zen,0.45}, name="sky_a"}
sky_b = mix{{s_mid,0.5},{s_top,0.5}, name="sky_b"}
sky_c = mix{{s_pale,0.5},{s_mid,0.5}, name="sky_c"}

--@ chunk 44
local function bump(y, c, r) local t = (y - c) / r; return math.max(0, 1 - t*t) end
local tb = brush{kind="filbert", width=6, lay=1.2}
work(wiped, {hand="body", tool=tb, coverage=3, clip=true, fill=true, angle=0, load=1.0, pressure={0.6,0.8},
  piles={{sky_c, function(x,y) return bump(y, 330, 70) end},
         {sky_b,  function(x,y) return bump(y, 250, 70) end},
         {sky_a,  function(x,y) return bump(y, 160, 80) end},
         {s_zen,  function(x,y) return bump(y, 70, 70) end}}})
blend(wiped, {tool="badger 10", angle=0})

--@ chunk 45
print(wait(3*24*60))

--@ chunk 46
gz_cool = pile{{"smalt",1},{"raw umber",0.5},{"bone black",0.1},{"lead white",0.3}, medium=0.8, name="gz cool"}
gz_warm = pile{{"raw umber",0.6},{"yellow ochre",0.4},{"lead white",0.6},{"smalt",0.2}, medium=0.8, name="gz warm"}
local m = ribbon({{534,160},{556,154},{566,128},{586,110},{596,92},{594,74}}, 17):soften(2)
work(m, {hand="glaze", pile=gz_cool, tool={kind="filbert", width=8, stiffness=0.2}, coverage=1.2, clip=true, load=0.4, pressure={0.2,0.35}})

--@ chunk 47
local m = ribbon({{534,160},{556,154},{566,128},{586,110},{596,92},{594,74}}, 18):soften(2)
blend(m, {tool="badger 12", angle=-0.8})
blend(m, {tool="badger 12", angle=0.6})

--@ chunk 48
g_hi = pile{{"lead white",2.2},{"smalt",2.4},{"cobalt blue",1.1},{"bone black",0.12},{"rose madder",0.16}, medium=0.55, name="g hi"}
g_md = pile{{"lead white",3},{"smalt",2.0},{"cobalt blue",0.8},{"bone black",0.08},{"rose madder",0.12}, medium=0.55, name="g md"}
g_lo = pile{{"lead white",4},{"smalt",1.2},{"cobalt blue",0.35},{"Naples yellow",0.2}, medium=0.55, name="g lo"}
local function bump(y, c, r) local t = (y - c) / r; return math.max(0, 1 - t*t) end
local m = wiped:soften(2)
work(m, {hand="body", tool={kind="filbert", width=8, stiffness=0.25}, coverage=1.5, clip=true, angle=0, load=0.5, pressure={0.3,0.45},
  piles={{g_lo, function(x,y) return bump(y, 340, 70) end},
         {g_md, function(x,y) return bump(y, 240, 80) end},
         {g_hi, function(x,y) return bump(y, 110, 110) end}}})
blend(m, {tool="badger 12", angle=0})

--@ chunk 49
print(drying(640,110), drying(380,160))
dk_hi = pile{{"lead white",2},{"smalt",2.6},{"cobalt blue",1.3},{"bone black",0.22},{"rose madder",0.12},{"raw umber",0.08}, name="dk hi"}
local m = ribbon({{566,128},{586,110},{596,92},{594,74}}, 18):soften(2)
work(m, {hand="body", tool={kind="filbert", width=7, stiffness=0.3}, pile=dk_hi, coverage=1.5, clip=true, angle=0, load=0.5, pressure={0.4,0.55}})
blend(m, {tool="badger 12", angle=0})

--@ chunk 50
local m = ribbon({{566,128},{586,110},{596,92},{594,74}}, 17)
work(m, {hand="body", tool={kind="filbert", width=6, stiffness=0.3}, pile=dk_hi, coverage=2.5, clip=true, fill=true, angle=0, load=0.8, pressure={0.5,0.7}})

--@ chunk 51
dk2 = pile{{"lead white",1.6},{"smalt",3},{"cobalt blue",1.5},{"bone black",0.3},{"rose madder",0.15},{"raw umber",0.1}, name="dk2"}
local m = ribbon({{566,128},{586,110},{596,92},{594,74}}, 17)
work(m, {hand="body", tool={kind="filbert", width=6, stiffness=0.3, lay=2}, pile=dk2, coverage=3, clip=true, fill=true, angle=0, load=1, pressure={0.6,0.8}})

--@ chunk 52
blend(ribbon({{566,128},{586,110},{596,92},{594,74}}, 22):soften(3), {tool="badger 12", angle=0, clip=false})
up1 = mix{{dk2,0.45},{sky_a,0.55}, name="up1"}
up2 = mix{{sky_a,0.6},{sky_b,0.4}, name="up2"}

--@ chunk 53
print(drying(503,120), drying(470,130), drying(600,115), drying(530,122))

--@ chunk 54
r4 = rag{width=12}
r4:dip(0.7)
local p1 = {{486,131},{510,126},{535,120},{562,114}}
local p2 = {{560,114},{600,112},{640,112},{680,113}}
for k=1,4 do r4:wipe(p1,{pressure=0.8}); r4:refold(); r4:wipe(p2,{pressure=0.8}); r4:refold() end

--@ chunk 55
for _,p in ipairs({{520,124},{560,116},{620,112},{660,112},{620,140},{620,90}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 56
r5 = rag{width=10}
local p1 = {{484,131},{510,126},{535,120},{562,115}}
local p2 = {{556,116},{600,113},{640,112},{684,113}}
local p3 = {{484,124},{520,119},{560,111}}
for k=1,5 do r5:dip(0.8); r5:wipe(p1,{pressure=0.9}); r5:refold(); r5:wipe(p2,{pressure=0.9}); r5:refold(); r5:wipe(p3,{pressure=0.9}); r5:refold() end

--@ chunk 57
print(wait(4*24*60))

--@ chunk 58
local paths = {
 {{398,280},{388,262},{392,240},{378,214},{352,196},{344,170}},
 {{503,238},{478,214},{470,186},{452,166},{456,140},{442,118}},
 {{612,290},{622,268},{612,244},{628,218},{652,204},{660,178}},
 {{514,180},{534,160},{556,154},{566,128},{586,110},{596,92},{594,74}},
 {{482,128},{520,121},{560,114},{620,112},{690,113}},
}
scar = nil
for i,p in ipairs(paths) do local m = ribbon(p, i==5 and 20 or 21); scar = scar and (scar + m) or m end
scar = scar:soften(1.5)
local function wA(x,y) return clamp((170-y)/60,0,1) end
local function wC(x,y) return clamp((y-240)/70,0,1) end
local function wB(x,y) return math.max(0, 1 - wA(x,y) - wC(x,y)) end
work(scar, {hand="body", tool={kind="filbert", width=6, stiffness=0.35, lay=1.4}, coverage=3.5, clip=true, fill=true, angle=0, load=1, pressure={0.6,0.8},
  piles={{sky_a, wA},{sky_b, wB},{sky_c, wC}}})
blend(scar, {tool="badger 10", angle=0})

--@ chunk 59
r6 = rag{width=16}
for k=1,3 do r6:dip(0.8); r6:wipe(scar, {pressure=0.85, angle=0, refold=0.2, passes=1}) end
r6 = rag{width=16}
for k=1,2 do r6:dip(0.9); r6:wipe(scar, {pressure=0.95, angle=0.5, refold=0.15, passes=1}) end

--@ chunk 60
print(wait(6*24*60))
print(drying(500,200), drying(330,200), drying(640,120))

--@ chunk 61
print(wait(3*24*60))
print(drying(500,200), drying(330,200), drying(640,120), drying(600,112))

--@ chunk 62
local function bump(y, c, r) local t = (y - c) / r; return math.max(0, 1 - t*t) end
band = (rect(0,0,1000,330) + rect(0,330,1000,14):soften(6)):soften(4)
work(band, {hand="broad", angle=0, coverage=2.8, angle_jitter=0.02, curve={0.02,0.0}, fill=true, load=1.0, pressure={0.5,0.8},
  piles={
    {s_pale, function(x,y) return bump(y, 340, 70) end},
    {s_mid,  function(x,y) return bump(y, 250, 90) end},
    {s_top,  function(x,y) return bump(y, 130, 100) end},
    {s_zen,  function(x,y) return y < 30 and 1 or bump(y, 30, 100) end}}})
blend(band, {angle=0})

--@ chunk 63
local function bump(y, c, r) local t = (y - c) / r; return math.max(0, 1 - t*t) end
local spots = ellipse(304,148,14,11) + ellipse(485,107,12,11) + ellipse(450,205,18,13) + ellipse(611,203,12,11)
spots = spots:soften(4)
for k=1,2 do
work(spots, {hand="body", tool="filbert 6", coverage=3, fill=true, angle=0, load=0.9, pressure={0.5,0.7},
  piles={
    {s_pale, function(x,y) return bump(y, 340, 70) end},
    {s_mid,  function(x,y) return bump(y, 250, 90) end},
    {s_top,  function(x,y) return bump(y, 130, 100) end},
    {s_zen,  function(x,y) return y < 30 and 1 or bump(y, 30, 100) end}}})
end
blend(spots:grow(6):soften(4), {angle=0, tool="badger 14"})

--@ chunk 64
print(wait(5*24*60))
print(drying(500,200), drying(450,205), drying(300,100), drying(500,330))

--@ chunk 65
print(wait(4*24*60))
print(drying(500,200), drying(450,205), drying(300,100), drying(500,330), drying(304,148))

--@ chunk 66
print(wait(7*24*60))
print(drying(450,205), drying(304,148), drying(485,107), drying(611,203))

--@ chunk 67
print(wait(5*24*60))
print(drying(304,148), drying(485,107))

--@ chunk 68
NA = {pts={{486,352},{470,322},{454,298},{440,272},{418,250},{396,236},{372,214},{350,200},{318,190},{288,188}}, w={24,20,17,15,13,11.5,10,8.5,7,5}}
NB = {pts={{502,348},{508,316},{500,288},{506,258},{520,232},{514,204},{504,178},{510,150},{503,124},{498,102}}, w={24,21,18,16,14,12,10.5,9,7.5,6.5}}
NC = {pts={{516,352},{536,330},{552,306},{576,288},{600,268},{618,246},{644,232},{672,224},{702,212},{730,206}}, w={23,19,16.5,15,13,11.5,10,8.5,7,5}}
local m = ribbon(NA.pts,NA.w) + ribbon(NB.pts,NB.w) + ribbon(NC.pts,NC.w)
-- collars at the crotch, and a join over the old flat top of the trunk
m = m + poly({{468,370},{474,340},{490,326},{514,326},{530,338},{534,370}}, true)
m = m:roughen(1.0, 7, 41)
for _,L in ipairs({NA,NB,NC}) do
  local a,b = L.pts[1], L.pts[#L.pts]
  local lm = ribbon(L.pts, L.w):roughen(1.0,7,41)
  work(lm, {hand="body", tool="filbert 5", pile=bark, angle=math.atan(b[2]-a[2], b[1]-a[1]), angle_jitter=0.25, coverage=2.5, clip=true, fill=true})
end
work(poly({{468,370},{474,340},{490,326},{514,326},{530,338},{534,370}}, true):roughen(1,7,42), {hand="body", tool="filbert 5", pile=bark, angle=math.pi/2, angle_jitter=0.4, coverage=2.5, clip=true, fill=true})

--@ chunk 69
local fl = poly({{466,352},{470,372},{474,392},{478,412},{481,432},{528,432},{526,412},{527,392},{532,372},{538,352},{520,346},{490,346}}, true):roughen(1.0,7,43)
work(fl, {hand="body", tool="filbert 5", pile=bark, angle=math.pi/2, angle_jitter=0.2, coverage=2.5, clip=true, fill=true})

--@ chunk 70
SECS = {
 -- from A
 {pts={{440,272},{432,244},{438,216},{424,190},{428,164},{418,142}}, w={10,8.5,7,5.5,4.2,3}},
 {pts={{372,214},{358,192},{362,168},{346,150},{338,128}}, w={8.5,7,5.6,4.2,3}},
 {pts={{418,252},{392,262},{362,268},{332,266},{302,276},{272,280},{250,292}}, w={9,8,6.8,5.6,4.6,3.6,2.6}},
 {pts={{318,190},{296,170},{290,146},{272,132}}, w={6.5,5.2,4,3}},
 -- from C
 {pts={{600,268},{606,238},{598,212},{610,186},{604,162},{612,140}}, w={10,8.5,7,5.5,4.2,3}},
 {pts={{644,232},{656,206},{650,184},{664,162},{660,140}}, w={8.5,7,5.6,4.4,3.2}},
 {pts={{578,288},{608,296},{640,300},{672,296},{704,304},{734,304},{752,316}}, w={9,8,6.8,5.6,4.6,3.6,2.6}},
 {pts={{702,212},{722,190},{718,168},{734,150}}, w={6.5,5.2,4,3}},
 -- from B
 {pts={{506,260},{486,238},{472,214},{466,188},{474,164},{462,140},{466,118}}, w={10,8.5,7,5.8,4.6,3.6,2.8}},
 {pts={{516,206},{536,190},{550,170},{566,152},{562,128},{574,110}}, w={9,7.5,6.2,5,3.8,2.8}},
}
for _,L in ipairs(SECS) do draw_branch(L.pts, L.w, bark, 0.95) end
-- stag-head of the leader: dead, bare, broken
draw_branch({{498,104},{494,84},{499,66},{495,50}}, {6,5,4.4,4}, bark, 0.9)
draw_branch({{499,80},{510,70},{514,58}}, {3.2,2.8,2.6}, bark, 0.8)
-- broken stub off C
draw_branch({{552,306},{562,286},{570,272}}, {8,7.5,7}, bark, 0.9)

--@ chunk 71
print(drying(726,170), drying(740,160))
rt = rag{width=8}
rt:dip(0.8)
local p = {{704,210},{722,190},{718,168},{734,150},{738,142}}
for k=1,4 do rt:wipe(p, {pressure=0.85}); rt:refold(); if k==2 then rt:dip(0.6) end end

--@ chunk 72
draw_branch({{702,212},{714,198},{723,186},{721,170},{730,156},{737,145}}, {8,7.2,6.4,5.4,4.6,3.6}, bark, 1.0)
draw_branch({{702,212},{714,198},{723,186},{721,170},{730,156},{737,145}}, {7,6.4,5.6,4.8,4,3.2}, bark, 1.0)

--@ chunk 73
function envelope(x,y)
  local dx=(x-500)/335; local dy=(y-255)/(y<255 and 215 or 175)
  return dx*dx+dy*dy
end
function tree_rec2(x,y,ang,len,w,depth)
  if w < 0.45 or depth > 5 or len < 7 then return end
  local nseg = math.max(2, math.floor(len/11))
  local pts, ws, aend = grow(x,y,ang,len,w,math.max(0.4,w*0.5),nseg, 0.42, 0.08)
  local cut = #pts
  for i=2,#pts do if envelope(pts[i][1],pts[i][2]) > 1.0 + rand(-0.08,0.1) then cut=i break end end
  local P,Wd = {},{}
  for i=1,cut do P[i]=pts[i]; Wd[i]=ws[i] end
  if #P < 2 then return end
  draw_branch(P, Wd, (w < 1.6) and twig or bark, 0.7)
  local nch = math.max(1, math.floor(#P*0.7))
  for k=1,nch do
    local i = math.random(2, #P)
    local px,py = P[i][1],P[i][2]
    local pa = (i < #P) and math.atan(P[i+1][2]-py, P[i+1][1]-px) or aend
    local side = (math.random() < 0.5) and -1 or 1
    tree_rec2(px,py, pa + side*rand(0.5, 1.1), len*rand(0.45,0.68), Wd[i]*rand(0.5,0.72), depth+1)
  end
end
-- spawn from the tips and along the secondaries
for _,L in ipairs(SECS) do
  local n=#L.pts
  local a,b = L.pts[n-1], L.pts[n]
  tree_rec2(b[1],b[2], math.atan(b[2]-a[2],b[1]-a[1]), 60, L.w[n]*0.9, 1)
  for i=2,n-1,2 do
    local p, q = L.pts[i], L.pts[i+1]
    local pa = math.atan(q[2]-p[2], q[1]-p[1])
    local side = (i%4==0) and -1 or 1
    tree_rec2(p[1],p[2], pa + side*rand(0.6,1.0), 55, L.w[i]*0.55, 2)
  end
end

--@ chunk 74
local function spawn_along(L, every, len, wf, depth)
  local n=#L.pts
  for i=2,n-1,every do
    local p, q = L.pts[i], L.pts[i+1]
    local pa = math.atan(q[2]-p[2], q[1]-p[1])
    local side = (math.random()<0.5) and -1 or 1
    tree_rec2(p[1],p[2], pa + side*rand(0.5,1.1), len, L.w[i]*wf, depth)
  end
  local a,b = L.pts[n-1], L.pts[n]
  tree_rec2(b[1],b[2], math.atan(b[2]-a[2],b[1]-a[1]), len*1.1, L.w[n]*0.85, depth-1)
end
for _,L in ipairs({L_D, L_E}) do spawn_along(L, 1, 48, 0.45, 2) end
for _,L in ipairs({NA, NC}) do spawn_along(L, 2, 42, 0.35, 3) end
for _,L in ipairs(SECS) do spawn_along(L, 1, 36, 0.4, 3) end

--@ chunk 75
local pts = {{468,428},{466,446},{463,466},{460,484},{454,498},{444,508},{428,515},{436,519},{470,518},{500,520},{530,518},{560,519},{578,516},{562,509},{550,498},{543,482},{539,462},{537,444},{536,428}}
trunk3 = poly(pts, true):roughen(1.4, 9, 51)
work(trunk3, {hand="body", tool="filbert 6", pile=bark, angle=math.pi/2, angle_jitter=0.15, coverage=3, clip=true, fill=true, load=1})
-- roots running into the snow, drawn
draw_branch({{450,506},{432,512},{414,516},{398,518}}, {9,6,3.5,1.5}, bark, 0.9)
draw_branch({{555,506},{574,512},{592,516},{606,517}}, {9,6,3.5,1.5}, bark, 0.9)

--@ chunk 76
local lft = poly({{474,400},{468,416},{464,432},{464,448},{470,452},{478,430},{480,404}}, true)
local rgt = poly({{530,400},{536,414},{542,430},{541,448},{534,452},{528,430},{526,404}}, true)
work((lft+rgt):roughen(1,7,52), {hand="body", tool="filbert 4", pile=bark, angle=math.pi/2, angle_jitter=0.2, coverage=3, clip=true, fill=true})

--@ chunk 77
rgf = brush{kind="rigger", width=1.1, point=1, stiffness=0.4}
local s=""
for _,p in ipairs({0.05,0.2,0.4,0.6,0.8,1}) do s=s..string.format("%.2f->%.2f  ",p,rgf:mark_width(p)) end
print(s)
twig2 = pile{{"bone black",1},{"raw umber",1.0},{"smalt",0.8},{"lead white",0.9},{"rose madder",0.05}, medium=0.3, name="twig2"}

--@ chunk 78
function sprig(x,y,ang,len,p0,depth)
  if depth > 3 or len < 4 then return end
  local nseg = math.max(2, math.floor(len/6))
  local pts = {}
  local a = ang
  local P = {{x,y,p0}}
  for i=1,nseg do
    a = a + rand(-0.45,0.45)
    local diff = math.atan(math.sin(-math.pi/2-a), math.cos(-math.pi/2-a)); a = a + diff*0.1
    x = x + math.cos(a)*len/nseg; y = y + math.sin(a)*len/nseg
    if envelope(x,y) > 1.12 then break end
    P[#P+1] = {x,y, lerp(p0, 0.05, i/nseg)}
  end
  if #P < 2 then return end
  if rgf:fullness() < 0.25 then rgf:load(twig2, 0.6) end
  rgf:gesture(P, {wobble=0.3})
  for k=2,#P do
    if math.random() < 0.55 then
      local q = P[k]
      local side = (math.random()<0.5) and -1 or 1
      sprig(q[1],q[2], a + side*rand(0.5,1.1), len*rand(0.4,0.65), q[3]*0.95, depth+1)
    end
  end
end
rgf:load(twig2, 0.6)
-- seeds: points along every limb list, sprigs aimed outward/upward
local count=0
for _,L in ipairs({NA,NB,NC,L_D,L_E, table.unpack(SECS)}) do
  local n=#L.pts
  for i=2,n do
    local p, q = L.pts[i-1], L.pts[i]
    local pa = math.atan(q[2]-p[2], q[1]-p[1])
    for k=1,2 do
      local t = math.random()
      local x,y = lerp(p[1],q[1],t), lerp(p[2],q[2],t)
      local side = (math.random()<0.5) and -1 or 1
      sprig(x,y, pa + side*rand(0.4,1.2), rand(14,30), 0.85, 1)
      count=count+1
    end
  end
end
print(count)

--@ chunk 79
twig2:add{{"bone black",0.3},{"raw umber",0.2}}
local n=0
local tries=0
while n < 220 and tries < 5000 do
  tries = tries + 1
  local x, y = rand(160, 840), rand(40, 400)
  local e = envelope(x,y)
  if e > 0.62 and e < 0.92 and y < 330 then
    local ra = math.atan((y-255)*1.6, x-500)  -- radial outward-ish
    local a = ra*0.6 + (-math.pi/2)*0.4
    if x < 500 and a > 0 then a = a - 2*math.pi end
    sprig(x,y, ra + rand(-0.5,0.5) - 0.2*math.cos(ra), rand(18,34), 0.95, 1)
    n = n + 1
  end
end
print(n, tries)

--@ chunk 80
for _,p in ipairs({{815,230},{830,180},{225,150},{200,190},{790,290}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 81
rw = rag{width=14}
rw:dip(0.6)
local zA = ellipse(822, 215, 30, 70):roughen(8, 30, 61)
local zB = ellipse(222, 150, 34, 30):roughen(6, 25, 62)
local zC = ellipse(195, 250, 22, 28):roughen(6, 25, 63)
rw:wipe(zA, {pressure=0.6, angle=0.3, passes=2, refold=0.25})
rw:refold(); rw:dip(0.5)
rw:wipe(zB, {pressure=0.6, angle=-0.4, passes=2, refold=0.25})
rw:refold(); rw:dip(0.5)
rw:wipe(zC, {pressure=0.6, angle=0.2, passes=2, refold=0.25})

--@ chunk 82
rw = rag{width=12}
local zA = (ellipse(818, 225, 32, 85) + ellipse(790, 140, 22, 22)):roughen(8, 30, 64)
local zB = (ellipse(205, 160, 40, 32) + ellipse(178, 215, 22, 30)):roughen(6, 25, 65)
local zD = ellipse(560, 42, 30, 16):roughen(5,20,66)
for _,z in ipairs({zA, zB, zD}) do
  for k=1,3 do rw:dip(0.8); rw:wipe(z, {pressure=0.85, angle=0.3*k, passes=1, refold=0.15}); rw:refold() end
end

--@ chunk 83
local keep = nil
for _,L in ipairs({NA,NB,NC, table.unpack(SECS)}) do
  local w = {}
  for i,v in ipairs(L.w) do w[i] = v + 6 end
  local m = ribbon(L.pts, w); keep = keep and (keep + m) or m
end
keep = keep + ribbon({{498,104},{494,84},{499,66},{495,50},{494,40}}, 12) + ribbon({{702,212},{714,198},{723,186},{721,170},{730,156},{737,145}}, 14)
ring = mask(function(x,y) local e = envelope(x,y); return (e > 0.5 and e < 1.3 and y < 345) and 1 or 0 end)
ring = (ring - keep):soften(2)
rr = rag{width=12}
for k=1,3 do rr:dip(0.8); rr:wipe(ring, {pressure=0.8, angle=0.4*k, passes=1, refold=0.12}); rr = rag{width=12} end

--@ chunk 84
for k=1,4 do
  local rr = rag{width=10}
  rr:dip(0.9)
  rr:wipe(ring, {pressure=0.9, angle=0.6*k+0.2, passes=1, refold=0.06})
end

--@ chunk 85
print(wait(8*24*60))
for _,p in ipairs({{300,150},{250,250},{700,150},{760,300},{500,60},{420,200}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 86
local function bump(y, c, r) local t = (y - c) / r; return math.max(0, 1 - t*t) end
structure = ribbon(NA.pts, NA.w) + ribbon(NB.pts, NB.w) + ribbon(NC.pts, NC.w)
  + ribbon({{498,104},{494,84},{499,66},{495,50}}, {6,5,4.4,4})
  + poly({{460,380},{468,340},{490,322},{516,322},{536,340},{544,380}}, true)
  + ribbon(L_D.pts, L_D.w) + ribbon(L_E.pts, L_E.w)
structure = structure:roughen(1.0, 7, 41):grow(0.6)
band2 = ((rect(0,0,1000,325) + rect(0,325,1000,16):soften(7)):soften(3) - structure)
work(band2, {hand="broad", angle=0, coverage=2.8, angle_jitter=0.02, curve={0.02,0.0}, fill=true, load=1.0, pressure={0.5,0.8}, clip=true,
  piles={
    {s_pale, function(x,y) return bump(y, 340, 70) end},
    {s_mid,  function(x,y) return bump(y, 250, 90) end},
    {s_top,  function(x,y) return bump(y, 130, 100) end},
    {s_zen,  function(x,y) return y < 30 and 1 or bump(y, 30, 100) end}}})
blend(band2, {angle=0})

--@ chunk 87
print(wait(7*24*60))
for _,p in ipairs({{300,150},{250,250},{700,150},{600,250},{500,30},{420,200}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 88
print(wait(4*24*60))
local function bump(y, c, r) local t = (y - c) / r; return math.max(0, 1 - t*t) end
local band3 = ((rect(0,0,1000,325) + rect(0,325,1000,14):soften(7)):soften(3) - structure)
work(band3, {hand="broad", angle=0, coverage=3, angle_jitter=0.02, curve={0.02,0.0}, fill=true, load=1.0, pressure={0.55,0.85}, clip=true,
  piles={
    {s_pale, function(x,y) return bump(y, 340, 70) end},
    {s_mid,  function(x,y) return bump(y, 250, 90) end},
    {s_top,  function(x,y) return bump(y, 130, 100) end},
    {s_zen,  function(x,y) return y < 30 and 1 or bump(y, 30, 100) end}}})
blend(band3, {angle=0})

--@ chunk 89
function densify(pts, ws, step)
  local P, Wd = {}, {}
  for i=1,#pts-1 do
    local a,b = pts[i], pts[i+1]
    local d = math.sqrt((b[1]-a[1])^2+(b[2]-a[2])^2)
    local n = math.max(1, math.floor(d/step))
    for k=0,n-1 do local t=k/n; P[#P+1]={lerp(a[1],b[1],t), lerp(a[2],b[2],t)}; Wd[#Wd+1]=lerp(ws[i],ws[i+1],t) end
  end
  P[#P+1]=pts[#pts]; Wd[#Wd+1]=ws[#ws]
  return P, Wd
end
function ang_branch(pts, w0, w1, pile, load)
  local n=#pts; local ws={}
  for i=1,n do ws[i]=lerp(w0,w1,((i-1)/(n-1))^0.8) end
  local P,Wd = densify(pts, ws, 5)
  draw_branch(P, Wd, pile or bark, load or 1.0)
  return pts, ws
end
CROWN = {
 -- left limb NA
 {{{418,252},{404,222},{405,192},{390,166},{393,138},{380,112}}, 7, 2.4},
 {{{374,214},{346,222},{318,226},{296,240},{268,244},{238,257},{210,262}}, 7, 2.2},
 {{{289,188},{266,182},{248,170},{241,147},{224,135},{204,126}}, 5.2, 2.2},
 {{{249,171},{226,184},{203,190},{182,205}}, 3.6, 1.8},
 {{{442,274},{450,244},{443,216},{450,196}}, 5, 2.2},
 -- right limb NC
 {{{600,268},{594,236},{604,206},{598,180},{610,152},{604,124}}, 7, 2.4},
 {{{644,232},{670,244},{700,248},{726,262},{756,266},{788,281}}, 7, 2.2},
 {{{729,206},{752,196},{768,179},{775,157},{791,145},{806,132}}, 5.2, 2.2},
 {{{768,180},{792,188},{812,200},{830,198}}, 3.6, 1.8},
 {{{576,290},{583,260},{576,236},{584,214}}, 5, 2.2},
 -- leader NB
 {{{506,258},{488,232},{470,210},{474,182},{460,160},{463,132},{452,110}}, 8, 2.4},
 {{{514,206},{530,186},{548,170},{546,146},{560,126},{556,100},{566,82}}, 7, 2.2},
 {{{505,176},{488,160},{476,138}}, 4.2, 2},
}
for _,c in ipairs(CROWN) do ang_branch(c[1], c[2], c[3]) end

--@ chunk 90
print(wait(12*24*60))
for _,p in ipairs({{300,150},{250,250},{700,150},{600,250},{500,30},{420,200},{560,120}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 91
local function bump(y, c, r) local t = (y - c) / r; return math.max(0, 1 - t*t) end
local band4 = ((rect(0,0,1000,325) + rect(0,325,1000,14):soften(7)):soften(3) - structure)
work(band4, {hand="broad", angle=0, coverage=3, angle_jitter=0.02, curve={0.02,0.0}, fill=true, load=1.0, pressure={0.55,0.85}, clip=true,
  piles={
    {s_pale, function(x,y) return bump(y, 340, 70) end},
    {s_mid,  function(x,y) return bump(y, 250, 90) end},
    {s_top,  function(x,y) return bump(y, 130, 100) end},
    {s_zen,  function(x,y) return y < 30 and 1 or bump(y, 30, 100) end}}})
blend(band4, {angle=0})

--@ chunk 92
print(wait(14*24*60))
for _,p in ipairs({{300,150},{250,250},{700,150},{600,250},{500,30},{420,200},{560,120},{100,300}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 93
function envelope(x,y)
  local dx=(x-500)/325; local dy=(y-250)/(y<250 and 212 or 160)
  return dx*dx+dy*dy
end
function oak_path(x,y,ang,len,up)
  local pts = {{x,y}}
  local a = ang
  local done = 0
  local elbows = {}
  while done < len do
    local sl = rand(8,18)
    if math.random() < 0.28 then a = a + (math.random()<0.5 and -1 or 1)*rand(0.35,0.75); elbows[#pts]=true
    else a = a + rand(-0.12,0.12) end
    local diff = math.atan(math.sin(-math.pi/2-a), math.cos(-math.pi/2-a)); a = a + diff*up
    x = x + math.cos(a)*sl; y = y + math.sin(a)*sl
    pts[#pts+1] = {x,y}
    done = done + sl
    if envelope(x,y) > 1.0 + rand(-0.06,0.08) then break end
  end
  return pts, elbows, a
end
function oak(x,y,ang,len,w,depth)
  if w < 0.4 or len < 5 or depth > 6 then return end
  local pts, elbows, aend = oak_path(x,y,ang,len, depth<=2 and 0.07 or 0.04)
  if #pts < 2 then return end
  local n=#pts
  local ws = {}
  local wend = math.max(0.35, w*0.42)
  for i=1,n do ws[i] = lerp(w, wend, ((i-1)/(n-1))^0.9) end
  local P,Wd = densify(pts, ws, 4)
  draw_branch(P, Wd, (w < 1.5) and twig2 or bark, 0.9)
  -- children: at elbows mostly, plus a few
  for i=2,n do
    local pc = elbows[i] and 0.85 or 0.35
    if math.random() < pc then
      local px,py = pts[i][1],pts[i][2]
      local pa = (i<n) and math.atan(pts[i+1][2]-py, pts[i+1][1]-px) or aend
      local side = (math.random()<0.5) and -1 or 1
      oak(px,py, pa + side*rand(0.45,1.05), len*rand(0.38,0.6), ws[i]*rand(0.5,0.7), depth+1)
      if elbows[i] and math.random()<0.4 then
        oak(px,py, pa - side*rand(0.4,0.9), len*rand(0.3,0.5), ws[i]*rand(0.45,0.6), depth+1)
      end
    end
  end
  -- terminal cluster
  oak(pts[n][1],pts[n][2], aend + rand(-0.4,0.4), len*0.45, wend*0.9, depth+1)
end
-- test: secondaries off the left limb NA
oak(440,272, -1.75, 140, 7.5, 1)
oak(396,236, -2.05, 130, 6.5, 1)
oak(372,214, 2.75, 130, 6.5, 1)

--@ chunk 94
-- NA tip and upper
oak(318,190, -2.6, 120, 6, 1)
oak(350,200, -1.9, 120, 6.5, 1)
oak(289,188, 2.9, 90, 5, 2)
-- NB leader, both sides
oak(500,288, -2.3, 130, 7.5, 1)
oak(506,258, -1.1, 130, 7.5, 1)
oak(514,204, -0.9, 120, 6.5, 1)
oak(504,178, -2.2, 110, 6, 1)
oak(503,124, -1.2, 70, 4, 2)
-- NC
oak(576,288, -1.45, 140, 7.5, 1)
oak(618,246, -1.25, 130, 6.5, 1)
oak(644,232, 0.25, 130, 6.5, 1)
oak(702,212, -0.95, 120, 6, 1)
oak(730,206, -0.3, 100, 5, 1)

--@ chunk 95
SEG = {}
TIPS = {}
local _draw = draw_branch
function draw_branch(pts, ws, pile, load)
  _draw(pts, ws, pile, load)
  for i=1,#pts-1 do SEG[#SEG+1] = {pts[i][1],pts[i][2],pts[i+1][1],pts[i+1][2], ws[i]} end
  TIPS[#TIPS+1] = {pts[#pts][1], pts[#pts][2], math.atan(pts[#pts][2]-pts[#pts-1][2], pts[#pts][1]-pts[#pts-1][1])}
end
oak(456,300, 2.95, 130, 6, 1)
oak(470,322, 2.7, 90, 4.5, 2)
oak(548,312, 0.2, 130, 6, 1)
oak(530,334, 0.45, 90, 4.5, 2)
oak(600,268, -1.35, 150, 7, 1)
oak(470,210, -2.0, 120, 5.5, 1)
print(#SEG, #TIPS)

--@ chunk 96
rgh = brush{kind="rigger", width=0.8, point=1, stiffness=0.4}
local s=""
for _,p in ipairs({0.1,0.4,0.6,0.8,1}) do s=s..string.format("%.2f->%.2f  ",p,rgh:mark_width(p)) end
print(s)
twig3 = pile{{"bone black",1.2},{"raw umber",1.0},{"smalt",0.6},{"lead white",0.35}, medium=0.3, name="twig3"}

--@ chunk 97
function hair(x,y,ang,len,p0,depth)
  if depth > 3 or len < 3 then return end
  local nseg = math.max(2, math.floor(len/4))
  local a = ang
  local P = {{x,y,p0}}
  for i=1,nseg do
    a = a + rand(-0.35,0.35)
    local diff = math.atan(math.sin(-math.pi/2-a), math.cos(-math.pi/2-a)); a = a + diff*0.12
    x = x + math.cos(a)*len/nseg; y = y + math.sin(a)*len/nseg
    if envelope(x,y) > 1.1 then break end
    P[#P+1] = {x,y, lerp(p0, 0.1, i/nseg)}
  end
  if #P < 2 then return end
  if rgh:fullness() < 0.3 then rgh:load(twig3, 0.7) end
  rgh:gesture(P, {wobble=0.2})
  for k=2,#P do
    if math.random() < 0.4 then
      local q = P[k]
      local side = (math.random()<0.5) and -1 or 1
      hair(q[1],q[2], a + side*rand(0.4,0.9), len*rand(0.45,0.6), q[3], depth+1)
    end
  end
end
rgh:load(twig3, 0.7)
local n=0
for _,t in ipairs(TIPS) do
  if envelope(t[1],t[2]) > 0.45 then
    hair(t[1],t[2], t[3]+rand(-0.3,0.3), rand(10,20), 0.8, 1)
    hair(t[1],t[2], t[3]+rand(-0.9,-0.4), rand(8,14), 0.7, 2)
    n=n+1
  end
end
print(n)

--@ chunk 98
print(wait(20*24*60))
for _,p in ipairs({{420,396},{500,450},{600,250},{350,200}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 99
local drift = outline{{404,522},{420,514},{436,511},{452,505},{462,501},{474,503},{486,499},{498,502},{510,500},{522,503},{534,500},{548,505},{562,509},{580,513},{604,518},{610,530},{400,530}, char="soft", amount=0.6, seed=7, closed=true}
driftm = drift:mask()
work(driftm, {hand="body", tool="filbert 4", pile=sn2_crest, angle=0, angle_jitter=0.3, coverage=3, fill=true, clip=true, load=0.9})
blend(driftm:shrink(1), {tool="badger 8", angle=0})

--@ chunk 100
for k=1,3 do local rr=rag{width=12}; rr:dip(0.7); rr:wipe(driftm:grow(2), {pressure=0.8, angle=0, passes=1, refold=0.1}) end

--@ chunk 101
print(wait(10*24*60))
print(drying(500,512), drying(450,515))

--@ chunk 102
local base = poly({{458,484},{456,494},{450,502},{438,508},{420,512},{404,514},{420,516},{460,514},{500,513},{540,514},{580,516},{598,515},{582,511},{566,507},{552,500},{546,490},{544,484}}, true):roughen(1.2,8,71)
work(base, {hand="body", tool="filbert 4", pile=bark, angle=0, angle_jitter=0.3, coverage=3, clip=true, fill=true, load=1})
-- snow lying over the root bases: soft-topped band
SN_TOP = {{360,520},{390,517},{415,515},{440,512},{462,511},{485,512},{505,510},{525,512},{548,511},{570,514},{595,516},{620,519},{650,521}}
local snowcap = (below(SN_TOP):roughen(1.2,10,72) * rect(340,500,330,48)):soften(1)
local function d(x,y) return clamp((y-510)/40,0,1) end
work(snowcap, {hand="body", tool="filbert 6", angle=0, angle_jitter=0.08, coverage=3, clip=true, fill=true, load=0.9,
  piles={{sn2_crest, function(x,y) return 1-d(x,y)*0.6 end},{sn2_mid, function(x,y) return d(x,y)*0.6 end}}})
blend(snowcap:shrink(2), {tool="badger 14", angle=0})

--@ chunk 103
local wl = poly({{464,460},{460,476},{455,488},{448,500},{440,507},{458,506},{466,480}}, true)
local wr = poly({{536,460},{540,476},{546,488},{553,500},{562,507},{544,506},{535,480}}, true)
work((wl+wr):roughen(0.8,6,73), {hand="body", tool="filbert 3", pile=bark, angle=math.pi/2, coverage=3, clip=true, fill=true, load=1})

--@ chunk 104
print(wait(12*24*60))
print(drying(500,520), drying(300,650))
gz_snow = pile{{"smalt",1},{"cobalt blue",0.3},{"raw umber",0.3},{"rose madder",0.12},{"lead white",0.15}, medium=0.75, name="gz snow"}
gz_sky = pile{{"smalt",1},{"cobalt blue",0.5},{"rose madder",0.08},{"raw umber",0.08}, medium=0.8, name="gz sky"}

--@ chunk 105
local lowm = (snowm * below(function(x) return 590 + 30*math.cos((x-500)/500*math.pi*0.5)^2 end)):soften(20)
work(lowm, {hand="glaze", pile=gz_snow, coverage=1.2, clip=true, angle=0, angle_jitter=0.03,
  load_at=function(x,y) return clamp(0.15 + (y-600)/180, 0.1, 0.6) end, pressure={0.25,0.4}})
blend(lowm, {angle=0})

--@ chunk 106
local lowm = (snowm * below(function(x) return 570 end)):soften(20)
blend(lowm, {angle=0.02})
blend(lowm, {angle=-0.02})
blend(lowm, {angle=0, tool="badger 60"})

--@ chunk 107
print(palette())

--@ chunk 108
s_glow = pile{{"lead white",6},{"Naples yellow",1.6},{"chrome yellow",0.14},{"rose madder",0.1}, name="glow"}
patch2 = mix{{s_glow,0.4},{s_pale,0.6}, name="patch2"}

--@ chunk 109
patch3 = mix{{s_pale,0.75},{s_glow,0.25}, name="patch3"}
local m = (ellipse(796,348,14,9) + ellipse(785,357,6,9)):soften(2)
work(m, {hand="body", tool="filbert 4", pile=patch3, coverage=3, fill=true, angle=0, load=0.7, pressure={0.4,0.6}, clip=true})
blend(m:grow(4):soften(3), {tool="badger 10", angle=0})

--@ chunk 110
bark_md = mix{{bark,0.55},{bark_lt,0.45}, name="bark md"}
trunk_all = (trunk2 + trunk3 + poly({{466,352},{470,372},{474,392},{478,412},{481,432},{528,432},{526,412},{527,392},{532,372},{538,352},{520,346},{490,346}}, true)):shrink(3)
local rb = brush{kind="round", width=2.2, point=0.8, stiffness=0.5}
for i=1,46 do
  local x = rand(470, 532); local y0 = rand(345, 495)
  local L = rand(14, 40)
  if trunk_all:at(x,y0) > 0.5 and trunk_all:at(x, y0+L) > 0.5 then
    if rb:fullness() < 0.3 then rb:load(bark_md, 0.5) end
    local dx = (x-500)*0.06
    rb:gesture({{x,y0,0.15},{x+rand(-2,2)+dx*0.5, y0+L*0.5, 0.45},{x+rand(-2,2)+dx, y0+L, 0.1}}, {wobble=1, clip=trunk_all})
  end
end

--@ chunk 111
blend(trunk_all:shrink(1), {tool="badger 8", angle=math.pi/2})

--@ chunk 112
print(wait(10*24*60))
print(drying(500,420), drying(490,470))
gz_bark = pile{{"bone black",1.2},{"raw umber",1.0},{"smalt",0.5},{"rose madder",0.08}, medium=0.6, name="gz bark"}

--@ chunk 113
work(trunk_all:shrink(1), {hand="glaze", pile=gz_bark, tool={kind="filbert", width=10, stiffness=0.25}, coverage=1.5, clip=true, angle=math.pi/2, angle_jitter=0.1, load=0.6, pressure={0.35,0.5}})

--@ chunk 114
local m = trunk_all:grow(2)
blend(m, {tool="badger 14", angle=math.pi/2})
blend(m, {tool="badger 14", angle=0})
blend(m, {tool="badger 14", angle=math.pi/2+0.2})

--@ chunk 115
local function firstn(L, n) local P,Wd={},{}; for i=1,math.min(n,#L.pts) do P[i]=L.pts[i]; Wd[i]=L.w[i] end; return ribbon(P,Wd) end
lower_tree = (trunk2 + trunk3 + poly({{466,352},{470,372},{474,392},{478,412},{481,432},{528,432},{526,412},{527,392},{532,372},{538,352},{520,346},{490,346}}, true)
  + poly({{460,380},{468,340},{490,322},{516,322},{536,340},{544,380}}, true)
  + firstn(NA,4) + firstn(NB,4) + firstn(NC,4) + firstn(L_D,3) + firstn(L_E,3)
  + poly({{458,484},{456,494},{450,502},{438,508},{420,512},{404,514},{420,516},{460,514},{500,513},{540,514},{580,516},{598,515},{582,511},{566,507},{552,500},{546,490},{544,484}}, true)
  + ribbon({{524,446},{540,438},{552,428},{557,420}}, {13,11,9.5,9}) + ribbon({{478,470},{466,466},{458,460}}, {10,8,7})) * above(function(x) return 511 end)
local ringm = lower_tree - trunk_all:grow(3)
work(ringm, {hand="glaze", pile=gz_bark, tool={kind="filbert", width=5, stiffness=0.25}, coverage=2, clip=lower_tree, angle=math.pi/2, angle_jitter=0.3, load=0.6, pressure={0.35,0.5}})
blend(lower_tree:shrink(1), {tool="badger 10", angle=math.pi/2})

--@ chunk 116
local zones = {
  {{{452,350},{462,368},{458,385}}},
  {{{550,350},{542,366},{548,382}}},
  {{{548,415},{552,425},{550,436}}},
}
for _,z in ipairs(zones) do
  for k=1,4 do local rr = rag{width=7}; rr:dip(0.8); rr:wipe(z[1], {pressure=0.8}) end
end
local bot = rect(410,507,190,9)
for k=1,3 do local rr = rag{width=8}; rr:dip(0.7); rr:wipe(bot, {pressure=0.7, angle=0}) end

--@ chunk 117
local zones = {
  {{450,348},{458,362},{456,376},{460,390}},
  {{446,352},{452,370},{448,388}},
  {{552,348},{548,362},{550,378}},
  {{556,352},{552,368},{556,384}},
  {{546,414},{548,424},{546,434},{550,440}},
  {{552,412},{556,426},{554,440}},
}
for _,z in ipairs(zones) do
  for k=1,4 do local rr = rag{width=6}; rr:dip(0.9); rr:wipe(z, {pressure=0.9}) end
end
local bot = rect(405,509,200,8)
for k=1,4 do local rr = rag{width=7}; rr:dip(0.9); rr:wipe(bot, {pressure=0.85, angle=0}) end

--@ chunk 118
print(wait(10*24*60))
print(drying(500,420), drying(462,365), drying(500,512))

--@ chunk 119
local fl_l = poly({{470,342},{462,348},{458,360},{458,374},{461,388},{468,396},{474,380},{474,350}}, true):roughen(0.8,6,81)
local fl_r = poly({{530,342},{540,348},{545,360},{544,374},{541,386},{535,394},{530,380},{530,350}}, true):roughen(0.8,6,82)
gz_bark2 = pile{{"bone black",1.3},{"raw umber",1.0},{"smalt",0.5},{"rose madder",0.06},{"lead white",0.25}, medium=0.2, name="bark2"}
work(fl_l+fl_r, {hand="body", tool="filbert 4", pile=gz_bark2, coverage=3, clip=true, fill=true, angle=math.pi/2, load=0.9})
-- snow at the root foot
local SN_TOP2 = {{380,519},{400,516},{420,514},{440,512},{462,511},{485,511},{505,511},{525,511},{548,511},{570,513},{595,515},{620,518},{640,520}}
local snowcap2 = (below(SN_TOP2):roughen(1.0,9,83) * rect(370,505,280,22)):soften(0.8)
work(snowcap2, {hand="body", tool="filbert 5", pile=sn2_crest, angle=0, angle_jitter=0.06, coverage=3, clip=true, fill=true, load=0.9})
blend(snowcap2:shrink(2), {tool="badger 10", angle=0})

--@ chunk 120
local fl_l = poly({{470,342},{462,348},{458,360},{458,374},{461,388},{468,396},{474,380},{474,350}}, true):grow(2)
local fl_r = poly({{530,342},{540,348},{545,360},{544,374},{541,386},{535,394},{530,380},{530,350}}, true):grow(2)
local keepT = trunk_all:grow(1)
for k=1,4 do local rr=rag{width=8}; rr:dip(0.9); rr:wipe(fl_l - keepT, {pressure=0.85, angle=math.pi/2}); rr=rag{width=8}; rr:dip(0.9); rr:wipe(fl_r - keepT, {pressure=0.85, angle=math.pi/2}) end

--@ chunk 121
print(wait(12*24*60))
print(drying(465,370), drying(540,370), drying(500,512))

--@ chunk 122
local fl_l = poly({{470,340},{460,347},{455,360},{455,374},{459,389},{467,399},{476,380},{476,350}}, true):grow(2)
local fl_r = poly({{530,340},{542,347},{548,360},{547,374},{543,387},{536,397},{528,380},{528,350}}, true):grow(2)
local body = (trunk2 + trunk3 + poly({{466,352},{470,372},{474,392},{478,412},{481,432},{528,432},{526,412},{527,392},{532,372},{538,352},{520,346},{490,346}}, true)
   + ribbon(NA.pts, NA.w) + ribbon(NC.pts, NC.w) + ribbon(L_D.pts, L_D.w) + ribbon(L_E.pts, L_E.w))
wing = ((fl_l + fl_r) - body):soften(0.6)
work(wing, {hand="body", tool="filbert 3", pile=patch3, coverage=4, fill=true, clip=true, angle=0, load=0.9, pressure={0.5,0.7}})

--@ chunk 123
blend(wing:grow(3):soften(2), {tool="badger 8", angle=math.pi/2, clip=true})

--@ chunk 124
print(wait(14*24*60))
print(drying(462,372), drying(540,372))
veil = pile{{"lead white",3},{"Naples yellow",0.4},{"smalt",0.6},{"raw umber",0.12},{"rose madder",0.05}, medium=0.6, name="veil"}

--@ chunk 125
work(wing:grow(1), {hand="glaze", tool={kind="filbert", width=6, stiffness=0.2}, pile=veil, coverage=1.2, clip=true, angle=math.pi/2, load=0.35, pressure={0.25,0.35}})
blend(wing:grow(2), {tool="badger 8", angle=math.pi/2, clip=true})

--@ chunk 126
print(wait(3*24*60))
local shL = poly({{478,420},{470,404},{460,390},{452,372},{448,354},{446,336},{452,322},{468,330},{480,360},{482,400}}, true):roughen(1.0,7,91)
local shR = poly({{524,420},{532,404},{542,390},{550,372},{553,354},{556,338},{552,324},{536,332},{524,360},{522,400}}, true):roughen(1.0,7,92)
shoulders = shL + shR
work(shoulders, {hand="body", tool="filbert 4", pile=bark, coverage=3.5, clip=true, fill=true, angle=math.pi/2, angle_jitter=0.2, load=1})

--@ chunk 127
local cutL = poly({{400,430},{474,430},{470,404},{465,384},{459,364},{451,344},{441,322},{400,300}}, false)
local cutR = poly({{600,430},{528,430},{532,404},{537,384},{543,364},{551,344},{561,322},{600,300}}, false)
local keepLimbs = ribbon(NA.pts, NA.w) + ribbon(NC.pts, NC.w) + ribbon(L_D.pts, L_D.w) + ribbon(L_E.pts, L_E.w)
local wl = (shoulders:grow(2) * (cutL + cutR)) - keepLimbs:grow(1)
for k=1,5 do local rr=rag{width=7}; rr:dip(0.9); rr:wipe(wl, {pressure=0.9, angle=math.pi/2 + 0.3*k}) end

--@ chunk 128
local keepB = (trunk2 + trunk3 + poly({{466,352},{470,372},{474,392},{478,412},{481,432},{528,432},{526,412},{527,392},{532,372},{538,352},{520,346},{490,346}}, true)
   + ribbon(NA.pts, NA.w) + ribbon(NC.pts, NC.w) + ribbon(L_D.pts, L_D.w) + ribbon(L_E.pts, L_E.w)):grow(1)
shw = (shoulders:grow(3) - keepB)
for k=1,6 do local rr=rag{width=6}; rr:dip(1.0); rr:wipe(shw, {pressure=1.0, angle=math.pi/2 + 0.25*k, passes=1, refold=0.05}) end

--@ chunk 129
for k=1,8 do local rr=rag{width=5}; rr:dip(1.0); rr:wipe(shw, {pressure=1.0, angle=0.4*k, passes=1, refold=0.03}) end

--@ chunk 130
print(wait(10*24*60))
print(drying(450,525), drying(300,700), drying(470,370), drying(100,540))

--@ chunk 131
local treebase = poly({{458,484},{456,494},{450,502},{438,508},{420,512},{404,514},{420,516},{460,514},{500,513},{540,514},{580,516},{598,515},{582,511},{566,507},{552,500},{546,490},{544,484}}, true)
  + trunk3
snow_re = (snowm - treebase:grow(1)):soften(0.6)
local function d(x,y) local s=snowy(x); return clamp((y-s)/(714-s),0,1) end
local function u(x,y) return d(x,y) + 0.22*math.abs(x-500)/500 end
work(snow_re, {hand="broad", coverage=2.6, fill=true, clip=true, load=1, pressure={0.5,0.75},
  angle=function(x,y) return (x<500 and -1 or 1) * 0.08 * (1 - d(x,y)*0.6) end, angle_jitter=0.03,
  piles={{sn2_crest, function(x,y) return math.max(0,1-u(x,y)*2.6) end},
         {sn2_mid,   function(x,y) return math.max(0,1-math.abs(u(x,y)-0.42)*2.6) end},
         {sn2_low,   function(x,y) return clamp((u(x,y)-0.45)*2.4,0,1) end}}})
blend(snow_re:shrink(1), {angle=0})
blend(snow_re:shrink(1), {angle=0.03, tool="badger 60"})

--@ chunk 132
gz_dark = pile{{"bone black",1.4},{"raw umber",0.8},{"smalt",0.6},{"rose madder",0.06}, medium=0.65, name="gz dark"}
local wood = (trunk2 + trunk3 + shoulders + poly({{466,352},{470,372},{474,392},{478,412},{481,432},{528,432},{526,412},{527,392},{532,372},{538,352},{520,346},{490,346}}, true)
  + poly({{458,484},{456,494},{450,502},{438,508},{420,512},{404,514},{420,516},{460,514},{500,513},{540,514},{580,516},{598,515},{582,511},{566,507},{552,500},{546,490},{544,484}}, true)
  + ribbon(NA.pts, NA.w) + ribbon(NB.pts, NB.w) + ribbon(NC.pts, NC.w) + ribbon(L_D.pts, L_D.w) + ribbon(L_E.pts, L_E.w)) * above(function(x) return 514 end)
wood_m = wood:shrink(1.2)
work(wood_m, {hand="glaze", pile=gz_dark, tool={kind="filbert", width=6, stiffness=0.25}, coverage=1.6, clip=true, angle=math.pi/2, angle_jitter=0.3, load=0.6, pressure={0.35,0.5}})
blend(wood_m, {tool="badger 8", angle=math.pi/2})

--@ chunk 133
local shL2 = poly({{480,432},{474,414},{466,398},{458,382},{452,364},{448,346},{452,332},{462,330},{474,346},{482,380},{484,420}}, true):roughen(1.4,6,93)
local shR2 = poly({{522,432},{528,414},{536,398},{544,382},{550,364},{553,348},{550,334},{540,332},{528,346},{520,380},{518,420}}, true):roughen(1.4,6,94)
shoulders2 = shL2 + shR2
work(shoulders2, {hand="body", tool="filbert 4", pile=bark, coverage=3.5, clip=true, fill=true, angle=math.pi/2, angle_jitter=0.25, load=1})
local rb = brush{kind="round", width=2, point=0.8, stiffness=0.5}
for i=1,22 do
  local x = rand(452, 552); local y0 = rand(340, 420)
  local L = rand(12, 30)
  if shoulders2:at(x,y0) > 0.5 and shoulders2:at(x, y0+L) > 0.5 then
    if rb:fullness() < 0.3 then rb:load(bark_md, 0.5) end
    rb:gesture({{x,y0,0.15},{x+rand(-2,2), y0+L*0.5, 0.45},{x+rand(-2,2), y0+L, 0.1}}, {wobble=1, clip=shoulders2})
  end
end

--@ chunk 134
print(wait(12*24*60))
print(drying(462,370), drying(540,370), drying(500,450))
work(shoulders2:shrink(0.8), {hand="glaze", pile=gz_dark, tool={kind="filbert", width=5, stiffness=0.25}, coverage=2, clip=true, angle=math.pi/2, angle_jitter=0.3, load=0.7, pressure={0.4,0.55}})
blend(shoulders2:shrink(0.8), {tool="badger 8", angle=math.pi/2})

--@ chunk 135
print(wait(5*24*60))
local flL = poly({{450,350},{452,372},{455,396},{457,420},{456,446},{452,470},{446,490},{436,504},{462,506},{472,470},{476,430},{476,380},{468,350}}, true):roughen(1.3,7,95)
local flR = poly({{552,350},{550,372},{547,396},{545,420},{546,446},{550,470},{556,490},{566,504},{540,506},{530,470},{526,430},{526,380},{534,350}}, true):roughen(1.3,7,96)
flanks = flL + flR
work(flanks, {hand="body", tool="filbert 4", pile=bark, coverage=3.5, clip=true, fill=true, angle=math.pi/2, angle_jitter=0.2, load=1})

--@ chunk 136
print(wait(14*24*60))
print(drying(462,420), drying(540,420))
trunk_full = (trunk_all:grow(3) + flanks + shoulders2) * above(function(x) return 508 end)
local rb = brush{kind="round", width=2.4, point=0.8, stiffness=0.5}
local rd = brush{kind="round", width=2.0, point=0.9, stiffness=0.5}
bark_dk = pile{{"bone black",1.6},{"raw umber",1.0},{"smalt",0.5}, name="bark dk"}
for i=1,140 do
  local x = rand(448, 556); local y0 = rand(340, 495)
  local L = rand(14, 45)
  if trunk_full:at(x,y0) > 0.5 and trunk_full:at(x, math.min(y0+L,505)) > 0.5 then
    local b = (i%3==0) and rb or rd
    local p = (i%3==0) and bark_md or bark_dk
    if b:fullness() < 0.3 then b:load(p, 0.55) end
    local lean = (x-500)*0.08*(y0-340)/160
    b:gesture({{x,y0,0.12},{x+rand(-1.5,1.5)+lean*0.5, y0+L*0.5, 0.5},{x+rand(-1.5,1.5)+lean, y0+L, 0.1}}, {wobble=0.8, clip=trunk_full})
  end
end

--@ chunk 137
print(wait(6*24*60))
print(drying(462,420), drying(500,420))
work(trunk_full:shrink(0.8), {hand="glaze", pile=gz_dark, tool={kind="filbert", width=8, stiffness=0.25}, coverage=1.8, clip=true, angle=math.pi/2, angle_jitter=0.2, load=0.7, pressure={0.4,0.55}})
blend(trunk_full:shrink(0.8), {tool="badger 12", angle=math.pi/2})

--@ chunk 138
print(wait(8*24*60))
print(drying(462,360), drying(540,360), drying(500,420))

--@ chunk 139
-- the trunk silhouette I want (drawn as its own outline), as the inside to keep
local keepT = poly({{432,506},{444,490},{452,468},{456,440},{459,412},{459,390},{455,368},{448,350},{438,334},{430,318},
  {470,300},{478,318},{483,332},{488,330},{494,300},{512,300},{518,330},{522,332},{528,316},{536,298},
  {572,318},{562,336},{551,354},{545,374},{543,394},{543,414},{546,440},{550,468},{558,490},{570,506}}, true)
local limbs = ribbon(NA.pts, NA.w) + ribbon(NB.pts, NB.w) + ribbon(NC.pts, NC.w) + ribbon(L_D.pts, L_D.w) + ribbon(L_E.pts, L_E.w)
carve = (rect(425,300,155,160) - keepT - limbs:grow(0.5)):soften(0.7)
print(carve:area())
local function w1(x,y) return clamp((350-y)/25,0,1) end
local function w3(x,y) return clamp((y-390)/40,0,1) end
local function w2(x,y) return math.max(0,1-w1(x,y)-w3(x,y)) end
sky_pc = mix{{s_pale,0.7},{sky_c,0.3}, name="sky pc"}
work(carve, {hand="body", tool={kind="filbert", width=4, stiffness=0.4, lay=1.5}, coverage=4, fill=true, clip=true, angle=0, angle_jitter=0.3, load=1, pressure={0.55,0.75},
  piles={{sky_pc,w1},{s_pale,w2},{patch3,w3}}})

--@ chunk 140
for k=1,6 do local rr=rag{width=10}; rr:dip(1.0); rr:wipe(carve:grow(3), {pressure=0.95, angle=0.5*k, passes=1, refold=0.04}) end

--@ chunk 141
local zone = rect(420,295,165,175)
for k=1,10 do local rr=rag{width=8}; rr:dip(1.0); rr:wipe(zone, {pressure=1.0, angle=0.3*k, passes=1, refold=0.02}) end

--@ chunk 142
local zone = rect(425,300,40,110) + rect(540,300,45,110)
for k=1,8 do local rr=rag{width=7}; rr:dip(1.0); rr:wipe(zone, {pressure=1.0, angle=math.pi/2 + 0.4*k, passes=1, refold=0.02}) end

--@ chunk 143
print(wait(10*24*60))
print(drying(440,350), drying(560,350))

--@ chunk 144
local keepT = poly({{432,506},{444,490},{452,468},{456,440},{459,412},{459,390},{455,368},{448,350},{438,334},{430,318},
  {470,300},{478,318},{483,332},{488,330},{494,300},{512,300},{518,330},{522,332},{528,316},{536,298},
  {572,318},{562,336},{551,354},{545,374},{543,394},{543,414},{546,440},{550,468},{558,490},{570,506}}, true)
local limbs = ribbon(NA.pts, NA.w) + ribbon(NB.pts, NB.w) + ribbon(NC.pts, NC.w) + ribbon(L_D.pts, L_D.w) + ribbon(L_E.pts, L_E.w)
local old = (trunk_full + shoulders + shoulders2 + flanks) * rect(420,330,170,125)
carve2 = (old - keepT - limbs:grow(1)):soften(0.6)
print(carve2:area())
local s = ""
for y=340,450,10 do
  local l,r = nil,nil
  for x=420,590 do if carve2:at(x,y) > 0.5 then if not l then l=x end; r=x end end
  s = s .. y .. ":" .. tostring(l) .. "-" .. tostring(r) .. "  "
end
print(s)

--@ chunk 145
local w1 = poly({{480,330},{483,318},{487,312},{492,312},{496,320},{497,330}}, true)
local w2 = poly({{510,330},{512,320},{516,313},{521,312},{526,318},{530,330}}, true)
local fl1 = poly({{444,336},{447,326},{452,316},{458,306},{463,318},{462,334}}, true)
local fl2 = poly({{560,336},{557,326},{553,316},{550,308},{542,318},{544,334}}, true)
local m = (w1 + w2 + fl1 + fl2):roughen(0.7,5,97)
work(m, {hand="body", tool="filbert 3", pile=bark, coverage=4, clip=true, fill=true, angle=-math.pi/2, angle_jitter=0.3, load=1})

--@ chunk 146
local w1 = poly({{480,330},{483,318},{487,312},{492,312},{496,320},{497,330}}, true)
local w2 = poly({{510,330},{512,320},{516,313},{521,312},{526,318},{530,330}}, true)
local fl1 = poly({{444,336},{447,326},{452,316},{458,306},{463,318},{462,334}}, true)
local fl2 = poly({{560,336},{557,326},{553,316},{550,308},{542,318},{544,334}}, true)
local m = (w1 + w2 + fl1 + fl2):grow(1.5)
for k=1,8 do local rr=rag{width=6}; rr:dip(1.0); rr:wipe(m, {pressure=1.0, angle=-math.pi/2+0.4*k, passes=1, refold=0.02}) end

--@ chunk 147
print(wait(10*24*60))
local crotch = poly({{455,345},{462,328},{476,322},{500,318},{526,322},{542,330},{548,345},{540,360},{460,360}}, true)
local m = (crotch + ribbon({NA.pts[1],NA.pts[2],NA.pts[3]}, {NA.w[1],NA.w[2],NA.w[3]}) + ribbon({NB.pts[1],NB.pts[2],NB.pts[3]}, {NB.w[1],NB.w[2],NB.w[3]}) + ribbon({NC.pts[1],NC.pts[2],NC.pts[3]}, {NC.w[1],NC.w[2],NC.w[3]})):shrink(1)
m = m * (trunk_full + ribbon(NA.pts, NA.w) + ribbon(NB.pts, NB.w) + ribbon(NC.pts, NC.w))
work(m, {hand="glaze", pile=gz_dark, tool={kind="filbert", width=5, stiffness=0.25}, coverage=1.8, clip=true, angle=math.pi/2, angle_jitter=0.4, load=0.6, pressure={0.35,0.5}})
blend(m, {tool="badger 8", angle=math.pi/2})

--@ chunk 148
local band = (rect(440,345,130,35):soften(8)) * trunk_full:shrink(1)
blend(band, {tool="badger 14", angle=math.pi/2})
blend(band, {tool="badger 14", angle=math.pi/2+0.3})

--@ chunk 149
local function d(x,y) local s=snowy(x); return clamp((y-s)/(714-s),0,1) end
local function u(x,y) return d(x,y) + 0.22*math.abs(x-500)/500 end
local tb = (poly({{458,484},{456,494},{450,502},{438,508},{420,512},{404,514},{420,516},{460,514},{500,513},{540,514},{580,516},{598,515},{582,511},{566,507},{552,500},{546,490},{544,484}}, true) + trunk3):grow(1)
mound = (ellipse(505,532,225,36) * snowm - tb)
work(mound, {hand="body", tool={kind="filbert", width=12, stiffness=0.4}, coverage=2.5, fill=true, load=0.9, pressure={0.5,0.7},
  angle=0, angle_jitter=0.05, edge="lost",
  piles={{sn2_crest, function(x,y) return math.max(0,1-u(x,y)*2.6) end},
         {sn2_mid,   function(x,y) return math.max(0,1-math.abs(u(x,y)-0.42)*2.6) end}}})
blend(mound:grow(20):soften(15) - tb, {angle=0, tool="badger 40"})
blend(mound:grow(20):soften(15) - tb, {angle=0.04, tool="badger 40"})

--@ chunk 150
local zone = ellipse(505,532,255,62):soften(4)
for k=1,12 do local rr=rag{width=16}; rr:dip(1.0); rr:wipe(zone, {pressure=0.95, angle=0.25*k, passes=1, refold=0.03}) end

--@ chunk 151
local m = rect(98,170,22,14):soften(3)
local gb = brush{kind="filbert", width=6, stiffness=0.2}
gb:load(gz_sky, 0.15)
for i=0,3 do gb:stroke({{96, 172+i*3.5},{122, 172+i*3.5}}, {pressure=0.25, clip=m}) end
blend(rect(90,165,40,24):soften(5), {tool="badger 10", angle=0})

--@ chunk 152
local m = rect(99,171,21,12):soften(2)
local gb = brush{kind="filbert", width=5, stiffness=0.2}
gb:load(gz_sky, 0.25)
for i=0,3 do gb:stroke({{97, 172+i*3},{121, 172+i*3}}, {pressure=0.3, clip=m}) end
blend(rect(92,166,36,22):soften(4), {tool="badger 10", angle=0})

--@ chunk 153
local m = ellipse(110,177,24,14):soften(6)
for k=1,4 do blend(m, {tool="badger 12", angle=0.4*k - 0.8}) end

--@ chunk 154
print(wait(6*24*60))
print(drying(110,177), drying(500,540))
local m = ellipse(110,177,16,10):soften(5)
stipple(m, {pile=sky_b, width=3, coverage=1.2, pressure={0.2,0.35}, feather=0.6, dips={12,0.3,0.5}})

--@ chunk 155
local lowm = (snowm * below(function(x) return 560 end)):soften(30)
work(lowm, {hand="glaze", pile=gz_snow, coverage=1.4, clip=true, angle=0, angle_jitter=0.02,
  load_at=function(x,y) return clamp((y-580)/150 + 0.15*math.abs(x-500)/500, 0.05, 0.75) end, pressure={0.3,0.45}})
blend(lowm, {angle=0, tool="badger 60"})
blend(lowm, {angle=0.02, tool="badger 60"})

--@ chunk 156
local lowm = (snowm * below(function(x) return 560 end)):soften(30)
for k=1,3 do blend(lowm, {angle=0.03*(k-2), tool="badger 60"}) end

--@ chunk 157
print(drying(300,100), drying(980,290))
local topm = rect(0,0,1000,260):soften(1) 
work(topm, {hand="glaze", pile=gz_sky, coverage=1.3, clip=true, angle=0, angle_jitter=0.02,
  load_at=function(x,y) return clamp(0.55*(1 - y/240)^1.3, 0.0, 0.55) end, pressure={0.25,0.4}})
for k=1,3 do blend(topm, {angle=0.02*(k-2), tool="badger 60"}) end

--@ chunk 158
print(wait(10*24*60))

--@ chunk 159
GROUPS = {}
local cur = nil
for i,s in ipairs(SEG) do
  if cur and math.abs(cur.pts[#cur.pts][1]-s[1])<0.01 and math.abs(cur.pts[#cur.pts][2]-s[2])<0.01 then
    cur.pts[#cur.pts+1] = {s[3],s[4]}; cur.w[#cur.w+1] = s[5]
  else
    cur = {pts={{s[1],s[2]},{s[3],s[4]}}, w={s[5], s[5]}}
    GROUPS[#GROUPS+1] = cur
  end
end
print(#SEG, #GROUPS)
local big=0; for _,g in ipairs(GROUPS) do if g.w[1] > 1.5 then big=big+1 end end
print(big)

--@ chunk 160
for _,y in ipairs({280,290,300,310,318,325}) do
  local s=""
  for x=410,600,4 do
    local v = structure:at(x,y)
    s = s .. (v>0.5 and "#" or (v>0.1 and "+" or "."))
  end
  print(y, s)
end
print(drying(470,300), drying(500,420), drying(110,178))

--@ chunk 161
local tl = poly({{462,352},{448,352},{447,334},{445,316},{442,300},{439,288},{452,291},{466,314},{470,330}}, true)
local tr = poly({{538,352},{553,352},{554,334},{558,318},{566,306},{580,296},{566,296},{550,310},{540,325}}, true)
local bl = poly({{456,398},{448,420},{446,445},{441,468},{431,489},{416,503},{398,513},{440,516},{458,505}}, true)
local br = poly({{546,398},{553,420},{556,445},{562,468},{573,489},{589,503},{608,513},{565,516},{545,505}}, true)
local cl = poly({{449,383},{441,390},{437,398},{441,405},{449,411}}, true)
local cr = poly({{552,389},{560,396},{564,404},{560,411},{552,415}}, true)
local x1 = ellipse(485,314,5,4) + ellipse(522,315,5,4)
FLARE = (tl + tr + bl + br + cl + cr + x1):roughen(1.2, 10, 7)
print(FLARE:area())
work(FLARE, {hand="body", tool="filbert 5", pile=bark, clip=true, coverage=2.5, angle=function(x,y) return math.pi/2 end})

--@ chunk 162
for _,p in ipairs({{485,314},{522,315}}) do
  for i=1,6 do
    local rr = rag{width=7}
    rr:dip(1.0)
    rr:blot(p[1]+rand(-1,1), p[2]+rand(-1,1), {pressure=0.9})
  end
end

--@ chunk 163
for _,p in ipairs({{485,314},{522,315}}) do
  for i=1,12 do
    local rr = rag{width=9}
    rr:dip(1.0)
    rr:blot(p[1]+rand(-1.5,1.5), p[2]+rand(-1.5,1.5), {pressure=1})
  end
end

--@ chunk 164
local n1 = below({{466,294},{478,298},{483,304},{487,307},{491,304},{496,298},{508,294}}) * rect(466,288,42,42)
local n2 = below({{510,294},{518,298},{522,304},{526,307},{530,304},{535,298},{546,294}}) * rect(510,288,36,42)
NOTCH = (n1 + n2):roughen(0.8, 8, 3)
work(NOTCH, {hand="detail", tool="round 4", pile=bark_dk, clip=true, coverage=3})
work(FLARE, {hand="body", tool="filbert 5", pile=bark_dk, clip=true, coverage=1.5, angle=math.pi/2})

--@ chunk 165
local pts = {}
for y=288,332,5 do for x=464,548,5 do if NOTCH:at(x,y) > 0.2 then pts[#pts+1]={x,y} end end end
print(#pts)
for pass=1,3 do
  for _,p in ipairs(pts) do
    local rr = rag{width=8}
    rr:dip(1.0)
    rr:blot(p[1]+rand(-1.5,1.5), p[2]+rand(-1.5,1.5), {pressure=1})
  end
end

--@ chunk 166
local pts = {}
for y=286,334,3 do for x=462,550,3 do if NOTCH:grow(2):at(x,y) > 0.2 then pts[#pts+1]={x,y} end end end
print(#pts)
for pass=1,3 do
  for _,p in ipairs(pts) do
    local rr = rag{width=7}
    rr:dip(1.0)
    rr:blot(p[1]+rand(-1,1), p[2]+rand(-1,1), {pressure=1})
  end
end

--@ chunk 167
bark_mx = mix{{bark,0.5},{bark_dk,0.5}, name="bark_mx"}
local n1 = below({{476,305},{482,309},{488,312},{494,309},{500,305}}) * rect(478,304,20,24)
local n2 = below({{515,315},{520,319},{525,322},{530,319},{536,315}}) * rect(517,314,18,16)
NOTCH2 = (n1+n2):soften(0.6)
print(NOTCH2:area())
work(NOTCH2, {hand="detail", tool="round 2", pile=bark_mx, clip=true, coverage=3})

--@ chunk 168
local pts = {}
for y=300,332,2.5 do for x=474,540,2.5 do if NOTCH2:grow(2):at(x,y) > 0.1 then pts[#pts+1]={x,y} end end end
print(#pts)
for pass=1,4 do
  for _,p in ipairs(pts) do
    local rr = rag{width=6}
    rr:dip(1.0)
    rr:blot(p[1]+rand(-0.8,0.8), p[2]+rand(-0.8,0.8), {pressure=1})
  end
end

--@ chunk 169
local pts = {}
for y=380,394,2.5 do for x=436,448,2.5 do pts[#pts+1]={x,y} end end
for y=386,399,2.5 do for x=553,566,2.5 do pts[#pts+1]={x,y} end end
for pass=1,4 do
  for _,p in ipairs(pts) do
    local rr = rag{width=6}
    rr:dip(1.0)
    rr:blot(p[1]+rand(-0.8,0.8), p[2]+rand(-0.8,0.8), {pressure=1})
  end
end

--@ chunk 170
wisp = mix{{sky_b,0.45},{sky_c,0.55}, name="wisp"}
local b = brush{kind="filbert", width=7, stiffness=0.3}
b:load(wisp, 0.35)
b:gesture({{28,181,0.05},{60,179,0.3},{100,178,0.6},{125,178,0.55},{165,176,0.3},{205,174,0.04}}, {wobble=1.5})
b:load(wisp, 0.25)
b:gesture({{70,184,0.05},{96,182,0.45},{118,182,0.5},{150,180,0.2},{180,180,0.02}}, {wobble=1})
b:load(wisp, 0.2)
b:gesture({{110,172,0.02},{140,171,0.25},{185,169,0.2},{230,168,0.02}}, {wobble=1})
blend(rect(20,160,220,32):soften(6), {angle=0})

--@ chunk 171
local s = SEG[1]
for k,v in pairs(s) do print(k, type(v), type(v)=="table" and #v or v) end
local n=0
for _,sg in ipairs(SEG) do
  local p = sg[1] or sg.a
end
print(#GROUPS)
local g = GROUPS[1]; for k,v in pairs(g) do print('G',k,type(v), type(v)=="table" and #v or v) end

--@ chunk 172
local n=0
for _,s in ipairs(SEG) do
  if s[1]>195 and s[1]<245 and s[2]>150 and s[2]<200 then n=n+1 end
end
print("seg in zone", n)
local cnt=0
for gi,g in ipairs(GROUPS) do
  for _,p in ipairs(g.pts) do if p[1]>195 and p[1]<245 and p[2]>150 and p[2]<200 then cnt=cnt+1; break end end
end
print("groups in zone", cnt)

--@ chunk 173
local m = rect(22,160,222,36)
for i=1,12 do
  local rr = rag{width=12}
  rr:dip(1.0)
  rr:wipe(m, {pressure=1, angle=0, passes=1, seed=100+i})
end

--@ chunk 174
local m = rect(22,158,224,40)
for i=1,14 do
  local rr = rag{width=10}
  rr:dip(1.0)
  rr:wipe(m, {pressure=1, angle=(i%2==0) and 0 or math.pi/2, passes=1, seed=200+i})
end

--@ chunk 175
wait(8*24*60)
print(drying(420,505), drying(590,505), drying(485,312), drying(500,512), drying(450,300))

--@ chunk 176
wait(6*24*60)
print(drying(420,505), drying(590,505), drying(600,510), drying(450,300), drying(560,300), drying(440,390))

--@ chunk 177
local top = {{380,516},{398,514},{410,512},{424,509},{438,505},{452,503},{466,500},{480,501},{494,499},{508,500},{522,501},{536,499},{550,502},{564,505},{578,509},{592,512},{606,514},{625,516}}
DRIFT = (below(top) * rect(380,494,250,26)):roughen(1.2, 12, 11):soften(0.8)
work(DRIFT, {hand="body", tool="filbert 6", piles={{sn2_crest,0.6},{sn_lt,0.4}}, angle=0, clip=true, coverage=2})
local sh = (below(top) * rect(380,494,250,26)) - (below(top):offset(0)):shrink(0)

--@ chunk 178
local m = DRIFT:grow(2)
for i=1,16 do
  local rr = rag{width=10}
  rr:dip(1.0)
  rr:wipe(m, {pressure=1, angle=(i%2==0) and 0 or 0.3, passes=1, seed=300+i})
end

--@ chunk 179
local m = DRIFT:grow(2)
for i=1,20 do
  local rr = rag{width=8}
  rr:dip(1.0)
  rr:wipe(m, {pressure=1, angle=(i%3)*0.4-0.4, passes=1, seed=400+i})
end

--@ chunk 180
local m = DRIFT:grow(1)
for i=1,25 do
  local rr = rag{width=6}
  rr:dip(1.0)
  rr:wipe(m, {pressure=1, angle=(i%3)*0.5-0.5, passes=1, seed=500+i})
end

--@ chunk 181
wait(3*24*60)
for _,y in ipairs({498,502,506,510,513,516}) do
  local s=""
  for x=390,620,5 do
    local a = (trunk_full + FLARE):at(x,y)
    local d = DRIFT:at(x,y)
    s = s .. (a>0.5 and (d>0.3 and "X" or "#") or (d>0.3 and "d" or "."))
  end
  print(y, s)
end
print(drying(500,505), drying(430,508))

--@ chunk 182
local base = (trunk_full + FLARE + rect(452,505,100,7)) * DRIFT:grow(2)
REP = base:roughen(0.6,8,5)
print(REP:area())
work(REP, {hand="body", tool="filbert 5", pile=bark_mx, clip=true, coverage=2.5, angle=0})

--@ chunk 183
wait(5*24*60)
print(drying(500,508), drying(440,512))

--@ chunk 184
wait(6*24*60)
print(drying(500,508), drying(440,512), drying(590,510))

--@ chunk 185
local lower = (trunk_full + FLARE + REP) * rect(395,440,215,80)
work(lower, {hand="glaze", tool={kind="filbert", width=8, stiffness=0.3}, pile=gz_dark, clip=true, coverage=1.2, angle=math.pi/2})
-- the pale old wipe streaks on the flanks
local fl = (trunk_full+FLARE) * (rect(440,372,22,40) + rect(545,372,22,40))
work(fl, {hand="glaze", tool={kind="filbert", width=5, stiffness=0.3}, pile=gz_dark, clip=true, coverage=1.5, angle=math.pi/2})

--@ chunk 186
local lower = (trunk_full + FLARE + REP) * rect(395,425,215,95)
blend(lower, {angle=math.pi/2})
blend(lower, {angle=0})

--@ chunk 187
local pts={}
for y=380,398,2 do for x=438,447,2 do pts[#pts+1]={x,y} end end
for pass=1,4 do for _,p in ipairs(pts) do
  local rr=rag{width=5}; rr:dip(1.0); rr:blot(p[1],p[2],{pressure=1})
end end

--@ chunk 188
snow_base = mix{{sn2_mid,0.7},{sn_lt,0.3}, name="snow_base"}
local top = {{392,517},{404,515.5},{416,513},{428,512.5},{440,510},{452,511.5},{462,509},{474,510.5},{488,507.5},{502,509},{516,508},{530,510.5},{542,508.5},{554,511},{566,510},{578,513},{590,512.5},{602,515},{616,517}}
local lobes = below(top) * rect(388,500,232,22)
MOUND2 = lobes:roughen(0.8, 9, 21)
work(MOUND2, {hand="body", tool="filbert 4", pile=snow_base, angle=0, clip=true, coverage=2})

--@ chunk 189
local n,wsum=0,0
local hist={}
for _,s in ipairs(SEG) do local b=math.floor(s[5]); hist[b]=(hist[b] or 0)+1 end
for k,v in pairs(hist) do print('w',k,v) end
local gp=0
for _,g in ipairs(GROUPS) do gp=gp+#g.pts end
print('group pts',gp)
local g=GROUPS[5]; print(g.pts[1][1],g.pts[1][2],g.w[1],g.pts[#g.pts][1],g.pts[#g.pts][2],g.w[#g.w])

--@ chunk 190
local cnt={}
for _,s in ipairs(SEG) do
  if s[5]>=1 then
    local key = math.floor(s[1]/200)..","..math.floor(s[2]/100)
    cnt[key]=(cnt[key] or 0)+1
  end
end
for k,v in pairs(cnt) do print(k,v) end

--@ chunk 191
twig_dk = pile{{"bone black",1.4},{"raw umber",1},{"smalt",0.6},{"rose madder",0.05}, medium=0.2, name="twig_dk"}
local rd = brush{kind="round", width=4, point=1, stiffness=0.5}
local n=0
for _,s in ipairs(SEG) do
  if s[5]>=1.5 and s[1]>640 and s[1]<820 and s[2]>140 and s[2]<300 then
    local w = s[5]*0.8
    if rd:fullness() < 0.3 then rd:reload(twig_dk, 0.6) end
    if n==0 then rd:reload(twig_dk,0.6) end
    local p = rd:pressure_for(w)
    rd:stroke({{s[1],s[2]},{s[3],s[4]}}, {pressure=p, ramps={0.05,0.05}})
    n=n+1
  end
end
print(n)

--@ chunk 192
rd2 = brush{kind="round", width=3, point=1, stiffness=0.5}
rd2:reload(twig_dk, 0.7)
function dk(pts)
  local g = {}
  for i,p in ipairs(pts) do g[#g+1] = {p[1], p[2], rd2:pressure_for(p[3])} end
  if rd2:fullness() < 0.35 then rd2:reload(twig_dk, 0.7) end
  rd2:gesture(g, {wobble=0})
end
dk({{701,210,2.6},{707,195,2.4},{713,180,2.2},{717,166,2.0},{716,150,1.8},{713,135,1.6},{710,115,1.3},{707,96,1.0}})

--@ chunk 193
for i=1,10 do
  local rr = rag{width=5}; rr:dip(1.0)
  rr:wipe({{701,212},{707,195},{713,180},{717,166},{716,150},{713,135},{710,115},{707,94}}, {pressure=1})
end

--@ chunk 194
local n=0
for _,s in ipairs(SEG) do
  if s[1]>680 and s[1]<780 and s[2]>70 and s[2]<215 then n=n+1 end
end
print(n)
local minx,maxx,miny,maxy=1e9,-1e9,1e9,-1e9
for _,s in ipairs(SEG) do minx=math.min(minx,s[1]);maxx=math.max(maxx,s[1]);miny=math.min(miny,s[2]);maxy=math.max(maxy,s[2]) end
print(minx,maxx,miny,maxy)

--@ chunk 195
for _,s in ipairs(SEG) do
  if s[1]>680 and s[1]<780 and s[2]>70 and s[2]<215 then print(string.format("%.0f,%.0f -> %.0f,%.0f w%.1f", s[1],s[2],s[3],s[4],s[5])) end
end

--@ chunk 196
print(drying(300,600), drying(700,650), drying(500,560))
local fg = snowm * below({{0,585},{250,570},{500,566},{750,572},{1000,588}}):soften(25)
work(fg, {hand="glaze", pile=gz_snow, angle=function(x,y) return 0.04 end, coverage=1, clip=true,
  load_at=function(x,y) return 0.12 + 0.5*smoothstep(570,714,y) end, pressure={0.15,0.3}})

--@ chunk 197
local m = snowm * below({{0,560},{1000,560}})
for i=1,10 do
  local rr = rag{width=30}; rr:dip(1.0)
  rr:wipe(m, {pressure=1, angle=(i%2==0) and 0 or 0.15, passes=1, seed=600+i})
end

--@ chunk 198
wait(20*24*60)
print(drying(500,505), drying(300,690), drying(440,390), drying(120,178))

--@ chunk 199
local a = math.random(); local b = rand(0,1)
print(string.format("%.10f %.10f", a, b))
print(type(math.randomseed))
local cands = {199, 1818+199, 1818*1000+199, 1818*10000+199, 1818*100000+199, 199*1818, (1818<<32)|199, (1818<<16)|199, 1818*1000003+199, 1818*31+199}
for _,c in ipairs(cands) do
  local ok = pcall(math.randomseed, c)
  if ok then print(c, string.format("%.10f", math.random())) end
end

--@ chunk 200
cornp = mix{{sn2_mid,0.85},{sn2_low,0.15}, name="cornp"}
cornp:add{{"rose madder",0.05},{"smalt",0.15}}
print(cornp)

--@ chunk 201
cornp2 = mix{{sn2_mid,0.8},{cornp,0.2}, name="cornp2"}

--@ chunk 202
print(drying(995,695), drying(790,680))
local m = poly({{988,668},{1000,664},{1000,714},{984,714},{986,694}}, true):soften(2)
work(m, {hand="body", tool={kind="filbert", width=5, stiffness=0.35}, pile=cornp2, coverage=3, fill=true, angle=0.1, load=0.8, pressure={0.5,0.7}})
blend(m:grow(4):soften(3), {tool="badger 10", angle=0})

--@ chunk 203
local m = rect(945,645,55,69)
for i=1,12 do
  local rr = rag{width=10}; rr:dip(1.0)
  rr:wipe(m, {pressure=1, angle=(i%2==0) and 0 or math.pi/2, passes=1, seed=700+i})
end

--@ chunk 204
moonp = pile{{"lead white",6},{"Naples yellow",0.7},{"chrome yellow",0.05}, name="moon"}
local cx, cy, r = 902, 128, 8
MOON = (ellipse(cx, cy, r, r) - ellipse(cx+3.2, cy-3.4, r*0.93, r*0.93)):soften(0.4)
print(MOON:area())
work(MOON, {hand="detail", tool={kind="round", width=1.6, point=0.8, stiffness=0.5}, pile=moonp, coverage=3, clip=true, load=0.8})

--@ chunk 205
print(wait(14*24*60)); print(drying(900,132), drying(990,690))

--@ chunk 206
rampp = mix{{sn2_crest,0.6},{sn2_mid,0.4}, name="rampp"}

--@ chunk 207
local m = rect(375,538,60,22):soften(5)
work(m, {hand="body", tool={kind="filbert", width=8, stiffness=0.3}, pile=rampp, coverage=2, fill=true, angle=0, angle_jitter=0.05,
  load_at=function(x,y) return clamp(0.9 - (y-543)/18, 0.2, 0.9) end, pressure={0.45,0.65}})
blend(m:grow(4):soften(4), {tool="badger 20", angle=0})

--@ chunk 208
local m = rect(366,532,80,34)
for i=1,14 do
  local rr = rag{width=10}; rr:dip(1.0)
  rr:wipe(m, {pressure=1, angle=(i%2==0) and 0 or math.pi/2, passes=1, seed=800+i})
end

--@ chunk 209
crowp = pile{{"bone black",1.5},{"raw umber",0.8},{"smalt",0.5},{"rose madder",0.05}, medium=0.12, name="crow"}
cbr = brush{kind="round", width=1.8, point=1, stiffness=0.5}
function crow(cx, cy, s, ph, tilt)
  tilt = tilt or 0
  local ct, st = math.cos(tilt), math.sin(tilt)
  local function P(dx, dy, p) return {cx + dx*ct - dy*st, cy + dx*st + dy*ct, p} end
  cbr:load(crowp, 0.7)
  -- body: tail to head, flying leftward-ish (head at -x)
  cbr:gesture({P(0.22*s, 0.02*s, 0.45), P(0.08*s, 0, 0.9), P(-0.08*s, -0.01*s, 0.85), P(-0.17*s, -0.02*s, 0.4)}, {})
  -- near wing (left on canvas)
  cbr:gesture({P(0.0, 0, 0.85), P(-0.12*s, -0.13*s*ph, 0.7), P(-0.27*s, -0.2*s*ph, 0.45), P(-0.47*s, -0.12*s*ph, 0.0)}, {})
  -- far wing
  cbr:gesture({P(0.02*s, 0, 0.85), P(0.13*s, -0.16*s*ph, 0.65), P(0.3*s, -0.22*s*ph, 0.4), P(0.5*s, -0.16*s*ph, 0.0)}, {})
end
print(cbr:mark_width(0.9))
crow(120, 245, 13, 1, 0.08)

--@ chunk 210
local s = SEG[1]
local function dump(t, d)
  d = d or 0
  if type(t) ~= "table" then return tostring(t) end
  local parts = {}
  local n = 0
  for k, v in pairs(t) do n = n + 1; if n > 8 then break end
    parts[#parts+1] = tostring(k) .. "=" .. (type(v)=="table" and (d<1 and dump(v, d+1) or "tbl#"..#v) or tostring(v)) end
  return "{" .. table.concat(parts, ", ") .. "}"
end
print(dump(SEG[1])); print(dump(SEG[500])); print(dump(GROUPS[1])); print(dump(TIPS[1]))
local wh = {}
for _, sg in ipairs(SEG) do local w = sg.w or sg[5] or 0; local b = math.floor((w or 0)*2)/2; wh[b] = (wh[b] or 0) + 1 end
for k, v in pairs(wh) do print("w", k, v) end

--@ chunk 211
-- where are GROUPS? summarize root point, root width, length, count by region
local n=0
for i, g in ipairs(GROUPS) do
  if i % 10 == 1 then
    local p1, pn = g.pts[1], g.pts[#g.pts]
    print(i, #g.pts, string.format("(%.0f,%.0f)->(%.0f,%.0f) w %.2f->%.2f", p1[1], p1[2], pn[1], pn[2], g.w[1], g.w[#g.w]))
  end
end

--@ chunk 212
for _, i in ipairs{211, 141, 181} do local g = GROUPS[i]; local t = {}
for j, p in ipairs(g.pts) do t[#t+1] = string.format("%.0f,%.0f/%.1f", p[1], p[2], g.w[j]) end
print(i, table.concat(t, " ")) end

--@ chunk 213
tb = brush{kind="round", width=1.6, point=1, stiffness=0.5}
function restate(g, pile, k)
  k = k or 0.9
  tb:load(pile, 0.6)
  local pts = {}
  for j, p in ipairs(g.pts) do pts[#pts+1] = {p[1], p[2], tb:pressure_for(g.w[j]*k)} end
  tb:gesture(pts, {})
end
restate(GROUPS[211], crowp)

--@ chunk 214
brp = pile{{"bone black",1.4},{"raw umber",1.1},{"smalt",0.5},{"rose madder",0.06},{"lead white",0.15}, medium=0.1, name="branch_dk"}
function restate2(g, pile, k, wmin)
  local pts = {}
  for j, p in ipairs(g.pts) do
    local w = g.w[j]
    if w < wmin then pts[#pts+1] = {p[1], p[2], 0}; break end
    pts[#pts+1] = {p[1], p[2], tb:pressure_for(math.min(w*k, 3.2))}
  end
  if #pts >= 3 then
    if tb:fullness() < 0.35 then tb:load(pile, 0.6) end
    tb:gesture(pts, {})
    return 1
  end
  return 0
end
tb:reload(brp, 0.6)
local n = 0
for i, g in ipairs(GROUPS) do
  local p = g.pts[1]
  if p[1] < 470 and p[2] < 300 then n = n + restate2(g, brp, 1.0, 0.55) end
end
print(n)

--@ chunk 215
local function d(t, depth)
  depth = depth or 0
  if type(t) ~= "table" then return tostring(t) end
  if depth > 2 then return "tbl#" .. #t end
  local parts, n = {}, 0
  for k, v in pairs(t) do n = n + 1; if n > 6 then parts[#parts+1] = "..."; break end
    parts[#parts+1] = tostring(k) .. "=" .. d(v, depth+1) end
  return "{" .. table.concat(parts, ",") .. "}"
end
print("SEC", #SEC, d(SEC[1])); print("SECS", #SECS, d(SECS[1])); print("CROWN", #CROWN, d(CROWN[1])); print("TIPS", d(TIPS[2]))

--@ chunk 216
for i, c in ipairs(CROWN) do local t = {}
 for _, p in ipairs(c[1]) do t[#t+1] = string.format("%.0f,%.0f", p[1], p[2]) end
 print("C"..i, c[2], c[3], table.concat(t, " ")) end
for i, c in ipairs(SECS) do local t = {}
 for j, p in ipairs(c.pts) do t[#t+1] = string.format("%.0f,%.0f/%.1f", p[1], p[2], c.w[j]) end
 print("S"..i, table.concat(t, " ")) end
for i, c in ipairs(SEC) do print("SEC"..i, table.concat(c, ",")) end

--@ chunk 217
function restate_c(c, k)
  local pts, n = {}, #c[1]
  for j, p in ipairs(c[1]) do
    local w = lerp(c[2], c[3], (j-1)/(n-1)) * k
    pts[#pts+1] = {p[1], p[2], tb:pressure_for(math.min(w, 3.2))}
  end
  tb:load(brp, 0.7)
  tb:gesture(pts, {})
end
restate_c(CROWN[2], 0.5)

--@ chunk 218
for i, c in ipairs(CROWN) do if i ~= 2 then restate_c(c, 0.5) end end
tb:reload(brp, 0.6)
local n = 0
for i, g in ipairs(GROUPS) do
  local p = g.pts[1]
  if not (p[1] < 470 and p[2] < 300) and i ~= 211 then n = n + restate2(g, brp, 1.0, 0.55) end
end
print(n)

--@ chunk 219
for _, y in ipairs{320, 360, 400, 440} do
  local first
  for x = 430, 480 do if trunk_full:at(x, y) > 0.5 then first = x; break end end
  print(y, first, drying(455, y))
end

--@ chunk 220
LF = poly({{451,338},{470,336},{471,446},{454,446},{451,420},{450,380}})
work(LF, {hand="glaze", pile=gz_dark, tool="filbert 4", angle=math.pi/2, length={20,50}, coverage=1.5, clip=true, pressure={0.4,0.6}})

--@ chunk 221
print(wait(6*24*60)); print(drying(400,110), drying(120,245), drying(460,400))

--@ chunk 222
local ok, e = pcall(function() local t = pile{{"lead white",1}, thinner=0.5}; print(t, t.thinner) end)
print(ok, e)
print(drying(110,180), drying(60,180))
for _,p in ipairs(palette()) do print(p) end

--@ chunk 223
GHOST = poly({{99,173},{122,175},{122.5,187},{99,185.5}}):soften(1.5)
gfill = pile{{"lead white",0.45},{"smalt",0.34},{"cobalt blue",0.15},{"bone black",0.015},{"rose madder",0.012}, thinner=0.6}
work(GHOST, {hand="body", tool="filbert 6", pile=gfill, angle=0.05, coverage=1, clip=true, pressure={0.3,0.45}, seed=11})
print(drying(110,180))

--@ chunk 224
print(wait(15))

--@ chunk 225
work(GHOST:grow(0.8):soften(1), {hand="body", tool="filbert 5", pile=gfill, angle=0.0, coverage=1.2, clip=true, pressure={0.3,0.5}, seed=23})
wait(15)

--@ chunk 226
work(GHOST:grow(1.2):soften(1.5), {hand="body", tool="filbert 4", pile=gfill, angle=0.03, coverage=1.3, clip=true, pressure={0.35,0.55}, seed=37})
blend(GHOST:grow(3):soften(2), {angle=0.02, tool={kind="badger", width=10}})
wait(20)

--@ chunk 227
gfill2 = pile{{"lead white",0.45},{"smalt",0.34},{"cobalt blue",0.15},{"bone black",0.015},{"rose madder",0.012}, thinner=0.35}
work(GHOST:grow(0.5):soften(1.5), {hand="body", tool="filbert 4", pile=gfill2, angle=0.03, coverage=1.5, clip=true, pressure={0.35,0.55}, seed=41})
wait(20)

--@ chunk 228
work(GHOST:grow(2):soften(3), {hand="body", tool="filbert 4", pile=gfill2, angle=0.02, coverage=1.0, clip=true, pressure={0.3,0.5}, seed=53})
wait(20)

--@ chunk 229
print(wait(3*24*60)); print(drying(110,180))

--@ chunk 230
for _,p in ipairs{{380,530},{500,530},{620,530},{500,560},{300,540},{700,540}} do print(p[1],p[2],drying(p[1],p[2])) end
print(DRIFT:area(), MOUND2:area(), REP:area(), mound:area(), driftm:area())
for _,m in ipairs{DRIFT,MOUND2,REP,mound,driftm} do print(m:at(380,530), m:at(500,540), m:at(640,540), m:at(500,560)) end

--@ chunk 231
local function bounds(m) local x0,x1,y0,y1=1e9,-1,1e9,-1 for y=0,714,3 do for x=0,1000,3 do if m:at(x,y)>0.5 then if x<x0 then x0=x end if x>x1 then x1=x end if y<y0 then y0=y end if y>y1 then y1=y end end end end return x0,x1,y0,y1 end
print("mound",bounds(mound)) print("snow_re",bounds(snow_re)) print("carve",bounds(carve))

--@ chunk 232
stalkp = pile{{"raw umber",1.0},{"bone black",0.6},{"yellow ochre",0.25},{"smalt",0.2},{"lead white",0.35}, medium=0.2, name="stalk"}
srg = brush{kind="rigger", width=1.0, point=1, stiffness=0.45}
local s="" for _,p in ipairs{0.1,0.3,0.5,0.7,0.9} do s=s..string.format("%.1f->%.2f ",p,srg:mark_width(p)) end print(s)
print(drying(170,690))

--@ chunk 233
function stalks(cx, cy, n, h, lean)
  for i=1,n do
    local x = cx + rand(-9,9); local y = cy + rand(-2.5,2.5)
    local a = -math.pi/2 + lean + rand(-0.28,0.28)
    local L = h*rand(0.45,1.0)
    local bend = rand(-0.12,0.12) + lean*0.4
    local pts = {{x,y,0.75}}
    local nseg = 4
    for k=1,nseg do
      a = a + bend/nseg*2
      x = x + math.cos(a)*L/nseg; y = y + math.sin(a)*L/nseg
      pts[#pts+1] = {x,y, lerp(0.7,0.05,k/nseg)}
    end
    if srg:fullness() < 0.35 then srg:load(stalkp, 0.6) end
    srg:gesture(pts, {wobble=0.3})
    if math.random() < 0.3 then
      local q = pts[3]; local side = (math.random()<0.5) and -1 or 1
      local b = a + side*rand(0.4,0.7); local l2 = L*rand(0.2,0.35)
      srg:gesture({{q[1],q[2],0.5},{q[1]+math.cos(b)*l2*0.5, q[2]+math.sin(b)*l2*0.5,0.3},{q[1]+math.cos(b)*l2, q[2]+math.sin(b)*l2,0.03}}, {wobble=0.2})
    end
  end
end
srg:load(stalkp, 0.6)
stalks(170, 692, 7, 30, 0.15)

--@ chunk 234
local m = rect(150,655,45,42)
for i=1,10 do local rr=rag{width=8}; rr:dip(1.0); rr:wipe(m, {pressure=1, angle=(i%2==0) and 0 or math.pi/2, passes=1, seed=900+i}) end
