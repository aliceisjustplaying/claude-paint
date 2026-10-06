-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{
  size = 500,
  aspect = 1.25,
  linen = {16, 14},
  seed = 101,
  ground = {
    {pile = {{"lead white", 8}, {"raw umber", 2}}, um = 90, apply = "knife", texture = 0.25},
    {pile = {{"lead white", 6}, {"yellow ochre", 2}, {"raw umber", 2}}, um = 35, apply = "brush"}
  }
}

--@ chunk 2
-- Underdrawing: establishing composition with chalk and pencil
local ch = chalk()
local p = pencil("HB")

-- Table edge lines
ch:line({{0, 525}, {1000, 520}}, {pressure = 0.35, smooth = false})
ch:line({{0, 675}, {1000, 675}}, {pressure = 0.45, smooth = false})

-- Pitcher contours
-- Lip and rim
ch:sketch({{545, 310}, {590, 303}, {635, 310}, {640, 320}, {590, 327}, {540, 320}, {545, 310}}, {passes = 2, wander = 1.5, pressure = 0.35})
-- Neck
ch:line({{550, 320}, {552, 370}}, {pressure = 0.4})
ch:line({{630, 320}, {628, 370}}, {pressure = 0.4})
-- Belly contour left
ch:sketch({{552, 370}, {515, 420}, {500, 480}, {510, 550}, {535, 605}, {550, 625}}, {passes = 2, wander = 2, pressure = 0.4})
-- Belly contour right
ch:sketch({{628, 370}, {665, 420}, {680, 480}, {670, 550}, {645, 605}, {630, 625}}, {passes = 2, wander = 2, pressure = 0.4})
-- Base
ch:sketch({{550, 625}, {590, 632}, {630, 625}}, {passes = 2, wander = 1.5, pressure = 0.4})
-- Handle
ch:sketch({{628, 355}, {675, 365}, {695, 410}, {690, 460}, {665, 490}}, {passes = 2, wander = 2, pressure = 0.4})
ch:sketch({{628, 370}, {660, 380}, {675, 415}, {672, 455}, {655, 485}}, {passes = 2, wander = 1.5, pressure = 0.35})

-- Napkin draped on left
ch:sketch({{150, 545}, {210, 555}, {260, 580}, {280, 630}, {270, 675}, {265, 735}, {230, 745}, {210, 730}, {180, 675}, {140, 660}}, {passes = 2, wander = 2.5, pressure = 0.35})
-- Fold creases in napkin
p:sketch({{180, 550}, {220, 620}, {250, 675}, {245, 735}}, {passes = 2, wander = 1.5, pressure = 0.3})
p:sketch({{220, 565}, {255, 630}, {265, 675}}, {passes = 2, wander = 1.5, pressure = 0.3})

-- Whole lemon (left-center)
ch:sketch({{305, 625}, {320, 605}, {365, 595}, {415, 615}, {425, 640}, {405, 665}, {355, 672}, {315, 655}, {305, 625}}, {passes = 3, wander = 2, pressure = 0.4})
-- Lemon half (cut face tilted)
ch:sketch({{445, 650}, {470, 625}, {515, 625}, {540, 650}, {535, 680}, {495, 700}, {455, 685}, {445, 650}}, {passes = 3, wander = 2, pressure = 0.4})
-- Inner pith circle for lemon half
p:sketch({{455, 652}, {475, 635}, {510, 635}, {528, 652}, {523, 675}, {492, 690}, {462, 678}, {455, 652}}, {passes = 2, wander = 1, pressure = 0.3})

-- Walnut on right
ch:sketch({{725, 655}, {740, 642}, {760, 650}, {765, 665}, {745, 672}, {730, 668}, {725, 655}}, {passes = 2, wander = 1.5, pressure = 0.35})
p:line({{742, 643}, {744, 671}}, {pressure = 0.4}) -- seam

-- Light shadow indications
p:hatch(ellipse(620, 645, 120, 25), {angle = 0.2, spacing = 8, pressure = 0.25})
p:hatch(ellipse(380, 670, 70, 18), {angle = 0.2, spacing = 8, pressure = 0.2})
p:hatch(ellipse(500, 695, 60, 15), {angle = 0.2, spacing = 8, pressure = 0.2})

print("Underdrawing completed.")

--@ chunk 3
-- Wall / Background lay-in
p_wall_dark = pile{{"raw umber", 5}, {"bone black", 3}, {"red earth", 1}, medium = 0.2}
p_wall_mid = pile{{"raw umber", 4}, {"yellow ochre", 3}, {"lead white", 2}, {"bone black", 1}, medium = 0.2}
p_wall_light = pile{{"lead white", 5}, {"yellow ochre", 3}, {"raw umber", 2}, medium = 0.25}

-- Pitcher silhouette mask to reserve the canvas
local pitcher_pts = {
  {545, 305}, {635, 305}, {645, 320}, {630, 365},
  {675, 360}, {700, 410}, {695, 465}, {665, 495}, {678, 510},
  {670, 560}, {645, 610}, {630, 628},
  {550, 628}, {535, 605}, {508, 550}, {500, 480}, {515, 420},
  {550, 370}, {545, 320}
}
pitcher_mask = poly(pitcher_pts, true):grow(4)

-- Wall area (above table line y = 525)
local wall_mask = rect(0, 0, 1000, 525) - pitcher_mask

-- Left wall (illuminated area)
local m_left = wall_mask * mask(function(x, y)
  local v = (500 - x) * 0.7 + (400 - y) * 0.5
  return smoothstep(-50, 150, v)
end)

-- Right wall (deep shadow)
local m_right = wall_mask * mask(function(x, y)
  local v = (x - 450) * 0.7 + (y - 100) * 0.3
  return smoothstep(-50, 150, v)
end)

-- Paint the passages
work(wall_mask, {hand = "broad", pile = p_wall_mid, angle = -0.4, coverage = 1.2, edge = 0.4})
work(m_right, {hand = "broad", pile = p_wall_dark, angle = -0.3, coverage = 1.4, edge = 0.6})
work(m_left, {hand = "broad", pile = p_wall_light, angle = -0.5, coverage = 1.3, edge = 0.5})

-- Soft diagonal blend across the wall to create continuous atmospheric space
blend(wall_mask, {angle = -0.4, coverage = 1.5})

print("Background blocked in successfully.")

--@ chunk 4
-- Table top and front apron lay-in

-- Define reserved masks for foreground objects
local napkin_pts = {
  {140, 545}, {210, 555}, {260, 580}, {280, 630}, {270, 675},
  {265, 745}, {225, 755}, {205, 735}, {175, 675}, {135, 650}
}
napkin_mask = poly(napkin_pts, true):shrink(2)
lemon_mask = ellipse(365, 635, 58, 38):shrink(2)
lemon_half_mask = ellipse(490, 660, 44, 34):shrink(2)
walnut_mask = ellipse(745, 660, 18, 14):shrink(2)

local reserved = napkin_mask + lemon_mask + lemon_half_mask + walnut_mask

-- Table top mask (y from 522 to 675)
local table_top = (rect(0, 520, 1000, 155) - reserved)

-- Table front apron (y from 675 to 800)
local table_front = (rect(0, 675, 1000, 125) - napkin_mask)

-- Table top piles
p_table_top = pile{{"yellow ochre", 5}, {"raw umber", 4}, {"lead white", 2}, {"red earth", 1}, medium = 0.15}
p_table_top_light = pile{{"yellow ochre", 5}, {"lead white", 4}, {"raw umber", 2}, {"red earth", 1}, medium = 0.15}
p_table_front = pile{{"raw umber", 6}, {"bone black", 3}, {"red earth", 2}, medium = 0.15}
p_table_front_warm = pile{{"raw umber", 5}, {"red earth", 3}, {"yellow ochre", 2}, medium = 0.15}
p_cast_shadow = pile{{"raw umber", 6}, {"bone black", 3}, {"red earth", 2}, medium = 0.25}

-- 1. Table top surface: horizontal wood strokes
local m_top_left = table_top * mask(function(x, y) return smoothstep(700, 200, x) end)
local m_top_right = table_top * mask(function(x, y) return smoothstep(300, 800, x) end)

work(table_top, {hand = "body", pile = p_table_top, angle = 0, coverage = 1.6, fill = true, edge = 0.2})
work(m_top_left, {hand = "body", pile = p_table_top_light, angle = 0, coverage = 1.3, fill = true, edge = 0.4})
work(m_top_right, {hand = "body", pile = p_table_top, angle = 0, coverage = 1.2, fill = true, edge = 0.3})

-- Blend wood surface horizontally along the grain
blend(table_top, {angle = 0, coverage = 1.2})

-- 2. Table front apron in shadow
work(table_front, {hand = "body", pile = p_table_front, angle = 0, coverage = 1.5, fill = true, edge = 0.2})
local m_front_bottom = table_front * mask(function(x, y) return smoothstep(680, 780, y) end)
work(m_front_bottom, {hand = "body", pile = p_table_front_warm, angle = 0, coverage = 1.0, edge = 0.5})
blend(table_front, {angle = 0, coverage = 1.2})

-- 3. Cast shadows on the table top
-- Shadow from pitcher (stretches to the right and slightly back/down)
local pitcher_shadow = poly({
  {580, 625}, {630, 625}, {720, 630}, {760, 635}, {780, 650},
  {760, 665}, {710, 665}, {620, 640}, {570, 630}
}, true) * table_top

-- Shadow from whole lemon
local lemon_shadow = poly({
  {360, 665}, {410, 665}, {440, 672}, {430, 680}, {390, 682}, {350, 675}
}, true) * table_top

-- Shadow from half lemon
local half_shadow = poly({
  {490, 685}, {535, 685}, {560, 695}, {540, 705}, {480, 700}
}, true) * (table_top + table_front)

-- Shadow from napkin
local napkin_shadow = poly({
  {240, 670}, {285, 670}, {295, 710}, {280, 755}, {265, 755}, {260, 710}
}, true) * (table_top + table_front)

work(pitcher_shadow, {hand = "glaze", pile = p_cast_shadow, angle = 0.1, coverage = 1.6, edge = 0.5})
work(lemon_shadow, {hand = "glaze", pile = p_cast_shadow, angle = 0.1, coverage = 1.4, edge = 0.4})
work(half_shadow, {hand = "glaze", pile = p_cast_shadow, angle = 0.1, coverage = 1.4, edge = 0.4})
work(napkin_shadow, {hand = "glaze", pile = p_cast_shadow, angle = 0.2, coverage = 1.5, edge = 0.5})

print("Table and cast shadows laid in.")

--@ chunk 5
print("Drying wall:", drying(300, 200), drying(800, 200))

--@ chunk 6
print("Table drying:", drying(500, 600), drying(500, 720))

--@ chunk 7
-- Soften table front while paint is still open
blend(rect(0, 675, 1000, 125), {angle = 0, coverage = 1.5})
print("Table apron blended.")

--@ chunk 8
print(wait(24 * 60))

--@ chunk 9
print("Drying:", drying(200, 200), drying(800, 200), drying(500, 600), drying(500, 720))

--@ chunk 10
print(wait(24 * 60)); print("Table drying:", drying(500, 600), drying(500, 720))

--@ chunk 11
-- Modeling the Glazed Earthenware Pitcher

-- Define Pitcher Piles
p_pot_base = pile{{"red earth", 5}, {"yellow ochre", 3}, {"raw umber", 2}, medium = 0.15}
p_pot_dark = pile{{"raw umber", 5}, {"bone black", 3}, {"red earth", 2}, medium = 0.1}
p_pot_deep = pile{{"bone black", 6}, {"raw umber", 4}, medium = 0.05}
p_pot_refl = pile{{"yellow ochre", 4}, {"red earth", 3}, {"raw umber", 2}, {"lead white", 1}, medium = 0.15}
p_pot_mid = pile{{"yellow ochre", 5}, {"red earth", 3}, {"lead white", 2}, {"raw umber", 1}, medium = 0.15}
p_pot_lit = pile{{"yellow ochre", 5}, {"lead white", 4}, {"red earth", 1}, {"raw umber", 1}, medium = 0.15}
p_pot_high = pile{{"lead white", 7}, {"yellow ochre", 2}, {"raw umber", 1}, medium = 0.1}
p_pot_glaze_olive = pile{{"yellow ochre", 4}, {"raw umber", 3}, {"green earth", 2}, {"lead white", 1}, medium = 0.2}

-- Geometry of pitcher
local body_pts = {
  {545, 310}, {590, 305}, {635, 310}, {642, 322}, {630, 370},
  {665, 415}, {680, 475}, {672, 545}, {645, 605}, {630, 626},
  {590, 631}, {550, 626}, {535, 605}, {508, 545}, {500, 475},
  {515, 415}, {550, 370}, {538, 322}
}
pitcher_full = poly(body_pts, true)

-- Handle
local handle_pts = {
  {630, 355}, {675, 365}, {696, 410}, {692, 455}, {668, 485}
}
local handle_ribbon = ribbon(handle_pts, 15)

-- Combined silhouette
local jug_all = pitcher_full + handle_ribbon

