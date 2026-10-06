-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{
  size = 500,
  aspect = 1.25,
  linen = {18, 15},
  seed = 42,
  ground = {
    {pile = {{"lead white", 10}, {"raw umber", 2}, {"yellow ochre", 1}}, um = 80, apply = "knife", texture = 0.22},
    {pile = {{"lead white", 12}, {"raw umber", 1}, {"yellow ochre", 0.5}}, um = 35, apply = "knife", texture = 0.12}
  }
}
print(string.format("Canvas ready: W=%.0f, H=%.0f", W, H))

--@ chunk 2
c = chalk()

-- Back edge of the stone table
c:sketch({{0, 520}, {250, 522}, {500, 518}, {750, 520}, {1000, 516}}, {pressure = 0.4, wander = 2})

-- Front edge of the stone ledge
c:sketch({{0, 715}, {260, 712}, {520, 714}, {780, 710}, {1000, 712}}, {pressure = 0.5, wander = 1.5})
-- Bottom of the ledge chamfer/face
c:sketch({{0, 726}, {260, 724}, {520, 725}, {780, 722}, {1000, 723}}, {pressure = 0.35, wander = 1.5})

-- The earthenware pitcher
-- Rim / mouth ellipse
c:sketch({{335, 270}, {355, 262}, {385, 262}, {405, 270}, {395, 278}, {365, 280}, {335, 270}}, {pressure = 0.45})
-- Neck & shoulder left
c:sketch({{335, 270}, {338, 310}, {330, 340}, {305, 380}, {280, 430}, {270, 480}, {276, 530}, {295, 570}, {320, 600}}, {pressure = 0.45, wander = 2})
-- Neck & shoulder right
c:sketch({{405, 270}, {402, 310}, {412, 340}, {435, 380}, {460, 430}, {472, 480}, {468, 530}, {450, 570}, {425, 600}}, {pressure = 0.45, wander = 2})
-- Base
c:sketch({{320, 600}, {345, 606}, {375, 608}, {405, 606}, {425, 600}}, {pressure = 0.5})

-- Handle on left
c:sketch({{330, 335}, {295, 345}, {265, 375}, {250, 415}, {252, 455}, {272, 485}, {288, 498}}, {pressure = 0.45})
c:sketch({{330, 350}, {305, 360}, {280, 385}, {270, 418}, {270, 448}, {282, 475}, {290, 490}}, {pressure = 0.4})

-- Shadow terminator line on pitcher (curving down from right neck through belly)
c:sketch({{380, 275}, {375, 330}, {370, 390}, {378, 450}, {390, 520}, {405, 580}, {415, 603}}, {pressure = 0.3})

-- Cast shadow of pitcher on table
c:sketch({{420, 600}, {470, 605}, {540, 615}, {560, 630}, {530, 642}, {460, 635}, {400, 612}}, {pressure = 0.35})

-- The Quince
-- Silhouette
c:sketch({
  {630, 485}, {648, 500}, {658, 525}, {678, 555}, {692, 590}, {690, 630},
  {672, 660}, {640, 672}, {605, 670}, {575, 652}, {560, 625}, {562, 590},
  {578, 555}, {600, 525}, {615, 500}, {630, 485}
}, {pressure = 0.45, wander = 2})
-- Stem of quince
c:sketch({{630, 485}, {632, 465}, {638, 452}, {646, 446}}, {pressure = 0.4})
-- Quince shadow terminator
c:sketch({{636, 495}, {642, 530}, {650, 570}, {652, 615}, {640, 655}, {630, 670}}, {pressure = 0.3})
-- Quince cast shadow
c:sketch({{640, 672}, {680, 675}, {720, 680}, {735, 690}, {710, 698}, {660, 692}, {605, 675}}, {pressure = 0.35})

-- Walnuts in foreground
-- Cracked walnut half at x=480, y=650
c:sketch({{465, 642}, {485, 635}, {502, 644}, {505, 658}, {490, 668}, {470, 664}, {462, 652}, {465, 642}}, {pressure = 0.4})
c:sketch({{474, 648}, {485, 642}, {494, 648}, {488, 660}, {478, 658}}, {pressure = 0.35})
-- Whole walnut at x=535, y=660
c:sketch({{520, 655}, {535, 646}, {552, 650}, {560, 665}, {550, 678}, {530, 678}, {518, 668}, {520, 655}}, {pressure = 0.4})
c:sketch({{522, 665}, {538, 662}, {555, 664}}, {pressure = 0.3})

-- Old Knife lying diagonally across right foreground
-- Blade running from x=680, y=685 down toward ledge at x=780, y=715
c:sketch({{680, 683}, {780, 712}, {815, 722}}, {pressure = 0.45})
c:sketch({{680, 686}, {778, 716}, {813, 727}}, {pressure = 0.4})
-- Knife bolster / handle
c:sketch({{815, 722}, {835, 728}, {890, 746}, {902, 755}, {895, 762}, {835, 738}, {813, 727}}, {pressure = 0.45})

print("Underdrawing complete.")

--@ chunk 3
-- Palette piles for wall, table, and shadow block-in
p_wall_dark = pile{{"raw umber", 6}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.12}
p_wall_warm = pile{{"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 1}, {"lead white", 2}, medium = 0.15}
p_wall_light = pile{{"lead white", 5}, {"yellow ochre", 3.5}, {"raw umber", 2.5}, {"green earth", 0.5}, medium = 0.18}

p_table_lit = pile{{"lead white", 5.5}, {"yellow ochre", 3.5}, {"raw umber", 2}, {"red earth", 0.3}, medium = 0.12}
p_table_deep = pile{{"raw umber", 5.5}, {"bone black", 2.5}, {"yellow ochre", 2.5}, {"red earth", 0.6}, medium = 0.15}
p_ledge_face = pile{{"raw umber", 5}, {"bone black", 4}, {"red earth", 1}, medium = 0.12}

-- Mask for the back wall (above the back edge of the table)
local table_back = {{0, 520}, {250, 522}, {500, 518}, {750, 520}, {1000, 516}}
local m_above_table = above(table_back)

-- Masks for the objects to preserve their underdrawing
local pts_jug_mask = {
  {330, 260}, {310, 330}, {230, 390}, {235, 450}, {270, 510},
  {260, 540}, {310, 608}, {435, 608}, {480, 540}, {475, 450},
  {438, 360}, {415, 310}, {412, 260}
}
local m_jug_protect = poly(pts_jug_mask):shrink(4)

local pts_quince_mask = {
  {625, 440}, {645, 440}, {640, 480}, {660, 510}, {685, 550},
  {700, 600}, {680, 668}, {635, 680}, {595, 678}, {560, 640},
  {550, 580}, {570, 530}, {610, 490}
}
local m_quince_protect = poly(pts_quince_mask):shrink(4)

local m_wall = m_above_table - m_jug_protect - m_quince_protect

-- Split wall into tonal regions:
-- Darker left (behind lit jug), warmer mid-tone center/right, falling off into shadow at far right and top
local m_wall_left = m_wall * mask(function(x, y) return x < 320 and 1 or math.max(0, 1 - (x - 320) / 180) end)
local m_wall_center = m_wall * mask(function(x, y)
  local t = (x - 280) / 450
  return (t > 0 and t < 1) and math.sin(t * math.pi) or 0
end)
local m_wall_right = m_wall * mask(function(x, y) return x > 650 and math.min(1, (x - 650) / 200) or 0 end)

-- Lay in the wall with broad, responsive handling
work(m_wall_left, {
  hand = "broad",
  pile = p_wall_dark,
  angle = 0.2,
  coverage = 1.6,
  edge = "firm"
})

work(m_wall_center, {
  hand = "broad",
  pile = p_wall_warm,
  angle = -0.15,
  coverage = 1.7,
  edge = "soft"
})

work(m_wall_right, {
  hand = "broad",
  pile = p_wall_dark,
  angle = 0.25,
  coverage = 1.5,
  edge = "firm"
})

-- Blend the wet wall passages together for an atmospheric surface
blend(m_wall, {angle = 0.1})

print("Wall laid in and blended.")

--@ chunk 4
p_glaze_wall = pile{{"raw umber", 7}, {"bone black", 3.5}, {"yellow ochre", 1.8}, medium = 0.25}

local table_back = {{0, 520}, {250, 522}, {500, 518}, {750, 520}, {1000, 516}}
m_above_table = above(table_back)

local pts_jug = {
  {330, 260}, {310, 330}, {230, 390}, {235, 450}, {270, 510},
  {260, 540}, {310, 608}, {435, 608}, {480, 540}, {475, 450},
  {438, 360}, {415, 310}, {412, 260}
}
m_jug_protect = poly(pts_jug):shrink(4)

local pts_quince = {
  {625, 440}, {645, 440}, {640, 480}, {660, 510}, {685, 550},
  {700, 600}, {680, 668}, {635, 680}, {595, 678}, {560, 640},
  {550, 580}, {570, 530}, {610, 490}
}
m_quince_protect = poly(pts_quince):shrink(4)

m_wall = m_above_table - m_jug_protect - m_quince_protect

-- Unify the wall with a rich, semi-transparent body/glaze coat
work(m_wall, {
  hand = "glaze",
  pile = p_glaze_wall,
  coverage = 2.4,
  fill = true,
  angle = 0.05
})

-- Blend thoroughly with soft crossing passes
blend(m_wall, {angle = 0})
blend(m_wall, {angle = math.pi / 2})

print("Wall unified and blended.")

--@ chunk 5
-- Palette piles for tabletop and ledge
p_stone_lit = pile{{"lead white", 7}, {"yellow ochre", 4}, {"raw umber", 2.2}, {"bone black", 0.4}, {"red earth", 0.3}, medium = 0.1}
p_stone_mid = pile{{"lead white", 4.5}, {"yellow ochre", 4}, {"raw umber", 3.5}, {"bone black", 1}, {"red earth", 0.6}, medium = 0.12}
p_stone_shadow = pile{{"raw umber", 6}, {"bone black", 3}, {"yellow ochre", 2.5}, {"red earth", 1}, medium = 0.15}
p_ledge_face = pile{{"raw umber", 6}, {"bone black", 4.5}, {"yellow ochre", 1.5}, {"red earth", 1}, medium = 0.12}

local table_back = {{0, 520}, {250, 522}, {500, 518}, {750, 520}, {1000, 516}}
local table_front = {{0, 715}, {260, 712}, {520, 714}, {780, 710}, {1000, 712}}

local m_below_back = below(table_back)
local m_above_front = above(table_front)
local m_below_front = below(table_front)

-- Table plane is between back and front
local m_table_plane = m_below_back * m_above_front

-- Protect objects on the table plane
local pts_quince_table = {
  {625, 480}, {660, 510}, {685, 550}, {700, 600}, {680, 668},
  {635, 680}, {595, 678}, {560, 640}, {550, 580}, {570, 530}, {610, 490}
}
local m_quince_tab = poly(pts_quince_table):shrink(3)

local pts_jug_base = {
  {310, 550}, {440, 550}, {435, 610}, {310, 610}
}
local m_jug_tab = poly(pts_jug_base):shrink(3)

local m_table = m_table_plane - m_quince_tab - m_jug_tab

-- Table lit on left, slightly cooler/darker to the right
local m_table_left = m_table * mask(function(x, y) return math.max(0, 1 - (x / 750)) end)
local m_table_right = m_table * mask(function(x, y) return math.min(1, x / 750) end)

work(m_table_left, {
  hand = "body",
  pile = p_stone_lit,
  angle = 0.01,
  coverage = 2.4,
  fill = true,
  edge = "soft"
})

work(m_table_right, {
  hand = "body",
  pile = p_stone_mid,
  angle = 0.0,
  coverage = 2.4,
  fill = true,
  edge = "soft"
})

-- Front vertical face of the stone ledge
local m_front = m_below_front
work(m_front, {
  hand = "body",
  pile = p_ledge_face,
  angle = 0.0,
  coverage = 2.4,
  fill = true,
  edge = "firm"
})

-- Cast shadows on the table
local m_cast_jug = poly({{370, 605}, {420, 600}, {490, 610}, {560, 625}, {565, 640}, {520, 648}, {450, 642}, {380, 618}}, true)
local m_cast_quince = poly({{635, 672}, {680, 676}, {735, 685}, {745, 695}, {715, 702}, {660, 696}, {605, 678}}, true)

work(m_cast_jug * m_table_plane, {
  hand = "body",
  pile = p_stone_shadow,
  angle = 0.05,
  coverage = 1.9,
  edge = "soft"
})

work(m_cast_quince * m_table_plane, {
  hand = "body",
  pile = p_stone_shadow,
  angle = 0.05,
  coverage = 1.9,
  edge = "soft"
})

-- Blend table surface gently along its grain
blend(m_table_plane, {angle = 0})

print("Tabletop, front ledge face, and cast shadows laid in.")

--@ chunk 6
-- Earthenware pitcher palette piles
p_jug_shadow = pile{{"raw umber", 6}, {"red earth", 3.5}, {"bone black", 2.5}, {"yellow ochre", 1}, medium = 0.12}
p_jug_core   = pile{{"raw umber", 4.5}, {"red earth", 5.5}, {"yellow ochre", 2.5}, {"bone black", 0.8}, medium = 0.12}
p_jug_mid    = pile{{"red earth", 5}, {"yellow ochre", 5.5}, {"lead white", 2.2}, {"raw umber", 1.5}, medium = 0.1}
p_jug_lit    = pile{{"yellow ochre", 6}, {"red earth", 3}, {"lead white", 5}, {"raw umber", 0.8}, medium = 0.1}
p_jug_hi     = pile{{"lead white", 7.5}, {"yellow ochre", 3.5}, {"red earth", 1}, medium = 0.08}
p_jug_reflect= pile{{"yellow ochre", 4}, {"raw umber", 3}, {"red earth", 3}, {"lead white", 1.5}, medium = 0.15}
p_jug_dark   = pile{{"bone black", 6}, {"raw umber", 4}, {"red earth", 1}, medium = 0.1}

-- Silhouette of the jug body
local pts_jug_body = {
  {335, 270}, {338, 310}, {330, 340}, {305, 380}, {280, 430}, {270, 480},
  {276, 530}, {295, 570}, {320, 604}, {345, 608}, {375, 610}, {405, 608},
  {425, 604}, {450, 570}, {468, 530}, {472, 480}, {460, 430}, {435, 380},
  {412, 340}, {402, 310}, {405, 270}, {385, 262}, {355, 262}
}
local m_jug_solid = poly(pts_jug_body, true)

-- Define lighting zones on the jug:
-- Shadow side: x > 380
-- Lit side: x < 380
local m_jug_shadow = m_jug_solid * mask(function(x, y)
  local term_x = 372 + (y - 350) * 0.12
  return smoothstep(term_x - 15, term_x + 15, x)
end)

local m_jug_lit_all = m_jug_solid - m_jug_shadow

-- Within lit side: midtone vs bright lit crest
local m_jug_lit_crest = m_jug_lit_all * mask(function(x, y)
  local crest_x = 310 + (y - 350) * 0.08
  local d = math.abs(x - crest_x)
  return smoothstep(50, 10, d)
end)

local m_jug_lit_body = m_jug_lit_all - m_jug_lit_crest

-- Reflected light zone on the far right flank
local m_jug_reflect = m_jug_shadow * mask(function(x, y)
  return smoothstep(435, 465, x)
end)

local m_jug_core_zone = m_jug_shadow - m_jug_reflect

-- 1. Lay in core shadow and deep shadow
work(m_jug_core_zone, {
  hand = "body",
  pile = p_jug_core,
  angle = math.pi / 2,
  coverage = 2.4,
  fill = true,
  edge = "soft"
})

-- 2. Reflected bounce on right flank
work(m_jug_reflect, {
  hand = "body",
  pile = p_jug_reflect,
  angle = math.pi / 2,
  coverage = 2.2,
  fill = true,
  edge = "soft"
})

-- 3. Lit body
work(m_jug_lit_body, {
  hand = "body",
  pile = p_jug_mid,
  angle = 1.3,
  coverage = 2.5,
  fill = true,
  edge = "firm"
})

-- 4. Lit crest
work(m_jug_lit_crest, {
  hand = "body",
  pile = p_jug_lit,
  angle = 1.4,
  coverage = 2.5,
  fill = true,
  edge = "soft"
})

-- 5. Soft blend across the terminator for roundness
blend(m_jug_solid, {angle = 0})

-- 6. Inside of mouth (dark hole)
local m_mouth = ellipse(370, 268, 33, 8)
work(m_mouth, {
  hand = "body",
  pile = p_jug_dark,
  angle = 0,
  coverage = 2.5,
  fill = true,
  edge = "firm"
})

-- 7. The Handle: constructed with drawn strokes
local b_handle = brush{kind = "filbert", width = 14, point = 0.2}
local pts_handle_outer = {
  {330, 335}, {300, 345}, {266, 375}, {248, 415}, {250, 455}, {270, 485}, {290, 498}
}
-- Handle shadow underneath
b_handle:load(p_jug_shadow, 0.9)
b_handle:stroke(pts_handle_outer, {
  pressure = {0.7, 0.8, 0.6},
  swell = {0.9, 1.2, 1.0}
})
-- Handle lit crest along the top curvature
local b_handle_lit = brush{kind = "filbert", width = 9, point = 0.3}
local pts_handle_crest = {
  {330, 332}, {296, 340}, {260, 370}, {244, 410}, {246, 445}
}
b_handle_lit:load(p_jug_lit, 0.85)
b_handle_lit:stroke(pts_handle_crest, {
  pressure = {0.5, 0.8, 0.4},
  swell = {0.8, 1.1, 0.7}
})

-- Rim highlight on the mouth's left lip
local b_rim = brush{kind = "round", width = 4, point = 0.8}
b_rim:load(p_jug_hi, 0.8)
b_rim:stroke({{335, 270}, {348, 276}, {370, 277}}, {pressure = {0.7, 0.9, 0.3}})

print("Pitcher body, handle, rim, and modeling complete.")

--@ chunk 7
-- Negative space inside the handle
local m_handle_hole = poly({{300, 360}, {280, 385}, {266, 420}, {268, 455}, {282, 475}, {295, 455}, {312, 400}, {320, 365}}, true)
local m_handle_wall = m_handle_hole * mask(function(x, y) return y <= 520 and 1 or 0 end)
local m_handle_tab  = m_handle_hole * mask(function(x, y) return y > 520 and 1 or 0 end)

work(m_handle_wall, {hand = "body", pile = p_wall_dark, coverage = 2.5, fill = true, edge = "firm"})
work(m_handle_tab,  {hand = "body", pile = p_stone_lit, coverage = 2.5, fill = true, edge = "firm"})

-- Firm contact shadow under pitcher base
local b_detail = brush{kind = "round", width = 4, point = 0.9}
b_detail:load(p_jug_dark, 0.9)
b_detail:stroke({{315, 604}, {345, 608}, {375, 610}, {405, 608}, {428, 604}}, {
  pressure = {0.8, 1.0, 0.7},
  swell = {0.8, 1.2, 0.8}
})

-- Palette piles for the quince
p_quince_shadow = pile{{"raw umber", 5}, {"yellow ochre", 4}, {"green earth", 2.2}, {"bone black", 0.5}, medium = 0.12}
p_quince_core   = pile{{"raw umber", 4}, {"yellow ochre", 5.5}, {"red earth", 2}, medium = 0.12}
p_quince_body   = pile{{"yellow ochre", 6}, {"lead white", 4}, {"chrome yellow", 2}, {"green earth", 0.8}, medium = 0.1}
p_quince_lit    = pile{{"lead white", 6.5}, {"yellow ochre", 4.5}, {"chrome yellow", 2}, {"vermilion", 0.3}, medium = 0.08}
p_quince_hi     = pile{{"lead white", 8.5}, {"chrome yellow", 1.5}, {"yellow ochre", 1}, medium = 0.06}
p_quince_reflect= pile{{"yellow ochre", 5}, {"raw umber", 2.5}, {"lead white", 2}, {"red earth", 0.8}, medium = 0.15}
p_stem          = pile{{"raw umber", 7}, {"bone black", 3}, {"yellow ochre", 2}, medium = 0.1}

-- The Quince solid mask
local pts_quince = {
  {630, 485}, {648, 500}, {658, 525}, {678, 555}, {692, 590}, {690, 630},
  {672, 660}, {640, 672}, {605, 670}, {575, 652}, {560, 625}, {562, 590},
  {578, 555}, {600, 525}, {615, 500}
}
local m_quince = poly(pts_quince, true)

-- Lighting zones for the quince
local m_q_shadow = m_quince * mask(function(x, y)
  local term_x = 636 + (y - 550) * 0.1
  return smoothstep(term_x - 12, term_x + 12, x)
end)

local m_q_lit_all = m_quince - m_q_shadow

local m_q_lit_crest = m_q_lit_all * mask(function(x, y)
  local dist = math.sqrt((x - 595)^2 + (y - 560)^2)
  return smoothstep(45, 12, dist)
end)

local m_q_lit_body = m_q_lit_all - m_q_lit_crest

local m_q_reflect = m_q_shadow * mask(function(x, y)
  return smoothstep(665, 688, x) + smoothstep(650, 670, y) * 0.5
end)

local m_q_core_zone = m_q_shadow - m_q_reflect

-- 1. Lay in shadow core
work(m_q_core_zone, {
  hand = "body",
  pile = p_quince_core,
  angle = 1.2,
  coverage = 2.4,
  fill = true,
  edge = "soft"
})

-- 2. Ambient bounce on shadow edge
work(m_q_reflect, {
  hand = "body",
  pile = p_quince_reflect,
  angle = 1.0,
  coverage = 2.2,
  fill = true,
  edge = "soft"
})

-- 3. Lit flesh body
work(m_q_lit_body, {
  hand = "body",
  pile = p_quince_body,
  angle = 1.3,
  coverage = 2.5,
  fill = true,
  edge = "firm"
})

-- 4. Lit crest
work(m_q_lit_crest, {
  hand = "body",
  pile = p_quince_lit,
  angle = 1.2,
  coverage = 2.6,
  fill = true,
  edge = "soft"
})

-- 5. Blend gently across the quince for organic skin transitions
blend(m_quince, {angle = 0.3})

-- 6. Quince contact shadow at table
b_detail:load(p_jug_dark, 0.9)
b_detail:stroke({{600, 670}, {635, 674}, {665, 665}}, {
  pressure = {0.8, 1.0, 0.7}
})

-- 7. Quince Stem
local pts_stem = {{631, 488}, {634, 470}, {639, 455}, {645, 446}}
b_detail:load(p_stem, 0.9)
b_detail:stroke(pts_stem, {
  pressure = {0.8, 0.6, 0.4},
  swell = {1.2, 0.9, 0.7}
})
-- Tiny calyx indentation shadow
b_detail:load(p_jug_dark, 0.8)
b_detail:touch(631, 487, {pressure = 0.7})

print("Quince, stem, contact shadows, and handle hole painted.")

--@ chunk 8
-- Palette piles for refining the pitcher
p_glaze_hi = pile{{"lead white", 8.5}, {"yellow ochre", 2.2}, {"vermilion", 0.3}, medium = 0.05}
p_terracotta_deep = pile{{"raw umber", 5}, {"red earth", 5.5}, {"bone black", 1.5}, {"yellow ochre", 1}, medium = 0.1}
p_terracotta_warm = pile{{"red earth", 6}, {"yellow ochre", 4.5}, {"vermilion", 1}, {"lead white", 1.5}, {"raw umber", 0.8}, medium = 0.08}
p_rim_hi = pile{{"lead white", 9}, {"yellow ochre", 1.8}, medium = 0.05}
p_jug_darks = pile{{"bone black", 5.5}, {"raw umber", 4.5}, {"red earth", 1}, medium = 0.1}

local b_f6 = brush("filbert", 7)
local b_r3 = brush{kind = "round", width = 3.5, point = 0.85}
local b_r2 = brush{kind = "round", width = 2.2, point = 0.9}

-- 1. Deepen the core shadow down the right side of the belly
b_f6:load(p_terracotta_deep, 0.85)
b_f6:stroke({
  {388, 280}, {385, 330}, {395, 390}, {408, 460}, {412, 520}, {418, 575}
}, {
  pressure = {0.6, 0.9, 0.7},
  swell = {0.8, 1.3, 0.9}
})

-- 2. Reflected warmth along the far right contour
b_f6:load(p_jug_reflect, 0.75)
b_f6:stroke({
  {402, 310}, {418, 350}, {442, 400}, {466, 465}, {464, 520}, {445, 565}, {425, 595}
}, {
  pressure = {0.4, 0.7, 0.5},
  swell = {0.8, 1.2, 0.8}
})

-- 3. Curving throwing rings across the illuminated front belly
b_f6:load(p_terracotta_warm, 0.8)
b_f6:stroke({{342, 305}, {362, 309}, {382, 306}}, {pressure = {0.5, 0.7, 0.4}})
b_f6:stroke({{318, 360}, {348, 368}, {385, 366}}, {pressure = {0.6, 0.8, 0.5}})
b_f6:stroke({{290, 425}, {340, 436}, {390, 435}}, {pressure = {0.7, 0.9, 0.5}})
b_f6:stroke({{280, 495}, {335, 510}, {395, 508}}, {pressure = {0.7, 0.9, 0.5}})
b_f6:stroke({{302, 565}, {350, 575}, {405, 572}}, {pressure = {0.6, 0.8, 0.4}})

-- 4. Clean blender over the body to melt the throwing rings into the terracotta form
local b_badger = brush("badger", 22)
b_badger:stroke({{370, 300}, {375, 450}, {380, 580}}, {pressure = {0.2, 0.35, 0.2}})

-- 5. Thick glaze impasto highlight on shoulder and neck
b_r3:load(p_glaze_hi, 0.95)
b_r3:stroke({{336, 365}, {330, 395}, {322, 435}}, {
  pressure = {0.5, 0.95, 0.3},
  swell = {0.8, 1.4, 0.6}
})
-- Neck highlight
b_r3:load(p_glaze_hi, 0.85)
b_r3:stroke({{348, 288}, {346, 308}, {343, 325}}, {
  pressure = {0.6, 0.85, 0.3}
})

-- 6. Crisp lip rim highlight on mouth
b_r2:load(p_rim_hi, 0.95)
b_r2:stroke({{334, 269}, {346, 275}, {368, 277}, {385, 274}}, {
  pressure = {0.7, 1.0, 0.4}
})

-- 7. Handle top crest highlight
b_r3:load(p_rim_hi, 0.9)
b_r3:stroke({{310, 342}, {280, 360}, {256, 388}, {246, 420}, {249, 445}}, {
  pressure = {0.5, 0.9, 0.4},
  swell = {0.7, 1.2, 0.6}
})

print("Pitcher refinement and highlights applied.")

--@ chunk 9
-- Palette piles for quince details and walnuts
p_quince_blush = pile{{"red earth", 4.5}, {"yellow ochre", 4}, {"raw umber", 1.5}, medium = 0.2}
p_walnut_meat  = pile{{"lead white", 7}, {"yellow ochre", 4}, {"raw umber", 1.5}, medium = 0.08}
p_walnut_shell = pile{{"raw umber", 6}, {"red earth", 3}, {"yellow ochre", 3}, {"bone black", 1}, medium = 0.1}
p_walnut_hi    = pile{{"lead white", 8.5}, {"yellow ochre", 2.5}, medium = 0.06}
p_walnut_dark  = pile{{"raw umber", 7}, {"bone black", 4.5}, medium = 0.1}

local b_f9 = brush("filbert", 9)
local b_r3 = brush{kind = "round", width = 3, point = 0.85}
local b_r1 = brush{kind = "round", width = 1.8, point = 0.95}

-- 1. Soften pitcher body rings into a cohesive terracotta surface
b_f9:load(p_terracotta_warm, 0.5)
b_f9:stroke({{310, 410}, {350, 425}, {390, 420}}, {pressure = {0.3, 0.45, 0.2}})
b_f9:stroke({{305, 480}, {355, 495}, {400, 490}}, {pressure = {0.3, 0.45, 0.2}})
b_f9:stroke({{315, 545}, {360, 555}, {410, 550}}, {pressure = {0.3, 0.45, 0.2}})

-- 2. Quince russet blush
local m_q_blush = poly({{575, 530}, {615, 530}, {630, 565}, {615, 605}, {575, 580}}, true)
work(m_q_blush, {
  hand = "scumble",
  pile = p_quince_blush,
  coverage = 1.2,
  edge = "soft"
})

-- 3. Quince primary impasto highlight on the knobby belly
b_r3:load(p_quince_hi, 0.95)
b_r3:stroke({{590, 550}, {595, 565}, {598, 580}}, {
  pressure = {0.6, 0.95, 0.4},
  swell = {0.8, 1.4, 0.7}
})
-- Secondary softer highlight on upper lobe
b_r3:load(p_quince_hi, 0.8)
b_r3:stroke({{612, 510}, {618, 524}}, {pressure = {0.5, 0.8, 0.3}})

-- Stem lit facet
b_r1:load(p_quince_body, 0.85)
b_r1:stroke({{632, 485}, {635, 470}, {640, 456}, {645, 447}}, {pressure = {0.4, 0.6, 0.3}})

-- 4. FOREGROUND WALNUTS
-- A. Whole walnut (x~535, y~662)
-- Cast shadow
local b_sh = brush{kind = "round", width = 5, point = 0.5}
b_sh:load(p_stone_shadow, 0.8)
b_sh:stroke({{535, 674}, {555, 678}, {575, 676}}, {pressure = {0.5, 0.7, 0.3}})

-- Solid shell body
local pts_walnut_whole = {
  {520, 655}, {535, 646}, {552, 650}, {560, 665}, {550, 676}, {530, 676}, {518, 666}
}
local m_wn_whole = poly(pts_walnut_whole, true)
work(m_wn_whole, {hand = "body", pile = p_walnut_shell, coverage = 2.5, fill = true, edge = "firm"})

-- Shell ridge and suture line
b_r1:load(p_walnut_dark, 0.9)
b_r1:stroke({{536, 647}, {538, 660}, {540, 675}}, {pressure = {0.4, 0.7, 0.4}})
-- Lit ridges on whole walnut shell
b_r1:load(p_walnut_hi, 0.85)
b_r1:stroke({{526, 654}, {532, 662}, {534, 672}}, {pressure = {0.5, 0.8, 0.3}})
b_r1:stroke({{544, 652}, {548, 664}, {547, 673}}, {pressure = {0.4, 0.7, 0.3}})

-- B. Cracked walnut half (x~480, y~650)
-- Cast shadow
b_sh:stroke({{478, 664}, {500, 668}, {520, 666}}, {pressure = {0.5, 0.7, 0.3}})

-- Outer shell rim
local pts_wn_half = {
  {465, 642}, {485, 635}, {502, 644}, {505, 658}, {490, 668}, {470, 664}, {462, 652}
}
local m_wn_half = poly(pts_wn_half, true)
work(m_wn_half, {hand = "body", pile = p_walnut_dark, coverage = 2.5, fill = true, edge = "firm"})

-- Woody rim of the shell half
b_r1:load(p_walnut_shell, 0.9)
b_r1:stroke({
  {465, 642}, {485, 635}, {502, 644}, {505, 658}, {490, 668}, {470, 664}, {462, 652}, {465, 642}
}, {pressure = 0.6})

-- Lit shell rim edge (catching light on upper-left edge)
b_r1:load(p_walnut_hi, 0.9)
b_r1:stroke({{464, 646}, {472, 638}, {488, 636}}, {pressure = {0.6, 0.8, 0.4}})

-- Walnut meat (creamy convoluted kernel folds)
b_r1:load(p_walnut_meat, 0.95)
-- Left kernel lobe
b_r1:stroke({{473, 646}, {477, 644}, {480, 652}, {478, 660}, {474, 656}}, {pressure = {0.6, 0.8, 0.5}})
-- Right kernel lobe
b_r1:stroke({{485, 644}, {492, 643}, {496, 650}, {492, 658}, {486, 656}}, {pressure = {0.6, 0.8, 0.5}})
-- Central kernel divider & highlights
b_r1:load(p_walnut_hi, 0.9)
b_r1:touch(476, 646, {pressure = 0.7})
b_r1:touch(490, 645, {pressure = 0.7})
b_r1:touch(480, 654, {pressure = 0.6})

-- C. Small broken shell chip
b_r1:load(p_walnut_shell, 0.8)
b_r1:stroke({{442, 660}, {448, 663}, {452, 661}}, {pressure = 0.6})
b_r1:load(p_walnut_hi, 0.8)
b_r1:touch(445, 660, {pressure = 0.5})

print("Quince refinement and foreground walnuts complete.")

--@ chunk 10
-- 1. Unify the pitcher belly: vertical, sweeping filbert strokes to melt the stripes
local b_f14 = brush("filbert", 14)
b_f14:load(p_jug_lit, 0.7)
b_f14:stroke({{325, 340}, {315, 410}, {310, 480}, {318, 560}}, {pressure = {0.4, 0.7, 0.4}})
b_f14:stroke({{345, 330}, {338, 410}, {335, 490}, {342, 570}}, {pressure = {0.4, 0.7, 0.4}})

b_f14:load(p_jug_mid, 0.7)
b_f14:stroke({{365, 330}, {362, 420}, {365, 500}, {372, 570}}, {pressure = {0.4, 0.7, 0.4}})
b_f14:stroke({{385, 335}, {385, 430}, {390, 510}, {400, 570}}, {pressure = {0.4, 0.7, 0.4}})

-- Blend the belly vertically and across
local b_blend = brush("badger", 30)
b_blend:stroke({{355, 320}, {355, 450}, {360, 580}}, {pressure = {0.2, 0.4, 0.2}})
b_blend:stroke({{310, 460}, {370, 470}, {430, 460}}, {pressure = {0.2, 0.35, 0.2}})

-- Re-touch the shoulder highlight crisply
local b_hi = brush{kind = "round", width = 3.5, point = 0.85}
b_hi:load(p_glaze_hi, 0.95)
b_hi:stroke({{332, 370}, {326, 400}, {320, 435}}, {
  pressure = {0.6, 0.95, 0.3},
  swell = {0.8, 1.4, 0.6}
})

-- 2. Unify the Quince: blend out the blush patch into the golden flesh
local b_q = brush("filbert", 10)
b_q:load(p_quince_lit, 0.7)
b_q:stroke({{580, 525}, {595, 560}, {590, 605}}, {pressure = {0.5, 0.8, 0.4}})
b_q:load(p_quince_body, 0.65)
b_q:stroke({{610, 520}, {620, 560}, {615, 610}}, {pressure = {0.5, 0.7, 0.4}})

-- Soft badger blend across the quince lobe
local b_badger_sm = brush("badger", 16)
b_badger_sm:stroke({{595, 520}, {605, 565}, {600, 615}}, {pressure = {0.2, 0.35, 0.2}})

-- Re-apply luminous impasto highlight on the crest of the quince
b_hi:load(p_quince_hi, 0.95)
b_hi:stroke({{588, 552}, {594, 565}, {597, 578}}, {
  pressure = {0.6, 1.0, 0.4},
  swell = {0.8, 1.4, 0.7}
})

-- 3. Repaint tabletop under walnut area to create a clean, integrated stone base
local m_table_patch = poly({{430, 625}, {585, 625}, {585, 695}, {430, 695}}, true)
work(m_table_patch, {
  hand = "body",
  pile = p_stone_lit,
  angle = 0.0,
  coverage = 2.4,
  fill = true,
  edge = "soft"
})
-- Table shadow from jug in that patch
local m_patch_shadow = poly({{430, 625}, {560, 625}, {565, 642}, {510, 650}, {440, 642}}, true)
work(m_patch_shadow, {
  hand = "body",
  pile = p_stone_shadow,
  angle = 0.0,
  coverage = 1.8,
  edge = "soft"
})
-- Smooth horizontal blend of the patch into the surrounding table
b_blend:stroke({{420, 660}, {500, 660}, {600, 660}}, {pressure = {0.25, 0.35, 0.25}})

print("Pitcher unified, quince blush integrated, table patch repainted.")

--@ chunk 11
-- 1. REPAINT THE TABLE PLANE
local table_back = {{0, 520}, {250, 522}, {500, 518}, {750, 520}, {1000, 516}}
local table_front = {{0, 715}, {260, 712}, {520, 714}, {780, 710}, {1000, 712}}
local m_below_back = below(table_back)
local m_above_front = above(table_front)
local m_table_plane = m_below_back * m_above_front

-- Leave small margins inside jug and quince so table meets their edges
local pts_jug_base = {{315, 560}, {435, 560}, {430, 606}, {315, 606}}
local m_jug_t = poly(pts_jug_base, true):shrink(2)
local pts_q_base = {{580, 550}, {680, 550}, {685, 668}, {595, 674}, {565, 630}}
local m_q_t = poly(pts_q_base, true):shrink(2)

local m_table_clean = m_table_plane - m_jug_t - m_q_t

local m_tab_l = m_table_clean * mask(function(x, y) return math.max(0, 1 - (x / 700)) end)
local m_tab_r = m_table_clean * mask(function(x, y) return math.min(1, x / 700) end)

work(m_tab_l, {hand = "body", pile = p_stone_lit, angle = 0.0, coverage = 2.6, fill = true, edge = "soft"})
work(m_tab_r, {hand = "body", pile = p_stone_mid, angle = 0.0, coverage = 2.6, fill = true, edge = "soft"})

-- Cast shadows on table
local m_cast_jug = poly({{370, 605}, {420, 600}, {490, 610}, {560, 625}, {565, 640}, {520, 648}, {450, 642}, {380, 618}}, true)
local m_cast_quince = poly({{635, 672}, {680, 676}, {735, 685}, {745, 695}, {715, 702}, {660, 696}, {605, 678}}, true)

work(m_cast_jug * m_table_plane, {hand = "body", pile = p_stone_shadow, angle = 0.05, coverage = 2.0, edge = "soft"})
work(m_cast_quince * m_table_plane, {hand = "body", pile = p_stone_shadow, angle = 0.05, coverage = 2.0, edge = "soft"})

-- Blend table surface horizontally
blend(m_table_plane, {angle = 0})

-- Raking highlight along the front ledge edge
p_stone_edge = pile{{"lead white", 9}, {"yellow ochre", 1.8}, {"raw umber", 0.5}, medium = 0.05}
local b_edge = brush{kind = "round", width = 3, point = 0.85}
b_edge:load(p_stone_edge, 0.95)
b_edge:stroke({{0, 715}, {260, 712}, {520, 714}, {780, 710}, {1000, 712}}, {
  pressure = {0.7, 0.85, 0.6},
  shake = 0.5
})

-- 2. REPAINT THE EARTHENWARE PITCHER IN OPAQUE BODY
local pts_jug_body = {
  {335, 270}, {338, 310}, {330, 340}, {305, 380}, {280, 430}, {270, 480},
  {276, 530}, {295, 570}, {320, 604}, {345, 608}, {375, 610}, {405, 608},
  {425, 604}, {450, 570}, {468, 530}, {472, 480}, {460, 430}, {435, 380},
  {412, 340}, {402, 310}, {405, 270}, {385, 262}, {355, 262}
}
local m_jug_solid = poly(pts_jug_body, true)

local m_j_shadow = m_jug_solid * mask(function(x, y)
  local term_x = 375 + (y - 350) * 0.12
  return smoothstep(term_x - 10, term_x + 10, x)
end)
local m_j_lit = m_jug_solid - m_j_shadow

work(m_j_lit, {hand = "body", pile = p_jug_lit, angle = 1.45, coverage = 2.8, fill = true, edge = "firm"})
work(m_j_shadow, {hand = "body", pile = p_terracotta_deep, angle = 1.45, coverage = 2.8, fill = true, edge = "firm"})

-- Reflected bounce on shadow flank
local m_j_ref = m_j_shadow * mask(function(x, y) return smoothstep(435, 468, x) end)
work(m_j_ref, {hand = "body", pile = p_jug_reflect, angle = 1.4, coverage = 2.2, fill = true, edge = "soft"})

-- Blend across the terminator for smooth roundness
blend(m_jug_solid, {angle = 0})

-- Impasto shoulder glaze highlight
local b_hi = brush{kind = "round", width = 3.5, point = 0.85}
b_hi:load(p_glaze_hi, 0.95)
b_hi:stroke({{334, 370}, {328, 400}, {322, 435}}, {
  pressure = {0.6, 0.95, 0.3},
  swell = {0.8, 1.4, 0.6}
})
-- Neck highlight & lip rim
b_hi:stroke({{346, 288}, {344, 308}, {342, 325}}, {pressure = {0.6, 0.85, 0.3}})
local b_r2 = brush{kind = "round", width = 2.2, point = 0.9}
b_r2:load(p_rim_hi, 0.95)
b_r2:stroke({{334, 269}, {346, 275}, {368, 277}, {385, 274}}, {pressure = {0.7, 1.0, 0.4}})

-- Contact shadow under base
local b_d = brush{kind = "round", width = 4, point = 0.9}
b_d:load(p_jug_darks, 0.9)
b_d:stroke({{315, 604}, {345, 608}, {375, 610}, {405, 608}, {428, 604}}, {pressure = {0.8, 1.0, 0.7}})

-- 3. REPAINT THE QUINCE IN OPAQUE BODY
local pts_quince = {
  {630, 485}, {648, 500}, {658, 525}, {678, 555}, {692, 590}, {690, 630},
  {672, 660}, {640, 672}, {605, 670}, {575, 652}, {560, 625}, {562, 590},
  {578, 555}, {600, 525}, {615, 500}
}
local m_quince = poly(pts_quince, true)

local m_q_sh = m_quince * mask(function(x, y)
  local term_x = 638 + (y - 550) * 0.1
  return smoothstep(term_x - 12, term_x + 12, x)
end)
local m_q_lt = m_quince - m_q_sh

work(m_q_lt, {hand = "body", pile = p_quince_body, angle = 1.3, coverage = 2.8, fill = true, edge = "firm"})
work(m_q_sh, {hand = "body", pile = p_quince_core, angle = 1.2, coverage = 2.8, fill = true, edge = "firm"})

-- Luminous crest on lit lobe
local m_q_cr = m_q_lt * mask(function(x, y)
  local dist = math.sqrt((x - 595)^2 + (y - 560)^2)
  return smoothstep(45, 10, dist)
end)
work(m_q_cr, {hand = "body", pile = p_quince_lit, angle = 1.2, coverage = 2.4, fill = true, edge = "soft"})

-- Blend across the quince
blend(m_quince, {angle = 0.2})

-- Quince impasto highlight
b_hi:load(p_quince_hi, 0.95)
b_hi:stroke({{590, 550}, {595, 565}, {598, 578}}, {
  pressure = {0.6, 1.0, 0.4},
  swell = {0.8, 1.4, 0.7}
})
-- Woody stem
b_d:load(p_stem, 0.9)
b_d:stroke({{631, 488}, {634, 470}, {639, 455}, {645, 446}}, {pressure = {0.8, 0.6, 0.4}})
b_d:load(p_jug_darks, 0.8)
b_d:touch(631, 487, {pressure = 0.7})
b_d:stroke({{600, 670}, {635, 674}, {665, 665}}, {pressure = {0.8, 1.0, 0.7}})

print("Consolidation complete.")

--@ chunk 12
-- Palette piles for foreground elements: walnuts and knife
p_steel_edge     = pile{{"lead white", 9}, {"bone black", 0.6}, {"cobalt blue", 0.4}, {"raw umber", 0.2}, medium = 0.05}
p_steel_body     = pile{{"lead white", 5.5}, {"bone black", 3}, {"cobalt blue", 1.2}, {"raw umber", 1}, medium = 0.1}
p_steel_shadow   = pile{{"bone black", 4.5}, {"raw umber", 4}, {"cobalt blue", 0.8}, medium = 0.12}
p_knife_handle   = pile{{"bone black", 5.5}, {"raw umber", 4}, {"red earth", 1.5}, medium = 0.08}
p_knife_handle_hi= pile{{"raw umber", 4}, {"yellow ochre", 4}, {"lead white", 3.5}, {"red earth", 1}, medium = 0.08}
p_brass          = pile{{"yellow ochre", 6}, {"lead white", 3.5}, {"raw umber", 1.2}, medium = 0.08}

p_nut_meat       = pile{{"lead white", 7.5}, {"yellow ochre", 3.8}, {"raw umber", 1.2}, medium = 0.08}
p_nut_shell      = pile{{"raw umber", 5}, {"red earth", 3.5}, {"yellow ochre", 3.2}, {"bone black", 1}, medium = 0.1}
p_nut_shadow     = pile{{"raw umber", 6.5}, {"bone black", 4}, medium = 0.12}
p_nut_lit        = pile{{"yellow ochre", 5.5}, {"raw umber", 3}, {"lead white", 3.5}, {"red earth", 1}, medium = 0.08}

local b_r2 = brush{kind = "round", width = 2.2, point = 0.92}
local b_r1 = brush{kind = "round", width = 1.4, point = 0.98}
local b_sh = brush{kind = "round", width = 4, point = 0.6}

-- 1. WALNUTS
-- Cast shadows on table
b_sh:load(p_nut_shadow, 0.7)
b_sh:stroke({{468, 654}, {486, 658}, {504, 654}}, {pressure = {0.4, 0.6, 0.3}})
b_sh:stroke({{518, 668}, {538, 673}, {556, 668}}, {pressure = {0.4, 0.6, 0.3}})

-- Split walnut half (x~482, y~646)
-- Outer woody shell cup
local pts_wn_cup = {{468, 640}, {464, 648}, {470, 656}, {485, 658}, {498, 653}, {496, 642}}
b_r2:load(p_nut_shell, 0.9)
b_r2:stroke(pts_wn_cup, {pressure = 0.7})

-- Dark interior cavity
local m_wn_in = poly(pts_wn_cup, true)
work(m_wn_in, {hand = "body", pile = p_nut_shadow, coverage = 2.0, fill = true, edge = "firm"})

-- Kernel lobes: creamy, convoluted folds
b_r1:load(p_nut_meat, 0.95)
-- Left lobe curlicues
b_r1:stroke({{472, 643}, {475, 648}, {473, 653}}, {pressure = {0.6, 0.8, 0.5}})
b_r1:stroke({{476, 644}, {478, 650}, {477, 654}}, {pressure = {0.6, 0.8, 0.4}})
-- Right lobe curlicues
b_r1:stroke({{483, 642}, {486, 647}, {484, 653}}, {pressure = {0.6, 0.8, 0.5}})
b_r1:stroke({{488, 643}, {492, 648}, {489, 653}}, {pressure = {0.6, 0.8, 0.4}})
-- Impasto touch on kernel crest
b_r1:load(p_stone_edge, 0.9)
b_r1:touch(476, 645, {pressure = 0.6})
b_r1:touch(485, 644, {pressure = 0.6})
-- Shell rim catching light on top-left edge
b_r1:stroke({{467, 640}, {473, 638}, {484, 637}}, {pressure = {0.5, 0.7, 0.3}})

-- Whole walnut (x~532, y~660)
-- Solid body
local pts_wn_w = {{518, 655}, {532, 647}, {548, 651}, {555, 663}, {546, 672}, {528, 672}, {516, 663}}
local m_wn_w = poly(pts_wn_w, true)
work(m_wn_w, {hand = "body", pile = p_nut_lit, coverage = 2.4, fill = true, edge = "firm"})

-- Shadow side of whole nut
local m_wn_w_sh = m_wn_w * mask(function(x, y) return (x - 530) * 0.7 + (y - 660) * 0.7 > 2 and 1 or 0 end)
work(m_wn_w_sh, {hand = "body", pile = p_nut_shadow, coverage = 2.0, fill = true, edge = "soft"})

-- Suture line and ribbed shell texture
b_r1:load(p_nut_shadow, 0.85)
b_r1:stroke({{532, 647}, {534, 660}, {536, 672}}, {pressure = {0.4, 0.7, 0.3}})
-- Lit furrow ridges
b_r1:load(p_stone_edge, 0.8)
b_r1:stroke({{523, 653}, {528, 661}, {529, 669}}, {pressure = {0.4, 0.6, 0.3}})
b_r1:stroke({{541, 651}, {544, 661}, {542, 668}}, {pressure = {0.3, 0.5, 0.2}})

-- Small broken shell chip
b_r1:load(p_nut_shell, 0.8)
b_r1:stroke({{446, 656}, {451, 659}, {454, 657}}, {pressure = 0.6})
b_r1:load(p_stone_edge, 0.8)
b_r1:touch(448, 657, {pressure = 0.5})

-- 2. THE OLD KNIFE
-- Cast shadow on the tabletop under blade
b_sh:load(p_stone_shadow, 0.8)
b_sh:stroke({{700, 683}, {750, 700}, {798, 718}}, {pressure = {0.3, 0.6, 0.4}})

-- Cast shadow of handle down the vertical face of the stone ledge
b_sh:load(p_stone_shadow, 0.85)
b_sh:stroke({{805, 718}, {815, 745}, {825, 775}}, {pressure = {0.5, 0.7, 0.4}})
b_sh:stroke({{845, 735}, {855, 765}, {865, 790}}, {pressure = {0.4, 0.6, 0.3}})

-- Blade polygon
local pts_blade = {{695, 680}, {796, 711}, {796, 716}, {695, 682}}
local m_blade = poly(pts_blade, true)
work(m_blade, {hand = "body", pile = p_steel_body, coverage = 2.5, fill = true, edge = "firm"})

-- Blade lower bevel shadow
b_r1:load(p_steel_shadow, 0.9)
b_r1:stroke({{696, 682}, {745, 698}, {796, 716}}, {pressure = {0.4, 0.7, 0.5}})

-- Sharp razor highlight on upper bevel of blade
b_r1:load(p_steel_edge, 0.98)
b_r1:stroke({{695, 680}, {745, 696}, {796, 711}}, {
  pressure = {0.5, 0.9, 0.6},
  swell = {0.8, 1.2, 0.8}
})

-- Brass bolster (collar)
local pts_bolster = {{796, 710}, {805, 713}, {804, 718}, {795, 716}}
local m_bolster = poly(pts_bolster, true)
work(m_bolster, {hand = "body", pile = p_brass, coverage = 2.5, fill = true, edge = "firm"})
b_r1:load(p_stone_edge, 0.9)
b_r1:stroke({{797, 711}, {804, 713}}, {pressure = 0.7})

-- Knife handle (turned dark wood/horn extending past ledge edge)
local pts_handle = {
  {804, 713}, {835, 725}, {868, 739}, {885, 747},
  {882, 756}, {864, 748}, {832, 734}, {803, 719}
}
local m_handle = poly(pts_handle, true)
work(m_handle, {hand = "body", pile = p_knife_handle, coverage = 2.6, fill = true, edge = "firm"})

-- Cylindrical specular highlight along top of handle
b_r2:load(p_knife_handle_hi, 0.9)
b_r2:stroke({{806, 714}, {835, 726}, {865, 740}, {880, 748}}, {
  pressure = {0.4, 0.75, 0.3},
  swell = {0.7, 1.1, 0.7}
})

-- Small bright rivet dots on handle
b_r1:load(p_brass, 0.9)
b_r1:touch(825, 725, {pressure = 0.6})
b_r1:touch(855, 738, {pressure = 0.6})

print("Walnuts and knife completed.")

--@ chunk 13
-- Piles for walnuts and stone face texture
p_kernel_pure = pile{{"lead white", 8.5}, {"yellow ochre", 3}, {"raw umber", 0.8}, medium = 0.04}
p_kernel_hi   = pile{{"lead white", 9.5}, {"yellow ochre", 1.2}, medium = 0.04}
p_shell_wood  = pile{{"red earth", 4.5}, {"yellow ochre", 4}, {"raw umber", 3}, {"lead white", 1.2}, medium = 0.06}
p_shell_shade = pile{{"raw umber", 6.5}, {"red earth", 3}, {"bone black", 2}, medium = 0.08}

local b_f10 = brush("filbert", 10)
local b_r2 = brush{kind = "round", width = 2.4, point = 0.9}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}
local b_sh = brush{kind = "round", width = 4.5, point = 0.6}

-- 1. Wipe out old muddy marks with solid stone tabletop paint
b_f10:load(p_stone_lit, 0.9)
b_f10:stroke({{450, 652}, {500, 652}, {570, 652}}, {pressure = {0.6, 0.9, 0.6}})
b_f10:stroke({{450, 665}, {500, 665}, {570, 665}}, {pressure = {0.6, 0.9, 0.6}})

-- Soft horizontal blend of the fresh stone
local b_b16 = brush("badger", 16)
b_b16:stroke({{440, 658}, {510, 658}, {580, 658}}, {pressure = {0.2, 0.35, 0.2}})

-- 2. CRACKED WALNUT HALF (x~482, y~648)
-- Cast shadow under walnut onto stone
b_sh:load(p_stone_shadow, 0.75)
b_sh:stroke({{472, 656}, {488, 660}, {505, 656}}, {pressure = {0.4, 0.7, 0.3}})

-- Outer woody shell bowl
local pts_wn_bowl = {
  {468, 641}, {464, 649}, {470, 657}, {485, 659}, {499, 655}, {498, 644}
}
b_r2:load(p_shell_wood, 0.95)
b_r2:stroke(pts_wn_bowl, {pressure = 0.75})

-- Solid kernel meat: chunky, creamy, opaque lobes
b_r2:load(p_kernel_pure, 1.0)
-- Left lobe
b_r2:stroke({{473, 643}, {475, 648}, {474, 654}}, {pressure = {0.7, 0.95, 0.6}})
b_r2:stroke({{477, 644}, {479, 649}, {478, 654}}, {pressure = {0.6, 0.9, 0.5}})
-- Right lobe
b_r2:stroke({{483, 643}, {485, 648}, {484, 654}}, {pressure = {0.7, 0.95, 0.6}})
b_r2:stroke({{488, 644}, {491, 649}, {489, 654}}, {pressure = {0.6, 0.9, 0.5}})

-- Central crevice between lobes
b_r1:load(p_shell_shade, 0.9)
b_r1:stroke({{481, 642}, {482, 649}, {481, 655}}, {pressure = {0.3, 0.6, 0.3}})
b_r1:stroke({{470, 651}, {472, 656}}, {pressure = 0.4})

-- Kernel crest impasto touches
b_r1:load(p_kernel_hi, 1.0)
b_r1:touch(475, 644, {pressure = 0.75})
b_r1:touch(485, 643, {pressure = 0.75})
b_r1:touch(479, 650, {pressure = 0.65})
b_r1:touch(489, 649, {pressure = 0.65})

-- Bright shell rim catching light on top-left
b_r1:load(p_stone_edge, 0.95)
b_r1:stroke({{467, 641}, {473, 638}, {484, 637}}, {pressure = {0.5, 0.85, 0.4}})

-- 3. WHOLE WALNUT (x~534, y~660)
-- Cast shadow
b_sh:load(p_stone_shadow, 0.75)
b_sh:stroke({{522, 668}, {542, 673}, {562, 668}}, {pressure = {0.4, 0.7, 0.3}})

-- Solid nut body
local pts_wn_w = {
  {520, 655}, {532, 647}, {546, 650}, {554, 661}, {546, 670}, {528, 670}, {518, 661}
}
local m_wn_w = poly(pts_wn_w, true)
work(m_wn_w, {hand = "body", pile = p_nut_lit, coverage = 2.6, fill = true, edge = "firm"})

-- Shaded flank
b_r2:load(p_shell_shade, 0.85)
b_r2:stroke({{538, 651}, {548, 660}, {545, 668}}, {pressure = {0.5, 0.85, 0.4}})

-- Suture ridge
b_r1:load(p_shell_shade, 0.9)
b_r1:stroke({{532, 647}, {534, 659}, {536, 670}}, {pressure = {0.35, 0.65, 0.35}})

-- Lit corrugated ridges
b_r1:load(p_kernel_hi, 0.85)
b_r1:stroke({{524, 653}, {528, 660}, {529, 668}}, {pressure = {0.4, 0.7, 0.3}})
b_r1:stroke({{541, 652}, {543, 661}, {541, 667}}, {pressure = {0.3, 0.55, 0.25}})

-- 4. DRESSED MASONRY TEXTURE ON FRONT LEDGE FACE
local b_flat = brush("flat", 18)
b_flat:load(p_stone_mid, 0.3)
-- Vertical chisel striations down the stone face
b_flat:stroke({{180, 722}, {180, 785}}, {pressure = {0.2, 0.35, 0.15}})
b_flat:stroke({{360, 722}, {360, 785}}, {pressure = {0.2, 0.3, 0.15}})
b_flat:stroke({{620, 722}, {620, 785}}, {pressure = {0.2, 0.35, 0.15}})
b_flat:stroke({{820, 722}, {820, 785}}, {pressure = {0.2, 0.3, 0.15}})

print("Walnuts reworked with tactile impasto; masonry texture added.")

--@ chunk 14
local t = wait(3 * 24 * 60)
print("Time passed to: " .. tostring(t))
print("Drying at (480, 650): " .. tostring(drying(480, 650)))
print("Drying at (350, 450): " .. tostring(drying(350, 450)))

--@ chunk 15
local t = wait(4 * 24 * 60)
print("Time passed to: " .. tostring(t))
print("Drying at (480, 650): " .. tostring(drying(480, 650)))
print("Drying at (350, 450): " .. tostring(drying(350, 450)))

--@ chunk 16
-- Re-establish the clean stone tabletop over the dry underlayer
local m_clean_table = poly({{430, 620}, {575, 620}, {575, 690}, {430, 690}}, true)

work(m_clean_table, {
  hand = "body",
  pile = p_stone_lit,
  angle = 0.0,
  coverage = 2.8,
  fill = true,
  edge = "soft"
})

-- Re-apply table cast shadow from jug in that area, soft and transparent
local m_soft_shadow = poly({{430, 620}, {560, 620}, {560, 638}, {500, 646}, {430, 638}}, true)
work(m_soft_shadow, {
  hand = "body",
  pile = p_stone_shadow,
  angle = 0.0,
  coverage = 1.6,
  edge = "soft"
})

-- Horizontal blend to feather into the dry table
local b_badger = brush("badger", 30)
b_badger:stroke({{420, 655}, {505, 655}, {585, 655}}, {pressure = {0.2, 0.35, 0.2}})

print("Tabletop restored cleanly over dry paint.")

--@ chunk 17
-- 1. UNIFY THE VERTICAL FRONT LEDGE FACE
local table_front = {{0, 715}, {260, 712}, {520, 714}, {780, 710}, {1000, 712}}
local m_below_front = below(table_front)

local pts_knife = {
  {690, 675}, {795, 708}, {890, 738}, {890, 765}, {800, 725}, {690, 685}
}
local m_knife_prot = poly(pts_knife, true):grow(2)

-- Front face excluding knife
local m_front_work = m_below_front - m_knife_prot
work(m_front_work, {
  hand = "body",
  pile = p_ledge_face,
  angle = 0.0,
  coverage = 2.6,
  fill = true,
  edge = "firm"
})

-- Re-apply knife handle shadow down the front face
local b_sh = brush{kind = "round", width = 5, point = 0.6}
b_sh:load(p_stone_shadow, 0.85)
b_sh:stroke({{805, 718}, {815, 745}, {825, 775}}, {pressure = {0.5, 0.7, 0.4}})
b_sh:stroke({{845, 735}, {855, 765}, {865, 790}}, {pressure = {0.4, 0.6, 0.3}})

-- 2. UNIFY THE HORIZONTAL TABLETOP PLANE
local table_back = {{0, 520}, {250, 522}, {500, 518}, {750, 520}, {1000, 516}}
local m_below_back = below(table_back)
local m_above_front = above(table_front)
local m_table_plane = m_below_back * m_above_front

local pts_jug_body = {
  {335, 270}, {338, 310}, {330, 340}, {305, 380}, {280, 430}, {270, 480},
  {276, 530}, {295, 570}, {320, 604}, {345, 608}, {375, 610}, {405, 608},
  {425, 604}, {450, 570}, {468, 530}, {472, 480}, {460, 430}, {435, 380},
  {412, 340}, {402, 310}, {405, 270}, {385, 262}, {355, 262}
}
local m_jug_prot = poly(pts_jug_body, true):shrink(2)

local pts_quince = {
  {630, 485}, {648, 500}, {658, 525}, {678, 555}, {692, 590}, {690, 630},
  {672, 660}, {640, 672}, {605, 670}, {575, 652}, {560, 625}, {562, 590},
  {578, 555}, {600, 525}, {615, 500}
}
local m_quince_prot = poly(pts_quince, true):shrink(2)

local m_table_work = m_table_plane - m_jug_prot - m_quince_prot - m_knife_prot

local m_tab_l = m_table_work * mask(function(x, y) return math.max(0, 1 - (x / 720)) end)
local m_tab_r = m_table_work * mask(function(x, y) return math.min(1, x / 720) end)

work(m_tab_l, {hand = "body", pile = p_stone_lit, angle = 0.0, coverage = 2.6, fill = true, edge = "soft"})
work(m_tab_r, {hand = "body", pile = p_stone_mid, angle = 0.0, coverage = 2.6, fill = true, edge = "soft"})

-- Cast shadows from pitcher and quince
local m_cast_jug = poly({{370, 605}, {420, 600}, {480, 610}, {535, 625}, {540, 640}, {490, 646}, {430, 638}, {380, 618}}, true)
local m_cast_quince = poly({{635, 672}, {680, 676}, {735, 685}, {745, 695}, {715, 702}, {660, 696}, {605, 678}}, true)

work(m_cast_jug * m_table_work, {hand = "body", pile = p_stone_shadow, angle = 0.05, coverage = 1.8, edge = "soft"})
work(m_cast_quince * m_table_work, {hand = "body", pile = p_stone_shadow, angle = 0.05, coverage = 1.8, edge = "soft"})

-- Blend table surface horizontally
blend(m_table_work, {angle = 0})

-- Raking highlight along the front ledge edge (skipping knife)
local b_edge = brush{kind = "round", width = 3, point = 0.85}
b_edge:load(p_stone_edge, 0.95)
b_edge:stroke({{0, 715}, {260, 712}, {520, 714}, {780, 710}}, {pressure = {0.7, 0.85, 0.6}, shake = 0.5})
b_edge:stroke({{890, 711}, {1000, 712}}, {pressure = {0.6, 0.75, 0.5}, shake = 0.5})

-- Contact shadow under pitcher base
local b_d = brush{kind = "round", width = 4, point = 0.9}
b_d:load(p_jug_darks, 0.9)
b_d:stroke({{315, 604}, {345, 608}, {375, 610}, {405, 608}, {428, 604}}, {pressure = {0.8, 1.0, 0.7}})

-- Contact shadow under quince
b_d:stroke({{600, 670}, {635, 674}, {665, 665}}, {pressure = {0.8, 1.0, 0.7}})

print("Ledge face and tabletop unified.")

--@ chunk 18
-- Touch in the quince base gap
local b_r2 = brush{kind = "round", width = 3, point = 0.85}
b_r2:load(p_quince_core, 0.9)
b_r2:stroke({{568, 645}, {574, 658}, {585, 668}}, {pressure = {0.5, 0.8, 0.4}})

-- Palette piles for the ripe black fig
p_fig_skin  = pile{{"bone black", 6}, {"Prussian blue", 2.5}, {"red earth", 2.5}, {"raw umber", 1.5}, medium = 0.08}
p_fig_dark  = pile{{"bone black", 7}, {"raw umber", 3}, {"red earth", 1.5}, medium = 0.1}
p_fig_bloom = pile{{"lead white", 6}, {"pale smalt", 3.5}, {"cobalt blue", 1}, {"red earth", 0.5}, {"bone black", 0.5}, medium = 0.15}
p_fig_flesh = pile{{"vermilion", 6}, {"red earth", 3}, {"yellow ochre", 1.5}, {"lead white", 1}, medium = 0.08}
p_fig_seeds = pile{{"lead white", 7}, {"yellow ochre", 4}, medium = 0.05}
p_fig_hi    = pile{{"lead white", 9}, {"cobalt blue", 0.5}, medium = 0.05}
p_fig_stem  = pile{{"raw umber", 6}, {"green earth", 3}, {"yellow ochre", 2}, medium = 0.1}

local b_r1 = brush{kind = "round", width = 1.4, point = 0.98}
local b_sh = brush{kind = "round", width = 5, point = 0.6}

-- Cast shadow under the fig on the stone
b_sh:load(p_stone_shadow, 0.8)
b_sh:stroke({{470, 667}, {496, 672}, {520, 668}}, {pressure = {0.5, 0.75, 0.4}})

-- Solid fig body
local pts_fig = {
  {485, 626}, {476, 634}, {466, 646}, {465, 658}, {474, 668},
  {490, 670}, {505, 666}, {508, 654}, {502, 642}, {494, 632}
}
local m_fig = poly(pts_fig, true)
work(m_fig, {hand = "body", pile = p_fig_skin, angle = 1.2, coverage = 2.6, fill = true, edge = "firm"})

-- Shadow side of the fig (right flank)
b_r2:load(p_fig_dark, 0.9)
b_r2:stroke({{490, 632}, {504, 644}, {507, 658}, {495, 668}}, {
  pressure = {0.4, 0.8, 0.5},
  swell = {0.8, 1.2, 0.8}
})

-- Waxy dusty bloom scumble across the illuminated shoulder
local m_fig_bloom = poly({{474, 634}, {466, 646}, {468, 658}, {485, 656}, {486, 638}}, true)
work(m_fig_bloom, {hand = "scumble", pile = p_fig_bloom, coverage = 1.2, edge = "soft"})

-- Ripe ruby fissure/split on the front
b_r1:load(p_fig_flesh, 0.95)
b_r1:stroke({{482, 642}, {485, 649}, {484, 657}}, {
  pressure = {0.4, 0.85, 0.3},
  swell = {0.7, 1.4, 0.7}
})

-- Tiny glistening seeds inside the split
b_r1:load(p_fig_seeds, 0.95)
b_r1:touch(483, 646, {pressure = 0.55})
b_r1:touch(485, 650, {pressure = 0.55})
b_r1:touch(484, 654, {pressure = 0.45})

-- Delicate specular highlight on taut skin
b_r1:load(p_fig_hi, 0.95)
b_r1:touch(472, 644, {pressure = 0.7})

-- Curved woody stem at apex
b_r1:load(p_fig_stem, 0.9)
b_r1:stroke({{485, 627}, {482, 621}, {477, 619}}, {
  pressure = {0.7, 0.5, 0.3}
})

print("Black fig painted.")

--@ chunk 19
-- Redefine fig piles with proper deep value and chromatic restraint
local p_fig_lit  = pile{{"bone black", 5}, {"cobalt blue", 2.5}, {"red earth", 2}, {"lead white", 1.5}, medium = 0.08}
local p_fig_shad = pile{{"bone black", 6.5}, {"raw umber", 3}, {"red earth", 1.5}, medium = 0.08}

local pts_fig = {
  {485, 626}, {476, 634}, {466, 646}, {465, 658}, {474, 668},
  {490, 670}, {505, 666}, {508, 654}, {502, 642}, {494, 632}
}
local m_fig = poly(pts_fig, true)

-- Lit left vs shadow right
local m_fig_l = m_fig * mask(function(x, y) return x < 486 and 1 or 0 end)
local m_fig_r = m_fig * mask(function(x, y) return x >= 486 and 1 or 0 end)

work(m_fig_l, {hand = "body", pile = p_fig_lit, angle = 1.2, coverage = 2.6, fill = true, edge = "firm"})
work(m_fig_r, {hand = "body", pile = p_fig_shad, angle = 1.2, coverage = 2.6, fill = true, edge = "firm"})

-- Soft blend across the fig volume
local b_badger_sm = brush("badger", 14)
b_badger_sm:stroke({{472, 648}, {486, 650}, {502, 652}}, {pressure = {0.2, 0.35, 0.2}})

-- Ruby split on front belly
local b_r1 = brush{kind = "round", width = 1.5, point = 0.98}
b_r1:load(p_fig_flesh, 0.95)
b_r1:stroke({{483, 644}, {485, 650}, {484, 656}}, {
  pressure = {0.4, 0.85, 0.3},
  swell = {0.7, 1.3, 0.7}
})

-- Golden seed specks
b_r1:load(p_fig_seeds, 0.95)
b_r1:touch(484, 647, {pressure = 0.5})
b_r1:touch(485, 651, {pressure = 0.5})

-- Specular gleam on taut shoulder
b_r1:load(p_fig_hi, 0.95)
b_r1:touch(473, 643, {pressure = 0.65})

-- Stem
b_r1:load(p_fig_stem, 0.9)
b_r1:stroke({{485, 627}, {483, 621}, {478, 619}}, {pressure = {0.7, 0.5, 0.3}})

print("Fig remodeled and unified.")

--@ chunk 20
local b_r2 = brush{kind = "round", width = 2.2, point = 0.9}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}

-- Clean up the outer contour of the fig with its deep skin tone
b_r2:load(p_fig_skin, 0.95)
b_r2:stroke({
  {485, 626}, {476, 634}, {466, 646}, {465, 658}, {474, 668},
  {490, 670}, {505, 666}, {508, 654}, {502, 642}, {494, 632}, {485, 626}
}, {pressure = 0.65})

-- Firm dark contact shadow under the fig base
b_r2:load(p_jug_darks, 0.9)
b_r2:stroke({{468, 668}, {488, 672}, {506, 668}}, {pressure = {0.6, 0.9, 0.5}})

-- Bright specular highlight on the taut curved shoulder
b_r1:load(p_stone_edge, 0.95)
b_r1:stroke({{472, 642}, {475, 646}}, {pressure = {0.7, 0.9, 0.4}})

-- Rich ruby pulp inside the slit
b_r1:load(p_fig_flesh, 1.0)
b_r1:stroke({{483, 645}, {485, 650}, {484, 655}}, {
  pressure = {0.5, 0.9, 0.4},
  swell = {0.8, 1.4, 0.8}
})

-- Golden seed dots
b_r1:load(p_fig_seeds, 1.0)
b_r1:touch(484, 648, {pressure = 0.7})
b_r1:touch(485, 651, {pressure = 0.65})

-- Stem: darken and refine
b_r1:load(p_stem, 0.95)
b_r1:stroke({{485, 627}, {482, 621}, {477, 619}}, {pressure = {0.8, 0.6, 0.4}})

print("Fig contour, contact shadow, and details perfected.")

--@ chunk 21
-- Ensure piles are global and available
p_fig_lit  = pile{{"bone black", 5}, {"cobalt blue", 2.5}, {"red earth", 2}, {"lead white", 1.5}, medium = 0.08}
p_fig_shad = pile{{"bone black", 6.5}, {"raw umber", 3}, {"red earth", 1.5}, medium = 0.08}

-- 1. Cut in from the outside with stone tabletop paint to cleanly define the fig's left flank
local b_f8 = brush("filbert", 8)
b_f8:load(p_stone_lit, 0.95)
b_f8:stroke({{440, 638}, {455, 642}, {465, 646}}, {pressure = {0.6, 0.9, 0.6}})
b_f8:stroke({{440, 650}, {455, 654}, {465, 656}}, {pressure = {0.6, 0.9, 0.6}})
b_f8:stroke({{442, 662}, {456, 665}, {468, 666}}, {pressure = {0.6, 0.9, 0.5}})

-- Horizontal feathering into the table
local b_badger_sm = brush("badger", 14)
b_badger_sm:stroke({{435, 652}, {458, 652}}, {pressure = {0.2, 0.35, 0.2}})

-- 2. Solidly fill the fig's body with velvety dark aubergine
local b_f6 = brush("filbert", 7)
b_f6:load(p_fig_lit, 0.9)
b_f6:stroke({{472, 636}, {475, 648}, {476, 662}}, {pressure = {0.6, 0.9, 0.6}})
b_f6:load(p_fig_shad, 0.9)
b_f6:stroke({{486, 634}, {490, 648}, {492, 662}}, {pressure = {0.6, 0.9, 0.6}})
b_f6:stroke({{498, 638}, {502, 650}, {500, 660}}, {pressure = {0.5, 0.8, 0.5}})

-- Blend across the fig for seamless roundness
b_badger_sm:stroke({{470, 650}, {486, 650}, {502, 650}}, {pressure = {0.2, 0.35, 0.2}})

-- 3. Lay the juicy ruby split
local b_r1 = brush{kind = "round", width = 1.8, point = 0.95}
b_r1:load(p_fig_flesh, 1.0)
b_r1:stroke({{480, 642}, {483, 649}, {482, 657}}, {
  pressure = {0.4, 0.9, 0.3},
  swell = {0.7, 1.4, 0.7}
})

-- Golden seed dots inside
b_r1:load(p_fig_seeds, 1.0)
b_r1:touch(482, 647, {pressure = 0.65})
b_r1:touch(483, 651, {pressure = 0.65})

-- Specular highlight on lit shoulder
b_r1:load(p_stone_edge, 0.95)
b_r1:stroke({{472, 642}, {474, 646}}, {pressure = {0.6, 0.9, 0.3}})

-- Clean stem
b_r1:load(p_stem, 0.95)
b_r1:stroke({{480, 630}, {477, 622}, {473, 619}}, {pressure = {0.7, 0.5, 0.3}})

-- Firm contact shadow under base
b_f6:load(p_jug_darks, 0.9)
b_f6:stroke({{470, 666}, {488, 670}, {506, 666}}, {pressure = {0.5, 0.85, 0.4}})

print("Fig carved, filled, and finished.")

--@ chunk 22
-- Re-define all piles globally
p_quince_body     = pile{{"yellow ochre", 6}, {"lead white", 4.5}, {"chrome yellow", 2}, {"green earth", 0.8}, medium = 0.08}
p_quince_core     = pile{{"raw umber", 4}, {"yellow ochre", 5.5}, {"red earth", 2}, medium = 0.1}
p_quince_lit      = pile{{"lead white", 7}, {"yellow ochre", 4.5}, {"chrome yellow", 2}, {"vermilion", 0.3}, medium = 0.08}
p_quince_hi       = pile{{"lead white", 9}, {"chrome yellow", 1.5}, {"yellow ochre", 1}, medium = 0.05}

p_jug_lit         = pile{{"yellow ochre", 6}, {"red earth", 3.5}, {"lead white", 5}, {"raw umber", 0.8}, medium = 0.08}
p_jug_mid         = pile{{"red earth", 5.5}, {"yellow ochre", 5}, {"lead white", 2.2}, {"raw umber", 1.5}, medium = 0.1}
p_terracotta_deep = pile{{"raw umber", 5}, {"red earth", 5.5}, {"bone black", 1.5}, {"yellow ochre", 1}, medium = 0.1}
p_jug_reflect     = pile{{"yellow ochre", 4}, {"raw umber", 3}, {"red earth", 3}, {"lead white", 1.5}, medium = 0.15}
p_jug_darks       = pile{{"bone black", 6}, {"raw umber", 4}, {"red earth", 1}, medium = 0.1}
p_glaze_hi        = pile{{"lead white", 8.5}, {"yellow ochre", 2.2}, {"vermilion", 0.3}, medium = 0.05}
p_rim_hi          = pile{{"lead white", 9}, {"yellow ochre", 1.8}, medium = 0.05}

p_stone_lit       = pile{{"lead white", 7}, {"yellow ochre", 4}, {"raw umber", 2.2}, {"bone black", 0.4}, {"red earth", 0.3}, medium = 0.1}
p_stone_mid       = pile{{"lead white", 4.5}, {"yellow ochre", 4}, {"raw umber", 3.5}, {"bone black", 1}, {"red earth", 0.6}, medium = 0.12}
p_stone_shadow    = pile{{"raw umber", 6}, {"bone black", 3}, {"yellow ochre", 2.5}, {"red earth", 1}, medium = 0.15}
p_stone_edge      = pile{{"lead white", 9}, {"yellow ochre", 1.8}, {"raw umber", 0.5}, medium = 0.05}
p_wall_dark       = pile{{"raw umber", 6}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.12}

p_steel_edge      = pile{{"lead white", 9}, {"bone black", 0.6}, {"cobalt blue", 0.4}, {"raw umber", 0.2}, medium = 0.05}
p_steel_body      = pile{{"lead white", 5.5}, {"bone black", 3}, {"cobalt blue", 1.2}, {"raw umber", 1}, medium = 0.1}
p_steel_shadow    = pile{{"bone black", 4.5}, {"raw umber", 4}, {"cobalt blue", 0.8}, medium = 0.12}
p_knife_handle    = pile{{"bone black", 5.5}, {"raw umber", 4}, {"red earth", 1.5}, medium = 0.08}
p_knife_handle_hi = pile{{"raw umber", 4}, {"yellow ochre", 4}, {"lead white", 3.5}, {"red earth", 1}, medium = 0.08}
p_brass           = pile{{"yellow ochre", 6}, {"lead white", 3.5}, {"raw umber", 1.2}, medium = 0.08}

local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.5, point = 0.85}
local b_r1  = brush{kind = "round", width = 1.4, point = 0.98}
local b_badger = brush("badger", 24)

-- 1. RESTORE THE QUINCE LOWER LOBE
-- Fill in the white hollow on the lower-left of the quince
b_f10:load(p_quince_body, 0.95)
b_f10:stroke({{560, 600}, {562, 635}, {575, 660}, {605, 672}}, {pressure = {0.7, 0.95, 0.7}})
b_f10:stroke({{575, 590}, {578, 625}, {590, 655}, {620, 672}}, {pressure = {0.7, 0.9, 0.6}})

-- Quince base shadow & roundness
b_f10:load(p_quince_core, 0.9)
b_f10:stroke({{575, 655}, {605, 672}, {645, 673}, {675, 660}}, {pressure = {0.5, 0.85, 0.5}})

-- Blend across the quince for smooth organic form
b_badger:stroke({{570, 620}, {610, 630}, {660, 620}}, {pressure = {0.2, 0.35, 0.2}})

-- Firm contact shadow under quince
b_r3:load(p_jug_darks, 0.95)
b_r3:stroke({{572, 666}, {610, 674}, {650, 674}, {678, 664}}, {pressure = {0.7, 1.0, 0.6}})

-- Re-touch quince impasto highlight
b_r3:load(p_quince_hi, 0.95)
b_r3:stroke({{590, 552}, {595, 565}, {598, 578}}, {
  pressure = {0.6, 1.0, 0.4},
  swell = {0.8, 1.4, 0.7}
})

-- 2. PITCHER: ROUND THE RIGHT BELLY AND FOOT
b_f10:load(p_terracotta_deep, 0.9)
b_f10:stroke({{425, 480}, {452, 530}, {445, 575}, {425, 604}}, {pressure = {0.6, 0.9, 0.7}})

-- Reflected bounce on shadow flank
b_f10:load(p_jug_reflect, 0.8)
b_f10:stroke({{448, 510}, {462, 545}, {450, 580}, {430, 602}}, {pressure = {0.4, 0.7, 0.4}})

-- Blend pitcher belly into shadow
b_badger:stroke({{360, 480}, {410, 510}, {445, 530}}, {pressure = {0.2, 0.35, 0.2}})

-- Contact shadow under pitcher base
b_r3:load(p_jug_darks, 0.95)
b_r3:stroke({{315, 604}, {350, 608}, {390, 610}, {428, 604}}, {pressure = {0.8, 1.0, 0.7}})

-- Restate glaze highlights on pitcher
b_r3:load(p_glaze_hi, 0.95)
b_r3:stroke({{334, 370}, {328, 400}, {322, 435}}, {pressure = {0.6, 0.95, 0.3}, swell = {0.8, 1.4, 0.6}})
b_r3:stroke({{346, 288}, {344, 308}, {342, 325}}, {pressure = {0.6, 0.85, 0.3}})

-- Lip rim
b_r1:load(p_rim_hi, 0.95)
b_r1:stroke({{334, 269}, {346, 275}, {368, 277}, {385, 274}}, {pressure = {0.7, 1.0, 0.4}})

-- 3. CLEAN THE CENTER TABLETOP
-- Sweep away the old fig marks with clean limestone paint
b_f14:load(p_stone_lit, 0.95)
b_f14:stroke({{420, 632}, {485, 632}, {555, 632}}, {pressure = {0.6, 0.9, 0.6}})
b_f14:stroke({{420, 646}, {485, 646}, {555, 646}}, {pressure = {0.6, 0.9, 0.6}})
b_f14:stroke({{420, 660}, {485, 660}, {555, 660}}, {pressure = {0.6, 0.9, 0.6}})
b_f14:stroke({{420, 674}, {485, 674}, {555, 674}}, {pressure = {0.6, 0.9, 0.6}})

-- Soft horizontal blend of table
b_badger:stroke({{410, 655}, {490, 655}, {570, 655}}, {pressure = {0.2, 0.35, 0.2}})

-- Lay soft, natural transparent cast shadow from pitcher
b_f10:load(p_stone_shadow, 0.6)
b_f10:stroke({{380, 610}, {435, 622}, {495, 632}, {535, 640}}, {pressure = {0.4, 0.65, 0.3}})
b_f10:stroke({{400, 618}, {455, 630}, {510, 640}, {540, 648}}, {pressure = {0.35, 0.55, 0.25}})

-- Soften the back table horizon (y~518) into the wall
b_f10:load(p_wall_dark, 0.7)
b_f10:stroke({{0, 520}, {250, 521}, {330, 520}}, {pressure = {0.3, 0.5, 0.2}})
b_f10:stroke({{430, 519}, {530, 519}, {620, 519}}, {pressure = {0.3, 0.5, 0.2}})
b_f10:stroke({{700, 519}, {850, 518}, {1000, 516}}, {pressure = {0.3, 0.5, 0.2}})

-- 4. RESTATE THE KNIFE CRISPLY
-- Blade cast shadow on table
b_r3:load(p_stone_shadow, 0.8)
b_r3:stroke({{700, 683}, {750, 700}, {798, 718}}, {pressure = {0.3, 0.6, 0.4}})

-- Cast shadow of handle down front face
b_r3:stroke({{805, 718}, {815, 745}, {825, 775}}, {pressure = {0.5, 0.7, 0.4}})
b_r3:stroke({{845, 735}, {855, 765}, {865, 790}}, {pressure = {0.4, 0.6, 0.3}})

-- Blade solid body
local pts_blade = {{695, 680}, {796, 711}, {796, 716}, {695, 682}}
work(poly(pts_blade, true), {hand = "body", pile = p_steel_body, coverage = 2.5, fill = true, edge = "firm"})

-- Blade lower bevel shadow
b_r1:load(p_steel_shadow, 0.9)
b_r1:stroke({{696, 682}, {745, 698}, {796, 716}}, {pressure = {0.4, 0.7, 0.5}})

-- Sharp razor bevel highlight
b_r1:load(p_steel_edge, 0.98)
b_r1:stroke({{695, 680}, {745, 696}, {796, 711}}, {
  pressure = {0.5, 0.95, 0.6},
  swell = {0.8, 1.2, 0.8}
})

-- Brass bolster
local pts_bolster = {{796, 710}, {805, 713}, {804, 718}, {795, 716}}
work(poly(pts_bolster, true), {hand = "body", pile = p_brass, coverage = 2.5, fill = true, edge = "firm"})
b_r1:load(p_stone_edge, 0.9)
b_r1:stroke({{797, 711}, {804, 713}}, {pressure = 0.7})

-- Knife handle extending over front ledge
local pts_handle = {
  {804, 713}, {835, 725}, {868, 739}, {885, 747},
  {882, 756}, {864, 748}, {832, 734}, {803, 719}
}
work(poly(pts_handle, true), {hand = "body", pile = p_knife_handle, coverage = 2.6, fill = true, edge = "firm"})

-- Specular wood grain sheen on top of handle
b_r3:load(p_knife_handle_hi, 0.9)
b_r3:stroke({{806, 714}, {835, 726}, {865, 740}, {880, 748}}, {
  pressure = {0.4, 0.75, 0.3},
  swell = {0.7, 1.1, 0.7}
})

-- Brass rivet dots
b_r1:load(p_brass, 0.9)
b_r1:touch(825, 725, {pressure = 0.6})
b_r1:touch(855, 738, {pressure = 0.6})

-- Raking highlight along the front ledge edge (skipping knife)
b_r1:load(p_stone_edge, 0.95)
b_r1:stroke({{0, 715}, {260, 712}, {520, 714}, {780, 710}}, {pressure = {0.7, 0.85, 0.6}, shake = 0.5})
b_r1:stroke({{890, 711}, {1000, 712}}, {pressure = {0.6, 0.75, 0.5}, shake = 0.5})

print("Consolidation and refinement complete.")

--@ chunk 23
-- 1. RESTORE THE ENTIRE LOWER BELLY OF THE PITCHER
local pts_jug_lower = {
  {290, 420}, {270, 480}, {276, 530}, {295, 570}, {320, 604},
  {345, 608}, {375, 610}, {405, 608}, {425, 604}, {450, 570},
  {468, 530}, {472, 480}, {450, 420}
}
local m_jug_low = poly(pts_jug_lower, true)

local m_jl_sh = m_jug_low * mask(function(x, y) return x >= 378 and 1 or 0 end)
local m_jl_lt = m_jug_low * mask(function(x, y) return x < 378 and 1 or 0 end)

work(m_jl_lt, {hand = "body", pile = p_jug_lit, angle = 1.4, coverage = 2.8, fill = true, edge = "firm"})
work(m_jl_sh, {hand = "body", pile = p_terracotta_deep, angle = 1.4, coverage = 2.8, fill = true, edge = "firm"})

-- Blend across the pitcher lower belly
local b_badger = brush("badger", 28)
b_badger:stroke({{310, 510}, {370, 510}, {440, 510}}, {pressure = {0.2, 0.4, 0.2}})

-- Contact shadow under pitcher base
local b_r3 = brush{kind = "round", width = 4, point = 0.9}
b_r3:load(p_jug_darks, 0.95)
b_r3:stroke({{315, 604}, {350, 608}, {390, 610}, {428, 604}}, {pressure = {0.8, 1.0, 0.7}})

-- Glaze highlight on shoulder
b_r3:load(p_glaze_hi, 0.95)
b_r3:stroke({{334, 370}, {328, 400}, {322, 435}}, {pressure = {0.6, 0.95, 0.3}, swell = {0.8, 1.4, 0.6}})

-- 2. RESTORE THE ENTIRE QUINCE SOLID BODY
local pts_quince = {
  {630, 485}, {648, 500}, {658, 525}, {678, 555}, {692, 590}, {690, 630},
  {672, 660}, {640, 672}, {605, 670}, {575, 652}, {560, 625}, {562, 590},
  {578, 555}, {600, 525}, {615, 500}
}
local m_quince_full = poly(pts_quince, true)

local m_q_sh = m_quince_full * mask(function(x, y) return x >= 635 and 1 or 0 end)
local m_q_lt = m_quince_full * mask(function(x, y) return x < 635 and 1 or 0 end)

work(m_q_lt, {hand = "body", pile = p_quince_body, angle = 1.3, coverage = 2.8, fill = true, edge = "firm"})
work(m_q_sh, {hand = "body", pile = p_quince_core, angle = 1.2, coverage = 2.8, fill = true, edge = "firm"})

-- Luminous crest on lit lobe
local m_q_cr = m_q_lt * mask(function(x, y)
  local dist = math.sqrt((x - 595)^2 + (y - 560)^2)
  return smoothstep(45, 10, dist)
end)
work(m_q_cr, {hand = "body", pile = p_quince_lit, angle = 1.2, coverage = 2.4, fill = true, edge = "soft"})

-- Blend across the quince
b_badger:stroke({{580, 580}, {630, 580}, {680, 580}}, {pressure = {0.2, 0.35, 0.2}})

-- Quince impasto highlight
b_r3:load(p_quince_hi, 0.95)
b_r3:stroke({{590, 550}, {595, 565}, {598, 578}}, {
  pressure = {0.6, 1.0, 0.4},
  swell = {0.8, 1.4, 0.7}
})

-- Woody stem at apex
b_r3:load(p_stem, 0.95)
b_r3:stroke({{631, 488}, {634, 470}, {639, 455}, {645, 446}}, {pressure = {0.8, 0.6, 0.4}})
b_r3:load(p_jug_darks, 0.8)
b_r3:touch(631, 487, {pressure = 0.7})

-- Contact shadow under quince
b_r3:stroke({{575, 665}, {610, 673}, {650, 672}, {675, 662}}, {pressure = {0.7, 1.0, 0.6}})

-- 3. CLEAN THE CENTER TABLETOP OVER THE OLD FIG MARKS
local m_center_tab = poly({{435, 615}, {565, 615}, {565, 695}, {435, 695}}, true)
work(m_center_tab, {
  hand = "body",
  pile = p_stone_lit,
  angle = 0.0,
  coverage = 2.8,
  fill = true,
  edge = "soft"
})

-- Transparent cast shadow from pitcher
local m_cast_jug_clean = poly({{380, 610}, {440, 618}, {515, 630}, {535, 642}, {470, 645}, {400, 625}}, true)
work(m_cast_jug_clean, {
  hand = "body",
  pile = p_stone_shadow,
  angle = 0.05,
  coverage = 1.6,
  edge = "soft"
})

-- Horizontal badger blend across center table
b_badger:stroke({{420, 655}, {500, 655}, {580, 655}}, {pressure = {0.2, 0.35, 0.2}})

print("Pitcher, quince, and center table completely restored.")

--@ chunk 24
local b_f20 = brush("filbert", 20)
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r4  = brush{kind = "round", width = 4, point = 0.88}
local b_r2  = brush{kind = "round", width = 2.2, point = 0.92}
local b_badger35 = brush("badger", 35)
local b_badger24 = brush("badger", 24)

-- 1. PITCHER: DIRECT BRUSHWORK AND WET-IN-WET BLENDING
-- Lit left side
b_f18:load(p_jug_lit, 0.95)
b_f18:stroke({{330, 310}, {310, 380}, {295, 460}, {305, 540}, {330, 600}}, {pressure = {0.7, 0.95, 0.7}})
b_f18:stroke({{350, 310}, {335, 380}, {325, 460}, {335, 540}, {355, 604}}, {pressure = {0.7, 0.95, 0.7}})

-- Center midtone
b_f18:load(p_jug_mid, 0.95)
b_f18:stroke({{370, 310}, {365, 380}, {360, 460}, {368, 540}, {385, 604}}, {pressure = {0.7, 0.95, 0.7}})

-- Core shadow right
b_f18:load(p_terracotta_deep, 0.95)
b_f18:stroke({{390, 310}, {400, 380}, {410, 460}, {415, 540}, {415, 602}}, {pressure = {0.7, 0.95, 0.7}})
b_f18:stroke({{405, 320}, {425, 380}, {448, 460}, {445, 540}, {425, 600}}, {pressure = {0.6, 0.9, 0.6}})

-- Reflected bounce light on right rim
b_f14:load(p_jug_reflect, 0.85)
b_f14:stroke({{420, 340}, {440, 400}, {465, 470}, {460, 530}, {438, 580}}, {pressure = {0.4, 0.75, 0.4}})

-- Badger blend: vertical then horizontal
b_badger35:stroke({{370, 290}, {370, 450}, {370, 600}}, {pressure = {0.2, 0.35, 0.2}})
b_badger35:stroke({{300, 460}, {370, 470}, {450, 460}}, {pressure = {0.2, 0.35, 0.2}})

-- Mouth interior
b_r4:load(p_jug_darks, 0.95)
b_r4:stroke({{350, 268}, {370, 269}, {390, 268}}, {pressure = 0.8})

-- Handle
b_f10:load(p_terracotta_deep, 0.9)
b_f10:stroke({{330, 335}, {300, 345}, {266, 375}, {248, 415}, {250, 455}, {270, 485}, {290, 498}}, {pressure = 0.7})
b_f10:load(p_jug_lit, 0.85)
b_f10:stroke({{330, 332}, {296, 340}, {260, 370}, {244, 410}, {246, 445}}, {pressure = {0.5, 0.8, 0.4}})

-- Shoulder glaze highlight
b_r4:load(p_glaze_hi, 1.0)
b_r4:stroke({{334, 370}, {328, 400}, {322, 435}}, {pressure = {0.6, 1.0, 0.4}, swell = {0.8, 1.5, 0.7}})
b_r4:stroke({{346, 288}, {344, 308}, {342, 325}}, {pressure = {0.6, 0.85, 0.3}})

-- Lip rim
b_r2:load(p_rim_hi, 1.0)
b_r2:stroke({{334, 269}, {346, 275}, {368, 277}, {385, 274}}, {pressure = {0.7, 1.0, 0.4}})

-- Contact shadow under pitcher
b_r4:load(p_jug_darks, 1.0)
b_r4:stroke({{315, 604}, {350, 608}, {390, 610}, {428, 604}}, {pressure = {0.8, 1.0, 0.7}})

-- 2. QUINCE: DIRECT BRUSHWORK AND BLENDING
b_f14:load(p_quince_lit, 0.95)
b_f14:stroke({{610, 500}, {595, 540}, {585, 580}, {590, 630}, {605, 665}}, {pressure = {0.7, 0.95, 0.7}})

b_f14:load(p_quince_body, 0.95)
b_f14:stroke({{630, 490}, {620, 540}, {615, 590}, {620, 640}, {635, 672}}, {pressure = {0.7, 0.95, 0.7}})
b_f14:stroke({{585, 540}, {570, 580}, {565, 620}, {575, 655}, {600, 670}}, {pressure = {0.6, 0.9, 0.6}})

b_f14:load(p_quince_core, 0.95)
b_f14:stroke({{645, 505}, {655, 550}, {668, 600}, {665, 645}, {650, 672}}, {pressure = {0.7, 0.95, 0.7}})
b_f10:load(p_quince_core, 0.85)
b_f10:stroke({{660, 540}, {680, 580}, {688, 620}, {678, 655}}, {pressure = {0.5, 0.8, 0.5}})

-- Badger blend across quince
b_badger24:stroke({{580, 580}, {630, 580}, {680, 580}}, {pressure = {0.2, 0.35, 0.2}})

-- Impasto highlight on knobby belly
b_r4:load(p_quince_hi, 1.0)
b_r4:stroke({{590, 550}, {595, 565}, {598, 578}}, {pressure = {0.6, 1.0, 0.4}, swell = {0.8, 1.4, 0.7}})

-- Stem
b_r2:load(p_stem, 1.0)
b_r2:stroke({{631, 488}, {634, 470}, {639, 455}, {645, 446}}, {pressure = {0.8, 0.6, 0.4}})
b_r2:load(p_jug_darks, 0.8)
b_r2:touch(631, 487, {pressure = 0.7})

-- Contact shadow under quince
b_r4:load(p_jug_darks, 0.95)
b_r4:stroke({{575, 665}, {610, 673}, {650, 672}, {675, 662}}, {pressure = {0.7, 1.0, 0.6}})

-- 3. CENTER TABLETOP: SOLID LIMESTONE & SOFT SHADOW
b_f20:load(p_stone_lit, 0.95)
b_f20:stroke({{415, 628}, {485, 628}, {555, 628}}, {pressure = {0.7, 0.95, 0.7}})
b_f20:stroke({{415, 646}, {485, 646}, {555, 646}}, {pressure = {0.7, 0.95, 0.7}})
b_f20:stroke({{415, 664}, {485, 664}, {555, 664}}, {pressure = {0.7, 0.95, 0.7}})
b_f20:stroke({{415, 680}, {485, 680}, {555, 680}}, {pressure = {0.7, 0.95, 0.7}})

b_badger35:stroke({{410, 655}, {490, 655}, {570, 655}}, {pressure = {0.2, 0.35, 0.2}})

-- Transparent cast shadow
b_f14:load(p_stone_shadow, 0.65)
b_f14:stroke({{380, 610}, {440, 618}, {500, 628}, {535, 638}}, {pressure = {0.4, 0.7, 0.3}})
b_f14:stroke({{400, 620}, {455, 628}, {505, 638}, {535, 645}}, {pressure = {0.35, 0.6, 0.25}})

b_badger24:stroke({{430, 630}, {480, 632}, {535, 640}}, {pressure = {0.15, 0.25, 0.15}})

print("Painterly consolidation complete.")

--@ chunk 25
local b_f20 = brush("filbert", 20)
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r4  = brush{kind = "round", width = 4, point = 0.88}
local b_r2  = brush{kind = "round", width = 2.2, point = 0.92}
local b_badger35 = brush("badger", 35)
local b_badger24 = brush("badger", 24)

-- 1. QUINCE: COAT IN SOLID GOLDEN FLESH & BLEND SHADOW WET-IN-WET
-- Coat the entire quince volume in dense, luminous golden body paint
b_f18:load(p_quince_body, 1.0)
b_f18:stroke({{630, 490}, {615, 530}, {595, 570}, {585, 620}, {595, 665}}, {pressure = {0.8, 1.0, 0.8}})
b_f18:stroke({{630, 490}, {630, 540}, {625, 590}, {620, 640}, {625, 672}}, {pressure = {0.8, 1.0, 0.8}})
b_f18:stroke({{635, 495}, {648, 540}, {658, 590}, {655, 640}, {650, 670}}, {pressure = {0.8, 1.0, 0.8}})
b_f14:load(p_quince_body, 0.95)
b_f14:stroke({{585, 540}, {570, 580}, {565, 620}, {575, 655}, {595, 670}}, {pressure = {0.7, 0.95, 0.7}})
b_f14:stroke({{655, 530}, {675, 575}, {685, 620}, {678, 655}, {655, 672}}, {pressure = {0.7, 0.95, 0.7}})

-- Lay warm shadow along right flank into the wet gold paint
b_f14:load(p_quince_core, 0.85)
b_f14:stroke({{642, 500}, {652, 545}, {662, 595}, {660, 645}, {648, 670}}, {pressure = {0.6, 0.9, 0.6}})
b_f10:load(p_quince_core, 0.8)
b_f10:stroke({{660, 540}, {678, 580}, {684, 620}, {674, 655}}, {pressure = {0.5, 0.8, 0.5}})

-- Luminous light on left lobe
b_f14:load(p_quince_lit, 0.9)
b_f14:stroke({{610, 510}, {595, 550}, {585, 590}, {590, 630}}, {pressure = {0.6, 0.9, 0.5}})

-- Badger blend across the quince: gentle crossing strokes
b_badger24:stroke({{575, 580}, {630, 580}, {685, 580}}, {pressure = {0.2, 0.35, 0.2}})
b_badger24:stroke({{630, 500}, {625, 580}, {620, 660}}, {pressure = {0.2, 0.3, 0.2}})

-- Impasto highlight on knobby crest
b_r4:load(p_quince_hi, 1.0)
b_r4:stroke({{588, 550}, {593, 565}, {596, 578}}, {
  pressure = {0.6, 1.0, 0.4},
  swell = {0.8, 1.4, 0.7}
})

-- Stem
b_r2:load(p_stem, 1.0)
b_r2:stroke({{631, 488}, {634, 470}, {639, 455}, {645, 446}}, {pressure = {0.8, 0.6, 0.4}})
b_r2:load(p_jug_darks, 0.8)
b_r2:touch(631, 487, {pressure = 0.7})

-- Contact shadow under quince
b_r4:load(p_jug_darks, 0.95)
b_r4:stroke({{575, 665}, {610, 673}, {650, 672}, {675, 662}}, {pressure = {0.7, 1.0, 0.6}})

-- 2. PITCHER: UNIFY SHADOW AND BODY
-- Full-length shadow stroke down right side
b_f18:load(p_terracotta_deep, 0.95)
b_f18:stroke({{395, 305}, {405, 380}, {415, 460}, {418, 540}, {415, 604}}, {pressure = {0.7, 0.95, 0.7}})
b_f14:load(p_terracotta_deep, 0.9)
b_f14:stroke({{410, 330}, {430, 400}, {450, 470}, {445, 540}, {425, 602}}, {pressure = {0.6, 0.9, 0.6}})

-- Full-length lit flank stroke down left side
b_f18:load(p_jug_lit, 0.95)
b_f18:stroke({{345, 305}, {330, 380}, {320, 460}, {330, 540}, {350, 604}}, {pressure = {0.7, 0.95, 0.7}})

-- Center midtone stroke
b_f18:load(p_jug_mid, 0.9)
b_f18:stroke({{370, 305}, {365, 380}, {360, 460}, {368, 540}, {385, 604}}, {pressure = {0.6, 0.9, 0.6}})

-- Vertical blend
b_badger35:stroke({{365, 300}, {365, 450}, {370, 600}}, {pressure = {0.2, 0.35, 0.2}})

-- Shoulder glaze highlight
b_r4:load(p_glaze_hi, 1.0)
b_r4:stroke({{334, 370}, {328, 400}, {322, 435}}, {pressure = {0.6, 1.0, 0.4}, swell = {0.8, 1.5, 0.7}})
b_r4:stroke({{346, 288}, {344, 308}, {342, 325}}, {pressure = {0.6, 0.85, 0.3}})

-- Lip rim
b_r2:load(p_rim_hi, 1.0)
b_r2:stroke({{334, 269}, {346, 275}, {368, 277}, {385, 274}}, {pressure = {0.7, 1.0, 0.4}})

-- Contact shadow under base
b_r4:load(p_jug_darks, 1.0)
b_r4:stroke({{315, 604}, {350, 608}, {390, 610}, {428, 604}}, {pressure = {0.8, 1.0, 0.7}})

-- 3. SMOOTH THE CENTER TABLETOP
b_f20:load(p_stone_lit, 0.95)
b_f20:stroke({{415, 630}, {485, 630}, {560, 630}}, {pressure = {0.7, 0.95, 0.7}})
b_f20:stroke({{415, 648}, {485, 648}, {560, 648}}, {pressure = {0.7, 0.95, 0.7}})
b_f20:stroke({{415, 665}, {485, 665}, {560, 665}}, {pressure = {0.7, 0.95, 0.7}})

-- Horizontal blend
b_badger35:stroke({{410, 650}, {490, 650}, {570, 650}}, {pressure = {0.2, 0.35, 0.2}})

-- Soft transparent cast shadow from pitcher
b_f14:load(p_stone_shadow, 0.6)
b_f14:stroke({{380, 610}, {435, 618}, {490, 626}, {525, 634}}, {pressure = {0.4, 0.65, 0.3}})
b_f14:stroke({{395, 618}, {450, 626}, {495, 634}, {525, 640}}, {pressure = {0.35, 0.55, 0.25}})

b_badger24:stroke({{410, 625}, {465, 628}, {525, 635}}, {pressure = {0.15, 0.25, 0.15}})

print("Unification pass complete.")

--@ chunk 26
-- 1. PITCHER: COMPLETE UNIFIED SOLID MODELING
local pts_jug_body = {
  {335, 270}, {338, 310}, {330, 340}, {305, 380}, {280, 430}, {270, 480},
  {276, 530}, {295, 570}, {320, 604}, {345, 608}, {375, 610}, {405, 608},
  {425, 604}, {450, 570}, {468, 530}, {472, 480}, {460, 430}, {435, 380},
  {412, 340}, {402, 310}, {405, 270}, {385, 262}, {355, 262}
}
local m_jug_solid = poly(pts_jug_body, true)

-- A. Coat the entire pitcher body in opaque warm terracotta
work(m_jug_solid, {
  hand = "body",
  pile = p_jug_lit,
  angle = 1.45,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- B. Continuous smoothstep core shadow
local m_jug_sh = m_jug_solid * mask(function(x, y)
  local term_x = 372 + (y - 350) * 0.12
  return smoothstep(term_x - 20, term_x + 25, x)
end)
work(m_jug_sh, {
  hand = "body",
  pile = p_terracotta_deep,
  angle = 1.45,
  coverage = 2.4,
  fill = true,
  edge = "soft"
})

-- C. Smoothstep reflected bounce light on right rim
local m_jug_ref = m_jug_solid * mask(function(x, y)
  return smoothstep(430, 465, x)
end)
work(m_jug_ref, {
  hand = "body",
  pile = p_jug_reflect,
  angle = 1.4,
  coverage = 1.8,
  edge = "soft"
})

-- D. Blend pitcher wet-in-wet for seamless spherical/cylindrical roundness
blend(m_jug_solid, {angle = 0})

-- E. Mouth interior
local m_mouth = ellipse(370, 268, 33, 8)
work(m_mouth, {hand = "body", pile = p_jug_darks, angle = 0, coverage = 2.5, fill = true, edge = "firm"})

-- F. Handle
local b_f10 = brush("filbert", 10)
b_f10:load(p_terracotta_deep, 0.95)
b_f10:stroke({{330, 335}, {300, 345}, {266, 375}, {248, 415}, {250, 455}, {270, 485}, {290, 498}}, {pressure = 0.75})
b_f10:load(p_jug_lit, 0.9)
b_f10:stroke({{330, 332}, {296, 340}, {260, 370}, {244, 410}, {246, 445}}, {pressure = {0.5, 0.85, 0.4}})

-- G. Shoulder glaze impasto highlight
local b_r4 = brush{kind = "round", width = 4, point = 0.88}
b_r4:load(p_glaze_hi, 1.0)
b_r4:stroke({{334, 370}, {328, 400}, {322, 435}}, {pressure = {0.6, 1.0, 0.4}, swell = {0.8, 1.5, 0.7}})
b_r4:stroke({{346, 288}, {344, 308}, {342, 325}}, {pressure = {0.6, 0.85, 0.3}})

-- Lip rim
local b_r2 = brush{kind = "round", width = 2.2, point = 0.92}
b_r2:load(p_rim_hi, 1.0)
b_r2:stroke({{334, 269}, {346, 275}, {368, 277}, {385, 274}}, {pressure = {0.7, 1.0, 0.4}})

-- Contact shadow under base
b_r4:load(p_jug_darks, 1.0)
b_r4:stroke({{315, 604}, {350, 608}, {390, 610}, {428, 604}}, {pressure = {0.8, 1.0, 0.7}})

-- 2. QUINCE: COMPLETE UNIFIED SOLID MODELING
local pts_quince = {
  {630, 485}, {648, 500}, {658, 525}, {678, 555}, {692, 590}, {690, 630},
  {672, 660}, {640, 672}, {605, 670}, {575, 652}, {560, 625}, {562, 590},
  {578, 555}, {600, 525}, {615, 500}
}
local m_quince_solid = poly(pts_quince, true)

-- A. Coat the entire quince volume in opaque golden body paint
work(m_quince_solid, {
  hand = "body",
  pile = p_quince_body,
  angle = 1.3,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- B. Continuous smoothstep shadow along right flank
local m_q_sh = m_quince_solid * mask(function(x, y)
  local term_x = 636 + (y - 550) * 0.1
  return smoothstep(term_x - 18, term_x + 22, x)
end)
work(m_q_sh, {
  hand = "body",
  pile = p_quince_core,
  angle = 1.2,
  coverage = 2.4,
  fill = true,
  edge = "soft"
})

-- C. Luminous crest on lit lobe
local m_q_cr = m_quince_solid * mask(function(x, y)
  local dist = math.sqrt((x - 595)^2 + (y - 560)^2)
  return smoothstep(50, 10, dist)
end)
work(m_q_cr, {
  hand = "body",
  pile = p_quince_lit,
  angle = 1.2,
  coverage = 2.2,
  fill = true,
  edge = "soft"
})

-- D. Blend across quince for smooth organic transitions
blend(m_quince_solid, {angle = 0.2})

-- E. Impasto highlight on knobby belly
b_r4:load(p_quince_hi, 1.0)
b_r4:stroke({{590, 550}, {595, 565}, {598, 578}}, {
  pressure = {0.6, 1.0, 0.4},
  swell = {0.8, 1.4, 0.7}
})

-- F. Stem
b_r2:load(p_stem, 1.0)
b_r2:stroke({{631, 488}, {634, 470}, {639, 455}, {645, 446}}, {pressure = {0.8, 0.6, 0.4}})
b_r2:load(p_jug_darks, 0.8)
b_r2:touch(631, 487, {pressure = 0.7})

-- G. Contact shadow under quince
b_r4:load(p_jug_darks, 0.95)
b_r4:stroke({{575, 665}, {610, 673}, {650, 672}, {675, 662}}, {pressure = {0.7, 1.0, 0.6}})

-- 3. CENTER TABLETOP: RE-ESTABLISH CLEAN STONE AND SOFT CAST SHADOW
local m_tab_mid = poly({{425, 615}, {565, 615}, {565, 685}, {425, 685}}, true)
work(m_tab_mid, {
  hand = "body",
  pile = p_stone_lit,
  angle = 0.0,
  coverage = 3.0,
  fill = true,
  edge = "soft"
})

-- Transparent cast shadow from pitcher
local m_cast_clean = poly({{380, 610}, {435, 618}, {495, 628}, {530, 638}, {470, 642}, {395, 622}}, true)
work(m_cast_clean, {
  hand = "body",
  pile = p_stone_shadow,
  angle = 0.05,
  coverage = 1.4,
  edge = "soft"
})

blend(m_tab_mid, {angle = 0})

print("Definitive modeling pass complete.")

--@ chunk 27
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r4  = brush{kind = "round", width = 3.5, point = 0.9}
local b_badger35 = brush("badger", 35)
local b_badger24 = brush("badger", 24)

-- 1. DISSOLVE THE CIRCULAR TABLETOP PATCH
b_f18:load(p_stone_lit, 0.65)
b_f18:stroke({{380, 635}, {480, 635}, {570, 635}}, {pressure = {0.3, 0.6, 0.2}})
b_f18:stroke({{380, 655}, {480, 655}, {570, 655}}, {pressure = {0.3, 0.6, 0.2}})
b_f18:stroke({{380, 675}, {480, 675}, {570, 675}}, {pressure = {0.3, 0.6, 0.2}})

-- Horizontal badger blend across the table
b_badger35:stroke({{360, 655}, {475, 655}, {590, 655}}, {pressure = {0.15, 0.3, 0.15}})

-- Pitcher cast shadow: soft, transparent, natural
b_f14:load(p_stone_shadow, 0.5)
b_f14:stroke({{405, 612}, {455, 622}, {505, 632}}, {pressure = {0.35, 0.55, 0.2}})
b_badger24:stroke({{400, 620}, {460, 625}, {515, 632}}, {pressure = {0.12, 0.22, 0.12}})

-- Quince cast shadow: soft pool stretching to the right
b_f14:load(p_stone_shadow, 0.55)
b_f14:stroke({{615, 672}, {660, 676}, {710, 680}}, {pressure = {0.35, 0.6, 0.25}})
b_badger24:stroke({{610, 675}, {665, 678}, {715, 682}}, {pressure = {0.12, 0.22, 0.12}})

-- 2. DEEPEN PITCHER SHADOW FLANK
b_f14:load(p_terracotta_deep, 0.85)
b_f14:stroke({{395, 310}, {410, 380}, {425, 460}, {435, 530}, {420, 600}}, {pressure = {0.4, 0.75, 0.4}})
b_f10:load(p_terracotta_deep, 0.8)
b_f10:stroke({{415, 340}, {435, 400}, {455, 470}, {450, 535}, {425, 595}}, {pressure = {0.4, 0.7, 0.4}})

-- Vertical blend along the right flank
b_badger24:stroke({{415, 310}, {430, 450}, {425, 600}}, {pressure = {0.15, 0.25, 0.15}})

-- 3. FIRM CONTACT SHADOWS
b_r4:load(p_jug_darks, 0.95)
b_r4:stroke({{320, 604}, {365, 609}, {425, 604}}, {pressure = {0.7, 1.0, 0.6}})
b_r4:stroke({{575, 666}, {615, 673}, {665, 666}}, {pressure = {0.7, 1.0, 0.6}})

print("Table blended, shadows and pitcher flank refined.")

--@ chunk 28
-- 1. FILL THE ENTIRE TABLE GAP BETWEEN PITCHER AND QUINCE SEAMLESSLY
local m_tab_gap = poly({{425, 520}, {562, 520}, {562, 712}, {425, 712}})
work(m_tab_gap, {
  hand = "body",
  pile = p_stone_lit,
  angle = 0.0,
  coverage = 2.8,
  fill = true,
  edge = "firm"
})

-- Soft transparent cast shadow from pitcher
local b_sh = brush("filbert", 12)
b_sh:load(p_stone_shadow, 0.55)
b_sh:stroke({{425, 612}, {465, 622}, {515, 632}}, {
  pressure = {0.5, 0.65, 0.25}
})
local b_badger16 = brush("badger", 16)
b_badger16:stroke({{425, 622}, {470, 626}, {520, 634}}, {pressure = {0.12, 0.2, 0.12}})

-- 2. RESTORE THE QUINCE LOWER-LEFT FLANK
local b_q = brush("filbert", 12)
b_q:load(p_quince_body, 0.95)
b_q:stroke({{560, 590}, {562, 625}, {572, 655}, {600, 670}}, {pressure = {0.7, 0.95, 0.7}})
b_q:load(p_quince_lit, 0.9)
b_q:stroke({{572, 580}, {576, 615}, {588, 650}, {612, 668}}, {pressure = {0.6, 0.9, 0.6}})

-- Soft blend inside the quince
local b_badger24 = brush("badger", 24)
b_badger24:stroke({{568, 620}, {595, 625}, {630, 620}}, {pressure = {0.15, 0.25, 0.15}})

-- Contact shadow under quince base
local b_r3 = brush{kind = "round", width = 3.5, point = 0.9}
b_r3:load(p_jug_darks, 0.95)
b_r3:stroke({{562, 665}, {605, 673}, {650, 672}, {675, 662}}, {pressure = {0.7, 1.0, 0.6}})

-- Cast shadow of quince to the right
b_sh:load(p_stone_shadow, 0.55)
b_sh:stroke({{635, 672}, {680, 676}, {725, 680}}, {pressure = {0.4, 0.65, 0.25}})
b_badger16:stroke({{635, 675}, {680, 678}, {725, 682}}, {pressure = {0.12, 0.2, 0.12}})

-- Re-touch quince impasto highlight
b_r3:load(p_quince_hi, 1.0)
b_r3:stroke({{588, 550}, {593, 565}, {596, 578}}, {pressure = {0.6, 1.0, 0.4}, swell = {0.8, 1.4, 0.7}})

-- Raking highlight on front ledge edge in that span
local b_r1 = brush{kind = "round", width = 1.4, point = 0.98}
b_r1:load(p_stone_edge, 0.95)
b_r1:stroke({{420, 714}, {565, 713}}, {pressure = {0.7, 0.85, 0.6}, shake = 0.5})

print("Table gap filled and quince flank restored.")

--@ chunk 29
-- 1. RESTORE PITCHER RIGHT FLANK TO ITS FULL NATURAL BULGE
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_badger24 = brush("badger", 24)

-- Deep terracotta body on right flank
b_f14:load(p_terracotta_deep, 0.95)
b_f14:stroke({
  {400, 340}, {425, 380}, {455, 430}, {470, 480}, {465, 530}, {445, 575}, {425, 604}
}, {
  pressure = {0.7, 0.95, 0.7},
  swell = {0.8, 1.3, 0.8}
})

b_f10:load(p_terracotta_deep, 0.9)
b_f10:stroke({
  {385, 330}, {405, 380}, {430, 440}, {440, 500}, {430, 560}, {415, 604}
}, {
  pressure = {0.6, 0.9, 0.6}
})

-- Reflected bounce light along the far right rim
b_f10:load(p_jug_reflect, 0.85)
b_f10:stroke({
  {420, 360}, {445, 410}, {468, 470}, {468, 520}, {448, 570}, {428, 602}
}, {
  pressure = {0.4, 0.7, 0.4}
})

-- Blend shadow softly toward center belly
b_badger24:stroke({{390, 460}, {430, 460}, {465, 460}}, {pressure = {0.15, 0.3, 0.15}})

-- Contact shadow under base
local b_r4 = brush{kind = "round", width = 3.5, point = 0.9}
b_r4:load(p_jug_darks, 0.95)
b_r4:stroke({{320, 604}, {365, 609}, {425, 604}}, {pressure = {0.7, 1.0, 0.6}})

-- Pitcher cast shadow extending onto table
b_f14:load(p_stone_shadow, 0.6)
b_f14:stroke({{425, 606}, {460, 616}, {510, 628}, {540, 638}}, {pressure = {0.4, 0.65, 0.25}})
b_badger24:stroke({{425, 612}, {470, 620}, {525, 632}}, {pressure = {0.12, 0.2, 0.12}})

-- 2. UNIFY TABLE WITH A WARM LIMESTONE GLAZE
p_stone_glaze = pile{{"yellow ochre", 4.5}, {"raw umber", 3.5}, {"lead white", 2.5}, medium = 0.35}

local table_back = {{0, 520}, {250, 522}, {500, 518}, {750, 520}, {1000, 516}}
local table_front = {{0, 715}, {260, 712}, {520, 714}, {780, 710}, {1000, 712}}
local m_table_all = below(table_back) * above(table_front)

-- Protect objects slightly
local pts_jug_p = {
  {320, 550}, {440, 550}, {475, 480}, {460, 430}, {425, 606}, {320, 606}
}
local pts_q_p = {
  {560, 580}, {690, 580}, {690, 675}, {560, 675}
}
local pts_k_p = {
  {690, 675}, {890, 715}, {890, 765}, {690, 685}
}
local m_protect = poly(pts_jug_p, true) + poly(pts_q_p, true) + poly(pts_k_p, true)

work(m_table_all - m_protect, {
  hand = "glaze",
  pile = p_stone_glaze,
  angle = 0.0,
  coverage = 1.6,
  edge = "soft"
})

-- Blend table glaze horizontally
local b_badger35 = brush("badger", 35)
b_badger35:stroke({{0, 620}, {500, 620}, {1000, 620}}, {pressure = {0.15, 0.25, 0.15}})
b_badger35:stroke({{0, 670}, {500, 670}, {1000, 670}}, {pressure = {0.15, 0.25, 0.15}})

print("Pitcher right flank restored, tabletop glazed and unified.")

--@ chunk 30
-- 1. PITCHER: MODELING STRICTLY CLIPPED TO MASK
local pts_jug_body = {
  {335, 270}, {338, 310}, {330, 340}, {305, 380}, {280, 430}, {270, 480},
  {276, 530}, {295, 570}, {320, 604}, {345, 608}, {375, 610}, {405, 608},
  {425, 604}, {450, 570}, {468, 530}, {472, 480}, {460, 430}, {435, 380},
  {412, 340}, {402, 310}, {405, 270}, {385, 262}, {355, 262}
}
local m_jug_solid = poly(pts_jug_body, true)

-- Opaque warm terracotta body
work(m_jug_solid, {
  hand = "body",
  pile = p_jug_lit,
  angle = 1.45,
  coverage = 3.0,
  fill = true,
  clip = true
})

-- Shaded right flank
local m_jug_sh = m_jug_solid * mask(function(x, y)
  local term_x = 372 + (y - 350) * 0.12
  return smoothstep(term_x - 18, term_x + 22, x)
end)
work(m_jug_sh, {
  hand = "body",
  pile = p_terracotta_deep,
  angle = 1.45,
  coverage = 2.4,
  fill = true,
  clip = true
})

-- Reflected bounce light on right rim
local m_jug_ref = m_jug_solid * mask(function(x, y)
  return smoothstep(430, 465, x)
end)
work(m_jug_ref, {
  hand = "body",
  pile = p_jug_reflect,
  angle = 1.4,
  coverage = 1.8,
  clip = true
})

-- Blend inside pitcher
blend(m_jug_solid, {angle = 0, clip = true})

-- Mouth interior
local m_mouth = ellipse(370, 268, 33, 8)
work(m_mouth, {hand = "body", pile = p_jug_darks, angle = 0, coverage = 2.5, fill = true, clip = true})

-- Handle
local b_f10 = brush("filbert", 10)
b_f10:load(p_terracotta_deep, 0.95)
b_f10:stroke({{330, 335}, {300, 345}, {266, 375}, {248, 415}, {250, 455}, {270, 485}, {290, 498}}, {pressure = 0.75})
b_f10:load(p_jug_lit, 0.9)
b_f10:stroke({{330, 332}, {296, 340}, {260, 370}, {244, 410}, {246, 445}}, {pressure = {0.5, 0.85, 0.4}})

-- Shoulder glaze highlight
local b_r4 = brush{kind = "round", width = 4, point = 0.88}
b_r4:load(p_glaze_hi, 1.0)
b_r4:stroke({{334, 370}, {328, 400}, {322, 435}}, {pressure = {0.6, 1.0, 0.4}, swell = {0.8, 1.5, 0.7}, clip = m_jug_solid})
b_r4:stroke({{346, 288}, {344, 308}, {342, 325}}, {pressure = {0.6, 0.85, 0.3}, clip = m_jug_solid})

-- Lip rim
local b_r2 = brush{kind = "round", width = 2.2, point = 0.92}
b_r2:load(p_rim_hi, 1.0)
b_r2:stroke({{334, 269}, {346, 275}, {368, 277}, {385, 274}}, {pressure = {0.7, 1.0, 0.4}, clip = m_jug_solid})

-- Contact shadow under base
b_r4:load(p_jug_darks, 1.0)
b_r4:stroke({{315, 604}, {350, 608}, {390, 610}, {428, 604}}, {pressure = {0.8, 1.0, 0.7}})

-- 2. QUINCE: MODELING STRICTLY CLIPPED TO MASK
local pts_quince = {
  {630, 485}, {648, 500}, {658, 525}, {678, 555}, {692, 590}, {690, 630},
  {672, 660}, {640, 672}, {605, 670}, {575, 652}, {560, 625}, {562, 590},
  {578, 555}, {600, 525}, {615, 500}
}
local m_quince_solid = poly(pts_quince, true)

-- Opaque golden flesh
work(m_quince_solid, {
  hand = "body",
  pile = p_quince_body,
  angle = 1.3,
  coverage = 3.0,
  fill = true,
  clip = true
})

-- Shaded right flank
local m_q_sh = m_quince_solid * mask(function(x, y)
  local term_x = 636 + (y - 550) * 0.1
  return smoothstep(term_x - 18, term_x + 22, x)
end)
work(m_q_sh, {
  hand = "body",
  pile = p_quince_core,
  angle = 1.2,
  coverage = 2.2,
  fill = true,
  clip = true
})

-- Lit crest
local m_q_cr = m_quince_solid * mask(function(x, y)
  local dist = math.sqrt((x - 595)^2 + (y - 560)^2)
  return smoothstep(50, 10, dist)
end)
work(m_q_cr, {
  hand = "body",
  pile = p_quince_lit,
  angle = 1.2,
  coverage = 2.2,
  fill = true,
  clip = true
})

-- Blend inside quince
blend(m_quince_solid, {angle = 0.2, clip = true})

-- Impasto highlight on knobby belly
b_r4:load(p_quince_hi, 1.0)
b_r4:stroke({{588, 550}, {593, 565}, {596, 578}}, {
  pressure = {0.6, 1.0, 0.4},
  swell = {0.8, 1.4, 0.7},
  clip = m_quince_solid
})

-- Stem
b_r2:load(p_stem, 1.0)
b_r2:stroke({{631, 488}, {634, 470}, {639, 455}, {645, 446}}, {pressure = {0.8, 0.6, 0.4}})
b_r2:load(p_jug_darks, 0.8)
b_r2:touch(631, 487, {pressure = 0.7})

-- Contact shadow under quince
b_r4:load(p_jug_darks, 0.95)
b_r4:stroke({{575, 665}, {610, 673}, {650, 672}, {675, 662}}, {pressure = {0.7, 1.0, 0.6}})

-- 3. CAST SHADOWS ON TABLETOP
local b_f14 = brush("filbert", 14)
-- Pitcher cast shadow
b_f14:load(p_stone_shadow, 0.5)
b_f14:stroke({{425, 608}, {465, 618}, {515, 628}, {540, 638}}, {pressure = {0.4, 0.6, 0.2}})

-- Quince cast shadow
b_f14:load(p_stone_shadow, 0.55)
b_f14:stroke({{635, 672}, {680, 676}, {725, 680}}, {pressure = {0.4, 0.65, 0.25}})

print("Clipped modeling pass complete.")

--@ chunk 31
local b_f18 = brush("filbert", 18)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.5, point = 0.85}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.92}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}
local b_badger24 = brush("badger", 24)

-- 1. HANDLE JOIN TO PITCHER BELLY
b_r3:load(p_terracotta_deep, 0.95)
b_r3:stroke({{265, 460}, {280, 485}, {305, 498}}, {pressure = {0.7, 0.9, 0.6}})

b_r2:load(p_jug_lit, 0.9)
b_r2:stroke({{260, 455}, {275, 480}, {295, 492}}, {pressure = {0.5, 0.8, 0.4}})

-- 2. WALL NEGATIVE SPACES AND CONTOURS
-- Negative space inside handle loop
b_f10:load(p_wall_dark, 0.95)
b_f10:stroke({{280, 400}, {290, 430}, {295, 460}}, {pressure = 0.7})

-- Behind right neck & shoulder
b_f10:stroke({{415, 290}, {425, 330}, {440, 360}}, {pressure = {0.6, 0.85, 0.5}})

-- Behind upper quince neck
b_f10:stroke({{595, 485}, {615, 500}, {625, 515}}, {pressure = {0.5, 0.75, 0.4}})

-- Deepen chiaroscuro left of pitcher
b_f18:load(p_wall_dark, 0.75)
b_f18:stroke({{200, 300}, {220, 380}, {230, 450}}, {pressure = {0.4, 0.7, 0.3}})
b_badger24:stroke({{180, 380}, {225, 380}, {260, 380}}, {pressure = {0.15, 0.25, 0.15}})

-- 3. PITCHER IMPASTO GLAZE HIGHLIGHTS
b_r3:load(p_glaze_hi, 1.0)
b_r3:stroke({{333, 372}, {327, 400}, {321, 430}}, {
  pressure = {0.7, 1.0, 0.4},
  swell = {0.8, 1.5, 0.7}
})

b_r2:load(p_glaze_hi, 1.0)
b_r2:stroke({{346, 290}, {344, 308}, {342, 322}}, {pressure = {0.6, 0.9, 0.3}})

b_r2:load(p_rim_hi, 1.0)
b_r2:stroke({{334, 269}, {346, 275}, {368, 277}, {385, 274}}, {pressure = {0.8, 1.0, 0.4}})

-- 4. QUINCE IMPASTO HIGHLIGHT AND CALYX
b_r3:load(p_quince_hi, 1.0)
b_r3:stroke({{589, 548}, {594, 563}, {597, 576}}, {
  pressure = {0.7, 1.0, 0.4},
  swell = {0.8, 1.5, 0.7}
})

b_r2:load(p_jug_darks, 0.9)
b_r2:touch(631, 487, {pressure = 0.75})

b_r2:load(p_stem, 1.0)
b_r2:stroke({{631, 488}, {634, 470}, {639, 455}, {645, 446}}, {pressure = {0.8, 0.6, 0.4}})

-- 5. CRISP CHAMFER LIGHT ALONG FRONT LEDGE RIM
b_r1:load(p_stone_edge, 1.0)
b_r1:stroke({{0, 715}, {260, 712}, {520, 714}, {780, 710}}, {pressure = {0.7, 0.9, 0.6}, shake = 0.4})
b_r1:stroke({{890, 711}, {1000, 712}}, {pressure = {0.6, 0.8, 0.5}, shake = 0.4})

-- 6. RAZOR BEVEL HIGHLIGHT ON KNIFE BLADE
b_r1:load(p_steel_edge, 1.0)
b_r1:stroke({{696, 680}, {745, 696}, {796, 711}}, {
  pressure = {0.6, 1.0, 0.6},
  swell = {0.8, 1.2, 0.8}
})

print("Finishing touches applied successfully.")

--@ chunk 32
-- 1. RESTORE CLEAN CURVE OF PITCHER NECK AND LEFT CONTOUR
local b_r3 = brush{kind = "round", width = 4, point = 0.88}
b_r3:load(p_jug_lit, 0.95)
b_r3:stroke({{336, 290}, {330, 340}, {312, 385}, {295, 430}, {285, 470}}, {
  pressure = {0.7, 0.95, 0.7}
})

-- Re-touch shoulder highlight
b_r3:load(p_glaze_hi, 1.0)
b_r3:stroke({{333, 372}, {327, 400}, {321, 430}}, {
  pressure = {0.7, 1.0, 0.4},
  swell = {0.8, 1.5, 0.7}
})

-- 2. RESTORE QUINCE APEX AND NECK CONTOUR
b_r3:load(p_quince_lit, 0.95)
b_r3:stroke({{630, 486}, {618, 505}, {605, 530}, {592, 555}}, {
  pressure = {0.6, 0.9, 0.5}
})

-- Calyx and stem
local b_r2 = brush{kind = "round", width = 2.0, point = 0.92}
b_r2:load(p_jug_darks, 0.8)
b_r2:touch(631, 487, {pressure = 0.7})
b_r2:load(p_stem, 1.0)
b_r2:stroke({{631, 488}, {634, 470}, {639, 455}, {645, 446}}, {pressure = {0.8, 0.6, 0.4}})

-- 3. MELT FLOATING WALL MARKS WITH BADGER BLENDER
local b_badger35 = brush("badger", 35)
b_badger35:stroke({{150, 360}, {220, 360}, {270, 360}}, {pressure = {0.2, 0.35, 0.2}})
b_badger35:stroke({{160, 420}, {220, 420}, {270, 420}}, {pressure = {0.2, 0.35, 0.2}})

print("Contour adjustments and wall blending complete.")

--@ chunk 33
-- 1. COVER WALL MARKS TO THE LEFT OF THE HANDLE WITH OPAQUE WALL PAINT
local b_f18 = brush("filbert", 18)
b_f18:load(p_wall_dark, 0.95)
b_f18:stroke({{170, 330}, {170, 420}, {170, 515}}, {pressure = {0.7, 0.9, 0.7}})
b_f18:stroke({{210, 330}, {210, 420}, {210, 515}}, {pressure = {0.7, 0.9, 0.7}})

-- Feather softly into the surrounding wall
b_f18:load(p_wall_dark, 0.5)
b_f18:stroke({{120, 400}, {180, 400}, {240, 400}}, {pressure = 0.4})
b_f18:stroke({{120, 490}, {180, 490}, {240, 490}}, {pressure = 0.4})

-- 2. CLEAN QUINCE CONTOUR
local b_r3 = brush{kind = "round", width = 3.5, point = 0.85}
b_r3:load(p_quince_lit, 0.95)
b_r3:stroke({{580, 580}, {590, 610}, {600, 635}}, {pressure = 0.8})

-- Stem restatement
local b_r2 = brush{kind = "round", width = 1.8, point = 0.95}
b_r2:load(p_stem, 1.0)
b_r2:stroke({{631, 488}, {634, 470}, {639, 455}, {645, 446}}, {pressure = {0.8, 0.6, 0.4}})

print("Wall marks covered and quince contour cleaned.")

--@ chunk 34
-- Cover the fenced patch to the left of the handle with the wall's true warm tone
p_wall_warm = pile{{"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 1}, {"lead white", 2}, medium = 0.15}

local m_wall_patch = rect(120, 320, 140, 198)

work(m_wall_patch, {
  hand = "body",
  pile = p_wall_warm,
  angle = 0.15,
  coverage = 2.8,
  fill = true,
  edge = "soft"
})

-- Soft blend of the wall patch into the surrounding wall
blend(rect(100, 300, 180, 230), {angle = 0})

print("Wall fence covered and integrated seamlessly.")

--@ chunk 35
p_glaze_wall = pile{{"raw umber", 7}, {"bone black", 3.5}, {"yellow ochre", 1.8}, medium = 0.25}

local table_back = {{0, 520}, {250, 522}, {500, 518}, {750, 520}, {1000, 516}}
local m_above_table = above(table_back)

local pts_jug = {
  {330, 260}, {310, 330}, {230, 390}, {235, 450}, {270, 510},
  {260, 540}, {310, 608}, {435, 608}, {480, 540}, {475, 450},
  {438, 360}, {415, 310}, {412, 260}
}
local m_jug_protect = poly(pts_jug):shrink(4)

local pts_quince = {
  {625, 440}, {645, 440}, {640, 480}, {660, 510}, {685, 550},
  {700, 600}, {680, 668}, {635, 680}, {595, 678}, {560, 640},
  {550, 580}, {570, 530}, {610, 490}
}
local m_quince_protect = poly(pts_quince):shrink(4)

local m_wall = m_above_table - m_jug_protect - m_quince_protect

-- Glaze the entire background wall in its true deep umber tone
work(m_wall, {
  hand = "glaze",
  pile = p_glaze_wall,
  coverage = 2.4,
  fill = true,
  angle = 0.05
})

-- Blend the wall
blend(m_wall, {angle = 0})

print("Background wall completely unified in deep chiaroscuro.")

--@ chunk 36
-- 1. EARTHENWARE PITCHER: STRICTLY CLIPPED OPAQUE SOLID MODELING
local pts_jug_body = {
  {335, 270}, {338, 310}, {330, 340}, {305, 380}, {280, 430}, {270, 480},
  {276, 530}, {295, 570}, {320, 604}, {345, 608}, {375, 610}, {405, 608},
  {425, 604}, {450, 570}, {468, 530}, {472, 480}, {460, 430}, {435, 380},
  {412, 340}, {402, 310}, {405, 270}, {385, 262}, {355, 262}
}
local m_jug_solid = poly(pts_jug_body, true)

-- Opaque warm terracotta foundation
work(m_jug_solid, {
  hand = "body",
  pile = p_jug_lit,
  angle = 1.45,
  coverage = 3.2,
  fill = true,
  clip = true
})

-- Continuous smoothstep core shadow
local m_jug_sh = m_jug_solid * mask(function(x, y)
  local term_x = 372 + (y - 350) * 0.12
  return smoothstep(term_x - 18, term_x + 22, x)
end)
work(m_jug_sh, {
  hand = "body",
  pile = p_terracotta_deep,
  angle = 1.45,
  coverage = 2.6,
  fill = true,
  clip = true
})

-- Smoothstep reflected bounce light on right rim
local m_jug_ref = m_jug_solid * mask(function(x, y)
  return smoothstep(430, 465, x)
end)
work(m_jug_ref, {
  hand = "body",
  pile = p_jug_reflect,
  angle = 1.4,
  coverage = 1.8,
  clip = true
})

-- Soft internal blend
blend(m_jug_solid, {angle = 0, clip = true})

-- Mouth interior
local m_mouth = ellipse(370, 268, 33, 8)
work(m_mouth, {hand = "body", pile = p_jug_darks, angle = 0, coverage = 2.5, fill = true, clip = true})

-- Handle with solid return join
local b_f10 = brush("filbert", 10)
b_f10:load(p_terracotta_deep, 0.95)
b_f10:stroke({{330, 335}, {300, 345}, {266, 375}, {248, 415}, {250, 455}, {270, 485}, {300, 498}}, {pressure = 0.8})

b_f10:load(p_jug_lit, 0.9)
b_f10:stroke({{330, 332}, {296, 340}, {260, 370}, {244, 410}, {246, 445}, {270, 480}, {295, 492}}, {pressure = {0.5, 0.85, 0.4}})

-- Shoulder glaze highlight
local b_r4 = brush{kind = "round", width = 4, point = 0.88}
b_r4:load(p_glaze_hi, 1.0)
b_r4:stroke({{334, 370}, {328, 400}, {322, 435}}, {
  pressure = {0.6, 1.0, 0.4},
  swell = {0.8, 1.5, 0.7},
  clip = m_jug_solid
})

-- Neck gleam
b_r4:stroke({{346, 288}, {344, 308}, {342, 325}}, {pressure = {0.6, 0.85, 0.3}, clip = m_jug_solid})

-- Lip rim
local b_r2 = brush{kind = "round", width = 2.2, point = 0.92}
b_r2:load(p_rim_hi, 1.0)
b_r2:stroke({{334, 269}, {346, 275}, {368, 277}, {385, 274}}, {pressure = {0.8, 1.0, 0.4}, clip = m_jug_solid})

-- Contact shadow under pitcher base
b_r4:load(p_jug_darks, 1.0)
b_r4:stroke({{315, 604}, {350, 608}, {390, 610}, {428, 604}}, {pressure = {0.8, 1.0, 0.7}})

-- 2. QUINCE: STRICTLY CLIPPED OPAQUE SOLID MODELING
local pts_quince = {
  {630, 485}, {648, 500}, {658, 525}, {678, 555}, {692, 590}, {690, 630},
  {672, 660}, {640, 672}, {605, 670}, {575, 652}, {560, 625}, {562, 590},
  {578, 555}, {600, 525}, {615, 500}
}
local m_quince_solid = poly(pts_quince, true)

-- Opaque golden flesh
work(m_quince_solid, {
  hand = "body",
  pile = p_quince_body,
  angle = 1.3,
  coverage = 3.2,
  fill = true,
  clip = true
})

-- Shaded right flank
local m_q_sh = m_quince_solid * mask(function(x, y)
  local term_x = 636 + (y - 550) * 0.1
  return smoothstep(term_x - 18, term_x + 22, x)
end)
work(m_q_sh, {
  hand = "body",
  pile = p_quince_core,
  angle = 1.2,
  coverage = 2.4,
  fill = true,
  clip = true
})

-- Lit crest
local m_q_cr = m_quince_solid * mask(function(x, y)
  local dist = math.sqrt((x - 595)^2 + (y - 560)^2)
  return smoothstep(50, 10, dist)
end)
work(m_q_cr, {
  hand = "body",
  pile = p_quince_lit,
  angle = 1.2,
  coverage = 2.2,
  fill = true,
  clip = true
})

-- Internal blend
blend(m_quince_solid, {angle = 0.2, clip = true})

-- Impasto highlight on knobby crest
b_r4:load(p_quince_hi, 1.0)
b_r4:stroke({{588, 550}, {593, 565}, {596, 578}}, {
  pressure = {0.7, 1.0, 0.4},
  swell = {0.8, 1.4, 0.7},
  clip = m_quince_solid
})

-- Calyx & woody stem
b_r2:load(p_stem, 1.0)
b_r2:stroke({{631, 488}, {634, 470}, {639, 455}, {645, 446}}, {pressure = {0.8, 0.6, 0.4}})
b_r2:load(p_jug_darks, 0.8)
b_r2:touch(631, 487, {pressure = 0.7})

-- Contact shadow under quince
b_r4:load(p_jug_darks, 0.95)
b_r4:stroke({{575, 665}, {610, 673}, {650, 672}, {675, 662}}, {pressure = {0.7, 1.0, 0.6}})

-- 3. CAST SHADOWS ON TABLETOP
local b_f14 = brush("filbert", 14)
b_f14:load(p_stone_shadow, 0.5)
b_f14:stroke({{425, 608}, {465, 618}, {515, 628}, {540, 638}}, {pressure = {0.4, 0.6, 0.2}})

b_f14:load(p_stone_shadow, 0.55)
b_f14:stroke({{635, 672}, {680, 676}, {725, 680}}, {pressure = {0.4, 0.65, 0.25}})

print("Clipped modeling pass complete.")

--@ chunk 37
-- 1. CLEAN THE FRONT LEDGE FACE UNDER KNIFE
local b_f14 = brush("filbert", 14)
b_f14:load(p_ledge_face, 0.95)
b_f14:stroke({{790, 760}, {850, 760}, {910, 760}}, {pressure = {0.7, 0.9, 0.7}})
b_f14:stroke({{790, 780}, {850, 780}, {910, 780}}, {pressure = {0.7, 0.9, 0.7}})

-- Soft cast shadow under the handle onto the vertical face
local b_f10 = brush("filbert", 10)
b_f10:load(p_stone_shadow, 0.75)
b_f10:stroke({{812, 726}, {840, 746}, {870, 760}}, {pressure = {0.4, 0.7, 0.3}})

-- 2. RESTATE THE KNIFE SOLIDLY
local pts_handle = {
  {804, 713}, {835, 725}, {868, 739}, {885, 747},
  {882, 756}, {864, 748}, {832, 734}, {803, 719}
}
work(poly(pts_handle, true), {hand = "body", pile = p_knife_handle, coverage = 2.6, fill = true, edge = "firm"})

-- Specular light on turned handle
local b_r3 = brush{kind = "round", width = 3, point = 0.88}
b_r3:load(p_knife_handle_hi, 0.95)
b_r3:stroke({{806, 714}, {835, 726}, {865, 740}, {880, 748}}, {
  pressure = {0.4, 0.8, 0.3},
  swell = {0.7, 1.1, 0.7}
})

-- Brass bolster & rivets
local pts_bolster = {{796, 710}, {805, 713}, {804, 718}, {795, 716}}
work(poly(pts_bolster, true), {hand = "body", pile = p_brass, coverage = 2.5, fill = true, edge = "firm"})

local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}
b_r1:load(p_brass, 0.95)
b_r1:touch(825, 725, {pressure = 0.65})
b_r1:touch(855, 738, {pressure = 0.65})

-- Razor bevel highlight on blade
b_r1:load(p_steel_edge, 1.0)
b_r1:stroke({{696, 680}, {745, 696}, {796, 711}}, {
  pressure = {0.6, 1.0, 0.6},
  swell = {0.8, 1.2, 0.8}
})

-- Front ledge raking highlight
b_r1:load(p_stone_edge, 1.0)
b_r1:stroke({{890, 711}, {1000, 712}}, {pressure = {0.6, 0.8, 0.5}, shake = 0.4})

print("Knife cleaned and restated.")

--@ chunk 38
local t = wait(0)
print("Current time:", t)
print("Drying wall (200, 300):", drying(200, 300))
print("Drying pitcher (370, 450):", drying(370, 450))
print("Drying quince (630, 580):", drying(630, 580))
print("Drying table (500, 650):", drying(500, 650))
print("Drying knife (850, 730):", drying(850, 730))

--@ chunk 39
local t = wait(5 * 24 * 60)
print("Time after 5 days:", t)
print("Drying wall (200, 300):", drying(200, 300))
print("Drying pitcher (370, 450):", drying(370, 450))
print("Drying quince (630, 580):", drying(630, 580))
print("Drying table (500, 650):", drying(500, 650))
print("Drying knife (850, 730):", drying(850, 730))

--@ chunk 40
local t = wait(2 * 24 * 60)
print("Time after 2 more days:", t)
print("Drying pitcher (370, 450):", drying(370, 450))

--@ chunk 41
p_wall_dark = pile{{"raw umber", 6.5}, {"bone black", 3.5}, {"yellow ochre", 1.2}, medium = 0.12}
p_wall_mid  = pile{{"raw umber", 5.5}, {"bone black", 2.0}, {"yellow ochre", 3.0}, {"lead white", 1.0}, medium = 0.12}
p_wall_warm = pile{{"raw umber", 4.5}, {"yellow ochre", 4.0}, {"lead white", 1.8}, {"bone black", 1.0}, medium = 0.10}

local b_f24 = brush("filbert", 24)
local b_f16 = brush("filbert", 16)
local b_r8  = brush("filbert", 8)
local b_badger = brush("badger", 28)

-- 1. Paint the dark wall behind the left flank of the pitcher (x: 0 to 350, y: 150 to 525)
-- to eliminate the halo and create a deep, solid chiaroscuro background
for y = 200, 520, 25 do
  b_f24:load(p_wall_dark, 0.95)
  b_f24:stroke({{0, y}, {150, y + 5}, {290, y + 2}}, {pressure = {0.8, 0.95, 0.7}})
end

-- Close in on the pitcher's lit contour and handle
-- Stroke right up to and overlapping the handle (x ~ 230-270)
b_f16:load(p_wall_dark, 0.95)
b_f16:stroke({{200, 320}, {250, 340}, {310, 340}}, {pressure = 0.85})
b_f16:stroke({{180, 380}, {235, 410}, {260, 430}}, {pressure = 0.85})
b_f16:stroke({{180, 440}, {230, 460}, {265, 490}}, {pressure = 0.85})
b_f16:stroke({{190, 500}, {250, 515}, {280, 525}}, {pressure = 0.85})

-- Cut into the negative space inside the handle loop (x ~ 250-320, y ~ 340-480)
b_r8:load(p_wall_dark, 1.0)
b_r8:stroke({{265, 360}, {295, 380}, {310, 420}, {285, 460}}, {pressure = 0.9})
b_r8:stroke({{280, 370}, {305, 400}, {305, 440}}, {pressure = 0.9})

-- Neck left contour (x ~ 310-335, y ~ 260-340)
b_f16:load(p_wall_dark, 0.95)
b_f16:stroke({{260, 265}, {305, 270}, {332, 270}}, {pressure = 0.85})
b_f16:stroke({{260, 290}, {310, 295}, {334, 305}}, {pressure = 0.85})
b_f16:stroke({{260, 320}, {310, 325}, {328, 335}}, {pressure = 0.85})

-- Behind top mouth (y ~ 240-265)
b_f16:load(p_wall_dark, 0.95)
b_f16:stroke({{300, 255}, {360, 255}, {430, 255}}, {pressure = 0.85})

-- Right of the pitcher neck and shoulder (x ~ 400-500, y ~ 260-450)
b_f16:load(p_wall_mid, 0.9)
b_f16:stroke({{408, 270}, {440, 275}, {480, 280}}, {pressure = 0.85})
b_f16:stroke({{406, 310}, {440, 315}, {480, 320}}, {pressure = 0.85})
b_f16:stroke({{425, 360}, {460, 365}, {510, 370}}, {pressure = 0.85})
b_f16:stroke({{455, 420}, {485, 425}, {530, 430}}, {pressure = 0.85})
b_f16:stroke({{468, 480}, {495, 485}, {540, 490}}, {pressure = 0.85})
b_f16:stroke({{455, 520}, {485, 522}, {530, 525}}, {pressure = 0.85})

-- Around the quince (x ~ 520-720, y ~ 420-530)
b_f16:load(p_wall_mid, 0.9)
b_f16:stroke({{520, 440}, {580, 440}, {625, 445}}, {pressure = 0.85})
b_f16:stroke({{645, 440}, {680, 440}, {720, 445}}, {pressure = 0.85})
b_f16:stroke({{510, 470}, {560, 475}, {605, 485}}, {pressure = 0.85})
b_f16:stroke({{650, 480}, {690, 480}, {740, 485}}, {pressure = 0.85})
b_f16:stroke({{665, 515}, {700, 518}, {750, 520}}, {pressure = 0.85})

-- Wall right side across to edge (x ~ 700 to 1000, y ~ 350 to 525)
for y = 350, 520, 30 do
  b_f24:load(p_wall_mid, 0.85)
  b_f24:stroke({{720, y}, {850, y + 2}, {1000, y}}, {pressure = {0.7, 0.85, 0.7}})
end

-- Atmosphere badger blend across the freshly painted wall
b_badger:stroke({{50, 280}, {250, 280}}, {pressure = 0.3})
b_badger:stroke({{50, 360}, {250, 360}}, {pressure = 0.3})
b_badger:stroke({{50, 450}, {250, 450}}, {pressure = 0.3})
b_badger:stroke({{450, 320}, {600, 320}, {750, 320}}, {pressure = 0.25})
b_badger:stroke({{750, 400}, {950, 400}}, {pressure = 0.25})

print("Wall repainted and halos covered.")

--@ chunk 42
blend(rect(0, 180, 1000, 340), {clip = false})

--@ chunk 43
local t = wait(5 * 24 * 60)
print("Time after 5 days:", t)
print("Drying wall (150, 350):", drying(150, 350))
print("Drying wall (700, 400):", drying(700, 400))

--@ chunk 44
p_stone_lit     = pile{{"lead white", 8.2}, {"yellow ochre", 2.2}, {"raw umber", 0.7}, {"red earth", 0.25}, medium = 0.06}
p_stone_mid     = pile{{"lead white", 7.0}, {"yellow ochre", 3.0}, {"raw umber", 1.2}, {"red earth", 0.4}, medium = 0.08}
p_stone_warm    = pile{{"lead white", 6.2}, {"yellow ochre", 3.5}, {"raw umber", 1.6}, {"red earth", 0.5}, medium = 0.08}
p_ledge_face    = pile{{"raw umber", 7.0}, {"bone black", 3.5}, {"red earth", 1.2}, medium = 0.08}
p_ledge_chamfer = pile{{"lead white", 9.2}, {"yellow ochre", 1.5}, {"raw umber", 0.3}, medium = 0.04}

local b_f24 = brush("filbert", 24)
local b_f16 = brush("filbert", 16)
local b_badger = brush("badger", 32)
local b_flat = brush("flat", 24)

-- 1. Front ledge vertical face (y: 715 to 800)
for y = 725, 795, 20 do
  b_flat:load(p_ledge_face, 0.95)
  b_flat:stroke({{0, y}, {500, y}, {1000, y}}, {pressure = {0.8, 0.9, 0.8}})
end

-- 2. Horizontal tabletop stone slab (y: 520 to 715)
-- Back table edge junction at y = 520
b_f16:load(p_stone_mid, 0.9)
b_f16:stroke({{0, 520}, {400, 521}, {750, 519}, {1000, 520}}, {pressure = 0.8})

-- Broad horizontal sweeps across the table plane
for y = 535, 705, 22 do
  local p_use = (y < 600) and p_stone_mid or ((y > 660) and p_stone_warm or p_stone_lit)
  b_f24:load(p_use, 0.95)
  -- Left half (brighter raking light)
  b_f24:stroke({{0, y}, {300, y - 1}, {550, y}}, {pressure = {0.85, 0.95, 0.8}})
  -- Right half (slightly warmer midtone)
  b_f24:load(p_stone_mid, 0.9)
  b_f24:stroke({{500, y}, {750, y + 1}, {1000, y}}, {pressure = {0.8, 0.9, 0.8}})
end

-- Chamfer / front edge of stone slab (y = 714)
b_f16:load(p_stone_lit, 0.95)
b_f16:stroke({{0, 714}, {500, 714}, {1000, 714}}, {pressure = 0.85})

-- Soft horizontal badger sweep across the stone table to unify the limestone plane
b_badger:stroke({{50, 540}, {500, 540}, {950, 540}}, {pressure = 0.3})
b_badger:stroke({{50, 600}, {500, 600}, {950, 600}}, {pressure = 0.3})
b_badger:stroke({{50, 660}, {500, 660}, {950, 660}}, {pressure = 0.3})
b_badger:stroke({{50, 705}, {500, 705}, {950, 705}}, {pressure = 0.25})

-- Chamfer highlight
local b_r2 = brush{kind = "round", width = 2.0, point = 0.9}
b_r2:load(p_ledge_chamfer, 1.0)
b_r2:stroke({{0, 715}, {400, 715}, {750, 715}, {1000, 715}}, {pressure = {0.6, 0.85, 0.5}, shake = 0.3})

print("Tabletop and front ledge re-established as solid limestone.")

--@ chunk 45
blend(rect(0, 515, 1000, 205), {clip = false})

--@ chunk 46
local b_f16 = brush("filbert", 16)
local b_flat = brush("flat", 24)
local b_r2 = brush{kind = "round", width = 2.2, point = 0.95}

-- 1. Redefine the back wall junction along y = 518
-- Left side (dark wall behind jug)
b_f16:load(p_wall_dark, 0.95)
b_f16:stroke({{0, 515}, {250, 516}, {500, 515}}, {pressure = 0.9})
-- Right side (mid wall)
b_f16:load(p_wall_mid, 0.95)
b_f16:stroke({{500, 515}, {750, 516}, {1000, 515}}, {pressure = 0.9})

-- Feather wall slightly above the line
local b_badger = brush("badger", 20)
b_badger:stroke({{0, 510}, {500, 510}, {1000, 510}}, {pressure = 0.2})

-- 2. Redefine the vertical front ledge face (y: 718 to 800)
for y = 725, 795, 20 do
  b_flat:load(p_ledge_face, 0.95)
  b_flat:stroke({{0, y}, {500, y}, {1000, y}}, {pressure = 0.9})
end

-- 3. Sharp, crisp chamfer highlight along the stone table's front edge (y = 715)
b_r2:load(p_ledge_chamfer, 1.0)
b_r2:stroke({{0, 715}, {350, 715}, {700, 715}, {1000, 715}}, {pressure = {0.6, 0.85, 0.6}, shake = 0.2})

print("Table boundaries crisply established.")

--@ chunk 47
local b_f24 = brush("filbert", 24)
local b_f16 = brush("filbert", 16)
local b_flat = brush("flat", 24)

-- 1. Paint out the light smears above the table junction (y: 470 to 520)
-- Left side (dark wall)
for y = 475, 518, 14 do
  b_f24:load(p_wall_dark, 0.95)
  b_f24:stroke({{0, y}, {250, y}, {500, y}}, {pressure = 0.95})
end

-- Right side (warm mid wall)
for y = 475, 518, 14 do
  b_f24:load(p_wall_mid, 0.95)
  b_f24:stroke({{500, y}, {750, y}, {1000, y}}, {pressure = 0.95})
end

-- Horizontal stroke right along the table seam at y = 520
b_f16:load(p_wall_dark, 0.95)
b_f16:stroke({{0, 520}, {250, 520}, {500, 520}}, {pressure = 0.9})
b_f16:load(p_wall_mid, 0.95)
b_f16:stroke({{500, 520}, {750, 520}, {1000, 520}}, {pressure = 0.9})

-- 2. Clean the vertical front ledge face (y: 715 to 800)
for y = 720, 795, 15 do
  b_flat:load(p_ledge_face, 0.98)
  b_flat:stroke({{0, y}, {500, y}, {1000, y}}, {pressure = 0.95})
end

-- Sharp chiseled chamfer highlight along y = 715
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_ledge_chamfer, 1.0)
b_r2:stroke({{0, 715}, {350, 715}, {700, 715}, {1000, 715}}, {pressure = {0.7, 0.9, 0.6}, shake = 0.2})

print("Wall horizon and front ledge cleaned cleanly.")

--@ chunk 48
local t = wait(4 * 24 * 60)
print("Time after drying:", t)
print("Drying wall (300, 400):", drying(300, 400))
print("Drying table (500, 600):", drying(500, 600))
print("Drying ledge (500, 750):", drying(500, 750))

--@ chunk 49
p_terracotta_deep  = pile{{"raw umber", 6.0}, {"red earth", 4.0}, {"bone black", 1.5}, medium = 0.06}
p_terracotta_core  = pile{{"red earth", 5.5}, {"raw umber", 3.5}, {"yellow ochre", 1.5}, medium = 0.06}
p_terracotta_mid   = pile{{"red earth", 4.5}, {"yellow ochre", 4.0}, {"lead white", 2.0}, {"raw umber", 1.0}, medium = 0.05}
p_terracotta_lit   = pile{{"yellow ochre", 4.5}, {"lead white", 4.0}, {"red earth", 2.5}, medium = 0.05}
p_terracotta_hi    = pile{{"lead white", 7.5}, {"yellow ochre", 2.5}, {"red earth", 0.8}, medium = 0.04}
p_glaze_highlight  = pile{{"lead white", 9.2}, {"yellow ochre", 1.2}, medium = 0.03}
p_jug_reflect      = pile{{"red earth", 4.0}, {"yellow ochre", 4.5}, {"raw umber", 2.0}, medium = 0.06}
p_interior_dark    = pile{{"bone black", 5.0}, {"raw umber", 4.0}, {"red earth", 2.0}, medium = 0.06}

local pts_jug_body = {
  {338, 268}, {340, 310}, {336, 340}, {302, 385}, {270, 440}, {266, 485},
  {280, 540}, {312, 590}, {326, 615}, {420, 615}, {435, 590}, {458, 540},
  {472, 485}, {468, 440}, {438, 385}, {405, 340}, {402, 310}, {404, 268}
}
local m_jug = poly(pts_jug_body, true)

-- 1. Solid opaque midtone foundation for the pitcher body
work(m_jug, {
  hand = "body",
  pile = p_terracotta_mid,
  angle = 1.45,
  coverage = 3.0,
  fill = true,
  edge = "firm"
})

-- 2. Sculpt the volumes wet-in-wet with directional filbert brush strokes
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r4  = brush{kind = "round", width = 4, point = 0.8}

-- Lit flank on left (curves following the roundness of the belly)
b_f14:load(p_terracotta_lit, 0.95)
b_f14:stroke({{344, 275}, {345, 315}, {342, 345}}, {pressure = 0.85})
b_f14:stroke({{342, 345}, {315, 390}, {288, 445}, {282, 490}, {296, 545}, {328, 595}, {345, 612}}, {pressure = {0.8, 0.95, 0.7}})
b_f14:stroke({{355, 350}, {335, 400}, {310, 460}, {310, 510}, {325, 560}, {355, 610}}, {pressure = {0.8, 0.95, 0.7}})

-- Deep core shadow along x ~ 380 to 445
b_f14:load(p_terracotta_deep, 0.95)
b_f14:stroke({{390, 275}, {392, 315}, {388, 345}}, {pressure = 0.85})
b_f14:stroke({{388, 345}, {405, 390}, {430, 445}, {438, 490}, {425, 545}, {405, 595}, {395, 612}}, {pressure = {0.85, 0.98, 0.85}})
b_f10:load(p_terracotta_deep, 0.95)
b_f10:stroke({{408, 360}, {430, 410}, {452, 465}, {456, 505}, {442, 555}, {418, 605}}, {pressure = {0.8, 0.95, 0.8}})

-- Reflected table bounce along the right rim (x ~ 445 to 470)
b_f10:load(p_jug_reflect, 0.9)
b_f10:stroke({{398, 275}, {400, 315}, {396, 345}}, {pressure = 0.65})
b_f10:stroke({{425, 385}, {450, 435}, {465, 480}, {455, 530}, {435, 580}, {418, 610}}, {pressure = {0.5, 0.75, 0.5}})

-- Soft badger sweep around the belly circumference to marry the tones into a seamless turning form
local b_badger = brush("badger", 18)
b_badger:stroke({{280, 440}, {340, 450}, {410, 450}, {460, 440}}, {pressure = 0.3})
b_badger:stroke({{275, 485}, {340, 495}, {415, 495}, {465, 485}}, {pressure = 0.35})
b_badger:stroke({{290, 535}, {350, 545}, {415, 545}, {450, 535}}, {pressure = 0.3})
b_badger:stroke({{320, 585}, {365, 595}, {410, 595}, {430, 585}}, {pressure = 0.25})

-- 3. Pitcher Mouth and Interior
-- Interior dark hole
local m_mouth_hole = ellipse(370, 269, 30, 6)
work(m_mouth_hole, {hand = "detail", pile = p_interior_dark, coverage = 3.0, fill = true})

-- Lip rim catching light on the left
local b_r2 = brush{kind = "round", width = 2.2, point = 0.95}
b_r2:load(p_terracotta_lit, 0.95)
b_r2:stroke({{336, 268}, {355, 273}, {375, 274}}, {pressure = 0.8})
b_r2:load(p_terracotta_deep, 0.95)
b_r2:stroke({{375, 274}, {395, 273}, {404, 268}}, {pressure = 0.75})

-- Contact shadow under foot on table
b_f10:load(p_terracotta_deep, 1.0)
b_f10:stroke({{320, 616}, {370, 618}, {425, 616}}, {pressure = {0.7, 0.95, 0.6}})

print("Pitcher body and mouth modeled.")

--@ chunk 50
local b_r4 = brush{kind = "round", width = 4.2, point = 0.85}
local b_r2 = brush{kind = "round", width = 2.4, point = 0.92}
local b_r1 = brush{kind = "round", width = 1.4, point = 0.98}
local b_f10 = brush("filbert", 10)

-- 1. PULLED CLAY LOOP HANDLE
-- Handle shadow / underside core
local pts_handle_under = {
  {334, 318}, {300, 335}, {260, 365}, {236, 405}, {238, 435}, {255, 465}, {270, 485}
}
b_r4:load(p_terracotta_deep, 0.95)
b_r4:stroke(pts_handle_under, {pressure = {0.6, 0.9, 0.7}, swell = {0.8, 1.2, 0.8}})

-- Handle top spine catching warm raking light
local pts_handle_lit = {
  {336, 315}, {302, 330}, {262, 360}, {240, 400}, {242, 430}, {258, 460}, {272, 482}
}
b_r2:load(p_terracotta_lit, 0.95)
b_r2:stroke(pts_handle_lit, {pressure = {0.5, 0.85, 0.4}})

-- Bright specular catchlight on apex of handle curve
b_r1:load(p_terracotta_hi, 1.0)
b_r1:stroke({{242, 390}, {240, 410}, {244, 430}}, {pressure = {0.4, 0.8, 0.4}})

-- Cast shadow of handle onto the pitcher's lit belly
b_r4:load(p_terracotta_core, 0.7)
b_r4:stroke({{280, 420}, {295, 445}, {305, 475}}, {pressure = {0.3, 0.6, 0.2}})

-- Clean the negative space inside the handle loop with dark wall paint
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke({{265, 385}, {295, 400}, {300, 435}, {280, 465}}, {pressure = 0.85})

-- 2. RIM AND MOUTH MODELING
-- Flared lip highlight on left
b_r2:load(p_terracotta_hi, 0.95)
b_r2:stroke({{334, 268}, {350, 273}, {370, 274}}, {pressure = {0.6, 0.85, 0.5}})

-- Glaze catchlight on the front lip
b_r1:load(p_glaze_highlight, 1.0)
b_r1:touch(348, 272, {pressure = 0.85})

-- 3. JUICY GLAZE IMPASTO ON THE LIT SHOULDER
-- Broad buttery gleam where the glossy slip catches the north window
b_r4:load(p_terracotta_hi, 0.95)
b_r4:stroke({{325, 375}, {320, 410}, {312, 445}}, {pressure = {0.5, 0.85, 0.4}, swell = {0.8, 1.3, 0.7}})

-- Crisp specular star highlight
b_r2:load(p_glaze_highlight, 1.0)
b_r2:stroke({{322, 395}, {319, 415}, {316, 430}}, {pressure = {0.5, 0.95, 0.4}})
b_r1:load(p_glaze_highlight, 1.0)
b_r1:touch(320, 410, {pressure = 0.9})

-- Subtle horizontal throwing ring striations catching light
b_r1:load(p_terracotta_lit, 0.75)
b_r1:stroke({{285, 465}, {330, 472}, {380, 470}}, {pressure = {0.2, 0.5, 0.2}, shake = 0.3})
b_r1:stroke({{295, 520}, {340, 526}, {390, 524}}, {pressure = {0.2, 0.45, 0.2}, shake = 0.3})

-- 4. PITCHER CAST SHADOW ON THE STONE TABLE
p_cast_shadow = pile{{"raw umber", 5.5}, {"bone black", 1.8}, {"yellow ochre", 2.2}, {"lead white", 1.2}, medium = 0.18}
b_f10:load(p_cast_shadow, 0.6)
b_f10:stroke({{415, 616}, {465, 624}, {525, 632}, {570, 638}}, {pressure = {0.6, 0.75, 0.3}, swell = {0.9, 1.3, 0.7}})
b_f10:stroke({{400, 618}, {450, 628}, {505, 636}}, {pressure = {0.4, 0.6, 0.2}})

-- Feather cast shadow edge into open table
local b_badger = brush("badger", 14)
b_badger:stroke({{440, 622}, {500, 630}, {560, 636}}, {pressure = 0.25})

print("Handle, glaze impasto, throwing rings, and cast shadow applied.")

--@ chunk 51
local b_f10 = brush("filbert", 10)
local b_r6  = brush{kind = "round", width = 6.0, point = 0.8}
local b_r3  = brush{kind = "round", width = 3.0, point = 0.9}
local b_r1  = brush{kind = "round", width = 1.4, point = 0.98}
local b_badger = brush("badger", 16)

-- 1. SUBSTANTIAL PULLED CLAY HANDLE
-- Thick clay core (width 7-10 units)
local pts_handle_mass = {
  {334, 320}, {310, 332}, {280, 355}, {255, 385}, {248, 415}, {255, 445}, {270, 475}, {282, 490}
}
b_r6:load(p_terracotta_mid, 0.95)
b_r6:stroke(pts_handle_mass, {pressure = {0.7, 0.95, 0.7}, swell = {0.9, 1.2, 0.9}})

-- Handle shadow underside
local pts_handle_dark = {
  {330, 328}, {302, 342}, {274, 368}, {250, 400}, {245, 425}, {252, 452}, {266, 480}
}
b_r3:load(p_terracotta_deep, 0.95)
b_r3:stroke(pts_handle_dark, {pressure = 0.85})

-- Handle lit outer crest
local pts_handle_lit = {
  {336, 316}, {312, 326}, {284, 350}, {258, 380}, {252, 410}, {258, 438}, {272, 470}, {284, 485}
}
b_r3:load(p_terracotta_lit, 0.95)
b_r3:stroke(pts_handle_lit, {pressure = 0.85})

-- Bright specular glint on the crest curve
b_r1:load(p_glaze_highlight, 1.0)
b_r1:stroke({{256, 395}, {253, 412}, {257, 430}}, {pressure = {0.4, 0.9, 0.4}})

-- Negative space inside loop cleanly cut with dark wall paint
b_r3:load(p_wall_dark, 1.0)
b_r3:stroke({{268, 385}, {295, 400}, {305, 435}, {285, 465}}, {pressure = 0.9})
b_r3:stroke({{280, 370}, {305, 405}, {310, 440}}, {pressure = 0.9})

-- 2. ENRICH AND UNIFY THE PITCHER'S CORE SHADOW & ROUNDNESS
-- Rich warm core shadow on belly
b_f10:load(p_terracotta_core, 0.85)
b_f10:stroke({{390, 350}, {415, 405}, {440, 465}, {445, 515}, {430, 565}, {405, 605}}, {pressure = {0.5, 0.85, 0.5}})

-- Warm reflected bounce light along right edge
b_f10:load(p_jug_reflect, 0.85)
b_f10:stroke({{430, 380}, {455, 435}, {468, 485}, {460, 535}, {440, 580}, {420, 610}}, {pressure = {0.4, 0.7, 0.4}})

-- Soft horizontal badger blend across the belly to melt any facets into spherical volume
b_badger:stroke({{290, 450}, {360, 455}, {440, 455}}, {pressure = 0.25})
b_badger:stroke({{290, 510}, {370, 515}, {450, 515}}, {pressure = 0.25})
b_badger:stroke({{310, 565}, {370, 570}, {430, 570}}, {pressure = 0.25})

-- Restate shoulder glaze impasto
b_r3:load(p_glaze_highlight, 1.0)
b_r3:stroke({{322, 395}, {319, 415}, {316, 430}}, {pressure = {0.5, 0.95, 0.4}})
b_r1:load(p_glaze_highlight, 1.0)
b_r1:touch(320, 412, {pressure = 0.95})

print("Handle made substantial and belly roundness enriched.")

--@ chunk 52
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r4  = brush{kind = "round", width = 4.5, point = 0.85}
local b_r2  = brush{kind = "round", width = 2.2, point = 0.95}
local b_badger = brush("badger", 16)

-- 1. Paint out the stray scratches on the pitcher's lit belly with solid terracotta
b_f10:load(p_terracotta_lit, 0.95)
b_f10:stroke({{275, 410}, {295, 435}, {310, 470}}, {pressure = 0.9})
b_f10:stroke({{280, 440}, {305, 465}, {320, 495}}, {pressure = 0.9})

-- Blend the lit belly smoothly into the midtone
b_f14:load(p_terracotta_mid, 0.9)
b_f14:stroke({{310, 360}, {330, 410}, {340, 470}, {335, 520}}, {pressure = 0.85})

-- 2. Solid, sculptural clay handle
-- First clean the negative space inside the handle loop with solid dark wall paint
b_r4:load(p_wall_dark, 1.0)
b_r4:stroke({{275, 345}, {295, 370}, {305, 405}, {285, 435}}, {pressure = 0.95})
b_r4:stroke({{265, 360}, {285, 385}, {290, 420}}, {pressure = 0.95})

-- Flared clay anchor at top neck (x ~ 325-340, y ~ 310-330)
b_r4:load(p_terracotta_lit, 0.95)
b_r4:stroke({{338, 312}, {320, 320}, {290, 340}, {262, 370}}, {pressure = 0.9})

-- Flared clay anchor at bottom belly (x ~ 265-280, y ~ 445-475)
b_r4:load(p_terracotta_mid, 0.95)
b_r4:stroke({{252, 420}, {258, 445}, {270, 470}}, {pressure = 0.9})

-- Handle body: thick ribbon cross section
-- Lit top edge
b_r2:load(p_terracotta_lit, 1.0)
b_r2:stroke({{336, 314}, {305, 326}, {270, 350}, {248, 382}, {246, 412}, {254, 442}, {268, 470}}, {
  pressure = {0.7, 0.95, 0.7}
})

-- Shaded under-edge of handle
b_r2:load(p_terracotta_deep, 1.0)
b_r2:stroke({{330, 324}, {300, 336}, {266, 362}, {242, 392}, {240, 420}, {248, 450}, {262, 475}}, {
  pressure = {0.7, 0.95, 0.7}
})

-- Specular catchlight on handle curve apex
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}
b_r1:load(p_glaze_highlight, 1.0)
b_r1:stroke({{248, 395}, {247, 412}, {251, 425}}, {pressure = {0.4, 0.85, 0.4}})

-- 3. Smooth the neck and core shadow facets
b_badger:stroke({{335, 300}, {365, 305}, {395, 300}}, {pressure = 0.3})
b_badger:stroke({{370, 370}, {405, 375}, {445, 370}}, {pressure = 0.3})
b_badger:stroke({{375, 430}, {420, 435}, {460, 430}}, {pressure = 0.3})
b_badger:stroke({{380, 490}, {425, 495}, {460, 490}}, {pressure = 0.3})

-- Rich glaze impasto on shoulder
b_r4:load(p_terracotta_hi, 0.95)
b_r4:stroke({{330, 375}, {326, 410}, {320, 445}}, {pressure = {0.6, 0.9, 0.5}, swell = {0.8, 1.3, 0.8}})
b_r2:load(p_glaze_highlight, 1.0)
b_r2:stroke({{328, 395}, {325, 415}, {322, 430}}, {pressure = {0.5, 0.95, 0.4}})
b_r1:load(p_glaze_highlight, 1.0)
b_r1:touch(326, 412, {pressure = 0.95})

print("Handle reworked, scratches removed, modeling unified.")

--@ chunk 53
local pts_jug_body = {
  {338, 268}, {340, 310}, {336, 340}, {302, 385}, {270, 440}, {266, 485},
  {280, 540}, {312, 590}, {326, 615}, {420, 615}, {435, 590}, {458, 540},
  {472, 485}, {468, 440}, {438, 385}, {405, 340}, {402, 310}, {404, 268}
}
local m_jug = poly(pts_jug_body, true)

-- 1. Blend the pitcher body to fuse open wet strokes into smooth roundness
blend(m_jug, {angle = 0, coverage = 1.2})

-- 2. Negative space inside the handle loop: solid dark wall paint
local pts_hole = {
  {325, 332}, {295, 350}, {268, 382}, {262, 420}, {274, 452}, {290, 415}, {312, 375}, {328, 340}
}
work(poly(pts_hole, true), {hand = "body", pile = p_wall_dark, coverage = 3.0, fill = true, edge = "firm"})

-- 3. Sculpt the handle cleanly over the negative space
local pts_handle_spine = {
  {338, 314}, {306, 326}, {268, 354}, {246, 388}, {244, 420}, {254, 452}, {272, 480}
}
-- Lit top edge
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
b_r3:load(p_terracotta_lit, 1.0)
b_r3:stroke(pts_handle_spine, {pressure = {0.7, 0.95, 0.7}})

-- Under-edge shadow of handle
local pts_handle_under = {
  {332, 324}, {300, 336}, {262, 364}, {240, 396}, {238, 426}, {248, 458}, {266, 485}
}
b_r3:load(p_terracotta_deep, 1.0)
b_r3:stroke(pts_handle_under, {pressure = {0.6, 0.9, 0.6}})

-- Glaze catchlight on handle apex
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}
b_r1:load(p_glaze_highlight, 1.0)
b_r1:stroke({{246, 396}, {244, 412}, {248, 426}}, {pressure = {0.4, 0.9, 0.4}})

-- 4. Shoulder glaze impasto
local b_r4 = brush{kind = "round", width = 4.2, point = 0.85}
b_r4:load(p_terracotta_hi, 0.95)
b_r4:stroke({{328, 380}, {324, 412}, {318, 442}}, {pressure = {0.6, 0.9, 0.5}, swell = {0.8, 1.3, 0.8}})

local b_r2 = brush{kind = "round", width = 2.2, point = 0.95}
b_r2:load(p_glaze_highlight, 1.0)
b_r2:stroke({{326, 398}, {323, 416}, {320, 430}}, {pressure = {0.5, 0.95, 0.4}})
b_r1:load(p_glaze_highlight, 1.0)
b_r1:touch(324, 414, {pressure = 0.95})

print("Pitcher body blended, handle perfected, glaze highlight restated.")

--@ chunk 54
-- 1. TOUCH UP PITCHER CONTOURS
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.0, point = 0.9}

-- Clean belly contour under handle (cover dark smudge)
b_f10:load(p_terracotta_lit, 0.95)
b_f10:stroke({{265, 445}, {282, 415}, {305, 380}, {325, 345}}, {pressure = 0.9})
b_f10:stroke({{278, 435}, {298, 405}, {318, 375}}, {pressure = 0.85})

-- Clean right neck contour
b_f10:load(p_wall_mid, 1.0)
b_f10:stroke({{404, 335}, {406, 365}, {418, 385}}, {pressure = 0.9})

-- 2. MIX PALETTE PILES FOR THE QUINCE
p_quince_base   = pile{{"yellow ochre", 5.5}, {"green earth", 2.5}, {"raw umber", 1.8}, {"lead white", 1.5}, medium = 0.06}
p_quince_lit    = pile{{"yellow ochre", 5.0}, {"lead white", 3.8}, {"chrome yellow", 3.0}, {"red earth", 0.4}, medium = 0.05}
p_quince_crest  = pile{{"lead white", 6.0}, {"chrome yellow", 4.0}, {"yellow ochre", 2.5}, medium = 0.04}
p_quince_hi     = pile{{"lead white", 8.5}, {"chrome yellow", 2.5}, {"yellow ochre", 1.0}, medium = 0.03}
p_quince_blush  = pile{{"red earth", 4.0}, {"yellow ochre", 4.5}, {"raw umber", 1.5}, medium = 0.06}
p_quince_core   = pile{{"raw umber", 5.5}, {"yellow ochre", 3.0}, {"red earth", 1.5}, {"bone black", 0.8}, medium = 0.08}
p_quince_deep   = pile{{"raw umber", 6.5}, {"bone black", 2.5}, {"red earth", 1.5}, medium = 0.07}
p_stem_wood     = pile{{"raw umber", 6.5}, {"bone black", 3.0}, {"red earth", 1.5}, medium = 0.05}

-- 3. LAY IN THE QUINCE SILHOUETTE
local pts_quince = {
  {625, 475}, {642, 475}, {658, 495}, {675, 525}, {696, 560},
  {708, 600}, {702, 635}, {684, 665}, {660, 675}, {620, 677},
  {585, 672}, {562, 648}, {550, 615}, {552, 580}, {572, 542},
  {596, 512}, {615, 485}
}
local m_quince = poly(pts_quince, true)

-- Solid opaque body foundation
work(m_quince, {
  hand = "body",
  pile = p_quince_base,
  angle = 1.35,
  coverage = 3.0,
  fill = true,
  edge = "firm"
})

-- 4. SCULPT THE KNOBBY BULGES AND chiaroscuro WET-IN-WET
local b_f14 = brush("filbert", 14)
local b_r4  = brush{kind = "round", width = 4.5, point = 0.85}
local b_r2  = brush{kind = "round", width = 2.4, point = 0.95}

-- Shaded right flank (turning into shadow)
b_f14:load(p_quince_core, 0.95)
b_f14:stroke({{648, 488}, {668, 528}, {686, 575}, {692, 620}, {675, 660}, {650, 674}}, {
  pressure = {0.8, 0.98, 0.8}
})

-- Deep core terminator line
b_f10:load(p_quince_deep, 0.9)
b_f10:stroke({{642, 505}, {658, 550}, {670, 600}, {668, 645}, {645, 672}}, {
  pressure = {0.6, 0.85, 0.6}
})

-- Illuminated undulating lobes on left flank
b_f14:load(p_quince_lit, 0.98)
-- Upper neck swell
b_f14:stroke({{620, 482}, {605, 510}, {590, 540}, {605, 570}}, {pressure = {0.7, 0.95, 0.7}})
-- Main sunlit belly bulge
b_f14:stroke({{575, 550}, {560, 585}, {562, 625}, {585, 655}, {618, 672}}, {
  pressure = {0.8, 0.98, 0.8}
})
-- Center-lit lobe
b_f14:stroke({{605, 530}, {602, 575}, {615, 625}, {632, 665}}, {pressure = {0.8, 0.95, 0.7}})

-- Warm russet/cider blush stippled across the sunlit crest
local b_stip = brush("stippler", 6)
b_stip:load(p_quince_blush, 0.7)
b_stip:stroke({{585, 565}, {595, 595}, {610, 620}}, {pressure = 0.5, shake = 0.6})
b_stip:stroke({{605, 550}, {620, 580}, {630, 610}}, {pressure = 0.45, shake = 0.6})

-- Soft badger sweep around the organic contours to marry the lobes
local b_badger = brush("badger", 16)
b_badger:stroke({{570, 580}, {615, 590}, {670, 590}}, {pressure = 0.3})
b_badger:stroke({{580, 630}, {625, 640}, {680, 635}}, {pressure = 0.3})

-- 5. LUMINOUS CREST HIGHLIGHT
b_r4:load(p_quince_crest, 0.98)
b_r4:stroke({{585, 565}, {582, 595}, {592, 620}}, {pressure = {0.6, 0.9, 0.5}})

-- Rich impasto catchlight on the sunlit knobby crest
local b_r1 = brush{kind = "round", width = 1.6, point = 0.98}
b_r1:load(p_quince_hi, 1.0)
b_r1:stroke({{580, 580}, {578, 598}, {584, 612}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:touch(579, 592, {pressure = 0.95})

-- 6. GNARLED WOODY STEM AND CALYX
-- Stem at apex
b_r2:load(p_stem_wood, 1.0)
b_r2:stroke({{632, 480}, {635, 465}, {642, 452}, {648, 444}}, {pressure = {0.9, 0.7, 0.4}})
-- Stem highlight
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{634, 478}, {637, 465}, {643, 454}}, {pressure = 0.5})

-- Deep contact shadow beneath the quince
b_f10:load(p_quince_deep, 1.0)
b_f10:stroke({{575, 672}, {620, 678}, {665, 676}, {685, 668}}, {pressure = {0.7, 0.98, 0.6}})

-- Quince cast shadow trailing to the right on the stone table
b_f14:load(p_cast_shadow, 0.65)
b_f14:stroke({{660, 674}, {710, 680}, {765, 686}}, {pressure = {0.6, 0.8, 0.3}, swell = {0.9, 1.3, 0.7}})
b_badger:stroke({{680, 678}, {735, 684}}, {pressure = 0.25})

print("Quince modeled with organic knobby lobes, blush, and tactile impasto.")

--@ chunk 55
-- Piles for glowing golden Spanish quince
p_gold_quince = pile{{"yellow ochre", 5.5}, {"chrome yellow", 4.0}, {"lead white", 3.0}, {"red earth", 0.5}, medium = 0.05}
p_gold_lit    = pile{{"chrome yellow", 5.0}, {"lead white", 5.0}, {"yellow ochre", 2.0}, medium = 0.04}
p_gold_hi     = pile{{"lead white", 8.5}, {"chrome yellow", 2.5}, {"yellow ochre", 0.8}, medium = 0.03}
p_gold_shade  = pile{{"raw umber", 4.5}, {"yellow ochre", 4.0}, {"red earth", 2.0}, {"bone black", 0.6}, medium = 0.06}
p_gold_deep   = pile{{"raw umber", 6.0}, {"bone black", 2.0}, {"red earth", 2.0}, medium = 0.07}
p_gold_blush  = pile{{"red earth", 4.5}, {"yellow ochre", 4.0}, {"vermilion", 1.0}, {"raw umber", 1.0}, medium = 0.05}

local pts_quince_full = {
  {615, 470}, {640, 468}, {660, 482}, {680, 515}, {702, 555},
  {714, 600}, {708, 638}, {688, 668}, {660, 676}, {615, 678},
  {575, 672}, {550, 645}, {540, 610}, {544, 572}, {565, 535},
  {590, 502}, {608, 480}
}
local m_quince_full = poly(pts_quince_full, true)

-- 1. Full opaque golden body foundation
work(m_quince_full, {
  hand = "body",
  pile = p_gold_quince,
  angle = 1.3,
  coverage = 3.0,
  fill = true,
  edge = "firm"
})

local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r4  = brush{kind = "round", width = 4.5, point = 0.85}
local b_r2  = brush{kind = "round", width = 2.2, point = 0.95}
local b_badger = brush("badger", 16)

-- 2. Shaded flank and core shadow
b_f14:load(p_gold_shade, 0.95)
b_f14:stroke({{655, 480}, {678, 520}, {696, 570}, {702, 615}, {682, 655}, {655, 674}}, {
  pressure = {0.8, 0.98, 0.8}
})

b_f10:load(p_gold_deep, 0.9)
b_f10:stroke({{648, 500}, {665, 545}, {676, 595}, {672, 640}, {648, 670}}, {
  pressure = {0.6, 0.85, 0.6}
})

-- 3. Illuminated voluptuous lobes on left flank
b_f14:load(p_gold_lit, 0.98)
-- Upper shoulder lobe
b_f14:stroke({{625, 475}, {605, 505}, {585, 535}, {600, 565}}, {pressure = {0.7, 0.95, 0.7}})
-- Main sunlit belly lobe
b_f14:stroke({{570, 545}, {550, 580}, {552, 620}, {575, 655}, {615, 674}}, {
  pressure = {0.85, 0.98, 0.85}
})
-- Center lobe
b_f14:stroke({{600, 520}, {595, 565}, {608, 620}, {630, 665}}, {pressure = {0.8, 0.95, 0.7}})

-- 4. Warm cider/vermilion blush stippled across the sunlit lobe
local b_stip = brush("stippler", 6)
b_stip:load(p_gold_blush, 0.75)
b_stip:stroke({{575, 560}, {588, 595}, {605, 625}}, {pressure = 0.5, shake = 0.5})
b_stip:stroke({{595, 540}, {612, 575}, {622, 610}}, {pressure = 0.45, shake = 0.5})

-- 5. Soft badger blend around the circumference to turn the form seamlessly
b_badger:stroke({{560, 575}, {610, 585}, {680, 585}}, {pressure = 0.3})
b_badger:stroke({{570, 625}, {620, 635}, {690, 630}}, {pressure = 0.3})

-- 6. Luminous impasto crest highlight
b_r4:load(p_gold_lit, 1.0)
b_r4:stroke({{578, 565}, {572, 595}, {582, 622}}, {pressure = {0.7, 0.95, 0.6}, swell = {0.8, 1.3, 0.7}})

local b_r1 = brush{kind = "round", width = 1.6, point = 0.98}
b_r1:load(p_gold_hi, 1.0)
b_r1:stroke({{574, 578}, {570, 598}, {576, 614}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.8}})
b_r1:touch(571, 595, {pressure = 0.95})

-- 7. Gnarled woody stem and depression at crown
-- Crown socket shadow
b_r2:load(p_stem_wood, 1.0)
b_r2:stroke({{622, 476}, {632, 473}, {642, 475}}, {pressure = 0.8})

-- Woody stem curving up
b_r2:load(p_stem_wood, 1.0)
b_r2:stroke({{630, 474}, {633, 460}, {640, 448}, {646, 440}}, {pressure = {0.9, 0.75, 0.4}})
b_r1:load(p_gold_lit, 0.9)
b_r1:stroke({{632, 472}, {635, 460}, {641, 450}}, {pressure = 0.5})

-- 8. Grounding contact shadow under quince
b_f10:load(p_gold_deep, 1.0)
b_f10:stroke({{565, 672}, {615, 678}, {665, 676}, {695, 668}}, {pressure = {0.7, 0.98, 0.6}})

print("Quince reworked in radiant golden Spanish palette.")

--@ chunk 56
local pts_quince_full = {
  {615, 470}, {640, 468}, {660, 482}, {680, 515}, {702, 555},
  {714, 600}, {708, 638}, {688, 668}, {660, 676}, {615, 678},
  {575, 672}, {550, 645}, {540, 610}, {544, 572}, {565, 535},
  {590, 502}, {608, 480}
}
local m_quince_full = poly(pts_quince_full, true)

-- 1. Blend the open wet paint across the entire quince to marry stipple texture and shadow into a smooth turning volume
blend(m_quince_full, {angle = 0.2, coverage = 1.2})

-- 2. Restate the lit crest with broad, confident strokes
local b_f10 = brush("filbert", 10)
b_f10:load(p_gold_lit, 0.95)
b_f10:stroke({{575, 560}, {565, 595}, {578, 630}}, {pressure = {0.6, 0.9, 0.5}})

-- Buttery impasto highlight on the most prominent sunlit bulge
local b_r3 = brush{kind = "round", width = 3.0, point = 0.9}
local b_r1 = brush{kind = "round", width = 1.4, point = 0.98}
b_r3:load(p_gold_hi, 1.0)
b_r3:stroke({{572, 580}, {568, 598}, {574, 615}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_gold_hi, 1.0)
b_r1:touch(569, 596, {pressure = 0.95})

-- 3. Gnarled woody stem
local b_r2 = brush{kind = "round", width = 2.2, point = 0.95}
b_r2:load(p_stem_wood, 1.0)
b_r2:stroke({{630, 474}, {633, 460}, {640, 448}, {646, 440}}, {pressure = {0.9, 0.75, 0.4}})
b_r1:load(p_gold_lit, 0.9)
b_r1:stroke({{632, 472}, {635, 460}, {641, 450}}, {pressure = 0.5})

-- Contact shadow touch
b_r3:load(p_gold_deep, 1.0)
b_r3:stroke({{580, 674}, {620, 678}, {665, 676}}, {pressure = {0.6, 0.9, 0.6}})

print("Quince blended into smooth volume and impasto restated.")

--@ chunk 57
-- 1. TOUCH UP PITCHER CONTOUR
local b_r3 = brush{kind = "round", width = 3.0, point = 0.9}
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}

-- Clean that small brown tab on the right neck (x: 405, y: 350-370) with wall paint
b_r3:load(p_wall_mid, 1.0)
b_r3:stroke({{404, 345}, {404, 375}, {412, 385}}, {pressure = 0.95})

-- 2. MIX PALETTE PILES FOR WALNUTS & KNIFE
p_shell_wood   = pile{{"red earth", 4.5}, {"yellow ochre", 4.0}, {"raw umber", 3.0}, {"lead white", 1.2}, medium = 0.06}
p_shell_dark   = pile{{"raw umber", 6.5}, {"bone black", 2.5}, {"red earth", 2.0}, medium = 0.07}
p_shell_rim    = pile{{"lead white", 7.0}, {"yellow ochre", 3.0}, {"raw umber", 1.0}, medium = 0.05}
p_kernel_meat  = pile{{"lead white", 8.2}, {"yellow ochre", 2.8}, {"raw umber", 0.8}, medium = 0.04}
p_kernel_hi    = pile{{"lead white", 9.5}, {"yellow ochre", 1.0}, medium = 0.03}

p_steel_body   = pile{{"lead white", 6.5}, {"bone black", 2.0}, {"raw umber", 1.0}, {"cobalt blue", 0.5}, medium = 0.06}
p_steel_edge   = pile{{"lead white", 9.5}, {"bone black", 0.3}, medium = 0.03}
p_brass        = pile{{"yellow ochre", 6.0}, {"raw umber", 2.5}, {"lead white", 2.0}, {"red earth", 0.8}, medium = 0.05}
p_horn_wood    = pile{{"bone black", 5.5}, {"raw umber", 3.5}, {"red earth", 1.5}, medium = 0.06}
p_horn_hi      = pile{{"raw umber", 4.5}, {"yellow ochre", 3.0}, {"lead white", 2.5}, medium = 0.05}

-- 3. PAINT THE WALNUTS (x ~ 460 to 535, y ~ 645 to 675)
-- Cast shadow under cracked walnut
local b_f10 = brush("filbert", 10)
b_f10:load(p_cast_shadow, 0.7)
b_f10:stroke({{460, 655}, {480, 660}, {500, 658}}, {pressure = {0.5, 0.75, 0.3}})

-- Outer woody shell cup of the cracked walnut
local pts_wn_bowl = {
  {462, 642}, {458, 650}, {464, 658}, {480, 662}, {496, 658}, {498, 646}
}
b_r3:load(p_shell_wood, 1.0)
b_r3:stroke(pts_wn_bowl, {pressure = 0.85})

-- Dark interior hollow behind kernel
b_r2:load(p_shell_dark, 0.95)
b_r2:stroke({{465, 646}, {478, 652}, {492, 648}}, {pressure = 0.8})

-- Convoluted ivory walnut kernel lobes (chunky impasto)
b_r2:load(p_kernel_meat, 1.0)
-- Left lobe
b_r2:stroke({{467, 644}, {470, 649}, {468, 655}}, {pressure = {0.7, 0.95, 0.6}})
b_r2:stroke({{472, 643}, {474, 648}, {473, 654}}, {pressure = {0.6, 0.9, 0.5}})
-- Right lobe
b_r2:stroke({{478, 643}, {480, 648}, {479, 654}}, {pressure = {0.7, 0.95, 0.6}})
b_r2:stroke({{484, 644}, {487, 649}, {485, 655}}, {pressure = {0.6, 0.9, 0.5}})

-- Central dividing septum and crevices
b_r1:load(p_shell_dark, 0.95)
b_r1:stroke({{476, 642}, {476, 650}, {475, 656}}, {pressure = 0.6})
b_r1:stroke({{465, 652}, {468, 656}}, {pressure = 0.45})

-- Buttery impasto highlights on kernel convolutions
b_r1:load(p_kernel_hi, 1.0)
b_r1:touch(469, 645, {pressure = 0.85})
b_r1:touch(473, 650, {pressure = 0.75})
b_r1:touch(480, 644, {pressure = 0.85})
b_r1:touch(485, 650, {pressure = 0.75})

-- Bright shell rim catching light
b_r1:load(p_shell_rim, 0.98)
b_r1:stroke({{461, 642}, {472, 638}, {486, 638}}, {pressure = {0.5, 0.85, 0.4}})

-- Whole Walnut (x ~ 512, y ~ 660)
-- Cast shadow
b_f10:load(p_cast_shadow, 0.7)
b_f10:stroke({{500, 668}, {522, 672}, {544, 668}}, {pressure = {0.4, 0.75, 0.3}})

-- Whole walnut body
local pts_wn_w = {
  {502, 655}, {514, 646}, {528, 648}, {538, 658}, {532, 668}, {512, 668}, {500, 660}
}
work(poly(pts_wn_w, true), {hand = "body", pile = p_shell_wood, coverage = 2.8, fill = true, edge = "firm"})

-- Shadow flank on right
b_r2:load(p_shell_dark, 0.9)
b_r2:stroke({{522, 650}, {532, 658}, {528, 667}}, {pressure = {0.5, 0.85, 0.4}})

-- Central suture ridge
b_r1:load(p_shell_dark, 0.95)
b_r1:stroke({{514, 646}, {517, 657}, {518, 668}}, {pressure = {0.4, 0.7, 0.4}})

-- Corrugated shell ridges catching raking light
b_r1:load(p_shell_rim, 0.9)
b_r1:stroke({{506, 652}, {510, 658}, {512, 666}}, {pressure = {0.4, 0.75, 0.3}})
b_r1:stroke({{523, 652}, {525, 660}, {524, 666}}, {pressure = {0.3, 0.6, 0.25}})

-- 4. PAINT THE BODEGÓN KNIFE (x ~ 695 to 885, y ~ 675 to 760)
-- Steel blade cast shadow onto stone
b_r3:load(p_cast_shadow, 0.6)
b_r3:stroke({{705, 676}, {745, 692}, {785, 708}}, {pressure = {0.4, 0.65, 0.3}})

-- Steel blade body
local pts_blade = {
  {695, 672}, {782, 706}, {780, 714}, {708, 680}
}
work(poly(pts_blade, true), {hand = "body", pile = p_steel_body, coverage = 2.6, fill = true, edge = "firm"})

-- Razor-sharp cutting edge catching brilliant specular line
b_r1:load(p_steel_edge, 1.0)
b_r1:stroke({{695, 672}, {740, 689}, {782, 706}}, {pressure = {0.6, 1.0, 0.7}, swell = {0.8, 1.2, 0.8}})

-- Brass bolster
local pts_bolster = {
  {782, 705}, {794, 709}, {791, 717}, {779, 713}
}
work(poly(pts_bolster, true), {hand = "body", pile = p_brass, coverage = 2.6, fill = true, edge = "firm"})
b_r1:load(p_steel_edge, 1.0)
b_r1:stroke({{783, 706}, {794, 709}}, {pressure = 0.7})

-- Turned horn/wood handle extending forward over the stone ledge
local pts_handle = {
  {794, 709}, {825, 722}, {855, 735}, {878, 746},
  {874, 755}, {850, 746}, {820, 732}, {791, 717}
}
work(poly(pts_handle, true), {hand = "body", pile = p_horn_wood, coverage = 2.8, fill = true, edge = "firm"})

-- Specular raking light on the turned handle curve
b_r2:load(p_horn_hi, 0.95)
b_r2:stroke({{795, 710}, {825, 723}, {855, 736}, {875, 746}}, {pressure = {0.4, 0.85, 0.3}})

-- Brass rivets on handle
b_r1:load(p_brass, 1.0)
b_r1:touch(818, 723, {pressure = 0.7})
b_r1:touch(848, 737, {pressure = 0.7})

-- Cast shadow of the projecting handle falling down the vertical face of the stone ledge
local b_f14 = brush("filbert", 14)
b_f14:load(p_cast_shadow, 0.85)
b_f14:stroke({{805, 720}, {830, 742}, {855, 762}}, {pressure = {0.6, 0.85, 0.3}})

-- Crisp front ledge chamfer light re-established
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{0, 715}, {400, 715}, {780, 715}}, {pressure = {0.6, 0.85, 0.6}, shake = 0.2})
b_r1:stroke({{880, 715}, {1000, 715}}, {pressure = {0.6, 0.85, 0.6}, shake = 0.2})

print("Walnuts and bodegón knife painted with tactile details.")

--@ chunk 58
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}
local b_badger = brush("badger", 16)
local b_flat = brush("flat", 24)

-- 1. CLEAN THE PITCHER
-- Clean the tab/spur on right shoulder (x ~ 405-430, y ~ 330-365) with wall paint
b_r3:load(p_wall_mid, 1.0)
b_r3:stroke({{404, 325}, {406, 350}, {415, 375}, {435, 385}}, {pressure = 0.98})
b_r3:stroke({{412, 335}, {420, 355}, {432, 370}}, {pressure = 0.98})

-- Clean the negative space inside the handle loop with solid dark wall paint
local pts_handle_hole = {
  {330, 328}, {300, 346}, {270, 378}, {258, 415}, {266, 445}, {278, 430}, {295, 395}, {318, 360}, {332, 335}
}
work(poly(pts_handle_hole, true), {hand = "body", pile = p_wall_dark, coverage = 3.2, fill = true, edge = "firm"})

-- Handle ribbon: lit top crest, shaded underside
local pts_handle_crest = {
  {338, 314}, {306, 326}, {268, 354}, {246, 388}, {244, 420}, {254, 452}, {272, 480}
}
b_r3:load(p_terracotta_lit, 1.0)
b_r3:stroke(pts_handle_crest, {pressure = {0.7, 0.95, 0.7}})

local pts_handle_under = {
  {332, 324}, {300, 336}, {262, 364}, {240, 396}, {238, 426}, {248, 458}, {266, 485}
}
b_r2:load(p_terracotta_deep, 1.0)
b_r2:stroke(pts_handle_under, {pressure = 0.85})

-- Handle specular gleam
b_r1:load(p_glaze_highlight, 1.0)
b_r1:stroke({{246, 395}, {244, 412}, {248, 426}}, {pressure = {0.4, 0.9, 0.4}})

-- Deepen core shadow on pitcher's right belly for massive cylindrical weight
b_f10:load(p_terracotta_deep, 0.95)
b_f10:stroke({{400, 360}, {425, 410}, {448, 465}, {452, 515}, {438, 565}, {412, 608}}, {
  pressure = {0.7, 0.95, 0.7}
})

-- Warm reflected bounce light along right contour
b_f10:load(p_jug_reflect, 0.85)
b_f10:stroke({{435, 410}, {462, 465}, {466, 515}, {455, 560}, {430, 605}}, {
  pressure = {0.4, 0.7, 0.4}
})

-- Blend belly wet-in-wet
b_badger:stroke({{330, 465}, {390, 470}, {455, 468}}, {pressure = 0.25})
b_badger:stroke({{330, 520}, {395, 525}, {455, 522}}, {pressure = 0.25})

-- Shoulder glaze impasto
b_r3:load(p_glaze_highlight, 1.0)
b_r3:stroke({{326, 395}, {323, 415}, {320, 430}}, {pressure = {0.5, 0.95, 0.4}})
b_r1:load(p_glaze_highlight, 1.0)
b_r1:touch(324, 413, {pressure = 0.95})

-- 2. RE-ESTABLISH THE FOREGROUND LIMESTONE TABLE PLANE
-- Paint over the walnut spot and thin knife marks with solid, warm limestone body color
for y = 645, 710, 16 do
  b_f14:load(p_stone_lit, 0.98)
  b_f14:stroke({{430, y}, {600, y}, {750, y}, {980, y}}, {pressure = {0.8, 0.95, 0.8}})
end

-- Feather the stone across the table plane
b_badger:stroke({{450, 655}, {700, 655}, {950, 655}}, {pressure = 0.25})
b_badger:stroke({{450, 690}, {700, 690}, {950, 690}}, {pressure = 0.25})

-- Re-anchor the base of the quince cleanly
b_r3:load(p_gold_deep, 1.0)
b_r3:stroke({{575, 674}, {615, 678}, {665, 676}, {688, 668}}, {pressure = {0.7, 0.98, 0.6}})

-- Quince cast shadow trailing to the right
b_f10:load(p_cast_shadow, 0.65)
b_f10:stroke({{660, 674}, {705, 678}, {755, 682}}, {pressure = {0.5, 0.75, 0.3}, swell = {0.8, 1.2, 0.7}})

-- Pitcher cast shadow trailing across the limestone table
b_f10:load(p_cast_shadow, 0.65)
b_f10:stroke({{415, 616}, {465, 624}, {525, 632}, {570, 638}}, {pressure = {0.6, 0.75, 0.3}, swell = {0.9, 1.3, 0.7}})

-- Re-establish the clean, dark vertical face of the stone ledge
for y = 720, 795, 18 do
  b_flat:load(p_ledge_face, 0.98)
  b_flat:stroke({{700, y}, {850, y}, {1000, y}}, {pressure = 0.95})
end

-- Sharp, luminous chamfer highlight along the ledge front rim (y = 715)
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{0, 715}, {350, 715}, {700, 715}, {1000, 715}}, {pressure = {0.6, 0.9, 0.6}, shake = 0.2})

print("Pitcher cleaned, table plane reset to solid limestone, ledge face unified.")

--@ chunk 59
local b_f16 = brush("filbert", 16)
local b_f10 = brush("filbert", 10)
local b_r4  = brush{kind = "round", width = 4.0, point = 0.85}
local b_flat = brush("flat", 24)

-- 1. CUT OUT THE TAB ON THE PITCHER'S RIGHT SHOULDER (x: 405-455, y: 315-375)
-- Fill with solid mid-wall paint
for y = 320, 370, 12 do
  b_f10:load(p_wall_mid, 1.0)
  b_f10:stroke({{404, y}, {430, y}, {460, y}}, {pressure = 0.98})
end
local b_badger = brush("badger", 14)
b_badger:stroke({{415, 345}, {455, 345}}, {pressure = 0.25})

-- Clean right neck contour with a crisp edge
local pts_neck_right = {{404, 270}, {403, 310}, {404, 340}, {420, 375}}
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_wall_mid, 1.0)
b_r2:stroke(pts_neck_right, {pressure = 0.95})

-- 2. REPAINT THE TABLE AROUND THE QUINCE (CLEAN LIMESTONE, NO DRAGGING)
-- A. Open table to the right of quince (x: 715 to 1000, y: 520 to 715)
for y = 530, 710, 18 do
  b_f16:load(p_stone_mid, 0.98)
  b_f16:stroke({{715, y}, {850, y}, {1000, y}}, {pressure = {0.8, 0.95, 0.8}})
end

-- B. Open table between pitcher and quince (x: 425 to 545, y: 620 to 678)
for y = 625, 675, 14 do
  b_f10:load(p_stone_lit, 0.98)
  b_f10:stroke({{425, y}, {485, y}, {540, y}}, {pressure = {0.8, 0.95, 0.8}})
end

-- C. Foreground slab in front of quince (x: 420 to 1000, y: 680 to 714)
for y = 682, 712, 14 do
  b_f16:load(p_stone_lit, 0.98)
  b_f16:stroke({{420, y}, {700, y}, {1000, y}}, {pressure = {0.85, 0.98, 0.85}})
end

-- Horizontal badger sweeps to marry the fresh stone into the table plane
b_badger:stroke({{720, 600}, {850, 600}, {980, 600}}, {pressure = 0.25})
b_badger:stroke({{720, 660}, {850, 660}, {980, 660}}, {pressure = 0.25})
b_badger:stroke({{430, 698}, {700, 698}, {980, 698}}, {pressure = 0.25})

-- 3. FRONT LEDGE FACE AND CHAMFER
for y = 720, 795, 18 do
  b_flat:load(p_ledge_face, 0.98)
  b_flat:stroke({{700, y}, {850, y}, {1000, y}}, {pressure = 0.95})
end

-- Sharp, crisp chamfer highlight along the ledge front edge (y = 715)
local b_r1 = brush{kind = "round", width = 1.6, point = 0.98}
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{0, 715}, {350, 715}, {700, 715}, {1000, 715}}, {pressure = {0.6, 0.9, 0.6}, shake = 0.2})

print("Pitcher tab removed, table plane repainted cleanly without dragging.")

--@ chunk 60
-- 1. FIX THE WALL PATCH TO THE RIGHT OF THE PITCHER'S NECK
-- The wall here must match p_wall_dark, not p_wall_mid
local m_neck_patch = rect(395, 290, 65, 90)
work(m_neck_patch, {
  hand = "body",
  pile = p_wall_dark,
  angle = 0.1,
  coverage = 3.0,
  fill = true,
  edge = "firm"
})
blend(rect(385, 280, 85, 110), {angle = 0})

-- 2. BLEND THE TABLETOP (STAYING INSIDE REGIONAL MASKS SO OBJECTS ARE UNTOUCHED)
-- A. The wide limestone table to the right of the quince (x: 705 to 1000, y: 520 to 715)
blend(rect(705, 520, 295, 195), {angle = 0, coverage = 1.2})

-- B. The foreground limestone slab in front of the objects (x: 0 to 1000, y: 680 to 715)
blend(rect(0, 680, 1000, 35), {angle = 0, coverage = 1.0})

-- C. Between the pitcher and quince (x: 425 to 545, y: 620 to 678)
blend(rect(425, 620, 120, 58), {angle = 0, coverage = 1.0})

-- 3. CRISP CHAMFER RIM RE-ESTABLISHED ALONG y = 715
local b_r1 = brush{kind = "round", width = 1.5, point = 0.98}
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{0, 715}, {350, 715}, {700, 715}, {1000, 715}}, {pressure = {0.6, 0.9, 0.6}, shake = 0.2})

print("Wall patch corrected to p_wall_dark, tabletop regions blended smoothly.")

--@ chunk 61
local b_badger = brush("badger", 32)

-- 1. Dissolve the square box on the wall beside the pitcher's neck
-- Horizontal badger sweeps across the boundary (x: 350 to 520, y: 280 to 390)
b_badger:stroke({{360, 300}, {480, 300}}, {pressure = 0.4})
b_badger:stroke({{360, 330}, {490, 330}}, {pressure = 0.45})
b_badger:stroke({{360, 360}, {490, 360}}, {pressure = 0.45})
b_badger:stroke({{370, 390}, {500, 390}}, {pressure = 0.4})

-- 2. Dissolve the box seams on the limestone tabletop
-- Horizontal badger sweeps across the table plane
b_badger:stroke({{400, 695}, {600, 695}, {850, 695}, {1000, 695}}, {pressure = 0.4})
b_badger:stroke({{650, 650}, {850, 650}, {1000, 650}}, {pressure = 0.4})
b_badger:stroke({{700, 580}, {850, 580}, {1000, 580}}, {pressure = 0.35})
b_badger:stroke({{700, 540}, {850, 540}, {1000, 540}}, {pressure = 0.35})

print("Badger sweeps across boundaries completed.")

--@ chunk 62
local t = wait(4 * 24 * 60)
print("Time after drying:", t)
print("Drying neck (370, 340):", drying(370, 340))
print("Drying wall (440, 340):", drying(440, 340))
print("Drying table (600, 680):", drying(600, 680))

--@ chunk 63
local t = wait(2 * 24 * 60)
print("Time after 2 more days:", t)
print("Drying wall (440, 340):", drying(440, 340))

--@ chunk 64
local b_f10 = brush("filbert", 10)
local b_f14 = brush("filbert", 14)
local b_r3  = brush{kind = "round", width = 3.0, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}

-- 1. REPAINT PITCHER NECK WITH OPAQUE VERTICAL FILBERT STROKES
-- Lit side of neck (x: 340 to 375, y: 270 to 360)
b_f10:load(p_terracotta_lit, 1.0)
b_f10:stroke({{348, 272}, {348, 310}, {345, 345}}, {pressure = {0.8, 0.95, 0.8}})
b_f10:stroke({{360, 274}, {360, 312}, {358, 350}}, {pressure = {0.8, 0.95, 0.8}})

-- Shaded side of neck (x: 375 to 404, y: 270 to 360)
b_f10:load(p_terracotta_mid, 1.0)
b_f10:stroke({{378, 274}, {378, 315}, {376, 355}}, {pressure = {0.8, 0.95, 0.8}})
b_f10:load(p_terracotta_deep, 1.0)
b_f10:stroke({{394, 273}, {395, 315}, {392, 355}, {408, 385}}, {pressure = {0.85, 0.98, 0.85}})

-- Soft blend down the neck
local b_badger = brush("badger", 14)
b_badger:stroke({{345, 310}, {375, 312}, {400, 310}}, {pressure = 0.3})
b_badger:stroke({{345, 345}, {375, 347}, {405, 345}}, {pressure = 0.3})

-- Lip highlight on left rim
b_r2:load(p_terracotta_hi, 1.0)
b_r2:stroke({{336, 268}, {355, 273}, {372, 274}}, {pressure = 0.85})

-- 2. REPAINT THE WALL TO THE RIGHT OF THE PITCHER'S NECK (x: 404 to 530, y: 275 to 420)
-- Sweeping horizontally and diagonally into the wall to cover the streaks
for y = 280, 410, 16 do
  b_f14:load(p_wall_mid, 0.98)
  b_f14:stroke({{406, y}, {460, y}, {520, y}}, {pressure = {0.85, 0.95, 0.75}})
end

-- Feather the wall out into the right side
b_badger:stroke({{450, 300}, {530, 300}}, {pressure = 0.3})
b_badger:stroke({{450, 350}, {540, 350}}, {pressure = 0.3})
b_badger:stroke({{450, 400}, {540, 400}}, {pressure = 0.3})

-- Crisp silhouette of the neck/shoulder right edge
b_r2:load(p_wall_mid, 1.0)
b_r2:stroke({{404, 270}, {403, 310}, {404, 345}, {418, 375}, {440, 405}}, {pressure = 0.95})

print("Pitcher neck repainted and wall behind it covered smoothly.")

--@ chunk 65
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}

-- 1. PITCHER: CONTINUOUS FLOWING MODELING FROM RIM DOWN ACROSS BELLY
-- Lit left flank (y: 270 to 490)
b_f14:load(p_terracotta_lit, 1.0)
b_f14:stroke({{345, 272}, {345, 330}, {330, 390}, {300, 450}, {295, 490}}, {
  pressure = {0.8, 0.95, 0.8, 0.6, 0.2}
})
b_f10:load(p_terracotta_lit, 1.0)
b_f10:stroke({{358, 274}, {358, 335}, {345, 395}, {320, 455}, {315, 495}}, {
  pressure = {0.8, 0.95, 0.8, 0.5, 0.15}
})

-- Center midtone (y: 270 to 490)
b_f14:load(p_terracotta_mid, 1.0)
b_f14:stroke({{375, 274}, {375, 335}, {370, 395}, {355, 455}, {350, 495}}, {
  pressure = {0.8, 0.95, 0.8, 0.5, 0.15}
})

-- Core shadow down the right flank (y: 270 to 520)
b_f14:load(p_terracotta_deep, 1.0)
b_f14:stroke({{395, 274}, {395, 335}, {410, 390}, {440, 450}, {442, 500}, {420, 550}}, {
  pressure = {0.8, 0.98, 0.9, 0.85, 0.4}
})

-- Table bounce along rightmost edge
b_f10:load(p_jug_reflect, 0.9)
b_f10:stroke({{402, 335}, {425, 385}, {460, 440}, {464, 495}, {445, 545}}, {
  pressure = {0.4, 0.75, 0.6, 0.4}
})

-- Soft badger blend across the belly to fuse the strokes into a turning sphere
local b_badger = brush("badger", 18)
b_badger:stroke({{330, 340}, {370, 345}, {405, 340}}, {pressure = 0.3})
b_badger:stroke({{310, 400}, {365, 405}, {425, 400}}, {pressure = 0.3})
b_badger:stroke({{290, 460}, {360, 465}, {440, 460}}, {pressure = 0.3})

-- Shoulder glaze impasto restatement
local b_r1 = brush{kind = "round", width = 1.4, point = 0.98}
b_r3:load(p_glaze_highlight, 1.0)
b_r3:stroke({{325, 390}, {322, 415}, {318, 435}}, {pressure = {0.5, 0.95, 0.4}})
b_r1:load(p_glaze_highlight, 1.0)
b_r1:touch(323, 413, {pressure = 0.95})

-- Lip highlight on left rim
b_r2:load(p_terracotta_hi, 1.0)
b_r2:stroke({{336, 268}, {355, 273}, {372, 274}}, {pressure = 0.85})

-- 2. BACKGROUND WALL: UNIFY WITH p_wall_dark ACROSS TO RIGHT
-- Sweeping all the way from the pitcher's right contour (x: 405) across to x: 750
for y = 275, 480, 18 do
  b_f14:load(p_wall_dark, 0.98)
  b_f14:stroke({{404, y}, {550, y}, {750, y}}, {pressure = {0.9, 0.95, 0.7}})
end

-- Feather across the wall with badger brush
b_badger:stroke({{450, 320}, {600, 320}, {750, 320}}, {pressure = 0.3})
b_badger:stroke({{450, 380}, {600, 380}, {750, 380}}, {pressure = 0.3})
b_badger:stroke({{450, 440}, {600, 440}, {750, 440}}, {pressure = 0.3})

-- Clean right silhouette of neck/shoulder
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke({{404, 270}, {403, 310}, {404, 340}, {418, 375}, {442, 410}}, {pressure = 0.95})

print("Pitcher modeled with continuous strokes, wall unified in p_wall_dark.")

--@ chunk 66
local pts_jug_body = {
  {338, 268}, {340, 310}, {336, 340}, {302, 385}, {270, 440}, {266, 485},
  {280, 540}, {312, 590}, {326, 615}, {420, 615}, {435, 590}, {458, 540},
  {472, 485}, {468, 440}, {438, 385}, {405, 340}, {402, 310}, {404, 268}
}
local m_jug = poly(pts_jug_body, true)

-- 1. Blend the pitcher body to fuse the ribbons into seamless ceramic roundness
blend(m_jug, {angle = 0.1, coverage = 1.5})

-- 2. Cut along the right silhouette of the pitcher with a continuous stroke of p_wall_dark
-- to slice off the blunt tooth ends and give a clean, sculptural contour
local pts_jug_right_contour = {
  {406, 265}, {404, 305}, {405, 340}, {422, 380}, {448, 425}, {472, 475}, {470, 510}, {455, 545}
}
local b_r3 = brush{kind = "round", width = 3.0, point = 0.9}
b_r3:load(p_wall_dark, 1.0)
b_r3:stroke(pts_jug_right_contour, {pressure = 0.95})

-- 3. Blend the background wall to the right of the contour
blend(rect(425, 265, 335, 255), {angle = 0, coverage = 1.2})

-- 4. Shoulder glaze impasto restatement
local b_r3_hi = brush{kind = "round", width = 3.2, point = 0.9}
local b_r1 = brush{kind = "round", width = 1.4, point = 0.98}
b_r3_hi:load(p_glaze_highlight, 1.0)
b_r3_hi:stroke({{325, 390}, {322, 415}, {318, 435}}, {pressure = {0.5, 0.95, 0.4}})
b_r1:load(p_glaze_highlight, 1.0)
b_r1:touch(323, 413, {pressure = 0.95})

-- Lip highlight
b_r3:load(p_terracotta_hi, 1.0)
b_r3:stroke({{336, 268}, {355, 273}, {372, 274}}, {pressure = 0.85})

print("Pitcher blended, right contour cut cleanly, wall blended.")

--@ chunk 67
local t = wait(5 * 24 * 60)
print("Time after drying:", t)
print("Drying pitcher (350, 400):", drying(350, 400))
print("Drying wall (500, 350):", drying(500, 350))
print("Drying table (600, 680):", drying(600, 680))

--@ chunk 68
-- PALETTE FOR ARCHITECTURAL STAGE
p_wall_dark     = pile{{"raw umber", 6.5}, {"bone black", 3.5}, {"yellow ochre", 1.2}, medium = 0.12}
p_stone_slab    = pile{{"lead white", 7.5}, {"yellow ochre", 2.8}, {"raw umber", 1.0}, {"red earth", 0.3}, medium = 0.08}
p_ledge_face    = pile{{"raw umber", 7.0}, {"bone black", 3.5}, {"red earth", 1.2}, medium = 0.08}
p_ledge_chamfer = pile{{"lead white", 9.2}, {"yellow ochre", 1.5}, {"raw umber", 0.3}, medium = 0.04}

-- 1. Unify the background wall (y: 0 to 520) in deep chiaroscuro
local m_wall = rect(0, 0, 1000, 520)
work(m_wall, {
  hand = "broad",
  pile = p_wall_dark,
  angle = 0.05,
  coverage = 2.4,
  fill = true
})
blend(m_wall, {angle = 0, coverage = 1.0})

-- 2. Unify the limestone table slab (y: 520 to 715) in luminous buff stone
local m_table = rect(0, 520, 1000, 195)
work(m_table, {
  hand = "broad",
  pile = p_stone_slab,
  angle = 0,
  coverage = 2.4,
  fill = true
})
blend(m_table, {angle = 0, coverage = 1.0})

-- 3. Unify the vertical front ledge face (y: 715 to 800) in deep masonry shadow
local m_ledge = rect(0, 715, 1000, 85)
work(m_ledge, {
  hand = "body",
  pile = p_ledge_face,
  angle = 0,
  coverage = 2.4,
  fill = true
})

-- 4. Sharp chiseled chamfer highlight along front edge (y = 715)
local b_r1 = brush{kind = "round", width = 1.6, point = 0.98}
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{0, 715}, {350, 715}, {700, 715}, {1000, 715}}, {pressure = {0.6, 0.9, 0.6}, shake = 0.2})

print("Architectural stage completely unified: wall, table, and ledge.")

--@ chunk 69
local b_f16 = brush("filbert", 16)
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}

-- 1. Clean horizontal back horizon line where table meets wall (y = 518)
b_f16:load(p_wall_dark, 1.0)
b_f16:stroke({{0, 517}, {500, 517}, {1000, 517}}, {pressure = 0.95})

-- 2. Clean straight chamfer highlight along the stone table's front edge (y = 715)
b_r2:load(p_ledge_chamfer, 1.0)
b_r2:stroke({{0, 715}, {350, 715}, {700, 715}, {1000, 715}}, {pressure = {0.6, 0.85, 0.6}, shake = 0.1})

-- 3. Now let the entire stage dry completely into a pristine rock-solid ground
local t = wait(6 * 24 * 60)
print("Time after drying:", t)
print("Drying wall (500, 300):", drying(500, 300))
print("Drying table (500, 600):", drying(500, 600))
print("Drying ledge (500, 750):", drying(500, 750))

--@ chunk 70
-- 1. Cover the white horns above the table back with solid dark wall paint (y: 470 to 522, x: 350 to 700)
local m_wall_low = rect(350, 470, 350, 53)
work(m_wall_low, {
  hand = "body",
  pile = p_wall_dark,
  angle = 0,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- Crisp straight horizon line where dark wall meets stone table (y = 522)
local b_f16 = brush("filbert", 16)
b_f16:load(p_wall_dark, 1.0)
b_f16:stroke({{0, 521}, {500, 521}, {1000, 521}}, {pressure = 0.95})

-- 2. Solid clean front ledge face (y: 718 to 800)
local m_ledge = rect(0, 718, 1000, 82)
work(m_ledge, {
  hand = "body",
  pile = p_ledge_face,
  angle = 0,
  coverage = 2.8,
  fill = true,
  edge = "firm"
})

-- Crisp chamfer highlight along the stone table's front edge (y = 715)
local b_r2 = brush{kind = "round", width = 1.8, point = 0.95}
b_r2:load(p_ledge_chamfer, 1.0)
b_r2:stroke({{0, 715}, {350, 715}, {700, 715}, {1000, 715}}, {pressure = {0.6, 0.9, 0.6}, shake = 0.1})

-- Quick dry so the edge is fixed
local t = wait(4 * 24 * 60)
print("Time after stage cleanup:", t)

--@ chunk 71
-- PALETTE FOR EARTHENWARE PITCHER
p_jug_body      = pile{{"red earth", 5.0}, {"yellow ochre", 4.0}, {"lead white", 2.0}, {"raw umber", 1.0}, medium = 0.05}
p_jug_lit       = pile{{"yellow ochre", 5.0}, {"lead white", 4.5}, {"red earth", 2.0}, medium = 0.04}
p_jug_hi        = pile{{"lead white", 8.0}, {"yellow ochre", 2.0}, {"red earth", 0.5}, medium = 0.03}
p_jug_core      = pile{{"raw umber", 5.5}, {"red earth", 4.5}, {"bone black", 1.5}, medium = 0.06}
p_jug_deep      = pile{{"raw umber", 6.5}, {"bone black", 2.5}, {"red earth", 2.0}, medium = 0.07}
p_jug_reflect   = pile{{"red earth", 4.0}, {"yellow ochre", 4.5}, {"raw umber", 2.0}, medium = 0.06}
p_jug_glaze     = pile{{"lead white", 9.2}, {"yellow ochre", 1.2}, medium = 0.03}
p_interior_dark = pile{{"bone black", 6.0}, {"raw umber", 3.5}, {"red earth", 1.5}, medium = 0.06}

local pts_jug_body = {
  {338, 268}, {340, 310}, {336, 340}, {302, 385}, {268, 440}, {264, 485},
  {278, 540}, {312, 590}, {326, 615}, {420, 615}, {435, 590}, {458, 540},
  {472, 485}, {468, 440}, {438, 385}, {405, 340}, {402, 310}, {404, 268}
}
local m_jug = poly(pts_jug_body, true)

-- 1. OPAQUE TERRACOTTA BODY FOUNDATION
work(m_jug, {
  hand = "body",
  pile = p_jug_body,
  angle = 1.45,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- 2. DIRECTIONAL FILBERT MODELING WET-IN-WET
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Lit flank on left (curving down the belly)
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{348, 274}, {348, 330}, {335, 385}, {305, 445}, {295, 490}, {305, 545}, {335, 595}, {350, 612}}, {
  pressure = {0.8, 0.95, 0.9, 0.9, 0.7}
})
b_f10:load(p_jug_lit, 1.0)
b_f10:stroke({{360, 276}, {360, 335}, {350, 395}, {330, 455}, {325, 505}, {340, 560}, {365, 612}}, {
  pressure = {0.75, 0.95, 0.85, 0.8, 0.6}
})

-- Deep core shadow along right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{394, 274}, {394, 335}, {412, 390}, {442, 450}, {446, 500}, {430, 555}, {405, 605}, {395, 612}}, {
  pressure = {0.85, 0.98, 0.95, 0.95, 0.8}
})
b_f10:load(p_jug_deep, 1.0)
b_f10:stroke({{415, 375}, {440, 425}, {456, 480}, {452, 530}, {435, 580}, {412, 610}}, {
  pressure = {0.7, 0.95, 0.9, 0.7}
})

-- Reflected table bounce along right contour
b_f10:load(p_jug_reflect, 0.9)
b_f10:stroke({{402, 335}, {428, 385}, {460, 440}, {466, 490}, {455, 540}, {435, 585}, {418, 612}}, {
  pressure = {0.4, 0.75, 0.7, 0.4}
})

-- Marry the tones across the turning volume
local b_badger = brush("badger", 16)
b_badger:stroke({{340, 320}, {375, 322}, {402, 320}}, {pressure = 0.25})
b_badger:stroke({{320, 390}, {375, 395}, {435, 390}}, {pressure = 0.25})
b_badger:stroke({{290, 460}, {370, 465}, {450, 460}}, {pressure = 0.25})
b_badger:stroke({{300, 520}, {375, 525}, {450, 520}}, {pressure = 0.25})
b_badger:stroke({{325, 575}, {375, 580}, {425, 575}}, {pressure = 0.25})

-- 3. MOUTH INTERIOR AND LIP
-- Dark interior opening
work(ellipse(370, 269, 30, 6), {hand = "detail", pile = p_interior_dark, coverage = 3.0, fill = true})

-- Turned lip catching light on left rim
b_r2:load(p_jug_hi, 1.0)
b_r2:stroke({{336, 268}, {355, 273}, {372, 274}}, {pressure = 0.85})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(348, 272, {pressure = 0.9})

-- 4. PULLED CLAY LOOP HANDLE
-- Negative space inside loop: solid dark wall
local pts_hole = {
  {330, 328}, {300, 346}, {270, 378}, {258, 415}, {266, 445}, {278, 430}, {295, 395}, {318, 360}, {332, 335}
}
work(poly(pts_hole, true), {hand = "body", pile = p_wall_dark, coverage = 3.2, fill = true, edge = "firm"})

-- Handle body: lit crest and shaded underside
local pts_handle_crest = {
  {338, 314}, {306, 326}, {268, 354}, {246, 388}, {244, 420}, {254, 452}, {272, 480}
}
b_r3:load(p_jug_lit, 1.0)
b_r3:stroke(pts_handle_crest, {pressure = {0.7, 0.95, 0.7}})

local pts_handle_under = {
  {332, 324}, {300, 336}, {262, 364}, {240, 396}, {238, 426}, {248, 458}, {266, 485}
}
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke(pts_handle_under, {pressure = 0.85})

-- Handle specular catchlight
b_r1:load(p_jug_glaze, 1.0)
b_r1:stroke({{246, 395}, {244, 412}, {248, 426}}, {pressure = {0.4, 0.9, 0.4}})

-- 5. LUSCIOUS GLAZE IMPASTO ON SHOULDER
b_r3:load(p_jug_hi, 0.98)
b_r3:stroke({{328, 380}, {324, 412}, {318, 442}}, {pressure = {0.6, 0.95, 0.5}, swell = {0.8, 1.3, 0.8}})
b_r2:load(p_jug_glaze, 1.0)
b_r2:stroke({{325, 396}, {322, 416}, {319, 430}}, {pressure = {0.5, 0.98, 0.4}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(323, 414, {pressure = 0.98})

-- 6. CONTACT SHADOW AT FOOT
b_f10:load(p_jug_deep, 1.0)
b_f10:stroke({{320, 616}, {370, 618}, {425, 616}}, {pressure = {0.7, 0.98, 0.6}})

print("Earthenware pitcher painted with complete sculptural form.")

--@ chunk 72
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r4  = brush{kind = "round", width = 4.5, point = 0.85}
local b_r3  = brush{kind = "round", width = 3.0, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. DEEPEN CORE SHADOW AND SHAPE ON RIGHT FLANK (MASSIVE CYLINDRICAL DEPTH)
b_f14:load(p_jug_deep, 1.0)
b_f14:stroke({{395, 275}, {395, 335}, {415, 390}, {445, 450}, {448, 505}, {430, 560}, {405, 608}}, {
  pressure = {0.85, 0.98, 0.98, 0.95, 0.85}
})

-- Reflected bounce light along the far right rim (x ~ 455 to 470)
b_f10:load(p_jug_reflect, 0.9)
b_f10:stroke({{402, 335}, {430, 385}, {462, 440}, {468, 490}, {456, 545}, {435, 590}, {418, 612}}, {
  pressure = {0.4, 0.75, 0.75, 0.4}
})

-- 2. SOLIDIFY LIT BELLY ON LEFT
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{342, 275}, {342, 330}, {328, 385}, {296, 445}, {288, 490}, {300, 545}, {330, 595}}, {
  pressure = {0.8, 0.98, 0.95, 0.9, 0.7}
})
b_f10:load(p_jug_lit, 1.0)
b_f10:stroke({{358, 276}, {358, 335}, {345, 395}, {320, 455}, {315, 505}, {330, 560}, {355, 610}}, {
  pressure = {0.75, 0.95, 0.85, 0.8, 0.6}
})

-- Soft vertical blend down the form to marry the planes without grid lines
local b_badger = brush("badger", 16)
b_badger:stroke({{345, 300}, {345, 580}}, {pressure = 0.25})
b_badger:stroke({{375, 300}, {375, 580}}, {pressure = 0.25})
b_badger:stroke({{415, 300}, {415, 580}}, {pressure = 0.25})

-- 3. SUBSTANTIAL PULLED CLAY LOOP HANDLE (THICK CLAY STRAP)
-- Negative space inside loop: solid dark wall
local pts_hole = {
  {330, 328}, {300, 346}, {270, 378}, {258, 415}, {266, 445}, {278, 430}, {295, 395}, {318, 360}, {332, 335}
}
work(poly(pts_hole, true), {hand = "body", pile = p_wall_dark, coverage = 3.2, fill = true, edge = "firm"})

-- Handle body: thick clay strap (width ~ 8 units)
local pts_handle_mass = {
  {338, 314}, {304, 328}, {265, 355}, {242, 390}, {240, 422}, {252, 455}, {270, 480}
}
b_r4:load(p_jug_body, 1.0)
b_r4:stroke(pts_handle_mass, {pressure = 0.9, swell = {0.9, 1.2, 0.9}})

-- Handle lit outer crest
b_r2:load(p_jug_lit, 1.0)
b_r2:stroke(pts_handle_mass, {pressure = 0.85})

-- Handle shaded underside
local pts_handle_under = {
  {332, 324}, {298, 338}, {258, 366}, {236, 400}, {234, 430}, {246, 462}, {264, 485}
}
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke(pts_handle_under, {pressure = 0.85})

-- Handle specular gleam
b_r1:load(p_jug_glaze, 1.0)
b_r1:stroke({{242, 398}, {240, 415}, {244, 430}}, {pressure = {0.4, 0.95, 0.4}})

-- 4. DECISIVE BUTTERY GLAZE IMPASTO ON SHOULDER
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{325, 385}, {320, 415}, {314, 440}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r2:load(p_jug_glaze, 1.0)
b_r2:stroke({{323, 398}, {320, 418}, {317, 432}}, {pressure = {0.5, 0.98, 0.4}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(321, 416, {pressure = 0.98})

-- Rim lip highlight on left
b_r2:load(p_jug_hi, 1.0)
b_r2:stroke({{336, 268}, {355, 273}, {372, 274}}, {pressure = 0.85})

-- 5. CONTACT SHADOW UNDER FOOT
b_f10:load(p_jug_deep, 1.0)
b_f10:stroke({{318, 616}, {370, 618}, {425, 616}}, {pressure = {0.7, 0.98, 0.6}})

print("Pitcher modeled with deep core shadow, solid handle, and rich glaze impasto.")

--@ chunk 73
-- 1. TOUCH UP THE PITCHER
-- Clean the tab on the right neck with dark wall paint
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}

b_r3:load(p_wall_dark, 1.0)
b_r3:stroke({{404, 275}, {404, 315}, {406, 350}}, {pressure = 0.98})

-- Soft internal blend to remove ghost lines from pitcher belly
local pts_jug_body = {
  {338, 268}, {340, 310}, {336, 340}, {302, 385}, {268, 440}, {264, 485},
  {278, 540}, {312, 590}, {326, 615}, {420, 615}, {435, 590}, {458, 540},
  {472, 485}, {468, 440}, {438, 385}, {405, 340}, {402, 310}, {404, 268}
}
blend(poly(pts_jug_body, true), {angle = 0, coverage = 1.0})

-- Substantial pulled handle ribbon (width 8-10 units)
local pts_handle_spine = {
  {338, 314}, {304, 328}, {265, 355}, {242, 390}, {240, 422}, {252, 455}, {270, 480}
}
local b_r6 = brush{kind = "round", width = 6.0, point = 0.85}
b_r6:load(p_jug_body, 1.0)
b_r6:stroke(pts_handle_spine, {pressure = 0.95, swell = {0.9, 1.3, 0.9}})

-- Lit top crest of handle
b_r3:load(p_jug_lit, 1.0)
b_r3:stroke(pts_handle_spine, {pressure = 0.9})

-- Under-edge shadow of handle
local pts_handle_under = {
  {332, 324}, {298, 338}, {258, 366}, {236, 400}, {234, 430}, {246, 462}, {264, 485}
}
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke(pts_handle_under, {pressure = 0.9})

-- Handle specular gleam
b_r1:load(p_jug_glaze, 1.0)
b_r1:stroke({{242, 398}, {240, 415}, {244, 430}}, {pressure = {0.4, 0.95, 0.4}})

-- Restate shoulder glaze impasto
b_r3:load(p_jug_glaze, 1.0)
b_r3:stroke({{325, 395}, {321, 416}, {317, 433}}, {pressure = {0.5, 0.98, 0.4}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(322, 414, {pressure = 0.98})

-- Contact shadow under pitcher foot
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{318, 616}, {370, 618}, {425, 616}}, {pressure = {0.7, 0.98, 0.6}})

-- 2. MIX PALETTE PILES FOR THE RIPE GOLDEN QUINCE
p_quince_gold   = pile{{"yellow ochre", 5.5}, {"chrome yellow", 4.0}, {"lead white", 3.0}, {"red earth", 0.5}, medium = 0.05}
p_quince_lit    = pile{{"chrome yellow", 5.0}, {"lead white", 5.5}, {"yellow ochre", 2.0}, medium = 0.04}
p_quince_crest  = pile{{"lead white", 8.5}, {"chrome yellow", 2.5}, {"yellow ochre", 1.0}, medium = 0.03}
p_quince_shade  = pile{{"raw umber", 5.0}, {"yellow ochre", 4.0}, {"red earth", 2.0}, {"bone black", 0.8}, medium = 0.06}
p_quince_deep   = pile{{"raw umber", 6.5}, {"bone black", 2.5}, {"red earth", 2.0}, medium = 0.07}
p_quince_blush  = pile{{"red earth", 4.5}, {"yellow ochre", 4.0}, {"vermilion", 1.2}, {"raw umber", 1.0}, medium = 0.05}
p_quince_stem   = pile{{"raw umber", 6.5}, {"bone black", 3.0}, {"red earth", 1.5}, medium = 0.05}

-- 3. LAY IN THE QUINCE SILHOUETTE
local pts_quince = {
  {615, 475}, {642, 472}, {665, 488}, {685, 520}, {705, 560},
  {715, 605}, {708, 642}, {686, 670}, {655, 678}, {615, 680},
  {575, 674}, {552, 645}, {542, 610}, {546, 572}, {568, 535},
  {592, 505}, {610, 482}
}
local m_quince = poly(pts_quince, true)

-- Opaque golden foundation
work(m_quince, {
  hand = "body",
  pile = p_quince_gold,
  angle = 1.3,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- 4. SCULPT KNOBBY BULGES AND chiaroscuro WET-IN-WET
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)

-- Core shadow down the right flank
b_f14:load(p_quince_shade, 1.0)
b_f14:stroke({{655, 480}, {678, 520}, {698, 570}, {704, 615}, {684, 655}, {655, 675}}, {
  pressure = {0.8, 0.98, 0.95, 0.8}
})
b_f10:load(p_quince_deep, 1.0)
b_f10:stroke({{650, 500}, {668, 545}, {678, 595}, {674, 640}, {648, 672}}, {
  pressure = {0.6, 0.85, 0.6}
})

-- Sunlit lobes on left flank
b_f14:load(p_quince_lit, 1.0)
-- Shoulder lobe
b_f14:stroke({{625, 478}, {605, 508}, {585, 538}, {602, 568}}, {pressure = {0.7, 0.95, 0.7}})
-- Main sunlit belly lobe
b_f14:stroke({{572, 548}, {552, 582}, {554, 622}, {576, 655}, {615, 675}}, {
  pressure = {0.85, 0.98, 0.85}
})
-- Center lobe
b_f14:stroke({{602, 522}, {596, 568}, {610, 622}, {632, 666}}, {pressure = {0.8, 0.95, 0.7}})

-- Warm cider/vermilion blush stippled on the sunlit crest
local b_stip = brush("stippler", 6)
b_stip:load(p_quince_blush, 0.75)
b_stip:stroke({{575, 560}, {588, 595}, {605, 625}}, {pressure = 0.5, shake = 0.5})
b_stip:stroke({{595, 540}, {612, 575}, {622, 610}}, {pressure = 0.45, shake = 0.5})

-- Blend across the quince to marry the tones into a knobby, spherical volume
blend(m_quince, {angle = 0.2, coverage = 1.0})

-- 5. DECISIVE BUTTERY IMPASTO HIGHLIGHT
b_f10:load(p_quince_lit, 1.0)
b_f10:stroke({{575, 562}, {568, 595}, {578, 628}}, {pressure = {0.6, 0.95, 0.5}})

b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{572, 580}, {568, 598}, {574, 615}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(569, 596, {pressure = 0.98})

-- 6. GNARLED WOODY STEM AT APEX
b_r2:load(p_quince_stem, 1.0)
b_r2:stroke({{630, 474}, {633, 460}, {640, 448}, {646, 440}}, {pressure = {0.9, 0.75, 0.4}})
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{632, 472}, {635, 460}, {641, 450}}, {pressure = 0.5})

-- 7. CONTACT SHADOW UNDER QUINCE
b_r3:load(p_quince_deep, 1.0)
b_r3:stroke({{570, 674}, {615, 678}, {665, 676}, {695, 668}}, {pressure = {0.7, 0.98, 0.6}})

print("Pitcher perfected, ripe golden quince modeled.")

--@ chunk 74
-- 1. ERASE THE PITCHER'S RIGHT NECK TAB
local b_r3 = brush{kind = "round", width = 3.5, point = 0.9}
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}

b_r3:load(p_wall_dark, 1.0)
b_r3:stroke({{408, 290}, {412, 320}, {412, 350}}, {pressure = 0.98})

-- Broaden lower clay anchor of handle into belly (x ~ 265, y ~ 460-480)
b_r3:load(p_jug_body, 1.0)
b_r3:stroke({{250, 445}, {260, 465}, {274, 480}}, {pressure = 0.9})
b_r1:load(p_jug_lit, 1.0)
b_r1:stroke({{254, 442}, {264, 462}, {276, 478}}, {pressure = 0.8})

-- 2. MIX PALETTE PILES FOR THE BODEGÓN KNIFE & SHADOWS
p_steel_body  = pile{{"lead white", 6.5}, {"bone black", 2.2}, {"raw umber", 1.0}, {"cobalt blue", 0.5}, medium = 0.06}
p_steel_edge  = pile{{"lead white", 9.5}, {"bone black", 0.2}, medium = 0.03}
p_brass       = pile{{"yellow ochre", 6.0}, {"raw umber", 2.5}, {"lead white", 2.0}, {"red earth", 0.8}, medium = 0.05}
p_horn_wood   = pile{{"bone black", 5.5}, {"raw umber", 3.5}, {"red earth", 1.5}, medium = 0.06}
p_horn_hi     = pile{{"raw umber", 4.5}, {"yellow ochre", 3.0}, {"lead white", 2.5}, medium = 0.05}
p_cast_shadow = pile{{"raw umber", 5.0}, {"yellow ochre", 3.0}, {"bone black", 1.5}, {"lead white", 1.5}, medium = 0.20}

-- 3. PAINT THE BODEGÓN KNIFE (x ~ 710 to 885, y ~ 665 to 755)
-- Steel blade cast shadow onto stone table
b_r2:load(p_cast_shadow, 0.75)
b_r2:stroke({{720, 672}, {755, 688}, {792, 706}}, {pressure = {0.4, 0.7, 0.4}})

-- Steel blade body (triangular forged blade, ~14 units wide at bolster)
local pts_blade = {
  {712, 668}, {794, 704}, {788, 716}, {740, 692}, {716, 673}
}
work(poly(pts_blade, true), {hand = "body", pile = p_steel_body, coverage = 3.0, fill = true, edge = "firm"})

-- Silvery blade face reflection
b_r2:load(p_steel_edge, 0.85)
b_r2:stroke({{720, 671}, {755, 686}, {788, 704}}, {pressure = {0.3, 0.6, 0.3}})

-- Razor-sharp cutting edge: brilliant specular line
b_r1:load(p_steel_edge, 1.0)
b_r1:stroke({{712, 668}, {740, 692}, {788, 716}}, {pressure = {0.6, 1.0, 0.7}, swell = {0.8, 1.3, 0.8}})

-- Faceted brass bolster
local pts_bolster = {
  {794, 703}, {805, 707}, {801, 718}, {788, 715}
}
work(poly(pts_bolster, true), {hand = "body", pile = p_brass, coverage = 3.0, fill = true, edge = "firm"})
b_r1:load(p_steel_edge, 1.0)
b_r1:stroke({{795, 704}, {805, 707}}, {pressure = 0.75})

-- Turned horn/walnut handle extending forward over the front ledge
local pts_handle = {
  {805, 707}, {832, 720}, {860, 734}, {880, 744},
  {876, 754}, {852, 744}, {824, 730}, {801, 718}
}
work(poly(pts_handle, true), {hand = "body", pile = p_horn_wood, coverage = 3.0, fill = true, edge = "firm"})

-- Specular raking light on the turned handle curve
b_r2:load(p_horn_hi, 0.95)
b_r2:stroke({{806, 708}, {832, 721}, {860, 735}, {878, 745}}, {pressure = {0.4, 0.85, 0.3}})

-- Brass rivets on handle
b_r1:load(p_brass, 1.0)
b_r1:touch(825, 722, {pressure = 0.75})
b_r1:touch(855, 736, {pressure = 0.75})

-- Cast shadow of the projecting handle falling down the dark vertical face of the stone ledge
local b_f10 = brush("filbert", 10)
b_f10:load(p_cast_shadow, 0.85)
b_f10:stroke({{812, 720}, {835, 742}, {860, 765}}, {pressure = {0.6, 0.9, 0.4}})

-- 4. SOFT CAST SHADOWS ON THE LIMESTONE TABLE
local b_f14 = brush("filbert", 14)
local b_badger = brush("badger", 16)

-- Pitcher cast shadow trailing across the table to the right
b_f14:load(p_cast_shadow, 0.65)
b_f14:stroke({{415, 616}, {465, 624}, {525, 632}, {570, 638}}, {pressure = {0.6, 0.8, 0.3}, swell = {0.9, 1.3, 0.7}})
b_badger:stroke({{440, 622}, {500, 630}, {560, 636}}, {pressure = 0.25})

-- Quince cast shadow trailing to the right
b_f14:load(p_cast_shadow, 0.65)
b_f14:stroke({{660, 674}, {705, 680}, {750, 686}}, {pressure = {0.55, 0.75, 0.25}, swell = {0.9, 1.2, 0.7}})
b_badger:stroke({{680, 678}, {725, 684}}, {pressure = 0.25})

-- Re-establish the razor chamfer light along front rim of the ledge
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{0, 715}, {350, 715}, {790, 715}}, {pressure = {0.6, 0.9, 0.6}, shake = 0.1})
b_r1:stroke({{885, 715}, {1000, 715}}, {pressure = {0.6, 0.9, 0.6}, shake = 0.1})

print("Pitcher neck tab erased, knife completed, cast shadows laid.")

--@ chunk 75
-- 1. ERASE PITCHER NECK TAB WITH SOLID WALL PAINT
local m_neck_tab = rect(406, 285, 20, 65)
work(m_neck_tab, {
  hand = "body",
  pile = p_wall_dark,
  angle = 0,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- 2. ERASE AWKWARD KNIFE: RE-ESTABLISH PRISTINE LIMESTONE TABLE AND SOLID LEDGE
-- Limestone table surface (y: 660 to 715, x: 700 to 920)
local m_clean_table = rect(700, 660, 220, 55)
work(m_clean_table, {
  hand = "body",
  pile = p_stone_slab,
  angle = 0,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- Front ledge vertical face (y: 715 to 790, x: 780 to 920)
local m_clean_ledge = rect(780, 715, 140, 75)
work(m_clean_ledge, {
  hand = "body",
  pile = p_ledge_face,
  angle = 0,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- Sharp chiseled chamfer highlight along y = 715
local b_r1 = brush{kind = "round", width = 1.6, point = 0.98}
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{680, 715}, {1000, 715}}, {pressure = {0.6, 0.9, 0.6}, shake = 0.1})

-- 3. RE-SCULPT THE QUINCE INTO A VOLUPTUOUS, KNOBBY ORGANIC MASS
-- Broader shoulders and waist: knobby organic silhouette
local pts_quince_knobby = {
  {610, 480}, {632, 478}, {655, 486}, {678, 508}, {696, 540},
  {712, 580}, {716, 620}, {702, 655}, {675, 676}, {630, 680},
  {585, 676}, {556, 650}, {544, 615}, {546, 575}, {562, 532},
  {582, 502}, {602, 485}
}
local m_quince_k = poly(pts_quince_knobby, true)

-- Opaque golden foundation
work(m_quince_k, {
  hand = "body",
  pile = p_quince_gold,
  angle = 1.3,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.0, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}

-- Core shadow down the right flank
b_f14:load(p_quince_shade, 1.0)
b_f14:stroke({{650, 485}, {675, 518}, {698, 565}, {705, 615}, {686, 655}, {655, 676}}, {
  pressure = {0.8, 0.98, 0.95, 0.8}
})
b_f10:load(p_quince_deep, 1.0)
b_f10:stroke({{645, 505}, {666, 545}, {680, 595}, {675, 640}, {648, 672}}, {
  pressure = {0.6, 0.85, 0.6}
})

-- Table bounce along bottom-right edge
b_f10:load(p_quince_gold, 0.9)
b_f10:stroke({{692, 625}, {710, 645}, {695, 668}, {668, 676}}, {pressure = 0.6})

-- Sunlit swells on left flank
b_f14:load(p_quince_lit, 1.0)
-- Broad sunlit shoulder
b_f14:stroke({{618, 485}, {595, 512}, {575, 545}, {585, 575}}, {pressure = {0.75, 0.95, 0.75}})
-- Main sunlit belly lobe
b_f14:stroke({{570, 545}, {550, 580}, {552, 622}, {576, 658}, {620, 676}}, {
  pressure = {0.85, 0.98, 0.85}
})
-- Center lobe
b_f14:stroke({{605, 520}, {598, 565}, {612, 620}, {635, 668}}, {pressure = {0.8, 0.95, 0.7}})

-- Warm cider/vermilion blush stippled across the sunlit flank
local b_stip = brush("stippler", 6)
b_stip:load(p_quince_blush, 0.7)
b_stip:stroke({{570, 560}, {585, 595}, {602, 625}}, {pressure = 0.5, shake = 0.5})
b_stip:stroke({{590, 540}, {608, 575}, {618, 610}}, {pressure = 0.45, shake = 0.5})

-- Internal blend to turn the volume seamlessly
blend(m_quince_k, {angle = 0.2, coverage = 1.0})

-- Buttery impasto highlight on the sunlit knobby crest
b_f10:load(p_quince_lit, 1.0)
b_f10:stroke({{572, 560}, {565, 592}, {575, 625}}, {pressure = {0.6, 0.95, 0.5}})

b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{568, 578}, {564, 598}, {570, 615}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(565, 595, {pressure = 0.98})

-- Gnarled woody stem at crown
b_r2:load(p_quince_stem, 1.0)
b_r2:stroke({{625, 480}, {628, 465}, {635, 452}, {642, 444}}, {pressure = {0.9, 0.75, 0.4}})
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{627, 478}, {630, 465}, {636, 454}}, {pressure = 0.5})

-- Contact shadow under quince
b_r3:load(p_quince_deep, 1.0)
b_r3:stroke({{570, 674}, {615, 678}, {665, 676}, {695, 668}}, {pressure = {0.7, 0.98, 0.6}})

-- Quince soft cast shadow trailing to the right
local b_badger = brush("badger", 14)
b_f14:load(p_cast_shadow, 0.6)
b_f14:stroke({{660, 674}, {700, 678}, {740, 682}}, {pressure = {0.5, 0.7, 0.25}, swell = {0.9, 1.2, 0.7}})
b_badger:stroke({{680, 676}, {720, 680}}, {pressure = 0.25})

print("Pitcher tab removed, knife erased, table restored, quince remodeled.")

--@ chunk 76
-- 1. SOFTEN AND MELT THE STAMP BESIDE THE PITCHER'S NECK
local b_badger = brush("badger", 16)
b_badger:stroke({{395, 290}, {440, 290}}, {pressure = 0.35})
b_badger:stroke({{395, 315}, {445, 315}}, {pressure = 0.4})
b_badger:stroke({{395, 340}, {445, 340}}, {pressure = 0.35})
b_badger:stroke({{400, 365}, {450, 365}}, {pressure = 0.3})

-- Clean right neck contour
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_jug_deep, 0.95)
b_r2:stroke({{404, 270}, {403, 310}, {404, 345}, {418, 375}, {442, 410}}, {pressure = 0.85})

-- 2. MIX PALETTE PILES FOR THE WALNUTS
p_shell_wood  = pile{{"red earth", 4.5}, {"yellow ochre", 4.0}, {"raw umber", 3.0}, {"lead white", 1.2}, medium = 0.05}
p_shell_shade = pile{{"raw umber", 6.5}, {"bone black", 2.5}, {"red earth", 2.0}, medium = 0.07}
p_shell_rim   = pile{{"lead white", 7.5}, {"yellow ochre", 2.5}, {"raw umber", 0.8}, medium = 0.04}
p_kernel_pure = pile{{"lead white", 8.5}, {"yellow ochre", 2.5}, {"raw umber", 0.6}, medium = 0.04}
p_kernel_hi   = pile{{"lead white", 9.6}, {"yellow ochre", 0.8}, medium = 0.03}
p_nut_shadow  = pile{{"raw umber", 6.0}, {"yellow ochre", 2.5}, {"bone black", 2.0}, {"lead white", 1.0}, medium = 0.12}

local b_r3 = brush{kind = "round", width = 3.0, point = 0.9}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}

-- 3. CRACKED WALNUT HALF (x ~ 768, y ~ 688)
-- Cast shadow under shell
b_r3:load(p_nut_shadow, 0.8)
b_r3:stroke({{755, 694}, {772, 698}, {790, 695}}, {pressure = {0.5, 0.8, 0.4}})

-- Outer woody shell cup
local pts_wn_cup = {
  {758, 680}, {754, 688}, {760, 696}, {775, 699}, {790, 696}, {792, 684}
}
b_r3:load(p_shell_wood, 1.0)
b_r3:stroke(pts_wn_cup, {pressure = 0.9})

-- Dark interior hollow behind the kernel meat
b_r2:load(p_shell_shade, 0.95)
b_r2:stroke({{760, 683}, {772, 689}, {786, 685}}, {pressure = 0.85})

-- Convoluted ivory walnut kernel lobes (chunky impasto)
b_r2:load(p_kernel_pure, 1.0)
-- Left lobe
b_r2:stroke({{762, 682}, {765, 687}, {763, 693}}, {pressure = {0.7, 0.95, 0.6}})
b_r2:stroke({{767, 681}, {769, 686}, {768, 692}}, {pressure = {0.6, 0.9, 0.5}})
-- Right lobe
b_r2:stroke({{773, 681}, {775, 686}, {774, 692}}, {pressure = {0.7, 0.95, 0.6}})
b_r2:stroke({{779, 682}, {782, 687}, {780, 693}}, {pressure = {0.6, 0.9, 0.5}})

-- Central dividing septum and crevices
b_r1:load(p_shell_shade, 0.95)
b_r1:stroke({{771, 680}, {771, 688}, {770, 694}}, {pressure = 0.6})
b_r1:stroke({{760, 690}, {763, 694}}, {pressure = 0.45})

-- Buttery impasto touches on kernel convolutions
b_r1:load(p_kernel_hi, 1.0)
b_r1:touch(764, 683, {pressure = 0.85})
b_r1:touch(768, 688, {pressure = 0.75})
b_r1:touch(775, 682, {pressure = 0.85})
b_r1:touch(780, 688, {pressure = 0.75})

-- Bright shell rim catching light on top-left
b_r1:load(p_shell_rim, 1.0)
b_r1:stroke({{757, 680}, {768, 676}, {782, 676}}, {pressure = {0.5, 0.9, 0.4}})

-- 4. WHOLE CORRUGATED WALNUT (x ~ 815, y ~ 692)
-- Cast shadow
b_r3:load(p_nut_shadow, 0.8)
b_r3:stroke({{802, 698}, {822, 702}, {842, 698}}, {pressure = {0.5, 0.8, 0.4}})

-- Solid nut body
local pts_wn_w = {
  {804, 686}, {816, 678}, {830, 680}, {838, 690}, {832, 699}, {814, 699}, {802, 691}
}
work(poly(pts_wn_w, true), {hand = "body", pile = p_shell_wood, coverage = 3.0, fill = true, edge = "firm"})

-- Shadow flank on right
b_r2:load(p_shell_shade, 0.9)
b_r2:stroke({{824, 682}, {834, 690}, {830, 698}}, {pressure = {0.5, 0.85, 0.4}})

-- Central suture ridge
b_r1:load(p_shell_shade, 0.95)
b_r1:stroke({{816, 678}, {818, 689}, {819, 699}}, {pressure = {0.4, 0.7, 0.4}})

-- Corrugated shell ridges catching raking light
b_r1:load(p_shell_rim, 0.95)
b_r1:stroke({{808, 683}, {811, 689}, {813, 697}}, {pressure = {0.4, 0.8, 0.3}})
b_r1:stroke({{825, 683}, {827, 691}, {826, 697}}, {pressure = {0.3, 0.65, 0.25}})

-- 5. CRISP CHAMFER LIGHT ALONG THE FRONT STONE RIM
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{0, 715}, {350, 715}, {700, 715}, {1000, 715}}, {pressure = {0.6, 0.9, 0.6}, shake = 0.1})

print("Neck patch softened, walnuts painted with rich impasto.")

--@ chunk 77
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.0, point = 0.9}

-- 1. COVER THE FOUR HORIZONTAL CLAW MARKS TO THE RIGHT OF THE PITCHER'S NECK
-- Use vertical strokes parallel to the neck (y: 275 to 400, x: 405 to 455)
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{415, 275}, {415, 340}, {420, 395}}, {pressure = {0.85, 0.98, 0.85}})
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{430, 275}, {430, 340}, {435, 395}}, {pressure = {0.85, 0.98, 0.85}})
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{445, 275}, {445, 340}, {450, 395}}, {pressure = {0.85, 0.98, 0.85}})

-- Clean right silhouette of neck and shoulder with a continuous razor stroke
local pts_neck_contour = {
  {404, 268}, {403, 310}, {404, 345}, {418, 380}, {442, 415}, {468, 460}
}
b_r3:load(p_wall_dark, 1.0)
b_r3:stroke(pts_neck_contour, {pressure = 0.95})

-- 2. COVER THE TINY WALNUTS ON THE FAR RIGHT TABLE
-- Re-establish the clean limestone table plane at x ~ 750 to 860, y ~ 670 to 715
for y = 675, 712, 14 do
  b_f14:load(p_stone_slab, 0.98)
  b_f14:stroke({{740, y}, {810, y}, {880, y}}, {pressure = {0.85, 0.98, 0.85}})
end

-- Sharp chiseled chamfer highlight along front edge (y = 715)
local b_r1 = brush{kind = "round", width = 1.6, point = 0.98}
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{720, 715}, {880, 715}}, {pressure = {0.6, 0.9, 0.6}, shake = 0.1})

print("Neck claw marks covered with vertical strokes; tiny walnuts painted out.")

--@ chunk 78
-- 1. PITCHER REFINEMENTS
-- Soften the dark patch to the right of the pitcher's neck into the surrounding wall
local b_badger = brush("badger", 20)
b_badger:stroke({{410, 290}, {460, 290}}, {pressure = 0.3})
b_badger:stroke({{410, 330}, {460, 330}}, {pressure = 0.35})
b_badger:stroke({{415, 370}, {465, 370}}, {pressure = 0.3})

-- Sharp sculptural silhouette along the right neck and shoulder
local b_r3 = brush{kind = "round", width = 3.0, point = 0.9}
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}

b_r2:load(p_jug_deep, 0.95)
b_r2:stroke({{404, 270}, {403, 310}, {404, 345}, {418, 380}, {442, 415}, {468, 460}}, {pressure = 0.85})

-- Buttery glaze impasto on the pitcher's lit shoulder
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{326, 385}, {321, 412}, {316, 438}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r2:load(p_jug_glaze, 1.0)
b_r2:stroke({{324, 398}, {321, 416}, {318, 430}}, {pressure = {0.5, 0.98, 0.4}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(322, 414, {pressure = 0.98})

-- Specular glint on flared lip
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(348, 272, {pressure = 0.95})

-- Specular glint on handle crest
b_r1:load(p_jug_glaze, 1.0)
b_r1:stroke({{242, 398}, {240, 415}, {244, 430}}, {pressure = {0.4, 0.95, 0.4}})

-- Deep contact shadow under foot
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{318, 616}, {370, 618}, {425, 616}}, {pressure = {0.7, 0.98, 0.6}})

-- 2. SCULPTURAL MASTER MODELING OF THE QUINCE
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)

-- Core shadow down the right flank: deep, warm, turning volume
b_f14:load(p_quince_shade, 1.0)
b_f14:stroke({{648, 485}, {672, 520}, {696, 568}, {704, 615}, {685, 655}, {652, 676}}, {
  pressure = {0.8, 0.98, 0.95, 0.8}
})
b_f10:load(p_quince_deep, 1.0)
b_f10:stroke({{642, 505}, {664, 548}, {678, 595}, {674, 640}, {645, 672}}, {
  pressure = {0.6, 0.85, 0.6}
})

-- Warm table bounce along the bottom-right rim
b_f10:load(p_quince_gold, 0.9)
b_f10:stroke({{688, 625}, {708, 645}, {692, 668}, {665, 676}}, {pressure = 0.6})

-- Sunlit knobby lobes on left flank
b_f14:load(p_quince_lit, 1.0)
-- Shoulder swell
b_f14:stroke({{618, 485}, {595, 512}, {575, 545}, {585, 575}}, {pressure = {0.75, 0.95, 0.75}})
-- Main sunlit belly bulge
b_f14:stroke({{570, 545}, {550, 580}, {552, 622}, {576, 658}, {620, 676}}, {
  pressure = {0.85, 0.98, 0.85}
})
-- Center lobe
b_f14:stroke({{605, 520}, {598, 565}, {612, 620}, {635, 668}}, {pressure = {0.8, 0.95, 0.7}})

-- Warm cider blush stippled across the sunlit flank
local b_stip = brush("stippler", 6)
b_stip:load(p_quince_blush, 0.75)
b_stip:stroke({{570, 560}, {585, 595}, {602, 625}}, {pressure = 0.5, shake = 0.5})
b_stip:stroke({{590, 540}, {608, 575}, {618, 610}}, {pressure = 0.45, shake = 0.5})

-- Soft badger blend across the quince to marry the lobes
b_badger:stroke({{560, 575}, {615, 585}, {680, 585}}, {pressure = 0.3})
b_badger:stroke({{570, 625}, {625, 635}, {685, 630}}, {pressure = 0.3})

-- Buttery impasto highlight on the knobby crest
b_f10:load(p_quince_lit, 1.0)
b_f10:stroke({{572, 560}, {565, 592}, {575, 625}}, {pressure = {0.6, 0.95, 0.5}})

b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{568, 578}, {564, 598}, {570, 615}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(565, 595, {pressure = 0.98})

-- Crown depression and gnarled woody stem
-- Crown socket shadow
b_r2:load(p_quince_stem, 1.0)
b_r2:stroke({{618, 482}, {628, 478}, {638, 482}}, {pressure = 0.8})

-- Gnarled woody stem curving up
b_r2:load(p_quince_stem, 1.0)
b_r2:stroke({{625, 480}, {628, 465}, {635, 452}, {642, 444}}, {pressure = {0.9, 0.75, 0.4}})
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{627, 478}, {630, 465}, {636, 454}}, {pressure = 0.5})

-- Calyx eye at base
b_r1:load(p_quince_deep, 1.0)
b_r1:touch(655, 672, {pressure = 0.85})
b_r1:stroke({{653, 670}, {657, 674}}, {pressure = 0.7})
b_r1:stroke({{656, 670}, {654, 674}}, {pressure = 0.7})

-- Deep grounding contact shadow under quince
b_r3:load(p_quince_deep, 1.0)
b_r3:stroke({{570, 674}, {615, 678}, {665, 676}, {695, 668}}, {pressure = {0.7, 0.98, 0.6}})

-- Quince soft cast shadow trailing across the stone table
b_f14:load(p_cast_shadow, 0.6)
b_f14:stroke({{660, 674}, {705, 678}, {750, 682}}, {pressure = {0.5, 0.7, 0.25}, swell = {0.9, 1.2, 0.7}})
b_badger:stroke({{680, 676}, {725, 680}}, {pressure = 0.25})

-- 3. UNIFY THE RIGHT LIMESTONE TABLE SURFACE (SEAMLESS STONE)
-- Feather the horizontal stone strokes into a continuous limestone plane
b_badger:stroke({{700, 685}, {850, 685}, {1000, 685}}, {pressure = 0.3})
b_badger:stroke({{700, 705}, {850, 705}, {1000, 705}}, {pressure = 0.3})

-- Razor chamfer highlight along the stone table's front edge (y = 715)
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{0, 715}, {350, 715}, {700, 715}, {1000, 715}}, {pressure = {0.6, 0.9, 0.6}, shake = 0.1})

print("Pitcher and quince brought to masterly completion.")

--@ chunk 79
local t = wait(4 * 24 * 60)
print("Time after drying:", t)
print("Drying quince (600, 600):", drying(600, 600))
print("Drying neck (420, 320):", drying(420, 320))
print("Drying table (800, 680):", drying(800, 680))

--@ chunk 80
local t = wait(2 * 24 * 60)
print("Time after 2 more days:", t)
print("Drying table (800, 680):", drying(800, 680))

--@ chunk 81
-- 1. CLEAN THE PITCHER'S RIGHT NECK WALL
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.0, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Vertical strokes of dark wall paint over the patch (y: 275 to 390, x: 405 to 455)
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{415, 275}, {415, 340}, {420, 395}}, {pressure = {0.85, 0.98, 0.85}})
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{432, 275}, {432, 340}, {435, 395}}, {pressure = {0.85, 0.98, 0.85}})
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{448, 275}, {448, 340}, {450, 395}}, {pressure = {0.85, 0.98, 0.85}})

-- Sharp sculptural silhouette along the right neck
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke({{404, 268}, {403, 310}, {404, 345}, {418, 380}, {442, 415}}, {pressure = 0.95})

-- 2. REPAINT THE QUINCE IN SOLID, SCULPTURAL BODY PAINT
-- Reset the entire quince face with rich, opaque golden paint
local pts_q_body = {
  {610, 480}, {632, 478}, {655, 486}, {678, 508}, {696, 540},
  {712, 580}, {716, 620}, {702, 655}, {675, 676}, {630, 680},
  {585, 676}, {556, 650}, {544, 615}, {546, 575}, {562, 532},
  {582, 502}, {602, 485}
}
work(poly(pts_q_body, true), {
  hand = "body",
  pile = p_quince_gold,
  angle = 1.4,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- Shaded right flank: deep, warm, turning core shadow (vertical strokes following the contour!)
b_f14:load(p_quince_shade, 1.0)
b_f14:stroke({{650, 488}, {674, 525}, {696, 570}, {704, 615}, {686, 655}, {655, 676}}, {
  pressure = {0.85, 0.98, 0.95, 0.85}
})

-- Table bounce on right edge
b_f10:load(p_quince_gold, 0.95)
b_f10:stroke({{690, 625}, {710, 645}, {694, 668}, {668, 676}}, {pressure = 0.65})

-- Lit flank on left: rich warm yellow-chrome strokes
b_f14:load(p_quince_lit, 1.0)
b_f14:stroke({{615, 488}, {592, 515}, {572, 550}, {580, 580}}, {pressure = {0.8, 0.98, 0.8}})
b_f14:stroke({{568, 550}, {548, 585}, {552, 625}, {575, 658}, {618, 676}}, {
  pressure = {0.85, 0.98, 0.85}
})

-- Center midtone marrying lit and shaded flanks
b_f14:load(p_quince_gold, 1.0)
b_f14:stroke({{605, 515}, {600, 565}, {615, 620}, {638, 668}}, {pressure = {0.8, 0.95, 0.75}})

-- Soft vertical blend down the form (VERTICAL, never horizontal across shadow!)
local b_badger = brush("badger", 16)
b_badger:stroke({{595, 510}, {595, 650}}, {pressure = 0.25})
b_badger:stroke({{635, 505}, {635, 655}}, {pressure = 0.25})
b_badger:stroke({{665, 515}, {665, 655}}, {pressure = 0.25})

-- Buttery impasto highlight on the knobby crest
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{568, 575}, {564, 598}, {570, 618}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(565, 595, {pressure = 0.98})

-- Gnarled woody stem at crown
b_r2:load(p_quince_stem, 1.0)
b_r2:stroke({{625, 480}, {628, 465}, {635, 452}, {642, 444}}, {pressure = {0.95, 0.8, 0.4}})
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{627, 478}, {630, 465}, {636, 454}}, {pressure = 0.5})

-- Contact shadow under quince
b_r3:load(p_quince_deep, 1.0)
b_r3:stroke({{570, 674}, {615, 678}, {665, 676}, {695, 668}}, {pressure = {0.7, 0.98, 0.6}})

-- Quince soft cast shadow trailing to right
b_f14:load(p_cast_shadow, 0.6)
b_f14:stroke({{660, 674}, {705, 678}, {750, 682}}, {pressure = {0.5, 0.7, 0.25}, swell = {0.9, 1.2, 0.7}})

-- 3. UNIFY RIGHT LIMESTONE TABLE SURFACE
for y = 680, 712, 16 do
  b_f14:load(p_stone_slab, 0.98)
  b_f14:stroke({{680, y}, {820, y}, {980, y}}, {pressure = {0.85, 0.98, 0.85}})
end

-- Sharp chiseled chamfer highlight along y = 715
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{0, 715}, {350, 715}, {700, 715}, {1000, 715}}, {pressure = {0.6, 0.9, 0.6}, shake = 0.1})

print("Pitcher neck wall cleaned; quince repainted with solid volume; table unified.")

--@ chunk 82
-- RESTORE THE PITCHER'S SWELLING RIGHT SHOULDER
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.5, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.2, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Fill the notched shoulder with solid, opaque terracotta core shadow
local pts_shoulder_fill = {
  {404, 340}, {438, 385}, {468, 440}, {445, 440}, {415, 385}, {400, 340}
}
work(poly(pts_shoulder_fill, true), {
  hand = "body",
  pile = p_jug_core,
  angle = 1.4,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- Shaded right contour curve of the swelling shoulder
local pts_shoulder_curve = {
  {404, 340}, {420, 365}, {440, 395}, {458, 430}, {470, 470}
}
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke(pts_shoulder_curve, {pressure = 0.95})

-- Soft internal blend down the shoulder into the belly
b_f10:load(p_jug_core, 0.9)
b_f10:stroke({{410, 350}, {430, 390}, {448, 440}}, {pressure = {0.6, 0.9, 0.6}})

-- Glaze catchlight on the pitcher's lit left shoulder
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{326, 385}, {321, 412}, {316, 438}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r2:load(p_jug_glaze, 1.0)
b_r2:stroke({{324, 398}, {321, 416}, {318, 430}}, {pressure = {0.5, 0.98, 0.4}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(322, 414, {pressure = 0.98})

-- Specular glint on left flared lip
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(348, 272, {pressure = 0.95})

-- Specular glint on handle crest
b_r1:load(p_jug_glaze, 1.0)
b_r1:stroke({{242, 398}, {240, 415}, {244, 430}}, {pressure = {0.4, 0.95, 0.4}})

-- Firm contact shadow under foot
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{318, 616}, {370, 618}, {425, 616}}, {pressure = {0.7, 0.98, 0.6}})

print("Pitcher shoulder fully restored with swelling volume and glaze impasto.")

--@ chunk 83
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}

-- Cover the dark shoulder patch with solid warm terracotta body paint
local pts_shoulder_fill = {
  {404, 340}, {438, 385}, {468, 440}, {445, 440}, {415, 385}, {400, 340}
}
work(poly(pts_shoulder_fill, true), {
  hand = "body",
  pile = p_jug_body,
  angle = 1.45,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- Model the shoulder with vertical filbert strokes following the form
b_f10:load(p_jug_body, 1.0)
b_f10:stroke({{408, 340}, {425, 385}, {445, 435}}, {pressure = {0.8, 0.95, 0.8}})
b_f10:stroke({{418, 345}, {435, 390}, {455, 440}}, {pressure = {0.8, 0.95, 0.8}})

-- Shaded right edge of the shoulder (raw umber core shadow)
b_r3:load(p_jug_core, 0.95)
b_r3:stroke({{404, 340}, {422, 370}, {445, 405}, {466, 445}}, {pressure = 0.85})

-- Clean right silhouette curve against the wall
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke({{404, 270}, {403, 310}, {404, 345}, {422, 372}, {446, 408}, {468, 448}}, {pressure = 0.95})

print("Pitcher shoulder tone harmonized with body.")

--@ chunk 84
-- 1. FUSE THE SHOULDER INTO THE BELLY WITH A SOFT REGIONAL BLEND
local m_shoulder_zone = rect(350, 320, 125, 170)
blend(m_shoulder_zone, {angle = 1.4, coverage = 1.2})

-- 2. CRISP, CONTINUOUS RIGHT SILHOUETTE FROM MOUTH DOWN TO BELLY EQUATOR
local pts_right_profile = {
  {404, 268}, {403, 310}, {404, 340}, {420, 380}, {448, 425}, {470, 475}, {468, 510}
}
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke(pts_right_profile, {pressure = 0.95})

-- 3. SOFTEN THE RIGHT FLANK'S CORE SHADOW INTO AMBIENT TABLE BOUNCE
local b_f10 = brush("filbert", 10)
b_f10:load(p_jug_reflect, 0.85)
b_f10:stroke({{425, 385}, {452, 435}, {468, 480}, {458, 530}}, {pressure = {0.3, 0.6, 0.3}})

print("Shoulder blended and right silhouette cleanly redrawn.")

--@ chunk 85
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. UNITE PITCHER'S RIGHT FLANK OUT TO TRUE SILHOUETTE
-- Paint over the dark inner line with solid terracotta
b_f10:load(p_jug_body, 1.0)
b_f10:stroke({{415, 350}, {438, 395}, {458, 445}, {465, 490}}, {pressure = 0.95})
b_f10:stroke({{425, 360}, {448, 405}, {465, 455}}, {pressure = 0.95})

-- Shaded core along the turning flank
b_f10:load(p_jug_core, 0.95)
b_f10:stroke({{430, 390}, {452, 435}, {466, 480}, {455, 530}}, {pressure = {0.6, 0.9, 0.6}})

-- Soft vertical blend down the right flank to marry the shoulder into the belly
local b_badger = brush("badger", 16)
b_badger:stroke({{430, 350}, {430, 510}}, {pressure = 0.25})
b_badger:stroke({{450, 380}, {450, 510}}, {pressure = 0.25})

-- Clean the background wall along the TRUE outer silhouette of the pitcher
local pts_true_silhouette = {
  {405, 268}, {404, 310}, {405, 340}, {435, 385}, {468, 440}, {472, 485}, {460, 535}, {430, 585}, {420, 615}
}
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke(pts_true_silhouette, {pressure = 0.95})

-- 2. PITCHER FINISHING TOUCHES
-- Glaze impasto catchlight on lit shoulder
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{326, 385}, {321, 412}, {316, 438}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r2:load(p_jug_glaze, 1.0)
b_r2:stroke({{324, 398}, {321, 416}, {318, 430}}, {pressure = {0.5, 0.98, 0.4}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(322, 414, {pressure = 0.98})

-- Specular glint on left flared lip
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(348, 272, {pressure = 0.95})

-- Specular glint on handle crest
b_r1:load(p_jug_glaze, 1.0)
b_r1:stroke({{242, 398}, {240, 415}, {244, 430}}, {pressure = {0.4, 0.95, 0.4}})

-- Firm contact shadow under foot
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{318, 616}, {370, 618}, {425, 616}}, {pressure = {0.7, 0.98, 0.6}})

-- 3. QUINCE FINISHING TOUCHES
-- Rich, buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{568, 575}, {564, 598}, {570, 618}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(565, 595, {pressure = 0.98})

-- Warm cider blush scumble on the sunlit curve
local b_stip = brush("stippler", 6)
b_stip:load(p_quince_blush, 0.6)
b_stip:stroke({{575, 565}, {588, 595}, {605, 625}}, {pressure = 0.35, shake = 0.4})

-- Gnarled woody stem arching from crown socket
b_r2:load(p_quince_stem, 1.0)
b_r2:stroke({{625, 480}, {628, 465}, {635, 452}, {642, 444}}, {pressure = {0.95, 0.8, 0.4}})
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{627, 478}, {630, 465}, {636, 454}}, {pressure = 0.5})

-- Calyx eye at base
b_r1:load(p_quince_deep, 1.0)
b_r1:touch(655, 672, {pressure = 0.85})
b_r1:stroke({{653, 670}, {657, 674}}, {pressure = 0.7})

-- Firm contact shadow under quince
b_r3:load(p_quince_deep, 1.0)
b_r3:stroke({{570, 674}, {615, 678}, {665, 676}, {695, 668}}, {pressure = {0.7, 0.98, 0.6}})

print("Pitcher flank unified out to true silhouette; quince and pitcher finished.")

--@ chunk 86
-- Soften the atmospheric light bloom behind the pitcher's shadow flank into the wall
local b_f14 = brush("filbert", 14)
local b_badger = brush("badger", 18)

b_f14:load(p_wall_mid, 0.4)
b_f14:stroke({{455, 340}, {510, 340}}, {pressure = {0.15, 0.35, 0.15}})
b_f14:stroke({{455, 380}, {515, 380}}, {pressure = {0.15, 0.35, 0.15}})
b_f14:stroke({{455, 420}, {510, 420}}, {pressure = {0.15, 0.35, 0.15}})

b_badger:stroke({{445, 380}, {525, 380}}, {pressure = 0.2})

print("Atmospheric background transition softened.")

--@ chunk 87
local b_f16 = brush("filbert", 16)
b_f16:load(p_wall_dark, 1.0)
b_f16:stroke({{475, 320}, {475, 380}, {475, 450}}, {pressure = 0.95})
b_f16:load(p_wall_dark, 1.0)
b_f16:stroke({{495, 320}, {495, 380}, {495, 450}}, {pressure = 0.95})
b_f16:load(p_wall_dark, 1.0)
b_f16:stroke({{515, 320}, {515, 380}, {515, 450}}, {pressure = 0.95})

print("Horizontal marks covered with vertical wall strokes.")

--@ chunk 88
local b_f24 = brush("filbert", 24)
local b_f16 = brush("filbert", 16)

-- Continuous horizontal sweeps of dark wall paint above the quince, covering all dashes completely
b_f24:load(p_wall_dark, 1.0)
b_f24:stroke({{406, 325}, {600, 325}, {850, 325}}, {pressure = {0.9, 0.95, 0.7}})
b_f24:load(p_wall_dark, 1.0)
b_f24:stroke({{406, 360}, {600, 360}, {850, 360}}, {pressure = {0.9, 0.95, 0.7}})
b_f24:load(p_wall_dark, 1.0)
b_f24:stroke({{415, 395}, {600, 395}, {850, 395}}, {pressure = {0.9, 0.95, 0.7}})
b_f16:load(p_wall_dark, 1.0)
b_f16:stroke({{435, 430}, {600, 430}, {850, 430}}, {pressure = {0.9, 0.95, 0.7}})

-- Crisp silhouette along the pitcher's right neck and shoulder
local pts_neck_shoulder = {
  {404, 268}, {403, 310}, {404, 340}, {435, 385}, {468, 440}, {472, 485}
}
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke(pts_neck_shoulder, {pressure = 0.95})

print("Background wall sweeps completed across upper field; neck silhouette cleaned.")

--@ chunk 89
local t = wait(5 * 24 * 60)
print("Time after drying:", t)
print("Drying pitcher shoulder (430, 400):", drying(430, 400))

--@ chunk 90
local pts_jug_body = {
  {338, 268}, {340, 310}, {336, 340}, {302, 385}, {268, 440}, {264, 485},
  {278, 540}, {312, 590}, {326, 615}, {420, 615}, {435, 590}, {458, 540},
  {472, 485}, {468, 440}, {438, 385}, {405, 340}, {402, 310}, {404, 268}
}
local m_jug = poly(pts_jug_body, true)

-- 1. ERASE ALL FINGERS OUTSIDE THE PITCHER WITH SOLID WALL PAINT
local m_wall_side = rect(400, 270, 160, 210) - m_jug
work(m_wall_side, {
  hand = "body",
  pile = p_wall_dark,
  angle = 0,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- 2. ERASE ALL FINGERS INSIDE THE PITCHER WITH SOLID OPAQUE TERRACOTTA
work(m_jug, {
  hand = "body",
  pile = p_jug_body,
  angle = 1.45,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- 3. MODEL PITCHER VOLUME WET-IN-WET
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Lit left flank
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{345, 274}, {345, 330}, {330, 385}, {296, 445}, {288, 490}, {300, 545}, {330, 595}, {348, 612}}, {
  pressure = {0.8, 0.98, 0.95, 0.9, 0.7}
})
b_f10:load(p_jug_lit, 1.0)
b_f10:stroke({{358, 276}, {358, 335}, {345, 395}, {320, 455}, {315, 505}, {330, 560}, {355, 610}}, {
  pressure = {0.75, 0.95, 0.85, 0.8, 0.6}
})

-- Core shadow down the right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{394, 274}, {394, 335}, {415, 390}, {445, 450}, {448, 500}, {430, 555}, {405, 605}}, {
  pressure = {0.85, 0.98, 0.95, 0.95, 0.8}
})
b_f10:load(p_jug_deep, 1.0)
b_f10:stroke({{415, 375}, {442, 425}, {458, 480}, {452, 530}, {435, 580}, {412, 610}}, {
  pressure = {0.7, 0.95, 0.9, 0.7}
})

-- Table bounce on right edge
b_f10:load(p_jug_reflect, 0.9)
b_f10:stroke({{402, 335}, {430, 385}, {462, 440}, {466, 490}, {455, 540}, {435, 585}, {418, 612}}, {
  pressure = {0.4, 0.75, 0.7, 0.4}
})

-- Internal blend to fuse planes into a cylindrical/spherical mass
blend(m_jug, {angle = 1.45, coverage = 1.0})

-- Mouth interior & flared rim
work(ellipse(370, 269, 30, 6), {hand = "detail", pile = p_interior_dark, coverage = 3.0, fill = true})
b_r2:load(p_jug_hi, 1.0)
b_r2:stroke({{336, 268}, {355, 273}, {372, 274}}, {pressure = 0.85})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(348, 272, {pressure = 0.95})

-- Handle
local pts_handle_hole = {
  {330, 328}, {300, 346}, {270, 378}, {258, 415}, {266, 445}, {278, 430}, {295, 395}, {318, 360}, {332, 335}
}
work(poly(pts_handle_hole, true), {hand = "body", pile = p_wall_dark, coverage = 3.2, fill = true, edge = "firm"})

local pts_handle_spine = {
  {338, 314}, {304, 328}, {265, 355}, {242, 390}, {240, 422}, {252, 455}, {270, 480}
}
local b_r6 = brush{kind = "round", width = 6.0, point = 0.85}
b_r6:load(p_jug_body, 1.0)
b_r6:stroke(pts_handle_spine, {pressure = 0.95, swell = {0.9, 1.3, 0.9}})
b_r3:load(p_jug_lit, 1.0)
b_r3:stroke(pts_handle_spine, {pressure = 0.9})

local pts_handle_under = {
  {332, 324}, {298, 338}, {258, 366}, {236, 400}, {234, 430}, {246, 462}, {264, 485}
}
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke(pts_handle_under, {pressure = 0.9})

b_r1:load(p_jug_glaze, 1.0)
b_r1:stroke({{242, 398}, {240, 415}, {244, 430}}, {pressure = {0.4, 0.95, 0.4}})

-- Shoulder glaze impasto
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{326, 385}, {321, 412}, {316, 438}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r2:load(p_jug_glaze, 1.0)
b_r2:stroke({{324, 398}, {321, 416}, {318, 430}}, {pressure = {0.5, 0.98, 0.4}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(322, 414, {pressure = 0.98})

-- Contact shadow at foot
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{318, 616}, {370, 618}, {425, 616}}, {pressure = {0.7, 0.98, 0.6}})

print("Pitcher completely restored and modeled with absolute clean contour.")

--@ chunk 91
local b_r3 = brush{kind = "round", width = 3.0, point = 0.9}
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. SPECULAR GLAZE IMPASTO ON THE PITCHER
-- North window highlight on shoulder curve
b_r3:load(p_jug_glaze, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 432}}, {pressure = {0.5, 0.98, 0.4}, swell = {0.8, 1.3, 0.7}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(322, 414, {pressure = 0.98})

-- Specular glint on left flared lip of mouth
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(348, 272, {pressure = 0.95})

-- Specular glint on handle crest
b_r1:load(p_jug_glaze, 1.0)
b_r1:stroke({{242, 398}, {240, 415}, {244, 430}}, {pressure = {0.4, 0.95, 0.4}})

-- 2. TACTILE IMPASTO ON THE QUINCE'S SUNLIT CREST
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{568, 575}, {564, 598}, {570, 618}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(565, 595, {pressure = 0.98})

-- 3. UNBROKEN RAZOR CHAMFER HIGHLIGHT ALONG FRONT STONE RIM (y = 715)
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{0, 715}, {350, 715}, {700, 715}, {1000, 715}}, {pressure = {0.6, 0.9, 0.6}, shake = 0.1})

print("Final highlights and chamfer line set.")

--@ chunk 92
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.0, point = 0.9}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. Soften the front of the quince: cover red scratch with radiant golden body paint
b_f10:load(p_quince_gold, 0.95)
b_f10:stroke({{585, 545}, {595, 580}, {610, 620}}, {pressure = 0.85})
b_f10:stroke({{600, 530}, {615, 575}, {625, 625}}, {pressure = 0.85})

-- 2. Soften the core shadow terminator into the golden mass
b_f10:load(p_quince_shade, 0.7)
b_f10:stroke({{645, 510}, {662, 555}, {672, 605}, {668, 645}}, {pressure = {0.4, 0.65, 0.4}})

-- 3. Restate the buttery impasto highlight on the sunlit crest
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{568, 575}, {564, 598}, {570, 618}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(565, 595, {pressure = 0.98})

print("Quince front softened and harmonized into continuous golden volume.")

--@ chunk 93
local pts_quince_knobby = {
  {610, 480}, {632, 478}, {655, 486}, {678, 508}, {696, 540},
  {712, 580}, {716, 620}, {702, 655}, {675, 676}, {630, 680},
  {585, 676}, {556, 650}, {544, 615}, {546, 575}, {562, 532},
  {582, 502}, {602, 485}
}
local m_quince_k = poly(pts_quince_knobby, true)

-- 1. Blend across the quince to melt the stripe and scratch into smooth organic volume
blend(m_quince_k, {angle = 1.35, coverage = 1.0})

-- 2. Buttery impasto highlight on the sunlit crest
local b_r3 = brush{kind = "round", width = 3.0, point = 0.9}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{568, 575}, {564, 598}, {570, 618}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(565, 595, {pressure = 0.98})

-- 3. Restate the woody stem at crown
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_quince_stem, 1.0)
b_r2:stroke({{625, 480}, {628, 465}, {635, 452}, {642, 444}}, {pressure = {0.95, 0.8, 0.4}})
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{627, 478}, {630, 465}, {636, 454}}, {pressure = 0.5})

print("Quince blended seamlessly and highlight restated.")

--@ chunk 94
print("Canvas size: ", W, H)
-- Check current time
print("Current painting time: ", wait(0))

--@ chunk 95
print("Quince drying:", drying(630, 580))
print("Pitcher drying:", drying(360, 480))
print("Table drying:", drying(500, 600))
print("Table right drying:", drying(800, 650))
print("Wall drying:", drying(500, 200))

--@ chunk 96
print("Quince 600, 580:", drying(600, 580))
print("Quince 568, 595:", drying(568, 595))
print("Quince 650, 550:", drying(650, 550))

--@ chunk 97
local t = wait(21 * 24 * 60)
print("Time after waiting:", t)
print("Quince drying:", drying(600, 580))
print("Pitcher drying:", drying(360, 480))
print("Table drying:", drying(500, 600))
print("Table right drying:", drying(800, 650))

--@ chunk 98
-- Check available tubes and test mixing fresh piles
print("Tubes: ", table.concat(tubes(), ", "))

--@ chunk 99
-- Piles for architectural elements: Tabletop, Ledge, Chamfer, Wall
p_stone_lit = pile{{"lead white", 6.5}, {"yellow ochre", 3.0}, {"raw umber", 1.2}, {"red earth", 0.2}, medium = 0.08}
p_stone_mid = pile{{"lead white", 4.5}, {"yellow ochre", 3.0}, {"raw umber", 2.2}, {"bone black", 0.4}, medium = 0.10}
p_stone_back = pile{{"lead white", 3.5}, {"yellow ochre", 2.5}, {"raw umber", 3.2}, {"bone black", 0.8}, medium = 0.12}

p_ledge_face = pile{{"raw umber", 6.0}, {"bone black", 3.5}, {"red earth", 1.0}, {"yellow ochre", 0.8}, medium = 0.10}
p_ledge_chamfer = pile{{"lead white", 8.0}, {"yellow ochre", 1.5}, {"raw umber", 0.5}, medium = 0.05}
p_cast_shadow = pile{{"raw umber", 4.5}, {"bone black", 2.0}, {"yellow ochre", 2.0}, {"lead white", 1.0}, medium = 0.25}

-- 1. Paint the vertical ledge face (y = 715 to 800) with solid, opaque dark masonry paint
local b_f24 = brush("filbert", 24)
local b_f18 = brush("filbert", 18)

b_f24:load(p_ledge_face, 1.0)
b_f24:stroke({{0, 755}, {350, 755}, {700, 755}, {1000, 755}}, {pressure = 0.95})
b_f24:load(p_ledge_face, 1.0)
b_f24:stroke({{0, 785}, {350, 785}, {700, 785}, {1000, 785}}, {pressure = 0.95})
b_f18:load(p_ledge_face, 1.0)
b_f18:stroke({{0, 725}, {350, 725}, {700, 725}, {1000, 725}}, {pressure = 0.95})

-- 2. Paint the limestone tabletop: sweep horizontal strokes across the table plane
-- First the right side and foreground to completely erase the weird yellow shadow bar and ragged stroke ends
-- Back edge of table right (y = 520 to 560)
b_f18:load(p_stone_back, 0.95)
b_f18:stroke({{700, 526}, {850, 525}, {1000, 524}}, {pressure = 0.9})

-- Mid tabletop right (y = 560 to 640)
b_f24:load(p_stone_mid, 1.0)
b_f24:stroke({{690, 560}, {850, 560}, {1000, 560}}, {pressure = 0.95})
b_f24:load(p_stone_lit, 1.0)
b_f24:stroke({{690, 600}, {850, 600}, {1000, 600}}, {pressure = 0.95})
b_f24:load(p_stone_lit, 1.0)
b_f24:stroke({{690, 640}, {850, 640}, {1000, 640}}, {pressure = 0.95})

-- Foreground table sweeps across the entire width (y = 665 to 715)
-- in front of pitcher and quince
b_f24:load(p_stone_lit, 1.0)
b_f24:stroke({{0, 680}, {300, 680}, {600, 680}, {1000, 680}}, {pressure = 0.95})
b_f24:load(p_stone_lit, 1.0)
b_f24:stroke({{0, 710}, {300, 710}, {600, 710}, {1000, 710}}, {pressure = 0.98})

-- And table surface to the left of the pitcher (y = 520 to 670)
b_f18:load(p_stone_back, 0.9)
b_f18:stroke({{0, 524}, {120, 524}, {240, 524}}, {pressure = 0.85})
b_f24:load(p_stone_mid, 0.95)
b_f24:stroke({{0, 560}, {120, 560}, {250, 560}}, {pressure = 0.9})
b_f24:load(p_stone_lit, 1.0)
b_f24:stroke({{0, 600}, {120, 600}, {260, 600}}, {pressure = 0.95})
b_f24:load(p_stone_lit, 1.0)
b_f24:stroke({{0, 640}, {140, 640}, {270, 640}}, {pressure = 0.95})

-- Blend table plane horizontally to fuse strokes into a single stone surface
local b_badger = brush("badger", 30)
b_badger:stroke({{0, 695}, {500, 695}, {1000, 695}}, {pressure = 0.3})
b_badger:stroke({{680, 620}, {850, 620}, {1000, 620}}, {pressure = 0.25})

-- Crisp raking chamfer line along y = 715
local b_r1 = brush{kind = "round", width = 1.6, point = 0.98}
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{0, 715}, {350, 715}, {700, 715}, {1000, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.05})

print("Tabletop, ledge face, and chamfer cleanly unified.")

--@ chunk 100
-- Piles for Wall, Tabletop, Ledge Face, and Chamfer
p_wall_dark = pile{{"raw umber", 6.5}, {"bone black", 4.0}, {"yellow ochre", 1.0}, medium = 0.10}
p_wall_mid  = pile{{"raw umber", 5.0}, {"yellow ochre", 3.0}, {"bone black", 2.0}, {"lead white", 1.0}, medium = 0.12}

p_stone_lit = pile{{"lead white", 7.0}, {"yellow ochre", 3.0}, {"raw umber", 1.2}, {"red earth", 0.2}, medium = 0.08}
p_stone_mid = pile{{"lead white", 5.0}, {"yellow ochre", 3.2}, {"raw umber", 2.0}, {"bone black", 0.4}, medium = 0.10}
p_stone_back = pile{{"lead white", 3.8}, {"yellow ochre", 2.5}, {"raw umber", 3.0}, {"bone black", 0.8}, medium = 0.12}

p_ledge_face = pile{{"raw umber", 6.0}, {"bone black", 4.0}, {"red earth", 1.0}, {"yellow ochre", 0.8}, medium = 0.10}
p_ledge_chamfer = pile{{"lead white", 8.5}, {"yellow ochre", 1.2}, {"raw umber", 0.4}, medium = 0.05}

-- 1. COVER THE TOP OF THE OLD QUINCE ON THE WALL (y = 460 to 522, x = 520 to 720)
local b_f24 = brush("filbert", 24)
for y = 465, 520, 12 do
  b_f24:load(p_wall_dark, 1.0)
  b_f24:stroke({{510, y}, {620, y}, {730, y}}, {pressure = 0.95})
end

-- 2. SOLID OPAQUE TABLETOP WITH OVERLAPPING HORIZONTAL STROKES (step = 10 units with brush 26)
local b_f26 = brush("filbert", 26)

-- A. Tabletop left of pitcher (x = -10 to 255, y = 522 to 614)
for y = 522, 614, 10 do
  local p = (y < 550) and p_stone_back or ((y < 585) and p_stone_mid or p_stone_lit)
  b_f26:load(p, 1.0)
  b_f26:stroke({{-10, y}, {120, y}, {255, y}}, {pressure = 0.95})
end

-- B. Tabletop right of pitcher (x = 445 to 1010, y = 522 to 614)
for y = 522, 614, 10 do
  local p = (y < 550) and p_stone_back or ((y < 585) and p_stone_mid or p_stone_lit)
  b_f26:load(p, 1.0)
  b_f26:stroke({{445, y}, {730, y}, {1010, y}}, {pressure = 0.95})
end

-- C. Tabletop foreground across ENTIRE canvas (x = -10 to 1010, y = 615 to 715)
for y = 615, 715, 10 do
  b_f26:load(p_stone_lit, 1.0)
  b_f26:stroke({{-10, y}, {350, y}, {700, y}, {1010, y}}, {pressure = 0.98})
end

-- 3. SOLID VERTICAL LEDGE FACE (y = 716 to 805, x = -10 to 1010)
for y = 722, 802, 12 do
  b_f26:load(p_ledge_face, 1.0)
  b_f26:stroke({{-10, y}, {350, y}, {700, y}, {1010, y}}, {pressure = 0.95})
end

-- 4. SOFTEN AND FUSE THE TABLETOP AND LEDGE WITH A LARGE BADGER BLENDER
local b_badger = brush("badger", 36)
-- Horizontal sweeps over the stone table to fuse the paint into smooth limestone
b_badger:stroke({{-10, 535}, {120, 535}, {250, 535}}, {pressure = 0.25})
b_badger:stroke({{450, 535}, {730, 535}, {1010, 535}}, {pressure = 0.25})
b_badger:stroke({{-10, 580}, {500, 580}, {1010, 580}}, {pressure = 0.25})
b_badger:stroke({{-10, 640}, {500, 640}, {1010, 640}}, {pressure = 0.25})
b_badger:stroke({{-10, 695}, {500, 695}, {1010, 695}}, {pressure = 0.30})

-- Soften the ledge face
b_badger:stroke({{-10, 755}, {500, 755}, {1010, 755}}, {pressure = 0.25})

-- 5. CRISP, CONTINUOUS RAKING CHAMFER HIGHLIGHT (y = 715)
local b_r2 = brush{kind = "round", width = 1.8, point = 0.98}
b_r2:load(p_ledge_chamfer, 1.0)
b_r2:stroke({{-5, 715}, {350, 715}, {700, 715}, {1005, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.05})

print("Continuous architectural space established.")

--@ chunk 101
-- 1. UNIFY WALL ABOVE TABLE (y = 450 to 520)
-- Left of pitcher (-10 to 240)
local b_f24 = brush("filbert", 24)
for y = 450, 520, 12 do
  b_f24:load(p_wall_dark, 1.0)
  b_f24:stroke({{-10, y}, {120, y}, {245, y}}, {pressure = 0.95})
end

-- Right of pitcher (465 to 1010)
for y = 450, 520, 12 do
  b_f24:load(p_wall_dark, 1.0)
  b_f24:stroke({{465, y}, {750, y}, {1010, y}}, {pressure = 0.95})
end

-- Soften the wall above the table
local b_badger = brush("badger", 32)
b_badger:stroke({{-10, 485}, {120, 485}, {245, 485}}, {pressure = 0.2})
b_badger:stroke({{465, 485}, {750, 485}, {1010, 485}}, {pressure = 0.2})

-- 2. SOLID LIMESTONE TABLETOP RUNNING RIGHT UP TO PITCHER CONTOUR
-- Pitcher left boundary coordinates:
-- y=522: x=275; y=550: x=285; y=580: x=305; y=615: x=330
-- Pitcher right boundary coordinates:
-- y=522: x=465; y=550: x=455; y=580: x=440; y=615: x=420

local b_f20 = brush("filbert", 20)

-- Left of pitcher (y = 522 to 615)
for y = 522, 615, 8 do
  local x_right = 270 + (y - 522) * (330 - 270) / (615 - 522)
  local p = (y < 550) and p_stone_back or ((y < 585) and p_stone_mid or p_stone_lit)
  b_f20:load(p, 1.0)
  b_f20:stroke({{-10, y}, {x_right * 0.5, y}, {x_right, y}}, {pressure = 0.95})
end

-- Right of pitcher (y = 522 to 615)
for y = 522, 615, 8 do
  local x_left = 468 - (y - 522) * (468 - 418) / (615 - 522)
  local p = (y < 550) and p_stone_back or ((y < 585) and p_stone_mid or p_stone_lit)
  b_f20:load(p, 1.0)
  b_f20:stroke({{x_left, y}, {700, y}, {1010, y}}, {pressure = 0.95})
end

-- Re-sweep foreground table (y = 615 to 715) to maintain perfect luminous continuity
local b_f26 = brush("filbert", 26)
for y = 615, 715, 10 do
  b_f26:load(p_stone_lit, 1.0)
  b_f26:stroke({{-10, y}, {350, y}, {700, y}, {1010, y}}, {pressure = 0.98})
end

-- Soft horizontal fusion with badger
b_badger:stroke({{-10, 540}, {260, 540}}, {pressure = 0.25})
b_badger:stroke({{460, 540}, {750, 540}, {1010, 540}}, {pressure = 0.25})
b_badger:stroke({{-10, 600}, {290, 600}}, {pressure = 0.25})
b_badger:stroke({{430, 600}, {750, 600}, {1010, 600}}, {pressure = 0.25})
b_badger:stroke({{-10, 665}, {500, 665}, {1010, 665}}, {pressure = 0.3})
b_badger:stroke({{-10, 705}, {500, 705}, {1010, 705}}, {pressure = 0.3})

-- Razor chamfer along front rim (y = 715)
local b_r2 = brush{kind = "round", width = 1.8, point = 0.98}
b_r2:load(p_ledge_chamfer, 1.0)
b_r2:stroke({{-5, 715}, {350, 715}, {700, 715}, {1005, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.04})

print("Tabletop perfectly flush with pitcher and wall.")

--@ chunk 102
-- Mix Piles for Terracotta Earthenware Pitcher
p_jug_lit     = pile{{"yellow ochre", 4.5}, {"red earth", 3.0}, {"lead white", 2.8}, {"vermilion", 0.7}, medium = 0.08}
p_jug_body    = pile{{"red earth", 4.2}, {"yellow ochre", 4.0}, {"raw umber", 2.0}, {"lead white", 1.0}, medium = 0.10}
p_jug_core    = pile{{"raw umber", 5.5}, {"red earth", 3.0}, {"bone black", 1.8}, medium = 0.12}
p_jug_deep    = pile{{"bone black", 5.0}, {"raw umber", 4.0}, {"red earth", 1.5}, medium = 0.10}
p_jug_reflect = pile{{"yellow ochre", 4.2}, {"raw umber", 2.8}, {"red earth", 2.2}, {"lead white", 1.5}, medium = 0.12}
p_jug_hi      = pile{{"lead white", 5.5}, {"yellow ochre", 3.2}, {"red earth", 1.2}, {"vermilion", 0.3}, medium = 0.06}
p_jug_glaze   = pile{{"lead white", 8.5}, {"yellow ochre", 1.5}, {"raw umber", 0.3}, medium = 0.04}

-- 1. BASE BODY COAT OVER PITCHER (y = 265 to 618)
local b_f18 = brush("filbert", 18)
local b_f12 = brush("filbert", 12)
local b_f8  = brush("filbert", 8)

-- Belly & neck fill in rich opaque terracotta body
for y = 270, 615, 8 do
  local xl, xr
  if y < 315 then
    -- Flared neck
    xl = 338 + (342 - 338) * (y - 270) / 45
    xr = 404 - (404 - 400) * (y - 270) / 45
  elseif y < 440 then
    -- Shoulder expanding
    xl = 342 - (342 - 272) * (y - 315) / 125
    xr = 400 + (465 - 400) * (y - 315) / 125
  elseif y < 510 then
    -- Maximum belly width
    xl = 272 - (272 - 264) * (y - 440) / 70
    xr = 465 + (474 - 465) * (y - 440) / 70
  elseif y < 580 then
    -- Belly narrowing
    xl = 264 + (305 - 264) * (y - 510) / 70
    xr = 474 - (474 - 442) * (y - 510) / 70
  else
    -- Foot
    xl = 305 + (325 - 305) * (y - 580) / 35
    xr = 442 - (442 - 422) * (y - 580) / 35
  end
  b_f18:load(p_jug_body, 1.0)
  b_f18:stroke({{xl, y}, {(xl + xr) * 0.5, y}, {xr, y}}, {pressure = 0.95})
end

-- 2. MODEL LIGHT AND SHADE ON THE BODY (Wet-in-wet)
-- Illuminated flank on the left (raking north light)
local b_f14 = brush("filbert", 14)
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{345, 274}, {348, 320}, {330, 380}, {295, 440}, {285, 490}, {298, 545}, {328, 595}, {345, 614}}, {
  pressure = {0.85, 0.98, 0.95, 0.9, 0.75}
})
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{358, 276}, {360, 325}, {348, 390}, {322, 450}, {314, 500}, {328, 555}, {352, 610}}, {
  pressure = {0.8, 0.95, 0.9, 0.85, 0.7}
})

-- Broad warm crest of light on the belly
b_f18:load(p_jug_lit, 0.95)
b_f18:stroke({{340, 420}, {330, 470}, {335, 520}, {355, 570}}, {pressure = {0.7, 0.95, 0.8, 0.6}})

-- Soft light crest on upper shoulder
b_f12:load(p_jug_hi, 1.0)
b_f12:stroke({{338, 360}, {325, 400}, {312, 440}}, {pressure = {0.6, 0.95, 0.6}, swell = {0.8, 1.2, 0.8}})

-- Core shadow down the right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{392, 274}, {395, 325}, {418, 385}, {448, 445}, {452, 495}, {435, 550}, {408, 605}}, {
  pressure = {0.85, 0.98, 0.95, 0.95, 0.8}
})
b_f12:load(p_jug_deep, 1.0)
b_f12:stroke({{418, 375}, {446, 425}, {460, 475}, {455, 525}, {438, 575}, {415, 610}}, {
  pressure = {0.7, 0.95, 0.9, 0.7}
})

-- Warm table bounce along the far right contour
b_f10 = brush("filbert", 10)
b_f10:load(p_jug_reflect, 0.95)
b_f10:stroke({{404, 320}, {435, 380}, {465, 435}, {470, 485}, {458, 535}, {438, 580}, {420, 612}}, {
  pressure = {0.4, 0.75, 0.75, 0.4}
})

-- Soft badger blend across the belly to melt planes into continuous sculptural roundness
local b_badger = brush("badger", 22)
b_badger:stroke({{310, 460}, {370, 460}, {440, 460}}, {pressure = 0.22})
b_badger:stroke({{320, 520}, {375, 520}, {435, 520}}, {pressure = 0.22})
b_badger:stroke({{335, 570}, {375, 570}, {420, 570}}, {pressure = 0.22})

-- 3. MOUTH, RIM, AND HOLLOW INTERIOR
-- Interior deep shadow
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
b_f8:load(p_jug_deep, 1.0)
b_f8:stroke({{344, 269}, {370, 270}, {396, 269}}, {pressure = 0.95})
b_f8:stroke({{350, 272}, {370, 273}, {390, 272}}, {pressure = 0.9})

-- Flared lip rim
local b_r2 = brush{kind = "round", width = 2.2, point = 0.95}
b_r2:load(p_jug_lit, 1.0)
b_r2:stroke({{336, 268}, {355, 273}, {372, 274}}, {pressure = 0.9})
b_r2:load(p_jug_core, 0.9)
b_r2:stroke({{372, 274}, {390, 273}, {404, 268}}, {pressure = 0.85})

-- Specular glint on left flared lip
local b_r1 = brush{kind = "round", width = 1.4, point = 0.98}
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(348, 272, {pressure = 0.98})

-- 4. ROBUST PULLED STRAP HANDLE (Asa)
-- First clean the negative space inside the handle loop with wall paint
local pts_handle_hole = {
  {338, 330}, {305, 350}, {274, 382}, {262, 420}, {270, 450}, {284, 435}, {302, 398}, {324, 362}, {338, 335}
}
local b_f8_wall = brush("filbert", 8)
for _, pt in ipairs(pts_handle_hole) do
  b_f8_wall:load(p_wall_dark, 1.0)
  b_f8_wall:touch(pt[1], pt[2], {pressure = 0.9})
end
b_f12:load(p_wall_dark, 1.0)
b_f12:stroke({{285, 410}, {310, 365}}, {pressure = 0.95})

-- Now paint the sturdy earthenware strap handle
-- Thick spine of the handle
local b_r6 = brush{kind = "round", width = 6.5, point = 0.85}
local pts_handle_spine = {
  {342, 314}, {306, 326}, {266, 354}, {242, 390}, {238, 424}, {250, 458}, {272, 484}
}
b_r6:load(p_jug_body, 1.0)
b_r6:stroke(pts_handle_spine, {pressure = 0.98, swell = {0.9, 1.3, 0.95}})

-- Upper lit facet of the handle
local b_r4 = brush{kind = "round", width = 3.8, point = 0.9}
b_r4:load(p_jug_lit, 1.0)
b_r4:stroke(pts_handle_spine, {pressure = 0.92})

-- Highlight on the outer crest of the handle
b_r2:load(p_jug_hi, 1.0)
b_r2:stroke({{300, 330}, {265, 358}, {242, 392}, {238, 422}}, {pressure = {0.5, 0.95, 0.95, 0.5}})

-- Glaze catchlight on the handle crest
b_r1:load(p_jug_glaze, 1.0)
b_r1:stroke({{242, 400}, {240, 416}, {243, 428}}, {pressure = {0.4, 0.98, 0.4}})

-- Deep shaded underside of the handle
local pts_handle_under = {
  {334, 326}, {298, 340}, {258, 368}, {236, 402}, {233, 430}, {246, 464}, {266, 488}
}
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke(pts_handle_under, {pressure = 0.92})

-- 5. FOOTRING AND FIRM CONTACT SHADOW
-- Footring rim
b_r3:load(p_jug_body, 1.0)
b_r3:stroke({{325, 615}, {372, 617}, {422, 615}}, {pressure = 0.95})
b_r2:load(p_jug_lit, 0.9)
b_r2:stroke({{326, 615}, {355, 616}}, {pressure = 0.85})
b_r2:load(p_jug_core, 0.9)
b_r2:stroke({{385, 616}, {422, 615}}, {pressure = 0.85})

-- Firm dark contact shadow beneath foot
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{322, 617}, {370, 619}, {425, 617}}, {pressure = {0.8, 1.0, 0.7}})

-- 6. GLAZE IMPASTO ON SHOULDER
-- Rich north-window specular highlight on the glazed shoulder curve
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r2:load(p_jug_glaze, 1.0)
b_r2:stroke({{324, 402}, {321, 418}, {319, 430}}, {pressure = {0.5, 0.98, 0.4}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})

print("Earthenware pitcher fully modeled with robust handle, footring, and glaze highlights.")

--@ chunk 103
-- 1. CLEAN THE PITCHER'S OUTER SILHOUETTES
-- Background wall along right neck and shoulder (y = 268 to 520)
local pts_wall_carve = {
  {406, 268}, {403, 310}, {408, 335}, {425, 365}, {448, 405}, {466, 445}, {473, 485}, {470, 520}
}
local b_r2 = brush{kind = "round", width = 2.4, point = 0.95}
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke(pts_wall_carve, {pressure = 0.95})

-- Table along right lower flank (y = 520 to 615)
local pts_table_carve = {
  {470, 520}, {462, 545}, {448, 575}, {435, 595}, {423, 615}
}
b_r2:load(p_stone_mid, 1.0)
b_r2:stroke(pts_table_carve, {pressure = 0.95})

-- Background wall inside handle loop
local pts_handle_loop = {
  {338, 328}, {312, 345}, {280, 375}, {262, 412}, {266, 445}, {278, 435}, {296, 398}, {320, 360}, {338, 332}
}
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke(pts_handle_loop, {pressure = 0.95})
local b_f6 = brush("filbert", 6)
b_f6:load(p_wall_dark, 1.0)
b_f6:touch(295, 385, {pressure = 0.9})
b_f6:touch(280, 415, {pressure = 0.9})

-- 2. ENRICH PITCHER CORE SHADOW & VOLUME
local b_f12 = brush("filbert", 12)
local b_f10 = brush("filbert", 10)

-- Core shadow down the right side of the belly
b_f12:load(p_jug_core, 1.0)
b_f12:stroke({{400, 340}, {425, 395}, {448, 450}, {450, 505}, {435, 555}, {410, 605}}, {
  pressure = {0.7, 0.95, 0.98, 0.95, 0.75}
})
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{430, 415}, {452, 465}, {452, 515}, {438, 565}, {416, 610}}, {
  pressure = {0.6, 0.9, 0.9, 0.6}
})

-- Warm reflected bounce along the turned edge
b_f8 = brush("filbert", 8)
b_f8:load(p_jug_reflect, 0.9)
b_f8:stroke({{404, 325}, {426, 375}, {452, 430}, {466, 480}, {456, 530}, {438, 575}, {420, 612}}, {
  pressure = {0.3, 0.65, 0.7, 0.35}
})

-- Soft internal blend across belly
local b_badger = brush("badger", 18)
b_badger:stroke({{330, 480}, {390, 480}, {450, 480}}, {pressure = 0.2})
b_badger:stroke({{340, 540}, {390, 540}, {440, 540}}, {pressure = 0.2})

-- 3. HANDLE CONSOLIDATION
-- Solid terracotta strap
local b_r5 = brush{kind = "round", width = 5.5, point = 0.88}
local pts_handle_spine = {
  {342, 316}, {310, 328}, {270, 355}, {242, 392}, {238, 426}, {248, 460}, {270, 486}
}
b_r5:load(p_jug_body, 1.0)
b_r5:stroke(pts_handle_spine, {pressure = 0.95, swell = {0.85, 1.2, 0.9}})

-- Lit face on handle
local b_r3 = brush{kind = "round", width = 3.0, point = 0.9}
b_r3:load(p_jug_lit, 1.0)
b_r3:stroke({{335, 318}, {306, 330}, {268, 358}, {242, 395}, {239, 424}}, {pressure = 0.9})
b_r2:load(p_jug_hi, 1.0)
b_r2:stroke({{295, 336}, {262, 364}, {242, 396}, {239, 422}}, {pressure = {0.5, 0.95, 0.5}})

-- Glaze gleam on handle crest
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}
b_r1:load(p_jug_glaze, 1.0)
b_r1:stroke({{242, 400}, {240, 415}, {242, 426}}, {pressure = {0.4, 0.98, 0.4}})

-- Deep shaded underside of handle
local pts_handle_under = {
  {334, 326}, {300, 340}, {260, 368}, {236, 404}, {234, 432}, {244, 464}, {264, 488}
}
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke(pts_handle_under, {pressure = 0.9})

-- Clay thumb attachment at belly join
b_f6:load(p_jug_body, 1.0)
b_f6:touch(270, 486, {pressure = 0.9})
b_r2:load(p_jug_lit, 0.9)
b_r2:stroke({{266, 482}, {274, 488}}, {pressure = 0.85})

-- 4. MOUTH & RIM REFINEMENT
b_f8:load(p_jug_deep, 1.0)
b_f8:stroke({{344, 269}, {370, 270}, {396, 269}}, {pressure = 0.95})
b_r2:load(p_jug_lit, 1.0)
b_r2:stroke({{335, 268}, {355, 273}, {372, 274}}, {pressure = 0.9})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(348, 272, {pressure = 1.0})

-- 5. FOOTRING AND OCCLUSION SHADOW
b_r3:load(p_jug_body, 1.0)
b_r3:stroke({{325, 615}, {372, 617}, {422, 615}}, {pressure = 0.95})
b_r2:load(p_jug_lit, 0.9)
b_r2:stroke({{325, 615}, {355, 616}}, {pressure = 0.85})
b_r2:load(p_jug_core, 0.9)
b_r2:stroke({{385, 616}, {422, 615}}, {pressure = 0.85})

-- Dark occlusion contact line under foot
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.8, 1.0, 0.8}})

-- 6. SHOULDER GLAZE IMPASTO
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r2:load(p_jug_glaze, 1.0)
b_r2:stroke({{324, 402}, {321, 418}, {319, 430}}, {pressure = {0.5, 0.98, 0.4}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})

print("Pitcher refined and anchored.")

--@ chunk 104
-- Quince Piles
p_quince_lit     = pile{{"chrome yellow", 5.5}, {"yellow ochre", 3.5}, {"lead white", 2.5}, {"green earth", 0.4}, medium = 0.08}
p_quince_gold    = pile{{"chrome yellow", 6.0}, {"yellow ochre", 4.0}, {"red earth", 0.5}, medium = 0.10}
p_quince_crest   = pile{{"lead white", 7.5}, {"chrome yellow", 3.0}, {"yellow ochre", 1.0}, medium = 0.05}
p_quince_blush   = pile{{"yellow ochre", 4.0}, {"red earth", 2.5}, {"vermilion", 1.0}, {"raw umber", 0.8}, medium = 0.12}
p_quince_shade   = pile{{"raw umber", 5.0}, {"yellow ochre", 3.5}, {"red earth", 1.5}, {"bone black", 0.8}, medium = 0.12}
p_quince_deep    = pile{{"raw umber", 6.0}, {"bone black", 3.0}, {"yellow ochre", 1.5}, medium = 0.10}
p_quince_reflect = pile{{"yellow ochre", 4.5}, {"lead white", 2.5}, {"raw umber", 2.0}, {"green earth", 0.5}, medium = 0.12}
p_quince_stem    = pile{{"raw umber", 6.0}, {"bone black", 2.5}, {"yellow ochre", 1.5}, {"lead white", 0.8}, medium = 0.08}

-- 1. TOUCH UP PITCHER LEFT FLANK BELOW HANDLE (x = 255 to 285, y = 490 to 550)
local b_f10 = brush("filbert", 10)
b_f10:load(p_jug_lit, 1.0)
b_f10:stroke({{265, 485}, {262, 515}, {272, 545}}, {pressure = 0.95})
b_f10:load(p_jug_body, 1.0)
b_f10:stroke({{274, 490}, {278, 525}, {288, 555}}, {pressure = 0.92})

-- 2. BLOCK IN THE KNOBBY RIPE QUINCE (x = 550 to 715, y = 475 to 676)
local b_f16 = brush("filbert", 16)
local b_f12 = brush("filbert", 12)

-- Block in mass with radiant golden midtone
for y = 480, 674, 8 do
  local xl, xr
  if y < 510 then
    -- Tapered crown around stem socket
    xl = 612 - (612 - 582) * (y - 480) / 30
    xr = 648 + (674 - 648) * (y - 480) / 30
  elseif y < 560 then
    -- Upper knobby swelling
    xl = 582 - (582 - 558) * (y - 510) / 50
    xr = 674 + (702 - 674) * (y - 510) / 50
  elseif y < 630 then
    -- Main voluptuous belly
    xl = 558 - (558 - 552) * (y - 560) / 70
    xr = 702 + (714 - 702) * (y - 560) / 70
  else
    -- Base rounding toward foot
    xl = 552 + (580 - 552) * (y - 630) / 44
    xr = 714 - (714 - 685) * (y - 630) / 44
  end
  b_f16:load(p_quince_gold, 1.0)
  b_f16:stroke({{xl, y}, {(xl + xr) * 0.5, y}, {xr, y}}, {pressure = 0.95})
end

-- 3. MODEL LIGHT AND SHADE ACROSS THE LOBES
-- Luminous sunlit left lobe
b_f12:load(p_quince_lit, 1.0)
b_f12:stroke({{610, 485}, {585, 515}, {565, 560}, {560, 605}, {575, 645}, {605, 670}}, {
  pressure = {0.8, 0.98, 0.98, 0.98, 0.9, 0.75}
})
b_f12:load(p_quince_lit, 1.0)
b_f12:stroke({{620, 500}, {600, 535}, {585, 580}, {582, 625}, {600, 660}}, {
  pressure = {0.8, 0.95, 0.95, 0.9, 0.7}
})

-- Central lobe turning slightly into light
b_f12:load(p_quince_gold, 0.95)
b_f12:stroke({{635, 505}, {625, 545}, {618, 595}, {625, 645}}, {pressure = {0.75, 0.9, 0.9, 0.7}})

-- Core shadow down the right flank and furrows
b_f12:load(p_quince_shade, 1.0)
b_f12:stroke({{648, 495}, {668, 530}, {686, 575}, {692, 620}, {675, 658}, {650, 672}}, {
  pressure = {0.8, 0.98, 0.98, 0.95, 0.9, 0.75}
})
local b_f8 = brush("filbert", 8)
b_f8:load(p_quince_deep, 0.95)
b_f8:stroke({{665, 520}, {685, 565}, {698, 610}, {685, 650}}, {pressure = {0.6, 0.92, 0.92, 0.65}})

-- Warm limestone bounce on lower right contour
b_f8:load(p_quince_reflect, 0.9)
b_f8:stroke({{675, 510}, {698, 550}, {710, 595}, {708, 635}, {690, 665}}, {pressure = {0.4, 0.75, 0.75, 0.4}})

-- 4. SOFT BADGER BLEND ACROSS KNOBBY PLANES
local b_badger = brush("badger", 16)
b_badger:stroke({{580, 540}, {635, 540}, {680, 540}}, {pressure = 0.22})
b_badger:stroke({{575, 595}, {635, 595}, {690, 595}}, {pressure = 0.22})
b_badger:stroke({{585, 645}, {635, 645}, {680, 645}}, {pressure = 0.22})

-- 5. RUSSET BLUSH SCUMBLE
local b_stip = brush("stippler", 5)
b_stip:load(p_quince_blush, 0.55)
b_stip:stroke({{575, 555}, {590, 590}, {608, 630}}, {pressure = 0.35, shake = 0.3})

-- 6. BUTTERY IMPASTO CREST HIGHLIGHT
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{572, 570}, {566, 595}, {570, 620}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(567, 593, {pressure = 1.0})

-- Soft secondary glint on upper shoulder
b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_quince_crest, 0.85)
b_r2:stroke({{610, 502}, {605, 518}}, {pressure = 0.7})

-- 7. GNARLED WOODY STEM & CROWN SOCKET
-- Crown socket depression
b_r2:load(p_quince_deep, 1.0)
b_r2:touch(628, 482, {pressure = 0.95})
b_r1:load(p_quince_shade, 0.9)
b_r1:stroke({{622, 484}, {634, 482}}, {pressure = 0.8})

-- Arching woody stalk
b_r2:load(p_quince_stem, 1.0)
b_r2:stroke({{628, 480}, {632, 465}, {638, 452}, {645, 444}}, {pressure = {0.98, 0.85, 0.5}})
-- Lit bark facet on stem
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{629, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})
-- Stem cast shadow on crown
b_r1:load(p_quince_deep, 0.9)
b_r1:stroke({{632, 484}, {638, 488}, {644, 492}}, {pressure = 0.7})

-- Calyx eye at base
b_r1:load(p_quince_deep, 1.0)
b_r1:touch(655, 673, {pressure = 0.9})
b_r1:stroke({{652, 672}, {658, 674}}, {pressure = 0.75})

-- Firm dark contact shadow under base
b_r3:load(p_quince_deep, 1.0)
b_r3:stroke({{575, 675}, {615, 677}, {660, 676}, {688, 672}}, {pressure = {0.8, 1.0, 0.9, 0.7}})

print("Ripe knobby quince fully painted and grounded.")

--@ chunk 105
-- Opaque Quince Body & Modeling Piles
p_quince_dense   = pile{{"lead white", 4.0}, {"yellow ochre", 5.0}, {"chrome yellow", 4.5}, {"red earth", 0.8}, medium = 0.06}
p_quince_lit_op  = pile{{"lead white", 6.0}, {"chrome yellow", 4.5}, {"yellow ochre", 2.5}, medium = 0.05}
p_quince_core_op = pile{{"raw umber", 5.5}, {"yellow ochre", 3.0}, {"red earth", 2.2}, {"bone black", 1.0}, medium = 0.08}
p_quince_refl_op = pile{{"yellow ochre", 4.5}, {"lead white", 3.0}, {"raw umber", 1.5}, {"red earth", 0.5}, medium = 0.08}
p_quince_crest   = pile{{"lead white", 8.0}, {"chrome yellow", 2.5}, {"yellow ochre", 0.8}, medium = 0.04}

-- 1. SCULPT THE OUTER SILHOUETTES WITH BACKGROUND & TABLE PAINT (Carving away saw-tooth steps)
local b_r3 = brush{kind = "round", width = 3.2, point = 0.95}
local b_f10 = brush("filbert", 10)

-- Carve wall above y = 520 around quince crown and upper flanks
local pts_wall_crown_left = {
  {540, 520}, {555, 510}, {575, 492}, {605, 480}, {622, 476}
}
b_f10:load(p_wall_dark, 1.0)
b_f10:stroke(pts_wall_crown_left, {pressure = 0.95})

local pts_wall_crown_right = {
  {635, 476}, {655, 482}, {680, 500}, {695, 520}
}
b_f10:load(p_wall_dark, 1.0)
b_f10:stroke(pts_wall_crown_right, {pressure = 0.95})

-- Carve table below y = 520 along lower flanks and base
local pts_table_quince_left = {
  {545, 520}, {550, 550}, {548, 595}, {555, 635}, {575, 668}, {595, 678}
}
b_f10:load(p_stone_mid, 1.0)
b_f10:stroke(pts_table_quince_left, {pressure = 0.95})

local pts_table_quince_right = {
  {695, 520}, {708, 555}, {714, 600}, {705, 642}, {685, 670}, {660, 678}
}
b_f10:load(p_stone_mid, 1.0)
b_f10:stroke(pts_table_quince_right, {pressure = 0.95})

-- 2. MODEL THE QUINCE WITH CURVING VERTICAL & CONTOUR STROKES
local b_f14 = brush("filbert", 14)
local b_f12 = brush("filbert", 12)

-- A. Dense golden body coat laid along the organic lobes
b_f14:load(p_quince_dense, 1.0)
b_f14:stroke({{618, 480}, {590, 510}, {568, 555}, {560, 605}, {576, 648}, {605, 674}}, {pressure = 0.98})
b_f14:load(p_quince_dense, 1.0)
b_f14:stroke({{625, 482}, {610, 520}, {595, 570}, {590, 625}, {615, 672}}, {pressure = 0.98})
b_f14:load(p_quince_dense, 1.0)
b_f14:stroke({{632, 484}, {635, 530}, {632, 585}, {635, 640}, {645, 674}}, {pressure = 0.95})

-- B. Luminous sunlit crest on the left lobe
b_f12:load(p_quince_lit_op, 1.0)
b_f12:stroke({{612, 486}, {585, 518}, {566, 565}, {562, 610}, {578, 650}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.8}, swell = {0.85, 1.25, 0.85}
})
b_f12:load(p_quince_lit_op, 1.0)
b_f12:stroke({{595, 535}, {578, 580}, {576, 620}, {592, 655}}, {
  pressure = {0.8, 0.95, 0.95, 0.8}
})

-- C. Deep sculptural core shadow along the right flank
b_f14:load(p_quince_core_op, 1.0)
b_f14:stroke({{635, 485}, {662, 515}, {686, 565}, {694, 615}, {678, 658}, {645, 674}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.9, 0.75}
})
b_f10:load(p_quince_deep, 0.95)
b_f10:stroke({{668, 525}, {690, 570}, {696, 615}, {682, 655}}, {
  pressure = {0.7, 0.95, 0.95, 0.7}
})

-- D. Warm table bounce into lower-right contour
b_f10:load(p_quince_refl_op, 0.9)
b_f10:stroke({{675, 515}, {698, 555}, {708, 600}, {704, 638}, {686, 666}}, {
  pressure = {0.4, 0.8, 0.8, 0.4}
})

-- E. Soft badger blend to fuse the knobby planes into continuous sculptural volume
local b_badger = brush("badger", 16)
b_badger:stroke({{575, 545}, {630, 545}, {685, 545}}, {pressure = 0.22})
b_badger:stroke({{570, 600}, {630, 600}, {695, 600}}, {pressure = 0.22})
b_badger:stroke({{585, 650}, {635, 650}, {680, 650}}, {pressure = 0.22})

-- 3. TACTILE SURFACE DETAILS
-- Russet orchard blush scumble across the illuminated curve
local b_stip = brush("stippler", 5)
b_stip:load(p_quince_blush, 0.5)
b_stip:stroke({{576, 555}, {592, 595}, {612, 635}}, {pressure = 0.35, shake = 0.3})

-- Buttery impasto highlight on the knobby crest of the sunlit lobe
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{572, 572}, {566, 598}, {570, 622}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(567, 596, {pressure = 1.0})

-- Woody stem arching from crown socket
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_quince_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_quince_stem, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.85, 0.5}})
b_r1:load(p_quince_lit_op, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})
b_r1:load(p_quince_deep, 0.85)
b_r1:stroke({{632, 484}, {638, 488}, {644, 492}}, {pressure = 0.65})

-- Calyx eye at base
b_r1:load(p_quince_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})
b_r1:stroke({{651, 673}, {657, 675}}, {pressure = 0.8})

-- Firm dark contact shadow under base
b_r3:load(p_quince_deep, 1.0)
b_r3:stroke({{576, 675}, {615, 677}, {660, 676}, {688, 672}}, {pressure = {0.8, 1.0, 0.9, 0.7}})

print("Quince resculpted with organic volume, lost-and-found edges, and tactile impasto.")

--@ chunk 106
-- Knobby Quince Silhouette Polygon
local pts_quince = {
  {628, 478}, {646, 484}, {670, 508}, {694, 546}, {710, 595},
  {706, 636}, {686, 665}, {662, 676}, {632, 678}, {598, 676},
  {574, 668}, {556, 642}, {546, 608}, {554, 562}, {575, 522},
  {598, 494}, {618, 480}
}
local m_quince = poly(pts_quince, true)

-- 1. CLEAN THE SURROUNDING FIELD WITH TABLE & WALL PAINT
-- Sweep table horizontally around the quince base and flanks
local b_f20 = brush("filbert", 20)
b_f20:load(p_stone_mid, 1.0)
b_f20:stroke({{450, 530}, {545, 530}}, {pressure = 0.95})
b_f20:stroke({{450, 565}, {545, 565}}, {pressure = 0.95})
b_f20:stroke({{450, 605}, {540, 605}}, {pressure = 0.95})
b_f20:stroke({{450, 645}, {545, 645}}, {pressure = 0.95})
b_f20:stroke({{450, 680}, {565, 680}}, {pressure = 0.95})

b_f20:stroke({{705, 530}, {780, 530}}, {pressure = 0.95})
b_f20:stroke({{715, 565}, {790, 565}}, {pressure = 0.95})
b_f20:stroke({{715, 605}, {790, 605}}, {pressure = 0.95})
b_f20:stroke({{710, 645}, {790, 645}}, {pressure = 0.95})
b_f20:stroke({{690, 680}, {790, 680}}, {pressure = 0.95})

-- Wall above table
local b_f16 = brush("filbert", 16)
b_f16:load(p_wall_dark, 1.0)
b_f16:stroke({{500, 480}, {595, 480}}, {pressure = 0.95})
b_f16:stroke({{500, 510}, {560, 510}}, {pressure = 0.95})
b_f16:stroke({{645, 480}, {740, 480}}, {pressure = 0.95})
b_f16:stroke({{680, 510}, {750, 510}}, {pressure = 0.95})

-- 2. SOLID OPAQUE BLOCK-IN OF QUINCE BODY VIA WORK
p_quince_dense = pile{{"lead white", 4.5}, {"yellow ochre", 5.0}, {"chrome yellow", 4.5}, {"red earth", 0.6}, medium = 0.05}
work(m_quince, {
  hand = "body",
  pile = p_quince_dense,
  angle = 1.4,
  coverage = 3.5,
  fill = true,
  edge = "firm"
})

-- 3. MODEL LIGHT AND SHADE WET-IN-WET
local b_f14 = brush("filbert", 14)
local b_f12 = brush("filbert", 12)
local b_f10 = brush("filbert", 10)

-- Lit left lobe (raking north light)
b_f12:load(p_quince_lit_op, 1.0)
b_f12:stroke({{612, 486}, {585, 518}, {564, 562}, {558, 608}, {574, 650}, {602, 672}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.9, 0.75}, swell = {0.85, 1.25, 0.85}
})
b_f12:load(p_quince_lit_op, 1.0)
b_f12:stroke({{622, 498}, {602, 532}, {585, 578}, {582, 622}, {602, 660}}, {
  pressure = {0.8, 0.95, 0.95, 0.85, 0.7}
})

-- Broad warm crest of light on central lobe
b_f14:load(p_quince_dense, 0.95)
b_f14:stroke({{632, 495}, {624, 540}, {616, 590}, {622, 642}}, {pressure = {0.7, 0.92, 0.92, 0.7}})

-- Deep core shadow along right turning flank
b_f14:load(p_quince_core_op, 1.0)
b_f14:stroke({{642, 488}, {668, 520}, {690, 568}, {696, 616}, {680, 658}, {648, 674}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.9, 0.75}
})
b_f10:load(p_quince_deep, 0.95)
b_f10:stroke({{668, 525}, {692, 572}, {698, 616}, {684, 656}}, {
  pressure = {0.7, 0.95, 0.95, 0.7}
})

-- Warm table bounce into lower-right contour
b_f10:load(p_quince_refl_op, 0.9)
b_f10:stroke({{675, 515}, {698, 555}, {708, 600}, {702, 638}, {684, 666}}, {
  pressure = {0.4, 0.8, 0.8, 0.4}
})

-- Soft badger blend across knobby volume
blend(m_quince, {angle = 1.35, coverage = 0.9})

-- 4. TACTILE SURFACE DETAILS
-- Russet orchard blush scumble across sunlit curve
local b_stip = brush("stippler", 5)
b_stip:load(p_quince_blush, 0.5)
b_stip:stroke({{575, 555}, {592, 595}, {612, 635}}, {pressure = 0.35, shake = 0.3})

-- Buttery impasto highlight on sunlit crest
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{572, 572}, {566, 598}, {570, 622}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(567, 596, {pressure = 1.0})

-- Woody stem arching from crown socket
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_quince_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_quince_stem, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.85, 0.5}})
b_r1:load(p_quince_lit_op, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})
b_r1:load(p_quince_deep, 0.85)
b_r1:stroke({{632, 484}, {638, 488}, {644, 492}}, {pressure = 0.65})

-- Calyx eye at base
b_r1:load(p_quince_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})
b_r1:stroke({{651, 673}, {657, 675}}, {pressure = 0.8})

-- Firm dark contact shadow under base
b_r3:load(p_quince_deep, 1.0)
b_r3:stroke({{576, 675}, {615, 677}, {660, 676}, {688, 672}}, {pressure = {0.8, 1.0, 0.9, 0.7}})

print("Quince solidly established with opaque body and sculptural volume.")

--@ chunk 107
-- 1. SMOOTH THE TABLETOP HORIZONTALLY AROUND THE QUINCE
-- Sweep p_stone_lit horizontally to eliminate all isolated dabs
local b_f20 = brush("filbert", 20)
local b_f24 = brush("filbert", 24)

-- Table between pitcher and quince (x = 425 to 555, y = 530 to 670)
for y = 530, 670, 10 do
  local p = (y < 560) and p_stone_mid or p_stone_lit
  b_f20:load(p, 1.0)
  b_f20:stroke({{425, y}, {490, y}, {555, y}}, {pressure = 0.95})
end

-- Table to the right of quince (x = 705 to 1010, y = 530 to 670)
for y = 530, 670, 10 do
  local p = (y < 560) and p_stone_mid or p_stone_lit
  b_f24:load(p, 1.0)
  b_f24:stroke({{705, y}, {850, y}, {1010, y}}, {pressure = 0.95})
end

-- Clean background wall around quince crown (y = 460 to 522)
local b_f14 = brush("filbert", 14)
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{460, 480}, {540, 480}, {615, 480}}, {pressure = 0.95})
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{460, 510}, {540, 510}, {570, 510}}, {pressure = 0.95})
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{640, 480}, {720, 480}, {800, 480}}, {pressure = 0.95})
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{690, 510}, {760, 510}, {840, 510}}, {pressure = 0.95})

-- Soft horizontal fusion with badger
local b_badger = brush("badger", 28)
b_badger:stroke({{425, 545}, {550, 545}}, {pressure = 0.22})
b_badger:stroke({{425, 610}, {550, 610}}, {pressure = 0.25})
b_badger:stroke({{705, 545}, {1000, 545}}, {pressure = 0.22})
b_badger:stroke({{705, 610}, {1000, 610}}, {pressure = 0.25})

-- 2. REFINE THE QUINCE: ELIMINATE SCRATCH, ENRICH LIGHT & SHADOW
local b_f12 = brush("filbert", 12)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Cover the red diagonal line on lower-left with dense golden body paint
b_f10:load(p_quince_dense, 1.0)
b_f10:stroke({{575, 540}, {585, 580}, {595, 625}, {605, 660}}, {pressure = 0.95})
b_f10:stroke({{585, 550}, {598, 595}, {610, 640}}, {pressure = 0.95})

-- Luminous sunlit crest on the left lobe
b_f12:load(p_quince_lit_op, 1.0)
b_f12:stroke({{610, 485}, {585, 520}, {565, 565}, {560, 610}, {575, 650}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.8}, swell = {0.85, 1.25, 0.85}
})
b_f10:load(p_quince_lit_op, 1.0)
b_f10:stroke({{595, 530}, {578, 575}, {575, 615}, {590, 650}}, {
  pressure = {0.8, 0.95, 0.95, 0.8}
})

-- Deep core shadow on right flank
b_f12:load(p_quince_core_op, 1.0)
b_f12:stroke({{642, 488}, {668, 522}, {688, 570}, {694, 618}, {678, 658}, {648, 674}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.9, 0.75}
})
b_f10:load(p_quince_deep, 0.95)
b_f10:stroke({{670, 530}, {692, 575}, {696, 618}, {682, 656}}, {
  pressure = {0.7, 0.95, 0.95, 0.7}
})

-- Warm table bounce on lower-right contour
b_f10:load(p_quince_refl_op, 0.9)
b_f10:stroke({{676, 520}, {698, 560}, {706, 602}, {700, 640}, {682, 668}}, {
  pressure = {0.4, 0.75, 0.75, 0.4}
})

-- Gentle badger blend across the knobby surface
local b_badger_sm = brush("badger", 14)
b_badger_sm:stroke({{575, 550}, {630, 550}, {685, 550}}, {pressure = 0.2})
b_badger_sm:stroke({{570, 605}, {630, 605}, {695, 605}}, {pressure = 0.2})
b_badger_sm:stroke({{585, 650}, {635, 650}, {680, 650}}, {pressure = 0.2})

-- Russet blush scumble
local b_stip = brush("stippler", 5)
b_stip:load(p_quince_blush, 0.5)
b_stip:stroke({{576, 555}, {592, 595}, {612, 635}}, {pressure = 0.35, shake = 0.3})

-- Buttery impasto highlight on the knobby crest
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{572, 572}, {566, 598}, {570, 622}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(567, 596, {pressure = 1.0})

-- Arching woody stalk
b_r2:load(p_quince_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_quince_stem, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.85, 0.5}})
b_r1:load(p_quince_lit_op, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})

-- Calyx eye at base
b_r1:load(p_quince_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})

-- Firm dark contact shadow under base
b_r3:load(p_quince_deep, 1.0)
b_r3:stroke({{576, 675}, {615, 677}, {660, 676}, {688, 672}}, {pressure = {0.8, 1.0, 0.9, 0.7}})

print("Quince and surrounding table perfectly harmonized.")

--@ chunk 108
-- Piles for Walnuts, Knife, and Shadows
p_walnut_shell  = pile{{"raw umber", 5.5}, {"yellow ochre", 3.5}, {"red earth", 2.0}, {"bone black", 0.8}, medium = 0.10}
p_walnut_rim    = pile{{"lead white", 5.5}, {"yellow ochre", 3.5}, {"raw umber", 1.5}, medium = 0.08}
p_walnut_kernel = pile{{"lead white", 6.5}, {"yellow ochre", 3.5}, {"raw umber", 1.0}, medium = 0.06}
p_walnut_deep   = pile{{"raw umber", 6.0}, {"bone black", 3.5}, {"red earth", 1.5}, medium = 0.10}

p_steel_lit   = pile{{"lead white", 8.0}, {"cobalt blue", 1.0}, {"raw umber", 0.8}, {"bone black", 0.5}, medium = 0.05}
p_steel_mid   = pile{{"lead white", 4.5}, {"cobalt blue", 2.0}, {"raw umber", 2.5}, {"bone black", 1.5}, medium = 0.10}
p_steel_dark  = pile{{"bone black", 4.5}, {"raw umber", 4.0}, {"cobalt blue", 1.5}, medium = 0.10}
p_brass       = pile{{"yellow ochre", 6.0}, {"red earth", 1.5}, {"lead white", 2.5}, medium = 0.08}
p_brass_hi    = pile{{"lead white", 8.0}, {"yellow ochre", 2.0}, medium = 0.04}
p_handle_lit  = pile{{"red earth", 4.0}, {"raw umber", 4.5}, {"yellow ochre", 2.0}, {"bone black", 1.5}, medium = 0.08}
p_handle_dark = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.0}, medium = 0.10}

p_cast_shadow = pile{{"raw umber", 4.5}, {"bone black", 2.0}, {"yellow ochre", 2.2}, {"lead white", 1.2}, medium = 0.25}

-- 1. SOFT BADGER BLEND ON UPPER TABLE TO MELT STROKE ENDS
local b_badger = brush("badger", 30)
b_badger:stroke({{410, 560}, {560, 560}}, {pressure = 0.2})
b_badger:stroke({{410, 600}, {560, 600}}, {pressure = 0.2})
b_badger:stroke({{690, 560}, {850, 560}}, {pressure = 0.2})
b_badger:stroke({{690, 600}, {850, 600}}, {pressure = 0.2})

-- 2. CAST SHADOWS (Transparent raking shadows from upper-left)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)

-- Pitcher cast shadow onto limestone table (x = 422 to 540, y = 615 to 642)
b_f14:load(p_cast_shadow, 0.75)
b_f14:stroke({{422, 617}, {470, 622}, {525, 632}}, {pressure = {0.8, 0.7, 0.3}, swell = {1.0, 1.3, 0.7}})
b_f10:load(p_cast_shadow, 0.7)
b_f10:stroke({{424, 624}, {465, 628}, {510, 636}}, {pressure = {0.7, 0.5, 0.2}})

-- Quince cast shadow onto limestone table (x = 660 to 765, y = 674 to 704)
b_f14:load(p_cast_shadow, 0.8)
b_f14:stroke({{660, 676}, {705, 682}, {755, 694}}, {pressure = {0.85, 0.7, 0.3}, swell = {1.0, 1.4, 0.7}})
b_f10:load(p_cast_shadow, 0.7)
b_f10:stroke({{670, 684}, {715, 690}, {750, 698}}, {pressure = {0.7, 0.5, 0.2}})

-- Soften shadow edges into limestone
b_badger:stroke({{440, 625}, {530, 635}}, {pressure = 0.18})
b_badger:stroke({{680, 685}, {760, 695}}, {pressure = 0.18})

-- Re-state deep contact shadows directly under bases
local b_r3 = brush{kind = "round", width = 3.0, point = 0.9}
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.8, 1.0, 0.8}})
b_r3:load(p_quince_deep, 1.0)
b_r3:stroke({{576, 675}, {615, 677}, {660, 676}, {688, 672}}, {pressure = {0.8, 1.0, 0.9, 0.7}})

-- 3. THE WALNUTS
local b_r3_pt = brush{kind = "round", width = 3.0, point = 0.95}
local b_r2    = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1    = brush{kind = "round", width = 1.2, point = 0.98}

-- A. Cracked Walnut Half (center around x = 482, y = 654)
-- Outer woody shell bowl
b_r3_pt:load(p_walnut_shell, 1.0)
b_r3_pt:stroke({{468, 654}, {476, 664}, {492, 665}, {500, 655}, {496, 646}, {480, 645}, {468, 654}}, {pressure = 0.95})

-- Dark interior hollow & septum
b_r2:load(p_walnut_deep, 1.0)
b_r2:stroke({{472, 653}, {484, 655}, {495, 653}}, {pressure = 0.9})
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{483, 647}, {484, 662}}, {pressure = 0.85})

-- Sculpted ivory kernel lobes (impasto dabs)
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{474, 650}, {478, 647}, {482, 651}}, {pressure = 0.95})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{473, 656}, {477, 659}, {482, 656}}, {pressure = 0.95})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{486, 650}, {490, 648}, {494, 652}}, {pressure = 0.95})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{485, 657}, {490, 660}, {494, 656}}, {pressure = 0.95})

-- Crisp highlight glints on ivory kernel convolutions
b_r1:load(p_walnut_rim, 1.0)
b_r1:touch(476, 648, {pressure = 1.0})
b_r1:touch(475, 658, {pressure = 1.0})
b_r1:touch(488, 649, {pressure = 0.95})

-- Sharp lit edge on cracked shell rim
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{468, 652}, {472, 646}, {482, 644}}, {pressure = 0.95})

-- Walnut half cast shadow
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{498, 655}, {512, 659}, {524, 662}}, {pressure = {0.8, 0.5, 0.2}})
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{476, 665}, {492, 666}}, {pressure = 0.9})

-- B. Intact Whole Walnut (center around x = 528, y = 662)
-- Solid ovoid body
b_r3_pt:load(p_walnut_shell, 1.0)
b_r3_pt:stroke({{514, 662}, {522, 653}, {536, 654}, {544, 664}, {538, 673}, {524, 672}, {514, 662}}, {pressure = 0.98})
b_r3_pt:load(p_walnut_shell, 1.0)
b_r3_pt:touch(528, 662, {pressure = 0.95})

-- Raking light on upper-left crest & corrugations
b_r2:load(p_walnut_rim, 1.0)
b_r2:stroke({{518, 658}, {526, 655}, {534, 657}}, {pressure = 0.9})
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{520, 662}, {528, 660}, {536, 662}}, {pressure = 0.85})

-- Suture ridge running equatorially
b_r1:load(p_walnut_deep, 0.9)
b_r1:stroke({{515, 663}, {528, 663}, {543, 664}}, {pressure = 0.8})
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{516, 662}, {528, 662}, {542, 663}}, {pressure = 0.9})

-- Shadow on lower-right flank
b_r2:load(p_walnut_deep, 0.95)
b_r2:stroke({{530, 668}, {540, 670}, {543, 665}}, {pressure = 0.85})

-- Whole walnut cast shadow
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{540, 666}, {554, 670}, {568, 673}}, {pressure = {0.8, 0.5, 0.2}})
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{524, 673}, {538, 674}}, {pressure = 0.9})

-- 4. THE OLD BODEGÓN KNIFE
-- A. Blade lying flat on limestone (tip at {675, 686}, crossing ledge at {786, 715})
-- Thin cast shadow under blade
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{678, 689}, {730, 703}, {784, 717}}, {pressure = {0.5, 0.8, 0.9}})

-- Steel blade flat
local b_f8_knife = brush("filbert", 6)
b_f8_knife:load(p_steel_mid, 1.0)
b_f8_knife:stroke({{675, 686}, {730, 700}, {785, 714}}, {pressure = 0.95})

-- Spine of blade (dark steel)
b_r1:load(p_steel_dark, 1.0)
b_r1:stroke({{675, 686}, {730, 698}, {785, 711}}, {pressure = 0.9})

-- Beveled cutting edge facing forward (crisp silvery-white specular light)
b_r1:load(p_steel_lit, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {785, 715}}, {pressure = 0.98})

-- B. Brass Bolster / Ferrule at table edge ({785, 713} to {798, 718})
b_r2:load(p_brass, 1.0)
b_r2:stroke({{785, 713}, {798, 718}}, {pressure = 0.95})
b_r1:load(p_brass_hi, 1.0)
b_r1:touch(790, 714, {pressure = 1.0})
b_r1:load(p_steel_dark, 1.0)
b_r1:stroke({{785, 716}, {798, 720}}, {pressure = 0.85})

-- C. Turned Wood Handle projecting over front ledge ({798, 718} to {865, 746})
-- Handle dark cylinder
local b_r5_h = brush{kind = "round", width = 5.2, point = 0.9}
b_r5_h:load(p_handle_dark, 1.0)
b_r5_h:stroke({{798, 718}, {832, 732}, {865, 746}}, {pressure = 0.98, swell = {0.9, 1.15, 0.95}})

-- Upper lit surface of turned handle
local b_r3_h = brush{kind = "round", width = 2.8, point = 0.92}
b_r3_h:load(p_handle_lit, 1.0)
b_r3_h:stroke({{799, 716}, {832, 730}, {864, 744}}, {pressure = 0.9})

-- Turned ornamental rings / moldings on handle
b_r1:load(p_brass, 0.9)
b_r1:stroke({{812, 720}, {810, 726}}, {pressure = 0.85})
b_r1:stroke({{835, 729}, {833, 736}}, {pressure = 0.85})
-- Highlight on pommel end
b_r1:load(p_handle_lit, 1.0)
b_r1:touch(865, 746, {pressure = 0.95})

-- D. Dramatic Vertical Cast Shadow down Ledge Face
-- The handle casts its shadow straight down the vertical face of the stone ledge
local b_f12_sh = brush("filbert", 10)
b_f12_sh:load(p_ledge_face, 0.6)
b_f12_sh:stroke({{806, 718}, {810, 740}, {815, 765}}, {pressure = {0.9, 0.7, 0.3}})
local b_r3_sh = brush{kind = "round", width = 3.0, point = 0.9}
b_r3_sh:load(p_walnut_deep, 0.85)
b_r3_sh:stroke({{802, 716}, {805, 735}, {808, 755}}, {pressure = {0.85, 0.6, 0.2}})

-- 5. CRISP RAZOR CHAMFER HIGHLIGHT (y = 715)
-- Left of knife (-5 to 784)
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {784, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.04})
-- Right of knife (799 to 1005)
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{799, 715}, {900, 715}, {1005, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.04})

print("Walnuts, bodegón knife, and unified cast shadows painted.")

--@ chunk 109
-- 1. HARMONIZE TABLE SURFACE BETWEEN PITCHER AND QUINCE
-- Sweep horizontal strokes across the table to melt the vertical step at x = 425
local b_f20 = brush("filbert", 20)
local b_badger = brush("badger", 32)

-- Sweep table tone horizontally from behind the pitcher's right flank across to the quince
for y = 525, 620, 10 do
  local p = (y < 560) and p_stone_mid or p_stone_lit
  b_f20:load(p, 0.9)
  b_f20:stroke({{410, y}, {480, y}, {550, y}}, {pressure = {0.4, 0.85, 0.4}})
end

-- Horizontal sweeps with badger blender to fuse the entire mid-ground table
b_badger:stroke({{390, 540}, {560, 540}}, {pressure = 0.3})
b_badger:stroke({{390, 580}, {560, 580}}, {pressure = 0.3})
b_badger:stroke({{390, 615}, {560, 615}}, {pressure = 0.3})

-- 2. PITCHER REFINEMENT & MODELING
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Smooth left belly contour below handle (y = 485 to 550)
b_f10:load(p_jug_lit, 1.0)
b_f10:stroke({{266, 480}, {264, 510}, {274, 545}, {295, 580}}, {pressure = 0.95})
b_f10:load(p_jug_body, 1.0)
b_f10:stroke({{274, 485}, {276, 520}, {288, 555}, {310, 590}}, {pressure = 0.92})

-- Clean wall outside the left belly
local b_f8_w = brush("filbert", 8)
b_f8_w:load(p_wall_dark, 1.0)
b_f8_w:stroke({{250, 480}, {255, 510}, {260, 535}}, {pressure = 0.9})

-- Clean negative space inside handle loop
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke({{336, 328}, {308, 346}, {278, 376}, {264, 412}, {268, 442}, {280, 432}, {298, 396}, {322, 358}, {336, 332}}, {pressure = 0.95})

-- Model illuminated flank on the left (raking north light)
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{345, 274}, {348, 320}, {330, 380}, {295, 440}, {286, 490}, {298, 545}, {328, 595}, {345, 614}}, {
  pressure = {0.85, 0.98, 0.95, 0.9, 0.75}
})
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{358, 276}, {360, 325}, {348, 390}, {322, 450}, {314, 500}, {328, 555}, {352, 610}}, {
  pressure = {0.8, 0.95, 0.95, 0.85, 0.7}
})

-- Broad warm crest of light on belly
b_f14:load(p_jug_lit, 0.95)
b_f14:stroke({{340, 410}, {330, 465}, {335, 520}, {355, 570}}, {pressure = {0.7, 0.95, 0.85, 0.65}})

-- Unify right flank: eliminate the 3 dents with continuous curving core shadow
b_f12 = brush("filbert", 12)
b_f12:load(p_jug_core, 1.0)
b_f12:stroke({{396, 274}, {400, 325}, {420, 385}, {450, 445}, {455, 495}, {440, 550}, {412, 605}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.9, 0.75}
})
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{420, 380}, {448, 430}, {460, 480}, {455, 530}, {438, 575}, {416, 610}}, {
  pressure = {0.7, 0.95, 0.95, 0.7}
})

-- Warm table bounce along right contour
b_f8 = brush("filbert", 8)
b_f8:load(p_jug_reflect, 0.95)
b_f8:stroke({{406, 325}, {432, 380}, {462, 435}, {470, 485}, {458, 535}, {438, 580}, {420, 612}}, {
  pressure = {0.4, 0.8, 0.8, 0.4}
})

-- Carve wall along true right contour of pitcher to make edge crisp and noble
local pts_wall_pitcher_r = {
  {406, 268}, {403, 310}, {408, 335}, {426, 368}, {450, 410}, {468, 450}, {474, 485}, {470, 520}
}
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke(pts_wall_pitcher_r, {pressure = 0.95})

-- Table along lower-right contour of pitcher
local pts_table_pitcher_r = {
  {470, 520}, {460, 550}, {446, 580}, {432, 600}, {422, 615}
}
b_r2:load(p_stone_mid, 1.0)
b_r2:stroke(pts_table_pitcher_r, {pressure = 0.95})

-- Internal blend across pitcher belly
local b_badger_jug = brush("badger", 20)
b_badger_jug:stroke({{320, 470}, {385, 470}, {445, 470}}, {pressure = 0.22})
b_badger_jug:stroke({{325, 530}, {385, 530}, {440, 530}}, {pressure = 0.22})
b_badger_jug:stroke({{335, 580}, {385, 580}, {425, 580}}, {pressure = 0.22})

-- Restate strap handle
local b_r5 = brush{kind = "round", width = 5.5, point = 0.88}
local pts_handle_spine = {
  {342, 316}, {310, 328}, {270, 355}, {242, 392}, {238, 426}, {248, 460}, {270, 486}
}
b_r5:load(p_jug_body, 1.0)
b_r5:stroke(pts_handle_spine, {pressure = 0.95, swell = {0.85, 1.2, 0.9}})

-- Lit face on handle
b_r3:load(p_jug_lit, 1.0)
b_r3:stroke({{335, 318}, {306, 330}, {268, 358}, {242, 395}, {239, 424}}, {pressure = 0.9})
b_r2:load(p_jug_hi, 1.0)
b_r2:stroke({{295, 336}, {262, 364}, {242, 396}, {239, 422}}, {pressure = {0.5, 0.95, 0.5}})

-- Glaze gleam on handle crest
b_r1:load(p_jug_glaze, 1.0)
b_r1:stroke({{242, 400}, {240, 415}, {242, 426}}, {pressure = {0.4, 0.98, 0.4}})

-- Deep shaded underside of handle
local pts_handle_under = {
  {334, 326}, {300, 340}, {260, 368}, {236, 404}, {234, 432}, {244, 464}, {264, 488}
}
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke(pts_handle_under, {pressure = 0.9})

-- Mouth & flared lip
local b_f8_mouth = brush("filbert", 8)
b_f8_mouth:load(p_jug_deep, 1.0)
b_f8_mouth:stroke({{344, 269}, {370, 270}, {396, 269}}, {pressure = 0.95})
b_r2:load(p_jug_lit, 1.0)
b_r2:stroke({{335, 268}, {355, 273}, {372, 274}}, {pressure = 0.9})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(348, 272, {pressure = 1.0})

-- Footring and occlusion shadow
b_r3:load(p_jug_body, 1.0)
b_r3:stroke({{325, 615}, {372, 617}, {422, 615}}, {pressure = 0.95})
b_r2:load(p_jug_lit, 0.9)
b_r2:stroke({{325, 615}, {355, 616}}, {pressure = 0.85})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.8, 1.0, 0.8}})

-- Juicy glaze impasto on shoulder curve
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r2:load(p_jug_glaze, 1.0)
b_r2:stroke({{324, 402}, {321, 418}, {319, 430}}, {pressure = {0.5, 0.98, 0.4}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})

-- Pitcher cast shadow onto table
b_f14:load(p_cast_shadow, 0.75)
b_f14:stroke({{422, 617}, {470, 622}, {525, 632}}, {pressure = {0.8, 0.7, 0.3}, swell = {1.0, 1.3, 0.7}})
b_f10:load(p_cast_shadow, 0.7)
b_f10:stroke({{424, 624}, {465, 628}, {510, 636}}, {pressure = {0.7, 0.5, 0.2}})

print("Pitcher modeled seamlessly, table step dissolved, cast shadow laid.")

--@ chunk 110
-- 1. RESTORE PITCHER'S SWELLING RIGHT FLANK IN SOLID OPAQUE TERRACOTTA
-- True right contour: y=440: x=465; y=485: x=472; y=520: x=468; y=550: x=456; y=580: x=440; y=615: x=422
local b_f14 = brush("filbert", 14)
local b_f12 = brush("filbert", 12)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}

-- Dense terracotta fill on right belly to bury all grey marks
for y = 430, 614, 8 do
  local xl = 360
  local xr
  if y < 485 then
    xr = 465 + (472 - 465) * (y - 430) / 55
  elseif y < 550 then
    xr = 472 - (472 - 456) * (y - 485) / 65
  else
    xr = 456 - (456 - 422) * (y - 550) / 64
  end
  b_f14:load(p_jug_body, 1.0)
  b_f14:stroke({{xl, y}, {(xl + xr) * 0.5, y}, {xr, y}}, {pressure = 0.98})
end

-- Core shadow down the right flank turning into depth
b_f12:load(p_jug_core, 1.0)
b_f12:stroke({{400, 340}, {425, 395}, {452, 450}, {456, 500}, {442, 550}, {415, 605}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.9, 0.75}
})
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{430, 400}, {454, 450}, {460, 495}, {448, 545}, {426, 595}, {416, 612}}, {
  pressure = {0.7, 0.95, 0.95, 0.7}
})

-- Warm reflected light from table along right edge
local b_f8 = brush("filbert", 8)
b_f8:load(p_jug_reflect, 0.95)
b_f8:stroke({{410, 335}, {435, 390}, {464, 445}, {470, 485}, {460, 535}, {442, 580}, {422, 614}}, {
  pressure = {0.35, 0.75, 0.8, 0.4}
})

-- Soft badger blend across belly to unite terracotta volume
local b_badger = brush("badger", 18)
b_badger:stroke({{340, 480}, {400, 480}, {455, 480}}, {pressure = 0.22})
b_badger:stroke({{340, 540}, {400, 540}, {445, 540}}, {pressure = 0.22})
b_badger:stroke({{350, 585}, {400, 585}, {430, 585}}, {pressure = 0.22})

-- Clean wall behind right shoulder
local pts_wall_pitcher_r = {
  {406, 268}, {403, 310}, {408, 335}, {426, 368}, {450, 410}, {468, 450}, {474, 485}, {470, 520}
}
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke(pts_wall_pitcher_r, {pressure = 0.95})

-- 2. SOLID TABLETOP BETWEEN PITCHER AND QUINCE (y = 520 to 615)
-- Exactly between pitcher contour and quince contour:
-- Pitcher right: y=520: 468; y=550: 456; y=580: 440; y=615: 422
-- Quince left: x ~ 550
for y = 522, 615, 8 do
  local xl = 468 - (y - 522) * (468 - 422) / (615 - 522) + 2
  local xr = 548
  local p = (y < 555) and p_stone_mid or p_stone_lit
  b_f12:load(p, 1.0)
  b_f12:stroke({{xl, y}, {(xl + xr) * 0.5, y}, {xr, y}}, {pressure = 0.95})
end

-- Horizontal badger blend strictly inside the gap between vessels
b_badger:stroke({{460, 540}, {545, 540}}, {pressure = 0.22})
b_badger:stroke({{450, 575}, {545, 575}}, {pressure = 0.22})
b_badger:stroke({{435, 610}, {545, 610}}, {pressure = 0.22})

-- Re-anchor footring and contact shadow
b_r3:load(p_jug_body, 1.0)
b_r3:stroke({{325, 615}, {372, 617}, {422, 615}}, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.8, 1.0, 0.8}})

-- Shoulder glaze catchlight
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.3, 0.7}})
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})

-- 3. CARVE QUINCE OUTER CONTOURS TO ELIMINATE SERRATIONS
-- Wall outside quince crown and upper flanks
local pts_wall_quince = {
  {540, 520}, {555, 508}, {578, 490}, {605, 478}, {628, 476},
  {646, 480}, {670, 500}, {692, 520}
}
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke(pts_wall_quince, {pressure = 0.95})

-- Table outside quince right flank (y = 520 to 675)
local pts_table_quince_r = {
  {692, 520}, {708, 555}, {714, 600}, {706, 640}, {686, 668}, {660, 676}
}
b_r2:load(p_stone_mid, 1.0)
b_r2:stroke(pts_table_quince_r, {pressure = 0.95})

-- Sweep table to the right of quince to keep limestone smooth
local b_f20 = brush("filbert", 20)
for y = 522, 670, 10 do
  local xl
  if y < 600 then
    xl = 712 + 2
  else
    xl = 714 - (y - 600) * (714 - 665) / 70 + 2
  end
  local p = (y < 555) and p_stone_mid or p_stone_lit
  b_f20:load(p, 1.0)
  b_f20:stroke({{xl, y}, {850, y}, {1010, y}}, {pressure = 0.95})
end

-- Horizontal badger blend on right table
b_badger:stroke({{720, 545}, {1000, 545}}, {pressure = 0.22})
b_badger:stroke({{720, 600}, {1000, 600}}, {pressure = 0.22})
b_badger:stroke({{700, 650}, {1000, 650}}, {pressure = 0.22})

-- 4. SCULPT & FINISH THE QUINCE
-- Rich golden body along the knobby lobes
b_f12:load(p_quince_dense, 1.0)
b_f12:stroke({{615, 480}, {588, 515}, {566, 560}, {558, 608}, {574, 650}, {605, 674}}, {pressure = 0.98})
b_f12:load(p_quince_dense, 1.0)
b_f12:stroke({{625, 482}, {610, 525}, {595, 575}, {592, 628}, {618, 672}}, {pressure = 0.98})

-- Luminous sunlit crest on left lobe
b_f12:load(p_quince_lit_op, 1.0)
b_f12:stroke({{610, 485}, {585, 520}, {564, 565}, {560, 610}, {576, 652}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.8}, swell = {0.85, 1.25, 0.85}
})
b_f10:load(p_quince_lit_op, 1.0)
b_f10:stroke({{595, 530}, {578, 578}, {575, 618}, {590, 652}}, {pressure = {0.8, 0.95, 0.95, 0.8}})

-- Deep core shadow on right flank
b_f12:load(p_quince_core_op, 1.0)
b_f12:stroke({{638, 486}, {665, 518}, {688, 565}, {696, 615}, {680, 658}, {648, 674}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.9, 0.75}
})
b_f10:load(p_quince_deep, 0.95)
b_f10:stroke({{668, 525}, {690, 570}, {696, 615}, {682, 656}}, {pressure = {0.7, 0.95, 0.95, 0.7}})

-- Warm table bounce on lower right contour
b_f8:load(p_quince_refl_op, 0.9)
b_f8:stroke({{675, 515}, {698, 555}, {706, 600}, {700, 638}, {682, 666}}, {pressure = {0.4, 0.75, 0.75, 0.4}})

-- Soft badger blend across knobby planes
b_badger:stroke({{575, 545}, {630, 545}, {685, 545}}, {pressure = 0.2})
b_badger:stroke({{570, 600}, {630, 600}, {695, 600}}, {pressure = 0.2})
b_badger:stroke({{585, 650}, {635, 650}, {680, 650}}, {pressure = 0.2})

-- Orchard blush scumble
local b_stip = brush("stippler", 5)
b_stip:load(p_quince_blush, 0.5)
b_stip:stroke({{576, 555}, {592, 595}, {612, 635}}, {pressure = 0.35, shake = 0.3})

-- Buttery impasto highlight on the knobby crest
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{572, 572}, {566, 598}, {570, 622}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(567, 596, {pressure = 1.0})

-- Arching woody stalk
b_r2:load(p_quince_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_quince_stem, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.85, 0.5}})
b_r1:load(p_quince_lit_op, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})

-- Calyx eye at base
b_r1:load(p_quince_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})

-- Firm dark contact shadow under base
b_r3:load(p_quince_deep, 1.0)
b_r3:stroke({{576, 675}, {615, 677}, {660, 676}, {688, 672}}, {pressure = {0.8, 1.0, 0.9, 0.7}})

print("Pitcher and quince contours perfected and modeled.")

--@ chunk 111
-- Check what piles are valid right now
local test_piles = {
  p_stone_lit = p_stone_lit,
  p_stone_mid = p_stone_mid,
  p_wall_dark = p_wall_dark,
  p_jug_body = p_jug_body,
  p_jug_core = p_jug_core,
  p_jug_deep = p_jug_deep,
  p_jug_lit = p_jug_lit,
  p_jug_reflect = p_jug_reflect,
  p_jug_hi = p_jug_hi,
  p_jug_glaze = p_jug_glaze,
  p_quince_dense = p_quince_dense,
  p_quince_stem = p_quince_stem,
  p_steel_mid = p_steel_mid,
  p_cast_shadow = p_cast_shadow
}
for name, p in pairs(test_piles) do
  print(name, p)
end

--@ chunk 112
-- Fresh piles for Quince Modeling
p_quince_lit   = pile{{"lead white", 6.5}, {"chrome yellow", 4.0}, {"yellow ochre", 2.0}, medium = 0.05}
p_quince_core  = pile{{"raw umber", 5.5}, {"yellow ochre", 3.0}, {"red earth", 2.2}, {"bone black", 1.0}, medium = 0.08}
p_quince_crest = pile{{"lead white", 8.5}, {"chrome yellow", 2.0}, {"yellow ochre", 0.6}, medium = 0.03}
p_quince_blush = pile{{"yellow ochre", 4.0}, {"red earth", 2.5}, {"vermilion", 1.0}, {"raw umber", 0.8}, medium = 0.12}

-- 1. OVERLAP TABLE UNDER QUINCE EDGES TO ELIMINATE SEAMS
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_badger = brush("badger", 24)

-- Table left of quince sweeps from behind pitcher right flank (x = 440) INTO quince left flank (x = 565)
for y = 525, 620, 10 do
  local p = (y < 560) and p_stone_mid or p_stone_lit
  b_f14:load(p, 0.95)
  b_f14:stroke({{440, y}, {500, y}, {565, y}}, {pressure = 0.95})
end

-- Table right of quince sweeps from inside quince shadow (x = 695) all the way to canvas edge (x = 1010)
for y = 525, 670, 10 do
  local p = (y < 560) and p_stone_mid or p_stone_lit
  b_f18:load(p, 0.95)
  b_f18:stroke({{695, y}, {850, y}, {1010, y}}, {pressure = 0.95})
end

-- Horizontal badger sweeps to fuse the stone surface
b_badger:stroke({{430, 540}, {560, 540}}, {pressure = 0.22})
b_badger:stroke({{430, 580}, {560, 580}}, {pressure = 0.22})
b_badger:stroke({{430, 615}, {560, 615}}, {pressure = 0.22})
b_badger:stroke({{700, 545}, {1000, 545}}, {pressure = 0.22})
b_badger:stroke({{700, 600}, {1000, 600}}, {pressure = 0.22})
b_badger:stroke({{700, 650}, {1000, 650}}, {pressure = 0.22})

-- 2. SOLID SCULPTURAL QUINCE MODELING OVER THE TABLE
local b_f12 = brush("filbert", 12)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Golden body coat establishing full organic volume
b_f12:load(p_quince_dense, 1.0)
b_f12:stroke({{618, 480}, {588, 515}, {564, 560}, {554, 608}, {570, 650}, {602, 674}}, {pressure = 0.98})
b_f12:load(p_quince_dense, 1.0)
b_f12:stroke({{625, 482}, {610, 525}, {595, 575}, {590, 628}, {615, 672}}, {pressure = 0.98})
b_f12:load(p_quince_dense, 1.0)
b_f12:stroke({{632, 484}, {635, 530}, {630, 585}, {632, 640}, {642, 674}}, {pressure = 0.95})

-- Luminous illuminated left lobe
b_f12:load(p_quince_lit, 1.0)
b_f12:stroke({{610, 485}, {585, 520}, {564, 565}, {558, 610}, {574, 652}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.8}, swell = {0.85, 1.25, 0.85}
})
b_f10:load(p_quince_lit, 1.0)
b_f10:stroke({{595, 530}, {578, 578}, {575, 618}, {588, 652}}, {pressure = {0.8, 0.95, 0.95, 0.8}})

-- Core shadow down the right flank
b_f12:load(p_quince_core, 1.0)
b_f12:stroke({{638, 486}, {666, 518}, {688, 565}, {696, 615}, {680, 658}, {648, 674}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.9, 0.75}
})
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{668, 525}, {690, 570}, {696, 615}, {682, 656}}, {pressure = {0.7, 0.95, 0.95, 0.7}})

-- Warm table bounce on lower right contour
local b_f8 = brush("filbert", 8)
b_f8:load(p_jug_reflect, 0.95)
b_f8:stroke({{675, 515}, {698, 555}, {706, 600}, {700, 638}, {682, 666}}, {pressure = {0.4, 0.75, 0.75, 0.4}})

-- Gentle badger blend across knobby volume
local b_badger_q = brush("badger", 16)
b_badger_q:stroke({{575, 545}, {630, 545}, {685, 545}}, {pressure = 0.2})
b_badger_q:stroke({{568, 600}, {630, 600}, {695, 600}}, {pressure = 0.2})
b_badger_q:stroke({{585, 650}, {635, 650}, {680, 650}}, {pressure = 0.2})

-- Russet blush scumble
local b_stip = brush("stippler", 5)
b_stip:load(p_quince_blush, 0.5)
b_stip:stroke({{576, 555}, {592, 595}, {612, 635}}, {pressure = 0.35, shake = 0.3})

-- Buttery impasto highlight on the knobby crest
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{572, 572}, {566, 598}, {570, 622}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(567, 596, {pressure = 1.0})

-- Arching woody stalk
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_quince_stem, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.85, 0.5}})
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})

-- Calyx eye at base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})

-- Firm dark contact shadow under base
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{576, 675}, {615, 677}, {660, 676}, {688, 672}}, {pressure = {0.8, 1.0, 0.9, 0.7}})

-- Quince cast shadow flowing to the right
b_f14:load(p_cast_shadow, 0.75)
b_f14:stroke({{660, 676}, {705, 682}, {755, 694}}, {pressure = {0.85, 0.7, 0.3}, swell = {1.0, 1.4, 0.7}})
b_badger:stroke({{680, 685}, {755, 695}}, {pressure = 0.18})

print("Quince and table seamlessly integrated.")

--@ chunk 113
-- Complete Foreground and Finishing Piles (16 piles)
p_walnut_shell  = pile{{"raw umber", 5.0}, {"red earth", 3.0}, {"yellow ochre", 3.0}, {"lead white", 1.8}, medium = 0.06}
p_walnut_kernel = pile{{"lead white", 7.5}, {"yellow ochre", 2.8}, {"raw umber", 0.6}, medium = 0.04}
p_walnut_rim    = pile{{"lead white", 8.0}, {"yellow ochre", 2.0}, medium = 0.04}
p_walnut_deep   = pile{{"bone black", 5.0}, {"raw umber", 4.0}, {"red earth", 1.5}, medium = 0.08}

p_steel_lit     = pile{{"lead white", 8.0}, {"cobalt blue", 1.0}, {"raw umber", 0.8}, {"bone black", 0.5}, medium = 0.05}
p_steel_mid     = pile{{"lead white", 4.5}, {"cobalt blue", 2.0}, {"raw umber", 2.5}, {"bone black", 1.5}, medium = 0.10}
p_steel_dark    = pile{{"bone black", 4.5}, {"raw umber", 4.0}, {"cobalt blue", 1.5}, medium = 0.10}
p_brass         = pile{{"yellow ochre", 6.0}, {"red earth", 1.5}, {"lead white", 2.5}, medium = 0.08}
p_brass_hi      = pile{{"lead white", 8.0}, {"yellow ochre", 2.0}, medium = 0.04}
p_handle_lit    = pile{{"red earth", 4.0}, {"raw umber", 4.5}, {"yellow ochre", 2.0}, {"bone black", 1.5}, medium = 0.08}
p_handle_dark   = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.0}, medium = 0.10}

p_cast_shadow   = pile{{"raw umber", 4.5}, {"bone black", 2.0}, {"yellow ochre", 2.2}, {"lead white", 1.2}, medium = 0.25}
p_quince_gold_op = pile{{"chrome yellow", 6.0}, {"yellow ochre", 4.0}, {"lead white", 2.5}, {"red earth", 0.5}, medium = 0.06}
p_quince_lit_op  = pile{{"lead white", 6.5}, {"chrome yellow", 4.0}, {"yellow ochre", 2.0}, medium = 0.05}
p_quince_crest   = pile{{"lead white", 8.5}, {"chrome yellow", 2.0}, {"yellow ochre", 0.6}, medium = 0.03}
p_ledge_chamfer = pile{{"lead white", 8.5}, {"yellow ochre", 1.2}, {"raw umber", 0.4}, medium = 0.05}

-- 1. UNIFY QUINCE WITH CURVING VERTICAL PASSES
local b_f14 = brush("filbert", 14)
local b_f12 = brush("filbert", 12)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Vertical sweeps down the knobby lobes
b_f12:load(p_quince_gold_op, 1.0)
b_f12:stroke({{618, 480}, {590, 515}, {566, 560}, {556, 608}, {572, 650}, {602, 674}}, {pressure = 0.98})
b_f12:load(p_quince_gold_op, 1.0)
b_f12:stroke({{625, 482}, {610, 525}, {595, 575}, {590, 628}, {615, 672}}, {pressure = 0.98})
b_f12:load(p_quince_gold_op, 1.0)
b_f12:stroke({{632, 484}, {635, 530}, {630, 585}, {632, 640}, {642, 674}}, {pressure = 0.95})

-- Sunlit left lobe
b_f12:load(p_quince_lit_op, 1.0)
b_f12:stroke({{610, 485}, {585, 520}, {564, 565}, {560, 610}, {576, 652}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.8}, swell = {0.85, 1.25, 0.85}
})

-- Core shadow down the right flank
b_f10:load(p_walnut_deep, 0.95)
b_f10:stroke({{642, 488}, {670, 522}, {690, 568}, {696, 616}, {680, 658}, {648, 674}}, {
  pressure = {0.8, 0.95, 0.98, 0.95, 0.85, 0.7}
})

-- Table bounce on right contour
b_f10:load(p_walnut_shell, 0.9)
b_f10:stroke({{675, 515}, {698, 555}, {706, 600}, {700, 638}, {682, 666}}, {pressure = {0.4, 0.75, 0.75, 0.4}})

-- Soft badger blend across knobby volume
local b_badger = brush("badger", 16)
b_badger:stroke({{575, 545}, {630, 545}, {685, 545}}, {pressure = 0.2})
b_badger:stroke({{568, 600}, {630, 600}, {695, 600}}, {pressure = 0.2})
b_badger:stroke({{585, 650}, {635, 650}, {680, 650}}, {pressure = 0.2})

-- Buttery impasto highlight on the knobby crest
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{572, 572}, {566, 598}, {570, 622}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(567, 596, {pressure = 1.0})

-- Arching woody stalk
b_r2:load(p_walnut_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_walnut_shell, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.85, 0.5}})
b_r1:load(p_quince_lit_op, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})

-- Calyx eye at base
b_r1:load(p_walnut_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})

-- Firm dark contact shadow under base
b_r3:load(p_walnut_deep, 1.0)
b_r3:stroke({{576, 675}, {615, 677}, {660, 676}, {688, 672}}, {pressure = {0.8, 1.0, 0.9, 0.7}})

-- Quince cast shadow flowing to the right
b_f14:load(p_cast_shadow, 0.75)
b_f14:stroke({{660, 676}, {705, 682}, {755, 694}}, {pressure = {0.85, 0.7, 0.3}, swell = {1.0, 1.4, 0.7}})

-- 2. PITCHER CAST SHADOW ONTO TABLE
b_f14:load(p_cast_shadow, 0.75)
b_f14:stroke({{422, 617}, {470, 622}, {525, 632}}, {pressure = {0.85, 0.7, 0.3}, swell = {1.0, 1.3, 0.7}})
local b_badger_lg = brush("badger", 24)
b_badger_lg:stroke({{440, 625}, {530, 635}}, {pressure = 0.18})

-- 3. THE WALNUTS IN TACTILE OIL IMPASTO
-- A. Cracked Walnut Half (center at x = 480, y = 654)
-- Solid woody shell cup
local b_r4_w = brush{kind = "round", width = 4.2, point = 0.9}
b_r4_w:load(p_walnut_shell, 1.0)
b_r4_w:stroke({{466, 654}, {474, 665}, {492, 666}, {500, 655}, {495, 645}, {478, 644}, {466, 654}}, {pressure = 0.98})
b_r4_w:load(p_walnut_shell, 1.0)
b_r4_w:touch(483, 655, {pressure = 0.95})

-- Shaded hollow & central septum
b_r2:load(p_walnut_deep, 1.0)
b_r2:stroke({{472, 654}, {483, 655}, {494, 654}}, {pressure = 0.92})
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{483, 646}, {483, 663}}, {pressure = 0.88})

-- Sculpted ivory kernel lobes in thick impasto dabs
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{473, 650}, {478, 647}, {482, 651}}, {pressure = 0.98})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{473, 657}, {477, 660}, {482, 657}}, {pressure = 0.98})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{485, 650}, {490, 648}, {494, 652}}, {pressure = 0.98})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{485, 658}, {490, 661}, {494, 657}}, {pressure = 0.98})

-- Crisp impasto catchlights on ivory convolutions
b_r1:load(p_walnut_rim, 1.0)
b_r1:touch(476, 648, {pressure = 1.0})
b_r1:touch(475, 658, {pressure = 1.0})
b_r1:touch(488, 649, {pressure = 0.98})
b_r1:touch(489, 659, {pressure = 0.98})

-- Crisp lit rim on broken woody shell
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{466, 652}, {472, 645}, {482, 643}}, {pressure = 0.95})

-- Dark contact shadow and cast shadow
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{498, 655}, {512, 659}, {524, 662}}, {pressure = {0.8, 0.5, 0.2}})
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{474, 666}, {492, 667}}, {pressure = 0.95})

-- B. Intact Whole Walnut (center at x = 528, y = 662)
-- Solid ovoid body
b_r4_w:load(p_walnut_shell, 1.0)
b_r4_w:stroke({{514, 662}, {522, 653}, {536, 654}, {544, 664}, {538, 673}, {524, 672}, {514, 662}}, {pressure = 0.98})
b_r4_w:load(p_walnut_shell, 1.0)
b_r4_w:touch(528, 662, {pressure = 0.95})

-- Raking light on upper crest & corrugated texture
b_r2:load(p_walnut_rim, 1.0)
b_r2:stroke({{518, 658}, {526, 655}, {534, 657}}, {pressure = 0.92})
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{520, 662}, {528, 660}, {536, 662}}, {pressure = 0.88})

-- Equatorial suture line
b_r1:load(p_walnut_deep, 0.9)
b_r1:stroke({{515, 663}, {528, 663}, {543, 664}}, {pressure = 0.85})
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{516, 662}, {528, 662}, {542, 663}}, {pressure = 0.92})

-- Shadow on lower flank
b_r2:load(p_walnut_deep, 0.95)
b_r2:stroke({{530, 669}, {540, 671}, {543, 666}}, {pressure = 0.88})

-- Whole walnut cast shadow
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{540, 666}, {554, 670}, {568, 673}}, {pressure = {0.8, 0.5, 0.2}})
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{522, 673}, {538, 674}}, {pressure = 0.95})

-- 4. BODEGÓN KNIFE REFINEMENT
-- Steel blade flat
local b_f8_k = brush("filbert", 6)
b_f8_k:load(p_steel_mid, 1.0)
b_f8_k:stroke({{675, 686}, {730, 700}, {785, 714}}, {pressure = 0.95})

-- Dark steel spine
b_r1:load(p_steel_dark, 1.0)
b_r1:stroke({{675, 686}, {730, 698}, {785, 711}}, {pressure = 0.92})

-- Silvery-white specular cutting edge
b_r1:load(p_steel_lit, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {785, 715}}, {pressure = 1.0})

-- Brass bolster glint
b_r2:load(p_brass, 1.0)
b_r2:stroke({{785, 713}, {798, 718}}, {pressure = 0.95})
b_r1:load(p_brass_hi, 1.0)
b_r1:touch(790, 714, {pressure = 1.0})

-- Turned handle
local b_r5_h = brush{kind = "round", width = 5.2, point = 0.9}
b_r5_h:load(p_handle_dark, 1.0)
b_r5_h:stroke({{798, 718}, {832, 732}, {865, 746}}, {pressure = 0.98, swell = {0.9, 1.15, 0.95}})
local b_r3_h = brush{kind = "round", width = 2.8, point = 0.92}
b_r3_h:load(p_handle_lit, 1.0)
b_r3_h:stroke({{799, 716}, {832, 730}, {864, 744}}, {pressure = 0.9})
b_r1:load(p_handle_lit, 1.0)
b_r1:touch(865, 746, {pressure = 0.98})

-- Vertical handle cast shadow down ledge face
local b_r3_sh = brush{kind = "round", width = 3.2, point = 0.9}
b_r3_sh:load(p_walnut_deep, 0.85)
b_r3_sh:stroke({{802, 716}, {805, 735}, {808, 755}}, {pressure = {0.85, 0.6, 0.2}})

-- 5. RAZOR CHAMFER LINE (y = 715)
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {784, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.04})
b_r1:stroke({{799, 715}, {900, 715}, {1005, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.04})

print("Still life composition fully consolidated and completed.")

--@ chunk 114
-- 1. UNIFY TABLE GAP BETWEEN PITCHER AND QUINCE (y = 522 to 620, x = 455 to 552)
local m_table_mid = rect(455, 522, 97, 98)
work(m_table_mid, {
  hand = "broad",
  pile = p_stone_mid,
  angle = 0,
  coverage = 2.5,
  fill = true,
  edge = "firm"
})
blend(m_table_mid, {angle = 0, coverage = 1.0})

-- 2. RESET QUINCE TO FULL OPAQUE GOLDEN BODY VIA WORK
local pts_quince = {
  {628, 478}, {646, 484}, {670, 508}, {694, 546}, {710, 595},
  {706, 636}, {686, 665}, {662, 676}, {632, 678}, {598, 676},
  {574, 668}, {556, 642}, {546, 608}, {554, 562}, {575, 522},
  {598, 494}, {618, 480}
}
local m_quince = poly(pts_quince, true)

-- Full opaque coat of rich golden body color (wiping all stripes completely)
work(m_quince, {
  hand = "body",
  pile = p_quince_gold_op,
  angle = 1.4,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- Shaded flank on right via soft mask
local m_quince_shade = (m_quince * mask(function(x, y) return x > 635 and 1 or 0 end)):blur(15)
work(m_quince_shade, {
  hand = "body",
  pile = p_walnut_deep,
  angle = 1.4,
  coverage = 1.6
})

-- Sunlit flank on left via soft mask
local m_quince_lit = (m_quince * mask(function(x, y) return x < 615 and 1 or 0 end)):blur(15)
work(m_quince_lit, {
  hand = "body",
  pile = p_quince_lit_op,
  angle = 1.4,
  coverage = 2.0
})

-- Table bounce on right contour
local m_quince_bounce = (m_quince * mask(function(x, y) return x > 675 and y > 530 and 1 or 0 end)):blur(8)
work(m_quince_bounce, {
  hand = "detail",
  pile = p_walnut_shell,
  coverage = 1.2
})

-- Internal blend across knobby volume to fuse planes seamlessly
blend(m_quince, {angle = 1.35, coverage = 1.0})

-- Impasto highlight on sunlit crest
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{572, 572}, {566, 598}, {570, 622}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(567, 596, {pressure = 1.0})

-- Arching woody stalk & crown socket
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_walnut_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_walnut_shell, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.85, 0.5}})
b_r1:load(p_quince_lit_op, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})

-- Calyx eye and contact shadow
b_r1:load(p_walnut_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})
b_r3:load(p_walnut_deep, 1.0)
b_r3:stroke({{576, 675}, {615, 677}, {660, 676}, {688, 672}}, {pressure = {0.8, 1.0, 0.9, 0.7}})

-- Quince cast shadow flowing to the right
local b_f14 = brush("filbert", 14)
b_f14:load(p_cast_shadow, 0.75)
b_f14:stroke({{660, 676}, {705, 682}, {755, 694}}, {pressure = {0.85, 0.7, 0.3}, swell = {1.0, 1.4, 0.7}})
local b_badger = brush("badger", 20)
b_badger:stroke({{680, 685}, {755, 695}}, {pressure = 0.18})

-- 3. SOLID TACTILE WALNUTS VIA DETAIL WORK & MODELING
-- A. Cracked Walnut Half (center at x = 482, y = 654)
local m_walnut1 = ellipse(482, 654, 16, 11)
work(m_walnut1, {
  hand = "detail",
  pile = p_walnut_shell,
  coverage = 3.0,
  fill = true,
  edge = "firm"
})

-- Shaded hollow & woody septum
b_r2:load(p_walnut_deep, 1.0)
b_r2:stroke({{472, 654}, {483, 655}, {494, 654}}, {pressure = 0.95})
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{483, 646}, {483, 663}}, {pressure = 0.9})

-- Sculpted ivory kernel lobes in juicy impasto dabs
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{473, 650}, {478, 647}, {482, 651}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{473, 657}, {477, 660}, {482, 657}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{485, 650}, {490, 648}, {494, 652}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{485, 658}, {490, 661}, {494, 657}}, {pressure = 1.0})

-- Impasto highlights on kernel ridges
b_r1:load(p_walnut_rim, 1.0)
b_r1:touch(476, 648, {pressure = 1.0})
b_r1:touch(475, 658, {pressure = 1.0})
b_r1:touch(488, 649, {pressure = 1.0})
b_r1:touch(489, 659, {pressure = 1.0})

-- Broken shell rim catching sharp raking light
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{466, 652}, {472, 645}, {482, 643}}, {pressure = 0.95})

-- Contact & cast shadow
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{474, 666}, {492, 667}}, {pressure = 0.95})
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{498, 655}, {512, 659}, {524, 662}}, {pressure = {0.8, 0.5, 0.2}})

-- B. Whole Walnut (center at x = 528, y = 662)
local m_walnut2 = ellipse(528, 662, 15, 11)
work(m_walnut2, {
  hand = "detail",
  pile = p_walnut_shell,
  coverage = 3.0,
  fill = true,
  edge = "firm"
})

-- Suture ridge & corrugations
b_r1:load(p_walnut_deep, 0.9)
b_r1:stroke({{515, 663}, {528, 663}, {543, 664}}, {pressure = 0.9})
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{516, 662}, {528, 662}, {542, 663}}, {pressure = 0.95})

-- Raking crest light on shell
b_r2:load(p_walnut_rim, 1.0)
b_r2:stroke({{518, 658}, {526, 655}, {534, 657}}, {pressure = 0.92})

-- Shadow on lower flank
b_r2:load(p_walnut_deep, 0.95)
b_r2:stroke({{530, 669}, {540, 671}, {543, 666}}, {pressure = 0.9})

-- Whole walnut contact & cast shadow
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{522, 673}, {538, 674}}, {pressure = 0.95})
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{540, 666}, {554, 670}, {568, 673}}, {pressure = {0.8, 0.5, 0.2}})

-- 4. PITCHER CAST SHADOW (connecting pitcher to walnut)
b_f14:load(p_cast_shadow, 0.75)
b_f14:stroke({{422, 617}, {470, 622}, {520, 632}}, {pressure = {0.85, 0.7, 0.3}, swell = {1.0, 1.3, 0.7}})

-- 5. KNIFE CUTTING EDGE & BOLSTER SPECULAR ACCENTS
b_r1:load(p_steel_lit, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {785, 715}}, {pressure = 1.0})
b_r1:load(p_brass_hi, 1.0)
b_r1:touch(790, 714, {pressure = 1.0})

print("Table gap unified, quince solid, walnuts sculpted, shadows cohesive.")

--@ chunk 115
-- Piles for Finishing Quince, Walnuts, Knife, and Table
p_wall_dark     = pile{{"raw umber", 6.5}, {"bone black", 4.0}, {"yellow ochre", 1.0}, medium = 0.10}
p_stone_lit     = pile{{"lead white", 7.0}, {"yellow ochre", 3.0}, {"raw umber", 1.2}, {"red earth", 0.2}, medium = 0.08}
p_stone_mid     = pile{{"lead white", 5.0}, {"yellow ochre", 3.2}, {"raw umber", 2.0}, {"bone black", 0.4}, medium = 0.10}

p_walnut_shell  = pile{{"raw umber", 5.0}, {"red earth", 3.0}, {"yellow ochre", 3.0}, {"lead white", 1.8}, medium = 0.06}
p_walnut_kernel = pile{{"lead white", 7.8}, {"yellow ochre", 2.5}, {"raw umber", 0.5}, medium = 0.03}
p_walnut_rim    = pile{{"lead white", 8.2}, {"yellow ochre", 1.8}, medium = 0.03}
p_walnut_deep   = pile{{"bone black", 5.0}, {"raw umber", 4.0}, {"red earth", 1.5}, medium = 0.08}

p_quince_crest  = pile{{"lead white", 8.5}, {"chrome yellow", 2.0}, {"yellow ochre", 0.6}, medium = 0.03}
p_quince_lit    = pile{{"lead white", 6.5}, {"chrome yellow", 4.0}, {"yellow ochre", 2.0}, medium = 0.05}
p_cast_shadow   = pile{{"raw umber", 4.5}, {"bone black", 2.0}, {"yellow ochre", 2.2}, {"lead white", 1.2}, medium = 0.25}

p_steel_lit     = pile{{"lead white", 8.2}, {"cobalt blue", 1.0}, {"raw umber", 0.6}, medium = 0.04}
p_steel_mid     = pile{{"lead white", 4.5}, {"cobalt blue", 2.0}, {"raw umber", 2.5}, {"bone black", 1.5}, medium = 0.10}
p_brass_hi      = pile{{"lead white", 8.0}, {"yellow ochre", 2.0}, medium = 0.04}
p_ledge_chamfer = pile{{"lead white", 8.5}, {"yellow ochre", 1.2}, {"raw umber", 0.4}, medium = 0.05}

-- 1. CARVE QUINCE OUTER CONTOURS TO PERFECTION (Eliminate all whiskers)
local b_f12 = brush("filbert", 12)
local b_f8  = brush("filbert", 8)
local b_r3  = brush{kind = "round", width = 3.0, point = 0.95}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Carve wall around crown (above y = 520)
local pts_wall_q_left = {
  {540, 520}, {555, 508}, {576, 490}, {605, 478}, {626, 476}
}
b_f8:load(p_wall_dark, 1.0)
b_f8:stroke(pts_wall_q_left, {pressure = 0.95})

local pts_wall_q_right = {
  {630, 476}, {648, 480}, {672, 500}, {694, 520}
}
b_f8:load(p_wall_dark, 1.0)
b_f8:stroke(pts_wall_q_right, {pressure = 0.95})

-- Carve table along lower flanks and base (below y = 520)
-- Left flank of quince
local pts_table_q_left = {
  {542, 520}, {548, 555}, {544, 595}, {552, 635}, {570, 668}
}
b_f8:load(p_stone_mid, 1.0)
b_f8:stroke(pts_table_q_left, {pressure = 0.95})

-- Right flank of quince
local pts_table_q_right = {
  {694, 520}, {708, 555}, {712, 600}, {704, 638}, {684, 666}
}
b_f8:load(p_stone_mid, 1.0)
b_f8:stroke(pts_table_q_right, {pressure = 0.95})

-- Base of quince: carve table underneath (y = 676 to 688) to eliminate downward whiskers
b_f12:load(p_stone_lit, 1.0)
b_f12:stroke({{550, 682}, {630, 682}, {710, 682}}, {pressure = 0.98})

-- Soft badger blend inside quince to melt bristle marks into smooth knobby volume
local b_badger = brush("badger", 16)
b_badger:stroke({{568, 545}, {630, 545}, {688, 545}}, {pressure = 0.22})
b_badger:stroke({{562, 600}, {630, 600}, {696, 600}}, {pressure = 0.22})
b_badger:stroke({{578, 650}, {635, 650}, {680, 650}}, {pressure = 0.22})

-- Buttery impasto crest highlight
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{570, 572}, {564, 598}, {568, 622}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(565, 596, {pressure = 1.0})

-- Arching woody stalk & crown socket
b_r2:load(p_walnut_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_walnut_shell, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.85, 0.5}})
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})

-- Calyx eye & firm dark contact shadow under base
b_r1:load(p_walnut_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})
b_r3:load(p_walnut_deep, 1.0)
b_r3:stroke({{572, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 1.0, 0.9, 0.7}})

-- Soft cast shadow of quince flowing to the right
b_f12:load(p_cast_shadow, 0.75)
b_f12:stroke({{660, 677}, {705, 683}, {755, 694}}, {pressure = {0.85, 0.7, 0.3}, swell = {1.0, 1.4, 0.7}})
b_badger:stroke({{680, 685}, {755, 695}}, {pressure = 0.18})

-- 2. SCULPT THE WALNUTS WITH THICK IMPASTO KERNEL LOBES
-- A. Cracked Walnut Half (center at x = 482, y = 654)
-- Shaded hollow & central septum
b_r2:load(p_walnut_deep, 1.0)
b_r2:stroke({{472, 654}, {483, 655}, {494, 654}}, {pressure = 0.95})
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{483, 646}, {483, 663}}, {pressure = 0.9})

-- Sculpted ivory kernel lobes in thick impasto dabs
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{473, 650}, {478, 647}, {482, 651}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{473, 657}, {477, 660}, {482, 657}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{485, 650}, {490, 648}, {494, 652}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{485, 658}, {490, 661}, {494, 657}}, {pressure = 1.0})

-- Impasto highlights on kernel ridges
b_r1:load(p_walnut_rim, 1.0)
b_r1:touch(476, 648, {pressure = 1.0})
b_r1:touch(475, 658, {pressure = 1.0})
b_r1:touch(488, 649, {pressure = 1.0})
b_r1:touch(489, 659, {pressure = 1.0})

-- Broken shell rim catching sharp raking light
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{466, 652}, {472, 645}, {482, 643}}, {pressure = 0.95})

-- Dark contact shadow and cast shadow
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{474, 666}, {492, 667}}, {pressure = 0.95})
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{498, 655}, {512, 659}, {524, 662}}, {pressure = {0.8, 0.5, 0.2}})

-- B. Whole Walnut (center at x = 528, y = 662)
-- Suture ridge & corrugations
b_r1:load(p_walnut_deep, 0.9)
b_r1:stroke({{515, 663}, {528, 663}, {543, 664}}, {pressure = 0.9})
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{516, 662}, {528, 662}, {542, 663}}, {pressure = 0.95})

-- Raking crest light on shell
b_r2:load(p_walnut_rim, 1.0)
b_r2:stroke({{518, 658}, {526, 655}, {534, 657}}, {pressure = 0.92})

-- Shadow on lower flank
b_r2:load(p_walnut_deep, 0.95)
b_r2:stroke({{530, 669}, {540, 671}, {543, 666}}, {pressure = 0.9})

-- Whole walnut contact & cast shadow
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{522, 673}, {538, 674}}, {pressure = 0.95})
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{540, 666}, {554, 670}, {568, 673}}, {pressure = {0.8, 0.5, 0.2}})

-- 3. KNIFE STEEL BLADE & SPECULAR ACCENTS
local b_f8_k = brush("filbert", 6)
b_f8_k:load(p_steel_mid, 1.0)
b_f8_k:stroke({{675, 686}, {730, 700}, {785, 714}}, {pressure = 0.95})
b_r1:load(p_steel_lit, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {785, 715}}, {pressure = 1.0})
b_r1:load(p_brass_hi, 1.0)
b_r1:touch(790, 714, {pressure = 1.0})

-- 4. RAZOR CHAMFER HIGHLIGHT (y = 715)
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {784, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.04})
b_r1:stroke({{799, 715}, {900, 715}, {1005, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.04})

print("Quince contours cleaned, walnuts sculpted with impasto, knife restored.")

--@ chunk 116
-- Piles for Definitive Consolidation
p_wall_dark     = pile{{"raw umber", 6.5}, {"bone black", 4.0}, {"yellow ochre", 1.0}, medium = 0.10}
p_stone_lit     = pile{{"lead white", 7.0}, {"yellow ochre", 3.0}, {"raw umber", 1.2}, {"red earth", 0.2}, medium = 0.08}
p_dense_gold    = pile{{"lead white", 5.0}, {"yellow ochre", 5.0}, {"chrome yellow", 4.2}, {"red earth", 0.5}, medium = 0.04}
p_quince_lit    = pile{{"lead white", 6.5}, {"chrome yellow", 4.0}, {"yellow ochre", 2.0}, medium = 0.04}
p_quince_crest  = pile{{"lead white", 8.5}, {"chrome yellow", 2.0}, {"yellow ochre", 0.6}, medium = 0.03}

p_walnut_shell  = pile{{"raw umber", 5.0}, {"red earth", 3.0}, {"yellow ochre", 3.0}, {"lead white", 1.8}, medium = 0.06}
p_walnut_kernel = pile{{"lead white", 7.8}, {"yellow ochre", 2.5}, {"raw umber", 0.5}, medium = 0.03}
p_walnut_rim    = pile{{"lead white", 8.2}, {"yellow ochre", 1.8}, medium = 0.03}
p_walnut_deep   = pile{{"bone black", 5.0}, {"raw umber", 4.0}, {"red earth", 1.5}, medium = 0.08}

p_cast_shadow   = pile{{"raw umber", 4.5}, {"bone black", 2.0}, {"yellow ochre", 2.2}, {"lead white", 1.2}, medium = 0.25}
p_steel_lit     = pile{{"lead white", 8.2}, {"cobalt blue", 1.0}, {"raw umber", 0.6}, medium = 0.04}
p_brass_hi      = pile{{"lead white", 8.0}, {"yellow ochre", 2.0}, medium = 0.04}
p_ledge_chamfer = pile{{"lead white", 8.5}, {"yellow ochre", 1.2}, {"raw umber", 0.4}, medium = 0.05}

-- 1. CLEAN THE FIELD AROUND THE QUINCE VIA INVERSE MASK (Eliminating all whiskers and seams)
local pts_quince = {
  {628, 478}, {646, 484}, {670, 508}, {694, 546}, {710, 595},
  {706, 636}, {686, 665}, {662, 676}, {632, 678}, {598, 676},
  {574, 668}, {556, 642}, {546, 608}, {554, 562}, {575, 522},
  {598, 494}, {618, 480}
}
local m_q = poly(pts_quince, true)
local m_outside = -m_q

-- Erase whiskers on wall above table
work(m_outside * rect(480, 440, 280, 80), {
  hand = "body",
  pile = p_wall_dark,
  clip = true,
  coverage = 3.0,
  fill = true
})

-- Erase whiskers and lines on table around quince and walnuts
work(m_outside * rect(440, 520, 360, 195), {
  hand = "body",
  pile = p_stone_lit,
  angle = 0,
  clip = true,
  coverage = 3.2,
  fill = true
})

-- 2. SOLID OPAQUE QUINCE MODELING INSIDE M_Q (Clipped to exact contour)
-- Pure dense golden body coat (wiping all interior stripes)
work(m_q, {
  hand = "body",
  pile = p_dense_gold,
  angle = 1.4,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- Shaded flank on right via soft clipped mask
local m_q_shade = (m_q * mask(function(x, y) return x > 630 and 1 or 0 end)):blur(15)
work(m_q_shade, {
  hand = "body",
  pile = p_walnut_deep,
  angle = 1.4,
  clip = m_q,
  coverage = 1.8
})

-- Sunlit flank on left via soft clipped mask
local m_q_lit = (m_q * mask(function(x, y) return x < 615 and 1 or 0 end)):blur(15)
work(m_q_lit, {
  hand = "body",
  pile = p_quince_lit,
  angle = 1.4,
  clip = m_q,
  coverage = 2.0
})

-- Table bounce along right contour
local m_q_bounce = (m_q * mask(function(x, y) return x > 675 and y > 540 and 1 or 0 end)):blur(8)
work(m_q_bounce, {
  hand = "detail",
  pile = p_walnut_shell,
  clip = m_q,
  coverage = 1.2
})

-- Blend inside quince to fuse into smooth, voluptuous mass
blend(m_q, {angle = 1.35, coverage = 1.0})

-- Impasto highlight on sunlit crest
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{570, 572}, {564, 598}, {568, 622}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(565, 596, {pressure = 1.0})

-- Arching woody stalk & crown socket
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_walnut_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_walnut_shell, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.85, 0.5}})
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow under base
b_r1:load(p_walnut_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})
b_r3:load(p_walnut_deep, 1.0)
b_r3:stroke({{572, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 1.0, 0.9, 0.7}})

-- Soft transparent cast shadow flowing to the right
local b_f14 = brush("filbert", 14)
b_f14:load(p_cast_shadow, 0.75)
b_f14:stroke({{660, 677}, {705, 683}, {755, 694}}, {pressure = {0.85, 0.7, 0.3}, swell = {1.0, 1.4, 0.7}})
local b_badger = brush("badger", 20)
b_badger:stroke({{680, 685}, {755, 695}}, {pressure = 0.18})

-- 3. SOLID TACTILE WALNUTS (Cleaned and built on pristine limestone)
-- A. Cracked Walnut Half (center at x = 482, y = 654)
local m_walnut1 = ellipse(482, 654, 16, 11)
work(m_walnut1, {
  hand = "detail",
  pile = p_walnut_shell,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- Shaded interior hollow
b_r2:load(p_walnut_deep, 1.0)
b_r2:stroke({{472, 654}, {483, 655}, {494, 654}}, {pressure = 0.95})
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{483, 646}, {483, 663}}, {pressure = 0.9})

-- Sculpted ivory kernel lobes in thick creamy impasto
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{473, 650}, {478, 647}, {482, 651}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{473, 657}, {477, 660}, {482, 657}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{485, 650}, {490, 648}, {494, 652}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{485, 658}, {490, 661}, {494, 657}}, {pressure = 1.0})

-- Impasto highlights on kernel ridges
b_r1:load(p_walnut_rim, 1.0)
b_r1:touch(476, 648, {pressure = 1.0})
b_r1:touch(475, 658, {pressure = 1.0})
b_r1:touch(488, 649, {pressure = 1.0})
b_r1:touch(489, 659, {pressure = 1.0})

-- Broken shell rim catching sharp raking light
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{466, 652}, {472, 645}, {482, 643}}, {pressure = 0.95})

-- Dark contact shadow and cast shadow
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{474, 666}, {492, 667}}, {pressure = 0.95})
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{498, 655}, {512, 659}, {524, 662}}, {pressure = {0.8, 0.5, 0.2}})

-- B. Whole Walnut (center at x = 528, y = 662)
local m_walnut2 = ellipse(528, 662, 15, 11)
work(m_walnut2, {
  hand = "detail",
  pile = p_walnut_shell,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- Suture ridge & corrugations
b_r1:load(p_walnut_deep, 0.9)
b_r1:stroke({{515, 663}, {528, 663}, {543, 664}}, {pressure = 0.9})
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{516, 662}, {528, 662}, {542, 663}}, {pressure = 0.95})

-- Raking crest light on shell
b_r2:load(p_walnut_rim, 1.0)
b_r2:stroke({{518, 658}, {526, 655}, {534, 657}}, {pressure = 0.92})

-- Shadow on lower flank
b_r2:load(p_walnut_deep, 0.95)
b_r2:stroke({{530, 669}, {540, 671}, {543, 666}}, {pressure = 0.9})

-- Whole walnut contact & cast shadow
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{522, 673}, {538, 674}}, {pressure = 0.95})
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{540, 666}, {554, 670}, {568, 673}}, {pressure = {0.8, 0.5, 0.2}})

-- 4. PITCHER CAST SHADOW
b_f14:load(p_cast_shadow, 0.75)
b_f14:stroke({{422, 617}, {470, 622}, {520, 632}}, {pressure = {0.85, 0.7, 0.3}, swell = {1.0, 1.3, 0.7}})

-- 5. BODEGÓN KNIFE REFINEMENT
local b_f8_k = brush("filbert", 6)
b_f8_k:load(p_steel_lit, 0.9)
b_f8_k:stroke({{675, 686}, {730, 700}, {785, 714}}, {pressure = 0.95})
b_r1:load(p_steel_lit, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {785, 715}}, {pressure = 1.0})
b_r1:load(p_brass_hi, 1.0)
b_r1:touch(790, 714, {pressure = 1.0})

-- 6. RAZOR CHAMFER HIGHLIGHT (y = 715)
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {784, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.04})
b_r1:stroke({{799, 715}, {900, 715}, {1005, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.04})

print("Definitive consolidation completed.")

--@ chunk 117
-- 1. DISSOLVE THE RECTANGLES: CONTINUOUS SWEEPS OF WALL AND TABLE
-- A. Sweep Background Wall (y = 440 to 520, x = 405 to 1010)
local b_f24 = brush("filbert", 24)
local b_f20 = brush("filbert", 20)
local b_f14 = brush("filbert", 14)
local b_badger_lg = brush("badger", 36)

-- Left of quince crown (405 to 580)
for y = 440, 520, 14 do
  b_f20:load(p_wall_dark, 1.0)
  b_f20:stroke({{405, y}, {490, y}, {580, y}}, {pressure = 0.95})
end

-- Right of quince crown (660 to 1010)
for y = 440, 520, 14 do
  b_f24:load(p_wall_dark, 1.0)
  b_f24:stroke({{660, y}, {835, y}, {1010, y}}, {pressure = 0.95})
end

-- Soft badger sweeps across the wall
b_badger_lg:stroke({{405, 480}, {570, 480}}, {pressure = 0.22})
b_badger_lg:stroke({{670, 480}, {1010, 480}}, {pressure = 0.22})

-- B. Sweep Tabletop Horizontally to Completely Dissolve the Rectangular Border
-- Across the entire foreground (y = 675 to 715, x = -10 to 1010)
for y = 675, 715, 10 do
  b_f24:load(p_stone_lit, 1.0)
  b_f24:stroke({{-10, y}, {350, y}, {700, y}, {1010, y}}, {pressure = 0.98})
end

-- Right of quince all the way to canvas edge (y = 522 to 675, x = 712 to 1010)
for y = 522, 675, 10 do
  local p = (y < 560) and p_stone_mid or p_stone_lit
  b_f24:load(p, 1.0)
  b_f24:stroke({{710, y}, {860, y}, {1010, y}}, {pressure = 0.95})
end

-- Between pitcher and quince (y = 522 to 670, x = 430 to 552)
for y = 522, 670, 10 do
  local p = (y < 560) and p_stone_mid or p_stone_lit
  b_f14:load(p, 0.95)
  b_f14:stroke({{430, y}, {490, y}, {552, y}}, {pressure = 0.95})
end

-- Soft horizontal badger sweeps across the whole table plane
b_badger_lg:stroke({{-10, 695}, {500, 695}, {1010, 695}}, {pressure = 0.28})
b_badger_lg:stroke({{710, 560}, {1010, 560}}, {pressure = 0.22})
b_badger_lg:stroke({{710, 620}, {1010, 620}}, {pressure = 0.22})
b_badger_lg:stroke({{425, 550}, {555, 550}}, {pressure = 0.22})
b_badger_lg:stroke({{425, 600}, {555, 600}}, {pressure = 0.22})

-- 2. RESTORE QUINCE KNOBBY EDGES WET-IN-WET
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}

-- Golden body coat along contour
b_f14:load(p_dense_gold, 1.0)
b_f14:stroke({{618, 480}, {588, 515}, {564, 560}, {554, 608}, {570, 650}, {602, 674}}, {pressure = 0.98})
b_f14:load(p_dense_gold, 1.0)
b_f14:stroke({{625, 482}, {610, 525}, {595, 575}, {590, 628}, {615, 672}}, {pressure = 0.98})

-- Luminous illuminated left lobe
b_f14:load(p_quince_lit, 1.0)
b_f14:stroke({{610, 485}, {585, 520}, {564, 565}, {558, 610}, {574, 652}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.8}, swell = {0.85, 1.25, 0.85}
})

-- Core shadow down the right flank
local b_f10 = brush("filbert", 10)
b_f10:load(p_walnut_deep, 0.95)
b_f10:stroke({{642, 488}, {670, 522}, {690, 568}, {696, 616}, {680, 658}, {648, 674}}, {
  pressure = {0.8, 0.95, 0.98, 0.95, 0.85, 0.7}
})

-- Table bounce on lower-right contour
b_f10:load(p_walnut_shell, 0.9)
b_f10:stroke({{675, 515}, {698, 555}, {706, 600}, {700, 638}, {682, 666}}, {pressure = {0.4, 0.75, 0.75, 0.4}})

-- Gentle badger blend across knobby volume
local b_badger_q = brush("badger", 16)
b_badger_q:stroke({{572, 545}, {630, 545}, {685, 545}}, {pressure = 0.2})
b_badger_q:stroke({{565, 600}, {630, 600}, {695, 600}}, {pressure = 0.2})
b_badger_q:stroke({{580, 650}, {635, 650}, {680, 650}}, {pressure = 0.2})

-- Buttery impasto highlight on the knobby crest
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{570, 572}, {564, 598}, {568, 622}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(565, 596, {pressure = 1.0})

-- Arching woody stalk & crown socket
b_r2:load(p_walnut_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_walnut_shell, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.85, 0.5}})
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow under base
b_r1:load(p_walnut_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})
b_r3:load(p_walnut_deep, 1.0)
b_r3:stroke({{572, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 1.0, 0.9, 0.7}})

-- Soft transparent cast shadow flowing to the right
b_f14:load(p_cast_shadow, 0.75)
b_f14:stroke({{660, 677}, {705, 683}, {755, 694}}, {pressure = {0.85, 0.7, 0.3}, swell = {1.0, 1.4, 0.7}})

-- 3. THE WALNUTS (Sculpted in rich tactile impasto)
-- A. Cracked Walnut Half (center at x = 482, y = 654)
-- Dense woody shell base
b_r3:load(p_walnut_shell, 1.0)
b_r3:stroke({{468, 654}, {476, 664}, {492, 665}, {498, 655}, {494, 646}, {478, 645}, {468, 654}}, {pressure = 1.0})
b_r2:load(p_walnut_shell, 1.0)
b_r2:stroke({{472, 650}, {482, 652}, {492, 650}}, {pressure = 1.0})
b_r2:load(p_walnut_shell, 1.0)
b_r2:stroke({{472, 657}, {482, 658}, {492, 657}}, {pressure = 1.0})

-- Shaded hollow & central septum
b_r2:load(p_walnut_deep, 1.0)
b_r2:stroke({{472, 654}, {483, 655}, {494, 654}}, {pressure = 0.95})
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{483, 646}, {483, 663}}, {pressure = 0.9})

-- Sculpted ivory kernel lobes in thick creamy impasto
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{473, 650}, {478, 647}, {482, 651}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{473, 657}, {477, 660}, {482, 657}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{485, 650}, {490, 648}, {494, 652}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{485, 658}, {490, 661}, {494, 657}}, {pressure = 1.0})

-- Impasto highlights on kernel ridges
b_r1:load(p_walnut_rim, 1.0)
b_r1:touch(476, 648, {pressure = 1.0})
b_r1:touch(475, 658, {pressure = 1.0})
b_r1:touch(488, 649, {pressure = 1.0})
b_r1:touch(489, 659, {pressure = 1.0})

-- Broken shell rim catching sharp raking light
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{466, 652}, {472, 645}, {482, 643}}, {pressure = 0.95})

-- Dark contact shadow and cast shadow
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{474, 666}, {492, 667}}, {pressure = 0.95})
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{498, 655}, {512, 659}, {524, 662}}, {pressure = {0.8, 0.5, 0.2}})

-- B. Whole Walnut (center at x = 528, y = 662)
-- Dense woody shell base
b_r3:load(p_walnut_shell, 1.0)
b_r3:stroke({{514, 662}, {522, 653}, {536, 654}, {544, 664}, {538, 673}, {524, 672}, {514, 662}}, {pressure = 1.0})
b_r2:load(p_walnut_shell, 1.0)
b_r2:stroke({{520, 660}, {530, 662}, {538, 660}}, {pressure = 1.0})
b_r2:load(p_walnut_shell, 1.0)
b_r2:stroke({{520, 666}, {530, 667}, {538, 666}}, {pressure = 1.0})

-- Suture ridge & corrugations
b_r1:load(p_walnut_deep, 0.9)
b_r1:stroke({{515, 663}, {528, 663}, {543, 664}}, {pressure = 0.9})
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{516, 662}, {528, 662}, {542, 663}}, {pressure = 0.95})

-- Raking crest light on shell
b_r2:load(p_walnut_rim, 1.0)
b_r2:stroke({{518, 658}, {526, 655}, {534, 657}}, {pressure = 0.92})

-- Shadow on lower flank
b_r2:load(p_walnut_deep, 0.95)
b_r2:stroke({{530, 669}, {540, 671}, {543, 666}}, {pressure = 0.9})

-- Whole walnut contact & cast shadow
b_r1:load(p_walnut_deep, 1.0)
b_r1:stroke({{522, 673}, {538, 674}}, {pressure = 0.95})
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{540, 666}, {554, 670}, {568, 673}}, {pressure = {0.8, 0.5, 0.2}})

-- 4. PITCHER CAST SHADOW (flowing across table to walnut)
b_f14:load(p_cast_shadow, 0.75)
b_f14:stroke({{422, 617}, {470, 622}, {520, 632}}, {pressure = {0.85, 0.7, 0.3}, swell = {1.0, 1.3, 0.7}})

-- 5. BODEGÓN KNIFE (Steel blade and brass bolster)
local b_f8_k = brush("filbert", 6)
b_f8_k:load(p_steel_lit, 0.9)
b_f8_k:stroke({{675, 686}, {730, 700}, {785, 714}}, {pressure = 0.95})
b_r1:load(p_steel_lit, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {785, 715}}, {pressure = 1.0})
b_r1:load(p_brass_hi, 1.0)
b_r1:touch(790, 714, {pressure = 1.0})

-- 6. RAZOR CHAMFER HIGHLIGHT (y = 715)
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {784, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.04})
b_r1:stroke({{799, 715}, {900, 715}, {1005, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.04})

print("Continuous surfaces reinstated: wall, table, quince, walnuts, and knife complete.")

--@ chunk 118
-- Piles for Architectural Ground and Pitcher
p_wall_dark     = pile{{"raw umber", 6.5}, {"bone black", 4.0}, {"yellow ochre", 1.0}, medium = 0.10}
p_stone_lit     = pile{{"lead white", 7.0}, {"yellow ochre", 3.0}, {"raw umber", 1.2}, {"red earth", 0.2}, medium = 0.08}
p_stone_mid     = pile{{"lead white", 5.0}, {"yellow ochre", 3.2}, {"raw umber", 2.0}, {"bone black", 0.4}, medium = 0.10}
p_stone_back    = pile{{"lead white", 3.8}, {"yellow ochre", 2.5}, {"raw umber", 3.0}, {"bone black", 0.8}, medium = 0.12}

p_jug_body      = pile{{"red earth", 4.2}, {"yellow ochre", 4.0}, {"raw umber", 2.0}, {"lead white", 1.0}, medium = 0.10}
p_jug_core      = pile{{"raw umber", 5.5}, {"red earth", 3.0}, {"bone black", 1.8}, medium = 0.12}
p_jug_deep      = pile{{"bone black", 5.0}, {"raw umber", 4.0}, {"red earth", 1.5}, medium = 0.10}
p_jug_reflect   = pile{{"yellow ochre", 4.2}, {"raw umber", 2.8}, {"red earth", 2.2}, {"lead white", 1.5}, medium = 0.12}
p_jug_hi        = pile{{"lead white", 5.5}, {"yellow ochre", 3.2}, {"red earth", 1.2}, {"vermilion", 0.3}, medium = 0.06}
p_jug_glaze     = pile{{"lead white", 8.5}, {"yellow ochre", 1.5}, {"raw umber", 0.3}, medium = 0.04}
p_ledge_chamfer = pile{{"lead white", 8.5}, {"yellow ochre", 1.2}, {"raw umber", 0.4}, medium = 0.05}

-- 1. UNIFIED BACKGROUND WALL SWEEPS (y = 300 to 520, x = 425 to 1010)
local b_f24 = brush("filbert", 24)
for y = 300, 520, 10 do
  b_f24:load(p_wall_dark, 1.0)
  b_f24:stroke({{425, y}, {720, y}, {1010, y}}, {pressure = 0.98})
end

-- 2. UNIFIED LIMESTONE TABLETOP SWEEPS (y = 520 to 715)
-- Across right half (y = 520 to 615, x = 425 to 1010)
local b_f28 = brush("filbert", 28)
for y = 520, 615, 8 do
  local p = (y < 545) and p_stone_back or ((y < 575) and p_stone_mid or p_stone_lit)
  b_f28:load(p, 1.0)
  b_f28:stroke({{425, y}, {720, y}, {1010, y}}, {pressure = 0.98})
end

-- Across the ENTIRE canvas foreground (y = 615 to 715, x = -10 to 1010)
for y = 615, 715, 8 do
  b_f28:load(p_stone_lit, 1.0)
  b_f28:stroke({{-10, y}, {350, y}, {700, y}, {1010, y}}, {pressure = 0.98})
end

-- 3. BADGER BLEND TO FUSE SURFACES INTO CONTINUOUS STONE AND WALL
local b_badger = brush("badger", 40)
-- Table horizontal sweeps
b_badger:stroke({{-10, 540}, {500, 540}, {1010, 540}}, {pressure = 0.25})
b_badger:stroke({{-10, 580}, {500, 580}, {1010, 580}}, {pressure = 0.25})
b_badger:stroke({{-10, 630}, {500, 630}, {1010, 630}}, {pressure = 0.28})
b_badger:stroke({{-10, 675}, {500, 675}, {1010, 675}}, {pressure = 0.30})
b_badger:stroke({{-10, 705}, {500, 705}, {1010, 705}}, {pressure = 0.30})

-- Wall horizontal sweeps
b_badger:stroke({{425, 360}, {720, 360}, {1010, 360}}, {pressure = 0.22})
b_badger:stroke({{425, 440}, {720, 440}, {1010, 440}}, {pressure = 0.22})
b_badger:stroke({{425, 500}, {720, 500}, {1010, 500}}, {pressure = 0.22})

-- 4. RESTORE PITCHER'S SWELLING RIGHT FLANK AND CORE SHADOW
local b_f16 = brush("filbert", 16)
local b_f12 = brush("filbert", 12)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Fill terracotta body on right belly
for y = 350, 614, 8 do
  local xl = 360
  local xr
  if y < 440 then
    xr = 402 + (465 - 402) * (y - 350) / 90
  elseif y < 485 then
    xr = 465 + (472 - 465) * (y - 440) / 45
  elseif y < 550 then
    xr = 472 - (472 - 456) * (y - 485) / 65
  else
    xr = 456 - (456 - 422) * (y - 550) / 64
  end
  b_f16:load(p_jug_body, 1.0)
  b_f16:stroke({{xl, y}, {(xl + xr) * 0.5, y}, {xr, y}}, {pressure = 0.98})
end

-- Deep core shadow turning down right flank
b_f12:load(p_jug_core, 1.0)
b_f12:stroke({{400, 340}, {425, 395}, {452, 450}, {456, 500}, {442, 550}, {415, 605}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.9, 0.75}
})
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{430, 400}, {454, 450}, {460, 495}, {448, 545}, {426, 595}, {416, 612}}, {
  pressure = {0.7, 0.95, 0.95, 0.7}
})

-- Warm table bounce on right contour
b_f10:load(p_jug_reflect, 0.95)
b_f10:stroke({{410, 335}, {435, 390}, {464, 445}, {470, 485}, {460, 535}, {442, 580}, {422, 614}}, {
  pressure = {0.35, 0.75, 0.8, 0.4}
})

-- Soft internal blend across pitcher belly
local b_badger_jug = brush("badger", 18)
b_badger_jug:stroke({{340, 480}, {400, 480}, {455, 480}}, {pressure = 0.22})
b_badger_jug:stroke({{340, 540}, {400, 540}, {445, 540}}, {pressure = 0.22})
b_badger_jug:stroke({{350, 585}, {400, 585}, {430, 585}}, {pressure = 0.22})

-- Footring and occlusion contact shadow
b_r3:load(p_jug_body, 1.0)
b_r3:stroke({{325, 615}, {372, 617}, {422, 615}}, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.8, 1.0, 0.8}})

-- Shoulder glaze catchlight
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})

-- Clean wall outside right neck and shoulder
local pts_wall_jug_r = {
  {406, 268}, {403, 310}, {408, 335}, {426, 368}, {450, 410}, {468, 450}, {474, 485}
}
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke(pts_wall_jug_r, {pressure = 0.95})

-- 5. CRISP RAZOR CHAMFER HIGHLIGHT (y = 715)
b_r1:load(p_ledge_chamfer, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {700, 715}, {1005, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.04})

print("Pristine architectural setting and complete earthenware pitcher established.")

--@ chunk 119
-- Piles for Definitive Still Life Execution (16 piles)
p_jug_body      = pile{{"red earth", 4.2}, {"yellow ochre", 4.0}, {"raw umber", 2.0}, {"lead white", 1.0}, medium = 0.08}
p_jug_lit       = pile{{"yellow ochre", 4.5}, {"red earth", 3.0}, {"lead white", 2.8}, {"vermilion", 0.7}, medium = 0.08}
p_jug_core      = pile{{"raw umber", 5.5}, {"red earth", 3.0}, {"bone black", 1.8}, medium = 0.10}
p_jug_deep      = pile{{"bone black", 5.0}, {"raw umber", 4.0}, {"red earth", 1.5}, medium = 0.08}
p_jug_glaze     = pile{{"lead white", 8.5}, {"yellow ochre", 1.5}, {"raw umber", 0.3}, medium = 0.04}

p_dense_gold    = pile{{"lead white", 5.0}, {"yellow ochre", 5.0}, {"chrome yellow", 4.2}, {"red earth", 0.5}, medium = 0.04}
p_quince_lit    = pile{{"lead white", 6.5}, {"chrome yellow", 4.0}, {"yellow ochre", 2.0}, medium = 0.04}
p_quince_crest  = pile{{"lead white", 8.5}, {"chrome yellow", 2.0}, {"yellow ochre", 0.6}, medium = 0.03}

p_walnut_shell  = pile{{"raw umber", 5.0}, {"red earth", 3.0}, {"yellow ochre", 3.0}, {"lead white", 1.8}, medium = 0.06}
p_walnut_kernel = pile{{"lead white", 7.8}, {"yellow ochre", 2.5}, {"raw umber", 0.5}, medium = 0.03}
p_walnut_rim    = pile{{"lead white", 8.2}, {"yellow ochre", 1.8}, medium = 0.03}

p_steel_lit     = pile{{"lead white", 8.2}, {"cobalt blue", 1.0}, {"raw umber", 0.6}, medium = 0.04}
p_steel_mid     = pile{{"lead white", 4.5}, {"cobalt blue", 2.0}, {"raw umber", 2.5}, {"bone black", 1.5}, medium = 0.10}
p_brass         = pile{{"yellow ochre", 6.0}, {"red earth", 1.5}, {"lead white", 2.5}, medium = 0.08}
p_brass_hi      = pile{{"lead white", 8.0}, {"yellow ochre", 2.0}, medium = 0.04}
p_cast_shadow   = pile{{"raw umber", 4.5}, {"bone black", 2.0}, {"yellow ochre", 2.2}, {"lead white", 1.2}, medium = 0.25}

-- 1. PITCHER BELLY VERTICAL CONSOLIDATION (Smooth, seamless terracotta volume)
local b_f16 = brush("filbert", 16)
local b_f14 = brush("filbert", 14)
local b_f12 = brush("filbert", 12)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Vertical sweeps down center and right belly
b_f16:load(p_jug_body, 1.0)
b_f16:stroke({{375, 340}, {380, 440}, {385, 530}, {375, 610}}, {pressure = 0.98})
b_f16:load(p_jug_body, 1.0)
b_f16:stroke({{405, 350}, {420, 440}, {425, 530}, {405, 610}}, {pressure = 0.98})

-- Lit flank on left
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{345, 274}, {348, 330}, {330, 390}, {295, 450}, {288, 500}, {300, 550}, {330, 595}, {345, 614}}, {
  pressure = {0.85, 0.98, 0.95, 0.9, 0.75}
})

-- Core shadow down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{400, 340}, {428, 400}, {452, 455}, {456, 505}, {440, 555}, {415, 608}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.9, 0.75}
})
b_f12:load(p_jug_deep, 0.95)
b_f12:stroke({{432, 410}, {455, 460}, {460, 505}, {448, 550}, {425, 598}, {416, 612}}, {
  pressure = {0.7, 0.95, 0.95, 0.7}
})

-- Table bounce on right contour
local b_f10 = brush("filbert", 10)
b_f10:load(p_walnut_shell, 0.95)
b_f10:stroke({{410, 335}, {435, 390}, {464, 445}, {470, 485}, {460, 535}, {442, 580}, {422, 614}}, {
  pressure = {0.35, 0.75, 0.8, 0.4}
})

-- Soft badger blend across belly
local b_badger = brush("badger", 22)
b_badger:stroke({{330, 480}, {395, 480}, {455, 480}}, {pressure = 0.22})
b_badger:stroke({{335, 540}, {395, 540}, {445, 540}}, {pressure = 0.22})
b_badger:stroke({{345, 585}, {395, 585}, {430, 585}}, {pressure = 0.22})

-- Shoulder glaze catchlight & footring
b_r3:load(p_jug_glaze, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.8, 1.0, 0.8}})

-- 2. SOLID SCULPTURAL QUINCE MODELING VIA CLIPPED WORK
local pts_quince = {
  {628, 478}, {646, 484}, {670, 508}, {694, 546}, {710, 595},
  {706, 636}, {686, 665}, {662, 676}, {632, 678}, {598, 676},
  {574, 668}, {556, 642}, {546, 608}, {554, 562}, {575, 522},
  {598, 494}, {618, 480}
}
local m_q = poly(pts_quince, true)

-- Dense golden body fill strictly inside silhouette
work(m_q, {
  hand = "body",
  pile = p_dense_gold,
  angle = 1.4,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- Shaded flank on right via soft clipped mask
local m_q_shade = (m_q * mask(function(x, y) return x > 630 and 1 or 0 end)):blur(15)
work(m_q_shade, {
  hand = "body",
  pile = p_jug_deep,
  angle = 1.4,
  clip = m_q,
  coverage = 1.8
})

-- Sunlit flank on left via soft clipped mask
local m_q_lit = (m_q * mask(function(x, y) return x < 615 and 1 or 0 end)):blur(15)
work(m_q_lit, {
  hand = "body",
  pile = p_quince_lit,
  angle = 1.4,
  clip = m_q,
  coverage = 2.0
})

-- Table bounce on right contour
local m_q_bounce = (m_q * mask(function(x, y) return x > 675 and y > 540 and 1 or 0 end)):blur(8)
work(m_q_bounce, {
  hand = "detail",
  pile = p_walnut_shell,
  clip = m_q,
  coverage = 1.2
})

-- Blend inside quince to fuse into smooth, knobby volume
blend(m_q, {angle = 1.35, coverage = 1.0})

-- Impasto highlight on sunlit crest
b_r3:load(p_quince_crest, 1.0)
b_r3:stroke({{570, 572}, {564, 598}, {568, 622}}, {pressure = {0.6, 1.0, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_crest, 1.0)
b_r1:touch(565, 596, {pressure = 1.0})

-- Arching woody stalk & crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_walnut_shell, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.85, 0.5}})
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow under base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{572, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 1.0, 0.9, 0.7}})

-- Soft transparent cast shadow of quince
b_f14:load(p_cast_shadow, 0.75)
b_f14:stroke({{660, 677}, {705, 683}, {755, 694}}, {pressure = {0.85, 0.7, 0.3}, swell = {1.0, 1.4, 0.7}})
local b_badger_sh = brush("badger", 18)
b_badger_sh:stroke({{680, 685}, {755, 695}}, {pressure = 0.18})

-- 3. THE WALNUTS (Solid tactile foreground jewels)
-- A. Cracked Walnut Half (center at x = 482, y = 654)
local m_w1 = ellipse(482, 654, 16, 11)
work(m_w1, {
  hand = "detail",
  pile = p_walnut_shell,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- Shaded hollow & central septum
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{472, 654}, {483, 655}, {494, 654}}, {pressure = 0.95})
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{483, 646}, {483, 663}}, {pressure = 0.9})

-- Sculpted ivory kernel lobes in thick creamy impasto
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{473, 650}, {478, 647}, {482, 651}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{473, 657}, {477, 660}, {482, 657}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{485, 650}, {490, 648}, {494, 652}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{485, 658}, {490, 661}, {494, 657}}, {pressure = 1.0})

-- Impasto highlights on kernel ridges
b_r1:load(p_walnut_rim, 1.0)
b_r1:touch(476, 648, {pressure = 1.0})
b_r1:touch(475, 658, {pressure = 1.0})
b_r1:touch(488, 649, {pressure = 1.0})
b_r1:touch(489, 659, {pressure = 1.0})

-- Broken shell rim catching sharp raking light
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{466, 652}, {472, 645}, {482, 643}}, {pressure = 0.95})

-- Contact & cast shadow
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{474, 666}, {492, 667}}, {pressure = 0.95})
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{498, 655}, {512, 659}, {524, 662}}, {pressure = {0.8, 0.5, 0.2}})

-- B. Whole Walnut (center at x = 528, y = 662)
local m_w2 = ellipse(528, 662, 15, 11)
work(m_w2, {
  hand = "detail",
  pile = p_walnut_shell,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- Suture ridge & corrugations
b_r1:load(p_jug_deep, 0.9)
b_r1:stroke({{515, 663}, {528, 663}, {543, 664}}, {pressure = 0.9})
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{516, 662}, {528, 662}, {542, 663}}, {pressure = 0.95})

-- Raking crest light on shell
b_r2:load(p_walnut_rim, 1.0)
b_r2:stroke({{518, 658}, {526, 655}, {534, 657}}, {pressure = 0.92})

-- Shadow on lower flank
b_r2:load(p_jug_deep, 0.95)
b_r2:stroke({{530, 669}, {540, 671}, {543, 666}}, {pressure = 0.9})

-- Whole walnut contact & cast shadow
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{522, 673}, {538, 674}}, {pressure = 0.95})
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{540, 666}, {554, 670}, {568, 673}}, {pressure = {0.8, 0.5, 0.2}})

-- 4. PITCHER CAST SHADOW (flowing across table to walnut)
b_f14:load(p_cast_shadow, 0.75)
b_f14:stroke({{422, 617}, {470, 622}, {520, 632}}, {pressure = {0.85, 0.7, 0.3}, swell = {1.0, 1.3, 0.7}})

-- 5. BODEGÓN KNIFE (Steel blade, brass bolster, turned handle)
-- Blade flat
local b_f8_k = brush("filbert", 6)
b_f8_k:load(p_steel_mid, 1.0)
b_f8_k:stroke({{675, 686}, {730, 700}, {785, 714}}, {pressure = 0.95})

-- Razor-sharp cutting edge catching brilliant specular light
b_r1:load(p_steel_lit, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {785, 715}}, {pressure = 1.0})

-- Brass bolster at ledge edge
b_r2:load(p_brass, 1.0)
b_r2:stroke({{785, 713}, {798, 718}}, {pressure = 0.95})
b_r1:load(p_brass_hi, 1.0)
b_r1:touch(790, 714, {pressure = 1.0})

-- Turned handle projecting over front ledge ({798, 718} to {865, 746})
local b_r5_h = brush{kind = "round", width = 5.2, point = 0.9}
b_r5_h:load(p_jug_deep, 1.0)
b_r5_h:stroke({{798, 718}, {832, 732}, {865, 746}}, {pressure = 0.98, swell = {0.9, 1.15, 0.95}})
local b_r3_h = brush{kind = "round", width = 2.8, point = 0.92}
b_r3_h:load(p_jug_body, 1.0)
b_r3_h:stroke({{799, 716}, {832, 730}, {864, 744}}, {pressure = 0.9})
b_r1:load(p_jug_body, 1.0)
b_r1:touch(865, 746, {pressure = 0.98})

-- Vertical handle cast shadow down ledge face
local b_r3_sh = brush{kind = "round", width = 3.2, point = 0.9}
b_r3_sh:load(p_jug_deep, 0.85)
b_r3_sh:stroke({{802, 716}, {805, 735}, {808, 755}}, {pressure = {0.85, 0.6, 0.2}})

-- Blade cast shadow under blade on table
b_r2:load(p_cast_shadow, 0.85)
b_r2:stroke({{678, 689}, {730, 703}, {784, 717}}, {pressure = {0.5, 0.8, 0.9}})

-- 6. RAZOR CHAMFER HIGHLIGHT ALONG FRONT RIM (y = 715)
b_r1:load(p_jug_glaze, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {784, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.04})
b_r1:stroke({{799, 715}, {900, 715}, {1005, 715}}, {pressure = {0.8, 0.95, 0.8}, shake = 0.04})

print("Still life completely painted, modeled, and consolidated.")

--@ chunk 120
print("Test chunk execution")

--@ chunk 121
-- Test defining piles
p_jug_body      = pile{{"red earth", 4.2}, {"yellow ochre", 4.0}, {"raw umber", 2.0}, {"lead white", 1.0}, medium = 0.08}
p_jug_lit       = pile{{"yellow ochre", 4.5}, {"red earth", 3.0}, {"lead white", 2.8}, {"vermilion", 0.7}, medium = 0.08}
p_jug_core      = pile{{"raw umber", 5.5}, {"red earth", 3.0}, {"bone black", 1.8}, medium = 0.10}
p_jug_deep      = pile{{"bone black", 5.0}, {"raw umber", 4.0}, {"red earth", 1.5}, medium = 0.08}
p_jug_glaze     = pile{{"lead white", 8.5}, {"yellow ochre", 1.5}, {"raw umber", 0.3}, medium = 0.04}

p_quince_body   = pile{{"lead white", 6.0}, {"chrome yellow", 5.0}, {"yellow ochre", 4.0}, {"red earth", 0.5}, medium = 0.03}
p_quince_lit    = pile{{"lead white", 7.5}, {"chrome yellow", 4.0}, {"yellow ochre", 1.5}, medium = 0.03}
p_quince_hi     = pile{{"lead white", 9.0}, {"chrome yellow", 2.0}, {"yellow ochre", 0.5}, medium = 0.02}
p_quince_shade  = pile{{"raw umber", 5.5}, {"red earth", 3.0}, {"yellow ochre", 3.5}, {"lead white", 1.5}, medium = 0.06}
p_quince_bounce = pile{{"yellow ochre", 4.5}, {"lead white", 3.5}, {"raw umber", 1.5}, {"red earth", 0.8}, medium = 0.06}

p_walnut_base   = pile{{"lead white", 4.5}, {"yellow ochre", 4.0}, {"raw umber", 3.0}, {"red earth", 2.0}, medium = 0.03}
p_walnut_kernel = pile{{"lead white", 8.0}, {"yellow ochre", 2.0}, {"raw umber", 0.5}, medium = 0.02}
p_walnut_rim    = pile{{"lead white", 8.5}, {"yellow ochre", 1.5}, medium = 0.02}

p_steel_lit     = pile{{"lead white", 8.2}, {"cobalt blue", 1.0}, {"raw umber", 0.6}, medium = 0.04}
p_steel_mid     = pile{{"lead white", 4.5}, {"cobalt blue", 2.0}, {"raw umber", 2.5}, {"bone black", 1.5}, medium = 0.10}
p_brass_hi      = pile{{"lead white", 8.0}, {"yellow ochre", 2.0}, medium = 0.04}

print("16 piles mixed successfully!")

--@ chunk 122
local b = brush("round", 2)
b:load(p_quince_shade, 0.8)
b:stroke({{678, 689}, {730, 703}, {784, 717}}, {pressure = {0.5, 0.8, 0.9}})

--@ chunk 123
-- Operation 1: Perfect the Earthenware Pitcher Belly & Core Shadow
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f12 = brush("filbert", 12)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Vertical sweeps down center and right belly to fuse all planes
b_f18:load(p_jug_body, 1.0)
b_f18:stroke({{380, 330}, {385, 430}, {385, 530}, {375, 612}}, {pressure = 0.98})
b_f14:load(p_jug_body, 1.0)
b_f14:stroke({{410, 350}, {425, 440}, {430, 530}, {408, 610}}, {pressure = 0.98})

-- Lit flank on left (raking north light)
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{345, 274}, {348, 330}, {330, 390}, {295, 450}, {288, 500}, {300, 550}, {330, 595}, {345, 614}}, {
  pressure = {0.85, 0.75}
})

-- Deep core shadow turning down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{415, 340}, {445, 420}, {456, 490}, {445, 550}, {415, 610}}, {
  pressure = {0.85, 0.75}
})
b_f12:load(p_jug_deep, 0.95)
b_f12:stroke({{435, 410}, {458, 460}, {460, 505}, {448, 550}, {426, 598}, {416, 612}}, {
  pressure = {0.7, 0.7}
})

-- Warm table bounce on right contour
local b_f10 = brush("filbert", 10)
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{410, 335}, {435, 390}, {464, 445}, {470, 485}, {460, 535}, {442, 580}, {422, 614}}, {
  pressure = {0.35, 0.4}
})

-- Soft badger blend across belly
local b_badger = brush("badger", 22)
b_badger:stroke({{330, 480}, {395, 480}, {455, 480}}, {pressure = 0.22})
b_badger:stroke({{335, 540}, {395, 540}, {445, 540}}, {pressure = 0.22})
b_badger:stroke({{345, 585}, {395, 585}, {430, 585}}, {pressure = 0.22})

-- Shoulder glaze catchlight & footring contact
b_r3:load(p_jug_glaze, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.8, 0.8}})

print("Pitcher belly smooth and noble.")

--@ chunk 124
-- Operation 2: Radiant Golden Quince
local pts_quince = {
  {628, 478}, {646, 484}, {670, 508}, {694, 546}, {710, 595},
  {706, 636}, {686, 665}, {662, 676}, {632, 678}, {598, 676},
  {574, 668}, {556, 642}, {546, 608}, {554, 562}, {575, 522},
  {598, 494}, {618, 480}
}
local m_q = poly(pts_quince, true)

-- 1. Solid opaque golden body coat
work(m_q, {
  hand = "body",
  pile = p_quince_body,
  angle = 1.4,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- 2. Model light and shade wet-in-wet with direct filbert strokes
local b_f14 = brush("filbert", 14)
local b_f12 = brush("filbert", 12)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Sunlit left lobe (raking light)
b_f12:load(p_quince_lit, 1.0)
b_f12:stroke({{610, 485}, {585, 520}, {564, 565}, {558, 610}, {574, 652}}, {
  pressure = {0.85, 0.8}, swell = {0.85, 1.25, 0.85}
})
b_f12:load(p_quince_lit, 1.0)
b_f12:stroke({{620, 500}, {598, 540}, {580, 585}, {582, 625}, {602, 660}}, {
  pressure = {0.8, 0.75}
})

-- Warm golden-amber core shadow on right flank (raw umber + red earth + yellow ochre, NO bone black!)
b_f12:load(p_quince_shade, 1.0)
b_f12:stroke({{642, 488}, {670, 522}, {690, 568}, {696, 616}, {680, 658}, {648, 674}}, {
  pressure = {0.85, 0.75}
})

-- Warm limestone bounce on lower-right contour
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{675, 515}, {698, 555}, {706, 600}, {700, 638}, {682, 666}}, {
  pressure = {0.4, 0.4}
})

-- Gentle badger blend across knobby volume
blend(m_q, {angle = 1.35, coverage = 0.8})

-- Buttery impasto highlight on sunlit crest
b_r3:load(p_quince_hi, 1.0)
b_r3:stroke({{570, 572}, {564, 598}, {568, 622}}, {pressure = {0.6, 0.5}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_hi, 1.0)
b_r1:touch(565, 596, {pressure = 1.0})

-- Arching woody stalk & crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_walnut_base, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow under base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{572, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 0.7}})

-- Transparent cast shadow of quince flowing to the right
b_f14:load(p_quince_shade, 0.7)
b_f14:stroke({{660, 677}, {705, 683}, {755, 694}}, {pressure = {0.85, 0.3}, swell = {1.0, 1.4, 0.7}})
local b_badger_sh = brush("badger", 18)
b_badger_sh:stroke({{680, 685}, {755, 695}}, {pressure = 0.18})

print("Quince radiant, knobby, and golden.")

--@ chunk 125
-- Operation 3: The Walnuts, Bodegón Knife, and Foreground Refinement
local b_f14 = brush("filbert", 14)
local b_f8  = brush("filbert", 8)
local b_r4  = brush{kind = "round", width = 4.2, point = 0.9}
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. CLEAN TABLE SPACE AROUND KNIFE (Soft horizontal strokes of stone color)
b_f14:load(p_stone_lit, 1.0)
b_f14:stroke({{660, 682}, {740, 682}, {800, 682}}, {pressure = 0.95})
b_f14:load(p_stone_lit, 1.0)
b_f14:stroke({{660, 695}, {740, 695}, {800, 695}}, {pressure = 0.95})
local b_badger_t = brush("badger", 20)
b_badger_t:stroke({{660, 690}, {800, 690}}, {pressure = 0.2})

-- Soft, transparent cast shadow of quince flowing to the right (above knife)
b_f14:load(p_quince_shade, 0.5)
b_f14:stroke({{665, 678}, {715, 682}, {765, 688}}, {pressure = {0.6, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 2. THE WALNUTS
-- A. Cracked Walnut Half (center at x = 480, y = 652)
local m_w1 = ellipse(480, 652, 16, 11)
work(m_w1, {
  hand = "detail",
  pile = p_walnut_base,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- Shaded cavity & central woody septum
b_r2:load(p_jug_core, 1.0)
b_r2:stroke({{472, 652}, {480, 653}, {490, 652}}, {pressure = 0.95})
b_r1:load(p_jug_core, 1.0)
b_r1:stroke({{481, 645}, {481, 661}}, {pressure = 0.9})

-- Sculpted ivory kernel lobes in thick creamy impasto
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{472, 648}, {476, 645}, {480, 649}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{472, 655}, {475, 658}, {480, 655}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{483, 648}, {488, 646}, {492, 650}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{483, 656}, {488, 659}, {492, 655}}, {pressure = 1.0})

-- Impasto highlights on ivory kernel ridges
b_r1:load(p_walnut_rim, 1.0)
b_r1:touch(475, 646, {pressure = 1.0})
b_r1:touch(474, 656, {pressure = 1.0})
b_r1:touch(486, 647, {pressure = 1.0})
b_r1:touch(487, 657, {pressure = 1.0})

-- Broken shell rim catching sharp raking light
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{466, 650}, {472, 643}, {480, 642}}, {pressure = 0.95})

-- Dark contact shadow & cast shadow
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{472, 664}, {490, 665}}, {pressure = 0.95})
b_r2:load(p_quince_shade, 0.75)
b_r2:stroke({{494, 653}, {508, 657}, {520, 660}}, {pressure = {0.8, 0.3}})

-- B. Whole Walnut (center at x = 530, y = 662)
local m_w2 = ellipse(530, 662, 15, 11)
work(m_w2, {
  hand = "detail",
  pile = p_walnut_base,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- Suture ridge & corrugations
b_r1:load(p_jug_core, 0.9)
b_r1:stroke({{517, 663}, {530, 663}, {545, 664}}, {pressure = 0.9})
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{518, 662}, {530, 662}, {544, 663}}, {pressure = 0.95})

-- Raking crest light on shell
b_r2:load(p_walnut_rim, 1.0)
b_r2:stroke({{520, 658}, {528, 655}, {536, 657}}, {pressure = 0.92})

-- Shadow on lower flank
b_r2:load(p_jug_core, 0.95)
b_r2:stroke({{532, 669}, {542, 671}}, {pressure = 0.9})

-- Whole walnut contact & cast shadow
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{524, 673}, {540, 674}}, {pressure = 0.95})
b_r2:load(p_quince_shade, 0.75)
b_r2:stroke({{542, 666}, {556, 670}, {570, 673}}, {pressure = {0.8, 0.3}})

-- 3. THE BODEGÓN KNIFE
-- Steel blade flat
b_f8:load(p_steel_mid, 1.0)
b_f8:stroke({{675, 686}, {730, 700}, {785, 714}}, {pressure = 0.95})

-- Dark steel spine
b_r1:load(p_jug_deep, 0.9)
b_r1:stroke({{675, 686}, {730, 698}, {785, 711}}, {pressure = 0.92})

-- Razor-sharp specular cutting edge
b_r1:load(p_steel_lit, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {785, 715}}, {pressure = 1.0})

-- Brass bolster at ledge edge
b_r2:load(p_brass_hi, 0.9)
b_r2:stroke({{785, 713}, {798, 718}}, {pressure = 0.95})
b_r1:load(p_brass_hi, 1.0)
b_r1:touch(790, 714, {pressure = 1.0})

-- Turned handle projecting over front ledge ({798, 718} to {865, 746})
local b_r5_h = brush{kind = "round", width = 5.2, point = 0.9}
b_r5_h:load(p_jug_deep, 1.0)
b_r5_h:stroke({{798, 718}, {832, 732}, {865, 746}}, {pressure = 0.98, swell = {0.9, 1.15, 0.95}})
local b_r3_h = brush{kind = "round", width = 2.8, point = 0.92}
b_r3_h:load(p_jug_body, 1.0)
b_r3_h:stroke({{799, 716}, {832, 730}, {864, 744}}, {pressure = 0.9})
b_r1:load(p_jug_body, 1.0)
b_r1:touch(865, 746, {pressure = 0.98})

-- Vertical handle cast shadow down ledge face
local b_r3_sh = brush{kind = "round", width = 3.2, point = 0.9}
b_r3_sh:load(p_jug_deep, 0.85)
b_r3_sh:stroke({{802, 716}, {805, 735}, {808, 755}}, {pressure = {0.85, 0.2}})

-- Blade cast shadow under blade on table
b_r2:load(p_quince_shade, 0.75)
b_r2:stroke({{678, 689}, {730, 703}, {784, 717}}, {pressure = {0.5, 0.8}})

-- 4. RAZOR CHAMFER HIGHLIGHT (y = 715)
b_r1:load(p_jug_glaze, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {784, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})
b_r1:stroke({{799, 715}, {900, 715}, {1005, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})

print("Walnuts, knife, and foreground ledge completed.")

--@ chunk 126
-- Piles for Finishing Touches
p_stone_lit     = pile{{"lead white", 7.0}, {"yellow ochre", 3.0}, {"raw umber", 1.2}, {"red earth", 0.2}, medium = 0.08}
p_wall_dark     = pile{{"raw umber", 6.5}, {"bone black", 4.0}, {"yellow ochre", 1.0}, medium = 0.10}

-- 1. ERASE STRAY UNDERDRAWING LINES ON TABLE BETWEEN OBJECTS
local b_r2 = brush{kind = "round", width = 2.4, point = 0.95}
b_r2:load(p_stone_lit, 1.0)
b_r2:stroke({{492, 656}, {518, 660}}, {pressure = 0.95})
b_r2:stroke({{544, 666}, {570, 670}}, {pressure = 0.95})
b_r2:stroke({{568, 670}, {615, 674}}, {pressure = 0.95})

-- Clean wall behind right neck/shoulder of pitcher
local pts_wall_pitcher = {
  {406, 268}, {403, 310}, {408, 335}, {426, 368}, {450, 410}, {468, 450}
}
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke(pts_wall_pitcher, {pressure = 0.95})

-- Clean negative space inside pitcher handle
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke({{336, 328}, {308, 346}, {278, 376}, {264, 412}, {268, 442}, {280, 432}, {298, 396}, {322, 358}, {336, 332}}, {pressure = 0.95})

-- 2. SCULPT TACTILE WALNUTS (Thick opaque impasto)
local b_r4 = brush{kind = "round", width = 4.5, point = 0.9}
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}

-- A. Cracked Walnut Half (center at x = 480, y = 652)
-- Dense woody shell base
b_r4:load(p_walnut_base, 1.0)
b_r4:touch(480, 652, {pressure = 1.0})
b_r3:load(p_jug_core, 1.0)
b_r3:touch(480, 652, {pressure = 0.95})

-- Broken shell rim
b_r2:load(p_walnut_rim, 1.0)
b_r2:stroke({{466, 650}, {474, 663}, {490, 664}, {496, 653}, {492, 644}, {476, 643}, {466, 650}}, {pressure = 0.95})

-- Ivory kernel lobes sculpted in thick creamy impasto
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{472, 648}, {476, 645}, {480, 649}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{472, 655}, {475, 658}, {480, 655}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{483, 648}, {488, 646}, {492, 650}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{483, 656}, {488, 659}, {492, 655}}, {pressure = 1.0})

-- Impasto highlights on ivory convolutions
b_r1:load(p_walnut_rim, 1.0)
b_r1:touch(476, 647, {pressure = 1.0})
b_r1:touch(475, 657, {pressure = 1.0})
b_r1:touch(487, 648, {pressure = 1.0})
b_r1:touch(488, 658, {pressure = 1.0})

-- Broken shell rim catching sharp raking light
b_r1:stroke({{466, 649}, {472, 643}, {480, 642}}, {pressure = 0.95})

-- Dark contact shadow & cast shadow
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{472, 664}, {490, 665}}, {pressure = 0.95})
b_r2:load(p_quince_shade, 0.7)
b_r2:stroke({{494, 653}, {508, 657}, {518, 660}}, {pressure = {0.8, 0.3}})

-- B. Whole Walnut (center at x = 530, y = 662)
-- Dense woody shell base
b_r4:load(p_walnut_base, 1.0)
b_r4:touch(530, 662, {pressure = 1.0})

-- Woody light & shade modeling
b_r2:load(p_walnut_rim, 1.0)
b_r2:stroke({{520, 657}, {530, 654}, {540, 657}}, {pressure = 0.95})
b_r2:load(p_jug_core, 1.0)
b_r2:stroke({{522, 667}, {532, 669}, {542, 666}}, {pressure = 0.95})

-- Equatorial suture ridge
b_r1:load(p_jug_deep, 0.9)
b_r1:stroke({{516, 663}, {530, 663}, {545, 664}}, {pressure = 0.85})
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{517, 662}, {530, 662}, {544, 663}}, {pressure = 0.95})

-- Contact & cast shadow
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{524, 673}, {540, 674}}, {pressure = 0.95})
b_r2:load(p_quince_shade, 0.7)
b_r2:stroke({{542, 666}, {556, 670}, {568, 673}}, {pressure = {0.8, 0.3}})

-- 3. MODEL QUINCE DRAMATIC VOLUMETRIC SCULPTING
local b_f14 = brush("filbert", 14)
local b_f12 = brush("filbert", 12)

-- Deep warm core shadow along right flank (raw umber + red earth + yellow ochre)
b_f12:load(p_quince_shade, 1.0)
b_f12:stroke({{642, 488}, {672, 525}, {692, 570}, {696, 616}, {680, 658}, {648, 674}}, {
  pressure = {0.85, 0.95, 0.95, 0.95, 0.85, 0.7}
})

-- Warm table bounce on lower right contour
local b_f10 = brush("filbert", 10)
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{675, 515}, {698, 555}, {706, 600}, {700, 638}, {682, 666}}, {pressure = {0.4, 0.4}})

-- Raking light on sunlit left lobe
b_f14:load(p_quince_lit, 1.0)
b_f14:stroke({{610, 485}, {585, 520}, {564, 565}, {558, 610}, {574, 652}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.8}, swell = {0.85, 1.25, 0.85}
})

-- Russet orchard blush scumble across central lobe
local b_stip = brush("stippler", 5)
b_stip:load(p_jug_lit, 0.5)
b_stip:stroke({{576, 555}, {592, 595}, {612, 635}}, {pressure = 0.35, shake = 0.3})

-- Buttery impasto highlight on the knobby crest
b_r3:load(p_quince_hi, 1.0)
b_r3:stroke({{570, 572}, {564, 598}, {568, 622}}, {pressure = {0.8, 0.7}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_hi, 1.0)
b_r1:touch(565, 596, {pressure = 1.0})

-- Arching woody stalk & crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_walnut_base, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow under base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{572, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 0.7}})

print("Walnuts sculpted, quince modeled with dramatic light and core shadow, contours cleaned.")

--@ chunk 127
-- 1. SCULPTURAL VOLUMETRIC MODELING OF THE QUINCE (Continuous 3D Turn)
local b_f18 = brush("filbert", 18)
local b_f16 = brush("filbert", 16)
local b_f14 = brush("filbert", 14)
local b_f12 = brush("filbert", 12)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- A. Lit crest & flank on left (raking north light)
b_f18:load(p_quince_lit, 1.0)
b_f18:stroke({{610, 482}, {585, 515}, {565, 560}, {558, 608}, {572, 650}, {598, 674}}, {pressure = 0.98})
b_f16:load(p_quince_lit, 1.0)
b_f16:stroke({{618, 485}, {598, 520}, {580, 570}, {575, 620}, {592, 660}}, {pressure = 0.95})

-- B. Glowing golden midtone / half-tone across central body
b_f18:load(p_quince_body, 1.0)
b_f18:stroke({{625, 485}, {615, 530}, {605, 580}, {605, 635}, {622, 672}}, {pressure = 0.95})
b_f16:load(p_quince_body, 1.0)
b_f16:stroke({{632, 486}, {630, 535}, {625, 590}, {628, 645}, {642, 674}}, {pressure = 0.95})

-- C. Broad, warm core shadow down right flank (raw umber + red earth + ochre)
b_f18:load(p_quince_shade, 1.0)
b_f18:stroke({{638, 488}, {655, 530}, {664, 585}, {666, 635}, {658, 672}}, {pressure = 0.95})
b_f16:load(p_quince_shade, 1.0)
b_f16:stroke({{648, 495}, {675, 540}, {686, 595}, {684, 645}, {670, 672}}, {pressure = 0.95})

-- D. Warm limestone bounce along right contour
b_f12:load(p_quince_bounce, 0.95)
b_f12:stroke({{660, 505}, {688, 550}, {702, 595}, {698, 638}, {682, 668}}, {pressure = 0.75})

-- E. Soft badger blend across knobby volume to fuse value transitions
local b_badger = brush("badger", 22)
b_badger:stroke({{565, 540}, {630, 540}, {695, 540}}, {pressure = 0.20})
b_badger:stroke({{560, 595}, {630, 595}, {700, 595}}, {pressure = 0.20})
b_badger:stroke({{570, 645}, {630, 645}, {690, 645}}, {pressure = 0.20})

-- F. Thick buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_quince_hi, 1.0)
b_r3:stroke({{570, 570}, {564, 595}, {568, 620}}, {pressure = {0.85, 0.7}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_hi, 1.0)
b_r1:touch(565, 593, {pressure = 1.0})
b_r2:load(p_quince_hi, 0.9)
b_r2:stroke({{605, 500}, {598, 515}}, {pressure = 0.8})

-- G. Woody stem & crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_walnut_base, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})

-- H. Calyx eye & dark contact shadow at base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{572, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 0.7}})

-- 2. SOLID, TACTILE, SUBSTANTIAL WALNUTS
-- A. Cracked Walnut Half (center at x = 472, y = 650, rx = 22, ry = 15)
local m_w1 = ellipse(472, 650, 22, 15)
work(m_w1, {
  hand = "detail",
  pile = p_walnut_base,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- Shaded hollow & central septum
b_r3:load(p_jug_core, 1.0)
b_r3:touch(472, 650, {pressure = 0.95})
b_r1:load(p_jug_core, 1.0)
b_r1:stroke({{472, 642}, {472, 658}}, {pressure = 0.9})

-- Sculpted ivory kernel lobes in thick creamy impasto
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{460, 646}, {466, 643}, {471, 648}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{460, 654}, {465, 658}, {471, 654}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{475, 646}, {481, 644}, {486, 649}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{475, 655}, {482, 659}, {486, 654}}, {pressure = 1.0})

-- Impasto highlights on kernel ridges
b_r1:load(p_walnut_rim, 1.0)
b_r1:touch(464, 644, {pressure = 1.0})
b_r1:touch(463, 656, {pressure = 1.0})
b_r1:touch(479, 645, {pressure = 1.0})
b_r1:touch(480, 657, {pressure = 1.0})

-- Broken shell rim catching sharp raking light
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{452, 648}, {462, 638}, {476, 636}, {488, 642}, {493, 650}}, {pressure = 0.95})

-- Dark contact shadow & cast shadow
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{460, 664}, {484, 665}}, {pressure = 0.95})
b_r2:load(p_quince_shade, 0.75)
b_r2:stroke({{492, 653}, {508, 657}, {522, 660}}, {pressure = {0.8, 0.3}})

-- B. Whole Walnut (center at x = 532, y = 660, rx = 20, ry = 15)
local m_w2 = ellipse(532, 660, 20, 15)
work(m_w2, {
  hand = "detail",
  pile = p_walnut_base,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- Woody light & shade modeling
b_r2:load(p_walnut_rim, 1.0)
b_r2:stroke({{518, 654}, {532, 650}, {546, 654}}, {pressure = 0.95})
b_r2:load(p_jug_core, 1.0)
b_r2:stroke({{520, 668}, {535, 671}, {548, 668}}, {pressure = 0.95})

-- Equatorial suture ridge
b_r1:load(p_jug_deep, 0.9)
b_r1:stroke({{514, 660}, {532, 660}, {550, 661}}, {pressure = 0.85})
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{515, 659}, {532, 659}, {549, 660}}, {pressure = 0.95})

-- Whole walnut contact & cast shadow
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{522, 673}, {544, 674}}, {pressure = 0.95})
b_r2:load(p_quince_shade, 0.75)
b_r2:stroke({{548, 664}, {565, 669}, {580, 672}}, {pressure = {0.8, 0.3}})

print("Quince volumetrically modeled, walnuts enlarged, tactile, and solid.")

--@ chunk 128
-- Operation: Pure Volumetric Modeling of Quince & Walnuts (NO horizontal badger ruts)
local b_f16 = brush("filbert", 16)
local b_f14 = brush("filbert", 14)
local b_f12 = brush("filbert", 12)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. OBLITERATE HORIZONTAL TRACKS WITH DENSE VERTICAL BODY STROKES
b_f16:load(p_quince_body, 1.0)
b_f16:stroke({{610, 482}, {600, 530}, {595, 580}, {595, 630}, {610, 672}}, {pressure = 0.98})
b_f16:load(p_quince_body, 1.0)
b_f16:stroke({{625, 485}, {620, 535}, {618, 585}, {620, 635}, {635, 674}}, {pressure = 0.98})

-- Lit flank on left (raking north light)
b_f14:load(p_quince_lit, 1.0)
b_f14:stroke({{605, 482}, {582, 515}, {562, 560}, {556, 608}, {570, 650}, {595, 674}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.8}, swell = {0.85, 1.25, 0.85}
})
b_f12:load(p_quince_lit, 1.0)
b_f12:stroke({{595, 520}, {578, 570}, {574, 615}, {588, 655}}, {pressure = 0.92})

-- Deep warm core shadow along right flank (raw umber + red earth + ochre)
b_f16:load(p_quince_shade, 1.0)
b_f16:stroke({{636, 488}, {655, 530}, {668, 585}, {670, 635}, {658, 672}}, {pressure = 0.98})
b_f14:load(p_quince_shade, 1.0)
b_f14:stroke({{645, 495}, {672, 540}, {686, 595}, {684, 645}, {668, 672}}, {pressure = 0.95})

-- Warm table bounce along right contour
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{660, 505}, {688, 550}, {702, 595}, {698, 638}, {682, 668}}, {pressure = 0.8})

-- Soft VERTICAL feathering with soft badger along the lobes (curving top to bottom, NO horizontal ruts!)
local b_badger_v = brush("badger", 16)
b_badger_v:stroke({{590, 510}, {580, 570}, {580, 630}}, {pressure = 0.18})
b_badger_v:stroke({{630, 510}, {640, 570}, {645, 630}}, {pressure = 0.18})
b_badger_v:stroke({{665, 520}, {675, 580}, {675, 640}}, {pressure = 0.18})

-- Luscious buttery impasto crest highlight
b_r3:load(p_quince_hi, 1.0)
b_r3:stroke({{568, 570}, {562, 595}, {566, 620}}, {pressure = {0.85, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_hi, 1.0)
b_r1:touch(563, 593, {pressure = 1.0})
b_r2:load(p_quince_hi, 0.9)
b_r2:stroke({{605, 500}, {598, 515}}, {pressure = 0.85})

-- Woody stem & crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_walnut_base, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_lit, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow at base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{572, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 0.7}})

-- 2. TACTILE WALNUTS (Thick opaque impasto)
-- A. Cracked Walnut Half
-- Convoluted ivory kernel lobes in thick impasto
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{460, 646}, {466, 643}, {471, 648}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{460, 654}, {465, 658}, {471, 654}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{475, 646}, {481, 644}, {486, 649}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{475, 655}, {482, 659}, {486, 654}}, {pressure = 1.0})

-- Impasto highlights on kernel ridges
b_r1:load(p_walnut_rim, 1.0)
b_r1:touch(464, 644, {pressure = 1.0})
b_r1:touch(463, 656, {pressure = 1.0})
b_r1:touch(479, 645, {pressure = 1.0})
b_r1:touch(480, 657, {pressure = 1.0})

-- Broken shell rim
b_r1:stroke({{452, 648}, {462, 638}, {476, 636}, {488, 642}, {493, 650}}, {pressure = 0.95})

-- Dark contact shadow
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{460, 664}, {484, 665}}, {pressure = 0.95})

-- B. Whole Walnut
-- Suture ridge & crest light
b_r1:load(p_jug_deep, 0.9)
b_r1:stroke({{514, 660}, {532, 660}, {550, 661}}, {pressure = 0.85})
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{515, 659}, {532, 659}, {549, 660}}, {pressure = 0.95})
b_r2:load(p_walnut_rim, 1.0)
b_r2:stroke({{518, 654}, {532, 650}, {546, 654}}, {pressure = 0.95})
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{522, 673}, {544, 674}}, {pressure = 0.95})

print("Quince vertical volume and tactile walnuts completed.")

--@ chunk 129
-- Step 1: Pitcher Right Contour Clean & Belly Vertical Unification
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. Carve away the light horizontal streak outside the pitcher's right neck and shoulder
local pts_wall_clean = {
  {406, 268}, {403, 310}, {408, 335}, {426, 368}, {450, 410}, {468, 445}, {474, 485}
}
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke(pts_wall_clean, {pressure = 0.98})
-- Broad wall sweep to cover the light streak completely
local b_f20 = brush("filbert", 20)
b_f20:load(p_wall_dark, 1.0)
b_f20:stroke({{430, 370}, {520, 370}, {620, 370}}, {pressure = 0.98})
b_f20:load(p_wall_dark, 1.0)
b_f20:stroke({{445, 390}, {530, 390}, {620, 390}}, {pressure = 0.98})

-- Clean wall inside handle loop
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke({{336, 328}, {308, 346}, {278, 376}, {264, 412}, {268, 442}, {280, 432}, {298, 396}, {322, 358}, {336, 332}}, {pressure = 0.95})

-- 2. Vertical sweeps down pitcher belly to unite terracotta volume
b_f18:load(p_jug_body, 1.0)
b_f18:stroke({{380, 320}, {385, 430}, {385, 530}, {375, 612}}, {pressure = 0.98})

-- Lit flank on left
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{345, 274}, {340, 350}, {310, 430}, {300, 500}, {320, 560}, {345, 612}}, {pressure = {0.85, 0.75}})

-- Core shadow down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{415, 340}, {445, 420}, {456, 490}, {445, 550}, {415, 610}}, {pressure = {0.85, 0.75}})
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{435, 410}, {458, 460}, {460, 505}, {448, 550}, {426, 598}, {416, 612}}, {pressure = {0.7, 0.7}})

-- Warm table bounce along right contour
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{410, 335}, {435, 390}, {464, 445}, {470, 485}, {460, 535}, {442, 580}, {422, 614}}, {pressure = {0.35, 0.4}})

-- Shoulder glaze catchlight & footring contact
b_r3:load(p_jug_glaze, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.8, 0.8}})

print("Pitcher contour cleaned and belly unified.")

--@ chunk 130
-- Step 2: Fresh Ground for Walnuts & Tactile Sculpting
-- 1. Erase the stray line and ghosts between pitcher and quince with solid limestone paint
local b_f18 = brush("filbert", 18)
for y = 635, 675, 10 do
  b_f18:load(p_stone_lit, 1.0)
  b_f18:stroke({{440, y}, {500, y}, {560, y}}, {pressure = 0.98})
end
local b_badger_t = brush("badger", 22)
b_badger_t:stroke({{440, 655}, {560, 655}}, {pressure = 0.22})

-- Clean the white streak outside the pitcher right shoulder (y = 355 to 375, x = 405 to 500)
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}

b_r3:load(p_wall_dark, 1.0)
b_r3:stroke({{405, 360}, {450, 360}, {500, 360}}, {pressure = 0.98})
b_r3:load(p_wall_dark, 1.0)
b_r3:stroke({{408, 370}, {450, 370}, {500, 370}}, {pressure = 0.98})

-- 2. PAINT THE WALNUTS ON FRESH GROUND
-- A. Cracked Walnut Half (center at x = 475, y = 650, rx = 22, ry = 15)
local m_w1 = ellipse(475, 650, 22, 15)
work(m_w1, {
  hand = "detail",
  pile = p_walnut_base,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- Shaded hollow & woody septum
b_r3:load(p_jug_core, 1.0)
b_r3:touch(475, 650, {pressure = 0.95})
b_r1:load(p_jug_core, 1.0)
b_r1:stroke({{475, 642}, {475, 658}}, {pressure = 0.9})

-- Sculpted ivory kernel lobes in thick creamy impasto
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{462, 646}, {468, 643}, {474, 648}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{462, 654}, {467, 658}, {474, 654}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{477, 646}, {483, 644}, {488, 649}}, {pressure = 1.0})
b_r2:load(p_walnut_kernel, 1.0)
b_r2:stroke({{477, 655}, {484, 659}, {488, 654}}, {pressure = 1.0})

-- Impasto highlights on kernel ridges
b_r1:load(p_walnut_rim, 1.0)
b_r1:touch(466, 644, {pressure = 1.0})
b_r1:touch(465, 656, {pressure = 1.0})
b_r1:touch(481, 645, {pressure = 1.0})
b_r1:touch(482, 657, {pressure = 1.0})

-- Broken shell rim catching sharp raking light
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{455, 648}, {465, 638}, {478, 636}, {490, 642}, {496, 650}}, {pressure = 0.95})

-- Dark contact shadow & cast shadow
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{460, 664}, {486, 665}}, {pressure = 0.95})
b_r2:load(p_quince_shade, 0.75)
b_r2:stroke({{495, 653}, {510, 657}, {524, 660}}, {pressure = {0.8, 0.3}})

-- B. Whole Walnut (center at x = 535, y = 660, rx = 20, ry = 15)
local m_w2 = ellipse(535, 660, 20, 15)
work(m_w2, {
  hand = "detail",
  pile = p_walnut_base,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- Woody light & shade modeling
b_r2:load(p_walnut_rim, 1.0)
b_r2:stroke({{520, 654}, {535, 650}, {550, 654}}, {pressure = 0.95})
b_r2:load(p_jug_core, 1.0)
b_r2:stroke({{522, 668}, {537, 671}, {552, 668}}, {pressure = 0.95})

-- Equatorial suture ridge
b_r1:load(p_jug_deep, 0.9)
b_r1:stroke({{516, 660}, {535, 660}, {554, 661}}, {pressure = 0.85})
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{517, 659}, {535, 659}, {553, 660}}, {pressure = 0.95})

-- Whole walnut contact & cast shadow
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{524, 673}, {546, 674}}, {pressure = 0.95})
b_r2:load(p_quince_shade, 0.75)
b_r2:stroke({{550, 664}, {568, 669}, {582, 672}}, {pressure = {0.8, 0.3}})

-- 3. PITCHER CAST SHADOW (flowing across table to walnuts)
b_f18:load(p_quince_shade, 0.7)
b_f18:stroke({{422, 617}, {465, 622}, {515, 632}}, {pressure = {0.85, 0.3}, swell = {1.0, 1.3, 0.7}})

print("Walnuts solidly sculpted on fresh limestone ground, pitcher shoulder cleaned.")

--@ chunk 131
-- Fresh piles for Walnut Mastery & Quince Core Shadow
p_quince_core_deep = pile{{"raw umber", 6.0}, {"red earth", 3.0}, {"yellow ochre", 2.0}, {"bone black", 0.8}, medium = 0.06}
p_kernel_solid     = pile{{"lead white", 8.2}, {"yellow ochre", 1.8}, {"raw umber", 0.4}, medium = 0.02}
p_shell_solid      = pile{{"raw umber", 4.5}, {"red earth", 3.5}, {"yellow ochre", 3.0}, {"lead white", 2.0}, medium = 0.03}

-- 1. OBLITERATE THE DIAGONAL CHALK LINE COMPLETELY
local b_f16 = brush("filbert", 16)
b_f16:load(p_stone_lit, 1.0)
b_f16:stroke({{460, 652}, {540, 662}, {625, 674}}, {pressure = 0.98})
b_f16:load(p_stone_lit, 1.0)
b_f16:stroke({{460, 660}, {540, 668}, {625, 676}}, {pressure = 0.98})
local b_badger = brush("badger", 22)
b_badger:stroke({{460, 662}, {625, 668}}, {pressure = 0.22})

-- 2. SCULPT WALNUT 1: CRACKED WALNUT HALF (center at x = 475, y = 650)
local b_r4 = brush{kind = "round", width = 4.5, point = 0.9}
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}

-- Woody shell bowl (solid opaque base)
b_r4:load(p_shell_solid, 1.0)
b_r4:stroke({{455, 650}, {464, 662}, {486, 663}, {495, 652}, {490, 640}, {468, 639}, {455, 650}}, {pressure = 1.0})
b_r3:load(p_shell_solid, 1.0)
b_r3:touch(475, 650, {pressure = 1.0})

-- Deep shaded interior cavity
b_r3:load(p_jug_core, 1.0)
b_r3:touch(475, 650, {pressure = 0.98})
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{475, 640}, {475, 660}}, {pressure = 0.95})

-- Solid, plump ivory kernel lobes (thick impasto dabs)
-- Left lobe (solid mass from 460 to 474)
b_r3:load(p_kernel_solid, 1.0)
b_r3:stroke({{463, 646}, {468, 643}, {473, 647}}, {pressure = 1.0})
b_r3:load(p_kernel_solid, 1.0)
b_r3:stroke({{463, 654}, {468, 657}, {473, 654}}, {pressure = 1.0})

-- Right lobe (solid mass from 477 to 491)
b_r3:load(p_kernel_solid, 1.0)
b_r3:stroke({{477, 646}, {482, 643}, {487, 647}}, {pressure = 1.0})
b_r3:load(p_kernel_solid, 1.0)
b_r3:stroke({{477, 654}, {482, 657}, {487, 654}}, {pressure = 1.0})

-- Delineate convoluted folds & central cleft
b_r1:load(p_jug_core, 0.95)
b_r1:stroke({{475, 641}, {475, 659}}, {pressure = 0.9})
b_r1:load(p_jug_core, 0.9)
b_r1:stroke({{465, 650}, {472, 650}}, {pressure = 0.85})
b_r1:load(p_jug_core, 0.9)
b_r1:stroke({{478, 650}, {485, 650}}, {pressure = 0.85})

-- Impasto crest catchlights on ivory convolutions
b_r1:load(p_walnut_rim, 1.0)
b_r1:touch(467, 644, {pressure = 1.0})
b_r1:touch(466, 656, {pressure = 1.0})
b_r1:touch(483, 644, {pressure = 1.0})
b_r1:touch(484, 656, {pressure = 1.0})

-- Broken shell rim catching sharp raking light
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{455, 648}, {464, 639}, {476, 638}, {488, 642}, {495, 650}}, {pressure = 0.98})

-- Dark contact shadow & cast shadow
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{458, 663}, {488, 664}}, {pressure = 0.98})
b_r2:load(p_quince_shade, 0.75)
b_r2:stroke({{492, 653}, {508, 657}, {522, 660}}, {pressure = {0.8, 0.3}})

-- 3. SCULPT WALNUT 2: INTACT WHOLE WALNUT (center at x = 535, y = 660)
-- Solid woody shell ovoid
b_r4:load(p_shell_solid, 1.0)
b_r4:stroke({{516, 660}, {524, 650}, {544, 651}, {554, 661}, {548, 671}, {528, 670}, {516, 660}}, {pressure = 1.0})
b_r3:load(p_shell_solid, 1.0)
b_r3:touch(535, 660, {pressure = 1.0})

-- Woody light on upper crest
b_r2:load(p_walnut_rim, 1.0)
b_r2:stroke({{522, 653}, {535, 650}, {548, 653}}, {pressure = 0.98})

-- Deep warm shadow on lower flank
b_r2:load(p_jug_core, 1.0)
b_r2:stroke({{522, 667}, {536, 670}, {548, 667}}, {pressure = 0.98})

-- Equatorial suture ridge (raised woody seam)
b_r1:load(p_jug_deep, 0.95)
b_r1:stroke({{516, 660}, {535, 660}, {554, 661}}, {pressure = 0.9})
b_r1:load(p_walnut_rim, 1.0)
b_r1:stroke({{517, 659}, {535, 659}, {553, 660}}, {pressure = 0.98})

-- Contact shadow & cast shadow
b_r1:load(p_jug_deep, 1.0)
b_r1:stroke({{522, 672}, {548, 673}}, {pressure = 0.98})
b_r2:load(p_quince_shade, 0.75)
b_r2:stroke({{550, 663}, {568, 668}, {582, 671}}, {pressure = {0.8, 0.3}})

-- 4. QUINCE DRAMATIC CORE SHADOW (Decisive sculptural turn)
local b_f14 = brush("filbert", 14)
local b_f12 = brush("filbert", 12)

-- Deep warm core shadow down right flank
b_f14:load(p_quince_core_deep, 1.0)
b_f14:stroke({{640, 488}, {662, 530}, {678, 580}, {680, 630}, {668, 670}}, {pressure = 0.95})
b_f12:load(p_quince_core_deep, 1.0)
b_f12:stroke({{650, 498}, {675, 540}, {688, 590}, {686, 640}, {672, 670}}, {pressure = 0.92})

-- Table bounce on right contour
local b_f10 = brush("filbert", 10)
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{665, 510}, {692, 555}, {704, 600}, {700, 638}, {684, 666}}, {pressure = 0.85})

-- Restate buttery impasto highlight on sunlit crest
b_r3:load(p_quince_hi, 1.0)
b_r3:stroke({{568, 570}, {562, 595}, {566, 620}}, {pressure = {0.85, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_hi, 1.0)
b_r1:touch(563, 593, {pressure = 1.0})

-- Dark contact shadow at quince base
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{572, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 0.7}})

print("Walnuts fully sculpted as solid objects, line erased, quince core shadow deepened.")

--@ chunk 132
-- Clean the foreground table between pitcher and quince (Eliminate the button symbols and wire lines)
local b_f24 = brush("filbert", 24)
local b_f18 = brush("filbert", 18)
local b_badger = brush("badger", 32)

-- Sweep opaque limestone body paint horizontally across the middle foreground (y = 635 to 685, x = 425 to 580)
for y = 635, 685, 8 do
  b_f24:load(p_stone_lit, 1.0)
  b_f24:stroke({{425, y}, {500, y}, {585, y}}, {pressure = 0.98})
end

-- Horizontal badger sweeps to fuse the stone surface seamlessly
b_badger:stroke({{420, 640}, {590, 640}}, {pressure = 0.25})
b_badger:stroke({{420, 660}, {590, 660}}, {pressure = 0.25})
b_badger:stroke({{420, 680}, {590, 680}}, {pressure = 0.25})

-- Re-lay the soft, transparent pitcher cast shadow flowing naturally to the right (x = 422 to 520, y = 616 to 635)
local b_f14 = brush("filbert", 14)
b_f14:load(p_quince_shade, 0.5)
b_f14:stroke({{422, 617}, {465, 622}, {515, 630}}, {pressure = {0.8, 0.2}, swell = {1.0, 1.3, 0.6}})

-- Pitcher footring contact line
local b_r3 = brush{kind = "round", width = 3.0, point = 0.9}
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.8, 0.8}})

print("Table surface unified and serene; pitcher shadow laid.")

--@ chunk 133
-- 1. COVER THE WHITE STREAK OUTSIDE PITCHER SHOULDER WITH DARK WALL PAINT
local b_f14 = brush("filbert", 14)
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{410, 365}, {480, 365}, {550, 365}}, {pressure = 1.0})
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{410, 375}, {480, 375}, {550, 375}}, {pressure = 1.0})

-- Clean wall contour right along the pitcher's true neck and shoulder
local b_r2 = brush{kind = "round", width = 2.4, point = 0.95}
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke({{406, 268}, {403, 310}, {408, 335}, {426, 368}, {450, 410}, {468, 445}}, {pressure = 1.0})

-- 2. RESTORE QUINCE LOWER-LEFT FLANK WITH DENSE GOLDEN BODY PAINT
local b_f16 = brush("filbert", 16)
local b_f12 = brush("filbert", 12)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Golden body coat filling the lower-left notch (x = 550 to 600, y = 620 to 675)
b_f16:load(p_quince_body, 1.0)
b_f16:stroke({{570, 610}, {560, 635}, {565, 660}, {585, 674}}, {pressure = 1.0})
b_f16:load(p_quince_body, 1.0)
b_f16:stroke({{585, 605}, {578, 635}, {585, 665}, {605, 674}}, {pressure = 1.0})

-- Lit flank on left (curving down from shoulder to base)
b_f14:load(p_quince_lit, 1.0)
b_f14:stroke({{610, 482}, {585, 515}, {565, 560}, {556, 608}, {568, 650}, {595, 674}}, {
  pressure = {0.85, 0.98, 1.0, 0.98, 0.8}, swell = {0.85, 1.25, 0.85}
})

-- Firm dark contact shadow under quince base
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{568, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 0.7}})

print("Pitcher shoulder streak covered, quince lower flank restored.")

--@ chunk 134
-- Piles for Quince Radiance
p_quince_pure_gold  = pile{{"yellow ochre", 5.0}, {"chrome yellow", 4.0}, {"lead white", 3.0}, {"red earth", 0.8}, medium = 0.03}
p_quince_sun        = pile{{"chrome yellow", 4.5}, {"lead white", 5.0}, {"yellow ochre", 2.5}, medium = 0.03}
p_quince_shadow     = pile{{"raw umber", 5.5}, {"red earth", 3.5}, {"yellow ochre", 2.5}, {"bone black", 0.6}, medium = 0.05}
p_quince_highlight  = pile{{"lead white", 9.0}, {"chrome yellow", 1.8}, {"yellow ochre", 0.4}, medium = 0.02}

-- 1. COVER THE WHITE STREAK ON THE WALL AT y = 450 (x = 425 to 550)
local b_f16 = brush("filbert", 16)
local b_f14 = brush("filbert", 14)
b_f16:load(p_wall_dark, 1.0)
b_f16:stroke({{425, 448}, {485, 448}, {555, 448}}, {pressure = 1.0})
b_f16:load(p_wall_dark, 1.0)
b_f16:stroke({{425, 458}, {485, 458}, {555, 458}}, {pressure = 1.0})

-- Clean wall edge along pitcher right contour
local b_r2 = brush{kind = "round", width = 2.4, point = 0.95}
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke({{426, 368}, {450, 410}, {468, 450}, {474, 485}}, {pressure = 1.0})

-- 2. RADIANT GOLDEN QUINCE (Volumetric modeling in rich warm body color)
local b_f12 = brush("filbert", 12)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Golden ochre body across central mass (y = 480 to 675, x = 575 to 645)
b_f16:load(p_quince_pure_gold, 1.0)
b_f16:stroke({{615, 482}, {600, 530}, {590, 580}, {588, 630}, {605, 674}}, {pressure = 1.0})
b_f16:load(p_quince_pure_gold, 1.0)
b_f16:stroke({{625, 485}, {618, 535}, {610, 585}, {612, 635}, {628, 674}}, {pressure = 1.0})

-- Radiant sunlit crest on left flank (x = 555 to 585)
b_f14:load(p_quince_sun, 1.0)
b_f14:stroke({{608, 482}, {584, 515}, {564, 560}, {556, 608}, {568, 650}, {595, 674}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.8}, swell = {0.85, 1.25, 0.85}
})
b_f12:load(p_quince_sun, 1.0)
b_f12:stroke({{595, 520}, {576, 570}, {572, 615}, {585, 655}}, {pressure = 0.95})

-- Broad, warm core shadow down right flank (x = 635 to 685)
b_f16:load(p_quince_shadow, 1.0)
b_f16:stroke({{638, 488}, {658, 530}, {670, 580}, {672, 630}, {660, 672}}, {pressure = 0.98})
b_f14:load(p_quince_shadow, 1.0)
b_f14:stroke({{648, 495}, {674, 540}, {686, 590}, {684, 640}, {670, 672}}, {pressure = 0.95})

-- Warm table bounce on right contour (x = 685 to 706)
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{665, 510}, {692, 555}, {704, 600}, {700, 638}, {684, 666}}, {pressure = 0.85})

-- Soft VERTICAL feathering with badger along the lobes (top to bottom)
local b_badger = brush("badger", 16)
b_badger:stroke({{585, 510}, {580, 575}, {580, 640}}, {pressure = 0.18})
b_badger:stroke({{625, 510}, {635, 575}, {640, 640}}, {pressure = 0.18})

-- Luscious buttery impasto crest highlight
b_r3:load(p_quince_highlight, 1.0)
b_r3:stroke({{568, 570}, {562, 595}, {566, 620}}, {pressure = {0.85, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_highlight, 1.0)
b_r1:touch(563, 593, {pressure = 1.0})
b_r2:load(p_quince_highlight, 0.9)
b_r2:stroke({{605, 500}, {598, 515}}, {pressure = 0.85})

-- Arching woody stalk & crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(627, 481, {pressure = 0.95})
b_r2:load(p_walnut_base, 1.0)
b_r2:stroke({{627, 480}, {632, 465}, {638, 452}, {646, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_sun, 0.9)
b_r1:stroke({{628, 478}, {633, 465}, {639, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow at base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(654, 674, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{568, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 0.7}})

print("Quince radiant and fully modeled; white wall streak obliterated.")

--@ chunk 135
-- 1. KILL THE WHITE STREAK ON WALL AT EXACT COORDINATES (y = 348 to 354, x = 420 to 490)
local b_f14 = brush("filbert", 14)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

b_r3:load(p_wall_dark, 1.0)
b_r3:stroke({{418, 350}, {455, 350}, {490, 350}}, {pressure = 1.0})
b_r3:stroke({{418, 353}, {455, 353}, {490, 353}}, {pressure = 1.0})

-- Clean wall edge along pitcher neck and shoulder (x = 406 at 270 down to 455 at 420)
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke({{406, 270}, {403, 310}, {408, 335}, {422, 360}, {445, 410}, {465, 450}}, {pressure = 1.0})

-- Model pitcher right shoulder in rich terracotta
b_r3:load(p_jug_body, 1.0)
b_r3:stroke({{405, 330}, {420, 365}, {442, 410}, {460, 460}}, {pressure = 0.98})
b_r2:load(p_jug_core, 1.0)
b_r2:stroke({{412, 345}, {426, 380}, {446, 425}, {462, 470}}, {pressure = 0.95})

-- 2. FULL VOLUMETRIC SCULPTING OF THE GOLDEN QUINCE
local b_f16 = brush("filbert", 16)
local b_f12 = brush("filbert", 12)
local b_f10 = brush("filbert", 10)

-- A. Broad, warm core shadow down the entire right flank (x = 645 to 695, y = 485 to 672)
b_f16:load(p_quince_shadow, 1.0)
b_f16:stroke({{638, 485}, {658, 530}, {672, 585}, {674, 635}, {660, 672}}, {pressure = 1.0})
b_f14:load(p_quince_shadow, 1.0)
b_f14:stroke({{648, 495}, {675, 545}, {688, 595}, {686, 640}, {670, 672}}, {pressure = 0.98})

-- B. Warm limestone bounce on far right contour (x = 685 to 705)
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{665, 510}, {692, 555}, {704, 600}, {700, 638}, {684, 666}}, {pressure = 0.85})

-- C. Glowing golden ochre midtone down center
b_f16:load(p_quince_pure_gold, 1.0)
b_f16:stroke({{622, 485}, {612, 535}, {605, 585}, {606, 635}, {620, 672}}, {pressure = 1.0})

-- D. Radiant sunlit crest on left flank (x = 550 to 595)
b_f16:load(p_quince_sun, 1.0)
b_f16:stroke({{608, 485}, {582, 525}, {562, 575}, {556, 615}, {568, 655}, {590, 674}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.8}, swell = {0.85, 1.25, 0.85}
})
b_f12:load(p_quince_sun, 1.0)
b_f12:stroke({{595, 510}, {576, 560}, {570, 610}, {582, 655}}, {pressure = 0.95})

-- E. Soft vertical blend along the organic lobes (top to bottom, NO horizontal ruts!)
local b_badger_v = brush("badger", 16)
b_badger_v:stroke({{585, 510}, {580, 580}, {580, 645}}, {pressure = 0.18})
b_badger_v:stroke({{625, 505}, {632, 580}, {635, 645}}, {pressure = 0.18})
b_badger_v:stroke({{665, 515}, {675, 580}, {675, 645}}, {pressure = 0.18})

-- F. Luscious buttery impasto crest highlight
b_r3:load(p_quince_highlight, 1.0)
b_r3:stroke({{568, 568}, {560, 595}, {564, 622}}, {pressure = {0.9, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_highlight, 1.0)
b_r1:touch(561, 593, {pressure = 1.0})
b_r2:load(p_quince_highlight, 0.9)
b_r2:stroke({{602, 498}, {595, 515}}, {pressure = 0.85})

-- G. Woody stem & crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(625, 478, {pressure = 0.95})
b_r2:load(p_walnut_base, 1.0)
b_r2:stroke({{625, 478}, {630, 465}, {636, 452}, {644, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_sun, 0.9)
b_r1:stroke({{626, 476}, {631, 465}, {637, 454}}, {pressure = 0.6})

-- H. Calyx eye & dark contact shadow at base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(650, 673, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{565, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 0.7}})

print("White streak killed, pitcher shoulder smoothed, quince fully sculpted with rich light and shade.")

--@ chunk 136
-- 1. PITCHER BELLY COMPLETE UNIFICATION (Smooth noble terracotta cylinder)
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f12 = brush("filbert", 12)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Vertical sweeps down right belly and core shadow (covering all residual horizontal marks)
b_f18:load(p_jug_body, 1.0)
b_f18:stroke({{405, 340}, {425, 430}, {435, 520}, {412, 605}}, {pressure = 0.98})
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{425, 360}, {448, 440}, {456, 510}, {440, 560}, {416, 610}}, {pressure = 0.98})
b_f12:load(p_jug_deep, 0.95)
b_f12:stroke({{438, 410}, {458, 465}, {460, 515}, {448, 555}, {426, 600}, {416, 612}}, {pressure = 0.75})

-- Table bounce on right contour
local b_f10 = brush("filbert", 10)
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{410, 335}, {435, 390}, {464, 445}, {470, 485}, {460, 535}, {442, 580}, {422, 614}}, {pressure = {0.35, 0.4}})

-- Soft vertical badger blend down pitcher belly
local b_badger = brush("badger", 24)
b_badger:stroke({{340, 360}, {340, 480}, {345, 600}}, {pressure = 0.2})
b_badger:stroke({{385, 360}, {390, 480}, {385, 600}}, {pressure = 0.2})
b_badger:stroke({{435, 380}, {445, 480}, {425, 600}}, {pressure = 0.2})

-- Shoulder glaze catchlight & footring contact
b_r3:load(p_jug_glaze, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r1:load(p_jug_glaze, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.8, 0.8}})

-- 2. QUINCE: FUSE VERTICAL STRIPES INTO VOLUPTUOUS SPHERICAL ROUNDNESS
local pts_quince = {
  {628, 478}, {646, 484}, {670, 508}, {694, 546}, {710, 595},
  {706, 636}, {686, 665}, {662, 676}, {632, 678}, {598, 676},
  {574, 668}, {556, 642}, {546, 608}, {554, 562}, {575, 522},
  {598, 494}, {618, 480}
}
local m_q = poly(pts_quince, true)

-- Blend horizontally across the vertical stripes to fuse into a seamless gradient
blend(m_q, {angle = 0, coverage = 1.0})

-- Restate buttery impasto crest highlight
b_r3:load(p_quince_highlight, 1.0)
b_r3:stroke({{568, 568}, {560, 595}, {564, 622}}, {pressure = {0.9, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_highlight, 1.0)
b_r1:touch(561, 593, {pressure = 1.0})
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_quince_highlight, 0.9)
b_r2:stroke({{602, 498}, {595, 515}}, {pressure = 0.85})

-- Arching woody stalk & crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(625, 478, {pressure = 0.95})
b_r2:load(p_walnut_base, 1.0)
b_r2:stroke({{625, 478}, {630, 465}, {636, 452}, {644, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_sun, 0.9)
b_r1:stroke({{626, 476}, {631, 465}, {637, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow at base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(650, 673, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{565, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 0.7}})

-- Soft transparent cast shadow of quince flowing to the right
b_f14:load(p_quince_shadow, 0.45)
b_f14:stroke({{665, 678}, {715, 683}, {765, 688}}, {pressure = {0.6, 0.2}})

-- 3. BODEGÓN KNIFE REFINEMENT
-- Silvery steel blade cutting edge
b_r1:load(p_steel_lit, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {785, 715}}, {pressure = 1.0})

-- Brass bolster glint
b_r1:load(p_brass_hi, 1.0)
b_r1:touch(790, 714, {pressure = 1.0})

-- Razor chamfer along front rim (y = 715)
b_r1:load(p_jug_glaze, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {784, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})
b_r1:stroke({{799, 715}, {900, 715}, {1005, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})

print("Pitcher belly perfected, quince blended horizontally into seamless volume, highlights restated.")

--@ chunk 137
-- 1. KILL THE WHITE BEAM OUTSIDE PITCHER NECK WITH FILBERT 16 (y = 352, x = 415 to 490)
local b_f16 = brush("filbert", 16)
b_f16:load(p_wall_dark, 1.0)
b_f16:stroke({{415, 352}, {455, 352}, {495, 352}}, {pressure = 1.0})

-- Clean wall edge along pitcher neck and shoulder (from rim 406, 270 down to 455, 420)
local b_r2 = brush{kind = "round", width = 2.4, point = 0.95}
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke({{406, 270}, {403, 310}, {408, 335}, {422, 360}, {442, 405}, {462, 445}}, {pressure = 1.0})

-- Model right neck/shoulder of pitcher in rich terracotta
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
b_r3:load(p_jug_body, 1.0)
b_r3:stroke({{405, 330}, {418, 360}, {438, 405}, {458, 450}}, {pressure = 0.98})
b_r2:load(p_jug_core, 1.0)
b_r2:stroke({{412, 340}, {424, 375}, {444, 420}, {460, 465}}, {pressure = 0.95})

-- 2. COMPLETE AND UNIFY THE BODEGÓN KNIFE
-- A. Clean the brown slug above the knife blade with limestone stone paint
b_f16:load(p_stone_lit, 1.0)
b_f16:stroke({{660, 680}, {730, 680}, {790, 680}}, {pressure = 1.0})
b_f16:load(p_stone_lit, 1.0)
b_f16:stroke({{660, 688}, {730, 688}, {785, 688}}, {pressure = 1.0})

-- B. Steel blade: continuous triangular blade from tip {675, 686} to bolster {786, 715}
local b_f8_k = brush("filbert", 7)
b_f8_k:load(p_steel_mid, 1.0)
b_f8_k:stroke({{675, 686}, {730, 700}, {786, 714}}, {pressure = 0.98})

-- Steel spine (dark carbon steel)
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}
b_r1:load(p_jug_deep, 0.95)
b_r1:stroke({{675, 686}, {730, 698}, {786, 712}}, {pressure = 0.92})

-- Razor-sharp specular cutting edge (silvery white)
b_r1:load(p_steel_lit, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {786, 715}}, {pressure = 1.0})

-- C. Brass Bolster / Collar bridging blade to handle ({784, 713} to {799, 718})
b_r2:load(p_brass, 1.0)
b_r2:stroke({{784, 713}, {792, 716}, {799, 718}}, {pressure = 1.0})
b_r1:load(p_brass_hi, 1.0)
b_r1:touch(791, 715, {pressure = 1.0})
b_r1:load(p_jug_deep, 0.9)
b_r1:stroke({{784, 716}, {799, 720}}, {pressure = 0.85})

-- D. Turned wood handle connection & pommel
local b_r5_h = brush{kind = "round", width = 5.2, point = 0.9}
b_r5_h:load(p_jug_deep, 1.0)
b_r5_h:stroke({{797, 718}, {832, 732}, {865, 746}}, {pressure = 0.98, swell = {0.9, 1.15, 0.95}})
local b_r3_h = brush{kind = "round", width = 2.8, point = 0.92}
b_r3_h:load(p_jug_body, 1.0)
b_r3_h:stroke({{798, 716}, {832, 730}, {864, 744}}, {pressure = 0.9})
b_r1:load(p_jug_body, 1.0)
b_r1:touch(865, 746, {pressure = 0.98})

-- Vertical handle shadow down ledge face
b_r3:load(p_jug_deep, 0.85)
b_r3:stroke({{802, 716}, {805, 735}, {808, 755}}, {pressure = {0.85, 0.2}})

-- Blade contact shadow on limestone table
b_r2:load(p_quince_shade, 0.8)
b_r2:stroke({{678, 688}, {730, 702}, {784, 716}}, {pressure = {0.5, 0.8}})

-- 3. QUINCE: DECISIVE SCULPTURAL CORE SHADOW & BUTTERY HIGHLIGHT
local b_f14 = brush("filbert", 14)
local b_f12 = brush("filbert", 12)
local b_f10 = brush("filbert", 10)

-- Core shadow running the FULL length of the right flank (y = 485 to 672)
b_f14:load(p_quince_shadow, 1.0)
b_f14:stroke({{638, 485}, {658, 530}, {672, 580}, {674, 630}, {660, 672}}, {pressure = 0.98})
b_f12:load(p_quince_shadow, 1.0)
b_f12:stroke({{648, 495}, {672, 545}, {686, 595}, {684, 640}, {670, 672}}, {pressure = 0.95})

-- Table bounce along right contour
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{665, 510}, {692, 555}, {704, 600}, {700, 638}, {684, 666}}, {pressure = 0.85})

-- Sunlit crest on left flank
b_f14:load(p_quince_sun, 1.0)
b_f14:stroke({{608, 485}, {582, 525}, {562, 575}, {556, 615}, {568, 655}, {590, 674}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.8}, swell = {0.85, 1.25, 0.85}
})

-- Golden midtone blend down center
b_f14:load(p_quince_pure_gold, 1.0)
b_f14:stroke({{620, 485}, {610, 535}, {604, 585}, {605, 635}, {618, 672}}, {pressure = 0.95})

-- Soft VERTICAL blend with badger to marry the core shadow into the midtone
local b_badger = brush("badger", 16)
b_badger:stroke({{625, 510}, {635, 575}, {638, 640}}, {pressure = 0.16})
b_badger:stroke({{585, 510}, {580, 575}, {580, 640}}, {pressure = 0.16})

-- Buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_quince_highlight, 1.0)
b_r3:stroke({{568, 568}, {560, 595}, {564, 622}}, {pressure = {0.9, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_highlight, 1.0)
b_r1:touch(561, 593, {pressure = 1.0})

-- Arching woody stalk & crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(625, 478, {pressure = 0.95})
b_r2:load(p_walnut_base, 1.0)
b_r2:stroke({{625, 478}, {630, 465}, {636, 452}, {644, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_sun, 0.9)
b_r1:stroke({{626, 476}, {631, 465}, {637, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow at base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(650, 673, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{565, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 0.7}})

-- Razor chamfer along front rim (y = 715)
b_r1:load(p_jug_glaze, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {784, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})
b_r1:stroke({{799, 715}, {900, 715}, {1005, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})

print("Pitcher neck beam killed, knife completed and connected, quince fully modeled.")

--@ chunk 138
local t = wait(3 * 24 * 60)
print("Time after drying:", t)
print("Drying quince (630, 580):", drying(630, 580))
print("Drying pitcher (380, 480):", drying(380, 480))
print("Drying table (500, 650):", drying(500, 650))

--@ chunk 139
local t = wait(4 * 24 * 60)
print("Time after drying:", t)
print("Drying quince (630, 580):", drying(630, 580))
print("Drying pitcher (380, 480):", drying(380, 480))
print("Drying table (500, 650):", drying(500, 650))

--@ chunk 140
local t = wait(3 * 24 * 60)
print("Time after drying:", t)
print("Drying quince (630, 580):", drying(630, 580))
print("Drying pitcher (380, 480):", drying(380, 480))
print("Drying table (500, 650):", drying(500, 650))

--@ chunk 141
-- Definitive Finishing Glazes and Impastos on Dry Ground
p_quince_glaze_gold   = pile{{"yellow ochre", 6.0}, {"chrome yellow", 4.0}, {"red earth", 1.0}, medium = 0.18}
p_quince_glaze_shadow = pile{{"raw umber", 6.0}, {"red earth", 3.0}, {"yellow ochre", 2.0}, {"bone black", 0.8}, medium = 0.15}
p_quince_glaze_bounce = pile{{"yellow ochre", 5.0}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.15}
p_quince_impasto_hi   = pile{{"lead white", 9.0}, {"chrome yellow", 2.0}, {"yellow ochre", 0.5}, medium = 0.02}
p_stem_dry            = pile{{"raw umber", 6.0}, {"bone black", 2.5}, {"yellow ochre", 1.5}, {"lead white", 0.8}, medium = 0.05}

p_jug_velatura        = pile{{"red earth", 5.0}, {"yellow ochre", 4.0}, {"raw umber", 2.0}, medium = 0.18}
p_glaze_touch         = pile{{"lead white", 9.2}, {"yellow ochre", 1.0}, medium = 0.02}
p_jug_deep            = pile{{"bone black", 5.0}, {"raw umber", 4.0}, {"red earth", 1.5}, medium = 0.08}

p_knife_edge          = pile{{"lead white", 9.0}, {"cobalt blue", 0.8}, {"raw umber", 0.4}, medium = 0.03}
p_brass_glint         = pile{{"lead white", 8.5}, {"yellow ochre", 2.0}, medium = 0.03}

-- 1. PITCHER FINISH (Velatura down belly & shoulder glint)
local b_f18 = brush("filbert", 18)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Smooth vertical velatura down center and right belly to fuse into uniform terracotta
b_f18:load(p_jug_velatura, 0.8)
b_f18:stroke({{375, 330}, {385, 430}, {385, 530}, {375, 612}}, {pressure = {0.5, 0.7, 0.5}})
b_f18:load(p_jug_velatura, 0.8)
b_f18:stroke({{410, 350}, {425, 440}, {430, 530}, {408, 610}}, {pressure = {0.5, 0.7, 0.5}})

-- Crisp impasto catchlight on shoulder curve
b_r3:load(p_glaze_touch, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r1:load(p_glaze_touch, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})

-- Specular glint on flared mouth rim
b_r1:load(p_glaze_touch, 1.0)
b_r1:touch(348, 272, {pressure = 1.0})

-- 2. QUINCE FINISH (Rich golden scumble, deep amber shadow, buttery crest)
local b_f16 = brush("filbert", 16)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)

-- Warm golden scumble across the entire body (y = 485 to 672, x = 560 to 650)
b_f16:load(p_quince_glaze_gold, 0.85)
b_f16:stroke({{610, 482}, {595, 530}, {585, 580}, {585, 630}, {602, 672}}, {pressure = 0.75})
b_f16:load(p_quince_glaze_gold, 0.85)
b_f16:stroke({{625, 485}, {618, 535}, {612, 585}, {615, 635}, {628, 672}}, {pressure = 0.75})

-- Deep warm core shadow running the full length of the right flank (x = 640 to 688)
b_f14:load(p_quince_glaze_shadow, 0.95)
b_f14:stroke({{638, 485}, {658, 530}, {672, 585}, {674, 635}, {660, 672}}, {pressure = 0.85})
b_f14:load(p_quince_glaze_shadow, 0.95)
b_f14:stroke({{648, 495}, {674, 545}, {686, 595}, {684, 640}, {668, 672}}, {pressure = 0.85})

-- Warm table bounce on right contour (x = 685 to 705)
b_f10:load(p_quince_glaze_bounce, 0.9)
b_f10:stroke({{665, 510}, {692, 555}, {704, 600}, {700, 638}, {684, 666}}, {pressure = 0.75})

-- Buttery impasto highlight on the knobby sunlit crest (x = 562, y = 595)
b_r3:load(p_quince_impasto_hi, 1.0)
b_r3:stroke({{568, 568}, {561, 595}, {565, 622}}, {pressure = {0.9, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_impasto_hi, 1.0)
b_r1:touch(562, 593, {pressure = 1.0})

local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_quince_impasto_hi, 0.9)
b_r2:stroke({{602, 498}, {595, 515}}, {pressure = 0.85})

-- Gnarled woody stem arching from crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(625, 478, {pressure = 0.95})
b_r2:load(p_stem_dry, 1.0)
b_r2:stroke({{625, 478}, {630, 465}, {636, 452}, {644, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_impasto_hi, 0.9)
b_r1:stroke({{626, 476}, {631, 465}, {637, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow under base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(650, 673, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{565, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 0.7}})

-- 3. BODEGÓN KNIFE SPECULAR HIGHLIGHTS
-- Razor-sharp specular cutting edge
b_r1:load(p_knife_edge, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {785, 715}}, {pressure = 1.0})

-- Specular glint on brass bolster
b_r1:load(p_brass_glint, 1.0)
b_r1:touch(791, 715, {pressure = 1.0})

-- Pommel glint
b_r1:load(p_brass_glint, 0.9)
b_r1:touch(865, 746, {pressure = 0.95})

-- 4. RAZOR CHAMFER HIGHLIGHT ALONG FRONT RIM (y = 715)
b_r1:load(p_glaze_touch, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {784, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})
b_r1:stroke({{799, 715}, {900, 715}, {1005, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})

print("Definitive glazes, velaturas, impastos, and specular highlights applied.")

--@ chunk 142
-- Fuse Open Wet Stripes into Sculptural Volumes
local b_badger = brush("badger", 24)

-- 1. FUSE PITCHER BELLY STRIPES WET-IN-WET
b_badger:stroke({{350, 420}, {440, 420}}, {pressure = 0.22})
b_badger:stroke({{350, 480}, {450, 480}}, {pressure = 0.22})
b_badger:stroke({{350, 540}, {440, 540}}, {pressure = 0.22})
b_badger:stroke({{355, 590}, {425, 590}}, {pressure = 0.22})
-- Vertical stroke to unify direction
b_badger:stroke({{395, 360}, {395, 600}}, {pressure = 0.18})

-- Restate shoulder glaze impasto catchlight
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}
b_r3:load(p_glaze_touch, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r1:load(p_glaze_touch, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})

-- 2. FUSE QUINCE STRIPES WET-IN-WET INTO SEAMLESS ROUNDNESS
local b_badger_q = brush("badger", 20)
b_badger_q:stroke({{565, 530}, {680, 530}}, {pressure = 0.22})
b_badger_q:stroke({{560, 580}, {690, 580}}, {pressure = 0.22})
b_badger_q:stroke({{565, 630}, {685, 630}}, {pressure = 0.22})
b_badger_q:stroke({{580, 660}, {675, 660}}, {pressure = 0.20})

-- Restate buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_quince_impasto_hi, 1.0)
b_r3:stroke({{568, 568}, {561, 595}, {565, 622}}, {pressure = {0.9, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_impasto_hi, 1.0)
b_r1:touch(562, 593, {pressure = 1.0})

-- Gnarled woody stem arching from crown socket
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(625, 478, {pressure = 0.95})
b_r2:load(p_stem_dry, 1.0)
b_r2:stroke({{625, 478}, {630, 465}, {636, 452}, {644, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_impasto_hi, 0.9)
b_r1:stroke({{626, 476}, {631, 465}, {637, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow under base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(650, 673, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{565, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 0.7}})

print("Stripes fused wet-in-wet into sculptural roundness.")

--@ chunk 143
-- Overlapping Vertical Filbert Sweeps (step = 6 units, brush = 16: no lattice, smooth gradients)
local b_f16 = brush("filbert", 16)

-- 1. PITCHER BELLY: UNIFIED TERRACOTTA VOLUME
for x = 332, 466, 6 do
  local p
  if x < 368 then
    p = p_jug_lit
  elseif x < 416 then
    p = p_jug_body
  elseif x < 452 then
    p = p_jug_core
  else
    p = p_quince_bounce
  end
  b_f16:load(p, 1.0)
  -- Calculate y_top and y_bottom for each vertical stroke from silhouette
  local y_top = (x < 360) and 350 or ((x < 420) and 320 or 350)
  local y_bot = 612
  b_f16:stroke({{x, y_top}, {x, (y_top + y_bot) * 0.5}, {x, y_bot}}, {pressure = 0.98})
end

-- Shoulder glaze catchlight & footring contact
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}
b_r3:load(p_glaze_touch, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r1:load(p_glaze_touch, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.8, 0.8}})

-- 2. QUINCE: UNIFIED RADIANT GOLDEN VOLUME
for x = 554, 706, 6 do
  local p
  if x < 594 then
    p = p_quince_sun
  elseif x < 644 then
    p = p_quince_pure_gold
  elseif x < 686 then
    p = p_quince_shadow
  else
    p = p_quince_bounce
  end
  b_f16:load(p, 1.0)
  -- Calculate y_top and y_bottom for each vertical stroke from quince silhouette
  local y_top, y_bot
  if x < 628 then
    y_top = 480 + (550 - 480) * (628 - x) / (628 - 554)
    y_bot = 674 - (674 - 635) * (628 - x) / (628 - 554)
  else
    y_top = 480 + (550 - 480) * (x - 628) / (706 - 628)
    y_bot = 674 - (674 - 635) * (x - 628) / (706 - 628)
  end
  b_f16:stroke({{x, y_top}, {x, (y_top + y_bot) * 0.5}, {x, y_bot}}, {pressure = 0.98})
end

-- Buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_quince_impasto_hi, 1.0)
b_r3:stroke({{568, 568}, {561, 595}, {565, 622}}, {pressure = {0.9, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_impasto_hi, 1.0)
b_r1:touch(562, 593, {pressure = 1.0})

-- Arching woody stalk & crown socket
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(625, 478, {pressure = 0.95})
b_r2:load(p_stem_dry, 1.0)
b_r2:stroke({{625, 478}, {630, 465}, {636, 452}, {644, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_sun, 0.9)
b_r1:stroke({{626, 476}, {631, 465}, {637, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow under base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(650, 673, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{565, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 0.7}})

print("Dense overlapping vertical filbert passes executed without blender.")

--@ chunk 144
-- 1. CARVE PITCHER BACK TO TRUE SWELLING SILHOUETTE
local b_f18 = brush("filbert", 18)
local b_f16 = brush("filbert", 16)
local b_f14 = brush("filbert", 14)
local b_f12 = brush("filbert", 12)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.4, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Carve wall outside pitcher's right neck and shoulder (cuts away the square box)
local pts_wall_pitcher = {
  {406, 270}, {402, 315}, {408, 345}, {425, 385}, {450, 435}, {472, 485}, {470, 520}
}
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke(pts_wall_pitcher, {pressure = 1.0})
b_f18:load(p_wall_dark, 1.0)
b_f18:stroke({{450, 360}, {500, 360}, {560, 360}}, {pressure = 1.0})
b_f18:load(p_wall_dark, 1.0)
b_f18:stroke({{465, 400}, {510, 400}, {560, 400}}, {pressure = 1.0})

-- Carve table outside pitcher's right lower flank (taper to foot)
local pts_table_pitcher = {
  {470, 520}, {460, 550}, {445, 580}, {430, 600}, {422, 615}
}
b_f14:load(p_stone_mid, 1.0)
b_f14:stroke(pts_table_pitcher, {pressure = 1.0})

-- 2. MODEL PITCHER WITH CURVING CONTOUR BRUSHWORK
-- Belly body tone curving with roundness
b_f18:load(p_jug_body, 1.0)
b_f18:stroke({{345, 320}, {330, 420}, {315, 485}, {320, 545}, {345, 612}}, {pressure = 0.98})
b_f18:load(p_jug_body, 1.0)
b_f18:stroke({{380, 320}, {385, 420}, {385, 500}, {380, 555}, {375, 612}}, {pressure = 0.98})

-- Lit flank on left
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{345, 274}, {340, 340}, {310, 420}, {295, 485}, {305, 545}, {335, 600}, {345, 614}}, {pressure = {0.85, 0.75}})

-- Core shadow curving down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{400, 320}, {425, 385}, {452, 445}, {456, 495}, {442, 550}, {415, 608}}, {pressure = {0.85, 0.75}})
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{430, 395}, {455, 445}, {460, 495}, {448, 545}, {426, 595}, {416, 612}}, {pressure = 0.75})

-- Reflected light along right contour
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{408, 330}, {430, 385}, {460, 440}, {468, 485}, {458, 535}, {440, 580}, {422, 614}}, {pressure = {0.35, 0.4}})

-- Shoulder glaze catchlight & footring contact
b_r3:load(p_glaze_touch, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r1:load(p_glaze_touch, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.8, 0.8}})

-- 3. CARVE QUINCE BACK TO ORGANIC KNOBBY SILHOUETTE
-- Carve wall around crown
local pts_wall_quince = {
  {545, 520}, {558, 505}, {580, 488}, {608, 476}, {628, 474},
  {648, 478}, {672, 495}, {695, 520}
}
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke(pts_wall_quince, {pressure = 1.0})

-- Carve table along quince flanks (cuts away the straight faceted sides)
local pts_table_q_left = {
  {545, 520}, {550, 555}, {546, 595}, {554, 635}, {572, 668}
}
b_f14:load(p_stone_mid, 1.0)
b_f14:stroke(pts_table_q_left, {pressure = 1.0})

local pts_table_q_right = {
  {695, 520}, {708, 555}, {710, 600}, {702, 638}, {682, 666}
}
b_f14:load(p_stone_mid, 1.0)
b_f14:stroke(pts_table_q_right, {pressure = 1.0})

-- 4. MODEL QUINCE WITH CURVING ORGANIC STROKES
-- Golden body curving through lobes
b_f16:load(p_quince_pure_gold, 1.0)
b_f16:stroke({{615, 482}, {595, 530}, {580, 580}, {578, 630}, {598, 672}}, {pressure = 0.98})
b_f16:load(p_quince_pure_gold, 1.0)
b_f16:stroke({{625, 485}, {615, 535}, {608, 585}, {610, 635}, {625, 672}}, {pressure = 0.98})

-- Lit crest on left flank
b_f14:load(p_quince_sun, 1.0)
b_f14:stroke({{608, 482}, {582, 525}, {562, 575}, {556, 615}, {568, 655}, {590, 674}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.8}, swell = {0.85, 1.25, 0.85}
})

-- Core shadow curving down right flank
b_f14:load(p_quince_shadow, 1.0)
b_f14:stroke({{638, 485}, {658, 530}, {672, 580}, {674, 630}, {660, 672}}, {pressure = 0.98})
b_f12:load(p_quince_shadow, 1.0)
b_f12:stroke({{648, 495}, {672, 545}, {686, 595}, {684, 640}, {670, 672}}, {pressure = 0.95})

-- Table bounce on right contour
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{665, 510}, {692, 555}, {704, 600}, {700, 638}, {684, 666}}, {pressure = 0.85})

-- Buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_quince_highlight, 1.0)
b_r3:stroke({{568, 568}, {561, 595}, {565, 622}}, {pressure = {0.9, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_highlight, 1.0)
b_r1:touch(561, 593, {pressure = 1.0})

-- Arching woody stalk & crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(625, 478, {pressure = 0.95})
b_r2:load(p_stem_dry, 1.0)
b_r2:stroke({{625, 478}, {630, 465}, {636, 452}, {644, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_sun, 0.9)
b_r1:stroke({{626, 476}, {631, 465}, {637, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow at base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(650, 673, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{565, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 0.7}})

-- Soft transparent cast shadow of quince
b_f14:load(p_quince_shadow, 0.45)
b_f14:stroke({{665, 678}, {715, 683}, {765, 688}}, {pressure = {0.6, 0.2}})

print("Silhouettes carved back to natural curves, volumes modeled organically.")

--@ chunk 145
-- 1. ERASE THE LIGHT BOARD OUTSIDE PITCHER CONTOUR
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.4, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- A. Wall outside pitcher contour (y = 330 to 520, x = contour to 520)
for y = 330, 520, 10 do
  local xc
  if y < 380 then
    xc = 405 + (435 - 405) * (y - 330) / 50
  elseif y < 440 then
    xc = 435 + (460 - 435) * (y - 380) / 60
  elseif y < 485 then
    xc = 460 + (472 - 460) * (y - 440) / 45
  else
    xc = 472 - (472 - 468) * (y - 485) / 35
  end
  b_f18:load(p_wall_dark, 1.0)
  b_f18:stroke({{xc + 2, y}, {490, y}, {530, y}}, {pressure = 1.0})
end

-- B. Table outside pitcher contour (y = 520 to 615, x = contour to 520)
for y = 520, 615, 8 do
  local xc = 468 - (y - 520) * (468 - 422) / (615 - 520)
  b_f18:load(p_stone_mid, 1.0)
  b_f18:stroke({{xc + 2, y}, {490, y}, {530, y}}, {pressure = 1.0})
end

-- Clean wall edge along pitcher neck and shoulder
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke({{406, 270}, {402, 315}, {408, 345}, {425, 385}, {450, 435}, {472, 485}}, {pressure = 1.0})

-- Clean table edge along pitcher lower flank
b_r2:load(p_stone_mid, 1.0)
b_r2:stroke({{472, 485}, {468, 520}, {456, 550}, {438, 585}, {422, 615}}, {pressure = 1.0})

-- 2. MODEL PITCHER SWELLING RIGHT FLANK AND CORE SHADOW
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{400, 320}, {425, 385}, {452, 445}, {456, 495}, {442, 550}, {415, 608}}, {pressure = 0.98})
local b_f10 = brush("filbert", 10)
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{430, 395}, {455, 445}, {460, 495}, {448, 545}, {426, 595}, {416, 612}}, {pressure = 0.75})

-- Reflected light along right contour
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{408, 330}, {430, 385}, {460, 440}, {468, 485}, {458, 535}, {440, 580}, {422, 614}}, {pressure = {0.35, 0.4}})

-- Shoulder glaze catchlight & footring contact
b_r3:load(p_glaze_touch, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r1:load(p_glaze_touch, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.8, 0.8}})

-- Pitcher cast shadow flowing to the right
b_f18:load(p_quince_shadow, 0.5)
b_f18:stroke({{422, 617}, {465, 622}, {515, 630}}, {pressure = {0.8, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 3. RESTORE QUINCE CROWN (Erase brown arch with golden body color)
local b_f16 = brush("filbert", 16)
b_f16:load(p_quince_pure_gold, 1.0)
b_f16:stroke({{610, 478}, {590, 505}, {580, 545}}, {pressure = 1.0})
b_f16:load(p_quince_pure_gold, 1.0)
b_f16:stroke({{625, 480}, {620, 515}, {615, 555}}, {pressure = 1.0})
b_f16:load(p_quince_pure_gold, 1.0)
b_f16:stroke({{638, 482}, {645, 515}, {650, 555}}, {pressure = 1.0})

-- Clean wall outside quince crown (above y = 478)
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{570, 470}, {610, 465}, {655, 470}}, {pressure = 1.0})
b_r2:load(p_wall_dark, 1.0)
b_r2:stroke({{575, 482}, {605, 472}, {628, 470}, {650, 474}, {670, 486}}, {pressure = 1.0})

-- Sunlit crest on left flank
b_f14:load(p_quince_sun, 1.0)
b_f14:stroke({{608, 482}, {582, 525}, {562, 575}, {556, 615}, {568, 655}, {590, 674}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.8}, swell = {0.85, 1.25, 0.85}
})

-- Core shadow down right flank
b_f14:load(p_quince_shadow, 1.0)
b_f14:stroke({{638, 485}, {658, 530}, {672, 580}, {674, 630}, {660, 672}}, {pressure = 0.98})
b_f14:load(p_quince_shadow, 1.0)
b_f14:stroke({{648, 495}, {672, 545}, {686, 595}, {684, 640}, {670, 672}}, {pressure = 0.95})

-- Table bounce on right contour
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{665, 510}, {692, 555}, {704, 600}, {700, 638}, {684, 666}}, {pressure = 0.85})

-- Buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_quince_highlight, 1.0)
b_r3:stroke({{568, 568}, {561, 595}, {565, 622}}, {pressure = {0.9, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_highlight, 1.0)
b_r1:touch(561, 593, {pressure = 1.0})

-- Arching woody stalk & crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(625, 478, {pressure = 0.95})
b_r2:load(p_stem_dry, 1.0)
b_r2:stroke({{625, 478}, {630, 465}, {636, 452}, {644, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_sun, 0.9)
b_r1:stroke({{626, 476}, {631, 465}, {637, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow at base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(650, 673, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{565, 675}, {615, 677}, {660, 676}, {686, 672}}, {pressure = {0.8, 0.7}})

print("Artifacts completely erased, noble silhouettes and volumes restored.")

--@ chunk 146
-- Precision Refinement of the Quince, Contour Integration, and Knife
local b_f14 = brush("filbert", 14)
local b_f12 = brush("filbert", 12)
local b_f10 = brush("filbert", 10)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. UNIFY WALL AND TABLE AROUND QUINCE SILHOUETTE
-- Wall outside crown left (x = 520 to 575, y = 470 to 520)
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{520, 480}, {550, 480}, {575, 480}}, {pressure = 1.0})
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{520, 505}, {545, 505}, {565, 505}}, {pressure = 1.0})

-- Wall outside crown right (x = 665 to 730, y = 470 to 520)
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{665, 480}, {695, 480}, {730, 480}}, {pressure = 1.0})
b_f14:load(p_wall_dark, 1.0)
b_f14:stroke({{675, 505}, {700, 505}, {730, 505}}, {pressure = 1.0})

-- Table outside left flank (x = 530 to 546, y = 520 to 670)
b_f12:load(p_stone_lit, 1.0)
b_f12:stroke({{535, 530}, {540, 590}, {545, 650}}, {pressure = 0.98})

-- Table outside right flank (x = 706 to 725, y = 520 to 670)
b_f12:load(p_stone_mid, 1.0)
b_f12:stroke({{715, 530}, {712, 590}, {708, 650}}, {pressure = 0.98})

-- Clean table under quince shadow (between quince base and knife blade)
b_f12:load(p_stone_lit, 1.0)
b_f12:stroke({{675, 678}, {720, 680}, {760, 682}}, {pressure = 0.95})

-- 2. RESTORE QUINCE CROWN AND BASE (Erase dark marks inside fruit)
-- Crown fill (y = 475 to 525, x = 575 to 665)
b_f14:load(p_quince_pure_gold, 1.0)
b_f14:stroke({{595, 482}, {590, 510}, {585, 540}}, {pressure = 1.0})
b_f14:load(p_quince_pure_gold, 1.0)
b_f14:stroke({{620, 476}, {618, 510}, {615, 545}}, {pressure = 1.0})
b_f14:load(p_quince_pure_gold, 1.0)
b_f14:stroke({{645, 480}, {642, 510}, {640, 545}}, {pressure = 1.0})

-- Sunlit crest on crown left
b_f12:load(p_quince_sun, 1.0)
b_f12:stroke({{585, 485}, {578, 515}, {568, 550}}, {pressure = 0.98})

-- Base fill down to y = 672 (eliminating rounded bristle ends)
b_f14:load(p_quince_pure_gold, 1.0)
b_f14:stroke({{580, 630}, {585, 655}, {595, 672}}, {pressure = 1.0})
b_f14:load(p_quince_pure_gold, 1.0)
b_f14:stroke({{610, 630}, {615, 655}, {625, 672}}, {pressure = 1.0})
b_f14:load(p_quince_shadow, 1.0)
b_f14:stroke({{645, 630}, {650, 655}, {655, 672}}, {pressure = 1.0})

-- Dark contact shadow under quince base (y = 672)
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{568, 672}, {615, 674}, {655, 673}, {678, 671}}, {pressure = {0.85, 0.75}})

-- 3. BUTTERY IMPASTO CREST HIGHLIGHT
b_r3:load(p_quince_highlight, 1.0)
b_r3:stroke({{566, 568}, {558, 595}, {562, 622}}, {pressure = {0.95, 0.8}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_highlight, 1.0)
b_r1:touch(559, 593, {pressure = 1.0})

-- Secondary crest glint on upper shoulder
b_r2:load(p_quince_highlight, 0.9)
b_r2:stroke({{595, 492}, {588, 510}}, {pressure = 0.85})

-- 4. GNARLED WOODY STEM & CROWN SOCKET
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(622, 476, {pressure = 0.95})
b_r2:load(p_stem_dry, 1.0)
b_r2:stroke({{622, 475}, {628, 462}, {635, 450}, {644, 442}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_sun, 0.9)
b_r1:stroke({{623, 473}, {629, 462}, {636, 452}}, {pressure = 0.6})

-- 5. BODEGÓN KNIFE REFINEMENT
-- Silvery steel blade cutting edge
b_r1:load(p_knife_edge, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {785, 715}}, {pressure = 1.0})

-- Brass bolster glint
b_r1:load(p_brass_glint, 1.0)
b_r1:touch(791, 715, {pressure = 1.0})

-- Razor chamfer along front rim (y = 715)
b_r1:load(p_glaze_touch, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {784, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})
b_r1:stroke({{799, 715}, {900, 715}, {1005, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})

print("Quince crown, base, and contours perfected; knife and ledge sharpened.")

--@ chunk 147
-- Precision Quince Restoration via Rim Fringe and Clipped Fill
local pts_quince = {
  {625, 476}, {645, 482}, {670, 506}, {695, 545}, {708, 595},
  {704, 636}, {684, 664}, {660, 672}, {630, 674}, {595, 672},
  {572, 665}, {554, 640}, {546, 605}, {554, 560}, {575, 520},
  {598, 492}, {616, 478}
}
local m_q = poly(pts_quince, true)

-- 1. ERASE WHISKERS OUTSIDE QUINCE VIA NARROW RIM MASKS (No rectangular boxes!)
local m_fringe = m_q:rim(16)
local m_fringe_wall = m_fringe * mask(function(x, y) return y <= 520 and 1 or 0 end)
local m_fringe_table = m_fringe * mask(function(x, y) return y > 520 and 1 or 0 end)

work(m_fringe_wall, {
  hand = "detail",
  pile = p_wall_dark,
  clip = true,
  coverage = 3.5,
  fill = true
})

work(m_fringe_table, {
  hand = "detail",
  pile = p_stone_lit,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- 2. SOLID OPAQUE GOLDEN BODY COAT INSIDE M_Q
work(m_q, {
  hand = "body",
  pile = p_quince_pure_gold,
  angle = 1.4,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- Shaded flank on right via soft clipped mask
local m_q_shade = (m_q * mask(function(x, y) return x > 635 and 1 or 0 end)):blur(15)
work(m_q_shade, {
  hand = "body",
  pile = p_quince_shadow,
  angle = 1.4,
  clip = m_q,
  coverage = 2.0
})

-- Sunlit flank on left via soft clipped mask
local m_q_lit = (m_q * mask(function(x, y) return x < 615 and 1 or 0 end)):blur(15)
work(m_q_lit, {
  hand = "body",
  pile = p_quince_sun,
  angle = 1.4,
  clip = m_q,
  coverage = 2.0
})

-- Warm table bounce on right contour
local m_q_bounce = (m_q * mask(function(x, y) return x > 680 and y > 530 and 1 or 0 end)):blur(8)
work(m_q_bounce, {
  hand = "detail",
  pile = p_quince_bounce,
  clip = m_q,
  coverage = 1.2
})

-- Blend inside quince to fuse into smooth, voluptuous mass
blend(m_q, {angle = 1.35, coverage = 1.0})

-- 3. BUTTERY IMPASTO HIGHLIGHT & DETAILS
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}

b_r3:load(p_quince_highlight, 1.0)
b_r3:stroke({{568, 568}, {561, 595}, {565, 622}}, {pressure = {0.9, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_highlight, 1.0)
b_r1:touch(561, 593, {pressure = 1.0})

-- Arching woody stalk & crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(625, 478, {pressure = 0.95})
b_r2:load(p_stem_dry, 1.0)
b_r2:stroke({{625, 478}, {630, 465}, {636, 452}, {644, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_sun, 0.9)
b_r1:stroke({{626, 476}, {631, 465}, {637, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow under base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(650, 673, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{565, 674}, {615, 675}, {660, 674}, {684, 671}}, {pressure = {0.85, 0.75}})

-- Soft transparent cast shadow of quince
local b_f14 = brush("filbert", 14)
b_f14:load(p_quince_shadow, 0.45)
b_f14:stroke({{665, 678}, {715, 683}, {765, 688}}, {pressure = {0.6, 0.2}})

print("Quince completely restored with rim fringe cleanup and clipped volumetric fill.")

--@ chunk 148
-- Dissolve Fringe Bands into Seamless Wall and Table
local b_f24 = brush("filbert", 24)
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_badger = brush("badger", 32)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. UNIFY WALL ABOVE TABLE (y = 440 to 520)
-- Sweep wall paint horizontally to erase all crown fringe marks
for y = 450, 515, 15 do
  b_f24:load(p_wall_dark, 1.0)
  b_f24:stroke({{450, y}, {530, y}, {610, y}}, {pressure = 0.98})
  b_f24:load(p_wall_dark, 1.0)
  b_f24:stroke({{645, y}, {730, y}, {850, y}}, {pressure = 0.98})
end

-- Horizontal badger sweeps over the wall
b_badger:stroke({{450, 480}, {610, 480}}, {pressure = 0.22})
b_badger:stroke({{645, 480}, {850, 480}}, {pressure = 0.22})

-- 2. UNIFY TABLETOP (y = 520 to 715)
-- Left of quince (x = 420 to 552, y = 525 to 670)
for y = 525, 665, 12 do
  b_f18:load(p_stone_lit, 1.0)
  b_f18:stroke({{420, y}, {490, y}, {552, y}}, {pressure = 0.98})
end

-- Right of quince (x = 702 to 1010, y = 525 to 670)
for y = 525, 665, 12 do
  b_f24:load(p_stone_lit, 1.0)
  b_f24:stroke({{702, y}, {850, y}, {1010, y}}, {pressure = 0.98})
end

-- Foreground in front of quince (x = 420 to 1010, y = 675 to 715)
for y = 675, 715, 10 do
  b_f24:load(p_stone_lit, 1.0)
  b_f24:stroke({{420, y}, {700, y}, {1010, y}}, {pressure = 0.98})
end

-- Horizontal badger sweeps across the table to fuse stone completely
b_badger:stroke({{420, 560}, {550, 560}}, {pressure = 0.22})
b_badger:stroke({{420, 620}, {550, 620}}, {pressure = 0.22})
b_badger:stroke({{705, 560}, {1000, 560}}, {pressure = 0.22})
b_badger:stroke({{705, 620}, {1000, 620}}, {pressure = 0.22})
b_badger:stroke({{420, 695}, {1010, 695}}, {pressure = 0.28})

-- 3. RE-ANCHOR OBJECT DETAILS
-- Pitcher cast shadow flowing rightward
b_f14:load(p_quince_glaze_shadow, 0.5)
b_f14:stroke({{422, 617}, {465, 622}, {515, 630}}, {pressure = {0.8, 0.2}, swell = {1.0, 1.3, 0.6}})

-- Quince dark contact shadow under base
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{565, 674}, {615, 675}, {660, 674}, {684, 671}}, {pressure = {0.85, 0.75}})

-- Quince transparent cast shadow flowing to the right
b_f14:load(p_quince_glaze_shadow, 0.45)
b_f14:stroke({{665, 678}, {715, 683}, {765, 688}}, {pressure = {0.6, 0.2}})

-- Woody stem arching from crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(625, 478, {pressure = 0.95})
b_r2:load(p_stem_dry, 1.0)
b_r2:stroke({{625, 478}, {630, 465}, {636, 452}, {644, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_glaze_touch, 0.9)
b_r1:stroke({{626, 476}, {631, 465}, {637, 454}}, {pressure = 0.6})

-- Buttery impasto highlight on quince crest
b_r3:load(p_quince_impasto_hi, 1.0)
b_r3:stroke({{568, 568}, {561, 595}, {565, 622}}, {pressure = {0.9, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_impasto_hi, 1.0)
b_r1:touch(561, 593, {pressure = 1.0})

-- Knife blade cutting edge & bolster
b_r1:load(p_knife_edge, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {785, 715}}, {pressure = 1.0})
b_r1:load(p_brass_glint, 1.0)
b_r1:touch(791, 715, {pressure = 1.0})

-- Razor chamfer along front rim (y = 715)
b_r1:load(p_glaze_touch, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {784, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})
b_r1:stroke({{799, 715}, {900, 715}, {1005, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})

print("Fringe bands dissolved, wall and table unified, still life anchored.")

--@ chunk 149
-- Step 1: Flawless Continuous Background Wall and Limestone Tabletop
local b_f26 = brush("filbert", 26)
local b_f24 = brush("filbert", 24)
local b_f18 = brush("filbert", 18)
local b_badger = brush("badger", 36)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. UNIFIED DARK BACKGROUND WALL (y = 440 to 520, x = 405 to 1010)
for y = 440, 520, 12 do
  b_f24:load(p_wall_dark, 1.0)
  b_f24:stroke({{405, y}, {710, y}, {1010, y}}, {pressure = 0.98})
end
b_badger:stroke({{405, 480}, {710, 480}, {1010, 480}}, {pressure = 0.22})

-- 2. UNIFIED LIMESTONE TABLETOP (y = 520 to 715)
-- Mid-ground right of pitcher (y = 520 to 615, x = 420 to 1010)
for y = 520, 615, 8 do
  local p = (y < 555) and p_stone_mid or p_stone_lit
  b_f26:load(p, 1.0)
  b_f26:stroke({{420, y}, {715, y}, {1010, y}}, {pressure = 0.98})
end

-- Entire foreground from edge to edge (y = 615 to 715, x = -10 to 1010)
for y = 615, 715, 8 do
  b_f26:load(p_stone_lit, 1.0)
  b_f26:stroke({{-10, y}, {350, y}, {700, y}, {1010, y}}, {pressure = 0.98})
end

-- Horizontal badger sweeps across the table plane
b_badger:stroke({{-10, 560}, {500, 560}, {1010, 560}}, {pressure = 0.25})
b_badger:stroke({{-10, 630}, {500, 630}, {1010, 630}}, {pressure = 0.28})
b_badger:stroke({{-10, 680}, {500, 680}, {1010, 680}}, {pressure = 0.30})
b_badger:stroke({{-10, 705}, {500, 705}, {1010, 705}}, {pressure = 0.30})

-- 3. RE-ANCHOR PITCHER CONTACT AND CAST SHADOW
-- Footring contact shadow
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.85, 0.85}})

-- Pitcher cast shadow flowing rightward across the stone
local b_f14 = brush("filbert", 14)
b_f14:load(p_quince_glaze_shadow, 0.6)
b_f14:stroke({{422, 617}, {470, 622}, {525, 630}}, {pressure = {0.85, 0.2}, swell = {1.0, 1.3, 0.6}})

-- Crisp razor chamfer line along y = 715 from edge to edge
b_r1:load(p_glaze_touch, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {700, 715}, {1005, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})

print("Stage completely pristine: wall, tabletop, and pitcher unified.")

--@ chunk 150
-- Step 2: Pitcher Belly Restoration, Golden Quince, and Bodegón Knife
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_f8  = brush("filbert", 8)
local b_r5  = brush{kind = "round", width = 5.2, point = 0.9}
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. RESTORE PITCHER'S SWELLING RIGHT BELLY AND NOBLE CURVATURE
-- Curving sweep down the right flank (x = 410 through 472 down to 422)
b_f18:load(p_jug_body, 1.0)
b_f18:stroke({{410, 360}, {440, 420}, {465, 475}, {472, 495}, {455, 550}, {422, 615}}, {pressure = 1.0})

-- Core shadow turning down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{425, 380}, {450, 435}, {468, 485}, {460, 535}, {438, 580}, {420, 612}}, {pressure = 0.98})
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{435, 410}, {458, 460}, {460, 505}, {448, 550}, {426, 598}, {416, 612}}, {pressure = 0.8})

-- Table bounce on right contour
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{430, 385}, {460, 440}, {470, 485}, {460, 535}, {442, 580}, {422, 614}}, {pressure = {0.35, 0.4}})

-- Shoulder glaze catchlight & footring contact
b_r3:load(p_glaze_touch, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r1:load(p_glaze_touch, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.85, 0.85}})

-- Pitcher cast shadow flowing rightward across the stone
b_f14:load(p_quince_glaze_shadow, 0.6)
b_f14:stroke({{422, 617}, {470, 622}, {525, 630}}, {pressure = {0.85, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 2. SOLID SCULPTURAL GOLDEN QUINCE (Clipped to exact silhouette)
local pts_quince = {
  {625, 476}, {645, 482}, {670, 506}, {695, 545}, {708, 595},
  {704, 636}, {684, 664}, {660, 672}, {630, 674}, {595, 672},
  {572, 665}, {554, 640}, {546, 605}, {554, 560}, {575, 520},
  {598, 492}, {616, 478}
}
local m_q = poly(pts_quince, true)

-- Dense golden body fill with high lead-white/chrome opacity
work(m_q, {
  hand = "body",
  pile = p_quince_pure_gold,
  angle = 1.4,
  clip = true,
  coverage = 3.5,
  fill = true
})

-- Warm core shadow flank on right (raw umber + red earth + ochre)
local m_q_shade = (m_q * mask(function(x, y) return x > 630 and 1 or 0 end)):blur(15)
work(m_q_shade, {
  hand = "body",
  pile = p_quince_shadow,
  angle = 1.4,
  clip = m_q,
  coverage = 2.0
})

-- Radiant sunlit flank on left
local m_q_lit = (m_q * mask(function(x, y) return x < 615 and 1 or 0 end)):blur(15)
work(m_q_lit, {
  hand = "body",
  pile = p_quince_sun,
  angle = 1.4,
  clip = m_q,
  coverage = 2.0
})

-- Warm table bounce on right contour
local m_q_bounce = (m_q * mask(function(x, y) return x > 680 and y > 530 and 1 or 0 end)):blur(8)
work(m_q_bounce, {
  hand = "detail",
  pile = p_quince_bounce,
  clip = m_q,
  coverage = 1.2
})

-- Internal blend to fuse into voluptuous, fleshy roundness
blend(m_q, {angle = 1.35, coverage = 1.0})

-- Buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_quince_highlight, 1.0)
b_r3:stroke({{568, 568}, {561, 595}, {565, 622}}, {pressure = {0.9, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_highlight, 1.0)
b_r1:touch(561, 593, {pressure = 1.0})

-- Arching woody stalk & crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(625, 478, {pressure = 0.95})
b_r2:load(p_stem_dry, 1.0)
b_r2:stroke({{625, 478}, {630, 465}, {636, 452}, {644, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_sun, 0.9)
b_r1:stroke({{626, 476}, {631, 465}, {637, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow at base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(650, 673, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{565, 674}, {615, 675}, {660, 674}, {684, 671}}, {pressure = {0.85, 0.75}})

-- Soft transparent cast shadow of quince flowing to the right
b_f14:load(p_quince_shadow, 0.45)
b_f14:stroke({{665, 678}, {715, 683}, {765, 688}}, {pressure = {0.6, 0.2}})

-- 3. THE BODEGÓN KNIFE (Steel blade, brass bolster, turned handle)
-- Steel blade flat
b_f8:load(p_steel_mid, 1.0)
b_f8:stroke({{675, 686}, {730, 700}, {786, 714}}, {pressure = 0.98})

-- Dark steel spine
b_r1:load(p_jug_deep, 0.95)
b_r1:stroke({{675, 686}, {730, 698}, {786, 712}}, {pressure = 0.92})

-- Razor-sharp specular cutting edge
b_r1:load(p_knife_edge, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {786, 715}}, {pressure = 1.0})

-- Brass bolster bridging blade to handle ({784, 713} to {799, 718})
b_r2:load(p_brass, 1.0)
b_r2:stroke({{784, 713}, {792, 716}, {799, 718}}, {pressure = 1.0})
b_r1:load(p_brass_glint, 1.0)
b_r1:touch(791, 715, {pressure = 1.0})
b_r1:load(p_jug_deep, 0.9)
b_r1:stroke({{784, 716}, {799, 720}}, {pressure = 0.85})

-- Turned handle projecting over front ledge ({797, 718} to {865, 746})
b_r5:load(p_jug_deep, 1.0)
b_r5:stroke({{797, 718}, {832, 732}, {865, 746}}, {pressure = 0.98, swell = {0.9, 1.15, 0.95}})
local b_r3_h = brush{kind = "round", width = 2.8, point = 0.92}
b_r3_h:load(p_jug_body, 1.0)
b_r3_h:stroke({{798, 716}, {832, 730}, {864, 744}}, {pressure = 0.9})
b_r1:load(p_brass_glint, 0.9)
b_r1:touch(865, 746, {pressure = 0.95})

-- Vertical handle shadow down ledge face
b_r3:load(p_jug_deep, 0.85)
b_r3:stroke({{802, 716}, {805, 735}, {808, 755}}, {pressure = {0.85, 0.2}})

-- Blade contact shadow on limestone table
b_r2:load(p_quince_shadow, 0.75)
b_r2:stroke({{678, 688}, {730, 702}, {784, 716}}, {pressure = {0.5, 0.8}})

-- 4. RAZOR CHAMFER HIGHLIGHT (y = 715)
b_r1:load(p_glaze_touch, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {784, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})
b_r1:stroke({{799, 715}, {900, 715}, {1005, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})

print("Still life composition fully consolidated: pitcher, quince, knife, and chamfer complete.")

--@ chunk 151
-- 1. UNIFY TABLETOP LEFT OF PITCHER (y = 520 to 570, x = -10 to 265)
local b_f20 = brush("filbert", 20)
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_badger = brush("badger", 32)
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

for y = 522, 568, 8 do
  local p = (y < 545) and p_stone_mid or p_stone_lit
  b_f20:load(p, 1.0)
  b_f20:stroke({{-10, y}, {120, y}, {265, y}}, {pressure = 0.98})
end
b_badger:stroke({{-10, 545}, {265, 545}}, {pressure = 0.25})

-- 2. SOLID TERRACOTTA FILL INSIDE PITCHER RIGHT BELLY (Buries the gap completely)
for y = 410, 610, 8 do
  local xl = 395
  local xr
  if y < 485 then
    xr = 450 + (472 - 450) * (y - 410) / 75
  elseif y < 550 then
    xr = 472 - (472 - 456) * (y - 485) / 65
  else
    xr = 456 - (456 - 422) * (y - 550) / 60
  end
  b_f18:load(p_jug_body, 1.0)
  b_f18:stroke({{xl, y}, {(xl + xr) * 0.5, y}, {xr, y}}, {pressure = 0.98})
end

-- Core shadow down the right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{425, 380}, {450, 440}, {462, 495}, {455, 545}, {435, 585}, {418, 612}}, {pressure = 0.95})
b_f14:load(p_jug_deep, 0.95)
b_f14:stroke({{440, 420}, {460, 470}, {465, 505}, {450, 555}, {428, 598}, {416, 612}}, {pressure = 0.8})

-- Table bounce along right contour
local b_f10 = brush("filbert", 10)
b_f10:load(p_quince_bounce, 0.95)
b_f10:stroke({{430, 385}, {460, 440}, {470, 485}, {460, 535}, {442, 580}, {422, 614}}, {pressure = {0.35, 0.4}})

-- Soft vertical badger blend down pitcher belly
local b_badger_jug = brush("badger", 22)
b_badger_jug:stroke({{385, 380}, {390, 480}, {385, 600}}, {pressure = 0.20})
b_badger_jug:stroke({{430, 400}, {445, 480}, {425, 600}}, {pressure = 0.20})

-- Shoulder glaze catchlight & footring contact
b_r3:load(p_glaze_touch, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r1:load(p_glaze_touch, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{320, 617}, {370, 619}, {425, 617}}, {pressure = {0.85, 0.85}})

-- Pitcher cast shadow flowing rightward across the stone
b_f14:load(p_quince_glaze_shadow, 0.55)
b_f14:stroke({{422, 617}, {470, 622}, {525, 630}}, {pressure = {0.85, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 3. QUINCE: BUTTERY IMPASTO CREST HIGHLIGHT & VELVETY ORCHARD BLUSH
local b_stip = brush("stippler", 6)
b_stip:load(p_quince_pure_gold, 0.5)
b_stip:stroke({{570, 560}, {585, 595}, {605, 630}}, {pressure = 0.35, shake = 0.3})

b_r3:load(p_quince_impasto_hi, 1.0)
b_r3:stroke({{568, 568}, {561, 595}, {565, 622}}, {pressure = {0.9, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_impasto_hi, 1.0)
b_r1:touch(562, 593, {pressure = 1.0})

-- Woody stem & crown socket
b_r2:load(p_jug_deep, 1.0)
b_r2:touch(625, 478, {pressure = 0.95})
b_r2:load(p_stem_dry, 1.0)
b_r2:stroke({{625, 478}, {630, 465}, {636, 452}, {644, 444}}, {pressure = {0.98, 0.5}})
b_r1:load(p_quince_sun, 0.9)
b_r1:stroke({{626, 476}, {631, 465}, {637, 454}}, {pressure = 0.6})

-- Calyx eye & dark contact shadow at base
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(650, 673, {pressure = 0.95})
b_r3:load(p_jug_deep, 1.0)
b_r3:stroke({{565, 674}, {615, 675}, {660, 674}, {684, 671}}, {pressure = {0.85, 0.75}})

-- 4. BODEGÓN KNIFE REFINEMENT
local b_f8_k = brush("filbert", 7)
b_f8_k:load(p_steel_mid, 1.0)
b_f8_k:stroke({{675, 686}, {730, 700}, {786, 714}}, {pressure = 0.98})

-- Razor cutting edge
b_r1:load(p_knife_edge, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {786, 715}}, {pressure = 1.0})

-- Brass bolster bridging blade to handle
b_r2:load(p_brass, 1.0)
b_r2:stroke({{784, 713}, {792, 716}, {799, 718}}, {pressure = 1.0})
b_r1:load(p_brass_glint, 1.0)
b_r1:touch(791, 715, {pressure = 1.0})
b_r1:load(p_jug_deep, 0.9)
b_r1:stroke({{784, 716}, {799, 720}}, {pressure = 0.85})

-- Razor chamfer along front rim (y = 715)
b_r1:load(p_glaze_touch, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {784, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})
b_r1:stroke({{799, 715}, {900, 715}, {1005, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})

print("Pitcher belly filled solid, left table unified, knife and quince perfected.")

--@ chunk 152
-- Final Master Touches: Impastos, Glaze Catchlights, and Specular Accents
local b_r3 = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. PITCHER SPECULAR GLAZE HIGHLIGHTS
-- North window highlight on glazed terracotta shoulder curve
b_r3:load(p_glaze_touch, 1.0)
b_r3:stroke({{325, 396}, {322, 415}, {318, 435}}, {pressure = {0.6, 0.5}, swell = {0.8, 1.3, 0.7}})
b_r1:load(p_glaze_touch, 1.0)
b_r1:touch(322, 415, {pressure = 1.0})

-- Specular glint on left flared lip of mouth
b_r1:load(p_glaze_touch, 1.0)
b_r1:touch(348, 272, {pressure = 1.0})

-- Specular glint on loop handle crest
b_r1:load(p_glaze_touch, 1.0)
b_r1:stroke({{242, 398}, {240, 414}, {243, 428}}, {pressure = {0.4, 0.95, 0.4}})

-- 2. QUINCE TACTILE IMPASTO CREST & CALYX ACCENTS
-- Luscious buttery lead-white & chrome-yellow impasto on sunlit lobe
b_r3:load(p_quince_impasto_hi, 1.0)
b_r3:stroke({{568, 568}, {561, 595}, {565, 622}}, {pressure = {0.9, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_quince_impasto_hi, 1.0)
b_r1:touch(562, 593, {pressure = 1.0})

-- Secondary glint on upper shoulder
b_r2:load(p_quince_impasto_hi, 0.9)
b_r2:stroke({{600, 498}, {594, 514}}, {pressure = 0.85})

-- Calyx eye touch
b_r1:load(p_jug_deep, 1.0)
b_r1:touch(650, 673, {pressure = 0.95})

-- 3. BODEGÓN KNIFE SPECULAR HIGHLIGHTS
-- Razor cutting edge along forged steel blade
b_r1:load(p_knife_edge, 1.0)
b_r1:stroke({{675, 686}, {730, 702}, {785, 715}}, {pressure = 1.0})

-- Specular glint on brass bolster collar
b_r1:load(p_brass_glint, 1.0)
b_r1:touch(791, 715, {pressure = 1.0})

-- Glint on turned pommel
b_r1:load(p_brass_glint, 0.95)
b_r1:touch(865, 746, {pressure = 0.95})

-- 4. UNBROKEN RAZOR CHAMFER HIGHLIGHT (y = 715)
-- Left of knife
b_r1:load(p_glaze_touch, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {784, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})
-- Right of knife
b_r1:load(p_glaze_touch, 1.0)
b_r1:stroke({{799, 715}, {900, 715}, {1005, 715}}, {pressure = {0.8, 0.8}, shake = 0.04})

print("Final master touches complete.")

--@ chunk 153
print("current time:", wait(0))
print("table drying:", drying(500, 600))
print("jug base drying:", drying(350, 610))
print("quince drying:", drying(620, 580))
print("knife drying:", drying(750, 700))

--@ chunk 154
print("Advancing clock by 14 days...")
local t = wait(14 * 24 * 60)
print("New time:", t)
print("table drying:", drying(500, 600))
print("jug drying:", drying(350, 450))
print("quince drying:", drying(620, 580))
print("knife drying:", drying(750, 700))
print("bottom ledge drying:", drying(500, 750))

--@ chunk 155
-- STEP 1: RESTORE ARCHITECTURAL FOUNDATION (LEDGE AND LIMESTONE TABLETOP)

-- Define stage palette piles
p_ledge = pile{{"bone black", 5.5}, {"raw umber", 4.0}, {"red earth", 1.2}, {"yellow ochre", 0.5}, medium = 0.05}
p_stone_base = pile{{"lead white", 6.8}, {"yellow ochre", 2.8}, {"raw umber", 1.2}, {"red earth", 0.3}, medium = 0.08}
p_stone_lit  = pile{{"lead white", 8.2}, {"yellow ochre", 2.0}, {"raw umber", 0.5}, medium = 0.08}
p_stone_dark = pile{{"lead white", 5.0}, {"yellow ochre", 3.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium = 0.08}

local b_f26 = brush("filbert", 26)
local b_f20 = brush("filbert", 20)
local b_f14 = brush("filbert", 14)
local b_badger = brush("badger", 36)
local b_r2 = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1 = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. SOLID FRONT LEDGE FACE (y = 715 to 805)
-- Cover the entire vertical masonry face edge to edge, especially bottom corners
for y = 716, 804, 12 do
  b_f26:load(p_ledge, 1.0)
  b_f26:stroke({{-10, y}, {300, y}, {700, y}, {1010, y}}, {pressure = 1.0})
end
-- Badger pass across the vertical ledge to blend into smooth dressed stone
b_badger:stroke({{-10, 755}, {500, 755}, {1010, 755}}, {pressure = 0.25})

-- 2. LIMESTONE TABLETOP PLANE (y = 520 to 715)
-- Left wing of the table (x = -10 to 260)
for y = 522, 630, 10 do
  local p = (y < 560) and p_stone_base or p_stone_lit
  b_f20:load(p, 1.0)
  b_f20:stroke({{-10, y}, {130, y}, {265, y}}, {pressure = 0.98})
end

-- Middle table between pitcher and quince (x = 440 to 555)
for y = 522, 630, 10 do
  local p = (y < 560) and p_stone_base or p_stone_lit
  b_f20:load(p, 1.0)
  b_f20:stroke({{440, y}, {495, y}, {555, y}}, {pressure = 0.98})
end

-- Right wing of the table (x = 705 to 1010)
for y = 522, 630, 10 do
  local p = (y < 560) and p_stone_base or p_stone_lit
  b_f20:load(p, 1.0)
  b_f20:stroke({{705, y}, {850, y}, {1010, y}}, {pressure = 0.98})
end

-- Entire foreground slab in front of objects (y = 625 to 715, x = -10 to 1010)
-- Sweeping horizontally across the full width completely erases the floating brown smear,
-- resets the knife area, and cleans the stone under the pitcher base
for y = 628, 714, 8 do
  local p = (y < 665) and p_stone_base or p_stone_lit
  b_f26:load(p, 1.0)
  b_f26:stroke({{-10, y}, {350, y}, {700, y}, {1010, y}}, {pressure = 1.0})
end

-- Soft badger sweeps horizontally across the open table plane
b_badger:stroke({{-10, 640}, {500, 640}, {1010, 640}}, {pressure = 0.22})
b_badger:stroke({{-10, 675}, {500, 675}, {1010, 675}}, {pressure = 0.22})
b_badger:stroke({{-10, 705}, {500, 705}, {1010, 705}}, {pressure = 0.22})

-- 3. CHISELED CHAMFER HIGHLIGHT ALONG FRONT RIM (y = 715)
local p_chamfer = pile{{"lead white", 8.8}, {"yellow ochre", 1.8}, medium = 0.05}
b_r1:load(p_chamfer, 0.95)
b_r1:stroke({{-5, 715}, {250, 715}, {600, 715}, {850, 715}, {1005, 715}}, {pressure = {0.85, 0.9, 0.85}, shake = 0.03})

print("Architectural foundation consolidated: ledge, table plane, and chamfer clean.")

--@ chunk 156
-- CHUNK 156: UNIFY ARCHITECTURAL STAGE (NO NOTCHES, SEAMLESS WALL & TABLE)

-- Palette piles for the stage
p_ledge = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.2}, medium = 0.05}
p_wall_dark = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"yellow ochre", 0.8}, medium = 0.06}
p_wall_soft = pile{{"raw umber", 5.0}, {"bone black", 3.0}, {"yellow ochre", 1.5}, {"lead white", 0.5}, medium = 0.08}
p_stone_back = pile{{"lead white", 6.0}, {"yellow ochre", 3.2}, {"raw umber", 1.8}, {"red earth", 0.4}, medium = 0.08}
p_stone_lit  = pile{{"lead white", 8.2}, {"yellow ochre", 2.2}, {"raw umber", 0.6}, medium = 0.08}

local b_f26 = brush("filbert", 26)
local b_f20 = brush("filbert", 20)
local b_badger = brush("badger", 36)

-- 1. FRONT LEDGE: Solid, dense masonry face covering full bottom edge and corners
for y = 716, 805, 10 do
  b_f26:load(p_ledge, 1.0)
  b_f26:stroke({{-15, y}, {300, y}, {700, y}, {1020, y}}, {pressure = 1.0})
end
b_badger:stroke({{-15, 760}, {500, 760}, {1020, 760}}, {pressure = 0.25})

-- 2. BACKGROUND WALL HORIZON (y = 440 to 522, x = 415 to 1020)
for y = 440, 520, 10 do
  b_f26:load(p_wall_dark, 1.0)
  b_f26:stroke({{415, y}, {700, y}, {1020, y}}, {pressure = 1.0})
end
-- Soft atmospheric transition along wall horizon
b_badger:stroke({{415, 480}, {700, 480}, {1020, 480}}, {pressure = 0.22})

-- 3. UNBROKEN LIMESTONE TABLETOP (x = 420 to 1020, y = 520 to 715)
-- Continuous sweeps eliminate ALL square notches and reset the stage cleanly
for y = 522, 630, 8 do
  local p = (y < 565) and p_stone_back or p_stone_lit
  b_f26:load(p, 1.0)
  b_f26:stroke({{420, y}, {700, y}, {1020, y}}, {pressure = 1.0})
end

-- Full width sweeps across foreground (y = 625 to 715, x = -15 to 1020)
for y = 628, 714, 8 do
  b_f26:load(p_stone_lit, 1.0)
  b_f26:stroke({{-15, y}, {350, y}, {700, y}, {1020, y}}, {pressure = 1.0})
end

-- Table to the left of the pitcher (x = -15 to 275, y = 520 to 630)
for y = 522, 626, 8 do
  local p = (y < 565) and p_stone_back or p_stone_lit
  b_f20:load(p, 1.0)
  b_f20:stroke({{-15, y}, {130, y}, {275, y}}, {pressure = 0.98})
end

-- Horizontal badger passes to create a serene, luminous stone surface
b_badger:stroke({{-15, 545}, {275, 545}}, {pressure = 0.20})
b_badger:stroke({{420, 550}, {1020, 550}}, {pressure = 0.22})
b_badger:stroke({{-15, 640}, {500, 640}, {1020, 640}}, {pressure = 0.24})
b_badger:stroke({{-15, 680}, {500, 680}, {1020, 680}}, {pressure = 0.24})
b_badger:stroke({{-15, 708}, {500, 708}, {1020, 708}}, {pressure = 0.24})

-- 4. RAZOR SPECULAR CHAMFER LINE ALONG LEDGE RIM (y = 715)
local b_r1 = brush{kind = "round", width = 1.4, point = 0.98}
local p_chamfer = pile{{"lead white", 8.8}, {"yellow ochre", 1.8}, medium = 0.05}
b_r1:load(p_chamfer, 1.0)
b_r1:stroke({{-10, 715}, {300, 715}, {650, 715}, {1015, 715}}, {pressure = {0.85, 0.9, 0.85}, shake = 0.02})

print("Stage unified: seamless wall and tabletop, clean ledge and chamfer.")

--@ chunk 157
-- CHUNK 157: UNIFY BACKGROUND WALL ATMOSPHERE AND TABLETOP HORIZON

p_wall_tone = pile{{"raw umber", 5.5}, {"bone black", 3.0}, {"yellow ochre", 2.5}, {"lead white", 1.2}, medium = 0.10}
p_wall_dark = pile{{"raw umber", 6.0}, {"bone black", 4.0}, {"yellow ochre", 1.5}, medium = 0.08}
p_stone_back = pile{{"lead white", 6.2}, {"yellow ochre", 3.0}, {"raw umber", 1.6}, {"red earth", 0.4}, medium = 0.08}

local b_f26 = brush("filbert", 26)
local b_f20 = brush("filbert", 20)
local b_badger = brush("badger", 36)

-- 1. UNIFY WALL BEHIND TABLE (x = 405 to 1020, y = 350 to 520)
-- Gradient from p_wall_tone at y=360 down to p_wall_dark at y=515
for y = 360, 440, 15 do
  b_f26:load(p_wall_tone, 1.0)
  b_f26:stroke({{405, y}, {710, y}, {1020, y}}, {pressure = 0.95})
end

for y = 445, 515, 12 do
  b_f26:load(p_wall_dark, 1.0)
  b_f26:stroke({{405, y}, {710, y}, {1020, y}}, {pressure = 0.98})
end

-- Horizontal and vertical badger blends to fuse wall seamlessly into upper field
b_badger:stroke({{405, 380}, {710, 380}, {1020, 380}}, {pressure = 0.25})
b_badger:stroke({{405, 430}, {710, 430}, {1020, 430}}, {pressure = 0.25})
b_badger:stroke({{405, 480}, {710, 480}, {1020, 480}}, {pressure = 0.25})

-- 2. WALL BEHIND LEFT TABLE (x = -15 to 250, y = 420 to 520)
for y = 430, 515, 14 do
  b_f20:load(p_wall_dark, 1.0)
  b_f20:stroke({{-15, y}, {120, y}, {250, y}}, {pressure = 0.95})
end
b_badger:stroke({{-15, 470}, {250, 470}}, {pressure = 0.22})

-- 3. CRISP, LUMINOUS HORIZON AT TABLE BACK (y = 520)
for y = 522, 538, 8 do
  b_f26:load(p_stone_back, 1.0)
  b_f26:stroke({{405, y}, {710, y}, {1020, y}}, {pressure = 0.98})
  b_f20:load(p_stone_back, 1.0)
  b_f20:stroke({{-15, y}, {120, y}, {250, y}}, {pressure = 0.98})
end
b_badger:stroke({{-15, 526}, {250, 526}}, {pressure = 0.18})
b_badger:stroke({{405, 526}, {1020, 526}}, {pressure = 0.18})

print("Background wall atmosphere unified across canvas; table horizon seamless.")

--@ chunk 158
-- CHUNK 158: ESTABLISH PRISTINE ARCHITECTURAL STAGE (WALL, TABLE, AND LEDGE)

-- Stage palette
p_wall = pile{{"raw umber", 6.5}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.08}
p_stone_back = pile{{"lead white", 6.5}, {"yellow ochre", 2.8}, {"raw umber", 1.5}, {"red earth", 0.3}, medium = 0.08}
p_stone_mid  = pile{{"lead white", 7.5}, {"yellow ochre", 2.4}, {"raw umber", 1.0}, {"red earth", 0.2}, medium = 0.08}
p_stone_lit  = pile{{"lead white", 8.2}, {"yellow ochre", 2.0}, {"raw umber", 0.5}, medium = 0.08}
p_ledge      = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.2}, medium = 0.05}

local b_f26 = brush("filbert", 26)
local b_badger = brush("badger", 36)

-- 1. DEEP CHIAROSCURO WALL (y = 0 to 520, x = -15 to 1020)
-- Sweep the entire wall from y = 150 down to y = 520 to unify completely
for y = 160, 515, 14 do
  b_f26:load(p_wall, 1.0)
  b_f26:stroke({{-15, y}, {300, y}, {700, y}, {1020, y}}, {pressure = 0.98})
end
-- Horizontal badger passes to create deep, quiet atmospheric resonance
b_badger:stroke({{-15, 250}, {500, 250}, {1020, 250}}, {pressure = 0.25})
b_badger:stroke({{-15, 380}, {500, 380}, {1020, 380}}, {pressure = 0.25})
b_badger:stroke({{-15, 480}, {500, 480}, {1020, 480}}, {pressure = 0.25})

-- 2. LIMESTONE TABLETOP SLAB (y = 520 to 715, x = -15 to 1020)
-- Swept horizontally from edge to edge into an unbroken plane
for y = 522, 600, 10 do
  b_f26:load(p_stone_back, 1.0)
  b_f26:stroke({{-15, y}, {350, y}, {700, y}, {1020, y}}, {pressure = 1.0})
end

for y = 605, 665, 10 do
  b_f26:load(p_stone_mid, 1.0)
  b_f26:stroke({{-15, y}, {350, y}, {700, y}, {1020, y}}, {pressure = 1.0})
end

for y = 670, 714, 8 do
  b_f26:load(p_stone_lit, 1.0)
  b_f26:stroke({{-15, y}, {350, y}, {700, y}, {1020, y}}, {pressure = 1.0})
end

-- Horizontal badger sweeps along table grain
b_badger:stroke({{-15, 545}, {500, 545}, {1020, 545}}, {pressure = 0.22})
b_badger:stroke({{-15, 615}, {500, 615}, {1020, 615}}, {pressure = 0.22})
b_badger:stroke({{-15, 675}, {500, 675}, {1020, 675}}, {pressure = 0.22})
b_badger:stroke({{-15, 705}, {500, 705}, {1020, 705}}, {pressure = 0.22})

-- 3. FRONT LEDGE (y = 715 to 805, x = -15 to 1020)
for y = 716, 805, 10 do
  b_f26:load(p_ledge, 1.0)
  b_f26:stroke({{-15, y}, {300, y}, {700, y}, {1020, y}}, {pressure = 1.0})
end
b_badger:stroke({{-15, 760}, {500, 760}, {1020, 760}}, {pressure = 0.25})

-- 4. SPECULAR CHAMFER RIM (y = 715)
local b_r1 = brush{kind = "round", width = 1.4, point = 0.98}
local p_chamfer = pile{{"lead white", 8.8}, {"yellow ochre", 1.8}, medium = 0.05}
b_r1:load(p_chamfer, 1.0)
b_r1:stroke({{-10, 715}, {300, 715}, {650, 715}, {1015, 715}}, {pressure = {0.85, 0.9, 0.85}, shake = 0.02})

print("Pristine architectural stage established.")

--@ chunk 159
-- CHUNK 159: PAINTING THE EARTHENWARE PITCHER (CÁNTARO)

-- Palette for authentic glazed terracotta earthenware
p_jug_body    = pile{{"red earth", 5.5}, {"yellow ochre", 4.0}, {"lead white", 2.2}, {"raw umber", 1.0}, medium = 0.08}
p_jug_lit     = pile{{"yellow ochre", 5.5}, {"red earth", 2.8}, {"lead white", 4.8}, {"raw umber", 0.5}, medium = 0.08}
p_jug_core    = pile{{"red earth", 5.0}, {"raw umber", 4.5}, {"bone black", 1.2}, {"yellow ochre", 1.5}, medium = 0.08}
p_jug_deep    = pile{{"bone black", 5.5}, {"raw umber", 4.0}, {"red earth", 1.8}, medium = 0.05}
p_jug_reflect = pile{{"yellow ochre", 5.0}, {"red earth", 2.5}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.10}
p_jug_hi      = pile{{"lead white", 8.8}, {"yellow ochre", 1.5}, {"red earth", 0.3}, medium = 0.04}
p_shadow_stone= pile{{"raw umber", 5.0}, {"yellow ochre", 3.0}, {"lead white", 2.0}, {"red earth", 0.6}, medium = 0.12}

local b_f20 = brush("filbert", 20)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_f8  = brush("filbert", 8)
local b_r5  = brush{kind = "round", width = 5.2, point = 0.9}
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}
local b_badger = brush("badger", 22)

-- 1. SOLID BLOCK-IN OF PITCHER VOLUME (Turned clay mass from mouth to foot)
-- Turned neck (y = 275 to 365, x = 340 to 400)
for y = 275, 365, 8 do
  b_f14:load(p_jug_body, 1.0)
  b_f14:stroke({{344, y}, {370, y}, {396, y}}, {pressure = 1.0})
end

-- Swelling shoulder and belly (y = 365 to 625)
for y = 368, 622, 8 do
  local xl, xr
  if y < 485 then
    -- Expanding from collar {344, 365} to belly {270, 485} on left, {396, 365} to {470, 485} on right
    local t = (y - 365) / 120
    xl = 344 - 74 * math.sin(t * math.pi * 0.5)
    xr = 396 + 74 * math.sin(t * math.pi * 0.5)
  else
    -- Tapering from belly {270, 485} to foot {328, 622} on left, {470, 485} to {422, 622} on right
    local t = (y - 485) / 137
    xl = 270 + 58 * (t * t * 0.6 + t * 0.4)
    xr = 470 - 48 * (t * t * 0.6 + t * 0.4)
  end
  b_f20:load(p_jug_body, 1.0)
  b_f20:stroke({{xl, y}, {(xl + xr) * 0.5, y}, {xr, y}}, {pressure = 1.0})
end

-- 2. MODELING LIGHT AND SHADE
-- Lit flank (left of center, facing north light)
for y = 280, 360, 10 do
  b_f10:load(p_jug_lit, 1.0)
  b_f10:stroke({{348, y}, {365, y}}, {pressure = 0.95})
end

b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{345, 370}, {320, 430}, {305, 485}, {325, 550}, {345, 615}}, {pressure = {0.9, 1.0, 1.0, 0.9, 0.8}, swell = {0.9, 1.3, 0.8}})
b_f10:load(p_jug_lit, 1.0)
b_f10:stroke({{355, 380}, {335, 440}, {322, 490}, {338, 550}, {355, 615}}, {pressure = 0.9})

-- Core shadow down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{385, 280}, {390, 330}, {405, 375}, {440, 435}, {460, 485}, {448, 545}, {425, 595}, {412, 620}}, {
  pressure = {0.85, 0.9, 0.98, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.8, 1.2, 0.8}
})

-- Deep shadow near the contour
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{390, 285}, {395, 335}, {412, 380}, {448, 440}, {466, 490}, {455, 545}, {435, 595}, {418, 622}}, {
  pressure = 0.85
})

-- Ambient limestone bounce on shadow contour
b_f8:load(p_jug_reflect, 0.9)
b_f8:stroke({{395, 310}, {415, 375}, {455, 445}, {468, 490}, {458, 545}, {440, 590}, {422, 620}}, {
  pressure = {0.4, 0.5, 0.5, 0.4}
})

-- 3. SOFT BADGER BLEND WET-IN-WET (Molds vessel into sculptural roundness)
b_badger:stroke({{350, 310}, {350, 360}}, {pressure = 0.20})
b_badger:stroke({{330, 400}, {320, 490}, {335, 600}}, {pressure = 0.22})
b_badger:stroke({{380, 400}, {400, 490}, {390, 600}}, {pressure = 0.22})
b_badger:stroke({{430, 410}, {445, 490}, {420, 600}}, {pressure = 0.22})

-- 4. TURNED MOUTH AND APERTURE
-- Dark interior cavity
b_r5:load(p_jug_deep, 1.0)
b_r5:stroke({{352, 274}, {370, 276}, {388, 274}}, {pressure = 1.0})

-- Flared lip rim
b_r2:load(p_jug_body, 1.0)
b_r2:stroke({{336, 272}, {352, 267}, {370, 265}, {388, 267}, {404, 272}}, {pressure = 0.95})
b_r2:stroke({{336, 272}, {352, 277}, {370, 279}, {388, 277}, {404, 272}}, {pressure = 0.95})

-- Lip rim highlight (catching north light on left)
b_r1:load(p_jug_hi, 1.0)
b_r1:stroke({{338, 272}, {352, 267}, {366, 266}}, {pressure = 0.9})

-- 5. STURDY HAND-PULLED LOOP HANDLE
-- Solid handle mass springing from shoulder {335, 375} out to {232, 420} down to {278, 535}
local pts_handle = {{335, 375}, {295, 380}, {255, 395}, {232, 420}, {235, 455}, {252, 495}, {278, 535}}
b_r5:load(p_jug_core, 1.0)
b_r5:stroke(pts_handle, {pressure = 1.0, swell = {0.8, 1.2, 0.9}})

-- Lit outer crest of handle
b_r3:load(p_jug_lit, 1.0)
b_r3:stroke({{332, 373}, {293, 378}, {253, 393}, {230, 418}, {233, 450}}, {pressure = 0.9})

-- Catchlight glint on handle shoulder
b_r1:load(p_jug_hi, 1.0)
b_r1:stroke({{248, 398}, {232, 418}, {234, 435}}, {pressure = {0.6, 0.95, 0.6}})

-- Clear negative space inside handle loop with dark wall umber
local b_f8_wall = brush("filbert", 8)
b_f8_wall:load(p_wall, 1.0)
b_f8_wall:stroke({{250, 425}, {270, 420}, {290, 410}}, {pressure = 0.95})
b_f8_wall:stroke({{255, 455}, {275, 455}, {295, 450}}, {pressure = 0.95})
b_f8_wall:stroke({{265, 490}, {280, 490}}, {pressure = 0.95})

-- 6. SPECULAR GLAZE HIGHLIGHT ON SHOULDER CURVE
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{325, 400}, {322, 418}, {318, 438}}, {pressure = {0.7, 0.98, 0.6}, swell = {0.8, 1.35, 0.7}})
b_r1:load(p_jug_hi, 1.0)
b_r1:touch(322, 417, {pressure = 1.0})

-- 7. FOOT RING SEATING AND CONTACT SHADOW
-- Turned foot ring rim
b_r3:load(p_jug_body, 1.0)
b_r3:stroke({{325, 622}, {372, 624}, {422, 622}}, {pressure = 0.95})

-- Intense dark contact occlusion shadow under the foot ring
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{320, 625}, {372, 627}, {425, 625}}, {pressure = {0.95, 1.0, 0.95}})

-- Pitcher cast shadow flowing rightward across the limestone slab
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{422, 624}, {470, 628}, {535, 636}}, {pressure = {0.85, 0.2}, swell = {1.0, 1.3, 0.6}})

print("Earthenware pitcher completely painted: solid volume, turned handle, foot ring, and cast shadow.")

--@ chunk 160
-- CHUNK 160: REFINING THE EARTHENWARE PITCHER (HANDLE, VOLUME, AND GLAZE)

local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_f8  = brush("filbert", 8)
local b_r5  = brush{kind = "round", width = 5.5, point = 0.9}
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}
local b_badger = brush("badger", 22)

-- 1. STURDY PULLED-CLAY STRAP HANDLE
-- Solid strap body (width ~10-12 units)
local pts_handle_outer = {{340, 365}, {295, 375}, {250, 395}, {226, 430}, {235, 475}, {260, 515}, {288, 545}}
b_r5:load(p_jug_body, 1.0)
b_r5:stroke(pts_handle_outer, {pressure = 1.0, swell = {0.9, 1.3, 1.0}})

-- Shadow on underside of handle
local pts_handle_under = {{335, 380}, {295, 390}, {255, 410}, {236, 440}, {245, 480}, {270, 520}, {290, 545}}
b_r3:load(p_jug_core, 1.0)
b_r3:stroke(pts_handle_under, {pressure = 0.95})

-- North-light lit ridge along outer edge of handle
b_r3:load(p_jug_lit, 1.0)
b_r3:stroke({{338, 364}, {292, 373}, {246, 393}, {224, 428}, {230, 465}}, {pressure = {0.8, 1.0, 0.8}})

-- Specular glaze catchlight on handle curve
b_r1:load(p_jug_hi, 1.0)
b_r1:stroke({{242, 398}, {226, 426}, {228, 442}}, {pressure = {0.5, 0.95, 0.5}})

-- Clear wall negative space inside handle
local b_f8_wall = brush("filbert", 7)
b_f8_wall:load(p_wall, 1.0)
b_f8_wall:stroke({{255, 435}, {280, 430}, {310, 420}}, {pressure = 0.98})
b_f8_wall:stroke({{260, 465}, {285, 465}, {310, 460}}, {pressure = 0.98})
b_f8_wall:stroke({{275, 495}, {295, 495}}, {pressure = 0.95})

-- 2. TURNED NECK AND FLUSH CONTOURS
-- Smooth vertical body strokes on neck
b_f10:load(p_jug_body, 1.0)
b_f10:stroke({{365, 275}, {365, 365}}, {pressure = 0.95})
b_f8:load(p_jug_lit, 1.0)
b_f8:stroke({{352, 275}, {352, 365}}, {pressure = 0.95})

-- Dark interior mouth aperture
b_r5:load(p_jug_deep, 1.0)
b_r5:stroke({{352, 274}, {370, 276}, {388, 274}}, {pressure = 1.0})

-- Flared lip
b_r2:load(p_jug_body, 1.0)
b_r2:stroke({{336, 272}, {352, 267}, {370, 265}, {388, 267}, {404, 272}}, {pressure = 0.95})
b_r1:load(p_jug_hi, 1.0)
b_r1:stroke({{338, 272}, {352, 267}, {368, 266}}, {pressure = 0.9})

-- 3. BELLY MODELING AND SCULPTURAL ROUNDNESS
-- Rich terracotta body passes
b_f14:load(p_jug_body, 1.0)
b_f14:stroke({{350, 380}, {365, 480}, {370, 600}}, {pressure = 0.95})

-- Lit flank facing light
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{335, 380}, {312, 460}, {302, 510}, {320, 580}, {340, 618}}, {pressure = 0.98})

-- Core shadow down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{400, 380}, {435, 440}, {455, 490}, {445, 545}, {425, 595}, {412, 620}}, {pressure = 0.98})

-- Deep shadow on far right contour
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{412, 385}, {448, 445}, {466, 490}, {456, 545}, {436, 595}, {420, 622}}, {pressure = 0.9})

-- Table reflection along shaded flank
b_f8:load(p_jug_reflect, 0.9)
b_f8:stroke({{418, 395}, {455, 450}, {468, 492}, {458, 545}, {440, 590}, {422, 620}}, {pressure = 0.45})

-- Circumferential / horizontal badger sweeps following the wheel marks
b_badger:stroke({{320, 420}, {380, 420}, {440, 420}}, {pressure = 0.22})
b_badger:stroke({{290, 485}, {370, 485}, {460, 485}}, {pressure = 0.22})
b_badger:stroke({{310, 550}, {370, 550}, {440, 550}}, {pressure = 0.22})
b_badger:stroke({{330, 605}, {370, 605}, {420, 605}}, {pressure = 0.20})

-- 4. DECISIVE GLAZE HIGHLIGHT ON SHOULDER
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{324, 402}, {320, 418}, {316, 436}}, {pressure = {0.7, 1.0, 0.7}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_jug_hi, 1.0)
b_r1:touch(320, 418, {pressure = 1.0})

-- 5. FOOT RING SEATING AND CAST SHADOW
b_r3:load(p_jug_body, 1.0)
b_r3:stroke({{325, 622}, {372, 624}, {422, 622}}, {pressure = 0.95})

-- Occlusion contact shadow under foot
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{320, 625}, {372, 627}, {425, 625}}, {pressure = {0.95, 1.0, 0.95}})

-- Cast shadow across limestone table
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{422, 624}, {470, 628}, {535, 636}}, {pressure = {0.85, 0.2}, swell = {1.0, 1.3, 0.6}})

print("Pitcher refined with sturdy strap handle, sculptural modeling, and glaze catchlight.")

--@ chunk 161
-- CHUNK 161: MASTER MODELING OF THE EARTHENWARE CANTARO

-- Terracotta earthenware piles
p_jug_body    = pile{{"red earth", 5.5}, {"yellow ochre", 4.0}, {"lead white", 2.2}, {"raw umber", 1.0}, medium = 0.08}
p_jug_lit     = pile{{"yellow ochre", 5.5}, {"red earth", 2.8}, {"lead white", 4.8}, {"raw umber", 0.5}, medium = 0.08}
p_jug_core    = pile{{"red earth", 5.0}, {"raw umber", 4.5}, {"bone black", 1.2}, {"yellow ochre", 1.5}, medium = 0.08}
p_jug_deep    = pile{{"bone black", 5.5}, {"raw umber", 4.0}, {"red earth", 1.8}, medium = 0.05}
p_jug_reflect = pile{{"yellow ochre", 5.0}, {"red earth", 2.5}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.10}
p_jug_hi      = pile{{"lead white", 8.8}, {"yellow ochre", 1.5}, {"red earth", 0.3}, medium = 0.04}

-- Clean stage piles for cutting in outside silhouette
p_wall = pile{{"raw umber", 6.5}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.08}
p_stone_back = pile{{"lead white", 6.5}, {"yellow ochre", 2.8}, {"raw umber", 1.5}, {"red earth", 0.3}, medium = 0.08}
p_stone_lit  = pile{{"lead white", 8.2}, {"yellow ochre", 2.0}, {"raw umber", 0.5}, medium = 0.08}
p_shadow_stone= pile{{"raw umber", 5.0}, {"yellow ochre", 3.0}, {"lead white", 2.0}, {"red earth", 0.6}, medium = 0.12}

local pts_cantaro = {
  {336, 272}, {352, 267}, {370, 265}, {388, 267}, {404, 272},
  {398, 310}, {395, 360},
  {420, 395}, {450, 435}, {470, 485},
  {458, 535}, {438, 580}, {414, 624},
  {370, 626}, {326, 624},
  {302, 580}, {282, 535}, {270, 485},
  {290, 435}, {320, 395}, {345, 360},
  {342, 310}
}
local m_cantaro = poly(pts_cantaro, true)

-- 1. OPAQUE BODY COAT (Obliterates plaid grid and comb marks)
work(m_cantaro, {
  hand = "body",
  pile = p_jug_body,
  angle = 1.45,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- 2. CARVE AND CLEAN OUTSIDE SILHOUETTE (Cut-in strokes)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_r5  = brush{kind = "round", width = 5.5, point = 0.9}
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- Clean wall outside right neck and shoulder (x > 395)
b_f10:load(p_wall, 1.0)
b_f10:stroke({{406, 270}, {402, 310}, {400, 360}, {415, 385}}, {pressure = 0.98})

-- Clean wall outside left neck
b_f10:load(p_wall, 1.0)
b_f10:stroke({{334, 270}, {338, 310}, {342, 360}}, {pressure = 0.98})

-- Clean stone table outside lower left belly (y > 520, x < contour)
b_f14:load(p_stone_lit, 1.0)
b_f14:stroke({{250, 530}, {265, 535}, {260, 560}}, {pressure = 0.98})
b_f14:stroke({{270, 570}, {285, 580}, {300, 610}, {320, 626}}, {pressure = 0.98})

-- Clean stone table outside lower right belly (y > 520, x > contour)
b_f14:load(p_stone_lit, 1.0)
b_f14:stroke({{475, 530}, {465, 545}, {445, 585}, {420, 626}}, {pressure = 0.98})

-- 3. MODEL LIGHT AND SHADE WET-IN-WET
-- Lit flank facing north light (left of center)
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{352, 275}, {352, 360}}, {pressure = 0.95})
b_f14:stroke({{345, 365}, {325, 410}, {305, 475}, {312, 535}, {330, 595}, {345, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.3, 0.85}
})

-- Core shadow down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{390, 275}, {390, 355}}, {pressure = 0.95})
b_f14:stroke({{400, 365}, {425, 415}, {452, 475}, {445, 535}, {428, 595}, {408, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.3, 0.85}
})

-- Deep shadow along right contour
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{402, 310}, {400, 360}, {422, 400}, {452, 440}, {468, 485}, {456, 540}, {436, 595}, {412, 624}}, {
  pressure = 0.85
})

-- Reflected ambient stone bounce on right edge
b_f8:load(p_jug_reflect, 0.9)
b_f8:stroke({{422, 405}, {453, 445}, {468, 485}, {458, 540}, {438, 590}, {414, 622}}, {pressure = 0.45})

-- 4. TURNED MOUTH AND APERTURE
-- Dark interior cavity
b_r5:load(p_jug_deep, 1.0)
b_r5:stroke({{352, 273}, {370, 275}, {388, 273}}, {pressure = 1.0})

-- Flared lip rim
b_r2:load(p_jug_body, 1.0)
b_r2:stroke({{336, 272}, {352, 267}, {370, 265}, {388, 267}, {404, 272}}, {pressure = 0.95})
b_r1:load(p_jug_hi, 1.0)
b_r1:stroke({{338, 272}, {352, 267}, {368, 266}}, {pressure = 0.9})

-- 5. STURDY STRAP HANDLE
local pts_h_outer = {{342, 365}, {295, 375}, {250, 395}, {226, 430}, {235, 475}, {260, 515}, {284, 540}}
b_r5:load(p_jug_body, 1.0)
b_r5:stroke(pts_h_outer, {pressure = 1.0, swell = {0.9, 1.3, 1.0}})

-- Shadow on underside of handle
local pts_h_under = {{338, 375}, {295, 385}, {255, 408}, {235, 438}, {244, 480}, {268, 518}, {285, 540}}
b_r3:load(p_jug_core, 1.0)
b_r3:stroke(pts_h_under, {pressure = 0.95})

-- North-light lit ridge along outer edge of handle
b_r3:load(p_jug_lit, 1.0)
b_r3:stroke({{340, 363}, {293, 372}, {247, 392}, {224, 428}, {230, 465}}, {pressure = {0.8, 1.0, 0.8}})

-- Specular glaze catchlight on handle curve
b_r1:load(p_jug_hi, 1.0)
b_r1:stroke({{242, 398}, {226, 426}, {228, 442}}, {pressure = {0.5, 0.95, 0.5}})

-- Clear negative space inside handle loop with dark wall umber
local b_f8_w = brush("filbert", 7)
b_f8_w:load(p_wall, 1.0)
b_f8_w:stroke({{255, 435}, {280, 430}, {305, 420}}, {pressure = 0.98})
b_f8_w:stroke({{260, 465}, {285, 465}, {305, 460}}, {pressure = 0.98})
b_f8_w:stroke({{272, 495}, {290, 495}}, {pressure = 0.95})

-- 6. DECISIVE GLAZE HIGHLIGHT ON SHOULDER
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{324, 402}, {320, 418}, {316, 436}}, {pressure = {0.7, 1.0, 0.7}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_jug_hi, 1.0)
b_r1:touch(320, 418, {pressure = 1.0})

-- 7. FOOT RING SEATING AND OCCLUSION SHADOW
b_r3:load(p_jug_body, 1.0)
b_r3:stroke({{326, 624}, {370, 626}, {414, 624}}, {pressure = 0.95})

-- Intense dark contact shadow along footring
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{322, 626}, {370, 628}, {418, 626}}, {pressure = {0.95, 1.0, 0.95}})

-- Cast shadow across limestone table
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{414, 625}, {465, 628}, {535, 636}}, {pressure = {0.85, 0.2}, swell = {1.0, 1.3, 0.6}})

print("Earthenware cantaro fully consolidated: noble geometry, solid mass, and clean contours.")

--@ chunk 162
-- CHUNK 162: REFINING PITCHER TO PERFECTION AND LAYING IN RIPE QUINCE

-- 1. DEFINE ALL PALETTE PILES
p_jug_rich   = pile{{"red earth", 6.0}, {"yellow ochre", 4.0}, {"lead white", 1.5}, {"raw umber", 1.0}, medium = 0.06}
p_jug_lit    = pile{{"yellow ochre", 6.0}, {"red earth", 2.5}, {"lead white", 4.5}, {"raw umber", 0.5}, medium = 0.06}
p_jug_core   = pile{{"red earth", 5.5}, {"raw umber", 4.5}, {"bone black", 1.2}, {"yellow ochre", 1.0}, medium = 0.06}
p_jug_deep   = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.8}, medium = 0.05}
p_jug_reflect= pile{{"yellow ochre", 5.0}, {"red earth", 2.0}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.08}
p_jug_hi     = pile{{"lead white", 9.0}, {"yellow ochre", 1.5}, {"red earth", 0.2}, medium = 0.04}

p_q_body     = pile{{"yellow ochre", 5.5}, {"chrome yellow", 4.0}, {"lead white", 2.5}, {"green earth", 0.8}, medium = 0.06}
p_q_lit      = pile{{"chrome yellow", 5.0}, {"yellow ochre", 3.0}, {"lead white", 5.0}, medium = 0.06}
p_q_core     = pile{{"raw umber", 4.5}, {"yellow ochre", 4.0}, {"red earth", 2.0}, {"bone black", 0.5}, medium = 0.06}
p_q_deep     = pile{{"raw umber", 5.5}, {"bone black", 2.5}, {"red earth", 1.5}, {"yellow ochre", 1.5}, medium = 0.05}
p_q_reflect  = pile{{"yellow ochre", 5.0}, {"lead white", 3.0}, {"raw umber", 1.5}, medium = 0.08}
p_q_hi       = pile{{"lead white", 8.8}, {"chrome yellow", 2.5}, {"yellow ochre", 0.8}, medium = 0.04}
p_stem       = pile{{"raw umber", 6.0}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.05}

p_wall       = pile{{"raw umber", 6.5}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.08}
p_stone_lit  = pile{{"lead white", 8.2}, {"yellow ochre", 2.0}, {"raw umber", 0.5}, medium = 0.08}
p_shadow_stone = pile{{"raw umber", 4.8}, {"yellow ochre", 3.2}, {"lead white", 2.2}, {"red earth", 0.6}, medium = 0.10}

local b_f22 = brush("filbert", 22)
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_f8  = brush("filbert", 8)
local b_r5  = brush{kind = "round", width = 5.5, point = 0.9}
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 2. PERFECTING THE EARTHENWARE CANTARO
-- A. Sweep broad vertical body color down the belly to obliterate grid marks
b_f18:load(p_jug_rich, 1.0)
b_f18:stroke({{360, 365}, {365, 485}, {370, 622}}, {pressure = 1.0})
b_f14:load(p_jug_rich, 1.0)
b_f14:stroke({{340, 370}, {335, 480}, {348, 620}}, {pressure = 0.98})
b_f14:stroke({{385, 370}, {405, 480}, {395, 620}}, {pressure = 0.98})

-- B. Lit flank on left (north raking light)
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{348, 275}, {348, 360}}, {pressure = 0.95})
b_f14:stroke({{340, 365}, {318, 420}, {300, 480}, {310, 545}, {330, 600}, {345, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- C. Core shadow on right
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{390, 275}, {390, 355}}, {pressure = 0.95})
b_f14:stroke({{400, 365}, {430, 425}, {455, 485}, {446, 545}, {428, 595}, {408, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.3, 0.85}
})

-- D. Deep shadow along right contour
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{402, 310}, {400, 360}, {422, 400}, {452, 440}, {468, 485}, {456, 540}, {436, 595}, {412, 624}}, {
  pressure = 0.85
})

-- E. Ambient table bounce
b_f8:load(p_jug_reflect, 0.9)
b_f8:stroke({{422, 405}, {453, 445}, {468, 485}, {458, 540}, {438, 590}, {414, 622}}, {pressure = 0.45})

-- F. Substantial pulled strap handle
local pts_h_outer = {{342, 365}, {295, 375}, {248, 395}, {222, 430}, {230, 475}, {255, 515}, {284, 542}}
b_r5:load(p_jug_rich, 1.0)
b_r5:stroke(pts_h_outer, {pressure = 1.0, swell = {0.9, 1.35, 1.0}})

local pts_h_under = {{338, 375}, {295, 385}, {253, 408}, {232, 438}, {240, 480}, {264, 518}, {285, 542}}
b_r3:load(p_jug_core, 1.0)
b_r3:stroke(pts_h_under, {pressure = 0.95})

b_r3:load(p_jug_lit, 1.0)
b_r3:stroke({{340, 363}, {293, 372}, {244, 392}, {220, 428}, {226, 465}}, {pressure = {0.8, 1.0, 0.8}})

b_r1:load(p_jug_hi, 1.0)
b_r1:stroke({{238, 398}, {222, 426}, {224, 442}}, {pressure = {0.5, 0.95, 0.5}})

-- Clear negative space inside handle
local b_f8_w = brush("filbert", 7)
b_f8_w:load(p_wall, 1.0)
b_f8_w:stroke({{250, 435}, {278, 430}, {302, 420}}, {pressure = 0.98})
b_f8_w:stroke({{255, 465}, {282, 465}, {302, 460}}, {pressure = 0.98})
b_f8_w:stroke({{268, 495}, {288, 495}}, {pressure = 0.95})

-- G. North-light glaze highlight on shoulder
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{324, 402}, {320, 418}, {316, 436}}, {pressure = {0.7, 1.0, 0.7}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_jug_hi, 1.0)
b_r1:touch(320, 418, {pressure = 1.0})

-- H. Clean tabletop along lower belly and foot
b_f14:load(p_stone_lit, 1.0)
b_f14:stroke({{230, 560}, {255, 560}, {280, 580}, {310, 622}}, {pressure = 0.98})
b_f14:stroke({{485, 560}, {460, 560}, {440, 585}, {420, 622}}, {pressure = 0.98})

-- Footring and contact shadow
b_r3:load(p_jug_rich, 1.0)
b_r3:stroke({{326, 624}, {370, 626}, {414, 624}}, {pressure = 0.95})
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{322, 626}, {370, 628}, {418, 626}}, {pressure = {0.95, 1.0, 0.95}})

-- Pitcher cast shadow flowing rightward
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{414, 625}, {465, 628}, {535, 636}}, {pressure = {0.85, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 3. PAINTING THE RIPE KNOPPY QUINCE (MEMBRILLO)
-- Silhouette points of the knobby organic quince
local pts_quince = {
  {630, 486}, {648, 492}, {672, 515}, {696, 555}, {718, 595},
  {724, 635}, {708, 665}, {678, 676}, {635, 678}, {590, 674},
  {562, 660}, {544, 632}, {540, 595}, {550, 550}, {574, 515},
  {602, 490}, {618, 485}
}
local m_q = poly(pts_quince, true)

-- A. Dense golden body coat (lead white + chrome yellow + yellow ochre)
work(m_q, {
  hand = "body",
  pile = p_q_body,
  angle = 1.35,
  coverage = 3.2,
  fill = true,
  edge = "firm"
})

-- B. Sculptural modeling of organic lobes
-- Lit sunlit flank (left lobe facing raking light)
b_f14:load(p_q_lit, 1.0)
b_f14:stroke({{610, 492}, {582, 530}, {562, 580}, {558, 625}, {572, 660}, {605, 674}}, {
  pressure = {0.85, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.3, 0.85}
})
b_f10:load(p_q_lit, 1.0)
b_f10:stroke({{625, 510}, {605, 560}, {590, 615}, {615, 665}}, {pressure = 0.92})

-- Core shadow down right flank
b_f14:load(p_q_core, 1.0)
b_f14:stroke({{645, 495}, {670, 540}, {695, 590}, {700, 635}, {680, 670}, {650, 676}}, {
  pressure = {0.85, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.3, 0.85}
})

-- Deep shadow near right contour
b_f10:load(p_q_deep, 0.95)
b_f10:stroke({{665, 520}, {692, 565}, {716, 610}, {714, 645}, {695, 670}}, {pressure = 0.85})

-- Reflected limestone bounce on right flank
b_f8:load(p_q_reflect, 0.9)
b_f8:stroke({{675, 535}, {702, 580}, {722, 618}, {718, 650}, {700, 670}}, {pressure = 0.45})

-- C. Tactile impasto highlight on the knobby sunlit crest
b_r3:load(p_q_hi, 1.0)
b_r3:stroke({{575, 570}, {566, 598}, {570, 626}}, {pressure = {0.9, 1.0, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_q_hi, 1.0)
b_r1:touch(566, 597, {pressure = 1.0})

-- D. Crown socket, calyx, and arching woody stem
-- Recessed socket at crown
b_r2:load(p_q_deep, 1.0)
b_r2:touch(628, 488, {pressure = 0.95})

-- Gnarled woody stem arching into dark wall
b_r2:load(p_stem, 1.0)
b_r2:stroke({{628, 488}, {634, 474}, {642, 460}, {652, 450}}, {pressure = {0.98, 0.9, 0.5}})
b_r1:load(p_q_lit, 0.85)
b_r1:stroke({{629, 486}, {635, 474}, {643, 461}}, {pressure = 0.6})

-- Calyx eye tucked at base
b_r2:load(p_q_deep, 1.0)
b_r2:touch(655, 676, {pressure = 0.95})

-- Occlusion contact shadow under base
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{580, 674}, {635, 678}, {680, 676}, {705, 668}}, {pressure = {0.85, 1.0, 0.9, 0.7}})

-- Transparent cast shadow of quince flowing rightward across limestone
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{690, 676}, {740, 682}, {790, 688}}, {pressure = {0.8, 0.2}, swell = {1.0, 1.3, 0.6}})

print("Pitcher perfected and ripe knobby quince laid in with full volume.")

--@ chunk 163
-- CHUNK 163: CONSOLIDATING PITCHER AND SCULPTING VOLUPTUOUS QUINCE

-- Piles for tabletop repairs
p_stone_mid  = pile{{"lead white", 7.5}, {"yellow ochre", 2.4}, {"raw umber", 1.0}, {"red earth", 0.2}, medium = 0.08}
p_stone_back = pile{{"lead white", 6.5}, {"yellow ochre", 2.8}, {"raw umber", 1.5}, {"red earth", 0.3}, medium = 0.08}
p_shadow_stone = pile{{"raw umber", 4.8}, {"yellow ochre", 3.2}, {"lead white", 2.2}, {"red earth", 0.6}, medium = 0.10}

-- Cantaro piles
p_jug_rich   = pile{{"red earth", 6.2}, {"yellow ochre", 3.8}, {"lead white", 1.2}, {"raw umber", 1.0}, medium = 0.06}
p_jug_lit    = pile{{"yellow ochre", 6.0}, {"red earth", 2.5}, {"lead white", 4.5}, {"raw umber", 0.5}, medium = 0.06}
p_jug_core   = pile{{"red earth", 5.5}, {"raw umber", 4.5}, {"bone black", 1.2}, {"yellow ochre", 1.0}, medium = 0.06}
p_jug_deep   = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.8}, medium = 0.05}
p_jug_reflect= pile{{"yellow ochre", 5.0}, {"red earth", 2.0}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.08}
p_jug_hi     = pile{{"lead white", 9.0}, {"yellow ochre", 1.5}, {"red earth", 0.2}, medium = 0.04}

-- Quince piles
p_q_rich_gold= pile{{"yellow ochre", 5.5}, {"chrome yellow", 4.5}, {"lead white", 2.0}, {"green earth", 0.5}, medium = 0.06}
p_q_lit      = pile{{"chrome yellow", 5.5}, {"yellow ochre", 2.5}, {"lead white", 5.5}, medium = 0.05}
p_q_core     = pile{{"raw umber", 5.0}, {"yellow ochre", 3.5}, {"red earth", 2.5}, {"bone black", 0.8}, medium = 0.06}
p_q_deep     = pile{{"raw umber", 6.0}, {"bone black", 3.0}, {"red earth", 1.8}, medium = 0.05}
p_q_reflect  = pile{{"yellow ochre", 5.5}, {"lead white", 3.0}, {"raw umber", 1.5}, {"red earth", 0.8}, medium = 0.08}
p_q_hi       = pile{{"lead white", 9.0}, {"chrome yellow", 2.5}, {"yellow ochre", 0.6}, medium = 0.04}
p_stem       = pile{{"raw umber", 6.0}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.05}

local b_f20 = brush("filbert", 20)
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_f8  = brush("filbert", 8)
local b_r5  = brush{kind = "round", width = 5.5, point = 0.9}
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}
local b_badger = brush("badger", 24)

-- 1. ERASE BAT-WING SMUDGES BESIDE PITCHER WITH TABLETOP BODY COLOR
-- Sweep horizontal strokes along table grain across y = 550 to 625
for y = 555, 622, 8 do
  local p = (y < 580) and p_stone_back or p_stone_mid
  -- Left of jug
  b_f14:load(p, 1.0)
  b_f14:stroke({{180, y}, {240, y}, {280, y}}, {pressure = 0.98})
  -- Right of jug
  b_f14:load(p, 1.0)
  b_f14:stroke({{425, y}, {480, y}, {540, y}}, {pressure = 0.98})
end
b_badger:stroke({{180, 585}, {280, 585}}, {pressure = 0.22})
b_badger:stroke({{425, 585}, {540, 585}}, {pressure = 0.22})

-- 2. PERFECTING THE EARTHENWARE CANTARO
-- Rich terracotta body passes down belly
b_f18:load(p_jug_rich, 1.0)
b_f18:stroke({{360, 365}, {365, 485}, {370, 622}}, {pressure = 1.0})
b_f14:load(p_jug_rich, 1.0)
b_f14:stroke({{335, 370}, {330, 480}, {345, 620}}, {pressure = 0.98})

-- Lit flank facing raking north light
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{340, 365}, {318, 420}, {298, 480}, {308, 545}, {328, 600}, {345, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Core shadow down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{395, 365}, {425, 425}, {452, 485}, {444, 545}, {426, 595}, {408, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Deep shadow on right contour
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{400, 360}, {422, 400}, {450, 440}, {468, 485}, {456, 540}, {436, 595}, {412, 624}}, {
  pressure = 0.9
})

-- Reflected ambient bounce
b_f8:load(p_jug_reflect, 0.9)
b_f8:stroke({{422, 405}, {452, 445}, {468, 485}, {458, 540}, {438, 590}, {414, 622}}, {pressure = 0.45})

-- Gentle vertical blend down the belly to fuse transitions
b_badger:stroke({{340, 380}, {320, 480}, {335, 610}}, {pressure = 0.18})
b_badger:stroke({{370, 380}, {370, 480}, {370, 610}}, {pressure = 0.18})
b_badger:stroke({{410, 380}, {430, 480}, {410, 610}}, {pressure = 0.18})

-- Substantial pulled strap handle
local pts_h_outer = {{342, 365}, {295, 375}, {248, 395}, {222, 430}, {230, 475}, {255, 515}, {284, 542}}
b_r5:load(p_jug_rich, 1.0)
b_r5:stroke(pts_h_outer, {pressure = 1.0, swell = {0.9, 1.35, 1.0}})

local pts_h_under = {{338, 375}, {295, 385}, {253, 408}, {232, 438}, {240, 480}, {264, 518}, {285, 542}}
b_r3:load(p_jug_core, 1.0)
b_r3:stroke(pts_h_under, {pressure = 0.95})

b_r3:load(p_jug_lit, 1.0)
b_r3:stroke({{340, 363}, {293, 372}, {244, 392}, {220, 428}, {226, 465}}, {pressure = {0.8, 1.0, 0.8}})

b_r1:load(p_jug_hi, 1.0)
b_r1:stroke({{238, 398}, {222, 426}, {224, 442}}, {pressure = {0.5, 0.95, 0.5}})

-- Glaze catchlight on shoulder
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{324, 402}, {320, 418}, {316, 436}}, {pressure = {0.7, 1.0, 0.7}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_jug_hi, 1.0)
b_r1:touch(320, 418, {pressure = 1.0})

-- Footring and contact shadow
b_r3:load(p_jug_rich, 1.0)
b_r3:stroke({{326, 624}, {370, 626}, {414, 624}}, {pressure = 0.95})
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{322, 626}, {370, 628}, {418, 626}}, {pressure = {0.95, 1.0, 0.95}})

-- Pitcher cast shadow flowing rightward
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{414, 625}, {465, 628}, {535, 636}}, {pressure = {0.85, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 3. VOLUPTUOUS SCULPTURAL MODELING OF THE QUINCE (MEMBRILLO)
-- A. Blend the open stipples into a smooth, dense golden flesh
local pts_quince = {
  {630, 486}, {648, 492}, {672, 515}, {696, 555}, {718, 595},
  {724, 635}, {708, 665}, {678, 676}, {635, 678}, {590, 674},
  {562, 660}, {544, 632}, {540, 595}, {550, 550}, {574, 515},
  {602, 490}, {618, 485}
}
local m_q = poly(pts_quince, true)

-- Blend wet paint across the entire fruit mass
blend(m_q, {angle = 1.35, coverage = 1.5})

-- B. Lay deep, rich, warm amber core shadow across the right hemisphere
b_f14:load(p_q_core, 1.0)
b_f14:stroke({{642, 492}, {668, 535}, {688, 580}, {695, 625}, {675, 665}, {645, 675}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.9, 1.3, 0.9}
})
b_f10:load(p_q_deep, 1.0)
b_f10:stroke({{665, 520}, {690, 565}, {712, 610}, {710, 645}, {690, 670}}, {pressure = 0.92})

-- Reflected limestone bounce along right shaded contour
b_f8:load(p_q_reflect, 0.9)
b_f8:stroke({{675, 535}, {702, 580}, {720, 618}, {716, 650}, {698, 670}}, {pressure = 0.5})

-- C. Luminous sunlit flank on left (golden yellow ochre + chrome yellow)
b_f14:load(p_q_lit, 1.0)
b_f14:stroke({{612, 490}, {585, 528}, {562, 575}, {556, 620}, {570, 658}, {602, 674}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})
b_f10:load(p_q_rich_gold, 1.0)
b_f10:stroke({{625, 505}, {605, 555}, {588, 610}, {612, 665}}, {pressure = 0.92})

-- Soft curved blend across the form to fuse light and shade
b_badger:stroke({{570, 580}, {630, 580}, {690, 580}}, {pressure = 0.16})
b_badger:stroke({{580, 630}, {635, 630}, {685, 630}}, {pressure = 0.16})

-- D. Buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_q_hi, 1.0)
b_r3:stroke({{575, 570}, {566, 598}, {570, 626}}, {pressure = {0.9, 1.0, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_q_hi, 1.0)
b_r1:touch(566, 597, {pressure = 1.0})

-- Secondary glint on upper lobe
b_r2:load(p_q_hi, 0.9)
b_r2:stroke({{606, 500}, {598, 516}}, {pressure = 0.85})

-- E. Crown socket, stem, and calyx
b_r2:load(p_q_deep, 1.0)
b_r2:touch(628, 488, {pressure = 0.95})

-- Gnarled woody stem
b_r2:load(p_stem, 1.0)
b_r2:stroke({{628, 488}, {634, 474}, {642, 460}, {652, 450}}, {pressure = {0.98, 0.9, 0.5}})
b_r1:load(p_q_lit, 0.85)
b_r1:stroke({{629, 486}, {635, 474}, {643, 461}}, {pressure = 0.6})

-- Calyx eye tucked at base
b_r2:load(p_q_deep, 1.0)
b_r2:touch(655, 676, {pressure = 0.95})

-- Dark occlusion contact shadow under the quince
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{580, 674}, {635, 678}, {680, 676}, {705, 668}}, {pressure = {0.85, 1.0, 0.9, 0.7}})

-- Transparent cast shadow of quince flowing rightward across the limestone
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{685, 676}, {735, 682}, {790, 688}}, {pressure = {0.8, 0.2}, swell = {1.0, 1.3, 0.6}})

print("Consolidation complete: bat-wings erased, pitcher and quince beautifully modeled.")

--@ chunk 164
-- CHUNK 164: THE BODEGÓN KNIFE, WALNUTS, AND REFINEMENT OF PITCHER AND QUINCE

-- 1. DEFINE ALL NEEDED PILES
-- Knife
p_steel_lit      = pile{{"lead white", 8.2}, {"bone black", 1.0}, {"smalt", 0.6}, {"raw umber", 0.4}, medium = 0.05}
p_steel_mid      = pile{{"lead white", 4.5}, {"bone black", 3.0}, {"smalt", 1.0}, {"raw umber", 1.5}, medium = 0.06}
p_steel_dark     = pile{{"bone black", 6.0}, {"raw umber", 3.0}, {"lead white", 1.0}, medium = 0.05}
p_brass          = pile{{"yellow ochre", 6.0}, {"red earth", 2.0}, {"raw umber", 2.0}, {"lead white", 1.5}, medium = 0.06}
p_brass_glint    = pile{{"lead white", 8.0}, {"yellow ochre", 3.0}, {"chrome yellow", 1.0}, medium = 0.04}
p_knife_wood     = pile{{"raw umber", 5.5}, {"red earth", 3.0}, {"bone black", 2.5}, {"yellow ochre", 1.0}, medium = 0.06}
p_knife_wood_lit = pile{{"red earth", 4.5}, {"yellow ochre", 3.5}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.06}

-- Walnuts
p_nut_shell      = pile{{"raw umber", 4.5}, {"yellow ochre", 4.0}, {"red earth", 2.5}, {"lead white", 1.5}, medium = 0.06}
p_nut_deep       = pile{{"raw umber", 6.0}, {"bone black", 3.0}, {"red earth", 1.5}, medium = 0.05}
p_nut_meat       = pile{{"lead white", 7.8}, {"yellow ochre", 2.8}, {"raw umber", 0.6}, medium = 0.05}
p_nut_meat_shade = pile{{"yellow ochre", 4.5}, {"raw umber", 3.0}, {"lead white", 3.0}, {"red earth", 0.8}, medium = 0.06}

-- Stage and adjustments
p_stone_mid      = pile{{"lead white", 7.5}, {"yellow ochre", 2.4}, {"raw umber", 1.0}, {"red earth", 0.2}, medium = 0.08}
p_stone_lit      = pile{{"lead white", 8.2}, {"yellow ochre", 2.0}, {"raw umber", 0.5}, medium = 0.08}
p_shadow_stone   = pile{{"raw umber", 4.8}, {"yellow ochre", 3.2}, {"lead white", 2.2}, {"red earth", 0.6}, medium = 0.10}

-- Cantaro and Quince
p_jug_rich       = pile{{"red earth", 6.2}, {"yellow ochre", 3.8}, {"lead white", 1.2}, {"raw umber", 1.0}, medium = 0.06}
p_jug_deep       = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.8}, medium = 0.05}
p_jug_hi         = pile{{"lead white", 9.0}, {"yellow ochre", 1.5}, {"red earth", 0.2}, medium = 0.04}
p_q_rich_gold    = pile{{"yellow ochre", 5.5}, {"chrome yellow", 4.5}, {"lead white", 2.0}, {"green earth", 0.5}, medium = 0.06}
p_q_lit          = pile{{"chrome yellow", 5.5}, {"yellow ochre", 2.5}, {"lead white", 5.5}, medium = 0.05}
p_q_core         = pile{{"raw umber", 5.0}, {"yellow ochre", 3.5}, {"red earth", 2.5}, {"bone black", 0.8}, medium = 0.06}
p_q_hi           = pile{{"lead white", 9.0}, {"chrome yellow", 2.5}, {"yellow ochre", 0.6}, medium = 0.04}

local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_f8  = brush("filbert", 8)
local b_r5  = brush{kind = "round", width = 5.5, point = 0.9}
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 2. REFINING THE PITCHER: BURY THE THREE DASHES & CLEAN TABLE
-- Broad rich terracotta sweep down left belly covers dashes completely
b_f14:load(p_jug_rich, 1.0)
b_f14:stroke({{315, 410}, {280, 470}, {270, 500}, {285, 545}, {310, 595}}, {pressure = 1.0})

-- Handle lower return solidly anchored
b_r5:load(p_jug_rich, 1.0)
b_r5:stroke({{230, 475}, {255, 515}, {285, 545}}, {pressure = 1.0})

-- Clean table to the left of the jug (x = 180 to 265, y = 560 to 625)
for y = 562, 622, 10 do
  b_f14:load(p_stone_mid, 1.0)
  b_f14:stroke({{170, y}, {220, y}, {265, y}}, {pressure = 0.98})
end

-- Decisive glaze impasto on pitcher shoulder
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{324, 402}, {320, 418}, {316, 436}}, {pressure = {0.7, 1.0, 0.7}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_jug_hi, 1.0)
b_r1:touch(320, 418, {pressure = 1.0})

-- 3. REFINING THE QUINCE: OBLITERATE BADGER GROOVES & SCULPT KNOPPY FLESH
-- Solid vertical golden body strokes down front center to bury badger grooves
b_f14:load(p_q_rich_gold, 1.0)
b_f14:stroke({{630, 500}, {615, 560}, {605, 620}, {620, 672}}, {pressure = 1.0})

-- Sunlit flank on left (golden yellow ochre and chrome yellow)
b_f14:load(p_q_lit, 1.0)
b_f14:stroke({{612, 490}, {585, 530}, {562, 575}, {556, 620}, {570, 658}, {602, 674}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Deep warm amber core shadow down right flank
b_f14:load(p_q_core, 1.0)
b_f14:stroke({{645, 495}, {672, 540}, {696, 590}, {700, 635}, {678, 668}, {645, 675}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.9, 1.35, 0.9}
})

-- Tactile buttery impasto highlight on the knobby crest
b_r3:load(p_q_hi, 1.0)
b_r3:stroke({{575, 570}, {566, 598}, {570, 626}}, {pressure = {0.9, 1.0, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_q_hi, 1.0)
b_r1:touch(566, 597, {pressure = 1.0})

-- Dark contact shadow under base
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{580, 674}, {635, 678}, {680, 676}, {705, 668}}, {pressure = {0.85, 1.0, 0.9, 0.7}})

-- Transparent cast shadow
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{685, 676}, {735, 682}, {790, 688}}, {pressure = {0.8, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 4. PAINTING THE BODEGÓN KNIFE (STEEL BLADE, BRASS BOLSTER, TURNED HANDLE)
-- Steel blade body (from point {660, 684} to bolster {772, 715})
b_f8:load(p_steel_mid, 1.0)
b_f8:stroke({{660, 684}, {715, 699}, {772, 715}}, {pressure = 0.98})

-- Steel spine (dark upper edge of blade)
b_r1:load(p_steel_dark, 1.0)
b_r1:stroke({{660, 684}, {715, 698}, {770, 713}}, {pressure = 0.95})

-- Reflected north light along blade bevel
b_r2:load(p_steel_lit, 1.0)
b_r2:stroke({{665, 685}, {718, 700}, {771, 715}}, {pressure = 0.95})

-- Razor-sharp specular cutting edge
b_r1:load(p_steel_lit, 1.0)
b_r1:stroke({{660, 684}, {715, 701}, {772, 716}}, {pressure = 1.0})

-- Blade contact shadow on limestone
b_r2:load(p_shadow_stone, 0.8)
b_r2:stroke({{662, 686}, {715, 703}, {770, 718}}, {pressure = {0.5, 0.85}})

-- Brass bolster collar ({770, 713} to {784, 717})
b_r3:load(p_brass, 1.0)
b_r3:stroke({{770, 713}, {777, 715}, {784, 717}}, {pressure = 1.0})
b_r1:load(p_brass_glint, 1.0)
b_r1:touch(777, 714, {pressure = 1.0})
b_r1:load(p_steel_dark, 0.9)
b_r1:stroke({{770, 716}, {784, 719}}, {pressure = 0.85})

-- Turned dark wood handle projecting over the front ledge ({784, 717} to {855, 755})
b_r5:load(p_knife_wood, 1.0)
b_r5:stroke({{784, 717}, {820, 736}, {855, 755}}, {pressure = 1.0, swell = {0.9, 1.25, 0.95}})

-- Lit upper crest of turned handle
b_r3:load(p_knife_wood_lit, 1.0)
b_r3:stroke({{785, 715}, {820, 734}, {854, 753}}, {pressure = 0.9})

-- Brass pommel cap glint at end of handle
b_r1:load(p_brass_glint, 1.0)
b_r1:touch(855, 755, {pressure = 1.0})

-- Handle shadow falling down the front vertical ledge face
b_r3:load(p_jug_deep, 0.85)
b_r3:stroke({{788, 717}, {792, 738}, {796, 760}}, {pressure = {0.9, 0.25}})

-- Specular chamfer highlight around knife
b_r1:load(p_jug_hi, 0.95)
b_r1:stroke({{-5, 715}, {350, 715}, {770, 715}}, {pressure = {0.85, 0.85}, shake = 0.02})
b_r1:stroke({{784, 715}, {900, 715}, {1005, 715}}, {pressure = {0.85, 0.85}, shake = 0.02})

-- 5. PAINTING FOREGROUND WALNUTS (BETWEEN CANTARO AND QUINCE)
-- A. Whole intact walnut at {475, 642}
-- Woody shell mass
b_r5:load(p_nut_shell, 1.0)
b_r5:stroke({{468, 644}, {475, 642}, {484, 644}}, {pressure = 1.0, swell = {0.85, 1.3, 0.85}})

-- Corrugated ridges & suture line
b_r2:load(p_nut_deep, 0.95)
b_r2:stroke({{468, 644}, {476, 641}, {484, 644}}, {pressure = 0.85})
b_r1:load(p_jug_hi, 0.9)
b_r1:stroke({{472, 640}, {476, 639}, {480, 641}}, {pressure = 0.8})

-- Shell contact shadow
b_r2:load(p_jug_deep, 0.95)
b_r2:stroke({{466, 647}, {476, 648}, {485, 647}}, {pressure = 0.85})
b_f8:load(p_shadow_stone, 0.5)
b_f8:stroke({{480, 646}, {500, 650}}, {pressure = {0.7, 0.2}})

-- B. Cracked walnut half with ivory kernel at {505, 652}
-- Woody shell cup
b_r5:load(p_nut_shell, 1.0)
b_r5:stroke({{496, 654}, {506, 656}, {518, 652}}, {pressure = 1.0, swell = {0.9, 1.3, 0.85}})

-- Shell rim and shadow inside cup
b_r2:load(p_nut_deep, 1.0)
b_r2:stroke({{496, 652}, {506, 650}, {518, 650}}, {pressure = 0.95})

-- Convoluted ivory kernel lobes
b_r3:load(p_nut_meat_shade, 1.0)
b_r3:stroke({{500, 651}, {508, 650}, {514, 651}}, {pressure = 0.95})
b_r2:load(p_nut_meat, 1.0)
b_r2:stroke({{499, 650}, {504, 649}, {507, 651}}, {pressure = 0.95})
b_r2:load(p_nut_meat, 1.0)
b_r2:stroke({{508, 651}, {512, 649}, {515, 650}}, {pressure = 0.95})
b_r1:load(p_jug_hi, 1.0)
b_r1:touch(503, 649, {pressure = 1.0})
b_r1:touch(511, 649, {pressure = 1.0})

-- Contact and cast shadow of walnut half
b_r2:load(p_jug_deep, 0.95)
b_r2:stroke({{496, 656}, {508, 658}, {520, 654}}, {pressure = 0.9})
b_f8:load(p_shadow_stone, 0.5)
b_f8:stroke({{515, 654}, {535, 657}}, {pressure = {0.7, 0.2}})

print("Knife, walnuts, pitcher and quince consolidated.")

--@ chunk 165
-- CHUNK 165: DRYING UNDERLAYERS AND UNIFYING THE STAGE

-- 1. Advance clock by 14 days so all underlayers are touch-dry
local t = wait(14 * 24 * 60)
print("Underlayers dried. Time:", t)

-- 2. Define clean stage piles
p_wall       = pile{{"raw umber", 6.5}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.08}
p_stone_back = pile{{"lead white", 6.5}, {"yellow ochre", 2.8}, {"raw umber", 1.5}, {"red earth", 0.3}, medium = 0.08}
p_stone_mid  = pile{{"lead white", 7.5}, {"yellow ochre", 2.4}, {"raw umber", 1.0}, {"red earth", 0.2}, medium = 0.08}
p_stone_lit  = pile{{"lead white", 8.2}, {"yellow ochre", 2.0}, {"raw umber", 0.5}, medium = 0.08}
p_ledge      = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.2}, medium = 0.05}
p_chamfer    = pile{{"lead white", 8.8}, {"yellow ochre", 1.8}, medium = 0.05}

local b_f26 = brush("filbert", 26)
local b_f20 = brush("filbert", 20)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_badger = brush("badger", 32)
local b_r1 = brush{kind = "round", width = 1.4, point = 0.98}

-- 3. UNIFY LEFT TABLETOP (Eliminates rectangular patch at x = 170 to 280)
for y = 522, 630, 8 do
  local p = (y < 570) and p_stone_back or p_stone_mid
  b_f20:load(p, 1.0)
  b_f20:stroke({{-15, y}, {120, y}, {270, y}}, {pressure = 0.98})
end
b_badger:stroke({{-15, 575}, {270, 575}}, {pressure = 0.22})

-- 4. CLEAN WALL & HANDLE NEGATIVE SPACE (Erases the 3 stray dashes on the left)
-- Dark wall behind upper handle and neck
b_f14:load(p_wall, 1.0)
b_f14:stroke({{-15, 480}, {120, 480}, {220, 480}}, {pressure = 0.98})
b_f14:stroke({{-15, 510}, {120, 510}, {220, 510}}, {pressure = 0.98})

-- Negative space inside handle loop (x = 245 to 300, y = 410 to 520)
b_f10:load(p_wall, 1.0)
b_f10:stroke({{250, 435}, {275, 430}, {300, 420}}, {pressure = 0.98})
b_f10:stroke({{255, 465}, {280, 465}, {300, 460}}, {pressure = 0.98})
b_f10:stroke({{265, 495}, {285, 495}, {295, 495}}, {pressure = 0.98})

-- Below y = 520 in handle negative space (table plane shows)
b_f10:load(p_stone_back, 1.0)
b_f10:stroke({{270, 525}, {285, 525}}, {pressure = 0.95})

-- 5. REFRESH CHAMFER LINE ALONG FRONT RIM (y = 715)
b_r1:load(p_chamfer, 1.0)
b_r1:stroke({{-10, 715}, {300, 715}, {650, 715}, {1015, 715}}, {pressure = {0.85, 0.9, 0.85}, shake = 0.02})

print("Stage cleaned and dried: left table seamless, stray dashes erased.")

--@ chunk 166
-- CHUNK 166: THE EARTHENWARE CÁNTARO AND VOLUPTUOUS QUINCE CONSOLIDATION

-- Palette piles
p_jug_rich   = pile{{"red earth", 6.2}, {"yellow ochre", 3.8}, {"lead white", 1.2}, {"raw umber", 1.0}, medium = 0.06}
p_jug_lit    = pile{{"yellow ochre", 6.0}, {"red earth", 2.5}, {"lead white", 4.5}, {"raw umber", 0.5}, medium = 0.06}
p_jug_core   = pile{{"red earth", 5.5}, {"raw umber", 4.5}, {"bone black", 1.2}, {"yellow ochre", 1.0}, medium = 0.06}
p_jug_deep   = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.8}, medium = 0.05}
p_jug_reflect= pile{{"yellow ochre", 5.0}, {"red earth", 2.0}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.08}
p_jug_hi     = pile{{"lead white", 9.0}, {"yellow ochre", 1.5}, {"red earth", 0.2}, medium = 0.04}

p_q_rich_gold= pile{{"yellow ochre", 5.5}, {"chrome yellow", 4.5}, {"lead white", 2.2}, {"green earth", 0.5}, medium = 0.06}
p_q_lit      = pile{{"chrome yellow", 5.5}, {"yellow ochre", 2.5}, {"lead white", 5.5}, medium = 0.05}
p_q_half     = pile{{"yellow ochre", 5.0}, {"green earth", 2.0}, {"raw umber", 1.5}, {"lead white", 2.2}, medium = 0.08}
p_q_core     = pile{{"raw umber", 5.0}, {"yellow ochre", 3.5}, {"red earth", 2.5}, {"bone black", 0.8}, medium = 0.06}
p_q_deep     = pile{{"raw umber", 6.0}, {"bone black", 3.0}, {"red earth", 1.8}, medium = 0.05}
p_q_reflect  = pile{{"yellow ochre", 5.5}, {"lead white", 3.0}, {"raw umber", 1.5}, {"red earth", 0.8}, medium = 0.08}
p_q_hi       = pile{{"lead white", 9.0}, {"chrome yellow", 2.5}, {"yellow ochre", 0.6}, medium = 0.04}
p_stem       = pile{{"raw umber", 6.0}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.05}

p_shadow_stone = pile{{"raw umber", 4.8}, {"yellow ochre", 3.2}, {"lead white", 2.2}, {"red earth", 0.6}, medium = 0.10}

local b_f20 = brush("filbert", 20)
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_f8  = brush("filbert", 8)
local b_r5  = brush{kind = "round", width = 5.5, point = 0.9}
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. EARTHENWARE CÁNTARO: BURY THREE TABS & PERFECT BELLY
-- A. Left flank sweep covers x = 270 to 295 at y = 500 to 580, completely burying the tabs
b_f18:load(p_jug_rich, 1.0)
b_f18:stroke({{330, 400}, {280, 475}, {270, 520}, {282, 565}, {315, 618}}, {pressure = 1.0})

-- B. Central belly sweep
b_f18:load(p_jug_rich, 1.0)
b_f18:stroke({{355, 365}, {365, 485}, {370, 622}}, {pressure = 1.0})

-- C. Lit flank (raking north light)
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{340, 365}, {318, 420}, {300, 480}, {310, 545}, {330, 600}, {345, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- D. Core shadow down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{395, 365}, {425, 425}, {455, 485}, {445, 545}, {426, 595}, {408, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- E. Deep shadow along right contour
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{400, 360}, {422, 400}, {450, 440}, {468, 485}, {456, 540}, {436, 595}, {412, 624}}, {
  pressure = 0.9
})

-- F. Ambient table bounce
b_f8:load(p_jug_reflect, 0.9)
b_f8:stroke({{422, 405}, {452, 445}, {468, 485}, {458, 540}, {438, 590}, {414, 622}}, {pressure = 0.45})

-- G. Sturdy strap handle
local pts_h_outer = {{342, 365}, {295, 375}, {248, 395}, {222, 430}, {230, 475}, {255, 515}, {284, 542}}
b_r5:load(p_jug_rich, 1.0)
b_r5:stroke(pts_h_outer, {pressure = 1.0, swell = {0.9, 1.35, 1.0}})

local pts_h_under = {{338, 375}, {295, 385}, {253, 408}, {232, 438}, {240, 480}, {264, 518}, {285, 542}}
b_r3:load(p_jug_core, 1.0)
b_r3:stroke(pts_h_under, {pressure = 0.95})

b_r3:load(p_jug_lit, 1.0)
b_r3:stroke({{340, 363}, {293, 372}, {244, 392}, {220, 428}, {226, 465}}, {pressure = {0.8, 1.0, 0.8}})

b_r1:load(p_jug_hi, 1.0)
b_r1:stroke({{238, 398}, {222, 426}, {224, 442}}, {pressure = {0.5, 0.95, 0.5}})

-- H. Specular glaze highlight on shoulder
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{324, 402}, {320, 418}, {316, 436}}, {pressure = {0.7, 1.0, 0.7}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_jug_hi, 1.0)
b_r1:touch(320, 418, {pressure = 1.0})

-- I. Foot ring and contact shadow
b_r3:load(p_jug_rich, 1.0)
b_r3:stroke({{326, 624}, {370, 626}, {414, 624}}, {pressure = 0.95})
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{322, 626}, {370, 628}, {418, 626}}, {pressure = {0.95, 1.0, 0.95}})

-- Pitcher cast shadow flowing rightward
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{414, 625}, {465, 628}, {535, 636}}, {pressure = {0.85, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 2. VOLUPTUOUS RIPE QUINCE (MEMBRILLO)
local pts_quince = {
  {632, 485}, {655, 495}, {685, 520}, {715, 560}, {736, 600},
  {740, 640}, {718, 670}, {680, 678}, {630, 680}, {585, 675},
  {555, 655}, {538, 625}, {535, 585}, {550, 540}, {576, 508},
  {605, 488}, {622, 484}
}
local m_q = poly(pts_quince, true)

-- A. Dense golden body coat (obliterates badger lines completely)
work(m_q, {
  hand = "body",
  pile = p_q_rich_gold,
  angle = 1.35,
  coverage = 3.5,
  fill = true,
  edge = "firm"
})

-- B. Sculpting the swelling lobes
-- Lit sunlit flank (golden yellow ochre and chrome yellow)
b_f18:load(p_q_lit, 1.0)
b_f18:stroke({{610, 490}, {580, 530}, {555, 580}, {552, 625}, {568, 660}, {600, 676}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Half-tone in central hollow
b_f10:load(p_q_half, 0.95)
b_f10:stroke({{625, 505}, {605, 560}, {590, 615}, {615, 668}}, {pressure = 0.92})

-- Deep warm amber core shadow down right flank
b_f18:load(p_q_core, 1.0)
b_f18:stroke({{645, 495}, {680, 545}, {710, 595}, {715, 640}, {690, 672}, {650, 678}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.9, 1.35, 0.9}
})

-- Deep shadow near right contour
b_f10:load(p_q_deep, 0.95)
b_f10:stroke({{675, 525}, {705, 570}, {730, 615}, {728, 650}, {705, 672}}, {pressure = 0.9})

-- Reflected limestone bounce along shaded contour
b_f10:load(p_q_reflect, 0.9)
b_f10:stroke({{685, 538}, {715, 585}, {734, 620}, {730, 652}, {710, 672}}, {pressure = 0.45})

-- C. Buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_q_hi, 1.0)
b_r3:stroke({{570, 565}, {562, 595}, {566, 625}}, {pressure = {0.9, 1.0, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_q_hi, 1.0)
b_r1:touch(562, 595, {pressure = 1.0})

-- Secondary glint on upper shoulder
b_r2:load(p_q_hi, 0.9)
b_r2:stroke({{606, 500}, {598, 516}}, {pressure = 0.85})

-- D. Crown socket and gnarled woody stem
b_r2:load(p_q_deep, 1.0)
b_r2:touch(628, 486, {pressure = 0.95})

b_r2:load(p_stem, 1.0)
b_r2:stroke({{628, 486}, {634, 472}, {642, 458}, {652, 448}}, {pressure = {0.98, 0.9, 0.5}})
b_r1:load(p_q_lit, 0.85)
b_r1:stroke({{629, 484}, {635, 472}, {643, 459}}, {pressure = 0.6})

-- Calyx eye tucked at base
b_r2:load(p_q_deep, 1.0)
b_r2:touch(655, 678, {pressure = 0.95})

-- Dark occlusion contact shadow under quince base
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{575, 676}, {630, 680}, {680, 678}, {715, 670}}, {pressure = {0.85, 1.0, 0.9, 0.7}})

-- Soft transparent cast shadow flowing rightward across limestone
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{700, 678}, {750, 684}, {810, 692}}, {pressure = {0.8, 0.2}, swell = {1.0, 1.3, 0.6}})

print("Earthenware cántaro and voluptuous quince fully consolidated.")

--@ chunk 167
-- CHUNK 167: MASTER CONSOLIDATION OF THE COMPLETE STILL LIFE

-- 1. PALETTE PILES
-- Cantaro
p_jug_rich   = pile{{"red earth", 6.2}, {"yellow ochre", 3.8}, {"lead white", 1.2}, {"raw umber", 1.0}, medium = 0.06}
p_jug_lit    = pile{{"yellow ochre", 6.0}, {"red earth", 2.5}, {"lead white", 4.5}, {"raw umber", 0.5}, medium = 0.06}
p_jug_core   = pile{{"red earth", 5.5}, {"raw umber", 4.5}, {"bone black", 1.2}, {"yellow ochre", 1.0}, medium = 0.06}
p_jug_deep   = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.8}, medium = 0.05}
p_jug_reflect= pile{{"yellow ochre", 5.0}, {"red earth", 2.0}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.08}
p_jug_hi     = pile{{"lead white", 9.0}, {"yellow ochre", 1.5}, {"red earth", 0.2}, medium = 0.04}

-- Quince
p_q_rich_gold= pile{{"yellow ochre", 5.5}, {"chrome yellow", 4.5}, {"lead white", 2.2}, {"green earth", 0.5}, medium = 0.06}
p_q_lit      = pile{{"chrome yellow", 5.5}, {"yellow ochre", 2.5}, {"lead white", 5.5}, medium = 0.05}
p_q_half     = pile{{"yellow ochre", 5.0}, {"green earth", 2.0}, {"raw umber", 1.5}, {"lead white", 2.2}, medium = 0.08}
p_q_core     = pile{{"raw umber", 5.0}, {"yellow ochre", 3.5}, {"red earth", 2.5}, {"bone black", 0.8}, medium = 0.06}
p_q_deep     = pile{{"raw umber", 6.0}, {"bone black", 3.0}, {"red earth", 1.8}, medium = 0.05}
p_q_reflect  = pile{{"yellow ochre", 5.5}, {"lead white", 3.0}, {"raw umber", 1.5}, {"red earth", 0.8}, medium = 0.08}
p_q_hi       = pile{{"lead white", 9.0}, {"chrome yellow", 2.5}, {"yellow ochre", 0.6}, medium = 0.04}
p_stem       = pile{{"raw umber", 6.0}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.05}

-- Knife
p_steel_lit      = pile{{"lead white", 8.2}, {"bone black", 1.0}, {"smalt", 0.6}, {"raw umber", 0.4}, medium = 0.05}
p_steel_mid      = pile{{"lead white", 4.8}, {"bone black", 3.0}, {"smalt", 1.0}, {"raw umber", 1.2}, medium = 0.06}
p_steel_dark     = pile{{"bone black", 6.0}, {"raw umber", 3.0}, {"lead white", 1.0}, medium = 0.05}
p_brass          = pile{{"yellow ochre", 6.0}, {"red earth", 2.0}, {"raw umber", 2.0}, {"lead white", 1.5}, medium = 0.06}
p_brass_glint    = pile{{"lead white", 8.0}, {"yellow ochre", 3.0}, {"chrome yellow", 1.0}, medium = 0.04}
p_knife_wood     = pile{{"raw umber", 5.5}, {"red earth", 3.0}, {"bone black", 2.5}, {"yellow ochre", 1.0}, medium = 0.06}
p_knife_wood_lit = pile{{"red earth", 4.5}, {"yellow ochre", 3.5}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.06}

-- Walnuts
p_nut_shell      = pile{{"raw umber", 4.5}, {"yellow ochre", 4.0}, {"red earth", 2.5}, {"lead white", 1.5}, medium = 0.06}
p_nut_deep       = pile{{"raw umber", 6.0}, {"bone black", 3.0}, {"red earth", 1.5}, medium = 0.05}
p_nut_meat       = pile{{"lead white", 7.8}, {"yellow ochre", 2.8}, {"raw umber", 0.6}, medium = 0.05}
p_nut_meat_shade = pile{{"yellow ochre", 4.5}, {"raw umber", 3.0}, {"lead white", 3.0}, {"red earth", 0.8}, medium = 0.06}

-- Stage & shadows
p_wall         = pile{{"raw umber", 6.5}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.08}
p_shadow_stone = pile{{"raw umber", 4.8}, {"yellow ochre", 3.2}, {"lead white", 2.2}, {"red earth", 0.6}, medium = 0.10}
p_chamfer      = pile{{"lead white", 8.8}, {"yellow ochre", 1.8}, medium = 0.05}

local b_f20 = brush("filbert", 20)
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_f8  = brush("filbert", 8)
local b_r5  = brush{kind = "round", width = 5.5, point = 0.9}
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 2. EARTHENWARE CÁNTARO: COMPLETELY OBLITERATE THE THREE DARK TABS
-- Broad opaque terracotta fill across x = 250 to 320 at y = 420 to 550
for y = 430, 545, 8 do
  local xl = 265
  local xr = 325
  b_f14:load(p_jug_rich, 1.0)
  b_f14:stroke({{xl, y}, {xr, y}}, {pressure = 1.0})
end

-- Long vertical sweep down the entire left flank
b_f18:load(p_jug_rich, 1.0)
b_f18:stroke({{330, 390}, {280, 460}, {268, 510}, {280, 560}, {312, 615}}, {pressure = 1.0})

-- Central belly sweep
b_f18:load(p_jug_rich, 1.0)
b_f18:stroke({{355, 365}, {365, 485}, {370, 622}}, {pressure = 1.0})

-- Lit flank (raking north light)
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{340, 365}, {318, 420}, {300, 480}, {310, 545}, {330, 600}, {345, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Core shadow down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{395, 365}, {425, 425}, {455, 485}, {445, 545}, {426, 595}, {408, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Deep shadow on right contour
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{400, 360}, {422, 400}, {450, 440}, {468, 485}, {456, 540}, {436, 595}, {412, 624}}, {
  pressure = 0.9
})

-- Ambient table bounce along right contour
b_f8:load(p_jug_reflect, 0.9)
b_f8:stroke({{422, 405}, {452, 445}, {468, 485}, {458, 540}, {438, 590}, {414, 622}}, {pressure = 0.45})

-- Sturdy pulled strap handle
local pts_h_outer = {{342, 365}, {295, 375}, {248, 395}, {222, 430}, {230, 475}, {255, 515}, {284, 542}}
b_r5:load(p_jug_rich, 1.0)
b_r5:stroke(pts_h_outer, {pressure = 1.0, swell = {0.9, 1.35, 1.0}})

local pts_h_under = {{338, 375}, {295, 385}, {253, 408}, {232, 438}, {240, 480}, {264, 518}, {285, 542}}
b_r3:load(p_jug_core, 1.0)
b_r3:stroke(pts_h_under, {pressure = 0.95})

b_r3:load(p_jug_lit, 1.0)
b_r3:stroke({{340, 363}, {293, 372}, {244, 392}, {220, 428}, {226, 465}}, {pressure = {0.8, 1.0, 0.8}})

b_r1:load(p_jug_hi, 1.0)
b_r1:stroke({{238, 398}, {222, 426}, {224, 442}}, {pressure = {0.5, 0.95, 0.5}})

-- North light glaze impasto on shoulder curve
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{324, 402}, {320, 418}, {316, 436}}, {pressure = {0.7, 1.0, 0.7}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_jug_hi, 1.0)
b_r1:touch(320, 418, {pressure = 1.0})

-- Foot ring and contact shadow
b_r3:load(p_jug_rich, 1.0)
b_r3:stroke({{326, 624}, {370, 626}, {414, 624}}, {pressure = 0.95})
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{322, 626}, {370, 628}, {418, 626}}, {pressure = {0.95, 1.0, 0.95}})

-- Pitcher cast shadow flowing rightward
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{414, 625}, {465, 628}, {535, 636}}, {pressure = {0.85, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 3. RIPE GOLDEN QUINCE REFINEMENT
-- Sunlit flank (chrome yellow + yellow ochre)
b_f18:load(p_q_lit, 1.0)
b_f18:stroke({{610, 490}, {580, 530}, {555, 580}, {552, 625}, {568, 660}, {600, 676}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Half-tone in central hollow
b_f10:load(p_q_half, 0.95)
b_f10:stroke({{625, 505}, {605, 560}, {590, 615}, {615, 668}}, {pressure = 0.92})

-- Deep warm amber core shadow down right flank
b_f18:load(p_q_core, 1.0)
b_f18:stroke({{645, 495}, {680, 545}, {710, 595}, {715, 640}, {690, 672}, {650, 678}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.9, 1.35, 0.9}
})

-- Deep shadow near right contour
b_f10:load(p_q_deep, 0.95)
b_f10:stroke({{675, 525}, {705, 570}, {730, 615}, {728, 650}, {705, 672}}, {pressure = 0.9})

-- Reflected limestone bounce
b_f10:load(p_q_reflect, 0.9)
b_f10:stroke({{685, 538}, {715, 585}, {734, 620}, {730, 652}, {710, 672}}, {pressure = 0.45})

-- Buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_q_hi, 1.0)
b_r3:stroke({{570, 565}, {562, 595}, {566, 625}}, {pressure = {0.9, 1.0, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_q_hi, 1.0)
b_r1:touch(562, 595, {pressure = 1.0})

-- Crown socket and gnarled woody stem
b_r2:load(p_q_deep, 1.0)
b_r2:touch(628, 486, {pressure = 0.95})
b_r2:load(p_stem, 1.0)
b_r2:stroke({{628, 486}, {634, 472}, {642, 458}, {652, 448}}, {pressure = {0.98, 0.9, 0.5}})
b_r1:load(p_q_lit, 0.85)
b_r1:stroke({{629, 484}, {635, 472}, {643, 459}}, {pressure = 0.6})

-- Calyx eye tucked at base
b_r2:load(p_q_deep, 1.0)
b_r2:touch(655, 678, {pressure = 0.95})

-- Contact shadow under base
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{575, 676}, {630, 680}, {680, 678}, {715, 670}}, {pressure = {0.85, 1.0, 0.9, 0.7}})

-- Transparent cast shadow
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{700, 678}, {750, 684}, {810, 692}}, {pressure = {0.8, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 4. SUBSTANTIAL BODEGÓN KNIFE (FORGED STEEL, BRASS BOLSTER, TURNED WALNUT HANDLE)
-- Blade flat (substantial triangular blade from {640, 680} to {765, 715})
local pts_blade = {{640, 680}, {762, 710}, {765, 716}, {640, 680}}
b_f14:load(p_steel_mid, 1.0)
b_f14:stroke({{640, 680}, {700, 696}, {765, 714}}, {pressure = 1.0})

-- Steel spine (dark forged back)
b_r2:load(p_steel_dark, 1.0)
b_r2:stroke({{640, 680}, {700, 695}, {763, 711}}, {pressure = 0.95})

-- Reflected north light across blade bevel
b_r3:load(p_steel_lit, 1.0)
b_r3:stroke({{645, 681}, {702, 697}, {764, 713}}, {pressure = 0.95})

-- Razor cutting edge
b_r1:load(p_steel_lit, 1.0)
b_r1:stroke({{640, 680}, {700, 698}, {765, 715}}, {pressure = 1.0})

-- Blade contact shadow on limestone
b_r2:load(p_shadow_stone, 0.85)
b_r2:stroke({{642, 682}, {702, 700}, {764, 717}}, {pressure = {0.5, 0.9}})

-- Heavy brass bolster collar ({763, 711} to {778, 716})
b_r5:load(p_brass, 1.0)
b_r5:stroke({{763, 711}, {771, 713}, {778, 716}}, {pressure = 1.0})
b_r2:load(p_brass_glint, 1.0)
b_r2:touch(771, 713, {pressure = 1.0})
b_r1:load(p_steel_dark, 0.9)
b_r1:stroke({{764, 715}, {778, 718}}, {pressure = 0.85})

-- Substantial turned dark walnut handle projecting over front ledge ({778, 716} to {855, 755})
b_r5:load(p_knife_wood, 1.0)
b_r5:stroke({{778, 716}, {815, 735}, {855, 755}}, {pressure = 1.0, swell = {1.0, 1.4, 1.0}})

-- Lit upper crest of turned handle
b_r3:load(p_knife_wood_lit, 1.0)
b_r3:stroke({{779, 714}, {815, 733}, {854, 753}}, {pressure = 0.92})

-- Turned brass pommel cap at end of handle
b_r2:load(p_brass_glint, 1.0)
b_r2:touch(855, 755, {pressure = 1.0})

-- Handle shadow falling down the front vertical ledge face
b_r5:load(p_jug_deep, 0.85)
b_r5:stroke({{782, 716}, {786, 738}, {790, 762}}, {pressure = {0.9, 0.25}})

-- Specular chamfer highlight around knife
b_r1:load(p_chamfer, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {763, 715}}, {pressure = {0.85, 0.85}, shake = 0.02})
b_r1:stroke({{778, 715}, {900, 715}, {1005, 715}}, {pressure = {0.85, 0.85}, shake = 0.02})

-- 5. SUBSTANTIAL WALNUTS (REAL BODÉGON SCALE)
-- A. Whole ribbed walnut at {470, 642} (width ~26 units)
b_r5:load(p_nut_shell, 1.0)
b_r5:stroke({{458, 644}, {470, 640}, {482, 644}}, {pressure = 1.0, swell = {0.9, 1.4, 0.9}})
b_r5:load(p_nut_shell, 1.0)
b_r5:stroke({{460, 646}, {471, 644}, {482, 646}}, {pressure = 1.0})

-- Suture line and ribbed shell crevices
b_r2:load(p_nut_deep, 0.95)
b_r2:stroke({{458, 644}, {470, 639}, {482, 644}}, {pressure = 0.85})
b_r1:load(p_q_hi, 0.9)
b_r1:stroke({{465, 638}, {471, 637}, {477, 639}}, {pressure = 0.85})

-- Contact and cast shadow of whole walnut
b_r3:load(p_jug_deep, 0.95)
b_r3:stroke({{456, 648}, {470, 650}, {484, 648}}, {pressure = 0.9})
b_f8:load(p_shadow_stone, 0.55)
b_f8:stroke({{478, 647}, {505, 651}}, {pressure = {0.8, 0.2}})

-- B. Cracked walnut half with ivory kernel at {515, 652} (width ~28 units)
-- Woody shell cup
b_r5:load(p_nut_shell, 1.0)
b_r5:stroke({{502, 655}, {515, 658}, {528, 653}}, {pressure = 1.0, swell = {0.9, 1.4, 0.9}})

-- Dark shadow inside shell cup
b_r3:load(p_nut_deep, 1.0)
b_r3:stroke({{504, 652}, {515, 650}, {526, 650}}, {pressure = 0.95})

-- Convoluted ivory kernel lobes in thick impasto
b_r3:load(p_nut_meat_shade, 1.0)
b_r3:stroke({{506, 651}, {515, 650}, {524, 651}}, {pressure = 0.95})

b_r2:load(p_nut_meat, 1.0)
b_r2:stroke({{506, 650}, {510, 648}, {514, 650}}, {pressure = 1.0})
b_r2:load(p_nut_meat, 1.0)
b_r2:stroke({{515, 650}, {519, 648}, {523, 650}}, {pressure = 1.0})

b_r1:load(p_q_hi, 1.0)
b_r1:touch(509, 648, {pressure = 1.0})
b_r1:touch(518, 648, {pressure = 1.0})

-- Shell rim highlight
b_r1:load(p_q_lit, 0.9)
b_r1:stroke({{502, 654}, {515, 657}, {528, 652}}, {pressure = 0.85})

-- Contact and cast shadow of walnut half
b_r3:load(p_jug_deep, 0.95)
b_r3:stroke({{502, 658}, {516, 660}, {530, 655}}, {pressure = 0.9})
b_f8:load(p_shadow_stone, 0.55)
b_f8:stroke({{525, 656}, {550, 659}}, {pressure = {0.8, 0.2}})

print("Master consolidation executed: tabs erased, knife and walnuts substantial, composition unified.")

--@ chunk 168
-- CHUNK 168: REFINING PITCHER, QUINCE, AND TABLETOP HARMONY

-- Palette piles
p_wall       = pile{{"raw umber", 6.5}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.08}
p_stone_lit  = pile{{"lead white", 8.2}, {"yellow ochre", 2.0}, {"raw umber", 0.5}, medium = 0.08}
p_stone_mid  = pile{{"lead white", 7.5}, {"yellow ochre", 2.4}, {"raw umber", 1.0}, {"red earth", 0.2}, medium = 0.08}
p_shadow_stone = pile{{"raw umber", 4.8}, {"yellow ochre", 3.2}, {"lead white", 2.2}, {"red earth", 0.6}, medium = 0.10}

p_jug_rich   = pile{{"red earth", 6.2}, {"yellow ochre", 3.8}, {"lead white", 1.2}, {"raw umber", 1.0}, medium = 0.06}
p_jug_lit    = pile{{"yellow ochre", 6.0}, {"red earth", 2.5}, {"lead white", 4.5}, {"raw umber", 0.5}, medium = 0.06}
p_jug_core   = pile{{"red earth", 5.5}, {"raw umber", 4.5}, {"bone black", 1.2}, {"yellow ochre", 1.0}, medium = 0.06}
p_jug_deep   = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.8}, medium = 0.05}
p_jug_reflect= pile{{"yellow ochre", 5.0}, {"red earth", 2.0}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.08}
p_jug_hi     = pile{{"lead white", 9.0}, {"yellow ochre", 1.5}, {"red earth", 0.2}, medium = 0.04}

p_q_rich_gold= pile{{"yellow ochre", 5.5}, {"chrome yellow", 4.5}, {"lead white", 2.2}, {"green earth", 0.5}, medium = 0.06}
p_q_lit      = pile{{"chrome yellow", 5.5}, {"yellow ochre", 2.5}, {"lead white", 5.5}, medium = 0.05}
p_q_core     = pile{{"raw umber", 5.0}, {"yellow ochre", 3.5}, {"red earth", 2.5}, {"bone black", 0.8}, medium = 0.06}
p_q_deep     = pile{{"raw umber", 6.0}, {"bone black", 3.0}, {"red earth", 1.8}, medium = 0.05}
p_q_reflect  = pile{{"yellow ochre", 5.5}, {"lead white", 3.0}, {"raw umber", 1.5}, {"red earth", 0.8}, medium = 0.08}
p_q_hi       = pile{{"lead white", 9.0}, {"chrome yellow", 2.5}, {"yellow ochre", 0.6}, medium = 0.04}

local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_f8  = brush("filbert", 8)
local b_r5  = brush{kind = "round", width = 5.5, point = 0.9}
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. PITCHER: OPEN THE HANDLE LOOP & BLEND BELLY STRIPES
-- A. Clean negative space inside handle loop with dark background wall
b_f10:load(p_wall, 1.0)
b_f10:stroke({{246, 400}, {236, 430}, {242, 465}, {260, 500}}, {pressure = 1.0})
b_f8:load(p_wall, 1.0)
b_f8:stroke({{250, 420}, {250, 455}, {262, 485}}, {pressure = 0.98})

-- B. Refine the pulled-clay loop handle
b_r5:load(p_jug_rich, 1.0)
b_r5:stroke({{342, 365}, {295, 375}, {245, 395}, {222, 430}, {230, 475}, {255, 515}, {284, 542}}, {
  pressure = 1.0, swell = {0.9, 1.35, 1.0}
})

b_r3:load(p_jug_core, 1.0)
b_r3:stroke({{338, 375}, {295, 385}, {253, 408}, {232, 438}, {240, 480}, {264, 518}, {285, 542}}, {
  pressure = 0.95
})

b_r3:load(p_jug_lit, 1.0)
b_r3:stroke({{340, 363}, {293, 372}, {244, 392}, {220, 428}, {226, 465}}, {pressure = {0.8, 1.0, 0.8}})

b_r1:load(p_jug_hi, 1.0)
b_r1:stroke({{238, 398}, {222, 426}, {224, 442}}, {pressure = {0.5, 0.95, 0.5}})

-- C. Blend belly stripes into smooth, continuous ceramic roundness
-- Midtone body wash across the belly
b_f18:load(p_jug_rich, 1.0)
b_f18:stroke({{345, 375}, {350, 480}, {360, 615}}, {pressure = 0.95})
b_f14:load(p_jug_rich, 1.0)
b_f14:stroke({{370, 375}, {385, 480}, {380, 615}}, {pressure = 0.95})

-- Soft, wide lit flank
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{335, 370}, {315, 440}, {302, 500}, {318, 565}, {338, 618}}, {
  pressure = {0.85, 0.95, 0.95, 0.9, 0.8}, swell = {0.9, 1.25, 0.9}
})

-- Broad core shadow down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{395, 370}, {425, 435}, {450, 485}, {442, 545}, {424, 595}, {406, 622}}, {
  pressure = {0.85, 0.98, 1.0, 0.95, 0.9, 0.8}, swell = {0.9, 1.25, 0.9}
})

-- Deep shadow on far right contour
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{400, 360}, {422, 400}, {450, 440}, {468, 485}, {456, 540}, {436, 595}, {412, 624}}, {
  pressure = 0.9
})

-- Ambient table bounce along right contour
b_f8:load(p_jug_reflect, 0.9)
b_f8:stroke({{422, 405}, {452, 445}, {468, 485}, {458, 540}, {438, 590}, {414, 622}}, {pressure = 0.45})

-- Shoulder glaze catchlight
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{324, 402}, {320, 418}, {316, 436}}, {pressure = {0.7, 1.0, 0.7}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_jug_hi, 1.0)
b_r1:touch(320, 418, {pressure = 1.0})

-- Footring and contact shadow
b_r3:load(p_jug_rich, 1.0)
b_r3:stroke({{326, 624}, {370, 626}, {414, 624}}, {pressure = 0.95})
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{322, 626}, {370, 628}, {418, 626}}, {pressure = {0.95, 1.0, 0.95}})

-- 2. QUINCE: TRANSFORM STRIPES INTO VOLUPTUOUS SPHERICAL MASS
-- A. Broad golden body coat across the entire center to bury the left stripe
b_f18:load(p_q_rich_gold, 1.0)
b_f18:stroke({{615, 490}, {605, 550}, {595, 610}, {615, 672}}, {pressure = 1.0})
b_f14:load(p_q_rich_gold, 1.0)
b_f14:stroke({{635, 495}, {635, 560}, {630, 620}, {640, 675}}, {pressure = 1.0})

-- B. Luminous sunlit flank on the left (wide, radiant volume)
b_f18:load(p_q_lit, 1.0)
b_f18:stroke({{605, 490}, {575, 535}, {552, 585}, {550, 628}, {568, 662}, {598, 676}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- C. BROAD SHADOW HEMISPHERE (Fills the entire right half, not a single stripe!)
for x_s = 650, 715, 15 do
  b_f14:load(p_q_core, 1.0)
  b_f14:stroke({{x_s, 505}, {x_s + 15, 560}, {x_s + 20, 615}, {x_s - 5, 672}}, {pressure = 0.95})
end

-- Deep shadow on far right contour
b_f10:load(p_q_deep, 0.95)
b_f10:stroke({{675, 525}, {705, 570}, {730, 615}, {728, 650}, {705, 672}}, {pressure = 0.9})

-- Reflected limestone bounce on right flank
b_f10:load(p_q_reflect, 0.9)
b_f10:stroke({{685, 538}, {715, 585}, {734, 620}, {730, 652}, {710, 672}}, {pressure = 0.45})

-- Buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_q_hi, 1.0)
b_r3:stroke({{570, 565}, {562, 595}, {566, 625}}, {pressure = {0.9, 1.0, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_q_hi, 1.0)
b_r1:touch(562, 595, {pressure = 1.0})

-- Woody stem and socket
b_r2:load(p_q_deep, 1.0)
b_r2:touch(628, 486, {pressure = 0.95})
local p_stem = pile{{"raw umber", 6.0}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.05}
b_r2:load(p_stem, 1.0)
b_r2:stroke({{628, 486}, {634, 472}, {642, 458}, {652, 448}}, {pressure = {0.98, 0.9, 0.5}})
b_r1:load(p_q_lit, 0.85)
b_r1:stroke({{629, 484}, {635, 472}, {643, 459}}, {pressure = 0.6})

-- Calyx and contact shadow
b_r2:load(p_q_deep, 1.0)
b_r2:touch(655, 678, {pressure = 0.95})
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{575, 676}, {630, 680}, {680, 678}, {715, 670}}, {pressure = {0.85, 1.0, 0.9, 0.7}})

-- Transparent cast shadow
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{700, 678}, {750, 684}, {810, 692}}, {pressure = {0.8, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 3. CLEAN STRAY SHADOW STREAKS ON LIMESTONE TABLE
-- Horizontal sweeps over the stray marks beside walnuts and behind knife
for y = 638, 672, 8 do
  b_f14:load(p_stone_mid, 1.0)
  b_f14:stroke({{440, y}, {500, y}, {560, y}}, {pressure = 0.95})
end

for y = 675, 705, 8 do
  b_f14:load(p_stone_lit, 1.0)
  b_f14:stroke({{720, y}, {780, y}, {840, y}}, {pressure = 0.95})
end

print("Pitcher loop opened, belly blended, quince volume unified, table cleaned.")

--@ chunk 169
-- CHUNK 169: PERFECTING CÁNTARO AND QUINCE WITH PRECISE GEOMETRY

-- 1. DEFINE PALETTE PILES
p_wall       = pile{{"raw umber", 6.5}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.08}
p_stone_back = pile{{"lead white", 6.5}, {"yellow ochre", 2.8}, {"raw umber", 1.5}, {"red earth", 0.3}, medium = 0.08}
p_stone_mid  = pile{{"lead white", 7.5}, {"yellow ochre", 2.4}, {"raw umber", 1.0}, {"red earth", 0.2}, medium = 0.08}
p_stone_lit  = pile{{"lead white", 8.2}, {"yellow ochre", 2.0}, {"raw umber", 0.5}, medium = 0.08}
p_ledge      = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.2}, medium = 0.05}
p_shadow_stone = pile{{"raw umber", 4.8}, {"yellow ochre", 3.2}, {"lead white", 2.2}, {"red earth", 0.6}, medium = 0.10}

-- Cantaro piles
p_jug_rich   = pile{{"red earth", 6.2}, {"yellow ochre", 3.8}, {"lead white", 1.2}, {"raw umber", 1.0}, medium = 0.06}
p_jug_lit    = pile{{"yellow ochre", 6.0}, {"red earth", 2.5}, {"lead white", 4.5}, {"raw umber", 0.5}, medium = 0.06}
p_jug_core   = pile{{"red earth", 5.5}, {"raw umber", 4.5}, {"bone black", 1.2}, {"yellow ochre", 1.0}, medium = 0.06}
p_jug_deep   = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.8}, medium = 0.05}
p_jug_reflect= pile{{"yellow ochre", 5.0}, {"red earth", 2.0}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.08}
p_jug_hi     = pile{{"lead white", 9.0}, {"yellow ochre", 1.5}, {"red earth", 0.2}, medium = 0.04}

-- Quince piles
p_q_rich_gold= pile{{"yellow ochre", 5.5}, {"chrome yellow", 4.5}, {"lead white", 2.2}, {"green earth", 0.5}, medium = 0.06}
p_q_lit      = pile{{"chrome yellow", 5.5}, {"yellow ochre", 2.5}, {"lead white", 5.5}, medium = 0.05}
p_q_core     = pile{{"raw umber", 5.0}, {"yellow ochre", 3.5}, {"red earth", 2.5}, {"bone black", 0.8}, medium = 0.06}
p_q_deep     = pile{{"raw umber", 6.0}, {"bone black", 3.0}, {"red earth", 1.8}, medium = 0.05}
p_q_reflect  = pile{{"yellow ochre", 5.5}, {"lead white", 3.0}, {"raw umber", 1.5}, {"red earth", 0.8}, medium = 0.08}
p_q_hi       = pile{{"lead white", 9.0}, {"chrome yellow", 2.5}, {"yellow ochre", 0.6}, medium = 0.04}
p_stem       = pile{{"raw umber", 6.0}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.05}

local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_f8  = brush("filbert", 8)
local b_r5  = brush{kind = "round", width = 5.5, point = 0.9}
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 2. STAGE CLEAN-UPS
-- A. Cut away the square terracotta block inside the pitcher's handle loop
local pts_handle_hole = {
  {232, 400}, {255, 405}, {266, 435}, {262, 475}, {248, 505},
  {234, 465}, {224, 430}
}
local m_hole = poly(pts_handle_hole, true)
work(m_hole, {
  hand = "detail",
  pile = p_wall,
  coverage = 3.5,
  fill = true,
  clip = true
})

-- B. Cut away the ugly brown block on the upper right of the quince
-- Wall area above y = 520 (x = 665 to 735, y = 490 to 522)
local pts_wall_notch = {{665, 490}, {735, 490}, {735, 522}, {665, 522}}
local m_wall_notch = poly(pts_wall_notch, true)
work(m_wall_notch, {
  hand = "detail",
  pile = p_wall,
  coverage = 3.5,
  fill = true,
  clip = true
})

-- Tabletop area below y = 520 (x = 705 to 745, y = 520 to 565)
for y = 522, 565, 8 do
  b_f10:load(p_stone_back, 1.0)
  b_f10:stroke({{705, y}, {730, y}, {750, y}}, {pressure = 0.98})
end

-- C. Erase floating brown shadow smears on tabletop (behind knife: x = 690 to 860, y = 665 to 705)
for y = 668, 705, 8 do
  b_f18:load(p_stone_lit, 1.0)
  b_f18:stroke({{690, y}, {775, y}, {860, y}}, {pressure = 1.0})
end

-- D. Erase dangling string shadows on front ledge face
for y = 718, 765, 10 do
  b_f14:load(p_ledge, 1.0)
  b_f14:stroke({{770, y}, {810, y}, {840, y}}, {pressure = 1.0})
end

-- 3. PERFECTING THE EARTHENWARE CÁNTARO
-- A. Strap handle redraw: sturdy, turned pulled-clay loop
local pts_h_outer = {{342, 365}, {295, 375}, {242, 395}, {216, 430}, {226, 475}, {252, 515}, {278, 540}}
b_r5:load(p_jug_rich, 1.0)
b_r5:stroke(pts_h_outer, {pressure = 1.0, swell = {0.9, 1.35, 1.0}})

local pts_h_under = {{338, 375}, {295, 385}, {248, 408}, {224, 438}, {234, 480}, {258, 518}, {280, 540}}
b_r3:load(p_jug_core, 1.0)
b_r3:stroke(pts_h_under, {pressure = 0.95})

b_r3:load(p_jug_lit, 1.0)
b_r3:stroke({{340, 363}, {293, 372}, {238, 392}, {214, 428}, {222, 465}}, {pressure = {0.8, 1.0, 0.8}})

b_r1:load(p_jug_hi, 1.0)
b_r1:stroke({{232, 398}, {216, 426}, {220, 442}}, {pressure = {0.5, 0.95, 0.5}})

-- B. Blend belly stripes into smooth, continuous ceramic roundness
-- Curved diagonal body sweeps across the form
b_f18:load(p_jug_rich, 1.0)
b_f18:stroke({{325, 410}, {355, 475}, {385, 545}, {365, 615}}, {pressure = 0.98})
b_f14:load(p_jug_rich, 1.0)
b_f14:stroke({{345, 380}, {370, 460}, {395, 540}, {380, 615}}, {pressure = 0.98})

-- Lit flank facing north light
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{335, 370}, {315, 435}, {302, 495}, {315, 560}, {335, 618}}, {
  pressure = {0.85, 0.95, 0.95, 0.9, 0.8}, swell = {0.9, 1.25, 0.9}
})

-- Core shadow down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{395, 370}, {425, 435}, {450, 485}, {442, 545}, {424, 595}, {406, 622}}, {
  pressure = {0.85, 0.98, 1.0, 0.95, 0.9, 0.8}, swell = {0.9, 1.25, 0.9}
})

-- Deep shadow on far right contour
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{400, 360}, {422, 400}, {450, 440}, {468, 485}, {456, 540}, {436, 595}, {412, 624}}, {
  pressure = 0.9
})

-- Ambient table bounce along right contour
b_f8:load(p_jug_reflect, 0.9)
b_f8:stroke({{422, 405}, {452, 445}, {468, 485}, {458, 540}, {438, 590}, {414, 622}}, {pressure = 0.45})

-- Decisive glaze impasto on shoulder curve
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{324, 402}, {320, 418}, {316, 436}}, {pressure = {0.7, 1.0, 0.7}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_jug_hi, 1.0)
b_r1:touch(320, 418, {pressure = 1.0})

-- Foot ring and contact shadow
b_r3:load(p_jug_rich, 1.0)
b_r3:stroke({{326, 624}, {370, 626}, {414, 624}}, {pressure = 0.95})
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{322, 626}, {370, 628}, {418, 626}}, {pressure = {0.95, 1.0, 0.95}})

-- Pitcher cast shadow flowing rightward
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{414, 625}, {465, 628}, {535, 636}}, {pressure = {0.85, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 4. PERFECTING THE RIPE QUINCE
-- A. Unify the golden body and erase vertical stripes
b_f18:load(p_q_rich_gold, 1.0)
b_f18:stroke({{610, 490}, {600, 550}, {590, 610}, {610, 672}}, {pressure = 1.0})

-- B. Luminous sunlit flank on the left (wide, radiant sphere)
b_f18:load(p_q_lit, 1.0)
b_f18:stroke({{600, 490}, {570, 535}, {548, 585}, {546, 628}, {565, 662}, {595, 676}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- C. Deep warm amber-umber core shadow down right flank (curving naturally to base)
b_f18:load(p_q_core, 1.0)
b_f18:stroke({{640, 495}, {665, 545}, {688, 595}, {692, 638}, {675, 668}, {645, 675}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.9, 1.35, 0.9}
})

-- Deep shadow on far right contour
b_f10:load(p_q_deep, 0.95)
b_f10:stroke({{660, 520}, {685, 565}, {708, 610}, {706, 645}, {688, 670}}, {pressure = 0.9})

-- Reflected limestone bounce along right shaded contour
b_f10:load(p_q_reflect, 0.9)
b_f10:stroke({{670, 535}, {695, 580}, {714, 618}, {710, 650}, {692, 670}}, {pressure = 0.45})

-- Buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_q_hi, 1.0)
b_r3:stroke({{568, 565}, {560, 595}, {564, 625}}, {pressure = {0.9, 1.0, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_q_hi, 1.0)
b_r1:touch(560, 595, {pressure = 1.0})

-- Arching gnarled woody stem
b_r2:load(p_q_deep, 1.0)
b_r2:touch(628, 486, {pressure = 0.95})
b_r2:load(p_stem, 1.0)
b_r2:stroke({{628, 486}, {634, 472}, {642, 458}, {652, 448}}, {pressure = {0.98, 0.9, 0.5}})
b_r1:load(p_q_lit, 0.85)
b_r1:stroke({{629, 484}, {635, 472}, {643, 459}}, {pressure = 0.6})

-- Calyx eye and contact shadow
b_r2:load(p_q_deep, 1.0)
b_r2:touch(645, 678, {pressure = 0.95})
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{565, 676}, {620, 680}, {670, 678}, {700, 670}}, {pressure = {0.85, 1.0, 0.9, 0.7}})

-- Transparent cast shadow of quince
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{685, 678}, {735, 684}, {795, 692}}, {pressure = {0.8, 0.2}, swell = {1.0, 1.3, 0.6}})

print("Handle loop opened, brown notch erased, cántaro and quince perfected.")

--@ chunk 170
-- CHUNK 170: DEFINITIVE MASTER CONSOLIDATION OF THE STILL LIFE

-- 1. DRY ALL UNDERLAYERS THOROUGHLY
local t = wait(14 * 24 * 60)
print("Underlayers dried to stable touch-dry ground. Time:", t)

-- 2. PALETTE PILES
-- Stage
p_wall         = pile{{"raw umber", 6.5}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.08}
p_stone_back   = pile{{"lead white", 6.5}, {"yellow ochre", 2.8}, {"raw umber", 1.5}, {"red earth", 0.3}, medium = 0.08}
p_stone_mid    = pile{{"lead white", 7.5}, {"yellow ochre", 2.4}, {"raw umber", 1.0}, {"red earth", 0.2}, medium = 0.08}
p_stone_lit    = pile{{"lead white", 8.2}, {"yellow ochre", 2.0}, {"raw umber", 0.5}, medium = 0.08}
p_ledge        = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.2}, medium = 0.05}
p_chamfer      = pile{{"lead white", 8.8}, {"yellow ochre", 1.8}, medium = 0.05}
p_shadow_stone = pile{{"raw umber", 4.8}, {"yellow ochre", 3.2}, {"lead white", 2.2}, {"red earth", 0.6}, medium = 0.10}

-- Cantaro (terracotta)
p_jug_rich     = pile{{"red earth", 6.2}, {"yellow ochre", 3.8}, {"lead white", 1.2}, {"raw umber", 1.0}, medium = 0.06}
p_jug_lit      = pile{{"yellow ochre", 6.0}, {"red earth", 2.5}, {"lead white", 4.5}, {"raw umber", 0.5}, medium = 0.06}
p_jug_core     = pile{{"red earth", 5.5}, {"raw umber", 4.5}, {"bone black", 1.2}, {"yellow ochre", 1.0}, medium = 0.06}
p_jug_deep     = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.8}, medium = 0.05}
p_jug_reflect  = pile{{"yellow ochre", 5.0}, {"red earth", 2.0}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.08}
p_jug_hi       = pile{{"lead white", 9.0}, {"yellow ochre", 1.5}, {"red earth", 0.2}, medium = 0.04}

-- Quince (golden membrillo)
p_q_rich_gold  = pile{{"yellow ochre", 5.5}, {"chrome yellow", 4.5}, {"lead white", 2.2}, {"green earth", 0.5}, medium = 0.06}
p_q_lit        = pile{{"chrome yellow", 5.5}, {"yellow ochre", 2.5}, {"lead white", 5.5}, medium = 0.05}
p_q_half       = pile{{"yellow ochre", 5.0}, {"green earth", 2.0}, {"raw umber", 1.5}, {"lead white", 2.2}, medium = 0.08}
p_q_core       = pile{{"raw umber", 5.0}, {"yellow ochre", 3.5}, {"red earth", 2.5}, {"bone black", 0.8}, medium = 0.06}
p_q_deep       = pile{{"raw umber", 6.0}, {"bone black", 3.0}, {"red earth", 1.8}, medium = 0.05}
p_q_reflect    = pile{{"yellow ochre", 5.5}, {"lead white", 3.0}, {"raw umber", 1.5}, {"red earth", 0.8}, medium = 0.08}
p_q_hi         = pile{{"lead white", 9.0}, {"chrome yellow", 2.5}, {"yellow ochre", 0.6}, medium = 0.04}
p_stem         = pile{{"raw umber", 6.0}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.05}

-- Knife (steel, brass, wood)
p_steel_lit    = pile{{"lead white", 8.2}, {"bone black", 1.0}, {"smalt", 0.6}, {"raw umber", 0.4}, medium = 0.05}
p_steel_mid    = pile{{"lead white", 4.8}, {"bone black", 3.0}, {"smalt", 1.0}, {"raw umber", 1.2}, medium = 0.06}
p_steel_dark   = pile{{"bone black", 6.0}, {"raw umber", 3.0}, {"lead white", 1.0}, medium = 0.05}
p_brass        = pile{{"yellow ochre", 6.0}, {"red earth", 2.0}, {"raw umber", 2.0}, {"lead white", 1.5}, medium = 0.06}
p_brass_glint  = pile{{"lead white", 8.0}, {"yellow ochre", 3.0}, {"chrome yellow", 1.0}, medium = 0.04}
p_knife_wood   = pile{{"raw umber", 5.5}, {"red earth", 3.0}, {"bone black", 2.5}, {"yellow ochre", 1.0}, medium = 0.06}
p_knife_wood_lit= pile{{"red earth", 4.5}, {"yellow ochre", 3.5}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.06}

-- Walnuts
p_nut_shell    = pile{{"raw umber", 4.5}, {"yellow ochre", 4.0}, {"red earth", 2.5}, {"lead white", 1.5}, medium = 0.06}
p_nut_deep     = pile{{"raw umber", 6.0}, {"bone black", 3.0}, {"red earth", 1.5}, medium = 0.05}
p_nut_meat     = pile{{"lead white", 7.8}, {"yellow ochre", 2.8}, {"raw umber", 0.6}, medium = 0.05}
p_nut_meat_shade= pile{{"yellow ochre", 4.5}, {"raw umber", 3.0}, {"lead white", 3.0}, {"red earth", 0.8}, medium = 0.06}

local b_f20 = brush("filbert", 20)
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_f8  = brush("filbert", 8)
local b_r5  = brush{kind = "round", width = 5.5, point = 0.9}
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 3. CLEANING STAGE & REMOVING ARTIFACTS
-- A. Clean negative space inside cántaro handle loop (reveals true handle arch)
-- The space between handle (x=215-235) and belly (x=270-330) from y=380 to y=510:
b_f14:load(p_wall, 1.0)
b_f14:stroke({{310, 385}, {265, 410}, {242, 445}, {255, 485}, {270, 510}}, {pressure = 1.0})
b_f10:load(p_wall, 1.0)
b_f10:stroke({{285, 415}, {255, 445}, {262, 485}}, {pressure = 1.0})

-- B. Clean tabletop behind knife (x = 690 to 860, y = 665 to 705)
for y = 668, 706, 8 do
  b_f18:load(p_stone_lit, 1.0)
  b_f18:stroke({{690, y}, {775, y}, {860, y}}, {pressure = 1.0})
end

-- C. Clean front ledge face below knife
for y = 718, 765, 10 do
  b_f14:load(p_ledge, 1.0)
  b_f14:stroke({{770, y}, {810, y}, {845, y}}, {pressure = 1.0})
end

-- 4. EARTHENWARE CÁNTARO: NOBLE SCULPTURAL FORM
-- A. Redraw graceful pulled strap handle
local pts_h_outer = {{342, 365}, {295, 375}, {240, 395}, {214, 430}, {224, 475}, {252, 515}, {278, 540}}
b_r5:load(p_jug_rich, 1.0)
b_r5:stroke(pts_h_outer, {pressure = 1.0, swell = {0.9, 1.35, 1.0}})

local pts_h_under = {{338, 375}, {295, 385}, {246, 408}, {222, 438}, {232, 480}, {258, 518}, {280, 540}}
b_r3:load(p_jug_core, 1.0)
b_r3:stroke(pts_h_under, {pressure = 0.95})

b_r3:load(p_jug_lit, 1.0)
b_r3:stroke({{340, 363}, {293, 372}, {236, 392}, {212, 428}, {220, 465}}, {pressure = {0.8, 1.0, 0.8}})

b_r1:load(p_jug_hi, 1.0)
b_r1:stroke({{230, 398}, {214, 426}, {218, 442}}, {pressure = {0.5, 0.95, 0.5}})

-- B. Solid terracotta belly (completely buries diagonal comma and vertical stripes)
b_f18:load(p_jug_rich, 1.0)
b_f18:stroke({{355, 365}, {365, 485}, {370, 622}}, {pressure = 1.0})
b_f14:load(p_jug_rich, 1.0)
b_f14:stroke({{340, 370}, {340, 480}, {350, 620}}, {pressure = 0.98})
b_f14:load(p_jug_rich, 1.0)
b_f14:stroke({{375, 370}, {390, 480}, {385, 620}}, {pressure = 0.98})

-- Lit flank on left (raking north light)
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{340, 365}, {318, 420}, {300, 480}, {310, 545}, {330, 600}, {345, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Core shadow down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{395, 365}, {425, 425}, {455, 485}, {445, 545}, {426, 595}, {408, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Deep shadow on right contour
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{400, 360}, {422, 400}, {450, 440}, {468, 485}, {456, 540}, {436, 595}, {412, 624}}, {
  pressure = 0.9
})

-- Ambient table bounce along right contour
b_f8:load(p_jug_reflect, 0.9)
b_f8:stroke({{422, 405}, {452, 445}, {468, 485}, {458, 540}, {438, 590}, {414, 622}}, {pressure = 0.45})

-- Decisive glaze impasto on shoulder curve
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{324, 402}, {320, 418}, {316, 436}}, {pressure = {0.7, 1.0, 0.7}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_jug_hi, 1.0)
b_r1:touch(320, 418, {pressure = 1.0})

-- Foot ring and contact shadow
b_r3:load(p_jug_rich, 1.0)
b_r3:stroke({{326, 624}, {370, 626}, {414, 624}}, {pressure = 0.95})
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{322, 626}, {370, 628}, {418, 626}}, {pressure = {0.95, 1.0, 0.95}})

-- Pitcher cast shadow flowing rightward
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{414, 625}, {465, 628}, {535, 636}}, {pressure = {0.85, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 5. RIPE GOLDEN QUINCE (MEMBRILLO): FULL VOLUPTUOUS SPHERE
-- Natural, continuous knobby silhouette
local pts_quince = {
  {630, 485}, {655, 495}, {680, 520}, {705, 565}, {724, 610},
  {725, 640}, {705, 668}, {675, 676}, {630, 678}, {585, 675},
  {555, 655}, {538, 625}, {535, 585}, {550, 540}, {575, 510},
  {605, 488}, {622, 484}
}
local m_q = poly(pts_quince, true)

-- A. Dense golden body coat (buries the grey box and white stripes completely!)
work(m_q, {
  hand = "body",
  pile = p_q_rich_gold,
  angle = 1.35,
  coverage = 3.5,
  fill = true,
  edge = "firm"
})

-- B. Luminous sunlit left lobe (chrome yellow + yellow ochre + lead white)
b_f18:load(p_q_lit, 1.0)
b_f18:stroke({{605, 490}, {575, 535}, {550, 585}, {548, 628}, {566, 662}, {598, 676}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Half-tone in central hollow
b_f10:load(p_q_half, 0.95)
b_f10:stroke({{625, 505}, {605, 560}, {590, 615}, {615, 668}}, {pressure = 0.92})

-- C. Broad warm amber-umber core shadow down right flank
b_f18:load(p_q_core, 1.0)
b_f18:stroke({{642, 495}, {670, 545}, {695, 595}, {700, 638}, {680, 670}, {650, 676}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.9, 1.35, 0.9}
})

-- Deep shadow on far right contour
b_f10:load(p_q_deep, 0.95)
b_f10:stroke({{665, 520}, {692, 565}, {715, 610}, {712, 645}, {695, 670}}, {pressure = 0.9})

-- Reflected limestone bounce along right shaded contour
b_f10:load(p_q_reflect, 0.9)
b_f10:stroke({{675, 535}, {702, 580}, {722, 618}, {718, 650}, {700, 670}}, {pressure = 0.45})

-- Buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_q_hi, 1.0)
b_r3:stroke({{570, 565}, {562, 595}, {566, 625}}, {pressure = {0.9, 1.0, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_q_hi, 1.0)
b_r1:touch(562, 595, {pressure = 1.0})

-- Arching gnarled woody stem
b_r2:load(p_q_deep, 1.0)
b_r2:touch(628, 486, {pressure = 0.95})
b_r2:load(p_stem, 1.0)
b_r2:stroke({{628, 486}, {634, 472}, {642, 458}, {652, 448}}, {pressure = {0.98, 0.9, 0.5}})
b_r1:load(p_q_lit, 0.85)
b_r1:stroke({{629, 484}, {635, 472}, {643, 459}}, {pressure = 0.6})

-- Calyx eye and contact shadow
b_r2:load(p_q_deep, 1.0)
b_r2:touch(650, 678, {pressure = 0.95})
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{570, 676}, {625, 680}, {675, 678}, {710, 670}}, {pressure = {0.85, 1.0, 0.9, 0.7}})

-- Transparent cast shadow of quince
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{690, 678}, {740, 684}, {805, 692}}, {pressure = {0.8, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 6. THE SUBSTANTIAL BODEGÓN KNIFE (FORGED STEEL, BRASS BOLSTER, TURNED WALNUT HANDLE)
-- Blade flat (substantial forged blade from {645, 682} to {765, 714})
b_f14:load(p_steel_mid, 1.0)
b_f14:stroke({{645, 682}, {705, 698}, {765, 714}}, {pressure = 1.0})

-- Steel spine (dark forged back)
b_r2:load(p_steel_dark, 1.0)
b_r2:stroke({{645, 682}, {705, 697}, {763, 711}}, {pressure = 0.95})

-- Reflected north light across blade bevel
b_r3:load(p_steel_lit, 1.0)
b_r3:stroke({{650, 683}, {707, 699}, {764, 713}}, {pressure = 0.95})

-- Razor cutting edge
b_r1:load(p_steel_lit, 1.0)
b_r1:stroke({{645, 682}, {705, 700}, {765, 715}}, {pressure = 1.0})

-- Blade contact shadow on limestone
b_r2:load(p_shadow_stone, 0.85)
b_r2:stroke({{647, 684}, {707, 702}, {764, 717}}, {pressure = {0.5, 0.9}})

-- Heavy brass bolster collar ({763, 711} to {778, 716})
b_r5:load(p_brass, 1.0)
b_r5:stroke({{763, 711}, {771, 713}, {778, 716}}, {pressure = 1.0})
b_r2:load(p_brass_glint, 1.0)
b_r2:touch(771, 713, {pressure = 1.0})
b_r1:load(p_steel_dark, 0.9)
b_r1:stroke({{764, 715}, {778, 718}}, {pressure = 0.85})

-- Substantial turned dark walnut handle projecting over front ledge ({778, 716} to {855, 755})
b_r5:load(p_knife_wood, 1.0)
b_r5:stroke({{778, 716}, {815, 735}, {855, 755}}, {pressure = 1.0, swell = {1.0, 1.45, 1.0}})

-- Lit upper crest of turned handle
b_r3:load(p_knife_wood_lit, 1.0)
b_r3:stroke({{779, 714}, {815, 733}, {854, 753}}, {pressure = 0.92})

-- Turned brass pommel cap at end of handle
b_r2:load(p_brass_glint, 1.0)
b_r2:touch(855, 755, {pressure = 1.0})

-- Handle shadow falling down the front vertical ledge face
b_r5:load(p_jug_deep, 0.85)
b_r5:stroke({{782, 716}, {786, 738}, {790, 762}}, {pressure = {0.9, 0.25}})

-- Specular chamfer highlight around knife
b_r1:load(p_chamfer, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {763, 715}}, {pressure = {0.85, 0.85}, shake = 0.02})
b_r1:stroke({{778, 715}, {900, 715}, {1005, 715}}, {pressure = {0.85, 0.85}, shake = 0.02})

-- 7. SUBSTANTIAL FOREGROUND WALNUTS (BETWEEN CÁNTARO AND QUINCE)
-- A. Whole ribbed walnut at {470, 644} (width ~26 units)
b_r5:load(p_nut_shell, 1.0)
b_r5:stroke({{458, 645}, {470, 641}, {482, 645}}, {pressure = 1.0, swell = {0.9, 1.4, 0.9}})
b_r5:load(p_nut_shell, 1.0)
b_r5:stroke({{460, 647}, {471, 645}, {482, 647}}, {pressure = 1.0})

-- Suture line and ribbed shell crevices
b_r2:load(p_nut_deep, 0.95)
b_r2:stroke({{458, 645}, {470, 640}, {482, 645}}, {pressure = 0.85})
b_r1:load(p_q_hi, 0.9)
b_r1:stroke({{465, 639}, {471, 638}, {477, 640}}, {pressure = 0.85})

-- Contact and cast shadow of whole walnut
b_r3:load(p_jug_deep, 0.95)
b_r3:stroke({{456, 649}, {470, 651}, {484, 649}}, {pressure = 0.9})
b_f8:load(p_shadow_stone, 0.55)
b_f8:stroke({{478, 648}, {502, 652}}, {pressure = {0.8, 0.2}})

-- B. Cracked walnut half with ivory kernel at {510, 652} (width ~28 units)
-- Woody shell cup
b_r5:load(p_nut_shell, 1.0)
b_r5:stroke({{498, 655}, {510, 658}, {524, 653}}, {pressure = 1.0, swell = {0.9, 1.4, 0.9}})

-- Dark shadow inside shell cup
b_r3:load(p_nut_deep, 1.0)
b_r3:stroke({{500, 652}, {510, 650}, {522, 650}}, {pressure = 0.95})

-- Convoluted ivory kernel lobes in thick impasto
b_r3:load(p_nut_meat_shade, 1.0)
b_r3:stroke({{502, 651}, {510, 650}, {520, 651}}, {pressure = 0.95})

b_r2:load(p_nut_meat, 1.0)
b_r2:stroke({{502, 650}, {506, 648}, {510, 650}}, {pressure = 1.0})
b_r2:load(p_nut_meat, 1.0)
b_r2:stroke({{511, 650}, {515, 648}, {519, 650}}, {pressure = 1.0})

b_r1:load(p_q_hi, 1.0)
b_r1:touch(505, 648, {pressure = 1.0})
b_r1:touch(514, 648, {pressure = 1.0})

-- Shell rim highlight
b_r1:load(p_q_lit, 0.9)
b_r1:stroke({{498, 654}, {510, 657}, {524, 652}}, {pressure = 0.85})

-- Contact and cast shadow of walnut half
b_r3:load(p_jug_deep, 0.95)
b_r3:stroke({{498, 658}, {512, 660}, {526, 655}}, {pressure = 0.9})
b_f8:load(p_shadow_stone, 0.55)
b_f8:stroke({{520, 656}, {545, 659}}, {pressure = {0.8, 0.2}})

print("Definitive master consolidation complete.")

--@ chunk 171
-- CHUNK 171: MASTER HARMONIZATION AND PICTORIAL CONSOLIDATION

-- Palette piles
p_wall       = pile{{"raw umber", 6.5}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.08}
p_stone_back = pile{{"lead white", 6.5}, {"yellow ochre", 2.8}, {"raw umber", 1.5}, {"red earth", 0.3}, medium = 0.08}
p_stone_mid  = pile{{"lead white", 7.5}, {"yellow ochre", 2.4}, {"raw umber", 1.0}, {"red earth", 0.2}, medium = 0.08}
p_stone_lit  = pile{{"lead white", 8.2}, {"yellow ochre", 2.0}, {"raw umber", 0.5}, medium = 0.08}
p_ledge      = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.2}, medium = 0.05}
p_chamfer    = pile{{"lead white", 8.8}, {"yellow ochre", 1.8}, medium = 0.05}
p_shadow_stone = pile{{"raw umber", 4.8}, {"yellow ochre", 3.2}, {"lead white", 2.2}, {"red earth", 0.6}, medium = 0.10}

-- Cantaro (terracotta)
p_jug_rich   = pile{{"red earth", 6.2}, {"yellow ochre", 3.8}, {"lead white", 1.2}, {"raw umber", 1.0}, medium = 0.06}
p_jug_lit    = pile{{"yellow ochre", 6.0}, {"red earth", 2.5}, {"lead white", 4.5}, {"raw umber", 0.5}, medium = 0.06}
p_jug_core   = pile{{"red earth", 5.5}, {"raw umber", 4.5}, {"bone black", 1.2}, {"yellow ochre", 1.0}, medium = 0.06}
p_jug_deep   = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.8}, medium = 0.05}
p_jug_reflect= pile{{"yellow ochre", 5.0}, {"red earth", 2.0}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.08}
p_jug_hi     = pile{{"lead white", 9.0}, {"yellow ochre", 1.5}, {"red earth", 0.2}, medium = 0.04}

-- Quince (golden membrillo)
p_q_rich_gold= pile{{"yellow ochre", 5.5}, {"chrome yellow", 4.5}, {"lead white", 2.2}, {"green earth", 0.5}, medium = 0.06}
p_q_lit      = pile{{"chrome yellow", 5.5}, {"yellow ochre", 2.5}, {"lead white", 5.5}, medium = 0.05}
p_q_half     = pile{{"yellow ochre", 5.0}, {"green earth", 2.0}, {"raw umber", 1.5}, {"lead white", 2.2}, medium = 0.08}
p_q_core     = pile{{"raw umber", 5.0}, {"yellow ochre", 3.5}, {"red earth", 2.5}, {"bone black", 0.8}, medium = 0.06}
p_q_deep     = pile{{"raw umber", 6.0}, {"bone black", 3.0}, {"red earth", 1.8}, medium = 0.05}
p_q_reflect  = pile{{"yellow ochre", 5.5}, {"lead white", 3.0}, {"raw umber", 1.5}, {"red earth", 0.8}, medium = 0.08}
p_q_hi       = pile{{"lead white", 9.0}, {"chrome yellow", 2.5}, {"yellow ochre", 0.6}, medium = 0.04}
p_stem       = pile{{"raw umber", 6.0}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.05}

-- Knife (steel, brass, wood)
p_steel_lit  = pile{{"lead white", 8.2}, {"bone black", 1.0}, {"smalt", 0.6}, {"raw umber", 0.4}, medium = 0.05}
p_steel_mid  = pile{{"lead white", 4.8}, {"bone black", 3.0}, {"smalt", 1.0}, {"raw umber", 1.2}, medium = 0.06}
p_steel_dark = pile{{"bone black", 6.0}, {"raw umber", 3.0}, {"lead white", 1.0}, medium = 0.05}
p_brass      = pile{{"yellow ochre", 6.0}, {"red earth", 2.0}, {"raw umber", 2.0}, {"lead white", 1.5}, medium = 0.06}
p_brass_glint= pile{{"lead white", 8.0}, {"yellow ochre", 3.0}, {"chrome yellow", 1.0}, medium = 0.04}
p_knife_wood = pile{{"raw umber", 5.5}, {"red earth", 3.0}, {"bone black", 2.5}, {"yellow ochre", 1.0}, medium = 0.06}
p_knife_wood_lit= pile{{"red earth", 4.5}, {"yellow ochre", 3.5}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.06}

-- Walnuts
p_nut_shell  = pile{{"raw umber", 4.5}, {"yellow ochre", 4.0}, {"red earth", 2.5}, {"lead white", 1.5}, medium = 0.06}
p_nut_deep   = pile{{"raw umber", 6.0}, {"bone black", 3.0}, {"red earth", 1.5}, medium = 0.05}
p_nut_meat   = pile{{"lead white", 7.8}, {"yellow ochre", 2.8}, {"raw umber", 0.6}, medium = 0.05}
p_nut_meat_shade= pile{{"yellow ochre", 4.5}, {"raw umber", 3.0}, {"lead white", 3.0}, {"red earth", 0.8}, medium = 0.06}

local b_f26 = brush("filbert", 26)
local b_f20 = brush("filbert", 20)
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_f8  = brush("filbert", 8)
local b_r5  = brush{kind = "round", width = 5.5, point = 0.9}
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}
local b_badger = brush("badger", 32)

-- 1. STAGE PURIFICATION
-- A. Clear negative space inside handle loop with dark background wall
b_f20:load(p_wall, 1.0)
b_f20:stroke({{290, 395}, {255, 430}, {255, 475}, {270, 505}}, {pressure = 1.0})
b_f14:load(p_wall, 1.0)
b_f14:stroke({{265, 420}, {242, 445}, {250, 480}}, {pressure = 1.0})

-- B. Erase grey box on upper right of quince with dark background wall (y = 480 to 522, x = 650 to 760)
for y = 485, 520, 8 do
  b_f18:load(p_wall, 1.0)
  b_f18:stroke({{655, y}, {710, y}, {765, y}}, {pressure = 1.0})
end

-- C. Unify entire limestone table from quince to right edge (y = 520 to 714, x = 680 to 1015)
-- Sweeps away the white pillar and the white knife patch into ONE luminous stone plane
for y = 522, 600, 10 do
  b_f26:load(p_stone_back, 1.0)
  b_f26:stroke({{690, y}, {850, y}, {1015, y}}, {pressure = 1.0})
end

for y = 605, 665, 10 do
  b_f26:load(p_stone_mid, 1.0)
  b_f26:stroke({{680, y}, {850, y}, {1015, y}}, {pressure = 1.0})
end

for y = 668, 714, 8 do
  b_f26:load(p_stone_lit, 1.0)
  b_f26:stroke({{660, y}, {840, y}, {1015, y}}, {pressure = 1.0})
end

-- Horizontal badger passes along table grain
b_badger:stroke({{680, 550}, {1015, 550}}, {pressure = 0.22})
b_badger:stroke({{680, 630}, {1015, 630}}, {pressure = 0.22})
b_badger:stroke({{660, 685}, {1015, 685}}, {pressure = 0.22})

-- D. Front vertical ledge below knife (solid dark masonry)
for y = 716, 765, 10 do
  b_f18:load(p_ledge, 1.0)
  b_f18:stroke({{760, y}, {850, y}, {1015, y}}, {pressure = 1.0})
end

-- 2. EARTHENWARE CÁNTARO (NOBLE TURNED FORM & LOOP HANDLE)
-- Redraw sturdy pulled strap handle
local pts_h_outer = {{342, 365}, {295, 375}, {240, 395}, {214, 430}, {224, 475}, {252, 515}, {278, 540}}
b_r5:load(p_jug_rich, 1.0)
b_r5:stroke(pts_h_outer, {pressure = 1.0, swell = {0.9, 1.35, 1.0}})

local pts_h_under = {{338, 375}, {295, 385}, {246, 408}, {222, 438}, {232, 480}, {258, 518}, {280, 540}}
b_r3:load(p_jug_core, 1.0)
b_r3:stroke(pts_h_under, {pressure = 0.95})

b_r3:load(p_jug_lit, 1.0)
b_r3:stroke({{340, 363}, {293, 372}, {236, 392}, {212, 428}, {220, 465}}, {pressure = {0.8, 1.0, 0.8}})

b_r1:load(p_jug_hi, 1.0)
b_r1:stroke({{230, 398}, {214, 426}, {218, 442}}, {pressure = {0.5, 0.95, 0.5}})

-- Solid terracotta belly sweeps
b_f18:load(p_jug_rich, 1.0)
b_f18:stroke({{355, 365}, {365, 485}, {370, 622}}, {pressure = 1.0})
b_f14:load(p_jug_rich, 1.0)
b_f14:stroke({{340, 370}, {340, 480}, {350, 620}}, {pressure = 0.98})
b_f14:load(p_jug_rich, 1.0)
b_f14:stroke({{375, 370}, {390, 480}, {385, 620}}, {pressure = 0.98})

-- Lit flank on left (raking north light)
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{340, 365}, {318, 420}, {300, 480}, {310, 545}, {330, 600}, {345, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Core shadow down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{395, 365}, {425, 425}, {455, 485}, {445, 545}, {426, 595}, {408, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Deep shadow on right contour
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{400, 360}, {422, 400}, {450, 440}, {468, 485}, {456, 540}, {436, 595}, {412, 624}}, {
  pressure = 0.9
})

-- Ambient table bounce along right contour
b_f8:load(p_jug_reflect, 0.9)
b_f8:stroke({{422, 405}, {452, 445}, {468, 485}, {458, 540}, {438, 590}, {414, 622}}, {pressure = 0.45})

-- Decisive glaze impasto on shoulder curve
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{324, 402}, {320, 418}, {316, 436}}, {pressure = {0.7, 1.0, 0.7}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_jug_hi, 1.0)
b_r1:touch(320, 418, {pressure = 1.0})

-- Foot ring and contact shadow
b_r3:load(p_jug_rich, 1.0)
b_r3:stroke({{326, 624}, {370, 626}, {414, 624}}, {pressure = 0.95})
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{322, 626}, {370, 628}, {418, 626}}, {pressure = {0.95, 1.0, 0.95}})

-- Pitcher cast shadow flowing rightward
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{414, 625}, {465, 628}, {535, 636}}, {pressure = {0.85, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 3. RIPE GOLDEN QUINCE (VOLUPTUOUS ORGANIC MASS)
-- A. Sculpting the full volume
b_f18:load(p_q_rich_gold, 1.0)
b_f18:stroke({{630, 485}, {620, 550}, {610, 610}, {625, 674}}, {pressure = 1.0})

-- Sunlit left lobe (chrome yellow + yellow ochre)
b_f18:load(p_q_lit, 1.0)
b_f18:stroke({{605, 490}, {575, 535}, {550, 585}, {548, 628}, {566, 662}, {598, 676}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Central half-tone
b_f10:load(p_q_half, 0.95)
b_f10:stroke({{625, 505}, {605, 560}, {590, 615}, {615, 668}}, {pressure = 0.92})

-- Broad warm amber core shadow down right flank
b_f18:load(p_q_core, 1.0)
b_f18:stroke({{638, 495}, {665, 545}, {692, 595}, {696, 638}, {678, 670}, {645, 676}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.9, 1.35, 0.9}
})

-- Deep shadow on far right contour
b_f10:load(p_q_deep, 0.95)
b_f10:stroke({{660, 520}, {688, 565}, {710, 610}, {708, 645}, {690, 670}}, {pressure = 0.9})

-- Reflected limestone bounce along shaded contour
b_f10:load(p_q_reflect, 0.9)
b_f10:stroke({{670, 535}, {698, 580}, {718, 618}, {714, 650}, {695, 670}}, {pressure = 0.45})

-- Buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_q_hi, 1.0)
b_r3:stroke({{570, 565}, {562, 595}, {566, 625}}, {pressure = {0.9, 1.0, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_q_hi, 1.0)
b_r1:touch(562, 595, {pressure = 1.0})

-- Arching gnarled woody stem
b_r2:load(p_q_deep, 1.0)
b_r2:touch(628, 486, {pressure = 0.95})
b_r2:load(p_stem, 1.0)
b_r2:stroke({{628, 486}, {634, 472}, {642, 458}, {652, 448}}, {pressure = {0.98, 0.9, 0.5}})
b_r1:load(p_q_lit, 0.85)
b_r1:stroke({{629, 484}, {635, 472}, {643, 459}}, {pressure = 0.6})

-- Calyx eye and contact shadow
b_r2:load(p_q_deep, 1.0)
b_r2:touch(650, 678, {pressure = 0.95})
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{570, 676}, {625, 680}, {675, 678}, {710, 670}}, {pressure = {0.85, 1.0, 0.9, 0.7}})

-- Transparent cast shadow of quince
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{690, 678}, {740, 684}, {805, 692}}, {pressure = {0.8, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 4. THE SUBSTANTIAL BODEGÓN KNIFE
-- Forged steel blade (width 12 units, from tip {645, 682} to bolster {765, 714})
b_f14:load(p_steel_mid, 1.0)
b_f14:stroke({{645, 682}, {705, 698}, {765, 714}}, {pressure = 1.0})

-- Steel spine (dark forged back)
b_r2:load(p_steel_dark, 1.0)
b_r2:stroke({{645, 682}, {705, 697}, {763, 711}}, {pressure = 0.95})

-- Reflected north light across blade bevel
b_r3:load(p_steel_lit, 1.0)
b_r3:stroke({{650, 683}, {707, 699}, {764, 713}}, {pressure = 0.95})

-- Razor cutting edge
b_r1:load(p_steel_lit, 1.0)
b_r1:stroke({{645, 682}, {705, 700}, {765, 715}}, {pressure = 1.0})

-- Blade contact shadow on limestone
b_r2:load(p_shadow_stone, 0.85)
b_r2:stroke({{647, 684}, {707, 702}, {764, 717}}, {pressure = {0.5, 0.9}})

-- Brass bolster collar ({763, 711} to {778, 716})
b_r5:load(p_brass, 1.0)
b_r5:stroke({{763, 711}, {771, 713}, {778, 716}}, {pressure = 1.0})
b_r2:load(p_brass_glint, 1.0)
b_r2:touch(771, 713, {pressure = 1.0})
b_r1:load(p_steel_dark, 0.9)
b_r1:stroke({{764, 715}, {778, 718}}, {pressure = 0.85})

-- Turned dark walnut handle projecting over front ledge ({778, 716} to {855, 755})
b_r5:load(p_knife_wood, 1.0)
b_r5:stroke({{778, 716}, {815, 735}, {855, 755}}, {pressure = 1.0, swell = {1.0, 1.45, 1.0}})

-- Lit upper crest of turned handle
b_r3:load(p_knife_wood_lit, 1.0)
b_r3:stroke({{779, 714}, {815, 733}, {854, 753}}, {pressure = 0.92})

-- Turned brass pommel cap at end of handle
b_r2:load(p_brass_glint, 1.0)
b_r2:touch(855, 755, {pressure = 1.0})

-- Soft handle cast shadow down front ledge face
b_r3:load(p_jug_deep, 0.8)
b_r3:stroke({{782, 716}, {786, 738}, {790, 758}}, {pressure = {0.85, 0.2}})

-- Specular chamfer highlight around knife
b_r1:load(p_chamfer, 1.0)
b_r1:stroke({{-5, 715}, {350, 715}, {763, 715}}, {pressure = {0.85, 0.85}, shake = 0.02})
b_r1:stroke({{778, 715}, {900, 715}, {1005, 715}}, {pressure = {0.85, 0.85}, shake = 0.02})

-- 5. FOREGROUND WALNUTS (BETWEEN CÁNTARO AND QUINCE)
-- A. Whole ribbed walnut at {470, 644}
b_r5:load(p_nut_shell, 1.0)
b_r5:stroke({{458, 645}, {470, 641}, {482, 645}}, {pressure = 1.0, swell = {0.9, 1.4, 0.9}})
b_r5:load(p_nut_shell, 1.0)
b_r5:stroke({{460, 647}, {471, 645}, {482, 647}}, {pressure = 1.0})

b_r2:load(p_nut_deep, 0.95)
b_r2:stroke({{458, 645}, {470, 640}, {482, 645}}, {pressure = 0.85})
b_r1:load(p_q_hi, 0.9)
b_r1:stroke({{465, 639}, {471, 638}, {477, 640}}, {pressure = 0.85})

b_r3:load(p_jug_deep, 0.95)
b_r3:stroke({{456, 649}, {470, 651}, {484, 649}}, {pressure = 0.9})
b_f8:load(p_shadow_stone, 0.55)
b_f8:stroke({{478, 648}, {502, 652}}, {pressure = {0.8, 0.2}})

-- B. Cracked walnut half with ivory kernel at {510, 652}
b_r5:load(p_nut_shell, 1.0)
b_r5:stroke({{498, 655}, {510, 658}, {524, 653}}, {pressure = 1.0, swell = {0.9, 1.4, 0.9}})

b_r3:load(p_nut_deep, 1.0)
b_r3:stroke({{500, 652}, {510, 650}, {522, 650}}, {pressure = 0.95})

b_r3:load(p_nut_meat_shade, 1.0)
b_r3:stroke({{502, 651}, {510, 650}, {520, 651}}, {pressure = 0.95})

b_r2:load(p_nut_meat, 1.0)
b_r2:stroke({{502, 650}, {506, 648}, {510, 650}}, {pressure = 1.0})
b_r2:load(p_nut_meat, 1.0)
b_r2:stroke({{511, 650}, {515, 648}, {519, 650}}, {pressure = 1.0})

b_r1:load(p_q_hi, 1.0)
b_r1:touch(505, 648, {pressure = 1.0})
b_r1:touch(514, 648, {pressure = 1.0})

b_r1:load(p_q_lit, 0.9)
b_r1:stroke({{498, 654}, {510, 657}, {524, 652}}, {pressure = 0.85})

b_r3:load(p_jug_deep, 0.95)
b_r3:stroke({{498, 658}, {512, 660}, {526, 655}}, {pressure = 0.9})
b_f8:load(p_shadow_stone, 0.55)
b_f8:stroke({{520, 656}, {545, 659}}, {pressure = {0.8, 0.2}})

print("Pictorial architecture fully consolidated.")

--@ chunk 172
-- CHUNK 172: THE TRUE PAINTERLY CAMPAIGN (BACK-TO-FRONT ARCHITECTURE)

-- Palette piles
p_wall       = pile{{"raw umber", 6.5}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.08}
p_stone_back = pile{{"lead white", 6.5}, {"yellow ochre", 2.8}, {"raw umber", 1.5}, {"red earth", 0.3}, medium = 0.08}
p_stone_mid  = pile{{"lead white", 7.5}, {"yellow ochre", 2.4}, {"raw umber", 1.0}, {"red earth", 0.2}, medium = 0.08}
p_stone_lit  = pile{{"lead white", 8.2}, {"yellow ochre", 2.0}, {"raw umber", 0.5}, medium = 0.08}
p_ledge      = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.2}, medium = 0.05}
p_shadow_stone = pile{{"raw umber", 4.8}, {"yellow ochre", 3.2}, {"lead white", 2.2}, {"red earth", 0.6}, medium = 0.10}

p_jug_rich   = pile{{"red earth", 6.2}, {"yellow ochre", 3.8}, {"lead white", 1.2}, {"raw umber", 1.0}, medium = 0.06}
p_jug_lit    = pile{{"yellow ochre", 6.0}, {"red earth", 2.5}, {"lead white", 4.5}, {"raw umber", 0.5}, medium = 0.06}
p_jug_core   = pile{{"red earth", 5.5}, {"raw umber", 4.5}, {"bone black", 1.2}, {"yellow ochre", 1.0}, medium = 0.06}
p_jug_deep   = pile{{"bone black", 6.0}, {"raw umber", 4.0}, {"red earth", 1.8}, medium = 0.05}
p_jug_reflect= pile{{"yellow ochre", 5.0}, {"red earth", 2.0}, {"lead white", 2.5}, {"raw umber", 1.5}, medium = 0.08}
p_jug_hi     = pile{{"lead white", 9.0}, {"yellow ochre", 1.5}, {"red earth", 0.2}, medium = 0.04}

p_q_rich_gold= pile{{"yellow ochre", 5.5}, {"chrome yellow", 4.5}, {"lead white", 2.2}, {"green earth", 0.5}, medium = 0.06}
p_q_lit      = pile{{"chrome yellow", 5.5}, {"yellow ochre", 2.5}, {"lead white", 5.5}, medium = 0.05}
p_q_half     = pile{{"yellow ochre", 5.0}, {"green earth", 2.0}, {"raw umber", 1.5}, {"lead white", 2.2}, medium = 0.08}
p_q_core     = pile{{"raw umber", 5.0}, {"yellow ochre", 3.5}, {"red earth", 2.5}, {"bone black", 0.8}, medium = 0.06}
p_q_deep     = pile{{"raw umber", 6.0}, {"bone black", 3.0}, {"red earth", 1.8}, medium = 0.05}
p_q_reflect  = pile{{"yellow ochre", 5.5}, {"lead white", 3.0}, {"raw umber", 1.5}, {"red earth", 0.8}, medium = 0.08}
p_q_hi       = pile{{"lead white", 9.0}, {"chrome yellow", 2.5}, {"yellow ochre", 0.6}, medium = 0.04}
p_stem       = pile{{"raw umber", 6.0}, {"bone black", 3.5}, {"yellow ochre", 1.5}, medium = 0.05}

local b_f22 = brush("filbert", 22)
local b_f18 = brush("filbert", 18)
local b_f14 = brush("filbert", 14)
local b_f10 = brush("filbert", 10)
local b_f8  = brush("filbert", 8)
local b_r5  = brush{kind = "round", width = 5.5, point = 0.9}
local b_r3  = brush{kind = "round", width = 3.2, point = 0.9}
local b_r2  = brush{kind = "round", width = 2.0, point = 0.95}
local b_r1  = brush{kind = "round", width = 1.3, point = 0.98}

-- 1. LAYER 1: RESTORING BACKGROUNDS BEHIND OBJECT CONTOURS
-- A. Dark background wall behind cántaro handle (wipes the square terracotta wing completely!)
for y = 390, 510, 10 do
  b_f14:load(p_wall, 1.0)
  b_f14:stroke({{220, y}, {260, y}, {295, y}}, {pressure = 1.0})
end

-- B. Dark background wall behind quince right shoulder (wipes the grey box completely!)
for y = 475, 520, 8 do
  b_f18:load(p_wall, 1.0)
  b_f18:stroke({{640, y}, {700, y}, {760, y}}, {pressure = 1.0})
end

-- C. Limestone tabletop behind quince right shoulder (wipes the white striped pillar completely!)
for y = 522, 590, 10 do
  b_f18:load(p_stone_back, 1.0)
  b_f18:stroke({{690, y}, {730, y}, {770, y}}, {pressure = 1.0})
end

-- 2. LAYER 2: PAINTING THE NOBLE EARTHENWARE CÁNTARO (ON TOP OF WALL)
-- A. Redraw the graceful, pulled-clay loop handle
local pts_h_outer = {{342, 365}, {295, 375}, {240, 395}, {214, 430}, {224, 475}, {252, 515}, {278, 540}}
b_r5:load(p_jug_rich, 1.0)
b_r5:stroke(pts_h_outer, {pressure = 1.0, swell = {0.9, 1.35, 1.0}})

local pts_h_under = {{338, 375}, {295, 385}, {246, 408}, {222, 438}, {232, 480}, {258, 518}, {280, 540}}
b_r3:load(p_jug_core, 1.0)
b_r3:stroke(pts_h_under, {pressure = 0.95})

b_r3:load(p_jug_lit, 1.0)
b_r3:stroke({{340, 363}, {293, 372}, {236, 392}, {212, 428}, {220, 465}}, {pressure = {0.8, 1.0, 0.8}})

b_r1:load(p_jug_hi, 1.0)
b_r1:stroke({{230, 398}, {214, 426}, {218, 442}}, {pressure = {0.5, 0.95, 0.5}})

-- B. Left belly contour curving gracefully in front of the wall
b_f14:load(p_jug_rich, 1.0)
b_f14:stroke({{330, 390}, {295, 435}, {272, 485}, {282, 535}, {312, 615}}, {pressure = 1.0})

-- C. Unified terracotta belly modeling (smooth, continuous cylindrical volume)
b_f18:load(p_jug_rich, 1.0)
b_f18:stroke({{355, 365}, {365, 485}, {370, 622}}, {pressure = 1.0})
b_f14:load(p_jug_rich, 1.0)
b_f14:stroke({{340, 370}, {340, 480}, {350, 620}}, {pressure = 0.98})
b_f14:load(p_jug_rich, 1.0)
b_f14:stroke({{375, 370}, {390, 480}, {385, 620}}, {pressure = 0.98})

-- Lit flank on left (raking north light)
b_f14:load(p_jug_lit, 1.0)
b_f14:stroke({{340, 365}, {318, 420}, {300, 480}, {310, 545}, {330, 600}, {345, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Core shadow down right flank
b_f14:load(p_jug_core, 1.0)
b_f14:stroke({{395, 365}, {425, 425}, {455, 485}, {445, 545}, {426, 595}, {408, 622}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Deep shadow on right contour
b_f10:load(p_jug_deep, 0.95)
b_f10:stroke({{400, 360}, {422, 400}, {450, 440}, {468, 485}, {456, 540}, {436, 595}, {412, 624}}, {
  pressure = 0.9
})

-- Ambient table bounce along right contour
b_f8:load(p_jug_reflect, 0.9)
b_f8:stroke({{422, 405}, {452, 445}, {468, 485}, {458, 540}, {438, 590}, {414, 622}}, {pressure = 0.45})

-- Decisive glaze impasto on shoulder curve
b_r3:load(p_jug_hi, 1.0)
b_r3:stroke({{324, 402}, {320, 418}, {316, 436}}, {pressure = {0.7, 1.0, 0.7}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_jug_hi, 1.0)
b_r1:touch(320, 418, {pressure = 1.0})

-- Foot ring and contact shadow
b_r3:load(p_jug_rich, 1.0)
b_r3:stroke({{326, 624}, {370, 626}, {414, 624}}, {pressure = 0.95})
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{322, 626}, {370, 628}, {418, 626}}, {pressure = {0.95, 1.0, 0.95}})

-- Pitcher cast shadow flowing rightward
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{414, 625}, {465, 628}, {535, 636}}, {pressure = {0.85, 0.2}, swell = {1.0, 1.3, 0.6}})

-- 3. LAYER 2: PAINTING THE RIPE GOLDEN QUINCE (ON TOP OF WALL AND TABLE)
-- A. Sculpting the entire golden volume
b_f22:load(p_q_rich_gold, 1.0)
b_f22:stroke({{630, 485}, {620, 545}, {610, 610}, {625, 674}}, {pressure = 1.0})

-- B. Natural right contour curving down from crown through shoulder into core shadow
b_f18:load(p_q_core, 1.0)
b_f18:stroke({{635, 488}, {665, 525}, {695, 575}, {715, 620}, {695, 665}, {650, 676}}, {
  pressure = {0.85, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.3, 0.85}
})

-- Deep shadow on far right contour
b_f10:load(p_q_deep, 0.95)
b_f10:stroke({{660, 520}, {688, 565}, {715, 610}, {710, 645}, {690, 670}}, {pressure = 0.9})

-- Reflected limestone bounce along shaded contour
b_f10:load(p_q_reflect, 0.9)
b_f10:stroke({{670, 535}, {698, 580}, {720, 618}, {714, 650}, {695, 670}}, {pressure = 0.45})

-- C. Luminous sunlit left lobe (chrome yellow + yellow ochre)
b_f18:load(p_q_lit, 1.0)
b_f18:stroke({{605, 490}, {575, 535}, {550, 585}, {548, 628}, {566, 662}, {598, 676}}, {
  pressure = {0.9, 1.0, 1.0, 0.95, 0.9, 0.8}, swell = {0.85, 1.35, 0.85}
})

-- Central half-tone
b_f10:load(p_q_half, 0.95)
b_f10:stroke({{625, 505}, {605, 560}, {590, 615}, {615, 668}}, {pressure = 0.92})

-- Buttery impasto highlight on the knobby sunlit crest
b_r3:load(p_q_hi, 1.0)
b_r3:stroke({{570, 565}, {562, 595}, {566, 625}}, {pressure = {0.9, 1.0, 0.75}, swell = {0.8, 1.4, 0.7}})
b_r1:load(p_q_hi, 1.0)
b_r1:touch(562, 595, {pressure = 1.0})

-- Arching gnarled woody stem
b_r2:load(p_q_deep, 1.0)
b_r2:touch(628, 486, {pressure = 0.95})
b_r2:load(p_stem, 1.0)
b_r2:stroke({{628, 486}, {634, 472}, {642, 458}, {652, 448}}, {pressure = {0.98, 0.9, 0.5}})
b_r1:load(p_q_lit, 0.85)
b_r1:stroke({{629, 484}, {635, 472}, {643, 459}}, {pressure = 0.6})

-- Calyx eye and contact shadow
b_r2:load(p_q_deep, 1.0)
b_r2:touch(650, 678, {pressure = 0.95})
b_r2:load(p_jug_deep, 1.0)
b_r2:stroke({{570, 676}, {625, 680}, {675, 678}, {710, 670}}, {pressure = {0.85, 1.0, 0.9, 0.7}})

-- Transparent cast shadow of quince
b_f14:load(p_shadow_stone, 0.55)
b_f14:stroke({{690, 678}, {740, 684}, {805, 692}}, {pressure = {0.8, 0.2}, swell = {1.0, 1.3, 0.6}})

print("Back-to-front layering complete: backgrounds restored, cántaro and quince sculpted.")

--@ chunk (finishing)
-- finishing, applied after the session by scripts/finish_painting
local function when_dry(f)
  for _ = 1, 120 do
    local ok, e = pcall(f)
    if ok then return end
    if not tostring(e):find('not all dry', 1, true) then error(e, 0) end
    wait(30 * 24 * 60)
  end
  error('still not dry after ten years', 0)
end
when_dry(function() varnish{coats=0.4} end)
when_dry(function() cracks{} end)
