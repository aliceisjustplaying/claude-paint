-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{size=700, aspect=0.8, linen={18, 15}, seed=7,
  ground={
    {pile={{"lead white", 3}, {"yellow ochre", 1}, {"raw umber", 1}}, um=90, apply="knife", texture=0.25},
    {pile={{"raw umber", 2}, {"red earth", 1}}, um=35, apply="brush"}
  }}
print("W="..W, "H="..H)

--@ chunk 2

head = body.ellipsoid({460, 420, 0}, {98, 128, 95}):turn({460, 420, 0}, 0.32, 0.06, 0.02)
torso = body.block({520, 950, 0}, {480, 650, 260}, 60):turn({520, 950, 0}, 0.18, 0, 0)

fig = form{ {head, dist=0}, {torso, dist=0.3},
  light={from={-0.6, -0.85}, front=0.35, ambient=0.24} }

head_sil = fig:silhouette{parts={1}}
torso_sil = fig:silhouette{parts={2}}
figure_mask = head_sil + torso_sil
bg_mask = everywhere() - figure_mask

dark_bg = pile{{"raw umber", 3}, {"bone black", 2}, {"red earth", 1}}
work(bg_mask, {hand="broad", pile=dark_bg, angle=0.3, coverage=2.2})

--@ chunk 3

clothing_base = pile{{"raw umber", 3}, {"bone black", 2}, {"red earth", 1}, {"lead white", 1}}
work(torso_sil, {hand="body", pile=clothing_base, angle=0.35, coverage=2.4, clip=true, edge="firm"})

flesh_base = pile{{"lead white", 2}, {"yellow ochre", 2}, {"red earth", 1}, {"vermilion", 0.5}}
work(head_sil, {hand="body", pile=flesh_base, angle=0.5, coverage=2.4, clip=true, edge="firm"})

--@ chunk 4

torso_shape = poly({
  {185, 705}, {225, 605}, {330, 558}, {430, 548}, {520, 548}, {605, 562},
  {690, 592}, {762, 648}, {818, 758}, {835, 1260}, {165, 1260}, {158, 782}
}, true)
torso_shape = torso_shape:roughen(5, 70, 3, 0.3)

cover_mask = torso_sil - torso_shape
work(cover_mask, {hand="broad", pile=dark_bg, angle=0.3, coverage=2.4})

clothing_base = pile{{"raw umber", 2}, {"bone black", 3}, {"Prussian blue", 0.5}, {"red earth", 1}}
work(torso_shape, {hand="body", pile=clothing_base, angle=0.35, coverage=2.6, clip=true, edge="firm"})
torso_sil = torso_shape
figure_mask = head_sil + torso_sil

--@ chunk 5

work(torso_shape, {hand="body", pile=clothing_base, angle=0.4, coverage=1.6, clip=true, edge="firm"})

--@ chunk 6

head_lit = fig:lit{parts={1}, soft=0.18}
head_shadow = fig:shadow{parts={1}}

-- fill any remaining bare specks and even the flesh base
work(head_sil, {hand="body", pile=flesh_base, angle=0.5, coverage=1.2, clip=true, edge="firm", fill=true})

shadow_mix = pile{{"yellow ochre",2},{"red earth",2},{"raw umber",1},{"bone black",0.4},{"cobalt blue",0.3}}
work(head_shadow * head_sil, {hand="scumble", pile=shadow_mix, angle=0.6, coverage=1.6, clip=head_sil})

light_mix = pile{{"lead white",3},{"yellow ochre",1},{"vermilion",0.4}}
work(head_lit * head_sil, {hand="scumble", pile=light_mix, angle=0.5, coverage=1.3, clip=head_sil})

--@ chunk 7

blend(head_sil, {angle=0.5})

--@ chunk 8

hairline_pts = { {300, 460}, {330, 380}, {365, 330}, {410, 305}, {460, 295}, {510, 305}, {555, 335}, {590, 400}, {615, 470} }
hair_mask = head_sil * above(hairline_pts)

hair_base = pile{{"bone black", 2}, {"raw umber", 2}, {"lead white", 1}}
work(hair_mask, {hand="body", pile=hair_base, angle=0.15, coverage=2.2, clip=true, edge="firm", fill=true})

--@ chunk 9

hairline_pts = { {350, 470}, {375, 410}, {405, 365}, {440, 345}, {460, 340}, {480, 345}, {515, 365}, {545, 410}, {570, 470} }
hair_mask = head_sil * above(hairline_pts)

work(hair_mask, {hand="body", pile=hair_base, angle=0.15, coverage=2.6, clip=true, edge="firm", fill=true})

--@ chunk 10

detail_brush = brush{kind="round", width=3.4, point=0.55}
fine_brush = brush{kind="round", width=1.6, point=0.8}

dark_line = pile{{"raw umber",2},{"bone black",2}, medium=0.25}

-- eyebrows
detail_brush:load(dark_line, 0.7)
detail_brush:stroke({{372, 397},{402, 388},{432, 392}}, {pressure={0.5,0.35}, ramps={0.3,0.5}, orient="along"})
detail_brush:stroke({{462, 395},{490, 392},{512, 398}}, {pressure={0.4,0.25}, ramps={0.3,0.5}, orient="along"})

-- eye socket shadow (upper lid crease + under-eye), light scumble with shadow_mix
detail_brush:load(shadow_mix, 0.5)
detail_brush:stroke({{378, 412},{402, 407},{428, 413}}, {pressure={0.45,0.3}, orient="along"})
detail_brush:stroke({{466, 411},{490, 408},{510, 414}}, {pressure={0.4,0.25}, orient="along"})

--@ chunk 11

white_pile = pile{{"lead white",4},{"yellow ochre",0.3}}
iris_pile = pile{{"raw umber",3},{"bone black",1}}
pupil_pile = pile{{"bone black",3}}
spark_pile = pile{{"lead white",1}}

eyeL_white = ellipse(403,417,13,6)
eyeR_white = ellipse(487,415,10,5)
work(eyeL_white, {hand="detail", pile=white_pile, coverage=2.2, clip=true})
work(eyeR_white, {hand="detail", pile=white_pile, coverage=2.0, clip=true})

irisL = ellipse(400,417,6,6) * eyeL_white
irisR = ellipse(486,415,4.5,4.5) * eyeR_white
work(irisL, {hand="detail", pile=iris_pile, coverage=2.2, clip=true})
work(irisR, {hand="detail", pile=iris_pile, coverage=2.0, clip=true})

pupilL = ellipse(400,417,2.6,2.6)
pupilR = ellipse(486,415,2,2)
work(pupilL, {hand="detail", pile=pupil_pile, coverage=2.5, clip=true})
work(pupilR, {hand="detail", pile=pupil_pile, coverage=2.5, clip=true})

fine_brush:load(dark_line, 0.5)
fine_brush:stroke({{391,421},{403,423},{416,421}}, {pressure={0.3,0.25}, orient="along"})
fine_brush:stroke({{478,419},{487,421},{496,419}}, {pressure={0.25,0.2}, orient="along"})

