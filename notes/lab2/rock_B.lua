-- easel session "rock_B": a painting replayed chunk by chunk.
--   easel run notes/lab2/rock_B.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
-- Round 6 lab 2, subject "rock": a weathered granite erratic in a summer meadow, afternoon light
-- from the left and a little in front, its shadow falling right. Shared setup for rock_B (round 1's
-- rock B recipe) and rock_H (hierarchical detail): canvas, palette, sky and meadow colors, the
-- rock drawn and inferred.
canvas{style="friedrich", aspect=3/2, size=300, seed=81}
HZ = 205
SUN = {from={-1, -0.7}, front=0.35, ambient=0.25, bounce=0.3, bounce_from={0.2, 1, 0.3}}
sky = function(x, y) return gradient({{0, "#7f95b3"}, {0.65, "#b4bfc6"}, {1, "#d8d2bc"}}, y / HZ) end
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}
-- the meadow: pale and blue-gray at the horizon, greener in the middle, darker and cooler near,
-- with a slow patchiness
gn = noise{seed=82, octaves=4, period=150}
groundcol = function(x, y)
  local t = smoothstep(HZ, H, y)
  local c = mix(mix("#98a088", "#76804f", smoothstep(0, 0.35, t)), "#56613a", smoothstep(0.35, 1, t))
  return shift(c, 0.03 * gn(x, y), -0.004 * gn(x + 90, y), 0.01 * gn(x, y + 70))
end
land = below(function(x) return HZ + 1.5 * math.sin(x / 70) end)
-- the erratic: a rounded crown over a broad weathered face turned to the sun, the foot tucked under
-- on the left, and on the right a fracture face (in shadow) whose upper edge falls in a straight
-- run to a blunt corner, with one crack down it
ERRATIC = {{300,552,"c"},{284,522},{280,482,"c"},{294,428},{316,380},{352,334,"c"},{404,302,"c"},{468,282},
           {528,288},{590,302,"c"},{648,336},{704,370,"c"},{726,408},{736,450,"c"},{726,478,"c"},{738,512},
           {742,550,"c"},{640,558},{500,561},{380,558}}
-- the arris between the lit face and the fracture face, and the brow between the crown and the face
ARRIS = {{590,302},{572,370},{566,440},{570,510},{576,556}}
BROW = {{352,336},{420,322},{500,318},{560,314},{590,304}}
CRACK = {{648,340},{658,400},{650,462},{660,522}}
erratic = outline{pts=ERRATIC, char="broken", seed=83}
rk = rock{outline=erratic, planes={ARRIS, BROW}, cracks={CRACK}, kind="granite", sun=SUN, seed=84}
print(rk)

--@ chunk 2 · clock 0
-- B chunk 2 (round 1's rock B recipe, adapted to this rock): one session, alla prima. The DARK family is
-- the rock's shadow side + its contact + its cast shadow (one connected shape across the rock/ground
-- boundary); the LIGHT family is the lit face + the sunlit meadow. Sky first (two thin layers wet into
-- wet, one blend), then the sunlit meadow with the rock and its shadow reserved, then the dark family
-- edge to edge with it.
ROCKM = rk:mask()
local c1, c7 = rk:cast():blur(1.2), rk:cast():blur(7)
foot = rk.base
-- the cast shadow: crisp at the foot, softening as it runs out to the right; none under the lit
-- front (the terminator here is at x ~575, so it fades in over the 140 units before it)
CASTM = mask(function(x, y) local t = smoothstep(740, 1000, x) return ((1 - t) * c1:at(x, y) + t * c7:at(x, y)) * smoothstep(435, 575, x) end) - ROCKM
SHADEM = rk:shadow(0.3) * ROCKM
NY = mask(function(x, y) if ROCKM:at(x, y) < 0.5 then return 0.5 end local s = rk:sample(x, y) if not s then return 0.5 end return clamp((s.n[2] + 1) / 2, 0, 1) end):blur(3)
LITB = rk:lit(0.2):blur(10)
skym = above(function(x) return HZ + 6 end)
work(skym, {hand="broad", color=sky, angle=0, coverage=5, medium=0.22, load=1, pal=skypal, clip=skym})
work(skym, {hand="broad", color=sky, angle=0.01, coverage=3.5, medium=0.25, load=0.9, pal=skypal, clip=skym})
blend(skym, {angle=0, coverage=1.5, clip=skym})
local lit = land - ROCKM:shrink(1.5) - CASTM:shrink(1.5)
work(lit, {hand="body", color=groundcol, angle=0.03, length={14, 40}, coverage=3.4, medium=0.18, clip=lit})
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
-- B chunk 3: next day, the shadow family SETTING. The lit face in few big strokes of stiffer paint,
-- down each plane, its values kept inside the light family (no chalk); at the terminator the strokes
-- thin out into the setting shadow (hug=false).
wait(1800)
print("shadow:", drying(650, 450), drying(620, 380), "ground:", drying(200, 560))
local v = rk:value()
local lo, hi = rk:levels(0.03, 0.97)
local mot = noise{seed=87, octaves=3, period=60}
LITFN = function(x, y)
  local t = smoothstep(lo, hi, v:at(x, y))
  local c = gradient({{0, "#6a645a"}, {0.45, "#81796c"}, {0.75, "#978e7e"}, {1, "#ada28c"}}, t)
  return shift(c, 0.03 * mot(x, y), 0.004 * mot(x + 40, y), 0.008 * mot(x, y + 40))
end
LITM = (rk:lit(0.25) + rk:cracks():grow(3) * rk:lit(0.1):grow(8)) * ROCKM
work(LITM, {hand="body", tool="filbert 5", color=LITFN, angle=rk:field("plane"), length={10, 24}, coverage=3.6, medium=0.15, load=0.85, hug=false, clip=ROCKM})
print(LITM:area())

--@ chunk 4 · clock 1800
-- B chunk 4: next day, the lit face SETTING. Seat the foot: the meadow grows up over the base in
-- irregular clumps of its OWN colors (short scrubbed strokes rising from below the base into the
-- rock's setting foot), so no line runs along the bottom.
wait(1800)
print("lit face:", drying(420, 500), "ground:", drying(200, 560))
BASEY = {}
for x = 270, 750, 2 do local yb = nil for y = 470, 575 do if ROCKM:at(x, y) > 0.5 then yb = y end end BASEY[x] = yb end
local function base(x) x = 2 * math.floor(x / 2) return BASEY[x] end
local patch = noise{seed=88, period=34}
local b = brush("filbert", 3)
local n = 0
for i = 1, 520 do
  local x = rand(272, 750)
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
print("meadow strokes", n)

--@ chunk 5 · clock 3600
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
