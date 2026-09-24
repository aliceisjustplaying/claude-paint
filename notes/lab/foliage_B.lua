-- easel session "lab1_foliage_B": a painting replayed chunk by chunk.
--   easel run paintings/lua/lab1_foliage_B.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
-- Round 6 lab, subject "foliage": one broadleaf crown in summer leaf against the sky,
-- trunk mostly hidden. Shared setup for foliage_A (old way) and foliage_B (new way):
-- canvas, palette, world and sun, sky colors, the crown and trunk drawn, the tree grown.
canvas{style="friedrich", palette="friedrich_1820_greens", aspect=4/3, size=320, seed=61}
HZ = 688
WORLD = world{horizon=HZ, sun={azimuth=-120, elevation=42}}
SUN = {-0.6, -0.6, 0.45}
-- a summer afternoon sky: clear blue-gray above, pale and warm at the horizon
sky = function(x, y)
  return gradient({{0, "#6d86a8"}, {0.45, "#95a8bc"}, {0.85, "#c6cbc6"}, {1, "#d6d3c3"}}, y / HZ)
end
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}
-- a far strip of land under the tree, hazy
land = below(function(x) return HZ + 3 * math.sin(x / 70) end)
landcol = function(x, y) return mix("#8d9278", "#5e6545", smoothstep(HZ, H, y)) end
-- the crown: broad, lopsided (heavier and lower on the right), lobed, a notch top left
CROWN = {{166,452},{132,392},{146,318},{196,268},{236,236},{246,196},{296,160},{356,150},{400,118},
         {462,96},{516,112},{540,146,"c"},{582,116},{646,106},{714,136},{744,184},{798,210},{846,262},
         {858,326},{830,372,"c"},{870,418},{884,488},{850,540},{790,556},{742,588},{672,590},{620,566,"c"},
         {560,590},{470,600},{420,578},{360,598},{290,590},{240,552},{212,508,"c"}}
TRUNK = {{508,752},{503,700},{497,640},{490,590}}
crown = outline{pts=CROWN, char="soft", seed=62}
tree = tree_in{crown=crown, trunk=TRUNK, species="oak", season="summer", sun=WORLD, seed=63}
print(tree)
-- leaf colors, deep shade to top light
LEAF = {deep="#212a1c", body="#34402a", sunny="#5c6a3a", shade="#263020", skyshade="#4a5446",
        mid="#46522e", lit="#6f7c45", top="#98a060"}
WOOD = {dark="#3b342c", light="#7d7566"}

--@ chunk 2 · clock 0

-- B chunk 2: the crown planned as masses first (one silhouette with only its big holes, and
-- the light grouped into families), then the sky alla prima AROUND it: the crown is reserved
-- on the warm ground, the sky laid up to its edge (dark over wet sky turns to a pale lace); the trunk reserved too. Two thin broad layers
-- wet into wet and one light blend (quiet surface: no stipple); the far land as tone.
local lv = tree:leaves()
-- the silhouette keeps the clumps' lobes (blur 2.5); inside, one solid mass (blur 8, shrunk)
local edge = lv:blur(2.5):map(function(v) return smoothstep(0.3, 0.55, v) end)
local core = lv:blur(8):map(function(v) return smoothstep(0.3, 0.55, v) end):shrink(14)
-- a few unequal sky holes, drawn where the limb masses part and a limb crosses, reserved now
-- (sky laid into the dark later came out as white cotton puffs at every stage)
HOLEPTS = {{425,318,20,12},{690,293,16,10},{760,445,22,11},{330,520,18,10},{560,393,11,7},{480,150,10,7},{812,505,12,8}}
local h = nil
for i, p in ipairs(HOLEPTS) do local e = ellipse(p[1], p[2], p[3], p[4]) h = h and (h + e) or e end
HOLES = h:roughen(5, 9, 5, 1.5)
MASS = edge + core - HOLES
local Lb, Mb = tree:light():blur(10), lv:blur(10)
LB = mask(function(x, y) local m = Mb:at(x, y) if m < 0.05 then return 0 end return clamp(Lb:at(x, y) / m, 0, 1) end)
skym = above(function(x) return HZ + 6 + 3 * math.sin(x / 70) end)
TRUNKM = tree:wood(9) * mask(function(x, y) return smoothstep(540, 580, y) end)
local sk = skym - MASS:shrink(1.5):soften(1.5) - TRUNKM:shrink(1.5)
work(sk, {hand="broad", color=sky, angle=0, coverage=5, medium=0.22, load=1, pal=skypal, clip=sk})
work(sk, {hand="broad", color=sky, angle=0.01, coverage=3.5, medium=0.25, load=0.9, pal=skypal, clip=sk})
blend(sk, {angle=0, coverage=1.5, clip=sk})
local ld = land - TRUNKM:shrink(1.5)
work(ld, {hand="body", color=landcol, angle=0.02, length={20, 60}, coverage=3.4, medium=0.18, clip=ld})