-- Sub-regions
local jug_interior = ellipse(590, 313, 38, 9)
local jug_rim = ribbon({{545, 318}, {590, 324}, {635, 318}}, 6)

-- 1. Base terracotta coat over the whole jug
work(jug_all, {hand = "body", pile = p_pot_base, angle = 0.1, coverage = 1.6, fill = true, edge = 0.2})

-- 2. Interior dark opening
work(jug_interior, {hand = "detail", pile = p_pot_deep, angle = 0, coverage = 2.0, clip = true})

-- 3. Core shadow on the right side of the belly and neck
local jug_shadow = jug_all * mask(function(x, y)
  -- Terminator runs diagonally across x ≈ 590 to 620
  local t = (x - 580) / 70
  return clamp(t, 0, 1)
end) - jug_interior
work(jug_shadow, {hand = "body", pile = p_pot_dark, angle = 0.15, coverage = 1.5, edge = 0.4})

-- 4. Cast shadow under handle on right belly
local handle_cast = ellipse(655, 430, 18, 35) * jug_all
work(handle_cast, {hand = "body", pile = p_pot_deep, angle = 0.2, coverage = 1.3, edge = 0.4})

-- 5. Reflected warm light along right edge (bouncing from table and room)
local jug_refl = jug_all * mask(function(x, y)
  if x < 645 then return 0 end
  local t = smoothstep(645, 675, x) * smoothstep(360, 420, y) * smoothstep(625, 550, y)
  return t * 0.7
end)
work(jug_refl, {hand = "scumble", pile = p_pot_refl, angle = 0.3, coverage = 1.2, edge = 0.4})

-- 6. Lit area on the left shoulder, belly and neck
local jug_lit = jug_all * mask(function(x, y)
  local t = (600 - x) / 90
  return clamp(t, 0, 1)
end) - jug_interior
work(jug_lit, {hand = "body", pile = p_pot_mid, angle = 0.1, coverage = 1.5, edge = 0.3})

-- Olive glaze wash over upper belly and shoulder
local jug_olive = jug_lit * ellipse(560, 460, 50, 70)
work(jug_olive, {hand = "glaze", pile = p_pot_glaze_olive, angle = 0.1, coverage = 1.2, edge = 0.5})

-- Stronger light on left shoulder curve
local jug_light_accent = jug_all * ellipse(545, 450, 28, 45)
work(jug_light_accent, {hand = "body", pile = p_pot_lit, angle = 0.05, coverage = 1.4, edge = 0.3})

-- 7. Rim and handle lighting
-- Rim lit edge
local rim_lit = jug_rim * mask(function(x, y) return x < 590 and 1 or 0 end)
work(rim_lit, {hand = "detail", pile = p_pot_lit, angle = 0.1, coverage = 1.5})
-- Handle top lit
local handle_lit = handle_ribbon * mask(function(x, y) return y < 410 and 1 or 0 end)
work(handle_lit, {hand = "detail", pile = p_pot_mid, angle = -0.5, coverage = 1.4})

-- 8. Blend across the belly and neck to fuse the ceramic glaze wet-into-wet
blend(jug_all - jug_interior, {angle = 0.1, coverage = 1.3})

-- 9. Crisp impasto highlights on the glaze
local b_point = brush{kind = "round", width = 3.5, point = 1, stiffness = 0.6}
b_point:load(p_pot_high, 0.8)
-- Specular dot/dash on shoulder
b_point:stroke({{538, 435}, {542, 455}, {544, 470}}, {pressure = {0.7, 0.9, 0.3}, orient = "along"})
-- Crisp touch on the lit rim
b_point:load(p_pot_high, 0.7)
b_point:stroke({{550, 316}, {570, 321}, {585, 322}}, {pressure = {0.6, 0.8, 0.2}, orient = "along"})
-- Touch on handle apex
b_point:load(p_pot_high, 0.5)
b_point:touch(685, 395, {pressure = 0.6})

print("Pitcher modeled with glazed ceramic form and specular highlights.")

--@ chunk 12
-- Refining the pitcher, handle opening, and base contact

-- 1. Fill handle opening with background dark wall paint
local handle_hole = poly({
  {636, 372}, {664, 388}, {675, 420}, {664, 462}, {652, 475}, {634, 400}
}, true)
work(handle_hole, {hand = "detail", pile = p_wall_dark, angle = -0.4, coverage = 2.0, clip = true})

-- 2. Fuse the shadow side of the belly and neck
local jug_shadow_area = pitcher_full * mask(function(x, y)
  return smoothstep(575, 630, x)
end) - ellipse(590, 313, 38, 9)

work(jug_shadow_area, {hand = "glaze", pile = p_pot_dark, angle = 1.3, coverage = 1.2, edge = 0.4})
blend(jug_shadow_area, {angle = 1.4, coverage = 1.5})

-- 3. Occlusion / contact shadow at base of jug
local jug_contact = ribbon({{545, 627}, {590, 632}, {632, 627}}, 6)
local p_occlusion = pile{{"bone black", 7}, {"raw umber", 3}, medium = 0.05}
work(jug_contact, {hand = "detail", pile = p_occlusion, angle = 0, coverage = 1.8, edge = 0.2})

-- 4. Cast shadow immediately behind and right of jug on the table
local jug_near_shadow = poly({
  {570, 627}, {630, 627}, {680, 635}, {720, 642}, {670, 645}, {600, 635}
}, true)
work(jug_near_shadow, {hand = "glaze", pile = p_cast_shadow, angle = 0.1, coverage = 1.4, edge = 0.3})

print("Pitcher refined and anchored.")

--@ chunk 13
-- Soften and unify the background wall and touch up handle opening

-- 1. Complete coverage in handle opening
local inner_handle_patch = ellipse(662, 420, 15, 45) * mask(function(x, y) return (x < 680 and x > 635) and 1 or 0 end)
work(inner_handle_patch, {hand = "detail", pile = p_wall_dark, angle = -0.4, coverage = 2.5, clip = true})

-- 2. Atmospheric veil over background wall
local handle_ribbon = ribbon({{630, 355}, {675, 365}, {696, 410}, {692, 455}, {668, 485}}, 16)
local wall_active = rect(0, 0, 1000, 525) - pitcher_full - handle_ribbon

-- Thin unifying glazes
local p_wall_glaze_dark = pile{{"raw umber", 5}, {"bone black", 3}, {"red earth", 1}, medium = 0.4}
local p_wall_glaze_light = pile{{"lead white", 5}, {"yellow ochre", 3}, {"raw umber", 2}, medium = 0.35}

local m_right_wall = wall_active * mask(function(x, y) return smoothstep(350, 750, x) end)
local m_left_wall = wall_active * mask(function(x, y) return smoothstep(650, 250, x) end)

work(m_right_wall, {hand = "glaze", pile = p_wall_glaze_dark, angle = -0.3, coverage = 1.4, edge = 0.4})
work(m_left_wall, {hand = "glaze", pile = p_wall_glaze_light, angle = -0.5, coverage = 1.3, edge = 0.4})

-- Soft broad blending pass across the entire wall
blend(wall_active, {angle = -0.4, coverage = 1.8})

print("Background unified into atmospheric chiaroscuro.")

--@ chunk 14
-- Blend across the transition zone in the background wall
local wall_transition = rect(350, 0, 200, 520) - pitcher_full
blend(wall_transition, {angle = -0.3, coverage = 2.0})
blend(wall_transition, {angle = 0, coverage = 1.5})
print("Wall transition smoothly blended.")

--@ chunk 15
local handle_ribbon = ribbon({{630, 355}, {675, 365}, {696, 410}, {692, 455}, {668, 485}}, 16)
local wall_mask_full = rect(0, 0, 1000, 525) - pitcher_full - handle_ribbon
blend(wall_mask_full, {angle = 0, coverage = 2.2})
print("Full wall blended seamlessly.")

--@ chunk 16
-- Painting the Whole Lemon and Cut Lemon Half

-- Lemon piles
p_lemon_lit = pile{{"chrome yellow", 5}, {"lead white", 4}, {"yellow ochre", 1}, medium = 0.1}
p_lemon_pure = pile{{"chrome yellow", 6}, {"yellow ochre", 3}, {"lead white", 1}, medium = 0.1}
p_lemon_mid = pile{{"yellow ochre", 6}, {"chrome yellow", 3}, {"raw umber", 1}, medium = 0.15}
p_lemon_green = pile{{"yellow ochre", 4}, {"green earth", 3}, {"chrome yellow", 2}, {"lead white", 1}, medium = 0.15}
p_lemon_shadow = pile{{"raw umber", 4}, {"yellow ochre", 4}, {"green earth", 2}, medium = 0.15}
p_lemon_refl = pile{{"yellow ochre", 5}, {"red earth", 2}, {"lead white", 2}, {"raw umber", 1}, medium = 0.2}
p_lemon_high = pile{{"lead white", 8}, {"chrome yellow", 2}, medium = 0.05}
p_pith = pile{{"lead white", 7}, {"yellow ochre", 2}, {"raw umber", 1}, medium = 0.1}
p_pulp_deep = pile{{"yellow ochre", 5}, {"chrome yellow", 3}, {"raw umber", 2}, medium = 0.2}
p_pulp_lit = pile{{"chrome yellow", 5}, {"lead white", 3}, {"yellow ochre", 2}, medium = 0.2}
p_sparkle = pile{{"lead white", 9}, {"chrome yellow", 1}, medium = 0.05}

-- -------------------------------------------------------------
-- 1. WHOLE LEMON
-- -------------------------------------------------------------
local whole_pts = {
  {305, 628}, {312, 615}, {330, 604}, {365, 597}, {395, 604},
  {418, 618}, {425, 638}, {415, 655}, {395, 668}, {365, 674},
  {335, 668}, {315, 652}, {305, 638}
}
local m_lemon_whole = poly(whole_pts, true)

-- Body color lay-in
work(m_lemon_whole, {hand = "body", pile = p_lemon_mid, angle = 0.2, coverage = 1.6, fill = true, edge = 0.2})

-- Lit upper-left volume
local m_w_lit = m_lemon_whole * mask(function(x, y)
  local v = (400 - x) * 0.6 + (665 - y) * 0.7
  return smoothstep(20, 70, v)
end)
work(m_w_lit, {hand = "body", pile = p_lemon_lit, angle = 0.2, coverage = 1.5, edge = 0.3})

-- Pure vibrant yellow in transition
local m_w_pure = m_lemon_whole * mask(function(x, y)
  local v = (x - 310) * (420 - x) * (y - 600) * (670 - y)
  return smoothstep(50000, 150000, v)
end)
work(m_w_pure, {hand = "body", pile = p_lemon_pure, angle = 0.2, coverage = 1.2, edge = 0.4})

-- Greenish tint near tip/nipple
local m_w_green = m_lemon_whole * ellipse(315, 626, 18, 14)
work(m_w_green, {hand = "glaze", pile = p_lemon_green, angle = 0.1, coverage = 1.3, edge = 0.4})

-- Core shadow on lower-right underbelly
local m_w_shadow = m_lemon_whole * mask(function(x, y)
  local v = (x - 340) * 0.5 + (y - 625) * 0.7
  return smoothstep(25, 60, v)
end)
work(m_w_shadow, {hand = "body", pile = p_lemon_shadow, angle = 0.3, coverage = 1.4, edge = 0.4})

-- Warm reflected light from table along bottom contour
local m_w_refl = m_lemon_whole * mask(function(x, y)
  if y < 652 then return 0 end
  return smoothstep(652, 672, y) * smoothstep(320, 360, x) * smoothstep(420, 380, x)
end)
work(m_w_refl, {hand = "glaze", pile = p_lemon_refl, angle = 0, coverage = 1.2, edge = 0.4})

-- Blend the whole lemon smoothly to round the volume
blend(m_lemon_whole, {angle = 0.2, coverage = 1.3})

-- Textured dimpling on peel (citrus pores)
stipple(m_w_lit, {pile = p_lemon_pure, width = 1.8, coverage = 0.9, feather = 0.3})
stipple(m_w_lit * ellipse(350, 620, 30, 20), {pile = p_lemon_lit, width = 1.6, coverage = 0.8, feather = 0.4})

-- Brilliant impasto highlight on the crest of the lemon
local b_high = brush{kind = "round", width = 3.5, point = 1, stiffness = 0.6}
b_high:load(p_lemon_high, 0.8)
b_high:stroke({{342, 615}, {355, 616}, {368, 620}}, {pressure = {0.7, 0.9, 0.3}, orient = "along"})
b_high:load(p_lemon_high, 0.5)
b_high:touch(310, 625, {pressure = 0.6}) -- tip accent

-- Dark contact occlusion under whole lemon
local b_dark = brush{kind = "round", width = 2.5, point = 1, stiffness = 0.5}
b_dark:load(p_pot_deep, 0.8)
b_dark:stroke({{335, 672}, {365, 675}, {395, 672}}, {pressure = {0.5, 0.9, 0.3}, orient = "along"})


-- -------------------------------------------------------------
-- 2. CUT LEMON HALF
-- -------------------------------------------------------------
local cut_pts = {
  {445, 652}, {460, 630}, {490, 624}, {520, 632}, {538, 652},
  {534, 678}, {515, 696}, {485, 703}, {458, 692}, {445, 672}
}
local m_lemon_cut = poly(cut_pts, true)

