-- easel session "r9a3_winter": a painting replayed chunk by chunk.
--   easel run paintings/lua/r9a3_winter.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).
-- sittings enforced: a sitting ends at its length; the easel refuses marks until rest(hours) (notes/time.md)

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.4, seed=29}; print(W, H); print(pal)

--@ chunk 2 · clock 0
HZ = 468
-- the drawing: horizon ruled, the mound, oak trunk, the cross, the walker
MOUND = {{-10,588},{60,572},{140,556},{220,546},{300,540},{370,546},{430,560},{500,582},{570,598},{650,606},{760,610},{880,606},{1010,600}}
local h = pencil("2H")
h:rule({0, HZ}, {1000, HZ}, {pressure=0.25})
h:sketch(MOUND, {pressure=0.3})
h:sketch({{318,548},{314,470},{322,390},{312,300},{330,220}}, {pressure=0.3})
local b = pencil("HB")
b:line({{596,604},{598,530},{600,488}}, {pressure={0.4,0.5,0.4}})
b:line({{582,512},{616,510}}, {pressure=0.45})
b:sketch({{700,622},{702,600},{704,580},{703,566}}, {pressure=0.35})

--@ chunk 3 · clock 0
-- the evening sky: gray-violet above, a lemon-rose glow low where the sun went down (left)
bands = noise{seed=41, octaves=4, period=260, stretch={0.03, 6}}
skycol = function(x, y)
  local t = clamp(y / HZ, 0, 1)
  local c = gradient({{0, "#58647c"}, {0.35, "#7c8497"}, {0.62, "#a8a3a8"}, {0.82, "#cdb9aa"}, {1, "#e4d3a8"}}, t)
  -- the glow sits left of center and fades to mauve on the right
  local glow = math.exp(-((x - 260) / 420)^2) * smoothstep(0.55, 1, t)
  local right = mix(c, "#b6a6ad", 0.55 * smoothstep(0.6, 1, t))
  c = mix(right, c, glow)
  -- long cloud banks lying across, darker gray-violet
  local bk = smoothstep(0.15, 0.55, bands(x, y)) * (0.6 - 0.4 * t)
  return mix(c, "#6a6878", bk)
end
skym = above(function(x) return HZ + 14 end)
work(skym, {hand="broad", color=skycol, angle=function(x, y) return 0.02 * bands(x * 0.3, y) end,
  length={120, 320}, coverage=4.5, medium=0.3})
-- blend later

--@ chunk 4 · clock 0
-- a second lay-in wet into the first: shorter sweeps, fuller loads, to close the gaps
work(skym, {hand="broad", color=skycol, angle=function(x, y) return 0.03 * bands(x * 0.3, y + 40) end,
  length={60, 170}, coverage=3.5, medium=0.3, load=1})
blend(skym, {angle=0.01})

--@ chunk 5 · clock 0
wait(24*60)
-- the snow in one wet lay-in, the mound modeled in it: its face toward the glow light and warm,
-- the plain in its lee cooler toward the crest, the far slope and the near drifts blue-violet
mound = function(x)
  local pts = MOUND
  for i = 1, #pts - 1 do
    local a, b = pts[i], pts[i + 1]
    if x >= a[1] and x <= b[1] then local t = (x - a[1]) / (b[1] - a[1]); return lerp(a[2], b[2], smoothstep(0, 1, t)) end
  end
  return 600
end
drift = noise{seed=52, octaves=4, period=140, stretch={0.05, 3}}
crestn = noise{seed=53, octaves=3, period=40}
snowcol = function(x, y)
  local d = clamp((y - HZ) / (714 - HZ), 0, 1)
  local far = mix("#cbc4c6", "#dcd2b6", math.exp(-((x - 260) / 380)^2) * 0.7)
  local c = mix(far, "#b9b8c4", smoothstep(0.05, 0.6, d))
  local m = mound(x) + 1.5 * crestn(x, 0)
  local onm = smoothstep(m - 1.5, m + 1.5, y)          -- 1 on the mound
  local west = 1 - smoothstep(400, 560, x)             -- the part of the mound facing the glow
  -- lee: the plain just behind the crest, cooler toward it
  local lee = (1 - onm) * smoothstep(HZ + 12, m, y) * (1 - smoothstep(560, 700, x))
  c = mix(c, "#b6b5c3", 0.8 * lee)
  -- face: light and warm under the crest, fading down the slope
  local face = onm * west * (1 - smoothstep(m + 4, m + 80, y))
  c = mix(c, mix("#e8ddca", "#dcd4cb", smoothstep(m, m + 60, y)), 0.9 * face)
  -- the slope turning away on the right of the mound
  local away = onm * smoothstep(400, 520, x) * (1 - smoothstep(600, 700, x)) * (1 - smoothstep(m + 6, m + 90, y))
  c = mix(c, "#b0b1c2", 0.8 * away)
  -- drifts: soft blue-violet shadows, stronger near
  c = mix(c, "#a4a7bb", (0.2 + 0.4 * d) * smoothstep(0.1, 0.6, drift(x, y)))
  return mix(c, "#9a9aab", 0.45 * smoothstep(0.6, 1, d))
