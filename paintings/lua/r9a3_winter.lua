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
  species="oak", season="winter", sun=WORLD, seed=84, detail=0.45, girth=0.12, voids=1.6}
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
work(headm, {hand="detail", tool="round 1.2", coverage=3, color="#3a2f27", edge="firm", length={1, 3}})
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
