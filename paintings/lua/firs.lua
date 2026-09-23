-- easel session "firs": a painting replayed chunk by chunk.
--   easel run paintings/lua/firs.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.4, seed=21}; print(H)

--@ chunk 2 · clock 0
-- a still winter afternoon: a pale gray sky warming toward the horizon
HZ = 505
sky = function(x, y) return gradient({{0,"#8a95a3"},{0.55,"#b3b6b4"},{0.9,"#d6ceb8"},{1,"#ddd2b6"}}, y/HZ) end
skym = above(function(x) return HZ + 12 end)
work(skym, {hand="broad", color=sky, angle=0, coverage=4.2, medium=0.25})
blend(skym, {angle=0})

--@ chunk 3 · clock 0
wait(24*60)
-- snow on the ground, a faint rise to the right; cool where it turns from the light
local g = noise{seed=4, octaves=4, period=240}
GROUND = function(x) return HZ + 6 + 10 * g(x, 0) end
snow = below(GROUND)
work(snow, {hand="body", length={20, 60}, angle=0.02, coverage=3.4, medium=0.18,
  color=function(x, y) return mix("#bdbdc0", "#d9d4c6", smoothstep(HZ, H, y) * 0.6 + 0.25 * g(x * 2, y)) end})

--@ chunk 4 · clock 1440
wait(24*60)
-- the lone spire: drawn as an envelope, grown into it, painted in short hatched strokes
SPIRE = {{520,112},{528,170},{540,240},{552,330},{566,420},{580,492,"c"},{548,505},{520,500},{492,506},{462,494,"c"},{478,420},{494,330},{505,240},{512,170}}
spire = fir{envelope=outline{pts=SPIRE, char="soft", seed=3}, foot={521, 540}, habit="spire", seed=5}
print(spire)
local nd = spire:needles()
-- 1. the dark body of the needles, hatched along the hang of the shoots
local turn = noise{seed=6, period=9}
work(nd, {hand="hatch", tool="round 1.8", length={3, 7}, coverage=2.6, clip=nd, medium=0.2,
  angle=function(x, y) return 1.57 + (x < 521 and 0.5 or -0.5) + 0.35 * turn(x, y) end, angle_jitter=0.4,
  color=function(x, y) return shift(mix("#1c2621", "#25302a", smoothstep(120, 500, y)), 0.015 * turn(x, y), 0, 0) end})
-- 2. the stem and boughs where they show
local rb = brush("rigger", 1.2)
rb:load("#2a2622", 0.9)
rb:stroke(spire.leader.pts, {pressure={0.9, 0.15}, ramps={0.02, 0.2}})
for i, b in ipairs(spire.boughs) do
  if i % 6 == 1 then rb:reload(b.dead and "#5b5650" or "#1d211e", 0.8) end
  rb:stroke(b.pts, {pressure={0.7, 0.05}, ramps={0.05, 0.6}})
end
-- 3. hatched strokes: the shaded shoots, then the half-lit, then the few that catch the light
local hb = brush("round", spire.hatch)
print(spire:paint(hb, {color="#18211d", lit={0, 0.5}}))
print(spire:paint(hb, {color="#34402f", lit={0.5, 0.7}}))
print(spire:paint(brush("round", spire.hatch * 0.8), {color="#667052", lit={0.7, 1}, every=6}))