--@ chunk 3 · clock 0

-- B chunk 3: an hour and a half later, the sky still open. The trunk, then the whole crown
-- as ONE connected dark mass with a big brush, on the reserved ground; at its edge it runs
-- a little way into the open sky, so the edge softens.
wait(90)
print("sky at the crown edge:", drying(160, 400), drying(820, 300))
local tr = TRUNKM
work(tr, {hand="body", tool="filbert 5", length={10, 30}, coverage=4.2, load=0.8, angle=1.55, angle_jitter=0.15, clip=tr, medium=0.18,
  color=function(x, y) return mix("#2e2a25", "#3d3831", smoothstep(620, 740, y)) end})
local big = noise{seed=21, period=45}
work(MASS, {hand="body", tool="filbert 7", length={10, 24}, coverage=2.6, medium=0.3, load=0.55, clip=MASS,
  angle=function(x, y) return 2.0 * big(x, y) end, angle_jitter=0.5, curve={0.25, 0.1},
  color=function(x, y) local v = LB:at(x, y) return mix(mix("#1f281b", "#2f3a26", smoothstep(0.15, 0.5, v)), "#3e4a2c", smoothstep(0.55, 0.8, v)) end})

--@ chunk 4 · clock 90

-- B chunk 4: 15 hours later, the dark SETTING (open: the lights plowed it into a camouflage mottle
-- down to the ground; tacky: flat poster slabs). The lights, few and unequal, laid into the setting
-- dark with stiffer paint (medium 0.12): the half-light masses on the sun side, then the top lights
-- on a few of them. Short strokes turning with the lobes; hug=false so their edges thin into the dark.
wait(900)
print("dark:", drying(300, 300), drying(600, 400))
local big = noise{seed=21, period=45}
local way = function(x, y) return 2.0 * big(x, y) + 0.4 end
LIT = MASS:shrink(2) * LB:map(function(v) return smoothstep(0.52, 0.68, v) end)
TOP = MASS:shrink(3) * LB:map(function(v) return smoothstep(0.72, 0.86, v) end)
work(LIT, {hand="body", tool="filbert 4", length={6, 14}, coverage=2.2, medium=0.12, hug=false, clip=MASS,
  angle=way, angle_jitter=0.6, curve={0.3, 0.1},
  color=function(x, y) return mix("#4b5732", "#627040", smoothstep(0.6, 0.85, LB:at(x, y))) end})
work(TOP, {hand="body", tool="filbert 3", length={4, 10}, coverage=1.6, medium=0.1, hug=false, clip=MASS,
  angle=way, angle_jitter=0.7, color="#8a9456"})
print(LIT:area(), TOP:area())
-- the trunk's lit flank on the sun side, only below the crown (in it, a flank reads as a gray pole)
local trv = TRUNKM - MASS
local fl = trv * mask(function(x, y) return 1 - TRUNKM:at(x - 6, y) end)
work(fl, {hand="body", tool="round 3", length={8, 20}, coverage=1.8, angle=1.55, clip=trv, medium=0.12, color="#6b6559"})