end
snowm = below(function(x) return HZ - 4 end)
work(snowm, {hand="body", color=snowcol, angle=function(x, y) return 0.04 * drift(x, y) end,
  length={25, 80}, coverage=3.5, medium=0.2})
blend(snowm, {angle=0.02})

--@ chunk 6 · clock 1440
wait(24*60)
-- the far wood on the plain, right of center: a low dark band, hazed by the evening
WOODTOP = {{548,471},{566,462},{590,457},{612,452},{640,449},{668,445},{700,447},{730,441},{760,444},{792,438},{826,442},{860,436},{900,440},{940,437},{975,442},{1010,440}}
wood = fir_wood{skyline=outline{pts=WOODTOP, open=true, char="soft", lobe=6, seed=61}, foot=474,
  depth=2, count=46, recede=0.6, air=0.5, seed=62}
print(wood)
air = "#9d9aa6"
for r = 2, 1, -1 do
  local hz, s, nd = wood:haze(r), wood:scale(r), wood:needles(r)
  local base = mix("#3a4046", air, 0.35 + 0.35 * hz)
  work(nd, {hand="hatch", tool="round 0.9", length={1.5, 4}, coverage=2.6, clip=nd, angle=1.57, angle_jitter=0.4, color=base})
  local rb = brush("rigger", 0.5)
  for n, f in ipairs(wood:trees(r)) do
    if n % 6 == 1 then rb:reload(mix("#34373a", air, 0.4 + 0.3 * hz), 0.8) end
    rb:stroke(f.leader.pts, {pressure={0.8, 0.1}, ramps={0.02, 0.4}})
  end
end

--@ chunk 7 · clock 2880
-- snow dragged across the far trees' feet: a few short horizontal pulls, not a band
local feet = rect(545, 471, 470, 6):roughen(1.5, 14, 72, 1)
work(feet, {hand="body", tool="round 1.6", coverage=1.2, angle=function(x, y) return 0.03 * drift(x, y) end, length={6, 22}, hug=false,
  color="#bdb9c2", medium=0.2})

--@ chunk 8 · clock 2880
dry()
WORLD = world{horizon=HZ, sun={azimuth=-80, elevation=2}}
-- an old oak on the mound: a short massive bole, limbs breaking out low and wide, torn open on the left,
-- one long limb reaching right toward the cross
OAK = {{110,420,"c"},{118,360},{160,330,"c"},{150,280},{196,250,"c"},{224,206},{270,214,"c"},{290,170},{332,150,"c"},
       {352,196},{396,200,"c"},{430,236},{478,246,"c"},{524,282},{580,318,"c"},{556,350},{506,352,"c"},{478,392},
       {428,420,"c"},{388,410},{356,440,"c"},{300,436},{262,452,"c"},{220,430},{180,446,"c"},{140,440}}
oak = tree_in{crown=outline{pts=OAK, char="broken", seed=81}, trunk={{322,556},{316,520},{326,488}},
  species="oak", season="winter", sun=WORLD, seed=84, detail=0.65, girth=0.12, voids=1.6}
print(oak)

--@ chunk 9 · clock 25747.828125
-- the oak nearly in silhouette against the glow: stout wood as body paint (its foot stops at the snow),
-- a faint lit flank toward the west, then the limbs thick to thin, all dark
footcut = above(function(x) return 553 + 2 * crestn(x, 5) end)
local thick = oak:wood(3.5) * footcut
work(thick, {hand="body", tool="round 2", length={4, 12}, coverage=3.5, angle=1.5, clip=thick, color="#342e29", medium=0.15})
local stout = oak:wood(7) * footcut
work(stout * mask(function(x, y) return 1 - stout:at(x - 2.2, y + 0.6) end),
     {hand="body", tool="round 1.4", coverage=1.6, angle=1.5, clip=thick, color="#50473f", medium=0.15})
oak:paint_wood(brush("round", 2.4), {color="#37302a", min=1.2, max=3.5})
oak:paint_wood(brush("rigger", 0.9), {color="#3b342e", min=0.5, max=1.2, every=2})
oak:paint_wood(brush("rigger", 0.55), {color="#48413c", max=0.5, pressure=0.04, every=2})

