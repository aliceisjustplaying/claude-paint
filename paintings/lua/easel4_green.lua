-- easel session "easel4_green": a painting replayed chunk by chunk.
--   easel run paintings/lua/easel4_green.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.4, seed=23}; print(W,H); print(pal)

--@ chunk 2 · clock 0
HZ = 296
w = world{horizon=HZ, eye=28, fov=52, sun={azimuth=-118, elevation=36},
  ground=function(X, Z) return 1.5*math.sin(X/70+0.4)*math.min(1, Z/300) + 6*math.sin(Z/900 + X/1300) end}
BROW = {{-10,474},{90,466},{190,470},{300,482},{400,494},{520,512},{640,534},{760,552},{880,566},{1010,574}}
RIVER = {{1010,420},{900,398},{780,372},{700,352},{640,338},{600,326},{560,316},{530,308},{505,302}}
h = pencil("2H")
h:rule({0, HZ}, {1000, HZ}, {pressure=0.25})
h:sketch(BROW, {pressure=0.3})
h:sketch(RIVER, {pressure=0.25})
-- oak: trunk and crown envelope
h:sketch({{205,482},{210,420},{206,360},{214,300}}, {pressure=0.3})
h:sketch({{228,482},{226,420},{230,360},{226,300}}, {pressure=0.3})
h:sketch({{120,330},{90,260},{110,180},{170,120},{240,98},{320,130},{352,210},{330,300},{280,340}}, {pressure=0.25})
-- boulder
h:sketch({{378,508},{384,478},{410,458},{446,452},{470,466},{482,496},{476,516}}, {pressure=0.3})
-- seated figure, back to us
h:sketch({{494,512},{492,494},{496,478},{500,470},{506,478},{510,496},{512,512}}, {pressure=0.3})
-- village and spire
h:sketch({{640,322},{660,318},{700,317},{740,319},{770,322}}, {pressure=0.25})
h:rule({708, 318}, {708, 282}, {pressure=0.3})

--@ chunk 3 · clock 0
b2 = pencil("2B")
b2:line(BROW, {pressure={0.5, 0.6, 0.5}})
b2:line({{120,330},{90,260},{110,180},{170,120},{240,98},{320,130},{352,210},{330,300},{280,340}}, {pressure=0.45})
b2:line({{205,482},{210,420},{206,360},{214,300}}, {pressure=0.5})
b2:line({{228,482},{226,420},{230,360},{226,300}}, {pressure=0.5})
b2:line({{378,508},{384,478},{410,458},{446,452},{470,466},{482,496},{476,516}}, {pressure=0.55, smooth=false})
b2:line({{494,512},{492,494},{496,478},{500,470},{506,478},{510,496},{512,512}}, {pressure=0.5})
b2:line(RIVER, {pressure=0.35})
b2:rule({0, HZ}, {1000, HZ}, {pressure=0.35})
show(BROW, {label="brow"})

--@ chunk 4 · clock 0
sk = w:sky{haze=2.0, uneven={0.5, 30000, 5}}
cl = w:clouds{sky=sk, cell=3,
  {kind="cumulus", x=-2600, z=7000, base=1200, width=2600, height=1700, seed=14},
  {kind="cumulus", x=1800, z=11000, base=1300, width=2000, height=1100, seed=6},
  {kind="cumulus", x=5200, z=16000, base=1300, width=2600, height=900, seed=31},
  {kind="bank", x0=-30000, x1=30000, z=40000, depth=9000, base=800, top=1900, seed=4},
  {kind="stratus", base=4200, thick=400, cover=0.25, seed=2, wind={-0.3, 2}}}
SKYPAL = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}
local skym = above(function(x) return HZ + 14 end)
work(skym, {hand="broad", color=cl, angle=0, coverage=4.5, medium=0.3, pal=SKYPAL})
blend(skym, {angle=0})

