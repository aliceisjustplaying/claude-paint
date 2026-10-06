-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{
  size=400,
  aspect=1.25,
  linen={16, 14},
  seed=42,
  ground={
    {pile={{"lead white", 10}, {"yellow ochre", 2}, {"raw umber", 1}}, um=80, apply="knife", texture=0.25},
    {pile={{"lead white", 4}, {"yellow ochre", 3}, {"raw umber", 1}, {"red earth", 0.5}}, um=35, apply="brush"}
  }
}

--@ chunk 2
local c = chalk()

-- Horizon / rear shelf line
c:sketch({{0, 535}, {350, 532}, {680, 528}, {1000, 525}}, {pressure=0.35, wander=1.0})

-- Front edge of wooden shelf
c:sketch({{0, 675}, {400, 672}, {750, 670}, {1000, 668}}, {pressure=0.4, wander=1.2})

-- Stoneware pitcher contour
c:sketch({
  {320, 265}, {310, 280}, {315, 330}, -- neck left
  {250, 390}, {235, 450}, {260, 520}, {310, 575}, -- belly left & base
  {440, 575}, {480, 520}, {495, 440}, {470, 380}, -- base right & belly right
  {415, 330}, {420, 280}, {410, 265}, -- neck right
  {365, 260}, {320, 265} -- rim
}, {pressure=0.45, wander=1.5, passes=2})

-- Pitcher mouth ellipse
c:sketch({{320, 265}, {365, 275}, {410, 265}, {365, 258}, {320, 265}}, {pressure=0.35})

-- Pitcher handle
c:sketch({
  {315, 305}, {250, 285}, {200, 320}, {195, 370}, {225, 420}, {255, 430}
}, {pressure=0.4, wander=1.2, passes=2})

-- Quince contour (pyriform, knobby)
c:sketch({
  {610, 450}, {585, 485}, {565, 535}, {550, 580}, {560, 625}, {605, 650},
  {665, 652}, {720, 630}, {745, 575}, {730, 525}, {675, 480}, {635, 455},
  {610, 450}
}, {pressure=0.45, wander=1.5, passes=2})

-- Quince stem
c:line({{610, 450}, {602, 430}, {595, 418}}, {pressure={0.5, 0.3}})

-- Bay sprig in foreground
c:sketch({{470, 620}, {510, 645}, {550, 665}, {560, 685}}, {pressure=0.35})
-- Leaf 1
c:sketch({{510, 645}, {490, 635}, {475, 645}, {510, 655}}, {pressure=0.35})
-- Leaf 2
c:sketch({{535, 655}, {560, 645}, {580, 655}, {545, 665}}, {pressure=0.35})

--@ chunk 3
-- Background wall and shelf lay-in
p_wall_left = pile{{"raw umber", 5}, {"yellow ochre", 3}, {"bone black", 2}, {"lead white", 1}, medium=0.15}
p_wall_right = pile{{"raw umber", 6}, {"bone black", 5}, {"red earth", 1}, medium=0.15}
p_shelf_top = pile{{"raw umber", 4}, {"yellow ochre", 5}, {"lead white", 3}, {"red earth", 1}, medium=0.15}
p_shelf_front = pile{{"bone black", 6}, {"raw umber", 4}, {"red earth", 1}, medium=0.15}

-- Wall mask: top half of canvas down to shelf line
local m_wall = mask(function(x, y)
  local y_shelf = 533 - 8 * (x / 1000)
  if y < y_shelf then
    return 1.0
  else
    return 0.0
  end
end)

-- Left wall (lighter chiaroscuro field)
local m_wall_left = mask(function(x, y)
  local y_shelf = 533 - 8 * (x / 1000)
  if y < y_shelf and x < 550 then
    local fade = clamp((550 - x) / 200, 0, 1)
    return fade
  end
  return 0
end)

-- Right wall (deep dark background behind quince)
local m_wall_right = mask(function(x, y)
  local y_shelf = 533 - 8 * (x / 1000)
  if y < y_shelf and x > 350 then
    local fade = clamp((x - 350) / 200, 0, 1)
    return fade
  end
  return 0
end)

-- Work the background wall
work(m_wall, {hand="broad", pile=p_wall_right, angle=0.2, coverage=1.4, fill=true})
work(m_wall_left, {hand="broad", pile=p_wall_left, angle=-0.2, coverage=1.2, fill=true})
blend(m_wall, {angle=0.1, coverage=1.0})

-- Shelf top mask
local m_shelf = mask(function(x, y)
  local y_top = 533 - 8 * (x / 1000)
  local y_bot = 673 - 6 * (x / 1000)
  if y >= y_top and y < y_bot then
    return 1.0
  end
  return 0.0
end)

work(m_shelf, {hand="broad", pile=p_shelf_top, angle=0.0, coverage=1.5, fill=true})
blend(m_shelf, {angle=0.0, coverage=0.8})

-- Shelf front apron (deep shadow)
local m_front = mask(function(x, y)
  local y_bot = 673 - 6 * (x / 1000)
  if y >= y_bot then
    return 1.0
  end
  return 0.0
end)

work(m_front, {hand="broad", pile=p_shelf_front, angle=0.05, coverage=1.6, fill=true})
blend(m_front, {angle=0.0, coverage=0.8})

