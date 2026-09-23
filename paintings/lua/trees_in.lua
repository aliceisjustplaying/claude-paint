-- easel session "trees_in": broadleaved trees grown into drawn crowns.
--   easel run paintings/lua/trees_in.lua [--width 3200]
-- A group of field trees, an oak in leaf, the same oak bare, a beech and a birch.
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
canvas{style="friedrich", aspect=1.6, seed=31}; print(W, H)

--@ chunk 2 · clock 0
-- a quiet summer afternoon: pale gray-blue overhead, warm low down; the sun high on the left
HZ = 452
-- the world's sun, behind the left shoulder; trees are lit from it (sun=WORLD)
WORLD = world{horizon=HZ, sun={azimuth=-125, elevation=38}}
SUN = {-0.6, -0.65, 0.4}   -- about the same, for the lit flank offsets
sky = function(x, y) return gradient({{0,"#8d9cae"},{0.5,"#b5bcbf"},{0.85,"#d4d1c0"},{1,"#dcd5bd"}}, y/HZ) end
skym = above(function(x) return HZ + 10 end)
work(skym, {hand="broad", color=sky, angle=0, coverage=4.2, medium=0.25})
blend(skym, {angle=0})

--@ chunk 3 · clock 0
wait(24*60)
-- the ground: a far plain, pale and cool, a near meadow rising a little, warmer and darker
local g = noise{seed=4, octaves=4, period=260}
GROUND = function(x) return HZ + 4 * g(x, 0) end
land = below(GROUND)
work(land, {hand="body", length={20, 60}, angle=0.02, coverage=3.4, medium=0.18,
  color=function(x, y) return mix(mix("#9aa08a", "#6f7650", smoothstep(HZ, HZ + 60, y)), "#57603a", smoothstep(HZ + 60, H, y) + 0.12 * g(x * 3, y)) end})

--@ chunk 4 · clock 1440
wait(24*60)
-- field trees on the plain, far first: three crowns drawn (a broad low oak, a tall lime, a
-- lopsided oak), and a group grown from them in depth
F1 = {{560,452},{566,438},{580,430},{596,427},{612,433},{624,446},{622,458},{604,462},{584,459},{566,462}}
F2 = {{640,448},{643,428},{649,412},{656,418},{662,438},{663,460},{651,466},{640,462}}
F3 = {{392,462},{396,447},{407,437},{425,439},{438,449},{442,462},{428,470},{404,469}}
field = tree_group{crowns={F1, F2, F3}, trunks={{{592,481},{590,460}}, {{651,483},{651,464}}, {{416,488},{418,468}}},
                   species={"oak", "lime", "oak"}, count=6, horizon=HZ, spread=0.8, sun=WORLD, seed=19}
print(field)
local air = "#aeb3ab"
local sh = field:shadow():blur(1.5)
glaze(sh, {color="#4d5539", coats=0.25})
for i, t in ipairs(field:trees()) do
  local h = t.haze
  local lv = t:leaves()
  local L = t:light()
  t:paint_wood(brush("round", 1.2), {color=mix("#2f2a24", air, 0.55 * h), min=0.8})
  work(lv, {hand="hatch", tool="round 1.2", length={1.5, 3.5}, coverage=2.4, clip=lv, angle_jitter=1.2, hug=false, medium=0.2,
    color=function(x, y) local v = L:at(x, y) return mix(mix("#27301f", "#56623a", smoothstep(0.3, 0.8, v)), air, 0.7 * h) end})
  t:paint(brush("round", math.max(0.8, t.touch_w)), {color=mix("#6d7746", air, 0.6 * h), lit={0.6, 1}, share=0.7})
end

--@ chunk 5 · clock 1440
-- the oak in leaf: a crown drawn lopsided and lobed, a trunk leaning a little
OAKCROWN = {{70,236},{96,192},{140,170},{178,150},{226,160},{262,150},{300,178},{330,214},{346,262,"c"},{334,318},
            {310,352},{322,392,"c"},{270,410},{220,402},{178,416},{130,404},{84,396,"c"},{50,366},{38,318},{52,272}}
OAKTRUNK = {{196,592},{192,520},{187,452}}
oak = tree_in{crown=outline{pts=OAKCROWN, char="soft", seed=5}, trunk=OAKTRUNK, species="oak", season="summer", sun=WORLD, seed=7}
print(oak)

function paint_wood(t, o)
  o = o or {}
  local dark, light = o.dark or "#3b342c", o.light or "#7d7566"
  -- trunk and big limbs as filled ribbons of body paint, their lit flank lighter
  local thick = t:wood(o.thick or 3.5)
  work(thick, {hand="body", tool="round 2", length={4, 12}, coverage=3.5, angle=1.5, angle_jitter=0.6, clip=thick, color=dark, medium=0.15})
  -- the lit flank only on the stout wood
  local stout = t:wood(o.stout or 7)
  local lx, ly = -SUN[1] * 3, -SUN[2] * 1.5
  local flank = stout * mask(function(x, y) return 1 - stout:at(x + lx, y + ly) end)
  work(flank, {hand="body", tool="round 1.4", length={3, 8}, coverage=2.5, angle=1.5, clip=thick, color=light, medium=0.15})
  -- the rest as strokes pressed to their width
  t:paint_wood(brush("round", 2.4), {color=dark, min=1.2, max=3.5})
  t:paint_wood(brush("rigger", o.fine or 0.9), {color=o.twig or dark, max=1.2, pressure=o.tip})