--@ chunk 5 · clock 0
wait(24*60)
local top = above(function(x) return HZ - 30 end)
local litm = (cl:mask{alpha={0.45, 0.85}, lit={0.5, 0.9}} * top):blur(3)
work(litm, {hand="scumble", color="#f1eadb", coverage=2.2, medium=0.35, angle=function(x,y) return -0.3 + 0.4*math.sin(x/40) end, length={6, 18}, hug=false, pal=SKYPAL})
local shm = (cl:mask{alpha={0.5, 0.95}, shade={0.4, 0.85}} * top):blur(4)
work(shm, {hand="glaze", color_over={shift={-0.05, 0, -0.012}}, coverage=2, medium=0.8, angle=0, length={10, 30}, hug=false})
air = haze{visibility=22000, height=900, mist={60, 4, 900, 7}}
rs = w:ranges{near=9000, far=26000, count=3, seed=19, heights={160, 620}, kinds={"dome", "saddle", "plateau"}}
for i = #rs, 1, -1 do
  local l = rs[i]
  work(l:mask() * above(function(x) return HZ + 3 end), {hand="body", length={20, 60}, coverage=3.5, angle=0.04,
    color=function(x, y) return mix(mix("#44566a", "#5c6d86", (i-1)/2), sk:airlight(x), 0.8*l:haze(air, x, y)) end, medium=0.3})
end
for i, l in ipairs(rs) do print(i, l:crest(200), l:crest(600), l:crest(900)) end

--@ chunk 6 · clock 1440

local top = above(function(x) return HZ - 24 end)
local shm = (cl:mask{alpha={0.3, 0.95}} * top):grow(4):blur(4)
blend(shm, {angle=function(x,y) return 0.25*math.sin(x/70) end, coverage=3})

--@ chunk 7 · clock 1440
fields = worley{seed=5, period=1}
fnz = noise{seed=9, octaves=4, period=90}
FIELDC = {"#5d7a33", "#6b8538", "#4f6b2c", "#7d8f3f", "#a39a52", "#6f7a36", "#58743a", "#8c8a4a"}
function valley(x, y)
  local p = w:to_ground(x, y); if not p then return sk:airlight(x) end
  local X, Z = p[1], p[3]
  local ca, sa = math.cos(0.45), math.sin(0.45)
  local U, V = X*ca - Z*sa, X*sa + Z*ca
  local _, _, edge, r = fields:at(U/55, V/95)
  local c = FIELDC[1 + math.floor(r * #FIELDC) % #FIELDC]
  c = shift(c, 0.03*fnz(x, y) - (edge < 0.06 and 0.05 or 0), 0, 0)
  return mix(c, sk:airlight(x), clamp(w:aerial(Z) * 0.95, 0, 0.9))
end
local land = below(function(x) return HZ end)
work(land, {hand="body", length={20, 60}, coverage=3.5, angle=function(x, y) return 0.03*math.sin(x/90) end, color=valley, medium=0.25})

--@ chunk 8 · clock 1440
wait(24*60)
RIVW = {{160, 170}, {120, 260}, {40, 360}, {-10, 480}, {30, 640}, {110, 820}, {60, 1050}, {-80, 1300}, {-160, 1700}, {-120, 2300}, {-260, 3300}, {-420, 4600}}
river = w:ribbon(RIVW, 16)
work(river, {hand="body", tool="round 2.4", length={5, 14}, coverage=2.6, angle=0, medium=0.35, load=0.45,
  color=function(x, y) local p = w:to_ground(x, y); local Z = p and p[3] or 3000
    return mix(mix("#8e9fb2", "#c3cbd0", smoothstep(300, 2500, Z)), sk:airlight(x), 0.5*w:aerial(Z)) end})
-- hedgerows and copses along some field edges
hedge = mask(function(x, y)
  if y < HZ + 2 then return 0 end
  local p = w:to_ground(x, y); if not p then return 0 end
  local X, Z = p[1], p[3]
  local ca, sa = math.cos(0.45), math.sin(0.45)
  local U, V = X*ca - Z*sa, X*sa + Z*ca
  local _, _, edge, r = fields:at(U/55, V/95)
  if Z < 280 then return 0 end
  local keep = (fnz(X*0.9, Z*0.35) > 0.05 and fnz(X*4.1 + 77, Z*1.3) > -0.25) and 1 or 0
  local _, _, _, r2 = fields:at(U/55 + 0.37, V/95 + 0.61)
  local copse = (r2 > 0.94) and 1 or 0
  return math.max(keep * (1 - smoothstep(0.02, 0.05, edge)), copse * (1 - smoothstep(0.1, 0.25, edge)))
end) - river
work(hedge, {hand="hatch", tool="round 1.6", length={2, 5}, coverage=2.5, angle=function(x,y) return 2.2*fnz(x*3,y*3) end, medium=0.25,
  color=function(x, y) local p = w:to_ground(x, y); local Z = p and p[3] or 3000
    return mix("#34472a", sk:airlight(x), clamp(w:aerial(Z)*0.9, 0, 0.85)) end})