--@ chunk 4
print("left:", drying(200, 300), "right:", drying(800, 300))

--@ chunk 5
p_unify = pile{{"raw umber", 6}, {"bone black", 3}, {"yellow ochre", 2}, {"red earth", 1}, medium=0.35}

local m_trans = mask(function(x, y)
  local y_shelf = 533 - 8 * (x / 1000)
  if y < y_shelf and x >= 200 and x <= 750 then
    local t = (x - 200) / 550
    return math.sin(t * math.pi)
  end
  return 0
end)

work(m_trans, {hand="broad", pile=p_unify, angle=0.05, coverage=1.3, fill=true})

local m_wall_all = mask(function(x, y)
  local y_shelf = 533 - 8 * (x / 1000)
  return y < y_shelf and 1 or 0
end)

blend(m_wall_all, {angle=0.02, coverage=1.8})

--@ chunk 6
-- Define palettes for objects
p_pitcher_dark = pile{{"raw umber", 6}, {"red earth", 3}, {"bone black", 3}, medium=0.1}
p_pitcher_mid = pile{{"raw umber", 4}, {"red earth", 3}, {"yellow ochre", 3}, {"lead white", 1.5}, medium=0.1}

p_quince_shadow = pile{{"yellow ochre", 5}, {"raw umber", 3}, {"green earth", 2}, medium=0.1}
p_quince_light = pile{{"yellow ochre", 7}, {"chrome yellow", 2}, {"lead white", 2}, {"raw umber", 1}, medium=0.1}

-- Refine shelf plane
local m_shelf = mask(function(x, y)
  local y_top = 533 - 8 * (x / 1000)
  local y_bot = 673 - 6 * (x / 1000)
  if y >= y_top and y < y_bot then
    return 1.0
  end
  return 0.0
end)

work(m_shelf, {hand="broad", pile=pile{{"yellow ochre", 5}, {"raw umber", 4}, {"lead white", 2.5}, {"red earth", 1}, medium=0.15}, angle=0.0, coverage=1.2, fill=true})
blend(m_shelf, {angle=0.0, coverage=0.8})

-- Helper for rotated elliptical shadow
local function rot_ellipse(cx, cy, rx, ry, theta)
  local cos_t, sin_t = math.cos(theta), math.sin(theta)
  return mask(function(x, y)
    local dx = x - cx
    local dy = y - cy
    local nx = cos_t * dx + sin_t * dy
    local ny = -sin_t * dx + cos_t * dy
    local d = (nx / rx)^2 + (ny / ry)^2
    if d <= 1.0 then
      return 1.0
    elseif d < 1.4 then
      return (1.4 - d) / 0.4
    else
      return 0.0
    end
  end)
end

-- Cast shadows on the shelf:
local m_shadow_pitcher = rot_ellipse(480, 600, 100, 35, 0.25)
local m_shadow_quince = rot_ellipse(720, 645, 70, 25, 0.2)

local p_cast_shadow = pile{{"bone black", 5}, {"raw umber", 5}, {"red earth", 1}, medium=0.25}
work(m_shadow_pitcher, {hand="body", pile=p_cast_shadow, angle=0.2, coverage=1.4})
work(m_shadow_quince, {hand="body", pile=p_cast_shadow, angle=0.2, coverage=1.4})
blend(m_shadow_pitcher + m_shadow_quince, {angle=0.2, coverage=0.8})

-- Pitcher body silhouette
local pts_pitcher = {
  {320, 265}, {310, 280}, {315, 330},
  {250, 390}, {235, 450}, {260, 520}, {310, 575},
  {440, 575}, {480, 520}, {495, 440}, {470, 380},
  {415, 330}, {420, 280}, {410, 265},
  {365, 260}
}
local m_pitcher_body = poly(pts_pitcher, true) + ellipse(365, 265, 45, 12)
local m_handle = ribbon({{320, 310}, {245, 285}, {195, 330}, {195, 375}, {225, 415}, {255, 430}}, 14)
local m_pitcher_all = m_pitcher_body + m_handle

-- Pitcher underpainting: lit side (left) vs shadow side (right)
local m_pitcher_lit = m_pitcher_all * mask(function(x, y) return x < 360 and 1 or clamp((410 - x) / 50, 0, 1) end)
local m_pitcher_shd = m_pitcher_all * mask(function(x, y) return x >= 360 and 1 or clamp((x - 310) / 50, 0, 1) end)

work(m_pitcher_shd, {hand="body", pile=p_pitcher_dark, angle=1.4, coverage=1.6, fill=true})
work(m_pitcher_lit, {hand="body", pile=p_pitcher_mid, angle=1.4, coverage=1.5, fill=true})
blend(m_pitcher_all, {angle=1.4, coverage=0.9})

-- Quince silhouette
local pts_quince = {
  {610, 450}, {585, 485}, {565, 535}, {550, 580}, {560, 625}, {605, 650},
  {665, 652}, {720, 630}, {745, 575}, {730, 525}, {675, 480}, {635, 455}
}
local m_quince_all = poly(pts_quince, true)

