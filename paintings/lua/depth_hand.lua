-- easel session "depth_hand": a painting replayed chunk by chunk.
--   easel run paintings/lua/depth_hand.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.4, seed=11}

--@ chunk 2 · clock 0
-- a low sun from the left, a beach falling to a calm sea
HZ = 330
w = world{horizon=HZ, eye=1.7, fov=46, sun={azimuth=-70, elevation=19}, backdrop=900,
  ground=function(X, Z) return 0.55 - 0.032 * Z + 0.05 * math.sin(X / 3 + Z / 5) end,
  water={level=0, ripple={0.02, 1.6, 0.35, 4}}}
-- stones at the water's edge (one sunk in the sea), a boulder on the beach
local function stone(X, Z, wd, ht, dp, seed)
  local s = w:spot_at(X, Z)
  local b = body.ellipsoid(s:p(0, ht * 0.35, 0), s:size(wd, ht, dp)):turn(s:p(0, 0, 0), 0.4 * seed, 0.05, -0.1)
    :rough(s:m(0.05), s:m(0.6), seed)
  local id
  w, id = w:place(s, b)
  return id
end
st1 = stone(0.25, 15.5, 0.9, 0.62, 0.8, 1)
st2 = stone(2.6, 19.5, 1.1, 0.7, 0.9, 2)
st3 = stone(-3.4, 13.0, 1.4, 0.95, 1.1, 3)
-- a man seen from behind, standing on the beach looking out: a proxy for his
-- shadow, and the outline he'll be painted in, registered at his depth
fs = w:spot_at(-0.35, 11)
w = w:proxy(fs, body.ellipsoid(fs:p(0, 0.85, 0), fs:size(0.26, 0.88, 0.18)))
local function at(r, u) return {fs.x + r * fs.s, fs.y - u * fs.s} end
local coat = {}
for _, p in ipairs{{-0.2, 0.02}, {-0.26, 0.3}, {-0.23, 0.8}, {-0.215, 1.2}, {-0.2, 1.36}, {-0.13, 1.42}, {-0.045, 1.45}, {-0.04, 1.51},
                   {0.04, 1.51}, {0.045, 1.45}, {0.13, 1.42}, {0.2, 1.36}, {0.215, 1.2}, {0.23, 0.8}, {0.26, 0.3}, {0.2, 0.02}} do
  coat[#coat + 1] = at(p[1], p[2])
end
coatm = poly(coat)
local h = at(0.01, 1.6)
headm = ellipse(h[1], h[2], 0.085 * fs.s, 0.1 * fs.s) + ellipse(h[1] + 0.01 * fs.s, h[2] - 0.075 * fs.s, 0.1 * fs.s, 0.035 * fs.s)
figm = coatm + headm
w = w:layer("figure", figm, fs)
v = w:view()
sk = w:sky{haze=2.4}
-- the shore line on the canvas, from the world
shore = {}
for x = 0, 1000, 10 do
  local y = HZ + 1
  while y < H do
    local p = w:to_ground(x, y)
    if p and not w:is_water(p[1], p[3]) then break end
    y = y + 1
  end
  shore[#shore + 1] = {x, y}
end
print(fs, "figure", fs:m(1.75), "units tall")

--@ chunk 3 · clock 0
-- the sky, thin, in long strokes, then fused
work(above(function(x) return HZ + 8 end), {hand="broad", color=sk, angle=0, coverage=4})
blend(above(function(x) return HZ + 8 end), {angle=0})

--@ chunk 4 · clock 0
-- the sea as one band (under the stones and the man: they go over it later)
seaband = below(function(x) return HZ - 1 end) * above(shore)
local function seacol(x, y)
  local t = clamp((y - HZ) / 110, 0, 1)
  return mix(shift(sk:at(x, 2 * HZ - y - 4), -0.05, 0, -0.01), "#39495a", 0.25 + 0.5 * t)
end
work(seaband, {hand="broad", color=seacol, angle=0, coverage=3.5, length={30, 90}})

--@ chunk 5 · clock 0
-- the beach: pale where the low sun rakes it, cooler near the water
beach = below(shore)
local grain = noise{seed=5, period=40}
work(beach, {hand="body", angle=0.05, length={12, 40}, coverage=3.2,
  color=function(x, y)
    local p = v:at(x, y)
    local lit = p.shade and p.shade.value or 0.5
    local c = mix("#8e8570", "#d4c7a2", clamp(lit * 1.3, 0, 1))
    return shift(mix(c, "#7d7a70", smoothstep(40, 0, y - 450)), 0.03 * grain(x, y), 0, 0)
  end})
dry()

--@ chunk 6 · clock 13833.7666015625
-- the stones from their light and shade, wherever they are seen
local f = v.form
work(v:visible("bodies"), {hand="body", tool="filbert 3", length={4, 12}, coverage=3.5, clip=true,
  angle=f:field("fall"),
  color=function(x, y) return mix("#35302c", "#a39580", clamp(f:value(x, y) * 1.2, 0, 1)) end})

--@ chunk 7 · clock 13833.7666015625
-- the man: a dark greatcoat and a cap, a warm edge on the side toward the sun
work(figm, {hand="body", tool="filbert 2.5", length={5, 14}, coverage=5, clip=true, angle=math.pi / 2,
  color="#221f1e"})
local lit = figm:rim(2.2, 1.2) * mask(function(x, y) return x < fs.x - fs:m(0.05) and 1 or 0 end)
work(lit, {hand="detail", tool="round 1.4", length={3, 8}, coverage=2.5, clip=true, angle=math.pi / 2,
  color="#7a5a40"})
dry()

--@ chunk 8 · clock 40741.9443359375
-- shadows by hand, the round-3 way: a softened ring around each stone's
-- silhouette for contact, a soft skewed band for the man's cast shadow
local stones = v:bodies_mask()
glaze((stones:grow(12) - stones):soften(6), {color="#2a2420", coats=0.9})
glaze(poly({{fs.x, fs.y - 3}, {fs.x + 420, fs.y + 14}, {fs.x + 420, fs.y + 22}, {fs.x, fs.y + 5}}):soften(4), {color="#4a4c60", coats=0.55})

--@ chunk 9 · clock 40741.9443359375
-- the same veil on the hand-made sea mask, nothing subtracted
local function veil(x, y) return mix(shift(sk:at(x, 2 * HZ - y - 4), -0.04, 0, -0.01), "#3e4e5e", 0.3 + 0.45 * clamp((y - HZ) / 110, 0, 1)) end
stipple(seaband, {width=2.6, color=veil, coverage=3, pressure={0.4, 0.8}, medium=0.55})

--@ chunk 10 · clock 40741.9443359375
-- the same glints on the hand-made sea mask, nothing subtracted
-- gathered in drifts, closer together toward the horizon
local drift = noise{seed=9, period=60, stretch={0, 5}}
local zone = seaband * mask(function(x, y) return smoothstep(HZ + 75, HZ + 6, y) * smoothstep(0.25, 0.6, drift:at01(x, y)) end)
work(zone, {hand="detail", tool="round 1", length={3, 11}, coverage=0.18, angle=0, order="scatter",
  color=function(x, y) return mix("#e6dcc0", "#aab0aa", clamp((y - HZ) / 70, 0, 1)) end})