--@ chunk 10 · clock 25747.828125
-- the wayside cross, old wood leaning a little right, half buried; a small gable roof over it
local post = brush{kind="flat", width=5.6, stiffness=0.7}
post:load("#3b3530", 1, {medium=0.12})
post:stroke({{606.4, 510}, {603.4, 542}, {600.6, 578}, {598.2, 612}}, {pressure={0.85, 0.9}, ramps={0.0, 0.0}, orient="across", shake=0.25})
post:reload("#403934", 0.9)
post:stroke({{598.4, 612}, {600.8, 580}, {603.6, 545}, {606.2, 512}}, {pressure={0.8, 0.85}, ramps={0.0, 0.02}, orient="across", shake=0.25})
local bar = brush{kind="flat", width=4.4, stiffness=0.7}
bar:load("#3a342f", 1, {medium=0.12})
bar:stroke({{578, 531.5}, {604, 529.8}, {633, 532.4}}, {pressure={0.85, 0.85}, ramps={0.0, 0.02}, orient="across", shake=0.25})
bar:stroke({{633, 532.6}, {604, 530.0}, {578.5, 531.8}}, {pressure={0.8, 0.8}, ramps={0.0, 0.02}, orient="across", shake=0.25})
-- weathered gray on the faces toward the glow
local lit = brush("round", 1.1)
lit:load("#6c645c", 0.6, {medium=0.2})
lit:stroke({{604.2, 513}, {601.8, 545}, {599.2, 580}, {597.6, 602}}, {pressure={0.5, 0.35}, ramps={0.1, 0.4}, shake=0.5})
lit:stroke({{580, 530.0}, {603, 528.6}}, {pressure={0.45, 0.3}, ramps={0.2, 0.4}, shake=0.5})
-- the gable: two short boards, dark underside
local roof = brush{kind="flat", width=2.6, stiffness=0.7}
roof:load("#2f2a26", 1)
roof:stroke({{596.5, 512.5}, {606.5, 502.5}}, {pressure={0.85, 0.85}, ramps={0, 0}, orient="across"})
roof:stroke({{606.5, 502.5}, {616.5, 512.8}}, {pressure={0.85, 0.85}, ramps={0, 0}, orient="across"})
-- snow lying on the roof and the crossbar
local sn = brush("round", 1.5)
sn:load("#e3ddd4", 0.9, {medium=0.1})
sn:stroke({{580, 528.9}, {592, 528.2}, {602, 527.8}}, {pressure={0.7, 0.45}, ramps={0.1, 0.3}, shake=0.6})
sn:stroke({{609, 528.2}, {620, 528.9}, {630, 530.2}}, {pressure={0.65, 0.35}, ramps={0.1, 0.4}, shake=0.6})
sn:stroke({{597, 510.2}, {606.3, 500.8}}, {pressure={0.6, 0.4}, ramps={0.1, 0.3}})
sn:stroke({{606.7, 500.8}, {615.4, 509.6}}, {pressure={0.55, 0.3}, ramps={0.1, 0.4}})

--@ chunk 11 · clock 25747.828125
-- the walker seen from behind, in a long dark greatcoat and a hat, a stick, going toward the cross
COAT = outline{pts={{695.5,575,"c"},{699,572.6},{704.5,572.8},{708.6,575.4,"c"},{709.8,584},{711.4,596},{713.2,607.5,"c"},
  {706,608.6},{700,608.2},{692.6,607,"c"},{694,596},{694.4,584}}, char="firm", seed=111}
coatm = COAT:mask()
work(coatm, {hand="body", tool="round 1.6", length={3, 9}, coverage=3.2, angle=1.62, angle_jitter=0.25, edge="firm",
  color=function(x, y) return mix("#262b29", "#2f3431", smoothstep(575, 606, y)) end, medium=0.15})
-- legs: the left planted, the right stepping off, heel up
local leg = brush("round", 2.2)
leg:load("#24211f", 1)
leg:stroke({{698.2, 606}, {698.0, 612}, {697.6, 618.2}}, {pressure={0.8, 0.8}, ramps={0, 0.05}})
leg:stroke({{705.6, 606}, {706.6, 611}, {708.4, 615.6}}, {pressure={0.75, 0.7}, ramps={0, 0.05}})
-- head (hair seen from behind), a high collar, and a round-crowned hat
local headm = ellipse(702.1, 570.2, 2.9, 3.4):roughen(0.3, 3, 112, 0.3)
work(headm, {hand="detail", tool="round 1.2", coverage=3, color="#2b2623", edge="firm", length={1, 3}})
local collar = poly({{697.4,575.2},{698.6,571.8},{702,573.2},{705.6,571.8},{707,575.4}}):roughen(0.3, 3, 113, 0.3)
work(collar, {hand="detail", tool="round 1.2", coverage=3, color="#232826", edge="firm", length={1, 3}})
local hatm = (ellipse(702.1, 563.6, 3.6, 3.4) + ellipse(702.1, 567.0, 6.2, 1.3)):roughen(0.25, 3, 114, 0.3)
work(hatm, {hand="detail", tool="round 1.2", coverage=3.5, color="#1b1b1c", edge="firm", length={1, 3}, angle=0})
-- the right arm reaching forward to the stick, and the stick planted ahead
local arm = brush("round", 2.4)
arm:load("#272b29", 1)
arm:stroke({{708.4, 577}, {710.4, 585}, {711.2, 591.5}}, {pressure={0.8, 0.7}, ramps={0, 0.1}})
local st = brush{kind="rigger", width=0.9, point=0.5}
st:load("#2e2823", 0.9)
st:stroke({{710.8, 588}, {713.6, 603}, {716.4, 619}}, {pressure={0.7, 0.6}, ramps={0, 0.1}})
-- the glow catching the coat's left shoulder and back edge
local rim = brush{kind="round", width=1.0, point=0.6}
rim:load("#6e675f", 0.6, {medium=0.2})
rim:stroke({{698.6, 573.4}, {696, 576}, {695.2, 586}, {694.4, 598}}, {pressure={0.5, 0.15}, ramps={0.1, 0.6}, shake=0.4})