-- Quince lit side (upper-left) vs shadow side (lower-right)
local m_quince_lit = m_quince_all * mask(function(x, y)
  local val = (1200 - (x + y)) / 60
  return clamp(val, 0, 1)
end)
local m_quince_shd = m_quince_all * mask(function(x, y)
  local val = ((x + y) - 1140) / 60
  return clamp(val, 0, 1)
end)

work(m_quince_shd, {hand="body", pile=p_quince_shadow, angle=0.8, coverage=1.6, fill=true})
work(m_quince_lit, {hand="body", pile=p_quince_light, angle=0.8, coverage=1.5, fill=true})
blend(m_quince_all, {angle=0.8, coverage=0.8})

--@ chunk 7
print(wait(0))

--@ chunk 8
print("pitcher:", drying(360, 450), "quince:", drying(650, 550), "bg:", drying(200, 300))

--@ chunk 9
-- Chunk 9: Carving negative space, modeling the jug and quince

-- 1. Carve handle negative space and outer wall boundary
local m_handle_hole = ellipse(265, 355, 18, 35)
work(m_handle_hole, {hand="body", pile=p_wall_left, angle=0.0, coverage=1.8, fill=true, clip=true})

-- Clean up outer left of handle
local m_outer_left = mask(function(x, y)
  if x < 190 and y > 260 and y < 520 then return 1.0 end
  if x < 210 and y > 430 and y < 520 then return clamp((210 - x)/20, 0, 1) end
  return 0.0
end)
work(m_outer_left, {hand="body", pile=p_wall_left, angle=0.0, coverage=1.5, fill=true})

-- 2. Pitcher mouth interior shadow
local p_interior = pile{{"bone black", 7}, {"raw umber", 4}, medium=0.05}
local m_mouth = ellipse(365, 267, 36, 9)
work(m_mouth, {hand="detail", pile=p_interior, angle=0.0, coverage=2.0, fill=true, clip=true})

-- 3. Pitcher body modeling:
-- Half-tone terracotta on lit flank
local p_pitcher_half = pile{{"raw umber", 4}, {"red earth", 4}, {"yellow ochre", 3}, {"lead white", 1.2}, medium=0.1}
local p_pitcher_bounce = pile{{"raw umber", 4}, {"yellow ochre", 3}, {"red earth", 2}, medium=0.15}

-- Lit flank of belly (x: 260 to 360, y: 360 to 520)
local m_belly_lit = ellipse(310, 440, 50, 70)
work(m_belly_lit, {hand="body", pile=p_pitcher_half, angle=0.3, coverage=1.5, fill=true})

-- Neck lit side
local m_neck_lit = ribbon({{330, 275}, {335, 330}}, 18)
work(m_neck_lit, {hand="body", pile=p_pitcher_half, angle=1.5, coverage=1.5, fill=true})

-- Reflected bounce light on the shadow side (right flank)
local m_bounce = ribbon({{475, 410}, {485, 460}, {460, 530}}, 14)
work(m_bounce, {hand="body", pile=p_pitcher_bounce, angle=1.3, coverage=1.3})

-- Blend the pitcher body to fuse internal transitions
local pts_pitcher = {
  {320, 265}, {310, 280}, {315, 330},
  {250, 390}, {235, 450}, {260, 520}, {310, 575},
  {440, 575}, {480, 520}, {495, 440}, {470, 380},
  {415, 330}, {420, 280}, {410, 265},
  {365, 260}
}
local m_pitcher_body = poly(pts_pitcher, true)
blend(m_pitcher_body, {angle=0.4, coverage=1.0, clip=true})

-- 4. Quince modeling:
-- Subtle greenish half-tone / bloom
local p_quince_bloom = pile{{"yellow ochre", 4}, {"green earth", 3}, {"pale smalt", 1}, {"lead white", 2}, medium=0.15}
local m_quince_bloom = ribbon({{585, 490}, {620, 540}, {660, 580}}, 26)
work(m_quince_bloom, {hand="body", pile=p_quince_bloom, angle=0.6, coverage=1.3})

-- Warm golden midtones
local p_quince_gold = pile{{"yellow ochre", 6}, {"chrome yellow", 3}, {"lead white", 3}, {"red earth", 0.5}, medium=0.1}
local m_quince_belly = ellipse(625, 530, 45, 55)
work(m_quince_belly, {hand="body", pile=p_quince_gold, angle=0.5, coverage=1.5, fill=true})

-- Blend quince body
local pts_quince = {
  {610, 450}, {585, 485}, {565, 535}, {550, 580}, {560, 625}, {605, 650},
  {665, 652}, {720, 630}, {745, 575}, {730, 525}, {675, 480}, {635, 455}
}
local m_quince_body = poly(pts_quince, true)
blend(m_quince_body, {angle=0.5, coverage=1.0, clip=true})

-- 5. Cast shadows unification & contact occlusion
local function rot_ellipse(cx, cy, rx, ry, theta)
  local cos_t, sin_t = math.cos(theta), math.sin(theta)
  return mask(function(x, y)
    local dx = x - cx
    local dy = y - cy
    local nx = cos_t * dx + sin_t * dy
    local ny = -sin_t * dx + cos_t * dy
    local d = (nx / rx)^2 + (ny / ry)^2
    return d <= 1.0 and 1.0 or (d < 1.3 and (1.3 - d) / 0.3 or 0.0)
  end)
end

