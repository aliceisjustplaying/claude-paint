-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box every
--@ engine 5

--@ chunk 1
canvas{size=650, aspect=1.3, linen={18,16}, seed=417,
 ground={{pile={{"lead white",6},{"yellow ochre",1},{"red earth",0.4}}, um=120, apply="knife", texture=0.4}}}
print(W,H)
print(table.concat(tubes(), ", "))

--@ chunk 2
sky_top = pile{{"lead white",4},{"cobalt blue",1},{"cobalt violet",0.6}, name="sky_top"}
sky_mid = pile{{"lead white",5},{"permanent alizarin",0.25},{"Naples yellow",1}, name="sky_mid"}
sky_low = pile{{"lead white",4},{"Naples yellow",2},{"cadmium yellow",0.6},{"orange chrome",0.2}, name="sky_low"}
hills = pile{{"lead white",2},{"cobalt violet",1},{"ultramarine blue",0.6},{"permanent alizarin",0.15}, name="hills"}
trees = pile{{"ultramarine blue",1},{"viridian",0.6},{"burnt sienna",0.6},{"lead white",0.6}, name="trees"}
field_lit = pile{{"lead white",2},{"yellow ochre",2},{"cadmium yellow",0.6},{"orange chrome",0.4}, name="field_lit"}
stack_lit = pile{{"orange chrome",2},{"cadmium yellow",1},{"lead white",1},{"vermilion",0.3}, name="stack_lit"}
stack_sh = pile{{"cobalt violet",1.5},{"ultramarine blue",1},{"permanent alizarin",0.4},{"lead white",1},{"burnt sienna",0.3}, name="stack_sh"}
cast = pile{{"ultramarine blue",1},{"cobalt violet",1},{"permanent alizarin",0.3},{"lead white",1.2}, name="cast"}
sketch = pile{{"burnt sienna",1},{"ultramarine blue",0.6}, turps=0.6, name="sketch"}

--@ chunk 3
local b = brush{kind="round", width=3, point=0.7}
b:load(sketch, 0.7)
-- horizon / tree line
b:gesture({{0,395,0.4},{150,385,0.5},{300,398,0.4},{520,392,0.5},{700,405,0.4},{1000,410,0.4}})
b:gesture({{0,428,0.4},{400,425,0.4},{1000,430,0.4}})
-- main stack: roof
b:load(sketch,0.7)
b:gesture({{222,478,0.5},{250,420,0.6},{300,340,0.6},{350,285,0.6},{385,268,0.5}})
b:gesture({{385,268,0.5},{425,285,0.6},{475,345,0.6},{522,420,0.6},{545,476,0.5}})
b:gesture({{222,478,0.5},{300,492,0.5},{385,496,0.5},{470,490,0.5},{545,476,0.5}})
-- drum
b:load(sketch,0.7)
b:gesture({{240,490,0.5},{235,560,0.6},{242,612,0.5}})
b:gesture({{530,488,0.5},{534,560,0.6},{525,612,0.5}})
b:gesture({{242,612,0.5},{320,630,0.5},{400,634,0.5},{470,628,0.5},{525,612,0.5}})
-- cast shadow
b:load(sketch,0.6)
b:gesture({{242,612,0.4},{150,640,0.4},{40,672,0.4},{0,684,0.3}})
b:gesture({{400,634,0.4},{250,690,0.4},{100,735,0.4},{0,760,0.3}})
-- distant stack
b:load(sketch,0.6)
b:gesture({{745,470,0.4},{760,435,0.4},{782,418,0.4},{805,436,0.4},{818,470,0.4}})
b:gesture({{750,470,0.4},{752,492,0.4},{812,492,0.4},{815,470,0.4}})

--@ chunk 4
stack_m = poly({{222,480},{240,430},{275,370},{320,310},{360,277},{385,268},{412,276},{450,310},{492,365},{525,425},{546,478},{532,492},{536,560},{527,612},{470,630},{400,636},{320,630},{242,614},{235,560},{238,492}}, true)
dist_m = poly({{745,472},{752,448},{766,428},{782,418},{798,428},{812,448},{819,472},{814,478},{813,492},{782,496},{752,493},{750,478}}, true)
tree_curve = {{0,393},{80,384},{150,382},{230,392},{300,397},{380,390},{460,386},{520,392},{600,398},{680,404},{760,408},{850,404},{920,410},{1000,412}}
sky_m = above(tree_curve)
ground_curve = {{0,428},{300,426},{600,429},{1000,432}}
field_m = below(ground_curve)
trees_m = -(sky_m) - field_m
print(sky_m:area(), trees_m:area(), field_m:area())

--@ chunk 5
sun_x, sun_y = 840, 372
local function glow(x,y) local d=math.sqrt((x-sun_x)^2+((y-sun_y)*1.6)^2); return math.exp(-(d/260)^2) end
local wtop = function(x,y) return clamp(1 - y/220,0,1)*(1-glow(x,y)) + 0.02 end
local wmid = function(x,y) return math.exp(-((y-210)/110)^2)*(1-0.7*glow(x,y)) + 0.02 end
local wlow = function(x,y) return clamp((y-200)/170,0,1) + 1.5*glow(x,y) + 0.02 end
work(sky_m:grow(6), {hand="broad", piles={{sky_top,wtop},{sky_mid,wmid},{sky_low,wlow}}, angle=function(x,y) return 0.08*math.sin(x/170) end, coverage=2.2, fill=true, edge="firm"})

--@ chunk 6
local wdark = function(x,y) return 0.3 + 0.7*clamp(1 - x/900,0,1) end
local wlight = function(x,y) return 0.4 + 0.8*clamp(x/900,0,1) end
work(trees_m, {hand="body", piles={{trees,wdark},{hills,wlight}}, angle=0, coverage=2.5, fill=true, edge="soft", length={15,40}})

--@ chunk 7
field_far = pile{{"lead white",3},{"Naples yellow",1.5},{"permanent alizarin",0.12},{"cobalt violet",0.4}, name="field_far"}
field_warm = pile{{"yellow ochre",2},{"orange chrome",1},{"lead white",1},{"burnt sienna",0.3}, name="field_warm"}
local wfar = function(x,y) return clamp(1-(y-430)/120,0,1)+0.03 end
local wlit = function(x,y) return math.exp(-((y-520)/90)^2) + 0.3*clamp(x/1000,0,1) end
local wwarm = function(x,y) return clamp((y-480)/200,0,1) + 0.03 end
work(field_m:grow(4), {hand="broad", piles={{field_far,wfar},{field_lit,wlit},{field_warm,wwarm}}, angle=function(x,y) return 0.05 + 0.1*math.sin(x/200+y/90) end, coverage=2.2, fill=true, scale_at=function(x,y) return 0.5 + (y-430)/340 end})

--@ chunk 8
stack_dk = pile{{"ultramarine blue",1},{"permanent alizarin",0.5},{"cobalt violet",1},{"burnt sienna",0.5},{"lead white",0.5}, name="stack_dk"}
stack_rust = pile{{"burnt sienna",1},{"permanent alizarin",0.4},{"cobalt violet",0.8},{"lead white",0.6},{"orange chrome",0.3}, name="stack_rust"}

--@ chunk 9
local shifted = mask(function(x,y) return stack_m:at(x+38,y-14) end)
rim_m = (stack_m - shifted):soften(3)
local wdk = function(x,y) return 0.5 + clamp((420-x)/150,0,1) end
local wsh = function(x,y) return 0.6 end
local wrust = function(x,y) return clamp((y-500)/120,0,1)*0.9 end
local roof = stack_m * above({{0,486},{1000,486}})
local drum = stack_m * below({{0,486},{1000,486}})
work(roof, {hand="body", piles={{stack_dk,wdk},{stack_sh,wsh},{stack_rust,function(x,y) return 0.15 end}}, angle=function(x,y) if x<385 then return -1.0 else return 1.0 end end, coverage=2.5, fill=true, edge="firm", length={20,50}})
work(drum, {hand="body", piles={{stack_dk,wdk},{stack_sh,wsh},{stack_rust,wrust}}, angle=1.5, angle_jitter=0.2, coverage=2.5, fill=true, edge="firm", length={20,50}})

--@ chunk 10
shadow_m = poly({{250,600},{180,620},{90,640},{0,656},{0,748},{120,712},{260,672},{380,646},{470,632},{520,618},{400,624},{300,612}}, true):roughen(6,30,11)
cast_warm = pile{{"cobalt violet",1},{"ultramarine blue",0.6},{"permanent alizarin",0.3},{"lead white",1},{"orange chrome",0.15}, name="cast_warm"}
work(shadow_m, {hand="body", piles={{cast,function(x,y) return 0.4+clamp((y-620)/120,0,1) end},{cast_warm,function(x,y) return 0.6 end}}, angle=function(x,y) return 0.25 end, angle_jitter=0.15, coverage=2.5, fill=true, edge={found=0.4, soft=0.6, period=40}, length={20,60}})

--@ chunk 11
print(wait(36*60)); print(drying(500,100), drying(380,400), drying(600,600), drying(200,650))

--@ chunk 12
print(wait(30*60)); print(drying(500,100), drying(380,400), drying(600,600), drying(200,650), drying(400,560))

--@ chunk 13
sk_blue = pile{{"lead white",3},{"cerulean blue",1},{"cobalt blue",0.5}, name="sk_blue"}
sk_turq = pile{{"lead white",4},{"cerulean blue",0.6},{"lemon chrome",0.25},{"viridian",0.08}, name="sk_turq"}
sk_rose = pile{{"lead white",3},{"rose madder",0.6},{"Naples yellow",0.6},{"vermilion",0.1}, name="sk_rose"}
sk_gold = pile{{"lead white",2},{"cadmium yellow",1},{"Naples yellow",1},{"orange chrome",0.3}, name="sk_gold"}
sk_orange = pile{{"orange chrome",1},{"cadmium yellow",0.6},{"lead white",1},{"vermilion",0.25}, name="sk_orange"}
sk_sun = pile{{"lead white",3},{"lemon chrome",1},{"cadmium yellow",0.3}, name="sk_sun"}

--@ chunk 14
sk_blue:add{{"cobalt blue",0.4},{"cerulean blue",0.4}}
local function glow(x,y) local d=math.sqrt((x-sun_x)^2+((y-sun_y)*1.8)^2); return math.exp(-(d/230)^2) end
local wb = function(x,y) return clamp(1 - y/170,0,1)*1.2 + 0.01 end
local wt = function(x,y) return math.exp(-((y-165)/60)^2)*0.8 + 0.01 end
local wr = function(x,y) return math.exp(-((y-225)/55)^2)*(1-glow(x,y)) + 0.01 end
local wg = function(x,y) return math.exp(-((y-310)/60)^2)*(1-0.5*glow(x,y)) + 0.01 end
local wo = function(x,y) return clamp((y-330)/50,0,1)*(1-glow(x,y))*0.8 + 0.01 end
local ws = function(x,y) return 2.2*glow(x,y)^1.5 + 0.005 end
work(sky_m:grow(4), {hand="broad", piles={{sk_blue,wb},{sk_turq,wt},{sk_rose,wr},{sk_gold,wg},{sk_orange,wo},{sk_sun,ws}}, angle=function(x,y) return 0.05*math.sin(x/150+y/40) end, coverage=2.0, fill=true, edge="firm", seed=21})

--@ chunk 15
local r = rag{width=40}
local roof_in = (stack_m * above({{0,486},{1000,486}})):shrink(14)
r:wipe(roof_in, {pressure=0.6, angle=0, passes=2, refold=0.4})

--@ chunk 16
roof_m = stack_m * above({{0,488},{1000,488}})
drum_m = stack_m * below({{0,488},{1000,488}})
roof_glow = pile{{"vermilion",0.6},{"orange chrome",0.6},{"cobalt violet",0.6},{"lead white",1}, name="roof_glow"}
local wdk = function(x,y) return 0.3 + 1.2*clamp((400-x)/160,0,1) end
local wsh = function(x,y) return 0.7 end
local wglow = function(x,y) return 0.9*clamp((x-380)/150,0,1) + 0.5*clamp((360-y)/90,0,1) end
local wr = function(x,y) return 0.25 end
work(roof_m, {hand="body", piles={{stack_dk,wdk},{stack_sh,wsh},{roof_glow,wglow},{stack_rust,wr}}, angle=function(x,y) return (x<385) and -1.05 or 1.05 end, angle_jitter=0.25, coverage=3, fill=true, edge="firm", length={18,45}, tool={kind="filbert", width=8, lay=2}, seed=31})

--@ chunk 17
local wdk = function(x,y) return 0.3 + 1.0*clamp((400-x)/160,0,1) + 1.5*clamp((520-y)/30,0,1) end
local wsh = function(x,y) return 0.6 end
local wr = function(x,y) return 0.2 + 0.9*clamp((y-520)/90,0,1) end
local wg = function(x,y) return 0.6*clamp((x-420)/110,0,1)*clamp((y-510)/40,0,1) end
work(drum_m, {hand="body", piles={{stack_dk,wdk},{stack_sh,wsh},{stack_rust,wr},{roof_glow,wg}}, angle=1.57, angle_jitter=0.2, coverage=3, fill=true, edge="firm", length={15,40}, tool={kind="filbert", width=7, lay=2}, seed=41})

--@ chunk 18
f_pink = pile{{"lead white",3},{"Naples yellow",1},{"rose madder",0.4},{"orange chrome",0.15}, name="f_pink"}
f_gold = pile{{"lead white",1},{"cadmium yellow",1},{"yellow ochre",0.6},{"orange chrome",0.3}, name="f_gold"}
f_orange = pile{{"orange chrome",1},{"yellow ochre",1},{"Mars orange",0.3},{"lead white",0.4}, name="f_orange"}
f_violet = pile{{"cobalt violet",1},{"cobalt blue",0.5},{"lead white",1},{"rose madder",0.2}, name="f_violet"}
local fm = field_m - stack_m:grow(3) - shadow_m:grow(2)
field_free = fm
local wp = function(x,y) return clamp(1-(y-430)/90,0,1) + 0.05 end
local wg = function(x,y) return math.exp(-((y-520)/80)^2) + 0.2 end
local wo = function(x,y) return clamp((y-540)/150,0,1) + 0.05 end
local wv = function(x,y) return 0.12 + 0.35*clamp((y-560)/200,0,1) end
work(fm, {hand="hatch", piles={{f_pink,wp},{f_gold,wg},{f_orange,wo},{f_violet,wv}}, angle=function(x,y) return 0.03 + 0.06*math.sin(x/120) end, angle_jitter=0.12, coverage=1.3, scale_at=function(x,y) return 0.5 + 1.6*clamp((y-430)/340,0,1) end, mix_jitter=0.5, seed=51})

