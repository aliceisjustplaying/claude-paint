-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{
  size = 450,
  aspect = 1.25,
  linen = {17, 14},
  seed = 42,
  ground = {
    {
      pile = {{"lead white", 10}, {"yellow ochre", 1}, {"raw umber", 0.5}},
      um = 80,
      apply = "knife",
      texture = 0.22
    },
    {
      pile = {{"yellow ochre", 4}, {"raw umber", 3}, {"lead white", 1}},
      um = 30,
      apply = "brush"
    }
  }
}

--@ chunk 2
c = chalk()

-- Stone shelf edges
c:sketch({{0, 520}, {1000, 520}}, {pressure=0.25, wander=1.2})
c:sketch({{0, 640}, {1000, 640}}, {pressure=0.35, wander=1.5})

-- Terracotta jug outline
-- Rim
c:sketch({{325, 280}, {380, 275}, {435, 280}, {440, 290}, {380, 295}, {320, 290}, {325, 280}}, {pressure=0.4})
-- Neck & shoulder
c:sketch({{340, 290}, {345, 340}, {310, 390}, {265, 470}, {280, 560}, {315, 620}}, {pressure=0.4})
c:sketch({{420, 290}, {415, 340}, {450, 390}, {495, 470}, {480, 560}, {445, 620}}, {pressure=0.4})
-- Base
c:sketch({{315, 620}, {380, 625}, {445, 620}}, {pressure=0.45})
-- Handle
c:sketch({{342, 330}, {270, 345}, {242, 400}, {248, 455}, {275, 485}}, {pressure=0.35})
c:sketch({{345, 345}, {285, 360}, {260, 405}, {265, 450}, {280, 475}}, {pressure=0.3})

-- Quince outline
c:sketch({
  {660, 545}, {695, 560}, {720, 600}, {715, 635},
  {685, 650}, {640, 650}, {610, 625}, {605, 590},
  {630, 555}, {660, 545}
}, {pressure=0.4})

-- Small peeled lemon & curling peel
c:sketch({
  {515, 615}, {545, 610}, {560, 630}, {550, 645},
  {520, 648}, {505, 635}, {515, 615}
}, {pressure=0.35})
-- Peel spiral trailing over the shelf edge
c:sketch({
  {535, 625}, {555, 632}, {552, 648}, {538, 665}, {542, 690}, {548, 705}
}, {pressure=0.35})

-- Shadow indications
c:sketch({{430, 620}, {540, 635}, {620, 642}}, {pressure=0.25})
c:sketch({{700, 645}, {770, 658}, {820, 665}}, {pressure=0.25})

--@ chunk 3
-- Piles for setting and underpainting
p_bg_dark = pile{{"bone black", 5}, {"raw umber", 4}, {"red earth", 1.5}, {"Prussian blue", 0.4}, medium=0.15}
p_bg_warm = pile{{"raw umber", 4}, {"yellow ochre", 3.5}, {"bone black", 1.2}, {"red earth", 1.5}, medium=0.15}
p_stone_top = pile{{"lead white", 4}, {"yellow ochre", 3}, {"raw umber", 2.5}, {"bone black", 0.6}, medium=0.12}
p_stone_front = pile{{"raw umber", 4}, {"yellow ochre", 2}, {"bone black", 2}, {"lead white", 1}, {"red earth", 0.8}, medium=0.15}
p_shadow = pile{{"raw umber", 4.5}, {"bone black", 2.5}, {"red earth", 1}, medium=0.2}

-- Geometric masks
pts_jug = {
  {380, 275}, {435, 280}, {440, 290}, {420, 310}, {415, 340}, {445, 390},
  {495, 470}, {485, 540}, {450, 600}, {445, 622}, {380, 625}, {315, 622},
  {310, 600}, {275, 540}, {265, 470}, {310, 390}, {345, 340}, {340, 310},
  {320, 290}, {325, 280}
}
m_jug = poly(pts_jug, true)

pts_handle = {
  {342, 330}, {270, 345}, {242, 400}, {248, 455}, {275, 485},
  {280, 475}, {265, 450}, {260, 405}, {285, 360}, {345, 345}
}
m_handle = poly(pts_handle, true)
m_jug_all = m_jug + m_handle

pts_quince = {
  {660, 545}, {695, 560}, {720, 600}, {715, 635},
  {685, 650}, {640, 650}, {610, 625}, {605, 590},
  {630, 555}
}
m_quince = poly(pts_quince, true)

-- Reserved foreground objects mask (shrunk slightly so background meets them cleanly)
m_objects = (m_jug_all + m_quince):shrink(3)

-- 1. Background wall: y <= 520, excluding objects
m_wall = mask(function(x, y) return y <= 522 and 1 or 0 end) - m_objects
m_wall_left = mask(function(x, y) return (y <= 522 and x < 450) and (1 - (x / 450)*0.5) or 0 end) - m_objects
m_wall_right = mask(function(x, y) return (y <= 522 and x >= 400) and 1 or 0 end) - m_objects

-- Lay in warm left background
work(m_wall_left, {hand="broad", pile=p_bg_warm, angle=0.4, coverage=1.4, fill=true})
-- Lay in deep dark right background
work(m_wall_right, {hand="broad", pile=p_bg_dark, angle=-0.3, coverage=1.6, fill=true})
-- Blend the background broadly to create soft atmospheric depth
blend(m_wall, {angle=0.1, coverage=1.2})

-- 2. Stone shelf top plane (520 <= y <= 640)
m_shelf_top = mask(function(x, y) return (y >= 520 and y <= 640) and 1 or 0 end) - m_objects
work(m_shelf_top, {hand="broad", pile=p_stone_top, angle=0.05, coverage=1.5, fill=true})

-- 3. Stone shelf front apron (640 <= y <= 800)
m_shelf_front = mask(function(x, y) return y >= 638 and 1 or 0 end)
work(m_shelf_front, {hand="broad", pile=p_stone_front, angle=0.02, coverage=1.6, fill=true})