local p_shadow_glaze = pile{{"bone black", 5}, {"raw umber", 5}, {"red earth", 1}, medium=0.35}
local m_shadow_all = rot_ellipse(480, 600, 105, 38, 0.25) + rot_ellipse(720, 645, 75, 28, 0.2)
work(m_shadow_all, {hand="body", pile=p_shadow_glaze, angle=0.2, coverage=1.0})
blend(m_shadow_all, {angle=0.2, coverage=1.2})

-- Crisp dark contact occlusion at base of jug and quince
local p_contact = pile{{"bone black", 8}, {"raw umber", 4}, medium=0.05}
local b_detail = brush("round", 3)
b_detail:load(p_contact, 0.8)
b_detail:stroke({{310, 577}, {375, 580}, {440, 577}}, {pressure={0.8, 0.4}})
b_detail:stroke({{570, 648}, {630, 655}, {700, 642}}, {pressure={0.7, 0.4}})

--@ chunk 10
-- Re-unify the left background wall
local m_wall_left_patch = mask(function(x, y)
  if x < 320 and y < 530 then
    -- Don't hit the pitcher body center
    if x > 220 and y > 380 and y < 530 then return 0 end
    return 1.0
  end
  return 0.0
end)

work(m_wall_left_patch, {hand="broad", pile=p_unify, angle=0.05, coverage=1.4, fill=true})
blend(mask(function(x, y) return x < 400 and y < 535 and 1 or 0 end), {angle=0.0, coverage=1.2})

-- Advance time to let underpainting set
wait(180)

print("Status after 3 hours drying:")
print("pitcher:", drying(360, 450))
print("quince:", drying(650, 550))
print("wall:", drying(200, 300))

--@ chunk 11
-- Chunk 12: Definitive modeling and brushwork on the stoneware pitcher

-- Piles for earthenware clay and glaze
local p_clay_shadow   = pile{{"raw umber", 6}, {"bone black", 3}, {"red earth", 2}, medium=0.08}
local p_clay_body     = pile{{"raw umber", 4}, {"red earth", 4}, {"yellow ochre", 2}, {"lead white", 0.5}, medium=0.08}
local p_clay_halftone = pile{{"yellow ochre", 4}, {"red earth", 3}, {"raw umber", 2}, {"lead white", 1.5}, medium=0.08}
local p_clay_lit      = pile{{"yellow ochre", 5}, {"lead white", 4}, {"red earth", 1.5}, {"raw umber", 1}, medium=0.05}
local p_clay_glaze_hi = pile{{"lead white", 8}, {"yellow ochre", 2}, {"raw umber", 0.5}, medium=0.02}
local p_clay_bounce   = pile{{"raw umber", 3}, {"yellow ochre", 4}, {"red earth", 2}, medium=0.1}
local p_deep_dark     = pile{{"bone black", 8}, {"raw umber", 3}, medium=0.04}

local b_filbert = brush("filbert", 12)
local b_round   = brush("round", 6)
local b_detail  = brush{kind="round", width=3, point=1, stiffness=0.6}

-- 1. Base and belly mass:
-- Shadow side strokes (following horizontal curvature of the belly)
b_filbert:load(p_clay_shadow, 0.8)
b_filbert:stroke({{350, 390}, {410, 400}, {460, 420}}, {pressure={0.8, 0.5}})
b_filbert:stroke({{350, 440}, {420, 455}, {485, 470}}, {pressure={0.9, 0.6}})
b_filbert:stroke({{350, 490}, {415, 510}, {470, 530}}, {pressure={0.8, 0.5}})
b_filbert:stroke({{340, 540}, {390, 555}, {440, 565}}, {pressure={0.8, 0.4}})

-- Belly midtone transitions
b_filbert:load(p_clay_body, 0.85)
b_filbert:stroke({{280, 390}, {330, 400}, {380, 410}}, {pressure={0.8, 0.7}})
b_filbert:stroke({{260, 440}, {320, 450}, {380, 460}}, {pressure={0.9, 0.8}})
b_filbert:stroke({{275, 490}, {330, 505}, {380, 515}}, {pressure={0.8, 0.7}})
b_filbert:stroke({{290, 540}, {340, 550}, {380, 555}}, {pressure={0.7, 0.6}})

-- Lit shoulder and belly
b_filbert:load(p_clay_halftone, 0.85)
b_filbert:stroke({{265, 370}, {300, 385}, {335, 395}}, {pressure={0.8, 0.7}})
b_filbert:stroke({{250, 415}, {290, 430}, {330, 440}}, {pressure={0.9, 0.8}})
b_filbert:stroke({{245, 460}, {285, 470}, {325, 475}}, {pressure={0.8, 0.7}})
b_filbert:stroke({{270, 515}, {300, 525}, {335, 530}}, {pressure={0.7, 0.6}})

-- Strong light plane on upper shoulder
b_filbert:load(p_clay_lit, 0.8)
b_filbert:stroke({{275, 360}, {310, 375}, {340, 380}}, {pressure={0.7, 0.5}})
b_filbert:stroke({{260, 400}, {295, 415}, {325, 420}}, {pressure={0.8, 0.6}})

