-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{
  size = 450,
  aspect = 1.25,
  linen = {16, 14},
  seed = 42,
  ground = {
    {pile = {{"lead white", 5}, {"raw umber", 1}}, um = 90, apply = "knife", texture = 0.22},
    {pile = {{"lead white", 4}, {"yellow ochre", 2}, {"raw umber", 1}}, um = 30, apply = "brush"}
  }
}

--@ chunk 2
local c = chalk()
-- Table ledge: shelf line
c:line({{0, 565}, {1000, 568}}, {pressure = {0.3, 0.4, 0.35}})
-- Shelf front bevel
c:line({{0, 580}, {1000, 583}}, {pressure = {0.25, 0.3, 0.25}})

-- Pitcher outline
c:sketch({
  {250, 260}, {235, 275}, {230, 310}, {215, 360}, {200, 420}, {205, 480}, {225, 530}, {260, 555},
  {310, 555}, {345, 530}, {365, 480}, {370, 420}, {355, 360}, {340, 310}, {335, 275}, {320, 260}, {250, 260}
}, {pressure = 0.35, wander = 2})

-- Pitcher rim ellipse
c:sketch({
  {250, 260}, {285, 252}, {320, 260}, {285, 268}, {250, 260}
}, {pressure = 0.3, wander = 1})

-- Pitcher handle
c:sketch({
  {345, 315}, {385, 330}, {400, 380}, {385, 430}, {355, 450}
}, {pressure = 0.3, wander = 1.5})

-- First quince (upright, center)
c:sketch({
  {510, 430}, {485, 470}, {470, 520}, {480, 560}, {530, 575}, {585, 565}, {605, 520}, {590, 465}, {550, 425}, {510, 430}
}, {pressure = 0.35, wander = 2})

-- Second quince (tilted, right)
c:sketch({
  {660, 525}, {635, 560}, {645, 595}, {680, 610}, {735, 605}, {775, 575}, {770, 540}, {725, 515}, {660, 525}
}, {pressure = 0.35, wander = 2})

-- Knife on the ledge
c:line({{350, 555}, {420, 572}}, {pressure = 0.3}) -- handle
c:line({{420, 572}, {505, 592}}, {pressure = 0.35}) -- blade

--@ chunk 3
-- Piles for underpainting and environment
p_bg_dark = pile{{"bone black", 3}, {"raw umber", 4}, {"red earth", 1}, medium = 0.2}
p_bg_warm = pile{{"raw umber", 4}, {"yellow ochre", 2}, {"bone black", 1}, medium = 0.25}
p_stone_top = pile{{"lead white", 4}, {"yellow ochre", 2}, {"raw umber", 2}, medium = 0.15}
p_stone_face = pile{{"raw umber", 4}, {"bone black", 2}, {"red earth", 1}, {"lead white", 1}, medium = 0.2}
p_shadow = pile{{"raw umber", 4}, {"bone black", 1}, medium = 0.3}

-- Object masks
o_pitcher = outline{
  {250, 260}, {235, 275}, {230, 310}, {215, 360}, {200, 420}, {205, 480}, {225, 530}, {260, 555},
  {310, 555}, {345, 530}, {365, 480}, {370, 420}, {355, 360}, {340, 310}, {335, 275}, {320, 260},
  closed = true, char = "firm"
}
m_pitcher = o_pitcher:mask()
m_handle = ribbon({{345, 315}, {385, 330}, {400, 380}, {385, 430}, {355, 450}}, 22)

o_quince1 = outline{
  {510, 430}, {485, 470}, {470, 520}, {480, 560}, {530, 575}, {585, 565}, {605, 520}, {590, 465}, {550, 425},
  closed = true, char = "soft"
}
m_quince1 = o_quince1:mask()

o_quince2 = outline{
  {660, 525}, {635, 560}, {645, 595}, {680, 610}, {735, 605}, {775, 575}, {770, 540}, {725, 515},
  closed = true, char = "soft"
}
m_quince2 = o_quince2:mask()

m_objects = (m_pitcher + m_handle + m_quince1 + m_quince2):shrink(4)

-- Background regions
local m_bg_all = rect(0, 0, 1000, 568) - m_objects
local m_bg_left = m_bg_all * mask(function(x, y) return clamp((500 - x) / 250, 0, 1) end)
local m_bg_right = m_bg_all * mask(function(x, y) return clamp((x - 300) / 300, 0, 1) end)