-- Cut face ellipse
local cx, cy = 488, 662
local rx, ry = 44, 37
local m_cut_face = ellipse(cx, cy, rx, ry)

-- 1. Outer rind (yellow skin rim)
work(m_lemon_cut, {hand = "body", pile = p_lemon_pure, angle = 0.2, coverage = 1.6, fill = true, edge = 0.2})

-- 2. Pale pith ring (albedo)
local m_pith_ring = ellipse(cx, cy, rx - 3, ry - 3) - ellipse(cx, cy, rx - 9, ry - 8)
work(m_pith_ring, {hand = "detail", pile = p_pith, angle = 0.1, coverage = 1.8, clip = true})

-- 3. Translucent pulp background
local m_pulp_area = ellipse(cx, cy, rx - 9, ry - 8)
work(m_pulp_area, {hand = "body", pile = p_pulp_deep, angle = 0.2, coverage = 1.6, fill = true, edge = 0.3})
work(m_pulp_area, {hand = "glaze", pile = p_pulp_lit, angle = 0, coverage = 1.3, edge = 0.4})

-- 4. Central pith hub and radial membranes (spokes)
local b_pith = brush{kind = "round", width = 1.8, point = 1, stiffness = 0.5}
b_pith:load(p_pith, 0.8)
-- Central hub
b_pith:touch(cx, cy, {pressure = 0.8})

-- Radial spokes
local n_segments = 9
for i = 1, n_segments do
  local ang = (i - 1) * (2 * math.pi / n_segments) + 0.15
  local x_end = cx + (rx - 9) * math.cos(ang)
  local y_end = cy + (ry - 8) * math.sin(ang)
  b_pith:load(p_pith, 0.7)
  b_pith:stroke({{cx, cy}, {cx + (x_end - cx) * 0.5, cy + (y_end - cy) * 0.5}, {x_end, y_end}},
    {pressure = {0.8, 0.6, 0.4}, orient = "along"})
end

-- 5. Moist, glistening juice sacs (sparkles and wet highlights)
local b_spark = brush{kind = "round", width = 1.5, point = 1, stiffness = 0.7}
for i = 1, n_segments do
  local ang1 = (i - 1) * (2 * math.pi / n_segments) + 0.15
  local ang2 = i * (2 * math.pi / n_segments) + 0.15
  local mid_ang = (ang1 + ang2) / 2
  local r_mid = (rx - 16) * 0.65
  local px = cx + r_mid * math.cos(mid_ang)
  local py = cy + (r_mid * (ry / rx)) * math.sin(mid_ang)
  
  -- Glistening dot on wet pulp
  b_spark:load(p_sparkle, 0.8)
  b_spark:touch(px, py, {pressure = 0.7})
  b_spark:touch(px + 3 * math.cos(mid_ang + 1.2), py + 2 * math.sin(mid_ang + 1.2), {pressure = 0.5})
end

-- Top lit edge of cut rind
local m_rind_lit = ribbon({{455, 638}, {485, 626}, {515, 632}}, 4)
work(m_rind_lit, {hand = "detail", pile = p_lemon_lit, angle = 0, coverage = 1.5})

-- Contact occlusion under half lemon
b_dark:load(p_pot_deep, 0.8)
b_dark:stroke({{475, 700}, {500, 704}, {525, 698}}, {pressure = {0.5, 0.9, 0.3}, orient = "along"})

print("Lemons completed.")

--@ chunk 17
-- Painting the Linen Napkin and the Walnut

-- Linen Piles
p_linen_high = pile{{"lead white", 9}, {"yellow ochre", 1}, medium = 0.05}
p_linen_lit = pile{{"lead white", 8}, {"yellow ochre", 1}, {"raw umber", 1}, medium = 0.1}
p_linen_mid = pile{{"lead white", 6}, {"raw umber", 2}, {"bone black", 1}, {"yellow ochre", 1}, medium = 0.15}
p_linen_shadow = pile{{"lead white", 5}, {"bone black", 2}, {"raw umber", 2}, {"cobalt blue", 1}, medium = 0.15}
p_linen_deep = pile{{"raw umber", 4}, {"bone black", 3}, {"lead white", 2}, medium = 0.1}

-- Walnut Piles
p_nut_base = pile{{"yellow ochre", 5}, {"raw umber", 4}, {"red earth", 2}, medium = 0.15}
p_nut_dark = pile{{"raw umber", 6}, {"bone black", 3}, {"red earth", 1}, medium = 0.1}
p_nut_lit = pile{{"yellow ochre", 5}, {"lead white", 3}, {"raw umber", 2}, medium = 0.1}
p_nut_high = pile{{"lead white", 7}, {"yellow ochre", 3}, medium = 0.05}

-- -------------------------------------------------------------
-- 1. LINEN NAPKIN
-- -------------------------------------------------------------
local napkin_pts = {
  {140, 545}, {205, 555}, {255, 580}, {278, 630}, {270, 675},
  {265, 745}, {225, 755}, {205, 735}, {175, 675}, {135, 650}
}
local m_napkin = poly(napkin_pts, true):grow(2)

-- Base coat of warm linen body
work(m_napkin, {hand = "body", pile = p_linen_lit, angle = 0.4, coverage = 1.6, fill = true, edge = 0.2})

-- Upper flat plane of the napkin on the table
local m_napkin_top = m_napkin * mask(function(x, y) return y <= 675 and 1 or 0 end)
-- Front draped overhang hanging down over table edge
local m_napkin_drape = m_napkin * mask(function(x, y) return y > 675 and 1 or 0 end)

-- Fold shadows: cool diagonal troughs running across the cloth
local fold1 = ribbon({{160, 555}, {205, 620}, {225, 675}, {215, 740}}, 18) * m_napkin
local fold2 = ribbon({{220, 570}, {250, 635}, {255, 675}}, 14) * m_napkin
work(fold1, {hand = "body", pile = p_linen_shadow, angle = 1.2, coverage = 1.4, edge = 0.4})
work(fold2, {hand = "body", pile = p_linen_shadow, angle = 1.2, coverage = 1.3, edge = 0.4})

-- Deepest crease line inside fold1
local b_crease = brush{kind = "round", width = 2.2, point = 1, stiffness = 0.6}
b_crease:load(p_linen_deep, 0.7)
b_crease:stroke({{175, 570}, {210, 630}, {226, 675}, {218, 735}}, {pressure = {0.3, 0.7, 0.4}, orient = "along"})

-- Light planes: sunlit crests of the folds
local crest1 = ribbon({{145, 548}, {185, 605}, {200, 672}, {190, 730}}, 16) * m_napkin
local crest2 = ribbon({{200, 560}, {235, 620}, {248, 674}, {245, 740}}, 14) * m_napkin
work(crest1, {hand = "body", pile = p_linen_lit, angle = 0.4, coverage = 1.5, edge = 0.3})
work(crest2, {hand = "body", pile = p_linen_lit, angle = 0.4, coverage = 1.4, edge = 0.3})

-- Soft blend to unite the linen folds
blend(m_napkin, {angle = 0.4, coverage = 1.2})

-- Brilliant impasto touches along the sharp corner where cloth bends over table edge
local b_impasto = brush{kind = "filbert", width = 5, stiffness = 0.7}
b_impasto:load(p_linen_high, 0.9)
b_impasto:stroke({{175, 673}, {205, 674}, {245, 674}, {270, 675}}, {pressure = {0.6, 0.9, 0.5}, orient = "across"})

-- Impasto ridge along main fold crest
local b_line = brush{kind = "round", width = 2.8, point = 1, stiffness = 0.6}
b_line:load(p_linen_high, 0.8)
b_line:stroke({{150, 552}, {180, 600}, {198, 670}}, {pressure = {0.5, 0.8, 0.4}, orient = "along"})
b_line:load(p_linen_high, 0.8)
b_line:stroke({{198, 675}, {190, 725}}, {pressure = {0.7, 0.5, 0.2}, orient = "along"})

-- Crisp crisp hem / bottom edge of the hanging cloth
b_line:load(p_linen_lit, 0.7)
b_line:stroke({{265, 745}, {245, 752}, {225, 755}, {205, 735}}, {pressure = {0.6, 0.8, 0.5}, orient = "along"})

-- Occlusion shadow under the hanging drape onto the dark table apron
local b_dark = brush{kind = "round", width = 3.0, point = 1, stiffness = 0.6}
b_dark:load(p_pot_deep, 0.8)
b_dark:stroke({{266, 748}, {245, 755}, {225, 758}, {205, 740}}, {pressure = {0.4, 0.8, 0.3}, orient = "along"})


-- -------------------------------------------------------------
-- 2. WALNUT
-- -------------------------------------------------------------
local walnut_pts = {
  {725, 655}, {735, 644}, {750, 642}, {762, 650}, {768, 662},
  {758, 672}, {742, 673}, {728, 666}, {725, 655}
}
local m_walnut = poly(walnut_pts, true):grow(2)

-- Base tone
work(m_walnut, {hand = "body", pile = p_nut_base, angle = 0.1, coverage = 1.8, fill = true, edge = 0.2})

-- Shadow side (right and underside)
local m_nut_shadow = m_walnut * mask(function(x, y) return (x > 745 or y > 660) and 1 or 0 end)
work(m_nut_shadow, {hand = "body", pile = p_nut_dark, angle = 0.2, coverage = 1.5, edge = 0.3})

-- Lit side (upper-left shell hemisphere)
local m_nut_lit = m_walnut * mask(function(x, y) return (x <= 745 and y <= 660) and 1 or 0 end)
work(m_nut_lit, {hand = "body", pile = p_nut_lit, angle = 0.1, coverage = 1.5, edge = 0.3})

-- Seam line down center of walnut
b_crease:load(p_nut_dark, 0.8)
b_crease:stroke({{744, 642}, {746, 656}, {744, 672}}, {pressure = {0.4, 0.8, 0.5}, orient = "along"})

-- Rugged shell texture (stippling)
stipple(m_walnut, {pile = p_nut_dark, width = 1.4, coverage = 0.8, feather = 0.2})
stipple(m_nut_lit, {pile = p_nut_high, width = 1.3, coverage = 0.7, feather = 0.3})

-- Specular ridge highlight on shell edge
b_line:load(p_nut_high, 0.7)
b_line:stroke({{738, 644}, {745, 643}}, {pressure = {0.5, 0.8, 0.2}, orient = "along"})

-- Contact and cast shadow of walnut on table
b_dark:load(p_pot_deep, 0.8)
b_dark:stroke({{726, 667}, {742, 674}, {760, 671}}, {pressure = {0.4, 0.8, 0.3}, orient = "along"})
local m_nut_cast = ellipse(772, 668, 16, 7)
work(m_nut_cast, {hand = "glaze", pile = p_cast_shadow, angle = 0.1, coverage = 1.4, edge = 0.4})

print("Linen napkin and walnut painted.")

--@ chunk 18
-- Sculpting and harmonizing the linen napkin

local napkin_pts = {
  {140, 545}, {205, 555}, {255, 580}, {278, 630}, {270, 675},
  {265, 745}, {225, 755}, {205, 735}, {175, 675}, {135, 650}
}
local m_napkin = poly(napkin_pts, true):grow(2)

-- Warm linen glazes to tone down the chalky brightness
local p_linen_tone = pile{{"lead white", 6}, {"yellow ochre", 2}, {"raw umber", 2}, medium = 0.25}
local p_linen_shadow_cool = pile{{"lead white", 4}, {"raw umber", 2}, {"bone black", 2}, {"cobalt blue", 2}, medium = 0.2}
local p_linen_shadow_deep = pile{{"raw umber", 5}, {"bone black", 3}, {"cobalt blue", 1}, {"lead white", 1}, medium = 0.15}
local p_linen_crest = pile{{"lead white", 8}, {"yellow ochre", 2}, medium = 0.1}

-- 1. Unifying wash over the napkin to bring it into tonal harmony with the room
work(m_napkin, {hand = "glaze", pile = p_linen_tone, angle = 0.3, coverage = 1.4, edge = 0.3})

-- 2. Sculpt distinct cylindrical flute folds hanging over the table edge
-- Fold A (leftmost flank, falling away into shadow)
local m_fold_a = m_napkin * mask(function(x, y) return smoothstep(185, 140, x) end)
work(m_fold_a, {hand = "body", pile = p_linen_shadow_cool, angle = 1.2, coverage = 1.3, edge = 0.4})

-- Crease hollow between Fold A and Fold B
local hollow1 = ribbon({{170, 560}, {195, 630}, {212, 675}, {205, 735}}, 14) * m_napkin
work(hollow1, {hand = "body", pile = p_linen_shadow_cool, angle = 1.2, coverage = 1.5, edge = 0.4})

-- Deepest crease line inside hollow1
local b_pen = brush{kind = "round", width = 2.0, point = 1, stiffness = 0.6}
b_pen:load(p_linen_shadow_deep, 0.7)
b_pen:stroke({{172, 570}, {196, 635}, {213, 675}, {206, 730}}, {pressure = {0.3, 0.7, 0.3}, orient = "along"})