-- Reflected bounce light on right shadow edge
b_round:load(p_clay_bounce, 0.6)
b_round:stroke({{470, 410}, {485, 460}, {465, 525}}, {pressure={0.4, 0.6, 0.3}})

-- 2. Neck and Collar:
-- Shadow on right of neck
b_filbert:load(p_clay_shadow, 0.7)
b_filbert:stroke({{375, 275}, {380, 310}, {385, 340}}, {pressure={0.7, 0.7}})
b_filbert:stroke({{395, 275}, {400, 305}, {405, 335}}, {pressure={0.6, 0.5}})

-- Lit side on left of neck
b_filbert:load(p_clay_halftone, 0.75)
b_filbert:stroke({{345, 275}, {345, 305}, {345, 335}}, {pressure={0.7, 0.7}})
b_round:load(p_clay_lit, 0.7)
b_round:stroke({{325, 275}, {328, 305}, {330, 335}}, {pressure={0.6, 0.5}})

-- 3. Mouth and Rim:
-- Deep black interior void
b_detail:load(p_deep_dark, 0.9)
b_detail:stroke({{335, 267}, {365, 268}, {395, 267}}, {pressure={0.8, 0.9, 0.6}})

-- Rounded lip contour catching light
b_detail:load(p_clay_lit, 0.85)
b_detail:stroke({{315, 268}, {345, 274}, {380, 273}}, {pressure={0.6, 0.8, 0.4}})
b_detail:load(p_clay_shadow, 0.7)
b_detail:stroke({{380, 273}, {405, 270}, {415, 266}}, {pressure={0.4, 0.5, 0.3}})

-- 4. The Arched Strap Handle:
-- Under-shadow of handle
b_round:load(p_clay_shadow, 0.85)
b_round:stroke({{315, 310}, {255, 290}, {205, 335}, {205, 385}, {235, 420}, {255, 430}},
               {pressure={0.7, 0.8, 0.8, 0.7, 0.6}})

-- Top spine / lit crest of handle
b_detail:load(p_clay_halftone, 0.8)
b_detail:stroke({{315, 303}, {250, 278}, {198, 330}, {198, 375}, {225, 412}, {250, 425}},
                {pressure={0.4, 0.7, 0.7, 0.5, 0.3}})
b_detail:load(p_clay_lit, 0.7)
b_detail:stroke({{280, 283}, {235, 286}, {202, 320}, {200, 355}}, {pressure={0.3, 0.6, 0.6, 0.2}})

-- 5. Specular Glaze Sheen / Highlights:
-- Shoulder highlight (juicy impasto)
b_detail:load(p_clay_glaze_hi, 0.9)
b_detail:stroke({{295, 375}, {312, 400}, {318, 425}}, {pressure={0.2, 0.7, 0.3}})
-- Neck highlight
b_detail:load(p_clay_glaze_hi, 0.75)
b_detail:stroke({{332, 280}, {334, 305}, {335, 325}}, {pressure={0.2, 0.5, 0.2}})
-- Rim gleam
b_detail:load(p_clay_glaze_hi, 0.8)
b_detail:touch(335, 273, {pressure=0.6})
-- Secondary belly gleam
b_detail:load(p_clay_glaze_hi, 0.65)
b_detail:stroke({{290, 445}, {295, 465}}, {pressure={0.2, 0.5, 0.1}})

--@ chunk 12
-- Chunk 13: Modeling the golden quince and refining the wooden shelf

-- 1. Piles for Quince
local p_quince_deep_shd = pile{{"raw umber", 4}, {"yellow ochre", 4}, {"green earth", 3}, {"bone black", 1}, medium=0.08}
local p_quince_shadow   = pile{{"yellow ochre", 6}, {"raw umber", 3}, {"green earth", 2}, medium=0.08}
local p_quince_bounce   = pile{{"yellow ochre", 6}, {"red earth", 2}, {"raw umber", 2}, {"lead white", 1}, medium=0.1}
local p_quince_bloom    = pile{{"yellow ochre", 4}, {"green earth", 2}, {"pale smalt", 1.5}, {"lead white", 3}, medium=0.12}
local p_quince_body     = pile{{"yellow ochre", 6}, {"chrome yellow", 4}, {"lead white", 2}, medium=0.06}
local p_quince_lit      = pile{{"chrome yellow", 6}, {"lead white", 5}, {"yellow ochre", 2}, medium=0.04}
local p_quince_high     = pile{{"lead white", 8}, {"chrome yellow", 3}, {"yellow ochre", 0.5}, medium=0.02}
local p_calyx_stem      = pile{{"raw umber", 6}, {"bone black", 4}, {"red earth", 1}, medium=0.05}

local b_filbert = brush("filbert", 10)
local b_round   = brush("round", 5)
local b_detail  = brush{kind="round", width=2.8, point=1, stiffness=0.6}

-- Deep shadow on lower right flank of quince
b_filbert:load(p_quince_deep_shd, 0.85)
b_filbert:stroke({{650, 520}, {690, 550}, {725, 590}, {710, 630}}, {pressure={0.7, 0.9, 0.8, 0.5}})
b_filbert:stroke({{630, 580}, {670, 610}, {700, 635}}, {pressure={0.8, 0.9, 0.6}})