-- Under-lip shadow on the apron (from y=638 to 655)
m_lip_shadow = mask(function(x, y) return (y >= 638 and y <= 654) and 1 or 0 end)
work(m_lip_shadow, {hand="body", pile=p_shadow, angle=0.0, coverage=1.5, fill=true})

--@ chunk 4
-- 1. Soften the background transition
p_bg_transition = pile{{"raw umber", 4}, {"bone black", 2.5}, {"yellow ochre", 2.5}, {"red earth", 1}, medium=0.3}
m_bg_mid = mask(function(x, y)
  if y > 522 or x < 240 or x > 650 then return 0 end
  return math.sin((x - 240) / (650 - 240) * math.pi)
end) - m_objects

work(m_bg_mid, {hand="broad", pile=p_bg_transition, angle=0.25, coverage=1.2, fill=true})
blend(m_wall, {angle=0.15, coverage=1.4})

-- 2. Modulate shelf top: shade toward right, plus cast shadows
p_shelf_shade = pile{{"raw umber", 3.5}, {"yellow ochre", 3}, {"bone black", 1.2}, {"lead white", 1.5}, medium=0.15}
m_shelf_right = mask(function(x, y)
  if y < 522 or y > 638 or x < 450 then return 0 end
  return math.min(1, (x - 450) / 250)
end) - m_objects
work(m_shelf_right, {hand="broad", pile=p_shelf_shade, angle=0.05, coverage=1.2, fill=true})

-- Cast shadows on the shelf top
pts_jug_shadow = {
  {420, 620}, {490, 615}, {620, 625}, {680, 638},
  {670, 642}, {520, 642}, {440, 630}
}
m_jug_shadow = poly(pts_jug_shadow, true) * mask(function(x, y) return y <= 645 and 1 or 0 end)
work(m_jug_shadow, {hand="body", pile=p_shadow, angle=0.1, coverage=1.6, fill=true, clip=true})

pts_quince_shadow = {
  {660, 642}, {720, 638}, {790, 645}, {830, 655},
  {810, 665}, {730, 662}, {670, 652}
}
m_quince_shadow = poly(pts_quince_shadow, true)
work(m_quince_shadow, {hand="body", pile=p_shadow, angle=0.15, coverage=1.6, fill=true, clip=true})

-- 3. Block in the Terracotta Jug
p_jug_lit = pile{{"yellow ochre", 4}, {"red earth", 3.2}, {"lead white", 2.2}, {"raw umber", 1}, medium=0.1}
p_jug_shadow = pile{{"raw umber", 5}, {"red earth", 3}, {"bone black", 1.5}, medium=0.15}
p_jug_core = pile{{"raw umber", 4.5}, {"bone black", 2.2}, {"red earth", 2.5}, medium=0.15}
p_interior = pile{{"bone black", 6}, {"raw umber", 4}, medium=0.1}

-- Mouth / interior dark hollow
m_jug_mouth = ellipse(380, 282, 44, 10)
work(m_jug_mouth, {hand="body", pile=p_interior, angle=0.0, coverage=1.8, fill=true, clip=true})

-- Jug lit side and shadow side
m_jug_lit = (m_jug - m_jug_mouth) * mask(function(x, y)
  if x < 370 then return 1 end
  if x > 420 then return 0 end
  return (420 - x) / 50
end)

m_jug_dark = (m_jug - m_jug_mouth) * mask(function(x, y)
  if x > 410 then return 1 end
  if x < 360 then return 0 end
  return (x - 360) / 50
end)

work(m_jug_dark, {hand="body", pile=p_jug_shadow, angle=1.45, coverage=1.5, fill=true, clip=true})
work(m_jug_lit, {hand="body", pile=p_jug_lit, angle=1.45, coverage=1.5, fill=true, clip=true})

-- Blend across the terminator on the jug body
m_jug_belly = m_jug * mask(function(x, y) return (y >= 300 and y <= 620) and 1 or 0 end)
blend(m_jug_belly, {angle=0.1, coverage=1.2, clip=true})

-- Handle
work(m_handle, {hand="body", pile=p_jug_shadow, angle=1.2, coverage=1.5, fill=true, clip=true})
m_handle_lit = m_handle * mask(function(x, y) return x < 265 and 1 or 0 end)
work(m_handle_lit, {hand="detail", pile=p_jug_lit, angle=1.2, coverage=1.4, fill=true, clip=true})

-- 4. Block in the Quince
p_quince_lit = pile{{"chrome yellow", 4.5}, {"yellow ochre", 3}, {"lead white", 2}, {"vermilion", 0.3}, medium=0.1}
p_quince_shadow = pile{{"yellow ochre", 3.5}, {"raw umber", 3}, {"green earth", 1.5}, {"bone black", 0.5}, medium=0.15}

m_quince_lit = m_quince * mask(function(x, y)
  local dx = (x - 660) / 60
  local dy = (y - 600) / 55
  local val = -0.7 * dx - 0.7 * dy
  if val > 0.1 then return 1 end
  if val < -0.3 then return 0 end
  return (val + 0.3) / 0.4
end)
m_quince_dark = m_quince - m_quince_lit

work(m_quince_dark, {hand="body", pile=p_quince_shadow, angle=0.8, coverage=1.5, fill=true, clip=true})
work(m_quince_lit, {hand="body", pile=p_quince_lit, angle=-0.6, coverage=1.5, fill=true, clip=true})
blend(m_quince, {angle=0.4, coverage=1.1, clip=true})

--@ chunk 5
-- Piles for refining and modeling
p_glaze_deep = pile{{"raw umber", 4}, {"red earth", 3}, {"bone black", 1}, medium=0.25}
p_glaze_amber = pile{{"yellow ochre", 4}, {"red earth", 3.5}, {"raw umber", 1.5}, medium=0.2}
p_jug_refl = pile{{"yellow ochre", 4}, {"red earth", 3}, {"raw umber", 2}, medium=0.18}
p_jug_light_warm = pile{{"lead white", 4.5}, {"yellow ochre", 3.5}, {"red earth", 2}, {"raw umber", 0.5}, medium=0.1}
p_rim_light = pile{{"lead white", 6}, {"yellow ochre", 3}, {"raw umber", 0.5}, medium=0.08}