-- Fold B crest (central lit fold, sweeping gracefully from table top down over front edge)
local crest_b = ribbon({{178, 555}, {208, 620}, {226, 674}, {222, 742}}, 12) * m_napkin
work(crest_b, {hand = "body", pile = p_linen_crest, angle = 1.1, coverage = 1.4, edge = 0.3})

-- Crease hollow between Fold B and Fold C
local hollow2 = ribbon({{220, 580}, {242, 635}, {248, 675}, {242, 735}}, 12) * m_napkin
work(hollow2, {hand = "body", pile = p_linen_shadow_cool, angle = 1.2, coverage = 1.4, edge = 0.4})

-- Fold C (right fold crest near lemon)
local crest_c = ribbon({{230, 585}, {255, 630}, {262, 674}, {258, 735}}, 10) * m_napkin
work(crest_c, {hand = "body", pile = p_linen_crest, angle = 1.1, coverage = 1.3, edge = 0.3})

-- 3. Soft blend across the folds to create natural cloth rolls rather than rigid edges
blend(m_napkin, {angle = 0.3, coverage = 1.2})

-- 4. Re-establish crisp highlights along the turning edge where the cloth crests bend over the table
local b_touch = brush{kind = "filbert", width = 4, stiffness = 0.6}
b_touch:load(p_linen_high, 0.8)
b_touch:touch(226, 674, {pressure = 0.8, angle = 0.2})
b_touch:touch(262, 674, {pressure = 0.7, angle = 0.2})
b_touch:load(p_linen_crest, 0.7)
b_touch:stroke({{150, 550}, {180, 595}, {205, 640}}, {pressure = {0.5, 0.8, 0.4}, orient = "along"})

-- 5. Soften the outer rim of the napkin where it meets the wood
lose(m_napkin, {pile = p_table_top, reach = {4, 6}, load = 0.2, pressure = {0.3, 0.05}})

print("Napkin sculpted with natural drapery folds and tonal harmony.")

--@ chunk 19
-- Unifying the background wall into a deep, atmospheric chiaroscuro backdrop

local handle_ribbon = ribbon({{630, 355}, {675, 365}, {696, 410}, {692, 455}, {668, 485}}, 16)
local handle_hole = poly({
  {636, 372}, {664, 388}, {675, 420}, {664, 462}, {652, 475}, {634, 400}
}, true)

local wall_mask = (rect(0, 0, 1000, 523) - pitcher_full - handle_ribbon) + handle_hole

p_wall_dark = pile{{"raw umber", 6}, {"bone black", 3}, {"yellow ochre", 1}, {"red earth", 1}, medium = 0.15}
p_wall_warm_mid = pile{{"raw umber", 5}, {"yellow ochre", 3}, {"bone black", 2}, {"lead white", 1}, medium = 0.15}
p_wall_left = pile{{"raw umber", 4}, {"yellow ochre", 4}, {"lead white", 2}, {"bone black", 1}, medium = 0.15}

-- 1. Base unifying coat across the entire wall
work(wall_mask, {hand = "broad", pile = p_wall_warm_mid, angle = 0, coverage = 1.8, fill = true, edge = 0.2})

-- 2. Modulate left and right smoothly
local m_wall_l = wall_mask * mask(function(x, y) return smoothstep(600, 100, x) end)
local m_wall_r = wall_mask * mask(function(x, y) return smoothstep(300, 800, x) end)

work(m_wall_l, {hand = "broad", pile = p_wall_left, angle = 0, coverage = 1.3, edge = 0.4})
work(m_wall_r, {hand = "broad", pile = p_wall_dark, angle = 0, coverage = 1.5, edge = 0.4})

-- 3. Soft broad blending horizontally across the entire wall
blend(wall_mask, {angle = 0, coverage = 2.0})

print("Background wall completely unified.")

--@ chunk 20
-- Refining Tabletop and Sculpting Linen Napkin

-- Piles
local p_table_wash = pile{{"yellow ochre", 5}, {"raw umber", 4}, {"lead white", 2}, {"red earth", 1}, medium = 0.25}
local p_edge_light = pile{{"lead white", 6}, {"yellow ochre", 3}, {"raw umber", 1}, medium = 0.1}
local p_linen_tone = pile{{"lead white", 7}, {"yellow ochre", 2}, {"raw umber", 1}, medium = 0.15}
local p_linen_crest = pile{{"lead white", 8}, {"yellow ochre", 2}, medium = 0.08}
local p_linen_high = pile{{"lead white", 9}, {"yellow ochre", 1}, medium = 0.05}
local p_linen_shadow = pile{{"lead white", 5}, {"bone black", 2}, {"raw umber", 2}, {"cobalt blue", 1}, medium = 0.15}
local p_linen_crease = pile{{"raw umber", 5}, {"bone black", 3}, {"cobalt blue", 1}, {"lead white", 1}, medium = 0.1}

-- 1. Redefine clean napkin silhouette
local clean_napkin_pts = {
  {145, 550}, {185, 550}, {235, 568}, {270, 615}, {266, 674},
  {260, 742}, {225, 752}, {205, 736}, {175, 674}, {140, 650}, {135, 600}
}
m_clean_napkin = poly(clean_napkin_pts, true)

-- 2. Clean table top and front face around napkin
local m_table_around_napkin = (rect(0, 520, 320, 155) - m_clean_napkin)
work(m_table_around_napkin, {hand = "body", pile = p_table_top_light, angle = 0, coverage = 1.6, fill = true, edge = 0.1})

local m_front_around_napkin = (rect(0, 675, 300, 125) - m_clean_napkin)
work(m_front_around_napkin, {hand = "body", pile = p_table_front, angle = 0, coverage = 1.6, fill = true, edge = 0.1})

-- Blend table surface horizontally
blend(rect(0, 520, 1000, 155) - m_clean_napkin - ellipse(365, 635, 60, 40) - ellipse(490, 660, 45, 35) - ellipse(745, 660, 20, 15),
  {angle = 0, coverage = 1.5})

-- 3. Sculpt the Napkin
-- Base coat
work(m_clean_napkin, {hand = "body", pile = p_linen_tone, angle = 0.3, coverage = 1.8, fill = true, edge = 0.15})

-- Troughs in cool blue-gray shadow
local trough1 = ribbon({{165, 560}, {190, 620}, {205, 674}, {198, 730}}, 14) * m_clean_napkin
local trough2 = ribbon({{225, 580}, {242, 630}, {248, 674}, {242, 735}}, 10) * m_clean_napkin
work(trough1, {hand = "body", pile = p_linen_shadow, angle = 1.2, coverage = 1.5, edge = 0.3})
work(trough2, {hand = "body", pile = p_linen_shadow, angle = 1.2, coverage = 1.4, edge = 0.3})

-- Crease lines inside troughs
local b_pen = brush{kind = "round", width = 1.8, point = 1, stiffness = 0.6}
b_pen:load(p_linen_crease, 0.7)
b_pen:stroke({{166, 565}, {192, 625}, {206, 674}, {200, 728}}, {pressure = {0.3, 0.6, 0.3}, orient = "along"})

-- Lit fold ridges
local ridge1 = ribbon({{145, 550}, {175, 600}, {190, 674}, {180, 725}}, 14) * m_clean_napkin
local ridge2 = ribbon({{185, 555}, {212, 620}, {228, 674}, {224, 742}}, 12) * m_clean_napkin
local ridge3 = ribbon({{235, 570}, {258, 625}, {264, 674}, {258, 735}}, 10) * m_clean_napkin
work(ridge1, {hand = "body", pile = p_linen_tone, angle = 0.4, coverage = 1.3, edge = 0.3})
work(ridge2, {hand = "body", pile = p_linen_crest, angle = 0.4, coverage = 1.5, edge = 0.3})
work(ridge3, {hand = "body", pile = p_linen_crest, angle = 0.4, coverage = 1.4, edge = 0.3})

-- Soft blend across folds
blend(m_clean_napkin, {angle = 0.3, coverage = 1.2})

-- Clean hem lines along cloth edges
local b_hem = brush{kind = "round", width = 2.4, point = 1, stiffness = 0.6}
b_hem:load(p_linen_crest, 0.8)
b_hem:stroke({{266, 674}, {260, 742}, {225, 752}, {205, 736}, {175, 674}}, {pressure = {0.6, 0.8, 0.5}, orient = "along"})
b_hem:stroke({{175, 674}, {140, 650}, {135, 600}, {145, 550}}, {pressure = {0.5, 0.7, 0.4}, orient = "along"})

-- Sharp impasto highlights on the table edge folds
b_hem:load(p_linen_high, 0.9)
b_hem:touch(228, 674, {pressure = 0.85})
b_hem:touch(264, 674, {pressure = 0.8})

-- Occlusion shadow under hanging drape
local b_dark = brush{kind = "round", width = 3.0, point = 1, stiffness = 0.6}
b_dark:load(p_pot_deep, 0.8)
b_dark:stroke({{262, 745}, {226, 755}, {204, 739}}, {pressure = {0.5, 0.9, 0.4}, orient = "along"})

-- 4. Table Front Chamfer / Highlight Line
local b_edge = brush{kind = "round", width = 1.6, point = 1, stiffness = 0.6}
b_edge:load(p_edge_light, 0.8)
b_edge:stroke({{268, 674}, {310, 674}, {440, 674}}, {pressure = {0.6, 0.8, 0.6}, orient = "along"})
b_edge:load(p_edge_light, 0.7)
b_edge:stroke({{538, 674}, {650, 674}, {720, 674}}, {pressure = {0.6, 0.7, 0.5}, orient = "along"})
b_edge:load(p_edge_light, 0.6)
b_edge:stroke({{766, 674}, {880, 674}, {1000, 674}}, {pressure = {0.5, 0.6, 0.4}, orient = "along"})

print("Napkin and table edge refined.")

--@ chunk 21
-- Unifying the Table Front Apron and Table Top, and Painting the Curling Lemon Peel

-- 1. Unify the table front apron across the entire canvas width
local napkin_drape_mask = poly({
  {175, 674}, {266, 674}, {262, 745}, {225, 755}, {205, 738}, {175, 674}
}, true)

local table_apron_full = rect(0, 675, 1000, 125) - napkin_drape_mask

work(table_apron_full, {hand = "body", pile = p_table_front, angle = 0, coverage = 1.8, fill = true, edge = 0.1})
blend(table_apron_full, {angle = 0, coverage = 2.0})

-- 2. Unify the table top plane
local table_top_full = rect(0, 523, 1000, 151) - m_clean_napkin - ellipse(365, 635, 65, 42) - ellipse(490, 660, 50, 38) - pitcher_full - ellipse(745, 660, 22, 16)
work(table_top_full, {hand = "glaze", pile = p_table_top, angle = 0, coverage = 1.4, edge = 0.2})
blend(table_top_full, {angle = 0, coverage = 1.6})

-- Re-apply cast shadows on tabletop
local pitcher_shadow = poly({
  {580, 625}, {630, 625}, {720, 630}, {760, 635}, {780, 650},
  {760, 665}, {710, 665}, {620, 640}, {570, 630}
}, true) * rect(0, 523, 1000, 151)
work(pitcher_shadow, {hand = "glaze", pile = p_cast_shadow, angle = 0.1, coverage = 1.4, edge = 0.4})

-- 3. Curling Lemon Peel (Zest)
-- A graceful spiraling ribbon curving forward from the lemon half
local p_peel_outer = pile{{"chrome yellow", 7}, {"yellow ochre", 2}, {"lead white", 1}, medium = 0.1}
local p_peel_inner = pile{{"lead white", 7}, {"yellow ochre", 2}, {"raw umber", 1}, medium = 0.1}
local p_peel_high = pile{{"lead white", 9}, {"chrome yellow", 1}, medium = 0.05}

-- Ribbon points of the curling peel
local peel_pts = {
  {448, 665}, {435, 680}, {420, 698}, {412, 715}, {422, 725}, {438, 718}, {442, 705}
}
local b_peel = brush{kind = "round", width = 3.6, point = 1, stiffness = 0.7}

-- Peel cast shadow on the table
local b_dark = brush{kind = "round", width = 3.0, point = 1, stiffness = 0.5}
b_dark:load(p_cast_shadow, 0.7)
b_dark:stroke({{440, 686}, {428, 708}, {422, 726}}, {pressure = {0.3, 0.6, 0.2}, orient = "along"})

-- Inner pale pith side of ribbon (visible on twist)
b_peel:load(p_peel_inner, 0.8)
b_peel:stroke({{448, 665}, {435, 680}, {422, 696}}, {pressure = {0.6, 0.8, 0.5}, orient = "along"})

-- Outer bright yellow skin side of ribbon
b_peel:load(p_peel_outer, 0.8)
b_peel:stroke({{422, 696}, {412, 715}, {422, 725}, {438, 718}, {442, 705}}, {pressure = {0.6, 0.9, 0.7, 0.4}, orient = "along"})

-- Highlighting the sharp edge of the curl
local b_fine = brush{kind = "round", width = 1.4, point = 1, stiffness = 0.8}
b_fine:load(p_peel_high, 0.8)
b_fine:stroke({{412, 712}, {420, 724}, {435, 719}}, {pressure = {0.4, 0.8, 0.3}, orient = "along"})

