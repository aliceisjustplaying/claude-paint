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
  -- 4. cracks and joints, dark gray, along their lines
  local cr = r:cracks()
  work(cr, {hand="detail", tool="round 1.4", color=P.crevice, angle=r:field("crack"), length={4, 12}, coverage=1.6, clip=m, hug=false})
  -- 5. the brightest lights on the convex arrises facing the sun
  local ar = r:arrises() * r:lit(0.1)
  work(ar, {hand="detail", tool="round 1.4", color=P.top, angle=r:field("crack"), length={3, 8}, coverage=0.6, clip=m, hug=false, medium=0.1})
end

-- the ground a rock stands on: its contact seam and cast shadow, as glazes
function seat_rock(r, shadow_col, k)
  glaze(r:cast():blur(1.2), {color=shadow_col, coats=0.45 * (k or 1)})
  glaze(r:contact(), {color="#2c2a2a", coats=0.35 * (k or 1)})
end

GRANITE = {core="#34322f", shadow="#57534d", half="#86807a", light="#aea79a", top="#cdc6b6", bounce="#7a6c5c", crevice="#2e2c2f"}

--@ chunk 3 · clock 0
-- panel A: an erratic on the heath, afternoon sun from the upper left
local sky = function(x, y) return gradient({{0,"#7b8ca6"},{0.7,"#b9bfc2"},{1,"#d8cfb6"}}, y / 250) end
work(rect(0, 0, 500, 250), {hand="broad", color=sky, angle=0, coverage=4})
local g = noise{seed=5, octaves=4, period=80}
work(rect(0, 245, 500, 112), {hand="body", color=function(x, y) return mix("#6c6a4a", "#4c4b36", smoothstep(250, 357, y) + 0.2 * g(x, y)) end,
  angle=0.05, length={8, 24}, coverage=3.4})

--@ chunk 4 · clock 60
wait(24*60)
ERRATIC = {{92,318,"c"},{98,276},{124,228,"c"},{170,196},{226,182,"c"},{292,188},{340,214,"c"},{378,254},{396,300,"c"},{388,326,"c"},{300,332},{200,334}}
erratic = outline{pts=ERRATIC, char="broken", seed=4}
A = rock{outline=erratic, cracks={{{214,190},{236,222},{262,262},{270,300}}}, kind="granite", sun={-1, -0.5, 0.25}, seed=7}
print(A)
paint_rock(A, GRANITE)
seat_rock(A, "#3d3a30")