-- Shadow transition (golden-olive)
b_filbert:load(p_quince_shadow, 0.85)
b_filbert:stroke({{620, 500}, {660, 530}, {680, 570}}, {pressure={0.8, 0.85, 0.7}})
b_filbert:stroke({{590, 550}, {630, 590}, {660, 625}}, {pressure={0.8, 0.85, 0.6}})

-- Reflected light along lower right contour
b_round:load(p_quince_bounce, 0.7)
b_round:stroke({{735, 570}, {730, 610}, {705, 640}}, {pressure={0.4, 0.6, 0.3}})

-- Delicate greenish bloom across the terminator
b_round:load(p_quince_bloom, 0.7)
b_round:stroke({{600, 480}, {625, 520}, {645, 565}, {635, 610}}, {pressure={0.4, 0.6, 0.5, 0.3}})

-- Warm golden body of the quince
b_filbert:load(p_quince_body, 0.9)
b_filbert:stroke({{585, 475}, {570, 520}, {560, 570}, {575, 615}}, {pressure={0.7, 0.85, 0.85, 0.6}})
b_filbert:stroke({{600, 465}, {590, 510}, {590, 560}, {605, 610}}, {pressure={0.8, 0.9, 0.85, 0.7}})
b_filbert:stroke({{615, 460}, {610, 500}, {615, 550}, {625, 590}}, {pressure={0.8, 0.85, 0.8, 0.6}})

-- Lit face and knobby bulges
b_filbert:load(p_quince_lit, 0.9)
b_filbert:stroke({{595, 470}, {580, 510}, {575, 555}}, {pressure={0.7, 0.85, 0.6}})
b_round:load(p_quince_lit, 0.85)
b_round:stroke({{565, 560}, {575, 595}, {600, 620}}, {pressure={0.6, 0.8, 0.5}})

-- Specular impasto highlights on the upper crest of the fruit
b_detail:load(p_quince_high, 0.95)
b_detail:stroke({{585, 475}, {580, 495}, {575, 520}}, {pressure={0.3, 0.8, 0.4}})
b_detail:touch(578, 490, {pressure=0.85})
b_detail:stroke({{595, 465}, {602, 480}}, {pressure={0.3, 0.7, 0.2}})

-- Calyx / blossom scar and woody stem
b_detail:load(p_calyx_stem, 0.9)
-- Calyx socket
b_detail:stroke({{602, 452}, {612, 450}, {618, 453}}, {pressure={0.5, 0.8, 0.4}})
b_detail:touch(610, 451, {pressure=0.8})
-- Woody curved stem pointing slightly up and left
b_detail:stroke({{610, 450}, {605, 435}, {596, 420}}, {pressure={0.6, 0.5, 0.3}})

-- 2. Wooden Shelf & Ledge
local p_wood_plane   = pile{{"yellow ochre", 5}, {"raw umber", 4}, {"lead white", 3}, {"red earth", 1}, medium=0.15}
local p_wood_grain   = pile{{"raw umber", 5}, {"yellow ochre", 3}, {"bone black", 1}, medium=0.15}
local p_ledge_bevel  = pile{{"lead white", 6}, {"yellow ochre", 4}, {"raw umber", 1}, medium=0.05}
local p_table_apron  = pile{{"bone black", 7}, {"raw umber", 4}, {"red earth", 1}, medium=0.12}

local b_flat = brush("flat", 8)

-- Wood grain strokes running horizontally across the shelf
b_flat:load(p_wood_plane, 0.8)
b_flat:stroke({{0, 545}, {260, 545}}, {pressure={0.7, 0.7}})
b_flat:stroke({{460, 540}, {560, 538}}, {pressure={0.6, 0.6}})
b_flat:stroke({{720, 535}, {1000, 530}}, {pressure={0.7, 0.7}})

b_flat:load(p_wood_grain, 0.6)
b_flat:stroke({{0, 590}, {250, 590}}, {pressure={0.5, 0.4}})
b_flat:stroke({{740, 580}, {1000, 575}}, {pressure={0.5, 0.5}})
b_flat:stroke({{0, 630}, {280, 630}}, {pressure={0.5, 0.4}})
b_flat:stroke({{740, 625}, {1000, 620}}, {pressure={0.5, 0.5}})

-- Sharp, luminous front bevel highlight along the ledge edge (y ~ 672)
local b_liner = brush{kind="round", width=2.5, point=1, stiffness=0.7}
b_liner:load(p_ledge_bevel, 0.9)
b_liner:stroke({{0, 674}, {280, 673}, {500, 672}, {750, 671}, {1000, 670}},
               {pressure={0.6, 0.8, 0.7, 0.8, 0.5}})

-- Deep shadow right underneath the bevel lip
local b_shadow = brush("round", 4)
b_shadow:load(p_table_apron, 0.85)
b_shadow:stroke({{0, 678}, {300, 677}, {600, 676}, {1000, 674}}, {pressure={0.7, 0.8, 0.7, 0.6}})

--@ chunk 13
-- Chunk 15: Integrating the pitcher shoulder, solidifying the handle, refining edges and cast shadows

