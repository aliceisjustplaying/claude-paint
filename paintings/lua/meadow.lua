-- easel session "meadow": a painting replayed chunk by chunk.
--   easel run paintings/lua/meadow.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=1.5, seed=11}

--@ chunk 2 · clock 0

HZ = H * 0.46
w = world{horizon=HZ, eye=1.7, fov=50, sun={azimuth=-125, elevation=38},
  ground=function(X, Z) return 1.2*math.sin(X/40 + 0.8) * math.min(1, Z/60) - 0.02*Z end}
sk = w:sky{haze=2.2, uneven={0.5, 30000, 5}}
cl = w:clouds{sky=sk, cell=3,
  {kind="cumulus", x=-3000, z=9000, base=1300, width=3000, height=1500, seed=4},
  {kind="cumulus", x=2600, z=12000, base=1500, width=2200, height=1100, seed=9},
  {kind="stratus", base=3500, thick=500, cover=0.35, seed=2, wind={-0.2, 2}}}
local skym = above(function(x) return HZ + 12 end)
work(skym, {hand="broad", color=cl, angle=0, coverage=4.5, medium=0.3, pal=pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}})
blend(skym, {angle=0})

--@ chunk 3 · clock 0

wait(24*60)
air = haze{visibility=18000, height=800}
rs = w:ranges{near=6000, far=22000, count=2, seed=8, heights={180, 700}, kinds={"dome", "saddle", "plateau"}}
for i = #rs, 1, -1 do
  local l = rs[i]
  work(l:mask() * above(function(x) return HZ + 3 end), {hand="body", length={20, 60}, coverage=3.5, angle=0.05,
    color=function(x, y) return mix(mix("#3f4c44", "#56657a", i - 1), sk:airlight(x), 0.75*l:haze(air, x, y)) end})
end
local land = below(function(x) return HZ end)
work(land, {hand="body", length={25, 70}, coverage=3.5, angle=function(x, y) return 0.04*math.sin(x/90) end,
  color=function(x, y)
    local p = w:to_ground(x, y); local Z = p and p[3] or 3000
    local near = mix("#4f6a2e", "#6d8a3a", smoothstep(0, 1, (y - HZ)/200))
    return mix(near, sk:airlight(x), w:aerial(Z) * 0.8)
  end})

--@ chunk 4 · clock 1440

wait(24*60)
local s = w:spot(700, 336)
local tall = w:height(s.x, s.y, 18)
beech = tree{habit="beech", x=s.x, y=s.y, height=tall, seed=5}
leaves = beech:foliage{sun={-0.6, -0.7, 0.35}, seed=5}
-- stand-ins for its shadow: a crown and a trunk in the world
w = w:proxy(s, body.ellipsoid(s:p(0, 11, 0), s:size(7, 6, 6)))
w = w:proxy(s, body.block(s:p(0, 3, 0), s:size(0.6, 6, 0.6), s:m(0.2)))
v = w:view()
local sa = w:shadow_angle(s.x, s.y + 2) or 0
work(v:shadows() * v:land(), {hand="body", tool="filbert 3", length={6, 16}, coverage=3, angle=sa,
  color_over={shift={-0.06, -0.004, -0.012}}})

--@ chunk 5 · clock 2880

local bark = "#3e372f"
local trunk, limb, twig = brush("round", 3.5), brush("round", 1.4), brush("rigger", 0.6)
for i, l in ipairs(beech.limbs) do
  if #l.pts >= 2 then
    local b = (l.order == 0) and trunk or ((l.w[1] > 0.9) and limb or twig)
    if i % 5 == 1 or b:fullness() < 0.3 then b:reload(bark, 0.9) end
    local p = (l.order == 0) and {1.0, 0.5} or {0.85, 0.2}
    b:stroke(l.pts, {pressure=p, ramps={0.03, 0.5}})
  end
end
local turn = noise{seed=7, period=12}
local leafangle = function(x, y) return 2.2 * turn(x, y) end
local crown = leaves:mask()
work(crown, {hand="hatch", tool="round 1.6", length={3, 7}, coverage=2.6, angle=leafangle, angle_jitter=0.6,
  color=function(x, y) return mix("#27351f", "#34452a", (y - 110)/200) end})
work(leaves:lit() * crown, {hand="hatch", tool="round 1.3", length={2, 5}, coverage=2.2, angle=leafangle, angle_jitter=0.6,
  color=function(x, y) return mix("#6f8a3e", "#58722f", (y - 110)/200) end})

--@ chunk 6 · clock 2880

wait(3*60)
tufts = sward{region=below(function(x) return HZ + 40 end), horizon=HZ, near=H, height=30, flowers=0.05, seed=4,
  wind={lean=0.15, gust=0.2, period=160, seed=2}}
local g = brush("rigger", 0.7)
local greens = {"#5e7a30", "#7a8f3c", "#44592a", "#8a9a50"}
local n = 0
for i, t in ipairs(tufts) do
  if i % 4 == 1 then g:reload(greens[1 + (i // 4) % #greens], 0.7) end
  local p = clamp(0.25 + 0.5 * t.scale, 0.2, 0.9)
  for _, bl in ipairs(t.blades) do
    g:stroke(bl, {pressure={p, 0.0}, ramps={0.05, 0.7}})
    n = n + 1
  end
end
local f = brush("round", 1.2)
local k = 0
for _, t in ipairs(tufts) do
  if t.flower then
    if k % 6 == 0 then f:reload(({"#e9e2c8", "#d9b83a", "#c9d0e0"})[1 + t.flower.kind % 3], 0.8) end
    f:touch(t.flower.x, t.flower.y, {pressure=clamp(t.flower.r / 2, 0.2, 0.8)})
    k = k + 1
  end
end
print(#tufts, "tufts", n, "blades", k, "flowers")

--@ chunk 7 · clock 3060
wait(24*60); varnish(); relief()