-- 1. Unify background behind the jug neck and upper wall
p_bg_melt = pile{{"raw umber", 4}, {"bone black", 3}, {"yellow ochre", 1.5}, medium=0.35}
m_bg_upper = mask(function(x, y)
  if y > 460 or x < 280 or x > 600 then return 0 end
  local dx = (x - 440) / 160
  local dy = (y - 200) / 260
  if dx*dx + dy*dy > 1 then return 0 end
  return 1 - (dx*dx + dy*dy)
end) - m_objects
work(m_bg_upper, {hand="glaze", pile=p_bg_melt, angle=0.2, coverage=1.1})
blend(m_wall * mask(function(x, y) return x > 250 and x < 650 and y < 500 and 1 or 0 end), {angle=0.1, coverage=1.2})

-- 2. Model the Jug
-- A. Glaze on the neck and shoulder (from y=285 to y=430)
m_jug_neck_glaze = (m_jug - m_jug_mouth) * mask(function(x, y)
  if y < 285 or y > 430 then return 0 end
  if y < 380 then return 1 end
  return 1 - (y - 380) / 50
end)
work(m_jug_neck_glaze, {hand="body", pile=p_glaze_amber, angle=1.4, coverage=1.2, fill=true, clip=true})

-- Deep shade on the right of the neck glaze
m_jug_neck_shade = m_jug_neck_glaze * mask(function(x, y) return x > 380 and 1 or 0 end)
work(m_jug_neck_shade, {hand="body", pile=p_glaze_deep, angle=1.45, coverage=1.4, fill=true, clip=true})

-- B. Reflected light along the right flank of the jug body (x: 435 to 495, y: 440 to 600)
m_jug_refl = m_jug * mask(function(x, y)
  if x < 440 or x > 495 or y < 430 or y > 615 then return 0 end
  return (x - 440) / 55
end)
work(m_jug_refl, {hand="body", pile=p_jug_refl, angle=1.5, coverage=1.3, fill=true, clip=true})

-- C. Primary light passage on the left belly (x: 280 to 370, y: 380 to 580)
m_jug_belly_light = m_jug * mask(function(x, y)
  if x < 280 or x > 380 or y < 380 or y > 580 then return 0 end
  local cx, cy = 330, 470
  local d = math.sqrt(((x - cx)/50)^2 + ((y - cy)/90)^2)
  if d > 1 then return 0 end
  return (1 - d)
end)
work(m_jug_belly_light, {hand="body", pile=p_jug_light_warm, angle=1.35, coverage=1.4, fill=true, clip=true})

-- Blend the jug belly to integrate reflected light, core shadow, and lit flank
blend(m_jug * mask(function(x, y) return y >= 320 and y <= 620 and 1 or 0 end), {angle=0.15, coverage=1.1, clip=true})

-- D. Rim and mouth definition
b_filbert = brush("filbert", 6)
b_round = brush{kind="round", width=3, point=1, stiffness=0.5}

-- Inside shadow of the mouth lip
b_round:load(p_interior, 0.8)
b_round:stroke({
  {335, 280}, {380, 276}, {425, 280}
}, {pressure={0.3, 0.6, 0.3}, clip=m_jug})

-- Lit rim curve
b_round:reload(p_rim_light, 0.8)
b_round:stroke({
  {325, 283}, {350, 288}, {380, 290}, {410, 288}, {435, 283}
}, {pressure={0.5, 0.7, 0.8, 0.5, 0.2}, clip=m_jug})

-- E. Handle modeling: cast shadow onto the neck, highlight along the spine
m_handle_cast = m_jug * mask(function(x, y)
  if x < 290 or x > 340 or y < 350 or y > 470 then return 0 end
  return 1
end)
work(m_handle_cast, {hand="detail", pile=p_jug_shadow, angle=1.2, coverage=1.4, fill=true, clip=true})

-- Highlight along handle spine
b_round:reload(p_jug_light_warm, 0.7)
b_round:stroke({
  {338, 332}, {280, 345}, {246, 395}, {249, 440}, {272, 475}
}, {pressure={0.2, 0.6, 0.7, 0.6, 0.3}, clip=m_handle})

-- 3. Quince Development
p_quince_calyx = pile{{"raw umber", 5}, {"bone black", 3}, {"red earth", 1}, medium=0.12}
p_quince_warm_refl = pile{{"yellow ochre", 4}, {"raw umber", 2}, {"red earth", 1}, medium=0.15}
p_quince_hi = pile{{"lead white", 6}, {"chrome yellow", 3}, {"yellow ochre", 1}, medium=0.08}

-- Reflected light along lower right base
m_quince_refl = m_quince * mask(function(x, y)
  if x < 670 or y < 610 then return 0 end
  return 1
end)
work(m_quince_refl, {hand="body", pile=p_quince_warm_refl, angle=0.2, coverage=1.2, fill=true, clip=true})

-- Highlight dome on upper left of quince
m_quince_dome = m_quince * mask(function(x, y)
  local dx = (x - 645) / 25
  local dy = (y - 580) / 25
  local d = dx*dx + dy*dy
  if d > 1 then return 0 end
  return (1 - d)
end)
work(m_quince_dome, {hand="detail", pile=p_quince_hi, angle=-0.5, coverage=1.4, fill=true, clip=true})

-- Calyx / eye of the quince
b_round:reload(p_quince_calyx, 0.8)
b_round:touch(692, 618, {pressure=0.6, drag={2, 1}})
b_round:stroke({{688, 616}, {694, 620}}, {pressure={0.4, 0.2}})
b_round:stroke({{692, 614}, {693, 622}}, {pressure={0.4, 0.2}})

-- 4. Citrus fruit and curled peel on the stone shelf
p_citrus_pulp = pile{{"chrome yellow", 4}, {"yellow ochre", 2}, {"lead white", 3.5}, medium=0.15}
p_citrus_pith = pile{{"lead white", 6}, {"yellow ochre", 2}, {"raw umber", 0.5}, medium=0.1}
p_citrus_peel = pile{{"chrome yellow", 5}, {"vermilion", 0.8}, {"yellow ochre", 2}, {"lead white", 1.5}, medium=0.1}
p_peel_shadow = pile{{"raw umber", 4}, {"bone black", 2}, {"red earth", 1}, medium=0.2}