sparkL = ellipse(397.5,414,1.1,1.1)
sparkR = ellipse(484,412,0.9,0.9)
work(sparkL, {hand="detail", pile=spark_pile, coverage=2.5, clip=true})
work(sparkR, {hand="detail", pile=spark_pile, coverage=2.5, clip=true})

--@ chunk 12

-- nose shadow (right/far side plane) and bridge
nose_shadow = poly({{429,404},{440,430},{444,462},{432,472},{420,468},{424,435}}, true)
work(nose_shadow, {hand="detail", pile=shadow_mix, coverage=1.6, clip=true, edge="soft"})

fine_brush:load(light_mix, 0.55)
fine_brush:stroke({{427,403},{420,432},{416,462}}, {pressure={0.35,0.2}, orient="along"})

-- nostrils
fine_brush:load(dark_line, 0.5)
fine_brush:touch(410,470,{pressure=0.45})
fine_brush:touch(428,472,{pressure=0.4})
fine_brush:stroke({{406,477},{418,480},{430,478}}, {pressure={0.3,0.35,0.25}, orient="along"})

-- mouth
fine_brush:load(dark_line, 0.45)
fine_brush:stroke({{394,501},{416,505},{440,502}}, {pressure={0.35,0.4,0.25}, orient="along"})
lip_pile = pile{{"vermilion",1},{"red earth",1},{"lead white",1}}
lower_lip = poly({{398,503},{416,509},{438,504},{424,514},{408,513}}, true)
work(lower_lip, {hand="detail", pile=lip_pile, coverage=1.6, clip=true, edge="soft"})
fine_brush:load(shadow_mix,0.4)
fine_brush:stroke({{406,517},{420,519},{432,516}}, {pressure={0.3,0.25}, orient="along"})

--@ chunk 13

soften_area = ellipse(425,440,45,55)
blend(soften_area, {angle=1.4})

--@ chunk 14

-- redo left eye (smudged by blend)
eyeL_white = ellipse(403,417,13,6)
work(eyeL_white, {hand="detail", pile=white_pile, coverage=2.6, clip=true})
irisL = ellipse(400,417,6,6) * eyeL_white
work(irisL, {hand="detail", pile=iris_pile, coverage=2.4, clip=true})
pupilL = ellipse(400,417,2.6,2.6)
work(pupilL, {hand="detail", pile=pupil_pile, coverage=2.6, clip=true})
sparkL = ellipse(397.5,414,1.1,1.1)
work(sparkL, {hand="detail", pile=spark_pile, coverage=2.6, clip=true})
fine_brush:load(dark_line, 0.5)
fine_brush:stroke({{391,421},{403,423},{416,421}}, {pressure={0.3,0.25}, orient="along"})

--@ chunk 15

fine_brush:load(shadow_mix, 0.45)
fine_brush:stroke({{433,408},{436,428},{433,452},{427,468}}, {pressure={0.4,0.45,0.3}, orient="along"})
fine_brush:load(shadow_mix, 0.35)
fine_brush:stroke({{437,415},{440,435},{436,455}}, {pressure={0.3,0.35,0.2}, orient="along"})

fine_brush:load(light_mix, 0.5)
fine_brush:stroke({{427,404},{422,432},{418,460}}, {pressure={0.35,0.4,0.15}, orient="along"})

fine_brush:load(dark_line, 0.45)
fine_brush:touch(410,469,{pressure=0.4})
fine_brush:touch(428,471,{pressure=0.35})
fine_brush:stroke({{407,476},{418,479},{429,477}}, {pressure={0.28,0.32,0.22}, orient="along"})

nose_line = ribbon({{429,403},{423,432},{418,462}}, 26)
blend(nose_line, {angle=1.5})

--@ chunk 16

jaw_shadow = ribbon({{375,505},{420,540},{470,538},{525,500}}, 22)
work(jaw_shadow * head_sil, {hand="detail", pile=shadow_mix, coverage=1.3, clip=true, edge="loose"})

-- left ear (near side)
ear_pile = pile{{"lead white",1.6},{"yellow ochre",1.6},{"red earth",1.2},{"vermilion",0.5}}
ear_shape = poly({{348,420},{362,405},{372,425},{368,460},{354,462},{345,440}}, true)
work(ear_shape, {hand="detail", pile=ear_pile, coverage=2.0, clip=true, edge="soft"})
fine_brush:load(shadow_mix,0.35)
fine_brush:stroke({{358,420},{362,438},{357,452}}, {pressure={0.25,0.3,0.2}, orient="along"})

--@ chunk 17

fix_area = poly({{372,492},{530,492},{535,562},{368,562}}, true)
work(fix_area * head_sil, {hand="detail", pile=flesh_base, coverage=2.2, clip=true, edge="soft"})

--@ chunk 18

lower_band = head_sil * mask(function(x,y) return y > 500 and 1 or 0 end)
rim_shadow = head_sil:rim(24, 10) * lower_band
work(rim_shadow, {hand="scumble", pile=shadow_mix, coverage=1.1, clip=head_sil, edge="loose"})

-- mouth redo
fine_brush:load(dark_line, 0.42)
fine_brush:stroke({{394,501},{416,505},{440,502}}, {pressure={0.32,0.38,0.24}, orient="along"})
lip_pile = pile{{"vermilion",1},{"red earth",1},{"lead white",1}}
lower_lip = poly({{398,503},{416,509},{438,504},{424,514},{408,513}}, true)
work(lower_lip, {hand="detail", pile=lip_pile, coverage=1.7, clip=true, edge="soft"})
fine_brush:load(shadow_mix,0.35)
fine_brush:stroke({{404,518},{420,521},{434,517}}, {pressure={0.28,0.24}, orient="along"})

--@ chunk 19

hair_dark = pile{{"bone black",3},{"raw umber",2}}
hair_light = pile{{"lead white",1.5},{"bone black",1.5},{"raw umber",1}}

work(hair_mask * head_shadow, {hand="scumble", pile=hair_dark, coverage=1.3, clip=hair_mask})
work(hair_mask * head_lit, {hand="scumble", pile=hair_light, coverage=1.1, clip=hair_mask})

-- a few strand highlights
strand_brush = brush{kind="rigger", width=1.4, point=0.9}
strand_brush:load(hair_light, 0.5)
strand_brush:stroke({{400,300},{380,340},{365,380}}, {pressure={0.35,0.15}, orient="along"})
strand_brush:stroke({{460,297},{455,330},{450,360}}, {pressure={0.3,0.12}, orient="along"})
strand_brush:stroke({{520,320},{535,355},{545,395}}, {pressure={0.3,0.12}, orient="along"})
strand_brush:load(hair_dark, 0.5)
strand_brush:stroke({{420,300},{435,335},{440,370}}, {pressure={0.3,0.12}, orient="along"})
strand_brush:stroke({{560,360},{575,400},{580,440}}, {pressure={0.3,0.12}, orient="along"})