--@ chunk 19
print(drying(700,450), drying(700,700), drying(100,500), drying(380,560), drying(380,350), drying(150,680), drying(500,100))

--@ chunk 20
local far = field_free * above({{0,540},{1000,545}})
blend(far, {angle=0.02})

--@ chunk 21
local band = field_free * mask(function(x,y) return smoothstep(520,560,y) * (1-smoothstep(600,650,y)) end)
blend(band, {angle=0.02, threshold=0.2})
local band2 = field_free * mask(function(x,y) return smoothstep(600,640,y) * (1-smoothstep(690,740,y)) end)
blend(band2, {angle=0.02, threshold=0.5})

--@ chunk 22
f_deep = pile{{"Mars orange",1},{"burnt sienna",0.5},{"yellow ochre",1},{"cobalt violet",0.4},{"lead white",0.3}, name="f_deep"}
f_cool = pile{{"cobalt violet",1},{"ultramarine blue",0.6},{"lead white",0.8},{"burnt sienna",0.2}, name="f_cool"}
local fg = field_free * mask(function(x,y) return smoothstep(560,640,y) end)
work(fg, {hand="body", piles={{f_deep,function(x,y) return 0.6 end},{f_cool,function(x,y) return 0.15+0.5*clamp((y-620)/150,0,1) end},{f_orange,function(x,y) return 0.4*clamp(x/1000,0,1) end}}, angle=function(x,y) return 0.04+0.05*math.sin(x/90) end, angle_jitter=0.1, coverage=1.1, scale_at=function(x,y) return 0.9 + 1.2*clamp((y-560)/200,0,1) end, length={25,70}, pressure={0.4,0.8}, mix_jitter=0.4, seed=61})