-- 4. Re-strike the continuous table edge chamfer / highlight
local p_edge_light = pile{{"lead white", 6}, {"yellow ochre", 3}, {"raw umber", 1}, medium = 0.1}
local b_edge = brush{kind = "rigger", width = 1.6, point = 1, stiffness = 0.7}
b_edge:load(p_edge_light, 0.8)
b_edge:stroke({{266, 674}, {410, 674}}, {pressure = {0.7, 0.8}, orient = "along"})
b_edge:load(p_edge_light, 0.8)
b_edge:stroke({{440, 674}, {722, 674}}, {pressure = {0.7, 0.8}, orient = "along"})
b_edge:load(p_edge_light, 0.7)
b_edge:stroke({{766, 674}, {1000, 674}}, {pressure = {0.6, 0.7, 0.4}, orient = "along"})

-- Occlusion shadow under napkin drape hem
b_dark:load(p_pot_deep, 0.8)
b_dark:stroke({{262, 745}, {226, 755}, {204, 739}}, {pressure = {0.5, 0.9, 0.4}, orient = "along"})

print("Apron unified, tabletop blended, and lemon peel spiral added.")

--@ chunk 22
print(wait(24 * 60)); print("Drying table:", drying(500, 600), drying(500, 720))

--@ chunk 23
print(wait(24 * 60)); print("Drying table:", drying(500, 600), drying(500, 720))

--@ chunk 24
-- Definitive painting of the Lemons and Spiral Peel

-- Piles
local p_lem_base = pile{{"yellow ochre", 5}, {"chrome yellow", 4}, {"raw umber", 1}, medium = 0.12}
local p_lem_pure = pile{{"chrome yellow", 7}, {"yellow ochre", 2}, {"lead white", 1}, medium = 0.1}
local p_lem_lit = pile{{"chrome yellow", 5}, {"lead white", 4}, {"yellow ochre", 1}, medium = 0.1}
local p_lem_high = pile{{"lead white", 8}, {"chrome yellow", 2}, medium = 0.05}
local p_lem_green = pile{{"yellow ochre", 4}, {"green earth", 4}, {"chrome yellow", 2}, medium = 0.15}
local p_lem_shad = pile{{"raw umber", 5}, {"yellow ochre", 3}, {"green earth", 2}, medium = 0.15}
local p_lem_refl = pile{{"yellow ochre", 5}, {"red earth", 2}, {"lead white", 2}, medium = 0.15}

local p_pith = pile{{"lead white", 7}, {"yellow ochre", 2}, {"raw umber", 1}, medium = 0.1}
local p_pulp_bg = pile{{"chrome yellow", 5}, {"yellow ochre", 4}, {"raw umber", 1}, medium = 0.15}
local p_pulp_lit = pile{{"chrome yellow", 6}, {"lead white", 3}, {"yellow ochre", 1}, medium = 0.15}
local p_spark = pile{{"lead white", 9}, {"chrome yellow", 1}, medium = 0.05}

local p_cast = pile{{"raw umber", 6}, {"bone black", 3}, {"red earth", 1}, medium = 0.2}
local p_deep = pile{{"bone black", 7}, {"raw umber", 3}, medium = 0.05}

-- -------------------------------------------------------------
-- 1. WHOLE LEMON REFINEMENT
-- -------------------------------------------------------------
local whole_pts = {
  {305, 628}, {312, 615}, {330, 604}, {365, 597}, {395, 604},
  {418, 618}, {425, 638}, {415, 655}, {395, 668}, {365, 674},
  {335, 668}, {315, 652}, {305, 638}
}
local m_lemon_whole = poly(whole_pts, true)

-- Body color coat
work(m_lemon_whole, {hand = "body", pile = p_lem_pure, angle = 0.2, coverage = 1.6, fill = true, edge = 0.15})

-- Lit volume
local m_w_lit = m_lemon_whole * mask(function(x, y)
  local v = (400 - x) * 0.6 + (665 - y) * 0.7
  return smoothstep(20, 70, v)
end)
work(m_w_lit, {hand = "body", pile = p_lem_lit, angle = 0.2, coverage = 1.5, edge = 0.3})

-- Greenish tint near nipple
local m_w_green = m_lemon_whole * ellipse(315, 626, 18, 14)
work(m_w_green, {hand = "glaze", pile = p_lem_green, angle = 0.1, coverage = 1.3, edge = 0.4})

-- Core shadow
local m_w_shadow = m_lemon_whole * mask(function(x, y)
  local v = (x - 340) * 0.5 + (y - 625) * 0.7
  return smoothstep(25, 60, v)
end)
work(m_w_shadow, {hand = "body", pile = p_lem_shad, angle = 0.3, coverage = 1.4, edge = 0.4})

-- Reflected warm light from oak table
local m_w_refl = m_lemon_whole * mask(function(x, y)
  if y < 652 then return 0 end
  return smoothstep(652, 672, y) * smoothstep(320, 360, x) * smoothstep(420, 380, x)
end)
work(m_w_refl, {hand = "glaze", pile = p_lem_refl, angle = 0, coverage = 1.2, edge = 0.4})

-- Blend whole lemon volume
blend(m_lemon_whole, {angle = 0.2, coverage = 1.3})

-- Peel dimple stippling
stipple(m_w_lit, {pile = p_lem_pure, width = 1.8, coverage = 0.9, feather = 0.3})
stipple(m_w_lit * ellipse(350, 620, 30, 20), {pile = p_lem_lit, width = 1.6, coverage = 0.8, feather = 0.4})

-- Impasto highlight
local b_high = brush{kind = "round", width = 3.5, point = 1, stiffness = 0.7}
b_high:load(p_lem_high, 0.8)
b_high:stroke({{342, 615}, {355, 616}, {368, 620}}, {pressure = {0.7, 0.9, 0.3}, orient = "along"})
b_high:load(p_lem_high, 0.6)
b_high:touch(309, 626, {pressure = 0.6})

-- Stem calyx dot on right
local b_calyx = brush{kind = "round", width = 2.0, point = 1, stiffness = 0.6}
b_calyx:load(p_lem_shad, 0.8)
b_calyx:touch(423, 638, {pressure = 0.7})

-- Contact occlusion under whole lemon
local b_dark = brush{kind = "round", width = 2.5, point = 1, stiffness = 0.6}
b_dark:load(p_deep, 0.8)
b_dark:stroke({{335, 673}, {365, 675}, {395, 673}}, {pressure = {0.5, 0.9, 0.3}, orient = "along"})


-- -------------------------------------------------------------
-- 2. CUT LEMON HALF (tilted, resting on table)
-- -------------------------------------------------------------
local cx, cy = 485, 652
local rx, ry = 42, 34

-- Whole cut lemon body silhouette
local m_cut_body = ellipse(cx, cy, rx + 2, ry + 2)

-- Outer rind
work(m_cut_body, {hand = "body", pile = p_lem_pure, angle = 0.1, coverage = 1.8, fill = true, edge = 0.1})

-- Pale pith ring
local m_pith_ring = ellipse(cx, cy, rx, ry) - ellipse(cx, cy, rx - 6, ry - 5)
work(m_pith_ring, {hand = "detail", pile = p_pith, angle = 0.1, coverage = 2.0, clip = true})

-- Translucent pulp bed
local m_pulp = ellipse(cx, cy, rx - 6, ry - 5)
work(m_pulp, {hand = "body", pile = p_pulp_bg, angle = 0.2, coverage = 1.8, fill = true, edge = 0.2})
work(m_pulp, {hand = "glaze", pile = p_pulp_lit, angle = 0, coverage = 1.3, edge = 0.3})

-- Central pith hub
local b_pith = brush{kind = "round", width = 2.0, point = 1, stiffness = 0.6}
b_pith:load(p_pith, 0.8)
b_pith:touch(cx, cy, {pressure = 0.85})

-- 8 radial membranes
local n_segs = 8
for i = 1, n_segs do
  local ang = (i - 1) * (2 * math.pi / n_segs) + 0.12
  local x_end = cx + (rx - 6) * math.cos(ang)
  local y_end = cy + (ry - 5) * math.sin(ang)
  b_pith:load(p_pith, 0.7)
  b_pith:stroke({{cx, cy}, {cx + (x_end - cx) * 0.5, cy + (y_end - cy) * 0.5}, {x_end, y_end}},
    {pressure = {0.8, 0.5, 0.3}, orient = "along"})
end

-- Wet glistening sparkles on pulp juice sacs
local b_spark = brush{kind = "round", width = 1.5, point = 1, stiffness = 0.8}
for i = 1, n_segs do
  local ang1 = (i - 1) * (2 * math.pi / n_segs) + 0.12
  local ang2 = i * (2 * math.pi / n_segs) + 0.12
  local mid_ang = (ang1 + ang2) / 2
  local r_mid = (rx - 12) * 0.65
  local px = cx + r_mid * math.cos(mid_ang)
  local py = cy + (r_mid * (ry / rx)) * math.sin(mid_ang)
  
  b_spark:load(p_spark, 0.8)
  b_spark:touch(px, py, {pressure = 0.75})
  b_spark:touch(px + 2.5 * math.cos(mid_ang + 1.2), py + 2 * math.sin(mid_ang + 1.2), {pressure = 0.55})
end

-- Lit top rim of the cut rind
local m_cut_rim_lit = ribbon({{452, 634}, {485, 622}, {518, 630}}, 3.5)
work(m_cut_rim_lit, {hand = "detail", pile = p_lem_lit, angle = 0, coverage = 1.6})

-- Contact shadow under cut lemon
b_dark:load(p_deep, 0.8)
b_dark:stroke({{460, 674}, {485, 676}, {515, 674}}, {pressure = {0.5, 0.9, 0.3}, orient = "along"})

-- Cast shadow to the right of cut lemon
local m_cut_cast = ellipse(535, 674, 25, 8)
work(m_cut_cast, {hand = "glaze", pile = p_cast, angle = 0.1, coverage = 1.4, edge = 0.4})


-- -------------------------------------------------------------
-- 3. CURLING LEMON PEEL (ZEST SPIRAL)
-- -------------------------------------------------------------
-- Starts from cut lemon left rim, curls forward and drops gracefully
local peel_b = brush{kind = "round", width = 3.4, point = 1, stiffness = 0.7}

-- Cast shadow of peel on table apron
b_dark:load(p_deep, 0.6)
b_dark:stroke({{440, 688}, {426, 708}, {420, 725}}, {pressure = {0.2, 0.5, 0.2}, orient = "along"})

-- Inner pale pith ribbon
peel_b:load(p_pith, 0.8)
peel_b:stroke({{450, 662}, {438, 678}, {425, 696}}, {pressure = {0.6, 0.8, 0.5}, orient = "along"})

-- Outer vibrant yellow peel ribbon
peel_b:load(p_lem_pure, 0.85)
peel_b:stroke({{425, 696}, {416, 715}, {425, 726}, {438, 720}, {444, 706}}, {pressure = {0.6, 0.9, 0.7, 0.4}, orient = "along"})

-- Crisp highlight along the sharp edge of the peel curl
local b_curl_edge = brush{kind = "round", width = 1.4, point = 1, stiffness = 0.8}
b_curl_edge:load(p_spark, 0.8)
b_curl_edge:stroke({{416, 712}, {424, 725}, {436, 720}}, {pressure = {0.4, 0.8, 0.3}, orient = "along"})

print("Lemons and spiral peel definitively rendered.")

--@ chunk 25
-- Painting the Walnut, Refined Linen Napkin, and Touch-up of Cut Lemon Pulp

-- Common piles
local p_pith = pile{{"lead white", 7}, {"yellow ochre", 2}, {"raw umber", 1}, medium = 0.1}
local p_pulp_bg = pile{{"chrome yellow", 5}, {"yellow ochre", 4}, {"raw umber", 1}, medium = 0.15}
local p_pulp_lit = pile{{"chrome yellow", 6}, {"lead white", 3}, {"yellow ochre", 1}, medium = 0.15}
local p_spark = pile{{"lead white", 9}, {"chrome yellow", 1}, medium = 0.05}

local p_nut_base = pile{{"yellow ochre", 5}, {"raw umber", 4}, {"red earth", 2}, medium = 0.12}
local p_nut_dark = pile{{"raw umber", 6}, {"bone black", 3}, {"red earth", 1}, medium = 0.1}
local p_nut_lit = pile{{"yellow ochre", 5}, {"lead white", 4}, {"raw umber", 2}, medium = 0.1}
local p_nut_high = pile{{"lead white", 7}, {"yellow ochre", 3}, medium = 0.05}

local p_linen_lit = pile{{"lead white", 8}, {"yellow ochre", 1}, {"raw umber", 1}, medium = 0.1}
local p_linen_crest = pile{{"lead white", 8}, {"yellow ochre", 2}, medium = 0.08}
local p_linen_high = pile{{"lead white", 9}, {"yellow ochre", 1}, medium = 0.05}
local p_linen_shadow = pile{{"lead white", 5}, {"bone black", 2}, {"raw umber", 2}, {"cobalt blue", 1}, medium = 0.15}
local p_linen_crease = pile{{"raw umber", 5}, {"bone black", 3}, {"cobalt blue", 1}, {"lead white", 1}, medium = 0.1}