--@ chunk 20

clothing_cover = pile{{"bone black",4},{"raw umber",2},{"lead white",0.6},{"Prussian blue",0.3}}
work(torso_shape, {hand="body", pile=clothing_cover, angle=0.3, coverage=2.8, clip=true, edge="firm", fill=true})

--@ chunk 21

torso_lit = fig:lit{parts={2}, soft=0.25}
torso_shadow = fig:shadow{parts={2}}

coat_light = pile{{"bone black",3},{"raw umber",2},{"lead white",1.2},{"Prussian blue",0.2}}
coat_dark = pile{{"bone black",4},{"raw umber",1}}

work(torso_shape * torso_lit, {hand="scumble", pile=coat_light, coverage=1.1, clip=torso_shape})
work(torso_shape * torso_shadow, {hand="scumble", pile=coat_dark, coverage=1.1, clip=torso_shape})
blend(torso_shape, {angle=0.4})

--@ chunk 22

work(torso_shape, {hand="body", pile=clothing_cover, angle=0.3, coverage=2.2, clip=true, edge="firm", fill=true})

--@ chunk 23

work(torso_shape, {hand="body", pile=clothing_cover, angle=0.5, coverage=3.2, clip=true, edge="firm", fill=true})
blend(torso_shape, {angle=0.35})

--@ chunk 24

shoulder_light = poly({{195,480},{330,455},{420,470},{430,560},{300,590},{190,570}}, true)
work(shoulder_light * torso_shape, {hand="scumble", pile=coat_light, coverage=0.9, clip=torso_shape, edge="loose"})

fold_brush = brush{kind="filbert", width=10, point=0.15}
fold_brush:load(coat_dark, 0.5)
fold_brush:stroke({{330,620},{318,780},{330,950},{320,1150}}, {pressure={0.3,0.45,0.35,0.3}, orient="along"})
fold_brush:stroke({{560,610},{575,760},{560,940},{575,1140}}, {pressure={0.25,0.4,0.3,0.25}, orient="along"})
fold_brush:load(coat_light, 0.4)
fold_brush:stroke({{260,600},{250,760},{260,920}}, {pressure={0.25,0.35,0.2}, orient="along"})
blend(torso_shape, {angle=1.5})

--@ chunk 25

palette_shape = poly({
  {300,878},{395,852},{498,868},{560,930},{568,1010},
  {510,1075},{400,1090},{305,1050},{258,970}
}, true)
thumb_hole = ellipse(322,978,20,28)
palette_shape = palette_shape - thumb_hole

wood_pile = pile{{"raw umber",2},{"yellow ochre",1.5},{"lead white",0.8}}
work(palette_shape, {hand="body", pile=wood_pile, angle=0.4, coverage=2.4, clip=true, edge="firm", fill=true})

-- rim shading on palette
pal_shadow = palette_shape:rim(18,8)
work(pal_shadow, {hand="scumble", pile=pile{{"raw umber",2},{"bone black",1}}, coverage=1.0, clip=palette_shape, edge="soft"})

--@ chunk 26

dab_brush = brush{kind="round", width=7, point=0.2}
dabs = {
  {350,866, pile{{"lead white",1}}},
  {388,854, pile{{"yellow ochre",1}}},
  {428,850, pile{{"vermilion",1}}},
  {466,858, pile{{"red earth",1}}},
  {500,875, pile{{"cobalt blue",1}}},
  {525,902, pile{{"green earth",1}}},
  {542,935, pile{{"bone black",1}}},
  {548,972, pile{{"raw umber",1}}},
}
for _, d in ipairs(dabs) do
  dab_brush:load(d[3], 0.75)
  dab_brush:touch(d[1], d[2], {pressure=0.7})
end

smear = pile{{"lead white",1},{"raw umber",0.6},{"yellow ochre",0.4}}
smear_brush = brush{kind="filbert", width=18, point=0}
smear_brush:load(smear, 0.5)
smear_brush:stroke({{400,940},{440,955},{400,970}}, {pressure={0.4,0.5,0.3}, orient="along"})

--@ chunk 27

wait(240)
print(drying(410, 960))

--@ chunk 28

dab_brush = brush{kind="round", width=11, point=0.1}
dabs = {
  {350,866, pile{{"lead white",1}}},
  {388,852, pile{{"yellow ochre",1}}},
  {428,848, pile{{"vermilion",1}}},
  {466,856, pile{{"red earth",1}}},
  {500,873, pile{{"cobalt blue",1}}},
  {525,900, pile{{"green earth",1}}},
  {542,933, pile{{"bone black",1}}},
}
for _, d in ipairs(dabs) do
  dab_brush:load(d[3], 0.9)
  dab_brush:touch(d[1], d[2], {pressure=0.85})
end

--@ chunk 29

test_brush = brush{kind="round", width=30, point=0}
test_brush:load(pile{{"lead white",1}}, 1.0)
test_brush:touch(700, 700, {pressure=1.0})
print(test_brush:fullness())

--@ chunk 30

test2 = brush{kind="round", width=15, point=0}
test2:load(pile{{"vermilion",1}}, 1.0)
test2:touch(410, 900, {pressure=1.0})

--@ chunk 31

dab_brush = brush{kind="round", width=15, point=0}
dabs = {
  {350,866, pile{{"lead white",1}}},
  {388,852, pile{{"yellow ochre",1}}},
  {466,856, pile{{"red earth",1}}},
  {500,873, pile{{"cobalt blue",1}}},
  {525,900, pile{{"green earth",1}}},
  {542,933, pile{{"bone black",1}}},
}
for i, d in ipairs(dabs) do
  dab_brush:load(d[3], 1.0)
  dab_brush:touch(d[1], d[2], {pressure=1.0})
end
print("done")

--@ chunk 32

test3 = brush{kind="round", width=15, point=0}
test3:load(pile{{"lead white",1}}, 1.0)
test3:touch(500, 873, {pressure=1.0})

--@ chunk 33

test4 = brush{kind="round", width=15, point=0}
test4:load(pile{{"lead white",1}}, 1.0)
test4:touch(500, 700, {pressure=1.0})
print(test4:fullness())

--@ chunk 34

print(drying(500,873), drying(410,900), drying(350,866))

--@ chunk 35

test5 = brush{kind="round", width=15, point=0}
test5:load(pile{{"lead white",1}}, 1.0)
test5:touch(900, 873, {pressure=1.0})
test5:load(pile{{"lead white",1}}, 1.0)
test5:touch(480, 873, {pressure=1.0})
test5:load(pile{{"lead white",1}}, 1.0)
test5:touch(500, 890, {pressure=1.0})

--@ chunk 36

