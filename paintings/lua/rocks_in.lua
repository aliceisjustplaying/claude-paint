-- easel session "rocks_in": rocks grown from drawn outlines (rock{}).
--   easel run paintings/lua/rocks_in.lua [--width 3200]
-- Four studies: a granite erratic on the heath; the same erratic in snow under a
-- low sun; a sandstone ledge; a scatter of stones on the shore.
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.4, seed=31}; print(H)

--@ chunk 2 · clock 0
-- how I paint a rock from rock{}: the shadow family first, thin and dark, then the
-- planes in the light in stiffer paint (each stroked down its own plane), fused,
-- then cracks, a few lights on the arrises, the foot and the shadow on the ground
function paint_rock(r, P, o)
  o = o or {}
  local m, v, core, refl = r:mask(), r:value(), r:core(), r:reflected()
  local lo, hi = r:levels(0.03, 0.97)
  local mot = noise{seed=o.seed or 1, octaves=3, period=r.r * 0.5}
  local function col(x, y)
    local t = smoothstep(lo, hi, v:at(x, y))
    local c = gradient({{0, P.core}, {0.3, P.shadow}, {0.55, P.half}, {0.85, P.light}, {1, P.top}}, t)
    c = mix(c, P.bounce, 0.55 * refl:at(x, y))
    c = mix(c, P.core, 0.45 * core:at(x, y))
    return shift(c, 0.035 * mot(x, y), 0.006 * mot(x + 50, y), 0.008 * mot(x, y + 50))
  end
  -- 1. the whole mass, thin, down the planes
  work(m, {hand="body", tool="filbert 4", color=col, angle=r:field("plane"), length={6, 16}, coverage=3.2, medium=0.25, clip=m})
  -- 2. the lit planes again, stiffer, each its own way
  local lit = r:lit(0.2)
  work(lit, {hand="body", tool="filbert 3", color=col, angle=r:field("plane"), length={5, 14}, coverage=2.4, medium=0.12, clip=m, hug=false})
  -- 3. fuse the shadow planes a little; the blender follows the planes, so the breaks stay
  blend(r:shadow(0.3), {angle=r:field("plane"), coverage=1.2})
  -- 4. cracks and joints (drawn, grown from corners, bed joints): each one stroke
  --    of a round brush along its line, heavier where it opens, lifting at the ends
  local rb = brush("round", o.crack or math.max(0.9, r.r * 0.018))
  for i, sm in ipairs(r.seams) do
    if i % 3 == 1 then rb:reload(P.crevice, 0.8) end
    rb:stroke(sm.pts, {pressure={sm.grown and 0.55 or 0.8, 0.2}, ramps={0.15, 0.45}, shake=0.5, clip=m})
  end
  -- 5. the brightest lights on the convex arrises facing the sun
  local ar = r:arrises() * r:lit(0.1)
  work(ar, {hand="detail", tool="round 1.4", color=P.top, angle=r:field("crack"), length={3, 8}, coverage=0.6, clip=m, hug=false, medium=0.1})
end

-- the ground a rock stands on: its contact seam and cast shadow, as glazes
function seat_rock(r, shadow_col, k, panel)
  local cast, touch = r:cast():blur(1.2), r:contact()
  if panel then cast, touch = cast * panel, touch * panel end
  glaze(cast, {color=shadow_col, coats=0.3 * (k or 1)})
  glaze(touch, {color="#2c2a2a", coats=0.35 * (k or 1)})
end

GRANITE = {core="#34322f", shadow="#57534d", half="#86807a", light="#aea79a", top="#cdc6b6", bounce="#7a6c5c", crevice="#2e2c2f"}

--@ chunk 3 · clock 0
-- the four grounds. A: heath under an afternoon sky. B: snow at dusk, a low sun.
-- C: a wooded slope under a sandstone ledge. D: a pebbly shore.
local g = noise{seed=5, octaves=4, period=80}
local skyA = function(x, y) return gradient({{0,"#7b8ca6"},{0.7,"#b9bfc2"},{1,"#d8cfb6"}}, y / 250) end
work(rect(0, 0, 500, 252), {hand="broad", color=skyA, angle=0, coverage=4})
work(rect(0, 246, 500, 111), {hand="body", color=function(x, y) return mix("#6c6a4a", "#4c4b36", smoothstep(250, 357, y) + 0.2 * g(x, y)) end,
  angle=0.05, length={8, 24}, coverage=3.4})
local skyB = function(x, y) return gradient({{0,"#6d7590"},{0.6,"#b3a9a8"},{1,"#e2c49a"}}, y / 262) end
work(rect(500, 0, 500, 264), {hand="broad", color=skyB, angle=0, coverage=4})
work(rect(500, 258, 500, 99), {hand="body", color=function(x, y) return mix("#c9c3bd", "#b5b6c0", smoothstep(262, 357, y) + 0.15 * g(x, y)) end,
  angle=0.02, length={14, 40}, coverage=3.4, medium=0.18})
work(rect(0, 357, 500, 357), {hand="body", color=function(x, y) return mix("#5b5d44", "#3a3d2e", smoothstep(380, 714, y) + 0.25 * g(x, y)) end,
  angle=0.3, length={8, 22}, coverage=3.4})
work(rect(500, 357, 500, 357), {hand="body", color=function(x, y) return mix("#b6ab96", "#8f8574", smoothstep(380, 714, y) + 0.25 * g(x, y)) end,
  angle=0.02, length={10, 30}, coverage=3.4})