end

function paint_leaves(t, o)
  o = o or {}
  local lv = t:leaves()
  local turn = noise{seed=o.seed or 9, period=9}
  local way = function(x, y) return 2.4 * turn(x, y) end
  -- 1. the body of the leaves, from the light on them: deep shade inside, lit masses toward the sun
  local L = t:light()
  local c0, c1, c2 = o.deep or "#222b1d", o.body or "#35412a", o.sunny or "#5c6a3a"
  work(lv, {hand="hatch", tool="round 1.6", length={3, 6}, coverage=2.4, clip=lv, angle=way, angle_jitter=0.8, hug=false, medium=0.2,
    color=function(x, y) local v = L:at(x, y) return mix(mix(c0, c1, smoothstep(0.1, 0.45, v)), c2, smoothstep(0.5, 0.85, v)) end})
  -- 2. the sky back into the crown's gaps, before the touches, so leaves break their edges
  local gp = t:gaps()
  work(gp, {hand="detail", tool="round 1.4", color=sky, clip=gp, coverage=2})
  -- 3. hooked touches: the shade, the half light, the light
  local b = brush("round", t.touch_w)
  t:paint(b, {color=o.shade or "#263020", lit={0, 0.4}})
  -- the front masses on the shade side catch the sky: cool, grayer, a share of them
  t:paint(b, {color=o.sky or "#4a5446", lit={0.15, 0.4}, depth={0.25, 1}, share=0.5})
  t:paint(b, {color=o.mid or "#46522e", lit={0.4, 0.6}})
  t:paint(brush("round", t.touch_w * 0.9), {color=o.lit or "#6f7c45", lit={0.6, 0.78}})
  t:paint(brush("round", t.touch_w * 0.8), {color=o.top or "#98a060", lit={0.78, 1}, every=6})
end

paint_wood(oak)
paint_leaves(oak)

--@ chunk 6 · clock 1440
-- the same oak in winter: the same crown, trunk and seed, moved right; bare, a few dead leaves kept low
local dx = 322
local function moved(pts) local o = {} for i, p in ipairs(pts) do o[i] = {p[1] + dx, p[2]} end return o end
bare = tree_in{crown=moved(oak.crown), trunk=moved(OAKTRUNK), species="oak", season="winter", sun=WORLD, seed=7}
print(bare)
-- the fine twigs lighter and thinner than the limbs: the crown's lace against the sky
paint_wood(bare, {twig="#554e45", fine=0.55, tip=0.04})
bare:paint(brush("round", bare.touch_w * 0.7), {color="#6e5234", every=4, share=0.4})

--@ chunk 7 · clock 1440
-- a beech: a tall dome, smooth gray trunk, rising limbs, level sprays
BEECH = {{700,214},{722,176},{760,160},{800,172},{826,208},{842,262},{846,322,"c"},{832,380},{800,418},{760,424},{722,414},{692,380},{680,320,"c"},{684,260}}
beech = tree_in{crown=outline{pts=BEECH, char="soft", seed=11}, trunk={{765,600},{762,500},{760,430}}, species="beech", sun=WORLD, seed=12}
print(beech)
paint_wood(beech, {dark="#5a5a55", light="#9d9a90", twig="#4c4a45"})
paint_leaves(beech, {deep="#1c2619", body="#324028", sunny="#5a6c38", shade="#1e2818", mid="#3e4c2a", lit="#687a40", top="#8e9a58", seed=13})

--@ chunk 8 · clock 1440
-- a birch: a narrow crown, a white stem leading high, hanging twigs, small airy leaves
BIRCH = {{918,150},{934,176},{948,230},{960,300,"c"},{966,372},{952,420},{918,432},{886,416},{872,360,"c"},{880,290},{894,220},{906,172}}
birch = tree_in{crown=outline{pts=BIRCH, char="soft", seed=15}, trunk={{921,610},{918,520},{922,440}}, species="birch", sun=WORLD, seed=16}
print(birch)
-- twigs and the thin upper wood dark and reddish, then the white stem where it is stout,
-- a gray shade side and black marks
birch:paint_wood(brush("rigger", 0.5), {color="#3d3833", max=1.2})
birch:paint_wood(brush("round", 1.4), {color="#4a3f38", min=1.2})
local stem = birch:wood(2.6)
work(stem, {hand="body", tool="round 1.6", length={3, 8}, coverage=3.2, angle=1.55, clip=stem, color="#d9d4c6", medium=0.15})
local shadeside = stem * mask(function(x, y) return 1 - stem:at(x - 2.2, y) end)
work(shadeside, {hand="body", tool="round 1.2", length={3, 6}, coverage=2.4, angle=1.55, clip=stem, color="#8f8c86"})
local bn = noise{seed=17, period=6, octaves=2}
-- black marks: short horizontal dashes and patches, heavier toward the foot
local marks = stem * mask(function(x, y) return smoothstep(0.42, 0.5, bn(x * 0.5, y * 2.2) + 0.3 * smoothstep(520, 610, y)) end)
work(marks, {hand="detail", tool="round 1.2", clip=stem, color="#26221f", coverage=1.5})
paint_leaves(birch, {deep="#2f3a24", body="#4a5a30", sunny="#7c8a45", shade="#34401f", mid="#566a30", lit="#86954a", top="#aab067", seed=18})