dab_brush = brush{kind="round", width=20, point=0}
dabs2 = {
  {345,905, pile{{"lead white",1}}},
  {385,895, pile{{"yellow ochre",1}}},
  {462,890, pile{{"cobalt blue",1}}},
  {500,905, pile{{"green earth",1}}},
  {530,940, pile{{"bone black",1}}},
}
for i, d in ipairs(dabs2) do
  dab_brush:load(d[3], 1.0)
  dab_brush:touch(d[1], d[2], {pressure=1.0})
end

--@ chunk 37

wait(3*24*60)
print(drying(500,873), drying(410,900), drying(345,905))

--@ chunk 38

wait(4*24*60)
print(drying(500,873), drying(410,900), drying(345,905))

--@ chunk 39

dab_brush = brush{kind="round", width=16, point=0}
dabs3 = {
  {350,864, pile{{"lead white",1}}},
  {388,850, pile{{"yellow ochre",1}}},
  {428,846, pile{{"vermilion",1}}},
  {466,854, pile{{"red earth",1}}},
  {500,871, pile{{"cobalt blue",1}}},
  {525,898, pile{{"green earth",1}}},
  {542,931, pile{{"bone black",1}}},
}
for i, d in ipairs(dabs3) do
  dab_brush:load(d[3], 1.0)
  dab_brush:touch(d[1], d[2], {pressure=0.9})
end

--@ chunk 40

dab_brush:wipe(1.0)
dab_brush:reload(pile{{"cobalt blue",1}}, 1.0)
dab_brush:touch(500,871,{pressure=0.9})
dab_brush:wipe(1.0)
dab_brush:reload(pile{{"green earth",1}}, 1.0)
dab_brush:touch(525,898,{pressure=0.9})
dab_brush:wipe(1.0)
dab_brush:reload(pile{{"bone black",1}}, 1.0)
dab_brush:touch(542,931,{pressure=0.9})

--@ chunk 41

thumb_pile = pile{{"lead white",1.5},{"yellow ochre",1.5},{"red earth",1},{"vermilion",0.4}}
thumb_shape = ellipse(320,980,13,21)
work(thumb_shape, {hand="detail", pile=thumb_pile, coverage=2.2, clip=true, edge="soft"})
fine_brush:load(shadow_mix,0.35)
fine_brush:stroke({{312,964},{318,980},{312,996}}, {pressure={0.25,0.3,0.2}, orient="along"})

-- fingers curling over the near-bottom edge of the palette
knuckle_brush = brush{kind="round", width=15, point=0.1}
knuckles = {{278,1015},{308,1038},{342,1052},{378,1058}}
for i, k in ipairs(knuckles) do
  knuckle_brush:load(thumb_pile, 0.8)
  knuckle_brush:touch(k[1], k[2], {pressure=0.7})
end
fine_brush:load(shadow_mix, 0.35)
fine_brush:stroke({{265,1000},{295,1028},{325,1046},{362,1055},{398,1058}}, {pressure={0.3,0.35,0.3,0.3,0.2}, orient="along"})

--@ chunk 42

cover_knuckles = poly({{250,995},{420,1000},{420,1080},{250,1080}}, true)
work(cover_knuckles * torso_shape, {hand="body", pile=clothing_cover, coverage=2.0, clip=torso_shape, edge="firm"})

--@ chunk 43

work(thumb_shape, {hand="detail", pile=clothing_cover, coverage=2.4, clip=true, edge="firm"})

--@ chunk 44

thumb2 = ellipse(320,985,10,16)
work(thumb2, {hand="detail", pile=thumb_pile, coverage=2.0, clip=true, edge="soft"})
fine_brush:load(shadow_mix,0.3)
fine_brush:stroke({{311,972},{309,985},{312,998}}, {pressure={0.2,0.22,0.18}, orient="along"})
fine_brush:load(light_mix,0.35)
fine_brush:stroke({{326,976},{328,986}}, {pressure={0.2,0.15}, orient="along"})

--@ chunk 45

handle_pile = pile{{"raw umber",2},{"yellow ochre",1},{"lead white",0.3}}
handle_brush = brush{kind="rigger", width=3.2, point=0.7}
handle_brush:load(handle_pile, 0.7)
handle_brush:stroke({{300,960},{255,820},{220,700}}, {pressure={0.5,0.4,0.15}, orient="along"})
handle_brush:load(handle_pile, 0.7)
handle_brush:stroke({{292,955},{235,830},{195,715}}, {pressure={0.5,0.4,0.15}, orient="along"})
handle_brush:load(handle_pile, 0.6)
handle_brush:stroke({{285,948},{270,810},{258,690}}, {pressure={0.45,0.35,0.12}, orient="along"})

-- ferrule + bristle hints
ferrule_pile = pile{{"lead white",2},{"bone black",1}}
handle_brush:load(ferrule_pile, 0.5)
handle_brush:stroke({{221,703},{215,682}}, {pressure={0.4,0.3}, orient="along"})
handle_brush:load(pile{{"raw umber",1},{"vermilion",0.3}}, 0.5)
handle_brush:stroke({{196,718},{190,698}}, {pressure={0.35,0.25}, orient="along"})

--@ chunk 46

stray_cover = ellipse(900,873,20,20)
work(stray_cover, {hand="detail", pile=dark_bg, coverage=2.0, clip=true, edge="soft"})

bg_smooth = pile{{"raw umber",3},{"bone black",2},{"red earth",0.5}, medium=0.15}
work(bg_mask, {hand="glaze", pile=bg_smooth, angle=0.4, coverage=1.6})
blend(bg_mask, {angle=0.2})

--@ chunk 47

work(bg_mask, {hand="glaze", pile=bg_smooth, angle=0.4, coverage=1.8, clip=true})

--@ chunk 48

work(torso_shape, {hand="body", pile=clothing_cover, angle=0.4, coverage=3.0, clip=true, edge="firm", fill=true})

--@ chunk 49

shoulder_light = poly({{195,480},{330,455},{420,470},{430,560},{300,590},{190,570}}, true)
work(shoulder_light * torso_shape, {hand="scumble", pile=coat_light, coverage=0.9, clip=torso_shape, edge="loose"})

fold_brush = brush{kind="filbert", width=10, point=0.15}
fold_brush:load(coat_dark, 0.5)
fold_brush:stroke({{330,620},{318,780},{330,950},{320,1150}}, {pressure={0.3,0.45,0.35,0.3}, orient="along"})
fold_brush:stroke({{560,610},{575,760},{560,940},{575,1140}}, {pressure={0.25,0.4,0.3,0.25}, orient="along"})
fold_brush:load(coat_light, 0.4)
fold_brush:stroke({{260,600},{250,760},{260,920}}, {pressure={0.25,0.35,0.2}, orient="along"})
blend(torso_shape, {angle=1.5})

--@ chunk 50

palette_shape = poly({
  {300,878},{395,852},{498,868},{560,930},{568,1010},
  {510,1075},{400,1090},{305,1050},{258,970}
}, true)
thumb_hole = ellipse(322,978,20,28)
palette_shape = palette_shape - thumb_hole