local p_cast = pile{{"raw umber", 6}, {"bone black", 3}, {"red earth", 1}, medium = 0.2}
local p_deep = pile{{"bone black", 7}, {"raw umber", 3}, medium = 0.05}

-- -------------------------------------------------------------
-- 1. TOUCH-UP CUT LEMON PULP
-- -------------------------------------------------------------
local cx, cy = 485, 652
local rx, ry = 42, 34

local m_pulp = ellipse(cx, cy, rx - 6, ry - 5)
work(m_pulp, {hand = "body", pile = p_pulp_bg, angle = 0.2, coverage = 2.0, fill = true, edge = 0.15})
work(m_pulp, {hand = "glaze", pile = p_pulp_lit, angle = 0, coverage = 1.4, edge = 0.2})

-- Central pith hub
local b_pith = brush{kind = "round", width = 2.0, point = 1, stiffness = 0.6}
b_pith:load(p_pith, 0.85)
b_pith:touch(cx, cy, {pressure = 0.9})

-- 8 radial membranes
local n_segs = 8
for i = 1, n_segs do
  local ang = (i - 1) * (2 * math.pi / n_segs) + 0.12
  local x_end = cx + (rx - 6) * math.cos(ang)
  local y_end = cy + (ry - 5) * math.sin(ang)
  b_pith:load(p_pith, 0.75)
  b_pith:stroke({{cx, cy}, {cx + (x_end - cx) * 0.5, cy + (y_end - cy) * 0.5}, {x_end, y_end}},
    {pressure = {0.8, 0.5, 0.3}, orient = "along"})
end

-- Sparkling highlights on juice vesicles
local b_spark = brush{kind = "round", width = 1.4, point = 1, stiffness = 0.8}
for i = 1, n_segs do
  local ang1 = (i - 1) * (2 * math.pi / n_segs) + 0.12
  local ang2 = i * (2 * math.pi / n_segs) + 0.12
  local mid_ang = (ang1 + ang2) / 2
  local r_mid = (rx - 12) * 0.65
  local px = cx + r_mid * math.cos(mid_ang)
  local py = cy + (r_mid * (ry / rx)) * math.sin(mid_ang)
  
  b_spark:load(p_spark, 0.85)
  b_spark:touch(px, py, {pressure = 0.8})
  b_spark:touch(px + 2.5 * math.cos(mid_ang + 1.2), py + 2 * math.sin(mid_ang + 1.2), {pressure = 0.6})
end


-- -------------------------------------------------------------
-- 2. WALNUT ON THE RIGHT
-- -------------------------------------------------------------
local wx, wy = 745, 658
local m_walnut = ellipse(wx, wy, 18, 14)

-- Body coat
work(m_walnut, {hand = "body", pile = p_nut_base, angle = 0.2, coverage = 1.8, fill = true, edge = 0.15})

-- Shadow side (right half)
local m_nut_sh = m_walnut * mask(function(x, y) return (x > wx - 2 or y > wy + 2) and 1 or 0 end)
work(m_nut_sh, {hand = "body", pile = p_nut_dark, angle = 0.3, coverage = 1.5, edge = 0.3})

-- Lit side (upper-left half)
local m_nut_lt = m_walnut * mask(function(x, y) return (x <= wx - 2 and y <= wy + 2) and 1 or 0 end)
work(m_nut_lt, {hand = "body", pile = p_nut_lit, angle = 0.1, coverage = 1.6, edge = 0.3})

-- Seam line down center
local b_line = brush{kind = "round", width = 1.8, point = 1, stiffness = 0.6}
b_line:load(p_nut_dark, 0.8)
b_line:stroke({{wx, wy - 13}, {wx + 1, wy}, {wx, wy + 13}}, {pressure = {0.3, 0.8, 0.4}, orient = "along"})

-- Rugged shell texture (stippling)
stipple(m_walnut, {pile = p_nut_dark, width = 1.4, coverage = 0.8, feather = 0.2})
stipple(m_nut_lt, {pile = p_nut_high, width = 1.2, coverage = 0.7, feather = 0.3})

-- Crisp shell ridge highlight
b_line:load(p_nut_high, 0.8)
b_line:stroke({{wx - 8, wy - 6}, {wx - 3, wy - 8}, {wx - 1, wy - 5}}, {pressure = {0.4, 0.8, 0.3}, orient = "along"})

-- Contact occlusion under walnut
local b_dark = brush{kind = "round", width = 2.5, point = 1, stiffness = 0.6}
b_dark:load(p_deep, 0.8)
b_dark:stroke({{wx - 15, wy + 11}, {wx, wy + 13}, {wx + 16, wy + 10}}, {pressure = {0.4, 0.9, 0.3}, orient = "along"})

-- Cast shadow to the right of walnut
local m_nut_cast = ellipse(wx + 22, wy + 8, 16, 7)
work(m_nut_cast, {hand = "glaze", pile = p_cast, angle = 0.1, coverage = 1.4, edge = 0.4})


-- -------------------------------------------------------------
-- 3. REFINED LINEN NAPKIN
-- -------------------------------------------------------------
local clean_napkin_pts = {
  {145, 550}, {185, 548}, {235, 565}, {270, 615}, {266, 674},
  {260, 742}, {225, 752}, {205, 736}, {175, 674}, {145, 645}, {138, 595}
}
local m_napkin_clean = poly(clean_napkin_pts, true)

-- Body coat of creamy linen
work(m_napkin_clean, {hand = "body", pile = p_linen_lit, angle = 0.3, coverage = 1.8, fill = true, edge = 0.12})

-- Cool blue-gray shadows in the troughs
local trough1 = ribbon({{165, 558}, {190, 620}, {205, 674}, {198, 730}}, 14) * m_napkin_clean
local trough2 = ribbon({{225, 578}, {242, 630}, {248, 674}, {242, 735}}, 10) * m_napkin_clean
work(trough1, {hand = "body", pile = p_linen_shadow, angle = 1.2, coverage = 1.5, edge = 0.3})
work(trough2, {hand = "body", pile = p_linen_shadow, angle = 1.2, coverage = 1.4, edge = 0.3})

-- Crease lines inside troughs
b_line:load(p_linen_crease, 0.7)
b_line:stroke({{166, 562}, {192, 625}, {206, 674}, {200, 728}}, {pressure = {0.3, 0.6, 0.3}, orient = "along"})

-- Lit fold ridges
local ridge1 = ribbon({{145, 550}, {175, 600}, {190, 674}, {180, 725}}, 14) * m_napkin_clean
local ridge2 = ribbon({{185, 552}, {212, 620}, {228, 674}, {224, 742}}, 12) * m_napkin_clean
local ridge3 = ribbon({{235, 568}, {258, 625}, {264, 674}, {258, 735}}, 10) * m_napkin_clean
work(ridge1, {hand = "body", pile = p_linen_lit, angle = 0.4, coverage = 1.3, edge = 0.3})
work(ridge2, {hand = "body", pile = p_linen_crest, angle = 0.4, coverage = 1.5, edge = 0.3})
work(ridge3, {hand = "body", pile = p_linen_crest, angle = 0.4, coverage = 1.4, edge = 0.3})

-- Soft blend across folds
blend(m_napkin_clean, {angle = 0.3, coverage = 1.2})

-- Clean hem lines along cloth edges
local b_hem = brush{kind = "round", width = 2.2, point = 1, stiffness = 0.6}
b_hem:load(p_linen_crest, 0.8)
b_hem:stroke({{266, 674}, {260, 742}, {225, 752}, {205, 736}, {175, 674}}, {pressure = {0.6, 0.8, 0.5}, orient = "along"})
b_hem:stroke({{175, 674}, {145, 645}, {138, 595}, {145, 550}}, {pressure = {0.5, 0.7, 0.4}, orient = "along"})

-- Sharp impasto highlights on the table edge folds
b_hem:load(p_linen_high, 0.9)
b_hem:touch(228, 674, {pressure = 0.85})
b_hem:touch(264, 674, {pressure = 0.8})

-- Occlusion shadow under hanging drape
b_dark:load(p_deep, 0.8)
b_dark:stroke({{262, 745}, {226, 755}, {204, 739}}, {pressure = {0.5, 0.9, 0.4}, orient = "along"})

print("Walnut, napkin, and lemon pulp completed.")

--@ chunk 26
-- Definitive Modeling of the Glazed Earthenware Pitcher

-- Pitcher Piles
local p_pot_terra = pile{{"red earth", 6}, {"yellow ochre", 3}, {"raw umber", 2}, medium = 0.12}
local p_pot_shadow = pile{{"raw umber", 6}, {"bone black", 3}, {"red earth", 2}, medium = 0.08}
local p_pot_throat = pile{{"bone black", 7}, {"raw umber", 3}, medium = 0.05}
local p_pot_olive = pile{{"yellow ochre", 5}, {"green earth", 3}, {"raw umber", 2}, {"lead white", 1}, medium = 0.15}
local p_pot_lit = pile{{"yellow ochre", 5}, {"red earth", 3}, {"lead white", 3}, medium = 0.12}
local p_pot_refl = pile{{"yellow ochre", 4}, {"red earth", 3}, {"raw umber", 2}, {"lead white", 1}, medium = 0.15}
local p_pot_spec = pile{{"lead white", 8}, {"yellow ochre", 2}, medium = 0.05}
local p_pot_contact = pile{{"bone black", 7}, {"raw umber", 3}, medium = 0.05}

-- Geometry
local body_pts = {
  {545, 310}, {590, 305}, {635, 310}, {642, 322}, {630, 370},
  {665, 415}, {680, 475}, {672, 545}, {645, 605}, {630, 626},
  {590, 631}, {550, 626}, {535, 605}, {508, 545}, {500, 475},
  {515, 415}, {550, 370}, {538, 322}
}
local jug_body = poly(body_pts, true)

local handle_pts = {
  {630, 355}, {675, 365}, {696, 410}, {692, 455}, {668, 485}
}
local handle_ribbon = ribbon(handle_pts, 16)
local handle_hole = poly({
  {636, 372}, {664, 388}, {675, 420}, {664, 462}, {652, 475}, {634, 400}
}, true)

local jug_all = (jug_body + handle_ribbon) - handle_hole
local jug_interior = ellipse(590, 313, 38, 9)

-- 1. Base terracotta coat over the jug body and handle
work(jug_all, {hand = "body", pile = p_pot_terra, angle = 0.1, coverage = 1.8, fill = true, edge = 0.15})

-- 2. Deep interior shadow inside mouth
work(jug_interior, {hand = "detail", pile = p_pot_throat, angle = 0, coverage = 2.0, clip = true})

-- 3. Core shadow on right side of belly and neck
local jug_sh = jug_all * mask(function(x, y)
  local t = (x - 575) / 75
  return clamp(t, 0, 1)
end) - jug_interior
work(jug_sh, {hand = "body", pile = p_pot_shadow, angle = 0.15, coverage = 1.6, edge = 0.35})

-- Cast shadow under handle on right belly
local handle_cast = ellipse(655, 430, 18, 35) * jug_all
work(handle_cast, {hand = "body", pile = p_pot_shadow, angle = 0.2, coverage = 1.4, edge = 0.35})

-- 4. Warm reflected light along right contour
local jug_rf = jug_all * mask(function(x, y)
  if x < 645 then return 0 end
  return smoothstep(645, 675, x) * smoothstep(360, 420, y) * smoothstep(625, 550, y) * 0.7
end)
work(jug_rf, {hand = "glaze", pile = p_pot_refl, angle = 0.3, coverage = 1.3, edge = 0.35})

-- 5. Lit area on left shoulder, belly, and neck
local jug_lt = jug_all * mask(function(x, y)
  local t = (600 - x) / 95
  return clamp(t, 0, 1)
end) - jug_interior
work(jug_lt, {hand = "body", pile = p_pot_lit, angle = 0.1, coverage = 1.6, edge = 0.25})

-- Olive glaze wash across the shoulder/belly transition
local jug_ol = jug_lt * ellipse(560, 460, 50, 70)
work(jug_ol, {hand = "glaze", pile = p_pot_olive, angle = 0.1, coverage = 1.3, edge = 0.4})

-- 6. Rim and handle lighting
-- Lit lip on left
local rim_lit = ribbon({{545, 318}, {585, 323}}, 5)
work(rim_lit, {hand = "detail", pile = p_pot_lit, angle = 0.1, coverage = 1.8})

-- Handle crest lit
local handle_lt = handle_ribbon * mask(function(x, y) return (y < 415 and x > 640) and 1 or 0 end) - handle_hole
work(handle_lt, {hand = "detail", pile = p_pot_lit, angle = -0.5, coverage = 1.5})

-- 7. Soft blend across the belly to fuse ceramic glaze wet-into-wet
blend(jug_all - jug_interior, {angle = 0.1, coverage = 1.3})