--@ chunk 12 · clock 25747.828125
dry()
-- snow drifted against the feet: each pull loaded with the snow beside it, a touch lighter on the heap's top
local function drag(pts, w, dL, p)
  local x0, y0 = pts[1][1], pts[1][2]
  local base = sample(x0, y0 + 4, 3)
  local b = brush{kind="filbert", width=w, stiffness=0.35}
  b:load(shift(base, dL or 0, 0, 0), 0.9, {medium=0.2})
  b:stroke(pts, {pressure=p or {0.55, 0.15}, ramps={0.15, 0.6}, shake=0.8})
end
for k = 0, 5 do   -- the oak: banked higher on the windward left
  local y = 553 - k * 1.6 + rand(-0.8, 0.8)
  drag({{262 + 6 * k + rand(-3, 3), y + 3}, {292 + 3 * k, y - 1.5}, {316 + rand(-2, 2), y - 2 + 0.5 * k}, {348 - 2 * k, y + 2.5}}, rand(3.5, 5.5), 0.005 * k)
end
drag({{352, 559}, {336, 555.5}, {326, 552.8}}, 3.5, -0.03, {0.5, 0.1})
for k = 0, 2 do   -- the cross
  local y = 609.5 - k * 1.2
  drag({{584 + 3 * k, y + 1}, {596, y - 1.8}, {603, y - 1.5}, {612 - 2 * k, y + 1}}, rand(2.5, 3.5), 0.004 * k)
end
drag({{693, 619.6}, {698, 617.6}, {703, 618}}, 1.8, 0, {0.5, 0.2})    -- the walker's boots
drag({{704.5, 617.4}, {709, 615.8}, {712.5, 616.6}}, 1.6, 0, {0.5, 0.2})
drag({{713, 620.4}, {716.5, 618.8}, {719.5, 619.8}}, 1.5, 0, {0.45, 0.2})

--@ chunk 13 · clock 38794.0380859375
-- the cross's wood darkened with a thin warm-dark glaze, only where the wood is (the snow on it stays)
local woodm = mask(function(x, y)
  if x < 574 or x > 638 or y < 498 or y > 614 then return 0 end
  local s = sample(x, y)
  return 1 - smoothstep(0.5, 0.7, s.L)
end)
glaze(woodm, {color="#2e2621", coats=0.45, pigment="semi"})

--@ chunk 14 · clock 48171.5732421875
dry()
-- bury the post's foot: a heap of snow over the bottom, its lee side a little cooler
local base = sample(586, 618, 3)
local b = brush{kind="filbert", width=4, stiffness=0.35}
for k = 0, 4 do
  b:reload(shift(base, 0.006 * (2 - k), 0, -0.002 * k), 0.9)
  local y = 614 - k * 1.9 + rand(-0.5, 0.5)
  b:stroke({{586 + 1.5 * k + rand(-1, 1), y + 1.2}, {594, y - 1.2}, {601, y - 1.6}, {611 - 1.5 * k, y + 0.8}}, {pressure={0.6, 0.2}, ramps={0.12, 0.55}, shake=0.7})
end
-- fresh snow back on the crossbar and roof, broken, not a clean line
local sn = brush{kind="round", width=1.2, point=0.5}
sn:load("#dcd6cf", 0.8, {medium=0.1})
sn:stroke({{581, 529.0}, {587, 528.6}, {592, 528.4}}, {pressure={0.55, 0.25}, ramps={0.1, 0.5}, shake=0.7})
sn:stroke({{595.5, 528.3}, {601.5, 527.8}}, {pressure={0.5, 0.2}, ramps={0.1, 0.5}, shake=0.7})
sn:stroke({{610, 528.5}, {618, 529.0}, {626, 530.0}}, {pressure={0.5, 0.2}, ramps={0.1, 0.5}, shake=0.7})
sn:stroke({{598.5, 509.2}, {603, 504.6}, {606.2, 501.5}}, {pressure={0.55, 0.2}, ramps={0.1, 0.5}})
sn:stroke({{607.2, 501.8}, {611, 505.6}}, {pressure={0.45, 0.15}, ramps={0.1, 0.5}})