-- Lay in background: darker toward right/top, warmer behind pitcher
work(m_bg_right, {hand = "broad", pile = p_bg_dark, angle = -0.35, coverage = 1.8, fill = true})
work(m_bg_left, {hand = "broad", pile = p_bg_warm, angle = 0.25, coverage = 1.6, fill = true})
blend(m_bg_all, {angle = 0, coverage = 1.0})

-- Stone shelf
local m_shelf_top = rect(0, 565, 1000, 18) - m_objects
local m_shelf_face = rect(0, 580, 1000, 220)

work(m_shelf_top, {hand = "body", tool = "filbert 16", pile = p_stone_top, angle = 0.05, coverage = 1.8, fill = true})
work(m_shelf_face, {hand = "body", tool = "filbert 20", pile = p_stone_face, angle = 1.57, coverage = 2.0, fill = true})
blend(m_shelf_face, {angle = 1.57, coverage = 0.8})

--@ chunk 4
-- Unifying background and establishing correct table perspective

-- 1. Unify upper background with a glazing pass and broad blending
local m_upper_wall = rect(0, 0, 1000, 520)
work(m_upper_wall, {
  hand = "broad",
  tool = "filbert 24",
  pile = pile{{"raw umber", 4}, {"bone black", 2}, {"yellow ochre", 1}, medium = 0.35},
  angle = -0.2,
  coverage = 1.2
})
blend(m_upper_wall, {hand = "blend", tool = "badger 40", coverage = 1.8, angle = 0})

-- 2. Stone tabletop plane: from y = 520 to y = 620
-- The table surface slopes very slightly or sits flat in perspective
local m_table_surface = rect(0, 520, 1000, 100)
p_table_top = pile{{"lead white", 4}, {"yellow ochre", 2}, {"raw umber", 2}, {"bone black", 1}, medium = 0.15}

-- Work table top horizontally with wide strokes
work(m_table_surface, {
  hand = "body",
  tool = "filbert 20",
  pile = p_table_top,
  angle = 0.0,
  coverage = 2.0,
  fill = true
})

-- 3. Stone ledge front face: from y = 622 down to 800
local m_table_face = rect(0, 622, 1000, 178)
p_table_face = pile{{"raw umber", 4}, {"bone black", 3}, {"red earth", 1}, medium = 0.2}

work(m_table_face, {
  hand = "body",
  tool = "filbert 22",
  pile = p_table_face,
  angle = 1.57,
  coverage = 2.0,
  fill = true
})
blend(m_table_face, {hand = "blend", angle = 1.57, coverage = 1.0})

-- 4. Crisp catch-light along the front stone edge (y = 620 to 622)
local b_line = brush{kind = "flat", width = 3, stiffness = 0.7}
b_line:load(pile{{"lead white", 5}, {"yellow ochre", 2}, {"raw umber", 1}}, 0.9)
b_line:stroke({{0, 621}, {1000, 621}}, {pressure = {0.7, 0.75, 0.65}, orient = "along"})

--@ chunk 5
print("canvas time:", wait(0))
print("table drying:", drying(500, 570))
print("bg drying:", drying(500, 200))

--@ chunk 6
-- 1. Cast shadows on the stone shelf
p_cast_shadow = pile{{"raw umber", 4}, {"bone black", 2}, {"red earth", 1}, medium = 0.3}

local m_cast_pitcher = poly({{260, 568}, {340, 565}, {475, 590}, {460, 612}, {300, 605}, {260, 568}}, true):soften(8)
local m_cast_q1 = poly({{500, 578}, {575, 572}, {655, 590}, {640, 608}, {520, 602}, {500, 578}}, true):soften(6)
local m_cast_q2 = poly({{680, 610}, {755, 595}, {835, 612}, {820, 628}, {705, 622}, {680, 610}}, true):soften(6)

work(m_cast_pitcher, {hand = "body", tool = "filbert 12", pile = p_cast_shadow, angle = 0.2, coverage = 1.4})
work(m_cast_q1, {hand = "body", tool = "filbert 9", pile = p_cast_shadow, angle = 0.2, coverage = 1.4})
work(m_cast_q2, {hand = "body", tool = "filbert 9", pile = p_cast_shadow, angle = 0.2, coverage = 1.4})