wood_pile = pile{{"raw umber",2},{"yellow ochre",1.5},{"lead white",0.8}}
work(palette_shape, {hand="body", pile=wood_pile, angle=0.4, coverage=2.4, clip=true, edge="firm", fill=true})

pal_shadow = palette_shape:rim(18,8)
work(pal_shadow, {hand="scumble", pile=pile{{"raw umber",2},{"bone black",1}}, coverage=1.0, clip=palette_shape, edge="soft"})

smear = pile{{"lead white",1},{"raw umber",0.6},{"yellow ochre",0.4}}
smear_brush = brush{kind="filbert", width=18, point=0}
smear_brush:load(smear, 0.5)
smear_brush:stroke({{400,940},{440,955},{400,970}}, {pressure={0.4,0.5,0.3}, orient="along"})

wait(4*24*60)
print(drying(410,900))

--@ chunk 51

dab_brush = brush{kind="round", width=16, point=0}
dabs3 = {
  {350,864, pile{{"lead white",1}}},
  {388,850, pile{{"yellow ochre",1}}},
  {428,846, pile{{"vermilion",1}}},
  {466,854, pile{{"red earth",1}}},
  {500,871, pile{{"cobalt blue",1}}},
  {525,898, pile{{"green earth",1}}},
  {542,931, pile{{"bone black",1}}},
}
for i, d in ipairs(dabs3) do
  dab_brush:wipe(1.0)
  dab_brush:reload(d[3], 1.0)
  dab_brush:touch(d[1], d[2], {pressure=0.9})
end

thumb_pile = pile{{"lead white",1.5},{"yellow ochre",1.5},{"red earth",1},{"vermilion",0.4}}
thumb2 = ellipse(320,985,10,16)
work(thumb2, {hand="detail", pile=thumb_pile, coverage=2.0, clip=true, edge="soft"})
fine_brush:load(shadow_mix,0.3)
fine_brush:stroke({{311,972},{309,985},{312,998}}, {pressure={0.2,0.22,0.18}, orient="along"})
fine_brush:load(light_mix,0.35)
fine_brush:stroke({{326,976},{328,986}}, {pressure={0.2,0.15}, orient="along"})

--@ chunk 52

handle_pile = pile{{"raw umber",2},{"yellow ochre",1},{"lead white",0.3}}
handle_brush = brush{kind="rigger", width=3.2, point=0.7}
handle_brush:load(handle_pile, 0.7)
handle_brush:stroke({{300,960},{255,820},{220,700}}, {pressure={0.5,0.4,0.15}, orient="along"})
handle_brush:load(handle_pile, 0.7)
handle_brush:stroke({{292,955},{235,830},{195,715}}, {pressure={0.5,0.4,0.15}, orient="along"})
handle_brush:load(handle_pile, 0.6)
handle_brush:stroke({{285,948},{270,810},{258,690}}, {pressure={0.45,0.35,0.12}, orient="along"})

ferrule_pile = pile{{"lead white",2},{"bone black",1}}
handle_brush:load(ferrule_pile, 0.5)
handle_brush:stroke({{221,703},{215,682}}, {pressure={0.4,0.3}, orient="along"})
handle_brush:load(pile{{"raw umber",1},{"vermilion",0.3}}, 0.5)
handle_brush:stroke({{196,718},{190,698}}, {pressure={0.35,0.25}, orient="along"})

--@ chunk 53

work(head_sil, {hand="body", pile=flesh_base, angle=0.5, coverage=3.0, clip=true, edge="firm", fill=true})

--@ chunk 54

hairline_pts = { {350, 470}, {375, 410}, {405, 365}, {440, 345}, {460, 340}, {480, 345}, {515, 365}, {545, 410}, {570, 470} }
hair_mask = head_sil * above(hairline_pts)
work(hair_mask, {hand="body", pile=hair_base, angle=0.15, coverage=2.6, clip=true, edge="firm", fill=true})

shadow_mix = pile{{"yellow ochre",2},{"red earth",2},{"raw umber",1},{"bone black",0.4},{"cobalt blue",0.3}}
work(head_shadow * head_sil, {hand="scumble", pile=shadow_mix, angle=0.6, coverage=1.6, clip=head_sil})

light_mix = pile{{"lead white",3},{"yellow ochre",1},{"vermilion",0.4}}
work(head_lit * head_sil, {hand="scumble", pile=light_mix, angle=0.5, coverage=1.3, clip=head_sil})

blend(head_sil, {angle=0.5})

--@ chunk 55

work(hair_mask, {hand="body", pile=hair_base, angle=0.15, coverage=2.6, clip=true, edge="firm", fill=true})

hair_dark = pile{{"bone black",3},{"raw umber",2}}
hair_light = pile{{"lead white",1.5},{"bone black",1.5},{"raw umber",1}}
work(hair_mask * head_shadow, {hand="scumble", pile=hair_dark, coverage=1.3, clip=hair_mask})
work(hair_mask * head_lit, {hand="scumble", pile=hair_light, coverage=1.1, clip=hair_mask})

strand_brush = brush{kind="rigger", width=1.4, point=0.9}
strand_brush:load(hair_light, 0.5)
strand_brush:stroke({{400,300},{380,340},{365,380}}, {pressure={0.35,0.15}, orient="along"})
strand_brush:stroke({{460,297},{455,330},{450,360}}, {pressure={0.3,0.12}, orient="along"})
strand_brush:stroke({{520,320},{535,355},{545,395}}, {pressure={0.3,0.12}, orient="along"})
strand_brush:load(hair_dark, 0.5)
strand_brush:stroke({{420,300},{435,335},{440,370}}, {pressure={0.3,0.12}, orient="along"})
strand_brush:stroke({{560,360},{575,400},{580,440}}, {pressure={0.3,0.12}, orient="along"})

--@ chunk 56

detail_brush = brush{kind="round", width=3.4, point=0.55}
fine_brush = brush{kind="round", width=1.6, point=0.8}
dark_line = pile{{"raw umber",2},{"bone black",2}, medium=0.25}

detail_brush:load(dark_line, 0.7)
detail_brush:stroke({{372, 397},{402, 388},{432, 392}}, {pressure={0.5,0.35}, ramps={0.3,0.5}, orient="along"})
detail_brush:stroke({{462, 395},{490, 392},{512, 398}}, {pressure={0.4,0.25}, ramps={0.3,0.5}, orient="along"})

detail_brush:load(shadow_mix, 0.5)
detail_brush:stroke({{378, 412},{402, 407},{428, 413}}, {pressure={0.45,0.3}, orient="along"})
detail_brush:stroke({{466, 411},{490, 408},{510, 414}}, {pressure={0.4,0.25}, orient="along"})

white_pile = pile{{"lead white",4},{"yellow ochre",0.3}}
iris_pile = pile{{"raw umber",3},{"bone black",1}}
pupil_pile = pile{{"bone black",3}}
spark_pile = pile{{"lead white",1}}