--@ chunk 23
print(wait(4*24*60)); for _,p in ipairs({{380,350},{380,560},{150,680},{700,700},{700,470},{500,100},{500,330}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 24
print(wait(5*24*60)); for _,p in ipairs({{380,350},{300,450},{380,560},{700,700},{450,600}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 25
tr_dark = pile{{"ultramarine blue",1},{"cobalt violet",0.8},{"permanent alizarin",0.25},{"lead white",0.8},{"burnt sienna",0.2}, name="tr_dark"}
tr_light = pile{{"lead white",2.5},{"cobalt violet",1},{"rose madder",0.3},{"cobalt blue",0.3}, name="tr_light"}
-- crowns: an irregular top line for the tree mass
local top = {}
local x = 0
local i = 0
while x <= 1010 do
  local base = 395 + 14*(x/1000)
  local h = 10 + 14*math.abs(math.sin(x/37 + 1.3)) + 8*math.abs(math.sin(x/13))
  if x > 560 and x < 600 then h = h + 22 end -- a clump of poplars
  if x > 120 and x < 140 then h = h + 26 end
  top[#top+1] = {x, base - h*(1 - 0.5*clamp((x-600)/400,0,1))}
  x = x + 9
end
tree_top = top
local crowns = below(top) * above({{0,428},{1000,431}}) - stack_m:grow(2)
treeband_m = crowns
work(crowns, {hand="body", piles={{tr_dark,function(x,y) return 0.2 + clamp(1-x/800,0,1) end},{tr_light,function(x,y) return 0.3 + 1.2*clamp((x-500)/400,0,1) end}}, angle=1.57, angle_jitter=0.5, coverage=2.5, fill=true, length={6,16}, tool={kind="filbert", width=5}, edge={found=0.3, soft=0.7, period=30}, seed=71})

--@ chunk 26
sun_x, sun_y = 835, 372
local halo = treeband_m * mask(function(x,y) local d=math.sqrt((x-sun_x)^2+((y-sun_y)*1.5)^2); return clamp(1-(d-30)/150,0,1) end)
work(halo, {hand="scumble", pile=sk_gold, coverage=1.0, threshold=0.25, pressure={0.2,0.45}, load=0.4, seed=81})
local disc = ellipse(sun_x, sun_y, 19, 18)
work(disc, {hand="detail", pile=sk_sun, coverage=3, fill=true, tool={kind="round", width=5, lay=3}})

--@ chunk 27
local halo = treeband_m * mask(function(x,y) local d=math.sqrt((x-sun_x)^2+((y-sun_y)*1.5)^2); return clamp(1-(d-20)/180,0,1) end)
blend(halo, {angle=1.57, threshold=0.1})
blend(treeband_m, {angle=0.0, threshold=0.4})

--@ chunk 28
print(wait(7*24*60)); for _,p in ipairs({{380,350},{300,450},{380,560},{450,600},{800,400},{100,400}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 29
gl_stack = pile{{"ultramarine blue",1},{"permanent alizarin",0.6},{"burnt sienna",0.3}, medium=0.6, name="gl_stack"}
work(stack_m:shrink(1), {hand="glaze", pile=gl_stack, coverage=1.5, clip=true, tool={kind="filbert", width=14, stiffness=0.3}, length={30,80}, angle=1.4, seed=91})

--@ chunk 30
blend(stack_m:shrink(1), {angle=1.4})
blend(stack_m:shrink(1), {angle=0.3})

--@ chunk 31
straw_mid = pile{{"burnt sienna",1},{"cobalt violet",1},{"rose madder",0.4},{"lead white",1},{"orange chrome",0.3}, name="straw_mid"}
local function ang(x,y)
  if y < 488 then return (x<385) and -1.1 or 1.1 end
  return 1.57 + (x-385)/900
end
local w_mid = function(x,y) return 0.2 + clamp((x-300)/200,0,1) end
local w_glow = function(x,y) return 0.8*clamp((x-430)/110,0,1) end
local w_dk = function(x,y) return 0.6*clamp((360-x)/120,0,1) end
work(stack_m:shrink(4), {hand="body", piles={{straw_mid,w_mid},{roof_glow,w_glow},{stack_dk,w_dk}}, angle=ang, angle_jitter=0.25, coverage=1.0, length={12,30}, tool={kind="filbert", width=4, lay=2}, pressure={0.3,0.6}, clip=stack_m, mix_jitter=0.4, seed=101})

--@ chunk 32
blend(stack_m:shrink(2), {angle=1.3})

--@ chunk 33
print(wait(6*24*60)); for _,p in ipairs({{380,350},{300,550},{480,560},{800,400},{100,400}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 34
print(wait(4*24*60)); for _,p in ipairs({{300,550},{480,560},{400,600}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 35
gl2 = pile{{"ultramarine blue",1},{"permanent alizarin",0.5},{"burnt sienna",0.25}, medium=0.75, name="gl2"}
local sh = stack_m:shrink(1) * mask(function(x,y) return clamp((505-x)/140,0,1) end)
work(sh, {hand="glaze", pile=gl2, coverage=1.2, clip=stack_m:shrink(1), threshold=0.2, tool={kind="filbert", width=14, stiffness=0.3}, length={30,80}, angle=1.45, seed=111})
blend(sh, {angle=1.45, threshold=0.2})

--@ chunk 36
tr_haze = pile{{"lead white",3},{"cobalt violet",1},{"rose madder",0.35},{"Naples yellow",0.5}, name="tr_haze"}
local glowd = function(x,y) local d=math.sqrt((x-sun_x)^2+((y-sun_y)*1.5)^2); return d end
local right = treeband_m * mask(function(x,y) return smoothstep(560,680,x) end)
work(right, {hand="body", piles={{tr_haze,function(x,y) return 1 end},{sk_gold,function(x,y) return 1.4*clamp(1-(glowd(x,y)-25)/110,0,1) end},{tr_light,function(x,y) return 0.4*clamp((glowd(x,y)-120)/150,0,1) end}}, angle=1.57, angle_jitter=0.6, coverage=2.2, fill=true, length={5,14}, tool={kind="filbert", width=5}, edge={found=0.4, soft=0.6, period=25}, seed=121})

--@ chunk 37
local glowd = function(x,y) return math.sqrt((x-sun_x)^2+((y-sun_y)*1.5)^2) end
local right = treeband_m * mask(function(x,y) return (x>515) and 1 or 0 end)
work(right, {hand="body", piles={
  {tr_dark,function(x,y) return 0.8*clamp((700-x)/180,0,1)+0.05 end},
  {tr_light,function(x,y) return 1.0 end},
  {tr_haze,function(x,y) return 0.5*clamp((x-650)/250,0,1) end},
  {sk_gold,function(x,y) return 1.2*clamp(1-(glowd(x,y)-20)/90,0,1) end}},
  angle=1.57, angle_jitter=0.6, coverage=2.0, fill=true, length={5,14}, tool={kind="filbert", width=5}, edge={found=0.4, soft=0.6, period=25}, seed=122})

--@ chunk 38
print(wait(20*60)); for _,p in ipairs({{300,550},{400,400},{480,560}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 39
local sh1 = mask(function(x,y) return stack_m:at(x+16,y-6) end)
rim1 = (stack_m - sh1) * mask(function(x,y) return (x>370 or y<300) and 1 or 0 end)
rim_hot = pile{{"orange chrome",1.5},{"cadmium yellow",0.6},{"vermilion",0.4},{"lead white",0.3}, name="rim_hot"}
work(rim1, {hand="body", piles={{rim_hot,function(x,y) return 1 end},{stack_lit,function(x,y) return 0.6*clamp((y-420)/120,0,1) end}}, angle=function(x,y) return (y<488) and 1.1 or 1.57 end, angle_jitter=0.2, coverage=2.2, length={10,25}, tool={kind="filbert", width=4, lay=2}, clip=stack_m, threshold=0.2, seed=131})

--@ chunk 40
local sh2 = mask(function(x,y) return stack_m:at(x+34,y-13) end)
local band = (stack_m:shrink(2) - sh2):soften(4)
blend(band, {angle=function(x,y) return (y<488) and -0.45 or 0.0 end, threshold=0.2})

--@ chunk 41
stack_plum = pile{{"permanent alizarin",0.6},{"cobalt violet",1},{"burnt sienna",0.6},{"ultramarine blue",0.3},{"lead white",0.5}, name="stack_plum"}
local function ang(x,y)
  if y < 486 then return (x<385) and -1.15 or 1.15 end
  return 1.57 + (x-385)/1200
end
local xr = function(x,y) -- 0 at left edge, 1 at right edge of the stack at that height
  return clamp((x-230)/310,0,1)
end
work(stack_m:shrink(1), {hand="body", piles={
  {stack_dk,function(x,y) local t=xr(x,y); return clamp(1-t/0.45,0,1)+0.03 end},
  {stack_plum,function(x,y) local t=xr(x,y); return math.exp(-((t-0.55)/0.2)^2) end},
  {roof_glow,function(x,y) local t=xr(x,y); return math.exp(-((t-0.8)/0.1)^2)*0.9 end},
  {rim_hot,function(x,y) local t=xr(x,y); return clamp((t-0.82)/0.12,0,1)*1.5 + clamp((330-y)/50,0,1)*clamp((x-370)/40,0,1) end}},
  angle=ang, angle_jitter=0.15, coverage=2.5, fill=true, length={14,34}, tool={kind="filbert", width=6, lay=2.5}, pressure={0.5,0.85}, clip=stack_m, threshold=0.2, mix_jitter=0.3, seed=141})

--@ chunk 42
local ln = poly({{216,476},{242,486},{240,535},{233,512},{226,494}}, true)
local rn = poly({{550,474},{530,486},{533,535},{540,512},{546,494}}, true)
work(ln, {hand="detail", pile=stack_dk, coverage=3, fill=true, angle=1.4, tool={kind="filbert", width=4, lay=2}})
work(rn, {hand="detail", piles={{rim_hot,function() return 1 end},{roof_glow,function() return 0.5 end}}, coverage=3, fill=true, angle=1.7, tool={kind="filbert", width=4, lay=2}})
stack2_m = stack_m + ln + rn

--@ chunk 43
eave_sh = pile{{"ultramarine blue",1},{"permanent alizarin",0.6},{"burnt sienna",0.4},{"cobalt violet",0.5}, name="eave_sh"}
eave_warm = pile{{"vermilion",1},{"permanent alizarin",0.4},{"burnt sienna",0.5},{"orange chrome",0.3}, name="eave_warm"}
local band = stack2_m * mask(function(x,y) local yc = 497 + 6*math.sin((x-230)/310*math.pi); return clamp(1-math.abs(y-yc)/10,0,1) end)
local b = brush{kind="filbert", width=5, lay=1.5}
for i=0,15 do
  local x0 = 236 + i*19
  local t = (x0-230)/310
  local p = t < 0.75 and eave_sh or eave_warm
  b:reload(p, 0.6)
  local yc = 497 + 6*math.sin(t*math.pi)
  b:stroke({{x0,yc-6},{x0+4,yc+1},{x0+12,yc+6}}, {pressure={0.6,0.2}, clip=stack2_m})
  b:stroke({{x0+8,yc-4},{x0+13,yc+3},{x0+20,yc+8}}, {pressure={0.5,0.1}, clip=stack2_m})
end

--@ chunk 44
local band = stack2_m:shrink(3) * mask(function(x,y) local t=(x-230)/310; local yc = 499 + 6*math.sin(t*math.pi); return clamp(1-math.abs(y-yc)/16,0,1) end)
blend(band, {angle=0, threshold=0.1})
blend(band, {angle=1.4, threshold=0.2})

--@ chunk 45
local xr = function(x) return clamp((x-230)/310,0,1) end
local band = stack2_m:shrink(2) * mask(function(x,y) local t=xr(x); local yc = 503 + 6*math.sin(t*math.pi); return clamp(1-math.abs(y-yc)/14,0,1) end)
work(band, {hand="body", piles={
  {stack_dk,function(x,y) local t=xr(x); return clamp(1-t/0.5,0,1)+0.05 end},
  {eave_sh,function(x,y) return 0.35 end},
  {stack_plum,function(x,y) local t=xr(x); return math.exp(-((t-0.6)/0.2)^2) end},
  {eave_warm,function(x,y) local t=xr(x); return clamp((t-0.7)/0.15,0,1) end}},
  angle=1.57, angle_jitter=0.2, coverage=1.6, length={10,22}, tool={kind="filbert", width=4, lay=2}, pressure={0.5,0.8}, clip=stack2_m, threshold=0.3, seed=151})

--@ chunk 46
local xr = function(x) return clamp((x-230)/310,0,1) end
local band = stack2_m:shrink(3) * mask(function(x,y) local t=xr(x); local yc = 503 + 6*math.sin(t*math.pi); local d=y-yc; if d<0 then return clamp(1+d/10,0,1) else return clamp(1-d/40,0,1) end end)
blend(band, {angle=1.57, threshold=0.15})
-- overhanging straw from the roof edge
local b = brush{kind="filbert", width=4, lay=2}
for i=0,22 do
  local x0 = 234 + i*13.5 + rand(-3,3)
  local t = xr(x0)
  local p = (t<0.45) and stack_dk or ((t<0.78) and stack_plum or roof_glow)
  if i%4==0 then b:reload(p,0.5) end
  local yc = 492 + 6*math.sin(t*math.pi)
  b:stroke({{x0,yc-12},{x0+rand(-2,2),yc+rand(2,8)}}, {pressure={0.7,0.15}, clip=stack2_m})
end

--@ chunk 47
local xr = function(x) return clamp((x-230)/310,0,1) end
local m = stack2_m:shrink(2) * mask(function(x,y) local t=xr(x); local yc=505+6*math.sin(t*math.pi); return smoothstep(0.62,0.75,t)*clamp(1-math.abs(y-yc)/16,0,1) end)
work(m, {hand="body", piles={{roof_glow,function(x,y) return 1 end},{rim_hot,function(x,y) return clamp((xr(x)-0.85)/0.1,0,1)*1.5 end},{stack_plum,function() return 0.3 end}}, angle=1.57, angle_jitter=0.2, coverage=2.2, length={10,24}, tool={kind="filbert", width=4, lay=2}, pressure={0.5,0.8}, clip=stack2_m, threshold=0.3, seed=161})

--@ chunk 48
local xr = function(x) return clamp((x-230)/310,0,1) end
local m = stack2_m:shrink(3) * mask(function(x,y) local t=xr(x); local yc=505+6*math.sin(t*math.pi); return smoothstep(0.55,0.7,t)*clamp(1-math.abs(y-yc)/22,0,1) end)
blend(m, {angle=1.57, threshold=0.2})

--@ chunk 49
shadow2_m = poly({{236,598},{200,610},{120,630},{40,648},{0,656},{0,760},{60,742},{160,712},{260,682},{360,656},{450,640},{510,626},{528,612},{470,630},{400,636},{320,630},{250,616}}, true):roughen(5,25,12)
sh_deep = pile{{"ultramarine blue",1},{"cobalt violet",1},{"permanent alizarin",0.35},{"lead white",0.6}, name="sh_deep"}
sh_light = pile{{"cobalt violet",1},{"cobalt blue",0.6},{"rose madder",0.2},{"lead white",1.5}, name="sh_light"}
work(shadow2_m - stack2_m:grow(1), {hand="body", piles={{sh_deep,function(x,y) return 0.5 + clamp((x-150)/300,0,1) end},{sh_light,function(x,y) return 0.6*clamp((250-x)/250,0,1)+0.15 end},{cast_warm,function() return 0.3 end}}, angle=function(x,y) return 0.3 - 0.1*clamp(x/500,0,1) end, angle_jitter=0.12, coverage=2.5, fill=true, edge={found=0.3, soft=0.7, period=40}, length={25,60}, tool={kind="filbert", width=9, lay=1.5}, seed=171})

--@ chunk 50
print(wait(3*24*60)); for _,p in ipairs({{100,700},{300,660},{380,560},{400,350}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 51
st_lit = pile{{"cadmium yellow",1},{"orange chrome",0.6},{"lead white",1},{"Naples yellow",0.6}, name="st_lit"}
st_hot = pile{{"orange chrome",1},{"vermilion",0.3},{"yellow ochre",0.6},{"lead white",0.3}, name="st_hot"}
st_shade = pile{{"cobalt violet",1},{"burnt sienna",0.6},{"rose madder",0.3},{"lead white",0.6},{"ultramarine blue",0.2}, name="st_shade"}
local S = function(x,y) return shadow2_m:at(x,y) end
local depth = function(y) return clamp((y-440)/330,0,1) end
local area = field_m * mask(function(x,y) return smoothstep(470,540,y) end) - stack2_m:grow(2)
work(area, {hand="hatch", piles={
  {st_lit,function(x,y) return (1-S(x,y))*(1.1-0.8*depth(y)) end},
  {st_hot,function(x,y) return (1-S(x,y))*(0.3+0.6*depth(y)) end},
  {st_shade,function(x,y) return (1-S(x,y))*(0.15+0.7*depth(y))+0.3*S(x,y) end},
  {sh_light,function(x,y) return S(x,y)*0.8 end},
  {sh_deep,function(x,y) return S(x,y)*0.8 end}},
  angle=function(x,y) return 0.12*math.sin(x/140) + (y>600 and -0.1 or 0) end, angle_jitter=0.2, coverage=1.1,
  scale_at=function(x,y) return 0.6 + 2.2*depth(y) end, length={8,22}, pressure={0.4,0.8}, mix_jitter=0.4, seed=181})

--@ chunk 52
print(wait(6*24*60)); for _,p in ipairs({{100,700},{300,660},{380,560},{400,350},{700,700},{700,500}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 53
print(wait(6*24*60)); for _,p in ipairs({{100,700},{300,660},{380,560},{400,350},{700,500}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 54
sk_deep = pile{{"lead white",2},{"cobalt blue",1},{"ultramarine blue",0.4},{"cobalt violet",0.3}, name="sk_deep"}
local m = sky_m * mask(function(x,y) return clamp(1-y/170,0,1) end)
work(m, {hand="broad", piles={{sk_deep,function(x,y) return 1 end},{sk_blue,function(x,y) return clamp(y/120,0,1)*0.8 end}}, angle=function(x,y) return 0.04*math.sin(x/130) end, coverage=1.6, threshold=0.15, pressure={0.4,0.8}, seed=191})
-- soft cloud bars catching rose light
sk_cloud = pile{{"lead white",3},{"rose madder",0.5},{"cobalt violet",0.4},{"Naples yellow",0.4}, name="sk_cloud"}
local b = brush{kind="filbert", width=10}
local bars = {{{40,205},{180,198},{330,204}}, {{520,182},{700,175},{900,186}}, {{600,262},{760,256},{960,266}}, {{120,288},{230,284}}, {{430,240},{560,236}}}
for i,pts in ipairs(bars) do b:reload(sk_cloud,0.5); b:stroke(pts,{pressure={0.5,0.25}, shake=1}) end

--@ chunk 55
local m = sky_m * mask(function(x,y) return clamp(1-(y-20)/180,0,1) end)
blend(m, {angle=0.02, threshold=0.05})
blend(m, {angle=0.1, threshold=0.05})

--@ chunk 56
local band = sky_m * mask(function(x,y) return smoothstep(95,135,y)*(1-smoothstep(175,215,y)) end)
work(band, {hand="broad", piles={{sk_rose,function() return 1 end},{sk_cloud,function() return 0.7 end},{sk_blue,function(x,y) return clamp((160-y)/50,0,1) end}}, angle=function(x,y) return 0.03*math.sin(x/110) end, coverage=1.3, threshold=0.2, pressure={0.3,0.6}, load=0.5, seed=201})
local band2 = sky_m * mask(function(x,y) return smoothstep(80,120,y)*(1-smoothstep(200,240,y)) end)
blend(band2, {angle=0.02, threshold=0.05})

--@ chunk 57
local m = sky_m * (ribbon({{420,238},{560,236}},18) + ribbon({{600,260},{760,256},{920,264}},18) + ribbon({{110,287},{235,284}},18) + ribbon({{40,204},{330,204}},18) + ribbon({{520,178},{900,184}},18)):soften(8)
blend(m, {angle=0.0, threshold=0.1})
blend(m, {angle=0.15, threshold=0.1})

--@ chunk 58
pop_c = pile{{"cobalt violet",1},{"ultramarine blue",0.5},{"lead white",1.2},{"rose madder",0.25}, name="pop_c"}
pop_far = pile{{"lead white",2.5},{"cobalt violet",1},{"rose madder",0.3},{"Naples yellow",0.4}, name="pop_far"}
local function poplar(cx, top, base, w, seed)
  local pts = {{cx-w*0.15, base},{cx-w*0.5, base-(base-top)*0.35},{cx-w*0.42, base-(base-top)*0.7},{cx-w*0.12, top+4},{cx, top},{cx+w*0.14, top+6},{cx+w*0.45, base-(base-top)*0.65},{cx+w*0.5, base-(base-top)*0.3},{cx+w*0.2, base}}
  return poly(pts, true):roughen(2.5, 9, seed)
end
local P = {{128,318,392,30},{148,330,392,24},{108,336,392,22},{575,312,400,30},{598,326,400,26},{552,340,400,22},{660,350,405,18},{676,342,405,20},{935,362,410,14},{952,358,411,15}}
pops_m = nil
for i,p in ipairs(P) do local m = poplar(p[1],p[2],p[3],p[4],300+i); pops_m = pops_m and (pops_m + m) or m end
pops_m = pops_m - stack2_m:grow(2)
local xfar = function(x) return clamp((x-450)/450,0,1) end
work(pops_m, {hand="body", piles={{tr_dark,function(x,y) return 0.6*(1-xfar(x)) + 0.05 end},{pop_c,function(x,y) return 0.8 end},{pop_far,function(x,y) return 1.2*xfar(x) end}}, angle=1.57, angle_jitter=0.3, coverage=2.5, fill=true, length={6,14}, tool={kind="filbert", width=4}, edge={found=0.5, soft=0.5, period=15}, seed=211})

--@ chunk 59
print(wait(5*24*60)); for _,p in ipairs({{100,700},{300,660},{380,560},{400,350},{128,360},{500,150}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 60
gl_sh = pile{{"ultramarine blue",1},{"cobalt violet",0.6},{"permanent alizarin",0.3}, medium=0.7, name="gl_sh"}
local m = shadow2_m:shrink(2) - stack2_m:grow(1)
work(m, {hand="glaze", pile=gl_sh, coverage=1.2, threshold=0.3, clip=shadow2_m, load_at=function(x,y) return 0.35 + 0.5*clamp((x-50)/400,0,1) end, angle=0.3, tool={kind="filbert", width=12, stiffness=0.3}, length={40,100}, seed=221})
blend(m, {angle=0.3, threshold=0.3})

--@ chunk 61
sh_stub = pile{{"cobalt violet",1},{"burnt sienna",0.4},{"lead white",0.9},{"ultramarine blue",0.3},{"orange chrome",0.15}, name="sh_stub"}
local depth = function(y) return clamp((y-440)/330,0,1) end
work(shadow2_m:grow(2) - stack2_m:grow(1), {hand="hatch", piles={{sh_stub,function() return 1 end},{st_shade,function() return 0.6 end},{sh_light,function(x,y) return 0.6*clamp((300-x)/300,0,1) end},{cast_warm,function() return 0.4 end}},
  angle=function(x,y) return 0.12*math.sin(x/140) - 0.1 end, angle_jitter=0.2, coverage=1.4, clip=shadow2_m:grow(3):soften(3), threshold=0.3,
  scale_at=function(x,y) return 0.6 + 2.2*depth(y) end, length={8,22}, pressure={0.4,0.8}, mix_jitter=0.4, seed=231})

--@ chunk 62
blend(shadow2_m:grow(1) - stack2_m:grow(1), {angle=0.05, threshold=0.3})

--@ chunk 63
print(wait(5*24*60)); for _,p in ipairs({{100,700},{300,660},{200,650},{128,360}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 64
wait(3*24*60)
sh_dk2 = pile{{"ultramarine blue",1},{"cobalt violet",1},{"burnt sienna",0.4},{"permanent alizarin",0.3},{"lead white",0.5}, name="sh_dk2"}
sh_mid2 = pile{{"cobalt violet",1},{"ultramarine blue",0.5},{"burnt sienna",0.3},{"lead white",0.9},{"rose madder",0.2}, name="sh_mid2"}
local depth = function(y) return clamp((y-440)/330,0,1) end
local near = function(x,y) return clamp((x-100)/350,0,1) end
work(shadow2_m - stack2_m:grow(1), {hand="hatch", piles={{sh_dk2,function(x,y) return 0.4+0.8*near(x,y) end},{sh_mid2,function(x,y) return 1.0-0.5*near(x,y) end}},
  angle=function(x,y) return 0.12*math.sin(x/140) - 0.1 end, angle_jitter=0.2, coverage=1.0, clip=shadow2_m:soften(2), threshold=0.3,
  scale_at=function(x,y) return 0.6 + 2.2*depth(y) end, length={8,22}, pressure={0.4,0.8}, mix_jitter=0.4, seed=241})

--@ chunk 65
ds_m = poly({{712,476},{716,458},{726,440},{740,428},{752,424},{764,429},{776,442},{785,458},{789,474},{786,478},{787,494},{752,498},{716,495},{714,480}}, true)
ds2_m = poly({{858,470},{862,458},{870,447},{879,442},{888,447},{895,458},{898,470},{896,483},{878,486},{860,484}}, true)
ds_dk = pile{{"cobalt violet",1},{"ultramarine blue",0.4},{"rose madder",0.3},{"lead white",1.2},{"burnt sienna",0.2}, name="ds_dk"}
local all = ds_m + ds2_m
local xr = function(x) if x < 820 then return clamp((x-712)/77,0,1) else return clamp((x-858)/40,0,1) end end
work(all, {hand="detail", piles={{ds_dk,function(x,y) return 1.2-xr(x) end},{roof_glow,function(x,y) return math.exp(-((xr(x)-0.75)/0.15)^2) end},{rim_hot,function(x,y) return clamp((xr(x)-0.8)/0.15,0,1)*1.5 end}}, angle=1.57, coverage=3, fill=true, tool={kind="filbert", width=3, lay=1.5}, seed=251})
-- their shadows
local s1 = poly({{716,488},{660,496},{590,506},{596,511},{680,505},{760,497}}, true)
local s2 = poly({{860,480},{820,486},{790,491},{795,494},{840,490},{880,485}}, true)
work(s1+s2, {hand="detail", pile=sh_mid2, angle=0.1, coverage=2.5, fill=true, tool={kind="filbert", width=3}, seed=252})

--@ chunk 66
-- shorten the fish-tail shadows with field paint
local cover = poly({{560,496},{640,494},{650,516},{560,518}}, true) + poly({{780,484},{815,483},{818,497},{780,498}}, true)
work(cover, {hand="detail", piles={{st_lit,function() return 1 end},{f_gold,function() return 0.6 end},{st_hot,function() return 0.3 end}}, angle=0.05, coverage=3, fill=true, tool={kind="filbert", width=3}, seed=261})
-- straw texture on distant stacks
local all = ds_m + ds2_m
local xr = function(x) if x < 820 then return clamp((x-712)/77,0,1) else return clamp((x-858)/40,0,1) end end
work(all:shrink(1), {hand="hatch", piles={{ds_dk,function(x,y) return 1.0-xr(x) end},{stack_plum,function() return 0.4 end},{roof_glow,function(x,y) return 0.8*xr(x) end}}, angle=function(x,y) return 1.57 + (x<820 and (x-750)/120 or (x-878)/60) end, coverage=0.8, length={4,8}, tool={kind="round", width=1.6}, clip=all, mix_jitter=0.5, seed=262})

--@ chunk 67
local r = rag{width=30}
r:dip(0.6)
local m = rect(555,420,350,102)
r:wipe(m, {pressure=0.7, angle=0, passes=3, refold=0.3})
print(drying(750,460), drying(600,505))

--@ chunk 68
for i=1,3 do
local r = rag{width=30}
r:dip(0.8)
r:wipe(rect(555,415,350,108), {pressure=0.85, angle=0, passes=2, refold=0.15})
end

--@ chunk 69
wait(30)
ds_m = poly({{700,474},{705,458},{714,444},{725,436},{733,434},{742,438},{752,450},{759,463},{762,474},{758,478},{759,490},{731,493},{703,490},{702,478}}, true):roughen(1.5,6,5)
local xr = function(x) return clamp((x-700)/62,0,1) end
work(ds_m, {hand="detail", piles={{ds_dk,function(x,y) return 1.3-1.2*xr(x) end},{stack_plum,function(x,y) return 0.5 end},{roof_glow,function(x,y) return math.exp(-((xr(x)-0.75)/0.15)^2) end},{rim_hot,function(x,y) return clamp((xr(x)-0.82)/0.12,0,1)*1.6 end}}, angle=function(x,y) return 1.57+(x-731)/90 end, coverage=3.5, fill=true, length={4,10}, tool={kind="filbert", width=2.5, lay=1.5}, mix_jitter=0.4, seed=271})
local b = brush{kind="filbert", width=4}
b:reload(sh_mid2,0.5)
b:stroke({{708,491},{680,495},{650,499}}, {pressure={0.7,0.2}})
b:reload(ds_dk,0.4)
b:stroke({{712,489},{690,492}}, {pressure={0.6,0.3}})

--@ chunk 70
local cover = ellipse(898,467,12,16)
work(cover, {hand="detail", piles={{field_far,function() return 0.6 end},{f_gold,function() return 0.5 end},{st_lit,function() return 0.6 end}}, angle=0.05, coverage=3, fill=true, tool={kind="filbert", width=3}, seed=281})
local s = poly({{705,486},{670,492},{640,498},{660,501},{700,497},{735,494}}, true)
work(s, {hand="detail", pile=sh_mid2, angle=0.08, coverage=2, fill=true, tool={kind="filbert", width=3}, seed=282})

--@ chunk 71
print(drying(100,410), drying(600,400), drying(300,440), drying(700,600))
tr_warm = pile{{"cobalt violet",1},{"rose madder",0.4},{"lead white",1.3},{"Naples yellow",0.5},{"burnt sienna",0.15}, name="tr_warm"}
tr_deep = pile{{"ultramarine blue",1},{"cobalt violet",0.8},{"burnt sienna",0.4},{"permanent alizarin",0.2},{"lead white",0.5}, name="tr_deep"}
local left = (treeband_m + pops_m) * mask(function(x,y) return 1 - smoothstep(480,560,x) end)
work(left, {hand="body", piles={{tr_warm,function(x,y) return 0.5 + clamp((400-y)/30,0,1) end},{tr_dark,function() return 0.6 end},{tr_deep,function(x,y) return clamp((y-405)/20,0,1) end}}, angle=1.57, angle_jitter=0.7, coverage=1.2, length={5,12}, tool={kind="filbert", width=4}, pressure={0.3,0.6}, load=0.5, threshold=0.4, mix_jitter=0.5, seed=291})

--@ chunk 72
local left = (treeband_m + pops_m) * mask(function(x,y) return 1 - smoothstep(480,560,x) end)
blend(left:shrink(1), {angle=1.57, threshold=0.4})

--@ chunk 73
local b = brush{kind="filbert", width=4}
-- hedge line: slightly darker dashes along the foot of the trees
for i=0,30 do
  local x0 = i*32 + rand(-6,6)
  if not (x0 > 215 and x0 < 550) then
    b:reload(i%3==0 and tr_deep or tr_dark, 0.35)
    local y0 = 426 + 3*(x0/1000) + rand(-2,2)
    b:stroke({{x0,y0},{x0+rand(15,30),y0+rand(-1,1)}}, {pressure={0.5,0.3}})
  end
end
-- warm field strokes overlapping the foot
for i=0,40 do
  local x0 = i*25 + rand(-8,8)
  if not (x0 > 215 and x0 < 550) then
    local sun = math.exp(-((x0-sun_x)/120)^2)
    b:reload(sun > 0.4 and sk_gold or f_pink, 0.4)
    local y0 = 432 + 3*(x0/1000) + rand(-1,2)
    b:stroke({{x0,y0},{x0+rand(18,36),y0+rand(-1,1)}}, {pressure={0.4,0.2}})
  end
end

--@ chunk 74
local band = mask(function(x,y) local yc = 429 + 3*(x/1000); return clamp(1-math.abs(y-yc)/9,0,1) end) - stack2_m:grow(3)
blend(band, {angle=0.0, threshold=0.2})
blend(band, {angle=1.2, threshold=0.2})
-- fix the patch at the erased ghost (898,467)
local cover = ellipse(898,467,16,20):soften(5)
blend(cover, {angle=0.0, threshold=0.2})

--@ chunk 75
for k=1,2 do
local r = rag{width=14}
r:dip(0.8)
local x = 550
while x < 1000 do
  r:wipe({{x,429+3*x/1000},{math.min(x+120,1000),429+3*math.min(x+120,1000)/1000}}, {pressure=0.8})
  r:refold()
  x = x + 110
end
x = 0
while x < 215 do
  r:wipe({{x,429},{math.min(x+110,212),429}}, {pressure=0.8})
  r:refold()
  x = x + 100
end
end

--@ chunk 76
print(wait(3*24*60)); print(drying(898,467), drying(560,505), drying(730,460), drying(300,430), drying(700,429))

--@ chunk 77
print(wait(5*24*60)); print(drying(898,467), drying(560,505), drying(730,460), drying(650,495))

--@ chunk 78
local depth = function(y) return clamp((y-430)/340,0,1) end
local sunlane = function(x,y) return math.exp(-((x-sun_x)/140)^2) end
local area = field_m * mask(function(x,y) return 1-smoothstep(520,580,y) end) - stack2_m:grow(3) - ds_m:grow(3) - shadow2_m
work(area, {hand="hatch", piles={
  {field_far,function(x,y) return 0.8*(1-depth(y)*2) + 0.1 end},
  {f_pink,function(x,y) return 0.5 end},
  {st_lit,function(x,y) return 0.6 + 0.8*sunlane(x,y) end},
  {f_gold,function(x,y) return 0.4 end},
  {st_shade,function(x,y) return 0.25 end}},
  angle=function(x,y) return 0.04*math.sin(x/90) end, angle_jitter=0.1, coverage=0.9,
  scale_at=function(x,y) return 0.45 + 2.0*depth(y) end, length={8,20}, pressure={0.3,0.6}, load=0.5, mix_jitter=0.5, seed=301})

--@ chunk 79
print(drying(300,510), drying(450,505), drying(238,580), drying(400,400))

--@ chunk 80
local xr = function(x) return clamp((x-230)/310,0,1) end
-- shadow cast by the overhang on the top of the drum
local under = stack2_m:shrink(2) * mask(function(x,y) local t=xr(x); local yc=503+6*math.sin(t*math.pi); local d=y-yc; if d<0 then return 0 end; return clamp(1-d/26,0,1) end)
work(under, {hand="body", piles={
  {eave_sh,function(x,y) return 1.2*clamp(1-xr(x)/0.7,0,1)+0.1 end},
  {stack_dk,function(x,y) return 0.5 end},
  {eave_warm,function(x,y) return 1.2*clamp((xr(x)-0.6)/0.25,0,1) end}},
  angle=1.57, angle_jitter=0.15, coverage=2.2, fill=true, length={8,22}, tool={kind="filbert", width=4, lay=1.5}, pressure={0.5,0.8}, clip=stack2_m, threshold=0.25, load_at=function(x,y) return 0.6 end, seed=311})

--@ chunk 81
local xr = function(x) return clamp((x-230)/310,0,1) end
local m = stack2_m:shrink(2) * mask(function(x,y) local t=xr(x); local yc=503+6*math.sin(t*math.pi); local d=y-yc; if d<-4 then return 0 end; return clamp(1-(d-10)/35,0,1) end)
blend(m, {angle=1.57, threshold=0.1})
blend(m, {angle=1.4, threshold=0.2})

--@ chunk 82
gl_fg = pile{{"burnt sienna",1},{"permanent alizarin",0.4},{"cobalt violet",0.3}, medium=0.75, name="gl_fg"}
local m = field_m * mask(function(x,y) return smoothstep(620,769,y) end) - stack2_m:grow(2)
print(drying(800,700), drying(100,720))
work(m, {hand="glaze", pile=gl_fg, coverage=1.0, threshold=0.15, load_at=function(x,y) return 0.25+0.45*clamp((y-640)/130,0,1) end, angle=0.05, tool={kind="filbert", width=16, stiffness=0.3}, length={80,200}, seed=321})
blend(m, {angle=0.05, threshold=0.15})

--@ chunk 83
print(wait(7*24*60)); for _,p in ipairs({{300,520},{450,525},{100,700},{800,700},{700,480},{300,470}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 84
local n = noise{seed=9, period=40, octaves=3}
shadow3_m = mask(function(x,y) local v = shadow2_m:at(x,y); return v end):blur(6):map(function(v) return v end)
local sm = mask(function(x,y) return clamp((shadow3_m:at(x,y) - 0.5 + 0.35*n(x,y))*4 + 0.5, 0, 1) end)
shadow_broken = sm
local depth = function(y) return clamp((y-430)/340,0,1) end
local nearbase = function(x,y) return clamp(1 - ((x-380)^2/220^2 + (y-625)^2/40^2)^0.5 ,0,1) end
work(sm - stack2_m:grow(1), {hand="hatch", piles={
  {sh_dk2,function(x,y) return 0.5 + 1.5*nearbase(x,y) end},
  {sh_mid2,function(x,y) return 0.8 end},
  {sh_deep,function(x,y) return 0.5 end},
  {cast_warm,function(x,y) return 0.3 end}},
  angle=function(x,y) return 0.12*math.sin(x/140) - 0.12 end, angle_jitter=0.2, coverage=1.3, threshold=0.4,
  scale_at=function(x,y) return 0.6 + 2.2*depth(y) end, length={8,22}, pressure={0.4,0.8}, mix_jitter=0.4, seed=331})

--@ chunk 85
print(wait(5*24*60)); for _,p in ipairs({{100,700},{300,650},{450,630}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 86
wait(3*24*60)
shadow4_m = poly({{238,598},{170,610},{80,626},{0,640},{0,769},{150,769},{250,728},{350,682},{440,648},{510,628},{530,612},{460,630},{380,636},{300,628}}, true):roughen(7,30,41)
sh_core = pile{{"ultramarine blue",1},{"cobalt violet",0.8},{"permanent alizarin",0.35},{"burnt sienna",0.25},{"lead white",0.35}, name="sh_core"}
local depth = function(y) return clamp((y-430)/340,0,1) end
local n2 = noise{seed=17, period=60, octaves=3}
work(shadow4_m - stack2_m:grow(1), {hand="hatch", piles={
  {sh_core,function(x,y) return 0.9 + 0.4*n2(x,y) end},
  {sh_dk2,function(x,y) return 0.8 end},
  {sh_mid2,function(x,y) return 0.35 + 0.4*clamp((120-x)/120,0,1) end},
  {eave_warm,function(x,y) return 0.12 end}},
  angle=function(x,y) return 0.1*math.sin(x/140) - 0.18 end, angle_jitter=0.2, coverage=1.8, fill=true, threshold=0.4,
  scale_at=function(x,y) return 0.7 + 2.2*depth(y) end, length={10,26}, pressure={0.5,0.85}, mix_jitter=0.4, seed=341})

--@ chunk 87
local m = shadow4_m - stack2_m:grow(4)
local lowerEdge = function(x,y) if y > 600 + (x/530)*15 + 20 and x > 120 then return 1 else return 0.2 end end
local n = lose(m, {pile=st_hot, where=lowerEdge, tool="filbert 5", reach={14,18}, load=0.3, pressure={0.4,0.05}, seed=351})
print(n)

--@ chunk 88
local e = (shadow4_m:grow(14) - shadow4_m:shrink(14)) - stack2_m:grow(2)
blend(e, {angle=-0.2, threshold=0.3})

--@ chunk 89
print(wait(9*24*60)); for _,p in ipairs({{300,650},{400,660},{200,600},{100,700}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 90
local depth = function(y) return clamp((y-430)/340,0,1) end
local outer = (shadow4_m:grow(16) - shadow4_m) - stack2_m:grow(2)
work(outer, {hand="hatch", piles={
  {st_hot,function(x,y) return 0.8 end},
  {f_deep,function(x,y) return 0.4+0.8*depth(y) end},
  {st_lit,function(x,y) return 0.8*(1-depth(y)) + 0.1 end},
  {f_gold,function(x,y) return 0.3 end}},
  angle=function(x,y) return 0.1*math.sin(x/140) - 0.12 end, angle_jitter=0.2, coverage=1.6, fill=true, threshold=0.4,
  scale_at=function(x,y) return 0.6 + 2.0*depth(y) end, length={8,22}, pressure={0.45,0.8}, mix_jitter=0.4, seed=361})
local inner = (shadow4_m - shadow4_m:shrink(18)) - stack2_m:grow(2)
work(inner, {hand="hatch", piles={
  {sh_core,function(x,y) return 0.9 end},
  {sh_dk2,function(x,y) return 0.6 end},
  {sh_mid2,function(x,y) return 0.4 end}},
  angle=function(x,y) return 0.1*math.sin(x/140) - 0.18 end, angle_jitter=0.2, coverage=1.6, fill=true, threshold=0.4,
  scale_at=function(x,y) return 0.6 + 2.0*depth(y) end, length={8,22}, pressure={0.45,0.8}, mix_jitter=0.4, seed=362})

--@ chunk 91
rim_light = pile{{"lead white",1},{"cadmium yellow",1},{"orange chrome",0.4}, name="rim_light"}
local b = brush{kind="round", width=3, point=0.8, lay=2}
b:load(rim_light, 0.7)
b:gesture({{372,272,0.15},{390,268,0.35},{412,276,0.5},{440,300,0.55},{470,335,0.5},{496,372,0.55}}, {wobble=1.5})
b:load(rim_light, 0.7)
b:gesture({{494,368,0.4},{515,405,0.55},{530,440,0.5},{543,470,0.4},{548,478,0.15}}, {wobble=1.5})
b:load(rim_hot, 0.6)
b:gesture({{550,490,0.3},{545,510,0.4},{537,540,0.45},{536,575,0.45},{531,600,0.35},{524,614,0.1}}, {wobble=1.2})

--@ chunk 92
local sh1 = mask(function(x,y) return stack2_m:at(x+10,y-4) end)
local band = ((stack2_m:grow(3) - sh1) * mask(function(x,y) return (x>375) and 1 or 0 end)):soften(2)
blend(band, {angle=function(x,y) return (y<488) and 1.0 or 1.57 end, threshold=0.2})
-- straw wisps catching the sun along the silhouette
local b = brush{kind="round", width=1.6, point=0.9}
local edge = {{392,270},{415,279},{438,298},{460,322},{480,348},{500,377},{515,405},{528,433},{540,462}}
for i,p in ipairs(edge) do
  for k=1,3 do
    b:reload(k==2 and rim_hot or rim_light, 0.5)
    local a = -0.9 + (i/#edge)*1.0 + rand(-0.5,0.5)
    local L = rand(5,11)
    local x0, y0 = p[1]+rand(-6,6), p[2]+rand(-3,3)
    b:gesture({{x0-4*math.cos(a), y0-4*math.sin(a), 0.5},{x0+L*math.cos(a), y0+L*math.sin(a), 0.02}})
  end
end

--@ chunk 93
for k=1,2 do
local r = rag{width=16}
r:dip(0.8)
r:wipe({{378,266},{410,272},{440,298},{470,335}}, {pressure=0.8}); r:refold()
r:wipe({{465,330},{495,372},{518,412},{535,450},{550,480}}, {pressure=0.8}); r:refold()
r:wipe({{552,486},{546,520},{540,560},{536,600},{526,616}}, {pressure=0.8}); r:refold()
end

--@ chunk 94
print(wait(4*24*60)); print(drying(500,380), drying(540,470), drying(420,280), drying(300,650))

--@ chunk 95
local sh1 = mask(function(x,y) return stack2_m:at(x+13,y-5) end)
local band = (stack2_m - sh1) * mask(function(x,y) return (x>372) and 1 or 0 end)
work(band, {hand="detail", piles={{rim_hot,function(x,y) return 1 end},{st_hot,function(x,y) return 0.5 end},{rim_light,function(x,y) return 0.25 + 0.6*clamp((330-y)/60,0,1) end}},
  angle=function(x,y) return (y<488) and 1.05 or 1.57 end, angle_jitter=0.15, coverage=3, fill=true, length={6,16}, tool={kind="filbert", width=3, lay=2}, pressure={0.5,0.8}, clip=stack2_m, threshold=0.2, mix_jitter=0.3, seed=371})

--@ chunk 96
local sh1 = mask(function(x,y) return stack2_m:at(x+13,y-5) end)
local sh2 = mask(function(x,y) return stack2_m:at(x+26,y-10) end)
local inner = (stack2_m:shrink(2) * sh1 - sh2):soften(4) * mask(function(x,y) return (x>372) and 1 or 0 end)
blend(inner, {angle=function(x,y) return (y<488) and -0.55 or 0.0 end, threshold=0.2})
-- outer olive line: overpaint with sky gold just outside the roof edge
local outer = (stack2_m:grow(4) - stack2_m) * mask(function(x,y) return (x>372 and y<486) and 1 or 0 end)
work(outer, {hand="detail", pile=sk_gold, angle=1.05, coverage=3, fill=true, tool={kind="filbert", width=2.5}, clip=outer, threshold=0.2, seed=381})
local outer2 = (stack2_m:grow(4) - stack2_m) * mask(function(x,y) return (x>500 and y>=486 and y<625) and 1 or 0 end)
work(outer2, {hand="detail", piles={{st_lit,function() return 1 end},{f_gold,function() return 0.5 end}}, angle=1.57, coverage=3, fill=true, tool={kind="filbert", width=2.5}, clip=outer2, threshold=0.2, seed=382})

--@ chunk 97
print(wait(5*24*60)); print(drying(500,380), drying(540,470), drying(420,280), drying(530,560))

--@ chunk 98
print(wait(6*24*60)); print(drying(500,380), drying(540,470), drying(420,280), drying(530,560))

--@ chunk 99
print(wait(5*24*60)); print(drying(500,380), drying(540,470), drying(530,560))

--@ chunk 100
local sh1 = mask(function(x,y) return stack2_m:at(x+13,y-5) end)
local zone = (stack2_m:shrink(3) - mask(function(x,y) return stack2_m:at(x+34,y-13) end)) * mask(function(x,y) return (x>365) and 1 or 0 end)
local function ang(x,y) if y < 488 then return 1.12 end return 1.57 end
work(zone, {hand="hatch", piles={{roof_glow,function() return 1 end},{stack_plum,function() return 0.5 end},{st_hot,function() return 0.5 end},{stack_rust,function() return 0.3 end}}, angle=ang, angle_jitter=0.12, coverage=1.0, length={10,24}, tool={kind="round", width=2.4, lay=1.5}, pressure={0.4,0.8}, clip=stack2_m, threshold=0.3, mix_jitter=0.4, seed=391})

--@ chunk 101
local b = brush{kind="round", width=2.2, point=0.7, lay=1.5}
local edge = {}
-- walk along the right silhouette sampling the mask
for y=272,612,7 do
  local xe = nil
  for x=560,370,-1 do if stack2_m:at(x,y) > 0.5 then xe = x; break end end
  if xe then edge[#edge+1] = {xe, y} end
end
for i,p in ipairs(edge) do
  local roof = p[2] < 488
  local a = roof and (1.12 + rand(-0.25,0.25)) or (1.57 + rand(-0.2,0.2))
  local pile_ = (i%3==0) and rim_light or ((i%3==1) and roof_glow or rim_hot)
  b:reload(pile_, 0.45)
  local L = rand(8,16)
  local x0, y0 = p[1]+rand(-1,3), p[2]
  b:gesture({{x0 - L*math.cos(a), y0 - L*math.sin(a), 0.6},{x0+2, y0+2, 0.35},{x0 + 0.3*L*math.cos(a)+rand(1,4), y0+0.3*L*math.sin(a), 0.02}})
end

--@ chunk 102
print(wait(6*24*60)); print(drying(500,380), drying(540,470), drying(530,560), drying(400,560))

--@ chunk 103
wait(3*24*60)
local xr = function(x) return clamp((x-232)/305,0,1) end
local drum = stack2_m:shrink(2) * mask(function(x,y) local yc=503+6*math.sin(xr(x)*math.pi); return smoothstep(yc+2, yc+14, y) end)
work(drum, {hand="hatch", piles={
  {stack_dk,function(x,y) return 1.4*clamp(1-xr(x)/0.55,0,1)+0.1 end},
  {sh_core,function(x,y) return 0.5*clamp(1-xr(x)/0.4,0,1) end},
  {stack_plum,function(x,y) return math.exp(-((xr(x)-0.6)/0.2)^2) + 0.2 end},
  {eave_warm,function(x,y) return 0.6*math.exp(-((xr(x)-0.82)/0.08)^2) end},
  {rim_hot,function(x,y) return 1.4*clamp((xr(x)-0.88)/0.08,0,1) end}},
  angle=function(x,y) return 1.57 + (x-385)/1100 end, angle_jitter=0.1, coverage=1.6, fill=true, length={10,24}, tool={kind="round", width=2.6, lay=1.5}, pressure={0.5,0.85}, clip=stack2_m, threshold=0.3, mix_jitter=0.4, seed=401})

--@ chunk 104
sun_core = pile{{"lead white",4},{"lemon chrome",0.5}, name="sun_core"}
local halo = (sky_m - treeband_m - pops_m) * mask(function(x,y) local d=math.sqrt((x-sun_x)^2+((y-sun_y)*1.3)^2); return clamp(1-(d-18)/70,0,1) end)
work(halo, {hand="scumble", piles={{sk_sun,function() return 1 end},{sun_core,function(x,y) local d=math.sqrt((x-sun_x)^2+((y-sun_y)*1.3)^2); return clamp(1-(d-18)/35,0,1) end}}, coverage=1.2, threshold=0.2, pressure={0.25,0.5}, load=0.45, tool={kind="filbert", width=6}, seed=411})
blend(halo:grow(10):soften(8), {angle=0.0, threshold=0.05})
local b = brush{kind="round", width=5, lay=3}
for i=1,6 do b:reload(sun_core,0.8); b:touch(sun_x+rand(-6,6), sun_y+rand(-5,4), {pressure=0.8}) end

--@ chunk 105
for k=1,3 do
local r = rag{width=30}
r:dip(0.8)
r:wipe(ellipse(sun_x, sun_y-5, 95, 60), {pressure=0.75, angle=0, passes=2, refold=0.15})
end

--@ chunk 106
print(wait(3*24*60)); print(drying(sun_x,sun_y), drying(700,330), drying(200,300), drying(800,395))

--@ chunk 107
gl_sky = pile{{"Indian yellow",1},{"rose madder",0.25},{"Mars orange",0.15}, medium=0.8, name="gl_sky"}
local disc = ellipse(sun_x, sun_y, 26, 24):soften(6)
local m = (sky_m:grow(2) * mask(function(x,y) return smoothstep(225,300,y) end)) - disc - stack2_m:grow(2)
local glowf = function(x,y) local d=math.sqrt((x-sun_x)^2+((y-sun_y)*1.5)^2); return d end
work(m, {hand="glaze", pile=gl_sky, coverage=1.0, threshold=0.2, load_at=function(x,y) return 0.25 + 0.25*clamp((glowf(x,y)-40)/300,0,1) end, angle=0.0, tool={kind="filbert", width=18, stiffness=0.3}, length={100,240}, seed=421})
blend(m, {angle=0.0, threshold=0.2})

--@ chunk 108
for k=1,3 do
local r = rag{width=24}
r:dip(0.8)
r:wipe(stack2_m:shrink(3) * above({{0,470},{1000,470}}), {pressure=0.75, angle=1.1, passes=1, refold=0.15})
r:wipe(((treeband_m + pops_m):shrink(3)) * mask(function(x,y) return (x<540) and 1 or 0 end), {pressure=0.75, angle=0, passes=1, refold=0.15})
end

--@ chunk 109
print(wait(5*24*60)); print(drying(sun_x,sun_y+30), drying(700,330), drying(200,300), drying(800,429))

--@ chunk 110
local b = brush{kind="filbert", width=8, lay=4}
-- sun disc, thick and pale
local disc = ellipse(sun_x, sun_y, 18, 17)
work(disc, {hand="detail", piles={{sun_core,function() return 1 end},{sk_sun,function() return 0.3 end}}, coverage=4, fill=true, angle=0, tool={kind="filbert", width=5, lay=4}, clip=disc, seed=431})
-- radiance: soft light scumble ring
local ring = (sky_m - treeband_m - pops_m) * mask(function(x,y) local d=math.sqrt((x-sun_x)^2+((y-sun_y)*1.25)^2); return (d>18) and clamp(1-(d-18)/55,0,1) or 0 end)
work(ring, {hand="scumble", pile=sk_sun, coverage=0.8, threshold=0.25, pressure={0.15,0.35}, load=0.3, tool={kind="filbert", width=5}, seed=432})
-- the hedge dashes: cover with haze strokes
local band = mask(function(x,y) local yc = 429 + 3*(x/1000); return clamp(1-math.abs(y-yc)/5,0,1) end) - stack2_m:grow(4)
work(band, {hand="detail", piles={{tr_haze,function(x,y) return 1 end},{field_far,function() return 0.6 end},{tr_light,function(x,y) return clamp((560-x)/200,0,1) end}}, angle=0, coverage=2.5, fill=true, length={10,30}, tool={kind="filbert", width=3}, threshold=0.4, seed=433})

--@ chunk 111
local ringm = ellipse(sun_x, sun_y, 75, 62) - ellipse(sun_x, sun_y, 20, 19)
for k=1,3 do
local r = rag{width=20}
r:dip(0.8)
r:wipe(ringm, {pressure=0.75, angle=0, passes=1, refold=0.15})
local x = 0
while x < 1000 do
  local x1 = math.min(x+90,1000)
  if not (x1 > 225 and x < 545) then r:wipe({{x,429+3*x/1000},{x1,429+3*x1/1000}}, {pressure=0.85}) end
  r:refold(); x = x + 80
end
end

--@ chunk 112
print(wait(8*24*60)); print(drying(400,640), drying(300,630), drying(sun_x,sun_y), drying(600,430))

--@ chunk 113
-- contact shadow under the drum: a band hugging the base, dark at the stack, merging into the cast shadow
local base = {}
for x=236,526,6 do
  local yb = nil
  for y=660,560,-1 do if stack2_m:at(x,y) > 0.5 then yb = y; break end end
  if yb then base[#base+1] = {x, yb} end
end
local pts = {}
for i,p in ipairs(base) do pts[#pts+1] = {p[1], p[2]+3} end
local w = {}
for i,p in ipairs(base) do local t=(p[1]-236)/290; w[#w+1] = 14*(1-0.6*t) end
local contact = ribbon(pts, w):roughen(3,14,77) - stack2_m
work(contact, {hand="hatch", piles={{sh_core,function() return 1 end},{sh_dk2,function() return 0.6 end},{eave_sh,function(x,y) return 0.4 end}}, angle=function(x,y) return 0.05 end, angle_jitter=0.2, coverage=2.2, fill=true, length={8,18}, tool={kind="round", width=3, lay=1.5}, pressure={0.5,0.85}, clip=contact:grow(2):soften(2), threshold=0.3, seed=441})

--@ chunk 114
local zone = mask(function(x,y)
  if x < 225 or x > 540 then return 0 end
  local yb = 634 - 16*((x-385)/150)^2
  return clamp(1-math.abs(y-(yb+6))/18,0,1)
end) - stack2_m:shrink(0)
blend(zone, {angle=0.1, threshold=0.1})
blend(zone, {angle=1.4, threshold=0.2})

--@ chunk 115
for k=1,4 do
local r = rag{width=18}
r:dip(0.9)
r:wipe({{220,608},{280,624},{340,634},{400,638},{460,634},{510,624},{545,612}}, {pressure=0.85}); r:refold()
r:wipe({{545,622},{480,642},{400,650},{320,646},{230,628},{215,612}}, {pressure=0.85}); r:refold()
end

--@ chunk 116
print(wait(7*24*60)); print(drying(400,640), drying(500,632), drying(250,620))

--@ chunk 117
local depth = function(y) return clamp((y-430)/340,0,1) end
-- right part under the base: lit field again
local rightz = poly({{420,628},{470,624},{515,614},{540,604},{560,610},{565,640},{500,650},{440,655},{420,648}}, true) - stack2_m:grow(1) - shadow4_m
work(rightz, {hand="hatch", piles={{st_hot,function() return 0.9 end},{f_deep,function() return 0.5 end},{st_lit,function() return 0.5 end},{f_gold,function() return 0.3 end}}, angle=-0.05, angle_jitter=0.2, coverage=2.2, fill=true, length={10,22}, tool={kind="round", width=3.5}, pressure={0.5,0.85}, scale_at=function(x,y) return 0.6+2.0*depth(y) end, threshold=0.3, mix_jitter=0.4, seed=451})
-- left part: the shadow continues right up to the base
local leftz = poly({{215,600},{260,618},{330,632},{420,638},{440,646},{380,656},{300,650},{220,630}}, true) - stack2_m:grow(1)
work(leftz, {hand="hatch", piles={{sh_core,function() return 1 end},{sh_dk2,function() return 0.8 end},{sh_mid2,function() return 0.3 end}}, angle=-0.15, angle_jitter=0.2, coverage=2.2, fill=true, length={10,22}, tool={kind="round", width=3.5}, pressure={0.5,0.85}, scale_at=function(x,y) return 0.6+2.0*depth(y) end, threshold=0.3, mix_jitter=0.4, seed=452})

--@ chunk 118
local b = brush{kind="round", width=2.5, point=0.6}
b:load(sh_core, 0.5)
b:gesture({{430,636,0.5},{470,632,0.45},{500,626,0.35},{520,618,0.2},{530,612,0.05}}, {wobble=1})

--@ chunk 119
local b = brush{kind="round", width=1.8, point=0.9}
b:load(eave_warm, 0.5)
-- a simple monogram "C" and year mark, lower right
b:gesture({{946,738,0.3},{938,733,0.5},{931,738,0.5},{930,748,0.5},{937,754,0.45},{946,751,0.2}})
b:load(eave_warm, 0.5)
b:gesture({{952,754,0.4},{955,744,0.5},{958,734,0.4}})
b:gesture({{958,734,0.4},{963,744,0.5},{967,754,0.4}})
b:gesture({{954,746,0.3},{965,746,0.3}})

--@ chunk 120
print(wait(30*24*60))

--@ chunk 121
print(shadow4_m ~= nil, st_hot ~= nil, field_m ~= nil, stack2_m ~= nil, field_far ~= nil, cast_warm ~= nil)
print(drying(300,650), drying(700,430), drying(800,700))
print(palette())

--@ chunk 122
sh_plum = pile{{"cobalt violet",1},{"permanent alizarin",0.3},{"burnt sienna",0.3},{"ultramarine blue",0.15},{"lead white",0.35}, name="sh_plum"}
local n = noise{seed=77, period=70, octaves=3}
local sb = shadow4_m:blur(22)
local depth = function(y) return clamp((y-430)/340,0,1) end
-- bays of lit stubble eating into the shadow along both long edges
local bays = mask(function(x,y)
  local s = shadow4_m:at(x,y)
  if s < 0.5 then return 0 end
  local v = sb:at(x,y)
  local thr = 0.55 + 0.32*n(x,y)
  return clamp((thr - v)*6, 0, 1)
end) - stack2_m:grow(18)
local lowside = function(x,y) return (y > 640 - 0.25*x + 40) and 1 or 0 end
work(bays, {hand="hatch", piles={
  {st_hot,function(x,y) return 0.6 + 0.4*lowside(x,y) end},
  {f_deep,function(x,y) return 0.3 + 0.8*depth(y)*lowside(x,y) end},
  {st_lit,function(x,y) return 0.8*(1-lowside(x,y)) end},
  {f_gold,function(x,y) return 0.4*(1-lowside(x,y)) end}},
  angle=function(x,y) return 0.1*math.sin(x/140) - 0.14 end, angle_jitter=0.2, coverage=1.5, threshold=0.35,
  scale_at=function(x,y) return 0.6 + 2.0*depth(y) end, length={8,22}, pressure={0.45,0.8}, mix_jitter=0.4, seed=501})
print(bays:area())

--@ chunk 123
local n = noise{seed=78, period=90, octaves=3}
local sb = shadow4_m:blur(36)
local depth = function(y) return clamp((y-430)/340,0,1) end
local lowside = function(x,y) return (y > 640 - 0.25*x + 40) and 1 or 0 end
local bays = mask(function(x,y)
  if shadow4_m:at(x,y) < 0.5 then return 0 end
  local thr = 0.62 + 0.42*n(x,y)
  return clamp((thr - sb:at(x,y))*5, 0, 1)
end) - stack2_m:grow(18)
local tongues = mask(function(x,y)
  if shadow4_m:at(x,y) >= 0.5 then return 0 end
  local thr = 0.38 + 0.42*n(x+500,y)
  return clamp((sb:at(x,y) - thr)*5, 0, 1)
end) - stack2_m:grow(4)
print(bays:area(), tongues:area())
work(bays, {hand="hatch", piles={
  {st_hot,function(x,y) return 0.6 + 0.4*lowside(x,y) end},
  {f_deep,function(x,y) return 0.3 + 0.8*depth(y)*lowside(x,y) end},
  {st_lit,function(x,y) return 0.8*(1-lowside(x,y)) end},
  {f_gold,function(x,y) return 0.4*(1-lowside(x,y)) end}},
  angle=function(x,y) return 0.1*math.sin(x/140) - 0.14 end, angle_jitter=0.2, coverage=1.6, threshold=0.35,
  scale_at=function(x,y) return 0.6 + 2.0*depth(y) end, length={8,22}, pressure={0.45,0.8}, mix_jitter=0.4, seed=502})
work(tongues, {hand="hatch", piles={
  {sh_dk2,function() return 0.8 end},{sh_mid2,function() return 0.6 end},{sh_plum,function() return 0.5 end}},
  angle=function(x,y) return 0.1*math.sin(x/140) - 0.14 end, angle_jitter=0.2, coverage=1.6, threshold=0.35,
  scale_at=function(x,y) return 0.6 + 2.0*depth(y) end, length={8,22}, pressure={0.45,0.8}, mix_jitter=0.4, seed=503})

--@ chunk 124
local depth = function(y) return clamp((y-430)/340,0,1) end
local n = noise{seed=91, period=110, octaves=2}
work(shadow4_m:shrink(6) - stack2_m:grow(3), {hand="hatch", piles={{sh_plum,function() return 1 end},{sh_core,function(x,y) return 0.3 end}},
  angle=function(x,y) return 0.1*math.sin(x/140) - 0.16 end, angle_jitter=0.2, coverage=0.4, threshold=0.4,
  load_at=function(x,y) return 0.5 + 0.2*n(x,y) end,
  scale_at=function(x,y) return 0.6 + 2.0*depth(y) end, length={8,22}, pressure={0.4,0.75}, mix_jitter=0.4, seed=511})

--@ chunk 125
local yc = function(x) return 429 + 3*x/1000 end
local excl = stack2_m:grow(2) + ds_m:grow(2)
local hedge = mask(function(x,y) local d = y - yc(x); return (d > -7 and d < 1) and 1 or 0 end):soften(1.5) - excl
local strip = mask(function(x,y) local d = y - yc(x); return (d >= 0 and d < 10) and 1 or 0 end):soften(1.5) - excl
local sunlane = function(x) return math.exp(-((x-sun_x)/130)^2) end
local leftside = function(x) return clamp((560-x)/300,0,1) end
work(hedge, {hand="hatch", piles={
  {tr_haze,function(x,y) return 0.6 + 0.6*(1-leftside(x)) end},
  {tr_light,function(x,y) return 0.5 end},
  {sh_mid2,function(x,y) return 0.6*leftside(x) + 0.15 end},
  {sk_gold,function(x,y) return 1.0*sunlane(x) end}},
  angle=1.45, angle_jitter=0.4, coverage=1.6, length={3,7}, tool={kind="round", width=2}, pressure={0.4,0.7}, threshold=0.3, mix_jitter=0.5, seed=521})
work(strip, {hand="hatch", piles={
  {field_far,function(x,y) return 0.8 end},
  {st_lit,function(x,y) return 0.5 + 1.0*sunlane(x) end},
  {f_gold,function(x,y) return 0.4 end},
  {sh_mid2,function(x,y) return 0.12 end}},
  angle=0.02, angle_jitter=0.08, coverage=1.6, length={5,12}, tool={kind="round", width=2}, pressure={0.4,0.7}, threshold=0.3, mix_jitter=0.5, seed=522})

--@ chunk 126
local yc = function(x) return 429 + 3*x/1000 end
local excl = stack2_m:grow(3) + ds_m:grow(3)
local hedge = mask(function(x,y) local d = y - yc(x); return clamp(1 - math.abs(d+3)/7, 0, 1) end) - excl
blend(hedge, {angle=0.0, threshold=0.15})

--@ chunk 127
print(wait(10*24*60)); for _,p in ipairs({{100,430},{700,432},{150,700},{300,660},{100,620},{450,600}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 128
local yc = function(x) return 429 + 3*x/1000 end
local n = noise{seed=31, period=25, octaves=2}
local foot = mask(function(x,y) local d = y - yc(x); local top = -9 + 2*n(x,0); return (d > top and d < 0.5) and 1 or 0 end):soften(1) * mask(function(x,y) return 1 - smoothstep(180,235,x) end) - stack2_m:grow(2)
work(foot, {hand="hatch", piles={{tr_dark,function() return 1 end},{sh_mid2,function() return 0.7 end},{tr_light,function() return 0.3 end}},
  angle=0.0, angle_jitter=0.25, coverage=1.4, length={4,10}, tool={kind="round", width=2.2}, pressure={0.4,0.7}, threshold=0.3, mix_jitter=0.5, seed=531})

--@ chunk 129
local yc = function(x) return 429 + 3*x/1000 end
local patch = mask(function(x,y) local d = y - yc(x); return (d > -9 and d < 0.5 and x > 170 and x < 245) and 1 or 0 end):soften(1) - stack2_m:grow(1)
work(patch, {hand="hatch", piles={{tr_dark,function() return 1 end},{sh_mid2,function() return 0.7 end}},
  angle=0.0, angle_jitter=0.25, coverage=2.0, length={4,9}, tool={kind="round", width=2.2}, pressure={0.4,0.7}, threshold=0.3, clip=patch:grow(1), mix_jitter=0.5, seed=532})
local band = mask(function(x,y) local d = y - yc(x); return clamp(1 - math.abs(d+4.5)/6, 0, 1) end) * mask(function(x,y) return 1 - smoothstep(230,250,x) end) - stack2_m:grow(3)
blend(band, {angle=0.0, threshold=0.3})

--@ chunk 130
local depth = function(y) return clamp((y-430)/340,0,1) end
local area = field_m * mask(function(x,y) return smoothstep(630,680,y) end) - shadow4_m:grow(4) - stack2_m:grow(3) - rect(915,720,70,45)
work(area, {hand="hatch", piles={
  {st_hot,function() return 0.8 end},
  {f_deep,function() return 0.7 end},
  {eave_warm,function() return 0.25 end},
  {st_lit,function(x,y) return 0.35 end},
  {sh_plum,function() return 0.15 end}},
  angle=function(x,y) return 0.08*math.sin(x/120) - 0.06 end, angle_jitter=0.25, coverage=0.55, threshold=0.4,
  scale_at=function(x,y) return 0.8 + 2.0*depth(y) end, length={10,24}, pressure={0.45,0.8}, mix_jitter=0.5, seed=541})

--@ chunk 131
local depth = function(y) return clamp((y-430)/340,0,1) end
local area = field_m * mask(function(x,y) return smoothstep(690,740,y) end) - shadow4_m:grow(4) - stack2_m:grow(3) - rect(915,720,70,45)
work(area, {hand="hatch", piles={
  {f_deep,function() return 0.8 end},
  {eave_warm,function() return 0.5 end},
  {sh_plum,function() return 0.3 end}},
  angle=function(x,y) return 0.08*math.sin(x/120) - 0.06 end, angle_jitter=0.25, coverage=0.4, threshold=0.4,
  scale_at=function(x,y) return 0.8 + 2.0*depth(y) end, length={10,24}, pressure={0.45,0.8}, mix_jitter=0.5, seed=551})

--@ chunk 132
print(wait(30*24*60)); print(drying(600,700), drying(100,430), drying(300,680))

--@ chunk 133
for _,l in ipairs({palette()}) do print(l) end print(drying(730,460))

--@ chunk 134
print(roof_glow, stack_dk, rim_hot, ds_dk)

--@ chunk 135

d_dk = pile{{"cobalt violet",1},{"ultramarine blue",0.4},{"rose madder",0.3},{"lead white",1.2},{"burnt sienna",0.2}, name="d_dk"}
d_plum = pile{{"permanent alizarin",0.6},{"cobalt violet",1},{"burnt sienna",0.6},{"ultramarine blue",0.3},{"lead white",0.5}, name="d_plum"}
d_glow = pile{{"vermilion",0.6},{"orange chrome",0.6},{"cobalt violet",0.6},{"lead white",1}, name="d_glow"}
d_rim = pile{{"orange chrome",1.5},{"cadmium yellow",0.6},{"vermilion",0.4},{"lead white",0.3}, name="d_rim"}
d_eave = pile{{"ultramarine blue",1},{"permanent alizarin",0.6},{"burnt sienna",0.4},{"cobalt violet",0.5}, name="d_eave"}
dcap_m = poly({{700,468},{705,459},{712,447},{720,438},{727,433},{734,432},{741,435},{749,443},{756,453},{762,462},{766,468},{760,471},{732,473},{705,471}}, true)
ddrum_m = poly({{705,469},{758,468},{759,480},{757,491},{732,494},{707,492},{705,480}}, true)
dnew_m = dcap_m + ddrum_m
-- 1. field over the old dome's flanks outside the new silhouette (drum inset) and the pale patch above the peak
local flank = (ds_m:grow(2) - dnew_m:grow(0.5)) * mask(function(x,y) return (y>470) and 1 or 0 end)
work(flank, {hand="detail", piles={{st_lit,function() return 1 end},{f_gold,function() return 0.7 end},{field_far,function() return 0.4 end}}, angle=0.03, coverage=3, fill=true, length={4,9}, tool={kind="filbert", width=2.5}, clip=flank:grow(1), threshold=0.2, seed=601})
local top = (rect(712,426,40,12) - dcap_m:grow(0.5)) * mask(function(x,y) return (y>=431) and 1 or 0 end)
work(top, {hand="detail", piles={{field_far,function() return 1 end},{st_lit,function() return 0.6 end}}, angle=0.0, coverage=3, fill=true, length={4,9}, tool={kind="filbert", width=2}, clip=top:grow(1), threshold=0.2, seed=602})
print(flank:area(), top:area())

--@ chunk 136

local xr = function(x) return clamp((x-701)/64,0,1) end
-- cap: strokes along the slopes
work(dcap_m, {hand="detail", piles={
  {d_dk,function(x,y) return 1.3-1.3*xr(x)+0.05 end},
  {d_plum,function(x,y) return 0.6*math.exp(-((xr(x)-0.5)/0.25)^2) end},
  {d_glow,function(x,y) return math.exp(-((xr(x)-0.75)/0.14)^2) + 0.5*clamp((442-y)/10,0,1)*clamp((x-722)/15,0,1) end},
  {d_rim,function(x,y) return 1.6*clamp((xr(x)-0.84)/0.1,0,1) end}},
  angle=function(x,y) return (x<733) and -1.05 or 1.05 end, angle_jitter=0.15, coverage=3.5, fill=true, length={4,9}, tool={kind="filbert", width=2.2, lay=1.5}, clip=dcap_m, threshold=0.2, mix_jitter=0.4, seed=611})
-- drum: vertical strokes, a touch darker (in the cap's shadow), warm on its right
work(ddrum_m, {hand="detail", piles={
  {d_dk,function(x,y) return 1.3-1.2*xr(x)+0.1 end},
  {d_plum,function(x,y) return 0.7*math.exp(-((xr(x)-0.55)/0.25)^2) end},
  {d_glow,function(x,y) return 0.7*math.exp(-((xr(x)-0.8)/0.12)^2) end},
  {d_rim,function(x,y) return 1.2*clamp((xr(x)-0.87)/0.08,0,1) end}},
  angle=1.57, angle_jitter=0.12, coverage=3.5, fill=true, length={4,9}, tool={kind="filbert", width=2.2, lay=1.5}, clip=ddrum_m, threshold=0.2, mix_jitter=0.4, seed=612})

--@ chunk 137
print(wait(4*24*60)); for _,p in ipairs({{730,450},{720,480},{730,434},{702,480},{762,480}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 138
print(wait(4*24*60)); for _,p in ipairs({{730,450},{720,480},{730,434},{702,480},{762,480}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 139
print(wait(4*24*60)); for _,p in ipairs({{730,450},{720,480},{730,434},{702,480}}) do print(p[1],p[2],drying(p[1],p[2])) end

--@ chunk 140

local b = brush{kind="round", width=2.2, point=0.6}
b:load(d_eave, 0.5)
b:gesture({{703,470,0.3},{715,472.5,0.6},{730,473.5,0.65},{745,472.5,0.55}}, {wobble=0.5})
b:reload(eave_warm, 0.5)
b:gesture({{743,472.5,0.5},{754,471,0.5},{763,469,0.2}}, {wobble=0.5})
-- field dashes over the flat repaint around the stack
local zone = (rect(696,429,74,10) + rect(694,466,78,30)) - dnew_m:grow(1)
work(zone, {hand="hatch", piles={{field_far,function() return 0.8 end},{st_lit,function() return 0.8 end},{f_gold,function() return 0.5 end},{st_shade,function() return 0.2 end}}, angle=0.02, angle_jitter=0.1, coverage=1.0, length={4,8}, tool={kind="round", width=1.8}, pressure={0.3,0.6}, load=0.5, clip=zone, threshold=0.3, mix_jitter=0.5, seed=621})

--@ chunk 141

local band = ddrum_m:shrink(1) * mask(function(x,y) return clamp(1-math.abs(y-475)/8,0,1) end)
blend(band:soften(1), {angle=1.57, threshold=0.15})

--@ chunk 142

d_drum = mix{{d_dk,0.6},{d_eave,0.25},{d_plum,0.15}, name="d_drum"}
local xr = function(x) return clamp((x-704)/56,0,1) end
work(ddrum_m:shrink(0.5) * mask(function(x,y) return smoothstep(471,477,y) end), {hand="detail", piles={
  {d_drum,function(x,y) return 1.4-1.2*xr(x)+0.1 end},
  {d_plum,function(x,y) return 0.6*math.exp(-((xr(x)-0.6)/0.2)^2) end},
  {d_glow,function(x,y) return 0.8*math.exp(-((xr(x)-0.85)/0.1)^2) end}},
  angle=1.57, angle_jitter=0.12, coverage=2.0, length={4,9}, tool={kind="filbert", width=2, lay=1.2}, clip=ddrum_m:shrink(0.5), threshold=0.2, mix_jitter=0.4, seed=631})
local b = brush{kind="filbert", width=3}
b:load(sh_mid2, 0.5)
b:stroke({{712,492},{695,494},{676,497}}, {pressure={0.7,0.3}})
b:load(sh_dk2, 0.3)
b:stroke({{716,491},{700,492.5}}, {pressure={0.6,0.3}})

--@ chunk 143

local b = brush{kind="round", width=1.8, point=0.6}
b:load(d_glow, 0.45)
b:gesture({{756,474,0.3},{757.5,482,0.5},{756,490,0.2}}, {wobble=0.3})
b:load(d_rim, 0.35)
b:gesture({{758,476,0.2},{758.8,483,0.35},{757.5,488,0.1}}, {wobble=0.3})

--@ chunk 144

print(drying(800,560), drying(650,600))
local depth = function(y) return clamp((y-430)/340,0,1) end
local sunlane = function(x,y) return math.exp(-((x-sun_x)/120)^2) end
local area = field_m * mask(function(x,y) return smoothstep(500,530,y)*(1-smoothstep(680,720,y)) end) - stack2_m:grow(6) - shadow4_m:grow(6) - dnew_m:grow(6)
work(area, {hand="hatch", piles={{st_shade,function() return 1 end},{sh_mid2,function(x,y) return 0.4 end}}, coverage=0.12,
  load_at=function(x,y) return 0.5 - 0.25*sunlane(x,y) end,
  angle=function(x,y) return 0.06*math.sin(x/120) end, angle_jitter=0.15,
  scale_at=function(x,y) return 0.6 + 2.0*depth(y) end, length={8,22}, pressure={0.4,0.7}, threshold=0.4, mix_jitter=0.4, seed=641})

--@ chunk 145

print(drying(800,600))
local area = field_m * mask(function(x,y) return smoothstep(495,525,y)*(1-smoothstep(690,730,y)) end) - stack2_m:grow(4) - shadow4_m:grow(2) - dnew_m:grow(3)
local r = rag{width=30}
r:dip(0.5)
r:wipe(area, {pressure=0.5, angle=0, passes=1, refold=0.25})

--@ chunk 146
print(wait(20*24*60)); print(drying(800,600), drying(730,450), drying(100,560))

--@ chunk 147
print(palette()) print(drying(380,505), drying(730,470))

--@ chunk 148
for _,p in ipairs({{240,495},{373,490},{508,495},{373,520},{230,495},{520,495},{373,300}}) do print(p[1],p[2], stack2_m:at(p[1],p[2])) end

--@ chunk 149
local s="" for x=200,560,10 do s=s..string.format("%d:%.1f ",x,stack2_m:at(x,500)) end print(s)
s="" for x=200,560,10 do s=s..string.format("%d:%.1f ",x,stack2_m:at(x,485)) end print(s)

--@ chunk 150

eave_cx, eave_hw, eave_y0, eave_sag = 373, 140, 500, 6
function yEave(x) local u=(x-eave_cx)/eave_hw return eave_y0 + eave_sag*(1-u*u) end
local function slope(x) return -2*eave_sag*(x-eave_cx)/(eave_hw*eave_hw) end
local skirt = mask(function(x,y) local a=yEave(x) return (x>233 and x<514 and y>a-20 and y<a+0.5) and 1 or 0 end):roughen(1.5, 12, 11):soften(2)
work(skirt, {hand="body", tool="filbert 4", length={10,24},
  piles={{d_dk,function(x,y) return math.max(0,1-(x-233)/200) end},
         {d_plum,function(x,y) return 0.6 end},
         {d_glow,function(x,y) return math.max(0,(x-400)/80) end},
         {d_rim,function(x,y) return math.max(0,(x-470)/40) end}},
  coverage=1.6, angle=function(x,y) return math.atan(slope(x),1) + 0.2*((x-eave_cx)/eave_hw) end,
  clip=stack2_m, seed=2471})
local band = mask(function(x,y) local a=yEave(x) local th = lerp(7,4,clamp((x-235)/277,0,1)) return (x>236 and x<512 and y>a and y<a+th) and 1 or 0 end):roughen(2, 10, 12)
work(band, {hand="body", tool="filbert 3", length={8,20},
  piles={{d_eave,function(x,y) return clamp(1-(x-430)/100,0.15,1) end},{eave_warm,function(x,y) return clamp((x-430)/70,0,1) end}},
  coverage=1.4, load=0.5, angle=function(x,y) return math.atan(slope(x),1) end, clip=stack2_m, seed=2472})

--@ chunk 151

r = rag{width=14}
r:dip(0.8)
local sk = mask(function(x,y) local a=yEave(x) return (x>226 and x<520 and y>a-24 and y<a-1.5) and 1 or 0 end)
r:wipe(sk, {pressure=0.8, angle=0, passes=3, refold=0.4})

--@ chunk 152

r = rag{width=14}
r:dip(1.0)
local sk = mask(function(x,y) local a=yEave(x) return (x>226 and x<520 and y>a-24 and y<a-1.5) and 1 or 0 end)
r:wipe(sk, {pressure=0.9, angle=0, passes=3, refold=0.3})
r:dip(1.0)
r:wipe(sk, {pressure=0.9, angle=0.1, passes=2, refold=0.3})
print(drying(373,495))

--@ chunk 153

sk_dk = mix{{d_dk,0.45},{d_eave,0.35},{d_plum,0.2}, name="sk_dk"}
sk_mid = mix{{d_plum,0.55},{d_eave,0.25},{d_glow,0.2}, name="sk_mid"}
local function slope(x) return -2*eave_sag*(x-eave_cx)/(eave_hw*eave_hw) end
local t = mask(function(x,y) local a=yEave(x) return (x>280 and x<330 and y>a-20 and y<a+0.5) and 1 or 0 end):roughen(1.5,12,11):soften(2)
work(t, {hand="body", tool="filbert 4", length={10,22}, piles={{sk_dk,function(x,y) return clamp((y-(yEave(x)-20))/20,0,1) end},{d_plum,function(x,y) return clamp(1-(y-(yEave(x)-20))/20,0,1)*0.7 end}},
  coverage=1.4, angle=function(x,y) return math.atan(slope(x),1)+0.15 end, clip=stack2_m, seed=2475})

--@ chunk 154

r = rag{width=12}
r:dip(1.0)
local t = mask(function(x,y) local a=yEave(x) return (x>274 and x<336 and y>a-24 and y<a-1.5) and 1 or 0 end)
r:wipe(t, {pressure=0.9, angle=0, passes=4, refold=0.3})
sk_blue = mix{{d_dk,0.4},{d_eave,0.35},{sh_mid2,0.25}, name="sk_blue"}
local function slope(x) return -2*eave_sag*(x-eave_cx)/(eave_hw*eave_hw) end
local fall = function(x,y) local u=(x-eave_cx)/eave_hw return math.pi/2 - 0.55*u end
local t2 = mask(function(x,y) local a=yEave(x) return (x>280 and x<330 and y>a-18 and y<a+0.5) and 1 or 0 end):roughen(2,12,13):soften(2)
work(t2, {hand="body", tool="filbert 3", length={8,16}, piles={{sk_blue,function(x,y) return clamp((y-(yEave(x)-18))/18,0.2,1) end},{d_dk,function(x,y) return clamp(1-(y-(yEave(x)-18))/18,0,0.8) end}},
  coverage=1.3, angle=fall, clip=stack2_m, seed=2476})

--@ chunk 155

r = rag{width=12}
r:dip(1.0)
local t = mask(function(x,y) local a=yEave(x) return (x>274 and x<336 and y>a-26 and y<a-1.5) and 1 or 0 end)
r:wipe(t, {pressure=0.9, angle=0, passes=4, refold=0.3})

--@ chunk 156

stack_dk = pile{{"ultramarine blue",1},{"permanent alizarin",0.5},{"cobalt violet",1},{"burnt sienna",0.5},{"lead white",0.5}, name="stack_dk"}
stack_plum = pile{{"permanent alizarin",0.6},{"cobalt violet",1},{"burnt sienna",0.6},{"ultramarine blue",0.3},{"lead white",0.5}, name="stack_plum"}
eave_sh = pile{{"ultramarine blue",1},{"permanent alizarin",0.6},{"burnt sienna",0.4},{"cobalt violet",0.5}, name="eave_sh"}
local fall = function(x,y) local u=(x-eave_cx)/eave_hw return math.pi/2 - 0.55*u end
local t2 = mask(function(x,y) local a=yEave(x) return (x>280 and x<330 and y>a-18 and y<a+0.5) and 1 or 0 end):roughen(2,12,13):soften(2)
work(t2, {hand="body", tool="filbert 3", length={8,16}, piles={{stack_dk,function(x,y) return 1 end},{eave_sh,function(x,y) return 0.5*clamp((y-(yEave(x)-18))/18,0,1) end}},
  coverage=1.3, angle=fall, clip=stack2_m, seed=2477})

--@ chunk 157

local fall = function(x,y) local u=(x-eave_cx)/eave_hw return math.pi/2 - 0.55*u end
local xr = function(x) return clamp((x-230)/310,0,1) end
local sk = mask(function(x,y) local a=yEave(x) return (x>228 and x<517 and y>a-27 and y<a+0.5) and 1 or 0 end):roughen(2.5,14,21):soften(3)
local near = function(x,y) return clamp((y-(yEave(x)-27))/27,0,1) end
work(sk, {hand="body", tool="filbert 3", length={8,18}, piles={
  {stack_dk,function(x,y) return clamp(1-xr(x)/0.5,0,1)+0.08 end},
  {stack_plum,function(x,y) return math.exp(-((xr(x)-0.55)/0.2)^2) end},
  {d_glow,function(x,y) return math.exp(-((xr(x)-0.8)/0.1)^2)*0.9 end},
  {d_rim,function(x,y) return clamp((xr(x)-0.84)/0.1,0,1)*1.3 end},
  {eave_sh,function(x,y) return 0.45*near(x,y)*clamp(1-xr(x)/0.75,0,1) end}},
  coverage=1.5, angle=fall, angle_jitter=0.15, clip=stack2_m, mix_jitter=0.3, seed=2478})

--@ chunk 158
print(wait(10*24*60)); print(drying(300,495), drying(470,495), drying(373,508))

--@ chunk 159
print(wait(6*24*60)); print(drying(300,495), drying(250,495), drying(373,508))

--@ chunk 160

local fall = function(x,y) local u=(x-eave_cx)/eave_hw return math.pi/2 - 0.6*u end
local xr = function(x) return clamp((x-230)/310,0,1) end
local seam = mask(function(x,y) local a=yEave(x) return (x>236 and x<440 and y>a-34 and y<a-20) and 1 or 0 end):roughen(3,14,31):soften(3)
work(seam, {hand="body", tool="filbert 3", length={10,20}, piles={
  {stack_dk,function(x,y) return clamp(1-xr(x)/0.5,0,1)+0.1 end},
  {stack_plum,function(x,y) return math.exp(-((xr(x)-0.55)/0.2)^2)+0.2 end}},
  coverage=0.9, angle=fall, angle_jitter=0.15, clip=stack2_m, mix_jitter=0.3, seed=2481})

--@ chunk 161
print(drying(300,480), drying(730,490), drying(706,488), drying(757,488)); for _,p in ipairs({{706,485},{707,491},{732,494},{757,490},{758,485}}) do print(p[1],p[2],dnew_m:at(p[1],p[2]), ddrum_m:at(p[1],p[2])) end

--@ chunk 162
-- trial: square the lit lower-right corner of the distant drum, then stubble overlapping its foot
local corner = poly({{752,482},{760,481},{761,488},{760,494},{748,495},{748,488}}, true)
local xr = function(x) return clamp((x-704)/56,0,1) end
work(corner, {hand="detail", piles={
  {d_plum,function(x,y) return 0.5 end},
  {d_glow,function(x,y) return 0.9*math.exp(-((xr(x)-0.85)/0.12)^2) end},
  {d_rim,function(x,y) return 1.2*clamp((xr(x)-0.9)/0.08,0,1) end}},
  angle=1.57, angle_jitter=0.1, coverage=2.5, fill=true, length={4,8}, tool={kind="filbert", width=2, lay=1.2}, clip=corner, threshold=0.2, mix_jitter=0.4, seed=2631})
local foot = poly({{735,492},{748,493},{763,492},{764,497},{735,498}}, true):roughen(1,6,7)
work(foot, {hand="hatch", piles={{st_lit,function() return 1 end},{f_gold,function() return 0.6 end},{field_far,function() return 0.4 end}},
  angle=0.03, angle_jitter=0.15, coverage=1.2, length={3,7}, tool={kind="round", width=1.6}, pressure={0.35,0.6}, load=0.5, threshold=0.3, mix_jitter=0.5, seed=2632})

--@ chunk 163
for k=1,2 do
local r = rag{width=10}
r:dip(0.9)
r:wipe(poly({{733,480},{763,480},{766,499},{733,499}}, true), {pressure=0.85, angle=0, passes=2, refold=0.3})
end

--@ chunk 164
print(wait(12*24*60)); print(drying(750,488), drying(300,480), drying(700,470))

--@ chunk 165
local depth = function(y) return clamp((y-430)/340,0,1) end
local sunlane = function(x,y) return math.exp(-((x-sun_x)/140)^2) end
local n = noise{seed=275, period=18, octaves=2}
local zone = mask(function(x,y)
  if y < 433 or y > 512 then return 0 end
  local cx, cy = 732, 468
  local d = math.sqrt(((x-cx)/58)^2 + ((y-cy)/38)^2)
  return clamp((1.0 + 0.18*n(x,y) - d)*6, 0, 1)
end)
local shad = poly({{716,486},{690,490},{660,494},{648,500},{690,499},{720,496}}, true):grow(2)
local foot = mask(function(x,y) if x < 730 then return 0 end return ddrum_m:at(x,y)*(1-ddrum_m:at(x,y+1.8)) end)
local area = (zone - dnew_m:grow(1.2) - shad) + foot
print(area:area())
work(area, {hand="hatch", piles={
  {field_far,function(x,y) return 0.8*(1-depth(y)*2) + 0.1 end},
  {f_pink,function(x,y) return 0.5 end},
  {st_lit,function(x,y) return 0.6 + 0.8*sunlane(x,y) end},
  {f_gold,function(x,y) return 0.4 end},
  {st_shade,function(x,y) return 0.25 end}},
  angle=function(x,y) return 0.04*math.sin(x/90) end, angle_jitter=0.1, coverage=1.1,
  scale_at=function(x,y) return 0.45 + 2.0*depth(y) end, length={8,20}, pressure={0.3,0.6}, load=0.5, threshold=0.3, mix_jitter=0.5, seed=2751})

--@ chunk 166
local foot = mask(function(x,y) if x < 730 then return 0 end return ddrum_m:at(x,y)*(1-ddrum_m:at(x,y+1.8)) end)
for k=1,3 do
local r = rag{width=8}
r:dip(0.9)
r:wipe(dnew_m - foot, {pressure=0.85, angle=1.57, passes=2, refold=0.3})
end

--@ chunk 167
print(wait(8*24*60)); print(drying(765,468), drying(760,485), drying(700,470), drying(732,432))

--@ chunk 168
local b = brush{kind="round", width=2, point=0.6}
b:load(d_rim, 0.5)
b:gesture({{750,445,0.15},{755,452,0.4},{760,460,0.5},{764,466,0.5},{767,469,0.15}}, {wobble=0.3})
b:load(d_glow, 0.4)
b:gesture({{745,446,0.1},{751,455,0.35},{757,463,0.4},{761,469,0.2}}, {wobble=0.4})
b:load(d_glow, 0.45)
b:gesture({{756,473,0.3},{757.5,482,0.5},{756.5,491,0.25}}, {wobble=0.3})
b:load(d_rim, 0.4)
b:gesture({{758.2,475,0.2},{759,483,0.4},{758,490,0.1}}, {wobble=0.3})

--@ chunk 169
print(wait(10*24*60)); print(drying(758,483), drying(760,460))

--@ chunk 170
print(sun_x, sun_y, drying(sun_x, sun_y), drying(sun_x+15, sun_y))
sun_flow2 = pile{{"lead white",4},{"lemon chrome",0.5}, medium=0.65, name="sun_flow2"}
local d = ellipse(sun_x, sun_y, 18, 17)
work(d, {hand="detail", pile=sun_flow2, coverage=3, fill=true, angle=0, tool={kind="filbert", width=6, lay=2}, clip=d, seed=4401})
print(wait(40*24*60))

--@ chunk 171
print(drying and drying(835,372) or "no drying fn")

--@ chunk 172
print(pops_m ~= nil, treeband_m ~= nil, pop_c ~= nil, pop_far ~= nil, tr_dark ~= nil, tr_light ~= nil, tr_haze ~= nil, sk_gold ~= nil)
print(drying(575,370), drying(660,390), drying(940,390))
local zone = pops_m * rect(530,300,90,105) - stack2_m:grow(3)
local xfar = function(x) return clamp((x-450)/450,0,1) end
work(zone, {hand="body", piles={{tr_dark,function(x,y) return 0.6*(1-xfar(x)) + 0.05 end},{pop_c,function() return 0.8 end},{pop_far,function(x,y) return 1.2*xfar(x) end}}, angle=1.57, angle_jitter=0.3, coverage=1.2, length={6,14}, tool={kind="filbert", width=4}, load=0.5, pressure={0.3,0.6}, clip=zone, seed=5101})

--@ chunk 173
local xfar = function(x) return clamp((x-450)/450,0,1) end
local glowd = function(x,y) return math.sqrt((x-sun_x)^2+((y-sun_y)*1.5)^2) end
local excl = stack2_m:grow(3) + ellipse(sun_x, sun_y, 22, 21)
local pz = pops_m * rect(620,300,380,110) - excl
work(pz, {hand="body", piles={{tr_dark,function(x,y) return 0.6*(1-xfar(x)) + 0.05 end},{pop_c,function() return 0.8 end},{pop_far,function(x,y) return 1.2*xfar(x) end}}, angle=1.57, angle_jitter=0.3, coverage=1.2, length={5,12}, tool={kind="filbert", width=3.5}, load=0.5, pressure={0.3,0.6}, clip=pz, seed=5102})
local hz = (treeband_m * mask(function(x,y) return (x>540 and y<416) and 1 or 0 end)) - pops_m - excl
work(hz, {hand="body", piles={
  {tr_dark,function(x,y) return 0.8*clamp((700-x)/180,0,1)+0.05 end},
  {tr_light,function(x,y) return 1.0 end},
  {tr_haze,function(x,y) return 0.5*clamp((x-650)/250,0,1) end},
  {sk_gold,function(x,y) return 1.2*clamp(1-(glowd(x,y)-20)/90,0,1) end}},
  angle=1.57, angle_jitter=0.6, coverage=1.2, length={5,12}, tool={kind="filbert", width=3.5}, load=0.5, pressure={0.3,0.6}, clip=hz, seed=5103})

--@ chunk 174
print(wait(12*24*60)); print(drying(575,370), drying(800,400), drying(945,400))

--@ chunk 175
local hz = treeband_m * mask(function(x,y) return (x>870 and y>=405 and y<424) and 1 or 0 end):soften(1.5)
work(hz, {hand="body", piles={{tr_light,function() return 1.0 end},{tr_haze,function() return 0.6 end},{pop_far,function() return 0.4 end}}, angle=1.57, angle_jitter=0.6, coverage=1.3, length={4,9}, tool={kind="filbert", width=3}, load=0.5, pressure={0.3,0.6}, clip=treeband_m, seed=5104})

--@ chunk 176
print(wait(20*24*60)); print(drying(940,415), drying(575,370))

--@ chunk 177
local glowd = function(x,y) return math.sqrt((x-sun_x)^2+((y-sun_y)*1.5)^2) end
local n = noise{seed=5105, period=14, octaves=2}
local hz = treeband_m * mask(function(x,y) if y < 398 or y > 425 then return 0 end; return clamp((smoothstep(835,885,x) + 0.35*n(x,y))*1.2,0,1) end) - pops_m:shrink(1)
work(hz, {hand="body", piles={{tr_haze,function() return 1.0 end},{sk_gold,function(x,y) return 0.25 + 0.9*clamp(1-(glowd(x,y)-20)/120,0,1) end},{field_far,function() return 0.3 end}}, angle=1.57, angle_jitter=0.6, coverage=0.9, length={4,9}, tool={kind="filbert", width=3}, load=0.45, pressure={0.3,0.55}, clip=treeband_m, threshold=0.3, seed=5106})

--@ chunk 178
local hz = treeband_m * mask(function(x,y) if y < 396 or y > 426 then return 0 end; return smoothstep(820,870,x) end) - pops_m:grow(1)
blend(hz, {angle=0.0, threshold=0.15})

--@ chunk 179
for k=1,3 do
local r = rag{width=14}
r:dip(0.9)
r:wipe(treeband_m * mask(function(x,y) return (x>815 and y>392 and y<428) and 1 or 0 end), {pressure=0.85, angle=0, passes=2, refold=0.3})
end

--@ chunk 180
print(wait(10*24*60)); print(drying(880,415), drying(950,415))

--@ chunk 181
print(gl_sky ~= nil)
local m = treeband_m * mask(function(x,y) if y < 390 or y > 428 then return 0 end; return clamp(1-(x-845)/110,0,1)*smoothstep(830,850,x) end) - pops_m:grow(1)
work(m, {hand="glaze", pile=gl_sky, coverage=1.0, threshold=0.15, load_at=function(x,y) return 0.35*clamp(1-(x-845)/110,0,1) end, angle=0.0, tool={kind="filbert", width=12, stiffness=0.3}, length={40,100}, seed=5107})
blend(m, {angle=0.0, threshold=0.15})

--@ chunk 182
for k=1,2 do
local r = rag{width=14}
r:dip(0.8)
r:wipe(treeband_m * mask(function(x,y) return (x>825 and x<965 and y>388 and y<430) and 1 or 0 end) - pops_m:grow(1), {pressure=0.8, angle=0, passes=2, refold=0.3})
end

--@ chunk 183
print(wait(20*24*60)); print(drying(830,410), drying(900,410))

--@ chunk 184
print(tr_light ~= nil, tr_haze ~= nil, pop_far ~= nil, sk_gold ~= nil, field_far ~= nil, treeband_m ~= nil)
print(drying(880,415), drying(950,415), drying(830,410))
print(tr_light, tr_haze, pop_far, sk_gold)

--@ chunk 185
hz_warm = mix{{tr_light,0.45},{tr_haze,0.35},{sk_gold,0.2}, name="hz_warm"}
local glowd = function(x,y) return math.sqrt((x-sun_x)^2+((y-sun_y)*1.5)^2) end
local n1 = noise{seed=6101, period=12, octaves=2}
local n2 = noise{seed=6102, period=12, octaves=2}
local band = function(y) return (y >= 399 and y <= 425) and 1 or 0 end
-- warm haze fingers reaching into the cool block
local warmz = treeband_m * mask(function(x,y) if band(y)==0 or x < 862 or x > 912 then return 0 end; return clamp((882 + 20*n1(x,y) - x)/5,0,1) end) - pops_m:grow(1)
-- cool lilac fingers reaching back into the warm haze
local coolz = treeband_m * mask(function(x,y) if band(y)==0 or x < 836 or x > 872 then return 0 end; return clamp((x - (856 + 14*n2(x,y)))/5,0,1) end) - pops_m:grow(1)
print(warmz:area(), coolz:area())
work(warmz, {hand="body", piles={{hz_warm,function() return 1 end},{sk_gold,function(x,y) return 0.25 end}}, angle=1.57, angle_jitter=0.6, coverage=0.8, length={4,9}, tool={kind="filbert", width=3}, load=0.45, pressure={0.3,0.55}, clip=treeband_m, threshold=0.3, seed=6103})
work(coolz, {hand="body", piles={{tr_light,function() return 1.0 end},{tr_haze,function() return 0.6 end},{pop_far,function() return 0.4 end}}, angle=1.57, angle_jitter=0.6, coverage=0.7, length={4,9}, tool={kind="filbert", width=3}, load=0.45, pressure={0.3,0.55}, clip=treeband_m, threshold=0.3, seed=6104})

--@ chunk 186
local r = rag{width=12}
r:dip(0.4)
r:wipe(treeband_m * mask(function(x,y) return (x>836 and x<912 and y>398 and y<426) and 1 or 0 end) - pops_m:grow(1), {pressure=0.45, angle=1.57, passes=1, refold=0.3})

--@ chunk 187
print(wait(20*24*60)); print(drying(860,412), drying(885,412))

--@ chunk (finishing)
-- after the session: let the paint dry right through, a month at a time until touch-dry
-- (at most ten years), as scripts/finish_painting does; no varnish, no cracks
local function all_dry()
  for i = 0, 39 do for j = 0, 29 do
    if drying(12.5 + i * 25, (j + 0.5) * H / 30) ~= "dry" then return false end
  end end
  return true
end
local months = 0
while not all_dry() and months < 120 do wait(30 * 24 * 60); months = months + 1 end
print("dried for " .. months .. " months")
