-- easel session "rock_H": a painting replayed chunk by chunk.
--   easel run notes/lab2/rock_H.lua [--width 3200]
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
-- H chunk 2 = B chunk 2 word for word (the grounding: sky, the sunlit meadow reserved around the
-- rock and its shadow, then the dark family, shadow side + foot + cast, edge to edge with it).
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
-- H chunk 3: next day, the shadow family SETTING. The lit side at the MIDDLE SCALE: not one light
-- over everything, but its three planes, each one value and temperature, each in its own stroke
-- direction and size, the terminator lost below the brow, then a few large weathered patches laid
-- wet into them.
--   crown top (plane 4, faces the sky): cool, a step below the shoulder, strokes across the form
--   shoulder (plane 28, faces the sun): the lightest, warm, strokes down the plane
--   lower face (plane 29, turned down to the meadow): a full step darker, warmer, a little green
--     from the grass it faces, darkening toward the foot; big quiet strokes
wait(1800)
print("shadow:", drying(650, 450), "ground:", drying(200, 560))
local v = rk:value()
local lo, hi = rk:levels(0.03, 0.97)
local slow = noise{seed=91, octaves=2, period=110}
PL = function(x, y) local s = rk:sample(x, y) return s and s.plane or 0, s end
-- (the light kept out of the solid shadow: a thin lit arris the tool finds inside it, painted in the
-- light, read as a pale gash)
LITALL = (rk:lit(0.25) + rk:cracks():grow(3) * rk:lit(0.1):grow(8)) * ROCKM * (-SHADEM:shrink(3))
-- the plane groups, their borders pushed around by a noise (up to ~16 units) and softened, so two
-- planes meet along an irregular, mostly lost border, not a ruled seam
local dx, dy = noise{seed=94, octaves=3, period=45}, noise{seed=95, octaves=3, period=45}
local function group(ids) return mask(function(x, y)
  if ROCKM:at(x, y) < 0.5 then return 0 end
  local id = PL(x + 16 * dx(x, y), y + 16 * dy(x, y))
  for _, i in ipairs(ids) do if id == i then return 1 end end
  return 0 end):blur(5) end
CROWN = group({4}) * LITALL
SHOULDER = group({28}) * LITALL
RIM = group({37}) * LITALL
LOWER = LITALL - CROWN - SHOULDER - RIM
-- within a plane the value moves only a little (40% pixel light, 60% the plane's own), so each plane
-- reads as one plane; a slow noise gives it a large irregular variation, not a texture
local function inplane(x, y, a, b)
  local t = smoothstep(lo, hi, v:at(x, y))
  return mix(a, b, clamp(0.5 + 0.4 * (t - 0.6) + 0.25 * slow(x, y), 0, 1))
end
CROWNFN = function(x, y) return inplane(x, y, "#a3a39c", "#bdbbb0") end
SHOULDERFN = function(x, y) return inplane(x, y, "#b4a58c", "#cbb99a") end
-- where the lower face turns into the shadow below the brow, its value comes down toward the
-- shadow's over the last ~30 units, so that stretch of the terminator is lost by value (above the
-- brow the shoulder meets the shadow crisply: the one high-contrast turn)
TX = {}
for y = 300, 560, 2 do for x = 480, 700 do if SHADEM:at(x, y) > 0.5 then TX[y] = x break end end end
function tx(y) y = 2 * math.floor(y / 2) return TX[y] end
local function turn(x, y) local t = tx(y) if not t then return 0 end return (1 - smoothstep(0, 32, t - x)) * smoothstep(410, 450, y) end
LOWERFN = function(x, y)
  local c = inplane(x, y, "#85796a", "#9a8c76")
  c = mix(c, "#7a7a5c", 0.35 * smoothstep(foot - 90, foot - 10, y))      -- the meadow's green bounce
  return mix(c, "#5f5950", 0.7 * turn(x, y))
end
work(SHOULDER, {hand="body", tool="filbert 6", color=SHOULDERFN, angle=rk:field("plane"), length={14, 30}, coverage=3.4, medium=0.15, load=0.9, hug=false, clip=ROCKM})
work(CROWN, {hand="body", tool="filbert 6", color=CROWNFN, angle=rk:field("across"), length={16, 34}, coverage=3.2, medium=0.15, load=0.9, hug=false, clip=ROCKM})
work(RIM, {hand="body", tool="filbert 5", color=function(x, y) return inplane(x, y, "#7f7b72", "#8e897d") end, angle=rk:field("plane"), length={12, 26}, coverage=3.0, medium=0.16, load=0.9, hug=false, clip=ROCKM})
work(LOWER, {hand="body", tool="filbert 8", color=LOWERFN, angle=rk:field("fall"), length={18, 44}, coverage=3.0, medium=0.16, load=0.9, hug=false, clip=ROCKM})
-- the lost stretch: a halftone band laid along the terminator, straddling it, while both sides are
-- wet enough to take it (laid a day later it broke up into a pale lace); it also covers the bare
-- sliver that the light and shadow masks leave between them
LOSTM = mask(function(x, y)
  local t = tx(y) if not t then return 0 end
  return (1 - smoothstep(5, 12, math.abs(x - t))) * smoothstep(420, 450, y) end):blur(2) * ROCKM