eyeL_white = ellipse(403,417,13,6)
eyeR_white = ellipse(487,415,10,5)
work(eyeL_white, {hand="detail", pile=white_pile, coverage=2.2, clip=true})
work(eyeR_white, {hand="detail", pile=white_pile, coverage=2.0, clip=true})

irisL = ellipse(400,417,6,6) * eyeL_white
irisR = ellipse(486,415,4.5,4.5) * eyeR_white
work(irisL, {hand="detail", pile=iris_pile, coverage=2.2, clip=true})
work(irisR, {hand="detail", pile=iris_pile, coverage=2.0, clip=true})

pupilL = ellipse(400,417,2.6,2.6)
pupilR = ellipse(486,415,2,2)
work(pupilL, {hand="detail", pile=pupil_pile, coverage=2.5, clip=true})
work(pupilR, {hand="detail", pile=pupil_pile, coverage=2.5, clip=true})

fine_brush:load(dark_line, 0.5)
fine_brush:stroke({{391,421},{403,423},{416,421}}, {pressure={0.3,0.25}, orient="along"})
fine_brush:stroke({{478,419},{487,421},{496,419}}, {pressure={0.25,0.2}, orient="along"})

sparkL = ellipse(397.5,414,1.1,1.1)
sparkR = ellipse(484,412,0.9,0.9)
work(sparkL, {hand="detail", pile=spark_pile, coverage=2.5, clip=true})
work(sparkR, {hand="detail", pile=spark_pile, coverage=2.5, clip=true})

--@ chunk 57

fine_brush:load(shadow_mix, 0.45)
fine_brush:stroke({{433,408},{436,428},{433,452},{427,468}}, {pressure={0.4,0.45,0.3}, orient="along"})
fine_brush:load(shadow_mix, 0.35)
fine_brush:stroke({{437,415},{440,435},{436,455}}, {pressure={0.3,0.35,0.2}, orient="along"})

fine_brush:load(light_mix, 0.5)
fine_brush:stroke({{427,404},{422,432},{418,460}}, {pressure={0.35,0.4,0.15}, orient="along"})

fine_brush:load(dark_line, 0.45)
fine_brush:touch(410,469,{pressure=0.4})
fine_brush:touch(428,471,{pressure=0.35})
fine_brush:stroke({{407,476},{418,479},{429,477}}, {pressure={0.28,0.32,0.22}, orient="along"})

nose_line = ribbon({{429,403},{423,432},{418,462}}, 26)
blend(nose_line, {angle=1.5})

-- mouth
fine_brush:load(dark_line, 0.42)
fine_brush:stroke({{394,501},{416,505},{440,502}}, {pressure={0.32,0.38,0.24}, orient="along"})
lip_pile = pile{{"vermilion",1},{"red earth",1},{"lead white",1}}
lower_lip = poly({{398,503},{416,509},{438,504},{424,514},{408,513}}, true)
work(lower_lip, {hand="detail", pile=lip_pile, coverage=1.7, clip=true, edge="soft"})
fine_brush:load(shadow_mix,0.35)
fine_brush:stroke({{404,518},{420,521},{434,517}}, {pressure={0.28,0.24}, orient="along"})

-- jaw rim shadow
lower_band = head_sil * mask(function(x,y) return y > 500 and 1 or 0 end)
rim_shadow = head_sil:rim(24, 10) * lower_band
work(rim_shadow, {hand="scumble", pile=shadow_mix, coverage=1.1, clip=head_sil, edge="loose"})

-- ear
ear_pile = pile{{"lead white",1.6},{"yellow ochre",1.6},{"red earth",1.2},{"vermilion",0.5}}
ear_shape = poly({{348,420},{362,405},{372,425},{368,460},{354,462},{345,440}}, true)
work(ear_shape, {hand="detail", pile=ear_pile, coverage=2.0, clip=true, edge="soft"})
fine_brush:load(shadow_mix,0.35)
fine_brush:stroke({{358,420},{362,438},{357,452}}, {pressure={0.25,0.3,0.2}, orient="along"})

--@ chunk 58

fix2 = poly({{375,458},{460,455},{465,562},{368,562}}, true)
work(fix2 * head_sil, {hand="detail", pile=flesh_base, coverage=2.4, clip=true, edge="soft"})

--@ chunk 59

fine_brush:load(shadow_mix, 0.4)
fine_brush:stroke({{433,408},{436,428},{433,452},{427,468}}, {pressure={0.35,0.4,0.28}, orient="along"})
fine_brush:load(light_mix, 0.45)
fine_brush:stroke({{427,404},{422,432},{418,460}}, {pressure={0.3,0.35,0.15}, orient="along"})
fine_brush:load(dark_line, 0.4)
fine_brush:touch(410,469,{pressure=0.35})
fine_brush:touch(428,471,{pressure=0.3})
fine_brush:stroke({{407,476},{418,479},{429,477}}, {pressure={0.24,0.28,0.2}, orient="along"})

fine_brush:load(dark_line, 0.4)
fine_brush:stroke({{394,501},{416,505},{440,502}}, {pressure={0.3,0.35,0.22}, orient="along"})
lip_pile = pile{{"vermilion",1},{"red earth",1},{"lead white",1}}
lower_lip = poly({{398,503},{416,509},{438,504},{424,514},{408,513}}, true)
work(lower_lip, {hand="detail", pile=lip_pile, coverage=1.6, clip=true, edge="soft"})
fine_brush:load(shadow_mix,0.3)
fine_brush:stroke({{404,518},{420,521},{434,517}}, {pressure={0.25,0.2}, orient="along"})

-- small chin shadow only
fine_brush:load(shadow_mix, 0.3)
fine_brush:stroke({{390,535},{420,548},{450,535}}, {pressure={0.25,0.3,0.2}, orient="along"})

--@ chunk 60

glow = ellipse(470,420,260,300):soften(120)
glow_pile = pile{{"raw umber",2},{"red earth",1},{"yellow ochre",0.4}, medium=0.2}
work(glow * bg_mask, {hand="glaze", pile=glow_pile, coverage=1.0, clip=true})

--@ chunk 61

work(glow, {hand="glaze", pile=bg_smooth, coverage=1.6, clip=true})
blend(glow, {angle=0.3})

--@ chunk 62

work(head_sil, {hand="body", pile=flesh_base, angle=0.5, coverage=3.0, clip=true, edge="firm", fill=true})
work(hair_mask, {hand="body", pile=hair_base, angle=0.15, coverage=2.6, clip=true, edge="firm", fill=true})
work(head_shadow * head_sil, {hand="scumble", pile=shadow_mix, angle=0.6, coverage=1.6, clip=head_sil})
work(head_lit * head_sil, {hand="scumble", pile=light_mix, angle=0.5, coverage=1.3, clip=head_sil})
blend(head_sil, {angle=0.5})
work(hair_mask, {hand="body", pile=hair_base, angle=0.15, coverage=2.6, clip=true, edge="firm", fill=true})
work(hair_mask * head_shadow, {hand="scumble", pile=hair_dark, coverage=1.3, clip=hair_mask})
work(hair_mask * head_lit, {hand="scumble", pile=hair_light, coverage=1.1, clip=hair_mask})

