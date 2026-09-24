-- easel session "bare_trees": bare broadleaved trees in winter that hold together.
--   easel run paintings/lua/bare_trees.lua [--width 3200]
-- A lime, the oak from trees_in.lua (same crown, trunk and seed), a beech and a birch, bare.
-- The stout wood is painted whole, a share of the fine twigs is drawn (tree_in detail=),
-- and the rest of the fine twig mass is indicated as a tone first (t:twig_mass()).
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.6, seed=31}; print(W, H)

--@ chunk 2 · clock 0
-- a quiet winter afternoon: pale gray-blue overhead, warm low down
HZ = 452
WORLD = world{horizon=HZ, sun={azimuth=-125, elevation=38}}
SUN = {-0.6, -0.65, 0.4}
sky = function(x, y) return gradient({{0,"#8d9cae"},{0.5,"#b5bcbf"},{0.85,"#d4d1c0"},{1,"#dcd5bd"}}, y/HZ) end
skym = above(function(x) return HZ + 10 end)
work(skym, {hand="broad", color=sky, angle=0, coverage=4.2, medium=0.25})
blend(skym, {angle=0})

--@ chunk 3 · clock 0
wait(24*60)
local g = noise{seed=4, octaves=4, period=260}
GROUND = function(x) return HZ + 4 * g(x, 0) end
land = below(GROUND)
work(land, {hand="body", length={20, 60}, angle=0.02, coverage=3.4, medium=0.18,
  color=function(x, y) return mix(mix("#9aa08a", "#6f7650", smoothstep(HZ, HZ + 60, y)), "#57603a", smoothstep(HZ + 60, H, y) + 0.12 * g(x * 3, y)) end})

--@ chunk 4 · clock 1440
wait(24*60)
-- how a bare tree is painted: the fine twig mass first, as a tone (a dry rigger dragged out
-- along the twigs, thin and pale, the sky through it), then the wood: the stout wood as body
-- paint with a lit flank, the thinner wood as strokes pressed to its width, each starting on
-- the wood it leaves from, the selected twigs last
function paint_bare(t, o)
  o = o or {}
  local dark, light, twig = o.dark or "#3b342c", o.light or "#7d7566", o.twig or "#554e45"
  local tm = t:twig_mass()
  local cx, cy = t.fork[1], t.fork[2]
  local way = o.hang and function(x, y) return 1.45 + 0.25 * clamp((x - cx) / 40, -1, 1) end
                      or function(x, y) return math.atan(y - cy, x - cx) end
  work(tm, {hand="body", tool="rigger 0.7", length={6, 14}, coverage=2, pressure={0.5, 0.2}, load=0.25, medium=0.35,
    threshold=0.05, hug=false, clip=tm:map(function(v) return (o.tone or 0.4) * v end), angle=way, angle_jitter=0.3,
    color=function(x, y) return mix(twig, sky(x, y), 0.5) end})
  local thick = t:wood(o.thick or 3.5)
  work(thick, {hand="body", tool="round 2", length={4, 12}, coverage=3.5, angle=1.5, angle_jitter=0.6, clip=thick, color=dark, medium=0.15})
  local stout = t:wood(o.stout or 7)
  local lx, ly = -SUN[1] * 3, -SUN[2] * 1.5
  local flank = stout * mask(function(x, y) return 1 - stout:at(x + lx, y + ly) end)
  work(flank, {hand="body", tool="round 1.4", length={3, 8}, coverage=2.5, angle=1.5, clip=thick, color=light, medium=0.15})
  t:paint_wood(brush("round", 2.4), {color=dark, min=1.2, max=o.thick or 3.5})
  t:paint_wood(brush("rigger", o.fine or 0.55), {color=twig, max=1.2, pressure=0.04})
end

--@ chunk 5 · clock 1440
-- a lime: a tall dense dome, many fine limbs
LIME = {{60,250},{84,196},{128,166},{176,160},{222,178},{258,214},{276,268,"c"},{280,330},{262,384},{220,412},{160,416},{104,404},{66,370},{48,316,"c"}}
lime = tree_in{crown=outline{pts=LIME, char="soft", seed=21}, trunk={{166,596},{164,520},{166,440}}, species="lime", season="winter", sun=WORLD, seed=22}
print(lime)
paint_bare(lime, {dark="#3d3630", twig="#5a5048"})

--@ chunk 6 · clock 1440
-- the oak of trees_in.lua, moved right: the same crown, trunk and seed, bare
OAKCROWN = {{70,236},{96,192},{140,170},{178,150},{226,160},{262,150},{300,178},{330,214},{346,262,"c"},{334,318},
            {310,352},{322,392,"c"},{270,410},{220,402},{178,416},{130,404},{84,396,"c"},{50,366},{38,318},{52,272}}
OAKTRUNK = {{196,592},{192,520},{187,452}}
local drawn = tree_in{crown=outline{pts=OAKCROWN, char="soft", seed=5}, trunk=OAKTRUNK, species="oak", season="winter", sun=WORLD, seed=7}
local function moved(pts, dx) local o = {} for i, p in ipairs(pts) do o[i] = {p[1] + dx, p[2]} end return o end
oak = tree_in{crown=moved(drawn.crown, 322), trunk=moved(OAKTRUNK, 322), species="oak", season="winter", sun=WORLD, seed=7}
print(oak, oak.detail)
paint_bare(oak)
-- the few dead leaves an oak keeps, low in the crown
oak:paint(brush("round", oak.touch_w * 0.7), {color="#6e5234", every=4, share=0.4})

--@ chunk 7 · clock 1440
-- a beech: smooth gray wood rising in a fan
BEECH = {{700,214},{722,176},{760,160},{800,172},{826,208},{842,262},{846,322,"c"},{832,380},{800,418},{760,424},{722,414},{692,380},{680,320,"c"},{684,260}}
beech = tree_in{crown=outline{pts=BEECH, char="soft", seed=11}, trunk={{765,600},{762,500},{760,430}}, species="beech", season="winter", sun=WORLD, seed=12}
print(beech)
paint_bare(beech, {dark="#5a5a55", light="#9d9a90", twig="#4c4a45"})

--@ chunk 8 · clock 1440
-- a birch: a white stem leading high, the fine twigs hanging in a purple-brown haze
BIRCH = {{918,150},{934,176},{948,230},{960,300,"c"},{966,372},{952,420},{918,432},{886,416},{872,360,"c"},{880,290},{894,220},{906,172}}
birch = tree_in{crown=outline{pts=BIRCH, char="soft", seed=15}, trunk={{921,610},{918,520},{922,440}}, species="birch", season="winter", sun=WORLD, seed=16}
print(birch)
paint_bare(birch, {dark="#4a3f38", twig="#4e3f3c", hang=true, tone=0.5, thick=2.6})
local stem = birch:wood(2.6)
work(stem, {hand="body", tool="round 1.6", length={3, 8}, coverage=3.2, angle=1.55, clip=stem, color="#d9d4c6", medium=0.15})
local shadeside = stem * mask(function(x, y) return 1 - stem:at(x - 2.2, y) end)
work(shadeside, {hand="body", tool="round 1.2", length={3, 6}, coverage=2.4, angle=1.55, clip=stem, color="#8f8c86"})
local bn = noise{seed=17, period=6, octaves=2}
local marks = stem * mask(function(x, y) return smoothstep(0.42, 0.5, bn(x * 0.5, y * 2.2) + 0.3 * smoothstep(520, 610, y)) end)
work(marks, {hand="detail", tool="round 1.2", clip=stem, color="#26221f", coverage=1.5})