work(LOSTM, {hand="body", tool="filbert 4", color=function(x, y) return mix(LOWERFN(x, y), SHADOWFN(x, y), 0.6) end, angle=rk:field("fall"), length={10, 24}, coverage=2.6, medium=0.2, load=0.8, hug=false, clip=ROCKM})
-- weathered patches: three large, placed by eye, irregular (a noise pushes their borders), wet into
-- wet so they sit in the plane without rims: a gray-green lichen bloom on the crown, one rust-brown
-- stain low on the lower face, and a darker weather streak running down from the brow. The shoulder
-- keeps its light untouched.
local pn = noise{seed=92, octaves=3, period=28}
local function blob(cx, cy, rx, ry) return mask(function(x, y)
  local d = ((x - cx) / rx) ^ 2 + ((y - cy) / ry) ^ 2 + 0.55 * pn(x, y)
  return 1 - smoothstep(0.7, 1.0, d) end):blur(3) end
LICHEN = blob(530, 300, 34, 12) * CROWN:grow(4)
STAIN = blob(372, 470, 26, 48) * LOWER
STREAK = mask(function(x, y) local d = math.abs(x - (478 + 0.1 * (y - 330)) + 5 * pn(x, y)) return (1 - smoothstep(5, 15, d)) * smoothstep(322, 345, y) * (1 - smoothstep(400, 480, y)) end) * LOWER
work(LICHEN, {hand="body", tool="filbert 5", color=function(x, y) return mix(CROWNFN(x, y), "#949680", 0.5) end, angle=rk:field("across"), length={8, 18}, coverage=2.2, medium=0.2, load=0.7, hug=false, clip=ROCKM})
work(STAIN, {hand="body", tool="filbert 6", color=function(x, y) return mix(LOWERFN(x, y), "#806a54", 0.5) end, angle=rk:field("fall"), length={12, 26}, coverage=2.4, medium=0.2, load=0.7, hug=false, clip=ROCKM})
work(STREAK, {hand="body", tool="filbert 4", color=function(x, y) return mix(LOWERFN(x, y), "#6b6457", 0.65) end, angle=1.5708, length={16, 40}, coverage=2.4, medium=0.2, load=0.7, hug=false, clip=ROCKM})
print("areas crown/shoulder/lower", CROWN:area(), SHOULDER:area(), LOWER:area(), "patches", LICHEN:area(), STAIN:area(), STREAK:area())

--@ chunk 4 · clock 1800
-- H chunk 4: next day, the lit side SETTING (a fine line holds on it). A few selected particulars,
-- found only where the light turns; everything else stays lost:
--  1. the brow, where the shoulder's light turns down onto the lower face (y ~390), is carried by
--     the value step alone (drawn accents there read as a ruled stick)
--  2. one crack down the lower face from the brow: dark with a light lip on its sunny (left) side,
--     sharp at the top, thinning out to nothing half way down
wait(1800)
print("lit:", drying(420, 360), drying(420, 480), "shadow:", drying(650, 450))
local r2 = brush("round", 1.4)
-- the crack: a dark line, then its light lip just to its left, both fading downward
local CK = {{425,388},{428,410},{425,434},{429,458},{427,478}}
r2:reload("#4b443c", 0.8)
r2:stroke(CK, {pressure={0.75, 0.5, 0.25, 0}, ramps={0.05, 0.7}, shake=0.6, clip=ROCKM})
local r3 = brush("round", 1.0)
r3:reload("#c2b394", 0.8)
r3:stroke({{423.4,390},{426.4,410},{423.4,434}}, {pressure={0.6, 0.3, 0}, ramps={0.05, 0.7}, clip=ROCKM})

