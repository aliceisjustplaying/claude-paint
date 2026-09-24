-- easel session "lab1_rock_A": a painting replayed chunk by chunk.
--   easel run paintings/lua/lab1_rock_A.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
-- Round 6 lab, subject "rock": one granite boulder sitting on a heath in afternoon daylight,
-- with its cast shadow. Shared setup for rock_A (old way) and rock_B (new way): canvas,
-- palette, sky and ground colors, the rock drawn and inferred, its light and shadow.
canvas{style="friedrich", aspect=3/2, size=300, seed=71}
HZ = 150
-- the sun from the upper left, a little in front: an afternoon light, the shadow falling right
SUN = {from={-1, -0.55}, front=0.35, ambient=0.25, bounce=0.3, bounce_from={0.2, 1, 0.3}}
sky = function(x, y) return gradient({{0, "#8395ad"}, {0.7, "#b7bfc2"}, {1, "#cfccbd"}}, y / HZ) end
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}
-- the heath: far and pale at the horizon, near and darker, warm, with a slow patchiness
gn = noise{seed=72, octaves=4, period=140}
groundcol = function(x, y)
  local t = smoothstep(HZ, H, y)
  local c = mix(mix("#9c9a82", "#7d7a58", smoothstep(0, 0.35, t)), "#5f5a3c", smoothstep(0.35, 1, t))
  return shift(c, 0.03 * gn(x, y), 0.004 * gn(x + 90, y), 0.01 * gn(x, y + 70))
end
land = below(function(x) return HZ + 2 * math.sin(x / 60) end)
-- the boulder, drawn lopsided: a high shoulder left, a broken back falling to the right, a flat
-- foot sunk in the heath; one crack running off the shoulder at an angle
BOULDER = {{258,520,"c"},{262,470},{286,402,"c"},{334,338},{398,300,"c"},{470,282},{528,296,"c"},
           {586,318},{640,352,"c"},{700,392},{742,440,"c"},{758,488},{752,528,"c"},{640,540},{470,544},{350,536}}
CRACK = {{470,286},{492,340},{506,392},{500,450}}
boulder = outline{pts=BOULDER, char="broken", seed=73}
rk = rock{outline=boulder, cracks={CRACK}, kind="granite", sun=SUN, seed=74}
print(rk)
GRANITE = {core="#34322f", shadow="#57534d", half="#86807a", light="#aea79a", top="#cdc6b6", bounce="#7a6c5c", crevice="#2e2c2f"}
SHADOWCOL = "#3d3a30"

--@ chunk 2 · clock 0

-- A chunk 2: the sky thin, blended; the heath as tone (sketchbook 1, 2)
skym = above(function(x) return HZ + 6 end)
work(skym, {hand="broad", color=sky, angle=0, coverage=4.2, medium=0.3, pal=skypal})
blend(skym, {angle=0})
work(land, {hand="body", color=groundcol, angle=0.03, length={14, 40}, coverage=3.4, medium=0.18})

--@ chunk 3 · clock 0

-- A chunk 3: a day later, the second sky layer stippled (kept off the land)
wait(24*60)
skyonly = above(function(x) return HZ - 2 end)
stipple(skyonly, {width=2.6, color=sky, coverage=2.6, pressure={0.4, 0.8}, dips={18, 0.35, 0.7}, medium=0.55, clip=skyonly})

--@ chunk 4 · clock 1440

-- A chunk 4: dry() before the rock goes over the heath; then the rock as in rocks_in (sketchbook 6):
-- mass thin down the planes, lit planes stiffer, blend the shadow only, seams, arris lights,
-- then seat it with the cast and contact glazes
dry()
local r, P = rk, GRANITE
local m, v, core, refl = r:mask(), r:value(), r:core(), r:reflected()
local lo, hi = r:levels(0.03, 0.97)
local mot = noise{seed=1, octaves=3, period=r.r * 0.5}
local function col(x, y)
  local t = smoothstep(lo, hi, v:at(x, y))
  local c = gradient({{0, P.core}, {0.3, P.shadow}, {0.55, P.half}, {0.85, P.light}, {1, P.top}}, t)
  c = mix(c, P.bounce, 0.55 * refl:at(x, y))
  c = mix(c, P.core, 0.45 * core:at(x, y))
  return shift(c, 0.035 * mot(x, y), 0.006 * mot(x + 50, y), 0.008 * mot(x, y + 50))
end
work(m, {hand="body", tool="filbert 4", color=col, angle=r:field("plane"), length={6, 16}, coverage=3.2, medium=0.25, clip=m})
local lit = r:lit(0.2)
work(lit, {hand="body", tool="filbert 3", color=col, angle=r:field("plane"), length={5, 14}, coverage=2.4, medium=0.12, clip=m, hug=false})
blend(r:shadow(0.3), {angle=r:field("plane"), coverage=1.2})
local rb = brush("round", math.max(0.9, r.r * 0.018))
for i, sm in ipairs(r.seams) do
  if i % 3 == 1 then rb:reload(P.crevice, 0.8) end
  rb:stroke(sm.pts, {pressure={sm.grown and 0.55 or 0.8, 0.2}, ramps={0.15, 0.45}, shake=0.5, clip=m})
end
local ar = r:arrises() * r:lit(0.1)
work(ar, {hand="detail", tool="round 1.4", color=P.top, angle=r:field("crack"), length={3, 8}, coverage=0.6, clip=m, hug=false, medium=0.1})
print(#r.seams, clock())

--@ chunk 5 · clock 12827.69921875

-- A chunk 5: seat it (dry first; glaze waits anyway): the cast shadow and the contact seam
dry()
glaze(rk:cast():blur(1.2), {color=SHADOWCOL, coats=0.3})
glaze(rk:contact(), {color="#2c2a2a", coats=0.35})

--@ chunk 6 · clock 31933.685546875

-- A chunk 6 (dark, clustered: the first try was a pale even bristle): sink the foot in grass (sketchbook 6, "a straight base line makes a rock
-- architecture"; sketchbook 8, curving rigger blades set down lightly and lifted off), on dry paint
dry()
local lean = noise{seed=75, period=120}
local cols = {"#46442b", "#5a5634", "#3a3a26", "#686240", "#4e4b2f", "#7d7550"}
local patch = noise{seed=76, period=70}
local g = brush("rigger", 0.8)
local n = 0
local foot = rk.foot or {}
for i = 1, 1400 do
  local x, y
  if i <= 700 then x = rand(240, 790); y = 522 + rand(-8, 26) + 0.02 * (x - 500)
  else x = rand(0, 1000); y = rand(560, 667) end
  if patch:at01(x, y) < 0.42 and i > 700 then goto skip end
  local depth = smoothstep(HZ, H, y)
  local h = (8 + 34 * depth) * rand(0.5, 1.6) * (i <= 700 and 0.7 or 1)
  local a = -1.5708 + 0.35 * lean(x, y) + randn(0, 0.22)
  local curl = randn(0, 0.25)
  local mx, my = x + math.cos(a) * h * 0.5, y + math.sin(a) * h * 0.5
  local tx, ty = x + math.cos(a + curl) * h, y + math.sin(a + curl) * h
  if i % 7 == 1 then g:reload(cols[(i // 7) % 6 + 1], 0.7) end
  g:stroke({{x, y}, {mx, my}, {tx, ty}}, {pressure={0.35 + 0.5 * depth, 0}, ramps={0.05, 0.75}})
  n = n + 1
  ::skip::
end
print("blades", n)

--@ chunk 7 · clock 31933.685546875
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