--@ chunk 5 · clock 2880
-- the ragged old fir, right of the spire: drawn lopsided, a crooked top, a torn flank
OLDFIR = {{742,168},{752,210},{770,262},{790,300},{782,330},{806,372},{826,420},{812,448},{838,500,"c"},{790,512},{742,506},{700,516,"c"},{690,470},{712,430},{700,380},{716,330},{724,280},{734,220}}
old = fir{envelope=outline{pts=OLDFIR, char="soft", seed=8}, foot={748, 560}, habit="old", seed=12}
-- the young fir, near and low on the right: dense to the snow
YOUNG = {{906,392},{922,440},{946,500},{966,560},{978,600,"c"},{930,612},{880,604,"c"},{870,560},{888,500},{896,440}}
young = fir{envelope=outline{pts=YOUNG, char="soft", seed=9}, foot={915, 618}, habit="young", seed=14}
print(old); print(young)
function paint_fir(f, o)
  o = o or {}
  local nd = f:needles()
  local ax = f.foot[1]
  local turn = noise{seed=o.seed or 6, period=9}
  local dark = o.dark or {"#1c2621", "#25302a"}
  work(nd, {hand="hatch", tool="round 1.8", length={3, 7}, coverage=2.4, clip=nd, medium=0.2,
    angle=function(x, y) return 1.57 + (x < ax and 0.5 or -0.5) + 0.35 * turn(x, y) end, angle_jitter=0.4,
    color=function(x, y) return shift(mix(dark[1], dark[2], smoothstep(f.apex[2], f.crown_base, y)), 0.015 * turn(x, y), 0, 0) end})
  local rb = brush("rigger", o.rigger or 1.2)
  rb:load("#221f1b", 0.9)
  local lp = f.leader.pts
  rb:stroke(lp, {pressure={0.95, 0.15}, ramps={0.02, 0.2}})
  for i, b in ipairs(f.boughs) do
    if i % 6 == 1 or b.dead then rb:reload(b.dead and "#6a655d" or "#1d211e", 0.8) end
    rb:stroke(b.pts, {pressure={b.dead and 0.8 or 0.7, 0.05}, ramps={0.05, 0.6}})
  end
  local tw = brush("rigger", 0.6)
  f:paint(tw, {color="#77716a", kind="twig", every=5})
  local hb = brush("round", f.hatch)
  f:paint(hb, {color=o.shade or "#18211d", lit={0, 0.5}, kind={"under", "top"}})
  f:paint(hb, {color=o.mid or "#34402f", lit={0.5, 0.7}, kind={"under", "top"}})
  f:paint(brush("round", f.hatch * 0.8), {color=o.light or "#667052", lit={0.7, 1}, kind={"under", "top"}, every=6})
end
-- the trunk of the old one shows below and through its gaps: paint it first, gray-brown, lit on the left
local tb = brush("round", 3)
tb:load("#3b342d", 0.9)
tb:stroke(old.leader.pts, {pressure={0.95, 0.3}, ramps={0.02, 0.4}})
paint_fir(old, {seed=13})
paint_fir(young, {seed=15, light="#6f7755"})

--@ chunk 6 · clock 2880
-- the wood's edge on the left: a drawn skyline, four rows of firs receding behind it
WOODTOP = {{-10,150},{50,132},{110,158},{170,146},{230,176},{290,214},{340,262},{385,320},{420,392},{445,455}}
wood = fir_wood{skyline=outline{pts=WOODTOP, open=true, char="soft", lobe=14, seed=31}, foot=516, depth=4, count=13, seed=32}
print(wood)
local air = "#a4a7a1"            -- the air between the rows: the sky low down, a shade darker
local turn = noise{seed=33, period=8}
local function paint_row(r)
  local h, s = wood:haze(r), wood:scale(r)
  local nd = wood:needles(r)
  local tool = "round " .. string.format("%.1f", math.max(0.9, 1.8 * s))
  work(nd, {hand="hatch", tool=tool, length={2, 7 * s + 1}, coverage=2.4, clip=nd, medium=0.2,
    angle=function(x, y) return 1.57 + 0.45 * turn(x, y) end, angle_jitter=0.5,
    color=function(x, y) return shift(mix(mix("#1c2621", "#27322b", smoothstep(150, 500, y)), air, 0.62 * h), 0.012 * turn(x, y), 0, 0) end})
  -- trunks under the crowns, fainter with distance
  local rb = brush("rigger", math.max(0.5, 1.4 * s))
  for n, f in ipairs(wood:trees(r)) do
    if n % 5 == 1 then rb:reload(mix("#221f1b", air, 0.6 * h), 0.85) end
    rb:stroke(f.leader.pts, {pressure={0.9, 0.2}, ramps={0.02, 0.3}})
  end
  -- hatched shoots on the nearer rows; the far ones stay a hatched tone
  if r <= 2 then
    local hb = brush("round", 2.2 * s + 0.3)
    wood:paint(hb, r, {color=mix("#161e1a", air, 0.6 * h), lit={0, 0.55}})
    wood:paint(hb, r, {color=mix("#3a4533", air, 0.6 * h), lit={0.55, 1}, every=6})
  end
end
paint_row(4); paint_row(3)
-- the floor of the wood: dim and shaded under the crowns, a little lighter toward the snow at its foot
local fl = wood:floor():roughen(7, 22, 34, 3)
work(fl, {hand="body", length={4, 12}, coverage=3, clip=fl, angle=1.5, angle_jitter=0.4, medium=0.2, hug=false,
  color=function(x, y) return mix("#2a302d", "#555c5a", smoothstep(485, 518, y)) end})
paint_row(2); paint_row(1)