-- 2. Pitcher Piles
p_jug_shadow = pile{{"raw umber", 5}, {"bone black", 3}, {"red earth", 2}, medium = 0.1}
p_jug_mid = pile{{"red earth", 4}, {"raw umber", 3}, {"yellow ochre", 2}, medium = 0.1}
p_jug_light = pile{{"yellow ochre", 4}, {"red earth", 2}, {"lead white", 2}, medium = 0.05}
p_jug_rim = pile{{"bone black", 5}, {"raw umber", 2}, medium = 0.1}

-- Pitcher body outline
o_pitcher = outline{
  {245, 275}, {240, 310}, {225, 360}, {195, 430}, {190, 480}, {210, 530}, {240, 568},
  {280, 573}, {320, 568}, {350, 530}, {370, 480}, {365, 430}, {335, 360}, {320, 310}, {315, 275},
  closed = true, char = "firm"
}
m_pitcher = o_pitcher:mask()

-- Shadow side of pitcher (right)
local m_pitcher_sh = m_pitcher * mask(function(x, y) return clamp((x - 265) / 60, 0, 1) end)
-- Midtone of pitcher (turning flank)
local m_pitcher_mid = m_pitcher * mask(function(x, y) return clamp(1 - math.abs(x - 265) / 55, 0, 1) end)
-- Light side of pitcher (left)
local m_pitcher_lt = m_pitcher * mask(function(x, y) return clamp((275 - x) / 55, 0, 1) end)

work(m_pitcher_sh, {hand = "body", tool = "filbert 12", pile = p_jug_shadow, angle = 1.4, coverage = 2.0, fill = true, clip = true})
work(m_pitcher_mid, {hand = "body", tool = "filbert 12", pile = p_jug_mid, angle = 1.3, coverage = 1.8, fill = true, clip = true})
work(m_pitcher_lt, {hand = "body", tool = "filbert 12", pile = p_jug_light, angle = 1.2, coverage = 1.8, fill = true, clip = true})

-- Blend across pitcher body to fuse values smoothly
blend(m_pitcher, {hand = "blend", tool = "badger 20", angle = 0.1, coverage = 1.2, clip = true})

-- Pitcher interior mouth cavity
local m_mouth = ellipse(280, 275, 34, 11)
work(m_mouth, {hand = "body", tool = "filbert 6", pile = p_jug_rim, angle = 0, coverage = 2.2, clip = true})

-- Handle
local m_handle = ribbon({{330, 320}, {385, 360}, {395, 410}, {380, 450}, {345, 475}}, 18)
work(m_handle, {hand = "body", tool = "filbert 6", pile = p_jug_mid, angle = 1.5, coverage = 2.0, clip = true})
local m_handle_sh = m_handle * mask(function(x, y) return clamp((x - 370) / 20, 0, 1) end)
work(m_handle_sh, {hand = "body", tool = "filbert 4", pile = p_jug_shadow, angle = 1.5, coverage = 1.5, clip = true})

-- 3. Quinces Piles
p_q_shadow = pile{{"raw umber", 3}, {"yellow ochre", 2}, {"green earth", 2}, medium = 0.1}
p_q_mid = pile{{"yellow ochre", 5}, {"chrome yellow", 2}, {"lead white", 2}, {"green earth", 1}, medium = 0.05}
p_q_light = pile{{"lead white", 4}, {"chrome yellow", 3}, {"yellow ochre", 2}, medium = 0.0}
p_q_blush = pile{{"vermilion", 2}, {"yellow ochre", 3}, {"lead white", 1}, medium = 0.1}

-- Quince 1 (upright)
o_q1 = outline{
  {515, 420}, {490, 455}, {475, 500}, {465, 545}, {490, 575}, {540, 582}, {585, 570}, {610, 535}, {605, 485}, {575, 440}, {545, 415},
  closed = true, char = "soft"
}
m_q1 = o_q1:mask()

local m_q1_sh = m_q1 * mask(function(x, y) return clamp((x - 520) / 45 + (y - 500) / 60, 0, 1) end)
local m_q1_mid = m_q1 * mask(function(x, y) return clamp(1 - math.abs((x - 520) / 45), 0, 1) end)
local m_q1_lt = m_q1 * mask(function(x, y) return clamp((530 - x) / 45 + (510 - y) / 60, 0, 1) end)