-- 8. Specular impasto highlights on the glaze
local b_point = brush{kind = "round", width = 3.2, point = 1, stiffness = 0.7}
b_point:load(p_pot_spec, 0.85)
-- Shoulder specular streak
b_point:stroke({{538, 432}, {542, 452}, {544, 468}}, {pressure = {0.75, 0.95, 0.3}, orient = "along"})
-- Rim specular touch
b_point:load(p_pot_spec, 0.8)
b_point:stroke({{550, 316}, {570, 321}, {582, 322}}, {pressure = {0.6, 0.85, 0.2}, orient = "along"})
-- Handle apex touch
b_point:load(p_pot_spec, 0.6)
b_point:touch(686, 395, {pressure = 0.7})

-- 9. Base contact occlusion shadow and near-cast shadow
b_point:load(p_pot_contact, 0.85)
b_point:stroke({{548, 627}, {590, 632}, {632, 627}}, {pressure = {0.5, 0.9, 0.4}, orient = "along"})

local jug_cast = poly({
  {580, 625}, {630, 625}, {710, 632}, {760, 638}, {780, 650},
  {760, 665}, {700, 665}, {620, 640}, {570, 630}
}, true)
work(jug_cast, {hand = "glaze", pile = p_cast_shadow, angle = 0.1, coverage = 1.5, edge = 0.4})

print("Glazed earthenware jug fully rendered.")

--@ chunk 27
-- Refining the Linen Napkin and Painting the Sunlit Walnut on the Right

-- Piles
local p_table_clean = pile{{"yellow ochre", 5}, {"raw umber", 4}, {"lead white", 2}, {"red earth", 1}, medium = 0.15}
local p_apron_clean = pile{{"raw umber", 6}, {"bone black", 3}, {"red earth", 2}, medium = 0.12}

local p_lin_lit = pile{{"lead white", 8}, {"yellow ochre", 1}, {"raw umber", 1}, medium = 0.1}
local p_lin_crest = pile{{"lead white", 8}, {"yellow ochre", 2}, medium = 0.08}
local p_lin_high = pile{{"lead white", 9}, {"yellow ochre", 1}, medium = 0.05}
local p_lin_sh = pile{{"lead white", 5}, {"bone black", 2}, {"raw umber", 2}, {"cobalt blue", 1}, medium = 0.15}
local p_lin_deep = pile{{"raw umber", 5}, {"bone black", 3}, {"cobalt blue", 1}, {"lead white", 1}, medium = 0.1}

local p_nut_base = pile{{"yellow ochre", 5}, {"raw umber", 4}, {"red earth", 2}, medium = 0.12}
local p_nut_dark = pile{{"raw umber", 6}, {"bone black", 3}, {"red earth", 1}, medium = 0.1}
local p_nut_lit = pile{{"yellow ochre", 5}, {"lead white", 4}, {"raw umber", 2}, medium = 0.1}
local p_nut_high = pile{{"lead white", 7}, {"yellow ochre", 3}, medium = 0.05}
local p_cast = pile{{"raw umber", 6}, {"bone black", 3}, {"red earth", 1}, medium = 0.2}
local p_deep = pile{{"bone black", 7}, {"raw umber", 3}, medium = 0.05}

-- -------------------------------------------------------------
-- 1. TAILORED LINEN NAPKIN
-- -------------------------------------------------------------
-- Define crisp, folded geometry
local nap_pts = {
  {135, 545}, {245, 548}, {265, 605}, {262, 674},
  {258, 742}, {218, 745}, {212, 742}, {165, 740}, {165, 674},
  {135, 630}, {130, 580}
}
local m_nap = poly(nap_pts, true)

-- Clean up table top and front apron around napkin
local m_cut_table = rect(100, 520, 200, 155) - m_nap
work(m_cut_table, {hand = "body", pile = p_table_clean, angle = 0, coverage = 1.8, fill = true, edge = 0.1})

local m_cut_apron = rect(100, 675, 200, 125) - m_nap
work(m_cut_apron, {hand = "body", pile = p_apron_clean, angle = 0, coverage = 1.8, fill = true, edge = 0.1})

-- Base coat of creamy linen
work(m_nap, {hand = "body", pile = p_lin_lit, angle = 0.3, coverage = 1.8, fill = true, edge = 0.1})

-- Sculpt two distinct vertical flute folds hanging over table front
-- Trough between Fold 1 and Fold 2
local trough = ribbon({{185, 555}, {205, 620}, {214, 674}, {210, 742}}, 12) * m_nap
work(trough, {hand = "body", pile = p_lin_sh, angle = 1.2, coverage = 1.5, edge = 0.3})

-- Left flank shadow (falling away toward left)
local left_sh = m_nap * mask(function(x, y) return smoothstep(165, 132, x) end)
work(left_sh, {hand = "body", pile = p_lin_sh, angle = 1.1, coverage = 1.4, edge = 0.3})

-- Fold 1 crest (left hanging fold)
local fold1 = ribbon({{155, 548}, {180, 610}, {188, 674}, {185, 740}}, 14) * m_nap
work(fold1, {hand = "body", pile = p_lin_crest, angle = 0.5, coverage = 1.5, edge = 0.3})

-- Fold 2 crest (right hanging fold)
local fold2 = ribbon({{215, 555}, {235, 615}, {242, 674}, {238, 742}}, 14) * m_nap
work(fold2, {hand = "body", pile = p_lin_crest, angle = 0.5, coverage = 1.5, edge = 0.3})

-- Blend across folds gently
blend(m_nap, {angle = 0.4, coverage = 1.2})

-- Crease lines with pointed brush
local b_pen = brush{kind = "round", width = 1.8, point = 1, stiffness = 0.6}
b_pen:load(p_lin_deep, 0.7)
b_pen:stroke({{188, 560}, {206, 625}, {215, 674}, {211, 740}}, {pressure = {0.3, 0.6, 0.3}, orient = "along"})

-- Sharp impasto highlights on the table edge folds
local b_touch = brush{kind = "filbert", width = 3.5, stiffness = 0.7}
b_touch:load(p_lin_high, 0.9)
b_touch:touch(188, 674, {pressure = 0.85})
b_touch:touch(242, 674, {pressure = 0.85})

-- Clean hem lines along bottom drape
local b_line = brush{kind = "round", width = 2.0, point = 1, stiffness = 0.6}
b_line:load(p_lin_crest, 0.8)
b_line:stroke({{165, 740}, {212, 742}, {218, 745}, {258, 742}}, {pressure = {0.6, 0.8, 0.5}, orient = "along"})

-- Occlusion shadow under hanging drape
local b_dark = brush{kind = "round", width = 2.8, point = 1, stiffness = 0.6}
b_dark:load(p_deep, 0.85)
b_dark:stroke({{164, 742}, {212, 745}, {218, 748}, {260, 744}}, {pressure = {0.5, 0.9, 0.4}, orient = "along"})


-- -------------------------------------------------------------
-- 2. SUNLIT WALNUT ON THE RIGHT (x = 790, y = 656)
-- -------------------------------------------------------------
local wx, wy = 790, 656
local m_walnut = ellipse(wx, wy, 16, 12)

-- Body coat
work(m_walnut, {hand = "body", pile = p_nut_base, angle = 0.2, coverage = 1.8, fill = true, edge = 0.12})

-- Shadow side (right and lower side)
local m_nut_sh = m_walnut * mask(function(x, y) return (x > wx - 1 or y > wy + 2) and 1 or 0 end)
work(m_nut_sh, {hand = "body", pile = p_nut_dark, angle = 0.3, coverage = 1.5, edge = 0.3})

-- Lit side (upper-left half)
local m_nut_lt = m_walnut * mask(function(x, y) return (x <= wx - 1 and y <= wy + 2) and 1 or 0 end)
work(m_nut_lt, {hand = "body", pile = p_nut_lit, angle = 0.1, coverage = 1.6, edge = 0.3})

-- Suture / seam line down center
b_line:load(p_nut_dark, 0.8)
b_line:stroke({{wx, wy - 11}, {wx + 1, wy}, {wx, wy + 11}}, {pressure = {0.3, 0.7, 0.4}, orient = "along"})

-- Rugged shell texture (stippling)
stipple(m_walnut, {pile = p_nut_dark, width = 1.3, coverage = 0.8, feather = 0.2})
stipple(m_nut_lt, {pile = p_nut_high, width = 1.1, coverage = 0.7, feather = 0.3})

-- Sharp crest highlight
b_line:load(p_nut_high, 0.85)
b_line:stroke({{wx - 7, wy - 5}, {wx - 3, wy - 7}, {wx - 1, wy - 4}}, {pressure = {0.4, 0.8, 0.2}, orient = "along"})

-- Contact occlusion under walnut
b_dark:load(p_deep, 0.85)
b_dark:stroke({{wx - 14, wy + 9}, {wx, wy + 11}, {wx + 14, wy + 8}}, {pressure = {0.4, 0.8, 0.3}, orient = "along"})

-- Cast shadow extending softly to the right
local m_nut_cast = ellipse(wx + 20, wy + 7, 15, 6)
work(m_nut_cast, {hand = "glaze", pile = p_cast, angle = 0.1, coverage = 1.4, edge = 0.4})

-- Clean up table edge highlight line in this region
local p_edge_light = pile{{"lead white", 6}, {"yellow ochre", 3}, {"raw umber", 1}, medium = 0.1}
local b_edge = brush{kind = "rigger", width = 1.5, point = 1, stiffness = 0.7}
b_edge:load(p_edge_light, 0.7)
b_edge:stroke({{wx + 25, 674}, {1000, 674}}, {pressure = {0.6, 0.7, 0.4}, orient = "along"})

print("Napkin and sunlit walnut completed.")

--@ chunk 28
-- Unifying the Entire Tabletop to Eliminate Any Seams

-- Piles
local p_table_top = pile{{"yellow ochre", 5}, {"raw umber", 4}, {"lead white", 2}, {"red earth", 1}, medium = 0.2}
local p_table_left = pile{{"yellow ochre", 5}, {"lead white", 4}, {"raw umber", 2}, {"red earth", 1}, medium = 0.2}
local p_cast = pile{{"raw umber", 6}, {"bone black", 3}, {"red earth", 1}, medium = 0.25}

-- Object masks to preserve
local m_nap = poly({
  {135, 545}, {245, 548}, {265, 605}, {262, 674},
  {258, 742}, {218, 745}, {212, 742}, {165, 740}, {165, 674},
  {135, 630}, {130, 580}
}, true)

local m_whole = poly({
  {305, 628}, {312, 615}, {330, 604}, {365, 597}, {395, 604},
  {418, 618}, {425, 638}, {415, 655}, {395, 668}, {365, 674},
  {335, 668}, {315, 652}, {305, 638}
}, true)

local m_cut = ellipse(485, 652, 44, 35)
local m_walnut = ellipse(790, 656, 17, 13)

local m_objects = m_nap + m_whole + m_cut + pitcher_full + m_walnut

-- Full tabletop plane (y from 523 to 674, x from 0 to 1000)
local table_top_full = rect(0, 523, 1000, 151) - m_objects

-- 1. Unify table surface across the entire canvas width
work(table_top_full, {hand = "body", pile = p_table_top, angle = 0, coverage = 1.6, fill = true, edge = 0.1})

-- Modulate light on the left
local m_left_top = table_top_full * mask(function(x, y) return smoothstep(500, 100, x) end)
work(m_left_top, {hand = "glaze", pile = p_table_left, angle = 0, coverage = 1.2, edge = 0.3})

-- Horizontal blending pass across the full tabletop
blend(table_top_full, {angle = 0, coverage = 1.8})

-- 2. Cast shadows on the tabletop
-- Pitcher cast shadow
local pitcher_shadow = poly({
  {580, 625}, {630, 625}, {710, 632}, {760, 638}, {780, 650},
  {760, 665}, {700, 665}, {620, 640}, {570, 630}
}, true) * rect(0, 523, 1000, 151)
work(pitcher_shadow, {hand = "glaze", pile = p_cast, angle = 0.1, coverage = 1.4, edge = 0.4})

-- Whole lemon cast shadow
local lemon_shadow = poly({
  {360, 665}, {410, 665}, {435, 670}, {425, 674}, {385, 674}, {350, 673}
}, true) * rect(0, 523, 1000, 151)
work(lemon_shadow, {hand = "glaze", pile = p_cast, angle = 0.1, coverage = 1.3, edge = 0.4})

-- Walnut cast shadow
local walnut_shadow = ellipse(810, 663, 15, 6) * rect(0, 523, 1000, 151)
work(walnut_shadow, {hand = "glaze", pile = p_cast, angle = 0.1, coverage = 1.3, edge = 0.4})

-- 3. Re-strike continuous table edge chamfer / highlight
local p_edge_light = pile{{"lead white", 6}, {"yellow ochre", 3}, {"raw umber", 1}, medium = 0.1}
local b_edge = brush{kind = "rigger", width = 1.6, point = 1, stiffness = 0.7}
b_edge:load(p_edge_light, 0.8)
b_edge:stroke({{262, 674}, {310, 674}}, {pressure = {0.7, 0.8}, orient = "along"})
b_edge:load(p_edge_light, 0.8)
b_edge:stroke({{425, 674}, {442, 674}}, {pressure = {0.7, 0.8}, orient = "along"})
b_edge:load(p_edge_light, 0.8)
b_edge:stroke({{530, 674}, {772, 674}}, {pressure = {0.7, 0.8, 0.6}, orient = "along"})
b_edge:load(p_edge_light, 0.7)
b_edge:stroke({{807, 674}, {1000, 674}}, {pressure = {0.6, 0.7, 0.4}, orient = "along"})