local p_wall_cut   = pile{{"raw umber", 6}, {"bone black", 3}, {"yellow ochre", 2}, medium=0.15}
local p_wall_dark  = pile{{"raw umber", 6}, {"bone black", 5}, {"red earth", 1}, medium=0.15}
local p_clay_glaze = pile{{"red earth", 4}, {"raw umber", 4}, {"yellow ochre", 2}, medium=0.25}
local p_clay_body  = pile{{"raw umber", 4}, {"red earth", 4}, {"yellow ochre", 3}, {"lead white", 1}, medium=0.08}
local p_clay_shadow = pile{{"raw umber", 6}, {"bone black", 4}, {"red earth", 2}, medium=0.08}
local p_clay_hi    = pile{{"lead white", 8}, {"yellow ochre", 2}, {"red earth", 0.5}, medium=0.03}
local p_deep_black = pile{{"bone black", 8}, {"raw umber", 3}, medium=0.04}

local b_filbert = brush("filbert", 10)
local b_round   = brush("round", 6)
local b_detail  = brush{kind="round", width=2.8, point=1, stiffness=0.6}

-- 1. Negative space inside handle loop
local m_handle_inner = poly({{220, 335}, {265, 305}, {305, 320}, {295, 370}, {255, 400}, {225, 380}}, true)
work(m_handle_inner, {hand="detail", pile=p_wall_cut, angle=0.0, coverage=1.8, fill=true, clip=true})

-- Outer contour cleanup around handle and neck
b_filbert:load(p_wall_cut, 0.8)
b_filbert:stroke({{185, 270}, {180, 340}, {185, 420}}, {pressure={0.6, 0.7, 0.5}})
b_filbert:stroke({{290, 255}, {350, 250}, {430, 255}}, {pressure={0.5, 0.6, 0.5}})

-- 2. Glaze over pitcher shoulder to fuse striped marks into rich earthenware form
b_filbert:load(p_clay_glaze, 0.7)
b_filbert:stroke({{250, 360}, {300, 380}, {340, 400}}, {pressure={0.6, 0.7, 0.5}})
b_filbert:stroke({{240, 400}, {290, 420}, {330, 435}}, {pressure={0.7, 0.8, 0.6}})
b_filbert:stroke({{245, 440}, {285, 455}, {325, 465}}, {pressure={0.6, 0.7, 0.5}})

-- Soft blend on the glazed shoulder
blend(ellipse(290, 420, 55, 65), {angle=0.4, coverage=0.8})

-- 3. Solid earthenware strap handle
-- Underbody / shadow of handle
b_round:load(p_clay_shadow, 0.85)
b_round:stroke({{315, 310}, {245, 285}, {195, 335}, {200, 380}, {230, 415}, {255, 430}},
               {pressure={0.8, 0.85, 0.85, 0.8, 0.6}})

-- Lit face / spine of handle
b_detail:load(p_clay_body, 0.85)
b_detail:stroke({{315, 304}, {245, 280}, {198, 330}, {202, 375}, {228, 410}, {250, 425}},
                {pressure={0.6, 0.8, 0.8, 0.6, 0.4}})

-- Gleam on handle top curve
b_detail:load(p_clay_hi, 0.8)
b_detail:stroke({{265, 283}, {230, 285}, {202, 315}, {200, 345}}, {pressure={0.3, 0.7, 0.7, 0.2}})

-- 4. Ceramic glaze specular highlights on pitcher
-- Natural curved highlight on shoulder swell
b_detail:load(p_clay_hi, 0.9)
b_detail:stroke({{280, 385}, {298, 410}, {308, 435}}, {pressure={0.2, 0.8, 0.3}})
-- Neck highlight
b_detail:load(p_clay_hi, 0.8)
b_detail:stroke({{332, 280}, {334, 305}, {335, 325}}, {pressure={0.2, 0.6, 0.2}})
-- Rim light on left lip
b_detail:touch(335, 273, {pressure=0.7})

-- 5. Firm up background wall behind Quince right silhouette
b_filbert:load(p_wall_dark, 0.85)
b_filbert:stroke({{640, 455}, {685, 485}, {740, 530}, {755, 580}, {735, 630}},
                 {pressure={0.6, 0.8, 0.8, 0.8, 0.5}})

-- 6. Translucent glaze across cast shadows on the wooden shelf
local p_shadow_glaze = pile{{"raw umber", 6}, {"bone black", 3}, medium=0.4}
local b_flat = brush("flat", 12)
b_flat:load(p_shadow_glaze, 0.6)
b_flat:stroke({{380, 600}, {480, 605}, {560, 615}}, {pressure={0.4, 0.5, 0.3}})
b_flat:stroke({{650, 640}, {730, 645}, {800, 650}}, {pressure={0.4, 0.5, 0.3}})

-- Firm dark contact occlusion at base of jug and quince
b_detail:load(p_deep_black, 0.95)
b_detail:stroke({{305, 577}, {375, 580}, {445, 576}}, {pressure={0.8, 0.9, 0.7}})
b_detail:stroke({{565, 648}, {630, 654}, {705, 641}}, {pressure={0.8, 0.9, 0.7}})

--@ chunk 14
-- Chunk 16: Bay leaves in foreground, quince edge integration, and refined details

