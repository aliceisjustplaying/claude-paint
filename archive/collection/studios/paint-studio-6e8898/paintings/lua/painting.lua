-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1

canvas{
  size = 700,
  aspect = 0.83,
  linen = {18, 15},
  seed = 7,
  ground = {
    {pile = {{"raw umber", 3}, {"yellow ochre", 2}, {"lead white", 4}}, um = 100, apply = "knife", texture = 0.3},
    {pile = {{"raw umber", 1}, {"red earth", 1}, {"lead white", 2}}, um = 40, apply = "brush"}
  }
}
print("W="..W.." H="..H)

--@ chunk 2

-- underdrawing: construction sketch for the head, loose black chalk on the toned ground
local c = chalk()

local headpts = {
  460,140,  -- crown
  560,160,  -- upper right cranium
  590,235,  -- right temple
  560,340,  -- right cheekbone (near/narrow side)
  520,430,  -- right jaw
  480,520,  -- chin
  365,460,  -- left jaw
  320,340,  -- left cheekbone (far/broad side)
  330,225,  -- left temple
  365,160,  -- upper left cranium
  460,140   -- back to crown, closes the loop
}
c:sketch(headpts, {pressure = 0.25, passes = 3, wander = 4})

-- center facial guide line, a little bowed toward the near (right) side
c:sketch({470,150, 476,260, 478,340, 480,407, 480,445, 480,520}, {pressure=0.2, passes=2, wander=2})

-- horizontal thirds
c:rule({350,293},{560,293},{pressure=0.18})  -- brow line
c:rule({345,407},{555,407},{pressure=0.18})  -- nose base
c:rule({365,445},{535,445},{pressure=0.18})  -- mouth line

print(c:width())

--@ chunk 3

-- a generous rough silhouette (head + hair + shoulders), oversized on purpose:
-- the real edges will be laid by the figure's own paint later, this just keeps
-- the background brush off the area where the figure will go.
figure_reserve = poly({
  420,75,  600,85,  655,170,  650,300,  615,400,  585,470,  545,545,
  520,600,
  700,635,  900,700,  1000,780,  1000,1205,
  0,1205,  0,780,  95,700,  300,600,
  270,470,  260,340,  270,200,  330,100
})

bg_pile = pile{{"raw umber", 4}, {"bone black", 3}, {"yellow ochre", 1}, {"Prussian blue", 1}, {"lead white", 1}, medium = 0.15}
print(bg_pile)

work(-figure_reserve, {hand="broad", pile=bg_pile, coverage=2.6, angle=function(x,y) return 0.3 + 0.15*math.sin(x*0.006+y*0.004) end, length={140,260}})

--@ chunk 4

work(-figure_reserve, {hand="broad", pile=bg_pile, coverage=2.2, fill=true,
     angle=function(x,y) return -0.2 + 0.2*math.cos(x*0.005 - y*0.006) end, length={120,220}})

--@ chunk 5

skin_mask = poly({
  345,225,  470,190,  585,230,
  560,340,  520,430,  505,480,  525,570,
  470,620,
  410,565,  385,475,  365,460,
  320,340
}, true)

hair_blob = poly({
  255,470,  240,300,  280,160,  400,80,  500,70,  610,90,
  660,220,  650,360,  600,460,  560,500,
  480,540,  400,520,  340,540
}, true)

hair_mask = hair_blob - skin_mask

print("skin area", skin_mask:area(), "hair area", hair_mask:area())

--@ chunk 6

hair_pile = pile{{"raw umber", 3}, {"bone black", 2}, {"yellow ochre", 1}, medium = 0.12}
work(hair_mask, {hand="body", pile=hair_pile, coverage=2.4, fill=true,
     angle=function(x,y) return 1.3 + 0.5*math.sin((x-470)*0.01) end, length={60,140}})

--@ chunk 7

work(hair_mask, {hand="body", pile=hair_pile, coverage=2.0, fill=true,
     angle=function(x,y) return 1.55 + 0.35*math.sin((x-470)*0.006) end, length={110,220},
     pressure={0.55,0.75}})
blend(hair_mask, {angle=1.5})

--@ chunk 8

work(hair_mask, {hand="broad", pile=hair_pile, coverage=2.2, fill=true,
     angle=function(x,y) return 1.55 + 0.25*math.sin((x-470)*0.005) end, length={160,260},
     pressure={0.6,0.8}})
blend(hair_mask, {angle=1.5})

--@ chunk 9

work(hair_mask, {hand="body", pile=hair_pile, coverage=2.2, clip=true, fill=true,
     angle=function(x,y) return 1.55 + 0.2*math.sin((x-470)*0.005) end, length={90,160},
     pressure={0.6,0.85}})
blend(hair_mask, {angle=1.5})

--@ chunk 10

garment_mask = figure_reserve - skin_mask - hair_mask
print("garment area", garment_mask:area())
garment_pile = pile{{"bone black", 3}, {"raw umber", 2}, {"Prussian blue", 1}, medium = 0.12}
work(garment_mask, {hand="broad", pile=garment_pile, coverage=2.4, fill=true,
     angle=function(x,y) return 1.2 + 0.3*math.sin(x*0.004) end, length={150,260}})

--@ chunk 11

work(hair_mask, {hand="body", pile=hair_pile, coverage=2.8, clip=true, fill=true,
     angle=function(x,y) return 1.55 + 0.2*math.sin((x-470)*0.005) end, length={70,130},
     pressure={0.6,0.85}})
blend(hair_mask, {angle=1.5})

--@ chunk 12

hair_pile2 = pile{{"bone black", 4}, {"raw umber", 2}, {"yellow ochre", 0.5}, medium = 0.05}
work(hair_mask, {hand="body", pile=hair_pile2, coverage=3.2, clip=true, fill=true,
     angle=function(x,y) return 1.55 + 0.2*math.sin((x-470)*0.005) end, length={70,130},
     pressure={0.75,0.95}})

--@ chunk 13

work(-figure_reserve, {hand="body", pile=bg_pile, coverage=2.6, clip=true, fill=true,
     angle=function(x,y) return 0.3 + 0.15*math.sin(x*0.006+y*0.004) end, length={90,160},
     pressure={0.7,0.9}})

--@ chunk 14

hair_pile3 = pile{{"bone black", 3}, {"red earth", 2}, {"raw umber", 2}, medium = 0.05}
work(hair_mask, {hand="body", pile=hair_pile3, coverage=2.6, clip=true, fill=true,
     angle=function(x,y) return 1.55 + 0.2*math.sin((x-470)*0.005) end, length={70,130},
     pressure={0.75,0.95}})
blend(hair_mask, {angle=1.5})

--@ chunk 15

print(drying(400, 200))
print(drying(470, 550))
print(t or "no t")

--@ chunk 16

flesh_base = pile{{"lead white", 4}, {"yellow ochre", 2}, {"red earth", 1.5}, {"vermilion", 0.5}, medium = 0.2}
work(skin_mask, {hand="body", pile=flesh_base, coverage=1.8, clip=true, fill=true,
     angle=function(x,y) return 0.9 + 0.3*math.sin(y*0.01) end, length={30,70},
     pressure={0.5,0.7}})

--@ chunk 17

