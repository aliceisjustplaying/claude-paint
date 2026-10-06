-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{
  size = 710,
  aspect = 1.42,
  linen = {15, 13},
  seed = 1818,
  ground = {
    {
      pile = {{"yellow ochre", 5}, {"red earth", 2}, {"lead white", 5}, {"raw umber", 1}},
      um = 70,
      apply = "knife",
      texture = 0.22
    },
    {
      pile = {{"lead white", 20}, {"yellow ochre", 1}, {"raw umber", 0.5}},
      um = 35,
      apply = "brush"
    }
  }
}
print("Canvas initialized. W = " .. W .. ", H = " .. H)

--@ chunk 2
local h = pencil("2H")
local b = pencil("HB")
local c = chalk()

-- 1. Ruled horizon line for the distant water
h:rule({50, 468}, {950, 468}, {pressure=0.25})

-- 2. Distant low hills/headlands across the water
h:sketch({
  {50, 468}, {150, 464}, {260, 459}, {360, 461}, {460, 457},
  {570, 460}, {690, 456}, {810, 462}, {950, 468}
}, {pressure=0.3, passes=2})

-- 3. Foreground knoll crest and terrain contours
h:sketch({
  {0, 488}, {120, 478}, {230, 462}, {320, 447}, {380, 441},
  {440, 443}, {530, 458}, {630, 466}, {740, 456}, {850, 463}, {1000, 478}
}, {pressure=0.4, passes=2})

-- Secondary terrain contour lines in foreground
h:sketch({{0, 530}, {180, 520}, {350, 505}, {540, 525}, {750, 515}, {1000, 540}}, {pressure=0.25, passes=1})
h:sketch({{0, 610}, {250, 595}, {500, 580}, {720, 600}, {1000, 620}}, {pressure=0.25, passes=1})

-- 4. Megalithic Tomb (Dolmen) outline
-- Capstone
b:line({
  {325, 428}, {335, 420}, {365, 415}, {405, 413}, {445, 416}, {470, 422},
  {472, 431}, {455, 434}, {415, 430}, {375, 431}, {338, 434}, {325, 428}
}, {pressure=0.6, smooth=false})

-- Support orthostats (uprights)
-- Left orthostat
b:line({{342, 432}, {338, 462}, {358, 463}, {360, 431}}, {pressure=0.55, smooth=false})
-- Center-rear orthostat
b:line({{388, 429}, {386, 457}, {406, 458}, {407, 429}}, {pressure=0.5, smooth=false})
-- Right orthostat
b:line({{434, 431}, {431, 459}, {452, 458}, {450, 432}}, {pressure=0.55, smooth=false})

-- Shadow under capstone
c:sketch({{362, 433}, {385, 433}, {385, 456}, {360, 458}}, {pressure=0.4, passes=2})
c:sketch({{408, 432}, {430, 433}, {429, 456}, {407, 456}}, {pressure=0.4, passes=2})

-- Surrounding erratic boulders
-- Leaning stone left of dolmen
b:line({{305, 456}, {320, 442}, {334, 445}, {328, 464}, {305, 456}}, {pressure=0.5, smooth=false})
-- Boulder right of dolmen
b:line({{475, 452}, {495, 442}, {515, 446}, {520, 458}, {490, 464}, {475, 452}}, {pressure=0.5, smooth=false})
-- Mid-foreground erratic boulder left
b:line({{210, 532}, {230, 515}, {265, 518}, {275, 538}, {235, 545}, {210, 532}}, {pressure=0.5, smooth=false})
-- Mid-foreground boulder cluster right
b:line({{645, 540}, {670, 526}, {705, 530}, {715, 548}, {665, 555}, {645, 540}}, {pressure=0.45, smooth=false})

-- 5. The Great Stag-Headed Oak (left of dolmen, trunk base at x ≈ 280)
-- Trunk roots and trunk contours
b:line({{252, 465}, {262, 452}, {268, 420}, {270, 385}, {265, 350}, {255, 320}, {248, 295}}, {pressure=0.6, smooth=true})
b:line({{315, 458}, {308, 440}, {302, 405}, {298, 365}, {295, 335}}, {pressure=0.6, smooth=true})

-- Hollow in the trunk
b:line({{278, 435}, {290, 430}, {292, 400}, {282, 395}, {276, 420}, {278, 435}}, {pressure=0.5, smooth=false})

-- Great eastern limb stretching over the dolmen
b:line({
  {297, 345}, {325, 335}, {360, 322}, {405, 318}, {445, 315}, {480, 298}, {505, 280}
}, {pressure=0.55, smooth=true})
b:line({
  {296, 358}, {330, 348}, {370, 335}, {410, 330}, {445, 326}, {475, 310}, {500, 290}
}, {pressure=0.5, smooth=true})

-- Branches off the eastern limb
b:line({{365, 322}, {380, 290}, {400, 268}, {415, 250}}, {pressure=0.45, smooth=true})
b:line({{380, 290}, {370, 265}, {360, 245}}, {pressure=0.4, smooth=true})
b:line({{435, 318}, {452, 290}, {475, 270}}, {pressure=0.4, smooth=true})
b:line({{470, 305}, {490, 325}, {510, 345}}, {pressure=0.4, smooth=true})

-- Western limb reaching left
b:line({{265, 345}, {240, 330}, {210, 310}, {180, 292}, {150, 280}, {125, 275}}, {pressure=0.5, smooth=true})
b:line({{220, 318}, {205, 280}, {190, 255}, {170, 235}}, {pressure=0.45, smooth=true})
b:line({{180, 292}, {165, 315}, {145, 330}}, {pressure=0.4, smooth=true})

-- Central trunk and stag-headed dead crown
b:line({{270, 325}, {272, 285}, {270, 245}, {266, 205}, {262, 168}, {258, 140}}, {pressure=0.5, smooth=true})
b:line({{270, 245}, {288, 215}, {305, 185}, {315, 155}}, {pressure=0.45, smooth=true})
b:line({{288, 215}, {278, 185}, {275, 158}}, {pressure=0.4, smooth=true})
b:line({{272, 285}, {295, 260}, {320, 235}, {338, 210}, {350, 185}}, {pressure=0.45, smooth=true})
b:line({{252, 295}, {235, 265}, {220, 238}, {210, 210}}, {pressure=0.4, smooth=true})

-- 6. Secondary Tree (right knoll, x ≈ 780)
b:line({{770, 465}, {772, 430}, {768, 390}, {762, 350}, {755, 310}, {745, 270}, {738, 240}}, {pressure=0.5, smooth=true})
b:line({{785, 463}, {782, 430}, {778, 390}, {772, 355}}, {pressure=0.45, smooth=true})
-- Branches
b:line({{768, 385}, {745, 360}, {720, 335}, {695, 318}, {675, 305}}, {pressure=0.45, smooth=true})
b:line({{772, 375}, {795, 355}, {825, 330}, {855, 310}, {880, 295}}, {pressure=0.45, smooth=true})
b:line({{760, 330}, {775, 300}, {790, 275}, {805, 250}}, {pressure=0.4, smooth=true})
b:line({{745, 270}, {730, 240}, {722, 215}}, {pressure=0.4, smooth=true})

print("Underdrawing completed.")

--@ chunk 3
local b = pencil("HB")
local h = pencil("2H")

-- Hatch shadow under the dolmen capstone (the chamber recess)
local m_dolmen_chamber = poly({
  {344, 432}, {448, 432}, {446, 458}, {342, 458}
})
b:hatch(m_dolmen_chamber, {angle=-0.8, pressure=0.45, spacing=1.2, length=12})

-- Hatch the hollow in the ancient oak trunk
local m_tree_hollow = poly({
  {278, 435}, {290, 430}, {292, 400}, {282, 395}, {276, 420}
})
b:hatch(m_tree_hollow, {angle=1.2, pressure=0.5, spacing=1.0, length=10})

-- Shade under the capstone overhang
local m_capstone_under = poly({
  {325, 428}, {472, 431}, {465, 435}, {330, 435}
})
b:hatch(m_capstone_under, {angle=0.1, pressure=0.4, spacing=1.5, length=8})

-- Fix the drawing
fix()

print("Shading hatched and drawing fixed.")

--@ chunk 4
-- Mix the sky piles
p_zenith = pile{{"lead white", 8}, {"smalt", 6}, {"cobalt blue", 2.5}, {"raw umber", 0.6}, {"bone black", 0.2}, medium=0.35}
p_mid_sky = pile{{"lead white", 14}, {"smalt", 4}, {"cobalt blue", 1.2}, {"yellow ochre", 0.6}, {"raw umber", 0.3}, medium=0.35}
p_trans = pile{{"lead white", 16}, {"yellow ochre", 2.5}, {"smalt", 1}, {"raw umber", 0.3}, {"vermilion", 0.15}, medium=0.4}
p_glow = pile{{"lead white", 14}, {"chrome yellow", 2.2}, {"yellow ochre", 2.5}, {"vermilion", 0.35}, medium=0.35}

local m_sky = rect(0, 0, 1000, 485)

-- Layer 1: Zenith band
local m_zenith = rect(0, 0, 1000, 180):soften(30)
work(m_zenith, {hand="glaze", pile=p_zenith, angle=0, order="across", coverage=1.2, fill=true})

-- Layer 2: Mid-sky band
local m_mid = rect(0, 120, 1000, 170):soften(30)
work(m_mid, {hand="glaze", pile=p_mid_sky, angle=0, order="across", coverage=1.2, fill=true})

-- Layer 3: Transition band
local m_lower_mid = rect(0, 240, 1000, 150):soften(30)
work(m_lower_mid, {hand="glaze", pile=p_trans, angle=0, order="across", coverage=1.2, fill=true})

-- Layer 4: Horizon glow band
local m_glow = rect(0, 340, 1000, 145):soften(25)
work(m_glow, {hand="glaze", pile=p_glow, angle=0, order="across", coverage=1.3, fill=true})

-- Blend the wet sky to achieve Friedrich's smooth, gossamer twilight gradation
blend(m_sky, {angle=0})

print("Sky lay-in painted and blended.")

--@ chunk 5
print("Drying state mid sky: " .. drying(500, 200))
print("Drying state horizon: " .. drying(500, 420))

--@ chunk 6
local p_sky_veil = pile{{"lead white", 32}, {"yellow ochre", 1.8}, {"chrome yellow", 0.5}, {"raw umber", 0.3}, {"vermilion", 0.15}, medium=0.5}
local p_sky_cool = pile{{"lead white", 26}, {"smalt", 4}, {"cobalt blue", 1.2}, {"raw umber", 0.5}, {"yellow ochre", 0.3}, medium=0.5}
local p_zenith_deep = pile{{"lead white", 12}, {"smalt", 7}, {"cobalt blue", 3}, {"raw umber", 0.8}, {"bone black", 0.3}, medium=0.4}

local m_sky_wide = rect(0, 0, 1000, 510)

-- Deepen and cool the upper sky (y = 0 to 140)
local m_upper = rect(0, 0, 1000, 140):soften(40)
work(m_upper, {hand="glaze", pile=p_zenith_deep, angle=0, coverage=1.0, fill=true})

-- Cool silvery middle transition (y = 100 to 280)
local m_mid_veil = rect(0, 100, 1000, 200):soften(50)
work(m_mid_veil, {hand="glaze", pile=p_sky_cool, angle=0, coverage=1.1, fill=true})

-- Pale luminous primrose veil over the lower sky (y = 250 to 510)
local m_lower_veil = rect(0, 250, 1000, 260):soften(40)
work(m_lower_veil, {hand="glaze", pile=p_sky_veil, angle=0, coverage=1.2, fill=true})

-- Blend across the entire sky to unify into a seamless Friedrich twilight
blend(m_sky_wide, {angle=0})

print("Unifying sky glazes applied and blended.")

--@ chunk 7
-- Mix the piles
local p_cloud_slate = pile{{"lead white", 18}, {"smalt", 4}, {"raw umber", 1.5}, {"vermilion", 0.5}, {"bone black", 0.2}, medium=0.45}
local p_cloud_rose = pile{{"lead white", 22}, {"vermilion", 0.8}, {"yellow ochre", 0.8}, {"raw umber", 0.7}, {"smalt", 1}, medium=0.5}
local p_dist_hills = pile{{"lead white", 14}, {"smalt", 5}, {"raw umber", 2.2}, {"red earth", 0.8}, {"bone black", 0.4}, medium=0.35}
local p_water_sheen = pile{{"lead white", 20}, {"yellow ochre", 2.2}, {"smalt", 2.2}, {"raw umber", 0.6}, {"vermilion", 0.25}, medium=0.4}
local p_mist = pile{{"lead white", 28}, {"yellow ochre", 1.8}, {"smalt", 1.5}, {"raw umber", 0.4}, medium=0.55}

-- 1. Paint subtle, delicate horizontal twilight clouds
local b_filb = brush("filbert", 10)
b_filb:load(p_cloud_slate, 0.4)
-- Upper thin cloud bands drifting across
b_filb:stroke({{60, 155}, {220, 152}, {450, 150}, {720, 154}, {920, 150}}, {pressure={0.15, 0.05}, ramps={0.1, 0.2}})
b_filb:stroke({{180, 175}, {360, 172}, {600, 170}, {840, 175}}, {pressure={0.2, 0.05}, ramps={0.15, 0.2}})
b_filb:stroke({{40, 215}, {260, 212}, {520, 210}, {780, 216}, {960, 212}}, {pressure={0.18, 0.05}, ramps={0.1, 0.15}})

-- Warmer lower cloud filaments catching the last pinkish-amber twilight
b_filb:reload(p_cloud_rose, 0.35)
b_filb:stroke({{120, 275}, {310, 270}, {540, 268}, {760, 274}, {940, 270}}, {pressure={0.2, 0.05}, ramps={0.1, 0.2}})
b_filb:stroke({{220, 310}, {440, 306}, {680, 304}, {880, 312}}, {pressure={0.18, 0.05}, ramps={0.1, 0.2}})
b_filb:stroke({{340, 345}, {510, 342}, {720, 340}, {860, 346}}, {pressure={0.15, 0.04}, ramps={0.15, 0.2}})

-- Soften the clouds slightly into the sky
local m_cloud_zone = rect(0, 130, 1000, 230)
blend(m_cloud_zone, {angle=0})

-- 2. Distant low hills across the water
-- Contour of hills:
local pts_hills = {
  {60, 468}, {140, 464}, {240, 458}, {350, 461}, {480, 455}, {600, 457}, {720, 453}, {830, 460}, {940, 468},
  {940, 475}, {60, 475}
}
local m_hills = poly(pts_hills):soften(2)
work(m_hills, {hand="body", pile=p_dist_hills, angle=0, coverage=1.2, fill=true, clip=true})

-- 3. Distant water surface (Bodden / bay)
local m_water = rect(0, 465, 1000, 25):soften(2)
work(m_water, {hand="glaze", pile=p_water_sheen, angle=0, coverage=1.2, fill=true})

-- Water reflections and subtle ripples
local b_flat = brush("flat", 6)
b_flat:load(p_dist_hills, 0.25)
b_flat:stroke({{150, 472}, {320, 472}}, {pressure={0.15, 0.08}})
b_flat:stroke({{450, 473}, {620, 473}}, {pressure={0.12, 0.06}})
b_flat:stroke({{680, 471}, {810, 471}}, {pressure={0.15, 0.08}})

-- 4. Mist hovering over water and dissolving hill bases
local m_mist_band = rect(0, 460, 1000, 20):soften(6)
work(m_mist_band, {hand="glaze", pile=p_mist, angle=0, coverage=0.9, fill=false})

-- Blend mist horizontally
blend(rect(0, 458, 1000, 25), {angle=0})

print("Distant clouds, hills, water, and mist painted.")

--@ chunk 8
-- Mix the earth piles for the terrain
p_ground_base = pile{{"raw umber", 8}, {"yellow ochre", 7}, {"red earth", 2.5}, {"green earth", 3}, {"lead white", 5}, {"bone black", 1.2}, medium=0.25}
p_heath_russet = pile{{"raw umber", 7}, {"yellow ochre", 6}, {"red earth", 3.5}, {"lead white", 4}, {"bone black", 1}, medium=0.2}
p_ground_shadow = pile{{"raw umber", 10}, {"bone black", 4}, {"green earth", 3.5}, {"red earth", 2}, {"lead white", 2}, medium=0.2}
p_ground_moss = pile{{"green earth", 7}, {"yellow ochre", 5}, {"raw umber", 4}, {"lead white", 3}, {"bone black", 1}, medium=0.25}

-- Define knoll boundary
local pts_knoll = {
  {0, 486}, {80, 481}, {160, 472}, {240, 458}, {320, 446},
  {380, 440}, {440, 442}, {520, 455}, {620, 466}, {700, 464},
  {760, 456}, {840, 460}, {920, 470}, {1000, 478},
  {1000, 705}, {0, 705}
}
local m_knoll = poly(pts_knoll, true)

-- Base lay-in of the knoll
work(m_knoll, {
  hand = "body",
  pile = p_ground_base,
  angle = 0.25,
  coverage = 1.3,
  edge = "firm",
  fill = true,
  clip = true
})

-- Middle & lower foreground: deeper russet and heath tones
local pts_fg_russet = {
  {0, 520}, {180, 505}, {360, 485}, {550, 495}, {720, 490}, {1000, 510},
  {1000, 705}, {0, 705}
}
local m_fg_russet = poly(pts_fg_russet, true)
work(m_fg_russet, {
  hand = "body",
  pile = p_heath_russet,
  angle = -0.2,
  coverage = 1.2,
  fill = true
})

-- Shaded hollows and frontal slopes
local pts_hollows = {
  {0, 600}, {220, 580}, {450, 565}, {680, 575}, {1000, 590},
  {1000, 705}, {0, 705}
}
local m_hollows = poly(pts_hollows, true)
work(m_hollows, {
  hand = "body",
  pile = p_ground_shadow,
  angle = 0.1,
  coverage = 1.1,
  fill = true
})

-- Mossy hummocks and damp patches along folds in the ground
local m_moss1 = ellipse(360, 500, 90, 25)
local m_moss2 = ellipse(680, 515, 110, 30)
local m_moss3 = ellipse(180, 570, 100, 35)
local m_moss_total = (m_moss1 + m_moss2 + m_moss3):soften(15)

work(m_moss_total, {
  hand = "scumble",
  pile = p_ground_moss,
  angle = 0.3,
  coverage = 1.0,
  fill = false
})

print("Foreground knoll underpainted.")

--@ chunk 9
local pts_knoll = {
  {0, 486}, {80, 481}, {160, 472}, {240, 458}, {320, 446},
  {380, 440}, {440, 442}, {520, 455}, {620, 466}, {700, 464},
  {760, 456}, {840, 460}, {920, 470}, {1000, 478},
  {1000, 705}, {0, 705}
}
local m_knoll = poly(pts_knoll, true)

-- Blend the wet paint across the knoll to fuse the patches into continuous rolling ground
blend(m_knoll, {angle=0.15})

print("Knoll blended.")

--@ chunk 10
-- Mix stone piles
p_stone_shadow = pile{{"raw umber", 9}, {"bone black", 5}, {"smalt", 2}, {"lead white", 2}, {"red earth", 1}, medium=0.15}
p_stone_body = pile{{"lead white", 9}, {"raw umber", 5}, {"bone black", 2.5}, {"smalt", 2}, {"yellow ochre", 2}, {"red earth", 0.8}, medium=0.2}
p_stone_half = pile{{"lead white", 13}, {"raw umber", 3.2}, {"yellow ochre", 2.2}, {"smalt", 2}, {"bone black", 1}, medium=0.2}
p_stone_light = pile{{"lead white", 18}, {"yellow ochre", 2.5}, {"smalt", 1.8}, {"raw umber", 1}, {"vermilion", 0.2}, medium=0.25}
p_lichen_pale = pile{{"lead white", 14}, {"green earth", 4.5}, {"yellow ochre", 3.5}, {"raw umber", 1}, medium=0.25}
p_lichen_ochre = pile{{"lead white", 9}, {"yellow ochre", 8}, {"chrome yellow", 1.2}, {"raw umber", 1.2}, {"red earth", 0.6}, medium=0.2}

-- 1. Dark interior chamber shadow under capstone
local m_chamber = poly({
  {342, 431}, {450, 431}, {448, 459}, {340, 460}
})
work(m_chamber, {hand="body", pile=p_stone_shadow, angle=0.8, coverage=1.4, fill=true, clip=true})

-- 2. Center-rear orthostat (in shadow, seen between front uprights)
local m_rear_orth = poly({{384, 428}, {406, 428}, {408, 458}, {386, 457}})
work(m_rear_orth, {hand="detail", pile=p_stone_body, angle=1.5, coverage=1.3, fill=true, clip=true})

-- 3. Left Orthostat
local m_left_orth = poly({{340, 430}, {360, 431}, {362, 462}, {338, 461}})
-- Shadow side
work(m_left_orth, {hand="detail", pile=p_stone_body, angle=1.4, coverage=1.4, fill=true, clip=true})
-- Light catch on top/left corner
local m_left_orth_light = poly({{339, 430}, {348, 430}, {346, 455}, {338, 450}})
work(m_left_orth_light, {hand="detail", pile=p_stone_half, angle=1.4, coverage=1.2, fill=true, clip=true})

-- 4. Right Orthostat
local m_right_orth = poly({{431, 431}, {451, 432}, {453, 460}, {429, 459}})
-- Shadow side
work(m_right_orth, {hand="detail", pile=p_stone_body, angle=1.4, coverage=1.4, fill=true, clip=true})
-- Sky light catch on outer face
local m_right_orth_light = poly({{443, 432}, {452, 432}, {454, 458}, {446, 459}})
work(m_right_orth_light, {hand="detail", pile=p_stone_half, angle=1.4, coverage=1.2, fill=true, clip=true})

-- 5. The Massive Capstone
local pts_capstone = {
  {324, 428}, {334, 420}, {365, 414}, {405, 412}, {445, 415}, {470, 421},
  {473, 430}, {455, 434}, {415, 430}, {375, 431}, {338, 434}, {324, 428}
}
local m_capstone = poly(pts_capstone)
-- Base mass of capstone
work(m_capstone, {hand="body", pile=p_stone_body, angle=0.1, coverage=1.4, fill=true, clip=true})

-- Underside shadow rim of capstone
local m_cap_shadow = poly({
  {324, 428}, {338, 434}, {375, 431}, {415, 430}, {455, 434}, {473, 430},
  {468, 435}, {420, 433}, {370, 434}, {332, 436}, {324, 428}
})
work(m_cap_shadow, {hand="detail", pile=p_stone_shadow, angle=0, coverage=1.3, fill=true, clip=true})

-- Light-facing top facets of capstone
local pts_cap_light = {
  {324, 428}, {334, 420}, {365, 414}, {405, 412}, {445, 415}, {470, 421},
  {468, 426}, {435, 422}, {395, 420}, {355, 423}, {328, 427}
}
local m_cap_light = poly(pts_cap_light)
work(m_cap_light, {hand="detail", pile=p_stone_light, angle=0.05, coverage=1.3, fill=true, clip=true})

-- Half-tone transition plane on capstone front
local pts_cap_mid = {
  {328, 427}, {355, 423}, {395, 420}, {435, 422}, {468, 426},
  {470, 430}, {445, 429}, {400, 427}, {360, 428}, {334, 431}
}
local m_cap_mid = poly(pts_cap_mid)
work(m_cap_mid, {hand="detail", pile=p_stone_half, angle=0.08, coverage=1.2, fill=true, clip=true})

-- 6. Lichen patches on capstone and uprights
local m_lich1 = poly({{350, 415}, {368, 414}, {372, 422}, {354, 423}})
local m_lich2 = poly({{420, 413}, {438, 415}, {440, 421}, {422, 419}})
local m_lich3 = poly({{342, 436}, {348, 436}, {350, 448}, {344, 448}})
work((m_lich1 + m_lich2 + m_lich3):soften(1), {hand="detail", pile=p_lichen_pale, angle=0.2, coverage=1.1, fill=false})

local m_lich_gold = poly({{390, 413}, {404, 413}, {406, 419}, {392, 418}})
work(m_lich_gold:soften(1), {hand="detail", pile=p_lichen_ochre, angle=-0.2, coverage=1.0, fill=false})

-- 7. Surrounding erratic boulders
-- Leaning slab left
local m_lean = poly({{305, 456}, {320, 442}, {334, 445}, {328, 464}, {305, 456}})
work(m_lean, {hand="detail", pile=p_stone_body, angle=0.5, coverage=1.3, fill=true, clip=true})
local m_lean_light = poly({{318, 443}, {333, 445}, {330, 452}, {317, 448}})
work(m_lean_light, {hand="detail", pile=p_stone_light, angle=0.3, coverage=1.2, fill=true, clip=true})

-- Boulder right of dolmen
local m_rbould = poly({{475, 452}, {496, 442}, {516, 446}, {521, 458}, {490, 464}, {475, 452}})
work(m_rbould, {hand="detail", pile=p_stone_body, angle=0.2, coverage=1.3, fill=true, clip=true})
local m_rbould_light = poly({{488, 443}, {514, 446}, {512, 452}, {486, 450}})
work(m_rbould_light, {hand="detail", pile=p_stone_half, angle=0.1, coverage=1.2, fill=true, clip=true})

print("Megaliths painted.")

--@ chunk 11
-- Refine the dolmen, deepen interior cavern shadow, and bed stones into the turf

local b_round = brush{kind="round", width=2.5, point=1, stiffness=0.6}
local b_fine = brush{kind="round", width=1.2, point=1, stiffness=0.7}

-- 1. Deepen the burial chamber cavern shadow
local p_cavern = pile{{"bone black", 8}, {"raw umber", 8}, {"smalt", 1.5}, {"lead white", 0.5}, medium=0.15}
local m_cavern_deep = poly({
  {342, 432}, {448, 432}, {447, 461}, {340, 462}
})
work(m_cavern_deep, {hand="detail", pile=p_cavern, angle=1.2, coverage=1.6, fill=true, clip=true})

-- 2. Weathering, fissures, and irregularities on the capstone
b_round:load(p_cavern, 0.5)
-- Fissures in the capstone face
b_fine:stroke({{370, 417}, {373, 423}, {371, 429}}, {pressure={0.4, 0.1}})
b_fine:stroke({{430, 418}, {432, 424}, {435, 430}}, {pressure={0.35, 0.1}})
b_fine:stroke({{338, 426}, {345, 428}}, {pressure={0.4, 0.1}})
-- Dark under-edge shadow accent
b_round:stroke({{324, 431}, {355, 433}, {395, 432}, {435, 432}, {472, 432}}, {pressure={0.6, 0.4}})

-- 3. Top rim light / horizon glow catching the top edge of the capstone
local p_stone_rim = pile{{"lead white", 22}, {"yellow ochre", 3}, {"raw umber", 0.8}, {"vermilion", 0.2}, medium=0.25}
b_fine:load(p_stone_rim, 0.4)
b_fine:stroke({{325, 427}, {338, 420}, {370, 414}, {410, 412}, {450, 415}, {471, 421}}, {pressure={0.4, 0.2}})

-- 4. Bedding the stones into the earth: turf, moss, and grass tufts around orthostats
local p_turf_shadow = pile{{"raw umber", 9}, {"bone black", 5}, {"green earth", 4}, {"lead white", 1}, medium=0.2}
local p_dry_grass = pile{{"yellow ochre", 9}, {"raw umber", 5}, {"lead white", 5}, {"red earth", 1.5}, medium=0.2}

-- Dark soil and contact shadow at base of uprights
b_round:load(p_turf_shadow, 0.6)
b_round:stroke({{330, 461}, {368, 462}}, {pressure={0.7, 0.5}})
b_round:stroke({{380, 458}, {412, 459}}, {pressure={0.7, 0.5}})
b_round:stroke({{425, 460}, {460, 460}}, {pressure={0.7, 0.5}})
b_round:stroke({{300, 463}, {336, 464}}, {pressure={0.6, 0.4}})
b_round:stroke({{472, 463}, {525, 462}}, {pressure={0.6, 0.4}})

-- Grass blades and dry stems overlapping stone bases
b_fine:load(p_dry_grass, 0.5)
local grass_pts = {
  {{335, 465}, {336, 456}}, {{338, 466}, {340, 454}}, {{343, 464}, {344, 455}},
  {{358, 465}, {360, 455}}, {{363, 466}, {364, 457}}, {{386, 462}, {387, 453}},
  {{405, 462}, {406, 452}}, {{428, 463}, {430, 454}}, {{433, 464}, {434, 455}},
  {{450, 464}, {451, 453}}, {{455, 465}, {456, 456}}, {{312, 466}, {314, 456}},
  {{322, 467}, {324, 458}}, {{480, 465}, {482, 455}}, {{505, 464}, {507, 455}}
}
for _, g in ipairs(grass_pts) do
  b_fine:stroke(g, {pressure={0.5, 0.05}, ramps={0.1, 0.4}})
end

-- 5. Model the foreground erratic boulders
-- Boulder left: x ≈ 210-275, y ≈ 515-545
local m_bould_l = poly({{210, 532}, {225, 516}, {255, 514}, {274, 524}, {275, 538}, {245, 545}, {215, 542}})
work(m_bould_l, {hand="detail", pile=p_stone_body, angle=0.4, coverage=1.3, fill=true, clip=true})
-- Light top plane
local m_bould_l_top = poly({{225, 516}, {255, 514}, {270, 522}, {245, 526}, {225, 522}})
work(m_bould_l_top, {hand="detail", pile=p_stone_half, angle=0.2, coverage=1.2, fill=true, clip=true})
-- Shadow base
b_round:load(p_turf_shadow, 0.5)
b_round:stroke({{210, 542}, {245, 546}, {278, 540}}, {pressure={0.7, 0.4}})

-- Boulder right: x ≈ 645-715, y ≈ 525-555
local m_bould_r = poly({{645, 540}, {665, 526}, {695, 525}, {715, 536}, {714, 548}, {680, 555}, {648, 550}})
work(m_bould_r, {hand="detail", pile=p_stone_body, angle=-0.3, coverage=1.3, fill=true, clip=true})
local m_bould_r_top = poly({{665, 526}, {695, 525}, {710, 532}, {685, 535}, {665, 532}})
work(m_bould_r_top, {hand="detail", pile=p_stone_half, angle=-0.2, coverage=1.2, fill=true, clip=true})
b_round:load(p_turf_shadow, 0.5)
b_round:stroke({{642, 550}, {680, 556}, {718, 548}}, {pressure={0.7, 0.4}})

print("Dolmen refined, chamber deepened, stones bedded into turf.")

--@ chunk 12
-- Paint the upright orthostats in front of the dark cavern void

-- 1. Center-rear orthostat (sitting deep in the chamber)
local pts_center_orth = {
  {386, 430}, {404, 430}, {406, 458}, {384, 458}
}
local m_center_orth = poly(pts_center_orth)
work(m_center_orth, {hand="detail", pile=p_stone_half, angle=1.5, coverage=1.4, fill=true, clip=true})

-- Center-rear orthostat side shadow
local m_center_orth_sh = poly({{386, 430}, {394, 430}, {392, 458}, {384, 458}})
work(m_center_orth_sh, {hand="detail", pile=p_stone_shadow, angle=1.5, coverage=1.3, fill=true, clip=true})

-- 2. Left Orthostat (front-left pillar supporting capstone)
local pts_left_orth = {
  {340, 430}, {362, 431}, {365, 445}, {363, 463}, {352, 464}, {338, 462}, {337, 442}
}
local m_left_orth = poly(pts_left_orth)
-- Shadow face (recess-facing)
work(m_left_orth, {hand="body", pile=p_stone_body, angle=1.4, coverage=1.4, fill=true, clip=true})

-- Front and outer sky-facing facet of left orthostat
local pts_left_orth_light = {
  {338, 431}, {348, 431}, {349, 463}, {338, 462}, {337, 442}
}
local m_left_orth_light = poly(pts_left_orth_light)
work(m_left_orth_light, {hand="detail", pile=p_stone_half, angle=1.3, coverage=1.4, fill=true, clip=true})

-- Highlight on the outer edge catching the twilight sky
local b_fine = brush{kind="round", width=1.3, point=1, stiffness=0.7}
b_fine:load(p_stone_light, 0.4)
b_fine:stroke({{338, 431}, {337, 445}, {338, 462}}, {pressure={0.4, 0.2}})

-- 3. Right Orthostat (front-right pillar supporting capstone)
local pts_right_orth = {
  {428, 431}, {452, 432}, {454, 448}, {452, 462}, {440, 463}, {427, 461}, {426, 445}
}
local m_right_orth = poly(pts_right_orth)
work(m_right_orth, {hand="body", pile=p_stone_body, angle=1.4, coverage=1.4, fill=true, clip=true})

-- Front lit facet of right orthostat
local pts_right_orth_light = {
  {440, 432}, {452, 432}, {454, 448}, {452, 462}, {441, 462}
}
local m_right_orth_light = poly(pts_right_orth_light)
work(m_right_orth_light, {hand="detail", pile=p_stone_half, angle=1.3, coverage=1.4, fill=true, clip=true})

b_fine:load(p_stone_light, 0.4)
b_fine:stroke({{452, 432}, {454, 448}, {452, 462}}, {pressure={0.4, 0.2}})

-- 4. Deep cast shadows beneath the capstone onto the tops of the orthostats
local b_round = brush{kind="round", width=2.5, point=1, stiffness=0.6}
b_round:load(p_stone_shadow, 0.6)
b_round:stroke({{336, 432}, {366, 433}}, {pressure={0.7, 0.5}})
b_round:stroke({{425, 432}, {455, 433}}, {pressure={0.7, 0.5}})
b_round:stroke({{384, 431}, {408, 431}}, {pressure={0.7, 0.5}})

-- 5. Earth and grass grounding the pillars firmly
local p_turf_shadow = pile{{"raw umber", 9}, {"bone black", 5}, {"green earth", 4}, {"lead white", 1}, medium=0.2}
local p_dry_grass = pile{{"yellow ochre", 9}, {"raw umber", 5}, {"lead white", 5}, {"red earth", 1.5}, medium=0.2}

b_round:load(p_turf_shadow, 0.6)
b_round:stroke({{330, 462}, {370, 463}}, {pressure={0.8, 0.5}})
b_round:stroke({{420, 462}, {460, 462}}, {pressure={0.8, 0.5}})
b_round:stroke({{375, 460}, {415, 460}}, {pressure={0.8, 0.5}})

-- Delicate grass blades sprouting around stone bases
b_fine:load(p_dry_grass, 0.5)
local grass = {
  {{334, 467}, {335, 455}}, {{338, 466}, {341, 453}}, {{344, 467}, {343, 456}},
  {{358, 467}, {360, 456}}, {{365, 466}, {367, 458}}, {{382, 463}, {383, 454}},
  {{405, 462}, {407, 453}}, {{424, 465}, {426, 454}}, {{432, 466}, {431, 455}},
  {{448, 466}, {450, 454}}, {{455, 465}, {457, 457}}
}
for _, g in ipairs(grass) do
  b_fine:stroke(g, {pressure={0.5, 0.05}, ramps={0.1, 0.4}})
end

print("Orthostats sculpted in front of cavern void.")

--@ chunk 13
-- 1. Naturalize capstone top & side boulders
local b_round = brush{kind="round", width=2.5, point=1, stiffness=0.6}
local b_fine = brush{kind="round", width=1.2, point=1, stiffness=0.7}

-- Round and weather the capstone top
local pts_cap_round = {
  {324, 428}, {330, 422}, {345, 417}, {370, 414}, {405, 412}, {440, 414}, {465, 419}, {472, 426},
  {470, 430}, {445, 424}, {400, 421}, {360, 422}, {330, 426}
}
local m_cap_round = poly(pts_cap_round, true)
work(m_cap_round, {hand="detail", pile=p_stone_light, angle=0.05, coverage=1.3, fill=true, clip=true})

-- Re-sculpt leaning slab left of dolmen
local pts_lean_rock = {
  {305, 458}, {312, 446}, {324, 442}, {335, 444}, {336, 456}, {326, 464}, {306, 462}
}
local m_lean_rock = poly(pts_lean_rock, true)
work(m_lean_rock, {hand="detail", pile=p_stone_body, angle=0.4, coverage=1.4, fill=true, clip=true})
-- Light top plane of leaning slab
local m_lean_top = poly({{312, 446}, {324, 442}, {335, 444}, {330, 450}, {316, 449}})
work(m_lean_top, {hand="detail", pile=p_stone_light, angle=0.2, coverage=1.3, fill=true, clip=true})
-- Base contact shadow
b_round:load(p_stone_shadow, 0.6)
b_round:stroke({{302, 462}, {332, 465}}, {pressure={0.7, 0.4}})

-- Re-sculpt boulder right of dolmen
local pts_rbould = {
  {474, 454}, {482, 444}, {500, 441}, {516, 445}, {522, 456}, {512, 463}, {485, 464}, {474, 454}
}
local m_rbould = poly(pts_rbould, true)
work(m_rbould, {hand="detail", pile=p_stone_body, angle=0.2, coverage=1.4, fill=true, clip=true})
local m_rbould_top = poly({{482, 444}, {500, 441}, {516, 445}, {510, 450}, {488, 449}})
work(m_rbould_top, {hand="detail", pile=p_stone_half, angle=0.1, coverage=1.3, fill=true, clip=true})
b_round:stroke({{472, 464}, {518, 464}}, {pressure={0.7, 0.4}})

-- 2. Mix tree piles
p_trunk_dark = pile{{"bone black", 9}, {"raw umber", 9}, {"red earth", 2}, {"smalt", 1}, {"lead white", 1}, medium=0.15}
p_bark_mid = pile{{"raw umber", 8}, {"bone black", 4}, {"yellow ochre", 3}, {"lead white", 3}, {"red earth", 1.5}, {"green earth", 1}, medium=0.2}
p_bark_light = pile{{"raw umber", 5}, {"yellow ochre", 5}, {"lead white", 6}, {"bone black", 1.5}, medium=0.25}
p_dead_wood = pile{{"lead white", 10}, {"raw umber", 4}, {"yellow ochre", 2}, {"bone black", 1.5}, {"smalt", 1}, medium=0.25}
p_wood_rim = pile{{"lead white", 16}, {"yellow ochre", 3}, {"raw umber", 1}, {"vermilion", 0.3}, medium=0.3}

-- 3. Solid body of the Oak Trunk
local pts_trunk = {
  {250, 468}, {258, 452}, {266, 420}, {269, 385}, {265, 350}, {255, 320}, {248, 295},
  {268, 290}, {278, 305}, {295, 325}, {298, 350}, {302, 385}, {307, 420}, {314, 444},
  {324, 462}, {305, 466}, {280, 465}, {250, 468}
}
local m_trunk = poly(pts_trunk, true)

-- Body paint for the trunk
work(m_trunk, {
  hand = "body",
  pile = p_trunk_dark,
  angle = 1.45,
  coverage = 1.5,
  fill = true,
  clip = true
})

-- Deep dark hollow in the trunk
local pts_hollow = {
  {276, 434}, {292, 430}, {294, 396}, {282, 392}, {275, 418}
}
local m_hollow = poly(pts_hollow, true)
work(m_hollow, {hand="detail", pile=p_trunk_dark, angle=1.5, coverage=1.6, fill=true, clip=true})

-- Deep shadow on interior edge of hollow
local b_hollow = brush{kind="round", width=2.0, point=1, stiffness=0.8}
b_hollow:load(p_trunk_dark, 0.8)
b_hollow:stroke({{276, 434}, {275, 418}, {282, 392}}, {pressure={0.7, 0.5}})

-- Bark ridges on lit/weathered side of trunk (left and outer surfaces)
local b_bark = brush{kind="round", width=2.2, point=1, stiffness=0.6}
b_bark:load(p_bark_mid, 0.5)
b_bark:stroke({{256, 458}, {263, 435}, {267, 405}, {268, 375}, {263, 345}, {254, 318}}, {pressure={0.6, 0.3}})
b_bark:stroke({{266, 448}, {272, 425}, {273, 395}, {271, 365}}, {pressure={0.5, 0.2}})
b_bark:stroke({{310, 452}, {305, 425}, {301, 395}, {297, 365}, {294, 335}}, {pressure={0.5, 0.2}})

-- Fissures and bark texture lines
b_fine:load(p_trunk_dark, 0.6)
b_fine:stroke({{261, 452}, {266, 425}, {268, 395}, {266, 365}, {258, 335}}, {pressure={0.4, 0.15}})
b_fine:stroke({{306, 442}, {302, 415}, {299, 385}, {296, 355}}, {pressure={0.4, 0.15}})

-- Twilight edge catch along the left contour of the trunk
b_fine:load(p_wood_rim, 0.4)
b_fine:stroke({{250, 466}, {258, 452}, {266, 420}, {269, 385}, {265, 350}, {255, 320}, {248, 295}}, {pressure={0.35, 0.1}})

print("Capstone refined, side boulders painted, oak trunk and hollow sculpted.")

--@ chunk 14
local pts_test = {{296, 348}, {330, 340}, {370, 328}}
local w_test = {16, 13, 10}
local m_test = ribbon(pts_test, w_test)
print("Ribbon test mask created, area = " .. m_test:area())

--@ chunk 15
-- Construct the limbs and branching architecture for both trees

local b_round = brush{kind="round", width=2.0, point=1, stiffness=0.6}
local b_fine = brush{kind="round", width=1.1, point=1, stiffness=0.7}

-- 1. Main Eastern Bough of Ancient Oak (arches over the dolmen)
local m_ebough = ribbon({
  {294, 348}, {330, 340}, {370, 328}, {415, 322}, {460, 312}, {495, 292}, {525, 270}
}, {16, 13, 10, 8, 6, 4.5, 3})
work(m_ebough, {hand="body", pile=p_trunk_dark, angle=0.2, coverage=1.4, fill=true, clip=true})

-- Sub-branches of Eastern Bough
local m_eb1 = ribbon({{370, 328}, {382, 298}, {398, 272}, {412, 248}, {422, 225}}, {6.5, 5, 4, 3, 2})
local m_eb1_sub = ribbon({{398, 272}, {388, 252}, {382, 235}}, {3, 2.2, 1.5})
local m_eb2 = ribbon({{430, 320}, {446, 292}, {468, 270}, {485, 248}}, {5, 4, 3, 2})
local m_eb3 = ribbon({{460, 312}, {478, 332}, {496, 350}, {512, 368}}, {4, 3, 2.2, 1.5})
local m_eb_tip = ribbon({{525, 270}, {548, 255}, {568, 242}}, {3, 2.2, 1.5})

local m_e_subs = m_eb1 + m_eb1_sub + m_eb2 + m_eb3 + m_eb_tip
work(m_e_subs, {hand="detail", pile=p_trunk_dark, angle=0.4, coverage=1.3, fill=true, clip=true})

-- 2. Western Limb of Ancient Oak (reaching leftward)
local m_wbough = ribbon({
  {264, 342}, {235, 328}, {205, 308}, {175, 290}, {145, 278}, {115, 270}, {85, 268}
}, {15, 12, 9, 7, 5, 3.5, 2})
work(m_wbough, {hand="body", pile=p_trunk_dark, angle=-0.3, coverage=1.4, fill=true, clip=true})

-- Sub-branches of Western Limb
local m_wb1 = ribbon({{205, 308}, {192, 275}, {180, 248}, {165, 222}, {150, 202}}, {6, 4.8, 3.8, 2.8, 1.8})
local m_wb1_sub = ribbon({{180, 248}, {195, 225}, {202, 205}}, {2.8, 2, 1.4})
local m_wb2 = ribbon({{175, 290}, {160, 315}, {140, 335}, {120, 348}}, {4.5, 3.5, 2.5, 1.5})
local m_wb3 = ribbon({{115, 270}, {100, 250}, {88, 235}}, {2.5, 1.8, 1.2})

local m_w_subs = m_wb1 + m_wb1_sub + m_wb2 + m_wb3
work(m_w_subs, {hand="detail", pile=p_trunk_dark, angle=-0.5, coverage=1.3, fill=true, clip=true})

-- 3. Central Trunk & Stag-Headed Crown (dead, bleached wood)
local m_stag_stem = ribbon({
  {268, 305}, {270, 265}, {268, 225}, {264, 185}, {258, 150}, {254, 125}
}, {13, 10, 7.5, 5.5, 3.5, 2})
work(m_stag_stem, {hand="detail", pile=p_dead_wood, angle=1.5, coverage=1.4, fill=true, clip=true})

-- Stag Horn 1 (right)
local m_sh1 = ribbon({{268, 225}, {288, 195}, {308, 168}, {320, 142}, {325, 120}}, {5, 4, 3, 2.2, 1.5})
local m_sh1_fork = ribbon({{288, 195}, {280, 168}, {275, 145}}, {2.8, 2, 1.3})
-- Stag Horn 2 (left)
local m_sh2 = ribbon({{270, 265}, {250, 235}, {232, 208}, {218, 180}, {208, 155}}, {5.5, 4.2, 3.2, 2.2, 1.5})
local m_sh2_fork = ribbon({{232, 208}, {242, 182}, {246, 160}}, {2.5, 1.8, 1.2})
-- Stag Horn 3 (upper right)
local m_sh3 = ribbon({{272, 285}, {295, 258}, {322, 232}, {342, 208}, {355, 182}}, {6, 4.8, 3.5, 2.5, 1.6})
local m_sh3_fork = ribbon({{322, 232}, {335, 205}, {340, 178}}, {2.6, 1.8, 1.2})

local m_horns = m_sh1 + m_sh1_fork + m_sh2 + m_sh2_fork + m_sh3 + m_sh3_fork
work(m_horns, {hand="detail", pile=p_dead_wood, angle=1.2, coverage=1.3, fill=true, clip=true})

-- 4. Secondary Companion Tree (Right Knoll, x ≈ 770)
local m_tree2_trunk = ribbon({
  {776, 465}, {774, 430}, {769, 395}, {763, 360}, {756, 325}, {748, 290}, {740, 255}
}, {11, 9.5, 8, 6.5, 5, 3.5, 2})
work(m_tree2_trunk, {hand="body", pile=p_trunk_dark, angle=1.4, coverage=1.4, fill=true, clip=true})

-- Secondary Tree Branches
local m_t2_l = ribbon({{769, 395}, {746, 372}, {724, 348}, {700, 328}, {678, 312}}, {5.5, 4.2, 3.2, 2.2, 1.5})
local m_t2_l_sub = ribbon({{724, 348}, {710, 322}, {700, 298}}, {2.5, 1.8, 1.2})
local m_t2_r = ribbon({{763, 360}, {788, 342}, {815, 322}, {842, 305}, {868, 290}}, {5, 4, 3, 2.2, 1.5})
local m_t2_r_sub = ribbon({{815, 322}, {832, 298}, {842, 275}}, {2.5, 1.8, 1.2})
local m_t2_top = ribbon({{748, 290}, {740, 255}, {732, 228}, {725, 205}}, {3.5, 2.5, 1.8, 1.2})

local m_t2_branches = m_t2_l + m_t2_l_sub + m_t2_r + m_t2_r_sub + m_t2_top
work(m_t2_branches, {hand="detail", pile=p_trunk_dark, angle=1.0, coverage=1.3, fill=true, clip=true})

print("Tree limbs, boughs, and stag horns constructed.")

--@ chunk 16
-- Model trunk, branch collars, stag horn transition, and bark anatomy

local b_round = brush{kind="round", width=2.4, point=1, stiffness=0.6}
local b_med = brush{kind="round", width=1.6, point=1, stiffness=0.7}
local b_fine = brush{kind="round", width=1.0, point=1, stiffness=0.8}

-- Piles for modeling bark and wood
local p_trunk_core = pile{{"bone black", 9}, {"raw umber", 9}, {"red earth", 2}, {"smalt", 1}, medium=0.15}
local p_bark_furrow = pile{{"raw umber", 8}, {"bone black", 5}, {"yellow ochre", 2.5}, {"lead white", 1.5}, medium=0.2}
local p_bark_ridge = pile{{"raw umber", 6}, {"yellow ochre", 5}, {"lead white", 4.5}, {"bone black", 1.5}, medium=0.2}
local p_dead_shade = pile{{"raw umber", 6}, {"bone black", 3}, {"lead white", 5}, {"yellow ochre", 2}, medium=0.25}
local p_dead_bleach = pile{{"lead white", 14}, {"raw umber", 3}, {"yellow ochre", 1.8}, {"bone black", 1}, medium=0.25}
local p_wood_rim = pile{{"lead white", 16}, {"yellow ochre", 3.5}, {"raw umber", 1}, {"vermilion", 0.3}, medium=0.25}

-- 1. Unify and model the trunk cylinder and hollow
-- Dark base coat over the patchy areas
b_round:load(p_trunk_core, 0.7)
b_round:stroke({{252, 465}, {260, 435}, {266, 400}, {267, 365}, {262, 335}, {255, 305}}, {pressure={0.8, 0.6}})
b_round:stroke({{275, 455}, {280, 420}, {282, 385}, {280, 350}, {276, 320}, {270, 295}}, {pressure={0.7, 0.5}})
b_round:stroke({{312, 458}, {306, 425}, {302, 390}, {298, 355}, {295, 330}}, {pressure={0.7, 0.5}})

-- Re-establish the deep hollow cavity
local pts_hollow_inner = {{276, 434}, {290, 430}, {292, 396}, {282, 392}, {275, 418}}
local m_hollow_in = poly(pts_hollow_inner, true)
work(m_hollow_in, {hand="detail", pile=p_trunk_core, angle=1.5, coverage=1.6, fill=true, clip=true})

-- 2. Branch Collars and Muscular Junctions
-- Eastern bough collar (swelling underneath at junction with trunk)
local pts_e_collar = {{288, 358}, {305, 355}, {320, 348}, {330, 342}, {325, 338}, {298, 342}}
local m_e_collar = poly(pts_e_collar, true)
work(m_e_collar, {hand="detail", pile=p_trunk_core, angle=0.3, coverage=1.4, fill=true, clip=true})

-- Eastern branch bark ridge (on top of junction)
b_med:load(p_bark_ridge, 0.5)
b_med:stroke({{285, 338}, {302, 336}, {320, 332}}, {pressure={0.6, 0.3}})

-- Western limb collar (swelling underneath)
local pts_w_collar = {{270, 352}, {255, 350}, {240, 342}, {235, 335}, {248, 330}, {265, 335}}
local m_w_collar = poly(pts_w_collar, true)
work(m_w_collar, {hand="detail", pile=p_trunk_core, angle=-0.4, coverage=1.4, fill=true, clip=true})

-- 3. Seamless Transition from Trunk into Stag Crown
-- Living bark envelops the base of the dead retrenched stem
b_round:load(p_bark_furrow, 0.6)
b_round:stroke({{255, 318}, {260, 298}, {266, 280}, {268, 265}}, {pressure={0.7, 0.4}})
b_round:stroke({{292, 325}, {285, 302}, {278, 285}, {274, 268}}, {pressure={0.6, 0.4}})

-- Dead branch stub on the right shoulder of the trunk
local m_stub = poly({{288, 320}, {298, 314}, {302, 318}, {292, 325}}, true)
work(m_stub, {hand="detail", pile=p_dead_shade, angle=0.5, coverage=1.3, fill=true, clip=true})
b_fine:load(p_dead_bleach, 0.4)
b_fine:stroke({{298, 314}, {302, 318}}, {pressure={0.4, 0.2}})

-- 4. Deep longitudinal bark furrows on the trunk
b_fine:load(p_trunk_core, 0.7)
b_fine:stroke({{258, 460}, {264, 432}, {268, 400}, {268, 365}, {263, 332}, {258, 305}}, {pressure={0.5, 0.2}})
b_fine:stroke({{268, 452}, {272, 428}, {274, 395}, {273, 360}, {270, 325}}, {pressure={0.5, 0.2}})
b_fine:stroke({{305, 455}, {302, 422}, {298, 388}, {295, 355}, {292, 325}}, {pressure={0.5, 0.2}})

-- Lit bark ridges catching evening light
b_fine:load(p_bark_ridge, 0.5)
b_fine:stroke({{255, 458}, {262, 430}, {266, 398}, {267, 362}, {261, 330}, {256, 305}}, {pressure={0.4, 0.15}})
b_fine:stroke({{270, 448}, {273, 422}, {275, 390}, {274, 355}}, {pressure={0.35, 0.1}})

-- 5. Stag Horns: weather the dead wood with shaded undersides and bleached light
-- Shaded undersides of stag antlers
b_med:load(p_dead_shade, 0.5)
b_med:stroke({{270, 265}, {272, 225}, {268, 185}, {262, 150}, {257, 126}}, {pressure={0.4, 0.2}})
b_med:stroke({{270, 225}, {290, 196}, {310, 169}, {322, 143}, {326, 122}}, {pressure={0.35, 0.15}})
b_med:stroke({{270, 265}, {252, 236}, {234, 209}, {220, 181}, {210, 156}}, {pressure={0.35, 0.15}})
b_med:stroke({{274, 285}, {297, 259}, {324, 233}, {344, 209}, {356, 183}}, {pressure={0.4, 0.15}})

-- Bleached tops catching the pale zenith
b_fine:load(p_dead_bleach, 0.5)
b_fine:stroke({{266, 265}, {266, 225}, {262, 185}, {256, 150}, {252, 125}}, {pressure={0.35, 0.1}})
b_fine:stroke({{267, 224}, {286, 194}, {306, 167}, {318, 141}, {323, 119}}, {pressure={0.3, 0.1}})
b_fine:stroke({{268, 264}, {248, 234}, {230, 207}, {216, 179}, {206, 154}}, {pressure={0.3, 0.1}})
b_fine:stroke({{271, 284}, {293, 257}, {320, 231}, {340, 207}, {353, 181}}, {pressure={0.3, 0.1}})

-- Fine cracks along the bleached dead wood
b_fine:load(p_trunk_core, 0.4)
b_fine:stroke({{268, 255}, {267, 210}, {263, 170}}, {pressure={0.25, 0.05}})
b_fine:stroke({{280, 210}, {298, 180}, {312, 155}}, {pressure={0.25, 0.05}})

print("Trunk anatomy, branch collars, and stag wood modeled.")

--@ chunk 17
-- Define needed piles globally
p_bark_furrow = pile{{"raw umber", 8}, {"bone black", 5}, {"yellow ochre", 2.5}, {"lead white", 1.5}, medium=0.2}
p_bark_ridge = pile{{"raw umber", 6}, {"yellow ochre", 5}, {"lead white", 4.5}, {"bone black", 1.5}, medium=0.2}
p_dead_shade = pile{{"raw umber", 6}, {"bone black", 3}, {"lead white", 5}, {"yellow ochre", 2}, medium=0.25}
p_dead_bleach = pile{{"lead white", 14}, {"raw umber", 3}, {"yellow ochre", 1.8}, {"bone black", 1}, medium=0.25}
p_wood_rim = pile{{"lead white", 16}, {"yellow ochre", 3.5}, {"raw umber", 1}, {"vermilion", 0.3}, medium=0.25}

-- 1. Seamlessly bridge trunk top into stag crown with gnarled bark wood
local pts_junction = {
  {254, 320}, {258, 290}, {264, 270}, {274, 268}, {284, 280}, {294, 305}, {296, 330},
  {285, 330}, {275, 310}, {265, 315}
}
local m_junction = poly(pts_junction, true)
work(m_junction, {hand="body", pile=p_trunk_dark, angle=1.4, coverage=1.6, fill=true, clip=true})

-- Model the junction with weathered bark folds and wood grain
local b_med = brush{kind="round", width=1.8, point=1, stiffness=0.7}
local b_fine = brush{kind="round", width=1.1, point=1, stiffness=0.8}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.8}

b_med:load(p_bark_furrow, 0.6)
b_med:stroke({{256, 315}, {262, 288}, {266, 268}}, {pressure={0.7, 0.4}})
b_med:stroke({{292, 318}, {284, 292}, {276, 272}}, {pressure={0.6, 0.3}})
b_med:stroke({{274, 325}, {273, 295}, {271, 270}}, {pressure={0.6, 0.3}})

-- 2. Unify the Eastern Bough and Western Limb
-- Eastern bough solid body
local b_thick = brush{kind="round", width=3.5, point=1, stiffness=0.6}
b_thick:load(p_trunk_dark, 0.8)
b_thick:stroke({
  {292, 348}, {330, 340}, {370, 328}, {415, 322}, {460, 312}, {495, 292}, {525, 270}
}, {pressure={0.9, 0.4}, swell={1.1, 0.95, 0.9, 0.85, 0.8, 0.7, 0.5}})

-- Western limb solid body
b_thick:stroke({
  {266, 342}, {235, 328}, {205, 308}, {175, 290}, {145, 278}, {115, 270}, {85, 268}
}, {pressure={0.9, 0.4}, swell={1.1, 0.95, 0.9, 0.85, 0.75, 0.6, 0.4}})

-- 3. Fine crooked oak twigs (sympodial, clustered branching)
b_fine:load(p_trunk_dark, 0.7)

-- Twigs off Eastern Bough upper branches
local e_twigs = {
  -- Off branch 1 (x ≈ 420, y ≈ 225)
  {{422, 225}, {428, 205}, {438, 192}},
  {{428, 205}, {420, 190}, {415, 175}},
  {{412, 248}, {425, 240}, {435, 228}},
  {{382, 235}, {375, 218}, {368, 205}},
  {{382, 235}, {390, 220}, {395, 202}},
  -- Off branch 2 (x ≈ 485, y ≈ 248)
  {{485, 248}, {498, 230}, {508, 215}},
  {{498, 230}, {490, 215}, {485, 198}},
  {{468, 270}, {482, 262}, {495, 250}},
  -- Off drooping shelter branch 3 (x ≈ 512, y ≈ 368)
  {{512, 368}, {522, 385}, {528, 402}},
  {{512, 368}, {505, 382}, {502, 398}},
  {{496, 350}, {508, 360}, {515, 372}},
  {{478, 332}, {488, 342}, {492, 355}},
  -- Off eastern tip continuation (x ≈ 568, y ≈ 242)
  {{568, 242}, {585, 232}, {602, 225}},
  {{585, 232}, {590, 218}, {595, 205}},
  {{548, 255}, {560, 245}, {570, 232}}
}
for _, tw in ipairs(e_twigs) do
  b_fine:stroke(tw, {pressure={0.5, 0.05}, ramps={0.05, 0.4}})
end

-- Twigs off Western Limb branches
local w_twigs = {
  -- Off branch W1 (x ≈ 150, y ≈ 202)
  {{150, 202}, {140, 185}, {132, 168}},
  {{150, 202}, {158, 188}, {162, 172}},
  {{165, 222}, {152, 215}, {142, 205}},
  {{180, 248}, {170, 235}, {162, 220}},
  {{202, 205}, {208, 188}, {212, 170}},
  {{202, 205}, {195, 190}, {190, 175}},
  -- Off drooping branch W2 (x ≈ 120, y ≈ 348)
  {{120, 348}, {108, 362}, {98, 375}},
  {{120, 348}, {128, 360}, {132, 372}},
  {{140, 335}, {130, 348}, {122, 360}},
  -- Off outer tip W3 (x ≈ 88, y ≈ 235)
  {{88, 235}, {75, 222}, {62, 212}},
  {{88, 235}, {92, 218}, {95, 202}},
  {{100, 250}, {88, 245}, {78, 238}},
  {{85, 268}, {70, 265}, {55, 262}},
  {{85, 268}, {78, 278}, {70, 288}}
}
for _, tw in ipairs(w_twigs) do
  b_fine:stroke(tw, {pressure={0.5, 0.05}, ramps={0.05, 0.4}})
end

-- Fine twigs off stag horns (bleached dead twiglets)
b_rigger:load(p_dead_bleach, 0.5)
local stag_twigs = {
  {{254, 125}, {252, 105}, {250, 88}},
  {{254, 125}, {260, 110}, {264, 95}},
  {{325, 120}, {330, 102}, {334, 88}},
  {{325, 120}, {320, 106}, {316, 92}},
  {{275, 145}, {272, 125}, {270, 110}},
  {{208, 155}, {202, 138}, {196, 122}},
  {{246, 160}, {248, 142}, {250, 126}},
  {{355, 182}, {365, 165}, {372, 148}},
  {{340, 178}, {345, 158}, {348, 142}}
}
for _, tw in ipairs(stag_twigs) do
  b_rigger:stroke(tw, {pressure={0.4, 0.05}, ramps={0.05, 0.4}})
end

print("Trunk junction bridged, limbs unified, and fine oak twigs drawn.")

--@ chunk 18
-- Re-solidify tree boles, limbs, branch crotches, and paint substantive oak twigs

local b_bole = brush{kind="round", width=4.5, point=1, stiffness=0.7}
local b_limb = brush{kind="round", width=3.0, point=1, stiffness=0.7}
local b_twig = brush{kind="round", width=1.8, point=1, stiffness=0.8}
local b_stag = brush{kind="round", width=2.2, point=1, stiffness=0.7}

-- Re-mix core dark pile to ensure ample paint
p_trunk_core = pile{{"bone black", 10}, {"raw umber", 8}, {"red earth", 2}, {"smalt", 1}, medium=0.15}
p_bark_mid = pile{{"raw umber", 8}, {"bone black", 4}, {"yellow ochre", 3}, {"lead white", 2.5}, {"red earth", 1.5}, medium=0.2}
p_dead_wood = pile{{"lead white", 12}, {"raw umber", 4}, {"yellow ochre", 2}, {"bone black", 1.5}, medium=0.25}
p_dead_shade = pile{{"raw umber", 7}, {"bone black", 4}, {"lead white", 4}, {"yellow ochre", 2}, medium=0.2}

-- 1. Ancient Oak Trunk & Bole Solidification
b_bole:load(p_trunk_core, 0.9)
-- Main trunk column from roots up to the fork
b_bole:stroke({{255, 465}, {262, 435}, {267, 400}, {268, 365}, {264, 335}, {258, 305}}, {pressure={0.9, 0.7}})
b_bole:stroke({{280, 460}, {282, 425}, {284, 390}, {282, 355}, {278, 320}, {272, 290}}, {pressure={0.9, 0.7}})
b_bole:stroke({{310, 458}, {305, 425}, {302, 390}, {298, 355}, {294, 325}, {285, 295}}, {pressure={0.9, 0.7}})

-- Re-establish the deep hollow cavity in the trunk
local pts_hollow_inner = {{276, 434}, {290, 430}, {292, 396}, {282, 392}, {275, 418}}
local m_hollow_in = poly(pts_hollow_inner, true)
work(m_hollow_in, {hand="detail", pile=p_trunk_core, angle=1.5, coverage=1.6, fill=true, clip=true})

-- 2. Eastern Bough Solidification (sweeping over dolmen)
b_limb:load(p_trunk_core, 0.85)
b_limb:stroke({
  {288, 350}, {325, 342}, {365, 330}, {410, 324}, {455, 314}, {490, 295}, {525, 272}
}, {pressure={0.9, 0.45}, ramps={0.05, 0.15}})

-- Eastern Bough main secondary limbs
b_limb:stroke({{368, 328}, {382, 300}, {398, 274}, {412, 248}, {422, 226}}, {pressure={0.75, 0.35}})
b_limb:stroke({{398, 274}, {388, 252}, {382, 235}}, {pressure={0.5, 0.3}})
b_limb:stroke({{425, 322}, {444, 294}, {466, 272}, {485, 248}}, {pressure={0.7, 0.35}})
b_limb:stroke({{455, 314}, {475, 334}, {494, 352}, {512, 370}}, {pressure={0.65, 0.35}})
b_limb:stroke({{525, 272}, {548, 256}, {568, 244}}, {pressure={0.55, 0.3}})

-- 3. Western Limb Solidification (reaching left)
b_limb:load(p_trunk_core, 0.85)
b_limb:stroke({
  {268, 345}, {238, 330}, {208, 310}, {178, 292}, {148, 280}, {118, 272}, {88, 270}
}, {pressure={0.9, 0.4}, ramps={0.05, 0.15}})

-- Western Limb main secondary limbs
b_limb:stroke({{208, 310}, {194, 278}, {182, 250}, {166, 224}, {150, 204}}, {pressure={0.75, 0.35}})
b_limb:stroke({{182, 250}, {196, 226}, {202, 206}}, {pressure={0.5, 0.3}})
b_limb:stroke({{178, 292}, {162, 316}, {142, 336}, {122, 350}}, {pressure={0.65, 0.35}})
b_limb:stroke({{118, 272}, {102, 252}, {88, 236}}, {pressure={0.55, 0.3}})

-- 4. Central Trunk & Stag-Headed Crown (Bleached, retrenched ancient wood)
b_stag:load(p_dead_wood, 0.85)
-- Central dead spire
b_stag:stroke({{266, 295}, {269, 260}, {268, 220}, {264, 180}, {258, 145}, {254, 120}}, {pressure={0.85, 0.35}})
-- Stag Horn 1 (right)
b_stag:stroke({{268, 220}, {288, 192}, {308, 165}, {320, 140}, {325, 118}}, {pressure={0.7, 0.3}})
b_stag:stroke({{288, 192}, {280, 166}, {274, 142}}, {pressure={0.5, 0.25}})
-- Stag Horn 2 (left)
b_stag:stroke({{269, 260}, {250, 232}, {232, 205}, {218, 178}, {208, 152}}, {pressure={0.75, 0.3}})
b_stag:stroke({{232, 205}, {242, 180}, {246, 158}}, {pressure={0.5, 0.25}})
-- Stag Horn 3 (upper right)
b_stag:stroke({{270, 280}, {295, 255}, {322, 230}, {342, 205}, {355, 180}}, {pressure={0.75, 0.3}})
b_stag:stroke({{322, 230}, {335, 202}, {340, 176}}, {pressure={0.5, 0.25}})

-- Shaded side of stag horn stems
b_twig:load(p_dead_shade, 0.6)
b_twig:stroke({{271, 260}, {270, 220}, {266, 180}, {260, 145}}, {pressure={0.5, 0.2}})
b_twig:stroke({{270, 220}, {290, 193}, {310, 166}, {322, 141}}, {pressure={0.45, 0.2}})
b_twig:stroke({{270, 260}, {252, 233}, {234, 206}, {220, 179}}, {pressure={0.45, 0.2}})

-- 5. Substantive Crooked Oak Twigs on Living Branches
b_twig:load(p_trunk_core, 0.8)

local live_twigs = {
  -- Eastern upper branches
  {{422, 226}, {430, 205}, {440, 190}},
  {{430, 205}, {422, 190}, {416, 172}},
  {{412, 248}, {426, 238}, {438, 225}},
  {{382, 235}, {374, 216}, {366, 200}},
  {{382, 235}, {392, 218}, {398, 198}},
  {{485, 248}, {498, 228}, {510, 212}},
  {{498, 228}, {490, 212}, {484, 195}},
  {{466, 272}, {482, 260}, {496, 246}},
  {{568, 244}, {586, 232}, {604, 222}},
  {{586, 232}, {592, 216}, {598, 202}},
  {{548, 256}, {562, 244}, {572, 230}},
  -- Eastern drooping shelter branch
  {{512, 370}, {524, 388}, {530, 405}},
  {{512, 370}, {504, 385}, {500, 402}},
  {{494, 352}, {508, 362}, {516, 375}},
  {{475, 334}, {486, 344}, {492, 358}},
  -- Western upper branches
  {{150, 204}, {138, 185}, {130, 166}},
  {{150, 204}, {160, 186}, {165, 168}},
  {{166, 224}, {152, 214}, {140, 202}},
  {{182, 250}, {170, 234}, {160, 218}},
  {{202, 206}, {210, 186}, {214, 168}},
  {{202, 206}, {194, 188}, {188, 172}},
  {{88, 236}, {74, 220}, {60, 210}},
  {{88, 236}, {94, 216}, {98, 198}},
  {{102, 252}, {88, 244}, {76, 236}},
  {{88, 270}, {72, 266}, {55, 262}},
  {{88, 270}, {80, 280}, {72, 292}},
  -- Western drooping branch
  {{122, 350}, {108, 364}, {96, 378}},
  {{122, 350}, {130, 362}, {135, 375}},
  {{142, 336}, {132, 350}, {124, 364}}
}

for _, tw in ipairs(live_twigs) do
  b_twig:stroke(tw, {pressure={0.7, 0.25}, ramps={0.05, 0.15}})
end

-- 6. Secondary Companion Tree on Right Knoll (x ≈ 770)
b_bole:load(p_trunk_core, 0.85)
b_bole:stroke({{778, 465}, {775, 430}, {770, 395}, {764, 360}, {756, 325}, {748, 290}, {740, 255}}, {pressure={0.85, 0.45}})

-- Companion tree main branches
b_limb:load(p_trunk_core, 0.8)
b_limb:stroke({{770, 395}, {746, 370}, {722, 345}, {698, 325}, {675, 310}}, {pressure={0.75, 0.35}})
b_limb:stroke({{764, 360}, {788, 340}, {816, 320}, {844, 302}, {870, 288}}, {pressure={0.75, 0.35}})
b_limb:stroke({{748, 290}, {738, 255}, {730, 225}, {722, 202}}, {pressure={0.65, 0.3}})

-- Companion tree twigs
local t2_twigs = {
  {{675, 310}, {658, 298}, {644, 288}},
  {{675, 310}, {668, 294}, {664, 278}},
  {{698, 325}, {686, 310}, {678, 295}},
  {{722, 345}, {710, 322}, {702, 302}},
  {{870, 288}, {888, 276}, {904, 268}},
  {{870, 288}, {878, 272}, {884, 256}},
  {{844, 302}, {856, 285}, {864, 268}},
  {{816, 320}, {830, 298}, {838, 276}},
  {{722, 202}, {715, 185}, {710, 168}},
  {{722, 202}, {728, 184}, {732, 168}},
  {{730, 225}, {740, 205}, {748, 188}}
}
for _, tw in ipairs(t2_twigs) do
  b_twig:stroke(tw, {pressure={0.65, 0.22}, ramps={0.05, 0.15}})
end

print("Trees sculpted with substantive boughs, collars, and twigs.")

--@ chunk 19
local b = brush{kind="round", width=2.4, point=1, stiffness=0.7}

-- Test reloading on each stroke
local test_twigs = {
  {{422, 226}, {432, 205}, {444, 188}},
  {{432, 205}, {424, 188}, {416, 170}},
  {{485, 248}, {500, 226}, {512, 208}},
  {{498, 228}, {490, 210}, {482, 192}},
  {{568, 244}, {588, 230}, {606, 218}},
  {{588, 230}, {595, 214}, {600, 198}}
}

for _, tw in ipairs(test_twigs) do
  b:load(p_trunk_core, 0.8)
  b:stroke(tw, {pressure={0.8, 0.35}, ramps={0.05, 0.2}})
end

print("Reloaded test twigs stroked.")

--@ chunk 20
-- Solidify all limbs and paint complete branching networks with per-stroke reloading

local b_limb = brush{kind="round", width=3.8, point=1, stiffness=0.7}
local b_twig = brush{kind="round", width=2.2, point=1, stiffness=0.8}
local b_stag = brush{kind="round", width=2.6, point=1, stiffness=0.7}
local b_stag_fine = brush{kind="round", width=1.6, point=1, stiffness=0.8}

-- 1. Solidify Ancient Oak Eastern Bough (segments with fresh reload)
local ebough_segs = {
  {{290, 350}, {330, 342}, {370, 330}},
  {{365, 330}, {415, 323}, {460, 313}},
  {{455, 314}, {492, 294}, {528, 272}},
  -- Upward branch 1
  {{368, 328}, {382, 300}, {398, 274}},
  {{396, 276}, {412, 248}, {422, 226}},
  {{396, 274}, {388, 252}, {382, 235}},
  -- Upward branch 2
  {{425, 322}, {444, 294}, {466, 272}},
  {{464, 274}, {485, 248}},
  -- Drooping shelter branch
  {{455, 314}, {475, 334}, {494, 352}},
  {{492, 354}, {512, 370}},
  -- Tip continuation
  {{525, 272}, {548, 256}, {570, 244}}
}
for _, seg in ipairs(ebough_segs) do
  b_limb:load(p_trunk_core, 0.85)
  b_limb:stroke(seg, {pressure={0.85, 0.5}, ramps={0.05, 0.15}})
end

-- 2. Solidify Western Limb
local wbough_segs = {
  {{270, 345}, {238, 330}, {208, 310}},
  {{210, 310}, {178, 292}, {148, 280}},
  {{150, 280}, {118, 272}, {88, 270}},
  -- Upward branch W1
  {{208, 310}, {194, 278}, {182, 250}},
  {{184, 252}, {166, 224}, {150, 204}},
  {{182, 250}, {196, 226}, {202, 206}},
  -- Drooping branch W2
  {{178, 292}, {162, 316}, {142, 336}},
  {{144, 336}, {122, 350}},
  -- Tip W3
  {{118, 272}, {102, 252}, {88, 236}}
}
for _, seg in ipairs(wbough_segs) do
  b_limb:load(p_trunk_core, 0.85)
  b_limb:stroke(seg, {pressure={0.85, 0.5}, ramps={0.05, 0.15}})
end

-- 3. Solidify and Integrate the Stag-Headed Crown
-- Blend the trunk bark up into the base of the dead wood
b_limb:load(p_trunk_core, 0.9)
b_limb:stroke({{258, 320}, {264, 285}, {268, 265}}, {pressure={0.9, 0.6}})
b_limb:stroke({{290, 325}, {282, 290}, {274, 268}}, {pressure={0.9, 0.6}})

-- Bleached stag antler main boughs
local stag_segs = {
  {{268, 268}, {268, 225}, {264, 182}},
  {{264, 185}, {258, 145}, {254, 118}},
  -- Stag horn 1 (right)
  {{268, 225}, {288, 194}, {308, 166}},
  {{306, 168}, {320, 140}, {326, 116}},
  {{288, 194}, {280, 166}, {274, 140}},
  -- Stag horn 2 (left)
  {{268, 265}, {250, 234}, {232, 206}},
  {{232, 206}, {218, 178}, {208, 150}},
  {{232, 206}, {242, 180}, {246, 156}},
  -- Stag horn 3 (upper right)
  {{272, 280}, {295, 256}, {322, 230}},
  {{320, 232}, {342, 206}, {356, 180}},
  {{322, 230}, {335, 202}, {340, 174}}
}
for _, seg in ipairs(stag_segs) do
  b_stag:load(p_dead_wood, 0.85)
  b_stag:stroke(seg, {pressure={0.85, 0.4}, ramps={0.05, 0.15}})
end

-- Fine bleached dead tips
local stag_fine_tips = {
  {{254, 118}, {252, 98}, {248, 80}},
  {{254, 118}, {262, 102}, {268, 88}},
  {{326, 116}, {332, 96}, {338, 80}},
  {{326, 116}, {320, 98}, {314, 82}},
  {{274, 140}, {270, 120}, {266, 102}},
  {{208, 150}, {202, 132}, {194, 115}},
  {{246, 156}, {248, 138}, {252, 120}},
  {{356, 180}, {366, 162}, {374, 145}},
  {{340, 174}, {346, 154}, {350, 136}}
}
for _, tip in ipairs(stag_fine_tips) do
  b_stag_fine:load(p_dead_wood, 0.8)
  b_stag_fine:stroke(tip, {pressure={0.7, 0.3}, ramps={0.05, 0.2}})
end

-- 4. Full Crooked Oak Twig System on Living Limbs
local all_live_twigs = {
  -- Eastern upper branches
  {{422, 226}, {432, 205}, {444, 188}},
  {{432, 205}, {424, 188}, {416, 170}},
  {{412, 248}, {426, 238}, {438, 225}},
  {{438, 225}, {448, 212}, {455, 198}},
  {{382, 235}, {374, 216}, {366, 198}},
  {{382, 235}, {392, 218}, {398, 198}},
  {{392, 218}, {385, 202}, {380, 188}},
  {{485, 248}, {500, 226}, {512, 208}},
  {{498, 228}, {490, 210}, {482, 192}},
  {{466, 272}, {482, 260}, {496, 246}},
  {{482, 260}, {476, 244}, {472, 230}},
  {{568, 244}, {588, 230}, {606, 218}},
  {{588, 230}, {595, 214}, {600, 198}},
  {{548, 256}, {562, 244}, {574, 230}},
  -- Eastern drooping shelter branch
  {{512, 370}, {524, 390}, {532, 408}},
  {{512, 370}, {504, 386}, {498, 404}},
  {{494, 352}, {508, 364}, {518, 378}},
  {{475, 334}, {486, 346}, {494, 360}},
  -- Western upper branches
  {{150, 204}, {138, 184}, {128, 164}},
  {{150, 204}, {160, 185}, {166, 166}},
  {{166, 224}, {152, 212}, {138, 200}},
  {{182, 250}, {170, 232}, {158, 215}},
  {{202, 206}, {212, 185}, {216, 166}},
  {{202, 206}, {194, 186}, {186, 170}},
  {{88, 236}, {74, 218}, {58, 206}},
  {{88, 236}, {94, 215}, {98, 196}},
  {{102, 252}, {88, 242}, {74, 234}},
  {{88, 270}, {72, 265}, {54, 260}},
  {{88, 270}, {80, 282}, {70, 295}},
  -- Western drooping branch
  {{122, 350}, {108, 365}, {95, 380}},
  {{122, 350}, {130, 364}, {136, 378}},
  {{142, 336}, {132, 352}, {124, 366}}
}
for _, tw in ipairs(all_live_twigs) do
  b_twig:load(p_trunk_core, 0.8)
  b_twig:stroke(tw, {pressure={0.75, 0.3}, ramps={0.05, 0.15}})
end

-- 5. Solidify Secondary Companion Tree (Right Knoll, x ≈ 770)
local t2_segs = {
  {{778, 465}, {775, 430}, {770, 395}},
  {{770, 395}, {764, 360}, {756, 325}},
  {{756, 325}, {748, 290}, {740, 255}},
  -- Left branches
  {{770, 395}, {746, 370}, {722, 345}},
  {{722, 345}, {698, 325}, {675, 310}},
  -- Right branches
  {{764, 360}, {788, 340}, {816, 320}},
  {{816, 320}, {844, 302}, {870, 288}},
  -- Upper stem
  {{748, 290}, {738, 255}, {730, 225}},
  {{730, 225}, {722, 202}}
}
for _, seg in ipairs(t2_segs) do
  b_limb:load(p_trunk_core, 0.85)
  b_limb:stroke(seg, {pressure={0.85, 0.5}, ramps={0.05, 0.15}})
end

-- Companion tree twigs
local t2_twigs = {
  {{675, 310}, {656, 296}, {640, 285}},
  {{675, 310}, {666, 292}, {660, 275}},
  {{698, 325}, {684, 308}, {674, 292}},
  {{722, 345}, {708, 320}, {700, 300}},
  {{870, 288}, {890, 274}, {908, 265}},
  {{870, 288}, {880, 270}, {886, 252}},
  {{844, 302}, {858, 284}, {866, 265}},
  {{816, 320}, {832, 296}, {840, 274}},
  {{722, 202}, {714, 182}, {708, 165}},
  {{722, 202}, {728, 182}, {734, 165}},
  {{730, 225}, {742, 204}, {750, 185}}
}
for _, tw in ipairs(t2_twigs) do
  b_twig:load(p_trunk_core, 0.8)
  b_twig:stroke(tw, {pressure={0.75, 0.3}, ramps={0.05, 0.15}})
end

print("Trees fully solidified and sympodial twig systems painted.")

--@ chunk 21
-- Mix deep opaque tree core pile
p_tree_solid = pile{{"bone black", 12}, {"raw umber", 8}, {"red earth", 2}, medium=0.15}

-- 1. Full solid mask of the Ancient Oak Trunk and Main Limbs
local pts_trunk_solid = {
  {250, 468}, {258, 445}, {265, 415}, {268, 380}, {266, 350},
  -- follow western limb base
  {240, 335}, {210, 315}, {180, 298}, {150, 285}, {120, 276}, {88, 272},
  {86, 266}, {118, 268}, {148, 276}, {178, 288}, {208, 304}, {236, 324},
  -- up left shoulder to stag base
  {258, 305}, {264, 275}, {268, 265}, {274, 265}, {278, 280}, {288, 305},
  -- follow eastern bough
  {310, 338}, {340, 335}, {375, 324}, {415, 318}, {460, 308}, {495, 290}, {528, 268},
  {526, 274}, {492, 295}, {456, 314}, {412, 324}, {368, 332}, {325, 344}, {295, 352},
  -- down right trunk to roots
  {300, 380}, {305, 415}, {312, 442}, {324, 462},
  {305, 466}, {280, 465}, {250, 468}
}
local m_tree_solid = poly(pts_trunk_solid, true)

work(m_tree_solid, {
  hand = "body",
  pile = p_tree_solid,
  angle = 1.4,
  coverage = 2.5,
  fill = true,
  clip = true
})

-- Re-establish the deep hollow cavity in the trunk
local pts_hollow_inner = {{276, 434}, {290, 430}, {292, 396}, {282, 392}, {275, 418}}
local m_hollow_in = poly(pts_hollow_inner, true)
work(m_hollow_in, {hand="detail", pile=p_tree_solid, angle=1.5, coverage=1.8, fill=true, clip=true})

-- 2. Solid Companion Tree on Right Knoll (x ≈ 770)
local pts_t2_solid = {
  {778, 465}, {775, 430}, {770, 395}, {746, 370}, {722, 345}, {698, 325}, {675, 310},
  {674, 306}, {698, 320}, {724, 340}, {750, 362},
  -- up stem
  {756, 325}, {748, 290}, {740, 255}, {732, 226}, {722, 202},
  {725, 202}, {736, 226}, {744, 255}, {752, 290},
  -- right branch
  {762, 335}, {788, 338}, {816, 318}, {844, 300}, {870, 286},
  {868, 292}, {842, 305}, {814, 324}, {786, 344}, {764, 362},
  -- down right trunk
  {772, 395}, {778, 430}, {782, 465}, {778, 465}
}
local m_t2_solid = poly(pts_t2_solid, true)

work(m_t2_solid, {
  hand = "body",
  pile = p_tree_solid,
  angle = 1.3,
  coverage = 2.5,
  fill = true,
  clip = true
})

print("Tree boles and main limbs fully unified with solid body paint.")

--@ chunk 22
-- Mix rich autumn foliage piles
p_foliage_shadow = pile{{"raw umber", 8}, {"red earth", 5}, {"bone black", 2.5}, {"yellow ochre", 2.5}, {"lead white", 1}, medium=0.2}
p_foliage_body = pile{{"yellow ochre", 7}, {"red earth", 5}, {"raw umber", 4}, {"lead white", 2.5}, {"bone black", 0.8}, medium=0.2}
p_foliage_light = pile{{"yellow ochre", 9}, {"lead white", 6}, {"chrome yellow", 1}, {"red earth", 2}, {"raw umber", 1}, {"vermilion", 0.4}, medium=0.25}

-- Define foliage cluster masks
-- Ancient Oak Eastern Bough clusters
local m_fol_e1 = (ellipse(395, 260, 35, 25) + ellipse(425, 240, 30, 20)):soften(10)
local m_fol_e2 = (ellipse(470, 270, 35, 22) + ellipse(490, 245, 28, 18)):soften(10)
local m_fol_e3 = (ellipse(505, 360, 30, 25) + ellipse(485, 340, 25, 20)):soften(10)
local m_fol_e4 = ellipse(555, 250, 32, 20):soften(8)

-- Ancient Oak Western Limb clusters
local m_fol_w1 = (ellipse(175, 240, 35, 22) + ellipse(150, 220, 28, 18)):soften(10)
local m_fol_w2 = (ellipse(135, 335, 32, 24) + ellipse(115, 355, 25, 18)):soften(10)
local m_fol_w3 = ellipse(90, 250, 30, 20):soften(8)

-- Companion Tree clusters
local m_fol_t1 = (ellipse(700, 325, 32, 22) + ellipse(675, 310, 25, 18)):soften(8)
local m_fol_t2 = (ellipse(835, 310, 35, 22) + ellipse(865, 290, 28, 18)):soften(8)
local m_fol_t3 = ellipse(725, 220, 25, 22):soften(8)

local m_oak_fol = m_fol_e1 + m_fol_e2 + m_fol_e3 + m_fol_e4 + m_fol_w1 + m_fol_w2 + m_fol_w3
local m_comp_fol = m_fol_t1 + m_fol_t2 + m_fol_t3
local m_all_fol = m_oak_fol + m_comp_fol

-- 1. Dark core stipple of foliage clusters
stipple(m_all_fol, {
  pile = p_foliage_shadow,
  width = 2.4,
  coverage = 1.3,
  cluster = {0.8, 10},
  drag = {2.5, 1.4},
  feather = 0.5
})

-- 2. Warm russet body foliage
stipple(m_all_fol, {
  pile = p_foliage_body,
  width = 2.0,
  coverage = 1.2,
  cluster = {0.85, 8},
  drag = {2.2, 1.3},
  feather = 0.6
})

-- 3. Twilight-lit leaf edges catching the horizon afterglow
stipple(m_all_fol, {
  pile = p_foliage_light,
  width = 1.6,
  coverage = 0.8,
  cluster = {0.9, 6},
  drag = {1.8, 1.2},
  feather = 0.7
})

print("Autumn marcescent foliage stippled onto both trees.")

--@ chunk 23
-- Sky piles for carving back foliage into delicate, ragged sprays
local p_sky_glow = pile{{"lead white", 30}, {"yellow ochre", 2.2}, {"chrome yellow", 0.6}, {"raw umber", 0.3}, {"vermilion", 0.15}, medium=0.35}
local p_sky_mid = pile{{"lead white", 26}, {"smalt", 4}, {"cobalt blue", 1.2}, {"yellow ochre", 0.6}, {"raw umber", 0.4}, medium=0.35}
local p_sky_upper = pile{{"lead white", 18}, {"smalt", 6}, {"cobalt blue", 2.2}, {"raw umber", 0.6}, medium=0.35}

local b_cut = brush{kind="round", width=6.0, point=1, stiffness=0.6}
local b_cut_fine = brush{kind="round", width=3.5, point=1, stiffness=0.6}

-- Carve back the outer rounded contours of the foliage blobs to break them into ragged natural leaf sprays
-- Upper foliage zones (mid-sky)
b_cut:load(p_sky_mid, 0.7)
b_cut:stroke({{360, 240}, {390, 235}, {420, 225}}, {pressure={0.7, 0.4}})
b_cut:stroke({{440, 225}, {480, 225}, {515, 230}}, {pressure={0.7, 0.4}})
b_cut:stroke({{540, 235}, {570, 235}, {590, 245}}, {pressure={0.7, 0.4}})
b_cut:stroke({{120, 215}, {160, 210}, {200, 215}}, {pressure={0.7, 0.4}})
b_cut:stroke({{60, 235}, {85, 230}, {110, 235}}, {pressure={0.7, 0.4}})
b_cut:stroke({{690, 210}, {725, 205}, {750, 210}}, {pressure={0.7, 0.4}})

-- Lower foliage zones (glow zone)
b_cut:load(p_sky_glow, 0.75)
b_cut:stroke({{470, 360}, {500, 380}, {525, 385}}, {pressure={0.8, 0.4}})
b_cut:stroke({{110, 340}, {135, 360}, {155, 360}}, {pressure={0.8, 0.4}})
b_cut:stroke({{670, 320}, {700, 340}, {725, 340}}, {pressure={0.8, 0.4}})
b_cut:stroke({{805, 310}, {840, 330}, {870, 325}}, {pressure={0.8, 0.4}})

-- Carve openings and sky windows through the middle of the foliage masses
b_cut_fine:load(p_sky_mid, 0.7)
b_cut_fine:stroke({{395, 255}, {405, 265}}, {pressure={0.6, 0.2}})
b_cut_fine:stroke({{460, 260}, {470, 270}}, {pressure={0.6, 0.2}})
b_cut_fine:stroke({{165, 235}, {175, 245}}, {pressure={0.6, 0.2}})
b_cut_fine:stroke({{830, 300}, {840, 312}}, {pressure={0.6, 0.2}})

-- Softly blend the carved edges into the surrounding sky
local m_fol_sky_zone = rect(50, 190, 850, 200)
blend(m_fol_sky_zone, {angle=0})

print("Foliage carved into ragged, natural sprays with sky windows.")

--@ chunk 24
print("Drying state at (400, 300): " .. drying(400, 300))

--@ chunk 25
local new_time = wait(5 * 24 * 60)
print("Time after waiting: " .. new_time)
print("Drying state at (400, 300): " .. drying(400, 300))

--@ chunk 26
-- Fresh, opaque sky piles with lead white body
p_sky_zenith = pile{{"lead white", 14}, {"smalt", 6}, {"cobalt blue", 2.8}, {"raw umber", 0.6}, {"bone black", 0.2}, medium=0.2}
p_sky_mid = pile{{"lead white", 24}, {"smalt", 3.5}, {"cobalt blue", 1.0}, {"yellow ochre", 0.6}, {"raw umber", 0.3}, medium=0.2}
p_sky_trans = pile{{"lead white", 28}, {"yellow ochre", 2.5}, {"chrome yellow", 0.4}, {"raw umber", 0.2}, {"vermilion", 0.15}, medium=0.22}
p_sky_glow = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"chrome yellow", 0.8}, {"vermilion", 0.25}, {"raw umber", 0.2}, medium=0.2}

-- Band 1: Zenith
local m_z = rect(0, 0, 1000, 180):soften(30)
work(m_z, {hand="body", pile=p_sky_zenith, angle=0, order="across", coverage=1.6, fill=true})

-- Band 2: Mid-sky
local m_m = rect(0, 130, 1000, 170):soften(30)
work(m_m, {hand="body", pile=p_sky_mid, angle=0, order="across", coverage=1.6, fill=true})

-- Band 3: Transition
local m_t = rect(0, 250, 1000, 150):soften(30)
work(m_t, {hand="body", pile=p_sky_trans, angle=0, order="across", coverage=1.6, fill=true})

-- Band 4: Horizon glow
local m_g = rect(0, 350, 1000, 125):soften(20)
work(m_g, {hand="body", pile=p_sky_glow, angle=0, order="across", coverage=1.7, fill=true})

-- Seamless horizontal blending across the entire canvas width
local m_sky_full = rect(0, 0, 1000, 475)
blend(m_sky_full, {angle=0})

print("Flawless full-width sky painted and blended.")

--@ chunk 27
-- 1. Soften the sky transition vertically to fuse the bands into a continuous atmospheric gradient
blend(rect(0, 40, 1000, 400), {angle=1.57})
blend(rect(0, 40, 1000, 400), {angle=0})

-- 2. Subtle, delicate twilight clouds drifting across the upper/mid sky
local p_cloud_slate = pile{{"lead white", 18}, {"smalt", 4.5}, {"raw umber", 1.8}, {"vermilion", 0.5}, {"bone black", 0.2}, medium=0.4}
local p_cloud_amber = pile{{"lead white", 22}, {"yellow ochre", 1.5}, {"vermilion", 0.8}, {"raw umber", 0.6}, {"smalt", 0.8}, medium=0.45}

local b_filb = brush("filbert", 12)
b_filb:load(p_cloud_slate, 0.35)
b_filb:stroke({{50, 140}, {250, 138}, {500, 135}, {750, 138}, {950, 135}}, {pressure={0.15, 0.04}, ramps={0.1, 0.2}})
b_filb:stroke({{180, 175}, {400, 172}, {680, 170}, {880, 175}}, {pressure={0.18, 0.05}, ramps={0.1, 0.2}})

b_filb:reload(p_cloud_amber, 0.35)
b_filb:stroke({{80, 260}, {300, 255}, {580, 252}, {820, 258}}, {pressure={0.18, 0.05}, ramps={0.1, 0.2}})
b_filb:stroke({{220, 310}, {460, 306}, {720, 304}, {920, 312}}, {pressure={0.15, 0.04}, ramps={0.1, 0.2}})

blend(rect(0, 120, 1000, 210), {angle=0})

-- 3. Distant low hills across the bay
local p_dist_hills = pile{{"lead white", 14}, {"smalt", 5}, {"raw umber", 2.5}, {"red earth", 0.8}, {"bone black", 0.4}, medium=0.3}
local pts_hills = {
  {40, 468}, {140, 464}, {250, 458}, {360, 461}, {480, 455}, {600, 457}, {720, 453}, {840, 460}, {960, 468},
  {960, 475}, {40, 475}
}
local m_hills = poly(pts_hills):soften(2)
work(m_hills, {hand="body", pile=p_dist_hills, angle=0, coverage=1.2, fill=true, clip=true})

-- 4. Distant water surface (Bodden) and glowing mist
local p_water_sheen = pile{{"lead white", 22}, {"yellow ochre", 2.2}, {"smalt", 2.0}, {"raw umber", 0.5}, {"vermilion", 0.2}, medium=0.35}
local p_mist = pile{{"lead white", 28}, {"yellow ochre", 1.8}, {"smalt", 1.2}, {"raw umber", 0.3}, medium=0.45}

local m_water = rect(0, 465, 1000, 22):soften(2)
work(m_water, {hand="glaze", pile=p_water_sheen, angle=0, coverage=1.2, fill=true})

local m_mist_band = rect(0, 460, 1000, 18):soften(5)
work(m_mist_band, {hand="glaze", pile=p_mist, angle=0, coverage=0.9, fill=false})
blend(rect(0, 458, 1000, 25), {angle=0})

print("Sky smoothed, clouds drifted, distant hills and glowing mist painted.")

--@ chunk 28
-- Re-establish foreground knoll crest and sculpt the Megalithic Tomb (Dolmen)

local p_knoll_crest = pile{{"raw umber", 9}, {"bone black", 4}, {"yellow ochre", 5}, {"red earth", 2.5}, {"green earth", 3}, medium=0.2}
local p_chamber_dark = pile{{"bone black", 10}, {"raw umber", 8}, {"smalt", 1.5}, medium=0.15}
local p_stone_shadow = pile{{"raw umber", 8}, {"bone black", 5}, {"smalt", 2}, {"lead white", 2}, {"red earth", 1}, medium=0.2}
local p_stone_body = pile{{"lead white", 10}, {"raw umber", 5}, {"bone black", 2.5}, {"yellow ochre", 2.5}, {"smalt", 1.5}, medium=0.2}
local p_stone_light = pile{{"lead white", 20}, {"yellow ochre", 3}, {"smalt", 1.5}, {"raw umber", 1}, {"vermilion", 0.2}, medium=0.25}
local p_lichen_sage = pile{{"lead white", 15}, {"green earth", 5}, {"yellow ochre", 3.5}, {"raw umber", 1}, medium=0.25}
local p_lichen_gold = pile{{"lead white", 10}, {"yellow ochre", 8}, {"chrome yellow", 1.5}, {"raw umber", 1}, medium=0.25}

-- 1. Knoll Crest meeting the mist with crisp, undulating turf edge
local pts_knoll_crest = {
  {0, 486}, {80, 481}, {160, 472}, {240, 458}, {320, 446},
  {380, 440}, {440, 442}, {520, 455}, {620, 466}, {700, 464},
  {760, 456}, {840, 460}, {920, 470}, {1000, 478},
  {1000, 520}, {0, 520}
}
local m_knoll_top = poly(pts_knoll_crest, true)
work(m_knoll_top, {hand="body", pile=p_knoll_crest, angle=0.2, coverage=1.6, fill=true, clip=true})

-- 2. The Dolmen Burial Chamber Void (shadow within)
local pts_chamber = {
  {340, 431}, {452, 431}, {450, 463}, {338, 464}
}
local m_chamber = poly(pts_chamber)
work(m_chamber, {hand="detail", pile=p_chamber_dark, angle=1.2, coverage=1.6, fill=true, clip=true})

-- 3. Center-Rear Orthostat (recessed inside chamber)
local pts_rear_orth = {{384, 430}, {404, 430}, {406, 460}, {384, 460}}
local m_rear_orth = poly(pts_rear_orth)
work(m_rear_orth, {hand="detail", pile=p_stone_body, angle=1.5, coverage=1.4, fill=true, clip=true})
local m_rear_sh = poly({{384, 430}, {394, 430}, {392, 460}, {384, 460}})
work(m_rear_sh, {hand="detail", pile=p_stone_shadow, angle=1.5, coverage=1.3, fill=true, clip=true})

-- 4. Left Orthostat (front pillar)
local pts_l_orth = {{338, 430}, {362, 431}, {365, 445}, {363, 464}, {350, 465}, {336, 463}, {335, 442}}
local m_l_orth = poly(pts_l_orth, true)
work(m_l_orth, {hand="detail", pile=p_stone_body, angle=1.4, coverage=1.5, fill=true, clip=true})
-- Light outer facet catching twilight
local pts_l_light = {{336, 430}, {348, 431}, {349, 464}, {336, 463}, {335, 442}}
local m_l_light = poly(pts_l_light, true)
work(m_l_light, {hand="detail", pile=p_stone_light, angle=1.3, coverage=1.4, fill=true, clip=true})

-- 5. Right Orthostat (front pillar)
local pts_r_orth = {{428, 431}, {452, 432}, {454, 448}, {452, 464}, {438, 465}, {426, 463}, {425, 445}}
local m_r_orth = poly(pts_r_orth, true)
work(m_r_orth, {hand="detail", pile=p_stone_body, angle=1.4, coverage=1.5, fill=true, clip=true})
local pts_r_light = {{440, 432}, {452, 432}, {454, 448}, {452, 464}, {440, 464}}
local m_r_light = poly(pts_r_light, true)
work(m_r_light, {hand="detail", pile=p_stone_light, angle=1.3, coverage=1.4, fill=true, clip=true})

-- 6. The Massive Capstone
local pts_capstone = {
  {324, 428}, {330, 420}, {350, 415}, {385, 412}, {425, 413}, {455, 417}, {472, 424},
  {474, 432}, {455, 435}, {415, 431}, {375, 432}, {336, 435}, {324, 428}
}
local m_capstone = poly(pts_capstone, true)
work(m_capstone, {hand="body", pile=p_stone_body, angle=0.1, coverage=1.5, fill=true, clip=true})

-- Underside shadow band of capstone
local pts_cap_sh = {
  {324, 428}, {336, 435}, {375, 432}, {415, 431}, {455, 435}, {474, 432},
  {470, 436}, {420, 434}, {370, 435}, {330, 437}, {324, 428}
}
local m_cap_sh = poly(pts_cap_sh)
work(m_cap_sh, {hand="detail", pile=p_stone_shadow, angle=0, coverage=1.4, fill=true, clip=true})

-- Top light plane of capstone catching sky
local pts_cap_top = {
  {324, 428}, {330, 420}, {350, 415}, {385, 412}, {425, 413}, {455, 417}, {472, 424},
  {468, 426}, {435, 422}, {395, 420}, {355, 422}, {328, 427}
}
local m_cap_top = poly(pts_cap_top, true)
work(m_cap_top, {hand="detail", pile=p_stone_light, angle=0.05, coverage=1.4, fill=true, clip=true})

-- Lichen patches on capstone
local m_lich_s = (ellipse(370, 418, 14, 6) + ellipse(435, 417, 16, 7)):soften(2)
work(m_lich_s, {hand="detail", pile=p_lichen_sage, angle=0.2, coverage=1.2, fill=false})
local m_lich_g = ellipse(405, 416, 12, 5):soften(2)
work(m_lich_g, {hand="detail", pile=p_lichen_gold, angle=-0.2, coverage=1.2, fill=false})

-- 7. Surrounding Erratic Boulders
-- Leaning stone left
local pts_lean = {{305, 458}, {312, 444}, {326, 442}, {336, 445}, {336, 458}, {324, 465}, {305, 462}}
local m_lean = poly(pts_lean, true)
work(m_lean, {hand="detail", pile=p_stone_body, angle=0.4, coverage=1.4, fill=true, clip=true})
local m_lean_l = poly({{312, 444}, {326, 442}, {336, 445}, {330, 451}, {316, 450}})
work(m_lean_l, {hand="detail", pile=p_stone_light, angle=0.2, coverage=1.3, fill=true, clip=true})

-- Boulder right of dolmen
local pts_rb = {{474, 454}, {484, 443}, {502, 441}, {518, 445}, {524, 456}, {512, 464}, {485, 465}, {474, 454}}
local m_rb = poly(pts_rb, true)
work(m_rb, {hand="detail", pile=p_stone_body, angle=0.2, coverage=1.4, fill=true, clip=true})
local m_rb_l = poly({{484, 443}, {502, 441}, {518, 445}, {512, 450}, {488, 449}})
work(m_rb_l, {hand="detail", pile=p_stone_light, angle=0.1, coverage=1.3, fill=true, clip=true})

-- 8. Bedding stones into earth: turf contact shadows and grass tufts
local b_round = brush{kind="round", width=2.4, point=1, stiffness=0.6}
local b_fine = brush{kind="round", width=1.1, point=1, stiffness=0.8}
local p_turf_dark = pile{{"raw umber", 9}, {"bone black", 6}, {"green earth", 3}, medium=0.15}
local p_dry_grass = pile{{"yellow ochre", 9}, {"raw umber", 4}, {"lead white", 6}, {"red earth", 1}, medium=0.2}

b_round:load(p_turf_dark, 0.7)
b_round:stroke({{300, 463}, {338, 465}}, {pressure={0.8, 0.4}})
b_round:stroke({{328, 464}, {370, 465}}, {pressure={0.8, 0.5}})
b_round:stroke({{420, 464}, {460, 465}}, {pressure={0.8, 0.5}})
b_round:stroke({{470, 465}, {528, 465}}, {pressure={0.8, 0.4}})

-- Grass blades overlapping stone bases
b_fine:load(p_dry_grass, 0.6)
local g_pts = {
  {{332, 468}, {334, 455}}, {{337, 467}, {340, 453}}, {{344, 468}, {343, 456}},
  {{358, 468}, {360, 455}}, {{366, 467}, {368, 458}}, {{425, 467}, {427, 454}},
  {{432, 468}, {431, 455}}, {{450, 467}, {452, 454}}, {{456, 468}, {458, 458}},
  {{312, 467}, {314, 456}}, {{322, 468}, {324, 458}}, {{480, 467}, {482, 455}},
  {{505, 466}, {507, 455}}, {{518, 466}, {520, 456}}
}
for _, g in ipairs(g_pts) do
  b_fine:load(p_dry_grass, 0.6)
  b_fine:stroke(g, {pressure={0.55, 0.1}, ramps={0.05, 0.3}})
end

print("Knoll crest re-established and Megalithic Tomb sculpted.")

--@ chunk 29
-- Piles for tree construction
p_trunk_solid = pile{{"bone black", 11}, {"raw umber", 9}, {"red earth", 2}, {"smalt", 1}, medium=0.15}
p_stag_wood = pile{{"lead white", 14}, {"raw umber", 4}, {"yellow ochre", 2}, {"bone black", 1.2}, medium=0.2}
p_stag_shade = pile{{"raw umber", 8}, {"bone black", 4}, {"lead white", 4}, {"yellow ochre", 2}, medium=0.2}
p_bark_furrow = pile{{"raw umber", 8}, {"bone black", 5}, {"yellow ochre", 2.5}, {"lead white", 1.5}, medium=0.2}
p_bark_ridge = pile{{"raw umber", 6}, {"yellow ochre", 5}, {"lead white", 4}, {"bone black", 1.5}, medium=0.2}

-- 1. Ancient Oak Trunk & Bole Mask
local pts_trunk = {
  {250, 468}, {258, 445}, {265, 415}, {268, 380}, {266, 350}, {258, 315}, {254, 285},
  {270, 280}, {280, 295}, {294, 320}, {298, 352}, {302, 380}, {306, 415}, {312, 442},
  {324, 464}, {305, 466}, {280, 465}, {250, 468}
}
local m_trunk = poly(pts_trunk, true)
work(m_trunk, {hand="body", pile=p_trunk_solid, angle=1.4, coverage=2.5, fill=true, clip=true})

-- Trunk hollow
local pts_hollow_inner = {{276, 434}, {290, 430}, {292, 396}, {282, 392}, {275, 418}}
local m_hollow_in = poly(pts_hollow_inner, true)
work(m_hollow_in, {hand="detail", pile=p_trunk_solid, angle=1.5, coverage=1.8, fill=true, clip=true})

-- 2. Eastern Bough and Sub-branches (Ribbons)
local m_eb_main = ribbon({
  {292, 350}, {330, 342}, {370, 330}, {415, 323}, {460, 313}, {495, 292}, {528, 270}
}, {18, 14, 11, 8.5, 6.5, 5, 3.5})
local m_eb_1 = ribbon({{370, 330}, {384, 300}, {400, 274}, {414, 248}, {424, 224}}, {7, 5.5, 4.2, 3.2, 2.2})
local m_eb_1_sub = ribbon({{400, 274}, {390, 252}, {384, 234}}, {3.5, 2.5, 1.8})
local m_eb_2 = ribbon({{425, 322}, {444, 294}, {466, 272}, {485, 246}}, {5.5, 4.2, 3.2, 2.2})
local m_eb_3 = ribbon({{460, 313}, {478, 334}, {496, 352}, {514, 370}}, {4.5, 3.5, 2.5, 1.8})
local m_eb_tip = ribbon({{528, 270}, {550, 255}, {572, 242}}, {3.5, 2.5, 1.8})

local m_ebough_all = m_eb_main + m_eb_1 + m_eb_1_sub + m_eb_2 + m_eb_3 + m_eb_tip
work(m_ebough_all, {hand="body", pile=p_trunk_solid, angle=0.2, coverage=2.2, fill=true, clip=true})

-- 3. Western Limb and Sub-branches (Ribbons)
local m_wb_main = ribbon({
  {266, 345}, {236, 330}, {206, 310}, {176, 292}, {146, 280}, {116, 272}, {86, 268}
}, {16, 13, 10, 7.5, 5.5, 4, 2.5})
local m_wb_1 = ribbon({{206, 310}, {194, 278}, {182, 250}, {166, 224}, {150, 202}}, {6.5, 5.2, 4.0, 3.0, 2.0})
local m_wb_1_sub = ribbon({{182, 250}, {196, 226}, {202, 205}}, {3.2, 2.2, 1.5})
local m_wb_2 = ribbon({{176, 292}, {162, 316}, {142, 336}, {122, 350}}, {5.0, 3.8, 2.8, 1.8})
local m_wb_tip = ribbon({{116, 272}, {100, 252}, {86, 236}}, {3.0, 2.2, 1.5})

local m_wbough_all = m_wb_main + m_wb_1 + m_wb_1_sub + m_wb_2 + m_wb_tip
work(m_wbough_all, {hand="body", pile=p_trunk_solid, angle=-0.3, coverage=2.2, fill=true, clip=true})

-- 4. Central Trunk & Stag-Headed Antlers (Bleached dead wood)
local m_stag_stem = ribbon({
  {268, 305}, {270, 265}, {268, 225}, {264, 185}, {258, 145}, {254, 118}
}, {13, 10, 7.5, 5.5, 3.8, 2.2})
local m_sh1 = ribbon({{268, 225}, {288, 194}, {308, 166}, {320, 140}, {326, 116}}, {5.5, 4.2, 3.2, 2.2, 1.5})
local m_sh1_f = ribbon({{288, 194}, {280, 166}, {274, 140}}, {3.0, 2.2, 1.4})
local m_sh2 = ribbon({{270, 265}, {250, 234}, {232, 206}, {218, 178}, {208, 150}}, {5.8, 4.5, 3.5, 2.4, 1.5})
local m_sh2_f = ribbon({{232, 206}, {242, 180}, {246, 156}}, {2.8, 2.0, 1.3})
local m_sh3 = ribbon({{272, 280}, {295, 256}, {322, 230}, {342, 206}, {356, 180}}, {6.2, 4.8, 3.6, 2.6, 1.6})
local m_sh3_f = ribbon({{322, 230}, {335, 202}, {340, 174}}, {2.8, 2.0, 1.3})

local m_stag_all = m_stag_stem + m_sh1 + m_sh1_f + m_sh2 + m_sh2_f + m_sh3 + m_sh3_f
work(m_stag_all, {hand="body", pile=p_stag_wood, angle=1.4, coverage=2.2, fill=true, clip=true})

-- Stag horn underside shading
work(m_stag_all, {hand="detail", pile=p_stag_shade, angle=1.2, coverage=1.1, fill=false})

-- Bridge living trunk bark up into stag crown base
local pts_bridge = {{256, 320}, {262, 285}, {268, 265}, {274, 265}, {284, 288}, {292, 320}}
local m_bridge = poly(pts_bridge, true)
work(m_bridge, {hand="body", pile=p_trunk_solid, angle=1.4, coverage=2.0, fill=true, clip=true})

-- 5. Companion Tree (Right Knoll, x ≈ 770)
local m_t2_trunk = ribbon({
  {778, 465}, {775, 430}, {770, 395}, {764, 360}, {756, 325}, {748, 290}, {740, 255}, {732, 226}, {724, 202}
}, {12, 10.5, 9, 7.5, 6, 4.5, 3.5, 2.5, 1.8})
local m_t2_l = ribbon({{770, 395}, {746, 370}, {722, 345}, {698, 325}, {675, 310}}, {6, 4.8, 3.6, 2.6, 1.8})
local m_t2_r = ribbon({{764, 360}, {788, 340}, {816, 320}, {844, 302}, {870, 288}}, {5.5, 4.4, 3.4, 2.4, 1.6})

local m_t2_all = m_t2_trunk + m_t2_l + m_t2_r
work(m_t2_all, {hand="body", pile=p_trunk_solid, angle=1.2, coverage=2.2, fill=true, clip=true})

print("Ancient Oak and Companion Tree scaffolds solidly constructed.")

--@ chunk 30
-- Unify and richly model the entire foreground terrain from knoll crest to bottom

local p_earth_dark = pile{{"raw umber", 9}, {"bone black", 5}, {"red earth", 2.5}, {"green earth", 2.5}, {"yellow ochre", 3}, medium=0.2}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5}, {"raw umber", 5}, {"bone black", 1}, {"lead white", 2}, medium=0.2}
local p_moss_olive = pile{{"green earth", 7}, {"yellow ochre", 5}, {"raw umber", 3.5}, {"lead white", 2.5}, {"bone black", 1}, medium=0.2}
local p_sand_path = pile{{"yellow ochre", 8}, {"lead white", 6}, {"raw umber", 4}, {"red earth", 1.5}, medium=0.2}

-- 1. Unify the knoll slope down across the previous horizontal line at y = 520
local pts_fg_slope = {
  {0, 480}, {180, 470}, {350, 455}, {550, 465}, {740, 460}, {1000, 475},
  {1000, 705}, {0, 705}
}
local m_fg_slope = poly(pts_fg_slope, true)
work(m_fg_slope, {hand="body", pile=p_earth_dark, angle=0.2, coverage=1.4, fill=true})

-- 2. Late-autumn russet heather banks and dried bracken swells
local m_heather1 = ellipse(220, 580, 180, 60):soften(20)
local m_heather2 = ellipse(750, 570, 200, 70):soften(25)
local m_heather3 = ellipse(480, 630, 220, 60):soften(20)
local m_heather_all = m_heather1 + m_heather2 + m_heather3
work(m_heather_all, {hand="scumble", pile=p_heath_russet, angle=-0.2, coverage=1.3, fill=true})

-- 3. Damp mossy hummocks in hollows
local m_moss1 = ellipse(380, 515, 90, 30):soften(15)
local m_moss2 = ellipse(620, 510, 100, 32):soften(15)
local m_moss3 = ellipse(140, 640, 110, 40):soften(20)
local m_moss4 = ellipse(860, 620, 120, 45):soften(20)
local m_moss_all = m_moss1 + m_moss2 + m_moss3 + m_moss4
work(m_moss_all, {hand="scumble", pile=p_moss_olive, angle=0.25, coverage=1.2, fill=true})

-- 4. Winding footpath through the heath up to the Hünengrab
local b_path = brush{kind="flat", width=12, stiffness=0.5}
b_path:load(p_sand_path, 0.4)
b_path:stroke({
  {660, 700}, {620, 640}, {570, 580}, {520, 530}, {460, 490}, {420, 465}
}, {pressure={0.25, 0.1}, ramps={0.05, 0.2}})

-- 5. Foreground Glacial Erratic Boulders
-- Boulder Left (x ≈ 210-275, y ≈ 520-550)
local pts_bould_l = {{210, 538}, {226, 520}, {258, 518}, {276, 528}, {278, 544}, {248, 552}, {215, 548}}
local m_bould_l = poly(pts_bould_l, true)
work(m_bould_l, {hand="detail", pile=p_stone_body, angle=0.3, coverage=1.5, fill=true, clip=true})
local m_bould_l_top = poly({{226, 520}, {258, 518}, {274, 526}, {246, 530}, {226, 526}})
work(m_bould_l_top, {hand="detail", pile=p_stone_light, angle=0.1, coverage=1.3, fill=true, clip=true})

-- Boulder Right (x ≈ 640-715, y ≈ 530-560)
local pts_bould_r = {{645, 545}, {666, 528}, {698, 526}, {716, 538}, {715, 552}, {682, 560}, {648, 555}}
local m_bould_r = poly(pts_bould_r, true)
work(m_bould_r, {hand="detail", pile=p_stone_body, angle=-0.2, coverage=1.5, fill=true, clip=true})
local m_bould_r_top = poly({{666, 528}, {698, 526}, {712, 534}, {686, 538}, {666, 534}})
work(m_bould_r_top, {hand="detail", pile=p_stone_light, angle=-0.1, coverage=1.3, fill=true, clip=true})

-- Deep contact shadows under boulders
local b_round = brush{kind="round", width=2.6, point=1, stiffness=0.7}
b_round:load(p_earth_dark, 0.7)
b_round:stroke({{208, 548}, {248, 554}, {280, 546}}, {pressure={0.8, 0.4}})
b_round:stroke({{642, 555}, {682, 562}, {718, 554}}, {pressure={0.8, 0.4}})

print("Foreground terrain fully unified, heather swells, path, and boulders painted.")

--@ chunk 31
-- Blend the open paint on the foreground slope to fuse the patches into continuous rolling heath
blend(rect(0, 480, 1000, 225), {angle=0.15})
blend(rect(0, 520, 1000, 185), {angle=-0.1})

print("Foreground heath blended into continuous undulating terrain.")

--@ chunk 32
-- Intricate crooked sympodial oak branching, stag antlers, and companion tree

local b_twig = brush{kind="round", width=2.0, point=1, stiffness=0.8}
local b_fine_twig = brush{kind="round", width=1.3, point=1, stiffness=0.8}
local b_stag_twig = brush{kind="round", width=1.6, point=1, stiffness=0.8}

-- 1. Fine Crooked Oak Twigs on Living Eastern Bough (arches over dolmen)
local eb_twigs = {
  -- Branch 1 upper fork (x ≈ 424, y ≈ 224)
  {{424, 224}, {432, 204}, {442, 186}},
  {{432, 204}, {425, 188}, {418, 172}},
  {{425, 188}, {432, 172}, {436, 156}},
  {{414, 248}, {426, 238}, {438, 224}},
  {{426, 238}, {434, 222}, {440, 208}},
  {{384, 234}, {376, 216}, {368, 198}},
  {{384, 234}, {392, 218}, {398, 198}},
  {{392, 218}, {386, 200}, {380, 185}},
  -- Branch 2 middle fork (x ≈ 485, y ≈ 246)
  {{485, 246}, {500, 226}, {514, 208}},
  {{500, 226}, {492, 208}, {486, 190}},
  {{492, 208}, {498, 192}, {502, 178}},
  {{466, 272}, {482, 260}, {496, 246}},
  {{482, 260}, {476, 244}, {470, 228}},
  -- Branch 3 drooping shelter branch (x ≈ 514, y ≈ 370)
  {{514, 370}, {526, 390}, {534, 408}},
  {{514, 370}, {506, 388}, {500, 406}},
  {{496, 352}, {510, 365}, {520, 380}},
  {{478, 334}, {488, 348}, {496, 362}},
  -- Branch 4 tip fork (x ≈ 572, y ≈ 242)
  {{572, 242}, {592, 228}, {612, 216}},
  {{592, 228}, {600, 212}, {606, 196}},
  {{592, 228}, {584, 214}, {578, 198}},
  {{550, 255}, {565, 242}, {578, 228}}
}

for _, tw in ipairs(eb_twigs) do
  b_twig:load(p_trunk_solid, 0.8)
  b_twig:stroke(tw, {pressure={0.8, 0.3}, ramps={0.05, 0.2}})
end

-- 2. Fine Crooked Oak Twigs on Living Western Limb (reaches left)
local wb_twigs = {
  -- Branch W1 upper fork (x ≈ 150, y ≈ 202)
  {{150, 202}, {138, 182}, {126, 162}},
  {{150, 202}, {160, 184}, {168, 165}},
  {{160, 184}, {154, 168}, {148, 152}},
  {{166, 224}, {152, 212}, {138, 198}},
  {{182, 250}, {170, 232}, {158, 214}},
  {{202, 205}, {212, 185}, {216, 165}},
  {{202, 205}, {194, 186}, {186, 168}},
  {{194, 186}, {198, 168}, {202, 152}},
  -- Branch W2 drooping fork (x ≈ 122, y ≈ 350)
  {{122, 350}, {108, 366}, {95, 382}},
  {{122, 350}, {132, 365}, {138, 380}},
  {{142, 336}, {132, 352}, {124, 368}},
  {{176, 292}, {164, 312}, {152, 328}},
  -- Branch W3 outer tip (x ≈ 86, y ≈ 236)
  {{86, 236}, {72, 218}, {56, 204}},
  {{86, 236}, {92, 215}, {96, 195}},
  {{92, 215}, {86, 198}, {80, 182}},
  {{100, 252}, {86, 242}, {72, 232}},
  {{86, 268}, {68, 265}, {50, 262}},
  {{86, 268}, {78, 282}, {68, 296}}
}

for _, tw in ipairs(wb_twigs) do
  b_twig:load(p_trunk_solid, 0.8)
  b_twig:stroke(tw, {pressure={0.8, 0.3}, ramps={0.05, 0.2}})
end

-- 3. Intricate Bleached Dead Antler Twigs on Stag Crown
local stag_antlers = {
  -- Central dead spire (x ≈ 254, y ≈ 118)
  {{254, 118}, {252, 95}, {248, 75}},
  {{254, 118}, {262, 100}, {268, 82}},
  {{262, 100}, {258, 85}, {254, 70}},
  -- Stag horn 1 right (x ≈ 326, y ≈ 116)
  {{326, 116}, {334, 94}, {340, 75}},
  {{326, 116}, {320, 96}, {314, 78}},
  {{320, 96}, {325, 80}, {328, 65}},
  {{274, 140}, {270, 118}, {265, 98}},
  {{270, 118}, {276, 102}, {280, 86}},
  -- Stag horn 2 left (x ≈ 208, y ≈ 150)
  {{208, 150}, {200, 130}, {192, 110}},
  {{208, 150}, {214, 132}, {218, 114}},
  {{246, 156}, {248, 136}, {252, 116}},
  -- Stag horn 3 upper right (x ≈ 356, y ≈ 180)
  {{356, 180}, {368, 160}, {376, 140}},
  {{356, 180}, {350, 162}, {344, 144}},
  {{340, 174}, {346, 152}, {352, 132}}
}

for _, ant in ipairs(stag_antlers) do
  b_stag_twig:load(p_stag_wood, 0.8)
  b_stag_twig:stroke(ant, {pressure={0.75, 0.3}, ramps={0.05, 0.2}})
end

-- 4. Companion Tree Branching & Windswept Twigs (Right Knoll, x ≈ 770)
local t2_all_twigs = {
  -- Left branches leaning toward dolmen
  {{675, 310}, {654, 294}, {636, 282}},
  {{675, 310}, {666, 290}, {658, 272}},
  {{666, 290}, {672, 274}, {676, 258}},
  {{698, 325}, {684, 306}, {672, 290}},
  {{722, 345}, {708, 318}, {698, 296}},
  -- Right branches spreading seaward
  {{870, 288}, {892, 274}, {912, 264}},
  {{870, 288}, {882, 270}, {888, 250}},
  {{882, 270}, {890, 252}, {896, 236}},
  {{844, 302}, {858, 282}, {868, 264}},
  {{816, 320}, {832, 295}, {842, 272}},
  -- Upper crown shoots
  {{724, 202}, {715, 180}, {708, 160}},
  {{724, 202}, {730, 180}, {736, 160}},
  {{732, 226}, {744, 204}, {752, 184}},
  {{740, 255}, {754, 234}, {762, 214}}
}

for _, tw in ipairs(t2_all_twigs) do
  b_twig:load(p_trunk_solid, 0.8)
  b_twig:stroke(tw, {pressure={0.75, 0.3}, ramps={0.05, 0.2}})
end

print("Intricate oak branching, stag antlers, and companion twigs painted.")

--@ chunk 33
-- Enrich root flares, trunk anatomy, crown density, and companion tree

local b_root = brush{kind="round", width=3.2, point=1, stiffness=0.7}
local b_branch = brush{kind="round", width=2.4, point=1, stiffness=0.7}
local b_twig = brush{kind="round", width=1.6, point=1, stiffness=0.8}
local b_fine = brush{kind="round", width=1.1, point=1, stiffness=0.8}

-- 1. Powerful Root Flares and Buttresses gripping the knoll
local roots = {
  -- Left claw
  {{258, 442}, {246, 456}, {232, 472}, {224, 478}},
  -- Front-left root
  {{268, 448}, {262, 462}, {254, 475}, {248, 482}},
  -- Center-front root
  {{282, 452}, {284, 468}, {286, 482}},
  -- Front-right root
  {{302, 446}, {308, 462}, {314, 475}},
  -- Right root toward dolmen
  {{314, 442}, {326, 455}, {338, 466}, {345, 470}}
}
for _, r in ipairs(roots) do
  b_root:load(p_trunk_solid, 0.85)
  b_root:stroke(r, {pressure={0.9, 0.4}, ramps={0.05, 0.2}})
end

-- 2. Epicormic Shoots and Knots on the Trunk
local epicormics = {
  {{262, 410}, {252, 405}, {244, 402}},
  {{252, 405}, {250, 395}, {248, 388}},
  {{304, 420}, {315, 415}, {322, 412}},
  {{315, 415}, {318, 405}, {322, 396}},
  {{265, 370}, {255, 362}, {246, 358}},
  {{298, 375}, {308, 368}, {316, 362}}
}
for _, ep in ipairs(epicormics) do
  b_fine:load(p_trunk_solid, 0.75)
  b_fine:stroke(ep, {pressure={0.7, 0.25}, ramps={0.05, 0.2}})
end

-- 3. Secondary Branches Filling the Crown Interior
local crown_fill = {
  -- Between trunk and eastern bough
  {{330, 342}, {346, 316}, {358, 290}, {366, 268}},
  {{346, 316}, {338, 295}, {332, 275}},
  -- Between eastern bough branches
  {{415, 323}, {430, 305}, {442, 285}, {452, 265}},
  {{444, 294}, {456, 280}, {464, 264}},
  {{460, 313}, {475, 298}, {488, 282}},
  -- Under eastern bough
  {{370, 330}, {390, 350}, {410, 366}, {425, 378}},
  {{390, 350}, {382, 368}, {378, 382}},
  -- Between trunk and western limb
  {{266, 345}, {248, 320}, {228, 296}, {210, 275}},
  {{248, 320}, {254, 298}, {258, 278}},
  -- Under western limb
  {{206, 310}, {192, 330}, {176, 348}, {162, 362}},
  {{192, 330}, {200, 348}, {204, 364}},
  -- Outer western fill
  {{146, 280}, {132, 262}, {118, 245}, {106, 230}},
  {{132, 262}, {138, 245}, {142, 228}}
}
for _, b in ipairs(crown_fill) do
  b_branch:load(p_trunk_solid, 0.8)
  b_branch:stroke(b, {pressure={0.8, 0.35}, ramps={0.05, 0.15}})
end

-- 4. Fine Twigs off Crown Fill Branches
local fill_twigs = {
  {{366, 268}, {374, 250}, {380, 234}},
  {{366, 268}, {360, 252}, {355, 236}},
  {{332, 275}, {326, 258}, {322, 242}},
  {{452, 265}, {460, 248}, {466, 232}},
  {{464, 264}, {472, 248}, {478, 232}},
  {{425, 378}, {435, 394}, {442, 408}},
  {{210, 275}, {202, 256}, {196, 238}},
  {{210, 275}, {218, 258}, {224, 240}},
  {{162, 362}, {152, 378}, {144, 392}},
  {{106, 230}, {98, 214}, {92, 198}},
  {{106, 230}, {114, 215}, {118, 200}}
}
for _, tw in ipairs(fill_twigs) do
  b_twig:load(p_trunk_solid, 0.75)
  b_twig:stroke(tw, {pressure={0.75, 0.25}, ramps={0.05, 0.2}})
end

-- 5. Enrich the Companion Tree (Right Knoll, x ≈ 770)
-- Thicken trunk and add muscular root base
b_root:load(p_trunk_solid, 0.85)
b_root:stroke({{776, 440}, {768, 460}, {760, 474}}, {pressure={0.85, 0.4}})
b_root:stroke({{780, 440}, {788, 460}, {796, 474}}, {pressure={0.85, 0.4}})

-- Additional companion tree branches
local t2_branches = {
  -- Left spreading limb
  {{764, 360}, {740, 335}, {715, 312}, {690, 295}},
  {{740, 335}, {732, 312}, {724, 290}},
  -- Right spreading limb
  {{756, 325}, {782, 305}, {810, 285}, {834, 268}},
  {{782, 305}, {792, 282}, {800, 260}},
  -- Upper crown branching
  {{748, 290}, {754, 255}, {760, 222}, {765, 192}},
  {{754, 255}, {744, 230}, {736, 206}},
  {{732, 226}, {720, 198}, {712, 172}},
  {{720, 198}, {728, 175}, {734, 154}}
}
for _, b in ipairs(t2_branches) do
  b_branch:load(p_trunk_solid, 0.8)
  b_branch:stroke(b, {pressure={0.8, 0.35}, ramps={0.05, 0.15}})
end

-- Companion tree fine twigs
local t2_fine = {
  {{690, 295}, {674, 280}, {660, 268}},
  {{690, 295}, {682, 278}, {676, 260}},
  {{834, 268}, {852, 252}, {868, 240}},
  {{834, 268}, {842, 248}, {848, 230}},
  {{765, 192}, {772, 172}, {778, 152}},
  {{765, 192}, {758, 170}, {752, 150}},
  {{712, 172}, {704, 152}, {698, 134}},
  {{734, 154}, {740, 136}, {744, 118}}
}
for _, tw in ipairs(t2_fine) do
  b_twig:load(p_trunk_solid, 0.75)
  b_twig:stroke(tw, {pressure={0.7, 0.25}, ramps={0.05, 0.2}})
end

print("Root flares, epicormic shoots, crown fill, and companion tree enriched.")

--@ chunk 34
-- Paint crisp, authentic marcescent autumn oak leaf clusters using individual leaf touches

local b_leaf = brush{kind="round", width=2.2, point=1, stiffness=0.7}

-- Leaf piles
local p_leaf_dark = pile{{"raw umber", 9}, {"red earth", 6}, {"bone black", 3}, {"yellow ochre", 2}, medium=0.2}
local p_leaf_russet = pile{{"red earth", 7}, {"yellow ochre", 7}, {"raw umber", 4}, {"lead white", 2}, medium=0.2}
local p_leaf_gold = pile{{"yellow ochre", 10}, {"lead white", 5}, {"chrome yellow", 1.5}, {"red earth", 2}, {"raw umber", 1}, medium=0.25}

-- Cluster centers along the living branches of the Ancient Oak
local oak_leaf_clusters = {
  -- Eastern Bough
  {380, 298, 12}, {400, 272, 14}, {418, 246, 12}, {430, 222, 10},
  {444, 294, 12}, {466, 272, 14}, {485, 246, 12}, {500, 226, 10},
  {476, 332, 12}, {496, 350, 14}, {514, 368, 12}, {526, 388, 10},
  {548, 255, 12}, {570, 242, 10}, {590, 228, 8},
  {346, 316, 10}, {358, 290, 10}, {366, 268, 10},
  -- Western Limb
  {194, 278, 12}, {182, 250, 14}, {166, 224, 12}, {150, 202, 10},
  {162, 316, 12}, {142, 336, 14}, {122, 350, 12}, {108, 366, 10},
  {100, 252, 12}, {86, 236, 10}, {72, 218, 8},
  {248, 320, 10}, {228, 296, 10}, {210, 275, 10}
}

-- Companion Tree leaf clusters
local comp_leaf_clusters = {
  {740, 335, 10}, {715, 312, 12}, {690, 295, 12}, {666, 290, 10},
  {782, 305, 10}, {810, 285, 12}, {834, 268, 12}, {852, 252, 10},
  {754, 255, 10}, {760, 222, 10}, {720, 198, 10}, {734, 154, 8}
}

local function paint_cluster(cx, cy, radius, n)
  for i = 1, n do
    local ang = rand(0, 6.28)
    local r = rand(0, radius)
    local x = cx + r * math.cos(ang)
    local y = cy + r * math.sin(ang)
    local drag_len = rand(1.2, 2.8)
    local drag_ang = rand(0.8, 1.8) -- hanging downward
    
    -- Pick color randomly with natural weighting
    local p
    local roll = rand(0, 1)
    if roll < 0.4 then
      p = p_leaf_dark
    elseif roll < 0.75 then
      p = p_leaf_russet
    else
      p = p_leaf_gold
    end
    
    b_leaf:load(p, 0.7)
    b_leaf:touch(x, y, {pressure=rand(0.4, 0.75), drag={drag_len, drag_ang}, twist=rand(-0.3, 0.3)})
  end
end

-- Paint Ancient Oak clusters
for _, cl in ipairs(oak_leaf_clusters) do
  paint_cluster(cl[1], cl[2], cl[3], 9)
end

-- Paint Companion Tree clusters
for _, cl in ipairs(comp_leaf_clusters) do
  paint_cluster(cl[1], cl[2], cl[3], 8)
end

print("Authentic marcescent oak leaf clusters painted touch by touch.")

--@ chunk 35
-- Foreground glacial boulders, thistle skeletons, and windswept bent-grass tufts

local b_round = brush{kind="round", width=2.4, point=1, stiffness=0.7}
local b_fine = brush{kind="round", width=1.1, point=1, stiffness=0.8}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.8}

local p_bould_sh = pile{{"bone black", 8}, {"raw umber", 8}, {"smalt", 2}, {"lead white", 1.5}, medium=0.15}
local p_bould_mid = pile{{"lead white", 10}, {"raw umber", 6}, {"bone black", 3}, {"yellow ochre", 2.5}, {"smalt", 2}, medium=0.2}
local p_bould_light = pile{{"lead white", 20}, {"smalt", 2.5}, {"yellow ochre", 2.5}, {"raw umber", 1}, medium=0.25}
local p_straw = pile{{"lead white", 12}, {"yellow ochre", 10}, {"raw umber", 3}, {"red earth", 1}, medium=0.2}
local p_straw_lit = pile{{"lead white", 18}, {"yellow ochre", 6}, {"chrome yellow", 1}, {"raw umber", 0.8}, medium=0.25}
local p_thistle_dark = pile{{"bone black", 9}, {"raw umber", 8}, {"red earth", 2}, medium=0.15}

-- 1. Re-sculpt Glacial Boulders with Crisp Facets
-- Left Boulder (x ≈ 210 to 280, y ≈ 520 to 555)
local pts_bl = {{210, 538}, {226, 520}, {258, 516}, {278, 526}, {280, 544}, {250, 554}, {215, 548}}
local m_bl = poly(pts_bl, true)
work(m_bl, {hand="detail", pile=p_bould_mid, angle=0.3, coverage=1.6, fill=true, clip=true})
-- Light top plane
local pts_bl_top = {{226, 520}, {258, 516}, {276, 524}, {248, 528}, {226, 525}}
local m_bl_top = poly(pts_bl_top, true)
work(m_bl_top, {hand="detail", pile=p_bould_light, angle=0.1, coverage=1.4, fill=true, clip=true})
-- Under shadow and deep turf contact
b_round:load(p_bould_sh, 0.8)
b_round:stroke({{206, 546}, {248, 556}, {284, 548}}, {pressure={0.85, 0.4}})
-- Granite facet cracks
b_fine:load(p_bould_sh, 0.6)
b_fine:stroke({{248, 517}, {250, 530}, {246, 544}}, {pressure={0.4, 0.1}})

-- Right Boulder (x ≈ 640 to 720, y ≈ 530 to 565)
local pts_br = {{642, 546}, {664, 528}, {698, 525}, {718, 536}, {716, 552}, {684, 562}, {646, 556}}
local m_br = poly(pts_br, true)
work(m_br, {hand="detail", pile=p_bould_mid, angle=-0.2, coverage=1.6, fill=true, clip=true})
local pts_br_top = {{664, 528}, {698, 525}, {714, 532}, {688, 536}, {664, 532}}
local m_br_top = poly(pts_br_top, true)
work(m_br_top, {hand="detail", pile=p_bould_light, angle=-0.1, coverage=1.4, fill=true, clip=true})
b_round:load(p_bould_sh, 0.8)
b_round:stroke({{638, 554}, {684, 564}, {720, 555}}, {pressure={0.85, 0.4}})
b_fine:load(p_bould_sh, 0.6)
b_fine:stroke({{692, 526}, {690, 540}, {686, 552}}, {pressure={0.4, 0.1}})

-- Center-Left Foreground Boulder (x ≈ 420-475, y ≈ 620-655)
local pts_bc = {{425, 638}, {442, 622}, {468, 624}, {476, 638}, {458, 652}, {428, 646}}
local m_bc = poly(pts_bc, true)
work(m_bc, {hand="detail", pile=p_bould_mid, angle=0.2, coverage=1.5, fill=true, clip=true})
local pts_bc_top = {{442, 622}, {468, 624}, {464, 634}, {440, 632}}
local m_bc_top = poly(pts_bc_top, true)
work(m_bc_top, {hand="detail", pile=p_bould_light, angle=0.1, coverage=1.3, fill=true, clip=true})
b_round:load(p_bould_sh, 0.8)
b_round:stroke({{422, 646}, {456, 654}, {480, 642}}, {pressure={0.85, 0.4}})

-- 2. Thistle / Dried Carline Stalks along the Knoll Crest against the Mist
local thistle_stalks = {
  -- Group 1 on left knoll (x ≈ 170-195)
  {{{175, 470}, {174, 452}, {172, 436}}, {{172, 442}, {166, 434}}, {{172, 442}, {178, 435}}},
  {{{186, 468}, {187, 448}, {188, 432}}, {{187, 440}, {194, 432}}},
  -- Group 2 near saddle (x ≈ 540-565)
  {{{546, 462}, {545, 444}, {544, 428}}, {{545, 436}, {538, 428}}, {{545, 436}, {552, 430}}},
  {{{558, 465}, {560, 446}, {562, 430}}, {{560, 438}, {568, 431}}},
  -- Group 3 on right knoll (x ≈ 840-865)
  {{{848, 460}, {847, 442}, {846, 426}}, {{847, 434}, {840, 426}}, {{847, 434}, {854, 428}}},
  {{{860, 464}, {862, 445}, {864, 430}}, {{862, 438}, {870, 432}}}
}
for _, grp in ipairs(thistle_stalks) do
  for _, st in ipairs(grp) do
    b_fine:load(p_thistle_dark, 0.75)
    b_fine:stroke(st, {pressure={0.65, 0.15}, ramps={0.05, 0.2}})
  end
end

-- 3. Tufts of Bent-Grass (Festuca) across the knoll crest and around stones
local function paint_grass_tuft(base_x, base_y, n_blades, height, spread)
  for i = 1, n_blades do
    local dx = rand(-spread, spread)
    local curve_x = dx * rand(1.1, 1.6)
    local dy = -height * rand(0.7, 1.2)
    local mid_y = dy * 0.55
    local tip_x = base_x + curve_x
    local tip_y = base_y + dy
    local mid_x = base_x + dx * 0.6
    
    local pts = {{base_x, base_y}, {mid_x, base_y + mid_y}, {tip_x, tip_y}}
    local p = (rand(0, 1) < 0.6) and p_straw or p_straw_lit
    b_rigger:load(p, 0.65)
    b_rigger:stroke(pts, {pressure={0.6, 0.05}, ramps={0.05, 0.4}})
  end
end

-- Crest grass tufts silhouetted against glowing mist
local crest_tuft_locs = {
  {60, 482}, {110, 476}, {150, 468}, {210, 458}, {240, 452},
  {350, 462}, {375, 460}, {415, 462}, {465, 462}, {490, 462},
  {525, 458}, {580, 465}, {630, 466}, {680, 464}, {730, 458},
  {810, 458}, {870, 462}, {930, 468}, {970, 474}
}
for _, loc in ipairs(crest_tuft_locs) do
  paint_grass_tuft(loc[1], loc[2], 7, 14, 8)
end

-- Grass tufts around boulders and tree roots
local boulder_tuft_locs = {
  {204, 546}, {220, 552}, {248, 556}, {276, 550}, {284, 542},
  {636, 554}, {658, 560}, {684, 564}, {714, 558}, {724, 550},
  {420, 646}, {444, 654}, {470, 652}, {482, 642},
  {230, 480}, {248, 484}, {286, 484}, {312, 478}, {340, 472},
  {756, 476}, {798, 476}
}
for _, loc in ipairs(boulder_tuft_locs) do
  paint_grass_tuft(loc[1], loc[2], 9, 16, 9)
end

-- Foreground grass tufts in heath
local fg_tuft_locs = {
  {120, 610}, {180, 660}, {290, 620}, {340, 670}, {530, 620},
  {600, 680}, {710, 640}, {790, 670}, {880, 610}, {940, 650}
}
for _, loc in ipairs(fg_tuft_locs) do
  paint_grass_tuft(loc[1], loc[2], 8, 18, 10)
end

print("Boulders re-sculpted, thistle skeletons drawn, and bent-grass tufts planted.")

--@ chunk 36
-- Sculpt heavy, muscular, twisting limbs on the Ancient Oak and weave foreground topography

local b_heavy = brush{kind="round", width=5.0, point=1, stiffness=0.8}
local b_limb = brush{kind="round", width=3.5, point=1, stiffness=0.8}
local b_knuckle = brush{kind="round", width=2.6, point=1, stiffness=0.8}
local b_stag_heavy = brush{kind="round", width=3.5, point=1, stiffness=0.7}
local b_stag_mid = brush{kind="round", width=2.4, point=1, stiffness=0.8}
local b_round = brush{kind="round", width=2.4, point=1, stiffness=0.7}

local p_tree_core = pile{{"bone black", 12}, {"raw umber", 8}, {"red earth", 2}, medium=0.15}
local p_stag_thick = pile{{"lead white", 14}, {"raw umber", 4}, {"yellow ochre", 2}, {"bone black", 1}, medium=0.2}

-- 1. Heavy Muscular Boughs of Ancient Oak
-- Eastern Bough main trunk-to-limb sweep with natural bow and knuckles
local eb_heavy_paths = {
  -- Main trunk collar through bough belly
  {{288, 350}, {315, 352}, {345, 345}, {375, 335}},
  {{370, 336}, {405, 330}, {440, 322}, {475, 310}},
  {{470, 312}, {500, 296}, {525, 278}, {550, 258}},
  -- Branch 1 upward muscular crook
  {{375, 335}, {380, 308}, {392, 278}, {406, 252}},
  {{404, 254}, {415, 230}, {426, 208}},
  {{392, 278}, {382, 256}, {374, 235}},
  -- Branch 2 middle crook
  {{438, 322}, {450, 298}, {468, 274}, {486, 250}},
  {{468, 274}, {478, 254}, {486, 232}},
  -- Drooping shelter bough curving over capstone
  {{445, 320}, {468, 340}, {492, 358}, {515, 375}},
  {{492, 358}, {505, 378}, {515, 396}},
  -- Branch 4 tip crook
  {{525, 278}, {545, 262}, {568, 246}, {588, 232}}
}
for _, pth in ipairs(eb_heavy_paths) do
  b_limb:load(p_tree_core, 0.9)
  b_limb:stroke(pth, {pressure={0.9, 0.5}, ramps={0.05, 0.15}})
end

-- Western Limb heavy muscular sweep
local wb_heavy_paths = {
  -- Collar and belly bow
  {{268, 345}, {242, 338}, {212, 320}, {182, 302}},
  {{184, 304}, {152, 290}, {122, 280}, {92, 274}},
  -- Upward muscular branch
  {{212, 320}, {198, 288}, {184, 258}, {168, 228}},
  {{184, 258}, {196, 232}, {205, 210}},
  {{168, 228}, {155, 206}, {142, 185}},
  -- Drooping muscular branch
  {{182, 302}, {166, 326}, {144, 348}, {124, 365}},
  {{144, 348}, {132, 368}, {122, 385}},
  -- Tip crook
  {{122, 280}, {104, 260}, {88, 242}, {72, 228}}
}
for _, pth in ipairs(wb_heavy_paths) do
  b_limb:load(p_tree_core, 0.9)
  b_limb:stroke(pth, {pressure={0.9, 0.5}, ramps={0.05, 0.15}})
end

-- 2. Heavy Bleached Antlers on the Stag Crown
local stag_heavy_paths = {
  -- Central dead trunk continuation
  {{268, 305}, {270, 265}, {268, 225}, {264, 182}},
  {{264, 182}, {258, 142}, {252, 115}},
  -- Stag horn 1 (right)
  {{268, 225}, {290, 192}, {312, 164}, {326, 138}},
  {{312, 164}, {324, 136}, {330, 112}},
  {{290, 192}, {282, 164}, {275, 138}},
  -- Stag horn 2 (left)
  {{270, 265}, {248, 232}, {228, 204}, {212, 175}},
  {{228, 204}, {216, 175}, {206, 148}},
  {{228, 204}, {240, 178}, {246, 152}},
  -- Stag horn 3 (upper right)
  {{272, 280}, {298, 254}, {326, 228}, {348, 204}},
  {{326, 228}, {346, 202}, {360, 178}},
  {{326, 228}, {338, 198}, {344, 170}}
}
for _, pth in ipairs(stag_heavy_paths) do
  b_stag_heavy:load(p_stag_thick, 0.85)
  b_stag_heavy:stroke(pth, {pressure={0.85, 0.45}, ramps={0.05, 0.15}})
end

-- 3. Weave Foreground Topography into Undulating Baltic Heath
local p_heath_deep = pile{{"raw umber", 9}, {"bone black", 5}, {"red earth", 3}, {"yellow ochre", 3}, medium=0.2}
local p_heath_warm = pile{{"yellow ochre", 7}, {"red earth", 5}, {"raw umber", 5}, {"lead white", 2.5}, medium=0.2}
local p_peat_dark = pile{{"bone black", 8}, {"raw umber", 8}, {"green earth", 3}, medium=0.15}
local p_bould_light = pile{{"lead white", 20}, {"smalt", 2.5}, {"yellow ochre", 2.5}, {"raw umber", 1}, medium=0.25}

local b_heath = brush{kind="flat", width=16, stiffness=0.6}

-- Layer overlapping horizontal and diagonal bands of heather and earth across the slope
local heath_bands = {
  -- High knoll terrace below dolmen
  {{200, 480}, {350, 475}, {500, 478}, {650, 482}},
  -- Mid-slope rolls
  {{50, 520}, {200, 515}, {380, 525}, {560, 530}},
  {{420, 535}, {600, 540}, {780, 535}, {950, 530}},
  -- Lower slope peat shelves and heather swells
  {{0, 570}, {180, 565}, {360, 575}, {540, 580}},
  {{380, 585}, {560, 590}, {760, 580}, {1000, 575}},
  -- Foreground peat depressions
  {{80, 630}, {260, 640}, {460, 635}, {660, 645}},
  {{480, 650}, {680, 655}, {880, 645}, {1000, 640}}
}

for i, band in ipairs(heath_bands) do
  local p = (i % 2 == 1) and p_heath_deep or p_heath_warm
  b_heath:load(p, 0.7)
  b_heath:stroke(band, {pressure={0.65, 0.35}, ramps={0.1, 0.15}})
end

-- Embed the boulders into the terrain with dark peat banks and hollows
local b_embed = brush{kind="round", width=4.0, point=1, stiffness=0.7}
b_embed:load(p_peat_dark, 0.8)
-- Bank behind left boulder
b_embed:stroke({{180, 525}, {220, 518}, {260, 516}, {295, 522}}, {pressure={0.8, 0.4}})
-- Bank behind right boulder
b_embed:stroke({{620, 532}, {660, 525}, {700, 524}, {735, 532}}, {pressure={0.8, 0.4}})
-- Bank behind center-left boulder
b_embed:stroke({{400, 626}, {440, 620}, {480, 622}, {500, 630}}, {pressure={0.8, 0.4}})

-- Re-state the boulders' crisp sky-lit facets
b_round:load(p_bould_light, 0.7)
b_round:stroke({{226, 520}, {258, 516}, {276, 524}}, {pressure={0.65, 0.3}})
b_round:stroke({{664, 528}, {698, 525}, {714, 532}}, {pressure={0.65, 0.3}})
b_round:stroke({{442, 622}, {468, 624}, {464, 634}}, {pressure={0.65, 0.3}})

print("Heavy muscular boughs sculpted, stag antlers thickened, and terrain woven.")

--@ chunk 37
-- Soften and fuse the ground strokes into smooth undulating heathland
blend(rect(0, 480, 1000, 225), {angle=0.2})
blend(rect(0, 520, 1000, 185), {angle=-0.15})

print("Ground blended into continuous natural slope.")

--@ chunk 38
-- Sculpt Ancient Oak Trunk with authentic longitudinal bark anatomy and masterfully render the Dolmen

local b_bark = brush{kind="round", width=2.2, point=1, stiffness=0.8}
local b_furrow = brush{kind="round", width=1.4, point=1, stiffness=0.8}
local b_fine = brush{kind="round", width=1.0, point=1, stiffness=0.8}
local b_root = brush{kind="round", width=3.6, point=1, stiffness=0.7}
local b_void = brush{kind="round", width=3.0, point=1, stiffness=0.8}

local p_bark_dark = pile{{"bone black", 11}, {"raw umber", 8}, {"red earth", 2}, medium=0.15}
local p_bark_mid = pile{{"raw umber", 8}, {"bone black", 4}, {"yellow ochre", 3.5}, {"lead white", 2.2}, {"red earth", 1}, medium=0.2}
local p_bark_light = pile{{"raw umber", 5}, {"yellow ochre", 5.5}, {"lead white", 5.5}, {"bone black", 1.2}, medium=0.2}
local p_hollow_void = pile{{"bone black", 12}, {"raw umber", 6}, medium=0.1}
local p_woundwood = pile{{"raw umber", 6}, {"yellow ochre", 5}, {"lead white", 3.5}, {"red earth", 2}, medium=0.2}

-- 1. Longitudinal Bark Modeling on Ancient Oak Bole (flowing from roots into boughs)
-- Dark furrow base strokes running vertically
local trunk_dark_strokes = {
  -- Left contour and flank
  {{246, 475}, {254, 450}, {262, 420}, {266, 385}, {264, 350}, {256, 320}, {248, 295}},
  -- Left of hollow
  {{258, 470}, {264, 442}, {268, 415}, {270, 385}, {268, 355}, {262, 325}},
  -- Passing above and around hollow
  {{270, 385}, {274, 360}, {274, 330}, {270, 305}},
  -- Right of hollow
  {{298, 465}, {296, 440}, {295, 415}, {290, 385}, {284, 355}, {278, 325}},
  -- Right contour and flank
  {{320, 468}, {312, 442}, {306, 415}, {302, 385}, {298, 355}, {292, 325}},
  -- Flowing into western limb
  {{262, 350}, {248, 340}, {230, 328}, {210, 314}},
  {{268, 360}, {252, 350}, {236, 338}, {218, 324}},
  -- Flowing into eastern bough
  {{295, 355}, {312, 350}, {335, 344}, {360, 336}},
  {{298, 365}, {318, 358}, {342, 350}, {368, 340}}
}
for _, s in ipairs(trunk_dark_strokes) do
  b_bark:load(p_bark_dark, 0.8)
  b_bark:stroke(s, {pressure={0.85, 0.5}, ramps={0.05, 0.1}})
end

-- Lit bark ridges running vertically between furrows
local trunk_ridge_strokes = {
  {{250, 465}, {258, 438}, {264, 410}, {267, 380}, {263, 348}, {255, 318}},
  {{262, 455}, {266, 430}, {270, 400}, {271, 375}, {269, 345}, {264, 315}},
  {{305, 458}, {302, 432}, {299, 402}, {295, 372}, {291, 340}},
  {{314, 455}, {308, 430}, {303, 400}, {300, 370}, {296, 340}},
  -- Collar sweeps into limbs
  {{260, 345}, {242, 335}, {224, 322}, {206, 310}},
  {{300, 350}, {320, 345}, {345, 338}, {370, 330}}
}
for _, s in ipairs(trunk_ridge_strokes) do
  b_furrow:load(p_bark_mid, 0.7)
  b_furrow:stroke(s, {pressure={0.7, 0.35}, ramps={0.05, 0.1}})
end

-- High-contrast bark ridge highlights
local trunk_high_strokes = {
  {{252, 460}, {260, 435}, {265, 408}, {267, 378}, {262, 345}},
  {{304, 452}, {301, 428}, {298, 398}, {294, 368}}
}
for _, s in ipairs(trunk_high_strokes) do
  b_fine:load(p_bark_light, 0.6)
  b_fine:stroke(s, {pressure={0.55, 0.2}, ramps={0.05, 0.1}})
end

-- 2. Sculpt the Hollow as an Irregular, Ancient Weathered Cavity
-- Deep rotted void
local pts_hollow_irreg = {
  {278, 438}, {284, 442}, {290, 435}, {293, 420}, {294, 405}, {291, 392},
  {285, 388}, {280, 395}, {275, 408}, {274, 422}
}
local m_hollow_irreg = poly(pts_hollow_irreg, true)
work(m_hollow_irreg, {hand="detail", pile=p_hollow_void, angle=1.4, coverage=1.8, fill=true, clip=true})

-- Wound-wood callus lip on the edges of the cavity
b_fine:load(p_woundwood, 0.7)
-- Left callus lip
b_fine:stroke({{274, 430}, {274, 415}, {276, 400}, {282, 390}}, {pressure={0.75, 0.3}})
-- Right callus lip
b_fine:stroke({{292, 432}, {294, 415}, {293, 400}, {288, 390}}, {pressure={0.75, 0.3}})
-- Highlight on the outer edge of left callus
b_fine:load(p_bark_light, 0.5)
b_fine:stroke({{273, 426}, {273, 412}, {275, 398}}, {pressure={0.5, 0.15}})

-- 3. Massive Root Buttresses Flaring into Turf
local p_turf_deep = pile{{"bone black", 8}, {"raw umber", 9}, {"green earth", 3}, medium=0.15}
local root_swells = {
  -- Left claw
  {{254, 445}, {242, 460}, {228, 474}, {220, 480}},
  -- Front-left root
  {{265, 448}, {260, 464}, {252, 478}, {244, 484}},
  -- Center-front root
  {{282, 452}, {284, 468}, {285, 484}},
  -- Right root toward dolmen
  {{310, 445}, {322, 458}, {334, 468}, {342, 472}}
}
for _, r in ipairs(root_swells) do
  b_root:load(p_bark_dark, 0.85)
  b_root:stroke(r, {pressure={0.9, 0.4}, ramps={0.05, 0.2}})
  -- Highlight along top of root ridge
  b_furrow:load(p_bark_mid, 0.6)
  b_furrow:stroke(r, {pressure={0.65, 0.2}, ramps={0.05, 0.2}})
end
-- Bed roots into dark soil
b_root:load(p_turf_deep, 0.8)
b_root:stroke({{216, 482}, {250, 486}, {290, 486}, {345, 474}}, {pressure={0.8, 0.4}})

-- 4. Masterfully Render the Dolmen (Hünengrab)
local p_cavern_pitch = pile{{"bone black", 12}, {"raw umber", 6}, {"smalt", 1}, medium=0.1}
local p_granite_mid = pile{{"lead white", 11}, {"raw umber", 6}, {"bone black", 3.5}, {"yellow ochre", 2.2}, {"smalt", 2}, medium=0.2}
local p_granite_sh = pile{{"raw umber", 8}, {"bone black", 6}, {"smalt", 2}, {"lead white", 1.5}, medium=0.15}
local p_granite_lit = pile{{"lead white", 22}, {"yellow ochre", 3}, {"smalt", 1.5}, {"raw umber", 0.8}, {"vermilion", 0.2}, medium=0.22}

-- A. Deep Pitch Darkness in the Chamber Voids between Orthostats
-- Void between left orthostat (x ≈ 362) and center orthostat (x ≈ 384)
local m_void_left = poly({{362, 432}, {385, 432}, {385, 462}, {362, 463}})
work(m_void_left, {hand="detail", pile=p_cavern_pitch, angle=1.2, coverage=1.8, fill=true, clip=true})

-- Void between center orthostat (x ≈ 406) and right orthostat (x ≈ 428)
local m_void_right = poly({{405, 432}, {428, 432}, {428, 462}, {405, 462}})
work(m_void_right, {hand="detail", pile=p_cavern_pitch, angle=1.2, coverage=1.8, fill=true, clip=true})

-- B. Orthostats Sculpted with Solid Facets
-- Left Orthostat
local pts_lo = {{338, 431}, {362, 431}, {364, 463}, {338, 463}}
local m_lo = poly(pts_lo)
work(m_lo, {hand="detail", pile=p_granite_mid, angle=1.4, coverage=1.5, fill=true, clip=true})
-- Shadow side facing the chamber
local m_lo_sh = poly({{352, 431}, {362, 431}, {364, 463}, {354, 463}})
work(m_lo_sh, {hand="detail", pile=p_granite_sh, angle=1.4, coverage=1.4, fill=true, clip=true})
-- Light outer edge
b_fine:load(p_granite_lit, 0.5)
b_fine:stroke({{338, 431}, {337, 448}, {338, 463}}, {pressure={0.5, 0.2}})

-- Center-Rear Orthostat
local pts_co = {{385, 430}, {405, 430}, {406, 460}, {385, 460}}
local m_co = poly(pts_co)
work(m_co, {hand="detail", pile=p_granite_sh, angle=1.4, coverage=1.4, fill=true, clip=true})

-- Right Orthostat
local pts_ro = {{428, 431}, {452, 432}, {452, 463}, {428, 463}}
local m_ro = poly(pts_ro)
work(m_ro, {hand="detail", pile=p_granite_mid, angle=1.4, coverage=1.5, fill=true, clip=true})
-- Shadow side facing chamber
local m_ro_sh = poly({{428, 431}, {438, 431}, {438, 463}, {428, 463}})
work(m_ro_sh, {hand="detail", pile=p_granite_sh, angle=1.4, coverage=1.4, fill=true, clip=true})
-- Light outer edge
b_fine:load(p_granite_lit, 0.5)
b_fine:stroke({{452, 432}, {453, 448}, {452, 463}}, {pressure={0.5, 0.2}})

-- C. The Massive Capstone Resurfaced
-- Full body pass to eliminate the yellow circle and establish granite tone
local pts_cap_clean = {
  {324, 428}, {332, 420}, {352, 415}, {385, 412}, {425, 413}, {455, 417}, {472, 424},
  {474, 432}, {455, 435}, {415, 431}, {375, 432}, {336, 435}, {324, 428}
}
local m_cap_clean = poly(pts_cap_clean, true)
work(m_cap_clean, {hand="body", pile=p_granite_mid, angle=0.08, coverage=1.6, fill=true, clip=true})

-- Deep cast shadow directly under capstone overhang projecting it forward
local pts_undercut = {
  {324, 428}, {336, 435}, {375, 432}, {415, 431}, {455, 435}, {474, 432},
  {470, 437}, {420, 435}, {370, 436}, {330, 438}, {324, 428}
}
local m_undercut = poly(pts_undercut)
work(m_undercut, {hand="detail", pile=p_cavern_pitch, angle=0, coverage=1.6, fill=true, clip=true})

-- Luminous top facet catching the pale twilight sky
local pts_cap_top_lit = {
  {324, 428}, {332, 420}, {352, 415}, {385, 412}, {425, 413}, {455, 417}, {472, 424},
  {468, 425}, {435, 420}, {395, 418}, {355, 420}, {328, 426}
}
local m_cap_top_lit = poly(pts_cap_top_lit, true)
work(m_cap_top_lit, {hand="detail", pile=p_granite_lit, angle=0.05, coverage=1.4, fill=true, clip=true})

-- Granite fractures, chips, and subtle natural lichen mottling
b_fine:load(p_granite_sh, 0.6)
b_fine:stroke({{370, 416}, {373, 424}, {370, 430}}, {pressure={0.4, 0.1}})
b_fine:stroke({{430, 416}, {434, 422}, {438, 430}}, {pressure={0.4, 0.1}})
-- Subtle pale grey-green crustose lichen (not yellow!)
local p_lich_cool = pile{{"lead white", 14}, {"green earth", 4.5}, {"yellow ochre", 2.5}, {"raw umber", 1}, medium=0.25}
local m_lich_cool = (ellipse(385, 418, 12, 5) + ellipse(445, 419, 14, 5)):soften(2)
work(m_lich_cool, {hand="detail", pile=p_lich_cool, angle=0.1, coverage=1.1, fill=false})

-- Bed orthostats into turf with dark earth and natural curling grasses
b_bark:load(p_turf_deep, 0.8)
b_bark:stroke({{330, 463}, {370, 464}}, {pressure={0.8, 0.5}})
b_bark:stroke({{422, 463}, {460, 464}}, {pressure={0.8, 0.5}})

print("Bark longitudinal furrows rendered, hollow sculpted, Dolmen cavern deepened and capstone resurfaced.")

--@ chunk 39
-- Define bark piles globally
p_bark_dark = pile{{"bone black", 11}, {"raw umber", 8}, {"red earth", 2}, medium=0.15}
p_bark_mid = pile{{"raw umber", 8}, {"bone black", 4}, {"yellow ochre", 3.5}, {"lead white", 2.2}, {"red earth", 1}, medium=0.2}
p_bark_light = pile{{"raw umber", 5}, {"yellow ochre", 5.5}, {"lead white", 5.5}, {"bone black", 1.2}, medium=0.2}
p_hollow_void = pile{{"bone black", 12}, {"raw umber", 6}, medium=0.1}
p_woundwood = pile{{"raw umber", 6}, {"yellow ochre", 5}, {"lead white", 3.5}, {"red earth", 2}, medium=0.2}
p_wood_rim = pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"raw umber", 1}, {"vermilion", 0.3}, medium=0.25}

local pts_trunk_full = {
  {248, 475}, {256, 445}, {264, 415}, {268, 380}, {266, 350}, {258, 315}, {252, 285},
  {270, 278}, {282, 295}, {294, 320}, {298, 352}, {304, 380}, {308, 415}, {314, 442},
  {326, 468}, {305, 472}, {280, 470}, {248, 475}
}
local m_trunk_full = poly(pts_trunk_full, true)

-- Work the trunk with pure vertical strokes (angle = 1.57) to create authentic vertical wood grain
work(m_trunk_full, {
  hand = "body",
  pile = p_bark_dark,
  angle = 1.57,
  length = {50, 100},
  coverage = 2.5,
  fill = true,
  clip = true
})

-- Carve a naturalistic, ragged hollow cavity
local pts_hollow_new = {
  {275, 440}, {282, 445}, {290, 438}, {294, 420}, {295, 402}, {290, 386},
  {284, 382}, {278, 390}, {273, 405}, {272, 424}
}
local m_hollow_new = poly(pts_hollow_new, true)
work(m_hollow_new, {hand="detail", pile=p_hollow_void, angle=1.57, coverage=2.0, fill=true, clip=true})

-- Wound-wood callus lip on the edges of the cavity
local b_callus = brush{kind="round", width=2.0, point=1, stiffness=0.8}
b_callus:load(p_woundwood, 0.8)
b_callus:stroke({{272, 435}, {272, 415}, {274, 398}, {280, 386}}, {pressure={0.8, 0.4}})
b_callus:stroke({{292, 436}, {294, 418}, {292, 400}, {286, 386}}, {pressure={0.8, 0.4}})

-- Flared root buttresses gripping the mound
local b_buttress = brush{kind="round", width=4.5, point=1, stiffness=0.7}
b_buttress:load(p_bark_dark, 0.9)
b_buttress:stroke({{260, 440}, {248, 458}, {232, 474}, {220, 482}}, {pressure={0.9, 0.5}})
b_buttress:stroke({{268, 445}, {262, 464}, {254, 478}, {246, 485}}, {pressure={0.9, 0.5}})
b_buttress:stroke({{282, 450}, {284, 468}, {285, 486}}, {pressure={0.85, 0.5}})
b_buttress:stroke({{308, 442}, {320, 458}, {334, 470}, {345, 475}}, {pressure={0.9, 0.5}})

-- Flow bark directly out into the eastern bough and western limb bases
local b_flow = brush{kind="round", width=3.8, point=1, stiffness=0.7}
b_flow:load(p_bark_dark, 0.85)
-- Into eastern bough
b_flow:stroke({{292, 345}, {315, 342}, {340, 335}, {370, 328}}, {pressure={0.9, 0.6}})
b_flow:stroke({{296, 358}, {320, 352}, {345, 344}, {372, 335}}, {pressure={0.85, 0.6}})
-- Into western limb
b_flow:stroke({{265, 340}, {245, 330}, {225, 318}, {200, 305}}, {pressure={0.9, 0.6}})
b_flow:stroke({{262, 352}, {242, 342}, {220, 328}, {195, 315}}, {pressure={0.85, 0.6}})

-- Fine vertical bark ridges in half-tone and highlight
local b_fine_ridge = brush{kind="round", width=1.5, point=1, stiffness=0.8}
b_fine_ridge:load(p_bark_mid, 0.7)
b_fine_ridge:stroke({{252, 462}, {258, 435}, {264, 405}, {266, 375}, {262, 340}, {256, 310}}, {pressure={0.65, 0.25}})
b_fine_ridge:stroke({{264, 450}, {268, 425}, {271, 395}, {270, 365}, {268, 330}}, {pressure={0.6, 0.25}})
b_fine_ridge:stroke({{304, 452}, {302, 425}, {298, 395}, {295, 365}, {292, 330}}, {pressure={0.6, 0.25}})
b_fine_ridge:stroke({{315, 452}, {310, 425}, {305, 395}, {301, 365}, {297, 330}}, {pressure={0.6, 0.25}})

-- Twilight rim-light on the left sky-facing contour of the bole
local b_rim = brush{kind="round", width=1.2, point=1, stiffness=0.8}
b_rim:load(p_wood_rim, 0.5)
b_rim:stroke({{248, 472}, {256, 445}, {264, 415}, {268, 380}, {266, 350}, {258, 315}, {252, 285}}, {pressure={0.4, 0.1}, ramps={0.05, 0.15}})

print("Ancient Oak trunk rebuilt with continuous vertical grain, flared buttresses, and seamless limb flow.")

--@ chunk 40
-- Rich branch weaving for both trees, foreground flora, and finishing twilight rim-lights

local b_limb = brush{kind="round", width=3.2, point=1, stiffness=0.8}
local b_branch = brush{kind="round", width=2.2, point=1, stiffness=0.8}
local b_twig = brush{kind="round", width=1.5, point=1, stiffness=0.8}
local b_fine = brush{kind="round", width=1.0, point=1, stiffness=0.8}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.8}

-- 1. Weave and Enrich the Canopy of the Ancient Oak (Twisting limbs & crossing boughs)
local oak_new_boughs = {
  -- Filling the space between trunk and eastern bough
  {{294, 335}, {312, 310}, {330, 285}, {348, 260}},
  {{312, 310}, {325, 292}, {338, 275}},
  {{348, 260}, {362, 240}, {374, 222}},
  {{348, 260}, {342, 242}, {338, 225}},
  -- Crossing over the middle of eastern bough
  {{375, 335}, {395, 315}, {412, 292}, {425, 268}},
  {{412, 292}, {404, 272}, {398, 252}},
  {{425, 268}, {438, 246}, {448, 225}},
  -- Reaching outward and upward
  {{440, 322}, {462, 305}, {480, 285}, {496, 262}},
  {{480, 285}, {472, 265}, {466, 246}},
  {{496, 262}, {512, 242}, {525, 224}},
  -- Drooping branches over dolmen right flank
  {{470, 335}, {490, 352}, {508, 370}, {524, 388}},
  {{490, 352}, {482, 370}, {476, 388}},
  -- Tip extensions
  {{545, 262}, {568, 248}, {590, 234}, {610, 222}},
  {{568, 248}, {576, 232}, {582, 216}},
  -- Filling western limb space
  {{262, 335}, {242, 310}, {220, 288}, {198, 268}},
  {{242, 310}, {250, 288}, {255, 268}},
  {{220, 288}, {212, 268}, {205, 248}},
  {{198, 268}, {184, 248}, {172, 228}},
  -- Drooping western branches
  {{170, 305}, {152, 328}, {134, 350}, {116, 370}},
  {{152, 328}, {160, 348}, {165, 368}},
  -- Outer western tip extensions
  {{104, 260}, {84, 245}, {66, 232}, {48, 222}},
  {{84, 245}, {90, 228}, {94, 212}},
  {{84, 245}, {76, 230}, {68, 216}}
}

for _, b in ipairs(oak_new_boughs) do
  b_branch:load(p_bark_dark, 0.85)
  b_branch:stroke(b, {pressure={0.85, 0.35}, ramps={0.05, 0.15}})
end

-- 2. Fine Crooked Oak Twiglets on New Boughs
local oak_new_twigs = {
  {{374, 222}, {382, 206}, {388, 190}},
  {{374, 222}, {366, 208}, {360, 192}},
  {{448, 225}, {456, 208}, {462, 192}},
  {{448, 225}, {440, 210}, {434, 195}},
  {{525, 224}, {534, 206}, {540, 188}},
  {{525, 224}, {518, 208}, {512, 192}},
  {{610, 222}, {622, 210}, {634, 200}},
  {{610, 222}, {616, 206}, {620, 190}},
  {{172, 228}, {164, 210}, {156, 194}},
  {{172, 228}, {178, 212}, {184, 196}},
  {{48, 222}, {38, 212}, {28, 204}},
  {{48, 222}, {54, 208}, {58, 194}}
}

for _, tw in ipairs(oak_new_twigs) do
  b_twig:load(p_bark_dark, 0.8)
  b_twig:stroke(tw, {pressure={0.75, 0.25}, ramps={0.05, 0.2}})
end

-- 3. Enrich the Companion Tree (Right Knoll, x ≈ 770)
local t2_enrich_boughs = {
  -- Left spreading arm
  {{756, 345}, {732, 320}, {708, 298}, {684, 280}},
  {{732, 320}, {724, 298}, {716, 276}},
  {{708, 298}, {696, 278}, {686, 258}},
  -- Right spreading arm
  {{752, 330}, {778, 310}, {805, 290}, {830, 272}},
  {{778, 310}, {788, 288}, {796, 266}},
  {{805, 290}, {818, 270}, {828, 250}},
  -- Upper crown
  {{744, 275}, {750, 240}, {756, 208}, {760, 178}},
  {{750, 240}, {740, 215}, {732, 190}},
  {{732, 226}, {720, 195}, {712, 168}},
  {{720, 195}, {728, 172}, {734, 150}}
}

for _, b in ipairs(t2_enrich_boughs) do
  b_branch:load(p_bark_dark, 0.85)
  b_branch:stroke(b, {pressure={0.8, 0.35}, ramps={0.05, 0.15}})
end

-- 4. Winding Sandy Footpath in Foreground
local p_sand_rut = pile{{"yellow ochre", 7}, {"lead white", 5}, {"raw umber", 4}, {"bone black", 1}, medium=0.2}
local p_sand_light = pile{{"lead white", 9}, {"yellow ochre", 8}, {"raw umber", 2.5}, {"vermilion", 0.3}, medium=0.22}

local b_rut = brush{kind="round", width=2.8, point=1, stiffness=0.7}
-- Left track of path
b_rut:load(p_sand_rut, 0.7)
b_rut:stroke({{675, 700}, {632, 645}, {585, 590}, {538, 542}, {482, 502}, {442, 474}}, {pressure={0.5, 0.15}, ramps={0.05, 0.2}})
-- Right track of path
b_rut:stroke({{692, 700}, {648, 645}, {602, 590}, {554, 542}, {498, 502}, {456, 474}}, {pressure={0.5, 0.15}, ramps={0.05, 0.2}})
-- Sandy center ridge catching light
b_rut:load(p_sand_light, 0.6)
b_rut:stroke({{683, 700}, {640, 645}, {593, 590}, {546, 542}, {490, 502}}, {pressure={0.4, 0.1}, ramps={0.05, 0.2}})

-- 5. Crisp Tufts of Windswept Bent-Grass along the Path and Heath
local p_grass_straw = pile{{"lead white", 14}, {"yellow ochre", 9}, {"raw umber", 2.5}, {"red earth", 0.8}, medium=0.2}
local p_grass_shade = pile{{"raw umber", 8}, {"yellow ochre", 6}, {"bone black", 3}, medium=0.2}

local function paint_natural_grass(x, y, n, h, spread, lean)
  for i = 1, n do
    local dx = rand(-spread, spread) + lean
    local dy = -h * rand(0.75, 1.25)
    local mid_x = x + dx * 0.55 + rand(-1.5, 1.5)
    local mid_y = y + dy * 0.55
    local tip_x = x + dx
    local tip_y = y + dy
    
    local pts = {{x, y}, {mid_x, mid_y}, {tip_x, tip_y}}
    local p = (rand(0, 1) < 0.65) and p_grass_straw or p_grass_shade
    b_rigger:load(p, 0.65)
    b_rigger:stroke(pts, {pressure={0.65, 0.05}, ramps={0.05, 0.35}})
  end
end

-- Grass clumps along path margins
local path_grass_spots = {
  {660, 680, -4}, {705, 680, 5}, {620, 630, -5}, {655, 630, 4},
  {570, 580, -4}, {610, 580, 5}, {525, 535, -4}, {562, 535, 4},
  {470, 495, -3}, {508, 495, 3}
}
for _, sp in ipairs(path_grass_spots) do
  paint_natural_grass(sp[1], sp[2], 8, 18, 7, sp[3])
end

-- Grass clumps around oak roots and dolmen base
local root_grass_spots = {
  {218, 482, -4}, {234, 480, -3}, {250, 484, -2}, {284, 486, 1},
  {310, 480, 3}, {335, 474, 4}, {348, 472, 4},
  {364, 466, 2}, {384, 464, -1}, {410, 464, 1}, {435, 466, 3},
  {462, 466, 4}, {490, 468, 3}
}
for _, sp in ipairs(root_grass_spots) do
  paint_natural_grass(sp[1], sp[2], 9, 20, 8, sp[3])
end

-- 6. Twilight Rim-Light Highlighting Contours against the Evening Sky
local p_twilight_rim = pile{{"lead white", 22}, {"yellow ochre", 3.5}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
b_fine:load(p_twilight_rim, 0.5)

-- Capstone top rim catching afterglow
b_fine:stroke({{325, 427}, {340, 419}, {375, 413}, {415, 412}, {455, 416}, {472, 423}}, {pressure={0.45, 0.15}})

-- Oak trunk left contour rim
b_fine:stroke({{250, 468}, {256, 442}, {263, 412}, {266, 378}, {264, 345}, {256, 315}, {250, 285}}, {pressure={0.45, 0.1}})

-- Stag horn tips rim
b_rigger:load(p_twilight_rim, 0.6)
b_rigger:stroke({{254, 118}, {252, 95}, {248, 75}}, {pressure={0.4, 0.1}})
b_rigger:stroke({{326, 116}, {334, 94}, {340, 75}}, {pressure={0.4, 0.1}})
b_rigger:stroke({{208, 150}, {200, 130}, {192, 110}}, {pressure={0.4, 0.1}})

print("Canopies richly woven, path articulated, natural grasses planted, and twilight rim-lights stroked.")

--@ chunk 41
-- 1. Natural Winding Footpath through the Heath
local p_sand_rut = pile{{"yellow ochre", 7}, {"lead white", 5}, {"raw umber", 4}, {"bone black", 1}, medium=0.2}
local p_sand_light = pile{{"lead white", 10}, {"yellow ochre", 8}, {"raw umber", 2.2}, {"vermilion", 0.3}, medium=0.22}

local pts_path = {{680, 705}, {635, 640}, {585, 580}, {535, 530}, {480, 490}, {435, 465}}
local w_path = {28, 22, 16, 12, 8, 5}
local m_path = ribbon(pts_path, w_path):soften(4)

work(m_path, {
  hand = "scumble",
  pile = p_sand_rut,
  angle = 0.4,
  coverage = 1.4,
  fill = true
})

-- Center light track on path
local m_path_center = ribbon(pts_path, {14, 11, 8, 6, 4, 2.5}):soften(2)
work(m_path_center, {
  hand = "scumble",
  pile = p_sand_light,
  angle = 0.35,
  coverage = 1.2,
  fill = true
})

-- 2. Weave the Foreground Heath with Rich, Broad Color
local p_heath_deep = pile{{"raw umber", 9}, {"bone black", 5}, {"red earth", 3.5}, {"yellow ochre", 2.5}, medium=0.2}
local p_heath_russet = pile{{"yellow ochre", 7}, {"red earth", 5.5}, {"raw umber", 4}, {"lead white", 2}, medium=0.2}
local p_moss_rich = pile{{"green earth", 8}, {"yellow ochre", 5}, {"raw umber", 3}, {"lead white", 2}, medium=0.2}

-- Broad, natural banks of heather and moss overlapping across the foreground
local m_h1 = (ellipse(260, 580, 200, 50) + ellipse(740, 570, 220, 55)):soften(25)
work(m_h1, {hand="body", pile=p_heath_russet, angle=0.2, coverage=1.2, fill=true})

local m_h2 = (ellipse(420, 640, 240, 50) + ellipse(860, 630, 160, 45)):soften(25)
work(m_h2, {hand="body", pile=p_heath_deep, angle=-0.15, coverage=1.2, fill=true})

local m_m1 = (ellipse(360, 520, 100, 25) + ellipse(600, 515, 110, 28) + ellipse(180, 650, 120, 35)):soften(20)
work(m_m1, {hand="body", pile=p_moss_rich, angle=0.25, coverage=1.1, fill=true})

-- Blend gently to integrate the heath and path margins
blend(rect(0, 480, 1000, 225), {angle=0.15})

-- 3. Rich, Luminous Autumn Foliage on the Living Oak Boughs and Companion Tree
local b_leaf = brush{kind="round", width=2.4, point=1, stiffness=0.7}
local p_leaf_dark = pile{{"raw umber", 9}, {"red earth", 6}, {"bone black", 3}, {"yellow ochre", 2}, medium=0.2}
local p_leaf_russet = pile{{"red earth", 7}, {"yellow ochre", 7}, {"raw umber", 4}, {"lead white", 2.2}, medium=0.2}
local p_leaf_gold = pile{{"yellow ochre", 11}, {"lead white", 6}, {"chrome yellow", 1.8}, {"red earth", 2}, {"raw umber", 1}, medium=0.25}

-- Generous foliage masses along the living boughs
local major_leaf_clusters = {
  -- Eastern Bough
  {385, 290, 28, 35},
  {415, 255, 30, 40},
  {445, 230, 26, 35},
  {470, 280, 30, 40},
  {495, 255, 28, 35},
  {490, 345, 26, 35},
  {515, 370, 25, 30},
  {555, 250, 26, 30},
  {580, 235, 22, 25},
  -- Western Limb
  {185, 260, 30, 40},
  {155, 220, 28, 35},
  {140, 340, 28, 35},
  {115, 360, 25, 30},
  {95, 245, 26, 30},
  {70, 225, 22, 25},
  -- Companion Tree
  {735, 325, 25, 30},
  {705, 305, 26, 30},
  {680, 285, 22, 25},
  {815, 295, 26, 32},
  {845, 275, 24, 28},
  {750, 235, 24, 28},
  {725, 195, 22, 25}
}

for _, cl in ipairs(major_leaf_clusters) do
  local cx, cy, rx, ry = cl[1], cl[2], cl[3], cl[4]
  for i = 1, 35 do
    local ang = rand(0, 6.28)
    local r = rand(0, 1) ^ 0.7
    local x = cx + r * rx * math.cos(ang)
    local y = cy + r * ry * math.sin(ang)
    local d_len = rand(1.5, 3.2)
    local d_ang = rand(0.9, 1.8)
    
    local p
    local roll = rand(0, 1)
    if roll < 0.35 then
      p = p_leaf_dark
    elseif roll < 0.72 then
      p = p_leaf_russet
    else
      p = p_leaf_gold
    end
    
    b_leaf:load(p, 0.75)
    b_leaf:touch(x, y, {pressure=rand(0.5, 0.8), drag={d_len, d_ang}, twist=rand(-0.3, 0.3)})
  end
end

-- 4. Re-affirm the Dark Boughs and Twigs passing through and supporting the Foliage
local b_limb_solid = brush{kind="round", width=3.0, point=1, stiffness=0.8}
local b_twig_solid = brush{kind="round", width=1.6, point=1, stiffness=0.8}

local supporting_boughs = {
  {{290, 350}, {330, 342}, {370, 330}, {415, 323}, {460, 313}, {495, 292}, {528, 270}},
  {{370, 330}, {384, 300}, {400, 274}, {414, 248}, {424, 224}},
  {{425, 322}, {444, 294}, {466, 272}, {485, 246}},
  {{460, 313}, {478, 334}, {496, 352}, {514, 370}},
  {{266, 345}, {236, 330}, {206, 310}, {176, 292}, {146, 280}, {116, 272}, {86, 268}},
  {{206, 310}, {194, 278}, {182, 250}, {166, 224}, {150, 202}},
  {{176, 292}, {162, 316}, {142, 336}, {122, 350}}
}
for _, b in ipairs(supporting_boughs) do
  b_limb_solid:load(p_bark_dark, 0.85)
  b_limb_solid:stroke(b, {pressure={0.85, 0.45}, ramps={0.05, 0.15}})
end

print("Winding footpath, woven heath, rich autumn foliage, and supporting boughs painted.")

--@ chunk 42
-- Build generous, lacy autumn foliage masses using soft outline masks and multi-layered stippling

local p_fol_deep = pile{{"raw umber", 9}, {"bone black", 4}, {"red earth", 5}, {"yellow ochre", 2}, medium=0.2}
local p_fol_russet = pile{{"red earth", 7}, {"yellow ochre", 7}, {"raw umber", 4}, {"lead white", 2.2}, medium=0.2}
local p_fol_gold = pile{{"yellow ochre", 11}, {"lead white", 6}, {"chrome yellow", 1.8}, {"red earth", 2}, {"raw umber", 1}, medium=0.25}

-- 1. Eastern Bough Foliage Outline Mask
local pts_east_fol = {
  {330, 340}, {360, 305}, {385, 255}, {420, 215}, {465, 205}, {505, 220},
  {545, 240}, {575, 260}, {535, 305}, {515, 370}, {475, 360}, {430, 338}, {380, 348}, {330, 340}
}
local o_east = outline{pts=pts_east_fol, char="soft", amount=0.35, lobe=12, edge=5}
local m_east = o_east:mask()

-- 2. Western Limb Foliage Outline Mask
local pts_west_fol = {
  {240, 330}, {210, 285}, {175, 235}, {140, 200}, {105, 210}, {70, 235},
  {80, 280}, {110, 340}, {138, 368}, {178, 335}, {240, 330}
}
local o_west = outline{pts=pts_west_fol, char="soft", amount=0.35, lobe=12, edge=5}
local m_west = o_west:mask()

-- 3. Companion Tree Foliage Outline Mask
local pts_comp_fol = {
  {740, 340}, {710, 305}, {675, 275}, {655, 260}, {685, 225}, {715, 180},
  {745, 170}, {768, 195}, {810, 235}, {855, 255}, {870, 280}, {835, 315}, {780, 335}, {740, 340}
}
local o_comp = outline{pts=pts_comp_fol, char="soft", amount=0.35, lobe=10, edge=5}
local m_comp = o_comp:mask()

local m_all_trees = m_east + m_west + m_comp

-- Multi-layered stippling to create rich, lacy, authentic autumn foliage
-- Layer A: Deep shadow interior
stipple(m_all_trees, {
  pile = p_fol_deep,
  width = 2.8,
  coverage = 0.95,
  cluster = {0.8, 12},
  drag = {2.4, 1.4},
  feather = 0.4
})

-- Layer B: Warm russet body leaves
stipple(m_all_trees, {
  pile = p_fol_russet,
  width = 2.2,
  coverage = 0.9,
  cluster = {0.85, 9},
  drag = {2.0, 1.3},
  feather = 0.5
})

-- Layer C: Twilight-lit golden edges catching the evening afterglow
stipple(m_all_trees, {
  pile = p_fol_gold,
  width = 1.8,
  coverage = 0.65,
  cluster = {0.9, 7},
  drag = {1.6, 1.2},
  feather = 0.6
})

-- 4. Re-draw the strong structural boughs and twigs through the foliage
local b_limb_core = brush{kind="round", width=3.4, point=1, stiffness=0.8}
local b_branch_core = brush{kind="round", width=2.2, point=1, stiffness=0.8}
local b_twig_core = brush{kind="round", width=1.4, point=1, stiffness=0.8}

-- Eastern bough main stem
b_limb_core:load(p_bark_dark, 0.9)
b_limb_core:stroke({{290, 350}, {330, 342}, {370, 330}, {415, 323}, {460, 313}, {495, 292}, {528, 270}}, {pressure={0.9, 0.45}, ramps={0.05, 0.15}})

-- Eastern bough secondary limbs reaching through the leaves
local e_core_limbs = {
  {{370, 330}, {384, 300}, {400, 274}, {414, 248}, {424, 224}},
  {{425, 322}, {444, 294}, {466, 272}, {485, 246}},
  {{460, 313}, {478, 334}, {496, 352}, {514, 370}},
  {{528, 270}, {550, 255}, {572, 242}}
}
for _, b in ipairs(e_core_limbs) do
  b_branch_core:load(p_bark_dark, 0.85)
  b_branch_core:stroke(b, {pressure={0.8, 0.35}, ramps={0.05, 0.15}})
end

-- Western limb main stem
b_limb_core:load(p_bark_dark, 0.9)
b_limb_core:stroke({{266, 345}, {236, 330}, {206, 310}, {176, 292}, {146, 280}, {116, 272}, {86, 268}}, {pressure={0.9, 0.45}, ramps={0.05, 0.15}})

-- Western limb secondary limbs reaching through the leaves
local w_core_limbs = {
  {{206, 310}, {194, 278}, {182, 250}, {166, 224}, {150, 202}},
  {{176, 292}, {162, 316}, {142, 336}, {122, 350}},
  {{116, 272}, {100, 252}, {86, 236}}
}
for _, b in ipairs(w_core_limbs) do
  b_branch_core:load(p_bark_dark, 0.85)
  b_branch_core:stroke(b, {pressure={0.8, 0.35}, ramps={0.05, 0.15}})
end

-- Fine twigs protruding past the foliage edges into the open sky
local outer_twigs = {
  {{424, 224}, {434, 202}, {444, 184}},
  {{485, 246}, {502, 224}, {516, 206}},
  {{572, 242}, {592, 226}, {610, 214}},
  {{514, 370}, {526, 392}, {534, 412}},
  {{150, 202}, {136, 180}, {124, 160}},
  {{122, 350}, {108, 368}, {94, 385}},
  {{86, 236}, {70, 216}, {54, 202}}
}
for _, tw in ipairs(outer_twigs) do
  b_twig_core:load(p_bark_dark, 0.8)
  b_twig_core:stroke(tw, {pressure={0.75, 0.25}, ramps={0.05, 0.2}})
end

print("Generous lacy autumn foliage masses built and structural limbs re-stated.")

--@ chunk 43
-- Carve sky channels through the foliage sails to break them into natural, hanging bough clusters

local b_sky_ch = brush{kind="round", width=12, point=1, stiffness=0.6}
local b_sky_med = brush{kind="round", width=7, point=1, stiffness=0.6}

-- 1. Eastern Bough Sky Cuts:
-- Cut A: Between trunk/stag horns and the upward branch 1 (open zenith sky)
b_sky_ch:load(p_sky_mid, 0.85)
b_sky_ch:stroke({{310, 240}, {340, 260}, {365, 290}}, {pressure={0.85, 0.6}})
b_sky_ch:stroke({{330, 210}, {355, 240}, {375, 270}}, {pressure={0.85, 0.6}})

-- Cut B: Large triangular sky opening between branch 1 and branch 2
b_sky_med:load(p_sky_mid, 0.85)
b_sky_med:stroke({{425, 215}, {435, 250}, {445, 280}}, {pressure={0.85, 0.5}})

-- Cut C: Sky opening between eastern main bough and drooping shelter branch
b_sky_med:load(p_sky_glow, 0.85)
b_sky_med:stroke({{450, 310}, {465, 335}, {475, 355}}, {pressure={0.8, 0.4}})

-- Cut D: Sky above the dolmen capstone under the shelter branch
b_sky_ch:load(p_sky_glow, 0.9)
b_sky_ch:stroke({{330, 370}, {380, 365}, {440, 365}, {470, 370}}, {pressure={0.9, 0.6}})

-- 2. Western Limb Sky Cuts:
-- Cut E: Between trunk/stag horns and upward western branch
b_sky_ch:load(p_sky_mid, 0.85)
b_sky_ch:stroke({{250, 240}, {230, 260}, {215, 285}}, {pressure={0.85, 0.6}})

-- Cut F: Wide sky opening between upward branch and drooping branch
b_sky_ch:load(p_sky_mid, 0.85)
b_sky_ch:stroke({{180, 280}, {165, 305}, {150, 325}}, {pressure={0.85, 0.6}})

-- Cut G: Sky under the western bough
b_sky_ch:load(p_sky_glow, 0.85)
b_sky_ch:stroke({{90, 315}, {130, 310}, {180, 320}, {230, 335}}, {pressure={0.85, 0.5}})

-- 3. Companion Tree Sky Cuts:
-- Cut H: Wide V-channel down the center of the companion tree
b_sky_ch:load(p_sky_mid, 0.85)
b_sky_ch:stroke({{755, 175}, {750, 220}, {745, 265}, {740, 300}}, {pressure={0.9, 0.6}})
b_sky_med:stroke({{730, 250}, {720, 280}}, {pressure={0.8, 0.4}})
b_sky_med:stroke({{770, 250}, {780, 280}}, {pressure={0.8, 0.4}})

print("Sky channels carved through foliage masses.")

--@ chunk 44
local new_time = wait(4 * 24 * 60)
print("Time after drying: " .. new_time)
print("Drying state: " .. drying(400, 250))

--@ chunk 45
-- Fresh, opaque lead-white sky piles to restore the pure Friedrich twilight firmament
p_sky_zenith = pile{{"lead white", 14}, {"smalt", 6}, {"cobalt blue", 2.8}, {"raw umber", 0.6}, {"bone black", 0.2}, medium=0.2}
p_sky_mid = pile{{"lead white", 26}, {"smalt", 3.5}, {"cobalt blue", 1.0}, {"yellow ochre", 0.6}, {"raw umber", 0.3}, medium=0.2}
p_sky_trans = pile{{"lead white", 28}, {"yellow ochre", 2.5}, {"chrome yellow", 0.4}, {"raw umber", 0.2}, {"vermilion", 0.15}, medium=0.2}
p_sky_glow = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"chrome yellow", 0.8}, {"vermilion", 0.25}, {"raw umber", 0.2}, medium=0.2}

-- Band 1: Zenith
local m_z = rect(0, 0, 1000, 160):soften(25)
work(m_z, {hand="body", pile=p_sky_zenith, angle=0, order="across", coverage=1.7, fill=true})

-- Band 2: Mid-sky
local m_m = rect(0, 120, 1000, 160):soften(25)
work(m_m, {hand="body", pile=p_sky_mid, angle=0, order="across", coverage=1.7, fill=true})

-- Band 3: Transition
local m_t = rect(0, 240, 1000, 140):soften(25)
work(m_t, {hand="body", pile=p_sky_trans, angle=0, order="across", coverage=1.7, fill=true})

-- Band 4: Horizon glow
local m_g = rect(0, 330, 1000, 125):soften(20)
work(m_g, {hand="body", pile=p_sky_glow, angle=0, order="across", coverage=1.8, fill=true})

-- Seamless blending across the full canvas width
local m_sky_full = rect(0, 0, 1000, 455)
blend(m_sky_full, {angle=1.57})
blend(m_sky_full, {angle=0})

print("Pure, seamless twilight firmament restored across full width.")

--@ chunk 46
-- Sculpt the Megalithic Tomb (Dolmen / Hünengrab) with monumental solidity

local p_chamber_pitch = pile{{"bone black", 12}, {"raw umber", 6}, {"smalt", 1}, medium=0.1}
local p_granite_sh = pile{{"raw umber", 8}, {"bone black", 6}, {"smalt", 2}, {"lead white", 1.5}, medium=0.15}
local p_granite_mid = pile{{"lead white", 11}, {"raw umber", 6}, {"bone black", 3.5}, {"yellow ochre", 2.2}, {"smalt", 2}, medium=0.2}
local p_granite_lit = pile{{"lead white", 22}, {"yellow ochre", 3}, {"smalt", 1.5}, {"raw umber", 0.8}, {"vermilion", 0.2}, medium=0.22}
local p_turf_dark = pile{{"bone black", 8}, {"raw umber", 9}, {"green earth", 3}, medium=0.15}

-- 1. Dark Chamber Void (interior cavern of prehistoric tomb)
local pts_chamber = {{340, 431}, {452, 431}, {450, 465}, {338, 465}}
local m_chamber = poly(pts_chamber)
work(m_chamber, {hand="detail", pile=p_chamber_pitch, angle=1.2, coverage=1.8, fill=true, clip=true})

-- 2. Center-Rear Orthostat (recessed inside chamber)
local pts_co = {{384, 430}, {404, 430}, {406, 460}, {384, 460}}
local m_co = poly(pts_co)
work(m_co, {hand="detail", pile=p_granite_sh, angle=1.4, coverage=1.5, fill=true, clip=true})

-- 3. Left Orthostat (front pillar)
local pts_lo = {{338, 431}, {362, 431}, {364, 465}, {338, 465}}
local m_lo = poly(pts_lo)
work(m_lo, {hand="detail", pile=p_granite_mid, angle=1.4, coverage=1.5, fill=true, clip=true})
local m_lo_sh = poly({{352, 431}, {362, 431}, {364, 465}, {354, 465}})
work(m_lo_sh, {hand="detail", pile=p_granite_sh, angle=1.4, coverage=1.4, fill=true, clip=true})

-- 4. Right Orthostat (front pillar)
local pts_ro = {{428, 431}, {452, 432}, {452, 465}, {428, 465}}
local m_ro = poly(pts_ro)
work(m_ro, {hand="detail", pile=p_granite_mid, angle=1.4, coverage=1.5, fill=true, clip=true})
local m_ro_sh = poly({{428, 431}, {438, 431}, {438, 465}, {428, 465}})
work(m_ro_sh, {hand="detail", pile=p_granite_sh, angle=1.4, coverage=1.4, fill=true, clip=true})

-- 5. The Massive Granite Capstone
local pts_capstone = {
  {324, 428}, {332, 420}, {352, 415}, {385, 412}, {425, 413}, {455, 417}, {472, 424},
  {474, 432}, {455, 435}, {415, 431}, {375, 432}, {336, 435}, {324, 428}
}
local m_capstone = poly(pts_capstone, true)
work(m_capstone, {hand="body", pile=p_granite_mid, angle=0.08, coverage=1.6, fill=true, clip=true})

-- Deep cast shadow directly under capstone overhang projecting it forward
local pts_undercut = {
  {324, 428}, {336, 435}, {375, 432}, {415, 431}, {455, 435}, {474, 432},
  {470, 437}, {420, 435}, {370, 436}, {330, 438}, {324, 428}
}
local m_undercut = poly(pts_undercut)
work(m_undercut, {hand="detail", pile=p_chamber_pitch, angle=0, coverage=1.7, fill=true, clip=true})

-- Luminous top facet catching the pale twilight sky
local pts_cap_top_lit = {
  {324, 428}, {332, 420}, {352, 415}, {385, 412}, {425, 413}, {455, 417}, {472, 424},
  {468, 425}, {435, 420}, {395, 418}, {355, 420}, {328, 426}
}
local m_cap_top_lit = poly(pts_cap_top_lit, true)
work(m_cap_top_lit, {hand="detail", pile=p_granite_lit, angle=0.05, coverage=1.5, fill=true, clip=true})

-- 6. Surrounding Glacial Erratic Stones
-- Leaning stone left
local pts_lean = {{305, 458}, {312, 444}, {326, 442}, {336, 445}, {336, 458}, {324, 465}, {305, 462}}
local m_lean = poly(pts_lean, true)
work(m_lean, {hand="detail", pile=p_granite_mid, angle=0.4, coverage=1.5, fill=true, clip=true})
local m_lean_l = poly({{312, 444}, {326, 442}, {336, 445}, {330, 451}, {316, 450}})
work(m_lean_l, {hand="detail", pile=p_granite_lit, angle=0.2, coverage=1.4, fill=true, clip=true})

-- Boulder right of dolmen
local pts_rb = {{474, 454}, {484, 443}, {502, 441}, {518, 445}, {524, 456}, {512, 464}, {485, 465}, {474, 454}}
local m_rb = poly(pts_rb, true)
work(m_rb, {hand="detail", pile=p_granite_mid, angle=0.2, coverage=1.5, fill=true, clip=true})
local m_rb_l = poly({{484, 443}, {502, 441}, {518, 445}, {512, 450}, {488, 449}})
work(m_rb_l, {hand="detail", pile=p_granite_lit, angle=0.1, coverage=1.4, fill=true, clip=true})

-- Bed orthostats and stones into turf with dark earth
local b_bark = brush{kind="round", width=2.6, point=1, stiffness=0.8}
b_bark:load(p_turf_dark, 0.8)
b_bark:stroke({{300, 463}, {370, 465}}, {pressure={0.8, 0.5}})
b_bark:stroke({{420, 464}, {525, 465}}, {pressure={0.8, 0.5}})

print("Megalithic Tomb sculpted with deep chamber void and faceted granite planes.")

--@ chunk 47
-- Define all required piles
p_bark_dark = pile{{"bone black", 12}, {"raw umber", 8}, {"red earth", 2}, medium=0.15}
p_bark_mid = pile{{"raw umber", 8}, {"bone black", 4}, {"yellow ochre", 3.5}, {"lead white", 2.2}, {"red earth", 1}, medium=0.2}
p_stag_wood = pile{{"lead white", 14}, {"raw umber", 4}, {"yellow ochre", 2}, {"bone black", 1.2}, medium=0.2}
p_stag_shade = pile{{"raw umber", 8}, {"bone black", 4}, {"lead white", 4}, {"yellow ochre", 2}, medium=0.2}
p_wood_rim = pile{{"lead white", 20}, {"yellow ochre", 3.5}, {"raw umber", 1}, {"vermilion", 0.3}, medium=0.22}
p_chamber_pitch = pile{{"bone black", 12}, {"raw umber", 6}, medium=0.1}

-- 1. Trunk and Root Buttresses
local pts_trunk = {
  {248, 475}, {256, 445}, {264, 415}, {268, 380}, {266, 350}, {258, 315}, {252, 285},
  {270, 278}, {282, 295}, {294, 320}, {298, 352}, {304, 380}, {308, 415}, {314, 442},
  {326, 468}, {305, 472}, {280, 470}, {248, 475}
}
local m_trunk = poly(pts_trunk, true)
work(m_trunk, {hand="body", pile=p_bark_dark, angle=1.57, length={50, 100}, coverage=2.5, fill=true, clip=true})

-- Flared root claws
local b_root = brush{kind="round", width=4.5, point=1, stiffness=0.8}
b_root:load(p_bark_dark, 0.9)
b_root:stroke({{260, 440}, {248, 458}, {232, 474}, {218, 482}}, {pressure={0.9, 0.5}})
b_root:stroke({{268, 445}, {262, 464}, {254, 478}, {244, 485}}, {pressure={0.9, 0.5}})
b_root:stroke({{282, 450}, {284, 468}, {285, 486}}, {pressure={0.85, 0.5}})
b_root:stroke({{308, 442}, {320, 458}, {334, 470}, {345, 474}}, {pressure={0.9, 0.5}})

-- Hollow cavity
local pts_hollow = {
  {275, 440}, {282, 445}, {290, 438}, {294, 420}, {295, 402}, {290, 386},
  {284, 382}, {278, 390}, {273, 405}, {272, 424}
}
local m_hollow = poly(pts_hollow, true)
work(m_hollow, {hand="detail", pile=p_chamber_pitch, angle=1.57, coverage=2.0, fill=true, clip=true})

-- 2. Eastern Bough and Branches (Sheltering the Dolmen)
local m_eb_main = ribbon({
  {292, 350}, {325, 345}, {360, 335}, {400, 328}, {445, 318}, {485, 302}, {520, 280}, {545, 260}
}, {18, 15, 12, 9.5, 7.5, 5.5, 4, 2.5})
local m_eb_1 = ribbon({{360, 335}, {375, 305}, {392, 275}, {408, 245}, {420, 220}}, {7.5, 6, 4.5, 3.2, 2})
local m_eb_1_sub = ribbon({{392, 275}, {382, 252}, {375, 232}}, {3.5, 2.5, 1.8})
local m_eb_2 = ribbon({{445, 318}, {462, 292}, {484, 268}, {502, 242}}, {6, 4.8, 3.5, 2.2})
local m_eb_3 = ribbon({{485, 302}, {505, 325}, {525, 348}, {540, 370}}, {5, 3.8, 2.6, 1.8})
local m_eb_tip = ribbon({{545, 260}, {568, 244}, {590, 230}}, {3.2, 2.2, 1.5})

local m_eb_all = m_eb_main + m_eb_1 + m_eb_1_sub + m_eb_2 + m_eb_3 + m_eb_tip
work(m_eb_all, {hand="body", pile=p_bark_dark, angle=0.2, coverage=2.2, fill=true, clip=true})

-- 3. Western Limb and Branches (Reaching Left)
local m_wb_main = ribbon({
  {266, 345}, {236, 330}, {206, 310}, {176, 292}, {146, 280}, {116, 272}, {86, 268}, {60, 265}
}, {16, 13, 10, 8, 6, 4.5, 3, 2})
local m_wb_1 = ribbon({{206, 310}, {194, 278}, {180, 248}, {164, 220}, {148, 195}}, {7, 5.5, 4.2, 3, 2})
local m_wb_1_sub = ribbon({{180, 248}, {194, 224}, {202, 202}}, {3.5, 2.5, 1.8})
local m_wb_2 = ribbon({{176, 292}, {160, 318}, {140, 340}, {120, 358}}, {5.5, 4, 2.8, 1.8})
local m_wb_tip = ribbon({{86, 268}, {70, 250}, {55, 235}}, {3, 2.2, 1.5})

local m_wb_all = m_wb_main + m_wb_1 + m_wb_1_sub + m_wb_2 + m_wb_tip
work(m_wb_all, {hand="body", pile=p_bark_dark, angle=-0.3, coverage=2.2, fill=true, clip=true})

-- 4. Central Trunk & Stag-Headed Antlers (Bleached Ancient Dead Wood)
local m_stag_stem = ribbon({
  {268, 305}, {270, 265}, {268, 225}, {264, 185}, {258, 145}, {252, 115}
}, {13, 10, 7.5, 5.5, 3.8, 2.2})
local m_sh1 = ribbon({{268, 225}, {288, 194}, {310, 165}, {324, 138}, {330, 112}}, {6, 4.8, 3.5, 2.4, 1.6})
local m_sh1_f = ribbon({{288, 194}, {280, 166}, {274, 138}}, {3.2, 2.4, 1.5})
local m_sh2 = ribbon({{270, 265}, {248, 234}, {228, 205}, {214, 176}, {205, 148}}, {6.2, 4.8, 3.6, 2.5, 1.6})
local m_sh2_f = ribbon({{228, 205}, {238, 178}, {244, 154}}, {3.0, 2.2, 1.4})
local m_sh3 = ribbon({{272, 280}, {296, 255}, {324, 228}, {346, 204}, {358, 178}}, {6.5, 5.0, 3.8, 2.6, 1.6})
local m_sh3_f = ribbon({{324, 228}, {338, 200}, {344, 172}}, {3.0, 2.2, 1.4})

local m_stag_all = m_stag_stem + m_sh1 + m_sh1_f + m_sh2 + m_sh2_f + m_sh3 + m_sh3_f
work(m_stag_all, {hand="body", pile=p_stag_wood, angle=1.4, coverage=2.2, fill=true, clip=true})
work(m_stag_all, {hand="detail", pile=p_stag_shade, angle=1.2, coverage=1.1, fill=false})

-- Bridge living bark into stag crown base
local pts_bridge = {{256, 320}, {262, 285}, {268, 265}, {274, 265}, {284, 288}, {292, 320}}
local m_bridge = poly(pts_bridge, true)
work(m_bridge, {hand="body", pile=p_bark_dark, angle=1.4, coverage=2.0, fill=true, clip=true})

print("Ancient Oak bole, boughs, and stag antlers sculpted.")

--@ chunk 48
-- Sculpt Companion Tree, intricate crooked oak branch systems, foreground boulders and grasses

local b_limb = brush{kind="round", width=3.4, point=1, stiffness=0.8}
local b_branch = brush{kind="round", width=2.2, point=1, stiffness=0.8}
local b_twig = brush{kind="round", width=1.5, point=1, stiffness=0.8}
local b_stag_twig = brush{kind="round", width=1.6, point=1, stiffness=0.8}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.8}

-- 1. Sculpt the Companion Tree on the Right Knoll (x ≈ 770)
local m_t2_trunk = ribbon({
  {778, 465}, {775, 430}, {770, 395}, {764, 360}, {756, 325}, {748, 290}, {740, 255}, {732, 226}, {724, 202}
}, {12, 10.5, 9, 7.5, 6, 4.5, 3.5, 2.5, 1.8})
local m_t2_l = ribbon({{770, 395}, {746, 370}, {722, 345}, {698, 325}, {675, 310}}, {6, 4.8, 3.6, 2.6, 1.8})
local m_t2_r = ribbon({{764, 360}, {788, 340}, {816, 320}, {844, 302}, {870, 288}}, {5.5, 4.4, 3.4, 2.4, 1.6})

local m_t2_all = m_t2_trunk + m_t2_l + m_t2_r
work(m_t2_all, {hand="body", pile=p_bark_dark, angle=1.2, coverage=2.2, fill=true, clip=true})

-- Companion tree root flare
local b_root = brush{kind="round", width=3.5, point=1, stiffness=0.8}
b_root:load(p_bark_dark, 0.85)
b_root:stroke({{776, 440}, {768, 460}, {758, 474}}, {pressure={0.85, 0.4}})
b_root:stroke({{780, 440}, {788, 460}, {798, 474}}, {pressure={0.85, 0.4}})

-- 2. Intricate Crooked Oak Branching on the Ancient Oak
local oak_branches = {
  -- Off Eastern Bough
  {{325, 345}, {342, 320}, {356, 292}, {368, 268}},
  {{342, 320}, {334, 298}, {328, 276}},
  {{360, 335}, {375, 305}, {392, 275}, {408, 245}},
  {{392, 275}, {382, 252}, {375, 232}},
  {{408, 245}, {418, 222}, {426, 202}},
  {{408, 245}, {402, 224}, {396, 206}},
  {{400, 328}, {418, 305}, {434, 282}, {446, 258}},
  {{445, 318}, {462, 292}, {484, 268}, {502, 242}},
  {{484, 268}, {474, 248}, {468, 228}},
  {{502, 242}, {516, 222}, {528, 204}},
  {{502, 242}, {508, 222}, {512, 204}},
  {{485, 302}, {505, 325}, {525, 348}, {540, 370}},
  {{505, 325}, {518, 342}, {528, 360}},
  {{520, 280}, {542, 262}, {565, 246}, {585, 232}},
  {{565, 246}, {575, 230}, {582, 214}},
  -- Off Western Limb
  {{236, 330}, {218, 306}, {198, 282}, {180, 260}},
  {{218, 306}, {226, 284}, {230, 264}},
  {{206, 310}, {194, 278}, {180, 248}, {164, 220}},
  {{180, 248}, {194, 224}, {202, 202}},
  {{164, 220}, {152, 198}, {142, 178}},
  {{164, 220}, {170, 200}, {174, 182}},
  {{176, 292}, {160, 318}, {140, 340}, {120, 358}},
  {{140, 340}, {148, 360}, {152, 378}},
  {{146, 280}, {130, 260}, {112, 242}, {96, 226}},
  {{116, 272}, {98, 252}, {82, 235}, {68, 220}},
  {{86, 268}, {68, 265}, {50, 262}},
  {{86, 268}, {76, 282}, {66, 296}}
}
for _, b in ipairs(oak_branches) do
  b_branch:load(p_bark_dark, 0.85)
  b_branch:stroke(b, {pressure={0.8, 0.35}, ramps={0.05, 0.15}})
end

-- 3. Fine Sympodial Twigs on Ancient Oak Limbs
local oak_fine_twigs = {
  {{426, 202}, {434, 184}, {440, 168}},
  {{426, 202}, {420, 186}, {415, 170}},
  {{375, 232}, {368, 214}, {362, 196}},
  {{375, 232}, {382, 215}, {388, 198}},
  {{528, 204}, {538, 186}, {545, 170}},
  {{528, 204}, {520, 188}, {515, 172}},
  {{585, 232}, {598, 218}, {610, 206}},
  {{585, 232}, {592, 215}, {596, 198}},
  {{540, 370}, {550, 390}, {558, 410}},
  {{540, 370}, {532, 388}, {526, 405}},
  {{142, 178}, {132, 160}, {124, 142}},
  {{142, 178}, {148, 160}, {152, 144}},
  {{202, 202}, {210, 182}, {216, 164}},
  {{120, 358}, {108, 375}, {96, 392}},
  {{120, 358}, {128, 374}, {134, 390}},
  {{68, 220}, {56, 205}, {44, 192}},
  {{68, 220}, {74, 204}, {78, 188}}
}
for _, tw in ipairs(oak_fine_twigs) do
  b_twig:load(p_bark_dark, 0.8)
  b_twig:stroke(tw, {pressure={0.75, 0.25}, ramps={0.05, 0.2}})
end

-- 4. Fine Bleached Dead Twigs on Stag Crown
local stag_fine_antlers = {
  {{252, 115}, {250, 92}, {246, 72}},
  {{252, 115}, {260, 96}, {266, 78}},
  {{260, 96}, {256, 80}, {252, 65}},
  {{330, 112}, {338, 90}, {344, 70}},
  {{330, 112}, {324, 92}, {318, 74}},
  {{324, 92}, {328, 76}, {332, 60}},
  {{274, 138}, {270, 116}, {265, 96}},
  {{270, 116}, {276, 100}, {280, 84}},
  {{205, 148}, {198, 128}, {190, 108}},
  {{205, 148}, {212, 130}, {216, 112}},
  {{244, 154}, {246, 134}, {250, 115}},
  {{358, 178}, {370, 158}, {378, 138}},
  {{358, 178}, {352, 160}, {346, 142}},
  {{344, 172}, {350, 150}, {355, 130}}
}
for _, ant in ipairs(stag_fine_antlers) do
  b_stag_twig:load(p_stag_wood, 0.8)
  b_stag_twig:stroke(ant, {pressure={0.75, 0.25}, ramps={0.05, 0.2}})
end

-- 5. Companion Tree Branching & Windswept Twigs
local t2_branches = {
  {{675, 310}, {654, 292}, {634, 278}},
  {{675, 310}, {666, 288}, {658, 270}},
  {{698, 325}, {684, 305}, {672, 288}},
  {{722, 345}, {708, 318}, {698, 295}},
  {{870, 288}, {892, 272}, {912, 260}},
  {{870, 288}, {882, 268}, {888, 248}},
  {{844, 302}, {858, 280}, {868, 262}},
  {{816, 320}, {832, 294}, {842, 270}},
  {{724, 202}, {714, 180}, {706, 158}},
  {{724, 202}, {732, 180}, {738, 158}},
  {{732, 226}, {744, 202}, {752, 182}},
  {{740, 255}, {754, 232}, {762, 210}}
}
for _, b in ipairs(t2_branches) do
  b_twig:load(p_bark_dark, 0.8)
  b_twig:stroke(b, {pressure={0.75, 0.25}, ramps={0.05, 0.2}})
end

-- 6. Foreground Glacial Boulders (Solid Re-statement)
local p_bould_sh = pile{{"bone black", 8}, {"raw umber", 8}, {"smalt", 2}, {"lead white", 1.5}, medium=0.15}
local p_bould_mid = pile{{"lead white", 11}, {"raw umber", 6}, {"bone black", 3}, {"yellow ochre", 2.2}, {"smalt", 2}, medium=0.2}
local p_bould_lit = pile{{"lead white", 22}, {"smalt", 2.5}, {"yellow ochre", 2.5}, {"raw umber", 1}, medium=0.22}

-- Left Boulder
local m_bl = poly({{210, 538}, {226, 520}, {258, 516}, {278, 526}, {280, 544}, {250, 554}, {215, 548}}, true)
work(m_bl, {hand="detail", pile=p_bould_mid, angle=0.3, coverage=1.6, fill=true, clip=true})
local m_bl_top = poly({{226, 520}, {258, 516}, {276, 524}, {248, 528}, {226, 525}}, true)
work(m_bl_top, {hand="detail", pile=p_bould_lit, angle=0.1, coverage=1.4, fill=true, clip=true})
b_root:load(p_bould_sh, 0.8)
b_root:stroke({{206, 546}, {248, 556}, {284, 548}}, {pressure={0.85, 0.4}})

-- Right Boulder
local m_br = poly({{642, 546}, {664, 528}, {698, 525}, {718, 536}, {716, 552}, {684, 562}, {646, 556}}, true)
work(m_br, {hand="detail", pile=p_bould_mid, angle=-0.2, coverage=1.6, fill=true, clip=true})
local m_br_top = poly({{664, 528}, {698, 525}, {714, 532}, {688, 536}, {664, 532}}, true)
work(m_br_top, {hand="detail", pile=p_bould_lit, angle=-0.1, coverage=1.4, fill=true, clip=true})
b_root:stroke({{638, 554}, {684, 564}, {720, 555}}, {pressure={0.85, 0.4}})

-- Center-Left Boulder
local m_bc = poly({{425, 638}, {442, 622}, {468, 624}, {476, 638}, {458, 652}, {428, 646}}, true)
work(m_bc, {hand="detail", pile=p_bould_mid, angle=0.2, coverage=1.5, fill=true, clip=true})
local m_bc_top = poly({{442, 622}, {468, 624}, {464, 634}, {440, 632}}, true)
work(m_bc_top, {hand="detail", pile=p_bould_lit, angle=0.1, coverage=1.3, fill=true, clip=true})
b_root:stroke({{422, 646}, {456, 654}, {480, 642}}, {pressure={0.85, 0.4}})

-- 7. Twilight Rim-Lights on Trees and Stones
b_rigger:load(p_wood_rim, 0.6)
-- Ancient oak bole left contour
b_rigger:stroke({{248, 472}, {256, 445}, {264, 415}, {268, 380}, {266, 350}, {258, 315}, {252, 285}}, {pressure={0.45, 0.1}})
-- Companion tree left contour
b_rigger:stroke({{778, 465}, {774, 430}, {768, 395}, {762, 360}, {754, 325}, {746, 290}}, {pressure={0.45, 0.1}})
-- Capstone top rim
b_rigger:stroke({{325, 427}, {340, 419}, {375, 413}, {415, 412}, {455, 416}, {472, 423}}, {pressure={0.45, 0.15}})

print("Companion Tree sculpted, branching systems developed, boulders solidified, and rim-lights stroked.")

--@ chunk 49
-- Final Master Refinements: Boulders, Heath, Pilgrim Path, Flora, and Twilight Glints

local b_fine = brush{kind="round", width=1.2, point=1, stiffness=0.8}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.8}
local b_touch = brush{kind="round", width=2.0, point=1, stiffness=0.7}
local b_heather = brush{kind="round", width=3.2, point=1, stiffness=0.7}

-- Refinement piles
local p_turf_base = pile{{"raw umber", 9}, {"bone black", 6}, {"green earth", 3.5}, medium=0.15}
local p_heather_bank = pile{{"raw umber", 7}, {"red earth", 5}, {"yellow ochre", 4}, {"bone black", 2}, medium=0.2}
local p_lichen_sage = pile{{"lead white", 14}, {"green earth", 5}, {"yellow ochre", 3}, {"raw umber", 1}, medium=0.25}
local p_lichen_gold = pile{{"lead white", 10}, {"yellow ochre", 8}, {"chrome yellow", 1.5}, {"raw umber", 1}, medium=0.25}
local p_straw_sharp = pile{{"lead white", 16}, {"yellow ochre", 8}, {"raw umber", 2}, {"red earth", 0.8}, medium=0.2}
local p_straw_shade = pile{{"raw umber", 8}, {"yellow ochre", 6}, {"bone black", 3}, medium=0.2}
local p_twilight_rim = pile{{"lead white", 22}, {"yellow ochre", 3.5}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_pebble = pile{{"lead white", 12}, {"raw umber", 6}, {"bone black", 3}, {"yellow ochre", 2}, medium=0.2}

-- 1. Bed Boulders naturally into the Heath with surrounding vegetation
-- Left Boulder (x ≈ 240, y ≈ 535)
b_heather:load(p_heather_bank, 0.7)
b_heather:stroke({{200, 550}, {230, 554}, {260, 552}, {290, 545}}, {pressure={0.75, 0.4}})
b_touch:load(p_lichen_sage, 0.6)
b_touch:touch(235, 524, {pressure=0.5, drag={1.5, 0.2}})
b_touch:touch(255, 520, {pressure=0.5, drag={1.8, 0.1}})
b_touch:load(p_lichen_gold, 0.5)
b_touch:touch(245, 522, {pressure=0.4, drag={1.2, 0.3}})

-- Right Boulder (x ≈ 680, y ≈ 545)
b_heather:load(p_heather_bank, 0.7)
b_heather:stroke({{630, 555}, {665, 562}, {700, 560}, {730, 552}}, {pressure={0.75, 0.4}})
b_touch:load(p_lichen_sage, 0.6)
b_touch:touch(675, 532, {pressure=0.5, drag={1.5, -0.2}})
b_touch:touch(695, 530, {pressure=0.5, drag={1.8, -0.1}})
b_touch:load(p_lichen_gold, 0.5)
b_touch:touch(685, 530, {pressure=0.4, drag={1.2, -0.2}})

-- Center Boulder (x ≈ 450, y ≈ 635)
b_heather:load(p_heather_bank, 0.7)
b_heather:stroke({{415, 648}, {445, 654}, {475, 650}}, {pressure={0.75, 0.4}})
b_touch:load(p_lichen_sage, 0.6)
b_touch:touch(448, 626, {pressure=0.5, drag={1.4, 0.1}})

-- 2. Articulate the Winding Pilgrim Path with Scattered Gravel and Margins
local path_pebbles = {
  {665, 675, 3.5, 2.0}, {645, 655, 4.0, 2.5}, {620, 625, 3.0, 1.8},
  {595, 600, 3.8, 2.2}, {570, 570, 3.2, 1.8}, {545, 545, 2.8, 1.5},
  {515, 520, 3.5, 2.0}, {490, 498, 2.5, 1.5}, {465, 480, 3.0, 1.6}
}
for _, p in ipairs(path_pebbles) do
  b_touch:load(p_pebble, 0.7)
  b_touch:touch(p[1], p[2], {pressure=0.6, drag={p[3], 0.2}})
  -- Highlight top of pebble
  b_rigger:load(p_twilight_rim, 0.5)
  b_rigger:touch(p[1], p[2] - 0.5, {pressure=0.35, drag={p[3]*0.6, 0.1}})
end

-- 3. Windswept Bent-Grass Tufts along the Path, Boulders, and Ridge Crest
local function draw_curved_blade(bx, by, len, angle, bend, p)
  local mid_x = bx + len * 0.55 * math.cos(angle + bend * 0.5)
  local mid_y = by - len * 0.55 * math.sin(angle + bend * 0.5)
  local tip_x = bx + len * math.cos(angle + bend)
  local tip_y = by - len * math.sin(angle + bend)
  
  b_rigger:load(p, 0.65)
  b_rigger:stroke({{bx, by}, {mid_x, mid_y}, {tip_x, tip_y}}, {pressure={0.6, 0.04}, ramps={0.05, 0.4}})
end

local function draw_grass_cluster(cx, cy, n, h, spread, main_lean)
  for i = 1, n do
    local bx = cx + rand(-spread, spread)
    local by = cy + rand(-2, 2)
    local len = h * rand(0.75, 1.3)
    local ang = 1.57 + rand(-0.3, 0.3) + main_lean * 0.3
    local bnd = main_lean * rand(0.2, 0.6) + rand(-0.15, 0.15)
    local p = (rand(0, 1) < 0.65) and p_straw_sharp or p_straw_shade
    draw_curved_blade(bx, by, len, ang, bnd, p)
  end
end

-- Grasses along the path
local path_grass_spots = {
  {660, 680, 7, 16, 5, -0.4}, {700, 680, 7, 15, 5, 0.4},
  {625, 635, 6, 14, 4, -0.4}, {650, 635, 6, 14, 4, 0.3},
  {575, 585, 6, 13, 4, -0.3}, {605, 585, 6, 13, 4, 0.3},
  {528, 535, 5, 12, 3, -0.3}, {555, 535, 5, 12, 3, 0.2},
  {475, 498, 5, 11, 3, -0.2}, {500, 498, 5, 11, 3, 0.2}
}
for _, sp in ipairs(path_grass_spots) do
  draw_grass_cluster(sp[1], sp[2], sp[3], sp[4], sp[5], sp[6])
end

-- Grasses around boulders and oak roots
local rock_grass_spots = {
  {205, 546, 7, 16, 5, -0.5}, {225, 552, 6, 14, 4, -0.2},
  {255, 554, 6, 15, 4, 0.1}, {280, 548, 7, 16, 5, 0.4},
  {640, 556, 7, 16, 5, -0.4}, {668, 562, 6, 15, 4, -0.1},
  {700, 562, 6, 15, 4, 0.2}, {725, 554, 7, 16, 5, 0.5},
  {425, 648, 6, 14, 4, -0.3}, {465, 652, 6, 14, 4, 0.3},
  {220, 482, 8, 18, 5, -0.4}, {242, 484, 8, 18, 5, -0.2},
  {285, 486, 7, 16, 4, 0.1}, {312, 480, 8, 17, 4, 0.3},
  {342, 474, 8, 16, 4, 0.4}
}
for _, sp in ipairs(rock_grass_spots) do
  draw_grass_cluster(sp[1], sp[2], sp[3], sp[4], sp[5], sp[6])
end

-- 4. Carline Thistle Skeletons along the Ridge Crest silhouetted against glowing mist
local thistle_groups = {
  -- Left ridge (x ≈ 150-180)
  {{{155, 470}, {154, 450}, {152, 432}}, {{154, 442}, {146, 434}}, {{154, 442}, {160, 435}}},
  {{{168, 466}, {169, 446}, {170, 428}}, {{169, 438}, {176, 430}}, {{169, 438}, {163, 431}}},
  -- Saddle (x ≈ 540-570)
  {{{546, 462}, {545, 442}, {544, 424}}, {{545, 434}, {538, 426}}, {{545, 434}, {552, 427}}},
  {{{560, 464}, {562, 444}, {564, 426}}, {{562, 436}, {570, 429}}},
  -- Right ridge (x ≈ 840-870)
  {{{848, 458}, {847, 438}, {846, 420}}, {{847, 430}, {840, 422}}, {{847, 430}, {855, 423}}},
  {{{862, 462}, {864, 442}, {866, 425}}, {{864, 434}, {872, 427}}}
}
for _, grp in ipairs(thistle_groups) do
  for _, st in ipairs(grp) do
    b_fine:load(p_bark_dark, 0.8)
    b_fine:stroke(st, {pressure={0.7, 0.2}, ramps={0.05, 0.2}})
  end
end

-- 5. Organic Crossing Twigs on Ancient Oak and Companion Tree to enrich canopy
local extra_twigs = {
  -- Ancient Oak
  {{342, 320}, {355, 305}, {368, 288}},
  {{356, 292}, {364, 275}, {370, 256}},
  {{418, 305}, {428, 288}, {436, 268}},
  {{462, 292}, {472, 274}, {480, 255}},
  {{505, 325}, {498, 342}, {492, 360}},
  {{218, 306}, {228, 288}, {234, 268}},
  {{194, 278}, {186, 260}, {180, 240}},
  {{130, 260}, {122, 242}, {116, 224}},
  -- Companion Tree
  {{746, 370}, {732, 350}, {718, 332}},
  {{788, 340}, {802, 322}, {814, 304}},
  {{754, 255}, {762, 230}, {768, 205}}
}
for _, tw in ipairs(extra_twigs) do
  b_fine:load(p_bark_dark, 0.85)
  b_fine:stroke(tw, {pressure={0.75, 0.25}, ramps={0.05, 0.2}})
end

-- 6. Subtle Pale Twilight Rim-Light on Sky-Facing Contours
b_rigger:load(p_twilight_rim, 0.6)
-- Capstone top rim catching horizon afterglow
b_rigger:stroke({{325, 427}, {345, 418}, {385, 412}, {425, 413}, {455, 417}, {472, 424}}, {pressure={0.45, 0.15}})
-- Ancient oak trunk left contour
b_rigger:stroke({{248, 472}, {256, 445}, {264, 415}, {268, 380}, {266, 350}, {258, 315}, {252, 285}}, {pressure={0.4, 0.1}})
-- Companion tree left contour
b_rigger:stroke({{778, 465}, {774, 430}, {768, 395}, {762, 360}, {754, 325}, {746, 290}}, {pressure={0.4, 0.1}})

print("Final master refinements executed: boulders bedded, path articulated, thistle skeletons and bent-grass planted, canopy enriched, and rim-lights stroked.")

--@ chunk 50
local t = wait(0)
print("Current time:", t)
print("Drying at sky (500, 200):", drying(500, 200))
print("Drying at tree (300, 300):", drying(300, 300))
print("Drying at ground (500, 600):", drying(500, 600))

--@ chunk 51
local t = wait(4 * 24 * 60)
print("New time:", t)
print("Drying at sky (500, 200):", drying(500, 200))
print("Drying at tree (300, 300):", drying(300, 300))
print("Drying at ground (500, 600):", drying(500, 600))

--@ chunk 52
-- Chunk 52: Restoring the Sublime Friedrichian Sky, Horizon, and Distant Headlands

-- 1. Piles for the vast evening firmament
p_sky_zenith = pile{{"lead white", 16}, {"cobalt blue", 5.5}, {"smalt", 4.0}, {"raw umber", 0.6}, {"bone black", 0.25}, medium=0.22}
p_sky_upper  = pile{{"lead white", 24}, {"cobalt blue", 2.8}, {"smalt", 3.0}, {"yellow ochre", 0.8}, {"raw umber", 0.3}, medium=0.22}
p_sky_mid    = pile{{"lead white", 28}, {"yellow ochre", 2.8}, {"chrome yellow", 0.6}, {"smalt", 1.0}, {"raw umber", 0.2}, {"vermilion", 0.1}, medium=0.22}
p_sky_glow   = pile{{"lead white", 26}, {"yellow ochre", 4.8}, {"chrome yellow", 1.8}, {"vermilion", 0.45}, {"raw umber", 0.25}, medium=0.22}

-- 2. Four broad horizontal bands spanning the entire canvas width to erase stick artifacts and establish seamless depth
local m_z = rect(0, 0, 1000, 160):soften(25)
work(m_z, {hand="body", pile=p_sky_zenith, angle=0, order="across", coverage=2.0, fill=true})

local m_u = rect(0, 120, 1000, 160):soften(25)
work(m_u, {hand="body", pile=p_sky_upper, angle=0, order="across", coverage=2.0, fill=true})

local m_m = rect(0, 240, 1000, 140):soften(25)
work(m_m, {hand="body", pile=p_sky_mid, angle=0, order="across", coverage=2.0, fill=true})

local m_g = rect(0, 340, 1000, 130):soften(20)
work(m_g, {hand="body", pile=p_sky_glow, angle=0, order="across", coverage=2.0, fill=true})

-- 3. Seamless badger blending across the full canvas width
local m_sky_full = rect(0, 0, 1000, 470)
blend(m_sky_full, {angle=0})
blend(m_sky_full, {angle=1.57})
blend(m_sky_full, {angle=0})

-- 4. Distant low headlands and islands across the Baltic water (y ≈ 452 to 468)
local p_dist_coast = pile{{"lead white", 14}, {"smalt", 5.5}, {"raw umber", 3.0}, {"red earth", 1.0}, {"bone black", 0.5}, medium=0.25}
local pts_distant_land = {
  {0, 466}, {80, 464}, {160, 460}, {250, 456}, {340, 458}, {440, 454},
  {540, 456}, {650, 452}, {760, 455}, {870, 460}, {1000, 466},
  {1000, 474}, {0, 474}
}
local m_coast = poly(pts_distant_land, true):soften(2.5)
work(m_coast, {hand="body", pile=p_dist_coast, angle=0, coverage=1.4, fill=true, clip=true})

-- 5. Calm evening water surface (Bodden) reflecting the glowing twilight
local p_water = pile{{"lead white", 22}, {"yellow ochre", 3.0}, {"smalt", 2.2}, {"raw umber", 0.6}, {"vermilion", 0.25}, medium=0.3}
local m_water = rect(0, 462, 1000, 16):soften(3)
work(m_water, {hand="glaze", pile=p_water, angle=0, coverage=1.2, fill=true})

-- 6. Soft drifting evening mist pooling in the low ground before the foreground ridge
local p_valley_mist = pile{{"lead white", 26}, {"yellow ochre", 2.0}, {"smalt", 1.5}, {"raw umber", 0.4}, medium=0.35}
local m_mist = rect(0, 458, 1000, 20):soften(6)
work(m_mist, {hand="glaze", pile=p_valley_mist, angle=0, coverage=1.0, fill=false})
blend(rect(0, 455, 1000, 25), {angle=0})

print("Sublime twilight sky, distant Baltic coast, calm water and evening mist established.")

--@ chunk 53
-- Chunk 53: Horizontal smoothing of sky and painting the waxing crescent moon

-- 1. Horizontal blend pass with badger blender to fuse any vertical striations into silky atmosphere
blend(rect(0, 0, 1000, 420), {angle=0})

-- 2. Crescent Moon in the calm twilight firmament (x ≈ 640, y ≈ 105)
-- The Moon: a delicate waxing crescent facing the setting sun in the west
local p_moon_bright = pile{{"lead white", 30}, {"yellow ochre", 1.5}, {"chrome yellow", 0.6}, medium=0.15}
local p_earthshine  = pile{{"lead white", 12}, {"smalt", 4}, {"raw umber", 1.2}, {"yellow ochre", 0.8}, medium=0.25}

-- Ashen light (earthshine): the shadowy globe of the moon faintly visible against the twilight blue
local m_moon_globe = ellipse(640, 105, 7.5, 7.5):soften(1.2)
work(m_moon_globe, {hand="detail", pile=p_earthshine, angle=0, coverage=0.8, fill=true})

-- Luminous crescent on the western limb of the moon
local b_crescent = brush{kind="round", width=1.4, point=1, stiffness=0.8}
b_crescent:load(p_moon_bright, 0.85)
b_crescent:stroke({
  {639.5, 98}, {635.5, 100}, {633.0, 105}, {634.5, 110}, {638.5, 112.5}
}, {pressure={0.1, 0.85, 0.9, 0.7, 0.1}, ramps={0.1, 0.1}})

-- Soft twilight lunar glow
local m_moon_aura = ellipse(636, 105, 18, 18):soften(8)
work(m_moon_aura, {hand="glaze", pile=p_moon_bright, angle=0, coverage=0.3, fill=false})
blend(ellipse(636, 105, 24, 24), {angle=0})

print("Sky smoothed horizontally and delicate waxing crescent moon painted.")

--@ chunk 54
local t = wait(2 * 24 * 60)
print("Time after sky dry:", t)
print("Drying at sky (500, 200):", drying(500, 200))

--@ chunk 55
-- Chunk 55: The Crescent Moon, Evening Star, and Rich Undulating Barrow Terrain

-- 1. Crescent Moon and Evening Star on the dry sky
local p_star_bright = pile{{"lead white", 32}, {"chrome yellow", 0.5}, {"yellow ochre", 0.5}, medium=0.1}
local p_moon_horn   = pile{{"lead white", 28}, {"yellow ochre", 2.0}, {"chrome yellow", 0.6}, medium=0.15}
local p_earthshine  = pile{{"lead white", 8}, {"cobalt blue", 2.5}, {"smalt", 2.0}, {"raw umber", 1.2}, medium=0.25}

-- Earthshine disc
local b_fine = brush{kind="round", width=1.2, point=1, stiffness=0.8}
local m_ash = ellipse(645, 112, 6.5, 6.5):soften(1.0)
work(m_ash, {hand="detail", pile=p_earthshine, angle=0, coverage=0.85, fill=true})

-- Luminous waxing crescent curve (facing down and right towards the setting sun)
local b_moon = brush{kind="round", width=1.5, point=1, stiffness=0.85}
b_moon:load(p_moon_horn, 0.9)
b_moon:stroke({
  {646, 106}, {642, 108}, {639.5, 112}, {640.5, 116}, {644.5, 118.5}
}, {pressure={0.08, 0.8, 0.95, 0.75, 0.08}, ramps={0.08, 0.08}})

-- Solitary Evening Star (Venus / Hesperus) at x = 688, y = 148
local b_star = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_star:load(p_star_bright, 0.95)
b_star:touch(688, 148, {pressure=0.7})
-- Subtle four-ray twinkle
b_star:stroke({{686.5, 148}, {689.5, 148}}, {pressure={0.15, 0.4, 0.15}})
b_star:stroke({{688, 146.5}, {688, 149.5}}, {pressure={0.15, 0.4, 0.15}})

-- 2. Distant Baltic Coast & Headland silhouetted against the afterglow (y ≈ 448 to 468)
local p_coast_haze = pile{{"lead white", 15}, {"smalt", 5.0}, {"cobalt blue", 1.5}, {"raw umber", 3.0}, {"red earth", 1.2}, medium=0.25}
local pts_distant_ridge = {
  {0, 464}, {90, 460}, {180, 455}, {280, 451}, {380, 453}, {480, 449},
  {580, 452}, {680, 448}, {780, 453}, {890, 458}, {1000, 464},
  {1000, 478}, {0, 478}
}
local m_ridge = poly(pts_distant_ridge, true):soften(1.8)
work(m_ridge, {hand="body", pile=p_coast_haze, angle=0, coverage=1.5, fill=true, clip=true})

-- Evening mist drifting in the valley before the barrow
local p_mist_glow = pile{{"lead white", 26}, {"yellow ochre", 3.5}, {"chrome yellow", 0.6}, {"smalt", 1.2}, {"raw umber", 0.3}, medium=0.3}
local m_valley_mist = rect(0, 452, 1000, 24):soften(5)
work(m_valley_mist, {hand="glaze", pile=p_mist_glow, angle=0, coverage=1.1, fill=false})

-- 3. The Grand Foreground Barrow Knoll - Complete Rework
-- Piles for rich, authentic Baltic heathland
local p_earth_core   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4}, {"yellow ochre", 3}, medium=0.18}
local p_heath_dark   = pile{{"raw umber", 8}, {"bone black", 5}, {"red earth", 3.5}, {"green earth", 3.5}, medium=0.2}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5}, {"lead white", 2}, {"bone black", 1}, medium=0.2}
local p_heath_gold   = pile{{"yellow ochre", 8}, {"raw umber", 4}, {"lead white", 4}, {"red earth", 2}, medium=0.2}
local p_peat_moss    = pile{{"green earth", 6}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}

-- A. The Knoll Crest: crisp, undulating silhouette against the glowing mist
local pts_barrow_crest = {
  {0, 485}, {70, 480}, {150, 470}, {230, 456}, {310, 444},
  {370, 439}, {430, 441}, {500, 452}, {590, 463}, {680, 461},
  {750, 453}, {830, 457}, {910, 467}, {1000, 476},
  {1000, 710}, {0, 710}
}
local m_barrow_body = poly(pts_barrow_crest, true)

-- Foundation body pass across the entire foreground (covers old path and bun rocks)
work(m_barrow_body, {
  hand = "body",
  pile = p_earth_core,
  angle = 0.25,
  length = {40, 90},
  coverage = 2.2,
  fill = true,
  clip = true
})

-- B. Sculpt the undulating topography with banks of heather, peat, and turf
-- Knoll crest rim of dark peat and dry heather
local m_crest_band = ribbon({
  {0, 485}, {70, 480}, {150, 470}, {230, 456}, {310, 444},
  {370, 439}, {430, 441}, {500, 452}, {590, 463}, {680, 461},
  {750, 453}, {830, 457}, {910, 467}, {1000, 476}
}, 16):soften(4)
work(m_crest_band, {hand="body", pile=p_heath_dark, angle=0.2, coverage=1.4, fill=true})

-- Undulating bank 1: Upper slope heather (russet and gold)
local m_bank1 = (ellipse(260, 490, 180, 35) + ellipse(620, 500, 220, 38) + ellipse(860, 495, 140, 30)):soften(15)
work(m_bank1 * m_barrow_body, {hand="body", pile=p_heath_russet, angle=0.18, coverage=1.3, fill=true})

-- Undulating bank 2: Mid-slope mossy peat hollows
local m_hollow = (ellipse(440, 545, 200, 40) + ellipse(180, 560, 160, 45)):soften(18)
work(m_hollow * m_barrow_body, {hand="body", pile=p_peat_moss, angle=-0.12, coverage=1.2, fill=true})

-- Undulating bank 3: Lower foreground heather ridges (warm ochre-russet)
local m_bank2 = (ellipse(320, 620, 260, 50) + ellipse(760, 610, 240, 45)):soften(20)
work(m_bank2 * m_barrow_body, {hand="body", pile=p_heath_gold, angle=0.22, coverage=1.3, fill=true})

-- Deep foreground base (dark, rich peaty loam at bottom margin)
local m_base = rect(0, 650, 1000, 60):soften(15)
work(m_base * m_barrow_body, {hand="body", pile=p_earth_core, angle=0.1, coverage=1.5, fill=true})

print("Moon, Evening Star, distant coast, and rich sculptural barrow terrain painted.")

--@ chunk 56
-- Chunk 56: Blending the foreground heather wet-into-wet and sculpting the smooth, undulating knoll crest

-- 1. Fusing the open foreground paint into a unified, rich peaty heathland
local m_ground = rect(0, 435, 1000, 275)
blend(m_ground, {angle=0.2})
blend(m_ground, {angle=-0.15})
blend(m_ground, {angle=0})

-- 2. Stroke the crisp, natural barrow crest with a round brush to eliminate saw-tooth edge
local p_crest_firm = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 3}, {"yellow ochre", 2}, medium=0.15}
local b_crest = brush{kind="round", width=3.8, point=1, stiffness=0.8}

local crest_pts = {
  {-10, 486}, {60, 482}, {140, 472}, {220, 458}, {290, 446},
  {350, 440}, {410, 441}, {470, 447}, {540, 458}, {620, 464},
  {700, 460}, {770, 454}, {840, 458}, {910, 467}, {1010, 476}
}

b_crest:load(p_crest_firm, 0.9)
b_crest:stroke(crest_pts, {pressure=0.8, ramps={0.02, 0.02}})

-- 3. Enhance the crescent moon's crisp brilliance against the sky
local p_moon_pure = pile{{"lead white", 35}, {"yellow ochre", 1.2}, {"chrome yellow", 0.4}, medium=0.1}
local b_moon_crisp = brush{kind="round", width=1.5, point=1, stiffness=0.9}
b_moon_crisp:load(p_moon_pure, 0.95)
b_moon_crisp:stroke({
  {646, 105}, {642, 107.5}, {639.2, 112}, {640.2, 116.5}, {644.5, 119}
}, {pressure={0.1, 0.85, 1.0, 0.8, 0.1}, ramps={0.05, 0.05}})

-- Touch the solitary Evening Star (Venus)
local b_star_crisp = brush{kind="round", width=1.2, point=1, stiffness=0.9}
b_star_crisp:load(p_moon_pure, 1.0)
b_star_crisp:touch(688, 148, {pressure=0.75})

print("Foreground heather fused into smooth continuous earth, crest drawn clean, moon and star defined.")

--@ chunk 57
-- Chunk 57: Perfecting the Moon, Sky Harmony, and Sculpting the Swelling Barrow Crest

-- 1. Refine the Crescent Moon: integrate earthshine softly into the twilight firmament
local p_sky_patch = pile{{"lead white", 22}, {"cobalt blue", 3.2}, {"smalt", 3.2}, {"yellow ochre", 0.8}, {"raw umber", 0.3}, medium=0.25}
local b_soft = brush{kind="filbert", width=8, stiffness=0.4}
b_soft:load(p_sky_patch, 0.45)
-- Softly glaze around the moon to dissolve the hard dark circle
b_soft:stroke({{630, 112}, {660, 112}}, {pressure={0.2, 0.05}})
blend(ellipse(645, 112, 14, 14), {angle=0})

-- Crisp, luminous waxing crescent
local p_moon_brilliant = pile{{"lead white", 34}, {"yellow ochre", 1.0}, {"chrome yellow", 0.4}, medium=0.12}
local b_rigger = brush{kind="round", width=1.3, point=1, stiffness=0.85}
b_rigger:load(p_moon_brilliant, 0.95)
b_rigger:stroke({
  {645.2, 105.5}, {641.5, 108.0}, {639.0, 112.5}, {640.2, 117.0}, {644.0, 119.5}
}, {pressure={0.08, 0.85, 1.0, 0.8, 0.08}, ramps={0.05, 0.05}})

-- Soften the star's cross-arms into a delicate point of twilight light
local b_star_soft = brush{kind="round", width=0.8, point=1, stiffness=0.7}
b_star_soft:load(p_sky_patch, 0.3)
b_star_soft:touch(688, 148, {pressure=0.2})
local b_star_core = brush{kind="round", width=0.9, point=1, stiffness=0.9}
b_star_core:load(p_moon_brilliant, 0.9)
b_star_core:touch(688, 148, {pressure=0.55})

-- 2. Sculpt the Swelling Barrow Mound & Natural Crest Contour
local p_heath_deep = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 3.5}, {"yellow ochre", 2.5}, medium=0.18}
local p_heath_mid  = pile{{"raw umber", 7}, {"yellow ochre", 6}, {"red earth", 4}, {"bone black", 3}, medium=0.2}

-- Re-establish the glowing mist bank behind the knoll crest
local p_mist_glow = pile{{"lead white", 26}, {"yellow ochre", 4.0}, {"chrome yellow", 0.8}, {"smalt", 1.0}, {"raw umber", 0.3}, medium=0.3}
local m_mist_hollow = rect(0, 420, 1000, 45):soften(8)
work(m_mist_hollow, {hand="glaze", pile=p_mist_glow, angle=0, coverage=1.2, fill=true})
blend(rect(0, 420, 1000, 45), {angle=0})

-- The natural, swelling barrow crest profile:
-- Rising from y ≈ 475 at left margin, swelling up over the barrow crown at y ≈ 432 (x ≈ 340-440),
-- dipping into a saddle at y ≈ 456 (x ≈ 580), swelling to y ≈ 446 (x ≈ 760), then sloping to y ≈ 470 at right.
local pts_natural_barrow = {
  {-20, 478}, {60, 472}, {150, 462}, {240, 448}, {320, 436},
  {380, 431}, {440, 433}, {510, 442}, {580, 455}, {660, 458},
  {740, 448}, {820, 447}, {900, 458}, {1020, 472},
  {1020, 720}, {-20, 720}
}
local m_barrow = poly(pts_natural_barrow, true)

-- Body pass for the upper knoll slope to eliminate the flat cut shelf at y ≈ 435
work(m_barrow * rect(0, 425, 1000, 80), {
  hand = "body",
  pile = p_heath_deep,
  angle = 0.2,
  coverage = 1.8,
  fill = true,
  clip = true
})

-- Paint along the natural crest with a round brush for authentic organic turf contour
local b_crest = brush{kind="round", width=3.2, point=1, stiffness=0.8}
b_crest:load(p_heath_deep, 0.85)
b_crest:stroke({
  {-20, 478}, {60, 472}, {150, 462}, {240, 448}, {320, 436},
  {380, 431}, {440, 433}, {510, 442}, {580, 455}, {660, 458},
  {740, 448}, {820, 447}, {900, 458}, {1020, 472}
}, {pressure=0.85, ramps={0.02, 0.02}})

-- Blend gently downwards from the crest into the barrow body
blend(rect(0, 440, 1000, 60), {angle=1.4})

print("Moon refined, glowing mist restored, natural swelling barrow crest sculpted.")

--@ chunk 58
-- Chunk 58: Seamless Horizon Mist and Sculptural Barrow with Mathematical Crest

-- 1. Mathematical spline for the barrow crest
local crest_nodes = {
  {x=-20, y=478}, {x=80, y=472}, {x=170, y=462}, {x=260, y=448},
  {x=350, y=434}, {x=400, y=431}, {x=450, y=434}, {x=520, y=446},
  {x=590, y=458}, {x=670, y=460}, {x=750, y=450}, {x=820, y=448},
  {x=900, y=458}, {x=1020, y=474}
}

local function barrow_crest(x)
  if x <= crest_nodes[1].x then return crest_nodes[1].y end
  if x >= crest_nodes[#crest_nodes].x then return crest_nodes[#crest_nodes].y end
  for i = 1, #crest_nodes - 1 do
    local p0 = crest_nodes[i]
    local p1 = crest_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 460
end

local m_knoll = below(barrow_crest)

-- 2. Restore seamless horizon mist across the sky down into the crest area
local p_mist_amber = pile{{"lead white", 26}, {"yellow ochre", 4.2}, {"chrome yellow", 1.2}, {"vermilion", 0.3}, {"smalt", 1.0}, {"raw umber", 0.25}, medium=0.25}
local p_mist_cool  = pile{{"lead white", 28}, {"yellow ochre", 2.2}, {"smalt", 1.8}, {"raw umber", 0.3}, medium=0.3}

-- Glaze across the entire lower sky (y = 350 to 480) to eliminate any horizontal shelves
local m_lower_sky = rect(0, 350, 1000, 130)
work(m_lower_sky, {hand="glaze", pile=p_mist_amber, angle=0, coverage=1.4, fill=true})
blend(rect(0, 320, 1000, 160), {angle=0})
blend(rect(0, 320, 1000, 160), {angle=1.57})
blend(rect(0, 320, 1000, 160), {angle=0})

-- Distant coast line shimmering in the mist
local p_dist_coast = pile{{"lead white", 16}, {"smalt", 5.0}, {"raw umber", 2.8}, {"red earth", 0.8}, medium=0.25}
local pts_distant_land = {
  {0, 464}, {100, 460}, {220, 454}, {340, 452}, {460, 448},
  {580, 451}, {700, 447}, {820, 452}, {920, 458}, {1000, 464},
  {1000, 475}, {0, 475}
}
local m_coast = poly(pts_distant_land, true):soften(2.5)
work(m_coast, {hand="body", pile=p_dist_coast, angle=0, coverage=1.2, fill=true, clip=true})

-- Cool valley mist pooling over the water
local m_water_mist = rect(0, 445, 1000, 25):soften(4)
work(m_water_mist, {hand="glaze", pile=p_mist_cool, angle=0, coverage=1.0, fill=false})
blend(rect(0, 440, 1000, 35), {angle=0})

-- 3. Lay in the Grand Barrow Knoll from the crest all the way to canvas bottom
local p_earth_body   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4}, {"yellow ochre", 3}, medium=0.18}
local p_heath_dark   = pile{{"raw umber", 8}, {"bone black", 5}, {"red earth", 3.5}, {"green earth", 3.5}, medium=0.2}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5}, {"lead white", 2}, {"bone black", 1}, medium=0.2}
local p_peat_moss    = pile{{"green earth", 6}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}
local p_heath_gold   = pile{{"yellow ochre", 8}, {"raw umber", 4}, {"lead white", 3.5}, {"red earth", 2}, medium=0.2}

-- Solid foundation across the entire knoll down to canvas bottom (no bottom cutoff!)
work(m_knoll, {
  hand = "body",
  pile = p_earth_body,
  angle = 0.22,
  length = {50, 110},
  coverage = 2.4,
  fill = true,
  clip = true
})

-- Heather banks and peaty hollows following the natural slope
local m_h_russet = (ellipse(300, 520, 220, 45) + ellipse(740, 530, 240, 50)):soften(20) * m_knoll
work(m_h_russet, {hand="body", pile=p_heath_russet, angle=0.2, coverage=1.3, fill=true})

local m_h_moss = (ellipse(480, 580, 220, 45) + ellipse(180, 600, 160, 45)):soften(22) * m_knoll
work(m_h_moss, {hand="body", pile=p_peat_moss, angle=-0.15, coverage=1.2, fill=true})

local m_h_gold = (ellipse(360, 640, 280, 55) + ellipse(820, 630, 200, 50)):soften(25) * m_knoll
work(m_h_gold, {hand="body", pile=p_heath_gold, angle=0.25, coverage=1.3, fill=true})

-- Blend the knoll internally to unify the heathland while preserving the crisp crest
blend(m_knoll, {angle=0.2})
blend(m_knoll, {angle=-0.15})

-- Refine the crest contour stroke for a crisp, organic Baltic barrow edge
local b_crest = brush{kind="round", width=2.6, point=1, stiffness=0.85}
b_crest:load(p_earth_body, 0.9)
local crest_line = {}
for x = -10, 1010, 20 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest:stroke(crest_line, {pressure=0.85, ramps={0.02, 0.02}})

print("Seamless horizon mist established, barrow sculpted with mathematical crest and unified terrain.")

--@ chunk 59
-- Chunk 59: Seamless full-height sky blending (y = 0 to 435) to eliminate the horizontal shelf

-- 1. Blend the entire sky from the top of the canvas down behind the knoll crest
local m_sky_full_height = rect(0, 0, 1000, 435)
blend(m_sky_full_height, {angle=1.57})
blend(m_sky_full_height, {angle=0})

-- 2. Restore the delicate crescent moon and evening star after the full sky blend
local p_moon_brilliant = pile{{"lead white", 34}, {"yellow ochre", 1.0}, {"chrome yellow", 0.4}, medium=0.12}
local b_rigger = brush{kind="round", width=1.3, point=1, stiffness=0.85}
b_rigger:load(p_moon_brilliant, 0.95)
b_rigger:stroke({
  {645.2, 105.5}, {641.5, 108.0}, {639.0, 112.5}, {640.2, 117.0}, {644.0, 119.5}
}, {pressure={0.08, 0.85, 1.0, 0.8, 0.08}, ramps={0.05, 0.05}})

local b_star = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_star:load(p_moon_brilliant, 1.0)
b_star:touch(688, 148, {pressure=0.6})

print("Entire sky blended from zenith to knoll; moon and star restored.")

--@ chunk 60
-- Chunk 60: Unified wet-into-wet twilight sky across full height, eliminating all ghosts and shelves

-- 1. Four overlapping sky bands from zenith (y = 0) down behind the knoll (y = 440)
local p_sky_zenith = pile{{"lead white", 18}, {"cobalt blue", 6.0}, {"smalt", 4.2}, {"raw umber", 0.6}, {"bone black", 0.25}, medium=0.22}
local p_sky_upper  = pile{{"lead white", 26}, {"cobalt blue", 3.0}, {"smalt", 3.0}, {"yellow ochre", 0.8}, {"raw umber", 0.3}, medium=0.22}
local p_sky_mid    = pile{{"lead white", 28}, {"yellow ochre", 3.2}, {"chrome yellow", 0.8}, {"smalt", 1.0}, {"raw umber", 0.2}, {"vermilion", 0.1}, medium=0.22}
local p_sky_glow   = pile{{"lead white", 26}, {"yellow ochre", 5.0}, {"chrome yellow", 2.0}, {"vermilion", 0.5}, {"raw umber", 0.25}, medium=0.22}

-- Band 1: Zenith
local m1 = rect(0, 0, 1000, 145):soften(30)
work(m1, {hand="body", pile=p_sky_zenith, angle=0, order="across", coverage=1.8, fill=true})

-- Band 2: Upper mid-sky
local m2 = rect(0, 105, 1000, 150):soften(30)
work(m2, {hand="body", pile=p_sky_upper, angle=0, order="across", coverage=1.8, fill=true})

-- Band 3: Transition
local m3 = rect(0, 215, 1000, 140):soften(30)
work(m3, {hand="body", pile=p_sky_mid, angle=0, order="across", coverage=1.8, fill=true})

-- Band 4: Horizon glow (down to y = 445 behind barrow crest)
local m4 = rect(0, 315, 1000, 130):soften(25)
work(m4, {hand="body", pile=p_sky_glow, angle=0, order="across", coverage=1.8, fill=true})

-- 2. Fuse the four wet bands into a seamless atmospheric dome
local m_sky_all = rect(0, 0, 1000, 442)
blend(m_sky_all, {angle=1.57})
blend(m_sky_all, {angle=0})

-- 3. Distant low Baltic hills silhouetted in soft atmospheric haze
local p_dist_coast = pile{{"lead white", 15}, {"smalt", 5.0}, {"raw umber", 3.0}, {"red earth", 0.9}, {"bone black", 0.4}, medium=0.25}
local pts_distant_land = {
  {0, 464}, {100, 460}, {220, 454}, {340, 452}, {460, 448},
  {580, 451}, {700, 447}, {820, 452}, {920, 458}, {1000, 464},
  {1000, 475}, {0, 475}
}
local m_coast = poly(pts_distant_land, true):soften(2.5)
work(m_coast, {hand="body", pile=p_dist_coast, angle=0, coverage=1.2, fill=true, clip=true})

-- 4. Calm water sheen and valley mist
local p_water_mist = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 1.8}, {"raw umber", 0.3}, medium=0.3}
local m_water = rect(0, 442, 1000, 25):soften(4)
work(m_water, {hand="glaze", pile=p_water_mist, angle=0, coverage=1.0, fill=false})
blend(rect(0, 438, 1000, 30), {angle=0})

-- 5. Crisp, natural barrow crest contour restored over the horizon mist
local p_earth_body = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4}, {"yellow ochre", 3}, medium=0.18}
local b_crest = brush{kind="round", width=3.0, point=1, stiffness=0.85}
b_crest:load(p_earth_body, 0.9)

local crest_nodes = {
  {x=-20, y=478}, {x=80, y=472}, {x=170, y=462}, {x=260, y=448},
  {x=350, y=434}, {x=400, y=431}, {x=450, y=434}, {x=520, y=446},
  {x=590, y=458}, {x=670, y=460}, {x=750, y=450}, {x=820, y=448},
  {x=900, y=458}, {x=1020, y=474}
}
local function barrow_crest(x)
  if x <= crest_nodes[1].x then return crest_nodes[1].y end
  if x >= crest_nodes[#crest_nodes].x then return crest_nodes[#crest_nodes].y end
  for i = 1, #crest_nodes - 1 do
    local p0 = crest_nodes[i]
    local p1 = crest_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 460
end

local crest_line = {}
for x = -10, 1010, 15 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest:stroke(crest_line, {pressure=0.9, ramps={0.02, 0.02}})

-- 6. Waxing crescent moon and evening star
local p_moon_pure = pile{{"lead white", 34}, {"yellow ochre", 1.0}, {"chrome yellow", 0.4}, medium=0.12}
local b_rigger = brush{kind="round", width=1.3, point=1, stiffness=0.85}
b_rigger:load(p_moon_pure, 0.95)
b_rigger:stroke({
  {645.2, 105.5}, {641.5, 108.0}, {639.0, 112.5}, {640.2, 117.0}, {644.0, 119.5}
}, {pressure={0.08, 0.85, 1.0, 0.8, 0.08}, ramps={0.05, 0.05}})

local b_star = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_star:load(p_moon_pure, 1.0)
b_star:touch(688, 148, {pressure=0.6})

print("Wet-into-wet sky, distant hills, barrow crest, moon and star completed seamlessly.")

--@ chunk 61
-- Chunk 61: Sculpting the Majestic Swelling Barrow Mound

local barrow_nodes = {
  {x=-20, y=475}, {x=80, y=468}, {x=180, y=456}, {x=270, y=438},
  {x=340, y=422}, {x=390, y=418}, {x=450, y=422}, {x=520, y=438},
  {x=600, y=455}, {x=680, y=456}, {x=760, y=442}, {x=830, y=442},
  {x=910, y=455}, {x=1020, y=472}
}

function barrow_crest(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 450
end

m_barrow = below(barrow_crest)

-- Earth and heather palette
local p_earth_core   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4}, {"yellow ochre", 3}, medium=0.18}
local p_heath_dark   = pile{{"raw umber", 8}, {"bone black", 5}, {"red earth", 3.5}, {"green earth", 3.5}, medium=0.2}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5}, {"lead white", 2}, {"bone black", 1}, medium=0.2}
local p_peat_moss    = pile{{"green earth", 6}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}
local p_heath_gold   = pile{{"yellow ochre", 8}, {"raw umber", 4}, {"lead white", 3.5}, {"red earth", 2}, medium=0.2}

-- 1. Solid foundation covering the upper slope up to the new elevated crest (y ≈ 418)
work(m_barrow, {
  hand = "body",
  pile = p_earth_core,
  angle = 0.22,
  length = {45, 95},
  coverage = 2.2,
  fill = true,
  clip = true
})

-- 2. Tactile banks of late-autumn heather and peat swells
-- Barrow summit bank around the megaliths
local m_summit_bank = (ellipse(390, 445, 160, 32) + ellipse(260, 460, 140, 35)):soften(15) * m_barrow
work(m_summit_bank, {hand="body", pile=p_heath_russet, angle=0.18, coverage=1.4, fill=true})

-- Right knoll bank
local m_rknoll_bank = ellipse(770, 465, 160, 35):soften(15) * m_barrow
work(m_rknoll_bank, {hand="body", pile=p_heath_russet, angle=-0.15, coverage=1.3, fill=true})

-- Mid-slope peaty moss hollow
local m_hollow = (ellipse(520, 525, 200, 40) + ellipse(180, 535, 150, 40)):soften(18) * m_barrow
work(m_hollow, {hand="body", pile=p_peat_moss, angle=0.25, coverage=1.2, fill=true})

-- Lower foreground warm heather ridges
local m_low_heather = (ellipse(350, 600, 260, 50) + ellipse(780, 590, 220, 45)):soften(20) * m_barrow
work(m_low_heather, {hand="body", pile=p_heath_gold, angle=0.2, coverage=1.3, fill=true})

-- Deep peaty base at the bottom margin
local m_bottom = rect(0, 640, 1000, 70):soften(15) * m_barrow
work(m_bottom, {hand="body", pile=p_earth_core, angle=0.1, coverage=1.4, fill=true})

-- 3. Crisp, natural barrow crest contour
local b_crest = brush{kind="round", width=2.8, point=1, stiffness=0.85}
b_crest:load(p_earth_core, 0.95)
local crest_line = {}
for x = -10, 1010, 15 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest:stroke(crest_line, {pressure=0.9, ramps={0.02, 0.02}})

-- Blend the interior of the knoll to fuse the heather into smooth, sculptural earth
blend(m_barrow, {angle=0.2})
blend(m_barrow, {angle=-0.15})

print("Majestic swelling barrow mound sculpted.")

--@ chunk 62
-- Chunk 62: Smoothing the Barrow Crest and Sculpting the Monumental Megalithic Tomb (Hünengrab)

-- 1. Smooth the barrow crest contour with a firm, continuous stroke
local p_peat_crest = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 3.5}, {"yellow ochre", 2.0}, medium=0.15}
local b_crest_firm = brush{kind="round", width=2.8, point=1, stiffness=0.9}
b_crest_firm:load(p_peat_crest, 0.95)

local crest_line = {}
for x = -10, 1010, 10 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest_firm:stroke(crest_line, {pressure=0.85, ramps={0.01, 0.01}})

-- 2. Piles for Ancient Swedish Granite (weathered Baltic erratic)
local p_cavern_pitch  = pile{{"bone black", 12}, {"raw umber", 7}, medium=0.1}
local p_granite_sh    = pile{{"raw umber", 8}, {"bone black", 6}, {"smalt", 2.5}, {"red earth", 1.2}, {"lead white", 1.5}, medium=0.18}
local p_granite_mid   = pile{{"lead white", 10}, {"raw umber", 6.5}, {"bone black", 3.0}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"red earth", 1.0}, medium=0.2}
local p_granite_lit   = pile{{"lead white", 22}, {"smalt", 2.5}, {"yellow ochre", 2.8}, {"raw umber", 1.2}, {"vermilion", 0.3}, medium=0.25}
local p_lichen_sage   = pile{{"lead white", 16}, {"green earth", 5.5}, {"yellow ochre", 3.5}, {"raw umber", 1.0}, medium=0.25}
local p_lichen_gold   = pile{{"lead white", 12}, {"yellow ochre", 8.5}, {"chrome yellow", 1.5}, {"raw umber", 0.8}, medium=0.25}
local p_turf_contact  = pile{{"bone black", 9}, {"raw umber", 8}, {"green earth", 3}, medium=0.15}

-- 3. The Sacred Burial Chamber Void (Deep shadow inside the dolmen)
local pts_chamber = {
  {342, 412}, {456, 412}, {454, 442}, {340, 442}
}
local m_chamber = poly(pts_chamber, true)
work(m_chamber, {hand="body", pile=p_cavern_pitch, angle=0, coverage=2.0, fill=true, clip=true})

-- 4. The Three Heavy Support Orthostats (Glacial granite uprights)
-- A. Left Orthostat (weathered, leaning slightly inward)
local pts_lo = {
  {340, 412}, {364, 412}, {368, 444}, {342, 444}, {338, 430}
}
local m_lo = poly(pts_lo)
work(m_lo, {hand="detail", pile=p_granite_mid, angle=1.45, coverage=1.6, fill=true, clip=true})
-- Shadow side (interior facing chamber)
local m_lo_sh = poly({{354, 412}, {364, 412}, {368, 444}, {358, 444}})
work(m_lo_sh, {hand="detail", pile=p_granite_sh, angle=1.5, coverage=1.4, fill=true, clip=true})
-- Light outer edge catching twilight sheen
local b_det = brush{kind="round", width=1.5, point=1, stiffness=0.85}
b_det:load(p_granite_lit, 0.7)
b_det:stroke({{340, 412}, {338, 430}, {342, 444}}, {pressure={0.7, 0.3}})

-- B. Center-Rear Orthostat (dimly seen within the chamber)
local pts_ro_mid = {{386, 412}, {412, 412}, {410, 438}, {388, 438}}
local m_mid = poly(pts_ro_mid)
work(m_mid, {hand="detail", pile=p_granite_sh, angle=1.5, coverage=1.5, fill=true, clip=true})

-- C. Right Orthostat (sturdy, crystalline, catching the evening light)
local pts_ro = {
  {430, 414}, {456, 414}, {458, 445}, {432, 445}, {428, 430}
}
local m_ro = poly(pts_ro)
work(m_ro, {hand="detail", pile=p_granite_mid, angle=1.4, coverage=1.6, fill=true, clip=true})
-- Outer plane catching twilight
local m_ro_lit = poly({{444, 414}, {456, 414}, {458, 445}, {448, 445}})
work(m_ro_lit, {hand="detail", pile=p_granite_lit, angle=1.4, coverage=1.3, fill=true, clip=true})

-- 5. The Monumental Granite Capstone (Massive glacial erratic slab)
local pts_capstone = {
  {318, 412}, {326, 403}, {344, 396}, {378, 392}, {418, 391}, {452, 394}, {474, 401},
  {478, 410}, {460, 415}, {420, 414}, {375, 415}, {332, 416}, {318, 412}
}
local m_capstone = poly(pts_capstone, true)
work(m_capstone, {hand="body", pile=p_granite_mid, angle=0.08, coverage=1.8, fill=true, clip=true})

-- Deep cast shadow directly under the capstone overhang
local pts_undercut = {
  {318, 412}, {332, 416}, {375, 415}, {420, 414}, {460, 415}, {478, 410},
  {472, 416}, {430, 417}, {380, 418}, {336, 419}, {318, 412}
}
local m_undercut = poly(pts_undercut)
work(m_undercut, {hand="detail", pile=p_cavern_pitch, angle=0, coverage=1.8, fill=true, clip=true})

-- Luminous upper facets catching the silvery twilight dome
local pts_cap_top = {
  {318, 412}, {326, 403}, {344, 396}, {378, 392}, {418, 391}, {452, 394}, {474, 401},
  {468, 404}, {435, 398}, {395, 396}, {355, 399}, {328, 406}
}
local m_cap_top = poly(pts_cap_top, true)
work(m_cap_top, {hand="detail", pile=p_granite_lit, angle=0.05, coverage=1.6, fill=true, clip=true})

-- Facet cleavage cracks and crystalline texture on capstone
b_det:load(p_granite_sh, 0.75)
b_det:stroke({{366, 395}, {370, 404}, {368, 414}}, {pressure={0.5, 0.15}})
b_det:stroke({{424, 394}, {428, 403}, {432, 414}}, {pressure={0.5, 0.15}})

-- Lichen encrustations on the ancient stone (pale sage-green and warm ochre)
local m_lich1 = (ellipse(350, 398, 14, 5) + ellipse(440, 397, 16, 5)):soften(1.5) * m_capstone
work(m_lich1, {hand="detail", pile=p_lichen_sage, angle=0.1, coverage=1.2, fill=false})

local m_lich2 = (ellipse(385, 395, 12, 4) + ellipse(460, 400, 10, 4)):soften(1.5) * m_capstone
work(m_lich2, {hand="detail", pile=p_lichen_gold, angle=0.1, coverage=1.1, fill=false})

-- 6. Surrounding Glacial Boulders flanking the Dolmen
-- Leaning stone left (x ≈ 298 to 326, y ≈ 424 to 446)
local pts_sl = {{298, 442}, {306, 428}, {322, 424}, {328, 432}, {324, 446}, {304, 446}}
local m_sl = poly(pts_sl, true)
work(m_sl, {hand="detail", pile=p_granite_mid, angle=0.3, coverage=1.5, fill=true, clip=true})
local m_sl_top = poly({{306, 428}, {322, 424}, {328, 432}, {318, 434}, {308, 432}})
work(m_sl_top, {hand="detail", pile=p_granite_lit, angle=0.15, coverage=1.3, fill=true, clip=true})

-- Fallen stone right (x ≈ 470 to 506, y ≈ 422 to 444)
local pts_sr = {{470, 436}, {482, 424}, {502, 422}, {510, 434}, {502, 444}, {476, 444}}
local m_sr = poly(pts_sr, true)
work(m_sr, {hand="detail", pile=p_granite_mid, angle=-0.2, coverage=1.5, fill=true, clip=true})
local m_sr_top = poly({{482, 424}, {502, 422}, {510, 434}, {498, 433}, {486, 430}})
work(m_sr_top, {hand="detail", pile=p_granite_lit, angle=-0.1, coverage=1.3, fill=true, clip=true})

-- Bed all orthostats and flanking stones deeply into dark peaty turf
local b_turf = brush{kind="round", width=3.2, point=1, stiffness=0.85}
b_turf:load(p_turf_contact, 0.9)
b_turf:stroke({{292, 446}, {372, 446}}, {pressure={0.85, 0.6}})
b_turf:stroke({{422, 446}, {516, 446}}, {pressure={0.85, 0.6}})

print("Crest smoothed, Megalithic Tomb sculpted with monumental granite capstone and dark chamber.")

--@ chunk 63
local pts = {{10, 10}, {20, 10}, {20, 20}}
local m = poly(pts, false)
print("Poly false ok:", m)

--@ chunk 64
local b = brush{kind="round", width=2}
local p = pile{{"lead white", 10}}
b:load(p, 0.5)
b:touch(100, 100, {pressure=0.5, drag={2.2, 0.1}})
print("touch ok")

--@ chunk 65
-- Test part 1 of chunk 63
local p_cavern_void   = pile{{"bone black", 12}, {"raw umber", 8}, medium=0.1}
local p_granite_deep  = pile{{"bone black", 8}, {"raw umber", 7}, {"smalt", 2.0}, {"red earth", 1.5}, medium=0.15}
local p_granite_shade = pile{{"raw umber", 8}, {"bone black", 5}, {"smalt", 2.2}, {"lead white", 2.5}, {"red earth", 1.2}, {"yellow ochre", 1.5}, medium=0.18}
local p_granite_body  = pile{{"lead white", 10}, {"raw umber", 6}, {"bone black", 3}, {"yellow ochre", 3.2}, {"smalt", 1.8}, {"red earth", 1.2}, medium=0.2}
local p_granite_facet = pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.22}
local p_granite_light = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}

local b_block = brush{kind="filbert", width=5.5, stiffness=0.85}
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_crack = brush{kind="round", width=0.9, point=1, stiffness=0.9}

-- 1. Chamber
b_block:load(p_cavern_void, 0.95)
b_block:stroke({{338, 416}, {385, 415}, {425, 415}, {456, 417}}, {pressure=0.9})
b_block:stroke({{344, 426}, {395, 428}, {450, 427}}, {pressure=0.9})
b_block:stroke({{350, 436}, {400, 438}, {446, 437}}, {pressure=0.9})
b_facet:load(p_cavern_void, 0.9)
b_facet:stroke({{365, 416}, {368, 442}}, {pressure={0.8, 0.4}})
b_facet:stroke({{425, 416}, {422, 442}}, {pressure={0.8, 0.4}})

-- 2. Left Orthostat
b_block:load(p_granite_shade, 0.9)
b_block:stroke({{336, 414}, {340, 428}, {344, 445}}, {pressure=0.85})
b_block:stroke({{346, 414}, {350, 428}, {355, 445}}, {pressure=0.85})
b_block:stroke({{356, 414}, {360, 428}, {365, 444}}, {pressure=0.8})

b_facet:load(p_granite_facet, 0.8)
b_facet:stroke({{336, 414}, {338, 425}, {342, 436}, {344, 446}}, {pressure={0.7, 0.5, 0.6, 0.2}})
b_facet:stroke({{342, 420}, {348, 426}, {350, 438}}, {pressure={0.5, 0.6, 0.3}})

print("Part 1 ok")

--@ chunk 66
-- Part 2: Rear and Right Orthostats, Capstone, Lichen, Flanking Erratics

local p_cavern_void   = pile{{"bone black", 12}, {"raw umber", 8}, medium=0.1}
local p_granite_deep  = pile{{"bone black", 8}, {"raw umber", 7}, {"smalt", 2.0}, {"red earth", 1.5}, medium=0.15}
local p_granite_shade = pile{{"raw umber", 8}, {"bone black", 5}, {"smalt", 2.2}, {"lead white", 2.5}, {"red earth", 1.2}, {"yellow ochre", 1.5}, medium=0.18}
local p_granite_body  = pile{{"lead white", 10}, {"raw umber", 6}, {"bone black", 3}, {"yellow ochre", 3.2}, {"smalt", 1.8}, {"red earth", 1.2}, medium=0.2}
local p_granite_facet = pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.22}
local p_granite_light = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage     = pile{{"lead white", 16}, {"green earth", 6.5}, {"yellow ochre", 3.5}, {"raw umber", 1.0}, medium=0.25}
local p_lich_gold     = pile{{"lead white", 12}, {"yellow ochre", 9.0}, {"chrome yellow", 1.8}, {"raw umber", 0.8}, medium=0.25}
local p_peat_dark     = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.0}, medium=0.15}

local b_block = brush{kind="filbert", width=5.5, stiffness=0.85}
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_crack = brush{kind="round", width=0.9, point=1, stiffness=0.9}

-- 3. Center Rear Orthostat
b_facet:load(p_granite_deep, 0.85)
b_facet:stroke({{392, 415}, {394, 430}, {396, 440}}, {pressure=0.75})
b_facet:stroke({{404, 415}, {406, 430}, {408, 440}}, {pressure=0.75})

-- 4. Right Orthostat
b_block:load(p_granite_shade, 0.9)
b_block:stroke({{428, 415}, {432, 428}, {434, 445}}, {pressure=0.85})
b_block:stroke({{438, 415}, {442, 428}, {445, 445}}, {pressure=0.85})
b_block:stroke({{448, 414}, {452, 426}, {455, 444}}, {pressure=0.85})

b_facet:load(p_granite_facet, 0.85)
b_facet:stroke({{448, 414}, {452, 424}, {456, 435}, {458, 445}}, {pressure={0.75, 0.8, 0.6, 0.2}})
b_facet:load(p_granite_light, 0.8)
b_facet:stroke({{450, 414}, {454, 420}, {452, 428}}, {pressure={0.65, 0.5, 0.2}})

-- 5. Capstone Underside & Front
b_block:load(p_granite_deep, 0.95)
b_block:stroke({{316, 413}, {355, 414}, {400, 415}, {445, 414}, {476, 411}}, {pressure=0.9})
b_crack:load(p_cavern_void, 0.95)
b_crack:stroke({{314, 414}, {350, 416}, {395, 417}, {440, 416}, {478, 413}}, {pressure=0.85})

b_block:load(p_granite_body, 0.9)
b_block:stroke({{318, 410}, {350, 411}, {390, 412}, {430, 411}, {470, 408}}, {pressure=0.85})
b_block:stroke({{324, 404}, {360, 405}, {405, 406}, {445, 405}, {472, 403}}, {pressure=0.85})

-- Top Facets
local m_fA = poly({{314, 411}, {322, 402}, {340, 394}, {368, 390}, {394, 389}, {392, 402}, {365, 404}, {338, 407}, {318, 411}})
work(m_fA, {hand="detail", pile=p_granite_light, angle=0.04, coverage=1.6, fill=true, clip=true})

local m_fB = poly({{394, 389}, {425, 391}, {455, 394}, {474, 400}, {478, 408}, {468, 409}, {440, 406}, {412, 404}, {392, 402}})
work(m_fB, {hand="detail", pile=p_granite_facet, angle=-0.04, coverage=1.6, fill=true, clip=true})

-- Fracture lines
b_crack:load(p_granite_deep, 0.9)
b_crack:stroke({{394, 389}, {392, 402}, {390, 416}}, {pressure={0.7, 0.85, 0.3}})
b_crack:stroke({{348, 393}, {352, 404}, {350, 414}}, {pressure={0.45, 0.6, 0.2}})
b_crack:stroke({{442, 393}, {444, 403}, {446, 414}}, {pressure={0.5, 0.65, 0.2}})

-- Top edge highlights
b_crack:load(p_granite_light, 0.95)
b_crack:stroke({{314, 411}, {322, 402}, {340, 394}, {368, 390}, {394, 389}}, {pressure={0.3, 0.8, 0.7, 0.85, 0.4}})
b_crack:stroke({{396, 390}, {425, 391}, {455, 394}, {474, 400}}, {pressure={0.4, 0.8, 0.75, 0.3}})

-- Lichen touches
local lichen_patches = {
  {335, 398, p_lich_sage, 2.2},
  {362, 394, p_lich_gold, 2.5},
  {380, 400, p_lich_sage, 1.8},
  {415, 396, p_lich_gold, 2.4},
  {438, 398, p_lich_sage, 2.0},
  {460, 402, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichen_patches) do
  b_facet:load(lp[3], 0.7)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- Flanking Erratics
local m_el = poly({{292, 444}, {300, 428}, {316, 422}, {326, 430}, {322, 446}, {296, 446}})
work(m_el, {hand="detail", pile=p_granite_body, angle=0.3, coverage=1.6, fill=true, clip=true})
local m_el_lit = poly({{300, 428}, {316, 422}, {326, 430}, {314, 434}, {304, 433}})
work(m_el_lit, {hand="detail", pile=p_granite_facet, angle=0.15, coverage=1.4, fill=true, clip=true})

local m_er = poly({{470, 444}, {478, 426}, {504, 422}, {516, 434}, {508, 446}, {474, 446}})
work(m_er, {hand="detail", pile=p_granite_body, angle=-0.2, coverage=1.6, fill=true, clip=true})
local m_er_lit = poly({{478, 426}, {504, 422}, {516, 434}, {500, 433}, {484, 431}})
work(m_er_lit, {hand="detail", pile=p_granite_facet, angle=-0.1, coverage=1.4, fill=true, clip=true})

-- Bed into turf
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{285, 446}, {330, 446}, {370, 446}}, {pressure=0.9})
b_block:stroke({{420, 446}, {470, 446}, {525, 446}}, {pressure=0.9})

print("Part 2 ok: Dolmen sculpted")

--@ chunk 67
-- Chunk 67: The Ancient Veteran Oak — Trunk, Root Buttresses, Fissured Bark, and Muscular Boughs

-- 1. Piles for Living Oak Bark and Bleached Stag Wood
local p_bark_core    = pile{{"bone black", 12}, {"raw umber", 8}, {"red earth", 2.0}, medium=0.15}
local p_bark_mid     = pile{{"raw umber", 8}, {"bone black", 5}, {"yellow ochre", 3.5}, {"lead white", 2.2}, {"red earth", 1.2}, medium=0.2}
local p_bark_ridge   = pile{{"raw umber", 5}, {"yellow ochre", 6.0}, {"lead white", 5.0}, {"bone black", 1.5}, medium=0.22}
local p_hollow_pitch = pile{{"bone black", 14}, {"raw umber", 6}, medium=0.1}
local p_woundwood    = pile{{"raw umber", 6}, {"yellow ochre", 5.5}, {"lead white", 3.8}, {"red earth", 2.0}, medium=0.2}
local p_stag_core    = pile{{"raw umber", 8}, {"bone black", 4}, {"lead white", 6}, {"yellow ochre", 3}, medium=0.2}
local p_stag_bleach  = pile{{"lead white", 18}, {"raw umber", 3.5}, {"yellow ochre", 2.5}, {"smalt", 1.0}, {"bone black", 0.5}, medium=0.22}
local p_heath_blend  = pile{{"raw umber", 9}, {"bone black", 6}, {"red earth", 3.5}, {"green earth", 3.0}, medium=0.18}

local b_bole   = brush{kind="filbert", width=8, stiffness=0.85}
local b_limb   = brush{kind="filbert", width=5, stiffness=0.85}
local b_branch = brush{kind="round", width=3.2, point=1, stiffness=0.85}
local b_detail = brush{kind="round", width=1.5, point=1, stiffness=0.85}

-- 2. First: Clean up the sausage smudges around the dolmen with fresh heathland turf
b_bole:load(p_heath_blend, 0.95)
b_bole:stroke({{250, 442}, {295, 444}, {330, 446}}, {pressure=0.9})
b_bole:stroke({{455, 444}, {490, 442}, {530, 440}}, {pressure=0.9})

-- 3. Trunk & Root Buttresses of the Ancient Oak (x ≈ 240 to 320, y ≈ 435 to 458)
-- Western flaring root buttress
b_bole:load(p_bark_core, 0.95)
b_bole:stroke({{232, 456}, {248, 448}, {262, 435}, {268, 410}, {270, 380}}, {pressure={0.9, 0.95, 0.9, 0.85, 0.8}})
-- Central root buttress
b_bole:stroke({{265, 458}, {272, 446}, {276, 420}, {278, 385}, {276, 360}}, {pressure={0.95, 0.95, 0.9, 0.85, 0.8}})
-- Eastern root buttress gripping the barrow and leaning dolmen stone
b_bole:stroke({{318, 452}, {305, 444}, {294, 425}, {288, 395}, {284, 365}}, {pressure={0.95, 0.9, 0.85, 0.8, 0.8}})

-- Solid body fill of trunk
b_bole:load(p_bark_core, 0.9)
b_bole:stroke({{252, 445}, {258, 415}, {264, 380}, {268, 355}}, {pressure=0.9})
b_bole:stroke({{280, 445}, {284, 415}, {282, 380}, {280, 355}}, {pressure=0.9})
b_bole:stroke({{298, 445}, {296, 415}, {290, 380}, {284, 355}}, {pressure=0.9})

-- 4. Longitudinal Bark Furrows and Textural Ridges
-- Deep shadow furrows
b_detail:load(p_hollow_pitch, 0.9)
b_detail:stroke({{246, 450}, {255, 432}, {260, 410}, {262, 375}}, {pressure={0.8, 0.6, 0.3}})
b_detail:stroke({{266, 452}, {270, 435}, {272, 410}, {270, 370}}, {pressure={0.8, 0.7, 0.4}})
b_detail:stroke({{292, 448}, {290, 430}, {286, 405}, {282, 375}}, {pressure={0.8, 0.6, 0.4}})
b_detail:stroke({{306, 448}, {300, 435}, {294, 415}, {288, 385}}, {pressure={0.8, 0.5, 0.3}})

-- Raised weathered bark ridges catching the evening light
b_detail:load(p_bark_ridge, 0.85)
b_detail:stroke({{240, 454}, {250, 442}, {256, 420}, {260, 390}, {262, 365}}, {pressure={0.4, 0.75, 0.7, 0.6, 0.2}})
b_detail:stroke({{260, 454}, {266, 438}, {268, 415}, {268, 380}}, {pressure={0.3, 0.7, 0.65, 0.3}})
b_detail:stroke({{276, 454}, {278, 438}, {278, 410}, {276, 375}}, {pressure={0.3, 0.65, 0.6, 0.2}})
b_detail:stroke({{300, 450}, {296, 436}, {290, 412}, {284, 380}}, {pressure={0.3, 0.7, 0.65, 0.2}})

-- 5. The Ancient Hollow: weathered rot cavity framed by rounded wound-wood calluses
-- Cavity void (raw umber + bone black)
local pts_void = {{274, 432}, {284, 428}, {286, 402}, {278, 396}, {272, 416}}
local m_void = poly(pts_void, true)
work(m_void, {hand="detail", pile=p_hollow_pitch, angle=1.5, coverage=1.8, fill=true, clip=true})

-- Wound-wood calluses wrapping the hollow edges
b_detail:load(p_woundwood, 0.85)
b_detail:stroke({{271, 435}, {272, 416}, {278, 395}}, {pressure={0.5, 0.85, 0.4}})
b_detail:stroke({{286, 430}, {287, 404}, {280, 395}}, {pressure={0.5, 0.85, 0.4}})
b_detail:load(p_bark_ridge, 0.7)
b_detail:stroke({{270, 428}, {271, 416}, {275, 400}}, {pressure={0.3, 0.6, 0.2}})

-- 6. The Great Eastern Bough: muscular living arm arching over the Megalithic Tomb
-- Main stem (heavy taper from collar at x=284, y=365 to outer tip at x=535, y=305)
b_bole:load(p_bark_core, 0.95)
b_bole:stroke({
  {284, 365}, {325, 355}, {370, 342}, {415, 332}, {460, 320}, {500, 305}, {535, 290}
}, {pressure={0.95, 0.85, 0.75, 0.65, 0.55, 0.45, 0.25}, ramps={0.05, 0.15}})

-- Branch collar swelling at base of eastern limb
b_limb:load(p_bark_core, 0.9)
b_limb:stroke({{280, 375}, {292, 372}, {304, 362}}, {pressure=0.85})
b_detail:load(p_bark_ridge, 0.8)
b_detail:stroke({{282, 358}, {296, 352}, {312, 348}}, {pressure={0.3, 0.75, 0.2}})

-- Secondary boughs off Eastern limb:
-- E1: Upward muscular bough
b_branch:load(p_bark_core, 0.9)
b_branch:stroke({
  {365, 344}, {382, 318}, {402, 288}, {420, 258}, {435, 230}
}, {pressure={0.85, 0.75, 0.65, 0.5, 0.2}, ramps={0.05, 0.15}})
-- E1 side branches
b_detail:load(p_bark_core, 0.85)
b_detail:stroke({{382, 318}, {374, 294}, {368, 268}}, {pressure={0.7, 0.5, 0.2}})
b_detail:stroke({{402, 288}, {416, 264}, {424, 240}}, {pressure={0.65, 0.45, 0.2}})

-- E2: Sheltering bough directly over dolmen capstone
b_branch:load(p_bark_core, 0.9)
b_branch:stroke({
  {420, 332}, {442, 348}, {468, 362}, {492, 372}
}, {pressure={0.8, 0.65, 0.5, 0.2}, ramps={0.05, 0.15}})
b_detail:load(p_bark_core, 0.85)
b_detail:stroke({{468, 362}, {480, 380}, {490, 395}}, {pressure={0.6, 0.4, 0.15}})

-- E3: Outer spreading canopy boughs
b_branch:load(p_bark_core, 0.85)
b_branch:stroke({
  {470, 318}, {495, 295}, {518, 275}, {538, 255}
}, {pressure={0.75, 0.6, 0.45, 0.2}, ramps={0.05, 0.15}})
b_branch:stroke({
  {500, 305}, {525, 318}, {550, 328}, {570, 335}
}, {pressure={0.7, 0.55, 0.4, 0.2}, ramps={0.05, 0.15}})

-- 7. The Western Living Bough: reaching west over the barrow flank
b_limb:load(p_bark_core, 0.95)
b_limb:stroke({
  {265, 360}, {230, 345}, {195, 328}, {160, 312}, {125, 298}, {90, 285}
}, {pressure={0.9, 0.8, 0.7, 0.55, 0.4, 0.2}, ramps={0.05, 0.15}})

-- W1: Upward western bough
b_branch:load(p_bark_core, 0.9)
b_branch:stroke({
  {210, 335}, {194, 305}, {178, 275}, {165, 245}, {150, 218}
}, {pressure={0.8, 0.65, 0.5, 0.4, 0.2}, ramps={0.05, 0.15}})
b_detail:load(p_bark_core, 0.85)
b_detail:stroke({{194, 305}, {205, 280}, {212, 255}}, {pressure={0.6, 0.4, 0.15}})

-- W2: Drooping western bough
b_branch:load(p_bark_core, 0.85)
b_branch:stroke({
  {175, 320}, {155, 342}, {135, 362}, {112, 380}
}, {pressure={0.75, 0.6, 0.45, 0.2}, ramps={0.05, 0.15}})

-- W3: Outer western branchlets
b_detail:load(p_bark_core, 0.85)
b_detail:stroke({{125, 298}, {108, 275}, {92, 255}}, {pressure={0.65, 0.45, 0.15}})
b_detail:stroke({{125, 298}, {110, 315}, {95, 330}}, {pressure={0.6, 0.4, 0.15}})

-- 8. The Retrenched Bleached Stag-Head Crown aloft
-- Main dead spire (spearing straight toward the cobalt zenith)
b_branch:load(p_stag_core, 0.9)
b_branch:stroke({
  {272, 340}, {268, 295}, {264, 245}, {260, 190}, {255, 135}, {250, 85}
}, {pressure={0.85, 0.75, 0.65, 0.5, 0.35, 0.15}, ramps={0.05, 0.15}})

-- Stag horn 1 (eastern dead bough)
b_branch:stroke({
  {268, 280}, {285, 240}, {305, 195}, {322, 150}, {335, 105}
}, {pressure={0.8, 0.65, 0.5, 0.35, 0.15}, ramps={0.05, 0.15}})
b_detail:load(p_stag_core, 0.85)
b_detail:stroke({{285, 240}, {278, 210}, {272, 180}}, {pressure={0.6, 0.4, 0.15}})
b_detail:stroke({{305, 195}, {318, 168}, {326, 140}}, {pressure={0.55, 0.35, 0.15}})

-- Stag horn 2 (western dead bough)
b_branch:stroke({
  {266, 290}, {245, 250}, {225, 205}, {208, 160}, {192, 115}
}, {pressure={0.8, 0.65, 0.5, 0.35, 0.15}, ramps={0.05, 0.15}})
b_detail:load(p_stag_core, 0.85)
b_detail:stroke({{245, 250}, {250, 220}, {254, 190}}, {pressure={0.6, 0.4, 0.15}})
b_detail:stroke({{225, 205}, {212, 175}, {200, 145}}, {pressure={0.55, 0.35, 0.15}})

-- Stag horn 3 (upper right spire)
b_detail:load(p_stag_core, 0.85)
b_detail:stroke({{262, 215}, {276, 180}, {288, 145}, {296, 110}}, {pressure={0.65, 0.5, 0.3, 0.12}})

-- Bleached wood highlights on the sky-facing surfaces of dead stag boughs
b_detail:load(p_stag_bleach, 0.9)
b_detail:stroke({{270, 335}, {266, 292}, {262, 242}, {258, 188}, {253, 132}, {248, 85}}, {pressure={0.3, 0.6, 0.55, 0.4, 0.25, 0.08}})
b_detail:stroke({{268, 278}, {286, 238}, {306, 192}, {323, 148}, {336, 105}}, {pressure={0.25, 0.55, 0.5, 0.35, 0.1}})
b_detail:stroke({{265, 288}, {244, 248}, {224, 202}, {207, 158}, {191, 115}}, {pressure={0.25, 0.55, 0.5, 0.35, 0.1}})

-- Splintered tips on dead antlers
b_detail:load(p_stag_bleach, 0.95)
b_detail:stroke({{250, 88}, {248, 80}}, {pressure={0.4, 0.05}})
b_detail:stroke({{250, 88}, {253, 82}}, {pressure={0.3, 0.05}})
b_detail:stroke({{335, 107}, {338, 100}}, {pressure={0.35, 0.05}})
b_detail:stroke({{192, 117}, {189, 110}}, {pressure={0.35, 0.05}})

print("Veteran Oak trunk, root buttresses, hollow, living limbs, and bleached stag crown sculpted.")

--@ chunk 68
-- Chunk 68: Solidifying the Ancient Oak Trunk, Muscular Ribbons, and Rich Autumn Foliage Masses

-- 1. Piles for Trunk, Boughs, and Foliage
local p_bark_solid   = pile{{"bone black", 12}, {"raw umber", 8}, {"red earth", 2.0}, medium=0.15}
local p_bark_ridge   = pile{{"raw umber", 6}, {"yellow ochre", 6.5}, {"lead white", 4.5}, {"bone black", 1.5}, medium=0.2}
local p_hollow_void  = pile{{"bone black", 14}, {"raw umber", 6}, medium=0.1}
local p_woundwood    = pile{{"raw umber", 6}, {"yellow ochre", 5.5}, {"lead white", 3.5}, {"red earth", 2.0}, medium=0.2}

local p_fol_deep     = pile{{"bone black", 8}, {"raw umber", 10}, {"red earth", 3.5}, {"yellow ochre", 2.0}, medium=0.2}
local p_fol_russet   = pile{{"yellow ochre", 7}, {"red earth", 6.0}, {"raw umber", 4.0}, {"lead white", 1.8}, {"bone black", 0.8}, medium=0.22}
local p_fol_gold     = pile{{"yellow ochre", 9}, {"chrome yellow", 2.0}, {"lead white", 4.5}, {"red earth", 1.5}, {"raw umber", 0.8}, {"vermilion", 0.3}, medium=0.25}

-- 2. Massive, Solid Trunk (100% opaque to obliterate any ghosting)
local pts_trunk = {
  {230, 456}, {240, 448}, {250, 430}, {256, 400}, {260, 365}, {258, 335},
  {275, 325}, {295, 335}, {306, 365}, {310, 400}, {316, 430}, {328, 452},
  {305, 456}, {280, 455}, {255, 456}, {230, 456}
}
local m_trunk = poly(pts_trunk, true)
work(m_trunk, {
  hand = "body",
  pile = p_bark_solid,
  angle = 1.57,
  length = {40, 80},
  coverage = 2.5,
  fill = true,
  clip = true
})

-- Longitudinal bark furrows and ridges
local b_det = brush{kind="round", width=1.8, point=1, stiffness=0.85}
b_det:load(p_hollow_void, 0.9)
b_det:stroke({{248, 450}, {256, 430}, {260, 405}, {262, 370}}, {pressure={0.8, 0.6, 0.3}})
b_det:stroke({{268, 452}, {271, 432}, {272, 405}, {270, 368}}, {pressure={0.8, 0.7, 0.3}})
b_det:stroke({{292, 448}, {290, 428}, {286, 400}, {282, 368}}, {pressure={0.8, 0.6, 0.3}})
b_det:stroke({{308, 446}, {302, 430}, {296, 408}, {290, 378}}, {pressure={0.8, 0.5, 0.3}})

b_det:load(p_bark_ridge, 0.85)
b_det:stroke({{240, 454}, {250, 440}, {255, 418}, {258, 385}}, {pressure={0.3, 0.75, 0.6, 0.2}})
b_det:stroke({{262, 454}, {266, 435}, {268, 410}, {268, 375}}, {pressure={0.3, 0.7, 0.6, 0.2}})
b_det:stroke({{278, 454}, {278, 435}, {278, 405}, {276, 370}}, {pressure={0.3, 0.65, 0.6, 0.2}})
b_det:stroke({{300, 450}, {296, 432}, {290, 408}, {284, 375}}, {pressure={0.3, 0.7, 0.6, 0.2}})

-- Weathered rot hollow and wound-wood callus
local m_void = poly({{272, 430}, {284, 426}, {286, 402}, {278, 396}, {270, 415}}, true)
work(m_void, {hand="detail", pile=p_hollow_void, angle=1.5, coverage=1.8, fill=true, clip=true})
b_det:load(p_woundwood, 0.85)
b_det:stroke({{269, 432}, {270, 414}, {277, 395}}, {pressure={0.5, 0.85, 0.4}})
b_det:stroke({{285, 428}, {286, 403}, {279, 395}}, {pressure={0.5, 0.85, 0.4}})

-- 3. Heavy, Muscular Bough Ribbons (True volume and anatomical taper)
-- Eastern living bough main trunk
local pts_east_stem = {{284, 365}, {330, 355}, {380, 342}, {430, 330}, {480, 315}, {530, 295}}
local w_east_stem   = {22, 18, 14, 11, 8, 5}
local m_east_stem   = ribbon(pts_east_stem, w_east_stem)
work(m_east_stem, {hand="body", pile=p_bark_solid, angle=0.2, coverage=2.2, fill=true, clip=true})

-- Eastern upward bough
local pts_e1 = {{370, 344}, {388, 316}, {408, 286}, {426, 256}, {440, 228}}
local w_e1   = {13, 11, 9, 6.5, 4.5}
local m_e1   = ribbon(pts_e1, w_e1)
work(m_e1, {hand="body", pile=p_bark_solid, angle=-0.5, coverage=2.0, fill=true, clip=true})

-- Eastern sheltering bough over dolmen
local pts_e2 = {{425, 332}, {448, 350}, {474, 364}, {500, 374}}
local w_e2   = {10, 8, 6.5, 4.5}
local m_e2   = ribbon(pts_e2, w_e2)
work(m_e2, {hand="body", pile=p_bark_solid, angle=0.4, coverage=2.0, fill=true, clip=true})

-- Western living bough main trunk
local pts_west_stem = {{265, 360}, {225, 345}, {185, 328}, {145, 310}, {95, 292}}
local w_west_stem   = {18, 15, 12, 9, 5}
local m_west_stem   = ribbon(pts_west_stem, w_west_stem)
work(m_west_stem, {hand="body", pile=p_bark_solid, angle=-0.2, coverage=2.2, fill=true, clip=true})

-- Western upward bough
local pts_w1 = {{210, 335}, {194, 305}, {178, 275}, {165, 245}, {148, 218}}
local w_w1   = {12, 10, 8, 6, 4}
local m_w1   = ribbon(pts_w1, w_w1)
work(m_w1, {hand="body", pile=p_bark_solid, angle=-0.8, coverage=2.0, fill=true, clip=true})

-- Western drooping bough
local pts_w2 = {{175, 320}, {155, 342}, {135, 362}, {110, 380}}
local w_w2   = {10, 8, 6.5, 4.5}
local m_w2   = ribbon(pts_w2, w_w2)
work(m_w2, {hand="body", pile=p_bark_solid, angle=0.6, coverage=2.0, fill=true, clip=true})

-- 4. Rich, Textured Autumn Foliage Masses along the Living Boughs
local m_fol_e = (ellipse(380, 325, 45, 32) + ellipse(430, 275, 42, 30) +
                 ellipse(475, 305, 40, 28) + ellipse(525, 285, 35, 25) +
                 ellipse(470, 360, 38, 26) + ellipse(550, 315, 28, 22)):soften(8)

local m_fol_w = (ellipse(210, 310, 42, 30) + ellipse(170, 260, 40, 28) +
                 ellipse(140, 320, 38, 26) + ellipse(95, 290, 32, 22) +
                 ellipse(125, 370, 30, 22)):soften(8)

local m_fol_all = m_fol_e + m_fol_w

-- Layer A: Deep shadow interior foliage
stipple(m_fol_all, {
  pile = p_fol_deep,
  width = 2.4,
  coverage = 1.4,
  cluster = {0.85, 9},
  drag = {2.2, 1.3},
  feather = 0.5
})

-- Layer B: Warm russet body foliage
stipple(m_fol_all, {
  pile = p_fol_russet,
  width = 2.0,
  coverage = 1.2,
  cluster = {0.85, 8},
  drag = {2.0, 1.2},
  feather = 0.6
})

-- Layer C: Twilight-lit golden-amber leaf edges catching the evening glow
stipple(m_fol_all, {
  pile = p_fol_gold,
  width = 1.6,
  coverage = 0.8,
  cluster = {0.9, 6},
  drag = {1.6, 1.1},
  feather = 0.65
})

print("Ancient Oak solid trunk, muscular bough ribbons, and rich autumn foliage masses painted.")

--@ chunk 69
-- Test outline and body_of capabilities
local o = outline{
  {100, 100}, {150, 80}, {200, 100}, {180, 150}, {120, 140},
  char = "soft",
  closed = true
}
print("Outline mask area:", o:mask():area())

local b = body_of{
  spine = {{200, 400}, {200, 300}, {200, 200}},
  widths = {30, 20, 10},
  limbs = {
    {pts = {{200, 300}, {260, 260}}, widths = {12, 6}}
  },
  blend = 0.8
}
print("body_of mask area:", b:mask():area())

--@ chunk 70
local t = wait(3 * 24 * 60)
print("Time after drying:", t)
print("Drying state at (300, 300):", drying(300, 300))
print("Drying state at sky (500, 200):", drying(500, 200))

--@ chunk 71
-- Chunk 71: Pass 1 — Master Friedrichian Sky, Distant Coast, Horizon Mist, Moon & Star

-- 1. Four overlapping sky bands from zenith (y = 0) down behind the knoll (y = 430)
local p_sky_zenith = pile{{"lead white", 18}, {"cobalt blue", 6.2}, {"smalt", 4.5}, {"raw umber", 0.6}, {"bone black", 0.25}, medium=0.22}
local p_sky_upper  = pile{{"lead white", 26}, {"cobalt blue", 3.2}, {"smalt", 3.2}, {"yellow ochre", 0.8}, {"raw umber", 0.3}, medium=0.22}
local p_sky_mid    = pile{{"lead white", 28}, {"yellow ochre", 3.5}, {"chrome yellow", 0.9}, {"smalt", 1.0}, {"raw umber", 0.2}, {"vermilion", 0.1}, medium=0.22}
local p_sky_glow   = pile{{"lead white", 26}, {"yellow ochre", 5.2}, {"chrome yellow", 2.2}, {"vermilion", 0.5}, {"raw umber", 0.25}, medium=0.22}

-- Band 1: Zenith
local m1 = rect(0, 0, 1000, 145):soften(30)
work(m1, {hand="body", pile=p_sky_zenith, angle=0, order="across", coverage=2.0, fill=true})

-- Band 2: Upper mid-sky
local m2 = rect(0, 105, 1000, 150):soften(30)
work(m2, {hand="body", pile=p_sky_upper, angle=0, order="across", coverage=2.0, fill=true})

-- Band 3: Transition
local m3 = rect(0, 215, 1000, 140):soften(30)
work(m3, {hand="body", pile=p_sky_mid, angle=0, order="across", coverage=2.0, fill=true})

-- Band 4: Horizon glow (down to y = 430 behind barrow crest)
local m4 = rect(0, 315, 1000, 125):soften(25)
work(m4, {hand="body", pile=p_sky_glow, angle=0, order="across", coverage=2.0, fill=true})

-- 2. Fuse the four wet bands into a seamless atmospheric dome
local m_sky_all = rect(0, 0, 1000, 430)
blend(m_sky_all, {angle=1.57})
blend(m_sky_all, {angle=0})

-- 3. Distant low Baltic hills silhouetted in soft atmospheric haze
local p_dist_coast = pile{{"lead white", 15}, {"smalt", 5.0}, {"raw umber", 3.0}, {"red earth", 0.9}, {"bone black", 0.4}, medium=0.25}
local pts_distant_land = {
  {0, 464}, {100, 460}, {220, 454}, {340, 452}, {460, 448},
  {580, 451}, {700, 447}, {820, 452}, {920, 458}, {1000, 464},
  {1000, 475}, {0, 475}
}
local m_coast = poly(pts_distant_land, true):soften(2.5)
work(m_coast, {hand="body", pile=p_dist_coast, angle=0, coverage=1.2, fill=true, clip=true})

-- 4. Calm water sheen and valley mist
local p_water_mist = pile{{"lead white", 26}, {"yellow ochre", 3.2}, {"smalt", 1.8}, {"raw umber", 0.3}, medium=0.3}
local m_water = rect(0, 438, 1000, 28):soften(4)
work(m_water, {hand="glaze", pile=p_water_mist, angle=0, coverage=1.1, fill=false})
blend(rect(0, 435, 1000, 32), {angle=0})

-- 5. Waxing crescent moon and solitary evening star
local p_moon_pure = pile{{"lead white", 34}, {"yellow ochre", 1.0}, {"chrome yellow", 0.4}, medium=0.12}
local b_rigger = brush{kind="round", width=1.3, point=1, stiffness=0.85}
b_rigger:load(p_moon_pure, 0.95)
b_rigger:stroke({
  {645.2, 105.5}, {641.5, 108.0}, {639.0, 112.5}, {640.2, 117.0}, {644.0, 119.5}
}, {pressure={0.08, 0.85, 1.0, 0.8, 0.08}, ramps={0.05, 0.05}})

local b_star = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_star:load(p_moon_pure, 1.0)
b_star:touch(688, 148, {pressure=0.6})

print("Pass 1 complete: Master sky, distant coast, water sheen, mist, moon, and star established.")

--@ chunk 72
-- Chunk 72: Step 2 & 3 — Sculpting the Grand Barrow Mound and the Monumental Megalithic Tomb

-- 1. Mathematical spline for the swelling barrow mound crest
local barrow_nodes = {
  {x=-20, y=475}, {x=80, y=468}, {x=180, y=456}, {x=270, y=438},
  {x=340, y=422}, {x=390, y=416}, {x=450, y=422}, {x=520, y=438},
  {x=600, y=454}, {x=680, y=456}, {x=760, y=442}, {x=830, y=442},
  {x=910, y=455}, {x=1020, y=472}
}

local function barrow_crest(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 450
end

local m_knoll = below(barrow_crest)

-- 2. Earth, Peat, and Heathland Palette
local p_earth_core   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4}, {"yellow ochre", 3}, medium=0.18}
local p_heath_dark   = pile{{"raw umber", 8}, {"bone black", 5}, {"red earth", 3.5}, {"green earth", 3.5}, medium=0.2}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5}, {"lead white", 2}, {"bone black", 1}, medium=0.2}
local p_peat_moss    = pile{{"green earth", 6}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}
local p_heath_gold   = pile{{"yellow ochre", 8}, {"raw umber", 4}, {"lead white", 3.5}, {"red earth", 2}, medium=0.2}

-- Solid body pass across the swelling barrow
work(m_knoll, {
  hand = "body",
  pile = p_earth_core,
  angle = 0.22,
  length = {45, 95},
  coverage = 2.2,
  fill = true,
  clip = true
})

-- Swelling heather banks
local m_summit_bank = (ellipse(390, 445, 160, 32) + ellipse(260, 460, 140, 35)):soften(15) * m_knoll
work(m_summit_bank, {hand="body", pile=p_heath_russet, angle=0.18, coverage=1.4, fill=true})

local m_rknoll_bank = ellipse(770, 465, 160, 35):soften(15) * m_knoll
work(m_rknoll_bank, {hand="body", pile=p_heath_russet, angle=-0.15, coverage=1.3, fill=true})

local m_hollow = (ellipse(520, 525, 200, 40) + ellipse(180, 535, 150, 40)):soften(18) * m_knoll
work(m_hollow, {hand="body", pile=p_peat_moss, angle=0.25, coverage=1.2, fill=true})

local m_low_heather = (ellipse(350, 600, 260, 50) + ellipse(780, 590, 220, 45)):soften(20) * m_knoll
work(m_low_heather, {hand="body", pile=p_heath_gold, angle=0.2, coverage=1.3, fill=true})

-- Blend the knoll interior to fuse the earth layers
blend(m_knoll, {angle=0.2})
blend(m_knoll, {angle=-0.15})

-- Firm, smooth stroke along the crest curve
local b_crest = brush{kind="round", width=2.6, point=1, stiffness=0.9}
b_crest:load(p_earth_core, 0.95)
local crest_line = {}
for x = -10, 1010, 10 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest:stroke(crest_line, {pressure=0.9, ramps={0.01, 0.01}})

-- 3. The Megalithic Tomb (Hünengrab) atop the Barrow Summit
-- Piles for Swedish Red-Gray Glacial Granite
local p_cavern_void   = pile{{"bone black", 12}, {"raw umber", 8}, medium=0.1}
local p_granite_deep  = pile{{"bone black", 8}, {"raw umber", 7}, {"smalt", 2.0}, {"red earth", 1.5}, medium=0.15}
local p_granite_shade = pile{{"raw umber", 8}, {"bone black", 5}, {"smalt", 2.2}, {"lead white", 2.5}, {"red earth", 1.2}, {"yellow ochre", 1.5}, medium=0.18}
local p_granite_body  = pile{{"lead white", 10}, {"raw umber", 6}, {"bone black", 3}, {"yellow ochre", 3.2}, {"smalt", 1.8}, {"red earth", 1.2}, medium=0.2}
local p_granite_facet = pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.22}
local p_granite_light = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage     = pile{{"lead white", 16}, {"green earth", 6.5}, {"yellow ochre", 3.5}, {"raw umber", 1.0}, medium=0.25}
local p_lich_gold     = pile{{"lead white", 12}, {"yellow ochre", 9.0}, {"chrome yellow", 1.8}, {"raw umber", 0.8}, medium=0.25}

local b_block = brush{kind="filbert", width=5.5, stiffness=0.85}
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_crack = brush{kind="round", width=0.9, point=1, stiffness=0.9}

-- A. Sacred Chamber Void
b_block:load(p_cavern_void, 0.95)
b_block:stroke({{344, 414}, {385, 414}, {425, 414}, {452, 415}}, {pressure=0.9})
b_block:stroke({{348, 424}, {395, 426}, {448, 425}}, {pressure=0.9})
b_block:stroke({{352, 434}, {400, 436}, {444, 435}}, {pressure=0.9})

-- B. Left Orthostat
b_block:load(p_granite_shade, 0.9)
b_block:stroke({{338, 412}, {342, 426}, {346, 442}}, {pressure=0.85})
b_block:stroke({{348, 412}, {352, 426}, {356, 442}}, {pressure=0.85})
b_facet:load(p_granite_facet, 0.8)
b_facet:stroke({{338, 412}, {340, 424}, {344, 435}, {346, 444}}, {pressure={0.7, 0.5, 0.6, 0.2}})

-- C. Rear Orthostat
b_facet:load(p_granite_deep, 0.85)
b_facet:stroke({{394, 414}, {396, 428}, {398, 438}}, {pressure=0.75})
b_facet:stroke({{406, 414}, {408, 428}, {410, 438}}, {pressure=0.75})

-- D. Right Orthostat
b_block:load(p_granite_shade, 0.9)
b_block:stroke({{428, 413}, {432, 426}, {434, 442}}, {pressure=0.85})
b_block:stroke({{438, 413}, {442, 426}, {445, 442}}, {pressure=0.85})
b_block:stroke({{448, 412}, {452, 425}, {455, 442}}, {pressure=0.85})
b_facet:load(p_granite_facet, 0.85)
b_facet:stroke({{448, 412}, {452, 422}, {456, 433}, {458, 442}}, {pressure={0.75, 0.8, 0.6, 0.2}})
b_facet:load(p_granite_light, 0.8)
b_facet:stroke({{450, 412}, {454, 418}, {452, 426}}, {pressure={0.65, 0.5, 0.2}})

-- E. Massive Granite Capstone
-- Deep underside cast shadow
b_block:load(p_granite_deep, 0.95)
b_block:stroke({{316, 411}, {355, 412}, {400, 413}, {445, 412}, {474, 410}}, {pressure=0.9})
b_crack:load(p_cavern_void, 0.95)
b_crack:stroke({{314, 412}, {350, 414}, {395, 415}, {440, 414}, {476, 411}}, {pressure=0.85})

-- Front face (crystalline texture)
b_block:load(p_granite_body, 0.9)
b_block:stroke({{318, 408}, {350, 409}, {390, 410}, {430, 409}, {468, 407}}, {pressure=0.85})
b_block:stroke({{324, 402}, {360, 403}, {405, 404}, {445, 403}, {470, 401}}, {pressure=0.85})

-- Top sky-facing facets (sharp, angular crystalline planes)
local pts_fA = {{314, 409}, {322, 400}, {340, 392}, {368, 388}, {394, 387}, {392, 400}, {365, 402}, {338, 405}, {318, 409}}
local m_fA = poly(pts_fA, false)
work(m_fA, {hand="detail", pile=p_granite_light, angle=0.04, coverage=1.6, fill=true, clip=true})

local pts_fB = {{394, 387}, {425, 389}, {455, 392}, {472, 398}, {476, 406}, {466, 407}, {440, 404}, {412, 402}, {392, 400}}
local m_fB = poly(pts_fB, false)
work(m_fB, {hand="detail", pile=p_granite_facet, angle=-0.04, coverage=1.6, fill=true, clip=true})

-- Fracture cleavage cracks
b_crack:load(p_granite_deep, 0.9)
b_crack:stroke({{394, 387}, {392, 400}, {390, 414}}, {pressure={0.7, 0.85, 0.3}})
b_crack:stroke({{348, 391}, {352, 402}, {350, 412}}, {pressure={0.45, 0.6, 0.2}})
b_crack:stroke({{442, 391}, {444, 401}, {446, 412}}, {pressure={0.5, 0.65, 0.2}})

-- Top edge crystalline highlights
b_crack:load(p_granite_light, 0.95)
b_crack:stroke({{314, 409}, {322, 400}, {340, 392}, {368, 388}, {394, 387}}, {pressure={0.3, 0.8, 0.7, 0.85, 0.4}})
b_crack:stroke({{396, 388}, {425, 389}, {455, 392}, {472, 398}}, {pressure={0.4, 0.8, 0.75, 0.3}})

-- Lichen touches
local lichen_patches = {
  {335, 396, p_lich_sage, 2.2},
  {362, 392, p_lich_gold, 2.5},
  {380, 398, p_lich_sage, 1.8},
  {415, 394, p_lich_gold, 2.4},
  {438, 396, p_lich_sage, 2.0},
  {458, 400, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichen_patches) do
  b_facet:load(lp[3], 0.7)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- F. Flanking Erratics
local pts_el = {{294, 442}, {302, 426}, {318, 420}, {328, 428}, {324, 444}, {298, 444}}
local m_el = poly(pts_el, false)
work(m_el, {hand="detail", pile=p_granite_body, angle=0.3, coverage=1.6, fill=true, clip=true})
local pts_el_lit = {{302, 426}, {318, 420}, {328, 428}, {316, 432}, {306, 431}}
local m_el_lit = poly(pts_el_lit, false)
work(m_el_lit, {hand="detail", pile=p_granite_facet, angle=0.15, coverage=1.4, fill=true, clip=true})

local pts_er = {{470, 442}, {478, 424}, {504, 420}, {516, 432}, {508, 444}, {474, 444}}
local m_er = poly(pts_er, false)
work(m_er, {hand="detail", pile=p_granite_body, angle=-0.2, coverage=1.6, fill=true, clip=true})
local pts_er_lit = {{478, 424}, {504, 420}, {516, 432}, {500, 431}, {484, 429}}
local m_er_lit = poly(pts_er_lit, false)
work(m_er_lit, {hand="detail", pile=p_granite_facet, angle=-0.1, coverage=1.4, fill=true, clip=true})

-- G. Anchor all stone bases into turf with dark peat
b_block:load(p_earth_core, 0.95)
b_block:stroke({{285, 444}, {330, 444}, {370, 444}}, {pressure=0.9})
b_block:stroke({{420, 444}, {470, 444}, {525, 444}}, {pressure=0.9})

print("Pass 2 complete: Barrow mound and Megalithic Tomb sculpted.")

--@ chunk 73
-- Chunk 73: The Veteran Oak Anatomy — Continuous Anatomical Skeleton via body_of

-- 1. Wood and Bark Piles
p_bark_solid   = pile{{"bone black", 12}, {"raw umber", 8}, {"red earth", 2.0}, medium=0.15}
p_bark_ridge   = pile{{"raw umber", 6}, {"yellow ochre", 6.5}, {"lead white", 4.5}, {"bone black", 1.5}, medium=0.2}
p_hollow_void  = pile{{"bone black", 14}, {"raw umber", 6}, medium=0.1}
p_woundwood    = pile{{"raw umber", 6}, {"yellow ochre", 5.5}, {"lead white", 3.5}, {"red earth", 2.0}, medium=0.2}
p_stag_wood    = pile{{"lead white", 16}, {"raw umber", 4.0}, {"yellow ochre", 2.5}, {"smalt", 1.0}, {"bone black", 0.5}, medium=0.22}
p_stag_shade   = pile{{"raw umber", 8}, {"bone black", 5}, {"lead white", 4}, {"yellow ochre", 2}, medium=0.2}

-- 2. Build the solid anatomical skeleton of the Veteran Oak
local tree_geom = body_of{
  spine = {
    {274, 454}, {276, 420}, {278, 380}, {274, 345}, {270, 305},
    {264, 250}, {258, 185}, {252, 125}, {246, 75}
  },
  widths = {65, 48, 38, 30, 22, 16, 11, 7, 3},
  limbs = {
    -- Eastern bough main stem (sweeping over dolmen)
    {
      pts = {{278, 380}, {325, 368}, {375, 355}, {425, 342}, {475, 325}, {520, 305}},
      widths = {22, 18, 14, 11, 7, 4}
    },
    -- Eastern upward bough
    {
      pts = {{375, 355}, {395, 325}, {418, 290}, {435, 252}},
      widths = {12, 9, 6.5, 3.5}
    },
    -- Eastern sheltering bough over dolmen
    {
      pts = {{425, 342}, {450, 360}, {476, 372}, {500, 380}},
      widths = {10, 7.5, 5, 3}
    },
    -- Western bough main stem
    {
      pts = {{276, 375}, {235, 358}, {195, 340}, {155, 322}, {110, 302}},
      widths = {18, 14, 11, 7.5, 3.5}
    },
    -- Western upward bough
    {
      pts = {{215, 348}, {198, 315}, {180, 280}, {160, 240}},
      widths = {11, 8.5, 6, 3.5}
    },
    -- Western drooping bough
    {
      pts = {{180, 332}, {160, 355}, {135, 372}},
      widths = {9, 6.5, 3.5}
    },
    -- Eastern dead stag antler
    {
      pts = {{270, 305}, {290, 255}, {315, 200}, {332, 145}, {344, 95}},
      widths = {13, 10, 7.5, 5, 2.5}
    },
    -- Western dead stag antler
    {
      pts = {{268, 310}, {246, 260}, {224, 205}, {205, 150}, {190, 100}},
      widths = {12, 9.5, 7, 4.5, 2.5}
    }
  },
  blend = 0.85
}

local m_oak_skeleton = tree_geom:mask()

-- Flaring root buttresses gripping the barrow
local m_roots = ribbon({{230, 456}, {248, 448}, {266, 435}}, {16, 12, 8}) +
                ribbon({{322, 452}, {306, 442}, {288, 425}}, {18, 14, 9})

local m_oak_full = m_oak_skeleton + m_roots

-- Fill the entire oak skeleton with rich, solid, opaque body paint
work(m_oak_full, {
  hand = "body",
  pile = p_bark_solid,
  angle = 1.57,
  length = {35, 75},
  coverage = 2.4,
  fill = true,
  clip = true
})

-- 3. Model the Bleached Stag-Head Crown aloft (dead wood tones on the upper spires)
local m_stag_crown = rect(150, 60, 230, 240) * m_oak_full
work(m_stag_crown, {
  hand = "body",
  pile = p_stag_shade,
  angle = 1.57,
  coverage = 1.8,
  fill = true,
  clip = true
})

-- Bleached wood highlights on the sky-facing surfaces of dead stag boughs
local b_det = brush{kind="round", width=1.5, point=1, stiffness=0.85}
b_det:load(p_stag_wood, 0.9)
b_det:stroke({{266, 275}, {262, 225}, {256, 175}, {250, 120}, {246, 75}}, {pressure={0.4, 0.7, 0.6, 0.4, 0.1}})
b_det:stroke({{274, 260}, {292, 215}, {314, 168}, {330, 122}, {344, 95}}, {pressure={0.35, 0.65, 0.55, 0.35, 0.1}})
b_det:stroke({{264, 265}, {244, 220}, {224, 172}, {206, 125}, {190, 100}}, {pressure={0.35, 0.65, 0.55, 0.35, 0.1}})

-- Splintered tips on dead antlers
b_det:stroke({{246, 78}, {244, 70}}, {pressure={0.4, 0.05}})
b_det:stroke({{246, 78}, {250, 72}}, {pressure={0.3, 0.05}})
b_det:stroke({{344, 97}, {348, 90}}, {pressure={0.35, 0.05}})
b_det:stroke({{190, 102}, {186, 94}}, {pressure={0.35, 0.05}})

-- 4. Longitudinal Bark Furrows, Ridges, and Hollow on the Living Trunk
b_det:load(p_hollow_void, 0.9)
b_det:stroke({{248, 452}, {256, 432}, {262, 408}, {264, 375}}, {pressure={0.8, 0.65, 0.3}})
b_det:stroke({{268, 454}, {271, 434}, {273, 406}, {270, 370}}, {pressure={0.8, 0.7, 0.4}})
b_det:stroke({{294, 450}, {292, 430}, {286, 402}, {280, 370}}, {pressure={0.8, 0.65, 0.35}})
b_det:stroke({{310, 448}, {304, 430}, {296, 408}, {288, 378}}, {pressure={0.8, 0.55, 0.3}})

b_det:load(p_bark_ridge, 0.85)
b_det:stroke({{240, 454}, {250, 440}, {256, 418}, {260, 385}}, {pressure={0.3, 0.75, 0.6, 0.2}})
b_det:stroke({{262, 454}, {266, 436}, {268, 410}, {268, 375}}, {pressure={0.3, 0.7, 0.6, 0.2}})
b_det:stroke({{278, 454}, {278, 436}, {278, 405}, {275, 370}}, {pressure={0.3, 0.65, 0.6, 0.2}})
b_det:stroke({{302, 450}, {298, 432}, {290, 408}, {284, 375}}, {pressure={0.3, 0.7, 0.6, 0.2}})

-- Weathered rot hollow
local m_void = poly({{272, 430}, {284, 426}, {286, 402}, {278, 396}, {270, 415}}, true)
work(m_void, {hand="detail", pile=p_hollow_void, angle=1.5, coverage=1.8, fill=true, clip=true})
b_det:load(p_woundwood, 0.85)
b_det:stroke({{269, 432}, {270, 414}, {277, 395}}, {pressure={0.5, 0.85, 0.4}})
b_det:stroke({{285, 428}, {286, 403}, {279, 395}}, {pressure={0.5, 0.85, 0.4}})

print("Veteran Oak anatomical skeleton modeled with body_of.")

--@ chunk 74
-- Chunk 74: Intricate Oak Twigs, Clinging Marcescent Leaves, and the Wind-Sculpted Companion Tree

-- 1. Piles for Fine Wood, Bleached Antlers, and Marcescent Leaves
local p_wood_core   = pile{{"bone black", 12}, {"raw umber", 8}, {"red earth", 2.0}, medium=0.15}
local p_stag_fine   = pile{{"lead white", 18}, {"raw umber", 4.0}, {"yellow ochre", 2.5}, {"smalt", 1.0}, medium=0.2}
local p_leaf_dark   = pile{{"bone black", 8}, {"raw umber", 9}, {"red earth", 4.0}, medium=0.2}
local p_leaf_russet = pile{{"yellow ochre", 7}, {"red earth", 6.5}, {"raw umber", 4.0}, {"lead white", 1.5}, medium=0.22}
local p_leaf_gold   = pile{{"yellow ochre", 9}, {"chrome yellow", 2.2}, {"lead white", 4.5}, {"red earth", 1.5}, {"vermilion", 0.3}, medium=0.25}

local b_twig   = brush{kind="round", width=1.5, point=1, stiffness=0.85}
local b_rigger = brush{kind="round", width=0.9, point=1, stiffness=0.9}
local b_leaf   = brush{kind="round", width=1.6, point=1, stiffness=0.8}

-- 2. Intricate Crooked Sympodial Twigs on the Veteran Oak
-- Eastern living bough twigs (weaving above and around the dolmen)
local east_twigs = {
  -- Off E1 (upward bough)
  {{435, 252}, {448, 230}, {460, 210}},
  {{435, 252}, {425, 232}, {416, 215}},
  {{418, 290}, {430, 275}, {442, 262}},
  {{395, 325}, {385, 305}, {378, 285}},
  {{395, 325}, {408, 308}, {418, 292}},
  -- Off outer eastern bough
  {{520, 305}, {542, 288}, {562, 275}},
  {{520, 305}, {532, 322}, {545, 340}},
  {{475, 325}, {490, 310}, {505, 295}},
  {{475, 325}, {465, 305}, {456, 288}},
  -- Off E2 (sheltering bough over dolmen)
  {{500, 380}, {515, 395}, {528, 410}},
  {{476, 372}, {488, 388}, {496, 404}},
  {{450, 360}, {440, 375}, {432, 390}},
  {{375, 355}, {362, 370}, {350, 385}},
  {{325, 368}, {338, 382}, {348, 395}}
}
for _, tw in ipairs(east_twigs) do
  b_twig:load(p_wood_core, 0.85)
  b_twig:stroke(tw, {pressure={0.75, 0.2}, ramps={0.05, 0.2}})
end

-- Western living bough twigs
local west_twigs = {
  -- Off W1 (upward bough)
  {{160, 240}, {145, 220}, {130, 202}},
  {{160, 240}, {172, 222}, {182, 205}},
  {{180, 280}, {168, 260}, {158, 242}},
  {{198, 315}, {212, 295}, {222, 275}},
  {{198, 315}, {188, 298}, {178, 282}},
  -- Off outer western bough
  {{110, 302}, {90, 285}, {72, 270}},
  {{110, 302}, {98, 318}, {85, 335}},
  {{155, 322}, {140, 308}, {125, 295}},
  -- Off W2 (drooping bough)
  {{135, 372}, {118, 390}, {102, 408}},
  {{160, 355}, {148, 375}, {138, 395}},
  {{180, 332}, {192, 350}, {202, 368}}
}
for _, tw in ipairs(west_twigs) do
  b_twig:load(p_wood_core, 0.85)
  b_twig:stroke(tw, {pressure={0.75, 0.2}, ramps={0.05, 0.2}})
end

-- Bleached dead stag antler twigs spearing into the cobalt zenith
local stag_antler_twigs = {
  {{246, 75}, {242, 55}, {238, 38}},
  {{246, 75}, {252, 58}, {256, 42}},
  {{252, 125}, {262, 102}, {270, 80}},
  {{258, 185}, {248, 160}, {240, 138}},
  {{344, 95}, {352, 75}, {358, 58}},
  {{344, 95}, {336, 78}, {330, 62}},
  {{332, 145}, {345, 120}, {355, 98}},
  {{315, 200}, {302, 172}, {292, 148}},
  {{190, 100}, {182, 80}, {175, 62}},
  {{190, 100}, {198, 82}, {204, 66}},
  {{205, 150}, {192, 125}, {182, 102}},
  {{224, 205}, {212, 178}, {202, 152}}
}
for _, st in ipairs(stag_antler_twigs) do
  b_rigger:load(p_stag_fine, 0.9)
  b_rigger:stroke(st, {pressure={0.65, 0.15}, ramps={0.05, 0.2}})
end

-- 3. Clinging Marcescent Autumn Leaves (small, delicate sprays along living twigs)
local leaf_clusters = {
  -- Eastern living bough leaf sprays
  {440, 245, 14}, {455, 225, 12}, {425, 225, 12}, {412, 280, 15},
  {485, 315, 16}, {515, 298, 15}, {540, 285, 12}, {530, 325, 12},
  {470, 365, 15}, {490, 380, 14}, {512, 400, 12}, {445, 365, 12},
  {380, 335, 16}, {395, 310, 14}, {355, 365, 12},
  -- Western living bough leaf sprays
  {155, 235, 14}, {140, 215, 12}, {170, 218, 12}, {175, 275, 15},
  {105, 295, 15}, {85, 280, 12}, {92, 320, 12}, {145, 315, 14},
  {130, 365, 14}, {112, 385, 12}, {150, 360, 12}, {195, 325, 15}
}

for _, cl in ipairs(leaf_clusters) do
  local cx, cy, n = cl[1], cl[2], cl[3]
  for i = 1, n do
    local ox = rand(-12, 12)
    local oy = rand(-10, 10)
    local roll = rand(0, 1)
    local p = (roll < 0.35) and p_leaf_dark or ((roll < 0.75) and p_leaf_russet or p_leaf_gold)
    b_leaf:load(p, 0.7)
    b_leaf:touch(cx + ox, cy + oy, {pressure=rand(0.35, 0.7), drag={rand(1.2, 2.4), rand(1.1, 1.7)}})
  end
end

-- 4. The Companion Tree on the Right Knoll (x ≈ 750 to 820)
-- A graceful, wind-beaten tree bowed toward the east by Baltic gales
local comp_geom = body_of{
  spine = {
    {768, 444}, {766, 415}, {762, 385}, {756, 355}, {750, 325},
    {746, 290}, {744, 255}, {745, 220}, {750, 185}
  },
  widths = {24, 18, 14, 11, 8.5, 6.5, 4.5, 3.0, 1.5},
  limbs = {
    -- Eastern spreading limb (bowed by wind)
    {
      pts = {{756, 355}, {778, 340}, {805, 322}, {832, 305}, {855, 290}},
      widths = {9, 7.5, 5.5, 3.8, 2.0}
    },
    -- Eastern upward branch
    {
      pts = {{805, 322}, {818, 300}, {828, 275}, {836, 250}},
      widths = {5.5, 4.0, 2.8, 1.5}
    },
    -- Western wind-stunted branch
    {
      pts = {{762, 385}, {745, 372}, {728, 360}, {712, 350}},
      widths = {7.5, 5.5, 3.5, 1.8}
    },
    -- Upper crown branches bowing east
    {
      pts = {{746, 290}, {762, 270}, {776, 248}, {786, 225}},
      widths = {5.5, 4.0, 2.8, 1.5}
    },
    {
      pts = {{744, 255}, {732, 235}, {724, 215}},
      widths = {4.0, 2.8, 1.5}
    }
  },
  blend = 0.8
}

local m_comp = comp_geom:mask()
work(m_comp, {
  hand = "body",
  pile = p_wood_core,
  angle = 1.4,
  length = {25, 55},
  coverage = 2.0,
  fill = true,
  clip = true
})

-- Companion tree fine windswept twigs
local comp_twigs = {
  {{855, 290}, {872, 280}, {888, 272}},
  {{855, 290}, {866, 302}, {876, 315}},
  {{836, 250}, {845, 232}, {852, 215}},
  {{786, 225}, {798, 208}, {808, 192}},
  {{750, 185}, {760, 165}, {768, 148}},
  {{750, 185}, {742, 168}, {736, 152}},
  {{712, 350}, {700, 342}, {688, 336}}
}
for _, tw in ipairs(comp_twigs) do
  b_rigger:load(p_wood_core, 0.85)
  b_rigger:stroke(tw, {pressure={0.65, 0.15}, ramps={0.05, 0.2}})
end

-- A few sparse autumn leaves on the companion tree
local comp_leaf_spots = {
  {785, 335, 8}, {820, 310, 10}, {845, 295, 8}, {825, 270, 8},
  {775, 255, 8}, {790, 225, 8}, {730, 360, 6}
}
for _, cl in ipairs(comp_leaf_spots) do
  local cx, cy, n = cl[1], cl[2], cl[3]
  for i = 1, n do
    local ox = rand(-10, 10)
    local oy = rand(-8, 8)
    local p = (rand(0, 1) < 0.5) and p_leaf_russet or p_leaf_gold
    b_leaf:load(p, 0.65)
    b_leaf:touch(cx + ox, cy + oy, {pressure=rand(0.3, 0.6), drag={rand(1.0, 2.0), 1.4}})
  end
end

print("Pass 3 complete: Intricate oak twigs, marcescent leaves, and windswept companion tree painted.")

--@ chunk 75
-- Chunk 75: Grounding the Oak, Smoothing Limbs, Building the Intricate Branching Canopy & Companion Tree

-- 1. Piles for Wood, Bark Ridges, and Marcescent Foliage
local p_bark_deep   = pile{{"bone black", 12}, {"raw umber", 8}, {"red earth", 2.0}, medium=0.15}
local p_bark_mid    = pile{{"raw umber", 8}, {"bone black", 5}, {"yellow ochre", 4.0}, {"lead white", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_bark_light  = pile{{"yellow ochre", 7}, {"lead white", 5}, {"raw umber", 4}, {"bone black", 1}, medium=0.2}
local p_stag_smooth = pile{{"lead white", 14}, {"raw umber", 5}, {"yellow ochre", 3}, {"smalt", 1}, {"bone black", 0.5}, medium=0.2}
local p_peat_earth  = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, {"green earth", 2.5}, medium=0.15}
local p_leaf_russet = pile{{"yellow ochre", 7}, {"red earth", 6.5}, {"raw umber", 4.0}, {"lead white", 1.5}, medium=0.22}
local p_leaf_gold   = pile{{"yellow ochre", 9}, {"chrome yellow", 2.5}, {"lead white", 4.0}, {"vermilion", 0.4}, medium=0.25}

local b_filb   = brush{kind="filbert", width=5.5, stiffness=0.85}
local b_branch = brush{kind="round", width=2.6, point=1, stiffness=0.85}
local b_twig   = brush{kind="round", width=1.4, point=1, stiffness=0.85}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}
local b_leaf   = brush{kind="round", width=1.8, point=1, stiffness=0.8}

-- 2. Ground the Oak Trunk: Bed the base firmly into the barrow slope with powerful root buttresses
-- Western flaring root claw
b_filb:load(p_bark_deep, 0.95)
b_filb:stroke({{215, 465}, {235, 456}, {255, 442}, {268, 415}}, {pressure={0.9, 0.9, 0.85, 0.7}})
-- Central anchoring root
b_filb:stroke({{265, 468}, {270, 452}, {274, 430}, {276, 400}}, {pressure={0.95, 0.9, 0.85, 0.7}})
-- Eastern root buttress gripping into the barrow near the dolmen
b_filb:stroke({{325, 460}, {310, 450}, {295, 432}, {284, 405}}, {pressure={0.95, 0.9, 0.85, 0.7}})

-- Dark peaty soil and heather wrapping around the root toes to anchor the tree
b_filb:load(p_peat_earth, 0.95)
b_filb:stroke({{205, 466}, {245, 462}, {285, 460}, {330, 458}}, {pressure=0.95})
b_filb:stroke({{220, 472}, {260, 470}, {300, 468}, {340, 464}}, {pressure=0.95})

-- 3. Smooth the Bumpy Trunk & Limbs with Continuous Longitudinal Strokes
-- Trunk longitudinal bark smoothing
b_filb:load(p_bark_deep, 0.9)
b_filb:stroke({{248, 445}, {254, 415}, {260, 380}, {264, 350}}, {pressure={0.85, 0.8, 0.75, 0.7}})
b_filb:stroke({{274, 448}, {275, 415}, {274, 380}, {270, 345}}, {pressure={0.85, 0.8, 0.75, 0.7}})
b_filb:stroke({{298, 445}, {294, 415}, {288, 380}, {280, 350}}, {pressure={0.85, 0.8, 0.75, 0.7}})

-- Smooth the Eastern Bough into a continuous muscular arch
b_branch:load(p_bark_deep, 0.9)
b_branch:stroke({
  {278, 375}, {325, 365}, {375, 352}, {425, 338}, {475, 322}, {525, 302}
}, {pressure={0.9, 0.85, 0.75, 0.65, 0.5, 0.3}, ramps={0.02, 0.05}})
-- Upper contour highlight on Eastern bough
b_twig:load(p_bark_mid, 0.8)
b_twig:stroke({
  {282, 368}, {328, 358}, {376, 346}, {426, 332}, {476, 316}, {522, 298}
}, {pressure={0.4, 0.7, 0.65, 0.5, 0.35, 0.15}})

-- Smooth the Western Bough
b_branch:load(p_bark_deep, 0.9)
b_branch:stroke({
  {274, 372}, {235, 355}, {195, 338}, {155, 320}, {110, 300}
}, {pressure={0.9, 0.8, 0.7, 0.55, 0.35, 0.2}, ramps={0.02, 0.05}})
b_twig:load(p_bark_mid, 0.8)
b_twig:stroke({
  {270, 366}, {232, 350}, {194, 332}, {154, 314}, {112, 296}
}, {pressure={0.4, 0.65, 0.6, 0.45, 0.25, 0.1}})

-- Smooth the Stag-head boughs with bleached wood
b_branch:load(p_stag_smooth, 0.85)
b_branch:stroke({
  {272, 340}, {266, 290}, {260, 235}, {255, 175}, {250, 115}, {246, 75}
}, {pressure={0.85, 0.75, 0.65, 0.5, 0.35, 0.15}, ramps={0.02, 0.05}})
b_branch:stroke({
  {270, 305}, {288, 255}, {312, 200}, {330, 145}, {344, 95}
}, {pressure={0.8, 0.65, 0.5, 0.35, 0.15}, ramps={0.02, 0.05}})
b_branch:stroke({
  {268, 310}, {246, 260}, {224, 205}, {205, 150}, {190, 100}
}, {pressure={0.8, 0.65, 0.5, 0.35, 0.15}, ramps={0.02, 0.05}})

-- 4. The Intricate, Webbed Canopy Architecture
-- Eastern Living Canopy: Natural forking branches that bridge out from the heavy boughs
local east_canopy_limbs = {
  -- Outer eastern crown (x ≈ 520 to 600)
  {{525, 302}, {548, 288}, {570, 275}, {595, 265}},
  {{548, 288}, {562, 305}, {578, 320}, {592, 332}},
  {{525, 302}, {538, 325}, {552, 345}, {565, 362}},
  -- Sheltering bough forks over dolmen
  {{500, 380}, {520, 396}, {542, 410}, {560, 420}},
  {{476, 372}, {492, 388}, {508, 402}},
  {{450, 360}, {462, 380}, {472, 398}},
  -- Upward eastern bough forks (x ≈ 420 to 500, y ≈ 200 to 280)
  {{435, 252}, {452, 228}, {470, 208}, {486, 192}},
  {{452, 228}, {442, 208}, {435, 190}},
  {{435, 252}, {422, 230}, {412, 212}, {404, 195}},
  {{418, 290}, {435, 272}, {452, 255}, {466, 240}},
  {{395, 325}, {382, 302}, {372, 282}, {365, 265}},
  {{395, 325}, {410, 308}, {424, 290}}
}
for _, b in ipairs(east_canopy_limbs) do
  b_branch:load(p_bark_deep, 0.85)
  b_branch:stroke(b, {pressure={0.75, 0.25}, ramps={0.02, 0.1}})
end

-- Western Living Canopy: Natural forking branches (x ≈ 50 to 220)
local west_canopy_limbs = {
  -- Outer western crown
  {{110, 300}, {88, 284}, {68, 270}, {50, 258}},
  {{88, 284}, {78, 302}, {68, 320}, {58, 335}},
  {{110, 300}, {100, 320}, {88, 340}, {76, 358}},
  -- Upward western forks
  {{160, 240}, {145, 218}, {130, 198}, {116, 180}},
  {{145, 218}, {158, 202}, {168, 188}},
  {{160, 240}, {175, 222}, {188, 205}, {198, 190}},
  {{180, 280}, {165, 260}, {152, 240}},
  {{198, 315}, {184, 295}, {172, 278}},
  -- Drooping western forks
  {{135, 372}, {118, 392}, {100, 410}},
  {{160, 355}, {145, 378}, {132, 398}}
}
for _, b in ipairs(west_canopy_limbs) do
  b_branch:load(p_bark_deep, 0.85)
  b_branch:stroke(b, {pressure={0.75, 0.25}, ramps={0.02, 0.1}})
end

-- Intricate sympodial twiglets weaving through the canopy
local fine_twigs = {
  -- East
  {{595, 265}, {610, 258}, {622, 252}},
  {{595, 265}, {602, 275}, {608, 288}},
  {{486, 192}, {498, 182}, {508, 174}},
  {{486, 192}, {478, 180}, {472, 170}},
  {{404, 195}, {396, 182}, {390, 170}},
  {{466, 240}, {478, 228}, {488, 218}},
  {{565, 362}, {576, 375}, {585, 388}},
  -- West
  {{50, 258}, {38, 250}, {28, 244}},
  {{50, 258}, {44, 268}, {40, 278}},
  {{116, 180}, {108, 168}, {102, 158}},
  {{116, 180}, {125, 168}, {132, 158}},
  {{198, 190}, {206, 178}, {212, 168}}
}
for _, tw in ipairs(fine_twigs) do
  b_twig:load(p_bark_deep, 0.8)
  b_twig:stroke(tw, {pressure={0.65, 0.15}, ramps={0.02, 0.15}})
end

-- 5. Clinging Marcescent Autumn Leaves (*Laub*) along the living twigs
local marcescent_sprays = {
  -- Eastern canopy leaf sprays
  {445, 235, 24}, {475, 210, 20}, {420, 220, 20}, {410, 275, 22},
  {460, 245, 24}, {490, 310, 25}, {525, 290, 25}, {555, 275, 22},
  {580, 270, 20}, {570, 315, 20}, {540, 335, 22}, {515, 385, 20},
  {545, 405, 18}, {475, 365, 22}, {495, 385, 20}, {385, 335, 24},
  {365, 365, 18}, {340, 380, 16},
  -- Western canopy leaf sprays
  {150, 225, 24}, {135, 205, 20}, {165, 208, 20}, {170, 265, 22},
  {105, 288, 25}, {85, 275, 22}, {65, 265, 20}, {75, 310, 20},
  {90, 330, 20}, {140, 310, 24}, {125, 360, 20}, {110, 380, 18},
  {150, 350, 20}, {190, 320, 22}
}
for _, cl in ipairs(marcescent_sprays) do
  local cx, cy, n = cl[1], cl[2], cl[3]
  for i = 1, n do
    local ox = rand(-14, 14)
    local oy = rand(-11, 11)
    local roll = rand(0, 1)
    local p = (roll < 0.4) and p_leaf_russet or p_leaf_gold
    b_leaf:load(p, 0.7)
    b_leaf:touch(cx + ox, cy + oy, {pressure=rand(0.35, 0.7), drag={rand(1.2, 2.6), rand(1.1, 1.6)}})
  end
end

-- 6. Smooth the Companion Tree into a Graceful, Windswept Baltic Tree
-- Stroke smooth continuous trunk
b_branch:load(p_bark_deep, 0.9)
b_branch:stroke({
  {766, 444}, {764, 415}, {760, 385}, {754, 355}, {748, 325},
  {744, 290}, {742, 255}, {744, 220}, {748, 185}
}, {pressure={0.85, 0.75, 0.65, 0.55, 0.45, 0.35, 0.25, 0.18, 0.1}, ramps={0.02, 0.05}})

-- Windswept branches sweeping eastward
local comp_smooth_branches = {
  {{754, 355}, {778, 340}, {805, 322}, {835, 305}, {862, 290}, {885, 278}},
  {{805, 322}, {822, 298}, {836, 272}, {848, 248}, {856, 225}},
  {{744, 290}, {764, 268}, {780, 244}, {794, 220}, {805, 198}},
  {{742, 255}, {756, 232}, {768, 208}, {778, 185}},
  {{748, 185}, {758, 162}, {768, 142}, {776, 125}}
}
for _, b in ipairs(comp_smooth_branches) do
  b_branch:load(p_bark_deep, 0.85)
  b_branch:stroke(b, {pressure={0.7, 0.2}, ramps={0.02, 0.1}})
end

-- Fine windswept twigs
local comp_fine_twigs = {
  {{885, 278}, {902, 270}, {918, 264}},
  {{885, 278}, {896, 290}, {908, 302}},
  {{856, 225}, {868, 210}, {878, 195}},
  {{805, 198}, {818, 182}, {828, 168}},
  {{776, 125}, {785, 108}, {792, 92}}
}
for _, tw in ipairs(comp_fine_twigs) do
  b_rigger:load(p_bark_deep, 0.8)
  b_rigger:stroke(tw, {pressure={0.6, 0.12}, ramps={0.02, 0.15}})
end

-- A few delicate marcescent leaf touches on companion tree
local comp_leaf_sprays = {
  {815, 315, 12}, {850, 295, 14}, {875, 280, 10}, {835, 260, 12},
  {785, 245, 12}, {795, 215, 10}, {765, 225, 8}
}
for _, cl in ipairs(comp_leaf_sprays) do
  local cx, cy, n = cl[1], cl[2], cl[3]
  for i = 1, n do
    local ox = rand(-10, 10)
    local oy = rand(-8, 8)
    local p = (rand(0, 1) < 0.5) and p_leaf_russet or p_leaf_gold
    b_leaf:load(p, 0.65)
    b_leaf:touch(cx + ox, cy + oy, {pressure=rand(0.3, 0.6), drag={rand(1.0, 2.0), 1.4}})
  end
end

print("Chunk 75 complete: Oak grounded, limbs smoothed, intricate canopy built, companion tree sculpted.")

--@ chunk 76
-- Chunk 76: Smoothing Limbs, Foreground Glacial Boulders, Pilgrim Path, Heather Banks, Grasses & Rim-Lights

-- 1. Piles for Boulders, Path, Heather, and Twilight Rim
local p_bark_dark    = pile{{"bone black", 12}, {"raw umber", 8}, {"red earth", 2.0}, medium=0.15}
local p_granite_sh   = pile{{"raw umber", 8}, {"bone black", 6}, {"smalt", 2.2}, {"red earth", 1.2}, medium=0.18}
local p_granite_mid  = pile{{"lead white", 10}, {"raw umber", 6.5}, {"bone black", 3.0}, {"yellow ochre", 3.0}, {"smalt", 1.8}, {"red earth", 1.0}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 22}, {"smalt", 2.5}, {"yellow ochre", 2.8}, {"raw umber", 1.0}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage    = pile{{"lead white", 16}, {"green earth", 6.5}, {"yellow ochre", 3.5}, {"raw umber", 1.0}, medium=0.25}
local p_lich_gold    = pile{{"lead white", 12}, {"yellow ochre", 9.0}, {"chrome yellow", 1.8}, {"raw umber", 0.8}, medium=0.25}

local p_sand_path    = pile{{"yellow ochre", 7}, {"lead white", 5}, {"raw umber", 4}, {"bone black", 1}, medium=0.2}
local p_sand_light   = pile{{"lead white", 12}, {"yellow ochre", 7}, {"raw umber", 2.5}, {"vermilion", 0.3}, medium=0.22}
local p_heather_bank = pile{{"raw umber", 9}, {"bone black", 6}, {"red earth", 4.0}, {"yellow ochre", 2.5}, medium=0.18}
local p_heather_warm = pile{{"yellow ochre", 7}, {"red earth", 6.0}, {"raw umber", 4.5}, {"lead white", 1.5}, medium=0.2}
local p_moss_green   = pile{{"green earth", 7}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}
local p_straw_sharp  = pile{{"lead white", 16}, {"yellow ochre", 9}, {"chrome yellow", 1.2}, {"raw umber", 1.0}, medium=0.22}
local p_straw_shade  = pile{{"yellow ochre", 7}, {"raw umber", 6}, {"bone black", 2.5}, {"lead white", 2.0}, medium=0.2}
local p_twilight_rim = pile{{"lead white", 24}, {"yellow ochre", 3.5}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.2}

local b_block  = brush{kind="filbert", width=6.0, stiffness=0.85}
local b_facet  = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_crack  = brush{kind="round", width=1.0, point=1, stiffness=0.9}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}

-- 2. Smooth the Scalloped Bumps on Oak Limbs and Companion Tree
-- Eastern Bough top & bottom contour strokes to straighten the scalloped meatball dips
b_facet:load(p_bark_dark, 0.95)
b_facet:stroke({{280, 368}, {330, 356}, {380, 344}, {430, 332}, {480, 318}, {530, 298}}, {pressure=0.9})
b_facet:stroke({{285, 380}, {335, 370}, {385, 356}, {435, 342}, {485, 326}, {532, 306}}, {pressure=0.9})

-- Western Bough top & bottom contour strokes
b_facet:stroke({{268, 364}, {228, 348}, {188, 332}, {148, 314}, {105, 296}}, {pressure=0.9})
b_facet:stroke({{274, 376}, {234, 360}, {194, 342}, {154, 324}, {110, 304}}, {pressure=0.9})

-- Companion Tree trunk smoothing
b_facet:stroke({{766, 442}, {762, 410}, {758, 380}, {752, 350}, {746, 320}, {744, 285}, {742, 250}, {746, 215}, {750, 185}}, {pressure=0.85})

-- 3. The Glacial Erratic Boulders (*Findlinge*) in the Foreground
-- A. Left Foreground Boulder (x ≈ 210 to 275, y ≈ 525 to 555)
-- Massive, angular block of Swedish red-gray granite
local pts_b1 = {{210, 545}, {224, 526}, {258, 522}, {276, 532}, {274, 552}, {245, 558}, {214, 554}}
local m_b1 = poly(pts_b1, false) -- angular facets!
work(m_b1, {hand="body", pile=p_granite_mid, angle=0.2, coverage=1.8, fill=true, clip=true})

-- Deep cast shadow on undercut face
local pts_b1_sh = {{210, 545}, {235, 552}, {274, 552}, {245, 558}, {214, 554}}
local m_b1_sh = poly(pts_b1_sh, false)
work(m_b1_sh, {hand="detail", pile=p_granite_sh, angle=0.2, coverage=1.6, fill=true, clip=true})

-- Top sky-facing facet catching twilight
local pts_b1_lit = {{224, 526}, {258, 522}, {276, 532}, {260, 536}, {230, 534}}
local m_b1_lit = poly(pts_b1_lit, false)
work(m_b1_lit, {hand="detail", pile=p_granite_lit, angle=0.05, coverage=1.6, fill=true, clip=true})

-- Fracture cracks & lichen on Boulder 1
b_crack:load(p_granite_sh, 0.85)
b_crack:stroke({{242, 524}, {246, 536}, {244, 554}}, {pressure={0.5, 0.7, 0.2}})
b_crack:load(p_lich_sage, 0.75)
b_crack:touch(238, 528, {pressure=0.6, drag={2.0, 0.1}})
b_crack:touch(252, 526, {pressure=0.55, drag={1.8, 0.1}})
b_crack:load(p_lich_gold, 0.7)
b_crack:touch(262, 530, {pressure=0.5, drag={1.5, 0.2}})

-- B. Right Foreground Boulder (x ≈ 650 to 715, y ≈ 535 to 565)
local pts_b2 = {{652, 554}, {666, 536}, {698, 532}, {714, 544}, {712, 562}, {680, 566}, {655, 562}}
local m_b2 = poly(pts_b2, false)
work(m_b2, {hand="body", pile=p_granite_mid, angle=-0.2, coverage=1.8, fill=true, clip=true})

local pts_b2_sh = {{652, 554}, {675, 560}, {712, 562}, {680, 566}, {655, 562}}
local m_b2_sh = poly(pts_b2_sh, false)
work(m_b2_sh, {hand="detail", pile=p_granite_sh, angle=-0.2, coverage=1.6, fill=true, clip=true})

local pts_b2_lit = {{666, 536}, {698, 532}, {714, 544}, {695, 546}, {672, 544}}
local m_b2_lit = poly(pts_b2_lit, false)
work(m_b2_lit, {hand="detail", pile=p_granite_lit, angle=-0.05, coverage=1.6, fill=true, clip=true})

b_crack:load(p_granite_sh, 0.85)
b_crack:stroke({{682, 534}, {684, 546}, {682, 562}}, {pressure={0.5, 0.7, 0.2}})
b_crack:load(p_lich_sage, 0.75)
b_crack:touch(678, 538, {pressure=0.6, drag={2.0, 0.1}})
b_crack:load(p_lich_gold, 0.7)
b_crack:touch(692, 536, {pressure=0.55, drag={1.8, 0.1}})

-- C. Lower Center Boulder (x ≈ 420 to 468, y ≈ 625 to 650)
local pts_b3 = {{422, 642}, {434, 628}, {456, 626}, {466, 638}, {458, 648}, {430, 648}}
local m_b3 = poly(pts_b3, false)
work(m_b3, {hand="detail", pile=p_granite_mid, angle=0.1, coverage=1.6, fill=true, clip=true})
local pts_b3_lit = {{434, 628}, {456, 626}, {466, 638}, {452, 638}, {436, 636}}
local m_b3_lit = poly(pts_b3_lit, false)
work(m_b3_lit, {hand="detail", pile=p_granite_lit, angle=0.05, coverage=1.5, fill=true, clip=true})

-- 4. Bed all Boulders and Roots into Rich Heather Banks and Dark Peaty Turf
b_block:load(p_heather_bank, 0.95)
-- Around Boulder 1
b_block:stroke({{198, 556}, {230, 562}, {265, 560}, {295, 552}}, {pressure=0.9})
-- Around Boulder 2
b_block:stroke({{640, 564}, {675, 570}, {710, 568}, {735, 558}}, {pressure=0.9})
-- Around Boulder 3
b_block:stroke({{410, 650}, {440, 656}, {475, 652}}, {pressure=0.9})
-- Around Oak Root Flare
b_block:stroke({{210, 468}, {245, 464}, {285, 462}, {330, 458}}, {pressure=0.95})

-- 5. The Winding Pilgrim / Shepherd Path through the Heath
-- An organic track flowing naturally from lower right toward the dolmen
local pts_path = {{720, 705}, {680, 650}, {635, 595}, {585, 545}, {535, 502}, {485, 470}, {445, 452}}
local w_path   = {24, 20, 16, 12, 9, 6, 4}
local m_path   = ribbon(pts_path, w_path):soften(3)

work(m_path, {
  hand = "scumble",
  pile = p_sand_path,
  angle = 0.4,
  coverage = 1.3,
  fill = true
})

-- Subtle light sandy center rut
local m_path_lit = ribbon(pts_path, {12, 10, 8, 6, 4, 3, 2}):soften(2)
work(m_path_lit, {
  hand = "scumble",
  pile = p_sand_light,
  angle = 0.35,
  coverage = 1.1,
  fill = true
})

-- Small embedded pebbles on the path
local path_pebbles = {
  {695, 670, 3.2}, {665, 630, 3.5}, {620, 580, 2.8},
  {575, 535, 3.0}, {525, 495, 2.5}, {475, 465, 2.2}
}
for _, pb in ipairs(path_pebbles) do
  b_crack:load(p_granite_sh, 0.8)
  b_crack:touch(pb[1], pb[2], {pressure=0.6, drag={pb[3], 0.1}})
  b_crack:load(p_granite_lit, 0.7)
  b_crack:touch(pb[1], pb[2] - 0.5, {pressure=0.4, drag={pb[3]*0.6, 0.1}})
end

-- 6. Undulating Banks of Late-Autumn Heather (*Heidekraut*) across the Foreground
local m_h_bank1 = (ellipse(260, 590, 200, 45) + ellipse(760, 580, 220, 50)):soften(18)
work(m_h_bank1, {hand="body", pile=p_heather_warm, angle=0.2, coverage=1.2, fill=true})

local m_h_bank2 = (ellipse(450, 545, 180, 38) + ellipse(180, 540, 140, 35)):soften(18)
work(m_h_bank2, {hand="body", pile=p_moss_green, angle=-0.15, coverage=1.2, fill=true})

-- 7. Windswept Bent-Grass Tufts curving in the coastal wind
local function draw_curved_grass(bx, by, len, ang, bend, p)
  local mx = bx + len * 0.55 * math.cos(ang + bend * 0.5)
  local my = by - len * 0.55 * math.sin(ang + bend * 0.5)
  local tx = bx + len * math.cos(ang + bend)
  local ty = by - len * math.sin(ang + bend)
  b_rigger:load(p, 0.7)
  b_rigger:stroke({{bx, by}, {mx, my}, {tx, ty}}, {pressure={0.7, 0.05}, ramps={0.05, 0.3}})
end

local function draw_grass_cluster(cx, cy, n, h, lean)
  for i = 1, n do
    local bx = cx + rand(-5, 5)
    local by = cy + rand(-2, 2)
    local len = h * rand(0.75, 1.25)
    local ang = 1.57 + rand(-0.25, 0.25) + lean * 0.3
    local bnd = lean * rand(0.2, 0.5)
    local p = (rand(0, 1) < 0.65) and p_straw_sharp or p_straw_shade
    draw_curved_grass(bx, by, len, ang, bnd, p)
  end
end

-- Grasses around Boulder 1
local b1_grass = {{200, 554, 7, 16, -0.4}, {225, 558, 6, 15, -0.2}, {268, 556, 7, 16, 0.3}, {285, 550, 6, 14, 0.4}}
for _, g in ipairs(b1_grass) do draw_grass_cluster(g[1], g[2], g[3], g[4], g[5]) end

-- Grasses around Boulder 2
local b2_grass = {{645, 562, 7, 16, -0.4}, {675, 568, 6, 15, -0.1}, {708, 566, 7, 15, 0.3}, {728, 558, 6, 14, 0.4}}
for _, g in ipairs(b2_grass) do draw_grass_cluster(g[1], g[2], g[3], g[4], g[5]) end

-- Grasses along path margins
local path_grass = {{710, 685, 6, 14, 0.3}, {670, 640, 6, 13, -0.3}, {625, 585, 5, 12, 0.2}, {575, 535, 5, 12, -0.2}, {525, 495, 5, 11, 0.2}}
for _, g in ipairs(path_grass) do draw_grass_cluster(g[1], g[2], g[3], g[4], g[5]) end

-- Grasses around Oak Roots & Dolmen
local crest_grass = {{215, 466, 8, 18, -0.4}, {240, 464, 8, 17, -0.2}, {280, 462, 7, 16, 0.1}, {320, 458, 7, 16, 0.3}, {340, 448, 6, 14, 0.4}, {465, 448, 6, 14, 0.4}}
for _, g in ipairs(crest_grass) do draw_grass_cluster(g[1], g[2], g[3], g[4], g[5]) end

-- 8. Dried Carline Thistle Skeletons along the Ridge Crest silhouetted against glowing mist
local thistle_spots = {
  -- Left flank
  {145, 462, 22}, {165, 458, 20},
  -- Saddle
  {575, 456, 20}, {595, 458, 18},
  -- Right slope
  {855, 452, 20}, {875, 456, 18}
}
for _, th in ipairs(thistle_spots) do
  local x, y, h = th[1], th[2], th[3]
  b_rigger:load(p_bark_dark, 0.85)
  -- Stem
  b_rigger:stroke({{x, y}, {x, y - h*0.6}, {x + rand(-2, 2), y - h}}, {pressure={0.7, 0.15}, ramps={0.05, 0.2}})
  -- Dried flower head
  b_crack:load(p_straw_sharp, 0.8)
  b_crack:stroke({{x - 3, y - h}, {x + 3, y - h}}, {pressure=0.4})
  b_crack:stroke({{x, y - h - 2}, {x, y - h + 2}}, {pressure=0.4})
end

-- 9. Twilight Rim-Lights on Sky-Facing Contours
b_rigger:load(p_twilight_rim, 0.65)
-- Capstone top rim
b_rigger:stroke({{318, 408}, {340, 396}, {380, 391}, {420, 392}, {455, 396}, {474, 403}}, {pressure={0.4, 0.7, 0.6, 0.2}})
-- Oak trunk left contour
b_rigger:stroke({{232, 456}, {242, 440}, {252, 420}, {258, 390}, {262, 360}}, {pressure={0.4, 0.6, 0.5, 0.2}})
-- Companion tree left contour
b_rigger:stroke({{766, 442}, {762, 410}, {758, 375}, {752, 340}, {746, 305}}, {pressure={0.35, 0.5, 0.4, 0.15}})

print("Chunk 76 complete: Limbs smoothed, boulders bedded, path laid, heather banks articulated, grasses planted, thistles standing, and rim-lights stroked.")

--@ chunk 77
-- Chunk 77: Retrenching the Oak into an Authentic Ancient Veteran, Softening Path & Heather

-- 1. Blend the foreground wet-into-wet to fuse the yellow path and heather ovals into subtle, continuous heathland
local m_heath_body = below(barrow_crest)
blend(m_heath_body, {angle=0.2})
blend(m_heath_body, {angle=-0.15})
blend(m_heath_body, {angle=0.1})

-- 2. Retrench the Oak's Upper Crown with Sky Body Paint (Cut away the three giant upright telephone poles above y ≈ 250)
local p_sky_zenith = pile{{"lead white", 18}, {"cobalt blue", 6.2}, {"smalt", 4.5}, {"raw umber", 0.6}, {"bone black", 0.25}, medium=0.22}
local p_sky_upper  = pile{{"lead white", 26}, {"cobalt blue", 3.2}, {"smalt", 3.2}, {"yellow ochre", 0.8}, {"raw umber", 0.3}, medium=0.22}
local p_sky_mid    = pile{{"lead white", 28}, {"yellow ochre", 3.5}, {"chrome yellow", 0.9}, {"smalt", 1.0}, {"raw umber", 0.2}, {"vermilion", 0.1}, medium=0.22}

-- Cover the tall poles on the left oak (x = 160 to 360, y = 50 to 250)
local m_cut_oak = rect(150, 40, 230, 210):soften(15)
work(m_cut_oak * rect(150, 40, 230, 110), {hand="body", pile=p_sky_zenith, angle=0, coverage=2.0, fill=true})
work(m_cut_oak * rect(150, 120, 230, 100), {hand="body", pile=p_sky_upper, angle=0, coverage=2.0, fill=true})
work(m_cut_oak * rect(150, 200, 230, 70), {hand="body", pile=p_sky_mid, angle=0, coverage=2.0, fill=true})

-- Cover the tall top of the companion tree (x = 700 to 820, y = 80 to 240)
local m_cut_comp = rect(700, 70, 140, 180):soften(12)
work(m_cut_comp * rect(700, 70, 140, 90), {hand="body", pile=p_sky_upper, angle=0, coverage=2.0, fill=true})
work(m_cut_comp * rect(700, 150, 140, 100), {hand="body", pile=p_sky_mid, angle=0, coverage=2.0, fill=true})

-- Blend the painted sky passages horizontally to seamlessly restore the vast twilight sky
blend(rect(140, 30, 250, 240), {angle=0})
blend(rect(690, 60, 160, 200), {angle=0})

-- 3. Sculpt the Shattered, Weathered Retrenchment Top of the Ancient Oak (y ≈ 250 to 320)
-- An authentic ancient Quercus robur shattered by centuries of Baltic storms
local p_stag_dark   = pile{{"raw umber", 8}, {"bone black", 6}, {"lead white", 3}, medium=0.18}
local p_stag_bleach = pile{{"lead white", 18}, {"raw umber", 4.0}, {"yellow ochre", 2.5}, {"smalt", 1.0}, medium=0.2}
local p_bark_solid  = pile{{"bone black", 12}, {"raw umber", 8}, {"red earth", 2.0}, medium=0.15}

local b_wood  = brush{kind="filbert", width=4.5, stiffness=0.85}
local b_sharp = brush{kind="round", width=1.5, point=1, stiffness=0.9}

-- Shattered crown summit (jagged, weathered heartwood stubs)
b_wood:load(p_stag_dark, 0.9)
b_wood:stroke({{260, 330}, {264, 290}, {266, 260}}, {pressure={0.85, 0.7, 0.3}})
b_wood:stroke({{274, 330}, {278, 295}, {282, 270}}, {pressure={0.85, 0.7, 0.3}})

-- Splintered jagged dead stubs pointing into the sky
b_sharp:load(p_stag_bleach, 0.95)
b_sharp:stroke({{264, 280}, {266, 258}}, {pressure={0.6, 0.1}})
b_sharp:stroke({{266, 275}, {262, 252}}, {pressure={0.5, 0.05}})
b_sharp:stroke({{278, 285}, {282, 265}}, {pressure={0.6, 0.1}})
b_sharp:stroke({{278, 285}, {286, 272}}, {pressure={0.5, 0.08}})

-- 4. Smooth and Unify the Great Living Boughs of the Oak
-- Smooth Eastern bough contour into a powerful muscular arch
b_wood:load(p_bark_solid, 0.9)
b_wood:stroke({
  {276, 365}, {320, 355}, {370, 345}, {420, 335}, {470, 322}, {515, 305}
}, {pressure={0.9, 0.85, 0.75, 0.65, 0.5, 0.3}, ramps={0.02, 0.05}})

-- Secondary boughs gracefully arching over the dolmen
b_sharp:load(p_bark_solid, 0.85)
b_sharp:stroke({{370, 345}, {390, 322}, {412, 298}, {430, 275}}, {pressure={0.75, 0.3}})
b_sharp:stroke({{420, 335}, {445, 352}, {472, 366}, {495, 376}}, {pressure={0.7, 0.25}})
b_sharp:stroke({{470, 322}, {495, 302}, {520, 285}}, {pressure={0.65, 0.2}})
b_sharp:stroke({{515, 305}, {535, 318}, {552, 330}}, {pressure={0.6, 0.2}})

-- Smooth Western living bough
b_wood:load(p_bark_solid, 0.9)
b_wood:stroke({
  {268, 362}, {228, 348}, {188, 332}, {148, 315}, {105, 298}
}, {pressure={0.85, 0.75, 0.65, 0.5, 0.3}, ramps={0.02, 0.05}})
b_sharp:stroke({{210, 340}, {192, 312}, {175, 282}, {158, 252}}, {pressure={0.7, 0.25}})
b_sharp:stroke({{170, 325}, {150, 348}, {128, 368}}, {pressure={0.65, 0.2}})

-- 5. Reshape the Companion Tree into a Natural, Windswept Silhouette (Top at y ≈ 250)
b_wood:load(p_bark_solid, 0.9)
b_wood:stroke({
  {766, 442}, {762, 405}, {756, 365}, {750, 325}, {746, 285}, {744, 252}
}, {pressure={0.8, 0.7, 0.6, 0.45, 0.3, 0.1}, ramps={0.02, 0.05}})
b_sharp:stroke({{750, 325}, {772, 310}, {798, 292}, {824, 278}}, {pressure={0.6, 0.15}})
b_sharp:stroke({{756, 365}, {778, 350}, {805, 332}, {832, 318}}, {pressure={0.65, 0.2}})

-- Restore the waxing crescent moon and evening star
local p_moon_pure = pile{{"lead white", 34}, {"yellow ochre", 1.0}, {"chrome yellow", 0.4}, medium=0.12}
local b_rigger = brush{kind="round", width=1.3, point=1, stiffness=0.85}
b_rigger:load(p_moon_pure, 0.95)
b_rigger:stroke({
  {645.2, 105.5}, {641.5, 108.0}, {639.0, 112.5}, {640.2, 117.0}, {644.0, 119.5}
}, {pressure={0.08, 0.85, 1.0, 0.8, 0.08}, ramps={0.05, 0.05}})

local b_star = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_star:load(p_moon_pure, 1.0)
b_star:touch(688, 148, {pressure=0.6})

print("Chunk 77 complete: Oak retrenched, telephone poles removed, sky restored, path and heath blended.")

--@ chunk 78
-- Chunk 78: Seamless Full-Width Sky Restoration (y = 0 to 350) eliminating rectangular boxes

-- 1. Sky Piles
local p_sky_zenith = pile{{"lead white", 18}, {"cobalt blue", 6.2}, {"smalt", 4.5}, {"raw umber", 0.6}, {"bone black", 0.25}, medium=0.22}
local p_sky_upper  = pile{{"lead white", 26}, {"cobalt blue", 3.2}, {"smalt", 3.2}, {"yellow ochre", 0.8}, {"raw umber", 0.3}, medium=0.22}
local p_sky_mid    = pile{{"lead white", 28}, {"yellow ochre", 3.5}, {"chrome yellow", 0.9}, {"smalt", 1.0}, {"raw umber", 0.2}, {"vermilion", 0.1}, medium=0.22}
local p_sky_glow   = pile{{"lead white", 26}, {"yellow ochre", 5.2}, {"chrome yellow", 2.0}, {"vermilion", 0.45}, {"raw umber", 0.25}, medium=0.22}

-- 2. Three overlapping horizontal bands across the ENTIRE canvas width (x = 0 to 1000)
local m1 = rect(0, 0, 1000, 140):soften(25)
work(m1, {hand="body", pile=p_sky_zenith, angle=0, order="across", coverage=2.0, fill=true})

local m2 = rect(0, 100, 1000, 145):soften(25)
work(m2, {hand="body", pile=p_sky_upper, angle=0, order="across", coverage=2.0, fill=true})

local m3 = rect(0, 205, 1000, 150):soften(25)
work(m3, {hand="body", pile=p_sky_mid, angle=0, order="across", coverage=2.0, fill=true})

-- 3. Seamless horizontal blend across the full canvas width
local m_sky_full = rect(0, 0, 1000, 360)
blend(m_sky_full, {angle=0})
blend(m_sky_full, {angle=1.57})
blend(m_sky_full, {angle=0})

-- 4. Restore the delicate crescent moon and evening star in the pristine twilight
local p_moon_pure = pile{{"lead white", 34}, {"yellow ochre", 1.0}, {"chrome yellow", 0.4}, medium=0.12}
local b_rigger = brush{kind="round", width=1.3, point=1, stiffness=0.85}
b_rigger:load(p_moon_pure, 0.95)
b_rigger:stroke({
  {645.2, 105.5}, {641.5, 108.0}, {639.0, 112.5}, {640.2, 117.0}, {644.0, 119.5}
}, {pressure={0.08, 0.85, 1.0, 0.8, 0.08}, ramps={0.05, 0.05}})

local b_star = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_star:load(p_moon_pure, 1.0)
b_star:touch(688, 148, {pressure=0.6})

print("Chunk 78 complete: Sky restored across full width; rectangular boxes eliminated.")

--@ chunk 79
-- Chunk 79: Advancing time to let all paint dry completely
local t = wait(4 * 24 * 60)
print("Time after drying:", t)
print("Drying state at sky (500, 250):", drying(500, 250))
print("Drying state at tree (270, 350):", drying(270, 350))
print("Drying state at ground (500, 550):", drying(500, 550))

--@ chunk 80
-- Chunk 80: Restoring the Master Friedrichian Sky across Full Width (y = 0 to 430)

-- 1. Sky Piles
local p_sky_zenith = pile{{"lead white", 18}, {"cobalt blue", 6.2}, {"smalt", 4.5}, {"raw umber", 0.6}, {"bone black", 0.25}, medium=0.22}
local p_sky_upper  = pile{{"lead white", 26}, {"cobalt blue", 3.2}, {"smalt", 3.2}, {"yellow ochre", 0.8}, {"raw umber", 0.3}, medium=0.22}
local p_sky_mid    = pile{{"lead white", 28}, {"yellow ochre", 3.5}, {"chrome yellow", 0.9}, {"smalt", 1.0}, {"raw umber", 0.2}, {"vermilion", 0.1}, medium=0.22}
local p_sky_glow   = pile{{"lead white", 26}, {"yellow ochre", 5.2}, {"chrome yellow", 2.2}, {"vermilion", 0.5}, {"raw umber", 0.25}, medium=0.22}

-- 2. Four overlapping bands covering x = 0 to 1000 from zenith down behind the barrow (y = 430)
local m1 = rect(0, 0, 1000, 145):soften(30)
work(m1, {hand="body", pile=p_sky_zenith, angle=0, order="across", coverage=2.2, fill=true})

local m2 = rect(0, 105, 1000, 150):soften(30)
work(m2, {hand="body", pile=p_sky_upper, angle=0, order="across", coverage=2.2, fill=true})

local m3 = rect(0, 215, 1000, 140):soften(30)
work(m3, {hand="body", pile=p_sky_mid, angle=0, order="across", coverage=2.2, fill=true})

local m4 = rect(0, 315, 1000, 125):soften(25)
work(m4, {hand="body", pile=p_sky_glow, angle=0, order="across", coverage=2.2, fill=true})

-- 3. Seamless badger blending across the full canvas width
local m_sky_all = rect(0, 0, 1000, 430)
blend(m_sky_all, {angle=1.57})
blend(m_sky_all, {angle=0})

-- 4. Distant low Baltic hills silhouetted in soft atmospheric haze
local p_dist_coast = pile{{"lead white", 15}, {"smalt", 5.0}, {"raw umber", 3.0}, {"red earth", 0.9}, {"bone black", 0.4}, medium=0.25}
local pts_distant_land = {
  {0, 464}, {100, 460}, {220, 454}, {340, 452}, {460, 448},
  {580, 451}, {700, 447}, {820, 452}, {920, 458}, {1000, 464},
  {1000, 475}, {0, 475}
}
local m_coast = poly(pts_distant_land, true):soften(2.5)
work(m_coast, {hand="body", pile=p_dist_coast, angle=0, coverage=1.2, fill=true, clip=true})

-- 5. Calm water sheen and valley mist pooling over the water
local p_water_mist = pile{{"lead white", 26}, {"yellow ochre", 3.2}, {"smalt", 1.8}, {"raw umber", 0.3}, medium=0.3}
local m_water = rect(0, 438, 1000, 28):soften(4)
work(m_water, {hand="glaze", pile=p_water_mist, angle=0, coverage=1.1, fill=false})
blend(rect(0, 435, 1000, 32), {angle=0})

-- 6. Waxing crescent moon and solitary evening star (Venus)
local p_moon_pure = pile{{"lead white", 34}, {"yellow ochre", 1.0}, {"chrome yellow", 0.4}, medium=0.12}
local b_rigger = brush{kind="round", width=1.3, point=1, stiffness=0.85}
b_rigger:load(p_moon_pure, 0.95)
b_rigger:stroke({
  {645.2, 105.5}, {641.5, 108.0}, {639.0, 112.5}, {640.2, 117.0}, {644.0, 119.5}
}, {pressure={0.08, 0.85, 1.0, 0.8, 0.08}, ramps={0.05, 0.05}})

local b_star = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_star:load(p_moon_pure, 1.0)
b_star:touch(688, 148, {pressure=0.6})

print("Chunk 80 complete: Master sky restored flawlessly across full canvas width.")

--@ chunk 81
-- Chunk 81: Phase A — Sculpting the Swelling Barrow Crest & Monumental Megalithic Tomb

-- 1. Mathematical spline for the swelling barrow mound crest
local barrow_nodes = {
  {x=-20, y=468}, {x=80, y=462}, {x=180, y=452}, {x=270, y=435},
  {x=340, y=422}, {x=385, y=416}, {x=450, y=422}, {x=520, y=436},
  {x=600, y=452}, {x=680, y=454}, {x=760, y=440}, {x=830, y=440},
  {x=910, y=452}, {x=1020, y=468}
}

local function barrow_crest(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 445
end

local m_knoll = below(barrow_crest)

-- 2. Earth & Peat Palette
local p_earth_body   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4}, {"yellow ochre", 3}, medium=0.18}
local p_peat_crest   = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.0}, medium=0.15}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5}, {"lead white", 2}, medium=0.2}

-- Cover the white horizon gap and seal the upper barrow slope down to y = 500
local m_crest_fill = m_knoll * rect(0, 410, 1000, 90)
work(m_crest_fill, {
  hand = "body",
  pile = p_earth_body,
  angle = 0.2,
  coverage = 2.2,
  fill = true,
  clip = true
})

-- Swelling heather bank on summit
local m_summit_bank = (ellipse(385, 440, 160, 30) + ellipse(260, 455, 140, 32)):soften(15) * m_knoll
work(m_summit_bank, {hand="body", pile=p_heath_russet, angle=0.18, coverage=1.4, fill=true})

-- Firm, continuous, organic stroke along the crest curve against the glowing mist
local b_crest = brush{kind="round", width=2.8, point=1, stiffness=0.9}
b_crest:load(p_peat_crest, 0.95)
local crest_line = {}
for x = -10, 1010, 10 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest:stroke(crest_line, {pressure=0.9, ramps={0.01, 0.01}})

-- 3. The Megalithic Tomb (Hünengrab) atop the Barrow Summit
local p_cavern_void   = pile{{"bone black", 12}, {"raw umber", 8}, medium=0.1}
local p_granite_deep  = pile{{"bone black", 8}, {"raw umber", 7}, {"smalt", 2.0}, {"red earth", 1.5}, medium=0.15}
local p_granite_shade = pile{{"raw umber", 8}, {"bone black", 5}, {"smalt", 2.2}, {"lead white", 2.5}, {"red earth", 1.2}, {"yellow ochre", 1.5}, medium=0.18}
local p_granite_body  = pile{{"lead white", 10}, {"raw umber", 6}, {"bone black", 3}, {"yellow ochre", 3.2}, {"smalt", 1.8}, {"red earth", 1.2}, medium=0.2}
local p_granite_facet = pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.22}
local p_granite_light = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage     = pile{{"lead white", 16}, {"green earth", 6.5}, {"yellow ochre", 3.5}, {"raw umber", 1.0}, medium=0.25}
local p_lich_gold     = pile{{"lead white", 12}, {"yellow ochre", 9.0}, {"chrome yellow", 1.8}, {"raw umber", 0.8}, medium=0.25}

local b_block = brush{kind="filbert", width=5.5, stiffness=0.85}
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_crack = brush{kind="round", width=0.9, point=1, stiffness=0.9}

-- A. Sacred Burial Chamber Void
b_block:load(p_cavern_void, 0.95)
b_block:stroke({{344, 414}, {385, 414}, {425, 414}, {452, 415}}, {pressure=0.9})
b_block:stroke({{348, 424}, {395, 426}, {448, 425}}, {pressure=0.9})
b_block:stroke({{352, 434}, {400, 436}, {444, 435}}, {pressure=0.9})

-- B. Left Orthostat
b_block:load(p_granite_shade, 0.9)
b_block:stroke({{338, 412}, {342, 426}, {346, 442}}, {pressure=0.85})
b_block:stroke({{348, 412}, {352, 426}, {356, 442}}, {pressure=0.85})
b_facet:load(p_granite_facet, 0.8)
b_facet:stroke({{338, 412}, {340, 424}, {344, 435}, {346, 444}}, {pressure={0.7, 0.5, 0.6, 0.2}})

-- C. Rear Orthostat
b_facet:load(p_granite_deep, 0.85)
b_facet:stroke({{394, 414}, {396, 428}, {398, 438}}, {pressure=0.75})
b_facet:stroke({{406, 414}, {408, 428}, {410, 438}}, {pressure=0.75})

-- D. Right Orthostat
b_block:load(p_granite_shade, 0.9)
b_block:stroke({{428, 413}, {432, 426}, {434, 442}}, {pressure=0.85})
b_block:stroke({{438, 413}, {442, 426}, {445, 442}}, {pressure=0.85})
b_block:stroke({{448, 412}, {452, 425}, {455, 442}}, {pressure=0.85})
b_facet:load(p_granite_facet, 0.85)
b_facet:stroke({{448, 412}, {452, 422}, {456, 433}, {458, 442}}, {pressure={0.75, 0.8, 0.6, 0.2}})
b_facet:load(p_granite_light, 0.8)
b_facet:stroke({{450, 412}, {454, 418}, {452, 426}}, {pressure={0.65, 0.5, 0.2}})

-- E. Massive Granite Capstone
-- Deep underside cast shadow
b_block:load(p_granite_deep, 0.95)
b_block:stroke({{316, 411}, {355, 412}, {400, 413}, {445, 412}, {474, 410}}, {pressure=0.9})
b_crack:load(p_cavern_void, 0.95)
b_crack:stroke({{314, 412}, {350, 414}, {395, 415}, {440, 414}, {476, 411}}, {pressure=0.85})

-- Front face (crystalline texture)
b_block:load(p_granite_body, 0.9)
b_block:stroke({{318, 408}, {350, 409}, {390, 410}, {430, 409}, {468, 407}}, {pressure=0.85})
b_block:stroke({{324, 402}, {360, 403}, {405, 404}, {445, 403}, {470, 401}}, {pressure=0.85})

-- Top sky-facing facets (sharp, angular crystalline planes)
local pts_fA = {{314, 409}, {322, 400}, {340, 392}, {368, 388}, {394, 387}, {392, 400}, {365, 402}, {338, 405}, {318, 409}}
local m_fA = poly(pts_fA, false)
work(m_fA, {hand="detail", pile=p_granite_light, angle=0.04, coverage=1.6, fill=true, clip=true})

local pts_fB = {{394, 387}, {425, 389}, {455, 392}, {472, 398}, {476, 406}, {466, 407}, {440, 404}, {412, 402}, {392, 400}}
local m_fB = poly(pts_fB, false)
work(m_fB, {hand="detail", pile=p_granite_facet, angle=-0.04, coverage=1.6, fill=true, clip=true})

-- Fracture cleavage cracks
b_crack:load(p_granite_deep, 0.9)
b_crack:stroke({{394, 387}, {392, 400}, {390, 414}}, {pressure={0.7, 0.85, 0.3}})
b_crack:stroke({{348, 391}, {352, 402}, {350, 412}}, {pressure={0.45, 0.6, 0.2}})
b_crack:stroke({{442, 391}, {444, 401}, {446, 412}}, {pressure={0.5, 0.65, 0.2}})

-- Top edge crystalline highlights
b_crack:load(p_granite_light, 0.95)
b_crack:stroke({{314, 409}, {322, 400}, {340, 392}, {368, 388}, {394, 387}}, {pressure={0.3, 0.8, 0.7, 0.85, 0.4}})
b_crack:stroke({{396, 388}, {425, 389}, {455, 392}, {472, 398}}, {pressure={0.4, 0.8, 0.75, 0.3}})

-- Lichen touches
local lichen_patches = {
  {335, 396, p_lich_sage, 2.2},
  {362, 392, p_lich_gold, 2.5},
  {380, 398, p_lich_sage, 1.8},
  {415, 394, p_lich_gold, 2.4},
  {438, 396, p_lich_sage, 2.0},
  {458, 400, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichen_patches) do
  b_facet:load(lp[3], 0.7)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- F. Flanking Erratics
local pts_el = {{294, 442}, {302, 426}, {318, 420}, {328, 428}, {324, 444}, {298, 444}}
local m_el = poly(pts_el, false)
work(m_el, {hand="detail", pile=p_granite_body, angle=0.3, coverage=1.6, fill=true, clip=true})
local pts_el_lit = {{302, 426}, {318, 420}, {328, 428}, {316, 432}, {306, 431}}
local m_el_lit = poly(pts_el_lit, false)
work(m_el_lit, {hand="detail", pile=p_granite_facet, angle=0.15, coverage=1.4, fill=true, clip=true})

local pts_er = {{470, 442}, {478, 424}, {504, 420}, {516, 432}, {508, 444}, {474, 444}}
local m_er = poly(pts_er, false)
work(m_er, {hand="detail", pile=p_granite_body, angle=-0.2, coverage=1.6, fill=true, clip=true})
local pts_er_lit = {{478, 424}, {504, 420}, {516, 432}, {500, 431}, {484, 429}}
local m_er_lit = poly(pts_er_lit, false)
work(m_er_lit, {hand="detail", pile=p_granite_facet, angle=-0.1, coverage=1.4, fill=true, clip=true})

-- G. Anchor all stone bases into turf with dark peat
b_block:load(p_peat_crest, 0.95)
b_block:stroke({{285, 444}, {330, 444}, {370, 444}}, {pressure=0.9})
b_block:stroke({{420, 444}, {470, 444}, {525, 444}}, {pressure=0.9})

print("Phase A complete: Barrow crest sealed, Megalithic Tomb sculpted.")

--@ chunk 82
-- Chunk 82: Fusing the Barrow Slope into Seamless Earth and Bedding the Dolmen

-- 1. Blend the entire barrow ground from the crest all the way to canvas bottom
-- This eliminates the horizontal seam at y = 500 and fuses the earth into a continuous, sculptural barrow
local m_ground = below(barrow_crest)
blend(m_ground, {angle=1.57})
blend(m_ground, {angle=0.2})
blend(m_ground, {angle=-0.15})

-- 2. Bed the dolmen orthostats and flanking stones firmly into the barrow soil
local p_peat_dark = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.0}, medium=0.15}
local b_turf = brush{kind="filbert", width=4.5, stiffness=0.85}
b_turf:load(p_peat_dark, 0.95)
b_turf:stroke({{285, 442}, {330, 444}, {370, 443}}, {pressure=0.9})
b_turf:stroke({{420, 443}, {465, 444}, {525, 442}}, {pressure=0.9})

-- Base contact shadow directly beneath the upright stones
local p_void = pile{{"bone black", 12}, {"raw umber", 8}, medium=0.1}
local b_det = brush{kind="round", width=2.0, point=1, stiffness=0.9}
b_det:load(p_void, 0.95)
b_det:stroke({{336, 442}, {368, 442}}, {pressure=0.8})
b_det:stroke({{428, 442}, {460, 442}}, {pressure=0.8})

print("Chunk 82 complete: Barrow ground fused into seamless earth, Dolmen firmly bedded.")

--@ chunk 83
-- Chunk 83: The Ancient Veteran Oak — Direct Masterful Brushwork

-- 1. Wood, Bark, Stag, and Foliage Piles
local p_bark_solid   = pile{{"bone black", 12}, {"raw umber", 8}, {"red earth", 2.0}, medium=0.15}
local p_bark_mid     = pile{{"raw umber", 8}, {"bone black", 5}, {"yellow ochre", 4.0}, {"lead white", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_bark_ridge   = pile{{"yellow ochre", 7}, {"lead white", 5.0}, {"raw umber", 4.0}, {"bone black", 1.2}, medium=0.2}
local p_hollow_pitch = pile{{"bone black", 14}, {"raw umber", 6}, medium=0.1}
local p_woundwood    = pile{{"raw umber", 6}, {"yellow ochre", 5.5}, {"lead white", 3.5}, {"red earth", 2.0}, medium=0.2}
local p_stag_shade   = pile{{"raw umber", 8}, {"bone black", 5}, {"lead white", 5}, {"yellow ochre", 2.5}, medium=0.2}
local p_stag_bleach  = pile{{"lead white", 18}, {"raw umber", 3.5}, {"yellow ochre", 2.5}, {"smalt", 1.0}, medium=0.22}
local p_peat_earth   = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, {"green earth", 2.5}, medium=0.15}

local p_leaf_russet  = pile{{"yellow ochre", 7}, {"red earth", 6.5}, {"raw umber", 4.0}, {"lead white", 1.5}, medium=0.22}
local p_leaf_gold    = pile{{"yellow ochre", 9}, {"chrome yellow", 2.5}, {"lead white", 4.0}, {"vermilion", 0.4}, medium=0.25}

local b_bole   = brush{kind="filbert", width=8.0, stiffness=0.85}
local b_limb   = brush{kind="filbert", width=5.0, stiffness=0.85}
local b_branch = brush{kind="round", width=2.6, point=1, stiffness=0.85}
local b_twig   = brush{kind="round", width=1.4, point=1, stiffness=0.85}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}
local b_leaf   = brush{kind="round", width=1.6, point=1, stiffness=0.8}

-- 2. Massive, Muscular Trunk & Root Buttresses (Firmly anchored into barrow soil)
-- Western flaring root buttress
b_bole:load(p_bark_solid, 0.95)
b_bole:stroke({{225, 460}, {242, 450}, {258, 435}, {266, 410}, {270, 380}}, {pressure={0.9, 0.95, 0.9, 0.85, 0.8}})
-- Central anchoring root
b_bole:stroke({{262, 462}, {268, 445}, {272, 420}, {275, 385}, {274, 360}}, {pressure={0.95, 0.95, 0.9, 0.85, 0.8}})
-- Eastern root buttress gripping the barrow and covering the dark left eyebrow
b_bole:stroke({{325, 454}, {308, 446}, {295, 430}, {286, 400}, {280, 365}}, {pressure={0.95, 0.9, 0.85, 0.8, 0.8}})

-- Solid body fill of the trunk core
b_bole:stroke({{248, 448}, {254, 420}, {260, 385}, {265, 355}}, {pressure=0.9})
b_bole:stroke({{276, 448}, {278, 420}, {276, 385}, {274, 355}}, {pressure=0.9})
b_bole:stroke({{296, 448}, {294, 420}, {288, 385}, {282, 355}}, {pressure=0.9})

-- Bed root toes deep into dark peat and heather
b_bole:load(p_peat_earth, 0.95)
b_bole:stroke({{210, 464}, {250, 460}, {290, 458}, {335, 454}}, {pressure=0.95})
b_bole:stroke({{415, 446}, {460, 446}, {520, 444}}, {pressure=0.95}) -- covers right dark eyebrow!

-- 3. Longitudinal Bark Ridges and Ancient Hollow
b_twig:load(p_hollow_pitch, 0.9)
b_twig:stroke({{246, 452}, {254, 432}, {258, 408}, {262, 375}}, {pressure={0.8, 0.6, 0.3}})
b_twig:stroke({{266, 454}, {270, 435}, {272, 408}, {270, 370}}, {pressure={0.8, 0.7, 0.35}})
b_twig:stroke({{292, 450}, {290, 430}, {285, 405}, {280, 372}}, {pressure={0.8, 0.6, 0.35}})

b_twig:load(p_bark_ridge, 0.85)
b_twig:stroke({{238, 455}, {248, 442}, {254, 420}, {258, 385}}, {pressure={0.3, 0.75, 0.6, 0.2}})
b_twig:stroke({{260, 455}, {265, 436}, {268, 412}, {268, 378}}, {pressure={0.3, 0.7, 0.6, 0.2}})
b_twig:stroke({{276, 455}, {277, 436}, {276, 408}, {274, 372}}, {pressure={0.3, 0.65, 0.6, 0.2}})
b_twig:stroke({{300, 452}, {296, 434}, {290, 410}, {284, 375}}, {pressure={0.3, 0.7, 0.6, 0.2}})

-- Rot hollow with wound-wood callus
b_twig:load(p_hollow_pitch, 0.95)
b_twig:stroke({{272, 430}, {275, 410}, {272, 396}}, {pressure=0.8})
b_twig:load(p_woundwood, 0.85)
b_twig:stroke({{269, 432}, {270, 412}, {276, 395}}, {pressure={0.5, 0.85, 0.4}})
b_twig:stroke({{278, 428}, {279, 405}, {275, 395}}, {pressure={0.5, 0.85, 0.4}})

-- 4. The Great Living Eastern Bough (Muscular arch sheltering over the Dolmen)
-- Main stem: heavy, continuous, tapering muscular arch
b_limb:load(p_bark_solid, 0.95)
b_limb:stroke({
  {278, 368}, {320, 356}, {365, 344}, {415, 332}, {465, 318}, {510, 302}
}, {pressure={0.95, 0.85, 0.75, 0.65, 0.5, 0.3}, ramps={0.02, 0.05}})

-- Upper bark highlight along Eastern bough
b_branch:load(p_bark_mid, 0.8)
b_branch:stroke({
  {282, 362}, {322, 350}, {368, 338}, {418, 326}, {466, 312}, {508, 296}
}, {pressure={0.35, 0.65, 0.6, 0.45, 0.3, 0.15}})

-- Secondary boughs branching off the Eastern limb
-- E1: Upward muscular bough
b_branch:load(p_bark_solid, 0.9)
b_branch:stroke({
  {365, 344}, {385, 318}, {408, 288}, {428, 258}
}, {pressure={0.85, 0.7, 0.55, 0.25}, ramps={0.02, 0.1}})
b_twig:load(p_bark_solid, 0.85)
b_twig:stroke({{385, 318}, {375, 295}, {368, 272}}, {pressure={0.7, 0.2}})
b_twig:stroke({{408, 288}, {422, 268}, {432, 245}}, {pressure={0.65, 0.2}})

-- E2: Sheltering bough drooping directly over dolmen capstone
b_branch:load(p_bark_solid, 0.9)
b_branch:stroke({
  {415, 332}, {440, 350}, {468, 364}, {492, 375}
}, {pressure={0.8, 0.65, 0.5, 0.2}, ramps={0.02, 0.1}})
b_twig:load(p_bark_solid, 0.85)
b_twig:stroke({{468, 364}, {480, 382}, {490, 396}}, {pressure={0.6, 0.2}})

-- E3: Outer spreading canopy boughs
b_branch:load(p_bark_solid, 0.85)
b_branch:stroke({
  {465, 318}, {490, 298}, {515, 280}
}, {pressure={0.75, 0.55, 0.2}, ramps={0.02, 0.1}})
b_branch:stroke({
  {510, 302}, {532, 315}, {552, 326}
}, {pressure={0.7, 0.5, 0.2}, ramps={0.02, 0.1}})

-- 5. The Living Western Bough (Reaching west over the barrow flank)
b_limb:load(p_bark_solid, 0.95)
b_limb:stroke({
  {268, 365}, {230, 350}, {190, 334}, {150, 318}, {108, 300}
}, {pressure={0.9, 0.8, 0.65, 0.5, 0.25}, ramps={0.02, 0.05}})

-- W1: Upward western bough
b_branch:load(p_bark_solid, 0.9)
b_branch:stroke({
  {210, 342}, {194, 312}, {178, 280}, {160, 248}
}, {pressure={0.8, 0.65, 0.5, 0.25}, ramps={0.02, 0.1}})
b_twig:load(p_bark_solid, 0.85)
b_twig:stroke({{194, 312}, {205, 290}, {214, 268}}, {pressure={0.65, 0.2}})

-- W2: Drooping western bough
b_branch:load(p_bark_solid, 0.85)
b_branch:stroke({
  {170, 326}, {150, 348}, {128, 368}
}, {pressure={0.75, 0.55, 0.2}, ramps={0.02, 0.1}})

-- 6. The Bleached Stag-Head Crown aloft (Weathered dead wood spearing into the zenith)
-- Central dead spire
b_branch:load(p_stag_shade, 0.9)
b_branch:stroke({
  {274, 345}, {270, 300}, {266, 250}, {262, 195}, {258, 140}, {254, 95}
}, {pressure={0.85, 0.75, 0.6, 0.45, 0.3, 0.12}, ramps={0.02, 0.05}})

-- Stag horn 1 (Eastern dead bough)
b_branch:stroke({
  {270, 290}, {288, 245}, {310, 195}, {326, 145}, {338, 105}
}, {pressure={0.8, 0.65, 0.5, 0.3, 0.12}, ramps={0.02, 0.05}})

-- Stag horn 2 (Western dead bough)
b_branch:stroke({
  {268, 295}, {248, 250}, {226, 200}, {208, 150}, {194, 110}
}, {pressure={0.8, 0.65, 0.5, 0.3, 0.12}, ramps={0.02, 0.05}})

-- Bleached wood highlights on the sky-facing surfaces of dead stag boughs
b_twig:load(p_stag_bleach, 0.95)
b_twig:stroke({{272, 335}, {268, 295}, {264, 245}, {260, 192}, {256, 138}, {252, 95}}, {pressure={0.3, 0.6, 0.5, 0.35, 0.2, 0.08}})
b_twig:stroke({{270, 285}, {289, 242}, {311, 192}, {327, 142}, {339, 105}}, {pressure={0.25, 0.55, 0.45, 0.3, 0.08}})
b_twig:stroke({{266, 290}, {246, 248}, {225, 198}, {207, 148}, {193, 110}}, {pressure={0.25, 0.55, 0.45, 0.3, 0.08}})

-- Splintered tips on dead stag antlers
b_rigger:load(p_stag_bleach, 0.95)
b_rigger:stroke({{254, 98}, {252, 90}}, {pressure={0.4, 0.05}})
b_rigger:stroke({{254, 98}, {257, 92}}, {pressure={0.3, 0.05}})
b_rigger:stroke({{338, 107}, {342, 100}}, {pressure={0.35, 0.05}})
b_rigger:stroke({{194, 112}, {190, 104}}, {pressure={0.35, 0.05}})

-- 7. Fine Crooked Sympodial Twigs
local sympodial_twigs = {
  -- East
  {{428, 258}, {442, 238}, {455, 220}},
  {{428, 258}, {418, 240}, {410, 222}},
  {{515, 280}, {532, 265}, {548, 252}},
  {{552, 326}, {564, 340}, {574, 355}},
  {{492, 375}, {506, 390}, {518, 404}},
  {{368, 272}, {360, 255}, {352, 240}},
  -- West
  {{160, 248}, {146, 228}, {132, 210}},
  {{160, 248}, {172, 230}, {182, 214}},
  {{108, 300}, {90, 282}, {74, 268}},
  {{108, 300}, {96, 318}, {84, 336}},
  {{128, 368}, {112, 388}, {96, 406}}
}
for _, tw in ipairs(sympodial_twigs) do
  b_rigger:load(p_bark_solid, 0.85)
  b_rigger:stroke(tw, {pressure={0.65, 0.15}, ramps={0.05, 0.2}})
end

-- 8. Clinging Marcescent Autumn Leaves (*Laub*) along the living twigs
local marcescent_clusters = {
  -- East
  {435, 245, 18}, {450, 225, 16}, {415, 230, 14},
  {480, 295, 20}, {510, 275, 18}, {535, 262, 16},
  {540, 320, 18}, {560, 335, 16}, {480, 365, 18}, {500, 385, 16},
  {380, 325, 18}, {400, 300, 16}, {355, 355, 14},
  -- West
  {150, 235, 18}, {135, 218, 16}, {170, 222, 16},
  {100, 290, 18}, {80, 275, 16}, {88, 325, 16},
  {140, 335, 18}, {120, 375, 16}, {180, 325, 18}
}
for _, cl in ipairs(marcescent_clusters) do
  local cx, cy, n = cl[1], cl[2], cl[3]
  for i = 1, n do
    local ox = rand(-12, 12)
    local oy = rand(-9, 9)
    local p = (rand(0, 1) < 0.45) and p_leaf_russet or p_leaf_gold
    b_leaf:load(p, 0.7)
    b_leaf:touch(cx + ox, cy + oy, {pressure=rand(0.35, 0.65), drag={rand(1.2, 2.4), rand(1.1, 1.6)}})
  end
end

print("Chunk 83 complete: Veteran Oak painted with direct, organic, authentic brushwork.")

--@ chunk 84
-- Chunk 84: Letting the sky dry completely so dark body paint sits 100% opaque without lifting
local t = wait(2 * 24 * 60)
print("Time after drying:", t)
print("Drying state at sky (500, 250):", drying(500, 250))
print("Drying state at tree (270, 350):", drying(270, 350))

--@ chunk 85
-- Chunk 85: Painting the Master Friedrichian Ancient Oak, Companion Tree, and Foreground Details on Bone-Dry Ground

-- 1. Piles for Wood, Bark, Stag Wood, Foliage, Granite, and Heath
local p_bark_solid   = pile{{"bone black", 12}, {"raw umber", 8}, {"red earth", 2.0}, medium=0.15}
local p_bark_mid     = pile{{"raw umber", 8}, {"bone black", 5}, {"yellow ochre", 4.0}, {"lead white", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_bark_ridge   = pile{{"yellow ochre", 7}, {"lead white", 5.0}, {"raw umber", 4.0}, {"bone black", 1.2}, medium=0.2}
local p_hollow_pitch = pile{{"bone black", 14}, {"raw umber", 6}, medium=0.1}
local p_woundwood    = pile{{"raw umber", 6}, {"yellow ochre", 5.5}, {"lead white", 3.5}, {"red earth", 2.0}, medium=0.2}
local p_stag_shade   = pile{{"raw umber", 8}, {"bone black", 5}, {"lead white", 5}, {"yellow ochre", 2.5}, medium=0.2}
local p_stag_bleach  = pile{{"lead white", 18}, {"raw umber", 3.5}, {"yellow ochre", 2.5}, {"smalt", 1.0}, medium=0.22}
local p_peat_earth   = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, {"green earth", 2.5}, medium=0.15}

local p_leaf_dark    = pile{{"bone black", 8}, {"raw umber", 9}, {"red earth", 4.0}, medium=0.2}
local p_leaf_russet  = pile{{"yellow ochre", 7}, {"red earth", 6.5}, {"raw umber", 4.0}, {"lead white", 1.5}, medium=0.22}
local p_leaf_gold    = pile{{"yellow ochre", 9}, {"chrome yellow", 2.5}, {"lead white", 4.0}, {"vermilion", 0.4}, medium=0.25}

local p_granite_sh   = pile{{"raw umber", 8}, {"bone black", 6}, {"smalt", 2.2}, {"red earth", 1.2}, medium=0.18}
local p_granite_mid  = pile{{"lead white", 10}, {"raw umber", 6.5}, {"bone black", 3.0}, {"yellow ochre", 3.0}, {"smalt", 1.8}, {"red earth", 1.0}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 22}, {"smalt", 2.5}, {"yellow ochre", 2.8}, {"raw umber", 1.0}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage    = pile{{"lead white", 16}, {"green earth", 6.5}, {"yellow ochre", 3.5}, {"raw umber", 1.0}, medium=0.25}
local p_lich_gold    = pile{{"lead white", 12}, {"yellow ochre", 9.0}, {"chrome yellow", 1.8}, {"raw umber", 0.8}, medium=0.25}

local p_sand_path    = pile{{"yellow ochre", 7}, {"lead white", 5}, {"raw umber", 4}, {"bone black", 1}, medium=0.2}
local p_sand_light   = pile{{"lead white", 12}, {"yellow ochre", 7}, {"raw umber", 2.5}, {"vermilion", 0.3}, medium=0.22}
local p_heather_bank = pile{{"raw umber", 9}, {"bone black", 6}, {"red earth", 4.0}, {"yellow ochre", 2.5}, medium=0.18}
local p_straw_sharp  = pile{{"lead white", 16}, {"yellow ochre", 9}, {"chrome yellow", 1.2}, {"raw umber", 1.0}, medium=0.22}
local p_straw_shade  = pile{{"yellow ochre", 7}, {"raw umber", 6}, {"bone black", 2.5}, {"lead white", 2.0}, medium=0.2}
local p_twilight_rim = pile{{"lead white", 24}, {"yellow ochre", 3.5}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.2}

local b_bole   = brush{kind="filbert", width=8.5, stiffness=0.9}
local b_limb   = brush{kind="filbert", width=5.5, stiffness=0.85}
local b_branch = brush{kind="round", width=2.8, point=1, stiffness=0.85}
local b_twig   = brush{kind="round", width=1.5, point=1, stiffness=0.85}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}
local b_leaf   = brush{kind="round", width=1.8, point=1, stiffness=0.8}

-- 2. Master Ancient Oak: Solid Trunk & Flaring Root Buttresses
-- Solid opaque multi-stroke trunk build-up (width ~60 units)
b_bole:load(p_bark_solid, 0.95)
-- Root flares
b_bole:stroke({{225, 458}, {242, 448}, {258, 435}, {266, 410}, {270, 380}}, {pressure={0.95, 0.95, 0.9, 0.85, 0.8}})
b_bole:stroke({{262, 460}, {268, 445}, {272, 420}, {275, 385}, {274, 360}}, {pressure={0.95, 0.95, 0.9, 0.85, 0.8}})
b_bole:stroke({{325, 452}, {308, 445}, {295, 430}, {286, 400}, {280, 365}}, {pressure={0.95, 0.9, 0.85, 0.8, 0.8}})
-- Trunk solid core
b_bole:stroke({{250, 448}, {256, 420}, {262, 385}, {266, 355}}, {pressure=0.95})
b_bole:stroke({{274, 448}, {276, 420}, {276, 385}, {274, 355}}, {pressure=0.95})
b_bole:stroke({{294, 448}, {292, 420}, {286, 385}, {280, 355}}, {pressure=0.95})

-- Anchor roots into dark peaty barrow soil
b_bole:load(p_peat_earth, 0.95)
b_bole:stroke({{210, 462}, {250, 458}, {290, 456}, {335, 452}}, {pressure=0.95})

-- Bark furrows and raised ridges
b_twig:load(p_hollow_pitch, 0.95)
b_twig:stroke({{248, 450}, {256, 430}, {260, 405}, {262, 370}}, {pressure={0.85, 0.65, 0.3}})
b_twig:stroke({{268, 452}, {271, 432}, {273, 406}, {270, 368}}, {pressure={0.85, 0.7, 0.35}})
b_twig:stroke({{292, 448}, {290, 428}, {285, 402}, {280, 370}}, {pressure={0.85, 0.65, 0.35}})

b_twig:load(p_bark_ridge, 0.85)
b_twig:stroke({{240, 454}, {250, 440}, {256, 418}, {258, 385}}, {pressure={0.3, 0.75, 0.6, 0.2}})
b_twig:stroke({{262, 454}, {266, 435}, {268, 410}, {268, 375}}, {pressure={0.3, 0.7, 0.6, 0.2}})
b_twig:stroke({{278, 454}, {278, 435}, {278, 405}, {275, 370}}, {pressure={0.3, 0.65, 0.6, 0.2}})
b_twig:stroke({{302, 450}, {298, 432}, {290, 408}, {284, 375}}, {pressure={0.3, 0.7, 0.6, 0.2}})

-- Hollow cavity
b_twig:load(p_hollow_pitch, 0.95)
b_twig:stroke({{272, 428}, {275, 410}, {272, 396}}, {pressure=0.85})
b_twig:load(p_woundwood, 0.85)
b_twig:stroke({{269, 430}, {270, 412}, {276, 395}}, {pressure={0.5, 0.85, 0.4}})
b_twig:stroke({{278, 426}, {279, 405}, {275, 395}}, {pressure={0.5, 0.85, 0.4}})

-- 3. Level 1 & 2 Muscular Boughs (Thick, solid, continuous strokes)
-- Great Eastern Living Bough arching over the Dolmen
b_limb:load(p_bark_solid, 0.95)
b_limb:stroke({
  {278, 368}, {320, 356}, {365, 344}, {415, 332}, {465, 318}, {510, 302}
}, {pressure={0.95, 0.9, 0.8, 0.7, 0.55, 0.35}, ramps={0.02, 0.05}})
-- Under-shadow and top-ridge modeling
b_branch:load(p_hollow_pitch, 0.9)
b_branch:stroke({{280, 374}, {325, 362}, {370, 350}, {420, 338}, {470, 324}}, {pressure={0.8, 0.6, 0.4, 0.2}})
b_branch:load(p_bark_mid, 0.85)
b_branch:stroke({{282, 362}, {322, 350}, {368, 338}, {418, 326}, {466, 312}}, {pressure={0.4, 0.7, 0.6, 0.3}})

-- Eastern secondary boughs
-- E1: Upward bough
b_branch:load(p_bark_solid, 0.95)
b_branch:stroke({{365, 344}, {385, 318}, {408, 288}, {428, 258}}, {pressure={0.9, 0.75, 0.6, 0.3}, ramps={0.02, 0.1}})
-- E2: Sheltering bough over dolmen capstone
b_branch:stroke({{415, 332}, {440, 350}, {468, 364}, {492, 375}}, {pressure={0.85, 0.7, 0.55, 0.25}, ramps={0.02, 0.1}})
-- E3: Outer boughs
b_branch:stroke({{465, 318}, {490, 298}, {515, 280}}, {pressure={0.8, 0.6, 0.25}, ramps={0.02, 0.1}})
b_branch:stroke({{510, 302}, {532, 315}, {552, 326}}, {pressure={0.75, 0.55, 0.25}, ramps={0.02, 0.1}})

-- Living Western Bough
b_limb:load(p_bark_solid, 0.95)
b_limb:stroke({
  {268, 365}, {230, 350}, {190, 334}, {150, 318}, {108, 300}
}, {pressure={0.95, 0.85, 0.7, 0.55, 0.3}, ramps={0.02, 0.05}})
b_branch:stroke({{210, 342}, {194, 312}, {178, 280}, {160, 248}}, {pressure={0.85, 0.7, 0.55, 0.3}, ramps={0.02, 0.1}})
b_branch:stroke({{170, 326}, {150, 348}, {128, 368}}, {pressure={0.8, 0.6, 0.25}, ramps={0.02, 0.1}})

-- 4. Bleached Stag-Head Crown Aloft (Stark dead boughs in the zenith)
b_branch:load(p_stag_shade, 0.95)
b_branch:stroke({{274, 345}, {270, 300}, {266, 250}, {262, 195}, {258, 140}, {254, 95}}, {pressure={0.9, 0.8, 0.65, 0.5, 0.35, 0.15}, ramps={0.02, 0.05}})
b_branch:stroke({{270, 290}, {288, 245}, {310, 195}, {326, 145}, {338, 105}}, {pressure={0.85, 0.7, 0.55, 0.35, 0.15}, ramps={0.02, 0.05}})
b_branch:stroke({{268, 295}, {248, 250}, {226, 200}, {208, 150}, {194, 110}}, {pressure={0.85, 0.7, 0.55, 0.35, 0.15}, ramps={0.02, 0.05}})

-- Bleached wood highlights & splintered tips
b_twig:load(p_stag_bleach, 0.95)
b_twig:stroke({{272, 335}, {268, 295}, {264, 245}, {260, 192}, {256, 138}, {252, 95}}, {pressure={0.35, 0.7, 0.6, 0.45, 0.25, 0.1}})
b_twig:stroke({{270, 285}, {289, 242}, {311, 192}, {327, 142}, {339, 105}}, {pressure={0.3, 0.65, 0.55, 0.4, 0.1}})
b_twig:stroke({{266, 290}, {246, 248}, {225, 198}, {207, 148}, {193, 110}}, {pressure={0.3, 0.65, 0.55, 0.4, 0.1}})
b_rigger:load(p_stag_bleach, 0.95)
b_rigger:stroke({{254, 98}, {252, 88}}, {pressure={0.5, 0.05}})
b_rigger:stroke({{254, 98}, {257, 90}}, {pressure={0.4, 0.05}})
b_rigger:stroke({{338, 107}, {342, 98}}, {pressure={0.45, 0.05}})
b_rigger:stroke({{194, 112}, {190, 102}}, {pressure={0.45, 0.05}})

-- 5. Level 3, 4, 5 Crooked Quercus Branchlets & Twigs
local oak_branches = {
  -- East
  {{428, 258}, {446, 238}, {462, 218}},
  {{428, 258}, {416, 238}, {406, 218}},
  {{408, 288}, {395, 270}, {386, 250}},
  {{385, 318}, {374, 298}, {366, 276}},
  {{515, 280}, {535, 264}, {554, 250}},
  {{552, 326}, {566, 342}, {578, 358}},
  {{492, 375}, {508, 392}, {522, 408}},
  {{440, 350}, {430, 370}, {420, 390}},
  -- West
  {{160, 248}, {144, 226}, {128, 206}},
  {{160, 248}, {174, 228}, {186, 210}},
  {{178, 280}, {165, 258}, {152, 238}},
  {{194, 312}, {208, 292}, {218, 270}},
  {{108, 300}, {88, 282}, {70, 266}},
  {{108, 300}, {96, 320}, {82, 338}},
  {{128, 368}, {110, 390}, {92, 410}}
}
for _, b in ipairs(oak_branches) do
  b_twig:load(p_bark_solid, 0.9)
  b_twig:stroke(b, {pressure={0.75, 0.25}, ramps={0.02, 0.15}})
end

-- Fine sympodial twigs
local fine_twigs = {
  {{462, 218}, {474, 206}, {484, 195}},
  {{406, 218}, {398, 206}, {390, 194}},
  {{554, 250}, {566, 242}, {576, 235}},
  {{128, 206}, {118, 194}, {110, 182}},
  {{70, 266}, {58, 256}, {46, 248}}
}
for _, tw in ipairs(fine_twigs) do
  b_rigger:load(p_bark_solid, 0.85)
  b_rigger:stroke(tw, {pressure={0.65, 0.12}, ramps={0.02, 0.15}})
end

-- 6. Clinging Marcescent Autumn Leaves (*Laub*) along the living twigs
local marcescent_clusters = {
  -- East
  {440, 240, 22}, {460, 220, 20}, {412, 225, 18}, {480, 290, 22},
  {515, 270, 20}, {545, 255, 18}, {550, 315, 20}, {570, 335, 18},
  {485, 360, 20}, {510, 385, 18}, {385, 320, 20}, {405, 295, 18},
  {360, 350, 16},
  -- West
  {150, 230, 22}, {132, 212, 20}, {172, 216, 18}, {100, 285, 22},
  {78, 270, 18}, {88, 320, 18}, {140, 330, 20}, {118, 370, 18},
  {180, 320, 20}
}
for _, cl in ipairs(marcescent_clusters) do
  local cx, cy, n = cl[1], cl[2], cl[3]
  for i = 1, n do
    local ox = rand(-14, 14)
    local oy = rand(-10, 10)
    local roll = rand(0, 1)
    local p = (roll < 0.35) and p_leaf_dark or ((roll < 0.75) and p_leaf_russet or p_leaf_gold)
    b_leaf:load(p, 0.75)
    b_leaf:touch(cx + ox, cy + oy, {pressure=rand(0.35, 0.7), drag={rand(1.2, 2.5), rand(1.1, 1.6)}})
  end
end

-- 7. The Companion Tree on the Right Knoll (x ≈ 750 to 800)
b_branch:load(p_bark_solid, 0.95)
b_branch:stroke({
  {766, 442}, {762, 405}, {756, 365}, {750, 325}, {746, 285}, {744, 252}
}, {pressure={0.9, 0.8, 0.65, 0.5, 0.35, 0.15}, ramps={0.02, 0.05}})

local comp_branches = {
  {{750, 325}, {774, 308}, {802, 290}, {828, 275}, {852, 262}},
  {{802, 290}, {818, 268}, {832, 245}, {842, 222}},
  {{746, 285}, {762, 260}, {776, 235}, {788, 210}},
  {{744, 252}, {754, 228}, {764, 202}, {772, 178}}
}
for _, b in ipairs(comp_branches) do
  b_twig:load(p_bark_solid, 0.9)
  b_twig:stroke(b, {pressure={0.75, 0.2}, ramps={0.02, 0.1}})
end

local comp_twigs = {
  {{852, 262}, {868, 254}, {882, 248}},
  {{842, 222}, {854, 206}, {864, 192}},
  {{788, 210}, {800, 194}, {810, 178}}
}
for _, tw in ipairs(comp_twigs) do
  b_rigger:load(p_bark_solid, 0.85)
  b_rigger:stroke(tw, {pressure={0.65, 0.12}, ramps={0.02, 0.15}})
end

local comp_leaf_sprays = {
  {810, 300, 12}, {840, 280, 14}, {865, 265, 12}, {828, 250, 12},
  {775, 240, 12}, {785, 210, 10}
}
for _, cl in ipairs(comp_leaf_sprays) do
  local cx, cy, n = cl[1], cl[2], cl[3]
  for i = 1, n do
    local ox = rand(-10, 10)
    local oy = rand(-8, 8)
    local p = (rand(0, 1) < 0.5) and p_leaf_russet or p_leaf_gold
    b_leaf:load(p, 0.7)
    b_leaf:touch(cx + ox, cy + oy, {pressure=rand(0.3, 0.65), drag={rand(1.0, 2.0), 1.4}})
  end
end

-- 8. Foreground Glacial Erratic Boulders (*Findlinge*)
-- Left Boulder (x ≈ 210 to 275, y ≈ 525 to 555)
local pts_b1 = {{210, 545}, {224, 526}, {258, 522}, {276, 532}, {274, 552}, {245, 558}, {214, 554}}
local m_b1 = poly(pts_b1, false)
work(m_b1, {hand="body", pile=p_granite_mid, angle=0.2, coverage=1.8, fill=true, clip=true})
local m_b1_sh = poly({{210, 545}, {235, 552}, {274, 552}, {245, 558}, {214, 554}}, false)
work(m_b1_sh, {hand="detail", pile=p_granite_sh, angle=0.2, coverage=1.6, fill=true, clip=true})
local m_b1_lit = poly({{224, 526}, {258, 522}, {276, 532}, {260, 536}, {230, 534}}, false)
work(m_b1_lit, {hand="detail", pile=p_granite_lit, angle=0.05, coverage=1.6, fill=true, clip=true})
b_twig:load(p_granite_sh, 0.85)
b_twig:stroke({{242, 524}, {246, 536}, {244, 554}}, {pressure={0.5, 0.7, 0.2}})
b_twig:load(p_lich_sage, 0.8)
b_twig:touch(238, 528, {pressure=0.6, drag={2.0, 0.1}})
b_twig:load(p_lich_gold, 0.75)
b_twig:touch(258, 528, {pressure=0.55, drag={1.8, 0.1}})

-- Right Boulder (x ≈ 650 to 715, y ≈ 535 to 565)
local pts_b2 = {{652, 554}, {666, 536}, {698, 532}, {714, 544}, {712, 562}, {680, 566}, {655, 562}}
local m_b2 = poly(pts_b2, false)
work(m_b2, {hand="body", pile=p_granite_mid, angle=-0.2, coverage=1.8, fill=true, clip=true})
local m_b2_sh = poly({{652, 554}, {675, 560}, {712, 562}, {680, 566}, {655, 562}}, false)
work(m_b2_sh, {hand="detail", pile=p_granite_sh, angle=-0.2, coverage=1.6, fill=true, clip=true})
local m_b2_lit = poly({{666, 536}, {698, 532}, {714, 544}, {695, 546}, {672, 544}}, false)
work(m_b2_lit, {hand="detail", pile=p_granite_lit, angle=-0.05, coverage=1.6, fill=true, clip=true})
b_twig:load(p_granite_sh, 0.85)
b_twig:stroke({{682, 534}, {684, 546}, {682, 562}}, {pressure={0.5, 0.7, 0.2}})
b_twig:load(p_lich_sage, 0.8)
b_twig:touch(678, 538, {pressure=0.6, drag={2.0, 0.1}})
b_twig:load(p_lich_gold, 0.75)
b_twig:touch(692, 536, {pressure=0.55, drag={1.8, 0.1}})

-- Bed boulders into heather
b_bole:load(p_heather_bank, 0.95)
b_bole:stroke({{198, 556}, {230, 562}, {265, 560}, {295, 552}}, {pressure=0.9})
b_bole:stroke({{640, 564}, {675, 570}, {710, 568}, {735, 558}}, {pressure=0.9})

-- 9. Subtle Pilgrim Path through the Heath
local pts_path = {{710, 705}, {675, 650}, {632, 595}, {585, 545}, {538, 502}, {488, 470}, {445, 452}}
local m_path   = ribbon(pts_path, {20, 16, 13, 10, 7, 5, 3}):soften(3)
work(m_path, {hand="scumble", pile=p_sand_path, angle=0.4, coverage=1.2, fill=true})
local m_path_lit = ribbon(pts_path, {10, 8, 6, 5, 3.5, 2.5, 1.5}):soften(2)
work(m_path_lit, {hand="scumble", pile=p_sand_light, angle=0.35, coverage=1.0, fill=true})

-- 10. Windswept Bent-Grass Tufts & Carline Thistles
local function draw_curved_grass(bx, by, len, ang, bend, p)
  local mx = bx + len * 0.55 * math.cos(ang + bend * 0.5)
  local my = by - len * 0.55 * math.sin(ang + bend * 0.5)
  local tx = bx + len * math.cos(ang + bend)
  local ty = by - len * math.sin(ang + bend)
  b_rigger:load(p, 0.75)
  b_rigger:stroke({{bx, by}, {mx, my}, {tx, ty}}, {pressure={0.7, 0.05}, ramps={0.05, 0.3}})
end

local function draw_grass_cluster(cx, cy, n, h, lean)
  for i = 1, n do
    local bx = cx + rand(-5, 5)
    local by = cy + rand(-2, 2)
    local len = h * rand(0.75, 1.25)
    local ang = 1.57 + rand(-0.25, 0.25) + lean * 0.3
    local bnd = lean * rand(0.2, 0.5)
    local p = (rand(0, 1) < 0.65) and p_straw_sharp or p_straw_shade
    draw_curved_grass(bx, by, len, ang, bnd, p)
  end
end

-- Grass clusters around boulders and path
local grasses = {
  {200, 554, 7, 16, -0.4}, {225, 558, 6, 15, -0.2}, {268, 556, 7, 16, 0.3},
  {645, 562, 7, 16, -0.4}, {675, 568, 6, 15, -0.1}, {708, 566, 7, 15, 0.3},
  {700, 685, 6, 14, 0.3}, {665, 640, 6, 13, -0.3}, {620, 585, 5, 12, 0.2},
  {215, 464, 8, 18, -0.4}, {240, 462, 8, 17, -0.2}, {320, 456, 7, 16, 0.3}
}
for _, g in ipairs(grasses) do draw_grass_cluster(g[1], g[2], g[3], g[4], g[5]) end

-- Carline thistles along ridge crest
local thistles = {{145, 460, 20}, {575, 454, 18}, {855, 450, 18}}
for _, th in ipairs(thistles) do
  local x, y, h = th[1], th[2], th[3]
  b_rigger:load(p_bark_solid, 0.85)
  b_rigger:stroke({{x, y}, {x, y - h*0.6}, {x + rand(-2, 2), y - h}}, {pressure={0.7, 0.15}, ramps={0.05, 0.2}})
  b_twig:load(p_straw_sharp, 0.8)
  b_twig:stroke({{x - 3, y - h}, {x + 3, y - h}}, {pressure=0.4})
end

-- 11. Subtle Twilight Rim-Lights
b_rigger:load(p_twilight_rim, 0.65)
b_rigger:stroke({{318, 408}, {340, 396}, {380, 391}, {420, 392}, {455, 396}, {474, 403}}, {pressure={0.4, 0.7, 0.6, 0.2}})
b_rigger:stroke({{230, 456}, {242, 440}, {252, 420}, {258, 390}, {262, 360}}, {pressure={0.4, 0.6, 0.5, 0.2}})
b_rigger:stroke({{766, 442}, {762, 410}, {758, 375}, {752, 340}, {746, 305}}, {pressure={0.35, 0.5, 0.4, 0.15}})

print("Chunk 85 complete: Masterpiece elements executed on dry ground.")

--@ chunk 86
-- Chunk 86: The Majestic Oak Canopy, Windswept Companion Tree, and Fusing Foreground Heathland

-- 1. Piles for Deep Foliage, Autumn Leaves, Wood, and Heath
local p_fol_deep    = pile{{"bone black", 11}, {"raw umber", 10}, {"red earth", 3.5}, medium=0.16}
local p_fol_russet  = pile{{"yellow ochre", 7}, {"red earth", 6.0}, {"raw umber", 4.0}, {"lead white", 1.8}, {"bone black", 0.8}, medium=0.2}
local p_fol_gold    = pile{{"yellow ochre", 9}, {"chrome yellow", 2.2}, {"lead white", 4.5}, {"red earth", 1.5}, {"vermilion", 0.3}, medium=0.22}
local p_bark_core   = pile{{"bone black", 12}, {"raw umber", 8}, {"red earth", 2.0}, medium=0.15}
local p_stag_bleach = pile{{"lead white", 18}, {"raw umber", 3.5}, {"yellow ochre", 2.5}, {"smalt", 1.0}, medium=0.22}

-- 2. Blend the foreground wet-into-wet to dissolve the yellow popcorn path and soften boulder edges
local m_ground = below(barrow_crest)
blend(m_ground, {angle=0.2})
blend(m_ground, {angle=-0.15})
blend(m_ground, {angle=0.1})

-- 3. The Continuous, Billowing Autumn Canopy of the Ancient Oak
-- A single, majestic, windswept canopy sheltering over the megalithic tomb
local o_oak_crown = outline{
  {240, 360}, {180, 335}, {130, 290}, {155, 240}, {210, 215},
  {280, 195}, {350, 190}, {420, 205}, {480, 230}, {535, 270},
  {555, 320}, {525, 360}, {460, 375}, {390, 365}, {320, 368},
  char = "soft",
  lobe = 20,
  amount = 0.45,
  closed = true
}
local m_oak_crown = o_oak_crown:mask()

-- Layer A: Deep shadow interior mass (covers the crane sticks completely!)
work(m_oak_crown, {
  hand = "body",
  pile = p_fol_deep,
  angle = 0.25,
  length = {35, 75},
  coverage = 2.0,
  fill = true,
  clip = false -- natural broken bristle contour!
})

-- Layer B: Warm late-autumn russet body leaves
stipple(m_oak_crown, {
  pile = p_fol_russet,
  width = 2.4,
  coverage = 1.3,
  cluster = {0.85, 9},
  drag = {2.2, 1.3},
  feather = 0.55
})

-- Layer C: Twilight-lit golden-amber leaf edges catching the evening afterglow
stipple(m_oak_crown, {
  pile = p_fol_gold,
  width = 1.8,
  coverage = 0.8,
  cluster = {0.9, 7},
  drag = {1.6, 1.2},
  feather = 0.65
})

-- 4. Muscular Limbs & Crooked Branches Weaving Through the Canopy
local b_limb   = brush{kind="filbert", width=5.0, stiffness=0.85}
local b_branch = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_rigger = brush{kind="round", width=0.9, point=1, stiffness=0.9}

-- Eastern bough main stem emerging from the trunk and arching through the leaves
b_limb:load(p_bark_core, 0.95)
b_limb:stroke({
  {278, 368}, {320, 356}, {365, 344}, {415, 332}, {465, 318}, {510, 302}
}, {pressure={0.9, 0.85, 0.75, 0.6, 0.45, 0.25}, ramps={0.02, 0.05}})

-- Western bough main stem
b_limb:stroke({
  {268, 365}, {230, 350}, {190, 334}, {150, 318}, {112, 300}
}, {pressure={0.9, 0.8, 0.65, 0.5, 0.25}, ramps={0.02, 0.05}})

-- Crooked branches protruding past the foliage into the open sky
local outer_oak_twigs = {
  -- East (reaching over dolmen)
  {{535, 270}, {555, 255}, {572, 242}},
  {{555, 320}, {575, 335}, {592, 350}},
  {{525, 360}, {542, 378}, {556, 395}},
  {{460, 375}, {472, 395}, {482, 412}},
  {{420, 205}, {435, 185}, {446, 168}},
  {{350, 190}, {358, 168}, {364, 148}},
  -- West
  {{155, 240}, {142, 220}, {128, 202}},
  {{130, 290}, {110, 275}, {92, 260}},
  {{180, 335}, {165, 355}, {148, 375}}
}
for _, tw in ipairs(outer_oak_twigs) do
  b_branch:load(p_bark_core, 0.9)
  b_branch:stroke(tw, {pressure={0.7, 0.2}, ramps={0.02, 0.15}})
end

-- 5. The Bleached Stag-Head Crown Aloft (Spearing proudly above the living canopy)
b_branch:load(p_stag_bleach, 0.95)
b_branch:stroke({{274, 210}, {270, 175}, {265, 135}, {258, 95}, {252, 60}}, {pressure={0.8, 0.65, 0.5, 0.3, 0.1}, ramps={0.02, 0.05}})
b_branch:stroke({{270, 195}, {288, 165}, {308, 130}, {324, 98}, {336, 70}}, {pressure={0.75, 0.6, 0.45, 0.25, 0.1}, ramps={0.02, 0.05}})
b_branch:stroke({{268, 198}, {248, 168}, {226, 135}, {208, 102}, {194, 72}}, {pressure={0.75, 0.6, 0.45, 0.25, 0.1}, ramps={0.02, 0.05}})

-- Splintered tips on dead stag antlers
b_rigger:load(p_stag_bleach, 0.95)
b_rigger:stroke({{252, 63}, {250, 52}}, {pressure={0.4, 0.05}})
b_rigger:stroke({{252, 63}, {255, 55}}, {pressure={0.3, 0.05}})
b_rigger:stroke({{336, 72}, {340, 62}}, {pressure={0.35, 0.05}})
b_rigger:stroke({{194, 74}, {190, 64}}, {pressure={0.35, 0.05}})

-- 6. The Windswept Companion Tree on the Right Knoll (x ≈ 740 to 860)
local o_comp_crown = outline{
  {760, 360}, {735, 320}, {740, 270}, {765, 230}, {805, 205},
  {845, 220}, {875, 255}, {885, 300}, {860, 340}, {810, 360},
  char = "soft",
  lobe = 16,
  amount = 0.4,
  closed = true
}
local m_comp_crown = o_comp_crown:mask()

work(m_comp_crown, {hand="body", pile=p_fol_deep, angle=0.2, coverage=1.8, fill=true, clip=false})
stipple(m_comp_crown, {pile=p_fol_russet, width=2.2, coverage=1.2, cluster={0.85, 8}, drag={2.0, 1.3}, feather=0.55})
stipple(m_comp_crown, {pile=p_fol_gold, width=1.6, coverage=0.75, cluster={0.9, 6}, drag={1.5, 1.2}, feather=0.65})

-- Companion tree trunk and wind-swept branches
b_limb:load(p_bark_core, 0.95)
b_limb:stroke({{766, 442}, {762, 405}, {756, 365}, {750, 325}, {746, 285}, {744, 252}}, {pressure={0.85, 0.7, 0.55, 0.4, 0.25, 0.1}, ramps={0.02, 0.05}})

local comp_twigs = {
  {{875, 255}, {895, 245}, {912, 238}},
  {{885, 300}, {902, 312}, {918, 325}},
  {{845, 220}, {858, 202}, {868, 185}}
}
for _, tw in ipairs(comp_twigs) do
  b_rigger:load(p_bark_core, 0.85)
  b_rigger:stroke(tw, {pressure={0.65, 0.12}, ramps={0.02, 0.15}})
end

print("Chunk 86 complete: Billowing oak canopy and companion crown sculpted, foreground fused.")

--@ chunk 87
-- Chunk 87: Letting the paint dry completely
local t = wait(3 * 24 * 60)
print("Time after drying:", t)
print("Drying state at (300, 300):", drying(300, 300))
print("Drying state at sky (500, 200):", drying(500, 200))

--@ chunk 88
-- Chunk 88: Restoring the Sublime Friedrichian Twilight Sky across Full Width (y = 0 to 425)

-- 1. Sky Piles
local p_sky_zenith = pile{{"lead white", 18}, {"cobalt blue", 6.2}, {"smalt", 4.5}, {"raw umber", 0.6}, {"bone black", 0.25}, medium=0.22}
local p_sky_upper  = pile{{"lead white", 26}, {"cobalt blue", 3.2}, {"smalt", 3.2}, {"yellow ochre", 0.8}, {"raw umber", 0.3}, medium=0.22}
local p_sky_mid    = pile{{"lead white", 28}, {"yellow ochre", 3.5}, {"chrome yellow", 0.9}, {"smalt", 1.0}, {"raw umber", 0.2}, {"vermilion", 0.1}, medium=0.22}
local p_sky_glow   = pile{{"lead white", 26}, {"yellow ochre", 5.2}, {"chrome yellow", 2.2}, {"vermilion", 0.5}, {"raw umber", 0.25}, medium=0.22}

-- 2. Four overlapping bands covering x = 0 to 1000 from zenith down behind the barrow (y = 425)
local m1 = rect(0, 0, 1000, 145):soften(30)
work(m1, {hand="body", pile=p_sky_zenith, angle=0, order="across", coverage=2.2, fill=true})

local m2 = rect(0, 105, 1000, 150):soften(30)
work(m2, {hand="body", pile=p_sky_upper, angle=0, order="across", coverage=2.2, fill=true})

local m3 = rect(0, 215, 1000, 140):soften(30)
work(m3, {hand="body", pile=p_sky_mid, angle=0, order="across", coverage=2.2, fill=true})

local m4 = rect(0, 315, 1000, 125):soften(25)
work(m4, {hand="body", pile=p_sky_glow, angle=0, order="across", coverage=2.2, fill=true})

-- 3. Seamless badger blending across the full canvas width
local m_sky_all = rect(0, 0, 1000, 425)
blend(m_sky_all, {angle=1.57})
blend(m_sky_all, {angle=0})

-- 4. Distant low Baltic hills silhouetted in soft atmospheric haze
local p_dist_coast = pile{{"lead white", 15}, {"smalt", 5.0}, {"raw umber", 3.0}, {"red earth", 0.9}, {"bone black", 0.4}, medium=0.25}
local pts_distant_land = {
  {0, 464}, {100, 460}, {220, 454}, {340, 452}, {460, 448},
  {580, 451}, {700, 447}, {820, 452}, {920, 458}, {1000, 464},
  {1000, 475}, {0, 475}
}
local m_coast = poly(pts_distant_land, true):soften(2.5)
work(m_coast, {hand="body", pile=p_dist_coast, angle=0, coverage=1.2, fill=true, clip=true})

-- 5. Calm water sheen and valley mist pooling over the water
local p_water_mist = pile{{"lead white", 26}, {"yellow ochre", 3.2}, {"smalt", 1.8}, {"raw umber", 0.3}, medium=0.3}
local m_water = rect(0, 435, 1000, 30):soften(4)
work(m_water, {hand="glaze", pile=p_water_mist, angle=0, coverage=1.1, fill=false})
blend(rect(0, 432, 1000, 34), {angle=0})

-- 6. Waxing crescent moon and solitary evening star (Venus)
local p_moon_pure = pile{{"lead white", 34}, {"yellow ochre", 1.0}, {"chrome yellow", 0.4}, medium=0.12}
local b_rigger = brush{kind="round", width=1.3, point=1, stiffness=0.85}
b_rigger:load(p_moon_pure, 0.95)
b_rigger:stroke({
  {645.2, 105.5}, {641.5, 108.0}, {639.0, 112.5}, {640.2, 117.0}, {644.0, 119.5}
}, {pressure={0.08, 0.85, 1.0, 0.8, 0.08}, ramps={0.05, 0.05}})

local b_star = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_star:load(p_moon_pure, 1.0)
b_star:touch(688, 148, {pressure=0.6})

print("Chunk 88 complete: Master sky restored seamlessly; hedgehogs obliterated.")

--@ chunk 89
-- Chunk 90: Sculpting the Majestic Swelling Barrow Mound across Full Lower Canvas

-- 1. Mathematical spline for the noble prehistoric barrow crest
local barrow_nodes = {
  {x=-20, y=465}, {x=80, y=458}, {x=180, y=448}, {x=260, y=430},
  {x=330, y=415}, {x=385, y=408}, {x=445, y=414}, {x=520, y=430},
  {x=600, y=448}, {x=680, y=450}, {x=760, y=436}, {x=830, y=436},
  {x=910, y=448}, {x=1020, y=465}
}

local function barrow_crest(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 440
end

local m_barrow = below(barrow_crest)

-- 2. Earth, Peat, and Heathland Palette
local p_earth_core   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4}, {"yellow ochre", 3}, medium=0.18}
local p_peat_dark    = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.0}, medium=0.15}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5}, {"lead white", 2}, {"bone black", 1}, medium=0.2}
local p_peat_moss    = pile{{"green earth", 6}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}
local p_heath_gold   = pile{{"yellow ochre", 8}, {"raw umber", 4}, {"lead white", 3.5}, {"red earth", 2}, medium=0.2}

-- Solid foundation covering the entire lower canvas down to y = 704 (buries old yellow spray)
work(m_barrow, {
  hand = "body",
  pile = p_earth_core,
  angle = 0.22,
  length = {50, 110},
  coverage = 2.4,
  fill = true,
  clip = true
})

-- Sculpt the undulating topography with banks of heather, peat, and turf
-- Summit heather bank around where the dolmen will stand
local m_summit = (ellipse(385, 435, 170, 32) + ellipse(250, 448, 140, 35)):soften(15) * m_barrow
work(m_summit, {hand="body", pile=p_heath_russet, angle=0.18, coverage=1.4, fill=true})

-- Right knoll bank
local m_rknoll = ellipse(770, 455, 160, 35):soften(15) * m_barrow
work(m_rknoll, {hand="body", pile=p_heath_russet, angle=-0.15, coverage=1.3, fill=true})

-- Mid-slope peaty moss hollow
local m_hollow = (ellipse(520, 520, 200, 40) + ellipse(180, 530, 150, 40)):soften(18) * m_barrow
work(m_hollow, {hand="body", pile=p_peat_moss, angle=0.25, coverage=1.2, fill=true})

-- Lower foreground warm heather ridges
local m_low_heather = (ellipse(350, 600, 260, 50) + ellipse(780, 590, 220, 45)):soften(20) * m_barrow
work(m_low_heather, {hand="body", pile=p_heath_gold, angle=0.2, coverage=1.3, fill=true})

-- Deep peaty base at the bottom margin
local m_bottom = rect(0, 640, 1000, 70):soften(15) * m_barrow
work(m_bottom, {hand="body", pile=p_earth_core, angle=0.1, coverage=1.4, fill=true})

-- Blend the interior of the knoll to fuse the heather into smooth, sculptural earth
blend(m_barrow, {angle=0.2})
blend(m_barrow, {angle=-0.15})

-- Firm, continuous, organic stroke along the crest curve against the glowing mist
local b_crest = brush{kind="round", width=2.8, point=1, stiffness=0.9}
b_crest:load(p_peat_dark, 0.95)
local crest_line = {}
for x = -10, 1010, 10 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest:stroke(crest_line, {pressure=0.9, ramps={0.01, 0.01}})

print("Chunk 90 complete: Majestic swelling barrow mound sculpted across full lower canvas.")

--@ chunk 90
-- Chunk 91: The Monumental Megalithic Tomb (Hünengrab), Ridge Thistles & Bent-Grass

-- 1. Smooth the barrow crest contour with a firm, continuous stroke
local p_peat_crest = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.0}, medium=0.15}
local b_crest = brush{kind="round", width=2.8, point=1, stiffness=0.9}
b_crest:load(p_peat_crest, 0.95)

local barrow_nodes = {
  {x=-20, y=465}, {x=80, y=458}, {x=180, y=448}, {x=260, y=430},
  {x=330, y=415}, {x=385, y=408}, {x=445, y=414}, {x=520, y=430},
  {x=600, y=448}, {x=680, y=450}, {x=760, y=436}, {x=830, y=436},
  {x=910, y=448}, {x=1020, y=465}
}

local function barrow_crest(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 440
end

local crest_line = {}
for x = -10, 1010, 10 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest:stroke(crest_line, {pressure=0.9, ramps={0.01, 0.01}})

-- 2. Piles for Ancient Swedish Granite (weathered Baltic erratic)
local p_cavern_void   = pile{{"bone black", 12}, {"raw umber", 8}, medium=0.1}
local p_granite_deep  = pile{{"bone black", 8}, {"raw umber", 7}, {"smalt", 2.0}, {"red earth", 1.5}, medium=0.15}
local p_granite_shade = pile{{"raw umber", 8}, {"bone black", 5}, {"smalt", 2.2}, {"lead white", 2.5}, {"red earth", 1.2}, {"yellow ochre", 1.5}, medium=0.18}
local p_granite_body  = pile{{"lead white", 10}, {"raw umber", 6}, {"bone black", 3}, {"yellow ochre", 3.2}, {"smalt", 1.8}, {"red earth", 1.2}, medium=0.2}
local p_granite_facet = pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.22}
local p_granite_light = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage     = pile{{"lead white", 16}, {"green earth", 6.5}, {"yellow ochre", 3.5}, {"raw umber", 1.0}, medium=0.25}
local p_lich_gold     = pile{{"lead white", 12}, {"yellow ochre", 9.0}, {"chrome yellow", 1.8}, {"raw umber", 0.8}, medium=0.25}

local b_block = brush{kind="filbert", width=5.5, stiffness=0.85}
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_crack = brush{kind="round", width=0.9, point=1, stiffness=0.9}

-- A. Sacred Chamber Void (raw umber + bone black)
b_block:load(p_cavern_void, 0.95)
b_block:stroke({{344, 406}, {385, 406}, {425, 406}, {452, 407}}, {pressure=0.9})
b_block:stroke({{348, 416}, {395, 418}, {448, 417}}, {pressure=0.9})
b_block:stroke({{352, 426}, {400, 428}, {444, 427}}, {pressure=0.9})

-- B. Left Orthostat
b_block:load(p_granite_shade, 0.9)
b_block:stroke({{338, 405}, {342, 418}, {346, 434}}, {pressure=0.85})
b_block:stroke({{348, 405}, {352, 418}, {356, 434}}, {pressure=0.85})
b_facet:load(p_granite_facet, 0.8)
b_facet:stroke({{338, 405}, {340, 416}, {344, 428}, {346, 436}}, {pressure={0.7, 0.5, 0.6, 0.2}})

-- C. Rear Orthostat
b_facet:load(p_granite_deep, 0.85)
b_facet:stroke({{394, 406}, {396, 420}, {398, 430}}, {pressure=0.75})
b_facet:stroke({{406, 406}, {408, 420}, {410, 430}}, {pressure=0.75})

-- D. Right Orthostat
b_block:load(p_granite_shade, 0.9)
b_block:stroke({{428, 405}, {432, 418}, {434, 434}}, {pressure=0.85})
b_block:stroke({{438, 405}, {442, 418}, {445, 434}}, {pressure=0.85})
b_block:stroke({{448, 404}, {452, 417}, {455, 434}}, {pressure=0.85})
b_facet:load(p_granite_facet, 0.85)
b_facet:stroke({{448, 404}, {452, 414}, {456, 425}, {458, 434}}, {pressure={0.75, 0.8, 0.6, 0.2}})
b_facet:load(p_granite_light, 0.8)
b_facet:stroke({{450, 404}, {454, 410}, {452, 418}}, {pressure={0.65, 0.5, 0.2}})

-- E. Massive Granite Capstone (Rough-hewn, angular, crystalline erratic)
-- Deep underside cast shadow
b_block:load(p_granite_deep, 0.95)
b_block:stroke({{316, 404}, {355, 405}, {400, 406}, {445, 405}, {474, 403}}, {pressure=0.9})
b_crack:load(p_cavern_void, 0.95)
b_crack:stroke({{314, 405}, {350, 407}, {395, 408}, {440, 407}, {476, 404}}, {pressure=0.85})

-- Front face (crystalline texture)
b_block:load(p_granite_body, 0.9)
b_block:stroke({{318, 401}, {350, 402}, {390, 403}, {430, 402}, {468, 400}}, {pressure=0.85})
b_block:stroke({{324, 395}, {360, 396}, {405, 397}, {445, 396}, {470, 394}}, {pressure=0.85})

-- Top sky-facing facets (sharp, angular crystalline planes)
local pts_fA = {{314, 402}, {322, 393}, {340, 385}, {368, 381}, {394, 380}, {392, 393}, {365, 395}, {338, 398}, {318, 402}}
local m_fA = poly(pts_fA, false)
work(m_fA, {hand="detail", pile=p_granite_light, angle=0.04, coverage=1.6, fill=true, clip=true})

local pts_fB = {{394, 380}, {425, 382}, {455, 385}, {472, 391}, {476, 399}, {466, 400}, {440, 397}, {412, 395}, {392, 393}}
local m_fB = poly(pts_fB, false)
work(m_fB, {hand="detail", pile=p_granite_facet, angle=-0.04, coverage=1.6, fill=true, clip=true})

-- Fracture cleavage cracks
b_crack:load(p_granite_deep, 0.9)
b_crack:stroke({{394, 380}, {392, 393}, {390, 407}}, {pressure={0.7, 0.85, 0.3}})
b_crack:stroke({{348, 384}, {352, 395}, {350, 405}}, {pressure={0.45, 0.6, 0.2}})
b_crack:stroke({{442, 384}, {444, 394}, {446, 405}}, {pressure={0.5, 0.65, 0.2}})

-- Top edge crystalline highlights catching twilight
b_crack:load(p_granite_light, 0.95)
b_crack:stroke({{314, 402}, {322, 393}, {340, 385}, {368, 381}, {394, 380}}, {pressure={0.3, 0.8, 0.7, 0.85, 0.4}})
b_crack:stroke({{396, 381}, {425, 382}, {455, 385}, {472, 391}}, {pressure={0.4, 0.8, 0.75, 0.3}})

-- Lichen touches on Capstone
local lichen_patches = {
  {335, 389, p_lich_sage, 2.2},
  {362, 385, p_lich_gold, 2.5},
  {380, 391, p_lich_sage, 1.8},
  {415, 387, p_lich_gold, 2.4},
  {438, 389, p_lich_sage, 2.0},
  {458, 393, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichen_patches) do
  b_facet:load(lp[3], 0.7)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- F. Flanking Erratics
local pts_el = {{294, 436}, {302, 420}, {318, 414}, {328, 422}, {324, 438}, {298, 438}}
local m_el = poly(pts_el, false)
work(m_el, {hand="detail", pile=p_granite_body, angle=0.3, coverage=1.6, fill=true, clip=true})
local pts_el_lit = {{302, 420}, {318, 414}, {328, 422}, {316, 426}, {306, 425}}
local m_el_lit = poly(pts_el_lit, false)
work(m_el_lit, {hand="detail", pile=p_granite_facet, angle=0.15, coverage=1.4, fill=true, clip=true})

local pts_er = {{470, 436}, {478, 418}, {504, 414}, {516, 426}, {508, 438}, {474, 438}}
local m_er = poly(pts_er, false)
work(m_er, {hand="detail", pile=p_granite_body, angle=-0.2, coverage=1.6, fill=true, clip=true})
local pts_er_lit = {{478, 418}, {504, 414}, {516, 426}, {500, 425}, {484, 423}}
local m_er_lit = poly(pts_er_lit, false)
work(m_er_lit, {hand="detail", pile=p_granite_facet, angle=-0.1, coverage=1.4, fill=true, clip=true})

-- G. Bed stones deeply into dark peat
b_block:load(p_peat_crest, 0.95)
b_block:stroke({{285, 438}, {330, 438}, {370, 438}}, {pressure=0.9})
b_block:stroke({{420, 438}, {470, 438}, {525, 438}}, {pressure=0.9})

-- 3. Windswept Bent-Grass Tufts & Carline Thistles along the Ridge
local p_straw_sharp = pile{{"lead white", 16}, {"yellow ochre", 9}, {"chrome yellow", 1.2}, {"raw umber", 1.0}, medium=0.22}
local p_straw_shade = pile{{"yellow ochre", 7}, {"raw umber", 6}, {"bone black", 2.5}, {"lead white", 2.0}, medium=0.2}
local p_thistle_dark = pile{{"bone black", 10}, {"raw umber", 8}, medium=0.15}

local function draw_curved_grass(bx, by, len, ang, bend, p)
  local mx = bx + len * 0.55 * math.cos(ang + bend * 0.5)
  local my = by - len * 0.55 * math.sin(ang + bend * 0.5)
  local tx = bx + len * math.cos(ang + bend)
  local ty = by - len * math.sin(ang + bend)
  local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}
  b_rigger:load(p, 0.75)
  b_rigger:stroke({{bx, by}, {mx, my}, {tx, ty}}, {pressure={0.7, 0.05}, ramps={0.05, 0.3}})
end

local function draw_grass_cluster(cx, cy, n, h, lean)
  for i = 1, n do
    local bx = cx + rand(-5, 5)
    local by = cy + rand(-2, 2)
    local len = h * rand(0.75, 1.25)
    local ang = 1.57 + rand(-0.25, 0.25) + lean * 0.3
    local bnd = lean * rand(0.2, 0.5)
    local p = (rand(0, 1) < 0.65) and p_straw_sharp or p_straw_shade
    draw_curved_grass(bx, by, len, ang, bnd, p)
  end
end

-- Grasses along the barrow crest
local crest_grasses = {
  {120, 452, 7, 16, -0.4}, {180, 444, 7, 15, -0.3}, {240, 432, 6, 14, -0.2},
  {310, 420, 6, 14, -0.1}, {475, 420, 6, 14, 0.2}, {540, 434, 7, 16, 0.3},
  {620, 446, 7, 16, 0.3}, {700, 448, 6, 15, 0.4}, {780, 436, 7, 16, 0.4}
}
for _, g in ipairs(crest_grasses) do draw_grass_cluster(g[1], g[2], g[3], g[4], g[5]) end

-- Carline thistles silhouetted against glowing mist
local thistles = {{150, 448, 22}, {560, 440, 20}, {840, 438, 18}}
for _, th in ipairs(thistles) do
  local x, y, h = th[1], th[2], th[3]
  b_crest:load(p_thistle_dark, 0.85)
  b_crest:stroke({{x, y}, {x, y - h*0.6}, {x + rand(-2, 2), y - h}}, {pressure={0.7, 0.15}, ramps={0.05, 0.2}})
  b_facet:load(p_straw_sharp, 0.8)
  b_facet:stroke({{x - 3, y - h}, {x + 3, y - h}}, {pressure=0.4})
end

print("Chunk 91 complete: Monumental Megalithic Tomb sculpted, crest articulated with bent-grass and thistles.")

--@ chunk 91
-- Chunk 92: Transforming the Megalithic Tomb into Authentic Weathered Ancient Granite

-- 1. Piles for Weathered Ancient Baltic Granite (deep, muted, crystalline tones — NOT bright white!)
local p_cavern_void   = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}
local p_granite_deep  = pile{{"bone black", 9}, {"raw umber", 8}, {"smalt", 2.5}, {"red earth", 1.5}, medium=0.15}
local p_granite_shade = pile{{"raw umber", 9}, {"bone black", 6}, {"smalt", 2.5}, {"lead white", 3.0}, {"yellow ochre", 2.5}, {"red earth", 1.2}, medium=0.18}
local p_granite_body  = pile{{"lead white", 8}, {"raw umber", 8}, {"bone black", 4}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"red earth", 1.2}, medium=0.2}
local p_granite_facet = pile{{"lead white", 14}, {"raw umber", 5}, {"yellow ochre", 3.5}, {"smalt", 2.0}, {"bone black", 2}, {"red earth", 0.8}, medium=0.22}
local p_granite_sheen = pile{{"lead white", 20}, {"smalt", 2.5}, {"yellow ochre", 2.5}, {"raw umber", 1.2}, {"vermilion", 0.25}, medium=0.22}

local p_lich_sage     = pile{{"lead white", 14}, {"green earth", 7.0}, {"yellow ochre", 3.5}, {"raw umber", 1.2}, medium=0.25}
local p_lich_gold     = pile{{"lead white", 10}, {"yellow ochre", 9.5}, {"chrome yellow", 1.8}, {"raw umber", 1.0}, medium=0.25}
local p_peat_dark     = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}

local b_block  = brush{kind="filbert", width=6.5, stiffness=0.85}
local b_facet  = brush{kind="round", width=2.6, point=1, stiffness=0.85}
local b_crack  = brush{kind="round", width=1.0, point=1, stiffness=0.9}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}

-- 2. The Sacred Burial Chamber Void (deep velvety black void within the stone ring)
b_block:load(p_cavern_void, 0.95)
b_block:stroke({{338, 408}, {385, 407}, {425, 407}, {456, 409}}, {pressure=0.95})
b_block:stroke({{342, 418}, {395, 420}, {450, 419}}, {pressure=0.95})
b_block:stroke({{346, 428}, {400, 430}, {446, 429}}, {pressure=0.95})

-- 3. Massive Chunky Orthostats (Heavy, rugged glacial granite uprights — NOT thin legs!)
-- A. Left Orthostat (weathered, leaning slightly inward, width ~18 units)
b_block:load(p_granite_shade, 0.95)
b_block:stroke({{336, 404}, {340, 418}, {344, 436}}, {pressure=0.9})
b_block:stroke({{346, 404}, {350, 418}, {354, 436}}, {pressure=0.9})
b_block:stroke({{356, 405}, {358, 418}, {362, 435}}, {pressure=0.85})
-- Angular facet catching twilight on outer edge
b_facet:load(p_granite_facet, 0.85)
b_facet:stroke({{336, 404}, {338, 416}, {342, 428}, {344, 438}}, {pressure={0.7, 0.6, 0.5, 0.2}})

-- B. Center Rear Orthostat (deep inside the chamber)
b_block:load(p_granite_deep, 0.9)
b_block:stroke({{390, 406}, {392, 420}, {394, 432}}, {pressure=0.85})
b_block:stroke({{402, 406}, {404, 420}, {406, 432}}, {pressure=0.85})

-- C. Right Orthostat (sturdy, crystalline pillar, width ~20 units)
b_block:load(p_granite_shade, 0.95)
b_block:stroke({{428, 405}, {432, 418}, {434, 436}}, {pressure=0.9})
b_block:stroke({{438, 405}, {442, 418}, {444, 436}}, {pressure=0.9})
b_block:stroke({{448, 404}, {452, 418}, {454, 435}}, {pressure=0.9})
-- Outer plane catching twilight
b_facet:load(p_granite_facet, 0.85)
b_facet:stroke({{448, 404}, {452, 416}, {455, 428}, {457, 436}}, {pressure={0.75, 0.8, 0.6, 0.2}})
b_facet:load(p_granite_sheen, 0.8)
b_facet:stroke({{450, 405}, {453, 412}, {452, 422}}, {pressure={0.65, 0.5, 0.2}})

-- 4. The Monumental Granite Capstone (Massive, heavy, rough-hewn erratic block — NOT an oval UFO!)
-- Resurface the whole capstone body with dark, heavy weathered granite
b_block:load(p_granite_shade, 0.95)
b_block:stroke({{314, 402}, {350, 403}, {395, 404}, {440, 403}, {474, 401}}, {pressure=0.95})
b_block:stroke({{318, 396}, {355, 397}, {400, 398}, {445, 397}, {472, 395}}, {pressure=0.95})

-- Underside deep cast shadow projection
b_crack:load(p_cavern_void, 0.95)
b_crack:stroke({{314, 405}, {350, 407}, {395, 408}, {440, 407}, {476, 404}}, {pressure=0.9})

-- Top sky-facing facets: sharp, angular, crystalline cleavage planes
-- Facet A: Western sloping block (x = 314 to 394, angular and stepped!)
local pts_fA = {{314, 402}, {320, 392}, {338, 384}, {366, 381}, {394, 380}, {392, 394}, {365, 396}, {338, 399}, {318, 402}}
local m_fA = poly(pts_fA, false)
work(m_fA, {hand="detail", pile=p_granite_facet, angle=0.04, coverage=1.6, fill=true, clip=true})

-- Facet B: Eastern stepped block (x = 394 to 474, slightly lower/different tilt)
local pts_fB = {{394, 380}, {425, 382}, {455, 386}, {472, 392}, {476, 400}, {466, 401}, {440, 398}, {412, 396}, {392, 394}}
local m_fB = poly(pts_fB, false)
work(m_fB, {hand="detail", pile=p_granite_body, angle=-0.04, coverage=1.6, fill=true, clip=true})

-- Major geological cleavage fracture cutting across the capstone at x ≈ 394
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{394, 380}, {392, 394}, {390, 408}}, {pressure={0.75, 0.85, 0.4}})
-- Secondary stress cracks
b_crack:stroke({{346, 385}, {350, 396}, {348, 406}}, {pressure={0.5, 0.65, 0.2}})
b_crack:stroke({{442, 386}, {444, 396}, {446, 406}}, {pressure={0.5, 0.65, 0.2}})

-- Subtle silvery twilight sheen on the top ridge (NOT a continuous white line!)
b_crack:load(p_granite_sheen, 0.85)
b_crack:stroke({{316, 400}, {324, 392}, {342, 384}}, {pressure={0.3, 0.7, 0.3}})
b_crack:stroke({{368, 381}, {394, 380}}, {pressure={0.4, 0.7, 0.3}})
b_crack:stroke({{398, 381}, {425, 382}, {455, 386}}, {pressure={0.35, 0.65, 0.25}})

-- Crustose lichens encrusting the ancient granite
local lichen_patches = {
  {335, 389, p_lich_sage, 2.2},
  {362, 385, p_lich_gold, 2.5},
  {380, 391, p_lich_sage, 1.8},
  {415, 387, p_lich_gold, 2.4},
  {438, 389, p_lich_sage, 2.0},
  {458, 393, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichen_patches) do
  b_facet:load(lp[3], 0.7)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- 5. Flanking Glacial Erratics (angular Scandinavian granite blocks)
local pts_el = {{294, 436}, {302, 420}, {318, 414}, {328, 422}, {324, 438}, {298, 438}}
local m_el = poly(pts_el, false)
work(m_el, {hand="detail", pile=p_granite_body, angle=0.3, coverage=1.6, fill=true, clip=true})
local pts_el_lit = {{302, 420}, {318, 414}, {328, 422}, {316, 426}, {306, 425}}
local m_el_lit = poly(pts_el_lit, false)
work(m_el_lit, {hand="detail", pile=p_granite_facet, angle=0.15, coverage=1.4, fill=true, clip=true})

local pts_er = {{470, 436}, {478, 418}, {504, 414}, {516, 426}, {508, 438}, {474, 438}}
local m_er = poly(pts_er, false)
work(m_er, {hand="detail", pile=p_granite_body, angle=-0.2, coverage=1.6, fill=true, clip=true})
local pts_er_lit = {{478, 418}, {504, 414}, {516, 426}, {500, 425}, {484, 423}}
local m_er_lit = poly(pts_er_lit, false)
work(m_er_lit, {hand="detail", pile=p_granite_facet, angle=-0.1, coverage=1.4, fill=true, clip=true})

-- 6. Bed the Stones Deeply into the Barrow Soil with Dark Peat and Heather (ELIMINATE THE TWO DARK EYEBROWS!)
-- Overlap the dark horizontal eyebrow strokes completely with organic peat and heather
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{280, 442}, {315, 445}, {350, 446}, {380, 444}}, {pressure=0.95})
b_block:stroke({{410, 444}, {445, 446}, {480, 445}, {530, 442}}, {pressure=0.95})

-- Dark peaty turf directly framing the orthostat contact points
b_facet:load(p_cavern_void, 0.95)
b_facet:stroke({{334, 436}, {364, 436}}, {pressure=0.85})
b_facet:stroke({{426, 436}, {458, 436}}, {pressure=0.85})

print("Chunk 92 complete: Megalithic Tomb sculpted as heavy, weathered, ancient Baltic granite.")

--@ chunk 92
-- Chunk 93: Resculpting the Dolmen as Authentic Massive Glacial Granite — Eliminating Table Legs and Eyebrows

-- 1. Piles
local p_cavern_pitch = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}
local p_granite_deep = pile{{"bone black", 9}, {"raw umber", 8}, {"smalt", 2.5}, {"red earth", 1.5}, medium=0.15}
local p_granite_body = pile{{"lead white", 10}, {"raw umber", 7}, {"bone black", 4}, {"yellow ochre", 3.5}, {"smalt", 2.0}, {"red earth", 1.0}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 20}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 1.2}, {"vermilion", 0.3}, medium=0.22}
local p_granite_sheen= pile{{"lead white", 26}, {"smalt", 2.5}, {"yellow ochre", 2.5}, {"raw umber", 0.8}, medium=0.22}
local p_lich_sage    = pile{{"lead white", 14}, {"green earth", 7.0}, {"yellow ochre", 3.5}, {"raw umber", 1.2}, medium=0.25}
local p_lich_gold    = pile{{"lead white", 10}, {"yellow ochre", 9.5}, {"chrome yellow", 1.8}, {"raw umber", 1.0}, medium=0.25}
local p_peat_base    = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 4.0}, {"yellow ochre", 2.0}, medium=0.15}

local b_block = brush{kind="filbert", width=8.0, stiffness=0.85}
local b_limb  = brush{kind="filbert", width=5.0, stiffness=0.85}
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_crack = brush{kind="round", width=0.9, point=1, stiffness=0.9}

-- 2. Obliterate the 8 table legs: Fill the entire burial chamber with a deep, solid velvety black void
b_block:load(p_cavern_pitch, 0.95)
b_block:stroke({{340, 408}, {395, 408}, {455, 408}}, {pressure=0.95})
b_block:stroke({{342, 418}, {395, 419}, {452, 418}}, {pressure=0.95})
b_block:stroke({{345, 428}, {395, 429}, {450, 428}}, {pressure=0.95})

-- 3. Left Orthostat: One massive, solid, rugged upright stone block (width ~28 units)
b_block:load(p_granite_body, 0.95)
b_block:stroke({{336, 404}, {340, 420}, {344, 438}}, {pressure=0.95})
b_block:stroke({{348, 404}, {352, 420}, {356, 438}}, {pressure=0.95})
-- Highlight on the outer left face
b_facet:load(p_granite_lit, 0.9)
b_facet:stroke({{335, 404}, {338, 416}, {342, 428}, {344, 438}}, {pressure={0.8, 0.7, 0.6, 0.2}})
-- Shadow side facing chamber
b_facet:load(p_granite_deep, 0.9)
b_facet:stroke({{358, 406}, {362, 420}, {364, 436}}, {pressure=0.8})

-- 4. Right Orthostat: One massive, solid, rugged upright stone block (width ~30 units)
b_block:load(p_granite_body, 0.95)
b_block:stroke({{435, 404}, {438, 420}, {442, 438}}, {pressure=0.95})
b_block:stroke({{448, 404}, {452, 420}, {456, 438}}, {pressure=0.95})
-- Highlight on the outer right face
b_facet:load(p_granite_lit, 0.9)
b_facet:stroke({{448, 404}, {452, 416}, {456, 428}, {458, 438}}, {pressure={0.8, 0.75, 0.6, 0.2}})
b_facet:load(p_granite_sheen, 0.85)
b_facet:stroke({{450, 405}, {454, 412}, {453, 422}}, {pressure={0.65, 0.5, 0.2}})
-- Shadow side facing chamber
b_facet:load(p_granite_deep, 0.9)
b_facet:stroke({{430, 406}, {432, 420}, {434, 436}}, {pressure=0.8})

-- 5. Rear Stone inside the chamber (dimly visible)
b_limb:load(p_granite_deep, 0.9)
b_limb:stroke({{390, 408}, {394, 422}, {396, 434}}, {pressure=0.85})
b_limb:stroke({{404, 408}, {406, 422}, {408, 434}}, {pressure=0.85})

-- 6. The Monumental Granite Capstone: Heavy, rugged, faceted glacial boulder
-- Deep cast shadow directly under the overhang
b_block:load(p_cavern_pitch, 0.95)
b_block:stroke({{314, 404}, {355, 405}, {400, 406}, {445, 405}, {476, 403}}, {pressure=0.95})

-- Capstone front face (solid weathered granite)
b_block:load(p_granite_body, 0.95)
b_block:stroke({{316, 400}, {350, 401}, {395, 402}, {440, 401}, {472, 399}}, {pressure=0.95})
b_block:stroke({{322, 394}, {360, 395}, {405, 396}, {445, 395}, {470, 393}}, {pressure=0.95})

-- Top faceted cleavage planes
local pts_cap_topA = {{314, 398}, {322, 388}, {340, 380}, {368, 377}, {394, 376}, {392, 390}, {365, 392}, {338, 395}, {316, 398}}
local m_cap_topA = poly(pts_cap_topA, false)
work(m_cap_topA, {hand="detail", pile=p_granite_lit, angle=0.04, coverage=1.6, fill=true, clip=true})

local pts_cap_topB = {{394, 376}, {425, 378}, {455, 382}, {472, 388}, {476, 396}, {466, 397}, {440, 394}, {412, 392}, {392, 390}}
local m_cap_topB = poly(pts_cap_topB, false)
work(m_cap_topB, {hand="detail", pile=p_granite_body, angle=-0.04, coverage=1.6, fill=true, clip=true})

-- Fracture cleavage cracks
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{394, 376}, {392, 390}, {390, 405}}, {pressure={0.75, 0.85, 0.4}})
b_crack:stroke({{346, 381}, {350, 392}, {348, 403}}, {pressure={0.5, 0.65, 0.2}})
b_crack:stroke({{442, 382}, {444, 392}, {446, 403}}, {pressure={0.5, 0.65, 0.2}})

-- Top edge crystalline highlights catching twilight
b_crack:load(p_granite_sheen, 0.95)
b_crack:stroke({{314, 398}, {322, 388}, {340, 380}, {368, 377}, {394, 376}}, {pressure={0.3, 0.8, 0.7, 0.85, 0.4}})
b_crack:stroke({{396, 377}, {425, 378}, {455, 382}, {472, 388}}, {pressure={0.4, 0.8, 0.75, 0.3}})

-- Lichen touches
local lichens = {
  {335, 385, p_lich_sage, 2.2}, {362, 381, p_lich_gold, 2.5},
  {380, 387, p_lich_sage, 1.8}, {415, 383, p_lich_gold, 2.4},
  {438, 385, p_lich_sage, 2.0}, {458, 389, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichens) do
  b_facet:load(lp[3], 0.75)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- 7. Flanking Glacial Boulders
-- Left leaning stone
local pts_sl = {{294, 436}, {304, 416}, {322, 410}, {332, 418}, {326, 438}, {298, 438}}
local m_sl = poly(pts_sl, false)
work(m_sl, {hand="detail", pile=p_granite_body, angle=0.3, coverage=1.6, fill=true, clip=true})
local pts_sl_lit = {{304, 416}, {322, 410}, {332, 418}, {320, 422}, {308, 421}}
local m_sl_lit = poly(pts_sl_lit, false)
work(m_sl_lit, {hand="detail", pile=p_granite_lit, angle=0.15, coverage=1.4, fill=true, clip=true})

-- Right erratic block
local pts_sr = {{468, 436}, {478, 414}, {506, 410}, {518, 422}, {510, 438}, {472, 438}}
local m_sr = poly(pts_sr, false)
work(m_sr, {hand="detail", pile=p_granite_body, angle=-0.2, coverage=1.6, fill=true, clip=true})
local pts_sr_lit = {{478, 414}, {506, 410}, {518, 422}, {504, 421}, {486, 419}}
local m_sr_lit = poly(pts_sr_lit, false)
work(m_sr_lit, {hand="detail", pile=p_granite_lit, angle=-0.1, coverage=1.4, fill=true, clip=true})

-- 8. Complete Elimination of Eyebrows and Pill Gaps: Solid Peat & Heather Bedding
-- Generous, solid strokes covering everything from y = 432 to 455
b_block:load(p_peat_base, 0.95)
b_block:stroke({{275, 438}, {315, 442}, {355, 444}, {395, 444}, {435, 444}, {475, 442}, {525, 438}}, {pressure=0.95})
b_block:stroke({{280, 446}, {330, 448}, {380, 450}, {430, 450}, {480, 448}, {530, 446}}, {pressure=0.95})

-- Dark turf contact directly framing the base of the two massive uprights
b_facet:load(p_cavern_pitch, 0.95)
b_facet:stroke({{332, 436}, {364, 436}}, {pressure=0.85})
b_facet:stroke({{428, 436}, {460, 436}}, {pressure=0.85})

print("Chunk 93 complete: Megalithic Tomb resculpted as true ancient granite; table legs and eyebrows eliminated.")

--@ chunk 93
-- Chunk 94: Resculpting the Megalithic Tomb with Full Hiding via work()

-- 1. Piles
local p_cavern_pitch = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}
local p_granite_deep = pile{{"bone black", 9}, {"raw umber", 8}, {"smalt", 2.5}, {"red earth", 1.5}, medium=0.15}
local p_granite_body = pile{{"lead white", 10}, {"raw umber", 7}, {"bone black", 4}, {"yellow ochre", 3.5}, {"smalt", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_granite_facet= pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage    = pile{{"lead white", 14}, {"green earth", 7.0}, {"yellow ochre", 3.5}, {"raw umber", 1.2}, medium=0.25}
local p_lich_gold    = pile{{"lead white", 10}, {"yellow ochre", 9.5}, {"chrome yellow", 1.8}, {"raw umber", 1.0}, medium=0.25}
local p_peat_dark    = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 4.0}, {"yellow ochre", 2.0}, medium=0.15}

-- 2. Obliterate the table legs: 100% opaque chamber void
local m_chamber = poly({{336, 400}, {456, 400}, {456, 442}, {336, 442}})
work(m_chamber, {hand="body", pile=p_cavern_pitch, angle=0, coverage=2.5, fill=true, clip=true})

-- 3. Left Orthostat: One massive, solid stone block (width ~30 units)
local m_lo = poly({{330, 398}, {362, 398}, {366, 440}, {332, 440}})
work(m_lo, {hand="body", pile=p_granite_body, angle=1.45, coverage=2.2, fill=true, clip=true})
local m_lo_lit = poly({{330, 398}, {342, 398}, {344, 440}, {332, 440}})
work(m_lo_lit, {hand="detail", pile=p_granite_facet, angle=1.45, coverage=1.6, fill=true, clip=true})

-- 4. Right Orthostat: One massive, solid stone block (width ~32 units)
local m_ro = poly({{428, 398}, {462, 398}, {460, 440}, {426, 440}})
work(m_ro, {hand="body", pile=p_granite_body, angle=1.4, coverage=2.2, fill=true, clip=true})
local m_ro_lit = poly({{446, 398}, {462, 398}, {460, 440}, {448, 440}})
work(m_ro_lit, {hand="detail", pile=p_granite_facet, angle=1.4, coverage=1.6, fill=true, clip=true})

-- 5. The Massive Granite Capstone: Heavy, angular, rugged erratic boulder (NOT an oval!)
local pts_cap = {
  {312, 404}, {322, 394}, {342, 384}, {372, 378}, {408, 376}, {445, 380},
  {475, 388}, {482, 398}, {466, 404}, {430, 404}, {385, 405}, {345, 404}, {312, 404}
}
local m_cap = poly(pts_cap, false) -- angular!
work(m_cap, {hand="body", pile=p_granite_body, angle=0.06, coverage=2.2, fill=true, clip=true})

-- Deep cast shadow directly under the capstone overhang
local pts_undercut = {
  {312, 404}, {345, 404}, {385, 405}, {430, 404}, {466, 404}, {482, 398},
  {478, 408}, {435, 409}, {385, 410}, {340, 409}, {312, 404}
}
local m_undercut = poly(pts_undercut, false)
work(m_undercut, {hand="detail", pile=p_cavern_pitch, angle=0, coverage=2.0, fill=true, clip=true})

-- Top sky-facing facets catching twilight
local pts_topA = {
  {312, 404}, {322, 394}, {342, 384}, {372, 378}, {400, 376},
  {396, 392}, {365, 394}, {338, 398}, {312, 404}
}
local m_topA = poly(pts_topA, false)
work(m_topA, {hand="detail", pile=p_granite_lit, angle=0.04, coverage=1.6, fill=true, clip=true})

local pts_topB = {
  {400, 376}, {445, 380}, {475, 388}, {482, 398}, {466, 404},
  {440, 400}, {412, 396}, {396, 392}
}
local m_topB = poly(pts_topB, false)
work(m_topB, {hand="detail", pile=p_granite_facet, angle=-0.04, coverage=1.6, fill=true, clip=true})

-- Cleavage fracture crack
local b_crack = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{400, 376}, {396, 392}, {392, 405}}, {pressure={0.75, 0.85, 0.4}})
b_crack:stroke({{348, 383}, {352, 394}, {350, 404}}, {pressure={0.5, 0.65, 0.2}})
b_crack:stroke({{442, 380}, {444, 392}, {446, 404}}, {pressure={0.5, 0.65, 0.2}})

-- Top ridge highlights catching the silvery sky
b_crack:load(p_granite_lit, 0.95)
b_crack:stroke({{312, 404}, {322, 394}, {342, 384}, {372, 378}, {400, 376}}, {pressure={0.3, 0.8, 0.7, 0.85, 0.4}})
b_crack:stroke({{402, 377}, {445, 380}, {475, 388}}, {pressure={0.4, 0.8, 0.75, 0.3}})

-- Lichens on Capstone
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local lichens = {
  {335, 388, p_lich_sage, 2.2}, {362, 382, p_lich_gold, 2.5},
  {380, 388, p_lich_sage, 1.8}, {420, 382, p_lich_gold, 2.4},
  {445, 386, p_lich_sage, 2.0}, {465, 392, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichens) do
  b_facet:load(lp[3], 0.75)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- 6. Flanking Glacial Boulders
-- Left leaning boulder
local m_sl = poly({{294, 436}, {304, 416}, {322, 410}, {330, 418}, {324, 438}, {296, 438}}, false)
work(m_sl, {hand="detail", pile=p_granite_body, angle=0.3, coverage=1.8, fill=true, clip=true})
local m_sl_lit = poly({{304, 416}, {322, 410}, {330, 418}, {318, 422}, {308, 421}}, false)
work(m_sl_lit, {hand="detail", pile=p_granite_facet, angle=0.15, coverage=1.5, fill=true, clip=true})

-- Right erratic boulder
local m_sr = poly({{466, 436}, {476, 414}, {504, 410}, {516, 422}, {508, 438}, {470, 438}}, false)
work(m_sr, {hand="detail", pile=p_granite_body, angle=-0.2, coverage=1.8, fill=true, clip=true})
local m_sr_lit = poly({{476, 414}, {504, 410}, {516, 422}, {502, 421}, {484, 419}}, false)
work(m_sr_lit, {hand="detail", pile=p_granite_facet, angle=-0.1, coverage=1.5, fill=true, clip=true})

-- 7. Obliterate the Eyebrows and Pill Gaps with Solid Dark Peat & Heather Bedding
local m_base_fill = rect(270, 436, 260, 22)
work(m_base_fill, {hand="body", pile=p_peat_dark, angle=0.1, coverage=2.2, fill=true})

-- Blend the peat base softly into the barrow slope
blend(rect(260, 434, 280, 28), {angle=0.2})

print("Chunk 94 complete: Megalithic Tomb resculpted with full hiding; table legs and eyebrows permanently obliterated.")

--@ chunk 94
-- Chunk 95: Pass 1 — Naturalizing the Dolmen, Bedding into Heather, Boulders & Winding Path

-- 1. Piles
local p_peat_earth   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4.0}, {"yellow ochre", 3.0}, medium=0.18}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5.0}, {"lead white", 2.0}, {"bone black", 1.0}, medium=0.2}
local p_moss_green   = pile{{"green earth", 7}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}
local p_sky_glow     = pile{{"lead white", 26}, {"yellow ochre", 5.2}, {"chrome yellow", 2.2}, {"vermilion", 0.5}, {"raw umber", 0.25}, medium=0.22}

local p_granite_deep = pile{{"bone black", 9}, {"raw umber", 8}, {"smalt", 2.5}, {"red earth", 1.5}, medium=0.15}
local p_granite_body = pile{{"lead white", 10}, {"raw umber", 7}, {"bone black", 4}, {"yellow ochre", 3.5}, {"smalt", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_granite_facet= pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage    = pile{{"lead white", 14}, {"green earth", 7.0}, {"yellow ochre", 3.5}, {"raw umber", 1.2}, medium=0.25}
local p_lich_gold    = pile{{"lead white", 10}, {"yellow ochre", 9.5}, {"chrome yellow", 1.8}, {"raw umber", 1.0}, medium=0.25}

local p_sand_path    = pile{{"yellow ochre", 7}, {"lead white", 5}, {"raw umber", 4}, {"bone black", 1}, medium=0.2}
local p_sand_light   = pile{{"lead white", 12}, {"yellow ochre", 7}, {"raw umber", 2.5}, {"vermilion", 0.3}, medium=0.22}

local b_block  = brush{kind="filbert", width=7.0, stiffness=0.85}
local b_facet  = brush{kind="round", width=2.5, point=1, stiffness=0.85}
local b_crack  = brush{kind="round", width=1.0, point=1, stiffness=0.9}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}

-- 2. Dissolve the rectangular trench at the dolmen base: Blend into natural barrow slope
-- Multi-directional strokes of rich peaty heather across y = 432 to 462
b_block:load(p_heath_russet, 0.95)
b_block:stroke({{250, 445}, {300, 448}, {360, 450}, {420, 450}, {480, 448}, {540, 445}}, {pressure=0.95})
b_block:load(p_peat_earth, 0.95)
b_block:stroke({{260, 452}, {320, 455}, {385, 456}, {450, 455}, {510, 452}, {550, 448}}, {pressure=0.95})
b_block:load(p_moss_green, 0.9)
b_block:stroke({{310, 448}, {350, 452}, {390, 452}, {440, 450}}, {pressure=0.85})

-- Blend the turf around the stone base softly into the barrow body
blend(rect(240, 438, 320, 24), {angle=0.2})
blend(rect(240, 438, 320, 24), {angle=-0.15})

-- 3. Naturalize the Capstone: Break up the smooth elliptical curve with crystalline chips and facets
-- Sky cuts to break the smooth top curve into jagged, weathered rock facets
b_crack:load(p_sky_glow, 0.95)
b_crack:stroke({{338, 384}, {342, 388}, {348, 386}}, {pressure={0.5, 0.8, 0.2}})
b_crack:stroke({{405, 376}, {408, 380}, {412, 377}}, {pressure={0.4, 0.7, 0.2}})
b_crack:stroke({{452, 381}, {456, 385}, {460, 383}}, {pressure={0.4, 0.7, 0.2}})

-- Angular fractured rock facets on the capstone
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{320, 394}, {340, 386}, {365, 382}}, {pressure={0.7, 0.85, 0.6}})
b_facet:stroke({{370, 382}, {400, 380}, {430, 382}}, {pressure={0.7, 0.85, 0.6}})
b_facet:stroke({{435, 382}, {460, 386}, {478, 394}}, {pressure={0.6, 0.8, 0.5}})

-- Sharp cleavage cracks cleaving the boulder
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{398, 378}, {395, 392}, {392, 405}}, {pressure={0.8, 0.85, 0.4}})
b_crack:stroke({{346, 384}, {350, 394}, {348, 404}}, {pressure={0.5, 0.65, 0.2}})
b_crack:stroke({{445, 383}, {448, 393}, {450, 404}}, {pressure={0.5, 0.65, 0.2}})

-- Crystalline shelf highlights on top edges
b_crack:load(p_granite_lit, 0.95)
b_crack:stroke({{316, 402}, {324, 393}, {342, 385}, {368, 381}, {398, 378}}, {pressure={0.3, 0.8, 0.7, 0.85, 0.4}})
b_crack:stroke({{400, 378}, {430, 381}, {458, 385}, {476, 394}}, {pressure={0.4, 0.8, 0.75, 0.3}})

-- Lichen patches encrusting the granite
local lichens = {
  {332, 388, p_lich_sage, 2.2}, {360, 383, p_lich_gold, 2.5},
  {382, 388, p_lich_sage, 1.8}, {418, 383, p_lich_gold, 2.4},
  {442, 386, p_lich_sage, 2.0}, {468, 392, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichens) do
  b_facet:load(lp[3], 0.8)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- 4. Flanking Boulders Naturalized (Bed into heather, break trapezoid shapes)
-- Left boulder
b_facet:load(p_granite_body, 0.9)
b_facet:stroke({{296, 436}, {304, 418}, {318, 414}, {326, 422}, {322, 438}}, {pressure=0.85})
b_facet:load(p_granite_lit, 0.85)
b_facet:stroke({{304, 418}, {318, 414}, {326, 422}}, {pressure={0.4, 0.8, 0.4}})
b_block:load(p_peat_earth, 0.9)
b_block:stroke({{290, 438}, {310, 442}, {330, 440}}, {pressure=0.85})

-- Right boulder
b_facet:load(p_granite_body, 0.9)
b_facet:stroke({{472, 436}, {480, 416}, {504, 412}, {516, 424}, {508, 438}}, {pressure=0.85})
b_facet:load(p_granite_lit, 0.85)
b_facet:stroke({{480, 416}, {504, 412}, {516, 424}}, {pressure={0.4, 0.8, 0.4}})
b_block:load(p_peat_earth, 0.9)
b_block:stroke({{465, 438}, {490, 442}, {520, 440}}, {pressure=0.85})

-- 5. Foreground Glacial Erratic Boulders (*Findlinge*)
-- Left Foreground Boulder (x ≈ 210 to 265, y ≈ 525 to 555)
local pts_b1 = {{210, 545}, {222, 528}, {250, 524}, {266, 534}, {264, 552}, {238, 556}, {212, 552}}
local m_b1 = poly(pts_b1, false)
work(m_b1, {hand="body", pile=p_granite_body, angle=0.2, coverage=1.8, fill=true, clip=true})
local m_b1_sh = poly({{210, 545}, {232, 552}, {264, 552}, {238, 556}, {212, 552}}, false)
work(m_b1_sh, {hand="detail", pile=p_granite_deep, angle=0.2, coverage=1.6, fill=true, clip=true})
local m_b1_lit = poly({{222, 528}, {250, 524}, {266, 534}, {252, 538}, {228, 536}}, false)
work(m_b1_lit, {hand="detail", pile=p_granite_lit, angle=0.05, coverage=1.6, fill=true, clip=true})

b_crack:load(p_granite_deep, 0.85)
b_crack:stroke({{236, 526}, {240, 538}, {238, 554}}, {pressure={0.5, 0.7, 0.2}})
b_crack:load(p_lich_sage, 0.8)
b_crack:touch(232, 530, {pressure=0.6, drag={2.0, 0.1}})
b_crack:load(p_lich_gold, 0.75)
b_crack:touch(250, 530, {pressure=0.55, drag={1.8, 0.1}})

-- Right Foreground Boulder (x ≈ 655 to 715, y ≈ 535 to 565)
local pts_b2 = {{656, 554}, {668, 538}, {696, 534}, {712, 546}, {708, 562}, {680, 566}, {658, 562}}
local m_b2 = poly(pts_b2, false)
work(m_b2, {hand="body", pile=p_granite_body, angle=-0.2, coverage=1.8, fill=true, clip=true})
local m_b2_sh = poly({{656, 554}, {675, 560}, {708, 562}, {680, 566}, {658, 562}}, false)
work(m_b2_sh, {hand="detail", pile=p_granite_deep, angle=-0.2, coverage=1.6, fill=true, clip=true})
local m_b2_lit = poly({{668, 538}, {696, 534}, {712, 546}, {694, 548}, {674, 546}}, false)
work(m_b2_lit, {hand="detail", pile=p_granite_lit, angle=-0.05, coverage=1.6, fill=true, clip=true})

b_crack:load(p_granite_deep, 0.85)
b_crack:stroke({{682, 536}, {684, 548}, {682, 562}}, {pressure={0.5, 0.7, 0.2}})
b_crack:load(p_lich_sage, 0.8)
b_crack:touch(678, 540, {pressure=0.6, drag={2.0, 0.1}})
b_crack:load(p_lich_gold, 0.75)
b_crack:touch(692, 538, {pressure=0.55, drag={1.8, 0.1}})

-- Bed boulders into surrounding heather and moss (NO hard black puddles!)
b_block:load(p_peat_earth, 0.95)
b_block:stroke({{198, 554}, {230, 560}, {265, 558}, {285, 550}}, {pressure=0.9})
b_block:stroke({{642, 562}, {675, 568}, {710, 566}, {730, 556}}, {pressure=0.9})

-- 6. Subtle, Natural Winding Pilgrim Track through the Heath
local pts_path = {{710, 705}, {672, 650}, {630, 595}, {582, 545}, {535, 502}, {485, 470}, {445, 452}}
local m_path   = ribbon(pts_path, {18, 15, 12, 9, 7, 5, 3}):soften(4)
work(m_path, {hand="scumble", pile=p_sand_path, angle=0.4, coverage=1.1, fill=true})

-- Subtle light track in center
local m_path_c = ribbon(pts_path, {9, 7.5, 6, 4.5, 3.5, 2.5, 1.5}):soften(2.5)
work(m_path_c, {hand="scumble", pile=p_sand_light, angle=0.35, coverage=0.9, fill=true})

-- Blend the path edges softly into the dark heather
blend(m_path, {angle=0.4})

print("Pass 1 complete: Dolmen naturalized, base bedded into turf, boulders sculpted, subtle path laid.")

--@ chunk 95
-- Chunk 96 (fixed): Restoring the Sacred Pristine Baltic Barrow — Eliminating Smoke Trail, Box Boulders, and Board

-- 1. Earth & Heather Palette
local p_earth_core   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4}, {"yellow ochre", 3}, medium=0.18}
local p_heath_dark   = pile{{"raw umber", 8}, {"bone black", 5}, {"red earth", 3.5}, {"green earth", 3.5}, medium=0.2}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5}, {"lead white", 2}, {"bone black", 1}, medium=0.2}
local p_peat_moss    = pile{{"green earth", 6}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}
local p_heath_gold   = pile{{"yellow ochre", 8}, {"raw umber", 4}, {"lead white", 3.5}, {"red earth", 2}, medium=0.2}

local b_sweep = brush{kind="filbert", width=14, stiffness=0.8}

-- 2. Sweep across the foreground to bury the smoke trail and box boulders completely
b_sweep:load(p_earth_core, 0.95)
b_sweep:stroke({{350, 445}, {450, 480}, {550, 520}, {650, 570}, {750, 630}, {820, 700}}, {pressure=0.95})
b_sweep:stroke({{380, 450}, {480, 490}, {580, 535}, {680, 590}, {770, 650}, {840, 705}}, {pressure=0.95})

-- Cover left box boulder (x ≈ 210 to 270, y ≈ 520 to 560)
b_sweep:stroke({{180, 520}, {220, 535}, {270, 545}, {310, 550}}, {pressure=0.95})
b_sweep:stroke({{190, 540}, {230, 550}, {280, 555}, {320, 560}}, {pressure=0.95})

-- Cover right box boulder (x ≈ 650 to 720, y ≈ 530 to 570)
b_sweep:stroke({{620, 530}, {670, 545}, {720, 555}, {760, 560}}, {pressure=0.95})
b_sweep:stroke({{630, 550}, {680, 560}, {730, 568}, {770, 570}}, {pressure=0.95})

-- Cover the dark rectangular board under the dolmen (x ≈ 260 to 540, y ≈ 435 to 460)
b_sweep:load(p_heath_russet, 0.95)
b_sweep:stroke({{250, 442}, {320, 446}, {390, 448}, {460, 447}, {530, 444}}, {pressure=0.95})
b_sweep:stroke({{260, 450}, {330, 454}, {400, 456}, {470, 455}, {540, 452}}, {pressure=0.95})

-- 3. Weave rich undulating banks of heather, peat, and moss across the whole barrow
local m_below_crest = rect(0, 430, 1000, 275)

local m_h_mid = (ellipse(480, 540, 260, 50) + ellipse(220, 560, 180, 45)):soften(20) * m_below_crest
work(m_h_mid, {hand="body", pile=p_peat_moss, angle=0.2, coverage=1.4, fill=true})

local m_h_warm = (ellipse(340, 620, 280, 55) + ellipse(760, 610, 240, 50)):soften(22) * m_below_crest
work(m_h_warm, {hand="body", pile=p_heath_gold, angle=-0.15, coverage=1.4, fill=true})

local m_h_base = rect(0, 650, 1000, 60):soften(15) * m_below_crest
work(m_h_base, {hand="body", pile=p_earth_core, angle=0.1, coverage=1.5, fill=true})

-- 4. Blend the barrow smoothly wet-into-wet
blend(rect(0, 430, 1000, 275), {angle=0.2})
blend(rect(0, 430, 1000, 275), {angle=-0.15})
blend(rect(0, 430, 1000, 275), {angle=0.1})

-- 5. Restore the crisp, organic barrow crest contour line
local barrow_nodes = {
  {x=-20, y=465}, {x=80, y=458}, {x=180, y=448}, {x=260, y=430},
  {x=330, y=415}, {x=385, y=408}, {x=445, y=414}, {x=520, y=430},
  {x=600, y=448}, {x=680, y=450}, {x=760, y=436}, {x=830, y=436},
  {x=910, y=448}, {x=1020, y=465}
}

local function barrow_crest(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 440
end

local b_crest = brush{kind="round", width=2.8, point=1, stiffness=0.9}
b_crest:load(p_earth_core, 0.95)
local crest_line = {}
for x = -10, 1010, 10 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest:stroke(crest_line, {pressure=0.9, ramps={0.01, 0.01}})

print("Chunk 96 complete: Barrow restored; smoke path, box boulders, and board permanently eliminated.")

--@ chunk 96
-- Chunk 97: Step 1 — Eliminating the Horizontal Shelf via below(barrow_crest) & Sculpting Granite Megalith

-- 1. Mathematical spline for the noble prehistoric barrow crest
local barrow_nodes = {
  {x=-20, y=465}, {x=80, y=458}, {x=180, y=448}, {x=260, y=430},
  {x=330, y=415}, {x=385, y=408}, {x=445, y=414}, {x=520, y=430},
  {x=600, y=448}, {x=680, y=450}, {x=760, y=436}, {x=830, y=436},
  {x=910, y=448}, {x=1020, y=465}
}

local function barrow_crest(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 440
end

local m_barrow = below(barrow_crest)

-- 2. Earth & Peat Palette
local p_earth_body   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4}, {"yellow ochre", 3}, medium=0.18}
local p_peat_dark    = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5}, {"lead white", 2}, medium=0.2}

-- 3. Cover the horizontal shelf at y = 430: Body pass covering the upper barrow slope down to y = 520
local m_upper_slope = m_barrow * rect(0, 400, 1000, 120)
work(m_upper_slope, {
  hand = "body",
  pile = p_earth_body,
  angle = 0.2,
  length = {45, 90},
  coverage = 2.0,
  fill = true,
  clip = true
})

-- Summit heather bank
local m_summit = (ellipse(385, 435, 170, 32) + ellipse(250, 448, 140, 35)):soften(15) * m_barrow
work(m_summit, {hand="body", pile=p_heath_russet, angle=0.18, coverage=1.4, fill=true})

-- Blend the barrow smoothly using m_barrow (no horizontal cutoff!)
blend(m_barrow, {angle=0.2})
blend(m_barrow, {angle=-0.15})

-- Firm, organic stroke along the natural barrow crest against the glowing mist
local b_crest = brush{kind="round", width=2.8, point=1, stiffness=0.9}
b_crest:load(p_peat_dark, 0.95)
local crest_line = {}
for x = -10, 1010, 10 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest:stroke(crest_line, {pressure=0.9, ramps={0.01, 0.01}})

-- 4. Sculpt the Megalithic Tomb as Authentic Weathered Scandinavian Granite
local p_cavern_pitch = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}
local p_granite_deep = pile{{"bone black", 9}, {"raw umber", 8}, {"smalt", 2.5}, {"red earth", 1.5}, medium=0.15}
local p_granite_body = pile{{"lead white", 10}, {"raw umber", 7}, {"bone black", 4}, {"yellow ochre", 3.5}, {"smalt", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_granite_facet= pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage    = pile{{"lead white", 14}, {"green earth", 7.0}, {"yellow ochre", 3.5}, {"raw umber", 1.2}, medium=0.25}
local p_lich_gold    = pile{{"lead white", 10}, {"yellow ochre", 9.5}, {"chrome yellow", 1.8}, {"raw umber", 1.0}, medium=0.25}

-- A. Sacred Burial Chamber Void (Solid deep shadow between the uprights)
local m_chamber = poly({{340, 404}, {452, 404}, {452, 436}, {340, 436}})
work(m_chamber, {hand="body", pile=p_cavern_pitch, angle=0, coverage=2.2, fill=true, clip=true})

-- B. Left Orthostat (Massive, rugged, leaning stone pillar, width ~30 units)
local m_lo = poly({{332, 400}, {362, 400}, {366, 438}, {334, 438}})
work(m_lo, {hand="body", pile=p_granite_body, angle=1.45, coverage=2.0, fill=true, clip=true})
local m_lo_lit = poly({{332, 400}, {344, 400}, {346, 438}, {334, 438}})
work(m_lo_lit, {hand="detail", pile=p_granite_facet, angle=1.45, coverage=1.6, fill=true, clip=true})

-- C. Right Orthostat (Massive, sturdy stone pillar, width ~32 units)
local m_ro = poly({{428, 400}, {460, 400}, {458, 438}, {426, 438}})
work(m_ro, {hand="body", pile=p_granite_body, angle=1.4, coverage=2.0, fill=true, clip=true})
local m_ro_lit = poly({{446, 400}, {460, 400}, {458, 438}, {448, 438}})
work(m_ro_lit, {hand="detail", pile=p_granite_facet, angle=1.4, coverage=1.6, fill=true, clip=true})

-- D. The Massive Granite Capstone: Heavy, angular, rugged erratic slab
local pts_cap = {
  {314, 404}, {324, 394}, {344, 384}, {374, 378}, {410, 376}, {446, 380},
  {474, 388}, {480, 398}, {466, 404}, {430, 404}, {385, 405}, {345, 404}, {314, 404}
}
local m_cap = poly(pts_cap, false)
work(m_cap, {hand="body", pile=p_granite_body, angle=0.06, coverage=2.2, fill=true, clip=true})

-- Deep cast shadow directly under the capstone overhang
local pts_undercut = {
  {314, 404}, {345, 404}, {385, 405}, {430, 404}, {466, 404}, {480, 398},
  {476, 408}, {435, 409}, {385, 410}, {340, 409}, {314, 404}
}
local m_undercut = poly(pts_undercut, false)
work(m_undercut, {hand="detail", pile=p_cavern_pitch, angle=0, coverage=2.0, fill=true, clip=true})

-- Top sky-facing facets catching twilight
local pts_topA = {
  {314, 404}, {324, 394}, {344, 384}, {374, 378}, {400, 376},
  {396, 392}, {365, 394}, {338, 398}, {314, 404}
}
local m_topA = poly(pts_topA, false)
work(m_topA, {hand="detail", pile=p_granite_lit, angle=0.04, coverage=1.6, fill=true, clip=true})

local pts_topB = {
  {400, 376}, {446, 380}, {474, 388}, {480, 398}, {466, 404},
  {440, 400}, {412, 396}, {396, 392}
}
local m_topB = poly(pts_topB, false)
work(m_topB, {hand="detail", pile=p_granite_facet, angle=-0.04, coverage=1.6, fill=true, clip=true})

-- Cleavage fracture crack
local b_crack = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{400, 376}, {396, 392}, {392, 405}}, {pressure={0.75, 0.85, 0.4}})
b_crack:stroke({{348, 383}, {352, 394}, {350, 404}}, {pressure={0.5, 0.65, 0.2}})
b_crack:stroke({{442, 380}, {444, 392}, {446, 404}}, {pressure={0.5, 0.65, 0.2}})

-- Top edge crystalline highlights catching twilight
b_crack:load(p_granite_lit, 0.95)
b_crack:stroke({{314, 404}, {324, 394}, {344, 384}, {374, 378}, {400, 376}}, {pressure={0.3, 0.8, 0.7, 0.85, 0.4}})
b_crack:stroke({{402, 377}, {446, 380}, {474, 388}}, {pressure={0.4, 0.8, 0.75, 0.3}})

-- Lichens on Capstone
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local lichens = {
  {335, 388, p_lich_sage, 2.2}, {362, 382, p_lich_gold, 2.5},
  {380, 388, p_lich_sage, 1.8}, {420, 382, p_lich_gold, 2.4},
  {445, 386, p_lich_sage, 2.0}, {465, 392, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichens) do
  b_facet:load(lp[3], 0.75)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- E. Flanking Glacial Boulders
local m_sl = poly({{296, 436}, {304, 418}, {320, 412}, {328, 420}, {324, 438}, {298, 438}}, false)
work(m_sl, {hand="detail", pile=p_granite_body, angle=0.3, coverage=1.8, fill=true, clip=true})
local m_sl_lit = poly({{304, 418}, {320, 412}, {328, 420}, {318, 423}, {308, 422}}, false)
work(m_sl_lit, {hand="detail", pile=p_granite_facet, angle=0.15, coverage=1.5, fill=true, clip=true})

local m_sr = poly({{466, 436}, {476, 416}, {502, 412}, {514, 424}, {506, 438}, {470, 438}}, false)
work(m_sr, {hand="detail", pile=p_granite_body, angle=-0.2, coverage=1.8, fill=true, clip=true})
local m_sr_lit = poly({{476, 416}, {502, 412}, {514, 424}, {500, 423}, {482, 421}}, false)
work(m_sr_lit, {hand="detail", pile=p_granite_facet, angle=-0.1, coverage=1.5, fill=true, clip=true})

-- Bed the base of the stones into rich peat and heather (NO horizontal trench!)
local b_turf = brush{kind="filbert", width=5.5, stiffness=0.85}
b_turf:load(p_peat_dark, 0.95)
b_turf:stroke({{280, 438}, {330, 440}, {380, 442}, {430, 442}, {480, 440}, {530, 438}}, {pressure=0.95})

print("Chunk 97 complete: Horizontal shelf eliminated via below(barrow_crest); Megalithic Tomb sculpted.")

--@ chunk 97
-- Chunk 98: The Veteran Gnarled Oak & Windswept Companion Tree — Master Botanical Brushwork

-- 1. Wood, Bark, Stag Wood, and Foliage Piles
local p_bark_core   = pile{{"bone black", 14}, {"raw umber", 8}, {"red earth", 2.0}, medium=0.15}
local p_bark_mid    = pile{{"raw umber", 8}, {"bone black", 5}, {"yellow ochre", 4.0}, {"lead white", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_bark_ridge  = pile{{"yellow ochre", 7}, {"lead white", 5.0}, {"raw umber", 4.0}, {"bone black", 1.2}, medium=0.2}
local p_hollow_pitch= pile{{"bone black", 14}, {"raw umber", 6}, medium=0.1}
local p_woundwood   = pile{{"raw umber", 6}, {"yellow ochre", 5.5}, {"lead white", 3.5}, {"red earth", 2.0}, medium=0.2}
local p_stag_shade  = pile{{"raw umber", 8}, {"bone black", 5}, {"lead white", 5}, {"yellow ochre", 2.5}, medium=0.2}
local p_stag_bleach = pile{{"lead white", 18}, {"raw umber", 3.5}, {"yellow ochre", 2.5}, {"smalt", 1.0}, medium=0.22}
local p_peat_earth  = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, {"green earth", 2.5}, medium=0.15}

local p_leaf_russet = pile{{"yellow ochre", 7}, {"red earth", 6.5}, {"raw umber", 4.0}, {"lead white", 1.5}, medium=0.22}
local p_leaf_gold   = pile{{"yellow ochre", 9}, {"chrome yellow", 2.5}, {"lead white", 4.0}, {"vermilion", 0.4}, medium=0.25}

local b_bole   = brush{kind="filbert", width=7.0, stiffness=0.9}
local b_limb   = brush{kind="filbert", width=4.5, stiffness=0.85}
local b_branch = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_twig   = brush{kind="round", width=1.4, point=1, stiffness=0.85}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}
local b_leaf   = brush{kind="round", width=1.6, point=1, stiffness=0.8}

-- 2. Master Ancient Oak: Solid, Muscular Trunk & Flaring Root Knees (Rooted at x ≈ 230 to 295, y ≈ 430 to 452)
b_bole:load(p_bark_core, 0.95)
-- Western flaring root claw gripping the barrow
b_bole:stroke({{225, 452}, {240, 442}, {255, 428}, {264, 405}, {266, 375}}, {pressure={0.95, 0.9, 0.85, 0.8, 0.75}})
-- Central anchoring root
b_bole:stroke({{258, 454}, {264, 438}, {268, 415}, {270, 385}, {270, 360}}, {pressure={0.95, 0.95, 0.9, 0.85, 0.8}})
-- Eastern root buttress gripping into the barrow slope near the dolmen
b_bole:stroke({{310, 446}, {296, 438}, {284, 422}, {278, 395}, {274, 365}}, {pressure={0.95, 0.9, 0.85, 0.8, 0.75}})

-- Solid body fill of the trunk core (width ~35-40 units)
b_bole:stroke({{248, 442}, {254, 418}, {258, 385}, {262, 355}}, {pressure=0.95})
b_bole:stroke({{274, 442}, {275, 418}, {274, 385}, {272, 355}}, {pressure=0.95})
b_bole:stroke({{290, 442}, {288, 418}, {282, 385}, {276, 355}}, {pressure=0.95})

-- Anchor root toes deep into dark peat and heather
b_bole:load(p_peat_earth, 0.95)
b_bole:stroke({{215, 456}, {245, 452}, {280, 450}, {320, 446}}, {pressure=0.95})

-- Bark furrows and raised ridges
b_twig:load(p_hollow_pitch, 0.95)
b_twig:stroke({{246, 444}, {252, 426}, {256, 402}, {260, 370}}, {pressure={0.85, 0.65, 0.3}})
b_twig:stroke({{266, 446}, {268, 428}, {270, 402}, {268, 366}}, {pressure={0.85, 0.7, 0.35}})
b_twig:stroke({{286, 442}, {284, 424}, {280, 398}, {275, 368}}, {pressure={0.85, 0.65, 0.35}})

b_twig:load(p_bark_ridge, 0.85)
b_twig:stroke({{238, 448}, {246, 435}, {252, 412}, {256, 380}}, {pressure={0.3, 0.75, 0.6, 0.2}})
b_twig:stroke({{258, 448}, {262, 430}, {265, 405}, {265, 372}}, {pressure={0.3, 0.7, 0.6, 0.2}})
b_twig:stroke({{274, 448}, {274, 430}, {274, 402}, {271, 368}}, {pressure={0.3, 0.65, 0.6, 0.2}})
b_twig:stroke({{294, 444}, {290, 428}, {284, 404}, {278, 372}}, {pressure={0.3, 0.7, 0.6, 0.2}})

-- Hollow cavity
b_twig:load(p_hollow_pitch, 0.95)
b_twig:stroke({{268, 424}, {270, 406}, {268, 392}}, {pressure=0.85})
b_twig:load(p_woundwood, 0.85)
b_twig:stroke({{265, 426}, {266, 408}, {272, 392}}, {pressure={0.5, 0.85, 0.4}})
b_twig:stroke({{274, 422}, {275, 402}, {271, 392}}, {pressure={0.5, 0.85, 0.4}})

-- 3. The Living Eastern Bough: Arching lovingly over the left side of the Dolmen to shelter it
-- Main muscular arch (starts at x=274, y=365 with branch collar, reaches to x=375, y=335)
b_limb:load(p_bark_core, 0.95)
b_limb:stroke({
  {274, 365}, {305, 355}, {340, 344}, {375, 335}
}, {pressure={0.95, 0.85, 0.7, 0.45}, ramps={0.02, 0.05}})

-- Upper bark highlight
b_branch:load(p_bark_mid, 0.85)
b_branch:stroke({{276, 360}, {306, 350}, {340, 339}, {372, 330}}, {pressure={0.4, 0.7, 0.6, 0.25}})

-- Secondary boughs off Eastern limb:
-- E1: Upward arching bough into the twilight
b_branch:load(p_bark_core, 0.95)
b_branch:stroke({
  {340, 344}, {358, 320}, {376, 292}, {392, 264}
}, {pressure={0.85, 0.7, 0.55, 0.25}, ramps={0.02, 0.1}})
b_twig:load(p_bark_core, 0.85)
b_twig:stroke({{358, 320}, {350, 298}, {344, 276}}, {pressure={0.7, 0.2}})
b_twig:stroke({{376, 292}, {388, 272}, {396, 252}}, {pressure={0.65, 0.2}})

-- E2: Sheltering branch drooping gently over the left of the capstone
b_branch:load(p_bark_core, 0.9)
b_branch:stroke({
  {375, 335}, {395, 348}, {416, 358}, {435, 366}
}, {pressure={0.8, 0.65, 0.5, 0.2}, ramps={0.02, 0.1}})
b_twig:load(p_bark_core, 0.85)
b_twig:stroke({{416, 358}, {425, 372}, {432, 384}}, {pressure={0.6, 0.2}})

-- 4. The Living Western Bough: Reaching west over the barrow slope
b_limb:load(p_bark_core, 0.95)
b_limb:stroke({
  {262, 365}, {230, 352}, {198, 338}, {165, 324}, {135, 310}
}, {pressure={0.9, 0.8, 0.65, 0.5, 0.25}, ramps={0.02, 0.05}})

-- W1: Upward western bough
b_branch:load(p_bark_core, 0.9)
b_branch:stroke({
  {198, 338}, {185, 312}, {172, 282}, {158, 252}
}, {pressure={0.8, 0.65, 0.5, 0.25}, ramps={0.02, 0.1}})
b_twig:load(p_bark_core, 0.85)
b_twig:stroke({{185, 312}, {194, 292}, {200, 272}}, {pressure={0.65, 0.2}})

-- W2: Drooping western bough
b_branch:load(p_bark_core, 0.85)
b_branch:stroke({
  {165, 324}, {148, 345}, {130, 364}
}, {pressure={0.75, 0.55, 0.2}, ramps={0.02, 0.1}})

-- 5. The Bleached Stag-Head Crown aloft (Weathered dead boughs in twilight)
b_branch:load(p_stag_shade, 0.95)
b_branch:stroke({{268, 345}, {266, 305}, {264, 265}, {262, 225}, {258, 185}}, {pressure={0.85, 0.75, 0.6, 0.4, 0.15}, ramps={0.02, 0.05}})
b_branch:stroke({{266, 295}, {280, 260}, {296, 222}, {308, 188}}, {pressure={0.8, 0.65, 0.5, 0.25}, ramps={0.02, 0.05}})
b_branch:stroke({{265, 300}, {250, 265}, {234, 228}, {220, 192}}, {pressure={0.8, 0.65, 0.5, 0.25}, ramps={0.02, 0.05}})

-- Bleached wood highlights & splintered tips
b_twig:load(p_stag_bleach, 0.95)
b_twig:stroke({{266, 335}, {264, 298}, {262, 260}, {260, 222}, {256, 185}}, {pressure={0.35, 0.7, 0.55, 0.35, 0.1}})
b_twig:stroke({{266, 290}, {281, 258}, {297, 220}, {309, 188}}, {pressure={0.3, 0.65, 0.5, 0.25}})
b_twig:stroke({{263, 295}, {248, 262}, {232, 225}, {219, 192}}, {pressure={0.3, 0.65, 0.5, 0.25}})

b_rigger:load(p_stag_bleach, 0.95)
b_rigger:stroke({{258, 188}, {256, 178}}, {pressure={0.45, 0.05}})
b_rigger:stroke({{258, 188}, {261, 180}}, {pressure={0.35, 0.05}})
b_rigger:stroke({{308, 190}, {312, 182}}, {pressure={0.4, 0.05}})
b_rigger:stroke({{220, 194}, {216, 185}}, {pressure={0.4, 0.05}})

-- 6. Crooked Sympodial Twigs
local oak_twigs = {
  -- East
  {{392, 264}, {404, 246}, {415, 230}},
  {{392, 264}, {384, 246}, {378, 230}},
  {{344, 276}, {338, 260}, {332, 245}},
  {{435, 366}, {446, 378}, {456, 390}},
  -- West
  {{158, 252}, {146, 234}, {134, 218}},
  {{158, 252}, {170, 235}, {180, 220}},
  {{135, 310}, {120, 295}, {106, 282}},
  {{135, 310}, {124, 325}, {112, 340}},
  {{130, 364}, {116, 382}, {102, 398}}
}
for _, tw in ipairs(oak_twigs) do
  b_rigger:load(p_bark_core, 0.85)
  b_rigger:stroke(tw, {pressure={0.65, 0.15}, ramps={0.05, 0.2}})
end

-- 7. Clinging Marcescent Autumn Leaves (*Laub*) along the living twigs
local marcescent_sprays = {
  -- East
  {360, 320, 18}, {380, 295, 18}, {400, 265, 18}, {412, 240, 15},
  {390, 345, 18}, {420, 360, 16}, {440, 375, 14}, {340, 345, 16},
  -- West
  {180, 310, 18}, {165, 275, 18}, {145, 240, 16}, {125, 225, 14},
  {125, 305, 16}, {110, 290, 14}, {140, 340, 16}, {120, 365, 14}
}
for _, cl in ipairs(marcescent_sprays) do
  local cx, cy, n = cl[1], cl[2], cl[3]
  for i = 1, n do
    local ox = rand(-12, 12)
    local oy = rand(-9, 9)
    local p = (rand(0, 1) < 0.45) and p_leaf_russet or p_leaf_gold
    b_leaf:load(p, 0.75)
    b_leaf:touch(cx + ox, cy + oy, {pressure=rand(0.35, 0.65), drag={rand(1.2, 2.4), rand(1.1, 1.6)}})
  end
end

-- 8. The Windswept Companion Tree on the Right Knoll (x ≈ 755 to 800)
b_branch:load(p_bark_core, 0.95)
b_branch:stroke({
  {766, 438}, {762, 405}, {756, 370}, {750, 335}, {746, 300}, {744, 270}
}, {pressure={0.85, 0.75, 0.6, 0.45, 0.3, 0.12}, ramps={0.02, 0.05}})

local comp_branches = {
  {{750, 335}, {772, 320}, {798, 305}, {824, 292}, {846, 280}},
  {{798, 305}, {814, 285}, {826, 265}, {836, 245}},
  {{746, 300}, {760, 278}, {772, 255}, {782, 232}},
  {{744, 270}, {752, 248}, {760, 225}}
}
for _, b in ipairs(comp_branches) do
  b_twig:load(p_bark_core, 0.9)
  b_twig:stroke(b, {pressure={0.7, 0.2}, ramps={0.02, 0.1}})
end

local comp_twigs = {
  {{846, 280}, {860, 274}, {872, 268}},
  {{836, 245}, {846, 232}, {854, 220}},
  {{782, 232}, {792, 218}, {800, 205}}
}
for _, tw in ipairs(comp_twigs) do
  b_rigger:load(p_bark_core, 0.85)
  b_rigger:stroke(tw, {pressure={0.6, 0.12}, ramps={0.02, 0.15}})
end

local comp_leaves = {
  {805, 310, 12}, {835, 292, 14}, {855, 280, 10}, {820, 265, 12},
  {770, 255, 12}, {775, 230, 10}
}
for _, cl in ipairs(comp_leaves) do
  local cx, cy, n = cl[1], cl[2], cl[3]
  for i = 1, n do
    local ox = rand(-10, 10)
    local oy = rand(-8, 8)
    local p = (rand(0, 1) < 0.5) and p_leaf_russet or p_leaf_gold
    b_leaf:load(p, 0.7)
    b_leaf:touch(cx + ox, cy + oy, {pressure=rand(0.3, 0.65), drag={rand(1.0, 2.0), 1.4}})
  end
end

print("Chunk 98 complete: Veteran Gnarled Oak & Companion Tree painted with authentic botanical brushwork.")

--@ chunk 98
-- Chunk 99: Advancing time to touch-dry, then establishing the Master Friedrichian Sky & Barrow

-- 1. Dry the canvas completely
wait(2 * 24 * 60)

-- 2. Sky Piles
local p_sky_zenith = pile{{"lead white", 18}, {"cobalt blue", 6.2}, {"smalt", 4.5}, {"raw umber", 0.6}, {"bone black", 0.25}, medium=0.22}
local p_sky_upper  = pile{{"lead white", 26}, {"cobalt blue", 3.2}, {"smalt", 3.2}, {"yellow ochre", 0.8}, {"raw umber", 0.3}, medium=0.22}
local p_sky_mid    = pile{{"lead white", 28}, {"yellow ochre", 3.5}, {"chrome yellow", 0.9}, {"smalt", 1.0}, {"raw umber", 0.2}, {"vermilion", 0.1}, medium=0.22}
local p_sky_glow   = pile{{"lead white", 26}, {"yellow ochre", 5.2}, {"chrome yellow", 2.2}, {"vermilion", 0.5}, {"raw umber", 0.25}, medium=0.22}

-- 3. Four overlapping bands covering x = 0 to 1000 from zenith down behind the barrow (y = 425)
local m1 = rect(0, 0, 1000, 145):soften(30)
work(m1, {hand="body", pile=p_sky_zenith, angle=0, order="across", coverage=2.2, fill=true})

local m2 = rect(0, 105, 1000, 150):soften(30)
work(m2, {hand="body", pile=p_sky_upper, angle=0, order="across", coverage=2.2, fill=true})

local m3 = rect(0, 215, 1000, 140):soften(30)
work(m3, {hand="body", pile=p_sky_mid, angle=0, order="across", coverage=2.2, fill=true})

local m4 = rect(0, 315, 1000, 125):soften(25)
work(m4, {hand="body", pile=p_sky_glow, angle=0, order="across", coverage=2.2, fill=true})

-- 4. Seamless badger blending across the full canvas width
local m_sky_all = rect(0, 0, 1000, 425)
blend(m_sky_all, {angle=1.57})
blend(m_sky_all, {angle=0})

-- 5. Distant low Baltic hills silhouetted in soft atmospheric haze
local p_dist_coast = pile{{"lead white", 15}, {"smalt", 5.0}, {"raw umber", 3.0}, {"red earth", 0.9}, {"bone black", 0.4}, medium=0.25}
local pts_distant_land = {
  {0, 464}, {100, 460}, {220, 454}, {340, 452}, {460, 448},
  {580, 451}, {700, 447}, {820, 452}, {920, 458}, {1000, 464},
  {1000, 475}, {0, 475}
}
local m_coast = poly(pts_distant_land, true):soften(2.5)
work(m_coast, {hand="body", pile=p_dist_coast, angle=0, coverage=1.2, fill=true, clip=true})

-- 6. Calm water sheen and valley mist pooling over the water
local p_water_mist = pile{{"lead white", 26}, {"yellow ochre", 3.2}, {"smalt", 1.8}, {"raw umber", 0.3}, medium=0.3}
local m_water = rect(0, 435, 1000, 30):soften(4)
work(m_water, {hand="glaze", pile=p_water_mist, angle=0, coverage=1.1, fill=false})
blend(rect(0, 432, 1000, 34), {angle=0})

-- 7. Waxing crescent moon and solitary evening star (Venus)
local p_moon_pure = pile{{"lead white", 34}, {"yellow ochre", 1.0}, {"chrome yellow", 0.4}, medium=0.12}
local b_rigger = brush{kind="round", width=1.3, point=1, stiffness=0.85}
b_rigger:load(p_moon_pure, 0.95)
b_rigger:stroke({
  {645.2, 105.5}, {641.5, 108.0}, {639.0, 112.5}, {640.2, 117.0}, {644.0, 119.5}
}, {pressure={0.08, 0.85, 1.0, 0.8, 0.08}, ramps={0.05, 0.05}})

local b_star = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_star:load(p_moon_pure, 1.0)
b_star:touch(688, 148, {pressure=0.6})

-- 8. The Noble Prehistoric Barrow Mound
local barrow_nodes = {
  {x=-20, y=465}, {x=80, y=458}, {x=180, y=448}, {x=260, y=430},
  {x=330, y=415}, {x=385, y=408}, {x=445, y=414}, {x=520, y=430},
  {x=600, y=448}, {x=680, y=450}, {x=760, y=436}, {x=830, y=436},
  {x=910, y=448}, {x=1020, y=465}
}

local function barrow_crest(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 440
end

local m_barrow = below(barrow_crest)

local p_earth_body   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4}, {"yellow ochre", 3}, medium=0.18}
local p_peat_dark    = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5}, {"lead white", 2}, medium=0.2}

-- Solid body fill across the entire barrow from the crest down to canvas bottom
work(m_barrow, {
  hand = "body",
  pile = p_earth_body,
  angle = 0.22,
  length = {50, 110},
  coverage = 2.4,
  fill = true,
  clip = true
})

-- Summit heather bank
local m_summit = (ellipse(385, 435, 170, 32) + ellipse(250, 448, 140, 35)):soften(15) * m_barrow
work(m_summit, {hand="body", pile=p_heath_russet, angle=0.18, coverage=1.4, fill=true})

-- Blend barrow smoothly (no horizontal cutoffs!)
blend(m_barrow, {angle=0.2})
blend(m_barrow, {angle=-0.15})

-- Firm, organic stroke along the natural barrow crest against the glowing mist
local b_crest = brush{kind="round", width=2.8, point=1, stiffness=0.9}
b_crest:load(p_peat_dark, 0.95)
local crest_line = {}
for x = -10, 1010, 10 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest:stroke(crest_line, {pressure=0.9, ramps={0.01, 0.01}})

print("Chunk 99 complete: Master sky restored, moon and star shining, barrow mound sculpted.")

--@ chunk 99
-- Chunk 100: Painting the Monumental Megalithic Tomb (Hünengrab), Boulders, Grasses & Thistles

-- 1. Piles for Ancient Swedish Glacial Granite, Peat, and Heath
local p_cavern_pitch = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}
local p_granite_deep = pile{{"bone black", 9}, {"raw umber", 8}, {"smalt", 2.5}, {"red earth", 1.5}, medium=0.15}
local p_granite_body = pile{{"lead white", 9}, {"raw umber", 7.5}, {"bone black", 4.5}, {"yellow ochre", 3.5}, {"smalt", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_granite_facet= pile{{"lead white", 16}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 24}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage    = pile{{"lead white", 14}, {"green earth", 7.0}, {"yellow ochre", 3.5}, {"raw umber", 1.2}, medium=0.25}
local p_lich_gold    = pile{{"lead white", 10}, {"yellow ochre", 9.5}, {"chrome yellow", 1.8}, {"raw umber", 1.0}, medium=0.25}

local p_peat_dark    = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, {"yellow ochre", 1.5}, medium=0.15}
local p_heather_bank = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5.0}, {"lead white", 2.0}, {"bone black", 1.0}, medium=0.2}
local p_straw_sharp  = pile{{"lead white", 16}, {"yellow ochre", 9}, {"chrome yellow", 1.2}, {"raw umber", 1.0}, medium=0.22}
local p_straw_shade  = pile{{"yellow ochre", 7}, {"raw umber", 6}, {"bone black", 2.5}, {"lead white", 2.0}, medium=0.2}
local p_thistle_dark = pile{{"bone black", 12}, {"raw umber", 8}, medium=0.15}
local p_twilight_rim = pile{{"lead white", 26}, {"yellow ochre", 3.5}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.2}

-- 2. The Megalithic Tomb (Hünengrab) atop the Barrow Summit
-- A. Sacred Chamber Void (Deep velvety black cavern portal between orthostats)
local m_chamber = poly({{345, 400}, {450, 400}, {450, 432}, {345, 432}})
work(m_chamber, {hand="body", pile=p_cavern_pitch, angle=0, coverage=2.5, fill=true, clip=true})

-- B. Left Orthostat (Massive, rugged upright stone block, width ~26 units)
local m_lo = poly({{335, 396}, {362, 396}, {366, 430}, {338, 430}})
work(m_lo, {hand="body", pile=p_granite_body, angle=1.45, coverage=2.2, fill=true, clip=true})
local m_lo_lit = poly({{335, 396}, {345, 396}, {347, 430}, {338, 430}})
work(m_lo_lit, {hand="detail", pile=p_granite_facet, angle=1.45, coverage=1.6, fill=true, clip=true})

-- C. Right Orthostat (Massive, rugged upright stone block, width ~28 units)
local m_ro = poly({{430, 396}, {458, 396}, {456, 430}, {428, 430}})
work(m_ro, {hand="body", pile=p_granite_body, angle=1.4, coverage=2.2, fill=true, clip=true})
local m_ro_lit = poly({{446, 396}, {458, 396}, {456, 430}, {448, 430}})
work(m_ro_lit, {hand="detail", pile=p_granite_facet, angle=1.4, coverage=1.6, fill=true, clip=true})

-- D. The Monumental Granite Capstone: Heavy, angular, rugged erratic slab
local pts_cap = {
  {318, 400}, {326, 390}, {344, 380}, {374, 375}, {408, 373}, {442, 376},
  {468, 384}, {475, 394}, {460, 400}, {425, 401}, {385, 401}, {345, 400}, {318, 400}
}
local m_cap = poly(pts_cap, false) -- angular facets!
work(m_cap, {hand="body", pile=p_granite_body, angle=0.06, coverage=2.2, fill=true, clip=true})

-- Deep cast shadow directly under capstone overhang
local pts_undercut = {
  {318, 400}, {345, 400}, {385, 401}, {425, 401}, {460, 400}, {475, 394},
  {470, 405}, {430, 406}, {385, 406}, {340, 405}, {318, 400}
}
local m_undercut = poly(pts_undercut, false)
work(m_undercut, {hand="detail", pile=p_cavern_pitch, angle=0, coverage=2.0, fill=true, clip=true})

-- Top sky-facing facets catching twilight
local pts_topA = {
  {318, 400}, {326, 390}, {344, 380}, {374, 375}, {400, 373},
  {396, 388}, {365, 390}, {338, 394}, {318, 400}
}
local m_topA = poly(pts_topA, false)
work(m_topA, {hand="detail", pile=p_granite_lit, angle=0.04, coverage=1.6, fill=true, clip=true})

local pts_topB = {
  {400, 373}, {442, 376}, {468, 384}, {475, 394}, {460, 400},
  {435, 396}, {412, 392}, {396, 388}
}
local m_topB = poly(pts_topB, false)
work(m_topB, {hand="detail", pile=p_granite_facet, angle=-0.04, coverage=1.6, fill=true, clip=true})

-- Fracture cleavage cracks cleaving the capstone
local b_crack = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{400, 373}, {396, 388}, {392, 402}}, {pressure={0.75, 0.85, 0.4}})
b_crack:stroke({{348, 380}, {352, 390}, {350, 400}}, {pressure={0.5, 0.65, 0.2}})
b_crack:stroke({{440, 378}, {442, 388}, {444, 400}}, {pressure={0.5, 0.65, 0.2}})

-- Top ridge highlights catching the silvery sky
b_crack:load(p_granite_lit, 0.95)
b_crack:stroke({{318, 400}, {326, 390}, {344, 380}, {374, 375}, {400, 373}}, {pressure={0.3, 0.8, 0.7, 0.85, 0.4}})
b_crack:stroke({{402, 374}, {442, 376}, {468, 384}}, {pressure={0.4, 0.8, 0.75, 0.3}})

-- Lichen touches on Capstone
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local lichens = {
  {338, 385, p_lich_sage, 2.2}, {365, 380, p_lich_gold, 2.5},
  {384, 385, p_lich_sage, 1.8}, {418, 379, p_lich_gold, 2.4},
  {444, 382, p_lich_sage, 2.0}, {462, 388, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichens) do
  b_facet:load(lp[3], 0.8)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- E. Flanking Glacial Boulders
local m_sl = poly({{300, 430}, {308, 412}, {324, 408}, {332, 416}, {326, 432}, {302, 432}}, false)
work(m_sl, {hand="detail", pile=p_granite_body, angle=0.3, coverage=1.8, fill=true, clip=true})
local m_sl_lit = poly({{308, 412}, {324, 408}, {332, 416}, {322, 419}, {312, 418}}, false)
work(m_sl_lit, {hand="detail", pile=p_granite_facet, angle=0.15, coverage=1.5, fill=true, clip=true})

local m_sr = poly({{464, 430}, {474, 412}, {498, 408}, {510, 418}, {504, 432}, {468, 432}}, false)
work(m_sr, {hand="detail", pile=p_granite_body, angle=-0.2, coverage=1.8, fill=true, clip=true})
local m_sr_lit = poly({{474, 412}, {498, 408}, {510, 418}, {498, 418}, {482, 416}}, false)
work(m_sr_lit, {hand="detail", pile=p_granite_facet, angle=-0.1, coverage=1.5, fill=true, clip=true})

-- Bed the base of the stones deeply into dark peat and late-autumn heather
local b_block = brush{kind="filbert", width=6.5, stiffness=0.85}
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{285, 428}, {330, 430}, {375, 432}, {420, 432}, {465, 430}, {515, 428}}, {pressure=0.95})
b_block:load(p_heather_bank, 0.9)
b_block:stroke({{295, 434}, {340, 436}, {390, 438}, {440, 437}, {490, 435}}, {pressure=0.9})

-- 3. Glacial Erratic Boulders (*Findlinge*) in the Mid-Ground
-- Left Boulder (x ≈ 215 to 265, y ≈ 520 to 548)
local pts_b1 = {{214, 540}, {226, 524}, {252, 520}, {266, 530}, {262, 546}, {238, 550}, {216, 546}}
local m_b1 = poly(pts_b1, false)
work(m_b1, {hand="body", pile=p_granite_body, angle=0.2, coverage=1.8, fill=true, clip=true})
local m_b1_sh = poly({{214, 540}, {234, 546}, {262, 546}, {238, 550}, {216, 546}}, false)
work(m_b1_sh, {hand="detail", pile=p_granite_deep, angle=0.2, coverage=1.6, fill=true, clip=true})
local m_b1_lit = poly({{226, 524}, {252, 520}, {266, 530}, {254, 534}, {230, 532}}, false)
work(m_b1_lit, {hand="detail", pile=p_granite_lit, angle=0.05, coverage=1.6, fill=true, clip=true})
b_crack:load(p_granite_deep, 0.85)
b_crack:stroke({{238, 522}, {242, 534}, {240, 548}}, {pressure={0.5, 0.7, 0.2}})
b_crack:load(p_lich_sage, 0.8)
b_crack:touch(234, 526, {pressure=0.6, drag={2.0, 0.1}})
b_crack:load(p_lich_gold, 0.75)
b_crack:touch(252, 526, {pressure=0.55, drag={1.8, 0.1}})

-- Right Boulder (x ≈ 660 to 715, y ≈ 530 to 558)
local pts_b2 = {{660, 550}, {672, 534}, {700, 530}, {714, 542}, {710, 556}, {684, 560}, {662, 556}}
local m_b2 = poly(pts_b2, false)
work(m_b2, {hand="body", pile=p_granite_body, angle=-0.2, coverage=1.8, fill=true, clip=true})
local m_b2_sh = poly({{660, 550}, {680, 556}, {710, 556}, {684, 560}, {662, 556}}, false)
work(m_b2_sh, {hand="detail", pile=p_granite_deep, angle=-0.2, coverage=1.6, fill=true, clip=true})
local m_b2_lit = poly({{672, 534}, {700, 530}, {714, 542}, {698, 544}, {678, 542}}, false)
work(m_b2_lit, {hand="detail", pile=p_granite_lit, angle=-0.05, coverage=1.6, fill=true, clip=true})
b_crack:load(p_granite_deep, 0.85)
b_crack:stroke({{686, 532}, {688, 544}, {686, 556}}, {pressure={0.5, 0.7, 0.2}})
b_crack:load(p_lich_sage, 0.8)
b_crack:touch(682, 536, {pressure=0.6, drag={2.0, 0.1}})
b_crack:load(p_lich_gold, 0.75)
b_crack:touch(696, 534, {pressure=0.55, drag={1.8, 0.1}})

-- Bed boulders naturally into the heather turf
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{202, 548}, {232, 554}, {266, 552}, {284, 545}}, {pressure=0.9})
b_block:stroke({{648, 556}, {678, 562}, {712, 560}, {730, 550}}, {pressure=0.9})

-- 4. Windswept Bent-Grass Tufts & Carline Thistles
local function draw_curved_grass(bx, by, len, ang, bend, p)
  local mx = bx + len * 0.55 * math.cos(ang + bend * 0.5)
  local my = by - len * 0.55 * math.sin(ang + bend * 0.5)
  local tx = bx + len * math.cos(ang + bend)
  local ty = by - len * math.sin(ang + bend)
  local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}
  b_rigger:load(p, 0.75)
  b_rigger:stroke({{bx, by}, {mx, my}, {tx, ty}}, {pressure={0.7, 0.05}, ramps={0.05, 0.3}})
end

local function draw_grass_cluster(cx, cy, n, h, lean)
  for i = 1, n do
    local bx = cx + rand(-5, 5)
    local by = cy + rand(-2, 2)
    local len = h * rand(0.75, 1.25)
    local ang = 1.57 + rand(-0.25, 0.25) + lean * 0.3
    local bnd = lean * rand(0.2, 0.5)
    local p = (rand(0, 1) < 0.65) and p_straw_sharp or p_straw_shade
    draw_curved_grass(bx, by, len, ang, bnd, p)
  end
end

-- Grasses around boulders and across the barrow crest
local grasses = {
  {204, 548, 7, 16, -0.4}, {228, 552, 6, 15, -0.2}, {264, 550, 7, 16, 0.3},
  {650, 556, 7, 16, -0.4}, {678, 562, 6, 15, -0.1}, {710, 560, 7, 15, 0.3},
  {120, 452, 7, 16, -0.4}, {180, 444, 7, 15, -0.3}, {240, 430, 6, 14, -0.2},
  {310, 420, 6, 14, -0.1}, {475, 420, 6, 14, 0.2}, {540, 434, 7, 16, 0.3},
  {620, 446, 7, 16, 0.3}, {700, 448, 6, 15, 0.4}, {780, 436, 7, 16, 0.4}
}
for _, g in ipairs(grasses) do draw_grass_cluster(g[1], g[2], g[3], g[4], g[5]) end

-- Dried carline thistle skeletons along the barrow crest silhouetted against glowing mist
local thistles = {{145, 450, 22}, {565, 440, 20}, {845, 438, 18}}
for _, th in ipairs(thistles) do
  local x, y, h = th[1], th[2], th[3]
  b_block:load(p_thistle_dark, 0.85)
  b_block:stroke({{x, y}, {x, y - h*0.6}, {x + rand(-2, 2), y - h}}, {pressure={0.7, 0.15}, ramps={0.05, 0.2}})
  b_facet:load(p_straw_sharp, 0.8)
  b_facet:stroke({{x - 3, y - h}, {x + 3, y - h}}, {pressure=0.4})
end

-- 5. Subtle Twilight Rim-Lights on Stone Top Facets
local b_rigger = brush{kind="round", width=1.0, point=1, stiffness=0.85}
b_rigger:load(p_twilight_rim, 0.7)
b_rigger:stroke({{318, 400}, {326, 390}, {344, 380}, {374, 375}, {408, 373}, {442, 376}, {468, 384}}, {pressure={0.35, 0.75, 0.7, 0.8, 0.7, 0.3}})

print("Chunk 100 complete: Megalithic Tomb, erratic boulders, grasses, and thistles painted.")

--@ chunk 100
-- Chunk 101: Eliminating Fire Hydrants, Shoe Boxes, and Trench — Perfecting the Pure Friedrich Landscape

-- 1. Piles
local p_mist_glow    = pile{{"lead white", 26}, {"yellow ochre", 4.5}, {"chrome yellow", 1.5}, {"smalt", 1.2}, {"raw umber", 0.3}, medium=0.25}
local p_earth_body   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4}, {"yellow ochre", 3}, medium=0.18}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5}, {"lead white", 2}, medium=0.2}
local p_peat_moss    = pile{{"green earth", 6}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}
local p_heath_gold   = pile{{"yellow ochre", 8}, {"raw umber", 4}, {"lead white", 3.5}, {"red earth", 2}, medium=0.2}

local p_straw_sharp  = pile{{"lead white", 16}, {"yellow ochre", 9}, {"chrome yellow", 1.2}, {"raw umber", 1.0}, medium=0.22}
local p_straw_shade  = pile{{"yellow ochre", 7}, {"raw umber", 6}, {"bone black", 2.5}, {"lead white", 2.0}, medium=0.2}
local p_thistle_hair = pile{{"raw umber", 8}, {"bone black", 6}, medium=0.2}

local b_block  = brush{kind="filbert", width=8.0, stiffness=0.85}
local b_detail = brush{kind="round", width=2.0, point=1, stiffness=0.85}
local b_rigger = brush{kind="round", width=0.7, point=1, stiffness=0.9}

-- 2. Cover the three black fire hydrants on the ridge with glowing horizon mist paint
-- Left hydrant at x ≈ 145, y ≈ 445
b_detail:load(p_mist_glow, 0.95)
b_detail:stroke({{140, 442}, {145, 452}, {150, 442}}, {pressure=0.9})
-- Center hydrant at x ≈ 565, y ≈ 438
b_detail:stroke({{560, 432}, {565, 444}, {570, 432}}, {pressure=0.9})
-- Right hydrant at x ≈ 845, y ≈ 436
b_detail:stroke({{840, 430}, {845, 442}, {850, 430}}, {pressure=0.9})

-- Blend the mist patches softly into the horizon sky
blend(ellipse(145, 445, 12, 10), {angle=0})
blend(ellipse(565, 438, 12, 10), {angle=0})
blend(ellipse(845, 436, 12, 10), {angle=0})

-- Re-stroke the natural barrow crest contour at those spots with peaty earth
b_rigger:load(p_earth_body, 0.9)
b_rigger:stroke({{130, 452}, {145, 450}, {160, 447}}, {pressure=0.8})
b_rigger:stroke({{550, 436}, {565, 438}, {580, 442}}, {pressure=0.8})
b_rigger:stroke({{830, 436}, {845, 438}, {860, 442}}, {pressure=0.8})

-- 3. Cover the two cardboard shoe boxes in the foreground with rich peaty heather
-- Left box (x ≈ 210 to 270, y ≈ 520 to 555)
b_block:load(p_earth_body, 0.95)
b_block:stroke({{200, 520}, {235, 532}, {275, 540}, {300, 545}}, {pressure=0.95})
b_block:stroke({{205, 535}, {240, 545}, {280, 552}, {310, 555}}, {pressure=0.95})
b_block:load(p_heath_russet, 0.9)
b_block:stroke({{210, 528}, {245, 538}, {280, 545}}, {pressure=0.9})

-- Right box (x ≈ 650 to 720, y ≈ 530 to 565)
b_block:load(p_earth_body, 0.95)
b_block:stroke({{640, 530}, {680, 542}, {720, 552}, {745, 558}}, {pressure=0.95})
b_block:stroke({{645, 545}, {685, 555}, {725, 562}, {750, 565}}, {pressure=0.95})
b_block:load(p_heath_russet, 0.9)
b_block:stroke({{655, 538}, {690, 548}, {725, 555}}, {pressure=0.9})

-- 4. Soften and blend the dark trench under the dolmen base
-- Sweep rich peaty heather upwards into the base of the stones to bed them naturally into the tumulus
b_block:load(p_heath_russet, 0.95)
b_block:stroke({{280, 432}, {320, 434}, {360, 435}, {400, 436}, {440, 435}, {480, 434}, {520, 432}}, {pressure=0.9})
b_block:load(p_earth_body, 0.9)
b_block:stroke({{290, 438}, {340, 440}, {390, 442}, {440, 442}, {490, 440}, {530, 438}}, {pressure=0.9})

-- Blend the barrow smoothly wet-into-wet across the whole lower slope
blend(rect(180, 510, 580, 60), {angle=0.2})
blend(rect(180, 510, 580, 60), {angle=-0.15})
blend(rect(260, 428, 280, 24), {angle=0.15})

-- 5. Delicate, Authentic Bent-Grass Tufts along the Ridge Crest
local function draw_delicate_grass(bx, by, len, ang, bend, p)
  local mx = bx + len * 0.55 * math.cos(ang + bend * 0.5)
  local my = by - len * 0.55 * math.sin(ang + bend * 0.5)
  local tx = bx + len * math.cos(ang + bend)
  local ty = by - len * math.sin(ang + bend)
  b_rigger:load(p, 0.65)
  b_rigger:stroke({{bx, by}, {mx, my}, {tx, ty}}, {pressure={0.5, 0.04}, ramps={0.05, 0.3}})
end

local function draw_fine_grass_cluster(cx, cy, n, h, lean)
  for i = 1, n do
    local bx = cx + rand(-4, 4)
    local by = cy + rand(-1.5, 1.5)
    local len = h * rand(0.75, 1.25)
    local ang = 1.57 + rand(-0.25, 0.25) + lean * 0.3
    local bnd = lean * rand(0.2, 0.5)
    local p = (rand(0, 1) < 0.65) and p_straw_sharp or p_straw_shade
    draw_delicate_grass(bx, by, len, ang, bnd, p)
  end
end

-- Fine grasses along the crest
local crest_grasses = {
  {115, 454, 6, 13, -0.4}, {175, 446, 6, 12, -0.3}, {235, 432, 5, 11, -0.2},
  {305, 422, 5, 11, -0.1}, {475, 422, 5, 11, 0.2}, {535, 434, 6, 13, 0.3},
  {615, 446, 6, 13, 0.3}, {695, 448, 5, 12, 0.4}, {775, 438, 6, 13, 0.4}
}
for _, g in ipairs(crest_grasses) do draw_fine_grass_cluster(g[1], g[2], g[3], g[4], g[5]) end

-- 6. Authentic Hairline Carline Thistle Skeletons on the Ridge (drawn with fine rigger hairline!)
local thistles = {{160, 448, 16}, {570, 438, 15}, {835, 438, 14}}
for _, th in ipairs(thistles) do
  local x, y, h = th[1], th[2], th[3]
  -- Slender dry stem
  b_rigger:load(p_thistle_hair, 0.75)
  b_rigger:stroke({{x, y}, {x, y - h*0.6}, {x + rand(-1, 1), y - h}}, {pressure={0.4, 0.1}, ramps={0.05, 0.2}})
  -- Dried flower head
  b_rigger:load(p_straw_sharp, 0.8)
  b_rigger:stroke({{x - 2, y - h}, {x + 2, y - h}}, {pressure=0.35})
  b_rigger:stroke({{x, y - h - 1.5}, {x, y - h + 1.5}}, {pressure=0.35})
end

-- 7. Ensure Waxing Crescent Moon and Evening Star are pure and crystalline in the twilight
local p_moon_pure = pile{{"lead white", 34}, {"yellow ochre", 1.0}, {"chrome yellow", 0.4}, medium=0.12}
b_rigger:load(p_moon_pure, 0.95)
b_rigger:stroke({
  {645.2, 105.5}, {641.5, 108.0}, {639.0, 112.5}, {640.2, 117.0}, {644.0, 119.5}
}, {pressure={0.08, 0.85, 1.0, 0.8, 0.08}, ramps={0.05, 0.05}})

b_rigger:load(p_moon_pure, 1.0)
b_rigger:touch(688, 148, {pressure=0.6})

print("Chunk 101 complete: Fire hydrants, shoe boxes, and trench eliminated; pure Friedrich landscape perfected.")

--@ chunk 101
-- Chunk 102: Fusing the Entire Ground, Covering Hydrants with Opaque Body Paint, and Perfecting the Landscape

-- 1. Piles
local p_mist_opaque  = pile{{"lead white", 28}, {"yellow ochre", 4.0}, {"chrome yellow", 1.2}, {"smalt", 1.0}, medium=0.15}
local p_peat_dark    = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}
local p_earth_body   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4.0}, {"yellow ochre", 3.0}, medium=0.18}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5.0}, {"lead white", 2.0}, medium=0.2}
local p_straw_sharp  = pile{{"lead white", 16}, {"yellow ochre", 9}, {"chrome yellow", 1.2}, {"raw umber", 1.0}, medium=0.22}
local p_straw_shade  = pile{{"yellow ochre", 7}, {"raw umber", 6}, {"bone black", 2.5}, {"lead white", 2.0}, medium=0.2}
local p_twilight_rim = pile{{"lead white", 26}, {"yellow ochre", 3.5}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.2}

local b_touch  = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_sweep  = brush{kind="filbert", width=12, stiffness=0.85}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}

-- 2. Cover the three black hydrants on the ridge with opaque body paint
b_touch:load(p_mist_opaque, 0.95)
b_touch:touch(145, 446, {pressure=0.9, drag={3.0, 0}})
b_touch:touch(565, 439, {pressure=0.9, drag={3.0, 0}})
b_touch:touch(845, 437, {pressure=0.9, drag={3.0, 0}})

-- 3. Cover the rectangular box cut in the middle of the ground with broad peaty heather sweeps
b_sweep:load(p_earth_body, 0.95)
b_sweep:stroke({{160, 525}, {360, 535}, {560, 545}, {760, 540}, {820, 530}}, {pressure=0.95})
b_sweep:stroke({{170, 545}, {380, 555}, {580, 560}, {770, 555}, {830, 545}}, {pressure=0.95})
b_sweep:load(p_heath_russet, 0.9)
b_sweep:stroke({{200, 535}, {400, 545}, {600, 550}, {780, 545}}, {pressure=0.9})

-- Cover the rectangular step under the dolmen
b_sweep:load(p_peat_dark, 0.95)
b_sweep:stroke({{250, 440}, {320, 444}, {390, 446}, {460, 445}, {530, 442}}, {pressure=0.95})

-- 4. FUSE THE ENTIRE GROUND SEAMLESSLY (from crest all the way down to y = 704 across full canvas width)
-- Using below(barrow_crest) ensures ZERO rectangular cut edges!
local barrow_nodes = {
  {x=-20, y=465}, {x=80, y=458}, {x=180, y=448}, {x=260, y=430},
  {x=330, y=415}, {x=385, y=408}, {x=445, y=414}, {x=520, y=430},
  {x=600, y=448}, {x=680, y=450}, {x=760, y=436}, {x=830, y=436},
  {x=910, y=448}, {x=1020, y=465}
}

local function barrow_crest(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 440
end

local m_barrow = below(barrow_crest)

blend(m_barrow, {angle=0.2})
blend(m_barrow, {angle=-0.15})
blend(m_barrow, {angle=1.57})
blend(m_barrow, {angle=0.1})

-- 5. Re-stroke the natural barrow crest contour line against the glowing mist
local b_crest = brush{kind="round", width=2.8, point=1, stiffness=0.9}
b_crest:load(p_earth_body, 0.95)
local crest_line = {}
for x = -10, 1010, 10 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest:stroke(crest_line, {pressure=0.9, ramps={0.01, 0.01}})

-- 6. Bed the Dolmen firmly into the rising summit turf
local b_turf = brush{kind="filbert", width=5.5, stiffness=0.85}
b_turf:load(p_peat_dark, 0.95)
b_turf:stroke({{290, 434}, {330, 436}, {375, 438}, {420, 438}, {465, 436}, {510, 434}}, {pressure=0.95})

-- 7. Subtle Twilight Rim-Light on the top edge of the Capstone catching the evening sky
b_rigger:load(p_twilight_rim, 0.75)
b_rigger:stroke({{318, 400}, {326, 390}, {344, 380}, {374, 375}, {408, 373}, {442, 376}, {468, 384}}, {pressure={0.35, 0.75, 0.7, 0.8, 0.7, 0.3}})

-- 8. Delicate Windswept Bent-Grass Tufts along the Barrow Crest
local function draw_delicate_grass(bx, by, len, ang, bend, p)
  local mx = bx + len * 0.55 * math.cos(ang + bend * 0.5)
  local my = by - len * 0.55 * math.sin(ang + bend * 0.5)
  local tx = bx + len * math.cos(ang + bend)
  local ty = by - len * math.sin(ang + bend)
  b_rigger:load(p, 0.7)
  b_rigger:stroke({{bx, by}, {mx, my}, {tx, ty}}, {pressure={0.55, 0.04}, ramps={0.05, 0.3}})
end

local function draw_fine_grass_cluster(cx, cy, n, h, lean)
  for i = 1, n do
    local bx = cx + rand(-4, 4)
    local by = cy + rand(-1.5, 1.5)
    local len = h * rand(0.75, 1.25)
    local ang = 1.57 + rand(-0.25, 0.25) + lean * 0.3
    local bnd = lean * rand(0.2, 0.5)
    local p = (rand(0, 1) < 0.65) and p_straw_sharp or p_straw_shade
    draw_delicate_grass(bx, by, len, ang, bnd, p)
  end
end

local crest_grasses = {
  {115, 454, 6, 13, -0.4}, {175, 446, 6, 12, -0.3}, {235, 432, 5, 11, -0.2},
  {305, 422, 5, 11, -0.1}, {475, 422, 5, 11, 0.2}, {535, 434, 6, 13, 0.3},
  {615, 446, 6, 13, 0.3}, {695, 448, 5, 12, 0.4}, {775, 438, 6, 13, 0.4}
}
for _, g in ipairs(crest_grasses) do draw_fine_grass_cluster(g[1], g[2], g[3], g[4], g[5]) end

print("Chunk 102 complete: Entire ground seamlessly fused, hydrants covered, landscape perfected.")

--@ chunk 102
-- Chunk 103: Eliminating the 3 Ridge Heads, Softening Dolmen Shadow, and Sculpting Asymmetrical Granite Capstone

-- 1. Piles
local p_sky_glow     = pile{{"lead white", 26}, {"yellow ochre", 5.2}, {"chrome yellow", 2.2}, {"vermilion", 0.5}, {"raw umber", 0.25}, medium=0.18}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5.0}, {"lead white", 2.0}, medium=0.2}
local p_peat_dark    = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}

local p_granite_deep = pile{{"bone black", 10}, {"raw umber", 8}, {"smalt", 2.5}, medium=0.15}
local p_granite_body = pile{{"lead white", 10}, {"raw umber", 7}, {"bone black", 4}, {"yellow ochre", 3.5}, {"smalt", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_granite_facet= pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage    = pile{{"lead white", 14}, {"green earth", 7.0}, {"yellow ochre", 3.5}, {"raw umber", 1.2}, medium=0.25}
local p_lich_gold    = pile{{"lead white", 10}, {"yellow ochre", 9.5}, {"chrome yellow", 1.8}, {"raw umber", 1.0}, medium=0.25}

local b_touch = brush{kind="round", width=2.4, point=1, stiffness=0.9}
local b_block = brush{kind="filbert", width=6.5, stiffness=0.85}
local b_facet = brush{kind="round", width=2.0, point=1, stiffness=0.85}
local b_crack = brush{kind="round", width=0.9, point=1, stiffness=0.9}

-- 2. Cover the three black heads sticking into the sky at the exact coordinates
b_touch:load(p_sky_glow, 0.95)
b_touch:stroke({{140, 424}, {145, 434}}, {pressure=0.95})
b_touch:stroke({{560, 422}, {565, 434}}, {pressure=0.95})
b_touch:stroke({{840, 420}, {845, 430}}, {pressure=0.95})

-- 3. Soften the dark crescent shadow under the dolmen
b_block:load(p_heath_russet, 0.95)
b_block:stroke({{280, 428}, {330, 432}, {385, 434}, {440, 434}, {490, 430}}, {pressure=0.9})
b_block:stroke({{290, 435}, {340, 438}, {390, 440}, {440, 439}, {485, 436}}, {pressure=0.9})

-- 4. Re-sculpt the Capstone: Break symmetry into an authentic, rugged, asymmetrical Scandinavian granite block
-- Left shoulder is heavy, blocky, and angular (x = 312 to 395, y = 376 to 404)
-- Right shoulder slopes down in rough crystalline steps (x = 395 to 478, y = 382 to 402)
-- Resurface body to eliminate the straight vertical center crack
local pts_cap_asym = {
  {310, 403}, {318, 390}, {336, 380}, {365, 374}, {405, 373},
  {445, 378}, {476, 386}, {482, 396}, {466, 403}, {425, 404},
  {385, 405}, {340, 404}, {310, 403}
}
local m_cap_asym = poly(pts_cap_asym, false)
work(m_cap_asym, {hand="body", pile=p_granite_body, angle=0.06, coverage=2.0, fill=true, clip=true})

-- Deep cast shadow directly under the overhang
local pts_undercut = {
  {310, 403}, {340, 404}, {385, 405}, {425, 404}, {466, 403}, {482, 396},
  {478, 407}, {435, 408}, {385, 409}, {335, 408}, {310, 403}
}
local m_undercut = poly(pts_undercut, false)
work(m_undercut, {hand="detail", pile=p_granite_deep, angle=0, coverage=1.8, fill=true, clip=true})

-- Top sky-facing facets: distinct angular planes (no single vertical crack!)
-- Western elevated block plane (x = 310 to 385)
local pts_planeA = {
  {310, 403}, {318, 390}, {336, 380}, {365, 374}, {390, 373},
  {388, 388}, {355, 390}, {330, 394}, {310, 403}
}
local m_planeA = poly(pts_planeA, false)
work(m_planeA, {hand="detail", pile=p_granite_lit, angle=0.04, coverage=1.6, fill=true, clip=true})

-- Eastern sloping plane (x = 390 to 482)
local pts_planeB = {
  {390, 373}, {405, 373}, {445, 378}, {476, 386}, {482, 396}, {466, 403},
  {435, 398}, {408, 394}, {388, 388}
}
local m_planeB = poly(pts_planeB, false)
work(m_planeB, {hand="detail", pile=p_granite_facet, angle=-0.04, coverage=1.6, fill=true, clip=true})

-- Natural geological joint fractures (diagonal/angular, NOT straight vertical!)
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{365, 374}, {368, 386}, {364, 404}}, {pressure={0.6, 0.7, 0.3}})
b_crack:stroke({{430, 376}, {434, 388}, {438, 404}}, {pressure={0.6, 0.7, 0.3}})
b_crack:stroke({{330, 382}, {334, 394}}, {pressure={0.4, 0.6, 0.2}})

-- Top ridge highlights catching twilight
b_crack:load(p_granite_lit, 0.95)
b_crack:stroke({{310, 403}, {318, 390}, {336, 380}, {365, 374}, {405, 373}}, {pressure={0.3, 0.8, 0.7, 0.85, 0.4}})
b_crack:stroke({{406, 373}, {445, 378}, {476, 386}}, {pressure={0.4, 0.8, 0.75, 0.3}})

-- Lichen patches encrusting the granite
local lichens = {
  {330, 384, p_lich_sage, 2.2}, {355, 378, p_lich_gold, 2.5},
  {380, 382, p_lich_sage, 1.8}, {418, 378, p_lich_gold, 2.4},
  {445, 382, p_lich_sage, 2.0}, {468, 390, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichens) do
  b_facet:load(lp[3], 0.8)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

print("Chunk 103 complete: Ridge heads covered, dolmen shadow softened, capstone sculpted as authentic granite.")

--@ chunk 103
-- Chunk 104: Elevating the Dolmen Chamber, Eliminating Ridge Stumps, and Sculpting Authentic Granite Orthostats

-- 1. Piles for Ancient Swedish Granite, Chamber Void, and Heath
local p_cavern_void   = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}
local p_granite_deep  = pile{{"bone black", 9}, {"raw umber", 8}, {"smalt", 2.5}, {"red earth", 1.5}, medium=0.15}
local p_granite_shade = pile{{"raw umber", 9}, {"bone black", 6}, {"smalt", 2.5}, {"lead white", 3.0}, {"yellow ochre", 2.5}, {"red earth", 1.2}, medium=0.18}
local p_granite_body  = pile{{"lead white", 10}, {"raw umber", 7}, {"bone black", 4}, {"yellow ochre", 3.5}, {"smalt", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_granite_facet = pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.2}
local p_granite_lit   = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage     = pile{{"lead white", 14}, {"green earth", 7.0}, {"yellow ochre", 3.5}, {"raw umber", 1.2}, medium=0.25}
local p_lich_gold     = pile{{"lead white", 10}, {"yellow ochre", 9.5}, {"chrome yellow", 1.8}, {"raw umber", 1.0}, medium=0.25}

local p_mist_opaque   = pile{{"lead white", 28}, {"yellow ochre", 4.2}, {"chrome yellow", 1.2}, {"smalt", 1.0}, medium=0.15}
local p_peat_dark     = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}
local p_earth_body    = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4.0}, {"yellow ochre", 3.0}, medium=0.18}

local b_block  = brush{kind="filbert", width=6.5, stiffness=0.85}
local b_facet  = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_crack  = brush{kind="round", width=1.0, point=1, stiffness=0.9}

-- 2. Permanently Cover the Three Black Stumps on the Ridge
-- Opaque mist paint from above and peaty earth from below
b_block:load(p_mist_opaque, 0.95)
b_block:stroke({{136, 436}, {144, 442}, {152, 436}}, {pressure=0.95})
b_block:stroke({{558, 434}, {566, 440}, {574, 434}}, {pressure=0.95})
b_block:stroke({{838, 430}, {846, 436}, {854, 430}}, {pressure=0.95})

b_facet:load(p_earth_body, 0.95)
b_facet:stroke({{130, 448}, {144, 444}, {158, 442}}, {pressure=0.9})
b_facet:stroke({{552, 440}, {566, 438}, {580, 440}}, {pressure=0.9})
b_facet:stroke({{832, 438}, {846, 436}, {860, 438}}, {pressure=0.9})

-- 3. Elevate the Dolmen: Open Chamber Void & Heavy Granite Orthostats
-- A. Deep, solemn burial chamber void (from under the capstone at y=400 down to turf at y=430)
b_block:load(p_cavern_void, 0.95)
b_block:stroke({{348, 404}, {395, 404}, {442, 404}}, {pressure=0.95})
b_block:stroke({{350, 414}, {395, 415}, {440, 414}}, {pressure=0.95})
b_block:stroke({{354, 424}, {395, 425}, {438, 424}}, {pressure=0.95})

-- B. Left Orthostat: Massive, rugged upright stone block (width ~24 units, height ~32 units)
b_block:load(p_granite_shade, 0.95)
b_block:stroke({{336, 400}, {338, 416}, {342, 432}}, {pressure=0.95})
b_block:stroke({{346, 400}, {348, 416}, {352, 432}}, {pressure=0.95})
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{335, 400}, {337, 414}, {340, 428}}, {pressure={0.8, 0.7, 0.3}})

-- C. Right Orthostat: Massive, rugged upright stone block (width ~26 units, height ~32 units)
b_block:load(p_granite_shade, 0.95)
b_block:stroke({{436, 400}, {438, 416}, {440, 432}}, {pressure=0.95})
b_block:stroke({{446, 400}, {448, 416}, {452, 432}}, {pressure=0.95})
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{448, 400}, {450, 414}, {454, 428}}, {pressure={0.8, 0.75, 0.3}})
b_facet:load(p_granite_lit, 0.85)
b_facet:stroke({{450, 402}, {452, 410}, {451, 420}}, {pressure={0.65, 0.5, 0.2}})

-- D. Rear Orthostat inside the chamber (dimly visible in deep shadow)
b_block:load(p_granite_deep, 0.9)
b_block:stroke({{390, 404}, {392, 416}, {394, 428}}, {pressure=0.85})
b_block:stroke({{402, 404}, {404, 416}, {406, 428}}, {pressure=0.85})

-- 4. The Monumental Granite Capstone: Heavy, rough, angular erratic slab resting on the orthostats
-- Underside heavy cast shadow projecting over the chamber entrance
b_crack:load(p_cavern_void, 0.95)
b_crack:stroke({{312, 402}, {355, 403}, {400, 404}, {445, 403}, {478, 401}}, {pressure=0.95})

-- Capstone front face (solid weathered granite)
b_block:load(p_granite_body, 0.95)
b_block:stroke({{316, 398}, {350, 399}, {395, 400}, {440, 399}, {474, 397}}, {pressure=0.95})
b_block:stroke({{322, 392}, {360, 393}, {405, 394}, {445, 393}, {470, 391}}, {pressure=0.95})

-- Top sky-facing facets (sharp, angular crystalline cleavage planes)
local pts_capA = {{312, 396}, {320, 386}, {338, 378}, {368, 373}, {398, 372}, {394, 388}, {365, 390}, {338, 393}, {312, 396}}
local m_capA = poly(pts_capA, false)
work(m_capA, {hand="detail", pile=p_granite_lit, angle=0.04, coverage=1.6, fill=true, clip=true})

local pts_capB = {{398, 372}, {442, 375}, {470, 383}, {478, 393}, {464, 398}, {435, 395}, {412, 391}, {394, 388}}
local m_capB = poly(pts_capB, false)
work(m_capB, {hand="detail", pile=p_granite_facet, angle=-0.04, coverage=1.6, fill=true, clip=true})

-- Fracture cleavage cracks
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{398, 372}, {394, 388}, {390, 402}}, {pressure={0.75, 0.85, 0.4}})
b_crack:stroke({{344, 379}, {348, 390}, {346, 400}}, {pressure={0.5, 0.65, 0.2}})
b_crack:stroke({{440, 376}, {442, 387}, {444, 398}}, {pressure={0.5, 0.65, 0.2}})

-- Top ridge highlights catching the silvery sky
b_crack:load(p_granite_lit, 0.95)
b_crack:stroke({{312, 396}, {320, 386}, {338, 378}, {368, 373}, {398, 372}}, {pressure={0.3, 0.8, 0.7, 0.85, 0.4}})
b_crack:stroke({{400, 373}, {442, 375}, {470, 383}}, {pressure={0.4, 0.8, 0.75, 0.3}})

-- Lichen touches
local lichens = {
  {334, 384, p_lich_sage, 2.2}, {362, 378, p_lich_gold, 2.5},
  {380, 384, p_lich_sage, 1.8}, {418, 378, p_lich_gold, 2.4},
  {444, 381, p_lich_sage, 2.0}, {465, 388, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichens) do
  b_facet:load(lp[3], 0.8)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- 5. Flanking Erratic Stones
local m_sl = poly({{298, 428}, {306, 410}, {322, 406}, {330, 414}, {326, 430}, {302, 430}}, false)
work(m_sl, {hand="detail", pile=p_granite_body, angle=0.3, coverage=1.8, fill=true, clip=true})
local m_sl_lit = poly({{306, 410}, {322, 406}, {330, 414}, {320, 417}, {310, 416}}, false)
work(m_sl_lit, {hand="detail", pile=p_granite_facet, angle=0.15, coverage=1.5, fill=true, clip=true})

local m_sr = poly({{464, 428}, {474, 410}, {498, 406}, {510, 416}, {504, 430}, {468, 430}}, false)
work(m_sr, {hand="detail", pile=p_granite_body, angle=-0.2, coverage=1.8, fill=true, clip=true})
local m_sr_lit = poly({{474, 410}, {498, 406}, {510, 416}, {498, 416}, {482, 414}}, false)
work(m_sr_lit, {hand="detail", pile=p_granite_facet, angle=-0.1, coverage=1.5, fill=true, clip=true})

-- 6. Bed the Base of the Stones into the Tumulus Turf at y ≈ 430 (NOT choking the capstone!)
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{285, 430}, {330, 432}, {375, 434}, {420, 434}, {465, 432}, {515, 430}}, {pressure=0.95})

print("Chunk 104 complete: Dolmen elevated with open chamber void, ridge stumps eliminated, capstone sculpted.")

--@ chunk 104
local p_sky_glow = pile{{"lead white", 26}, {"yellow ochre", 5.2}, {"chrome yellow", 2.2}, {"vermilion", 0.5}, {"raw umber", 0.25}, medium=0.15}
local p_earth_body = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4.0}, {"yellow ochre", 3.0}, medium=0.18}
local p_peat_dark = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}
local p_cavern_void = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}
local p_granite_body = pile{{"lead white", 10}, {"raw umber", 7.5}, {"bone black", 4.0}, {"yellow ochre", 3.5}, {"smalt", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_granite_facet= pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.2}

print("Piles:", p_sky_glow, p_earth_body, p_peat_dark, p_cavern_void, p_granite_body, p_granite_facet)

--@ chunk 105
-- Chunk 105 (fixed): Permanent Elimination of Ducks, Udders, and Trench — Solidifying the Megalithic Tomb

-- 1. Piles
local p_sky_glow     = pile{{"lead white", 26}, {"yellow ochre", 5.2}, {"chrome yellow", 2.2}, {"vermilion", 0.5}, {"raw umber", 0.25}, medium=0.15}
local p_earth_body   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4.0}, {"yellow ochre", 3.0}, medium=0.18}
local p_peat_dark    = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}
local p_cavern_void  = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}

local p_granite_body = pile{{"lead white", 10}, {"raw umber", 7.5}, {"bone black", 4.0}, {"yellow ochre", 3.5}, {"smalt", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_granite_facet= pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}

-- 2. Permanent Elimination of the Three Black Ducks on the Ridge
local duck_spots = {
  {144, 436, 446},
  {566, 434, 444},
  {846, 432, 442}
}
for _, d in ipairs(duck_spots) do
  -- Sky half of duck
  local m_sky_duck = ellipse(d[1], d[2], 12, 10)
  work(m_sky_duck, {hand="body", pile=p_sky_glow, angle=0, coverage=2.5, fill=true, clip=true})
  -- Ground half of duck
  local m_gnd_duck = ellipse(d[1], d[3], 12, 10)
  work(m_gnd_duck, {hand="body", pile=p_earth_body, angle=0.2, coverage=2.5, fill=true, clip=true})
end

-- Re-stroke the natural crest contour across the duck spots
local b_crest = brush{kind="round", width=2.8, point=1, stiffness=0.9}
b_crest:load(p_earth_body, 0.95)
b_crest:stroke({{125, 452}, {144, 446}, {165, 442}}, {pressure=0.9})
b_crest:stroke({{545, 438}, {566, 436}, {585, 440}}, {pressure=0.9})
b_crest:stroke({{825, 438}, {846, 436}, {865, 440}}, {pressure=0.9})

-- 3. Permanent Elimination of the Udders & Solidifying the Megalithic Tomb
-- A. Solid Chamber Void (100% opaque cavern black portal between the uprights, width 60 units)
local m_chamber = poly({{365, 396}, {425, 396}, {425, 436}, {365, 436}})
work(m_chamber, {hand="body", pile=p_cavern_void, angle=0, coverage=2.5, fill=true, clip=true})

-- B. Left Orthostat: One single, solid, massive stone pillar (width ~35 units)
local m_lo = poly({{330, 395}, {365, 395}, {365, 436}, {330, 436}})
work(m_lo, {hand="body", pile=p_granite_body, angle=1.45, coverage=2.5, fill=true, clip=true})
-- Light outer face catching twilight
local m_lo_lit = poly({{330, 395}, {345, 395}, {345, 436}, {330, 436}})
work(m_lo_lit, {hand="detail", pile=p_granite_facet, angle=1.45, coverage=1.6, fill=true, clip=true})
-- Deep shadow on inner chamber face
local b_crack = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_crack:load(p_cavern_void, 0.95)
b_crack:stroke({{365, 396}, {365, 436}}, {pressure=0.9})

-- C. Right Orthostat: One single, solid, massive stone pillar (width ~35 units)
local m_ro = poly({{425, 395}, {460, 395}, {460, 436}, {425, 436}})
work(m_ro, {hand="body", pile=p_granite_body, angle=1.4, coverage=2.5, fill=true, clip=true})
-- Light outer face catching twilight
local m_ro_lit = poly({{445, 395}, {460, 395}, {460, 436}, {445, 436}})
work(m_ro_lit, {hand="detail", pile=p_granite_facet, angle=1.4, coverage=1.6, fill=true, clip=true})
b_crack:load(p_granite_lit, 0.9)
b_crack:stroke({{450, 396}, {452, 415}, {450, 432}}, {pressure=0.6})
-- Deep shadow on inner chamber face
b_crack:load(p_cavern_void, 0.95)
b_crack:stroke({{425, 396}, {425, 436}}, {pressure=0.9})

-- D. Heavy cast shadow directly under capstone overhang resting on the two pillars
local b_block = brush{kind="filbert", width=6.5, stiffness=0.85}
b_block:load(p_cavern_void, 0.95)
b_block:stroke({{315, 401}, {360, 402}, {410, 403}, {455, 402}, {475, 400}}, {pressure=0.95})

-- 4. Bed the Pillars Deeply into the Tumulus Turf (Eliminate trench and white gaps)
local m_base = rect(280, 432, 230, 24)
work(m_base, {hand="body", pile=p_peat_dark, angle=0.15, coverage=2.2, fill=true})
blend(rect(275, 430, 240, 28), {angle=0.15})

print("Chunk 105 complete: Ducks obliterated, udders replaced with solid granite pillars, chamber void deepened.")

--@ chunk 106
-- Chunk 106: Eliminating Mines & Bunker — Sculpting True Weathered Glacial Granite Megaliths

-- 1. Piles
local p_mist_glow    = pile{{"lead white", 26}, {"yellow ochre", 4.5}, {"chrome yellow", 1.5}, {"smalt", 1.2}, {"raw umber", 0.3}, medium=0.25}
local p_earth_body   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4.0}, {"yellow ochre", 3.0}, medium=0.18}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5.0}, {"lead white", 2.0}, medium=0.2}
local p_peat_dark    = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}
local p_cavern_void  = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}

local p_granite_deep = pile{{"bone black", 10}, {"raw umber", 8}, {"smalt", 2.5}, medium=0.15}
local p_granite_body = pile{{"lead white", 10}, {"raw umber", 7.5}, {"bone black", 4.0}, {"yellow ochre", 3.5}, {"smalt", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_granite_facet= pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage    = pile{{"lead white", 14}, {"green earth", 7.0}, {"yellow ochre", 3.5}, {"raw umber", 1.2}, medium=0.25}
local p_lich_gold    = pile{{"lead white", 10}, {"yellow ochre", 9.5}, {"chrome yellow", 1.8}, {"raw umber", 1.0}, medium=0.25}

local b_block = brush{kind="filbert", width=8.0, stiffness=0.85}
local b_limb  = brush{kind="filbert", width=5.0, stiffness=0.85}
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_crack = brush{kind="round", width=0.9, point=1, stiffness=0.9}

-- 2. Permanently Eliminate the Three Mine Circles on the Ridge
-- Glaze horizon mist across the ridge line in those areas
local m_mist_patch = (ellipse(144, 440, 25, 14) + ellipse(566, 436, 25, 14) + ellipse(846, 434, 25, 14)):soften(6)
work(m_mist_patch, {hand="glaze", pile=p_mist_glow, angle=0, coverage=1.4, fill=true})
blend(m_mist_patch, {angle=0})

-- Re-stroke the natural barrow crest contour firmly
local barrow_nodes = {
  {x=-20, y=465}, {x=80, y=458}, {x=180, y=448}, {x=260, y=430},
  {x=330, y=415}, {x=385, y=408}, {x=445, y=414}, {x=520, y=430},
  {x=600, y=448}, {x=680, y=450}, {x=760, y=436}, {x=830, y=436},
  {x=910, y=448}, {x=1020, y=465}
}
local function barrow_crest(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 440
end

local b_crest = brush{kind="round", width=2.8, point=1, stiffness=0.9}
b_crest:load(p_earth_body, 0.95)
local crest_line = {}
for x = -10, 1010, 10 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest:stroke(crest_line, {pressure=0.9, ramps={0.01, 0.01}})

-- 3. Transform the Bunker into an Authentic Prehistoric Megalithic Tomb
-- A. Break the square garage door: Paint an irregular, natural cavern void (shadow within)
b_block:load(p_cavern_void, 0.95)
b_block:stroke({{355, 403}, {395, 404}, {435, 403}}, {pressure=0.95})
b_block:stroke({{358, 414}, {395, 415}, {432, 414}}, {pressure=0.95})
b_block:stroke({{362, 424}, {395, 425}, {428, 424}}, {pressure=0.95})

-- B. Left Orthostat: Natural, chunky, irregular leaning glacial boulder (NOT a rectangle!)
-- Round the square corners and model natural rock bulk
b_limb:load(p_granite_body, 0.95)
b_limb:stroke({{334, 402}, {338, 418}, {344, 434}}, {pressure=0.95})
b_limb:stroke({{344, 402}, {348, 418}, {354, 434}}, {pressure=0.95})
b_limb:stroke({{354, 404}, {358, 418}, {362, 432}}, {pressure=0.9})

-- Rounded outer facet catching twilight
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{332, 404}, {335, 416}, {340, 426}, {344, 434}}, {pressure={0.8, 0.75, 0.6, 0.2}})
-- Inner shadowed flank curving into the chamber
b_facet:load(p_cavern_void, 0.9)
b_facet:stroke({{360, 405}, {364, 418}, {366, 432}}, {pressure=0.8})

-- C. Right Orthostat: Massive, rounded, crystalline glacial boulder (NOT a rectangle!)
b_limb:load(p_granite_body, 0.95)
b_limb:stroke({{426, 404}, {430, 418}, {434, 434}}, {pressure=0.9})
b_limb:stroke({{436, 402}, {440, 418}, {444, 434}}, {pressure=0.95})
b_limb:stroke({{446, 402}, {450, 418}, {454, 434}}, {pressure=0.95})

-- Rounded outer facet catching twilight
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{448, 402}, {452, 414}, {456, 426}, {458, 434}}, {pressure={0.8, 0.75, 0.6, 0.2}})
b_facet:load(p_granite_lit, 0.85)
b_facet:stroke({{450, 404}, {453, 410}, {452, 420}}, {pressure={0.65, 0.5, 0.2}})
-- Inner shadowed flank
b_facet:load(p_cavern_void, 0.9)
b_facet:stroke({{425, 405}, {428, 418}, {430, 432}}, {pressure=0.8})

-- D. Capstone: Break the straight horizontal underside and soften the ellipse
-- Jagged, rough, natural rock undercut shadow
b_crack:load(p_cavern_void, 0.95)
b_crack:stroke({{312, 402}, {330, 404}, {360, 402}, {395, 403}, {430, 402}, {460, 404}, {478, 401}}, {pressure={0.7, 0.95, 0.85, 0.95, 0.85, 0.95, 0.6}})

-- Natural rock texture and fractures across the capstone face
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{395, 374}, {392, 388}, {388, 402}}, {pressure={0.7, 0.8, 0.3}})
b_crack:stroke({{344, 380}, {348, 392}, {346, 402}}, {pressure={0.5, 0.65, 0.2}})
b_crack:stroke({{440, 378}, {442, 388}, {444, 400}}, {pressure={0.5, 0.65, 0.2}})

-- Lichens on the Capstone
local lichens = {
  {334, 386, p_lich_sage, 2.2}, {362, 380, p_lich_gold, 2.5},
  {380, 386, p_lich_sage, 1.8}, {418, 380, p_lich_gold, 2.4},
  {444, 383, p_lich_sage, 2.0}, {465, 390, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichens) do
  b_facet:load(lp[3], 0.8)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- 4. Dissolve the Brown Foundation Box: Bed the Stones Naturally into Swelling Barrow Turf
-- Sweeping strokes of heather and peaty loam wrapping around and between the stones
b_block:load(p_heath_russet, 0.95)
b_block:stroke({{260, 436}, {310, 440}, {365, 442}, {425, 442}, {480, 440}, {535, 436}}, {pressure=0.95})
b_block:load(p_earth_body, 0.95)
b_block:stroke({{270, 444}, {330, 448}, {395, 450}, {460, 449}, {520, 446}}, {pressure=0.95})

-- Dark peat contact turf around the base of the uprights (natural irregular mounds)
b_facet:load(p_peat_dark, 0.95)
b_facet:stroke({{325, 432}, {345, 434}, {370, 433}}, {pressure=0.9})
b_facet:stroke({{420, 433}, {445, 434}, {468, 432}}, {pressure=0.9})

-- Blend the turf around the stone base softly into the barrow slope
blend(rect(250, 430, 300, 26), {angle=0.2})
blend(rect(250, 430, 300, 26), {angle=-0.15})

print("Chunk 106 complete: Mines eliminated, bunker transformed into authentic weathered glacial granite.")

--@ chunk 107
-- Chunk 107: Full-Width Horizon Mist, Burying the Board via below(barrow_crest), and Sculpting Rugged Granite Megaliths

-- 1. Piles
local p_mist_glow    = pile{{"lead white", 26}, {"yellow ochre", 4.2}, {"chrome yellow", 1.2}, {"smalt", 1.5}, {"raw umber", 0.3}, medium=0.25}
local p_earth_body   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4.0}, {"yellow ochre", 3.0}, medium=0.18}
local p_peat_dark    = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5.0}, {"lead white", 2.0}, medium=0.2}
local p_cavern_void  = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}

local p_granite_deep = pile{{"bone black", 10}, {"raw umber", 8}, {"smalt", 2.5}, medium=0.15}
local p_granite_body = pile{{"lead white", 10}, {"raw umber", 7.5}, {"bone black", 4.0}, {"yellow ochre", 3.5}, {"smalt", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_granite_facet= pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage    = pile{{"lead white", 14}, {"green earth", 7.0}, {"yellow ochre", 3.5}, {"raw umber", 1.2}, medium=0.25}
local p_lich_gold    = pile{{"lead white", 10}, {"yellow ochre", 9.5}, {"chrome yellow", 1.8}, {"raw umber", 1.0}, medium=0.25}

-- 2. Full-Width Horizon Mist (x = 0 to 1000): Dissolves the three smoke bombs into a seamless glowing mist band
local m_mist_full = rect(0, 420, 1000, 32):soften(4)
work(m_mist_full, {hand="glaze", pile=p_mist_glow, angle=0, coverage=1.4, fill=true})
blend(rect(0, 415, 1000, 40), {angle=0})

-- 3. Prehistoric Barrow Mound: Continuous from crest all the way down to canvas bottom (Buries the 2x4 board!)
local barrow_nodes = {
  {x=-20, y=465}, {x=80, y=458}, {x=180, y=448}, {x=260, y=430},
  {x=330, y=415}, {x=385, y=408}, {x=445, y=414}, {x=520, y=430},
  {x=600, y=448}, {x=680, y=450}, {x=760, y=436}, {x=830, y=436},
  {x=910, y=448}, {x=1020, y=465}
}

local function barrow_crest(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 440
end

local m_barrow = below(barrow_crest)

-- Solid body fill across the entire barrow from crest to bottom
work(m_barrow, {
  hand = "body",
  pile = p_earth_body,
  angle = 0.22,
  length = {50, 110},
  coverage = 2.4,
  fill = true,
  clip = true
})

-- Summit heather bank
local m_summit = (ellipse(385, 435, 170, 32) + ellipse(250, 448, 140, 35)):soften(15) * m_barrow
work(m_summit, {hand="body", pile=p_heath_russet, angle=0.18, coverage=1.4, fill=true})

-- Blend barrow smoothly (no horizontal cutoffs!)
blend(m_barrow, {angle=0.2})
blend(m_barrow, {angle=-0.15})

-- Firm, organic stroke along the natural barrow crest against the glowing mist
local b_crest = brush{kind="round", width=2.8, point=1, stiffness=0.9}
b_crest:load(p_peat_dark, 0.95)
local crest_line = {}
for x = -10, 1010, 10 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest:stroke(crest_line, {pressure=0.9, ramps={0.01, 0.01}})

-- 4. Sculpt the Megalithic Tomb as Authentic Rugged Scandinavian Glacial Granite
-- A. Sacred Chamber Void (Deep velvety black cavern portal between orthostats)
local m_chamber = poly({{355, 400}, {435, 400}, {435, 432}, {355, 432}})
work(m_chamber, {hand="body", pile=p_cavern_void, angle=0, coverage=2.5, fill=true, clip=true})

-- B. Left Orthostat: Massive, rugged, chunky leaning stone pillar (width ~30 units)
local m_lo = poly({{332, 396}, {362, 396}, {365, 432}, {335, 432}}, false)
work(m_lo, {hand="body", pile=p_granite_body, angle=1.45, coverage=2.2, fill=true, clip=true})
local m_lo_lit = poly({{332, 396}, {344, 396}, {346, 432}, {335, 432}}, false)
work(m_lo_lit, {hand="detail", pile=p_granite_facet, angle=1.45, coverage=1.6, fill=true, clip=true})

-- C. Right Orthostat: Massive, rugged, chunky stone pillar (width ~30 units)
local m_ro = poly({{428, 396}, {458, 396}, {456, 432}, {426, 432}}, false)
work(m_ro, {hand="body", pile=p_granite_body, angle=1.4, coverage=2.2, fill=true, clip=true})
local m_ro_lit = poly({{444, 396}, {458, 396}, {456, 432}, {446, 432}}, false)
work(m_ro_lit, {hand="detail", pile=p_granite_facet, angle=1.4, coverage=1.6, fill=true, clip=true})

-- D. The Monumental Granite Capstone: Heavy, angular, rugged erratic slab
local pts_cap = {
  {318, 400}, {326, 390}, {344, 380}, {374, 375}, {408, 373}, {442, 376},
  {468, 384}, {475, 394}, {460, 400}, {425, 401}, {385, 401}, {345, 400}, {318, 400}
}
local m_cap = poly(pts_cap, false)
work(m_cap, {hand="body", pile=p_granite_body, angle=0.06, coverage=2.2, fill=true, clip=true})

-- Deep cast shadow directly under capstone overhang
local pts_undercut = {
  {318, 400}, {345, 400}, {385, 401}, {425, 401}, {460, 400}, {475, 394},
  {470, 405}, {430, 406}, {385, 406}, {340, 405}, {318, 400}
}
local m_undercut = poly(pts_undercut, false)
work(m_undercut, {hand="detail", pile=p_cavern_void, angle=0, coverage=2.0, fill=true, clip=true})

-- Top sky-facing facets catching twilight
local pts_topA = {
  {318, 400}, {326, 390}, {344, 380}, {374, 375}, {400, 373},
  {396, 388}, {365, 390}, {338, 394}, {318, 400}
}
local m_topA = poly(pts_topA, false)
work(m_topA, {hand="detail", pile=p_granite_lit, angle=0.04, coverage=1.6, fill=true, clip=true})

local pts_topB = {
  {400, 373}, {442, 376}, {468, 384}, {475, 394}, {460, 400},
  {435, 396}, {412, 392}, {396, 388}
}
local m_topB = poly(pts_topB, false)
work(m_topB, {hand="detail", pile=p_granite_facet, angle=-0.04, coverage=1.6, fill=true, clip=true})

-- Fracture cleavage cracks cleaving the capstone
local b_crack = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{400, 373}, {396, 388}, {392, 402}}, {pressure={0.75, 0.85, 0.4}})
b_crack:stroke({{348, 380}, {352, 390}, {350, 400}}, {pressure={0.5, 0.65, 0.2}})
b_crack:stroke({{440, 378}, {442, 388}, {444, 400}}, {pressure={0.5, 0.65, 0.2}})

-- Top ridge highlights catching the silvery sky
b_crack:load(p_granite_lit, 0.95)
b_crack:stroke({{318, 400}, {326, 390}, {344, 380}, {374, 375}, {400, 373}}, {pressure={0.3, 0.8, 0.7, 0.85, 0.4}})
b_crack:stroke({{402, 374}, {442, 376}, {468, 384}}, {pressure={0.4, 0.8, 0.75, 0.3}})

-- Lichen touches on Capstone
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local lichens = {
  {338, 385, p_lich_sage, 2.2}, {365, 380, p_lich_gold, 2.5},
  {384, 385, p_lich_sage, 1.8}, {418, 379, p_lich_gold, 2.4},
  {444, 382, p_lich_sage, 2.0}, {462, 388, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichens) do
  b_facet:load(lp[3], 0.8)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- E. Flanking Glacial Boulders
local m_sl = poly({{300, 430}, {308, 412}, {324, 408}, {332, 416}, {326, 432}, {302, 432}}, false)
work(m_sl, {hand="detail", pile=p_granite_body, angle=0.3, coverage=1.8, fill=true, clip=true})
local m_sl_lit = poly({{308, 412}, {324, 408}, {332, 416}, {322, 419}, {312, 418}}, false)
work(m_sl_lit, {hand="detail", pile=p_granite_facet, angle=0.15, coverage=1.5, fill=true, clip=true})

local m_sr = poly({{464, 430}, {474, 412}, {498, 408}, {510, 418}, {504, 432}, {468, 432}}, false)
work(m_sr, {hand="detail", pile=p_granite_body, angle=-0.2, coverage=1.8, fill=true, clip=true})
local m_sr_lit = poly({{474, 412}, {498, 408}, {510, 418}, {498, 418}, {482, 416}}, false)
work(m_sr_lit, {hand="detail", pile=p_granite_facet, angle=-0.1, coverage=1.5, fill=true, clip=true})

-- Bed the base of the stones deeply into dark peat and late-autumn heather
local b_block = brush{kind="filbert", width=6.5, stiffness=0.85}
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{285, 428}, {330, 430}, {375, 432}, {420, 432}, {465, 430}, {515, 428}}, {pressure=0.95})
b_block:load(p_heath_russet, 0.9)
b_block:stroke({{295, 434}, {340, 436}, {390, 438}, {440, 437}, {490, 435}}, {pressure=0.9})

print("Chunk 107 complete: Smoke bombs dissolved, board buried, Megalithic Tomb sculpted.")

--@ chunk 108
-- Chunk 108: Bedding the Dolmen, Painting the Ancient Veteran Oak & Windswept Companion Tree

-- 1. Piles
local p_peat_earth   = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5.0}, {"lead white", 2.0}, medium=0.2}
local p_moss_green   = pile{{"green earth", 7}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}

local p_bark_solid   = pile{{"bone black", 14}, {"raw umber", 8}, {"red earth", 2.0}, medium=0.15}
local p_bark_mid     = pile{{"raw umber", 8}, {"bone black", 5}, {"yellow ochre", 4.0}, {"lead white", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_bark_ridge   = pile{{"yellow ochre", 7}, {"lead white", 5.0}, {"raw umber", 4.0}, {"bone black", 1.2}, medium=0.2}
local p_hollow_pitch = pile{{"bone black", 14}, {"raw umber", 6}, medium=0.1}
local p_woundwood    = pile{{"raw umber", 6}, {"yellow ochre", 5.5}, {"lead white", 3.5}, {"red earth", 2.0}, medium=0.2}
local p_stag_shade   = pile{{"raw umber", 8}, {"bone black", 5}, {"lead white", 5}, {"yellow ochre", 2.5}, medium=0.2}
local p_stag_bleach  = pile{{"lead white", 18}, {"raw umber", 3.5}, {"yellow ochre", 2.5}, {"smalt", 1.0}, medium=0.22}

local p_leaf_russet  = pile{{"yellow ochre", 7}, {"red earth", 6.5}, {"raw umber", 4.0}, {"lead white", 1.5}, medium=0.22}
local p_leaf_gold    = pile{{"yellow ochre", 9}, {"chrome yellow", 2.5}, {"lead white", 4.0}, {"vermilion", 0.4}, medium=0.25}

local p_granite_facet= pile{{"lead white", 16}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 24}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage    = pile{{"lead white", 14}, {"green earth", 7.0}, {"yellow ochre", 3.5}, {"raw umber", 1.2}, medium=0.25}

local b_bole   = brush{kind="filbert", width=7.5, stiffness=0.9}
local b_limb   = brush{kind="filbert", width=4.5, stiffness=0.85}
local b_branch = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_twig   = brush{kind="round", width=1.4, point=1, stiffness=0.85}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}
local b_leaf   = brush{kind="round", width=1.6, point=1, stiffness=0.8}

-- 2. Bed the Dolmen into the Barrow Turf (Eliminate the dark puddle under the stones)
-- Multi-directional strokes of warm heather and peaty loam sweeping over the puddle
b_bole:load(p_heath_russet, 0.95)
b_bole:stroke({{270, 436}, {320, 439}, {375, 440}, {430, 440}, {485, 438}, {530, 435}}, {pressure=0.95})
b_bole:load(p_peat_earth, 0.95)
b_bole:stroke({{280, 442}, {330, 445}, {385, 446}, {440, 445}, {495, 443}}, {pressure=0.95})
b_bole:load(p_moss_green, 0.9)
b_bole:stroke({{310, 440}, {360, 442}, {410, 442}, {460, 440}}, {pressure=0.85})

-- Refine capstone surface with crystalline facets and lichens
b_branch:load(p_granite_facet, 0.85)
b_branch:stroke({{325, 394}, {355, 386}, {390, 382}}, {pressure={0.6, 0.8, 0.5}})
b_branch:stroke({{400, 380}, {435, 382}, {465, 390}}, {pressure={0.5, 0.75, 0.5}})
b_twig:load(p_granite_lit, 0.9)
b_twig:stroke({{320, 398}, {340, 386}, {370, 380}, {405, 378}}, {pressure={0.3, 0.75, 0.8, 0.4}})
b_twig:load(p_lich_sage, 0.8)
b_twig:touch(345, 388, {pressure=0.6, drag={2.0, 0.1}})
b_twig:touch(420, 384, {pressure=0.6, drag={2.0, 0.1}})

-- 3. The Ancient Veteran Oak (Rooted on left barrow slope, x ≈ 230 to 290, y ≈ 432 to 452)
-- Solid, muscular trunk with flaring root buttresses gripping the barrow soil
b_bole:load(p_bark_solid, 0.95)
-- Western flaring root claw
b_bole:stroke({{225, 452}, {240, 442}, {255, 428}, {264, 405}, {266, 375}}, {pressure={0.95, 0.9, 0.85, 0.8, 0.75}})
-- Central anchoring root
b_bole:stroke({{258, 454}, {264, 438}, {268, 415}, {270, 385}, {270, 360}}, {pressure={0.95, 0.95, 0.9, 0.85, 0.8}})
-- Eastern root buttress gripping into the barrow slope near the dolmen
b_bole:stroke({{310, 446}, {296, 438}, {284, 422}, {278, 395}, {274, 365}}, {pressure={0.95, 0.9, 0.85, 0.8, 0.75}})

-- Solid body fill of the trunk core (width ~35 units)
b_bole:stroke({{248, 442}, {254, 418}, {258, 385}, {262, 355}}, {pressure=0.95})
b_bole:stroke({{274, 442}, {275, 418}, {274, 385}, {272, 355}}, {pressure=0.95})
b_bole:stroke({{290, 442}, {288, 418}, {282, 385}, {276, 355}}, {pressure=0.95})

-- Bark furrows and raised ridges
b_twig:load(p_hollow_pitch, 0.95)
b_twig:stroke({{246, 444}, {252, 426}, {256, 402}, {260, 370}}, {pressure={0.85, 0.65, 0.3}})
b_twig:stroke({{266, 446}, {268, 428}, {270, 402}, {268, 366}}, {pressure={0.85, 0.7, 0.35}})
b_twig:stroke({{286, 442}, {284, 424}, {280, 398}, {275, 368}}, {pressure={0.85, 0.65, 0.35}})

b_twig:load(p_bark_ridge, 0.85)
b_twig:stroke({{238, 448}, {246, 435}, {252, 412}, {256, 380}}, {pressure={0.3, 0.75, 0.6, 0.2}})
b_twig:stroke({{258, 448}, {262, 430}, {265, 405}, {265, 372}}, {pressure={0.3, 0.7, 0.6, 0.2}})
b_twig:stroke({{274, 448}, {274, 430}, {274, 402}, {271, 368}}, {pressure={0.3, 0.65, 0.6, 0.2}})
b_twig:stroke({{294, 444}, {290, 428}, {284, 404}, {278, 372}}, {pressure={0.3, 0.7, 0.6, 0.2}})

-- Hollow cavity
b_twig:load(p_hollow_pitch, 0.95)
b_twig:stroke({{268, 424}, {270, 406}, {268, 392}}, {pressure=0.85})
b_twig:load(p_woundwood, 0.85)
b_twig:stroke({{265, 426}, {266, 408}, {272, 392}}, {pressure={0.5, 0.85, 0.4}})
b_twig:stroke({{274, 422}, {275, 402}, {271, 392}}, {pressure={0.5, 0.85, 0.4}})

-- 4. The Great Living Eastern Bough: Arching over the left side of the Dolmen to shelter it
b_limb:load(p_bark_solid, 0.95)
b_limb:stroke({
  {274, 365}, {305, 355}, {340, 344}, {375, 335}
}, {pressure={0.95, 0.85, 0.7, 0.45}, ramps={0.02, 0.05}})

b_branch:load(p_bark_mid, 0.85)
b_branch:stroke({{276, 360}, {306, 350}, {340, 339}, {372, 330}}, {pressure={0.4, 0.7, 0.6, 0.25}})

-- Secondary boughs off Eastern limb:
-- E1: Upward arching bough into twilight
b_branch:load(p_bark_solid, 0.95)
b_branch:stroke({
  {340, 344}, {358, 320}, {376, 292}, {392, 264}
}, {pressure={0.85, 0.7, 0.55, 0.25}, ramps={0.02, 0.1}})
b_twig:load(p_bark_solid, 0.85)
b_twig:stroke({{358, 320}, {350, 298}, {344, 276}}, {pressure={0.7, 0.2}})
b_twig:stroke({{376, 292}, {388, 272}, {396, 252}}, {pressure={0.65, 0.2}})

-- E2: Sheltering branch drooping gently over the left of the capstone
b_branch:load(p_bark_solid, 0.9)
b_branch:stroke({
  {375, 335}, {395, 348}, {416, 358}, {435, 366}
}, {pressure={0.8, 0.65, 0.5, 0.2}, ramps={0.02, 0.1}})
b_twig:load(p_bark_solid, 0.85)
b_twig:stroke({{416, 358}, {425, 372}, {432, 384}}, {pressure={0.6, 0.2}})

-- 5. The Living Western Bough (Reaching west over the barrow slope)
b_limb:load(p_bark_solid, 0.95)
b_limb:stroke({
  {262, 365}, {230, 352}, {198, 338}, {165, 324}, {135, 310}
}, {pressure={0.9, 0.8, 0.65, 0.5, 0.25}, ramps={0.02, 0.05}})

-- W1: Upward western bough
b_branch:load(p_bark_solid, 0.9)
b_branch:stroke({
  {198, 338}, {185, 312}, {172, 282}, {158, 252}
}, {pressure={0.8, 0.65, 0.5, 0.25}, ramps={0.02, 0.1}})
b_twig:load(p_bark_solid, 0.85)
b_twig:stroke({{185, 312}, {194, 292}, {200, 272}}, {pressure={0.65, 0.2}})

-- W2: Drooping western bough
b_branch:load(p_bark_solid, 0.85)
b_branch:stroke({
  {165, 324}, {148, 345}, {130, 364}
}, {pressure={0.75, 0.55, 0.2}, ramps={0.02, 0.1}})

-- 6. The Bleached Stag-Head Crown aloft (Weathered dead boughs in twilight, height up to y=185)
b_branch:load(p_stag_shade, 0.95)
b_branch:stroke({{268, 345}, {266, 305}, {264, 265}, {262, 225}, {258, 185}}, {pressure={0.85, 0.75, 0.6, 0.4, 0.15}, ramps={0.02, 0.05}})
b_branch:stroke({{266, 295}, {280, 260}, {296, 222}, {308, 188}}, {pressure={0.8, 0.65, 0.5, 0.25}, ramps={0.02, 0.05}})
b_branch:stroke({{265, 300}, {250, 265}, {234, 228}, {220, 192}}, {pressure={0.8, 0.65, 0.5, 0.25}, ramps={0.02, 0.05}})

-- Bleached wood highlights & splintered tips
b_twig:load(p_stag_bleach, 0.95)
b_twig:stroke({{266, 335}, {264, 298}, {262, 260}, {260, 222}, {256, 185}}, {pressure={0.35, 0.7, 0.55, 0.35, 0.1}})
b_twig:stroke({{266, 290}, {281, 258}, {297, 220}, {309, 188}}, {pressure={0.3, 0.65, 0.5, 0.25}})
b_twig:stroke({{263, 295}, {248, 262}, {232, 225}, {219, 192}}, {pressure={0.3, 0.65, 0.5, 0.25}})

b_rigger:load(p_stag_bleach, 0.95)
b_rigger:stroke({{258, 188}, {256, 178}}, {pressure={0.45, 0.05}})
b_rigger:stroke({{258, 188}, {261, 180}}, {pressure={0.35, 0.05}})
b_rigger:stroke({{308, 190}, {312, 182}}, {pressure={0.4, 0.05}})
b_rigger:stroke({{220, 194}, {216, 185}}, {pressure={0.4, 0.05}})

-- 7. Crooked Sympodial Twigs
local oak_twigs = {
  -- East
  {{392, 264}, {404, 246}, {415, 230}},
  {{392, 264}, {384, 246}, {378, 230}},
  {{344, 276}, {338, 260}, {332, 245}},
  {{435, 366}, {446, 378}, {456, 390}},
  -- West
  {{158, 252}, {146, 234}, {134, 218}},
  {{158, 252}, {170, 235}, {180, 220}},
  {{135, 310}, {120, 295}, {106, 282}},
  {{135, 310}, {124, 325}, {112, 340}},
  {{130, 364}, {116, 382}, {102, 398}}
}
for _, tw in ipairs(oak_twigs) do
  b_rigger:load(p_bark_solid, 0.85)
  b_rigger:stroke(tw, {pressure={0.65, 0.15}, ramps={0.05, 0.2}})
end

-- 8. Clinging Marcescent Autumn Leaves (*Laub*) along the living twigs
local marcescent_sprays = {
  -- East
  {360, 320, 18}, {380, 295, 18}, {400, 265, 18}, {412, 240, 15},
  {390, 345, 18}, {420, 360, 16}, {440, 375, 14}, {340, 345, 16},
  -- West
  {180, 310, 18}, {165, 275, 18}, {145, 240, 16}, {125, 225, 14},
  {125, 305, 16}, {110, 290, 14}, {140, 340, 16}, {120, 365, 14}
}
for _, cl in ipairs(marcescent_sprays) do
  local cx, cy, n = cl[1], cl[2], cl[3]
  for i = 1, n do
    local ox = rand(-12, 12)
    local oy = rand(-9, 9)
    local p = (rand(0, 1) < 0.45) and p_leaf_russet or p_leaf_gold
    b_leaf:load(p, 0.75)
    b_leaf:touch(cx + ox, cy + oy, {pressure=rand(0.35, 0.65), drag={rand(1.2, 2.4), rand(1.1, 1.6)}})
  end
end

-- 9. The Windswept Companion Tree on the Right Knoll (x ≈ 755 to 800)
b_branch:load(p_bark_solid, 0.95)
b_branch:stroke({
  {766, 438}, {762, 405}, {756, 370}, {750, 335}, {746, 300}, {744, 270}
}, {pressure={0.85, 0.75, 0.6, 0.45, 0.3, 0.12}, ramps={0.02, 0.05}})

local comp_branches = {
  {{750, 335}, {772, 320}, {798, 305}, {824, 292}, {846, 280}},
  {{798, 305}, {814, 285}, {826, 265}, {836, 245}},
  {{746, 300}, {760, 278}, {772, 255}, {782, 232}},
  {{744, 270}, {752, 248}, {760, 225}}
}
for _, b in ipairs(comp_branches) do
  b_twig:load(p_bark_solid, 0.9)
  b_twig:stroke(b, {pressure={0.7, 0.2}, ramps={0.02, 0.1}})
end

local comp_twigs = {
  {{846, 280}, {860, 274}, {872, 268}},
  {{836, 245}, {846, 232}, {854, 220}},
  {{782, 232}, {792, 218}, {800, 205}}
}
for _, tw in ipairs(comp_twigs) do
  b_rigger:load(p_bark_solid, 0.85)
  b_rigger:stroke(tw, {pressure={0.6, 0.12}, ramps={0.02, 0.15}})
end

local comp_leaves = {
  {805, 310, 12}, {835, 292, 14}, {855, 280, 10}, {820, 265, 12},
  {770, 255, 12}, {775, 230, 10}
}
for _, cl in ipairs(comp_leaves) do
  local cx, cy, n = cl[1], cl[2], cl[3]
  for i = 1, n do
    local ox = rand(-10, 10)
    local oy = rand(-8, 8)
    local p = (rand(0, 1) < 0.5) and p_leaf_russet or p_leaf_gold
    b_leaf:load(p, 0.7)
    b_leaf:touch(cx + ox, cy + oy, {pressure=rand(0.3, 0.65), drag={rand(1.0, 2.0), 1.4}})
  end
end

print("Chunk 108 complete: Dolmen bedded into turf, Ancient Oak & Companion Tree painted.")

--@ chunk 109
-- Chunk 109: Drying the canvas, restoring the pristine Master Sky, Moon, Star, and Swelling Barrow Mound

-- 1. Dry the canvas completely
wait(2 * 24 * 60)

-- 2. Sky Piles
local p_sky_zenith = pile{{"lead white", 18}, {"cobalt blue", 6.2}, {"smalt", 4.5}, {"raw umber", 0.6}, {"bone black", 0.25}, medium=0.22}
local p_sky_upper  = pile{{"lead white", 26}, {"cobalt blue", 3.2}, {"smalt", 3.2}, {"yellow ochre", 0.8}, {"raw umber", 0.3}, medium=0.22}
local p_sky_mid    = pile{{"lead white", 28}, {"yellow ochre", 3.5}, {"chrome yellow", 0.9}, {"smalt", 1.0}, {"raw umber", 0.2}, {"vermilion", 0.1}, medium=0.22}
local p_sky_glow   = pile{{"lead white", 26}, {"yellow ochre", 5.2}, {"chrome yellow", 2.2}, {"vermilion", 0.5}, {"raw umber", 0.25}, medium=0.22}

-- 3. Four overlapping bands covering x = 0 to 1000 from zenith down behind the barrow (y = 425)
local m1 = rect(0, 0, 1000, 145):soften(30)
work(m1, {hand="body", pile=p_sky_zenith, angle=0, order="across", coverage=2.2, fill=true})

local m2 = rect(0, 105, 1000, 150):soften(30)
work(m2, {hand="body", pile=p_sky_upper, angle=0, order="across", coverage=2.2, fill=true})

local m3 = rect(0, 215, 1000, 140):soften(30)
work(m3, {hand="body", pile=p_sky_mid, angle=0, order="across", coverage=2.2, fill=true})

local m4 = rect(0, 315, 1000, 125):soften(25)
work(m4, {hand="body", pile=p_sky_glow, angle=0, order="across", coverage=2.2, fill=true})

-- 4. Seamless badger blending across the full canvas width
local m_sky_all = rect(0, 0, 1000, 425)
blend(m_sky_all, {angle=1.57})
blend(m_sky_all, {angle=0})

-- 5. Distant low Baltic hills silhouetted in soft atmospheric haze
local p_dist_coast = pile{{"lead white", 15}, {"smalt", 5.0}, {"raw umber", 3.0}, {"red earth", 0.9}, {"bone black", 0.4}, medium=0.25}
local pts_distant_land = {
  {0, 464}, {100, 460}, {220, 454}, {340, 452}, {460, 448},
  {580, 451}, {700, 447}, {820, 452}, {920, 458}, {1000, 464},
  {1000, 475}, {0, 475}
}
local m_coast = poly(pts_distant_land, true):soften(2.5)
work(m_coast, {hand="body", pile=p_dist_coast, angle=0, coverage=1.2, fill=true, clip=true})

-- 6. Calm water sheen and valley mist pooling over the water
local p_water_mist = pile{{"lead white", 26}, {"yellow ochre", 3.2}, {"smalt", 1.8}, {"raw umber", 0.3}, medium=0.3}
local m_water = rect(0, 435, 1000, 30):soften(4)
work(m_water, {hand="glaze", pile=p_water_mist, angle=0, coverage=1.1, fill=false})
blend(rect(0, 432, 1000, 34), {angle=0})

-- 7. Waxing crescent moon and solitary evening star (Venus)
local p_moon_pure = pile{{"lead white", 34}, {"yellow ochre", 1.0}, {"chrome yellow", 0.4}, medium=0.12}
local b_rigger = brush{kind="round", width=1.3, point=1, stiffness=0.85}
b_rigger:load(p_moon_pure, 0.95)
b_rigger:stroke({
  {645.2, 105.5}, {641.5, 108.0}, {639.0, 112.5}, {640.2, 117.0}, {644.0, 119.5}
}, {pressure={0.08, 0.85, 1.0, 0.8, 0.08}, ramps={0.05, 0.05}})

local b_star = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_star:load(p_moon_pure, 1.0)
b_star:touch(688, 148, {pressure=0.6})

-- 8. The Noble Prehistoric Barrow Mound across Full Lower Canvas
local barrow_nodes = {
  {x=-20, y=465}, {x=80, y=458}, {x=180, y=448}, {x=260, y=430},
  {x=330, y=415}, {x=385, y=408}, {x=445, y=414}, {x=520, y=430},
  {x=600, y=448}, {x=680, y=450}, {x=760, y=436}, {x=830, y=436},
  {x=910, y=448}, {x=1020, y=465}
}

local function barrow_crest(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 440
end

local m_barrow = below(barrow_crest)

local p_earth_body   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4}, {"yellow ochre", 3}, medium=0.18}
local p_peat_dark    = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5}, {"lead white", 2}, medium=0.2}

-- Solid body fill across the entire barrow from crest to bottom (covers the tree stump completely!)
work(m_barrow, {
  hand = "body",
  pile = p_earth_body,
  angle = 0.22,
  length = {50, 110},
  coverage = 2.4,
  fill = true,
  clip = true
})

-- Summit heather bank
local m_summit = (ellipse(385, 435, 170, 32) + ellipse(250, 448, 140, 35)):soften(15) * m_barrow
work(m_summit, {hand="body", pile=p_heath_russet, angle=0.18, coverage=1.4, fill=true})

-- Blend barrow smoothly (no horizontal cutoffs!)
blend(m_barrow, {angle=0.2})
blend(m_barrow, {angle=-0.15})

-- Firm, organic stroke along the natural barrow crest against the glowing mist
local b_crest = brush{kind="round", width=2.8, point=1, stiffness=0.9}
b_crest:load(p_peat_dark, 0.95)
local crest_line = {}
for x = -10, 1010, 10 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest:stroke(crest_line, {pressure=0.9, ramps={0.01, 0.01}})

print("Chunk 109 complete: Pristine sky restored, bicycle spokes obliterated, barrow mound sculpted.")

--@ chunk 110
-- Chunk 110: The Masterpiece — Sculpting the Megalithic Tomb, Glacial Erratics, Bent-Grass, and Twilight Rim-Lights

-- 1. Piles for Scandinavian Glacial Granite, Peat, Heather, and Twilight Light
local p_cavern_void   = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}
local p_granite_deep  = pile{{"bone black", 9}, {"raw umber", 8}, {"smalt", 2.5}, {"red earth", 1.5}, medium=0.15}
local p_granite_body  = pile{{"lead white", 9}, {"raw umber", 7.5}, {"bone black", 4.5}, {"yellow ochre", 3.5}, {"smalt", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_granite_facet = pile{{"lead white", 16}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.2}
local p_granite_lit   = pile{{"lead white", 24}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage     = pile{{"lead white", 14}, {"green earth", 7.0}, {"yellow ochre", 3.5}, {"raw umber", 1.2}, medium=0.25}
local p_lich_gold     = pile{{"lead white", 10}, {"yellow ochre", 9.5}, {"chrome yellow", 1.8}, {"raw umber", 1.0}, medium=0.25}

local p_peat_dark     = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}
local p_heather_bank  = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5.0}, {"lead white", 2.0}, medium=0.2}
local p_straw_sharp   = pile{{"lead white", 16}, {"yellow ochre", 9}, {"chrome yellow", 1.2}, {"raw umber", 1.0}, medium=0.22}
local p_straw_shade   = pile{{"yellow ochre", 7}, {"raw umber", 6}, {"bone black", 2.5}, {"lead white", 2.0}, medium=0.2}
local p_thistle_hair  = pile{{"raw umber", 8}, {"bone black", 6}, medium=0.2}
local p_twilight_rim  = pile{{"lead white", 26}, {"yellow ochre", 3.5}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.2}

-- 2. The Megalithic Tomb (Hünengrab) atop the Barrow Summit
-- A. Sacred Burial Chamber Void (Solid deep shadow between orthostats)
local m_chamber = poly({{365, 396}, {430, 396}, {430, 428}, {365, 428}})
work(m_chamber, {hand="body", pile=p_cavern_void, angle=0, coverage=2.5, fill=true, clip=true})

-- B. Left Orthostat (Massive, rugged upright stone block, width ~32 units)
local m_lo = poly({{332, 394}, {365, 394}, {368, 428}, {335, 428}}, false)
work(m_lo, {hand="body", pile=p_granite_body, angle=1.45, coverage=2.2, fill=true, clip=true})
local m_lo_lit = poly({{332, 394}, {345, 394}, {347, 428}, {335, 428}}, false)
work(m_lo_lit, {hand="detail", pile=p_granite_facet, angle=1.45, coverage=1.6, fill=true, clip=true})

-- C. Right Orthostat (Massive, rugged upright stone block, width ~32 units)
local m_ro = poly({{428, 394}, {460, 394}, {458, 428}, {426, 428}}, false)
work(m_ro, {hand="body", pile=p_granite_body, angle=1.4, coverage=2.2, fill=true, clip=true})
local m_ro_lit = poly({{446, 394}, {460, 394}, {458, 428}, {448, 428}}, false)
work(m_ro_lit, {hand="detail", pile=p_granite_facet, angle=1.4, coverage=1.6, fill=true, clip=true})

-- D. The Monumental Granite Capstone: Heavy, angular, rugged erratic slab
local pts_cap = {
  {316, 398}, {324, 388}, {344, 378}, {374, 373}, {408, 371}, {442, 374},
  {468, 382}, {475, 392}, {460, 398}, {425, 399}, {385, 399}, {345, 398}, {316, 398}
}
local m_cap = poly(pts_cap, false)
work(m_cap, {hand="body", pile=p_granite_body, angle=0.06, coverage=2.2, fill=true, clip=true})

-- Deep cast shadow directly under capstone overhang
local pts_undercut = {
  {316, 398}, {345, 398}, {385, 399}, {425, 399}, {460, 398}, {475, 392},
  {470, 403}, {430, 404}, {385, 404}, {340, 403}, {316, 398}
}
local m_undercut = poly(pts_undercut, false)
work(m_undercut, {hand="detail", pile=p_cavern_void, angle=0, coverage=2.0, fill=true, clip=true})

-- Top sky-facing facets catching twilight
local pts_topA = {
  {316, 398}, {324, 388}, {344, 378}, {374, 373}, {400, 371},
  {396, 386}, {365, 388}, {338, 392}, {316, 398}
}
local m_topA = poly(pts_topA, false)
work(m_topA, {hand="detail", pile=p_granite_lit, angle=0.04, coverage=1.6, fill=true, clip=true})

local pts_topB = {
  {400, 371}, {442, 374}, {468, 382}, {475, 392}, {460, 398},
  {435, 394}, {412, 390}, {396, 386}
}
local m_topB = poly(pts_topB, false)
work(m_topB, {hand="detail", pile=p_granite_facet, angle=-0.04, coverage=1.6, fill=true, clip=true})

-- Fracture cleavage cracks cleaving the capstone
local b_crack = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{400, 371}, {396, 386}, {392, 400}}, {pressure={0.75, 0.85, 0.4}})
b_crack:stroke({{348, 378}, {352, 388}, {350, 398}}, {pressure={0.5, 0.65, 0.2}})
b_crack:stroke({{440, 376}, {442, 386}, {444, 398}}, {pressure={0.5, 0.65, 0.2}})

-- Top ridge highlights catching the silvery sky
b_crack:load(p_granite_lit, 0.95)
b_crack:stroke({{316, 398}, {324, 388}, {344, 378}, {374, 373}, {400, 371}}, {pressure={0.3, 0.8, 0.7, 0.85, 0.4}})
b_crack:stroke({{402, 372}, {442, 374}, {468, 382}}, {pressure={0.4, 0.8, 0.75, 0.3}})

-- Lichen touches on Capstone
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local lichens = {
  {338, 383, p_lich_sage, 2.2}, {365, 378, p_lich_gold, 2.5},
  {384, 383, p_lich_sage, 1.8}, {418, 377, p_lich_gold, 2.4},
  {444, 380, p_lich_sage, 2.0}, {462, 386, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichens) do
  b_facet:load(lp[3], 0.8)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- E. Flanking Glacial Boulders
local m_sl = poly({{300, 428}, {308, 410}, {324, 406}, {332, 414}, {326, 430}, {302, 430}}, false)
work(m_sl, {hand="detail", pile=p_granite_body, angle=0.3, coverage=1.8, fill=true, clip=true})
local m_sl_lit = poly({{308, 410}, {324, 406}, {332, 414}, {322, 417}, {312, 416}}, false)
work(m_sl_lit, {hand="detail", pile=p_granite_facet, angle=0.15, coverage=1.5, fill=true, clip=true})

local m_sr = poly({{464, 428}, {474, 410}, {498, 406}, {510, 416}, {504, 430}, {468, 430}}, false)
work(m_sr, {hand="detail", pile=p_granite_body, angle=-0.2, coverage=1.8, fill=true, clip=true})
local m_sr_lit = poly({{474, 410}, {498, 406}, {510, 416}, {498, 416}, {482, 414}}, false)
work(m_sr_lit, {hand="detail", pile=p_granite_facet, angle=-0.1, coverage=1.5, fill=true, clip=true})

-- Bed the base of the stones into dark peat and heather naturally
local b_block = brush{kind="filbert", width=6.5, stiffness=0.85}
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{290, 428}, {330, 430}, {375, 432}, {420, 432}, {465, 430}, {510, 428}}, {pressure=0.95})
b_block:load(p_heather_bank, 0.9)
b_block:stroke({{300, 434}, {345, 436}, {395, 438}, {445, 437}, {495, 435}}, {pressure=0.9})

-- 3. Glacial Erratic Boulders (*Findlinge*) in the Mid-Ground
-- Left Boulder (x ≈ 215 to 265, y ≈ 522 to 548)
local pts_b1 = {{214, 540}, {226, 524}, {252, 520}, {266, 530}, {262, 546}, {238, 550}, {216, 546}}
local m_b1 = poly(pts_b1, false)
work(m_b1, {hand="body", pile=p_granite_body, angle=0.2, coverage=1.8, fill=true, clip=true})
local m_b1_sh = poly({{214, 540}, {234, 546}, {262, 546}, {238, 550}, {216, 546}}, false)
work(m_b1_sh, {hand="detail", pile=p_granite_deep, angle=0.2, coverage=1.6, fill=true, clip=true})
local m_b1_lit = poly({{226, 524}, {252, 520}, {266, 530}, {254, 534}, {230, 532}}, false)
work(m_b1_lit, {hand="detail", pile=p_granite_lit, angle=0.05, coverage=1.6, fill=true, clip=true})
b_crack:load(p_granite_deep, 0.85)
b_crack:stroke({{238, 522}, {242, 534}, {240, 548}}, {pressure={0.5, 0.7, 0.2}})
b_crack:load(p_lich_sage, 0.8)
b_crack:touch(234, 526, {pressure=0.6, drag={2.0, 0.1}})
b_crack:load(p_lich_gold, 0.75)
b_crack:touch(252, 526, {pressure=0.55, drag={1.8, 0.1}})

-- Right Boulder (x ≈ 660 to 715, y ≈ 532 to 558)
local pts_b2 = {{660, 550}, {672, 534}, {700, 530}, {714, 542}, {710, 556}, {684, 560}, {662, 556}}
local m_b2 = poly(pts_b2, false)
work(m_b2, {hand="body", pile=p_granite_body, angle=-0.2, coverage=1.8, fill=true, clip=true})
local m_b2_sh = poly({{660, 550}, {680, 556}, {710, 556}, {684, 560}, {662, 556}}, false)
work(m_b2_sh, {hand="detail", pile=p_granite_deep, angle=-0.2, coverage=1.6, fill=true, clip=true})
local m_b2_lit = poly({{672, 534}, {700, 530}, {714, 542}, {698, 544}, {678, 542}}, false)
work(m_b2_lit, {hand="detail", pile=p_granite_lit, angle=-0.05, coverage=1.6, fill=true, clip=true})
b_crack:load(p_granite_deep, 0.85)
b_crack:stroke({{686, 532}, {688, 544}, {686, 556}}, {pressure={0.5, 0.7, 0.2}})
b_crack:load(p_lich_sage, 0.8)
b_crack:touch(682, 536, {pressure=0.6, drag={2.0, 0.1}})
b_crack:load(p_lich_gold, 0.75)
b_crack:touch(696, 534, {pressure=0.55, drag={1.8, 0.1}})

-- Bed boulders naturally into the heather turf
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{202, 548}, {232, 554}, {266, 552}, {284, 545}}, {pressure=0.9})
b_block:stroke({{648, 556}, {678, 562}, {712, 560}, {730, 550}}, {pressure=0.9})

-- 4. Windswept Bent-Grass Tufts & Carline Thistles
local function draw_curved_grass(bx, by, len, ang, bend, p)
  local mx = bx + len * 0.55 * math.cos(ang + bend * 0.5)
  local my = by - len * 0.55 * math.sin(ang + bend * 0.5)
  local tx = bx + len * math.cos(ang + bend)
  local ty = by - len * math.sin(ang + bend)
  local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}
  b_rigger:load(p, 0.7)
  b_rigger:stroke({{bx, by}, {mx, my}, {tx, ty}}, {pressure={0.55, 0.04}, ramps={0.05, 0.3}})
end

local function draw_grass_cluster(cx, cy, n, h, lean)
  for i = 1, n do
    local bx = cx + rand(-4, 4)
    local by = cy + rand(-1.5, 1.5)
    local len = h * rand(0.75, 1.25)
    local ang = 1.57 + rand(-0.25, 0.25) + lean * 0.3
    local bnd = lean * rand(0.2, 0.5)
    local p = (rand(0, 1) < 0.65) and p_straw_sharp or p_straw_shade
    draw_curved_grass(bx, by, len, ang, bnd, p)
  end
end

-- Grasses around boulders and across the barrow crest
local grasses = {
  {204, 548, 6, 14, -0.4}, {228, 552, 6, 13, -0.2}, {264, 550, 6, 14, 0.3},
  {650, 556, 6, 14, -0.4}, {678, 562, 6, 13, -0.1}, {710, 560, 6, 13, 0.3},
  {120, 452, 6, 13, -0.4}, {180, 444, 6, 12, -0.3}, {240, 430, 5, 11, -0.2},
  {310, 420, 5, 11, -0.1}, {475, 420, 5, 11, 0.2}, {540, 434, 6, 13, 0.3},
  {620, 446, 6, 13, 0.3}, {700, 448, 5, 12, 0.4}, {780, 436, 6, 13, 0.4}
}
for _, g in ipairs(grasses) do draw_grass_cluster(g[1], g[2], g[3], g[4], g[5]) end

-- Hairline carline thistle skeletons along the barrow crest silhouetted against glowing mist
local thistles = {{160, 446, 16}, {570, 436, 15}, {835, 436, 14}}
for _, th in ipairs(thistles) do
  local x, y, h = th[1], th[2], th[3]
  local b_rigger = brush{kind="round", width=0.7, point=1, stiffness=0.9}
  b_rigger:load(p_thistle_hair, 0.75)
  b_rigger:stroke({{x, y}, {x, y - h*0.6}, {x + rand(-1, 1), y - h}}, {pressure={0.4, 0.1}, ramps={0.05, 0.2}})
  b_rigger:load(p_straw_sharp, 0.8)
  b_rigger:stroke({{x - 2, y - h}, {x + 2, y - h}}, {pressure=0.35})
  b_rigger:stroke({{x, y - h - 1.5}, {x, y - h + 1.5}}, {pressure=0.35})
end

-- 5. Subtle Twilight Rim-Lights on Stone Top Facets
local b_rigger = brush{kind="round", width=1.0, point=1, stiffness=0.85}
b_rigger:load(p_twilight_rim, 0.7)
b_rigger:stroke({{316, 398}, {324, 388}, {344, 378}, {374, 373}, {408, 371}, {442, 374}, {468, 382}}, {pressure={0.35, 0.75, 0.7, 0.8, 0.7, 0.3}})

print("Chunk 110 complete: Megalithic Tomb, erratic boulders, grasses, and thistles painted.")

--@ chunk 111
-- Chunk 111: Eliminating Sugar Cubes & Ditch — Weaving Rich Heathland & Subtle Pilgrim Track

-- 1. Piles
local p_earth_body   = pile{{"raw umber", 10}, {"bone black", 6}, {"red earth", 4.0}, {"yellow ochre", 3.0}, medium=0.18}
local p_peat_dark    = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5.0}, {"lead white", 2.0}, medium=0.2}
local p_peat_moss    = pile{{"green earth", 7}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}
local p_heath_gold   = pile{{"yellow ochre", 8}, {"raw umber", 4}, {"lead white", 3.5}, {"red earth", 2}, medium=0.2}

local p_sand_track   = pile{{"yellow ochre", 7}, {"raw umber", 5}, {"lead white", 3.5}, {"bone black", 1.5}, medium=0.22}
local p_straw_sharp  = pile{{"lead white", 16}, {"yellow ochre", 9}, {"chrome yellow", 1.2}, {"raw umber", 1.0}, medium=0.22}
local p_straw_shade  = pile{{"yellow ochre", 7}, {"raw umber", 6}, {"bone black", 2.5}, {"lead white", 2.0}, medium=0.2}

local b_sweep  = brush{kind="filbert", width=12, stiffness=0.85}
local b_detail = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}

-- 2. Cover the two sugar-cube boulders in the foreground permanently
-- Left sugar cube (x ≈ 210 to 270, y ≈ 520 to 555)
b_sweep:load(p_earth_body, 0.95)
b_sweep:stroke({{190, 525}, {230, 538}, {270, 545}, {300, 548}}, {pressure=0.95})
b_sweep:stroke({{200, 540}, {240, 548}, {280, 552}, {310, 555}}, {pressure=0.95})
b_sweep:load(p_heath_russet, 0.9)
b_sweep:stroke({{205, 532}, {245, 542}, {285, 548}}, {pressure=0.9})

-- Right sugar cube (x ≈ 650 to 720, y ≈ 530 to 565)
b_sweep:load(p_earth_body, 0.95)
b_sweep:stroke({{640, 535}, {680, 545}, {720, 554}, {745, 558}}, {pressure=0.95})
b_sweep:stroke({{645, 548}, {685, 556}, {725, 562}, {750, 565}}, {pressure=0.95})
b_sweep:load(p_heath_russet, 0.9)
b_sweep:stroke({{655, 542}, {690, 550}, {725, 556}}, {pressure=0.9})

-- 3. Smooth and dissolve the dark ditch in front of the dolmen base
-- Sweep warm heather turf up to the base of the upright stones to embed them naturally into the tumulus
b_sweep:load(p_heath_russet, 0.95)
b_sweep:stroke({{270, 432}, {320, 436}, {375, 438}, {430, 438}, {485, 436}, {530, 432}}, {pressure=0.9})
b_sweep:load(p_earth_body, 0.9)
b_sweep:stroke({{280, 438}, {330, 442}, {385, 444}, {440, 443}, {495, 440}}, {pressure=0.9})

-- 4. Weave rich undulating banks of heather, peat, and moss across the whole barrow
local barrow_nodes = {
  {x=-20, y=465}, {x=80, y=458}, {x=180, y=448}, {x=260, y=430},
  {x=330, y=415}, {x=385, y=408}, {x=445, y=414}, {x=520, y=430},
  {x=600, y=448}, {x=680, y=450}, {x=760, y=436}, {x=830, y=436},
  {x=910, y=448}, {x=1020, y=465}
}
local function barrow_crest(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 440
end

local m_barrow = below(barrow_crest)

-- Mid-slope mossy hollows
local m_h_mid = (ellipse(480, 535, 260, 45) + ellipse(220, 550, 180, 40)):soften(18) * m_barrow
work(m_h_mid, {hand="body", pile=p_peat_moss, angle=0.2, coverage=1.3, fill=true})

-- Lower warm heather banks
local m_h_warm = (ellipse(340, 610, 280, 50) + ellipse(760, 600, 240, 45)):soften(20) * m_barrow
work(m_h_warm, {hand="body", pile=p_heath_gold, angle=-0.15, coverage=1.3, fill=true})

-- Deep peaty loam at bottom margin
local m_h_base = rect(0, 645, 1000, 65):soften(15) * m_barrow
work(m_h_base, {hand="body", pile=p_earth_body, angle=0.1, coverage=1.4, fill=true})

-- 5. Subtle, Worn Pilgrim Track through the Heath
-- A faint, quiet track winding naturally through the dark heather from lower right toward the dolmen
local pts_track = {{700, 705}, {660, 645}, {615, 585}, {565, 530}, {515, 485}, {465, 450}, {425, 435}}
local b_track = brush{kind="round", width=3.0, point=1, stiffness=0.7}
b_track:load(p_sand_track, 0.45)
b_track:stroke(pts_track, {pressure={0.5, 0.15}, ramps={0.05, 0.2}})

-- 6. Blend the barrow smoothly wet-into-wet
-- Using m_barrow ensures ZERO horizontal cutoffs!
blend(m_barrow, {angle=0.2})
blend(m_barrow, {angle=-0.15})

-- 7. Re-stroke the natural barrow crest contour line against the glowing mist
local b_crest = brush{kind="round", width=2.8, point=1, stiffness=0.9}
b_crest:load(p_peat_dark, 0.95)
local crest_line = {}
for x = -10, 1010, 10 do
  table.insert(crest_line, {x, barrow_crest(x)})
end
b_crest:stroke(crest_line, {pressure=0.9, ramps={0.01, 0.01}})

-- 8. Delicate Windswept Bent-Grass Tufts along the Barrow Crest
local function draw_delicate_grass(bx, by, len, ang, bend, p)
  local mx = bx + len * 0.55 * math.cos(ang + bend * 0.5)
  local my = by - len * 0.55 * math.sin(ang + bend * 0.5)
  local tx = bx + len * math.cos(ang + bend)
  local ty = by - len * math.sin(ang + bend)
  b_rigger:load(p, 0.7)
  b_rigger:stroke({{bx, by}, {mx, my}, {tx, ty}}, {pressure={0.55, 0.04}, ramps={0.05, 0.3}})
end

local function draw_fine_grass_cluster(cx, cy, n, h, lean)
  for i = 1, n do
    local bx = cx + rand(-4, 4)
    local by = cy + rand(-1.5, 1.5)
    local len = h * rand(0.75, 1.25)
    local ang = 1.57 + rand(-0.25, 0.25) + lean * 0.3
    local bnd = lean * rand(0.2, 0.5)
    local p = (rand(0, 1) < 0.65) and p_straw_sharp or p_straw_shade
    draw_delicate_grass(bx, by, len, ang, bnd, p)
  end
end

local crest_grasses = {
  {115, 454, 6, 13, -0.4}, {175, 446, 6, 12, -0.3}, {235, 432, 5, 11, -0.2},
  {305, 422, 5, 11, -0.1}, {475, 422, 5, 11, 0.2}, {535, 434, 6, 13, 0.3},
  {615, 446, 6, 13, 0.3}, {695, 448, 5, 12, 0.4}, {775, 438, 6, 13, 0.4}
}
for _, g in ipairs(crest_grasses) do draw_fine_grass_cluster(g[1], g[2], g[3], g[4], g[5]) end

print("Chunk 111 complete: Sugar cubes and ditch eliminated; rich heathland woven with subtle track and fine grasses.")

--@ chunk 112
-- Chunk 112: Elevating the Megalithic Tomb — Tall Granite Orthostats, Deep Open Chamber Void, Rugged Erratic Capstone

-- 1. Piles for Ancient Scandinavian Glacial Granite, Chamber Void, Peat, and Twilight Light
local p_cavern_void   = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}
local p_granite_deep  = pile{{"bone black", 9}, {"raw umber", 8}, {"smalt", 2.5}, {"red earth", 1.5}, medium=0.15}
local p_granite_body  = pile{{"lead white", 10}, {"raw umber", 7.5}, {"bone black", 4.0}, {"yellow ochre", 3.5}, {"smalt", 2.0}, {"red earth", 1.0}, medium=0.18}
local p_granite_facet = pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 0.6}, medium=0.2}
local p_granite_lit   = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.22}
local p_lich_sage     = pile{{"lead white", 14}, {"green earth", 7.0}, {"yellow ochre", 3.5}, {"raw umber", 1.2}, medium=0.25}
local p_lich_gold     = pile{{"lead white", 10}, {"yellow ochre", 9.5}, {"chrome yellow", 1.8}, {"raw umber", 1.0}, medium=0.25}

local p_peat_dark     = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}
local p_heath_russet  = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5.0}, {"lead white", 2.0}, medium=0.2}
local p_straw_sharp   = pile{{"lead white", 16}, {"yellow ochre", 9}, {"chrome yellow", 1.2}, {"raw umber", 1.0}, medium=0.22}
local p_straw_shade   = pile{{"yellow ochre", 7}, {"raw umber", 6}, {"bone black", 2.5}, {"lead white", 2.0}, medium=0.2}
local p_twilight_rim  = pile{{"lead white", 26}, {"yellow ochre", 3.5}, {"raw umber", 0.8}, {"vermilion", 0.25}, medium=0.2}

-- 2. Open the Tall, Noble Burial Chamber Void (Solid pitch-black cavern portal, height 32 units)
local m_chamber = poly({{360, 395}, {430, 395}, {430, 427}, {360, 427}})
work(m_chamber, {hand="body", pile=p_cavern_void, angle=0, coverage=2.5, fill=true, clip=true})

-- 3. Left Orthostat: Massive, rugged upright granite boulder (width ~30 units, height ~32 units)
local m_lo = poly({{332, 394}, {364, 394}, {366, 427}, {334, 427}}, false)
work(m_lo, {hand="body", pile=p_granite_body, angle=1.45, coverage=2.2, fill=true, clip=true})
-- Light outer face catching twilight
local m_lo_lit = poly({{332, 394}, {344, 394}, {346, 427}, {334, 427}}, false)
work(m_lo_lit, {hand="detail", pile=p_granite_facet, angle=1.45, coverage=1.6, fill=true, clip=true})
-- Inner shadowed face framing the chamber
local b_crack = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_crack:load(p_cavern_void, 0.95)
b_crack:stroke({{364, 395}, {366, 427}}, {pressure=0.9})

-- 4. Right Orthostat: Massive, rugged upright granite boulder (width ~30 units, height ~32 units)
local m_ro = poly({{428, 394}, {458, 394}, {456, 427}, {426, 427}}, false)
work(m_ro, {hand="body", pile=p_granite_body, angle=1.4, coverage=2.2, fill=true, clip=true})
-- Light outer face catching twilight
local m_ro_lit = poly({{444, 394}, {458, 394}, {456, 427}, {446, 427}}, false)
work(m_ro_lit, {hand="detail", pile=p_granite_facet, angle=1.4, coverage=1.6, fill=true, clip=true})
b_crack:load(p_granite_lit, 0.9)
b_crack:stroke({{448, 395}, {450, 412}, {448, 426}}, {pressure=0.6})
-- Inner shadowed face framing the chamber
b_crack:load(p_cavern_void, 0.95)
b_crack:stroke({{428, 395}, {426, 427}}, {pressure=0.9})

-- 5. The Monumental Granite Capstone: Heavy, rough-hewn erratic slab resting on the orthostats
local pts_cap = {
  {316, 397}, {324, 387}, {344, 377}, {374, 372}, {408, 370}, {442, 373},
  {468, 381}, {475, 391}, {460, 397}, {425, 398}, {385, 398}, {345, 397}, {316, 397}
}
local m_cap = poly(pts_cap, false)
work(m_cap, {hand="body", pile=p_granite_body, angle=0.06, coverage=2.2, fill=true, clip=true})

-- Deep cast shadow directly under capstone overhang
local pts_undercut = {
  {316, 397}, {345, 397}, {385, 398}, {425, 398}, {460, 397}, {475, 391},
  {470, 402}, {430, 403}, {385, 403}, {340, 402}, {316, 397}
}
local m_undercut = poly(pts_undercut, false)
work(m_undercut, {hand="detail", pile=p_cavern_void, angle=0, coverage=2.0, fill=true, clip=true})

-- Top sky-facing facets catching twilight
local pts_topA = {
  {316, 397}, {324, 387}, {344, 377}, {374, 372}, {400, 370},
  {396, 385}, {365, 387}, {338, 391}, {316, 397}
}
local m_topA = poly(pts_topA, false)
work(m_topA, {hand="detail", pile=p_granite_lit, angle=0.04, coverage=1.6, fill=true, clip=true})

local pts_topB = {
  {400, 370}, {442, 373}, {468, 381}, {475, 391}, {460, 397},
  {435, 393}, {412, 389}, {396, 385}
}
local m_topB = poly(pts_topB, false)
work(m_topB, {hand="detail", pile=p_granite_facet, angle=-0.04, coverage=1.6, fill=true, clip=true})

-- Cleavage fracture cracks
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{400, 370}, {396, 385}, {392, 399}}, {pressure={0.75, 0.85, 0.4}})
b_crack:stroke({{348, 377}, {352, 387}, {350, 397}}, {pressure={0.5, 0.65, 0.2}})
b_crack:stroke({{440, 375}, {442, 385}, {444, 397}}, {pressure={0.5, 0.65, 0.2}})

-- Top ridge highlights catching the silvery sky
b_crack:load(p_granite_lit, 0.95)
b_crack:stroke({{316, 397}, {324, 387}, {344, 377}, {374, 372}, {400, 370}}, {pressure={0.3, 0.8, 0.7, 0.85, 0.4}})
b_crack:stroke({{402, 371}, {442, 373}, {468, 381}}, {pressure={0.4, 0.8, 0.75, 0.3}})

-- Lichen touches on Capstone
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local lichens = {
  {338, 382, p_lich_sage, 2.2}, {365, 377, p_lich_gold, 2.5},
  {384, 382, p_lich_sage, 1.8}, {418, 376, p_lich_gold, 2.4},
  {444, 379, p_lich_sage, 2.0}, {462, 385, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichens) do
  b_facet:load(lp[3], 0.8)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- 6. Flanking Glacial Boulders
local m_sl = poly({{300, 426}, {308, 408}, {324, 404}, {332, 412}, {326, 428}, {302, 428}}, false)
work(m_sl, {hand="detail", pile=p_granite_body, angle=0.3, coverage=1.8, fill=true, clip=true})
local m_sl_lit = poly({{308, 408}, {324, 404}, {332, 412}, {322, 415}, {312, 414}}, false)
work(m_sl_lit, {hand="detail", pile=p_granite_facet, angle=0.15, coverage=1.5, fill=true, clip=true})

local m_sr = poly({{462, 426}, {472, 408}, {496, 404}, {508, 414}, {502, 428}, {466, 428}}, false)
work(m_sr, {hand="detail", pile=p_granite_body, angle=-0.2, coverage=1.8, fill=true, clip=true})
local m_sr_lit = poly({{472, 408}, {496, 404}, {508, 414}, {496, 414}, {480, 412}}, false)
work(m_sr_lit, {hand="detail", pile=p_granite_facet, angle=-0.1, coverage=1.5, fill=true, clip=true})

-- 7. Bed the Base of the Stones into the Tumulus Turf at y ≈ 427 (Natural, soft, organic bedding!)
local b_block = brush{kind="filbert", width=6.5, stiffness=0.85}
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{285, 427}, {330, 429}, {375, 431}, {420, 431}, {465, 429}, {510, 427}}, {pressure=0.95})
b_block:load(p_heath_russet, 0.9)
b_block:stroke({{295, 433}, {340, 435}, {390, 437}, {440, 436}, {490, 434}}, {pressure=0.9})

-- 8. Subtle Twilight Rim-Light on the top edge of the Capstone
local b_rigger = brush{kind="round", width=1.0, point=1, stiffness=0.85}
b_rigger:load(p_twilight_rim, 0.75)
b_rigger:stroke({{316, 397}, {324, 387}, {344, 377}, {374, 372}, {408, 370}, {442, 373}, {468, 381}}, {pressure={0.35, 0.75, 0.7, 0.8, 0.7, 0.3}})

-- 9. Fine Windswept Bent-Grass Tufts around the Dolmen Base
local function draw_curved_grass(bx, by, len, ang, bend, p)
  local mx = bx + len * 0.55 * math.cos(ang + bend * 0.5)
  local my = by - len * 0.55 * math.sin(ang + bend * 0.5)
  local tx = bx + len * math.cos(ang + bend)
  local ty = by - len * math.sin(ang + bend)
  b_rigger:load(p, 0.7)
  b_rigger:stroke({{bx, by}, {mx, my}, {tx, ty}}, {pressure={0.55, 0.04}, ramps={0.05, 0.3}})
end

local function draw_grass_cluster(cx, cy, n, h, lean)
  for i = 1, n do
    local bx = cx + rand(-4, 4)
    local by = cy + rand(-1.5, 1.5)
    local len = h * rand(0.75, 1.25)
    local ang = 1.57 + rand(-0.25, 0.25) + lean * 0.3
    local bnd = lean * rand(0.2, 0.5)
    local p = (rand(0, 1) < 0.65) and p_straw_sharp or p_straw_shade
    draw_curved_grass(bx, by, len, ang, bnd, p)
  end
end

local dolmen_grasses = {
  {295, 429, 6, 14, -0.3}, {330, 429, 6, 13, -0.2}, {460, 429, 6, 13, 0.2}, {500, 428, 6, 14, 0.3}
}
for _, g in ipairs(dolmen_grasses) do draw_grass_cluster(g[1], g[2], g[3], g[4], g[5]) end

print("Chunk 112 complete: Dolmen elevated with open chamber void, tall granite orthostats, and rugged capstone.")

--@ chunk 113
-- Test where b:load failed
local p_cavern_pitch = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}
local p_granite_deep = pile{{"bone black", 9}, {"raw umber", 8}, {"smalt", 2.5}, {"red earth", 1.5}, medium=0.15}
local p_granite_dark = pile{{"raw umber", 8}, {"bone black", 6}, {"red earth", 2.5}, {"yellow ochre", 2.0}, {"smalt", 1.5}, {"lead white", 2.0}, medium=0.18}
local p_granite_body = pile{{"lead white", 10}, {"raw umber", 7.0}, {"yellow ochre", 3.5}, {"bone black", 3.5}, {"red earth", 2.0}, {"smalt", 1.5}, medium=0.2}
local p_granite_facet= pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.2}, {"raw umber", 2.0}, {"red earth", 1.2}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 2.0}, {"raw umber", 0.8}, {"vermilion", 0.3}, medium=0.22}
local p_granite_sheen= pile{{"lead white", 30}, {"yellow ochre", 2.0}, {"smalt", 1.5}, {"raw umber", 0.5}, medium=0.2}
local p_lich_sage    = pile{{"lead white", 14}, {"green earth", 7.0}, {"yellow ochre", 3.5}, {"raw umber", 1.2}, medium=0.25}
local p_lich_gold    = pile{{"lead white", 10}, {"yellow ochre", 9.5}, {"chrome yellow", 1.8}, {"raw umber", 1.0}, medium=0.25}
local p_peat_dark    = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5.0}, {"lead white", 2.0}, medium=0.2}
local p_moss_green   = pile{{"green earth", 7}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}
local p_straw_sharp  = pile{{"lead white", 16}, {"yellow ochre", 9}, {"chrome yellow", 1.2}, {"raw umber", 1.0}, medium=0.22}

print("All 13 piles ok")

--@ chunk 114
-- Chunk 113 (fixed): Master Resculpting of the Megalithic Tomb — Authentic Crystalline Scandinavian Granite

-- 1. Piles
local p_cavern_pitch = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}
local p_granite_deep = pile{{"bone black", 9}, {"raw umber", 8}, {"smalt", 2.5}, {"red earth", 1.5}, medium=0.15}
local p_granite_dark = pile{{"raw umber", 8}, {"bone black", 6}, {"red earth", 2.5}, {"yellow ochre", 2.0}, {"smalt", 1.5}, {"lead white", 2.0}, medium=0.18}
local p_granite_body = pile{{"lead white", 10}, {"raw umber", 7.0}, {"yellow ochre", 3.5}, {"bone black", 3.5}, {"red earth", 2.0}, {"smalt", 1.5}, medium=0.2}
local p_granite_facet= pile{{"lead white", 18}, {"yellow ochre", 3.5}, {"smalt", 2.0}, {"raw umber", 2.0}, {"red earth", 1.2}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"smalt", 1.8}, {"raw umber", 0.8}, {"vermilion", 0.3}, medium=0.22}
local p_granite_sheen= pile{{"lead white", 30}, {"yellow ochre", 2.0}, {"smalt", 1.5}, {"raw umber", 0.5}, medium=0.2}

local p_lich_sage    = pile{{"lead white", 14}, {"green earth", 7.0}, {"yellow ochre", 3.5}, {"raw umber", 1.2}, medium=0.25}
local p_lich_gold    = pile{{"lead white", 10}, {"yellow ochre", 9.5}, {"chrome yellow", 1.8}, {"raw umber", 1.0}, medium=0.25}

local p_peat_dark    = pile{{"raw umber", 10}, {"bone black", 7}, {"red earth", 3.5}, medium=0.15}
local p_heath_russet = pile{{"yellow ochre", 6}, {"red earth", 5.5}, {"raw umber", 5.0}, {"lead white", 2.0}, medium=0.2}
local p_moss_green   = pile{{"green earth", 7}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}
local p_straw_sharp  = pile{{"lead white", 16}, {"yellow ochre", 9}, {"chrome yellow", 1.2}, {"raw umber", 1.0}, medium=0.22}

local b_block = brush{kind="filbert", width=5.5, stiffness=0.85}
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_crack = brush{kind="round", width=1.0, point=1, stiffness=0.9}
local b_rigger= brush{kind="round", width=0.8, point=1, stiffness=0.9}

-- 2. Deepen and Enrich the Chamber Void (Solid, velvety dark cavern portal between the uprights)
b_block:load(p_cavern_pitch, 0.95)
b_block:stroke({{362, 396}, {395, 396}, {428, 396}}, {pressure=0.95})
b_block:stroke({{364, 408}, {395, 409}, {426, 408}}, {pressure=0.95})
b_block:stroke({{366, 420}, {395, 421}, {424, 420}}, {pressure=0.95})

-- Internal shaded rocky floor and inner rock faces of chamber
b_facet:load(p_granite_deep, 0.9)
b_facet:stroke({{368, 412}, {385, 418}, {405, 416}}, {pressure=0.75})
b_facet:stroke({{390, 422}, {412, 424}}, {pressure=0.7})

-- 3. The Left Orthostat: Massive, rugged, leaning Scandinavian granite boulder (width ~32 units)
b_block:load(p_granite_body, 0.95)
b_block:stroke({{334, 396}, {338, 412}, {342, 428}}, {pressure=0.95})
b_block:stroke({{344, 396}, {348, 412}, {352, 428}}, {pressure=0.95})
b_block:stroke({{354, 397}, {358, 412}, {362, 427}}, {pressure=0.9})

-- Rounded outer facet catching twilight
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{332, 396}, {335, 410}, {339, 422}, {342, 428}}, {pressure={0.8, 0.75, 0.6, 0.2}})
b_crack:load(p_granite_lit, 0.85)
b_crack:stroke({{332, 396}, {334, 408}, {336, 418}}, {pressure={0.6, 0.7, 0.2}})

-- Deep shadow on inner chamber face
b_crack:load(p_cavern_pitch, 0.95)
b_crack:stroke({{364, 396}, {366, 427}}, {pressure=0.9})

-- 4. The Right Orthostat: Massive, rugged, crystalline granite boulder (width ~32 units)
b_block:load(p_granite_body, 0.95)
b_block:stroke({{426, 397}, {428, 412}, {430, 427}}, {pressure=0.9})
b_block:stroke({{436, 396}, {438, 412}, {442, 428}}, {pressure=0.95})
b_block:stroke({{446, 396}, {448, 412}, {452, 428}}, {pressure=0.95})

-- Rounded outer facet catching twilight
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{446, 396}, {450, 410}, {454, 422}, {456, 428}}, {pressure={0.8, 0.75, 0.6, 0.2}})
b_crack:load(p_granite_lit, 0.85)
b_crack:stroke({{448, 396}, {452, 408}, {454, 418}}, {pressure={0.6, 0.7, 0.2}})

-- Deep shadow on inner chamber face
b_crack:load(p_cavern_pitch, 0.95)
b_crack:stroke({{426, 396}, {426, 427}}, {pressure=0.9})

-- 5. The Monumental Granite Capstone: Heavy, rough, angular erratic slab
-- A. Jagged, natural rock undercut shadow
b_crack:load(p_cavern_pitch, 0.95)
b_crack:stroke({{314, 401}, {335, 403}, {365, 401}, {400, 402}, {435, 401}, {462, 403}, {474, 400}}, {pressure={0.7, 0.95, 0.85, 0.95, 0.85, 0.95, 0.6}})

-- B. Capstone Front Face: Rich, mottled, weathered Scandinavian granite texture
b_block:load(p_granite_body, 0.95)
b_block:stroke({{318, 397}, {352, 398}, {395, 399}, {438, 398}, {470, 396}}, {pressure=0.95})
b_block:stroke({{324, 391}, {360, 392}, {405, 393}, {445, 392}, {468, 390}}, {pressure=0.95})

-- C. Top sky-facing facets: natural crystalline joint planes
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{322, 387}, {342, 378}, {370, 374}, {398, 372}}, {pressure={0.65, 0.85, 0.8, 0.5}})
b_facet:stroke({{400, 372}, {435, 374}, {460, 380}, {474, 390}}, {pressure={0.5, 0.8, 0.75, 0.4}})

-- D. Natural chips and fractured steps along the skyline
b_crack:load(p_granite_lit, 0.95)
b_crack:stroke({{316, 398}, {324, 388}, {344, 378}, {374, 373}, {400, 371}}, {pressure={0.3, 0.8, 0.7, 0.85, 0.4}})
b_crack:stroke({{402, 371}, {442, 374}, {468, 382}, {474, 392}}, {pressure={0.4, 0.8, 0.75, 0.3}})

-- E. Natural geological joint fractures
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{398, 372}, {395, 386}, {392, 400}}, {pressure={0.7, 0.8, 0.3}})
b_crack:stroke({{350, 376}, {354, 388}, {352, 398}}, {pressure={0.5, 0.65, 0.2}})
b_crack:stroke({{438, 374}, {440, 386}, {442, 398}}, {pressure={0.5, 0.65, 0.2}})

-- F. Crustose lichens encrusting the granite
local lichens = {
  {336, 384, p_lich_sage, 2.4}, {362, 378, p_lich_gold, 2.5},
  {382, 384, p_lich_sage, 2.0}, {418, 377, p_lich_gold, 2.4},
  {442, 380, p_lich_sage, 2.2}, {464, 387, p_lich_gold, 2.2}
}
for _, lp in ipairs(lichens) do
  b_facet:load(lp[3], 0.8)
  b_facet:touch(lp[1], lp[2], {pressure=0.6, drag={lp[4], 0.1}})
end

-- 6. Flanking Glacial Erratics: Weathered, chunky boulders bedded into heather
b_block:load(p_granite_body, 0.9)
b_block:stroke({{302, 426}, {312, 412}, {324, 410}, {328, 424}}, {pressure=0.85})
b_facet:load(p_granite_facet, 0.85)
b_facet:stroke({{310, 412}, {322, 410}, {328, 420}}, {pressure={0.4, 0.8, 0.4}})
b_crack:load(p_lich_sage, 0.75)
b_crack:touch(318, 412, {pressure=0.5, drag={1.5, 0.1}})

b_block:load(p_granite_body, 0.9)
b_block:stroke({{466, 426}, {476, 412}, {490, 410}, {494, 424}}, {pressure=0.85})
b_facet:load(p_granite_facet, 0.85)
b_facet:stroke({{474, 412}, {488, 410}, {494, 420}}, {pressure={0.4, 0.8, 0.4}})
b_crack:load(p_lich_gold, 0.75)
b_crack:touch(482, 412, {pressure=0.5, drag={1.5, 0.1}})

-- 7. Bed all Stones Naturally into the Tumulus Turf
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{285, 428}, {330, 430}, {375, 432}, {420, 432}, {465, 430}, {510, 428}}, {pressure=0.95})
b_block:load(p_heath_russet, 0.9)
b_block:stroke({{295, 434}, {340, 436}, {390, 438}, {440, 437}, {490, 435}}, {pressure=0.9})
b_block:load(p_moss_green, 0.85)
b_block:stroke({{320, 431}, {360, 433}, {405, 433}, {445, 431}}, {pressure=0.8})

-- 8. Subtle Silvery Twilight Sheen on the Top Crest of the Capstone
b_rigger:load(p_granite_sheen, 0.8)
b_rigger:stroke({{316, 398}, {324, 388}, {344, 378}, {374, 373}, {408, 371}, {442, 374}, {468, 382}}, {pressure={0.3, 0.75, 0.7, 0.8, 0.7, 0.25}})

-- 9. Fine Windswept Bent-Grass Tufts around the Dolmen Base
local function draw_curved_grass(bx, by, len, ang, bend, p)
  local mx = bx + len * 0.55 * math.cos(ang + bend * 0.5)
  local my = by - len * 0.55 * math.sin(ang + bend * 0.5)
  local tx = bx + len * math.cos(ang + bend)
  local ty = by - len * math.sin(ang + bend)
  b_rigger:load(p, 0.7)
  b_rigger:stroke({{bx, by}, {mx, my}, {tx, ty}}, {pressure={0.55, 0.04}, ramps={0.05, 0.3}})
end

local function draw_grass_cluster(cx, cy, n, h, lean)
  for i = 1, n do
    local bx = cx + rand(-4, 4)
    local by = cy + rand(-1.5, 1.5)
    local len = h * rand(0.75, 1.25)
    local ang = 1.57 + rand(-0.25, 0.25) + lean * 0.3
    local bnd = lean * rand(0.2, 0.5)
    local p = (rand(0, 1) < 0.65) and p_straw_sharp or p_granite_dark
    draw_curved_grass(bx, by, len, ang, bnd, p)
  end
end

local grasses = {
  {295, 429, 6, 14, -0.3}, {328, 430, 6, 13, -0.2}, {458, 430, 6, 13, 0.2}, {498, 428, 6, 14, 0.3}
}
for _, g in ipairs(grasses) do draw_grass_cluster(g[1], g[2], g[3], g[4], g[5]) end

print("Chunk 113 complete: Megalithic Tomb resculpted as authentic, crystalline, weathered Scandinavian granite.")

--@ chunk 115
-- Chunk 114: Final Master Touches — Crystalline Evening Star, Gilt Crescent Rim, and Grass Accents

-- 1. Piles
local p_star_pure    = pile{{"lead white", 35}, {"chrome yellow", 0.5}, {"yellow ochre", 0.5}, medium=0.1}
local p_moon_gilt    = pile{{"lead white", 32}, {"yellow ochre", 1.2}, {"chrome yellow", 0.4}, medium=0.12}
local p_straw_sharp  = pile{{"lead white", 16}, {"yellow ochre", 9}, {"chrome yellow", 1.2}, {"raw umber", 1.0}, medium=0.22}
local p_straw_shade  = pile{{"yellow ochre", 7}, {"raw umber", 6}, {"bone black", 2.5}, {"lead white", 2.0}, medium=0.2}

local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}

-- 2. The Brilliant Solitary Evening Star (Venus / Hesperus) at x = 688, y = 148
-- A sparkling diamond point of twilight light
b_rigger:load(p_star_pure, 1.0)
b_rigger:touch(688, 148, {pressure=0.8})
-- Delicate four-ray twinkle
b_rigger:stroke({{686.8, 148}, {689.2, 148}}, {pressure={0.2, 0.5, 0.2}})
b_rigger:stroke({{688, 146.8}, {688, 149.2}}, {pressure={0.2, 0.5, 0.2}})

-- 3. Enhance the Waxing Crescent Moon's sharp silver-gilt contour
b_rigger:load(p_moon_gilt, 0.95)
b_rigger:stroke({
  {645.0, 105.5}, {641.2, 108.0}, {638.8, 112.5}, {640.0, 117.0}, {643.8, 119.5}
}, {pressure={0.08, 0.85, 1.0, 0.8, 0.08}, ramps={0.05, 0.05}})

-- 4. A Few Final Windswept Bent-Grass Accents along the Crest
local function draw_curved_grass(bx, by, len, ang, bend, p)
  local mx = bx + len * 0.55 * math.cos(ang + bend * 0.5)
  local my = by - len * 0.55 * math.sin(ang + bend * 0.5)
  local tx = bx + len * math.cos(ang + bend)
  local ty = by - len * math.sin(ang + bend)
  b_rigger:load(p, 0.7)
  b_rigger:stroke({{bx, by}, {mx, my}, {tx, ty}}, {pressure={0.55, 0.04}, ramps={0.05, 0.3}})
end

local final_grasses = {
  {298, 427, 6, 13, -0.3}, {332, 428, 6, 12, -0.2},
  {456, 428, 6, 12, 0.2}, {492, 426, 6, 13, 0.3},
  {540, 434, 7, 15, 0.3}, {760, 436, 7, 15, 0.4}
}
for _, g in ipairs(final_grasses) do
  local p = (rand(0, 1) < 0.7) and p_straw_sharp or p_straw_shade
  draw_curved_grass(g[1], g[2], g[3], g[4], g[5], p)
end

print("Chunk 114 complete: Final master touches executed — Evening Star brilliant, Crescent Moon gilt, grasses swaying.")

--@ chunk 116
print("Drying at sky (200, 200):", drying(200, 200))
print("Drying at sky (750, 250):", drying(750, 250))
print("Drying at dolmen (400, 400):", drying(400, 400))
print("Drying at barrow (500, 550):", drying(500, 550))

--@ chunk 117
local t = wait(3 * 24 * 60)
print("Time now:", t)
print("Drying at sky (200, 200):", drying(200, 200))
print("Drying at sky (750, 250):", drying(750, 250))
print("Drying at dolmen (400, 400):", drying(400, 400))
print("Drying at barrow (500, 550):", drying(500, 550))

--@ chunk 118
local t = wait(2 * 24 * 60)
print("Time now:", t)
print("Drying at dolmen (400, 400):", drying(400, 400))

--@ chunk 119
local t = wait(4 * 24 * 60)
print("Time now:", t)
print("Drying at dolmen (400, 400):", drying(400, 400))

--@ chunk 120
local t = wait(10 * 24 * 60)
print("Time now:", t)
print("Drying at dolmen (400, 400):", drying(400, 400))

--@ chunk 121
-- Sculpt the Megalithic Tomb (Hünengrab) with Authentic Scandinavian Crystalline Granite

-- 1. Pigment Piles
local p_void_pitch   = pile{{"bone black", 15}, {"raw umber", 7}, medium=0.1}
local p_granite_deep = pile{{"bone black", 10}, {"raw umber", 8}, {"red earth", 2.0}, {"smalt", 2.0}, medium=0.15}
local p_granite_dark = pile{{"raw umber", 9}, {"bone black", 6}, {"red earth", 2.5}, {"yellow ochre", 2.0}, {"lead white", 2.0}, {"smalt", 1.5}, medium=0.18}
local p_granite_body = pile{{"lead white", 10}, {"raw umber", 7.0}, {"bone black", 3.5}, {"yellow ochre", 3.5}, {"red earth", 2.0}, {"smalt", 1.5}, medium=0.2}
local p_granite_facet= pile{{"lead white", 17}, {"yellow ochre", 4.0}, {"raw umber", 3.0}, {"red earth", 1.5}, {"smalt", 1.5}, {"bone black", 1.0}, medium=0.22}
local p_granite_lit  = pile{{"lead white", 24}, {"yellow ochre", 4.0}, {"red earth", 1.2}, {"smalt", 1.5}, {"raw umber", 0.8}, medium=0.22}
local p_granite_rim  = pile{{"lead white", 28}, {"yellow ochre", 3.0}, {"chrome yellow", 0.8}, {"smalt", 1.2}, medium=0.2}

local p_lich_orange  = pile{{"lead white", 8}, {"yellow ochre", 10}, {"chrome yellow", 3.0}, {"red earth", 2.5}, {"raw umber", 0.8}, medium=0.25}
local p_lich_sage    = pile{{"lead white", 14}, {"green earth", 7.5}, {"yellow ochre", 3.5}, {"raw umber", 1.5}, medium=0.25}

local p_peat_dark    = pile{{"raw umber", 11}, {"bone black", 8}, {"red earth", 3.0}, medium=0.15}
local p_heather_deep = pile{{"raw umber", 8}, {"bone black", 6}, {"red earth", 4.0}, {"yellow ochre", 2.0}, medium=0.18}

local b_block  = brush{kind="filbert", width=5.5, stiffness=0.85}
local b_facet  = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_crack  = brush{kind="round", width=1.0, point=1, stiffness=0.9}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}

-- 2. The Sacred Inner Chamber Void
-- Solid pitch darkness deep inside the tomb between the orthostats
local m_chamber = poly({{358, 400}, {432, 400}, {430, 434}, {358, 434}}, false)
work(m_chamber, {hand="detail", pile=p_void_pitch, angle=1.3, coverage=2.0, fill=true, clip=true})

-- Center-rear orthostat silhouette deep in the chamber gloom
b_block:load(p_granite_deep, 0.95)
b_block:stroke({{390, 402}, {392, 418}, {394, 432}}, {pressure=0.9})
b_block:stroke({{402, 402}, {404, 418}, {406, 432}}, {pressure=0.9})

-- 3. Left Orthostat: Massive, rugged, crystalline glacial boulder
-- Core body
local pts_lo_body = {{332, 399}, {362, 399}, {366, 435}, {336, 435}}
local m_lo_body = poly(pts_lo_body, false)
work(m_lo_body, {hand="detail", pile=p_granite_body, angle=1.4, coverage=1.8, fill=true, clip=true})

-- Inward-facing shadow plane (facing the dark chamber)
local m_lo_sh = poly({{350, 399}, {362, 399}, {366, 435}, {354, 435}}, false)
work(m_lo_sh, {hand="detail", pile=p_granite_dark, angle=1.4, coverage=1.6, fill=true, clip=true})

-- Outward-facing crystalline facet catching twilight light
local m_lo_facet = poly({{332, 399}, {344, 399}, {348, 435}, {336, 435}}, false)
work(m_lo_facet, {hand="detail", pile=p_granite_facet, angle=1.35, coverage=1.6, fill=true, clip=true})

-- Sharp weathered rock edge and stress crack
b_crack:load(p_granite_rim, 0.85)
b_crack:stroke({{332, 399}, {334, 416}, {337, 434}}, {pressure={0.6, 0.75, 0.3}})
b_crack:load(p_granite_deep, 0.9)
b_crack:stroke({{344, 402}, {346, 418}, {345, 430}}, {pressure={0.5, 0.65, 0.2}})

-- 4. Right Orthostat: Colossal, faceted crystalline boulder
-- Core body
local pts_ro_body = {{424, 399}, {456, 399}, {458, 435}, {426, 435}}
local m_ro_body = poly(pts_ro_body, false)
work(m_ro_body, {hand="detail", pile=p_granite_body, angle=1.4, coverage=1.8, fill=true, clip=true})

-- Inward-facing shadow plane (facing the chamber)
local m_ro_sh = poly({{424, 399}, {436, 399}, {438, 435}, {426, 435}}, false)
work(m_ro_sh, {hand="detail", pile=p_granite_dark, angle=1.4, coverage=1.6, fill=true, clip=true})

-- Outward-facing facet catching twilight
local m_ro_facet = poly({{444, 399}, {456, 399}, {458, 435}, {446, 435}}, false)
work(m_ro_facet, {hand="detail", pile=p_granite_facet, angle=1.45, coverage=1.6, fill=true, clip=true})

-- Sharp outer edge
b_crack:load(p_granite_rim, 0.85)
b_crack:stroke({{456, 399}, {457, 416}, {458, 434}}, {pressure={0.6, 0.75, 0.3}})
b_crack:load(p_granite_deep, 0.9)
b_crack:stroke({{442, 403}, {444, 418}, {445, 430}}, {pressure={0.5, 0.65, 0.2}})

-- 5. The Colossal Scandinavian Granite Capstone
-- Full opaque body pass covering all earlier smudges, pencil marks, and false shapes
local pts_cap = {
  {314, 402}, {320, 392}, {334, 382}, {355, 375}, {385, 371}, {420, 372},
  {450, 377}, {472, 386}, {478, 396}, {474, 404}, {450, 406}, {410, 404},
  {370, 405}, {335, 406}, {314, 402}
}
local m_cap = poly(pts_cap, false)
work(m_cap, {hand="body", pile=p_granite_body, angle=0.05, coverage=2.2, fill=true, clip=true})

-- Underside heavy overhang and deep cast shadow
local pts_cap_undercut = {
  {314, 402}, {335, 406}, {370, 405}, {410, 404}, {450, 406}, {474, 404},
  {476, 409}, {445, 411}, {405, 410}, {365, 411}, {330, 411}, {314, 402}
}
local m_cap_undercut = poly(pts_cap_undercut, false)
work(m_cap_undercut, {hand="detail", pile=p_void_pitch, angle=0, coverage=1.8, fill=true, clip=true})

-- Front face shadow and texture
local pts_cap_front = {
  {314, 402}, {324, 394}, {355, 388}, {395, 387}, {440, 388}, {472, 394},
  {474, 404}, {450, 406}, {410, 404}, {370, 405}, {335, 406}, {314, 402}
}
local m_cap_front = poly(pts_cap_front, false)
work(m_cap_front, {hand="detail", pile=p_granite_dark, angle=0.08, coverage=1.6, fill=true, clip=true})

-- Major skyward-facing crystalline cleavage facet (Western slope)
local pts_facet_west = {
  {314, 402}, {320, 392}, {334, 382}, {355, 375}, {385, 371},
  {388, 386}, {355, 388}, {324, 394}, {314, 402}
}
local m_facet_west = poly(pts_facet_west, false)
work(m_facet_west, {hand="detail", pile=p_granite_facet, angle=0.06, coverage=1.8, fill=true, clip=true})

-- Major skyward-facing facet (Eastern slope, catching warm twilight angle)
local pts_facet_east = {
  {385, 371}, {420, 372}, {450, 377}, {472, 386}, {478, 396},
  {472, 394}, {440, 388}, {395, 387}, {388, 386}, {385, 371}
}
local m_facet_east = poly(pts_facet_east, false)
work(m_facet_east, {hand="detail", pile=p_granite_lit, angle=-0.04, coverage=1.8, fill=true, clip=true})

-- Prominent diagonal geological cleavage fracture across the capstone at x ≈ 386
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{385, 371}, {388, 387}, {386, 405}}, {pressure={0.7, 0.85, 0.4}})
-- Secondary natural joint fractures
b_crack:stroke({{344, 378}, {348, 390}, {346, 404}}, {pressure={0.5, 0.65, 0.2}})
b_crack:stroke({{436, 375}, {438, 388}, {440, 404}}, {pressure={0.5, 0.65, 0.2}})

-- Chipped, crystalline rock rim along top skyline catching the pale twilight sky
b_crack:load(p_granite_rim, 0.9)
b_crack:stroke({{316, 400}, {322, 391}, {336, 381}, {358, 374}, {385, 371}}, {pressure={0.3, 0.7, 0.8, 0.85, 0.4}})
b_crack:stroke({{386, 371}, {420, 372}, {450, 377}, {472, 386}, {478, 396}}, {pressure={0.4, 0.85, 0.8, 0.7, 0.25}})

-- 6. Crustose Lichen Colonies Encrusting the Capstone
-- Authentic golden-orange Xanthoria and sage-green Lecanora
local lichens = {
  {330, 386, p_lich_orange, 2.2, 0.2},
  {352, 380, p_lich_sage,   2.5, 0.1},
  {375, 378, p_lich_orange, 2.0, 0.1},
  {408, 376, p_lich_sage,   2.4, -0.1},
  {432, 382, p_lich_orange, 2.2, -0.2},
  {460, 388, p_lich_sage,   2.0, -0.2},
  {370, 394, p_lich_sage,   1.8, 0.1},
  {418, 392, p_lich_orange, 2.0, -0.1}
}
for _, l in ipairs(lichens) do
  b_facet:load(l[3], 0.85)
  b_facet:touch(l[1], l[2], {pressure=0.6, drag={l[4], l[5]}})
end

-- 7. Flanking Glacial Erratics (Permanently cover the weird yellow arches!)
-- Left Erratic (x ≈ 295 to 330)
local pts_le = {{296, 428}, {304, 412}, {322, 408}, {332, 418}, {330, 434}, {298, 434}}
local m_le = poly(pts_le, false)
work(m_le, {hand="detail", pile=p_granite_body, angle=0.2, coverage=2.0, fill=true, clip=true})
local m_le_facet = poly({{304, 412}, {322, 408}, {330, 418}, {316, 420}}, false)
work(m_le_facet, {hand="detail", pile=p_granite_facet, angle=0.15, coverage=1.6, fill=true, clip=true})
b_crack:load(p_granite_deep, 0.9)
b_crack:stroke({{316, 410}, {318, 424}, {316, 434}}, {pressure={0.5, 0.65, 0.2}})
b_facet:load(p_lich_orange, 0.8)
b_facet:touch(312, 414, {pressure=0.55, drag={1.8, 0.1}})

-- Right Erratic (x ≈ 462 to 498)
local pts_re = {{464, 434}, {466, 418}, {480, 408}, {496, 414}, {498, 428}, {494, 434}}
local m_re = poly(pts_re, false)
work(m_re, {hand="detail", pile=p_granite_body, angle=-0.2, coverage=2.0, fill=true, clip=true})
local m_re_facet = poly({{468, 418}, {480, 408}, {494, 414}, {484, 420}}, false)
work(m_re_facet, {hand="detail", pile=p_granite_lit, angle=-0.15, coverage=1.6, fill=true, clip=true})
b_crack:load(p_granite_deep, 0.9)
b_crack:stroke({{482, 410}, {484, 424}, {486, 434}}, {pressure={0.5, 0.65, 0.2}})
b_facet:load(p_lich_sage, 0.8)
b_facet:touch(486, 414, {pressure=0.55, drag={1.8, 0.1}})

-- 8. Bed the Megalithic Tomb deeply into the ancient burial tumulus
-- Deep peaty earth and dark heather wrapping around the bases of all stones
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{285, 432}, {320, 434}, {355, 436}, {395, 437}, {435, 436}, {470, 434}, {505, 432}}, {pressure=0.95})
b_block:load(p_heather_deep, 0.9)
b_block:stroke({{290, 436}, {330, 438}, {380, 440}, {430, 439}, {480, 437}, {510, 435}}, {pressure=0.9})

print("Dolmen resculpted as authentic, monumental Scandinavian granite.")

--@ chunk 122
-- Remodel the Megalithic Tomb (Hünengrab) with Organic, Rugged Scandinavian Granite

-- 1. Piles
local p_cavern_pitch = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}
local p_granite_deep = pile{{"bone black", 9}, {"raw umber", 8}, {"smalt", 2.0}, {"red earth", 2.0}, medium=0.15}
local p_granite_dark = pile{{"raw umber", 9}, {"bone black", 5}, {"red earth", 2.5}, {"yellow ochre", 2.5}, {"lead white", 2.0}, medium=0.18}
local p_granite_body = pile{{"lead white", 10}, {"raw umber", 7.0}, {"yellow ochre", 4.0}, {"bone black", 3.0}, {"red earth", 2.0}, {"smalt", 1.2}, medium=0.2}
local p_granite_facet= pile{{"lead white", 18}, {"yellow ochre", 4.5}, {"raw umber", 2.5}, {"red earth", 1.5}, {"smalt", 1.2}, medium=0.22}
local p_granite_lit  = pile{{"lead white", 26}, {"yellow ochre", 4.0}, {"chrome yellow", 0.6}, {"red earth", 1.0}, {"raw umber", 0.8}, medium=0.22}
local p_granite_rim  = pile{{"lead white", 30}, {"yellow ochre", 2.5}, {"smalt", 1.2}, {"raw umber", 0.5}, medium=0.2}

local p_lich_orange  = pile{{"lead white", 8}, {"yellow ochre", 11}, {"chrome yellow", 3.5}, {"red earth", 2.5}, medium=0.25}
local p_lich_sage    = pile{{"lead white", 15}, {"green earth", 8.0}, {"yellow ochre", 3.5}, {"raw umber", 1.5}, medium=0.25}

local p_peat_dark    = pile{{"raw umber", 11}, {"bone black", 8}, {"red earth", 3.0}, medium=0.15}
local p_heather_deep = pile{{"raw umber", 8}, {"bone black", 6}, {"red earth", 4.5}, {"yellow ochre", 2.0}, medium=0.18}
local p_turf_moss    = pile{{"green earth", 6}, {"raw umber", 6}, {"yellow ochre", 5}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}

local b_block  = brush{kind="filbert", width=5.5, stiffness=0.85}
local b_facet  = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_crack  = brush{kind="round", width=1.0, point=1, stiffness=0.9}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}

-- 2. Soften and Organicize the Capstone Underside & Chamber Void
-- Break the flat horizontal black bar with irregular rock undercuts and natural chamber depth
b_block:load(p_cavern_pitch, 0.95)
b_block:stroke({{360, 403}, {395, 404}, {428, 403}}, {pressure=0.95})
b_block:stroke({{362, 414}, {395, 416}, {426, 415}}, {pressure=0.95})
b_block:stroke({{365, 425}, {395, 426}, {424, 425}}, {pressure=0.95})

-- Rocky, uneven floor and interior walls inside the burial chamber
b_facet:load(p_granite_deep, 0.9)
b_facet:stroke({{368, 412}, {385, 418}, {410, 415}}, {pressure=0.8})
b_facet:stroke({{375, 424}, {398, 427}, {418, 423}}, {pressure=0.75})

-- 3. Sculpt the Left Orthostat (Rugged Scandinavian Glacial Erratic Boulder)
-- Natural leaning boulder volume
b_block:load(p_granite_body, 0.95)
b_block:stroke({{334, 398}, {338, 414}, {342, 432}}, {pressure=0.95})
b_block:stroke({{344, 397}, {348, 414}, {352, 432}}, {pressure=0.95})
b_block:stroke({{354, 398}, {357, 414}, {360, 430}}, {pressure=0.9})

-- Outer rounded contour catching twilight
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{332, 398}, {335, 412}, {339, 424}, {342, 432}}, {pressure={0.7, 0.85, 0.65, 0.2}})
-- Inner shadowed face curving into chamber
b_facet:load(p_granite_dark, 0.9)
b_facet:stroke({{356, 399}, {359, 414}, {362, 430}}, {pressure=0.85})
-- Weathered fracture and rock chip
b_crack:load(p_granite_rim, 0.85)
b_crack:stroke({{332, 400}, {334, 412}, {336, 422}}, {pressure={0.5, 0.7, 0.2}})
b_crack:load(p_granite_deep, 0.9)
b_crack:stroke({{342, 404}, {344, 418}, {343, 428}}, {pressure={0.5, 0.6, 0.2}})

-- 4. Sculpt the Right Orthostat (Massive, Chunky Erratic Granite Pillar)
b_block:load(p_granite_body, 0.95)
b_block:stroke({{426, 398}, {429, 414}, {432, 432}}, {pressure=0.9})
b_block:stroke({{436, 397}, {439, 414}, {442, 432}}, {pressure=0.95})
b_block:stroke({{446, 398}, {449, 414}, {452, 432}}, {pressure=0.95})

-- Outer crystalline facet catching twilight
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{446, 398}, {450, 412}, {453, 424}, {455, 432}}, {pressure={0.75, 0.85, 0.65, 0.2}})
b_crack:load(p_granite_rim, 0.85)
b_crack:stroke({{448, 398}, {452, 410}, {454, 422}}, {pressure={0.6, 0.75, 0.2}})
-- Inner shadow
b_facet:load(p_granite_dark, 0.9)
b_facet:stroke({{426, 399}, {428, 414}, {430, 430}}, {pressure=0.85})

-- 5. Organicize and Model the Massive Capstone
-- Soften the low-poly look by blending rock planes with organic brush strokes
-- Underside: irregular chipped rock edge overhanging the uprights (NOT a flat bar!)
b_crack:load(p_cavern_pitch, 0.95)
b_crack:stroke({{316, 403}, {334, 404}, {352, 402}, {378, 404}, {405, 403}, {434, 404}, {458, 403}, {474, 401}},
  {pressure={0.6, 0.9, 0.8, 0.9, 0.85, 0.9, 0.8, 0.5}})

-- Capstone front face: weathered granite with warm scumbles
b_block:load(p_granite_body, 0.9)
b_block:stroke({{322, 397}, {355, 398}, {395, 399}, {435, 398}, {468, 396}}, {pressure=0.9})
b_facet:load(p_granite_dark, 0.85)
b_facet:stroke({{325, 400}, {360, 401}, {400, 402}, {440, 401}, {465, 399}}, {pressure=0.8})

-- Western sloping facet (weathered, crystalline)
b_facet:load(p_granite_facet, 0.85)
b_facet:stroke({{320, 393}, {342, 386}, {370, 382}, {386, 380}}, {pressure={0.6, 0.85, 0.8, 0.5}})
b_facet:stroke({{326, 396}, {350, 391}, {375, 388}, {388, 386}}, {pressure={0.5, 0.75, 0.7, 0.4}})

-- Eastern sloping facet (catching warm evening light)
b_facet:load(p_granite_lit, 0.85)
b_facet:stroke({{388, 380}, {415, 381}, {445, 384}, {468, 391}}, {pressure={0.5, 0.8, 0.75, 0.4}})
b_facet:stroke({{390, 386}, {420, 387}, {448, 390}, {466, 395}}, {pressure={0.4, 0.7, 0.65, 0.3}})

-- Natural rock fracture lines (jagged, weathered)
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{386, 372}, {389, 386}, {387, 402}}, {pressure={0.65, 0.8, 0.35}})
b_crack:stroke({{348, 380}, {352, 391}, {350, 402}}, {pressure={0.45, 0.6, 0.2}})
b_crack:stroke({{438, 378}, {440, 389}, {442, 402}}, {pressure={0.45, 0.6, 0.2}})

-- Top skyline: rough, crystalline chipped edge (breaking the smooth dome!)
b_crack:load(p_granite_rim, 0.9)
b_crack:stroke({{316, 400}, {324, 391}, {338, 383}, {362, 376}, {386, 372}}, {pressure={0.3, 0.75, 0.7, 0.8, 0.4}})
b_crack:stroke({{386, 372}, {412, 373}, {440, 376}, {464, 384}, {476, 394}}, {pressure={0.4, 0.8, 0.75, 0.7, 0.3}})

-- 6. Flanking Erratics (Integrated naturally into the knoll)
-- Left Boulder
b_block:load(p_granite_body, 0.9)
b_block:stroke({{298, 428}, {308, 416}, {320, 412}, {328, 422}}, {pressure=0.85})
b_facet:load(p_granite_facet, 0.85)
b_facet:stroke({{308, 416}, {318, 412}, {326, 418}}, {pressure={0.5, 0.8, 0.4}})
b_crack:load(p_granite_deep, 0.9)
b_crack:stroke({{316, 414}, {318, 426}}, {pressure=0.6})

-- Right Boulder
b_block:load(p_granite_body, 0.9)
b_block:stroke({{468, 428}, {476, 416}, {488, 412}, {494, 422}}, {pressure=0.85})
b_facet:load(p_granite_lit, 0.85)
b_facet:stroke({{474, 416}, {486, 412}, {492, 418}}, {pressure={0.5, 0.8, 0.4}})
b_crack:load(p_granite_deep, 0.9)
b_crack:stroke({{482, 414}, {484, 426}}, {pressure=0.6})

-- 7. Lichen Colonies Encrusting the Stones
local lichens = {
  {334, 387, p_lich_orange, 2.2, 0.2},
  {356, 381, p_lich_sage,   2.4, 0.1},
  {376, 378, p_lich_orange, 2.0, 0.1},
  {412, 376, p_lich_sage,   2.2, -0.1},
  {435, 381, p_lich_orange, 2.0, -0.1},
  {458, 387, p_lich_sage,   2.0, -0.2},
  {314, 415, p_lich_orange, 1.8, 0.1},
  {486, 415, p_lich_sage,   1.8, -0.1}
}
for _, l in ipairs(lichens) do
  b_facet:load(l[3], 0.8)
  b_facet:touch(l[1], l[2], {pressure=0.55, drag={l[4], l[5]}})
end

-- 8. Natural Bedding into Heather and Peat
-- Deep peaty earth and dark heather wrapping around the bases of all stones
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{285, 432}, {320, 434}, {355, 435}, {395, 436}, {435, 435}, {470, 434}, {505, 432}}, {pressure=0.95})
b_block:load(p_heather_deep, 0.9)
b_block:stroke({{290, 435}, {330, 437}, {380, 438}, {430, 437}, {480, 436}, {510, 434}}, {pressure=0.9})
b_facet:load(p_turf_moss, 0.85)
b_facet:stroke({{325, 435}, {365, 436}, {410, 436}, {455, 435}}, {pressure=0.75})

print("Dolmen remodeled with organic Scandinavian granite brushwork.")

--@ chunk 123
-- Perfect the Megalithic Tomb: Eliminate black bar, add rear orthostat, shape crystalline granite

-- 1. Piles
local p_cavern_deep  = pile{{"bone black", 12}, {"raw umber", 8}, {"red earth", 2}, medium=0.15}
local p_granite_sh   = pile{{"raw umber", 8}, {"bone black", 5}, {"red earth", 2.5}, {"smalt", 2.0}, {"lead white", 2.0}, medium=0.18}
local p_granite_rear = pile{{"raw umber", 7}, {"bone black", 4}, {"yellow ochre", 3}, {"lead white", 3.0}, {"red earth", 1.5}, medium=0.2}
local p_granite_body = pile{{"lead white", 10}, {"raw umber", 6.5}, {"yellow ochre", 4.0}, {"bone black", 3.0}, {"red earth", 2.0}, {"smalt", 1.2}, medium=0.2}
local p_granite_facet= pile{{"lead white", 18}, {"yellow ochre", 4.5}, {"raw umber", 2.5}, {"red earth", 1.5}, {"smalt", 1.2}, medium=0.22}
local p_granite_lit  = pile{{"lead white", 25}, {"yellow ochre", 4.0}, {"chrome yellow", 0.6}, {"red earth", 1.0}, {"raw umber", 0.8}, medium=0.22}
local p_granite_rim  = pile{{"lead white", 28}, {"yellow ochre", 2.5}, {"smalt", 1.2}, {"raw umber", 0.5}, medium=0.2}

local p_peat_dark    = pile{{"raw umber", 11}, {"bone black", 8}, {"red earth", 3.0}, medium=0.15}
local p_heather_deep = pile{{"raw umber", 8}, {"bone black", 6}, {"red earth", 4.5}, {"yellow ochre", 2.0}, medium=0.18}
local p_turf_moss    = pile{{"green earth", 6}, {"raw umber", 6}, {"yellow ochre", 5}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}

local b_block  = brush{kind="filbert", width=6.0, stiffness=0.85}
local b_facet  = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_crack  = brush{kind="round", width=1.0, point=1, stiffness=0.9}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}

-- 2. Build the Center-Rear Orthostat (closes the "garage door" into an authentic dolmen chamber)
-- The rear orthostat stands deep in the chamber between x = 378 and 416, y = 398 to 432
b_block:load(p_granite_rear, 0.95)
b_block:stroke({{382, 399}, {384, 415}, {386, 431}}, {pressure=0.95})
b_block:stroke({{394, 398}, {396, 415}, {398, 431}}, {pressure=0.95})
b_block:stroke({{406, 399}, {408, 415}, {410, 431}}, {pressure=0.95})

-- Soft shadow on left and right sides of rear orthostat
b_facet:load(p_cavern_deep, 0.9)
b_facet:stroke({{372, 401}, {374, 416}, {376, 430}}, {pressure=0.85})
b_facet:stroke({{418, 401}, {419, 416}, {420, 430}}, {pressure=0.85})
-- Top plane of rear orthostat catching dim light
b_facet:load(p_granite_sh, 0.85)
b_facet:stroke({{380, 403}, {398, 405}, {414, 404}}, {pressure=0.7})

-- 3. Extend Left and Right Orthostats up to MEET the Capstone Underside (eliminates the black bar!)
-- Left Orthostat: massive, solid Scandinavian granite pillar
b_block:load(p_granite_body, 0.95)
b_block:stroke({{330, 396}, {334, 414}, {338, 432}}, {pressure=0.95})
b_block:stroke({{342, 395}, {345, 414}, {348, 432}}, {pressure=0.95})
b_block:stroke({{354, 396}, {356, 414}, {358, 430}}, {pressure=0.95})

-- Left orthostat outer face catching twilight
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{328, 396}, {332, 412}, {336, 424}, {340, 432}}, {pressure={0.7, 0.85, 0.65, 0.2}})
b_crack:load(p_granite_rim, 0.85)
b_crack:stroke({{328, 398}, {330, 412}, {334, 424}}, {pressure={0.5, 0.7, 0.2}})

-- Right Orthostat: massive crystalline boulder meeting capstone
b_block:load(p_granite_body, 0.95)
b_block:stroke({{428, 396}, {430, 414}, {432, 432}}, {pressure=0.95})
b_block:stroke({{440, 395}, {442, 414}, {444, 432}}, {pressure=0.95})
b_block:stroke({{452, 396}, {454, 414}, {456, 432}}, {pressure=0.95})

-- Right orthostat outer face catching twilight
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{448, 396}, {452, 412}, {455, 424}, {457, 432}}, {pressure={0.75, 0.85, 0.65, 0.2}})
b_crack:load(p_granite_rim, 0.85)
b_crack:stroke({{450, 397}, {454, 410}, {456, 422}}, {pressure={0.6, 0.75, 0.2}})

-- 4. Remodel the Capstone Front and Underside
-- Cover the black bar on the left overhang (x = 314 to 330) and right overhang (x = 455 to 476)
b_block:load(p_granite_sh, 0.95)
b_block:stroke({{316, 401}, {332, 401}}, {pressure=0.9})
b_block:stroke({{452, 401}, {474, 400}}, {pressure=0.9})

-- Natural rock shadow immediately under the contact points (narrow, jagged, organic)
b_crack:load(p_cavern_deep, 0.95)
b_crack:stroke({{328, 399}, {345, 398}, {362, 400}}, {pressure={0.4, 0.75, 0.4}})
b_crack:stroke({{424, 400}, {442, 398}, {458, 399}}, {pressure={0.4, 0.75, 0.4}})
-- Chamber shadow under capstone center
b_crack:stroke({{362, 400}, {395, 401}, {424, 400}}, {pressure=0.8})

-- Solidify capstone body with rich, mottled Scandinavian granite
b_block:load(p_granite_body, 0.95)
b_block:stroke({{320, 396}, {355, 396}, {395, 397}, {435, 396}, {470, 394}}, {pressure=0.95})
b_block:stroke({{326, 390}, {360, 390}, {400, 391}, {440, 390}, {466, 388}}, {pressure=0.95})

-- Model the heavy western shoulder (x = 318 to 386)
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{320, 394}, {342, 386}, {368, 382}, {386, 380}}, {pressure={0.65, 0.85, 0.8, 0.5}})
b_facet:stroke({{324, 398}, {350, 392}, {374, 388}, {388, 386}}, {pressure={0.5, 0.75, 0.7, 0.4}})

-- Eastern stepped facet (x = 386 to 472, crystalline cleavage plane catching twilight)
b_facet:load(p_granite_lit, 0.9)
b_facet:stroke({{386, 380}, {415, 381}, {445, 384}, {470, 391}}, {pressure={0.55, 0.85, 0.8, 0.45}})
b_facet:stroke({{388, 386}, {420, 387}, {448, 390}, {468, 395}}, {pressure={0.45, 0.75, 0.7, 0.35}})

-- Sharp crystalline cleavage fracture across capstone at x ≈ 386
b_crack:load(p_granite_sh, 0.95)
b_crack:stroke({{385, 372}, {388, 386}, {386, 400}}, {pressure={0.65, 0.8, 0.35}})
b_crack:stroke({{346, 380}, {350, 390}, {348, 400}}, {pressure={0.45, 0.6, 0.2}})
b_crack:stroke({{440, 378}, {442, 389}, {444, 400}}, {pressure={0.45, 0.6, 0.2}})

-- Top skyline: rough, crystalline chipped edge
b_crack:load(p_granite_rim, 0.9)
b_crack:stroke({{316, 400}, {324, 391}, {338, 383}, {362, 376}, {385, 372}}, {pressure={0.3, 0.75, 0.7, 0.8, 0.4}})
b_crack:stroke({{385, 372}, {412, 373}, {440, 376}, {464, 384}, {474, 394}}, {pressure={0.4, 0.8, 0.75, 0.7, 0.3}})

-- Lichens on the capstone
local p_lich_orange = pile{{"lead white", 8}, {"yellow ochre", 11}, {"chrome yellow", 3.5}, {"red earth", 2.5}, medium=0.25}
local p_lich_sage   = pile{{"lead white", 15}, {"green earth", 8.0}, {"yellow ochre", 3.5}, {"raw umber", 1.5}, medium=0.25}
local lichens = {
  {334, 386, p_lich_orange, 2.2, 0.2},
  {356, 381, p_lich_sage,   2.4, 0.1},
  {376, 378, p_lich_orange, 2.0, 0.1},
  {412, 376, p_lich_sage,   2.2, -0.1},
  {435, 381, p_lich_orange, 2.0, -0.1},
  {460, 388, p_lich_sage,   2.0, -0.2},
}
for _, l in ipairs(lichens) do
  b_facet:load(l[3], 0.8)
  b_facet:touch(l[1], l[2], {pressure=0.55, drag={l[4], l[5]}})
end

-- 5. Flanking Erratics & Ground Bedding
-- Left Erratic (x ≈ 298 to 330)
b_block:load(p_granite_body, 0.9)
b_block:stroke({{298, 428}, {308, 416}, {320, 412}, {328, 422}}, {pressure=0.85})
b_facet:load(p_granite_facet, 0.85)
b_facet:stroke({{308, 416}, {318, 412}, {326, 418}}, {pressure={0.5, 0.8, 0.4}})
b_crack:load(p_granite_sh, 0.9)
b_crack:stroke({{316, 414}, {318, 426}}, {pressure=0.6})

-- Right Erratic (x ≈ 468 to 496)
b_block:load(p_granite_body, 0.9)
b_block:stroke({{468, 428}, {476, 416}, {488, 412}, {494, 422}}, {pressure=0.85})
b_facet:load(p_granite_lit, 0.85)
b_facet:stroke({{474, 416}, {486, 412}, {492, 418}}, {pressure={0.5, 0.8, 0.4}})
b_crack:load(p_granite_sh, 0.9)
b_crack:stroke({{482, 414}, {484, 426}}, {pressure=0.6})

-- Bed all stones with peat, heather, and moss
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{285, 432}, {320, 434}, {355, 435}, {395, 436}, {435, 435}, {470, 434}, {505, 432}}, {pressure=0.95})
b_block:load(p_heather_deep, 0.9)
b_block:stroke({{290, 435}, {330, 437}, {380, 438}, {430, 437}, {480, 436}, {510, 434}}, {pressure=0.9})
b_facet:load(p_turf_moss, 0.85)
b_facet:stroke({{325, 435}, {365, 436}, {410, 436}, {455, 435}}, {pressure=0.75})

print("Dolmen refined: rear orthostat placed, orthostats connected to capstone, black bar removed.")

--@ chunk 124
-- Solidify and Sculpt the Megalithic Tomb with 100% Opaque Body Paint

-- 1. Piles (Stiff, high-hiding lead-white and earth body piles)
local p_cavern_pitch = pile{{"bone black", 14}, {"raw umber", 8}, medium=0.1}
local p_granite_dark = pile{{"raw umber", 9}, {"bone black", 6}, {"red earth", 2.5}, {"lead white", 3.0}, {"yellow ochre", 2.0}, {"smalt", 1.5}, medium=0.15}
local p_granite_body = pile{{"lead white", 14}, {"raw umber", 7.0}, {"yellow ochre", 4.0}, {"bone black", 3.0}, {"red earth", 2.0}, {"smalt", 1.2}, medium=0.18}
local p_granite_facet= pile{{"lead white", 22}, {"yellow ochre", 4.5}, {"raw umber", 2.5}, {"red earth", 1.5}, {"smalt", 1.2}, medium=0.18}
local p_granite_lit  = pile{{"lead white", 28}, {"yellow ochre", 4.0}, {"chrome yellow", 0.6}, {"red earth", 1.0}, {"raw umber", 0.8}, medium=0.18}

local p_peat_dark    = pile{{"raw umber", 11}, {"bone black", 8}, {"red earth", 3.0}, medium=0.15}
local p_heather_deep = pile{{"raw umber", 8}, {"bone black", 6}, {"red earth", 4.5}, {"yellow ochre", 2.0}, medium=0.18}

-- 2. Massive Left Orthostat (x = 320 to 372, y = 394 to 435)
-- Broad, chunky glacial erratic boulder that completely covers the left black overhang!
local pts_lo = {{322, 396}, {370, 396}, {372, 434}, {326, 434}}
local m_lo = poly(pts_lo, false)
work(m_lo, {hand="body", pile=p_granite_body, angle=1.4, coverage=2.2, fill=true, clip=true})

-- Outer facet of left orthostat catching twilight
local pts_lo_f = {{322, 396}, {344, 396}, {348, 434}, {326, 434}}
local m_lo_f = poly(pts_lo_f, false)
work(m_lo_f, {hand="detail", pile=p_granite_facet, angle=1.35, coverage=1.8, fill=true, clip=true})

-- Inner shadow plane facing chamber
local pts_lo_sh = {{358, 396}, {370, 396}, {372, 434}, {360, 434}}
local m_lo_sh = poly(pts_lo_sh, false)
work(m_lo_sh, {hand="detail", pile=p_granite_dark, angle=1.45, coverage=1.8, fill=true, clip=true})

-- 3. Massive Right Orthostat (x = 414 to 468, y = 394 to 435)
-- Broad, chunky Scandinavian granite block that completely covers the right black overhang!
local pts_ro = {{414, 396}, {464, 396}, {466, 434}, {418, 434}}
local m_ro = poly(pts_ro, false)
work(m_ro, {hand="body", pile=p_granite_body, angle=1.4, coverage=2.2, fill=true, clip=true})

-- Outer facet of right orthostat catching twilight
local pts_ro_f = {{440, 396}, {464, 396}, {466, 434}, {444, 434}}
local m_ro_f = poly(pts_ro_f, false)
work(m_ro_f, {hand="detail", pile=p_granite_facet, angle=1.4, coverage=1.8, fill=true, clip=true})

-- Inner shadow plane facing chamber
local pts_ro_sh = {{414, 396}, {426, 396}, {428, 434}, {418, 434}}
local m_ro_sh = poly(pts_ro_sh, false)
work(m_ro_sh, {hand="detail", pile=p_granite_dark, angle=1.4, coverage=1.8, fill=true, clip=true})

-- 4. Deep Sacred Chamber Void (Between the massive uprights: x = 370 to 416, y = 396 to 434)
local pts_void = {{370, 396}, {416, 396}, {418, 434}, {372, 434}}
local m_void = poly(pts_void, false)
work(m_void, {hand="detail", pile=p_cavern_pitch, angle=1.3, coverage=2.0, fill=true, clip=true})

-- Center-rear orthostat silhouette inside chamber gloom
local pts_co = {{384, 398}, {404, 398}, {406, 432}, {386, 432}}
local m_co = poly(pts_co, false)
work(m_co, {hand="detail", pile=p_granite_dark, angle=1.4, coverage=1.8, fill=true, clip=true})

-- 5. The Massive Scandinavian Granite Capstone
-- Redesigned with organic, rugged, asymmetrical Scandinavian erratic block silhouette
local pts_cap = {
  {318, 398}, {324, 390}, {340, 381}, {365, 375}, {395, 374}, {430, 376},
  {455, 382}, {470, 390}, {468, 398}, {440, 399}, {400, 398}, {360, 399},
  {330, 400}, {318, 398}
}
local m_cap = poly(pts_cap, false)
work(m_cap, {hand="body", pile=p_granite_body, angle=0.05, coverage=2.4, fill=true, clip=true})

-- Front face shadow and texture
local pts_cap_sh = {
  {318, 398}, {328, 392}, {360, 388}, {400, 389}, {445, 389}, {468, 393},
  {468, 398}, {440, 399}, {400, 398}, {360, 399}, {330, 400}, {318, 398}
}
local m_cap_sh = poly(pts_cap_sh, false)
work(m_cap_sh, {hand="detail", pile=p_granite_dark, angle=0.08, coverage=1.8, fill=true, clip=true})

-- Skyward-facing Western facet
local pts_f_west = {
  {318, 398}, {324, 390}, {340, 381}, {365, 375}, {395, 374},
  {396, 388}, {360, 388}, {328, 392}, {318, 398}
}
local m_f_west = poly(pts_f_west, false)
work(m_f_west, {hand="detail", pile=p_granite_facet, angle=0.06, coverage=1.8, fill=true, clip=true})

-- Skyward-facing Eastern facet (warm twilight reflection)
local pts_f_east = {
  {395, 374}, {430, 376}, {455, 382}, {470, 390}, {468, 393},
  {445, 389}, {400, 389}, {396, 388}, {395, 374}
}
local m_f_east = poly(pts_f_east, false)
work(m_f_east, {hand="detail", pile=p_granite_lit, angle=-0.04, coverage=1.8, fill=true, clip=true})

-- Underside contact cast shadows (organic, nestled over the orthostats)
local b_crack = brush{kind="round", width=1.1, point=1, stiffness=0.9}
b_crack:load(p_cavern_pitch, 0.95)
b_crack:stroke({{320, 398}, {345, 397}, {370, 398}}, {pressure={0.4, 0.8, 0.5}})
b_crack:stroke({{370, 398}, {395, 398}, {415, 398}}, {pressure=0.95})
b_crack:stroke({{415, 398}, {440, 397}, {466, 397}}, {pressure={0.5, 0.8, 0.4}})

-- Major crystalline cleavage fracture across capstone at x ≈ 395
b_crack:load(p_granite_dark, 0.95)
b_crack:stroke({{395, 374}, {397, 387}, {396, 398}}, {pressure={0.7, 0.85, 0.35}})
b_crack:stroke({{350, 379}, {353, 389}, {351, 399}}, {pressure={0.5, 0.65, 0.2}})
b_crack:stroke({{442, 379}, {444, 389}, {445, 399}}, {pressure={0.5, 0.65, 0.2}})

-- Top crystalline skyline rim catching twilight
local p_granite_rim = pile{{"lead white", 28}, {"yellow ochre", 2.5}, {"smalt", 1.2}, {"raw umber", 0.5}, medium=0.2}
b_crack:load(p_granite_rim, 0.9)
b_crack:stroke({{320, 394}, {330, 386}, {348, 379}, {372, 375}, {395, 374}}, {pressure={0.3, 0.75, 0.7, 0.85, 0.4}})
b_crack:stroke({{395, 374}, {420, 375}, {445, 379}, {462, 386}, {469, 392}}, {pressure={0.4, 0.85, 0.75, 0.65, 0.25}})

-- Lichens nestled in fractures
local p_lich_orange = pile{{"lead white", 8}, {"yellow ochre", 11}, {"chrome yellow", 3.5}, {"red earth", 2.5}, medium=0.25}
local p_lich_sage   = pile{{"lead white", 15}, {"green earth", 8.0}, {"yellow ochre", 3.5}, {"raw umber", 1.5}, medium=0.25}
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local lichens = {
  {338, 384, p_lich_orange, 2.2, 0.2},
  {362, 378, p_lich_sage,   2.4, 0.1},
  {382, 376, p_lich_orange, 2.0, 0.1},
  {418, 377, p_lich_sage,   2.2, -0.1},
  {442, 381, p_lich_orange, 2.0, -0.1},
  {460, 388, p_lich_sage,   2.0, -0.2},
  {332, 412, p_lich_orange, 1.8, 0.1},
  {458, 412, p_lich_sage,   1.8, -0.1}
}
for _, l in ipairs(lichens) do
  b_facet:load(l[3], 0.8)
  b_facet:touch(l[1], l[2], {pressure=0.55, drag={l[4], l[5]}})
end

-- 6. Flanking Erratics (Integrated naturally into the knoll)
local pts_le = {{294, 428}, {304, 414}, {318, 410}, {326, 420}, {324, 434}, {296, 434}}
local m_le = poly(pts_le, false)
work(m_le, {hand="detail", pile=p_granite_body, angle=0.2, coverage=2.0, fill=true, clip=true})
local pts_le_f = {{304, 414}, {318, 410}, {326, 420}, {314, 422}}
local m_le_f = poly(pts_le_f, false)
work(m_le_f, {hand="detail", pile=p_granite_facet, angle=0.15, coverage=1.6, fill=true, clip=true})

local pts_re = {{462, 434}, {466, 418}, {480, 410}, {494, 416}, {496, 428}, {490, 434}}
local m_re = poly(pts_re, false)
work(m_re, {hand="detail", pile=p_granite_body, angle=-0.2, coverage=2.0, fill=true, clip=true})
local pts_re_f = {{468, 418}, {480, 410}, {492, 416}, {482, 420}}
local m_re_f = poly(pts_re_f, false)
work(m_re_f, {hand="detail", pile=p_granite_lit, angle=-0.15, coverage=1.6, fill=true, clip=true})

-- 7. Bed deeply into peat and dark heather
local b_block = brush{kind="filbert", width=6.0, stiffness=0.85}
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{285, 432}, {320, 434}, {355, 435}, {395, 436}, {435, 435}, {470, 434}, {505, 432}}, {pressure=0.95})
b_block:load(p_heather_deep, 0.9)
b_block:stroke({{290, 435}, {330, 437}, {380, 438}, {430, 437}, {480, 436}, {510, 434}}, {pressure=0.9})

print("Dolmen solidified with 100% opaque body paint, black bar fully covered, chamber proportioned.")

--@ chunk 125
-- Clean up above the capstone and shape a rugged, lower, crystalline rock contour

-- 1. Sky glow pile matching the local twilight sky at y ≈ 350 to 380
local p_sky_glow = pile{{"lead white", 27}, {"yellow ochre", 5.0}, {"chrome yellow", 2.0}, {"vermilion", 0.45}, {"raw umber", 0.25}, medium=0.22}
local p_granite_rim = pile{{"lead white", 26}, {"yellow ochre", 3.5}, {"chrome yellow", 0.6}, {"smalt", 1.2}, {"raw umber", 0.5}, medium=0.2}
local p_heather_deep = pile{{"raw umber", 9}, {"bone black", 7}, {"red earth", 4.0}, medium=0.15}

-- 2. Carve back the high dome peak above the capstone with sky body paint
-- This erases the old dome remnant and the black ticks above the stone (y = 350 to 376, x = 310 to 480)
local m_sky_trim = poly({
  {310, 350}, {485, 350}, {485, 388},
  {468, 386}, {435, 377}, {395, 376}, {360, 378}, {335, 384}, {312, 396}, {310, 350}
}, false)
work(m_sky_trim, {hand="body", pile=p_sky_glow, angle=0, order="across", coverage=2.2, fill=true, clip=true})

-- Soft horizontal blend of this sky patch to integrate seamlessly into surrounding twilight
local m_sky_blend = rect(305, 345, 185, 40):soften(6)
blend(m_sky_blend, {angle=0})

-- 3. Cover the black wedge on the far right of the capstone (x = 464 to 478, y = 396 to 406)
local pts_re_cover = {{464, 396}, {478, 396}, {478, 408}, {464, 408}}
local m_re_cover = poly(pts_re_cover, false)
work(m_re_cover, {hand="detail", pile=p_heather_deep, angle=0, coverage=2.0, fill=true, clip=true})

-- 4. Crisp, rugged, crystalline granite rim along the newly lowered capstone top
local b_crack = brush{kind="round", width=1.1, point=1, stiffness=0.9}
b_crack:load(p_granite_rim, 0.9)
b_crack:stroke({{314, 398}, {326, 390}, {342, 383}, {368, 379}, {395, 376}}, {pressure={0.3, 0.75, 0.7, 0.85, 0.4}})
b_crack:stroke({{395, 376}, {420, 377}, {445, 381}, {464, 388}, {470, 394}}, {pressure={0.4, 0.85, 0.75, 0.65, 0.25}})

print("Capstone lowered and trimmed with sky body paint; black wedge and upper ticks eliminated.")

--@ chunk 126
-- Construct the Monumental Ancient Veteran Oak (Quercus robur)

-- 1. Pigment Piles
local p_bark_core   = pile{{"bone black", 13}, {"raw umber", 8}, {"red earth", 2.5}, medium=0.14}
local p_bark_furrow = pile{{"bone black", 15}, {"raw umber", 7}, medium=0.12}
local p_bark_mid    = pile{{"raw umber", 8}, {"bone black", 4}, {"yellow ochre", 3.5}, {"lead white", 2.5}, {"red earth", 1.5}, medium=0.18}
local p_bark_ridge  = pile{{"yellow ochre", 5.5}, {"raw umber", 5.0}, {"lead white", 5.0}, {"bone black", 1.5}, {"red earth", 1.0}, medium=0.2}

local p_hollow_void = pile{{"bone black", 16}, {"raw umber", 6}, medium=0.1}
local p_woundwood   = pile{{"raw umber", 6}, {"yellow ochre", 5}, {"lead white", 4}, {"red earth", 2}, medium=0.2}

local p_stag_dark   = pile{{"bone black", 8}, {"raw umber", 8}, {"lead white", 4}, {"smalt", 1.5}, medium=0.16}
local p_stag_bleach = pile{{"lead white", 18}, {"raw umber", 4.5}, {"yellow ochre", 2.5}, {"smalt", 1.5}, medium=0.18}
local p_stag_tip    = pile{{"lead white", 24}, {"raw umber", 3.0}, {"yellow ochre", 2.0}, {"smalt", 1.2}, medium=0.18}

local p_leaf_russet = pile{{"red earth", 7}, {"yellow ochre", 6}, {"raw umber", 5}, {"lead white", 2}, medium=0.2}
local p_leaf_gold   = pile{{"yellow ochre", 11}, {"chrome yellow", 2.5}, {"red earth", 2.5}, {"lead white", 3}, medium=0.22}
local p_leaf_rim    = pile{{"lead white", 14}, {"yellow ochre", 10}, {"chrome yellow", 3.0}, medium=0.2}

local p_turf_deep   = pile{{"raw umber", 11}, {"bone black", 8}, {"red earth", 3.0}, medium=0.15}

-- 2. Solid Trunk Bole & Root Buttresses
-- Define the authentic hollowed ancient trunk contour from roots to main fork
local pts_trunk = {
  {236, 458}, {246, 442}, {254, 420}, {258, 395}, {260, 370},
  {268, 360}, {278, 360}, {284, 372}, {282, 395}, {284, 420},
  {294, 442}, {312, 456}, {296, 460}, {276, 458}, {256, 460}, {236, 458}
}
local m_trunk = poly(pts_trunk, false)
work(m_trunk, {hand="body", pile=p_bark_core, angle=1.5, coverage=2.4, fill=true, clip=true})

-- Deep longitudinal bark furrows and ancient texture
local b_furrow = brush{kind="round", width=1.4, point=1, stiffness=0.9}
local b_ridge  = brush{kind="round", width=1.8, point=1, stiffness=0.85}

-- Furrows running down the bole
b_furrow:load(p_bark_furrow, 0.95)
b_furrow:stroke({{262, 365}, {260, 395}, {256, 425}, {248, 452}}, {pressure={0.8, 0.9, 0.8, 0.3}})
b_furrow:stroke({{270, 365}, {268, 395}, {266, 425}, {262, 455}}, {pressure={0.8, 0.9, 0.8, 0.4}})
b_furrow:stroke({{278, 368}, {278, 395}, {276, 425}, {278, 455}}, {pressure={0.8, 0.9, 0.8, 0.4}})
b_furrow:stroke({{282, 385}, {282, 415}, {288, 440}, {304, 455}}, {pressure={0.7, 0.85, 0.8, 0.3}})

-- Raised weathered bark ridges catching evening light
b_ridge:load(p_bark_mid, 0.9)
b_ridge:stroke({{264, 365}, {263, 395}, {258, 422}, {252, 450}}, {pressure={0.5, 0.75, 0.7, 0.2}})
b_ridge:stroke({{274, 366}, {273, 395}, {270, 422}, {270, 454}}, {pressure={0.5, 0.75, 0.7, 0.2}})
b_ridge:stroke({{280, 375}, {280, 405}, {282, 430}, {296, 452}}, {pressure={0.5, 0.75, 0.7, 0.2}})

-- Highlights along crest of bark ridges
b_furrow:load(p_bark_ridge, 0.85)
b_furrow:stroke({{264, 380}, {263, 405}, {259, 430}}, {pressure={0.3, 0.65, 0.2}})
b_furrow:stroke({{274, 380}, {273, 405}, {271, 430}}, {pressure={0.3, 0.65, 0.2}})

-- Hollow cavity in the ancient trunk (x = 266 to 275, y = 398 to 430)
local pts_hollow = {{267, 402}, {274, 400}, {275, 426}, {268, 428}, {267, 402}}
local m_hollow = poly(pts_hollow, false)
work(m_hollow, {hand="detail", pile=p_hollow_void, angle=1.5, coverage=2.0, fill=true, clip=true})

-- Rounded wound-wood calluses framing the hollow
b_ridge:load(p_woundwood, 0.9)
b_ridge:stroke({{266, 400}, {265, 414}, {267, 429}}, {pressure={0.6, 0.85, 0.5}})
b_ridge:stroke({{274, 398}, {276, 414}, {275, 427}}, {pressure={0.6, 0.85, 0.5}})

-- Bed roots firmly into dark soil
local b_bole = brush{kind="filbert", width=5.5, stiffness=0.85}
b_bole:load(p_turf_deep, 0.95)
b_bole:stroke({{225, 456}, {255, 460}, {285, 460}, {320, 454}}, {pressure=0.95})

-- 3. The Great Living Eastern Bough (Arching over the Megalithic Tomb)
-- A solid tapered muscular ribbon extending from (278, 362) across to (485, 328)
local pts_east_bough = {
  {276, 355}, {315, 345}, {360, 335}, {410, 326}, {455, 320}, {485, 322},
  {485, 332}, {455, 332}, {410, 338}, {360, 348}, {315, 358}, {280, 368}
}
local m_east_bough = poly(pts_east_bough, false)
work(m_east_bough, {hand="body", pile=p_bark_core, angle=0.15, coverage=2.2, fill=true, clip=true})

-- Bark modeling and light on top of eastern bough
b_ridge:load(p_bark_mid, 0.9)
b_ridge:stroke({{278, 356}, {315, 347}, {360, 337}, {410, 328}, {455, 322}, {485, 324}},
  {pressure={0.7, 0.75, 0.7, 0.65, 0.5, 0.3}, ramps={0.02, 0.05}})
b_furrow:load(p_bark_ridge, 0.85)
b_furrow:stroke({{315, 347}, {360, 337}, {410, 328}, {450, 322}},
  {pressure={0.3, 0.6, 0.55, 0.2}})

-- Swollen branch collar where eastern bough joins trunk
b_bole:load(p_bark_core, 0.95)
b_bole:stroke({{274, 358}, {282, 364}, {280, 374}}, {pressure=0.9})
b_ridge:load(p_bark_mid, 0.85)
b_ridge:stroke({{274, 356}, {284, 362}}, {pressure={0.4, 0.8, 0.3}})

-- Secondary Limbs off Eastern Bough:
-- E1: Arches upward into twilight sky above dolmen
local pts_e1 = {
  {342, 340}, {360, 318}, {380, 290}, {398, 264},
  {404, 267}, {386, 293}, {366, 322}, {348, 343}
}
work(poly(pts_e1, false), {hand="body", pile=p_bark_core, angle=1.0, coverage=2.0, fill=true, clip=true})
b_ridge:load(p_bark_mid, 0.85)
b_ridge:stroke({{344, 339}, {362, 317}, {382, 289}, {400, 263}}, {pressure={0.6, 0.65, 0.5, 0.2}})

-- E2: Outer upward limb reaching eastward
local pts_e2 = {
  {412, 332}, {435, 308}, {458, 282}, {476, 260},
  {481, 263}, {463, 285}, {440, 312}, {418, 335}
}
work(poly(pts_e2, false), {hand="body", pile=p_bark_core, angle=0.8, coverage=2.0, fill=true, clip=true})
b_ridge:load(p_bark_mid, 0.85)
b_ridge:stroke({{414, 331}, {436, 307}, {460, 282}, {478, 259}}, {pressure={0.55, 0.6, 0.45, 0.2}})

-- E3: Lower limb sheltering east of capstone
local b_limb = brush{kind="round", width=3.2, point=1, stiffness=0.85}
b_limb:load(p_bark_core, 0.95)
b_limb:stroke({{455, 328}, {480, 340}, {504, 350}, {524, 356}}, {pressure={0.85, 0.7, 0.5, 0.2}, ramps={0.02, 0.1}})

-- 4. The Living Western Bough (Reaching out to the left knoll)
local pts_west_bough = {
  {262, 356}, {225, 344}, {185, 330}, {145, 312}, {115, 295},
  {115, 304}, {145, 322}, {185, 340}, {225, 355}, {260, 368}
}
local m_west_bough = poly(pts_west_bough, false)
work(m_west_bough, {hand="body", pile=p_bark_core, angle=-0.15, coverage=2.2, fill=true, clip=true})

-- Bark light on western bough
b_ridge:load(p_bark_mid, 0.9)
b_ridge:stroke({{260, 357}, {225, 345}, {185, 331}, {145, 313}, {115, 296}},
  {pressure={0.7, 0.75, 0.65, 0.55, 0.25}, ramps={0.02, 0.05}})

-- Western collar
b_bole:load(p_bark_core, 0.95)
b_bole:stroke({{264, 358}, {258, 364}, {260, 374}}, {pressure=0.9})

-- Secondary Western Limbs:
-- W1: Upward limb reaching toward zenith west
local pts_w1 = {
  {190, 334}, {175, 308}, {158, 278}, {142, 248},
  {147, 246}, {163, 276}, {180, 306}, {196, 332}
}
work(poly(pts_w1, false), {hand="body", pile=p_bark_core, angle=-1.1, coverage=2.0, fill=true, clip=true})
b_ridge:load(p_bark_mid, 0.85)
b_ridge:stroke({{192, 333}, {177, 307}, {160, 277}, {144, 246}}, {pressure={0.55, 0.6, 0.5, 0.2}})

-- W2: Lower droop reaching west
b_limb:load(p_bark_core, 0.95)
b_limb:stroke({{145, 318}, {125, 336}, {102, 352}}, {pressure={0.8, 0.6, 0.2}, ramps={0.02, 0.1}})

-- 5. The Bleached Stag-Head Crown Aloft (Spearing into the Cobalt Zenith)
-- Central dead spire (rising to y = 125!)
local pts_stag_center = {
  {265, 360}, {264, 315}, {262, 268}, {259, 218}, {256, 168}, {252, 125},
  {255, 125}, {260, 168}, {264, 218}, {268, 268}, {271, 315}, {273, 360}
}
local m_stag_center = poly(pts_stag_center, false)
work(m_stag_center, {hand="body", pile=p_stag_dark, angle=1.55, coverage=2.2, fill=true, clip=true})

-- Bleached bone-gray highlights along dead trunk
b_ridge:load(p_stag_bleach, 0.95)
b_ridge:stroke({{268, 355}, {266, 310}, {264, 265}, {261, 215}, {258, 165}, {253, 126}},
  {pressure={0.5, 0.75, 0.7, 0.65, 0.45, 0.1}, ramps={0.02, 0.05}})

-- Sharp splintered tips at the dead summit
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}
b_rigger:load(p_stag_tip, 1.0)
b_rigger:stroke({{253, 130}, {252, 118}}, {pressure={0.5, 0.05}})
b_rigger:stroke({{253, 130}, {256, 120}}, {pressure={0.4, 0.05}})
b_rigger:stroke({{253, 130}, {249, 122}}, {pressure={0.35, 0.05}})

-- Antler Branch S1 (East Stag Antler, reaching toward the evening star & moon!)
local pts_antler_e = {
  {267, 290}, {284, 252}, {304, 214}, {320, 175}, {332, 138},
  {335, 139}, {324, 177}, {308, 216}, {288, 254}, {270, 292}
}
work(poly(pts_antler_e, false), {hand="detail", pile=p_stag_dark, angle=0.9, coverage=2.0, fill=true, clip=true})
b_ridge:load(p_stag_bleach, 0.95)
b_ridge:stroke({{269, 290}, {286, 252}, {306, 214}, {322, 175}, {334, 137}},
  {pressure={0.5, 0.7, 0.6, 0.45, 0.1}, ramps={0.02, 0.05}})
b_rigger:load(p_stag_tip, 1.0)
b_rigger:stroke({{334, 140}, {338, 128}}, {pressure={0.4, 0.05}})
b_rigger:stroke({{334, 140}, {332, 130}}, {pressure={0.3, 0.05}})

-- Antler Branch S2 (West Stag Antler)
local pts_antler_w = {
  {263, 280}, {245, 242}, {226, 204}, {208, 166}, {194, 130},
  {197, 129}, {212, 165}, {230, 202}, {249, 240}, {266, 278}
}
work(poly(pts_antler_w, false), {hand="detail", pile=p_stag_dark, angle=-0.9, coverage=2.0, fill=true, clip=true})
b_ridge:load(p_stag_bleach, 0.95)
b_ridge:stroke({{264, 279}, {246, 241}, {227, 203}, {209, 165}, {195, 128}},
  {pressure={0.5, 0.7, 0.6, 0.45, 0.1}, ramps={0.02, 0.05}})
b_rigger:load(p_stag_tip, 1.0)
b_rigger:stroke({{195, 130}, {192, 120}}, {pressure={0.4, 0.05}})
b_rigger:stroke({{195, 130}, {197, 122}}, {pressure={0.3, 0.05}})

-- 6. Sympodial Crooked Oak Twigs (Quercus branching architecture)
local b_twig = brush{kind="round", width=1.3, point=1, stiffness=0.9}
local twigs = {
  -- Off E1 (above dolmen)
  {{400, 263}, {412, 246}, {422, 230}},
  {{400, 263}, {394, 245}, {388, 230}},
  {{382, 289}, {372, 270}, {364, 252}},
  {{362, 317}, {352, 298}, {346, 280}},
  -- Off E2 (reaching east)
  {{478, 259}, {490, 242}, {502, 226}},
  {{478, 259}, {470, 240}, {464, 222}},
  {{460, 282}, {472, 265}, {482, 248}},
  -- Off E3 (lower east)
  {{504, 350}, {520, 362}, {535, 372}},
  {{524, 356}, {542, 352}, {558, 348}},
  -- Off W1 (reaching west)
  {{144, 246}, {132, 228}, {122, 212}},
  {{144, 246}, {152, 226}, {158, 208}},
  {{160, 277}, {148, 258}, {138, 240}},
  {{177, 307}, {166, 288}, {158, 270}},
  -- Off W2 (lower west)
  {{125, 336}, {110, 350}, {96, 362}},
  {{102, 352}, {88, 346}, {74, 340}}
}
for _, tw in ipairs(twigs) do
  b_twig:load(p_bark_core, 0.9)
  b_twig:stroke(tw, {pressure={0.75, 0.25}, ramps={0.02, 0.15}})
end

-- Fine outer sympodial twigs with rigger
local fine_twigs = {
  {{422, 230}, {430, 218}, {436, 208}},
  {{388, 230}, {382, 216}, {378, 204}},
  {{502, 226}, {512, 214}, {520, 202}},
  {{482, 248}, {492, 235}, {500, 222}},
  {{122, 212}, {114, 200}, {108, 190}},
  {{138, 240}, {128, 226}, {120, 214}}
}
for _, ftw in ipairs(fine_twigs) do
  b_rigger:load(p_bark_core, 0.85)
  b_rigger:stroke(ftw, {pressure={0.65, 0.12}, ramps={0.02, 0.15}})
end

-- 7. Ragged Marcescent Late-Autumn Oak Foliage
-- Sparse, delicate sprays of clinging dried russet and golden leaves along living branches
local b_leaf = brush{kind="round", width=1.8, point=1, stiffness=0.85}
local leaf_clusters = {
  -- Along eastern living limbs
  {408, 250, 14}, {388, 275, 12}, {365, 305, 14}, {350, 290, 10},
  {470, 270, 14}, {455, 295, 12}, {495, 235, 12}, {480, 255, 10},
  {510, 355, 12}, {530, 360, 10}, {440, 335, 12}, {470, 338, 12},
  -- Along western living limbs
  {180, 318, 14}, {160, 290, 12}, {145, 260, 14}, {130, 235, 12},
  {120, 340, 12}, {100, 355, 10}, {170, 335, 12}, {135, 315, 12}
}
for _, cl in ipairs(leaf_clusters) do
  local cx, cy, n = cl[1], cl[2], cl[3]
  for i = 1, n do
    local ox = rand(-10, 10)
    local oy = rand(-8, 8)
    local p = (rand(0, 1) < 0.6) and p_leaf_russet or p_leaf_gold
    b_leaf:load(p, 0.75)
    b_leaf:touch(cx + ox, cy + oy, {pressure=rand(0.35, 0.7), drag={rand(1.2, 2.2), rand(0.8, 1.6)}})
  end
end

-- Gilt twilight rim highlights on outer leaf edges
for _, cl in ipairs(leaf_clusters) do
  local cx, cy = cl[1], cl[2]
  for i = 1, 4 do
    local ox = rand(-8, 8)
    local oy = rand(-6, 6)
    b_rigger:load(p_leaf_rim, 0.7)
    b_rigger:touch(cx + ox, cy + oy, {pressure=rand(0.25, 0.5), drag={1.0, 1.2}})
  end
end

print("Monumental Ancient Veteran Oak constructed: trunk, root flares, eastern bough sheltering tomb, western limb, bleached stag crown, twigs, and marcescent foliage.")

--@ chunk 127
-- Wipe the stiff polygon branches and restore a pure, luminous twilight firmament

-- 1. Sky Piles calibrated to exact canvas heights
local p_sky_zenith = pile{{"lead white", 18}, {"cobalt blue", 6.2}, {"smalt", 4.5}, {"raw umber", 0.6}, {"bone black", 0.25}, medium=0.22}
local p_sky_upper  = pile{{"lead white", 26}, {"cobalt blue", 3.2}, {"smalt", 3.2}, {"yellow ochre", 0.8}, {"raw umber", 0.3}, medium=0.22}
local p_sky_mid    = pile{{"lead white", 28}, {"yellow ochre", 3.5}, {"chrome yellow", 0.9}, {"smalt", 1.0}, {"raw umber", 0.2}, {"vermilion", 0.1}, medium=0.22}
local p_sky_glow   = pile{{"lead white", 27}, {"yellow ochre", 5.0}, {"chrome yellow", 2.0}, {"vermilion", 0.45}, {"raw umber", 0.25}, medium=0.22}

-- 2. Clean passage covering the stiff polygon sticks and sky patch (x = 80 to 520, y = 80 to 355)
-- We lay three vertically graded bands across this region with generous soft edges
local m_clean_top = (rect(80, 80, 440, 110) * rect(70, 70, 460, 130)):soften(20)
work(m_clean_top, {hand="body", pile=p_sky_zenith, angle=0, order="across", coverage=2.2, fill=true})

local m_clean_mid = (rect(80, 160, 440, 110) * rect(70, 140, 460, 150)):soften(20)
work(m_clean_mid, {hand="body", pile=p_sky_upper, angle=0, order="across", coverage=2.2, fill=true})

local m_clean_low = (rect(80, 240, 440, 115) * rect(70, 220, 460, 140)):soften(18)
work(m_clean_low, {hand="body", pile=p_sky_mid, angle=0, order="across", coverage=2.2, fill=true})

-- Smooth horizontal badger blending to integrate perfectly with the whole canvas sky
local m_sky_blend = rect(60, 70, 480, 290):soften(15)
blend(m_sky_blend, {angle=0})

print("Stiff polygon sticks removed; twilight firmament restored clean and seamless.")

--@ chunk 128
print("Drying at sky patch (200, 200):", drying(200, 200))
print("Drying at dolmen (400, 400):", drying(400, 400))
print("Drying at trunk (270, 400):", drying(270, 400))

--@ chunk 129
local t = wait(3 * 24 * 60)
print("Time now:", t)
print("Drying at sky (200, 200):", drying(200, 200))
print("Drying at dolmen (400, 400):", drying(400, 400))

--@ chunk 130
-- Restore the Master Friedrichian Twilight Firmament across Full Width (x = 0 to 1000)

-- 1. Sky Piles calibrated for authentic Kubelka-Munk optical transition
local p_sky_zenith = pile{{"lead white", 18}, {"cobalt blue", 6.2}, {"smalt", 4.5}, {"raw umber", 0.6}, {"bone black", 0.25}, medium=0.22}
local p_sky_upper  = pile{{"lead white", 26}, {"cobalt blue", 3.2}, {"smalt", 3.2}, {"yellow ochre", 0.8}, {"raw umber", 0.3}, medium=0.22}
local p_sky_mid    = pile{{"lead white", 28}, {"yellow ochre", 3.5}, {"chrome yellow", 0.9}, {"smalt", 1.0}, {"raw umber", 0.2}, {"vermilion", 0.1}, medium=0.22}
local p_sky_glow   = pile{{"lead white", 26}, {"yellow ochre", 5.2}, {"chrome yellow", 2.2}, {"vermilion", 0.5}, {"raw umber", 0.25}, medium=0.22}

-- 2. Define the skyline contour: down to the capstone top at dolmen (y ≈ 376), and barrow crest elsewhere
local barrow_nodes = {
  {x=-20, y=465}, {x=80, y=458}, {x=180, y=448}, {x=260, y=430},
  {x=315, y=390}, {x=350, y=376}, {x=395, y=375}, {x=440, y=377},
  {x=475, y=390}, {x=520, y=430}, {x=600, y=448}, {x=680, y=450},
  {x=760, y=436}, {x=830, y=436}, {x=910, y=448}, {x=1020, y=465}
}

local function skyline(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 430
end

local m_sky_domain = above(skyline)

-- 3. Four overlapping full-width bands covering x = 0 to 1000 from zenith to horizon
local m1 = (rect(0, 0, 1000, 140):soften(25)) * m_sky_domain
work(m1, {hand="body", pile=p_sky_zenith, angle=0, order="across", coverage=2.4, fill=true})

local m2 = (rect(0, 105, 1000, 145):soften(25)) * m_sky_domain
work(m2, {hand="body", pile=p_sky_upper, angle=0, order="across", coverage=2.4, fill=true})

local m3 = (rect(0, 210, 1000, 135):soften(25)) * m_sky_domain
work(m3, {hand="body", pile=p_sky_mid, angle=0, order="across", coverage=2.4, fill=true})

local m4 = (rect(0, 305, 1000, 145):soften(20)) * m_sky_domain
work(m4, {hand="body", pile=p_sky_glow, angle=0, order="across", coverage=2.4, fill=true})

-- 4. Seamless full-width horizontal blending to fuse bands perfectly
local m_sky_blend = rect(0, 0, 1000, 410) * m_sky_domain
blend(m_sky_blend, {angle=1.57})
blend(m_sky_blend, {angle=0})

-- 5. Distant low Baltic hills silhouetted in soft atmospheric haze (Rügen / Bodden headland)
local p_dist_coast = pile{{"lead white", 15}, {"smalt", 5.0}, {"raw umber", 3.0}, {"red earth", 0.9}, {"bone black", 0.4}, medium=0.25}
local pts_distant_land = {
  {0, 464}, {100, 460}, {220, 454}, {310, 450}, {310, 460},
  {500, 452}, {580, 450}, {700, 447}, {820, 452}, {920, 458}, {1000, 464},
  {1000, 475}, {0, 475}
}
local m_coast = (poly(pts_distant_land, true):soften(2.5)) * m_sky_domain
work(m_coast, {hand="body", pile=p_dist_coast, angle=0, coverage=1.4, fill=true, clip=true})

-- 6. Calm water sheen and valley evening mist pooling over the water
local p_water_mist = pile{{"lead white", 26}, {"yellow ochre", 3.2}, {"smalt", 1.8}, {"raw umber", 0.3}, medium=0.3}
local m_water = (rect(0, 434, 1000, 30):soften(4)) * m_sky_domain
work(m_water, {hand="glaze", pile=p_water_mist, angle=0, coverage=1.2, fill=false})
blend(rect(0, 430, 1000, 36) * m_sky_domain, {angle=0})

-- 7. The Waxing Crescent Moon and Solitary Evening Star (Venus / Hesperus)
local p_moon_pure = pile{{"lead white", 34}, {"yellow ochre", 1.0}, {"chrome yellow", 0.4}, medium=0.12}
local b_rigger = brush{kind="round", width=1.1, point=1, stiffness=0.9}
b_rigger:load(p_moon_pure, 0.95)
b_rigger:stroke({
  {645.2, 105.5}, {641.5, 108.0}, {639.0, 112.5}, {640.2, 117.0}, {644.0, 119.5}
}, {pressure={0.08, 0.85, 1.0, 0.8, 0.08}, ramps={0.05, 0.05}})

local b_star = brush{kind="round", width=0.9, point=1, stiffness=0.9}
b_star:load(p_moon_pure, 1.0)
b_star:touch(688, 148, {pressure=0.8})
b_star:stroke({{686.8, 148}, {689.2, 148}}, {pressure={0.2, 0.5, 0.2}})
b_star:stroke({{688, 146.8}, {688, 149.2}}, {pressure={0.2, 0.5, 0.2}})

print("Full-width Friedrich twilight firmament restored; moon, star, and distant Baltic coast established.")

--@ chunk 131
-- Sculpt the Prehistoric Barrow Tumulus, Heathland, Pilgrim Path, Erratics, and Bed the Tomb

-- 1. Piles for Heathland, Peat, and Glacial Granite
local p_peat_dark    = pile{{"raw umber", 11}, {"bone black", 8}, {"red earth", 3.5}, medium=0.15}
local p_peat_body    = pile{{"raw umber", 9}, {"bone black", 5}, {"red earth", 3.5}, {"yellow ochre", 3.0}, medium=0.18}
local p_heather_bank = pile{{"raw umber", 7}, {"red earth", 5.5}, {"bone black", 3.5}, {"smalt", 2.0}, {"yellow ochre", 2.5}, {"lead white", 1.5}, medium=0.2}
local p_turf_ochre   = pile{{"yellow ochre", 8}, {"raw umber", 5.5}, {"red earth", 2.5}, {"lead white", 3.0}, {"bone black", 1.0}, medium=0.2}
local p_moss_deep    = pile{{"green earth", 7}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}
local p_sand_path    = pile{{"lead white", 14}, {"yellow ochre", 8}, {"raw umber", 3.5}, {"red earth", 1.5}, medium=0.22}

local p_granite_deep = pile{{"bone black", 10}, {"raw umber", 8}, {"red earth", 2.0}, {"smalt", 2.0}, medium=0.15}
local p_granite_body = pile{{"lead white", 12}, {"raw umber", 7.0}, {"yellow ochre", 4.0}, {"bone black", 3.0}, {"red earth", 2.0}, {"smalt", 1.2}, medium=0.18}
local p_granite_facet= pile{{"lead white", 20}, {"yellow ochre", 4.5}, {"raw umber", 2.5}, {"red earth", 1.5}, {"smalt", 1.2}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 26}, {"yellow ochre", 4.0}, {"chrome yellow", 0.6}, {"red earth", 1.0}, {"raw umber", 0.8}, medium=0.2}
local p_granite_rim  = pile{{"lead white", 30}, {"yellow ochre", 2.5}, {"smalt", 1.2}, {"raw umber", 0.5}, medium=0.2}

local p_lich_orange  = pile{{"lead white", 8}, {"yellow ochre", 11}, {"chrome yellow", 3.5}, {"red earth", 2.5}, medium=0.25}
local p_lich_sage    = pile{{"lead white", 15}, {"green earth", 8.0}, {"yellow ochre", 3.5}, {"raw umber", 1.5}, medium=0.25}

-- 2. Define the Barrow Crest and Sculpt Full Tumulus Body
local barrow_nodes = {
  {x=-20, y=465}, {x=80, y=458}, {x=180, y=448}, {x=260, y=430},
  {x=330, y=415}, {x=385, y=408}, {x=445, y=414}, {x=520, y=430},
  {x=600, y=448}, {x=680, y=450}, {x=760, y=436}, {x=830, y=436},
  {x=910, y=448}, {x=1020, y=465}
}

local function barrow_crest(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 430
end

local m_barrow = below(barrow_crest)

-- Base body of peaty loam covering the entire lower canvas (eliminates pale horizon gap!)
work(m_barrow, {hand="body", pile=p_peat_body, angle=0.2, coverage=2.4, fill=true, clip=true})

-- Firm, crisp peaty contour along the ridge crest against the glowing evening mist
local b_crest = brush{kind="round", width=2.4, point=1, stiffness=0.9}
b_crest:load(p_peat_dark, 0.95)
local crest_pts = {}
for x = -10, 1010, 8 do
  table.insert(crest_pts, {x, barrow_crest(x)})
end
b_crest:stroke(crest_pts, {pressure=0.95, ramps={0.01, 0.01}})

-- 3. Rolling Banks of Late-Autumn Heather (Calluna vulgaris) & Peat Hollows
local b_broad = brush{kind="filbert", width=14.0, stiffness=0.85}

-- Heather banks on left slope
b_broad:load(p_heather_bank, 0.95)
b_broad:stroke({{60, 480}, {140, 495}, {240, 505}, {340, 510}}, {pressure=0.9})
b_broad:stroke({{80, 530}, {180, 545}, {290, 555}, {400, 555}}, {pressure=0.9})
b_broad:stroke({{100, 590}, {220, 605}, {360, 615}, {480, 610}}, {pressure=0.9})

-- Heather banks on right slope
b_broad:stroke({{600, 495}, {720, 480}, {840, 475}, {950, 480}}, {pressure=0.9})
b_broad:stroke({{550, 545}, {680, 535}, {810, 530}, {940, 535}}, {pressure=0.9})
b_broad:stroke({{520, 610}, {660, 605}, {800, 600}, {930, 605}}, {pressure=0.9})

-- Mossy hollows and sunken peaty folds
b_broad:load(p_moss_deep, 0.9)
b_broad:stroke({{180, 515}, {280, 525}, {380, 528}}, {pressure=0.8})
b_broad:stroke({{540, 520}, {640, 512}, {740, 508}}, {pressure=0.8})
b_broad:stroke({{250, 640}, {400, 650}, {550, 648}}, {pressure=0.85})

-- Warm golden turf ridges
b_broad:load(p_turf_ochre, 0.85)
b_broad:stroke({{120, 465}, {200, 475}, {280, 480}}, {pressure=0.75})
b_broad:stroke({{650, 465}, {760, 455}, {870, 458}}, {pressure=0.75})
b_broad:stroke({{160, 565}, {280, 575}, {410, 578}}, {pressure=0.75})
b_broad:stroke({{580, 570}, {720, 565}, {860, 560}}, {pressure=0.75})

-- 4. Trodden Pilgrim / Sheep Track through the Heather
-- A subtle, winding sandy path from lower right up toward the ancient megalith
local pts_path = {
  {720, 704}, {680, 650}, {635, 595}, {585, 545}, {535, 500}, {485, 465}, {445, 442}
}
local m_path = ribbon(pts_path, {24, 20, 16, 12, 9, 6, 4}):soften(3)
work(m_path, {hand="scumble", pile=p_sand_path, angle=0.4, coverage=1.4, fill=true})

-- 5. Foreground Glacial Erratic Boulders (*Findlinge*)
-- A. Mid-Foreground Boulder Left (x ≈ 215 to 275, y ≈ 525 to 555)
local pts_b1 = {{215, 545}, {226, 528}, {258, 524}, {274, 534}, {272, 552}, {245, 556}, {218, 554}}
local m_b1 = poly(pts_b1, false)
work(m_b1, {hand="body", pile=p_granite_body, angle=0.2, coverage=2.0, fill=true, clip=true})
local pts_b1_lit = {{226, 528}, {258, 524}, {274, 534}, {260, 536}, {232, 536}}
work(poly(pts_b1_lit, false), {hand="detail", pile=p_granite_facet, angle=0.05, coverage=1.6, fill=true, clip=true})
local b_crack = brush{kind="round", width=1.0, point=1, stiffness=0.9}
b_crack:load(p_granite_deep, 0.9)
b_crack:stroke({{242, 526}, {246, 538}, {244, 554}}, {pressure={0.5, 0.7, 0.2}})
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
b_facet:load(p_lich_orange, 0.8)
b_facet:touch(238, 530, {pressure=0.6, drag={2.0, 0.1}})
b_facet:load(p_lich_sage, 0.8)
b_facet:touch(258, 530, {pressure=0.55, drag={1.8, 0.1}})

-- B. Mid-Foreground Boulder Cluster Right (x ≈ 650 to 715, y ≈ 535 to 565)
local pts_b2 = {{654, 554}, {666, 536}, {698, 532}, {714, 544}, {712, 562}, {680, 566}, {658, 562}}
local m_b2 = poly(pts_b2, false)
work(m_b2, {hand="body", pile=p_granite_body, angle=-0.2, coverage=2.0, fill=true, clip=true})
local pts_b2_lit = {{666, 536}, {698, 532}, {714, 544}, {696, 546}, {674, 544}}
work(poly(pts_b2_lit, false), {hand="detail", pile=p_granite_lit, angle=-0.05, coverage=1.6, fill=true, clip=true})
b_crack:load(p_granite_deep, 0.9)
b_crack:stroke({{682, 534}, {684, 546}, {682, 562}}, {pressure={0.5, 0.7, 0.2}})
b_facet:load(p_lich_sage, 0.8)
b_facet:touch(678, 538, {pressure=0.6, drag={2.0, 0.1}})
b_facet:load(p_lich_orange, 0.8)
b_facet:touch(694, 536, {pressure=0.55, drag={1.8, 0.1}})

-- Bed foreground erratics into heather
local b_bole = brush{kind="filbert", width=6.0, stiffness=0.85}
b_bole:load(p_heather_bank, 0.95)
b_bole:stroke({{202, 556}, {235, 562}, {268, 560}, {295, 552}}, {pressure=0.9})
b_bole:stroke({{642, 564}, {678, 570}, {712, 568}, {738, 558}}, {pressure=0.9})

-- 6. Model and Bed the Megalithic Tomb (Hünengrab)
-- Left Orthostat: massive, crystalline Scandinavian boulder
local pts_lo = {{326, 396}, {370, 396}, {372, 434}, {328, 434}}
work(poly(pts_lo, false), {hand="body", pile=p_granite_body, angle=1.4, coverage=2.2, fill=true, clip=true})
local pts_lo_f = {{326, 396}, {344, 396}, {348, 434}, {328, 434}}
work(poly(pts_lo_f, false), {hand="detail", pile=p_granite_facet, angle=1.35, coverage=1.8, fill=true, clip=true})

-- Right Orthostat: massive, crystalline boulder
local pts_ro = {{414, 396}, {462, 396}, {464, 434}, {418, 434}}
work(poly(pts_ro, false), {hand="body", pile=p_granite_body, angle=1.4, coverage=2.2, fill=true, clip=true})
local pts_ro_f = {{438, 396}, {462, 396}, {464, 434}, {442, 434}}
work(poly(pts_ro_f, false), {hand="detail", pile=p_granite_lit, angle=1.4, coverage=1.8, fill=true, clip=true})

-- Deep Burial Chamber Void between orthostats (mysterious sacred darkness)
local pts_void = {{370, 396}, {414, 396}, {416, 434}, {372, 434}}
work(poly(pts_void, false), {hand="detail", pile=p_peat_dark, angle=1.3, coverage=2.2, fill=true, clip=true})

-- Center-rear orthostat silhouette inside chamber
local pts_co = {{384, 398}, {404, 398}, {406, 432}, {386, 432}}
work(poly(pts_co, false), {hand="detail", pile=p_granite_deep, angle=1.4, coverage=1.8, fill=true, clip=true})

-- The Massive Capstone: Rugged Scandinavian Erratic Boulder
local pts_cap = {
  {318, 398}, {324, 390}, {340, 381}, {365, 375}, {395, 374}, {430, 376},
  {455, 382}, {468, 390}, {466, 398}, {440, 399}, {400, 398}, {360, 399},
  {330, 400}, {318, 398}
}
work(poly(pts_cap, false), {hand="body", pile=p_granite_body, angle=0.05, coverage=2.4, fill=true, clip=true})

-- Western skyward-facing facet
local pts_f_west = {
  {318, 398}, {324, 390}, {340, 381}, {365, 375}, {395, 374},
  {396, 388}, {360, 388}, {328, 392}, {318, 398}
}
work(poly(pts_f_west, false), {hand="detail", pile=p_granite_facet, angle=0.06, coverage=1.8, fill=true, clip=true})

-- Eastern skyward-facing facet (warm twilight reflection)
local pts_f_east = {
  {395, 374}, {430, 376}, {455, 382}, {468, 390}, {466, 394},
  {445, 389}, {400, 389}, {396, 388}, {395, 374}
}
work(poly(pts_f_east, false), {hand="detail", pile=p_granite_lit, angle=-0.04, coverage=1.8, fill=true, clip=true})

-- Underside rock shadow contact points
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{320, 398}, {345, 397}, {370, 398}}, {pressure={0.4, 0.8, 0.5}})
b_crack:stroke({{370, 398}, {395, 398}, {414, 398}}, {pressure=0.95})
b_crack:stroke({{414, 398}, {440, 397}, {464, 397}}, {pressure={0.5, 0.8, 0.4}})

-- Geological fracture across capstone at x ≈ 395
b_crack:stroke({{395, 374}, {397, 387}, {396, 398}}, {pressure={0.7, 0.85, 0.35}})

-- Top crystalline rim catching twilight
b_crack:load(p_granite_rim, 0.9)
b_crack:stroke({{320, 394}, {330, 386}, {348, 379}, {372, 375}, {395, 374}}, {pressure={0.3, 0.75, 0.7, 0.85, 0.4}})
b_crack:stroke({{395, 374}, {420, 375}, {445, 379}, {462, 386}, {467, 392}}, {pressure={0.4, 0.85, 0.75, 0.65, 0.25}})

-- Lichens on capstone
local lichens = {
  {338, 384, p_lich_orange, 2.2, 0.2},
  {362, 378, p_lich_sage,   2.4, 0.1},
  {382, 376, p_lich_orange, 2.0, 0.1},
  {418, 377, p_lich_sage,   2.2, -0.1},
  {442, 381, p_lich_orange, 2.0, -0.1},
  {460, 388, p_lich_sage,   2.0, -0.2},
  {332, 412, p_lich_orange, 1.8, 0.1},
  {456, 412, p_lich_sage,   1.8, -0.1}
}
for _, l in ipairs(lichens) do
  b_facet:load(l[3], 0.8)
  b_facet:touch(l[1], l[2], {pressure=0.55, drag={l[4], l[5]}})
end

-- Bed all stones deeply into turf and heather
b_bole:load(p_peat_dark, 0.95)
b_bole:stroke({{285, 432}, {320, 434}, {355, 435}, {395, 436}, {435, 435}, {470, 434}, {505, 432}}, {pressure=0.95})
b_bole:load(p_heather_bank, 0.9)
b_bole:stroke({{290, 435}, {330, 437}, {380, 438}, {430, 437}, {480, 436}, {510, 434}}, {pressure=0.9})

print("Barrow tumulus, heathland, pilgrim path, erratics, and dolmen sculpted and bedded.")

--@ chunk 132
-- Transform Dolmen into Rugged Scandinavian Granite & Naturalize the Heathland and Path

-- 1. Piles
local p_granite_core = pile{{"raw umber", 8}, {"bone black", 5}, {"red earth", 3.0}, {"yellow ochre", 3.0}, {"lead white", 4.0}, {"smalt", 1.5}, medium=0.18}
local p_granite_sh   = pile{{"raw umber", 9}, {"bone black", 7}, {"red earth", 2.5}, {"smalt", 1.5}, medium=0.15}
local p_granite_facet= pile{{"lead white", 12}, {"yellow ochre", 5.0}, {"raw umber", 4.0}, {"red earth", 2.0}, {"smalt", 1.2}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 18}, {"yellow ochre", 4.5}, {"chrome yellow", 0.8}, {"red earth", 1.2}, {"raw umber", 1.0}, medium=0.2}
local p_granite_rim  = pile{{"lead white", 24}, {"yellow ochre", 3.0}, {"chrome yellow", 0.6}, {"smalt", 1.2}, medium=0.2}

local p_lich_orange  = pile{{"yellow ochre", 10}, {"chrome yellow", 3.5}, {"red earth", 2.5}, {"lead white", 6.0}, medium=0.25}
local p_lich_sage    = pile{{"lead white", 12}, {"green earth", 8.0}, {"yellow ochre", 4.0}, {"raw umber", 2.0}, medium=0.25}

local p_peat_deep    = pile{{"raw umber", 11}, {"bone black", 8}, {"red earth", 3.5}, medium=0.15}
local p_heather_purp = pile{{"raw umber", 7}, {"red earth", 6.0}, {"bone black", 3.5}, {"smalt", 2.0}, {"yellow ochre", 2.0}, medium=0.18}
local p_turf_warm    = pile{{"yellow ochre", 8}, {"raw umber", 5.0}, {"red earth", 2.5}, {"bone black", 1.2}, {"lead white", 2.0}, medium=0.2}
local p_sand_broken  = pile{{"yellow ochre", 7}, {"raw umber", 5.0}, {"lead white", 6.0}, {"red earth", 1.5}, medium=0.2}

local b_block = brush{kind="filbert", width=7.0, stiffness=0.85}
local b_broad = brush{kind="filbert", width=14.0, stiffness=0.85}
local b_facet = brush{kind="round", width=2.6, point=1, stiffness=0.85}
local b_crack = brush{kind="round", width=1.1, point=1, stiffness=0.9}

-- 2. Resurface the Megalithic Tomb (Scandinavian Granite — warm, weathered, crystalline)
-- A. Capstone Body & Facets: Break the white dome into rugged, angular rock planes
b_block:load(p_granite_core, 0.95)
-- Scumble across the capstone to tone down the chalky white
b_block:stroke({{318, 396}, {355, 396}, {395, 397}, {435, 396}, {466, 394}}, {pressure=0.95})
b_block:stroke({{325, 388}, {360, 386}, {400, 387}, {440, 386}, {464, 388}}, {pressure=0.95})
b_block:stroke({{335, 380}, {368, 378}, {405, 377}, {438, 378}, {458, 382}}, {pressure=0.9})

-- Western sloping facet (angular, weathered, reddish-gray granite)
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{322, 393}, {342, 386}, {366, 382}, {386, 380}}, {pressure={0.65, 0.85, 0.8, 0.5}})
b_facet:stroke({{328, 396}, {352, 391}, {376, 387}, {390, 385}}, {pressure={0.5, 0.75, 0.7, 0.4}})

-- Eastern stepped facet (catching warm evening light)
b_facet:load(p_granite_lit, 0.85)
b_facet:stroke({{388, 380}, {415, 381}, {442, 384}, {464, 390}}, {pressure={0.55, 0.85, 0.8, 0.45}})
b_facet:stroke({{392, 386}, {420, 387}, {446, 389}, {462, 394}}, {pressure={0.45, 0.75, 0.7, 0.35}})

-- Sharp cleavage fractures and weathered joint cracks
b_crack:load(p_granite_sh, 0.95)
b_crack:stroke({{386, 374}, {389, 387}, {387, 399}}, {pressure={0.7, 0.85, 0.35}})
b_crack:stroke({{346, 379}, {350, 389}, {348, 399}}, {pressure={0.5, 0.65, 0.2}})
b_crack:stroke({{440, 379}, {442, 389}, {443, 399}}, {pressure={0.5, 0.65, 0.2}})

-- Top crystalline skyline rim
b_crack:load(p_granite_rim, 0.9)
b_crack:stroke({{318, 396}, {328, 388}, {344, 382}, {368, 378}, {388, 376}}, {pressure={0.3, 0.75, 0.7, 0.85, 0.4}})
b_crack:stroke({{388, 376}, {415, 377}, {442, 380}, {460, 386}, {466, 392}}, {pressure={0.4, 0.85, 0.75, 0.65, 0.25}})

-- B. Left and Right Orthostats: Tone down chalky white, model rugged boulder volume
-- Left Orthostat (x ≈ 326 to 365, y ≈ 396 to 434)
b_block:load(p_granite_core, 0.95)
b_block:stroke({{330, 398}, {334, 415}, {338, 432}}, {pressure=0.95})
b_block:stroke({{342, 397}, {345, 415}, {348, 432}}, {pressure=0.95})
b_block:stroke({{354, 398}, {356, 415}, {358, 430}}, {pressure=0.95})
b_facet:load(p_granite_facet, 0.85)
b_facet:stroke({{328, 398}, {332, 412}, {336, 424}, {340, 432}}, {pressure={0.65, 0.8, 0.6, 0.2}})

-- Right Orthostat (x ≈ 420 to 462, y ≈ 396 to 434)
b_block:load(p_granite_core, 0.95)
b_block:stroke({{424, 398}, {426, 415}, {428, 432}}, {pressure=0.95})
b_block:stroke({{436, 397}, {438, 415}, {440, 432}}, {pressure=0.95})
b_block:stroke({{448, 398}, {450, 415}, {452, 432}}, {pressure=0.95})
b_facet:load(p_granite_lit, 0.85)
b_facet:stroke({{442, 398}, {446, 412}, {448, 424}, {450, 432}}, {pressure={0.65, 0.8, 0.6, 0.2}})

-- C. The Sacred Chamber Void (Break up the square doorway!)
-- Left inner boulder flank projecting into chamber
b_facet:load(p_granite_sh, 0.9)
b_facet:stroke({{362, 398}, {365, 415}, {368, 430}}, {pressure=0.85})
-- Right inner boulder flank
b_facet:stroke({{416, 398}, {414, 415}, {412, 430}}, {pressure=0.85})
-- Deep velvety void within
b_crack:load(p_peat_deep, 0.95)
b_crack:stroke({{374, 402}, {395, 403}, {408, 402}}, {pressure=0.95})
b_crack:stroke({{376, 414}, {395, 416}, {406, 415}}, {pressure=0.95})
b_crack:stroke({{378, 424}, {395, 425}, {404, 424}}, {pressure=0.95})

-- Underside rock shadow contact points
b_crack:load(p_granite_sh, 0.95)
b_crack:stroke({{322, 398}, {345, 397}, {368, 398}}, {pressure={0.5, 0.85, 0.5}})
b_crack:stroke({{368, 398}, {395, 398}, {416, 398}}, {pressure=0.95})
b_crack:stroke({{416, 398}, {440, 397}, {462, 397}}, {pressure={0.5, 0.85, 0.4}})

-- D. Crustose Lichen Colonies Encrusting the Megalith
local lichens = {
  {336, 384, p_lich_orange, 2.2, 0.2},
  {360, 378, p_lich_sage,   2.4, 0.1},
  {382, 376, p_lich_orange, 2.0, 0.1},
  {416, 377, p_lich_sage,   2.2, -0.1},
  {440, 381, p_lich_orange, 2.0, -0.1},
  {458, 388, p_lich_sage,   2.0, -0.2},
  {332, 412, p_lich_orange, 1.8, 0.1},
  {450, 412, p_lich_sage,   1.8, -0.1}
}
for _, l in ipairs(lichens) do
  b_facet:load(l[3], 0.8)
  b_facet:touch(l[1], l[2], {pressure=0.55, drag={l[4], l[5]}})
end

-- 3. Weave and Naturalize the Heathland & Break the Straight Path
-- Broad sweeps of dark heather and peat crossing over the path, making it natural and subtle
b_broad:load(p_heather_purp, 0.95)
-- Cross the path at several points to break the continuous stripe
b_broad:stroke({{520, 480}, {570, 495}, {630, 505}, {700, 510}}, {pressure=0.9})
b_broad:stroke({{460, 535}, {530, 545}, {610, 555}, {700, 550}}, {pressure=0.9})
b_broad:stroke({{540, 600}, {620, 615}, {710, 620}, {800, 610}}, {pressure=0.9})
b_broad:stroke({{600, 660}, {670, 670}, {750, 672}, {820, 665}}, {pressure=0.9})

-- Rolling heather banks across the lower canvas
b_broad:stroke({{80, 520}, {190, 535}, {310, 545}, {440, 545}}, {pressure=0.9})
b_broad:stroke({{60, 585}, {180, 600}, {320, 610}, {460, 605}}, {pressure=0.9})
b_broad:stroke({{100, 650}, {250, 660}, {420, 665}, {580, 660}}, {pressure=0.9})

-- Peaty hollows and turf folds
b_broad:load(p_peat_deep, 0.9)
b_broad:stroke({{160, 505}, {260, 515}, {370, 520}}, {pressure=0.85})
b_broad:stroke({{500, 520}, {600, 528}, {720, 520}}, {pressure=0.85})
b_broad:stroke({{220, 630}, {380, 642}, {540, 638}}, {pressure=0.85})

-- Warm golden-ochre dead bent-grass tufts across the slopes
b_broad:load(p_turf_warm, 0.85)
b_broad:stroke({{130, 470}, {220, 480}, {310, 485}}, {pressure=0.75})
b_broad:stroke({{660, 465}, {770, 455}, {880, 458}}, {pressure=0.75})
b_broad:stroke({{170, 560}, {290, 570}, {420, 572}}, {pressure=0.75})
b_broad:stroke({{570, 565}, {700, 560}, {830, 555}}, {pressure=0.75})

-- Re-state the path as a subtle, worn sheep-run (narrow, broken, meandering)
local b_path = brush{kind="round", width=2.8, point=1, stiffness=0.85}
b_path:load(p_sand_broken, 0.8)
b_path:stroke({{705, 704}, {675, 665}, {642, 620}}, {pressure={0.5, 0.35}})
b_path:stroke({{620, 595}, {585, 555}, {552, 520}}, {pressure={0.4, 0.3}})
b_path:stroke({{530, 495}, {495, 468}, {460, 446}}, {pressure={0.35, 0.2}})

-- 4. Naturalize Foreground Erratics (Bed them deeply into heather)
-- Left Boulder (x ≈ 235, y ≈ 540)
b_block:load(p_granite_core, 0.9)
b_block:stroke({{224, 536}, {246, 530}, {266, 538}, {258, 550}, {230, 550}}, {pressure=0.85})
b_facet:load(p_granite_facet, 0.8)
b_facet:stroke({{232, 532}, {252, 530}, {264, 536}}, {pressure={0.5, 0.7, 0.3}})
b_crack:load(p_granite_sh, 0.9)
b_crack:stroke({{246, 530}, {248, 542}, {246, 552}}, {pressure=0.6})
b_facet:load(p_lich_orange, 0.75)
b_facet:touch(242, 534, {pressure=0.5, drag={1.5, 0.1}})

-- Right Boulder (x ≈ 680, y ≈ 550)
b_block:load(p_granite_core, 0.9)
b_block:stroke({{668, 544}, {690, 538}, {710, 546}, {702, 558}, {674, 558}}, {pressure=0.85})
b_facet:load(p_granite_lit, 0.8)
b_facet:stroke({{676, 540}, {696, 538}, {708, 544}}, {pressure={0.5, 0.7, 0.3}})
b_crack:load(p_granite_sh, 0.9)
b_crack:stroke({{690, 538}, {692, 550}, {690, 560}}, {pressure=0.6})
b_facet:load(p_lich_sage, 0.75)
b_facet:touch(686, 542, {pressure=0.5, drag={1.5, 0.1}})

-- Bed both boulders with dark heather wraps
b_block:load(p_heather_purp, 0.95)
b_block:stroke({{210, 552}, {240, 556}, {275, 554}}, {pressure=0.9})
b_block:stroke({{655, 560}, {690, 564}, {722, 562}}, {pressure=0.9})

-- Bed dolmen base with dark peat and heather
b_block:load(p_peat_deep, 0.95)
b_block:stroke({{285, 432}, {320, 434}, {355, 435}, {395, 436}, {435, 435}, {470, 434}, {505, 432}}, {pressure=0.95})
b_block:load(p_heather_purp, 0.9)
b_block:stroke({{290, 435}, {330, 437}, {380, 438}, {430, 437}, {480, 436}, {510, 434}}, {pressure=0.9})

print("Dolmen remodeled into weathered crystalline Scandinavian granite; heathland and path naturalized.")

--@ chunk 133
-- Transform Dolmen into Rugged Scandinavian Glacial Erratics & Cover Driveway Path with Wild Heath

-- 1. Piles
local p_granite_deep = pile{{"bone black", 10}, {"raw umber", 8}, {"red earth", 2.5}, {"smalt", 2.0}, medium=0.15}
local p_granite_core = pile{{"raw umber", 8}, {"bone black", 5}, {"red earth", 3.0}, {"yellow ochre", 3.0}, {"lead white", 4.0}, {"smalt", 1.5}, medium=0.18}
local p_granite_facet= pile{{"lead white", 12}, {"yellow ochre", 5.0}, {"raw umber", 4.0}, {"red earth", 2.0}, {"smalt", 1.2}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 18}, {"yellow ochre", 4.5}, {"chrome yellow", 0.8}, {"red earth", 1.2}, {"raw umber", 1.0}, medium=0.2}
local p_granite_rim  = pile{{"lead white", 24}, {"yellow ochre", 3.0}, {"chrome yellow", 0.6}, {"smalt", 1.2}, medium=0.2}

local p_lich_orange  = pile{{"yellow ochre", 10}, {"chrome yellow", 3.5}, {"red earth", 2.5}, {"lead white", 6.0}, medium=0.25}
local p_lich_sage    = pile{{"lead white", 12}, {"green earth", 8.0}, {"yellow ochre", 4.0}, {"raw umber", 2.0}, medium=0.25}

local p_peat_deep    = pile{{"raw umber", 11}, {"bone black", 8}, {"red earth", 3.5}, medium=0.15}
local p_heather_purp = pile{{"raw umber", 7}, {"red earth", 6.0}, {"bone black", 3.5}, {"smalt", 2.0}, {"yellow ochre", 2.0}, medium=0.18}
local p_turf_warm    = pile{{"yellow ochre", 8}, {"raw umber", 5.0}, {"red earth", 2.5}, {"bone black", 1.2}, {"lead white", 2.0}, medium=0.2}
local p_moss_deep    = pile{{"green earth", 7}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}

local b_broad = brush{kind="filbert", width=14.0, stiffness=0.85}
local b_block = brush{kind="filbert", width=7.0, stiffness=0.85}
local b_facet = brush{kind="round", width=2.6, point=1, stiffness=0.85}
local b_crack = brush{kind="round", width=1.1, point=1, stiffness=0.9}

-- 2. Permanently Eliminate the "Driveway" Path & Unify the Wild Heathland
-- Lay broad, curving sweeps of dark heather, peat, and turf across the entire path (x = 420 to 740, y = 440 to 704)
b_broad:load(p_heather_purp, 0.95)
b_broad:stroke({{430, 450}, {480, 465}, {540, 480}, {620, 495}, {720, 500}}, {pressure=0.95})
b_broad:stroke({{440, 490}, {500, 510}, {570, 530}, {650, 545}, {740, 550}}, {pressure=0.95})
b_broad:stroke({{480, 545}, {550, 570}, {630, 590}, {710, 605}, {800, 610}}, {pressure=0.95})
b_broad:stroke({{520, 610}, {600, 635}, {680, 655}, {760, 670}, {840, 675}}, {pressure=0.95})
b_broad:stroke({{560, 665}, {640, 685}, {720, 700}, {800, 704}}, {pressure=0.95})

-- Dark peaty ruts and hollows
b_broad:load(p_peat_deep, 0.9)
b_broad:stroke({{460, 470}, {520, 490}, {600, 505}, {680, 510}}, {pressure=0.9})
b_broad:stroke({{500, 530}, {570, 555}, {650, 570}, {730, 575}}, {pressure=0.9})
b_broad:stroke({{550, 595}, {630, 620}, {720, 635}, {800, 638}}, {pressure=0.9})

-- Warm autumn bent-grass drifts
b_broad:load(p_turf_warm, 0.85)
b_broad:stroke({{450, 460}, {530, 475}, {620, 485}, {710, 488}}, {pressure=0.8})
b_broad:stroke({{490, 515}, {580, 535}, {680, 545}, {780, 548}}, {pressure=0.8})
b_broad:stroke({{540, 575}, {640, 595}, {740, 610}, {840, 612}}, {pressure=0.8})

-- Submerge the two "sugar cube" boulders into deep heather and peat
b_broad:load(p_heather_purp, 0.95)
b_broad:stroke({{210, 540}, {240, 548}, {270, 544}}, {pressure=0.95})
b_broad:stroke({{655, 548}, {690, 555}, {725, 552}}, {pressure=0.95})

-- 3. Transform the Dolmen: ELIMINATE the Square "Garage Door" & Model Organic Glacial Erratics
-- A. Place a massive, rounded Scandinavian glacial erratic boulder in the center (x ≈ 362 to 415, y ≈ 400 to 435)
-- This permanently closes the square opening!
b_block:load(p_granite_core, 0.95)
b_block:stroke({{370, 404}, {373, 418}, {376, 432}}, {pressure=0.95})
b_block:stroke({{385, 402}, {388, 418}, {390, 434}}, {pressure=0.95})
b_block:stroke({{398, 402}, {400, 418}, {402, 434}}, {pressure=0.95})
b_block:stroke({{410, 404}, {412, 418}, {412, 432}}, {pressure=0.95})

-- Center boulder rounded facet catching twilight
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{376, 404}, {388, 403}, {402, 404}, {408, 412}}, {pressure={0.6, 0.8, 0.75, 0.3}})
b_facet:stroke({{380, 412}, {392, 414}, {404, 416}}, {pressure=0.7})

-- Narrow, deep, mysterious shadow fissures between the boulders (NOT a doorway!)
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{366, 402}, {368, 418}, {369, 432}}, {pressure={0.7, 0.85, 0.4}})
b_crack:stroke({{415, 402}, {414, 418}, {413, 432}}, {pressure={0.7, 0.85, 0.4}})

-- B. Left Orthostat: Natural, rounded, leaning glacial boulder
b_block:load(p_granite_core, 0.95)
b_block:stroke({{328, 402}, {332, 418}, {336, 434}}, {pressure=0.95})
b_block:stroke({{340, 400}, {344, 418}, {348, 434}}, {pressure=0.95})
b_block:stroke({{352, 402}, {355, 418}, {358, 432}}, {pressure=0.95})

b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{326, 402}, {330, 416}, {334, 428}, {338, 434}}, {pressure={0.7, 0.85, 0.65, 0.2}})

-- C. Right Orthostat: Heavy, weathered, crystalline boulder
b_block:load(p_granite_core, 0.95)
b_block:stroke({{422, 402}, {424, 418}, {426, 434}}, {pressure=0.95})
b_block:stroke({{434, 400}, {436, 418}, {438, 434}}, {pressure=0.95})
b_block:stroke({{446, 402}, {448, 418}, {450, 434}}, {pressure=0.95})

b_facet:load(p_granite_lit, 0.9)
b_facet:stroke({{440, 402}, {444, 416}, {446, 428}, {448, 434}}, {pressure={0.7, 0.85, 0.65, 0.2}})

-- D. Capstone: Heavy Scandinavian erratic block resting solidly on the boulders
-- Erase the two artificial vertical lines on the capstone with rich granite body
b_block:load(p_granite_core, 0.95)
b_block:stroke({{320, 396}, {355, 396}, {395, 396}, {435, 396}, {465, 395}}, {pressure=0.95})
b_block:stroke({{326, 388}, {360, 388}, {400, 388}, {440, 388}, {462, 388}}, {pressure=0.95})
b_block:stroke({{336, 382}, {370, 380}, {405, 379}, {438, 380}, {456, 382}}, {pressure=0.9})

-- Western natural granite slope
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{322, 394}, {342, 387}, {368, 383}, {388, 381}}, {pressure={0.65, 0.85, 0.8, 0.5}})
b_facet:stroke({{328, 396}, {352, 392}, {376, 388}, {392, 386}}, {pressure={0.5, 0.75, 0.7, 0.4}})

-- Eastern stepped facet catching evening light
b_facet:load(p_granite_lit, 0.9)
b_facet:stroke({{388, 381}, {415, 382}, {442, 385}, {462, 391}}, {pressure={0.55, 0.85, 0.8, 0.45}})
b_facet:stroke({{392, 387}, {420, 388}, {446, 390}, {460, 395}}, {pressure={0.45, 0.75, 0.7, 0.35}})

-- Heavy rock undercut shadow immediately beneath capstone overhang
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{322, 398}, {348, 398}, {372, 399}, {400, 399}, {430, 398}, {460, 398}}, {pressure=0.95})

-- Natural geological fracture across capstone at x ≈ 388
b_crack:stroke({{388, 377}, {390, 388}, {389, 399}}, {pressure={0.65, 0.8, 0.35}})

-- Top crystalline chipped rock rim
b_crack:load(p_granite_rim, 0.9)
b_crack:stroke({{320, 396}, {330, 388}, {346, 382}, {368, 379}, {388, 377}}, {pressure={0.3, 0.75, 0.7, 0.85, 0.4}})
b_crack:stroke({{388, 377}, {415, 378}, {440, 381}, {458, 387}, {464, 393}}, {pressure={0.4, 0.85, 0.75, 0.65, 0.25}})

-- Lichens nestled in fractures
local lichens = {
  {338, 385, p_lich_orange, 2.2, 0.2},
  {362, 380, p_lich_sage,   2.4, 0.1},
  {382, 378, p_lich_orange, 2.0, 0.1},
  {416, 379, p_lich_sage,   2.2, -0.1},
  {440, 382, p_lich_orange, 2.0, -0.1},
  {456, 389, p_lich_sage,   2.0, -0.2},
  {334, 414, p_lich_orange, 1.8, 0.1},
  {446, 414, p_lich_sage,   1.8, -0.1}
}
for _, l in ipairs(lichens) do
  b_facet:load(l[3], 0.8)
  b_facet:touch(l[1], l[2], {pressure=0.55, drag={l[4], l[5]}})
end

-- 4. Deeply Bed the Dolmen into Ancient Peat & Heather
b_block:load(p_peat_deep, 0.95)
b_block:stroke({{280, 430}, {320, 433}, {365, 435}, {410, 435}, {455, 434}, {495, 430}}, {pressure=0.95})
b_block:load(p_heather_purp, 0.95)
b_block:stroke({{285, 434}, {330, 436}, {380, 438}, {430, 437}, {475, 435}, {505, 432}}, {pressure=0.95})

print("Dolmen remodeled into authentic erratic granite boulders; square doorway eliminated; driveway path covered.")

--@ chunk 134
-- Stage 1: Obliterate White Path & Resculpt Dolmen into Rugged Scandinavian Glacial Erratics

-- 1. Piles
local p_peat_dark    = pile{{"raw umber", 11}, {"bone black", 8}, {"red earth", 3.5}, medium=0.15}
local p_peat_body    = pile{{"raw umber", 9}, {"bone black", 5}, {"red earth", 3.5}, {"yellow ochre", 3.0}, medium=0.18}
local p_heather_purp = pile{{"raw umber", 7}, {"red earth", 6.0}, {"bone black", 3.5}, {"smalt", 2.0}, {"yellow ochre", 2.0}, medium=0.18}
local p_turf_warm    = pile{{"yellow ochre", 8}, {"raw umber", 5.0}, {"red earth", 2.5}, {"bone black", 1.2}, {"lead white", 2.0}, medium=0.2}

local p_granite_deep = pile{{"bone black", 11}, {"raw umber", 8}, {"red earth", 2.5}, medium=0.14}
local p_granite_core = pile{{"raw umber", 8}, {"bone black", 5}, {"red earth", 3.0}, {"yellow ochre", 3.0}, {"lead white", 3.5}, {"smalt", 1.5}, medium=0.18}
local p_granite_facet= pile{{"lead white", 10}, {"yellow ochre", 5.0}, {"raw umber", 4.0}, {"red earth", 2.0}, {"bone black", 1.0}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 16}, {"yellow ochre", 4.5}, {"chrome yellow", 0.8}, {"red earth", 1.2}, {"raw umber", 1.0}, medium=0.2}
local p_granite_rim  = pile{{"lead white", 22}, {"yellow ochre", 3.0}, {"chrome yellow", 0.6}, {"smalt", 1.2}, medium=0.2}

local p_lich_orange  = pile{{"yellow ochre", 10}, {"chrome yellow", 3.5}, {"red earth", 2.5}, {"lead white", 5.0}, medium=0.25}
local p_lich_sage    = pile{{"lead white", 12}, {"green earth", 8.0}, {"yellow ochre", 4.0}, {"raw umber", 2.0}, medium=0.25}

local p_sky_glow     = pile{{"lead white", 27}, {"yellow ochre", 5.0}, {"chrome yellow", 2.0}, {"vermilion", 0.45}, {"raw umber", 0.25}, medium=0.22}

-- 2. 100% Obliterate the White Path with Opaque Peat and Heather
local pts_path_kill = {
  {420, 440}, {480, 440}, {550, 490}, {620, 540}, {690, 600}, {760, 660}, {780, 704},
  {660, 704}, {610, 650}, {540, 590}, {470, 530}, {420, 480}, {420, 440}
}
local m_path_kill = poly(pts_path_kill, false)
work(m_path_kill, {hand="body", pile=p_peat_dark, angle=0.3, coverage=2.6, fill=true, clip=true})

-- Blend heather and warm turf across the obliterated path
local b_broad = brush{kind="filbert", width=14.0, stiffness=0.85}
b_broad:load(p_heather_purp, 0.95)
b_broad:stroke({{440, 460}, {520, 485}, {610, 510}, {710, 520}}, {pressure=0.95})
b_broad:stroke({{460, 520}, {550, 545}, {640, 565}, {740, 570}}, {pressure=0.95})
b_broad:stroke({{500, 580}, {600, 605}, {700, 625}, {800, 630}}, {pressure=0.95})
b_broad:stroke({{540, 640}, {640, 665}, {740, 685}, {840, 690}}, {pressure=0.95})

b_broad:load(p_turf_warm, 0.85)
b_broad:stroke({{450, 490}, {540, 515}, {630, 535}, {720, 540}}, {pressure=0.8})
b_broad:stroke({{490, 550}, {580, 575}, {680, 595}, {770, 600}}, {pressure=0.8})
b_broad:stroke({{530, 610}, {630, 635}, {730, 655}, {820, 660}}, {pressure=0.8})

-- Submerge the two "sugar cube" boulders completely into heather
b_broad:load(p_peat_dark, 0.95)
b_broad:stroke({{210, 538}, {245, 546}, {275, 542}}, {pressure=0.95})
b_broad:stroke({{650, 546}, {690, 554}, {730, 550}}, {pressure=0.95})
b_broad:load(p_heather_purp, 0.95)
b_broad:stroke({{205, 542}, {245, 550}, {280, 546}}, {pressure=0.95})
b_broad:stroke({{645, 550}, {690, 558}, {735, 554}}, {pressure=0.95})

-- 3. Cut Down the "Shed" Wings: Trim Dolmen to Natural Megalithic Width (x ≈ 335 to 442)
-- Cut left wing (x = 310 to 335) with peaty turf and heather
local m_trim_left = poly({{308, 390}, {336, 390}, {336, 440}, {308, 440}}, false)
work(m_trim_left, {hand="body", pile=p_peat_dark, angle=0.2, coverage=2.4, fill=true, clip=true})

-- Cut right wing (x = 442 to 478) with peaty turf and heather
local m_trim_right = poly({{442, 388}, {480, 388}, {480, 440}, {442, 440}}, false)
work(m_trim_right, {hand="body", pile=p_peat_dark, angle=-0.2, coverage=2.4, fill=true, clip=true})

-- Cut the top skyline above capstone (y = 368 to 382) to lower the capstone
local m_sky_trim = poly({{330, 368}, {450, 368}, {450, 382}, {330, 382}}, false)
work(m_sky_trim, {hand="body", pile=p_sky_glow, angle=0, coverage=2.2, fill=true, clip=true})

-- 4. Resculpt the Dolmen: A True Scandinavian Glacial Erratic Cromlech (Width ~105 units)
local b_block = brush{kind="filbert", width=6.5, stiffness=0.85}
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_crack = brush{kind="round", width=1.0, point=1, stiffness=0.9}

-- A. The Left Orthostat: Massive, rounded, weathered Scandinavian granite boulder (x ≈ 335 to 375, y ≈ 400 to 435)
local pts_lo = {{336, 400}, {374, 400}, {376, 434}, {338, 434}}
work(poly(pts_lo, false), {hand="body", pile=p_granite_core, angle=1.4, coverage=2.4, fill=true, clip=true})
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{336, 402}, {340, 416}, {344, 428}, {348, 434}}, {pressure={0.7, 0.85, 0.65, 0.2}})

-- B. The Right Orthostat: Massive, weathered granite boulder leaning inward (x ≈ 400 to 442, y ≈ 400 to 435)
local pts_ro = {{402, 400}, {440, 400}, {442, 434}, {404, 434}}
work(poly(pts_ro, false), {hand="body", pile=p_granite_core, angle=1.4, coverage=2.4, fill=true, clip=true})
b_facet:load(p_granite_lit, 0.9)
b_facet:stroke({{430, 402}, {434, 416}, {436, 428}, {438, 434}}, {pressure={0.7, 0.85, 0.65, 0.2}})

-- C. The Sacred Chamber Void: A mysterious, narrow dark crevice (x ≈ 374 to 402, y ≈ 402 to 434 — width only 28 units!)
local pts_void = {{374, 402}, {402, 402}, {404, 434}, {376, 434}}
work(poly(pts_void, false), {hand="detail", pile=p_granite_deep, angle=1.3, coverage=2.2, fill=true, clip=true})

-- Center rear boulder partially seen inside the narrow crevice
b_facet:load(p_granite_core, 0.85)
b_facet:stroke({{386, 404}, {388, 418}, {390, 430}}, {pressure=0.8})

-- D. The Massive Erratic Capstone: Heavy, angular, weathered slab (x ≈ 332 to 444, y ≈ 382 to 404)
local pts_cap = {
  {334, 402}, {338, 394}, {352, 386}, {375, 382}, {405, 381},
  {428, 384}, {442, 392}, {440, 402}, {415, 403}, {385, 404},
  {355, 404}, {334, 402}
}
work(poly(pts_cap, false), {hand="body", pile=p_granite_core, angle=0.08, coverage=2.4, fill=true, clip=true})

-- Western natural granite slope
local pts_f_west = {
  {334, 402}, {338, 394}, {352, 386}, {375, 382}, {390, 382},
  {392, 394}, {365, 396}, {340, 398}, {334, 402}
}
work(poly(pts_f_west, false), {hand="detail", pile=p_granite_facet, angle=0.06, coverage=1.8, fill=true, clip=true})

-- Eastern stepped facet catching evening light
local pts_f_east = {
  {390, 382}, {405, 381}, {428, 384}, {442, 392}, {440, 398},
  {420, 396}, {392, 394}, {390, 382}
}
work(poly(pts_f_east, false), {hand="detail", pile=p_granite_lit, angle=-0.04, coverage=1.8, fill=true, clip=true})

-- Heavy rock undercut shadow immediately under capstone
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{336, 402}, {360, 402}, {388, 403}, {415, 402}, {438, 402}}, {pressure=0.95})

-- Natural geological fracture across capstone at x ≈ 390
b_crack:stroke({{390, 382}, {392, 393}, {391, 403}}, {pressure={0.65, 0.8, 0.35}})

-- Top crystalline chipped rock rim catching twilight
b_crack:load(p_granite_rim, 0.9)
b_crack:stroke({{334, 402}, {342, 393}, {356, 386}, {375, 382}, {390, 381}}, {pressure={0.3, 0.75, 0.7, 0.85, 0.4}})
b_crack:stroke({{390, 381}, {410, 382}, {428, 385}, {438, 392}}, {pressure={0.4, 0.85, 0.75, 0.3}})

-- Lichens in crevice clusters
local lichens = {
  {346, 390, p_lich_orange, 2.0, 0.2},
  {366, 385, p_lich_sage,   2.2, 0.1},
  {384, 383, p_lich_orange, 1.8, 0.1},
  {412, 383, p_lich_sage,   2.0, -0.1},
  {430, 387, p_lich_orange, 1.8, -0.1},
  {342, 412, p_lich_orange, 1.6, 0.1},
  {432, 412, p_lich_sage,   1.6, -0.1}
}
for _, l in ipairs(lichens) do
  b_facet:load(l[3], 0.8)
  b_facet:touch(l[1], l[2], {pressure=0.55, drag={l[4], l[5]}})
end

-- 5. Bed Stones Deeply into Tumulus Peat & Dark Heather
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{310, 432}, {340, 434}, {385, 436}, {430, 435}, {465, 432}}, {pressure=0.95})
b_block:load(p_heather_purp, 0.95)
b_block:stroke({{305, 435}, {345, 437}, {390, 439}, {435, 437}, {470, 434}}, {pressure=0.95})

print("Path obliterated with opaque peat/heather; dolmen wings trimmed; natural erratic cromlech sculpted.")

--@ chunk 135
local t = wait(3 * 24 * 60)
print("Time now:", t)
print("Drying at sky (200, 200):", drying(200, 200))
print("Drying at dolmen (400, 400):", drying(400, 400))
print("Drying at barrow (500, 550):", drying(500, 550))

--@ chunk 136
local t = wait(3 * 24 * 60)
print("Time now:", t)
print("Drying at dolmen (400, 400):", drying(400, 400))

--@ chunk 137
local t = wait(10 * 24 * 60)
print("Time now:", t)
print("Drying at dolmen (400, 400):", drying(400, 400))

--@ chunk 138
-- Stage 1: Restore the Master Friedrich Twilight Firmament & Noble Barrow Heathland across Full Width

-- 1. Sky Piles
local p_sky_zenith = pile{{"lead white", 18}, {"cobalt blue", 6.2}, {"smalt", 4.5}, {"raw umber", 0.6}, {"bone black", 0.25}, medium=0.22}
local p_sky_upper  = pile{{"lead white", 26}, {"cobalt blue", 3.2}, {"smalt", 3.2}, {"yellow ochre", 0.8}, {"raw umber", 0.3}, medium=0.22}
local p_sky_mid    = pile{{"lead white", 28}, {"yellow ochre", 3.5}, {"chrome yellow", 0.9}, {"smalt", 1.0}, {"raw umber", 0.2}, {"vermilion", 0.1}, medium=0.22}
local p_sky_glow   = pile{{"lead white", 26}, {"yellow ochre", 5.2}, {"chrome yellow", 2.2}, {"vermilion", 0.5}, {"raw umber", 0.25}, medium=0.22}

-- 2. Define the Natural Swelling Barrow Crest
local barrow_nodes = {
  {x=-20, y=465}, {x=80, y=458}, {x=180, y=448}, {x=260, y=430},
  {x=330, y=415}, {x=385, y=408}, {x=445, y=414}, {x=520, y=430},
  {x=600, y=448}, {x=680, y=450}, {x=760, y=436}, {x=830, y=436},
  {x=910, y=448}, {x=1020, y=465}
}

local function barrow_crest(x)
  if x <= barrow_nodes[1].x then return barrow_nodes[1].y end
  if x >= barrow_nodes[#barrow_nodes].x then return barrow_nodes[#barrow_nodes].y end
  for i = 1, #barrow_nodes - 1 do
    local p0 = barrow_nodes[i]
    local p1 = barrow_nodes[i+1]
    if x >= p0.x and x <= p1.x then
      local u = (x - p0.x) / (p1.x - p0.x)
      local s = u * u * (3 - 2 * u)
      return p0.y + s * (p1.y - p0.y)
    end
  end
  return 430
end

local m_sky_domain = above(barrow_crest)

-- 3. Four overlapping full-width bands covering x = 0 to 1000 from zenith down to barrow crest
local m1 = (rect(0, 0, 1000, 140):soften(25)) * m_sky_domain
work(m1, {hand="body", pile=p_sky_zenith, angle=0, order="across", coverage=2.4, fill=true})

local m2 = (rect(0, 105, 1000, 145):soften(25)) * m_sky_domain
work(m2, {hand="body", pile=p_sky_upper, angle=0, order="across", coverage=2.4, fill=true})

local m3 = (rect(0, 210, 1000, 135):soften(25)) * m_sky_domain
work(m3, {hand="body", pile=p_sky_mid, angle=0, order="across", coverage=2.4, fill=true})

-- Band 4 extends fully down behind the barrow crest, covering all previous dolmen artifacts above crest!
local m4 = (rect(0, 305, 1000, 160):soften(20)) * m_sky_domain
work(m4, {hand="body", pile=p_sky_glow, angle=0, order="across", coverage=2.6, fill=true})

-- Seamless badger blending across full canvas width
local m_sky_blend = rect(0, 0, 1000, 420) * m_sky_domain
blend(m_sky_blend, {angle=1.57})
blend(m_sky_blend, {angle=0})

-- 4. Distant low Baltic hills silhouetted in soft atmospheric haze
local p_dist_coast = pile{{"lead white", 15}, {"smalt", 5.0}, {"raw umber", 3.0}, {"red earth", 0.9}, {"bone black", 0.4}, medium=0.25}
local pts_distant_land = {
  {0, 464}, {100, 460}, {220, 454}, {310, 450}, {310, 460},
  {500, 452}, {580, 450}, {700, 447}, {820, 452}, {920, 458}, {1000, 464},
  {1000, 475}, {0, 475}
}
local m_coast = (poly(pts_distant_land, true):soften(2.5)) * m_sky_domain
work(m_coast, {hand="body", pile=p_dist_coast, angle=0, coverage=1.4, fill=true, clip=true})

-- Calm water sheen and valley evening mist pooling over the water
local p_water_mist = pile{{"lead white", 26}, {"yellow ochre", 3.2}, {"smalt", 1.8}, {"raw umber", 0.3}, medium=0.3}
local m_water = (rect(0, 434, 1000, 30):soften(4)) * m_sky_domain
work(m_water, {hand="glaze", pile=p_water_mist, angle=0, coverage=1.2, fill=false})
blend(rect(0, 430, 1000, 36) * m_sky_domain, {angle=0})

-- 5. Waxing Crescent Moon & Solitary Evening Star (Venus / Hesperus)
local p_moon_pure = pile{{"lead white", 34}, {"yellow ochre", 1.0}, {"chrome yellow", 0.4}, medium=0.12}
local b_rigger = brush{kind="round", width=1.1, point=1, stiffness=0.9}
b_rigger:load(p_moon_pure, 0.95)
b_rigger:stroke({
  {645.2, 105.5}, {641.5, 108.0}, {639.0, 112.5}, {640.2, 117.0}, {644.0, 119.5}
}, {pressure={0.08, 0.85, 1.0, 0.8, 0.08}, ramps={0.05, 0.05}})

local b_star = brush{kind="round", width=0.9, point=1, stiffness=0.9}
b_star:load(p_moon_pure, 1.0)
b_star:touch(688, 148, {pressure=0.8})
b_star:stroke({{686.8, 148}, {689.2, 148}}, {pressure={0.2, 0.5, 0.2}})
b_star:stroke({{688, 146.8}, {688, 149.2}}, {pressure={0.2, 0.5, 0.2}})

-- 6. The Noble Prehistoric Barrow Tumulus (Wild Baltic Heathland)
local p_peat_dark    = pile{{"raw umber", 11}, {"bone black", 8}, {"red earth", 3.5}, medium=0.15}
local p_peat_body    = pile{{"raw umber", 9}, {"bone black", 5}, {"red earth", 3.5}, {"yellow ochre", 3.0}, medium=0.18}
local p_heather_purp = pile{{"raw umber", 7}, {"red earth", 6.0}, {"bone black", 3.5}, {"smalt", 2.0}, {"yellow ochre", 2.0}, medium=0.18}
local p_turf_warm    = pile{{"yellow ochre", 8}, {"raw umber", 5.0}, {"red earth", 2.5}, {"bone black", 1.2}, {"lead white", 2.0}, medium=0.2}
local p_moss_deep    = pile{{"green earth", 7}, {"raw umber", 5}, {"yellow ochre", 4}, {"bone black", 2}, {"lead white", 1.5}, medium=0.2}

local m_barrow = below(barrow_crest)

-- 100% Opaque Peat Foundation across the entire lower canvas (wipes all previous paths and cuts!)
work(m_barrow, {hand="body", pile=p_peat_body, angle=0.2, coverage=2.6, fill=true, clip=true})

-- Crisp, firm peaty crest contour against the glowing evening mist
local b_crest = brush{kind="round", width=2.4, point=1, stiffness=0.9}
b_crest:load(p_peat_dark, 0.95)
local crest_pts = {}
for x = -10, 1010, 8 do
  table.insert(crest_pts, {x, barrow_crest(x)})
end
b_crest:stroke(crest_pts, {pressure=0.95, ramps={0.01, 0.01}})

-- 7. Rolling Banks of Late-Autumn Heather (Calluna vulgaris) & Peaty Folds
local b_broad = brush{kind="filbert", width=16.0, stiffness=0.85}

-- Heather drifts sweeping organically with the rolling topography
b_broad:load(p_heather_purp, 0.95)
b_broad:stroke({{60, 480}, {160, 495}, {280, 505}, {400, 508}}, {pressure=0.9})
b_broad:stroke({{50, 535}, {180, 550}, {310, 560}, {450, 558}}, {pressure=0.9})
b_broad:stroke({{80, 595}, {220, 610}, {370, 620}, {520, 615}}, {pressure=0.9})
b_broad:stroke({{100, 660}, {260, 672}, {440, 678}, {620, 672}}, {pressure=0.9})

b_broad:stroke({{580, 495}, {700, 482}, {830, 478}, {950, 482}}, {pressure=0.9})
b_broad:stroke({{540, 545}, {670, 536}, {800, 532}, {940, 538}}, {pressure=0.9})
b_broad:stroke({{510, 610}, {650, 602}, {790, 598}, {930, 605}}, {pressure=0.9})
b_broad:stroke({{480, 670}, {630, 668}, {780, 662}, {920, 668}}, {pressure=0.9})

-- Deep peaty hollows
b_broad:load(p_peat_dark, 0.9)
b_broad:stroke({{180, 518}, {290, 528}, {410, 532}}, {pressure=0.85})
b_broad:stroke({{520, 520}, {630, 515}, {750, 512}}, {pressure=0.85})
b_broad:stroke({{240, 638}, {410, 648}, {580, 645}}, {pressure=0.85})

-- Mossy hollows
b_broad:load(p_moss_deep, 0.85)
b_broad:stroke({{140, 550}, {250, 562}, {380, 565}}, {pressure=0.8})
b_broad:stroke({{560, 570}, {690, 565}, {820, 560}}, {pressure=0.8})

-- Warm golden bent-grass ridges catching evening light
b_broad:load(p_turf_warm, 0.85)
b_broad:stroke({{120, 465}, {220, 475}, {320, 480}}, {pressure=0.75})
b_broad:stroke({{640, 465}, {760, 455}, {880, 458}}, {pressure=0.75})
b_broad:stroke({{160, 575}, {300, 585}, {440, 588}}, {pressure=0.75})
b_broad:stroke({{570, 585}, {720, 580}, {860, 575}}, {pressure=0.75})

print("Stage 1 complete: Master Friedrichian firmament and wild Baltic barrow heathland restored.")

--@ chunk 139
-- Paint the Authentic Megalithic Tomb (Scandinavian Granite Glacial Erratics)

-- 1. Piles
local p_void_pitch   = pile{{"bone black", 15}, {"raw umber", 7}, medium=0.1}
local p_granite_deep = pile{{"bone black", 10}, {"raw umber", 8}, {"red earth", 2.5}, {"smalt", 1.5}, medium=0.15}
local p_granite_core = pile{{"raw umber", 8}, {"bone black", 5}, {"red earth", 3.0}, {"yellow ochre", 3.0}, {"lead white", 4.0}, {"smalt", 1.5}, medium=0.18}
local p_granite_facet= pile{{"lead white", 12}, {"yellow ochre", 5.0}, {"raw umber", 4.0}, {"red earth", 2.0}, {"smalt", 1.2}, medium=0.2}
local p_granite_lit  = pile{{"lead white", 18}, {"yellow ochre", 4.5}, {"chrome yellow", 0.8}, {"red earth", 1.2}, {"raw umber", 1.0}, medium=0.2}
local p_granite_rim  = pile{{"lead white", 24}, {"yellow ochre", 3.0}, {"chrome yellow", 0.6}, {"smalt", 1.2}, medium=0.2}

local p_lich_orange  = pile{{"yellow ochre", 10}, {"chrome yellow", 3.5}, {"red earth", 2.5}, {"lead white", 6.0}, medium=0.25}
local p_lich_sage    = pile{{"lead white", 12}, {"green earth", 8.0}, {"yellow ochre", 4.0}, {"raw umber", 2.0}, medium=0.25}

local p_peat_dark    = pile{{"raw umber", 11}, {"bone black", 8}, {"red earth", 3.5}, medium=0.15}
local p_heather_purp = pile{{"raw umber", 7}, {"red earth", 6.0}, {"bone black", 3.5}, {"smalt", 2.0}, medium=0.18}

local b_block = brush{kind="filbert", width=5.5, stiffness=0.85}
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local b_crack = brush{kind="round", width=1.0, point=1, stiffness=0.9}

-- 2. The Mysterious Sacred Chamber Void (Narrow, dark crevice between x = 378 and 394, y = 398 to 418)
b_facet:load(p_void_pitch, 0.95)
b_facet:stroke({{380, 400}, {384, 412}, {386, 420}}, {pressure=0.95})
b_facet:stroke({{388, 400}, {390, 412}, {392, 420}}, {pressure=0.95})

-- 3. Left Orthostat: Massive, rounded Scandinavian glacial erratic boulder (x ≈ 352 to 380, y ≈ 396 to 420)
b_block:load(p_granite_core, 0.95)
b_block:stroke({{356, 398}, {359, 410}, {362, 420}}, {pressure=0.95})
b_block:stroke({{366, 396}, {368, 410}, {370, 420}}, {pressure=0.95})
b_block:stroke({{374, 398}, {376, 410}, {378, 418}}, {pressure=0.9})

-- Rounded outer facet catching twilight
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{354, 398}, {357, 408}, {361, 418}}, {pressure={0.7, 0.85, 0.3}})
-- Inner shadow facing chamber
b_crack:load(p_granite_deep, 0.9)
b_crack:stroke({{378, 399}, {380, 412}, {381, 419}}, {pressure=0.8})

-- 4. Right Orthostat: Heavy, weathered, crystalline boulder (x ≈ 392 to 420, y ≈ 396 to 420)
b_block:load(p_granite_core, 0.95)
b_block:stroke({{394, 398}, {396, 410}, {398, 418}}, {pressure=0.9})
b_block:stroke({{404, 396}, {405, 410}, {406, 420}}, {pressure=0.95})
b_block:stroke({{414, 398}, {415, 410}, {416, 420}}, {pressure=0.95})

-- Outer facet catching warm evening light
b_facet:load(p_granite_lit, 0.9)
b_facet:stroke({{410, 398}, {413, 408}, {416, 418}}, {pressure={0.7, 0.85, 0.3}})
-- Inner shadow facing chamber
b_crack:load(p_granite_deep, 0.9)
b_crack:stroke({{393, 399}, {394, 412}, {394, 419}}, {pressure=0.8})

-- 5. The Colossal Scandinavian Granite Capstone (x ≈ 344 to 426, y ≈ 376 to 400)
-- A thick, heavy, irregular erratic block resting securely upon both boulders
b_block:load(p_granite_core, 0.95)
b_block:stroke({{346, 396}, {370, 397}, {395, 397}, {422, 395}}, {pressure=0.95})
b_block:stroke({{350, 389}, {375, 389}, {400, 390}, {420, 388}}, {pressure=0.95})
b_block:stroke({{358, 382}, {380, 380}, {402, 380}, {416, 382}}, {pressure=0.9})

-- Western natural granite slope (weathered, crystalline)
b_facet:load(p_granite_facet, 0.9)
b_facet:stroke({{346, 395}, {362, 387}, {382, 382}}, {pressure={0.65, 0.85, 0.4}})
b_facet:stroke({{352, 397}, {370, 391}, {386, 386}}, {pressure={0.5, 0.75, 0.35}})

-- Eastern stepped facet catching evening light
b_facet:load(p_granite_lit, 0.9)
b_facet:stroke({{384, 382}, {404, 381}, {420, 386}}, {pressure={0.55, 0.85, 0.4}})
b_facet:stroke({{386, 388}, {406, 388}, {422, 393}}, {pressure={0.45, 0.75, 0.35}})

-- Heavy rock undercut shadow beneath capstone overhang
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{346, 398}, {368, 398}, {385, 399}, {405, 398}, {422, 397}}, {pressure=0.95})

-- Natural geological fracture across capstone at x ≈ 384
b_crack:stroke({{383, 379}, {385, 388}, {384, 398}}, {pressure={0.6, 0.8, 0.3}})

-- Top crystalline chipped rock rim catching twilight sky
b_crack:load(p_granite_rim, 0.9)
b_crack:stroke({{346, 396}, {355, 388}, {368, 383}, {384, 380}}, {pressure={0.3, 0.75, 0.8, 0.4}})
b_crack:stroke({{384, 380}, {402, 381}, {415, 384}, {422, 392}}, {pressure={0.4, 0.85, 0.7, 0.25}})

-- Crustose lichens nestled on stone facets
local lichens = {
  {356, 388, p_lich_orange, 1.8, 0.1},
  {372, 383, p_lich_sage,   2.0, 0.1},
  {396, 382, p_lich_orange, 1.8, -0.1},
  {412, 386, p_lich_sage,   1.8, -0.1},
  {362, 408, p_lich_orange, 1.6, 0.1},
  {408, 408, p_lich_sage,   1.6, -0.1}
}
for _, l in ipairs(lichens) do
  b_facet:load(l[3], 0.8)
  b_facet:touch(l[1], l[2], {pressure=0.5, drag={l[4], l[5]}})
end

-- 6. Bed the Stones Deeply into Tumulus Peat & Dark Heather
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{335, 418}, {365, 420}, {395, 421}, {425, 420}, {445, 418}}, {pressure=0.95})
b_block:load(p_heather_purp, 0.95)
b_block:stroke({{340, 420}, {370, 422}, {400, 422}, {430, 421}}, {pressure=0.9})

print("Authentic Scandinavian granite dolmen painted atop the barrow summit.")

--@ chunk 140
-- Construct the Megalithic Tomb as Three Solid, Smoothed Scandinavian Glacial Boulders

-- 1. Piles
local p_void_pitch   = pile{{"bone black", 15}, {"raw umber", 7}, medium=0.1}
local p_granite_deep = pile{{"bone black", 10}, {"raw umber", 8}, {"red earth", 2.5}, {"smalt", 1.5}, medium=0.15}
local p_granite_core = pile{{"raw umber", 8}, {"bone black", 5}, {"red earth", 3.0}, {"yellow ochre", 3.0}, {"lead white", 4.0}, {"smalt", 1.5}, medium=0.16}
local p_granite_facet= pile{{"lead white", 14}, {"yellow ochre", 5.0}, {"raw umber", 3.5}, {"red earth", 2.0}, {"smalt", 1.2}, medium=0.18}
local p_granite_lit  = pile{{"lead white", 20}, {"yellow ochre", 4.5}, {"chrome yellow", 0.8}, {"red earth", 1.2}, {"raw umber", 1.0}, medium=0.18}
local p_granite_rim  = pile{{"lead white", 26}, {"yellow ochre", 3.0}, {"chrome yellow", 0.6}, {"smalt", 1.2}, medium=0.18}

local p_lich_orange  = pile{{"yellow ochre", 10}, {"chrome yellow", 3.5}, {"red earth", 2.5}, {"lead white", 5.0}, medium=0.25}
local p_lich_sage    = pile{{"lead white", 12}, {"green earth", 8.0}, {"yellow ochre", 4.0}, {"raw umber", 2.0}, medium=0.25}

local p_peat_dark    = pile{{"raw umber", 11}, {"bone black", 8}, {"red earth", 3.5}, medium=0.15}
local p_heather_purp = pile{{"raw umber", 7}, {"red earth", 6.0}, {"bone black", 3.5}, {"smalt", 2.0}, medium=0.18}

-- 2. First cover the previous streaky marks with background sky/peat so the stones sit clean
local p_sky_glow = pile{{"lead white", 26}, {"yellow ochre", 5.2}, {"chrome yellow", 2.2}, {"vermilion", 0.5}, {"raw umber", 0.25}, medium=0.22}
local m_clean = rect(330, 370, 115, 55):soften(4)
work(m_clean, {hand="body", pile=p_sky_glow, angle=0, coverage=2.2, fill=true, clip=false})

-- 3. The Chamber Void (Deep mysterious darkness between the two supporting boulders)
local pts_void = {{365, 396}, {402, 396}, {404, 422}, {364, 422}}
local m_void = poly(pts_void, true)
work(m_void, {hand="body", pile=p_void_pitch, angle=1.4, coverage=2.4, fill=true, clip=true})

-- 4. Left Supporting Boulder (Massive, rounded Scandinavian glacial erratic, width ~34)
local pts_left = {{340, 422}, {338, 404}, {348, 395}, {368, 396}, {374, 406}, {372, 422}}
local m_left = poly(pts_left, true)
work(m_left, {hand="body", pile=p_granite_core, angle=1.4, coverage=2.5, fill=true, clip=true})

-- Left boulder facet catching twilight
local pts_left_f = {{338, 404}, {348, 395}, {360, 398}, {352, 420}, {340, 420}}
work(poly(pts_left_f, true), {hand="detail", pile=p_granite_facet, angle=1.35, coverage=1.8, fill=true, clip=true})

-- 5. Right Supporting Boulder (Massive, weathered granite erratic, width ~34)
local pts_right = {{396, 422}, {394, 404}, {404, 395}, {424, 396}, {430, 406}, {426, 422}}
local m_right = poly(pts_right, true)
work(m_right, {hand="body", pile=p_granite_core, angle=1.4, coverage=2.5, fill=true, clip=true})

-- Right boulder facet catching warm evening light
local pts_right_f = {{408, 396}, {424, 396}, {430, 406}, {426, 422}, {416, 420}}
work(poly(pts_right_f, true), {hand="detail", pile=p_granite_lit, angle=1.4, coverage=1.8, fill=true, clip=true})

-- 6. The Massive Erratic Capstone (Colossal weathered slab resting on both boulders, width ~96)
local pts_cap = {
  {336, 400}, {338, 388}, {355, 380}, {385, 376}, {415, 378},
  {432, 386}, {434, 398}, {410, 400}, {385, 401}, {355, 401}
}
local m_cap = poly(pts_cap, true)
work(m_cap, {hand="body", pile=p_granite_core, angle=0.08, coverage=2.6, fill=true, clip=true})

-- Western skyward facet of capstone (crystalline granite)
local pts_cap_w = {
  {336, 400}, {338, 388}, {355, 380}, {385, 376},
  {388, 388}, {360, 392}, {342, 396}
}
work(poly(pts_cap_w, true), {hand="detail", pile=p_granite_facet, angle=0.06, coverage=1.8, fill=true, clip=true})

-- Eastern skyward facet (warm twilight reflection)
local pts_cap_e = {
  {385, 376}, {415, 378}, {432, 386}, {434, 398},
  {412, 396}, {390, 392}, {385, 376}
}
work(poly(pts_cap_e, true), {hand="detail", pile=p_granite_lit, angle=-0.04, coverage=1.8, fill=true, clip=true})

-- Deep undercut cast shadow beneath capstone overhang
local b_crack = brush{kind="round", width=1.1, point=1, stiffness=0.9}
b_crack:load(p_granite_deep, 0.95)
b_crack:stroke({{338, 400}, {365, 401}, {385, 401}, {408, 400}, {432, 400}}, {pressure=0.95})

-- Natural cleavage fracture across capstone at x ≈ 386
b_crack:stroke({{385, 376}, {387, 388}, {386, 401}}, {pressure={0.65, 0.8, 0.35}})

-- Chipped crystalline rock rim along skyline catching twilight
b_crack:load(p_granite_rim, 0.9)
b_crack:stroke({{336, 398}, {344, 389}, {358, 382}, {385, 376}}, {pressure={0.3, 0.75, 0.8, 0.4}})
b_crack:stroke({{385, 376}, {405, 377}, {422, 382}, {432, 392}}, {pressure={0.4, 0.85, 0.7, 0.25}})

-- Crustose lichens nestled on rock planes
local b_facet = brush{kind="round", width=2.4, point=1, stiffness=0.85}
local lichens = {
  {352, 386, p_lich_orange, 2.0, 0.1},
  {370, 382, p_lich_sage,   2.2, 0.1},
  {398, 380, p_lich_orange, 1.8, -0.1},
  {418, 384, p_lich_sage,   2.0, -0.1},
  {346, 408, p_lich_orange, 1.8, 0.1},
  {420, 408, p_lich_sage,   1.8, -0.1}
}
for _, l in ipairs(lichens) do
  b_facet:load(l[3], 0.8)
  b_facet:touch(l[1], l[2], {pressure=0.55, drag={l[4], l[5]}})
end

-- 7. Bed Stones Deeply into Peat and Dark Heather
local b_block = brush{kind="filbert", width=6.5, stiffness=0.85}
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{325, 418}, {355, 420}, {385, 421}, {415, 420}, {445, 418}}, {pressure=0.95})
b_block:load(p_heather_purp, 0.95)
b_block:stroke({{330, 421}, {360, 422}, {390, 423}, {420, 422}, {440, 420}}, {pressure=0.9})

print("Dolmen constructed: three solid, smoothed Scandinavian glacial erratics.")

--@ chunk 141
-- Construct the Monumental Ancient Veteran Oak, Companion Tree, and Heath Accents

-- 1. Pigment Piles
local p_bark_core   = pile{{"bone black", 13}, {"raw umber", 8}, {"red earth", 2.5}, medium=0.14}
local p_bark_furrow = pile{{"bone black", 16}, {"raw umber", 6}, medium=0.12}
local p_bark_mid    = pile{{"raw umber", 8}, {"bone black", 4}, {"yellow ochre", 3.5}, {"lead white", 2.5}, {"red earth", 1.5}, medium=0.18}
local p_bark_ridge  = pile{{"yellow ochre", 6}, {"raw umber", 5}, {"lead white", 4.5}, {"bone black", 1.2}, {"red earth", 1.0}, medium=0.2}

local p_hollow_void = pile{{"bone black", 16}, {"raw umber", 6}, medium=0.1}
local p_woundwood   = pile{{"raw umber", 6}, {"yellow ochre", 5}, {"lead white", 4}, {"red earth", 2}, medium=0.2}

local p_stag_dark   = pile{{"bone black", 8}, {"raw umber", 8}, {"lead white", 4}, {"smalt", 1.5}, medium=0.16}
local p_stag_bleach = pile{{"lead white", 18}, {"raw umber", 4.5}, {"yellow ochre", 2.5}, {"smalt", 1.5}, medium=0.18}
local p_stag_tip    = pile{{"lead white", 24}, {"raw umber", 3.0}, {"yellow ochre", 2.0}, {"smalt", 1.2}, medium=0.18}

local p_leaf_russet = pile{{"red earth", 7}, {"yellow ochre", 6}, {"raw umber", 5}, {"lead white", 2}, medium=0.2}
local p_leaf_gold   = pile{{"yellow ochre", 11}, {"chrome yellow", 2.5}, {"red earth", 2.5}, {"lead white", 3}, medium=0.22}
local p_leaf_rim    = pile{{"lead white", 14}, {"yellow ochre", 10}, {"chrome yellow", 3.0}, medium=0.2}

local p_peat_dark    = pile{{"raw umber", 11}, {"bone black", 8}, {"red earth", 3.5}, medium=0.15}
local p_heather_purp = pile{{"raw umber", 7}, {"red earth", 6.0}, {"bone black", 3.5}, {"smalt", 2.0}, medium=0.18}
local p_straw_sharp  = pile{{"lead white", 18}, {"yellow ochre", 9}, {"chrome yellow", 1.2}, {"raw umber", 1.0}, medium=0.22}
local p_straw_shade  = pile{{"yellow ochre", 7}, {"raw umber", 6}, {"bone black", 2.5}, {"lead white", 2.0}, medium=0.2}

local b_block  = brush{kind="filbert", width=7.0, stiffness=0.85}
local b_branch = brush{kind="round", width=3.2, point=1, stiffness=0.85}
local b_twig   = brush{kind="round", width=1.4, point=1, stiffness=0.9}
local b_rigger = brush{kind="round", width=0.8, point=1, stiffness=0.9}

-- 2. Bed the Dolmen Base into Tumulus Peat & Heather
b_block:load(p_peat_dark, 0.95)
b_block:stroke({{320, 416}, {355, 418}, {385, 420}, {415, 419}, {445, 416}}, {pressure=0.95})
b_block:load(p_heather_purp, 0.95)
b_block:stroke({{325, 418}, {360, 420}, {390, 421}, {420, 420}, {440, 418}}, {pressure=0.9})

-- 3. The Ancient Veteran Oak: Solid Trunk Bole & Root Buttresses
-- Define smoothed organic trunk contour (width ~26 to 34)
local pts_trunk = {
  {238, 452}, {248, 436}, {256, 415}, {260, 390}, {262, 365},
  {270, 355}, {278, 355}, {284, 368}, {282, 392}, {284, 415},
  {295, 434}, {315, 446}, {300, 454}, {278, 452}, {255, 454}
}
local m_trunk = poly(pts_trunk, true)
work(m_trunk, {hand="body", pile=p_bark_core, angle=1.5, coverage=2.5, fill=true, clip=true})

-- Deep longitudinal bark furrows
b_twig:load(p_bark_furrow, 0.95)
b_twig:stroke({{264, 362}, {262, 390}, {258, 418}, {250, 444}}, {pressure={0.8, 0.9, 0.8, 0.3}})
b_twig:stroke({{271, 360}, {270, 390}, {268, 418}, {264, 448}}, {pressure={0.8, 0.9, 0.8, 0.4}})
b_twig:stroke({{278, 365}, {278, 392}, {277, 418}, {280, 448}}, {pressure={0.8, 0.9, 0.8, 0.4}})
b_twig:stroke({{282, 380}, {283, 408}, {290, 432}, {306, 446}}, {pressure={0.7, 0.85, 0.8, 0.3}})

-- Raised weathered bark ridges
b_twig:load(p_bark_mid, 0.9)
b_twig:stroke({{266, 362}, {264, 390}, {260, 416}, {254, 442}}, {pressure={0.5, 0.75, 0.7, 0.2}})
b_twig:stroke({{274, 362}, {274, 390}, {272, 416}, {272, 446}}, {pressure={0.5, 0.75, 0.7, 0.2}})
b_twig:stroke({{280, 372}, {281, 400}, {284, 424}, {298, 444}}, {pressure={0.5, 0.75, 0.7, 0.2}})

-- Highlights along crest of bark ridges
b_rigger:load(p_bark_ridge, 0.85)
b_rigger:stroke({{266, 375}, {265, 400}, {261, 424}}, {pressure={0.3, 0.65, 0.2}})
b_rigger:stroke({{275, 375}, {275, 400}, {273, 424}}, {pressure={0.3, 0.65, 0.2}})

-- Ancient hollow cavity in trunk (x = 267 to 275, y = 396 to 424)
local m_hollow = poly({{267, 398}, {274, 396}, {276, 420}, {269, 422}}, true)
work(m_hollow, {hand="detail", pile=p_hollow_void, angle=1.5, coverage=2.2, fill=true, clip=true})

-- Wound-wood calluses framing the hollow
b_twig:load(p_woundwood, 0.9)
b_twig:stroke({{267, 396}, {266, 410}, {268, 424}}, {pressure={0.5, 0.8, 0.4}})
b_twig:stroke({{274, 395}, {276, 410}, {275, 422}}, {pressure={0.5, 0.8, 0.4}})

-- 4. The Great Living Eastern Bough (Arching over the Megalithic Tomb)
-- Smoothed tapered ribbon extending from (276, 358) across to (475, 328)
local pts_east_bough = {
  {276, 355}, {312, 346}, {355, 336}, {405, 328}, {450, 324}, {475, 326},
  {475, 332}, {450, 332}, {405, 338}, {355, 346}, {312, 356}, {280, 365}
}
local m_east_bough = poly(pts_east_bough, true)
work(m_east_bough, {hand="body", pile=p_bark_core, angle=0.15, coverage=2.4, fill=true, clip=true})

-- Bark light along crest of eastern bough
b_twig:load(p_bark_mid, 0.9)
b_twig:stroke({{278, 356}, {312, 348}, {355, 338}, {405, 330}, {450, 326}, {475, 328}},
  {pressure={0.7, 0.75, 0.7, 0.65, 0.5, 0.25}, ramps={0.02, 0.05}})

-- Swelling branch collar at trunk junction
b_block:load(p_bark_core, 0.95)
b_block:stroke({{274, 356}, {282, 362}, {280, 370}}, {pressure=0.9})

-- Secondary Branches off Eastern Bough:
-- E1 (above left of dolmen): arches upward into twilight
local pts_e1 = {
  {342, 340}, {360, 318}, {378, 292}, {395, 268},
  {399, 270}, {383, 294}, {365, 321}, {347, 342}
}
work(poly(pts_e1, true), {hand="body", pile=p_bark_core, angle=1.0, coverage=2.2, fill=true, clip=true})
b_twig:load(p_bark_mid, 0.85)
b_twig:stroke({{344, 339}, {362, 318}, {380, 292}, {397, 268}}, {pressure={0.6, 0.65, 0.45, 0.15}})

-- E2 (above right of dolmen): arches upward and eastward
local pts_e2 = {
  {408, 332}, {430, 308}, {452, 284}, {470, 264},
  {474, 266}, {457, 286}, {435, 311}, {413, 334}
}
work(poly(pts_e2, true), {hand="body", pile=p_bark_core, angle=0.8, coverage=2.2, fill=true, clip=true})
b_twig:load(p_bark_mid, 0.85)
b_twig:stroke({{410, 331}, {432, 308}, {454, 284}, {472, 264}}, {pressure={0.55, 0.6, 0.45, 0.15}})

-- E3 (drooping shelter east of dolmen):
b_branch:load(p_bark_core, 0.95)
b_branch:stroke({{450, 328}, {475, 338}, {498, 348}, {518, 355}}, {pressure={0.85, 0.7, 0.45, 0.15}, ramps={0.02, 0.1}})

-- 5. The Living Western Bough (Reaching out over descending left knoll)
local pts_west_bough = {
  {262, 356}, {225, 345}, {185, 332}, {145, 316}, {118, 300},
  {118, 308}, {145, 324}, {185, 340}, {225, 354}, {260, 366}
}
local m_west_bough = poly(pts_west_bough, true)
work(m_west_bough, {hand="body", pile=p_bark_core, angle=-0.15, coverage=2.4, fill=true, clip=true})

b_twig:load(p_bark_mid, 0.9)
b_twig:stroke({{260, 357}, {225, 346}, {185, 333}, {145, 317}, {118, 302}},
  {pressure={0.7, 0.75, 0.65, 0.5, 0.2}, ramps={0.02, 0.05}})

-- Western collar
b_block:load(p_bark_core, 0.95)
b_block:stroke({{264, 356}, {258, 362}, {260, 370}}, {pressure=0.9})

-- Secondary Western Limbs:
-- W1 (upward reaching toward zenith west)
local pts_w1 = {
  {188, 335}, {172, 310}, {155, 280}, {140, 252},
  {144, 250}, {160, 278}, {177, 308}, {193, 333}
}
work(poly(pts_w1, true), {hand="body", pile=p_bark_core, angle=-1.1, coverage=2.2, fill=true, clip=true})
b_twig:load(p_bark_mid, 0.85)
b_twig:stroke({{190, 334}, {174, 309}, {157, 279}, {142, 251}}, {pressure={0.55, 0.6, 0.45, 0.15}})

-- W2 (lower western droop)
b_branch:load(p_bark_core, 0.95)
b_branch:stroke({{145, 320}, {125, 338}, {104, 352}}, {pressure={0.8, 0.55, 0.15}, ramps={0.02, 0.1}})

-- 6. The Bleached Stag-Head Crown Aloft (Spearing into the Cobalt Zenith)
-- Central dead spire (rising up to y = 125!)
local pts_stag_center = {
  {266, 358}, {264, 315}, {262, 268}, {259, 218}, {256, 168}, {253, 126},
  {255, 126}, {259, 168}, {263, 218}, {267, 268}, {270, 315}, {272, 358}
}
local m_stag_center = poly(pts_stag_center, true)
work(m_stag_center, {hand="body", pile=p_stag_dark, angle=1.55, coverage=2.4, fill=true, clip=true})

-- Bleached bone-gray highlights along dead trunk
b_twig:load(p_stag_bleach, 0.95)
b_twig:stroke({{268, 355}, {266, 310}, {263, 265}, {260, 215}, {257, 165}, {254, 128}},
  {pressure={0.5, 0.75, 0.7, 0.65, 0.45, 0.1}, ramps={0.02, 0.05}})

-- Splintered tips at summit
b_rigger:load(p_stag_tip, 1.0)
b_rigger:stroke({{254, 130}, {253, 118}}, {pressure={0.45, 0.05}})
b_rigger:stroke({{254, 130}, {257, 120}}, {pressure={0.35, 0.05}})
b_rigger:stroke({{254, 130}, {250, 122}}, {pressure={0.3, 0.05}})

-- Antler Branch S1 (East Stag Antler, reaching toward the celestial star & moon)
local pts_antler_e = {
  {266, 292}, {284, 255}, {304, 216}, {320, 178}, {332, 140},
  {335, 141}, {323, 180}, {307, 218}, {288, 257}, {270, 294}
}
work(poly(pts_antler_e, true), {hand="detail", pile=p_stag_dark, angle=0.9, coverage=2.2, fill=true, clip=true})
b_twig:load(p_stag_bleach, 0.95)
b_twig:stroke({{268, 292}, {286, 255}, {306, 216}, {322, 178}, {334, 139}},
  {pressure={0.5, 0.7, 0.6, 0.4, 0.1}, ramps={0.02, 0.05}})
b_rigger:load(p_stag_tip, 1.0)
b_rigger:stroke({{334, 141}, {338, 128}}, {pressure={0.4, 0.05}})
b_rigger:stroke({{334, 141}, {332, 130}}, {pressure={0.3, 0.05}})

-- Antler Branch S2 (West Stag Antler)
local pts_antler_w = {
  {263, 282}, {245, 245}, {226, 206}, {208, 168}, {194, 132},
  {197, 131}, {211, 167}, {229, 204}, {248, 243}, {266, 280}
}
work(poly(pts_antler_w, true), {hand="detail", pile=p_stag_dark, angle=-0.9, coverage=2.2, fill=true, clip=true})
b_twig:load(p_stag_bleach, 0.95)
b_twig:stroke({{264, 281}, {246, 244}, {227, 205}, {209, 167}, {195, 130}},
  {pressure={0.5, 0.7, 0.6, 0.4, 0.1}, ramps={0.02, 0.05}})
b_rigger:load(p_stag_tip, 1.0)
b_rigger:stroke({{195, 132}, {192, 122}}, {pressure={0.4, 0.05}})
b_rigger:stroke({{195, 132}, {197, 124}}, {pressure={0.3, 0.05}})

-- 7. Sympodial Crooked Oak Twigs
local oak_twigs = {
  -- Off E1
  {{397, 268}, {408, 250}, {418, 234}},
  {{397, 268}, {390, 248}, {384, 232}},
  {{380, 292}, {370, 272}, {362, 254}},
  {{362, 318}, {352, 298}, {346, 282}},
  -- Off E2
  {{472, 264}, {484, 246}, {496, 230}},
  {{472, 264}, {464, 244}, {458, 226}},
  {{454, 284}, {466, 266}, {476, 250}},
  -- Off E3
  {{498, 348}, {514, 360}, {528, 370}},
  {{518, 355}, {534, 352}, {550, 348}},
  -- Off W1
  {{142, 251}, {130, 232}, {120, 216}},
  {{142, 251}, {150, 230}, {156, 212}},
  {{157, 279}, {146, 260}, {136, 242}},
  {{174, 309}, {164, 290}, {156, 272}},
  -- Off W2
  {{125, 338}, {110, 352}, {96, 364}},
  {{104, 352}, {90, 346}, {76, 340}}
}
for _, tw in ipairs(oak_twigs) do
  b_twig:load(p_bark_core, 0.9)
  b_twig:stroke(tw, {pressure={0.75, 0.2}, ramps={0.02, 0.15}})
end

-- Fine outer sympodial twigs
local fine_twigs = {
  {{418, 234}, {426, 222}, {432, 212}},
  {{384, 232}, {378, 218}, {374, 206}},
  {{496, 230}, {506, 218}, {514, 206}},
  {{476, 250}, {486, 236}, {494, 224}},
  {{120, 216}, {112, 204}, {106, 194}},
  {{136, 242}, {126, 228}, {118, 216}}
}
for _, ftw in ipairs(fine_twigs) do
  b_rigger:load(p_bark_core, 0.85)
  b_rigger:stroke(ftw, {pressure={0.65, 0.1}, ramps={0.02, 0.15}})
end

-- 8. Ragged Marcescent Late-Autumn Oak Foliage
local b_leaf = brush{kind="round", width=1.8, point=1, stiffness=0.85}
local leaf_clusters = {
  -- Along eastern living limbs
  {405, 252, 14}, {385, 278, 12}, {362, 308, 14}, {348, 292, 10},
  {468, 272, 14}, {452, 296, 12}, {492, 238, 12}, {478, 258, 10},
  {508, 356, 12}, {528, 362, 10}, {438, 336, 12}, {468, 340, 12},
  -- Along western living limbs
  {178, 320, 14}, {158, 292, 12}, {142, 262, 14}, {128, 238, 12},
  {118, 342, 12}, {98, 356, 10}, {168, 336, 12}, {132, 318, 12}
}
for _, cl in ipairs(leaf_clusters) do
  local cx, cy, n = cl[1], cl[2], cl[3]
  for i = 1, n do
    local ox = rand(-10, 10)
    local oy = rand(-8, 8)
    local p = (rand(0, 1) < 0.6) and p_leaf_russet or p_leaf_gold
    b_leaf:load(p, 0.75)
    b_leaf:touch(cx + ox, cy + oy, {pressure=rand(0.35, 0.7), drag={rand(1.2, 2.2), rand(0.8, 1.6)}})
  end
end

-- Gilt twilight rim highlights on outer leaf edges
for _, cl in ipairs(leaf_clusters) do
  local cx, cy = cl[1], cl[2]
  for i = 1, 4 do
    local ox = rand(-8, 8)
    local oy = rand(-6, 6)
    b_rigger:load(p_leaf_rim, 0.7)
    b_rigger:touch(cx + ox, cy + oy, {pressure=rand(0.25, 0.5), drag={1.0, 1.2}})
  end
end

-- 9. The Windswept Companion Tree (Right knoll, x ≈ 760, y ≈ 436)
local pts_comp_trunk = {
  {766, 442}, {762, 405}, {756, 365}, {750, 325}, {746, 285}, {744, 252},
  {748, 252}, {752, 285}, {758, 325}, {764, 365}, {770, 405}, {774, 442}
}
work(poly(pts_comp_trunk, true), {hand="body", pile=p_bark_core, angle=1.4, coverage=2.4, fill=true, clip=true})
b_twig:load(p_bark_mid, 0.9)
b_twig:stroke({{766, 440}, {762, 405}, {756, 365}, {750, 325}, {746, 285}, {744, 252}},
  {pressure={0.75, 0.7, 0.6, 0.45, 0.3, 0.1}, ramps={0.02, 0.05}})

-- Windswept branches leaning with Baltic gales
local comp_branches = {
  {{750, 325}, {774, 308}, {802, 290}, {828, 275}, {852, 262}},
  {{802, 290}, {818, 268}, {832, 245}, {842, 222}},
  {{746, 285}, {762, 260}, {776, 235}, {788, 210}},
  {{744, 252}, {754, 228}, {764, 202}, {772, 178}}
}
for _, b in ipairs(comp_branches) do
  b_twig:load(p_bark_core, 0.9)
  b_twig:stroke(b, {pressure={0.75, 0.2}, ramps={0.02, 0.1}})
end

local comp_twigs = {
  {{852, 262}, {868, 254}, {882, 248}},
  {{842, 222}, {854, 206}, {864, 192}},
  {{788, 210}, {800, 194}, {810, 178}}
}
for _, tw in ipairs(comp_twigs) do
  b_rigger:load(p_bark_core, 0.85)
  b_rigger:stroke(tw, {pressure={0.65, 0.12}, ramps={0.02, 0.15}})
end

-- Sparse clinging leaves on companion tree
local comp_leaf_sprays = {
  {810, 300, 12}, {840, 280, 14}, {865, 265, 12}, {828, 250, 12},
  {775, 240, 12}, {785, 210, 10}
}
for _, cl in ipairs(comp_leaf_sprays) do
  local cx, cy, n = cl[1], cl[2], cl[3]
  for i = 1, n do
    local ox = rand(-10, 10)
    local oy = rand(-8, 8)
    local p = (rand(0, 1) < 0.5) and p_leaf_russet or p_leaf_gold
    b_leaf:load(p, 0.7)
    b_leaf:touch(cx + ox, cy + oy, {pressure=rand(0.3, 0.65), drag={rand(1.0, 2.0), 1.4}})
  end
end

-- 10. Windswept Bent-Grass Tufts & Dried Carline Thistles along Crest
local function draw_curved_grass(bx, by, len, ang, bend, p)
  local mx = bx + len * 0.55 * math.cos(ang + bend * 0.5)
  local my = by - len * 0.55 * math.sin(ang + bend * 0.5)
  local tx = bx + len * math.cos(ang + bend)
  local ty = by - len * math.sin(ang + bend)
  b_rigger:load(p, 0.75)
  b_rigger:stroke({{bx, by}, {mx, my}, {tx, ty}}, {pressure={0.6, 0.04}, ramps={0.05, 0.3}})
end

local function draw_grass_cluster(cx, cy, n, h, lean)
  for i = 1, n do
    local bx = cx + rand(-5, 5)
    local by = cy + rand(-2, 2)
    local len = h * rand(0.75, 1.25)
    local ang = 1.57 + rand(-0.25, 0.25) + lean * 0.3
    local bnd = lean * rand(0.2, 0.5)
    local p = (rand(0, 1) < 0.65) and p_straw_sharp or p_straw_shade
    draw_curved_grass(bx, by, len, ang, bnd, p)
  end
end

local crest_grasses = {
  {160, 448, 8, 16, -0.35}, {210, 438, 7, 14, -0.3},
  {310, 416, 7, 13, -0.2},  {435, 415, 7, 13, 0.2},
  {480, 422, 8, 15, 0.25},  {530, 432, 8, 15, 0.3},
  {630, 448, 7, 14, 0.3},   {720, 440, 8, 16, 0.35},
  {810, 436, 8, 15, 0.35},  {890, 446, 7, 14, 0.4}
}
for _, g in ipairs(crest_grasses) do
  draw_grass_cluster(g[1], g[2], g[3], g[4], g[5])
end

-- Dried carline thistle skeletons (Carlina vulgaris) silhouetted against glowing mist
local thistles = {
  {180, 448, 18}, {230, 435, 16}, {510, 428, 17}, {610, 448, 18}, {850, 440, 16}
}
for _, th in ipairs(thistles) do
  local x, y, h = th[1], th[2], th[3]
  b_rigger:load(p_straw_shade, 0.8)
  b_rigger:stroke({{x, y}, {x, y - h*0.6}, {x + rand(-2, 2), y - h}}, {pressure={0.65, 0.15}, ramps={0.05, 0.2}})
  b_rigger:load(p_straw_sharp, 0.85)
  b_rigger:stroke({{x - 3, y - h}, {x + 3, y - h}}, {pressure=0.4})
  b_rigger:stroke({{x, y - h - 2}, {x, y - h + 2}}, {pressure=0.4})
end

print("Ancient Veteran Oak, Companion Tree, and Crest Heathland Accents painted.")

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