work(m_q1_sh, {hand = "body", tool = "filbert 8", pile = p_q_shadow, angle = 1.2, coverage = 2.0, fill = true, clip = true})
work(m_q1_mid, {hand = "body", tool = "filbert 8", pile = p_q_mid, angle = 1.0, coverage = 1.8, fill = true, clip = true})
work(m_q1_lt, {hand = "body", tool = "filbert 8", pile = p_q_light, angle = 0.8, coverage = 1.8, fill = true, clip = true})
blend(m_q1, {hand = "blend", tool = "badger 12", angle = 0.5, coverage = 1.0, clip = true})

-- Quince 2 (reclining)
o_q2 = outline{
  {660, 535}, {635, 565}, {640, 595}, {675, 615}, {725, 615}, {770, 590}, {785, 555}, {765, 525}, {715, 515},
  closed = true, char = "soft"
}
m_q2 = o_q2:mask()

local m_q2_sh = m_q2 * mask(function(x, y) return clamp((x - 700) / 50 + (y - 565) / 40, 0, 1) end)
local m_q2_mid = m_q2 * mask(function(x, y) return clamp(1 - math.abs((x - 700) / 50), 0, 1) end)
local m_q2_lt = m_q2 * mask(function(x, y) return clamp((715 - x) / 50 + (570 - y) / 40, 0, 1) end)

work(m_q2_sh, {hand = "body", tool = "filbert 8", pile = p_q_shadow, angle = 1.4, coverage = 2.0, fill = true, clip = true})
work(m_q2_mid, {hand = "body", tool = "filbert 8", pile = p_q_mid, angle = 1.1, coverage = 1.8, fill = true, clip = true})
work(m_q2_lt, {hand = "body", tool = "filbert 8", pile = p_q_light, angle = 0.9, coverage = 1.8, fill = true, clip = true})
blend(m_q2, {hand = "blend", tool = "badger 12", angle = 0.5, coverage = 1.0, clip = true})

--@ chunk 7
-- CHUNK 7: Sculpting forms, glazed pitcher, quinces, knife, and foliage

-- 1. Unify and smooth the stone shelf plane
p_stone_smooth = pile{{"lead white", 5}, {"yellow ochre", 2}, {"raw umber", 2}, {"bone black", 1}, medium = 0.2}
local m_table_plane = rect(0, 520, 1000, 102) - (m_pitcher + m_q1 + m_q2):shrink(2)
work(m_table_plane, {hand = "broad", tool = "filbert 22", pile = p_stone_smooth, angle = 0.0, coverage = 1.6, fill = true})
blend(m_table_plane, {hand = "blend", tool = "badger 30", angle = 0.0, coverage = 1.0})

-- Reinforced crisp catch-light on stone shelf edge
local b_rim = brush{kind = "flat", width = 3, stiffness = 0.8}
b_rim:load(pile{{"lead white", 6}, {"yellow ochre", 1}, {"raw umber", 1}}, 0.9)
b_rim:stroke({{0, 621}, {1000, 621}}, {pressure = {0.6, 0.7, 0.6}, orient = "along"})

-- Deepen contact shadows under objects
local p_contact = pile{{"bone black", 4}, {"raw umber", 4}, medium = 0.2}
local b_sh = brush{kind = "filbert", width = 8, stiffness = 0.6}
b_sh:load(p_contact, 0.8)
-- Under pitcher base
b_sh:stroke({{230, 570}, {330, 570}, {460, 590}}, {pressure = {0.8, 0.5, 0.1}})
-- Under Quince 1
b_sh:stroke({{490, 580}, {570, 580}, {640, 595}}, {pressure = {0.7, 0.5, 0.1}})
-- Under Quince 2
b_sh:stroke({{660, 612}, {740, 615}, {820, 622}}, {pressure = {0.7, 0.5, 0.1}})

-- 2. Refine the earthenware pitcher
-- Pitcher mouth opening and lip
local p_lip_lt = pile{{"lead white", 5}, {"yellow ochre", 3}, {"red earth", 1}}
local p_mouth_dark = pile{{"bone black", 6}, {"raw umber", 2}, medium = 0.05}
local b_detail = brush{kind = "round", width = 3, point = 1, stiffness = 0.7}

-- Interior dark of the neck
b_detail:load(p_mouth_dark, 0.9)
b_detail:stroke({{255, 275}, {280, 277}, {305, 275}}, {pressure = {0.8, 0.9, 0.7}})