--@ chunk 5 · clock 3600
-- H chunk 5: next day, the lit side setting. The contact with the grass, varied along its length
-- (no fringe, no line):
--  a. under the tucked-in left foot, one small soft pocket of the dark family (the overhang's shadow)
--  b. along the lit foot, three spans treated three ways: a few grass tufts of different size lapping
--     over the foot at irregular places; low strokes of the meadow laid along the ground, riding
--     over the base by different amounts (lost); and gaps left as the plain meeting (found). (Two
--     soft "rises" of meadow mass read as pasted green cushions and were dropped.)
--  c. in the shadow, the edge lost: a few broad strokes of the rock-foot/cast mix straddling it, and
--     one dark tuft against the shadowed rock
wait(1800)
print("lit:", drying(420, 500), "ground:", drying(200, 560))
local BY = {}
for x = 270, 750, 2 do local yb for y = 470, 580 do if ROCKM:at(x, y) > 0.5 then yb = y end end BY[x] = yb end
local function base(x) x = 2 * math.floor(x / 2) return BY[x] end
-- a. the pocket under the overhang
local POCKET = mask(function(x, y) local d = ((x - 300) / 20) ^ 2 + ((y - 556) / 4) ^ 2 return 1 - smoothstep(0.5, 1, d) end):blur(2) - ROCKM:shrink(2)
work(POCKET, {hand="body", tool="filbert 4", color=function(x, y) return mix(groundcol(x, y), CASTFN(x, y), 0.5) end, angle=0.05, length={10, 22}, coverage=2.0, medium=0.2, load=0.7, hug=false})
-- b. the meadow lapping the lit foot: a few low strokes laid along the ground (not up it), each
--    riding a different height over the base (0 to 5 units), in irregular spans with gaps where the
--    rock meets the meadow plainly
local lap = {{312, 40}, {352, 30}, {440, 46}, {486, 22}}
local f4 = brush("filbert", 3)
for _, s in ipairs(lap) do
  local x0, len = s[1], s[2]
  for k = 1, 3 do
    local xa = x0 + rand(-6, 6)
    local ya = (base(xa) or 558) + rand(1, 4)
    local xb = xa + len * rand(0.6, 1.0)
    local yb = (base(xb) or 558) - rand(0, 3)
    f4:reload(shift(sample(xa, ya + 6, 3), rand(-0.03, 0), 0, 0), 0.7)
    f4:stroke({{xa, ya}, {(xa + xb) / 2, (ya + yb) / 2}, {xb, yb}}, {pressure={0.45, 0.55, 0.05}, ramps={0.3, 0.6}})
  end
end
-- tufts: blades set down at the root and lifted off, fanned, of different heights, darker than the
-- lit rock they cross (pale blades vanished on it), a few catching the sun; each tuft its own size
local g = brush("round", 2.8)
local function tuft(cx, cy, n, h, lean, sun)
  for i = 1, n do
    local fx, fy = cx + randn(0, 2 + 0.4 * n), cy + rand(0, 3)
    local a = -1.5708 + lean + randn(0, 0.38)
    local len = h * rand(0.45, 1.1)
    local bend = randn(0, 0.25)
    local tx, ty = fx + math.cos(a) * len, fy + math.sin(a) * len
    local mx, my = fx + math.cos(a - bend) * len * 0.5, fy + math.sin(a - bend) * len * 0.5
    local c = mix("#3b4526", sun and "#8a9448" or "#5c6440", sun and rand(0, 0.6) ^ 1.5 or rand(0, 0.4))
    g:reload(c, 0.7)
    g:stroke({{fx, fy}, {mx, my}, {tx, ty}}, {pressure={rand(0.7, 0.95), 0.05}, ramps={0.05, 0.8}})
  end
end
local T = {{318, 10, 26, -0.15}, {396, 16, 38, 0.1}, {412, 7, 20, 0.3}, {506, 12, 30, -0.05}}
local nb = 0
for _, t in ipairs(T) do tuft(t[1], base(t[1]) + 3, t[2], t[3], t[4], true) nb = nb + t[2] end
-- c. the shadow foot lost: broad strokes straddling the base, and one dark tuft
local f = brush("filbert", 5)
for _, x in ipairs({548, 571, 604, 652, 671, 716}) do
  local yb = base(x) or 556
  f:reload(mix(CASTFN(x, yb + 8), SHADOWFN(x, yb - 6), rand(0.3, 0.6)), 0.6)
  f:stroke({{x, yb + 7}, {x + rand(8, 18), yb - rand(2, 8)}}, {pressure={0.55, 0.2}, ramps={0.2, 0.5}})
end
g = brush("round", 1.5)
for i = 1, 8 do
  local fx, fy = 626 + randn(0, 5), (base(626) or 556) + rand(3, 7)
  local a = -1.5708 + 0.15 + randn(0, 0.4)
  local len = rand(8, 20)
  g:reload(mix("#2f3624", "#4a5134", rand()), 0.7)
  g:stroke({{fx, fy}, {fx + math.cos(a) * len * 0.5, fy + math.sin(a) * len * 0.5}, {fx + math.cos(a) * len, fy + math.sin(a) * len}}, {pressure={0.7, 0}, ramps={0.05, 0.75}})
end
print("lit tuft blades", nb, "shadow tuft blades 8, shadow strokes 6")

--@ chunk 6 · clock 5400
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