--@ chunk 63

strand_brush = brush{kind="rigger", width=1.4, point=0.9}
strand_brush:load(hair_light, 0.5)
strand_brush:stroke({{400,300},{380,340},{365,380}}, {pressure={0.35,0.15}, orient="along"})
strand_brush:stroke({{460,297},{455,330},{450,360}}, {pressure={0.3,0.12}, orient="along"})
strand_brush:stroke({{520,320},{535,355},{545,395}}, {pressure={0.3,0.12}, orient="along"})
strand_brush:load(hair_dark, 0.5)
strand_brush:stroke({{420,300},{435,335},{440,370}}, {pressure={0.3,0.12}, orient="along"})
strand_brush:stroke({{560,360},{575,400},{580,440}}, {pressure={0.3,0.12}, orient="along"})

detail_brush:load(dark_line, 0.7)
detail_brush:stroke({{372, 397},{402, 388},{432, 392}}, {pressure={0.5,0.35}, ramps={0.3,0.5}, orient="along"})
detail_brush:stroke({{462, 395},{490, 392},{512, 398}}, {pressure={0.4,0.25}, ramps={0.3,0.5}, orient="along"})
detail_brush:load(shadow_mix, 0.5)
detail_brush:stroke({{378, 412},{402, 407},{428, 413}}, {pressure={0.45,0.3}, orient="along"})
detail_brush:stroke({{466, 411},{490, 408},{510, 414}}, {pressure={0.4,0.25}, orient="along"})

eyeL_white = ellipse(403,417,13,6)
eyeR_white = ellipse(487,415,10,5)
work(eyeL_white, {hand="detail", pile=white_pile, coverage=2.2, clip=true})
work(eyeR_white, {hand="detail", pile=white_pile, coverage=2.0, clip=true})
irisL = ellipse(400,417,6,6) * eyeL_white
irisR = ellipse(486,415,4.5,4.5) * eyeR_white
work(irisL, {hand="detail", pile=iris_pile, coverage=2.2, clip=true})
work(irisR, {hand="detail", pile=iris_pile, coverage=2.0, clip=true})
pupilL = ellipse(400,417,2.6,2.6)
pupilR = ellipse(486,415,2,2)
work(pupilL, {hand="detail", pile=pupil_pile, coverage=2.5, clip=true})
work(pupilR, {hand="detail", pile=pupil_pile, coverage=2.5, clip=true})
fine_brush:load(dark_line, 0.5)
fine_brush:stroke({{391,421},{403,423},{416,421}}, {pressure={0.3,0.25}, orient="along"})
fine_brush:stroke({{478,419},{487,421},{496,419}}, {pressure={0.25,0.2}, orient="along"})
sparkL = ellipse(397.5,414,1.1,1.1)
sparkR = ellipse(484,412,0.9,0.9)
work(sparkL, {hand="detail", pile=spark_pile, coverage=2.5, clip=true})
work(sparkR, {hand="detail", pile=spark_pile, coverage=2.5, clip=true})

--@ chunk 64

fine_brush:load(shadow_mix, 0.4)
fine_brush:stroke({{433,408},{436,428},{433,452},{427,468}}, {pressure={0.35,0.4,0.28}, orient="along"})
fine_brush:load(light_mix, 0.45)
fine_brush:stroke({{427,404},{422,432},{418,460}}, {pressure={0.3,0.35,0.15}, orient="along"})
fine_brush:load(dark_line, 0.4)
fine_brush:touch(410,469,{pressure=0.35})
fine_brush:touch(428,471,{pressure=0.3})
fine_brush:stroke({{407,476},{418,479},{429,477}}, {pressure={0.24,0.28,0.2}, orient="along"})

fine_brush:load(dark_line, 0.4)
fine_brush:stroke({{394,501},{416,505},{440,502}}, {pressure={0.3,0.35,0.22}, orient="along"})
lower_lip = poly({{398,503},{416,509},{438,504},{424,514},{408,513}}, true)
work(lower_lip, {hand="detail", pile=lip_pile, coverage=1.6, clip=true, edge="soft"})
fine_brush:load(shadow_mix,0.3)
fine_brush:stroke({{404,518},{420,521},{434,517}}, {pressure={0.25,0.2}, orient="along"})
fine_brush:load(shadow_mix, 0.3)
fine_brush:stroke({{390,535},{420,548},{450,535}}, {pressure={0.25,0.3,0.2}, orient="along"})

ear_shape = poly({{348,420},{362,405},{372,425},{368,460},{354,462},{345,440}}, true)
work(ear_shape, {hand="detail", pile=ear_pile, coverage=2.0, clip=true, edge="soft"})
fine_brush:load(shadow_mix,0.35)
fine_brush:stroke({{358,420},{362,438},{357,452}}, {pressure={0.25,0.3,0.2}, orient="along"})

--@ chunk 65

darken_pile = pile{{"raw umber",3},{"bone black",2}, medium=0.1}
work(glow * bg_mask, {hand="glaze", pile=darken_pile, coverage=1.3, clip=true})
blend(glow * bg_mask, {angle=0.4})

--@ chunk 66

deep_shadow = pile{{"red earth",2},{"raw umber",2},{"bone black",0.6},{"cobalt blue",0.2}}
right_cheek = poly({{460,395},{560,420},{555,520},{470,540},{440,470}}, true) * head_sil
work(right_cheek, {hand="scumble", pile=deep_shadow, coverage=0.9, clip=head_sil, edge="loose"})
blend(right_cheek, {angle=0.6})

-- ear depth
fine_brush:load(deep_shadow, 0.35)
fine_brush:stroke({{357,425},{361,440},{356,455}}, {pressure={0.3,0.35,0.25}, orient="along"})

--@ chunk 67

fix3 = poly({{455,392},{562,415},{558,545},{450,545},{430,460}}, true) * head_sil
work(fix3, {hand="detail", pile=flesh_base, coverage=2.6, clip=true, edge="soft"})
work(head_lit * fix3, {hand="scumble", pile=light_mix, coverage=0.6, clip=head_sil})

--@ chunk 68

work(head_sil, {hand="body", pile=flesh_base, angle=0.5, coverage=3.0, clip=true, edge="firm", fill=true})
work(hair_mask, {hand="body", pile=hair_base, angle=0.15, coverage=2.6, clip=true, edge="firm", fill=true})
work(head_shadow * head_sil, {hand="scumble", pile=shadow_mix, angle=0.6, coverage=1.6, clip=head_sil})
work(head_lit * head_sil, {hand="scumble", pile=light_mix, angle=0.5, coverage=1.3, clip=head_sil})
blend(head_sil, {angle=0.5})
work(hair_mask, {hand="body", pile=hair_base, angle=0.15, coverage=2.6, clip=true, edge="firm", fill=true})
work(hair_mask * head_shadow, {hand="scumble", pile=hair_dark, coverage=1.3, clip=hair_mask})
work(hair_mask * head_lit, {hand="scumble", pile=hair_light, coverage=1.1, clip=hair_mask})