-- Halved lemon body at (525, 630)
pts_lemon = {
  {505, 630}, {515, 618}, {535, 615}, {550, 622}, {555, 635},
  {545, 646}, {525, 648}, {508, 640}
}
m_lemon = poly(pts_lemon, true)
work(m_lemon, {hand="body", pile=p_citrus_pulp, angle=0.0, coverage=1.5, fill=true, clip=true})

-- Lemon rind rim
m_lemon_rim = m_lemon:rim(3, 1)
work(m_lemon_rim, {hand="detail", pile=p_citrus_pith, angle=0.0, coverage=1.3, fill=true, clip=true})

-- Cast shadow of lemon on shelf top
m_lemon_sh = ellipse(545, 645, 20, 8) * mask(function(x, y) return y <= 642 and 1 or 0 end)
work(m_lemon_sh, {hand="detail", pile=p_shadow, angle=0.1, coverage=1.5, fill=true, clip=true})

-- Peeling ribbon curving over the shelf edge
-- Cast shadow of the trailing peel down the front apron
b_round:reload(p_peel_shadow, 0.8)
b_round:stroke({
  {552, 644}, {556, 658}, {554, 678}, {558, 698}
}, {pressure={0.5, 0.6, 0.5, 0.3}})

-- Outside of peel (rich chrome-vermilion yellow)
b_filbert:load(p_citrus_peel, 0.9)
b_filbert:stroke({
  {538, 624}, {554, 632}, {556, 646}, {544, 665}, {546, 688}, {552, 706}
}, {pressure={0.4, 0.7, 0.8, 0.7, 0.6, 0.4}})

-- Inside of peel (white pith highlight catching upper light)
b_round:reload(p_citrus_pith, 0.8)
b_round:stroke({
  {539, 625}, {551, 631}, {552, 644}, {542, 663}
}, {pressure={0.3, 0.5, 0.5, 0.2}})

--@ chunk 6
-- 1. Unify the background wall
p_bg_unify = pile{{"bone black", 4.5}, {"raw umber", 4}, {"red earth", 1.2}, medium=0.35}
m_bg_unify = mask(function(x, y)
  if y > 522 or x < 220 then return 0 end
  return math.min(1, (x - 220) / 250)
end) - m_objects

work(m_bg_unify, {hand="glaze", pile=p_bg_unify, angle=0.1, coverage=1.2})
blend(m_wall, {angle=0.1, coverage=1.4})

-- 2. Repair and model the jug shoulder / handle shadow
-- Paint over the rectangular patch on the neck/shoulder
m_jug_shoulder_fix = m_jug * mask(function(x, y)
  if x < 280 or x > 360 or y < 330 or y > 490 then return 0 end
  return 1
end)
work(m_jug_shoulder_fix, {hand="body", pile=p_glaze_amber, angle=1.35, coverage=1.6, fill=true, clip=true})
work(m_jug_shoulder_fix * mask(function(x, y) return y > 400 and 1 or 0 end),
  {hand="body", pile=p_jug_lit, angle=1.35, coverage=1.4, fill=true, clip=true})

-- Soft blend across the jug shoulder into the belly
blend(m_jug * mask(function(x, y) return (y >= 300 and y <= 500 and x >= 270 and x <= 420) and 1 or 0 end),
  {angle=0.2, coverage=1.3, clip=true})

-- Cast shadow under the arch of the handle (curved, following the handle anatomy)
b_round = brush{kind="round", width=3.5, point=1, stiffness=0.5}
b_round:load(p_jug_shadow, 0.8)
b_round:stroke({
  {340, 345}, {310, 360}, {285, 395}, {290, 440}, {305, 470}
}, {pressure={0.3, 0.6, 0.7, 0.6, 0.2}, clip=m_jug})

-- 3. Repair and model the quince
-- Re-model the whole quince shadow and reflected light naturally
m_quince_shadow_smooth = m_quince * mask(function(x, y)
  local dx = (x - 660) / 60
  local dy = (y - 600) / 55
  local val = 0.7 * dx + 0.7 * dy
  if val < -0.1 then return 0 end
  return math.min(1, (val + 0.1) / 0.5)
end)
work(m_quince_shadow_smooth, {hand="body", pile=p_quince_shadow, angle=0.8, coverage=1.5, fill=true, clip=true})

-- Reflected bounce light along the bottom-right curve
m_quince_bounce = m_quince * mask(function(x, y)
  local dx = (x - 675) / 45
  local dy = (y - 625) / 30
  local d = dx*dx + dy*dy
  if d > 1 or dx < 0 or dy < 0 then return 0 end
  return (1 - d)
end)
work(m_quince_bounce, {hand="body", pile=p_quince_warm_refl, angle=0.3, coverage=1.3, fill=true, clip=true})

-- Re-establish the calyx eye of the quince
b_round:reload(p_quince_calyx, 0.85)
b_round:touch(692, 618, {pressure=0.7, drag={1.5, 0.8}})
b_round:stroke({{689, 616}, {695, 620}}, {pressure={0.4, 0.2}})
b_round:stroke({{693, 614}, {692, 622}}, {pressure={0.4, 0.2}})

-- Blend quince shadow area softly
blend(m_quince * mask(function(x, y) return (x > 630 or y > 590) and 1 or 0 end),
  {angle=0.3, coverage=1.2, clip=true})

-- 4. Modulate the stone shelf and cast shadow edges
-- Soften the jug cast shadow edge
blend(m_jug_shadow:rim(6, 2), {angle=0.1, coverage=1.2})
blend(m_quince_shadow:rim(6, 2), {angle=0.1, coverage=1.2})

-- Ledge front edge definition (a subtle crisp highlight along the front lip where light catches)
p_ledge_highlight = pile{{"lead white", 5}, {"yellow ochre", 3}, {"raw umber", 1}, medium=0.08}
b_filbert = brush("filbert", 4)
b_filbert:load(p_ledge_highlight, 0.8)
b_filbert:stroke({
  {0, 638}, {250, 638}, {500, 638}, {750, 638}, {1000, 638}
}, {pressure={0.2, 0.5, 0.6, 0.4, 0.2}, clip=mask(function(x, y) return (x > 530 and x < 560) and 0 or 1 end)})