--@ chunk 5 · clock 990
-- B chunk 5: the same session, mass and sky both setting. The edge decided by hand: ~450 short
-- flicks pulled from inside the setting mass out past its contour (a contour dragged into the
-- sky), each in its light family (dark on the shade side, lit where the sun reaches the rim),
-- so the silhouette is leaves, not a cut line. No touches inside the masses.
print("mass:", drying(150, 330), "sky:", drying(90, 330))
local MB = MASS:blur(4)
local pts = {}
local tries = 0
while #pts < 460 and tries < 200000 do
  tries = tries + 1
  local x, y = rand(90, 950), rand(60, 640)
  local m = MASS:at(x, y)
  if m > 0.35 and m < 0.65 and HOLES:at(x, y) < 0.1 and y < 600 then pts[#pts + 1] = {x, y} end
end
local bd, bm, bl = brush("round", 2.6), brush("round", 2.4), brush("round", 2.2)
for i, p in ipairs(pts) do
  local x, y = p[1], p[2]
  local gx = MB:at(x + 2, y) - MB:at(x - 2, y)
  local gy = MB:at(x, y + 2) - MB:at(x, y - 2)
  local g = math.sqrt(gx * gx + gy * gy) + 1e-6
  local ox, oy = -gx / g, -gy / g                   -- outward
  local a = math.atan(oy, ox) + randn(0, 0.35)
  local out, inn = rand(3, 9), rand(3, 6)
  local hook = randn(0, 0.5)
  local x0, y0 = x - math.cos(a) * inn, y - math.sin(a) * inn
  local x2, y2 = x + math.cos(a + hook) * out, y + math.sin(a + hook) * out
  local v = LB:at(x0, y0)
  local b, c = bd, "#26311f"
  if v > 0.68 then b, c = bl, "#7f8a4e" elseif v > 0.5 then b, c = bm, "#4f5c35" end
  if i % 5 == 1 or b:fullness() < 0.3 then b:reload(c, 0.7) end
  b:stroke({{x0, y0}, {x, y}, {x2, y2}}, {pressure={0.7, 0}, ramps={0.05, 0.7}})
end
print("flicks", #pts)

--@ chunk 6 · clock 990
-- B chunk 6: once the sky in the holes is DRY (over setting sky a loaded dark stroke laid a
-- translucent gray streak). One limb per hole, drawn once:
-- the stoutest limb that crosses each sky hole, a single round-brush stroke pressed to its width,
-- clipped to the hole. (Filling the grown wood mask there gave crossed planks at 3200.)
wait(1020)
local t0 = clock()
while drying(760, 445) ~= "dry" and clock() - t0 < 30*24*60 do wait(12*60) end
print("waited for the hole sky to dry:", (clock() - t0) / 60, "h")
print("holes:", drying(425,318), drying(760,445), drying(330,520))
local HG = HOLES:grow(3)
local best = {}
for li, l in ipairs(tree.limbs) do
  for k, p in ipairs(l.pts) do
    if HG:at(p[1], p[2]) > 0.5 then
      for hi, h in ipairs(HOLEPTS) do
        if math.abs(p[1] - h[1]) < h[3] + 6 and math.abs(p[2] - h[2]) < h[4] + 6 then
          local w = l.w[k] or l.w[1]
          if not best[hi] or w > best[hi].w then best[hi] = {w=w, limb=l, h=h} end
        end
      end
    end
  end
end
local b = brush("round", 3)
local n = 0
for hi, e in pairs(best) do
  if e.w >= 1.2 then
    -- only the stretch of the limb near its hole (the whole limb, clipped, ran the brush dry
    -- before the hole: a gray translucent streak)
    local seg, h = {}, e.h
    for _, p in ipairs(e.limb.pts) do
      if math.abs(p[1] - h[1]) < h[3] + 14 and math.abs(p[2] - h[2]) < h[4] + 14 then seg[#seg + 1] = p end
    end
    if #seg < 2 then goto continue end
    b:reload("#2d2924", 0.9)
    b:stroke(seg, {pressure={b:pressure_for(math.min(3, e.w + 0.6)), b:pressure_for(math.max(0.8, e.w * 0.6))}, clip=HG})
    n = n + 1
  end
  ::continue::
end
print("limbs", n)

--@ chunk 7 · clock 14970
-- B chunk 7: the same day. The last accents, a very few: the brightest lights as single touches
-- on the tops of the lit lobes. (A cool veil on the front shade masses, tried here, laid flat
-- gray slabs in the exact shape of its mask over the tacky dark, even thin and a step off the
-- dark: dropped. The dark half stays one mass.)
local top, k = {}, 0
for _, t in ipairs(tree:touches{lit={0.84, 1}}) do k = k + 1 if k % 25 == 0 and MASS:at(t.pts[1][1], t.pts[1][2]) > 0.8 then top[#top + 1] = t end end
local b = brush("round", tree.touch_w * 0.9)
for i, t in ipairs(top) do
  if i % 8 == 1 then b:reload("#a5aa6c", 0.7) end
  b:stroke(t.pts, {pressure={0.8, 0.1}, ramps={0.1, 0.6}})
end
print("top lights", #top)

--@ chunk 8 · clock 14970

-- B chunk 8: the trunk showed the warm ground in streaks through its one body pass (a second
-- wet pass plowed more of it open): one transparent dark glaze once it is dry, trunk only
glaze(TRUNKM - MASS, {color="#2c2723", coats=0.45, pigment="transparent"})

--@ chunk 9 · clock 30137.3603515625
wait(24*60); varnish{color="#e6d3a4", coats=0.3, vary=0.1}; relief()