--@ chunk 69

strand_brush:load(hair_light, 0.5)
strand_brush:stroke({{400,300},{380,340},{365,380}}, {pressure={0.35,0.15}, orient="along"})
strand_brush:stroke({{460,297},{455,330},{450,360}}, {pressure={0.3,0.12}, orient="along"})
strand_brush:stroke({{520,320},{535,355},{545,395}}, {pressure={0.3,0.12}, orient="along"})
strand_brush:load(hair_dark, 0.5)
strand_brush:stroke({{420,300},{435,335},{440,370}}, {pressure={0.3,0.12}, orient="along"})
strand_brush:stroke({{560,360},{575,400},{580,440}}, {pressure={0.3,0.12}, orient="along"})

detail_brush:load(dark_line, 0.7)
detail_brush:stroke({{372, 397},{402, 388},{432, 392}}, {pressure={0.5,0.35}, ramps={0.3,0.5}, orient="along"})
detail_brush:stroke({{462, 395},{490, 392},{512, 398}}, {pressure={0.4,0.25}, ramps={0.3,0.5}, orient="along"})
detail_brush:load(shadow_mix, 0.5)
detail_brush:stroke({{378, 412},{402, 407},{428, 413}}, {pressure={0.45,0.3}, orient="along"})
detail_brush:stroke({{466, 411},{490, 408},{510, 414}}, {pressure={0.4,0.25}, orient="along"})

eyeL_white = ellipse(403,417,13,6)
eyeR_white = ellipse(487,415,10,5)
work(eyeL_white, {hand="detail", pile=white_pile, coverage=2.2, clip=true})
work(eyeR_white, {hand="detail", pile=white_pile, coverage=2.0, clip=true})
irisL = ellipse(400,417,6,6) * eyeL_white
irisR = ellipse(486,415,4.5,4.5) * eyeR_white
work(irisL, {hand="detail", pile=iris_pile, coverage=2.2, clip=true})
work(irisR, {hand="detail", pile=iris_pile, coverage=2.0, clip=true})
pupilL = ellipse(400,417,2.6,2.6)
pupilR = ellipse(486,415,2,2)
work(pupilL, {hand="detail", pile=pupil_pile, coverage=2.5, clip=true})
work(pupilR, {hand="detail", pile=pupil_pile, coverage=2.5, clip=true})
fine_brush:load(dark_line, 0.5)
fine_brush:stroke({{391,421},{403,423},{416,421}}, {pressure={0.3,0.25}, orient="along"})
fine_brush:stroke({{478,419},{487,421},{496,419}}, {pressure={0.25,0.2}, orient="along"})
sparkL = ellipse(397.5,414,1.1,1.1)
sparkR = ellipse(484,412,0.9,0.9)
work(sparkL, {hand="detail", pile=spark_pile, coverage=2.5, clip=true})
work(sparkR, {hand="detail", pile=spark_pile, coverage=2.5, clip=true})

fine_brush:load(shadow_mix, 0.4)
fine_brush:stroke({{433,408},{436,428},{433,452},{427,468}}, {pressure={0.35,0.4,0.28}, orient="along"})
fine_brush:load(light_mix, 0.45)
fine_brush:stroke({{427,404},{422,432},{418,460}}, {pressure={0.3,0.35,0.15}, orient="along"})
fine_brush:load(dark_line, 0.4)
fine_brush:touch(410,469,{pressure=0.35})
fine_brush:touch(428,471,{pressure=0.3})
fine_brush:stroke({{407,476},{418,479},{429,477}}, {pressure={0.24,0.28,0.2}, orient="along"})

fine_brush:load(dark_line, 0.4)
fine_brush:stroke({{394,501},{416,505},{440,502}}, {pressure={0.3,0.35,0.22}, orient="along"})
lower_lip = poly({{398,503},{416,509},{438,504},{424,514},{408,513}}, true)
work(lower_lip, {hand="detail", pile=lip_pile, coverage=1.6, clip=true, edge="soft"})
fine_brush:load(shadow_mix,0.3)
fine_brush:stroke({{404,518},{420,521},{434,517}}, {pressure={0.25,0.2}, orient="along"})
fine_brush:load(shadow_mix, 0.3)
fine_brush:stroke({{390,535},{420,548},{450,535}}, {pressure={0.25,0.3,0.2}, orient="along"})

ear_shape = poly({{348,420},{362,405},{372,425},{368,460},{354,462},{345,440}}, true)
work(ear_shape, {hand="detail", pile=ear_pile, coverage=2.0, clip=true, edge="soft"})
fine_brush:load(shadow_mix,0.35)
fine_brush:stroke({{358,420},{362,438},{357,452}}, {pressure={0.25,0.3,0.2}, orient="along"})

--@ chunk 70

collar_brush = brush{kind="round", width=4, point=0.5}
collar_brush:load(coat_light, 0.5)
collar_brush:stroke({{330,558},{430,580},{540,562}}, {pressure={0.4,0.5,0.35}, orient="along"})
collar_brush:load(coat_dark, 0.4)
collar_brush:stroke({{335,570},{430,595},{535,575}}, {pressure={0.35,0.45,0.3}, orient="along"})

-- rim light along lit shoulder edge
rim_brush = brush{kind="filbert", width=6, point=0.3}
rim_pile = pile{{"raw umber",2},{"yellow ochre",1},{"lead white",0.5}}
rim_brush:load(rim_pile, 0.5)
rim_brush:stroke({{200,590},{230,500},{300,460}}, {pressure={0.3,0.35,0.2}, orient="along"})

--@ chunk 71

detail_brush:load(dark_line, 0.55)
detail_brush:stroke({{372, 397},{402, 388},{432, 392}}, {pressure={0.45,0.32}, orient="along"})
detail_brush:stroke({{462, 395},{490, 392},{512, 398}}, {pressure={0.38,0.25}, orient="along"})

fine_brush:load(dark_line, 0.35)
fine_brush:stroke({{396,502},{416,506},{438,503}}, {pressure={0.22,0.26,0.18}, orient="along"})
fine_brush:load(lip_pile, 0.4)
fine_brush:stroke({{402,510},{416,513},{432,509}}, {pressure={0.2,0.25,0.16}, orient="along"})

--@ chunk 72

sig_brush = brush{kind="rigger", width=1.2, point=0.85}
sig_brush:load(pile{{"raw umber",1},{"bone black",1}}, 0.4)
sig_brush:stroke({{835,1205},{845,1198},{852,1208},{862,1197}}, {pressure={0.25,0.3,0.25,0.3}, orient="along"})
sig_brush:load(pile{{"raw umber",1},{"bone black",1}}, 0.35)
sig_brush:stroke({{868,1200},{878,1210}}, {pressure={0.25,0.2}, orient="along"})

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