--@ chunk 4 · clock 60
wait(24*60)
-- A: the erratic, drawn lopsided: a high shoulder on the left, a lower broken back to the right
ERRATIC = {{84,322,"c"},{92,282},{112,238,"c"},{152,206},{204,188,"c"},{252,196},{290,214,"c"},{334,226},{372,256,"c"},{398,292},{404,320,"c"},{330,330},{210,334}}
CRACK = {{206,192},{222,226},{236,250},{244,292}}
A = rock{outline=outline{pts=ERRATIC, char="broken", seed=4}, cracks={CRACK}, kind="granite", sun={-1, -0.5, 0.3}, seed=7}
print(A)
paint_rock(A, GRANITE)
seat_rock(A, "#3d3a30", 1, rect(0, 0, 500, 357))

--@ chunk 5 · clock 1500
wait(24*60)
-- B: the same erratic in snow, a low sun from the left at dusk
local E2 = {}
for i, p in ipairs(ERRATIC) do E2[i] = {p[1] + 500, p[2] + 6, p[3]} end
local C2 = {}
for i, p in ipairs(CRACK) do C2[i] = {p[1] + 500, p[2] + 6} end
-- the snow throws a lot of light back up into the rock's shadow side: a strong bounce from below
B = rock{outline=outline{pts=E2, char="broken", seed=4}, cracks={C2}, kind="granite", seed=7,
  sun={from={-1, -0.14}, front=0.3, ambient=0.25, bounce=0.55, bounce_from={0.3, 1, 0.45}}}
print(B)
paint_rock(B, {core="#2c2c34", shadow="#4c4e5a", half="#7c7a80", light="#b09a88", top="#d6b89a", bounce="#8a90a4", crevice="#26252c"})
seat_rock(B, "#6a7090", 0.9, rect(500, 0, 500, 357))

--@ chunk 6 · clock 3000
wait(24*60)
-- snow on the faces that turn up, and a cap standing over the top edge; lit warm, shaded blue
-- (softened and re-cut: the raw mask follows every grain of the surface and leaves dark specks)
local sn = B:snow{amount=0.7, depth=3, seed=3}:blur(2):map(function(v) return smoothstep(0.3, 0.6, v) end)
local bv = B:value():blur(3)
local blo, bhi = B:levels(0.03, 0.97)
-- masstone and full cover: laid thin over the dark rock, the snow let the stone speck through
work(sn, {hand="body", tool="filbert 3", length={4, 12}, coverage=5, medium=0.1, aim="masstone", angle=B:field("across"),
  color=function(x, y) return mix("#8e96ae", "#efe3cf", smoothstep(blo, bhi, bv:at(x, y) + 0.1)) end})

--@ chunk 7 · clock 4500
wait(24*60)
-- C: a sandstone ledge breaking out of the slope, two joints drawn, beds found by the tool
SANDSTONE = {core="#3a3026", shadow="#655642", half="#96806a", light="#c0a884", top="#d9c6a0", bounce="#8c6c4c", crevice="#2c261e"}
LEDGE = {{36,650,"c"},{44,580},{58,520,"c"},{92,500,"c"},{120,462,"c"},{240,452},{330,456,"c"},{352,482,"c"},{410,488,"c"},{436,540,"c"},{452,560,"c"},{462,610},{470,656,"c"},{300,664},{150,662}}
C = rock{outline=outline{pts=LEDGE, char="firm", seed=12}, cracks={{{210,455},{206,520},{214,600},{208,655}}, {{360,465},{372,540},{366,600}}},
  kind="sandstone", sun={-1, -0.5, 0.3}, seed=21}
print(C)
paint_rock(C, SANDSTONE)
seat_rock(C, "#2e3024", 1, rect(0, 357, 500, 357))

--@ chunk 8 · clock 6000
wait(24*60)
-- D: a scatter of stones on the shore, each drawn in a few points
STONES = {
  {{560,560,"c"},{572,520},{610,500,"c"},{660,506},{690,540,"c"},{684,566,"c"},{620,572}},
  {{720,600,"c"},{728,574},{756,560,"c"},{796,566},{812,596,"c"},{770,606}},
  {{830,540,"c"},{842,500},{880,484,"c"},{930,492},{952,530,"c"},{946,548,"c"},{890,552}},
  {{640,650,"c"},{650,626},{680,618,"c"},{712,628},{716,652,"c"},{680,660}},
  {{860,640,"c"},{866,616},{896,606,"c"},{930,614},{940,640,"c"},{900,648}},
  {{770,670,"c"},{778,652},{800,646},{820,656,"c"},{818,674,"c"},{790,678}},
}
local kinds = {"granite", "granite", "sandstone", "granite", "granite", "granite"}
local pals = {GRANITE, {core="#3a3533", shadow="#605854", half="#8f8580", light="#b8aea4", top="#d2c8bc", bounce="#7c6c60", crevice="#2e2a2a"}, SANDSTONE, GRANITE, GRANITE, GRANITE}
for i, pts in ipairs(STONES) do
  local s = rock{outline=outline{pts=pts, char=(i % 2 == 0) and "firm" or "broken", seed=40 + i}, kind=kinds[i], sun={-1, -0.5, 0.3}, seed=50 + i}
  paint_rock(s, pals[i], {seed=i})
  seat_rock(s, "#5f5648", 0.8, rect(500, 357, 500, 357))
end