--@ chunk 7
-- Advance clock by 3 hours so the underpainting reaches setting/tacky state
local t = wait(180)
print("Time after drying wait: " .. tostring(t))

-- Palette piles for unified modeling
p_jug_body = pile{{"red earth", 4.5}, {"yellow ochre", 3.5}, {"lead white", 2.2}, {"raw umber", 1}, medium=0.1}
p_jug_light = pile{{"lead white", 5}, {"yellow ochre", 4}, {"red earth", 2.2}, {"raw umber", 0.5}, medium=0.08}
p_jug_core = pile{{"raw umber", 5}, {"bone black", 2.5}, {"red earth", 2.5}, medium=0.12}
p_jug_bounce = pile{{"yellow ochre", 4}, {"red earth", 3.5}, {"raw umber", 1.8}, medium=0.14}
p_glaze_amber = pile{{"yellow ochre", 4}, {"red earth", 3.5}, {"raw umber", 2}, medium=0.18}
p_glaze_shade = pile{{"raw umber", 5}, {"bone black", 3}, {"red earth", 1.5}, medium=0.15}
p_specular = pile{{"lead white", 8}, {"yellow ochre", 1.5}, {"raw umber", 0.2}, medium=0.05}

-- 1. UNIFIED MODELING OF THE TERRACOTTA JUG
-- A. Lay base terracotta body tone over the entire jug belly and base
m_jug_belly_all = (m_jug - m_jug_mouth) * mask(function(x, y) return math.max(0, math.min(1, (y - 320) / 40)) end)
work(m_jug_belly_all, {hand="body", pile=p_jug_body, angle=1.4, coverage=1.6, fill=true, clip=true})

-- B. Core shadow on the right side of the belly (smooth continuous ramp)
m_jug_core_smooth = m_jug_belly_all * mask(function(x, y)
  return math.max(0, 1 - math.abs(x - 425) / 50)
end)
work(m_jug_core_smooth, {hand="body", pile=p_jug_core, angle=1.45, coverage=1.5, fill=true, clip=true})

-- C. Reflected warm light along the right contour
m_jug_bounce_smooth = m_jug_belly_all * mask(function(x, y)
  return math.max(0, math.min(1, (x - 425) / 50))
end)
work(m_jug_bounce_smooth, {hand="body", pile=p_jug_bounce, angle=1.45, coverage=1.4, fill=true, clip=true})

-- D. Lit flank on the left side of the belly
m_jug_lit_smooth = m_jug_belly_all * mask(function(x, y)
  return math.max(0, math.min(1, (400 - x) / 70))
end)
work(m_jug_lit_smooth, {hand="body", pile=p_jug_light, angle=1.35, coverage=1.5, fill=true, clip=true})

-- E. Model the glazed upper neck and shoulder
m_jug_neck_smooth = (m_jug - m_jug_mouth) * mask(function(x, y)
  return math.max(0, math.min(1, (410 - y) / 60))
end)
work(m_jug_neck_smooth, {hand="body", pile=p_glaze_amber, angle=1.35, coverage=1.5, fill=true, clip=true})

-- Shadow on right of neck glaze
m_jug_neck_sh = m_jug_neck_smooth * mask(function(x, y)
  return math.max(0, math.min(1, (x - 375) / 45))
end)
work(m_jug_neck_sh, {hand="body", pile=p_glaze_shade, angle=1.45, coverage=1.4, fill=true, clip=true})

-- F. Soft blend across the whole jug body to fuse all modeling into smooth ceramic relief
blend(m_jug - m_jug_mouth, {angle=0.15, coverage=1.2, clip=true})

-- G. Rim and mouth depth
b_round = brush{kind="round", width=3, point=1, stiffness=0.5}
b_fine = brush{kind="round", width=1.8, point=1, stiffness=0.6}

-- Deep mouth interior
b_round:load(p_interior, 0.9)
b_round:stroke({
  {335, 281}, {380, 277}, {425, 281}
}, {pressure={0.4, 0.7, 0.4}, clip=m_jug})

-- Flared lip highlight
b_round:reload(p_specular, 0.8)
b_round:stroke({
  {324, 284}, {348, 289}, {380, 290}, {408, 289}, {432, 284}
}, {pressure={0.4, 0.7, 0.75, 0.5, 0.2}, clip=m_jug})

-- Glaze specular highlight on shoulder curve
b_round:reload(p_specular, 0.85)
b_round:stroke({
  {332, 345}, {337, 360}, {342, 375}
}, {pressure={0.3, 0.8, 0.3}, clip=m_jug})

-- Belly specular touch
b_round:touch(320, 465, {pressure=0.7, drag={2, 4}, clip=m_jug})

-- H. Handle refinement: unify the handle, soft shadow behind it
work(m_handle, {hand="body", pile=p_jug_body, angle=1.2, coverage=1.4, fill=true, clip=true})
b_round:reload(p_jug_shadow, 0.8)
b_round:stroke({
  {342, 345}, {305, 365}, {280, 400}, {285, 445}, {300, 475}
}, {pressure={0.2, 0.5, 0.6, 0.5, 0.2}, clip=m_jug})

b_fine:load(p_jug_light, 0.8)
b_fine:stroke({
  {338, 332}, {278, 345}, {245, 395}, {248, 440}, {270, 475}
}, {pressure={0.3, 0.6, 0.7, 0.5, 0.3}, clip=m_handle})

-- 2. REPAIR AND REFINE QUINCE
p_quince_sh_opaque = pile{{"yellow ochre", 4}, {"raw umber", 3.5}, {"green earth", 2}, {"bone black", 0.8}, medium=0.1}
m_quince_lower_sh = m_quince * mask(function(x, y)
  local dx = (x - 660) / 60
  local dy = (y - 600) / 55
  local val = 0.6 * dx + 0.8 * dy
  if val < 0.1 then return 0 end
  return math.min(1, (val - 0.1) / 0.4)
end)
work(m_quince_lower_sh, {hand="body", pile=p_quince_sh_opaque, angle=0.8, coverage=1.6, fill=true, clip=true})