-- Lip rim catching light on the left
b_detail:load(p_lip_lt, 0.9)
b_detail:stroke({{245, 273}, {270, 273}, {295, 274}}, {pressure = {0.8, 0.6, 0.2}})
-- Pouring spout dip
b_detail:stroke({{242, 272}, {247, 278}}, {pressure = {0.6, 0.3}})

-- Smooth the pitcher handle
local p_handle_body = pile{{"raw umber", 4}, {"red earth", 3}, {"bone black", 1}}
local p_handle_lt = pile{{"yellow ochre", 4}, {"red earth", 3}, {"lead white", 2}}
local b_h = brush{kind = "filbert", width = 7, stiffness = 0.6}
b_h:load(p_handle_body, 0.85)
b_h:stroke({{332, 320}, {372, 350}, {390, 395}, {382, 435}, {348, 465}}, {pressure = {0.7, 0.8, 0.8, 0.7, 0.6}})
-- Handle outer crest highlight
local b_pt = brush{kind = "round", width = 2.5, point = 1, stiffness = 0.7}
b_pt:load(p_handle_lt, 0.8)
b_pt:stroke({{335, 318}, {376, 348}, {393, 392}}, {pressure = {0.3, 0.6, 0.2}})

-- Modeling pitcher body volume with curved strokes
local p_jug_glow = pile{{"red earth", 4}, {"yellow ochre", 4}, {"lead white", 1}, medium = 0.05}
local b_body = brush{kind = "filbert", width = 12, stiffness = 0.5}
b_body:load(p_jug_glow, 0.7)
b_body:stroke({{225, 340}, {215, 410}, {225, 480}, {245, 540}}, {pressure = {0.5, 0.7, 0.7, 0.4}})
b_body:stroke({{250, 335}, {240, 410}, {250, 490}, {275, 550}}, {pressure = {0.5, 0.7, 0.6, 0.3}})

-- Glaze Specular Sheen (glossy highlight)
local p_glaze_hi = pile{{"lead white", 6}, {"yellow ochre", 1}, medium = 0.0}
b_pt:load(p_glaze_hi, 0.85)
b_pt:stroke({{235, 345}, {223, 405}, {228, 465}}, {pressure = {0.2, 0.65, 0.15}, ramps = {0.1, 0.2}})
-- Small shoulder glint
b_pt:touch(240, 330, {pressure = 0.5, drag = {1, 2}})

-- Subtle reflected light on the shaded right flank of pitcher
local p_jug_bounce = pile{{"yellow ochre", 3}, {"red earth", 2}, {"lead white", 1}, medium = 0.3}
b_body:load(p_jug_bounce, 0.4)
b_body:stroke({{360, 400}, {365, 450}, {355, 510}}, {pressure = {0.3, 0.4, 0.2}})

-- 3. Modeling Quince 1 (upright)
local p_q_crest = pile{{"chrome yellow", 4}, {"lead white", 3}, {"yellow ochre", 2}}
local p_q_blush_v = pile{{"vermilion", 3}, {"yellow ochre", 3}, {"lead white", 1}, medium = 0.1}
local p_q_olive_sh = pile{{"yellow ochre", 3}, {"raw umber", 3}, {"green earth", 2}, medium = 0.1}

-- Upper crest and shoulder of Quince 1
local b_q = brush{kind = "filbert", width = 8, stiffness = 0.5}
b_q:load(p_q_crest, 0.8)
b_q:stroke({{515, 430}, {495, 470}, {485, 520}, {500, 560}}, {pressure = {0.6, 0.8, 0.7, 0.4}})
b_q:stroke({{535, 435}, {525, 480}, {520, 535}, {535, 570}}, {pressure = {0.5, 0.8, 0.7, 0.4}})

-- Warm vermilion blush on the crest
b_q:load(p_q_blush_v, 0.5)
b_q:stroke({{505, 465}, {495, 505}, {505, 545}}, {pressure = {0.3, 0.5, 0.2}})

-- Deepen shadow on right flank of Quince 1
b_q:load(p_q_olive_sh, 0.7)
b_q:stroke({{565, 450}, {590, 495}, {595, 545}, {575, 570}}, {pressure = {0.5, 0.7, 0.7, 0.4}})

-- Woody stem at top of Quince 1
local p_wood = pile{{"raw umber", 4}, {"bone black", 2}, {"lead white", 1}}
b_pt:load(p_wood, 0.85)
b_pt:stroke({{525, 430}, {522, 410}, {515, 395}, {510, 385}}, {pressure = {0.7, 0.5, 0.4, 0.2}})
-- Stem highlight
b_pt:load(pile{{"lead white", 4}, {"yellow ochre", 2}, {"raw umber", 1}}, 0.5)
b_pt:stroke({{523, 410}, {517, 395}}, {pressure = {0.3, 0.2}})