print("Full tabletop unified seamlessly.")

--@ chunk 29
-- Final Sculptural Refinement of Jug, Lemon Peel, and Napkin

-- Piles
local p_pot_terra = pile{{"red earth", 6}, {"yellow ochre", 3}, {"raw umber", 2}, medium = 0.15}
local p_pot_shadow = pile{{"raw umber", 6}, {"bone black", 3}, {"red earth", 2}, medium = 0.1}
local p_pot_lit = pile{{"yellow ochre", 5}, {"red earth", 3}, {"lead white", 3}, medium = 0.12}
local p_pot_refl = pile{{"yellow ochre", 4}, {"red earth", 3}, {"raw umber", 2}, {"lead white", 1}, medium = 0.15}
local p_pot_spec = pile{{"lead white", 8}, {"yellow ochre", 2}, medium = 0.05}
local p_pot_throat = pile{{"bone black", 7}, {"raw umber", 3}, medium = 0.05}
local p_deep = pile{{"bone black", 7}, {"raw umber", 3}, medium = 0.05}

local p_lem_pure = pile{{"chrome yellow", 7}, {"yellow ochre", 2}, {"lead white", 1}, medium = 0.1}
local p_pith = pile{{"lead white", 7}, {"yellow ochre", 2}, {"raw umber", 1}, medium = 0.1}
local p_spark = pile{{"lead white", 9}, {"chrome yellow", 1}, medium = 0.05}

local p_lin_lit = pile{{"lead white", 8}, {"yellow ochre", 1}, {"raw umber", 1}, medium = 0.1}
local p_lin_crest = pile{{"lead white", 8}, {"yellow ochre", 2}, medium = 0.08}
local p_lin_high = pile{{"lead white", 9}, {"yellow ochre", 1}, medium = 0.05}
local p_lin_sh = pile{{"lead white", 5}, {"bone black", 2}, {"raw umber", 2}, {"cobalt blue", 1}, medium = 0.15}

-- -------------------------------------------------------------
-- 1. EARTHENWARE JUG HARMONIZATION
-- -------------------------------------------------------------
local body_pts = {
  {545, 310}, {590, 305}, {635, 310}, {642, 322}, {630, 370},
  {665, 415}, {680, 475}, {672, 545}, {645, 605}, {630, 626},
  {590, 631}, {550, 626}, {535, 605}, {508, 545}, {500, 475},
  {515, 415}, {550, 370}, {538, 322}
}
local jug_body = poly(body_pts, true)
local jug_interior = ellipse(590, 313, 38, 9)

-- Warm unifying glaze across the belly to melt vertical stripes into smooth roundness
local m_belly_fuse = jug_body * mask(function(x, y) return (y > 360 and y < 615) and 1 or 0 end) - jug_interior
work(m_belly_fuse, {hand = "glaze", pile = p_pot_terra, angle = 0.1, coverage = 1.4, edge = 0.3})

-- Core shadow blending
local m_jug_sh = jug_body * mask(function(x, y) return (x > 585 and y > 360 and y < 615) and 1 or 0 end)
work(m_jug_sh, {hand = "glaze", pile = p_pot_shadow, angle = 0.2, coverage = 1.2, edge = 0.4})

-- Reflected light along right edge
local m_jug_rf = jug_body * mask(function(x, y)
  if x < 650 then return 0 end
  return smoothstep(650, 678, x) * smoothstep(380, 430, y) * smoothstep(620, 560, y) * 0.7
end)
work(m_jug_rf, {hand = "glaze", pile = p_pot_refl, angle = 0.3, coverage = 1.2, edge = 0.3})

-- Blend across the jug body to create smooth ceramic turning
blend(m_belly_fuse, {angle = 0.1, coverage = 1.6})

-- Crisp throat opening
work(jug_interior, {hand = "detail", pile = p_pot_throat, angle = 0, coverage = 2.0, clip = true})

-- Lip highlight on left
local b_point = brush{kind = "round", width = 2.8, point = 1, stiffness = 0.7}
b_point:load(p_pot_lit, 0.8)
b_point:stroke({{545, 317}, {570, 322}, {585, 323}}, {pressure = {0.6, 0.85, 0.3}, orient = "along"})

-- Specular glaze highlight on shoulder
b_point:load(p_pot_spec, 0.9)
b_point:stroke({{538, 432}, {542, 452}, {544, 468}}, {pressure = {0.75, 0.95, 0.3}, orient = "along"})
b_point:load(p_pot_spec, 0.7)
b_point:touch(540, 448, {pressure = 0.85})

-- Handle modeling: lit strap top, shadow underside
local b_handle = brush{kind = "round", width = 3.2, point = 1, stiffness = 0.6}
b_handle:load(p_pot_lit, 0.8)
b_handle:stroke({{632, 358}, {675, 368}, {694, 408}}, {pressure = {0.6, 0.8, 0.5}, orient = "along"})
b_handle:load(p_pot_shadow, 0.8)
b_handle:stroke({{636, 372}, {670, 382}, {686, 420}, {678, 465}, {664, 485}}, {pressure = {0.4, 0.7, 0.4}, orient = "along"})


-- -------------------------------------------------------------
-- 2. LEMON PEEL CONNECTION TO CUT LEMON
-- -------------------------------------------------------------
-- Connect the peel start to the lemon cut face rim
local b_peel = brush{kind = "round", width = 3.4, point = 1, stiffness = 0.7}
b_peel:load(p_lem_pure, 0.9)
b_peel:stroke({{455, 648}, {450, 660}, {442, 672}, {430, 686}}, {pressure = {0.7, 0.85, 0.8, 0.6}, orient = "along"})

-- Inner pith highlight along the connect
local b_fine = brush{kind = "round", width = 1.6, point = 1, stiffness = 0.8}
b_fine:load(p_pith, 0.8)
b_fine:stroke({{452, 650}, {447, 662}, {436, 678}}, {pressure = {0.5, 0.7, 0.4}, orient = "along"})

-- Little nipple tip on whole lemon
b_fine:load(p_lem_pure, 0.8)
b_fine:stroke({{312, 626}, {304, 626}, {307, 628}}, {pressure = {0.6, 0.8, 0.4}, orient = "along"})


-- -------------------------------------------------------------
-- 3. TAILORING THE FOLDED LINEN NAPKIN
-- -------------------------------------------------------------
-- Clean, crisp linen body coat
local m_nap_body = poly({
  {140, 546}, {242, 548}, {264, 605}, {262, 674},
  {258, 742}, {218, 745}, {212, 742}, {165, 740}, {165, 674},
  {138, 630}, {134, 580}
}, true)

work(m_nap_body, {hand = "body", pile = p_lin_lit, angle = 0.3, coverage = 1.6, fill = true, edge = 0.1})

-- Two clean vertical folds hanging over the table edge
local v_trough = ribbon({{195, 560}, {210, 625}, {216, 674}, {212, 742}}, 12) * m_nap_body
work(v_trough, {hand = "body", pile = p_lin_sh, angle = 1.2, coverage = 1.5, edge = 0.25})

local v_left_sh = m_nap_body * mask(function(x, y) return smoothstep(165, 134, x) end)
work(v_left_sh, {hand = "body", pile = p_lin_sh, angle = 1.1, coverage = 1.3, edge = 0.25})

-- Crease lines
b_fine:load(p_deep, 0.6)
b_fine:stroke({{196, 565}, {211, 628}, {217, 674}, {213, 740}}, {pressure = {0.2, 0.5, 0.2}, orient = "along"})

-- Lit fold crests
local v_crest1 = ribbon({{155, 548}, {180, 610}, {188, 674}, {185, 740}}, 14) * m_nap_body
local v_crest2 = ribbon({{220, 555}, {238, 615}, {244, 674}, {240, 742}}, 14) * m_nap_body
work(v_crest1, {hand = "body", pile = p_lin_crest, angle = 0.5, coverage = 1.4, edge = 0.25})
work(v_crest2, {hand = "body", pile = p_lin_crest, angle = 0.5, coverage = 1.4, edge = 0.25})

-- Soft blend across folds
blend(m_nap_body, {angle = 0.4, coverage = 1.1})

-- Crisp folded linen hems
local b_hem = brush{kind = "round", width = 2.0, point = 1, stiffness = 0.7}
b_hem:load(p_lin_crest, 0.85)
b_hem:stroke({{262, 674}, {258, 742}, {218, 745}, {212, 742}, {165, 740}}, {pressure = {0.6, 0.8, 0.6}, orient = "along"})
b_hem:stroke({{165, 740}, {165, 674}, {138, 630}, {134, 580}, {140, 546}, {242, 548}}, {pressure = {0.5, 0.7, 0.4}, orient = "along"})

-- Sharp impasto highlights on the table edge folds
local b_imp = brush{kind = "filbert", width = 3.5, stiffness = 0.8}
b_imp:load(p_lin_high, 0.9)
b_imp:touch(188, 674, {pressure = 0.9})
b_imp:touch(244, 674, {pressure = 0.9})

-- Occlusion shadow under hanging drape
local b_dark = brush{kind = "round", width = 2.6, point = 1, stiffness = 0.6}
b_dark:load(p_deep, 0.85)
b_dark:stroke({{164, 742}, {212, 745}, {218, 748}, {260, 744}}, {pressure = {0.5, 0.9, 0.4}, orient = "along"})

print("Final sculptural refinements completed.")

--@ chunk 30
-- Adding the Botanical Lemon Leaf and Stem, and Final Specular Accents

-- Piles
local p_stem = pile{{"raw umber", 6}, {"yellow ochre", 3}, {"bone black", 2}, medium = 0.1}
local p_leaf_dark = pile{{"green earth", 5}, {"yellow ochre", 3}, {"raw umber", 2}, {"Prussian blue", 1}, medium = 0.12}
local p_leaf_lit = pile{{"green earth", 4}, {"yellow ochre", 4}, {"lead white", 2}, medium = 0.12}
local p_leaf_vein = pile{{"yellow ochre", 5}, {"green earth", 3}, {"lead white", 3}, medium = 0.1}
local p_cast = pile{{"raw umber", 6}, {"bone black", 3}, {"red earth", 1}, medium = 0.25}
local p_spec = pile{{"lead white", 9}, {"yellow ochre", 1}, medium = 0.05}

-- 1. Cast shadow of leaf on table
local leaf_shadow = poly({
  {275, 618}, {262, 610}, {238, 608}, {226, 614}, {238, 624}, {260, 626}, {275, 618}
}, true)
work(leaf_shadow, {hand = "glaze", pile = p_cast, angle = 0.1, coverage = 1.3, edge = 0.3})

-- 2. Leaf silhouette
local leaf_pts = {
  {276, 615}, {262, 602}, {240, 598}, {226, 604}, {236, 616}, {258, 620}, {276, 615}
}
local m_leaf = poly(leaf_pts, true)

-- Leaf body coat
work(m_leaf, {hand = "body", pile = p_leaf_dark, angle = -0.4, coverage = 1.8, fill = true, edge = 0.1})

-- Upper lit facet of the leaf
local m_leaf_lit = m_leaf * mask(function(x, y) return (y < 610) and 1 or 0 end)
work(m_leaf_lit, {hand = "body", pile = p_leaf_lit, angle = -0.4, coverage = 1.5, edge = 0.2})

-- Blend leaf softly
blend(m_leaf, {angle = -0.4, coverage = 1.2})

-- Woody stem connecting leaf to lemon tip
local b_stem = brush{kind = "round", width = 1.8, point = 1, stiffness = 0.7}
b_stem:load(p_stem, 0.8)
b_stem:stroke({{310, 626}, {295, 622}, {276, 615}}, {pressure = {0.6, 0.8, 0.5}, orient = "along"})

-- Central midrib vein of leaf
local b_vein = brush{kind = "round", width = 1.2, point = 1, stiffness = 0.8}
b_vein:load(p_leaf_vein, 0.8)
b_vein:stroke({{276, 615}, {252, 608}, {228, 604}}, {pressure = {0.5, 0.7, 0.3}, orient = "along"})

-- 3. Final specular jewel-like impasto accents on the earthenware jug
local b_jewel = brush{kind = "round", width = 2.2, point = 1, stiffness = 0.8}
b_jewel:load(p_spec, 0.95)
-- Lip sparkle
b_jewel:touch(554, 317, {pressure = 0.9})
-- Shoulder glaze specular highlight
b_jewel:stroke({{539, 436}, {542, 452}}, {pressure = {0.85, 0.95}, orient = "along"})

print("Lemon leaf and final touches completed.")

--@ chunk 31
print("Painting clock:", wait(0))

--@ chunk 32
print("Final dry time:", wait(10 * 24 * 60))

--@ chunk 33
print("Drying state:", drying(200, 200), drying(550, 450), drying(360, 630), drying(485, 650), drying(200, 650), drying(790, 656))

--@ chunk 34
print("Session clock:", wait(0))

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