--@ chunk 15 · clock 48171.5732421875
dry()
-- two stones pushing through the snow, painted by hand: dark body, a snow cap, snow over the base
STONES = {
  {pts={{92,678},{98,662},{116,649},{146,642},{174,646},{198,657},{214,676}}, cap={{100,660},{120,647.5},{146,641.5},{172,645},{190,652}}, lean=0.2},
  {pts={{236,700},{244,689},{258,683},{276,684},{290,700}}, cap={{245,688},{258,682.5},{274,683}}, lean=0.1},
}
local shade = noise{seed=171, period=9, octaves=3}
for i, st in ipairs(STONES) do
  local m = poly(st.pts, true):roughen(1.0, 8, 170 + i, 0.4)
  local x0, x1 = st.pts[1][1], st.pts[#st.pts][1]
  work(m, {hand="body", tool="filbert 2.5", length={4, 10}, coverage=3.4, medium=0.18, edge="firm",
    angle=function(x, y) return 1.2 + 0.9 * (x - (x0 + x1) / 2) / (x1 - x0) + 0.3 * shade(x, y) end,
    color=function(x, y)
      local t = (x - x0) / (x1 - x0)
      local c = mix("#6c675f", "#3c3c44", smoothstep(0.15, 0.7, t))
      return mix(c, "#2e2e34", 0.35 * smoothstep(0.2, 0.7, shade(x, y)))
    end})
  -- the cap: long pulls along the top, heavy in the middle, cool where it turns away
  local b = brush{kind="filbert", width=(i == 1) and 5.5 or 3.5, stiffness=0.4}
  b:load("#d8d3cc", 0.95, {medium=0.1})
  b:stroke(st.cap, {pressure={0.55, 0.85, 0.3}, ramps={0.2, 0.45}, shake=0.8})
  b:reload("#c3c3cc", 0.8)
  local tail = {}
  for k = math.max(1, #st.cap - 2), #st.cap do tail[#tail + 1] = {st.cap[k][1] + 2, st.cap[k][2] + 1.5} end
  tail[#tail + 1] = {st.pts[#st.pts - 1][1] + 2, st.pts[#st.pts - 1][2] + 2}
  b:stroke(tail, {pressure={0.5, 0.2}, ramps={0.1, 0.6}, shake=0.8})
end

--@ chunk 16 · clock 56628.9853515625
dry()
-- the snow drifted up over the stones' feet: one or two pulls each, loaded with the snow beneath
for i, st in ipairs(STONES) do
  local x0, x1, yb = st.pts[1][1], st.pts[#st.pts][1], st.pts[1][2]
  local b = brush{kind="filbert", width=(i == 1) and 10 or 6, stiffness=0.35}
  b:load(sample((x0 + x1) / 2, yb + 10, 4), 0.95, {medium=0.2})
  b:stroke({{x0 - 10, yb + 3.5}, {x0 + 25, yb + 0.5}, {(x0 + x1) / 2 + 10, yb}, {x1 + 8, yb + 2.5}}, {pressure={0.4, 0.75, 0.2}, ramps={0.15, 0.5}, shake=0.8})
  b:reload(shift(sample((x0 + x1) / 2, yb + 10, 4), -0.02, 0, -0.01), 0.8)
  b:stroke({{x1 + 12, yb + 5}, {(x0 + x1) / 2 + 20, yb + 2.5}, {x0 + 10, yb + 4}}, {pressure={0.5, 0.2}, ramps={0.1, 0.6}, shake=0.8})
end

--@ chunk 17 · clock 73188.6396484375
dry()
-- his way back through the snow toward the lower right: a trodden furrow, then the dents of his steps,
-- spaced in perspective (the step grows as it nears), none alike
local path = {{713, 621}, {724, 633}, {737, 646}, {752, 660}, {768, 675}, {785, 691}, {803, 709}, {812, 718}}
local function at(u)   -- a point along the path, u in 0..1
  local L, seg = 0, {}
  for i = 1, #path - 1 do seg[i] = math.sqrt((path[i+1][1]-path[i][1])^2 + (path[i+1][2]-path[i][2])^2); L = L + seg[i] end
  local d = u * L
  for i = 1, #path - 1 do
    if d <= seg[i] then local t = d / seg[i]; return lerp(path[i][1], path[i+1][1], t), lerp(path[i][2], path[i+1][2], t) end
    d = d - seg[i]
  end
  return path[#path][1], path[#path][2]
end
-- the furrow: one thin cool drag, broken
local fur = brush{kind="filbert", width=3, stiffness=0.3}
for k = 0, 3 do
  local u0, u1 = k / 4, (k + 1) / 4 - 0.03
  local pts = {}
  for j = 0, 4 do local x, y = at(lerp(u0, u1, j / 4)); pts[#pts + 1] = {x + rand(-0.5, 0.5), y} end
  fur:reload(shift(sample(pts[3][1], pts[3][2], 3), -0.025, 0.001, -0.012), 0.35)
  fur:stroke(pts, {pressure={0.3 + 0.1 * k, 0.5 + 0.1 * k}, ramps={0.2, 0.3}, shake=1})
end
-- the steps
local u, n = 0.0, 0
while u < 1 do
  local x, y = at(u)
  local s = 0.8 + (y - 618) / 28
  n = n + 1
  local side = (n % 2 == 0) and 1 or -1
  local cx, cy = x + side * 1.3 * s + rand(-0.5, 0.5) * s, y + rand(-0.4, 0.4)
  local w = (1.8 + rand(0, 1.2)) * s
  local b = brush{kind="filbert", width=clamp(0.9 * s, 0.9, 5), stiffness=0.4}
  b:load(shift(sample(cx, cy, 2), -0.045 - rand(0, 0.02), 0.002, -0.016), 0.7, {medium=0.2})
  local tilt = rand(-0.25, 0.25) * s
  b:stroke({{cx - w * 0.5, cy + tilt}, {cx + rand(-0.3, 0.3), cy + rand(-0.3, 0.3) * s}, {cx + w * 0.5, cy - tilt}}, {pressure={0.55, 0.45}, ramps={0.2, 0.35}, orient="across", shake=0.8})
  local lp = brush{kind="round", width=clamp(0.35 * s, 0.5, 1.8), point=0.5}
  lp:load(shift(sample(cx, cy - s, 2), 0.035, 0, 0.004), 0.6, {medium=0.15})
  lp:stroke({{cx - w * 0.4 + rand(-0.4, 0.4), cy - 0.6 * s}, {cx + w * 0.4, cy - (0.5 + rand(0, 0.3)) * s}}, {pressure={0.35, 0.15}, ramps={0.2, 0.5}, shake=0.6})
  u = u + (0.028 + 0.05 * u) * rand(0.8, 1.2)
end
print(n, "steps")

--@ chunk 18 · clock 81438.1611328125
-- dry grass through the snow, laid over the finished snow in fine upturning strokes
local clumps = {}
local function add(x, y, s) clumps[#clumps + 1] = {x, y, s} end
-- by the stones
add(86, 680, 1.2); add(118, 684, 1.0); add(208, 679, 1.1); add(226, 690, 0.8); add(296, 702, 1.0); add(60, 700, 1.3); add(30, 668, 1.1)
-- along the mound's crest and face, small with distance
for i, x in ipairs(uneven(14, 20, 520, 0.6, 0.5, 181)) do add(x, mound(x) + rand(2, 26), 0.45 + rand(0, 0.25)) end
-- round the cross and in the right foreground
add(590, 612, 0.5); add(612, 613, 0.45); add(574, 616, 0.4)
for i, x in ipairs(uneven(9, 840, 990, 0.6, 0.4, 182)) do add(x, rand(640, 708), 0.8 + rand(0, 0.5)) end
for i, x in ipairs(uneven(6, 380, 560, 0.6, 0.4, 183)) do add(x, rand(660, 710), 0.9 + rand(0, 0.4)) end
local g = brush{kind="rigger", width=0.7, point=1}
local cols = {"#7b6a4c", "#8e7b58", "#5b4e3b", "#a08a62", "#4a4136"}
for i, c in ipairs(clumps) do
  local x, y, s = c[1], c[2], c[3]
  local nb = math.floor(4 + 7 * s * rand(0.6, 1.2))
  local lean = randn(0.15, 0.25)
  for k = 1, nb do
    if k % 3 == 1 then g:reload(cols[1 + (i + k) % #cols], 0.7) end
    local bx = x + randn(0, 2.2 * s)
    local h = (5 + rand(0, 9)) * s
    local a = -1.57 + lean + randn(0, 0.28)
    local bend = randn(0, 0.25)
    local mx, my = bx + math.cos(a) * h * 0.5, y + math.sin(a) * h * 0.5
    local tx, ty = bx + math.cos(a + bend) * h, y + math.sin(a + bend) * h
    g:stroke({{bx, y + rand(0, 1.2)}, {mx, my}, {tx, ty}}, {pressure={clamp(0.35 + 0.35 * s, 0.25, 0.85), 0}, ramps={0.05, 0.7}})
  end
end
print(#clumps, "clumps")

--@ chunk 19 · clock 81438.1611328125
-- far off on the left, under the glow: the plain's edge as a broken line of low hedges and copses,
-- and a village church, all nearly lost in the air
local hn = noise{seed=193, period=18, octaves=3}
local hedge = mask(function(x, y)
  if x > 560 then return 0 end
  local top = 467 - 4.2 * smoothstep(-0.3, 0.7, hn(x, 0)) - 1.6 * smoothstep(0, 0.8, hn(x * 4, 17)) - 3 * math.exp(-((x - 205) / 16)^2) - 3.5 * math.exp(-((x - 40) / 22)^2)
  local on = smoothstep(-0.3, 0.05, hn(x * 0.7, 40)) * (0.35 + 0.65 * smoothstep(-0.2, 0.3, hn(x * 2.3, 71)))
  return on * smoothstep(top - 0.4, top + 0.4, y) * (1 - smoothstep(467.5, 468.5, y))
end) * (-oak:mask():grow(0.6))
work(hedge, {hand="hatch", tool="round 0.8", length={1, 3}, coverage=1.8, angle=1.45, angle_jitter=0.7, edge="soft", hug=false,
  color=function(x, y) return mix("#9c95a2", "#bab1b4", smoothstep(60, 540, x)) end, medium=0.25})
local tower = rect(124.6, 452.5, 3.4, 15)
local nave = poly({{128, 468}, {128, 461.2}, {133.5, 458.6}, {140.5, 461.4}, {140.5, 468}})
work(tower + nave, {hand="detail", tool="round 0.8", length={1, 3}, coverage=3, angle=1.57, edge="firm", color="#8e8794", medium=0.2})
local sp = brush{kind="round", width=1.6, point=1}
sp:load("#8a8390", 0.9)
sp:stroke({{126.3, 453.2}, {126.3, 449}, {126.3, 443.5}}, {pressure={0.9, 0.0}, ramps={0, 0.95}})

--@ chunk 20 · clock 81438.1611328125
dry()
-- a thin young moon high on the right, its lit limb toward the sun that has set (lower left), and the evening star
local cres = ellipse(812, 148, 7.2, 7.2) - ellipse(814.6, 146.0, 6.9, 7.0)
work(cres:soften(0.3), {hand="detail", tool="round 0.9", length={1, 3}, coverage=3.5, angle=2.3, edge="found", color="#ebe3c9", medium=0.2})
local star = brush{kind="round", width=1.3}
star:load("#eee6cf", 0.8)
star:touch(652, 352, {pressure=0.75, twist=0.4})
-- crows: two perched in the oak's upper limbs, three going over toward the far wood
local cr = brush{kind="round", width=1.6, point=0.6}
local function perched(x, y, s, face)
  -- a crow hunched on the limb: a heavy body, the head sunk in, the tail hanging down past the limb
  local body = brush{kind="round", width=2.6 * s, point=0.3}
  body:load("#1c1a1b", 0.95)
  body:stroke({{x - 1.8 * s * face, y - 1.0 * s}, {x, y - 1.9 * s}, {x + 1.6 * s * face, y - 2.1 * s}}, {pressure={0.8, 0.9}, ramps={0.1, 0.15}})
  local hd = brush{kind="round", width=1.7 * s, point=0.4}
  hd:load("#1a1819", 0.95)
  hd:stroke({{x + 1.6 * s * face, y - 2.6 * s}, {x + 2.4 * s * face, y - 3.1 * s}}, {pressure={0.8, 0.7}, ramps={0, 0.2}})
  local bill = brush{kind="round", width=0.7 * s, point=1}
  bill:load("#1a1819", 0.9)
  bill:stroke({{x + 2.8 * s * face, y - 3.0 * s}, {x + 3.9 * s * face, y - 2.7 * s}}, {pressure={0.8, 0.1}, ramps={0, 0.7}})
  local tl = brush{kind="round", width=1.3 * s, point=0.8}
  tl:load("#1e1c1d", 0.9)
  tl:stroke({{x - 1.6 * s * face, y - 1.0 * s}, {x - 2.6 * s * face, y + 0.4 * s}, {x - 3.1 * s * face, y + 1.8 * s}}, {pressure={0.8, 0.2}, ramps={0, 0.6}})
end
perched(368.2, 256.0, 1.6, 1)
perched(470.6, 322.4, 1.4, -1)
local wing = brush{kind="rigger", width=0.8, point=1}
local function flying(x, y, s, up)
  wing:reload("#232124", 0.85)
  wing:stroke({{x - 4 * s, y - up * s}, {x - 1.8 * s, y - 0.8 * s}, {x, y}}, {pressure={0.15, 0.8}, ramps={0.3, 0.1}})
  wing:stroke({{x, y}, {x + 1.8 * s, y - 0.9 * s}, {x + 4.2 * s, y - (up - 0.4) * s}}, {pressure={0.8, 0.15}, ramps={0.1, 0.4}})
  wing:stroke({{x - 0.5 * s, y + 0.1 * s}, {x + 0.6 * s, y + 0.2 * s}}, {pressure={0.9, 0.9}, ramps={0, 0}})
end
flying(598, 332, 1.3, 1.6)
flying(626, 318, 1.0, 0.4)
flying(668, 340, 0.8, 1.2)

--@ chunk 21 · clock 85419.57836914063
-- the near foreground: denser stubble and dry stalks along the bottom, thicker in the corners
local g = brush{kind="rigger", width=0.8, point=1}
local cols = {"#6d5d43", "#8a7654", "#4d4337", "#9c8660", "#3f372f", "#7c6c50"}
local n = 0
local xs = uneven(46, 0, 1000, 0.7, 0.6, 211)
for i, x in ipairs(xs) do
  local corner = math.max(1 - smoothstep(0, 340, x), smoothstep(780, 1000, x))
  if rand() < 0.35 + 0.65 * corner then
    local y = 714 - rand(0, 14 + 22 * corner)
    local s = 1.0 + 0.8 * corner * rand(0.5, 1.2)
    local nb = math.floor((6 + 16 * corner) * rand(0.6, 1.2))
    local lean = randn(0.2, 0.25)
    for k = 1, nb do
      if k % 3 == 1 then g:reload(cols[1 + (i * 3 + k) % #cols], 0.75) end
      local bx = x + randn(0, 3 * s)
      local h = (6 + rand(0, 12)) * s
      local a = -1.57 + lean + randn(0, 0.3)
      local bend = randn(0.05, 0.3)
      g:stroke({{bx, y + rand(0, 2)}, {bx + math.cos(a) * h * 0.5, y + math.sin(a) * h * 0.5},
        {bx + math.cos(a + bend) * h, y + math.sin(a + bend) * h}}, {pressure={clamp(0.45 + 0.25 * s, 0.3, 0.95), 0}, ramps={0.05, 0.7}})
      n = n + 1
    end
  end
end
-- a few seed heads and bent-over stalks, and two bramble arcs in the left corner
local br = brush{kind="rigger", width=1.0, point=0.8}
br:load("#3c322b", 0.85)
br:stroke({{36, 712}, {44, 690}, {60, 680}, {76, 684}, {84, 694}}, {pressure={0.7, 0.1}, ramps={0.05, 0.6}, shake=0.8})
br:stroke({{58, 713}, {70, 696}, {90, 690}, {102, 697}}, {pressure={0.6, 0.05}, ramps={0.05, 0.6}, shake=0.8})
br:reload("#4a3d33", 0.8)
br:stroke({{930, 713}, {938, 694}, {952, 686}, {968, 690}}, {pressure={0.6, 0.05}, ramps={0.05, 0.6}, shake=0.8})
print(n, "stalks")

--@ chunk 22 · clock 85419.57836914063
dry()
-- the drift round the oak's foot carried on to the right, where it turns from the glow and goes cool;
-- it climbs the bole unevenly, so the trunk goes into the snow, not onto it
local base = sample(372, 560, 3)
local b = brush{kind="filbert", width=6, stiffness=0.35}
for k = 0, 5 do
  local y = 557 - k * 1.3 + rand(-0.5, 0.5)
  b:reload(shift(base, 0.012 - 0.006 * k, 0.001, -0.004 * k), 0.95)
  b:stroke({{312 + rand(-4, 4) + 2 * k, y - 1.5 + 0.3 * k}, {332, y - 2.2 - 0.4 * k}, {348 + rand(-2, 2), y - 1 - 0.3 * k}, {368 - 3 * k, y + 1.5}},
    {pressure={0.65, 0.3}, ramps={0.1, 0.55}, shake=0.8})
end
local sm = brush{kind="filbert", width=3, stiffness=0.35}
sm:load(shift(base, -0.035, 0.002, -0.018), 0.8)
sm:stroke({{351, 555.5}, {358, 556.8}, {368, 559.5}}, {pressure={0.5, 0.15}, ramps={0.1, 0.6}, shake=0.8})
sm:stroke({{338, 552.5}, {346, 553.2}, {352, 555}}, {pressure={0.45, 0.15}, ramps={0.1, 0.6}, shake=0.8})
-- tiny bare trees rising out of a few far copses under the glow
local tb = brush{kind="rigger", width=0.55, point=1}
for i, x in ipairs({22, 48, 60, 168, 214, 236, 292, 402, 470}) do
  tb:reload(mix("#948d9a", "#b2a9b0", x / 520), 0.7)
  local h = rand(4, 9)
  tb:stroke({{x, 466}, {x + rand(-0.4, 0.4), 466 - h * 0.6}, {x + rand(-0.8, 0.8), 466 - h}}, {pressure={0.6, 0.0}, ramps={0, 0.8}})
end

--@ chunk 23 · clock 89945.97534179688
-- a faint dark veil growing toward the edges and corners (as he advised Carus), the glow left clear
local edges = mask(function(x, y)
  local dx = math.abs(x - 470) / 530
  local dy = math.abs(y - 400) / 330
  local r = math.sqrt(dx * dx * 0.8 + dy * dy)
  return smoothstep(0.75, 1.35, r)
end)
glaze(edges, {color="#3a3640", coats=0.14, pigment="semi"})
wait(24*60)
varnish{color="#e6d3a4", coats=0.35, vary=0.12}
cracks{dirt=0.4, vary=1, veil=0.5}
relief()