local term_pts = {
  {200,500}, {293,490}, {340,480}, {407,472}, {445,470}, {520,478}, {600,480}
}
function terminator_x(y)
  for i=1,#term_pts-1 do
    local y0,x0 = term_pts[i][1], term_pts[i][2]
    local y1,x1 = term_pts[i+1][1], term_pts[i+1][2]
    if y >= y0 and y <= y1 then
      return lerp(x0, x1, (y-y0)/(y1-y0))
    end
  end
  if y < term_pts[1][1] then return term_pts[1][2] end
  return term_pts[#term_pts][2]
end

function shadow_strength(x, y)
  local tx = terminator_x(y)
  return smoothstep(tx - 10, tx + 55, x)
end

shadow_mask = mask(function(x,y) return shadow_strength(x,y) end) * skin_mask
light_mask = mask(function(x,y) return 1 - shadow_strength(x,y) end) * skin_mask
print("shadow area", shadow_mask:area(), "light area", light_mask:area())

--@ chunk 18

shadow_flesh = pile{{"lead white", 2}, {"raw umber", 2}, {"red earth", 1}, {"green earth", 0.5}, medium = 0.2}
light_flesh = pile{{"lead white", 5}, {"yellow ochre", 1.5}, {"vermilion", 1}, {"red earth", 0.5}, medium = 0.15}

work(shadow_mask, {hand="body", pile=shadow_flesh, coverage=1.6, clip=true, fill=true,
     angle=function(x,y) return 0.3 + 0.4*math.sin(y*0.012) end, length={25,55}, pressure={0.45,0.65}})

work(light_mask, {hand="body", pile=light_flesh, coverage=1.6, clip=true, fill=true,
     angle=function(x,y) return 0.3 + 0.4*math.sin(y*0.012) end, length={25,55}, pressure={0.45,0.65}})

--@ chunk 19

shadow_flesh2 = pile{{"lead white", 1}, {"raw umber", 3}, {"red earth", 1}, {"green earth", 0.5}, medium = 0.12}
work(shadow_mask, {hand="body", pile=shadow_flesh2, coverage=2.4, clip=true, fill=true,
     angle=function(x,y) return 0.3 + 0.4*math.sin(y*0.012) end, length={25,55}, pressure={0.55,0.75}})

--@ chunk 20

function shadow_strength2(x, y)
  local tx = terminator_x(y)
  return smoothstep(tx - 8, tx + 20, x)
end
shadow_mask2 = mask(function(x,y) return shadow_strength2(x,y) end) * skin_mask

shadow_flesh3 = pile{{"raw umber", 3}, {"red earth", 1}, {"bone black", 0.5}, {"lead white", 0.5}, medium = 0.1}
work(shadow_mask2, {hand="body", pile=shadow_flesh3, coverage=2.6, clip=true, fill=true,
     angle=function(x,y) return 0.3 + 0.4*math.sin(y*0.012) end, length={25,55}, pressure={0.65,0.85}})

--@ chunk 21

print(wait(240))

--@ chunk 22

print("shadow zone:", drying(560,340))
print("light zone:", drying(360,340))
print("hair zone:", drying(400,200))
print("bg zone:", drying(150,150))
print("garment zone:", drying(470,700))

--@ chunk 23

cavity_pile = pile{{"raw umber", 3}, {"bone black", 1}, {"green earth", 0.5}, {"red earth", 0.5}, medium = 0.15}

brow_l = ellipse(398, 288, 46, 16)
brow_r = ellipse(516, 290, 30, 13)
nose_side = ribbon({{478,300},{475,350},{472,400}}, {12,16,20})
undernose = ellipse(478,411,28,9)
nostril_l = ellipse(463,406,7,4)
nostril_r = ellipse(492,404,6,4)
nasolabial = ribbon({{494,402},{510,425},{520,446}}, 7)
underlip = ellipse(482,463,33,8)
jaw_shadow = ribbon({{560,340},{535,400},{505,455},{480,510}}, {10,13,13,10})
chin_neck = ellipse(482,538,46,18)

cavity_mask = (brow_l + brow_r + nose_side + undernose + nostril_l + nostril_r + nasolabial + underlip + jaw_shadow + chin_neck) * skin_mask

work(cavity_mask, {hand="detail", pile=cavity_pile, coverage=2.0, tool="round 5",
     pressure={0.4,0.6}, length={8,16}})

--@ chunk 24

-- knock back the oversized brow/nose blobs with flesh tone + blend
fix_mask = (brow_l + brow_r + nose_side + undernose) * skin_mask
work(fix_mask * light_mask, {hand="body", pile=light_flesh, coverage=2.2, clip=true, fill=true,
     angle=0.3, length={20,40}, pressure={0.5,0.7}})
work(fix_mask * shadow_mask2, {hand="body", pile=shadow_flesh3, coverage=1.6, clip=true, fill=true,
     angle=0.3, length={20,40}, pressure={0.4,0.55}})
blend(fix_mask:grow(6), {angle=0.4})

--@ chunk 25

brow_pile = pile{{"raw umber", 2}, {"bone black", 2}, medium = 0.1}

brow_l_shape = ribbon({365,279, 385,268, 410,264, 435,270}, {4,7.5,7,3})
brow_r_shape = ribbon({497,283, 512,275, 528,273, 542,278}, {3,6,5.5,2.5})

work(brow_l_shape, {hand="detail", pile=brow_pile, coverage=2.2, clip=true, tool="round 2.5", length={6,12}, pressure={0.5,0.7}})
work(brow_r_shape, {hand="detail", pile=brow_pile, coverage=2.2, clip=true, tool="round 2.2", length={5,10}, pressure={0.45,0.6}})

--@ chunk 26

sclera_pile = pile{{"lead white", 3}, {"yellow ochre", 0.3}, {"red earth", 0.1}, medium = 0.3}
iris_pile = pile{{"yellow ochre", 1.5}, {"raw umber", 1}, {"red earth", 0.3}, {"lead white", 0.3}, medium = 0.2}
pupil_pile = pile{{"bone black", 1}, medium = 0.1}
lid_pile = pile{{"raw umber", 2}, {"bone black", 1.5}, medium = 0.1}

-- LEFT eye (broad, lit)
eye_l = poly({372,304, 398,294, 428,301, 398,311}, true)
work(eye_l, {hand="detail", pile=sclera_pile, coverage=2.0, clip=true, tool="round 3", length={6,10}, pressure={0.4,0.55}})

iris_l = ellipse(399, 302, 10, 10)
work(iris_l, {hand="detail", pile=iris_pile, coverage=2.4, clip=true, tool="round 3", length={4,8}, pressure={0.55,0.7}})
pupil_l = ellipse(399, 302, 4.2, 4.2)
work(pupil_l, {hand="detail", pile=pupil_pile, coverage=2.4, clip=true, tool="round 2", length={3,5}, pressure={0.6,0.75}})

-- lid lines
lid_l_top = ribbon({372,304, 398,293, 428,300.5}, {1.6,2.4,1.4})
work(lid_l_top, {hand="detail", pile=lid_pile, coverage=2.2, clip=true, tool="round 1.6", length={4,8}, pressure={0.5,0.65}})
lid_l_bottom = ribbon({376,308, 398,311, 424,304}, {1,1.6,1})
work(lid_l_bottom, {hand="detail", pile=shadow_flesh3, coverage=1.6, clip=true, tool="round 1.6", length={4,8}, pressure={0.3,0.4}})

--@ chunk 27

blend(eye_l:grow(3), {angle=0.2})

catchlight_l = ellipse(396.5, 299, 1.6, 1.6)
work(catchlight_l, {hand="detail", pile=pile{{"lead white",1},medium=0.1}, coverage=2.5, clip=true, tool="round 1.2", length={2,3}, pressure={0.55,0.65}})

corner_in_l = ellipse(374,304,3,2.2)
corner_out_l = ellipse(426,301,3,2.2)
work(corner_in_l+corner_out_l, {hand="detail", pile=shadow_flesh3, coverage=1.8, clip=true, tool="round 1.4", length={3,5}, pressure={0.35,0.45}})

--@ chunk 28

work(iris_l, {hand="detail", pile=iris_pile, coverage=1.6, clip=true, tool="round 3", length={4,7}, pressure={0.5,0.6}})
work(pupil_l, {hand="detail", pile=pupil_pile, coverage=2.2, clip=true, tool="round 2", length={3,5}, pressure={0.6,0.72}})
work(catchlight_l, {hand="detail", pile=pile{{"lead white",1},medium=0.1}, coverage=2.2, clip=true, tool="round 1.2", length={2,3}, pressure={0.55,0.65}})
work(lid_l_top, {hand="detail", pile=lid_pile, coverage=1.6, clip=true, tool="round 1.6", length={4,8}, pressure={0.45,0.6}})

--@ chunk 29

-- RIGHT eye (narrow, foreshortened, shadow side)
eye_r = poly({497,308, 516,299, 537,304, 516,313}, true)
sclera_r_pile = pile{{"lead white", 2.5}, {"yellow ochre", 0.3}, {"raw umber", 0.3}, medium = 0.28}
work(eye_r, {hand="detail", pile=sclera_r_pile, coverage=2.4, clip=true, tool="round 2.4", length={4,8}, pressure={0.4,0.55}})

iris_r_pile = pile{{"yellow ochre", 1}, {"raw umber", 1.3}, {"green earth", 0.3}, {"lead white", 0.2}, medium = 0.2}
iris_r = ellipse(517, 306, 8, 8.5)
work(iris_r, {hand="detail", pile=iris_r_pile, coverage=2.0, clip=true, tool="round 2.4", length={4,7}, pressure={0.5,0.62}})
pupil_r = ellipse(517, 306, 3.4, 3.6)
work(pupil_r, {hand="detail", pile=pupil_pile, coverage=2.2, clip=true, tool="round 1.6", length={3,5}, pressure={0.6,0.7}})

catchlight_r = ellipse(514.5, 303, 1.2, 1.2)
work(catchlight_r, {hand="detail", pile=pile{{"lead white",1},medium=0.1}, coverage=1.8, clip=true, tool="round 1", length={2,3}, pressure={0.45,0.55}})

lid_r_top = ribbon({497,308.5, 516,298, 537,303.5}, {1.2,1.8,1.1})
work(lid_r_top, {hand="detail", pile=lid_pile, coverage=2.0, clip=true, tool="round 1.3", length={3,6}, pressure={0.4,0.55}})
lid_r_bottom = ribbon({500,311, 516,313, 534,307}, {0.8,1.3,0.8})
work(lid_r_bottom, {hand="detail", pile=shadow_flesh3, coverage=1.6, clip=true, tool="round 1.2", length={3,6}, pressure={0.3,0.4}})

corner_in_r = ellipse(499,308,2.5,1.8)
corner_out_r = ellipse(535,304,2.5,1.8)
work(corner_in_r+corner_out_r, {hand="detail", pile=shadow_flesh3, coverage=1.8, clip=true, tool="round 1.2", length={3,5}, pressure={0.35,0.45}})

--@ chunk 30

socket_edge_l = brow_l:rim(10, 6) * skin_mask
stipple(socket_edge_l, {pile=light_flesh, width=10, coverage=0.9, pressure={0.25,0.4}, cluster=0.4})
blend(brow_l:grow(8), {angle=0.3})

work(iris_l, {hand="detail", pile=iris_pile, coverage=1.2, clip=true, tool="round 2.6", length={4,6}, pressure={0.45,0.55}})
work(pupil_l, {hand="detail", pile=pupil_pile, coverage=1.6, clip=true, tool="round 1.8", length={3,4}, pressure={0.55,0.65}})
work(lid_l_top, {hand="detail", pile=lid_pile, coverage=1.2, clip=true, tool="round 1.5", length={4,7}, pressure={0.4,0.5}})

--@ chunk 31

cover_l = (brow_l - eye_l:grow(3)) * skin_mask
work(cover_l, {hand="body", pile=light_flesh, coverage=3.0, clip=true, fill=true,
     angle=0.3, length={15,30}, pressure={0.6,0.8}})
work(cover_l, {hand="body", pile=light_flesh, coverage=2.0, clip=true, fill=true,
     angle=0.9, length={15,30}, pressure={0.6,0.8}})

--@ chunk 32

repaint_l = ellipse(398,300,75,48) * skin_mask * light_mask - eye_l:grow(2) - brow_l_shape:grow(1.5)
work(repaint_l, {hand="body", pile=light_flesh, coverage=2.4, clip=true, fill=true,
     angle=function(x,y) return 0.2+0.5*math.sin(x*0.03+y*0.02) end, length={12,26}, pressure={0.45,0.65}})

--@ chunk 33

nose_highlight_pile = pile{{"lead white", 5}, {"yellow ochre", 1}, {"vermilion", 0.3}, medium = 0.15}
nose_hl = ribbon({470,298, 471,335, 473,370, 475,396}, {5,6,6,5})
work(nose_hl, {hand="detail", pile=nose_highlight_pile, coverage=2.0, clip=true, tool="round 3", length={10,20}, pressure={0.4,0.55}})

nose_shadow_line = ribbon({484,300, 486,335, 488,372, 487,398}, {3.5,4.5,4.5,3.5})
work(nose_shadow_line, {hand="detail", pile=shadow_flesh3, coverage=1.8, clip=true, tool="round 2.2", length={10,18}, pressure={0.4,0.55}})

-- reinforce nostrils and undernose
work(nostril_l+nostril_r, {hand="detail", pile=cavity_pile, coverage=2.0, clip=true, tool="round 1.6", length={4,6}, pressure={0.5,0.65}})

-- tip highlight
tip_hl = ellipse(478,394,6,4)
work(tip_hl, {hand="detail", pile=nose_highlight_pile, coverage=1.8, clip=true, tool="round 2.6", length={5,9}, pressure={0.35,0.5}})

blend((nose_hl+nose_shadow_line):grow(2), {angle=1.4})

--@ chunk 34

upper_lip = poly({438,446, 458,436, 479,433, 498,437, 517,443, 500,448, 479,450, 460,449}, true)
lower_lip = poly({438,446, 460,449, 479,450, 500,448, 517,443, 515,447, 500,459, 479,463, 460,460, 440,449}, true)

upper_lip_pile = pile{{"red earth", 1.5}, {"raw umber", 0.5}, {"vermilion", 0.5}, {"lead white", 0.5}, medium = 0.15}
lower_lip_pile = pile{{"vermilion", 1.5}, {"red earth", 0.5}, {"lead white", 1}, medium = 0.15}
seam_pile = pile{{"raw umber", 1.5}, {"bone black", 0.5}, {"red earth", 0.5}, medium = 0.1}

work(upper_lip, {hand="detail", pile=upper_lip_pile, coverage=2.2, clip=true, tool="round 2.6", length={8,16}, pressure={0.4,0.55}})
work(lower_lip, {hand="detail", pile=lower_lip_pile, coverage=2.2, clip=true, tool="round 2.8", length={8,16}, pressure={0.45,0.6}})

seam_line = ribbon({438,446, 460,449, 479,450, 500,448, 517,443}, {1.2,1.8,2.0,1.6,1.0})
work(seam_line, {hand="detail", pile=seam_pile, coverage=2.2, clip=true, tool="round 1.4", length={5,10}, pressure={0.5,0.65}})

lip_hl = ellipse(475,456,14,4)
work(lip_hl, {hand="detail", pile=pile{{"lead white",2},{"vermilion",0.3},medium=0.2}, coverage=1.6, clip=true, tool="round 2.4", length={6,10}, pressure={0.3,0.4}})

corner_shadow = ellipse(438,446,4,3) + ellipse(517,443,4,3)
work(corner_shadow, {hand="detail", pile=seam_pile, coverage=1.8, clip=true, tool="round 1.4", length={3,5}, pressure={0.4,0.5}})

--@ chunk 35

ear_outer = poly({555,300, 572,295, 583,312, 580,347, 569,373, 557,368, 549,341, 551,315}, true)
ear_pile = pile{{"red earth", 1}, {"lead white", 1.5}, {"raw umber", 1}, medium = 0.15}
work(ear_outer, {hand="detail", pile=ear_pile, coverage=2.6, clip=true, tool="round 4", length={10,20}, pressure={0.55,0.7}})

ear_inner = ellipse(566,336,7,17)
work(ear_inner, {hand="detail", pile=cavity_pile, coverage=2.0, clip=true, tool="round 2.4", length={6,12}, pressure={0.45,0.6}})

ear_rim_hl = ribbon({558,303, 574,298, 582,314, 578,345}, {1.6,1.8,1.8,1.4})
work(ear_rim_hl, {hand="detail", pile=light_flesh, coverage=1.8, clip=true, tool="round 1.6", length={6,12}, pressure={0.35,0.5}})

lobe_hl = ellipse(566,368,5,5)
work(lobe_hl, {hand="detail", pile=ear_pile, coverage=1.6, clip=true, tool="round 2.2", length={5,8}, pressure={0.4,0.5}})

--@ chunk 36

neck_wide = poly({
  335,465,  355,555,  325,650,  470,700,  625,650,  595,555,  568,455
}, true)

skin_mask2 = skin_mask + neck_wide
new_area = neck_wide - skin_mask

new_shadow = mask(function(x,y) return shadow_strength(x,y) end) * new_area
new_light  = mask(function(x,y) return 1 - shadow_strength(x,y) end) * new_area

work(new_light, {hand="body", pile=light_flesh, coverage=2.2, clip=true, fill=true,
     angle=function(x,y) return 1.4 + 0.2*math.sin(x*0.01) end, length={20,45}, pressure={0.5,0.7}})
work(new_shadow, {hand="body", pile=shadow_flesh3, coverage=2.2, clip=true, fill=true,
     angle=function(x,y) return 1.4 + 0.2*math.sin(x*0.01) end, length={20,45}, pressure={0.5,0.7}})

--@ chunk 37

work(new_light, {hand="body", pile=light_flesh, coverage=1.6, clip=true, fill=true,
     angle=1.4, length={20,40}, pressure={0.45,0.6}})
work(new_shadow, {hand="body", pile=shadow_flesh3, coverage=1.6, clip=true, fill=true,
     angle=1.4, length={20,40}, pressure={0.45,0.6}})
blend(neck_wide, {angle=1.5})

--@ chunk 38

work(new_light, {hand="body", pile=light_flesh, coverage=2.0, clip=true, fill=true,
     angle=1.4, length={20,40}, pressure={0.5,0.65}})
work(new_shadow, {hand="body", pile=shadow_flesh3, coverage=1.6, clip=true, fill=true,
     angle=1.4, length={20,40}, pressure={0.45,0.6}})

--@ chunk 39

work(chin_neck * light_mask, {hand="body", pile=light_flesh, coverage=2.4, clip=true, fill=true,
     angle=1.3, length={20,35}, pressure={0.55,0.7}})
work(chin_neck * shadow_mask2, {hand="body", pile=shadow_flesh3, coverage=1.8, clip=true, fill=true,
     angle=1.3, length={20,35}, pressure={0.5,0.6}})

-- a small, subtler true chin-shadow right under the jawline
chin_shadow_small = ellipse(478,527,32,7)
work(chin_shadow_small, {hand="detail", pile=shadow_flesh3, coverage=1.6, clip=true, tool="round 3", length={8,15}, pressure={0.4,0.5}})

--@ chunk 40

print(drying(478,538))
strong_cover = pile{{"lead white", 6}, {"yellow ochre", 1.5}, {"red earth", 0.6}, {"vermilion", 0.4}, medium = 0.08}
work(chin_neck:grow(4) * light_mask, {hand="body", pile=strong_cover, coverage=3.0, clip=true, fill=true,
     angle=1.3, length={20,35}, pressure={0.75,0.9}})

--@ chunk 41

print(wait(200))
print(drying(478,538))

--@ chunk 42

work(chin_neck:grow(4) * light_mask, {hand="body", pile=strong_cover, coverage=2.6, clip=true, fill=true,
     angle=1.3, length={20,35}, pressure={0.8,0.95}})
work(chin_neck:grow(4) * shadow_mask2, {hand="body", pile=shadow_flesh3, coverage=2.0, clip=true, fill=true,
     angle=1.3, length={20,35}, pressure={0.7,0.85}})

--@ chunk 43

work(chin_neck:grow(4) * light_mask, {hand="body", pile=light_flesh, coverage=1.6, clip=true, fill=true,
     angle=0.6, length={20,35}, pressure={0.55,0.7}})
work(chin_neck:grow(4) * shadow_mask2, {hand="body", pile=shadow_flesh3, coverage=1.4, clip=true, fill=true,
     angle=0.6, length={20,35}, pressure={0.5,0.6}})
chin_shadow_small2 = ellipse(478,525,30,6)
work(chin_shadow_small2, {hand="detail", pile=shadow_flesh3, coverage=1.4, clip=true, tool="round 3", length={8,14}, pressure={0.35,0.45}})

--@ chunk 44

chin_round = ellipse(480,515,55,25)
work(chin_round * light_mask, {hand="body", pile=light_flesh, coverage=1.6, clip=true, fill=true,
     angle=0.5, length={15,30}, pressure={0.45,0.6}})
work(chin_round * shadow_mask2, {hand="body", pile=shadow_flesh3, coverage=1.4, clip=true, fill=true,
     angle=0.5, length={15,30}, pressure={0.4,0.55}})

--@ chunk 45

blush_pile = pile{{"vermilion", 0.6}, {"lead white", 2}, {"red earth", 0.4}, medium = 0.2}
cheek_blush_l = ellipse(345,378,36,26) * skin_mask * light_mask
work(cheek_blush_l, {hand="scumble", pile=blush_pile, coverage=1.1, clip=true,
     length={10,20}, pressure={0.25,0.35}})

cheekbone_hl = ellipse(360,332,32,11) * skin_mask * light_mask
work(cheekbone_hl, {hand="detail", pile=pile{{"lead white",3},{"yellow ochre",0.4},medium=0.15}, coverage=1.4, clip=true, tool="round 4", length={10,18}, pressure={0.35,0.5}})

cheek_hollow_r = ellipse(538,378,26,20) * skin_mask * shadow_mask2
work(cheek_hollow_r, {hand="scumble", pile=shadow_flesh3, coverage=1.2, clip=true, length={8,16}, pressure={0.4,0.5}})

--@ chunk 46

work(cheek_blush_l, {hand="scumble", pile=light_flesh, coverage=1.8, clip=true, length={10,20}, pressure={0.4,0.5}})
work(cheekbone_hl, {hand="scumble", pile=light_flesh, coverage=1.6, clip=true, length={10,18}, pressure={0.4,0.5}})
work(cheek_hollow_r, {hand="scumble", pile=pile{{"lead white",0.3},{"raw umber",1},{"red earth",0.5},medium=0.2}, coverage=1.0, clip=true, length={8,16}, pressure={0.35,0.45}})

--@ chunk 47

-- erase hard edges: repaint a generous soft area with plain flesh
erase_zone_l = (cheek_blush_l:grow(15) + cheekbone_hl:grow(15)):soften(20) * skin_mask * light_mask
work(erase_zone_l, {hand="body", pile=light_flesh, coverage=2.2, clip=true, fill=true,
     angle=function(x,y) return 0.3+0.4*math.sin(x*0.02+y*0.015) end, length={15,30}, pressure={0.5,0.65}})

erase_zone_r = cheek_hollow_r:grow(15):soften(20) * skin_mask * shadow_mask2
work(erase_zone_r, {hand="body", pile=shadow_flesh3, coverage=1.8, clip=true, fill=true,
     angle=function(x,y) return 0.3+0.4*math.sin(x*0.02+y*0.015) end, length={15,30}, pressure={0.45,0.6}})

--@ chunk 48

cheek_blush_l2 = ellipse(348,378,30,20):soften(22) * skin_mask * light_mask
work(cheek_blush_l2, {hand="scumble", pile=blush_pile, coverage=0.5, clip=true, length={12,22}, pressure={0.2,0.3}})

cheekbone_hl2 = ellipse(362,330,28,9):soften(16) * skin_mask * light_mask
work(cheekbone_hl2, {hand="scumble", pile=pile{{"lead white",3},{"yellow ochre",0.4},medium=0.2}, coverage=0.6, clip=true, length={12,20}, pressure={0.25,0.35}})

--@ chunk 49

work(cheekbone_hl2, {hand="scumble", pile=light_flesh, coverage=1.0, clip=true, length={12,20}, pressure={0.3,0.4}})

--@ chunk 50

term_pts = {
  {200,500}, {293,490}, {340,480}, {407,472}, {445,470}, {520,478}, {600,480}
}
term_line_pts = {}
for _,p in ipairs(term_pts) do table.insert(term_line_pts, p[2]); table.insert(term_line_pts, p[1]) end
halftone_band = ribbon(term_line_pts, 46):soften(20) * skin_mask
halftone_pile = pile{{"lead white",2},{"yellow ochre",0.6},{"raw umber",0.6},{"red earth",0.3}, medium=0.25}
work(halftone_band, {hand="scumble", pile=halftone_pile, coverage=0.7, clip=true, length={15,28}, pressure={0.25,0.35}})

--@ chunk 51

band_light = halftone_band * light_mask
band_shadow = halftone_band * shadow_mask2
work(band_light, {hand="body", pile=light_flesh, coverage=2.4, clip=true, fill=true,
     angle=function(x,y) return 0.3+0.3*math.sin(y*0.015) end, length={15,30}, pressure={0.55,0.7}})
work(band_shadow, {hand="body", pile=shadow_flesh3, coverage=2.2, clip=true, fill=true,
     angle=function(x,y) return 0.3+0.3*math.sin(y*0.015) end, length={15,30}, pressure={0.5,0.65}})

--@ chunk 52

work(brow_r_shape, {hand="detail", pile=brow_pile, coverage=2.0, clip=true, tool="round 2.2", length={5,10}, pressure={0.45,0.6}})
work(nose_hl, {hand="detail", pile=nose_highlight_pile, coverage=1.8, clip=true, tool="round 3", length={10,20}, pressure={0.4,0.55}})
work(nose_shadow_line, {hand="detail", pile=shadow_flesh3, coverage=1.6, clip=true, tool="round 2.2", length={10,18}, pressure={0.4,0.5}})
work(tip_hl, {hand="detail", pile=nose_highlight_pile, coverage=1.4, clip=true, tool="round 2.6", length={5,9}, pressure={0.3,0.45}})
work(nostril_l+nostril_r, {hand="detail", pile=cavity_pile, coverage=1.6, clip=true, tool="round 1.6", length={4,6}, pressure={0.45,0.6}})

--@ chunk 53

mouth_zone = ellipse(478,448,55,30) * skin_mask
work(mouth_zone * light_mask, {hand="body", pile=light_flesh, coverage=1.6, clip=true, fill=true, angle=0.6, length={12,22}, pressure={0.45,0.55}})
work(mouth_zone * shadow_mask2, {hand="body", pile=shadow_flesh3, coverage=1.4, clip=true, fill=true, angle=0.6, length={12,22}, pressure={0.4,0.5}})

work(upper_lip, {hand="detail", pile=upper_lip_pile, coverage=2.4, clip=true, tool="round 2.6", length={8,16}, pressure={0.45,0.6}})
work(lower_lip, {hand="detail", pile=lower_lip_pile, coverage=2.4, clip=true, tool="round 2.8", length={8,16}, pressure={0.5,0.65}})
work(seam_line, {hand="detail", pile=seam_pile, coverage=2.2, clip=true, tool="round 1.4", length={5,10}, pressure={0.5,0.65}})
work(lip_hl, {hand="detail", pile=pile{{"lead white",2},{"vermilion",0.3},medium=0.2}, coverage=1.4, clip=true, tool="round 2.4", length={6,10}, pressure={0.3,0.4}})
work(corner_shadow, {hand="detail", pile=seam_pile, coverage=1.6, clip=true, tool="round 1.4", length={3,5}, pressure={0.4,0.5}})

--@ chunk 54

hair_highlight_pile = pile{{"red earth",1},{"yellow ochre",1},{"lead white",0.6}, medium=0.15}
hair_hl_mask = ellipse(365,125,115,55):soften(25) * hair_mask
work(hair_hl_mask, {hand="body", pile=hair_highlight_pile, coverage=1.3, clip=true,
     angle=function(x,y) return 1.5 + 0.2*math.sin((x-365)*0.01) end, length={60,120}, pressure={0.4,0.55}})

-- a few finer flyaway strand highlights near the temple
strand1 = ribbon({300,210, 285,255, 278,300}, {2.5,1.8,1})
strand2 = ribbon({260,340, 250,385, 250,420}, {2,1.5,0.8})
work((strand1+strand2)*hair_mask, {hand="detail", pile=hair_highlight_pile, coverage=1.8, clip=true, tool="round 1.6", length={12,25}, pressure={0.35,0.5}})

--@ chunk 55

work(hair_hl_mask, {hand="body", pile=hair_pile3, coverage=2.4, clip=true,
     angle=function(x,y) return 1.5 + 0.2*math.sin((x-365)*0.01) end, length={60,120}, pressure={0.6,0.75}})
work((strand1+strand2)*hair_mask, {hand="detail", pile=hair_pile3, coverage=1.8, clip=true, tool="round 1.8", length={12,25}, pressure={0.5,0.6}})

--@ chunk 56

work(hair_hl_mask, {hand="body", pile=hair_pile3, coverage=1.2, clip=true,
     angle=function(x,y) return 1.5 + 0.2*math.sin((x-365)*0.01) end, length={60,120}, pressure={0.45,0.6}})
blend(hair_hl_mask:grow(15), {angle=1.5})

--@ chunk 57

glow_pile = pile{{"yellow ochre",1},{"raw umber",1},{"lead white",0.6}, medium=0.3}
glow_mask = ellipse(260,260,300,360):soften(170) * (-figure_reserve)
work(glow_mask, {hand="scumble", pile=glow_pile, coverage=0.6, clip=true, length={40,90}, pressure={0.2,0.3}})

--@ chunk 58

work(glow_mask:grow(10), {hand="body", pile=bg_pile, coverage=2.4, clip=true, fill=true,
     angle=function(x,y) return 0.3 + 0.15*math.sin(x*0.006+y*0.004) end, length={100,200}, pressure={0.6,0.8}})

--@ chunk 59

work(glow_mask:grow(10), {hand="body", pile=bg_pile, coverage=1.1, clip=true, fill=true,
     angle=function(x,y) return 0.4 + 0.15*math.cos(x*0.007-y*0.005) end, length={110,210}, pressure={0.45,0.6}})

--@ chunk 60

shoulder_hl_pile = pile{{"bone black",1.5},{"raw umber",1},{"Prussian blue",0.5},{"lead white",1.4}, medium=0.15}
shoulder_hl = ribbon({120,662, 200,625, 280,598, 335,585}, {16,20,16,10}) * garment_mask
work(shoulder_hl, {hand="scumble", pile=shoulder_hl_pile, coverage=0.7, clip=true, length={25,45}, pressure={0.3,0.4}})

--@ chunk 61

stray_fix = ellipse(277,472,10,8) + ellipse(300,468,8,7)
work(stray_fix, {hand="detail", pile=garment_pile, coverage=2.2, clip=true, tool="round 4", length={8,14}, pressure={0.5,0.65}})

--@ chunk 62

work(stray_fix, {hand="detail", pile=hair_pile3, coverage=2.4, clip=true, tool="round 4", length={8,14}, pressure={0.55,0.7}})

real_stray = ellipse(306,584,12,10) + ellipse(331,583,10,9)
work(real_stray, {hand="detail", pile=garment_pile, coverage=2.2, clip=true, tool="round 5", length={8,16}, pressure={0.5,0.65}})

--@ chunk 63

trouble_rect = rect(230,430,140,190)
work(trouble_rect * hair_mask, {hand="body", pile=hair_pile3, coverage=2.6, clip=true, fill=true,
     angle=1.5, length={20,40}, pressure={0.6,0.75}})
work(trouble_rect * garment_mask, {hand="body", pile=garment_pile, coverage=2.4, clip=true, fill=true,
     angle=1.2, length={20,40}, pressure={0.6,0.75}})
work(trouble_rect * (-figure_reserve), {hand="body", pile=bg_pile, coverage=2.4, clip=true, fill=true,
     angle=0.3, length={20,40}, pressure={0.6,0.75}})

--@ chunk 64

soft_trouble = trouble_rect:soften(30)
work((soft_trouble * hair_mask):grow(0), {hand="body", pile=hair_pile3, coverage=1.6, clip=true, fill=true,
     angle=1.5, length={30,60}, pressure={0.5,0.65}})
work((soft_trouble * garment_mask), {hand="body", pile=garment_pile, coverage=1.6, clip=true, fill=true,
     angle=1.2, length={30,60}, pressure={0.5,0.65}})
work((soft_trouble * (-figure_reserve)), {hand="body", pile=bg_pile, coverage=1.6, clip=true, fill=true,
     angle=0.3, length={30,60}, pressure={0.5,0.65}})

--@ chunk 65

edge_break = trouble_rect:rim(18,20) * hair_mask
work(edge_break, {hand="body", pile=hair_pile3, coverage=1.4, clip=true, fill=true,
     angle=function(x,y) return 1.5+0.15*math.sin(x*0.02) end, length={40,80}, pressure={0.45,0.6}})
edge_break2 = trouble_rect:rim(18,20) * garment_mask
work(edge_break2, {hand="body", pile=garment_pile, coverage=1.4, clip=true, fill=true,
     angle=1.2, length={40,80}, pressure={0.45,0.6}})

--@ chunk 66

smooth_zone = rect(465,195,90,215) * shadow_mask2 * skin_mask
work(smooth_zone, {hand="body", pile=shadow_flesh3, coverage=2.4, clip=true, fill=true,
     angle=function(x,y) return 0.3+0.3*math.sin(y*0.02) end, length={18,32}, pressure={0.55,0.68}})
-- restore nose shadow line and eyebrow/eye if touched
work(nose_shadow_line, {hand="detail", pile=shadow_flesh3, coverage=1.4, clip=true, tool="round 2.2", length={10,18}, pressure={0.4,0.5}})
work(brow_r_shape, {hand="detail", pile=brow_pile, coverage=1.6, clip=true, tool="round 2.2", length={5,10}, pressure={0.45,0.6}})

--@ chunk 67

hair_lock = poly({225,415, 265,445, 300,490, 325,545, 310,585, 270,590, 230,555, 195,490, 195,445}, true)
work(hair_lock, {hand="body", pile=hair_pile3, coverage=2.4, clip=true, fill=true,
     angle=function(x,y) return 1.45+0.15*math.sin(x*0.02) end, length={40,90}, pressure={0.55,0.7}})
lose(hair_lock, {pile=garment_pile, where=0.6})
lose(hair_lock, {pile=bg_pile, where=0.4})

-- a couple of fine loose strands trailing off the lock, for softness
lock_strand1 = ribbon({230,560, 240,590, 250,615}, {2,1.4,0.7})
lock_strand2 = ribbon({280,575, 285,600, 288,620}, {1.6,1.1,0.6})
work(lock_strand1+lock_strand2, {hand="detail", pile=hair_pile3, coverage=1.8, clip=true, tool="round 1.5", length={12,22}, pressure={0.4,0.5}})

--@ chunk 68

bridge = poly({210,370, 260,360, 300,400, 290,450, 250,460, 210,430}, true)
work(bridge, {hand="body", pile=hair_pile3, coverage=2.2, clip=true, fill=true,
     angle=1.5, length={30,60}, pressure={0.55,0.7}})
blend(bridge:grow(20), {angle=1.4})

--@ chunk 69
print(hair_mask and hair_mask:area() or "nil")
print(garment_mask and garment_mask:area() or "nil")
print(figure_reserve and figure_reserve:area() or "nil")
print(skin_mask2 and skin_mask2:area() or "nil")
print(bg_pile)
print(hair_pile3)
print(drying(478,538))
print(drying(250,450))
print(drying(150,150))

--@ chunk 70
print(wait(60*24*2))  -- wait 2 days
print("bg", drying(150,150))
print("bridge/lock zone", drying(250,420))
print("hair_lock", drying(260,500))
print("garment near lock", drying(350,600))
print("neck light", drying(430,480))
print("neck shadow", drying(500,480))
print("hair crown", drying(450,150))
print("face light", drying(360,300))
print("face shadow", drying(540,300))
print("mouth", drying(478,448))

--@ chunk 71
print(wait(60*24*3))  -- wait 3 more days
print("bg", drying(150,150))
print("bridge/lock zone", drying(250,420))
print("hair_lock", drying(260,500))
print("garment near lock", drying(350,600))
print("neck light", drying(430,480))
print("neck shadow", drying(500,480))

--@ chunk 72
print(wait(60*24*4))
print("garment near lock", drying(350,600))
print("garment general", drying(600,900))

--@ chunk 73
cleanup_rect = rect(140,335,310,340)  -- x140-450, y335-675

local bg_part = cleanup_rect * (-figure_reserve)
local hair_part = cleanup_rect * hair_mask
local garment_part = cleanup_rect * garment_mask
local skin_part = cleanup_rect * skin_mask2

print("bg", bg_part:area(), "hair", hair_part:area(), "garment", garment_part:area(), "skin", skin_part:area())

work(bg_part, {hand="body", pile=bg_pile, coverage=3.0, clip=true, fill=true,
     angle=function(x,y) return 0.3 + 0.15*math.sin(x*0.006+y*0.004) end, length={70,140}, pressure={0.7,0.9}})

--@ chunk 74
local hair_part = cleanup_rect * hair_mask
work(hair_part, {hand="body", pile=hair_pile3, coverage=3.0, clip=true, fill=true,
     angle=function(x,y) return 1.5 + 0.2*math.sin((x-470)*0.005) end, length={60,120}, pressure={0.7,0.9}})

local garment_part = cleanup_rect * garment_mask
work(garment_part, {hand="body", pile=garment_pile, coverage=3.0, clip=true, fill=true,
     angle=function(x,y) return 1.2 + 0.3*math.sin(x*0.004) end, length={70,140}, pressure={0.7,0.9}})

--@ chunk 75
local skin_part = cleanup_rect * skin_mask2
local skin_light = skin_part * mask(function(x,y) return 1 - shadow_strength(x,y) end)
local skin_shadow = skin_part * mask(function(x,y) return shadow_strength(x,y) end)
work(skin_light, {hand="body", pile=light_flesh, coverage=2.6, clip=true, fill=true,
     angle=function(x,y) return 1.3 + 0.2*math.sin(x*0.01) end, length={20,40}, pressure={0.6,0.8}})
work(skin_shadow, {hand="body", pile=shadow_flesh3, coverage=2.6, clip=true, fill=true,
     angle=function(x,y) return 1.3 + 0.2*math.sin(x*0.01) end, length={20,40}, pressure={0.6,0.8}})

--@ chunk 76
print(drying(500,550))
print(drying(480,600))

--@ chunk 77
local skin_part = cleanup_rect * skin_mask2
local skin_light = skin_part * mask(function(x,y) return 1 - shadow_strength(x,y) end)
local skin_shadow = skin_part * mask(function(x,y) return shadow_strength(x,y) end)
work(skin_light, {hand="body", pile=light_flesh, coverage=2.6, clip=true, fill=true,
     angle=function(x,y) return 1.3 + 0.2*math.sin(x*0.01) end, length={20,40}, pressure={0.6,0.8}})
work(skin_shadow, {hand="body", pile=shadow_flesh3, coverage=2.6, clip=true, fill=true,
     angle=function(x,y) return 1.3 + 0.2*math.sin(x*0.01) end, length={20,40}, pressure={0.6,0.8}})

--@ chunk 78
local skin_part = cleanup_rect * skin_mask2
work(skin_part, {hand="body", pile=strong_cover, coverage=3.4, clip=true, fill=true,
     angle=function(x,y) return 1.3 + 0.2*math.sin(x*0.01) end, length={20,40}, pressure={0.75,0.92}})

--@ chunk 79
local skin_part = cleanup_rect * skin_mask2
work(skin_part, {hand="body", pile=strong_cover, coverage=2.4, clip=true, fill=true,
     angle=function(x,y) return 0.6 + 0.2*math.cos(x*0.01) end, length={20,40}, pressure={0.7,0.88}})
blend(skin_part, {angle=1.4})

--@ chunk 80
local skin_part = cleanup_rect * skin_mask2
work(skin_part, {hand="body", pile=light_flesh, coverage=1.6, clip=true, fill=true,
     angle=function(x,y) return 1.3 + 0.2*math.sin(x*0.01) end, length={20,45}, pressure={0.5,0.65}})

--@ chunk 81
mouth_protect = (upper_lip + lower_lip + seam_line):grow(6)
neck_zone = (neck_wide + chin_neck + chin_round):soften(3) * skin_mask2 - mouth_protect
print("neck_zone area", neck_zone:area())

work(neck_zone, {hand="body", pile=strong_cover, coverage=3.2, clip=true, fill=true,
     angle=function(x,y) return 1.3 + 0.25*math.sin(x*0.012) end, length={20,42}, pressure={0.75,0.92}})

--@ chunk 82
local neck_light = neck_zone * mask(function(x,y) return 1 - shadow_strength(x,y) end)
local neck_shadow = neck_zone * mask(function(x,y) return shadow_strength(x,y) end)
work(neck_light, {hand="body", pile=light_flesh, coverage=2.2, clip=true, fill=true,
     angle=function(x,y) return 1.3 + 0.25*math.sin(x*0.012) end, length={20,42}, pressure={0.55,0.72}})
work(neck_shadow, {hand="body", pile=shadow_flesh3, coverage=2.2, clip=true, fill=true,
     angle=function(x,y) return 1.3 + 0.25*math.sin(x*0.012) end, length={20,42}, pressure={0.55,0.72}})
blend(neck_zone, {angle=1.45})

--@ chunk 83
print(drying(480,550))

--@ chunk 84
work(neck_zone, {hand="body", pile=strong_cover, coverage=3.0, clip=true, fill=true,
     angle=function(x,y) return 0.6 + 0.2*math.cos(x*0.015) end, length={18,36}, pressure={0.7,0.88}})

local neck_light = neck_zone * mask(function(x,y) return 1 - shadow_strength(x,y) end)
local neck_shadow = neck_zone * mask(function(x,y) return shadow_strength(x,y) end)
work(neck_light, {hand="body", pile=light_flesh, coverage=2.6, clip=true, fill=true,
     angle=function(x,y) return 1.3 + 0.2*math.sin(x*0.015) end, length={18,36}, pressure={0.55,0.7}})
work(neck_shadow, {hand="body", pile=shadow_flesh3, coverage=2.6, clip=true, fill=true,
     angle=function(x,y) return 1.3 + 0.2*math.sin(x*0.015) end, length={18,36}, pressure={0.55,0.7}})

--@ chunk 85
blend(neck_zone, {angle=1.4})

--@ chunk 86
local names = {"ear_outer","ear_inner","ear_rim_hl","lobe_hl","ear_pile","cavity_pile",
  "brow_r_shape","brow_pile","eye_r","sclera_r_pile","iris_r","iris_r_pile","pupil_r","pupil_pile",
  "catchlight_r","lid_r_top","lid_r_bottom","lid_pile","corner_in_r","corner_out_r",
  "nose_shadow_line","jaw_shadow","shadow_flesh3","light_flesh","skin_mask","shadow_mask2"}
for _,n in ipairs(names) do
  print(n, _G[n] ~= nil)
end

--@ chunk 87
local names = {"nose_hl","nose_highlight_pile","tip_hl","nostril_l","nostril_r","brow_l_shape"}
for _,n in ipairs(names) do print(n, _G[n] ~= nil) end
print(skin_mask:area())

--@ chunk 88
local names={"nasolabial","undernose","brow_r"}
for _,n in ipairs(names) do print(n, _G[n]~=nil) end

opaque_shadow_base = pile{{"lead white",2},{"raw umber",2},{"red earth",1},{"bone black",0.3}, medium=0.08}
print(opaque_shadow_base)

repair_zone = (rect(478,195,140,250) * skin_mask) - mouth_protect
print("repair_zone area", repair_zone:area())

--@ chunk 89
work(repair_zone, {hand="body", pile=opaque_shadow_base, coverage=3.4, clip=true, fill=true,
     angle=function(x,y) return 0.3 + 0.3*math.sin(y*0.02) end, length={18,34}, pressure={0.75,0.92}})

--@ chunk 90
local ear_zone = ear_outer:grow(2)
work(ear_zone, {hand="detail", pile=opaque_shadow_base, coverage=3.2, clip=true, tool="round 5",
     length={10,20}, pressure={0.7,0.85}})
work(ear_zone, {hand="detail", pile=ear_pile, coverage=2.8, clip=true, tool="round 5",
     length={10,20}, pressure={0.6,0.75}})

--@ chunk 91
work(ear_inner, {hand="detail", pile=cavity_pile, coverage=2.2, clip=true, tool="round 2.4", length={6,12}, pressure={0.45,0.6}})
work(ear_rim_hl, {hand="detail", pile=light_flesh, coverage=2.0, clip=true, tool="round 1.6", length={6,12}, pressure={0.4,0.55}})
work(lobe_hl, {hand="detail", pile=ear_pile, coverage=1.8, clip=true, tool="round 2.2", length={5,8}, pressure={0.4,0.5}})

--@ chunk 92
-- eyebrow
work(brow_r_shape, {hand="detail", pile=brow_pile, coverage=2.2, clip=true, tool="round 2.2", length={5,10}, pressure={0.5,0.65}})

-- eye
work(eye_r, {hand="detail", pile=sclera_r_pile, coverage=2.4, clip=true, tool="round 2.4", length={4,8}, pressure={0.4,0.55}})
work(iris_r, {hand="detail", pile=iris_r_pile, coverage=2.0, clip=true, tool="round 2.4", length={4,7}, pressure={0.5,0.62}})
work(pupil_r, {hand="detail", pile=pupil_pile, coverage=2.2, clip=true, tool="round 1.6", length={3,5}, pressure={0.6,0.7}})
work(catchlight_r, {hand="detail", pile=pile{{"lead white",1},medium=0.1}, coverage=1.8, clip=true, tool="round 1", length={2,3}, pressure={0.45,0.55}})
work(lid_r_top, {hand="detail", pile=lid_pile, coverage=2.0, clip=true, tool="round 1.3", length={3,6}, pressure={0.4,0.55}})
work(lid_r_bottom, {hand="detail", pile=shadow_flesh3, coverage=1.6, clip=true, tool="round 1.2", length={3,6}, pressure={0.3,0.4}})
work(corner_in_r+corner_out_r, {hand="detail", pile=shadow_flesh3, coverage=1.8, clip=true, tool="round 1.2", length={3,5}, pressure={0.35,0.45}})

--@ chunk 93
-- nose shadow / nostril / nasolabial / jaw shadow refresh in this zone
work(nose_shadow_line, {hand="detail", pile=shadow_flesh3, coverage=1.8, clip=true, tool="round 2.2", length={10,18}, pressure={0.4,0.55}})
work(nostril_r, {hand="detail", pile=cavity_pile, coverage=2.0, clip=true, tool="round 1.6", length={4,6}, pressure={0.5,0.65}})
work(nasolabial, {hand="detail", pile=cavity_pile, coverage=1.6, clip=true, tool="round 1.6", length={6,12}, pressure={0.4,0.5}})
work(jaw_shadow, {hand="detail", pile=cavity_pile, coverage=1.6, clip=true, tool="round 3", length={8,16}, pressure={0.4,0.5}})

--@ chunk 94
work(nose_hl, {hand="detail", pile=nose_highlight_pile, coverage=2.2, clip=true, tool="round 3", length={10,20}, pressure={0.45,0.6}})
work(nose_shadow_line, {hand="detail", pile=shadow_flesh3, coverage=2.0, clip=true, tool="round 2.2", length={10,18}, pressure={0.45,0.58}})

local blend_zone = (nose_hl + nose_shadow_line):grow(2) - eye_r:grow(5) - eye_l:grow(5)
blend(blend_zone, {angle=1.45})

work(tip_hl, {hand="detail", pile=nose_highlight_pile, coverage=1.6, clip=true, tool="round 2.6", length={5,9}, pressure={0.3,0.45}})
work(nostril_l+nostril_r, {hand="detail", pile=cavity_pile, coverage=1.8, clip=true, tool="round 1.6", length={4,6}, pressure={0.45,0.6}})
work(undernose, {hand="detail", pile=cavity_pile, coverage=1.2, clip=true, tool="round 3", length={5,10}, pressure={0.3,0.4}})

--@ chunk 95
local soften_jaw = jaw_shadow:grow(3) * skin_mask2
work(soften_jaw, {hand="body", pile=shadow_flesh3, coverage=2.6, clip=true, fill=true,
     angle=function(x,y) return 0.3+0.3*math.sin(y*0.02) end, length={15,28}, pressure={0.55,0.7}})
blend(soften_jaw, {angle=0.4})

--@ chunk 96
local cover_jaw = jaw_shadow:grow(4) * skin_mask2
work(cover_jaw, {hand="body", pile=shadow_flesh3, coverage=3.0, clip=true, fill=true,
     angle=function(x,y) return 0.3+0.3*math.sin(y*0.02) end, length={15,28}, pressure={0.6,0.75}})
blend(cover_jaw, {angle=0.5})

--@ chunk 97
local cover_jaw = jaw_shadow:grow(4) * skin_mask2
work(cover_jaw, {hand="body", pile=opaque_shadow_base, coverage=3.4, clip=true, fill=true,
     angle=function(x,y) return 0.3+0.3*math.sin(y*0.02) end, length={15,28}, pressure={0.75,0.9}})
work(cover_jaw, {hand="body", pile=shadow_flesh3, coverage=2.2, clip=true, fill=true,
     angle=function(x,y) return 0.3+0.3*math.sin(y*0.02) end, length={15,28}, pressure={0.55,0.7}})
blend(cover_jaw, {angle=0.5})

--@ chunk 98
local cover_jaw = jaw_shadow:grow(5) * skin_mask2
work(cover_jaw, {hand="body", pile=opaque_shadow_base, coverage=4.5, clip=true, fill=true,
     angle=function(x,y) return 0.3+0.3*math.sin(y*0.02) end, length={15,28}, pressure={0.85,0.98}})
print(drying(520,420))

--@ chunk 99
local cover_jaw = jaw_shadow:grow(5) * skin_mask2
work(cover_jaw, {hand="body", pile=opaque_shadow_base, coverage=2.0, clip=true, fill=true,
     angle=0.9, length={15,28}, pressure={0.8,0.95}})
work(cover_jaw, {hand="body", pile=shadow_flesh3, coverage=2.4, clip=true, fill=true,
     angle=function(x,y) return 0.3+0.3*math.sin(y*0.02) end, length={15,28}, pressure={0.6,0.75}})

--@ chunk 100
local cover_jaw = jaw_shadow:grow(6) * skin_mask2
blend(cover_jaw, {angle=0.5})

--@ chunk 101
local top_part = jaw_shadow:grow(5) * rect(400,330,220,90) * skin_mask2
work(top_part, {hand="body", pile=shadow_flesh3, coverage=1.8, clip=true, fill=true,
     angle=0.7, length={15,25}, pressure={0.55,0.68}})
blend(top_part:grow(6), {angle=0.6})

--@ chunk 102
local clear_zone = (nostril_l + nostril_r + undernose + nasolabial):grow(4) * skin_mask
local cz_light = clear_zone * light_mask
local cz_shadow = clear_zone * shadow_mask2
work(cz_light, {hand="detail", pile=light_flesh, coverage=2.4, clip=true, tool="round 3", length={8,14}, pressure={0.5,0.65}})
work(cz_shadow, {hand="detail", pile=shadow_flesh3, coverage=2.4, clip=true, tool="round 3", length={8,14}, pressure={0.5,0.65}})

--@ chunk 103
opaque_light_base = pile{{"lead white",4},{"yellow ochre",1},{"red earth",0.3}, medium=0.08}
local clear_zone = (nostril_l + nostril_r + undernose + nasolabial):grow(4) * skin_mask
local cz_light = clear_zone * light_mask
local cz_shadow = clear_zone * shadow_mask2
work(cz_light, {hand="detail", pile=opaque_light_base, coverage=3.0, clip=true, tool="round 3", length={8,14}, pressure={0.7,0.85}})
work(cz_shadow, {hand="detail", pile=opaque_shadow_base, coverage=3.0, clip=true, tool="round 3", length={8,14}, pressure={0.7,0.85}})
work(cz_light, {hand="detail", pile=light_flesh, coverage=2.0, clip=true, tool="round 3", length={8,14}, pressure={0.5,0.6}})
work(cz_shadow, {hand="detail", pile=shadow_flesh3, coverage=2.0, clip=true, tool="round 3", length={8,14}, pressure={0.5,0.6}})

--@ chunk 104
local b = brush("round", 2.4)
b:load(cavity_pile, 0.8)
b:touch(463, 406, {pressure=0.55, drag={2,0.3}, angle=0.3})
b:touch(491, 404, {pressure=0.5, drag={2,0.2}, angle=0.2})

local undernose_soft = undernose:soften(6)
work(undernose_soft, {hand="scumble", pile=cavity_pile, coverage=0.5, clip=true, length={10,16}, pressure={0.2,0.3}})

--@ chunk 105
work(nasolabial, {hand="detail", pile=cavity_pile, coverage=1.0, clip=true, tool="round 1.4", length={6,10}, pressure={0.3,0.4}})
blend((undernose:grow(6)), {angle=1.3})

--@ chunk 106
local seam_fix = rect(150,300,350,70) * hair_mask
work(seam_fix, {hand="body", pile=hair_pile3, coverage=2.0, clip=true, fill=true,
     angle=function(x,y) return 1.5 + 0.2*math.sin((x-470)*0.005) end, length={50,100}, pressure={0.55,0.72}})
blend(seam_fix:grow(15), {angle=1.5})

--@ chunk 107
print(drying(300,300))
print(drying(300,150))

--@ chunk 108
work(hair_mask, {hand="body", pile=hair_pile3, coverage=2.6, clip=true, fill=true,
     angle=function(x,y) return 1.5 + 0.2*math.sin((x-470)*0.006) end, length={60,130}, pressure={0.6,0.78}})

--@ chunk 109
local x,y = 240,330
print("hair_mask", hair_mask:at(x,y))
print("figure_reserve", figure_reserve:at(x,y))
print("skin_mask", skin_mask:at(x,y))
print("garment_mask", garment_mask:at(x,y))
print("hair_blob", hair_blob:at(x,y))
print("skin_mask2", skin_mask2:at(x,y))

--@ chunk 110
local patch_fix = rect(190,255,100,160) * hair_mask
work(patch_fix, {hand="body", pile=hair_pile3, coverage=3.6, clip=true, fill=true,
     angle=function(x,y) return 1.5 + 0.15*math.sin((x-470)*0.008) end, length={40,80}, pressure={0.75,0.9}})