-- Quince bounce light
work(m_quince_bounce, {hand="body", pile=p_quince_warm_refl, angle=0.3, coverage=1.4, fill=true, clip=true})
blend(m_quince * mask(function(x, y) return y > 585 and 1 or 0 end), {angle=0.2, coverage=1.1, clip=true})

-- Crisp calyx star
b_fine:load(p_quince_calyx, 0.9)
b_fine:touch(692, 618, {pressure=0.8, drag={1, 1}})
b_fine:stroke({{688, 616}, {695, 620}}, {pressure={0.5, 0.3}})
b_fine:stroke({{692, 614}, {692, 622}}, {pressure={0.5, 0.3}})

-- 3. REPAIR LEMON SLICE
work(m_lemon, {hand="body", pile=p_citrus_pulp, angle=0.0, coverage=1.6, fill=true, clip=true})
work(m_lemon_rim, {hand="detail", pile=p_citrus_pith, angle=0.0, coverage=1.5, fill=true, clip=true})

-- Radial segment lines of lemon pulp
p_segment_line = pile{{"lead white", 6}, {"yellow ochre", 3}, medium=0.08}
b_fine:load(p_segment_line, 0.7)
b_fine:stroke({{527, 631}, {512, 625}}, {pressure={0.4, 0.2}, clip=m_lemon})
b_fine:stroke({{527, 631}, {524, 618}}, {pressure={0.4, 0.2}, clip=m_lemon})
b_fine:stroke({{527, 631}, {540, 621}}, {pressure={0.4, 0.2}, clip=m_lemon})
b_fine:stroke({{527, 631}, {548, 634}}, {pressure={0.4, 0.2}, clip=m_lemon})
b_fine:stroke({{527, 631}, {538, 644}}, {pressure={0.4, 0.2}, clip=m_lemon})
b_fine:stroke({{527, 631}, {518, 643}}, {pressure={0.4, 0.2}, clip=m_lemon})

-- Juicy pulp glistening highlight
p_pulp_glint = pile{{"lead white", 8}, {"chrome yellow", 1.5}, medium=0.05}
b_fine:reload(p_pulp_glint, 0.8)
b_fine:touch(522, 626, {pressure=0.6})
b_fine:touch(534, 624, {pressure=0.5})
b_fine:touch(525, 638, {pressure=0.5})

--@ chunk 8
-- 1. HANDLE HOLE & NEGATIVE SPACE
-- Background wall tone at x~300, y~400 is warm ochre-umber
p_bg_handle = pile{{"raw umber", 4}, {"yellow ochre", 3}, {"bone black", 1.8}, {"red earth", 1.2}, medium=0.15}
pts_handle_inner = {
  {334, 352}, {295, 360}, {270, 390}, {266, 420}, {274, 452},
  {286, 465}, {298, 440}, {312, 400}, {326, 370}
}
m_handle_inner = poly(pts_handle_inner, true)
work(m_handle_inner, {hand="body", pile=p_bg_handle, angle=0.4, coverage=1.8, fill=true, clip=true})

-- Re-establish the handle arch
pts_handle_spine = {
  {338, 332}, {280, 345}, {246, 395}, {249, 440}, {272, 475}
}
b_filbert = brush("filbert", 6)
b_round = brush{kind="round", width=3, point=1, stiffness=0.6}
b_fine = brush{kind="round", width=1.8, point=1, stiffness=0.7}

-- Body of the handle
b_filbert:load(p_jug_body, 0.85)
b_filbert:stroke(pts_handle_spine, {pressure={0.4, 0.7, 0.8, 0.7, 0.4}, clip=m_handle})

-- Handle shadow along the inner curve
b_round:load(p_jug_shadow, 0.8)
b_round:stroke({
  {328, 354}, {290, 362}, {268, 395}, {270, 435}, {280, 460}
}, {pressure={0.3, 0.6, 0.7, 0.6, 0.3}, clip=m_handle})

-- Highlight along handle outer edge
b_fine:load(p_jug_light, 0.85)
b_fine:stroke({
  {336, 334}, {276, 346}, {244, 395}, {247, 435}, {268, 472}
}, {pressure={0.3, 0.6, 0.8, 0.6, 0.3}, clip=m_handle})

-- 2. COMPLETE UNIFICATION OF QUINCE LOWER SHADOW
p_quince_lower_opaque = pile{{"yellow ochre", 4}, {"raw umber", 4}, {"green earth", 2}, {"lead white", 1.2}, {"bone black", 0.6}, medium=0.08}
m_quince_lower = m_quince * mask(function(x, y)
  return math.max(0, math.min(1, (y - 565) / 35))
end)
work(m_quince_lower, {hand="body", pile=p_quince_lower_opaque, angle=0.8, coverage=1.8, fill=true, clip=true})

-- Soft blend across the whole quince to merge light dome and shadow
blend(m_quince, {angle=0.25, coverage=1.1, clip=true})

-- Subtle reflected light along bottom right curve
b_round:load(p_quince_warm_refl, 0.7)
b_round:stroke({
  {660, 646}, {690, 642}, {714, 626}
}, {pressure={0.2, 0.6, 0.4}, clip=m_quince})

-- Crisp calyx star
b_fine:load(p_quince_calyx, 0.9)
b_fine:touch(692, 618, {pressure=0.8, drag={1, 1}})
b_fine:stroke({{688, 616}, {695, 620}}, {pressure={0.5, 0.3}})
b_fine:stroke({{692, 614}, {692, 622}}, {pressure={0.5, 0.3}})

-- 3. SPECULAR IMPASTO HIGHLIGHTS
-- Glazed shoulder specular highlight
p_specular = pile{{"lead white", 8.5}, {"yellow ochre", 1.2}, {"raw umber", 0.2}, medium=0.04}
b_round:load(p_specular, 0.9)
b_round:stroke({
  {334, 345}, {338, 358}, {342, 372}
}, {pressure={0.3, 0.85, 0.3}, clip=m_jug})