-- 1. Integrate Quince right shadow edge into background
local p_quince_sfumato = pile{{"raw umber", 5}, {"bone black", 4}, {"yellow ochre", 2}, medium=0.25}
local b_filbert = brush("filbert", 8)
b_filbert:load(p_quince_sfumato, 0.5)
b_filbert:stroke({{660, 470}, {715, 515}, {745, 565}, {735, 615}}, {pressure={0.3, 0.5, 0.5, 0.3}})

-- Re-touch the warm bounce inside the quince shadow
local p_quince_bounce = pile{{"yellow ochre", 6}, {"red earth", 2}, {"raw umber", 2}, {"lead white", 1}, medium=0.1}
local b_round = brush("round", 4)
b_round:load(p_quince_bounce, 0.65)
b_round:stroke({{725, 560}, {720, 600}, {695, 635}}, {pressure={0.3, 0.6, 0.3}})

-- 2. Bay Leaves Palettes
local p_leaf_dark   = pile{{"green earth", 6}, {"raw umber", 4}, {"bone black", 1}, {"Prussian blue", 0.5}, medium=0.08}
local p_leaf_mid    = pile{{"green earth", 5}, {"yellow ochre", 4}, {"raw umber", 2}, {"lead white", 1}, medium=0.08}
local p_leaf_lit    = pile{{"yellow ochre", 5}, {"green earth", 3}, {"lead white", 3}, {"chrome yellow", 1}, medium=0.06}
local p_leaf_vein   = pile{{"lead white", 5}, {"yellow ochre", 4}, {"green earth", 1}, medium=0.04}
local p_stem        = pile{{"raw umber", 7}, {"red earth", 3}, {"bone black", 1}, medium=0.06}
local p_leaf_shadow = pile{{"bone black", 6}, {"raw umber", 5}, medium=0.25}

local b_leaf = brush("filbert", 5)
local b_detail = brush{kind="round", width=2.2, point=1, stiffness=0.7}

-- Cast shadows of bay leaves on shelf
b_detail:load(p_leaf_shadow, 0.7)
b_detail:stroke({{460, 633}, {485, 638}, {505, 642}}, {pressure={0.3, 0.5, 0.2}})
b_detail:stroke({{530, 654}, {560, 653}, {582, 656}}, {pressure={0.3, 0.5, 0.2}})
-- Cast shadow of overhanging leaf on the vertical apron
b_detail:stroke({{552, 680}, {560, 695}, {565, 708}}, {pressure={0.4, 0.6, 0.3}})

-- Woody twig / stem
b_detail:load(p_stem, 0.85)
b_detail:stroke({{475, 615}, {505, 638}, {535, 660}, {552, 680}}, {pressure={0.4, 0.7, 0.6, 0.3}})

-- Leaf 1 (Left leaf, pointing back-left)
-- Dark underside
b_leaf:load(p_leaf_dark, 0.85)
b_leaf:stroke({{500, 633}, {475, 630}, {448, 624}}, {pressure={0.6, 0.8, 0.3}})
-- Lit upper facet
b_leaf:load(p_leaf_mid, 0.85)
b_leaf:stroke({{498, 628}, {472, 622}, {448, 624}}, {pressure={0.5, 0.7, 0.2}})
-- Highlight along the leaf ridge
b_detail:load(p_leaf_lit, 0.8)
b_detail:stroke({{490, 627}, {470, 622}, {452, 624}}, {pressure={0.2, 0.6, 0.2}})
-- Pale central vein
b_detail:load(p_leaf_vein, 0.75)
b_detail:stroke({{495, 630}, {470, 625}, {450, 624}}, {pressure={0.2, 0.4, 0.1}})

-- Leaf 2 (Center-right leaf, resting on table)
-- Dark underside
b_leaf:load(p_leaf_dark, 0.85)
b_leaf:stroke({{520, 648}, {550, 646}, {580, 652}}, {pressure={0.5, 0.7, 0.3}})
-- Lit upper facet
b_leaf:load(p_leaf_lit, 0.85)
b_leaf:stroke({{522, 644}, {552, 640}, {580, 652}}, {pressure={0.5, 0.7, 0.2}})
-- Central vein
b_detail:load(p_leaf_vein, 0.75)
b_detail:stroke({{522, 646}, {550, 643}, {578, 652}}, {pressure={0.2, 0.4, 0.1}})

-- Leaf 3 (Curling over the front bevel edge)
-- Leaf curving over edge
b_leaf:load(p_leaf_dark, 0.85)
b_leaf:stroke({{535, 663}, {548, 675}, {558, 695}}, {pressure={0.6, 0.8, 0.4}})
b_leaf:load(p_leaf_lit, 0.85)
b_leaf:stroke({{534, 660}, {546, 672}, {556, 693}}, {pressure={0.5, 0.8, 0.3}})
-- Bevel crest catch-light on the curled leaf
b_detail:load(p_leaf_vein, 0.85)
b_detail:stroke({{544, 671}, {548, 676}}, {pressure={0.3, 0.7, 0.3}})

-- 3. Pitcher finishing specular accents
local p_specular = pile{{"lead white", 9}, {"chrome yellow", 1}, medium=0.01}
-- Crisp point highlight on the ceramic shoulder
b_detail:load(p_specular, 0.95)
b_detail:touch(296, 402, {pressure=0.8})
b_detail:touch(333, 275, {pressure=0.7})
b_detail:touch(335, 300, {pressure=0.6})
