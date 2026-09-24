-- easel session "lab1_rock_B": a painting replayed chunk by chunk.
--   easel run paintings/lua/lab1_rock_B.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

-- Adapted for the r6-wet engine (notes/wet.md §8): chunk 3 blends the lit face into the setting shadow along the terminator.
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
-- B chunk 2: one session, alla prima. The picture planned as value families: the DARK family is
-- the rock's shadow side + its contact + its cast shadow (one connected shape across the rock/ground
-- boundary); the LIGHT family is the lit face + the sunlit heath. Sky first (two thin layers wet into
-- wet, one blend), then the sunlit heath with the whole rock and its shadow reserved, then the dark
-- family edge to edge with it (dark laid over wet light turned to lace in the foliage study).
ROCKM = rk:mask()
local c1, c7 = rk:cast():blur(1.2), rk:cast():blur(7)
local foot = rk.base
-- the cast shadow: crisp at the foot, softening as it runs out to the right; none under the lit
-- front (there a dark strip reads as an outline)
CASTM = mask(function(x, y) local t = smoothstep(700, 1000, x) return ((1 - t) * c1:at(x, y) + t * c7:at(x, y)) * smoothstep(400, 540, x) end) - ROCKM
SHADEM = rk:shadow(0.3) * ROCKM
DARKM = SHADEM + CASTM + rk:contact() * (-ROCKM)
-- the rock's surface normal (down-facing planes get the ground's bounce, sky-facing ones the sky)
NY = mask(function(x, y) if ROCKM:at(x, y) < 0.5 then return 0.5 end local s = rk:sample(x, y) return clamp((s.n[2] + 1) / 2, 0, 1) end):blur(3)
LITB = rk:lit(0.2):blur(10)
skym = above(function(x) return HZ + 6 end)
work(skym, {hand="broad", color=sky, angle=0, coverage=5, medium=0.22, load=1, pal=skypal, clip=skym})
work(skym, {hand="broad", color=sky, angle=0.01, coverage=3.5, medium=0.25, load=0.9, pal=skypal, clip=skym})
blend(skym, {angle=0, coverage=1.5, clip=skym})
local lit = land - ROCKM:shrink(1.5) - CASTM:shrink(1.5)
work(lit, {hand="body", color=groundcol, angle=0.03, length={14, 40}, coverage=3.4, medium=0.18, clip=lit})
-- the dark family: on the rock, cool where a plane turns to the sky, warm where it faces the heath,
-- darkest at the turn (core) and at the foot, where it takes the cast shadow's own value
SHADOWFN = function(x, y)
  local ny = NY:at(x, y)
  local c = mix("#5b5d62", "#51483d", smoothstep(0.4, 0.75, ny))
  c = mix(c, "#34322e", 0.6 * LITB:at(x, y))
  return mix(c, "#34332a", smoothstep(foot - 70, foot - 6, y))
end
CASTFN = function(x, y)
  local far = smoothstep(740, 1000, x)
  return shift(groundcol(x, y), -0.15 + 0.05 * far, -0.004, -0.02 + 0.008 * far)
end
work(SHADEM, {hand="body", tool="filbert 5", color=SHADOWFN, angle=rk:field("plane"), length={8, 20}, coverage=3.8, medium=0.25, load=0.85, clip=ROCKM})
work(CASTM, {hand="body", tool="filbert 5", color=CASTFN, angle=0.04, length={14, 36}, coverage=4.2, medium=0.25, load=0.9, hug=false, clip=land - ROCKM:shrink(3)})
print(SHADEM:area(), CASTM:area(), foot)

--@ chunk 3 · clock 0
-- B chunk 3: next day, the shadow family SETTING (open, a light laid into it plows; tacky, it sits
-- as a flat slab). The lit face in few big strokes of stiffer paint, down each plane, its values
-- kept inside the light family (no chalk); at the terminator the strokes thin out into the setting
-- shadow (hug=false), so the form turns instead of splitting along a seam.
wait(1800)
print("shadow:", drying(600, 400), drying(560, 330), "ground:", drying(300, 560))
local v = rk:value()
local lo, hi = rk:levels(0.03, 0.97)
local mot = noise{seed=77, octaves=3, period=60}
LITFN = function(x, y)
  local t = smoothstep(lo, hi, v:at(x, y))
  local c = gradient({{0, "#6a645a"}, {0.45, "#81796c"}, {0.75, "#978e7e"}, {1, "#ada28c"}}, t)
  return shift(c, 0.03 * mot(x, y), 0.004 * mot(x + 40, y), 0.008 * mot(x, y + 40))
end
-- the crack groove fell in the shadow family (a wide dark band): the light goes over it; the crack
-- is restated later as one thin stroke
LITM = (rk:lit(0.25) + rk:cracks():grow(3) * rk:lit(0.1):grow(8)) * ROCKM
work(LITM, {hand="body", tool="filbert 5", color=LITFN, angle=rk:field("plane"), length={10, 24}, coverage=3.6, medium=0.15, load=0.85, hug=false, clip=ROCKM})
-- (r6-wet) the stroke ends at the terminator no longer sink into the setting shadow by
-- themselves: lose them deliberately, blending along the planes across the turn
local TERM = ((rk:lit(0.1) - rk:lit(0.4)) * ROCKM):blur(3)
blend(TERM, {angle=rk:field("plane"), coverage=1.8, length={10, 30}, hug=false, clip=ROCKM})
print(LITM:area())

--@ chunk 4 · clock 1800
-- B chunk 4: next day, the lit face SETTING, ground and shadow tacky. Seat the foot: the heath grows
-- up over the base in irregular clumps of its OWN colors (short scrubbed strokes rising from below
-- the base into the rock's setting foot), so no line
-- runs along the bottom (no pale strip, no dark outline: the two old failures). A first try with
-- ~600 rigger blades read as a beard of hair on the lit side and straw on the dark side;
-- dark touches along the undercut read as a row of beads.
wait(1800)
print("lit face:", drying(320, 500), "ground:", drying(300, 560))
BASEY = {}
for x = 250, 760, 2 do local yb = nil for y = 460, 560 do if ROCKM:at(x, y) > 0.5 then yb = y end end BASEY[x] = yb end
local function base(x) x = 2 * math.floor(x / 2) return BASEY[x] end
local patch = noise{seed=78, period=34}
local b = brush("filbert", 3)
local n = 0
for i = 1, 520 do
  local x = rand(252, 766)
  local yb = base(x)
  local p = patch:at01(x, 0)
  if yb and p > 0.35 then
    local y0 = yb + rand(2, 14)
    local reach = rand(4, 8 + 16 * p)
    local a = -1.5708 + randn(0, 0.25)
    local inshadow = CASTM:at(x, yb + 6) > 0.5 or SHADEM:at(x, yb - 6) > 0.5
    local c = inshadow and mix(CASTFN(x, yb + 8), SHADOWFN(x, yb - 4), 0.4) or groundcol(x, yb + 12)
    if not inshadow and rand() < 0.25 then c = shift(c, -0.06, 0, -0.01) end
    if inshadow and rand() < 0.5 then goto skip end
    b:reload(c, 0.55)
    b:stroke({{x, y0}, {x + math.cos(a) * reach * 0.5, y0 + math.sin(a) * reach * 0.5}, {x + math.cos(a) * reach, y0 + math.sin(a) * reach}},
      {pressure={0.7, 0.05}, ramps={0.05, 0.7}})
    n = n + 1
  end
  ::skip::
end
print("heath strokes", n)

--@ chunk 5 · clock 3600
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