-- Flared lip highlight
b_fine:load(p_specular, 0.85)
b_fine:stroke({
  {330, 285}, {352, 289}, {378, 290}, {405, 289}
}, {pressure={0.3, 0.7, 0.75, 0.4}, clip=m_jug})

-- Quince impasto sun glint
b_round:reload(p_specular, 0.85)
b_round:touch(642, 565, {pressure=0.75, drag={1.5, 1}, clip=m_quince})

-- Lemon peel edge highlight catching direct light
b_fine:reload(p_specular, 0.8)
b_fine:stroke({
  {548, 630}, {556, 636}, {554, 648}
}, {pressure={0.3, 0.7, 0.4}})

-- 4. CONTACT SHADOWS (bedding objects firmly onto the stone surface)
p_contact = pile{{"bone black", 6}, {"raw umber", 4}, medium=0.08}
b_round:load(p_contact, 0.9)

-- Jug base contact shadow
b_round:stroke({
  {315, 622}, {350, 623}, {385, 624}, {415, 623}, {445, 622}
}, {pressure={0.4, 0.8, 0.9, 0.8, 0.4}})

-- Quince base contact shadow
b_round:stroke({
  {630, 648}, {655, 650}, {680, 650}
}, {pressure={0.3, 0.7, 0.4}})

-- Lemon contact shadow
b_fine:load(p_contact, 0.8)
b_fine:stroke({
  {515, 646}, {532, 647}, {546, 645}
}, {pressure={0.3, 0.6, 0.3}})

--@ chunk 9
-- Advance time so previous layers set firmly
local t = wait(120)
print("Time at start of refinement chunk: " .. tostring(t))

-- 1. RESTORE RADIANT GOLDEN LIGHT TO QUINCE
p_quince_gold = pile{{"chrome yellow", 5.5}, {"yellow ochre", 3}, {"lead white", 2.5}, {"vermilion", 0.4}, medium=0.06}
p_quince_mid = pile{{"yellow ochre", 4.5}, {"chrome yellow", 3}, {"raw umber", 1.5}, {"red earth", 0.8}, medium=0.1}
p_quince_impasto = pile{{"lead white", 8}, {"chrome yellow", 2.5}, {"yellow ochre", 0.8}, medium=0.04}
p_calyx = pile{{"raw umber", 5}, {"bone black", 3.5}, {"red earth", 1}, medium=0.08}

-- Golden upper dome (opaque, buttery body paint)
m_quince_top = m_quince * mask(function(x, y)
  return math.max(0, math.min(1, (620 - y) / 50))
end)
work(m_quince_top, {hand="body", pile=p_quince_gold, angle=-0.5, coverage=1.7, fill=true, clip=true})

-- Warm intermediate half-tone across the equator (y: 590 to 625)
m_quince_equator = m_quince * mask(function(x, y)
  if y < 585 or y > 630 then return 0 end
  return math.sin((y - 585) / 45 * math.pi)
end)
work(m_quince_equator, {hand="body", pile=p_quince_mid, angle=0.2, coverage=1.3, fill=true, clip=true})

-- Blend ONLY along the equator to merge dome into shadow without muddying the dome
blend(m_quince_equator, {angle=0.15, coverage=1.0, clip=true})

-- Crisp impasto sun highlight on upper-left curve of quince
b_round = brush{kind="round", width=3, point=1, stiffness=0.6}
b_fine = brush{kind="round", width=1.6, point=1, stiffness=0.7}

b_round:load(p_quince_impasto, 0.9)
b_round:touch(644, 566, {pressure=0.8, drag={1.5, 0.8}, clip=m_quince})
b_round:touch(652, 560, {pressure=0.6, drag={1.0, 0.5}, clip=m_quince})

-- Dried blossom calyx cluster (star at blossom eye)
b_fine:load(p_calyx, 0.9)
b_fine:touch(692, 618, {pressure=0.8})
b_fine:stroke({{692, 618}, {686, 615}}, {pressure={0.6, 0.2}})
b_fine:stroke({{692, 618}, {698, 615}}, {pressure={0.6, 0.2}})
b_fine:stroke({{692, 618}, {695, 624}}, {pressure={0.6, 0.2}})
b_fine:stroke({{692, 618}, {688, 623}}, {pressure={0.6, 0.2}})
b_fine:stroke({{692, 618}, {691, 612}}, {pressure={0.6, 0.2}})

-- 2. REFINE HANDLE AND JUG CONTOURS
-- Soften inner handle hole boundary with background
p_bg_warm_local = pile{{"raw umber", 4}, {"yellow ochre", 3}, {"bone black", 1.5}, medium=0.2}
m_handle_hole_blend = poly(pts_handle_inner, true):rim(4, 2)
blend(m_handle_hole_blend, {angle=0.3, coverage=1.2})

-- Clay joints where handle meets neck and shoulder
p_clay_body = pile{{"red earth", 4}, {"yellow ochre", 3.5}, {"raw umber", 2}, {"lead white", 1.8}, medium=0.1}
b_filbert = brush("filbert", 5)

-- Top joint (neck attachment)
b_filbert:load(p_clay_body, 0.8)
b_filbert:stroke({{325, 332}, {340, 334}, {345, 342}}, {pressure={0.4, 0.7, 0.4}, clip=m_jug})

-- Bottom joint (shoulder attachment)
b_filbert:stroke({{265, 465}, {276, 480}, {285, 485}}, {pressure={0.4, 0.7, 0.4}, clip=m_jug})

-- Handle crest catch light
b_fine:load(p_specular, 0.85)
b_fine:stroke({
  {334, 333}, {278, 346}, {246, 395}, {248, 435}, {268, 470}
}, {pressure={0.3, 0.7, 0.8, 0.6, 0.2}, clip=m_handle})

-- Jug mouth deep throat touch-up (cover any light rim line on inside back)
b_round:load(p_interior, 0.95)
b_round:stroke({{345, 278}, {380, 275}, {415, 278}}, {pressure={0.5, 0.8, 0.5}, clip=m_jug})