-- Quince leaves
local p_leaf_body = pile{{"copper green", 4}, {"yellow ochre", 2}, {"raw umber", 2}, {"bone black", 1}}
local p_leaf_hi = pile{{"copper green", 3}, {"yellow ochre", 3}, {"lead white", 2}}
-- Leaf 1: reaching out to the left
local m_leaf1 = poly({{518, 405}, {490, 395}, {465, 400}, {485, 415}, {518, 408}}, true)
work(m_leaf1, {hand = "detail", tool = "round 2.5", pile = p_leaf_body, coverage = 1.8, clip = true})
b_pt:load(p_leaf_hi, 0.6)
b_pt:stroke({{515, 406}, {488, 402}, {468, 401}}, {pressure = {0.4, 0.3, 0.1}}) -- vein

-- Leaf 2: pointing up-right behind stem
local m_leaf2 = poly({{520, 398}, {538, 385}, {555, 375}, {548, 392}, {522, 400}}, true)
work(m_leaf2, {hand = "detail", tool = "round 2.5", pile = p_leaf_body, coverage = 1.8, clip = true})

-- 4. Modeling Quince 2 (reclining)
-- Upper illuminated ridge
b_q:load(p_q_crest, 0.8)
b_q:stroke({{670, 545}, {710, 535}, {750, 540}, {770, 560}}, {pressure = {0.5, 0.8, 0.7, 0.3}})
b_q:stroke({{660, 565}, {700, 555}, {740, 560}, {765, 575}}, {pressure = {0.4, 0.7, 0.6, 0.3}})

-- Flank blush
b_q:load(p_q_blush_v, 0.45)
b_q:stroke({{690, 545}, {725, 545}, {750, 555}}, {pressure = {0.2, 0.4, 0.2}})

-- Calyx / blossom end on Quince 2 (dark sepals)
local p_calyx = pile{{"bone black", 4}, {"raw umber", 4}}
b_pt:load(p_calyx, 0.8)
b_pt:touch(772, 562, {pressure = 0.5, drag = {2, 0}})
b_pt:touch(770, 560, {pressure = 0.4, drag = {1, -2}})
b_pt:touch(774, 565, {pressure = 0.4, drag = {2, 2}})
b_pt:touch(768, 564, {pressure = 0.4, drag = {-1, 1}})

-- 5. The Knife on the Stone Table
local p_knife_handle = pile{{"bone black", 4}, {"raw umber", 3}, {"red earth", 2}}
local p_knife_bolster = pile{{"yellow ochre", 4}, {"lead white", 2}, {"raw umber", 1}}
local p_knife_blade = pile{{"lead white", 5}, {"cobalt blue", 1}, {"bone black", 1}, {"raw umber", 1}}
local p_knife_edge = pile{{"lead white", 6}, {"bone black", 1}}

-- Cast shadow under knife
local b_k_sh = brush{kind = "round", width = 2, stiffness = 0.7}
b_k_sh:load(pile{{"bone black", 4}, {"raw umber", 4}}, 0.7)
b_k_sh:stroke({{368, 572}, {432, 588}, {515, 608}}, {pressure = {0.5, 0.6, 0.4}})

-- Knife Handle (dark carved walnut)
local b_k = brush{kind = "flat", width = 6, stiffness = 0.7}
b_k:load(p_knife_handle, 0.9)
b_k:stroke({{365, 568}, {428, 584}}, {pressure = {0.8, 0.85}, orient = "across"})
-- Brass bolster ring
b_detail:load(p_knife_bolster, 0.9)
b_detail:stroke({{428, 581}, {432, 587}}, {pressure = {0.7, 0.7}})

-- Steel blade
local m_blade = poly({{432, 583}, {512, 603}, {510, 606}, {432, 588}}, true)
work(m_blade, {hand = "detail", tool = "round 2", pile = p_knife_blade, coverage = 2.0, clip = true})

-- Razor cutting edge catch-light along the top of the blade
b_pt:load(p_knife_edge, 0.9)
b_pt:stroke({{432, 583}, {512, 603}}, {pressure = {0.6, 0.4, 0.1}})