-- Front lip crisp rim highlight
b_fine:load(p_specular, 0.9)
b_fine:stroke({{330, 285}, {355, 289}, {380, 290}, {405, 289}}, {pressure={0.4, 0.8, 0.85, 0.5}, clip=m_jug})

-- Glazed shoulder specular highlight touch
b_round:load(p_specular, 0.9)
b_round:stroke({{335, 350}, {338, 360}, {341, 370}}, {pressure={0.4, 0.9, 0.4}, clip=m_jug})

-- 3. LEMON & STONE LEDGE REFINEMENTS
-- Juicy highlights on lemon pulp
p_lemon_glint = pile{{"lead white", 8.5}, {"chrome yellow", 1.2}, medium=0.04}
b_fine:load(p_lemon_glint, 0.85)
b_fine:touch(523, 627, {pressure=0.7})
b_fine:touch(535, 624, {pressure=0.6})
b_fine:touch(528, 638, {pressure=0.6})

-- Trailing peel: enhance the rich yellow zest
p_peel_zest = pile{{"chrome yellow", 5}, {"vermilion", 0.7}, {"lead white", 1.5}, medium=0.08}
b_fine:load(p_peel_zest, 0.85)
b_fine:stroke({
  {540, 626}, {554, 634}, {556, 646}, {544, 665}, {547, 688}, {552, 706}
}, {pressure={0.4, 0.8, 0.85, 0.7, 0.6, 0.3}})

-- White pith highlight on inner curl of peel
b_fine:load(p_specular, 0.8)
b_fine:stroke({{552, 632}, {553, 644}, {543, 662}}, {pressure={0.3, 0.7, 0.3}})

-- Deep shadow line right under stone ledge overhang (y: 639 to 644)
p_lip_deep = pile{{"bone black", 5}, {"raw umber", 4}, medium=0.1}
b_filbert:load(p_lip_deep, 0.8)
b_filbert:stroke({
  {0, 641}, {250, 641}, {500, 641}, {750, 641}, {1000, 641}
}, {pressure={0.3, 0.7, 0.8, 0.7, 0.3}, clip=mask(function(x, y) return (x > 500 and x < 560) and 0 or 1 end)})

--@ chunk 10
-- Advance time so previous layers are set
local t = wait(120)
print("Canvas clock at chunk 10: " .. tostring(t))

-- 1. RADIANT SOLID QUINCE
p_quince_sun = pile{{"chrome yellow", 6.5}, {"lead white", 4.5}, {"yellow ochre", 2}, {"vermilion", 0.3}, medium=0.04}
p_quince_sh_solid = pile{{"yellow ochre", 4}, {"raw umber", 3.2}, {"green earth", 1.8}, {"lead white", 1.5}, medium=0.06}
p_quince_hi_pure = pile{{"lead white", 9}, {"chrome yellow", 2}, {"yellow ochre", 0.5}, medium=0.03}
p_calyx = pile{{"raw umber", 5}, {"bone black", 4}, {"red earth", 1}, medium=0.08}

-- Geometric split of quince into light and shade
m_quince_lit_clean = m_quince * mask(function(x, y)
  local dx = (x - 660) / 60
  local dy = (y - 600) / 55
  local val = -0.7 * dx - 0.7 * dy
  if val > 0.05 then return 1 end
  if val < -0.15 then return 0 end
  return (val + 0.15) / 0.2
end)
m_quince_dark_clean = m_quince - m_quince_lit_clean

-- Lay opaque body color
work(m_quince_dark_clean, {hand="body", pile=p_quince_sh_solid, angle=0.8, coverage=2.2, fill=true, clip=true})
work(m_quince_lit_clean, {hand="body", pile=p_quince_sun, angle=-0.6, coverage=2.5, fill=true, clip=true})

-- Blend narrowly along the terminator zone
m_quince_seam = m_quince * mask(function(x, y)
  local dx = (x - 660) / 60
  local dy = (y - 600) / 55
  local val = math.abs(-0.7 * dx - 0.7 * dy)
  if val > 0.2 then return 0 end
  return 1 - val / 0.2
end)
blend(m_quince_seam, {angle=0.2, coverage=0.8, clip=true})

-- Quince impasto crest highlight
b_round = brush{kind="round", width=3, point=1, stiffness=0.7}
b_fine = brush{kind="round", width=1.6, point=1, stiffness=0.7}

b_round:load(p_quince_hi_pure, 0.95)
b_round:touch(644, 564, {pressure=0.85, drag={1.5, 0.8}, clip=m_quince})

-- Delicate dried calyx star
b_fine:load(p_calyx, 0.95)
b_fine:touch(692, 618, {pressure=0.85})
b_fine:stroke({{692, 618}, {686, 615}}, {pressure={0.6, 0.2}})
b_fine:stroke({{692, 618}, {698, 615}}, {pressure={0.6, 0.2}})
b_fine:stroke({{692, 618}, {695, 624}}, {pressure={0.6, 0.2}})
b_fine:stroke({{692, 618}, {688, 623}}, {pressure={0.6, 0.2}})
b_fine:stroke({{692, 618}, {691, 612}}, {pressure={0.6, 0.2}})

-- 2. JUG FORM CREST & TACTILE CLAY HIGHLIGHT
p_belly_swell = pile{{"lead white", 5.5}, {"yellow ochre", 4}, {"red earth", 2.2}, {"raw umber", 0.4}, medium=0.06}
b_filbert = brush("filbert", 6)
b_filbert:load(p_belly_swell, 0.85)
b_filbert:stroke({
  {315, 450}, {326, 470}, {330, 490}
}, {pressure={0.3, 0.75, 0.4}, clip=m_jug})

-- 3. FINAL CONTACT SHADOWS & ANCHORING
p_bedding = pile{{"bone black", 6.5}, {"raw umber", 3.5}, medium=0.08}
b_round:load(p_bedding, 0.9)
b_round:stroke({{315, 622}, {380, 624}, {445, 622}}, {pressure={0.5, 0.85, 0.5}})
b_round:stroke({{632, 649}, {660, 651}, {685, 650}}, {pressure={0.4, 0.8, 0.4}})
